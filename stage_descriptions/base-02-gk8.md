In this stage, you'll implement the `top-pair` command to find the pair of
tokens a tokenizer should combine next.

### Why combine pairs?

GPT-2 sees `Hello world` as 2 tokens, `Hello` and ` world`. Your `bytes` command
sees it as 11 bytes. A tokenizer gets from 11 down to 2 by repeatedly combining
the most common pair of tokens into a new token.

This stage finds the most common pair. Later stages combine it and repeat.

### The `top-pair` command

The `top-pair` command counts every pair of neighboring ids and prints the one
that appears most often. Overlapping pairs count, so `aaa` has the pair
`(97, 97)` twice. When two pairs tie, the pair that appears first, reading left
to right, wins.

Your program reads ids from `ids.json` and parses it as JSON. The ids are
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
overlapping pairs, and the ids of `aaabdaaabac`.

### Notes

- Print only the winning pair. Later stages give that pair its own new id.
- Don't assume ids fit in a byte. Later stages create ids 256 and up and count
  their pairs the same way.
