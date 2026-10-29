In this stage, you'll implement the `merge` command to replace a pair of ids with a new id.

### The `merge` command

A merge gives a pair of ids its own new id. Byte ids go from 0 to 255, so the new id in this stage is always 256. Later stages combine this with the most frequent pair from earlier stages to train a tokenizer.

The `merge` command reads the ids from left to right. Each time it finds the pair, it replaces those two ids with the new id.

`hihi` is `[104, 105, 104, 105]`. Replacing the pair `(104, 105)` with `256` gives:

```json
[256, 256]
```

Each id can be part of only one match. In `[97, 97, 97]`, the first two ids become `256`, and the last `97` has no partner left. Replacing the pair `(97, 97)` with `256` gives:

```json
[256, 97]
```

### Tests

The tester will write the ids of `aaabdaaabac` to `ids.json` as a JSON array and execute your program like this:

```bash
$ ./your_program.sh merge ids.json --pair 97,97
```

Your program must print the merged ids as a JSON array:

```json
[256, 97, 98, 100, 256, 97, 98, 97, 99]
```

The tester will also run your program on a few other lists, including overlapping matches and a pair that never appears.

### Notes

- `--pair` is two integers separated by a comma, with no spaces.
- Later stages create more ids, counting up from 256.
