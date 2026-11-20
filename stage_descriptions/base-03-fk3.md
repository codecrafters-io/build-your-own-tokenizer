In this stage, you'll implement the `merge` command to combine a pair of tokens
into a new token.

### Your progress

<details>
  <summary>22 → 18 tokens for the example sentence `to the top of the tree`</summary>

Here's how many tokens the sentence takes after one merge:

```text
bytes     ██████████████████████  22 tokens
merge     ██████████████████      18 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

</details>

### Why merge?

Merging is how GPT-2 grew its vocabulary beyond the 256 byte values. In its
training text, a space followed by `t` was the most common pair of bytes, the
start of words like `the`, `to`, and `that`. GPT-2 merged that pair first, so
`the`, `to`, and `that` take one token fewer wherever a space comes before them.

A word at the start of a text has no space before it, so its `t` stays separate.
In `to the top of the tree`, 4 of the 5 words that start with `t` follow a
space, so the sentence drops from 22 tokens to 18.

This stage counts in byte values, so the pair is `(32, 116)` and the merged
token is 256. GPT-2's own vocabulary file numbers the same two tokens
differently, and later stages switch to those numbers.

### The `merge` command

A merge gives a pair of ids its own new id. Byte ids go from 0 to 255, so the
first free id is 256. This stage makes one merge, so its new id is always 256.
Later stages combine `merge` with `top-pair` to train a tokenizer and give each
new pair the next id.

Like `top-pair`, the `merge` command parses `ids.json` as a JSON array of
integers. It reads the ids from left to right. Each time it finds the pair given
by `--pair`, it replaces those two ids with the new id.

For example, in `[1, 2, 3, 1, 2]`, the pair `(1, 2)` appears twice. Replacing it
with `256` gives:

```json
[256, 3, 256]
```

Matches can't overlap. In `[97, 97, 97]`, reading from the left, the first two
ids match `(97, 97)` and become `256`. The third `97` has nothing left to pair
with, so it stays:

```json
[256, 97]
```

This is different from counting. `top-pair` counts `(97, 97)` twice in this
list, but a merge can't use the middle `97` twice.

### Tests

The tester will write `[1, 2, 3, 1, 2]` to `ids.json` and execute your program
like this:

```bash
$ ./your_program.sh merge ids.json --pair 1,2
```

Your program must print the merged ids as a JSON array:

```json
[256, 3, 256]
```

The tester will also run your program on a few other lists, including
overlapping matches, a pair that never appears, and the ids of `aaabdaaabac`.

### Notes

- `--pair` is two integers separated by a comma, with no spaces.
- Order matters. `--pair 1,2` matches `1` followed by `2`, never `2` followed by
  `1`.
- GPT-2's vocabulary file gives a space and `t` the IDs 220 and 83. This stage
  uses byte values (32 and 116) instead, and later stages load GPT-2's file.
