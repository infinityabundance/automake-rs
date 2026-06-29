// automake-rs-core: Event-based parser architecture (rust-analyzer style)
//
// ARCHITECTURE:
//   Tokenizer → [lossless tokens]
//   Parser   → [Event stream]  (no tree knowledge, pure logic)
//   TreeSink → GreenNode       (consumes events, attaches trivia)
//   AST layer → AmStatement    (typed wrappers over CST)
//
// This separation makes the parser testable in isolation,
// enables error recovery, and follows the rust-analyzer pattern
// that has become the reference Rust parser architecture.
//
// Court: AM.PARSER.MAKEFILE_AM.1
// Reference: rust-analyzer crates/syntax + crates/parser

use rowan::{GreenNode, GreenNodeBuilder};

use crate::conditionals::{Condition, DisjConditions};
use crate::makefile_am::{AmStatement, AssignmentOp, MakefileAm, ParseError};

// ─── Syntax Kind ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[allow(dead_code)]
pub enum SyntaxKind {
    // Trivia (attached to tokens, not seen by parser)
    Whitespace = 0,
    Newline,
    Comment,

    // Tokens (seen by parser)
    Ident,
    Eq,
    PlusEq,
    QuestionEq,
    ColonEq,
    Colon,
    Backslash,
    Tab,
    LParen,
    RParen,
    Dollar,
    Text,

    // Composite nodes (from ungrammar)
    Root,
    SourceFile,
    Statement,
    VariableDef,
    SimpleAssign,
    AppendAssign,
    CondAssign,
    OverrideAssign,
    PrimaryAssign,
    PrimaryName,
    Name,
    Value,
    Target,
    Condition,
    FilePath,

    ConditionalBlock,
    IfBranch,
    IfKeyword,
    ElseBranch,

    TargetRule,
    Dependency,
    Recipe,

    IncludeDirective,
    IncludeKeyword,

    // Error recovery
    Error,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        rowan::SyntaxKind(kind as u16)
    }
}

// ─── Language ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MakeLanguage {}

impl rowan::Language for MakeLanguage {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Error as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

// ─── Events ───────────────────────────────────────────────────────

/// Parser event — emitted by the parser, consumed by TreeSink.
/// This is the core of the event-based architecture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Start a composite node of the given kind.
    StartNode { kind: SyntaxKind },
    /// Add a token (leaf) to the current node.
    AddToken { kind: SyntaxKind, text: String },
    /// Finish the current composite node.
    FinishNode,
    /// An error occurred at this position.
    Error { msg: String },
}

/// The TreeSink consumes events and builds a GreenNode.
/// It also handles whitespace attachment (trivia).
pub struct TreeSink {
    builder: GreenNodeBuilder<'static>,
    /// Pending trivia tokens to attach before next real token
    pending_trivia: Vec<(SyntaxKind, String)>,
    /// Whether we're inside a node (for error recovery)
    depth: usize,
}

impl Default for TreeSink {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeSink {
    pub fn new() -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            pending_trivia: Vec::new(),
            depth: 0,
        }
    }

    /// Process a single event, handling trivia attachment.
    pub fn process(&mut self, event: Event) {
        match event {
            Event::StartNode { kind } => {
                self.flush_trivia();
                self.builder.start_node(kind.into());
                self.depth += 1;
            }
            Event::AddToken { kind, text } => {
                // Attach trivia before the token
                self.flush_trivia();
                // Check if this is trivia itself
                if kind == SyntaxKind::Whitespace
                    || kind == SyntaxKind::Newline
                    || kind == SyntaxKind::Comment
                {
                    self.pending_trivia.push((kind, text));
                } else {
                    self.builder.token(kind.into(), &text);
                }
            }
            Event::FinishNode => {
                self.flush_trivia();
                if self.depth > 0 {
                    self.builder.finish_node();
                    self.depth -= 1;
                }
            }
            Event::Error { msg: _ } => {
                // Error events create ERROR nodes for resilience
                self.flush_trivia();
                self.builder.start_node(SyntaxKind::Error.into());
                self.builder.finish_node();
            }
        }
    }

    /// Flush pending trivia tokens into the tree as proper trivia nodes.
    fn flush_trivia(&mut self) {
        for (kind, text) in self.pending_trivia.drain(..) {
            self.builder.token(kind.into(), &text);
        }
    }

    /// Finish building and return the GreenNode.
    /// Ensures all pending trivia is flushed and nodes are balanced.
    pub fn finish(mut self) -> GreenNode {
        self.flush_trivia();
        // Close any remaining open nodes (error recovery)
        while self.depth > 0 {
            self.builder.finish_node();
            self.depth -= 1;
        }
        self.builder.finish()
    }
}

// ─── Token ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: SyntaxKind,
    pub text: String,
}

// ─── Tokenizer ────────────────────────────────────────────────────

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let bytes = input.as_bytes();
    let mut pos = 0usize;

    while pos < bytes.len() {
        let remaining = &input[pos..];

        if remaining.starts_with("\r\n") {
            tokens.push(Token {
                kind: SyntaxKind::Newline,
                text: "\r\n".into(),
            });
            pos += 2;
            continue;
        }
        if remaining.starts_with('\n') {
            tokens.push(Token {
                kind: SyntaxKind::Newline,
                text: "\n".into(),
            });
            pos += 1;
            continue;
        }
        if remaining.starts_with(' ') {
            let end = remaining
                .find(|c: char| c != ' ')
                .unwrap_or(remaining.len());
            tokens.push(Token {
                kind: SyntaxKind::Whitespace,
                text: remaining[..end].into(),
            });
            pos += end;
            continue;
        }
        if remaining.starts_with('\t') {
            tokens.push(Token {
                kind: SyntaxKind::Tab,
                text: "\t".into(),
            });
            pos += 1;
            continue;
        }
        if remaining.starts_with('#') {
            let end = remaining.find('\n').unwrap_or(remaining.len());
            tokens.push(Token {
                kind: SyntaxKind::Comment,
                text: remaining[..end].into(),
            });
            pos += end;
            continue;
        }
        if remaining.starts_with("+=") {
            tokens.push(Token {
                kind: SyntaxKind::PlusEq,
                text: "+=".into(),
            });
            pos += 2;
            continue;
        }
        if remaining.starts_with("?=") {
            tokens.push(Token {
                kind: SyntaxKind::QuestionEq,
                text: "?=".into(),
            });
            pos += 2;
            continue;
        }
        if remaining.starts_with(":=") {
            tokens.push(Token {
                kind: SyntaxKind::ColonEq,
                text: ":=".into(),
            });
            pos += 2;
            continue;
        }
        if remaining.starts_with('\\') {
            tokens.push(Token {
                kind: SyntaxKind::Backslash,
                text: "\\".into(),
            });
            pos += 1;
            continue;
        }
        if remaining.starts_with('=') {
            tokens.push(Token {
                kind: SyntaxKind::Eq,
                text: "=".into(),
            });
            pos += 1;
            continue;
        }
        if remaining.starts_with(':') {
            tokens.push(Token {
                kind: SyntaxKind::Colon,
                text: ":".into(),
            });
            pos += 1;
            continue;
        }
        if remaining.starts_with('$') {
            tokens.push(Token {
                kind: SyntaxKind::Dollar,
                text: "$".into(),
            });
            pos += 1;
            continue;
        }
        if remaining.starts_with('(') {
            tokens.push(Token {
                kind: SyntaxKind::LParen,
                text: "(".into(),
            });
            pos += 1;
            continue;
        }
        if remaining.starts_with(')') {
            tokens.push(Token {
                kind: SyntaxKind::RParen,
                text: ")".into(),
            });
            pos += 1;
            continue;
        }
        // Text token
        let end = remaining
            .find(|c: char| {
                matches!(
                    c,
                    '\n' | '\r' | ' ' | '\t' | '#' | '=' | ':' | '$' | '\\' | '(' | ')'
                )
            })
            .unwrap_or(remaining.len());
        if end > 0 {
            tokens.push(Token {
                kind: SyntaxKind::Text,
                text: remaining[..end].into(),
            });
            pos += end;
        } else {
            pos += 1;
        }
    }
    tokens
}

// ─── Event-Based Parser ───────────────────────────────────────────

/// Parse a Makefile.am string into AmStatement AST using the event pipeline.
pub fn parse(input: &str) -> Result<MakefileAm, ParseError> {
    let tokens = tokenize(input);
    let events = parse_tokens_to_events(&tokens)?;
    let mut sink = TreeSink::new();
    for event in &events {
        sink.process(event.clone());
    }
    let _green = sink.finish();
    // Convert events to AST (skip TreeSink for now, build AST directly from events)
    events_to_ast(&events)
}

/// Convert tokens to parser events.
fn parse_tokens_to_events(tokens: &[Token]) -> Result<Vec<Event>, ParseError> {
    let mut events: Vec<Event> = Vec::new();
    let mut pos = 0usize;
    let mut condition_stack: Vec<Condition> = Vec::new();

    events.push(Event::StartNode {
        kind: SyntaxKind::Root,
    });

    while pos < tokens.len() {
        // Emit trivia as AddToken events
        while pos < tokens.len()
            && matches!(
                tokens[pos].kind,
                SyntaxKind::Whitespace | SyntaxKind::Newline
            )
        {
            events.push(Event::AddToken {
                kind: tokens[pos].kind,
                text: tokens[pos].text.clone(),
            });
            pos += 1;
        }
        if pos >= tokens.len() {
            break;
        }

        let t = &tokens[pos];

        match t.kind {
            SyntaxKind::Comment => {
                events.push(Event::AddToken {
                    kind: SyntaxKind::Comment,
                    text: t.text.clone(),
                });
                pos += 1;
            }
            SyntaxKind::Text => {
                let text = t.text.as_str();
                match text {
                    "if" => {
                        pos += 1;
                        let cond = collect_line(tokens, &mut pos);
                        let condition = cond.trim().to_string();
                        events.push(Event::StartNode {
                            kind: SyntaxKind::ConditionalBlock,
                        });
                        events.push(Event::StartNode {
                            kind: SyntaxKind::IfBranch,
                        });
                        // Emit the condition text as a child of IfBranch
                        events.push(Event::StartNode {
                            kind: SyntaxKind::Condition,
                        });
                        events.push(Event::AddToken {
                            kind: SyntaxKind::Text,
                            text: condition.clone(),
                        });
                        events.push(Event::FinishNode); // Condition
                        condition_stack.push(Condition::new(&condition, false));
                        parse_body_to_events(
                            tokens,
                            &mut pos,
                            &mut events,
                            &mut condition_stack,
                            &["else", "endif"],
                        )?;
                        events.push(Event::FinishNode); // IfBranch
                                                        // Check for else
                        if peek_text(tokens, pos) == Some("else") {
                            pos += 1; // skip else
                            skip_trivia(tokens, &mut pos);
                            events.push(Event::StartNode {
                                kind: SyntaxKind::ElseBranch,
                            });
                            condition_stack.pop();
                            condition_stack.push(Condition::new(&condition, true));
                            parse_body_to_events(
                                tokens,
                                &mut pos,
                                &mut events,
                                &mut condition_stack,
                                &["endif"],
                            )?;
                            condition_stack.pop();
                            condition_stack.push(Condition::new(&condition, false));
                            events.push(Event::FinishNode); // ElseBranch
                        }
                        if peek_text(tokens, pos) == Some("endif") {
                            pos += 1;
                        }
                        condition_stack.pop();
                        events.push(Event::FinishNode); // ConditionalBlock
                    }
                    "if!" => {
                        pos += 1;
                        let cond = collect_line(tokens, &mut pos);
                        let condition = format!("!{}", cond.trim()); // Encode negated in condition
                        events.push(Event::StartNode {
                            kind: SyntaxKind::ConditionalBlock,
                        });
                        events.push(Event::StartNode {
                            kind: SyntaxKind::IfBranch,
                        });
                        // Emit condition text
                        events.push(Event::StartNode {
                            kind: SyntaxKind::Condition,
                        });
                        events.push(Event::AddToken {
                            kind: SyntaxKind::Text,
                            text: condition.clone(),
                        });
                        events.push(Event::FinishNode); // Condition
                        condition_stack.push(Condition::new(&condition, true));
                        parse_body_to_events(
                            tokens,
                            &mut pos,
                            &mut events,
                            &mut condition_stack,
                            &["else", "endif"],
                        )?;
                        events.push(Event::FinishNode);
                        if peek_text(tokens, pos) == Some("else") {
                            pos += 1;
                            skip_trivia(tokens, &mut pos);
                            events.push(Event::StartNode {
                                kind: SyntaxKind::ElseBranch,
                            });
                            condition_stack.pop();
                            condition_stack.push(Condition::new(&condition, false));
                            parse_body_to_events(
                                tokens,
                                &mut pos,
                                &mut events,
                                &mut condition_stack,
                                &["endif"],
                            )?;
                            condition_stack.pop();
                            condition_stack.push(Condition::new(&condition, true));
                            events.push(Event::FinishNode);
                        }
                        if peek_text(tokens, pos) == Some("endif") {
                            pos += 1;
                        }
                        condition_stack.pop();
                        events.push(Event::FinishNode);
                    }
                    "include" | "-include" => {
                        pos += 1;
                        let file = collect_line(tokens, &mut pos).trim().to_string();
                        events.push(Event::StartNode {
                            kind: SyntaxKind::IncludeDirective,
                        });
                        events.push(Event::AddToken {
                            kind: SyntaxKind::Text,
                            text: file,
                        });
                        events.push(Event::FinishNode);
                    }
                    _ => {
                        // Assignment or target rule
                        let line = collect_line_continued(tokens, &mut pos);
                        if !line.trim().is_empty() {
                            emit_line_events(&line, &mut events);
                        }
                    }
                }
            }
            SyntaxKind::Tab => {
                // Recipe line
                let recipe = collect_line(tokens, &mut pos);
                events.push(Event::StartNode {
                    kind: SyntaxKind::Recipe,
                });
                events.push(Event::AddToken {
                    kind: SyntaxKind::Text,
                    text: recipe,
                });
                events.push(Event::FinishNode);
            }
            _ => {
                // Unknown — error recovery
                events.push(Event::Error {
                    msg: format!("Unexpected token: {:?}", t.kind),
                });
                pos += 1;
            }
        }
    }

    events.push(Event::FinishNode); // Root
    Ok(events)
}

fn parse_body_to_events(
    tokens: &[Token],
    pos: &mut usize,
    events: &mut Vec<Event>,
    condition_stack: &mut Vec<Condition>,
    stop_at: &[&str],
) -> Result<(), ParseError> {
    let mut depth = 1u32;
    while *pos < tokens.len() {
        skip_trivia(tokens, pos);
        if *pos >= tokens.len() {
            break;
        }

        // Check for stop keywords at current depth
        if let Some(text) = peek_text(tokens, *pos) {
            if stop_at.contains(&text) && depth == 1 {
                break;
            }
            if text == "endif" {
                if depth == 1 {
                    break;
                }
                depth -= 1;
                *pos += 1;
                skip_trivia(tokens, pos);
                continue;
            }
        }

        let t = &tokens[*pos];
        match t.kind {
            SyntaxKind::Text => {
                let text = t.text.as_str();
                if text == "if" || text == "if!" {
                    depth += 1;
                    // Parse nested conditional recursively
                    let negated = text == "if!";
                    *pos += 1;
                    let cond = collect_line(tokens, pos).trim().to_string();
                    let condition = if negated {
                        cond.trim_start_matches("if! ").trim().to_string()
                    } else {
                        cond.trim_start_matches("if ").trim().to_string()
                    };
                    events.push(Event::StartNode {
                        kind: SyntaxKind::ConditionalBlock,
                    });
                    events.push(Event::StartNode {
                        kind: SyntaxKind::IfBranch,
                    });
                    // Emit condition text inside IfBranch (encode negated with ! prefix)
                    let cond_text = if negated {
                        format!("!{}", cond)
                    } else {
                        cond.clone()
                    };
                    events.push(Event::StartNode {
                        kind: SyntaxKind::Condition,
                    });
                    events.push(Event::AddToken {
                        kind: SyntaxKind::Text,
                        text: cond_text,
                    });
                    events.push(Event::FinishNode); // Condition
                    condition_stack.push(Condition::new(cond.trim(), negated));
                    parse_body_to_events(tokens, pos, events, condition_stack, &["else", "endif"])?;
                    events.push(Event::FinishNode);
                    if peek_text(tokens, *pos) == Some("else") {
                        *pos += 1;
                        skip_trivia(tokens, pos);
                        events.push(Event::StartNode {
                            kind: SyntaxKind::ElseBranch,
                        });
                        condition_stack.pop();
                        condition_stack.push(Condition::new(&condition, !negated));
                        parse_body_to_events(tokens, pos, events, condition_stack, &["endif"])?;
                        condition_stack.pop();
                        condition_stack.push(Condition::new(&condition, negated));
                        events.push(Event::FinishNode);
                    }
                    if peek_text(tokens, *pos) == Some("endif") {
                        *pos += 1;
                        depth -= 1;
                    }
                    condition_stack.pop();
                    events.push(Event::FinishNode);
                } else if text == "else" {
                    if depth == 1 {
                        break;
                    }
                    *pos += 1;
                } else {
                    let line = collect_line_continued(tokens, pos);
                    if !line.trim().is_empty() {
                        emit_line_events(&line, events);
                    }
                }
            }
            SyntaxKind::Comment => {
                events.push(Event::AddToken {
                    kind: SyntaxKind::Comment,
                    text: t.text.clone(),
                });
                *pos += 1;
            }
            SyntaxKind::Tab => {
                let recipe = collect_line(tokens, pos);
                events.push(Event::StartNode {
                    kind: SyntaxKind::Recipe,
                });
                events.push(Event::AddToken {
                    kind: SyntaxKind::Text,
                    text: recipe,
                });
                events.push(Event::FinishNode);
            }
            _ => {
                *pos += 1;
            }
        }
    }
    Ok(())
}

fn emit_line_events(line: &str, events: &mut Vec<Event>) {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return;
    }

    if let Some((name, op, values)) = parse_assignment(trimmed) {
        if let Some(primary) = classify_primary(&name, &values) {
            events.push(Event::StartNode {
                kind: SyntaxKind::PrimaryAssign,
            });
            events.push(Event::AddToken {
                kind: SyntaxKind::Text,
                text: name,
            });
            for tgt in &primary.4 {
                events.push(Event::AddToken {
                    kind: SyntaxKind::Text,
                    text: tgt.clone(),
                });
            }
            events.push(Event::FinishNode);
        } else {
            let kind = match op {
                AssignmentOp::Equals => SyntaxKind::SimpleAssign,
                AssignmentOp::Append => SyntaxKind::AppendAssign,
                AssignmentOp::IfEquals => SyntaxKind::CondAssign,
                AssignmentOp::Override => SyntaxKind::OverrideAssign,
            };
            events.push(Event::StartNode { kind });
            events.push(Event::AddToken {
                kind: SyntaxKind::Text,
                text: name,
            });
            events.push(Event::AddToken {
                kind: SyntaxKind::Text,
                text: values.join(" "),
            });
            events.push(Event::FinishNode);
        }
    } else if let Some((target, deps)) = parse_target_line(trimmed) {
        events.push(Event::StartNode {
            kind: SyntaxKind::TargetRule,
        });
        events.push(Event::AddToken {
            kind: SyntaxKind::Text,
            text: target,
        });
        for dep in &deps {
            events.push(Event::AddToken {
                kind: SyntaxKind::Text,
                text: dep.clone(),
            });
        }
        events.push(Event::FinishNode);
    } else {
        events.push(Event::AddToken {
            kind: SyntaxKind::Text,
            text: trimmed.to_string(),
        });
    }
}

// ─── Helpers ──────────────────────────────────────────────────────

fn peek_text(tokens: &[Token], pos: usize) -> Option<&str> {
    let mut i = pos;
    while i < tokens.len() && matches!(tokens[i].kind, SyntaxKind::Whitespace | SyntaxKind::Newline)
    {
        i += 1;
    }
    if i < tokens.len() && tokens[i].kind == SyntaxKind::Text {
        Some(tokens[i].text.as_str())
    } else {
        None
    }
}

fn skip_trivia(tokens: &[Token], pos: &mut usize) {
    while *pos < tokens.len()
        && matches!(
            tokens[*pos].kind,
            SyntaxKind::Whitespace | SyntaxKind::Newline
        )
    {
        *pos += 1;
    }
}

fn collect_line(tokens: &[Token], pos: &mut usize) -> String {
    let mut s = String::new();
    while *pos < tokens.len() {
        let t = &tokens[*pos];
        match t.kind {
            SyntaxKind::Newline => {
                *pos += 1;
                break;
            }
            _ => {
                s.push_str(&t.text);
                *pos += 1;
            }
        }
    }
    s
}

fn collect_line_continued(tokens: &[Token], pos: &mut usize) -> String {
    let mut s = String::new();
    while *pos < tokens.len() {
        let t = &tokens[*pos];
        match t.kind {
            SyntaxKind::Newline => {
                *pos += 1;
                break;
            }
            SyntaxKind::Backslash => {
                *pos += 1;
                if *pos < tokens.len() && tokens[*pos].kind == SyntaxKind::Newline {
                    // genuine line-continuation: consume the newline + leading whitespace
                    *pos += 1;
                    while *pos < tokens.len() && tokens[*pos].kind == SyntaxKind::Whitespace {
                        *pos += 1;
                    }
                } else {
                    // a LITERAL backslash in the value, e.g. `\"` in -DFOO="\"$(path)\"" — preserve it,
                    // or automake-rs emits `""...""` and the compiler sees an unquoted path
                    // ("expected expression before '/'"). Only `\`+newline is a make continuation.
                    s.push('\\');
                }
            }
            _ => {
                s.push_str(&t.text);
                *pos += 1;
            }
        }
    }
    s
}

// ─── Semantic helpers (shared with rowan_parser) ──────────────────

fn parse_assignment(content: &str) -> Option<(String, AssignmentOp, Vec<String>)> {
    if content.starts_with('\t') {
        return None;
    }
    let (name, op, rest) = if let Some(p) = content.find("+=") {
        (
            content[..p].trim().to_string(),
            AssignmentOp::Append,
            content[p + 2..].trim(),
        )
    } else if let Some(p) = content.find("?=") {
        (
            content[..p].trim().to_string(),
            AssignmentOp::IfEquals,
            content[p + 2..].trim(),
        )
    } else if let Some(p) = content.find(":=") {
        (
            content[..p].trim().to_string(),
            AssignmentOp::Override,
            content[p + 2..].trim(),
        )
    } else if let Some(p) = content.find('=') {
        (
            content[..p].trim().to_string(),
            AssignmentOp::Equals,
            content[p + 1..].trim(),
        )
    } else {
        return None;
    };
    if name.is_empty() || name.contains(char::is_whitespace) {
        return None;
    }
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
    let values = if is_primary {
        // For primaries the value is a target list, so an inline `#` comment must be stripped
        // (Automake excludes it from the targets). Non-primary variables preserve the full value
        // verbatim, including any trailing comment, exactly as the GNU oracle does.
        let r = match rest.find('#') {
            Some(i) => rest[..i].trim_end(),
            None => rest,
        };
        r.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        vec![rest.to_string()]
    };
    Some((name, op, values))
}

fn classify_primary(
    name: &str,
    values: &[String],
) -> Option<(String, bool, bool, String, Vec<String>)> {
    let known = [
        "LTLIBRARIES",
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
    let prefixes = [
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
    ];
    for p in &known {
        if let Some(pp) = name.strip_suffix(p) {
            let prefix = pp.trim_end_matches('_');
            let (dir, nd, nb) = if let Some(r) = prefix.strip_prefix("nodist_") {
                (r.to_string(), true, false)
            } else if let Some(r) = prefix.strip_prefix("dist_") {
                (r.to_string(), false, false)
            } else if let Some(r) = prefix.strip_prefix("nobase_") {
                (r.to_string(), false, true)
            } else {
                (prefix.to_string(), false, false)
            };
            if dir.is_empty()
                || prefixes.contains(&dir.as_str())
                || dir.chars().all(|c| c.is_alphanumeric() || c == '_')
            {
                return Some((dir, nd, nb, p.to_string(), values.to_vec()));
            }
        }
    }
    None
}

fn parse_target_line(line: &str) -> Option<(String, Vec<String>)> {
    let t = line.trim();
    if t.is_empty()
        || t.starts_with("if ")
        || t.starts_with("if! ")
        || t == "else"
        || t == "endif"
        || t.starts_with("include ")
        || t.starts_with("-include ")
    {
        return None;
    }
    if let Some(p) = t.find(':') {
        if t.contains("::") || t.contains(":=") {
            return None;
        }
        let target = t[..p].trim().to_string();
        let deps: Vec<String> = t[p + 1..]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        return Some((target, deps));
    }
    None
}

// ─── Events → AST ─────────────────────────────────────────────────

/// Convert parser events to our AmStatement AST.
/// Walks the event stream and reconstructs the semantic structure.
fn events_to_ast(events: &[Event]) -> Result<MakefileAm, ParseError> {
    let mut statements = Vec::new();
    let mut condition_stack: Vec<Condition> = Vec::new();
    let mut idx = 0usize;

    // Skip Root StartNode
    if idx < events.len()
        && matches!(
            &events[idx],
            Event::StartNode {
                kind: SyntaxKind::Root
            }
        )
    {
        idx += 1;
    }

    while idx < events.len() {
        match &events[idx] {
            Event::StartNode { kind } => match kind {
                SyntaxKind::ConditionalBlock => {
                    let (stmt, next) =
                        parse_conditional_from_events(events, idx, &mut condition_stack)?;
                    statements.push(stmt);
                    idx = next;
                    continue;
                }
                SyntaxKind::SimpleAssign
                | SyntaxKind::AppendAssign
                | SyntaxKind::CondAssign
                | SyntaxKind::OverrideAssign => {
                    let (stmt, next) =
                        parse_assign_from_events(events, idx, kind, &condition_stack);
                    statements.push(stmt);
                    idx = next;
                    continue;
                }
                SyntaxKind::PrimaryAssign => {
                    let (stmt, next) = parse_primary_from_events(events, idx);
                    statements.push(stmt);
                    idx = next;
                    continue;
                }
                SyntaxKind::TargetRule => {
                    let (stmt, next) = parse_target_from_events(events, idx);
                    statements.push(stmt);
                    idx = next;
                    continue;
                }
                SyntaxKind::Recipe => {
                    // Attach recipe to the last TargetRule
                    if let Some(AmStatement::TargetRule {
                        ref mut recipe_lines,
                        ..
                    }) = statements.last_mut()
                    {
                        let recipe_text = extract_recipe_text(events, idx);
                        recipe_lines.push(recipe_text);
                    }
                    idx = skip_node(events, idx);
                    continue;
                }
                SyntaxKind::IncludeDirective => {
                    let (stmt, next) = parse_include_from_events(events, idx);
                    statements.push(stmt);
                    idx = next;
                    continue;
                }
                _ => {
                    idx += 1;
                }
            },
            Event::AddToken { kind, text } => {
                if *kind == SyntaxKind::Comment {
                    statements.push(AmStatement::Comment(text.clone()));
                }
                idx += 1;
            }
            _ => {
                idx += 1;
            }
        }
    }

    Ok(MakefileAm {
        statements,
        source_path: None,
    })
}

fn parse_conditional_from_events(
    events: &[Event],
    start: usize,
    condition_stack: &mut Vec<Condition>,
) -> Result<(AmStatement, usize), ParseError> {
    let mut idx = start + 1; // Skip StartNode ConditionalBlock
    let mut condition = String::new();
    let mut negated = false;
    let mut if_body = Vec::new();
    let mut else_body = Vec::new();

    while idx < events.len() {
        match &events[idx] {
            Event::StartNode {
                kind: SyntaxKind::Condition,
            } => {
                // Extract condition text
                idx += 1;
                while idx < events.len() {
                    match &events[idx] {
                        Event::AddToken {
                            kind: SyntaxKind::Text,
                            text,
                        } => {
                            // Detect negated: "!COND" → condition="COND", negated=true
                            if let Some(stripped) = text.strip_prefix('!') {
                                negated = true;
                                condition = stripped.to_string();
                            } else {
                                condition = text.clone();
                            }
                            idx += 1;
                        }
                        Event::FinishNode => {
                            idx += 1;
                            break;
                        }
                        _ => {
                            idx += 1;
                        }
                    }
                }
            }
            Event::StartNode {
                kind: SyntaxKind::IfBranch,
            } => {
                // Peek inside IfBranch to extract condition before recursing
                let mut peek_idx = idx + 1;
                while peek_idx < events.len() {
                    match &events[peek_idx] {
                        Event::StartNode {
                            kind: SyntaxKind::Condition,
                        } => {
                            peek_idx += 1;
                            while peek_idx < events.len() {
                                if let Event::AddToken {
                                    kind: SyntaxKind::Text,
                                    text,
                                } = &events[peek_idx]
                                {
                                    // Detect negated from "!" prefix
                                    if let Some(stripped) = text.strip_prefix('!') {
                                        negated = true;
                                        condition = stripped.to_string();
                                    } else {
                                        condition = text.clone();
                                    }
                                    break;
                                }
                                peek_idx += 1;
                            }
                            break;
                        }
                        _ => {
                            peek_idx += 1;
                        }
                    }
                }
                idx += 1;
                // Push condition onto stack so inner variables get tagged
                condition_stack.push(Condition::new(&condition, negated));
                let (sub_stmts, next) = collect_statement_events(events, idx, condition_stack)?;
                condition_stack.pop();
                if_body = sub_stmts;
                idx = next;
            }
            Event::StartNode {
                kind: SyntaxKind::ElseBranch,
            } => {
                idx += 1;
                // Push opposite condition for else branch
                condition_stack.push(Condition::new(&condition, !negated));
                let (sub_stmts, next) = collect_statement_events(events, idx, condition_stack)?;
                condition_stack.pop();
                else_body = sub_stmts;
                idx = next;
            }
            Event::FinishNode => {
                idx += 1;
                break;
            }
            _ => {
                idx += 1;
            }
        }
    }
    Ok((
        AmStatement::ConditionalBlock {
            condition,
            negated,
            if_branch: if_body,
            else_branch: else_body,
        },
        idx,
    ))
}

fn collect_statement_events(
    events: &[Event],
    mut idx: usize,
    condition_stack: &mut Vec<Condition>,
) -> Result<(Vec<AmStatement>, usize), ParseError> {
    let mut stmts = Vec::new();
    while idx < events.len() {
        match &events[idx] {
            Event::FinishNode => {
                idx += 1;
                break;
            }
            Event::StartNode { kind } => match kind {
                SyntaxKind::ConditionalBlock => {
                    let (s, n) = parse_conditional_from_events(events, idx, condition_stack)?;
                    stmts.push(s);
                    idx = n;
                    continue;
                }
                SyntaxKind::SimpleAssign
                | SyntaxKind::AppendAssign
                | SyntaxKind::CondAssign
                | SyntaxKind::OverrideAssign => {
                    let (s, n) = parse_assign_from_events(events, idx, kind, condition_stack);
                    stmts.push(s);
                    idx = n;
                    continue;
                }
                SyntaxKind::PrimaryAssign => {
                    let (s, n) = parse_primary_from_events(events, idx);
                    stmts.push(s);
                    idx = n;
                    continue;
                }
                SyntaxKind::Condition => {
                    // Condition node — skip (extracted by caller)
                    idx = skip_node(events, idx);
                    continue;
                }
                SyntaxKind::Recipe => {
                    // Attach recipe to the last TargetRule in this scope
                    if let Some(AmStatement::TargetRule {
                        ref mut recipe_lines,
                        ..
                    }) = stmts.last_mut()
                    {
                        let recipe_text = extract_recipe_text(events, idx);
                        recipe_lines.push(recipe_text);
                    }
                    idx = skip_node(events, idx);
                    continue;
                }
                _ => {
                    idx += 1;
                }
            },
            Event::AddToken {
                kind: SyntaxKind::Comment,
                text,
            } => {
                stmts.push(AmStatement::Comment(text.clone()));
                idx += 1;
            }
            _ => {
                idx += 1;
            }
        }
    }
    Ok((stmts, idx))
}

fn parse_assign_from_events(
    events: &[Event],
    start: usize,
    kind: &SyntaxKind,
    condition_stack: &[Condition],
) -> (AmStatement, usize) {
    let mut idx = start + 1;
    let mut name = String::new();
    let mut value = String::new();
    while idx < events.len() {
        match &events[idx] {
            Event::AddToken {
                kind: SyntaxKind::Text,
                text,
            } => {
                if name.is_empty() {
                    name = text.clone();
                } else {
                    value = text.clone();
                }
                idx += 1;
            }
            Event::FinishNode => {
                idx += 1;
                break;
            }
            _ => {
                idx += 1;
            }
        }
    }
    let op = match kind {
        SyntaxKind::AppendAssign => AssignmentOp::Append,
        SyntaxKind::CondAssign => AssignmentOp::IfEquals,
        SyntaxKind::OverrideAssign => AssignmentOp::Override,
        _ => AssignmentOp::Equals,
    };
    let cond = if condition_stack.is_empty() {
        None
    } else {
        Some(DisjConditions {
            conditions: vec![condition_stack.to_vec()],
        })
    };
    (
        AmStatement::VariableAssignment {
            name,
            op,
            values: vec![value],
            conditional: cond,
        },
        idx,
    )
}

fn parse_primary_from_events(events: &[Event], start: usize) -> (AmStatement, usize) {
    let mut idx = start + 1;
    let mut var_name = String::new();
    let mut targets = Vec::new();
    while idx < events.len() {
        match &events[idx] {
            Event::AddToken {
                kind: SyntaxKind::Text,
                text,
            } => {
                if var_name.is_empty() {
                    var_name = text.clone();
                } else {
                    targets.push(text.clone());
                }
                idx += 1;
            }
            Event::FinishNode => {
                idx += 1;
                break;
            }
            _ => {
                idx += 1;
            }
        }
    }
    if let Some(primary) = classify_primary(&var_name, &targets) {
        (
            AmStatement::Primary {
                var_name,
                dir_prefix: primary.0,
                no_dist: primary.1,
                nobase: primary.2,
                primary: primary.3,
                targets: primary.4,
            },
            idx,
        )
    } else {
        (
            AmStatement::VariableAssignment {
                name: var_name,
                op: AssignmentOp::Equals,
                values: targets,
                conditional: None,
            },
            idx,
        )
    }
}

fn parse_target_from_events(events: &[Event], start: usize) -> (AmStatement, usize) {
    let mut idx = start + 1;
    let mut target = String::new();
    let mut deps = Vec::new();
    while idx < events.len() {
        match &events[idx] {
            Event::AddToken {
                kind: SyntaxKind::Text,
                text,
            } => {
                if target.is_empty() {
                    target = text.clone();
                } else {
                    deps.push(text.clone());
                }
                idx += 1;
            }
            Event::FinishNode => {
                idx += 1;
                break;
            }
            _ => {
                idx += 1;
            }
        }
    }
    (
        AmStatement::TargetRule {
            target,
            dependencies: deps,
            recipe_lines: vec![],
        },
        idx,
    )
}

fn parse_include_from_events(events: &[Event], start: usize) -> (AmStatement, usize) {
    let mut idx = start + 1;
    let mut file = String::new();
    while idx < events.len() {
        match &events[idx] {
            Event::AddToken {
                kind: SyntaxKind::Text,
                text,
            } => {
                file = text.clone();
                idx += 1;
            }
            Event::FinishNode => {
                idx += 1;
                break;
            }
            _ => {
                idx += 1;
            }
        }
    }
    (AmStatement::Include(file), idx)
}

/// Extract the text content from a Recipe node.
fn extract_recipe_text(events: &[Event], start: usize) -> String {
    let mut idx = start + 1;
    let mut text = String::new();
    while idx < events.len() {
        match &events[idx] {
            Event::AddToken { text: t, .. } => {
                text.push_str(t);
                idx += 1;
            }
            Event::FinishNode => {
                break;
            }
            _ => {
                idx += 1;
            }
        }
    }
    text
}

/// Skip past a node and all its children, returning the index after the FinishNode.
fn skip_node(events: &[Event], start: usize) -> usize {
    let mut idx = start + 1;
    let mut depth = 1u32;
    while idx < events.len() && depth > 0 {
        match &events[idx] {
            Event::StartNode { .. } => depth += 1,
            Event::FinishNode => depth -= 1,
            _ => {}
        }
        idx += 1;
    }
    idx
}

// ─── Public API ───────────────────────────────────────────────────
// The event-based parser is the primary parser.
// See rowan_parser.rs for the legacy direct parser.
