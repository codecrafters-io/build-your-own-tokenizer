In this stage, you'll encode new text by applying the earliest merge that still appears.

### Encoding

In earlier stages, you learned merges by recounting which pair was most common. Encoding does not recount. Read `merges.json`, a JSON array of pairs in the order they were learned. Pair number `i` created token `256 + i`. While two or more ids remain, merge the pair with the earliest rank that is still present.

The three merges from `aaabdaaabac` encode the text `aaab` as:

```json
[258]
```

The same merges encode the training text as `[258, 100, 258, 97, 99]`.

Rank is not frequency. The text `ababa` with merges `[[98, 97], [97, 98]]` still contains both pairs, and each pair occurs twice. The earliest rank is `(98, 97)`, so the output is:

```json
[97, 256, 256]
```

### Tests

The tester will write `aaab` to `text.txt` and `[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then execute your program like this:

```bash
$ ./your_program.sh encode text.txt --merges merges.json
```

Your program must print the token ids as a JSON array:

```json
[258]
```

The tester will also run your program on a few other files, including the training text, the `ababa` example above, and an empty file.

### Notes

- Always apply the earliest learned merge that is still present.
- Applying a later merge first can produce ids a model never saw in training.
