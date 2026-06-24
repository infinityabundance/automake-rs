// automake-rs-core: Automake engine (oracle-bridge stub)
//
// Reconstructs the GNU automake behavior: reads Makefile.am +
// configure.ac trace facts and generates Makefile.in.
//
// This module depends on m4-rs-core and autoconf-rs-core for
// M4 expansion and Autoconf trace extraction.
//
// Court: AM.CLI.AUTOMAKE.1 — not yet sealed.

/// The Automake engine.
pub struct Automake {
    /// Path to the oracle profile for subordinate validation
    pub oracle_profile_path: Option<String>,
}

impl Automake {
    /// Create a new Automake engine.
    pub fn new() -> Self {
        Self {
            oracle_profile_path: None,
        }
    }

    /// Process a Makefile.am and generate Makefile.in.
    ///
    /// This is a stub — not yet implemented.
    /// Will be admitted court-by-court.
    pub fn process(&self, _makefile_am_path: &str) -> Result<String, AutomakeError> {
        Err(AutomakeError::NotYetImplemented(
            "AM.CLI.AUTOMAKE.1".to_string(),
        ))
    }
}

impl Default for Automake {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum AutomakeError {
    NotYetImplemented(String),
    Io(std::io::Error),
    ParseError(String),
    GenerationError(String),
}

impl std::fmt::Display for AutomakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutomakeError::NotYetImplemented(court) => {
                write!(f, "feature not yet implemented (court: {})", court)
            }
            AutomakeError::Io(e) => write!(f, "I/O error: {}", e),
            AutomakeError::ParseError(s) => write!(f, "parse error: {}", s),
            AutomakeError::GenerationError(s) => write!(f, "generation error: {}", s),
        }
    }
}

impl std::error::Error for AutomakeError {}

impl From<std::io::Error> for AutomakeError {
    fn from(e: std::io::Error) -> Self {
        AutomakeError::Io(e)
    }
}
