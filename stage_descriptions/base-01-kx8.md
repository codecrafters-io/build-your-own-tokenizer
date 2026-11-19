In this stage, you'll implement a `bytes` command that reads a file and prints
its UTF-8 bytes.

### The `bytes` command

GPT-2 receives text as a sequence of numbers called token IDs. Each ID
identifies a piece of text, and a tokenizer converts text into this sequence.
These bytes are the starting tokens for the tokenizer you'll build.

[UTF-8](https://www.rfc-editor.org/rfc/rfc3629) represents text as bytes, each
with a value from 0 to 255. A character can occupy several bytes. Starting with
all 256 byte values lets our tokenizer represent any UTF-8 text before it learns
larger tokens.

The `bytes` command reads a file's bytes in order and prints them as a JSON
array, one number per byte. At this stage, each byte's value is also its token
ID.

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
- Every output value must be between 0 and 255, inclusive. A larger number means
  you're printing characters instead of bytes.
- The tester reads your output as JSON, so `[104,105]` and `[104, 105]` both
  pass. Print only the JSON array to stdout, and send debug output to stderr,
  like the starter code's log line.
