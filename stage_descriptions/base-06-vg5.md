Your encoder turns text into IDs. Decoding reverses that representation so you
can inspect the text in a token sequence. In this stage, you'll add `decode` to
recover text from IDs using the same merge rules.

### Recover the bytes

`ids.json` contains IDs, and `merges.json` contains the pairs defining them.

Reconstruct the vocabulary in merge order. Each ID below 256 represents its
single byte value. For merge `i`, token `256 + i` contains the first token's
bytes followed by the second token's bytes.

With `[[97, 97], [256, 97], [257, 98]]`, token 256 holds `aa`, 257 holds `aaa`,
and 258 holds `aaab`. The IDs `[258, 100]` therefore represent `aaabd`.

Collect the bytes for every input ID, join them in order, and decode the
complete byte sequence as UTF-8.

### Join bytes before decoding

A token can contain only part of a character. For example, with the merges
`[[240, 159], [145, 139]]`, token 256 holds the first two bytes of 👋 and token
257 holds the last two. Decoding `[256, 257]` must join them into
`[240, 159, 145, 139]` before turning them into the emoji.

Decoding these tokens separately would lose the emoji.

### Handle invalid UTF-8

Decode invalid UTF-8 with the replacement character `�` instead of failing.
Unicode assigns each character a number called a **code point**. `U+FFFD` is the
hexadecimal notation for this character's code point.

An incomplete valid prefix of a character becomes one `�`. A byte that cannot
start a character becomes one `�` by itself:

| Bytes        | JSON string      | Reason                                                                                   |
| ------------ | ---------------- | ---------------------------------------------------------------------------------------- |
| `[240, 159]` | `"\uFFFD"`       | These are the first two bytes of a four-byte character.                                  |
| `[255, 255]` | `"\uFFFD\uFFFD"` | Each 255 is invalid on its own.                                                          |
| `[240, 240]` | `"\uFFFD\uFFFD"` | The second 240 cannot continue the first character, so it starts another incomplete one. |

{{#lang_is_python}}

Python's `data.decode("utf-8", errors="replace")`, where `data` is the joined
`bytes` object, provides this behavior.

{{/lang_is_python}}

{{#lang_is_typescript}}

Use `new TextDecoder("utf-8", { ignoreBOM: true }).decode(bytes)` on the joined
byte array. It replaces invalid sequences and preserves a leading byte order
mark if one is present.

{{/lang_is_typescript}}

{{#lang_is_java}}

`new String(bytes, StandardCharsets.UTF_8)` provides the replacement behavior
needed for this stage's inputs.

{{/lang_is_java}}

{{#lang_is_rust}}

`String::from_utf8_lossy` provides this behavior for the joined bytes.

{{/lang_is_rust}}

{{#lang_is_go}}

Write a decoding loop using the rules below. Ranging over a string replaces each
invalid byte separately, while
[`strings.ToValidUTF8`](https://pkg.go.dev/strings#ToValidUTF8) replaces each
run of invalid bytes once. Neither matches all the examples above. Once you have
a complete valid character,
[`utf8.DecodeRune`](https://pkg.go.dev/unicode/utf8#DecodeRune) decodes it.

{{/lang_is_go}}

{{#lang_is_cpp}}

Write a decoding loop using the rules below. The C++ standard library has no
UTF-8 decoder with this replacement behavior. Copy valid characters' bytes to
the output and use the bytes `[239, 191, 189]` for each `�`.

{{/lang_is_cpp}}

<details>
  <summary>UTF-8 rules for a decoding loop</summary>

Read the buffer from left to right:

- A byte from 0 to 127 is a complete one-byte character.
- Bytes 194–223 start two-byte characters, 224–239 start three-byte characters,
  and 240–244 start four-byte characters.
- Subsequent bytes must be in the range 128–191. Four starting bytes have
  narrower ranges for the second byte, as shown below.
- If the sequence is complete and valid, output that character. If a byte does
  not fit, emit one `�` for the valid prefix so far and resume at the byte that
  did not fit. An incomplete valid prefix at the end also produces one `�`.
- A byte that cannot start a character, including a stray byte from 128–191,
  produces one `�` on its own.

| First byte | Character length in bytes | Allowed second byte |
| ---------- | ------------------------- | ------------------- |
| 224        | 3                         | 160–191             |
| 237        | 3                         | 128–159             |
| 240        | 4                         | 144–191             |
| 244        | 4                         | 128–143             |

These ranges exclude invalid Unicode representations. Follow Unicode's
[maximal subpart rule](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G66453).
The invalid inputs tested here are truncated characters and bytes such as 255.
Java's decoder differs for surrogate encodings, which these tests do not use.

</details>

### Tests

The tester writes `[258, 100]` to `ids.json` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then runs:

```bash
$ ./your_program.sh decode ids.json --merges merges.json
```

Your program must print a JSON string:

```json
"aaabd"
```

Use your JSON library to serialize the string so quotes, backslashes, and
newlines are escaped correctly. `"\uFFFD"` and `"�"` are equivalent JSON
representations.

The tester also checks a complete emoji, a truncated emoji, an emoji split
across two tokens, invalid bytes, characters needing JSON escaping, and the
encoded sentence `to the top of the tree`.
