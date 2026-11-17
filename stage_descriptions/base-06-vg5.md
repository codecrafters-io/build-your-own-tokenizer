In this stage, you'll implement the `decode` command to turn token IDs back into
text.

### Your progress

<details>
  <summary>Back to the original text for the example sentence `to the top of the tree`</summary>

For the exact sentence `to the top of the tree`, with no trailing newline,
decoding the 11 IDs from earlier stages with the same five merges recovers all
22 bytes and the original text:

```text
bytes     ██████████████████████  22 tokens
encode    ███████████             11 tokens
decode    ██████████████████████  22 bytes, the original text   ← this stage
GPT-2     ██████                   6 tokens   ← comparison
```

The `decode` command prints the text, not a new list of byte IDs. GPT-2's
six-token result is a comparison, not the result of these five merges.

</details>

### Why decode?

Encoding groups bytes into tokens. Decoding recovers text from those token IDs,
but a token can hold only part of a character. You must recover the bytes of
every token and join them before turning any of them into text.

For example, 👋 is four UTF-8 bytes, `[240, 159, 145, 139]`. With merges
`[[240, 159], [145, 139]]`, ID 256 holds `[240, 159]` and ID 257 holds
`[145, 139]`. Together, IDs `[256, 257]` represent 👋. Neither token holds a
complete character. Decoding them separately and then joining the text produces
`���`, because the decoder replaces incomplete or invalid byte sequences with
`�`. Joining their bytes first produces the full four-byte encoding of 👋.

### The `decode` command

The `decode` command parses two JSON files in the formats from earlier stages:
`ids.json` is an array of token IDs, and `merges.json` is an array of learned
pairs. To recover bytes, you need a mapping from each token ID to the byte
sequence it represents. This mapping is the tokenizer's **vocabulary**.

1. Start the vocabulary with IDs 0 through 255. ID `n` represents the single
   byte `n`.
2. Read the merges in list order. The pair at zero-based index `i` defines ID
   `256 + i`. Its bytes are the first ID's byte sequence followed by the second
   ID's byte sequence. A pair can include earlier merged IDs, whose bytes you
   have already reconstructed.
3. Look up each ID in `ids.json` and join its bytes into one buffer, in input
   order. Join across every token boundary, including between merged tokens.
4. Decode the whole buffer as UTF-8, using the replacement character `�`
   (U+FFFD) for malformed byte sequences, as described below. Print the
   resulting text as one JSON string.

For example, `[258, 100]` with the three merges from `aaabdaaabac` prints:

```json
"aaabd"
```

`258` expands to `257` and `98`, then to `256`, `97`, and `98`, and finally to
the bytes of `aaab`. `100` is `d`.

With no merges, `[240, 159, 145, 139]` also prints the wave emoji as the JSON
string `"👋"`.

### How many replacement characters?

U+FFFD is the Unicode identifier for the replacement _character_ `�`, not a
replacement byte. One `�` can stand for several bytes, but it must not swallow
the bytes of a valid character that follows.

UTF-8 gives each character a valid byte pattern. In a multi-byte encoding, the
first byte is the **leading byte** and indicates the expected number of bytes.
The remaining bytes are **continuation bytes**. Read the joined buffer from left
to right using these rules:

- A byte in 0–127 is a complete one-byte encoding. Leading bytes in 194–223
  expect two bytes in total, 224–239 expect three, and 240–244 expect four.
  Expected length alone does not make a sequence valid.
- Continuation bytes must be in 128–191. Four leading bytes further restrict the
  second byte, as listed in "The exact rule" below. A complete sequence that
  obeys these ranges decodes to its character.
- If the buffer ends during an otherwise valid prefix, replace that whole prefix
  with one `�`. If the next byte is not allowed at its position, replace the
  valid prefix so far with one `�` and reconsider that next byte as the start of
  a new sequence.
- A byte that cannot start a valid encoding, such as 255 or a stray continuation
  byte, is one `�` by itself.

| Bytes        | Prints           | Why                                                                                                                                                  |
| ------------ | ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `[240, 159]` | `"\uFFFD"`       | 240 expects four bytes and 159 is an allowed second byte. This valid prefix is cut off, so both bytes become one `�`.                                |
| `[255, 255]` | `"\uFFFD\uFFFD"` | 255 cannot start a valid encoding, so each 255 is its own `�`.                                                                                       |
| `[240, 240]` | `"\uFFFD\uFFFD"` | The second 240 cannot continue the first, so the first is one `�`. Reconsidering the second 240 starts another truncated prefix, giving another `�`. |

<details>
  <summary>The exact rule</summary>

The malformed inputs tested here are `[240, 159]` and `[255, 255]`. For the
general replacement rule, the second-byte restrictions also matter. UTF-8 does
not allow every combination of leading and continuation bytes as a character
encoding. These four leading bytes have narrower second-byte ranges:

| Leading byte | Expected bytes in total | Allowed second byte |
| ------------ | ----------------------- | ------------------- |
| 224          | 3                       | 160–191             |
| 237          | 3                       | 128–159             |
| 240          | 4                       | 144–191             |
| 244          | 4                       | 128–143             |

A second byte outside its allowed range does not extend a valid prefix. For
example, in `[240, 128]`, 128 is a continuation byte but is not allowed
after 240. Replace 240 with one `�`, then reconsider 128. It cannot start a
valid encoding, so it becomes another `�`.

Replacing the longest valid prefix, or a single invalid starter, is called
[substitution of maximal subparts](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G66453)
in the Unicode Standard. This is the replacement behavior required here, not one
replacement per invalid byte or one per run of invalid bytes.

</details>

### Tests

The tester will write `[258, 100]` to `ids.json` and
`[[97, 97], [256, 97], [257, 98]]` to `merges.json`, then execute your program
like this:

```bash
$ ./your_program.sh decode ids.json --merges merges.json
```

Your program must print the text as a JSON string:

```json
"aaabd"
```

The tester will also check the full wave emoji with no merges, the truncated
wave bytes `[240, 159]`, invalid bytes `[255, 255]`, quotes, a backslash and a
newline that need JSON escaping, the emoji split across merged tokens, and the
IDs of the sentence from "Your progress".

### Notes

- Join every token's bytes before decoding. The truncated wave bytes produce one
  replacement character. Keep running when the bytes are not valid UTF-8.

{{#lang_is_python}}

- `bytes.decode("utf-8", errors="replace")` replaces invalid bytes this way.

{{/lang_is_python}}

{{#lang_is_typescript}}

- `new TextDecoder("utf-8", { ignoreBOM: true }).decode(bytes)` decodes with
  replacement. `ignoreBOM: true` preserves an initial byte-order mark in the
  text instead of dropping it.

{{/lang_is_typescript}}

{{#lang_is_java}}

- `new String(bytes, StandardCharsets.UTF_8)` gives the required replacements
  for the malformed inputs tested here. Its grouping can differ for other
  malformed sequences. To match the general rule on those too, follow the
  byte-pattern rules above.

{{/lang_is_java}}

{{#lang_is_rust}}

- `String::from_utf8_lossy` replaces invalid bytes this way.

{{/lang_is_rust}}

{{#lang_is_go}}

- Write the decoding loop yourself, following the rule in "How many replacement
  characters?". Ranging over a string gives one U+FFFD per invalid byte, and
  [`strings.ToValidUTF8`](https://pkg.go.dev/strings#ToValidUTF8) gives one per
  run of invalid bytes, so neither matches. Once a character's bytes are
  complete, [`utf8.DecodeRune`](https://pkg.go.dev/unicode/utf8#DecodeRune)
  decodes them.

{{/lang_is_go}}

{{#lang_is_cpp}}

- Write the decoding loop yourself, following the rule in "How many replacement
  characters?". Valid encodings can be copied to the output as their bytes, and
  each `�` is the 3 bytes 239, 191, 189.

{{/lang_is_cpp}}

- Print only the resulting JSON string to stdout, using your JSON library so
  quotes, backslashes, and newlines are escaped correctly. Send debug output to
  stderr. `"\uFFFD"` and `"�"` are the same JSON string, so either passes.
