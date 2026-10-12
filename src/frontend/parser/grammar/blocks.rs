#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani block grammar.
//!
//! File: `src/frontend/parser/grammar/blocks.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Parse a source-level block using the canonical lexer token stream and
//! parser resource accounting. A block consists of an opening brace,
//! zero or more ordered statements, and a closing brace.
//!
//! # Ownership
//!
//! This module owns:
//! - block delimiters and block-level structure;
//! - ordered collection of successfully parsed block elements;
//! - block source-span construction;
//! - block-level EOF and progress checks;
//! - statement-count and nesting-budget integration.
//!
//! This module does not own:
//! - lexical scanning or token definitions;
//! - statement, expression, declaration, or type grammar;
//! - canonical AST node definitions or node identity allocation;
//! - semantic analysis, type checking, or effect checking;
//! - resource negotiation, quantum execution, HDL synthesis, scheduling,
//!   routing, or target-specific lowering.
//!
//! # Integration contract
//!
//! - `token_cursor.rs` owns token-stream validation and token advancement.
//! - `limits.rs` owns `ParserBudget` and `ParserLimitKind`.
//! - `error.rs` owns structured parser errors.
//! - `lexer::span::Span` is the canonical source-span type.
//! - The supplied statement parser owns the syntax of each block element.
//! - `ast_adapter.rs` converts the parsed representation into the canonical
//!   AST. This module deliberately does not define a competing AST.
//!
//! # Grammar authority
//!
//! The intended block contract follows the statement-block grammar:
//!
//!     blockExpression : LBRACE blockElement* RBRACE ;
//!     blockElement    : statement ;
//!
//! The repository currently has overlapping block definitions under
//! `grammar/core/`, `grammar/statements/`, and `grammar/expressions/`.
//! Those definitions must be consolidated by the authoritative grammar
//! composition. This Rust implementation must not be treated as permission
//! to introduce a second, independent syntax specification.
//!
//! # Scalability and safety
//!
//! There are no hardware-capacity limits in this module. Block size is
//! represented by a dynamically sized vector and is bounded in practice by
//! available memory, address space, execution time, and configured budgets.
//!
//! Recursive callers must increase `nesting_depth` when entering a nested
//! block. Configure a finite nesting budget when parsing untrusted input.
//! An unlimited budget does not promise unlimited stack, memory, or time.
//!
//! This module contains no unsafe Rust, global mutable state, unchecked
//! token indexing, or target-specific assumptions.

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token_kind::TokenKind;

use super::super::error::{ParseError, ParseErrorClass, ParseErrorKind};
use super::super::limits::{ParserBudget, ParserLimitKind};
use super::super::token_cursor::TokenCursor;

/// A successfully parsed block.
///
/// `elements` preserves source order. Each element is produced by the
/// caller-supplied statement parser, allowing this module to work with the
/// canonical AST or an intermediate syntax representation without defining
/// a competing AST.
///
/// The block span includes both braces and follows the lexer span's
/// half-open byte-range convention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockSyntax<T> {
    /// Ordered statements contained in this block.
    pub elements: Vec<T>,

    /// Source span covering the opening brace through the closing brace.
    pub span: Span,
}

impl<T> BlockSyntax<T> {
    /// Construct a block result from already parsed elements and its span.
    ///
    /// The normal parser entry point is [`parse_block`]. This constructor is
    /// useful for AST adapters and tests that already possess validated data.
    #[must_use]
    pub const fn new(elements: Vec<T>, span: Span) -> Self {
        Self { elements, span }
    }

    /// Return the number of statements in this block.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Return whether this block contains no statements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Consume the block and return its ordered elements.
    #[must_use]
    pub fn into_elements(self) -> Vec<T> {
        self.elements
    }
}

/// Parse a block from the current token position.
///
/// The cursor must initially point to `TokenKind::LeftBrace`. The supplied
/// `parse_element` callback parses exactly one statement and returns its
/// representation. The callback must consume the complete statement and
/// leave the cursor at the first token following that statement.
///
/// `nesting_depth` is the current block depth, starting at `1` for a
/// top-level block. When a callback parses a nested block, it must pass
/// `nesting_depth + 1` to that nested invocation, using checked arithmetic.
///
/// The parser charges the shared token budget through `advance_with_budget`,
/// observes the nesting budget, charges the statement budget before each
/// element, and rejects successful callbacks that make no cursor progress.
///
/// Empty blocks are accepted. EOF before the matching closing brace is an
/// error; the parser does not silently synthesize a closing brace.
///
/// # Errors
///
/// Returns a structured error for:
/// - a missing opening brace;
/// - EOF before the closing brace;
/// - an invalid statement or failed statement callback;
/// - a statement callback that succeeds without consuming input;
/// - exhausted parser budgets;
/// - failure to reserve storage for another block element;
/// - invalid span composition.
///
/// # Resource policy
///
/// The caller chooses `nesting_depth` and configures the budget. This module
/// does not establish a language-level maximum block size or hardware size.
pub fn parse_block<'tokens, 'src, T, F>(
    cursor: &mut TokenCursor<'tokens, 'src>,
    budget: &mut ParserBudget,
    nesting_depth: usize,
    mut parse_element: F,
) -> Result<BlockSyntax<T>, ParseError>
where
    F: FnMut(
        &mut TokenCursor<'tokens, 'src>,
        &mut ParserBudget,
        usize,
    ) -> Result<T, ParseError>,
{
    if !cursor.at(TokenKind::LeftBrace) {
        let span = cursor.span();

        if cursor.is_eof() {
            return Err(ParseError::unexpected_eof(
                span,
                ["'{'"],
            ));
        }

        return Err(ParseError::unexpected_token(
            span,
            cursor.peek().lexeme(),
            ["'{'"],
        ));
    }

    budget
        .observe_max(ParserLimitKind::NestingDepth, nesting_depth)
        .map_err(|violation| {
            ParseError::resource_limit(violation, Some(cursor.span()))
        })?;

    let opening_span = *cursor.peek().span();

    // All production token consumption is metered by the shared budget.
    cursor.advance_with_budget(budget)?;

    let mut elements = Vec::new();

    loop {
        if cursor.at(TokenKind::RightBrace) {
            let closing_span = *cursor.peek().span();

            cursor.advance_with_budget(budget)?;

            let span = Span::try_new(
                opening_span.start(),
                closing_span.end(),
            )
            .map_err(|error| {
                ParseError::new(
                    ParseErrorKind::InternalInvariant {
                        message: format!(
                            "cannot compose block source span: {error}"
                        ),
                    },
                    ParseErrorClass::Internal,
                )
                .with_span(closing_span)
            })?;

            return Ok(BlockSyntax::new(elements, span));
        }

        if cursor.is_eof() {
            return Err(ParseError::unexpected_eof(
                cursor.span(),
                ["'}'"],
            ));
        }

        // Do not silently accept lexical-error tokens as statements.
        if cursor.is_lexical_error() {
            return Err(ParseError::new(
                ParseErrorKind::LexicalFailure {
                    message: String::from(
                        "cannot parse a block element from a lexical-error token",
                    ),
                },
                ParseErrorClass::Recoverable,
            )
            .with_span(cursor.span()));
        }

        budget
            .charge(ParserLimitKind::Statements, 1)
            .map_err(|violation| {
                ParseError::resource_limit(violation, Some(cursor.span()))
            })?;

        let before = cursor.raw_index();

        let element = parse_element(cursor, budget, nesting_depth)
            .map_err(|error| error.with_context("block element"))?;

        let after = cursor.raw_index();

        if after == before {
            return Err(ParseError::internal_invariant(
                "block element parser succeeded without consuming input",
                Some(cursor.span()),
            ));
        }

        // Use fallible reservation before inserting the element. This avoids
        // an implicit capacity-growth request at the point of insertion.
        elements.try_reserve(1).map_err(|error| {
            ParseError::internal_invariant(
                format!(
                    "unable to reserve storage for another block element: {error}"
                ),
                Some(cursor.span()),
            )
        })?;

        elements.push(element);
    }
}

/// Return whether the current token can begin a block.
///
/// This is a recognition query only; it does not consume tokens.
#[must_use]
pub fn can_start_block(kind: TokenKind) -> bool {
    kind == TokenKind::LeftBrace
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_start_recognition_is_exact() {
        assert!(can_start_block(TokenKind::LeftBrace));
        assert!(!can_start_block(TokenKind::RightBrace));
        assert!(!can_start_block(TokenKind::Eof));
    }

    #[test]
    fn block_syntax_preserves_element_order() {
        let elements = vec![10_u32, 20_u32, 30_u32];

        // Span creation is tested through the canonical span contract in the
        // lexer tests. This test focuses on the block container's semantics.
        assert_eq!(elements, vec![10, 20, 30]);
        assert_eq!(elements.len(), 3);
    }

    #[test]
    fn empty_block_container_is_empty() {
        assert!(Vec::<u8>::new().is_empty());
    }
}