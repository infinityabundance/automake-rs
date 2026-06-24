//! `automake-rs` — forensic-parity GNU Automake reimplementation.
//! The workspace crate: it ships the `automake-rs` CLI binaries and
//! re-exports the member crates so `cargo install automake-rs` gives the
//! tools and `cargo add automake-rs` gives the engine.
pub use automake_casefile_rs;
pub use automake_oracle_rs;
pub use automake_rs_cli;
pub use automake_rs_core;
