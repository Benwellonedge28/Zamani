
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! AI-domain syntax adapter for the Zamani frontend.
//!
//! Path: src/frontend/parser/domains/ai.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Architectural authority
//!
//! The syntax and integration contracts are defined by:
//! - `grammar/DESIGN.md`
//! - `grammar/spec/ai.md`
//! - `grammar/ai/ai.g4`
//! - `grammar/ai/models.g4`
//! - the relevant canonical AI leaf grammars
//! - the canonical frontend lexer and AST contracts
//!
//! # Ownership
//!
//! This module owns:
//! - recognition of registered annotation-led AI constructs;
//! - a loss-preserving intermediate syntax representation;
//! - balanced-delimiter tracking for construct boundaries;
//! - local parser safety budgets;
//! - deterministic construct registration;
//! - structured syntax errors;
//! - incremental parsing of one construct at a time.
//!
//! This module does not own:
//! - lexical scanning, token identities, or source-span definitions;
//! - the canonical frontend AST;
//! - canonical expression, statement, or type grammar;
//! - name resolution, inference, or type checking;
//! - model or tensor execution;
//! - learning algorithms or reasoning engines;
//! - effect checking, authorization, or policy enforcement;
//! - resource discovery, allocation, or capability negotiation;
//! - classical IR, AI-specific IR, or quantum IR;
//! - scheduling, lowering, hardware realization, or runtime execution.
//!
//! # Integration
//!
//! The shared parser should invoke `AiDomainParser::parse_next()` at an
//! approved AI grammar extension point. The returned syntax representation
//! must be mapped to the canonical frontend AST by the shared AST adapter.
//!
//! This module intentionally does not assume constructors for AST types that
//! have not been verified. It consumes the canonical lexer token stream and
//! retains token kinds, spellings, and spans.
//!
//! # Scalability
//!
//! No hardware-capacity ceiling is defined here. Model count, tensor
//! dimensions, agent count, parameter count, dataset size, and the number of
//! constructs are not language-level limits.
//!
//! Parser budgets protect implementation resources. They are configurable,
//! and exhaustion produces a structured error instead of silent truncation.
//!
//! A finite process is necessarily constrained by available memory, address
//! space, execution time, and configured safety budgets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

/// Safety budgets for one AI-domain parser instance.
///
/// These values constrain parser work, not the computational capacity
/// expressed by a Zamani program.
///
/// The embedding parser should coordinate these budgets with its shared
/// parser configuration and resource-accounting facilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiParserLimits {
    /// Maximum nested delimiter depth within one construct.
    ///
    /// This is a safety budget, not a limit on model depth, tensor rank,
    /// neural-network depth, or computational graph size.
    pub max_nesting_depth: usize,

    /// Maximum number of token positions consumed by one construct.
    ///
    /// `usize::MAX` disables this local count limit. Shared parser budgets
    /// may still apply.
    pub max_tokens_per_construct: usize,

    /// Whether an unregistered annotation can be parsed explicitly.
    ///
    /// This setting does not cause `parse_next()` to claim every annotation
    /// in a mixed-domain program.
    pub allow_opaque_annotations: bool,
}

impl Default for AiParserLimits {
    fn default() -> Self {
        Self {
            max_nesting_depth: 512,
            max_tokens_per_construct: usize::MAX,
            allow_opaque_annotations: false,
        }
    }
}

// -----------------------------------------------------------------------------
// Open-world construct registry
// -----------------------------------------------------------------------------

/// Names recognized by the AI-domain dispatcher.
///
/// Names are stored in a sorted set so registration and lookup are
/// deterministic. The registry contains syntax dispatch names, not an
/// exhaustive list of AI algorithms, model architectures, operations,
/// hardware targets, or computational capabilities.
///
/// Applications can register additional names without editing this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiSyntaxRegistry {
    names: BTreeSet<String>,
}

impl Default for AiSyntaxRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}

impl AiSyntaxRegistry {
    /// Create a registry containing the standard annotation-led AI
    /// composition names.
    ///
    /// These names are dispatch defaults, not a closed AI vocabulary.
    /// Individual grammar owners remain authoritative for their syntax.
    #[must_use]
    pub fn with_builtins() -> Self {
        let names = [
            "model",
            "dataset",
            "tensor",
            "training",
            "train",
            "inference",
            "infer",
            "pipeline",
            "agent",
            "multi_agent",
            "cognitive",
            "reason",
            "deduce",
            "induce",
            "abduce",
            "learn",
            "adapt",
            "feedback",
            "transfer_learning",
            "reinforcement",
            "federated_learning",
            "knowledge",
            "fact",
            "assert",
            "retract",
            "query",
            "uncertainty",
            "probability",
            "probabilistic",
            "confidence",
            "evidence",
            "explain",
            "provenance",
            "policy",
            "neural",
            "symbolic",
            "neural_symbolic",
            "planning",
            "differentiate",
            "evaluate",
            "accelerator",
            "deploy",
            "goal",
            "observe",
            "act",
            "delegate",
            "coordinate",
            "message",
            "memory",
            "tool",
            "requires",
        ];

        Self {
            names: names.into_iter().map(str::to_owned).collect(),
        }
    }

    /// Create an empty registry.
    ///
    /// Use this when an embedding parser owns the complete registration
    /// policy or when testing extension behavior.
    #[must_use]
    pub fn new() -> Self {
        Self {
            names: BTreeSet::new(),
        }
    }

    /// Register a construct name.
    ///
    /// Returns `true` when a new name was inserted and `false` when it
    /// was already present. Names are case-sensitive and are not silently
    /// normalized.
    ///
    /// Empty names are rejected.
    pub fn register(
        &mut self,
        name: impl Into<String>,
    ) -> Result<bool, AiRegistryError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(AiRegistryError::EmptyName);
        }

        Ok(self.names.insert(name))
    }

    /// Test whether a name is registered.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.names.contains(name)
    }

    /// Iterate through registered names in deterministic order.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.names.iter().map(String::as_str)
    }

    /// Return the number of registered names.
    #[must_use]
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Return whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

/// An invalid AI syntax registry operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiRegistryError {
    EmptyName,
}

impl fmt::Display for AiRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => f.write_str(
                "an AI syntax registration name must not be empty",
            ),
        }
    }
}

impl Error for AiRegistryError {}

// -----------------------------------------------------------------------------
// Loss-preserving syntax representation
// -----------------------------------------------------------------------------

/// A snapshot of a canonical lexer token.
///
/// This is not another lexer token type. It retains the token identity,
/// exact spelling, and original source span for AST adaptation and
/// diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiSyntaxToken {
    pub kind: TokenKind,
    pub spelling: String,
    pub span: Span,
}

impl AiSyntaxToken {
    fn from_token(token: &Token<'_>) -> Self {
        Self {
            kind: token.kind(),
            spelling: token.lexeme().to_owned(),
            span: *token.span(),
        }
    }
}

/// The broad syntactic form of an AI construct.
///
/// `Annotation` retains the complete source-level payload. It does not
/// assert that the construct is semantically valid or executable.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiConstructForm {
    /// An annotation introduced by a separate `@` token.
    AtAnnotation,

    /// An annotation already represented by a canonical lexer token.
    ///
    /// The parser does not split or reinterpret its original spelling.
    LexicalAnnotation,

    /// A construct parsed explicitly through the opaque extension entry
    /// point rather than claimed by the registered dispatcher.
    OpaqueAnnotation,
}

/// A loss-preserving AI-domain syntax construct.
///
/// `name` is the annotation's dispatch name. `tokens` retains the original
/// token sequence, including the annotation introducer and any payload
/// consumed by this adapter.
///
/// `body` is populated when the construct contains a top-level braced body.
/// The outer braces are excluded from `body`, but remain in `tokens`.
///
/// Expressions, type expressions, annotations, attributes, nested constructs,
/// and other grammar details remain available in `tokens`. The canonical
/// grammar and AST adapter own their interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiConstruct {
    pub name: String,
    pub form: AiConstructForm,
    pub tokens: Vec<AiSyntaxToken>,
    pub body: Option<Vec<AiSyntaxToken>>,
    pub span: Span,
}

// -----------------------------------------------------------------------------
// Structured errors
// -----------------------------------------------------------------------------

/// Machine-readable classification of AI syntax failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiParseErrorKind {
    UnexpectedToken,
    MissingAnnotationName,
    UnsupportedAnnotation,
    UnbalancedDelimiter,
    MissingDelimiter,
    NestingBudgetExceeded,
    ConstructTokenBudgetExceeded,
    InvalidTokenStream,
    NoProgress,
}

/// A structured AI parser error.
///
/// Consumers should use `kind` for programmatic handling and `span` for
/// source highlighting. Human-readable messages are not stable identifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiParseError {
    pub kind: AiParseErrorKind,
    pub message: String,
    pub span: Option<Span>,
    pub expected: Option<String>,
    pub found: Option<String>,
}

impl AiParseError {
    fn new(
        kind: AiParseErrorKind,
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

    fn expected(
        kind: AiParseErrorKind,
        expected: impl Into<String>,
        found: Option<String>,
        span: Option<Span>,
    ) -> Self {
        let expected = expected.into();
        let message = match found.as_deref() {
            Some(actual) => {
                format!("expected {expected}, found {actual}")
            }
            None => {
                format!("expected {expected}, found end of input")
            }
        };

        Self {
            kind,
            message,
            span,
            expected: Some(expected),
            found,
        }
    }
}

impl fmt::Display for AiParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for AiParseError {}

// -----------------------------------------------------------------------------
// Parser
// -----------------------------------------------------------------------------

/// Incremental AI-domain parser over canonical Zamani lexer tokens.
///
/// A parser instance owns its cursor and local safety configuration. It does
/// not mutate tokens or maintain global state.
///
/// The shared parser remains responsible for cross-domain checkpoints,
/// diagnostic aggregation, compatibility policy, and aggregate resource
/// accounting.
pub struct AiDomainParser<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    index: usize,
    limits: AiParserLimits,
    registry: AiSyntaxRegistry,
}

impl<'tokens, 'src> AiDomainParser<'tokens, 'src> {
    /// Construct a parser using the standard AI construct registry.
    #[must_use]
    pub fn new(
        tokens: &'tokens [Token<'src>],
        limits: AiParserLimits,
    ) -> Self {
        Self::with_registry(tokens, limits, AiSyntaxRegistry::default())
    }

    /// Construct a parser with an explicitly supplied registry.
    #[must_use]
    pub fn with_registry(
        tokens: &'tokens [Token<'src>],
        limits: AiParserLimits,
        registry: AiSyntaxRegistry,
    ) -> Self {
        Self {
            tokens,
            index: 0,
            limits,
            registry,
        }
    }

    /// Return the current token index.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.index
    }

    /// Return the registry used by this parser.
    #[must_use]
    pub const fn registry(&self) -> &AiSyntaxRegistry {
        &self.registry
    }

    /// Set the token index for integration with a shared parser.
    ///
    /// The position is checked against the token slice. An invalid position
    /// never changes the existing cursor.
    pub fn set_position(
        &mut self,
        position: usize,
    ) -> Result<(), AiParseError> {
        if position > self.tokens.len() {
            return Err(AiParseError::new(
                AiParseErrorKind::InvalidTokenStream,
                "parser position lies beyond the token stream",
                None,
            ));
        }

        self.index = position;
        Ok(())
    }

    /// Parse the next registered AI construct.
    ///
    /// Returns `Ok(None)` when the next significant token does not begin a
    /// registered AI construct. This lets the shared parser retain ownership
    /// of ordinary annotations, statements, and other computing domains.
    ///
    /// This method consumes at most one construct and does not require EOF.
    pub fn parse_next(
        &mut self,
    ) -> Result<Option<AiConstruct>, AiParseError> {
        self.skip_trivia();

        let Some(token) = self.current() else {
            return Ok(None);
        };

        if token.kind() == TokenKind::Eof {
            return Ok(None);
        }

        let Some(name) = self.annotation_name_at(self.index) else {
            return Ok(None);
        };

        if !self.registry.contains(&name) {
            return Ok(None);
        }

        self.parse_construct(name, false).map(Some)
    }

    /// Parse one annotation explicitly, including an unregistered annotation
    /// when opaque annotations are enabled.
    ///
    /// Unlike `parse_next()`, this method is intended for use after the
    /// shared grammar has already selected the AI extension point.
    pub fn parse_annotation(
        &mut self,
    ) -> Result<AiConstruct, AiParseError> {
        self.skip_trivia();

        let Some(name) = self.annotation_name_at(self.index) else {
            return Err(self.expected_current(
                AiParseErrorKind::MissingAnnotationName,
                "an annotation followed by a construct name",
            ));
        };

        let registered = self.registry.contains(&name);

        if !registered && !self.limits.allow_opaque_annotations {
            return Err(AiParseError::new(
                AiParseErrorKind::UnsupportedAnnotation,
                format!("unregistered AI annotation `{name}`"),
                self.current_span(),
            ));
        }

        self.parse_construct(name, !registered)
    }

    /// Parse consecutive registered AI constructs until EOF or a closing
    /// brace owned by the enclosing grammar.
    ///
    /// An unrecognized construct is not silently swallowed. The shared
    /// parser can call `parse_next()` instead when it needs mixed-domain
    /// dispatch at each item boundary.
    pub fn parse_region(
        &mut self,
    ) -> Result<Vec<AiConstruct>, AiParseError> {
        let mut constructs = Vec::new();

        loop {
            self.skip_trivia();

            if self.at_eof() || self.current_is(TokenKind::RightBrace) {
                break;
            }

            let before = self.index;

            match self.parse_next()? {
                Some(construct) => constructs.push(construct),
                None => {
                    return Err(self.expected_current(
                        AiParseErrorKind::UnexpectedToken,
                        "a registered AI annotation, EOF, or closing brace",
                    ));
                }
            }

            if self.index <= before {
                return Err(AiParseError::new(
                    AiParseErrorKind::NoProgress,
                    "AI parser completed a construct without advancing",
                    self.current_span(),
                ));
            }
        }

        Ok(constructs)
    }

    // -------------------------------------------------------------------------
    // Construct parsing
    // -------------------------------------------------------------------------

    fn parse_construct(
        &mut self,
        name: String,
        opaque: bool,
    ) -> Result<AiConstruct, AiParseError> {
        self.skip_trivia();

        let start_index = self.index;
        let start_span = self
            .current_span()
            .ok_or_else(|| {
                AiParseError::new(
                    AiParseErrorKind::InvalidTokenStream,
                    "expected an annotation at the end of the token stream",
                    None,
                )
            })?;

        let first_kind = self.current().map(Token::kind);

        let form = match first_kind {
            Some(TokenKind::At) => AiConstructForm::AtAnnotation,
            Some(TokenKind::NanoAnnotation) => {
                AiConstructForm::LexicalAnnotation
            }
            _ if opaque => AiConstructForm::OpaqueAnnotation,
            _ => {
                return Err(self.expected_current(
                    AiParseErrorKind::MissingAnnotationName,
                    "`@name` or a canonical annotation token",
                ));
            }
        };

        let mut tokens = Vec::new();
        let mut body = None;

        // Consume the annotation introducer and name. A canonical
        // NanoAnnotation may already contain both in a single token.
        match form {
            AiConstructForm::AtAnnotation => {
                tokens.push(self.bump_snapshot().ok_or_else(|| {
                    self.invalid_stream("missing annotation introducer")
                })?);

                self.skip_trivia();

                if !self.current().is_some_and(|token| {
                    token.kind() == TokenKind::Identifier
                }) {
                    return Err(self.expected_current(
                        AiParseErrorKind::MissingAnnotationName,
                        "an identifier after `@`",
                    ));
                }

                let actual_name = self
                    .current()
                    .map(|token| token.lexeme().to_owned())
                    .unwrap_or_default();

                if actual_name != name {
                    return Err(AiParseError::new(
                        AiParseErrorKind::InvalidTokenStream,
                        "annotation name changed during parsing",
                        self.current_span(),
                    ));
                }

                tokens.push(self.bump_snapshot().ok_or_else(|| {
                    self.invalid_stream("missing annotation name")
                })?);
            }
            AiConstructForm::LexicalAnnotation
            | AiConstructForm::OpaqueAnnotation => {
                tokens.push(self.bump_snapshot().ok_or_else(|| {
                    self.invalid_stream("missing annotation token")
                })?);
            }
        }

        let mut delimiters: Vec<(TokenKind, Span)> = Vec::new();
        let mut body_start = None;
        let mut body_end = None;

        loop {
            self.skip_trivia();

            let Some(token) = self.current() else {
                if !delimiters.is_empty() {
                    return Err(self.unclosed_delimiter(&delimiters));
                }

                break;
            };

            let kind = token.kind();

            if kind == TokenKind::Eof {
                if !delimiters.is_empty() {
                    return Err(self.unclosed_delimiter(&delimiters));
                }

                break;
            }

            if self.index.saturating_sub(start_index)
                >= self.limits.max_tokens_per_construct
            {
                return Err(AiParseError::new(
                    AiParseErrorKind::ConstructTokenBudgetExceeded,
                    "AI construct exceeded its configured token budget",
                    self.current_span(),
                ));
            }

            // A top-level semicolon terminates an annotation-led construct.
            if delimiters.is_empty() && kind == TokenKind::Semicolon {
                tokens.push(self.bump_snapshot().ok_or_else(|| {
                    self.invalid_stream("missing statement terminator")
                })?);
                break;
            }

            // Opening delimiters are tracked explicitly. This preserves
            // arbitrary nested expression and body syntax without parsing
            // the same expression grammar a second time.
            if Self::is_open_delimiter(kind) {
                if delimiters.len() >= self.limits.max_nesting_depth {
                    return Err(AiParseError::new(
                        AiParseErrorKind::NestingBudgetExceeded,
                        "AI construct exceeded its nesting-depth budget",
                        self.current_span(),
                    ));
                }

                let span = *token.span();

                if kind == TokenKind::LeftBrace
                    && delimiters.is_empty()
                    && body_start.is_none()
                {
                    body_start = Some(tokens.len());
                }

                delimiters.push((kind, span));

                tokens.push(self.bump_snapshot().ok_or_else(|| {
                    self.invalid_stream("missing opening delimiter")
                })?);

                continue;
            }

            if Self::is_close_delimiter(kind) {
                let Some((open_kind, _open_span)) = delimiters.last().copied()
                else {
                    return Err(AiParseError::new(
                        AiParseErrorKind::UnbalancedDelimiter,
                        "unexpected closing delimiter in AI construct",
                        self.current_span(),
                    ));
                };

                if !Self::delimiters_match(open_kind, kind) {
                    return Err(AiParseError::expected(
                        AiParseErrorKind::UnbalancedDelimiter,
                        Self::closing_name(open_kind),
                        Some(token.lexeme().to_owned()),
                        self.current_span(),
                    ));
                }

                let closing_snapshot =
                    self.bump_snapshot().ok_or_else(|| {
                        self.invalid_stream("missing closing delimiter")
                    })?;

                delimiters.pop();
                tokens.push(closing_snapshot);

                // A complete top-level braced body ends this construct.
                // Nested braces inside expressions do not end it early.
                if kind == TokenKind::RightBrace
                    && delimiters.is_empty()
                    && body_start.is_some()
                {
                    body_end = Some(tokens.len() - 1);

                    // A semicolon after the body is permitted and belongs
                    // to this construct.
                    self.skip_trivia();

                    if self.current_is(TokenKind::Semicolon) {
                        tokens.push(self.bump_snapshot().ok_or_else(|| {
                            self.invalid_stream("missing trailing semicolon")
                        })?);
                    }

                    break;
                }

                continue;
            }

            tokens.push(self.bump_snapshot().ok_or_else(|| {
                self.invalid_stream("token disappeared during parsing")
            })?);
        }

        if !delimiters.is_empty() {
            return Err(self.unclosed_delimiter(&delimiters));
        }

        if tokens.len() < 2 {
            return Err(AiParseError::new(
                AiParseErrorKind::MissingAnnotationName,
                "AI construct does not contain its annotation and name",
                Some(start_span),
            ));
        }

        let end_span = tokens
            .last()
            .map(|token| token.span)
            .unwrap_or(start_span);

        let span = Span::try_new(start_span.start(), end_span.end())
            .unwrap_or(start_span);

        if let (Some(start), Some(end)) = (body_start, body_end) {
            if start < end && end <= tokens.len() {
                // The braces occupy `start` and `end - 1`.
                // Keep the original tokens separately in `tokens`.
                body = Some(tokens[start + 1..end - 1].to_vec());
            }
        }

        Ok(AiConstruct {
            name,
            form,
            tokens,
            body,
            span,
        })
    }

    // -------------------------------------------------------------------------
    // Annotation-name recognition
    // -------------------------------------------------------------------------

    fn annotation_name_at(&self, index: usize) -> Option<String> {
        let token = self.tokens.get(index)?;

        match token.kind() {
            TokenKind::At => {
                let next = self.next_significant_index(index + 1)?;
                let name_token = self.tokens.get(next)?;

                if name_token.kind() != TokenKind::Identifier {
                    return None;
                }

                Some(name_token.lexeme().to_owned())
            }
            TokenKind::NanoAnnotation => {
                Self::extract_annotation_name(token.lexeme())
            }
            _ => None,
        }
    }

    fn extract_annotation_name(spelling: &str) -> Option<String> {
        let spelling = spelling.trim().strip_prefix('@')?;
        let name = spelling
            .split(|ch: char| {
                ch.is_whitespace()
                    || matches!(ch, '(' | '{' | '[' | ':' | ';')
            })
            .next()?
            .trim();

        if name.is_empty() {
            None
        } else {
            Some(name.to_owned())
        }
    }

    // -------------------------------------------------------------------------
    // Cursor and token utilities
    // -------------------------------------------------------------------------

    fn current(&self) -> Option<&Token<'src>> {
        self.tokens.get(self.index)
    }

    fn current_span(&self) -> Option<Span> {
        self.current().map(|token| *token.span())
    }

    fn current_is(&self, kind: TokenKind) -> bool {
        self.current()
            .is_some_and(|token| token.kind() == kind)
    }

    fn at_eof(&self) -> bool {
        self.current_is(TokenKind::Eof) || self.index >= self.tokens.len()
    }

    fn next_significant_index(&self, from: usize) -> Option<usize> {
        let mut index = from;

        while let Some(token) = self.tokens.get(index) {
            if !token.is_trivia_channel() {
                return Some(index);
            }

            index = index.checked_add(1)?;
        }

        None
    }

    fn skip_trivia(&mut self) {
        while self
            .current()
            .is_some_and(Token::is_trivia_channel)
        {
            self.index = match self.index.checked_add(1) {
                Some(index) => index,
                None => self.tokens.len(),
            };
        }
    }

    fn bump_snapshot(&mut self) -> Option<AiSyntaxToken> {
        let token = self.current()?;
        let snapshot = AiSyntaxToken::from_token(token);

        self.index = self.index.checked_add(1)?;

        Some(snapshot)
    }

    fn expected_current(
        &self,
        kind: AiParseErrorKind,
        expected: &str,
    ) -> AiParseError {
        let found = self
            .current()
            .map(|token| token.lexeme().to_owned());

        AiParseError::expected(
            kind,
            expected,
            found,
            self.current_span(),
        )
    }

    fn invalid_stream(&self, message: &str) -> AiParseError {
        AiParseError::new(
            AiParseErrorKind::InvalidTokenStream,
            message,
            self.current_span(),
        )
    }

    fn unclosed_delimiter(
        &self,
        delimiters: &[(TokenKind, Span)],
    ) -> AiParseError {
        let expected = delimiters
            .last()
            .map(|(kind, _)| Self::closing_name(*kind))
            .unwrap_or("a closing delimiter");

        AiParseError::expected(
            AiParseErrorKind::MissingDelimiter,
            expected,
            None,
            delimiters.last().map(|(_, span)| *span),
        )
    }

    // -------------------------------------------------------------------------
    // Delimiter utilities
    // -------------------------------------------------------------------------

    fn is_open_delimiter(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::LeftParen
                | TokenKind::LeftBracket
                | TokenKind::LeftBrace
        )
    }

    fn is_close_delimiter(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::RightParen
                | TokenKind::RightBracket
                | TokenKind::RightBrace
        )
    }

    fn delimiters_match(open: TokenKind, close: TokenKind) -> bool {
        matches!(
            (open, close),
            (TokenKind::LeftParen, TokenKind::RightParen)
                | (TokenKind::LeftBracket, TokenKind::RightBracket)
                | (TokenKind::LeftBrace, TokenKind::RightBrace)
        )
    }

    fn closing_name(kind: TokenKind) -> &'static str {
        match kind {
            TokenKind::LeftParen => "`)`",
            TokenKind::LeftBracket => "`]`",
            TokenKind::LeftBrace => "`}`",
            _ => "a closing delimiter",
        }
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_registry_contains_common_ai_boundaries() {
        let registry = AiSyntaxRegistry::default();

        assert!(registry.contains("model"));
        assert!(registry.contains("dataset"));
        assert!(registry.contains("agent"));
        assert!(registry.contains("learn"));
        assert!(registry.contains("reason"));
    }

    #[test]
    fn registry_accepts_future_construct_names() {
        let mut registry = AiSyntaxRegistry::new();

        assert_eq!(
            registry.register("future_computing_domain"),
            Ok(true)
        );
        assert_eq!(
            registry.register("future_computing_domain"),
            Ok(false)
        );

        assert!(registry.contains("future_computing_domain"));
    }

    #[test]
    fn registry_rejects_empty_names() {
        let mut registry = AiSyntaxRegistry::new();

        assert_eq!(
            registry.register("  "),
            Err(AiRegistryError::EmptyName)
        );
    }

    #[test]
    fn registry_iteration_is_deterministic() {
        let mut registry = AiSyntaxRegistry::new();

        registry.register("zeta").expect("valid name");
        registry.register("alpha").expect("valid name");
        registry.register("mu").expect("valid name");

        let names: Vec<_> = registry.names().collect();

        assert_eq!(names, vec!["alpha", "mu", "zeta"]);
    }

    #[test]
    fn annotation_name_extraction_preserves_identifier_spelling() {
        assert_eq!(
            AiDomainParser::<'static, 'static>::extract_annotation_name(
                "@model Example {"
            ),
            Some("model".to_owned())
        );

        assert_eq!(
            AiDomainParser::<'static, 'static>::extract_annotation_name(
                "@future_domain(value)"
            ),
            Some("future_domain".to_owned())
        );
    }

    #[test]
    fn annotation_name_extraction_rejects_non_annotation_text() {
        assert_eq!(
            AiDomainParser::<'static, 'static>::extract_annotation_name(
                "model Example"
            ),
            None
        );

        assert_eq!(
            AiDomainParser::<'static, 'static>::extract_annotation_name("@"),
            None
        );
    }

    #[test]
    fn delimiters_are_matched_by_kind() {
        assert!(AiDomainParser::delimiters_match(
            TokenKind::LeftParen,
            TokenKind::RightParen
        ));

        assert!(AiDomainParser::delimiters_match(
            TokenKind::LeftBracket,
            TokenKind::RightBracket
        ));

        assert!(AiDomainParser::delimiters_match(
            TokenKind::LeftBrace,
            TokenKind::RightBrace
        ));

        assert!(!AiDomainParser::delimiters_match(
            TokenKind::LeftParen,
            TokenKind::RightBrace
        ));
    }

    #[test]
    fn registry_size_has_no_domain_capacity_assumption() {
        let mut registry = AiSyntaxRegistry::new();

        for index in 0..1_000 {
            registry
                .register(format!("extension_{index}"))
                .expect("generated names are non-empty");
        }

        assert_eq!(registry.len(), 1_000);
    }
}
