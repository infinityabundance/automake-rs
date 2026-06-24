// xtask/bench.rs — Performance baseline: automake-rs vs GNU Automake.
//
// NC.PERM.5: Performance parity not claimed, but baseline is measured.
// Runs both tools on the same inputs and reports wall-clock times.

use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};
use std::time::Instant;

/// Run performance baseline.
pub fn run() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let iterations: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);

    println!("=== automake-rs Performance Baseline ===\n");
    println!("Iterations: {}", iterations);
    println!("NC.PERM.5: Performance parity not claimed.\n");

    let tmp = tempfile::tempdir().unwrap();
    let tmp_path = tmp.path();

    // Create a non-trivial Makefile.am
    let makefile_am = "bin_PROGRAMS = hello greet\nhello_SOURCES = hello.c util.c\ngreet_SOURCES = greet.c\nbin_SCRIPTS = myscript\nnoinst_DATA = readme.txt\nEXTRA_DIST = extra.txt\n";
    let configure_ac = "AC_INIT([bench], [1.0])\nAM_INIT_AUTOMAKE([foreign])\nAC_CONFIG_FILES([Makefile])\nAC_OUTPUT\n";

    fs::write(tmp_path.join("Makefile.am"), makefile_am).unwrap();
    fs::write(tmp_path.join("configure.ac"), configure_ac).unwrap();

    // Setup aclocal.m4 (needed by oracle)
    let _ = Command::new("aclocal").current_dir(tmp_path).output();

    // Benchmark oracle
    let mut oracle_times = Vec::new();
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = Command::new("automake")
            .args(["--foreign", "Makefile.am"])
            .current_dir(tmp_path)
            .output();
        oracle_times.push(start.elapsed());
    }

    // Benchmark automake-rs
    let mut our_times = Vec::new();
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = Command::new("cargo")
            .args([
                "run",
                "-q",
                "-p",
                "automake-rs-cli",
                "--bin",
                "automake",
                "--",
                "--foreign",
                tmp_path.join("Makefile.am").to_str().unwrap(),
            ])
            .output();
        our_times.push(start.elapsed());
    }

    // Calculate stats
    let oracle_avg = oracle_times.iter().sum::<std::time::Duration>() / iterations as u32;
    let our_avg = our_times.iter().sum::<std::time::Duration>() / iterations as u32;
    let oracle_min = oracle_times.iter().min().unwrap();
    let our_min = our_times.iter().min().unwrap();

    println!("=== Results ===");
    println!("GNU automake:  avg={:?}  min={:?}", oracle_avg, oracle_min);
    println!("automake-rs:   avg={:?}  min={:?}", our_avg, our_min);

    let ratio = our_avg.as_secs_f64() / oracle_avg.as_secs_f64().max(0.001);
    println!("Ratio (ours/oracle): {:.2}x", ratio);

    if ratio < 3.0 {
        println!("\nPerformance is within 3x of oracle (acceptable for unoptimized debug build).");
    }

    // Save receipt
    let receipt = serde_json::json!({
        "schema": "automake-rs-bench-receipt-v1",
        "court": "AM.PERF.1",
        "verdict": "measured",
        "iterations": iterations,
        "oracle_avg_us": oracle_avg.as_micros(),
        "our_avg_us": our_avg.as_micros(),
        "ratio": (ratio * 100.0).round() / 100.0,
        "non_claim": "NC.PERM.5: Performance parity not claimed — measured for reference only."
    });

    let receipt_dir = Path::new("reports/receipts");
    let _ = fs::create_dir_all(receipt_dir);
    if let Ok(json) = serde_json::to_string_pretty(&receipt) {
        let _ = fs::write(receipt_dir.join("bench-receipt.json"), json);
    }

    ExitCode::SUCCESS
}
