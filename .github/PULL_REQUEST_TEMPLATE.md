## What

<!-- One paragraph. What changes, and for whom. -->

## Why

<!-- The problem, with a real bundle or a real scenario if there is one. -->

## How to verify

<!-- The exact commands. If it changes the format, the vector that proves it. -->

```bash
# the commands a reviewer can run
```

## Checklist

- [ ] `synsema test src/proof.test.syn` passes
- [ ] `./demo.cmd` exits 0
- [ ] `node reference/ts/verify.mjs testvectors/valid` and `... testvectors/invalid` pass
- [ ] `cargo test --manifest-path reference/rust/Cargo.toml` passes
- [ ] the commit is signed off (`git commit -s`)
- [ ] if this changes the spec: a PSCP is in `spec/pscp/`, the spec is updated, **and** both
      reference verifiers and the vectors are updated in the same commit
- [ ] if this adds an attack: a vector under `testvectors/invalid/` and a test in
      `src/proof.test.syn` that both reject it
- [ ] no secret, key or token is in the diff
