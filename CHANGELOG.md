# Changelog

All notable changes to this project are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The **licence never changes** ([`GOVERNANCE.md`](GOVERNANCE.md) §1). Every version below is
Apache-2.0 with the patent grant in [`PATENTS.md`](PATENTS.md).

---

## [Unreleased]

Nothing yet.

---

## [0.1.0] - 2026-11-22

The first release. Everything below is new; there is no previous behaviour to preserve.

### Added

**The format**

- `spec/SPEC.md` — the normative specification of the proof bundle: the canonical JSON form, the
  intent and proposal models, the conformance check, the approval binding, the signature, and the
  nine checks a verifier performs. Plain text; readable and implementable without Synsema.
- `spec/bundle.schema.json` — the JSON Schema of a bundle.
- `spec/pscp/` — the directory where Portable Spec Change Proposals go, with the template.

**The reference verifiers**

- `reference/ts/verify.mjs` — a TypeScript verifier using **only the Node standard library**, no
  npm dependencies at all. It performs the nine checks offline and prints `VERIFIED` or
  `REJECTED` with the reason.
- `reference/rust/` — the same nine checks in Rust.
- `reference/implementations.md` — the registry of independent implementations.

**The test vectors**

- `testvectors/valid/` — bundles that must verify.
- `testvectors/invalid/` — one file per attack: a modified amount, a modified proposal, a
  recomputed verdict that does not match the constraints, a missing human assertion, an assertion
  bound to a different proposal, and more.

**The reference implementation in Synsema**

- `src/intent.syn` — a request becomes machine-checkable constraints; the fingerprint is SHA-256
  over the canonical JSON.
- `src/conformance.syn` — the semantic check: does the proposal meet the constraints?
- `src/proof.syn` — the signing, the W3C-style credential, the WebAuthn approval and the bindings.
- `src/verify.syn` — the offline verifier in Synsema.
- `src/proof.test.syn` — 31 tests, including the tampering cases and the golden file.
- `web.syn` — the console: request → proposal → approval → verdict, with the three panels.
- `demo.cmd` — the whole thing in one command.

**The governance**

- Apache-2.0 with an explicit patent grant (`PATENTS.md`).
- A DCO, not a CLA (`CONTRIBUTING.md`).
- `GOVERNANCE.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, `CITATION.cff`, `REUSE.toml`.
- CI that runs the unit tests, both reference verifiers against the vectors, the DCO check and
  the licence check on every push.

### Known limitations

- The bundle is **AEP-shaped but not AEP-complete**: it has RFC 8785 canonicalisation, Ed25519
  signatures and offline verification, but not the eight-layer chain, `previous_hash` inside the
  hash input, the seal at `authorization`, or the `DisputeBundle` export. **AEP-complete is not
  even achievable today**: AEP publishes conformance vectors (a), (b) and (h) only; vectors
  (c)–(m), including the signed dispute bundle, are not published, so no implementation can claim
  full AEP conformance against published vectors.
- It is **not Verifiable Intent compatible**: there is no SD-JWT, no `cnf`, no `sd_hash`, and the
  approval key is not ES256.
- There is no checkout, no payment processor and no settlement. The proof is produced *before*
  settlement and is verified independently of it.
- The demo scenario is a hotel booking. The travel scenario the thesis calls for (a flight, six
  constraints) is specified in `research/THESIS.md` §3 and not yet implemented.
- **This is not a competing standard.** AEP (A-Comm Technologies) is the evidence *chain*; this is
  the intent-and-conformance layer. They compose. See `spec/SPEC.md` §1.2.

[Unreleased]: https://github.com/SYNSEMA/agent-payments/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/SYNSEMA/agent-payments/releases/tag/v0.1.0
