In this stage, you'll find the adjacent pair of token ids that shows up most often.

### The winning pair

In earlier stages, you turned text into byte ids. This is highly inefficient as a simple `hi 👋` string required 8 bytes `[104, 105, 32, 240, 159, 145, 139]`. In this stage, we will identify the most frequented pair, so that we can assign them a new token ID and reduce the number of tokens required. 

As part of this stage, count every pair of neighboring token ids to identify the most frequently occuring pair. Overlapping positions each count, so in `aaa` the pair `(97, 97)` occurs twice. When two pairs have the same count, the one that appears earliest in a left-to-right scan wins.

The ids of `aaabdaaabac` are `[97, 97, 97, 98, 100, 97, 97, 97, 98, 97, 99]`. The most frequent pair is:

```json
[97, 97]
```

That pair appears 4 times. The tie input `[8, 9, 1, 2]` has two pairs that each occur once, and the earliest one is:

```json
[8, 9]
```

### Tests

The tester runs:

```bash
$ ./your_program.sh top-pair ids.json
```

For the bytes of `aaabdaaabac`, `stdout` is:

```json
[97, 97]
```

The tester checks that:

- `aaabdaaabac` reports `[97, 97]`
- `[8, 9, 1, 2]` reports `[8, 9]`, the earliest pair on a tie
- overlapping positions in `aaa` each count

### Notes

- This stage only reports the winning pair. Later stages repeat the same tie-break after every merge.
