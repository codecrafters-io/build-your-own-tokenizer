In this stage, you'll implement `decode` to turn token IDs back into text using
their merge list.

### Reconstructing bytes

Read `ids.json` as an array of IDs and `merges.json` as an array of pairs. IDs
0–255 represent their byte values. For merge `i`, counting from 0, ID `256 + i`
contains the first token's bytes followed by the second's. Build these entries
in merge order, then join the requested tokens' bytes in input order and decode
the whole buffer as UTF-8.

Token boundaries need not match character boundaries. 👋 uses four bytes,
`[240, 159, 145, 139]`. GPT-2's tokens `[41840, 233]` hold `[240, 159, 145]` and
`[139]`. Joining their bytes before decoding recovers 👋 instead of replacement
characters. Your decoder still uses custom IDs, not GPT-2's.

With the merges learned from `aaabdaaabac`, `[258, 100]` decodes to:

```json
"aaabd"
```

The merges expand `258` through `257` and `256` into `aaab`. Byte `100` adds
`d`.

### Invalid UTF-8

Replace malformed sequences with U+FFFD (`�`) rather than stopping. One
replacement character can represent several bytes.

Read left to right, using these validity rules:

- Bytes 0–127 decode directly. Lead bytes 194–223 start two-byte sequences,
  224–239 start three-byte sequences, and 240–244 start four-byte sequences.
- Continuation bytes must be 128–191, except for these tighter second-byte
  limits:

| First byte | Length | Second byte |
| ---------- | ------ | ----------- |
| 224        | 3      | 160–191     |
| 237        | 3      | 128–159     |
| 240        | 4      | 144–191     |
| 244        | 4      | 128–143     |

These limits exclude encodings longer than needed, surrogate values reserved for
UTF-16, and values above U+10FFFF, Unicode's highest code point.

Emit complete valid sequences normally. For malformed input, follow the
[maximal subpart rule](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G66453):
replace the longest prefix that could begin a valid sequence with one `�`, then
reconsider the byte that didn't fit. An unfinished prefix at the buffer's end
also gets one `�`. If the first byte cannot start a valid sequence, replace that
byte alone.

| Bytes        | Prints           | Why                                    |
| ------------ | ---------------- | -------------------------------------- |
| `[240, 159]` | `"\uFFFD"`       | A truncated four-byte sequence.        |
| `[255, 255]` | `"\uFFFD\uFFFD"` | Two invalid lead bytes.                |
| `[240, 240]` | `"\uFFFD\uFFFD"` | Neither lead byte gets a continuation. |

### Your progress

<details>
  <summary>Decoding `to the top of the tree`</summary>

Decoding its 11 IDs with the same five learned merges recovers the original 22
bytes:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens
decode    ██████████████████████  22 bytes, the original text   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

</details>

### Language guidance

{{#lang_is_python}}

- `bytes.decode("utf-8", errors="replace")` replaces invalid bytes this way.

{{/lang_is_python}}

{{#lang_is_typescript}}

- Use `new TextDecoder("utf-8", { ignoreBOM: true }).decode(bytes)` to replace
  malformed sequences and preserve a leading U+FEFF byte order mark. The default
  `new TextDecoder().decode(bytes)` removes that leading character.

{{/lang_is_typescript}}

{{#lang_is_java}}

- Java 25's `new String(bytes, StandardCharsets.UTF_8)` handles these tests'
  malformed inputs, but differs from the full rule. `[237, 160, 128]`, an
  invalid surrogate encoding, becomes one `�` rather than three. To follow the
  full rule, implement the byte checks above instead of relying on this
  constructor alone.

{{/lang_is_java}}

{{#lang_is_rust}}

- `String::from_utf8_lossy` replaces invalid bytes this way.

{{/lang_is_rust}}

{{#lang_is_go}}

- Follow the byte rules above. Ranging over a string gives one U+FFFD per
  invalid byte. [`strings.ToValidUTF8`](https://pkg.go.dev/strings#ToValidUTF8)
  gives one per run of invalid bytes. Neither matches this rule. Once a sequence
  is complete and valid,
  [`utf8.DecodeRune`](https://pkg.go.dev/unicode/utf8#DecodeRune) decodes it.

{{/lang_is_go}}

{{#lang_is_cpp}}

- Follow the byte rules above. Copy valid UTF-8 sequences to the output as
  bytes, and write the 3 bytes 239, 191, 189 for each `�`.

{{/lang_is_cpp}}

### Tests

The tester writes `[258, 100]` to `ids.json` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then runs:

```bash
$ ./your_program.sh decode ids.json --merges merges.json
```

Print one JSON string to stdout:

```json
"aaabd"
```

Tests also cover emoji bytes, the progress sentence, and JSON escaping.
Malformed-input tests cover truncated sequences and invalid bytes, not every
case above. Use your JSON library to escape quotes, backslashes, and newlines.
`"\uFFFD"` and `"�"` are equivalent JSON strings.
