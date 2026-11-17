In this stage, you'll implement `encode` to turn text into token IDs using a
fixed list of learned merge rules.

### Your progress

<details>
  <summary>22 → 11 tokens for the example sentence `to the top of the tree`</summary>

For the exact sentence `to the top of the tree`, with no trailing newline,
encoding with these five rules learned from the sentence itself reduces the
sequence from 22 tokens to 11:

```json
[
  [32, 116],
  [256, 104],
  [257, 101],
  [258, 256],
  [116, 111]
]
```

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← comparison
```

GPT-2's six-token result is a comparison, not the result of these five rules.

</details>

### Why encode?

Training chooses pairs by frequency and learns merge rules from a corpus.
Encoding reuses those rules to turn text into token IDs. The text can differ
from the training corpus, but the rules stay fixed. Your `encode` command must
not retrain on its input or choose pairs by their frequency in that input.

### The `encode` command

The `encode` command reads text from `text.txt` and merges from `merges.json`, a
JSON array of pairs in the same format `train` prints. Each pair's zero-based
position in the list is its **rank**. Lower ranks have higher priority: rank 0
comes before rank 1. The pair at index `i` has replacement ID `256 + i`, just as
it did during training.

1. Start with the text's UTF-8 bytes as token IDs, preserving all whitespace.
2. Among neighboring pairs in the current IDs that appear in the rule list,
   choose the pair with the lowest rank.
3. Replace that pair's non-overlapping matches from left to right with its
   replacement ID. Keep every unmatched ID unchanged and in order.
4. Repeat on the updated sequence until no neighboring pair appears in the rule
   list.

If no rule matches, leave the IDs unchanged. An empty input produces `[]`.
Unlike `train`, `encode` prints the final IDs, not the rules.

For example, training on `aaabdaaabac` gave the merges
`[[97, 97], [256, 97], [257, 98]]`. Here's how they encode the text `aaab`:

| Round | IDs at start       | Pair with the lowest rank | Replacement ID |
| ----- | ------------------ | ------------------------- | -------------- |
| 1     | `[97, 97, 97, 98]` | `(97, 97)`, rank 0        | 256            |
| 2     | `[256, 97, 98]`    | `(256, 97)`, rank 1       | 257            |
| 3     | `[257, 98]`        | `(257, 98)`, rank 2       | 258            |

No pairs are left, so the output is:

```json
[258]
```

The same merges encode the training text as `[258, 100, 258, 97, 99]`.

### Why rank matters

Take the text `ababa`, and say `merges.json` is `[[98, 97], [97, 98]]`. The
text's bytes are `[97, 98, 97, 98, 97]`, so `(97, 98)` and `(98, 97)` each
appear twice.

Although `(97, 98)` occurs first in the text, `(98, 97)` has rank 0 in the rule
list and is merged first. That turns the IDs into `[97, 256, 256]`. Both `98`s
are now used up, so `(97, 98)` is no longer present, and the output is:

```json
[97, 256, 256]
```

A merge consumes both IDs in each match, so overlapping pairs compete. If you
applied the later rule `(97, 98)` first instead, you'd get `[257, 257, 97]`.
Rank determines which of these token sequences to produce.

Rank also takes priority over frequency. For `abababb` with rules
`[[98, 98], [97, 98]]`, `(98, 98)` occurs once and `(97, 98)` occurs three
times. Encoding still merges `(98, 98)` first because it has rank 0, then merges
the remaining `(97, 98)` matches with ID 257. The result is
`[257, 257, 97, 256]`.

### Tests

The tester will write `aaab` to `text.txt`, with no trailing newline, and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then execute your program
like this:

```bash
$ ./your_program.sh encode text.txt --merges merges.json
```

Your program must print the token IDs as a JSON array:

```json
[258]
```

The tester will also run your program on the training text, the `ababa` example
above, an empty file, text with accented characters, competing pairs with
different ranks, and the sentence from "Your progress".

### Notes

- Print only the final JSON array of IDs to stdout, and send debug output to
  stderr. The tester parses JSON, so spacing and line breaks are up to you.
