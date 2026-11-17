In this stage, you'll implement `bytes` to print a file's UTF-8 bytes as a JSON
array.

### Text as bytes

[UTF-8](https://www.rfc-editor.org/rfc/rfc3629) is an encoding that represents
text as a sequence of bytes. Each byte has an integer value from 0 to 255. A
character can take more than one byte, so reading a string's characters isn't
the same as reading its bytes.

For example, in UTF-8, `h` is encoded as the single byte `104`, `i` as `105`,
and a space as `32`. The wave emoji `👋` is encoded as four bytes:
`240, 159, 145, 139`. The file already contains these encoded bytes. Your
`bytes` command must print their values in file order, preserving all spaces,
newlines, and tabs.

### Bytes as tokens

A tokenizer converts text into a sequence of tokens. In the tokenizer you're
building, a token is a sequence of one or more bytes, represented by a numeric
ID. This tokenizer starts with one token per byte and uses the byte's value as
its ID. So the output of `bytes` is also the starting sequence of token IDs. For
`👋`, that means four starting tokens, not one token for the whole emoji. Later
stages will group bytes into longer tokens.

### Tests

The tester will write `hi 👋` to `text.txt`, with no trailing newline, and
execute your program like this:

```bash
$ ./your_program.sh bytes text.txt
```

Your program must print the byte values as a JSON array:

```json
[104, 105, 32, 240, 159, 145, 139]
```

The tester will also run your program on other files, including an empty file, a
file with only an emoji, and a file with several lines. An empty file must
produce `[]`.

### Notes

- Preserve whitespace at the start and end of the file too. In the test files, a
  newline is the single byte `10`. A space is `32`, and a tab is `9`.
- The tester parses your output as JSON, so spaces after commas are optional.
  Print only the JSON array to stdout, and send debug output to stderr.
