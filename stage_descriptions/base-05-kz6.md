In this stage, you'll implement the `encode` command to turn new text into token
ids with a list of learned merges.

<details>
  <summary>Your progress: 11 tokens for the example sentence `to the top of the tree`</summary>

Here's how many tokens the sentence `to the top of the tree` takes so far:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

</details>

### Why encode?

GPT-2's tokenizer was trained once. Since then, it has encoded every prompt with
the same 50,000 merges, replayed in the order they were learned. That is how
`Hello world` becomes `[15496, 995]`, the same two ids every time.

Your `train` command produces exactly that kind of list. In this stage, you'll
use it.

### The `encode` command

The `encode` command reads text from `text.txt` and merges from `merges.json`, a
JSON array of pairs in the same format `train` prints. A pair's position in the
list is its rank, and the pair at index `i`, counting from 0, creates id
`256 + i`.

1. Convert the text into UTF-8 bytes.
2. Find the pair with the earliest rank that is still present in the ids.
3. Merge every occurrence of that pair, the same way the `merge` command does.
4. Repeat until no pair of neighboring ids is in the list.

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
