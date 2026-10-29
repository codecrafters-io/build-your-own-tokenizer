In this stage, you'll implement the `top-pair` command to find the most frequent pair of token ids.

### The `top-pair` command

In earlier stages, you turned text into byte ids. Byte ids make long lists. `hi 👋` has 4 characters, but it takes 7 bytes.

A tokenizer shortens the list by giving a common pair of ids its own new id. The first step is to find the most frequent pair.

The `top-pair` command counts every pair of neighboring ids and prints the one that appears most often. Overlapping pairs count, so `aaa` has the pair `(97, 97)` twice. When two pairs tie, the pair that appears first, reading left to right, wins.

The ids of `aaabdaaabac` are `[97, 97, 97, 98, 100, 97, 97, 97, 98, 97, 99]`. The most frequent pair is:

```json
[97, 97]
```

That pair appears 4 times.

In `[8, 9, 1, 2]`, the pairs `(8, 9)`, `(9, 1)`, and `(1, 2)` each appear once. They all tie, so the first pair from the left wins:

```json
[8, 9]
```

### Tests

The tester will write the ids of `aaabdaaabac` to `ids.json` as a JSON array and execute your program like this:

```bash
$ ./your_program.sh top-pair ids.json
```

Your program must print the pair as a JSON array:

```json
[97, 97]
```

The tester will also run your program on a few other lists, including ties, overlapping pairs, and a list with only one pair.

### Notes

- Print only the winning pair. Later stages give that pair its own new id.
