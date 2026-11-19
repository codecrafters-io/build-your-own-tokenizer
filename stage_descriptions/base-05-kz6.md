In this stage, you'll implement the `encode` command to turn new text into token
IDs with a list of learned merges.

### Your progress

<details>
  <summary>11 tokens for the example sentence `to the top of the tree`</summary>

Here's the running comparison for `to the top of the tree`:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

The 11 comes from encoding the sentence with the five merges learned from it.

</details>

### Why encode?

GPT-2's tokenizer was trained once. Since then, it has encoded every prompt with
the same 50,000 merges, replayed in the order they were learned. That is how
`Hello world` becomes `[15496, 995]`, the same two IDs every time.

Your `train` command produces exactly that kind of list. In this stage, you'll
use it.

### The `encode` command

The `encode` command reads text from `text.txt` and merges from `merges.json`, a
JSON array of pairs in the same format `train` prints. A pair's position in the
list is its rank, and the pair at index `i`, counting from 0, creates ID
`256 + i`.

1. Convert the text into UTF-8 bytes.
2. Find the pair with the earliest rank that is still present in the IDs.
3. Merge every occurrence of that pair, the same way the `merge` command does.
4. Repeat until no pair of neighboring IDs is in the list.

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

Training chooses pairs by frequency. Encoding chooses among learned pairs by
rank, regardless of their frequency in the new text.

To see the difference, take a new text, `ababab`, and say `merges.json` is
`[[98, 97], [97, 98]]`. The text's bytes are `[97, 98, 97, 98, 97, 98]`.
`(97, 98)` appears three times, while `(98, 97)` appears twice.

`(98, 97)` is first in `merges.json`, so it has the earliest rank and is merged
first despite being less frequent. That turns the IDs into `[97, 256, 256, 98]`.
The remaining `97` and `98` are no longer neighbors, so neither learned pair is
present. The output is:

```json
[97, 256, 256, 98]
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
training text, competing merges for `ababa`, an empty file, and the sentence
from "Your progress".

### Notes

- An empty input file produces `[]`.
- Applying a later merge first can change the token sequence. A model expects
  the encoding rules used during its training.
