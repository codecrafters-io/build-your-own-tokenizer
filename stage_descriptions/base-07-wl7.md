Your encoder currently applies merges across the entire input. GPT-2, the
language model whose tokenizer you'll match, first splits text into chunks that
limit where pairs can merge. In this stage, you'll add `--pretokenize gpt2` to
split text before encoding each chunk.

### Why split before merging?

Without boundaries, BPE can learn separate tokens for `dog.`, `dog!`, and
`dog?`. Keeping the word and punctuation in separate chunks lets a token for
`dog` be reused with each punctuation mark.

GPT-2 splits text before learning or applying its merges. This step is called
**pretokenization**. A chunk is an intermediate piece of text, and encoding one
chunk can still produce several tokens. Merges must stay inside each chunk, even
if a rule would otherwise join it to its neighbor.

For this stage, add splitting only to `encode` when `--pretokenize gpt2` is
present. Keep `train`, `decode`, and encoding without the flag unchanged.

### Find the chunks

Collect the successive matches of GPT-2's pattern as the chunks:

```text
'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s
```

Here, `\p{L}` matches a Unicode letter, `\p{N}` matches a Unicode number, and
`\s` matches whitespace. The pattern groups runs of letters, numbers, other
symbols, or whitespace. Its first branch handles the contractions `'s`, `'d`,
`'m`, `'t`, `'ll`, `'ve`, and `'re`.

Use the matched text as input to your existing encoder. Keep every character,
including whitespace, and concatenate the resulting ID arrays in chunk order.
Each chunk starts from its own UTF-8 bytes and uses the same merge file.

For example, `Hello world's 123!!` splits into five chunks:

```text
"Hello"  " world"  "'s"  " 123"  "!!"
```

With the rules `[[108, 108], [111, 32]]`, `ll` inside `Hello` becomes 256. The
rule for `o` followed by a space cannot apply because those characters belong to
different chunks. These five chunks produce 18 token IDs with this merge file.

The exact pattern determines the boundaries, including these cases:

- The optional leading space is the ordinary space, U+0020. In `foo\u00a0bar`,
  where `\u00a0` denotes a non-breaking space, the chunks are `foo`, the
  non-breaking space, and `bar`.
- Contractions are case sensitive. `don't` splits into `don` and `'t`. `DON'T`
  splits into `DON`, `'`, and `T`. These apostrophes are U+0027.
- Unicode classifies some parts of written characters as combining marks. They
  fall outside `\p{L}`. The Hindi text `हिंदी` splits into `ह`, `िं`, `द`, and
  `ी`.

Use the supplied pattern for repeated spaces and line breaks too. Trimming
chunks or splitting only on ordinary spaces changes the result.

{{#lang_is_python}}

Your project includes the [`regex`](https://pypi.org/project/regex/) package,
which supports this pattern's Unicode properties and `++` syntax. Use
`import regex` in place of the built-in `re`.

{{/lang_is_python}}

{{#lang_is_typescript}}

TypeScript regexes do not support `++`. Replace each `++` with `+` and use the
`g` and `u` flags. Replace `\s` with `\p{White_Space}` and `\S` with
`\P{White_Space}` throughout the pattern. JavaScript's built-in whitespace class
differs for characters such as U+0085 and U+FEFF.

{{/lang_is_typescript}}

{{#lang_is_java}}

Compile the pattern with `Pattern.UNICODE_CHARACTER_CLASS` so `\s` also matches
Unicode whitespace such as a non-breaking space.

{{/lang_is_java}}

{{#lang_is_go}}

Go's `regexp` package has no lookahead. Your project includes
[`regexp2/v2`](https://pkg.go.dev/github.com/dlclark/regexp2/v2), which runs the
pattern unchanged. Compile it with `regexp2.None` and iterate with
`FindStringMatch` and `FindNextMatch`. With `regexp2.RE2`, `\s` does not match a
non-breaking space.

{{/lang_is_go}}

{{#lang_is_rust}}

The `regex` crate has no lookahead. Your project includes
[`fancy-regex`](https://docs.rs/fancy-regex/0.20.1/fancy_regex/), which runs the
pattern unchanged. Use `Regex::new` to compile it and `find_iter` to collect the
chunks.

{{/lang_is_rust}}

{{#lang_is_cpp}}

`std::regex` cannot run this pattern. Your project includes
[PCRE2](https://pcre2project.github.io/pcre2/doc/pcre2api/). Compile with
`PCRE2_UTF | PCRE2_UCP` and replace `\s` with `\p{White_Space}` and `\S` with
`\P{White_Space}` throughout the pattern. These explicit properties avoid
library-specific differences in `\s`, such as its handling of U+180E. Call
`pcre2_match` in a loop, starting each search where the last chunk ended.

{{/lang_is_cpp}}

### Tests

The tester writes `Hello world's 123!!` to `text.txt` and
`[[108, 108], [111, 32]]` to `merges.json`, then runs:

```bash
$ ./your_program.sh encode text.txt --merges merges.json --pretokenize gpt2
```

Your program must print:

```json
[
  72, 101, 256, 111, 32, 119, 111, 114, 108, 100, 39, 115, 32, 49, 50, 51, 33,
  33
]
```

The tester also checks uppercase contractions, Hindi combining marks,
non-breaking spaces, and `to the top of the tree`.

With the five merges learned from that sentence, splitting increases the output
from 11 tokens to 13. One learned rule joins ` the` to the ` t` of the next
word. The new chunk boundaries prevent that rule from applying.
