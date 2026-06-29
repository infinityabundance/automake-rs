# automake-rs build-atlas — recipe schema v3

Every corpus point becomes a reproducible, versioned **build-court record**. One JSON file per repo
under `recipes/<owner>__<name>.json`. v2 grows the v1 cache (probe results + deps + quirks) into a
full auditable knowledge record: a sealed receipt, the GNU-autotools **oracle differential** (so a
failure is classified as *our bug* vs *not-standalone*), deep expansion forensics, and the exact
syntactic/macro context needed to debug — captured so a bug is readable from the recipe without
re-running anything on a VM.

```jsonc
{
  "schema": "automake-rs.build-atlas/v3",
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

## directory_context (deep subdir/multi-directory context)

The multi-directory build structure that drives — and breaks — the make layer. Captured by walking the
cloned tree + parsing configure.ac and every Makefile.am, so a make failure is debuggable from the
recipe without re-cloning.

```jsonc
"directory_context": {
  "config_files": [ {"path":"src/lib/Makefile","depth":2,"top_builddir":"../..","has_template":true} ],
  "config_headers": ["config.h"],                 // AC_CONFIG_HEADERS targets
  "subdirs": ["src","man","lib","hesinfo"],        // the SUBDIRS recursion tree
  "build_dirs": [ {"dir":"src/lib","targets":["libhesiod.la","hestest"],"subdirs":[],
                   "sources_include_config_h":true,"am_cppflags":""} ],
  "max_depth": 2,
  "config_h_consumers_below_root": ["src/lib"]     // dirs that #include config.h but sit BELOW where it's
                                                   // generated -> their -I$(top_builddir) MUST be correct
}
```

- `config_files[].top_builddir` is the **correct** relative `..`-path for that file's depth — a subdir
  Makefile needs `..` (not `.`), or `-I$(top_builddir)` in DEFAULT_INCLUDES misses a top-level `config.h`
  ("config.h: No such file" — the make-layer root for SUBDIRS projects, fixed in autoconf-rs 0.1.19).
- `config_h_consumers_below_root` names exactly the directories where the relative-path logic is
  load-bearing — the cross-directory context that pinpoints a SUBDIRS make failure.

## v3 deep-context fields (Automake provenance + Makefile pathology + environment)

v3 turns the atlas from configure-autopsy into a full diagnostic+repair system. Every field below is
optional (populated only when relevant) so recipes stay backward-compatible.

- **`makefile_forensics`** — per generated Makefile: `first_parse_error` (line/kind/text/previous_lines/
  `probable_cause` ∈ lost-tab | unexpanded-var | unexpanded-automake-token | bare-macro |
  shell-fragment-in-make), `unexpanded_vars` (`@LIBOBJS@`…), `unexpanded_automake_tokens`
  (`%reldir%`/`$(am__`), `recipe_tab_anomalies`. Turns `missing separator` from a string into a bug class.
- **`make_graph`** — `targets`, `key_variables` (CC/CFLAGS/LDFLAGS/LIBS/DEFAULT_INCLUDES as the Makefile
  sets them), `generated_files` (Makefile/config.status/config.h/libtool), `recursion_depth`.
- **`macro_inventory`** — `macro_dirs`, `defined_macros` (name/source/kind from m4/+acinclude.m4),
  `called_macros`, `unresolved_macros` (called AX_/custom macros with no local def → the fix-class driver).
- **`source_to_generated_map`** — provenance: `configure_origins` (leaked macro → configure.ac:line),
  `m4_trace_depth` (engine stack/divergence risk), `shadowed_macros` (local overrides of standard macros).
- **`conditional_context`** — generated-configure if/fi + case/esac counts + `balanced`, AM_CONDITIONAL names.
- **`config_aux_inventory`** — aux_dir + present/missing of install-sh/missing/depcomp/compile/config.guess/
  sub/ltmain.sh (makes replay prescriptive: synthesize the missing helpers).
- **`tool_requirements`** — build-time executables `detected` + `missing` (name/phase/suggested_package);
  the command-not-found cluster becomes auto-dep hints.
- **`language_surface`** — `source_suffixes`, configure compiler macros, `needs_cxx`/`needs_fortran`,
  `sets_c_std` (else GCC14+/Clang18+ default-strict risk).
- **`toolchain_interaction`** — compiler + version, `c_std_default_risk`, sampled `-D` defines.
- **`libtool_context`** — uses_libtool, macros, ltmain_present, libtool_m4_sources, age (old|modern).
- **`gettext_intl_context`** — uses_gettext/intltool, po_dir_present, missing support files (config.rpath…).
- **`vpath_analysis`** — hardcoded `./`/`$(srcdir)/` paths, abs-path leakage, BUILT_SOURCES, yacc/lex
  generated-source targets (VPATH/distcheck + parallel-build hazards).
- **`feature_probe_gap`** — headers AC_CHECK'd vs headers actually `#include`d-but-unchecked (musl/clean-
  distro assumption risk), implicit `-l` libs not routed via PKG_CHECK_MODULES/AC_SEARCH_LIBS.
- **`quirk_history`** — applied/matched quirks + effect (the learnable autotools-wisdom log).
- **`verification`** — `vs_gnu` (identical-status | ours-better | ours-worse | both-fail), `replay_success`,
  `drift_noise` (non-claim acceptable-noise classes).
- **`repair_hints`** — ranked, evidence-backed fix candidates (id/phase/confidence/evidence/action/
  expected_effect) derived from all the above — the self-training repair corpus.
- **`environment`** (enriched) — host_triplet, kernel_version, libc {name,version}, pkg_config_path,
  env_vars_influential, posix_flavor (gnu|bsd), shell, oracle_tool_versions (autoconf/automake/m4/perl),
  env_var_whitelist.

## v3.1 deterministic-envelope fields + toolchain interceptor

- **`dialect_reconciliation`** — `enforce_standards_tier` (c89_gnu/c99/c11/gnu++), `strip_modern_poison_flags`
  (`-Werror=implicit-function-declaration`…), `inject_legacy_shims`, `compiler_aliasing` (gcc-14+ →
  `-std=gnu89 -fpermissive`). The containment policy for modern-compiler drift on vintage code.
- **`m4_side_effect_isolation`** — `unquoted_subst_in_conditional`, `shadowed_builtins`,
  `permitted_mutations` (AC_ARG_ENABLE/WITH) vs `suspect_global_mutations`.
- **`parallel_build_safety`** — `vpath_out_of_tree_safe`, `generators` (yacc/lex/protoc/gperf),
  `unordered_generated_sources` (gen-source not in BUILT_SOURCES → `make -j` race), `built_sources_declared`.
- **`host_environment_veil`** — `header_injection_candidates` (drifted headers needing mock/fallback),
  `symbol_aliasing_candidates` (obsolete symbols: sys_errlist→strerror, bzero/index…).
- **`semantic_context`** — included headers, `undefined_symbols` (from link errors), provided symbols,
  `llvm_native_preview` (null until native codegen).
- **`make_graph`** (enriched) — + `top_targets`, `make_diagnostics` (command/error_type/message:
  command-not-found | compiler-error | linker-error | missing-header | make-syntax).
- **`verification`** (enriched) — + `output_match`, `test_suite_pass_rate`.
- **`quirk_history`** — + `effectiveness` (high/medium/low/unknown).
- **`risk_factors`** — top-level brittle-aspect list (subdir-config-h-include-path, unresolved-macros,
  ancient-libtool, modern-compiler-strictness, deep-macro-expansion, parallel-build-race, vpath-unsafe).

### Toolchain interceptor shim (`ATLAS_SHIM=1`)

A PATH-proxy that makes vintage code build under GCC14+/Clang18+ **without mutating any Makefile**
(forensic byte-parity preserved). Before the `make` step, a temp dir of compiler shims (cc/gcc/clang/
c++/g++/clang++) is prepended to `PATH`; each shim strips modern poison `-Werror` flags and appends
legacy-leniency (`-Wno-error=implicit-function-declaration`/`int-conversion`/`incompatible-pointer-types`
+ `-fcommon`; `-fpermissive` for C++), then `exec`s the real `/usr/bin` compiler. Env-gated for A/B
measurement via `atlas-diff`. **Proven**: legacy C that errors `implicit declaration of 'puts'` +
multiple-definition compiles cleanly under the shim. The `dialect_reconciliation` block is the per-recipe
policy that drives it.

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

## v3.2 scenario context (the stateful-observer snapshot)

Captures the *scenario*, not just the static project — so a divergence from the GNU oracle can be
explained by a host/state mismatch.

**`environment`** (further enriched, host-level):
- `shell_flavor` (dash/bash/zsh/posix) + `shell_echo_n_works` + `shell_supports_local` — shell dialect
  fingerprint (the Oracle's generated quoting/echo logic depends on it)
- `fs_case_sensitive`, `fs_supports_symlinks`, `fs_supports_hardlinks`, `fs_max_path` — filesystem
  strictness (dist/install/VPATH behavior)
- `install_sh_version`, `libtool_version`, `gettext_version` — sub-tool capabilities + plugin versions
- `poison_vars_present` vs `poison_vars_confirmed_unset` — the *negative context* (GREP_OPTIONS/CDPATH/
  CLICOLOR/POSIXLY_CORRECT/MAKEFLAGS/IFS confirmed absent, the famous silent Automake breakers)

**`scenario_context`** (per-recipe):
- `temporal_map` — relative mtime offsets (ms) of the rebuild-trigger inputs vs configure.ac (Automake's
  dependency clock; drives the maintainer rebuild rules)
- `m4_ancestry` — each called macro → its resolved source (local m4/ file, acinclude.m4, system-aclocal,
  or unresolved) — the macro-parity ancestry
- `rebuild_trigger_risk` — aclocal.m4/configure newer than configure.ac (would re-trigger autoreconf)

**`oracle.internal_traces`** — the GNU autoreconf/automake DECISION log (installing aux files, libtoolize/
aclocal/automake choices, macro requires) — compares the *path taken*, not just the final artifact.

## v3.3 variable_indirection (the "No rule to make target" root context)

Makefile.am primaries (`*_PROGRAMS`/`*_LIBRARIES`/`*_SOURCES`/`*_LDADD`/`*_LIBADD`) whose value is a
`$(var)` reference rather than a literal list. Automake resolves these to generate per-target object/link
rules; if the toolchain doesn't, the targets get **no rules** → `make: No rule to make target 'X.o'`.

```jsonc
"variable_indirection": {
  "indirect_primaries": [
    { "primary": "check_PROGRAMS", "var": "tests", "resolved_to": ["test_rtsp","test_wpa"], "resolved": true },
    { "primary": "test_rtsp_SOURCES", "var": "test_sources", "resolved_to": ["test/test_common.h"], "resolved": true },
    { "primary": "libshl_la_LIBADD", "var": "AM_LIBADD", "resolved_to": [], "resolved": false }
  ],
  "unresolved_refs": ["libshl_la_LIBADD = $(AM_LIBADD)"],
  "indirection_count": 6
}
```

Each primary→var→resolved-list makes the indirection surface explicit. `resolved:false` / `unresolved_refs`
are the highest make-fail risk. autoconf-rs/automake-rs 0.1.14 resolves `$(var)` program lists so the
per-target rules are generated (e.g. `check_PROGRAMS += $(tests)` → builds test_rtsp/test_wpa).
