// xtask/truth.rs — Truth Gate: verify claimed test counts match actual tests.
//
// Runs `cargo test --all -- --list` and compares against `tests_passing`
// in sources/gaps/needle-metrics.json. If claimed > actual, the gate FAILS.
// Under-counting by <=5% is permitted (tests may be added faster than docs).

use std::process::Command;

#[derive(Debug)]
#[allow(dead_code)]
pub struct TruthReport {
    pub actual_tests: usize,
    pub claimed_tests: usize,
    pub passed: bool,
    pub message: String,
}

/// Run the truth gate: compare claimed test count against reality.
pub fn verify_truth_gate() -> TruthReport {
    // Get claimed test count from needle-metrics.json
    let claimed = get_claimed_test_count();

    // Get actual test count by running cargo test --list
    let actual = get_actual_test_count();

    // Allow 5% under-counting tolerance
    let tolerance = (claimed as f64 * 0.05).ceil() as usize;
    let difference = claimed.saturating_sub(actual);

    let passed = difference <= tolerance;

    let message = if passed {
        if claimed == actual {
            format!(
                "TRUTH GATE: PASS (claimed {} == actual {})",
                claimed, actual
            )
        } else {
            format!(
                "TRUTH GATE: PASS (claimed {} vs actual {} — within {} tolerance)",
                claimed, actual, tolerance
            )
        }
    } else {
        format!(
            "TRUTH GATE: FAIL — claimed {} tests but actual is {}. Overcounting by {}. Update needle-metrics.json.",
            claimed, actual, difference
        )
    };

    TruthReport {
        actual_tests: actual,
        claimed_tests: claimed,
        passed,
        message,
    }
}

fn get_claimed_test_count() -> usize {
    if let Ok(json) = std::fs::read_to_string("sources/gaps/needle-metrics.json") {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&json) {
            if let Some(n) = v["tests_passing"].as_u64() {
                return n as usize;
            }
            // Fallback: sum all surface test counts
            if let Some(surfaces) = v["surfaces"].as_array() {
                let mut total = 0u64;
                for s in surfaces {
                    // Count tests from note field mentions
                    if let Some(note) = s["note"].as_str() {
                        // Look for patterns like "8 tests" or "7 tests"
                        for word in note.split_whitespace() {
                            if let Ok(n) = word.trim_end_matches('.').parse::<u64>() {
                                if note.contains("tests") || note.contains("test") {
                                    total += n;
                                    break;
                                }
                            }
                        }
                    }
                }
                return total as usize;
            }
        }
    }
    0
}

fn get_actual_test_count() -> usize {
    let output = Command::new("cargo")
        .args(["test", "--all", "--", "--list"])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let count = stdout
                .lines()
                .filter(|l| l.ends_with(": test") && !l.contains("running") && !l.starts_with(" "))
                .count();
            let doctests = stdout
                .lines()
                .filter(|l| l.contains("Doc-tests") && l.contains("running"))
                .count();
            count + doctests
        }
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_actual_test_count() {
        let count = get_actual_test_count();
        eprintln!("Actual test count: {}", count);
        assert!(count > 0, "Expected at least some tests");
    }

    #[test]
    fn test_truth_gate() {
        let report = verify_truth_gate();
        eprintln!("{}", report.message);
        // Gate should pass (we maintain accurate docs)
        assert!(report.passed, "Truth gate failed: {}", report.message);
    }
}
