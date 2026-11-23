# agent-payments — the Rust reference verifier

A second independent implementation of [`spec/SPEC.md`](../../spec/SPEC.md), for the auditor: a
Rust crate with no platform dependency, no network and no account, that reaches the same verdict as
the TypeScript verifier and as the Synsema reference implementation.

## Why this one has dependencies

Rust's standard library has no cryptography. This crate uses crates.io:

| Crate | For |
|---|---|
| `ed25519-dalek` | the Ed25519 signature over the data-integrity message |
| `p256` + `ecdsa` | the P-256 (ES256) WebAuthn signature |
| `sha2` | SHA-256 |
| `serde_json` | JSON |
| `base64` | base64url |

That is a deliberate trade: **audited dependencies instead of a hand-rolled curve implementation.**
The TypeScript verifier has zero dependencies because Node ships the cryptography; Rust does not.

`Cargo.lock` is committed, so the build is reproducible.

## Use it

```bash
cargo test --manifest-path reference/rust/Cargo.toml
cargo run --manifest-path reference/rust/Cargo.toml --bin apv -- testvectors/valid
cargo run --manifest-path reference/rust/Cargo.toml --bin apv -- testvectors/invalid
```

Exit code: `0` = VERIFIED (or every expectation held), `1` = REJECTED, `2` = the file could not be
read.

## As a library

```rust
use agent_payments_verifier::{verify_bundle, DEFAULT_ISSUER};

let bundle: serde_json::Value = serde_json::from_str(&text)?;
let verdict = verify_bundle(&bundle, DEFAULT_ISSUER);
assert!(verdict.ok);
```

## What it does

The same nine checks, in the same order, with the same verdicts. **A divergence between this
verifier, the TypeScript one and the Synsema one on any vector is a release blocker** — that is
why all three run the same set in CI.

## Requirements

A stable Rust toolchain (1.75 or newer). Nothing else.

## Licence

Apache-2.0, with the patent grant in [`../../PATENTS.md`](../../PATENTS.md).
