In this stage, you'll add `--vocab` to `encode` so it can use GPT-2's
vocabulary files. Your encoder already splits text like GPT-2 and applies
ranked merges within each chunk. Loading its vocabulary will let you produce
the same token IDs.

### A vocabulary with its own IDs

Earlier stages used byte values as the first 256 token IDs, then assigned
256, 257, and so on to learned merges. GPT-2 has its own ID for every token.
For example, its ID for a space is 220, even though the space byte is 32.

The `--vocab` argument names a directory containing `vocab.json` and
`merges.txt`. This mode automatically uses GPT-2 pre-tokenization. It does
not require `--merges` or `--pretokenize gpt2` on the command line.

The tester supplies both files. You don't need model weights, an API key, or a
download during execution. Your existing `--merges` mode continues to use the
JSON format from earlier stages.

### How the files represent bytes

GPT-2's files spell tokens using printable characters that stand for bytes.
These characters are a storage representation. A space in the input, for
example, is written as `Ġ` in a token's vocabulary spelling.

Build the byte-to-character mapping as follows:

- Byte values 33–126, 161–172, and 174–255 map to the Unicode character with
  the same numeric value.
- Visit the remaining 68 byte values in increasing order. Map them to the
  characters U+0100, U+0101, and onward.

The `U+` notation names a Unicode character by its hexadecimal number.
U+0100 is character number 256. The first 33 remapped bytes are 0 through 32,
so byte `n` in that range maps to U+0100 + `n`.

These examples distinguish the original byte, its spelling in the files, and
its final token ID:

| Original byte | Byte value | Stored as | ID in `vocab.json` |
| --- | --- | --- | --- |
| `!` | 33 | `!` | 0 |
| `a` | 97 | `a` | 64 |
| Tab | 9 | `ĉ` (U+0109) | 197 |
| Newline | 10 | `Ċ` (U+010A) | 198 |
| Space | 32 | `Ġ` (U+0120) | 220 |

[`vocab.json`](https://huggingface.co/gpt2/resolve/main/vocab.json) maps
token spellings to IDs, such as
`{"!": 0, "Hello": 15496, "Ġworld": 995}`. Parse it as JSON so escapes such
as `"\u0120"` become their actual characters.

[`merges.txt`](https://huggingface.co/gpt2/resolve/main/merges.txt) lists
pairs of token spellings in rank order. Skip blank lines and the initial
`#version: 0.2` header. Each remaining line contains two spellings separated
by one space. Keep real pairs such as `# #`, even though they start with `#`.

### Encoding with GPT-2's rules

For each chunk from the previous stage's pattern:

1. Convert the chunk to UTF-8 bytes.
2. Map each byte to its vocabulary spelling character.
3. Choose the lowest-ranked pair present in the chunk, then concatenate its
   two spellings at every non-overlapping occurrence from left to right.
   Repeat until no ranked pair remains.
4. Look up each resulting spelling in `vocab.json` to get its token ID.

Combine the IDs from all chunks into one output list. For the chunk
` Hello`, the transformations are:

```text
" Hello" -> bytes 32 72 101 108 108 111
         -> characters Ġ H e l l o
         -> merged spelling ĠHello
         -> vocab.json ID 18435
```

Keep the chunk boundaries. In `Hello\n\nworld`, where `\n` denotes a newline,
the two interior newlines are separate chunks. Their IDs remain separate:

```json
[15496, 198, 198, 6894]
```

### Tests

The tester will write `Hello world` to `text.txt`, place GPT-2's two files in
the `gpt2` directory, and run:

```bash
$ ./your_program.sh encode text.txt --vocab gpt2
```

Your program must print:

```json
[15496, 995]
```

Those IDs represent `Hello` and ` world`. When a short text passes, the
tester shows the text pieces behind the IDs. When one fails, it compares the
expected pieces with yours.

The tester also checks Hindi text, indented code, blank lines, and
`strawberry strawberry`. For that last input, the expected result is:

```json
[301, 1831, 8396, 41236]
```

The first word becomes `st`, `raw`, and `berry`. The second word, including
its leading space, is the single token ` strawberry`. The vocabulary now
lets you reproduce that difference.

The test texts exclude the special token `<|endoftext|>`. This stage adds
GPT-2 encoding. Training GPT-2's vocabulary and decoding its IDs are outside
the current task.

If a vocabulary lookup fails, check the byte-to-character table first. A
spelling that doesn't match `vocab.json` usually means a byte was mapped to the
wrong character.

<details>
  <summary>Your progress: matching GPT-2 on the example sentence</summary>

With GPT-2's vocabulary, `to the top of the tree` produces:

```json
[1462, 262, 1353, 286, 262, 5509]
```

Its six tokens are `to`, ` the`, ` top`, ` of`, ` the`, and ` tree`.
You started with 22 byte IDs and can now encode the sentence with GPT-2's
own vocabulary.

</details>
