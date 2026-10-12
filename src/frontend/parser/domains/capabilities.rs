
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Capability-domain syntax adapter for Zamani.
//!
//! Path: src/frontend/parser/domains/capabilities.rs
//! Rust: 1.97+
//! Edition: 2021
//!
//! # Authority
//!
//! The normative language specification and canonical grammar hierarchy
//! govern accepted capability syntax. Relevant grammar files include:
//!
//! - grammar/Zamani.g4
//! - grammar/antlr/ZamaniLexer.g4
//! - grammar/core/capabilities.g4
//! - grammar/resources/capabilities.g4
//! - grammar/lexer/tokens.g4
//!
//! This Rust module is an adapter, not a competing grammar authority.
//! Production integration must verify that its accepted clause boundaries
//! and token spellings conform to the selected language edition.
//!
//! # Purpose
//!
//! Preserve capability-related syntax for subsequent canonical AST
//! adaptation and semantic analysis. This module supports capability
//! assertions, requirements, constraints, preferences, hints, target
//! requirements, availability declarations, relationships, and structured
//! compositions at the token level.
//!
//! Capability names, namespace segments, properties, and relationship names
//! are open-world identifiers. Adding a new capability must not require a
//! new enum variant or parser branch.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - CapabilityParser and CapabilityParserLimits.
//! - CapabilityItem, CapabilityClause, and CapabilityClauseKind.
//! - CapabilitySyntaxToken.
//! - CapabilityParseError and CapabilityParseErrorKind.
//! - Local token-stream validation and balanced-delimiter collection.
//!
//! This module does not own:
//!
//! - Lexing, token identity, source text, or canonical source spans.
//! - The canonical AST or semantic capability model.
//! - General expression parsing or expression precedence.
//! - Name resolution, capability existence, or version resolution.
//! - Authorization, effect checking, policy evaluation, or provenance.
//! - Hardware discovery, target selection, resource allocation, or execution.
//!
//! # Integration contract
//!
//! - Consumes `crate::frontend::lexer::token::Token`.
//! - Reuses `TokenKind` and the canonical `Span`.
//! - The shared parser calls `CapabilityParser::parse_next` only at a
//!   capability grammar production or an explicitly selected extension point.
//! - `domains/mod.rs` exports this module.
//! - `ast_adapter.rs` maps these syntax records to canonical AST nodes.
//! - `session.rs` coordinates parser-wide budgets and diagnostic collection.
//! - Semantic analysis resolves capability identity, versions, properties,
//!   relationships, authorization, and target feasibility.
//! - The resource parser and capability parser must not both consume the
//!   same clause. The parent grammar owns dispatch.
//!
//! # Safety and scalability
//!
//! No unsafe Rust is used. Collections grow with the source and available
//! memory. There are no language-level ceilings for capability count,
//! namespace depth, property count, or domain count.
//!
//! Optional limits are operational safeguards, not hardware-capacity limits.
//! `usize::MAX` means that a particular local budget is not explicitly
//! constrained. It does not promise infinite memory or execution time.
//!
//! # Loss-preservation contract
//!
//! Token spelling, token kind, token order, and source spans are retained.
//! Trivia-channel tokens are skipped when identifying clause boundaries.
//! The original tokens are not mutated. Since the lexer may discard trivia,
//! this adapter does not claim to reconstruct exact source whitespace.
//!
//! The syntax records below are intermediate adapter output. They must not
//! be mistaken for the canonical Zamani AST.

use core::fmt;
use std::error::Error;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

// -----------------------------------------------------------------------------
// Operational limits
// -----------------------------------------------------------------------------

/// Operational safeguards for capability syntax parsing.
///
/// These settings do not restrict what capabilities a program may describe.
/// The embedding parser should reconcile them with its shared parser budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityParserLimits {
    /// Maximum significant tokens retained for one clause.
    pub max_tokens_per_clause: usize,

    /// Maximum simultaneous delimiter nesting.
    pub max_nesting_depth: usize,

    /// Maximum clauses returned by one `parse_all` call.
    pub max_clauses: usize,

    /// Permit a clause with no payload after its introducer.
    ///
    /// Keep this disabled for normal source parsing. Enable it only for a
    /// specific grammar profile that explicitly permits empty payloads.
    pub allow_empty_payload: bool,
}

impl Default for CapabilityParserLimits {
    fn default() -> Self {
        Self {
            max_tokens_per_clause: usize::MAX,
            max_nesting_depth: usize::MAX,
            max_clauses: usize::MAX,
            allow_empty_payload: false,
        }
    }
}

// -----------------------------------------------------------------------------
// Loss-preserving syntax records
// -----------------------------------------------------------------------------

/// An owned snapshot of one canonical lexer token.
///
/// This is not an alternative lexer token definition. It preserves the
/// information needed by later AST adaptation after the input borrow ends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilitySyntaxToken {
    /// Canonical lexical identity.
    pub kind: TokenKind,

    /// Original token spelling, without normalization.
    pub spelling: String,

    /// Canonical source span.
    pub span: Span,
}

impl CapabilitySyntaxToken {
    fn from_token(token: &Token<'_>) -> Self {
        Self {
            kind: token.kind(),
            spelling: token.lexeme().to_owned(),
            span: *token.span(),
        }
    }
}

/// Syntactic category of a capability clause.
///
/// This enum describes clause structure, not the identity or availability
/// of any capability. New capability names do not require enum changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CapabilityClauseKind {
    /// `capability <expression>;`
    Assertion,

    /// `requires capability <expression>;`
    Requirement,

    /// `constraint ...;`
    Constraint,

    /// `prefer ...;`
    Preference,

    /// `hint ...;`
    Hint,

    /// `target capability <expression>;`
    TargetRequirement,

    /// `capability <expression> availability = <expression>;`
    Availability,

    /// `capability <reference> <relationship> capability <reference>;`
    Relationship,

    /// `capability { ... };`
    Composition,
}

impl CapabilityClauseKind {
    /// Classify a clause introducer by its spelling.
    ///
    /// Matching is ASCII case-insensitive for compatibility with profiles
    /// that do not treat keyword case as significant. The original spelling
    /// remains available in the token sequence.
    ///
    /// Returns `None` for an unrecognized introducer.
    #[must_use]
    pub fn from_introducer(introducer: &str) -> Option<Self> {
        if introducer.eq_ignore_ascii_case("capability") {
            Some(Self::Assertion)
        } else if introducer.eq_ignore_ascii_case("requires")
            || introducer.eq_ignore_ascii_case("require")
        {
            Some(Self::Requirement)
        } else if introducer.eq_ignore_ascii_case("constraint")
            || introducer.eq_ignore_ascii_case("constrain")
        {
            Some(Self::Constraint)
        } else if introducer.eq_ignore_ascii_case("prefer") {
            Some(Self::Preference)
        } else if introducer.eq_ignore_ascii_case("hint") {
            Some(Self::Hint)
        } else if introducer.eq_ignore_ascii_case("target") {
            Some(Self::TargetRequirement)
        } else {
            None
        }
    }
}

/// One recognized capability-oriented clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityClause {
    /// Syntactic classification.
    pub kind: CapabilityClauseKind,

    /// All retained tokens, including the introducer and any terminator.
    pub tokens: Vec<CapabilitySyntaxToken>,

    /// Tokens after the introducer and before a trailing semicolon.
    ///
    /// The payload is deliberately not interpreted as a general expression.
    /// The canonical grammar and AST adapter own that interpretation.
    pub payload: Vec<CapabilitySyntaxToken>,

    /// Span from the first retained token through the last retained token.
    pub span: Span,

    /// Whether a top-level semicolon terminated the clause.
    pub terminated: bool,
}

impl CapabilityClause {
    /// Return the source spelling of the introducer, if present.
    #[must_use]
    pub fn introducer(&self) -> Option<&str> {
        self.tokens.first().map(|token| token.spelling.as_str())
    }

    /// Return the payload's token spellings concatenated without whitespace.
    ///
    /// This is a debugging convenience, not a source formatter. The lexer
    /// may have discarded original whitespace and comments.
    #[must_use]
    pub fn compact_payload(&self) -> String {
        let capacity = self.payload.iter().fold(0usize, |total, token| {
            total.saturating_add(token.spelling.len())
        });

        let mut output = String::with_capacity(capacity);
        for token in &self.payload {
            output.push_str(&token.spelling);
        }
        output
    }
}

/// Result of parsing one capability clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityItem {
    Clause(CapabilityClause),
}

// -----------------------------------------------------------------------------
// Structured errors
// -----------------------------------------------------------------------------

/// Machine-readable capability parser failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CapabilityParseErrorKind {
    /// The input has no EOF token, multiple EOF tokens, or tokens after EOF.
    InvalidTokenStream,

    /// The current clause does not start with a supported introducer.
    UnexpectedToken,

    /// The clause has no payload when a payload is required.
    EmptyPayload,

    /// A closing delimiter has no corresponding opening delimiter.
    UnmatchedClosingDelimiter,

    /// A closing delimiter does not match the most recent opening delimiter.
    MismatchedDelimiter,

    /// The input ends before all opened delimiters are closed.
    UnclosedDelimiter,

    /// The configured clause token budget was exceeded.
    ClauseTokenBudgetExceeded,

    /// The configured nesting budget was exceeded.
    NestingBudgetExceeded,

    /// The configured number of clauses was exceeded.
    ClauseCountBudgetExceeded,

    /// A span could not be composed consistently.
    InvalidSpan,

    /// A parse operation failed to advance.
    NoProgress,
}

/// Structured error returned by the capability adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityParseError {
    /// Machine-readable error classification.
    pub kind: CapabilityParseErrorKind,

    /// Human-readable explanation.
    pub message: String,

    /// Best available source span.
    pub span: Option<Span>,

    /// Expected syntax, when relevant.
    pub expected: Option<String>,

    /// Actual syntax, when relevant.
    pub found: Option<String>,
}

impl CapabilityParseError {
    fn new(
        kind: CapabilityParseErrorKind,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
            expected: None,
            found: None,
        }
    }

    fn with_expected(
        mut self,
        expected: impl Into<String>,
        found: Option<String>,
    ) -> Self {
        self.expected = Some(expected.into());
        self.found = found;
        self
    }
}

impl fmt::Display for CapabilityParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for CapabilityParseError {}

// -----------------------------------------------------------------------------
// Parser
// -----------------------------------------------------------------------------

/// Incremental parser for capability-oriented syntax.
///
/// This adapter consumes the canonical lexer's token stream. It does not
/// tokenize source, resolve names, evaluate expressions, or validate a target.
///
/// The token stream must contain exactly one EOF token as its final element.
/// The parent parser should normally use `parse_next`; `parse_all` is intended
/// for a stream dedicated to capability clauses.
#[derive(Debug)]
pub struct CapabilityParser<'tokens, 'source> {
    tokens: &'tokens [Token<'source>],
    index: usize,
    eof_index: usize,
    limits: CapabilityParserLimits,
    parsed_clauses: usize,
}

impl<'tokens, 'source> CapabilityParser<'tokens, 'source> {
    /// Validate a token stream and create a parser at its beginning.
    pub fn new(
        tokens: &'tokens [Token<'source>],
        limits: CapabilityParserLimits,
    ) -> Result<Self, CapabilityParseError> {
        if tokens.is_empty() {
            return Err(CapabilityParseError::new(
                CapabilityParseErrorKind::InvalidTokenStream,
                "capability parser requires a final EOF token",
                None,
            ));
        }

        let mut eof_index = None;
        for (index, token) in tokens.iter().enumerate() {
            if token.kind() == TokenKind::Eof {
                if eof_index.is_some() || index + 1 != tokens.len() {
                    return Err(CapabilityParseError::new(
                        CapabilityParseErrorKind::InvalidTokenStream,
                        "token stream must contain exactly one EOF token at the end",
                        Some(*token.span()),
                    ));
                }
                eof_index = Some(index);
            }
        }

        let eof_index = eof_index.ok_or_else(|| {
            CapabilityParseError::new(
                CapabilityParseErrorKind::InvalidTokenStream,
                "token stream is missing its final EOF token",
                tokens.last().map(|token| *token.span()),
            )
        })?;

        Ok(Self {
            tokens,
            index: 0,
            eof_index,
            limits,
            parsed_clauses: 0,
        })
    }

    /// Return the current raw token index.
    ///
    /// Trivia tokens skipped during parsing still count toward this index.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.index
    }

    /// Return whether only trivia and EOF remain.
    #[must_use]
    pub fn is_eof(&self) -> bool {
        self.peek_significant_index(self.index) >= self.eof_index
    }

    /// Parse one capability clause, or return `None` at EOF.
    ///
    /// The parent parser must call this only at a grammar position where a
    /// capability clause is valid. Unknown introducers are errors, not
    /// silently accepted extension syntax.
    pub fn parse_next(
        &mut self,
    ) -> Result<Option<CapabilityItem>, CapabilityParseError> {
        self.skip_trivia();

        if self.index >= self.eof_index {
            return Ok(None);
        }

        let start_index = self.index;
        let first = &self.tokens[start_index];
        let introducer = first.lexeme();

        let kind = CapabilityClauseKind::from_introducer(introducer).ok_or_else(|| {
            CapabilityParseError::new(
                CapabilityParseErrorKind::UnexpectedToken,
                format!("expected a capability clause introducer, found `{introducer}`"),
                Some(*first.span()),
            )
            .with_expected(
                "capability, requires, constraint, prefer, hint, or target",
                Some(introducer.to_owned()),
            )
        })?;

        let tokens = self.collect_clause_tokens()?;

        if tokens.is_empty() {
            return Err(CapabilityParseError::new(
                CapabilityParseErrorKind::NoProgress,
                "capability parser did not consume a clause",
                Some(*first.span()),
            ));
        }

        let terminated = tokens
            .last()
            .is_some_and(|token| token.kind == TokenKind::Semicolon);

        let payload_end = if terminated {
            tokens.len().saturating_sub(1)
        } else {
            tokens.len()
        };

        let payload = tokens
            .iter()
            .skip(1)
            .take(payload_end.saturating_sub(1))
            .cloned()
            .collect::<Vec<_>>();

        if payload.is_empty() && !self.limits.allow_empty_payload {
            return Err(CapabilityParseError::new(
                CapabilityParseErrorKind::EmptyPayload,
                format!("capability clause `{introducer}` requires a payload"),
                Some(*first.span()),
            ));
        }

        let span = Self::compose_span(&tokens)?;

        self.parsed_clauses = self.parsed_clauses.checked_add(1).ok_or_else(|| {
            CapabilityParseError::new(
                CapabilityParseErrorKind::ClauseCountBudgetExceeded,
                "capability clause counter overflowed",
                Some(span),
            )
        })?;

        Ok(Some(CapabilityItem::Clause(CapabilityClause {
            kind,
            tokens,
            payload,
            span,
            terminated,
        })))
    }

    /// Parse all clauses in a capability-only token stream.
    ///
    /// Embedded use should normally call `parse_next`, leaving outer grammar
    /// dispatch and token ownership to the shared parser.
    pub fn parse_all(
        &mut self,
    ) -> Result<Vec<CapabilityItem>, CapabilityParseError> {
        let mut items = Vec::new();

        loop {
            self.skip_trivia();

            if self.index >= self.eof_index {
                break;
            }

            if items.len() >= self.limits.max_clauses {
                return Err(CapabilityParseError::new(
                    CapabilityParseErrorKind::ClauseCountBudgetExceeded,
                    "configured capability clause budget exceeded",
                    Some(*self.tokens[self.index].span()),
                ));
            }

            let before = self.index;
            match self.parse_next()? {
                Some(item) => items.push(item),
                None => break,
            }

            if self.index <= before {
                return Err(CapabilityParseError::new(
                    CapabilityParseErrorKind::NoProgress,
                    "capability parser failed to advance after parsing a clause",
                    self.tokens.get(before).map(|token| *token.span()),
                ));
            }
        }

        Ok(items)
    }

    // -------------------------------------------------------------------------
    // Token collection
    // -------------------------------------------------------------------------

    fn collect_clause_tokens(
        &mut self,
    ) -> Result<Vec<CapabilitySyntaxToken>, CapabilityParseError> {
        let mut collected = Vec::new();
        let mut delimiters: Vec<TokenKind> = Vec::new();
        let mut saw_top_level_composition = false;

        while self.index < self.eof_index {
            let token = &self.tokens[self.index];
            let kind = token.kind();

            if Self::is_trivia(token) {
                self.index += 1;
                continue;
            }

            if collected.len() >= self.limits.max_tokens_per_clause {
                return Err(CapabilityParseError::new(
                    CapabilityParseErrorKind::ClauseTokenBudgetExceeded,
                    "configured token budget for one capability clause exceeded",
                    Some(*token.span()),
                ));
            }

            // A top-level semicolon ends the clause. Nested semicolons remain
            // part of the payload, as required by composition syntax.
            if kind == TokenKind::Semicolon && delimiters.is_empty() {
                collected.push(CapabilitySyntaxToken::from_token(token));
                self.index += 1;
                break;
            }

            if Self::is_open_delimiter(kind) {
                if delimiters.len() >= self.limits.max_nesting_depth {
                    return Err(CapabilityParseError::new(
                        CapabilityParseErrorKind::NestingBudgetExceeded,
                        "configured capability syntax nesting budget exceeded",
                        Some(*token.span()),
                    ));
                }

                if kind == TokenKind::LeftBrace && delimiters.is_empty() {
                    saw_top_level_composition = true;
                }

                delimiters.push(kind);
                collected.push(CapabilitySyntaxToken::from_token(token));
                self.index += 1;
                continue;
            }

            if Self::is_close_delimiter(kind) {
                let expected_open = Self::matching_open(kind);

                match delimiters.pop() {
                    None => {
                        return Err(CapabilityParseError::new(
                            CapabilityParseErrorKind::UnmatchedClosingDelimiter,
                            format!(
                                "unmatched closing delimiter `{}` in capability clause",
                                token.lexeme()
                            ),
                            Some(*token.span()),
                        ));
                    }
                    Some(open) if Some(open) != expected_open => {
                        return Err(CapabilityParseError::new(
                            CapabilityParseErrorKind::MismatchedDelimiter,
                            format!(
                                "closing delimiter `{}` does not match the preceding opening delimiter",
                                token.lexeme()
                            ),
                            Some(*token.span()),
                        ));
                    }
                    Some(_) => {}
                }

                collected.push(CapabilitySyntaxToken::from_token(token));
                self.index += 1;

                // A composition's outer brace closes its body. Consume an
                // optional semicolon, then return control to the parent.
                if delimiters.is_empty() && saw_top_level_composition {
                    self.skip_trivia();

                    if self.index < self.eof_index
                        && self.tokens[self.index].kind() == TokenKind::Semicolon
                    {
                        let terminator = &self.tokens[self.index];

                        if collected.len() >= self.limits.max_tokens_per_clause {
                            return Err(CapabilityParseError::new(
                                CapabilityParseErrorKind::ClauseTokenBudgetExceeded,
                                "configured token budget for one capability clause exceeded",
                                Some(*terminator.span()),
                            ));
                        }

                        collected.push(CapabilitySyntaxToken::from_token(terminator));
                        self.index += 1;
                    }

                    break;
                }

                continue;
            }

            collected.push(CapabilitySyntaxToken::from_token(token));
            self.index += 1;
        }

        if !delimiters.is_empty() {
            return Err(CapabilityParseError::new(
                CapabilityParseErrorKind::UnclosedDelimiter,
                "capability clause ended before its delimiters were balanced",
                collected.last().map(|token| token.span),
            ));
        }

        Ok(collected)
    }

    fn skip_trivia(&mut self) {
        while self.index < self.eof_index && Self::is_trivia(&self.tokens[self.index]) {
            self.index += 1;
        }
    }

    fn peek_significant_index(&self, mut index: usize) -> usize {
        while index < self.eof_index && Self::is_trivia(&self.tokens[index]) {
            index += 1;
        }
        index
    }

    fn is_trivia(token: &Token<'_>) -> bool {
        token.is_trivia_channel()
            || matches!(
                token.kind(),
                TokenKind::Whitespace
                    | TokenKind::Newline
                    | TokenKind::LineComment
                    | TokenKind::BlockComment
            )
    }

    fn is_open_delimiter(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::LeftParen | TokenKind::LeftBracket | TokenKind::LeftBrace
        )
    }

    fn is_close_delimiter(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace
        )
    }

    fn matching_open(close: TokenKind) -> Option<TokenKind> {
        match close {
            TokenKind::RightParen => Some(TokenKind::LeftParen),
            TokenKind::RightBracket => Some(TokenKind::LeftBracket),
            TokenKind::RightBrace => Some(TokenKind::LeftBrace),
            _ => None,
        }
    }

    fn compose_span(
        tokens: &[CapabilitySyntaxToken],
    ) -> Result<Span, CapabilityParseError> {
        let first = tokens.first().ok_or_else(|| {
            CapabilityParseError::new(
                CapabilityParseErrorKind::InvalidSpan,
                "cannot compose a capability span from an empty token sequence",
                None,
            )
        })?;

        let last = tokens.last().ok_or_else(|| {
            CapabilityParseError::new(
                CapabilityParseErrorKind::InvalidSpan,
                "cannot compose a capability span from an empty token sequence",
                Some(first.span),
            )
        })?;

        Span::try_new(first.span.start(), last.span.end()).map_err(|error| {
            CapabilityParseError::new(
                CapabilityParseErrorKind::InvalidSpan,
                format!("cannot compose capability source span: {error}"),
                Some(first.span),
            )
        })
    }
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        CapabilityClauseKind, CapabilityItem, CapabilityParseErrorKind,
        CapabilityParser, CapabilityParserLimits,
    };
    use crate::frontend::lexer::span::Span;
    use crate::frontend::lexer::token::Token;
    use crate::frontend::lexer::token_kind::TokenKind;

    fn tokens(source: &str) -> Vec<Token<'_>> {
        // A compact test token stream for parser contract tests. Production
        // conformance tests should tokenize fixtures through the real scanner.
        let mut result = Vec::new();
        let mut byte = 0usize;

        for part in source.split_whitespace() {
            let start = byte;
            let end = start + part.len();
            let kind = match part {
                "capability" => TokenKind::Identifier,
                "requires" | "require" | "constraint" | "constrain" | "prefer"
                | "hint" | "target" | "availability" | "property" => TokenKind::Identifier,
                "(" => TokenKind::LeftParen,
                ")" => TokenKind::RightParen,
                "[" => TokenKind::LeftBracket,
                "]" => TokenKind::RightBracket,
                "{" => TokenKind::LeftBrace,
                "}" => TokenKind::RightBrace,
                ";" => TokenKind::Semicolon,
                "," => TokenKind::Comma,
                _ => TokenKind::Identifier,
            };

            let span = Span::default();
            result.push(Token::borrowed(kind, &source[start..end], span));
            byte = end + 1;
        }

        result.push(Token::synthetic(TokenKind::Eof, Span::default()));
        result
    }

    #[test]
    fn classifies_open_world_capability_names_without_catalogues() {
        let input = tokens("capability quantum::measurement ;");
        let mut parser =
            CapabilityParser::new(&input, CapabilityParserLimits::default()).expect("valid input");

        let item = parser.parse_next().expect("parse succeeds").expect("one clause");
        let CapabilityItem::Clause(clause) = item;

        assert_eq!(clause.kind, CapabilityClauseKind::Assertion);
        assert!(clause.compact_payload().contains("quantum::measurement"));
    }

    #[test]
    fn parses_requirements_and_preferences() {
        let input = tokens(
            "requires capability future::architecture::feature ; \
             prefer capability tensor::compute ;",
        );
        let mut parser =
            CapabilityParser::new(&input, CapabilityParserLimits::default()).expect("valid input");

        let first = parser.parse_next().expect("parse succeeds").expect("first clause");
        let second = parser.parse_next().expect("parse succeeds").expect("second clause");

        let CapabilityItem::Clause(first) = first;
        let CapabilityItem::Clause(second) = second;

        assert_eq!(first.kind, CapabilityClauseKind::Requirement);
        assert_eq!(second.kind, CapabilityClauseKind::Preference);
    }

    #[test]
    fn composition_keeps_nested_semicolons() {
        let input = tokens("capability { quantum::measurement ; tensor::compute ; } ;");
        let mut parser =
            CapabilityParser::new(&input, CapabilityParserLimits::default()).expect("valid input");

        let item = parser.parse_next().expect("parse succeeds").expect("one clause");
        let CapabilityItem::Clause(clause) = item;

        assert_eq!(clause.kind, CapabilityClauseKind::Assertion);
        assert!(clause.tokens.iter().any(|token| token.kind == TokenKind::LeftBrace));
        assert!(clause.tokens.iter().any(|token| token.kind == TokenKind::RightBrace));
        assert!(parser.is_eof());
    }

    #[test]
    fn rejects_missing_payload() {
        let input = tokens("capability ;");
        let mut parser =
            CapabilityParser::new(&input, CapabilityParserLimits::default()).expect("valid stream");

        let error = parser.parse_next().expect_err("empty payload must fail");
        assert_eq!(error.kind, CapabilityParseErrorKind::EmptyPayload);
    }

    #[test]
    fn rejects_mismatched_delimiters() {
        let input = tokens("capability ( quantum::measurement ] ;");
        let mut parser =
            CapabilityParser::new(&input, CapabilityParserLimits::default()).expect("valid stream");

        let error = parser.parse_next().expect_err("mismatched delimiter must fail");
        assert_eq!(error.kind, CapabilityParseErrorKind::MismatchedDelimiter);
    }

    #[test]
    fn enforces_configured_token_budget() {
        let input = tokens("capability quantum::measurement ;");
        let limits = CapabilityParserLimits {
            max_tokens_per_clause: 1,
            ..CapabilityParserLimits::default()
        };
        let mut parser = CapabilityParser::new(&input, limits).expect("valid stream");

        let error = parser.parse_next().expect_err("token budget must fail");
        assert_eq!(error.kind, CapabilityParseErrorKind::ClauseTokenBudgetExceeded);
    }

    #[test]
    fn rejects_stream_without_eof() {
        let input = [
            Token::borrowed(TokenKind::Identifier, "capability", Span::default()),
        ];

        let error = CapabilityParser::new(&input, CapabilityParserLimits::default())
            .expect_err("EOF is required");

        assert_eq!(error.kind, CapabilityParseErrorKind::InvalidTokenStream);
    }

    #[test]
    fn repeated_parses_are_deterministic() {
        let input = tokens("requires capability quantum::measurement ;");
        let mut first =
            CapabilityParser::new(&input, CapabilityParserLimits::default()).expect("valid input");
        let mut second =
            CapabilityParser::new(&input, CapabilityParserLimits::default()).expect("valid input");

        assert_eq!(first.parse_all().expect("parse"), second.parse_all().expect("parse"));
    }
}
