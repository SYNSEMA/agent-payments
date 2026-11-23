# agent-payments — the TypeScript reference verifier

An independent implementation of [`spec/SPEC.md`](../../spec/SPEC.md). It is the portability proof
of this project: a different language, a different codebase, the **same verdict**.

## Zero dependencies

**This package has no dependencies at all** — not even devDependencies. It uses the Node standard
library only: `node:crypto` for SHA-256 and the signature verifications, `node:fs` for the files.

That is deliberate. `npm install` is a supply-chain surface, and a standard that needs one cannot be
audited by a small team. Copy `verify.mjs` into a project and it works.

## Use it

```bash
node verify.mjs proof-bundle.json          # verify one bundle
node verify.mjs ../../testvectors/valid     # every file must verify
node verify.mjs ../../testvectors/invalid   # every file must be rejected
```

Exit code: `0` = VERIFIED (or every expectation held), `1` = REJECTED, `2` = the file could not be
read.

## As a library

```js
import { verifyBundle, verifyStatement, checkAll, jcs, fingerprint } from "@agent-payments/verifier";

const verdict = verifyBundle(bundle, "did:key:z6Mk...");
if (!verdict.ok) {
  for (const f of verdict.failures) console.error(f.name, "—", f.detail);
}
```

`verifyBundle` returns `{ok, summary, checks, failures}` — `checks` has the nine checks in the
normative order, each with its own `detail` explaining what it established.

## What it does

Nine checks, in this order: `signature`, `intent_integrity`, `proposal_integrity`,
`verdict_recomputed`, `conformance`, `approval_signed`, `human_approval`, `assertion_binding`,
`assertion_shape`.

It **fails closed**: a malformed bundle is `REJECTED`, never an exception. A verifier that raises
on a tampered artefact answers nothing, and *nothing is not a verdict*.

## Requirements

Node 20 or newer. Nothing else. No network, no account, no keys.

## Licence

Apache-2.0, with the patent grant in [`../../PATENTS.md`](../../PATENTS.md).
