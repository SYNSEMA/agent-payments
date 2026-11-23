#!/usr/bin/env node
/**
 * agent-payments — reference verifier (TypeScript / Node).
 *
 * ZERO dependencies: Node's standard library only. No npm packages, no network,
 * no account, no Synsema. That is the portability proof of this project: an
 * independent implementation, in a different language, that reaches the same
 * verdict on the same bundle.
 *
 *   node reference/ts/verify.mjs <bundle.json>          verify one bundle
 *   node reference/ts/verify.mjs testvectors/valid       every file must verify
 *   node reference/ts/verify.mjs testvectors/invalid     every file must be rejected
 *
 * Exit code: 0 = VERIFIED (or every expectation held), 1 = REJECTED (or an
 * expectation was broken), 2 = the file could not be read.
 *
 * The algorithm below is the normative one, specified in spec/SPEC.md.
 */

import { readFileSync, readdirSync, statSync } from "node:fs";
import { createHash, createPublicKey, verify as nodeVerify } from "node:crypto";
import { pathToFileURL } from "node:url";

// ───────────────────────────────────────────────────────────── canonical JSON ──
// RFC 8785 (JCS). Object keys sorted by UTF-16 code unit, no whitespace,
// numbers in ECMAScript Number::toString form, non-ASCII left as UTF-8.
export function jcs(value) {
  if (value === null) return "null";
  if (typeof value === "boolean") return value ? "true" : "false";
  if (typeof value === "number") {
    if (!Number.isFinite(value)) throw new Error("a non-finite number has no canonical form");
    if (Object.is(value, -0)) return "0";
    return JSON.stringify(value);
  }
  if (typeof value === "string") return JSON.stringify(value);
  if (Array.isArray(value)) return "[" + value.map(jcs).join(",") + "]";
  if (typeof value === "object") {
    const keys = Object.keys(value).filter((k) => value[k] !== undefined).sort();
    return "{" + keys.map((k) => JSON.stringify(k) + ":" + jcs(value[k])).join(",") + "}";
  }
  throw new Error("a value of type " + typeof value + " has no canonical JSON form");
}

const sha256 = (bytes) => createHash("sha256").update(bytes).digest();

// ───────────────────────────────────────────────────────────────────── did:key ──
// did:key:z<base58btc multicodec ed25519-pub (0xed01) + 32-byte key>
const B58 = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

export function base58Decode(s) {
  let n = 0n;
  for (const c of s) {
    const i = B58.indexOf(c);
    if (i < 0) throw new Error("not a base58btc character: " + c);
    n = n * 58n + BigInt(i);
  }
  let hex = n.toString(16);
  if (hex.length % 2) hex = "0" + hex;
  const out = Buffer.from(hex, "hex");
  let zeros = 0;
  while (s[zeros] === "1") zeros++;
  return Buffer.concat([Buffer.alloc(zeros), out]);
}

/** multibase: 'z' = base58btc, 'u' = base64url (no padding). */
export function multibaseDecode(s) {
  if (typeof s !== "string" || s.length === 0) throw new Error("empty multibase value");
  if (s[0] === "z") return base58Decode(s.slice(1));
  if (s[0] === "u") return Buffer.from(s.slice(1), "base64url");
  throw new Error("unsupported multibase prefix: " + s[0]);
}

/** The raw 32-byte Ed25519 public key behind a did:key. */
export function didKeyToEd25519(did) {
  if (typeof did !== "string" || !did.startsWith("did:key:")) throw new Error("not a did:key: " + did);
  const bytes = multibaseDecode(did.slice("did:key:".length));
  if (bytes.length !== 34 || bytes[0] !== 0xed || bytes[1] !== 0x01) {
    throw new Error("not an ed25519-pub did:key");
  }
  return bytes.subarray(2);
}

/** The SPKI DER of an Ed25519 public key (Node wants a KeyObject). */
function ed25519KeyObject(raw) {
  return createPublicKey({
    key: Buffer.concat([Buffer.from("302a300506032b6570032100", "hex"), raw]),
    format: "der",
    type: "spki",
  });
}

/** The SPKI DER of a P-256 (ES256) public key given the COSE x and y coordinates. */
function p256KeyObject(cose) {
  const x = Buffer.from(cose.x, "base64url");
  const y = Buffer.from(cose.y, "base64url");
  if (x.length !== 32 || y.length !== 32) throw new Error("a P-256 coordinate is not 32 bytes");
  const point = Buffer.concat([Buffer.from([0x04]), x, y]);
  const der = Buffer.concat([
    Buffer.from("3059", "hex"),
    Buffer.from("3013", "hex"), Buffer.from("06072a8648ce3d0201", "hex"), Buffer.from("06082a8648ce3d030107", "hex"),
    Buffer.from("0342", "hex"), Buffer.from("00", "hex"), point,
  ]);
  return createPublicKey({ key: der, format: "der", type: "spki" });
}

// ───────────────────────────────────────────────────────── data integrity proof ──
// The signed message is NOT the document. It is
//     sha256(JCS(proofConfig)) ‖ sha256(JCS(document without `proof`))
// where proofConfig is the proof map with `proofValue` removed. See spec/SPEC.md §5.
export function dataIntegrityMessage(proofConfig, docWithoutProof) {
  return Buffer.concat([sha256(Buffer.from(jcs(proofConfig), "utf8")), sha256(Buffer.from(jcs(docWithoutProof), "utf8"))]);
}

/**
 * Verify a signed statement against the issuer's did:key.
 * Returns {ok, detail}. Never throws.
 */
export function verifyStatement(signed, issuerDid) {
  try {
    if (!signed || typeof signed !== "object") return { ok: false, detail: "the signed statement is not an object" };
    const proof = signed.proof;
    if (!proof || typeof proof !== "object") return { ok: false, detail: "the signed statement carries no proof" };
    if (proof.type !== "DataIntegrityProof") return { ok: false, detail: "the proof is not a DataIntegrityProof" };
    if (proof.cryptosuite !== "eddsa-jcs-2022") {
      return { ok: false, detail: "unsupported cryptosuite: " + proof.cryptosuite };
    }
    if (typeof proof.proofValue !== "string" || !proof.proofValue.startsWith("z")) {
      return { ok: false, detail: "the proof carries no multibase proofValue" };
    }

    // The proof's @context, if it has one, must be the document's.
    const docContext = signed["@context"];
    if (proof["@context"] !== undefined && docContext !== undefined) {
      if (jcs(proof["@context"]) !== jcs(docContext)) {
        return { ok: false, detail: "the proof's @context is not the document's @context" };
      }
    }

    // The verification method must name the issuer whose key we are checking against.
    const expectedFragment = "#" + issuerDid.slice("did:key:".length);
    if (typeof proof.verificationMethod !== "string" || !proof.verificationMethod.startsWith(issuerDid + "#")) {
      return { ok: false, detail: "the proof's verificationMethod does not name the issuer " + issuerDid };
    }
    if (!proof.verificationMethod.endsWith(expectedFragment)) {
      return { ok: false, detail: "the proof's verificationMethod does not carry the issuer's key id" };
    }

    // proofConfig = the proof minus proofValue.
    const proofConfig = { ...proof };
    delete proofConfig.proofValue;

    const docWithoutProof = { ...signed };
    delete docWithoutProof.proof;

    const raw = didKeyToEd25519(issuerDid);
    const sig = multibaseDecode(proof.proofValue);
    if (sig.length !== 64) return { ok: false, detail: "the signature is not 64 bytes" };

    const message = dataIntegrityMessage(proofConfig, docWithoutProof);
    let ok = false;
    try {
      ok = nodeVerify(null, message, ed25519KeyObject(raw), sig);
    } catch (e) {
      return { ok: false, detail: "the signature could not be checked: " + e.message };
    }
    if (!ok) return { ok: false, detail: "the signature does not verify against " + issuerDid };
    return { ok: true, detail: "the statement carries a valid signature from " + issuerDid };
  } catch (e) {
    return { ok: false, detail: "the statement is malformed and cannot be verified: " + e.message };
  }
}

// ────────────────────────────────────────────────────────────────── webauthn ──
/**
 * Verify the WebAuthn assertion against the credential the signed approval
 * names. Returns {ok, detail}. Never throws.
 */
export function verifyHumanAssertion(assertion, approval) {
  try {
    if (!assertion || typeof assertion !== "object") {
      return { ok: false, detail: "the bundle carries no human assertion: an approval without the human's passkey is not a proof" };
    }
    const response = assertion.response;
    if (!response || typeof response !== "object") {
      return { ok: false, detail: "the human assertion carries no response" };
    }

    // ── client data ──
    const clientDataJSON = Buffer.from(response.clientDataJSON, "base64url");
    const clientData = JSON.parse(clientDataJSON.toString("utf8"));
    if (clientData.type !== "webauthn.get") {
      return { ok: false, detail: "the client data is not a webauthn.get" };
    }
    if (clientData.challenge !== approval.challenge) {
      return { ok: false, detail: "the assertion answers a different challenge than the signed approval" };
    }
    if (clientData.origin !== approval.origin) {
      return { ok: false, detail: "the assertion comes from a different origin than the signed approval" };
    }

    // ── authenticator data ──
    const authData = Buffer.from(response.authenticatorData, "base64url");
    if (authData.length < 37) return { ok: false, detail: "the authenticator data is too short to be a WebAuthn assertion" };
    const rpIdHash = sha256(Buffer.from(approval.rp_id, "utf8"));
    if (!authData.subarray(0, 32).equals(rpIdHash)) {
      return { ok: false, detail: "the authenticator data is not bound to the relying party " + approval.rp_id };
    }
    const flags = authData[32];
    if ((flags & 0x01) === 0) return { ok: false, detail: "the user was not present for this assertion" };
    const signCount = authData.readUInt32BE(33);
    if (approval.sign_count > 0 && signCount <= approval.sign_count) {
      return { ok: false, detail: "the authenticator's signature counter did not advance: a cloned authenticator" };
    }

    // ── the signature over authenticatorData ‖ sha256(clientDataJSON) ──
    const signedData = Buffer.concat([authData, sha256(clientDataJSON)]);
    const sig = Buffer.from(response.signature, "base64url");
    const key = p256KeyObject(approval.credential_public_key);
    let ok = false;
    try {
      ok = nodeVerify(null, signedData, { key, dsaEncoding: "der" }, sig);
    } catch (e) {
      return { ok: false, detail: "the passkey signature could not be checked: " + e.message };
    }
    if (!ok) return { ok: false, detail: "the passkey signature does not verify" };

    return {
      ok: true,
      detail: "the passkey assertion verifies (user_present=" + ((flags & 0x01) !== 0) +
        ", user_verified=" + ((flags & 0x04) !== 0) + ")",
    };
  } catch (e) {
    return { ok: false, detail: "the passkey material in the bundle is malformed and cannot be verified" };
  }
}

// ─────────────────────────────────────────────────────────────── conformance ──
/**
 * Re-evaluate every constraint of the request against the proposal.
 * The verdict is "conformant" only if EVERY constraint holds.
 */
export function checkAll(intent, proposal) {
  const constraints = intent && typeof intent === "object" ? intent.constraints : undefined;
  if (!Array.isArray(constraints)) {
    return {
      verdict: "non_conformant",
      checks: [],
      passed: 0,
      total: 0,
      violations: [{ ok: false, observed: null, detail: "the request names no constraints, so nothing can be shown to conform" }],
    };
  }
  const checks = constraints.map((c) => checkConstraint(c, proposal));
  const violations = checks.filter((r) => !r.ok);
  return {
    verdict: violations.length === 0 ? "conformant" : "non_conformant",
    checks,
    passed: checks.length - violations.length,
    total: checks.length,
    violations,
  };
}

function checkConstraint(c, proposal) {
  const field = c && typeof c === "object" ? c.field : undefined;
  if (field === undefined) {
    return { constraint: c, ok: false, observed: null, detail: "the constraint is malformed (it names no field)" };
  }
  const op = c.op;
  if (op === undefined) {
    return { constraint: c, ok: false, observed: null, detail: "the constraint is malformed (it names no operator)" };
  }
  const bound = c.value;
  if (bound === undefined) {
    return { constraint: c, ok: false, observed: null, detail: "the constraint is malformed (it names no bound)" };
  }
  const observed = proposal && typeof proposal === "object" ? proposal[field] : undefined;
  if (observed === undefined) {
    return { constraint: c, ok: false, observed: null, detail: "the proposal does not state a " + field };
  }

  let ok = false;
  let detail = "";
  try {
    if (op === "<=") {
      if (typeof observed !== "number" || typeof bound !== "number") throw new Error("not comparable");
      ok = observed <= bound;
      detail = ok ? "within the limit (" + observed + " <= " + bound + ")"
        : "the proposal's " + field + " (" + observed + ") exceeds the limit (" + bound + ")";
    } else if (op === "==") {
      if (typeof observed !== typeof bound) throw new Error("not comparable");
      ok = observed === bound;
      detail = ok ? "matches (" + observed + ")" : "the proposal's " + field + " is " + observed + ", not " + bound;
    } else if (op === ">=") {
      if (typeof observed !== "number" || typeof bound !== "number") throw new Error("not comparable");
      ok = observed >= bound;
      detail = ok ? "meets the minimum (" + observed + " >= " + bound + ")"
        : "the proposal's " + field + " (" + observed + ") is below the required minimum (" + bound + ")";
    } else if (op === "contains") {
      ok = typeof observed === "string" && observed.includes(String(bound));
      detail = ok ? "contains " + bound : "the proposal's " + field + " does not contain " + bound;
    } else if (op === "in") {
      ok = Array.isArray(bound) && bound.some((x) => x === observed);
      detail = ok ? "one of the allowed ones" : "the proposal's " + field + " is not among the allowed ones";
    } else {
      detail = "the constraint names an unknown operator: " + op;
    }
  } catch {
    return { constraint: c, ok: false, observed, detail: "the proposal's " + field + " could not be compared against the constraint" };
  }
  return { constraint: c, ok, observed, detail };
}

// ─────────────────────────────────────────────────────────────── the 9 checks ──
export const DEFAULT_ISSUER = "did:key:z6MkvRXNYcE7MMduynWTgeKbDaT1iijDSC8pZqXZc8rHPrf2";

export function verifyBundle(bundle, issuerDid = DEFAULT_ISSUER) {
  const checks = [];
  const add = (name, ok, detail) => checks.push({ name, ok, detail });

  // A bundle that is not shaped like a bundle is REJECTED, never an error.
  if (!bundle || typeof bundle !== "object" || Array.isArray(bundle)) {
    add("bundle_wellformed", false, "the bundle is not an object");
    return { ok: false, checks, failures: checks, summary: "REJECTED" };
  }
  const signed = bundle.signed_statement;
  const cs = signed && typeof signed === "object" ? signed.credentialSubject : undefined;
  if (!cs || typeof cs !== "object") {
    add("bundle_wellformed", false, "the bundle carries no credentialSubject");
    return { ok: false, checks, failures: checks, summary: "REJECTED" };
  }

  // 1. the signature over the statement is genuine and from the claimed issuer
  const sig = verifyStatement(signed, issuerDid);
  add("signature", sig.ok, sig.detail);

  // 2. the recorded intent is the one the fingerprint commits to
  let iOk = false;
  let iDetail = "the intent was modified after signing";
  try {
    iOk = "sha256:" + sha256(Buffer.from(jcs(bundle.intent), "utf8")).toString("hex") === cs.intent_fingerprint;
    iDetail = iOk ? "the intent matches the fingerprint that was signed" : "the intent was modified after signing";
  } catch (e) {
    iDetail = "the intent cannot be canonicalized: " + e.message;
  }
  add("intent_integrity", iOk, iDetail);

  // 3. the recorded proposal is the one the fingerprint commits to
  let pOk = false;
  let pDetail = "the proposal was modified after signing";
  try {
    pOk = "sha256:" + sha256(Buffer.from(jcs(bundle.proposal), "utf8")).toString("hex") === cs.proposal_fingerprint;
    pDetail = pOk ? "the proposal matches the fingerprint that was signed" : "the proposal was modified after signing";
  } catch (e) {
    pDetail = "the proposal cannot be canonicalized: " + e.message;
  }
  add("proposal_integrity", pOk, pDetail);

  // 4. re-run the conformance check from scratch and compare to the signed verdict
  const recomputed = checkAll(bundle.intent, bundle.proposal);
  const vOk = recomputed.verdict === cs.verdict;
  add("verdict_recomputed", vOk, vOk
    ? "re-running the checks yields the signed verdict (" + recomputed.verdict + ")"
    : "re-running the checks yields " + recomputed.verdict + ", but the signed verdict says " + cs.verdict);

  // 5. the RECOMPUTED verdict must be conformant for the approval to mean anything
  const confOk = recomputed.verdict === "conformant";
  add("conformance", confOk, confOk
    ? "the approved proposal satisfies every constraint of the request"
    : "the approved proposal does NOT satisfy the request: " + recomputed.violations.map((v) => v.detail).join("; "));

  // 6. the approval record is INSIDE the signed subject
  const approval = cs.approval;
  const apPresent = approval !== undefined && approval !== null && typeof approval === "object";
  add("approval_signed", apPresent, apPresent
    ? "the approval record is covered by the signature"
    : "the signed statement carries no approval record");

  // 7. the human's own assertion must verify against the SIGNED credential
  const hasAssertion = bundle.human_assertion !== undefined && bundle.human_assertion !== null;
  let humanOk = false;
  let humanDetail = "the bundle carries no human assertion: an approval without the human's passkey is not a proof";
  if (apPresent && hasAssertion) {
    const hv = verifyHumanAssertion(bundle.human_assertion, approval);
    humanOk = hv.ok;
    humanDetail = hv.detail;
  }
  add("human_approval", humanOk, humanDetail);

  // 8. the assertion must be the one the signed approval names
  let idOk = false;
  let idDetail = "the bundle carries no human assertion";
  if (apPresent && hasAssertion) {
    const declared = bundle.human_assertion.id;
    const expected = approval.credential_id;
    idOk = declared === expected;
    idDetail = idOk
      ? "the assertion is the credential the signed approval names"
      : "the assertion names a different credential than the signed approval";
  }
  add("assertion_binding", idOk, idDetail);

  // 9. WebAuthn does not cover type, rawId or userHandle; they must still agree
  let shapeOk = true;
  let shapeDetail = "no human assertion recorded";
  if (apPresent && hasAssertion) {
    const a = bundle.human_assertion;
    const typeOk = a.type === "public-key";
    const rawOk = a.rawId === a.id;
    const handleOk = (a.response && a.response.userHandle) === approval.user_handle;
    shapeOk = typeOk && rawOk && handleOk;
    if (shapeOk) {
      shapeDetail = "the assertion's type, rawId and userHandle agree with the signed approval";
    } else {
      let why = "";
      if (!typeOk) why += "the assertion's type is not public-key; ";
      if (!rawOk) why += "the assertion's rawId does not match its id; ";
      if (!handleOk) why += "the assertion's userHandle does not match the signed approval; ";
      shapeDetail = why;
    }
  }
  add("assertion_shape", shapeOk, shapeDetail);

  const failures = checks.filter((c) => !c.ok);
  return { ok: failures.length === 0, checks, failures, summary: failures.length === 0 ? "VERIFIED" : "REJECTED" };
}

// ──────────────────────────────────────────────────────────────────────── CLI ──
function readBundle(path) {
  const raw = readFileSync(path, "utf8");
  return JSON.parse(raw);
}

function printVerdict(path, verdict) {
  console.log("agent-payments · offline verification");
  console.log("bundle   → " + path);
  console.log("issuer   → " + DEFAULT_ISSUER);
  console.log("");
  for (const c of verdict.checks) {
    console.log((c.ok ? "  [ok] " : "  [XX] ") + c.name);
    console.log("        " + c.detail);
  }
  console.log("");
  console.log("─── " + verdict.summary + " ───");
  for (const f of verdict.failures) console.log("  " + f.name + ": " + f.detail);
}

function main(argv) {
  if (argv.length !== 1) {
    console.error("usage: node verify.mjs <bundle.json | directory>");
    return 2;
  }
  const target = argv[0];
  let st;
  try {
    st = statSync(target);
  } catch {
    console.error("cannot read: " + target);
    return 2;
  }

  if (st.isDirectory()) {
    const expectVerified = /(^|[/\\])valid$/.test(target);
    const files = readdirSync(target).filter((f) => f.endsWith(".json")).sort();
    if (files.length === 0) {
      console.error("no .json files in " + target);
      return 2;
    }
    let bad = 0;
    for (const f of files) {
      const path = target.replace(/[/\\]+$/, "") + "/" + f;
      let verdict;
      try {
        verdict = verifyBundle(readBundle(path));
      } catch (e) {
        console.log("  [XX] " + f + ": the file is not valid JSON (" + e.message + ")");
        bad++;
        continue;
      }
      const shouldVerify = expectVerified;
      const good = verdict.ok === shouldVerify;
      if (!good) bad++;
      console.log((good ? "  [ok] " : "  [XX] ") + f + " → " + verdict.summary +
        (expectVerified ? "" : " (expected REJECTED)"));
      if (!good) for (const x of verdict.failures) console.log("        " + x.name + ": " + x.detail);
    }
    console.log("");
    console.log(files.length + " file(s), " + (files.length - bad) + " as expected, " + bad + " not.");
    return bad === 0 ? 0 : 1;
  }

  let bundle;
  try {
    bundle = readBundle(target);
  } catch (e) {
    console.error("the file is not valid JSON: " + e.message);
    return 2;
  }
  const verdict = verifyBundle(bundle);
  printVerdict(target, verdict);
  return verdict.ok ? 0 : 1;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.exit(main(process.argv.slice(2)));
}
