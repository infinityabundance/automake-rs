// autoreconf-rs — the native bootstrap driver (BOOTSTRAP.1).
//
// Orchestrates the GNU-free Autotools pipeline, mirroring `autoreconf -fi`:
//   1. aclocal-rs       configure.ac (+ m4 dirs) -> aclocal.m4
//   2. autoconf-rs      configure.ac (+ aclocal.m4) -> configure
//   3. autoheader-rs    configure.ac -> config.h.in   (when AC_CONFIG_HEADERS present)
//   4. automake-rs      --add-missing                  (native aux files + receipt)
//   5. automake-rs      every Makefile.am -> Makefile.in
//
// Doctrine (NATIVE.1): no GNU tool is invoked. The configure/aclocal/autoheader stages are
// delegated to the autoconf-rs native tools, resolved by explicit env override or by name; if a
// resolved tool turns out to be GNU (or is missing under --forbid-gnu), the step FAILS CLOSED with
// typed evidence — never a silent GNU fallback. A bootstrap receipt records every provider and
// asserts `gnu_tools_invoked: []`.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Resolution of one external native stage tool.
struct Tool {
    /// Logical stage name (for the receipt / errors).
    stage: &'static str,
    /// Resolved executable path, if found.
    path: Option<PathBuf>,
    /// Whether the resolved tool looks like GNU (a forbidden provider).
    is_gnu: bool,
}

/// Resolve a native tool: prefer the explicit env override, then the autoconf-rs conventional
/// names, then the bare name. A name resolving to a GNU build is flagged (and rejected later).
fn resolve_tool(stage: &'static str, env_var: &str, candidates: &[&str]) -> Tool {
    let mut tried: Vec<String> = Vec::new();
    if let Ok(p) = std::env::var(env_var) {
        tried.push(p);
    }
    for c in candidates {
        tried.push(c.to_string());
    }
    for cand in &tried {
        if let Some(path) = which(cand) {
            let is_gnu = looks_like_gnu(&path);
            return Tool { stage, path: Some(path), is_gnu };
        }
    }
    Tool { stage, path: None, is_gnu: false }
}

/// Minimal PATH lookup (also accepts an absolute/relative path that exists).
fn which(name: &str) -> Option<PathBuf> {
    let p = Path::new(name);
    if p.is_absolute() || name.contains('/') {
        return if p.exists() { Some(p.to_path_buf()) } else { None };
    }
    let path = std::env::var("PATH").unwrap_or_default();
    for dir in path.split(':') {
        let cand = Path::new(dir).join(name);
        if cand.exists() {
            return Some(cand);
        }
    }
    None
}

/// Heuristic: does this tool identify as GNU? (`--version` mentions GNU and not "rs").
fn looks_like_gnu(path: &Path) -> bool {
    let out = Command::new(path).arg("--version").output();
    if let Ok(o) = out {
        let v = String::from_utf8_lossy(&o.stdout).to_lowercase();
        let is_rs = v.contains("-rs") || v.contains("rust") || v.contains("automake-rs");
        return v.contains("gnu") && !is_rs;
    }
    false
}

/// Result of a bootstrap run (for the receipt + exit code).
pub struct BootstrapReport {
    pub ok: bool,
    pub receipt_json: String,
}

/// Run the native bootstrap in `dir`. `forbid_gnu` makes a GNU-resolved or missing native tool a
/// hard, typed failure (the default for the "no GNU dependency" claim).
pub fn run_bootstrap(dir: &Path, forbid_gnu: bool, verbose: bool) -> BootstrapReport {
    let cf = ["configure.ac", "configure.in"]
        .iter()
        .map(|n| dir.join(n))
        .find(|p| p.exists());
    let cf = match cf {
        Some(p) => p,
        None => {
            return BootstrapReport {
                ok: false,
                receipt_json: err_receipt("no configure.ac / configure.in found"),
            }
        }
    };
    let ac_text = std::fs::read_to_string(&cf).unwrap_or_default();
    let needs_header = ac_text.contains("AC_CONFIG_HEADERS")
        || ac_text.contains("AC_CONFIG_HEADER")
        || ac_text.contains("AM_CONFIG_HEADER");

    // Resolve the native stage tools. automake-rs is self (this binary's own generator).
    let aclocal = resolve_tool("aclocal", "ACLOCAL_RS", &["aclocal-rs", "acrs-aclocal", "aclocal"]);
    let autoconf = resolve_tool("autoconf", "AUTOCONF_RS", &["autoconf-rs", "acrs-autoconf", "autoconf"]);
    let autoheader = resolve_tool("autoheader", "AUTOHEADER_RS", &["autoheader-rs", "acrs-autoheader", "autoheader"]);

    let mut steps: Vec<String> = Vec::new();
    let mut ok = true;
    let mut run = |tool: &Tool, args: &[&str], stdout_to: Option<&Path>| -> bool {
        // Fail closed: missing native tool, or a GNU tool under --forbid-gnu.
        let path = match &tool.path {
            Some(p) => p,
            None => {
                steps.push(step_json(tool.stage, "missing", "no native provider found", None));
                return false;
            }
        };
        if tool.is_gnu && forbid_gnu {
            steps.push(step_json(tool.stage, "rejected-gnu", &path.display().to_string(), None));
            return false;
        }
        let mut cmd = Command::new(path);
        cmd.current_dir(dir).args(args);
        let output = cmd.output();
        match output {
            Ok(o) => {
                if let Some(dest) = stdout_to {
                    let _ = std::fs::write(dir.join(dest), &o.stdout);
                }
                let provider = if tool.is_gnu { "GNU (allowed: --forbid-gnu off)" } else { "native (autoconf-rs)" };
                let st = if o.status.success() { "ok" } else { "nonzero-exit" };
                steps.push(step_json(tool.stage, st, &path.display().to_string(), Some(provider)));
                o.status.success()
            }
            Err(e) => {
                steps.push(step_json(tool.stage, "spawn-failed", &e.to_string(), None));
                false
            }
        }
    };

    if verbose { eprintln!("autoreconf-rs: aclocal -> aclocal.m4"); }
    // aclocal-rs writes aclocal.m4 itself (its -o default). Do NOT also capture its stdout into
    // aclocal.m4 — that clobbered the real file with empty stdout, leaving 0-line aclocal.m4 so
    // autoconf had no macro definitions to prepend (AX_*/AM_* "command not found"). Let it write.
    let _ = run(&aclocal, &[cf.file_name().unwrap().to_str().unwrap()], None);

    if verbose { eprintln!("autoreconf-rs: autoconf -> configure"); }
    let cfg_ok = run(&autoconf, &[cf.file_name().unwrap().to_str().unwrap()], Some(Path::new("configure")));
    if cfg_ok {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(dir.join("configure"), std::fs::Permissions::from_mode(0o755));
        }
    } else {
        ok = false;
    }

    if needs_header {
        // The header template name is NOT always config.h.in — AC_CONFIG_HEADERS([ethtool-config.h])
        // means autoheader must write ethtool-config.h.in (else config.status creates an empty
        // header and the program sees `PACKAGE undeclared`). Parse the configured header name.
        let header = config_header_name(&ac_text);
        let header_in = format!("{}.in", header);
        if verbose { eprintln!("autoreconf-rs: autoheader -> {}", header_in); }
        // autoheader-rs writes <header>.in ITSELF (and GNU autoheader does too). Pass None — do NOT
        // capture its stdout into header_in, which is EMPTY (it prints only a stderr diagnostic) and
        // CLOBBERED the good template with a 0-byte file. config.status then had no `#undef PACKAGE`
        // to convert, so a config-header project using bare PACKAGE/VERSION (dangerousben/jsonval:
        // `printf(..., PACKAGE)`) failed `make` with `'PACKAGE' undeclared`. Same clobber the aclocal
        // step above already avoids; autoheader wrote the header name from config_header_name().
        let _ = run(&autoheader, &[cf.file_name().unwrap().to_str().unwrap()], None);
    }

    // Steps 4 + 5 (aux + Makefile.in) are automake-rs itself; the caller runs them after this
    // returns, so the receipt records them as native-self.
    steps.push(step_json("aux-files", "delegated", "automake-rs --add-missing (native)", Some("native (automake-rs)")));
    steps.push(step_json("makefile.in", "delegated", "automake-rs (native)", Some("native (automake-rs)")));

    let gnu_used: Vec<&str> = [&aclocal, &autoconf, &autoheader]
        .iter()
        .filter(|t| t.is_gnu && !forbid_gnu && t.path.is_some())
        .map(|t| t.stage)
        .collect();

    let receipt = format!(
        "{{\n  \"native_bootstrap\": {},\n  \"forbid_gnu\": {},\n  \"gnu_tools_invoked\": [{}],\n  \"providers\": {{\n    \"aclocal\": {},\n    \"autoconf\": {},\n    \"autoheader\": {},\n    \"aux\": \"automake-rs\",\n    \"makefile_in\": \"automake-rs\"\n  }},\n  \"steps\": [\n{}\n  ]\n}}\n",
        ok && gnu_used.is_empty(),
        forbid_gnu,
        gnu_used.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(", "),
        provider_str(&aclocal, forbid_gnu),
        provider_str(&autoconf, forbid_gnu),
        provider_str(&autoheader, forbid_gnu),
        steps.iter().map(|s| format!("    {}", s)).collect::<Vec<_>>().join(",\n"),
    );
    let _ = std::fs::write(dir.join("bootstrap-receipt.json"), &receipt);
    BootstrapReport { ok, receipt_json: receipt }
}

fn provider_str(t: &Tool, forbid_gnu: bool) -> String {
    match &t.path {
        None => "\"unresolved\"".to_string(),
        Some(p) => {
            let kind = if t.is_gnu {
                if forbid_gnu { "rejected-gnu" } else { "gnu" }
            } else {
                "native"
            };
            format!("{{ \"path\": {:?}, \"kind\": {:?} }}", p.display().to_string(), kind)
        }
    }
}

fn step_json(stage: &str, status: &str, detail: &str, provider: Option<&str>) -> String {
    match provider {
        Some(p) => format!(
            "{{ \"stage\": {:?}, \"status\": {:?}, \"detail\": {:?}, \"provider\": {:?} }}",
            stage, status, detail, p
        ),
        None => format!(
            "{{ \"stage\": {:?}, \"status\": {:?}, \"detail\": {:?} }}",
            stage, status, detail
        ),
    }
}

fn err_receipt(msg: &str) -> String {
    format!("{{ \"native_bootstrap\": false, \"error\": {:?} }}\n", msg)
}

/// Extract the first config-header name from AC_CONFIG_HEADERS / AC_CONFIG_HEADER / AM_CONFIG_HEADER
/// (e.g. `ethtool-config.h`), defaulting to `config.h`. Only the first whitespace-separated token of
/// the first arg names the header to generate.
fn config_header_name(ac_text: &str) -> String {
    for macro_name in ["AC_CONFIG_HEADERS", "AC_CONFIG_HEADER", "AM_CONFIG_HEADER"] {
        if let Some(pos) = ac_text.find(macro_name) {
            let after = &ac_text[pos + macro_name.len()..];
            if let Some(open) = after.find('(') {
                if let Some(close) = after[open..].find(')') {
                    let inner = &after[open + 1..open + close];
                    let first = inner
                        .trim()
                        .trim_start_matches('[')
                        .split([']', ',', ' ', '\t', '\n'])
                        .next()
                        .unwrap_or("")
                        .trim();
                    if !first.is_empty() {
                        return first.to_string();
                    }
                }
            }
        }
    }
    "config.h".to_string()
}
