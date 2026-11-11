In this stage, you'll implement the `decode` command to turn token ids back into
text.

### Your progress

<details>
  <summary>Back to the original text for the example sentence `to the top of the tree`</summary>

Here's how many tokens the sentence `to the top of the tree` takes so far:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens
decode    ██████████████████████  22 bytes, the original text   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

Decoding those 11 ids with the same five merges gives the sentence back.

</details>

### Why decode?

A language model writes its answer as token ids. Decode turns those ids back
into text.

👋 is four UTF-8 bytes, `[240, 159, 145, 139]`. GPT-2 splits them across two
tokens, `[41840, 233]`, ids from its own vocabulary. Token boundaries don't have to line up with character
boundaries: the first token holds `[240, 159, 145]` and the second holds
`[139]`. UTF-8 needs all four bytes to make 👋, so decoding the first token by
itself gives the replacement character `�`. Join the bytes from both tokens
first, and the result is 👋.

Your decoder does the same with any text. Complete characters come back as
themselves. Bytes that stop halfway through a character come back as `�`.

### The `decode` command

The `decode` command parses two JSON files in the formats from earlier stages:
`ids.json` is an array of ids, and `merges.json` is an array of pairs.

1. Build the same vocabulary training would. Id `n` below 256 is the single byte
   `n`. Id `256 + i` is the bytes of the first id in merge `i`, followed by the
   bytes of the second.
2. Join every token's bytes into one buffer.
3. Decode that buffer as UTF-8. Wherever the bytes are invalid, use the
   replacement character U+FFFD.

For example, `[258, 100]` with the three merges from `aaabdaaabac` prints:

```json
"aaabd"
```

`258` expands to `257` and `98`, then to `256`, `97`, and `98`, and finally to
the bytes of `aaab`. `100` is `d`.

`[240, 159, 145, 139]` prints the wave emoji.

### How many replacement characters?

U+FFFD, shown as `�`, is a replacement _character_, not a replacement byte. One
`�` can stand for several bytes. Read the buffer left to right, one character at
a time:

- The first byte says how long the character is. 0–127 is a 1-byte character,
  194–223 starts a 2-byte one, 224–239 a 3-byte one, and 240–244 a 4-byte one.
- Every byte after the first must be a continuation byte, 128–191.
- If the character is complete, output it. If a byte doesn't fit, or the buffer
  ends first, output one `�` for the bytes read so far and start the next
  character at the byte that didn't fit.
- A byte that can't start a character, like 255 or a stray continuation byte,
  is one `�` by itself.

| Bytes        | Prints           | Why                                                                                                                                 |
| ------------ | ---------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `[240, 159]` | `"\uFFFD"`       | 240 starts a 4-byte character and 159 continues it. The buffer ends first, so both bytes are one `�`.                               |
| `[255, 255]` | `"\uFFFD\uFFFD"` | 255 never starts a character, so each 255 is its own `�`.                                                                           |
| `[240, 240]` | `"\uFFFD\uFFFD"` | The second 240 doesn't continue the first, so the first 240 is one `�`. The buffer ends after the second 240, so it is another `�`. |

<details>
  <summary>The exact rule</summary>

For invalid input, the tests in this stage only use cut-off characters and
bytes that never appear in UTF-8, so the rule above is enough. UTF-8 also limits the second byte after
four lead bytes, which rules out overlong encodings, surrogates, and values
above U+10FFFF:

| First byte | Length | Second byte |
| ---------- | ------ | ----------- |
| 224        | 3      | 160–191     |
| 237        | 3      | 128–159     |
| 240        | 4      | 144–191     |
| 244        | 4      | 128–143     |

A second byte outside its range doesn't fit, just like any other byte that
doesn't continue a character. This is the Unicode Standard's
[maximal subpart rule](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G66453).
The built-in decoders in Python, Rust, and TypeScript follow it exactly. Java's
differs only on surrogates, which these tests don't use.

</details>

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
ids of the sentence from "Your progress".

### Notes

- Join every token's bytes before decoding. The half-emoji is one replacement
  character. The program should keep running when the bytes are not valid UTF-8.

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

- Print the text with your JSON library, so quotes, backslashes, and newlines
  are escaped correctly. `"\uFFFD"` and `"�"` are the same JSON string, so
  either passes.
