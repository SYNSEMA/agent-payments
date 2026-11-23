#!/usr/bin/env node
/**
 * agent-payments — generates the test vectors from the golden file.
 *
 *   node scripts/make-vectors.mjs
 *
 * The vectors are DERIVED, not hand-written: every one of them is the golden
 * file with exactly one thing changed. That is what makes them useful — a
 * negative vector proves one specific attack is detected, and it cannot rot
 * because it is regenerated from the golden file.
 *
 * testvectors/valid/    must verify
 * testvectors/invalid/  must be rejected, each for its own reason
 */

import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const GOLDEN = join(root, "src", "fixture-bundle.json");
const bundle = JSON.parse(readFileSync(GOLDEN, "utf8"));

const clone = () => JSON.parse(JSON.stringify(bundle));

const VALID = join(root, "testvectors", "valid");
const INVALID = join(root, "testvectors", "invalid");
mkdirSync(VALID, { recursive: true });
mkdirSync(INVALID, { recursive: true });

const write = (dir, name, obj) => {
  writeFileSync(join(dir, name), JSON.stringify(obj, null, 2) + "\n");
  console.log("  " + name);
};

// ── valid ────────────────────────────────────────────────────────────────────
console.log("valid:");
write(VALID, "conformant-hotel.json", clone());

// ── invalid: one attack each ─────────────────────────────────────────────────
console.log("invalid:");

// 1. the amount was changed after the human approved it
{
  const b = clone();
  b.proposal.amount = 590;
  write(INVALID, "amount-changed-after-approval.json", b);
}

// 2. the proposal now violates a constraint the human asked for, but the
//    signed verdict still says "conformant" — the lie a dishonest issuer tells
{
  const b = clone();
  b.proposal.refundable_days = 0;
  write(INVALID, "constraint-violated-verdict-lies.json", b);
}

// 3. the signed verdict disagrees with the constraints it was computed from:
//    the issuer signs "non_conformant" over a proposal that IS conformant, so
//    re-running the checks does not reproduce the signed verdict
{
  const b = clone();
  b.signed_statement.credentialSubject.verdict = "non_conformant";
  b.signed_statement.credentialSubject.conformance.verdict = "non_conformant";
  write(INVALID, "verdict-flipped.json", b);
}

// 4. the request was rewritten after the fact
{
  const b = clone();
  b.intent.raw = "book a hotel in NYC for 2 nights under $900 total, free cancellation until the day before arrival";
  write(INVALID, "intent-rewritten.json", b);
}

// 5. the human's passkey assertion was removed: an approval with no human in it
{
  const b = clone();
  delete b.human_assertion;
  write(INVALID, "human-assertion-removed.json", b);
}

// 6. the assertion is a different credential than the signed approval names
{
  const b = clone();
  b.human_assertion.id = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
  b.human_assertion.rawId = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
  write(INVALID, "assertion-bound-to-another-credential.json", b);
}

// 7. the assertion's userHandle was swapped for someone else's
{
  const b = clone();
  b.human_assertion.response.userHandle = "dXNlci0y";
  write(INVALID, "assertion-user-handle-swapped.json", b);
}

// 8. the signature was tampered with
{
  const b = clone();
  const pv = b.signed_statement.proof.proofValue;
  // flip the last base58 character
  const last = pv.slice(-1) === "z" ? "y" : "z";
  b.signed_statement.proof.proofValue = pv.slice(0, -1) + last;
  write(INVALID, "signature-tampered.json", b);
}

// 9. the proof claims a verification method that is not the issuer's: a
//    different did:key, built by flipping one byte of the issuer's raw key
{
  const b = clone();
  const issuerDid = b.signed_statement.issuer;
  const A = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
  const decode = (s) => { let n = 0n; for (const c of s) { const i = A.indexOf(c); n = n * 58n + BigInt(i); } let h = n.toString(16); if (h.length % 2) h = "0" + h; return Buffer.from(h, "hex"); };
  const encode = (buf) => { let n = 0n; for (const x of buf) n = n * 256n + BigInt(x); let s = ""; while (n > 0n) { s = A[Number(n % 58n)] + s; n /= 58n; } let z = 0; while (buf[z] === 0) z++; return "z" + "1".repeat(z) + s; };
  const raw = decode(issuerDid.slice("did:key:".length + 1));
  const other = Buffer.from(raw);
  other[31] ^= 0x01;
  const otherDid = "did:key:" + encode(other);
  b.signed_statement.proof.verificationMethod = otherDid + "#" + otherDid.slice("did:key:".length);
  write(INVALID, "verification-method-not-the-issuer.json", b);
}

// 10. the approval record was taken out of the signed subject
{
  const b = clone();
  delete b.signed_statement.credentialSubject.approval;
  write(INVALID, "approval-not-signed.json", b);
}

// 11. the conformance report inside the signature was doctored
{
  const b = clone();
  b.signed_statement.credentialSubject.conformance.checks[0].observed = 1;
  write(INVALID, "conformance-report-doctored.json", b);
}

// 12. the whole thing is not a bundle
{
  const b = clone();
  delete b.signed_statement;
  write(INVALID, "no-signed-statement.json", b);
}

// 13. the constraint the human asked for was removed from the signed request
{
  const b = clone();
  b.intent.constraints = b.intent.constraints.filter((c) => c.kind !== "attribute" || c.field !== "refundable_days");
  write(INVALID, "constraint-dropped-from-request.json", b);
}

console.log("\ndone. verify with:");
console.log("  node reference/ts/verify.mjs testvectors/valid");
console.log("  node reference/ts/verify.mjs testvectors/invalid");
