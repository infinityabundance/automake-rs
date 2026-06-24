# FORENSIC GAP ANALYSIS — GNU Automake → automake-rs

**Oracle:** GNU Automake 1.18.1  
**Strategy:** Clean-room behavioral reconstruction  
**Licensing:** MIT OR Apache-2.0 — Zero GPL entanglement  
**Generated:** 1782325795

## Summary

The gap analysis catalogues every surface of GNU Automake and maps it to the corresponding automake-rs module. Each entry tracks implementation status (implemented/partial/missing).

## Source Files

GNU Automake is written in Perl (~30,000 lines) with M4 macros (~5,000 lines), shell scripts, and make fragments. automake-rs replaces each component with clean-room Rust.

| Component | GNU | automake-rs | Status |
|---|---|---|---|
| CLI (automake) | automake.in (Perl) | cli.rs + automake_rs_cli | ✅ Sealed |
| CLI (aclocal) | aclocal.in (Perl) | aclocal.rs | ✅ Sealed |
| Makefile.am parser | Automake::Parser (Perl) | makefile_am.rs | ✅ Sealed |
| Macro engine | Automake::Configure (Perl) + .m4 files | automake_macros.rs | ✅ Sealed |
| Autoconf bridge | autoconf/autom4te traces | autoconf_bridge.rs | ✅ Sealed |
| Makefile.in gen | Automake::Generate (Perl) | makefile_in.rs | ✅ Sealed |
| Primaries | Automake::Variable (Perl) | primaries.rs | 🔧 Scaffolded |
| Dependency tracking | depcomp + depend2.am | dependency_tracking.rs | 🔧 Scaffolded |
| Install rules | install.am fragments | rules (install section) | 🔧 Scaffolded |
| Dist rules | dist.am fragments | rules (dist section) | 🔧 Scaffolded |
| Test harness | test-driver + check.am | rules (check section) | ✅ Sealed |
| Diagnostics | Automake::ChannelDefs (Perl) | diagnostics.rs | 🔧 Scaffolded |
| Oracle admission | N/A (new) | oracle-rs crate | ✅ Sealed |
| Receipt system | N/A (new) | casefile-rs crate | ✅ Sealed |

## Cross-Cutting Gaps

| ID | Gap | Impact | Status |
|---|---|---|---|
| CROSS.1 | Perl regex vs Rust regex | Different regex engines for Makefile.am parsing | ✅ Resolved |
| CROSS.2 | Perl M4 bridge vs autom4te oracle | Trace extraction delegates to oracle | ⚠ Monitored |
| CROSS.3 | VPATH generation | Our generator is simpler than GNU's  | 🔧 Not yet |
| CROSS.4 | GNU make detection (am__is_gnu_make) | Not yet implemented | 🔧 Not yet |
| CROSS.5 | Dependency tracking (depcomp) | Delegates to oracle via --add-missing | ⚠ Monitored |
| CROSS.6 | i18n (gettext translations) | Permanent non-claim | ⛔ Permanent |
