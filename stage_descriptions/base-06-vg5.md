Your encoder already replaces byte sequences with IDs. The decoder will use the same merge definitions to recover those bytes. In this stage, you'll implement `decode` to turn token IDs back into text.

### Rebuilding the bytes

Read the IDs from `ids.json` and the learned pairs from `merges.json`. Reconstruct the vocabulary in merge order:

1. Each ID from 0 through 255 represents the single byte with that value.
2. For merge `i`, join the bytes of its two IDs and assign that sequence to ID `256 + i`.
3. Join the byte sequences for all requested IDs, in input order.
4. Decode the combined buffer as UTF-8, replacing invalid sequences with the replacement character `�`, also named U+FFFD.

With `[[97, 97], [256, 97], [257, 98]]`, ID 256 represents `aa`, 257 represents `aaa`, and 258 represents `aaab`, so `[258, 100]` decodes to `aaabd`.

A token can end partway through a character. With `[[240, 159], [145, 139]]`, ID 256 holds the first two bytes of `👋` and ID 257 the last two. Neither decodes alone. Join `[256, 257]` first, and the bytes decode to `👋`.

Some inputs deliberately contain invalid UTF-8. Keep running and replace those sequences:

| Bytes | Decoded text as JSON | Result |
| --- | --- | --- |
| `[240, 159]` | `"\uFFFD"` | An incomplete four-byte character becomes one replacement character. |
| `[255, 255]` | `"\uFFFD\uFFFD"` | Each 255 is an invalid start byte and gets its own replacement character. |

One replacement character can stand for several bytes. U+FFFD names that character, and it is separate from the IDs in your input.

<details>
  <summary>Your progress: recovering the original sentence</summary>

The 11 IDs from earlier stages, decoded with the same five merge rules, produce `"to the top of the tree"`. Encoding shortened the ID sequence while keeping all 22 bytes recoverable.

</details>

### How many replacement characters?

<details>
  <summary>Rules for a custom UTF-8 decoder</summary>

Read the buffer from left to right. A byte from 0 through 127 is a complete character, 194–223 starts a two-byte character, 224–239 a three-byte one, and 240–244 a four-byte one. The bytes that follow must be continuations, 128–191, within the limits below.

When a character is complete, output it. If a byte doesn't fit, or the buffer ends first, output one `�` for the valid prefix so far and resume at the byte that didn't fit. A byte that can't start a character, such as 255 or a stray continuation byte, becomes one `�` by itself.

| First byte | Length | Second byte |
| --- | --- | --- |
| 224 | 3 | 160–191 |
| 237 | 3 | 128–159 |
| 240 | 4 | 144–191 |
| 244 | 4 | 128–143 |

These limits exclude overlong encodings, surrogate values, and values above U+10FFFF, following the Unicode Standard's [maximal subpart rule](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G66453). The tests use truncated characters and bytes that cannot appear in valid UTF-8.

</details>

### Tests

The tester will write `[258, 100]` to `ids.json` and `[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then run:

```bash
$ ./your_program.sh decode ids.json --merges merges.json
```

Your program must print the decoded text as one JSON string:

```json
"aaabd"
```

Serialize the result with your JSON library so quotes, backslashes, and newlines are escaped correctly. `"\uFFFD"` and `"�"` are the same JSON string, so either form passes.

The tester also checks an emoji split across tokens, an incomplete emoji, invalid bytes, and text containing quotes, backslashes, and newlines.

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
