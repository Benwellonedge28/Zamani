
#![forbid(unsafe_code)]

//! Integration-contract tests for the Zamani parser token cursor.
//!
//! File: src/frontend/parser/tests/cursor.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verify the public behavior of `TokenCursor` against the canonical lexer
//! token, channel, token-kind, and source-span contracts.
//!
//! # Ownership
//!
//! These tests do not implement a cursor, lexer, parser grammar, AST, or
//! diagnostic subsystem. They exercise the implementation in
//! `src/frontend/parser/token_cursor.rs`.
//!
//! # Contract coverage
//!
//! - Validated token-stream construction and EOF invariants.
//! - Parser-visible lookahead and trivia normalization.
//! - Stable EOF behavior and advancement accounting.
//! - Checkpoint restoration and foreign-stream rejection.
//! - Invalid raw indices and malformed token streams.
//! - Documentation and lexical-error channel preservation.
//! - UTF-8 byte spans.
//! - Atomic token-budget accounting.
//! - Repeated and large-stream traversal.
//!
//! The generated stream size below is a test workload, not a parser limit.
//! No test establishes a maximum supported program or hardware size.

use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;
use crate::frontend::parser::limits::{
    ParserBudget, ParserLimitKind, ParserLimits, ResourceLimit,
};
use crate::frontend::parser::token_cursor::{
    TokenCursor, TokenCursorCheckpointError,
};
use crate::frontend::parser::error::ParseErrorKind;

use core::num::NonZeroUsize;

/// Construct a single-line span using zero-based UTF-8 byte offsets and
/// one-based columns.
///
/// Tests using non-ASCII text should calculate columns from Unicode scalar
/// positions rather than assuming that byte offsets equal column numbers.
fn span(start_byte: usize, end_byte: usize) -> Span {
    Span::new(
        CursorPosition::new(start_byte, 1, start_byte + 1),
        CursorPosition::new(end_byte, 1, end_byte + 1),
    )
}

/// Construct a representative token stream containing trivia.
///
/// Parser navigation should expose `alpha`, `+`, and `beta`, while retaining
/// the trivia token in the original token slice.
fn tokens_with_trivia() -> Vec<Token<'static>> {
    vec![
        Token::borrowed(TokenKind::Identifier, "alpha", span(0, 5)),
        Token::borrowed(TokenKind::Whitespace, " ", span(5, 6))
            .with_channel(TokenChannel::Trivia),
        Token::borrowed(TokenKind::Plus, "+", span(6, 7)),
        Token::borrowed(TokenKind::Identifier, "beta", span(7, 11)),
        Token::synthetic(TokenKind::Eof, span(11, 11)),
    ]
}

/// Assert that malformed streams are rejected as invalid token streams.
fn assert_invalid_stream(tokens: &[Token<'_>]) {
    let error = TokenCursor::new(tokens)
        .expect_err("malformed token streams must be rejected");

    assert!(
        matches!(error.kind(), ParseErrorKind::InvalidTokenStream { .. }),
        "expected InvalidTokenStream, got {:?}",
        error.kind()
    );
}

#[test]
fn starts_at_the_first_parser_visible_token() {
    let tokens = tokens_with_trivia();
    let cursor = TokenCursor::new(&tokens).expect("valid token stream");

    assert_eq!(cursor.peek().kind(), TokenKind::Identifier);
    assert_eq!(cursor.peek().lexeme(), "alpha");
    assert_eq!(cursor.raw_index(), 0);
    assert_eq!(cursor.eof_index(), tokens.len() - 1);
    assert_eq!(cursor.consumed_tokens(), 0);
    assert!(!cursor.is_eof());
}

#[test]
fn retains_the_original_token_stream_without_copying_it() {
    let tokens = tokens_with_trivia();
    let cursor = TokenCursor::new(&tokens).expect("valid token stream");

    assert_eq!(cursor.tokens().len(), tokens.len());

    for (actual, original) in cursor.tokens().iter().zip(tokens.iter()) {
        assert_eq!(actual, original);
    }
}

#[test]
fn lookahead_skips_trivia_without_mutating_the_cursor() {
    let tokens = tokens_with_trivia();
    let cursor = TokenCursor::new(&tokens).expect("valid token stream");

    assert_eq!(cursor.peek_n(0).lexeme(), "alpha");
    assert_eq!(cursor.peek_n(1).kind(), TokenKind::Plus);
    assert_eq!(cursor.peek_n(2).lexeme(), "beta");
    assert_eq!(cursor.peek_n(3).kind(), TokenKind::Eof);

    assert_eq!(cursor.raw_index(), 0);
    assert_eq!(cursor.consumed_tokens(), 0);
}

#[test]
fn kind_predicates_agree_with_parser_visible_lookahead() {
    let tokens = tokens_with_trivia();
    let cursor = TokenCursor::new(&tokens).expect("valid token stream");

    assert!(cursor.at(TokenKind::Identifier));
    assert!(cursor.at_n(1, TokenKind::Plus));
    assert!(cursor.at_n(2, TokenKind::Identifier));
    assert!(cursor.at_n(3, TokenKind::Eof));
    assert!(!cursor.at(TokenKind::Plus));
    assert!(!cursor.at_n(1, TokenKind::Eof));
}

#[test]
fn advance_skips_trivia_and_counts_non_eof_advancements() {
    let tokens = tokens_with_trivia();
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");

    let next = cursor.advance();
    assert_eq!(next.kind(), TokenKind::Plus);
    assert_eq!(cursor.raw_index(), 2);
    assert_eq!(cursor.consumed_tokens(), 1);

    let next = cursor.advance();
    assert_eq!(next.lexeme(), "beta");
    assert_eq!(cursor.raw_index(), 3);
    assert_eq!(cursor.consumed_tokens(), 2);

    let next = cursor.advance();
    assert_eq!(next.kind(), TokenKind::Eof);
    assert!(cursor.is_eof());
    assert_eq!(cursor.consumed_tokens(), 3);
}

#[test]
fn eof_is_stable_and_does_not_count_as_consumed() {
    let tokens = tokens_with_trivia();
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");

    cursor.advance();
    cursor.advance();
    cursor.advance();

    assert!(cursor.is_eof());

    let eof_index = cursor.raw_index();
    let consumed = cursor.consumed_tokens();

    for _ in 0..4 {
        assert_eq!(cursor.peek().kind(), TokenKind::Eof);
        assert_eq!(cursor.advance().kind(), TokenKind::Eof);
        assert_eq!(cursor.raw_index(), eof_index);
        assert_eq!(cursor.consumed_tokens(), consumed);
    }
}

#[test]
fn lookahead_beyond_eof_saturates_at_eof() {
    let tokens = tokens_with_trivia();
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");

    assert_eq!(cursor.peek_n(usize::MAX).kind(), TokenKind::Eof);

    while !cursor.is_eof() {
        cursor.advance();
    }

    for distance in [0, 1, 2, usize::MAX] {
        assert_eq!(cursor.peek_n(distance).kind(), TokenKind::Eof);
    }
}

#[test]
fn checkpoint_restores_position_but_not_cumulative_work() {
    let tokens = tokens_with_trivia();
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");
    let checkpoint = cursor.checkpoint();

    cursor.advance();
    cursor.advance();

    assert_eq!(cursor.peek().lexeme(), "beta");
    assert_eq!(cursor.consumed_tokens(), 2);

    cursor.restore(checkpoint).expect("checkpoint belongs to stream");

    assert_eq!(cursor.peek().lexeme(), "alpha");
    assert_eq!(cursor.raw_index(), 0);
    assert_eq!(
        cursor.consumed_tokens(),
        2,
        "restoring position must not erase work already performed"
    );
}

#[test]
fn checkpoint_can_restore_the_eof_position() {
    let tokens = tokens_with_trivia();
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");

    while !cursor.is_eof() {
        cursor.advance();
    }

    let checkpoint = cursor.checkpoint();
    cursor.restore_index(0).expect("initial index is valid");
    assert!(!cursor.is_eof());

    cursor.restore(checkpoint).expect("EOF checkpoint is valid");
    assert!(cursor.is_eof());
}

#[test]
fn checkpoint_from_another_token_stream_is_rejected() {
    let first = tokens_with_trivia();
    let second = tokens_with_trivia();

    let checkpoint = TokenCursor::new(&first)
        .expect("first stream is valid")
        .checkpoint();

    let mut other_cursor =
        TokenCursor::new(&second).expect("second stream is valid");

    assert_eq!(
        other_cursor.restore(checkpoint),
        Err(TokenCursorCheckpointError::ForeignStream)
    );
}

#[test]
fn restoring_a_raw_index_normalizes_trivia() {
    let tokens = tokens_with_trivia();
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");

    cursor
        .restore_index(1)
        .expect("trivia index is within the stream");

    assert_eq!(cursor.peek().kind(), TokenKind::Plus);
    assert_eq!(cursor.raw_index(), 2);
}

#[test]
fn restoring_the_eof_index_is_valid() {
    let tokens = tokens_with_trivia();
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");

    cursor
        .restore_index(tokens.len() - 1)
        .expect("EOF index is valid");

    assert!(cursor.is_eof());
}

#[test]
fn restoring_an_index_past_eof_is_rejected_without_moving() {
    let tokens = tokens_with_trivia();
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");

    cursor.advance();
    let original_index = cursor.raw_index();
    let original_kind = cursor.peek().kind();

    assert!(matches!(
        cursor.restore_index(tokens.len()),
        Err(TokenCursorCheckpointError::InvalidIndex { .. })
    ));

    assert_eq!(cursor.raw_index(), original_index);
    assert_eq!(cursor.peek().kind(), original_kind);
}

#[test]
fn rejects_an_empty_token_stream() {
    let tokens: Vec<Token<'static>> = Vec::new();
    assert_invalid_stream(&tokens);
}

#[test]
fn rejects_a_stream_without_eof() {
    let tokens = vec![Token::borrowed(
        TokenKind::Identifier,
        "alpha",
        span(0, 5),
    )];

    assert_invalid_stream(&tokens);
}

#[test]
fn rejects_eof_before_the_final_token() {
    let tokens = vec![
        Token::synthetic(TokenKind::Eof, span(0, 0)),
        Token::borrowed(TokenKind::Identifier, "alpha", span(0, 5)),
    ];

    assert_invalid_stream(&tokens);
}

#[test]
fn rejects_more_than_one_eof_token() {
    let tokens = vec![
        Token::synthetic(TokenKind::Eof, span(0, 0)),
        Token::synthetic(TokenKind::Eof, span(0, 0)),
    ];

    assert_invalid_stream(&tokens);
}

#[test]
fn rejects_non_zero_width_eof() {
    let tokens = vec![Token::synthetic(TokenKind::Eof, span(0, 1))];

    assert_invalid_stream(&tokens);
}

#[test]
fn rejects_overlapping_token_spans() {
    let tokens = vec![
        Token::borrowed(TokenKind::Identifier, "alpha", span(0, 5)),
        Token::borrowed(TokenKind::Plus, "+", span(4, 5)),
        Token::synthetic(TokenKind::Eof, span(5, 5)),
    ];

    assert_invalid_stream(&tokens);
}

#[test]
fn rejects_out_of_order_token_spans() {
    let tokens = vec![
        Token::borrowed(TokenKind::Identifier, "alpha", span(5, 10)),
        Token::borrowed(TokenKind::Plus, "+", span(2, 3)),
        Token::synthetic(TokenKind::Eof, span(10, 10)),
    ];

    assert_invalid_stream(&tokens);
}

#[test]
fn allows_adjacent_non_overlapping_spans() {
    let tokens = vec![
        Token::borrowed(TokenKind::Identifier, "a", span(0, 1)),
        Token::borrowed(TokenKind::Plus, "+", span(1, 2)),
        Token::borrowed(TokenKind::Identifier, "b", span(2, 3)),
        Token::synthetic(TokenKind::Eof, span(3, 3)),
    ];

    let mut cursor = TokenCursor::new(&tokens).expect("adjacent spans are valid");

    assert_eq!(cursor.peek().lexeme(), "a");
    cursor.advance();
    assert_eq!(cursor.peek().lexeme(), "+");
    cursor.advance();
    assert_eq!(cursor.peek().lexeme(), "b");
    cursor.advance();
    assert!(cursor.is_eof());
}

#[test]
fn documentation_tokens_remain_visible() {
    let tokens = vec![
        Token::borrowed(TokenKind::Identifier, "doc", span(0, 3))
            .with_channel(TokenChannel::Documentation),
        Token::synthetic(TokenKind::Eof, span(3, 3)),
    ];

    let cursor = TokenCursor::new(&tokens).expect("valid token stream");

    assert_eq!(cursor.peek().channel(), TokenChannel::Documentation);
    assert_eq!(cursor.peek().lexeme(), "doc");
    assert!(!cursor.is_lexical_error());
}

#[test]
fn lexical_error_tokens_remain_visible() {
    let tokens = vec![
        Token::borrowed(TokenKind::Invalid, "?", span(0, 1))
            .with_channel(TokenChannel::Error),
        Token::synthetic(TokenKind::Eof, span(1, 1)),
    ];

    let cursor = TokenCursor::new(&tokens).expect("valid token stream");

    assert_eq!(cursor.peek().kind(), TokenKind::Invalid);
    assert!(cursor.is_lexical_error());
}

#[test]
fn utf8_spans_use_byte_offsets_without_splitting_characters() {
    // "λ" is one Unicode scalar value encoded as two UTF-8 bytes.
    let tokens = vec![
        Token::borrowed(
            TokenKind::Identifier,
            "λ",
            Span::new(
                CursorPosition::new(0, 1, 1),
                CursorPosition::new(2, 1, 2),
            ),
        ),
        Token::borrowed(
            TokenKind::Plus,
            "+",
            Span::new(
                CursorPosition::new(2, 1, 2),
                CursorPosition::new(3, 1, 3),
            ),
        ),
        Token::synthetic(
            TokenKind::Eof,
            Span::at(CursorPosition::new(3, 1, 3)),
        ),
    ];

    let mut cursor = TokenCursor::new(&tokens).expect("valid UTF-8 spans");

    assert_eq!(cursor.peek().lexeme(), "λ");
    assert_eq!(cursor.span().byte_range(), 0..2);

    cursor.advance();
    assert_eq!(cursor.peek().kind(), TokenKind::Plus);
    assert_eq!(cursor.span().byte_range(), 2..3);

    cursor.advance();
    assert!(cursor.is_eof());
    assert_eq!(cursor.span().byte_range(), 3..3);
}

#[test]
fn metered_advancement_charges_the_token_budget() {
    let tokens = tokens_with_trivia();
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");
    let mut budget = ParserBudget::new(ParserLimits::unlimited());

    cursor
        .advance_with_budget(&mut budget)
        .expect("first token fits budget");

    assert_eq!(budget.usage().get(ParserLimitKind::Tokens), 1);
    assert_eq!(cursor.peek().kind(), TokenKind::Plus);

    cursor
        .advance_with_budget(&mut budget)
        .expect("second visible token fits budget");

    assert_eq!(budget.usage().get(ParserLimitKind::Tokens), 2);
    assert_eq!(cursor.peek().lexeme(), "beta");
}

#[test]
fn failed_budget_charge_does_not_advance_or_consume_budget() {
    let tokens = tokens_with_trivia();
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");

    let limits = ParserLimits {
        tokens: ResourceLimit::AtMost(
            NonZeroUsize::new(1).expect("one is non-zero"),
        ),
        ..ParserLimits::unlimited()
    };
    let mut budget = ParserBudget::new(limits);

    cursor
        .advance_with_budget(&mut budget)
        .expect("the first visible token fits");

    let index_before = cursor.raw_index();
    let consumed_before = cursor.consumed_tokens();
    let usage_before = budget.usage();

    let result = cursor.advance_with_budget(&mut budget);

    assert!(result.is_err());
    assert_eq!(cursor.raw_index(), index_before);
    assert_eq!(cursor.consumed_tokens(), consumed_before);
    assert_eq!(budget.usage(), usage_before);
}

#[test]
fn eof_does_not_consume_a_token_budget_unit() {
    let tokens = vec![Token::synthetic(TokenKind::Eof, span(0, 0))];
    let mut cursor = TokenCursor::new(&tokens).expect("valid empty program stream");
    let mut budget = ParserBudget::new(ParserLimits::unlimited());

    for _ in 0..4 {
        cursor
            .advance_with_budget(&mut budget)
            .expect("EOF observation does not require budget");
    }

    assert!(cursor.is_eof());
    assert_eq!(cursor.consumed_tokens(), 0);
    assert_eq!(budget.usage().get(ParserLimitKind::Tokens), 0);
}

#[test]
fn repeated_traversal_is_deterministic() {
    let first_tokens = tokens_with_trivia();
    let second_tokens = tokens_with_trivia();

    let mut first = TokenCursor::new(&first_tokens).expect("valid first stream");
    let mut second =
        TokenCursor::new(&second_tokens).expect("valid second stream");

    loop {
        let left = first.peek();
        let right = second.peek();

        assert_eq!(left.kind(), right.kind());
        assert_eq!(left.lexeme(), right.lexeme());
        assert_eq!(left.span(), right.span());
        assert_eq!(left.channel(), right.channel());

        if first.is_eof() {
            break;
        }

        first.advance();
        second.advance();
    }

    assert_eq!(first.raw_index(), second.raw_index());
    assert_eq!(first.consumed_tokens(), second.consumed_tokens());
}

#[test]
fn traverses_a_dynamically_constructed_stream() {
    // This is a repeatable test workload, not a maximum supported token count.
    const TEST_TOKEN_COUNT: usize = 4_096;

    let mut tokens = Vec::with_capacity(TEST_TOKEN_COUNT + 1);
    let mut offset = 0usize;

    for index in 0..TEST_TOKEN_COUNT {
        let lexeme = format!("v{index}");
        let end = offset + lexeme.len();

        tokens.push(Token::owned(
            TokenKind::Identifier,
            lexeme,
            span(offset, end),
        ));

        offset = end;
    }

    tokens.push(Token::synthetic(TokenKind::Eof, span(offset, offset)));

    let mut cursor = TokenCursor::new(&tokens).expect("generated stream is valid");

    for index in 0..TEST_TOKEN_COUNT {
        assert_eq!(cursor.peek().kind(), TokenKind::Identifier);
        assert_eq!(cursor.peek().lexeme(), format!("v{index}"));
        cursor.advance();
    }

    assert!(cursor.is_eof());
    assert_eq!(cursor.consumed_tokens(), TEST_TOKEN_COUNT);
    assert_eq!(cursor.peek().kind(), TokenKind::Eof);
}
