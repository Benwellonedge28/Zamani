
#![forbid(unsafe_code)]

//! Zamani parser — compilation-unit grammar.
//!
//! File: src/frontend/parser/grammar/program.rs
//! Rust: 1.97+
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Parse a complete source unit by repeatedly delegating top-level syntax
//! to the authoritative item parser.
//!
//! # Responsibilities
//!
//! - Define the compilation-unit parsing boundary.
//! - Preserve top-level item order.
//! - Require an explicit end-of-file token.
//! - Detect a parser that makes no progress.
//! - Propagate item-parser and token-cursor failures.
//! - Reject missing EOF rather than silently accepting truncated input.
//! - Keep source syntax independent of execution targets.
//!
//! # Does not own
//!
//! - Lexical recognition or token construction.
//! - Declaration, statement, expression, or type productions.
//! - AST node allocation or AST storage.
//! - Semantic analysis, type checking, or name resolution.
//! - Quantum operation registries or HDL implementations.
//! - Hardware discovery, placement, scheduling, or lowering.
//!
//! # Integration contract
//!
//! The parser session implements `ProgramDriver` by adapting the canonical
//! lexer token cursor and delegating `parse_item` to `grammar/items.rs`.
//!
//! The AST adapter consumes the returned ordered items and constructs the
//! repository's canonical Program AST using its NodeId allocator, source
//! span, metadata, and AST storage contracts.
//!
//! This module intentionally does not invent a competing AST representation.
//!
//! # Scalability
//!
//! No machine-size, hardware-capacity, or domain-specific limits are defined
//! here. The number of parsed items is governed by available resources and
//! the parser's configured resource policy.
//!
//! The driver must enforce its configured token, work, nesting, and allocation
//! budgets in the corresponding cursor/session/limits implementations.

use std::error::Error;
use std::fmt;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token_kind::TokenKind;

/// Operations required from the parser session to parse a source unit.
///
/// Implement this trait in the parser-session integration layer. Do not
/// implement a second lexer or token vocabulary for the program grammar.
///
/// `position()` must return a monotonically increasing token-stream position
/// for each successfully consumed token. A position must not wrap around.
/// Repeated observations without consumption must return the same position.
///
/// `current_kind()` returns `None` only when the token stream has ended
/// without an explicit EOF token. A valid complete source stream must expose
/// `TokenKind::Eof`.
pub trait ProgramDriver {
    /// The top-level item produced by the authoritative item grammar.
    type Item;

    /// Error returned by the cursor or item parser.
    type Error: Error + Send + Sync + 'static;

    /// Return the kind of the current token.
    ///
    /// The implementation must not advance the cursor.
    fn current_kind(&self) -> Option<TokenKind>;

    /// Return the current token's source span, if available.
    fn current_span(&self) -> Option<Span>;

    /// Return the current token-stream position.
    fn position(&self) -> usize;

    /// Consume exactly one current token.
    ///
    /// Calling this at EOF must be handled according to the session's cursor
    /// contract; this grammar consumes EOF at most once.
    fn advance(&mut self) -> Result<(), Self::Error>;

    /// Parse one complete top-level item.
    ///
    /// This method must be implemented by the shared item grammar. It must
    /// either consume source input and return an item, or return an error.
    /// It must not report success without making progress.
    ///
    /// It must not consume EOF as an ordinary item.
    fn parse_item(&mut self) -> Result<Self::Item, Self::Error>;
}

/// Result of parsing a complete source unit.
///
/// Items remain in source order. Construction of the canonical AST
/// `Program` is performed by the AST adapter, which has access to the
/// repository's node allocator and AST storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedProgram<I> {
    items: Vec<I>,
    eof_span: Span,
}

impl<I> ParsedProgram<I> {
    /// Construct a parsed program from already validated parser output.
    ///
    /// This constructor is intended for integration and testing. The
    /// production entry point is [`parse_program`].
    #[must_use]
    pub fn new(items: Vec<I>, eof_span: Span) -> Self {
        Self { items, eof_span }
    }

    /// Return top-level items in source order.
    #[must_use]
    pub fn items(&self) -> &[I] {
        &self.items
    }

    /// Consume the result and return its top-level items.
    #[must_use]
    pub fn into_items(self) -> Vec<I> {
        self.items
    }

    /// Return the source span of the EOF token.
    #[must_use]
    pub const fn eof_span(&self) -> &Span {
        &self.eof_span
    }

    /// Return the number of top-level items.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Return whether the source unit contains no top-level items.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Failure while parsing a compilation unit.
#[derive(Debug)]
#[non_exhaustive]
pub enum ProgramParseError<E> {
    /// The token stream ended without an explicit EOF token.
    MissingEof {
        /// Last available source location, if any.
        span: Option<Span>,
    },

    /// The item parser succeeded without consuming input.
    ///
    /// This prevents infinite loops caused by faulty grammar dispatch or
    /// extension handlers.
    NoProgress {
        /// Cursor position before parsing the item.
        position: usize,

        /// Current source location, if available.
        span: Option<Span>,
    },

    /// The token cursor failed while consuming EOF.
    EofConsumption(E),

    /// The authoritative item grammar failed.
    Item(E),
}

impl<E: fmt::Display> fmt::Display for ProgramParseError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEof { span } => {
                write!(f, "source unit is missing an explicit EOF token")?;
                if let Some(span) = span {
                    write!(f, " near {span}")?;
                }
                Ok(())
            }
            Self::NoProgress { position, span } => {
                write!(
                    f,
                    "top-level parser made no progress at token position {position}"
                )?;
                if let Some(span) = span {
                    write!(f, " near {span}")?;
                }
                Ok(())
            }
            Self::EofConsumption(error) => {
                write!(f, "failed to consume EOF token: {error}")
            }
            Self::Item(error) => {
                write!(f, "failed to parse top-level item: {error}")
            }
        }
    }
}

impl<E> Error for ProgramParseError<E>
where
    E: Error + Send + Sync + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EofConsumption(error) | Self::Item(error) => Some(error),
            Self::MissingEof { .. } | Self::NoProgress { .. } => None,
        }
    }
}

/// Parse a complete source unit using the shared item grammar.
///
/// The function preserves item order, requires an explicit EOF token, and
/// refuses to continue if an item parser reports success without consuming
/// input.
///
/// Lexical errors, syntax errors, recovery decisions, diagnostics, and
/// configured resource-budget enforcement remain the responsibility of the
/// session and item grammar.
///
/// An empty source unit is accepted if the language edition permits it and
/// the lexer supplies EOF as the first token. Whether an empty program is
/// semantically valid is determined by the language specification.
///
/// # Errors
///
/// Returns an error when EOF is absent, item parsing fails, an item parser
/// makes no progress, or EOF consumption fails.
pub fn parse_program<D>(
    driver: &mut D,
) -> Result<ParsedProgram<D::Item>, ProgramParseError<D::Error>>
where
    D: ProgramDriver,
{
    let mut items = Vec::new();

    loop {
        match driver.current_kind() {
            Some(TokenKind::Eof) => {
                let eof_span = driver.current_span().ok_or_else(|| {
                    ProgramParseError::MissingEof {
                        span: None,
                    }
                })?;

                driver
                    .advance()
                    .map_err(ProgramParseError::EofConsumption)?;

                return Ok(ParsedProgram::new(items, eof_span));
            }

            None => {
                return Err(ProgramParseError::MissingEof {
                    span: driver.current_span(),
                });
            }

            Some(_) => {
                let before = driver.position();
                let item = driver
                    .parse_item()
                    .map_err(ProgramParseError::Item)?;
                let after = driver.position();

                if after <= before {
                    return Err(ProgramParseError::NoProgress {
                        position: before,
                        span: driver.current_span(),
                    });
                }

                items.push(item);
            }
        }
    }
}
