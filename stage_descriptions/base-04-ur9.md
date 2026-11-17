In this stage, you'll implement `train` to learn an ordered list of merge pairs
from a text file.

### Your progress

<details>
  <summary>22 → 11 tokens for the example sentence `to the top of the tree`</summary>

For the exact sentence `to the top of the tree`, with no trailing newline,
training with `--num-merges 5` reduces the internal sequence from 22 tokens to
11:

```text
bytes     ██████████████████████  22 tokens
train     ███████████             11 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← comparison
```

The five merges are learned from the sentence itself. The 11 is the number of
IDs left inside training, not the command's output. The command prints the five
learned pairs. GPT-2's six-token result is a comparison, not the result of these
five merges.

</details>

### Training a tokenizer

You can already select the most frequent pair and replace its non-overlapping
occurrences. Repeating these operations lets the tokenizer learn longer tokens
from text. Learning and recording the pairs to merge is called training. The
text used for training is called a corpus. Here, the corpus is the contents of
`corpus.txt`.

This repeated pair selection and merging is the training part of byte-pair
encoding (BPE). Each learned pair gets a fresh token ID. Later pairs can include
those new IDs, so training can build longer tokens from tokens it learned
earlier. The result is an ordered list of merge rules, rather than just a
shortened sequence for this one file.

### The `train` command

The `train` command reads the corpus and learns as many pairs as `--num-merges`
asks:

1. Start with the file's UTF-8 bytes as token IDs, preserving all whitespace.
2. Count ordered neighboring pairs in the current sequence, including overlaps.
   Select the most frequent pair. On a tie, choose the pair whose first
   occurrence is earliest in that sequence, reading left to right.
3. Give the selected pair the next fresh ID: 256 for the first learned pair, 257
   for the second, and so on. Replace its non-overlapping occurrences from left
   to right with that ID, keeping unmatched IDs unchanged and in order.
4. Record the selected pair and repeat on the updated sequence until you have
   learned `--num-merges` pairs. Keep going even when the winning pair appears
   only once.

Re-count after every merge. Count pairs of the current IDs, which can include
IDs 256 and up, rather than counting the original bytes again.

Reuse the replacement operation from earlier stages with a replacement ID that
changes each round. The standalone `merge` command must still use 256 on every
invocation. Training needs varying IDs, but the existing command's contract
doesn't change.

For example, here's `aaabdaaabac`, with no trailing newline, and
`--num-merges 3`. Each row shows the IDs at the start of that round:

| Round | IDs at start                                    | Selected pair                                          | New ID |
| ----- | ----------------------------------------------- | ------------------------------------------------------ | ------ |
| 1     | `[97, 97, 97, 98, 100, 97, 97, 97, 98, 97, 99]` | `(97, 97)`, 4 times                                    | 256    |
| 2     | `[256, 97, 98, 100, 256, 97, 98, 97, 99]`       | `(256, 97)`, tied with `(97, 98)` at 2 each, and first | 257    |
| 3     | `[257, 98, 100, 257, 98, 97, 99]`               | `(257, 98)`, 2 times                                   | 258    |

In round 1, `(97, 97)` is counted four times, but only two matches can be
replaced without overlap. Both replacements use 256. In round 2, 256 represents
`aa`, so merging it with `a` (97) creates 257 for `aaa`.

After round 3, the internal sequence is `[258, 100, 258, 97, 99]`. If asked for
a fourth merge, continue even though every pair now appears only once. The first
pair, `(258, 100)`, wins and gets ID 259.

For `--num-merges 3`, print the three learned pairs in order, not the final ID
sequence:

```json
[
  [97, 97],
  [256, 97],
  [257, 98]
]
```

The list's order records when each pair was learned. A pair at zero-based index
`i` creates ID `256 + i`. In this output, `[97, 97]` creates 256, `[256, 97]`
creates 257, and `[257, 98]` creates 258. No separate IDs are needed in the
output.

### Tests

The tester will write `aaabdaaabac` to `corpus.txt`, with no trailing newline,
and execute your program like this:

```bash
$ ./your_program.sh train corpus.txt --num-merges 3
```

Your program must print the learned pairs as a JSON array:

```json
[
  [97, 97],
  [256, 97],
  [257, 98]
]
```

The tester will also ask for 6, 10, and 50 merges, including on a corpus with
spaces and newlines, for 4 merges on text containing accented characters and
emoji, and for 5 merges on the sentence from "Your progress".

### Notes

- Print only the JSON list of pairs to stdout, and send debug output to stderr.
  The tester parses JSON, so spacing and line breaks are up to you.
  `[[97,97],[256,97],[257,98]]` on one line passes too.
- The tester never asks for more merges than the text allows.
