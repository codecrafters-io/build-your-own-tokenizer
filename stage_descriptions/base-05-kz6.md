In this stage, you'll implement `encode` to turn text into token IDs using
the merge rules learned by `train`. Training chooses the vocabulary once.
Encoding reuses it, so the same text and merge list always produce the same
IDs.

### Applying learned rules

Read the UTF-8 text from `text.txt` and the list of pairs from `merges.json`.
This list has the format your `train` command prints. You can save that
command's output to create a merge file for your own experiments.

A rule's position in the list is its rank. Lower ranks take priority. As in
training, the pair at index `i`, counting from zero, creates ID `256 + i`.
That ID stays fixed even when earlier rules don't apply to the new text.

Encode the text as follows:

1. Start with its UTF-8 byte values.
2. Find the lowest-ranked pair that occurs in the current ID list.
3. Replace all non-overlapping occurrences of that pair, from left to right,
   with its defined ID.
4. Repeat until no neighboring pair has a rule in the merge list.

Training chooses rules by frequency in the corpus. Encoding follows the saved
ranks, regardless of how often a pair occurs in the new input. Leave bytes
without an applicable merge as their original IDs.

### Following a merge list

Training on `aaabdaaabac` with three rounds produced
`[[97, 97], [256, 97], [257, 98]]`. Applied to the shorter text `aaab`, those
rules give:

| Rule rank | IDs before the merge | Pair | New ID |
| --- | --- | --- | --- |
| 0 | `[97, 97, 97, 98]` | `(97, 97)` | 256 |
| 1 | `[256, 97, 98]` | `(256, 97)` | 257 |
| 2 | `[257, 98]` | `(257, 98)` | 258 |

The final sequence contains only ID 258, which represents `aaab`.

To see why rank matters, use `ababa` with a different list,
`[[98, 97], [97, 98]]`. Both pairs occur twice, and `(97, 98)` appears first
in the input. But `(98, 97)` has rank 0, so it merges first. The result is
`[97, 256, 256]`. No listed pair remains.

### Tests

The tester will write `aaab` to `text.txt` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then run:

```bash
$ ./your_program.sh encode text.txt --merges merges.json
```

Your program must print the token IDs as a JSON array:

```json
[258]
```

The tester also checks the original training text, competing merge ranks,
overlapping matches, and multibyte characters. An empty text file must produce
`[]`.

This stage applies merges across the whole input. Splitting the text into
separate regions before merging comes later.

<details>
  <summary>Your progress: reusing the five learned rules</summary>

Encoding `to the top of the tree` with the five rules learned in the previous
stage produces:

```json
[260, 259, 111, 112, 32, 111, 102, 259, 114, 101, 101]
```

Those 11 IDs represent the original 22 bytes. The next stage will turn them
back into text.

</details>
