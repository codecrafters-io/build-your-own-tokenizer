In this stage, you'll implement a `bytes` command that converts text into UTF-8
bytes, the starting tokens of your tokenizer.

### The `bytes` command

A language model can't read text. It reads numbers called token IDs, one per
token. A tokenizer turns text into that sequence of numbers.

[UTF-8](https://www.rfc-editor.org/rfc/rfc3629) writes text as bytes, and each
byte has a value from 0 to 255. Those 256 values cover every UTF-8 text, so a
tokenizer that starts with all of them can read any input. In this stage, each
byte is its own token, and its value is its token ID. Later stages merge bytes
into larger tokens.

The `bytes` command reads a file and prints its bytes as a JSON array, one
number per byte.

```bash
$ ./your_program.sh bytes text.txt
```

For example, if the file contains `hi 👋`, the output is:

```json
[104, 105, 32, 240, 159, 145, 139]
```

`h` is 104, `i` is 105, and the space is 32. The last four numbers,
`240, 159, 145, 139`, are the wave emoji.

### Tests

The tester will write `hi 👋` to `text.txt` and execute your program like this:

```bash
$ ./your_program.sh bytes text.txt
```

Your program must print the bytes as a JSON array:

```json
[104, 105, 32, 240, 159, 145, 139]
```

The tester will also run your program on a few other files, including an empty
file, a file with only an emoji, and a file with several lines.

### Notes

- Spaces, newlines, and tabs are bytes too. Keep every one of them: a space is
  `32`, a newline is `10`, and a tab is `9`. An empty file has no bytes, so its
  output is `[]`.
- Every value you print must be between 0 and 255, inclusive. A number above 255
  means you're printing a character instead of a byte.
- The tester reads your output as JSON, so `[104,105]` and `[104, 105]` both
  pass. Print only the JSON array to stdout, and send debug output to stderr,
  like the starter code's log line.
