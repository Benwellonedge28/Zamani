
#![forbid(unsafe_code)]

//! Validated, deterministic cursor over the canonical Zamani lexer token stream.
//!
//! # Purpose
//!
//! Provide the parser with safe token lookahead, advancement, EOF handling,
//! source-span access, trivia skipping, and cursor checkpoints.
//!
//! # Authority and ownership
//!
//! This module consumes, but does not own, the canonical lexer types:
//!
//! - `lexer::token::Token`
//! - `lexer::token::TokenChannel`
//! - `lexer::token_kind::TokenKind`
//! - `lexer::span::Span`
//!
//! It also integrates with the parser's canonical resource accounting:
//!
//! - `parser::limits::ParserBudget`
//! - `parser::limits::ParserLimitKind`
//! - `parser::error::ParseError`
//!
//! It does not tokenize source, construct AST nodes, decide grammar rules,
//! perform semantic analysis, or inspect execution hardware.
//!
//! # Token-stream contract
//!
//! The input must contain exactly one EOF token, and that token must be the
//! final token. Its span must be zero-width. Token spans must be ordered and
//! non-overlapping by UTF-8 byte offset.
//!
//! Tokens on the `Trivia` channel are skipped for parser-facing lookahead.
//! Documentation tokens remain visible so grammar rules can attach them to
//! declarations. Error-channel tokens remain visible so the parser can
//! report or preserve lexical failures instead of silently discarding them.
//!
//! # Resource accounting
//!
//! `advance_with_budget` charges one unit to the parser's cumulative token
//! budget before consuming a non-EOF token. If the charge fails, the cursor
//! does not advance.
//!
//! EOF observations do not consume a token-budget unit. Speculative parsing
//! still consumes parser work: rolling the cursor back does not refund the
//! cumulative budget.
//!
//! `advance` is an unmetered primitive for low-level integrations and tests.
//! Production parser sessions should use `advance_with_budget` so token
//! consumption is included in the session's `ParserBudget`.
//!
//! # Scalability
//!
//! The cursor stores a borrowed token slice and constant-size position state.
//! It does not copy the token stream or impose hardware-capacity limits.
//! Work and memory remain bounded by the actual input, configured budgets,
//! address space, and available resources.
//!
//! # Integration
//!
//! - `session.rs` owns the parse-local `ParserBudget` and supplies it to
//!   `advance_with_budget`.
//! - `checkpoint.rs` can snapshot `raw_index()` in `CheckpointState` and
//!   restore it with `restore_index()`.
//! - Grammar productions use `peek`, `peek_n`, `advance_with_budget`,
//!   `expect_kind`-style checks in their owning grammar layer, and `span`.
//! - `recovery.rs` uses `restore_index` or a cursor checkpoint when
//!   implementing a documented recovery strategy.
//! - `diagnostics.rs` and `error.rs` remain the owners of parser errors.
//!
//! This module deliberately does not depend on an unfinished `session.rs`,
//! `recovery.rs`, or grammar module.

use core::fmt;

use super::error::{ParseError, ParseErrorClass, ParseErrorKind};
use super::limits::{ParserBudget, ParserLimitKind};
use super::super::lexer::span::Span;
use super::super::lexer::token::{Token, TokenChannel};
use super::super::lexer::token_kind::TokenKind;

/// A validated position in a particular token slice.
///
/// Checkpoints are produced by `TokenCursor::checkpoint`. The private slice
/// reference binds the checkpoint to the stream from which it was created;
/// a checkpoint from another cursor cannot be restored accidentally.
#[derive(Debug, Clone, Copy)]
pub struct TokenCursorCheckpoint<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    index: usize,
}

impl<'tokens, 'src> TokenCursorCheckpoint<'tokens, 'src> {
    /// Raw token index captured by this checkpoint.
    ///
    /// The index addresses the original token slice, including trivia.
    #[must_use]
    pub const fn raw_index(self) -> usize {
        self.index
    }
}

/// Errors returned when restoring a cursor position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TokenCursorCheckpointError {
    /// The checkpoint was created from another token slice.
    ForeignStream,

    /// The requested index is outside the valid cursor range.
    InvalidIndex {
        index: usize,
        eof_index: usize,
    },
}

impl fmt::Display for TokenCursorCheckpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForeignStream => {
                f.write_str("token cursor checkpoint belongs to another stream")
            }
            Self::InvalidIndex { index, eof_index } => write!(
                f,
                "token index {index} is outside the valid range 0..={eof_index}"
            ),
        }
    }
}

impl std::error::Error for TokenCursorCheckpointError {}

/// A validated cursor over an immutable canonical token stream.
///
/// The cursor borrows tokens rather than cloning them. Its current index is
/// always normalized to a parser-visible token or the EOF token.
#[derive(Debug)]
pub struct TokenCursor<'tokens, 'src> {
    tokens: &'tokens [Token<'src>],
    index: usize,
    eof_index: usize,
    consumed_tokens: usize,
}

/// Construct or validate a cursor from an existing token slice.
///
/// The cursor never mutates the supplied tokens.
impl<'tokens, 'src> TokenCursor<'tokens, 'src> {
    /// Validate a complete token stream and create a cursor at its beginning.
    ///
    /// Returns a fatal `InvalidTokenStream` error if EOF is missing, appears
    /// more than once, is not the final token, has a non-zero-width span, or
    /// if token spans are out of order or overlap.
    pub fn new(tokens: &'tokens [Token<'src>]) -> Result<Self, ParseError> {
        if tokens.is_empty() {
            return Err(Self::invalid_stream(
                "token stream is empty; an explicit EOF token is required",
                None,
            ));
        }

        let mut eof_index = None;
        let mut previous_end = 0usize;
        let mut have_previous = false;

        for (index, token) in tokens.iter().enumerate() {
            let span = token.span();
            let start = span.start_byte();
            let end = span.end_byte();

            if end < start {
                return Err(Self::invalid_stream(
                    "token span ends before it starts",
                    Some(*span),
                ));
            }

            if have_previous && start < previous_end {
                return Err(Self::invalid_stream(
                    "token spans are not ordered or overlap",
                    Some(*span),
                ));
            }

            previous_end = end;
            have_previous = true;

            if token.kind() == TokenKind::Eof {
                if eof_index.is_some() {
                    return Err(Self::invalid_stream(
                        "token stream contains more than one EOF token",
                        Some(*span),
                    ));
                }

                if start != end {
                    return Err(Self::invalid_stream(
                        "EOF token must have a zero-width source span",
                        Some(*span),
                    ));
                }

                eof_index = Some(index);
            }
        }

        let eof_index = match eof_index {
            Some(index) if index == tokens.len() - 1 => index,
            Some(index) => {
                return Err(Self::invalid_stream(
                    "EOF token must be the final token",
                    tokens.get(index).map(|token| *token.span()),
                ));
            }
            None => {
                return Err(Self::invalid_stream(
                    "token stream does not contain an explicit EOF token",
                    tokens.last().map(|token| *token.span()),
                ));
            }
        };

        let mut cursor = Self {
            tokens,
            index: 0,
            eof_index,
            consumed_tokens: 0,
        };

        cursor.normalize_index();
        Ok(cursor)
    }

    /// Return the original token slice.
    #[must_use]
    pub fn tokens(&self) -> &'tokens [Token<'src>] {
        self.tokens
    }

    /// Return the current raw index into the original token slice.
    ///
    /// Trivia tokens are included in this index space, even though normal
    /// cursor navigation skips them.
    #[must_use]
    pub const fn raw_index(&self) -> usize {
        self.index
    }

    /// Return the index of the required final EOF token.
    #[must_use]
    pub const fn eof_index(&self) -> usize {
        self.eof_index
    }

    /// Return the number of non-EOF token advancements performed since
    /// cursor creation.
    ///
    /// This counts actual advancements, including speculative advancements
    /// later undone by restoring a checkpoint. It is not the parser's
    /// cumulative token budget; `ParserBudget` owns that accounting.
    #[must_use]
    pub const fn consumed_tokens(&self) -> usize {
        self.consumed_tokens
    }

    /// Return the current parser-visible token.
    ///
    /// This always returns a token because construction validates the stream
    /// and the cursor remains at or before EOF.
    #[must_use]
    pub fn peek(&self) -> &'tokens Token<'src> {
        // The constructor and all cursor movement maintain this invariant.
        // Keep a defensive fallback rather than panicking if a future edit
        // accidentally violates it.
        match self.tokens.get(self.index) {
            Some(token) => token,
            None => &self.tokens[self.eof_index],
        }
    }

    /// Return the current token's canonical source span.
    #[must_use]
    pub fn span(&self) -> Span {
        *self.peek().span()
    }

    /// Return true when the current token is EOF.
    #[must_use]
    pub fn is_eof(&self) -> bool {
        self.peek().kind() == TokenKind::Eof
    }

    /// Return the token `distance` parser-visible tokens ahead.
    ///
    /// `peek_n(0)` is equivalent to `peek()`. Trivia-channel tokens do not
    /// count toward the distance. Lookahead past EOF remains at EOF, which
    /// makes arbitrary lookahead safe without requiring sentinel tokens
    /// beyond the validated stream.
    #[must_use]
    pub fn peek_n(&self, distance: usize) -> &'tokens Token<'src> {
        let mut index = self.index;

        for _ in 0..distance {
            if index >= self.eof_index {
                return &self.tokens[self.eof_index];
            }

            index += 1;
            while index < self.eof_index
                && self.tokens[index].channel() == TokenChannel::Trivia
            {
                index += 1;
            }
        }

        if index > self.eof_index {
            index = self.eof_index;
        }

        &self.tokens[index]
    }

    /// Advance to the next parser-visible token without resource accounting.
    ///
    /// If the current token is EOF, the cursor stays at EOF. This primitive
    /// exists for low-level integrations and tests; production parser code
    /// should prefer `advance_with_budget`.
    pub fn advance(&mut self) -> &'tokens Token<'src> {
        if self.index < self.eof_index {
            self.index += 1;
            self.normalize_index();
            self.consumed_tokens += 1;
        }

        self.peek()
    }

    /// Charge the parser token budget and advance atomically.
    ///
    /// If budget accounting fails, the cursor position and advancement count
    /// remain unchanged. EOF is returned without charging or advancing.
    pub fn advance_with_budget(
        &mut self,
        budget: &mut ParserBudget,
    ) -> Result<&'tokens Token<'src>, ParseError> {
        if self.is_eof() {
            return Ok(self.peek());
        }

        let span = self.span();

        budget
            .charge(ParserLimitKind::Tokens, 1)
            .map_err(|violation| ParseError::resource_limit(violation, Some(span)))?;

        Ok(self.advance())
    }

    /// Create a checkpoint for this cursor's current position.
    ///
    /// The checkpoint does not snapshot parser diagnostics, nesting depth,
    /// recovery state, or resource-budget usage. Those remain owned by the
    /// parser session and its checkpoint manager.
    #[must_use]
    pub fn checkpoint(&self) -> TokenCursorCheckpoint<'tokens, 'src> {
        TokenCursorCheckpoint {
            tokens: self.tokens,
            index: self.index,
        }
    }

    /// Restore a checkpoint created by this exact token slice.
    ///
    /// Restoring position does not refund token-budget usage and does not
    /// decrement `consumed_tokens`, because both represent cumulative work
    /// already performed by the parser.
    pub fn restore(
        &mut self,
        checkpoint: TokenCursorCheckpoint<'tokens, 'src>,
    ) -> Result<(), TokenCursorCheckpointError> {
        if !core::ptr::eq(self.tokens, checkpoint.tokens) {
            return Err(TokenCursorCheckpointError::ForeignStream);
        }

        self.restore_index(checkpoint.index)
    }

    /// Restore a raw token index supplied by the parser session.
    ///
    /// The index must not be past EOF. If it points at trivia, the cursor
    /// advances to the next parser-visible token. This method is intended
    /// for integration with `CheckpointState::token_index`.
    ///
    /// Cumulative budget usage and `consumed_tokens` are intentionally not
    /// rolled back.
    pub fn restore_index(
        &mut self,
        index: usize,
    ) -> Result<(), TokenCursorCheckpointError> {
        if index > self.eof_index {
            return Err(TokenCursorCheckpointError::InvalidIndex {
                index,
                eof_index: self.eof_index,
            });
        }

        self.index = index;
        self.normalize_index();
        Ok(())
    }

    /// Return whether the current token has the supplied kind.
    #[must_use]
    pub fn at(&self, kind: TokenKind) -> bool {
        self.peek().kind() == kind
    }

    /// Return whether the token at the given lookahead distance has the
    /// supplied kind.
    #[must_use]
    pub fn at_n(&self, distance: usize, kind: TokenKind) -> bool {
        self.peek_n(distance).kind() == kind
    }

    /// Return whether the current token belongs to the lexical error channel.
    ///
    /// The cursor deliberately does not convert lexical errors into parser
    /// errors automatically. The parser/session boundary should preserve the
    /// original lexer diagnostic whenever it is available.
    #[must_use]
    pub fn is_lexical_error(&self) -> bool {
        self.peek().channel() == TokenChannel::Error
    }

    fn normalize_index(&mut self) {
        while self.index < self.eof_index
            && self.tokens[self.index].channel() == TokenChannel::Trivia
        {
            self.index += 1;
        }

        if self.index > self.eof_index {
            self.index = self.eof_index;
        }
    }

    fn invalid_stream(message: &str, span: Option<Span>) -> ParseError {
        ParseError::new(
            ParseErrorKind::InvalidTokenStream {
                message: message.to_owned(),
            },
            ParseErrorClass::Fatal,
        )
        .with_span_if_some(span)
    }
}

// Keep the optional-span construction explicit without requiring callers to
// manufacture a default span or invent a source coordinate.
trait ParseErrorOptionalSpan {
    fn with_span_if_some(self, span: Option<Span>) -> Self;
}

impl ParseErrorOptionalSpan for ParseError {
    fn with_span_if_some(self, span: Option<Span>) -> Self {
        match span {
            Some(span) => self.with_span(span),
            None => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::cursor::CursorPosition;
    use crate::frontend::lexer::span::Span;
    use crate::frontend::lexer::token::Token;
    use crate::frontend::lexer::token_kind::TokenKind;
    use crate::frontend::parser::limits::{ParserLimits, ResourceLimit};
    use core::num::NonZeroUsize;

    fn span(start: usize, end: usize) -> Span {
        Span::new(
            CursorPosition::new(start, 1, start + 1),
            CursorPosition::new(end, 1, end + 1),
        )
    }

    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token::borrowed(TokenKind::Identifier, "alpha", span(0, 5)),
            Token::borrowed(TokenKind::Whitespace, " ", span(5, 6))
                .with_channel(TokenChannel::Trivia),
            Token::borrowed(TokenKind::Plus, "+", span(6, 7)),
            Token::synthetic(TokenKind::Eof, span(7, 7)),
        ]
    }

    #[test]
    fn starts_at_first_parser_visible_token() {
        let input = tokens();
        let cursor = TokenCursor::new(&input).expect("valid stream");

        assert_eq!(cursor.peek().kind(), TokenKind::Identifier);
        assert_eq!(cursor.raw_index(), 0);
        assert_eq!(cursor.consumed_tokens(), 0);
    }

    #[test]
    fn skips_trivia_during_lookahead_and_advance() {
        let input = tokens();
        let mut cursor = TokenCursor::new(&input).expect("valid stream");

        assert_eq!(cursor.peek_n(1).kind(), TokenKind::Plus);
        assert_eq!(cursor.advance().kind(), TokenKind::Plus);
        assert_eq!(cursor.raw_index(), 2);
        assert_eq!(cursor.consumed_tokens(), 1);
    }

    #[test]
    fn lookahead_past_eof_stays_at_eof() {
        let input = tokens();
        let mut cursor = TokenCursor::new(&input).expect("valid stream");

        cursor.advance();
        cursor.advance();

        assert!(cursor.is_eof());
        assert_eq!(cursor.peek_n(usize::MAX).kind(), TokenKind::Eof);
    }

    #[test]
    fn eof_is_stable_and_not_consumed() {
        let input = tokens();
        let mut cursor = TokenCursor::new(&input).expect("valid stream");

        cursor.advance();
        cursor.advance();

        let index = cursor.raw_index();
        assert_eq!(cursor.advance().kind(), TokenKind::Eof);
        assert_eq!(cursor.advance().kind(), TokenKind::Eof);
        assert_eq!(cursor.raw_index(), index);
        assert_eq!(cursor.consumed_tokens(), 2);
    }

    #[test]
    fn checkpoint_restores_position_without_refunding_work() {
        let input = tokens();
        let mut cursor = TokenCursor::new(&input).expect("valid stream");
        let checkpoint = cursor.checkpoint();

        cursor.advance();
        cursor.advance();

        assert_eq!(cursor.peek().kind(), TokenKind::Eof);
        cursor.restore(checkpoint).expect("same stream");

        assert_eq!(cursor.peek().kind(), TokenKind::Identifier);
        assert_eq!(cursor.consumed_tokens(), 2);
    }

    #[test]
    fn checkpoint_from_another_stream_is_rejected() {
        let first = tokens();
        let second = tokens();
        let checkpoint = TokenCursor::new(&first)
            .expect("valid stream")
            .checkpoint();
        let mut cursor = TokenCursor::new(&second).expect("valid stream");

        assert_eq!(
            cursor.restore(checkpoint),
            Err(TokenCursorCheckpointError::ForeignStream)
        );
    }

    #[test]
    fn restore_index_rejects_positions_past_eof() {
        let input = tokens();
        let mut cursor = TokenCursor::new(&input).expect("valid stream");

        assert!(matches!(
            cursor.restore_index(input.len()),
            Err(TokenCursorCheckpointError::InvalidIndex { .. })
        ));
    }

    #[test]
    fn restore_index_normalizes_trivia() {
        let input = tokens();
        let mut cursor = TokenCursor::new(&input).expect("valid stream");

        cursor.restore_index(1).expect("index is within stream");
        assert_eq!(cursor.peek().kind(), TokenKind::Plus);
        assert_eq!(cursor.raw_index(), 2);
    }

    #[test]
    fn rejects_missing_eof() {
        let input = vec![Token::borrowed(
            TokenKind::Identifier,
            "alpha",
            span(0, 5),
        )];

        assert!(matches!(
            TokenCursor::new(&input),
            Err(ParseError {
                kind: ParseErrorKind::InvalidTokenStream { .. },
                ..
            })
        ));
    }

    #[test]
    fn rejects_eof_that_is_not_last() {
        let input = vec![
            Token::synthetic(TokenKind::Eof, span(0, 0)),
            Token::borrowed(TokenKind::Identifier, "alpha", span(0, 5)),
        ];

        assert!(TokenCursor::new(&input).is_err());
    }

    #[test]
    fn rejects_non_zero_width_eof() {
        let input = vec![Token::synthetic(TokenKind::Eof, span(0, 1))];

        assert!(TokenCursor::new(&input).is_err());
    }

    #[test]
    fn rejects_overlapping_or_out_of_order_spans() {
        let input = vec![
            Token::borrowed(TokenKind::Identifier, "alpha", span(0, 5)),
            Token::borrowed(TokenKind::Plus, "+", span(4, 5)),
            Token::synthetic(TokenKind::Eof, span(5, 5)),
        ];

        assert!(TokenCursor::new(&input).is_err());
    }

    #[test]
    fn metered_advance_charges_tokens() {
        let input = tokens();
        let mut cursor = TokenCursor::new(&input).expect("valid stream");
        let mut budget = ParserBudget::new(ParserLimits::unlimited());

        cursor
            .advance_with_budget(&mut budget)
            .expect("budget permits advancement");

        assert_eq!(budget.usage().get(ParserLimitKind::Tokens), 1);
        assert_eq!(cursor.peek().kind(), TokenKind::Plus);
    }

    #[test]
    fn failed_budget_charge_does_not_advance() {
        let input = tokens();
        let mut cursor = TokenCursor::new(&input).expect("valid stream");

        let limits = ParserLimits {
            tokens: ResourceLimit::AtMost(
                NonZeroUsize::new(1).expect("one is non-zero"),
            ),
            ..ParserLimits::unlimited()
        };
        let mut budget = ParserBudget::new(limits);

        cursor
            .advance_with_budget(&mut budget)
            .expect("first token fits");

        let before = cursor.raw_index();
        let result = cursor.advance_with_budget(&mut budget);

        assert!(result.is_err());
        assert_eq!(cursor.raw_index(), before);
        assert_eq!(budget.usage().get(ParserLimitKind::Tokens), 1);
    }

    #[test]
    fn eof_does_not_consume_budget() {
        let input = tokens();
        let mut cursor = TokenCursor::new(&input).expect("valid stream");
        let mut budget = ParserBudget::new(ParserLimits::unlimited());

        cursor.advance_with_budget(&mut budget).expect("first token");
        cursor.advance_with_budget(&mut budget).expect("second token");
        assert!(cursor.is_eof());

        cursor.advance_with_budget(&mut budget).expect("EOF is stable");
        assert_eq!(budget.usage().get(ParserLimitKind::Tokens), 2);
    }

    #[test]
    fn documentation_channel_is_not_silently_skipped() {
        let input = vec![
            Token::borrowed(TokenKind::Identifier, "doc", span(0, 3))
                .with_channel(TokenChannel::Documentation),
            Token::synthetic(TokenKind::Eof, span(3, 3)),
        ];
        let cursor = TokenCursor::new(&input).expect("valid stream");

        assert_eq!(cursor.peek().channel(), TokenChannel::Documentation);
    }

    #[test]
    fn lexical_error_channel_remains_visible() {
        let input = vec![
            Token::borrowed(TokenKind::Invalid, "?", span(0, 1))
                .with_channel(TokenChannel::Error),
            Token::synthetic(TokenKind::Eof, span(1, 1)),
        ];
        let cursor = TokenCursor::new(&input).expect("valid stream");

        assert!(cursor.is_lexical_error());
    }
}
