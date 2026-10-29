In this stage, you'll turn token ids back into text, including bytes that are
not valid UTF-8.

### Decoding

In earlier stages, you encoded text into token ids. The `decode` command parses
two JSON files in the formats from earlier stages: `ids.json` is an array of
ids, and `merges.json` is an array of pairs.

Build the same vocabulary that training would. Id `n` below 256 is the single
byte `n`. Id `256 + i` is the bytes of the first id in merge `i`, followed by
the bytes of the second. Join every token's bytes into one buffer, then decode
that buffer as UTF-8. Where the bytes are not valid, use the replacement
character U+FFFD.

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
- Python's `decode(errors="replace")`, JavaScript's `TextDecoder`, Java's
  `new String(bytes, UTF_8)`, and Rust's `String::from_utf8_lossy` replace
  invalid bytes this way. Go's built-ins don't: `range` gives one U+FFFD per
  byte, and `strings.ToValidUTF8` gives one per run. In Go and C++, write the
  decoding loop yourself.
- Print the text with your JSON library, so quotes, backslashes, and newlines
  are escaped correctly. `"\uFFFD"` and `"�"` are the same JSON string, so
  either passes.
