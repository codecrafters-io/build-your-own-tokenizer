In this stage, you'll encode new text by applying the earliest merge that still
appears.

### Encoding

In earlier stages, you trained a tokenizer. It counted pairs and merged the most
common one, over and over, and the result was a list of merges. Encoding uses
that list to turn new text into token ids. It's what a tokenizer does every time
it's used, like when you send a message to a language model.

Encoding doesn't count pairs again. It reuses the merges from training, so the
same text always gets the same ids.

Read `merges.json`, a JSON array of pairs in the order they were learned. A
pair's position in the list is its rank, and the pair at index `i`, counting
from 0, created id `256 + i`. Then:

1. Start with the text's UTF-8 bytes.
2. Find the pair with the earliest rank that is still present, and merge every
   occurrence of it, like the `merge` command does.
3. Repeat step 2 until no pair of neighboring ids is in `merges.json`.

For example, the three merges from `aaabdaaabac` encode the text `aaab` as:

```json
[258]
```

The same merges encode the training text as `[258, 100, 258, 97, 99]`.

Rank is not frequency. The text `ababa` with merges `[[98, 97], [97, 98]]` still
contains both pairs, and each pair occurs twice. The earliest rank is
`(98, 97)`, so the output is:

```json
[97, 256, 256]
```

### Tests

The tester will write `aaab` to `text.txt` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then execute your program
like this:

```bash
$ ./your_program.sh encode text.txt --merges merges.json
```

Your program must print the token ids as a JSON array:

```json
[258]
```

The tester will also run your program on a few other files, including the
training text, the `ababa` example above, and an empty file.

### Notes

- Always apply the earliest learned merge that is still present.
- Applying a later merge first can produce ids a model never saw in training.
- Here `--merges` is the path to a merges file. In `train`, it was a count.
