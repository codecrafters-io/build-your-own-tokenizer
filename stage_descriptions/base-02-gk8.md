Your program can represent text as one token per byte. Frequent pairs are
candidates for larger, reusable tokens that can shorten the sequence a model
receives. In this stage, you'll add `top-pair` to find the most frequent pair of
neighboring token IDs.

### Count neighboring pairs

The command reads a JSON array of IDs from `ids.json`. These are already
integers, so this command does not convert text to UTF-8.

Count every pair of adjacent IDs. The pair is ordered: `(1, 2)` means 1 followed
by 2, and `(2, 1)` is a different pair. Overlapping pairs count too. The bytes
of `aaa` are `[97, 97, 97]`, which contains `(97, 97)` at two positions.

Choose the pair with the highest total count. If several pairs tie, choose the
one whose first occurrence is furthest left in the input:

- In `[8, 9, 1, 2]`, every pair occurs once, so `(8, 9)` wins.
- In `[1, 2, 3, 4, 3, 4, 1, 2]`, both `(1, 2)` and `(3, 4)` occur twice.
  `(1, 2)` wins because its first occurrence is earlier. The order in which
  pairs reach their final counts does not change the winner.

Print the winning pair as a two-element JSON array. Leave the input IDs
unchanged. Inputs in this stage contain at least two IDs, and IDs can be larger
than 255.

### Tests

The tester writes `[1, 2, 3, 1, 2]` to `ids.json` and runs:

```bash
$ ./your_program.sh top-pair ids.json
```

The pair `(1, 2)` appears twice, while the other pairs appear once. Your program
must print:

```json
[1, 2]
```

The tester also checks ties, overlapping pairs, pairs starting at different
positions, reversed pairs, and IDs above 255. Tie cases run repeatedly, so the
same input must always produce the same winner.
