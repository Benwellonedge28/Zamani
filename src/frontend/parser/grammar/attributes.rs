
#![forbid(unsafe_code)]

//! Zamani parser — generic source-level attributes.
//!
//! Intended path: `src/frontend/parser/grammar/attributes.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Responsibilities
//!
//! - Parse the syntax specified by `grammar/core/attributes.g4`.
//! - Preserve namespaced attribute names and source order.
//! - Preserve positional/named argument distinctions and duplicates.
//! - Parse scalar, symbolic, list, map, and nested attribute values.
//! - Preserve source spans and source-level numeric spelling.
//! - Apply caller-supplied parser budgets.
//!
//! # Does not own
//!
//! - Lexing, token spelling, or identifier classification.
//! - Attribute semantics, name resolution, or type checking.
//! - Attribute attachment to declarations or other AST nodes.
//! - Hardware selection, resource allocation, or execution.
//! - Parser-wide diagnostics, recovery policy, or source-map ownership.
//!
//! # Integration
//!
//! The parser's item/declaration rules call `parse_attribute` or
//! `parse_attribute_prefix`. This module consumes the existing lexer tokens
//! and constructs the canonical AST attribute representation.
//!
//! The caller supplies a `SourceId`, since the lexer's `Span` does not itself
//! identify a source file. Lexer and AST spans use different representations;
//! this module converts between them explicitly.
//!
//! This module is deliberately independent of any not-yet-established
//! `ParserSession`, `ParserContext`, or `TokenCursor` API.

use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::annotations::attribute::{
    Attribute,
    AttributeArgument,
    AttributeName,
    AttributeObjectEntry,
    AttributeValue,
};
use crate::frontend::ast::node::source::{SourceId, Span as AstSpan};
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;

/// Caller-controlled resource budgets for parsing attribute syntax.
///
/// These are parser safety budgets, not language-level limits and not
/// limits on hardware resources. Set them from the parser's shared
/// configuration. No machine-size or hardware-capacity assumptions belong
/// here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AttributeParseLimits {
    /// Maximum permitted nested attribute/value depth.
    pub max_nesting: usize,

    /// Maximum number of attributes, arguments, and values constructed by
    /// one parse operation, counted together.
    pub max_constructs: usize,
}

impl AttributeParseLimits {
    /// Creates explicit budgets supplied by the calling parser.
    #[must_use]
    pub const fn new(max_nesting: usize, max_constructs: usize) -> Self {
        Self {
            max_nesting,
            max_constructs,
        }
    }
}

/// Structured failures produced by this grammar component.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AttributeParseErrorKind {
    /// No token was available where an attribute was required.
    UnexpectedEnd,

    /// A token did not match the expected grammar production.
    UnexpectedToken {
        expected: &'static str,
        found: String,
    },

    /// A name did not contain a valid identifier token.
    InvalidName,

    /// A literal's source spelling was malformed.
    InvalidLiteral {
        reason: &'static str,
    },

    /// A nesting budget supplied by the caller was exhausted.
    NestingBudgetExceeded {
        limit: usize,
    },

    /// A construction budget supplied by the caller was exhausted.
    ConstructBudgetExceeded {
        limit: usize,
    },

    /// Source coordinates could not be converted into a canonical AST span.
    InvalidSourceSpan,

    /// The token stream contains a token that cannot be consumed as
    /// ordinary parser input.
    InvalidTokenStream,
}

impl fmt::Display for AttributeParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd => {
                f.write_str("unexpected end of input while parsing an attribute")
            }
            Self::UnexpectedToken { expected, found } => {
                write!(f, "expected {expected}, found {found}")
            }
            Self::InvalidName => f.write_str("invalid attribute name"),
            Self::InvalidLiteral { reason } => {
                write!(f, "invalid attribute literal: {reason}")
            }
            Self::NestingBudgetExceeded { limit } => {
                write!(f, "attribute nesting budget exceeded ({limit})")
            }
            Self::ConstructBudgetExceeded { limit } => {
                write!(f, "attribute construction budget exceeded ({limit})")
            }
            Self::InvalidSourceSpan => {
                f.write_str("unable to construct a valid attribute source span")
            }
            Self::InvalidTokenStream => {
                f.write_str("invalid token stream supplied to attribute parser")
            }
        }
    }
}

/// Attribute parsing error with a canonical source span.
///
/// The span is suitable for conversion into the parser's shared diagnostic
/// type. This component does not format terminal colors or emit diagnostics
/// directly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttributeParseError {
    pub kind: AttributeParseErrorKind,
    pub span: AstSpan,
}

impl fmt::Display for AttributeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Error for AttributeParseError {}

/// Parse one attribute at `start`.
///
/// Returns the parsed attribute and the index of the next non-trivia token.
/// The input may contain trivia-channel tokens; those tokens are skipped
/// without changing the source spans of significant tokens.
///
/// The caller owns the token vector and decides whether the following token
/// is legal in the enclosing grammar.
///
/// # Accepted forms
///
/// - `@inline`
/// - `@inline()`
/// - `@quantum::logical`
/// - `@resource(memory = required_memory)`
/// - `@meta::list([a, b,])`
/// - `@meta::map({mode = "fast",})`
/// - `@outer(@inner(value))`
///
/// Nested attribute values require `AttributeValue::Nested` in the canonical
/// AST as documented above.
pub fn parse_attribute<'src>(
    tokens: &'src [Token<'src>],
    start: usize,
    source_id: SourceId,
    limits: AttributeParseLimits,
) -> Result<(Attribute, usize), AttributeParseError> {
    let mut parser = AttributeParser::new(tokens, start, source_id, limits)?;
    let attribute = parser.parse_attribute_at_depth(0)?;
    Ok((attribute, parser.position()))
}

/// Parse zero or more consecutive attributes.
///
/// This function does not require an attribute to be present. It is suitable
/// for optional attribute prefixes; use `parse_attribute` when at least one
/// attribute is required.
///
/// Parsing stops at the first non-attribute token. The returned index points
/// at that token after trivia has been skipped.
pub fn parse_attribute_prefix<'src>(
    tokens: &'src [Token<'src>],
    start: usize,
    source_id: SourceId,
    limits: AttributeParseLimits,
) -> Result<(Vec<Attribute>, usize), AttributeParseError> {
    let mut parser = AttributeParser::new(tokens, start, source_id, limits)?;
    let mut attributes = Vec::new();

    while parser.peek_kind() == Some(TokenKind::At) {
        attributes.push(parser.parse_attribute_at_depth(0)?);
    }

    Ok((attributes, parser.position()))
}

struct AttributeParser<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    index: usize,
    source_id: SourceId,
    limits: AttributeParseLimits,
    constructs: usize,
}

impl<'tokens, 'src> AttributeParser<'tokens, 'src> {
    fn new(
        tokens: &'tokens [Token<'src>],
        start: usize,
        source_id: SourceId,
        limits: AttributeParseLimits,
    ) -> Result<Self, AttributeParseError> {
        if start > tokens.len() {
            let span = span_at(tokens, start, source_id)?;
            return Err(AttributeParseError {
                kind: AttributeParseErrorKind::InvalidTokenStream,
                span,
            });
        }

        let mut parser = Self {
            tokens,
            index: start,
            source_id,
            limits,
            constructs: 0,
        };
        parser.skip_trivia();
        Ok(parser)
    }

    fn position(&self) -> usize {
        self.index
    }

    fn skip_trivia(&mut self) {
        while self
            .tokens
            .get(self.index)
            .is_some_and(|token| {
                matches!(
                    token.channel(),
                    TokenChannel::Trivia | TokenChannel::Documentation
                )
            })
        {
            self.index += 1;
        }
    }

    fn peek(&self) -> Option<&'tokens Token<'src>> {
        let mut index = self.index;

        while let Some(token) = self.tokens.get(index) {
            if matches!(
                token.channel(),
                TokenChannel::Trivia | TokenChannel::Documentation
            ) {
                index += 1;
                continue;
            }
            return Some(token);
        }

        None
    }

    fn peek_nth(&self, n: usize) -> Option<&'tokens Token<'src>> {
        let mut index = self.index;
        let mut significant = 0usize;

        while let Some(token) = self.tokens.get(index) {
            if matches!(
                token.channel(),
                TokenChannel::Trivia | TokenChannel::Documentation
            ) {
                index += 1;
                continue;
            }

            if significant == n {
                return Some(token);
            }

            significant = significant.saturating_add(1);
            index += 1;
        }

        None
    }

    fn peek_kind(&self) -> Option<TokenKind> {
        self.peek().map(Token::kind)
    }

    fn bump(&mut self) -> Option<&'tokens Token<'src>> {
        self.skip_trivia();

        let token = self.tokens.get(self.index)?;
        if token.channel() == TokenChannel::Error {
            return None;
        }

        self.index += 1;
        self.skip_trivia();
        Some(token)
    }

    fn consume(
        &mut self,
        expected: TokenKind,
        description: &'static str,
    ) -> Result<&'tokens Token<'src>, AttributeParseError> {
        let Some(token) = self.peek() else {
            return Err(self.error_here(AttributeParseErrorKind::UnexpectedEnd));
        };

        if token.kind() != expected {
            return Err(self.error_at(
                token,
                AttributeParseErrorKind::UnexpectedToken {
                    expected: description,
                    found: token.lexeme().to_owned(),
                },
            ));
        }

        self.bump().ok_or_else(|| {
            self.error_here(AttributeParseErrorKind::InvalidTokenStream)
        })
    }

    fn tick(&mut self) -> Result<(), AttributeParseError> {
        if self.constructs >= self.limits.max_constructs {
            return Err(self.error_here(
                AttributeParseErrorKind::ConstructBudgetExceeded {
                    limit: self.limits.max_constructs,
                },
            ));
        }

        self.constructs = self.constructs.saturating_add(1);
        Ok(())
    }

    fn check_depth(&self, depth: usize) -> Result<(), AttributeParseError> {
        if depth > self.limits.max_nesting {
            return Err(self.error_here(
                AttributeParseErrorKind::NestingBudgetExceeded {
                    limit: self.limits.max_nesting,
                },
            ));
        }

        Ok(())
    }

    fn parse_attribute_at_depth(
        &mut self,
        depth: usize,
    ) -> Result<Attribute, AttributeParseError> {
        self.check_depth(depth)?;
        self.tick()?;

        let at = self.consume(TokenKind::At, "`@`")?;
        let start = at.span().start_byte();
        let name = self.parse_attribute_name()?;

        let arguments = if self.peek_kind() == Some(TokenKind::LeftParen) {
            self.parse_arguments(depth.saturating_add(1))?
        } else {
            Vec::new()
        };

        let end = self
            .previous_significant_token()
            .map(|token| token.span().end_byte())
            .unwrap_or(start);

        let span = self.make_span(start, end)?;

        let attribute = if arguments.is_empty() {
            // `@name()` and `@name` have the same canonical empty-argument
            // representation. Source spelling remains available from tokens
            // and source spans when a lossless syntax tree is required.
            Attribute::flag(name, span)
        } else {
            Attribute::new(name, arguments, span)
        };

        attribute.validate().map_err(|_| {
            AttributeParseError {
                kind: AttributeParseErrorKind::InvalidName,
                span,
            }
        })?;

        Ok(attribute)
    }

    fn parse_attribute_name(&mut self) -> Result<AttributeName, AttributeParseError> {
        let first = self.consume_identifier("attribute name")?;
        let mut components = vec![first.lexeme().to_owned()];

        while self.peek_kind() == Some(TokenKind::DoubleColon) {
            self.bump();
            let segment = self.consume_identifier("qualified attribute name")?;
            components.push(segment.lexeme().to_owned());
        }

        let Some(name) = components.pop() else {
            return Err(self.error_here(AttributeParseErrorKind::InvalidName));
        };

        if components.is_empty() {
            Ok(AttributeName::new(name))
        } else {
            Ok(AttributeName::namespaced(components, name))
        }
    }

    fn parse_arguments(
        &mut self,
        depth: usize,
    ) -> Result<Vec<AttributeArgument>, AttributeParseError> {
        self.check_depth(depth)?;
        self.consume(TokenKind::LeftParen, "`(`")?;

        let mut arguments = Vec::new();

        if self.peek_kind() == Some(TokenKind::RightParen) {
            self.bump();
            return Ok(arguments);
        }

        loop {
            self.tick()?;

            let named = self.peek_kind() == Some(TokenKind::Identifier)
                && self.peek_nth(1).map(Token::kind) == Some(TokenKind::Assign);

            let argument = if named {
                let name = self.consume_identifier("argument name")?;
                self.consume(TokenKind::Assign, "`=`")?;
                let value = self.parse_value(depth.saturating_add(1))?;
                AttributeArgument::named(name.lexeme(), value)
            } else {
                AttributeArgument::positional(
                    self.parse_value(depth.saturating_add(1))?,
                )
            };

            arguments.push(argument);

            if self.peek_kind() == Some(TokenKind::Comma) {
                self.bump();

                if self.peek_kind() == Some(TokenKind::RightParen) {
                    self.bump();
                    break;
                }

                continue;
            }

            self.consume(TokenKind::RightParen, "`,` or `)`")?;
            break;
        }

        Ok(arguments)
    }

    fn parse_value(
        &mut self,
        depth: usize,
    ) -> Result<AttributeValue, AttributeParseError> {
        self.check_depth(depth)?;
        self.tick()?;

        let Some(token) = self.peek() else {
            return Err(self.error_here(AttributeParseErrorKind::UnexpectedEnd));
        };

        match token.kind() {
            TokenKind::IntegerLiteral => {
                let spelling = token.lexeme().to_owned();
                self.bump();
                Ok(AttributeValue::Integer(spelling))
            }

            TokenKind::FloatLiteral => {
                let spelling = token.lexeme().to_owned();
                self.bump();
                Ok(AttributeValue::Float(spelling))
            }

            TokenKind::StringLiteral => {
                let spelling = token.lexeme().to_owned();
                let span = self.token_span(token)?;
                self.bump();

                decode_quoted(&spelling, '"')
                    .map(AttributeValue::String)
                    .map_err(|reason| AttributeParseError {
                        kind: AttributeParseErrorKind::InvalidLiteral { reason },
                        span,
                    })
            }

            TokenKind::CharacterLiteral => {
                let spelling = token.lexeme().to_owned();
                let span = self.token_span(token)?;
                self.bump();

                let decoded = decode_quoted(&spelling, '\'').map_err(|reason| {
                    AttributeParseError {
                        kind: AttributeParseErrorKind::InvalidLiteral { reason },
                        span,
                    }
                })?;

                let mut chars = decoded.chars();
                let Some(value) = chars.next() else {
                    return Err(AttributeParseError {
                        kind: AttributeParseErrorKind::InvalidLiteral {
                            reason: "empty character literal",
                        },
                        span,
                    });
                };

                if chars.next().is_some() {
                    return Err(AttributeParseError {
                        kind: AttributeParseErrorKind::InvalidLiteral {
                            reason: "character literal must contain one Unicode scalar",
                        },
                        span,
                    });
                }

                Ok(AttributeValue::Character(value))
            }

            TokenKind::BooleanLiteral
            | TokenKind::KeywordTrue
            | TokenKind::KeywordFalse => {
                let spelling = token.lexeme();
                let span = self.token_span(token)?;

                let value = match spelling {
                    "true" => true,
                    "false" => false,
                    _ => {
                        return Err(AttributeParseError {
                            kind: AttributeParseErrorKind::InvalidLiteral {
                                reason: "unrecognized boolean spelling",
                            },
                            span,
                        });
                    }
                };

                self.bump();
                Ok(AttributeValue::Boolean(value))
            }

            TokenKind::LeftBracket => self.parse_array(depth.saturating_add(1)),
            TokenKind::LeftBrace => self.parse_object(depth.saturating_add(1)),

            TokenKind::At => {
                let nested = self.parse_attribute_at_depth(depth.saturating_add(1))?;
                Ok(AttributeValue::Nested(Box::new(nested)))
            }

            TokenKind::Identifier => self.parse_symbolic_path(),

            _ => Err(self.error_at(
                token,
                AttributeParseErrorKind::UnexpectedToken {
                    expected: "attribute value",
                    found: token.lexeme().to_owned(),
                },
            )),
        }
    }

    fn parse_symbolic_path(&mut self) -> Result<AttributeValue, AttributeParseError> {
        let first = self.consume_identifier("symbolic attribute value")?;
        let mut segments = vec![first.lexeme().to_owned()];

        while self.peek_kind() == Some(TokenKind::DoubleColon) {
            self.bump();
            let segment = self.consume_identifier("qualified name segment")?;
            segments.push(segment.lexeme().to_owned());
        }

        if segments.len() == 1 {
            Ok(AttributeValue::Identifier(segments.remove(0)))
        } else {
            Ok(AttributeValue::Path(segments))
        }
    }

    fn parse_array(
        &mut self,
        depth: usize,
    ) -> Result<AttributeValue, AttributeParseError> {
        self.check_depth(depth)?;
        self.consume(TokenKind::LeftBracket, "`[`")?;

        let mut values = Vec::new();

        if self.peek_kind() == Some(TokenKind::RightBracket) {
            self.bump();
            return Ok(AttributeValue::Array(values));
        }

        loop {
            values.push(self.parse_value(depth.saturating_add(1))?);

            if self.peek_kind() == Some(TokenKind::Comma) {
                self.bump();

                if self.peek_kind() == Some(TokenKind::RightBracket) {
                    self.bump();
                    break;
                }

                continue;
            }

            self.consume(TokenKind::RightBracket, "`,` or `]`")?;
            break;
        }

        Ok(AttributeValue::Array(values))
    }

    fn parse_object(
        &mut self,
        depth: usize,
    ) -> Result<AttributeValue, AttributeParseError> {
        self.check_depth(depth)?;
        self.consume(TokenKind::LeftBrace, "`{`")?;

        let mut entries = Vec::new();

        if self.peek_kind() == Some(TokenKind::RightBrace) {
            self.bump();
            return Ok(AttributeValue::Object(entries));
        }

        loop {
            self.tick()?;

            let key_token = self.peek().ok_or_else(|| {
                self.error_here(AttributeParseErrorKind::UnexpectedEnd)
            })?;

            let key = match key_token.kind() {
                TokenKind::Identifier => {
                    let value = key_token.lexeme().to_owned();
                    self.bump();
                    value
                }
                TokenKind::StringLiteral => {
                    let spelling = key_token.lexeme().to_owned();
                    let span = self.token_span(key_token)?;
                    self.bump();

                    decode_quoted(&spelling, '"').map_err(|reason| {
                        AttributeParseError {
                            kind: AttributeParseErrorKind::InvalidLiteral { reason },
                            span,
                        }
                    })?
                }
                _ => {
                    return Err(self.error_at(
                        key_token,
                        AttributeParseErrorKind::UnexpectedToken {
                            expected: "identifier or string map key",
                            found: key_token.lexeme().to_owned(),
                        },
                    ));
                }
            };

            self.consume(TokenKind::Assign, "`=`")?;
            let value = self.parse_value(depth.saturating_add(1))?;
            entries.push(AttributeObjectEntry::new(key, value));

            if self.peek_kind() == Some(TokenKind::Comma) {
                self.bump();

                if self.peek_kind() == Some(TokenKind::RightBrace) {
                    self.bump();
                    break;
                }

                continue;
            }

            self.consume(TokenKind::RightBrace, "`,` or `}`")?;
            break;
        }

        Ok(AttributeValue::Object(entries))
    }

    fn consume_identifier(
        &mut self,
        expected: &'static str,
    ) -> Result<&'tokens Token<'src>, AttributeParseError> {
        let Some(token) = self.peek() else {
            return Err(self.error_here(AttributeParseErrorKind::UnexpectedEnd));
        };

        if token.kind() != TokenKind::Identifier {
            return Err(self.error_at(
                token,
                AttributeParseErrorKind::UnexpectedToken {
                    expected,
                    found: token.lexeme().to_owned(),
                },
            ));
        }

        self.bump().ok_or_else(|| {
            self.error_here(AttributeParseErrorKind::InvalidTokenStream)
        })
    }

    fn previous_significant_token(&self) -> Option<&'tokens Token<'src>> {
        self.tokens[..self.index.min(self.tokens.len())]
            .iter()
            .rev()
            .find(|token| {
                !matches!(
                    token.channel(),
                    TokenChannel::Trivia | TokenChannel::Documentation
                )
            })
    }

    fn token_span(
        &self,
        token: &Token<'src>,
    ) -> Result<AstSpan, AttributeParseError> {
        AstSpan::from_usize(
            self.source_id,
            token.span().start_byte(),
            token.span().end_byte(),
        )
        .map_err(|_| AttributeParseError {
            kind: AttributeParseErrorKind::InvalidSourceSpan,
            span: span_at(self.tokens, self.index, self.source_id)
                .unwrap_or_else(|_| AstSpan::point_usize(self.source_id, 0)
                    .expect("zero is a representable source offset")),
        })
    }

    fn make_span(
        &self,
        start: usize,
        end: usize,
    ) -> Result<AstSpan, AttributeParseError> {
        AstSpan::from_usize(self.source_id, start, end).map_err(|_| {
            AttributeParseError {
                kind: AttributeParseErrorKind::InvalidSourceSpan,
                span: span_at(self.tokens, self.index, self.source_id)
                    .unwrap_or_else(|_| {
                        AstSpan::point_usize(self.source_id, 0)
                            .expect("zero is a representable source offset")
                    }),
            }
        })
    }

    fn error_here(&self, kind: AttributeParseErrorKind) -> AttributeParseError {
        AttributeParseError {
            kind,
            span: span_at(self.tokens, self.index, self.source_id)
                .unwrap_or_else(|_| {
                    AstSpan::point_usize(self.source_id, 0)
                        .expect("zero is a representable source offset")
                }),
        }
    }

    fn error_at(
        &self,
        token: &Token<'src>,
        kind: AttributeParseErrorKind,
    ) -> AttributeParseError {
        AttributeParseError {
            kind,
            span: self.token_span(token).unwrap_or_else(|_| {
                AstSpan::point_usize(self.source_id, 0)
                    .expect("zero is a representable source offset")
            }),
        }
    }
}

fn span_at(
    tokens: &[Token<'_>],
    index: usize,
    source_id: SourceId,
) -> Result<AstSpan, crate::frontend::ast::node::source::span::SpanError> {
    if let Some(token) = tokens.get(index) {
        return AstSpan::point_usize(source_id, token.span().start_byte());
    }

    let end = tokens
        .iter()
        .rev()
        .find(|token| token.channel() == TokenChannel::Default)
        .map(|token| token.span().end_byte())
        .unwrap_or(0);

    AstSpan::point_usize(source_id, end)
}

/// Decode the quoted literal forms accepted by this attribute grammar.
///
/// Numeric values deliberately remain lexical strings; strings and characters
/// are decoded because the AST represents their values, not their original
/// quote delimiters.
///
/// Unknown escapes are rejected rather than silently changing their meaning.
fn decode_quoted(
    spelling: &str,
    quote: char,
) -> Result<String, &'static str> {
    let mut chars = spelling.chars();

    if chars.next() != Some(quote) {
        return Err("missing opening quote");
    }

    let mut body = chars.collect::<Vec<_>>();
    if body.pop() != Some(quote) {
        return Err("missing closing quote");
    }

    let mut output = String::new();
    let mut index = 0usize;

    while index < body.len() {
        let ch = body[index];

        if ch != '\\' {
            output.push(ch);
            index += 1;
            continue;
        }

        index = index.saturating_add(1);
        let Some(escaped) = body.get(index).copied() else {
            return Err("incomplete escape sequence");
        };

        match escaped {
            'n' => output.push('\n'),
            'r' => output.push('\r'),
            't' => output.push('\t'),
            '0' => output.push('\0'),
            '\\' => output.push('\\'),
            '\'' => output.push('\''),
            '"' => output.push('"'),
            _ => return Err("unsupported escape sequence"),
        }

        index = index.saturating_add(1);
    }

    Ok(output)
}
