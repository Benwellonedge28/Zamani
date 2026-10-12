#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani classical-domain grammar adapter.
//!
//! File: src/frontend/parser/domains/classical.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Route classical source constructs to the existing shared grammar
//! productions without creating a second parser or a competing AST.
//!
//! # Ownership
//!
//! This module owns:
//! - classification of classical construct entry points;
//! - the classical-production integration contract;
//! - deterministic delegation to shared grammar implementations;
//! - successful-production progress verification;
//! - explicit reporting of unsupported construct starts.
//!
//! This module does not own:
//! - lexical scanning, token spelling, or token registries;
//! - expression precedence or expression construction;
//! - declaration, statement, type, pattern, or block grammar;
//! - canonical AST definitions or AST allocation;
//! - name resolution, type checking, effects, or contract evaluation;
//! - resource negotiation, hardware discovery, scheduling, or lowering.
//!
//! # Integration contract
//!
//! Upstream:
//! - `crate::frontend::lexer::token_kind::TokenKind`
//! - `crate::frontend::parser::token_cursor::TokenCursor`
//!
//! Downstream:
//! - the shared declaration grammar;
//! - the shared statement grammar;
//! - the shared expression grammar;
//! - the universal item grammar and parser orchestration;
//! - the canonical AST adapter.
//!
//! An implementation of `ClassicalProductions` must delegate to the
//! existing shared productions and return their canonical output type.
//! It must not implement a competing expression parser, declaration parser,
//! or AST representation.
//!
//! # Scalability
//!
//! This module imposes no language-level ceilings on declarations,
//! statements, expression sizes, types, generic parameters, or computing
//! resources. Parsing is necessarily bounded by available resources and
//! by configurable budgets enforced by the parser infrastructure.
//!
//! # Error and recovery policy
//!
//! This adapter never silently accepts an unrecognized construct.
//! Grammar failures are propagated without conversion to success.
//! A production that reports success without advancing the cursor is
//! rejected as a parser integration error.
//!
//! Recovery remains the responsibility of the shared parser/recovery layer.
//!
//! # Compatibility
//!
//! Token classification uses canonical `TokenKind` identities, not
//! duplicated keyword strings. Unknown and future token kinds are not
//! automatically interpreted as classical syntax.

use core::fmt;

use crate::frontend::lexer::token_kind::TokenKind;
use crate::frontend::parser::token_cursor::TokenCursor;

/// The syntactic family of a classical construct.
///
/// These variants classify grammar entry points. They do not represent
/// semantic categories, AST node kinds, or execution targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ClassicalConstructKind {
    /// Module declaration.
    Module,

    /// Import or use declaration.
    Import,

    /// Function, binding, aggregate, trait, implementation, or type
    /// declaration.
    Declaration,

    /// A statement, including control flow and loop control.
    Statement,

    /// An expression that may appear in an expression-statement position.
    ExpressionStatement,
}

impl ClassicalConstructKind {
    /// Classify the current token as a possible classical construct start.
    ///
    /// This function is deterministic and has no side effects.
    ///
    /// Classification is intentionally conservative: an expression-start
    /// token indicates only that an expression production may be attempted.
    /// The shared grammar remains responsible for validating the complete
    /// construct and resolving ambiguities.
    ///
    /// Modifiers and ambiguous prefixes must be handled by the enclosing
    /// item grammar, which has access to lookahead and the active language
    /// edition.
    #[must_use]
    pub const fn from_token_kind(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::KeywordModule => Some(Self::Module),

            TokenKind::KeywordImport | TokenKind::KeywordUse => {
                Some(Self::Import)
            }

            TokenKind::KeywordFn
            | TokenKind::KeywordLet
            | TokenKind::KeywordVar
            | TokenKind::KeywordConst
            | TokenKind::KeywordType
            | TokenKind::KeywordStruct
            | TokenKind::KeywordRecord
            | TokenKind::KeywordEnum
            | TokenKind::KeywordClass
            | TokenKind::KeywordTrait
            | TokenKind::KeywordInterface
            | TokenKind::KeywordImpl => Some(Self::Declaration),

            TokenKind::KeywordReturn
            | TokenKind::KeywordIf
            | TokenKind::KeywordElse
            | TokenKind::KeywordFor
            | TokenKind::KeywordWhile
            | TokenKind::KeywordLoop
            | TokenKind::KeywordBreak
            | TokenKind::KeywordContinue
            | TokenKind::KeywordMatch => Some(Self::Statement),

            TokenKind::Identifier
            | TokenKind::IntegerLiteral
            | TokenKind::FloatLiteral
            | TokenKind::StringLiteral
            | TokenKind::CharacterLiteral
            | TokenKind::BooleanLiteral
            | TokenKind::QuantumLiteral
            | TokenKind::DurationLiteral
            | TokenKind::SizeLiteral
            | TokenKind::HardwareLiteral
            | TokenKind::MtsLiteral
            | TokenKind::KeywordTrue
            | TokenKind::KeywordFalse
            | TokenKind::LeftParen
            | TokenKind::LeftBracket
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::LogicalNot
            | TokenKind::BitNot
            | TokenKind::BitAnd
            | TokenKind::Star
            | TokenKind::KeywordInfer => Some(Self::ExpressionStatement),

            // EOF, delimiters, punctuation, operators that cannot begin
            // an expression, invalid tokens, and unrecognized future token
            // kinds must be handled by their owning grammar or diagnostics.
            _ => None,
        }
    }
}

/// A failure raised by classical grammar dispatch.
///
/// `E` is the concrete error type of the shared grammar implementation.
/// This adapter does not erase the original error or depend on a particular
/// AST representation.
#[derive(Debug, PartialEq, Eq)]
pub enum ClassicalDispatchError<E> {
    /// No classical production was selected for the current token.
    UnsupportedStart {
        /// Token identity encountered at the current position.
        token_kind: TokenKind,
    },

    /// The delegated grammar failed.
    Production(E),

    /// A production returned success without consuming input.
    ///
    /// This is an integration invariant failure and must not be treated as
    /// a successful parse.
    NoProgress {
        /// Production family that failed to advance.
        construct: ClassicalConstructKind,

        /// Cursor position before the production was invoked.
        before: usize,

        /// Cursor position after the production returned success.
        after: usize,
    },
}

impl<E: fmt::Display> fmt::Display for ClassicalDispatchError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedStart { token_kind } => {
                write!(
                    f,
                    "no classical production accepts token {token_kind:?}"
                )
            }
            Self::Production(error) => {
                write!(f, "classical grammar production failed: {error}")
            }
            Self::NoProgress {
                construct,
                before,
                after,
            } => {
                write!(
                    f,
                    "classical {construct:?} production made no progress \
                     (cursor position {before} -> {after})"
                )
            }
        }
    }
}

impl<E> std::error::Error for ClassicalDispatchError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Production(error) => Some(error),
            Self::UnsupportedStart { .. } | Self::NoProgress { .. } => None,
        }
    }
}

/// Concrete classical grammar productions supplied by parser integration.
///
/// `Output` must be the existing canonical item or syntax representation.
/// `Error` must be the error type used by the shared grammar integration.
///
/// Implementations should reuse:
/// - `grammar::declarations` for declarations;
/// - `grammar::statements` for statements;
/// - `grammar::expressions` for expressions;
/// - the shared module and import productions;
/// - the canonical AST adapter.
///
/// The methods deliberately receive the same cursor type. This prevents
/// classical parsing from creating a separate token stream or cursor.
///
/// These methods parse syntax only. They must not perform name resolution,
/// type checking, capability evaluation, target selection, or execution.
pub trait ClassicalProductions<'tokens, 'src> {
    /// Canonical representation produced by the shared grammar.
    type Output;

    /// Error type used by the shared grammar.
    type Error;

    /// Parse one module declaration.
    fn parse_module(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, Self::Error>;

    /// Parse one import or use declaration.
    fn parse_import(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, Self::Error>;

    /// Parse one classical declaration using the shared declaration grammar.
    fn parse_declaration(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, Self::Error>;

    /// Parse one statement using the shared statement grammar.
    fn parse_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, Self::Error>;

    /// Parse an expression in a context where expression statements are
    /// permitted by the normative language grammar.
    ///
    /// The implementation must apply the shared expression grammar and
    /// the language's existing expression-statement terminator policy.
    fn parse_expression_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, Self::Error>;
}

/// Parse one classical construct using the shared grammar implementation.
///
/// The caller must invoke this function only at a grammar position where a
/// classical construct is allowed. It does not skip documentation, decide
/// top-level versus block-level placement, or perform error recovery.
///
/// # Progress invariant
///
/// A successful production must advance `TokenCursor::raw_index()`.
/// This catches a faulty handler before it can cause an enclosing parser
/// loop to repeat indefinitely.
///
/// The cursor index includes trivia positions in the underlying token slice.
/// Since the cursor normalizes parser-visible positions, a valid production
/// should advance beyond its starting raw index.
///
/// # Resource safety
///
/// This function does not allocate or buffer tokens. Token accounting and
/// nesting/work budgets remain owned by the parser session and delegated
/// grammar productions. Production implementations must use the configured
/// budget-aware cursor operations where required.
pub fn parse_classical_construct<'tokens, 'src, P>(
    productions: &mut P,
    cursor: &mut TokenCursor<'tokens, 'src>,
) -> Result<P::Output, ClassicalDispatchError<P::Error>>
where
    P: ClassicalProductions<'tokens, 'src>,
{
    let token_kind = cursor.peek().kind();

    let construct = ClassicalConstructKind::from_token_kind(token_kind)
        .ok_or(ClassicalDispatchError::UnsupportedStart { token_kind })?;

    let before = cursor.raw_index();

    let result = match construct {
        ClassicalConstructKind::Module => productions.parse_module(cursor),
        ClassicalConstructKind::Import => productions.parse_import(cursor),
        ClassicalConstructKind::Declaration => {
            productions.parse_declaration(cursor)
        }
        ClassicalConstructKind::Statement => {
            productions.parse_statement(cursor)
        }
        ClassicalConstructKind::ExpressionStatement => {
            productions.parse_expression_statement(cursor)
        }
    }
    .map_err(ClassicalDispatchError::Production)?;

    let after = cursor.raw_index();

    if after <= before {
        return Err(ClassicalDispatchError::NoProgress {
            construct,
            before,
            after,
        });
    }

    Ok(result)
}

/// Return whether a token can begin a construct recognized by this adapter.
///
/// This is a syntactic preflight check, not proof that a complete construct
/// is valid. The delegated grammar must validate the entire production.
#[must_use]
pub const fn can_start_classical_construct(kind: TokenKind) -> bool {
    ClassicalConstructKind::from_token_kind(kind).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_common_declarations() {
        assert_eq!(
            ClassicalConstructKind::from_token_kind(TokenKind::KeywordFn),
            Some(ClassicalConstructKind::Declaration)
        );
        assert_eq!(
            ClassicalConstructKind::from_token_kind(TokenKind::KeywordStruct),
            Some(ClassicalConstructKind::Declaration)
        );
        assert_eq!(
            ClassicalConstructKind::from_token_kind(TokenKind::KeywordConst),
            Some(ClassicalConstructKind::Declaration)
        );
    }

    #[test]
    fn classifies_control_flow_as_statements() {
        for kind in [
            TokenKind::KeywordReturn,
            TokenKind::KeywordIf,
            TokenKind::KeywordFor,
            TokenKind::KeywordWhile,
            TokenKind::KeywordLoop,
            TokenKind::KeywordMatch,
            TokenKind::KeywordBreak,
            TokenKind::KeywordContinue,
        ] {
            assert_eq!(
                ClassicalConstructKind::from_token_kind(kind),
                Some(ClassicalConstructKind::Statement),
                "unexpected classification for {kind:?}"
            );
        }
    }

    #[test]
    fn classifies_module_and_import_entries() {
        assert_eq!(
            ClassicalConstructKind::from_token_kind(TokenKind::KeywordModule),
            Some(ClassicalConstructKind::Module)
        );
        assert_eq!(
            ClassicalConstructKind::from_token_kind(TokenKind::KeywordImport),
            Some(ClassicalConstructKind::Import)
        );
        assert_eq!(
            ClassicalConstructKind::from_token_kind(TokenKind::KeywordUse),
            Some(ClassicalConstructKind::Import)
        );
    }

    #[test]
    fn classifies_expression_starts_without_resolving_meaning() {
        for kind in [
            TokenKind::Identifier,
            TokenKind::IntegerLiteral,
            TokenKind::StringLiteral,
            TokenKind::LeftParen,
            TokenKind::LeftBracket,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::LogicalNot,
        ] {
            assert_eq!(
                ClassicalConstructKind::from_token_kind(kind),
                Some(ClassicalConstructKind::ExpressionStatement),
                "unexpected classification for {kind:?}"
            );
        }
    }

    #[test]
    fn rejects_non_entry_tokens() {
        for kind in [
            TokenKind::Eof,
            TokenKind::Semicolon,
            TokenKind::RightParen,
            TokenKind::Comma,
            TokenKind::Invalid,
        ] {
            assert_eq!(
                ClassicalConstructKind::from_token_kind(kind),
                None,
                "unexpected classification for {kind:?}"
            );
        }
    }

    #[test]
    fn preflight_matches_the_classifier() {
        assert!(can_start_classical_construct(TokenKind::KeywordFn));
        assert!(can_start_classical_construct(TokenKind::Identifier));
        assert!(!can_start_classical_construct(TokenKind::Eof));
        assert!(!can_start_classical_construct(TokenKind::Invalid));
    }
}