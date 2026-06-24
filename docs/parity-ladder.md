# Parity Ladder — automake-rs vs GNU Automake 1.18.1

Tracks which surfaces automake-rs claims to match against GNU Automake 1.18.1. Generated from `reports/claim-ladder.json`. All 14 courts SEALED at 100%. 131 tests. 27/32 packages. 159/159 features. 42 documented gaps tracked in `reports/FILE-PARITY-AUDIT.md`.

## Legend

| Status| Count| Meaning|
|---|---|---|
| ✅ sealed| 14| Oracle-admitted with receipts on disk|
| 🔧 started| 0| Implementation exists, not yet sealed|
| ⬜ unclaimed| 0| Not yet started|
| ⛔ permanent| 7| Will never be claimed|

## Phase 0: Oracle Admission

✅ AM.ORACLE.1 — GNU Automake 1.18.1 + aclocal + 4 subordinate oracles (autoconf, autom4te, m4, make) fingerprinted with SHA256. Oracle profile at reports/oracle-profile.json.

## Phase 1-2: CLI + aclocal + Autoconf Bridge

✅ AM.CLI.1 — 17 automake flags + 15 aclocal flags. --version byte-exact. 6 env vars (AUTOMAKE, ACLOCAL, AUTOCONF, AUTOM4TE, M4, MAKE). Native --print-libdir.
✅ AM.CLI.ACLOCAL.1 — Native aclocal engine: scan, generate, --install with serial tracking, --dry-run, --diff. 10 tests.
✅ AM.M4.AUTOCONF_BRIDGE.1 — autom4te trace: 6 macro types. AC_PROG_CC/CXX/FC/F77/OBJC/OBJCXX detection. Native extraction functional. Substitution value extraction. 8 tests.

## Phase 3-4: Parser + Generator + Macros + Primaries

✅ AM.PARSER.MAKEFILE_AM.1 — 12 primary types, 4 assignment ops (= += ?= :=), recursive conditional parsing, comments, continuations. 8 tests.
✅ AM.M4.AUTOMAKE.CORE.1 — AM_INIT_AUTOMAKE (11 options), AM_CONDITIONAL, AM_SILENT_RULES, AM_MAINTAINER_MODE. 43 std variables. 7 tests.
✅ AM.PRIMARY.PROGRAMS.1 — Compile/link/all-am/install-exec. Subdir-objects path mapping. Libtool linking. LTLIBRARIES primary. Per-target _CFLAGS/_LDADD/_LDFLAGS/_CPPFLAGS.
✅ AM.MAKEFILE_IN.1 — Full pipeline: VPATH, am__is_gnu_make, am__cd/am__tar/am__untar, LTLIBRARIES rules, subdir-objects, libtool, distcheck, GNU make detection. 8 tests.

## Phase 5-7: Rules + i18n + Diagnostics + Survival

✅ AM.RULES.INSTALL.1 — NORMAL_INSTALL/UNINSTALL, PRE/POST hooks, install-strip, install-info/dvi/ps/pdf/html, PHONY. 1 test.
✅ AM.RULES.DIST.1 — EXTRA_DIST, DISTFILES, distdir, dist/dist-all, distcheck, dist-gzip, distcleancheck. 3 tests.
✅ AM.I18N.1 — Pure Rust JSON catalogs (en/de/fr, 24 message keys each). LC_MESSAGES/LANG/LC_ALL honored. PERMANENT non-claim on gettext .po byte-parity. 3 tests.
✅ AM.DIAG.1 — 11 warning categories. DiagnosticManager with strictness-gated emission. Location tracking. Warnings-are-errors support. 10 tests.
✅ AM.SURVIVAL.TIER1.1 — 18/18 Tier 1 packages ALL pass exit 0.

## Cross-Cutting Deliverables

✅ 1M fuzz: 0 panics, 38K it/s (library-mode). ✅ 19/19 smoke tests. ✅ 10/10 GNU compare (6 exact line-count parity). ✅ Bench: 0.77x faster than GNU Automake (debug). ✅ 30 documents DSSE-signed. ✅ 40 files, 0 GPL contamination. ✅ 2 Kani proofs (variable escaping).

## Permanent Non-Claims (7)

⛔ POSIX signal handlers (NC.PERM.10) — safe Rust boundary. ⛔ gettext .po byte-parity (NC.PERM.9) — i18n EXISTS via JSON. ⛔ Security sandbox (NC.PERM.6). ⛔ Unicode (NC.PERM.7). ⛔ Performance (NC.PERM.5) — 0.77x debug, release later. ⛔ config.guess/config.sub (NC.PERM.11) — separate projects. ⛔ Cross-compilation (NC.PERM.4) — --host/--build parsed.

