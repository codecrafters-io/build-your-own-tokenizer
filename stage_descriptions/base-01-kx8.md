Your tokenizer will start with one token per byte before learning to combine
them. In this stage, you'll implement `bytes`, a command that prints a file's
bytes as numbers.

### Starting with bytes

A tokenizer represents text as a sequence of token IDs for a language model.
Each ID identifies a piece of the input, called a token. In this challenge,
tokens start as single bytes and later grow to represent longer pieces.

A byte holds an integer from 0 to 255.
[UTF-8](https://www.rfc-editor.org/rfc/rfc3629) represents each Unicode
character as one or more bytes. For example, `h` takes one byte, while `👋`
takes four.

Those 256 possible byte values can represent text in any language. They give
your tokenizer a fixed starting vocabulary, the set of tokens it recognizes.
For now, each token's ID equals its byte value.

The `bytes` command reads a file's bytes in order and prints them as a JSON
array to stdout. You don't need to split words or combine bytes yet.

### Tests

The tester will write `hi 👋` to `text.txt` and execute your program like this:

```bash
$ ./your_program.sh bytes text.txt
```

Your program must print the bytes as a JSON array:

```json
[104, 105, 32, 240, 159, 145, 139]
```

`h` is 104, `i` is 105, and the space is 32. The last four numbers,
`240, 159, 145, 139`, together represent the wave emoji.

Other inputs include an empty file, an emoji on its own, and several lines of
text.

### Notes

- Spaces, newlines, and tabs are bytes too. Keep every one of them: a space is
  `32`, a newline is `10`, and a tab is `9`. An empty file has no bytes, so its
  output is `[]`.
- Every output number must be between 0 and 255. A character's Unicode number
  can exceed that range, so reading character numbers would give different
  results.
- The tester reads your output as JSON, so `[104,105]` and `[104, 105]` both
  pass. Print only the JSON array to stdout, and send debug output to stderr,
  like the starter code's log line.
