# Security Policy

## Reporting a vulnerability

**Do not open a public issue.** Use GitHub's private vulnerability reporting:

**Repository → Security → Report a vulnerability** (the "Security" tab → "Report a vulnerability"
button). It opens a private advisory that only the Maintainer can see.

If that button is unavailable, open a GitHub Security Advisory draft directly at
<https://github.com/SYNSEMA/agent-payments/security/advisories/new> and mention only the
vulnerability — no exploit code in the title.

Please include: what you ran, what you expected, what happened, and the smallest bundle that
reproduces it. A bundle is enough on its own — it is self-contained and carries everything a
verifier needs.

There is no bug bounty. There is also no payment, ever, for a report. This project is not funded
yet; if it ever is, the bounty programme will be announced here.

## What gets a response

| | |
|---|---|
| Acknowledgement | within **3 days** |
| Triage and a verdict (valid / not valid / already known) | within **14 days** |
| A fix or a published mitigation plan | within **14 days** of the verdict |

These are promises from a single maintainer, not from a team with a security rota. If a deadline
is going to be missed, you will be told before it passes, not after.

## What is in scope

- The **verifiers**: `reference/ts/verify.mjs`, `reference/rust/`, and `src/verify.syn`. A verifier
  that accepts a bundle it should reject is the most serious bug this project can have.
- The **conformance checker**: `src/conformance.syn`. A constraint that silently passes is the
  same class of bug.
- The **specification**: `spec/SPEC.md`. A specification ambiguity that admits two
  implementations with different verdicts is a security bug here.
- The **test vectors** and the CI.
- The **recipe**: `web.syn` and the capability declarations it makes.

## What is *not* a vulnerability

These are properties of the design, not bugs. Reporting them is a waste of the maintainer's time
and of the reporter's:

- **A malicious issuer can sign a bundle whose `intent` is not what the user actually asked.**
  That is the point of the evidence layer: the verifier proves *that a signed intent existed and
  was approved*, and proves whether the purchase meets **that** intent. It cannot read a user's
  mind, and no format can. The attack this project exists to make **detectable** is the one where
  the agent buys something that does not meet the stated constraints — see
  [`THESIS.md`](THESIS.md) §2 and arXiv:2609.11757.
- **The issuer's key can be compromised.** The bundle binds to a `did:key`; if the issuer's key
  leaks, whoever holds it can issue bundles. Mitigation is key custody — which is the commercial
  service, not the format. Rotating the issuer key invalidates the bundles it signed, which is
  visible to a verifier.
- **A verifier run against a hostile bundle can be made to read a file.** The reference verifiers
  read one file and no others, and take no network. A crash on a malformed bundle is a bug; a
  file read outside the bundle is a bug; refusing to parse is not.
- **The reference implementation's development issuer key is public** (`0303…03`, in
  `web.syn`). It is a fixed seed so the demo is reproducible. It signs nothing that matters.

## The threat model in one paragraph

The bundle is produced by an issuer that the verifier does not trust. The verifier is given the
bundle and the issuer's public key, and nothing else — no network, no account, no shared secret.
It must answer one question: *does this bundle prove that a human asked for these constraints and
approved this specific purchase, and does the purchase meet the constraints?* An attacker who
controls the issuer may sign **false** bundles; what they may not do is produce one that verifies
as something it is not. Every check in `spec/SPEC.md` exists to make one specific forgery
detectable, and each one has a negative vector under `testvectors/invalid/` that proves it.

## Disclosure

A valid report is fixed in a private advisory, published together with the fix, and credited
unless you ask otherwise. If a fix cannot ship quickly, the mitigation is published first and the
advisory stays private until there is one.
