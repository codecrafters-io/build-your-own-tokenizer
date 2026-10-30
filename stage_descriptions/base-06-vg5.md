In this stage, you'll implement the `decode` command to turn token ids back into
text.

<details>
  <summary>Your progress: back to the original text for the example sentence `to the top of the tree`</summary>

Here's how many tokens the sentence `to the top of the tree` takes so far:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens
decode    ██████████████████████  22 bytes, the original text   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

</details>

### Why decode?

A language model writes its answer as token ids. Decode turns those ids back
into text.

👋 is four UTF-8 bytes. GPT-2 splits them across two tokens, `[41840, 233]`.
The first token holds the first three bytes. The second holds the last byte.
UTF-8 needs all four bytes to make 👋, so decoding the first token by itself
gives the replacement character `�`. Join the bytes from both tokens first, and
the result is 👋.

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

`[240, 159]` is only the first half of the wave emoji. Joined, those two bytes
are one replacement character:

```json
"\uFFFD"
```

`[240, 159, 145, 139]` prints the wave emoji.

A byte that can never appear in UTF-8, like 255, is its own replacement
character. `[255, 255]` prints `"\uFFFD\uFFFD"`.

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
wave emoji, half of the wave emoji, and bytes that are never valid UTF-8.

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

- Write the decoding loop yourself. Ranging over a string gives one U+FFFD per
  invalid byte, and `strings.ToValidUTF8` gives one per run of invalid bytes.

{{/lang_is_go}}

{{#lang_is_cpp}}

- Write the decoding loop yourself. The C++ standard library has no UTF-8
  decoder that replaces invalid bytes.

{{/lang_is_cpp}}

- Print the text with your JSON library, so quotes, backslashes, and newlines
  are escaped correctly. `"\uFFFD"` and `"�"` are the same JSON string, so
  either passes.
