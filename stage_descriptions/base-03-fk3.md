Merging a pair reduces the number of IDs needed to represent the same bytes. Your `top-pair` command chooses which pair to combine. In this stage, you'll implement `merge` to replace a selected pair with one new token ID.

### Giving a pair its own ID

A vocabulary associates each token ID with the bytes it represents. So far, IDs 0 through 255 each represent one byte. The first available ID for a larger token is 256.

This command makes one kind of replacement, so it always uses 256 as the new ID. For example, merging `(97, 97)` creates a token representing the two bytes of `aa`. Later stages learn several merges and give each one its own ID.

Read the JSON array from `ids.json`. The `--pair` argument supplies two integers separated by a comma, with no spaces. Replace every occurrence of that ordered pair while scanning the original list from left to right. Copy unmatched IDs unchanged.

### Overlapping matches

Each input ID can be used only once during a merge. Given `[97, 97, 97]` and the pair `(97, 97)`, combine the first two IDs. The third has no partner left, so the result is:

```json
[256, 97]
```

Your `top-pair` command counts two overlapping pairs in this input. Replacing both would reuse the middle ID and change the represented text.

Continue scanning after the IDs you consumed. A newly emitted 256 is part of the output and does not take part in another replacement during this pass.

<details>
  <summary>Your progress: one merge shortens a sentence</summary>

The sentence `to the top of the tree` contains 22 bytes, with no trailing newline. Merging `(32, 116)`, a space followed by `t`, replaces four pairs with four new tokens. The result has 18 IDs and still represents all 22 bytes.

</details>

### Tests

The tester will write `[1, 2, 3, 1, 2]` to `ids.json` and run:

```bash
$ ./your_program.sh merge ids.json --pair 1,2
```

Your program must print the resulting IDs as a JSON array:

```json
[256, 3, 256]
```

The tester also checks overlapping matches, ordered pairs, and a pair that doesn't occur. When the pair is absent, print the original list unchanged.
