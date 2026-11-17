Your `encode` command turns text into token IDs. In this stage, add `decode` to
turn those IDs back into text using the same merge list.

### Why decode?

A decoder reconstructs the bytes represented by each token ID. Join those bytes
before interpreting them as text: a token can contain only part of a character.

👋 is four UTF-8 bytes, `[240, 159, 145, 139]`. GPT-2 splits them across two
tokens, `[41840, 233]`, IDs from its own vocabulary. Token boundaries don't have
to line up with character boundaries: the first token holds `[240, 159, 145]`
and the second holds `[139]`. UTF-8 needs all four bytes to make 👋, so decoding
the first token by itself gives the replacement character `�`. Join the bytes
from both tokens first, and the result is 👋.

Your decoder will use your custom IDs, not GPT-2's vocabulary yet. Complete
UTF-8 sequences recover the original text; malformed sequences produce
replacement characters as described below.

### The `decode` command

The `decode` command parses two JSON files in the formats from earlier stages:
`ids.json` is an array of IDs, and `merges.json` is an array of pairs.

1. Build the same vocabulary training would. ID `n` below 256 represents the
   single byte `n`. ID `256 + i` represents the bytes of the first ID in merge
   `i`, followed by the bytes of the second. Process the merges in order so each
   pair's byte sequences are already available.
2. Join every token's bytes into one buffer.
3. Decode that buffer as UTF-8. Wherever the bytes are invalid, use the
   replacement character U+FFFD rather than stopping with an error.

For example, `[258, 100]` with the three merges from `aaabdaaabac` prints:

```json
"aaabd"
```

`258` expands to `257` and `98`, then to `256`, `97`, and `98`, and finally to
the bytes of `aaab`. `100` is `d`.

`[240, 159, 145, 139]` prints the wave emoji.

### How many replacement characters?

U+FFFD, shown as `�`, is a replacement _character_, not a replacement byte. One
`�` can stand for several bytes. Use the
[maximal subpart rule](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G66453):
when a sequence is malformed, replace the longest prefix that could still begin
a valid UTF-8 sequence. If the first byte cannot begin any valid sequence,
replace that byte alone.

Read the buffer from left to right:

- Bytes 0–127 form complete one-byte sequences. A byte from 194–223 starts a
  two-byte sequence, 224–239 a three-byte sequence, and 240–244 a four-byte
  sequence.
- Following bytes must be continuation bytes, 128–191, with the tighter
  second-byte limits in the table below.
- If the sequence is complete and valid, output its character. If a byte doesn't
  fit, output one `�` for the valid prefix read so far, then reconsider the byte
  that didn't fit as the start of the next sequence. If the buffer ends during a
  valid prefix, replace that prefix with one `�`.
- A byte that cannot start a sequence, such as 255 or a stray continuation byte,
  produces one `�` by itself.

Four lead bytes restrict the second byte further. These limits exclude encodings
longer than necessary, surrogate values reserved for UTF-16, and values above
U+10FFFF, Unicode's highest code point:

| First byte | Length | Second byte |
| ---------- | ------ | ----------- |
| 224        | 3      | 160–191     |
| 237        | 3      | 128–159     |
| 240        | 4      | 144–191     |
| 244        | 4      | 128–143     |

A second byte outside its range doesn't fit, just like any other byte that
doesn't continue a sequence.

| Bytes        | Prints           | Why                                                                                                                                   |
| ------------ | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `[240, 159]` | `"\uFFFD"`       | 240 starts a 4-byte sequence and 159 continues it. The buffer ends first, so both bytes become one `�`.                               |
| `[255, 255]` | `"\uFFFD\uFFFD"` | 255 cannot start a sequence, so each 255 becomes its own `�`.                                                                         |
| `[240, 240]` | `"\uFFFD\uFFFD"` | The second 240 doesn't continue the first, so the first becomes one `�`. The buffer ends after the second 240, producing another `�`. |

### Your progress

<details>
  <summary>Back to the original text for the example sentence `to the top of the tree`</summary>

Decoding the 11 IDs from the previous stage with the same five merges recovers
the sentence's original 22 bytes:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens
decode    ██████████████████████  22 bytes, the original text   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

</details>

### Tests

The tester will write `[258, 100]` to `ids.json` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then execute your program
like this:

```bash
$ ./your_program.sh decode ids.json --merges merges.json
```

Your program must print the text as a JSON string to stdout:

```json
"aaabd"
```

The tester will also run your program on a few other lists, including the full
wave emoji, half of the wave emoji, bytes that are never valid UTF-8, and the
IDs of the sentence from "Your progress". The malformed-input tests cover
truncated sequences and bytes that cannot appear in valid UTF-8; they do not
exercise every case in the rule above.

Print the text with your JSON library, so quotes, backslashes, and newlines are
escaped correctly. `"\uFFFD"` and `"�"` are the same JSON string, so either
passes.

### Language guidance

{{#lang_is_python}}

- `bytes.decode("utf-8", errors="replace")` replaces invalid bytes this way.

{{/lang_is_python}}

{{#lang_is_typescript}}

- Use `new TextDecoder("utf-8", { ignoreBOM: true }).decode(bytes)`. It replaces
  malformed sequences and preserves a leading U+FEFF character, the byte order
  mark. The default `new TextDecoder().decode(bytes)` removes that leading
  character instead of returning all the decoded text.

{{/lang_is_typescript}}

{{#lang_is_java}}

- Java 25's `new String(bytes, StandardCharsets.UTF_8)` handles the malformed
  inputs in these tests, but differs from the full replacement rule above. For
  example, `[237, 160, 128]`, an invalid encoding of a surrogate, becomes one
  `�` rather than three. To follow the full rule for those inputs, implement the
  byte checks above instead of relying on this constructor alone.

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

- You can write the decoding loop using the rule in "How many replacement
  characters?". Copy valid UTF-8 sequences to the output as bytes, and write the
  3 bytes 239, 191, 189 for each `�`.

{{/lang_is_cpp}}
