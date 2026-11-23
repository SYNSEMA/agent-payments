# Independent implementations

The registry of implementations that are **not** maintained by the steward of this repository.

This file is the countable side of the neutrality trigger in [`../GOVERNANCE.md`](../GOVERNANCE.md)
§6: **three independent implementers** — people or organisations with no employment, funding or
contractual relationship with the Maintainer — shipping interoperable implementations that pass
the shared conformance vectors is what starts the process of donating `spec/` and `reference/` to a
neutral steward.

## How to get listed

An implementation is listed when:

1. It is **independent**: not maintained by the Maintainer, not funded by them, not contracted by
   them.
2. It **passes the conformance suite**: every vector in `testvectors/valid/` verifies and every
   vector in `testvectors/invalid/` is rejected.
3. You open a pull request adding the entry below, with the output of your conformance run.

The maintainers run the suite against your implementation before merging. "It works on my machine"
is not a conformance result; the vectors are the test.

## The implementations

| Implementation | Language | Author | Conformance | Notes |
|---|---|---|---|---|
| `reference/ts` | TypeScript | the Maintainer | passing | Zero dependencies. [Source](../ts/) |
| `reference/rust` | Rust | the Maintainer | passing | crates.io dependencies. [Source](../rust/) |
| `src/` | Synsema | the Maintainer | passing | The reference implementation. [Source](../../src/) |

*The three rows above are the Maintainer's own, and they do **not** count towards the trigger.*

## The conformance suite

```bash
node reference/ts/verify.mjs testvectors/valid
node reference/ts/verify.mjs testvectors/invalid
cargo test --manifest-path reference/rust/Cargo.toml
sh scripts/verify-vectors.sh
```

The vectors are derived from the golden file by `scripts/make-vectors.mjs`: each one is the golden
file with exactly one thing changed. They cannot rot, because they are regenerated.
