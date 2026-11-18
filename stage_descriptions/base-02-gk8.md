Your tokenizer starts with one token per byte. In this stage, you'll implement
`top-pair` to choose which neighboring pair to combine when learning a larger
vocabulary.

### Why combine pairs?

A token can represent several bytes. Giving a frequent byte sequence its own
ID lets a tokenizer represent that sequence with fewer tokens.

For example, GPT-2's learned merge rules turn the 11 bytes of `Hello world`
into two tokens: `Hello` and ` world`. Learning rules like these starts by
finding a frequent pair in training text. Encoding new text later reuses the
learned rules.

This stage only selects the pair. Combining its tokens comes next.

### The `top-pair` command

The `top-pair` command counts every pair of neighboring IDs and prints the one
that appears most often. Overlapping pairs count, so `aaa` has the pair
`(97, 97)` twice. When two pairs tie, the pair that appears first, reading left
to right, wins.

Your program reads IDs from `ids.json` and parses it as JSON. The IDs are
already integers, so there is no UTF-8 step for this stage.

For example, in `[1, 2, 3, 1, 2]`, the pairs are `(1, 2)`, `(2, 3)`, `(3, 1)`,
and `(1, 2)` again. The pair `(1, 2)` appears twice, so the output is `[1, 2]`.

In `[8, 9, 1, 2]`, the pairs `(8, 9)`, `(9, 1)`, and `(1, 2)` each appear once.
They all tie, so the first pair from the left wins: `[8, 9]`.

In `[1, 2, 3, 4, 3, 4, 1, 2]`, `(1, 2)` and `(3, 4)` both appear twice. `(1, 2)`
appears first, so it wins again: `[1, 2]`.

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
- The tests provide at least two IDs, so there is always a pair to select.
- Don't assume IDs fit in a byte. Later stages create IDs 256 and up and count
  their pairs the same way.
