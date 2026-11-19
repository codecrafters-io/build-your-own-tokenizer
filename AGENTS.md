# Repository instructions

This repository holds the course content for the "Build your own Tokenizer"
challenge. `course-definition.yml` defines the stages, and
`stage_descriptions/*.md` holds the instructions learners read.

## Keep deliberate edge-case teaching

Several notes exist because a learner hit that exact problem, or because the
behavior is easy to get wrong and the tester exercises it. Copy reviews and
rewrites may reword a note, but must keep the observation it carries. Check the
behavior against `codecrafters-io/tokenizer-tester` before removing any of them.

- Stage 1, byte range: output values stay between 0 and 255, and a larger number
  means the program printed characters where bytes belong. Learner feedback
  added this diagnostic after someone hit it.
- Stage 6, replacement characters: one U+FFFD can cover several invalid bytes,
  and the tests only include cut-off characters and bytes that never appear in
  UTF-8. Java groups encoded surrogates differently, and a default `TextDecoder`
  drops a leading BOM.
- Stage 7, whitespace: one optional U+0020 space can join the following word
  chunk. Whitespace runs, including non-breaking spaces, can form their own
  chunks. A native JavaScript `\s` is not Unicode `White_Space`, and the bundled
  PCRE2 uses Unicode 15.0 tables.
- Stage 8, merge-file parsing: `# #` is a real merge, not a comment.

Removing a note needs evidence that the behavior changed, not only a preference
for shorter prose.
