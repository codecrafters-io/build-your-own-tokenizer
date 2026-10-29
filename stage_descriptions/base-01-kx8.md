In this stage, you'll implement the first step of a tokenizer, a `bytes` command
that converts text into UTF-8 bytes.

### The `bytes` command

A language model can't read text. It only reads numbers, called token IDs. A
tokenizer turns the text into token IDs for the model.

The first step of a tokenizer is to convert raw text into bytes.

The `bytes` command reads a file and prints its
[UTF-8](https://www.rfc-editor.org/rfc/rfc3629) bytes as a JSON array, one
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
- Every byte is a number from 0 to 255. A larger number means you're printing
  characters, not bytes.
- The tester reads your output as JSON, so `[104,105]` and `[104, 105]` both
  pass. Print only the JSON array to stdout, and send debug output to stderr,
  like the starter code's log line.
