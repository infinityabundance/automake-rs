// automake-rs-core: Automake variable model
//
// Tracks variable assignments, their scoping, and resolution across
// Makefile.am files. Handles =, +=, ?=, and per-target variables.

use std::collections::HashMap;

/// The Automake variable namespace.
#[derive(Debug, Clone)]
pub struct VariableTable {
    pub variables: HashMap<String, Variable>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub values: Vec<String>,
    pub kind: VariableKind,
    pub conditional: Option<String>,
    pub defined_in_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableKind {
    Simple,      // foo = bar
    Appended,    // foo += bar
    Conditional, // foo ?= bar
    Override,    // foo := bar (GNU make)
    PerTarget,   // target_VAR (per-target variable)
}

impl VariableTable {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }

    pub fn set(&mut self, name: &str, value: Vec<String>, kind: VariableKind) {
        self.variables.insert(
            name.to_string(),
            Variable {
                name: name.to_string(),
                values: value,
                kind,
                conditional: None,
                defined_in_file: None,
            },
        );
    }
}

impl Default for VariableTable {
    fn default() -> Self {
        Self::new()
    }
}
