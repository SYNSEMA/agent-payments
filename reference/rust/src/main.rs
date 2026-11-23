//! agent-payments — the reference verifier's command line.
//!
//!   apv <bundle.json>            verify one bundle
//!   apv <directory>              verify every .json in it (the directory's
//!                                name decides the expectation: "valid" must
//!                                verify, "invalid" must be rejected)
//!
//! Exit code: 0 = VERIFIED (or every expectation held), 1 = REJECTED, 2 = the
//! file could not be read.

use agent_payments_verifier::{verify_bundle, DEFAULT_ISSUER};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("usage: apv <bundle.json | directory>");
        return ExitCode::from(2);
    }
    let target = &args[0];
    let path = Path::new(target);
    if path.is_dir() {
        let expect_verified = path
            .file_name()
            .map(|n| n.to_string_lossy() == "valid")
            .unwrap_or(false);
        let mut entries: Vec<_> = std::fs::read_dir(path)
            .expect("cannot read the directory")
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|x| x == "json").unwrap_or(false))
            .collect();
        entries.sort();
        if entries.is_empty() {
            eprintln!("no .json files in {}", target);
            return ExitCode::from(2);
        }
        let mut bad = 0;
        for f in &entries {
            let name = f.file_name().unwrap().to_string_lossy().to_string();
            let bundle: serde_json::Value = match std::fs::read_to_string(f)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
            {
                Some(v) => v,
                None => {
                    println!("  [XX] {}: the file is not valid JSON", name);
                    bad += 1;
                    continue;
                }
            };
            let verdict = verify_bundle(&bundle, DEFAULT_ISSUER);
            let good = verdict.ok == expect_verified;
            if !good {
                bad += 1;
            }
            println!(
                "  [{}] {} → {}{}",
                if good { "ok" } else { "XX" },
                name,
                verdict.summary,
                if expect_verified { "" } else { " (expected REJECTED)" }
            );
            if !good {
                for c in verdict.failures() {
                    println!("        {}: {}", c.name, c.detail);
                }
            }
        }
        println!();
        println!("{} file(s), {} as expected, {} not.", entries.len(), entries.len() - bad, bad);
        if bad == 0 {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    } else {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("cannot read {}: {}", target, e);
                return ExitCode::from(2);
            }
        };
        let bundle: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("the file is not valid JSON: {}", e);
                return ExitCode::from(2);
            }
        };
        let verdict = verify_bundle(&bundle, DEFAULT_ISSUER);
        println!("agent-payments · offline verification (Rust)");
        println!("bundle   → {}", target);
        println!("issuer   → {}", DEFAULT_ISSUER);
        println!();
        for c in &verdict.checks {
            println!("  [{}] {}", if c.ok { "ok" } else { "XX" }, c.name);
            println!("        {}", c.detail);
        }
        println!();
        println!("─── {} ───", verdict.summary);
        for c in verdict.failures() {
            println!("  {}: {}", c.name, c.detail);
        }
        if verdict.ok {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}
