// automake-rs-core: Recursive make (SUBDIRS)
//
// Handles SUBDIRS recursion and ordering for recursive Automake builds.

#[derive(Debug, Clone)]
pub struct RecursiveConfig {
    pub subdirs: Vec<String>,
    pub dist_subdirs: Vec<String>,
    pub conditionals: Vec<String>,
}

impl RecursiveConfig {
    pub fn new() -> Self {
        Self {
            subdirs: vec![],
            dist_subdirs: vec![],
            conditionals: vec![],
        }
    }
}

impl Default for RecursiveConfig {
    fn default() -> Self {
        Self::new()
    }
}
