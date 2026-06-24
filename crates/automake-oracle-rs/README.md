# automake-oracle-rs

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
