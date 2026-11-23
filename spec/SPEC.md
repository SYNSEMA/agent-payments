# agent-payments — the proof bundle, specified

**Status:** normative. **Version:** `intent-proof/proof/1` · **Licence:** Apache-2.0 with the
patent grant in [`PATENTS.md`](../PATENTS.md).

This document specifies a **proof bundle**: a signed, self-contained artefact that lets a third
party answer, offline and without trusting the issuer:

1. **What did the user ask for?**
2. **What did the agent propose?**
3. **Who authorised it, and when?**
4. **How much was authorised to be spent?**
5. **What was bought?**
6. **Does the purchase actually meet the request?**

A bundle is produced by an **issuer**. It is verified by a **verifier** that holds nothing but the
bundle and the issuer's public key. The verifier has no network, no account and no secret.

---

## 1. Scope, and what this is not

**In scope.** The format of the bundle, the algorithm that produces the signature, the algorithm
that recomputes the verdict, and the nine checks a verifier performs.

**Out of scope.** How the issuer gets the user's request; how a payment is made; how a dispute is
filed; what a merchant does with the bundle. This specification says nothing about whether a
purchase is a good idea, and nothing about who is liable if it is not.

### 1.1 The threat model, in one paragraph

The issuer is **not trusted**. It may be compromised, malicious, or merely buggy, and it may sign
anything. The verifier's job is to detect every forgery that a dishonest issuer could produce.
What this format **cannot** do is know what the user really wanted: an issuer can sign a bundle
whose `intent` is not what the user asked. What it *can* do is make that visible, because the
intent is inside the signed subject and can be shown to the user, to a merchant, or to an auditor.
The attack this format exists to make **detectable** is the one where an agent buys something that
does not meet the request it was given — an attack that leaves no structural trace in the payment
records, and that a signature alone does not catch ([arXiv:2609.11757](https://arxiv.org/abs/2609.11757)).

### 1.2 Relationship to other specifications

This format is **not** a replacement for any of these, and does not claim compatibility with them:

| Specification | What it does | What this adds |
|---|---|---|
| **AP2** (Google → FIDO Alliance) | Mandates and credentials for agent-initiated payment | AP2 constrains the *authorisation*. It does not record whether the purchase met the request |
| **Verifiable Intent** (Mastercard + Google) | Proves the agent stayed inside its authority | That is a different layer: *was the agent authorised to do this at all*, not *is this the thing that was asked for* |
| **A-Comm Evidence Protocol** (AEP) | A chain of evidence artefacts for a dispute | The shape of this bundle follows AEP's ideas (RFC 8785 canonicalisation, SHA-256, Ed25519, offline verification). It is **not yet AEP-complete**: it has no eight-layer chain, no `previous_hash`, no seal at `authorization`, and no `DisputeBundle` export. See [`../CHANGELOG.md`](../CHANGELOG.md) § Known limitations |
| **W3C Verifiable Credentials** | The credential shape | The bundle uses a VC-shaped `signed_statement` |

A verifier of this format is a **verifier of the intent**, not of the mandate. The two compose.

---

## 2. Terminology

| Term | Meaning |
|---|---|
| **issuer** | The party that produces the bundle. It holds the signing key. It is not trusted by the verifier |
| **verifier** | The party that checks a bundle. It holds only public material |
| **request** (intent) | What the user asked for, normalised into machine-checkable **constraints** |
| **proposal** | The concrete purchase the agent proposed: a merchant, an amount, and attributes |
| **conformance** | Whether the proposal satisfies every constraint of the request |
| **approval** | The record of a human authorising one exact proposal, made with a passkey |
| **bundle** | The JSON artefact that carries all of the above, signed |

---

## 3. The bundle

A bundle is a JSON object with exactly four members.

```jsonc
{
  "intent":            { ... },   // §4   the request, as constraints
  "proposal":          { ... },   // §5   the concrete purchase
  "signed_statement":  { ... },   // §6   the VC-shaped document and its proof
  "human_assertion":   { ... }    // §7   the WebAuthn assertion (mandatory)
}
```

The JSON Schema is [`bundle.schema.json`](bundle.schema.json).

**There is deliberately no unsigned copy of the approval or of the conformance report outside the
signed subject.** A field that is not covered by the signature is a field nobody can defend: if
`approved_by` sat beside the signed statement, anyone could rewrite it and the verifier would not
notice.

---

## 4. The request (intent)

```jsonc
{
  "version": "intent-proof/intent/1",
  "raw": "the user's literal words",
  "constraints": [ ... ]
}
```

`raw` is the legal record of what was asked and is **never interpreted by the verifier**. The
machine-checkable form is `constraints`.

### 4.1 A constraint

```jsonc
{
  "kind":     "budget" | "category" | "attribute" | "quantity",
  "field":    "the field of the proposal it constrains",
  "op":       "<=" | "==" | ">=" | "contains" | "in",
  "value":    <the bound>,
  "currency": "USD",            // optional, only for a budget constraint
  "label":    "a human sentence, shown to the user at approval time"
}
```

**`kind` is informational.** The verifier evaluates `field`, `op` and `value`. `kind` exists so a
human reading the bundle can tell a budget from a category, and so a future version of the format
can add semantics per kind without breaking old bundles.

**Ordering rule.** `>=` and `<=` are meaningful **only on numbers**. A string comparison is
lexicographic, so `"none" >= "day_before_checkin"` is true and a non-refundable rate sails through
as valid. Anything ordinal — days of notice, number of nights, number of bags — **must** be
modelled as a number.

---

## 5. The proposal

Free-form by design: the format does not constrain what a purchase looks like. It must have at
least a `summary` and an `amount`. The fields the constraints refer to must be present on the
proposal, or the constraint **fails** (§8).

```jsonc
{
  "merchant":   "The Standard, NYC",
  "merchant_id": "standard-nyc",
  "summary":    "The Standard NYC — 2 nights, $540, free cancellation until 24h before arrival",
  "category":   "hotel",
  "amount":     540,
  "currency":   "USD",
  "nights":     2,
  "refundable_days": 1
}
```

---

## 6. The signed statement

```jsonc
{
  "@context": ["https://www.w3.org/2018/credentials/v1"],
  "type": ["VerifiableCredential", "IntentProof"],
  "issuer": "did:key:z6Mk...",
  "credentialSubject": {
    "version": "intent-proof/proof/1",
    "intent_fingerprint":    "sha256:<64 hex>",
    "proposal_fingerprint":  "sha256:<64 hex>",
    "verdict": "conformant" | "non_conformant",
    "approval":   { ... },   // §7.1  the WHOLE approval record
    "conformance": { ... }   // §8      the WHOLE conformance report
  },
  "proof": { ... }            // §6.3
}
```

### 6.1 Fingerprints

```
intent_fingerprint   = "sha256:" + hex(SHA-256(JCS(intent)))
proposal_fingerprint = "sha256:" + hex(SHA-256(JCS(proposal)))
```

Two objects with the same fingerprint are byte-identical under canonicalisation. A fingerprint is a
**commitment**, not a summary: it cannot collide by accident.

### 6.2 Canonicalisation — RFC 8785 (JCS)

The canonical form of a JSON value is:

1. Object members sorted by **UTF-16 code unit** order of their names.
2. No whitespace anywhere.
3. Strings escaped as JSON escapes; non-ASCII characters are **not** escaped (they are UTF-8 in the
   output).
4. Numbers in ECMAScript `Number::toString` form; `-0` canonicalises to `0`.
5. `null`, `true`, `false` as themselves.

**A number that cannot be represented exactly** (an integer beyond 2^53) **must not** appear in a
signed document: put it in the document as text. A canonical form that rounds is not canonical.

### 6.3 The proof

```jsonc
{
  "@context": ["https://www.w3.org/2018/credentials/v1"],   // equals the document's @context
  "type": "DataIntegrityProof",
  "cryptosuite": "eddsa-jcs-2022",
  "verificationMethod": "did:key:z6Mk...#z6Mk...",
  "proofPurpose": "assertionMethod",
  "proofValue": "z<base58btc>"
}
```

**The signed message is not the document.** This is the part that trips people up, so it is
stated precisely:

```
proofConfig = the proof map with `proofValue` removed
              (it keeps @context, type, cryptosuite, verificationMethod, proofPurpose,
               and `created` / `challenge` / `domain` if they are present)

message     = SHA-256(JCS(proofConfig))  ‖  SHA-256(JCS(document without `proof`))

proofValue  = "z" + base58btc( Ed25519-Sign(issuerKey, message) )
```

The `‖` is a **concatenation of the two 32-byte digests**: the signed message is 64 bytes long.
It is **not** the canonical document, and it is **not** a hash of it.

**A verifier MUST:**

1. Remove `proof` from the document, giving `document without proof`.
2. Take the proof map, remove `proofValue`, giving `proofConfig`.
3. If both the proof and the document carry an `@context`, **require** that they are canonically
   equal; otherwise **reject**.
4. Require `proof.type == "DataIntegrityProof"` and `proof.cryptosuite == "eddsa-jcs-2022"`;
   otherwise **reject**.
5. Require `proof.verificationMethod` to be `<issuer did:key>#<its multibase key id>`, where the
   issuer is the did:key the verifier was given; otherwise **reject**.
6. Compute `message` as above and verify the Ed25519 signature against the issuer's public key;
   otherwise **reject**.

> **Note on the key id.** The multibase key id of a `did:key` is the multibase encoding of the
> same bytes as the did's key, so `<did>#<multibase>` repeats the did's suffix. A verifier that
> checks only that the verification method *starts with* the issuer's did accepts a fragment that
> names a different key id. Check the whole fragment.

### 6.4 The issuer's key

`did:key` with the `ed25519-pub` multicodec (`0xed 0x01`) and a 32-byte raw key, base58btc-encoded
with the `z` multibase prefix. The verifier resolves it **offline**, with no DID document and no
network.

---

## 7. The human approval

### 7.1 The approval record (inside the signed subject)

```jsonc
{
  "approved_at": 1795238561.972181,
  "approved_by": "user-1",
  "credential_id": "base64url of the credential id",
  "issuer": "did:key:z6Mk...",
  "rp_id": "localhost",
  "origin": "http://localhost:8477",
  "challenge": "base64url of the challenge the server issued",
  "credential_public_key": { "kty": "EC", "crv": "P-256", "alg": "ES256", "x": "...", "y": "..." },
  "sign_count": 1,
  "user_handle": "base64url"
}
```

### 7.2 The assertion (outside, but bound by the signature's contents)

```jsonc
{
  "id": "base64url",           // the credential id
  "rawId": "base64url",        // MUST equal `id`
  "type": "public-key",
  "response": {
    "clientDataJSON": "base64url",
    "authenticatorData": "base64url",
    "signature": "base64url",  // DER-encoded ECDSA over P-256
    "userHandle": "base64url"  // MUST equal the approval's user_handle
  }
}
```

**The human assertion is mandatory.** A bundle without one is **rejected**. An approval with no
human's passkey behind it is not a proof of anything.

### 7.3 Verifying the assertion

A verifier MUST check all of the following, and MUST reject if any fails:

1. `clientDataJSON` parses as JSON, and `type == "webauthn.get"`.
2. `clientDataJSON.challenge` **equals** `approval.challenge`.
3. `clientDataJSON.origin` **equals** `approval.origin`.
4. `authenticatorData` is at least 37 bytes, and its first 32 bytes **equal** `SHA-256(rp_id)`.
5. The **User Present** flag (bit 0 of byte 32) is set.
6. The signature counter in `authenticatorData` (bytes 33..37, big-endian) is **greater than**
   `approval.sign_count`, unless `approval.sign_count` is 0. A counter that does not advance is a
   cloned authenticator.
7. The signature verifies over `authenticatorData ‖ SHA-256(clientDataJSON)` under the P-256 key in
   `approval.credential_public_key`.
8. `assertion.type == "public-key"`, `assertion.rawId == assertion.id`, and
   `assertion.response.userHandle == approval.user_handle`.

> **Why 6 and 8 matter.** WebAuthn's signature does **not** cover `type`, `rawId` or `userHandle`
> — the specification leaves them out on purpose, because they carry no authority. They are still
> fields in the bundle, so they still have to agree with what the signed approval says. Without
> check 8, the claim "every field in the bundle is authenticated" is not literally true.

---

## 8. The conformance check

The verdict is computed from the request and the proposal **alone**. It is never read from the
bundle.

For each constraint, in order:

1. If `field`, `op` or `value` is missing → the constraint **fails** ("malformed").
2. If the proposal does not carry `field` → the constraint **fails** ("the proposal does not state
   a …").
3. Otherwise evaluate:

| `op` | holds when |
|---|---|
| `<=` | `observed <= value` |
| `==` | `observed == value` (same type; no coercion) |
| `>=` | `observed >= value` |
| `contains` | `observed` contains `value` as a substring |
| `in` | `observed` is one of the members of `value` |

4. If the comparison is not possible (the types do not line up) → the constraint **fails**.

**The verdict is `conformant` only if every constraint holds.** There is no partial credit and no
"close enough": an agent that books the wrong rate is non-conformant, period.

The report (`verdict`, `checks`, `passed`, `total`, `violations`) is **inside the signed subject**,
so the issuer cannot change it after the fact — and a verifier **recomputes** it and compares.

---

## 9. The nine checks

A verifier performs these nine checks. The bundle is **verified** if and only if **all nine** hold.
The order is normative: the checks are reported in this order.

| # | Name | What it establishes |
|---|---|---|
| 1 | `signature` | The proof verifies against the issuer's public key, and its `verificationMethod` names that issuer (§6.3) |
| 2 | `intent_integrity` | `SHA-256(JCS(intent))` equals the signed `intent_fingerprint` |
| 3 | `proposal_integrity` | `SHA-256(JCS(proposal))` equals the signed `proposal_fingerprint` |
| 4 | `verdict_recomputed` | Re-running §8 yields exactly the signed `verdict` |
| 5 | `conformance` | The **recomputed** verdict is `conformant` — the proposal really does meet the request |
| 6 | `approval_signed` | The signed subject carries an approval record |
| 7 | `human_approval` | The passkey assertion verifies against the **signed** credential (§7.3) |
| 8 | `assertion_binding` | The assertion is the credential the signed approval names |
| 9 | `assertion_shape` | The assertion's `type`, `rawId` and `userHandle` agree with the signed approval |

**Check 4 uses the recomputed verdict, and check 5 uses the recomputed verdict.** Never the signed
one. A signed verdict is the signer's **claim**; a signer who lies would sign `conformant` over a
proposal that is not. Only recomputing from the artefacts tells the truth.

**Check 5 is the one that matters.** Checks 1–4 prove that a bundle says what it says. Check 5
proves that what it says is true.

### 9.1 Failing closed

**Every check fails closed.** A malformed bundle produces the verdict **REJECTED**, never an error,
never an exception, never a partial success. A verifier that raises on a tampered artefact answers
nothing — and *nothing is not a verdict*.

Concretely: a missing `credentialSubject`, a missing approval, a corrupted public key, a
constraint whose `value` is a list where a number was expected — all of these are **REJECTED**.

---

## 10. Versioning

The bundle's `intent.version` and `credentialSubject.version` carry the format version. A change
that alters the **meaning** of a field requires a **major** version; a change that only clarifies
prose requires none. Both versions MUST verify for at least one release.

---

## 11. Conformance

An implementation is **conformant** when it:

1. Passes every vector in `testvectors/valid/` (each must verify).
2. Rejects every vector in `testvectors/invalid/` (each must be rejected, for the reason the
   vector names).
3. Reaches the **same verdict** as the reference implementation on every vector.

The vectors are **derived** from the golden file by [`../scripts/make-vectors.mjs`](../scripts/make-vectors.mjs):
each one is the golden file with exactly one thing changed. They cannot rot, because they are
regenerated.

A conformant implementation must not require the issuer's private key, a network connection, an
account, or any particular programming language.

---

## 12. Security considerations

- **The issuer can lie about the request.** It can sign an `intent` the user never expressed. The
  format cannot know otherwise. What it does is make the lie *visible*: the intent is inside the
  signed subject, in the user's own words, and the verifier re-derives everything from it.
- **A compromised issuer key** can issue bundles that verify. Rotation is the mitigation: the
  bundles signed by a retired key are visibly signed by a key the verifier was not told to trust.
- **A verifier must not trust the issuer.** It must not fetch anything, and it must not accept a
  verdict it did not recompute. Checks 4 and 5 exist for that reason.
- **The bundle is not confidential.** It contains what was bought, for how much, and by whom. It
  is evidence, so it is public by nature — but a deployment should be aware of it.
- **Replay.** A bundle is a statement about one purchase, not a capability. It authorises nothing.
  A verifier that treats it as a token is misusing it.

---

## 13. References

| | |
|---|---|
| RFC 8785 | JSON Canonicalization Scheme (JCS) |
| W3C | Verifiable Credentials Data Model 2.0; VC Data Integrity, EdDSA Cryptosuites v1.0 (2025-05-15) |
| W3C | Web Authentication Level 3 |
| FIDO | AP2 — Agentic Payment Protocol |
| Mastercard + Google | Verifiable Intent |
| — | A-Comm Evidence Protocol (AEP) |
| arXiv:2609.11757 | *Signing the Transaction but Not the Decision* |
