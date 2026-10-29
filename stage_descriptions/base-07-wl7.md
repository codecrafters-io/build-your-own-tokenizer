In this stage, you'll split text the way GPT-2 does before any merge is applied.

### Chunks

In earlier stages, a merge could join any two neighboring ids. GPT-2 splits text
into chunks first, so a merge cannot cross from one chunk into the next. The
pattern is the one in
[`tiktoken`](https://github.com/openai/tiktoken/blob/main/tiktoken_ext/openai_public.py)
for its GPT-2 encoding.

Split with this pattern, encode each chunk with the merges file, and concatenate
the ids:

```text
'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s
```

The optional space before letters, numbers, and punctuation is the character
U+0020 only. Contractions are case sensitive, so `don't` splits into `don` and
`'t`, while `DON'T` splits into `DON`, `'`, and `T`. Marks such as Devanagari
vowel signs are their own chunks, because they are not letters. A non-breaking
space stays apart from the words around it.

The text `Hello world's 123!!` splits into `Hello`, ` world`, `'s`, ` 123`, and
`!!`. With merges `[[108, 108], [111, 32]]`, the ids are:

```json
[
  72, 101, 256, 111, 32, 119, 111, 114, 108, 100, 39, 115, 32, 49, 50, 51, 33,
  33
]
```

`ll` inside `Hello` becomes 256. The `o` and the following space are in
different chunks, so they stay 111 and 32. The apostrophe in the example is
U+0027.

### Tests

The tester will write `Hello world's 123!!` to `text.txt` and
`[[108, 108], [111, 32]]` to `merges.json`, then execute your program like this:

```bash
$ ./your_program.sh encode text.txt --merges merges.json --pretokenize gpt2
```

Your program must print the token ids as a JSON array:

```json
[
  72, 101, 256, 111, 32, 119, 111, 114, 108, 100, 39, 115, 32, 49, 50, 51, 33,
  33
]
```

The tester will also run your program on a few other files, including `DON'T`,
`हिंदी`, and a non-breaking space.

### Notes

- The apostrophe in the example is U+0027.
- Go, Rust, and C++ standard libraries cannot express this pattern. A
  left-to-right scan that tries the alternatives in order is enough.
