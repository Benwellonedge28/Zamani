
#![forbid(unsafe_code)]

//! Zamani frontend — declaration grammar dispatcher.
//!
//! File: src/frontend/parser/grammar/declarations.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Own the single dispatch boundary for source-level declarations.
//!
//! This module recognizes declaration-family starting tokens and delegates
//! concrete syntax parsing to the corresponding production implementation.
//! It does not define a second AST, tokenize source, resolve names, or
//! interpret declarations for a particular computing target.
//!
//! # Ownership
//!
//! This module owns:
//! - declaration-family classification;
//! - declaration-start recognition;
//! - deterministic dispatch to concrete productions;
//! - the declaration-production integration contract;
//! - successful-production progress checks;
//! - structured errors for unsupported declaration starts.
//!
//! It does not own:
//! - tokenization, keyword spelling, or source spans;
//! - concrete function, variable, type, aggregate, or implementation syntax;
//! - expression, type, generic, path, attribute, or block parsing;
//! - canonical AST definitions or AST storage;
//! - name resolution, type checking, effects, capabilities, or contracts;
//! - resource negotiation, hardware discovery, scheduling, or lowering.
//!
//! # Integration
//!
//! This module consumes the canonical lexer and parser contracts:
//!
//! - `lexer::token_kind::TokenKind`;
//! - `parser::parser::TokenCursor`;
//! - `parser::parser::ParserError`;
//! - `parser::parser::ParserErrorKind`.
//!
//! The parser/session integration implements `DeclarationProductions`
//! using the canonical AST adapter and the shared grammar productions.
//! The universal item grammar delegates declaration items to
//! `parse_declaration`.
//!
//! # Scalability
//!
//! No hardware capacities, declaration counts, parameter counts, qubit
//! counts, tensor ranks, device counts, or domain-specific operation
//! catalogs are fixed here. Input processing remains subject to available
//! resources and configurable parser budgets.
//!
//! # Extension policy
//!
//! A new declaration family may be provided by a registered grammar
//! extension. An extension must not silently reinterpret a declaration
//! already owned by the core grammar. Extension recognition must be
//! deterministic and must not consume input.
//!
//! # Completion contract
//!
//! A successful production must consume at least one token. Productions
//! must preserve source information and return the canonical AST item
//! representation expected by the parser integration. Semantic validation
//! remains downstream.

use crate::frontend::lexer::token_kind::TokenKind;

use super::super::parser::{ParserError, ParserErrorKind, TokenCursor};

/// Core declaration families recognized by the canonical lexer.
///
/// This enum groups syntax families, not individual language keywords
/// within every possible dialect and not semantic computing domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeclarationKind {
    /// `fn` declarations.
    Function,

    /// `let` and `var` bindings.
    Variable,

    /// `const` bindings.
    Constant,

    /// `type` aliases or declarations.
    Type,

    /// `struct` declarations.
    Struct,

    /// `record` declarations.
    Record,

    /// `enum` declarations.
    Enum,

    /// `class` declarations.
    Class,

    /// `trait` declarations.
    Trait,

    /// `interface` declarations.
    Interface,

    /// `impl` declarations.
    Implementation,

    /// A declaration owned by an enabled extension.
    Extension,
}

impl DeclarationKind {
    /// Classify a declaration's first token.
    ///
    /// Returns `None` when the token is not a core declaration start.
    /// This function is pure and does not depend on registry order.
    #[must_use]
    pub const fn from_token_kind(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::KeywordFn => Some(Self::Function),

            TokenKind::KeywordLet | TokenKind::KeywordVar => {
                Some(Self::Variable)
            }

            TokenKind::KeywordConst => Some(Self::Constant),
            TokenKind::KeywordType => Some(Self::Type),
            TokenKind::KeywordStruct => Some(Self::Struct),
            TokenKind::KeywordRecord => Some(Self::Record),
            TokenKind::KeywordEnum => Some(Self::Enum),
            TokenKind::KeywordClass => Some(Self::Class),
            TokenKind::KeywordTrait => Some(Self::Trait),
            TokenKind::KeywordInterface => Some(Self::Interface),
            TokenKind::KeywordImpl => Some(Self::Implementation),

            _ => None,
        }
    }

    /// Return whether this declaration family is implemented by a core
    /// production rather than by an extension.
    #[must_use]
    pub const fn is_core(self) -> bool {
        !matches!(self, Self::Extension)
    }
}

/// Return whether `kind` starts a core declaration.
///
/// This predicate is intentionally independent of the active dialect.
/// Extension-defined declaration starts are recognized through the
/// `DeclarationProductions` extension hook instead.
#[must_use]
pub const fn is_declaration_start(kind: TokenKind) -> bool {
    DeclarationKind::from_token_kind(kind).is_some()
}

/// Concrete declaration parsers supplied by the parser integration.
///
/// `Output` must be the canonical item representation expected by the
/// universal item grammar. It may be an AST item reference, an AST node
/// identifier wrapped in the repository's item type, or an established
/// parser intermediate representation that is converted by `ast_adapter`.
///
/// Implementations should delegate to the existing shared productions for
/// identifiers, paths, types, generics, attributes, expressions, and blocks.
/// They must not duplicate those grammars or construct a competing AST.
///
/// Recognition methods must be deterministic and must not consume tokens.
pub trait DeclarationProductions<'tokens, 'src> {
    /// Canonical representation returned for a successfully parsed item.
    type Output;

    /// Parse a function declaration.
    fn parse_function(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Parse a variable binding (`let` or `var`).
    fn parse_variable(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Parse a constant declaration.
    fn parse_constant(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Parse a type declaration or type alias.
    fn parse_type(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Parse a structure declaration.
    fn parse_struct(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Parse a record declaration.
    fn parse_record(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Parse an enumeration declaration.
    fn parse_enum(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Parse a class declaration.
    fn parse_class(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Parse a trait declaration.
    fn parse_trait(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Parse an interface declaration.
    fn parse_interface(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Parse an implementation declaration.
    fn parse_implementation(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;

    /// Return whether an enabled extension owns the given declaration start.
    ///
    /// This method must be side-effect-free. Core declaration starts always
    /// take precedence, so an extension cannot shadow a core declaration.
    fn recognizes_extension(&self, _kind: TokenKind) -> bool {
        false
    }

    /// Parse an extension-defined declaration.
    ///
    /// Called only when no core production recognizes the current token and
    /// `recognizes_extension` returned true.
    fn parse_extension(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError>;
}

/// Parse exactly one declaration using the core productions or an explicitly
/// registered extension.
///
/// The caller is responsible for ensuring that the current grammar position
/// permits a declaration. This function does not skip documentation or
/// trivia; trivia handling belongs to the canonical cursor and item layer.
///
/// # Errors
///
/// Returns a structured parser error when:
///
/// - the cursor has no current token;
/// - the current token cannot begin a core or registered declaration;
/// - the selected production fails;
/// - a production returns success without advancing the token cursor.
///
/// # Progress guarantee
///
/// A production that returns `Ok` must advance the cursor position. This
/// prevents a malformed production from causing an infinite item-parsing
/// loop. Token consumption and recovery work should be charged to the active
/// parser budget by the production/session integration.
pub fn parse_declaration<'tokens, 'src, P>(
    productions: &mut P,
    cursor: &mut TokenCursor<'tokens, 'src>,
) -> Result<P::Output, ParserError>
where
    P: DeclarationProductions<'tokens, 'src>,
{
    let (kind, before) = {
        let token = cursor.current().ok_or_else(|| {
            ParserError::new(
                ParserErrorKind::InvalidParserState,
                empty_span(),
                "cannot parse a declaration: cursor has no current token",
            )
        })?;

        (token.kind(), cursor.position())
    };

    // Core syntax always wins over extension dispatch. In particular,
    // registration order cannot change the meaning of a core declaration.
    let core_kind = DeclarationKind::from_token_kind(kind);

    let selected = match core_kind {
        Some(DeclarationKind::Function) => {
            productions.parse_function(cursor)?
        }
        Some(DeclarationKind::Variable) => {
            productions.parse_variable(cursor)?
        }
        Some(DeclarationKind::Constant) => {
            productions.parse_constant(cursor)?
        }
        Some(DeclarationKind::Type) => productions.parse_type(cursor)?,
        Some(DeclarationKind::Struct) => {
            productions.parse_struct(cursor)?
        }
        Some(DeclarationKind::Record) => {
            productions.parse_record(cursor)?
        }
        Some(DeclarationKind::Enum) => productions.parse_enum(cursor)?,
        Some(DeclarationKind::Class) => {
            productions.parse_class(cursor)?
        }
        Some(DeclarationKind::Trait) => {
            productions.parse_trait(cursor)?
        }
        Some(DeclarationKind::Interface) => {
            productions.parse_interface(cursor)?
        }
        Some(DeclarationKind::Implementation) => {
            productions.parse_implementation(cursor)?
        }
        Some(DeclarationKind::Extension) | None => {
            if productions.recognizes_extension(kind) {
                productions.parse_extension(cursor)?
            } else {
                return Err(unexpected_declaration(cursor));
            }
        }
    };

    let after = cursor.position();

    if after <= before {
        return Err(ParserError::new(
            ParserErrorKind::NoProgress,
            cursor
                .current_span()
                .unwrap_or_else(|_| empty_span()),
            format!(
                "declaration production returned success without consuming \
                 input (token position {before})"
            ),
        ));
    }

    Ok(selected)
}

/// Construct an error for a token that cannot begin a declaration.
fn unexpected_declaration<'tokens, 'src>(
    cursor: &TokenCursor<'tokens, 'src>,
) -> ParserError {
    match cursor.current() {
        Some(token) => ParserError::at_token(
            ParserErrorKind::UnexpectedToken,
            token,
            format!(
                "expected a declaration, found token kind {:?}",
                token.kind()
            ),
        ),
        None => ParserError::new(
            ParserErrorKind::InvalidParserState,
            empty_span(),
            "cannot parse a declaration: cursor has no current token",
        ),
    }
}

/// Construct the canonical zero-width fallback span.
///
/// The normal path uses the current token's span. This fallback is reserved
/// for invalid cursor states in which no token location is available.
fn empty_span() -> crate::frontend::lexer::span::Span {
    crate::frontend::lexer::span::Span::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_declaration_starts_are_classified() {
        let cases = [
            (TokenKind::KeywordFn, DeclarationKind::Function),
            (TokenKind::KeywordLet, DeclarationKind::Variable),
            (TokenKind::KeywordVar, DeclarationKind::Variable),
            (TokenKind::KeywordConst, DeclarationKind::Constant),
            (TokenKind::KeywordType, DeclarationKind::Type),
            (TokenKind::KeywordStruct, DeclarationKind::Struct),
            (TokenKind::KeywordRecord, DeclarationKind::Record),
            (TokenKind::KeywordEnum, DeclarationKind::Enum),
            (TokenKind::KeywordClass, DeclarationKind::Class),
            (TokenKind::KeywordTrait, DeclarationKind::Trait),
            (
                TokenKind::KeywordInterface,
                DeclarationKind::Interface,
            ),
            (TokenKind::KeywordImpl, DeclarationKind::Implementation),
        ];

        for (token, expected) in cases {
            assert_eq!(
                DeclarationKind::from_token_kind(token),
                Some(expected),
                "incorrect classification for {token:?}"
            );

            assert!(is_declaration_start(token));
        }
    }

    #[test]
    fn non_declaration_tokens_are_not_core_declaration_starts() {
        let cases = [
            TokenKind::Identifier,
            TokenKind::IntegerLiteral,
            TokenKind::LeftBrace,
            TokenKind::Semicolon,
            TokenKind::KeywordIf,
            TokenKind::KeywordReturn,
            TokenKind::Eof,
        ];

        for token in cases {
            assert_eq!(
                DeclarationKind::from_token_kind(token),
                None,
                "{token:?} must not be classified as a core declaration"
            );

            assert!(!is_declaration_start(token));
        }
    }

    #[test]
    fn core_kinds_are_not_extensions() {
        let kinds = [
            DeclarationKind::Function,
            DeclarationKind::Variable,
            DeclarationKind::Constant,
            DeclarationKind::Type,
            DeclarationKind::Struct,
            DeclarationKind::Record,
            DeclarationKind::Enum,
            DeclarationKind::Class,
            DeclarationKind::Trait,
            DeclarationKind::Interface,
            DeclarationKind::Implementation,
        ];

        for kind in kinds {
            assert!(kind.is_core());
        }
    }

    #[test]
    fn extension_kind_is_not_a_core_kind() {
        assert!(!DeclarationKind::Extension.is_core());
    }
}
