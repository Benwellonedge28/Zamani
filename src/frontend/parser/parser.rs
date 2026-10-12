#![forbid(unsafe_code)]

//! Zamani frontend parser orchestration.
//!
//! File: src/frontend/parser/parser.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Coordinate complete-program parsing using the canonical frontend token
//! representation. Grammar productions are provided by an implementation of
//! `ParserGrammar`; this module owns token-stream integrity, EOF enforcement,
//! parser progress checks, and the public orchestration boundary.
//!
//! # Responsibilities
//!
//! - Consume canonical frontend tokens without re-lexing source.
//! - Validate the basic token-stream contract before parsing.
//! - Expose deterministic lookahead and token advancement.
//! - Require complete input consumption for complete-program parsing.
//! - Provide structured parser errors with canonical source spans.
//! - Detect non-progressing grammar implementations.
//! - Keep per-parse state local to a parser instance.
//!
//! # Does not own
//!
//! - Lexical recognition or token spelling registries.
//! - The normative grammar or ANTLR composition.
//! - Canonical AST node definitions.
//! - Type checking, name resolution, effect analysis, capability checking,
//!   resource negotiation, or semantic interpretation.
//! - Quantum operation catalogs, HDL synthesis, scheduling, or target choice.
//!
//! # Integration contract
//!
//! - `crate::frontend::lexer::token::Token` is the canonical token type.
//! - `crate::frontend::lexer::token_kind::TokenKind` is the canonical kind.
//! - `crate::frontend::lexer::span::Span` is the canonical source span.
//! - Grammar modules implement `ParserGrammar` and construct the existing AST.
//! - `api.rs` owns the stable public source/token entry points.
//! - `session.rs` may later own configuration, budgets, and diagnostics.
//! - `ast_adapter.rs` owns any conversion required by the existing AST.
//!
//! This file intentionally does not invent AST types or assume constructors
//! for AST nodes whose exact signatures must be taken from the repository.
//!
//! # Resource behavior
//!
//! Token storage is borrowed. This module imposes no language-level maximum
//! token count, nesting depth, qubit count, tensor rank, device count, or
//! hardware capacity. The caller remains responsible for applying the
//! configured source and parser budgets before or during token production.
//!
//! No finite implementation can process literally infinite input. Parsing
//! is bounded by available resources and explicit implementation budgets.

use core::fmt;
use std::error::Error;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

/// Parser-level failure classification.
///
/// These categories describe parsing and token-stream integrity only.
/// Semantic analysis should use its own error classifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ParserErrorKind {
    /// The token stream is empty or does not contain an explicit EOF token.
    MissingEof,

    /// An EOF token appears before the final token.
    EofNotLast,

    /// More than one EOF token appears in the stream.
    DuplicateEof,

    /// The token stream contains a token classified as lexically invalid.
    InvalidToken,

    /// The grammar encountered a token where another construct was expected.
    UnexpectedToken,

    /// The grammar returned successfully without consuming the complete input.
    TrailingInput,

    /// The grammar did not advance while claiming to parse a construct.
    NoProgress,

    /// The grammar implementation failed.
    GrammarFailure,

    /// A caller supplied an inconsistent parser state or token stream.
    InvalidParserState,
}

impl fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingEof => "token stream has no final EOF token",
            Self::EofNotLast => "EOF token is not the final token",
            Self::DuplicateEof => "token stream contains multiple EOF tokens",
            Self::InvalidToken => "token stream contains an invalid token",
            Self::UnexpectedToken => "unexpected token",
            Self::TrailingInput => "unconsumed input remains after parsing",
            Self::NoProgress => "grammar did not make parsing progress",
            Self::GrammarFailure => "grammar implementation failed",
            Self::InvalidParserState => "invalid parser state",
        };

        f.write_str(message)
    }
}

impl Error for ParserErrorKind {}

/// A structured parser error.
///
/// `span` identifies the relevant source range. `message` is intended for
/// people; callers should use `kind` for machine-readable classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub span: Span,
    pub message: String,
}

impl ParserError {
    /// Create a parser error with a stable classification and source span.
    #[must_use]
    pub fn new(
        kind: ParserErrorKind,
        span: Span,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    /// Create a token-stream error using the token's canonical source span.
    #[must_use]
    pub fn at_token(
        kind: ParserErrorKind,
        token: &Token<'_>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(kind, *token.span(), message)
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl Error for ParserError {}

/// Borrowed cursor over an already-tokenized source.
///
/// The cursor never performs lexical recognition. It also does not allocate
/// or clone tokens during ordinary lookahead and advancement.
#[derive(Debug, Clone)]
pub struct TokenCursor<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    position: usize,
}

impl<'tokens, 'src> TokenCursor<'tokens, 'src> {
    /// Construct a cursor after validating the complete token-stream shape.
    ///
    /// The stream must contain exactly one EOF token, and it must be last.
    /// A lexical error token is rejected separately so callers can preserve
    /// the original lexical diagnostic rather than receiving a generic
    /// parser error.
    pub fn new(tokens: &'tokens [Token<'src>]) -> Result<Self, ParserError> {
        let Some(last) = tokens.last() else {
            return Err(ParserError::new(
                ParserErrorKind::MissingEof,
                empty_span(),
                "cannot parse an empty token slice; provide the lexer's EOF token",
            ));
        };

        let mut eof_count = 0usize;

        for (index, token) in tokens.iter().enumerate() {
            if token.kind() == TokenKind::Eof {
                eof_count = eof_count.checked_add(1).ok_or_else(|| {
                    ParserError::at_token(
                        ParserErrorKind::InvalidParserState,
                        token,
                        "EOF counter overflow",
                    )
                })?;

                if index + 1 != tokens.len() {
                    return Err(ParserError::at_token(
                        ParserErrorKind::EofNotLast,
                        token,
                        "the EOF token must be the final token",
                    ));
                }
            }

            if token.kind() == TokenKind::Invalid || token.is_error_channel() {
                return Err(ParserError::at_token(
                    ParserErrorKind::InvalidToken,
                    token,
                    "resolve or report the lexical error before parsing this stream",
                ));
            }
        }

        if eof_count == 0 {
            return Err(ParserError::at_token(
                ParserErrorKind::MissingEof,
                last,
                "the token stream must end with exactly one EOF token",
            ));
        }

        if eof_count != 1 {
            return Err(ParserError::at_token(
                ParserErrorKind::DuplicateEof,
                last,
                "the token stream must contain exactly one EOF token",
            ));
        }

        Ok(Self {
            tokens,
            position: 0,
        })
    }

    /// Return the complete token slice.
    #[must_use]
    pub const fn tokens(&self) -> &'tokens [Token<'src>] {
        self.tokens
    }

    /// Return the current token index.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Return the number of tokens in the stream, including EOF.
    #[must_use]
    pub const fn token_count(&self) -> usize {
        self.tokens.len()
    }

    /// Return the current token without advancing.
    ///
    /// Returns `None` only if the cursor is already in an invalid state.
    /// A correctly constructed cursor always has a current token until EOF
    /// has been consumed.
    #[must_use]
    pub fn current(&self) -> Option<&'tokens Token<'src>> {
        self.tokens.get(self.position)
    }

    /// Return the current token kind.
    #[must_use]
    pub fn current_kind(&self) -> Option<TokenKind> {
        self.current().map(Token::kind)
    }

    /// Return the token at a lookahead distance from the current position.
    ///
    /// Lookahead is relative to the current token: distance zero is current.
    #[must_use]
    pub fn lookahead(&self, distance: usize) -> Option<&'tokens Token<'src>> {
        self.position
            .checked_add(distance)
            .and_then(|index| self.tokens.get(index))
    }

    /// Return whether the current token is EOF.
    #[must_use]
    pub fn at_eof(&self) -> bool {
        self.current_kind() == Some(TokenKind::Eof)
    }

    /// Advance by one token, but never advance past EOF.
    ///
    /// Returns the token that was current before advancement. Repeated calls
    /// at EOF return the EOF token and leave the cursor at EOF.
    pub fn advance(&mut self) -> Result<&'tokens Token<'src>, ParserError> {
        let token = self.current().ok_or_else(|| {
            ParserError::new(
                ParserErrorKind::InvalidParserState,
                empty_span(),
                "cursor position is outside the token stream",
            )
        })?;

        if token.kind() != TokenKind::Eof {
            self.position = self.position.checked_add(1).ok_or_else(|| {
                ParserError::at_token(
                    ParserErrorKind::InvalidParserState,
                    token,
                    "token cursor position overflow",
                )
            })?;
        }

        Ok(token)
    }

    /// Consume the current token if it has the expected kind.
    pub fn consume_if(
        &mut self,
        expected: TokenKind,
    ) -> Result<Option<&'tokens Token<'src>>, ParserError> {
        if self.current_kind() == Some(expected) {
            self.advance().map(Some)
        } else {
            Ok(None)
        }
    }

    /// Consume the current token, requiring its kind to match `expected`.
    pub fn expect(
        &mut self,
        expected: TokenKind,
    ) -> Result<&'tokens Token<'src>, ParserError> {
        let Some(token) = self.current() else {
            return Err(ParserError::new(
                ParserErrorKind::InvalidParserState,
                empty_span(),
                "expected a token but the cursor has no current token",
            ));
        };

        if token.kind() != expected {
            return Err(ParserError::at_token(
                ParserErrorKind::UnexpectedToken,
                token,
                format!(
                    "expected token kind {expected:?}, found {:?}",
                    token.kind()
                ),
            ));
        }

        self.advance()
    }

    /// Save the current cursor position for speculative parsing.
    #[must_use]
    pub const fn checkpoint(&self) -> usize {
        self.position
    }

    /// Restore a previously saved position.
    ///
    /// The position must identify an existing token, including EOF.
    pub fn restore(&mut self, checkpoint: usize) -> Result<(), ParserError> {
        if checkpoint >= self.tokens.len() {
            let token = self.tokens.last().ok_or_else(|| {
                ParserError::new(
                    ParserErrorKind::InvalidParserState,
                    empty_span(),
                    "cannot restore a cursor over an empty token stream",
                )
            })?;

            return Err(ParserError::at_token(
                ParserErrorKind::InvalidParserState,
                token,
                "checkpoint is outside the token stream",
            ));
        }

        self.position = checkpoint;
        Ok(())
    }

    /// Return the current token's source span.
    pub fn current_span(&self) -> Result<Span, ParserError> {
        self.current()
            .map(|token| *token.span())
            .ok_or_else(|| {
                ParserError::new(
                    ParserErrorKind::InvalidParserState,
                    empty_span(),
                    "cursor has no current token",
                )
            })
    }

    /// Require EOF without consuming any trailing input.
    pub fn require_eof(&self) -> Result<(), ParserError> {
        let Some(token) = self.current() else {
            return Err(ParserError::new(
                ParserErrorKind::InvalidParserState,
                empty_span(),
                "cursor has no current token",
            ));
        };

        if token.kind() != TokenKind::Eof {
            return Err(ParserError::at_token(
                ParserErrorKind::TrailingInput,
                token,
                "the grammar returned before consuming the complete source",
            ));
        }

        Ok(())
    }
}

/// Grammar implementation used by the parser orchestrator.
///
/// Implement this trait in the shared grammar layer. The associated `Program`
/// type must be the repository's canonical AST program/root type, not a
/// competing AST defined by the parser.
///
/// Grammar methods should return structured errors and must not silently
/// ignore unexpected input. Semantic validation belongs downstream.
pub trait ParserGrammar<'tokens, 'src> {
    /// Canonical AST type returned by complete-program parsing.
    type Program;

    /// Parse one complete source program using the supplied cursor.
    ///
    /// On success, the cursor must be positioned at EOF. The orchestrator
    /// verifies this postcondition independently.
    fn parse_program(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Program, ParserError>;
}

/// Parser orchestration over canonical tokens and a grammar implementation.
///
/// The grammar is supplied by the caller so shared grammar productions and
/// registered domain extensions can evolve independently of this file.
#[derive(Debug)]
pub struct Parser<'tokens, 'src, G> {
    cursor: TokenCursor<'tokens, 'src>,
    grammar: G,
}

impl<'tokens, 'src, G> Parser<'tokens, 'src, G>
where
    G: ParserGrammar<'tokens, 'src>,
{
    /// Create a parser after validating the token stream.
    pub fn new(
        tokens: &'tokens [Token<'src>],
        grammar: G,
    ) -> Result<Self, ParserError> {
        Ok(Self {
            cursor: TokenCursor::new(tokens)?,
            grammar,
        })
    }

    /// Parse a complete program and require complete input consumption.
    ///
    /// This function does not claim that the returned program is
    /// semantically valid. Type, effect, capability, resource, contract, and
    /// target checks belong to later compiler stages.
    pub fn parse_program(mut self) -> Result<G::Program, ParserError> {
        let start = self.cursor.position();

        let program = self.grammar.parse_program(&mut self.cursor)?;

        self.cursor.require_eof()?;

        if self.cursor.position() < start {
            return Err(ParserError::new(
                ParserErrorKind::InvalidParserState,
                self.cursor.current_span()?,
                "grammar moved the cursor backwards without restoring a checkpoint",
            ));
        }

        Ok(program)
    }

    /// Borrow the current token cursor.
    #[must_use]
    pub const fn cursor(&self) -> &TokenCursor<'tokens, 'src> {
        &self.cursor
    }

    /// Mutably borrow the current token cursor.
    ///
    /// This is intended for parser orchestration and integration tests.
    /// Grammar code should ordinarily receive the cursor through
    /// `ParserGrammar::parse_program`.
    pub fn cursor_mut(&mut self) -> &mut TokenCursor<'tokens, 'src> {
        &mut self.cursor
    }

    /// Borrow the grammar implementation.
    #[must_use]
    pub const fn grammar(&self) -> &G {
        &self.grammar
    }

    /// Mutably borrow the grammar implementation.
    pub fn grammar_mut(&mut self) -> &mut G {
        &mut self.grammar
    }

    /// Consume the parser and return the grammar implementation.
    #[must_use]
    pub fn into_grammar(self) -> G {
        self.grammar
    }
}

/// Construct a neutral span for errors that have no valid token location.
///
/// This helper uses the canonical span's zero-width constructor and the
/// canonical cursor-position type. It is isolated here to keep all other
/// parser code independent of coordinate construction details.
fn empty_span() -> Span {
    use crate::frontend::lexer::cursor::CursorPosition;

    Span::at(CursorPosition::new(0, 1, 1))
}