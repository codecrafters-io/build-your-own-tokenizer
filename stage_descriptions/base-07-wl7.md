Your encoder currently applies merges to the whole text. In this stage, you'll
add `--pretokenize gpt2` to split text into chunks and merge within each one.

### Why split first?

The
[GPT-2 paper](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf)
describes splitting text before learning merges to avoid separate tokens for
`dog.`, `dog!`, and `dog?`.

GPT-2 encodes `dog. dog! dog?` as `dog`, `.`, `␠dog`, `!`, `␠dog`, and `?`. Here
`␠` marks a space, and `␠dog` is reused rather than joined to punctuation.

### Splitting with GPT-2's pattern

Keep using your custom merge list and token IDs. Without the new flag, `encode`
must keep its whole-text behavior.

Use this GPT-2 splitting pattern from
[`tiktoken`](https://github.com/openai/tiktoken/blob/main/tiktoken_ext/openai_public.py):

```text
'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s
```

The pattern starts with an apostrophe, U+0027. Its first branch matches the
contractions `'s`, `'d`, `'m`, `'t`, `'ll`, `'ve`, and `'re`. `\p{L}` matches
Unicode letters and `\p{N}` matches Unicode numbers. The ` ?` branches can
attach one ordinary space, U+0020, to a following run. The final branches handle
other whitespace runs.

Use the matches as chunks, preserving all characters in order. Convert each
chunk to UTF-8 bytes, apply your ranked merges, and concatenate the IDs. Never
merge across chunk boundaries.

For example, `Hello world's 123!!` splits into five chunks:

```text
"Hello"  " world"  "'s"  " 123"  "!!"
```

Chunks are not final tokens: each chunk can produce several token IDs. With
merges `[[108, 108], [111, 32]]`, the output is:

```json
[
  72, 101, 256, 111, 32, 119, 111, 114, 108, 100, 39, 115, 32, 49, 50, 51, 33,
  33
]
```

`ll` inside `Hello` becomes 256. The `o` and the following space are in
different chunks, so they stay 111 and 32.

These examples illustrate the pattern's boundaries:

- The optional leading space is U+0020 only. In `foo\u00A0bar`, `\u00A0` denotes
  one non-breaking space. The chunks are `foo`, that space, and `bar`. Preserve
  whitespace and use the pattern's matches.
- Contractions are case sensitive. `don't` splits into `don` and `'t`, and
  `DON'T` splits into `DON`, `'`, and `T`.
- The combining marks in `हिंदी` are outside `\p{L}` and `\p{N}`. The word
  splits into `ह`, `िं`, `द`, and `ी`. Marks can share a chunk with punctuation.

### Your progress

<details>
  <summary>11 → 13 tokens for the example sentence `to the top of the tree`</summary>

The same five learned merges now produce 13 tokens rather than 11:

```text
bytes     ██████████████████████  22 tokens
train     ███████████             11 tokens
split     █████████████           13 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← the goal
```

One learned rule joins `␠the` to the `␠t` of the next word. Chunk boundaries now
block that merge, so the higher count is expected.

</details>

### Regex libraries and Unicode versions

The reference uses Unicode's `White_Space` property for `\s`. Some libraries
need the syntax or whitespace adaptations below.

Letter and number categories also depend on the Unicode version. Even the
challenge's Python reference and native `tiktoken` disagree on some recently
added letters. Passing these tests does not guarantee identical tokenization
across Unicode versions.

{{#lang_is_python}}

- Your project already includes the [`regex`](https://pypi.org/project/regex/)
  package, which supports `\p{L}` and `++`. Use `import regex` in place of the
  built-in `re`.

{{/lang_is_python}}

{{#lang_is_typescript}}

- JavaScript regexes don't support possessive quantifiers. For this pattern,
  replace each `++` with `+`. Also replace `\s` with `\p{White_Space}` and `\S`
  with `\P{White_Space}`: JavaScript's `\s` includes U+FEFF and excludes U+0085,
  unlike the reference. Use `g` to collect all matches and `u` for Unicode
  property escapes such as `\p{L}`.

{{/lang_is_typescript}}

{{#lang_is_java}}

- Compile the pattern with `Pattern.UNICODE_CHARACTER_CLASS` so `\s` uses
  Unicode whitespace, including non-breaking spaces. This flag does not change
  the Unicode version supplied by Java.

{{/lang_is_java}}

{{#lang_is_go}}

- Go's `regexp` package has no lookahead, so your project already includes
  [`regexp2/v2`](https://pkg.go.dev/github.com/dlclark/regexp2/v2). Its included
  version supports this pattern's syntax. Compile it with `regexp2.None` and
  step through the chunks with `FindStringMatch` and `FindNextMatch`. With
  `regexp2.RE2`, `\s` doesn't match a non-breaking space.

{{/lang_is_go}}

{{#lang_is_rust}}

- The `regex` crate has no lookahead, so your project already includes
  [`fancy-regex`](https://docs.rs/fancy-regex/0.20.1/fancy_regex/), which
  supports this pattern's syntax. `Regex::new` compiles it, and `find_iter`
  gives the chunks.

{{/lang_is_rust}}

{{#lang_is_cpp}}

- `std::regex` has no `\p{L}`, so your project already includes
  [PCRE2](https://pcre2project.github.io/pcre2/doc/pcre2api/). Replace `\s` with
  `\p{White_Space}` and `\S` with `\P{White_Space}`: the included PCRE2
  version's `\s` also treats U+180E as whitespace, unlike the reference. Compile
  with `PCRE2_UTF | PCRE2_UCP`, then call `pcre2_match` in a loop, starting each
  search where the last chunk ended. The explicit whitespace properties do not
  update PCRE2's letter and number tables. The Unicode-version limit above still
  applies.

{{/lang_is_cpp}}

### Tests

The tester will write `Hello world's 123!!` to `text.txt` and
`[[108, 108], [111, 32]]` to `merges.json`, then execute your program like this:

```bash
$ ./your_program.sh encode text.txt --merges merges.json --pretokenize gpt2
```

Your program must print the token IDs as a JSON array to stdout:

```json
[
  72, 101, 256, 111, 32, 119, 111, 114, 108, 100, 39, 115, 32, 49, 50, 51, 33,
  33
]
```

The tester will also run your program on a few other files, including `DON'T`,
`हिंदी`, a non-breaking space, and the sentence from "Your progress".
