# Contributing

Thanks for looking at this. Two things first, because they decide whether your change can be
merged at all.

## 1. Sign off your commits (DCO)

Every commit needs a `Signed-off-by` line, and it must match the author:

```bash
git commit -s -m "your message"
```

That is the **Developer Certificate of Origin**, version 1.1. Read it before you sign:
<https://developercertificate.org/>. It is not a CLA, and it does not transfer copyright. It says
that you have the right to contribute the work, and that the project may distribute it under the
licence it already has.

If a commit is missing the sign-off, CI fails with the exact commit that lacks it, and adding the
line is a normal `git commit --amend -s --no-edit` once the branch is pushed. It is not a
permanent block.

**Why DCO and not a CLA:** a one-way CLA (the kind that says "you assign us your copyright, we
may relicense this") is rejected by the legal departments of most companies, and it makes a later
donation to a foundation a negotiation instead of a formality. The Linux kernel, Kubernetes, the
CNCF and OpenSSF all use the DCO.

## 2. The licence never changes

Apache-2.0, with the patent grant in [`PATENTS.md`](PATENTS.md). See
[`GOVERNANCE.md`](GOVERNANCE.md) §1. Do not propose a change to it; a change to it is not
proposed, it is rejected.

---

## Running it

You need the Synsema engine (**0.6.66** or newer) and, for the reference verifiers, Node 24 and
a stable Rust toolchain.

```bash
# the engine
curl -fsSL https://synsema.com/install.sh | sh     # or: npm i -g synsema

# the unit tests (31)
synsema test src/proof.test.syn

# the offline verifier: the honest bundle verifies
synsema run --cap-set "stdout,file.read" src/verify.syn -- proof-bundle.json

# the offline verifier: the tampered bundle is rejected (exit code 1 is the good outcome)
synsema run --cap-set "stdout,file.read" src/verify.syn -- tampered.json

# everything at once
./demo.cmd
```

The console:

```bash
synsema serve web.syn     # http://localhost:8477
```

The reference verifiers (no platform dependency, no account, no network):

```bash
node reference/ts/verify.mjs testvectors/valid      # every vector must verify
node reference/ts/verify.mjs testvectors/invalid      # every vector must be rejected
cargo test --manifest-path reference/rust/Cargo.toml
```

## The repository map

| Path | What it is |
|---|---|
| `spec/SPEC.md` | **the normative specification.** Plain text. Readable without Synsema |
| `spec/bundle.schema.json` | the JSON Schema of a bundle |
| `reference/ts/` | the TypeScript reference verifier (Node built-ins only, **no dependencies**) |
| `reference/rust/` | the Rust reference verifier |
| `reference/implementations.md` | the registry of independent implementations (§6 of the governance) |
| `testvectors/valid/` | bundles that **must** verify |
| `testvectors/invalid/` | bundles that **must** be rejected, one file per attack |
| `src/` | the reference implementation in Synsema (`intent`, `conformance`, `proof`, `verify`) |
| `web.syn` | the console: the entry of the recipe |
| `web/` | the console's static UI |
| `research/` | the market research behind the thesis, with sources and dates |

## What makes a good pull request

- **A bug fix** comes with a test that fails before it and passes after.
- **A spec change** comes as a PSCP (`spec/pscp/NNNN-slug.md`) with the vectors that prove it.
  See `GOVERNANCE.md` §4.
- **A new attack surface** comes with a vector under `testvectors/invalid/` that the verifiers
  reject, and with the line in `src/proof.test.syn` that proves the reference implementation
  rejects it too.
- **A conformance change** updates `spec/SPEC.md` **and** the vectors **and** both reference
  verifiers in the same commit. A verifier that disagrees with the spec is a bug, not a variant.

## Rules for the reference verifiers

They are the portability proof of this project, so they have rules the rest of the code does not:

1. **No dependency on Synsema, on this repository's code, or on a service.** A verifier must run
   in a clean checkout with the language's standard library and nothing else.
2. **No network.** A verifier that calls out to check a signature is not offline.
3. **The TypeScript verifier uses the Node standard library only.** No npm dependencies at all.
   This is deliberate: `npm install` is a supply-chain surface, and a standard that needs one
   cannot be audited by a small team.
4. **Both verifiers must agree.** If the TypeScript one says VERIFIED and the Rust one says
   REJECTED on the same file, that is a release blocker.
5. **A new check in the spec is added to both verifiers in the same PR**, with its vector.

## Commit style

Plain imperative subjects, under 72 characters:

```
add the baggage constraint to the conformance checker
reject a bundle whose approval binds a different proposal
```

Reference issues with `#123` and pull requests with `!456`.

## Reporting a security problem

Not in a public issue. See [`SECURITY.md`](SECURITY.md).

## Code of conduct

[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
