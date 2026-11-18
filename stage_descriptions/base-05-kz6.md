Your `train` command produces reusable merge rules. In this stage, you'll
implement `encode` to apply those rules to new text and print its token IDs.

### Your progress

<details>
  <summary>11 tokens for the example sentence `to the top of the tree`</summary>

Here's how many tokens the sentence `to the top of the tree` takes so far:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

The 11 comes from encoding the sentence with the five merges learned from it.

</details>

### Why encode?

GPT-2's tokenizer uses a fixed list of 50,000 merges. Keeping the rules fixed
gives each token the same meaning across different texts. With GPT-2's rules
and vocabulary, `Hello world` becomes `[15496, 995]` each time.

You'll use your own learned list here. Encoding applies those rules without
learning any new ones.

### The `encode` command

The `encode` command reads text from `text.txt` and merges from `merges.json`, a
JSON array of pairs in the same format `train` prints. A pair's position in the
list is its rank, and the pair at index `i`, counting from 0, creates ID
`256 + i`.

1. Convert the text into UTF-8 bytes.
2. Find the pair with the earliest rank that is still present in the IDs.
3. Merge every occurrence of that pair, the same way the `merge` command does.
4. Repeat until no pair of neighboring IDs is in the merge list.

For example, training on `aaabdaaabac` gave the merges
`[[97, 97], [256, 97], [257, 98]]`. Here's how they encode the text `aaab`:

| Round | IDs                | Pair with the earliest rank | New ID |
| ----- | ------------------ | --------------------------- | ------ |
| 1     | `[97, 97, 97, 98]` | `(97, 97)`, rank 0          | 256    |
| 2     | `[256, 97, 98]`    | `(256, 97)`, rank 1         | 257    |
| 3     | `[257, 98]`        | `(257, 98)`, rank 2         | 258    |

No pairs are left, so the output is `[258]`.

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

### Tests

The tester will write `aaab` to `text.txt` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then execute your program
like this:

```bash
$ ./your_program.sh encode text.txt --merges merges.json
```

Your program must print the token IDs as a JSON array:

```json
[258]
```

The tester will also run your program on a few other files, including the
training text, the `ababa` example above, an empty file, and the sentence from
"Your progress".

### Notes

- Applying a later merge first can produce a different token sequence for the
  same text. Keep the rank order from `merges.json`.
- Bytes with no applicable merge keep their original IDs. An empty file
  produces `[]`.
