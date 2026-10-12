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
//! Provide a single dispatch boundary for top-level source elements.
//!
//! # Ownership
//!
//! This module owns source-item classification and dispatch only.
//! Concrete grammar productions and canonical AST construction remain with
//! their owning grammar modules and AST integration layer.
//!
//! # Integration
//!
//! Uses the canonical Token, TokenKind, TokenChannel, Span, TokenCursor,
//! ParserError, and ParserErrorKind types.
//!
//! The grammar driver implements ItemGrammar. program.rs delegates each
//! non-EOF source element to parse_item through its ProgramDriver adapter.
//!
//! # Scalability
//!
//! No hardware-capacity ceilings or fixed domain-operation catalogs are
//! defined here. Resource consumption is governed by the available resources
//! and the parser's configured implementation budgets.
//!
//! # Safety
//!
//! This module uses safe Rust only.

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;

use super::super::parser::{ParserError, ParserErrorKind, TokenCursor};

/// Classification used to route a source element to its owning grammar.
///
/// This enum describes syntax categories, not AST node kinds or semantic
/// computing domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ItemCategory {
    Documentation,
    Attribute,
    Pragma,
    Package,
    Module,
    Import,
    Export,
    Declaration,
    Statement,
    Domain,
    Extension,
}

/// Concrete grammar interface used by the universal item dispatcher.
///
/// Every successful handler must consume at least one non-trivia token and
/// return the canonical AST item type used by the parser integration.
///
/// Recognition methods must be deterministic and must not consume tokens.
pub trait ItemGrammar<'tokens, 'src> {
    /// Canonical AST representation of one source element.
    type Item;

    /// Parse a documentation element.
    fn parse_documentation(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse an attribute or annotation.
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
    ///
    /// Package recognition is delegated to the grammar driver because the
    /// inspected canonical TokenKind vocabulary has no dedicated package
    /// token. A registered dialect may provide this syntax explicitly.
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

    /// Parse a universal declaration.
    fn parse_declaration(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse a statement or permitted expression statement.
    fn parse_statement(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Parse a recognized computing-domain element.
    fn parse_domain(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;

    /// Check whether the active grammar recognizes a package declaration.
    ///
    /// The default is false. Implementations may consult an immutable,
    /// versioned dialect or extension registry.
    fn is_package_item_start(&self, _token: &Token<'src>) -> bool {
        false
    }

    /// Check whether a registered extension recognizes this token.
    ///
    /// Must not mutate parser state or consume input.
    fn can_parse_extension_item(&self, token: &Token<'src>) -> bool;

    /// Parse an item recognized by a registered extension.
    fn parse_extension_item(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Item, ParserError>;
}

/// Parse exactly one top-level source element.
///
/// Trivia is skipped first. Documentation tokens are preserved for the
/// documentation handler. EOF is a source-unit terminator, never an item.
///
/// The core grammar owns recognized core syntax. Package and extension
/// recognition is consulted only when no core production matches.
///
/// # Errors
///
/// Returns a structured error for invalid cursor state, unexpected EOF,
/// unrecognized syntax, handler failures, or a handler that succeeds without
/// advancing the cursor.
pub fn parse_item<'tokens, 'src, G>(
    grammar: &mut G,
    cursor: &mut TokenCursor<'tokens, 'src>,
) -> Result<G::Item, ParserError>
where
    G: ItemGrammar<'tokens, 'src>,
{
    skip_trivia(cursor)?;

    let before = cursor.position();

    // Finish all immutable recognition before passing the cursor mutably to
    // a handler. This avoids borrowing a token across a mutable cursor call.
    let (category, is_package, is_extension) = {
        let token = cursor.current().ok_or_else(|| {
            ParserError::new(
                ParserErrorKind::InvalidParserState,
                empty_span(),
                "cannot parse a source item: cursor has no current token",
            )
        })?;

        if token.kind() == TokenKind::Eof {
            return Err(ParserError::at_token(
                ParserErrorKind::UnexpectedToken,
                token,
                "EOF terminates the source unit and cannot begin an item",
            ));
        }

        let category = classify_item(token);

        let is_package = category.is_none()
            && grammar.is_package_item_start(token);

        let is_extension = category.is_none()
            && !is_package
            && grammar.can_parse_extension_item(token);

        (category, is_package, is_extension)
    };

    let item = match category {
        Some(ItemCategory::Documentation) => {
            grammar.parse_documentation(cursor)?
        }
        Some(ItemCategory::Attribute) => grammar.parse_attribute(cursor)?,
        Some(ItemCategory::Pragma) => grammar.parse_pragma(cursor)?,
        Some(ItemCategory::Module) => grammar.parse_module(cursor)?,
        Some(ItemCategory::Import) => grammar.parse_import(cursor)?,
        Some(ItemCategory::Export) => grammar.parse_export(cursor)?,
        Some(ItemCategory::Declaration) => grammar.parse_declaration(cursor)?,
        Some(ItemCategory::Statement) => grammar.parse_statement(cursor)?,
        Some(ItemCategory::Domain) => grammar.parse_domain(cursor)?,

        // These variants remain available for future explicit classifications.
        Some(ItemCategory::Package) => grammar.parse_package(cursor)?,
        Some(ItemCategory::Extension) => {
            if !is_extension {
                return Err(unexpected_item(cursor));
            }
            grammar.parse_extension_item(cursor)?
        }

        None if is_package => grammar.parse_package(cursor)?,
        None if is_extension => grammar.parse_extension_item(cursor)?,
        None => return Err(unexpected_item(cursor)),
    };

    let after = cursor.position();

    if after <= before {
        return Err(ParserError::new(
            ParserErrorKind::NoProgress,
            cursor
                .current_span()
                .unwrap_or_else(|_| empty_span()),
            format!(
                "item grammar returned success without consuming input \
                 (token position {before})"
            ),
        ));
    }

    Ok(item)
}

/// Classify core source-element starting tokens.
///
/// This table describes shared syntax categories, not individual operations,
/// device names, vendor names, or resource capacities. Future syntax should
/// use the existing generic grammar productions or explicit extensions.
fn classify_item(token: &Token<'_>) -> Option<ItemCategory> {
    if token.channel() == TokenChannel::Documentation {
        return Some(ItemCategory::Documentation);
    }

    match token.kind() {
        // Annotations are identified by the canonical lexer.
        TokenKind::Annotation
        | TokenKind::NanoAnnotation
        | TokenKind::At => Some(ItemCategory::Attribute),

        // Module composition.
        TokenKind::KeywordModule => Some(ItemCategory::Module),
        TokenKind::KeywordImport | TokenKind::KeywordUse => {
            Some(ItemCategory::Import)
        }
        TokenKind::KeywordExport => Some(ItemCategory::Export),

        // Shared declaration families.
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
        | TokenKind::KeywordOverride => Some(ItemCategory::Declaration),

        // Known domain-level entry points. Individual operations are not
        // enumerated here; they belong to grammar rules and metadata.
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

        // Statements and control-flow constructs.
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

        // Expression statements, when permitted by the active grammar.
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

        // Directive introducer. The pragma grammar validates its contents.
        TokenKind::Hash => Some(ItemCategory::Pragma),

        // Unknown syntax is not accepted by default. A registered extension
        // may explicitly claim it in parse_item.
        _ => None,
    }
}

/// Return an unexpected-item diagnostic using the current canonical token.
fn unexpected_item(
    cursor: &TokenCursor<'_, '_>,
) -> ParserError {
    match cursor.current() {
        Some(token) => ParserError::at_token(
            ParserErrorKind::UnexpectedToken,
            token,
            format!(
                "no core grammar production or registered extension \
                 recognizes {:?} ({:?}) as a top-level source element",
                token.kind(),
                token.lexeme(),
            ),
        ),
        None => ParserError::new(
            ParserErrorKind::InvalidParserState,
            empty_span(),
            "cannot report an unexpected item: cursor has no current token",
        ),
    }
}

/// Skip parser-irrelevant trivia.
///
/// Documentation is preserved even when the lexer has a dedicated
/// documentation channel. The lexer remains the sole authority for token
/// construction and token-channel assignment.
fn skip_trivia<'tokens, 'src>(
    cursor: &mut TokenCursor<'tokens, 'src>,
) -> Result<(), ParserError> {
    loop {
        let Some(token) = cursor.current() else {
            return Err(ParserError::new(
                ParserErrorKind::InvalidParserState,
                empty_span(),
                "cursor has no current token while skipping trivia",
            ));
        };

        if token.kind() == TokenKind::Eof
            || token.channel() == TokenChannel::Documentation
        {
            return Ok(());
        }

        let trivia = token.channel() == TokenChannel::Trivia
            || matches!(
                token.kind(),
                TokenKind::Whitespace
                    | TokenKind::Newline
                    | TokenKind::LineComment
                    | TokenKind::BlockComment
            );

        if !trivia {
            return Ok(());
        }

        cursor.advance()?;
    }
}

/// Construct a neutral span for a parser-state error without a usable token.
///
/// Uses the repository's canonical span and cursor-position constructors.
fn empty_span() -> Span {
    use crate::frontend::lexer::cursor::CursorPosition;

    Span::at(CursorPosition::new(0, 1, 1))
}