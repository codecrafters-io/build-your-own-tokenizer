In this stage, you'll add `--vocab` to encode text with GPT-2's merge rules and
token IDs instead of your custom numbering.

### GPT-2's files

`--vocab` names a directory containing `vocab.json` and `merges.txt`. It enables
GPT-2 pretokenization without `--pretokenize gpt2` or `--merges`. Keep the
existing `--merges` mode working.

GPT-2 labels bytes with Unicode characters. Bytes 33–126, 161–172, and 174–255
map to characters with that numeric code point. The other 68 bytes, in
increasing order, map to U+0100 and upward. Byte `n` from 0–32 maps to U+0100 +
`n`. These labels are neither token IDs nor necessarily the original text's
characters:

| Byte    | Value | Stored as    | ID in `vocab.json` |
| ------- | ----- | ------------ | ------------------ |
| `!`     | 33    | `!`          | 0                  |
| `a`     | 97    | `a`          | 64                 |
| Tab     | 9     | `ĉ` (U+0109) | 197                |
| Newline | 10    | `Ċ` (U+010A) | 198                |
| Space   | 32    | `Ġ` (U+0120) | 220                |

Parse [`vocab.json`](https://huggingface.co/gpt2/resolve/main/vocab.json) as
JSON, converting escapes such as `"\u0120"` to `Ġ`. It maps label strings to
IDs, like `{"!": 0, "Hello": 15496, "Ġworld": 995}`.

[`merges.txt`](https://huggingface.co/gpt2/resolve/main/merges.txt) contains
50,000 rules in rank order. Skip blank lines and the first line,
`#version: 0.2`. Split remaining lines on a single space. Other `#` lines are
real rules, including `# #`.

Ranks determine merge order. Unlike your custom byte IDs 0–255 and `256 + i`
merged IDs, every output ID now comes from `vocab.json`, even for unmerged
bytes.

### Encoding text

1. Apply your GPT-2 pretokenization pattern.
2. Convert each chunk to UTF-8 bytes, then map those bytes to labels. Treat each
   label as one symbol, even if it occupies multiple bytes in the vocabulary
   file.
3. Within each chunk, repeatedly merge the lowest-ranked present pair, joining
   label strings from left to right without overlapping matches.
4. When no ranked pair remains, append each final string's vocabulary ID to the
   output.

```text
" Hello" → bytes 32 72 101 108 108 111
         → characters Ġ H e l l o
         → merged by rank into ĠHello
         → vocab.json id 18435
```

In `Hello\n\nworld`, `\n` denotes a newline. The two newlines occupy separate
chunks, preventing a merge into double-newline token 628:

```json
[15496, 198, 198, 6894]
```

Regex libraries can classify letters, numbers, and whitespace differently across
Unicode versions. Loading the vocabulary does not change those classifications.

### Token boundaries

`Hello world` encodes as:

```json
[15496, 995]
```

A leading space, ` Hello world`, changes the first ID:

```json
[18435, 995]
```

`strawberry strawberry` encodes as:

```json
[301, 1831, 8396, 41236]
```

The first word splits into `st`, `raw`, and `berry`. The space-prefixed
` strawberry` is one token.

### Your progress

<details>
  <summary>6 tokens for `to the top of the tree`</summary>

Its six chunks each have one GPT-2 token:

```text
bytes     ██████████████████████  22 tokens
split     █████████████           13 tokens
GPT-2     ██████                   6 tokens   ← this stage
```

</details>

### Tests

The tester puts `Hello world` in `text.txt`, supplies GPT-2's files in `gpt2`,
and runs:

```bash
$ ./your_program.sh encode text.txt --vocab gpt2
```

Print the token IDs as a JSON array to stdout:

```json
[15496, 995]
```

Other tests cover `strawberry strawberry`, Hindi, indented code, and the
progress sentence. No test text contains `<|endoftext|>`. Special-token handling
is not required.

For short lists of valid IDs, logs can show the text represented by each token,
such as `"Hello" " world"`.

### Notes

- Check your byte-to-character table if a lookup fails.
