
#![forbid(unsafe_code)]

//! Zamani HDL domain parser.
//!
//! Path: src/frontend/parser/domains/hdl.rs
//! Rust: 1.97+
//! Edition: 2021
//!
//! # Authority
//!
//! Syntax and architecture must remain consistent with:
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - the HDL delegate grammars under `grammar/hdl/`
//! - `grammar/DESIGN.md`
//! - the canonical lexer and frontend AST contracts.
//!
//! # Responsibilities
//!
//! - Recognize HDL-owned declaration and statement boundaries.
//! - Preserve token spelling, kind, and source locations.
//! - Handle nested delimiters and configurable parser safety budgets.
//! - Expose one-construct-at-a-time parsing for shared-parser integration.
//! - Leave operation names, primitive names, widths, dimensions, and target
//!   identities open-ended.
//!
//! # Does not own
//!
//! - Lexing or token classification.
//! - The canonical Zamani AST.
//! - Type checking, elaboration, or name resolution.
//! - Clock-domain crossing analysis, synthesis, placement, or routing.
//! - Hardware discovery, resource allocation, or target selection.
//!
//! # Integration
//!
//! The shared parser calls `HdlDomainParser::parse_next()` when its grammar
//! dispatch identifies an HDL construct. The returned loss-preserving syntax
//! representation is then converted by the shared AST adapter.
//!
//! This module intentionally does not invent constructors for AST types
//! whose actual invariants belong to the canonical AST implementation.

use std::error::Error;
use std::fmt;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

/// Resource budgets for one HDL parser instance.
///
/// These are parser-safety budgets, not limits on the size of a hardware
/// design. They do not define maximum signal counts, port counts, widths,
/// dimensions, module counts, or target capacities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlParserLimits {
    /// Maximum nested delimiter depth while consuming one construct.
    pub max_nesting_depth: usize,

    /// Maximum significant tokens consumed by one construct.
    pub max_tokens_per_construct: usize,

    /// Whether to retain unrecognized HDL-led constructs as opaque syntax.
    ///
    /// This does not make arbitrary syntax valid. It is intended for a
    /// caller that has explicitly enabled an extension-compatible mode.
    pub allow_opaque_extensions: bool,
}

impl Default for HdlParserLimits {
    fn default() -> Self {
        Self {
            max_nesting_depth: 512,
            max_tokens_per_construct: usize::MAX,
            allow_opaque_extensions: false,
        }
    }
}

// -----------------------------------------------------------------------------
// Loss-preserving syntax representation
// -----------------------------------------------------------------------------

/// A token snapshot retained by the HDL syntax tree.
///
/// This is not a replacement lexer token. It preserves the canonical token's
/// kind, exact spelling, and source span for AST adaptation and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlSyntaxToken {
    pub kind: TokenKind,
    pub spelling: String,
    pub span: Span,
}

impl HdlSyntaxToken {
    fn from_token(token: &Token<'_>) -> Self {
        Self {
            kind: token.kind(),
            spelling: token.lexeme().to_owned(),
            span: *token.span(),
        }
    }
}

/// Broad syntactic category of an HDL construct.
///
/// These categories help parser integration and tooling. They do not
/// enumerate the set of hardware primitives, vendor operations, or targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HdlConstructKind {
    Module,
    Interface,
    Port,
    Signal,
    Wire,
    Register,
    Memory,
    Clock,
    Reset,
    Process,
    Combinational,
    Sequential,
    StateMachine,
    Pipeline,
    Protocol,
    Timing,
    Constraint,
    Verification,
    Generate,
    Instance,
    Import,
    Export,
    Declaration,
    Statement,
    Extension,
}

/// A loss-preserving HDL construct.
///
/// `header` contains the tokens before the construct's body or terminator.
/// `body` contains tokens inside a braced body, excluding its outer braces.
/// `trailing` contains tokens after the closing brace, such as a terminating
/// semicolon.
///
/// The parser deliberately preserves expressions and type syntax as tokens.
/// The shared grammar and AST adapter remain responsible for interpreting
/// those structures according to the authoritative grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlConstruct {
    pub kind: HdlConstructKind,
    pub header: Vec<HdlSyntaxToken>,
    pub body: Option<Vec<HdlSyntaxToken>>,
    pub trailing: Vec<HdlSyntaxToken>,
    pub span: Span,
}

/// Structured errors produced by the HDL domain parser.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HdlParseErrorKind {
    InvalidTokenStream,
    UnexpectedEndOfInput,
    UnexpectedClosingDelimiter,
    UnclosedDelimiter,
    NestingBudgetExceeded,
    ConstructTokenBudgetExceeded,
    InvalidConfiguration,
    UnsupportedConstruct,
    NoProgress,
}

/// An HDL parsing error with a stable kind and optional source location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlParseError {
    pub kind: HdlParseErrorKind,
    pub message: String,
    pub span: Option<Span>,
    pub expected: Option<String>,
    pub found: Option<String>,
}

impl HdlParseError {
    fn new(
        kind: HdlParseErrorKind,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
            expected: None,
            found: None,
        }
    }

    fn expected(
        kind: HdlParseErrorKind,
        expected: impl Into<String>,
        found: Option<String>,
        span: Option<Span>,
    ) -> Self {
        let expected = expected.into();
        let message = match found.as_deref() {
            Some(actual) => format!("expected {expected}, found {actual}"),
            None => format!("expected {expected}, found end of input"),
        };

        Self {
            kind,
            message,
            span,
            expected: Some(expected),
            found,
        }
    }
}

impl fmt::Display for HdlParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for HdlParseError {}

// -----------------------------------------------------------------------------
// Parser
// -----------------------------------------------------------------------------

/// Incremental HDL parser over the canonical Zamani token stream.
///
/// One instance belongs to one parser session. It borrows the token stream,
/// maintains no global mutable state, and does not re-tokenize source text.
///
/// This is an HDL domain adapter, not the full Zamani parser. It should be
/// called only at a shared-parser HDL extension point.
pub struct HdlDomainParser<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    index: usize,
    limits: HdlParserLimits,
    construct_start: usize,
}

impl<'tokens, 'src> HdlDomainParser<'tokens, 'src> {
    /// Create an HDL parser over an existing canonical token slice.
    ///
    /// Tokens in trivia channels are ignored during parsing but are not
    /// modified or removed from the original slice.
    pub fn new(
        tokens: &'tokens [Token<'src>],
        limits: HdlParserLimits,
    ) -> Result<Self, HdlParseError> {
        if limits.max_nesting_depth == 0 {
            return Err(HdlParseError::new(
                HdlParseErrorKind::InvalidConfiguration,
                "HDL nesting budget must be greater than zero",
                None,
            ));
        }

        if limits.max_tokens_per_construct == 0 {
            return Err(HdlParseError::new(
                HdlParseErrorKind::InvalidConfiguration,
                "HDL construct token budget must be greater than zero",
                None,
            ));
        }

        Ok(Self {
            tokens,
            index: 0,
            limits,
            construct_start: 0,
        })
    }

    /// Return the current raw token index.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.index
    }

    /// Restore the raw token index after a shared-parser checkpoint.
    ///
    /// The index is validated before it is accepted. This operation does
    /// not alter tokens, diagnostics, or the enclosing parser's state.
    pub fn set_position(&mut self, position: usize) -> Result<(), HdlParseError> {
        if position > self.tokens.len() {
            return Err(HdlParseError::new(
                HdlParseErrorKind::InvalidTokenStream,
                "HDL parser position lies beyond the token stream",
                None,
            ));
        }

        self.index = position;
        Ok(())
    }

    /// Parse the next construct owned by this HDL adapter.
    ///
    /// Returns `Ok(None)` if the next significant token does not start an
    /// HDL construct. This allows the shared parser to dispatch other
    /// language constructs without losing its cursor position.
    ///
    /// A successful call consumes one construct. It does not require EOF.
    pub fn parse_next(
        &mut self,
    ) -> Result<Option<HdlConstruct>, HdlParseError> {
        self.skip_trivia();

        let Some(token) = self.current() else {
            return Ok(None);
        };

        if token.kind() == TokenKind::Eof {
            return Ok(None);
        }

        let Some(kind) = classify_construct(token.lexeme()) else {
            return Ok(None);
        };

        self.construct_start = self.index;
        let construct = self.parse_construct(kind)?;
        Ok(Some(construct))
    }

    /// Parse a sequence of HDL constructs until EOF or a closing brace.
    ///
    /// This method is suitable for HDL-only regions. It does not silently
    /// consume ordinary Zamani statements.
    pub fn parse_region(
        &mut self,
    ) -> Result<Vec<HdlConstruct>, HdlParseError> {
        let mut constructs = Vec::new();

        loop {
            self.skip_trivia();

            if self.at_eof() || self.current_is("}") {
                return Ok(constructs);
            }

            let before = self.index;
            match self.parse_next()? {
                Some(construct) => constructs.push(construct),
                None => {
                    let token = self.current();
                    let found = token.map(|item| item.lexeme().to_owned());
                    return Err(HdlParseError::expected(
                        HdlParseErrorKind::UnsupportedConstruct,
                        "HDL construct",
                        found,
                        token.map(|item| *item.span()),
                    ));
                }
            }

            if self.index <= before {
                return Err(HdlParseError::new(
                    HdlParseErrorKind::NoProgress,
                    "HDL parser failed to advance while parsing a construct",
                    self.current().map(|item| *item.span()),
                ));
            }
        }
    }

    fn parse_construct(
        &mut self,
        kind: HdlConstructKind,
    ) -> Result<HdlConstruct, HdlParseError> {
        let start = self.current_span().ok_or_else(|| {
            HdlParseError::new(
                HdlParseErrorKind::UnexpectedEndOfInput,
                "expected an HDL construct",
                None,
            )
        })?;

        let mut header = Vec::new();
        let mut body = None;
        let mut trailing = Vec::new();
        let mut delimiters: Vec<&'static str> = Vec::new();
        let mut consumed = 0usize;
        let mut saw_body = false;

        loop {
            self.skip_trivia();

            let Some(token) = self.current() else {
                if !delimiters.is_empty() {
                    return Err(self.unclosed_delimiter(&delimiters, None));
                }

                if !saw_body && requires_terminator(kind) {
                    return Err(HdlParseError::new(
                        HdlParseErrorKind::UnexpectedEndOfInput,
                        "unexpected end of input inside HDL construct",
                        Some(start),
                    ));
                }

                break;
            };

            if token.kind() == TokenKind::Eof {
                if !delimiters.is_empty() {
                    return Err(self.unclosed_delimiter(
                        &delimiters,
                        Some(*token.span()),
                    ));
                }
                break;
            }

            let spelling = token.lexeme();

            // A semicolon ends a declaration or statement only when no
            // nested delimiter is currently open.
            if delimiters.is_empty() && spelling == ";" {
                if saw_body {
                    trailing.push(HdlSyntaxToken::from_token(token));
                } else {
                    header.push(HdlSyntaxToken::from_token(token));
                }
                self.bump();
                consumed = consumed.saturating_add(1);
                break;
            }

            // The first top-level opening brace starts a braced body.
            if delimiters.is_empty() && spelling == "{" && !saw_body {
                saw_body = true;
                self.bump();
                consumed = consumed.saturating_add(1);

                let body_tokens = self.consume_braced_body(&mut consumed)?;
                body = Some(body_tokens);

                // Preserve optional trailing punctuation, but do not absorb
                // the next unrelated declaration.
                self.skip_trivia();
                if self.current_is(";") {
                    if let Some(next) = self.current() {
                        trailing.push(HdlSyntaxToken::from_token(next));
                    }
                    self.bump();
                    consumed = consumed.saturating_add(1);
                }

                break;
            }

            // Closing delimiters not matched within this construct must
            // remain available to the enclosing parser.
            if delimiters.is_empty()
                && matches!(spelling, ")" | "]" | "}")
            {
                return Err(HdlParseError::new(
                    HdlParseErrorKind::UnexpectedClosingDelimiter,
                    format!("unexpected closing delimiter `{spelling}` in HDL construct"),
                    Some(*token.span()),
                ));
            }

            if delimiters.is_empty()
                && spelling == "}"
            {
                break;
            }

            update_delimiters(
                &mut delimiters,
                spelling,
                *token.span(),
            )?;

            if !saw_body {
                header.push(HdlSyntaxToken::from_token(token));
            } else {
                trailing.push(HdlSyntaxToken::from_token(token));
            }

            self.bump();
            consumed = consumed.saturating_add(1);

            self.check_token_budget(consumed)?;
            self.check_depth(&delimiters, self.current_span())?;
        }

        self.check_token_budget(consumed)?;

        if header.is_empty() && body.is_none() {
            return Err(HdlParseError::new(
                HdlParseErrorKind::UnexpectedEndOfInput,
                "HDL construct contains no syntax after its introducer",
                Some(start),
            ));
        }

        let end = trailing
            .last()
            .map(|token| token.span)
            .or_else(|| {
                body.as_ref()
                    .and_then(|tokens| tokens.last())
                    .map(|token| token.span)
            })
            .or_else(|| header.last().map(|token| token.span))
            .unwrap_or(start);

        let span = Span::try_new(start.start(), end.end()).map_err(|error| {
            HdlParseError::new(
                HdlParseErrorKind::InvalidTokenStream,
                format!("could not construct HDL source span: {error}"),
                Some(start),
            )
        })?;

        Ok(HdlConstruct {
            kind,
            header,
            body,
            trailing,
            span,
        })
    }

    fn consume_braced_body(
        &mut self,
        consumed: &mut usize,
    ) -> Result<Vec<HdlSyntaxToken>, HdlParseError> {
        let mut body = Vec::new();
        let mut depth = 1usize;

        loop {
            self.skip_trivia();

            let Some(token) = self.current() else {
                return Err(HdlParseError::new(
                    HdlParseErrorKind::UnclosedDelimiter,
                    "end of input before the HDL body was closed",
                    body.last().map(|item: &HdlSyntaxToken| item.span),
                ));
            };

            if token.kind() == TokenKind::Eof {
                return Err(HdlParseError::new(
                    HdlParseErrorKind::UnclosedDelimiter,
                    "end of input before the HDL body was closed",
                    Some(*token.span()),
                ));
            }

            let spelling = token.lexeme();

            if spelling == "{" {
                depth = depth.checked_add(1).ok_or_else(|| {
                    HdlParseError::new(
                        HdlParseErrorKind::NestingBudgetExceeded,
                        "HDL nesting depth accounting overflowed",
                        Some(*token.span()),
                    )
                })?;
                self.check_depth_value(depth, Some(*token.span()))?;
                body.push(HdlSyntaxToken::from_token(token));
                self.bump();
                *consumed = consumed.saturating_add(1);
                self.check_token_budget(*consumed)?;
                continue;
            }

            if spelling == "}" {
                self.bump();
                *consumed = consumed.saturating_add(1);

                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Ok(body);
                }

                body.push(HdlSyntaxToken::from_token(token));
                self.check_token_budget(*consumed)?;
                continue;
            }

            body.push(HdlSyntaxToken::from_token(token));
            self.bump();
            *consumed = consumed.saturating_add(1);
            self.check_token_budget(*consumed)?;
        }
    }

    fn check_depth(
        &self,
        delimiters: &[&'static str],
        span: Option<Span>,
    ) -> Result<(), HdlParseError> {
        self.check_depth_value(delimiters.len(), span)
    }

    fn check_depth_value(
        &self,
        depth: usize,
        span: Option<Span>,
    ) -> Result<(), HdlParseError> {
        if depth > self.limits.max_nesting_depth {
            return Err(HdlParseError::new(
                HdlParseErrorKind::NestingBudgetExceeded,
                "HDL nesting budget exhausted",
                span,
            ));
        }

        Ok(())
    }

    fn check_token_budget(
        &self,
        consumed: usize,
    ) -> Result<(), HdlParseError> {
        if consumed > self.limits.max_tokens_per_construct {
            return Err(HdlParseError::new(
                HdlParseErrorKind::ConstructTokenBudgetExceeded,
                "HDL construct token budget exhausted",
                self.current_span(),
            ));
        }

        Ok(())
    }

    fn unclosed_delimiter(
        &self,
        delimiters: &[&'static str],
        span: Option<Span>,
    ) -> HdlParseError {
        let expected = delimiters.last().copied().unwrap_or("closing delimiter");
        HdlParseError::expected(
            HdlParseErrorKind::UnclosedDelimiter,
            expected,
            None,
            span,
        )
    }

    fn skip_trivia(&mut self) {
        while let Some(token) = self.current() {
            if token.is_trivia_channel()
                || token.is_documentation_channel()
                || matches!(
                    token.kind(),
                    TokenKind::Whitespace
                        | TokenKind::Newline
                        | TokenKind::LineComment
                        | TokenKind::BlockComment
                )
            {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn current(&self) -> Option<&Token<'src>> {
        self.tokens.get(self.index)
    }

    fn current_span(&self) -> Option<Span> {
        self.current().map(|token| *token.span())
    }

    fn current_is(&self, spelling: &str) -> bool {
        self.current()
            .is_some_and(|token| token.lexeme() == spelling)
    }

    fn at_eof(&self) -> bool {
        self.current()
            .is_none_or(|token| token.kind() == TokenKind::Eof)
    }

    fn bump(&mut self) {
        if self.index < self.tokens.len() {
            self.index += 1;
        }
    }
}

// -----------------------------------------------------------------------------
// Grammar-independent helpers
// -----------------------------------------------------------------------------

fn classify_construct(word: &str) -> Option<HdlConstructKind> {
    // These are grammar-level introducers, not a list of supported
    // primitives. Primitive and vendor operation names remain extensible.
    Some(match word {
        "module" | "hardware" => HdlConstructKind::Module,
        "interface" => HdlConstructKind::Interface,
        "port" | "ports" => HdlConstructKind::Port,
        "signal" | "signals" => HdlConstructKind::Signal,
        "wire" | "net" => HdlConstructKind::Wire,
        "register" | "reg" => HdlConstructKind::Register,
        "memory" => HdlConstructKind::Memory,
        "clock" => HdlConstructKind::Clock,
        "reset" => HdlConstructKind::Reset,
        "process" | "always" => HdlConstructKind::Process,
        "combinational" | "comb" => HdlConstructKind::Combinational,
        "sequential" | "seq" => HdlConstructKind::Sequential,
        "state_machine" | "state" => HdlConstructKind::StateMachine,
        "pipeline" => HdlConstructKind::Pipeline,
        "protocol" => HdlConstructKind::Protocol,
        "timing" => HdlConstructKind::Timing,
        "constraint" | "constraints" => HdlConstructKind::Constraint,
        "verify" | "verification" | "property" | "assert" => {
            HdlConstructKind::Verification
        }
        "generate" => HdlConstructKind::Generate,
        "instance" | "instantiate" => HdlConstructKind::Instance,
        "import" | "use" => HdlConstructKind::Import,
        "export" => HdlConstructKind::Export,
        "declare" | "declaration" => HdlConstructKind::Declaration,
        "assign" | "drive" | "connect" => HdlConstructKind::Statement,
        _ => return None,
    })
}

fn requires_terminator(kind: HdlConstructKind) -> bool {
    matches!(
        kind,
        HdlConstructKind::Port
            | HdlConstructKind::Signal
            | HdlConstructKind::Wire
            | HdlConstructKind::Register
            | HdlConstructKind::Memory
            | HdlConstructKind::Clock
            | HdlConstructKind::Reset
            | HdlConstructKind::Constraint
            | HdlConstructKind::Verification
            | HdlConstructKind::Statement
            | HdlConstructKind::Import
            | HdlConstructKind::Export
    )
}

fn update_delimiters(
    delimiters: &mut Vec<&'static str>,
    spelling: &str,
    span: Span,
) -> Result<(), HdlParseError> {
    match spelling {
        "(" => delimiters.push(")"),
        "[" => delimiters.push("]"),
        "{" => delimiters.push("}"),
        ")" | "]" | "}" => {
            let expected = delimiters.last().copied();
            if expected != Some(spelling) {
                return Err(HdlParseError::expected(
                    HdlParseErrorKind::UnexpectedClosingDelimiter,
                    expected.unwrap_or("opening delimiter"),
                    Some(spelling.to_owned()),
                    Some(span),
                ));
            }
            delimiters.pop();
        }
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::cursor::CursorPosition;

    fn token<'a>(
        kind: TokenKind,
        spelling: &'a str,
        start: usize,
        end: usize,
    ) -> Token<'a> {
        let start_position = CursorPosition::new(start, 1, start + 1);
        let end_position = CursorPosition::new(end, 1, end + 1);
        let span = Span::try_new(start_position, end_position)
            .expect("test span should be valid");
        Token::borrowed(kind, spelling, span)
    }

    #[test]
    fn recognizes_a_module_without_hardware_assumptions() {
        let tokens = [
            token(TokenKind::Identifier, "module", 0, 6),
            token(TokenKind::Identifier, "fabric", 7, 13),
            token(TokenKind::LeftBrace, "{", 14, 15),
            token(TokenKind::Identifier, "vendor.block", 16, 28),
            token(TokenKind::Semicolon, ";", 28, 29),
            token(TokenKind::RightBrace, "}", 30, 31),
            token(TokenKind::Eof, "", 31, 31),
        ];

        let mut parser = HdlDomainParser::new(
            &tokens,
            HdlParserLimits::default(),
        )
        .expect("default limits should be valid");

        let parsed = parser
            .parse_next()
            .expect("parse should not fail")
            .expect("module should be recognized");

        assert_eq!(parsed.kind, HdlConstructKind::Module);
        assert!(parsed.body.is_some());
        assert_eq!(parsed.header[1].spelling, "fabric");
    }

    #[test]
    fn leaves_non_hdl_tokens_for_the_shared_parser() {
        let tokens = [
            token(TokenKind::Identifier, "fn", 0, 2),
            token(TokenKind::Identifier, "compute", 3, 10),
            token(TokenKind::Eof, "", 10, 10),
        ];

        let mut parser = HdlDomainParser::new(
            &tokens,
            HdlParserLimits::default(),
        )
        .expect("default limits should be valid");

        assert!(parser.parse_next().expect("parse should succeed").is_none());
        assert_eq!(parser.position(), 0);
    }

    #[test]
    fn rejects_unclosed_module_body() {
        let tokens = [
            token(TokenKind::Identifier, "module", 0, 6),
            token(TokenKind::Identifier, "fabric", 7, 13),
            token(TokenKind::LeftBrace, "{", 14, 15),
            token(TokenKind::Identifier, "signal", 16, 22),
            token(TokenKind::Eof, "", 22, 22),
        ];

        let mut parser = HdlDomainParser::new(
            &tokens,
            HdlParserLimits::default(),
        )
        .expect("default limits should be valid");

        let error = parser.parse_next().expect_err("body must be closed");
        assert_eq!(error.kind, HdlParseErrorKind::UnclosedDelimiter);
    }

    #[test]
    fn rejects_zero_nesting_budget() {
        let result = HdlDomainParser::new(
            &[],
            HdlParserLimits {
                max_nesting_depth: 0,
                ..HdlParserLimits::default()
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_construct_budget() {
        let result = HdlDomainParser::new(
            &[],
            HdlParserLimits {
                max_tokens_per_construct: 0,
                ..HdlParserLimits::default()
            },
        );

        assert!(result.is_err());
    }
}
