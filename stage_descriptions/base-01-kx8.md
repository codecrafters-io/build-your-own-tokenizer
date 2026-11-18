Before your tokenizer can group text into pieces, it needs a consistent way to
represent the input. In this stage, you'll add a `bytes` command that reads a
text file and prints its UTF-8 bytes.

### Tokens, IDs, and bytes

A tokenizer converts text into a sequence of pieces called **tokens**. Each
token has an integer label called a **token ID**, which is how the tokenizer
represents that piece to a language model.

In this build, each token initially holds one byte. A byte is a value from 0
to 255. **UTF-8** represents text using these byte values, sometimes using
several bytes for one character. Starting with all 256 possible byte values lets
your tokenizer represent any UTF-8 text, including words it has never
encountered.

For now, a token's ID is its byte value. The text `hi 👋` becomes:

```json
[104, 105, 32, 240, 159, 145, 139]
```

`h` is byte 104, `i` is 105, and the space is 32. The wave emoji takes the last
four bytes. One visible character can therefore occupy several starting tokens.

### The `bytes` command

Read the file named after `bytes` and print its bytes in order as a JSON array.
Preserve the entire file, including spaces, tabs, and line endings. A space is
32, a tab is 9, and a newline represented by `\n` is 10. An empty file produces
`[]`.

Print exactly one JSON value to stdout. Send debug messages to stderr, as the
starter code's log line does. The tester parses the JSON, so spacing between
array elements does not matter.

The starter contains a commented-out implementation of this command. Find that
block and enable it to pass this stage.

### Tests

The tester creates a UTF-8 file called `text.txt` containing `hi 👋`, with no
trailing newline, and runs:

```bash
$ ./your_program.sh bytes text.txt
```

The expected output is the seven-byte array shown above. The tester also checks
an empty file, an emoji on its own, and text with multiple lines. Every output
value must be between 0 and 255.
