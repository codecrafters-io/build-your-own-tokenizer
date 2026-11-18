Your encoder turns text into token IDs. In this stage, you'll implement
`decode` to recover the text using the same merge rules.

### Your progress

<details>
  <summary>Back to the original text for the example sentence `to the top of the tree`</summary>

Here's the round trip for the sentence `to the top of the tree`:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens
decode    ██████████████████████  22 bytes, the original text   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

Decoding those 11 IDs with the same five merges gives the sentence back.

</details>

### Why decode?

A language model produces token IDs. Decoding makes that output readable.
Each ID refers to bytes in its tokenizer's vocabulary, and those bytes may
contain only part of a character.

👋 is four UTF-8 bytes, `[240, 159, 145, 139]`. GPT-2 splits them across two
tokens, `[41840, 233]`, IDs from its own vocabulary. The first token holds
`[240, 159, 145]` and the second holds `[139]`. UTF-8 needs all four bytes to
make 👋, so decoding the first token by
itself gives the replacement character `�`. Join the bytes from both tokens
first, and the result is 👋.

Your decoder does the same with any text. Complete characters come back as
themselves. Bytes that stop halfway through a character come back as `�`.

### The `decode` command

The `decode` command parses two JSON files in the formats from earlier stages:
`ids.json` is an array of IDs, and `merges.json` is an array of pairs.

IDs below 256 represent their single byte values. ID `256 + i` represents the
bytes of the first ID in merge `i`, followed by the bytes of the second.

Recover each token's bytes and join them in input order before decoding the
complete buffer as UTF-8. Use the replacement character U+FFFD, shown as `�`,
where the bytes are invalid. The command prints the result as a JSON string.

For example, `[258, 100]` with the three merges from `aaabdaaabac` represents
`aaabd`.

`258` expands to `257` and `98`, then to `256`, `97`, and `98`, and finally to
the bytes of `aaab`. `100` is `d`.

`[240, 159, 145, 139]` prints the wave emoji.

### How many replacement characters?

One `�` can stand for several bytes. An incomplete prefix of a valid UTF-8
character becomes one replacement character. A byte that cannot start a
character, such as 255, becomes a replacement character on its own.

| Bytes        | Prints           | Why                                                                                                                                 |
| ------------ | ---------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `[240, 159]` | `"\uFFFD"`       | 240 starts a 4-byte character and 159 continues it. The buffer ends first, so both bytes are one `�`.                               |
| `[255, 255]` | `"\uFFFD\uFFFD"` | 255 never starts a character, so each 255 is its own `�`.                                                                           |
| `[240, 240]` | `"\uFFFD\uFFFD"` | The second 240 doesn't continue the first, so the first 240 is one `�`. The buffer ends after the second 240, so it is another `�`. |

### Tests

The tester will write `[258, 100]` to `ids.json` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then execute your program
like this:

```bash
$ ./your_program.sh decode ids.json --merges merges.json
```

Your program must print the text as a JSON string:

```json
"aaabd"
```

The tester will also run your program on a few other lists, including the full
wave emoji, half of the wave emoji, bytes that are never valid UTF-8, and the
IDs of the sentence from "Your progress".

### Notes

- Invalid UTF-8 must produce replacement characters without stopping the
  program. The malformed inputs in this stage are cut-off characters and bytes
  that can never appear in UTF-8.

{{#lang_is_python}}

- The `bytes.decode` method accepts `errors="replace"` for this replacement
  behavior. Use it on the complete byte buffer with UTF-8 as the encoding.

{{/lang_is_python}}

{{#lang_is_typescript}}

- `new TextDecoder("utf-8", { ignoreBOM: true }).decode(bytes)` replaces
  invalid bytes this way. The option also preserves an initial U+FEFF
  character, which the default decoder removes.

{{/lang_is_typescript}}

{{#lang_is_java}}

- `new String(bytes, StandardCharsets.UTF_8)` handles the invalid inputs in
  this stage. Its behavior differs from the exact rule below for surrogate
  encodings, which these tests don't use.

{{/lang_is_java}}

{{#lang_is_rust}}

- `String::from_utf8_lossy` replaces invalid bytes this way.

{{/lang_is_rust}}

{{#lang_is_go}}

- Write the decoding loop yourself, following "The exact rule" below.
  Ranging over a string gives one U+FFFD per invalid byte, and
  [`strings.ToValidUTF8`](https://pkg.go.dev/strings#ToValidUTF8) gives one per
  run of invalid bytes, so neither matches. Once a character's bytes are
  complete, [`utf8.DecodeRune`](https://pkg.go.dev/unicode/utf8#DecodeRune)
  decodes them.

{{/lang_is_go}}

{{#lang_is_cpp}}

- Write the decoding loop yourself, following "The exact rule" below.
  The C++ standard library has no UTF-8 decoder that replaces
  invalid bytes. Valid characters can be copied to the output as their bytes,
  and each `�` is the 3 bytes 239, 191, 189.

{{/lang_is_cpp}}

- Print the text with your JSON library, so quotes, backslashes, and newlines
  are escaped correctly. `"\uFFFD"` and `"�"` are the same JSON string, so
  either passes.

<details>
  <summary>The exact rule</summary>

If you're implementing UTF-8 replacement yourself, read the joined bytes from
left to right. A leading byte indicates how many bytes a character needs.

A byte with a value from 0 to 127 is a complete one-byte character. Bytes
194–223 start two-byte characters, 224–239 start three-byte characters, and
240–244 start four-byte characters. Following bytes must be continuation
bytes, with values 128–191, subject to the second-byte limits below.

If a character is complete, output it. If a byte doesn't fit, or the buffer
ends first, output one `�` for the valid prefix read so far. Resume at the byte
that didn't fit. A byte that cannot start a character, such as 255 or a stray
continuation byte, produces one `�` on its own.

UTF-8 limits the second byte after four lead bytes. These limits exclude
overlong encodings, which use more bytes than necessary, and numbers outside
the range of valid Unicode characters:

| First byte | Length | Second byte |
| ---------- | ------ | ----------- |
| 224        | 3      | 160–191     |
| 237        | 3      | 128–159     |
| 240        | 4      | 144–191     |
| 244        | 4      | 128–143     |

A second byte outside its range doesn't fit, just like any other byte that
doesn't continue a character. This is the Unicode Standard's
[maximal subpart rule](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G66453).
It also excludes surrogate encodings and values above U+10FFFF. Surrogates are
reserved Unicode numbers that cannot be encoded as UTF-8 characters.

</details>
