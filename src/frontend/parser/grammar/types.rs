
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani parser — source-level type grammar.
//!
//! # Ownership
//!
//! This module parses type syntax into the canonical
//! `frontend::ast::node::types::type_expr::TypeExpr` representation.
//!
//! It does not own tokenization, name resolution, type inference, semantic
//! validation, target selection, hardware allocation, or IR lowering.
//!
//! # Integration contract
//!
//! The shared parser cursor implements `TypeTokenCursor` using the canonical
//! lexer token stream. In particular, `consume_type_closer` must consume one
//! closing angle bracket in type context, including when the lexer represents
//! nested `>>` as a single `RightShift` token. It must preserve any remaining
//! closing bracket for the enclosing type parser.
//!
//! The grammar entry point is `parse_type`. It parses exactly one type
//! expression and leaves the first token after that expression unconsumed.
//! The caller owns separator handling and complete-input enforcement.
//!
//! # Resource safety
//!
//! Type nesting is controlled by `TypeParseOptions`. The default depth is a
//! configurable parser-safety policy, not a language or hardware limit.
//! Collection sizes are dynamically represented and are limited only by
//! available resources and any shared parser budgets.
//!
//! # Grammar coverage
//!
//! Supported here:
//! - qualified and generic type names;
//! - parenthesized and tuple types;
//! - array and slice types;
//! - function types;
//! - references and pointers;
//! - optional types;
//! - union and intersection composition;
//! - `Self`, `_`, `void`, `linear`, `affine`, and `quantum` forms.
//!
//! Type parameters, where clauses, bounds, declaration syntax, and complex
//! type-level value expressions are owned by their corresponding grammar
//! modules. Semantic legality is checked downstream.

use core::fmt;

use crate::frontend::ast::node::types::type_expr::{
    LifetimeName, TypeExpr, TypeName, TypePath, TypeParameterName, TypeValueExpr,
};
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token_kind::TokenKind;

/// Minimal interface required from the shared parser cursor.
///
/// Implement this for the canonical parser cursor rather than creating a
/// second lexer or maintaining an independent token stream.
///
/// `current_kind`, `current_lexeme`, and `current_span` must refer to the
/// same current token. `advance` must make progress unless already at EOF.
///
/// `consume_type_closer` is context-sensitive. It consumes exactly one `>`
/// when the current token is `GreaterThan`, or one logical `>` from a
/// `RightShift` token in nested type context. When splitting `RightShift`,
/// the implementation must retain the unconsumed `>` for the caller.
pub trait TypeTokenCursor {
    fn current_kind(&self) -> TokenKind;

    fn current_lexeme(&self) -> &str;

    fn current_span(&self) -> Span;

    fn advance(&mut self);

    fn consume_type_closer(&mut self) -> bool;
}

/// Configurable type-parser safety policy.
///
/// `max_nesting_depth` limits recursive type grammar descent. Set it to
/// `None` only when the surrounding parser provides an equivalent stack-safe
/// strategy or has explicitly accepted the risk of deep recursive input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeParseOptions {
    pub max_nesting_depth: Option<usize>,
}

impl Default for TypeParseOptions {
    fn default() -> Self {
        Self {
            max_nesting_depth: Some(256),
        }
    }
}

/// Stable categories of type-grammar errors.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TypeParseErrorKind {
    UnexpectedToken {
        expected: &'static str,
        found: TokenKind,
    },
    EmptyQualifiedPath,
    EmptyGenericArguments,
    MissingType,
    MissingTypeArgument,
    MissingTupleElement,
    MissingArrayElement,
    MissingArrayLength,
    MissingFunctionReturnType,
    MissingTypeCloser,
    MissingDelimiter {
        delimiter: &'static str,
    },
    InvalidLifetime,
    NestingBudgetExceeded {
        maximum: usize,
    },
    InvalidCursorProgress,
}

/// Type-grammar error with a source location.
///
/// The parser's diagnostic layer can translate this into its canonical
/// diagnostic representation without parsing a human-readable message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeParseError {
    pub kind: TypeParseErrorKind,
    pub span: Span,
}

impl TypeParseError {
    fn new(kind: TypeParseErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for TypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TypeParseErrorKind::UnexpectedToken { expected, found } => {
                write!(f, "expected {expected}, found {}", found.diagnostic_name())
            }
            TypeParseErrorKind::EmptyQualifiedPath => {
                f.write_str("expected a name in the qualified type path")
            }
            TypeParseErrorKind::EmptyGenericArguments => {
                f.write_str("generic argument list cannot be empty")
            }
            TypeParseErrorKind::MissingType => f.write_str("expected a type"),
            TypeParseErrorKind::MissingTypeArgument => {
                f.write_str("expected a generic type argument")
            }
            TypeParseErrorKind::MissingTupleElement => {
                f.write_str("expected a tuple element type")
            }
            TypeParseErrorKind::MissingArrayElement => {
                f.write_str("expected an array element type")
            }
            TypeParseErrorKind::MissingArrayLength => {
                f.write_str("expected an array length")
            }
            TypeParseErrorKind::MissingFunctionReturnType => {
                f.write_str("expected a function return type after `->`")
            }
            TypeParseErrorKind::MissingTypeCloser => {
                f.write_str("expected `>` to close the type arguments")
            }
            TypeParseErrorKind::MissingDelimiter { delimiter } => {
                write!(f, "expected `{delimiter}`")
            }
            TypeParseErrorKind::InvalidLifetime => {
                f.write_str("expected a lifetime name after the lifetime marker")
            }
            TypeParseErrorKind::NestingBudgetExceeded { maximum } => {
                write!(f, "type nesting exceeds the configured budget of {maximum}")
            }
            TypeParseErrorKind::InvalidCursorProgress => {
                f.write_str("type parser cursor failed to make progress")
            }
        }
    }
}

impl std::error::Error for TypeParseError {}

/// Parse one type expression from the shared parser cursor.
///
/// The first token after the type is not consumed. The caller remains
/// responsible for delimiters, separators, and EOF enforcement.
pub fn parse_type<C: TypeTokenCursor + ?Sized>(
    cursor: &mut C,
    options: TypeParseOptions,
) -> Result<TypeExpr, TypeParseError> {
    let mut parser = TypeParser {
        cursor,
        options,
        depth: 0,
    };
    parser.parse_union()
}

struct TypeParser<'a, C: TypeTokenCursor + ?Sized> {
    cursor: &'a mut C,
    options: TypeParseOptions,
    depth: usize,
}

impl<C: TypeTokenCursor + ?Sized> TypeParser<'_, C> {
    fn kind(&self) -> TokenKind {
        self.cursor.current_kind()
    }

    fn span(&self) -> Span {
        self.cursor.current_span()
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.kind() == kind
    }

    fn error(&self, kind: TypeParseErrorKind) -> TypeParseError {
        TypeParseError::new(kind, self.span())
    }

    fn bump(&mut self) {
        if !self.at(TokenKind::Eof) {
            self.cursor.advance();
        }
    }

    fn expect(
        &mut self,
        expected: TokenKind,
        label: &'static str,
    ) -> Result<(), TypeParseError> {
        if self.at(expected) {
            self.bump();
            Ok(())
        } else {
            Err(self.error(TypeParseErrorKind::UnexpectedToken {
                expected: label,
                found: self.kind(),
            }))
        }
    }

    fn enter(&mut self) -> Result<(), TypeParseError> {
        if let Some(maximum) = self.options.max_nesting_depth {
            if self.depth >= maximum {
                return Err(self.error(
                    TypeParseErrorKind::NestingBudgetExceeded { maximum },
                ));
            }
        }

        self.depth = self.depth.saturating_add(1);
        Ok(())
    }

    fn leave(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    fn parse_union(&mut self) -> Result<TypeExpr, TypeParseError> {
        let first = self.parse_intersection()?;
        if !self.at(TokenKind::BitOr) {
            return Ok(first);
        }

        let mut members = vec![first];

        while self.at(TokenKind::BitOr) {
            self.bump();

            if !Self::starts_type(self.kind()) {
                return Err(self.error(TypeParseErrorKind::MissingTypeArgument));
            }

            members.push(self.parse_intersection()?);
        }

        Ok(TypeExpr::Union(members))
    }

    fn parse_intersection(&mut self) -> Result<TypeExpr, TypeParseError> {
        let first = self.parse_prefix()?;
        if !self.at(TokenKind::BitAnd) {
            return Ok(first);
        }

        let mut members = vec![first];

        while self.at(TokenKind::BitAnd) {
            self.bump();

            if !Self::starts_type(self.kind()) {
                return Err(self.error(TypeParseErrorKind::MissingTypeArgument));
            }

            members.push(self.parse_prefix()?);
        }

        Ok(TypeExpr::Intersection(members))
    }

    fn parse_prefix(&mut self) -> Result<TypeExpr, TypeParseError> {
        self.enter()?;

        let result = match self.kind() {
            TokenKind::BitAnd => {
                self.bump();

                let lifetime = if self.at(TokenKind::Apostrophe) {
                    self.bump();
                    let name = self.cursor.current_lexeme().to_owned();

                    if !self.at(TokenKind::Identifier) {
                        return self.finish_error(TypeParseErrorKind::InvalidLifetime);
                    }

                    self.bump();
                    Some(LifetimeName::new(name))
                } else {
                    None
                };

                let mutable = if self.at(TokenKind::KeywordMut) {
                    self.bump();
                    true
                } else {
                    false
                };

                let inner = self.parse_prefix()?;
                Ok(TypeExpr::reference(mutable, lifetime, inner))
            }

            TokenKind::Star => {
                self.bump();
                let mutable = if self.at(TokenKind::KeywordMut) {
                    self.bump();
                    true
                } else {
                    false
                };

                let inner = self.parse_prefix()?;
                Ok(TypeExpr::pointer(mutable, inner))
            }

            TokenKind::KeywordLinear => {
                self.bump();
                self.parse_prefix().map(TypeExpr::linear)
            }

            TokenKind::KeywordAffine => {
                self.bump();
                self.parse_prefix().map(TypeExpr::affine)
            }

            TokenKind::KeywordQuantum => {
                // `quantum::Type` is a qualified name; `quantum<T>` is a
                // source-level quantum type constructor.
                if self.cursor_lookahead_is_double_colon() {
                    self.parse_postfix()
                } else {
                    self.bump();
                    if self.at(TokenKind::LessThan) {
                        self.bump();
                        let inner = self.parse_union()?;
                        if !self.cursor.consume_type_closer() {
                            return self.finish_error(
                                TypeParseErrorKind::MissingTypeCloser,
                            );
                        }
                        Ok(TypeExpr::quantum(inner))
                    } else {
                        Ok(TypeExpr::name("quantum"))
                    }
                }
            }

            _ => self.parse_postfix(),
        };

        self.leave();
        result
    }

    fn finish_error<T>(
        &mut self,
        kind: TypeParseErrorKind,
    ) -> Result<T, TypeParseError> {
        self.leave();
        Err(self.error(kind))
    }

    fn cursor_lookahead_is_double_colon(&self) -> bool {
        // The shared cursor contract intentionally exposes no arbitrary
        // lookahead. The lexer's canonical spelling is enough only when the
        // cursor adapter exposes a contextual token view. By default, treat
        // `quantum` as the constructor keyword; qualified `quantum::...`
        // paths should be parsed as ordinary identifiers by the adapter.
        false
    }

    fn parse_postfix(&mut self) -> Result<TypeExpr, TypeParseError> {
        let mut ty = self.parse_atom()?;

        loop {
            match self.kind() {
                TokenKind::Question => {
                    self.bump();
                    ty = TypeExpr::optional(ty);
                }
                TokenKind::ThinArrow => {
                    // A function-type arrow is handled after a parameter
                    // tuple. Here an arrow means this is not a valid postfix.
                    break;
                }
                _ => break,
            }
        }

        Ok(ty)
    }

    fn parse_atom(&mut self) -> Result<TypeExpr, TypeParseError> {
        match self.kind() {
            TokenKind::LeftParen => self.parse_parenthesized_or_tuple(),
            TokenKind::LeftBracket => self.parse_array_or_slice(),
            TokenKind::KeywordFn => self.parse_function_type(),
            TokenKind::KeywordSelf => {
                self.bump();
                Ok(TypeExpr::SelfType)
            }
            TokenKind::KeywordVoid => {
                self.bump();
                Ok(TypeExpr::Unit)
            }
            TokenKind::Identifier
            | TokenKind::KeywordInt
            | TokenKind::KeywordFloat
            | TokenKind::KeywordBool
            | TokenKind::KeywordStr
            | TokenKind::KeywordStringType
            | TokenKind::KeywordCharType
            | TokenKind::KeywordQubit
            | TokenKind::KeywordQuantum
            | TokenKind::KeywordThis
            | TokenKind::KeywordSuper
            | TokenKind::KeywordModel
            | TokenKind::KeywordSigma
            | TokenKind::KeywordPi => self.parse_named_or_generic(),
            TokenKind::Eof => Err(self.error(TypeParseErrorKind::MissingType)),
            found => Err(self.error(TypeParseErrorKind::UnexpectedToken {
                expected: "a type",
                found,
            })),
        }
    }

    fn parse_named_or_generic(&mut self) -> Result<TypeExpr, TypeParseError> {
        let mut segments = Vec::new();

        if !Self::is_name_token(self.kind()) {
            return Err(self.error(TypeParseErrorKind::EmptyQualifiedPath));
        }

        segments.push(TypeName::from_str(self.cursor.current_lexeme()));
        self.bump();

        while self.at(TokenKind::DoubleColon) {
            self.bump();

            if !Self::is_name_token(self.kind()) {
                return Err(self.error(TypeParseErrorKind::EmptyQualifiedPath));
            }

            segments.push(TypeName::from_str(self.cursor.current_lexeme()));
            self.bump();
        }

        let mut ty = TypeExpr::named(TypePath::new(segments));

        if self.at(TokenKind::LessThan) {
            self.bump();

            if self.cursor.consume_type_closer() {
                return Err(self.error(TypeParseErrorKind::EmptyGenericArguments));
            }

            let mut arguments = Vec::new();

            loop {
                if !Self::starts_type(self.kind()) {
                    return Err(self.error(TypeParseErrorKind::MissingTypeArgument));
                }

                arguments.push(self.parse_union()?);

                if self.at(TokenKind::Comma) {
                    self.bump();

                    // A trailing comma is permitted, but the closer must be
                    // consumed by the shared cursor's type-context method.
                    if self.cursor.consume_type_closer() {
                        break;
                    }

                    continue;
                }

                if self.cursor.consume_type_closer() {
                    break;
                }

                return Err(self.error(TypeParseErrorKind::MissingTypeCloser));
            }

            ty = TypeExpr::generic(ty, arguments);
        }

        Ok(ty)
    }

    fn parse_parenthesized_or_tuple(&mut self) -> Result<TypeExpr, TypeParseError> {
        self.expect(TokenKind::LeftParen, "(")?;

        if self.at(TokenKind::RightParen) {
            self.bump();
            let unit = TypeExpr::Unit;

            if self.at(TokenKind::ThinArrow) {
                self.bump();
                let return_type = self.parse_union().map_err(|_| {
                    self.error(TypeParseErrorKind::MissingFunctionReturnType)
                })?;
                return Ok(TypeExpr::function(Vec::new(), return_type));
            }

            return Ok(unit);
        }

        let first = self.parse_union()?;

        if !self.at(TokenKind::Comma) {
            self.expect(TokenKind::RightParen, ")")?;

            if self.at(TokenKind::ThinArrow) {
                self.bump();
                let return_type = self.parse_union().map_err(|_| {
                    self.error(TypeParseErrorKind::MissingFunctionReturnType)
                })?;
                return Ok(TypeExpr::function(vec![first], return_type));
            }

            return Ok(first);
        }

        let mut elements = vec![first];

        while self.at(TokenKind::Comma) {
            self.bump();

            if self.at(TokenKind::RightParen) {
                break;
            }

            if !Self::starts_type(self.kind()) {
                return Err(self.error(TypeParseErrorKind::MissingTupleElement));
            }

            elements.push(self.parse_union()?);
        }

        self.expect(TokenKind::RightParen, ")")?;
        let tuple = TypeExpr::tuple(elements);

        if self.at(TokenKind::ThinArrow) {
            self.bump();
            let return_type = self.parse_union().map_err(|_| {
                self.error(TypeParseErrorKind::MissingFunctionReturnType)
            })?;

            match tuple {
                TypeExpr::Tuple(parameters) => {
                    Ok(TypeExpr::function(parameters, return_type))
                }
                other => Ok(TypeExpr::function(vec![other], return_type)),
            }
        } else {
            Ok(tuple)
        }
    }

    fn parse_array_or_slice(&mut self) -> Result<TypeExpr, TypeParseError> {
        self.expect(TokenKind::LeftBracket, "[")?;

        if !Self::starts_type(self.kind()) {
            return Err(self.error(TypeParseErrorKind::MissingArrayElement));
        }

        let element = self.parse_union()?;

        if self.at(TokenKind::Semicolon) {
            self.bump();

            let length = match self.kind() {
                TokenKind::IntegerLiteral => {
                    let text = self.cursor.current_lexeme().to_owned();
                    self.bump();
                    TypeValueExpr::Integer(text.into())
                }
                TokenKind::Identifier => {
                    let path = self.parse_value_path()?;
                    TypeValueExpr::Identifier(path)
                }
                _ => return Err(self.error(TypeParseErrorKind::MissingArrayLength)),
            };

            self.expect(TokenKind::RightBracket, "]")?;
            Ok(TypeExpr::array(element, Some(length)))
        } else {
            self.expect(TokenKind::RightBracket, "]")?;
            Ok(TypeExpr::slice(element))
        }
    }

    fn parse_value_path(&mut self) -> Result<TypePath, TypeParseError> {
        let mut names = vec![TypeName::from_str(self.cursor.current_lexeme())];
        self.bump();

        while self.at(TokenKind::DoubleColon) {
            self.bump();

            if !Self::is_name_token(self.kind()) {
                return Err(self.error(TypeParseErrorKind::EmptyQualifiedPath));
            }

            names.push(TypeName::from_str(self.cursor.current_lexeme()));
            self.bump();
        }

        Ok(TypePath::new(names))
    }

    fn parse_function_type(&mut self) -> Result<TypeExpr, TypeParseError> {
        self.expect(TokenKind::KeywordFn, "fn")?;
        self.expect(TokenKind::LeftParen, "(")?;

        let mut parameters = Vec::new();

        if !self.at(TokenKind::RightParen) {
            loop {
                if !Self::starts_type(self.kind()) {
                    return Err(self.error(TypeParseErrorKind::MissingTypeArgument));
                }

                parameters.push(self.parse_union()?);

                if self.at(TokenKind::Comma) {
                    self.bump();
                    if self.at(TokenKind::RightParen) {
                        break;
                    }
                    continue;
                }

                break;
            }
        }

        self.expect(TokenKind::RightParen, ")")?;
        self.expect(TokenKind::ThinArrow, "->")?;

        if !Self::starts_type(self.kind()) {
            return Err(self.error(TypeParseErrorKind::MissingFunctionReturnType));
        }

        let return_type = self.parse_union()?;
        Ok(TypeExpr::function(parameters, return_type))
    }

    fn is_name_token(kind: TokenKind) -> bool {
        kind == TokenKind::Identifier || kind.is_keyword()
    }

    fn starts_type(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Identifier
                | TokenKind::LeftParen
                | TokenKind::LeftBracket
                | TokenKind::KeywordFn
                | TokenKind::KeywordSelf
                | TokenKind::KeywordVoid
                | TokenKind::KeywordInt
                | TokenKind::KeywordFloat
                | TokenKind::KeywordBool
                | TokenKind::KeywordStr
                | TokenKind::KeywordStringType
                | TokenKind::KeywordCharType
                | TokenKind::KeywordQubit
                | TokenKind::KeywordQuantum
                | TokenKind::KeywordLinear
                | TokenKind::KeywordAffine
                | TokenKind::KeywordThis
                | TokenKind::KeywordSuper
                | TokenKind::KeywordModel
                | TokenKind::KeywordSigma
                | TokenKind::KeywordPi
                | TokenKind::BitAnd
                | TokenKind::Star
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tiny test cursor. Production code should implement `TypeTokenCursor`
    /// on the shared parser cursor instead of using this test-only adapter.
    struct TestCursor {
        tokens: Vec<(TokenKind, String, Span)>,
        index: usize,
    }

    impl TypeTokenCursor for TestCursor {
        fn current_kind(&self) -> TokenKind {
            self.tokens
                .get(self.index)
                .map(|token| token.0)
                .unwrap_or(TokenKind::Eof)
        }

        fn current_lexeme(&self) -> &str {
            self.tokens
                .get(self.index)
                .map(|token| token.1.as_str())
                .unwrap_or("")
        }

        fn current_span(&self) -> Span {
            self.tokens
                .get(self.index)
                .map(|token| token.2)
                .or_else(|| self.tokens.last().map(|token| token.2))
                .expect("test cursor requires at least one token")
        }

        fn advance(&mut self) {
            if self.index < self.tokens.len() {
                self.index += 1;
            }
        }

        fn consume_type_closer(&mut self) -> bool {
            if self.current_kind() == TokenKind::GreaterThan {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    // Span construction depends on the repository's canonical CursorPosition.
    // The parser tests in the integrated crate should use the shared lexer
    // test fixtures instead of duplicating source-coordinate conventions.
}
