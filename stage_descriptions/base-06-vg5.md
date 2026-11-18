In this stage, you'll implement `decode` to turn token IDs back into text.
Your encoder already replaces byte sequences with IDs. The decoder will use
the same merge definitions to recover those bytes, then read them as UTF-8.

### Rebuilding the bytes

Read the IDs from `ids.json` and the learned pairs from `merges.json`.
Reconstruct the vocabulary in merge order:

1. Each ID from 0 through 255 represents the single byte with that value.
2. For merge `i`, join the bytes of its two IDs. Assign that byte sequence to
   ID `256 + i`.
3. Join the byte sequences for all the requested IDs, in input order.
4. Decode the combined buffer as UTF-8, replacing invalid sequences with the
   replacement character `�`, also named U+FFFD.

For example, with `[[97, 97], [256, 97], [257, 98]]`, ID 256 represents `aa`,
257 represents `aaa`, and 258 represents `aaab`. The input `[258, 100]`
therefore decodes to `aaabd`.

### Join bytes before decoding

A token can end partway through a UTF-8 character. Suppose the merge list is
`[[240, 159], [145, 139]]`. ID 256 holds the first two bytes of `👋`, and ID
257 holds the last two.

Decoding either token's bytes separately loses the character. Joining the bytes
for `[256, 257]` first gives `[240, 159, 145, 139]`, which decodes to `👋`.

Some inputs deliberately contain invalid UTF-8. The command must keep running
and replace those sequences:

| Bytes | Decoded text as JSON | Reason |
| --- | --- | --- |
| `[240, 159]` | `"\uFFFD"` | An incomplete four-byte character becomes one replacement character. |
| `[255, 255]` | `"\uFFFD\uFFFD"` | Each 255 is an invalid start byte and gets its own replacement character. |
| `[240, 240]` | `"\uFFFD\uFFFD"` | The second 240 cannot continue the first character. Each starts an incomplete sequence. |

A replacement character can stand for several bytes. Its Unicode name,
U+FFFD, identifies the character to insert. It is separate from the token IDs
in your input.

### Tests

The tester will write `[258, 100]` to `ids.json` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then run:

```bash
$ ./your_program.sh decode ids.json --merges merges.json
```

Your program must print the decoded text as one JSON string:

```json
"aaabd"
```

The tester also checks an emoji split across tokens, an incomplete emoji,
invalid bytes, and text containing quotes, backslashes, and newlines. Serialize
the result with your JSON library so those characters are escaped correctly.
The JSON strings `"\uFFFD"` and `"�"` have the same value, so either form
passes.

### Notes

{{#lang_is_python}}

- `bytes.decode("utf-8", errors="replace")` replaces invalid bytes this way.

{{/lang_is_python}}

{{#lang_is_typescript}}

- `new TextDecoder().decode(bytes)` replaces invalid bytes this way.

{{/lang_is_typescript}}

{{#lang_is_java}}

- `new String(bytes, StandardCharsets.UTF_8)` replaces invalid bytes this way.

{{/lang_is_java}}

{{#lang_is_rust}}

- `String::from_utf8_lossy` replaces invalid bytes this way.

{{/lang_is_rust}}

{{#lang_is_go}}

- Write the decoding loop yourself, following the rule in "How many replacement
  characters?". Ranging over a string gives one U+FFFD per invalid byte, and
  [`strings.ToValidUTF8`](https://pkg.go.dev/strings#ToValidUTF8) gives one per
  run of invalid bytes, so neither matches. Once a character's bytes are
  complete, [`utf8.DecodeRune`](https://pkg.go.dev/unicode/utf8#DecodeRune)
  decodes them.

{{/lang_is_go}}

{{#lang_is_cpp}}

- Write the decoding loop yourself, following the rule in "How many replacement
  characters?". The C++ standard library has no UTF-8 decoder that replaces
  invalid bytes. Valid characters can be copied to the output as their bytes,
  and each `�` is the 3 bytes 239, 191, 189.

{{/lang_is_cpp}}

### How many replacement characters?

<details>
  <summary>Rules for a custom UTF-8 decoder</summary>

The decoder reads the combined buffer from left to right:

- A byte from 0 through 127 is a complete one-byte character. Bytes 194–223
  start a two-byte character, 224–239 start a three-byte character, and 240–244
  start a four-byte character.
- Subsequent bytes must be continuation bytes, 128–191, with the extra
  second-byte limits below.
- When a character is complete, output it. If the next byte doesn't fit, or
  the buffer ends first, output one `�` for the valid prefix consumed so far.
  Resume at the byte that didn't fit.
- A byte that can't start a character, such as 255 or a stray continuation
  byte, becomes one `�` by itself.

Four starting bytes have narrower second-byte ranges:

| First byte | Length | Second byte |
| --- | --- | --- |
| 224 | 3 | 160–191 |
| 237 | 3 | 128–159 |
| 240 | 4 | 144–191 |
| 244 | 4 | 128–143 |

These limits exclude overlong encodings, surrogate values, and values above
U+10FFFF. A second byte outside its range is handled as a byte that doesn't
fit. This is the Unicode Standard's
[maximal subpart rule](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G66453).

The stage's invalid-input tests use truncated characters and bytes that cannot
appear in valid UTF-8. Java's built-in decoder differs on surrogate encodings,
which these tests do not use.

</details>

<details>
  <summary>Your progress: recovering the original sentence</summary>

The 11 IDs from the previous stage, decoded with the same five merge rules,
produce the JSON string `"to the top of the tree"`. Encoding shortened the
sequence of IDs while keeping the original 22 bytes recoverable.

</details>
