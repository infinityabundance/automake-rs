// automake-rs-core: configure.ac parser/analyzer
//
// Parses configure.ac to extract the metadata Automake needs:
// AC_INIT info, AC_CONFIG_FILES, AC_CONFIG_HEADERS, AC_SUBST,
// AM_INIT_AUTOMAKE options, AM_CONDITIONAL definitions, etc.
//
// Court: AM.PARSER.MAKEFILE_AM.1 (configure.ac aspect)

/// configure.ac analysis result.
#[derive(Debug, Clone)]
pub struct ConfigureAc {
    pub package_name: Option<String>,
    pub package_version: Option<String>,
    pub bug_report: Option<String>,
    pub strictness: Option<String>, // foreign, gnu, gnits
    pub config_files: Vec<String>,
    pub config_headers: Vec<String>,
}

impl ConfigureAc {
    pub fn new() -> Self {
        Self {
            package_name: None,
            package_version: None,
            bug_report: None,
            strictness: None,
            config_files: vec![],
            config_headers: vec![],
        }
    }
}

impl Default for ConfigureAc {
    fn default() -> Self {
        Self::new()
    }
}
