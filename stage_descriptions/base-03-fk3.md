Your `top-pair` command finds a pair worth combining. A token holding two
neighbors' combined bytes can shorten the sequence without losing the text they
represent. In this stage, you'll add `merge` to replace a chosen pair with a
single token ID.

### Replace a pair

A **merge** joins two neighboring tokens into one. The byte tokens already use
IDs 0 through 255, so this stage assigns the new token ID 256. For example,
merging `(97, 98)` gives the byte sequence for `ab` a single ID, 256. The ID
labels those bytes. It is not the sum of their values.

Read the input IDs from `ids.json`. The `--pair` argument gives the two IDs to
merge as comma-separated integers, such as `--pair 1,2`.

Scan the input from left to right. Replace each occurrence of the specified pair
with 256, consuming both IDs. Keep all unmatched IDs in their original order. A
pair that never appears leaves the array unchanged.

Matches cannot overlap. In `[97, 97, 97]`, merging `(97, 97)` consumes the first
two IDs and leaves the third:

```json
[256, 97]
```

Your `top-pair` command counts two overlapping occurrences in this input.
Replacement can use each input ID only once. Pair order still matters:
`--pair 1,2` only matches 1 followed by 2.

### What a merge saves

The text `to the top of the tree` starts as 22 byte tokens. The pair
`(32, 116)`, a space followed by `t`, occurs four times. Replacing those
occurrences with 256 leaves 18 tokens. Each 256 still represents both the space
and the `t`.

### Tests

The tester writes `[1, 2, 3, 1, 2]` to `ids.json` and runs:

```bash
$ ./your_program.sh merge ids.json --pair 1,2
```

Your program must print:

```json
[256, 3, 256]
```

The tester also checks overlapping matches, a missing pair, reversed pairs,
multi-digit IDs, and the text example above. This command always uses 256 for
the replacement token.
