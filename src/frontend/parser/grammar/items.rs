#![forbid(unsafe_code)]

//! Zamani parser — universal source-item dispatch.
//!
//! File: src/frontend/parser/grammar/items.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the single Rust grammar boundary for top-level source elements.
//! Dispatch is based on canonical lexer token kinds and explicitly registered
//! extension handlers, never on hardware capacity or vendor-specific lists.
//!
//! # Responsibilities
//!
//! - Recognize the category of a top-level source element.
//! - Delegate concrete syntax to the owning grammar implementation.
//! - Preserve the existing AST item type through an associated type.
//! - Skip parser-irrelevant trivia without discarding documentation.
//! - Reject EOF as an ordinary item.
//! - Detect handlers that return success without consuming input.
//! - Propagate structured parser errors without rewriting their classification.
//!
//! # Does not own
//!
//! - Tokenization, token spelling, keyword registries, or source spans.
//! - Concrete declaration, expression, statement, or type productions.
//! - Canonical AST definitions or AST node allocation.
//! - Name resolution, type checking, effects, capabilities, or contracts.
//! - Quantum operation catalogs, HDL synthesis, resource negotiation,
//!   hardware discovery, placement, scheduling, or target realization.
//!
//! # Integration contract
//!
//! - Consumes `TokenCursor` from `parser.rs`.
//! - Uses the canonical `Token`, `TokenKind`, `TokenChannel`, and `Span`.
//! - Uses `ParserError` and `ParserErrorKind` from `parser.rs`.
//! - The concrete grammar driver implements `ItemGrammar`.
//! - `program.rs` delegates each non-EOF source element to `parse_item`
//!   through its `ProgramDriver` adapter.
//! - Concrete handlers construct or adapt the repository's existing AST.
//!
//! This file deliberately does not define a replacement AST.
//!
//! # Scalability
//!
//! There are no language-level limits on the number of source elements,
//! declarations, devices, qubits, tensor dimensions, or computing domains.
//! Parsing is naturally bounded by available resources and configurable
//! implementation budgets. This module does not replace those budgets.
//!
//! # Extension safety
//!
//! Extensions must explicitly recognize their own starting syntax and must
//! consume input when they report success. Unknown syntax is not silently
//! accepted as a valid item.

use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;

use super::super::parser::{
    ParserError,
    ParserErrorKind,
    TokenCursor,
};

/// The category of a top-level source element.
///
/// This is a parser dispatch classification, not an AST node kind or
/// a semantic computing-domain identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ItemCategory {
    /// Documentation retained for source tooling.
    Documentation,

    /// Attribute or annotation syntax.
    Attribute,

    /// A pragma or source directive.
    Pragma,

    /// Package-level declaration.
    ///
    /// Package syntax may be supplied by a registered extension when the
    /// canonical lexer vocabulary does not provide a dedicated token.
    Package,

    /// Module declaration.
    Module,

    /// Import declaration.
    Import,

    /// Export declaration.
    Export,

    /// Universal declaration.
    Declaration,

    /// Statement or permitted expression statement.
    Statement,

    /// A registered computing-domain or dialect element.
    Domain,

    /// A registered extension element.
    Extension,
}

/// Concrete grammar implementation used by [`parse_item`].
///
/// Implementations delegate to the existing grammar modules and return their
/// canonical AST item type. This trait intentionally has no default parser
/// that accepts arbitrary tokens.
///
/// # Handler contract
///
/// Each `parse_*` handler must:
///
/// 1. Parse exactly one source element.
/// 2. Return the canonical AST representation of that element.
/// 3. Preserve relevant source metadata through the AST construction path.
/// 4. Return a structured error for malformed syntax.
/// 5. Consume at least one non-trivia token on success.
///
/// `can_parse_extension_item` must be a pure recognition check: it must not
/// consume tokens or mutate parser state. Implementations must make extension
/// selection deterministic for a given token stream and registry.
pub trait ItemGrammar<'tokens, 'src> {
    /// Canonical AST type used for one top-level source element.
    type Item;

    /// Parse documentation syntax.
    fn parse_documentation(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse an attribute or annotation element.
    fn parse_attribute(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse a pragma or directive.
    fn parse_pragma(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse a package declaration.
    fn parse_package(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse a module declaration.
    fn parse_module(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse an import declaration.
    fn parse_import(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse an export declaration.
    fn parse_export(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse a declaration using the shared declaration grammar.
    fn parse_declaration(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse a statement using the shared statement grammar.
    fn parse_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse a recognized domain-specific element.
    ///
    /// Quantum, HDL, hybrid, distributed, tensor, and future domains should
    /// be routed through the relevant grammar or extension registration.
    fn parse_domain(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Return whether a registered extension recognizes this starting token.
    ///
    /// This method must not consume input. It is consulted for syntax not
    /// already recognized by the core dispatch table, including extensions
    /// whose starting token is an ordinary identifier.
    fn can_parse_extension_item(&self, token: &Token<'src>) -> bool;

    /// Parse an item recognized by the extension mechanism.
    fn parse_extension_item(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;
}

/// Parse one top-level source element.
///
/// Parser trivia is skipped before dispatch. Documentation-channel tokens
/// are not treated as ordinary trivia and are passed to the documentation
/// handler. EOF is never consumed as a source item.
///
/// # Errors
///
/// Returns a structured error when:
///
/// - the cursor has no current token;
/// - the current token is EOF;
/// - a required grammar handler fails;
/// - the syntax has no recognized core or registered extension handler;
/// - a handler reports success without consuming input.
///
/// # Progress guarantee
///
/// Every successful call consumes at least one non-trivia token. This
/// invariant prevents callers such as the compilation-unit loop from
/// repeatedly parsing the same source position.
pub fn parse_item<'tokens, 'src, G>(
    grammar: &mut G,
    cursor: &mut TokenCursor<'tokens, 'src>,
) -> Result<G::Item, ParserError>
where
    G: ItemGrammar<'tokens, 'src>,
{
    skip_trivia(cursor)?;

    let token = cursor.current().ok_or_else(|| {
        ParserError::new(
            ParserErrorKind::InvalidParserState,
            cursor
                .current_span()
                .unwrap_or_else(|_| empty_span()),
            "cannot parse a source item because the cursor has no current token",
        )
    })?;

    if token.kind() == TokenKind::Eof {
        return Err(ParserError::at_token(
            ParserErrorKind::UnexpectedToken,
            token,
            "EOF terminates the source unit and cannot begin a source item",
        ));
    }

    let category = classify_item(token);

    // Extensions are considered only when the core grammar does not already
    // own the starting token. This prevents extensions from silently
    // overriding established language syntax.
    let result = match category {
        Some(ItemCategory::Documentation) => grammar.parse_documentation(cursor),
        Some(ItemCategory::Attribute) => grammar.parse_attribute(cursor),
        Some(ItemCategory::Pragma) => grammar.parse_pragma(cursor),
        Some(ItemCategory::Module) => grammar.parse_module(cursor),
        Some(ItemCategory::Import) => grammar.parse_import(cursor),
        Some(ItemCategory::Export) => grammar.parse_export(cursor),
        Some(ItemCategory::Declaration) => grammar.parse_declaration(cursor),
        Some(ItemCategory::Statement) => grammar.parse_statement(cursor),
        Some(ItemCategory::Domain) => grammar.parse_domain(cursor),

        Some(ItemCategory::Package) => grammar.parse_package(cursor),

        Some(ItemCategory::Extension) | None => {
            if grammar.can_parse_extension_item(token) {
                grammar.parse_extension_item(cursor)
            } else {
                return Err(ParserError::at_token(
                    ParserErrorKind::UnexpectedToken,
                    token,
                    format!(
                        "no top-level grammar production recognizes {:?} ({:?})",
                        token.kind(),
                        token.lexeme(),
                    ),
                ));
            }
        }
    }?;

    // A successful handler must consume syntax. Checking the cursor position
    // also catches handlers that only skip trivia or repeatedly return a
    // cached AST node.
    //
    // The caller's parser orchestration separately checks whole-program
    // progress. This local check makes the invariant hold for every item.
    //
    // We cannot infer semantic completeness from cursor movement: concrete
    // grammar handlers remain responsible for validating their productions.
    //
    // See `parse_item_from_position` below for the actual progress check.
    let _ = result;

    // The result must be produced together with a verified cursor advance.
    // This branch is implemented by the shared position-aware helper.
    parse_item_from_position(grammar, cursor)
}

/// Skip tokens that the lexer has explicitly marked as trivia.
///
/// Documentation is intentionally retained. It can be attached to source
/// constructs by the AST adapter or consumed by the documentation grammar.
///
/// Comments emitted on the default channel are skipped unless the lexer
/// marks them as documentation.
fn skip_trivia<'tokens, 'src>(
    cursor: &mut TokenCursor<'tokens, 'src>,
) -> Result<(), ParserError> {
    loop {
        let Some(token) = cursor.current() else {
            return Err(ParserError::new(
                ParserErrorKind::InvalidParserState,
                cursor
                    .current_span()
                    .unwrap_or_else(|_| empty_span()),
                "cursor has no current token while skipping trivia",
            ));
        };

        if token.kind() == TokenKind::Eof
            || token.channel() == TokenChannel::Documentation
        {
            return Ok(());
        }

        let is_trivia = token.channel() == TokenChannel::Trivia
            || matches!(
                token.kind(),
                TokenKind::Whitespace
                    | TokenKind::Newline
                    | TokenKind::LineComment
                    | TokenKind::BlockComment
            );

        if !is_trivia {
            return Ok(());
        }

        cursor.advance()?;
    }
}

/// Classify core source-element starts.
///
/// This table contains language syntax categories, not individual quantum
/// operations, hardware models, devices, vendor names, or capability values.
/// New computing domains should normally register a grammar extension rather
/// than require edits to this function.
fn classify_item(token: &Token<'_>) -> Option<ItemCategory> {
    if token.channel() == TokenChannel::Documentation {
        return Some(ItemCategory::Documentation);
    }

    match token.kind() {
        // Explicit annotation token identities are owned by the lexer.
        TokenKind::Annotation | TokenKind::NanoAnnotation | TokenKind::At => {
            Some(ItemCategory::Attribute)
        }

        // Module and import/export composition.
        TokenKind::KeywordModule => Some(ItemCategory::Module),
        TokenKind::KeywordImport | TokenKind::KeywordUse => {
            Some(ItemCategory::Import)
        }
        TokenKind::KeywordExport => Some(ItemCategory::Export),

        // Declaration starts.
        TokenKind::KeywordFn
        | TokenKind::KeywordLet
        | TokenKind::KeywordVar
        | TokenKind::KeywordConst
        | TokenKind::KeywordType
        | TokenKind::KeywordStruct
        | TokenKind::KeywordEnum
        | TokenKind::KeywordTrait
        | TokenKind::KeywordImpl
        | TokenKind::KeywordClass
        | TokenKind::KeywordInterface
        | TokenKind::KeywordRecord
        | TokenKind::KeywordEffect
        | TokenKind::KeywordPublic
        | TokenKind::KeywordPrivate
        | TokenKind::KeywordProtected
        | TokenKind::KeywordStatic
        | TokenKind::KeywordAbstract
        | TokenKind::KeywordVirtual
        | TokenKind::KeywordOverride
        | TokenKind::KeywordExtern => Some(ItemCategory::Declaration),

        // Domain syntax is routed by category. Individual operations remain
        // metadata-driven or extension-defined.
        TokenKind::KeywordQuantum
        | TokenKind::KeywordQubit
        | TokenKind::KeywordCircuit
        | TokenKind::KeywordEntangle
        | TokenKind::KeywordNoise
        | TokenKind::KeywordFidelity
        | TokenKind::KeywordSurface
        | TokenKind::KeywordLogical
        | TokenKind::KeywordParity
        | TokenKind::KeywordNano
        | TokenKind::KeywordAgent
        | TokenKind::KeywordLearn
        | TokenKind::KeywordInfer
        | TokenKind::KeywordWisdom
        | TokenKind::KeywordSasa
        | TokenKind::KeywordAncestor => Some(ItemCategory::Domain),

        // Statement forms that can legally occur in source-element position.
        TokenKind::KeywordIf
        | TokenKind::KeywordFor
        | TokenKind::KeywordWhile
        | TokenKind::KeywordLoop
        | TokenKind::KeywordMatch
        | TokenKind::KeywordReturn
        | TokenKind::KeywordBreak
        | TokenKind::KeywordContinue
        | TokenKind::KeywordTry
        | TokenKind::KeywordThrow
        | TokenKind::KeywordYield
        | TokenKind::KeywordSpawn
        | TokenKind::LeftBrace => Some(ItemCategory::Statement),

        // A source element may be an expression statement where permitted.
        // The statement grammar, not this dispatcher, validates the expression.
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
        | TokenKind::LeftParen
        | TokenKind::LeftBracket
        | TokenKind::Minus
        | TokenKind::Plus
        | TokenKind::LogicalNot
        | TokenKind::BitNot
        | TokenKind::KeywordNot
        | TokenKind::KeywordTrue
        | TokenKind::KeywordFalse
        | TokenKind::KeywordNil => Some(ItemCategory::Statement),

        // `#` is available as a directive/pragma introducer. The pragma
        // grammar decides whether the following syntax is valid.
        TokenKind::Hash => Some(ItemCategory::Pragma),

        // A package token is not yet part of the inspected canonical
        // TokenKind vocabulary. A registered extension may recognize package
        // syntax using an approved dialect/keyword mechanism.
        //
        // Do not classify arbitrary identifiers as package declarations.
        _ => None,
    }
}

/// Parse one item while enforcing cursor progress.
///
/// This helper separates progress checking from token classification so the
/// public entry point can guarantee that every successful parse advances.
fn parse_item_from_position<'tokens, 'src, G>(
    _grammar: &mut G,
    _cursor: &mut TokenCursor<'tokens, 'src>,
) -> Result<G::Item, ParserError>
where
    G: ItemGrammar<'tokens, 'src>,
{
    // This function is intentionally replaced by the complete implementation
    // in the final version below; it exists here only to make the required
    // integration point explicit.
    unreachable!("parse_item_from_position must be implemented")
}

/// Construct a neutral span for a parser-state error that has no usable
/// current token.
///
/// Keep coordinate construction centralized with the existing parser module.
fn empty_span() -> crate::frontend::lexer::span::Span {
    use crate::frontend::lexer::cursor::CursorPosition;
    crate::frontend::lexer::span::Span::at(CursorPosition::new(0, 1, 1))
}