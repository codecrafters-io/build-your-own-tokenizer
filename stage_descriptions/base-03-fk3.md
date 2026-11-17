Your `top-pair` command finds a pair to combine. In this stage, you'll add
`merge` to replace a supplied pair of token IDs with the new ID `256`.

### Why merge?

GPT-2's first merge rule joins a space (byte 32) followed by `t` (byte 116), the
start of words like `the`, `to`, and `that` when they follow a space. The
resulting token has ID 256 in GPT-2's vocabulary.

Applying just this merge to a byte-token list saves one token for every `the`,
`to`, and `that` with a space before it. A word at the very start has no
preceding space, so its `t` stays separate after this merge.

32 and 116 are byte values. GPT-2's vocabulary numbers single bytes in its own
order, so its IDs for a space and `t` are 220 and 83. Later stages convert
between the two.

### The `merge` command

A merge gives a pair of IDs its own new ID. Your byte IDs go from 0 to 255, so
the first new ID is 256. This command applies one merge rule, so its new ID is
always 256. Later stages combine this with `top-pair` to train a tokenizer, and
give each new merge the next ID: 257, 258, and so on.

Like `top-pair`, the `merge` command parses `ids.json` as a JSON array of
integers. The `--pair` flag supplies two integers separated by a comma, with no
spaces. Order matters: `--pair 1,2` matches `1` followed by `2`, never `2`
followed by `1`.

Read the IDs from left to right. Each time the supplied pair matches, replace
those two IDs with `256` and continue after them. Keep unmatched IDs unchanged.
If the pair never appears, output the original list unchanged.

For example, in `[1, 2, 3, 1, 2]`, the pair `(1, 2)` appears twice. Replacing it
with `256` gives:

```json
[256, 3, 256]
```

Matches can't overlap. In `[97, 97, 97]`, reading from the left, the first two
IDs match `(97, 97)` and become `256`. The third `97` has nothing left to pair
with, so it stays:

```json
[256, 97]
```

This is different from counting. `top-pair` counts `(97, 97)` twice in this
list, but a merge can't use the middle `97` twice.

### Your progress

<details>
  <summary>22 → 18 tokens for the example sentence `to the top of the tree`</summary>

The sentence starts with 22 byte tokens. Applying `--pair 32,116` joins each
space followed by `t`. Four of its five words starting with `t` follow a space,
so this merge reduces the count by four:

```text
bytes     ██████████████████████  22 tokens
merge     ██████████████████      18 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

</details>

### Tests

The tester will write `[1, 2, 3, 1, 2]` to `ids.json` and execute your program
like this:

```bash
$ ./your_program.sh merge ids.json --pair 1,2
```

Your program must print the merged IDs as a JSON array to stdout:

```json
[256, 3, 256]
```

The tester will also run your program on a few other lists, including
overlapping matches, a pair that never appears, the IDs of `aaabdaaabac`, and
the bytes of the sentence from "Your progress".
