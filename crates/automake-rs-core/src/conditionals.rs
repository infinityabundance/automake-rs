// automake-rs-core: Conditional handling (AM_CONDITIONAL)
//
// Handles Automake conditionals defined via AM_CONDITIONAL in
// configure.ac. These control conditional sections in Makefile.am.
//
// Clean-room reconstruction based on:
//   - GNU Automake manual §20 (Conditionals), GFDL licensed
//   - Black-box oracle interrogation
// No GNU Automake GPL source code was consulted.

use std::collections::{HashMap, HashSet};

/// The conditional namespace.
/// Tracks AM_CONDITIONAL definitions and provides query methods.
#[derive(Debug, Clone)]
pub struct ConditionalTable {
    pub conditionals: HashMap<String, Conditional>,
}

/// A single AM_CONDITIONAL definition.
#[derive(Debug, Clone)]
pub struct Conditional {
    pub name: String,
    pub value: bool,
    pub condition: Option<String>, // nested condition
}

/// A set of conditions that must all be true (conjunction).
/// Represents what GNU Automake calls a "Condition" — a single
/// atomic condition like "COND1" or "!COND2".
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Condition {
    /// The condition name (without the ! prefix)
    pub name: String,
    /// Whether the condition is negated
    pub negated: bool,
}

impl Condition {
    pub fn new(name: &str, negated: bool) -> Self {
        Self {
            name: name.to_string(),
            negated,
        }
    }

    /// Format for Makefile.am conditional syntax: "COND" or "!COND"
    pub fn to_am_string(&self) -> String {
        if self.negated {
            format!("!{}", self.name)
        } else {
            self.name.clone()
        }
    }

    /// Format for @COND_TRUE@/@COND_FALSE@ configure substitution
    pub fn to_subst_prefix(&self) -> String {
        if self.negated {
            format!("@{}_FALSE@", self.name)
        } else {
            format!("@{}_TRUE@", self.name)
        }
    }
}

/// A disjunction of conditions — any of these conjunctions being true
/// makes the whole thing true. Used for tracking which conditional
/// contexts a variable definition applies in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisjConditions {
    /// Each inner Vec is a conjunction (AND), the outer Vec is a disjunction (OR)
    /// Empty means "always true" (unconditional)
    pub conditions: Vec<Vec<Condition>>,
}

impl DisjConditions {
    /// The always-true (unconditional) condition set
    pub fn always_true() -> Self {
        Self {
            conditions: vec![vec![]],
        }
    }

    /// Check if this represents the unconditional case
    pub fn is_always_true(&self) -> bool {
        self.conditions.is_empty() || self.conditions == vec![vec![]]
    }

    /// Check if this is impossible (always false)
    pub fn is_always_false(&self) -> bool {
        self.conditions.is_empty()
    }

    /// Merge two condition sets with AND semantics.
    /// The result is true when both are true.
    pub fn and(&self, other: &DisjConditions) -> DisjConditions {
        if self.is_always_false() || other.is_always_false() {
            return DisjConditions { conditions: vec![] };
        }
        if self.is_always_true() {
            return other.clone();
        }
        if other.is_always_true() {
            return self.clone();
        }

        let mut result = vec![];
        for a_conj in &self.conditions {
            for b_conj in &other.conditions {
                let mut merged = a_conj.clone();
                merged.extend(b_conj.clone());
                result.push(merged);
            }
        }
        DisjConditions { conditions: result }
    }

    /// Merge two condition sets with OR semantics.
    /// The result is true when either is true.
    pub fn or(&self, other: &DisjConditions) -> DisjConditions {
        if self.is_always_true() || other.is_always_true() {
            return DisjConditions::always_true();
        }
        if self.is_always_false() {
            return other.clone();
        }
        if other.is_always_false() {
            return self.clone();
        }

        let mut result = self.conditions.clone();
        result.extend(other.conditions.clone());
        DisjConditions { conditions: result }
    }

    /// Negate this condition set using De Morgan's laws.
    /// !(A∧B) = !A∨!B,  !(A∨B) = !A∧!B
    pub fn negate(&self) -> DisjConditions {
        if self.is_always_false() {
            return DisjConditions::always_true();
        }
        if self.is_always_true() {
            return DisjConditions { conditions: vec![] };
        }
        // De Morgan: NOT(A₁∨A₂∨...) ≡ NOT(A₁)∧NOT(A₂)∧...
        // Where each Aᵢ is a conjunction c₁∧c₂∧...
        // NOT(c₁∧c₂∧c₃) = !c₁∨!c₂∨!c₃
        //
        // So: NOT(∨ᵢ(∧ⱼ c_{i,j})) = ∧ᵢ(∨ⱼ !c_{i,j})
        // This is the cross-product of all per-conjunction disjunctions.
        //
        // For simple cases: single conjunction → negate each condition into OR
        if self.conditions.len() == 1 {
            // Single conjunction: NOT(c₁∧c₂∧c₃) = !c₁∨!c₂∨!c₃
            let inner = &self.conditions[0];
            let mut result: Vec<Vec<Condition>> = vec![];
            for cond in inner {
                let mut negated = cond.clone();
                negated.negated = !negated.negated;
                result.push(vec![negated]);
            }
            return DisjConditions { conditions: result };
        }
        // For general case: cross-product of per-conjunction negations
        // Each original conjunction becomes an OR of single negated conditions
        // Then we take the AND across all conjunctions = cross product
        let mut per_conj: Vec<Vec<Vec<Condition>>> = vec![];
        for conj in &self.conditions {
            let mut negated_or: Vec<Vec<Condition>> = vec![];
            for cond in conj {
                let mut negated = cond.clone();
                negated.negated = !negated.negated;
                negated_or.push(vec![negated]);
            }
            if negated_or.is_empty() {
                // Empty conjunction = always true, so negation = always false
                return DisjConditions { conditions: vec![] };
            }
            per_conj.push(negated_or);
        }
        // Cross-product: AND across the OR-lists
        let mut result: Vec<Vec<Condition>> = vec![vec![]];
        for or_list in &per_conj {
            let mut new_result: Vec<Vec<Condition>> = vec![];
            for existing in &result {
                for cond_vec in or_list {
                    let mut merged = existing.clone();
                    merged.extend(cond_vec.clone());
                    new_result.push(merged);
                }
            }
            result = new_result;
        }
        DisjConditions { conditions: result }
    }

    /// Normalize the condition set into canonical DNF form:
    /// - Sort conditions within each conjunction by name
    /// - Remove duplicate conjunctions
    /// - Detect contradictions (A∧!A → remove conjunction)
    /// - Remove always-false conjunctions
    pub fn normalize(&self) -> DisjConditions {
        if self.is_always_true() || self.is_always_false() {
            return self.clone();
        }

        let mut normalized: Vec<Vec<Condition>> = vec![];
        for conj in &self.conditions {
            // Sort conditions by name for canonical form
            let mut sorted = conj.clone();
            sorted.sort_by(|a, b| a.name.cmp(&b.name));

            // Detect contradiction: A∧!A in the same conjunction
            let mut has_contradiction = false;
            for i in 0..sorted.len() {
                for j in (i + 1)..sorted.len() {
                    if sorted[i].name == sorted[j].name && sorted[i].negated != sorted[j].negated {
                        has_contradiction = true;
                        break;
                    }
                }
                if has_contradiction {
                    break;
                }
            }
            if has_contradiction {
                continue; // This conjunction is always false, skip it
            }

            // Deduplicate identical conditions within the conjunction
            let mut deduped: Vec<Condition> = vec![];
            let mut seen: HashSet<String> = HashSet::new();
            for cond in &sorted {
                let key = format!("{}/{}", cond.name, cond.negated);
                if !seen.contains(&key) {
                    seen.insert(key);
                    deduped.push(cond.clone());
                }
            }
            normalized.push(deduped);
        }

        // Remove duplicate conjunctions
        normalized.sort_by(|a, b| {
            let a_str: Vec<String> = a
                .iter()
                .map(|c| format!("{}/{}", c.name, c.negated))
                .collect();
            let b_str: Vec<String> = b
                .iter()
                .map(|c| format!("{}/{}", c.name, c.negated))
                .collect();
            a_str.cmp(&b_str)
        });
        normalized.dedup_by(|a, b| {
            if a.len() != b.len() {
                return false;
            }
            a.iter()
                .zip(b.iter())
                .all(|(ca, cb)| ca.name == cb.name && ca.negated == cb.negated)
        });

        if normalized.is_empty() {
            DisjConditions { conditions: vec![] } // always false
        } else if normalized == vec![vec![]] {
            DisjConditions::always_true()
        } else {
            DisjConditions {
                conditions: normalized,
            }
        }
    }

    /// Simplify: remove subsumed conjunctions.
    /// If conjunction A subsumes B (A is a subset of B's conditions),
    /// then B is redundant because A being true already guarantees B.
    /// Example: A subsumes A∧B, so A∧B can be removed.
    pub fn simplify(&self) -> DisjConditions {
        let normalized = self.normalize();
        if normalized.is_always_true() || normalized.is_always_false() {
            return normalized;
        }

        let mut result: Vec<Vec<Condition>> = vec![];
        for (i, conj_i) in normalized.conditions.iter().enumerate() {
            let mut subsumed = false;
            for (j, conj_j) in normalized.conditions.iter().enumerate() {
                if i == j {
                    continue;
                }
                // Check if conj_j subsumes conj_i (j is a subset of i)
                if conj_j.len() <= conj_i.len() {
                    let all_in = conj_j.iter().all(|cj| {
                        conj_i
                            .iter()
                            .any(|ci| ci.name == cj.name && ci.negated == cj.negated)
                    });
                    if all_in {
                        subsumed = true;
                        break;
                    }
                }
            }
            if !subsumed {
                result.push(conj_i.clone());
            }
        }

        if result.is_empty() {
            DisjConditions { conditions: vec![] }
        } else {
            DisjConditions { conditions: result }
        }
    }

    /// Format for @COND_TRUE@/@COND_FALSE@ conditional prefix (single condition only).
    pub fn to_subst_prefix(&self) -> Option<String> {
        if self.conditions.len() == 1 && self.conditions[0].len() == 1 {
            Some(self.conditions[0][0].to_subst_prefix())
        } else {
            None
        }
    }

    /// Check if a given set of true conditions satisfies this condition set.
    pub fn is_satisfied_by(&self, true_conditions: &HashSet<String>) -> bool {
        if self.is_always_true() {
            return true;
        }
        if self.is_always_false() {
            return false;
        }
        // For each conjunction (AND), all conditions must be satisfied
        for conj in &self.conditions {
            let mut all_satisfied = true;
            for cond in conj {
                let is_true = true_conditions.contains(&cond.name);
                if cond.negated == is_true {
                    // condition says !X but X is true, or condition says X but X is false
                    all_satisfied = false;
                    break;
                }
            }
            if all_satisfied {
                return true; // This conjunction is satisfied → whole disjunction is true
            }
        }
        false
    }
}

impl ConditionalTable {
    pub fn new() -> Self {
        Self {
            conditionals: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: bool) {
        self.conditionals.insert(
            name.to_string(),
            Conditional {
                name: name.to_string(),
                value,
                condition: None,
            },
        );
    }

    pub fn is_true(&self, name: &str) -> Option<bool> {
        self.conditionals.get(name).map(|c| c.value)
    }

    /// Get the set of all defined conditional names that are true.
    pub fn true_condition_names(&self) -> HashSet<String> {
        self.conditionals
            .iter()
            .filter(|(_, c)| c.value)
            .map(|(n, _)| n.clone())
            .collect()
    }

    /// Build a DisjConditions from a conditional path (list of conditions in order).
    pub fn disj_from_path(path: &[Condition]) -> DisjConditions {
        if path.is_empty() {
            DisjConditions::always_true()
        } else {
            DisjConditions {
                conditions: vec![path.to_vec()],
            }
        }
    }
}

impl Default for ConditionalTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disj_always_true() {
        let d = DisjConditions::always_true();
        assert!(d.is_always_true());
        let empty: HashSet<String> = HashSet::new();
        assert!(d.is_satisfied_by(&empty));
    }

    #[test]
    fn test_disj_single_condition() {
        let d = DisjConditions {
            conditions: vec![vec![Condition::new("COND1", false)]],
        };
        let mut true_set = HashSet::new();
        true_set.insert("COND1".to_string());
        assert!(d.is_satisfied_by(&true_set));

        let empty: HashSet<String> = HashSet::new();
        assert!(!d.is_satisfied_by(&empty));
    }

    #[test]
    fn test_disj_negated_condition() {
        let d = DisjConditions {
            conditions: vec![vec![Condition::new("COND1", true)]],
        };
        // !COND1 is satisfied when COND1 is false (not in set)
        let empty: HashSet<String> = HashSet::new();
        assert!(d.is_satisfied_by(&empty));

        let mut true_set = HashSet::new();
        true_set.insert("COND1".to_string());
        assert!(!d.is_satisfied_by(&true_set));
    }

    #[test]
    fn test_disj_and_or() {
        let a = DisjConditions {
            conditions: vec![vec![Condition::new("A", false)]],
        };
        let b = DisjConditions {
            conditions: vec![vec![Condition::new("B", false)]],
        };

        // A AND B
        let and_ab = a.and(&b);
        let mut ab_true = HashSet::new();
        ab_true.insert("A".to_string());
        ab_true.insert("B".to_string());
        assert!(and_ab.is_satisfied_by(&ab_true));

        let mut a_only = HashSet::new();
        a_only.insert("A".to_string());
        assert!(!and_ab.is_satisfied_by(&a_only));

        // A OR B
        let or_ab = a.or(&b);
        assert!(or_ab.is_satisfied_by(&a_only));
    }

    #[test]
    fn test_conditional_table() {
        let mut table = ConditionalTable::new();
        table.define("MAINTAINER_MODE", true);
        table.define("DEBUG", false);

        assert_eq!(table.is_true("MAINTAINER_MODE"), Some(true));
        assert_eq!(table.is_true("DEBUG"), Some(false));
        assert_eq!(table.is_true("UNKNOWN"), None);

        let true_names = table.true_condition_names();
        assert!(true_names.contains("MAINTAINER_MODE"));
        assert!(!true_names.contains("DEBUG"));
    }

    #[test]
    fn test_condition_subst_prefix() {
        let cond = Condition::new("AMDEP", false);
        assert_eq!(cond.to_subst_prefix(), "@AMDEP_TRUE@");

        let neg = Condition::new("AMDEP", true);
        assert_eq!(neg.to_subst_prefix(), "@AMDEP_FALSE@");
    }

    // ===========================================================
    // DNF Normalization & Negation Tests
    // ===========================================================

    #[test]
    fn test_dnf_negate_single() {
        // !A = !A
        let a = DisjConditions {
            conditions: vec![vec![Condition::new("A", false)]],
        };
        let not_a = a.negate();
        assert_eq!(not_a.conditions.len(), 1);
        assert_eq!(not_a.conditions[0][0].name, "A");
        assert!(not_a.conditions[0][0].negated);
    }

    #[test]
    fn test_dnf_negate_conjunction() {
        // !(A∧B) = !A∨!B
        let ab = DisjConditions {
            conditions: vec![vec![Condition::new("A", false), Condition::new("B", false)]],
        };
        let not_ab = ab.negate();
        // Should be: {!A} ∨ {!B} = [[!A], [!B]]
        assert_eq!(not_ab.conditions.len(), 2);
        // Verify both !A and !B are represented
        let has_not_a = not_ab
            .conditions
            .iter()
            .any(|c| c.len() == 1 && c[0].name == "A" && c[0].negated);
        let has_not_b = not_ab
            .conditions
            .iter()
            .any(|c| c.len() == 1 && c[0].name == "B" && c[0].negated);
        assert!(has_not_a, "!A should be present");
        assert!(has_not_b, "!B should be present");
    }

    #[test]
    fn test_dnf_normalize_contradiction() {
        // A∧!A → always false
        let contradiction = DisjConditions {
            conditions: vec![vec![Condition::new("A", false), Condition::new("A", true)]],
        };
        let normalized = contradiction.normalize();
        assert!(normalized.is_always_false(), "A∧!A should be always false");
    }

    #[test]
    fn test_dnf_normalize_duplicate() {
        // A∧A → A (dedup)
        let dup = DisjConditions {
            conditions: vec![vec![Condition::new("A", false), Condition::new("A", false)]],
        };
        let normalized = dup.normalize();
        assert_eq!(normalized.conditions.len(), 1);
        assert_eq!(normalized.conditions[0].len(), 1);
        assert_eq!(normalized.conditions[0][0].name, "A");
        assert!(!normalized.conditions[0][0].negated);
    }

    #[test]
    fn test_dnf_simplify_subsumption() {
        // A ∨ (A∧B) → A (B is redundant)
        let redundant = DisjConditions {
            conditions: vec![
                vec![Condition::new("A", false)],
                vec![Condition::new("A", false), Condition::new("B", false)],
            ],
        };
        let simplified = redundant.simplify();
        assert_eq!(
            simplified.conditions.len(),
            1,
            "A∨(A∧B) should simplify to A"
        );
        assert_eq!(simplified.conditions[0].len(), 1);
        assert_eq!(simplified.conditions[0][0].name, "A");
    }

    #[test]
    fn test_dnf_negate_double() {
        // !!A = A
        let a = DisjConditions {
            conditions: vec![vec![Condition::new("A", false)]],
        };
        let not_not_a = a.negate().negate();
        assert_eq!(not_not_a.conditions.len(), 1);
        assert_eq!(not_not_a.conditions[0].len(), 1);
        assert_eq!(not_not_a.conditions[0][0].name, "A");
        assert!(!not_not_a.conditions[0][0].negated);
    }

    #[test]
    fn test_dnf_negate_disjunction() {
        // !(A∨B) = !A∧!B
        let a_or_b = DisjConditions {
            conditions: vec![
                vec![Condition::new("A", false)],
                vec![Condition::new("B", false)],
            ],
        };
        let not_a_or_b = a_or_b.negate();
        // Should be: !A∧!B = [[!A, !B]]
        assert_eq!(not_a_or_b.conditions.len(), 1);
        let conj = &not_a_or_b.conditions[0];
        assert_eq!(conj.len(), 2);
        assert!(conj.iter().any(|c| c.name == "A" && c.negated));
        assert!(conj.iter().any(|c| c.name == "B" && c.negated));
    }
}
