//! `xtask atlas` — build-profiler atlas.
//!
//! Turns every point in the build corpus into a reproducible, versioned recipe: probe results,
//! feature flags, dependency-graph snapshot, the optimal (working) pass pipeline, target settings,
//! known quirks, and verified outputs. A future build reads the recipe — installs the recorded
//! deps, applies the known pipeline + quirks, reproduces the verified outputs — instead of
//! rediscovering everything. The whole thing is GNU-free: only autoreconf-rs / acrs-* are invoked.
//!
//! Usage: `cargo xtask atlas <corpus-list> [out-dir]`
//!   corpus-list : file with one `owner/name` GitHub repo per line.
//!   out-dir     : recipe output dir (default `atlas/recipes`).
//! Tools resolved from env: AUTOCONF_RS, AUTOHEADER_RS, ACLOCAL_RS, AUTOMAKE_RS, and
//! `autoreconf-rs` on PATH (or AUTORECONF_RS).

use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Serialize)]
struct Recipe {
    schema: &'static str,
    repo: String,
    source: Source,
    toolchain: Toolchain,
    target: Target,
    pass_pipeline: Vec<Step>,
    probe_results: BTreeMap<String, u8>,
    feature_flags: FeatureFlags,
    dependencies: Dependencies,
    quirks: Vec<String>,
    outputs: Vec<Output>,
    status: String,
    verified: bool,
    diagnostic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    deep_expansion: Option<DeepExpansion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle: Option<Oracle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    divergence: Option<Divergence>,
    receipt: Receipt,
    environment: Environment,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    suggested_deps: Vec<SuggestedDep>,
}
#[derive(Serialize)]
struct Source {
    url: String,
    git_sha: String,
    snapshot_utc: String,
}
#[derive(Serialize)]
struct Toolchain {
    autoconf_rs: String,
    automake_rs: String,
    m4_rs_core: String,
    gnu_free: bool,
}
#[derive(Serialize)]
struct Target {
    cc: String,
    cflags: String,
    host: String,
}
#[derive(Serialize)]
struct Step {
    step: String,
    tool: String,
    status: String,
}
#[derive(Serialize)]
struct FeatureFlags {
    configure_args: Vec<String>,
}
#[derive(Serialize)]
struct Dependencies {
    pkg_config: Vec<String>,
    system_libs: Vec<String>,
    headers_needed: Vec<String>,
    missing: Vec<String>,
}
#[derive(Serialize)]
struct Output {
    path: String,
    sha256: String,
    kind: String,
}

/// Deep-expansion forensics on the GENERATED configure — the diagnostics that otherwise require
/// hand-spelunking each configure on the VM. Captured per recipe so a bug is readable from the atlas.
#[derive(Serialize, Default)]
struct DeepExpansion {
    configure_lines: usize,
    /// Autoconf/m4 macro calls that survived into the generated shell unexpanded (AC_/AX_/AM_/LT_/
    /// PKG_/AS_/m4_/_AC_…) — each is a missing or partial macro definition leaking its arg list.
    leaked_macros: Vec<LeakedMacro>,
    /// `cat … <<_ACEOF >conftest` openers vs lone `_ACEOF` terminators. A negative imbalance is the
    /// missing-heredoc-opener bug (compile probe emitted as raw shell -> `syntax error near '('`).
    heredoc_openers: usize,
    heredoc_terminators: usize,
    heredoc_imbalance: i64,
    /// Each `./configure: line N: syntax error …` cross-referenced to the offending source line(s).
    syntax_errors: Vec<SyntaxError>,
    /// Malformed `${…}` cache-var references (embedded newline, PID-garbage `$$`, empty `${}`).
    cache_var_anomalies: Vec<String>,
    /// Leftover `@VAR@` substitution placeholders config.status never filled.
    residual_placeholders: Vec<String>,
    /// The last `checking for …` message printed before configure died — names the actual probe ours
    /// broke on (the real divergence), not the cascade line where the shell finally reports a syntax
    /// error. This is the actionable root: "ours died during <this check>".
    failed_during_check: String,
    /// Lines of configure-run output AFTER the last checking message (the crash's immediate fallout).
    failure_tail: Vec<String>,
    /// Conftest archaeology: C preprocessor directives MANGLED by m4 expanding `include`/`ifdef`/
    /// `define` builtins inside conftest source — `#include <x>` -> `# <x>`, `#ifdef Y` -> `# Y`.
    /// Each entry is `line N: <mangled text> (<which directive eaten>)`. A non-empty list means
    /// compile/link probes are silently failing on corrupted conftests (deep autotools bug).
    conftest_corruption: Vec<String>,
    /// Count of intact vs mangled `#include`/directive lines in the generated configure — the headline
    /// corruption ratio for fast triage.
    conftest_directives_intact: usize,
    conftest_directives_mangled: usize,
    /// The actual C conftest programs the macros emitted (the body between `<<_ACEOF >conftest` and
    /// `_ACEOF`) — the deep macro-OUTPUT context. Debugging a compile/link probe needs to see the
    /// exact C source that was compiled (intact or mangled), not infer it.
    conftest_programs: Vec<String>,
}
/// The GNU-autotools oracle outcome for the SAME repo, run right after ours on a git-reset tree.
/// This is the compass: `classification` says whether a failure is OUR bug (real succeeds, we don't —
/// fixable, with the divergence below showing where) or not-standalone (real fails too).
#[derive(Serialize, Default)]
struct Oracle {
    real_autoreconf: String,     // ok | fail
    real_configure: String,      // ok | fail | skipped
    real_make: String,           // ok | fail | skipped
    real_configure_lines: usize,
    real_first_error: String,
    /// BOTH_OK | OURS_BUG_CONFIGURE | OURS_BUG_MAKE | OURS_GEN_FAIL | NOT_STANDALONE |
    /// BOTH_CONFIGURE_FAIL | BOTH_MAKE_FAIL | OURS_BETTER | UNKNOWN
    classification: String,
}
/// Where ours diverges from a real run that got further — only populated for OURS_BUG_* recipes.
#[derive(Serialize, Default)]
struct Divergence {
    stage: String,                         // autoreconf | configure | make
    ours_error: String,                    // ours' first hard error line
    ours_error_context: Vec<String>,       // generated-shell lines around the failure (the actual bug site)
    ours_configure_lines: usize,
    real_configure_lines: usize,
    macros_ours_left_undefined: Vec<String>, // leaked macros = the macros to define/fix
}
#[derive(Serialize)]
struct LeakedMacro {
    name: String,
    line: usize,
    context: String,
}
#[derive(Serialize)]
struct SyntaxError {
    line: usize,
    token: String,
    source: String,
    /// The surrounding generated-shell construct (a few lines either side) — the syntactic context a
    /// human needs to see WHY it broke (the enclosing if/case/heredoc), not just the one error line.
    block: Vec<String>,
}

/// Sealed build-court receipt — makes each recipe auditable: the probe-execution trace (every HAVE_*
/// decision + why), the quirk rules that fired, and a sha256 hash-chain over the recipe's load-bearing
/// fields (toolchain + probe trace + outputs + oracle verdict). court_status is the verdict.
#[derive(Serialize, Default)]
struct Receipt {
    /// sealed (FUNC_OK, matches oracle) | partial (configure cleared, make failed) |
    /// quirk_dependent (needed a quirk) | not_standalone (oracle also fails) | failed
    court_status: String,
    probe_trace: Vec<ProbeStep>,
    quirks_matched: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    quirks_applied: Vec<QuirkApplied>,
    receipt_hash: String,
    schema: &'static str,
}
#[derive(Serialize)]
struct ProbeStep {
    name: String,   // HAVE_FOO_H / func | -llib
    kind: String,   // header | func | lib
    result: String, // yes | no
    reason: String, // ok | header-not-found | symbol-not-found | link-failed | not-recorded
}
/// An auto-applied quirk fix and whether it actually helped (re-run cleared a stage it didn't before).
#[derive(Serialize)]
struct QuirkApplied {
    id: String,
    action: String,  // the GNU-free fix applied (configure flag / mkdir / env)
    verified: bool,  // true = the build got further AFTER applying it
}
/// Build-environment fingerprint for hermeticity: the exact toolchain + paths + env that shaped the
/// build, so a recipe is reproducible across machines and time.
#[derive(Serialize, Default)]
struct Environment {
    cc: String,
    cc_version: String,
    host_triplet: String,
    pkg_config_version: String,
    make_version: String,
    relevant_env: Vec<String>,
}
/// Missing-dep inference: a package that would satisfy a failed header/lib probe.
#[derive(Serialize)]
struct SuggestedDep {
    missing: String, // foo.h | -lfoo
    kind: String,    // header | lib
    package: String, // distro package that provides it
}

fn tool(env_var: &str, default: &str) -> String {
    std::env::var(env_var).unwrap_or_else(|_| default.to_string())
}

/// Run `cmd` (wrapped in coreutils `timeout`) in `dir`, capturing combined stdout+stderr to a FILE.
/// Two layers stop a build from ever hanging the worker:
///   * `timeout -k 10 -s KILL` — SIGKILL at the deadline (unignorable) + a 10s grace.
///   * redirect to a file, NOT a pipe — `.output()` would block reading the stdout pipe until EOF,
///     which orphaned grandchildren (that survive the kill) hold open forever. `.status()` only
///     waits for the direct child (timeout), so leaked children can never wedge us.
fn run_timed(dir: &Path, secs: u32, program: &str, args: &[&str]) -> (bool, String) {
    use std::fs::File;
    let log = dir.join(".atlas_runlog");
    let f = match File::create(&log) {
        Ok(f) => f,
        Err(e) => return (false, e.to_string()),
    };
    let f2 = match f.try_clone() {
        Ok(f) => f,
        Err(e) => return (false, e.to_string()),
    };
    let status = Command::new("timeout")
        .arg("-k")
        .arg("10")
        .arg("-s")
        .arg("KILL")
        .arg(secs.to_string())
        .arg(program)
        .args(args)
        .current_dir(dir)
        .stdin(std::process::Stdio::null())
        .stdout(f)
        .stderr(f2)
        .status();
    let text = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_file(&log);
    match status {
        Ok(s) => (s.success(), text),
        Err(e) => (false, format!("{}\n{}", text, e)),
    }
}

fn first_line(program: &str, arg: &str) -> String {
    Command::new(program)
        .arg(arg)
        .output()
        .ok()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .to_string()
        })
        .unwrap_or_default()
}

fn sha256_16(path: &Path) -> Option<String> {
    let data = std::fs::read(path).ok()?;
    let mut h = Sha256::new();
    h.update(&data);
    Some(format!("{:x}", h.finalize())[..16].to_string())
}

pub fn run() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let list = match args.get(2) {
        Some(p) => p.clone(),
        None => {
            eprintln!("usage: cargo xtask atlas <corpus-list> [out-dir]");
            return ExitCode::from(2);
        }
    };
    let out_dir = PathBuf::from(args.get(3).cloned().unwrap_or_else(|| "atlas/recipes".into()));
    std::fs::create_dir_all(&out_dir).ok();

    let acrs_ac = tool("AUTOCONF_RS", "acrs-autoconf");
    let am = tool("AUTOMAKE_RS", "automake");
    let ars = tool("AUTORECONF_RS", "autoreconf-rs");
    let ac_ver = first_line(&acrs_ac, "--version");
    let am_ver = first_line(&am, "--version");

    let body = match std::fs::read_to_string(&list) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("atlas: cannot read {}: {}", list, e);
            return ExitCode::from(2);
        }
    };

    let mut n = 0;
    let mut ok = 0;
    for repo in body.lines().map(str::trim).filter(|l| !l.is_empty()) {
        n += 1;
        let slug = repo.replace('/', "__");
        let base = std::env::temp_dir().join(format!("atlasx_{}", slug));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).ok();
        let d = base.join("s");

        let mut pipeline = Vec::new();
        let mut status = "CLONE_FAIL".to_string();
        let mut git_sha = String::new();
        let mut diagnostic = String::new();

        let (cloned, _) = run_timed(
            &base,
            120,
            "git",
            &["clone", "--depth", "1", "-q", &format!("https://github.com/{}", repo), d.to_str().unwrap()],
        );
        if cloned {
            git_sha = Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(&d)
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_default();

            let has_ac = d.join("configure.ac").exists() || d.join("configure.in").exists();
            if !has_ac {
                status = "NO_AC".to_string();
            } else {
                let ac_text = std::fs::read_to_string(d.join("configure.ac"))
                    .or_else(|_| std::fs::read_to_string(d.join("configure.in")))
                    .unwrap_or_default();
                let mut quirks = quirks_from_ac(&ac_text, &d);

                status = "CONFIGURE_GEN_FAIL".to_string();
                let (_b, _bl) = run_timed(&d, 150, &ars, &["-fi", "."]);
                pipeline.push(Step {
                    step: "autoreconf".into(),
                    tool: "autoreconf-rs".into(),
                    status: if d.join("configure").exists() { "ok".into() } else { "fail".into() },
                });

                let mut cf_log = String::new();
                let mut mk_log = String::new();
                // ATLAS_SCAN_ONLY=1: stop after generating configure. The deep_expansion ranking
                // (leaked macros, heredoc balance, residual @VAR@) is static analysis of the GENERATED
                // configure — it needs neither a configure-run nor make. Scan-only sweeps the full 1000
                // in a fraction of the time (no per-repo 120s/300s timeouts), giving the ranked backlog
                // fast. The full run (with run+make, for FUNC_OK) stays the default.
                let scan_only = std::env::var("ATLAS_SCAN_ONLY").is_ok();
                if d.join("configure").exists() {
                    if scan_only {
                        status = "CONFIGURE_GENERATED".to_string();
                    } else {
                        status = "CONFIGURE_RUN_FAIL".to_string();
                        let (cfok, cfl) = run_timed(&d, 180, "./configure", &[]);
                        cf_log = cfl;
                        pipeline.push(Step {
                            step: "configure".into(),
                            tool: "./configure".into(),
                            status: if cfok { "ok".into() } else { "fail".into() },
                        });
                        if cfok {
                            status = "MAKE_FAIL".to_string();
                            let (mkok, mkl) = run_timed(&d, 300, "make", &["-j2"]);
                            mk_log = mkl;
                            pipeline.push(Step {
                                step: "make".into(),
                                tool: "make".into(),
                                status: if mkok { "ok".into() } else { "fail".into() },
                            });
                            if mkok {
                                status = "FUNC_OK".to_string();
                                ok += 1;
                            }
                        }
                    }
                }
                // === auto-apply quirks: if configure failed, try GNU-free quirk fixes and re-run ===
                let quirks_matched = match_quirks(&ac_text, &d, &cf_log);
                let mut quirks_applied = Vec::new();
                if !scan_only && status == "CONFIGURE_RUN_FAIL" {
                    let (ap, ns, oki) = auto_apply_quirks(&d, &quirks_matched, &status);
                    if !ap.is_empty() {
                        pipeline.push(Step {
                            step: "auto-quirk".into(),
                            tool: "quirk-engine".into(),
                            status: if ns == "FUNC_OK" || ns == "MAKE_FAIL" { "ok".into() } else { "fail".into() },
                        });
                        quirks_applied = ap;
                        status = ns;
                        if oki { ok += 1; }
                    }
                }
                let (probes, libs, hdrs, pkgs) = collect(&d, &cf_log, &mk_log);
                if status != "FUNC_OK" {
                    diagnostic = diag_line(&cf_log, &mk_log);
                }
                let verified = status == "FUNC_OK";
                let outputs = if verified { collect_outputs(&d) } else { Vec::new() };
                let deep_expansion = analyze_expansion(&d, &cf_log);
                // The compass: run the GNU-autotools oracle on the same repo and classify ours vs it.
                let (oracle, divergence) = if std::env::var("ATLAS_ORACLE").is_ok() {
                    let (o, dv) = run_oracle(&d, &status, &deep_expansion);
                    (Some(o), dv)
                } else {
                    (None, None)
                };
                if !libs.is_empty() {
                    quirks.push(format!("LIBS={}", libs.join(" ")));
                }
                // === build-court receipt + hermeticity + missing-dep inference ===
                let environment = fingerprint_environment();
                let probe_trace = build_probe_trace(&probes, &cf_log);
                let suggested_deps = infer_missing_deps(&hdrs, &cf_log);
                let toolchain = Toolchain {
                    autoconf_rs: ac_ver.clone(),
                    automake_rs: am_ver.clone(),
                    m4_rs_core: "0.1.4".into(),
                    gnu_free: true,
                };
                let court = court_status(&status, &oracle, &quirks_matched);
                let receipt_hash = compute_receipt_hash(&toolchain, &probes, &outputs, &oracle, &court);
                let receipt = Receipt {
                    court_status: court,
                    probe_trace,
                    quirks_matched,
                    quirks_applied,
                    receipt_hash,
                    schema: "automake-rs.build-court/v1",
                };
                write_recipe(
                    &out_dir,
                    &slug,
                    Recipe {
                        schema: "automake-rs.build-atlas/v2",
                        repo: repo.to_string(),
                        source: Source {
                            url: format!("https://github.com/{}", repo),
                            git_sha: git_sha.clone(),
                            snapshot_utc: "2026-06-27".into(),
                        },
                        toolchain,
                        target: Target {
                            cc: "cc".into(),
                            cflags: "-g -O2".into(),
                            host: "x86_64-pc-linux-gnu".into(),
                        },
                        pass_pipeline: pipeline,
                        probe_results: probes,
                        feature_flags: FeatureFlags { configure_args: Vec::new() },
                        dependencies: Dependencies {
                            pkg_config: pkgs,
                            system_libs: libs,
                            headers_needed: hdrs.clone(),
                            missing: hdrs,
                        },
                        quirks,
                        outputs,
                        status: status.clone(),
                        verified,
                        diagnostic,
                        deep_expansion,
                        oracle,
                        divergence,
                        receipt,
                        environment,
                        suggested_deps,
                    },
                );
                println!("[{:>3}] {:<40} {}", n, repo, status);
                let _ = std::fs::remove_dir_all(&base);
                continue;
            }
        }
        // clone/no-ac path
        write_recipe(
            &out_dir,
            &slug,
            Recipe {
                schema: "automake-rs.build-atlas/v2",
                repo: repo.to_string(),
                source: Source { url: format!("https://github.com/{}", repo), git_sha, snapshot_utc: "2026-06-27".into() },
                toolchain: Toolchain { autoconf_rs: ac_ver.clone(), automake_rs: am_ver.clone(), m4_rs_core: "0.1.4".into(), gnu_free: true },
                target: Target { cc: "cc".into(), cflags: "-g -O2".into(), host: "x86_64-pc-linux-gnu".into() },
                pass_pipeline: pipeline,
                probe_results: BTreeMap::new(),
                feature_flags: FeatureFlags { configure_args: Vec::new() },
                dependencies: Dependencies { pkg_config: vec![], system_libs: vec![], headers_needed: vec![], missing: vec![] },
                quirks: vec![],
                outputs: vec![],
                status: status.clone(),
                verified: false,
                diagnostic,
                deep_expansion: None,
                oracle: None,
                divergence: None,
                receipt: Receipt {
                    court_status: if status == "CLONE_FAIL" { "failed".into() } else { "failed".into() },
                    schema: "automake-rs.build-court/v1",
                    ..Default::default()
                },
                environment: fingerprint_environment(),
                suggested_deps: vec![],
            },
        );
        println!("[{:>3}] {:<40} {}", n, repo, status);
        let _ = std::fs::remove_dir_all(&base);
    }

    write_index(&out_dir);
    println!("\natlas: {} recipes, {} FUNC_OK ({}%)", n, ok, if n > 0 { ok * 100 / n } else { 0 });
    ExitCode::SUCCESS
}

fn quirks_from_ac(ac: &str, d: &Path) -> Vec<String> {
    let mut q = Vec::new();
    if ac.contains("AX_") {
        q.push("autoconf-archive-macros".into());
    }
    if ac.contains("LT_INIT") || ac.contains("AC_PROG_LIBTOOL") {
        q.push("libtool".into());
    }
    if ac.contains("PKG_CHECK_MODULES") {
        q.push("pkg-config".into());
    }
    if ac.contains("AM_GNU_GETTEXT") {
        q.push("gettext".into());
    }
    if ac.contains("AC_CONFIG_SUBDIRS") || d.join("Makefile.am").exists() && std::fs::read_to_string(d.join("Makefile.am")).unwrap_or_default().contains("SUBDIRS") {
        q.push("subdirs".into());
    }
    q
}

/// Run the REAL GNU autotools (system autoreconf/aclocal/autoconf + configure[+make]) on a git-reset
/// copy of the repo, then classify ours vs the oracle. `ours_status` is our pipeline's final status.
/// Returns the Oracle outcome and — when ours fails but the oracle got further — the Divergence that
/// shows exactly what to fix. Gated by ATLAS_ORACLE=1 (it ~doubles per-repo time).
fn run_oracle(
    d: &Path,
    ours_status: &str,
    de: &Option<DeepExpansion>,
) -> (Oracle, Option<Divergence>) {
    let mut o = Oracle::default();
    // snapshot ours' configure size + error before we reset the tree
    let ours_cfg_lines = std::fs::read_to_string(d.join("configure"))
        .map(|s| s.lines().count())
        .unwrap_or(0);
    // reset the working tree so the real run starts clean (remove our generated files)
    let _ = Command::new("git").arg("-C").arg(d).arg("clean").arg("-fdxq").status();
    let _ = Command::new("git").arg("-C").arg(d).arg("checkout").arg("--").arg(".").status();

    let (ar_ok, ar_log) = run_timed(d, 150, "autoreconf", &["-fi", "."]);
    o.real_autoreconf = if ar_ok && d.join("configure").exists() { "ok".into() } else { "fail".into() };
    if o.real_autoreconf != "ok" {
        o.real_first_error = first_err(&ar_log);
        o.real_configure = "skipped".into();
        o.real_make = "skipped".into();
    } else {
        let (cf_ok, cf_log) = run_timed(d, 180, "./configure", &[]);
        o.real_configure_lines = std::fs::read_to_string(d.join("configure")).map(|s| s.lines().count()).unwrap_or(0);
        o.real_configure = if cf_ok { "ok".into() } else { o.real_first_error = first_err(&cf_log); "fail".into() };
        if cf_ok {
            let (mk_ok, _mk) = run_timed(d, 300, "make", &["-j2"]);
            o.real_make = if mk_ok { "ok".into() } else { "fail".into() };
        } else {
            o.real_make = "skipped".into();
        }
    }

    let ours_cleared = matches!(ours_status, "MAKE_FAIL" | "FUNC_OK");
    let ours_made = ours_status == "FUNC_OK";
    let real_cleared = o.real_configure == "ok";
    let real_made = o.real_make == "ok";
    o.classification = if !real_cleared && o.real_autoreconf != "ok" {
        if ours_status == "CONFIGURE_GEN_FAIL" { "NOT_STANDALONE".into() } else { "OURS_BETTER".into() }
    } else if !real_cleared {
        if ours_cleared { "OURS_BETTER".into() } else { "BOTH_CONFIGURE_FAIL".into() }
    } else if !ours_cleared {
        if ours_status == "CONFIGURE_GEN_FAIL" { "OURS_GEN_FAIL".into() } else { "OURS_BUG_CONFIGURE".into() }
    } else if real_made && !ours_made {
        "OURS_BUG_MAKE".into()
    } else if ours_made && real_made {
        "BOTH_OK".into()
    } else {
        "BOTH_MAKE_FAIL".into()
    };

    // Divergence detail only for the fixable buckets (real got further than ours).
    let div = if o.classification.starts_with("OURS_BUG") || o.classification == "OURS_GEN_FAIL" {
        let dd = de.as_ref();
        Some(Divergence {
            stage: if o.classification == "OURS_BUG_MAKE" { "make".into() }
                   else if o.classification == "OURS_GEN_FAIL" { "autoreconf".into() }
                   else { "configure".into() },
            ours_error: dd.and_then(|x| x.syntax_errors.first().map(|s| format!("line {}: {} (near `{}`)", s.line, s.source, s.token)))
                .unwrap_or_default(),
            ours_error_context: dd.map(|x| x.syntax_errors.iter().take(3).map(|s| s.source.clone()).collect()).unwrap_or_default(),
            ours_configure_lines: ours_cfg_lines,
            real_configure_lines: o.real_configure_lines,
            macros_ours_left_undefined: dd.map(|x| {
                let mut v: Vec<String> = x.leaked_macros.iter().map(|m| m.name.clone()).collect();
                v.sort(); v.dedup(); v.truncate(15); v
            }).unwrap_or_default(),
        })
    } else {
        None
    };
    (o, div)
}

/// First hard error line from a log (configure/autoreconf), skipping known noise.
fn first_err(log: &str) -> String {
    for l in log.lines() {
        let ll = l.to_lowercase();
        if ll.contains("confdefs.h: no such file") { continue; }
        if ll.contains("error:") || ll.contains("syntax error") || ll.contains("command not found")
            || ll.contains("no such file") || ll.contains("undefined macro") || ll.contains("possibly undefined")
        {
            return l.trim().chars().take(120).collect();
        }
    }
    String::new()
}

/// Forensic scan of the generated `configure` plus the configure run-log. Surfaces the exact
/// expansion failures (leaked macros, heredoc imbalance, syntax-error source context, malformed
/// cache vars, residual @VAR@) so each can be fixed from the recipe without re-running on the VM.
fn analyze_expansion(d: &Path, cf_log: &str) -> Option<DeepExpansion> {
    let cfg = std::fs::read_to_string(d.join("configure")).ok()?;
    let lines: Vec<&str> = cfg.lines().collect();
    let mut de = DeepExpansion {
        configure_lines: lines.len(),
        ..Default::default()
    };

    const MACRO_PREFIXES: &[&str] =
        &["AC_", "AX_", "AM_", "LT_", "PKG_", "AS_", "AH_", "_AC_", "_AX_", "_LT_", "m4_", "AT_"];
    for (i, l) in lines.iter().enumerate() {
        let t = l.trim_start();
        // A leaked macro call: <PREFIX><UPPER/under name>( at the start of a shell statement.
        if let Some(pfx) = MACRO_PREFIXES.iter().find(|p| t.starts_with(**p)) {
            // name = leading run of [A-Za-z0-9_], must be followed by '(' to be a macro call.
            let name: String = t.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
            let after = &t[name.len()..];
            if after.starts_with('(') && name.len() > pfx.len() && de.leaked_macros.len() < 40 {
                de.leaked_macros.push(LeakedMacro {
                    name,
                    line: i + 1,
                    context: t.chars().take(72).collect(),
                });
            }
        }
        // Heredoc accounting: count ALL `<<_ACEOF` openers (conftest probes, config.status, help)
        // vs lone `_ACEOF` terminators. A genuine imbalance == a missing opener (the compile-probe
        // emitted as raw shell -> `syntax error near '('`), not just legit multi-heredoc usage.
        if l.contains("<<_ACEOF") || l.contains("<<\\_ACEOF") {
            de.heredoc_openers += 1;
        }
        if l.trim() == "_ACEOF" {
            de.heredoc_terminators += 1;
        }
        // Malformed cache-var refs.
        if t.contains("${") {
            if let Some(rest) = t.split("${").nth(1) {
                let head = rest.chars().take(2).collect::<String>();
                if head.starts_with('$') || head.starts_with(' ') || head.starts_with('}') {
                    if de.cache_var_anomalies.len() < 20 {
                        de.cache_var_anomalies.push(format!("line {}: {}", i + 1, t.chars().take(60).collect::<String>()));
                    }
                }
            }
        }
        // Residual substitution placeholders (@VAR@ that config.status never filled).
        let mut rest = *l;
        while let Some(a) = rest.find('@') {
            let tail = &rest[a + 1..];
            if let Some(b) = tail.find('@') {
                let var = &tail[..b];
                if !var.is_empty()
                    && var.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    && var.chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false)
                    && de.residual_placeholders.len() < 30
                    && !de.residual_placeholders.iter().any(|s| s == var)
                {
                    de.residual_placeholders.push(var.to_string());
                }
                rest = &tail[b + 1..];
            } else {
                break;
            }
        }
    }
    de.heredoc_imbalance = de.heredoc_terminators as i64 - de.heredoc_openers as i64;

    // Conftest archaeology: detect C preprocessor directives mangled by m4 (the deep autotools bug
    // where `include`/`ifdef`/`define` builtins expand inside conftest C source). Track only inside
    // conftest heredoc regions so real `# comment` shell lines aren't misread.
    {
        // in_conftest spans ONLY a real conftest heredoc: `cat … <<_ACEOF >conftest.$ac_ext` … `_ACEOF`
        // (not the prologue's `/* confdefs.h */` init, which would flag real shell comments).
        let mut in_conftest = false;
        let mut cur_program: Vec<String> = Vec::new();
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim_start();
            if l.contains("<<_ACEOF") && (l.contains(">conftest") || l.contains("confdefs.h -")) {
                in_conftest = true;
                cur_program.clear();
            } else if l.trim() == "_ACEOF" {
                if in_conftest && !cur_program.is_empty() && de.conftest_programs.len() < 6 {
                    de.conftest_programs.push(cur_program.join("\\n"));
                }
                in_conftest = false;
            } else if in_conftest && cur_program.len() < 12 {
                cur_program.push(l.trim().chars().take(76).collect());
            }
            if t.starts_with("#include") || t.starts_with("#ifdef") || t.starts_with("#ifndef")
                || t.starts_with("#define") || t.starts_with("#if ") || t.starts_with("#endif") {
                de.conftest_directives_intact += 1;
            }
            // Mangled directive inside a conftest. Only the UNAMBIGUOUS forms are flagged: `# <hdr>`
            // (an eaten `#include <...>`) and `# "hdr"` (eaten `#include "..."`). The earlier `# WORD`
            // heuristic (eaten `#ifdef`/`#define`) is indistinguishable from a legitimate shell comment
            // (e.g. AC_DEFINE's `# Define unquoted: USE_STRUCT_MNTTAB`) and produced 55 false positives,
            // so it's dropped — `#ifdef`/`#define` corruption is still implied when `# <` co-occurs.
            if in_conftest && (t.starts_with("# <") || t.starts_with("# \"")) {
                de.conftest_directives_mangled += 1;
                if de.conftest_corruption.len() < 20 {
                    de.conftest_corruption.push(format!("line {}: {} (include eaten)", i + 1, t.chars().take(40).collect::<String>()));
                }
            }
        }
    }

    // Cross-reference each "./configure: line N: syntax error" with its source line.
    for ll in cf_log.lines() {
        if ll.contains("syntax error") {
            if let Some(npos) = ll.find("line ") {
                let num: String = ll[npos + 5..].chars().take_while(|c| c.is_ascii_digit()).collect();
                if let Ok(n) = num.parse::<usize>() {
                    let token = ll
                        .split("unexpected token")
                        .nth(1)
                        .map(|s| s.trim().trim_matches('`').trim_matches('\'').chars().take(24).collect::<String>())
                        .unwrap_or_default();
                    let source = lines.get(n.saturating_sub(1)).map(|s| s.trim().chars().take(72).collect()).unwrap_or_default();
                    // capture the enclosing construct: 4 lines before .. 3 after the error line
                    let lo = n.saturating_sub(5);
                    let hi = (n + 3).min(lines.len());
                    let block: Vec<String> = (lo..hi)
                        .map(|j| format!("{}: {}", j + 1, lines[j].chars().take(76).collect::<String>()))
                        .collect();
                    if de.syntax_errors.len() < 20 {
                        de.syntax_errors.push(SyntaxError { line: n, token, source, block });
                    }
                }
            }
        }
    }
    // Name the probe ours died on: the last `checking …` message printed before the run ended, plus
    // the few lines after it (the crash fallout). configure prints "checking X... " with no newline,
    // so the message + the failure often share a line; we split on "checking" to recover it.
    {
        let log_lines: Vec<&str> = cf_log.lines().collect();
        let mut last_check_idx = None;
        for (i, l) in log_lines.iter().enumerate() {
            if l.contains("checking ") {
                last_check_idx = Some(i);
            }
        }
        if let Some(i) = last_check_idx {
            if let Some(pos) = log_lines[i].find("checking ") {
                de.failed_during_check = log_lines[i][pos..].chars().take(80).collect();
            }
            de.failure_tail = log_lines[i..]
                .iter()
                .take(5)
                .map(|s| s.trim().chars().take(80).collect::<String>())
                .filter(|s| !s.is_empty())
                .collect();
        } else if !log_lines.is_empty() {
            de.failure_tail = log_lines.iter().rev().take(4).rev()
                .map(|s| s.trim().chars().take(80).collect::<String>())
                .filter(|s| !s.is_empty()).collect();
        }
    }
    Some(de)
}

/// Fingerprint the build environment (hermeticity layer): the exact toolchain + the env vars that
/// shape a build, so a recipe is reproducible across machines.
fn fingerprint_environment() -> Environment {
    let run = |prog: &str, args: &[&str]| -> String {
        Command::new(prog).args(args).output().ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).lines().next().unwrap_or("").trim().to_string())
            .unwrap_or_default()
    };
    let mut relevant_env = Vec::new();
    for k in ["CC", "CXX", "CFLAGS", "CXXFLAGS", "CPPFLAGS", "LDFLAGS", "LIBS", "PKG_CONFIG_PATH"] {
        if let Ok(v) = std::env::var(k) {
            if !v.is_empty() { relevant_env.push(format!("{}={}", k, v)); }
        }
    }
    Environment {
        cc: std::env::var("CC").unwrap_or_else(|_| "cc".into()),
        cc_version: run("cc", &["--version"]),
        host_triplet: run("cc", &["-dumpmachine"]),
        pkg_config_version: run("pkg-config", &["--version"]),
        make_version: run("make", &["--version"]),
        relevant_env,
    }
}

/// Probe-failure root-cause: turn the generated config.h HAVE_* results + the configure run-log into a
/// per-probe trace that says WHY each probe passed or failed (header not found, symbol not found, link
/// failed) rather than just yes/no.
fn build_probe_trace(probes: &BTreeMap<String, u8>, cf_log: &str) -> Vec<ProbeStep> {
    let log_l = cf_log.to_lowercase();
    let mut trace = Vec::new();
    for (name, &val) in probes.iter() {
        let yes = val == 1;
        // derive kind from the HAVE_ name
        let kind = if name.ends_with("_H") || name.contains("HEADER") { "header" }
            else if name.starts_with("HAVE_LIB") { "lib" }
            else { "func" };
        let reason = if yes { "ok".to_string() } else {
            // why did it fail? look for a clue in the run log
            let stem = name.trim_start_matches("HAVE_").to_lowercase();
            if log_l.contains(&format!("{}: no such file", stem.replace('_', "."))) { "header-not-found".into() }
            else if kind == "lib" && log_l.contains("cannot find -l") { "link-failed".into() }
            else if log_l.contains("undefined reference") { "symbol-not-found".into() }
            else { "probe-returned-no".into() }
        };
        if trace.len() < 200 {
            trace.push(ProbeStep { name: name.clone(), kind: kind.into(), result: if yes {"yes".into()} else {"no".into()}, reason });
        }
    }
    trace
}

/// Missing-dep inference: for failed header/lib probes (and any "X.h: No such file" in the log),
/// suggest the distro package that would satisfy it.
fn infer_missing_deps(hdrs_needed: &[String], cf_log: &str) -> Vec<SuggestedDep> {
    // header/lib stem -> providing package (Debian/Ubuntu names; the common autotools deps)
    const PKGS: &[(&str, &str, &str)] = &[
        ("zlib.h", "header", "zlib1g-dev"), ("zconf.h", "header", "zlib1g-dev"),
        ("openssl/", "header", "libssl-dev"), ("curl/", "header", "libcurl4-openssl-dev"),
        ("pcre.h", "header", "libpcre3-dev"), ("pcre2.h", "header", "libpcre2-dev"),
        ("fuse.h", "header", "libfuse-dev"), ("fuse3/", "header", "libfuse3-dev"),
        ("ncurses.h", "header", "libncurses-dev"), ("curses.h", "header", "libncurses-dev"),
        ("sqlite3.h", "header", "libsqlite3-dev"), ("expat.h", "header", "libexpat1-dev"),
        ("libxml/", "header", "libxml2-dev"), ("png.h", "header", "libpng-dev"),
        ("jpeglib.h", "header", "libjpeg-dev"), ("ffi.h", "header", "libffi-dev"),
        ("gmp.h", "header", "libgmp-dev"), ("readline/", "header", "libreadline-dev"),
        ("libusb", "header", "libusb-1.0-0-dev"), ("alsa/", "header", "libasound2-dev"),
        ("X11/", "header", "libx11-dev"), ("GL/", "header", "libgl-dev"),
        ("dbus/", "header", "libdbus-1-dev"), ("systemd/", "header", "libsystemd-dev"),
        ("pcap.h", "header", "libpcap-dev"), ("ldns/", "header", "libldns-dev"),
        ("cryptsetup", "header", "libcryptsetup-dev"), ("gcrypt.h", "header", "libgcrypt20-dev"),
        ("event.h", "header", "libevent-dev"), ("json-c/", "header", "libjson-c-dev"),
    ];
    let log_l = cf_log.to_lowercase();
    let mut out: Vec<SuggestedDep> = Vec::new();
    let mut consider = |needle: &str| {
        for (stem, kind, pkg) in PKGS {
            if needle.contains(stem) && !out.iter().any(|s| s.package == *pkg) {
                out.push(SuggestedDep { missing: needle.to_string(), kind: (*kind).to_string(), package: (*pkg).to_string() });
            }
        }
    };
    for h in hdrs_needed { consider(&h.to_lowercase()); }
    // also scan the run log for "X.h: No such file"
    for l in log_l.lines() {
        if l.contains("no such file") && l.contains(".h") {
            if let Some(h) = l.split(':').next() { consider(h.trim()); }
        }
    }
    out.truncate(20);
    out
}

/// Quirk rule engine: versioned heuristics matched on configure.ac text, file presence, or run-log.
/// Each fired rule is recorded so we know which quirks a build depended on.
fn match_quirks(ac_text: &str, d: &Path, cf_log: &str) -> Vec<String> {
    // (id, predicate-kind, needle): kind a=ac_text, f=file-exists, l=run-log
    const RULES: &[(&str, char, &str)] = &[
        ("uses-libtool", 'a', "LT_INIT"),
        ("uses-libtool-old", 'a', "AC_PROG_LIBTOOL"),
        ("uses-pkg-config", 'a', "PKG_CHECK_MODULES"),
        ("uses-intltool", 'a', "IT_PROG_INTLTOOL"),
        ("uses-gettext", 'a', "AM_GNU_GETTEXT"),
        ("uses-python", 'a', "AM_PATH_PYTHON"),
        ("uses-subdir-objects", 'a', "subdir-objects"),
        ("uses-maintainer-mode", 'a', "AM_MAINTAINER_MODE"),
        ("has-m4-macro-dir", 'f', "m4"),
        ("has-acinclude", 'f', "acinclude.m4"),
        ("vendored-aclocal", 'f', "aclocal.m4"),
        ("uses-ax-archive", 'a', "AX_"),
        ("uses-pthread-check", 'a', "AX_PTHREAD"),
        ("emits-config-commands-post", 'a', "AC_CONFIG_COMMANDS_POST"),
        ("perl-in-configure", 'a', "PERL"),
    ];
    let mut out = Vec::new();
    for (id, kind, needle) in RULES {
        let hit = match kind {
            'a' => ac_text.contains(needle),
            'f' => d.join(needle).exists(),
            'l' => cf_log.contains(needle),
            _ => false,
        };
        if hit { out.push(id.to_string()); }
    }
    out
}

/// GNU-free fix for a matched quirk: a `./configure` flag (or empty if the quirk has no auto-fix).
/// Never copies GNU aux or invokes GNU tools — only flags/env/mkdir are permitted.
fn quirk_fix(id: &str) -> Option<String> {
    match id {
        "uses-maintainer-mode" => Some("--disable-maintainer-mode".into()),
        "uses-subdir-objects" => Some("--disable-dependency-tracking".into()),
        _ => None,
    }
}

/// Auto-apply quirks: when configure failed, collect GNU-free configure flags from the matched fixable
/// quirks, re-run configure (+make), and record which quirks actually got the build further. Returns
/// (applied-with-verdict, possibly-improved-status). `ok_inc` is set true if it newly reached FUNC_OK.
fn auto_apply_quirks(d: &Path, matched: &[String], cur_status: &str) -> (Vec<QuirkApplied>, String, bool) {
    let mut applied = Vec::new();
    let fixes: Vec<(String, String)> = matched
        .iter()
        .filter_map(|id| quirk_fix(id).map(|f| (id.clone(), f)))
        .collect();
    if fixes.is_empty() || matches!(cur_status, "MAKE_FAIL" | "FUNC_OK") {
        return (applied, cur_status.to_string(), false);
    }
    let flag_args: Vec<&str> = fixes.iter().map(|(_, f)| f.as_str()).collect();
    let (cfok, _) = run_timed(d, 180, "./configure", &flag_args);
    let mut new_status = cur_status.to_string();
    let mut ok_inc = false;
    if cfok {
        new_status = "MAKE_FAIL".to_string();
        let (mkok, _) = run_timed(d, 300, "make", &["-j2"]);
        if mkok { new_status = "FUNC_OK".to_string(); ok_inc = true; }
    }
    // verified = applying the quirks actually cleared configure (which had failed before)
    for (id, action) in fixes {
        applied.push(QuirkApplied { id, action, verified: cfok });
    }
    (applied, new_status, ok_inc)
}

/// Build-court verdict for a recipe.
fn court_status(status: &str, oracle: &Option<Oracle>, quirks: &[String]) -> String {
    let cls = oracle.as_ref().map(|o| o.classification.as_str()).unwrap_or("");
    if status == "FUNC_OK" {
        if quirks.is_empty() { "sealed".into() } else { "quirk_dependent".into() }
    } else if status == "MAKE_FAIL" {
        "partial".into()
    } else if cls == "NOT_STANDALONE" || cls.starts_with("BOTH_") {
        "not_standalone".into()
    } else {
        "failed".into()
    }
}

/// sha256 hash-chain over the recipe's load-bearing fields — the sealed-receipt anchor.
fn compute_receipt_hash(
    toolchain: &Toolchain, probes: &BTreeMap<String, u8>, outputs: &[Output],
    oracle: &Option<Oracle>, court: &str,
) -> String {
    let mut h = Sha256::new();
    h.update(format!("ac={} am={} m4={}\n", toolchain.autoconf_rs, toolchain.automake_rs, toolchain.m4_rs_core).as_bytes());
    for (k, v) in probes.iter() { h.update(format!("{}={}\n", k, v).as_bytes()); }
    for o in outputs { h.update(format!("{}@{}\n", o.path, o.sha256).as_bytes()); }
    if let Some(o) = oracle { h.update(format!("oracle={}\n", o.classification).as_bytes()); }
    h.update(format!("court={}\n", court).as_bytes());
    format!("{:x}", h.finalize())
}

fn collect(d: &Path, cf: &str, mk: &str) -> (BTreeMap<String, u8>, Vec<String>, Vec<String>, Vec<String>) {
    // probe results from the generated config header
    let mut probes = BTreeMap::new();
    for entry in std::fs::read_dir(d).into_iter().flatten().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if (name == "config.h" || name.ends_with("config.h")) && !name.ends_with(".in") {
            if let Ok(txt) = std::fs::read_to_string(entry.path()) {
                for l in txt.lines() {
                    if let Some(rest) = l.strip_prefix("#define HAVE_") {
                        if let Some(var) = rest.split_whitespace().next() {
                            probes.insert(format!("HAVE_{}", var), 1);
                        }
                    }
                }
            }
        }
    }
    let libs: Vec<String> = std::fs::read_to_string(d.join("Makefile"))
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("LIBS ="))
        .map(|l| l.trim_start_matches("LIBS =").split_whitespace().map(|s| s.to_string()).collect())
        .unwrap_or_default();
    let mut hdrs: Vec<String> = Vec::new();
    for log in [cf, mk] {
        for l in log.lines() {
            if let Some(idx) = l.find(".h: No such file") {
                let pre = &l[..idx + 2];
                if let Some(h) = pre.rsplit([' ', '"', '<', ':']).next() {
                    if h.ends_with(".h") && !hdrs.contains(&h.to_string()) {
                        hdrs.push(h.to_string());
                    }
                }
            }
        }
    }
    let mut pkgs: Vec<String> = Vec::new();
    for l in cf.lines() {
        if l.contains("pkg-config") || l.contains(">= ") {
            if pkgs.len() < 8 {
                pkgs.push(l.trim().chars().take(80).collect());
            }
        }
    }
    (probes, libs, hdrs, pkgs)
}

fn collect_outputs(d: &Path) -> Vec<Output> {
    let mut outs = Vec::new();
    walk(d, d, &mut outs, 0);
    outs.truncate(12);
    outs
}
fn walk(root: &Path, dir: &Path, outs: &mut Vec<Output>, depth: usize) {
    if depth > 3 || outs.len() >= 12 {
        return;
    }
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().into_owned();
        if name == ".git" {
            continue;
        }
        if p.is_dir() {
            walk(root, &p, outs, depth + 1);
        } else {
            let is_lib = name.ends_with(".a") || name.contains(".so");
            let is_exe = !name.contains('.')
                && std::fs::metadata(&p)
                    .map(|m| {
                        use std::os::unix::fs::PermissionsExt;
                        m.permissions().mode() & 0o111 != 0
                    })
                    .unwrap_or(false);
            if (is_lib || is_exe) && outs.len() < 12 {
                if let Some(h) = sha256_16(&p) {
                    outs.push(Output {
                        path: p.strip_prefix(root).unwrap_or(&p).display().to_string(),
                        sha256: h,
                        kind: if is_lib { "library".into() } else { "executable".into() },
                    });
                }
            }
        }
    }
}

fn diag_line(cf: &str, mk: &str) -> String {
    for log in [mk, cf] {
        for l in log.lines() {
            let ll = l.to_lowercase();
            if ll.contains("confdefs.h: no such file") { continue; }
            if ll.contains("configure: error")
                || ll.contains("syntax error")
                || ll.contains("command not found")
                || ll.contains(": error:")
                || ll.contains("no such file")
            {
                return l.trim().chars().take(140).collect();
            }
        }
    }
    String::new()
}

/// `xtask atlas-replay <recipe.json> [--keep]` — REPRODUCER + regression gate. Reads a recipe, clones
/// the repo at the pinned git_sha into a clean dir, re-applies the recorded pass_pipeline + the
/// configure flags from feature_flags/quirks_applied, rebuilds, and verifies the rebuilt artifacts'
/// sha256s against the recipe's `outputs`. Emits a replay receipt (reproduced | diverged | build_failed
/// | clone_failed) and exits non-zero on anything but a clean reproduction — so it gates regressions.
pub fn replay() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let arg = match args.get(2) {
        Some(p) => p.clone(),
        None => {
            eprintln!("usage: cargo xtask atlas-replay <recipe.json | owner/name | slug> [--keep]");
            eprintln!("  resolves a recipe by path, by `owner/name`, or by `owner__name` slug under atlas/recipes/");
            return ExitCode::from(2);
        }
    };
    let keep = args.iter().any(|a| a == "--keep");
    // Resolve the recipe: an existing path, else a repo slug under atlas/recipes/ (owner/name or owner__name).
    let recipe_path = {
        let direct = PathBuf::from(&arg);
        if direct.is_file() {
            direct
        } else {
            let slug = arg.replace('/', "__");
            let cand = PathBuf::from("atlas/recipes").join(format!("{}.json", slug));
            if cand.is_file() { cand } else {
                eprintln!("atlas-replay: no recipe found for '{}' (tried {} and atlas/recipes/{}.json)", arg, direct.display(), slug);
                return ExitCode::from(2);
            }
        }
    };
    eprintln!("atlas-replay: recipe {}", recipe_path.display());
    let v: serde_json::Value = match std::fs::read_to_string(&recipe_path).ok().and_then(|s| serde_json::from_str(&s).ok()) {
        Some(v) => v,
        None => { eprintln!("atlas-replay: cannot read/parse {}", recipe_path.display()); return ExitCode::from(2); }
    };

    let repo = v["repo"].as_str().unwrap_or("").to_string();
    let url = v["source"]["url"].as_str().filter(|s| !s.is_empty()).map(|s| s.to_string())
        .unwrap_or_else(|| format!("https://github.com/{}", repo));
    let pinned_sha = v["source"]["git_sha"].as_str().unwrap_or("").to_string();
    // configure flags: feature_flags.configure_args + any --flag actions the recipe's quirks applied
    let mut cfg_args: Vec<String> = v["feature_flags"]["configure_args"].as_array().into_iter().flatten()
        .filter_map(|x| x.as_str().map(|s| s.to_string())).collect();
    if let Some(qa) = v["receipt"]["quirks_applied"].as_array() {
        for q in qa { if let Some(a) = q["action"].as_str() { if a.starts_with("--") && !cfg_args.iter().any(|c| c == a) { cfg_args.push(a.to_string()); } } }
    }
    let recipe_outputs: BTreeMap<String, String> = v["outputs"].as_array().into_iter().flatten()
        .filter_map(|o| Some((o["path"].as_str()?.to_string(), o["sha256"].as_str()?.to_string()))).collect();

    // autoreconf-rs resolves AUTOCONF_RS / AUTOMAKE_RS from the env itself (same as the atlas run).
    let ars = tool("AUTORECONF_RS", "autoreconf-rs");

    let base = std::env::temp_dir().join(format!("atlasreplay_{}", repo.replace('/', "__")));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).ok();
    let d = base.join("s");

    let mut steps: Vec<serde_json::Value> = Vec::new();
    let mut replay_status = "clone_failed";
    let mut sha_used = String::new();

    eprintln!("  [1/5] clone {} ...", url);
    let (cloned, _) = run_timed(&base, 120, "git", &["clone", "-q", &url, d.to_str().unwrap()]);
    eprintln!("        clone: {}", if cloned { "ok" } else { "FAIL" });
    steps.push(serde_json::json!({"step": "clone", "status": if cloned {"ok"} else {"fail"}}));
    let mut out_compare = serde_json::json!({});
    let mut matched = 0usize; let mut mismatched = 0usize; let mut missing = 0usize;
    if cloned {
        // pin to the recorded sha for a faithful replay; fall back to HEAD if the sha is gone.
        let pin_ok = !pinned_sha.is_empty() && Command::new("git").args(["checkout", "-q", &pinned_sha]).current_dir(&d).status().map(|s| s.success()).unwrap_or(false);
        sha_used = if pin_ok { pinned_sha.clone() } else {
            Command::new("git").args(["rev-parse", "HEAD"]).current_dir(&d).output().ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default()
        };
        eprintln!("  [2/5] checkout {} ({})", &sha_used[..sha_used.len().min(12)], if pin_ok { "pinned" } else { "HEAD fallback — pinned sha gone" });
        steps.push(serde_json::json!({"step": "checkout", "sha": sha_used, "pinned": pin_ok, "status": if pin_ok {"ok"} else {"head-fallback"}}));

        replay_status = "build_failed";
        eprintln!("  [3/5] autoreconf-rs -fi ...");
        let (_a, _) = run_timed(&d, 150, &ars, &["-fi", "."]);
        let gen_ok = d.join("configure").exists();
        eprintln!("        autoreconf: {}", if gen_ok { "ok (configure generated)" } else { "FAIL (no configure)" });
        steps.push(serde_json::json!({"step": "autoreconf", "tool": ars, "status": if gen_ok {"ok"} else {"fail"}}));
        if gen_ok {
            eprintln!("  [4/5] ./configure {} ...", cfg_args.join(" "));
            let argref: Vec<&str> = cfg_args.iter().map(|s| s.as_str()).collect();
            let (cfok, cfl) = run_timed(&d, 180, "./configure", &argref);
            eprintln!("        configure: {}", if cfok { "ok" } else { "FAIL" });
            steps.push(serde_json::json!({"step": "configure", "args": cfg_args, "status": if cfok {"ok"} else {"fail"}}));
            if cfok {
                eprintln!("  [5/5] make -j2 ...");
                let (mkok, mkl) = run_timed(&d, 300, "make", &["-j2"]);
                eprintln!("        make: {}", if mkok { "ok" } else { "FAIL" });
                steps.push(serde_json::json!({"step": "make", "status": if mkok {"ok"} else {"fail"}}));
                let _ = (cfl, mkl);
                // verify: rebuilt artifacts vs recipe outputs (by path -> sha256)
                let rebuilt = collect_outputs(&d);
                let rebuilt_map: BTreeMap<String, String> = rebuilt.iter().map(|o| (o.path.clone(), o.sha256.clone())).collect();
                let mut details = Vec::new();
                for (path, want) in &recipe_outputs {
                    match rebuilt_map.get(path) {
                        Some(got) if got == want => { matched += 1; details.push(serde_json::json!({"path": path, "verdict": "match"})); }
                        Some(got) => { mismatched += 1; details.push(serde_json::json!({"path": path, "verdict": "hash-mismatch", "recipe": want, "replay": got})); }
                        None => { missing += 1; details.push(serde_json::json!({"path": path, "verdict": "missing-in-replay"})); }
                    }
                }
                out_compare = serde_json::json!({"matched": matched, "mismatched": mismatched, "missing": missing, "details": details});
                if !recipe_outputs.is_empty() {
                    eprintln!("        verify: {} matched, {} hash-mismatch, {} missing (of {} recorded outputs)", matched, mismatched, missing, recipe_outputs.len());
                }
                replay_status = if !mkok { "build_failed" }
                    else if recipe_outputs.is_empty() { "reproduced_no_outputs" }
                    else if mismatched == 0 && missing == 0 { "reproduced" }
                    else { "diverged" };
            }
        }
    }

    let receipt = serde_json::json!({
        "schema": "automake-rs.replay-receipt/v1",
        "repo": repo, "recipe": recipe_path.display().to_string(),
        "replay_status": replay_status,
        "pinned_sha": pinned_sha, "sha_replayed": sha_used,
        "configure_args": cfg_args,
        "steps": steps,
        "output_verification": out_compare,
    });
    let receipt_path = base.join("replay-receipt.json");
    let _ = std::fs::write(&receipt_path, serde_json::to_string_pretty(&receipt).unwrap_or_default() + "\n");
    println!("{}", serde_json::to_string_pretty(&receipt).unwrap_or_default());
    println!("\natlas-replay: {} — {} (receipt: {})", repo, replay_status, receipt_path.display());
    if !keep { let _ = std::fs::remove_dir_all(&base); }

    match replay_status {
        "reproduced" | "reproduced_no_outputs" => ExitCode::SUCCESS,
        _ => ExitCode::from(1), // gate: any divergence / failure is non-zero
    }
}

/// `xtask atlas-diff <baseline-dir> <experiment-dir>` — A/B compare two recipe sets by court verdict.
/// Reports flips (baseline-not-clear -> experiment-clear), regressions (clear -> not-clear), and the net,
/// over the repos present in BOTH sets. "Clear" = configure cleared (sealed | quirk_dependent | partial).
/// This is how a corpus-wide change (e.g. the leaked-macro neutralizer) is gated: ship only if net > 0.
pub fn diff() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let (a_dir, b_dir) = match (args.get(2), args.get(3)) {
        (Some(a), Some(b)) => (PathBuf::from(a), PathBuf::from(b)),
        _ => {
            eprintln!("usage: cargo xtask atlas-diff <baseline-dir> <experiment-dir>");
            return ExitCode::from(2);
        }
    };
    let load = |d: &Path| -> BTreeMap<String, String> {
        let mut m = BTreeMap::new();
        for e in std::fs::read_dir(d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) != Some("json")
                || p.file_name().and_then(|s| s.to_str()) == Some("INDEX.json")
            {
                continue;
            }
            if let Ok(txt) = std::fs::read_to_string(&p) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if let (Some(repo), Some(cs)) =
                        (v["repo"].as_str(), v.get("receipt").and_then(|r| r["court_status"].as_str()))
                    {
                        m.insert(repo.to_string(), cs.to_string());
                    }
                }
            }
        }
        m
    };
    let base = load(&a_dir);
    let exp = load(&b_dir);
    let is_clear = |c: &str| c == "sealed" || c == "quirk_dependent" || c == "partial";

    let mut flips: Vec<(String, String, String)> = Vec::new();
    let mut regress: Vec<(String, String, String)> = Vec::new();
    let mut compared = 0usize;
    for (repo, nc) in &exp {
        if let Some(bc) = base.get(repo) {
            compared += 1;
            match (is_clear(bc), is_clear(nc)) {
                (false, true) => flips.push((repo.clone(), bc.clone(), nc.clone())),
                (true, false) => regress.push((repo.clone(), bc.clone(), nc.clone())),
                _ => {}
            }
        }
    }
    println!("atlas-diff: {} -> {}", a_dir.display(), b_dir.display());
    println!("compared {} repos present in both", compared);
    println!("  FLIPS  (failed -> clear):     {}", flips.len());
    println!("  REGRESS (clear -> failed):    {}", regress.len());
    println!("  NET: {:+}", flips.len() as i64 - regress.len() as i64);
    if !flips.is_empty() {
        println!("\nflips:");
        for (r, b, n) in flips.iter().take(40) {
            println!("  + {}: {} -> {}", r, b, n);
        }
    }
    if !regress.is_empty() {
        println!("\nREGRESSIONS:");
        for (r, b, n) in &regress {
            println!("  - {}: {} -> {}", r, b, n);
        }
    }
    ExitCode::SUCCESS
}

fn write_recipe(out_dir: &Path, slug: &str, rec: Recipe) {
    let path = out_dir.join(format!("{}.json", slug));
    if let Ok(s) = serde_json::to_string_pretty(&rec) {
        let _ = std::fs::write(path, s + "\n");
    }
}

/// Classify a `divergence.ours_error` into an ACTIONABLE root. The shell error format is
/// `line N: <offending> (near `<token>`)` — that backtick is the SHELL's own quoting, not a backtick in
/// the source (the old "backtick-in-source" bucket was an artifact of matching it). So: if <offending>
/// is a macro call (UPPER/_ identifier immediately followed by `(`), the root is that leaked macro
/// (`macro:NAME`, aggregating with macros_ours_left_undefined); otherwise bucket by the near-token
/// (fi/else -> unbalanced conditional; a punctuation token -> that syntax token).
/// Classify a `make` failure diagnostic into an actionable error class — the make-layer "next front"
/// after configure-clear. Groups the diverse per-repo messages into the handful of underlying causes.
fn classify_make_error(d: &str) -> String {
    let dl = d.to_lowercase();
    if d.is_empty() {
        "(no diagnostic captured)".to_string()
    } else if dl.contains("no rule to make target") {
        "no-rule-to-make-target".to_string()
    } else if dl.contains("undefined reference") {
        "undefined-reference (link)".to_string()
    } else if dl.contains("missing separator") {
        "makefile-missing-separator".to_string()
    } else if (dl.contains("no such file") || dl.contains("not found")) && dl.contains(".h") {
        "missing-header-at-compile".to_string()
    } else if dl.contains("command not found") {
        "command-not-found".to_string()
    } else if dl.contains("syntax error") {
        "makefile/shell-syntax-error".to_string()
    } else if dl.contains(": error:") || dl.contains("error:") {
        "compiler-error".to_string()
    } else if dl.contains("permission denied") {
        "permission-denied".to_string()
    } else {
        "other".to_string()
    }
}

fn bucket_error(e: &str) -> String {
    let txt = e.find(": ").map(|p| &e[p + 2..]).unwrap_or(e);
    let offending = txt.split(" (near").next().unwrap_or(txt).trim();
    let mac: String = offending.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
    // A leaked macro appears as `IDENT(args` (no space before the paren — shell functions are called
    // without parens, so `ident(` at an error site is an unexpanded m4 macro). Accept any identifier,
    // not just UPPER-case ones, so m4_require/_LT_*/ac_* internals are classified instead of dumped into
    // syntax:other. Require a macro-ish shape (has an underscore or an uppercase letter) to avoid
    // catching a stray C token.
    if !mac.is_empty()
        && mac.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && (mac.contains('_') || mac.chars().any(|c| c.is_ascii_uppercase()))
        && offending[mac.len()..].starts_with('(')
    {
        return format!("macro:{}", mac);
    }
    // A conditional keyword followed by prose (`fi because it's not...`) is leaked AC_MSG/comment text
    // that escaped a macro body — distinct from a truly unbalanced `fi` standing alone.
    let after_kw = |kw: &str| offending.starts_with(kw)
        && offending[kw.len()..].starts_with(|c: char| c == ' ' || c == '\t')
        && offending[kw.len()..].trim().chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false);
    if after_kw("fi") || after_kw("else") || after_kw("then") {
        return "syntax:leaked-text-after-conditional".to_string();
    }
    let near = txt.find("near `").map(|i| &txt[i + 6..]).and_then(|s| s.split('`').next()).unwrap_or("").trim();
    match near {
        "fi" | "else" | "then" | "elif" => "syntax:unbalanced-conditional".to_string(),
        "done" | "do" => "syntax:unbalanced-loop".to_string(),
        "esac" | "in" => "syntax:unbalanced-case".to_string(),
        "" => "syntax:other".to_string(),
        t if t.chars().all(|c| !c.is_ascii_alphanumeric()) => format!("syntax:token:{}", t),
        _ => "syntax:other".to_string(),
    }
}

fn write_index(out_dir: &Path) {
    let mut by_status: BTreeMap<String, usize> = BTreeMap::new();
    let mut total = 0;
    let mut lines = Vec::new();
    // Corpus-wide expansion-bug tallies — rank the fixes by how many repos each unblocks.
    let mut leaked: BTreeMap<String, usize> = BTreeMap::new();
    let mut syntax_tokens: BTreeMap<String, usize> = BTreeMap::new();
    let mut residual: BTreeMap<String, usize> = BTreeMap::new();
    let mut heredoc_broken = 0usize;
    let mut with_leaks = 0usize;
    let mut conftest_corrupt = 0usize;
    // Oracle compass: classification counts + the fixable backlog (roots of OURS_BUG repos, ranked).
    let mut classif: BTreeMap<String, usize> = BTreeMap::new();
    let mut fixable_roots: BTreeMap<String, usize> = BTreeMap::new();
    let mut failed_checks: BTreeMap<String, usize> = BTreeMap::new();
    let mut courts: BTreeMap<String, usize> = BTreeMap::new();
    let mut sugg_pkgs: BTreeMap<String, usize> = BTreeMap::new();
    let mut ours_clear = 0usize;
    let mut real_clear = 0usize;
    // Analytics: quirk hotspots (automation candidates), dependency patterns, heavy hitters,
    // partial->full candidates (configure cleared but make fails where GNU makes — the closest wins).
    let mut quirk_hot: BTreeMap<String, usize> = BTreeMap::new();
    let mut hdr_needed: BTreeMap<String, usize> = BTreeMap::new();
    let mut dep_missing: BTreeMap<String, usize> = BTreeMap::new();
    let mut heavy: Vec<(usize, String, String)> = Vec::new();
    let mut p2f_diag: BTreeMap<String, usize> = BTreeMap::new();
    let mut make_fail_class: BTreeMap<String, usize> = BTreeMap::new();
    let mut partial_total = 0usize;
    let mut p2f_make = 0usize;
    // working/non-working roster for RECIPES.md: (court_status, status, repo, diagnostic)
    let mut roster: Vec<(String, String, String, String)> = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(out_dir).into_iter().flatten().flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("json") || p.file_name().and_then(|s| s.to_str()) == Some("INDEX.json") {
            continue;
        }
        if let Ok(txt) = std::fs::read_to_string(&p) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                let st = v["status"].as_str().unwrap_or("?").to_string();
                let repo = v["repo"].as_str().unwrap_or("?").to_string();
                *by_status.entry(st.clone()).or_default() += 1;
                total += 1;
                lines.push(serde_json::json!({"repo": repo, "status": st, "verified": v["verified"]}));
                if let Some(de) = v.get("deep_expansion") {
                    if let Some(arr) = de["leaked_macros"].as_array() {
                        if !arr.is_empty() { with_leaks += 1; }
                        // count each macro NAME once per repo (repos-unblocked, not raw occurrences)
                        let mut seen = std::collections::BTreeSet::new();
                        for m in arr {
                            if let Some(name) = m["name"].as_str() { seen.insert(name.to_string()); }
                        }
                        for name in seen { *leaked.entry(name).or_default() += 1; }
                    }
                    if de["heredoc_imbalance"].as_i64().unwrap_or(0) != 0 { heredoc_broken += 1; }
                    if de["conftest_directives_mangled"].as_u64().unwrap_or(0) > 0 { conftest_corrupt += 1; }
                    if let Some(arr) = de["syntax_errors"].as_array() {
                        for s in arr {
                            if let Some(tok) = s["token"].as_str() { if !tok.is_empty() { *syntax_tokens.entry(tok.to_string()).or_default() += 1; } }
                        }
                    }
                    if let Some(arr) = de["residual_placeholders"].as_array() {
                        let mut seen = std::collections::BTreeSet::new();
                        for r in arr { if let Some(s) = r.as_str() { seen.insert(s.to_string()); } }
                        for s in seen { *residual.entry(s).or_default() += 1; }
                    }
                    // the actual probe ours died on (the divergence root, not the cascade line)
                    if st != "FUNC_OK" {
                        if let Some(c) = de["failed_during_check"].as_str() {
                            if !c.is_empty() {
                                // normalize: drop trailing "... <value>" so similar checks bucket together
                                let norm = c.split("...").next().unwrap_or(c).trim().to_string();
                                *failed_checks.entry(norm).or_default() += 1;
                            }
                        }
                    }
                }
                if st == "MAKE_FAIL" || st == "FUNC_OK" { ours_clear += 1; }
                if let Some(c) = v.get("receipt").and_then(|r| r["court_status"].as_str()) {
                    *courts.entry(c.to_string()).or_default() += 1;
                }
                roster.push((
                    v.get("receipt").and_then(|r| r["court_status"].as_str()).unwrap_or("?").to_string(),
                    st.clone(),
                    repo.clone(),
                    v["diagnostic"].as_str().unwrap_or("").chars().take(90).collect::<String>(),
                ));
                if let Some(arr) = v.get("suggested_deps").and_then(|s| s.as_array()) {
                    let mut seen = std::collections::BTreeSet::new();
                    for s in arr { if let Some(p) = s["package"].as_str() { seen.insert(p.to_string()); } }
                    for p in seen { *sugg_pkgs.entry(p).or_default() += 1; }
                }
                // quirk hotspots — the automation candidates (count each quirk once per repo)
                if let Some(arr) = v.get("receipt").and_then(|r| r["quirks_matched"].as_array()) {
                    for q in arr { if let Some(s) = q.as_str() { *quirk_hot.entry(s.to_string()).or_default() += 1; } }
                }
                // dependency patterns — headers needed / deps missing
                if let Some(dep) = v.get("dependencies") {
                    if let Some(arr) = dep["headers_needed"].as_array() {
                        for h in arr { if let Some(s) = h.as_str() { *hdr_needed.entry(s.to_string()).or_default() += 1; } }
                    }
                    if let Some(arr) = dep["missing"].as_array() {
                        for m in arr { if let Some(s) = m.as_str() { *dep_missing.entry(s.to_string()).or_default() += 1; } }
                    }
                }
                // heavy hitters — configure size as a complexity proxy
                if let Some(de) = v.get("deep_expansion") {
                    let cl = de["configure_lines"].as_u64().unwrap_or(0) as usize;
                    if cl > 0 {
                        let court = v.get("receipt").and_then(|r| r["court_status"].as_str()).unwrap_or("?").to_string();
                        heavy.push((cl, repo.clone(), court));
                    }
                }
                // partial -> full candidates: configure cleared, make failed, and GNU makes it (OURS_BUG_MAKE)
                let court = v.get("receipt").and_then(|r| r["court_status"].as_str()).unwrap_or("");
                if court == "partial" {
                    partial_total += 1;
                    // classify the make failure (the next-front root): all partial repos, by error class.
                    *make_fail_class.entry(classify_make_error(v["diagnostic"].as_str().unwrap_or(""))).or_default() += 1;
                    if v.get("oracle").and_then(|o| o["classification"].as_str()) == Some("OURS_BUG_MAKE") {
                        p2f_make += 1;
                        if let Some(d) = v["diagnostic"].as_str() { if !d.is_empty() {
                            // bucket leaked-macro / undefined-macro diagnostics together by the macro name
                            let bucket = if let Some(pos) = d.find(": command not found") {
                                let pre = &d[..pos];
                                format!("leaked-macro:{}", pre.rsplit([' ', ':']).next().unwrap_or(pre))
                            } else if d.contains("undefined macro") {
                                "undefined-macro".to_string()
                            } else { d.chars().take(40).collect::<String>() };
                            *p2f_diag.entry(bucket).or_default() += 1;
                        }}
                    }
                }
                if let Some(orc) = v.get("oracle") {
                    if let Some(c) = orc["classification"].as_str() { *classif.entry(c.to_string()).or_default() += 1; }
                    if orc["real_configure"].as_str() == Some("ok") { real_clear += 1; }
                }
                // fixable backlog: roots of repos where real got further than ours
                if let Some(dv) = v.get("divergence") {
                    let mut seen = std::collections::BTreeSet::new();
                    if let Some(arr) = dv["macros_ours_left_undefined"].as_array() {
                        for m in arr { if let Some(s) = m.as_str() { seen.insert(format!("macro:{}", s)); } }
                    }
                    if seen.is_empty() {
                        if let Some(e) = dv["ours_error"].as_str() { if !e.is_empty() {
                            seen.insert(bucket_error(e));
                        }}
                    }
                    for s in seen { *fixable_roots.entry(s).or_default() += 1; }
                }
            }
        }
    }
    // rank helper: map -> Vec of {name,repos} sorted desc, top 30
    let rank = |m: &BTreeMap<String, usize>| -> Vec<serde_json::Value> {
        let mut v: Vec<_> = m.iter().map(|(k, c)| (k.clone(), *c)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v.into_iter().take(30).map(|(k, c)| serde_json::json!({"name": k, "repos": c})).collect()
    };
    let heavy_hitters_json: Vec<serde_json::Value> = {
        let mut h = heavy.clone();
        h.sort_by(|a, b| b.0.cmp(&a.0));
        h.into_iter().take(15).map(|(l, r, c)| serde_json::json!({"repo": r, "configure_lines": l, "court": c})).collect()
    };
    let index = serde_json::json!({
        "schema": "automake-rs.build-atlas/index/v2",
        "total": total,
        "by_status": by_status,
        "expansion_bugs": {
            "repos_with_leaked_macros": with_leaks,
            "repos_with_heredoc_imbalance": heredoc_broken,
            "repos_with_conftest_corruption": conftest_corrupt,
            "top_leaked_macros": rank(&leaked),
            "top_syntax_tokens": rank(&syntax_tokens),
            "top_residual_placeholders": rank(&residual),
        },
        "oracle_compass": {
            "ours_configure_clear": ours_clear,
            "real_configure_clear": real_clear,
            "headroom_our_bugs": real_clear.saturating_sub(ours_clear),
            "classification": classif,
            "fixable_backlog_roots": rank(&fixable_roots),
            "died_during_check": rank(&failed_checks),
        },
        "courts": courts.clone(),
        "suggested_packages": rank(&sugg_pkgs),
        "analytics": {
            "quirk_hotspots": rank(&quirk_hot),
            "most_needed_headers": rank(&hdr_needed),
            "most_missing_deps": rank(&dep_missing),
            "heavy_hitters": heavy_hitters_json,
            "partial_to_full": {
                "partial_total": partial_total,
                "ours_bug_make": p2f_make,
                "top_blockers": rank(&p2f_diag),
            },
            "make_failure_roots": rank(&make_fail_class),
        },
        "recipes": lines,
    });
    let _ = std::fs::write(out_dir.join("INDEX.json"), serde_json::to_string_pretty(&index).unwrap_or_default() + "\n");

    // COURTS.md — human-readable gap-analysis summary of the build-court verdicts.
    let mut md = String::new();
    md.push_str("# Build Courts — automake-rs Atlas gap analysis\n\n");
    md.push_str(&format!("Total recipes: **{}**\n\n## Court status\n\n", total));
    md.push_str("| status | count | meaning |\n|---|---|---|\n");
    let meaning = |s: &str| match s {
        "sealed" => "FUNC_OK, no quirks — fully reproduced",
        "quirk_dependent" => "FUNC_OK but needed a quirk rule",
        "partial" => "configure cleared, make failed",
        "not_standalone" => "oracle (GNU) also fails — not our bug",
        "failed" => "ours fails before make",
        _ => "",
    };
    for (k, c) in courts.iter() {
        md.push_str(&format!("| {} | {} | {} |\n", k, c, meaning(k)));
    }
    md.push_str(&format!("\n## Oracle headroom\n\nours configure-clear: **{}** · GNU configure-clear: **{}** · fixable our-bug headroom: **{}**\n\n",
        ours_clear, real_clear, real_clear.saturating_sub(ours_clear)));
    md.push_str("## Top fixable roots (real succeeds, ours fails)\n\n");
    for r in rank(&fixable_roots).iter().take(15) {
        md.push_str(&format!("- {} — {} repos\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }
    md.push_str("\n## Most-needed packages (missing-dep inference)\n\n");
    for r in rank(&sugg_pkgs).iter().take(15) {
        md.push_str(&format!("- {} — {} repos\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }
    md.push_str(&format!("\n## Make-layer roots (the next front: {} partial repos clear configure but fail make)\n\n", partial_total));
    for r in rank(&make_fail_class).iter().take(12) {
        md.push_str(&format!("- {} — {} repos\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }
    let _ = std::fs::write(out_dir.join("COURTS.md"), md);

    // ANALYTICS.md — self-documenting corpus intelligence: quirk hotspots (automation candidates),
    // failure modes, dependency patterns, heavy hitters, and the partial->full shortlist.
    let mut a = String::new();
    a.push_str("# Atlas Analytics — corpus intelligence\n\n");
    a.push_str(&format!("Total recipes: **{}** · court mix: {}\n\n",
        total, courts.iter().map(|(k, c)| format!("{} {}", c, k)).collect::<Vec<_>>().join(", ")));

    a.push_str("## Quirk hotspots (automation candidates)\n\nQuirks matched across recipes — the most frequent are the highest-leverage to auto-apply.\n\n| quirk | repos |\n| --- | --- |\n");
    for r in rank(&quirk_hot).iter().take(15) {
        a.push_str(&format!("| {} | {} |\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }

    a.push_str("\n## Top failure roots (the check ours died on)\n\n| check | repos |\n| --- | --- |\n");
    for r in rank(&failed_checks).iter().take(10) {
        a.push_str(&format!("| {} | {} |\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }

    a.push_str("\n## Dependency patterns\n\n**Most-needed headers**\n\n| header | repos |\n| --- | --- |\n");
    for r in rank(&hdr_needed).iter().take(12) {
        a.push_str(&format!("| {} | {} |\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }
    if !dep_missing.is_empty() {
        a.push_str("\n**Most-missing deps**\n\n| dep | repos |\n| --- | --- |\n");
        for r in rank(&dep_missing).iter().take(12) {
            a.push_str(&format!("| {} | {} |\n", r["name"].as_str().unwrap_or(""), r["repos"]));
        }
    }

    a.push_str("\n## Heavy hitters (configure size = complexity proxy)\n\n| configure lines | repo | court |\n| --- | --- | --- |\n");
    let mut hh = heavy.clone();
    hh.sort_by(|x, y| y.0.cmp(&x.0));
    for (l, r, c) in hh.into_iter().take(12) {
        a.push_str(&format!("| {} | {} | {} |\n", l, r, c));
    }

    a.push_str(&format!("\n## Partial -> full shortlist\n\n**{}** recipes cleared configure but failed make; **{}** of those are `OURS_BUG_MAKE` (GNU makes it, we don't) — the closest wins. Top blockers:\n\n| blocker | repos |\n| --- | --- |\n", partial_total, p2f_make));
    for r in rank(&p2f_diag).iter().take(12) {
        a.push_str(&format!("| {} | {} |\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }
    let _ = std::fs::write(out_dir.join("ANALYTICS.md"), a);

    // RECIPES.md — the working / non-working roster. "Working" = built end-to-end (FUNC_OK, i.e. court
    // sealed or quirk_dependent). Non-working is split: partial (configure cleared, make failed),
    // not_standalone (GNU also fails — not our bug), failed (ours fails before make).
    roster.sort_by(|x, y| x.2.to_lowercase().cmp(&y.2.to_lowercase()));
    let count = |court: &str| roster.iter().filter(|(c, _, _, _)| c == court).count();
    let working = count("sealed") + count("quirk_dependent");
    let mut r = String::new();
    r.push_str("# Atlas Recipes — working / non-working roster\n\n");
    r.push_str(&format!("Total **{}** recipes. **Working (built end-to-end): {}** · non-working: {} partial · {} not-standalone · {} failed.\n\n",
        total, working, count("partial"), count("not_standalone"), count("failed")));
    r.push_str("\"Working\" means the full pipeline (autoreconf → configure → make) succeeded under the GNU-free toolchain. `quirk_dependent` needed an auto-applied quirk; `sealed` needed none.\n\n");

    r.push_str(&format!("## ✅ Working ({})\n\n", working));
    let work_rows: Vec<_> = roster.iter().filter(|(c, _, _, _)| c == "sealed" || c == "quirk_dependent").collect();
    if work_rows.is_empty() {
        r.push_str("_None in this scan slice._\n\n");
    } else {
        r.push_str("| repo | court |\n| --- | --- |\n");
        for (c, _s, repo, _d) in &work_rows { r.push_str(&format!("| {} | {} |\n", repo, c)); }
        r.push('\n');
    }

    for (title, court, with_diag) in [
        ("🟡 Non-working — partial (configure cleared, make failed)", "partial", true),
        ("❌ Non-working — failed (ours fails before make)", "failed", true),
        ("⚪ Non-working — not standalone (GNU autotools also fails; not our bug)", "not_standalone", false),
    ] {
        let rows: Vec<_> = roster.iter().filter(|(c, _, _, _)| c == court).collect();
        r.push_str(&format!("## {} ({})\n\n", title, rows.len()));
        if rows.is_empty() { r.push_str("_None._\n\n"); continue; }
        if with_diag {
            r.push_str("| repo | stage | first error |\n| --- | --- | --- |\n");
            for (_c, s, repo, d) in rows { r.push_str(&format!("| {} | {} | {} |\n", repo, s, d.replace('|', "\\|"))); }
        } else {
            r.push_str("| repo | stage |\n| --- | --- |\n");
            for (_c, s, repo, _d) in rows { r.push_str(&format!("| {} | {} |\n", repo, s)); }
        }
        r.push('\n');
    }
    let _ = std::fs::write(out_dir.join("RECIPES.md"), r);
}

/// `xtask atlas-index <out-dir>` — rebuild INDEX.json from existing recipes (no builds). Used after
/// parallel atlas workers populate the recipe dir.
pub fn index_only() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let out_dir = PathBuf::from(args.get(2).cloned().unwrap_or_else(|| "atlas/recipes".into()));
    write_index(&out_dir);
    println!("atlas-index: rebuilt INDEX.json in {}", out_dir.display());
    ExitCode::SUCCESS
}

/// `xtask atlas-query <term> [out-dir]` — find every recipe that touches a dependency, header,
/// function, macro, package, or quirk. The ecosystem search interface over the corpus.
pub fn query() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let term = match args.get(2) {
        Some(t) => t.to_lowercase(),
        None => {
            eprintln!("usage: xtask atlas-query <term> [out-dir]");
            return ExitCode::FAILURE;
        }
    };
    let out_dir = PathBuf::from(args.get(3).cloned().unwrap_or_else(|| "atlas/recipes".into()));
    let mut hits: Vec<(String, String, String)> = Vec::new(); // repo, where, detail
    for e in std::fs::read_dir(&out_dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("json")
            || p.file_name().and_then(|s| s.to_str()) == Some("INDEX.json") { continue; }
        let txt = match std::fs::read_to_string(&p) { Ok(t) => t, Err(_) => continue };
        let v: serde_json::Value = match serde_json::from_str(&txt) { Ok(v) => v, Err(_) => continue };
        let repo = v["repo"].as_str().unwrap_or("?").to_string();
        let st = v["status"].as_str().unwrap_or("?").to_string();
        let mut found = |where_: &str, detail: String| hits.push((repo.clone(), where_.to_string(), detail));
        let dep = &v["dependencies"];
        for (key, label) in [("pkg_config","pkg-config"),("system_libs","lib"),("headers_needed","header")] {
            if let Some(arr) = dep[key].as_array() {
                for x in arr { if let Some(s)=x.as_str() { if s.to_lowercase().contains(&term) { found(label, format!("{} ({})", s, st)); } } }
            }
        }
        if let Some(obj) = v["probe_results"].as_object() {
            for k in obj.keys() { if k.to_lowercase().contains(&term) { found("probe", format!("{} ({})", k, st)); break; } }
        }
        if let Some(arr) = v["suggested_deps"].as_array() {
            for x in arr { if let Some(s)=x["package"].as_str() { if s.to_lowercase().contains(&term) { found("suggested-pkg", format!("{} ({})", s, st)); } } }
        }
        if let Some(arr) = v["receipt"]["quirks_matched"].as_array() {
            for x in arr { if let Some(s)=x.as_str() { if s.to_lowercase().contains(&term) { found("quirk", format!("{} ({})", s, st)); } } }
        }
        if let Some(de) = v.get("deep_expansion") {
            if let Some(arr) = de["leaked_macros"].as_array() {
                for m in arr { if let Some(s)=m["name"].as_str() { if s.to_lowercase().contains(&term) { found("leaked-macro", format!("{} ({})", s, st)); break; } } }
            }
        }
    }
    println!("atlas-query \"{}\": {} hit(s)", term, hits.len());
    for (repo, where_, detail) in hits.iter().take(100) {
        println!("  {:<40} [{}] {}", repo, where_, detail);
    }
    ExitCode::SUCCESS
}
