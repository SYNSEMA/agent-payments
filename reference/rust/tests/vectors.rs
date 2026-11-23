//! The conformance suite: every vector must behave as `spec/SPEC.md` says.
//!
//! These are the same vectors CI runs. The TypeScript verifier and the Synsema
//! reference implementation run the identical set, so a divergence between the
//! three is a release blocker.

use agent_payments_verifier::{verify_bundle, DEFAULT_ISSUER};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn vectors(dir: &str) -> Vec<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("testvectors")
        .join(dir);
    let mut out: Vec<PathBuf> = fs::read_dir(&root)
        .expect("the test vectors are missing")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "json").unwrap_or(false))
        .collect();
    out.sort();
    out
}

fn load(p: &PathBuf) -> Value {
    serde_json::from_str(&fs::read_to_string(p).expect("cannot read the vector")).expect("the vector is not JSON")
}

#[test]
fn every_valid_vector_verifies() {
    let files = vectors("valid");
    assert!(!files.is_empty(), "there are no valid vectors");
    for f in files {
        let verdict = verify_bundle(&load(&f), DEFAULT_ISSUER);
        assert!(verdict.ok, "{} should have verified: {:?}", f.display(), verdict.failures());
    }
}

#[test]
fn every_invalid_vector_is_rejected() {
    let files = vectors("invalid");
    assert!(!files.is_empty(), "there are no invalid vectors");
    for f in files {
        let verdict = verify_bundle(&load(&f), DEFAULT_ISSUER);
        assert!(!verdict.ok, "{} should have been rejected but it verified", f.display());
    }
}

#[test]
fn a_malformed_bundle_is_rejected_not_an_error() {
    for junk in [
        serde_json::json!({}),
        serde_json::json!({"intent": null, "proposal": null}),
        serde_json::json!({"signed_statement": {}}),
        serde_json::json!({"signed_statement": {"credentialSubject": {}}}),
        serde_json::json!([]),
        serde_json::json!("not a bundle"),
    ] {
        let verdict = verify_bundle(&junk, DEFAULT_ISSUER);
        assert!(!verdict.ok);
        assert_eq!(verdict.summary, "REJECTED");
    }
}

#[test]
fn the_canonical_form_matches_rfc8785() {
    use agent_payments_verifier::jcs;
    assert_eq!(jcs(&serde_json::json!({"b": 1, "a": [true, null, "é"]})), "{\"a\":[true,null,\"é\"],\"b\":1}");
    assert_eq!(jcs(&serde_json::json!({"z": 1, "a": 2})), "{\"a\":2,\"z\":1}");
}
