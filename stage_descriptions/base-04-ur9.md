In this stage, you'll learn an ordered list of merges from a corpus.

### Training

In earlier stages, you counted pairs and applied one merge. Training repeats both until the requested number of merges is done.

Start from the UTF-8 bytes. Merge number `i`, counting from 0, creates token `256 + i`. After each merge, count pairs again on the new sequence. On a tie, the pair that appears earliest in that sequence wins.

For the corpus `aaabdaaabac` and `--merges 3`, the learned pairs are:

```json
[[97, 97], [256, 97], [257, 98]]
```

`aa` is the most common pair, so it becomes token 256. Next, `(256, 97)` and `(97, 98)` both occur twice, and `(256, 97)` appears first, so it becomes token 257. `(257, 98)` becomes token 258. The ids are then `[258, 100, 258, 97, 99]`.

### Tests

The tester runs:

```bash
$ ./your_program.sh train corpus.txt --merges 3
```

For a corpus whose text is `aaabdaaabac`, `stdout` is:

```json
[[97, 97], [256, 97], [257, 98]]
```

The tester checks that:

- `--merges 3` on `aaabdaaabac` prints `[[97, 97], [256, 97], [257, 98]]`
- hidden corpora ask for 1, 6, 10, 15, 20, and 50 merges
- each merge is only the pair, in the order it was learned

### Notes

- Print only the pairs, in the order they were learned (left to right). The new id is implied by the position.
