# automake-rs-cli

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
