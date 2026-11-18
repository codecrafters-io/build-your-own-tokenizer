Earlier stages used byte values as the first 256 token IDs and assigned new IDs to learned merges. GPT-2 has its own ID for every token. In this stage, you'll add `--vocab` to `encode` so it can use GPT-2's vocabulary files.

### A vocabulary with its own IDs

GPT-2's ID for a space is 220, even though the space byte is 32.

The `--vocab` argument names a directory containing `vocab.json` and `merges.txt`. This mode automatically uses GPT-2 pre-tokenization, so the command needs neither `--merges` nor `--pretokenize gpt2`. The tester supplies both files, so you need no model weights, API key, or download.

### How the files represent bytes

The files spell tokens using printable characters that stand for bytes, so a space is written as `Ġ`. Build the mapping as follows:

- Byte values 33–126, 161–172, and 174–255 map to the character with the same numeric value.
- Visit the remaining 68 byte values in increasing order, mapping them to U+0100, U+0101, and onward.

`U+` names a character by its hexadecimal number, so U+0100 is character 256. The first 33 remapped bytes are 0 through 32, so byte `n` in that range maps to U+0100 + `n`.

| Original byte | Byte value | Stored as | ID in `vocab.json` |
| --- | --- | --- | --- |
| `!` | 33 | `!` | 0 |
| `a` | 97 | `a` | 64 |
| Tab | 9 | `ĉ` (U+0109) | 197 |
| Newline | 10 | `Ċ` (U+010A) | 198 |
| Space | 32 | `Ġ` (U+0120) | 220 |

[`vocab.json`](https://huggingface.co/gpt2/resolve/main/vocab.json) maps spellings to IDs, such as `{"!": 0, "Hello": 15496, "Ġworld": 995}`. Parse it as JSON so escapes such as `"\u0120"` become real characters.

[`merges.txt`](https://huggingface.co/gpt2/resolve/main/merges.txt) lists pairs of spellings in rank order. Skip blank lines and the initial `#version: 0.2` header. Each remaining line holds two spellings separated by one space. Keep real pairs such as `# #`, even though they start with `#`.

### Encoding with GPT-2's rules

For each chunk from the pre-tokenization pattern you added in earlier stages:

1. Convert the chunk to UTF-8 bytes.
2. Map each byte to its spelling character.
3. Choose the lowest-ranked pair present in the chunk, join its two spellings at every non-overlapping occurrence from left to right, and repeat until no ranked pair remains.
4. Look up each resulting spelling in `vocab.json` to get its ID.

Combine the IDs from all chunks. For ` Hello`:

```text
" Hello" -> bytes 32 72 101 108 108 111
         -> characters Ġ H e l l o
         -> merged spelling ĠHello
         -> vocab.json ID 18435
```

Keep the boundaries. In `Hello\n\nworld`, the two interior newlines are separate chunks, so their IDs stay separate:

```json
[15496, 198, 198, 6894]
```

### Tests

The tester will write `Hello world` to `text.txt`, place GPT-2's two files in `gpt2`, and run:

```bash
$ ./your_program.sh encode text.txt --vocab gpt2
```

Your program must print:

```json
[15496, 995]
```

Those IDs represent `Hello` and ` world`. The tester shows the text behind each ID when a short text passes, and the expected pieces when one fails.

It also checks Hindi text, indented code, blank lines, and `strawberry strawberry`, which must produce:

```json
[301, 1831, 8396, 41236]
```

The first word becomes `st`, `raw`, and `berry`. The second, with its leading space, is the single token ` strawberry`.

The test texts exclude `<|endoftext|>`. Training GPT-2's vocabulary and decoding its IDs are outside this stage.
