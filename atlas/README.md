# automake-rs build-atlas

A **build-profiler atlas + build-court** (schema v2): every point in the build corpus is turned into a
reproducible, versioned recipe (`recipes/<owner>__<name>.json`). Each recipe pins the source (git SHA)
and the GNU-free toolchain, and records:

- **probe results** (config.h `HAVE_*`) and a **probe trace** with *why* each probe passed/failed
  (header-not-found / symbol-not-found / link-failed)
- the **dependency graph** (system libs / pkg-config / headers needed / missing) plus
  **missing-dep inference** (`suggested_deps`: failed probe → providing package)
- the **pass pipeline**, **target settings**, **quirks matched** and **quirks auto-applied**
  (a matched quirk applies a GNU-free fix, re-runs, and records whether it helped)
- the **verified outputs** (sha256) and a **sealed receipt** (court status + sha256 hash-chain)
- the **GNU-autotools oracle** result on the same repo, and a **classification** of the failure
- **deep expansion forensics** + the exact **syntax/macro context** to debug it

The point: a future build *reads the recipe* — installs the recorded deps, applies the known pipeline
and quirks, and reproduces the verified output. And the **oracle compass** turns the corpus into a
ranked fix-list: it knows which failures are *our bugs* (the real GNU toolchain succeeds where we don't)
versus *not standalone* (GNU fails too), so effort goes where it compounds.

## The compass (what makes success compound)

With `ATLAS_ORACLE=1`, each recipe is classified against the real GNU toolchain run on a git-reset copy:

| court_status | meaning |
| --- | --- |
| `sealed` | `FUNC_OK`, no quirks — fully reproduced, matches GNU |
| `quirk_dependent` | `FUNC_OK` but a quirk had to be applied |
| `partial` | configure cleared, make failed |
| `not_standalone` | the GNU oracle **also** fails — not our bug |
| `failed` | ours fails before make |

`recipes/INDEX.json` aggregates this into `oracle_compass` — `ours_configure_clear` vs
`real_configure_clear`, the `headroom_our_bugs` gap, and the `fixable_backlog_roots` /
`died_during_check` rankings — and `COURTS.md` is the human-readable gap analysis. That's the
ranked backlog: defeat the top root, the headroom shrinks, the next pass starts from rendered evidence.

## Notes

- **GNU-free**: only `autoreconf-rs` / `acrs-*` are invoked for the build; `toolchain.gnu_free: true`
  asserts no GNU autotools binary ran. The `oracle` block runs the real GNU toolchain **only for
  comparison**, on a separate git-reset tree — it never counts toward the build.
- **Schema**: see [SCHEMA.md](SCHEMA.md) (`automake-rs.build-atlas/v2`).
- **Regenerate**: `cargo xtask atlas <corpus-list> [out-dir]` — Rust only, no scripts
  (`xtask/src/atlas.rs`). `ATLAS_ORACLE=1` adds the oracle/court fields; `ATLAS_SCAN_ONLY=1` does a
  fast generate-only expansion sweep (no configure-run/make).
- **Query**: `cargo xtask atlas-query <term>` finds every recipe touching a dep / header / probe /
  package / quirk / macro.
- **Re-index**: `cargo xtask atlas-index <out-dir>` rebuilds `INDEX.json` + `COURTS.md` from existing
  recipes (no builds).
