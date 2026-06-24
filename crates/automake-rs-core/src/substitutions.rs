// automake-rs-core: @VAR@ substitution bridge
//
// Handles configure-time @VAR@ substitutions in generated Makefile.in.
// These correspond to AC_SUBST variables from configure.ac.

use std::collections::HashMap;

/// The substitution table.
#[derive(Debug, Clone)]
pub struct SubstitutionTable {
    pub substitutions: HashMap<String, String>,
}

impl SubstitutionTable {
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::new(),
        }
    }

    pub fn add(&mut self, var: &str, value: &str) {
        self.substitutions
            .insert(var.to_string(), value.to_string());
    }

    pub fn resolve(&self, var: &str) -> Option<&str> {
        self.substitutions.get(var).map(String::as_str)
    }

    /// Substitute @VAR@ patterns in the given text.
    pub fn substitute(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (var, value) in &self.substitutions {
            result = result.replace(&format!("@{}@", var), value);
        }
        result
    }
}

impl Default for SubstitutionTable {
    fn default() -> Self {
        Self::new()
    }
}
