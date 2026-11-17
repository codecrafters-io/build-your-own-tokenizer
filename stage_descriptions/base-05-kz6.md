Your `train` command produces a list of merges. In this stage, add `encode` to
apply a supplied merge list to new text and output token IDs, without learning
new merges.

### Why encode?

GPT-2's vocabulary comes with a fixed list of 50,000 merge rules. Its encoder
uses that list rather than counting the most frequent pairs in each new input.
With GPT-2's vocabulary, `Hello world` becomes `[15496, 995]`.

You'll apply the same rank-based selection principle to your own learned merges.
Keep your current numbering: single-byte tokens use IDs 0–255, and learned
tokens start at 256. GPT-2's own numbering comes later.

### The `encode` command

The `encode` command reads text from `text.txt` and merges from `merges.json`, a
JSON array of pairs in the same format `train` prints. A pair's position in the
list is its rank, and the pair at index `i`, counting from 0, creates ID
`256 + i`.

1. Convert the text into UTF-8 bytes.
2. Among neighboring pairs currently present in the IDs, choose the one with the
   lowest rank in the supplied list.
3. Merge its non-overlapping occurrences from left to right, as in `merge`.
4. Repeat until no neighboring pair appears in the merge list. Leave the
   remaining IDs unchanged.

For example, training on `aaabdaaabac` gave the merges
`[[97, 97], [256, 97], [257, 98]]`. Here's how they encode the text `aaab`:

| Round | IDs                | Pair with the earliest rank | New ID |
| ----- | ------------------ | --------------------------- | ------ |
| 1     | `[97, 97, 97, 98]` | `(97, 97)`, rank 0          | 256    |
| 2     | `[256, 97, 98]`    | `(256, 97)`, rank 1         | 257    |
| 3     | `[257, 98]`        | `(257, 98)`, rank 2         | 258    |

No pairs are left, so the output is:

```json
[258]
```

The same merges encode the training text as `[258, 100, 258, 97, 99]`.

Rank is not frequency. Training picked the most frequent pair, but encoding
ignores how often a pair appears. It only looks at rank.

To see the difference, take a new text, `ababa`, and say `merges.json` is
`[[98, 97], [97, 98]]`. The text's bytes are `[97, 98, 97, 98, 97]`, so
`(97, 98)` and `(98, 97)` each appear twice.

`(98, 97)` is first in `merges.json`, so it has the earliest rank and is merged
first. That turns the IDs into `[97, 256, 256]`. Both `98`s are now used up, so
`(97, 98)` is no longer present, and the output is:

```json
[97, 256, 256]
```

Choosing `(97, 98)` first would use up different neighbors and change the
output. Follow the supplied ranks, not frequency or the pair's position in the
input.

### Your progress

<details>
  <summary>11 tokens for the example sentence `to the top of the tree`</summary>

Encoding the sentence with the five merges learned from it produces 11 token
IDs:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

</details>

### Tests

The tester will write `aaab` to `text.txt` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then execute your program
like this:

```bash
$ ./your_program.sh encode text.txt --merges merges.json
```

Your program must print the token IDs as a JSON array to stdout:

```json
[258]
```

The tester will also run your program on a few other files, including the
training text, the `ababa` example above, an empty file, and the sentence from
"Your progress". An empty file must produce `[]`.
