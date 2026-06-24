// automake-rs-core: Makefile.am parser — forensic-parity implementation
//
// Court: AM.PARSER.MAKEFILE_AM.1
//
// Parses Makefile.am into a structured AST. Handles variable assignments,
// Automake primaries (bin_PROGRAMS, etc.), conditional blocks, comments,
// line continuations (backslash-newline), and passthrough make syntax.
//
// Clean-room reconstruction based on:
//   - Black-box oracle interrogation (running GNU automake on test inputs)
//   - GNU Automake manual §3, §4, §5, §6, §7, §8, §9, §10, §11, §12 (GFDL)
//   - GNU Make manual (GFDL, for passthrough syntax)
//   - POSIX make specification
// No GNU Automake GPL source code was consulted.

use std::fs;
use std::path::Path;

use crate::conditionals::{Condition, DisjConditions};

/// Parsed Makefile.am AST.
#[derive(Debug, Clone)]
pub struct MakefileAm {
    /// All statements in order
    pub statements: Vec<AmStatement>,
    /// Source file path
    pub source_path: Option<String>,
}

/// A single statement in a Makefile.am.
#[derive(Debug, Clone, PartialEq)]
pub enum AmStatement {
    /// A variable assignment: foo = bar
    VariableAssignment {
        name: String,
        op: AssignmentOp,
        values: Vec<String>,
        /// The conditional path under which this assignment occurs
        conditional: Option<DisjConditions>,
    },
    /// A make target/rule (passthrough)
    TargetRule {
        target: String,
        dependencies: Vec<String>,
        recipe_lines: Vec<String>,
    },
    /// An Automake primary: bin_PROGRAMS = hello
    Primary {
        /// Full variable name, e.g., "bin_PROGRAMS"
        var_name: String,
        /// The directory prefix, e.g., "bin"
        dir_prefix: String,
        /// Whether it's nodist_
        no_dist: bool,
        /// Whether it's nobase_ (preserve directory structure on install)
        nobase: bool,
        /// The primary kind, e.g., "PROGRAMS"
        primary: String,
        /// The values
        targets: Vec<String>,
    },
    /// A conditional block wrapping: if COND ... [else ...] endif
    ConditionalBlock {
        condition: String,
        negated: bool,
        if_branch: Vec<AmStatement>,
        else_branch: Vec<AmStatement>,
    },
    /// An include directive: include file.am
    Include(String),
    /// A comment line
    Comment(String),
    /// A blank line
    Blank,
}

/// Variable assignment operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentOp {
    /// = (lazy recursive expansion)
    Equals,
    /// += (append)
    Append,
    /// ?= (conditional assign — set if not already set)
    IfEquals,
    /// := (immediate expansion — GNU make extension)
    Override,
}

impl MakefileAm {
    /// Create an empty Makefile.am.
    pub fn new() -> Self {
        Self {
            statements: vec![],
            source_path: None,
        }
    }

    /// Parse a Makefile.am from a file, resolving include directives.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        let content = fs::read_to_string(path.as_ref()).map_err(ParseError::Io)?;
        let base_dir = path.as_ref().parent().unwrap_or(Path::new("."));
        let mut seen = std::collections::HashSet::new();
        let resolved = Self::resolve_includes(&content, base_dir, &mut seen)?;
        let mut am = Self::parse(&resolved)?;
        am.source_path = Some(path.as_ref().to_string_lossy().to_string());
        Ok(am)
    }

    /// Resolve include directives in the input, recursively merging included files.
    /// `seen` tracks canonical paths to prevent circular includes.
    /// Only resolves includes that reference literal filenames (no make variables).
    fn resolve_includes(
        input: &str,
        base_dir: &Path,
        seen: &mut std::collections::HashSet<String>,
    ) -> Result<String, ParseError> {
        let mut output = String::new();
        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("include ") {
                let file = trimmed.trim_start_matches("include ").trim();
                // Skip includes with make variables — they're resolved at make time
                if file.contains("$(") || file.contains("${") {
                    output.push_str(line);
                    output.push('\n');
                } else {
                    Self::merge_include_file(file, base_dir, seen, &mut output)?;
                }
            } else if trimmed.starts_with("-include ") {
                let file = trimmed.trim_start_matches("-include ").trim();
                if file.contains("$(") || file.contains("${") {
                    output.push_str(line);
                    output.push('\n');
                } else {
                    let _ = Self::merge_include_file(file, base_dir, seen, &mut output);
                }
            } else {
                output.push_str(line);
                output.push('\n');
            }
        }
        Ok(output)
    }

    /// Read an included file and merge its content (recursively resolving its includes).
    /// Returns Ok(()) if the file was resolved, or passes through the include line if not found.
    fn merge_include_file(
        file: &str,
        base_dir: &Path,
        seen: &mut std::collections::HashSet<String>,
        output: &mut String,
    ) -> Result<(), ParseError> {
        let inc_path = base_dir.join(file);
        if !inc_path.exists() {
            // File doesn't exist — pass through the include line unchanged
            return Ok(());
        }
        let canonical = inc_path.canonicalize().map_err(|e| ParseError::Parse {
            line: 0,
            msg: format!("include '{}': {}", file, e),
        })?;
        let canon_str = canonical.to_string_lossy().to_string();
        if !seen.insert(canon_str.clone()) {
            // Circular include — skip
            return Ok(());
        }
        let content = fs::read_to_string(&inc_path).map_err(|e| ParseError::Parse {
            line: 0,
            msg: format!("include '{}': {}", file, e),
        })?;
        let inc_dir = canonical.parent().unwrap_or(base_dir);
        let resolved = Self::resolve_includes(&content, inc_dir, seen)?;
        output.push_str(&resolved);
        Ok(())
    }

    /// Parse a Makefile.am string using the event-based CST parser.
    /// Architecture: Tokenizer → Parser Events → TreeSink → GreenNode → AST
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        crate::event_parser::parse(input)
    }

    /// Parse using the rowan direct parser (kept for comparison).
    #[allow(dead_code)]
    pub fn parse_rowan(input: &str) -> Result<Self, ParseError> {
        crate::rowan_parser::parse_rowan(input)
    }

    /// Parse using the legacy hand-rolled parser (kept for comparison).
    #[allow(dead_code)]
    pub fn parse_legacy(input: &str) -> Result<Self, ParseError> {
        MakefileAmParser::new(input).parse()
    }

    /// Get all primary variable names (e.g., "bin_PROGRAMS", "noinst_SCRIPTS").
    pub fn primaries(&self) -> Vec<&AmStatement> {
        self.statements
            .iter()
            .filter(|s| matches!(s, AmStatement::Primary { .. }))
            .collect()
    }

    /// Get all variable assignments (including primaries).
    pub fn variables(&self) -> Vec<&AmStatement> {
        self.statements
            .iter()
            .filter(|s| {
                matches!(
                    s,
                    AmStatement::VariableAssignment { .. } | AmStatement::Primary { .. }
                )
            })
            .collect()
    }

    /// Expand conditionals based on a set of true condition names.
    /// Returns a new MakefileAm with conditionals resolved.
    /// - If the condition is true, the if_branch is inlined.
    /// - If the condition is false and else_branch exists, else_branch is inlined.
    /// - If the condition is false and no else_branch, the block is dropped.
    /// - Nested conditionals are recursively expanded.
    /// - All VariableAssignment conditional fields are cleared (they've been resolved).
    pub fn expand_conditionals(&self, true_conditions: &std::collections::HashSet<String>) -> Self {
        let mut expanded = vec![];
        for stmt in &self.statements {
            match stmt {
                AmStatement::ConditionalBlock {
                    condition,
                    negated,
                    if_branch,
                    else_branch,
                } => {
                    let is_true = true_conditions.contains(condition);
                    let take_if = if *negated { !is_true } else { is_true };
                    let selected = if take_if { if_branch } else { else_branch };
                    // Recursively expand the selected branch
                    let branch_am = MakefileAm {
                        statements: selected.clone(),
                        source_path: self.source_path.clone(),
                    };
                    let branch_expanded = branch_am.expand_conditionals(true_conditions);
                    expanded.extend(branch_expanded.statements);
                }
                _ => expanded.push(stmt.clone()),
            }
        }
        // Strip conditional context from all expanded variables —
        // the conditions have been resolved, so conditional tags are stale.
        let cleaned: Vec<AmStatement> = expanded
            .into_iter()
            .map(|stmt| {
                if let AmStatement::VariableAssignment {
                    name, op, values, ..
                } = stmt
                {
                    AmStatement::VariableAssignment {
                        name,
                        op,
                        values,
                        conditional: None,
                    }
                } else {
                    stmt
                }
            })
            .collect();
        MakefileAm {
            statements: cleaned,
            source_path: self.source_path.clone(),
        }
    }

    /// Collect per-target flag variables for a given target.
    /// Returns (cflags, cxxflags, ldadd, ldflags, cppflags, dependencies).
    pub fn per_target_flags(&self, target: &str) -> PerTargetFlags {
        let mut flags = PerTargetFlags::default();
        let prefix = format!("{}_", target);
        for stmt in &self.statements {
            if let AmStatement::VariableAssignment { name, values, .. } = stmt {
                if let Some(suffix) = name.strip_prefix(&prefix) {
                    let val = values.join(" ");
                    match suffix {
                        "CFLAGS" => flags.cflags = Some(val),
                        "CXXFLAGS" => flags.cxxflags = Some(val),
                        "LDADD" => flags.ldadd = Some(val),
                        "LDFLAGS" => flags.ldflags = Some(val),
                        "CPPFLAGS" => flags.cppflags = Some(val),
                        "DEPENDENCIES" => flags.dependencies = Some(val),
                        "LIBADD" => flags.libadd = Some(val),
                        "SOURCES" => flags.sources = Some(val),
                        _ => {}
                    }
                }
            }
        }
        flags
    }
}

/// Per-target compiler/linker flags extracted from the Makefile.am.
#[derive(Debug, Clone, Default)]
pub struct PerTargetFlags {
    pub cflags: Option<String>,
    pub cxxflags: Option<String>,
    pub ldadd: Option<String>,
    pub ldflags: Option<String>,
    pub cppflags: Option<String>,
    pub dependencies: Option<String>,
    pub libadd: Option<String>,
    pub sources: Option<String>,
}

impl Default for MakefileAm {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal parser state machine.
struct MakefileAmParser<'a> {
    input: &'a str,
    /// Current position in bytes
    pos: usize,
    /// Current line number (1-based)
    line: usize,
    /// Current conditional stack (innermost first)
    condition_stack: Vec<Condition>,
}

impl<'a> MakefileAmParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
            condition_stack: vec![],
        }
    }

    fn parse(mut self) -> Result<MakefileAm, ParseError> {
        let mut statements = vec![];

        while !self.is_eof() {
            self.skip_blank_lines();

            if self.is_eof() {
                break;
            }

            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }

        Ok(MakefileAm {
            statements,
            source_path: None,
        })
    }

    fn parse_statement(&mut self) -> Result<Option<AmStatement>, ParseError> {
        let line = self.current_line().to_string();

        if line.is_empty() {
            self.advance_line();
            return Ok(Some(AmStatement::Blank));
        }

        if line.starts_with("#") {
            let comment = line.clone();
            self.advance_line();
            return Ok(Some(AmStatement::Comment(comment)));
        }

        // Handle conditional directives
        if let Some(stmt) = self.try_parse_conditional(&line) {
            return Ok(Some(stmt));
        }

        // Handle include directives
        if line.starts_with("include ") {
            let file = line.trim_start_matches("include ").trim().to_string();
            self.advance_line();
            return Ok(Some(AmStatement::Include(file)));
        }

        // Handle variable assignments (including primaries)
        if let Some((name, op, value)) = self.try_parse_assignment(&line) {
            // Check if this is an Automake primary
            if let Some(primary) = self.classify_primary(&name, &value) {
                self.advance_line();
                return Ok(Some(AmStatement::Primary {
                    var_name: name,
                    dir_prefix: primary.0,
                    no_dist: primary.1,
                    nobase: primary.2,
                    primary: primary.3,
                    targets: primary.4,
                }));
            }

            // Regular variable assignment
            self.advance_line();
            return Ok(Some(AmStatement::VariableAssignment {
                name,
                op,
                values: value,
                conditional: self.current_condition(),
            }));
        }

        // Handle target rules (lines with ':' that aren't assignments)
        if let Some((target, deps)) = self.try_parse_target(&line) {
            let recipe_lines = self.collect_recipe_lines();
            return Ok(Some(AmStatement::TargetRule {
                target,
                dependencies: deps,
                recipe_lines,
            }));
        }

        // Everything else is a passthrough: emit as a target rule with no recipe
        let line_str = line.clone();
        self.advance_line();
        // Try to parse as a target rule first (handles colons)
        if let Some((target, deps)) = Self::try_parse_target_static(&line_str) {
            let recipe_lines = self.collect_recipe_lines();
            return Ok(Some(AmStatement::TargetRule {
                target,
                dependencies: deps,
                recipe_lines,
            }));
        }
        Ok(Some(AmStatement::TargetRule {
            target: line_str,
            dependencies: vec![],
            recipe_lines: vec![],
        }))
    }

    /// Try to parse a conditional directive: if/else/endif (recursive body collection).
    /// When an `if COND` is found, this recursively parses the body until `else`/`endif`.
    fn try_parse_conditional(&mut self, line: &str) -> Option<AmStatement> {
        let trimmed = line.trim();
        if trimmed.starts_with("if! ") {
            let condition = trimmed.trim_start_matches("if! ").trim().to_string();
            self.condition_stack.push(Condition::new(&condition, true));
            self.advance_line();
            let if_body = self.collect_conditional_body();
            let else_body = if self.current_conditional_keyword() == Some("else") {
                self.advance_line();
                // Pop the negated condition for else branch context
                self.condition_stack.pop();
                self.condition_stack.push(Condition::new(&condition, false));
                let body = self.collect_conditional_body();
                self.condition_stack.pop();
                // Restore negated condition for endif
                self.condition_stack.push(Condition::new(&condition, true));
                body
            } else {
                vec![]
            };
            self.advance_line(); // consume endif
            self.condition_stack.pop(); // pop the condition
            Some(AmStatement::ConditionalBlock {
                condition,
                negated: true,
                if_branch: if_body,
                else_branch: else_body,
            })
        } else if trimmed.starts_with("if ") {
            let condition = trimmed.trim_start_matches("if ").trim().to_string();
            self.condition_stack.push(Condition::new(&condition, false));
            self.advance_line();
            let if_body = self.collect_conditional_body();
            let else_body = if self.current_conditional_keyword() == Some("else") {
                self.advance_line();
                // Pop the positive condition for else branch context
                self.condition_stack.pop();
                self.condition_stack.push(Condition::new(&condition, true));
                let body = self.collect_conditional_body();
                self.condition_stack.pop();
                // Restore positive condition for endif
                self.condition_stack.push(Condition::new(&condition, false));
                body
            } else {
                vec![]
            };
            self.advance_line(); // consume endif
            self.condition_stack.pop(); // pop the condition
            Some(AmStatement::ConditionalBlock {
                condition,
                negated: false,
                if_branch: if_body,
                else_branch: else_body,
            })
        } else {
            None
        }
    }

    /// Peek at the current line to see if it's a conditional keyword (else, endif).
    fn current_conditional_keyword(&self) -> Option<&str> {
        let line = self.current_line().trim();
        if line == "else" || line == "endif" {
            Some(line)
        } else {
            None
        }
    }

    /// Collect statements within a conditional body until `else` or `endif`.
    /// parse_statement() handles nested conditionals recursively, so we only
    /// need to check for `else`/`endif` at the TOP of the loop (after inner
    /// blocks have been fully consumed by recursive calls).
    fn collect_conditional_body(&mut self) -> Vec<AmStatement> {
        let mut body = vec![];
        while !self.is_eof() {
            self.skip_blank_lines();
            if self.is_eof() {
                break;
            }
            let kw = self.current_conditional_keyword();
            if kw == Some("else") || kw == Some("endif") {
                break;
            }
            if let Some(stmt) = self.parse_statement().unwrap_or(None) {
                body.push(stmt);
            }
        }
        body
    }
    fn try_parse_assignment(&self, _line: &str) -> Option<(String, AssignmentOp, Vec<String>)> {
        // Handle line continuations: join lines ending with \
        let full_line = self.read_continued_line();
        let content = &full_line;

        // Try different operators
        let (name, op, rest) = if let Some(pos) = content.find("+=") {
            let name = content[..pos].trim().to_string();
            let op = AssignmentOp::Append;
            let rest = content[pos + 2..].trim().to_string();
            (name, op, rest)
        } else if let Some(pos) = content.find("?=") {
            let name = content[..pos].trim().to_string();
            let op = AssignmentOp::IfEquals;
            let rest = content[pos + 2..].trim().to_string();
            (name, op, rest)
        } else if let Some(pos) = content.find(":=") {
            let name = content[..pos].trim().to_string();
            let op = AssignmentOp::Override;
            let rest = content[pos + 2..].trim().to_string();
            (name, op, rest)
        } else if let Some(pos) = content.find('=') {
            let name = content[..pos].trim().to_string();
            let op = AssignmentOp::Equals;
            let rest = content[pos + 1..].trim().to_string();
            (name, op, rest)
        } else {
            return None;
        };

        // Validate variable name
        if name.is_empty() || name.contains(char::is_whitespace) {
            return None;
        }

        // For primaries, split by whitespace (target names).
        // For regular variables, preserve the full value string.
        let is_primary = name.contains("PROGRAMS")
            || name.contains("LIBRARIES")
            || name.contains("SCRIPTS")
            || name.contains("DATA")
            || name.contains("HEADERS")
            || name.contains("MANS")
            || name.contains("TEXINFOS")
            || name.contains("TESTS")
            || name.contains("LISP")
            || name.contains("PYTHON")
            || name.contains("JAVA");

        let values: Vec<String> = if is_primary {
            rest.split_whitespace().map(|s| s.to_string()).collect()
        } else {
            // Preserve the full value — don't split by whitespace
            vec![rest]
        };

        Some((name, op, values))
    }

    /// Classify a variable name as an Automake primary, returning
    /// (dir_prefix, is_nodist, is_nobase, primary_kind, targets).
    fn classify_primary(
        &self,
        name: &str,
        values: &[String],
    ) -> Option<(String, bool, bool, String, Vec<String>)> {
        let known_primaries = [
            "LTLIBRARIES", // Check BEFORE LIBRARIES (longer match)
            "LIBRARIES",
            "PROGRAMS",
            "SCRIPTS",
            "DATA",
            "HEADERS",
            "MANS",
            "TEXINFOS",
            "TESTS",
            "LISP",
            "PYTHON",
            "JAVA",
        ];

        let known_prefixes = [
            "bin",
            "sbin",
            "libexec",
            "pkglibexec",
            "lib",
            "pkglib",
            "noinst",
            "check",
            "pkgdata",
            "lisp",
            "python",
            "java",
            "man",
            "info",
            "include",
            "oldinclude",
            // Custom dirs handled as primary with custom prefix
        ];

        for primary in &known_primaries {
            if let Some(prefix_part) = name.strip_suffix(primary) {
                let prefix = prefix_part.trim_end_matches('_');

                // Check for nodist_ and nobase_ prefixes
                let (dir_prefix, no_dist, nobase) =
                    if let Some(rest) = prefix.strip_prefix("nodist_") {
                        (rest.to_string(), true, false)
                    } else if let Some(rest) = prefix.strip_prefix("dist_") {
                        (rest.to_string(), false, false)
                    } else if let Some(rest) = prefix.strip_prefix("nobase_") {
                        (rest.to_string(), false, true)
                    } else if let Some(rest) = prefix.strip_prefix("nobase_dist_") {
                        (rest.to_string(), false, true)
                    } else if let Some(rest) = prefix.strip_prefix("nobase_nodist_") {
                        (rest.to_string(), true, true)
                    } else {
                        (prefix.to_string(), false, false)
                    };

                // Validate the prefix is known or acceptable
                if dir_prefix.is_empty()
                    || known_prefixes.contains(&dir_prefix.as_str())
                    || dir_prefix.chars().all(|c| c.is_alphanumeric() || c == '_')
                {
                    return Some((
                        dir_prefix,
                        no_dist,
                        nobase,
                        primary.to_string(),
                        values.to_vec(),
                    ));
                }
            }
        }

        None
    }

    /// Try to parse a target rule: target : deps (static version)
    fn try_parse_target_static(line: &str) -> Option<(String, Vec<String>)> {
        let trimmed = line.trim();
        if let Some(colon_pos) = trimmed.find(':') {
            if trimmed.contains("::") || trimmed.contains(":=") {
                return None;
            }
            let target = trimmed[..colon_pos].trim().to_string();
            let deps_part = trimmed[colon_pos + 1..].trim();
            let deps: Vec<String> = deps_part
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            return Some((target, deps));
        }
        None
    }

    /// Try to parse a target rule: target : deps
    fn try_parse_target(&self, line: &str) -> Option<(String, Vec<String>)> {
        Self::try_parse_target_static(line)
    }

    /// Collect recipe lines (indented with tab) following a target rule.
    fn collect_recipe_lines(&mut self) -> Vec<String> {
        let mut lines = vec![];
        self.advance_line();
        while !self.is_eof() {
            let line = self.current_line();
            if line.is_empty() || (!line.starts_with('\t') && !line.starts_with("    ")) {
                break;
            }
            lines.push(line.to_string());
            self.advance_line();
        }
        lines
    }

    /// Read a possibly line-continued logical line.
    fn read_continued_line(&self) -> String {
        let mut result = String::new();
        let mut pos = self.pos;

        loop {
            let remaining = &self.input[pos..];
            let line = if let Some(nl) = remaining.find('\n') {
                &remaining[..nl]
            } else {
                remaining
            };

            let trimmed = line.trim_end_matches('\r');

            if let Some(stripped) = trimmed.strip_suffix('\\') {
                // Line continues — strip the backslash and append a space
                result.push_str(stripped);
                result.push(' ');
                // Advance past the backslash and newline
                if let Some(nl) = remaining.find('\n') {
                    pos += nl + 1;
                } else {
                    break;
                }
            } else {
                result.push_str(line);
                break;
            }
        }

        result
    }

    /// Get the current logical line (no advance)
    fn current_line(&self) -> &str {
        if self.pos >= self.input.len() {
            return "";
        }

        let remaining = &self.input[self.pos..];
        if let Some(nl) = remaining.find('\n') {
            &remaining[..nl]
        } else {
            remaining
        }
    }

    /// Advance past the current line
    fn advance_line(&mut self) {
        if self.pos >= self.input.len() {
            return;
        }
        let remaining = &self.input[self.pos..];
        if let Some(nl) = remaining.find('\n') {
            self.pos += nl + 1;
            self.line += 1;
        } else {
            self.pos = self.input.len();
        }
    }

    /// Skip blank lines and lines with only whitespace
    fn skip_blank_lines(&mut self) {
        while !self.is_eof() {
            let line = self.current_line();
            if line.trim().is_empty() {
                self.advance_line();
            } else {
                break;
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn current_condition(&self) -> Option<DisjConditions> {
        if self.condition_stack.is_empty() {
            None
        } else {
            Some(DisjConditions {
                conditions: vec![self.condition_stack.clone()],
            })
        }
    }
}

/// Errors that can occur during Makefile.am parsing.
#[derive(Debug)]
pub enum ParseError {
    Io(std::io::Error),
    Parse { line: usize, msg: String },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Io(e) => write!(f, "I/O: {}", e),
            ParseError::Parse { line, msg } => write!(f, "line {}: {}", line, msg),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        ParseError::Io(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let am = MakefileAm::parse("").unwrap();
        assert!(am.statements.is_empty());
    }

    #[test]
    fn test_parse_simple_variable() {
        let am = MakefileAm::parse("bin_PROGRAMS = hello\n").unwrap();
        assert_eq!(am.statements.len(), 1);
        match &am.statements[0] {
            AmStatement::Primary {
                var_name,
                dir_prefix,
                primary,
                targets,
                ..
            } => {
                assert_eq!(var_name, "bin_PROGRAMS");
                assert_eq!(dir_prefix, "bin");
                assert_eq!(primary, "PROGRAMS");
                assert_eq!(targets, &vec!["hello".to_string()]);
            }
            _ => panic!("Expected Primary"),
        }
    }

    #[test]
    fn test_parse_append() {
        let am = MakefileAm::parse("CFLAGS = -Wall\nCFLAGS += -g\n").unwrap();
        assert_eq!(am.statements.len(), 2);
        match &am.statements[0] {
            AmStatement::VariableAssignment {
                name, op, values, ..
            } => {
                assert_eq!(name, "CFLAGS");
                assert_eq!(op, &AssignmentOp::Equals);
                assert_eq!(values, &vec!["-Wall".to_string()]);
            }
            _ => panic!("Expected VariableAssignment"),
        }
        match &am.statements[1] {
            AmStatement::VariableAssignment {
                name, op, values, ..
            } => {
                assert_eq!(name, "CFLAGS");
                assert_eq!(op, &AssignmentOp::Append);
                assert_eq!(values, &vec!["-g".to_string()]);
            }
            _ => panic!("Expected VariableAssignment"),
        }
    }

    #[test]
    fn test_parse_nodist() {
        let am = MakefileAm::parse("nodist_bin_SCRIPTS = myscript\n").unwrap();
        match &am.statements[0] {
            AmStatement::Primary {
                var_name,
                dir_prefix,
                no_dist,
                primary,
                targets,
                ..
            } => {
                assert_eq!(var_name, "nodist_bin_SCRIPTS");
                assert_eq!(dir_prefix, "bin");
                assert!(no_dist);
                assert_eq!(primary, "SCRIPTS");
                assert_eq!(targets, &vec!["myscript".to_string()]);
            }
            _ => panic!("Expected Primary"),
        }
    }

    #[test]
    fn test_parse_conditional() {
        let input = "if WANT_FOO\nbin_PROGRAMS = foo\nelse\nbin_PROGRAMS = bar\nendif\n";
        let am = MakefileAm::parse(input).unwrap();
        assert!(am.statements.len() >= 1);
        match &am.statements[0] {
            AmStatement::ConditionalBlock {
                condition, negated, ..
            } => {
                assert_eq!(condition, "WANT_FOO");
                assert!(!negated);
            }
            _ => panic!("Expected ConditionalBlock"),
        }
    }

    #[test]
    fn test_parse_negated_conditional() {
        // "if!" should produce negated=true on the ConditionalBlock
        let input = "if! NO_FOO\nVAR = bar\nendif\n";
        let am = MakefileAm::parse(input).unwrap();
        assert!(
            am.statements.len() >= 1,
            "Expected at least 1 statement, got {}",
            am.statements.len()
        );
        match &am.statements[0] {
            AmStatement::ConditionalBlock {
                condition, negated, ..
            } => {
                assert_eq!(condition, "NO_FOO");
                assert!(*negated, "if! should produce negated=true, got false");
            }
            other => panic!(
                "Expected ConditionalBlock, got {:?}",
                std::mem::discriminant(other)
            ),
        }
    }

    #[test]
    fn test_parse_line_continuation() {
        let input = "bin_PROGRAMS = hello \\\n  world\n";
        let am = MakefileAm::parse(input).unwrap();
        match &am.statements[0] {
            AmStatement::Primary { targets, .. } => {
                assert_eq!(targets, &vec!["hello".to_string(), "world".to_string()]);
            }
            _ => panic!("Expected Primary"),
        }
    }

    #[test]
    fn test_parse_comments() {
        let input = "# This is a comment\nbin_PROGRAMS = hello\n# Another comment\n";
        let am = MakefileAm::parse(input).unwrap();
        assert_eq!(am.statements.len(), 3);
        match &am.statements[0] {
            AmStatement::Comment(_) => {}
            _ => panic!("Expected Comment"),
        }
    }

    #[test]
    fn test_parse_multiple_primaries() {
        let input = "bin_PROGRAMS = hello goodbye\nnoinst_DATA = readme.txt\n";
        let am = MakefileAm::parse(input).unwrap();
        assert_eq!(am.statements.len(), 2);
        let primaries = am.primaries();
        assert_eq!(primaries.len(), 2);
    }
}
