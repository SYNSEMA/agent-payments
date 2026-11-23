# Governance

**agent-payments** is a portable, offline-verifiable evidence format for purchases made by AI
agents: *what was requested, who authorised it, how much was authorised, what was bought, and
whether the purchase met the request* — provable by a third party with public keys only.

This document says who decides what, and what can never change.

---

## 1. The commitment that never changes

The specification, the reference verifiers, the test vectors and the conformance programme are
released under **Apache-2.0 with an explicit patent grant** ([`PATENTS.md`](PATENTS.md)).

> **The licence of this repository never changes.** Not to a proprietary licence, not to
> source-available, not to "open core with a commercial clause". A change to the licence of
> `spec/`, `reference/` or `testvectors/` is rejected by review, and CI fails the build.

The precedent we refuse to repeat: a project that changed its licence and split its community in
two. A standard that can be relicensed is not a standard.

**The thing you pay for is never the format.** See §2.

---

## 2. What is open and what is not

| Path | Licence | What it is |
|---|---|---|
| `spec/` | Apache-2.0 + patent grant | the **normative** specification. Plain text and JSON. Implementable without Synsema |
| `reference/` | Apache-2.0 + patent grant | reference verifiers (TypeScript and Rust). **Zero platform dependency** |
| `testvectors/` | Apache-2.0 + patent grant | the vectors every implementation must pass |
| `web.syn`, `src/` | Apache-2.0 + patent grant | the reference implementation in Synsema, and the recipe |
| the conformance programme | Apache-2.0 + patent grant | the badge, the vector runner, the implementation registry |
| **the hosted trust service** | **proprietary** | key custody, the evidence vault, attestation, compliance exports, SLA |

**The rule that decides every future argument:**

> If a competitor can reproduce the product by running our free code on their own infrastructure,
> the commercial layer is in the wrong place. **A log is not a moat. A control is.**

The commercial value is **trust** — custody, rotation, availability, and evidence an auditor or a
card network accepts. It is never the format, the verifier or the vectors. Those stay free, run
offline, and need no account.

---

## 3. Roles

| Role | Who | What they decide |
|---|---|---|
| **Maintainer** | Csibeings LLC, through the `synsema` organisation on GitHub | merges, releases, security response, the roadmap |
| **Spec steward** | the Maintainer, until §6 applies | normative changes to `spec/` |
| **Implementer** | anyone | their own implementation; a conformance badge when they pass the shared vectors |
| **Reviewer** | invited per §5 | review of normative changes |

The Maintainer is a single accountable party on purpose. A specification with a committee of one
is a specification; a specification with a committee of nobody is a document.

---

## 4. How the spec changes — PSCP

A **Portable Spec Change Proposal** is a file at `spec/pscp/NNNN-slug.md` containing:

1. **The problem**, with a real bundle or scenario that demonstrates it — not a hypothetical.
2. **The proposed normative text**, as a diff against `spec/SPEC.md`.
3. **The conformance impact**: exactly what a verifier must now check, and in which order.
4. **Backwards compatibility**: how a bundle produced under the previous version still verifies
   under the new one.
5. **The test vectors** that come with it, under `testvectors/`.

Rules:

- A PSCP that changes the *meaning* of an existing field needs one PSCP and a Maintainer merge.
- A PSCP that changes the *encoding* needs a new `version` string in the bundle and a **major**
  bump; both versions verify for at least one release.
- A PSCP that only clarifies prose needs review and nothing else.
- **No proposal is merged without the vector that proves it.** A specification without vectors is a
  document, not a standard.

---

## 5. Who reviews a normative change

- A PSCP gets **one named reviewer** who is not the author, drawn from the implementers listed in
  [`reference/implementations.md`](reference/implementations.md).
- If fewer than three independent implementers exist, the Maintainer reviews their own proposal
  and says so in the merge commit. This is stated plainly rather than faked.
- A PSCP that changes a cryptographic construction or the canonicalisation needs **two** reviewers
  and a written rationale.

---

## 6. Neutrality — the donation trigger

Today the `synsema` organisation is the steward. That is correct while there is one implementer.

> **Trigger.** When **three independent implementers** — people or organisations with no
> employment, funding or contractual relationship with the Maintainer — ship interoperable
> implementations that pass the shared conformance vectors, the Maintainer **proposes the donation**
> of `spec/` and `reference/` to a neutral steward: a foundation, a consortium, or a
> community-elected steward.

The recipe and the language stay in `synsema`. **The specification goes neutral.** The split is
deliberate: a foundation holds a standard, not a product line.

**The trigger is countable, not a feeling.** "Three independent implementers" means three distinct
entries in `reference/implementations.md`, each with a passing conformance run, each verifiably
independent of the Maintainer.

---

## 7. Security

Security reports are handled under [`SECURITY.md`](SECURITY.md). The Maintainer is the security
contact until a neutral steward exists. Reports are never handled in a public issue.

---

## 8. Changing this document

This document may be amended **only to add guarantees**: a stricter rule, an additional
commitment, a clearer process.

Removing a guarantee — the licence commitment in §1, the openness of the format in §2, the
donation trigger in §6 — is **not** an amendment. It is a reversal, and it would need the consent
of the people who relied on it. In practice it does not happen, because the licence does not allow
it: §1 is enforced by the licence itself, not by this file.

---

## 9. Code of conduct

[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) is the Contributor Covenant 2.1. The Maintainer is the
enforcement contact. See §10 of that document for how to reach them.

---

## 10. Where the money is not

The Maintainer may sell trust services built **on** this format. The Maintainer may not:

- charge for the format, the verifier or the vectors;
- require an account to verify a bundle;
- make the conformance vectors inaccessible;
- add a clause to the licence that restricts commercial use.

The first three would make the format a business for the few. The fourth would make it a
licence, not a standard.
