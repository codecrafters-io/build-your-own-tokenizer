In this stage, you'll implement `top-pair` to find the most frequent pair of
neighboring token IDs.

### Why combine pairs?

Combining two neighboring tokens into one longer token shortens the token
sequence. If the same pair occurs in several places, that longer token can be
reused in those places. Counting pairs helps us choose which longer token to
add.

This stage only selects the most frequent pair. Later stages will give that pair
a new ID and combine its occurrences.

### The `top-pair` command

The `top-pair` command counts every pair of neighboring IDs and prints the one
that appears most often. Pair order matters: `(1, 2)` and `(2, 1)` are different
pairs. Count a pair starting at every position except the last, including
overlapping pairs. For example, `[5, 5, 5]` contains `(5, 5)` twice, because the
middle ID belongs to both pairs.

If several pairs have the same highest count, choose the pair whose first
occurrence is earliest in the input, reading left to right.

Your program reads IDs from `ids.json` and parses it as JSON. The IDs are
already integers, so there is no UTF-8 step for this stage.

For example, in `[1, 2, 3, 1, 2]`, the pairs are `(1, 2)`, `(2, 3)`, `(3, 1)`,
and `(1, 2)` again. The pair `(1, 2)` appears twice, so the most frequent pair
is:

```json
[1, 2]
```

In `[8, 9, 1, 2]`, the pairs `(8, 9)`, `(9, 1)`, and `(1, 2)` each appear once.
They all tie, so the first pair from the left wins:

```json
[8, 9]
```

In `[1, 2, 3, 4, 3, 4, 1, 2]`, `(1, 2)` and `(3, 4)` both appear twice. `(1, 2)`
appears first, so it wins:

```json
[1, 2]
```

### Tests

The tester will write `[1, 2, 3, 1, 2]` to `ids.json` and execute your program
like this:

```bash
$ ./your_program.sh top-pair ids.json
```

Your program must print the pair as a JSON array:

```json
[1, 2]
```

The tester will also run your program on other lists, including ties,
overlapping pairs, and the IDs of `aaabdaaabac`.

### Notes

- Print only the winning pair as a JSON array to stdout. Send debug output to
  stderr.
- Don't assume IDs fit in a byte. Later stages create IDs 256 and up and count
  their pairs the same way.
