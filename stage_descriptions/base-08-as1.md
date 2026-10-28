In this stage, you'll produce GPT-2's token ids from its vocabulary files.

### Vocabulary

In earlier stages, a token id below 256 was the byte itself. GPT-2's files do not store raw bytes. Each byte is renamed to a printable character, and the token id comes from [`vocab.json`](https://huggingface.co/gpt2/resolve/main/vocab.json).

[`merges.txt`](https://huggingface.co/gpt2/resolve/main/merges.txt) lists pairs in rank order, after a `#version` line. Printable bytes map to themselves. The other 68 bytes, in increasing order, map to U+0100 and upward. A space is byte 32, which becomes `Ġ` and token 220. `!` is token 0. `a` is token 64. A newline is token 198.

Split the text with the same pattern as earlier stages. For each chunk, map UTF-8 bytes through that table, merge in that character space using the ranks in `merges.txt`, and look up each piece in `vocab.json`.

The text `Hello world` encodes as:

```json
[15496, 995]
```

The text ` Hello world`, with a leading space, encodes as:

```json
[18435, 995]
```

### Tests

The tester runs:

```bash
$ ./your_program.sh encode text.txt --vocab gpt2/
```

`gpt2/` contains `vocab.json` and `merges.txt`. For `Hello world`, `stdout` is:

```json
[15496, 995]
```

` Hello world`, with a leading space, is:

```json
[18435, 995]
```

The tester checks that:

- `Hello world` prints `[15496, 995]`
- ` Hello world` prints `[18435, 995]`
- fifty hidden texts, including Hindi, Tamil, code, emoji, and whitespace, match GPT-2

### Notes

- Skip blank lines and a leading line that starts with `#`. Split each remaining line on a single space.
- None of the hidden texts contain `<|endoftext|>`.
- Ids from earlier stages are byte values. Here, look every token up in `vocab.json`.
