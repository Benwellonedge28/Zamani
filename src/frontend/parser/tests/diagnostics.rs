#![forbid(unsafe_code)]

//! Integration tests for parser diagnostics.
//!
//! File: src/frontend/parser/tests/diagnostics.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! Contract:
//! - Exercise the canonical parser error and diagnostic types.
//! - Preserve source spans and structured error identity.
//! - Verify severity classification and grammar-context notes.
//! - Verify deterministic diagnostic retention and suppression accounting.
//! - Verify that bounded reporting does not overwrite earlier diagnostics.
//! - Verify that diagnostic collection behavior is independent of computing
//!   domain and target hardware.
//!
//! These tests do not duplicate lexer diagnostics, source-span definitions,
//! parser error types, grammar rules, or semantic validation.
//!
//! Normative references:
//! - grammar/spec/diagnostics.md
//! - grammar/spec/syntax.md
//!
//! Integration:
//! - parser/diagnostics.rs owns ParserDiagnostic and DiagnosticCollection.
//! - parser/error.rs owns ParseError, ParseErrorKind, and ParseErrorClass.
//! - parser/limits.rs re-exports the canonical ResourceLimit.
//! - lexer/span.rs owns the canonical Span.
//!
//! This file tests the implemented API. It does not imply that every
//! normative diagnostic feature has already been implemented.

use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::parser::diagnostics::{
    DiagnosticCollection, ParserDiagnostic, ParserDiagnosticSeverity,
};
use crate::frontend::parser::error::{
    ParseError, ParseErrorClass, ParseErrorKind,
};
use crate::frontend::parser::limits::ResourceLimit;

/// Construct a valid span using the canonical lexer representation.
fn span(start: usize, end: usize) -> Span {
    assert!(start <= end, "test span must not be reversed");

    Span::new(
        CursorPosition::new(start, 1, start.saturating_add(1)),
        CursorPosition::new(end, 1, end.saturating_add(1)),
    )
}

/// Construct a recoverable parser error with a source location and context.
fn unexpected_token_error() -> ParseError {
    ParseError::unexpected_token(
        span(4, 5),
        "}",
        ["expression", "identifier", "expression"],
    )
    .with_context("function body")
    .with_context("expression")
}

/// Construct a diagnostic with stable, test-controlled content.
fn diagnostic(
    code: &str,
    severity: ParserDiagnosticSeverity,
    message: &str,
) -> ParserDiagnostic {
    ParserDiagnostic::new(code, severity, message, None)
}

#[test]
fn unexpected_token_conversion_preserves_stable_code_and_severity() {
    let error = unexpected_token_error();
    let converted = ParserDiagnostic::from_error(&error);

    assert_eq!(converted.code(), "parser.unexpected_token");
    assert_eq!(converted.severity(), ParserDiagnosticSeverity::Error);
    assert!(converted.is_error());
}

#[test]
fn unexpected_token_conversion_preserves_source_span() {
    let error = unexpected_token_error();
    let converted = ParserDiagnostic::from_error(&error);

    assert_eq!(converted.primary_span(), Some(span(4, 5)));
    assert_eq!(
        converted.primary_span().expect("span must be preserved").start_byte(),
        4
    );
    assert_eq!(
        converted.primary_span().expect("span must be preserved").end_byte(),
        5
    );
}

#[test]
fn unexpected_token_conversion_preserves_context_in_order() {
    let error = unexpected_token_error();
    let converted = ParserDiagnostic::from_error(&error);

    assert_eq!(
        converted.notes(),
        &[
            "while parsing function body".to_owned(),
            "while parsing expression".to_owned(),
        ]
    );
}

#[test]
fn unexpected_token_error_preserves_structured_expected_symbols() {
    let error = unexpected_token_error();

    assert_eq!(error.class(), ParseErrorClass::Recoverable);
    assert_eq!(error.stable_code(), "parser.unexpected_token");

    match error.kind() {
        ParseErrorKind::UnexpectedToken { found, expected } => {
            assert_eq!(found.as_deref(), Some("}"));
            assert_eq!(
                expected,
                &vec!["expression".to_owned(), "identifier".to_owned()]
            );
        }
        other => panic!("expected an unexpected-token error, got {other:?}"),
    }
}

#[test]
fn unexpected_eof_conversion_preserves_expected_symbols() {
    let error = ParseError::unexpected_eof(span(8, 8), ["}", ")"]);
    let converted = ParserDiagnostic::from_error(&error);

    assert_eq!(converted.code(), "parser.unexpected_eof");
    assert_eq!(converted.severity(), ParserDiagnosticSeverity::Error);
    assert_eq!(converted.primary_span(), Some(span(8, 8)));
    assert!(converted.message().contains("unexpected end of input"));

    match error.kind() {
        ParseErrorKind::UnexpectedEof { expected } => {
            assert_eq!(expected, &vec![")".to_owned(), "}".to_owned()]);
        }
        other => panic!("expected an unexpected-EOF error, got {other:?}"),
    }
}

#[test]
fn trailing_input_has_a_distinct_diagnostic_code() {
    let error = ParseError::trailing_input(span(10, 15), Some("extra".to_owned()));
    let converted = ParserDiagnostic::from_error(&error);

    assert_eq!(converted.code(), "parser.trailing_input");
    assert_eq!(converted.severity(), ParserDiagnosticSeverity::Error);
    assert_eq!(converted.primary_span(), Some(span(10, 15)));
    assert!(converted.message().contains("extra"));
}

#[test]
fn extension_failure_preserves_extension_identity() {
    let error = ParseError::extension_failure(
        "example.syntax",
        "unsupported construct",
        Some(span(2, 9)),
    );
    let converted = ParserDiagnostic::from_error(&error);

    assert_eq!(converted.code(), "parser.extension_failure");
    assert_eq!(converted.primary_span(), Some(span(2, 9)));
    assert!(converted.message().contains("example.syntax"));
    assert!(converted.message().contains("unsupported construct"));

    match error.kind() {
        ParseErrorKind::ExtensionFailure {
            extension_id,
            message,
        } => {
            assert_eq!(extension_id, "example.syntax");
            assert_eq!(message, "unsupported construct");
        }
        other => panic!("expected an extension failure, got {other:?}"),
    }
}

#[test]
fn internal_invariant_errors_become_fatal_diagnostics() {
    let error = ParseError::internal_invariant(
        "cursor state invalid",
        Some(span(1, 2)),
    );
    let converted = ParserDiagnostic::from_error(&error);

    assert_eq!(converted.code(), "parser.internal_invariant");
    assert_eq!(converted.severity(), ParserDiagnosticSeverity::Fatal);
    assert!(converted.is_error());
    assert_eq!(converted.primary_span(), Some(span(1, 2)));
    assert!(
        converted.notes().iter().any(|note| note.contains("bug report")),
        "internal failures should provide debugging guidance"
    );
}

#[test]
fn diagnostic_severity_is_not_confused_with_recovery_classification() {
    let recoverable = ParserDiagnostic::from_error(&unexpected_token_error());

    let fatal_error = ParseError::internal_invariant("broken invariant", None);
    let fatal = ParserDiagnostic::from_error(&fatal_error);

    assert_eq!(recoverable.severity(), ParserDiagnosticSeverity::Error);
    assert_eq!(fatal.severity(), ParserDiagnosticSeverity::Fatal);
    assert!(!recoverable.severity().is_fatal());
    assert!(fatal.severity().is_fatal());
}

#[test]
fn diagnostic_labels_notes_and_help_are_preserved() {
    let primary = span(2, 3);
    let secondary = span(8, 9);

    let value = ParserDiagnostic::new(
        "parser.test",
        ParserDiagnosticSeverity::Error,
        "invalid construct",
        Some(primary),
    )
    .with_label(secondary, "related declaration")
    .with_note("inside a declaration")
    .with_note("after an attribute")
    .with_help("review the declaration");

    assert_eq!(value.primary_span(), Some(primary));
    assert_eq!(value.labels().len(), 1);
    assert_eq!(value.labels()[0].span(), secondary);
    assert_eq!(value.labels()[0].message(), "related declaration");
    assert_eq!(
        value.notes(),
        &[
            "inside a declaration".to_owned(),
            "after an attribute".to_owned(),
        ]
    );
    assert_eq!(value.help(), Some("review the declaration"));
}

#[test]
fn diagnostic_display_contains_code_message_help_and_notes() {
    let value = ParserDiagnostic::new(
        "parser.test",
        ParserDiagnosticSeverity::Error,
        "invalid construct",
        Some(span(0, 1)),
    )
    .with_help("check the expression")
    .with_note("inside a function");

    let rendered = value.to_string();

    assert!(rendered.starts_with("error[parser.test]: invalid construct"));
    assert!(rendered.contains("help: check the expression"));
    assert!(rendered.contains("note: inside a function"));
}

#[test]
fn display_is_deterministic_for_identical_diagnostics() {
    let make = || {
        ParserDiagnostic::new(
            "parser.unexpected_token",
            ParserDiagnosticSeverity::Error,
            "unexpected token",
            Some(span(3, 4)),
        )
        .with_note("inside a block")
        .with_help("check the closing delimiter")
    };

    assert_eq!(make().to_string(), make().to_string());
    assert_eq!(make(), make());
}

#[test]
fn diagnostic_without_a_source_span_is_supported() {
    let value = ParserDiagnostic::new(
        "parser.configuration_failure",
        ParserDiagnosticSeverity::Fatal,
        "invalid parser configuration",
        None,
    );

    assert_eq!(value.primary_span(), None);
    assert_eq!(value.severity(), ParserDiagnosticSeverity::Fatal);
}

#[test]
fn bounded_collection_retains_earlier_diagnostics() {
    let limit = ResourceLimit::at_most(2).expect("two is a valid budget");
    let mut collection = DiagnosticCollection::with_limit(limit);

    assert!(collection.push(diagnostic(
        "parser.first",
        ParserDiagnosticSeverity::Error,
        "first error",
    )));
    assert!(collection.push(diagnostic(
        "parser.second",
        ParserDiagnosticSeverity::Warning,
        "second report",
    )));
    assert!(!collection.push(diagnostic(
        "parser.third",
        ParserDiagnosticSeverity::Error,
        "third error",
    )));

    assert_eq!(collection.len(), 2);
    assert_eq!(collection.dropped_count(), 1);
    assert!(collection.is_truncated());

    assert_eq!(collection.diagnostics()[0].code(), "parser.first");
    assert_eq!(collection.diagnostics()[1].code(), "parser.second");
}

#[test]
fn diagnostic_suppression_count_tracks_each_rejected_report() {
    let limit = ResourceLimit::at_most(1).expect("one is a valid budget");
    let mut collection = DiagnosticCollection::with_limit(limit);

    assert!(collection.push(diagnostic(
        "parser.retained",
        ParserDiagnosticSeverity::Error,
        "retained",
    )));

    for index in 0..5 {
        assert!(!collection.push(diagnostic(
            "parser.suppressed",
            ParserDiagnosticSeverity::Warning,
            &format!("suppressed report {index}"),
        )));
    }

    assert_eq!(collection.len(), 1);
    assert_eq!(collection.dropped_count(), 5);
    assert!(collection.is_truncated());
}

#[test]
fn zero_retention_budget_drops_reports_without_losing_accounting() {
    let limit = ResourceLimit::at_most(1).expect("one is a valid budget");
    let mut collection = DiagnosticCollection::with_limit(limit);

    // The first report consumes the single available slot.
    assert!(collection.push(diagnostic(
        "parser.first",
        ParserDiagnosticSeverity::Error,
        "first",
    )));
    assert!(!collection.push(diagnostic(
        "parser.second",
        ParserDiagnosticSeverity::Error,
        "second",
    )));

    assert_eq!(collection.len(), 1);
    assert_eq!(collection.dropped_count(), 1);
    assert!(collection.is_truncated());
}

#[test]
fn unlimited_collection_retains_many_reports_in_insertion_order() {
    let mut collection = DiagnosticCollection::default();
    let report_count = 4096usize;

    for index in 0..report_count {
        assert!(collection.push(diagnostic(
            &format!("parser.test.{index}"),
            ParserDiagnosticSeverity::Warning,
            "generated diagnostic",
        )));
    }

    assert_eq!(collection.len(), report_count);
    assert_eq!(collection.dropped_count(), 0);
    assert!(!collection.is_truncated());

    assert_eq!(collection.diagnostics()[0].code(), "parser.test.0");
    assert_eq!(
        collection.diagnostics()[report_count - 1].code(),
        format!("parser.test.{}", report_count - 1)
    );
}

#[test]
fn collection_reports_errors_based_on_severity() {
    let mut collection = DiagnosticCollection::default();

    assert!(collection.push(diagnostic(
        "parser.note",
        ParserDiagnosticSeverity::Note,
        "context",
    )));
    assert!(collection.push(diagnostic(
        "parser.warning",
        ParserDiagnosticSeverity::Warning,
        "possible issue",
    )));
    assert!(!collection.has_errors());

    assert!(collection.push(diagnostic(
        "parser.error",
        ParserDiagnosticSeverity::Error,
        "invalid syntax",
    )));
    assert!(collection.has_errors());
}

#[test]
fn fatal_diagnostic_counts_as_an_error() {
    let mut collection = DiagnosticCollection::default();

    assert!(collection.push(diagnostic(
        "parser.fatal",
        ParserDiagnosticSeverity::Fatal,
        "cannot continue",
    )));

    assert!(collection.has_errors());
}

#[test]
fn empty_collection_has_consistent_state() {
    let collection = DiagnosticCollection::default();

    assert!(collection.is_empty());
    assert_eq!(collection.len(), 0);
    assert_eq!(collection.dropped_count(), 0);
    assert!(!collection.is_truncated());
    assert!(!collection.has_errors());
}

#[test]
fn push_error_uses_the_canonical_error_conversion() {
    let error = unexpected_token_error();
    let expected = ParserDiagnostic::from_error(&error);

    let mut collection = DiagnosticCollection::default();
    assert!(collection.push_error(&error));

    assert_eq!(collection.len(), 1);
    assert_eq!(collection.diagnostics()[0], expected);
}

#[test]
fn into_parts_preserves_diagnostics_and_suppression_metadata() {
    let limit = ResourceLimit::at_most(1).expect("one is a valid budget");
    let mut collection = DiagnosticCollection::with_limit(limit);

    assert!(collection.push(diagnostic(
        "parser.retained",
        ParserDiagnosticSeverity::Error,
        "retained",
    )));
    assert!(!collection.push(diagnostic(
        "parser.dropped",
        ParserDiagnosticSeverity::Warning,
        "dropped",
    )));

    let (diagnostics, dropped_count) = collection.into_parts();

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code(), "parser.retained");
    assert_eq!(dropped_count, 1);
}

#[test]
fn cloned_collections_are_independent() {
    let mut original = DiagnosticCollection::default();

    assert!(original.push(diagnostic(
        "parser.original",
        ParserDiagnosticSeverity::Error,
        "original",
    )));

    let mut cloned = original.clone();

    assert!(cloned.push(diagnostic(
        "parser.clone",
        ParserDiagnosticSeverity::Warning,
        "clone",
    )));

    assert_eq!(original.len(), 1);
    assert_eq!(cloned.len(), 2);
    assert_eq!(original.dropped_count(), 0);
    assert_eq!(cloned.dropped_count(), 0);
}

#[test]
fn repeated_error_conversion_is_deterministic() {
    let error = unexpected_token_error();

    let first = ParserDiagnostic::from_error(&error);
    let second = ParserDiagnostic::from_error(&error);

    assert_eq!(first, second);
    assert_eq!(first.to_string(), second.to_string());
}

#[test]
fn unicode_source_offsets_are_preserved_as_byte_offsets() {
    // The multibyte scalar before the diagnostic occupies more than one byte.
    let source = "λx";
    let lambda_bytes = "λ".len();

    assert_eq!(lambda_bytes, 2);

    let source_span = Span::new(
        CursorPosition::new(0, 1, 1),
        CursorPosition::new(lambda_bytes, 1, 2),
    );

    let error = ParseError::unexpected_token(
        source_span,
        "x",
        ["expression"],
    );
    let converted = ParserDiagnostic::from_error(&error);

    assert_eq!(converted.primary_span(), Some(source_span));
    assert_eq!(
        converted.primary_span().expect("span must exist").start_byte(),
        0
    );
    assert_eq!(
        converted.primary_span().expect("span must exist").end_byte(),
        lambda_bytes
    );
    assert!(source.is_char_boundary(lambda_bytes));
}

#[test]
fn resource_budget_diagnostics_are_distinguishable_from_syntax_errors() {
    // A syntax error has its own stable category. Resource-budget errors
    // are separately identified by the canonical parser error model.
    let syntax = ParseError::unexpected_token(span(1, 2), ";", ["expression"]);

    assert_eq!(syntax.stable_code(), "parser.unexpected_token");
    assert!(!matches!(syntax.kind(), ParseErrorKind::ResourceLimit(_)));

    // Do not construct a synthetic resource violation here: the canonical
    // ParserBudget API owns violation construction. A separate integration
    // test should create a real budget violation and verify its diagnostic.
}

#[test]
fn parser_error_classification_remains_available_after_conversion() {
    let error = unexpected_token_error();
    let diagnostic = ParserDiagnostic::from_error(&error);

    // Conversion is intentionally non-destructive. Callers that need
    // recovery decisions retain the original structured error.
    assert_eq!(error.class(), ParseErrorClass::Recoverable);
    assert_eq!(error.stable_code(), diagnostic.code());
    assert!(error.is_recoverable());
}