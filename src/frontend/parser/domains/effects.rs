
#![forbid(unsafe_code)]

//! Zamani frontend — source-level effect syntax.
//!
//! File: `src/frontend/parser/domains/effects.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provide a loss-preserving, target-independent parser for effect references,
//! effect sets, and effect declaration boundaries.
//!
//! # Ownership
//!
//! This module owns:
//! - source-level effect reference parsing;
//! - qualified effect names;
//! - effect-set syntax, including empty sets and trailing commas;
//! - loss-preserving effect declaration boundaries;
//! - local syntax errors and caller-controlled safety budgets.
//!
//! This module does not own:
//! - lexical recognition or token definitions;
//! - the canonical AST or AST node identity;
//! - expression, type, generic-parameter, or attribute grammar;
//! - effect resolution, inference, normalization, or authorization;
//! - capability and resource analysis;
//! - execution, scheduling, hardware discovery, or IR lowering.
//!
//! # Grammar authority
//!
//! The normative grammar remains:
//! - `grammar/effects/effects.g4`;
//! - `grammar/effects/effect-sets.g4`;
//! - `grammar/effects/effect-declarations.g4`;
//! - the imported core name, type, expression, and attribute grammars.
//!
//! This Rust module is a parser-facing adapter, not a replacement for those
//! grammar files. In particular, a declaration's signature and body are
//! preserved as token ranges for the shared grammar and AST adapter to parse.
//!
//! # Integration
//!
//! The module consumes `crate::frontend::lexer::token::Token` and the
//! canonical `TokenKind` and `Span` types. It intentionally does not depend
//! on a not-yet-frozen `ParserSession` API.
//!
//! The caller should:
//! 1. provide tokens from the canonical lexer;
//! 2. supply budgets from the shared parser configuration;
//! 3. use the returned token ranges to invoke shared type/expression/attribute
//!    productions where a complete declaration AST is required;
//! 4. convert the result through `ast_adapter.rs` into the canonical AST;
//! 5. run semantic effect analysis downstream.
//!
//! # Scalability and safety
//!
//! There are no hardware-capacity limits or enumerated effect catalogue.
//! Names such as `quantum::Measurement`, `vendor::future::Interaction`,
//! `networking::request`, and future qualified names are treated uniformly.
//!
//! The caller controls parser safety budgets. `usize::MAX` means that this
//! component applies no smaller explicit budget for that dimension; it does
//! not promise unlimited memory, address space, or execution time.
//!
//! The implementation is iterative and uses safe Rust only.

use core::fmt;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;

/// Half-open range of raw token indices in the original token stream.
///
/// Trivia tokens are included in this index space. Keeping raw ranges allows
/// downstream grammar productions and AST adapters to preserve exact source
/// structure without copying or re-tokenizing the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenRange {
    start: usize,
    end: usize,
}

impl TokenRange {
    /// Construct a range if its bounds are ordered.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Option<Self> {
        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    /// First raw token index, inclusive.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Raw token index immediately after the range.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// Number of raw tokens in this range.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Whether this range contains no tokens.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// An effect name represented by its qualified source-level segments.
///
/// This type deliberately does not resolve the name or assign it semantic
/// meaning. Duplicate references and source ordering are preserved.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectReference {
    segments: Vec<String>,
    token_range: TokenRange,
    span: Span,
}

impl EffectReference {
    /// Qualified-name segments in source order.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Token range covering the complete reference.
    #[must_use]
    pub const fn token_range(&self) -> TokenRange {
        self.token_range
    }

    /// Source span covering the complete reference.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }
}

/// A syntactic effect set.
///
/// An empty set is represented explicitly. This type does not imply purity,
/// authorization, support by a target, or successful effect checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectSet {
    references: Vec<EffectReference>,
    token_range: TokenRange,
    span: Span,
}

impl EffectSet {
    /// Effect references in source order, including duplicate references.
    #[must_use]
    pub fn references(&self) -> &[EffectReference] {
        &self.references
    }

    /// Token range including the opening and closing braces.
    #[must_use]
    pub const fn token_range(&self) -> TokenRange {
        self.token_range
    }

    /// Source span including the opening and closing braces.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Whether the source explicitly contains an empty effect set.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.references.is_empty()
    }
}

/// The source range of an effect declaration.
///
/// The complete signature and body remain in the original token stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectDeclaration {
    name: EffectReference,
    token_range: TokenRange,
    span: Span,
    body_range: Option<TokenRange>,
    has_terminator: bool,
}

impl EffectDeclaration {
    /// Declared effect name.
    #[must_use]
    pub fn name(&self) -> &EffectReference {
        &self.name
    }

    /// Token range covering the complete declaration.
    #[must_use]
    pub const fn token_range(&self) -> TokenRange {
        self.token_range
    }

    /// Source span covering the complete declaration.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Body token range, when the declaration contains a braced body.
    ///
    /// The range includes the body braces. Its contents must be parsed by
    /// the shared effect-operation grammar rather than by this boundary parser.
    #[must_use]
    pub const fn body_range(&self) -> Option<TokenRange> {
        self.body_range
    }

    /// Whether a semicolon terminates the declaration.
    #[must_use]
    pub const fn has_terminator(&self) -> bool {
        self.has_terminator
    }
}

/// Caller-controlled safety budgets for effect syntax parsing.
///
/// Supply these values from the shared parser configuration. They are
/// implementation-work budgets, not language-level effect-count limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectParseLimits {
    /// Maximum number of significant tokens this component may inspect.
    pub max_tokens: usize,

    /// Maximum number of effect references constructed in one operation.
    pub max_references: usize,

    /// Maximum delimiter nesting observed while finding declaration bounds.
    pub max_nesting: usize,
}

impl EffectParseLimits {
    /// Construct explicit caller-supplied budgets.
    #[must_use]
    pub const fn new(
        max_tokens: usize,
        max_references: usize,
        max_nesting: usize,
    ) -> Self {
        Self {
            max_tokens,
            max_references,
            max_nesting,
        }
    }

    /// No smaller explicit budgets for this module.
    ///
    /// This does not remove the process's actual memory, address-space,
    /// operating-system, or runtime constraints.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_tokens: usize::MAX,
            max_references: usize::MAX,
            max_nesting: usize::MAX,
        }
    }
}

impl Default for EffectParseLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// Stable categories of syntax failures reported by this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EffectParseErrorKind {
    /// The starting index lies beyond the token stream.
    InvalidStart,

    /// The input ended before the required syntax was complete.
    UnexpectedEnd,

    /// The current token does not match the required production.
    UnexpectedToken,

    /// A qualified name is missing a name segment.
    MissingNameSegment,

    /// A separator or delimiter was not closed correctly.
    UnclosedDelimiter,

    /// A closing delimiter has no matching opening delimiter.
    UnexpectedClosingDelimiter,

    /// A token sequence violates the effect-set grammar.
    InvalidEffectSet,

    /// The input does not begin with an effect declaration.
    ExpectedEffectDeclaration,

    /// A caller-supplied token budget was exhausted.
    TokenBudgetExceeded,

    /// A caller-supplied reference budget was exhausted.
    ReferenceBudgetExceeded,

    /// A caller-supplied nesting budget was exhausted.
    NestingBudgetExceeded,

    /// A token stream contains an explicit lexical-error token.
    LexicalError,

    /// The input span could not be composed consistently.
    InvalidSpan,
}

/// Structured effect syntax error.
///
/// `span` is absent only when no valid token position is available, such as
/// an invalid starting index beyond the input slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectParseError {
    kind: EffectParseErrorKind,
    message: String,
    span: Option<Span>,
    token_index: usize,
}

impl EffectParseError {
    fn new(
        kind: EffectParseErrorKind,
        message: impl Into<String>,
        span: Option<Span>,
        token_index: usize,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
            token_index,
        }
    }

    /// Machine-readable failure category.
    #[must_use]
    pub const fn kind(&self) -> EffectParseErrorKind {
        self.kind
    }

    /// Human-readable diagnostic detail.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Source span associated with the failure, if available.
    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        self.span
    }

    /// Raw token index where the failure was detected.
    #[must_use]
    pub const fn token_index(&self) -> usize {
        self.token_index
    }
}

impl fmt::Display for EffectParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at token {}", self.message, self.token_index)
    }
}

impl std::error::Error for EffectParseError {}

/// Parse one qualified effect reference beginning at `start`.
///
/// The returned index points just after the final name segment. Trivia between
/// segments is accepted according to the lexer's token channels.
///
/// This function does not require the reference to consume the entire stream.
pub fn parse_effect_reference<'src>(
    tokens: &[Token<'src>],
    start: usize,
    limits: EffectParseLimits,
) -> Result<(EffectReference, usize), EffectParseError> {
    let mut parser = EffectParser::new(tokens, start, limits)?;
    let reference = parser.parse_reference()?;
    Ok((reference, parser.position))
}

/// Parse one effect set, for example `{ IO, quantum::Measurement, }`.
///
/// The empty set `{}` and a trailing comma are valid. Duplicate references
/// are preserved because normalization belongs to semantic analysis.
pub fn parse_effect_set<'src>(
    tokens: &[Token<'src>],
    start: usize,
    limits: EffectParseLimits,
) -> Result<(EffectSet, usize), EffectParseError> {
    let mut parser = EffectParser::new(tokens, start, limits)?;
    let set = parser.parse_set()?;
    Ok((set, parser.position))
}

/// Parse an effect declaration boundary.
///
/// This routine validates the `effect` introducer and qualified declaration
/// name, then finds the syntactic end of the declaration. The shared grammar
/// remains responsible for attributes, visibility, generic parameters,
/// signatures, operation declarations, types, expressions, and their detailed
/// diagnostics.
///
/// The declaration ends at a top-level semicolon or the closing brace of its
/// body. Nested delimiters are balanced iteratively. The returned body range
/// includes the outer body braces.
///
/// This function intentionally preserves the original token sequence instead
/// of attempting to parse types or operation bodies a second time.
pub fn parse_effect_declaration<'src>(
    tokens: &[Token<'src>],
    start: usize,
    limits: EffectParseLimits,
) -> Result<(EffectDeclaration, usize), EffectParseError> {
    let mut parser = EffectParser::new(tokens, start, limits)?;
    parser.skip_trivia();

    let declaration_start = parser.position;
    let introducer = parser.peek_significant()?.ok_or_else(|| {
        parser.error_at(
            EffectParseErrorKind::UnexpectedEnd,
            "expected an effect declaration",
            parser.position,
        )
    })?;

    if introducer.lexeme() != "effect" {
        return Err(parser.error_at(
            EffectParseErrorKind::ExpectedEffectDeclaration,
            "expected the `effect` declaration introducer",
            parser.position,
        ));
    }

    parser.advance_significant()?;

    let name = parser.parse_reference()?;
    let mut body_range = None;
    let mut stack: Vec<TokenKind> = Vec::new();
    let mut saw_body = false;
    let mut end_index = parser.position;
    let mut has_terminator = false;

    loop {
        parser.skip_trivia();

        let index = parser.position;
        let token = match parser.peek_significant()? {
            Some(token) => token,
            None => {
                return Err(parser.error_at(
                    EffectParseErrorKind::UnexpectedEnd,
                    "effect declaration is not terminated",
                    index,
                ));
            }
        };

        if token.kind() == TokenKind::Eof {
            return Err(parser.error_at(
                EffectParseErrorKind::UnexpectedEnd,
                "effect declaration reaches end of input before termination",
                index,
            ));
        }

        parser.charge_token(index)?;

        if stack.is_empty() && token.kind() == TokenKind::Semicolon {
            parser.advance_significant()?;
            end_index = parser.position;
            has_terminator = true;
            break;
        }

        match token.kind() {
            TokenKind::LeftParen => stack.push(TokenKind::RightParen),
            TokenKind::LeftBracket => stack.push(TokenKind::RightBracket),
            TokenKind::LeftBrace => {
                if stack.is_empty() && !saw_body {
                    saw_body = true;
                }
                stack.push(TokenKind::RightBrace);
            }
            TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace => {
                let expected = stack.pop().ok_or_else(|| {
                    parser.error_at(
                        EffectParseErrorKind::UnexpectedClosingDelimiter,
                        "unexpected closing delimiter in effect declaration",
                        index,
                    )
                })?;

                if expected != token.kind() {
                    return Err(parser.error_at(
                        EffectParseErrorKind::UnclosedDelimiter,
                        "closing delimiter does not match its opening delimiter",
                        index,
                    ));
                }

                if stack.is_empty() && saw_body {
                    parser.advance_significant()?;
                    let body_end = parser.position;
                    body_range = TokenRange::new(
                        body_start_index(tokens, declaration_start, index),
                        body_end,
                    );

                    parser.skip_trivia();
                    if parser.peek_kind() == Some(TokenKind::Semicolon) {
                        parser.advance_significant()?;
                        has_terminator = true;
                    }

                    end_index = parser.position;
                    break;
                }
            }
            _ => {}
        }

        parser.advance_significant()?;
        end_index = parser.position;
    }

    if end_index <= declaration_start {
        return Err(parser.error_at(
            EffectParseErrorKind::InvalidSpan,
            "effect declaration produced an empty token range",
            declaration_start,
        ));
    }

    let span = parser.compose_span(declaration_start, end_index)?;

    let declaration = EffectDeclaration {
        name,
        token_range: TokenRange::new(declaration_start, end_index)
            .ok_or_else(|| {
                parser.error_at(
                    EffectParseErrorKind::InvalidSpan,
                    "invalid effect declaration token range",
                    declaration_start,
                )
            })?,
        span,
        body_range,
        has_terminator,
    };

    Ok((declaration, end_index))
}

struct EffectParser<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    position: usize,
    limits: EffectParseLimits,
    inspected: usize,
    references: usize,
}

impl<'tokens, 'src> EffectParser<'tokens, 'src> {
    fn new(
        tokens: &'tokens [Token<'src>],
        start: usize,
        limits: EffectParseLimits,
    ) -> Result<Self, EffectParseError> {
        if start > tokens.len() {
            return Err(EffectParseError::new(
                EffectParseErrorKind::InvalidStart,
                "starting token index is outside the token stream",
                None,
                start,
            ));
        }

        Ok(Self {
            tokens,
            position: start,
            limits,
            inspected: 0,
            references: 0,
        })
    }

    fn error_at(
        &self,
        kind: EffectParseErrorKind,
        message: impl Into<String>,
        index: usize,
    ) -> EffectParseError {
        EffectParseError::new(
            kind,
            message,
            self.tokens.get(index).map(|token| *token.span()),
            index,
        )
    }

    fn charge_token(&mut self, index: usize) -> Result<(), EffectParseError> {
        let next = self.inspected.checked_add(1).ok_or_else(|| {
            self.error_at(
                EffectParseErrorKind::TokenBudgetExceeded,
                "effect parser token accounting overflowed",
                index,
            )
        })?;

        if next > self.limits.max_tokens {
            return Err(self.error_at(
                EffectParseErrorKind::TokenBudgetExceeded,
                "effect parser token budget exceeded",
                index,
            ));
        }

        self.inspected = next;
        Ok(())
    }

    fn charge_reference(&mut self, index: usize) -> Result<(), EffectParseError> {
        let next = self.references.checked_add(1).ok_or_else(|| {
            self.error_at(
                EffectParseErrorKind::ReferenceBudgetExceeded,
                "effect parser reference accounting overflowed",
                index,
            )
        })?;

        if next > self.limits.max_references {
            return Err(self.error_at(
                EffectParseErrorKind::ReferenceBudgetExceeded,
                "effect parser reference budget exceeded",
                index,
            ));
        }

        self.references = next;
        Ok(())
    }

    fn skip_trivia(&mut self) {
        while self
            .tokens
            .get(self.position)
            .is_some_and(|token| {
                matches!(
                    token.channel(),
                    TokenChannel::Trivia | TokenChannel::Documentation
                )
            })
        {
            self.position += 1;
        }
    }

    fn peek_significant(&mut self) -> Result<Option<&'tokens Token<'src>>, EffectParseError> {
        self.skip_trivia();

        let token = self.tokens.get(self.position);
        if token.is_some_and(|token| token.is_error_channel() || token.kind() == TokenKind::Invalid)
        {
            return Err(self.error_at(
                EffectParseErrorKind::LexicalError,
                "effect syntax contains a lexical-error token",
                self.position,
            ));
        }

        Ok(token)
    }

    fn peek_kind(&mut self) -> Option<TokenKind> {
        self.skip_trivia();
        self.tokens.get(self.position).map(Token::kind)
    }

    fn advance_significant(&mut self) -> Result<(), EffectParseError> {
        self.skip_trivia();
        let index = self.position;

        let token = self.tokens.get(index).ok_or_else(|| {
            self.error_at(
                EffectParseErrorKind::UnexpectedEnd,
                "unexpected end of token stream",
                index,
            )
        })?;

        if token.kind() == TokenKind::Eof {
            return Err(self.error_at(
                EffectParseErrorKind::UnexpectedEnd,
                "cannot advance beyond end of input",
                index,
            ));
        }

        self.charge_token(index)?;
        self.position = index.checked_add(1).ok_or_else(|| {
            self.error_at(
                EffectParseErrorKind::TokenBudgetExceeded,
                "token index overflowed",
                index,
            )
        })?;

        Ok(())
    }

    fn parse_reference(&mut self) -> Result<EffectReference, EffectParseError> {
        self.skip_trivia();

        let start = self.position;
        let first = self.peek_significant()?.ok_or_else(|| {
            self.error_at(
                EffectParseErrorKind::UnexpectedEnd,
                "expected an effect name",
                self.position,
            )
        })?;

        if first.kind() != TokenKind::Identifier {
            return Err(self.error_at(
                EffectParseErrorKind::UnexpectedToken,
                "expected an identifier in the effect name",
                self.position,
            ));
        }

        self.charge_reference(start)?;

        let mut segments = Vec::new();
        let mut last_index;

        loop {
            self.skip_trivia();
            let index = self.position;

            let token = self.peek_significant()?.ok_or_else(|| {
                self.error_at(
                    EffectParseErrorKind::UnexpectedEnd,
                    "expected an effect name segment",
                    self.position,
                )
            })?;

            if token.kind() != TokenKind::Identifier {
                return Err(self.error_at(
                    EffectParseErrorKind::MissingNameSegment,
                    "expected an identifier after the qualified-name separator",
                    index,
                ));
            }

            segments.try_reserve(1).map_err(|_| {
                self.error_at(
                    EffectParseErrorKind::ReferenceBudgetExceeded,
                    "unable to reserve storage for effect-name segments",
                    index,
                )
            })?;

            segments.push(token.lexeme().to_owned());
            last_index = index;
            self.advance_significant()?;

            self.skip_trivia();
            if self.peek_kind() != Some(TokenKind::DoubleColon) {
                break;
            }

            self.advance_significant()?;
            self.skip_trivia();

            let next = self.peek_significant()?.ok_or_else(|| {
                self.error_at(
                    EffectParseErrorKind::MissingNameSegment,
                    "qualified effect name ends after `::`",
                    self.position,
                )
            })?;

            if next.kind() != TokenKind::Identifier {
                return Err(self.error_at(
                    EffectParseErrorKind::MissingNameSegment,
                    "expected an identifier after `::`",
                    self.position,
                ));
            }
        }

        let end = self.position;
        let span = self.compose_span(start, end)?;

        // Keep the local variable explicit to document that the reference
        // contains at least one consumed identifier.
        let _ = last_index;

        Ok(EffectReference {
            segments,
            token_range: TokenRange::new(start, end).ok_or_else(|| {
                self.error_at(
                    EffectParseErrorKind::InvalidSpan,
                    "invalid effect reference token range",
                    start,
                )
            })?,
            span,
        })
    }

    fn parse_set(&mut self) -> Result<EffectSet, EffectParseError> {
        self.skip_trivia();

        let start = self.position;
        let open = self.peek_significant()?.ok_or_else(|| {
            self.error_at(
                EffectParseErrorKind::UnexpectedEnd,
                "expected `{` to begin an effect set",
                self.position,
            )
        })?;

        if open.kind() != TokenKind::LeftBrace {
            return Err(self.error_at(
                EffectParseErrorKind::InvalidEffectSet,
                "expected `{` to begin an effect set",
                self.position,
            ));
        }

        self.advance_significant()?;

        let mut references = Vec::new();

        loop {
            self.skip_trivia();

            match self.peek_kind() {
                Some(TokenKind::RightBrace) => {
                    self.advance_significant()?;
                    break;
                }
                None | Some(TokenKind::Eof) => {
                    return Err(self.error_at(
                        EffectParseErrorKind::UnclosedDelimiter,
                        "effect set is missing its closing `}`",
                        self.position,
                    ));
                }
                _ => {}
            }

            let reference = self.parse_reference()?;
            references.try_reserve(1).map_err(|_| {
                self.error_at(
                    EffectParseErrorKind::ReferenceBudgetExceeded,
                    "unable to reserve storage for effect references",
                    self.position,
                )
            })?;
            references.push(reference);

            self.skip_trivia();
            match self.peek_kind() {
                Some(TokenKind::Comma) => {
                    self.advance_significant()?;

                    // A trailing comma is part of the normative effect-set
                    // grammar and is accepted before the closing brace.
                    self.skip_trivia();
                    if self.peek_kind() == Some(TokenKind::RightBrace) {
                        self.advance_significant()?;
                        break;
                    }
                }
                Some(TokenKind::RightBrace) => {
                    self.advance_significant()?;
                    break;
                }
                _ => {
                    return Err(self.error_at(
                        EffectParseErrorKind::InvalidEffectSet,
                        "expected `,` or `}` after an effect reference",
                        self.position,
                    ));
                }
            }
        }

        let end = self.position;
        let span = self.compose_span(start, end)?;

        Ok(EffectSet {
            references,
            token_range: TokenRange::new(start, end).ok_or_else(|| {
                self.error_at(
                    EffectParseErrorKind::InvalidSpan,
                    "invalid effect-set token range",
                    start,
                )
            })?,
            span,
        })
    }

    fn compose_span(
        &self,
        start: usize,
        end: usize,
    ) -> Result<Span, EffectParseError> {
        let first = self.tokens.get(start).ok_or_else(|| {
            self.error_at(
                EffectParseErrorKind::InvalidSpan,
                "cannot find the first token for a source span",
                start,
            )
        })?;

        let last_index = end.checked_sub(1).ok_or_else(|| {
            self.error_at(
                EffectParseErrorKind::InvalidSpan,
                "cannot compose a span from an empty token range",
                start,
            )
        })?;

        let last = self.tokens.get(last_index).ok_or_else(|| {
            self.error_at(
                EffectParseErrorKind::InvalidSpan,
                "cannot find the final token for a source span",
                last_index,
            )
        })?;

        let first_span = first.span();
        let last_span = last.span();

        Span::try_new(first_span.start(), last_span.end()).map_err(|_| {
            self.error_at(
                EffectParseErrorKind::InvalidSpan,
                "effect syntax spans could not be composed",
                start,
            )
        })
    }
}

/// Find the opening body brace associated with a declaration body.
///
/// This helper is deliberately lexical and does not interpret nested syntax.
/// The declaration scanner calls it only after the outer body has been
/// identified. It returns a token index, not a source offset.
fn body_start_index<'src>(
    tokens: &[Token<'src>],
    declaration_start: usize,
    closing_brace_index: usize,
) -> usize {
    let mut depth = 0usize;
    let mut candidate = declaration_start;

    for (index, token) in tokens
        .iter()
        .enumerate()
        .take(closing_brace_index.saturating_add(1))
        .skip(declaration_start)
    {
        if matches!(
            token.channel(),
            TokenChannel::Trivia | TokenChannel::Documentation
        ) {
            continue;
        }

        match token.kind() {
            TokenKind::LeftBrace => {
                if depth == 0 {
                    candidate = index;
                }
                depth = depth.saturating_add(1);
            }
            TokenKind::RightBrace => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }

    candidate
}
