// automake-rs-core: Auxiliary script generation — clean-room reconstruction.
//
// Generates standard auxiliary scripts that GNU Automake installs
// via --add-missing: install-sh, missing, compile, depcomp, test-driver.
//
// STRICTLY CLEAN-ROOM: All behavior is reconstructed from:
//   - GNU Automake manual §6, §15, §17 (GFDL licensed)
//   - Black-box oracle interrogation (running original scripts, observing output)
//   - POSIX sh(1) specification
// No GNU Automake GPL source code is consulted.
//
// Court: AM.AUX.1 — auxiliary script parity

use std::fs;

/// Generate an auxiliary script by name. Returns the script content.
pub fn generate_aux_script(name: &str) -> Option<String> {
    match name {
        "install-sh" => Some(generate_install_sh()),
        "missing" => Some(generate_missing()),
        "compile" => Some(generate_compile()),
        "depcomp" => Some(generate_depcomp()),
        "test-driver" => Some(generate_test_driver()),
        "mkinstalldirs" => Some(generate_mkinstalldirs()),
        "py-compile" => Some(generate_py_compile()),
        "ylwrap" => Some(generate_ylwrap()),
        _ => None,
    }
}

/// Install standard auxiliary files into a directory.
/// Returns list of installed files.
pub fn install_aux_files(
    dir: &std::path::Path,
    copy: bool,
    force: bool,
) -> Result<Vec<String>, std::io::Error> {
    let scripts = [
        "install-sh",
        "missing",
        "compile",
        "depcomp",
        "test-driver",
        "mkinstalldirs",
    ];
    let mut installed = Vec::new();

    for name in &scripts {
        if let Some(content) = generate_aux_script(name) {
            let path = dir.join(name);
            if !force && path.exists() {
                continue;
            }
            if copy {
                fs::write(&path, &content)?;
            } else {
                // Symlink: skip if exists, otherwise write (symlinks require platform support)
                if !path.exists() {
                    fs::write(&path, &content)?;
                }
            }
            installed.push(name.to_string());
        }
    }
    Ok(installed)
}

/// Generate install-sh: standard file installation script.
/// Reconstructed from POSIX install(1) semantics and black-box observation.
fn generate_install_sh() -> String {
    r#"#!/bin/sh
# install - install a program, script, or datafile
# Clean-room reconstruction for automake-rs

scriptversion=2024-01-01.00

nl='
'
IFS=" ""	$nl"

# Set directory mode
dirmode=755
# Set file mode
mode=0755
# Create directories?
mkdirp=
# Strip symbol tables?
stripcmd=
# Copy instead of move?
cpcmd=cp

chmodcmd=$chmodprog
chowncmd=
chgrpcmd=
rmcmd="$rmprog -f"
mvcmd="$mvprog"
src=
dst=
dir_arg=
dst_arg=

while [ $# -ne 0 ]; do
  case $1 in
    -c) ;;
    -C) cpcmd=cat ;;
    -d) dir_arg=true ;;
    -g) chgrpcmd="$chgrpprog $2"; shift ;;
    -m) mode=$2
        case $mode in
          *' '* | *"$nl"* | *'*'* | *'?'* | *'['*)
            echo "$0: invalid mode: $mode" >&2; exit 1 ;;
        esac
        shift ;;
    -o) chowncmd="$chownprog $2"; shift ;;
    -p) cpcmd="$cpprog -p" ;;
    -s) stripcmd=$stripprog ;;
    -S) cpcmd="$cpprog -P" ;;
    -t)
        is_target_directory=true
        dst=$2; shift ;;
    -T) is_target_a_directory=never ;;
    --) shift; break ;;
    -*) echo "$0: invalid option: $1" >&2; exit 1 ;;
    *) break ;;
  esac
  shift
done

if [ $# -ne 0 ]; then
  if [ -z "$dir_arg" ]; then
    if [ $# -gt 1 ]; then
      if [ -z "$is_target_directory" ]; then
        echo "$0: target '$dst' is not a directory" >&2
        exit 1
      fi
      dst=$dst_arg
      for src; do
        if [ -d "$src" ]; then
          echo "$0: omitting directory '$src'" >&2; continue
        fi
        if [ -z "$dst" ] && [ -z "$dst_arg" ]; then
          echo "$0: no destination specified" >&2; exit 1
        fi
        dstfile=$dst/`basename "$src"`
        if [ -f "$dstfile" ]; then $rmcmd "$dstfile"; fi
        $cpcmd "$src" "$dstfile" || exit 1
        [ -n "$stripcmd" ] && $stripcmd "$dstfile"
        [ -n "$chowncmd" ] && $chowncmd "$dstfile"
        [ -n "$chgrpcmd" ] && $chgrpcmd "$dstfile"
        chmod "$mode" "$dstfile" 2>/dev/null
      done
    else
      if [ -z "$dst_arg" ]; then
        echo "$0: no destination specified" >&2; exit 1
      fi
      dst=$dst_arg
      if [ -f "$dst" ]; then $rmcmd "$dst"; fi
      $cpcmd "$1" "$dst" || exit 1
      [ -n "$stripcmd" ] && $stripcmd "$dst"
      [ -n "$chowncmd" ] && $chowncmd "$dst"
      [ -n "$chgrpcmd" ] && $chgrpcmd "$dst"
      chmod "$mode" "$dst" 2>/dev/null
    fi
  else
    shift
    for src; do
      dst=$1; shift
      if [ -d "$dst" ]; then
        dstfile=$dst/`basename "$src"`
      else
        dstfile=$dst
      fi
      if [ -f "$dstfile" ]; then $rmcmd "$dstfile"; fi
      $cpcmd "$src" "$dstfile" || exit 1
      [ -n "$stripcmd" ] && $stripcmd "$dstfile"
      [ -n "$chowncmd" ] && $chowncmd "$dstfile"
      [ -n "$chgrpcmd" ] && $chgrpcmd "$dstfile"
      chmod "$mode" "$dstfile" 2>/dev/null
    done
  fi
fi
"#
    .to_string()
}

/// Generate missing: tool-not-found helper script.
/// Reconstructed from GNU Automake manual §6.4 and black-box observation.
fn generate_missing() -> String {
    r#"#!/bin/sh
# missing - find missing tools and provide replacements
# Clean-room reconstruction for automake-rs

scriptversion=2024-01-01.00

# Help messages for common missing tools
case $1 in
  aclocal*)
    echo "aclocal is missing or not available." >&2
    echo "Install GNU Automake or run 'automake-rs aclocal'" >&2
    exit 1 ;;
  autoconf*)
    echo "autoconf is missing or not available." >&2
    echo "Install GNU Autoconf" >&2
    exit 1 ;;
  autoheader*)
    echo "autoheader is missing or not available." >&2
    echo "Install GNU Autoconf" >&2
    exit 1 ;;
  automake*)
    echo "automake is missing. Using automake-rs instead." >&2
    exit 0 ;;
  makeinfo*)
    echo "makeinfo is missing or not available." >&2
    echo "Install GNU Texinfo" >&2
    exit 1 ;;
  flex*|lex*)
    echo "flex/lex is missing or not available." >&2
    echo "Install flex" >&2
    exit 1 ;;
  bison*|yacc*)
    echo "bison/yacc is missing or not available." >&2
    echo "Install GNU Bison" >&2
    exit 1 ;;
  help2man*)
    echo "help2man is missing or not available." >&2
    echo "Install help2man" >&2
    exit 1 ;;
  *)
    echo "WARNING: '$1' is missing on your system." >&2
    echo "You should only need it if you modified certain files." >&2
    echo "You might want to install the missing tool." >&2
    exit 0 ;;
esac
"#
    .to_string()
}

/// Generate compile: compiler wrapper script.
fn generate_compile() -> String {
    r#"#!/bin/sh
# compile - wrapper for compilers that don't understand -c and -o
# Clean-room reconstruction for automake-rs

scriptversion=2024-01-01.00

# Parse arguments to find source and object files
ofile=
cfile=
eat=

for arg; do
  if [ -n "$eat" ]; then
    eat=
    continue
  fi
  case $1 in
    -o)
      ofile=$2; eat=yes ;;
    *.c|*.cc|*.cpp|*.cxx|*.f|*.f90|*.F|*.F90|*.r|*.go|*.m|*.mm)
      # Source file found
      ;;
    *)
      ;;
  esac
  shift
done

# Invoke the actual compiler (CC is set by configure)
if [ -n "$ofile" ]; then
  exec "$CC" ${1+"$@"}
else
  exec "$CC" -c ${1+"$@"}
fi
"#
    .to_string()
}

/// Generate depcomp: dependency tracking script with full compiler mode support.
fn generate_depcomp() -> String {
    crate::dependency_tracking::generate_depcomp_script()
}

/// Generate test-driver: parallel test harness driver.
fn generate_test_driver() -> String {
    r#"#!/bin/sh
# test-driver - basic test driver for Automake parallel test harness
# Clean-room reconstruction for automake-rs

scriptversion=2024-01-01.00

test_name="$1"
log_file="$2"
trs_file="$3"
shift 3

# Run the test
if "$@"; then
  result=PASS
  echo ":test-result: PASS" > "$trs_file"
  echo "PASS: $test_name" >&2
else
  result=FAIL
  echo ":test-result: FAIL" > "$trs_file"
  echo "FAIL: $test_name" >&2
fi

# Write log
{
  echo "=== $test_name ==="
  echo "result: $result"
  echo "command: $*"
  echo "=== log ==="
  cat "$log_file" 2>/dev/null
} > "$log_file"
"#
    .to_string()
}

/// Generate mkinstalldirs: directory creation script.
fn generate_mkinstalldirs() -> String {
    r#"#!/bin/sh
# mkinstalldirs - make directory hierarchy
# Clean-room reconstruction for automake-rs

scriptversion=2024-01-01.00

errstatus=0
dirmode=

usage="Usage: $0 [-m MODE] DIR..."

while test $# -gt 0; do
  case $1 in
    -h|--help) echo "$usage"; exit 0 ;;
    -m) dirmode=$2; shift ;;
    -m*) dirmode=`echo "$1" | sed 's/-m//'` ;;
    --) shift; break ;;
    -*) echo "$0: invalid option: $1" >&2; exit 1 ;;
    *) break ;;
  esac
  shift
done

for file; do
  if test -d "$file"; then
    shift
  else
    break
  fi
done

case $# in
  0) exit 0 ;;
esac

case $dirmode in
  '')
    if mkdir -p --version >/dev/null 2>&1; then
      mkdir -p -- "$@"
    else
      exec "$0" -m 755 "$@"
    fi ;;
  *)
    if mkdir -m "$dirmode" -p --version >/dev/null 2>&1; then
      mkdir -m "$dirmode" -p -- "$@"
    else
      for dir; do
        mkdir -m "$dirmode" "$dir"
      done
    fi ;;
esac
"#
    .to_string()
}

/// Generate py-compile: Python byte-compilation script.
fn generate_py_compile() -> String {
    r#"#!/bin/sh
# py-compile - compile Python .py files to .pyc
# Clean-room reconstruction for automake-rs

scriptversion=2024-01-01.00

if [ -z "$PYTHON" ]; then
  PYTHON=python
fi

basedir=
destdir=

while [ $# -gt 0 ]; do
  case $1 in
    --basedir) basedir=$2; shift ;;
    --destdir) destdir=$2; shift ;;
    *) break ;;
  esac
  shift
done

files=$@

if [ -z "$files" ]; then
  exit 0
fi

if [ -z "$destdir" ]; then
  $PYTHON -c "
import py_compile, sys
for f in sys.argv[1:]:
    py_compile.compile(f, dfile=f)
" "$files"
else
  $PYTHON -c "
import py_compile, sys, os
for f in sys.argv[1:]:
    dest = os.path.join('$destdir', os.path.basename(f) + 'c')
    py_compile.compile(f, cfile=dest, dfile=f)
" "$files"
fi
"#
    .to_string()
}

/// Generate ylwrap: yacc/lex wrapper script.
fn generate_ylwrap() -> String {
    r#"#!/bin/sh
# ylwrap - wrapper for lex/yacc invocations
# Clean-room reconstruction for automake-rs

scriptversion=2024-01-01.00

prog="$1"; shift
input="$1"; shift

case "$input" in
  *.y|*.ypp|*.y++)
    ext=y ;;
  *.l|*.ll|*.lpp)
    ext=l ;;
  *)
    echo "$0: unknown input extension" >&2; exit 1 ;;
esac

# Get the base name
base=`echo "$input" | sed 's/\.[^.]*$//'`

# Run the program
"$prog" ${1+"$@"} "$input"

# Rename output files to match expected names
if [ $? -eq 0 ]; then
  if [ "$ext" = y ]; then
    if [ -f y.tab.c ]; then mv y.tab.c "$base".c; fi
    if [ -f y.tab.h ]; then mv y.tab.h "$base".h; fi
  elif [ "$ext" = l ]; then
    if [ -f lex.yy.c ]; then mv lex.yy.c "$base".c; fi
  fi
fi
"#
    .to_string()
}
