In this stage, you'll implement the `bytes` command.

### The `bytes` command

A language model can't read text. It only reads numbers, called token IDs. A tokenizer turns the text into token IDs for the model, before the model can start acting on it. 

Raw bytes are already numbers, but they make really long lists. A tokenizer groups common pieces of text, like `the` or `ing`, into one token ID each. A shorter list of tokens lets the model read more text at once.

The first step of a tokenizer is to convert text into [UTF-8](https://www.rfc-editor.org/rfc/rfc3629) bytes. The `bytes` command prints those bytes, one number each.

```bash
$ ./your_program.sh bytes text.txt
```

For a file that contains `hi 👋`, the output is:

```json
[104, 105, 32, 240, 159, 145, 139]
```

`h` is 104, `i` is 105, and the space is 32. The last four numbers, `240, 159, 145, 139`, are the wave emoji.

### Tests

The tester will write a UTF-8 file and execute your program like this:

```bash
$ ./your_program.sh bytes text.txt
```

Your program must print the bytes as a JSON array, like this:

```json
[104, 105, 32, 240, 159, 145, 139]
```

The tester will also check a file containing `🙂`, which prints `[240, 159, 153, 130]`, and an empty file, which prints `[]`.

