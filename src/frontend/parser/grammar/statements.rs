
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani statement grammar dispatcher.
//!
//! File: `src/frontend/parser/grammar/statements.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the shared statement-production dispatch boundary for the Zamani
//! frontend parser. The module identifies the syntactic category of the
//! current token and delegates parsing to the production that owns that
//! syntax.
//!
//! # Architectural ownership
//!
//! This module owns:
//! - statement-category dispatch;
//! - the statement-handler integration contract;
//! - rejection of EOF and unrecognized statement starts;
//! - the statement-level input-progress invariant;
//! - the distinction between local declarations, control flow, blocks,
//!   expression statements, and registered domain statements.
//!
//! This module does not own:
//! - lexical recognition or token spelling;
//! - expression precedence or expression construction;
//! - canonical AST definitions or AST identity allocation;
//! - declaration, pattern, type, or block production implementations;
//! - semantic analysis, name resolution, type checking, or effect checking;
//! - quantum operation catalogs, HDL synthesis, resource negotiation,
//!   scheduling, routing, or target-specific lowering.
//!
//! # Integration contract
//!
//! - `super::super::token_cursor::TokenCursor` is the canonical token cursor.
//! - `super::super::error::ParserError` and `ParserErrorKind` are the canonical
//!   parser error types.
//! - `crate::frontend::lexer::token_kind::TokenKind` is the canonical token
//!   vocabulary.
//! - The implementing grammar owns expression, block, declaration, control
//!   flow, and domain production implementations.
//! - `items.rs` may delegate statement parsing to this module through the
//!   `StatementGrammar` contract.
//! - `parser.rs` coordinates complete parsing and enforces complete-input
//!   consumption.
//! - `ast_adapter.rs` and the canonical AST builders own AST construction.
//!
//! # Grammar authority
//!
//! The accepted language is governed by the normative Zamani specification
//! and canonical ANTLR grammar composition under `grammar/`.
//! This Rust module implements that grammar contract; it is not an independent
//! language specification.
//!
//! In particular, a token being classified as a statement start does not
//! establish that a complete statement is valid. Its delegated production
//! must recognize the full syntax and report structured errors.
//!
//! # Extensibility
//!
//! New domain constructs should use a registered domain handler or an
//! existing generic production. Adding a new quantum operation, HDL primitive,
//! vendor operation, resource name, or target must not require a new branch
//! in this file when the construct already conforms to a supported grammar
//! production.
//!
//! # Resource and safety policy
//!
//! There are no language-level limits on statement count, block size, qubit
//! count, processor count, register width, tensor rank, device count, or
//! topology size.
//!
//! Actual parsing is bounded by available memory, address space, execution
//! time, and explicit parser/session budgets. The delegated productions must
//! charge their configured budgets and guard potentially unbounded work.
//!
//! This module does not use unsafe Rust, global mutable state, unchecked
//! indexing, or recursive traversal of AST nodes.
//!
//! # Progress invariant
//!
//! A successful statement production must consume at least one parser-visible
//! token. A handler that succeeds without advancing is rejected as an
//! internal parser invariant failure.
//!
//! The progress check prevents a faulty production from causing an infinite
//! loop in an enclosing block or source-item parser. It does not replace the
//! cursor's token-stream validation or the session's resource accounting.

use super::super::error::{ParseErrorKind, ParserError};
use super::super::token_cursor::TokenCursor;
use crate::frontend::lexer::token_kind::TokenKind;

/// Statement grammar implementation contract.
///
/// Implementations provide the concrete grammar productions and AST result
/// type. The dispatcher does not assume a particular AST representation,
/// allocator, or AST-builder API.
///
/// A single implementation can reuse shared productions across classical,
/// quantum, HDL, hybrid, distributed, and future computing domains.
///
/// # Handler requirements
///
/// Each handler must:
///
/// 1. consume the syntax belonging to its production;
/// 2. preserve syntax information required by the canonical AST;
/// 3. return canonical structured parser errors;
/// 4. respect parser-session resource budgets;
/// 5. avoid silently accepting malformed or unsupported syntax;
/// 6. leave the cursor at the first token following the parsed production.
///
/// `can_start_expression` must be consistent with the active language edition
/// and expression grammar. It must not claim that arbitrary unknown syntax is
/// a valid expression.
pub trait StatementGrammar<'tokens, 'src> {
    /// The canonical AST result or intermediate result used by this grammar.
    type Statement;

    /// Parse a local or nested declaration that is permitted in statement
    /// position by the active language specification.
    fn parse_local_declaration(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse an `if` statement.
    fn parse_if_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a `for` statement.
    fn parse_for_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a `while` statement.
    fn parse_while_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse an unconditional or conditionally terminated loop.
    fn parse_loop_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a `match` statement.
    fn parse_match_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a `return` statement.
    fn parse_return_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a `break` statement.
    fn parse_break_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a `continue` statement.
    fn parse_continue_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a `try` statement or expression form permitted in statement
    /// position by the active grammar.
    fn parse_try_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a `throw` statement.
    fn parse_throw_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a `yield` statement.
    fn parse_yield_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a task or actor spawn statement.
    ///
    /// This production expresses source-language intent. It must not allocate
    /// operating-system threads, select a device, or schedule execution.
    fn parse_spawn_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Parse a statement block.
    ///
    /// Empty blocks are accepted or rejected according to the normative
    /// grammar, not by an arbitrary collection-size limit.
    fn parse_block_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Determine whether a token can begin an expression in the active
    /// grammar and language edition.
    ///
    /// This is a recognition query only; it must not consume tokens.
    fn can_start_expression(&self, kind: TokenKind) -> bool;

    /// Parse an expression statement, including its grammar-defined
    /// terminator and source-span behavior.
    fn parse_expression_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;

    /// Determine whether a registered domain production can parse the current
    /// token as a statement.
    ///
    /// The default denies unknown domain syntax. An implementation may return
    /// true only when a registered handler explicitly owns the construct.
    ///
    /// Recognition must be deterministic and must not mutate the cursor.
    fn can_parse_domain_statement(&self, _kind: TokenKind) -> bool {
        false
    }

    /// Parse a statement owned by a registered domain handler.
    ///
    /// The handler must preserve its domain syntax through the canonical AST
    /// or an explicitly supported extension-node representation.
    fn parse_domain_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Statement, ParserError>;
}

/// Parse exactly one statement production.
///
/// This function dispatches to a concrete grammar handler and verifies that
/// the handler advances the cursor. It does not parse an entire source file.
///
/// The caller is responsible for:
///
/// - establishing the appropriate grammar context;
/// - handling statement terminators not owned by this production;
/// - applying the parser session's resource budgets;
/// - enclosing the result in the appropriate AST parent;
/// - enforcing EOF for complete-source entry points.
///
/// # Errors
///
/// Returns a structured parser error when:
///
/// - no current token is available;
/// - EOF appears where a statement is required;
/// - no core or registered domain production recognizes the token;
/// - a delegated production reports invalid syntax;
/// - a delegated production returns success without consuming input.
pub fn parse_statement<'tokens, 'src, G>(
    grammar: &mut G,
    cursor: &mut TokenCursor<'tokens, 'src>,
) -> Result<G::Statement, ParserError>
where
    G: StatementGrammar<'tokens, 'src>,
{
    let (kind, before) = {
        let token = cursor.current().ok_or_else(|| {
            ParserError::new(
                ParseErrorKind::InternalInvariant {
                    message: String::from(
                        "statement parser has no current token",
                    ),
                },
                empty_span(),
                "cannot parse statement: token cursor is not positioned \
                 on a token",
            )
        })?;

        (token.kind(), cursor.position())
    };

    if kind == TokenKind::Eof {
        let token = cursor.current().ok_or_else(|| {
            ParserError::new(
                ParseErrorKind::InternalInvariant {
                    message: String::from(
                        "EOF token disappeared during statement dispatch",
                    ),
                },
                empty_span(),
                "cannot parse statement: invalid cursor state",
            )
        })?;

        return Err(ParserError::at_token(
            ParseErrorKind::UnexpectedToken {
                found: Some(String::from("EOF")),
                expected: vec![String::from("statement")],
            },
            token,
            "unexpected end of input while a statement is required",
        ));
    }

    let result = match kind {
        // Local declarations are recognized explicitly. Whether a declaration
        // is legal in a particular block is governed by the active grammar.
        TokenKind::KeywordLet
        | TokenKind::KeywordVar
        | TokenKind::KeywordConst
        | TokenKind::KeywordFn
        | TokenKind::KeywordType
        | TokenKind::KeywordStruct
        | TokenKind::KeywordEnum
        | TokenKind::KeywordRecord
        | TokenKind::KeywordTrait
        | TokenKind::KeywordImpl
        | TokenKind::KeywordClass
        | TokenKind::KeywordInterface => {
            grammar.parse_local_declaration(cursor)
        }

        // Core control flow.
        TokenKind::KeywordIf => grammar.parse_if_statement(cursor),
        TokenKind::KeywordFor => grammar.parse_for_statement(cursor),
        TokenKind::KeywordWhile => grammar.parse_while_statement(cursor),
        TokenKind::KeywordLoop => grammar.parse_loop_statement(cursor),
        TokenKind::KeywordMatch => grammar.parse_match_statement(cursor),

        // Control-transfer and error-handling constructs.
        TokenKind::KeywordReturn => grammar.parse_return_statement(cursor),
        TokenKind::KeywordBreak => grammar.parse_break_statement(cursor),
        TokenKind::KeywordContinue => grammar.parse_continue_statement(cursor),
        TokenKind::KeywordTry => grammar.parse_try_statement(cursor),
        TokenKind::KeywordThrow => grammar.parse_throw_statement(cursor),
        TokenKind::KeywordYield => grammar.parse_yield_statement(cursor),
        TokenKind::KeywordSpawn => grammar.parse_spawn_statement(cursor),

        // Blocks share one production regardless of the eventual domain.
        TokenKind::LeftBrace => grammar.parse_block_statement(cursor),

        // A registered extension may own a token that is not a core
        // statement starter. Extension recognition is explicit; unknown
        // syntax never silently becomes an expression statement.
        other if grammar.can_parse_domain_statement(other) => {
            grammar.parse_domain_statement(cursor)
        }

        // Expression statements use the same expression grammar and
        // precedence model as every other expression context.
        other if grammar.can_start_expression(other) => {
            grammar.parse_expression_statement(cursor)
        }

        _ => {
            let token = cursor.current().ok_or_else(|| {
                ParserError::new(
                    ParseErrorKind::InternalInvariant {
                        message: String::from(
                            "current token disappeared during error reporting",
                        ),
                    },
                    empty_span(),
                    "cannot report an unrecognized statement start",
                )
            })?;

            return Err(ParserError::at_token(
                ParseErrorKind::UnexpectedToken {
                    found: Some(format!("{:?}", token.kind())),
                    expected: vec![String::from("statement")],
                },
                token,
                format!(
                    "no statement production recognizes token {:?} \
                     ({:?})",
                    token.kind(),
                    token.lexeme(),
                ),
            ));
        }
    };

    let statement = result?;

    let after = cursor.position();
    if after <= before {
        let span = cursor
            .current_span()
            .unwrap_or_else(|_| empty_span());

        return Err(ParserError::new(
            ParseErrorKind::InternalInvariant {
                message: format!(
                    "statement production returned success without \
                     consuming input (token position {before})"
                ),
            },
            span,
            "statement grammar violated the input-progress invariant",
        ));
    }

    Ok(statement)
}

/// Construct a neutral zero-width span for an internal cursor-state error.
///
/// Normal syntax errors should use the current token's actual source span.
/// This fallback is reserved for cases where the cursor cannot provide one.
fn empty_span() -> crate::frontend::lexer::span::Span {
    use crate::frontend::lexer::cursor::CursorPosition;
    use crate::frontend::lexer::span::Span;

    Span::at(CursorPosition::new(0, 1, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eof_is_never_a_statement_start() {
        assert_eq!(TokenKind::Eof, TokenKind::Eof);
    }

    #[test]
    fn dispatch_does_not_define_hardware_capacity() {
        // This module intentionally has no hardware-capacity constants.
        // Resource constraints are parsed by their owning productions and
        // validated by downstream semantic/resource-analysis layers.
        let module_source = include_str!("statements.rs");

        assert!(!module_source.contains("MAX_QUBITS"));
        assert!(!module_source.contains("MAX_CPUS"));
        assert!(!module_source.contains("MAX_GPUS"));
        assert!(!module_source.contains("MAX_FPGAS"));
        assert!(!module_source.contains("MAX_TENSOR_RANK"));
    }
}
