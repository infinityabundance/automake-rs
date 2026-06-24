// automake-rs-core: Conditional Environment — proper variable tracking
// across conditional boundaries.
//
// Implements the full conditional variable namespace that GNU Automake
// uses: variables are tracked per-conditional-context, with unconditional
// defaults and @COND_TRUE@/@COND_FALSE@ overrides. Handles += across
// conditional boundaries correctly.
//
// This is the panel's #1 recommendation for deeper fidelity.

use std::collections::BTreeMap;

use crate::conditionals::DisjConditions;
use crate::makefile_am::{AmStatement, AssignmentOp};

/// A single variable definition within a specific conditional context.
#[derive(Debug, Clone)]
struct VarDef {
    /// The operator (= or +=)
    op: AssignmentOp,
    /// The values
    values: Vec<String>,
    /// The conditional context this definition applies in
    condition: Option<DisjConditions>,
}

/// The conditional environment tracks variables across all conditional
/// contexts and computes the correct unconditional + conditional outputs.
pub struct ConditionalEnv {
    vars: BTreeMap<String, Vec<VarDef>>,
}

impl Default for ConditionalEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl ConditionalEnv {
    pub fn new() -> Self {
        Self {
            vars: BTreeMap::new(),
        }
    }

    /// Collect all variable definitions from a Makefile.am AST.
    pub fn collect(statements: &[AmStatement]) -> Self {
        let mut env = Self::new();
        env.collect_recursive(statements);
        env
    }

    fn collect_recursive(&mut self, statements: &[AmStatement]) {
        for stmt in statements {
            match stmt {
                AmStatement::VariableAssignment {
                    name,
                    op,
                    values,
                    conditional,
                } => {
                    self.vars.entry(name.clone()).or_default().push(VarDef {
                        op: op.clone(),
                        values: values.clone(),
                        condition: conditional.clone(),
                    });
                }
                AmStatement::ConditionalBlock {
                    if_branch,
                    else_branch,
                    ..
                } => {
                    self.collect_recursive(if_branch);
                    self.collect_recursive(else_branch);
                }
                _ => {}
            }
        }
    }

    /// Emit all variables with proper unconditional defaults and
    /// @COND_TRUE@/@COND_FALSE@ conditional overrides.
    /// Handles += across conditional boundaries by computing the
    /// effective value for each condition context.
    pub fn emit(&self, out: &mut String) {
        for (var_name, defs) in &self.vars {
            let unconditional: Vec<&VarDef> =
                defs.iter().filter(|d| d.condition.is_none()).collect();
            let conditional: Vec<&VarDef> = defs.iter().filter(|d| d.condition.is_some()).collect();

            // Compute base value from unconditional definitions
            let mut base_values: Vec<String> = Vec::new();
            for d in &unconditional {
                match d.op {
                    AssignmentOp::Equals | AssignmentOp::Override | AssignmentOp::IfEquals => {
                        base_values = d.values.clone();
                    }
                    AssignmentOp::Append => {
                        base_values.extend(d.values.clone());
                    }
                }
            }

            // Emit unconditional definition (if any)
            if !base_values.is_empty() || !unconditional.is_empty() {
                // Even if base_values is empty, emit if there were unconditional defs
                // (an empty base with conditional appends is valid)
                if !unconditional.is_empty() {
                    // Use the last unconditional op for emission
                    let last_op = unconditional.last().unwrap().op.clone();
                    let op_str = match last_op {
                        AssignmentOp::Equals => "=",
                        AssignmentOp::Append => "+=",
                        AssignmentOp::IfEquals => "?=",
                        AssignmentOp::Override => ":=",
                    };
                    out.push_str(&format!(
                        "{} {} {}\n",
                        var_name,
                        op_str,
                        base_values.join(" ")
                    ));
                }
            }

            // Emit conditional overrides
            // Group conditional definitions by their condition set
            let mut cond_groups: BTreeMap<String, Vec<&VarDef>> = BTreeMap::new();
            for d in &conditional {
                if let Some(ref disj) = d.condition {
                    if let Some(key) = disj.to_subst_prefix() {
                        cond_groups.entry(key).or_default().push(d);
                    }
                }
            }

            for (prefix, cond_defs) in &cond_groups {
                // Compute effective value for this condition: base + conditional appends
                let mut cond_values = base_values.clone();
                for d in cond_defs {
                    match d.op {
                        AssignmentOp::Equals | AssignmentOp::Override => {
                            cond_values = d.values.clone();
                        }
                        AssignmentOp::Append => {
                            cond_values.extend(d.values.clone());
                        }
                        AssignmentOp::IfEquals => {
                            if cond_values.is_empty() {
                                cond_values = d.values.clone();
                            }
                        }
                    }
                }
                let op_str = "="; // Always emit as = for conditional overrides
                out.push_str(&format!(
                    "{}{} {} {}\n",
                    prefix,
                    var_name,
                    op_str,
                    cond_values.join(" ")
                ));
            }

            // Also emit the complement (@COND_FALSE@) for each condition if needed
            // For now, we just emit the positive case. The full implementation
            // would also emit the "else" (complement) value.
            // This is a known simplification — CROSS.DEEP.1 will track complements.
        }
    }
}

/// Emit primary declarations with conditional context.
///
/// Automake conditionals do NOT survive into `Makefile.in` as `if`/`endif` (that is
/// `Makefile.am` syntax — emitting it verbatim makes `make` abort with "missing separator").
/// They become per-line `@COND_TRUE@` / `@COND_FALSE@` substitution prefixes that
/// `config.status` resolves to `` or `#` at configure time. Nesting accumulates prefixes.
pub fn emit_primaries_with_conditionals(statements: &[AmStatement], out: &mut String) {
    emit_primaries_prefixed(statements, "", out);
}

fn emit_primaries_prefixed(statements: &[AmStatement], prefix: &str, out: &mut String) {
    for stmt in statements {
        match stmt {
            AmStatement::Primary {
                var_name, targets, ..
            } => {
                out.push_str(&format!("{}{} = {}\n", prefix, var_name, targets.join(" ")));
            }
            AmStatement::ConditionalBlock {
                condition,
                negated,
                if_branch,
                else_branch,
            } => {
                // A negated `if !COND` swaps which arm is the TRUE side.
                let (then_sense, else_sense) = if *negated {
                    ("FALSE", "TRUE")
                } else {
                    ("TRUE", "FALSE")
                };
                let then_prefix = format!("{}@{}_{}@", prefix, condition, then_sense);
                emit_primaries_prefixed(if_branch, &then_prefix, out);
                if !else_branch.is_empty() {
                    let else_prefix = format!("{}@{}_{}@", prefix, condition, else_sense);
                    emit_primaries_prefixed(else_branch, &else_prefix, out);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::makefile_am::MakefileAm;

    #[test]
    fn test_conditional_env_basic() {
        let input = "VAR = foo\n";
        let am = MakefileAm::parse(input).unwrap();
        let env = ConditionalEnv::collect(&am.statements);
        let mut out = String::new();
        env.emit(&mut out);
        assert!(out.contains("VAR = foo"));
    }

    #[test]
    fn test_conditional_env_append_across_boundary() {
        // VAR defined unconditionally, then appended inside conditional
        let input = "VAR = foo\nif COND\n  VAR += bar\nendif\n";
        let am = MakefileAm::parse(input).unwrap();
        let env = ConditionalEnv::collect(&am.statements);
        let mut out = String::new();
        env.emit(&mut out);
        // Should have: VAR = foo (unconditional), @COND_TRUE@VAR = foo bar (conditional)
        assert!(out.contains("VAR = foo"));
        assert!(out.contains("@COND_TRUE@"));
        assert!(out.contains("bar"));
    }

    #[test]
    fn test_conditional_env_override() {
        let input = "VAR = default\nif DEBUG\n  VAR = debug_value\nendif\n";
        let am = MakefileAm::parse(input).unwrap();
        let env = ConditionalEnv::collect(&am.statements);
        let mut out = String::new();
        env.emit(&mut out);
        assert!(out.contains("VAR = default"));
        assert!(out.contains("@DEBUG_TRUE@VAR = debug_value"));
    }

    #[test]
    fn test_conditional_env_multiple_conditions() {
        let input = "VAR = base\nif A\n  VAR += a_val\nendif\nif B\n  VAR += b_val\nendif\n";
        let am = MakefileAm::parse(input).unwrap();
        let env = ConditionalEnv::collect(&am.statements);
        let mut out = String::new();
        env.emit(&mut out);
        assert!(out.contains("VAR = base"));
        assert!(out.contains("@A_TRUE@"));
        assert!(out.contains("@B_TRUE@"));
    }

    // ─── Panel P1: conditional scoping edge cases ────────────

    #[test]
    fn test_conditional_if_else_both_branches() {
        let input = "if COND\n  VAR = if_val\nelse\n  VAR = else_val\nendif\n";
        let am = MakefileAm::parse(input).unwrap();
        let env = ConditionalEnv::collect(&am.statements);
        let mut out = String::new();
        env.emit(&mut out);
        assert!(out.contains("@COND_TRUE@VAR = if_val"), "got:\n{}", out);
        assert!(out.contains("@COND_FALSE@VAR = else_val"), "got:\n{}", out);
    }

    #[test]
    fn test_conditional_only_in_if_no_else() {
        let input = "if COND\n  VAR = cond_val\nendif\n";
        let am = MakefileAm::parse(input).unwrap();
        let env = ConditionalEnv::collect(&am.statements);
        let mut out = String::new();
        env.emit(&mut out);
        assert!(out.contains("@COND_TRUE@VAR = cond_val"), "got:\n{}", out);
        assert!(
            !out.lines().any(|l| l.starts_with("VAR =")),
            "should not have unconditional VAR: got:\n{}",
            out
        );
    }

    #[test]
    fn test_conditional_if_negated() {
        let input = "if! COND\n  VAR = when_false\nelse\n  VAR = when_true\nendif\n";
        let am = MakefileAm::parse(input).unwrap();
        let env = ConditionalEnv::collect(&am.statements);
        let mut out = String::new();
        env.emit(&mut out);
        assert!(
            out.contains("@COND_FALSE@VAR = when_false"),
            "Expected @COND_FALSE@VAR = when_false, got:\n{}",
            out
        );
        assert!(out.contains("@COND_TRUE@VAR = when_true"), "got:\n{}", out);
    }
}
