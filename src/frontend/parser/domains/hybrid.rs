// src/frontend/parser/domains/hybrid.rs
//
// Zamani hybrid-domain syntax adapter.
// Rust edition: 2021
// Minimum Rust: 1.97
// Safety: safe Rust only; no unsafe code.
//
// CONTRACT
// --------
// Owns:
// - Recognition of an explicit `hybrid { ... }` region.
// - Structural classification of hybrid-region elements.
// - Validation of balanced delimiters in the region.
// - Source-span and token-range preservation.
// - Structured syntax errors with source locations.
//
// Does not own:
// - Lexing, expression precedence, general statement parsing, or type parsing.
// - The canonical AST or a second expression/statement language.
// - Type/effect/capability/resource validation.
// - Quantum-operation inventories or hardware-specific limits.
// - Hardware selection, scheduling, routing, lowering, or execution.
//
// Integration:
// - Consumes the canonical frontend lexer Token, TokenKind, TokenChannel,
//   and Span types.
// - Produces a loss-preserving syntax adapter result. The shared parser's
//   AST adapter must map it into the canonical AST when that API is available.
// - `grammar/hybrid/hybrid.g4` remains the syntax authority.
//
// Token ranges refer to the parser-visible token sequence after trivia and
// documentation-channel tokens have been excluded. Ranges are half-open.
//
// This module intentionally does not invent dependencies on parser APIs or
// hybrid AST types that are not currently established in the repository.

#![forbid(unsafe_code)]

use core::fmt;
use core::ops::Range;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;

/// The syntactic category of an element in an explicit hybrid region.
///
/// This is a syntax classification, not a semantic claim about where an
/// operation executes or which capabilities it requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HybridElementKind {
    /// A domain-qualified invocation, such as `quantum::measure(...)`.
    DomainInvocation,

    /// A binding such as `let result = expression;`.
    Binding,

    /// A hybrid control construct such as `when condition { ... }`.
    Control,

    /// A resource or capability requirement.
    Requirement,

    /// A capability declaration or capability query.
    Capability,

    /// An advisory preference.
    Preference,

    /// A constraint.
    Constraint,

    /// An advisory hint.
    Hint,

    /// A nested block-like construct.
    BlockConstruct,

    /// A statement or construct owned by another grammar domain.
    ///
    /// The shared parser is responsible for fully parsing and validating it.
    DelegatedConstruct,
}

/// A loss-preserving description of one hybrid-region element.
///
/// `token_range` is a half-open range into the parser-visible token sequence.
/// The adapter does not own those tokens. A caller that needs to revisit the
/// source tokens must retain the same token sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridElement {
    kind: HybridElementKind,
    span: Span,
    token_range: Range<usize>,
}

impl HybridElement {
    #[must_use]
    pub const fn kind(&self) -> HybridElementKind {
        self.kind
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub fn token_range(&self) -> Range<usize> {
        self.token_range.clone()
    }
}

/// A parsed `hybrid { ... }` region.
///
/// The node preserves the marker, braces, element order, and source spans.
/// It deliberately does not build a competing expression or statement AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridRegion {
    span: Span,
    marker_span: Span,
    open_brace_span: Span,
    close_brace_span: Span,
    elements: Vec<HybridElement>,
}

impl HybridRegion {
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub const fn marker_span(&self) -> Span {
        self.marker_span
    }

    #[must_use]
    pub const fn open_brace_span(&self) -> Span {
        self.open_brace_span
    }

    #[must_use]
    pub const fn close_brace_span(&self) -> Span {
        self.close_brace_span
    }

    #[must_use]
    pub fn elements(&self) -> &[HybridElement] {
        &self.elements
    }

    #[must_use]
    pub fn into_elements(self) -> Vec<HybridElement> {
        self.elements
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// Machine-readable hybrid syntax error category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HybridSyntaxErrorKind {
    /// No `hybrid` marker was found at the start of the supplied input.
    ExpectedHybridMarker,

    /// The marker was not followed by an opening brace.
    ExpectedOpeningBrace,

    /// A closing brace was missing.
    MissingClosingBrace,

    /// An opening parenthesis, bracket, or nested brace was not closed.
    UnclosedDelimiter,

    /// A closing delimiter did not match an open delimiter.
    UnexpectedClosingDelimiter,

    /// The supplied token stream contains an invalid source-span ordering.
    InvalidSourceSpan,

    /// Non-trivia tokens remain after the region.
    TrailingTokens,

    /// The token stream ended before a required construct was complete.
    UnexpectedEndOfInput,
}

/// Structured syntax error.
///
/// Human-readable messages are for diagnostics only. Callers should branch
/// on `kind()` rather than parsing the message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridSyntaxError {
    kind: HybridSyntaxErrorKind,
    message: String,
    span: Option<Span>,
}

impl HybridSyntaxError {
    #[must_use]
    pub const fn kind(&self) -> HybridSyntaxErrorKind {
        self.kind
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        self.span
    }

    fn new(
        kind: HybridSyntaxErrorKind,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
        }
    }
}

impl fmt::Display for HybridSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            Some(span) => write!(f, "{} at {:?}", self.message, span),
            None => f.write_str(&self.message),
        }
    }
}

impl std::error::Error for HybridSyntaxError {}

/// Parse one complete explicit hybrid region from canonical lexer tokens.
///
/// The input may include trivia and documentation-channel tokens; those are
/// ignored. An explicit EOF token is optional. If present, it is accepted
/// only after the region and no further non-trivia token is allowed.
///
/// This function intentionally parses only the hybrid composition boundary.
/// Expressions, types, and delegated statements remain the responsibility of
/// the shared parser. The returned token ranges allow that parser to delegate
/// each element without losing its original syntax.
///
/// # Example
///
/// ```ignore
/// let tokens = lex(source)?;
/// let region = parse_hybrid_region(&tokens)?;
/// for element in region.elements() {
///     // Delegate element.token_range() to the canonical parser.
/// }
/// ```
pub fn parse_hybrid_region(
    tokens: &[Token<'_>],
) -> Result<HybridRegion, HybridSyntaxError> {
    let visible: Vec<&Token<'_>> = tokens
        .iter()
        .filter(|token| {
            matches!(token.channel(), TokenChannel::Default | TokenChannel::Error)
        })
        .collect();

    let mut parser = HybridRegionParser {
        tokens: visible,
        position: 0,
    };

    parser.parse_region()
}

struct HybridRegionParser<'tokens, 'source> {
    tokens: Vec<&'tokens Token<'source>>,
    position: usize,
}

impl<'tokens, 'source> HybridRegionParser<'tokens, 'source> {
    fn parse_region(&mut self) -> Result<HybridRegion, HybridSyntaxError> {
        let marker_index = self.position;
        let marker = self.current().ok_or_else(|| {
            HybridSyntaxError::new(
                HybridSyntaxErrorKind::ExpectedHybridMarker,
                "expected `hybrid` region",
                None,
            )
        })?;

        if !is_word(marker, "hybrid") {
            return Err(HybridSyntaxError::new(
                HybridSyntaxErrorKind::ExpectedHybridMarker,
                "expected `hybrid` marker",
                Some(*marker.span()),
            ));
        }

        let marker_span = *marker.span();
        self.position += 1;

        let open_index = self.position;
        let open = self.current().ok_or_else(|| {
            HybridSyntaxError::new(
                HybridSyntaxErrorKind::ExpectedOpeningBrace,
                "expected `{` after `hybrid`",
                Some(marker_span),
            )
        })?;

        if !is_punctuation(open, "{") {
            return Err(HybridSyntaxError::new(
                HybridSyntaxErrorKind::ExpectedOpeningBrace,
                "expected `{` after `hybrid`",
                Some(*open.span()),
            ));
        }

        let open_brace_span = *open.span();
        self.position += 1;

        let mut elements = Vec::new();

        loop {
            let Some(token) = self.current() else {
                return Err(HybridSyntaxError::new(
                    HybridSyntaxErrorKind::MissingClosingBrace,
                    "reached end of input before closing the hybrid region",
                    Some(open_brace_span),
                ));
            };

            if token.kind() == TokenKind::Eof {
                return Err(HybridSyntaxError::new(
                    HybridSyntaxErrorKind::MissingClosingBrace,
                    "reached EOF before closing the hybrid region",
                    Some(*token.span()),
                ));
            }

            if is_punctuation(token, "}") {
                let close_brace_span = *token.span();
                self.position += 1;

                if self.current().is_some_and(|next| next.kind() != TokenKind::Eof) {
                    let next = self.current().expect("checked above");
                    return Err(HybridSyntaxError::new(
                        HybridSyntaxErrorKind::TrailingTokens,
                        "unexpected tokens after the hybrid region",
                        Some(*next.span()),
                    ));
                }

                let span = checked_join(marker_span, close_brace_span)?;

                return Ok(HybridRegion {
                    span,
                    marker_span,
                    open_brace_span,
                    close_brace_span,
                    elements,
                });
            }

            let start = self.position;
            let end = self.scan_element()?;

            if end <= start {
                let span = self.current().map(|token| *token.span());
                return Err(HybridSyntaxError::new(
                    HybridSyntaxErrorKind::UnexpectedEndOfInput,
                    "unable to make progress while parsing hybrid region",
                    span,
                ));
            }

            let first = self.tokens[start];
            let last = self.tokens[end - 1];
            let span = checked_join(*first.span(), *last.span())?;

            elements.push(HybridElement {
                kind: classify_element(&self.tokens[start..end]),
                span,
                token_range: start..end,
            });
        }
    }

    /// Consume one top-level element without consuming the hybrid region's
    /// closing brace.
    ///
    /// This scanner tracks nested delimiter structure and stops at a
    /// top-level semicolon, or after a recognized block-bearing construct.
    /// It does not attempt to parse the internal expression grammar.
    fn scan_element(&mut self) -> Result<usize, HybridSyntaxError> {
        let start = self.position;
        let block_bearing = self.current().is_some_and(is_block_bearing_start);
        let mut delimiters: Vec<Delimiter> = Vec::new();
        let mut saw_nested_block = false;

        while let Some(token) = self.current() {
            if token.kind() == TokenKind::Eof {
                return Err(HybridSyntaxError::new(
                    HybridSyntaxErrorKind::UnexpectedEndOfInput,
                    "reached EOF inside a hybrid-region element",
                    Some(*token.span()),
                ));
            }

            let spelling = token.lexeme();

            if delimiters.is_empty() && spelling == "}" {
                break;
            }

            match spelling {
                "(" => delimiters.push(Delimiter::Parenthesis(*token.span())),
                "[" => delimiters.push(Delimiter::Bracket(*token.span())),
                "{" => {
                    delimiters.push(Delimiter::Brace(*token.span()));
                    saw_nested_block = true;
                }
                ")" => close_delimiter(
                    &mut delimiters,
                    DelimiterKind::Parenthesis,
                    *token.span(),
                )?,
                "]" => close_delimiter(
                    &mut delimiters,
                    DelimiterKind::Bracket,
                    *token.span(),
                )?,
                "}" => {
                    close_delimiter(
                        &mut delimiters,
                        DelimiterKind::Brace,
                        *token.span(),
                    )?;

                    self.position += 1;

                    // A control/function/block construct may end at its
                    // matching brace without requiring a trailing semicolon.
                    if delimiters.is_empty() && block_bearing {
                        return Ok(self.position);
                    }

                    continue;
                }
                ";" if delimiters.is_empty() => {
                    self.position += 1;
                    return Ok(self.position);
                }
                _ => {}
            }

            self.position += 1;
        }

        if !delimiters.is_empty() {
            let opener = delimiters.last().map(Delimiter::span);
            return Err(HybridSyntaxError::new(
                HybridSyntaxErrorKind::UnclosedDelimiter,
                "unclosed delimiter inside hybrid-region element",
                opener,
            ));
        }

        if self.position == start {
            return Err(HybridSyntaxError::new(
                HybridSyntaxErrorKind::UnexpectedEndOfInput,
                "expected a hybrid-region element",
                self.current().map(|token| *token.span()),
            ));
        }

        // A nested block that was not part of a recognized block-bearing
        // construct may be part of an expression or declaration. Continue
        // scanning until the semicolon or region boundary.
        let _ = saw_nested_block;
        Ok(self.position)
    }

    fn current(&self) -> Option<&'tokens Token<'source>> {
        self.tokens.get(self.position).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DelimiterKind {
    Parenthesis,
    Bracket,
    Brace,
}

#[derive(Debug, Clone, Copy)]
enum Delimiter {
    Parenthesis(Span),
    Bracket(Span),
    Brace(Span),
}

impl Delimiter {
    fn kind(self) -> DelimiterKind {
        match self {
            Self::Parenthesis(_) => DelimiterKind::Parenthesis,
            Self::Bracket(_) => DelimiterKind::Bracket,
            Self::Brace(_) => DelimiterKind::Brace,
        }
    }

    fn span(self) -> Span {
        match self {
            Self::Parenthesis(span) | Self::Bracket(span) | Self::Brace(span) => span,
        }
    }
}

fn close_delimiter(
    stack: &mut Vec<Delimiter>,
    expected: DelimiterKind,
    closing_span: Span,
) -> Result<(), HybridSyntaxError> {
    let Some(opening) = stack.pop() else {
        return Err(HybridSyntaxError::new(
            HybridSyntaxErrorKind::UnexpectedClosingDelimiter,
            "closing delimiter has no matching opener",
            Some(closing_span),
        ));
    };

    if opening.kind() != expected {
        return Err(HybridSyntaxError::new(
            HybridSyntaxErrorKind::UnexpectedClosingDelimiter,
            "closing delimiter does not match the most recent opener",
            Some(closing_span),
        ));
    }

    Ok(())
}

fn classify_element(tokens: &[&Token<'_>]) -> HybridElementKind {
    let Some(first) = tokens.first().copied() else {
        return HybridElementKind::DelegatedConstruct;
    };

    if tokens.iter().any(|token| is_punctuation(token, "::"))
        && tokens.iter().any(|token| is_punctuation(token, "("))
    {
        return HybridElementKind::DomainInvocation;
    }

    match first.lexeme() {
        "let" | "var" | "const" => HybridElementKind::Binding,
        "when" | "if" | "match" => HybridElementKind::Control,
        "requires" => HybridElementKind::Requirement,
        "capability" => HybridElementKind::Capability,
        "prefer" => HybridElementKind::Preference,
        "constraint" | "constrain" => HybridElementKind::Constraint,
        "hint" => HybridElementKind::Hint,
        "fn" | "function" | "hybrid" => HybridElementKind::BlockConstruct,
        _ if tokens.iter().any(|token| is_punctuation(token, "{")) => {
            HybridElementKind::BlockConstruct
        }
        _ => HybridElementKind::DelegatedConstruct,
    }
}

fn is_block_bearing_start(token: &&Token<'_>) -> bool {
    matches!(
        token.lexeme(),
        "when" | "if" | "match" | "fn" | "function" | "hybrid" | "circuit"
    )
}

fn is_word(token: &Token<'_>, expected: &str) -> bool {
    token.lexeme() == expected
}

fn is_punctuation(token: &Token<'_>, expected: &str) -> bool {
    // Prefer canonical token identities for delimiters where available.
    // Spelling checks keep this adapter compatible with contextual names
    // and grammar-supported punctuation without duplicating lexer tables.
    match expected {
        "(" => token.kind() == TokenKind::LeftParen,
        ")" => token.kind() == TokenKind::RightParen,
        "{" => token.kind() == TokenKind::LeftBrace,
        "}" => token.kind() == TokenKind::RightBrace,
        "[" => token.kind() == TokenKind::LeftBracket,
        "]" => token.kind() == TokenKind::RightBracket,
        ";" => token.kind() == TokenKind::Semicolon,
        "," => token.kind() == TokenKind::Comma,
        ":" => token.kind() == TokenKind::Colon,
        "::" => token.kind() == TokenKind::DoubleColon,
        _ => token.lexeme() == expected,
    }
}

fn checked_join(first: Span, last: Span) -> Result<Span, HybridSyntaxError> {
    Span::try_new(first.start(), last.end()).map_err(|_| {
        HybridSyntaxError::new(
            HybridSyntaxErrorKind::InvalidSourceSpan,
            "source spans are not ordered consistently",
            Some(first),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::cursor::CursorPosition;

    fn token<'a>(
        kind: TokenKind,
        text: &'a str,
        start: usize,
        end: usize,
    ) -> Token<'a> {
        let start_position = CursorPosition::new(start, 1, start + 1);
        let end_position = CursorPosition::new(end, 1, end + 1);
        Token::borrowed(
            kind,
            text,
            Span::new(start_position, end_position),
        )
    }

    fn simple_tokens<'a>() -> Vec<Token<'a>> {
        vec![
            token(TokenKind::Identifier, "hybrid", 0, 6),
            token(TokenKind::LeftBrace, "{", 7, 8),
            token(TokenKind::Identifier, "quantum", 9, 16),
            token(TokenKind::DoubleColon, "::", 16, 18),
            token(TokenKind::Identifier, "measure", 18, 25),
            token(TokenKind::LeftParen, "(", 25, 26),
            token(TokenKind::Identifier, "state", 26, 31),
            token(TokenKind::RightParen, ")", 31, 32),
            token(TokenKind::Semicolon, ";", 32, 33),
            token(TokenKind::RightBrace, "}", 34, 35),
            token(TokenKind::Eof, "", 35, 35),
        ]
    }

    #[test]
    fn parses_empty_hybrid_region() {
        let tokens = vec![
            token(TokenKind::Identifier, "hybrid", 0, 6),
            token(TokenKind::LeftBrace, "{", 7, 8),
            token(TokenKind::RightBrace, "}", 8, 9),
            token(TokenKind::Eof, "", 9, 9),
        ];

        let parsed = parse_hybrid_region(&tokens).expect("region should parse");
        assert!(parsed.is_empty());
        assert_eq!(parsed.elements().len(), 0);
    }

    #[test]
    fn preserves_domain_invocation_and_token_range() {
        let tokens = simple_tokens();
        let parsed = parse_hybrid_region(&tokens).expect("region should parse");

        assert_eq!(parsed.elements().len(), 1);
        assert_eq!(
            parsed.elements()[0].kind(),
            HybridElementKind::DomainInvocation
        );
        assert_eq!(parsed.elements()[0].token_range(), 2..9);
    }

    #[test]
    fn rejects_missing_closing_brace() {
        let tokens = vec![
            token(TokenKind::Identifier, "hybrid", 0, 6),
            token(TokenKind::LeftBrace, "{", 7, 8),
            token(TokenKind::Identifier, "quantum", 9, 16),
        ];

        let error = parse_hybrid_region(&tokens).expect_err("must reject malformed region");
        assert_eq!(error.kind(), HybridSyntaxErrorKind::UnexpectedEndOfInput);
    }

    #[test]
    fn rejects_wrong_opening_token() {
        let tokens = vec![
            token(TokenKind::Identifier, "hybrid", 0, 6),
            token(TokenKind::Semicolon, ";", 6, 7),
        ];

        let error = parse_hybrid_region(&tokens).expect_err("must reject malformed region");
        assert_eq!(error.kind(), HybridSyntaxErrorKind::ExpectedOpeningBrace);
    }

    #[test]
    fn rejects_trailing_tokens() {
        let tokens = vec![
            token(TokenKind::Identifier, "hybrid", 0, 6),
            token(TokenKind::LeftBrace, "{", 7, 8),
            token(TokenKind::RightBrace, "}", 8, 9),
            token(TokenKind::Identifier, "unexpected", 10, 20),
        ];

        let error = parse_hybrid_region(&tokens).expect_err("must reject trailing input");
        assert_eq!(error.kind(), HybridSyntaxErrorKind::TrailingTokens);
    }

    #[test]
    fn supports_symbolic_and_domain_neutral_names() {
        let tokens = vec![
            token(TokenKind::Identifier, "hybrid", 0, 6),
            token(TokenKind::LeftBrace, "{", 7, 8),
            token(TokenKind::Identifier, "future_domain", 9, 22),
            token(TokenKind::DoubleColon, "::", 22, 24),
            token(TokenKind::Identifier, "compute", 24, 31),
            token(TokenKind::LeftParen, "(", 31, 32),
            token(TokenKind::Identifier, "input", 32, 37),
            token(TokenKind::RightParen, ")", 37, 38),
            token(TokenKind::Semicolon, ";", 38, 39),
            token(TokenKind::RightBrace, "}", 40, 41),
            token(TokenKind::Eof, "", 41, 41),
        ];

        let parsed = parse_hybrid_region(&tokens).expect("region should parse");
        assert_eq!(
            parsed.elements()[0].kind(),
            HybridElementKind::DomainInvocation
        );
    }
}