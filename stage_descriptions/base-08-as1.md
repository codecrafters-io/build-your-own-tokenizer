In this stage, you'll implement the `--vocab` option to produce GPT-2's exact
token ids from its vocabulary files.

### Your progress

<details>
  <summary>6 tokens for the example sentence `to the top of the tree`</summary>

Here's how many tokens the sentence `to the top of the tree` takes so far:

```text
bytes     ██████████████████████  22 tokens
split     █████████████           13 tokens
GPT-2     ██████                   6 tokens   ← this stage
```

The sentence splits into 6 chunks, and GPT-2's vocabulary has one token for
each of them.

</details>

### Matching GPT-2

Your encoder already splits text like GPT-2. The last piece is GPT-2's own
vocabulary: its 50,000 merges and the id of every token. With those, the text
`Hello world` encodes as:

```json
[15496, 995]
```

The text ` Hello world`, with a leading space, encodes as:

```json
[18435, 995]
```

These are the same ids OpenAI's tokenizer produces, and you can watch GPT-2's
quirks happen. The text `strawberry strawberry` encodes as:

```json
[301, 1831, 8396, 41236]
```

The first `strawberry` has no space before it and splits into `st`, `raw`, and
`berry`. The second one includes its space, and ` strawberry` is a single token.

### GPT-2's files

The `--vocab` flag names the directory that holds `vocab.json` and `merges.txt`.
GPT-2's files store each byte as a printable character. Bytes 33–126, 161–172,
and 174–255 are printable, so they map to themselves. The other 68 bytes, in
increasing order, map to U+0100 and upward. The first 33 of them are bytes 0 to
32, so a byte `n` below 33 is stored as U+0100 + `n`.

Check your table against these five bytes:

| Byte    | Value | Stored as    | Id in `vocab.json` |
| ------- | ----- | ------------ | ------------------ |
| `!`     | 33    | `!`          | 0                  |
| `a`     | 97    | `a`          | 64                 |
| Tab     | 9     | `ĉ` (U+0109) | 197                |
| Newline | 10    | `Ċ` (U+010A) | 198                |
| Space   | 32    | `Ġ` (U+0120) | 220                |

[`vocab.json`](https://huggingface.co/gpt2/resolve/main/vocab.json) is a JSON
object that maps each token to its id, like
`{"!": 0, "Hello": 15496, "Ġworld": 995}`. Parse it with your language's JSON
library. Its keys use JSON escapes such as `"\u0120"` for `Ġ`, and the library
turns them back into characters.

[`merges.txt`](https://huggingface.co/gpt2/resolve/main/merges.txt) lists pairs
in rank order, after a `#version` line.

Split the text with the same pattern as earlier stages. For each chunk, map its
UTF-8 bytes to characters, merge the characters using the ranks in `merges.txt`,
and look up each piece in `vocab.json`:

```text
" Hello" → bytes 32 72 101 108 108 111
         → characters Ġ H e l l o
         → merged by rank into ĠHello
         → vocab.json id 18435
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

When a short text passes, the tester's log shows the piece of text behind each
id, like `"Hello" " world"`. When one fails, it shows the expected pieces next
to yours.

The tester will also run your program on a few other files, including
`strawberry strawberry`, a Hindi sentence, a short program with indented lines,
and the sentence from "Your progress".

### Notes

- In `merges.txt`, skip blank lines and only the first line, `#version: 0.2`.
  Real merges such as `# #` also start with `#`. Split each remaining line on a
  single space.
- None of the hidden texts contain `<|endoftext|>`.
- Ids from earlier stages are byte values. Here, look every token up in
  `vocab.json`. If a lookup fails, check your byte-to-character table.
