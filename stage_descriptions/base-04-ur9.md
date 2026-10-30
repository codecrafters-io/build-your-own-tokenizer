In this stage, you'll implement the `train` command to learn which pairs to
merge from a text file.

<details>
  <summary>Your progress: 22 → 11 tokens for the example sentence `to the top of the tree`</summary>

Here's how many tokens the sentence `to the top of the tree` takes so far:

```text
bytes     ██████████████████████  22 tokens
train     ███████████             11 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

</details>

### How GPT-2 learned its merges

GPT-2's tokenizer comes from repeating the steps of earlier stages: find the
most frequent pair, merge it, and repeat. GPT-2 did this 50,000 times on its
training text. Its first six merges were:

| New id | Pair         | Token |
| ------ | ------------ | ----- |
| 256    | `(32, 116)`  | ` t`  |
| 257    | `(32, 97)`   | ` a`  |
| 258    | `(104, 101)` | `he`  |
| 259    | `(105, 110)` | `in`  |
| 260    | `(114, 101)` | `re`  |
| 261    | `(111, 110)` | `on`  |

Later merges build on earlier ones. Merge 262 combines ` t` and `he` into
` the`. This loop is called byte-pair encoding, or BPE.

### The `train` command

The `train` command reads text from a file and runs the loop as many times as
`--merges` asks:

1. Convert the text into UTF-8 bytes.
2. Find the most frequent pair. On a tie, the pair that appears first, reading
   left to right, wins.
3. Replace that pair with the next new id. The first merge creates 256, the
   second creates 257, and so on.
4. Repeat steps 2 and 3 on the new list until you have done `--merges` merges.
   Keep going even when the most frequent pair appears only once.

For example, here's `aaabdaaabac` with `--merges 3`:

| Round | Ids                                             | Most frequent pair                          | New id |
| ----- | ----------------------------------------------- | ------------------------------------------- | ------ |
| 1     | `[97, 97, 97, 98, 100, 97, 97, 97, 98, 97, 99]` | `(97, 97)`, 4 times                         | 256    |
| 2     | `[256, 97, 98, 100, 256, 97, 98, 97, 99]`       | `(256, 97)`, tied with `(97, 98)` and first | 257    |
| 3     | `[257, 98, 100, 257, 98, 97, 99]`               | `(257, 98)`, 2 times                        | 258    |

The command prints the merged pairs in order:

```json
[
  [97, 97],
  [256, 97],
  [257, 98]
]
```

### Tests

The tester will write `aaabdaaabac` to `corpus.txt` and execute your program
like this:

```bash
$ ./your_program.sh train corpus.txt --merges 3
```

Your program must print the merged pairs as a JSON array:

```json
[
  [97, 97],
  [256, 97],
  [257, 98]
]
```

The tester will also ask for 6, 10, and 50 merges, including on a corpus with
spaces and newlines.

### Notes

- Print only the pairs, in the order you merged them. A pair's position gives
  its id, so the list needs no ids.
- The tester never asks for more merges than the text allows.
