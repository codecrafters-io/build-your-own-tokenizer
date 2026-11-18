Your encoder now follows GPT-2's chunk boundaries while using your own merge
rules and IDs. GPT-2's vocabulary supplies the tokens needed to match its
output. In this stage, you'll add `--vocab` to load its vocabulary and merges
for encoding ordinary text.

### Load GPT-2's files

The `--vocab` argument names a directory containing two files:

- [`vocab.json`](https://huggingface.co/gpt2/resolve/main/vocab.json) maps token
  strings to IDs, such as `{"Hello": 15496, "Ġworld": 995}`.
- [`merges.txt`](https://huggingface.co/gpt2/resolve/main/merges.txt) lists
  50,000 pairs of token strings in rank order, after a `#version: 0.2` header.
  The first pair has rank 0.

The token strings use a special character to label each byte. For example, `Ġ`
represents byte 32, so the key `Ġworld` represents the bytes of a space followed
by `world`.

Parse `vocab.json` with your JSON library. It converts escapes such as
`"\u0120"` into the character `Ġ`.

When reading `merges.txt`, skip blank lines and the initial version header. Each
remaining line contains two token strings separated by a space. Keep real merges
such as `# #`, even though they also start with `#`.

### Map bytes to their file labels

Construct a table with one character for each byte:

- Bytes 33–126, 161–172, and 174–255 use the Unicode character with the same
  numeric value.
- Assign the remaining 68 byte values, in increasing order, to characters
  starting at U+0100 (decimal 256) and continuing upward.

Byte 0 maps to U+0100. Check your mapping against these entries:

| Byte    | Value | Character in the files | ID in `vocab.json` |
| ------- | ----- | ---------------------- | ------------------ |
| `!`     | 33    | `!`                    | 0                  |
| `a`     | 97    | `a`                    | 64                 |
| Tab     | 9     | `ĉ` (U+0109)           | 197                |
| Newline | 10    | `Ċ` (U+010A)           | 198                |
| Space   | 32    | `Ġ` (U+0120)           | 220                |

Earlier stages used byte values as byte-token IDs and assigned 256 upward to
merged tokens. Here, look up every final ID in `vocab.json`, including IDs for
single-byte tokens.

Map the original text's UTF-8 bytes, even when the input itself contains a
character such as `Ġ`. These labels are a file representation.

### Encode with the loaded vocabulary

For `encode text.txt --vocab gpt2`:

1. Split the input with the GPT-2 pattern from earlier stages. This mode enables
   that splitting automatically.
2. For each chunk, convert its UTF-8 bytes to character labels. Start with one
   token string per byte.
3. Find the adjacent pair with the lowest rank in `merges.txt`. Join its two
   strings at every non-overlapping occurrence, scanning left to right. Repeat
   until no listed pair remains.
4. Look up each resulting token string in `vocab.json` and concatenate the IDs
   from all chunks in order.

This is the same ranked merge process you already implemented, using strings to
represent the tokens during merging. For example:

```text
" Hello" → bytes 32 72 101 108 108 111
         → labels Ġ H e l l o
         → merged by rank into ĠHello
         → vocab.json ID 18435
```

Keep chunk boundaries intact. In `Hello\n\nworld`, the pattern puts the two
newlines in separate chunks. The output is `[15496, 198, 198, 6894]`. They
cannot become GPT-2's double-newline token 628.

### Tests

The tester writes `Hello world` to `text.txt`, supplies `vocab.json` and
`merges.txt` in a `gpt2` directory, and runs:

```bash
$ ./your_program.sh encode text.txt --vocab gpt2
```

Your program must print:

```json
[15496, 995]
```

For `strawberry strawberry`, the output is:

```json
[301, 1831, 8396, 41236]
```

The first word splits into `st`, `raw`, and `berry`. The second word, including
its leading space, is one token: ` strawberry`.

The tester also checks a Hindi sentence, indented code, a Markdown heading, and
`to the top of the tree`, which now produces six tokens. For short inputs, test
logs show the text behind each ID.

The special token `<|endoftext|>` is outside this stage's scope. Keep the
earlier commands and their merge-file behavior unchanged.
