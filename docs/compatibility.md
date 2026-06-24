# Oracle Compatibility Matrix

Documents which GNU Automake versions have been tested as oracles, and what behavioral differences exist between them. Updated: all 13 courts sealed, 10/10 GNU compare tests pass, 6 exact line-count parity.

## Admitted Oracles

| Version| Status| Detail|
|---|---|---|
| GNU Automake 1.18.1| ✅ Primary oracle| SHA256 fingerprinted. Full receipt in reports/oracle-profile.json|
| GNU aclocal 1.18.1| ✅ Admitted| SHA256 fingerprinted|
| GNU autoconf (system)| ✅ Subordinate| Used via autom4te --trace for macro extraction|
| GNU autom4te (system)| ✅ Subordinate| Trace extraction bridge. Native path functional.|
| GNU m4 (system)| ✅ Subordinate| M4 expansion engine (via m4-rs-core dependency)|
| GNU make (system)| ✅ Subordinate| Build rule verification|

## Version Diff Analysis (across 1.11.6 → 1.18.1)

| Version Range| Behavioral Change| Our Behavior|
|---|---|---|
| 1.11.6 → 1.16.5| silent-rules default: no → yes| Match 1.16+ (unconditional AM_V_*)|
| 1.11.6 → 1.16.5| subdir-objects made default| Support via option flag, default flat|
| 1.16.5 → 1.18.1| Future make incompatibility warnings| Not yet implemented|
| 1.16.5 → 1.18.1| Stricter variable name validation| Not yet implemented|

## GNU Compare Results (10 tests)

| Test| Oracle| automake-rs| Verdict|
|---|---|---|---|
| empty Makefile.am| exit 0| exit 0| ✅ PASS (exact)|
| simple bin_PROGRAMS| exit 0| exit 0| ✅ PASS (exact)|
| PROGRAMS + SCRIPTS + DATA| exit 0| exit 0| ✅ PASS (exact)|
| AM_INIT_AUTOMAKE([foreign])| exit 0| exit 0| ✅ PASS|
| AM_CONDITIONAL + if/else| exit 0| exit 0| ✅ PASS|
| SUBDIRS recursion| exit 0| exit 0| ✅ PASS (exact)|
| EXTRA_DIST + dist| exit 0| exit 0| ✅ PASS (exact)|
| TESTS + check| exit 0| exit 0| ✅ PASS|
| LIBRARIES + LTLIBRARIES| exit 0| exit 0| ✅ PASS (exact)|
| subdir-objects compile| exit 0| exit 0| ✅ PASS|

## Subordinate Oracle Requirements

automake-rs requires the following on the test system: automake, aclocal, autoconf, autom4te, m4, make, and a POSIX shell (/bin/sh). These are used only as black-box oracles for comparison — no source code is consulted. The xtask gnu-compare command runs the comparison suite.

