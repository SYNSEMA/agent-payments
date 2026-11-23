//! agent-payments — reference verifier (Rust).
//!
//! The second independent implementation of the specification in `spec/SPEC.md`.
//! It exists for the auditor: a Rust crate with no platform dependency, no
//! network and no account, that reaches the same verdict as the TypeScript
//! verifier and as the Synsema reference implementation.
//!
//! Unlike the TypeScript one, this crate uses crates.io (`ed25519-dalek`,
//! `p256`, `sha2`, `serde_json`). Rust's standard library has no cryptography,
//! so this is a deliberate trade: audited dependencies instead of a
//! hand-rolled curve implementation.

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

// ───────────────────────────────────────────────────────────── canonical JSON ──

/// RFC 8785 (JCS). Object members sorted by UTF-16 code unit order, no
/// whitespace, numbers in ECMAScript `Number::toString` form.
pub fn jcs(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => {
            if let Some(u) = n.as_u64() {
                u.to_string()
            } else if let Some(i) = n.as_i64() {
                i.to_string()
            } else {
                let f = n.as_f64().unwrap_or(f64::NAN);
                let mut s = format!("{}", f);
                if s == "-0" {
                    s = "0".to_string();
                }
                s
            }
        }
        Value::String(s) => json_string(s),
        Value::Array(a) => {
            let parts: Vec<String> = a.iter().map(jcs).collect();
            format!("[{}]", parts.join(","))
        }
        Value::Object(o) => {
            let mut keys: Vec<&String> = o.keys().collect();
            keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
            let parts: Vec<String> = keys
                .iter()
                .map(|k| format!("{}:{}", json_string(k), jcs(&o[*k])))
                .collect();
            format!("{{{}}}", parts.join(","))
        }
    }
}

/// JSON string escaping: the mandatory ones, and the control characters.
/// Non-ASCII is left as UTF-8, as RFC 8785 requires.
fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    let out = h.finalize();
    out.iter().map(|b| format!("{:02x}", b)).collect()
}

/// The fingerprint of a value: `"sha256:" + hex(SHA-256(JCS(value)))`.
pub fn fingerprint(value: &Value) -> String {
    format!("sha256:{}", sha256_hex(jcs(value).as_bytes()))
}

// ───────────────────────────────────────────────────────────────── multibase ──

const B58: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn base58_decode(s: &str) -> Result<Vec<u8>, String> {
    let mut n: Vec<u8> = Vec::new(); // big-endian byte accumulator
    for c in s.bytes() {
        let idx = B58.iter().position(|&x| x == c).ok_or("not a base58btc character")?;
        let mut carry = idx as u32;
        for byte in n.iter_mut().rev() {
            let v = (*byte as u32) * 58 + carry;
            *byte = (v & 0xff) as u8;
            carry = v >> 8;
        }
        while carry > 0 {
            n.insert(0, (carry & 0xff) as u8);
            carry >>= 8;
        }
    }
    let zeros = s.bytes().take_while(|&c| c == b'1').count();
    let mut out = vec![0u8; zeros];
    out.extend_from_slice(&n);
    Ok(out)
}

/// multibase: `z` = base58btc, `u` = base64url.
pub fn multibase_decode(s: &str) -> Result<Vec<u8>, String> {
    use base64::Engine;
    match s.chars().next() {
        Some('z') => base58_decode(&s[1..]),
        Some('u') => base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(&s[1..])
            .map_err(|e| e.to_string()),
        other => Err(format!("unsupported multibase prefix: {:?}", other)),
    }
}

/// The raw 32-byte Ed25519 public key behind a `did:key`.
pub fn did_key_to_ed25519(did: &str) -> Result<[u8; 32], String> {
    let suffix = did.strip_prefix("did:key:").ok_or("not a did:key")?;
    let bytes = multibase_decode(suffix)?;
    if bytes.len() != 34 || bytes[0] != 0xed || bytes[1] != 0x01 {
        return Err("not an ed25519-pub did:key".to_string());
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes[2..]);
    Ok(out)
}

// ───────────────────────────────────────────────────────── data integrity ──

/// `SHA-256(JCS(proofConfig)) ‖ SHA-256(JCS(document without proof))`.
pub fn data_integrity_message(proof_config: &Value, doc_without_proof: &Value) -> Vec<u8> {
    let mut out = Sha256::digest(jcs(proof_config).as_bytes()).to_vec();
    out.extend_from_slice(&Sha256::digest(jcs(doc_without_proof).as_bytes())[..]);
    out
}

/// Verify a signed statement against the issuer's `did:key`.
pub fn verify_statement(signed: &Value, issuer_did: &str) -> (bool, String) {
    let obj = match signed.as_object() {
        Some(o) => o,
        None => return (false, "the signed statement is not an object".to_string()),
    };
    let proof = match obj.get("proof").and_then(|v| v.as_object()) {
        Some(p) => p.clone(),
        None => return (false, "the signed statement carries no proof".to_string()),
    };
    if proof.get("type").and_then(|v| v.as_str()) != Some("DataIntegrityProof") {
        return (false, "the proof is not a DataIntegrityProof".to_string());
    }
    if proof.get("cryptosuite").and_then(|v| v.as_str()) != Some("eddsa-jcs-2022") {
        return (false, "unsupported cryptosuite".to_string());
    }
    let pv = match proof.get("proofValue").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => return (false, "the proof carries no multibase proofValue".to_string()),
    };

    // The proof's @context, if it has one, must be the document's.
    if let (Some(pc), Some(dc)) = (proof.get("@context"), obj.get("@context")) {
        if jcs(pc) != jcs(dc) {
            return (false, "the proof's @context is not the document's @context".to_string());
        }
    }

    // The verification method must name the issuer whose key we check against.
    let key_id = issuer_did.strip_prefix("did:key:").unwrap_or("");
    let expected_vm = format!("{}#{}", issuer_did, key_id);
    match proof.get("verificationMethod").and_then(|v| v.as_str()) {
        Some(vm) if vm == expected_vm => {}
        _ => {
            return (
                false,
                format!("the proof's verificationMethod does not name the issuer {}", issuer_did),
            )
        }
    }

    // proofConfig = the proof minus proofValue.
    let mut cfg: Map<String, Value> = proof.clone();
    cfg.remove("proofValue");
    let cfg_val = Value::Object(cfg);

    let mut doc_without_proof = obj.clone();
    doc_without_proof.remove("proof");

    let raw = match did_key_to_ed25519(issuer_did) {
        Ok(r) => r,
        Err(e) => return (false, format!("the issuer's key is unusable: {}", e)),
    };
    let sig_bytes = match multibase_decode(pv) {
        Ok(s) => s,
        Err(e) => return (false, format!("the proofValue is not decodable: {}", e)),
    };
    if sig_bytes.len() != 64 {
        return (false, "the signature is not 64 bytes".to_string());
    }
    let sig: [u8; 64] = match sig_bytes.try_into() {
        Ok(s) => s,
        Err(_) => return (false, "the signature is not 64 bytes".to_string()),
    };
    let vk = match ed25519_dalek::VerifyingKey::from_bytes(&raw.into()) {
        Ok(k) => k,
        Err(e) => return (false, format!("the issuer's key is invalid: {}", e)),
    };
    let message = data_integrity_message(&cfg_val, &Value::Object(doc_without_proof));
    match ed25519_dalek::Verifier::verify(&vk, &message, &ed25519_dalek::Signature::from_bytes(&sig)) {
        Ok(()) => (true, format!("the statement carries a valid signature from {}", issuer_did)),
        Err(_) => (false, format!("the signature does not verify against {}", issuer_did)),
    }
}

/// Whether two JSON values are of the same kind, so that `==` never coerces.
fn same_kind(a: &Value, b: &Value) -> bool {
    matches!(
        (a, b),
        (Value::Null, Value::Null)
            | (Value::Bool(_), Value::Bool(_))
            | (Value::Number(_), Value::Number(_))
            | (Value::String(_), Value::String(_))
            | (Value::Array(_), Value::Array(_))
            | (Value::Object(_), Value::Object(_))
    )
}

// ───────────────────────────────────────────────────────────────── webauthn ──

/// Verify the WebAuthn assertion against the credential the signed approval names.
pub fn verify_human_assertion(assertion: &Value, approval: &Value) -> (bool, String) {
    use base64::Engine;
    let b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let obj = match assertion.as_object() {
        Some(o) => o,
        None => {
            return (
                false,
                "the bundle carries no human assertion: an approval without the human's passkey is not a proof"
                    .to_string(),
            )
        }
    };
    let response = match obj.get("response").and_then(|v| v.as_object()) {
        Some(r) => r,
        None => return (false, "the human assertion carries no response".to_string()),
    };
    let get = |m: &Map<String, Value>, k: &str| m.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string();

    let client_data_json = match b64.decode(get(response, "clientDataJSON")) {
        Ok(b) => b,
        Err(_) => return (false, "the client data is not base64url".to_string()),
    };
    let client_data: Value = match serde_json::from_slice(&client_data_json) {
        Ok(v) => v,
        Err(_) => return (false, "the client data is not JSON".to_string()),
    };
    let cd = client_data.as_object().cloned().unwrap_or_default();
    if cd.get("type").and_then(|v| v.as_str()) != Some("webauthn.get") {
        return (false, "the client data is not a webauthn.get".to_string());
    }
    if cd.get("challenge").and_then(|v| v.as_str()) != approval.get("challenge").and_then(|v| v.as_str()) {
        return (false, "the assertion answers a different challenge than the signed approval".to_string());
    }
    if cd.get("origin").and_then(|v| v.as_str()) != approval.get("origin").and_then(|v| v.as_str()) {
        return (false, "the assertion comes from a different origin than the signed approval".to_string());
    }

    let auth_data = match b64.decode(get(response, "authenticatorData")) {
        Ok(b) => b,
        Err(_) => return (false, "the authenticator data is not base64url".to_string()),
    };
    if auth_data.len() < 37 {
        return (false, "the authenticator data is too short to be a WebAuthn assertion".to_string());
    }
    let rp_id = approval.get("rp_id").and_then(|v| v.as_str()).unwrap_or("");
    let rp_hash = Sha256::digest(rp_id.as_bytes());
    if &auth_data[..32] != &rp_hash[..] {
        return (false, format!("the authenticator data is not bound to the relying party {}", rp_id));
    }
    let flags = auth_data[32];
    if flags & 0x01 == 0 {
        return (false, "the user was not present for this assertion".to_string());
    }
    let sign_count = u32::from_be_bytes([auth_data[33], auth_data[34], auth_data[35], auth_data[36]]);
    let stored = approval.get("sign_count").and_then(|v| v.as_u64()).unwrap_or(0);
    if stored > 0 && sign_count as u64 <= stored {
        return (false, "the authenticator's signature counter did not advance: a cloned authenticator".to_string());
    }

    let cose = match approval.get("credential_public_key").and_then(|v| v.as_object()) {
        Some(c) => c,
        None => return (false, "the approval carries no credential public key".to_string()),
    };
    let x = match b64.decode(get(cose, "x")) {
        Ok(v) => v,
        Err(_) => return (false, "the credential's x coordinate is not base64url".to_string()),
    };
    let y = match b64.decode(get(cose, "y")) {
        Ok(v) => v,
        Err(_) => return (false, "the credential's y coordinate is not base64url".to_string()),
    };
    let xa: [u8; 32] = match x.try_into() {
        Ok(v) => v,
        Err(_) => return (false, "the x coordinate is not 32 bytes".to_string()),
    };
    let ya: [u8; 32] = match y.try_into() {
        Ok(v) => v,
        Err(_) => return (false, "the y coordinate is not 32 bytes".to_string()),
    };
    let point = p256::EncodedPoint::from_affine_coordinates((&xa).into(), (&ya).into(), false);
    let vk = match p256::ecdsa::VerifyingKey::from_encoded_point(&point) {
        Ok(k) => k,
        Err(_) => return (false, "the credential's public key is not a valid P-256 point".to_string()),
    };

    let sig_der = match b64.decode(get(response, "signature")) {
        Ok(v) => v,
        Err(_) => return (false, "the signature is not base64url".to_string()),
    };
    let sig = match ecdsa::Signature::from_der(&sig_der) {
        Ok(s) => s,
        Err(_) => return (false, "the signature is not a DER ECDSA signature".to_string()),
    };

    let mut signed_data = auth_data.clone();
    signed_data.extend_from_slice(&Sha256::digest(&client_data_json));
    use p256::ecdsa::signature::Verifier;
    match p256::ecdsa::signature::Verifier::verify(&vk, &signed_data, &sig) {
        Ok(()) => (
            true,
            format!(
                "the passkey assertion verifies (user_present={}, user_verified={})",
                flags & 0x01 != 0,
                flags & 0x04 != 0
            ),
        ),
        Err(_) => (false, "the passkey signature does not verify".to_string()),
    }
}

// ─────────────────────────────────────────────────────────────── conformance ──

/// One check over one constraint.
pub fn check_constraint(c: &Value, proposal: &Value) -> Value {
    let obj = match c.as_object() {
        Some(o) => o,
        None => return serde_json::json!({"constraint": c, "ok": false, "observed": Value::Null,
            "detail": "the constraint is malformed (it is not an object)"}),
    };
    let field = match obj.get("field").and_then(|v| v.as_str()) {
        Some(f) => f.to_string(),
        None => return serde_json::json!({"constraint": c, "ok": false, "observed": Value::Null,
            "detail": "the constraint is malformed (it names no field)"}),
    };
    let op = match obj.get("op").and_then(|v| v.as_str()) {
        Some(o) => o.to_string(),
        None => return serde_json::json!({"constraint": c, "ok": false, "observed": Value::Null,
            "detail": "the constraint is malformed (it names no operator)"}),
    };
    let bound = match obj.get("value") {
        Some(v) => v.clone(),
        None => return serde_json::json!({"constraint": c, "ok": false, "observed": Value::Null,
            "detail": "the constraint is malformed (it names no bound)"}),
    };
    let observed = proposal.get(&field).cloned();
    let observed = match observed {
        Some(v) => v,
        None => return serde_json::json!({"constraint": c, "ok": false, "observed": Value::Null,
            "detail": format!("the proposal does not state a {}", field)}),
    };

    let num = |v: &Value| v.as_f64();
    let mut ok = false;
    let detail: String;
    match op.as_str() {
        "<=" | ">=" => {
            match (num(&observed), num(&bound)) {
                (Some(o), Some(b)) => {
                    ok = if op == "<=" { o <= b } else { o >= b };
                    detail = if ok {
                        format!("{} ({}) {} {}", if op == "<=" { "within the limit (" } else { "meets the minimum (" },
                            observed, op, bound)
                    } else if op == "<=" {
                        format!("the proposal's {} ({}) exceeds the limit ({})", field, observed, bound)
                    } else {
                        format!("the proposal's {} ({}) is below the required minimum ({})", field, observed, bound)
                    };
                }
                _ => {
                    return serde_json::json!({"constraint": c, "ok": false, "observed": observed,
                        "detail": format!("the proposal's {} could not be compared against the constraint", field)})
                }
            }
        }
        "==" => {
            if !same_kind(&observed, &bound) {
                return serde_json::json!({"constraint": c, "ok": false, "observed": observed,
                    "detail": format!("the proposal's {} could not be compared against the constraint", field)});
            }
            ok = observed == bound;
            detail = if ok {
                format!("matches ({})", observed)
            } else {
                format!("the proposal's {} is {}, not {}", field, observed, bound)
            };
        }
        "contains" => {
            ok = observed.as_str().map(|s| s.contains(bound.as_str().unwrap_or(""))).unwrap_or(false);
            detail = if ok {
                format!("contains {}", bound)
            } else {
                format!("the proposal's {} does not contain {}", field, bound)
            };
        }
        "in" => {
            ok = bound.as_array().map(|a| a.iter().any(|x| *x == observed)).unwrap_or(false);
            detail = if ok {
                "one of the allowed ones".to_string()
            } else {
                format!("the proposal's {} is not among the allowed ones", field)
            };
        }
        other => {
            detail = format!("the constraint names an unknown operator: {}", other);
        }
    }
    serde_json::json!({"constraint": c, "ok": ok, "observed": observed, "detail": detail})
}

/// Re-evaluate every constraint of the request against the proposal.
pub fn check_all(intent: &Value, proposal: &Value) -> Value {
    let constraints = intent.get("constraints").and_then(|v| v.as_array());
    let constraints = match constraints {
        Some(c) => c,
        None => return serde_json::json!({
            "verdict": "non_conformant", "checks": [], "passed": 0, "total": 0,
            "violations": [{"ok": false, "observed": Value::Null,
                "detail": "the request names no constraints, so nothing can be shown to conform"}]
        }),
    };
    let checks: Vec<Value> = constraints.iter().map(|c| check_constraint(c, proposal)).collect();
    let violations: Vec<Value> = checks
        .iter()
        .filter(|c| c.get("ok").and_then(|v| v.as_bool()) != Some(true))
        .cloned()
        .collect();
    let verdict = if violations.is_empty() { "conformant" } else { "non_conformant" };
    serde_json::json!({
        "verdict": verdict,
        "checks": checks,
        "passed": checks.len() - violations.len(),
        "total": checks.len(),
        "violations": violations
    })
}

// ──────────────────────────────────────────────────────────────── the 9 checks ──

pub const DEFAULT_ISSUER: &str = "did:key:z6MkvRXNYcE7MMduynWTgeKbDaT1iijDSC8pZqXZc8rHPrf2";

#[derive(Debug, Clone)]
pub struct Check {
    pub name: &'static str,
    pub ok: bool,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct Verdict {
    pub ok: bool,
    pub summary: &'static str,
    pub checks: Vec<Check>,
}

impl Verdict {
    pub fn failures(&self) -> Vec<&Check> {
        self.checks.iter().filter(|c| !c.ok).collect()
    }
}

/// Verify a bundle. **Fails closed**: a malformed bundle is REJECTED, never
/// an error and never a partial success.
pub fn verify_bundle(bundle: &Value, issuer_did: &str) -> Verdict {
    let mut checks: Vec<Check> = Vec::new();
    let obj = match bundle.as_object() {
        Some(o) => o,
        None => {
            checks.push(Check { name: "bundle_wellformed", ok: false, detail: "the bundle is not an object".into() });
            return Verdict { ok: false, summary: "REJECTED", checks };
        }
    };
    let signed = obj.get("signed_statement");
    let cs = signed.and_then(|s| s.get("credentialSubject")).and_then(|c| c.as_object());
    let cs = match cs {
        Some(c) => c.clone(),
        None => {
            checks.push(Check { name: "bundle_wellformed", ok: false, detail: "the bundle carries no credentialSubject".into() });
            return Verdict { ok: false, summary: "REJECTED", checks };
        }
    };

    // 1. signature
    let (sig_ok, sig_detail) = match signed {
        Some(s) => verify_statement(s, issuer_did),
        None => (false, "the bundle carries no signed statement".to_string()),
    };
    checks.push(Check { name: "signature", ok: sig_ok, detail: sig_detail });

    // 2. intent integrity
    let i_fp = fingerprint(obj.get("intent").unwrap_or(&Value::Null));
    let i_want = cs.get("intent_fingerprint").and_then(|v| v.as_str()).unwrap_or("");
    let i_ok = i_fp == i_want;
    checks.push(Check {
        name: "intent_integrity",
        ok: i_ok,
        detail: if i_ok { "the intent matches the fingerprint that was signed".into() }
                 else { "the intent was modified after signing".into() },
    });

    // 3. proposal integrity
    let p_fp = fingerprint(obj.get("proposal").unwrap_or(&Value::Null));
    let p_want = cs.get("proposal_fingerprint").and_then(|v| v.as_str()).unwrap_or("");
    let p_ok = p_fp == p_want;
    checks.push(Check {
        name: "proposal_integrity",
        ok: p_ok,
        detail: if p_ok { "the proposal matches the fingerprint that was signed".into() }
                 else { "the proposal was modified after signing".into() },
    });

    // 4. the verdict recomputed
    let recomputed = check_all(obj.get("intent").unwrap_or(&Value::Null), obj.get("proposal").unwrap_or(&Value::Null));
    let r_verdict = recomputed.get("verdict").and_then(|v| v.as_str()).unwrap_or("");
    let signed_verdict = cs.get("verdict").and_then(|v| v.as_str()).unwrap_or("");
    let v_ok = r_verdict == signed_verdict;
    checks.push(Check {
        name: "verdict_recomputed",
        ok: v_ok,
        detail: if v_ok {
            format!("re-running the checks yields the signed verdict ({})", r_verdict)
        } else {
            format!("re-running the checks yields {}, but the signed verdict says {}", r_verdict, signed_verdict)
        },
    });

    // 5. the RECOMPUTED verdict must be conformant
    let conf_ok = r_verdict == "conformant";
    let violations: Vec<String> = recomputed
        .get("violations")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().map(|v| v.get("detail").and_then(|d| d.as_str()).unwrap_or("").to_string()).collect())
        .unwrap_or_default();
    checks.push(Check {
        name: "conformance",
        ok: conf_ok,
        detail: if conf_ok {
            "the approved proposal satisfies every constraint of the request".into()
        } else {
            format!("the approved proposal does NOT satisfy the request: {}", violations.join("; "))
        },
    });

    // 6. the approval record is inside the signed subject
    let approval = cs.get("approval").cloned();
    let ap_present = approval.as_ref().map(|a| a.is_object()).unwrap_or(false);
    checks.push(Check {
        name: "approval_signed",
        ok: ap_present,
        detail: if ap_present { "the approval record is covered by the signature".into() }
                 else { "the signed statement carries no approval record".into() },
    });

    let assertion = obj.get("human_assertion").filter(|v| !v.is_null()).cloned();

    // 7. the human's assertion
    let (human_ok, human_detail) = match (ap_present, assertion.as_ref()) {
        (true, Some(a)) => {
            let approval = approval.as_ref().unwrap();
            verify_human_assertion(a, approval)
        }
        _ => (
            false,
            "the bundle carries no human assertion: an approval without the human's passkey is not a proof".into(),
        ),
    };
    checks.push(Check { name: "human_approval", ok: human_ok, detail: human_detail });

    // 8. the assertion is the credential the signed approval names
    let (id_ok, id_detail) = match (ap_present, assertion.as_ref()) {
        (true, Some(a)) => {
            let expected = approval.as_ref().unwrap().get("credential_id").and_then(|v| v.as_str());
            let declared = a.get("id").and_then(|v| v.as_str());
            let ok = expected.is_some() && expected == declared;
            (
                ok,
                if ok { "the assertion is the credential the signed approval names".into() }
                else { "the assertion names a different credential than the signed approval".into() },
            )
        }
        _ => (false, "the bundle carries no human assertion".to_string()),
    };
    checks.push(Check { name: "assertion_binding", ok: id_ok, detail: id_detail });

    // 9. the assertion's shape
    let (shape_ok, shape_detail) = match (ap_present, assertion.as_ref()) {
        (true, Some(a)) => {
            let expected_handle = approval.as_ref().unwrap().get("user_handle").and_then(|v| v.as_str());
            let type_ok = a.get("type").and_then(|v| v.as_str()) == Some("public-key");
            let raw_ok = a.get("rawId").and_then(|v| v.as_str()) == a.get("id").and_then(|v| v.as_str());
            let handle_ok = a
                .get("response")
                .and_then(|r| r.get("userHandle"))
                .and_then(|v| v.as_str())
                .is_some()
                && a.get("response")
                    .and_then(|r| r.get("userHandle"))
                    .and_then(|v| v.as_str())
                    == expected_handle;
            let ok = type_ok && raw_ok && handle_ok;
            let detail = if ok {
                "the assertion's type, rawId and userHandle agree with the signed approval".to_string()
            } else {
                let mut why = String::new();
                if !type_ok { why.push_str("the assertion's type is not public-key; "); }
                if !raw_ok { why.push_str("the assertion's rawId does not match its id; "); }
                if !handle_ok { why.push_str("the assertion's userHandle does not match the signed approval; "); }
                why
            };
            (ok, detail)
        }
        _ => (true, "no human assertion recorded".to_string()),
    };
    checks.push(Check { name: "assertion_shape", ok: shape_ok, detail: shape_detail });

    let ok = checks.iter().all(|c| c.ok);
    Verdict {
        ok,
        summary: if ok { "VERIFIED" } else { "REJECTED" },
        checks,
    }
}
