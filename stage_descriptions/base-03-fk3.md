In this stage, you'll implement `merge` to replace non-overlapping occurrences
of a given token pair with ID `256`.

### Your progress

<details>
  <summary>22 → 18 tokens for the example sentence `to the top of the tree`</summary>

For the exact sentence `to the top of the tree`, with no trailing newline,
merging the pair `(32, 116)` reduces the sequence from 22 tokens to 18:

```text
bytes     ██████████████████████  22 tokens
merge     ██████████████████      18 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

</details>

### Why merge?

A merge combines two neighboring tokens into one token. The new token represents
the bytes of the first token followed by the bytes of the second. Its ID is a
label for that combined byte sequence, not the sum of the two original IDs.

For example, merging a space (ID `32`) followed by `t` (ID `116`) creates a
token for the two bytes `[32, 116]`. In this stage, that token's ID is `256`.
The text hasn't changed, but each match now takes one token instead of two.

GPT-2's
[published merge list](https://openaipublic.blob.core.windows.net/gpt-2/models/124M/vocab.bpe)
starts by combining a space and `t`. We're using that same pair in the progress
example. In `to the top of the tree`, four of the five words starting with `t`
follow a space. Merging those four pairs removes four tokens, taking the count
from 22 to 18. The initial `t` has no space before it, so it stays separate.

### The `merge` command

This tokenizer starts with byte IDs 0 to 255, so its first new token ID is 256.
The standalone `merge` command always uses `256` for every match, on every
invocation. Use the pair supplied by `--pair` rather than choosing one with
`top-pair`.

Like `top-pair`, the `merge` command parses `ids.json` as a JSON array of
integers. It reads the input IDs from left to right. Each time it finds the pair
given by `--pair`, it replaces those two IDs with `256` and continues after both
of them. Keep every unmatched ID unchanged and in its original order. If the
pair never appears, the output array is unchanged.

For example, in `[1, 2, 3, 1, 2]`, the pair `(1, 2)` appears twice. Replacing it
with `256` gives:

```json
[256, 3, 256]
```

Matches can't overlap. In `[97, 97, 97]`, reading from the left, the first two
IDs match `(97, 97)` and become `256`. The third `97` has nothing left to pair
with, so it stays:

```json
[256, 97]
```

This is different from counting. `top-pair` counts `(97, 97)` twice in this
list, but a merge can't use the middle `97` twice.

### Tests

The tester will write `[1, 2, 3, 1, 2]` to `ids.json` and execute your program
like this:

```bash
$ ./your_program.sh merge ids.json --pair 1,2
```

Your program must print the merged IDs as a JSON array:

```json
[256, 3, 256]
```

The tester will also run your program on a few other lists, including
overlapping matches, a pair that never appears, the IDs of `aaabdaaabac`, and
the bytes of the sentence from "Your progress".

### Notes

- `--pair` is two integers separated by a comma, with no spaces.
- Order matters. `--pair 1,2` matches `1` followed by `2`, never `2` followed by
  `1`.
- Print only the merged array to stdout, and send debug output to stderr.
