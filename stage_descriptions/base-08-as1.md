Your encoder can split text into chunks and apply custom merges. In this stage,
add `--vocab` to load GPT-2's merge rules and vocabulary, so the output uses
GPT-2's token IDs instead of your custom numbering.

### Matching GPT-2

GPT-2's files supply 50,000 merge rules and the ID of each token. With those
files and the previous stage's pretokenization, `Hello world` encodes as:

```json
[15496, 995]
```

The text ` Hello world`, with a leading space, encodes as:

```json
[18435, 995]
```

The leading space changes which merges apply. Another example,
`strawberry strawberry`, encodes as:

```json
[301, 1831, 8396, 41236]
```

The first `strawberry` has no space before it and splits into `st`, `raw`, and
`berry`. The second one includes its space, and ` strawberry` is a single token.

The Unicode-version limits from the previous stage still apply. Loading the
vocabulary does not change which characters your regex library classifies as
letters, numbers, or whitespace.

### GPT-2's files

The `--vocab` flag names the directory that holds `vocab.json` and `merges.txt`.
This mode always uses GPT-2 pretokenization; it does not need
`--pretokenize gpt2` or `--merges`. Keep the earlier `--merges` mode working as
before.

GPT-2's files label each byte with a Unicode character. Bytes 33–126, 161–172,
and 174–255 map to characters with the same numeric code point. The other 68
bytes, in increasing order, map to U+0100 and upward. The first 33 of them are
bytes 0 to 32, so a byte `n` below 33 is stored as U+0100 + `n`. These
characters label bytes; they are not token IDs or necessarily the original
text's characters.

Check your table against these five bytes:

| Byte    | Value | Stored as    | ID in `vocab.json` |
| ------- | ----- | ------------ | ------------------ |
| `!`     | 33    | `!`          | 0                  |
| `a`     | 97    | `a`          | 64                 |
| Tab     | 9     | `ĉ` (U+0109) | 197                |
| Newline | 10    | `Ċ` (U+010A) | 198                |
| Space   | 32    | `Ġ` (U+0120) | 220                |

[`vocab.json`](https://huggingface.co/gpt2/resolve/main/vocab.json) is a JSON
object that maps each token's label string to its ID, like
`{"!": 0, "Hello": 15496, "Ġworld": 995}`. Parse it with your language's JSON
library. Its keys use JSON escapes such as `"\u0120"` for `Ġ`, and the library
turns them back into characters.

[`merges.txt`](https://huggingface.co/gpt2/resolve/main/merges.txt) lists pairs
of label strings in rank order. Skip blank lines and the first line,
`#version: 0.2`. Split each remaining line on a single space. Do not skip other
lines beginning with `#`: `# #` is a real merge rule.

A rule's rank determines when to merge, not which ID to print. Previously, IDs
0–255 were byte values and learned tokens used `256 + i`. Now every output ID,
even for an unmerged byte, must come from `vocab.json`.

### Encoding with GPT-2's vocabulary

1. Split the input with the previous stage's pattern.
2. Convert each chunk to UTF-8 bytes and map each byte to its label character.
   Treat each label as one initial symbol, even if that label occupies multiple
   bytes in the vocabulary file.
3. Within each chunk, repeatedly merge the currently present pair with the
   lowest rank in `merges.txt`. Join non-overlapping occurrences from left to
   right, concatenating their label strings.
4. When no ranked pair remains, look up each resulting string in `vocab.json`
   and append its ID to the output.

For example:

```text
" Hello" → bytes 32 72 101 108 108 111
         → characters Ġ H e l l o
         → merged by rank into ĠHello
         → vocab.json id 18435
```

Splitting first still matters. In `Hello\n\nworld`, where `\n` represents a
newline, the two newlines are separate chunks, so they stay two newline tokens
instead of becoming GPT-2's double-newline token, 628:

```json
[15496, 198, 198, 6894]
```

### Your progress

<details>
  <summary>6 tokens for the example sentence `to the top of the tree`</summary>

The sentence splits into 6 chunks, and GPT-2's vocabulary has one token for each
of them. With its merges and IDs, the result is now 6 tokens:

```text
bytes     ██████████████████████  22 tokens
split     █████████████           13 tokens
GPT-2     ██████                   6 tokens   ← this stage
```

</details>

### Tests

The tester will write `Hello world` to `text.txt`, put GPT-2's `vocab.json` and
`merges.txt` in a `gpt2` directory, and execute your program like this:

```bash
$ ./your_program.sh encode text.txt --vocab gpt2
```

Your program must print the token IDs as a JSON array to stdout:

```json
[15496, 995]
```

The tester's log can show the text pieces behind short lists of valid IDs, such
as `"Hello" " world"`. On a mismatch, use these pieces to compare token
boundaries.

The tester will also run your program on a few other files, including
`strawberry strawberry`, a Hindi sentence, a short program with indented lines,
and the sentence from "Your progress". None of the test texts contain
`<|endoftext|>`; special-token handling is not required in this stage.

### Notes

- If a lookup in `vocab.json` fails, check your byte-to-character table and make
  sure you concatenated the labels rather than converting them back to bytes
  before the lookup.
