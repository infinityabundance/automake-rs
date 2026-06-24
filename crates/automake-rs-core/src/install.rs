// automake-rs-core: install/uninstall rule generation
//
// Court: AM.RULES.INSTALL.1 — not yet sealed.

/// Install targets and behavior.
#[derive(Debug, Clone)]
pub struct InstallConfig {
    pub install_exec_targets: Vec<String>,
    pub install_data_targets: Vec<String>,
    pub exec_prefix: Option<String>,
    pub prefix: Option<String>,
}

impl InstallConfig {
    pub fn new() -> Self {
        Self {
            install_exec_targets: vec![],
            install_data_targets: vec![],
            exec_prefix: None,
            prefix: None,
        }
    }
}

impl Default for InstallConfig {
    fn default() -> Self {
        Self::new()
    }
}
