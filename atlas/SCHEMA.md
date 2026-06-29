# automake-rs build-atlas — recipe schema v2

Every corpus point becomes a reproducible, versioned **build-court record**. One JSON file per repo
under `recipes/<owner>__<name>.json`. v2 grows the v1 cache (probe results + deps + quirks) into a
full auditable knowledge record: a sealed receipt, the GNU-autotools **oracle differential** (so a
failure is classified as *our bug* vs *not-standalone*), deep expansion forensics, and the exact
syntactic/macro context needed to debug — captured so a bug is readable from the recipe without
re-running anything on a VM.

```jsonc
{
  "schema": "automake-rs.build-atlas/v2",
  "repo": "owner/name",
  "source":    { "url", "git_sha", "snapshot_utc" },      // reproducible pin
  "toolchain": { "autoconf_rs", "automake_rs", "m4_rs_core", "gnu_free": true },
  "target":    { "cc", "cflags", "host" },
  "pass_pipeline": [ {"step","tool","status"} ... ],       // steps that ran (autoreconf/configure/make/auto-quirk)
  "probe_results": { "HAVE_*": 1, ... },                   // config.h feature-probe outcomes
  "feature_flags": { "configure_args": [...] },
  "dependencies": { "pkg_config":[], "system_libs":[], "headers_needed":[], "missing":[] },
  "quirks": [ "human-readable notes" ],
  "outputs": [ {"path","sha256","kind"} ... ],             // verified build artifacts
  "status": "FUNC_OK | MAKE_FAIL | CONFIGURE_RUN_FAIL | CONFIGURE_GEN_FAIL | NO_AC | CLONE_FAIL",
  "verified": true|false,
  "diagnostic": "first hard error line",

  // ---- v2 additions ----

  // Sealed build-court receipt: makes the recipe auditable.
  "receipt": {
    "court_status": "sealed | quirk_dependent | partial | not_standalone | failed",
    "probe_trace":  [ {"name":"HAVE_FOO_H","kind":"header|func|lib","result":"yes|no",
                       "reason":"ok|header-not-found|symbol-not-found|link-failed|probe-returned-no"} ],
    "quirks_matched": [ "uses-libtool", "uses-pkg-config", "vendored-aclocal", ... ],
    "quirks_applied": [ {"id","action":"--disable-maintainer-mode","verified":true} ], // auto-apply
    "receipt_hash": "sha256 hash-chain over toolchain + probes + outputs + oracle verdict",
    "schema": "automake-rs.build-court/v1"
  },

  // The GNU-autotools ORACLE run on the same repo (git-reset tree). Only with ATLAS_ORACLE=1.
  // This is the compass: classification says whether a failure is OURS to fix.
  "oracle": {
    "real_autoreconf": "ok|fail", "real_configure": "ok|fail|skipped", "real_make": "ok|fail|skipped",
    "real_configure_lines": N, "real_first_error": "...",
    "classification": "BOTH_OK | OURS_BUG_CONFIGURE | OURS_BUG_MAKE | OURS_GEN_FAIL |
                       NOT_STANDALONE | BOTH_CONFIGURE_FAIL | BOTH_MAKE_FAIL | OURS_BETTER"
  },

  // Where ours diverges from a real run that got further — only for OURS_BUG_* (the fixable backlog).
  "divergence": {
    "stage": "autoreconf|configure|make",
    "ours_error": "...", "ours_error_context": [...],
    "ours_configure_lines": N, "real_configure_lines": M,
    "macros_ours_left_undefined": [ "the macros to define/fix" ]
  },

  // Deep-expansion forensics on the GENERATED configure (no re-spelunking needed).
  "deep_expansion": {
    "configure_lines": N,
    "leaked_macros": [ {"name":"AC_FOO","line":N,"context":"..."} ], // unexpanded AC_/AX_/AM_/m4_ calls
    "heredoc_openers": N, "heredoc_terminators": N, "heredoc_imbalance": I, // missing-conftest-opener bug
    "syntax_errors": [ {"line":N,"token":"(","source":"...",
                        "block":["the enclosing if/case/heredoc construct"]} ],
    "cache_var_anomalies": [ "malformed ${...} refs" ],
    "residual_placeholders": [ "@VAR@ config.status never filled" ],
    "failed_during_check": "checking for X...",   // the probe ours died on (root, not the cascade line)
    "failure_tail": [ "run-log lines after the last checking message" ],
    "conftest_corruption": [ "line N: # <x> (include eaten)" ], // m4 ate include/ifdef/define in C
    "conftest_directives_intact": N, "conftest_directives_mangled": N,
    "conftest_programs": [ "the exact C conftest source the macros emitted" ]
  },

  // Build-environment fingerprint (hermeticity): the toolchain + env that shaped the build.
  "environment": { "cc","cc_version","host_triplet","pkg_config_version","make_version","relevant_env":[...] },

  // Missing-dep inference: failed header/lib probe -> the providing package.
  "suggested_deps": [ {"missing":"zlib.h","kind":"header","package":"zlib1g-dev"} ]
}
```

## court_status

- **sealed** — `FUNC_OK`, no quirks needed: fully reproduced, matches the oracle.
- **quirk_dependent** — `FUNC_OK` but a quirk rule had to be applied.
- **partial** — configure cleared, make failed.
- **not_standalone** — the GNU oracle *also* fails (e.g. external macro archive not in the repo); not our bug.
- **failed** — ours fails before make.

## INDEX.json (aggregate)

`recipes/INDEX.json` (schema `automake-rs.build-atlas/index/v2`) rolls the recipes up:

- `by_status`, `courts` — status / court_status counts
- `expansion_bugs` — top leaked macros / syntax tokens / residual placeholders, repos with conftest corruption
- `oracle_compass` — `ours_configure_clear` vs `real_configure_clear`, `headroom_our_bugs`, classification
  counts, `fixable_backlog_roots` (leaked macro / syntax token, via `bucket_error`), `died_during_check`
- `suggested_packages` — missing-dep → package inference
- `analytics` — self-documenting corpus intelligence:
  - `quirk_hotspots` (quirks_matched tallied → the auto-apply backlog)
  - `most_needed_headers` / `most_missing_deps`
  - `heavy_hitters` (configure size; surfaces runaway-expansion bugs)
  - `partial_to_full` (`partial_total`, `ours_bug_make`, `top_blockers`) — the closest wins
  - `make_failure_roots` — the make-layer "next front": partial-repo make errors by class
    (`no-rule-to-make-target`, `undefined-reference`, `missing-header-at-compile`, `command-not-found`,
    `makefile/shell-syntax-error`, `compiler-error`, …)

Generated docs (regenerated on every index): `COURTS.md` (court table + headroom + top fixable roots +
needed packages + make-layer roots), `ANALYTICS.md` (the analytics block), `RECIPES.md` (working /
non-working roster).

GNU-free: only `autoreconf-rs` / `acrs-*` are invoked for the build; `toolchain.gnu_free:true` asserts
no GNU autotools binary ran. The `oracle` block runs the real GNU toolchain **only for comparison**, on
a separate git-reset tree — it never counts toward the GNU-free build. (The autoconf-rs toolchain's
leaked-macro *neutralizer* is on by default; opt out with `AUTOCONF_RS_NO_NEUTRALIZE=1`.)

## Replay receipt (`automake-rs.replay-receipt/v1`)

`cargo xtask atlas-replay <recipe>` emits a separate receipt verifying a reproduction:
`replay_status` (`reproduced` | `reproduced_no_outputs` | `diverged` | `build_failed` | `clone_failed`),
`pinned_sha`/`sha_replayed`, `configure_args`, per-step `steps`, and `output_verification`
(per-path match / hash-mismatch / missing vs the recipe's `outputs`).

## Commands

- `cargo xtask atlas <corpus-list> [out-dir]` — scan. `ATLAS_ORACLE=1` adds oracle/court fields;
  `ATLAS_SCAN_ONLY=1` is a fast generate-only expansion sweep.
- `cargo xtask atlas-index <out-dir>` — rebuild INDEX + COURTS.md + ANALYTICS.md + RECIPES.md (no builds).
- `cargo xtask atlas-query <term>` — find every recipe touching a dep/header/probe/package/quirk/macro.
- `cargo xtask atlas-replay <recipe | owner/name | slug> [--keep]` — reproduce + verify a recipe.
- `cargo xtask atlas-diff <baseline-dir> <experiment-dir>` — A/B the court verdicts (flips/regressions/net).
