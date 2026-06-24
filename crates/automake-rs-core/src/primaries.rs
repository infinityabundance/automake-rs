// automake-rs-core: Primary families (PROGRAMS, LIBRARIES, etc.)
//
// Automake primaries are the special variable categories that trigger
// rule generation: PROGRAMS, LIBRARIES, LTLIBRARIES, SCRIPTS, DATA,
// HEADERS, MANS, TEXINFOS, TESTS, LISP, PYTHON, JAVA.
//
// Court: AM.PRIMARY.PROGRAMS.1, etc. — not yet sealed.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimaryKind {
    Programs,
    Libraries,
    LtLibraries,
    Scripts,
    Data,
    Headers,
    Mans,
    Texinfos,
    Tests,
    Lisp,
    Python,
    Java,
}

impl PrimaryKind {
    /// Map a primary name (e.g., "PROGRAMS") to its kind.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "PROGRAMS" => Some(Self::Programs),
            "LIBRARIES" => Some(Self::Libraries),
            "LTLIBRARIES" => Some(Self::LtLibraries),
            "SCRIPTS" => Some(Self::Scripts),
            "DATA" => Some(Self::Data),
            "HEADERS" => Some(Self::Headers),
            "MANS" => Some(Self::Mans),
            "TEXINFOS" => Some(Self::Texinfos),
            "TESTS" => Some(Self::Tests),
            "LISP" => Some(Self::Lisp),
            "PYTHON" => Some(Self::Python),
            "JAVA" => Some(Self::Java),
            _ => None,
        }
    }

    /// The directory category for install destination.
    pub fn install_dir_category(&self) -> &'static str {
        match self {
            Self::Programs | Self::Libraries | Self::LtLibraries => "exec",
            Self::Scripts => "exec",
            Self::Data => "data",
            Self::Headers => "data",
            Self::Mans => "data",
            Self::Texinfos => "data",
            Self::Tests => "data",
            Self::Lisp => "data",
            Self::Python => "data",
            Self::Java => "data",
        }
    }
}

/// Information about an Automake primary variable.
#[derive(Debug, Clone)]
pub struct PrimaryVariable {
    pub kind: PrimaryKind,
    pub base_name: String,    // e.g., "bin_PROGRAMS"
    pub dir_category: String, // e.g., "bin", "noinst", "check"
    pub dist_kind: DistKind,  // dist or nodist
    pub targets: Vec<String>, // target names
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistKind {
    Dist,    // included in distribution
    NoDist,  // excluded from distribution
    Default, // neither prefix used
}
