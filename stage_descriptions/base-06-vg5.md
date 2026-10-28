In this stage, you'll turn token ids back into text, including bytes that are not valid UTF-8.

### Decoding

In earlier stages, you encoded text into token ids. Build the same vocabulary training would. Id `n` below 256 is the single byte `n`. Id `256 + i` is the concatenation of the two ids in merge `i`. Join every token's bytes into one buffer, then decode that buffer as UTF-8. Where the bytes are not valid, use the replacement character U+FFFD.

`[258, 100]` with the three merges from `aaabdaaabac` prints:

```json
"aaabd"
```

`[240, 159]` is only the first half of the wave emoji. Joined, those two bytes are one replacement character:

```json
"\uFFFD"
```

`[240, 159, 145, 139]` prints the wave emoji.

### Tests

The tester runs:

```bash
$ ./your_program.sh decode ids.json --merges merges.json
```

`[258, 100]` with the three merges from `aaabdaaabac` prints:

```json
"aaabd"
```

The tester checks that:

- `[258, 100]` prints `"aaabd"`
- `[240, 159]` prints one replacement character, `"\uFFFD"`
- `[240, 159, 145, 139]` prints the wave emoji

### Notes

- Join every token's bytes before decoding. The half-emoji is one replacement character.
- `stdout` is a JSON string, so the replacement character stays exact.
- The program should keep running when the bytes are not valid UTF-8.
