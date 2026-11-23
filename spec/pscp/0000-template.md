# PSCP 0000 — template

Copy this file to `spec/pscp/NNNN-your-slug.md`, fill it in, and open a pull request.

A **Portable Spec Change Proposal** is how the normative specification changes. The process is in
[`../../GOVERNANCE.md`](../../GOVERNANCE.md) §4.

---

## NNNN — <title>

**Author:** <name>
**Status:** proposed
**Affects:** `spec/SPEC.md` §<sections> · `spec/bundle.schema.json` · `testvectors/`

### 1. The problem

What is wrong today, and **how you know**. A real bundle, a real scenario, a real attack — not a
hypothetical.

> Attach the bundle or the reproduction here.

### 2. The change

The proposed normative text, as a diff against `spec/SPEC.md`.

```diff
```

### 3. Conformance impact

What a verifier must now check, and in which order. If a check is added, say whether it replaces
one of the nine or is an addition.

| # | Name | What it establishes |
|---|---|---|
| | | |

**Does this change the number of checks?** If the nine become ten, say so plainly: the order is
normative.

### 4. Backwards compatibility

How a bundle produced under the previous version still verifies under the new one. If it does not,
say what the version bump is and how long both versions verify.

### 5. The vectors

**A proposal without vectors is not mergeable.** A specification without vectors is a document, not
a standard.

| File | Under | Expected |
|---|---|---|
| `testvectors/valid/…` | | VERIFIED |
| `testvectors/invalid/…` | | REJECTED, because … |

### 6. Implementations affected

Which existing implementations change, and what they have to do.

### 7. Review

| | |
|---|---|
| Reviewer | |
| Second reviewer (required for a change to a cryptographic construction or the canonicalisation) | |
