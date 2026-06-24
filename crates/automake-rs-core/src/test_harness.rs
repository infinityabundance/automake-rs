// automake-rs-core: Test harness (TESTS, parallel-tests, LOG_COMPILER)
//
// Court: AM.RULES.CHECK.1 — not yet sealed.

#[derive(Debug, Clone)]
pub struct TestHarnessConfig {
    pub tests: Vec<String>,
    pub xfail_tests: Vec<String>,
    pub log_compiler: Option<String>,
    pub parallel_tests: bool,
    pub serial_tests: bool,
}

impl TestHarnessConfig {
    pub fn new() -> Self {
        Self {
            tests: vec![],
            xfail_tests: vec![],
            log_compiler: None,
            parallel_tests: false,
            serial_tests: false,
        }
    }
}

impl Default for TestHarnessConfig {
    fn default() -> Self {
        Self::new()
    }
}
