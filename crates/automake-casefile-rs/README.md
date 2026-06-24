# automake-casefile-rs

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
