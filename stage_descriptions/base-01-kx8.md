These bytes will be the starting tokens for your tokenizer. In this stage, you'll implement a `bytes` command that reads a text file and prints its UTF-8 bytes.

### Starting with bytes

A tokenizer turns text into a sequence of numbers called token IDs. Each ID identifies a piece of text, called a token, that a language model can use as input.

Your tokenizer will begin with one token for each possible byte value, from 0 to 255. For these initial tokens, the ID equals the byte value. Later stages combine frequently occurring sequences into larger tokens.

[UTF-8](https://www.rfc-editor.org/rfc/rfc3629) represents text as bytes. Starting with all 256 byte values lets your tokenizer represent any UTF-8 text without needing a separate vocabulary entry for every word. A character can use several bytes: `h` uses one, while `👋` uses four.

The input file already contains UTF-8 bytes. Read those bytes in order and keep them unchanged, including spaces, tabs, and newlines. A space is `32`, a tab is `9`, and a newline is `10`.

### Output format

The tester parses stdout as JSON, so spaces and line breaks around the numbers are allowed. Print only the JSON result to stdout. Send debugging messages to stderr, as the starter code's log line does.

### Tests

The tester will write `hi 👋` to `text.txt`, with no trailing newline, and run:

```bash
$ ./your_program.sh bytes text.txt
```

Your program must print one JSON array of byte values to stdout:

```json
[104, 105, 32, 240, 159, 145, 139]
```

Here, `104` is `h`, `105` is `i`, and `32` is the space. The remaining four bytes, `240, 159, 145, 139`, together represent `👋`.

The tester also uses an empty file, a file containing only an emoji, and files with multiple lines. An empty file must produce `[]`.
