In this stage, you'll implement the `--vocab` option to produce GPT-2's exact
token IDs from its vocabulary files.

### Your progress

<details>
  <summary>6 tokens for the example sentence `to the top of the tree`</summary>

Here's how many tokens the exact sentence `to the top of the tree`, with no
trailing newline, takes so far:

```text
bytes     ██████████████████████  22 tokens
split     █████████████           13 tokens
GPT-2     ██████                   6 tokens   ← this stage
```

The sentence splits into `["to", " the", " top", " of", " the", " tree"]`. These
six chunks each happen to become one GPT-2 token, giving:

```json
[1462, 262, 1353, 286, 262, 5509]
```

Other chunks can still produce several tokens. This result uses GPT-2's
published rules and IDs, not the five rules learned from this sentence in
earlier stages. The goal here is exact compatibility with GPT-2.

</details>

### Matching GPT-2

With `--vocab gpt2`, `encode` must automatically split text with the GPT-2
pattern from earlier stages, even without `--pretokenize gpt2`. The directory
`gpt2` holds `vocab.json` and `merges.txt`, which supply GPT-2's token IDs and
50,000 ranked merges. The earlier commands, including `train` and `decode`, and
the `encode --merges` modes, with or without pre-tokenization, keep their
earlier behavior.

In earlier stages, the initial token IDs were byte values 0–255, and learned
pair `i` created ID `256 + i`. GPT-2 assigns IDs through its vocabulary file
instead. Even a single-byte token can have a different ID, as you'll see below.
With GPT-2's files, `Hello world` encodes as:

```json
[15496, 995]
```

The text ` Hello world`, with a leading space, encodes as:

```json
[18435, 995]
```

These are the same IDs OpenAI's GPT-2 tokenizer produces, and you can watch its
quirks happen. The text `strawberry strawberry` encodes as:

```json
[301, 1831, 8396, 41236]
```

The first `strawberry` has no space before it and splits into `st`, `raw`, and
`berry`. The second one includes its space, and ` strawberry` is a single token.

### Bytes as stored symbols

GPT-2's files spell tokens using Unicode **symbols**, with one distinct symbol
for each possible byte value. A token's symbol string is an internal, reversible
spelling of its bytes. It isn't the visible input text or the integer ID you'll
print. A merged token's string can contain several symbols.

Build a byte-to-symbol table. For byte values in the inclusive ranges 33–126,
161–172, and 174–255, use the Unicode character whose code point equals that
value. For example, byte 97 maps to `a` (U+0061), and byte 161 maps to `¡`
(U+00A1). This is a choice of labels, not UTF-8 decoding: byte 161 alone isn't a
valid UTF-8 encoding, but it still gets the symbol `¡`.

The other 68 byte values, in increasing order, map to U+0100 and upward. The
first 33 of them are bytes 0 to 32, so a byte `n` below 33 is stored as the
character at code point U+0100 + `n`. This keeps literal spaces and control
characters out of the stored strings.

Check your table against these five bytes:

| Byte    | Value | Stored as    | ID in `vocab.json` |
| ------- | ----- | ------------ | ------------------ |
| `!`     | 33    | `!`          | 0                  |
| `a`     | 97    | `a`          | 64                 |
| Tab     | 9     | `ĉ` (U+0109) | 197                |
| Newline | 10    | `Ċ` (U+010A) | 198                |
| Space   | 32    | `Ġ` (U+0120) | 220                |

### Loading GPT-2's files

[`vocab.json`](https://huggingface.co/gpt2/resolve/main/vocab.json) is a JSON
object that maps each token's symbol string to its integer ID, like
`{"!": 0, "Hello": 15496, "Ġworld": 995}`. Parse it with your language's JSON
library. Its keys use JSON escapes such as `"\u0120"` for `Ġ`, and the library
turns them back into characters.

[`merges.txt`](https://huggingface.co/gpt2/resolve/main/merges.txt) lists
ordered pairs of symbol strings in rank order. Its first lines are:

```text
#version: 0.2
Ġ t
Ġ a
h e
```

Skip the first physical line only if it starts with `#version`. Skip empty
lines, and split each remaining pair line at its single literal space (U+0020).
Do not discard all lines starting with `#`: `# #` is a real pair that merges
into `##`.

Store a mapping from each ordered pair of strings to its zero-based rank among
the pair lines. Thus, `("Ġ", "t")` has rank 0 and `("Ġ", "a")` has rank 1. Each
side can be a multi-symbol string built by earlier merges. A rank chooses which
pair to merge first. It is not an output token ID.

### Encoding with string pairs

Use the ranked procedure you already know, but keep the current tokens as symbol
strings rather than integer IDs:

1. Split the original Unicode text with the GPT-2 pattern. Convert each chunk
   separately to UTF-8 bytes, then replace each byte with its symbol from the
   table. Start with one single-symbol string per byte.
2. Among the currently neighboring string pairs that appear in the rank mapping,
   choose the lowest-ranked pair. Replace its non-overlapping matches from left
   to right by concatenating the two strings. For example, `"Ġ"` followed by
   `"t"` becomes `"Ġt"`. Do not add integers or allocate a new `256 + i` ID.
   Repeat on the updated strings until no ranked pair matches.
3. Look up every remaining token string in `vocab.json` to get its integer ID.
   Concatenate the ID arrays in chunk order, without merging across chunks.

For the chunk ` Hello`, the stages of representation are:

```text
" Hello" → bytes 32 72 101 108 108 111
         → symbol strings "Ġ" "H" "e" "l" "l" "o"
         → merged by rank into ĠHello
         → vocab.json ID 18435
```

Splitting first still matters. In `Hello\n\nworld`, where `\n` denotes an actual
newline, the two newlines are separate chunks, so they stay two newline tokens
instead of becoming GPT-2's double-newline token, 628:

```json
[15496, 198, 198, 6894]
```

### Tests

The tester will write `Hello world` to `text.txt`, with no trailing newline, put
GPT-2's `vocab.json` and `merges.txt` in a `gpt2` directory, and execute your
program like this:

```bash
$ ./your_program.sh encode text.txt --vocab gpt2
```

Your program must print the token IDs as a JSON array:

```json
[15496, 995]
```

When a short text passes, the tester's log shows the piece of text behind each
ID, like `"Hello" " world"`. When one fails, it shows the expected pieces next
to yours.

The tester will also run your program on a few other files, including
`strawberry strawberry`, a Hindi sentence, a short program with indented lines,
a Markdown heading such as `## Title`, and the sentence from "Your progress".
The heading uses the real `# #` merge to produce token ID 2235 for `##`.

### Notes

- Print only the final JSON array to stdout, and send debug output to stderr.
- None of the hidden texts contain `<|endoftext|>`.
- If a vocabulary lookup fails, check your byte-to-symbol table. A space is
  stored as `Ġ`, and a newline as `Ċ`.
