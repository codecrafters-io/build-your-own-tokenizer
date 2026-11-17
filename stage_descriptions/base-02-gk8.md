Your `bytes` command exposes the starting tokens. In this stage, you'll add
`top-pair` to find the most frequent neighboring pair in a JSON list of token
IDs.

### Why combine pairs?

GPT-2 encodes `Hello world` as 2 tokens: `Hello`, and `world` with the space
before it. Your `bytes` command outputs 11 bytes. A token can represent several
bytes joined together.

To learn which tokens to combine, your training algorithm will repeatedly find
the most frequent pair and merge it. This stage only finds the pair. Later
stages combine it and repeat. Encoding new text will use those learned merges
rather than learn new ones.

### The `top-pair` command

The `top-pair` command counts every ordered pair of neighboring IDs and prints
the one that appears most often. Overlapping pairs count, so `aaa` has the pair
`(97, 97)` twice. When two pairs tie, the pair that appears first, reading left
to right, wins.

Your program reads IDs from `ids.json` and parses it as a JSON array of
integers. There is no UTF-8 conversion for this command. IDs can be 256 or
greater: later merges create tokens beyond the initial 256 byte values, and
their pairs are counted the same way.

For example, in `[1, 2, 3, 1, 2]`, the pairs are `(1, 2)`, `(2, 3)`, `(3, 1)`,
and `(1, 2)` again. The pair `(1, 2)` appears twice, so the most frequent pair
is:

```json
[1, 2]
```

In `[8, 9, 1, 2]`, the pairs `(8, 9)`, `(9, 1)`, and `(1, 2)` each appear once.
They all tie, so the first pair from the left wins:

```json
[8, 9]
```

In `[1, 2, 3, 4, 3, 4, 1, 2]`, `(1, 2)` and `(3, 4)` both appear twice. `(1, 2)`
appears first, so it wins:

```json
[1, 2]
```

### Tests

The tester will write `[1, 2, 3, 1, 2]` to `ids.json` and execute your program
like this:

```bash
$ ./your_program.sh top-pair ids.json
```

Your program must print only the winning pair as a JSON array to stdout:

```json
[1, 2]
```

The tester will also run your program on a few other lists, including ties,
overlapping pairs, IDs above 255, and the IDs of `aaabdaaabac`. The test lists
contain at least two IDs. Do not merge the pair or assign it a new ID yet.
