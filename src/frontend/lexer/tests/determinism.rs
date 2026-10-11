#![forbid(unsafe_code)]

//! Determinism regression tests for the canonical Zamani frontend lexer.
//!
//! File: src/frontend/lexer/tests/determinism.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verify that equivalent scanner inputs and configurations produce
//! repeatable token streams, token spans, channels, and scanner errors.
//!
//! # Integration contract
//!
//! This module tests the public-facing contracts of:
//!
//! - `scanner.rs`: streaming source-to-token behavior;
//! - `api.rs`: `TokenSource` retrieval;
//! - `config.rs`: explicit lexical policies and budgets;
//! - `cursor.rs`: stable source positions;
//! - `span.rs`: half-open UTF-8 byte ranges;
//! - `token.rs`: token identity, spelling, and channel;
//! - `token_kind.rs`: canonical token kinds;
//! - `keyword_registry.rs`: keyword classification configuration;
//! - `operator_registry.rs`: operator classification configuration.
//!
//! It deliberately does not reimplement lexical recognition, normalize
//! source text, infer token meanings, or impose hardware-related limits.
//!
//! Register with `mod determinism;` in `tests/mod.rs`.
//!
//! # Determinism contract
//!
//! For the same source bytes, lexer configuration, keyword registry, and
//! operator registry, scanning must produce the same observable result:
//!
//! - token kinds;
//! - exact lexeme spelling;
//! - token channels;
//! - source byte ranges;
//! - explicit EOF behavior;
//! - error categories, ranges, and messages.
//!
//! Determinism is tested independently of eventual classical, quantum,
//! HDL, distributed, accelerator, or future compilation targets.
//!
//! # Resource contract
//!
//! Tests use finite input fixtures. Fixture lengths are not language limits.
//! Production budgets remain explicit configuration, separate from language
//! semantics. No artificial hardware capacities are defined here.

use crate::frontend::lexer::api::TokenSource;
use crate::frontend::lexer::config::{
    LexerConfig,
    LexerLimits,
    ResourceLimit,
    TriviaPolicy,
};
use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::keyword_registry::KeywordRegistry;
use crate::frontend::lexer::operator_registry::OperatorRegistry;
use crate::frontend::lexer::scanner::{
    Scanner,
    ScannerError,
    ScannerErrorKind,
};
use crate::frontend::lexer::token::TokenChannel;
use crate::frontend::lexer::token_kind::TokenKind;

use std::ops::Range;

/// Stable observable representation used by these tests.
///
/// This deliberately excludes implementation-specific allocation details
/// and scanner internals. Borrowed and owned token storage that represents
/// the same source token must be observationally equivalent.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenSnapshot {
    kind: TokenKind,
    lexeme: String,
    byte_range: Range<usize>,
    start: CursorPosition,
    end: CursorPosition,
    channel: TokenChannel,
}

impl TokenSnapshot {
    fn from_token(token: &crate::frontend::lexer::token::Token<'_>) -> Self {
        let span = token.span();

        Self {
            kind: token.kind(),
            lexeme: token.lexeme().to_owned(),
            byte_range: span.byte_range(),
            start: span.start(),
            end: span.end(),
            channel: token.channel(),
        }
    }
}

/// The scanner's observable result, including the location and category of
/// any failure encountered while requesting the next token.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ScanOutcome {
    Complete(Vec<TokenSnapshot>),
    Failed {
        tokens: Vec<TokenSnapshot>,
        kind: ScannerErrorKind,
        byte_range: Range<usize>,
        message: String,
    },
}

/// Runs the canonical scanner with explicit, empty registries.
///
/// Empty registries are intentional here: these tests check deterministic
/// mechanics without relying on an unfinished global keyword or operator
/// catalog. Unknown characters are handled by the configured recovery
/// policy, so the same unregistered spelling should behave consistently.
fn scan_with(
    source: &str,
    config: LexerConfig,
    keywords: KeywordRegistry,
    operators: OperatorRegistry,
) -> ScanOutcome {
    let mut scanner = match Scanner::new(source, config, keywords, operators) {
        Ok(scanner) => scanner,
        Err(error) => {
            return ScanOutcome::Failed {
                tokens: Vec::new(),
                kind: error.kind(),
                byte_range: error.byte_range(),
                message: error.message().to_owned(),
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
                    // Explicit EOF is terminal. Ask for one more token to
                    // verify the scanner's stream termination contract.
                    return match scanner.next_token() {
                        Ok(None) => ScanOutcome::Complete(tokens),
                        Ok(Some(extra)) => {
                            tokens.push(TokenSnapshot::from_token(&extra));
                            ScanOutcome::Failed {
                                tokens,
                                kind: ScannerErrorKind::InternalInvariant,
                                byte_range: extra.span().byte_range(),
                                message: String::from(
                                    "scanner emitted a token after explicit EOF",
                                ),
                            }
                        }
                        Err(error) => failed_outcome(tokens, error),
                    };
                }
            }
            Ok(None) => return ScanOutcome::Complete(tokens),
            Err(error) => return failed_outcome(tokens, error),
        }
    }
}

fn failed_outcome(
    tokens: Vec<TokenSnapshot>,
    error: ScannerError,
) -> ScanOutcome {
    ScanOutcome::Failed {
        tokens,
        kind: error.kind(),
        byte_range: error.byte_range(),
        message: error.message().to_owned(),
    }
}

fn scan_default(source: &str) -> ScanOutcome {
    scan_with(
        source,
        LexerConfig::default(),
        KeywordRegistry::new(),
        OperatorRegistry::new(),
    )
}

fn assert_repeatable(source: &str) {
    let first = scan_default(source);

    for _ in 0..3 {
        assert_eq!(
            scan_default(source),
            first,
            "scanning identical source must be repeatable: {source:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Basic repeatability
// ---------------------------------------------------------------------------

#[test]
fn empty_source_has_a_repeatable_terminal_result() {
    assert_repeatable("");
}

#[test]
fn identifiers_and_delimiters_are_repeatable() {
    assert_repeatable("alpha (beta, gamma);");
}

#[test]
fn numeric_literals_are_repeatable() {
    assert_repeatable("0 17 123456789 3.125");
}

#[test]
fn string_and_character_literals_are_repeatable() {
    assert_repeatable(r#""text" 'x' "escaped\ntext""#);
}

#[test]
fn whitespace_and_comments_are_repeatable() {
    assert_repeatable(
        "alpha  \t beta\n// comment\n/* block comment */ gamma",
    );
}

#[test]
fn unicode_source_is_repeatable() {
    assert_repeatable("λ界 alpha_2");
}

#[test]
fn line_ending_variants_are_repeatable() {
    for source in [
        "first\nsecond",
        "first\r\nsecond",
        "first\rsecond",
        "first\u{2028}second",
        "first\u{2029}second",
    ] {
        assert_repeatable(source);
    }
}

// ---------------------------------------------------------------------------
// Token stream invariants
// ---------------------------------------------------------------------------

#[test]
fn every_emitted_span_is_within_the_source() {
    let source = "alpha λ界\nbeta 123 \"text\"";

    let outcome = scan_default(source);
    let tokens = match outcome {
        ScanOutcome::Complete(tokens) => tokens,
        ScanOutcome::Failed { .. } => {
            panic!("valid representative input should scan successfully")
        }
    };

    for token in tokens {
        let start = token.byte_range.start;
        let end = token.byte_range.end;

        assert!(start <= end, "span must be half-open and ordered");
        assert!(
            end <= source.len(),
            "token span must not exceed the source byte length"
        );

        assert!(
            source.is_char_boundary(start),
            "token start must be a UTF-8 boundary"
        );
        assert!(
            source.is_char_boundary(end),
            "token end must be a UTF-8 boundary"
        );

        assert_eq!(
            &source[start..end],
            token.lexeme,
            "token lexeme must equal the exact source slice"
        );

        assert_eq!(
            end - start,
            token.lexeme.len(),
            "token span length must equal lexeme UTF-8 byte length"
        );
    }
}

#[test]
fn consecutive_non_synthetic_tokens_have_monotonic_source_ranges() {
    let source = "alpha beta gamma";
    let outcome = scan_default(source);

    let tokens = match outcome {
        ScanOutcome::Complete(tokens) => tokens,
        ScanOutcome::Failed { .. } => {
            panic!("representative identifiers should scan successfully")
        }
    };

    let mut previous_start = 0;
    let mut previous_end = 0;

    for token in tokens {
        assert!(token.byte_range.start >= previous_start);
        assert!(token.byte_range.end >= previous_end);

        previous_start = token.byte_range.start;
        previous_end = token.byte_range.end;
    }
}

#[test]
fn explicit_eof_is_emitted_at_most_once() {
    let outcome = scan_default("alpha");

    let tokens = match outcome {
        ScanOutcome::Complete(tokens) => tokens,
        ScanOutcome::Failed { .. } => {
            panic!("representative input should scan successfully")
        }
    };

    let eof_positions: Vec<_> = tokens
        .iter()
        .enumerate()
        .filter(|(_, token)| token.kind == TokenKind::Eof)
        .map(|(index, _)| index)
        .collect();

    assert_eq!(eof_positions.len(), 1);
    assert_eq!(eof_positions[0], tokens.len() - 1);
}

#[test]
fn eof_has_a_zero_width_span_at_the_source_end() {
    let source = "alpha λ";
    let outcome = scan_default(source);

    let tokens = match outcome {
        ScanOutcome::Complete(tokens) => tokens,
        ScanOutcome::Failed { .. } => {
            panic!("representative input should scan successfully")
        }
    };

    let eof = tokens
        .last()
        .expect("the configured default emits EOF");

    assert_eq!(eof.kind, TokenKind::Eof);
    assert!(eof.lexeme.is_empty());
    assert_eq!(eof.byte_range, source.len()..source.len());
    assert_eq!(eof.start.byte_offset(), source.len());
    assert_eq!(eof.end.byte_offset(), source.len());
}

#[test]
fn skipped_trivia_does_not_change_the_spelling_of_remaining_tokens() {
    let plain = scan_default("alpha beta");
    let with_comments = scan_default("alpha /* comment */ beta");

    let plain_tokens = match plain {
        ScanOutcome::Complete(tokens) => tokens,
        other => panic!("unexpected scan result: {other:?}"),
    };

    let comment_tokens = match with_comments {
        ScanOutcome::Complete(tokens) => tokens,
        other => panic!("unexpected scan result: {other:?}"),
    };

    let plain_spellings: Vec<_> = plain_tokens
        .iter()
        .map(|token| (token.kind, token.lexeme.as_str()))
        .collect();

    let comment_spellings: Vec<_> = comment_tokens
        .iter()
        .map(|token| (token.kind, token.lexeme.as_str()))
        .collect();

    assert_eq!(plain_spellings, comment_spellings);
}

#[test]
fn preserved_trivia_is_repeatable() {
    let config = LexerConfig::default()
        .with_trivia_policy(TriviaPolicy::Preserve);

    let source = "alpha // note\nbeta /* detail */";

    let first = scan_with(
        source,
        config,
        KeywordRegistry::new(),
        OperatorRegistry::new(),
    );

    for _ in 0..3 {
        assert_eq!(
            scan_with(
                source,
                config,
                KeywordRegistry::new(),
                OperatorRegistry::new(),
            ),
            first
        );
    }
}

// ---------------------------------------------------------------------------
// Domain neutrality
// ---------------------------------------------------------------------------

#[test]
fn classical_quantum_hdl_and_future_names_use_the_same_lexical_rules() {
    let source = concat!(
        "classical_value ",
        "quantum_operation ",
        "hdl_signal ",
        "tensor_transform ",
        "distributed_worker ",
        "future_accelerator"
    );

    let outcome = scan_default(source);

    let tokens = match outcome {
        ScanOutcome::Complete(tokens) => tokens,
        ScanOutcome::Failed { .. } => {
            panic!("representative identifier input should scan successfully")
        }
    };

    let names: Vec<_> = tokens
        .iter()
        .filter(|token| token.kind == TokenKind::Identifier)
        .map(|token| token.lexeme.as_str())
        .collect();

    assert_eq!(
        names,
        [
            "classical_value",
            "quantum_operation",
            "hdl_signal",
            "tensor_transform",
            "distributed_worker",
            "future_accelerator",
        ]
    );
}

#[test]
fn changing_the_domain_name_does_not_change_token_kind() {
    let names = [
        "classical_kernel",
        "quantum_kernel",
        "hdl_kernel",
        "tensor_kernel",
        "distributed_kernel",
        "custom_future_kernel",
    ];

    for name in names {
        let outcome = scan_default(name);

        let tokens = match outcome {
            ScanOutcome::Complete(tokens) => tokens,
            ScanOutcome::Failed { .. } => {
                panic!("identifier {name:?} should scan successfully")
            }
        };

        assert_eq!(tokens.len(), 2, "identifier plus EOF expected");
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].lexeme, name);
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }
}

// ---------------------------------------------------------------------------
// Repeatable failures
// ---------------------------------------------------------------------------

#[test]
fn disallowed_initial_bom_failure_is_repeatable() {
    let config = LexerConfig::default().with_initial_bom(false);
    let source = "\u{FEFF}alpha";

    let first = scan_with(
        source,
        config,
        KeywordRegistry::new(),
        OperatorRegistry::new(),
    );

    assert!(matches!(
        first,
        ScanOutcome::Failed {
            kind: ScannerErrorKind::InitialBomNotAllowed,
            ..
        }
    ));

    for _ in 0..3 {
        assert_eq!(
            scan_with(
                source,
                config,
                KeywordRegistry::new(),
                OperatorRegistry::new(),
            ),
            first
        );
    }
}

#[test]
fn source_budget_failure_is_repeatable() {
    let mut limits = LexerLimits::unlimited();
    limits.source_bytes =
        ResourceLimit::at_most(3).expect("positive source budget");

    let config = LexerConfig::default().with_limits(limits);
    let source = "alpha";

    let first = scan_with(
        source,
        config,
        KeywordRegistry::new(),
        OperatorRegistry::new(),
    );

    assert!(matches!(
        first,
        ScanOutcome::Failed {
            kind: ScannerErrorKind::SourceBudgetExceeded,
            ..
        }
    ));

    for _ in 0..3 {
        assert_eq!(
            scan_with(
                source,
                config,
                KeywordRegistry::new(),
                OperatorRegistry::new(),
            ),
            first
        );
    }
}

#[test]
fn unterminated_block_comment_failure_is_repeatable() {
    let source = "alpha /* missing terminator";

    let first = scan_default(source);

    assert!(matches!(
        first,
        ScanOutcome::Failed {
            kind: ScannerErrorKind::UnterminatedBlockComment,
            ..
        }
    ));

    for _ in 0..3 {
        assert_eq!(scan_default(source), first);
    }
}

// ---------------------------------------------------------------------------
// Registry and configuration independence
// ---------------------------------------------------------------------------

#[test]
fn equivalent_empty_registries_produce_equivalent_results() {
    let source = "alpha beta 123";

    let first = scan_with(
        source,
        LexerConfig::default(),
        KeywordRegistry::new(),
        OperatorRegistry::new(),
    );

    let second = scan_with(
        source,
        LexerConfig::default(),
        KeywordRegistry::new(),
        OperatorRegistry::new(),
    );

    assert_eq!(first, second);
}

#[test]
fn repeating_a_scan_does_not_mutate_the_callers_configuration() {
    let config = LexerConfig::default();
    let before = config;

    let _ = scan_with(
        "alpha beta",
        config,
        KeywordRegistry::new(),
        OperatorRegistry::new(),
    );

    assert_eq!(config, before);
}

#[test]
fn scanner_error_ranges_are_ordered_and_within_source_bounds() {
    let source = "\u{FEFF}alpha";
    let config = LexerConfig::default().with_initial_bom(false);

    let outcome = scan_with(
        source,
        config,
        KeywordRegistry::new(),
        OperatorRegistry::new(),
    );

    let (start, end) = match outcome {
        ScanOutcome::Failed {
            byte_range,
            kind: ScannerErrorKind::InitialBomNotAllowed,
            ..
        } => (byte_range.start, byte_range.end),
        other => panic!("expected BOM error, got {other:?}"),
    };

    assert!(start <= end);
    assert!(end <= source.len());
    assert!(source.is_char_boundary(start));
    assert!(source.is_char_boundary(end));
}