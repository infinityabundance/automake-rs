// xtask/survival.rs — Tier 1 package survival testing.
//
// Tests automake-rs against real GNU package Makefile.am files.
// Clones packages from git.savannah.gnu.org and runs automake-rs on each.
// Reports pass/fail status. Replaces all Python-based package testing.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const TIER1_PACKAGES: &[(&str, &str)] = &[
    ("hello", "https://git.savannah.gnu.org/git/hello.git"),
    ("grep", "https://git.savannah.gnu.org/git/grep.git"),
    ("sed", "https://git.savannah.gnu.org/git/sed.git"),
    ("make", "https://git.savannah.gnu.org/git/make.git"),
    ("gawk", "https://git.savannah.gnu.org/git/gawk.git"),
    (
        "diffutils",
        "https://git.savannah.gnu.org/git/diffutils.git",
    ),
    ("gzip", "https://git.savannah.gnu.org/git/gzip.git"),
    ("tar", "https://git.savannah.gnu.org/git/tar.git"),
    ("bison", "https://git.savannah.gnu.org/git/bison.git"),
    ("flex", "https://github.com/westes/flex.git"),
    (
        "findutils",
        "https://git.savannah.gnu.org/git/findutils.git",
    ),
    (
        "coreutils",
        "https://git.savannah.gnu.org/git/coreutils.git",
    ),
    ("wget", "https://git.savannah.gnu.org/git/wget.git"),
    ("patch", "https://git.savannah.gnu.org/git/patch.git"),
    ("texinfo", "https://git.savannah.gnu.org/git/texinfo.git"),
    ("libtool", "https://git.savannah.gnu.org/git/libtool.git"),
    ("autoconf", "https://git.savannah.gnu.org/git/autoconf.git"),
    ("readline", "https://git.savannah.gnu.org/git/readline.git"),
];

const TIER2_PACKAGES: &[(&str, &str)] = &[
    ("zlib", "https://github.com/madler/zlib.git"),
    ("libpng", "https://github.com/pnggroup/libpng.git"),
    ("curl", "https://github.com/curl/curl.git"),
    ("openssl", "https://github.com/openssl/openssl.git"),
    ("sqlite", "https://github.com/sqlite/sqlite.git"),
    ("gettext", "https://git.savannah.gnu.org/git/gettext.git"),
    (
        "pkg-config",
        "https://git.savannah.gnu.org/git/pkg-config.git",
    ),
    ("ncurses", "https://github.com/mirror/ncurses.git"),
    ("bash", "https://git.savannah.gnu.org/git/bash.git"),
    ("gdbm", "https://git.savannah.gnu.org/git/gdbm.git"),
];

const TIER3_PACKAGES: &[(&str, &str)] = &[
    ("binutils", "https://sourceware.org/git/binutils-gdb.git"),
    ("gcc", "https://gcc.gnu.org/git/gcc.git"),
    ("glibc", "https://sourceware.org/git/glibc.git"),
    ("make", "https://git.savannah.gnu.org/git/make.git"),
];

/// Run survival tests against all Tier 1 and Tier 2 packages.
pub fn run() -> Result<(), String> {
    println!("=== automake-rs Tier 1+2 Survival Test ===\n");

    let work_dir = Path::new("/tmp/am-survival");
    fs::create_dir_all(work_dir).map_err(|e| format!("mkdir: {}", e))?;

    // Tier 1
    let mut t1_passed = Vec::new();
    let mut t1_failed = Vec::new();
    let mut t1_skipped = Vec::new();

    println!("--- Tier 1 ---");
    test_packages(
        TIER1_PACKAGES,
        work_dir,
        &mut t1_passed,
        &mut t1_failed,
        &mut t1_skipped,
    );

    println!("\n=== Tier 1 RESULTS ===");
    print_results(&t1_passed, &t1_failed, &t1_skipped, TIER1_PACKAGES.len());

    // Tier 2
    let mut t2_passed = Vec::new();
    let mut t2_failed = Vec::new();
    let mut t2_skipped = Vec::new();

    println!("\n--- Tier 2 ---");
    test_packages(
        TIER2_PACKAGES,
        work_dir,
        &mut t2_passed,
        &mut t2_failed,
        &mut t2_skipped,
    );

    println!("\n=== Tier 2 RESULTS ===");
    print_results(&t2_passed, &t2_failed, &t2_skipped, TIER2_PACKAGES.len());

    // Tier 3
    let mut t3_passed = Vec::new();
    let mut t3_failed = Vec::new();
    let mut t3_skipped = Vec::new();

    println!("\n--- Tier 3 ---");
    test_packages(
        TIER3_PACKAGES,
        work_dir,
        &mut t3_passed,
        &mut t3_failed,
        &mut t3_skipped,
    );

    println!("\n=== Tier 3 RESULTS ===");
    print_results(&t3_passed, &t3_failed, &t3_skipped, TIER3_PACKAGES.len());

    let total = TIER1_PACKAGES.len() + TIER2_PACKAGES.len() + TIER3_PACKAGES.len();
    let total_passed = t1_passed.len() + t2_passed.len() + t3_passed.len();
    println!(
        "\n=== OVERALL: {}/{} packages passed ===",
        total_passed, total
    );

    Ok(())
}

fn test_packages(
    packages: &[(&str, &str)],
    work_dir: &Path,
    passed: &mut Vec<String>,
    failed: &mut Vec<String>,
    skipped: &mut Vec<String>,
) {
    for (name, url) in packages {
        println!("=== {} ===", name);
        let pkg_dir = work_dir.join(name);

        // Clone if needed
        if !pkg_dir.join("Makefile.am").exists() {
            if pkg_dir.exists() {
                let _ = fs::remove_dir_all(&pkg_dir);
            }
            let status = Command::new("git")
                .args(["clone", "--depth=1", url, name])
                .current_dir(work_dir)
                .status()
                .map_err(|e| format!("git clone {}: {}", name, e));

            let status = match status {
                Ok(s) => s,
                Err(e) => {
                    println!("  SKIP: {}", e);
                    skipped.push(name.to_string());
                    continue;
                }
            };
            if !status.success() {
                println!("  SKIP: clone failed");
                skipped.push(name.to_string());
                continue;
            }
        }

        // Run bootstrap if needed (autoreconf -fi)
        if !pkg_dir.join("configure").exists() && pkg_dir.join("configure.ac").exists() {
            println!("  Bootstrapping (autoreconf -fi)...");
            let _ = Command::new("autoreconf")
                .args(["-fi"])
                .current_dir(&pkg_dir)
                .output();
        }

        // Find Makefile.am
        let makefile = find_makefile_am(&pkg_dir);
        match makefile {
            Some(am_path) => {
                println!("  Makefile.am: {} lines", count_lines(&am_path));

                // Run automake-rs
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
                        am_path.to_str().unwrap_or("Makefile.am"),
                    ])
                    .output()
                    .map_err(|e| format!("cargo run: {}", e));

                let output = match output {
                    Ok(o) => o,
                    Err(e) => {
                        println!("  FAIL: {}", e);
                        failed.push(name.to_string());
                        continue;
                    }
                };

                let exit = output.status.code().unwrap_or(-1);
                let stderr = String::from_utf8_lossy(&output.stderr);

                if exit == 0 {
                    if stderr.contains("error") {
                        println!("  PASS (exit 0, warnings)");
                    } else {
                        println!("  PASS");
                    }
                    passed.push(name.to_string());
                } else {
                    println!("  FAIL: exit {}", exit);
                    for line in stderr.lines().take(5) {
                        println!("    {}", line);
                    }
                    failed.push(name.to_string());
                }
            }
            None => {
                println!("  SKIP: no Makefile.am or Makefile.in found");
                skipped.push(name.to_string());
            }
        }
    }
}

fn print_results(passed: &[String], failed: &[String], skipped: &[String], total: usize) {
    println!("PASSED:  {} — {:?}", passed.len(), passed.join(", "));
    println!("FAILED:  {} — {:?}", failed.len(), failed.join(", "));
    println!("SKIPPED: {} — {:?}", skipped.len(), skipped.join(", "));
    println!("Total: {}/{} passed", passed.len(), total);
}

/// Find the main Makefile.am in a package directory, falling back to Makefile.in.
fn find_makefile_am(dir: &Path) -> Option<PathBuf> {
    // Check root Makefile.am first
    let root_am = dir.join("Makefile.am");
    if root_am.exists() {
        return Some(root_am);
    }

    // Search subdirectories, excluding doc/po/tests
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name == "doc" || name == "po" || name == "tests" || name == "gnulib" {
                    continue;
                }
                let sub_am = path.join("Makefile.am");
                if sub_am.exists() {
                    return Some(sub_am);
                }
            }
        }
    }

    // Fallback: if no Makefile.am, try Makefile.in (hand-maintained projects like glibc)
    let root_in = dir.join("Makefile.in");
    if root_in.exists() && dir.join("configure.ac").exists() {
        return Some(root_in);
    }
    None
}

fn count_lines(path: &Path) -> usize {
    fs::read_to_string(path)
        .map(|c| c.lines().count())
        .unwrap_or(0)
}
