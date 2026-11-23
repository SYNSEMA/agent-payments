# Patent Grant

In addition to the Apache License 2.0 (see [`LICENSE`](LICENSE)), each contributor grants a
**perpetual, worldwide, non-exclusive, royalty-free, irrevocable** (except as stated below) patent
licence to make, have made, use, sell, offer for sale, import and otherwise transfer the Work,
where such licence applies **only to those patent claims licensable by such contributor that are
necessarily infringed by their Contribution(s) alone or by combination of their Contribution(s)
with the Work.

This grant applies to every path in this repository that carries the Apache-2.0 licence: the
specification (`spec/`), the reference implementations (`reference/`), the test vectors
(`testvectors/`) and the recipe (`web.syn`, `src/`).

## The narrowing, and where it lives

Apache-2.0 §3 grants a patent licence only over claims "necessarily infringed by their
Contribution(s)". For a **specification** that is a weak fit: a specification arguably
contributes no infringing claims at all, because it is not an implementation.

So the grant here is narrowed the way the A-Comm Evidence Protocol narrows it: **`NOTICE`**
declares a licence over **Necessary Claims** — the claims necessarily infringed by an
implementation of the format as specified in `spec/SPEC.md` — and this document carries the
termination. **The normative text of the narrowing is in `NOTICE`, not here**, so that the
`LICENSE` file stays byte-identical to the canonical Apache-2.0 text that OSI, SPDX and the
OpenSSF checks look for.

## Termination

If you or your agent or affiliate institute directly or indirectly, or take a direct financial
interest in, any **Patent Assertion** against the Work or any entity distributing the Work, then
any rights granted to you under the Apache License 2.0 for the Work **shall terminate** as of the
date such Patent Assertion is filed.

A "Patent Assertion" is any claim, or a threat to sue, alleging that the Work or anything it
contains infringes a patent, that the Work is necessary for practicing a patent, or that an
entity distributing the Work has made a patent licence available.

This termination does not apply to a patent assertion that is brought **solely** to assert that
the Work itself (or a combination of the Work with the Work of another contributor that is also
licensed here) infringes that patent.

## Why this document exists

Apache-2.0 §3 already grants a patent licence. It does not, however, give a patent holder a way
to stop a user of an open standard from enforcing a patent **against users of the standard**. For
a specification whose whole purpose is to be implemented by people we will never meet, that
distinction matters.

This document is the same shape as the grant used by the A-Comm Evidence Protocol and by other
specification projects. It is written to be compatible with Apache-2.0 §3 — it **adds** the
termination, it does not replace or contradict the base grant.

## Interaction with the Apache-2.0 §3 grant

Nothing in this document limits, narrows or expands the patent grant in §3 of the Apache License
2.0. Where the two differ, §3 governs, and this document only adds the termination above.

## Interaction with the licence commitment

`GOVERNANCE.md` §1 states that the licence of this repository never changes. This patent grant is
part of that commitment: it cannot be removed, weakened or replaced by a future amendment to
`GOVERNANCE.md` either.
