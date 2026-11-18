Your `bytes` command gives each byte its own token. Choosing a frequent pair is the first step toward combining those tokens into larger pieces. In this stage, you'll implement `top-pair` to find the most frequent pair of neighboring token IDs.

### Counting neighboring pairs

Your tokenizer will learn larger tokens by repeatedly choosing and combining frequent pairs in sample text. This stage chooses one pair. Later stages perform the replacement.

The input to `top-pair` is already a JSON array of integer IDs. Parse that array directly. There is no text-to-UTF-8 conversion in this command, and IDs can be greater than 255.

Count the pair at every position in the list. For `[1, 2, 3, 1, 2]`, the pairs are `(1, 2)`, `(2, 3)`, `(3, 1)`, and `(1, 2)` again. The winning pair is `(1, 2)` because it appears twice.

Pairs are ordered: `(1, 2)` and `(2, 1)` have separate counts. Overlapping pairs count too. In `[97, 97, 97]`, the pair `(97, 97)` appears twice, once starting at each of the first two positions.

### Breaking ties

Count the entire list before choosing a winner. If multiple pairs share the highest count, choose the one whose first occurrence is furthest left in the input.

For example, every pair in `[8, 9, 1, 2]` appears once, so `(8, 9)` wins. In `[1, 2, 3, 4, 3, 4, 1, 2]`, both `(1, 2)` and `(3, 4)` appear twice. Although `(3, 4)` reaches a count of two earlier during the scan, `(1, 2)` appears first in the input and wins.

### Tests

The tester will write `[1, 2, 3, 1, 2]` to `ids.json` and run:

```bash
$ ./your_program.sh top-pair ids.json
```

Your program must print the winning pair as a JSON array:

```json
[1, 2]
```

Print only the two IDs in that pair. Later stages create a new ID for the pair.

The tester also checks ties, overlapping pairs, reversed pairs, and IDs greater than 255. The supplied lists contain at least two IDs.
