In this stage, you'll add `--pretokenize gpt2` to `encode`. Your encoder
currently allows merges across the whole input. This option will divide text
into chunks first and allow merges only within each chunk, matching GPT-2's
boundaries.

### Why split before merging?

GPT-2 separates words and punctuation before learning its merges. This lets
word pieces be reused with different punctuation instead of spending
vocabulary entries on combinations such as `dog.`, `dog!`, and `dog?`.
The [GPT-2 paper](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf)
describes this choice.

An encoder must apply the same boundaries used during training. This initial
split is called pre-tokenization. Each chunk is a region where merges are
allowed, and it can still produce several token IDs.

For now, keep using your existing JSON merge list. The next stage supplies
GPT-2's full vocabulary. This stage changes only encoding with the new option.
Keep `train` and encoding without the option working as before.

### Finding chunks

Use this case-sensitive pattern from
[`tiktoken`](https://github.com/openai/tiktoken/blob/main/tiktoken_ext/openai_public.py):

```text
'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s
```

Collect the complete regex matches in order. Each match is a chunk to encode.
The matches are the text you keep.

The alternatives match lowercase contractions, runs of letters, runs of
numbers, punctuation, and whitespace. Here, `\p{L}` means Unicode letters,
`\p{N}` means Unicode numbers, and `\s` means whitespace. The optional space
before letters, numbers, and punctuation is one ordinary space, U+0020.
Use the full expression so its whitespace alternatives stay intact.

For example, `Hello world's 123!!` produces five chunks:

```text
"Hello"  " world"  "'s"  " 123"  "!!"
```

Convert each chunk to UTF-8 bytes and apply your existing rank-based encoding
to it. Concatenate the resulting ID lists into one flat list. With
`[[108, 108], [111, 32]]` as the merges, `ll` inside `Hello` becomes ID 256.
The `o` and the following space are in different chunks, so their pair cannot
merge.

### Boundaries to preserve

- The apostrophe is U+0027. Lowercase contractions include `'s`, `'d`, `'m`,
  `'t`, `'ll`, `'ve`, and `'re`. Thus `don't` splits into `don` and `'t`,
  while `DON'T` splits into `DON`, `'`, and `T`.
- A non-breaking space, U+00A0, is handled by the whitespace alternatives.
  Between `foo` and `bar`, one non-breaking space forms a separate chunk.
  Whitespace runs can form larger chunks, so follow the complete pattern.
- Unicode marks are outside `\p{L}`. The vowel signs in `हिंदी` are marks,
  so that input splits into `ह`, `िं`, `द`, and `ी`.

### Tests

The tester will write `Hello world's 123!!` to `text.txt` and
`[[108, 108], [111, 32]]` to `merges.json`, then run:

```bash
$ ./your_program.sh encode text.txt --merges merges.json --pretokenize gpt2
```

Your program must print the concatenated token IDs:

```json
[
  72, 101, 256, 111, 32, 119, 111, 114, 108, 100, 39, 115, 32, 49, 50, 51, 33,
  33
]
```

The five chunks produce 18 IDs with this small merge list. The tester also
checks uppercase and lowercase contractions, Hindi text, non-breaking spaces,
and the sentence from the progress note below.

### Notes

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

<details>
  <summary>Your progress: enforcing chunk boundaries</summary>

With the same five learned rules, `to the top of the tree` now encodes to 13
IDs instead of 11. One rule previously joined ` the` to ` t` at the start
of the next word. Those pieces now belong to different chunks, so that rule
cannot apply.

The extra IDs are expected. These rules were learned from the sentence
without chunk boundaries. The next stage uses GPT-2's vocabulary, whose merges
were learned with its boundaries.

</details>
