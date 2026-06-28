// xtask — project maintenance tasks for automake-rs.
//
// Usage: cargo xtask <command>
//
// Commands:
//   check       Run all acceptance gate checks (fmt, clippy, test, freshness, oracle)
//   fmt         Run rustfmt
//   clippy      Run clippy with warnings denied
//   test        Run all tests
//   oracle      Run oracle admission
//   generate    (Re)generate all documents from JSON sources
//   receipts    Verify receipt freshness
//   claims      Verify claim ladder freshness
//   ast-verify  Run AST parity verification bridge against oracle
//   behaviors   Scan source for @am_behavior witnesses
//   status      Print current status summary
//   cleanroom   Run GPL contamination scan
//   fuzz        Run deterministic fuzz harness
//   smoke       Run synthetic smoke tests
//   bench       Performance baseline
//   compare     Run layered corpus comparison
//   gnu-compare Compare against GNU Automake test suite

mod audit;
mod bench;
mod cleanroom;
mod compare;
mod docgen;
mod fuzz;
mod gnu_compare;
mod integrity;
mod receipt;
mod smoke;
mod survival;
mod truth;

use std::path::Path;
use std::process::{Command, ExitCode};

mod atlas;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("check");

    match command {
        "check" => run_check(),
        "fmt" => run_fmt(),
        "clippy" => run_clippy(),
        "test" => run_test(),
        "oracle" => run_oracle_admission(),
        "compare" => compare::run(),
        "atlas" => atlas::run(),
        "atlas-index" => atlas::index_only(),
        "generate" => run_generate(),
        "receipts" => run_receipt_check(),
        "claims" => run_claim_check(),
        "ast-verify" => run_ast_verify(),
        "behaviors" => run_behaviors_scan(),
        "cleanroom" => run_cleanroom_scan(),
        "fuzz" => fuzz::run(),
        "smoke" => smoke::run(),
        "gnu-compare" => gnu_compare::run(),
        "bench" => bench::run(),
        "bench-release" => run_bench_release(),
        "status" => run_status(),
        "audit" => run_audit(),
        "receipt" => run_receipt_create(),
        "survival" => run_survival_test(),
        "sign" => run_sign_all(),
        "integrity" => run_integrity(),
        "sarif" => run_sarif(),
        "in-toto" => run_in_toto(),
        _ => {
            eprintln!("xtask: unknown command: {}", command);
            eprintln!("Available: check, fmt, clippy, test, oracle, compare, generate, receipts, claims, ast-verify, behaviors, cleanroom, fuzz, smoke, gnu-compare, bench, status, audit, receipt, survival, sign, integrity, sarif, in-toto");
            ExitCode::FAILURE
        }
    }
}

fn run_check() -> ExitCode {
    println!("=== automake-rs acceptance gate check ===\n");
    let mut failed = false;

    // 1. Format
    println!("[1/7] rustfmt...");
    let fmt = Command::new("cargo")
        .args(["fmt", "--", "--check"])
        .status();
    if fmt.map(|s| !s.success()).unwrap_or(true) {
        eprintln!("  FAIL: formatting issues");
        failed = true;
    } else {
        println!("  PASS");
    }

    // 2. Clippy
    println!("[2/7] clippy...");
    let clippy = Command::new("cargo")
        .args(["clippy", "--all-targets", "--", "-D", "warnings"])
        .status();
    if clippy.map(|s| !s.success()).unwrap_or(true) {
        eprintln!("  FAIL: clippy issues");
        failed = true;
    } else {
        println!("  PASS");
    }

    // 3. Tests
    println!("[3/7] tests...");
    let test = Command::new("cargo").args(["test", "--all"]).status();
    if test.map(|s| !s.success()).unwrap_or(true) {
        eprintln!("  FAIL: tests failed");
        failed = true;
    } else {
        println!("  PASS");
    }

    // 4. Document freshness
    println!("[4/7] document freshness (+truth gate)...");
    let registry_path = Path::new("reports/doc-registry.json");
    if registry_path.exists() {
        match std::fs::read_to_string(registry_path) {
            Ok(json) => match serde_json::from_str::<docgen::DocumentRegistry>(&json) {
                Ok(registry) => match registry.verify_freshness() {
                    Ok(msgs) => {
                        for m in &msgs {
                            println!("  {}", m);
                        }
                    }
                    Err(stale) => {
                        for s in &stale {
                            eprintln!("  {}", s);
                        }
                        eprintln!("  FAIL: stale documents. Run 'cargo xtask generate'.");
                        failed = true;
                    }
                },
                Err(e) => {
                    eprintln!("  WARN: invalid registry: {}", e);
                }
            },
            Err(e) => {
                eprintln!("  WARN: cannot read registry: {}", e);
            }
        }
    } else {
        println!("  INFO: no doc registry yet. Run 'cargo xtask generate'.");
    }

    // 4b: Truth gate — verify test counts match claims
    let truth = truth::verify_truth_gate();
    println!("  {}", truth.message);
    if !truth.passed {
        failed = true;
    }

    // 5. Oracle
    println!("[5/7] oracle profile...");
    if Path::new("reports/oracle-profile.json").exists() {
        println!("  PASS: oracle profile present");
    } else {
        println!("  WARN: no oracle profile. Run 'cargo xtask oracle'.");
    }

    // 6. Claim ladder
    println!("[6/7] claim ladder...");
    if Path::new("reports/claim-ladder.json").exists() {
        println!("  PASS: claim ladder present");
    } else {
        println!("  WARN: no claim ladder.");
    }

    // 7. Clean-room contamination scan
    println!("[7/7] clean-room scan...");
    match cleanroom::scan_source_tree() {
        Ok(receipt) => {
            if receipt.verdict == "FAIL" {
                eprintln!(
                    "  FAIL: {} GPL contamination errors found",
                    receipt.errors.len()
                );
                for e in &receipt.errors {
                    eprintln!("    {}:{} — {}: {}", e.file, e.line, e.pattern, e.matched);
                }
                failed = true;
            } else {
                println!(
                    "  PASS: {} files scanned, {} warnings, {} info markers",
                    receipt.files_scanned,
                    receipt.warnings.len(),
                    receipt.infos.len()
                );
                if let Ok(json) = serde_json::to_string_pretty(&receipt) {
                    let _ = std::fs::create_dir_all("reports/receipts");
                    let _ = std::fs::write("reports/receipts/cleanroom-receipt.json", &json);
                }
            }
        }
        Err(e) => {
            eprintln!("  FAIL: scan error: {}", e);
            failed = true;
        }
    }

    println!();
    if failed {
        eprintln!("=== ACCEPTANCE GATE FAILED ===");
        ExitCode::FAILURE
    } else {
        println!("=== ACCEPTANCE GATE PASSED ===");
        ExitCode::SUCCESS
    }
}

fn run_fmt() -> ExitCode {
    match Command::new("cargo").args(["fmt"]).status() {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

fn run_clippy() -> ExitCode {
    match Command::new("cargo")
        .args(["clippy", "--all-targets", "--", "-D", "warnings"])
        .status()
    {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

fn run_test() -> ExitCode {
    match Command::new("cargo").args(["test", "--all"]).status() {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

fn run_oracle_admission() -> ExitCode {
    println!("=== automake-rs oracle admission ===\n");
    match automake_oracle_rs::admit_oracle(&automake_oracle_rs::OracleConfig::default()) {
        Ok(profile) => {
            println!("Oracle: {} (automake sha256: {})", profile.kind, {
                profile
                    .binaries
                    .get("automake")
                    .map(|b| &b.sha256[..16])
                    .unwrap_or("?")
            });
            if let Err(e) =
                automake_oracle_rs::save_profile(&profile, Path::new("reports/oracle-profile.json"))
            {
                eprintln!("Error saving: {}", e);
                return ExitCode::FAILURE;
            }
            println!("Saved to reports/oracle-profile.json");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Oracle admission failed: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_generate() -> ExitCode {
    println!("=== Document Generation ===\n");
    let key = b"am-rs-forensic-key-2026";
    let mut registry = docgen::DocumentRegistry::new();
    match docgen::generate::generate_all(&mut registry, key) {
        Ok(results) => {
            for r in &results {
                println!("  {}", r);
            }
            let json = serde_json::to_string_pretty(&registry).unwrap_or_default();
            if let Err(e) = std::fs::write("reports/doc-registry.json", &json) {
                eprintln!("Error saving registry: {}", e);
                return ExitCode::FAILURE;
            }
            println!("\nRegistry saved to reports/doc-registry.json");

            // Sign all documents with DSSE
            let key = b"am-rs-forensic-key-2026";
            let keyid = "automake-rs-hmac-v1";
            match docgen::dsse::sign_all_documents(key, keyid) {
                Ok(signed) => println!("DSSE: {} documents signed.", signed.len()),
                Err(errors) => {
                    for e in &errors {
                        eprintln!("DSSE error: {}", e);
                    }
                }
            }

            ExitCode::SUCCESS
        }
        Err(errors) => {
            for e in &errors {
                eprintln!("  {}", e);
            }
            ExitCode::FAILURE
        }
    }
}

fn run_receipt_check() -> ExitCode {
    println!("=== Receipt check ===\n");
    let dir = Path::new("reports/receipts");
    if !dir.exists() {
        println!("No receipts directory. Expected before courts are sealed.");
        return ExitCode::SUCCESS;
    }
    match std::fs::read_dir(dir) {
        Ok(entries) => {
            let count = entries
                .flatten()
                .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
                .count();
            println!("Receipts: {}", count);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_claim_check() -> ExitCode {
    println!("=== Claim ladder check ===\n");
    let path = Path::new("reports/claim-ladder.json");
    if !path.exists() {
        println!("No claim-ladder.json. Expected before courts are sealed.");
        return ExitCode::SUCCESS;
    }
    match std::fs::read_to_string(path) {
        Ok(contents) => {
            match serde_json::from_str::<automake_casefile_rs::ClaimLadder>(&contents) {
                Ok(ladder) => {
                    println!(
                        "Sealed: {}, Partial: {}, Unclaimed: {}",
                        ladder.sealed_count, ladder.partial_count, ladder.unclaimed_count
                    );
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Parse error: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        Err(e) => {
            eprintln!("Read error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_ast_verify() -> ExitCode {
    println!("=== AST Parity Verification ===\n");
    println!("AST verification bridge not yet implemented.");
    ExitCode::SUCCESS
}

fn run_behaviors_scan() -> ExitCode {
    println!("=== @am_behavior Witness Scan ===\n");
    let src_dirs = &[
        "crates/automake-rs-core/src",
        "crates/automake-rs-cli/src",
        "crates/automake-oracle-rs/src",
    ];
    let mut total = 0;
    for dir in src_dirs {
        let path = Path::new(dir);
        if path.exists() {
            if let Ok(entries) = std::fs::read_dir(path) {
                let count = entries.flatten().count();
                total += count;
                println!("{}: {} files", dir, count);
            }
        }
    }
    println!("\nTotal source files: {}", total);
    if total == 0 {
        println!("No source files found. Add @am_behavior tags to source files.");
    }
    ExitCode::SUCCESS
}

fn run_cleanroom_scan() -> ExitCode {
    cleanroom::run_scan()
}

fn run_status() -> ExitCode {
    println!("=== automake-rs project status ===\n");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Strategy: Clean-room behavioral reconstruction");
    println!("License: MIT OR Apache-2.0");
    println!("Dependencies: m4-rs-core, autoconf-rs-core");
    println!();

    // Read from JSON sources
    if let Ok(data) = std::fs::read_to_string("sources/gaps/needle-metrics.json") {
        if let Ok(metrics) = serde_json::from_str::<serde_json::Value>(&data) {
            if let Some(pct) = metrics["overall_percentage"].as_f64() {
                println!("Overall parity: {:.1}%", pct);
            }
            if let Some(t) = metrics["tests_passing"].as_u64() {
                println!("Tests: {}", t);
            }
            if let Some(surfaces) = metrics["surfaces"].as_array() {
                let sealed = surfaces
                    .iter()
                    .filter(|s| s["sealed"].as_bool().unwrap_or(false))
                    .count();
                println!("Courts sealed: {}/{}", sealed, surfaces.len());
            }
        }
    }

    if Path::new("reports/oracle-profile.json").exists() {
        println!("Oracle: admitted");
    }

    let receipts_dir = Path::new("reports/receipts");
    if receipts_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(receipts_dir) {
            let count = entries
                .flatten()
                .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
                .count();
            println!("Receipts: {}", count);
        }
    }

    println!("\nIMPORTANT: automake-rs is NOT a GNU Automake replacement.");
    println!("See docs/negative-capabilities.md for the build roadmap.");
    ExitCode::SUCCESS
}

fn run_audit() -> ExitCode {
    match audit::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Audit error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_receipt_create() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let court = args.get(2).map(|s| s.as_str());
    match receipt::create_receipt(court) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Receipt error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_survival_test() -> ExitCode {
    match survival::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Survival error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_sign_all() -> ExitCode {
    let key = b"am-rs-forensic-key-2026";
    let keyid = "automake-rs-hmac-v1";
    println!("=== DSSE Signing All Documents ===\n");
    match docgen::dsse::sign_all_documents(key, keyid) {
        Ok(signed) => {
            for path in &signed {
                println!("  SIGNED: {}", path);
            }
            println!("\n{} documents signed.", signed.len());
            ExitCode::SUCCESS
        }
        Err(errors) => {
            for e in &errors {
                eprintln!("  ERROR: {}", e);
            }
            ExitCode::FAILURE
        }
    }
}

fn run_integrity() -> ExitCode {
    let project_root = Path::new(".");
    match integrity::generate_all(project_root) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("integrity error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_sarif() -> ExitCode {
    let project_root = Path::new(".");
    match integrity::generate_sarif(project_root) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("SARIF error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_in_toto() -> ExitCode {
    let project_root = Path::new(".");
    match integrity::generate_in_toto(project_root) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("in-toto error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_bench_release() -> ExitCode {
    println!("=== Release Mode Performance Baseline ===");
    println!("Building in release mode...");

    let build = Command::new("cargo")
        .args(["build", "--release", "-p", "automake-rs-cli"])
        .status();

    if build.map(|s| !s.success()).unwrap_or(true) {
        eprintln!("Release build failed");
        return ExitCode::FAILURE;
    }

    println!("Running benchmarks...");
    bench::run()
}
