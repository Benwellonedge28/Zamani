
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Contract-domain syntax adapter for Zamani.
//!
//! Path: src/frontend/parser/domains/contracts.rs
//! Rust: 1.97+
//! Edition: 2021
//!
//! # Grammar authority
//!
//! Standalone contract statements are defined by
//! `grammar/statements/contract.g4`.
//!
//! Function-attached contract blocks are defined by
//! `grammar/functions/contracts.g4`, and distributed contract declarations
//! are defined by `grammar/distributed/contracts.g4`.
//!
//! The canonical lexical vocabulary and punctuation are owned by
//! `grammar/lexer/keywords.g4` and `grammar/core/punctuation.g4`.
//! Contract conditions use the shared expression grammar.
//!
//! This module is an intermediate syntax adapter. It does not replace the
//! authoritative grammar, general expression parser, canonical AST, or
//! semantic contract model.
//!
//! # Responsibilities
//!
//! - Recognize the six standalone contract statement forms.
//! - Preserve condition tokens, token order, token kinds, and spans.
//! - Validate the outer condition delimiters and statement terminator.
//! - Detect malformed delimiters and malformed token streams.
//! - Enforce optional operational parsing budgets.
//! - Provide deterministic, structured errors.
//!
//! # Does not own
//!
//! - Lexing or keyword-token registration.
//! - General expression parsing or operator precedence.
//! - Function-contract blocks or distributed contract declarations.
//! - Canonical AST construction or source-file identity.
//! - Name/type/effect/capability/resource resolution.
//! - Contract evaluation, proof, verification, or policy enforcement.
//! - Hardware discovery, resource allocation, lowering, or execution.
//!
//! # Integration contract
//!
//! - Consumes `lexer::token::Token`.
//! - Reuses `lexer::token_kind::TokenKind` and `lexer::span::Span`.
//! - The parent grammar invokes `ContractParser::parse_next` only where a
//!   standalone contract statement is syntactically permitted.
//! - `parse_all` is for isolated contract-statement streams and requires EOF.
//! - `domains/mod.rs` exports this module.
//! - `ast_adapter.rs` converts the syntax records into canonical AST nodes.
//! - The shared expression parser validates `condition_tokens` and constructs
//!   the expression node.
//! - `session.rs` coordinates these local budgets with the global parse budget.
//! - Semantic analysis determines the meaning and validity of each condition.
//!
//! This adapter intentionally does not parse `contract { ... }` blocks.
//! Those belong to the function-declaration grammar and its parser handler.
//!
//! # Scalability and safety
//!
//! There are no language-level limits on contract count, expression length,
//! computational-domain count, or hardware capacity. Configurable limits are
//! implementation safeguards only. `usize::MAX` means no explicit local
//! budget; it does not imply infinite memory, address space, or execution time.
//!
//! No unsafe Rust, global mutable state, I/O, or target-specific behavior
//! is required.

use core::fmt;
use std::error::Error;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

/// Operational safeguards for standalone contract parsing.
///
/// These settings constrain parser work, not the semantics or capacity of a
/// Zamani program. The embedding parser should reconcile them with its
/// shared parsing budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractParserLimits {
    /// Maximum significant tokens retained for one contract statement.
    ///
    /// The introducer, condition delimiters, condition tokens, and
    /// terminator count toward this budget.
    pub max_tokens_per_clause: usize,

    /// Maximum simultaneous delimiter nesting, including the required
    /// outer condition parentheses.
    pub max_nesting_depth: usize,

    /// Maximum number of statements accepted by this parser instance.
    pub max_clauses: usize,
}

impl Default for ContractParserLimits {
    fn default() -> Self {
        Self {
            max_tokens_per_clause: usize::MAX,
            max_nesting_depth: usize::MAX,
            max_clauses: usize::MAX,
        }
    }
}

// -----------------------------------------------------------------------------
// Loss-preserving syntax records
// -----------------------------------------------------------------------------

/// Owned snapshot of a canonical lexer token.
///
/// This is not a second token model. It preserves the lexical identity,
/// original spelling, and source span for AST adaptation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractSyntaxToken {
    /// Canonical lexer token kind.
    pub kind: TokenKind,

    /// Exact token spelling, without normalization.
    pub spelling: String,

    /// Canonical source span.
    pub span: Span,
}

impl ContractSyntaxToken {
    fn from_token(token: &Token<'_>) -> Self {
        Self {
            kind: token.kind(),
            spelling: token.lexeme().to_owned(),
            span: *token.span(),
        }
    }
}

/// Syntactic category of a standalone contract statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ContractClauseKind {
    /// `requires(condition);`
    Requires,

    /// `ensures(condition);`
    Ensures,

    /// `invariant(condition);`
    Invariant,

    /// `assume(condition);`
    Assume,

    /// `guarantee(condition);`
    Guarantee,

    /// `property(condition);`
    Property,
}

impl ContractClauseKind {
    /// Classify a contract introducer by its source spelling.
    ///
    /// Matching is ASCII case-insensitive to support profiles that choose
    /// case-insensitive keyword recognition. The original spelling remains
    /// available in the retained token sequence.
    ///
    /// Keyword spelling remains governed by the active language grammar and
    /// lexer configuration. This helper does not reserve new keywords.
    #[must_use]
    pub fn from_introducer(introducer: &str) -> Option<Self> {
        if introducer.eq_ignore_ascii_case("requires") {
            Some(Self::Requires)
        } else if introducer.eq_ignore_ascii_case("ensures") {
            Some(Self::Ensures)
        } else if introducer.eq_ignore_ascii_case("invariant") {
            Some(Self::Invariant)
        } else if introducer.eq_ignore_ascii_case("assume") {
            Some(Self::Assume)
        } else if introducer.eq_ignore_ascii_case("guarantee") {
            Some(Self::Guarantee)
        } else if introducer.eq_ignore_ascii_case("property") {
            Some(Self::Property)
        } else {
            None
        }
    }

    /// Return the canonical display label for diagnostics and tooling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requires => "requires",
            Self::Ensures => "ensures",
            Self::Invariant => "invariant",
            Self::Assume => "assume",
            Self::Guarantee => "guarantee",
            Self::Property => "property",
        }
    }
}

/// One standalone contract statement.
///
/// `tokens` retains the introducer, condition delimiters, condition tokens,
/// and required statement terminator. Trivia is omitted, consistent with
/// parser-facing token consumption.
///
/// `condition_tokens` contains the tokens inside the outer condition
/// parentheses. It is a syntax-preserving hand-off to the shared expression
/// parser, not a parsed or semantically validated expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractClause {
    /// Contract statement category.
    pub kind: ContractClauseKind,

    /// Retained tokens for the complete statement.
    pub tokens: Vec<ContractSyntaxToken>,

    /// Tokens inside the outer condition parentheses.
    pub condition_tokens: Vec<ContractSyntaxToken>,

    /// Span from the introducer through the statement terminator.
    pub span: Span,

    /// Span covering the condition expression tokens.
    pub condition_span: Span,
}

impl ContractClause {
    /// Return the original introducer spelling, if present.
    #[must_use]
    pub fn introducer(&self) -> Option<&str> {
        self.tokens.first().map(|token| token.spelling.as_str())
    }

    /// Return the condition token spellings concatenated without whitespace.
    ///
    /// This is a debugging convenience, not a formatter or source
    /// reconstruction API. The lexer may have discarded trivia.
    #[must_use]
    pub fn compact_condition(&self) -> String {
        let capacity = self.condition_tokens.iter().fold(
            0usize,
            |total, token| total.saturating_add(token.spelling.len()),
        );

        let mut output = String::with_capacity(capacity);
        for token in &self.condition_tokens {
            output.push_str(&token.spelling);
        }
        output
    }
}

/// One parsed standalone contract statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractItem {
    /// A standalone contract clause.
    Clause(ContractClause),
}

// -----------------------------------------------------------------------------
// Structured errors
// -----------------------------------------------------------------------------

/// Machine-readable contract parser failure category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ContractParseErrorKind {
    /// The stream is empty, lacks EOF, or has invalid EOF placement.
    InvalidTokenStream,

    /// A token is lexically invalid or explicitly marked as an error.
    InvalidToken,

    /// The current token is not a recognized contract introducer.
    UnexpectedToken,

    /// The required opening parenthesis is missing.
    MissingOpeningDelimiter,

    /// The condition has no significant tokens.
    EmptyCondition,

    /// A closing delimiter has no matching opening delimiter.
    UnmatchedClosingDelimiter,

    /// A closing delimiter does not match the latest opening delimiter.
    MismatchedDelimiter,

    /// EOF occurs before the outer condition parentheses close.
    UnclosedDelimiter,

    /// The required statement terminator is missing.
    MissingTerminator,

    /// The configured token budget for a statement was exceeded.
    TokenBudgetExceeded,

    /// The configured nesting budget was exceeded.
    NestingBudgetExceeded,

    /// The configured statement-count budget was exceeded.
    ClauseCountBudgetExceeded,

    /// A source span could not be composed consistently.
    InvalidSpan,

    /// A parse operation did not advance when progress was required.
    NoProgress,
}

/// Structured parser error with an optional source location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractParseError {
    /// Machine-readable classification.
    pub kind: ContractParseErrorKind,

    /// Human-readable diagnostic text.
    pub message: String,

    /// Best available source span.
    pub span: Option<Span>,

    /// Expected syntax, when applicable.
    pub expected: Option<String>,

    /// Actual syntax, when applicable.
    pub found: Option<String>,
}

impl ContractParseError {
    fn new(
        kind: ContractParseErrorKind,
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

    fn with_expected(
        mut self,
        expected: impl Into<String>,
        found: Option<String>,
    ) -> Self {
        self.expected = Some(expected.into());
        self.found = found;
        self
    }
}

impl fmt::Display for ContractParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ContractParseError {}

// -----------------------------------------------------------------------------
// Parser
// -----------------------------------------------------------------------------

/// Incremental parser for standalone contract statements.
///
/// The input must contain exactly one EOF token at the end. Each successful
/// `parse_next` consumes one complete statement. A failed `parse_next` rolls
/// the cursor back to the first significant token of that statement, allowing
/// the parent parser to choose its own recovery strategy.
///
/// This parser validates contract statement structure; it delegates general
/// expression syntax to the shared expression parser.
#[derive(Debug)]
pub struct ContractParser<'tokens, 'source> {
    tokens: &'tokens [Token<'source>],
    index: usize,
    eof_index: usize,
    limits: ContractParserLimits,
    parsed_clauses: usize,
}

impl<'tokens, 'source> ContractParser<'tokens, 'source> {
    /// Validate a token stream and create a parser at its beginning.
    pub fn new(
        tokens: &'tokens [Token<'source>],
        limits: ContractParserLimits,
    ) -> Result<Self, ContractParseError> {
        if tokens.is_empty() {
            return Err(ContractParseError::new(
                ContractParseErrorKind::InvalidTokenStream,
                "contract parser requires a final EOF token",
                None,
            ));
        }

        let mut eof_index = None;

        for (index, token) in tokens.iter().enumerate() {
            if token.kind() == TokenKind::Eof {
                if eof_index.is_some() || index + 1 != tokens.len() {
                    return Err(ContractParseError::new(
                        ContractParseErrorKind::InvalidTokenStream,
                        "token stream must contain exactly one EOF token at the end",
                        Some(*token.span()),
                    ));
                }
                eof_index = Some(index);
            }
        }

        let eof_index = eof_index.ok_or_else(|| {
            ContractParseError::new(
                ContractParseErrorKind::InvalidTokenStream,
                "token stream is missing its final EOF token",
                tokens.last().map(|token| *token.span()),
            )
        })?;

        Ok(Self {
            tokens,
            index: 0,
            eof_index,
            limits,
            parsed_clauses: 0,
        })
    }

    /// Return the current raw token index.
    ///
    /// Trivia skipped by this parser still contributes to the raw index.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.index
    }

    /// Return whether only trivia and EOF remain.
    #[must_use]
    pub fn is_eof(&self) -> bool {
        self.peek_significant_index(self.index) >= self.eof_index
    }

    /// Parse one standalone contract statement, or return `None` at EOF.
    ///
    /// This method does not accept function-attached `contract { ... }`
    /// blocks. Those are dispatched by the function grammar.
    pub fn parse_next(
        &mut self,
    ) -> Result<Option<ContractItem>, ContractParseError> {
        self.skip_trivia();

        if self.index >= self.eof_index {
            return Ok(None);
        }

        let start_index = self.index;

        if self.parsed_clauses >= self.limits.max_clauses {
            return Err(ContractParseError::new(
                ContractParseErrorKind::ClauseCountBudgetExceeded,
                "configured contract statement budget exceeded",
                self.tokens.get(self.index).map(|token| *token.span()),
            ));
        }

        match self.parse_clause_inner() {
            Ok(clause) => {
                self.parsed_clauses = match self.parsed_clauses.checked_add(1) {
                    Some(count) => count,
                    None => {
                        self.index = start_index;
                        return Err(ContractParseError::new(
                            ContractParseErrorKind::ClauseCountBudgetExceeded,
                            "contract statement counter overflowed",
                            Some(clause.span),
                        ));
                    }
                };

                Ok(Some(ContractItem::Clause(clause)))
            }
            Err(error) => {
                // Transactional failure: leave recovery to the parent parser.
                self.index = start_index;
                Err(error)
            }
        }
    }

    /// Parse all statements in a token stream containing only contract
    /// statements and EOF.
    ///
    /// For normal source parsing, prefer `parse_next` and let the parent
    /// grammar dispatch among statements and declarations.
    pub fn parse_all(
        &mut self,
    ) -> Result<Vec<ContractItem>, ContractParseError> {
        let mut items = Vec::new();

        loop {
            self.skip_trivia();

            if self.index >= self.eof_index {
                break;
            }

            let before = self.index;
            let item = self.parse_next()?.ok_or_else(|| {
                ContractParseError::new(
                    ContractParseErrorKind::NoProgress,
                    "contract parser stopped before EOF without a result",
                    self.tokens.get(before).map(|token| *token.span()),
                )
            })?;

            items.push(item);

            if self.index <= before {
                return Err(ContractParseError::new(
                    ContractParseErrorKind::NoProgress,
                    "contract parser did not advance after parsing a statement",
                    self.tokens.get(before).map(|token| *token.span()),
                ));
            }
        }

        Ok(items)
    }

    // -------------------------------------------------------------------------
    // Statement parsing
    // -------------------------------------------------------------------------

    fn parse_clause_inner(&mut self) -> Result<ContractClause, ContractParseError> {
        let first = self.tokens.get(self.index).ok_or_else(|| {
            ContractParseError::new(
                ContractParseErrorKind::UnexpectedToken,
                "expected a contract statement",
                None,
            )
        })?;

        Self::reject_invalid_token(first)?;

        let introducer = first.lexeme();
        let kind = ContractClauseKind::from_introducer(introducer).ok_or_else(|| {
            ContractParseError::new(
                ContractParseErrorKind::UnexpectedToken,
                format!(
                    "expected requires, ensures, invariant, assume, guarantee, or property; found `{introducer}`"
                ),
                Some(*first.span()),
            )
            .with_expected(
                "requires | ensures | invariant | assume | guarantee | property",
                Some(introducer.to_owned()),
            )
        })?;

        let mut tokens = Vec::new();
        let mut condition_tokens = Vec::new();

        self.push_current(&mut tokens)?;
        self.index += 1;
        self.skip_trivia();

        let opening = self.tokens.get(self.index).ok_or_else(|| {
            ContractParseError::new(
                ContractParseErrorKind::MissingOpeningDelimiter,
                format!("expected `(` after `{introducer}`"),
                Some(*first.span()),
            )
        })?;

        Self::reject_invalid_token(opening)?;

        if opening.kind() != TokenKind::LeftParen {
            return Err(ContractParseError::new(
                ContractParseErrorKind::MissingOpeningDelimiter,
                format!("expected `(` after `{introducer}`"),
                Some(*opening.span()),
            )
            .with_expected("(", Some(opening.lexeme().to_owned())));
        }

        if self.limits.max_nesting_depth == 0 {
            return Err(ContractParseError::new(
                ContractParseErrorKind::NestingBudgetExceeded,
                "configured contract nesting budget does not permit the required condition parentheses",
                Some(*opening.span()),
            ));
        }

        self.push_current(&mut tokens)?;
        self.index += 1;

        // The outer condition parenthesis is already open.
        let mut delimiters = vec![TokenKind::LeftParen];

        loop {
            self.skip_trivia();

            if self.index >= self.eof_index {
                return Err(ContractParseError::new(
                    ContractParseErrorKind::UnclosedDelimiter,
                    format!("contract `{}` condition is missing `)`", kind.as_str()),
                    tokens.last().map(|token| token.span),
                ));
            }

            let token = &self.tokens[self.index];
            Self::reject_invalid_token(token)?;

            let token_kind = token.kind();

            if Self::is_close_delimiter(token_kind) {
                let expected_open = Self::matching_open(token_kind);

                match delimiters.last().copied() {
                    None => {
                        return Err(ContractParseError::new(
                            ContractParseErrorKind::UnmatchedClosingDelimiter,
                            format!("unmatched closing delimiter `{}`", token.lexeme()),
                            Some(*token.span()),
                        ));
                    }
                    Some(open) if Some(open) != expected_open => {
                        return Err(ContractParseError::new(
                            ContractParseErrorKind::MismatchedDelimiter,
                            format!(
                                "closing delimiter `{}` does not match the preceding opening delimiter",
                                token.lexeme()
                            ),
                            Some(*token.span()),
                        ));
                    }
                    Some(_) => {}
                }

                // The closing delimiter for the outer condition is not part
                // of the condition expression itself.
                if delimiters.len() == 1 {
                    if condition_tokens.is_empty() {
                        return Err(ContractParseError::new(
                            ContractParseErrorKind::EmptyCondition,
                            format!("contract `{}` requires a non-empty condition", kind.as_str()),
                            Some(*token.span()),
                        ));
                    }

                    self.push_current(&mut tokens)?;
                    self.index += 1;
                    delimiters.pop();
                    break;
                }

                self.push_current(&mut tokens)?;
                condition_tokens.push(ContractSyntaxToken::from_token(token));
                delimiters.pop();
                self.index += 1;
                continue;
            }

            if Self::is_open_delimiter(token_kind) {
                if delimiters.len() >= self.limits.max_nesting_depth {
                    return Err(ContractParseError::new(
                        ContractParseErrorKind::NestingBudgetExceeded,
                        "configured contract delimiter nesting budget exceeded",
                        Some(*token.span()),
                    ));
                }

                self.push_current(&mut tokens)?;
                condition_tokens.push(ContractSyntaxToken::from_token(token));
                delimiters.push(token_kind);
                self.index += 1;
                continue;
            }

            self.push_current(&mut tokens)?;
            condition_tokens.push(ContractSyntaxToken::from_token(token));
            self.index += 1;
        }

        self.skip_trivia();

        let terminator = self.tokens.get(self.index).ok_or_else(|| {
            ContractParseError::new(
                ContractParseErrorKind::MissingTerminator,
                format!("expected `;` after `{}` condition", kind.as_str()),
                tokens.last().map(|token| token.span),
            )
        })?;

        Self::reject_invalid_token(terminator)?;

        if terminator.kind() != TokenKind::Semicolon {
            return Err(ContractParseError::new(
                ContractParseErrorKind::MissingTerminator,
                format!("expected `;` after `{}` condition", kind.as_str()),
                Some(*terminator.span()),
            )
            .with_expected(";", Some(terminator.lexeme().to_owned())));
        }

        self.push_current(&mut tokens)?;
        self.index += 1;

        let span = Self::compose_span(&tokens)?;
        let condition_span = Self::compose_span(&condition_tokens)?;

        Ok(ContractClause {
            kind,
            tokens,
            condition_tokens,
            span,
            condition_span,
        })
    }

    fn push_current(
        &self,
        destination: &mut Vec<ContractSyntaxToken>,
    ) -> Result<(), ContractParseError> {
        let token = self.tokens.get(self.index).ok_or_else(|| {
            ContractParseError::new(
                ContractParseErrorKind::NoProgress,
                "contract parser attempted to consume a token outside the stream",
                None,
            )
        })?;

        if destination.len() >= self.limits.max_tokens_per_clause {
            return Err(ContractParseError::new(
                ContractParseErrorKind::TokenBudgetExceeded,
                "configured token budget for one contract statement exceeded",
                Some(*token.span()),
            ));
        }

        destination.push(ContractSyntaxToken::from_token(token));
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Token and span helpers
    // -------------------------------------------------------------------------

    fn skip_trivia(&mut self) {
        while self.index < self.eof_index && Self::is_trivia(&self.tokens[self.index]) {
            self.index += 1;
        }
    }

    fn peek_significant_index(&self, mut index: usize) -> usize {
        while index < self.eof_index && Self::is_trivia(&self.tokens[index]) {
            index += 1;
        }
        index
    }

    fn is_trivia(token: &Token<'_>) -> bool {
        token.is_trivia_channel()
            || matches!(
                token.kind(),
                TokenKind::Whitespace
                    | TokenKind::Newline
                    | TokenKind::LineComment
                    | TokenKind::BlockComment
            )
    }

    fn reject_invalid_token(token: &Token<'_>) -> Result<(), ContractParseError> {
        if token.kind() == TokenKind::Invalid || token.is_error_channel() {
            return Err(ContractParseError::new(
                ContractParseErrorKind::InvalidToken,
                format!("invalid token `{}` in contract statement", token.lexeme()),
                Some(*token.span()),
            ));
        }

        Ok(())
    }

    fn is_open_delimiter(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::LeftParen | TokenKind::LeftBracket | TokenKind::LeftBrace
        )
    }

    fn is_close_delimiter(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace
        )
    }

    fn matching_open(close: TokenKind) -> Option<TokenKind> {
        match close {
            TokenKind::RightParen => Some(TokenKind::LeftParen),
            TokenKind::RightBracket => Some(TokenKind::LeftBracket),
            TokenKind::RightBrace => Some(TokenKind::LeftBrace),
            _ => None,
        }
    }

    fn compose_span(
        tokens: &[ContractSyntaxToken],
    ) -> Result<Span, ContractParseError> {
        let first = tokens.first().ok_or_else(|| {
            ContractParseError::new(
                ContractParseErrorKind::InvalidSpan,
                "cannot compose a contract span from an empty token sequence",
                None,
            )
        })?;

        let last = tokens.last().ok_or_else(|| {
            ContractParseError::new(
                ContractParseErrorKind::InvalidSpan,
                "cannot compose a contract span from an empty token sequence",
                Some(first.span),
            )
        })?;

        Span::try_new(first.span.start(), last.span.end()).map_err(|error| {
            ContractParseError::new(
                ContractParseErrorKind::InvalidSpan,
                format!("cannot compose contract source span: {error}"),
                Some(first.span),
            )
        })
    }
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        ContractClauseKind, ContractItem, ContractParseErrorKind, ContractParser,
        ContractParserLimits,
    };
    use crate::frontend::lexer::span::Span;
    use crate::frontend::lexer::token::Token;
    use crate::frontend::lexer::token_kind::TokenKind;

    fn tokenize_for_test(source: &str) -> Vec<Token<'_>> {
        let mut tokens = Vec::new();
        let mut offset = 0usize;

        while offset < source.len() {
            let rest = &source[offset..];
            let Some(ch) = rest.chars().next() else {
                break;
            };

            if ch.is_whitespace() {
                offset += ch.len_utf8();
                continue;
            }

            if ch == '"' || ch == '\'' {
                let quote = ch;
                let start = offset;
                offset += ch.len_utf8();
                let mut escaped = false;

                while offset < source.len() {
                    let next = source[offset..]
                        .chars()
                        .next()
                        .expect("offset remains on a character boundary");
                    offset += next.len_utf8();

                    if escaped {
                        escaped = false;
                    } else if next == '\\' {
                        escaped = true;
                    } else if next == quote {
                        break;
                    }
                }

                let kind = if quote == '"' {
                    TokenKind::StringLiteral
                } else {
                    TokenKind::CharacterLiteral
                };

                tokens.push(Token::borrowed(
                    kind,
                    &source[start..offset],
                    Span::default(),
                ));
                continue;
            }

            let punctuation = match ch {
                '(' => Some(TokenKind::LeftParen),
                ')' => Some(TokenKind::RightParen),
                '[' => Some(TokenKind::LeftBracket),
                ']' => Some(TokenKind::RightBracket),
                '{' => Some(TokenKind::LeftBrace),
                '}' => Some(TokenKind::RightBrace),
                ';' => Some(TokenKind::Semicolon),
                ',' => Some(TokenKind::Comma),
                _ => None,
            };

            if let Some(kind) = punctuation {
                let start = offset;
                offset += ch.len_utf8();
                tokens.push(Token::borrowed(
                    kind,
                    &source[start..offset],
                    Span::default(),
                ));
                continue;
            }

            let start = offset;
            offset += ch.len_utf8();

            while offset < source.len() {
                let next = source[offset..]
                    .chars()
                    .next()
                    .expect("offset remains on a character boundary");

                if next.is_whitespace()
                    || matches!(next, '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' | '"' | '\'')
                {
                    break;
                }

                offset += next.len_utf8();
            }

            let spelling = &source[start..offset];
            let kind = match spelling {
                "true" | "false" => TokenKind::BooleanLiteral,
                _ => TokenKind::Identifier,
            };

            tokens.push(Token::borrowed(kind, spelling, Span::default()));
        }

        tokens.push(Token::synthetic(TokenKind::Eof, Span::default()));
        tokens
    }

    fn parse_one(source: &str) -> Result<ContractItem, super::ContractParseError> {
        let tokens = tokenize_for_test(source);
        let mut parser =
            ContractParser::new(&tokens, ContractParserLimits::default())?;

        parser.parse_next()?.ok_or_else(|| {
            super::ContractParseError {
                kind: ContractParseErrorKind::NoProgress,
                message: "expected one contract statement".to_owned(),
                span: None,
                expected: None,
                found: None,
            }
        })
    }

    #[test]
    fn parses_all_six_canonical_statement_kinds() {
        let cases = [
            ("requires(x > 0);", ContractClauseKind::Requires),
            ("ensures(result >= 0);", ContractClauseKind::Ensures),
            ("invariant(state.is_valid());", ContractClauseKind::Invariant),
            ("assume(input.is_normalized());", ContractClauseKind::Assume),
            ("guarantee(output.is_valid());", ContractClauseKind::Guarantee),
            ("property(result.is_consistent());", ContractClauseKind::Property),
        ];

        for (source, expected_kind) in cases {
            let item = parse_one(source).expect("valid contract syntax");
            let ContractItem::Clause(clause) = item;
            assert_eq!(clause.kind, expected_kind, "{source}");
            assert!(!clause.condition_tokens.is_empty(), "{source}");
        }
    }

    #[test]
    fn preserves_cross_domain_conditions_without_target_assumptions() {
        let item = parse_one(
            r#"requires(capability("quantum.measurement"));"#,
        )
        .expect("valid condition");

        let ContractItem::Clause(clause) = item;
        assert_eq!(clause.kind, ContractClauseKind::Requires);
        assert!(clause.compact_condition().contains("quantum.measurement"));

        let item = parse_one("requires(memory >= required_memory);")
            .expect("valid resource condition");
        let ContractItem::Clause(clause) = item;
        assert!(clause.compact_condition().contains("required_memory"));
    }

    #[test]
    fn parses_nested_delimiters_iteratively() {
        let item = parse_one(
            "invariant(state.is_valid((left + right), [index]));",
        )
        .expect("balanced nested condition");

        let ContractItem::Clause(clause) = item;
        assert_eq!(clause.kind, ContractClauseKind::Invariant);
    }

    #[test]
    fn rejects_empty_condition() {
        let error = parse_one("requires();").expect_err("condition must not be empty");
        assert_eq!(error.kind, ContractParseErrorKind::EmptyCondition);
    }

    #[test]
    fn rejects_missing_terminator() {
        let error =
            parse_one("ensures(result >= 0)").expect_err("semicolon is required");
        assert_eq!(error.kind, ContractParseErrorKind::MissingTerminator);
    }

    #[test]
    fn rejects_unknown_introducer() {
        let error = parse_one("contract { requires(x); }")
            .expect_err("function contract blocks are not standalone statements");
        assert_eq!(error.kind, ContractParseErrorKind::UnexpectedToken);
    }

    #[test]
    fn rejects_mismatched_delimiters() {
        let error = parse_one("requires(items[0));")
            .expect_err("mismatched delimiters must be rejected");
        assert_eq!(error.kind, ContractParseErrorKind::MismatchedDelimiter);
    }

    #[test]
    fn rejects_missing_closing_delimiter() {
        let error = parse_one("requires((x > 0);")
            .expect_err("outer condition must close");
        assert_eq!(error.kind, ContractParseErrorKind::UnclosedDelimiter);
    }

    #[test]
    fn enforces_token_budget() {
        let tokens = tokenize_for_test("requires(x > 0);");
        let limits = ContractParserLimits {
            max_tokens_per_clause: 2,
            ..ContractParserLimits::default()
        };

        let mut parser = ContractParser::new(&tokens, limits).expect("valid token stream");
        let error = parser.parse_next().expect_err("budget must be enforced");
        assert_eq!(error.kind, ContractParseErrorKind::TokenBudgetExceeded);
    }

    #[test]
    fn enforces_nesting_budget() {
        let tokens = tokenize_for_test("requires((x > 0));");
        let limits = ContractParserLimits {
            max_nesting_depth: 1,
            ..ContractParserLimits::default()
        };

        let mut parser = ContractParser::new(&tokens, limits).expect("valid token stream");
        let error = parser.parse_next().expect_err("nested delimiters exceed budget");
        assert_eq!(error.kind, ContractParseErrorKind::NestingBudgetExceeded);
    }

    #[test]
    fn enforces_statement_count_budget() {
        let tokens = tokenize_for_test("requires(x); ensures(y);");
        let limits = ContractParserLimits {
            max_clauses: 1,
            ..ContractParserLimits::default()
        };

        let mut parser = ContractParser::new(&tokens, limits).expect("valid token stream");
        parser.parse_next().expect("first statement");

        let error = parser.parse_next().expect_err("second statement exceeds budget");
        assert_eq!(error.kind, ContractParseErrorKind::ClauseCountBudgetExceeded);
    }

    #[test]
    fn failed_parse_rolls_back_to_statement_start() {
        let tokens = tokenize_for_test("requires();");
        let mut parser =
            ContractParser::new(&tokens, ContractParserLimits::default())
                .expect("valid token stream");

        let before = parser.position();
        assert!(parser.parse_next().is_err());
        assert_eq!(parser.position(), before);
    }

    #[test]
    fn rejects_invalid_eof_layout() {
        let source = "requires(x);";
        let mut tokens = tokenize_for_test(source);
        let eof = tokens.pop().expect("synthetic EOF");
        tokens.push(eof.clone());
        tokens.push(eof);

        let error = ContractParser::new(&tokens, ContractParserLimits::default())
            .expect_err("multiple EOF tokens are invalid");

        assert_eq!(error.kind, ContractParseErrorKind::InvalidTokenStream);
    }

    #[test]
    fn parse_all_consumes_multiple_statements() {
        let tokens = tokenize_for_test(
            "requires(x > 0); ensures(result >= 0); invariant(state.is_valid());",
        );
        let mut parser =
            ContractParser::new(&tokens, ContractParserLimits::default())
                .expect("valid token stream");

        let items = parser.parse_all().expect("all statements parse");
        assert_eq!(items.len(), 3);
        assert!(parser.is_eof());
    }
}
