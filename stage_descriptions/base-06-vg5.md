In this stage, you'll implement the `decode` command to turn token IDs back into
text.

### Your progress

<details>
  <summary>Back to the original text for the example sentence `to the top of the tree`</summary>

Here's the running comparison for `to the top of the tree`:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens
decode    ██████████████████████  22 bytes, the original text   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

Decoding those 11 IDs with the same five merges gives the sentence back.

</details>

### Why decode?

A language model writes its answer as token IDs. Decode turns those IDs back
into text.

👋 is four UTF-8 bytes, `[240, 159, 145, 139]`. GPT-2 splits them across two
tokens, `[41840, 233]`, IDs from its own vocabulary. Token boundaries don't have
to line up with character boundaries: the first token holds `[240, 159, 145]`
and the second holds `[139]`. UTF-8 needs all four bytes to make 👋, so decoding
the first token by itself gives the replacement character `�`. Join the bytes
from both tokens first, and the result is 👋.

Complete characters come back as themselves, and bytes that stop halfway through
a character come back as `�`.

### The `decode` command

The `decode` command reads `ids.json` and `merges.json` in the earlier formats.

1. Build the same vocabulary training would. ID `n` below 256 is the single byte
   `n`. ID `256 + i` is the bytes of the first ID in merge `i`, followed by the
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

### How many replacement characters?

Unicode identifies the replacement character `�` as U+FFFD. One `�` can stand
for several invalid bytes. Read the buffer left to right, one character at a
time:

- The first byte says how long the character is. 0–127 is a 1-byte character,
  194–223 starts a 2-byte one, 224–239 a 3-byte one, and 240–244 a 4-byte one.
- Every byte after the first must be a continuation byte, 128–191.
- If the character is complete, output it. If a byte doesn't fit, or the buffer
  ends first, output one `�` for the bytes read so far and start the next
  character at the byte that didn't fit.
- A byte that can't start a character, like 255 or a stray continuation byte, is
  one `�` by itself.

| Bytes        | Prints           | Why                                                                                                   |
| ------------ | ---------------- | ----------------------------------------------------------------------------------------------------- |
| `[240, 159]` | `"\uFFFD"`       | 240 starts a 4-byte character and 159 continues it. The buffer ends first, so both bytes are one `�`. |
| `[255, 255]` | `"\uFFFD\uFFFD"` | 255 never starts a character, so each 255 is its own `�`.                                             |

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

The tester will also run your program on other lists, including the full and
half wave emoji, bytes that are never valid UTF-8, and the sentence from "Your
progress".

### Notes

- Join every token's bytes before decoding. The half-emoji is one replacement
  character. The program should keep running when the bytes are not valid UTF-8.

- This stage's invalid inputs are cut-off characters and bytes that never appear
  in UTF-8. The Unicode Standard's
  [maximal subpart rule](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G66453)
  covers the rest, including overlong encodings, surrogate values, and values
  above U+10FFFF.

{{#lang_is_python}}

- `bytes.decode("utf-8", errors="replace")` replaces invalid bytes this way.

{{/lang_is_python}}

{{#lang_is_typescript}}

- `new TextDecoder("utf-8", { ignoreBOM: true }).decode(bytes)` replaces invalid
  bytes this way. `ignoreBOM: true` preserves an initial U+FEFF character, whose
  UTF-8 bytes are `[239, 187, 191]`. The default decoder removes it.

{{/lang_is_typescript}}

{{#lang_is_java}}

- `new String(bytes, StandardCharsets.UTF_8)` gives the required replacements
  for this stage's test inputs. It groups encoded surrogate values differently
  from the full rule above. Those inputs are outside this stage's tests.

{{/lang_is_java}}

{{#lang_is_rust}}

- `String::from_utf8_lossy` replaces invalid bytes this way.

{{/lang_is_rust}}

{{#lang_is_go}}

- Check that your decoder follows the grouping rule in "How many replacement
  characters?". Ranging over a string gives one U+FFFD per invalid byte, and
  [`strings.ToValidUTF8`](https://pkg.go.dev/strings#ToValidUTF8) gives one per
  run of invalid bytes, so neither matches. Once a character's bytes are
  complete, [`utf8.DecodeRune`](https://pkg.go.dev/unicode/utf8#DecodeRune)
  decodes them.

{{/lang_is_go}}

{{#lang_is_cpp}}

- Check that your decoder follows the grouping rule in "How many replacement
  characters?". The standard library has no UTF-8 decoder that replaces invalid
  bytes this way. Valid characters can be copied to the output as their bytes,
  and each `�` is the 3 bytes 239, 191, 189.

{{/lang_is_cpp}}

- Print the text with your JSON library, so quotes, backslashes, and newlines
  are escaped correctly. `"\uFFFD"` and `"�"` are the same JSON string, so
  either passes.
