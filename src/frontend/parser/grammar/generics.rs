
#![forbid(unsafe_code)]

//! Generic syntax productions for the Zamani frontend.
//!
//! File: `src/frontend/parser/grammar/generics.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Parse generic parameter lists, generic argument lists, and `where`
//! clauses using the canonical lexer and parser cursor.
//!
//! # Ownership
//!
//! This module owns generic-list delimiters, comma separation, parameter
//! classification, top-level constraint/default separators, balanced
//! delimiter handling, and syntax-preserving generic fragments.
//!
//! It does not own tokenization, type resolution, trait resolution,
//! constant evaluation, type inference, semantic constraints, or AST
//! constructors.
//!
//! # Integration
//!
//! Uses:
//! - `crate::frontend::lexer::token::Token`;
//! - `crate::frontend::lexer::token_kind::TokenKind`;
//! - `crate::frontend::lexer::span::Span`;
//! - `super::super::parser::{ParserError, ParserErrorKind, TokenCursor}`.
//!
//! The grammar integration should call these productions from declarations,
//! types, and other constructs that permit generic syntax. The AST adapter
//! converts the loss-preserving fragments into the repository's canonical
//! type, expression, path, and generic AST nodes.
//!
//! # Resource policy
//!
//! No generic parameter count, argument count, nesting depth, or fragment
//! length is hard-coded as a language limit. The caller supplies a budget
//! derived from the parser's resource policy. Processing is necessarily
//! limited by available memory, address space, and that budget.
//!
//! # Grammar policy
//!
//! This module does not guess semantic meaning from identifiers. It retains
//! syntax fragments so later grammar/AST integration can interpret bounds,
//! types, and const expressions using their authoritative productions.
//!
//! A lexer that combines adjacent closing angle brackets into `>>` must
//! provide a parser-compatible tokenization policy for nested generic types.
//! This module deliberately does not reinterpret a shift operator as two
//! closing delimiters, because that decision is context-sensitive.

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

use super::super::parser::{ParserError, ParserErrorKind, TokenCursor};

/// Caller-controlled safety budget for generic parsing.
///
/// These are implementation budgets, not limits imposed by the Zamani
/// language. Use the parser's shared resource policy to construct this
/// value; do not maintain a separate, conflicting budget per grammar file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenericParseBudget {
    /// Maximum number of tokens accepted in one generic fragment.
    pub max_fragment_tokens: usize,

    /// Maximum simultaneously nested delimiter levels.
    pub max_nesting_depth: usize,

    /// Maximum total tokens consumed by generic productions in this call.
    pub max_total_tokens: usize,
}

impl GenericParseBudget {
    /// Construct a budget from caller-selected implementation limits.
    #[must_use]
    pub const fn new(
        max_fragment_tokens: usize,
        max_nesting_depth: usize,
        max_total_tokens: usize,
    ) -> Self {
        Self {
            max_fragment_tokens,
            max_nesting_depth,
            max_total_tokens,
        }
    }
}

/// A single preserved token from a generic syntax fragment.
///
/// The original token's identity, spelling, and source span are retained.
/// This snapshot is independent of the source string's lifetime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericToken {
    /// Canonical lexical identity.
    pub kind: TokenKind,

    /// Exact source spelling.
    pub lexeme: String,

    /// Canonical source span.
    pub span: Span,
}

impl GenericToken {
    fn from_token(token: &Token<'_>) -> Self {
        Self {
            kind: token.kind(),
            lexeme: token.lexeme().to_owned(),
            span: *token.span(),
        }
    }
}

/// A loss-preserving token sequence representing a type, bound, const
/// expression, path, or other generic subproduction.
///
/// Interpretation is deliberately delegated to the shared grammar and
/// AST adapter. This type is not a second canonical AST.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GenericFragment {
    /// Tokens in source order, excluding the separator that ended the
    /// fragment and excluding the outer generic-list delimiters.
    pub tokens: Vec<GenericToken>,
}

impl GenericFragment {
    /// Return whether this fragment contains no tokens.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Return the number of tokens retained in this fragment.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Return the first token's source span, if present.
    #[must_use]
    pub fn first_span(&self) -> Option<Span> {
        self.tokens.first().map(|token| token.span)
    }

    /// Return the last token's source span, if present.
    #[must_use]
    pub fn last_span(&self) -> Option<Span> {
        self.tokens.last().map(|token| token.span)
    }

    /// Reconstruct the fragment's token spellings without adding spaces.
    ///
    /// This is useful for diagnostics and debugging only. It is not a
    /// canonical formatter because original whitespace and trivia may not
    /// be present in the parser-facing token stream.
    #[must_use]
    pub fn concatenated_lexemes(&self) -> String {
        let capacity = self
            .tokens
            .iter()
            .fold(0usize, |sum, token| sum.saturating_add(token.lexeme.len()));

        let mut result = String::with_capacity(capacity);

        for token in &self.tokens {
            result.push_str(&token.lexeme);
        }

        result
    }
}

/// The syntactic category of a generic parameter.
///
/// Whether a bound is satisfiable or a default is type-correct is a
/// semantic-analysis concern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericParameterKind {
    /// A type parameter such as `T: Display`.
    Type,

    /// A const/value parameter such as `const N: usize`.
    Const,
}

/// One generic parameter as written in a parameter list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericParameter {
    /// Parameter category.
    pub kind: GenericParameterKind,

    /// Identifier spelling exactly as it appeared in source.
    pub name: String,

    /// Bounds or constraints following a top-level `:`.
    ///
    /// For type parameters this usually represents trait/type bounds.
    /// For const parameters, this field is normally empty.
    pub bounds: Option<GenericFragment>,

    /// Declared type of a const parameter, when applicable.
    pub const_type: Option<GenericFragment>,

    /// Default type or const expression following a top-level `=`.
    pub default: Option<GenericFragment>,

    /// Source span of the parameter's name.
    pub name_span: Span,
}

/// Parsed generic parameter list.
///
/// The outer angle brackets are consumed by the parser and are not part of
/// the parameter collection.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GenericParameters {
    /// Parameters in declaration order.
    pub parameters: Vec<GenericParameter>,
}

/// Parsed generic argument list.
///
/// Each argument remains a fragment for interpretation by the shared type,
/// expression, lifetime, or path grammar as required by the language.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GenericArguments {
    /// Arguments in source order.
    pub arguments: Vec<GenericFragment>,
}

/// A single predicate in a `where` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WherePredicate {
    /// Predicate tokens excluding the separating comma.
    ///
    /// Examples include `T: Display`, `T: Trait<U>`, and
    /// `N: ValidDimension`. Semantic meaning is resolved later.
    pub expression: GenericFragment,
}

/// Parsed `where` clause.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhereClause {
    /// Predicates in source order.
    pub predicates: Vec<WherePredicate>,
}

/// Parse a generic parameter list beginning with `<`.
///
/// Supported parameter shapes include:
///
/// - `T`
/// - `T: Bound`
/// - `T: Bound + OtherBound`
/// - `T = DefaultType`
/// - `T: Bound = DefaultType`
/// - `const N: usize`
/// - `const N: usize = DEFAULT`
///
/// Nested type and expression syntax is preserved as token fragments.
/// The caller's grammar must supply a budget appropriate for the current
/// parser session.
///
/// # Errors
///
/// Returns a structured parser error for missing delimiters, malformed
/// parameters, unexpected EOF, unbalanced delimiters, or budget exhaustion.
pub fn parse_generic_parameters<'tokens, 'src>(
    cursor: &mut TokenCursor<'tokens, 'src>,
    budget: GenericParseBudget,
) -> Result<GenericParameters, ParserError> {
    let start = cursor.position();
    cursor.expect(TokenKind::LessThan)?;

    let mut result = GenericParameters::default();

    if cursor.current_kind() == Some(TokenKind::GreaterThan) {
        return Err(error_at_current(
            cursor,
            "generic parameter lists cannot be empty",
        ));
    }

    loop {
        ensure_total_budget(cursor, start, budget)?;

        let fragment = take_fragment(
            cursor,
            budget,
            FragmentStop::CommaOrAngleClose,
        )?;

        if fragment.is_empty() {
            return Err(error_at_current(
                cursor,
                "expected a generic parameter before the separator",
            ));
        }

        result.parameters.push(parse_parameter_fragment(fragment)?);

        match cursor.current_kind() {
            Some(TokenKind::Comma) => {
                cursor.advance()?;

                // Permit a trailing comma in parameter lists.
                if cursor.current_kind() == Some(TokenKind::GreaterThan) {
                    cursor.advance()?;
                    return Ok(result);
                }
            }
            Some(TokenKind::GreaterThan) => {
                cursor.advance()?;
                return Ok(result);
            }
            Some(TokenKind::Eof) | None => {
                return Err(error_at_current(
                    cursor,
                    "unterminated generic parameter list; expected `>`",
                ));
            }
            _ => {
                return Err(error_at_current(
                    cursor,
                    "expected `,` or `>` after a generic parameter",
                ));
            }
        }
    }
}

/// Parse a generic argument list beginning with `<`.
///
/// Arguments are preserved without assuming that every argument is a type:
/// the language may permit types, const expressions, lifetimes, or other
/// specification-defined forms. The shared grammar/AST adapter determines
/// which forms are legal in a particular generic application.
///
/// Empty argument lists are rejected. Trailing commas are accepted.
pub fn parse_generic_arguments<'tokens, 'src>(
    cursor: &mut TokenCursor<'tokens, 'src>,
    budget: GenericParseBudget,
) -> Result<GenericArguments, ParserError> {
    let start = cursor.position();
    cursor.expect(TokenKind::LessThan)?;

    let mut result = GenericArguments::default();

    if cursor.current_kind() == Some(TokenKind::GreaterThan) {
        return Err(error_at_current(
            cursor,
            "generic argument lists cannot be empty",
        ));
    }

    loop {
        ensure_total_budget(cursor, start, budget)?;

        let argument = take_fragment(
            cursor,
            budget,
            FragmentStop::CommaOrAngleClose,
        )?;

        if argument.is_empty() {
            return Err(error_at_current(
                cursor,
                "expected a generic argument before the separator",
            ));
        }

        result.arguments.push(argument);

        match cursor.current_kind() {
            Some(TokenKind::Comma) => {
                cursor.advance()?;

                if cursor.current_kind() == Some(TokenKind::GreaterThan) {
                    cursor.advance()?;
                    return Ok(result);
                }
            }
            Some(TokenKind::GreaterThan) => {
                cursor.advance()?;
                return Ok(result);
            }
            Some(TokenKind::Eof) | None => {
                return Err(error_at_current(
                    cursor,
                    "unterminated generic argument list; expected `>`",
                ));
            }
            _ => {
                return Err(error_at_current(
                    cursor,
                    "expected `,` or `>` after a generic argument",
                ));
            }
        }
    }
}

/// Parse a `where` clause beginning with the `where` keyword.
///
/// Predicates are split at top-level commas. Parentheses, brackets, braces,
/// and nested angle-bracket groups protect commas within nested constructs.
/// Parsing stops before a top-level block opener or semicolon, leaving that
/// delimiter for the caller.
///
/// The `where` token itself must already be present at the cursor.
pub fn parse_where_clause<'tokens, 'src>(
    cursor: &mut TokenCursor<'tokens, 'src>,
    budget: GenericParseBudget,
) -> Result<WhereClause, ParserError> {
    let start = cursor.position();
    cursor.expect(TokenKind::KeywordWhere)?;

    let mut result = WhereClause::default();

    loop {
        ensure_total_budget(cursor, start, budget)?;

        match cursor.current_kind() {
            Some(TokenKind::LeftBrace) | Some(TokenKind::Semicolon) => {
                if result.predicates.is_empty() {
                    return Err(error_at_current(
                        cursor,
                        "expected at least one predicate after `where`",
                    ));
                }
                return Ok(result);
            }
            Some(TokenKind::Eof) | None => {
                return Err(error_at_current(
                    cursor,
                    "unterminated `where` clause",
                ));
            }
            _ => {}
        }

        let predicate = take_fragment(
            cursor,
            budget,
            FragmentStop::CommaOrWhereTerminator,
        )?;

        if predicate.is_empty() {
            return Err(error_at_current(
                cursor,
                "expected a predicate in the `where` clause",
            ));
        }

        result.predicates.push(WherePredicate {
            expression: predicate,
        });

        match cursor.current_kind() {
            Some(TokenKind::Comma) => {
                cursor.advance()?;

                if matches!(
                    cursor.current_kind(),
                    Some(TokenKind::LeftBrace)
                        | Some(TokenKind::Semicolon)
                        | Some(TokenKind::Eof)
                ) {
                    return Err(error_at_current(
                        cursor,
                        "expected a predicate after `,` in the `where` clause",
                    ));
                }
            }
            Some(TokenKind::LeftBrace) | Some(TokenKind::Semicolon) => {
                return Ok(result);
            }
            Some(TokenKind::Eof) | None => {
                return Err(error_at_current(
                    cursor,
                    "unterminated `where` clause",
                ));
            }
            _ => {
                return Err(error_at_current(
                    cursor,
                    "expected `,`, a block, or `;` after a `where` predicate",
                ));
            }
        }
    }
}

/// Parse the already-collected tokens for one generic parameter.
///
/// A top-level `:` separates a type parameter's bounds or a const
/// parameter's declared type. A top-level `=` separates its default.
/// Delimiters nested inside type/const syntax do not act as separators.
fn parse_parameter_fragment(
    fragment: GenericFragment,
) -> Result<GenericParameter, ParserError> {
    let tokens = fragment.tokens;

    let Some(first) = tokens.first() else {
        return Err(ParserError::new(
            ParserErrorKind::UnexpectedToken,
            Span::default(),
            "generic parameter is empty",
        ));
    };

    let is_const = first.kind == TokenKind::KeywordConst;
    let name_index = usize::from(is_const);

    let Some(name_token) = tokens.get(name_index) else {
        return Err(ParserError::new(
            ParserErrorKind::UnexpectedToken,
            first.span,
            "expected an identifier after the generic parameter introducer",
        ));
    };

    if name_token.kind != TokenKind::Identifier {
        return Err(ParserError::new(
            ParserErrorKind::UnexpectedToken,
            name_token.span,
            "expected an identifier as the generic parameter name",
        ));
    }

    let tail = &tokens[name_index + 1..];
    let separators = top_level_separators(tail);

    let colon = separators
        .iter()
        .find(|(index, kind)| *kind == TokenKind::Colon)
        .map(|(index, _)| *index);

    let assign = separators
        .iter()
        .find(|(index, kind)| *kind == TokenKind::Assign)
        .map(|(index, _)| *index);

    if let (Some(colon), Some(assign)) = (colon, assign) {
        if assign < colon {
            return Err(ParserError::new(
                ParserErrorKind::UnexpectedToken,
                tail[assign].span,
                "a generic parameter default cannot precede its declared type or bounds",
            ));
        }
    }

    let constraint_start = colon.map(|index| index + 1);
    let default_start = assign.map(|index| index + 1);

    let constraint_end = assign.unwrap_or(tail.len());

    let constraint = match constraint_start {
        Some(start) if start < constraint_end => {
            Some(GenericFragment {
                tokens: tail[start..constraint_end].to_vec(),
            })
        }
        Some(_) => {
            return Err(ParserError::new(
                ParserErrorKind::UnexpectedToken,
                tail[colon.unwrap_or(0)].span,
                "expected a type or bound after `:`",
            ));
        }
        None => None,
    };

    let default = match default_start {
        Some(start) if start < tail.len() => Some(GenericFragment {
            tokens: tail[start..].to_vec(),
        }),
        Some(_) => {
            return Err(ParserError::new(
                ParserErrorKind::UnexpectedToken,
                tail[assign.unwrap_or(0)].span,
                "expected a default value after `=`",
            ));
        }
        None => None,
    };

    if is_const && constraint.is_none() {
        return Err(ParserError::new(
            ParserErrorKind::UnexpectedToken,
            name_token.span,
            "a const generic parameter requires a declared type, such as `const N: usize`",
        ));
    }

    Ok(GenericParameter {
        kind: if is_const {
            GenericParameterKind::Const
        } else {
            GenericParameterKind::Type
        },
        name: name_token.lexeme.clone(),
        bounds: if is_const { None } else { constraint.clone() },
        const_type: if is_const { constraint } else { None },
        default,
        name_span: name_token.span,
    })
}

/// Separators appearing at the top level of a fragment.
///
/// The fragment has already been collected with delimiter balancing, so this
/// scan uses the same balancing rules to avoid splitting inside nested types
/// or expressions.
fn top_level_separators(
    tokens: &[GenericToken],
) -> Vec<(usize, TokenKind)> {
    let mut separators = Vec::new();
    let mut delimiters: Vec<TokenKind> = Vec::new();
    let mut angle_depth = 0usize;

    for (index, token) in tokens.iter().enumerate() {
        match token.kind {
            TokenKind::LeftParen
            | TokenKind::LeftBracket
            | TokenKind::LeftBrace => delimiters.push(token.kind),

            TokenKind::RightParen
            | TokenKind::RightBracket
            | TokenKind::RightBrace => {
                if !delimiters.is_empty() {
                    delimiters.pop();
                }
            }

            TokenKind::LessThan if delimiters.is_empty() => {
                angle_depth = angle_depth.saturating_add(1);
            }

            TokenKind::GreaterThan if delimiters.is_empty() && angle_depth > 0 => {
                angle_depth -= 1;
            }

            TokenKind::Colon | TokenKind::Assign
                if delimiters.is_empty() && angle_depth == 0 =>
            {
                separators.push((index, token.kind));
            }

            _ => {}
        }
    }

    separators
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FragmentStop {
    CommaOrAngleClose,
    CommaOrWhereTerminator,
}

/// Collect one comma-separated fragment, preserving every token.
///
/// Delimiters inside nested parentheses, brackets, braces, or angle-bracket
/// groups do not terminate the fragment. The outer separator is left
/// unconsumed for the caller.
///
/// This routine is intentionally conservative: mismatched closing
/// delimiters are reported rather than silently discarded.
fn take_fragment<'tokens, 'src>(
    cursor: &mut TokenCursor<'tokens, 'src>,
    budget: GenericParseBudget,
    stop: FragmentStop,
) -> Result<GenericFragment, ParserError> {
    let mut fragment = GenericFragment::default();
    let mut delimiters: Vec<(TokenKind, Span)> = Vec::new();
    let mut angle_depth = 0usize;

    loop {
        let Some(token) = cursor.current() else {
            return Err(error_at_current(
                cursor,
                "unexpected end of token stream while parsing generic syntax",
            ));
        };

        let kind = token.kind();

        if delimiters.is_empty() && angle_depth == 0 {
            match kind {
                TokenKind::Comma => break,
                TokenKind::GreaterThan
                    if stop == FragmentStop::CommaOrAngleClose =>
                {
                    break;
                }
                TokenKind::LeftBrace | TokenKind::Semicolon
                    if stop == FragmentStop::CommaOrWhereTerminator =>
                {
                    break;
                }
                TokenKind::Eof => {
                    return Err(ParserError::at_token(
                        ParserErrorKind::UnexpectedToken,
                        token,
                        "unexpected EOF inside generic syntax",
                    ));
                }
                _ => {}
            }
        }

        match kind {
            TokenKind::LeftParen
            | TokenKind::LeftBracket
            | TokenKind::LeftBrace => {
                push_delimiter(
                    &mut delimiters,
                    kind,
                    *token.span(),
                    budget,
                    token,
                )?;
            }

            TokenKind::RightParen
            | TokenKind::RightBracket
            | TokenKind::RightBrace => {
                let expected_open = matching_open(kind);

                match delimiters.last() {
                    Some((open, _)) if *open == expected_open => {
                        delimiters.pop();
                    }
                    Some((open, _)) => {
                        return Err(ParserError::at_token(
                            ParserErrorKind::UnexpectedToken,
                            token,
                            format!(
                                "mismatched closing delimiter {kind:?}; \
                                 current nested delimiter is {open:?}"
                            ),
                        ));
                    }
                    None => {
                        return Err(ParserError::at_token(
                            ParserErrorKind::UnexpectedToken,
                            token,
                            format!("unexpected closing delimiter {kind:?}"),
                        ));
                    }
                }
            }

            TokenKind::LessThan if delimiters.is_empty() => {
                angle_depth = angle_depth.checked_add(1).ok_or_else(|| {
                    ParserError::at_token(
                        ParserErrorKind::InvalidParserState,
                        token,
                        "generic angle-bracket nesting counter overflow",
                    )
                })?;

                if angle_depth > budget.max_nesting_depth {
                    return Err(ParserError::at_token(
                        ParserErrorKind::UnexpectedToken,
                        token,
                        "generic parsing exceeded the configured nesting budget",
                    ));
                }
            }

            TokenKind::GreaterThan if delimiters.is_empty() && angle_depth > 0 => {
                angle_depth -= 1;
            }

            _ => {}
        }

        if fragment.tokens.len() >= budget.max_fragment_tokens {
            return Err(ParserError::at_token(
                ParserErrorKind::UnexpectedToken,
                token,
                "generic fragment exceeded the configured token budget",
            ));
        }

        fragment.tokens.push(GenericToken::from_token(token));
        cursor.advance()?;
    }

    if !delimiters.is_empty() || angle_depth != 0 {
        let span = delimiters
            .last()
            .map(|(_, span)| *span)
            .unwrap_or_else(|| {
                fragment
                    .tokens
                    .last()
                    .map(|token| token.span)
                    .unwrap_or_default()
            });

        return Err(ParserError::new(
            ParserErrorKind::UnexpectedToken,
            span,
            "unclosed delimiter in generic syntax",
        ));
    }

    Ok(fragment)
}

fn push_delimiter(
    delimiters: &mut Vec<(TokenKind, Span)>,
    kind: TokenKind,
    span: Span,
    budget: GenericParseBudget,
    token: &Token<'_>,
) -> Result<(), ParserError> {
    if delimiters.len() >= budget.max_nesting_depth {
        return Err(ParserError::at_token(
            ParserErrorKind::UnexpectedToken,
            token,
            "generic parsing exceeded the configured nesting budget",
        ));
    }

    delimiters.push((kind, span));
    Ok(())
}

fn matching_open(close: TokenKind) -> TokenKind {
    match close {
        TokenKind::RightParen => TokenKind::LeftParen,
        TokenKind::RightBracket => TokenKind::LeftBracket,
        TokenKind::RightBrace => TokenKind::LeftBrace,
        _ => close,
    }
}

fn ensure_total_budget(
    cursor: &TokenCursor<'_, '_>,
    start: usize,
    budget: GenericParseBudget,
) -> Result<(), ParserError> {
    let consumed = cursor.position().saturating_sub(start);

    if consumed > budget.max_total_tokens {
        return Err(error_at_current(
            cursor,
            "generic parsing exceeded the configured total-token budget",
        ));
    }

    Ok(())
}

fn error_at_current(
    cursor: &TokenCursor<'_, '_>,
    message: impl Into<String>,
) -> ParserError {
    match cursor.current() {
        Some(token) => ParserError::at_token(
            ParserErrorKind::UnexpectedToken,
            token,
            message,
        ),
        None => ParserError::new(
            ParserErrorKind::InvalidParserState,
            Span::default(),
            message,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_kind_is_explicit() {
        assert_ne!(
            GenericParameterKind::Type,
            GenericParameterKind::Const
        );
    }

    #[test]
    fn empty_fragment_is_well_defined() {
        let fragment = GenericFragment::default();

        assert!(fragment.is_empty());
        assert_eq!(fragment.len(), 0);
        assert_eq!(fragment.first_span(), None);
        assert_eq!(fragment.last_span(), None);
        assert_eq!(fragment.concatenated_lexemes(), "");
    }

    #[test]
    fn fragment_lexemes_preserve_token_order() {
        let fragment = GenericFragment {
            tokens: vec![
                GenericToken {
                    kind: TokenKind::Identifier,
                    lexeme: "Vector".to_owned(),
                    span: Span::default(),
                },
                GenericToken {
                    kind: TokenKind::LessThan,
                    lexeme: "<".to_owned(),
                    span: Span::default(),
                },
                GenericToken {
                    kind: TokenKind::Identifier,
                    lexeme: "T".to_owned(),
                    span: Span::default(),
                },
                GenericToken {
                    kind: TokenKind::GreaterThan,
                    lexeme: ">".to_owned(),
                    span: Span::default(),
                },
            ],
        };

        assert_eq!(fragment.concatenated_lexemes(), "Vector<T>");
    }

    #[test]
    fn budget_is_caller_configurable() {
        let budget = GenericParseBudget::new(17, 5, 100);

        assert_eq!(budget.max_fragment_tokens, 17);
        assert_eq!(budget.max_nesting_depth, 5);
        assert_eq!(budget.max_total_tokens, 100);
    }
}
