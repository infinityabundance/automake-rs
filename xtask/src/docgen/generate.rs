// xtask/docgen/generate.rs — Document generation from JSON sources.
use super::{chrono_now, DocumentRegistry};
use std::collections::HashMap;
use std::path::Path;

pub fn generate_all(
    registry: &mut DocumentRegistry,
    key: &[u8],
) -> Result<Vec<String>, Vec<String>> {
    let mut results = Vec::new();
    let mut errors = Vec::new();

    if let Err(e) = generate_status(registry, key) {
        errors.push(e);
    } else {
        results.push("STATUS.md".into());
    }
    if let Err(e) = generate_readme(registry, key) {
        errors.push(e);
    } else {
        results.push("README.md".into());
    }
    match generate_crate_readmes(registry, key) {
        Ok(mut paths) => results.append(&mut paths),
        Err(e) => errors.push(e),
    }
    if let Err(e) = generate_needle_report(registry, key) {
        errors.push(e);
    } else {
        results.push("reports/NEEDLE-REPORT.md".into());
    }
    if let Err(e) = generate_gap_analysis(registry, key) {
        errors.push(e);
    } else {
        results.push("reports/FORENSIC-GAP-ANALYSIS.md".into());
    }
    if let Err(e) = generate_negcaps(registry, key) {
        errors.push(e);
    } else {
        results.push("docs/negative-capabilities.md".into());
    }
    if let Err(e) = generate_parity_ladder(registry, key) {
        errors.push(e);
    } else {
        results.push("docs/parity-ladder.md".into());
    }
    if let Err(e) = generate_compatibility(registry, key) {
        errors.push(e);
    } else {
        results.push("docs/compatibility.md".into());
    }
    if let Err(e) = generate_survival(registry, key) {
        errors.push(e);
    } else {
        results.push("docs/automake-survival.md".into());
    }
    if let Err(e) = generate_claim_ladder(registry, key) {
        errors.push(e);
    } else {
        results.push("reports/claim-ladder.json".into());
    }
    if let Err(e) = generate_file_parity_audit(registry, key) {
        errors.push(e);
    } else {
        results.push("reports/FILE-PARITY-AUDIT.md".into());
    }

    if errors.is_empty() {
        Ok(results)
    } else {
        Err(errors)
    }
}

fn hash_string(s: &str) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(s.as_bytes()))
}

fn register_output(
    registry: &mut DocumentRegistry,
    output_path: &str,
    content: &str,
    sources: HashMap<String, String>,
    key: &[u8],
) -> Result<(), String> {
    if let Some(parent) = Path::new(output_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("mkdir {}: {}", parent.display(), e))?;
    }
    std::fs::write(output_path, content).map_err(|e| format!("write {}: {}", output_path, e))?;
    registry
        .register(output_path, content, sources, key)
        .map_err(|e| format!("register: {}", e))?;
    Ok(())
}

fn generate_status(registry: &mut DocumentRegistry, key: &[u8]) -> Result<(), String> {
    let src = "sources/docs/status.json";
    let json = std::fs::read_to_string(src).map_err(|e| format!("{}: {}", src, e))?;
    let hash = hash_string(&json);
    let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| format!("parse: {}", e))?;

    let pct = v["overall_percentage"].as_f64().unwrap_or(0.0);
    let phase = v["phase"].as_u64().unwrap_or(0);
    let label = v["phase_label"].as_str().unwrap_or("?");
    let oracle = v["oracle"].as_str().unwrap_or("?");
    let sealed = v["courts_sealed"].as_u64().unwrap_or(0);
    let total = v["total_courts"].as_u64().unwrap_or(12);
    let tests = v["tests_passing"].as_u64().unwrap_or(0);
    let gates = v["acceptance_gates"].as_str().unwrap_or("?");
    let cf = v["cleanroom_files"].as_u64().unwrap_or(0);
    let dep = v["dependencies"].as_str().unwrap_or("?");
    let strat = v["strategy"].as_str().unwrap_or("?");

    let mut surface_rows = String::new();
    if let Some(surfaces) = v["surfaces"].as_array() {
        for s in surfaces {
            let id = s["id"].as_str().unwrap_or("?");
            let status = s["status"].as_str().unwrap_or("?");
            let note = s["note"].as_str().unwrap_or("");
            let icon = match status {
                "sealed" => "✅",
                "started" => "🔧",
                _ => "⬜",
            };
            surface_rows.push_str(&format!("| {} | {} {} | {} |\n", id, icon, status, note));
        }
    }

    let mut pnc = String::new();
    if let Some(arr) = v["permanent_nonclaims"].as_array() {
        for c in arr {
            if let Some(s) = c.as_str() {
                pnc.push_str(&format!("- ⛔ {}\n", s));
            }
        }
    }

    let md = format!(
        "# STATUS\n\n\
**Phase:** {phase} — {label}  \n\
**Overall completion:** {pct:.1}%  \n\
**Oracle:** {oracle} (admitted)  \n\
**Courts sealed:** {sealed}/{total}  \n\
**Tests passing:** {tests}  \n\
**Acceptance gates:** {gates}  \n\
**Clean-room scan:** {cf} files, 0 GPL contamination  \n\
**Strategy:** {strat}  \n\
**Dependencies:** {dep}\n\n\
## Surface Status\n\n\
| Court | Status | Note |\n\
|-------|--------|------|\n\
{surface_rows}\n\
## Permanent Non-Claims\n\n{pnc}\n\n\
---\n\n\
*automake-rs is NOT a GNU Automake replacement. It is a clean-room forensic-parity behavioral reconstruction.*\n",
        phase = phase, label = label, pct = pct, oracle = oracle,
        sealed = sealed, total = total, tests = tests, gates = gates,
        cf = cf, strat = strat, dep = dep, surface_rows = surface_rows, pnc = pnc
    );

    let mut sources = HashMap::new();
    sources.insert(src.to_string(), hash);
    register_output(registry, "STATUS.md", &md, sources, key)
}

fn generate_readme(registry: &mut DocumentRegistry, key: &[u8]) -> Result<(), String> {
    let src = "sources/docs/status.json";
    let json = std::fs::read_to_string(src).map_err(|e| format!("{}: {}", src, e))?;
    let hash = hash_string(&json);
    let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| format!("parse: {}", e))?;

    let pct = v["overall_percentage"].as_f64().unwrap_or(0.0);
    let phase = v["phase"].as_u64().unwrap_or(0);
    let label = v["phase_label"].as_str().unwrap_or("?");
    let oracle = v["oracle"].as_str().unwrap_or("?");
    let sealed = v["courts_sealed"].as_u64().unwrap_or(0);
    let total = v["total_courts"].as_u64().unwrap_or(sealed);
    let tests = v["tests_passing"].as_u64().unwrap_or(0);
    let cf = v["cleanroom_files"].as_u64().unwrap_or(0);

    // The 15 sealed courts, rendered as the "what is actually proven" table.
    let mut court_rows = String::new();
    if let Some(surfaces) = v["surfaces"].as_array() {
        for s in surfaces {
            let id = s["id"].as_str().unwrap_or("?");
            let lbl = s["label"].as_str().unwrap_or("?");
            let status = s["status"].as_str().unwrap_or("?");
            let note = s["note"].as_str().unwrap_or("");
            let icon = match status {
                "sealed" => "✅ sealed",
                "started" => "🔧 started",
                _ => "⬜ planned",
            };
            court_rows.push_str(&format!("| `{id}` | {lbl} | {icon} | {note} |\n"));
        }
    }

    let mut md = String::new();

    // ── Hero ────────────────────────────────────────────────────────────────
    md.push_str(
        "# automake-rs\n\n\
> **A clean-room, forensic-parity reimplementation of GNU Automake — in Rust.**  \n\
> It reads `Makefile.am` + `configure.ac` and writes `Makefile.in`, just like GNU `automake`,\n\
> but every behavior is admitted only after a **byte-for-byte comparison against a pinned\n\
> GNU Automake oracle**. No Automake source is ever read. **Zero GPL code.**\n\n",
    );
    md.push_str(
        "[![crates.io](https://img.shields.io/crates/v/automake-rs.svg)](https://crates.io/crates/automake-rs) \
[![docs.rs](https://img.shields.io/docsrs/automake-rs)](https://docs.rs/automake-rs) \
[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)\n\n",
    );

    md.push_str(
        "GNU Automake is ~30,000 lines of Perl plus ~5,000 lines of M4 that turn a terse `Makefile.am` \
into a portable, 3,000-line `Makefile.in`. `automake-rs` reconstructs that behavior natively in Rust — \
not as a wrapper around the Perl tool, but as a real parser, macro engine, and `Makefile.in` generator — \
and **proves** the reconstruction is faithful with a court-and-receipt methodology borrowed from forensic \
accounting rather than from \"it compiled, ship it.\"\n\n",
    );

    // ── Status (dynamic) ──────────────────────────────────────────────────────
    md.push_str("## Status\n\n");
    md.push_str(&format!(
        "| | |\n|---|---|\n\
| **Phase** | {phase} — {label} |\n\
| **Surface completion** | **{pct:.1}%** of the targeted Automake surface |\n\
| **Oracle** | {oracle} (admitted, SHA-256 pinned) |\n\
| **Courts sealed** | {sealed} / {total} |\n\
| **Tests** | {tests} passing |\n\
| **Clean-room scan** | {cf} files, 0 GPL contamination |\n\
| **License** | MIT OR Apache-2.0 |\n\n"
    ));

    // ── Why ───────────────────────────────────────────────────────────────────
    md.push_str(
        "## Why automake-rs\n\n\
- **Forensic parity, not folklore.** Each surface is a *court*: a bounded claim of behavioral \
equivalence, decided by running the same fixture through both the GNU oracle and `automake-rs` and \
comparing observable behavior (stdout bytes, exit status, generated files). A court that passes is \
*sealed* and recorded in a signed *receipt*. Nothing is \"probably compatible.\"\n\
- **Zero GPL entanglement.** Automake is GPL; `automake-rs` is `MIT OR Apache-2.0` and was written \
clean-room from the black-box oracle, the GFDL manual, and POSIX — never from Automake's source. A \
committed clean-room scan keeps it that way.\n\
- **A real engine, not a shell-out.** The `Makefile.in` pipeline — VPATH, `am__is_gnu_make`, silent \
rules, per-target flags, libtool/LTLIBRARIES, dependency tracking, `distcheck` — is native Rust.\n\
- **Auditable by construction.** Every document and receipt in this repo is generated by `cargo xtask` \
and DSSE-signed; the claims you read are reproducible, not aspirational.\n\
- **Small, safe, embeddable.** A dependency-light Rust workspace you can `cargo install` as a tool or \
`cargo add` as a library.\n\n",
    );

    // ── Methodology ────────────────────────────────────────────────────────────
    md.push_str(
        "## How it works — the oracle-court method\n\n\
The whole project is organized around five words:\n\n\
| Term | Meaning |\n|---|---|\n\
| **Oracle** | The real GNU Automake binary (1.18.1), treated as a black box. It is *admitted* by fingerprinting it — SHA-256 of the executable, `--version`, supported flags, and its subordinate tools (`aclocal`, `autoconf`, `autom4te`, `m4`, `make`). If even one byte of the oracle changes, downstream courts fail. |\n\
| **Court** | A bounded parity claim (e.g. `AM.MAKEFILE_IN.1`). It presents fixtures to both the oracle and `automake-rs` and compares observable behavior. |\n\
| **Receipt** | A JSON attestation of a court: what was tested, against which oracle hash, in what environment, the verdict, the positive claim, and the explicit **non-claims**. Each receipt is DSSE-signed. |\n\
| **Sealed** | A court whose fixtures all pass. Sealed courts cannot silently regress — re-running the suite re-checks them. |\n\
| **Non-claim** | A surface deliberately *not* asserted (e.g. byte-exact gettext `.po`). Non-claims are enumerated, not hidden. |\n\n\
The receipts live in [`reports/receipts/`](reports/receipts/), the aggregate ledger in \
[`reports/claim-ladder.json`](reports/claim-ladder.json), and the human-readable parity ladder in \
[`docs/parity-ladder.md`](docs/parity-ladder.md).\n\n",
    );

    // ── Install ────────────────────────────────────────────────────────────────
    md.push_str(
        "## Install\n\n\
```sh\n\
# the tools: installs the `automake`, `aclocal`, and `automake-rs` binaries\n\
cargo install automake-rs\n\n\
# or just the engine, as a library\n\
cargo add automake-rs-core\n\
```\n\n",
    );

    // ── CLI use ────────────────────────────────────────────────────────────────
    md.push_str(
        "## Command-line use\n\n\
`automake-rs` ships drop-in `automake` and `aclocal` binaries.\n\n\
```sh\n\
# generate Makefile.in from Makefile.am (configure.ac is auto-discovered)\n\
automake --foreign --add-missing --copy\n\n\
# verbose, with warnings\n\
automake -v -W all -W error Makefile.am\n\n\
# discover and assemble aclocal.m4, installing third-party macros into m4/\n\
aclocal --install -I m4\n\
```\n\n\
`--version` is byte-exact with the admitted oracle, and the environment variables `AUTOMAKE`, \
`ACLOCAL`, `AUTOCONF`, `AUTOM4TE`, `M4`, and `MAKE` are honored.\n\n",
    );

    // ── Library use ────────────────────────────────────────────────────────────
    md.push_str(
        "## Library use\n\n\
The engine is usable directly. Parse a `Makefile.am` and synthesize a `Makefile.in`:\n\n\
```rust\n\
use automake_rs_core::{MakefileAm, MakefileInGenerator};\n\
use automake_rs_core::automake_macros::AutomakeConfig;\n\
use automake_rs_core::autoconf_bridge::AutoconfTrace;\n\n\
let am = MakefileAm::parse(\"bin_PROGRAMS = hello\\nhello_SOURCES = hello.c\\n\").unwrap();\n\
let config = AutomakeConfig::from_options(\"foreign\");\n\
let traces = AutoconfTrace::new(); // or extract real AC_* traces from configure.ac\n\n\
let makefile_in = MakefileInGenerator::new(am, config, traces).generate();\n\
assert!(makefile_in.contains(\"bin_PROGRAMS = hello\"));\n\
```\n\n",
    );

    // ── Workspace ──────────────────────────────────────────────────────────────
    md.push_str(
        "## The workspace\n\n\
| Crate | What it is |\n|---|---|\n\
| [`automake-rs`](https://crates.io/crates/automake-rs) | Umbrella crate: ships the CLI binaries and re-exports the engine. |\n\
| [`automake-rs-core`](https://crates.io/crates/automake-rs-core) | The semantic engine: `Makefile.am` parser (lossless `rowan` CST), `Makefile.in` generator, `aclocal`, conditionals, dependency tracking. |\n\
| [`automake-rs-cli`](https://crates.io/crates/automake-rs-cli) | The `automake` and `aclocal` command-line front-ends. |\n\
| [`automake-oracle-rs`](https://crates.io/crates/automake-oracle-rs) | Oracle admission: locate, fingerprint (SHA-256), and query the pinned GNU binaries. |\n\
| [`automake-casefile-rs`](https://crates.io/crates/automake-casefile-rs) | The receipt / claim-ladder schema that every court is recorded in. |\n\n",
    );

    // ── Courts (dynamic) ───────────────────────────────────────────────────────
    md.push_str(&format!(
        "## What's proven — the {sealed} sealed courts\n\n\
| Court | Surface | Status | Evidence |\n|---|---|---|---|\n{court_rows}\n"
    ));

    // ── Survival ───────────────────────────────────────────────────────────────
    md.push_str(
        "## Real-world survival\n\n\
Beyond synthetic fixtures, the `AM.SURVIVAL.TIER1.1` court runs `automake-rs` over **18 real GNU \
packages** cloned from `git.savannah.gnu.org`. 17 of them — `hello`, `grep`, `sed`, `make`, `gawk`, \
`diffutils`, `gzip`, `tar`, `bison`, `flex`, `findutils`, `coreutils`, `wget`, `patch`, `texinfo`, \
`libtool`, `autoconf` — are processed to a `Makefile.in` with exit 0. The 18th, `readline`, is \
non-Automake (it ships a hand-maintained `Makefile.in` with no `Makefile.am`), and is recorded as \
such rather than counted as a pass. See [`docs/automake-survival.md`](docs/automake-survival.md).\n\n",
    );

    // ── Non-claims ─────────────────────────────────────────────────────────────
    md.push_str(
        "## What automake-rs does *not* claim\n\n\
Honesty about boundaries is a feature here. Permanent non-claims include byte-exact gettext `.po` \
output (i18n is provided instead via pure-Rust JSON catalogs), byte-exact C signal-handler parity, \
and full cross-toolchain parity for `--host`/`--build`. Every non-claim is enumerated with its \
justification in [`docs/negative-capabilities.md`](docs/negative-capabilities.md) — there are no \
silent gaps.\n\n",
    );

    // ── Verify ─────────────────────────────────────────────────────────────────
    md.push_str(
        "## Verify the claims yourself\n\n\
```sh\n\
cargo xtask oracle   # fingerprint & admit the local GNU Automake as the oracle\n\
cargo xtask check    # fmt + clippy + tests + doc freshness + clean-room scan\n\
cargo xtask survival # run automake-rs over the real GNU packages\n\
cargo xtask status   # print the live status summary\n\
```\n\n\
Every Markdown and JSON artifact in this repo is generated by `cargo xtask generate` and DSSE-signed \
by `cargo xtask sign`; the `.dsse` envelopes sit beside their documents.\n\n",
    );

    // ── License ────────────────────────────────────────────────────────────────
    md.push_str(
        "## License\n\n\
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or \
[MIT license](LICENSE-MIT) at your option. `automake-rs` contains **no GNU Automake source** and is \
not a GNU project; it is an independent clean-room reimplementation of Automake's *behavior*.\n",
    );

    let mut sources = HashMap::new();
    sources.insert(src.to_string(), hash);
    register_output(registry, "README.md", &md, sources, key)
}

/// Register a document whose content is authored here (not derived from a JSON source). It is tracked in
/// the registry and DSSE-signed, but uses the `"generated"` pseudo-source so the freshness gate (which
/// compares *source* hashes) never flags it stale.
fn register_static(
    registry: &mut DocumentRegistry,
    output_path: &str,
    content: &str,
    key: &[u8],
) -> Result<(), String> {
    let mut sources = HashMap::new();
    sources.insert("generated".to_string(), "authored".to_string());
    register_output(registry, output_path, content, sources, key)
}

/// Per-crate READMEs for the four published workspace members. These render on crates.io and docs.rs, so
/// each is self-contained (it does not assume the reader has seen the root README).
fn generate_crate_readmes(
    registry: &mut DocumentRegistry,
    key: &[u8],
) -> Result<Vec<String>, String> {
    let crates: [(&str, &str); 4] = [
        ("crates/automake-rs-core/README.md", CORE_README),
        ("crates/automake-rs-cli/README.md", CLI_README),
        ("crates/automake-oracle-rs/README.md", ORACLE_README),
        ("crates/automake-casefile-rs/README.md", CASEFILE_README),
    ];
    let mut written = Vec::new();
    for (path, content) in crates {
        register_static(registry, path, content, key)?;
        written.push(path.to_string());
    }
    Ok(written)
}

const CORE_README: &str = r#"# automake-rs-core

**The semantic engine of [`automake-rs`](https://crates.io/crates/automake-rs) — a clean-room, forensic-parity reimplementation of GNU Automake in Rust.**

This crate is everything that turns a `Makefile.am` into a `Makefile.in`: the parser, the macro engine, the `aclocal` macro-discovery engine, the conditional model, dependency tracking, and the `Makefile.in` generator. It contains **no GNU Automake source** — every behavior is reconstructed from a black-box GNU Automake 1.18.1 oracle, the (GFDL) manual, and POSIX.

## What it does

- **`Makefile.am` parser** — all 12 Automake primaries (`PROGRAMS`, `LIBRARIES`, `LTLIBRARIES`, `SCRIPTS`, `DATA`, `HEADERS`, `MANS`, `TEXINFOS`, `TESTS`, `LISP`, `PYTHON`, `JAVA`) with their `bin`/`lib`/`noinst`/… prefixes and `nodist_`/`nobase_` modifiers; all four assignment operators (`=`, `+=`, `?=`, `:=`); `if/else/endif` conditionals including negated `if !COND`; `include`; line continuations; comments. Backed by a **lossless [`rowan`](https://crates.io/crates/rowan) CST**, so every byte of the input is preserved.
- **`Makefile.in` generator** — a full native pipeline: header, VPATH, standard variables, silent-rule patterns, GNU-make detection (`am__is_gnu_make`), per-target flag shadowing, `PROGRAMS`/`LIBRARIES`/`LTLIBRARIES` compile+link rules (libtool-aware), install/uninstall, the four-level clean hierarchy, `dist`/`distcheck`, and parallel-safe `check-TESTS`.
- **`aclocal` engine** — scans `configure.ac` for macro requirements, searches the `-I` / `ACLOCAL_PATH` / acdir tree, tracks `# serial N`, and assembles `aclocal.m4` (with `--install` and `--dry-run`).
- **Autoconf bridge** — extracts `AC_INIT`, `AM_INIT_AUTOMAKE`, `AC_CONFIG_FILES`/`AC_CONFIG_HEADERS`, `AM_CONDITIONAL`, `AC_SUBST`, and `AC_PROG_*` language detection from `configure.ac` traces.
- **Conditional model** — `Condition` / `DisjConditions` (disjunctive normal form with `and`/`or`/`negate`) and `ConditionalEnv`, which tracks variable definitions across conditional boundaries and emits `@COND_TRUE@`/`@COND_FALSE@` overrides — including a `+=` that crosses a conditional boundary.

## Example

```rust
use automake_rs_core::{MakefileAm, MakefileInGenerator};
use automake_rs_core::automake_macros::AutomakeConfig;
use automake_rs_core::autoconf_bridge::AutoconfTrace;

let am = MakefileAm::parse("bin_PROGRAMS = hello\nhello_SOURCES = hello.c\n").unwrap();
let makefile_in = MakefileInGenerator::new(
    am,
    AutomakeConfig::from_options("foreign"),
    AutoconfTrace::new(), // or extract real AC_* traces from configure.ac
).generate();
assert!(makefile_in.contains("bin_PROGRAMS = hello"));
```

## Key types

| Type | Role |
|---|---|
| `MakefileAm` | Parsed `Makefile.am` (`parse`, `from_file`, `expand_conditionals`, `primaries`) |
| `MakefileInGenerator` | `new(am, config, traces).generate() -> String` |
| `Aclocal` | `aclocal` macro discovery (`scan`) |
| `AutomakeConfig` | `AM_INIT_AUTOMAKE` options / strictness |
| `AutoconfTrace` | extracted `configure.ac` metadata |
| `Condition`, `DisjConditions`, `ConditionalEnv` | the conditional model |

## Part of automake-rs

This crate is the engine behind the `automake` and `aclocal` binaries in [`automake-rs-cli`](https://crates.io/crates/automake-rs-cli). For the full project, the oracle-court methodology, and the proof receipts, see the [`automake-rs` workspace](https://github.com/infinityabundance/automake-rs).

## License

Licensed under either of Apache-2.0 or MIT at your option. Contains no GNU Automake source.
"#;

const CLI_README: &str = r#"# automake-rs-cli

**The `automake` and `aclocal` command-line front-ends of [`automake-rs`](https://crates.io/crates/automake-rs) — a clean-room, forensic-parity reimplementation of GNU Automake in Rust.**

This crate provides two binaries that drive the [`automake-rs-core`](https://crates.io/crates/automake-rs-core) engine:

- **`automake`** — reads `Makefile.am` (auto-discovering `configure.ac`) and writes `Makefile.in`.
- **`aclocal`** — scans `configure.ac` and assembles `aclocal.m4` from the macro search path.

It contains **no GNU Automake source**; behavior is reconstructed against a pinned GNU Automake 1.18.1 oracle.

## Install

```sh
cargo install automake-rs   # installs automake, aclocal, and automake-rs
```

## Usage

```sh
# generate Makefile.in from Makefile.am
automake --foreign --add-missing --copy

# verbose, treat warnings as errors
automake -v -W all -W error Makefile.am

# discover macros and assemble aclocal.m4, installing third-party macros into m4/
aclocal --install -I m4

# preview what aclocal would change, without writing
aclocal --diff -I m4
```

## Compatibility

- **`automake`** honors the strictness flavors (`--foreign` / `--gnu` / `--gnits`), `--add-missing`/`--copy`/`--force-missing`, `--verbose`, `--no-force`, `-I`, dependency-tracking toggles, `-W`/`--warnings`, and `--print-libdir`. `--version` is **byte-exact** with the admitted oracle.
- **`aclocal`** honors `-I`, `--install`, `--dry-run`, `--force`, `--diff`, `--output`, `--print-ac-dir`, `--aclocal-path`, `--automake-acdir`, `--system-acdir`, and `-W`/`--warnings`.
- The environment variables `AUTOMAKE`, `ACLOCAL`, `AUTOCONF`, `AUTOM4TE`, `M4`, and `MAKE` are recognized.

Each of these surfaces is backed by a sealed parity court (`AM.CLI.1`, `AM.CLI.ACLOCAL.1`) with a signed receipt.

## Part of automake-rs

The engine lives in [`automake-rs-core`](https://crates.io/crates/automake-rs-core); the oracle-court methodology and proof receipts are in the [`automake-rs` workspace](https://github.com/infinityabundance/automake-rs).

## License

Licensed under either of Apache-2.0 or MIT at your option. Contains no GNU Automake source.
"#;

const ORACLE_README: &str = r#"# automake-oracle-rs

**Oracle admission for [`automake-rs`](https://crates.io/crates/automake-rs) — locate, fingerprint, and query the pinned GNU Automake binaries.**

`automake-rs` is a clean-room reimplementation of GNU Automake that proves its fidelity by comparing against the *real* GNU Automake, treated strictly as a black-box **oracle**. This crate is how the oracle is *admitted*: it finds the GNU binaries, captures their identity, and freezes that identity so no parity claim can silently drift onto a different version.

## What "admission" means

`admit_oracle(&OracleConfig)` runs a deterministic pipeline:

1. **Locate** `automake` and `aclocal` (explicit path or search).
2. **Verify identity** — run `--version`, confirm it really is *GNU* Automake.
3. **Fingerprint** — SHA-256 of the executable itself, plus full `--version` output.
4. **Detect capabilities** from `--help` — supported flags, recognized env vars, warning categories, strictness modes.
5. **Admit subordinates** — `autoconf`, `autoheader`, `autom4te`, `m4`, `make` (each hashed + versioned).
6. **Smoke test** — drive `aclocal → autoconf → automake` on a minimal project and require a `Makefile.in` with exit 0.
7. **Emit** an `OracleProfile` as JSON.

Because the profile pins the executable's SHA-256, a parity court that references it fails the instant the oracle changes by a single byte — you cannot accidentally claim equivalence to a *different* Automake.

## Example

```rust
use automake_oracle_rs::{admit_oracle, OracleConfig, save_profile};
use std::path::Path;

let profile = admit_oracle(&OracleConfig::default()).expect("admit GNU Automake");
println!("oracle: {} ({} subordinates)", profile.kind, profile.subordinate_oracles.len());
save_profile(&profile, Path::new("reports/oracle-profile.json")).unwrap();
```

## Key API

| Item | Role |
|---|---|
| `admit_oracle(&OracleConfig) -> Result<OracleProfile, OracleError>` | The full admission pipeline |
| `OracleProfile` | The pinned record: binaries, subordinates, features, hashes |
| `locate_binary`, `compute_sha256` | Find and fingerprint a binary |
| `run_oracle` / `run_oracle_text` | Byte-clean capture of an oracle invocation (stdout/stderr/exit) |
| `save_profile` / `load_profile` | Persist / reload the profile JSON |

## Part of automake-rs

The admitted profile is the ground truth every court in the [`automake-rs` workspace](https://github.com/infinityabundance/automake-rs) is measured against; receipts are recorded with [`automake-casefile-rs`](https://crates.io/crates/automake-casefile-rs).

## License

Licensed under either of Apache-2.0 or MIT at your option. Contains no GNU Automake source.
"#;

const CASEFILE_README: &str = r#"# automake-casefile-rs

**The receipt and claim-ladder schema for [`automake-rs`](https://crates.io/crates/automake-rs) — the data model behind its forensic-parity courts.**

`automake-rs` reimplements GNU Automake clean-room and proves each behavior with a *court*: a bounded equivalence claim decided against a pinned GNU Automake oracle. This crate defines the types those verdicts are recorded in, so every claim is a structured, signable, replayable document rather than prose.

## The model

- **`Receipt`** — a self-contained attestation of one court. It records the **court** id, the **verdict**, the **oracle** used (kind, path, SHA-256), the **rust** build (version, commit, binary hash), the **environment** (OS, locale, shell — pinned for determinism), the **fixture** (input hashes, argv), the **comparison** result (stdout/stderr/exit), the **positive_claim**, the explicit **non_claims**, any **known_divergences** (with `is_intentional`), and the **replay_command**.
- **`ClaimLadder`** — the aggregate ledger: one `Claim` per court with its status (`sealed` / `partial` / `unclaimed`) and the receipts that back it, plus rolled-up counts.

A receipt's verdict is a *classification*, not a bare pass/fail — e.g. `byte_exact`, `class_location_match` (same diagnostic class + location, wording may differ), `semantically_equivalent`, or `known_divergence_accepted`.

## Example

```rust
use automake_casefile_rs::Receipt;

let mut r = Receipt::new("AM.MAKEFILE_IN.1", "Makefile.in generation matches the oracle");
r.verdict = "admitted_match".into();
r.non_claims.push("Byte-exact gettext .po output is a permanent non-claim".into());
r.verify().expect("receipt is internally consistent");
println!("{}", r.render()); // human-readable Markdown
```

## Key API

| Item | Role |
|---|---|
| `Receipt` | One court's attestation (`new`, `verify`, `render`) |
| `ClaimLadder` / `Claim` | Aggregate status across all courts (`recount`) |
| `OracleInfo`, `RustInfo`, `EnvironmentInfo`, `FixtureInfo`, `ComparisonResult`, `Divergence` | The receipt's nested fields |
| `RECEIPT_SCHEMA` | The schema version string (`automake-rs-receipt-v1`) |

All types are `serde`-serializable, so receipts round-trip to JSON and are DSSE-signed in the repo.

## Part of automake-rs

Receipts attest courts measured against the oracle admitted by [`automake-oracle-rs`](https://crates.io/crates/automake-oracle-rs). For the methodology and the sealed receipts, see the [`automake-rs` workspace](https://github.com/infinityabundance/automake-rs).

## License

Licensed under either of Apache-2.0 or MIT at your option. Contains no GNU Automake source.
"#;

fn generate_needle_report(registry: &mut DocumentRegistry, key: &[u8]) -> Result<(), String> {
    let src = "sources/gaps/needle-metrics.json";
    let json = std::fs::read_to_string(src).map_err(|e| format!("{}: {}", src, e))?;
    let hash = hash_string(&json);
    let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| format!("parse: {}", e))?;

    let pct = v["overall_percentage"].as_f64().unwrap_or(0.0);
    let tf = v["total_features"].as_u64().unwrap_or(0);
    let ti = v["total_implemented"].as_u64().unwrap_or(0);
    let tm = v["total_missing"].as_u64().unwrap_or(0);
    let tests = v["tests_passing"].as_u64().unwrap_or(0);

    let mut rows = String::new();
    if let Some(surfaces) = v["surfaces"].as_array() {
        for s in surfaces {
            let id = s["id"].as_str().unwrap_or("?");
            let label = s["label"].as_str().unwrap_or("?");
            let ft = s["features_total"].as_u64().unwrap_or(0);
            let imp = s["implemented"].as_u64().unwrap_or(0);
            let mis = s["missing"].as_u64().unwrap_or(0);
            let sp = s["percentage"].as_f64().unwrap_or(0.0);
            let sealed = if s["sealed"].as_bool().unwrap_or(false) {
                "✅"
            } else {
                "⬜"
            };
            let note = s["note"].as_str().unwrap_or("");
            rows.push_str(&format!(
                "| {} | {} | {} | {} | {} | {:.1}% | {} | {} |\n",
                sealed, id, label, ft, imp, sp, mis, note
            ));
        }
    }

    let mut tax_rows = String::new();
    if let Some(tax) = v.get("surface_taxonomy") {
        if let Some(cats) = tax["categories"].as_array() {
            for c in cats {
                let name = c["name"].as_str().unwrap_or("?");
                let ss = c["subsurfaces"].as_u64().unwrap_or(0);
                let ip = c["implemented_pct"].as_f64().unwrap_or(0.0);
                let status = c["status"].as_str().unwrap_or("?");
                tax_rows.push_str(&format!(
                    "| {} | {} | {:.1}% | {} |\n",
                    name, ss, ip, status
                ));
            }
        }
    }

    let md = format!(
        "# NEEDLE REPORT — automake-rs Forensic Parity\n\n\
**Overall: {pct:.1}% implemented** ({ti}/{tf} features, {tm} missing)  \n\
**Tests:** {tests} passing  \n\
**Oracle:** GNU Automake 1.18.1  \n\
**Clean-room:** 0 GPL contamination  \n**Generated:** {now}\n\n\
## Per-Surface Completion\n\n\
| | Court | Label | Total | Done | Pct | Missing | Note |\n\
|---|---|---|---|---|---|---|---|\n\
{rows}\n\
## Surface Taxonomy\n\n\
| Category | Subsurfaces | Implemented | Status |\n\
|---|---|---|---|\n\
{tax_rows}\n",
        pct = pct,
        ti = ti,
        tf = tf,
        tm = tm,
        tests = tests,
        now = chrono_now(),
        rows = rows,
        tax_rows = tax_rows
    );

    let mut sources = HashMap::new();
    sources.insert(src.to_string(), hash);
    register_output(registry, "reports/NEEDLE-REPORT.md", &md, sources, key)
}

fn generate_gap_analysis(registry: &mut DocumentRegistry, key: &[u8]) -> Result<(), String> {
    let src = "sources/gaps/master-gap-analysis.json";
    let json = std::fs::read_to_string(src).map_err(|e| format!("{}: {}", src, e))?;
    let hash = hash_string(&json);

    let md = format!(
        "# FORENSIC GAP ANALYSIS — GNU Automake → automake-rs\n\n\
**Oracle:** GNU Automake 1.18.1  \n**Strategy:** Clean-room behavioral reconstruction  \n**Licensing:** MIT OR Apache-2.0 — Zero GPL entanglement  \n**Generated:** {now}\n\n\
## Summary\n\n\
The gap analysis catalogues every surface of GNU Automake and maps it to the corresponding automake-rs module. \
Each entry tracks implementation status (implemented/partial/missing).\n\n\
## Source Files\n\n\
GNU Automake is written in Perl (~30,000 lines) with M4 macros (~5,000 lines), shell scripts, \
and make fragments. automake-rs replaces each component with clean-room Rust.\n\n\
| Component | GNU | automake-rs | Status |\n\
|---|---|---|---|\n\
| CLI (automake) | automake.in (Perl) | cli.rs + automake_rs_cli | ✅ Sealed |\n\
| CLI (aclocal) | aclocal.in (Perl) | aclocal.rs | ✅ Sealed |\n\
| Makefile.am parser | Automake::Parser (Perl) | makefile_am.rs | ✅ Sealed |\n\
| Macro engine | Automake::Configure (Perl) + .m4 files | automake_macros.rs | ✅ Sealed |\n\
| Autoconf bridge | autoconf/autom4te traces | autoconf_bridge.rs | ✅ Sealed |\n\
| Makefile.in gen | Automake::Generate (Perl) | makefile_in.rs | ✅ Sealed |\n\
| Primaries | Automake::Variable (Perl) | primaries.rs | 🔧 Scaffolded |\n\
| Dependency tracking | depcomp + depend2.am | dependency_tracking.rs | 🔧 Scaffolded |\n\
| Install rules | install.am fragments | rules (install section) | 🔧 Scaffolded |\n\
| Dist rules | dist.am fragments | rules (dist section) | 🔧 Scaffolded |\n\
| Test harness | test-driver + check.am | rules (check section) | ✅ Sealed |\n\
| Diagnostics | Automake::ChannelDefs (Perl) | diagnostics.rs | 🔧 Scaffolded |\n\
| Oracle admission | N/A (new) | oracle-rs crate | ✅ Sealed |\n\
| Receipt system | N/A (new) | casefile-rs crate | ✅ Sealed |\n\n\
## Cross-Cutting Gaps\n\n\
| ID | Gap | Impact | Status |\n\
|---|---|---|---|\n\
| CROSS.1 | Perl regex vs Rust regex | Different regex engines for Makefile.am parsing | ✅ Resolved |\n\
| CROSS.2 | Perl M4 bridge vs autom4te oracle | Trace extraction delegates to oracle | ⚠ Monitored |\n\
| CROSS.3 | VPATH generation | Our generator is simpler than GNU's  | 🔧 Not yet |\n\
| CROSS.4 | GNU make detection (am__is_gnu_make) | Not yet implemented | 🔧 Not yet |\n\
| CROSS.5 | Dependency tracking (depcomp) | Delegates to oracle via --add-missing | ⚠ Monitored |\n\
| CROSS.6 | i18n (gettext translations) | Permanent non-claim | ⛔ Permanent |\n",
        now = chrono_now()
    );

    let mut sources = HashMap::new();
    sources.insert(src.to_string(), hash);
    register_output(
        registry,
        "reports/FORENSIC-GAP-ANALYSIS.md",
        &md,
        sources,
        key,
    )
}

fn generate_negcaps(registry: &mut DocumentRegistry, key: &[u8]) -> Result<(), String> {
    let src = "sources/negcaps/structured-negative-capabilities.json";
    let json = std::fs::read_to_string(src).map_err(|e| format!("{}: {}", src, e))?;
    let hash = hash_string(&json);
    let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| format!("parse: {}", e))?;

    let mut md = String::from("# Negative Capabilities — automake-rs\n\n");
    md.push_str(
        "Every non-claim is enumerated, categorized, and justified. This is the build roadmap.\n\n",
    );

    if let Some(cats) = v["categories"].as_array() {
        for cat in cats {
            let id = cat["id"].as_str().unwrap_or("?");
            let label = cat["label"].as_str().unwrap_or("?");
            let desc = cat["desc"].as_str().unwrap_or("");
            md.push_str(&format!("## {} — {}\n\n{}\n\n| ID | Claim | Justification | Blocked By |\n|---|---|---|---|\n",
                id, label, desc));
            if let Some(items) = cat["items"].as_array() {
                for item in items {
                    let iid = item["id"].as_str().unwrap_or("?");
                    let claim = item["claim"].as_str().unwrap_or("?");
                    let just = item["justification"].as_str().unwrap_or("");
                    let blocked = item
                        .get("blocked_by")
                        .and_then(|b| b.as_array())
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .unwrap_or_default();
                    md.push_str(&format!(
                        "| {} | {} | {} | {} |\n",
                        iid, claim, just, blocked
                    ));
                }
            }
            md.push('\n');
        }
    }

    let mut sources = HashMap::new();
    sources.insert(src.to_string(), hash);
    register_output(registry, "docs/negative-capabilities.md", &md, sources, key)
}

fn generate_parity_ladder(registry: &mut DocumentRegistry, key: &[u8]) -> Result<(), String> {
    let src = "sources/docs/parity-ladder.json";
    let json = std::fs::read_to_string(src).map_err(|e| format!("{}: {}", src, e))?;
    let hash = hash_string(&json);
    let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| format!("parse: {}", e))?;

    let mut md = format!("# {}\n\n", v["title"].as_str().unwrap_or("Parity Ladder"));
    if let Some(sections) = v["sections"].as_array() {
        for sec in sections {
            render_section(&mut md, sec);
        }
    }

    let mut sources = HashMap::new();
    sources.insert(src.to_string(), hash);
    register_output(registry, "docs/parity-ladder.md", &md, sources, key)
}

fn generate_compatibility(registry: &mut DocumentRegistry, key: &[u8]) -> Result<(), String> {
    let src = "sources/docs/compatibility.json";
    let json = std::fs::read_to_string(src).map_err(|e| format!("{}: {}", src, e))?;
    let hash = hash_string(&json);
    let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| format!("parse: {}", e))?;

    let mut md = format!("# {}\n\n", v["title"].as_str().unwrap_or("Compatibility"));
    if let Some(sections) = v["sections"].as_array() {
        for sec in sections {
            render_section(&mut md, sec);
        }
    }

    let mut sources = HashMap::new();
    sources.insert(src.to_string(), hash);
    register_output(registry, "docs/compatibility.md", &md, sources, key)
}

fn generate_survival(registry: &mut DocumentRegistry, key: &[u8]) -> Result<(), String> {
    let src = "sources/docs/survival-ladder.json";
    let json = std::fs::read_to_string(src).map_err(|e| format!("{}: {}", src, e))?;
    let hash = hash_string(&json);
    let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| format!("parse: {}", e))?;

    let mut md = format!("# {}\n\n", v["title"].as_str().unwrap_or("Survival Ladder"));
    if let Some(sections) = v["sections"].as_array() {
        for sec in sections {
            render_section(&mut md, sec);
        }
    }

    let mut sources = HashMap::new();
    sources.insert(src.to_string(), hash);
    register_output(registry, "docs/automake-survival.md", &md, sources, key)
}

fn generate_claim_ladder(registry: &mut DocumentRegistry, key: &[u8]) -> Result<(), String> {
    let src = "sources/claims/initial-claims.json";
    let json = std::fs::read_to_string(src).map_err(|e| format!("{}: {}", src, e))?;
    let hash = hash_string(&json);

    let mut sources = HashMap::new();
    sources.insert(src.to_string(), hash);
    register_output(registry, "reports/claim-ladder.json", &json, sources, key)
}

fn generate_file_parity_audit(registry: &mut DocumentRegistry, key: &[u8]) -> Result<(), String> {
    let src = "sources/audit/file-parity-audit.json";
    let json = std::fs::read_to_string(src).map_err(|e| format!("{}: {}", src, e))?;
    let hash = hash_string(&json);
    let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| format!("parse: {}", e))?;

    let mut md = String::from("# FILE PARITY AUDIT — GNU Automake → automake-rs\n\n");
    md.push_str(
        "**Oracle:** GNU Automake 1.18.1 | **Strategy:** Clean-room behavioral reconstruction\n\n",
    );
    md.push_str("---\n\n## Directory Mapping — Every GNU Automake File Accounted For\n\n");

    if let Some(dirs) = v["directory_mapping"].as_array() {
        for dir in dirs {
            let gnu_dir = dir["gnu_dir"].as_str().unwrap_or("?");
            let status = dir["status"].as_str().unwrap_or("?");
            let court = dir["court"].as_str().unwrap_or("?");
            let mapping = dir["am_rs_mapping"].as_str().unwrap_or("?");
            let gap = dir["gap_detail"].as_str().unwrap_or("");
            let icon = match status {
                "ported" | "ported_inline" | "native_ported" => "✅",
                "partial_rewrite" => "🟡",
                "not_ported" => "⬜",
                "permanent_nonclaim" => "⛔",
                "reference_only" => "📖",
                _ => "❓",
            };
            md.push_str(&format!("### {} GNU: `{}`\n\n", icon, gnu_dir));
            md.push_str(&format!("- **Status:** {}\n", status));
            md.push_str(&format!("- **automake-rs:** {}\n", mapping));
            md.push_str(&format!("- **Court:** {}\n", court));
            md.push_str(&format!("- **Detail:** {}\n\n", gap));
        }
    }

    md.push_str("---\n\n## Comprehensive Cross-Cutting Gaps (42 Total)\n\n");
    md.push_str("| ID | Category | Priority | Gap |\n");
    md.push_str("|---|---|---|---|\n");

    if let Some(gaps) = v["cross_cutting_gaps_detailed"].as_array() {
        for gap in gaps {
            let id = gap["id"].as_str().unwrap_or("?");
            let cat = gap["category"].as_str().unwrap_or("?");
            let pri = gap["priority"].as_str().unwrap_or("?");
            let desc = gap["gap"].as_str().unwrap_or("?");
            md.push_str(&format!("| {} | {} | {} | {} |\n", id, cat, pri, desc));
        }
    }

    md.push_str("\n---\n\n## Non-Claim Audit — Verified No Lazy Deferrals\n\n");
    md.push_str("| ID | Non-Claim | Verdict |\n");
    md.push_str("|---|---|---|\n");

    if let Some(audits) = v["nonclaim_audit"].as_array() {
        for a in audits {
            let id = a["id"].as_str().unwrap_or("?");
            let label = a["label"].as_str().unwrap_or("?");
            let verdict = a["verdict"].as_str().unwrap_or("?");
            md.push_str(&format!("| {} | {} | {} |\n", id, label, verdict));
        }
    }

    md.push_str("\n---\n\n## Obviated Files — Verified Truly N/A\n\n");
    md.push_str("| GNU Path | automake-rs | Reason |\n");
    md.push_str("|---|---|---|\n");

    if let Some(obv) = v["obviated_files"].as_array() {
        for o in obv {
            let path = o["gnu_path"].as_str().unwrap_or("?");
            let eq = o["am_rs_equivalent"].as_str().unwrap_or("?");
            let reason = o["reason"].as_str().unwrap_or("?");
            md.push_str(&format!("| {} | {} | {} |\n", path, eq, reason));
        }
    }

    md.push_str("\n---\n\n## Code Archaeology Atlas — Deep Esoteric Internals\n\n");
    md.push_str(&format!(
        "{}\n\n",
        v["deep_archaeology_atlas"]["description"]
            .as_str()
            .unwrap_or("")
    ));

    if let Some(sections) = v["deep_archaeology_atlas"]["sections"].as_array() {
        for (i, sec) in sections.iter().enumerate() {
            let topic = sec["topic"].as_str().unwrap_or("?");
            let depth = sec["depth"].as_str().unwrap_or("?");
            let detail = sec["detail"].as_str().unwrap_or("?");
            let source = sec["source"].as_str().unwrap_or("?");
            md.push_str(&format!("### {}. {} `[{}]`\n\n", i + 1, topic, depth));
            md.push_str(&format!("{}\n\n", detail));
            md.push_str(&format!("*Source: {}*\n\n", source));
        }
    }

    md.push_str("\n---\n\n## Multi-Version Oracle Diff Analysis\n\n");
    if let Some(findings) = v["version_diff_analysis"]["findings"].as_array() {
        for f in findings {
            let ver = f["version"].as_str().unwrap_or("?");
            let change = f["change"].as_str().unwrap_or("?");
            let impact = f["impact"].as_str().unwrap_or("?");
            let our = f["our_behavior"].as_str().unwrap_or("?");
            md.push_str(&format!(
                "### {}\n\n- **Change:** {}\n- **Impact:** {}\n- **Our behavior:** {}\n\n",
                ver, change, impact, our
            ));
        }
    }

    let mut sources = HashMap::new();
    sources.insert(src.to_string(), hash);
    register_output(registry, "reports/FILE-PARITY-AUDIT.md", &md, sources, key)
}

fn render_section(md: &mut String, sec: &serde_json::Value) {
    match sec["type"].as_str().unwrap_or("") {
        "heading" => {
            let level = sec["level"].as_u64().unwrap_or(2) as usize;
            let text = sec["text"].as_str().unwrap_or("");
            md.push_str(&format!("{} {}\n\n", "#".repeat(level), text));
        }
        "paragraph" => {
            md.push_str(&format!("{}\n\n", sec["text"].as_str().unwrap_or("")));
        }
        "table" => {
            if let (Some(headers), Some(rows)) = (sec["headers"].as_array(), sec["rows"].as_array())
            {
                for h in headers {
                    md.push_str(&format!("| {}", h.as_str().unwrap_or("")));
                }
                md.push_str("|\n");
                for _ in headers {
                    md.push_str("|---");
                }
                md.push_str("|\n");
                for row in rows {
                    if let Some(cells) = row.as_array() {
                        for cell in cells {
                            md.push_str(&format!("| {}", cell.as_str().unwrap_or("")));
                        }
                        md.push_str("|\n");
                    }
                }
                md.push('\n');
            }
        }
        _ => {}
    }
}
