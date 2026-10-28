In this stage, you'll replace one pair with a new token id, scanning from left to right.

### One merge

In earlier stages, you found the pair that shows up most often. A merge walks the ids from left to right and replaces each match with a new id. Once two ids are merged, they are not available for an overlapping match.

The ids `[97, 97, 97]`, merged on `97,97` into `256`, become:

```json
[256, 97]
```
Since a UTF-8 bytes contain values from 0-255, we are generating new IDs starting from 256. 

The first two values become the new id and the last one stays. `[97, 97, 97, 97]` becomes `[256, 256]`. 

### Tests

The tester runs:

```bash
$ ./your_program.sh merge ids.json --pair 97,97 --new 256
```

For the bytes of `aaabdaaabac`, `stdout` is:

```json
[256, 97, 98, 100, 256, 97, 98, 97, 99]
```

The tester checks that:

- `aaabdaaabac` becomes `[256, 97, 98, 100, 256, 97, 98, 97, 99]`
- `[97, 97, 97]` becomes `[256, 97]`, and `[97, 97, 97, 97]` becomes `[256, 256]`
- a missing pair leaves the list unchanged

### Notes

- `--pair` is two integers separated by a comma, with no spaces.
- The new id is whatever `--new` says. Later stages assign these ids from the merge order.
