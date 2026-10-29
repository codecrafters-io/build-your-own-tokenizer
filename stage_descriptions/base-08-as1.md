In this stage, you'll produce GPT-2's token ids from its vocabulary files.

### Vocabulary

In earlier stages, a token id below 256 was the byte itself. GPT-2's files do
not store raw bytes. Each byte is renamed to a printable character, and the
token id comes from
[`vocab.json`](https://huggingface.co/gpt2/resolve/main/vocab.json). The
`--vocab` flag names the directory that holds `vocab.json` and `merges.txt`.

`vocab.json` is a JSON object that maps each token to its id, like
`{"!": 0, "Hello": 15496, "Ġworld": 995}`. Parse it with your language's JSON
library. Its keys use JSON escapes such as `"\u0120"` for `Ġ`, and the library
turns them back into characters.

[`merges.txt`](https://huggingface.co/gpt2/resolve/main/merges.txt) lists pairs
in rank order, after a `#version` line. Bytes 33–126, 161–172, and 174–255 are
printable, so they map to themselves. The other 68 bytes, in increasing order,
map to U+0100 and upward. A space is byte 32, which becomes `Ġ` and token 220.
`!` is token 0. `a` is token 64. A newline is token 198.

Split the text with the same pattern as earlier stages. For each chunk, map
UTF-8 bytes through that table, merge the resulting characters using the ranks
in `merges.txt`, and look up each piece in `vocab.json`:

```text
" Hello" → bytes 32 72 101 108 108 111
         → characters Ġ H e l l o
         → merged by rank into ĠHello
         → vocab.json id 18435
```

For example, the text `Hello world` encodes as:

```json
[15496, 995]
```

The text ` Hello world`, with a leading space, encodes as:

```json
[18435, 995]
```

Splitting first still matters. In `Hello\n\nworld`, the two newlines are
separate chunks, so they stay two newline tokens instead of becoming GPT-2's
double-newline token, 628:

```json
[15496, 198, 198, 6894]
```

### Tests

The tester will write `Hello world` to `text.txt`, put GPT-2's `vocab.json` and
`merges.txt` in a `gpt2` directory, and execute your program like this:

```bash
$ ./your_program.sh encode text.txt --vocab gpt2
```

Your program must print the token ids as a JSON array:

```json
[15496, 995]
```

The tester will also run your program on a few other files, including
` Hello world` with a leading space, a Hindi sentence, and a short program with
indented lines.

### Notes

- In `merges.txt`, skip blank lines and only the first line, `#version: 0.2`.
  Real merges such as `# #` also start with `#`. Split each remaining line on a
  single space.
- None of the hidden texts contain `<|endoftext|>`.
- Ids from earlier stages are byte values. Here, look every token up in
  `vocab.json`. If a lookup fails, check your byte-to-character table.
