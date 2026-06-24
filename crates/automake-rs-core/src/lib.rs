// automake-rs-core: Core Automake semantic engine.
//
// This crate provides the Makefile.am parser, Makefile.in generator,
// aclocal macro discovery, primary families, rule synthesis, dependency
// tracking, and all core Automake behavior.
//
// Clean-room design: all behavior is reconstructed from black-box
// oracle interrogation, the GNU Automake manual (GFDL licensed), and
// POSIX specifications. No GNU Automake GPL source code is consulted.
//
// Current Phase: Phase 3 — Makefile.in Generation — Native Pipeline Working.
// Full native pipeline: parser → config → generator → Makefile.in output.
// 12 courts sealed, 59 tests, 5 real packages processed.

pub mod aclocal;
pub mod autoconf_bridge;
pub mod automake;
pub mod automake_macros;
pub mod aux_scripts;
pub mod cli;
pub mod conditional_env;
pub mod conditionals;
pub mod configure_ac;
pub mod dependency_tracking;
pub mod diagnostics;
pub mod dist;
pub mod event_parser;
pub mod i18n;
pub mod install;
pub mod m4_engine;
pub mod makefile_am;
pub mod makefile_in;
pub mod primaries;
pub mod profile;
pub mod recursive_make;
pub mod rowan_parser;
pub mod rules;
pub mod substitutions;
pub mod test_harness;
pub mod variables;

pub use aclocal::Aclocal;
/// Re-export key types for convenience.
pub use automake::Automake;
pub use makefile_am::MakefileAm;
pub use makefile_in::MakefileInGenerator;
