#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Parser recovery contract tests for the Zamani frontend.
//!
//! File: src/frontend/parser/tests/recovery.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verify that the production recovery implementation:
//! - synchronizes on caller-selected grammar boundaries;
//! - preserves synchronization tokens when requested;
//! - optionally consumes a synchronization token;
//! - stops at EOF without advancing past it;
//! - respects explicit recovery-work budgets;
//! - makes bounded, observable progress;
//! - behaves deterministically for equivalent inputs;
//! - records recovery in the owning parser session;
//! - keeps independent parser sessions isolated;
//! - does not introduce hardware-capacity limits.
//!
//! # Ownership
//!
//! These tests exercise the production recovery engine. They do not define
//! another lexer, token cursor, parser session, grammar, AST, or diagnostic
//! system.
//!
//! # Integration contract
//!
//! Required production dependencies:
//! - `lexer::token::{Token, TokenChannel}`;
//! - `lexer::token_kind::TokenKind`;
//! - `lexer::cursor::CursorPosition`;
//! - `lexer::span::Span`;
//! - `parser::token_cursor::TokenCursor`;
//! - `parser::session::{ParserSession, SessionId}`;
//! - `parser::token_sets::TokenSet`;
//! - `parser::recovery::{RecoveryEngine, RecoveryPolicy, RecoveryStopReason}`.
//!
//! The recovery module must expose the API exercised below. If its final
//! approved names differ, reconcile this file against that frozen API before
//! merging; do not add a second recovery implementation inside the tests.
//!
//! # Recovery contract exercised here
//!
//! `RecoveryEngine::new(policy)` constructs a deterministic recovery engine.
//! `synchronize(session, synchronization_tokens)` returns a structured
//! outcome describing skipped tokens, the stopping condition, and whether
//! the synchronization token was consumed.
//!
//! The policy supports:
//! - an optional maximum number of skipped tokens;
//! - explicit control over consuming the synchronization token.
//!
//! The outcome exposes:
//! - `skipped_tokens()`;
//! - `stop_reason()`;
//! - `synchronization_token()`;
//! - `synchronization_token_consumed()`.
//!
//! `RecoveryStopReason` distinguishes reaching a synchronization token,
//! reaching EOF, and exhausting the configured skip budget.
//!
//! These names form the proposed contract for the missing production module;
//! they are not assertions that the current repository already exports them.
//!
//! # Scalability and safety
//!
//! No test introduces a maximum number of qubits, processors, devices,
//! registers, tensor dimensions, declarations, or other program concepts.
//! Test workload sizes are test parameters, not language restrictions.
//!
//! Unlimited recovery budgets do not promise unlimited physical memory,
//! stack space, address space, or execution time.

use core::num::NonZeroUsize;

use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;

use crate::frontend::parser::recovery::{
    RecoveryEngine,
    RecoveryPolicy,
    RecoveryStopReason,
};
use crate::frontend::parser::session::{
    ParserSession,
    SessionId,
};
use crate::frontend::parser::token_cursor::TokenCursor;
use crate::frontend::parser::token_sets::TokenSet;

/// Construct a source span using UTF-8 byte offsets.
///
/// These fixtures are single-line inputs. Production source-position logic
/// remains owned by the lexer.
fn span(start_byte: usize, end_byte: usize) -> Span {
    Span::new(
        CursorPosition::new(start_byte, 1, start_byte + 1),
        CursorPosition::new(end_byte, 1, end_byte + 1),
    )
}

/// Build a canonical token stream from token kinds and source spellings.
///
/// Each token's span follows its spelling's byte length. The final EOF token
/// has a zero-width span at the end of the stream.
///
/// This helper deliberately uses the real lexer Token and Span types.
fn make_tokens(parts: &[(TokenKind, &'static str)]) -> Vec<Token<'static>> {
    let mut tokens = Vec::with_capacity(parts.len() + 1);
    let mut offset = 0usize;

    for &(kind, lexeme) in parts {
        let end = offset
            .checked_add(lexeme.len())
            .expect("test fixture byte offsets must fit usize");

        tokens.push(Token::borrowed(kind, lexeme, span(offset, end)));
        offset = end;
    }

    tokens.push(Token::synthetic(TokenKind::Eof, span(offset, offset)));
    tokens
}

/// Construct a fresh parser session around the canonical token cursor.
///
/// A distinct session ID is supplied by each test. These IDs are fixture
/// identities, not global counters or hardware identifiers.
fn make_session<'tokens>(
    id: u64,
    tokens: &'tokens [Token<'static>],
) -> ParserSession<'tokens, 'static> {
    let cursor = TokenCursor::new(tokens).expect("fixture token stream must be valid");

    ParserSession::unlimited(SessionId::new(id), cursor)
        .expect("an unlimited parser session should accept a valid cursor")
}

/// Construct a token set for a grammar synchronization boundary.
fn sync_set(name: &str, kinds: &[TokenKind]) -> TokenSet {
    TokenSet::from_static(name, kinds)
}

/// Create the default policy: no explicit skip ceiling and no consumption
/// of the synchronization token.
fn default_engine() -> RecoveryEngine {
    RecoveryEngine::new(RecoveryPolicy::default())
}

#[test]
fn skips_unexpected_tokens_until_the_requested_boundary() {
    let tokens = make_tokens(&[
        (TokenKind::Identifier, "broken"),
        (TokenKind::Plus, "+"),
        (TokenKind::Identifier, "expression"),
        (TokenKind::Semicolon, ";"),
        (TokenKind::Identifier, "next"),
    ]);

    let mut session = make_session(1, &tokens);
    let synchronization = sync_set("statement terminators", &[TokenKind::Semicolon]);
    let mut recovery = default_engine();

    let outcome = recovery
        .synchronize(&mut session, &synchronization)
        .expect("recovery should reach the statement boundary");

    assert_eq!(outcome.skipped_tokens(), 3);
    assert_eq!(
        outcome.stop_reason(),
        RecoveryStopReason::SynchronizationToken
    );
    assert_eq!(
        outcome.synchronization_token(),
        Some(TokenKind::Semicolon)
    );
    assert!(!outcome.synchronization_token_consumed());

    assert_eq!(session.cursor().peek().kind(), TokenKind::Semicolon);
    assert!(session.has_recovered());
}

#[test]
fn preserves_the_synchronization_token_by_default() {
    let tokens = make_tokens(&[
        (TokenKind::Identifier, "invalid"),
        (TokenKind::Comma, ","),
        (TokenKind::Identifier, "following"),
    ]);

    let mut session = make_session(2, &tokens);
    let synchronization = sync_set("argument separator", &[TokenKind::Comma]);
    let mut recovery = default_engine();

    let outcome = recovery
        .synchronize(&mut session, &synchronization)
        .expect("recovery should stop at the comma");

    assert_eq!(outcome.skipped_tokens(), 1);
    assert!(!outcome.synchronization_token_consumed());
    assert_eq!(session.cursor().peek().kind(), TokenKind::Comma);
}

#[test]
fn optionally_consumes_the_synchronization_token() {
    let tokens = make_tokens(&[
        (TokenKind::Identifier, "invalid"),
        (TokenKind::Semicolon, ";"),
        (TokenKind::Identifier, "following"),
    ]);

    let mut session = make_session(3, &tokens);
    let synchronization = sync_set("statement terminator", &[TokenKind::Semicolon]);

    let policy = RecoveryPolicy::default()
        .with_consume_synchronization_token(true);

    let mut recovery = RecoveryEngine::new(policy);

    let outcome = recovery
        .synchronize(&mut session, &synchronization)
        .expect("recovery should consume the configured boundary");

    assert_eq!(outcome.skipped_tokens(), 1);
    assert!(outcome.synchronization_token_consumed());
    assert_eq!(session.cursor().peek().kind(), TokenKind::Identifier);
    assert_eq!(session.cursor().peek().lexeme(), "following");
}

#[test]
fn stops_at_eof_when_no_synchronization_token_remains() {
    let tokens = make_tokens(&[
        (TokenKind::Identifier, "broken"),
        (TokenKind::Plus, "+"),
    ]);

    let mut session = make_session(4, &tokens);
    let synchronization = sync_set(
        "unreachable boundary",
        &[TokenKind::Semicolon, TokenKind::RightBrace],
    );

    let mut recovery = default_engine();

    let outcome = recovery
        .synchronize(&mut session, &synchronization)
        .expect("recovery should terminate at EOF");

    assert_eq!(outcome.skipped_tokens(), 2);
    assert_eq!(outcome.stop_reason(), RecoveryStopReason::EndOfInput);
    assert_eq!(outcome.synchronization_token(), None);
    assert!(!outcome.synchronization_token_consumed());
    assert!(session.is_eof());
}

#[test]
fn eof_is_never_consumed_or_skipped_as_source() {
    let tokens = make_tokens(&[]);
    let mut session = make_session(5, &tokens);

    let synchronization = sync_set(
        "all supported boundaries",
        &[
            TokenKind::Semicolon,
            TokenKind::Comma,
            TokenKind::RightParen,
            TokenKind::RightBrace,
        ],
    );

    let mut recovery = default_engine();

    let first = recovery
        .synchronize(&mut session, &synchronization)
        .expect("EOF recovery should terminate");

    assert_eq!(first.skipped_tokens(), 0);
    assert_eq!(first.stop_reason(), RecoveryStopReason::EndOfInput);
    assert!(session.is_eof());

    let second = recovery
        .synchronize(&mut session, &synchronization)
        .expect("repeated EOF recovery should terminate");

    assert_eq!(second.skipped_tokens(), 0);
    assert_eq!(second.stop_reason(), RecoveryStopReason::EndOfInput);
    assert!(session.is_eof());
}

#[test]
fn does_not_skip_a_boundary_already_at_the_cursor() {
    let tokens = make_tokens(&[
        (TokenKind::Semicolon, ";"),
        (TokenKind::Identifier, "next"),
    ]);

    let mut session = make_session(6, &tokens);
    let synchronization = sync_set("statement terminator", &[TokenKind::Semicolon]);
    let mut recovery = default_engine();

    let outcome = recovery
        .synchronize(&mut session, &synchronization)
        .expect("the current token is already a synchronization boundary");

    assert_eq!(outcome.skipped_tokens(), 0);
    assert_eq!(
        outcome.stop_reason(),
        RecoveryStopReason::SynchronizationToken
    );
    assert_eq!(session.cursor().peek().kind(), TokenKind::Semicolon);
}

#[test]
fn respects_the_explicit_skip_budget_without_crossing_the_budget() {
    let tokens = make_tokens(&[
        (TokenKind::Identifier, "one"),
        (TokenKind::Plus, "+"),
        (TokenKind::Identifier, "two"),
        (TokenKind::Semicolon, ";"),
        (TokenKind::Identifier, "next"),
    ]);

    let mut session = make_session(7, &tokens);
    let synchronization = sync_set("statement terminator", &[TokenKind::Semicolon]);

    let budget = NonZeroUsize::new(2).expect("two is nonzero");
    let policy = RecoveryPolicy::default().with_max_skipped_tokens(Some(budget));
    let mut recovery = RecoveryEngine::new(policy);

    let outcome = recovery
        .synchronize(&mut session, &synchronization)
        .expect("skip-budget exhaustion should be a structured outcome");

    assert_eq!(outcome.skipped_tokens(), 2);
    assert_eq!(
        outcome.stop_reason(),
        RecoveryStopReason::SkipBudgetExhausted
    );

    // The third token and the synchronization boundary remain unconsumed.
    assert_eq!(session.cursor().peek().kind(), TokenKind::Identifier);
    assert_eq!(session.cursor().peek().lexeme(), "two");
}

#[test]
fn skip_budget_of_one_allows_at_most_one_skipped_token() {
    let tokens = make_tokens(&[
        (TokenKind::Identifier, "unexpected"),
        (TokenKind::Plus, "+"),
        (TokenKind::Semicolon, ";"),
    ]);

    let mut session = make_session(8, &tokens);
    let synchronization = sync_set("statement terminator", &[TokenKind::Semicolon]);

    let budget = NonZeroUsize::new(1).expect("one is nonzero");
    let policy = RecoveryPolicy::default().with_max_skipped_tokens(Some(budget));
    let mut recovery = RecoveryEngine::new(policy);

    let outcome = recovery
        .synchronize(&mut session, &synchronization)
        .expect("recovery should respect a one-token budget");

    assert_eq!(outcome.skipped_tokens(), 1);
    assert_eq!(
        outcome.stop_reason(),
        RecoveryStopReason::SkipBudgetExhausted
    );
    assert_eq!(session.cursor().peek().kind(), TokenKind::Plus);
}

#[test]
fn recovery_does_not_depend_on_hardware_or_domain_specific_token_counts() {
    let tokens = make_tokens(&[
        (TokenKind::Identifier, "requires"),
        (TokenKind::Identifier, "