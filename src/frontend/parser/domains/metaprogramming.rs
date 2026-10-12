
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani metaprogramming-domain syntax adapter.
//!
//! Path: `src/frontend/parser/domains/metaprogramming.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Recognize metaprogramming entry points in the canonical Zamani token
//! stream and preserve each construct as loss-preserving syntax for the
//! shared parser and AST adapter.
//!
//! # Grammar authority
//!
//! The normative syntax remains owned by:
//!
//! - `grammar/metaprogramming/metaprogramming.g4`
//! - `grammar/metaprogramming/compile-time-execution.g4`
//! - `grammar/metaprogramming/generation.g4`
//! - `grammar/metaprogramming/code-generation.g4`
//! - `grammar/metaprogramming/reflection.g4`
//! - `grammar/metaprogramming/introspection.g4`
//! - `grammar/metaprogramming/quotation.g4`
//! - `grammar/metaprogramming/unquotation.g4`
//! - `grammar/metaprogramming/specialization.g4`
//! - `grammar/metaprogramming/type-level.g4`
//! - `grammar/metaprogramming/schemas.g4`
//! - `grammar/metaprogramming/syntax-tree.g4`
//! - `grammar/macros/` and canonical expression/type/declaration grammars.
//!
//! This adapter does not replace those grammars or claim to perform complete
//! grammar validation. The authoritative parser must validate the captured
//! syntax using the corresponding grammar productions.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - metaprogramming construct classification at parser extension points;
//! - loss-preserving syntax snapshots;
//! - balanced-delimiter scanning;
//! - configurable construct-token and nesting budgets;
//! - structured errors for malformed token streams;
//! - deterministic recognition of built-in and registered entry introducers.
//!
//! It does not own:
//!
//! - tokenization or keyword spelling registries;
//! - the canonical AST or a competing type system;
//! - macro expansion or hygiene;
//! - compile-time evaluation or constant folding;
//! - reflection or introspection execution;
//! - generated-source execution;
//! - semantic validation, authorization, or provenance assignment;
//! - classical IR, `quantum::ir`, HDL IR, or runtime execution;
//! - hardware discovery, allocation, routing, scheduling, or QEC.
//!
//! # POCO-REAF
//!
//! No hardware-capacity assumptions are encoded here. Construct cardinality
//! and nesting budgets are configurable parser-safety policies, not language
//! limits. Generated syntax must re-enter the canonical frontend and pass
//! ordinary validation and semantic analysis before lowering.
//!
//! # Integration
//!
//! The shared parser may call `MetaprogrammingDomainParser::parse_next()`
//! when it reaches a metaprogramming extension point. `Ok(None)` means that
//! the current significant token is not recognized by this adapter.
//!
//! The returned representation is an intermediate syntax boundary. Convert
//! it through the canonical AST adapter after the relevant AST constructors
//! and extension-node contracts have been verified.

use std::error::Error;
use std::fmt;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;

// =============================================================================
// Configuration
// =============================================================================

/// Resource budgets for scanning metaprogramming constructs.
///
/// `None` means that this adapter does not impose that particular budget.
/// A containing parser session should normally supply shared, coordinated
/// limits rather than independently resetting budgets for every sub-parser.
///
/// These limits protect compiler resources. They do not constrain the number
/// of generated declarations, types, qubits, devices, or hardware resources
/// expressible by the language.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MetaprogrammingParserLimits {
    /// Maximum significant and trivia tokens consumed by one construct.
    pub max_tokens_per_construct: Option<usize>,

    /// Maximum simultaneously open delimiters in one construct.
    pub max_nesting_depth: Option<usize>,
}

/// Policy for recognizing metaprogramming constructs.
///
/// Extension introducers are exact identifier spellings supplied by the
/// embedding parser or dialect configuration. They are not automatically
/// treated as valid syntax: the authoritative grammar or registered handler
/// must still validate them.
///
/// Built-in recognition is deliberately small and does not enumerate every
/// possible generated operation, type, domain, or vendor-specific facility.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MetaprogrammingParserConfig {
    pub limits: MetaprogrammingParserLimits,

    /// Additional explicitly registered introducer spellings.
    ///
    /// Matching is ASCII-case-insensitive. For language constructs whose
    /// spelling is case-sensitive, the authoritative grammar remains the
    /// final authority and must reject a mismatched spelling.
    pub extension_introducers: Vec<String>,
}

// =============================================================================
// Loss-preserving syntax
// =============================================================================

/// Owned snapshot of a canonical lexer token.
///
/// This is an intermediate syntax representation, not a second lexer token
/// model. It preserves token identity, exact spelling, and the canonical span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaprogrammingSyntaxToken {
    pub kind: TokenKind,
    pub spelling: String,
    pub span: Span,
    pub channel: TokenChannel,
}

impl MetaprogrammingSyntaxToken {
    fn from_token(token: &Token<'_>) -> Self {
        Self {
            kind: token.kind(),
            spelling: token.lexeme().to_owned(),
            span: *token.span(),
            channel: token.channel(),
        }
    }
}

/// Broad classification for a recognized metaprogramming entry point.
///
/// Classification is intentionally not semantic validation. For example,
/// recognizing a specialization request does not prove that its target
/// exists or can be specialized.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MetaprogrammingConstructKind {
    CompileTimeExecution,
    SourceGeneration,
    CodeGeneration,
    Reflection,
    Introspection,
    Quotation,
    Unquotation,
    Specialization,
    TypeLevel,
    Schema,
    Capability,
    SyntaxTree,
    Extension(String),
}

/// A metaprogramming construct preserved from the token stream.
///
/// `tokens` includes the introducer and all scanned construct tokens,
/// including a top-level semicolon when it terminates the construct.
/// Trivia encountered inside the scanned range is preserved.
///
/// `token_range` is a half-open range into the original token slice. It
/// identifies the exact source tokens consumed by this adapter and is useful
/// when the containing parser retains the original token stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaprogrammingConstruct {
    pub kind: MetaprogrammingConstructKind,
    pub tokens: Vec<MetaprogrammingSyntaxToken>,
    pub token_range: std::ops::Range<usize>,
    pub span: Option<Span>,
}

/// A recognized but not yet semantically interpreted metaprogramming region.
///
/// The shared AST adapter is responsible for mapping this representation to
/// the canonical AST without discarding information.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MetaprogrammingSyntaxRegion {
    pub constructs: Vec<MetaprogrammingConstruct>,
}

// =============================================================================
// Structured errors
// =============================================================================

/// Machine-readable failures produced by this adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MetaprogrammingParseErrorKind {
    InvalidTokenStream,
    InvalidToken,
    UnexpectedClosingDelimiter,
    MismatchedClosingDelimiter,
    UnterminatedDelimiter,
    TokenBudgetExceeded,
    NestingBudgetExceeded,
    InvalidCursorPosition,
}

/// Structured parser error.
///
/// Use `kind` for programmatic handling. The message is intended for people
/// and may evolve without changing the classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaprogrammingParseError {
    pub kind: MetaprogrammingParseErrorKind,
    pub message: String,
    pub span: Option<Span>,
    pub token_index: Option<usize>,
}

impl MetaprogrammingParseError {
    fn new(
        kind: MetaprogrammingParseErrorKind,
        message: impl Into<String>,
        span: Option<Span>,
        token_index: Option<usize>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
            token_index,
        }
    }
}

impl fmt::Display for MetaprogrammingParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for MetaprogrammingParseError {}

// =============================================================================
// Parser
// =============================================================================

/// Metaprogramming syntax adapter over the canonical Zamani token stream.
///
/// The adapter borrows tokens and does not re-tokenize source. It uses an
/// iterative delimiter stack rather than recursive scanning, so deeply nested
/// syntax does not consume the Rust call stack.
///
/// One instance should be owned by one parser invocation. It has no global
/// mutable state and can be discarded after its containing parser has
/// converted the result or reported an error.
pub struct MetaprogrammingDomainParser<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    index: usize,
    config: MetaprogrammingParserConfig,
}

impl<'tokens, 'src> MetaprogrammingDomainParser<'tokens, 'src> {
    /// Create an adapter over the canonical lexer token stream.
    #[must_use]
    pub fn new(
        tokens: &'tokens [Token<'src>],
        config: MetaprogrammingParserConfig,
    ) -> Self {
        Self {
            tokens,
            index: 0,
            config,
        }
    }

    /// Current token-slice position.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.index
    }

    /// Change the cursor position after checking the token-slice boundary.
    ///
    /// This is intended for integration with the shared parser's checkpoint
    /// and dispatch machinery. It does not validate that the position is a
    /// grammar boundary.
    pub fn set_position(
        &mut self,
        position: usize,
    ) -> Result<(), MetaprogrammingParseError> {
        if position > self.tokens.len() {
            return Err(MetaprogrammingParseError::new(
                MetaprogrammingParseErrorKind::InvalidCursorPosition,
                "metaprogramming parser position is beyond the token stream",
                None,
                Some(position),
            ));
        }

        self.index = position;
        Ok(())
    }

    /// Return the configuration used by this parser instance.
    #[must_use]
    pub const fn config(&self) -> &MetaprogrammingParserConfig {
        &self.config
    }

    /// Parse one recognized metaprogramming construct.
    ///
    /// Returns `Ok(None)` when the next significant token is not a recognized
    /// metaprogramming introducer. In that case, the cursor is positioned at
    /// the next significant token.
    ///
    /// This method does not require EOF because it is designed for use inside
    /// a larger parser. Syntax-specific validation remains the responsibility
    /// of the authoritative grammar.
    pub fn parse_next(
        &mut self,
    ) -> Result<Option<MetaprogrammingConstruct>, MetaprogrammingParseError> {
        self.skip_trivia();

        let Some(token) = self.current() else {
            return Ok(None);
        };

        if token.kind() == TokenKind::Eof {
            return Ok(None);
        }

        if token.kind() == TokenKind::Invalid || token.channel() == TokenChannel::Error {
            return Err(MetaprogrammingParseError::new(
                MetaprogrammingParseErrorKind::InvalidToken,
                "invalid token encountered at a metaprogramming parser boundary",
                Some(*token.span()),
                Some(self.index),
            ));
        }

        let Some(kind) = self.classify_current() else {
            return Ok(None);
        };

        self.scan_construct(kind).map(Some)
    }

    /// Parse consecutive recognized constructs.
    ///
    /// Parsing stops at the first significant token that is not recognized by
    /// this adapter, or at EOF. The containing parser remains responsible for
    /// consuming the rest of the source.
    pub fn parse_region(
        &mut self,
    ) -> Result<MetaprogrammingSyntaxRegion, MetaprogrammingParseError> {
        let mut region = MetaprogrammingSyntaxRegion::default();

        loop {
            self.skip_trivia();

            if self.at_eof() {
                break;
            }

            let before = self.index;

            match self.parse_next()? {
                Some(construct) => region.constructs.push(construct),
                None => break,
            }

            if self.index <= before {
                return Err(MetaprogrammingParseError::new(
                    MetaprogrammingParseErrorKind::InvalidTokenStream,
                    "metaprogramming parser did not advance after recognizing a construct",
                    self.current_span(),
                    Some(self.index),
                ));
            }
        }

        Ok(region)
    }

    // -------------------------------------------------------------------------
    // Construct recognition
    // -------------------------------------------------------------------------

    fn classify_current(&self) -> Option<MetaprogrammingConstructKind> {
        let token = self.current()?;
        let spelling = token.lexeme();

        // Compile-time execution uses the repository's existing `const`
        // vocabulary. Do not classify an ordinary constant declaration as a
        // compile-time execution block merely because it begins with `const`.
        if spelling.eq_ignore_ascii_case("const") {
            let next = self.next_significant_token(self.index.saturating_add(1));
            if next.is_some_and(|next| {
                next.lexeme() == "(" || next.lexeme() == "{"
            }) {
                return Some(MetaprogrammingConstructKind::CompileTimeExecution);
            }
        }

        if spelling.eq_ignore_ascii_case("synthesize") {
            return Some(MetaprogrammingConstructKind::SourceGeneration);
        }

        if spelling.eq_ignore_ascii_case("reflect") {
            return Some(MetaprogrammingConstructKind::Reflection);
        }

        if spelling.eq_ignore_ascii_case("introspect") {
            return Some(MetaprogrammingConstructKind::Introspection);
        }

        if spelling.eq_ignore_ascii_case("quote") {
            return Some(MetaprogrammingConstructKind::Quotation);
        }

        if spelling.eq_ignore_ascii_case("unquote")
            || spelling.eq_ignore_ascii_case("splice")
        {
            return Some(MetaprogrammingConstructKind::Unquotation);
        }

        if spelling.eq_ignore_ascii_case("specialize") {
            return Some(MetaprogrammingConstructKind::Specialization);
        }

        if spelling.eq_ignore_ascii_case("typelevel")
            || spelling.eq_ignore_ascii_case("type_level")
        {
            return Some(MetaprogrammingConstructKind::TypeLevel);
        }

        if spelling.eq_ignore_ascii_case("meta_schema")
            || spelling.eq_ignore_ascii_case("metaschema")
        {
            return Some(MetaprogrammingConstructKind::Schema);
        }

        if spelling.eq_ignore_ascii_case("metaprogramming_capability") {
            return Some(MetaprogrammingConstructKind::Capability);
        }

        if spelling.eq_ignore_ascii_case("syntax_tree")
            || spelling.eq_ignore_ascii_case("syntaxTree")
        {
            return Some(MetaprogrammingConstructKind::SyntaxTree);
        }

        if spelling.eq_ignore_ascii_case("codegen")
            || spelling.eq_ignore_ascii_case("code_generation")
        {
            return Some(MetaprogrammingConstructKind::CodeGeneration);
        }

        self.config
            .extension_introducers
            .iter()
            .find(|registered| registered.eq_ignore_ascii_case(spelling))
            .map(|registered| {
                MetaprogrammingConstructKind::Extension(registered.clone())
            })
    }

    // -------------------------------------------------------------------------
    // Loss-preserving construct scanner
    // -------------------------------------------------------------------------

    fn scan_construct(
        &mut self,
        kind: MetaprogrammingConstructKind,
    ) -> Result<MetaprogrammingConstruct, MetaprogrammingParseError> {
        let start = self.index;
        let mut output = Vec::new();
        let mut delimiters: Vec<(TokenKind, Span)> = Vec::new();
        let mut root_block_opened = false;

        loop {
            let consumed = self.index.saturating_sub(start);

            if let Some(maximum) = self.config.limits.max_tokens_per_construct {
                if consumed >= maximum {
                    // Ensure an impossible zero-token budget cannot cause the
                    // caller to retry this same introducer indefinitely.
                    if self.index == start && self.current().is_some() {
                        self.bump();
                    }

                    return Err(MetaprogrammingParseError::new(
                        MetaprogrammingParseErrorKind::TokenBudgetExceeded,
                        format!(
                            "metaprogramming construct exceeded its configured token budget ({maximum})"
                        ),
                        self.current_span().or_else(|| output.last().map(|t: &MetaprogrammingSyntaxToken| t.span)),
                        Some(self.index),
                    ));
                }
            }

            let Some(token) = self.current() else {
                if let Some((_, opening_span)) = delimiters.last() {
                    return Err(MetaprogrammingParseError::new(
                        MetaprogrammingParseErrorKind::UnterminatedDelimiter,
                        "end of token stream reached before a delimiter was closed",
                        Some(*opening_span),
                        Some(self.index),
                    ));
                }

                break;
            };

            if token.kind() == TokenKind::Eof {
                if let Some((_, opening_span)) = delimiters.last() {
                    return Err(MetaprogrammingParseError::new(
                        MetaprogrammingParseErrorKind::UnterminatedDelimiter,
                        "end of input reached before a delimiter was closed",
                        Some(*opening_span),
                        Some(self.index),
                    ));
                }

                break;
            }

            if token.kind() == TokenKind::Invalid
                || token.channel() == TokenChannel::Error
            {
                return Err(MetaprogrammingParseError::new(
                    MetaprogrammingParseErrorKind::InvalidToken,
                    "invalid token encountered inside a metaprogramming construct",
                    Some(*token.span()),
                    Some(self.index),
                ));
            }

            let token_kind = token.kind();
            let token_span = *token.span();

            if is_opening_delimiter(token_kind) {
                if token_kind == TokenKind::LeftBrace && delimiters.is_empty() {
                    root_block_opened = true;
                }

                if let Some(maximum) = self.config.limits.max_nesting_depth {
                    if delimiters.len() >= maximum {
                        self.bump();

                        return Err(MetaprogrammingParseError::new(
                            MetaprogrammingParseErrorKind::NestingBudgetExceeded,
                            format!(
                                "metaprogramming construct exceeded its configured nesting budget ({maximum})"
                            ),
                            Some(token_span),
                            Some(self.index.saturating_sub(1)),
                        ));
                    }
                }

                delimiters.push((token_kind, token_span));
                output.push(MetaprogrammingSyntaxToken::from_token(token));
                self.bump();
                continue;
            }

            if is_closing_delimiter(token_kind) {
                let Some((opening_kind, opening_span)) = delimiters.last().copied() else {
                    return Err(MetaprogrammingParseError::new(
                        MetaprogrammingParseErrorKind::UnexpectedClosingDelimiter,
                        "unexpected closing delimiter in metaprogramming construct",
                        Some(token_span),
                        Some(self.index),
                    ));
                };

                if !matches_delimiters(opening_kind, token_kind) {
                    return Err(MetaprogrammingParseError::new(
                        MetaprogrammingParseErrorKind::MismatchedClosingDelimiter,
                        format!(
                            "closing delimiter {:?} does not match opening delimiter {:?}",
                            token_kind, opening_kind
                        ),
                        Some(token_span),
                        Some(self.index),
                    ));
                }

                delimiters.pop();
                output.push(MetaprogrammingSyntaxToken::from_token(token));
                self.bump();

                // A top-level braced body is a useful incremental boundary
                // when the source form has no trailing semicolon.
                if delimiters.is_empty() && root_block_opened {
                    break;
                }

                continue;
            }

            let is_top_level_terminator =
                delimiters.is_empty() && token_kind == TokenKind::Semicolon;

            output.push(MetaprogrammingSyntaxToken::from_token(token));
            self.bump();

            if is_top_level_terminator {
                break;
            }
        }

        if output.is_empty() {
            return Err(MetaprogrammingParseError::new(
                MetaprogrammingParseErrorKind::InvalidTokenStream,
                "recognized metaprogramming introducer produced an empty construct",
                self.current_span(),
                Some(start),
            ));
        }

        let end = self.index;
        let span = span_from_tokens(&output);

        Ok(MetaprogrammingConstruct {
            kind,
            tokens: output,
            token_range: start..end,
            span,
        })
    }

    // -------------------------------------------------------------------------
    // Cursor helpers
    // -------------------------------------------------------------------------

    fn current(&self) -> Option<&Token<'src>> {
        self.tokens.get(self.index)
    }

    fn current_span(&self) -> Option<Span> {
        self.current().map(|token| *token.span())
    }

    fn at_eof(&self) -> bool {
        self.current()
            .is_none_or(|token| token.kind() == TokenKind::Eof)
    }

    fn bump(&mut self) {
        if self.index < self.tokens.len() {
            self.index += 1;
        }
    }

    fn skip_trivia(&mut self) {
        while self
            .current()
            .is_some_and(|token| token.channel() == TokenChannel::Trivia)
        {
            self.bump();
        }
    }

    fn next_significant_token(&self, mut position: usize) -> Option<&Token<'src>> {
        while let Some(token) = self.tokens.get(position) {
            if token.kind() == TokenKind::Eof {
                return None;
            }

            if token.channel() != TokenChannel::Trivia {
                return Some(token);
            }

            position = position.saturating_add(1);
        }

        None
    }
}

// =============================================================================
// Delimiter and span helpers
// =============================================================================

fn is_opening_delimiter(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::LeftParen | TokenKind::LeftBracket | TokenKind::LeftBrace
    )
}

fn is_closing_delimiter(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace
    )
}

fn matches_delimiters(opening: TokenKind, closing: TokenKind) -> bool {
    matches!(
        (opening, closing),
        (TokenKind::LeftParen, TokenKind::RightParen)
            | (TokenKind::LeftBracket, TokenKind::RightBracket)
            | (TokenKind::LeftBrace, TokenKind::RightBrace)
    )
}

fn span_from_tokens(tokens: &[MetaprogrammingSyntaxToken]) -> Option<Span> {
    let first = tokens.first()?;
    let last = tokens.last()?;

    Span::try_new(first.span.start(), last.span.end()).ok()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::cursor::CursorPosition;

    fn test_span(offset: usize) -> Span {
        let start = CursorPosition::new(offset, 1, offset.saturating_add(1));
        let end = CursorPosition::new(
            offset.saturating_add(1),
            1,
            offset.saturating_add(2),
        );

        Span::try_new(start, end).expect("test span must be valid")
    }

    fn token<'a>(kind: TokenKind, spelling: &'a str, offset: usize) -> Token<'a> {
        Token::borrowed(kind, spelling, test_span(offset))
    }

    fn parser<'a>(
        tokens: &'a [Token<'a>],
    ) -> MetaprogrammingDomainParser<'a, 'a> {
        MetaprogrammingDomainParser::new(
            tokens,
            MetaprogrammingParserConfig::default(),
        )
    }

    #[test]
    fn recognizes_specialization_and_preserves_tokens() {
        let tokens = [
            token(TokenKind::Identifier, "specialize", 0),
            token(TokenKind::Identifier, "compute", 10),
            token(TokenKind::LessThan, "<", 17),
            token(TokenKind::Identifier, "T", 18),
            token(TokenKind::GreaterThan, ">", 19),
            token(TokenKind::Semicolon, ";", 20),
            token(TokenKind::Eof, "", 21),
        ];

        let mut parser = parser(&tokens);
        let parsed = parser
            .parse_next()
            .expect("parsing should succeed")
            .expect("specialization should be recognized");

        assert_eq!(parsed.kind, MetaprogrammingConstructKind::Specialization);
        assert_eq!(parsed.tokens.len(), 6);
        assert_eq!(parsed.token_range, 0..6);
        assert_eq!(parsed.tokens[1].spelling, "compute");
    }

    #[test]
    fn recognizes_quotation_with_balanced_body() {
        let tokens = [
            token(TokenKind::Identifier, "quote", 0),
            token(TokenKind::LeftBrace, "{", 5),
            token(TokenKind::Identifier, "let", 6),
            token(TokenKind::Identifier, "x", 10),
            token(TokenKind::Assign, "=", 12),
            token(TokenKind::IntegerLiteral, "1", 14),
            token(TokenKind::Semicolon, ";", 15),
            token(TokenKind::RightBrace, "}", 16),
            token(TokenKind::Eof, "", 17),
        ];

        let mut parser = parser(&tokens);
        let parsed = parser
            .parse_next()
            .expect("parsing should succeed")
            .expect("quotation should be recognized");

        assert_eq!(parsed.kind, MetaprogrammingConstructKind::Quotation);
        assert_eq!(parsed.token_range, 0..8);
        assert_eq!(parsed.tokens.last().map(|t| t.spelling.as_str()), Some("}"));
    }

    #[test]
    fn ordinary_const_declaration_is_not_automatically_metaprogramming() {
        let tokens = [
            token(TokenKind::KeywordConst, "const", 0),
            token(TokenKind::Identifier, "ANSWER", 6),
            token(TokenKind::Assign, "=", 13),
            token(TokenKind::IntegerLiteral, "42", 15),
            token(TokenKind::Semicolon, ";", 17),
            token(TokenKind::Eof, "", 18),
        ];

        let mut parser = parser(&tokens);
        assert!(parser.parse_next().expect("parse should succeed").is_none());
    }

    #[test]
    fn recognizes_explicit_const_execution_block() {
        let tokens = [
            token(TokenKind::KeywordConst, "const", 0),
            token(TokenKind::LeftBrace, "{", 6),
            token(TokenKind::Identifier, "compute", 7),
            token(TokenKind::LeftParen, "(", 14),
            token(TokenKind::RightParen, ")", 15),
            token(TokenKind::RightBrace, "}", 16),
            token(TokenKind::Eof, "", 17),
        ];

        let mut parser = parser(&tokens);
        let parsed = parser
            .parse_next()
            .expect("parse should succeed")
            .expect("compile-time block should be recognized");

        assert_eq!(
            parsed.kind,
            MetaprogrammingConstructKind::CompileTimeExecution
        );
    }

    #[test]
    fn nested_delimiters_are_scanned_iteratively() {
        let mut tokens = vec![
            token(TokenKind::Identifier, "quote", 0),
            token(TokenKind::LeftBrace, "{", 5),
        ];

        for offset in 0..128 {
            tokens.push(token(TokenKind::LeftParen, "(", 6 + offset));
        }

        for offset in 0..128 {
            tokens.push(token(TokenKind::RightParen, ")", 134 + offset));
        }

        tokens.push(token(TokenKind::RightBrace, "}", 262));
        tokens.push(token(TokenKind::Eof, "", 263));

        let mut parser = parser(&tokens);
        let parsed = parser
            .parse_next()
            .expect("balanced nesting should parse")
            .expect("quotation should be recognized");

        assert_eq!(parsed.kind, MetaprogrammingConstructKind::Quotation);
    }

    #[test]
    fn mismatched_delimiter_returns_structured_error() {
        let tokens = [
            token(TokenKind::Identifier, "quote", 0),
            token(TokenKind::LeftParen, "(", 5),
            token(TokenKind::RightBracket, "]", 6),
            token(TokenKind::Eof, "", 7),
        ];

        let mut parser = parser(&tokens);
        let error = parser.parse_next().expect_err("delimiter mismatch expected");

        assert_eq!(
            error.kind,
            MetaprogrammingParseErrorKind::MismatchedClosingDelimiter
        );
    }

    #[test]
    fn nesting_budget_is_configurable() {
        let tokens = [
            token(TokenKind::Identifier, "quote", 0),
            token(TokenKind::LeftBrace, "{", 5),
            token(TokenKind::LeftParen, "(", 6),
            token(TokenKind::RightParen, ")", 7),
            token(TokenKind::RightBrace, "}", 8),
            token(TokenKind::Eof, "", 9),
        ];

        let config = MetaprogrammingParserConfig {
            limits: MetaprogrammingParserLimits {
                max_tokens_per_construct: None,
                max_nesting_depth: Some(1),
            },
            extension_introducers: Vec::new(),
        };

        let mut parser = MetaprogrammingDomainParser::new(&tokens, config);
        let error = parser.parse_next().expect_err("nesting budget should fail");

        assert_eq!(
            error.kind,
            MetaprogrammingParseErrorKind::NestingBudgetExceeded
        );
    }

    #[test]
    fn extension_introducers_are_explicitly_registered() {
        let tokens = [
            token(TokenKind::Identifier, "vendor_meta", 0),
            token(TokenKind::LeftParen, "(", 11),
            token(TokenKind::IntegerLiteral, "1", 12),
            token(TokenKind::RightParen, ")", 13),
            token(TokenKind::Semicolon, ";", 14),
            token(TokenKind::Eof, "", 15),
        ];

        let config = MetaprogrammingParserConfig {
            limits: MetaprogrammingParserLimits::default(),
            extension_introducers: vec!["vendor_meta".to_owned()],
        };

        let mut parser = MetaprogrammingDomainParser::new(&tokens, config);
        let parsed = parser
            .parse_next()
            .expect("extension parsing should succeed")
            .expect("registered extension should be recognized");

        assert_eq!(
            parsed.kind,
            MetaprogrammingConstructKind::Extension("vendor_meta".to_owned())
        );
    }

    #[test]
    fn repeated_parses_do_not_share_mutable_state() {
        let tokens = [
            token(TokenKind::Identifier, "specialize", 0),
            token(TokenKind::Identifier, "first", 11),
            token(TokenKind::Semicolon, ";", 16),
            token(TokenKind::Identifier, "specialize", 18),
            token(TokenKind::Identifier, "second", 29),
            token(TokenKind::Semicolon, ";", 35),
            token(TokenKind::Eof, "", 36),
        ];

        let mut parser = parser(&tokens);

        let first = parser
            .parse_next()
            .expect("first parse should succeed")
            .expect("first construct should be recognized");

        let second = parser
            .parse_next()
            .expect("second parse should succeed")
            .expect("second construct should be recognized");

        assert_eq!(first.tokens[1].spelling, "first");
        assert_eq!(second.tokens[1].spelling, "second");
        assert!(first.token_range.end <= second.token_range.start);
    }

    #[test]
    fn zero_token_budget_fails_without_retrying_same_position() {
        let tokens = [
            token(TokenKind::Identifier, "specialize", 0),
            token(TokenKind::Semicolon, ";", 10),
            token(TokenKind::Eof, "", 11),
        ];

        let config = MetaprogrammingParserConfig {
            limits: MetaprogrammingParserLimits {
                max_tokens_per_construct: Some(0),
                max_nesting_depth: None,
            },
            extension_introducers: Vec::new(),
        };

        let mut parser = MetaprogrammingDomainParser::new(&tokens, config);
        let error = parser.parse_next().expect_err("zero budget should fail");

        assert_eq!(
            error.kind,
            MetaprogrammingParseErrorKind::TokenBudgetExceeded
        );
        assert!(parser.position() > 0);
    }
}
