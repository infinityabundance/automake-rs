// xtask/smoke.rs — Hostile input smoke test harness.
// Court: AM.HOSTILE.1 — hostile input / no-panic verification.

use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

pub fn run() -> ExitCode {
    println!("=== automake-rs Hostile Input Smoke Tests ===\n");

    let tmp = tempfile::tempdir().unwrap();
    let tmp_path = tmp.path();
    let configure_ac = "AC_INIT([smoke], [1.0])\nAM_INIT_AUTOMAKE([foreign])\nAC_CONFIG_FILES([Makefile])\nAC_OUTPUT\n";
    fs::write(tmp_path.join("configure.ac"), configure_ac).unwrap();

    let long_var_val = format!("VAR = {}\n", "x".repeat(10000));
    let many_prim_val = generate_many_primaries();
    let nested_cond_val = generate_nested_conditionals(20);
    let large_val = generate_large_makefile(500);
    let subdirs_val = format!(
        "SUBDIRS = {}\n",
        (0..100)
            .map(|i| format!("dir{}", i))
            .collect::<Vec<_>>()
            .join(" ")
    );

    let tests: Vec<(&str, String, bool)> = vec![
        ("empty", String::new(), true),
        (
            "comments",
            "# just a comment\n# another comment\n".into(),
            true,
        ),
        ("blanks", "\n\n\n".into(), true),
        ("long_var", long_var_val, true),
        ("many_primaries", many_prim_val, true),
        ("binary", "VAR = hello\x00world\n".into(), true),
        ("nested_cond", nested_cond_val, true),
        ("recursive_var", "A = $(A)\n".into(), true),
        (
            "circular_cond",
            "if COND1\nif COND2\nendif\nendif\n".into(),
            true,
        ),
        ("unmatched_cond", "if COND\nVAR = x\n".into(), true),
        ("bad_primary", "bin_INVALID = foo\n".into(), true),
        (
            "dup_primary",
            "bin_PROGRAMS = a\nbin_PROGRAMS = b\n".into(),
            true,
        ),
        ("large", large_val, true),
        (
            "escaped",
            "VAR = hello\\ world\nFOO = bar\\\\baz\n".into(),
            true,
        ),
        ("unicode", "VAR = hell wörld\n# cömment\n".into(), true),
        ("null_bytes", "VAR = hello\0world\n".into(), true),
        ("many_subdirs", subdirs_val, true),
        (
            "dot_target",
            "bin_PROGRAMS = my.prog\nmy_prog_SOURCES = my.prog.c\n".into(),
            true,
        ),
        ("mixed_ws", "VAR\t =\t value\nFOO  +=  bar\n".into(), true),
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (name, content, expect_ok) in &tests {
        let am_path = tmp_path.join("Makefile.am");
        fs::write(&am_path, content.as_bytes()).unwrap();

        let output = Command::new("cargo")
            .args([
                "run",
                "-q",
                "-p",
                "automake-rs-cli",
                "--bin",
                "automake",
                "--",
                "--foreign",
                am_path.to_str().unwrap(),
            ])
            .output();

        match output {
            Ok(out) => {
                let ok = out.status.success();
                if ok == *expect_ok {
                    passed += 1;
                    println!("  PASS: {}", name);
                } else {
                    failed += 1;
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    println!(
                        "  FAIL: {} (expected_ok={}, got_ok={})",
                        name, expect_ok, ok
                    );
                    println!("    stderr: {}", stderr.lines().next().unwrap_or(""));
                }
            }
            Err(e) => {
                failed += 1;
                println!("  CRASH: {} — {}", name, e);
            }
        }
    }

    println!("\n=== Smoke Results ===");
    println!("Total:   {}", tests.len());
    println!("Passed:  {}", passed);
    println!("Failed:  {}", failed);

    let receipt_dir = Path::new("reports/receipts");
    let _ = fs::create_dir_all(receipt_dir);
    let receipt = serde_json::json!({
        "schema": "automake-rs-smoke-receipt-v1",
        "court": "AM.HOSTILE.1",
        "verdict": if failed == 0 { "passed" } else { "partial" },
        "total": tests.len(),
        "passed": passed,
        "failed": failed
    });
    if let Ok(json) = serde_json::to_string_pretty(&receipt) {
        let _ = fs::write(receipt_dir.join("smoke-receipt.json"), json);
    }

    if failed > 0 {
        eprintln!("\nSmoke tests: {} failures — partial", failed);
        ExitCode::FAILURE
    } else {
        println!("\nSmoke tests: all passed — PASS");
        ExitCode::SUCCESS
    }
}

fn generate_many_primaries() -> String {
    let mut s = String::new();
    for i in 0..50 {
        s.push_str(&format!("bin_PROGRAMS += prog{}\n", i));
        s.push_str(&format!("prog{}_SOURCES = prog{}.c\n", i, i));
    }
    s.push_str("bin_SCRIPTS = s1 s2 s3\n");
    s.push_str("data_DATA = d1 d2\n");
    s.push_str("include_HEADERS = h1.h h2.h\n");
    s
}

fn generate_nested_conditionals(depth: usize) -> String {
    let mut s = String::new();
    for i in 0..depth {
        s.push_str(&format!("if COND{}\n", i));
        s.push_str(&format!("VAR{} = val{}\n", i, i));
    }
    for _i in (0..depth).rev() {
        s.push_str("endif\n");
    }
    s
}

fn generate_large_makefile(lines: usize) -> String {
    let mut s = String::new();
    for i in 0..lines {
        s.push_str(&format!("# line {}\nVAR{} = value{}\n", i, i, i));
    }
    s
}
