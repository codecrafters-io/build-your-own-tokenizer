In this stage, you'll implement the `train` command to merge the most frequent pair, over and over.

### The `train` command

In earlier stages, you found the most frequent pair and merged a pair into a new id. Repeating those two steps is called byte pair encoding, or BPE. GPT-2 and [GPT-4](https://github.com/openai/tiktoken) build their tokenizers this way.

Training a tokenizer means running BPE on a text file. Each round of training adds one merge to the list.

1. Convert the text into UTF-8 bytes.
2. Find the most frequent pair. On a tie, the pair that appears first, reading left to right, wins.
3. Replace that pair with the next new id. The first merge creates 256, the second creates 257, and so on.
4. Repeat steps 2 and 3 on the new list until you have done `--merges` merges.

Here's `aaabdaaabac` with `--merges 3`:

| Round | Ids | Most frequent pair | New id |
| --- | --- | --- | --- |
| 1 | `[97, 97, 97, 98, 100, 97, 97, 97, 98, 97, 99]` | `(97, 97)`, 4 times | 256 |
| 2 | `[256, 97, 98, 100, 256, 97, 98, 97, 99]` | `(256, 97)`, tied with `(97, 98)` and first | 257 |
| 3 | `[257, 98, 100, 257, 98, 97, 99]` | `(257, 98)`, 2 times | 258 |

The command prints the merged pairs in order:

```json
[[97, 97], [256, 97], [257, 98]]
```

### Tests

The tester will write `aaabdaaabac` to `corpus.txt` and execute your program like this:

```bash
$ ./your_program.sh train corpus.txt --merges 3
```

Your program must print the merged pairs as a JSON array:

```json
[[97, 97], [256, 97], [257, 98]]
```

The tester will also ask for 6, 10, and 50 merges, including on a corpus with spaces and newlines.

### Notes

- Print only the pairs, in the order you merged them. A pair's position gives its id, so the list needs no ids.
