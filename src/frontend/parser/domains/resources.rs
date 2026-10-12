#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Resource-domain syntax adapter for Zamani.
//!
//! Path: src/frontend/parser/domains/resources.rs
//! Rust: 1.97+
//! Edition: 2021
//!
//! # Authority
//!
//! Resource syntax is governed by the normative language specification and
//! the canonical grammar hierarchy, especially:
//!
//! - grammar/Zamani.g4
//! - grammar/resources/resources.g4
//! - grammar/resources/requirements.g4
//! - grammar/resources/constraints.g4
//! - grammar/resources/preferences.g4
//! - grammar/resources/budgets.g4
//! - grammar/resources/capabilities.g4
//! - grammar/resources/negotiation.g4
//! - grammar/hardware/resources.g4
//!
//! The extended design reference is informative where it has not been
//! promoted into the normative specification and grammar.
//!
//! # Purpose
//!
//! Recognize resource-oriented syntax and preserve it without interpreting
//! the availability, meaning, or physical realization of any resource.
//!
//! Supported clause families include requirements, constraints, preferences,
//! hints, permissions, prohibitions, resource declarations, budgets,
//! capability references, topology expressions, and extensible properties.
//!
//! This module is a loss-preserving syntax adapter. The shared parser remains
//! responsible for deciding when to call it. The canonical AST adapter and
//! semantic analysis remain responsible for subsequent integration.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - ResourceParser and ResourceParserLimits.
//! - ResourceItem, ResourceClause, ResourceClauseKind, and
//!   ResourceSyntaxToken.
//! - ResourceParseError and ResourceParseErrorKind.
//! - Resource clause boundaries and delimiter-aware token collection.
//! - Local validation of token-stream shape and parser progress.
//!
//! This module does not own:
//!
//! - Lexing, token identities, source spans, or source text.
//! - The canonical AST or its resource types.
//! - Expression precedence or general expression parsing.
//! - Capability, effect, type, or contract resolution.
//! - Resource availability, capacity, or negotiation.
//! - Hardware discovery, allocation, placement, routing, or scheduling.
//! - Backend selection, IR construction, or execution.
//!
//! # Integration contract
//!
//! - Consumes `lexer::token::Token`.
//! - Reuses `lexer::token_kind::TokenKind` and `lexer::span::Span`.
//! - The shared parser calls `ResourceParser::parse_next` at a resource
//!   grammar extension point, or `parse_all` for a resource-only token stream.
//! - `domains/mod.rs` exports this module.
//! - `ast_adapter.rs` maps recognized syntax into the canonical AST.
//! - `session.rs` must coordinate this parser's operational budgets with the
//!   shared ParserBudget. The limits here are local safeguards, not a second
//!   language-level resource model.
//! - Semantic analysis interprets resource expressions and resolves names.
//! - Resource planning checks actual target capabilities and constraints.
//!
//! # Scalability
//!
//! Resource names and values are open-world strings/tokens, not closed
//! hardware enums. Collections grow with input and available memory. This
//! module defines no maximum number of qubits, processors, devices, nodes,
//! registers, tensor dimensions, resource properties, or resource clauses.
//!
//! Configurable parsing budgets protect implementations processing large or
//! untrusted input. An unlimited budget is not a promise of infinite memory,
//! address space, stack, or execution time.
//!
//! # Parsing model
//!
//! A resource item is retained as a sequence of canonical lexer tokens.
//! Clause classification is deliberately conservative: it classifies the
//! leading keyword, but does not pretend that this adapter replaces the
//! authoritative expression, name, capability, or constraint grammar.
//!
//! Semicolons terminate a clause only at top-level delimiter depth. Braced
//! resource bodies are consumed as balanced constructs. Strings and comments
//! are already tokenized by the lexer and are not re-scanned as source text.
//!
//! Unknown resource properties remain available to extension and semantic
//! layers. Unknown syntax is not silently converted into a known operation.

use core::fmt;
use std::error::Error;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

/// Operational safeguards for parsing resource-domain syntax.
///
/// These limits protect the parser implementation. They do not limit the
/// number or capacity of resources that a Zamani program may describe.
///
/// The embedding parser should reconcile these settings with its shared
/// parser configuration and budget accounting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceParserLimits {
    /// Maximum number of tokens consumed by a single resource item.
    ///
    /// `usize::MAX` means no explicit per-item limit.
    pub max_tokens_per_item: usize,

    /// Maximum nesting depth of parentheses, brackets, and braces.
    ///
    /// `usize::MAX` means no explicit nesting limit. This adapter scans
    /// delimiters iteratively, so the configured limit does not depend on
    /// Rust call-stack depth.
    pub max_nesting_depth: usize,

    /// Maximum number of items returned by one `parse_all` operation.
    ///
    /// `usize::MAX` means no explicit item-count limit.
    pub max_items: usize,

    /// Whether an empty clause payload is accepted.
    ///
    /// Resource declarations and empty resource bodies may be valid according
    /// to their particular grammar production. This flag is not a substitute
    /// for production-specific validation.
    pub allow_empty_payload: bool,
}

impl Default for ResourceParserLimits {
    fn default() -> Self {
        Self {
            max_tokens_per_item: usize::MAX,
            max_nesting_depth: usize::MAX,
            max_items: usize::MAX,
            allow_empty_payload: false,
        }
    }
}

// -----------------------------------------------------------------------------
// Loss-preserving syntax
// -----------------------------------------------------------------------------

/// A token snapshot used by the resource syntax representation.
///
/// This is not a second lexer token definition. It retains the canonical
/// token's kind, spelling, and source span after the original token stream
/// is no longer borrowed by downstream processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceSyntaxToken {
    /// Canonical lexer token identity.
    pub kind: TokenKind,

    /// Exact token spelling, without normalization.
    pub spelling: String,

    /// Canonical lexer source span.
    pub span: Span,
}

impl ResourceSyntaxToken {
    fn from_token(token: &Token<'_>) -> Self {
        Self {
            kind: token.kind(),
            spelling: token.lexeme().to_owned(),
            span: *token.span(),
        }
    }
}

/// Resource syntax recognized by this adapter.
///
/// This is an intermediate syntax representation, not a replacement for the
/// canonical AST in `frontend::ast::node::resources`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceItem {
    /// A source-level resource clause.
    Clause(ResourceClause),
}

/// Classification of a resource clause by its introducer.
///
/// This enum is intentionally limited to syntax categories. Resource names,
/// property names, capability names, and topology identifiers are retained
/// as source tokens rather than added as enum variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceClauseKind {
    /// `requires ...`
    Requirement,

    /// `constrain ...` or `constraint ...`
    Constraint,

    /// `prefer ...`
    Preference,

    /// `hint ...`
    Hint,

    /// `allow ...`
    Allow,

    /// `forbid ...`
    Forbid,

    /// `resource ...`
    Declaration,

    /// `budget ...`
    Budget,

    /// `capability ...`
    Capability,

    /// `topology ...`
    Topology,

    /// `reserve ...`
    Reservation,

    /// `release ...`
    Release,

    /// `negotiate ...`
    Negotiation,

    /// `target ...`
    Target,

    /// `capacity ...`
    Capacity,

    /// `availability ...`
    Availability,

    /// `scale ...` or `scalability ...`
    Scalability,

    /// A grammar extension introduced by an explicitly selected extension.
    ///
    /// This category is only a syntactic fallback for a caller that has
    /// already selected this parser at an extension point. It must not be
    /// used to make arbitrary unknown top-level syntax valid.
    Extension,
}

impl ResourceClauseKind {
    /// Classify a source introducer.
    ///
    /// Matching is ASCII case-insensitive for compatibility with dialects
    /// that classify these words without preserving case as a distinction.
    /// The original spelling is always retained in the syntax token.
    ///
    /// Returns `None` for unrecognized introducers.
    #[must_use]
    pub fn from_introducer(introducer: &str) -> Option<Self> {
        if introducer.eq_ignore_ascii_case("requires")
            || introducer.eq_ignore_ascii_case("require")
        {
            Some(Self::Requirement)
        } else if introducer.eq_ignore_ascii_case("constrain")
            || introducer.eq_ignore_ascii_case("constraint")
        {
            Some(Self::Constraint)
        } else if introducer.eq_ignore_ascii_case("prefer") {
            Some(Self::Preference)
        } else if introducer.eq_ignore_ascii_case("hint") {
            Some(Self::Hint)
        } else if introducer.eq_ignore_ascii_case("allow") {
            Some(Self::Allow)
        } else if introducer.eq_ignore_ascii_case("forbid") {
            Some(Self::Forbid)
        } else if introducer.eq_ignore_ascii_case("resource") {
            Some(Self::Declaration)
        } else if introducer.eq_ignore_ascii_case("budget") {
            Some(Self::Budget)
        } else if introducer.eq_ignore_ascii_case("capability") {
            Some(Self::Capability)
        } else if introducer.eq_ignore_ascii_case("topology") {
            Some(Self::Topology)
        } else if introducer.eq_ignore_ascii_case("reserve") {
            Some(Self::Reservation)
        } else if introducer.eq_ignore_ascii_case("release") {
            Some(Self::Release)
        } else if introducer.eq_ignore_ascii_case("negotiate") {
            Some(Self::Negotiation)
        } else if introducer.eq_ignore_ascii_case("target") {
            Some(Self::Target)
        } else if introducer.eq_ignore_ascii_case("capacity") {
            Some(Self::Capacity)
        } else if introducer.eq_ignore_ascii_case("availability") {
            Some(Self::Availability)
        } else if introducer.eq_ignore_ascii_case("scale")
            || introducer.eq_ignore_ascii_case("scalability")
        {
            Some(Self::Scalability)
        } else {
            None
        }
    }
}

/// A loss-preserving resource clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceClause {
    /// Syntactic category inferred from the first token.
    pub kind: ResourceClauseKind,

    /// All original tokens in this clause, including the introducer and
    /// a terminating semicolon when present.
    ///
    /// Keeping the introducer in the token sequence preserves exact source
    /// structure and makes future grammar refinement possible without
    /// discarding information.
    pub tokens: Vec<ResourceSyntaxToken>,

    /// Tokens following the introducer and preceding the terminator.
    ///
    /// The payload remains unclassified until the authoritative grammar and
    /// canonical AST adapter interpret it. It may contain comparison
    /// operators, qualified names, expressions, nested groups, capability
    /// calls, or topology specifications.
    pub payload: Vec<ResourceSyntaxToken>,

    /// Source span from the first token through the final retained token.
    pub span: Span,

    /// Whether a top-level semicolon was present.
    pub terminated: bool,
}

impl ResourceClause {
    /// Return the source introducer, if one was retained.
    #[must_use]
    pub fn introducer(&self) -> Option<&str> {
        self.tokens.first().map(|token| token.spelling.as_str())
    }

    /// Return the exact source spelling of the payload tokens concatenated
    /// without inserted whitespace.
    ///
    /// This is useful for diagnostics and debugging, not as a canonical
    /// pretty-printer. Original trivia may have been removed by the lexer.
    #[must_use]
    pub fn compact_payload(&self) -> String {
        let byte_capacity = self
            .payload
            .iter()
            .fold(0usize, |total, token| total.saturating_add(token.spelling.len()));

        let mut output = String::with_capacity(byte_capacity);
        for token in &self.payload {
            output.push_str(&token.spelling);
        }
        output
    }
}

// -----------------------------------------------------------------------------
// Structured errors
// -----------------------------------------------------------------------------

/// Machine-readable resource parser error classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceParseErrorKind {
    /// The token slice is empty or does not contain a valid final EOF token.
    InvalidTokenStream,

    /// The next item does not begin with a recognized resource introducer.
    UnexpectedToken,

    /// The clause has no payload where this parser's policy requires one.
    EmptyPayload,

    /// A closing delimiter has no corresponding opening delimiter.
    UnmatchedClosingDelimiter,

    /// A closing delimiter does not match the most recent opening delimiter.
    MismatchedDelimiter,

    /// The clause ended before its delimiters were balanced.
    UnclosedDelimiter,

    /// The configured delimiter-nesting budget was exceeded.
    NestingBudgetExceeded,

    /// The configured token budget for one item was exceeded.
    ItemTokenBudgetExceeded,

    /// The configured item budget for `parse_all` was exceeded.
    ItemCountBudgetExceeded,

    /// A source span could not be composed consistently.
    InvalidSpan,

    /// The parser could not make progress.
    NoProgress,
}

/// A structured syntax error.
///
/// Consumers should branch on `kind`, not on the human-readable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceParseError {
    /// Machine-readable classification.
    pub kind: ResourceParseErrorKind,

    /// Human-readable diagnostic message.
    pub message: String,

    /// Best available source span.
    pub span: Option<Span>,

    /// Expected syntax, when applicable.
    pub expected: Option<String>,

    /// Actual syntax, when applicable.
    pub found: Option<String>,
}

impl ResourceParseError {
    fn new(
        kind: ResourceParseErrorKind,
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

impl fmt::Display for ResourceParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ResourceParseError {}

// -----------------------------------------------------------------------------
// Parser
// -----------------------------------------------------------------------------

/// Incremental parser for resource-oriented syntax.
///
/// The parser borrows a canonical lexer token slice. It never re-tokenizes
/// source, resolves resource names, or modifies the token stream.
///
/// The input must contain exactly one EOF token as its final element.
/// Trivia-channel tokens may be supplied by the lexer; this parser skips
/// whitespace and comment token kinds when locating clause boundaries.
#[derive(Debug)]
pub struct ResourceParser<'tokens, 'source> {
    tokens: &'tokens [Token<'source>],
    index: usize,
    limits: ResourceParserLimits,
    eof_index: usize,
    parsed_items: usize,
}

impl<'tokens, 'source> ResourceParser<'tokens, 'source> {
    /// Validate a token stream and construct a parser at its beginning.
    pub fn new(
        tokens: &'tokens [Token<'source>],
        limits: ResourceParserLimits,
    ) -> Result<Self, ResourceParseError> {
        if tokens.is_empty() {
            return Err(ResourceParseError::new(
                ResourceParseErrorKind::InvalidTokenStream,
                "resource parser requires a token stream with a final EOF token",
                None,
            ));
        }

        let mut eof_index = None;
        for (index, token) in tokens.iter().enumerate() {
            if token.kind() == TokenKind::Eof {
                if eof_index.is_some() || index + 1 != tokens.len() {
                    return Err(ResourceParseError::new(
                        ResourceParseErrorKind::InvalidTokenStream,
                        "token stream must contain exactly one EOF token at the end",
                        Some(*token.span()),
                    ));
                }
                eof_index = Some(index);
            }
        }

        let eof_index = eof_index.ok_or_else(|| {
            ResourceParseError::new(
                ResourceParseErrorKind::InvalidTokenStream,
                "token stream is missing its final EOF token",
                tokens.last().map(|token| *token.span()),
            )
        })?;

        Ok(Self {
            tokens,
            index: 0,
            limits,
            eof_index,
            parsed_items: 0,
        })
    }

    /// Return the current raw token index, including any trivia tokens skipped
    /// while advancing between items.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.index
    }

    /// Return true when the parser has reached EOF after skipping trivia.
    #[must_use]
    pub fn is_eof(&self) -> bool {
        self.peek_significant_index(self.index) >= self.eof_index
    }

    /// Parse the next resource clause.
    ///
    /// Returns `Ok(None)` at EOF. An unrecognized introducer is an error:
    /// callers must invoke this parser only at an appropriate resource
    /// grammar extension point.
    ///
    /// A clause ends at a top-level semicolon or at the end of a balanced
    /// braced construct. Nested semicolons do not terminate the outer clause.
    pub fn parse_next(
        &mut self,
    ) -> Result<Option<ResourceItem>, ResourceParseError> {
        self.skip_trivia();

        if self.index >= self.eof_index {
            return Ok(None);
        }

        let start_index = self.index;
        let first = &self.tokens[start_index];
        let introducer = first.lexeme();

        let kind = ResourceClauseKind::from_introducer(introducer).ok_or_else(|| {
            ResourceParseError::new(
                ResourceParseErrorKind::UnexpectedToken,
                format!(
                    "expected a resource clause introducer, found `{introducer}`"
                ),
                Some(*first.span()),
            )
            .with_expected("resource clause introducer", Some(introducer.to_owned()))
        })?;

        let clause_tokens = self.collect_clause_tokens()?;

        if clause_tokens.is_empty() {
            return Err(ResourceParseError::new(
                ResourceParseErrorKind::NoProgress,
                "resource parser did not consume a clause",
                Some(*first.span()),
            ));
        }

        let terminated = clause_tokens
            .last()
            .is_some_and(|token| token.kind() == TokenKind::Semicolon);

        let payload_end = if terminated {
            clause_tokens.len().saturating_sub(1)
        } else {
            clause_tokens.len()
        };

        let payload = clause_tokens
            .iter()
            .skip(1)
            .take(payload_end.saturating_sub(1))
            .cloned()
            .collect::<Vec<_>>();

        if payload.is_empty() && !self.limits.allow_empty_payload {
            // Empty braced bodies remain represented as a non-empty payload
            // because the braces themselves are retained.
            return Err(ResourceParseError::new(
                ResourceParseErrorKind::EmptyPayload,
                format!(
                    "resource clause `{introducer}` requires a payload"
                ),
                Some(*first.span()),
            ));
        }

        let span = Self::compose_span(&clause_tokens)?;

        self.parsed_items = self.parsed_items.checked_add(1).ok_or_else(|| {
            ResourceParseError::new(
                ResourceParseErrorKind::ItemCountBudgetExceeded,
                "resource item counter overflowed",
                Some(span),
            )
        })?;

        Ok(Some(ResourceItem::Clause(ResourceClause {
            kind,
            tokens: clause_tokens,
            payload,
            span,
            terminated,
        })))
    }

    /// Parse all resource clauses in the token stream.
    ///
    /// This method is suitable for a resource-only stream. When embedded in
    /// a complete Zamani program, the shared parser should normally call
    /// `parse_next` and retain ownership of the outer token cursor.
    pub fn parse_all(
        &mut self,
    ) -> Result<Vec<ResourceItem>, ResourceParseError> {
        let mut items = Vec::new();

        loop {
            self.skip_trivia();

            if self.index >= self.eof_index {
                break;
            }

            if items.len() >= self.limits.max_items {
                return Err(ResourceParseError::new(
                    ResourceParseErrorKind::ItemCountBudgetExceeded,
                    "configured resource parser item budget exceeded",
                    Some(*self.tokens[self.index].span()),
                ));
            }

            let before = self.index;
            match self.parse_next()? {
                Some(item) => items.push(item),
                None => break,
            }

            if self.index <= before {
                return Err(ResourceParseError::new(
                    ResourceParseErrorKind::NoProgress,
                    "resource parser failed to advance after parsing an item",
                    self.tokens.get(before).map(|token| *token.span()),
                ));
            }
        }

        Ok(items)
    }

    // -------------------------------------------------------------------------
    // Token collection and validation
    // -------------------------------------------------------------------------

    fn collect_clause_tokens(
        &mut self,
    ) -> Result<Vec<ResourceSyntaxToken>, ResourceParseError> {
        let mut collected = Vec::new();
        let mut delimiters: Vec<TokenKind> = Vec::new();
        let mut saw_top_level_braced_body = false;

        while self.index < self.eof_index {
            let token = &self.tokens[self.index];
            let kind = token.kind();

            if Self::is_trivia(kind) {
                self.index += 1;
                continue;
            }

            if collected.len() >= self.limits.max_tokens_per_item {
                return Err(ResourceParseError::new(
                    ResourceParseErrorKind::ItemTokenBudgetExceeded,
                    "configured token budget for one resource item exceeded",
                    Some(*token.span()),
                ));
            }

            if kind == TokenKind::Semicolon && delimiters.is_empty() {
                collected.push(ResourceSyntaxToken::from_token(token));
                self.index += 1;
                break;
            }

            if Self::is_open_delimiter(kind) {
                if delimiters.len() >= self.limits.max_nesting_depth {
                    return Err(ResourceParseError::new(
                        ResourceParseErrorKind::NestingBudgetExceeded,
                        "configured resource syntax nesting budget exceeded",
                        Some(*token.span()),
                    ));
                }

                if kind == TokenKind::LeftBrace && delimiters.is_empty() {
                    saw_top_level_braced_body = true;
                }

                delimiters.push(kind);
                collected.push(ResourceSyntaxToken::from_token(token));
                self.index += 1;
                continue;
            }

            if Self::is_close_delimiter(kind) {
                let expected_open = Self::matching_open(kind);

                match delimiters.pop() {
                    None => {
                        return Err(ResourceParseError::new(
                            ResourceParseErrorKind::UnmatchedClosingDelimiter,
                            format!(
                                "unmatched closing delimiter `{}` in resource clause",
                                token.lexeme()
                            ),
                            Some(*token.span()),
                        ));
                    }
                    Some(open) if Some(open) != expected_open => {
                        return Err(ResourceParseError::new(
                            ResourceParseErrorKind::MismatchedDelimiter,
                            format!(
                                "closing delimiter `{}` does not match the preceding opening delimiter",
                                token.lexeme()
                            ),
                            Some(*token.span()),
                        ));
                    }
                    Some(_) => {}
                }

                collected.push(ResourceSyntaxToken::from_token(token));
                self.index += 1;

                // A top-level braced resource body is a complete construct.
                // Consume an optional semicolon in the next iteration.
                if delimiters.is_empty() && saw_top_level_braced_body {
                    self.skip_trivia();
                    if self.index < self.eof_index
                        && self.tokens[self.index].kind() == TokenKind::Semicolon
                    {
                        let terminator = &self.tokens[self.index];
                        if collected.len() >= self.limits.max_tokens_per_item {
                            return Err(ResourceParseError::new(
                                ResourceParseErrorKind::ItemTokenBudgetExceeded,
                                "configured token budget for one resource item exceeded",
                                Some(*terminator.span()),
                            ));
                        }
                        collected.push(ResourceSyntaxToken::from_token(terminator));
                        self.index += 1;
                    }
                    break;
                }

                continue;
            }

            collected.push(ResourceSyntaxToken::from_token(token));
            self.index += 1;
        }

        if !delimiters.is_empty() {
            let span = collected.last().map(|token| token.span);
            return Err(ResourceParseError::new(
                ResourceParseErrorKind::UnclosedDelimiter,
                "resource clause ended before its delimiters were balanced",
                span,
            ));
        }

        Ok(collected)
    }

    fn skip_trivia(&mut self) {
        while self.index < self.eof_index
            && Self::is_trivia(self.tokens[self.index].kind())
        {
            self.index += 1;
        }
    }

    fn peek_significant_index(&self, mut index: usize) -> usize {
        while index < self.eof_index && Self::is_trivia(self.tokens[index].kind()) {
            index += 1;
        }
        index
    }

    fn is_trivia(kind: TokenKind) -> bool {
        matches!(
            kind,
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
        tokens: &[ResourceSyntaxToken],
    ) -> Result<Span, ResourceParseError> {
        let first = tokens.first().ok_or_else(|| {
            ResourceParseError::new(
                ResourceParseErrorKind::InvalidSpan,
                "cannot construct a resource clause span from an empty token sequence",
                None,
            )
        })?;

        let last = tokens.last().ok_or_else(|| {
            ResourceParseError::new(
                ResourceParseErrorKind::InvalidSpan,
                "cannot construct a resource clause span from an empty token sequence",
                Some(first.span),
            )
        })?;

        Span::try_new(first.span.start(), last.span.end()).map_err(|error| {
            ResourceParseError::new(
                ResourceParseErrorKind::InvalidSpan,
                format!("cannot compose resource clause source span: {error}"),
                Some(first.span),
            )
        })
    }
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------
//
// Token construction is intentionally delegated to the canonical lexer test
// helpers. Add integration tests in parser/tests/domains.rs using the project's
// scanner API so these tests exercise the same token contract as production.
// Required cases include:
// - requires qubits >= required_qubits;
// - requires memory >= required_memory;
// - requires capability("quantum.measurement");
// - requires topology(required_topology);
// - nested groups containing semicolons;
// - resource declarations with braced bodies;
// - mismatched, unmatched, and unclosed delimiters;
// - EOF and trivia handling;
// - each configurable budget boundary;
// - unknown resource property names preserved exactly;
// - deterministic output for repeated parses.