// automake-rs-core: M4 engine bridge
//
// Wraps m4-rs-core as the M4 expansion engine for Automake-specific
// M4 processing. Automake uses M4 via autoconf traces to extract
// configuration metadata from configure.ac, then generates
// Makefile.in with @VAR@ substitutions.
//
// Dependencies: m4-rs-core, autoconf-rs-core

/// Bridge to the m4-rs-core expansion engine.
///
/// This module provides the Automake-specific M4 processing layer:
/// loading Automake's own .m4 macros, running autoconf traces,
/// and handling the expansion pipeline used to build Makefile.in.
pub struct M4Engine {
    /// Paths to Automake's built-in macro files (reconstructed in Rust)
    pub macro_paths: Vec<String>,
    /// Whether the engine has been initialized with Automake macros
    pub initialized: bool,
}

impl M4Engine {
    /// Create a new M4 engine bridge.
    pub fn new() -> Self {
        Self {
            macro_paths: vec![],
            initialized: false,
        }
    }

    /// Initialize the engine with Automake's built-in macros.
    ///
    /// These macros are reconstructed in Rust as built-in behavior,
    /// not copied from GNU Automake .m4 files.
    pub fn initialize(&mut self) -> Result<(), M4EngineError> {
        if self.initialized {
            return Ok(());
        }

        // Register Automake-specific M4 macros as built-in behavior.
        // Each macro is admitted separately via its own court.
        // AM.CLI.AUTOMAKE.1 not yet sealed — stub only.
        self.initialized = true;
        Ok(())
    }

    /// Expand a string with Automake macros loaded.
    pub fn expand(&self, _input: &[u8]) -> Result<Vec<u8>, M4EngineError> {
        if !self.initialized {
            return Err(M4EngineError::NotInitialized);
        }
        Err(M4EngineError::NotYetImplemented(
            "AM.M4.AUTOMAKE.CORE.1".to_string(),
        ))
    }
}

impl Default for M4Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum M4EngineError {
    NotInitialized,
    NotYetImplemented(String),
    ExpansionError(String),
    Io(std::io::Error),
}

impl std::fmt::Display for M4EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            M4EngineError::NotInitialized => write!(f, "M4 engine not initialized"),
            M4EngineError::NotYetImplemented(c) => write!(f, "not yet implemented ({})", c),
            M4EngineError::ExpansionError(s) => write!(f, "expansion error: {}", s),
            M4EngineError::Io(e) => write!(f, "I/O: {}", e),
        }
    }
}

impl std::error::Error for M4EngineError {}
