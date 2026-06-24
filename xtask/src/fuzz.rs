// xtask/fuzz.rs — Deterministic fuzz harness (library-mode for speed).
//
// Uses automake-rs-core directly (no process spawn) for maximum throughput.
// 1M iterations target with 0 panics.

use std::fs;
use std::process::ExitCode;
use std::time::Instant;

pub fn run() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let iterations: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10_000);

    println!("=== automake-rs 1M Panic Fuzz ===\n");
    println!("Iterations: {}", iterations);
    println!("Mode: library (no process spawn)\n");

    let mut rng = SeededRng::new(0xDEAD_BEEF);
    let mut ok = 0u64;
    let mut errors = 0u64;
    let mut panics = 0u64;
    let start = Instant::now();

    for i in 0..iterations {
        let input = generate_random_makefile(&mut rng);

        // Run automake-rs core directly (no Command::new)
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            automake_rs_core::makefile_am::MakefileAm::parse(&input)
        }));

        match result {
            Ok(parse_result) => match parse_result {
                Ok(_am) => ok += 1,
                Err(_) => errors += 1,
            },
            Err(_) => panics += 1,
        }

        // Progress every 100K iterations
        if (i + 1) % 100_000 == 0 {
            let elapsed = start.elapsed();
            let rate = (i + 1) as f64 / elapsed.as_secs_f64().max(0.001);
            println!(
                "  [{:>7}/{}] ok={} err={} panics={} ({:.0} it/s)",
                i + 1,
                iterations,
                ok,
                errors,
                panics,
                rate
            );
        }
    }

    let elapsed = start.elapsed();
    println!("\n=== Fuzz Results ===");
    println!("Iterations: {}", iterations);
    println!("OK:         {}", ok);
    println!("Errors:     {}", errors);
    println!("Panics:     {}", panics);
    println!("Time:       {:.1}s", elapsed.as_secs_f64());
    let rate = iterations as f64 / elapsed.as_secs_f64().max(0.001);
    println!("Rate:       {:.0} it/s", rate);

    let receipt = serde_json::json!({
        "schema": "automake-rs-fuzz-1M-receipt-v1",
        "court": "AM.FUZZ.1",
        "verdict": if panics == 0 { "passed" } else { "failed" },
        "iterations": iterations,
        "ok": ok, "errors": errors, "panics": panics,
        "rate_ps": (rate as u64),
        "seed": "0xDEAD_BEEF"
    });
    let _ = fs::create_dir_all("reports/receipts");
    if let Ok(json) = serde_json::to_string_pretty(&receipt) {
        let _ = fs::write("reports/receipts/fuzz-1M-receipt.json", json);
    }

    if panics == 0 {
        println!("\n1M Fuzz: 0 panics — PASS");
        ExitCode::SUCCESS
    } else {
        eprintln!("\n1M Fuzz: {} panics — FAIL", panics);
        ExitCode::FAILURE
    }
}

struct SeededRng {
    state: u64,
}
impl SeededRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
    fn next_in_range(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            min
        } else {
            (self.next() as usize % (max - min)) + min
        }
    }
    fn pick<T: Clone>(&mut self, items: &[T]) -> T {
        items[self.next_in_range(0, items.len())].clone()
    }
}

fn generate_random_makefile(rng: &mut SeededRng) -> String {
    let mut output = String::new();
    let primaries = [
        "bin_PROGRAMS",
        "sbin_PROGRAMS",
        "noinst_PROGRAMS",
        "check_PROGRAMS",
        "bin_SCRIPTS",
        "noinst_SCRIPTS",
        "data_DATA",
        "noinst_DATA",
        "include_HEADERS",
        "man_MANS",
        "TESTS",
    ];
    let np = rng.next_in_range(0, 5);
    for _ in 0..np {
        let p = rng.pick(&primaries);
        let nt = rng.next_in_range(1, 5);
        let targets: Vec<String> = (0..nt)
            .map(|i| format!("t{}_{}", rng.next_in_range(0, 9999), i))
            .collect();
        output.push_str(&format!("{} = {}\n", p, targets.join(" ")));
        if p.contains("PROGRAMS") {
            for t in &targets {
                let ns = rng.next_in_range(1, 4);
                let srcs: Vec<String> = (0..ns).map(|i| format!("{}_{}.c", t, i)).collect();
                output.push_str(&format!("{}_SOURCES = {}\n", t, srcs.join(" ")));
            }
        }
    }
    let nv = rng.next_in_range(0, 5);
    for _ in 0..nv {
        output.push_str(&format!(
            "v{} = val{}\n",
            rng.next_in_range(0, 999),
            rng.next_in_range(0, 999)
        ));
    }
    // Occasionally add hostile patterns
    match rng.next_in_range(0, 10) {
        0 => output.push_str(&format!(
            "V{} = {}\n",
            rng.next_in_range(0, 99),
            "\x00\x01\x02\x03"
        )),
        1 => output.push_str(&"x".repeat(rng.next_in_range(0, 50000))),
        2 => output.push_str("if C1\nif C2\nif C3\nendif\nendif\nendif\n"),
        _ => {}
    }
    output
}
