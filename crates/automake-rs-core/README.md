# automake-rs-core

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
