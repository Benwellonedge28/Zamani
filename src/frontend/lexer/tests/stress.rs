#![forbid(unsafe_code)]

//! Stress and scalability regression tests for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/tests/stress.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Exercise the real lexer implementation with progressively larger inputs,
//! long identifiers and literals, Unicode text, many tokens, and configured
//! resource budgets.
//!
//! # Integration contract
//!
//! This module tests the existing implementations in:
//!
//! - `api.rs`: streaming `TokenSource` contract;
//! - `config.rs`: lexical configuration and explicit resource budgets;
//! - `cursor.rs`: UTF-8-safe source traversal;
//! - `keyword_registry.rs`: configurable keyword classification;
//! - `operator_registry.rs`: configurable operator matching;
//! - `scanner.rs`: streaming lexical recognition and error reporting;
//! - `span.rs`: half-open source ranges;
//! - `token.rs`: token spelling, identity, and channel;
//! - `token_kind.rs`: canonical token kinds;
//! - `grammar/spec/lexical.md`: normative lexical behavior;
//! - `grammar/specification/lexical.md`: lexical specification;
//! - `grammar/lexer/identifiers.g4`: Unicode identifier rules;
//! - `grammar/Zamani.g4`: canonical grammar composition.
//!
//! The tests do not implement a second scanner, impose a language-level
//! maximum input size, or assume any particular hardware capacity.
//!
//! # Resource policy
//!
//! Every finite fixture in this file is a test workload, not a language
//! limit. Production limits are explicit configuration choices. An unlimited
//! budget means that the lexer imposes no additional configured ceiling;
//! actual execution remains bounded by available memory, address space,
//! execution time, and operating-system constraints.
//!
//! # Registration
//!
//! Register this module from `src/frontend/lexer/tests/mod.rs`:
//!
//!     mod stress;
//!
//! Register the test module through the existing `#[cfg(test)]` module
//! hierarchy in `src/frontend/lexer/mod.rs`.
//!
//! # Running
//!
//!     cargo test frontend::lexer::tests::stress
//!
//! These tests verify lexer properties at representative scales. They do
//! not prove literal infinite capacity or guarantee that every possible
//! input is accepted by the language.

use crate::frontend::lexer::api::TokenSource;
use crate::frontend::lexer::config::{
    LexerConfig,
    ResourceLimit,
};
use crate::frontend::lexer::cursor::{Cursor, CursorPosition};
use crate::frontend::lexer::keyword_registry::KeywordRegistry;
use crate::frontend::lexer::operator_registry::OperatorRegistry;
use crate::frontend::lexer::scanner::{
    Scanner,
    ScannerError,
    ScannerErrorKind,
};
use crate::frontend::lexer::token::TokenChannel;
use crate::frontend::lexer::token_kind::TokenKind;

use core::ops::Range;

/// The stable, observable information retained from one emitted token.
///
/// Source spans use zero-based UTF-8 byte offsets and half-open ranges.
/// The snapshot intentionally excludes scanner internals and allocation
/// details so implementation refactoring does not weaken the tests.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenSnapshot {
    kind: TokenKind,
    lexeme: String,
    byte_range: Range<usize>,
    channel: TokenChannel,
}

impl TokenSnapshot {
    fn from_token(
        token: &crate::frontend::lexer::token::Token<'_>,
    ) -> Self {
        Self {
            kind: token.kind(),
            lexeme: token.lexeme().to_owned(),
            byte_range: token.span().byte_range(),
            channel: token.channel(),
        }
    }
}

/// Observable result of a complete or failed scan.
///
/// Retaining the tokens emitted before an error makes it possible to check
/// repeatability without hiding partial progress.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ScanOutcome {
    tokens: Vec<TokenSnapshot>,
    error: Option<ScanFailure>,
}

/// Stable, test-local representation of a scanner failure.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ScanFailure {
    kind: ScannerErrorKind,
    byte_range: Range<usize>,
    message: String,
}

impl From<&ScannerError> for ScanFailure {
    fn from(error: &ScannerError) -> Self {
        Self {
            kind: error.kind(),
            byte_range: error.byte_range(),
            message: error.message().to_owned(),
        }
    }
}

/// Creates a scanner with explicit empty registries.
///
/// These tests use ordinary identifiers, numeric literals, strings,
/// whitespace, and punctuation supported independently of a global
/// keyword/operator catalog. Domain-specific words remain identifiers
/// unless registered by the caller.
fn scanner_for(
    source: &str,
    config: LexerConfig,
) -> Result<Scanner<'_>, ScannerError> {
    Scanner::new(
        source,
        config,
        KeywordRegistry::new(),
        OperatorRegistry::new(),
    )
}

/// Scans to exhaustion and records every observable token.
///
/// A safety guard is intentionally not implemented as an arbitrary token
/// count: the configured budget and source length are the relevant controls.
/// Progress is verified by checking that each non-EOF token has a valid
/// source range and that the emitted stream terminates.
fn scan_all(
    source: &str,
    config: LexerConfig,
) -> ScanOutcome {
    let mut scanner = match scanner_for(source, config) {
        Ok(scanner) => scanner,
        Err(error) => {
            return ScanOutcome {
                tokens: Vec::new(),
                error: Some(ScanFailure::from(&error)),
            };
        }
    };

    let mut tokens = Vec::new();

    loop {
        match scanner.next_token() {
            Ok(Some(token)) => {
                let is_eof = token.kind() == TokenKind::Eof;
                tokens.push(TokenSnapshot::from_token(&token));

                if is_eof {
                    // The canonical default configuration emits one EOF
                    // token, then exhausts the stream.
                    return match scanner.next_token() {
                        Ok(None) => ScanOutcome {
                            tokens,
                            error: None,
                        },
                        Ok(Some(extra)) => {
                            let start = extra.span().start();
                            let end = extra.span().end();

                            ScanOutcome {
                                tokens,
                                error: Some(ScanFailure {
                                    kind: ScannerErrorKind::InternalInvariant,
                                    byte_range: start.byte_offset()
                                        ..end.byte_offset(),
                                    message: String::from(
                                        "scanner emitted a token after EOF",
                                    ),
                                }),
                            }
                        }
                        Err(error) => ScanOutcome {
                            tokens,
                            error: Some(ScanFailure::from(&error)),
                        },
                    };
                }
            }
            Ok(None) => {
                return ScanOutcome {
                    tokens,
                    error: None,
                };
            }
            Err(error) => {
                return ScanOutcome {
                    tokens,
                    error: Some(ScanFailure::from(&error)),
                };
            }
        }
    }
}

/// Checks that each token range is within the source and lies on UTF-8
/// character boundaries. Non-synthetic token spelling must equal its
/// original source slice.
fn assert_token_ranges_are_valid(
    source: &str,
    tokens: &[TokenSnapshot],
) {
    for token in tokens {
        let range = &token.byte_range;

        assert!(
            range.start <= range.end,
            "reversed token range: {token:?}"
        );
        assert!(
            range.end <= source.len(),
            "token extends beyond source: {token:?}"
        );
        assert!(
            source.is_char_boundary(range.start),
            "token start is not a UTF-8 boundary: {token:?}"
        );
        assert!(
            source.is_char_boundary(range.end),
            "token end is not a UTF-8 boundary: {token:?}"
        );

        assert_eq!(
            &source[range.clone()],
            token.lexeme.as_str(),
            "token spelling differs from its original source slice: {token:?}"
        );
    }
}

fn assert_successful_scan(
    source: &str,
    outcome: &ScanOutcome,
) {
    assert!(
        outcome.error.is_none(),
        "expected successful scan of {} source bytes; got {:?}",
        source.len(),
        outcome.error
    );

    assert_token_ranges_are_valid(source, &outcome.tokens);
}

// ---------------------------------------------------------------------------
// Cursor stress
// ---------------------------------------------------------------------------

#[test]
fn cursor_consumes_large_unicode_source_without_losing_utf8_alignment() {
    // This is a finite stress fixture, not an identifier or source limit.
    let source = "λ界🛰️".repeat(8_192);
    let expected_scalars = source.chars().count();
    let expected_bytes = source.len();

    let mut cursor = Cursor::new(&source);
    let mut consumed_scalars = 0usize;

    while cursor.consume().is_some() {
        consumed_scalars = consumed_scalars
            .checked_add(1)
            .expect("test scalar counter should remain representable");

        assert!(cursor.byte_offset() <= expected_bytes);
        assert!(source.is_char_boundary(cursor.byte_offset()));
    }

    assert_eq!(consumed_scalars, expected_scalars);
    assert_eq!(cursor.byte_offset(), expected_bytes);
    assert_eq!(cursor.position().byte_offset(), expected_bytes);
    assert_eq!(
        cursor.position().column(),
        expected_scalars.saturating_add(1)
    );
    assert!(cursor.is_eof());

    // EOF is stable and repeated reads must not advance the cursor.
    let eof_position = cursor.position();

    for _ in 0..32 {
        assert_eq!(cursor.consume(), None);
        assert_eq!(cursor.position(), eof_position);
    }
}

#[test]
fn cursor_checkpoints_restore_large_source_positions() {
    let source = format!("{}tail", "α".repeat(16_384));
    let mut cursor = Cursor::new(&source);

    for _ in 0..8_192 {
        assert!(cursor.consume().is_some());
    }

    let checkpoint = cursor.checkpoint();
    let checkpoint_position = cursor.position();

    for _ in 0..128 {
        let _ = cursor.consume();
    }

    cursor
        .restore(checkpoint)
        .expect("checkpoint from the same source must restore");

    assert_eq!(cursor.position(), checkpoint_position);
    assert_eq!(cursor.byte_offset(), checkpoint.byte_offset());
    assert_eq!(cursor.remaining(), &source[checkpoint.byte_offset()..]);
}

#[test]
fn cursor_positions_remain_byte_based_for_multibyte_scalars() {
    let source = "aλ界🛰️z";
    let mut cursor = Cursor::new(source);

    let expected = [
        (1usize, 1usize, 2usize),
        (3, 1, 3),
        (6, 1, 4),
    ];

    for (byte_offset, line, column) in expected {
        assert!(cursor.consume().is_some());

        assert_eq!(
            cursor.position(),
            CursorPosition::new(byte_offset, line, column)
        );
    }

    assert!(source.is_char_boundary(cursor.byte_offset()));
}

// ---------------------------------------------------------------------------
// Long identifier and literal stress
// ---------------------------------------------------------------------------

#[test]
fn scanner_accepts_a_long_unicode_identifier_with_unlimited_budget() {
    let identifier = format!("λ{}", "界".repeat(32_768));
    let source = identifier.as_str();

    let outcome = scan_all(source, LexerConfig::default());

    assert_successful_scan(source, &outcome);
    assert_eq!(outcome.tokens.len(), 2);
    assert_eq!(outcome.tokens[0].kind, TokenKind::Identifier);
    assert_eq!(outcome.tokens[0].lexeme, identifier);
    assert_eq!(outcome.tokens[0].byte_range, 0..source.len());
    assert_eq!(outcome.tokens[1].kind, TokenKind::Eof);
    assert!(outcome.tokens[1].byte_range.is_empty());
}

#[test]
fn scanner_accepts_a_long_string_literal_with_unlimited_budget() {
    let content = "λ世界🙂".repeat(16_384);
    let source = format!("\"{content}\"");

    let outcome = scan_all(&source, LexerConfig::default());

    assert_successful_scan(&source, &outcome);
    assert_eq!(outcome.tokens.len(), 2);
    assert_eq!(outcome.tokens[0].kind, TokenKind::StringLiteral);
    assert_eq!(outcome.tokens[0].lexeme, source);
    assert_eq!(outcome.tokens[0].byte_range, 0..source.len());
    assert_eq!(outcome.tokens[1].kind, TokenKind::Eof);
}

#[test]
fn scanner_preserves_large_integer_source_spelling() {
    // Numeric conversion belongs to a later stage. The lexer must preserve
    // the spelling without first converting it to a machine integer.
    let digits = "1234567890".repeat(4_096);
    let source = digits.as_str();

    let outcome = scan_all(source, LexerConfig::default());

    assert_successful_scan(source, &outcome);
    assert_eq!(outcome.tokens.len(), 2);
    assert_eq!(outcome.tokens[0].kind, TokenKind::IntegerLiteral);
    assert_eq!(outcome.tokens[0].lexeme, digits);
    assert_eq!(outcome.tokens[0].byte_range, 0..source.len());
}

// ---------------------------------------------------------------------------
// Large token streams and determinism
// ---------------------------------------------------------------------------

#[test]
fn scanner_handles_many_tokens_without_changing_source_ranges() {
    let mut source = String::new();

    // This count controls the test workload only. It is not a language
    // limit, a token ceiling, or a hardware capacity.
    for index in 0..2_048usize {
        use core::fmt::Write as _;

        writeln!(
            &mut source,
            "value_{index} {index} \"payload_{index}\""
        )
        .expect("writing to a String must succeed");
    }

    let outcome = scan_all(&source, LexerConfig::default());

    assert_successful_scan(&source, &outcome);

    // Each generated line contains an identifier, integer and string,
    // followed by the one explicit EOF token.
    assert_eq!(outcome.tokens.len(), 2_048 * 3 + 1);
    assert_eq!(
        outcome.tokens.last().map(|token| token.kind),
        Some(TokenKind::Eof)
    );

    assert!(
        outcome.tokens.iter().any(|token| {
            token.kind == TokenKind::StringLiteral
        }),
        "the generated stream should contain string literals"
    );

    assert!(
        outcome.tokens.iter().any(|token| {
            token.kind == TokenKind::IntegerLiteral
        }),
        "the generated stream should contain integer literals"
    );
}

#[test]
fn repeated_scans_of_large_input_are_observationally_identical() {
    let mut source = String::new();

    for index in 0..1_024usize {
        use core::fmt::Write as _;

        writeln!(
            &mut source,
            "register_{index} 0x{:X} \"λ_{index}\"",
            index
        )
        .expect("writing to a String must succeed");
    }

    let first = scan_all(&source, LexerConfig::default());
    assert_successful_scan(&source, &first);

    for repetition in 0..3 {
        let next = scan_all(&source, LexerConfig::default());

        assert_eq!(
            next,
            first,
            "identical source/configuration must produce identical \
             tokens and failures; repetition {repetition}"
        );
    }
}

#[test]
fn source_with_mixed_computing_domain_names_is_lexically_domain_neutral() {
    let source = concat!(
        "classical_value quantum_register hdl_signal ",
        "tensor_shape distributed_node accelerator_task ",
        "custom_qpu_vendor future_compute_target"
    );

    let outcome = scan_all(source, LexerConfig::default());

    assert_successful_scan(source, &outcome);

    let identifiers: Vec<&str> = outcome.tokens
        .iter()
        .filter(|token| token.kind == TokenKind::Identifier)
        .map(|token| token.lexeme.as_str())
        .collect();

    assert_eq!(
        identifiers,
        vec![
            "classical_value",
            "quantum_register",
            "hdl_signal",
            "tensor_shape",
            "distributed_node",
            "accelerator_task",
            "custom_qpu_vendor",
            "future_compute_target",
        ]
    );
}

// ---------------------------------------------------------------------------
// Explicit resource-budget boundaries
// ---------------------------------------------------------------------------

#[test]
fn source_budget_accepts_input_at_the_configured_boundary() {
    let source = "alpha";
    let mut config = LexerConfig::default();

    config.limits.source_bytes =
        ResourceLimit::at_most(source.len())
            .expect("nonzero source length has a valid budget");

    let outcome = scan_all(source, config);

    assert_successful_scan(source, &outcome);
}

#[test]
fn source_budget_rejects_input_above_the_configured_boundary() {
    let source = "alpha";
    let mut config = LexerConfig::default();

    config.limits.source_bytes =
        ResourceLimit::at_most(source.len() - 1)
            .expect("the test source has more than one byte");

    let first = scan_all(source, config);

    assert_eq!(
        first.error.as_ref().map(|error| error.kind),
        Some(ScannerErrorKind::SourceBudgetExceeded)
    );

    let failure = first.error
        .as_ref()
        .expect("over-budget input must fail");

    assert_eq!(failure.byte_range, 0..source.len());
    assert!(!failure.message.is_empty());

    // Resource failures are deterministic for identical source and policy.
    let second = scan_all(source, config);
    assert_eq!(second, first);
}

#[test]
fn token_budget_is_enforced_without_a_language_level_token_ceiling() {
    let source = "alpha beta gamma";
    let mut config = LexerConfig::default();

    // The scanner's token budget includes emitted tokens, including EOF
    // when enabled. A budget of one cannot accommodate this whole stream.
    config.limits.tokens =
        ResourceLimit::at_most(1)
            .expect("one token is a valid explicit budget");

    let outcome = scan_all(source, config);

    assert_eq!(
        outcome.error.as_ref().map(|error| error.kind),
        Some(ScannerErrorKind::TokenBudgetExceeded)
    );
}

#[test]
fn token_byte_budget_rejects_an_overlong_lexeme_explicitly() {
    let source = "identifier";
    let mut config = LexerConfig::default();

    config.limits.token_bytes =
        ResourceLimit::at_most(4)
            .expect("four bytes is a valid explicit budget");

    let outcome = scan_all(source, config);

    assert_eq!(
        outcome.error.as_ref().map(|error| error.kind),
        Some(ScannerErrorKind::TokenBudgetForLexemeExceeded)
    );
}

#[test]
fn explicit_unlimited_budgets_remain_unlimited_for_all_budget_fields() {
    let limits = crate::frontend::lexer::config::LexerLimits::unlimited();

    assert!(limits.source_bytes.permits(usize::MAX));
    assert!(limits.tokens.permits(usize::MAX));
    assert!(limits.token_bytes.permits(usize::MAX));
    assert!(limits.numeric_digits.permits(usize::MAX));
    assert!(limits.literal_bytes.permits(usize::MAX));
    assert!(limits.lexical_nesting_depth.permits(usize::MAX));
    assert!(limits.diagnostics.permits(usize::MAX));
}

// ---------------------------------------------------------------------------
// Terminal behavior and repeatable failures
// ---------------------------------------------------------------------------

#[test]
fn explicit_eof_is_emitted_once_for_empty_and_nonempty_sources() {
    for source in ["", "identifier", "123", "\"text\""] {
        let mut scanner = scanner_for(
            source,
            LexerConfig::default(),
        )
        .expect("default scanner configuration should be valid");

        let mut eof_count = 0usize;

        loop {
            match scanner.next_token() {
                Ok(Some(token)) => {
                    if token.kind() == TokenKind::Eof {
                        eof_count += 1;
                    }
                }
                Ok(None) => break,
                Err(error) => {
                    panic!(
                        "unexpected scan error for {source:?}: {error}"
                    );
                }
            }
        }

        assert_eq!(
            eof_count, 1,
            "the explicit EOF token must occur exactly once for {source:?}"
        );

        assert!(scanner.next_token().expect("exhausted source").is_none());
    }
}

#[test]
fn malformed_string_failure_is_repeatable() {
    let source = "\"unterminated";
    let first = scan_all(source, LexerConfig::default());

    assert_eq!(
        first.error.as_ref().map(|error| error.kind),
        Some(ScannerErrorKind::InvalidLiteral)
    );

    let second = scan_all(source, LexerConfig::default());

    assert_eq!(
        second,
        first,
        "the same malformed source must produce the same failure"
    );
}

#[test]
fn initial_bom_policy_is_enforced_consistently() {
    let source = "\u{FEFF}identifier";
    let mut config = LexerConfig::default();

    config.allow_initial_bom = false;

    let first = scan_all(source, config);

    assert_eq!(
        first.error.as_ref().map(|error| error.kind),
        Some(ScannerErrorKind::InitialBomNotAllowed)
    );

    let second = scan_all(source, config);
    assert_eq!(second, first);
}

#[test]
fn accepted_initial_bom_does_not_corrupt_following_token_spans() {
    let source = "\u{FEFF}identifier";
    let mut config = LexerConfig::default();

    config.allow_initial_bom = true;

    let outcome = scan_all(source, config);

    assert_successful_scan(source, &outcome);

    assert_eq!(outcome.tokens[0].kind, TokenKind::Identifier);
    assert_eq!(outcome.tokens[0].lexeme, "identifier");
    assert_eq!(outcome.tokens[0].byte_range, "\u{FEFF}".len()..source.len());
    assert_eq!(outcome.tokens[1].kind, TokenKind::Eof);
}

// ---------------------------------------------------------------------------
// Regression notes
// ---------------------------------------------------------------------------
//
// Keep future stress coverage additive and deterministic:
//
// - nested-comment stress belongs here when nested-comment behavior is
//   implemented and specified;
// - quantum notation stress should use the canonical quantum lexical
//   contract and must not enumerate a fixed catalog of quantum operations;
// - HDL and other dialect stress should configure registries through their
//   public interfaces rather than hard-code vendor-specific behavior;
// - fuzz/property testing should complement, not replace, these regressions;
// - benchmarks should measure throughput and memory separately from tests;
// - resource exhaustion must remain distinct from malformed syntax;
// - no stress fixture establishes a language-level maximum.