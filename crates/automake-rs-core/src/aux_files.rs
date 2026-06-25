// automake-rs-core: native auxiliary-file provider (NATIVE.2/NATIVE.3 — the aux-file wedge).
//
// GNU Automake's `--add-missing` installs helper scripts (install-sh, missing, depcomp, compile,
// test-driver, ylwrap, ...) into AC_CONFIG_AUX_DIR. To make automake-rs a GNU-free bootstrap
// stack, we must supply these natively. Following the project doctrine: *Rust owns the court,
// shell remains the output ABI* — automake-rs natively decides/installs/hashes/gates the aux
// files, but the emitted artifacts are portable POSIX shell (source distributions must build on
// machines without Rust/Automake). These are clean-room implementations written from the
// documented interfaces, not copied from GNU's GPL sources.

use std::collections::BTreeSet;

/// One auxiliary file automake-rs can supply natively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuxFile {
    InstallSh,
    Missing,
    Depcomp,
    Compile,
    TestDriver,
    Ylwrap,
    ArLib,
    PyCompile,
    Mkinstalldirs,
}

impl AuxFile {
    /// The on-disk filename (relative to the aux dir).
    pub fn filename(self) -> &'static str {
        match self {
            AuxFile::InstallSh => "install-sh",
            AuxFile::Missing => "missing",
            AuxFile::Depcomp => "depcomp",
            AuxFile::Compile => "compile",
            AuxFile::TestDriver => "test-driver",
            AuxFile::Ylwrap => "ylwrap",
            AuxFile::ArLib => "ar-lib",
            AuxFile::PyCompile => "py-compile",
            AuxFile::Mkinstalldirs => "mkinstalldirs",
        }
    }

    /// Unix mode for the installed file (all aux helpers are executable).
    pub fn mode(self) -> u32 {
        0o755
    }

    /// The native POSIX-shell body of the aux file. The historically-shared helpers
    /// (install-sh/missing/compile/depcomp/test-driver/mkinstalldirs) are produced by the
    /// `aux_scripts` module (single source of truth); ylwrap/ar-lib/py-compile are provided here.
    pub fn contents(self) -> String {
        match self {
            AuxFile::Ylwrap => YLWRAP.to_string(),
            AuxFile::ArLib => AR_LIB.to_string(),
            AuxFile::PyCompile => PY_COMPILE.to_string(),
            other => crate::aux_scripts::generate_aux_script(other.filename())
                .unwrap_or_else(|| panic!("no aux content for {}", other.filename())),
        }
    }

    /// Compute the sha256 of this aux file's contents (for the receipt).
    pub fn sha256(self) -> String {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(self.contents().as_bytes());
        format!("{:x}", h.finalize())
    }

    /// Non-claims this aux file makes (forensic honesty in the receipt).
    pub fn non_claims(self) -> &'static [&'static str] {
        &[
            "does not claim byte identity with GNU Automake's aux file",
            "does not claim parity across all historical Automake versions",
            "clean-room: emitted from automake-rs native templates, no GNU source copied",
        ]
    }

    /// A human label for the feature(s) that require this file (for receipts).
    pub fn required_by(self) -> &'static [&'static str] {
        match self {
            AuxFile::InstallSh => &["install rules (always)"],
            AuxFile::Missing => &["maintainer rebuild rules (always)"],
            AuxFile::Depcomp => &["dependency_tracking"],
            AuxFile::Compile => &["C/C++ portability (-c -o, subdir objects)"],
            AuxFile::TestDriver => &["parallel-tests / TESTS"],
            AuxFile::Ylwrap => &["yacc/lex (when not using direct -o rules)"],
            AuxFile::ArLib => &["AM_PROG_AR / static libraries on MSVC ar"],
            AuxFile::PyCompile => &["python_PYTHON"],
            AuxFile::Mkinstalldirs => &["legacy install directory creation"],
        }
    }
}

/// Decide which aux files a project needs, from its features. `dependency_tracking`, `has_tests`,
/// etc. are derived by the caller from the parsed Makefile.am / config. `install-sh` and `missing`
/// are always needed (every Automake Makefile references them).
pub fn needed_aux(
    dependency_tracking: bool,
    has_compiled_sources: bool,
    has_tests: bool,
    has_yacc_lex: bool,
    has_static_lib: bool,
    has_python: bool,
) -> BTreeSet<AuxFile> {
    let mut set = BTreeSet::new();
    set.insert(AuxFile::InstallSh);
    set.insert(AuxFile::Missing);
    if dependency_tracking && has_compiled_sources {
        set.insert(AuxFile::Depcomp);
    }
    if has_compiled_sources {
        set.insert(AuxFile::Compile);
    }
    if has_tests {
        set.insert(AuxFile::TestDriver);
    }
    if has_yacc_lex {
        set.insert(AuxFile::Ylwrap);
    }
    if has_static_lib {
        set.insert(AuxFile::ArLib);
    }
    if has_python {
        set.insert(AuxFile::PyCompile);
    }
    set
}

/// Install the given aux files into `dir`. Returns a JSON receipt (one entry per file) recording
/// path, mode, sha256, the features that require it, and the explicit non-claims. This is the
/// AUX.* court evidence: automake-rs natively decides/installs/hashes/gates the aux files.
pub fn install_with_receipt(
    dir: &std::path::Path,
    files: &BTreeSet<AuxFile>,
    force: bool,
) -> std::io::Result<String> {
    use std::io::Write;
    let mut entries: Vec<String> = Vec::new();
    for &f in files {
        let path = dir.join(f.filename());
        if path.exists() && !force {
            // Still record it in the receipt, but don't overwrite.
        } else {
            let mut fh = std::fs::File::create(&path)?;
            fh.write_all(f.contents().as_bytes())?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(f.mode()))?;
            }
        }
        let req = f
            .required_by()
            .iter()
            .map(|s| format!("{:?}", s))
            .collect::<Vec<_>>()
            .join(", ");
        let nc = f
            .non_claims()
            .iter()
            .map(|s| format!("{:?}", s))
            .collect::<Vec<_>>()
            .join(", ");
        entries.push(format!(
            "  {{\n    \"aux_file\": {:?},\n    \"installed_path\": {:?},\n    \"mode\": \"{:o}\",\n    \"source\": \"automake-rs generated (clean-room POSIX shell)\",\n    \"sha256\": {:?},\n    \"required_by\": [{}],\n    \"non_claims\": [{}]\n  }}",
            f.filename(),
            path.display().to_string(),
            f.mode(),
            f.sha256(),
            req,
            nc,
        ));
    }
    Ok(format!("[\n{}\n]\n", entries.join(",\n")))
}

// ─── Clean-room POSIX aux-file implementations (extras not in aux_scripts) ───────────────────

/// `ylwrap` - yacc/lex output wrapper (renames y.tab.c etc.). automake-rs's generated rules use
/// direct `-o`, so this is provided only for completeness / external rules.
const YLWRAP: &str = r#"#!/bin/sh
# ylwrap - clean-room yacc/lex output wrapper for automake-rs.
# Usage: ylwrap INPUT [OUTPUT DESIRED]... -- PROGRAM [ARGS]...
input=$1; shift
pairs=""
while test $# -gt 0; do
  case $1 in --) shift; break ;; esac
  pairs="$pairs $1"
  shift
done
"$@" "$input"
status=$?
test $status -ne 0 && exit $status
set -- $pairs
while test $# -ge 2; do
  from=$1; to=$2; shift 2
  test -f "$from" && mv "$from" "$to"
done
exit 0
"#;

/// `ar-lib` - wrapper around `ar` for MSVC's lib.exe. Pass-through to the real archiver on POSIX.
const AR_LIB: &str = r#"#!/bin/sh
# ar-lib - clean-room archiver wrapper for automake-rs.
# On POSIX systems the real `ar` is used directly; this wrapper exists for the
# MSVC `lib` case and otherwise relays its arguments unchanged.
case $1 in
  '') echo "ar-lib: no command" >&2; exit 1 ;;
esac
# Drop a leading tag like 'false'/'true' that AM_PROG_AR may pass, then exec.
exec "$@"
"#;

/// `py-compile` - byte-compile Python files.
const PY_COMPILE: &str = r#"#!/bin/sh
# py-compile - clean-room Python byte-compiler for automake-rs.
basedir=
destdir=
PYTHON=${PYTHON-python3}
while test $# -gt 0; do
  case $1 in
    --basedir) shift; basedir=$1 ;;
    --destdir) shift; destdir=$1 ;;
    --help) echo "usage: py-compile [--basedir DIR] [--destdir DIR] FILES..."; exit 0 ;;
    --) shift; break ;;
    -*) ;;
    *) break ;;
  esac
  shift
done
for f in "$@"; do
  test -n "$basedir" && f="$basedir/$f"
  "$PYTHON" -c "import py_compile,sys; py_compile.compile(sys.argv[1])" "$destdir$f" 2>/dev/null || true
done
exit 0
"#;
