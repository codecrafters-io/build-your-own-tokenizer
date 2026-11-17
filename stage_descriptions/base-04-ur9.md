You can now find and merge one pair. In this stage, combine those operations in
`train` to learn an ordered list of merges from a text file.

### Learning a vocabulary

Repeatedly selecting the most frequent pair and merging it learns new tokens
from the training text. This is the training step of byte-pair encoding, or BPE.
GPT-2's merge file contains 50,000 learned rules. Its first six are shown below,
with each input pair written as byte values rather than GPT-2 token IDs:

| GPT-2 token ID | Pair (byte values) | Text |
| -------------- | ------------------ | ---- |
| 256            | `(32, 116)`        | `␠t` |
| 257            | `(32, 97)`         | `␠a` |
| 258            | `(104, 101)`       | `he` |
| 259            | `(105, 110)`       | `in` |
| 260            | `(114, 101)`       | `re` |
| 261            | `(111, 110)`       | `on` |

`␠` marks a space, byte 32. Later merges build on earlier ones: the next rule
combines `␠t` and `he` into `␠the`, token ID 262. Your training text determines
your own learned pairs; you aren't trying to reproduce GPT-2's list here.

### The `train` command

The `train` command reads a text file and learns `--num-merges` pairs. For this
command, treat the whole file as one sequence, including spaces and newlines:

1. Convert the text into UTF-8 bytes.
2. Count neighboring pairs, including overlaps, and find the most frequent one.
   On a tie, the pair that appears first, reading left to right, wins.
3. Replace non-overlapping occurrences of that pair from left to right with the
   next new ID. The first merge creates 256, the second creates 257, and so on.
4. Recount the pairs in the new list and repeat until you have done
   `--num-merges` merges. Keep going even when the most frequent pair appears
   only once.

For example, here's `aaabdaaabac` with `--num-merges 3`:

| Round | IDs                                             | Most frequent pair                          | New ID |
| ----- | ----------------------------------------------- | ------------------------------------------- | ------ |
| 1     | `[97, 97, 97, 98, 100, 97, 97, 97, 98, 97, 99]` | `(97, 97)`, 4 times                         | 256    |
| 2     | `[256, 97, 98, 100, 256, 97, 98, 97, 99]`       | `(256, 97)`, tied with `(97, 98)` and first | 257    |
| 3     | `[257, 98, 100, 257, 98, 97, 99]`               | `(257, 98)`, 2 times                        | 258    |

Print only the pairs in the order they were learned, not the final token
sequence. The pair at index `i`, counting from 0, creates ID `256 + i`, so the
output needs no separate ID field:

```json
[
  [97, 97],
  [256, 97],
  [257, 98]
]
```

The tester parses your output as JSON, so spacing and line breaks are up to you.
`[[97,97],[256,97],[257,98]]` on one line passes too.

### Your progress

<details>
  <summary>22 → 11 tokens for the example sentence `to the top of the tree`</summary>

With `--num-merges 5`, training on this sentence learns five pairs. Applying
them reduces its 22 byte tokens to 11 tokens:

```text
bytes     ██████████████████████  22 tokens
train     ███████████             11 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

The `train` command prints the five learned pairs, not these 11 token IDs.

</details>

### Tests

The tester will write `aaabdaaabac` to `corpus.txt` and execute your program
like this:

```bash
$ ./your_program.sh train corpus.txt --num-merges 3
```

Your program must print the learned pairs as a JSON array to stdout:

```json
[
  [97, 97],
  [256, 97],
  [257, 98]
]
```

The tester will also ask for 6, 10, and 50 merges, including on a corpus with
spaces and newlines, and for 5 merges on the sentence from "Your progress". The
tester never asks for more merges than the text allows.
