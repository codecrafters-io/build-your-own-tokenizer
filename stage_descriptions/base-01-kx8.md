In this stage, you'll implement a `bytes` command that reads a UTF-8 text file
and prints its bytes as a JSON array.

### The `bytes` command

A tokenizer turns text into numbered pieces called tokens. The numbers are token
IDs. Your byte-level tokenizer will start with one token for each byte, then
learn to combine neighboring tokens in later stages.

[UTF-8](https://www.rfc-editor.org/rfc/rfc3629) represents text as a sequence of
bytes. Each byte has a value from 0 to 255, and a character can occupy more than
one byte. For now, print the bytes without combining them.

The `bytes` command takes a file path:

```bash
$ ./your_program.sh bytes text.txt
```

For example, if the file contains `hi 👋` with no trailing newline, the output
is:

```json
[104, 105, 32, 240, 159, 145, 139]
```

`h` is byte 104, `i` is 105, and the space is 32. The wave emoji uses four
bytes: `240, 159, 145, 139`. Iterating over characters instead of bytes would
miss this distinction.

Read the file's bytes without trimming or changing them. Keep spaces (`32`),
newlines (`10` for a line feed), and tabs (`9`). An empty file has no bytes, so
its output is `[]`.

### Tests

The tester will write `hi 👋` to `text.txt` and execute your program like this:

```bash
$ ./your_program.sh bytes text.txt
```

Your program must print one number per byte as a JSON array to stdout:

```json
[104, 105, 32, 240, 159, 145, 139]
```

The tester will also run your program on a few other files, including an empty
file, a file with only an emoji, and a file with several lines.

Every output number must be between 0 and 255. The tester parses your output as
JSON, so `[104,105]` and `[104, 105]` both pass. Print only the JSON array to
stdout, and send debug output to stderr, like the starter code's log line.
