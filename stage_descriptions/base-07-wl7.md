In this stage, you'll add `--pretokenize gpt2` to `encode` to split Unicode text
into chunks before byte conversion and merging.

### Encoding chunks independently

**Pre-tokenization** divides text into chunks before BPE. These chunks set
boundaries that merges cannot cross. A chunk isn't necessarily a final token:
encoding it can produce several token IDs.

With `--pretokenize gpt2`, your `encode` command must:

1. Read `text.txt` as Unicode text and collect the pattern's matches in order,
   using the pattern below. Each whole match is a chunk, not a delimiter to
   discard. Joining the chunks must recover the exact input, including all
   whitespace.
2. Convert each chunk separately to its UTF-8 byte IDs.
3. Encode each chunk independently using the ranked BPE procedure from earlier
   stages and the same fixed `merges.json`. Keep the rules' ranks and IDs
   unchanged: the pair at index `i` still creates ID `256 + i`.
4. Concatenate the resulting ID arrays in chunk order. Do not run another merge
   pass on the combined array.

This option changes encoding only. Without it, `encode` must retain its earlier
behavior, and `train` remains unchanged. You're adding GPT-2's chunk boundaries,
not loading its vocabulary or producing its token IDs.

### Splitting with GPT-2's pattern

Use this exact pattern from
[`tiktoken`'s GPT-2 definition](https://github.com/openai/tiktoken/blob/main/tiktoken_ext/openai_public.py):

```text
'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s
```

Unicode classifies characters into categories. Here, `\p{L}` matches letters and
`\p{N}` matches numbers across writing systems, not just ASCII letters and
digits. `\s` matches Unicode whitespace, and `\S` matches non-whitespace.
`[^\s\p{L}\p{N}]` matches a character outside all three groups, such as
punctuation or an emoji.

At each position, the regex tries the alternatives separated by `|` from left to
right:

- The first alternative matches the lowercase contractions `'s`, `'d`, `'m`,
  `'t`, `'ll`, `'ve`, and `'re`.
- ` ?\p{L}++` matches a run of letters, optionally preceded by one literal
  space, U+0020. ` ?\p{N}++` does the same for a run of numbers.
- ` ?[^\s\p{L}\p{N}]++` matches a run of other non-whitespace characters, also
  with at most one leading U+0020 space.
- `\s++$` matches a trailing whitespace run at the end of the text.
- `\s+(?!\S)` matches a whitespace run only if another whitespace character or
  the end of the text follows it. The `(?!\S)` is a negative lookahead: it
  checks what follows without consuming it. Before non-whitespace, this branch
  leaves the last whitespace character for a later match.
- The final `\s` matches one remaining whitespace character.

`+` means one or more, taking as many as possible but allowing the regex to give
some back if the rest of that alternative fails. `++` is possessive repetition:
it takes the whole run without giving characters back. Keep the alternatives in
the supplied order and use case-sensitive matching.

For example, `Hello world's 123!!` splits into five chunks:

```json
["Hello", " world", "'s", " 123", "!!"]
```

With merges `[[108, 108], [111, 32]]`, those five chunks produce 18 token IDs,
shown in "Tests" below. `ll` inside `Hello` becomes 256. The `o` and the
following space are in different chunks, so the rule `[111, 32]` cannot join
them. They stay 111 and 32.

### Whitespace, case, and combining marks

Only the optional literal U+0020 space can attach to the following letter,
number, or other non-whitespace run. A tab or a non-breaking space (U+00A0)
cannot. The last three alternatives handle the remaining whitespace, so don't
trim or collapse runs.

These examples use JSON escapes for actual characters in the input, not literal
backslashes:

| Text                   | Chunks                               |
| ---------------------- | ------------------------------------ |
| `"foo  bar"`           | `["foo", " ", " bar"]`               |
| `"foo\tbar"`           | `["foo", "\t", "bar"]`               |
| `"foo  "`              | `["foo", "  "]`                      |
| `"foo\u00a0bar"`       | `["foo", "\u00a0", "bar"]`           |
| `"foo\u00a0\u00a0bar"` | `["foo", "\u00a0", "\u00a0", "bar"]` |

Contractions are case sensitive. `don't` splits into `["don", "'t"]`, but
`DON'T` splits into `["DON", "'", "T"]`. Together, `don't DON'T` splits into
`["don", "'t", " DON", "'", "T"]`. The apostrophe here is U+0027.

Some vowel signs and other marks are encoded separately from the letters they
modify. These are **combining marks**. Unicode classifies them separately from
letters, so `\p{L}` doesn't include them. In this pattern, they match the
other-non-whitespace branch. For example, `हिंदी` splits into
`["ह", "िं", "द", "ी"]`. The second chunk contains two marks, not a letter.

### Your progress

<details>
  <summary>11 → 13 tokens for the example sentence `to the top of the tree`</summary>

For the exact sentence `to the top of the tree`, with no trailing newline, keep
the same five rules from earlier stages:

```json
[
  [32, 116],
  [256, 104],
  [257, 101],
  [258, 256],
  [116, 111]
]
```

The pattern produces `["to", " the", " top", " of", " the", " tree"]`. Encoding
these chunks independently gives 13 IDs instead of 11:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens   without pre-tokenization
split     █████████████           13 tokens   ← this stage
GPT-2     ██████                   6 tokens   ← comparison
```

The rule `[258, 256]` joins `␠the` to the `␠t` of the following word, where `␠`
marks a space. It previously applied twice. Those pieces now lie in different
chunks, so both merges are forbidden and two more IDs remain. The text is
unchanged. The higher count reflects the new boundary rule, not a change in
merge ranks or IDs. GPT-2's six-token result is a comparison, not the result of
these five rules.

</details>

### Tests

The tester will write `Hello world's 123!!` to `text.txt`, with no trailing
newline, and `[[108, 108], [111, 32]]` to `merges.json`, then execute your
program like this:

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

The tester will also check `DON'T`, `हिंदी`, `foo\u00a0bar`, `don't DON'T`,
`foo\u00a0\u00a0bar`, and the sentence from "Your progress". Here, `\u00a0`
denotes an actual non-breaking space in the input.

### Notes

- Print only the final JSON array to stdout, and send debug output to stderr.

{{#lang_is_python}}

- Your project already includes the [`regex`](https://pypi.org/project/regex/)
  package, which supports `\p{L}` and `++`. Use `import regex` in place of the
  built-in `re`, and collect whole matches with `findall` or `finditer`.

{{/lang_is_python}}

{{#lang_is_typescript}}

- TypeScript uses JavaScript's `RegExp`, which doesn't support possessive
  quantifiers and has a different `\s` whitespace set. For this supplied
  pattern, replace each `++` with `+`, every `\s` with `\p{White_Space}`, and
  every `\S` with `\P{White_Space}`. The latter two match Unicode whitespace and
  its complement. Use `g` to collect all matches and `u` for Unicode property
  escapes. These substitutions are specific to this pattern, not a general way
  to remove possessive quantifiers.

{{/lang_is_typescript}}

{{#lang_is_java}}

- Compile the pattern with `Pattern.UNICODE_CHARACTER_CLASS`. Otherwise `\s`
  doesn't match a non-breaking space. Collect whole matches with
  `Matcher.find()`.

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
  [PCRE2](https://pcre2project.github.io/pcre2/doc/pcre2api/). Compile with
  `PCRE2_UTF | PCRE2_UCP`. For this pattern, replace every `\s` with
  `\p{White_Space}` and every `\S` with `\P{White_Space}` to match Unicode
  whitespace and its complement. PCRE2's `\s` also includes U+180E, which would
  create different chunks. Call `pcre2_match` in a loop, starting each search
  where the last chunk ended.

{{/lang_is_cpp}}
