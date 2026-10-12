
#![forbid(unsafe_code)]

//! Production stress and robustness tests for the Zamani frontend parser.
//!
//! File: src/frontend/parser/tests/stress.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Exercise parser token-stream handling under large, deeply indexed,
//! malformed, Unicode-containing, and resource-constrained workloads.
//!
//! These tests focus on the canonical frontend TokenCursor and its integration
//! with the lexer token model and parser resource accounting. They do not
//! pretend that testing a cursor alone proves that every grammar production,
//! AST conversion, or compiler integration is production-ready.
//!
//! # Architecture
//!
//! - The lexer owns tokenization and canonical token representations.
//! - The parser cursor owns validated token traversal.
//! - ParserLimits and ParserBudget own configurable parser-work accounting.
//! - Grammar productions own syntax recognition.
//! - AST construction and semantic analysis remain downstream concerns.
//!
//! # Scalability
//!
//! Workload sizes in this file are test inputs, not language limits.
//! No test establishes a maximum number of qubits, processors, devices,
//! registers, tensor dimensions, declarations, or source constructs.
//!
//! No finite test can prove support for literally infinite input. Real
//! capacity is bounded by available memory, address space, time, and any
//! explicitly configured implementation budgets.
//!
//! # Integration
//!
//! This module expects the following canonical types:
//!
//! - crate::frontend::lexer::cursor::CursorPosition
//! - crate::frontend::lexer::span::Span
//! - crate::frontend::lexer::token::{Token, TokenChannel}
//! - crate::frontend::lexer::token_kind::TokenKind
//! - crate::frontend::parser::limits::{ParserBudget, ParserLimitKind,
//!   ParserLimits, ResourceLimit}
//! - crate::frontend::parser::token_cursor::TokenCursor
//!
//! Wire this file into the parser test module as described after the code.
//!
//! # Safety
//!
//! This file uses safe Rust only. It performs no unsafe memory access,
//! filesystem access, network access, hardware discovery, or execution of
//! parsed source programs.

use std::num::NonZeroUsize;

use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;
use crate::frontend::parser::limits::{
    ParserBudget,
    ParserLimitKind,
    ParserLimits,
    ResourceLimit,
};
use crate::frontend::parser::token_cursor::TokenCursor;

/// Build a span from UTF-8 byte offsets.
///
/// These synthetic stress tokens occupy a single logical line. The helper
/// uses byte offsets for source boundaries; the lexer remains responsible
/// for calculating real source positions when scanning source text.
fn span(start: usize, end: usize) -> Span {
    Span::new(
        CursorPosition::new(start, 1, start.saturating_add(1)),
        CursorPosition::new(end, 1, end.saturating_add(1)),
    )
}

/// Build a valid, ordered stream of synthetic tokens.
///
/// The generated stream alternates identifiers and plus operators, followed
/// by exactly one zero-width EOF token. Token lexemes are borrowed static
/// strings so the test does not allocate a separate string for every token.
///
/// `operand_count` describes the number of identifier tokens. The total
/// token count is therefore proportional to the requested workload.
fn arithmetic_tokens(operand_count: usize) -> Vec<Token<'static>> {
    let operator_count = operand_count.saturating_sub(1);
    let token_count = operand_count
        .checked_add(operator_count)
        .expect("stress-test token count must fit usize");
    let capacity = token_count
        .checked_add(1)
        .expect("stress-test EOF slot must fit usize");

    let mut tokens = Vec::new();
    tokens
        .try_reserve_exact(capacity)
        .expect("the test environment must be able to reserve its requested workload");

    let mut offset = 0usize;

    for index in 0..operand_count {
        tokens.push(Token::borrowed(
            TokenKind::Identifier,
            "x",
            span(offset, offset + 1),
        ));
        offset += 1;

        if index < operator_count {
            tokens.push(Token::borrowed(
                TokenKind::Plus,
                "+",
                span(offset, offset + 1),
            ));
            offset += 1;
        }
    }

    tokens.push(Token::synthetic(
        TokenKind::Eof,
        span(offset, offset),
    ));

    tokens
}

/// Build a stream containing a single identifier and EOF.
fn single_identifier_tokens() -> Vec<Token<'static>> {
    vec![
        Token::borrowed(TokenKind::Identifier, "x", span(0, 1)),
        Token::synthetic(TokenKind::Eof, span(1, 1)),
    ]
}

#[test]
fn empty_source_token_stream_with_only_eof_is_valid() {
    let tokens = vec![Token::synthetic(TokenKind::Eof, span(0, 0))];

    let cursor = TokenCursor::new(&tokens).expect("EOF-only input is valid");

    assert!(cursor.is_eof());
    assert_eq!(cursor.peek().kind(), TokenKind::Eof);
    assert_eq!(cursor.consumed_tokens(), 0);
    assert_eq!(cursor.span(), span(0, 0));
}

#[test]
fn large_token_stream_preserves_order_and_progress() {
    // This is a repeatable regression workload, not a parser capacity limit.
    let operand_count = 25_000;
    let expected_token_count = operand_count
        .checked_mul(2)
        .expect("test workload must fit usize")
        - 1;

    let tokens = arithmetic_tokens(operand_count);

    assert_eq!(tokens.len(), expected_token_count + 1);

    let mut cursor = TokenCursor::new(&tokens).expect("valid large stream");

    let mut consumed = 0usize;
    let mut expected_offset = 0usize;

    while !cursor.is_eof() {
        let token = cursor.peek();

        assert!(
            !token.is_eof(),
            "the cursor must not report a non-EOF position as EOF"
        );

        let expected_kind = if consumed % 2 == 0 {
            TokenKind::Identifier
        } else {
            TokenKind::Plus
        };

        assert_eq!(token.kind(), expected_kind);
        assert_eq!(token.span().start_byte(), expected_offset);
        assert_eq!(token.span().end_byte(), expected_offset + 1);

        let before = cursor.raw_index();
        let advanced = cursor.advance();

        assert_eq!(advanced.kind(), expected_kind);
        assert!(
            cursor.raw_index() > before,
            "advancing a non-EOF token must make progress"
        );

        consumed += 1;
        expected_offset += 1;
    }

    assert_eq!(consumed, expected_token_count);
    assert_eq!(cursor.peek().kind(), TokenKind::Eof);
    assert_eq!(cursor.consumed_tokens(), expected_token_count);
    assert_eq!(cursor.span(), span(expected_offset, expected_offset));
}

#[test]
fn lookahead_is_stable_for_large_distances() {
    let tokens = arithmetic_tokens(512);
    let mut cursor = TokenCursor::new(&tokens).expect("valid stream");

    assert_eq!(cursor.peek_n(0).kind(), TokenKind::Identifier);
    assert_eq!(cursor.peek_n(1).kind(), TokenKind::Plus);
    assert_eq!(cursor.peek_n(2).kind(), TokenKind::Identifier);

    // Large lookahead must saturate at EOF instead of overflowing an index.
    assert_eq!(cursor.peek_n(usize::MAX).kind(), TokenKind::Eof);

    let before = cursor.raw_index();
    assert_eq!(cursor.peek_n(usize::MAX).kind(), TokenKind::Eof);
    assert_eq!(cursor.raw_index(), before);
}

#[test]
fn repeated_eof_access_does_not_consume_tokens() {
    let tokens = arithmetic_tokens(8);
    let mut cursor = TokenCursor::new(&tokens).expect("valid stream");

    while !cursor.is_eof() {
        cursor.advance();
    }

    let index_at_eof = cursor.raw_index();
    let consumed_at_eof = cursor.consumed_tokens();

    for _ in 0..1_000 {
        assert!(cursor.is_eof());
        assert_eq!(cursor.peek().kind(), TokenKind::Eof);
        assert_eq!(cursor.advance().kind(), TokenKind::Eof);
        assert_eq!(cursor.peek_n(usize::MAX).kind(), TokenKind::Eof);
    }

    assert_eq!(cursor.raw_index(), index_at_eof);
    assert_eq!(cursor.consumed_tokens(), consumed_at_eof);
}

#[test]
fn checkpoints_restore_large_stream_positions_without_invalidating_work() {
    let tokens = arithmetic_tokens(2_048);
    let mut cursor = TokenCursor::new(&tokens).expect("valid stream");

    for _ in 0..127 {
        cursor.advance();
    }

    let checkpoint = cursor.checkpoint();
    let saved_index = cursor.raw_index();
    let saved_kind = cursor.peek().kind();
    let saved_consumed = cursor.consumed_tokens();

    for _ in 0..257 {
        if cursor.is_eof() {
            break;
        }
        cursor.advance();
    }

    let work_after_speculation = cursor.consumed_tokens();

    cursor
        .restore(checkpoint)
        .expect("checkpoint belongs to this cursor");

    assert_eq!(cursor.raw_index(), saved_index);
    assert_eq!(cursor.peek().kind(), saved_kind);

    // Restoring position must not erase cumulative work already performed.
    assert_eq!(cursor.consumed_tokens(), work_after_speculation);
    assert!(cursor.consumed_tokens() >= saved_consumed);
}

#[test]
fn trivia_heavy_stream_remains_bounded_and_deterministic() {
    let mut tokens = Vec::new();
    let item_count = 4_096usize;

    tokens
        .try_reserve_exact(item_count.saturating_mul(3).saturating_add(1))
        .expect("test environment must reserve trivia workload");

    let mut offset = 0usize;

    for _ in 0..item_count {
        tokens.push(Token::borrowed(
            TokenKind::Identifier,
            "x",
            span(offset, offset + 1),
        ));
        offset += 1;

        tokens.push(
            Token::borrowed(
                TokenKind::Whitespace,
                " ",
                span(offset, offset + 1),
            )
            .with_channel(TokenChannel::Trivia),
        );
        offset += 1;

        tokens.push(Token::borrowed(
            TokenKind::Plus,
            "+",
            span(offset, offset + 1),
        ));
        offset += 1;
    }

    tokens.push(Token::synthetic(
        TokenKind::Eof,
        span(offset, offset),
    ));

    let mut cursor = TokenCursor::new(&tokens).expect("valid trivia-heavy stream");

    // The cursor skips trivia when presenting parser-visible tokens.
    for _ in 0..item_count {
        assert_eq!(cursor.peek().kind(), TokenKind::Identifier);
        cursor.advance();

        assert_eq!(cursor.peek().kind(), TokenKind::Plus);
        cursor.advance();

        // The generated stream ends in a plus token. The final iteration
        // therefore reaches EOF only after the complete visible stream.
        if !cursor.is_eof() {
            assert_eq!(cursor.peek().kind(), TokenKind::Identifier);
        }
    }

    // The cursor must terminate even if the source ends in a syntactically
    // incomplete expression. Syntax validity belongs to grammar parsing.
    while !cursor.is_eof() {
        cursor.advance();
    }

    assert_eq!(cursor.peek().kind(), TokenKind::Eof);
}

#[test]
fn parser_budget_allows_a_stream_when_configured_unlimited() {
    let tokens = arithmetic_tokens(2_048);
    let mut cursor = TokenCursor::new(&tokens).expect("valid stream");
    let mut budget = ParserBudget::new(ParserLimits::unlimited());

    while !cursor.is_eof() {
        cursor
            .advance_with_budget(&mut budget)
            .expect("unlimited token budget permits advancement");
    }

    assert_eq!(
        budget.usage().get(ParserLimitKind::Tokens),
        cursor.consumed_tokens()
    );
}

#[test]
fn token_budget_exhaustion_does_not_advance_the_cursor() {
    let tokens = arithmetic_tokens(16);
    let mut cursor = TokenCursor::new(&tokens).expect("valid stream");

    let allowed = NonZeroUsize::new(3).expect("three is non-zero");
    let limits = ParserLimits {
        tokens: ResourceLimit::AtMost(allowed),
        ..ParserLimits::unlimited()
    };
    let mut budget = ParserBudget::new(limits);

    for _ in 0..3 {
        cursor
            .advance_with_budget(&mut budget)
            .expect("the first three tokens fit the budget");
    }

    let before_index = cursor.raw_index();
    let before_kind = cursor.peek().kind();
    let before_consumed = cursor.consumed_tokens();

    let result = cursor.advance_with_budget(&mut budget);

    assert!(
        result.is_err(),
        "advancing beyond the configured token budget must fail"
    );
    assert_eq!(cursor.raw_index(), before_index);
    assert_eq!(cursor.peek().kind(), before_kind);
    assert_eq!(cursor.consumed_tokens(), before_consumed);
    assert_eq!(
        budget.usage().get(ParserLimitKind::Tokens),
        3,
        "failed budget accounting must not increment token usage"
    );
}

#[test]
fn eof_does_not_consume_the_token_budget() {
    let tokens = single_identifier_tokens();
    let mut cursor = TokenCursor::new(&tokens).expect("valid stream");
    let mut budget = ParserBudget::new(ParserLimits::unlimited());

    cursor
        .advance_with_budget(&mut budget)
        .expect("identifier fits the budget");

    assert!(cursor.is_eof());

    let usage_before_eof = budget.usage().get(ParserLimitKind::Tokens);

    cursor
        .advance_with_budget(&mut budget)
        .expect("EOF is stable");

    assert_eq!(
        budget.usage().get(ParserLimitKind::Tokens),
        usage_before_eof
    );
}

#[test]
fn malformed_missing_eof_stream_is_rejected() {
    let tokens = vec![Token::borrowed(
        TokenKind::Identifier,
        "x",
        span(0, 1),
    )];

    assert!(
        TokenCursor::new(&tokens).is_err(),
        "a complete token stream must contain explicit EOF"
    );
}

#[test]
fn malformed_early_eof_stream_is_rejected() {
    let tokens = vec![
        Token::synthetic(TokenKind::Eof, span(0, 0)),
        Token::borrowed(TokenKind::Identifier, "x", span(0, 1)),
    ];

    assert!(
        TokenCursor::new(&tokens).is_err(),
        "EOF cannot appear before the final token"
    );
}

#[test]
fn malformed_duplicate_eof_stream_is_rejected() {
    let tokens = vec![
        Token::synthetic(TokenKind::Eof, span(0, 0)),
        Token::synthetic(TokenKind::Eof, span(0, 0)),
    ];

    assert!(
        TokenCursor::new(&tokens).is_err(),
        "a stream must contain exactly one EOF token"
    );
}

#[test]
fn malformed_nonzero_width_eof_is_rejected() {
    let tokens = vec![Token::synthetic(TokenKind::Eof, span(0, 1))];

    assert!(
        TokenCursor::new(&tokens).is_err(),
        "EOF must have a zero-width source span"
    );
}

#[test]
fn overlapping_token_spans_are_rejected() {
    let tokens = vec![
        Token::borrowed(TokenKind::Identifier, "abc", span(0, 3)),
        Token::borrowed(TokenKind::Plus, "+", span(2, 3)),
        Token::synthetic(TokenKind::Eof, span(3, 3)),
    ];

    assert!(
        TokenCursor::new(&tokens).is_err(),
        "overlapping token spans must not enter the parser"
    );
}

#[test]
fn reversed_token_spans_are_rejected() {
    let tokens = vec![
        Token::borrowed(TokenKind::Identifier, "x", span(2, 3)),
        Token::borrowed(TokenKind::Plus, "+", span(1, 2)),
        Token::synthetic(TokenKind::Eof, span(3, 3)),
    ];

    assert!(
        TokenCursor::new(&tokens).is_err(),
        "token spans must be ordered"
    );
}

#[test]
fn unicode_byte_spans_are_preserved() {
    // λ occupies two bytes in UTF-8. Cursor boundaries are byte-based.
    let tokens = vec![
        Token::borrowed(TokenKind::Identifier, "λ", span(0, 2)),
        Token::borrowed(TokenKind::Plus, "+", span(2, 3)),
        Token::borrowed(TokenKind::Identifier, "x", span(3, 4)),
        Token::synthetic(TokenKind::Eof, span(4, 4)),
    ];

    let mut cursor = TokenCursor::new(&tokens).expect("valid Unicode spans");

    assert_eq!(cursor.peek().lexeme(), "λ");
    assert_eq!(cursor.peek().span().start_byte(), 0);
    assert_eq!(cursor.peek().span().end_byte(), 2);

    cursor.advance();

    assert_eq!(cursor.peek().kind(), TokenKind::Plus);
    assert_eq!(cursor.peek().span().start_byte(), 2);

    cursor.advance();

    assert_eq!(cursor.peek().lexeme(), "x");
    assert_eq!(cursor.peek().span().start_byte(), 3);
    assert_eq!(cursor.peek().span().end_byte(), 4);
}

#[test]
fn repeated_cursor_construction_is_deterministic() {
    let tokens = arithmetic_tokens(1_024);

    let first = TokenCursor::new(&tokens).expect("valid stream");
    let second = TokenCursor::new(&tokens).expect("same valid stream");

    assert_eq!(first.raw_index(), second.raw_index());
    assert_eq!(first.consumed_tokens(), second.consumed_tokens());
    assert_eq!(first.peek().kind(), second.peek().kind());
    assert_eq!(first.peek().lexeme(), second.peek().lexeme());
    assert_eq!(first.span(), second.span());

    for distance in [0, 1, 2, 7, 31, 127, usize::MAX] {
        assert_eq!(
            first.peek_n(distance).kind(),
            second.peek_n(distance).kind(),
            "lookahead must be deterministic at distance {distance}"
        );
    }
}

/// An opt-in larger workload for local profiling and CI stress jobs.
///
/// Run with:
///
/// `cargo test --lib frontend::parser::tests::stress::million_token_stream -- --ignored`
///
/// This test is intentionally ignored by default because its memory and runtime
/// costs vary by environment. It is a workload, not a supported-size ceiling.
#[test]
#[ignore = "opt-in large parser token-stream workload"]
fn million_token_stream_terminates_and_preserves_token_accounting() {
    let operand_count = 500_001usize;
    let tokens = arithmetic_tokens(operand_count);
    let expected_non_eof = operand_count
        .checked_mul(2)
        .expect("test workload must fit usize")
        - 1;

    let mut cursor = TokenCursor::new(&tokens).expect("valid large stream");
    let mut budget = ParserBudget::new(ParserLimits::unlimited());

    while !cursor.is_eof() {
        cursor
            .advance_with_budget(&mut budget)
            .expect("unlimited configured budget");
    }

    assert_eq!(cursor.consumed_tokens(), expected_non_eof);
    assert_eq!(
        budget.usage().get(ParserLimitKind::Tokens),
        expected_non_eof
    );
    assert_eq!(cursor.peek().kind(), TokenKind::Eof);
}
