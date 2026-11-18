Your program can find a frequent pair and merge it. Learning reusable pairs will
let your tokenizer use the same pieces when it encounters other text. In this
stage, you'll combine those operations in a `train` command that learns merge
rules from a text file.

### Learn a vocabulary

The example text used for training is called a **corpus**. Training here means
choosing token pieces and assigning their IDs. You won't train a language model.

A tokenizer's **vocabulary** associates each ID with the bytes it represents.
Yours starts with the 256 byte tokens. Each learned merge adds another token:
the first gets ID 256, the second gets 257, and so on. New tokens can themselves
be part of later pairs, allowing longer pieces to form.

Repeatedly selecting a frequent pair and combining it is called **byte-pair
encoding**, or **BPE**. Although the starting tokens are bytes, later rounds can
combine tokens that each represent several bytes.

### The `train` command

Read the corpus file and perform the number of merges requested by
`--num-merges`:

1. Start with the corpus's UTF-8 byte values as token IDs.
2. Count adjacent pairs in the current sequence, including overlaps.
3. Select the most frequent pair, breaking ties by its first occurrence.
4. Replace all non-overlapping occurrences of that pair from left to right with
   the next unused ID. Record the pair.
5. Repeat from step 2 until you have performed the requested number of merges.

Recount after every round because a merge changes which tokens are neighbors.
Continue even when the most frequent pair occurs only once. The tests request no
more merges than the corpus allows.

For `aaabdaaabac` and three merges, the rounds are:

| Round | IDs before the merge                            | Selected pair                               | New ID |
| ----- | ----------------------------------------------- | ------------------------------------------- | ------ |
| 1     | `[97, 97, 97, 98, 100, 97, 97, 97, 98, 97, 99]` | `(97, 97)`, four occurrences                | 256    |
| 2     | `[256, 97, 98, 100, 256, 97, 98, 97, 99]`       | `(256, 97)`, tied with `(97, 98)` and first | 257    |
| 3     | `[257, 98, 100, 257, 98, 97, 99]`               | `(257, 98)`, two occurrences                | 258    |

Print the recorded pairs in the order they were selected. The pair at zero-based
index `i` creates token ID `256 + i`, so the output needs no separate ID field.
The final sequence for the corpus stays internal.

### Tests

The tester writes `aaabdaaabac` to `corpus.txt` and runs:

```bash
$ ./your_program.sh train corpus.txt --num-merges 3
```

Your program must print:

```json
[
  [97, 97],
  [256, 97],
  [257, 98]
]
```

The tester also requests 6, 10, and 50 merges, and uses corpora containing
spaces, newlines, accented letters, and emoji. It also requests five merges for
`to the top of the tree`. Those five rules reduce that corpus from 22 byte
tokens to 11 tokens.
