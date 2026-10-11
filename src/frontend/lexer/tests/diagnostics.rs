#![forbid(unsafe_code)]

//! Production conformance tests for Zamani lexer diagnostics.
//!
//! File: src/frontend/lexer/tests/diagnostics.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Normative references
//!
//! - grammar/spec/diagnostics.md
//! - grammar/lexer/diagnostics.md
//! - grammar/specification/lexical.md
//! - grammar/spec/source-spans.md, if present
//! - grammar/grammar.md, if present
//!
//! # Implementation under test
//!
//! - src/frontend/lexer/diagnostics.rs
//! - src/frontend/lexer/span.rs
//! - src/frontend/lexer/cursor.rs
//! - src/frontend/lexer/config.rs
//! - src/frontend/lexer/recovery.rs
//!
//! # Registration
//!
//! Register this file exactly once in src/frontend/lexer/tests/mod.rs:
//!
//!     mod diagnostics;
//!
//! # Contract
//!
//! These tests verify:
//!
//! - stable machine-readable diagnostic codes;
//! - stable severity classifications;
//! - human-readable messages independent of machine-readable identity;
//! - primary and secondary source labels;
//! - notes and remediation help;
//! - half-open source spans and zero-width EOF spans;
//! - deterministic source-oriented diagnostic ordering;
//! - bounded diagnostic retention;
//! - explicit truncation and dropped-diagnostic accounting;
//! - unlimited retention policy semantics;
//! - error counting and severity distinctions;
//! - clear/reset behavior;
//! - deterministic aggregation of diagnostics;
//! - UTF-8 source coordinates;
//! - resource budgets that are not language-level limits.
//!
//! This module does not implement a second diagnostic system.
//! It does not test target discovery, parser semantics, quantum semantics,
//! hardware allocation, or runtime execution.
//!
//! # Scalability
//!
//! Resource budgets are configurable implementation policies.
//! Unlimited retention does not imply unlimited physical memory.
//! Tests use finite workloads so they remain suitable for ordinary CI.
//!
//! # Safety
//!
//! All tests use safe Rust.

use super::super::config::{LexerLimits, ResourceLimit};
use super::super::cursor::CursorPosition;
use super::super::diagnostics::{
    Diagnostic,
    DiagnosticBag,
    DiagnosticCode,
    DiagnosticSeverity,
    LabelKind,
    PushOutcome,
};
use super::super::span::{Span, SpanError};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn position(byte_offset: usize, line: usize, column: usize) -> CursorPosition {
    CursorPosition::new(byte_offset, line, column)
}

fn span(start: usize, end: usize) -> Span {
    Span::new(
        position(start, 1, start.saturating_add(1)),
        position(end, 1, end.saturating_add(1)),
    )
}

fn diagnostic(
    code: DiagnosticCode,
    severity: DiagnosticSeverity,
    message: &str,
    start: usize,
    end: usize,
) -> Diagnostic {
    Diagnostic::new(code, severity, message, span(start, end))
}

fn lexical_error(
    code: DiagnosticCode,
    message: &str,
    start: usize,
    end: usize,
) -> Diagnostic {
    Diagnostic::error(code, message, span(start, end))
}

// ---------------------------------------------------------------------------
// Stable diagnostic identity
// ---------------------------------------------------------------------------

#[test]
fn diagnostic_codes_have_stable_public_identifiers() {
    let cases = [
        (DiagnosticCode::InvalidCharacter, "LEX001"),
        (DiagnosticCode::InvalidIdentifier, "LEX002"),
        (DiagnosticCode::InvalidNumber, "LEX003"),
        (DiagnosticCode::InvalidString, "LEX004"),
        (DiagnosticCode::InvalidCharacterLiteral, "LEX005"),
        (DiagnosticCode::UnterminatedConstruct, "LEX006"),
        (DiagnosticCode::InvalidEscape, "LEX007"),
        (DiagnosticCode::InvalidOperator, "LEX008"),
        (DiagnosticCode::InvalidAnnotation, "LEX009"),
        (DiagnosticCode::InvalidUnicode, "LEX010"),
        (DiagnosticCode::InvalidByteOrderMark, "LEX011"),
        (DiagnosticCode::SourceLimitExceeded, "LEX012"),
        (DiagnosticCode::TokenLimitExceeded, "LEX013"),
        (DiagnosticCode::TokenSizeLimitExceeded, "LEX014"),
        (DiagnosticCode::LiteralLimitExceeded, "LEX015"),
        (DiagnosticCode::NumericLimitExceeded, "LEX016"),
        (DiagnosticCode::NestingLimitExceeded, "LEX017"),
        (DiagnosticCode::DiagnosticLimitExceeded, "LEX018"),
        (DiagnosticCode::InternalInvariant, "LEX019"),
        (DiagnosticCode::CounterOverflow, "LEX020"),
        (DiagnosticCode::InvalidFrontendState, "LEX021"),
        (DiagnosticCode::CompatibilityFailure, "LEX022"),
        (DiagnosticCode::Other, "LEX099"),
    ];

    for (code, expected) in cases {
        assert_eq!(code.as_str(), expected);
        assert!(!code.name().is_empty());
    }
}

#[test]
fn diagnostic_codes_are_unique() {
    let codes = [
        DiagnosticCode::InvalidCharacter,
        DiagnosticCode::InvalidIdentifier,
        DiagnosticCode::InvalidNumber,
        DiagnosticCode::InvalidString,
        DiagnosticCode::InvalidCharacterLiteral,
        DiagnosticCode::UnterminatedConstruct,
        DiagnosticCode::InvalidEscape,
        DiagnosticCode::InvalidOperator,
        DiagnosticCode::InvalidAnnotation,
        DiagnosticCode::InvalidUnicode,
        DiagnosticCode::InvalidByteOrderMark,
        DiagnosticCode::SourceLimitExceeded,
        DiagnosticCode::TokenLimitExceeded,
        DiagnosticCode::TokenSizeLimitExceeded,
        DiagnosticCode::LiteralLimitExceeded,
        DiagnosticCode::NumericLimitExceeded,
        DiagnosticCode::NestingLimitExceeded,
        DiagnosticCode::DiagnosticLimitExceeded,
        DiagnosticCode::InternalInvariant,
        DiagnosticCode::CounterOverflow,
        DiagnosticCode::InvalidFrontendState,
        DiagnosticCode::CompatibilityFailure,
        DiagnosticCode::Other,
    ];

    for (index, code) in codes.iter().enumerate() {
        assert!(
            !codes[..index]
                .iter()
                .any(|earlier| earlier.as_str() == code.as_str()),
            "duplicate diagnostic identifier: {}",
            code.as_str()
        );
    }
}

#[test]
fn diagnostic_identity_does_not_depend_on_message_wording() {
    let first = lexical_error(
        DiagnosticCode::InvalidCharacter,
        "unexpected character",
        0,
        1,
    );

    let second = lexical_error(
        DiagnosticCode::InvalidCharacter,
        "character is not valid here",
        0,
        1,
    );

    assert_eq!(first.code(), second.code());
    assert_eq!(first.code_str(), "LEX001");
    assert_ne!(first.message(), second.message());
}

// ---------------------------------------------------------------------------
// Severity contracts
// ---------------------------------------------------------------------------

#[test]
fn severity_classifies_errors_consistently() {
    assert!(!DiagnosticSeverity::Note.is_error());
    assert!(!DiagnosticSeverity::Warning.is_error());
    assert!(DiagnosticSeverity::Error.is_error());
    assert!(DiagnosticSeverity::Fatal.is_error());

    assert!(!DiagnosticSeverity::Note.prevents_success());
    assert!(!DiagnosticSeverity::Warning.prevents_success());
    assert!(DiagnosticSeverity::Error.prevents_success());
    assert!(DiagnosticSeverity::Fatal.prevents_success());
}

#[test]
fn diagnostic_constructors_assign_the_expected_severity() {
    let source_span = span(0, 1);

    let note = Diagnostic::note(
        DiagnosticCode::Other,
        "additional information",
        source_span,
    );

    let warning = Diagnostic::warning(
        DiagnosticCode::InvalidCharacter,
        "suspicious source character",
        source_span,
    );

    let error = Diagnostic::error(
        DiagnosticCode::InvalidCharacter,
        "invalid source character",
        source_span,
    );

    let fatal = Diagnostic::fatal(
        DiagnosticCode::InternalInvariant,
        "internal invariant failed",
        source_span,
    );

    assert_eq!(note.severity(), DiagnosticSeverity::Note);
    assert_eq!(warning.severity(), DiagnosticSeverity::Warning);
    assert_eq!(error.severity(), DiagnosticSeverity::Error);
    assert_eq!(fatal.severity(), DiagnosticSeverity::Fatal);

    assert!(!note.is_error());
    assert!(!warning.is_error());
    assert!(error.is_error());
    assert!(fatal.is_error());
}

#[test]
fn display_contains_machine_code_and_human_message() {
    let item = lexical_error(
        DiagnosticCode::InvalidEscape,
        "invalid escape sequence",
        3,
        5,
    );

    let rendered = item.to_string();

    assert!(rendered.contains("LEX007"));
    assert!(rendered.contains("invalid escape sequence"));
    assert!(rendered.contains("error"));
}

// ---------------------------------------------------------------------------
// Source spans
// ---------------------------------------------------------------------------

#[test]
fn diagnostic_preserves_its_primary_source_span() {
    let source_span = Span::new(
        position(4, 2, 3),
        position(9, 2, 8),
    );

    let item = Diagnostic::error(
        DiagnosticCode::InvalidIdentifier,
        "invalid identifier",
        source_span,
    );

    assert_eq!(item.primary_span(), source_span);
    assert_eq!(item.primary_span().start_byte(), 4);
    assert_eq!(item.primary_span().end_byte(), 9);
    assert_eq!(item.primary_span().byte_len(), 5);
    assert!(!item.primary_span().is_empty());
}

#[test]
fn zero_width_eof_diagnostics_are_supported() {
    let eof = Span::at(position(12, 3, 1));

    let item = Diagnostic::error(
        DiagnosticCode::UnterminatedConstruct,
        "expected a closing delimiter before EOF",
        eof,
    );

    assert!(item.primary_span().is_empty());
    assert_eq!(item.primary_span().start_byte(), 12);
    assert_eq!(item.primary_span().end_byte(), 12);
    assert_eq!(item.primary_span().byte_len(), 0);
}

#[test]
fn span_construction_rejects_reversed_byte_ranges() {
    let result = Span::try_new(
        position(9, 1, 10),
        position(4, 1, 5),
    );

    assert!(matches!(
        result,
        Err(SpanError::ReversedByteRange {
            start: 9,
            end: 4
        })
    ));
}

#[test]
fn span_construction_rejects_reversed_positions() {
    let result = Span::try_new(
        position(4, 2, 1),
        position(9, 1, 10),
    );

    assert!(matches!(
        result,
        Err(SpanError::ReversedPositionRange)
    ));
}

#[test]
fn adjacent_spans_remain_distinct_half_open_ranges() {
    let first = span(0, 3);
    let second = span(3, 7);

    assert_eq!(first.byte_range(), 0..3);
    assert_eq!(second.byte_range(), 3..7);

    assert_eq!(first.end_byte(), second.start_byte());
    assert_eq!(first.byte_len(), 3);
    assert_eq!(second.byte_len(), 4);
}

// ---------------------------------------------------------------------------
// Primary and secondary labels
// ---------------------------------------------------------------------------

#[test]
fn secondary_labels_preserve_their_spans_and_messages() {
    let primary = span(4, 7);
    let secondary = span(0, 3);

    let item = Diagnostic::error(
        DiagnosticCode::InvalidIdentifier,
        "conflicting identifier",
        primary,
    )
    .with_secondary_label(secondary, "previous occurrence");

    assert_eq!(item.primary_span(), primary);
    assert_eq!(item.labels().len(), 1);

    let label = &item.labels()[0];

    assert_eq!(label.kind(), LabelKind::Secondary);
    assert_eq!(label.span(), secondary);
    assert_eq!(label.message(), "previous occurrence");
}

#[test]
fn primary_labels_do_not_replace_the_canonical_primary_span() {
    let canonical = span(2, 5);
    let label_span = span(2, 5);

    let item = Diagnostic::error(
        DiagnosticCode::InvalidCharacter,
        "invalid character",
        canonical,
    )
    .with_primary_label(label_span, "offending character");

    assert_eq!(item.primary_span(), canonical);
    assert_eq!(item.labels().len(), 1);
    assert_eq!(item.labels()[0].kind(), LabelKind::Primary);
    assert_eq!(item.labels()[0].span(), label_span);
}

#[test]
fn diagnostics_support_multiple_secondary_labels() {
    let item = Diagnostic::error(
        DiagnosticCode::InvalidIdentifier,
        "identifier conflicts with earlier declarations",
        span(20, 24),
    )
    .with_secondary_label(span(0, 4), "first declaration")
    .with_secondary_label(span(8, 12), "second declaration")
    .with_secondary_label(span(14, 18), "third declaration");

    assert_eq!(item.labels().len(), 3);

    assert_eq!(item.labels()[0].span(), span(0, 4));
    assert_eq!(item.labels()[1].span(), span(8, 12));
    assert_eq!(item.labels()[2].span(), span(14, 18));

    assert!(item
        .labels()
        .iter()
        .all(|label| label.kind() == LabelKind::Secondary));
}

// ---------------------------------------------------------------------------
// Notes, help, and human-readable context
// ---------------------------------------------------------------------------

#[test]
fn diagnostic_notes_preserve_insertion_order() {
    let item = Diagnostic::error(
        DiagnosticCode::InvalidEscape,
        "invalid escape sequence",
        span(3, 5),
    )
    .with_note("escape sequences begin with a backslash")
    .with_note("the following character is not supported")
    .with_note("consult the lexical specification");

    assert_eq!(item.notes().len(), 3);

    assert_eq!(
        item.notes()[0].message(),
        "escape sequences begin with a backslash"
    );

    assert_eq!(
        item.notes()[1].message(),
        "the following character is not supported"
    );

    assert_eq!(
        item.notes()[2].message(),
        "consult the lexical specification"
    );
}

#[test]
fn diagnostic_help_is_optional_and_preserved() {
    let without_help = Diagnostic::error(
        DiagnosticCode::InvalidOperator,
        "invalid operator",
        span(1, 2),
    );

    assert_eq!(without_help.help(), None);

    let with_help = without_help.with_help(
        "check the operator spelling in the lexical specification",
    );

    assert_eq!(
        with_help.help(),
        Some("check the operator spelling in the lexical specification")
    );
}

#[test]
fn diagnostic_metadata_supports_unicode_messages() {
    let item = Diagnostic::error(
        DiagnosticCode::InvalidUnicode,
        "invalid Unicode character: λ界",
        span(0, 4),
    )
    .with_secondary_label(span(5, 8), "関連する文字")
    .with_note("source spelling is preserved")
    .with_help("review the identifier character categories");

    assert!(item.message().contains("λ界"));
    assert_eq!(item.labels()[0].message(), "関連する文字");
    assert_eq!(item.notes()[0].message(), "source spelling is preserved");
    assert!(item.help().unwrap().contains("identifier"));
}

// ---------------------------------------------------------------------------
// Deterministic ordering
// ---------------------------------------------------------------------------

#[test]
fn deterministic_ordering_uses_source_offsets_first() {
    let later = lexical_error(
        DiagnosticCode::InvalidCharacter,
        "later",
        20,
        21,
    );

    let earlier = lexical_error(
        DiagnosticCode::InvalidString,
        "earlier",
        2,
        3,
    );

    assert!(
        earlier.deterministic_cmp(&later).is_lt()
    );

    assert!(
        later.deterministic_cmp(&earlier).is_gt()
    );
}

#[test]
fn deterministic_ordering_breaks_ties_consistently() {
    let first = lexical_error(
        DiagnosticCode::InvalidCharacter,
        "same location",
        4,
        6,
    );

    let second = lexical_error(
        DiagnosticCode::InvalidString,
        "same location",
        4,
        6,
    );

    assert_ne!(
        first.deterministic_cmp(&second),
        core::cmp::Ordering::Equal
    );

    assert_eq!(
        first.deterministic_cmp(&second),
        second.deterministic_cmp(&first).reverse()
    );
}

#[test]
fn diagnostic_bag_sorts_deterministically() {
    let mut bag = DiagnosticBag::unlimited();

    bag.push(lexical_error(
        DiagnosticCode::InvalidCharacter,
        "third",
        8,
        9,
    ));

    bag.push(lexical_error(
        DiagnosticCode::InvalidNumber,
        "first",
        0,
        1,
    ));

    bag.push(lexical_error(
        DiagnosticCode::InvalidString,
        "second",
        3,
        4,
    ));

    assert_eq!(bag.diagnostics()[0].message(), "third");

    bag.sort_deterministically();

    assert_eq!(bag.diagnostics()[0].message(), "first");
    assert_eq!(bag.diagnostics()[1].message(), "second");
    assert_eq!(bag.diagnostics()[2].message(), "third");
}

#[test]
fn repeated_diagnostic_construction_produces_equal_values() {
    let first = Diagnostic::error(
        DiagnosticCode::InvalidNumber,
        "malformed number",
        span(10, 14),
    )
    .with_secondary_label(span(0, 2), "related declaration")
    .with_note("numeric literal syntax is invalid")
    .with_help("correct the literal");

    let second = Diagnostic::error(
        DiagnosticCode::InvalidNumber,
        "malformed number",
        span(10, 14),
    )
    .with_secondary_label(span(0, 2), "related declaration")
    .with_note("numeric literal syntax is invalid")
    .with_help("correct the literal");

    assert_eq!(first, second);
    assert_eq!(
        first.deterministic_cmp(&second),
        core::cmp::Ordering::Equal
    );
}

// ---------------------------------------------------------------------------
// Diagnostic collection and error accounting
// ---------------------------------------------------------------------------

#[test]
fn unlimited_bag_retains_all_finite_test_diagnostics() {
    let mut bag = DiagnosticBag::unlimited();

    for index in 0..128 {
        bag.push(lexical_error(
            DiagnosticCode::InvalidCharacter,
            "invalid character",
            index,
            index.saturating_add(1),
        ));
    }

    assert_eq!(bag.len(), 128);
    assert!(!bag.is_empty());
    assert_eq!(bag.dropped_count(), 0);
    assert!(!bag.is_truncated());
}

#[test]
fn bounded_bag_retains_up_to_its_configured_limit() {
    let limit = ResourceLimit::at_most(2).expect("nonzero limit");
    let mut bag = DiagnosticBag::new(limit);

    assert_eq!(
        bag.push(lexical_error(
            DiagnosticCode::InvalidCharacter,
            "first",
            0,
            1,
        )),
        PushOutcome::Added
    );

    assert_eq!(
        bag.push(lexical_error(
            DiagnosticCode::InvalidNumber,
            "second",
            2,
            3,
        )),
        PushOutcome::Added
    );

    assert_eq!(
        bag.push(lexical_error(
            DiagnosticCode::InvalidString,
            "third",
            4,
            5,
        )),
        PushOutcome::LimitReached
    );

    assert_eq!(bag.len(), 2);
    assert_eq!(bag.dropped_count(), 1);
    assert!(bag.is_truncated());
}

#[test]
fn bounded_bag_does_not_evict_earlier_diagnostics() {
    let limit = ResourceLimit::at_most(1).expect("nonzero limit");
    let mut bag = DiagnosticBag::new(limit);

    let first = lexical_error(
        DiagnosticCode::InvalidCharacter,
        "first diagnostic",
        0,
        1,
    );

    assert_eq!(bag.push(first.clone()), PushOutcome::Added);

    assert_eq!(
        bag.push(lexical_error(
            DiagnosticCode::InvalidString,
            "later diagnostic",
            10,
            11,
        )),
        PushOutcome::LimitReached
    );

    assert_eq!(bag.diagnostics(), &[first]);
    assert_eq!(bag.dropped_count(), 1);
}

#[test]
fn dropped_diagnostic_count_tracks_rejected_entries() {
    let limit = ResourceLimit::at_most(1).expect("nonzero limit");
    let mut bag = DiagnosticBag::new(limit);

    for index in 0..5 {
        bag.push(lexical_error(
            DiagnosticCode::InvalidCharacter,
            "diagnostic",
            index,
            index.saturating_add(1),
        ));
    }

    assert_eq!(bag.len(), 1);
    assert_eq!(bag.dropped_count(), 4);
    assert!(bag.is_truncated());
}

#[test]
fn push_if_permitted_reports_retention_success() {
    let mut bag = DiagnosticBag::new(
        ResourceLimit::at_most(1).expect("nonzero limit"),
    );

    assert!(bag.push_if_permitted(lexical_error(
        DiagnosticCode::InvalidCharacter,
        "first",
        0,
        1,
    )));

    assert!(!bag.push_if_permitted(lexical_error(
        DiagnosticCode::InvalidString,
        "second",
        2,
        3,
    )));

    assert_eq!(bag.len(), 1);
    assert_eq!(bag.dropped_count(), 1);
}

#[test]
fn empty_bag_reports_no_retained_errors() {
    let bag = DiagnosticBag::unlimited();

    assert!(bag.is_empty());
    assert_eq!(bag.len(), 0);
    assert_eq!(bag.error_count(), 0);
    assert!(!bag.has_errors());
    assert!(!bag.is_truncated());
}

#[test]
fn error_count_excludes_notes_and_warnings() {
    let mut bag = DiagnosticBag::unlimited();

    bag.push(diagnostic(
        DiagnosticCode::Other,
        DiagnosticSeverity::Note,
        "note",
        0,
        1,
    ));

    bag.push(diagnostic(
        DiagnosticCode::InvalidCharacter,
        DiagnosticSeverity::Warning,
        "warning",
        2,
        3,
    ));

    bag.push(lexical_error(
        DiagnosticCode::InvalidNumber,
        "error",
        4,
        5,
    ));

    bag.push(Diagnostic::fatal(
        DiagnosticCode::InternalInvariant,
        "fatal",
        span(6, 7),
    ));

    assert_eq!(bag.len(), 4);
    assert_eq!(bag.error_count(), 2);
    assert!(bag.has_errors());
}

#[test]
fn clear_resets_entries_and_truncation_but_preserves_limit() {
    let limit = ResourceLimit::at_most(1).expect("nonzero limit");
    let mut bag = DiagnosticBag::new(limit);

    bag.push(lexical_error(
        DiagnosticCode::InvalidCharacter,
        "retained",
        0,
        1,
    ));

    bag.push(lexical_error(
        DiagnosticCode::InvalidString,
        "dropped",
        2,
        3,
    ));

    assert!(bag.is_truncated());

    bag.clear();

    assert!(bag.is_empty());
    assert_eq!(bag.dropped_count(), 0);
    assert!(!bag.is_truncated());
    assert_eq!(bag.limit(), limit);

    assert_eq!(
        bag.push(lexical_error(
            DiagnosticCode::InvalidNumber,
            "new diagnostic",
            4,
            5,
        )),
        PushOutcome::Added
    );
}

// ---------------------------------------------------------------------------
// Aggregation
// ---------------------------------------------------------------------------

#[test]
fn extend_respects_the_receiving_bags_retention_policy() {
    let mut destination = DiagnosticBag::new(
        ResourceLimit::at_most(2).expect("nonzero limit"),
    );

    let incoming = [
        lexical_error(DiagnosticCode::InvalidCharacter, "one", 0, 1),
        lexical_error(DiagnosticCode::InvalidNumber, "two", 2, 3),
        lexical_error(DiagnosticCode::InvalidString, "three", 4, 5),
    ];

    let added = destination.extend(incoming);

    assert_eq!(added, 2);
    assert_eq!(destination.len(), 2);
    assert_eq!(destination.dropped_count(), 1);
    assert!(destination.is_truncated());
}

#[test]
fn aggregation_does_not_silently_merge_source_bag_truncation() {
    let mut source = DiagnosticBag::new(
        ResourceLimit::at_most(1).expect("nonzero limit"),
    );

    source.push(lexical_error(
        DiagnosticCode::InvalidCharacter,
        "retained",
        0,
        1,
    ));

    source.push(lexical_error(
        DiagnosticCode::InvalidString,
        "dropped",
        2,
        3,
    ));

    assert_eq!(source.dropped_count(), 1);

    let mut destination = DiagnosticBag::unlimited();
    let added = destination.extend(source.diagnostics().iter().cloned());

    assert_eq!(added, 1);
    assert_eq!(destination.len(), 1);

    // The receiving bag accounts only for entries rejected by itself.
    // Callers must inspect and propagate the source bag's truncation
    // metadata explicitly when aggregating diagnostic results.
    assert_eq!(destination.dropped_count(), 0);
}

// ---------------------------------------------------------------------------
// Resource policy contracts
// ---------------------------------------------------------------------------

#[test]
fn resource_limits_distinguish_unlimited_and_bounded_policies() {
    let unlimited = ResourceLimit::unlimited();
    let bounded = ResourceLimit::at_most(4).expect("nonzero limit");

    assert!(!unlimited.is_bounded());
    assert!(unlimited.bound().is_none());
    assert!(unlimited.permits(usize::MAX));

    assert!(bounded.is_bounded());
    assert_eq!(bounded.bound().map(|value| value.get()), Some(4));
    assert!(bounded.permits(4));
    assert!(!bounded.permits(5));
    assert!(bounded.is_exceeded_by(5));
}

#[test]
fn zero_is_not_a_valid_positive_resource_budget() {
    assert!(ResourceLimit::at_most(0).is_none());
}

#[test]
fn lexer_limits_can_be_unlimited_without_changing_language_semantics() {
    let limits = LexerLimits::unlimited();

    assert!(!limits.source_bytes.is_bounded());
    assert!(!limits.tokens.is_bounded());
    assert!(!limits.token_bytes.is_bounded());
    assert!(!limits.numeric_digits.is_bounded());
    assert!(!limits.literal_bytes.is_bounded());
    assert!(!limits.lexical_nesting_depth.is_bounded());
    assert!(!limits.diagnostics.is_bounded());
}

#[test]
fn diagnostic_bag_can_be_constructed_from_lexer_limits() {
    let mut limits = LexerLimits::unlimited();
    limits.diagnostics =
        ResourceLimit::at_most(2).expect("nonzero limit");

    let mut bag = DiagnosticBag::from_limits(&limits);

    assert_eq!(bag.limit(), limits.diagnostics);

    bag.push(lexical_error(
        DiagnosticCode::InvalidCharacter,
        "first",
        0,
        1,
    ));

    bag.push(lexical_error(
        DiagnosticCode::InvalidNumber,
        "second",
        2,
        3,
    ));

    assert_eq!(
        bag.push(lexical_error(
            DiagnosticCode::InvalidString,
            "third",
            4,
            5,
        )),
        PushOutcome::LimitReached
    );

    assert_eq!(bag.len(), 2);
    assert_eq!(bag.dropped_count(), 1);
}

// ---------------------------------------------------------------------------
// Cross-domain neutrality
// ---------------------------------------------------------------------------

#[test]
fn diagnostic_codes_are_not_bound_to_a_hardware_domain() {
    // Lexical failures have the same diagnostic identity regardless of
    // whether the source program later targets classical, quantum, HDL,
    // distributed, or future computational domains.
    let samples = [
        "classical source character",
        "quantum source character",
        "HDL source character",
        "distributed source character",
        "future-domain source character",
    ];

    for message in samples {
        let item = lexical_error(
            DiagnosticCode::InvalidCharacter,
            message,
            0,
            1,
        );

        assert_eq!(item.code(), DiagnosticCode::InvalidCharacter);
        assert_eq!(item.code_str(), "LEX001");
        assert_eq!(item.primary_span(), span(0, 1));
    }
}

#[test]
fn diagnostics_do_not_encode_artificial_hardware_capacities() {
    // The lexical diagnostic API reports actual errors and explicitly
    // configured resource budgets. It must not require a universal CPU,
    // qubit, register-width, node, memory, or accelerator ceiling.
    let limits = LexerLimits::unlimited();

    let bag = DiagnosticBag::from_limits(&limits);

    assert!(!bag.limit().is_bounded());
    assert_eq!(bag.dropped_count(), 0);
    assert!(!bag.is_truncated());
}

// ---------------------------------------------------------------------------
// Stable integration expectations
// ---------------------------------------------------------------------------

#[test]
fn diagnostic_primary_span_remains_authoritative_with_extra_context() {
    let primary = span(10, 13);

    let item = Diagnostic::error(
        DiagnosticCode::InvalidOperator,
        "invalid operator",
        primary,
    )
    .with_secondary_label(span(0, 4), "related source")
    .with_note("operator spelling is invalid")
    .with_help("check the operator registry");

    assert_eq!(item.primary_span(), primary);
    assert_eq!(item.labels().len(), 1);
    assert_eq!(item.notes().len(), 1);
    assert!(item.help().is_some());
}

#[test]
fn diagnostics_can_be_cloned_and_aggregated_without_changing_identity() {
    let original = lexical_error(
        DiagnosticCode::UnterminatedConstruct,
        "unterminated string",
        5,
        12,
    )
    .with_note("string begins here")
    .with_help("close the string literal");

    let cloned = original.clone();

    assert_eq!(original, cloned);

    let mut bag = DiagnosticBag::unlimited();
    assert_eq!(bag.push(cloned), PushOutcome::Added);

    assert_eq!(bag.len(), 1);
    assert_eq!(bag.diagnostics()[0], original);
    assert_eq!(
        bag.diagnostics()[0].code_str(),
        original.code_str()
    );
}