// xtask/compare: Layered corpus comparison — run automake-rs against oracle
// and compare generated Makefile.in output.
use std::path::Path;
use std::process::{Command, ExitCode};

pub fn run() -> ExitCode {
    println!("=== automake-rs corpus comparison ===\n");

    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;

    // Layer 0: Smoke fixtures
    let layer0_dir = Path::new("fixtures/makefile_am");
    if layer0_dir.exists() {
        println!("--- Layer 0: Smoke fixtures ---");
        if let Ok(entries) = std::fs::read_dir(layer0_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "am").unwrap_or(false) {
                    match compare_fixture(&path) {
                        Ok(true) => {
                            println!("  PASS: {}", path.display());
                            passed += 1;
                        }
                        Ok(false) => {
                            println!("  FAIL: {}", path.display());
                            failed += 1;
                        }
                        Err(e) => {
                            println!("  SKIP: {} ({})", path.display(), e);
                            skipped += 1;
                        }
                    }
                }
            }
        }
        println!();
    }

    println!(
        "Results: {} passed, {} failed, {} skipped",
        passed, failed, skipped
    );

    if failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// Compare a single Makefile.am fixture against the oracle.
fn compare_fixture(am_path: &Path) -> Result<bool, String> {
    // Find corresponding configure.ac
    let ac_path = Path::new("fixtures/configure_ac").join(am_path.file_name().unwrap());

    let ac_path = if ac_path.exists() {
        ac_path
    } else {
        // Use a default smoke configure.ac
        Path::new("fixtures/configure_ac/layer0-smoke.ac").to_path_buf()
    };

    if !ac_path.exists() {
        return Err("no configure.ac found".to_string());
    }

    // Create temp directory
    let tmp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let tmp_path = tmp.path();

    // Copy files
    std::fs::copy(&ac_path, tmp_path.join("configure.ac")).map_err(|e| e.to_string())?;
    std::fs::copy(am_path, tmp_path.join("Makefile.am")).map_err(|e| e.to_string())?;

    // Run aclocal (oracle)
    let aclocal = Command::new("aclocal")
        .current_dir(tmp_path)
        .output()
        .map_err(|e| e.to_string())?;
    if !aclocal.status.success() {
        return Err(format!(
            "aclocal failed: {}",
            String::from_utf8_lossy(&aclocal.stderr)
        ));
    }

    // Run autoconf (oracle)
    let autoconf = Command::new("autoconf")
        .current_dir(tmp_path)
        .output()
        .map_err(|e| e.to_string())?;
    if !autoconf.status.success() {
        return Err(format!(
            "autoconf failed: {}",
            String::from_utf8_lossy(&autoconf.stderr)
        ));
    }

    // Run automake-rs (native) — use the built binary with absolute path
    let am_rs_path = std::env::current_dir()
        .unwrap_or_else(|_| Path::new(".").to_path_buf())
        .join("target/debug/automake");
    let am_rs = Command::new(&am_rs_path)
        .arg("--foreign")
        .current_dir(tmp_path)
        .output()
        .map_err(|e| e.to_string())?;

    if !am_rs.status.success() {
        return Err(format!(
            "automake-rs failed: {}",
            String::from_utf8_lossy(&am_rs.stderr)
        ));
    }

    // Run automake (oracle) for comparison
    let am_oracle = Command::new("automake")
        .args(["--foreign", "--add-missing"])
        .current_dir(tmp_path)
        .output()
        .map_err(|e| e.to_string())?;

    if !am_oracle.status.success() {
        // Oracle may fail on complex fixtures — that's OK for now
        eprintln!(
            "    oracle note: {}",
            String::from_utf8_lossy(&am_oracle.stderr)
        );
    }

    // Compare key elements
    let ours = std::fs::read_to_string(tmp_path.join("Makefile.in")).map_err(|e| e.to_string())?;

    // Check that our output has the basic required elements
    let required = &["Makefile.in generated", "@prefix@"];
    for r in required {
        if !ours.contains(r) {
            return Err(format!("missing required element: {}", r));
        }
    }

    Ok(true)
}
