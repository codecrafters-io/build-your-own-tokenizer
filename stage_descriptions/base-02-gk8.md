Your `bytes` command gives each byte its own token. In this stage, you'll
implement `top-pair` to find the most frequent neighboring pair of token IDs.
This is how your tokenizer will choose which pair to combine when learning
larger tokens.

### Why combine pairs?

GPT-2 encodes `Hello world` as 2 tokens: `Hello`, and `world` with the space
before it. Your `bytes` command produces 11 tokens for the same text. Larger
tokens let a tokenizer represent recurring text with fewer IDs.

To learn larger tokens, your tokenizer will repeatedly combine the most frequent
pair in its training text. Encoding new text uses the rules learned during
training. This stage only finds a pair to combine.

### The `top-pair` command

The `top-pair` command counts every pair of neighboring IDs and prints the one
that appears most often. Overlapping pairs count, so `aaa` has the pair
`(97, 97)` twice. When two pairs tie, the pair that appears first, reading left
to right, wins.

Your program reads IDs from `ids.json` and parses it as JSON. The IDs are
already integers, so there is no UTF-8 step for this stage.

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

Your program must print the pair as a JSON array:

```json
[1, 2]
```

The tester will also run your program on a few other lists, including ties,
overlapping pairs, and the IDs of `aaabdaaabac`.

### Notes

- Print only the winning pair. Later stages give that pair its own new ID.
- The test lists contain at least two IDs, so there is always a pair to choose.
- Don't assume IDs fit in a byte. Later stages create IDs 256 and up and count
  their pairs the same way.
