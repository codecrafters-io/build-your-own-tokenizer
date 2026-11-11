In this stage, you'll implement the `--pretokenize gpt2` option to split text
into chunks before merging, the way GPT-2 does.

### Your progress

<details>
  <summary>11 → 13 tokens for the example sentence `to the top of the tree`</summary>

Here's how many tokens the sentence `to the top of the tree` takes so far:

```text
bytes     ██████████████████████  22 tokens
train     ███████████             11 tokens
split     █████████████           13 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

</details>

### Why split first?

The
[GPT-2 paper](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf)
found plain BPE spending merges on `dog.`, `dog!`, and `dog?`, one token for
each variation. So GPT-2's merges were learned on text that was split into
chunks first, and no merge ever joined two chunks.

An encoder has to split the same way. Otherwise it merges across boundaries
that training never crossed, and produces tokens GPT-2 never saw in those
places. GPT-2 encodes `dog. dog! dog?` as `dog`, `.`, `␠dog`, `!`, `␠dog`, and
`?`, where `␠` marks a space. `␠dog` is one token, used twice.

### Splitting with GPT-2's pattern

GPT-2 splits with this pattern, from
[`tiktoken`](https://github.com/openai/tiktoken/blob/main/tiktoken_ext/openai_public.py):

```text
'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s
```

Split the text with this pattern, encode each chunk with the merges from
`--merges`, and concatenate the ids. A space joins the chunk that follows it.
For example, the text `Hello world's 123!!` splits into five chunks:

```text
"Hello"  " world"  "'s"  " 123"  "!!"
```

These are exactly GPT-2's five tokens for it. With merges
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

{{#lang_is_python}}

- Your project already includes the [`regex`](https://pypi.org/project/regex/)
  package, which supports `\p{L}` and `++`. Use `import regex` in place of the
  built-in `re`.

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

- Go's `regexp` package has no lookahead, so your project already includes
  [`regexp2/v2`](https://pkg.go.dev/github.com/dlclark/regexp2/v2), which runs
  the pattern unchanged. Compile it with `regexp2.None` and step through the
  chunks with `FindStringMatch` and `FindNextMatch`. With `regexp2.RE2`, `\s`
  doesn't match a non-breaking space.

{{/lang_is_go}}

{{#lang_is_rust}}

- The `regex` crate has no lookahead, so your project already includes
  [`fancy-regex`](https://docs.rs/fancy-regex/0.20.1/fancy_regex/), which runs
  the pattern unchanged. `Regex::new` compiles it, and `find_iter` gives the
  chunks.

{{/lang_is_rust}}

{{#lang_is_cpp}}

- `std::regex` has no `\p{L}`, so your project already includes
  [PCRE2](https://pcre2project.github.io/pcre2/doc/pcre2api/), which runs the
  pattern unchanged. Compile it with `PCRE2_UTF | PCRE2_UCP`, then call
  `pcre2_match` in a loop, starting each search where the last chunk ended.
  Without `PCRE2_UCP`, `\s` doesn't match a non-breaking space.

{{/lang_is_cpp}}
