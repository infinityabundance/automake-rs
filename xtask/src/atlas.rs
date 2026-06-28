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
                if d.join("configure").exists() {
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
                let (probes, libs, hdrs, pkgs) = collect(&d, &cf_log, &mk_log);
                if status != "FUNC_OK" {
                    diagnostic = diag_line(&cf_log, &mk_log);
                }
                let verified = status == "FUNC_OK";
                let outputs = if verified { collect_outputs(&d) } else { Vec::new() };
                let deep_expansion = analyze_expansion(&d, &cf_log);
                if !libs.is_empty() {
                    quirks.push(format!("LIBS={}", libs.join(" ")));
                }
                write_recipe(
                    &out_dir,
                    &slug,
                    Recipe {
                        schema: "automake-rs.build-atlas/v1",
                        repo: repo.to_string(),
                        source: Source {
                            url: format!("https://github.com/{}", repo),
                            git_sha: git_sha.clone(),
                            snapshot_utc: "2026-06-27".into(),
                        },
                        toolchain: Toolchain {
                            autoconf_rs: ac_ver.clone(),
                            automake_rs: am_ver.clone(),
                            m4_rs_core: "0.1.4".into(),
                            gnu_free: true,
                        },
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
                schema: "automake-rs.build-atlas/v1",
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
        // Heredoc accounting for conftest probes.
        if l.contains("<<_ACEOF") && (l.contains(">conftest") || l.contains("confdefs.h")) {
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
                    if de.syntax_errors.len() < 20 {
                        de.syntax_errors.push(SyntaxError { line: n, token, source });
                    }
                }
            }
        }
    }
    Some(de)
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

fn write_recipe(out_dir: &Path, slug: &str, rec: Recipe) {
    let path = out_dir.join(format!("{}.json", slug));
    if let Ok(s) = serde_json::to_string_pretty(&rec) {
        let _ = std::fs::write(path, s + "\n");
    }
}

fn write_index(out_dir: &Path) {
    let mut by_status: BTreeMap<String, usize> = BTreeMap::new();
    let mut total = 0;
    let mut lines = Vec::new();
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
            }
        }
    }
    let index = serde_json::json!({
        "schema": "automake-rs.build-atlas/index/v1",
        "total": total,
        "by_status": by_status,
        "recipes": lines,
    });
    let _ = std::fs::write(out_dir.join("INDEX.json"), serde_json::to_string_pretty(&index).unwrap_or_default() + "\n");
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
