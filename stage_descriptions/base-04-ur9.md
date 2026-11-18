Your existing commands can choose a pair and replace it. Repeating those operations builds a vocabulary that your encoder can apply to new text. In this stage, you'll implement `train` to learn a reusable list of merge rules from a text file.

### Learning a vocabulary

The sample text used for training is called a corpus. Start with its UTF-8 bytes, then repeatedly combine the most frequent neighboring pair into a new token. This is the training step of byte-pair encoding, or BPE.

Each round adds one token to the vocabulary. Record its pair so you can apply the same rule again when encoding other text. This trains the tokenizer's vocabulary. No language model or model weights are involved.

### The training loop

Read the corpus and begin with one ID per UTF-8 byte. Repeat these steps `--num-merges` times:

1. Count neighboring pairs in the current ID list, including overlaps.
2. Choose the most frequent pair. Break ties by its first occurrence, as in `top-pair`.
3. Replace every non-overlapping occurrence from left to right, as in `merge`. Use a fresh ID for each round: 256, then 257, then 258, and so on.
4. Append the selected pair to the list of learned merges.

Recount after every round because the replacement changes the pairs that are present. Keep going even when the most frequent pair appears only once.

For `aaabdaaabac` with `--num-merges 3`, the rounds are:

| Round | IDs before the merge | Selected pair | New ID |
| --- | --- | --- | --- |
| 1 | `[97, 97, 97, 98, 100, 97, 97, 97, 98, 97, 99]` | `(97, 97)`, four occurrences | 256 |
| 2 | `[256, 97, 98, 100, 256, 97, 98, 97, 99]` | `(256, 97)`, tied with `(97, 98)` and first in the list | 257 |
| 3 | `[257, 98, 100, 257, 98, 97, 99]` | `(257, 98)`, two occurrences | 258 |

One round can replace several occurrences. The requested merge count is the number of rounds, so this example learns three rules.

<details>
  <summary>Your progress: learning five rules from a sentence</summary>

Training on `to the top of the tree` with `--num-merges 5` learns:

```json
[[32, 116], [256, 104], [257, 101], [258, 256], [116, 111]]
```

After those five rounds, the sentence has 11 IDs instead of its original 22. Later stages use these rules to produce encoded text.

</details>

### Tests

The tester will write `aaabdaaabac` to `corpus.txt` and run:

```bash
$ ./your_program.sh train corpus.txt --num-merges 3
```

Your program must print the selected pairs in the order they were learned:

```json
[
  [97, 97],
  [256, 97],
  [257, 98]
]
```

The pair at index `i`, counting from zero, defines token ID `256 + i`. For example, the second pair defines ID 257 and uses the previously created ID 256. Print only this list of pairs. The final encoded corpus and pair counts are excluded from the output.

The tester also requests 6, 10, and 50 rounds on other inputs, including text with spaces, newlines, and multibyte characters. It never requests more rounds than the corpus allows.
