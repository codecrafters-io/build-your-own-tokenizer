In this stage, you'll implement the `--pretokenize gpt2` option to split text
into chunks before merging, the way GPT-2 does.

### Your progress

<details>
  <summary>11 → 13 tokens for the example sentence `to the top of the tree`</summary>

Here's how many tokens the sentence takes so far:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens
split     █████████████           13 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

The count rises here because a merge cannot cross a chunk boundary. The five
merges from the previous stages were learned without splitting, so some of them
joined text that now falls on opposite sides of a boundary. GPT-2 learned its
merges on split text, so its merges only ever work within a chunk.

</details>

### Why split first?

The
[GPT-2 paper](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf)
found plain BPE spending merges on `dog.`, `dog!`, and `dog?`, one token for
each variation. GPT-2 splits text into chunks before it merges, and no merge
joins two chunks. It encodes `dog. dog! dog?` as `dog`, `.`, ` dog`, `!`,
` dog`, and `?`, so ` dog` is one token, used twice.

An encoder has to split the same way. Otherwise a merge can join pieces that
GPT-2 kept apart, and the token IDs no longer match.

### Splitting with GPT-2's pattern

GPT-2 splits with this pattern, from
[`tiktoken`](https://github.com/openai/tiktoken/blob/main/tiktoken_ext/openai_public.py):

```text
'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s
```

The `|` characters separate seven alternatives, and the first one that matches
at a position wins:

- `'(?:[sdmt]|ll|ve|re)` matches an apostrophe followed by `s`, `d`, `m`, `t`,
  `ll`, `ve`, or `re`, the endings of contractions.
- ` ?\p{L}++` matches a run of Unicode letters. The ` ?` lets one ASCII space
  join the run.
- ` ?\p{N}++` does the same for a run of Unicode digits.
- ` ?[^\s\p{L}\p{N}]++` matches a run of anything else that is not whitespace,
  again with one optional space in front.
- `\s++$` matches whitespace at the end of the text. `\s+(?!\S)` matches
  whitespace followed by more whitespace, so a run can stop one space before the
  next chunk. `\s` matches a single whitespace character on its own.

Whitespace that no earlier alternative claimed becomes its own chunk.
`foo   bar` splits into `foo`, `"  "`, and `" bar"`, so the two spaces in the
middle stay together and the last space joins `bar`.

Each match of the pattern is one chunk. Encode each chunk with the merges from
`--merges`, and concatenate the ids in chunk order. A merge applies only inside
a chunk, so two ids from different chunks never combine.

For example, the text `Hello world's 123!!` splits into five chunks:

```text
"Hello"  " world"  "'s"  " 123"  "!!"
```

GPT-2's vocabulary holds a single token for each of these chunks. With merges
`[[108, 108], [111, 32]]`, the ids are:

```json
[
  72, 101, 256, 111, 32, 119, 111, 114, 108, 100, 39, 115, 32, 49, 50, 51, 33,
  33
]
```

`ll` inside `Hello` becomes 256. The `o` and the following space are in
different chunks, so they stay 111 and 32.

Three rules settle the tricky cases:

- The optional space before letters, numbers, and punctuation is U+0020 only. A
  non-breaking space is a chunk of its own.
- Contractions are case sensitive. `don't` splits into `don` and `'t`, and
  `DON'T` splits into `DON`, `'`, and `T`.
- Devanagari vowel signs are marks, outside `\p{L}`, so a run of them is its own
  chunk. `हिंदी` splits into `ह`, `िं`, `द`, and `ी`.

With `--pretokenize gpt2`, the `encode` command reads the same `text.txt` and
`merges.json` as before.

### Tests

The tester will write `Hello world's 123!!` to `text.txt` and
`[[108, 108], [111, 32]]` to `merges.json`, then execute your program like this:

```bash
$ ./your_program.sh encode text.txt --merges merges.json --pretokenize gpt2
```

Your program must print the token IDs as a JSON array:

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

{{#lang_is_python}}

- Python's built-in `re` doesn't support `\p{L}`. Use the
  [`regex`](https://pypi.org/project/regex/) package (`uv add regex`) instead.

{{/lang_is_python}}

{{#lang_is_typescript}}

- TypeScript regexes don't support possessive quantifiers. Replace each `++`
  with `+`, which gives the same chunks here, and use the `g` and `u` flags.
  Without `u`, `\p{L}` matches the literal text `p{L}`.

{{/lang_is_typescript}}

{{#lang_is_java}}

- Compile the pattern with `Pattern.UNICODE_CHARACTER_CLASS`. Otherwise `\s`
  doesn't match a non-breaking space.

{{/lang_is_java}}

{{#lang_is_go}}

- Go's standard library cannot express this pattern. A left-to-right scan that
  tries the alternatives in order is enough.

{{/lang_is_go}}

{{#lang_is_rust}}

- Rust's standard library cannot express this pattern. A left-to-right scan that
  tries the alternatives in order is enough.

{{/lang_is_rust}}

{{#lang_is_cpp}}

- C++'s standard library cannot express this pattern. A left-to-right scan that
  tries the alternatives in order is enough.

{{/lang_is_cpp}}
