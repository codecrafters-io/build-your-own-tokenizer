Your `train` command produces a list of merge rules. Reusing those rules keeps
each token ID's meaning fixed across different inputs. In this stage, you'll add
`encode` to apply the learned rules to new text.

### Apply learned rules

The command reads text from `text.txt` and an ordered JSON array of pairs from
`merges.json`, in the format that `train` prints.

A pair's **rank** is its zero-based position in that list. Lower ranks have
higher priority: rank 0 applies before rank 1. As in training, the pair at index
`i` creates ID `256 + i`.

1. Convert the input text to UTF-8 byte IDs.
2. Among adjacent pairs present in the current sequence, find the pair with the
   lowest rank in the merge list. Ignore pairs absent from the list.
3. Replace all non-overlapping occurrences of that pair from left to right with
   its assigned ID.
4. Repeat until no adjacent pair has a rule in the merge list.

Keep any IDs that cannot be merged. An empty input produces `[]`.

For example, the rules `[[97, 97], [256, 97], [257, 98]]` learned from
`aaabdaaabac` encode `aaab` as follows:

| Round | Current IDs        | Lowest-ranked available pair | New ID |
| ----- | ------------------ | ---------------------------- | ------ |
| 1     | `[97, 97, 97, 98]` | `(97, 97)`, rank 0           | 256    |
| 2     | `[256, 97, 98]`    | `(256, 97)`, rank 1          | 257    |
| 3     | `[257, 98]`        | `(257, 98)`, rank 2          | 258    |

The result is `[258]`. The same rules encode the training corpus `aaabdaaabac`
as `[258, 100, 258, 97, 99]`.

### Training and encoding choose differently

Training selects a pair by its frequency in the changing corpus. Encoding uses
the priorities already recorded in the merge file. It does not learn new rules
from the input.

For `ababa`, suppose the merge file is `[[98, 97], [97, 98]]`. Both pairs occur
twice in the input bytes `[97, 98, 97, 98, 97]`. Encoding chooses `(98, 97)`
because it has rank 0, producing:

```json
[97, 256, 256]
```

Using the training tie-breaker would choose the leftmost pair, `(97, 98)`, and
produce a different result. During encoding, rank determines which pair to
select. The left-to-right rule determines how to replace occurrences of that
selected pair.

### Tests

The tester writes `aaab` to `text.txt` and `[[97, 97], [256, 97], [257, 98]]` to
`merges.json`, then runs:

```bash
$ ./your_program.sh encode text.txt --merges merges.json
```

Your program must print:

```json
[258]
```

The tester also checks the training corpus, rank ordering, non-ASCII text, an
empty file, and `to the top of the tree` with its five learned merges.
