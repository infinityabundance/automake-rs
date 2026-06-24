# automake-rs

**A native Rust forensic-parity implementation of GNU Automake behavior, built through oracle courts.**

`automake-rs` is a clean-room behavioral reconstruction of GNU Automake. Each supported surface is admitted only after byte comparison against a pinned GNU Automake oracle. Unsupported surfaces are explicit non-claims.

## Status

| Metric | Value |
|--------|-------|
| Phase | 3 — Makefile.in Generation — Native Pipeline Working |
| Overall completion | **100.0%** |
| Oracle | GNU Automake 1.18.1 (admitted) |
| Courts sealed | 15 |
| Tests passing | 175 |
| Strategy | Clean-room behavioral reconstruction, forensic parity methodology |
| License | MIT OR Apache-2.0 — Zero GPL code |

## Surface Status

- ✅ **AM.ORACLE.1**: GNU Automake 1.18.1 + aclocal + 4 subordinate oracles admitted.
- ✅ **AM.CLI.1**: 17+15 flags. --version byte-exact. Env vars: AUTOMAKE, ACLOCAL, AUTOCONF, AUTOM4TE, M4, MAKE.
- ✅ **AM.CLI.ACLOCAL.1**: SEALED. 10 tests. Full engine: scan, generate, --install with serial tracking.
- ✅ **AM.M4.AUTOCONF_BRIDGE.1**: autom4te trace: 6 macro types + AC_PROG_* language detection. Native extraction active. Substitution values extracted.
- ✅ **AM.PARSER.MAKEFILE_AM.1**: 12 primary types, 4 assignment ops, conditionals. 8 tests.
- ✅ **AM.M4.AUTOMAKE.CORE.1**: AM_INIT_AUTOMAKE, AM_CONDITIONAL, AM_SILENT_RULES. 43 std vars. 7 tests.
- ✅ **AM.PRIMARY.PROGRAMS.1**: Compile/link/all-am/install-exec. Subdir-objects. Libtool linking. LTLIBRARIES primary.
- ✅ **AM.MAKEFILE_IN.1**: SEALED. 8 tests. VPATH, am__is_gnu_make, LTLIBRARIES, subdir-objects, libtool, distcheck.
- ✅ **AM.RULES.INSTALL.1**: NORMAL_INSTALL/UNINSTALL, PRE/POST hooks, install-strip, PHONY.
- ✅ **AM.I18N.1**: SEALED. 3 tests. Pure Rust JSON catalogs (en/de/fr). PERMANENT non-claim on gettext .po.
- ✅ **AM.RULES.DIST.1**: SEALED. 3 tests. EXTRA_DIST, DISTFILES, distdir, dist/dist-all, distcheck, dist-gzip, distcleancheck.
- ✅ **AM.DIAG.1**: SEALED. 10 tests. 11 warning categories. DiagnosticManager wired to CLI.
- ✅ **AM.SURVIVAL.TIER1.1**: SEALED. 18/18 packages ALL pass exit 0. Zero exceptions. Zero deferrals.
- ✅ **AM.COND.NAMESPACE.1**: SEALED. DisjConditions + Condition type + conditional stack parser + @COND_TRUE@/@COND_FALSE@ prefix generation. Variables tracked across conditional boundaries. 4 new integration tests.
- ✅ **AM.COND.ENV.1**: SEALED. ConditionalEnv tracks variables per-conditional-context. Handles += across boundaries, @COND_TRUE@/@COND_FALSE@ overrides, base+conditional value computation. Panel's #1 recommendation.

## Quick Start

```sh
cargo build
cargo xtask oracle
cargo xtask check
cargo xtask status
```

## License

MIT OR Apache-2.0. Zero GPL code. Clean-room behavioral reconstruction.
