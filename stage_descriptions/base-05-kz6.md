In this stage, you'll encode new text by applying the earliest merge that still
appears.

### Encoding

In earlier stages, you trained a tokenizer. It counted pairs and merged the most
common one, over and over, and the result was a list of merges. Encoding uses
that list to turn new text into token ids.

Encoding doesn't count pairs again. It reuses the merges from training, so the
same text always gets the same ids.

Read `merges.json`, a JSON array of pairs in the order they were learned. A
pair's position in the list is its rank, and the pair at index `i`, counting
from 0, creates id `256 + i`. To encode text:

1. Start with the text's UTF-8 bytes.
2. Find the pair with the earliest rank that is still present, and merge every
   occurrence of it, like the `merge` command does.
3. Repeat step 2 until no pair of neighboring ids is in `merges.json`.

For example, training on `aaabdaaabac` gave the merges
`[[97, 97], [256, 97], [257, 98]]`. Here's how they encode the text `aaab`:

| Round | Ids                | Pair with the earliest rank | New id |
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
first. That turns the ids into `[97, 256, 256]`. Both `98`s are now used up, so
`(97, 98)` is no longer present, and the output is:

```json
[97, 256, 256]
```

### Tests

The tester will write `aaab` to `text.txt` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then execute your program
like this:

```bash
$ ./your_program.sh encode text.txt --merges merges.json
```

Your program must print the token ids as a JSON array:

```json
[258]
```

The tester will also run your program on a few other files, including the
training text, the `ababa` example above, and an empty file.

### Notes

- Always apply the earliest learned merge that is still present.
- Applying a later merge first can produce ids a model never saw in training.
- Here `--merges` is the path to a merges file. In `train`, it was a count.
