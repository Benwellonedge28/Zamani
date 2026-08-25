//! Production contract tests for `quantum::frontend::core::diagnostics`.
//!
//! This file intentionally tests the diagnostics contract from outside the
//! implementation file. It therefore verifies behavior exposed to the rest of
//! the frontend rather than relying on private implementation details.
//!
//! # Scope
//!
//! These tests cover:
//!
//! - diagnostic severity;
//! - stable diagnostic codes;
//! - source labels and spans;
//! - primary/secondary label invariants;
//! - notes and help;
//! - child-entry limits;
//! - diagnostic truncation;
//! - bounded diagnostic bags;
//! - diagnostic counts;
//! - insertion-order preservation;
//! - deterministic sorting;
//! - rendering;
//! - empty/boundary cases;
//! - deterministic behavior across repeated operations;
//! - compatibility with the canonical `SourceSpan` contract.
//!
//! # Integration contract
//!
//! This module depends only on:
//!
//! - `frontend::core::diagnostics`;
//! - `frontend::core::source`.
//!
//! It must not depend on OpenQASM, Quantum IR, parsers, validators,
//! importers, exporters, hardware, filesystem, network, or runtime code.
//!
//! The diagnostics implementation owns behavior. These tests only verify the
//! public contract.
//!
//! # Rust compatibility
//!
//! Rust 2021 / Rust 1.97.1.
//! No nightly features.
//! No additional dependencies.
//!
//! # Test integration
//!
//! Wire this module from `src/quantum/frontend/tests/mod.rs`:
//
//! ```ignore
//! pub mod diagnostics;
//! ```
//!
//! and wire the frontend test namespace from the frontend test module as
//! appropriate for the repository's final module layout.
//!
//! These tests deliberately use `SourceId::from_raw`, `SourceOffset::from_raw`,
//! and `SourceSpan::new`, matching the current canonical source API.

use crate::quantum::frontend::core::diagnostics::{
    error, note, render_plain, warning, Diagnostic, DiagnosticBag,
    DiagnosticBuilder, DiagnosticCode, DiagnosticSeverity,
};
use crate::quantum::frontend::core::source::{
    SourceId, SourceOffset, SourceSpan,
};

/// Creates a stable non-zero diagnostic code for a test.
fn code(number: u32) -> DiagnosticCode {
    DiagnosticCode::new(number).expect("test diagnostic code must be non-zero")
}

/// Creates a valid source span using the canonical frontend source API.
fn span(source: u32, start: u32, end: u32) -> SourceSpan {
    SourceSpan::new(
        SourceId::from_raw(source),
        SourceOffset::from_raw(start),
        SourceOffset::from_raw(end),
    )
    .expect("test span must be valid")
}

// =============================================================================
// Severity
// =============================================================================

#[test]
fn severity_strings_are_stable() {
    assert_eq!(DiagnosticSeverity::Note.as_str(), "note");
    assert_eq!(DiagnosticSeverity::Warning.as_str(), "warning");
    assert_eq!(DiagnosticSeverity::Error.as_str(), "error");
}

#[test]
fn severity_display_matches_machine_string() {
    assert_eq!(DiagnosticSeverity::Note.to_string(), "note");
    assert_eq!(DiagnosticSeverity::Warning.to_string(), "warning");
    assert_eq!(DiagnosticSeverity::Error.to_string(), "error");
}

#[test]
fn severity_classification_is_mutually_consistent() {
    assert!(DiagnosticSeverity::Error.is_error());
    assert!(!DiagnosticSeverity::Error.is_warning());
    assert!(!DiagnosticSeverity::Error.is_note());

    assert!(!DiagnosticSeverity::Warning.is_error());
    assert!(DiagnosticSeverity::Warning.is_warning());
    assert!(!DiagnosticSeverity::Warning.is_note());

    assert!(!DiagnosticSeverity::Note.is_error());
    assert!(!DiagnosticSeverity::Note.is_warning());
    assert!(DiagnosticSeverity::Note.is_note());
}

#[test]
fn severity_order_is_stable() {
    assert!(DiagnosticSeverity::Note < DiagnosticSeverity::Warning);
    assert!(DiagnosticSeverity::Warning < DiagnosticSeverity::Error);
}

// =============================================================================
// Diagnostic codes
// =============================================================================

#[test]
fn diagnostic_code_rejects_reserved_zero() {
    assert!(DiagnosticCode::new(0).is_none());
}

#[test]
fn diagnostic_code_preserves_numeric_value() {
    let diagnostic_code = code(42);

    assert_eq!(diagnostic_code.number(), 42);
}

#[test]
fn diagnostic_code_has_stable_qf_representation() {
    assert_eq!(code(1).as_str(), "QF0001");
    assert_eq!(code(42).as_str(), "QF0042");
    assert_eq!(code(9999).as_str(), "QF9999");
}

#[test]
fn diagnostic_code_display_matches_string_representation() {
    for number in [1, 2, 42, 100, 999, 9999] {
        let diagnostic_code = code(number);

        assert_eq!(
            diagnostic_code.to_string(),
            diagnostic_code.as_str()
        );
    }
}

#[test]
fn diagnostic_codes_are_value_comparable() {
    assert_eq!(code(10), code(10));
    assert_ne!(code(10), code(11));
    assert!(code(10) < code(11));
}

// =============================================================================
// Diagnostic construction
// =============================================================================

#[test]
fn diagnostic_constructor_preserves_core_fields() {
    let diagnostic = Diagnostic::new(
        DiagnosticSeverity::Error,
        code(100),
        "invalid quantum operation",
    );

    assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
    assert_eq!(diagnostic.code(), code(100));
    assert_eq!(
        diagnostic.message(),
        "invalid quantum operation"
    );
    assert!(diagnostic.is_bare());
    assert_eq!(diagnostic.child_count(), 0);
    assert!(!diagnostic.children_truncated());
}

#[test]
fn convenience_error_constructor_creates_error() {
    let diagnostic = error(code(101), "invalid syntax");

    assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
    assert_eq!(diagnostic.code(), code(101));
    assert_eq!(diagnostic.message(), "invalid syntax");
}

#[test]
fn convenience_warning_constructor_creates_warning() {
    let diagnostic = warning(code(102), "deprecated construct");

    assert_eq!(diagnostic.severity(), DiagnosticSeverity::Warning);
    assert_eq!(diagnostic.code(), code(102));
    assert_eq!(diagnostic.message(), "deprecated construct");
}

#[test]
fn convenience_note_constructor_creates_note() {
    let diagnostic = note(code(103), "using default behavior");

    assert_eq!(diagnostic.severity(), DiagnosticSeverity::Note);
    assert_eq!(diagnostic.code(), code(103));
    assert_eq!(diagnostic.message(), "using default behavior");
}

// =============================================================================
// Primary source labels
// =============================================================================

#[test]
fn primary_label_is_stored_and_retrievable() {
    let location = span(1, 10, 15);

    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(200),
        "invalid gate",
    )
    .primary(location, "gate invocation")
    .build();

    assert_eq!(diagnostic.primary_span(), Some(location));

    let primary = diagnostic
        .primary_label()
        .expect("primary label must exist");

    assert!(primary.is_primary());
    assert!(!primary.is_secondary());
    assert_eq!(primary.span(), location);
    assert_eq!(primary.message(), "gate invocation");
}

#[test]
fn primary_label_is_first_label() {
    let primary_span = span(1, 10, 12);
    let secondary_span = span(1, 30, 32);

    let mut diagnostic = error(code(201), "mismatch");

    diagnostic.set_primary_label(primary_span, "use");
    assert!(diagnostic.add_secondary_label(
        secondary_span,
        "declaration",
    ));

    assert_eq!(diagnostic.labels().len(), 2);
    assert!(diagnostic.labels()[0].is_primary());
    assert!(diagnostic.labels()[1].is_secondary());
}

#[test]
fn setting_primary_label_replaces_existing_primary_without_duplication() {
    let first = span(1, 10, 12);
    let second = span(1, 20, 22);

    let mut diagnostic = error(code(202), "invalid");

    diagnostic.set_primary_label(first, "first");
    diagnostic.set_primary_label(second, "second");

    assert_eq!(diagnostic.labels().len(), 1);
    assert_eq!(diagnostic.primary_span(), Some(second));
    assert_eq!(
        diagnostic
            .primary_label()
            .expect("primary label")
            .message(),
        "second"
    );
}

#[test]
fn replacing_primary_label_preserves_existing_secondary_labels() {
    let first = span(1, 10, 12);
    let replacement = span(1, 20, 22);
    let secondary = span(1, 40, 42);

    let mut diagnostic = error(code(203), "invalid");

    diagnostic.set_primary_label(first, "first");
    assert!(diagnostic.add_secondary_label(
        secondary,
        "related declaration",
    ));

    diagnostic.set_primary_label(replacement, "replacement");

    assert_eq!(diagnostic.labels().len(), 2);
    assert_eq!(diagnostic.primary_span(), Some(replacement));
    assert_eq!(diagnostic.labels()[1].span(), secondary);
    assert!(diagnostic.labels()[1].is_secondary());
}

#[test]
fn diagnostic_without_primary_label_has_no_primary_span() {
    let diagnostic = error(code(204), "invalid");

    assert!(diagnostic.primary_label().is_none());
    assert!(diagnostic.primary_span().is_none());
}

// =============================================================================
// Secondary labels
// =============================================================================

#[test]
fn secondary_labels_preserve_insertion_order() {
    let first = span(1, 20, 21);
    let second = span(1, 30, 31);
    let third = span(1, 40, 41);

    let mut diagnostic = error(code(300), "related locations");

    diagnostic.set_primary_label(span(1, 10, 11), "primary");

    assert!(diagnostic.add_secondary_label(first, "first"));
    assert!(diagnostic.add_secondary_label(second, "second"));
    assert!(diagnostic.add_secondary_label(third, "third"));

    assert_eq!(diagnostic.labels().len(), 4);
    assert_eq!(diagnostic.labels()[1].span(), first);
    assert_eq!(diagnostic.labels()[2].span(), second);
    assert_eq!(diagnostic.labels()[3].span(), third);
}

#[test]
fn secondary_label_reports_correct_kind() {
    let mut diagnostic = error(code(301), "mismatch");

    assert!(diagnostic.add_secondary_label(
        span(1, 5, 6),
        "related",
    ));

    let label = &diagnostic.labels()[0];

    assert!(label.is_secondary());
    assert!(!label.is_primary());
}

// =============================================================================
// Notes and help
// =============================================================================

#[test]
fn notes_are_preserved_in_insertion_order() {
    let mut diagnostic = error(code(400), "invalid");

    assert!(diagnostic.add_note("first note"));
    assert!(diagnostic.add_note("second note"));
    assert!(diagnostic.add_note("third note"));

    assert_eq!(
        diagnostic.notes(),
        &[
            "first note".to_owned(),
            "second note".to_owned(),
            "third note".to_owned(),
        ]
    );
}

#[test]
fn help_messages_are_preserved_in_insertion_order() {
    let mut diagnostic = error(code(401), "invalid");

    assert!(diagnostic.add_help("first help"));
    assert!(diagnostic.add_help("second help"));

    assert_eq!(
        diagnostic.helps(),
        &[
            "first help".to_owned(),
            "second help".to_owned(),
        ]
    );
}

#[test]
fn notes_and_help_contribute_to_child_count() {
    let mut diagnostic = error(code(402), "invalid");

    diagnostic.set_primary_label(span(1, 1, 2), "primary");
    assert!(diagnostic.add_secondary_label(
        span(1, 3, 4),
        "secondary",
    ));
    assert!(diagnostic.add_note("note"));
    assert!(diagnostic.add_help("help"));

    assert_eq!(diagnostic.child_count(), 4);
    assert!(!diagnostic.is_bare());
}

// =============================================================================
// Child bounds
// =============================================================================

#[test]
fn child_limit_is_shared_by_labels_notes_and_help() {
    let mut diagnostic = Diagnostic::with_max_children(
        DiagnosticSeverity::Error,
        code(500),
        "bounded",
        4,
    );

    diagnostic.set_primary_label(span(1, 1, 2), "primary");

    assert!(diagnostic.add_secondary_label(
        span(1, 3, 4),
        "secondary",
    ));

    assert!(diagnostic.add_note("note"));
    assert!(diagnostic.add_help("help"));

    assert_eq!(diagnostic.child_count(), 4);
    assert!(!diagnostic.children_truncated());

    assert!(!diagnostic.add_note("overflow"));
    assert!(diagnostic.children_truncated());
    assert_eq!(diagnostic.child_count(), 4);
}

#[test]
fn child_limit_zero_rejects_all_children() {
    let mut diagnostic = Diagnostic::with_max_children(
        DiagnosticSeverity::Error,
        code(501),
        "bounded",
        0,
    );

    diagnostic.set_primary_label(span(1, 1, 2), "primary");

    assert!(diagnostic.labels().is_empty());
    assert!(diagnostic.children_truncated());

    assert!(!diagnostic.add_secondary_label(
        span(1, 3, 4),
        "secondary",
    ));
    assert!(!diagnostic.add_note("note"));
    assert!(!diagnostic.add_help("help"));

    assert!(diagnostic.children_truncated());
    assert_eq!(diagnostic.child_count(), 0);
}

#[test]
fn replacing_existing_primary_does_not_consume_an_additional_child_slot() {
    let mut diagnostic = Diagnostic::with_max_children(
        DiagnosticSeverity::Error,
        code(502),
        "bounded",
        1,
    );

    diagnostic.set_primary_label(span(1, 1, 2), "first");
    assert!(!diagnostic.children_truncated());

    diagnostic.set_primary_label(span(1, 3, 4), "replacement");

    assert_eq!(diagnostic.child_count(), 1);
    assert!(!diagnostic.children_truncated());
    assert_eq!(
        diagnostic.primary_span(),
        Some(span(1, 3, 4))
    );
}

#[test]
fn rejected_child_does_not_change_existing_children() {
    let mut diagnostic = Diagnostic::with_max_children(
        DiagnosticSeverity::Error,
        code(503),
        "bounded",
        1,
    );

    diagnostic.set_primary_label(span(1, 1, 2), "primary");

    assert!(!diagnostic.add_help("rejected"));

    assert_eq!(diagnostic.child_count(), 1);
    assert_eq!(diagnostic.labels().len(), 1);
    assert!(diagnostic.notes().is_empty());
    assert!(diagnostic.helps().is_empty());
    assert!(diagnostic.children_truncated());
}

// =============================================================================
// DiagnosticBag construction and bounds
// =============================================================================

#[test]
fn default_bag_is_empty() {
    let bag = DiagnosticBag::new();

    assert!(bag.is_empty());
    assert_eq!(bag.len(), 0);
    assert!(!bag.is_truncated());
}

#[test]
fn bag_accepts_diagnostics_until_limit() {
    let mut bag = DiagnosticBag::with_max_diagnostics(2);

    assert!(bag.push(error(code(600), "first")));
    assert!(bag.push(error(code(601), "second")));

    assert_eq!(bag.len(), 2);
    assert!(!bag.is_truncated());
}

#[test]
fn bag_rejects_diagnostics_after_limit() {
    let mut bag = DiagnosticBag::with_max_diagnostics(2);

    assert!(bag.push(error(code(602), "first")));
    assert!(bag.push(error(code(603), "second")));
    assert!(!bag.push(error(code(604), "third")));

    assert_eq!(bag.len(), 2);
    assert!(bag.is_truncated());
}

#[test]
fn zero_capacity_bag_is_always_truncated_after_push_attempt() {
    let mut bag = DiagnosticBag::with_max_diagnostics(0);

    assert!(!bag.push(error(code(605), "rejected")));

    assert!(bag.is_empty());
    assert_eq!(bag.len(), 0);
    assert!(bag.is_truncated());
}

#[test]
fn bag_u64_capacity_conversion_succeeds_for_supported_usize_value() {
    let bag = DiagnosticBag::with_max_diagnostics_u64(32)
        .expect("32 must fit into usize");

    assert_eq!(bag.max_diagnostics(), 32);
}

#[test]
fn bag_u64_capacity_conversion_is_checked() {
    let value = (usize::MAX as u64).min(u64::MAX);

    let bag = DiagnosticBag::with_max_diagnostics_u64(value);

    if usize::try_from(value).is_ok() {
        assert!(bag.is_some());
    } else {
        assert!(bag.is_none());
    }
}

#[test]
fn bag_extend_stops_at_capacity() {
    let mut bag = DiagnosticBag::with_max_diagnostics(2);

    let inserted = bag.extend([
        error(code(610), "first"),
        warning(code(611), "second"),
        note(code(612), "third"),
    ]);

    assert_eq!(inserted, 2);
    assert_eq!(bag.len(), 2);
    assert!(bag.is_truncated());
}

#[test]
fn bag_extend_empty_iterator_does_not_truncate() {
    let mut bag = DiagnosticBag::with_max_diagnostics(2);

    let inserted = bag.extend(std::iter::empty());

    assert_eq!(inserted, 0);
    assert!(bag.is_empty());
    assert!(!bag.is_truncated());
}

// =============================================================================
// Diagnostic counts
// =============================================================================

#[test]
fn bag_counts_severities_independently() {
    let mut bag = DiagnosticBag::with_max_diagnostics(10);

    bag.push(error(code(700), "error one"));
    bag.push(error(code(701), "error two"));
    bag.push(warning(code(702), "warning one"));
    bag.push(note(code(703), "note one"));
    bag.push(note(code(704), "note two"));

    assert_eq!(bag.error_count(), 2);
    assert_eq!(bag.warning_count(), 1);
    assert_eq!(bag.note_count(), 2);

    assert!(bag.has_errors());
    assert!(bag.has_warnings());
}

#[test]
fn empty_bag_has_no_severity_counts() {
    let bag = DiagnosticBag::new();

    assert_eq!(bag.error_count(), 0);
    assert_eq!(bag.warning_count(), 0);
    assert_eq!(bag.note_count(), 0);
    assert!(!bag.has_errors());
    assert!(!bag.has_warnings());
}

#[test]
fn note_only_bag_has_no_errors_or_warnings() {
    let mut bag = DiagnosticBag::with_max_diagnostics(4);

    bag.push(note(code(701), "informational"));

    assert_eq!(bag.error_count(), 0);
    assert_eq!(bag.warning_count(), 0);
    assert_eq!(bag.note_count(), 1);
    assert!(!bag.has_errors());
    assert!(!bag.has_warnings());
}

// =============================================================================
// Bag ordering
// =============================================================================

#[test]
fn bag_preserves_insertion_order() {
    let mut bag = DiagnosticBag::with_max_diagnostics(10);

    bag.push(error(code(800), "third"));
    bag.push(error(code(801), "first"));
    bag.push(error(code(802), "second"));

    assert_eq!(bag.diagnostics()[0].message(), "third");
    assert_eq!(bag.diagnostics()[1].message(), "first");
    assert_eq!(bag.diagnostics()[2].message(), "second");
}

#[test]
fn sorted_returns_new_vector_without_mutating_original_order() {
    let mut bag = DiagnosticBag::with_max_diagnostics(10);

    bag.push(error(code(810), "later"));
    bag.push(error(code(811), "earlier"));

    let sorted = bag.sorted();

    assert_eq!(bag.diagnostics()[0].message(), "later");
    assert_eq!(bag.diagnostics()[1].message(), "earlier");

    assert_eq!(sorted[0].message(), "earlier");
    assert_eq!(sorted[1].message(), "later");
}

#[test]
fn sorting_orders_source_locations_deterministically() {
    let mut bag = DiagnosticBag::with_max_diagnostics(10);

    let later = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(820),
        "later",
    )
    .primary(span(1, 30, 31), "later")
    .build();

    let earlier = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(821),
        "earlier",
    )
    .primary(span(1, 10, 11), "earlier")
    .build();

    bag.push(later);
    bag.push(earlier);

    let sorted = bag.sorted();

    assert_eq!(sorted[0].primary_span(), Some(span(1, 10, 11)));
    assert_eq!(sorted[1].primary_span(), Some(span(1, 30, 31)));
}

#[test]
fn sorting_orders_sources_deterministically() {
    let mut bag = DiagnosticBag::with_max_diagnostics(10);

    let source_two = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(830),
        "source two",
    )
    .primary(span(2, 1, 2), "source two")
    .build();

    let source_one = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(831),
        "source one",
    )
    .primary(span(1, 1, 2), "source one")
    .build();

    bag.push(source_two);
    bag.push(source_one);

    let sorted = bag.sorted();

    assert_eq!(
        sorted[0].primary_span(),
        Some(span(1, 1, 2))
    );
    assert_eq!(
        sorted[1].primary_span(),
        Some(span(2, 1, 2))
    );
}

#[test]
fn diagnostics_without_primary_span_sort_after_located_diagnostics() {
    let mut bag = DiagnosticBag::with_max_diagnostics(10);

    let unlocated = error(code(840), "unlocated");
    let located = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(841),
        "located",
    )
    .primary(span(1, 1, 2), "location")
    .build();

    bag.push(unlocated);
    bag.push(located);

    let sorted = bag.sorted();

    assert!(sorted[0].primary_span().is_some());
    assert!(sorted[1].primary_span().is_none());
}

#[test]
fn equal_source_location_is_tiebroken_deterministically() {
    let mut bag = DiagnosticBag::with_max_diagnostics(10);

    let first = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(850),
        "same location",
    )
    .primary(span(1, 10, 12), "same")
    .build();

    let second = DiagnosticBuilder::new(
        DiagnosticSeverity::Warning,
        code(851),
        "same location",
    )
    .primary(span(1, 10, 12), "same")
    .build();

    bag.push(second);
    bag.push(first);

    let sorted = bag.sorted();

    // The canonical ordering puts errors before warnings at the same source
    // location.
    assert_eq!(sorted[0].severity(), DiagnosticSeverity::Error);
    assert_eq!(sorted[1].severity(), DiagnosticSeverity::Warning);
}

#[test]
fn repeated_sorting_is_deterministic() {
    let mut bag = DiagnosticBag::with_max_diagnostics(20);

    bag.push(error(code(860), "z"));
    bag.push(note(code(861), "a"));
    bag.push(warning(code(862), "m"));
    bag.push(
        DiagnosticBuilder::new(
            DiagnosticSeverity::Error,
            code(863),
            "source",
        )
        .primary(span(3, 5, 8), "source")
        .build(),
    );
    bag.push(
        DiagnosticBuilder::new(
            DiagnosticSeverity::Error,
            code(864),
            "source earlier",
        )
        .primary(span(1, 1, 2), "source")
        .build(),
    );

    let first = bag.sorted();
    let second = bag.sorted();
    let third = bag.sorted();

    assert_eq!(first, second);
    assert_eq!(second, third);
}

// =============================================================================
// In-place sorting
// =============================================================================

#[test]
fn in_place_sort_matches_sorted_copy() {
    let mut bag = DiagnosticBag::with_max_diagnostics(10);

    bag.push(error(code(870), "third"));
    bag.push(
        error(code(871), "first")
    );
    bag.push(
        DiagnosticBuilder::new(
            DiagnosticSeverity::Warning,
            code(872),
            "second",
        )
        .primary(span(1, 20, 21), "second")
        .build(),
    );

    let expected = bag.sorted();

    bag.sort();

    assert_eq!(bag.diagnostics(), expected.as_slice());
}

#[test]
fn sorting_empty_bag_is_safe() {
    let mut bag = DiagnosticBag::new();

    bag.sort();

    assert!(bag.is_empty());
    assert!(!bag.is_truncated());
}

// =============================================================================
// Clear/reset behavior
// =============================================================================

#[test]
fn clear_removes_diagnostics_and_resets_truncation() {
    let mut bag = DiagnosticBag::with_max_diagnostics(1);

    assert!(bag.push(error(code(900), "first")));
    assert!(!bag.push(error(code(901), "overflow")));

    assert_eq!(bag.len(), 1);
    assert!(bag.is_truncated());

    bag.clear();

    assert!(bag.is_empty());
    assert_eq!(bag.len(), 0);
    assert!(!bag.is_truncated());

    assert!(bag.push(error(code(902), "after clear")));
    assert_eq!(bag.len(), 1);
    assert!(!bag.is_truncated());
}

// =============================================================================
// Iteration contracts
// =============================================================================

#[test]
fn_shared_iteration_preserves_insertion_order() {
    let mut bag = DiagnosticBag::with_max_diagnostics(3);

    bag.push(error(code(910), "one"));
    bag.push(error(code(911), "two"));
    bag.push(error(code(912), "three"));

    let messages: Vec<&str> =
        bag.iter().map(Diagnostic::message).collect();

    assert_eq!(messages, vec!["one", "two", "three"]);
}

#[test]
fn borrowed_into_iterator_matches_iter() {
    let mut bag = DiagnosticBag::with_max_diagnostics(3);

    bag.push(error(code(920), "one"));
    bag.push(error(code(921), "two"));

    let from_iter: Vec<&str> =
        (&bag).into_iter().map(Diagnostic::message).collect();

    let from_method: Vec<&str> =
        bag.iter().map(Diagnostic::message).collect();

    assert_eq!(from_iter, from_method);
}

#[test]
fn owned_into_iterator_consumes_bag() {
    let mut bag = DiagnosticBag::with_max_diagnostics(3);

    bag.push(error(code(930), "one"));
    bag.push(error(code(931), "two"));

    let messages: Vec<String> = bag
        .into_iter()
        .map(|diagnostic| diagnostic.message().to_owned())
        .collect();

    assert_eq!(messages, vec!["one", "two"]);
}

#[test]
fn into_vec_preserves_insertion_order() {
    let mut bag = DiagnosticBag::with_max_diagnostics(3);

    bag.push(error(code(940), "one"));
    bag.push(error(code(941), "two"));

    let diagnostics = bag.into_vec();

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].message(), "one");
    assert_eq!(diagnostics[1].message(), "two");
}

#[test]
fn mutable_iteration_can_enrich_diagnostics() {
    let mut bag = DiagnosticBag::with_max_diagnostics(2);

    bag.push(error(code(950), "original"));

    for diagnostic in bag.iter_mut() {
        diagnostic.set_primary_label(
            span(1, 5, 6),
            "enriched",
        );
    }

    assert_eq!(
        bag.diagnostics()[0].primary_span(),
        Some(span(1, 5, 6))
    );
}

// =============================================================================
// Builder
// =============================================================================

#[test]
fn builder_constructs_complete_diagnostic() {
    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1000),
        "invalid operation",
    )
    .primary(span(1, 10, 12), "operation")
    .secondary(span(1, 20, 22), "declaration")
    .note("the operands must have equal width")
    .help("use matching registers")
    .build();

    assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
    assert_eq!(diagnostic.code(), code(1000));
    assert_eq!(diagnostic.message(), "invalid operation");
    assert_eq!(diagnostic.labels().len(), 2);
    assert_eq!(diagnostic.notes().len(), 1);
    assert_eq!(diagnostic.helps().len(), 1);
    assert_eq!(diagnostic.child_count(), 4);
    assert!(!diagnostic.children_truncated());
}

#[test]
fn builder_respects_explicit_child_limit() {
    let diagnostic = DiagnosticBuilder::with_max_children(
        DiagnosticSeverity::Error,
        code(1001),
        "bounded",
        2,
    )
    .primary(span(1, 1, 2), "primary")
    .secondary(span(1, 3, 4), "secondary")
    .note("rejected by limit")
    .help("also rejected")
    .build();

    assert_eq!(diagnostic.child_count(), 2);
    assert_eq!(diagnostic.labels().len(), 2);
    assert!(diagnostic.notes().is_empty());
    assert!(diagnostic.helps().is_empty());
    assert!(diagnostic.children_truncated());
}

#[test]
fn builder_with_zero_child_limit_produces_bare_diagnostic() {
    let diagnostic = DiagnosticBuilder::with_max_children(
        DiagnosticSeverity::Error,
        code(1002),
        "bounded",
        0,
    )
    .primary(span(1, 1, 2), "primary")
    .secondary(span(1, 3, 4), "secondary")
    .note("note")
    .help("help")
    .build();

    assert!(diagnostic.is_bare());
    assert_eq!(diagnostic.child_count(), 0);
    assert!(diagnostic.children_truncated());
}

// =============================================================================
// Rendering
// =============================================================================

#[test]
fn plain_renderer_contains_severity_code_and_message() {
    let diagnostic = error(
        code(1100),
        "invalid gate invocation",
    );

    let rendered = render_plain(&diagnostic);

    assert!(rendered.contains("error"));
    assert!(rendered.contains("QF1100"));
    assert!(rendered.contains("invalid gate invocation"));
}

#[test]
fn plain_renderer_contains_primary_and_secondary_labels() {
    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1101),
        "register mismatch",
    )
    .primary(
        span(1, 10, 12),
        "operation uses this register",
    )
    .secondary(
        span(1, 30, 32),
        "register declared here",
    )
    .build();

    let rendered = render_plain(&diagnostic);

    assert!(rendered.contains("QF1101"));
    assert!(rendered.contains("operation uses this register"));
    assert!(rendered.contains("register declared here"));
    assert!(rendered.contains("1:10-12"));
    assert!(rendered.contains("1:30-32"));
}

#[test]
fn plain_renderer_contains_notes_and_help() {
    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1102),
        "invalid operation",
    )
    .note("the operation requires two qubits")
    .help("provide two qubit operands")
    .build();

    let rendered = render_plain(&diagnostic);

    assert!(rendered.contains("note: the operation requires two qubits"));
    assert!(rendered.contains("help: provide two qubit operands"));
}

#[test]
fn plain_renderer_reports_truncated_children() {
    let mut diagnostic = Diagnostic::with_max_children(
        DiagnosticSeverity::Error,
        code(1103),
        "bounded",
        1,
    );

    diagnostic.set_primary_label(
        span(1, 1, 2),
        "primary",
    );

    assert!(!diagnostic.add_help("truncated"));

    let rendered = render_plain(&diagnostic);

    assert!(rendered.contains(
        "additional diagnostic details were truncated"
    ));
}

#[test]
fn plain_renderer_is_deterministic() {
    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1104),
        "deterministic diagnostic",
    )
    .primary(span(2, 100, 105), "primary")
    .secondary(span(2, 200, 205), "secondary")
    .note("note")
    .help("help")
    .build();

    let first = render_plain(&diagnostic);
    let second = render_plain(&diagnostic);

    assert_eq!(first, second);
}

#[test]
fn plain_renderer_handles_bare_diagnostic() {
    let diagnostic = error(code(1105), "bare diagnostic");

    let rendered = render_plain(&diagnostic);

    assert_eq!(
        rendered,
        "error[QF1105]: bare diagnostic"
    );
}

// =============================================================================
// Source-span integration
// =============================================================================

#[test]
fn diagnostics_can_reference_multiple_source_files() {
    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1200),
        "cross-source relationship",
    )
    .primary(
        span(1, 10, 12),
        "use site",
    )
    .secondary(
        span(2, 20, 22),
        "declaration site",
    )
    .build();

    assert_eq!(
        diagnostic.primary_span(),
        Some(span(1, 10, 12))
    );
    assert_eq!(
        diagnostic.labels()[1].span(),
        span(2, 20, 22)
    );
}

#[test]
fn empty_spans_are_valid_diagnostic_locations() {
    let location = span(1, 10, 10);

    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1201),
        "unexpected end of input",
    )
    .primary(location, "expected token here")
    .build();

    assert_eq!(diagnostic.primary_span(), Some(location));
    assert!(location.is_empty());
}

#[test]
fn diagnostic_location_uses_half_open_source_span_contract() {
    let location = span(1, 10, 15);

    assert_eq!(location.start().as_raw(), 10);
    assert_eq!(location.end().as_raw(), 15);
    assert_eq!(location.len_bytes(), 5);
    assert!(!location.is_empty());
}

// =============================================================================
// Deterministic complete diagnostic ordering
// =============================================================================

#[test]
fn canonical_sorting_orders_location_then_severity_then_code_then_message() {
    let mut bag = DiagnosticBag::with_max_diagnostics(20);

    // Same source and same span: severity is the next deterministic key.
    bag.push(
        DiagnosticBuilder::new(
            DiagnosticSeverity::Warning,
            code(1302),
            "warning",
        )
        .primary(span(1, 10, 12), "warning")
        .build(),
    );

    bag.push(
        DiagnosticBuilder::new(
            DiagnosticSeverity::Error,
            code(1303),
            "error",
        )
        .primary(span(1, 10, 12), "error")
        .build(),
    );

    // Earlier source location must precede later location regardless of
    // insertion order.
    bag.push(
        DiagnosticBuilder::new(
            DiagnosticSeverity::Error,
            code(1304),
            "earlier",
        )
        .primary(span(1, 5, 7), "earlier")
        .build(),
    );

    let sorted = bag.sorted();

    assert_eq!(
        sorted[0].primary_span(),
        Some(span(1, 5, 7))
    );
    assert_eq!(
        sorted[1].severity(),
        DiagnosticSeverity::Error
    );
    assert_eq!(
        sorted[2].severity(),
        DiagnosticSeverity::Warning
    );
}

#[test]
fn canonical_sorting_is_independent_of_insertion_order() {
    let a = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1310),
        "a",
    )
    .primary(span(1, 10, 11), "a")
    .build();

    let b = DiagnosticBuilder::new(
        DiagnosticSeverity::Warning,
        code(1311),
        "b",
    )
    .primary(span(1, 5, 6), "b")
    .build();

    let c = error(code(1312), "c");

    let mut first = DiagnosticBag::with_max_diagnostics(10);
    first.push(a.clone());
    first.push(b.clone());
    first.push(c.clone());

    let mut second = DiagnosticBag::with_max_diagnostics(10);
    second.push(c);
    second.push(a);
    second.push(b);

    assert_eq!(first.sorted(), second.sorted());
}

// =============================================================================
// Resource-boundary behavior
// =============================================================================

#[test]
fn diagnostic_bag_limit_is_explicit_and_queryable() {
    let bag = DiagnosticBag::with_max_diagnostics(7);

    assert_eq!(bag.max_diagnostics(), 7);
}

#[test]
fn diagnostic_bag_never_grows_beyond_configured_limit() {
    let limit = 4;
    let mut bag = DiagnosticBag::with_max_diagnostics(limit);

    for number in 1..=32 {
        let _ = bag.push(error(
            code(1400 + number),
            "bounded diagnostic",
        ));
    }

    assert_eq!(bag.len(), limit);
    assert!(bag.is_truncated());
}

#[test]
fn diagnostic_child_collection_never_grows_beyond_configured_limit() {
    let limit = 3;
    let mut diagnostic = Diagnostic::with_max_children(
        DiagnosticSeverity::Error,
        code(1401),
        "bounded diagnostic",
        limit,
    );

    diagnostic.set_primary_label(
        span(1, 1, 2),
        "primary",
    );

    for index in 0..32 {
        let _ = diagnostic.add_note(format!("note {index}"));
    }

    assert_eq!(diagnostic.child_count(), limit);
    assert!(diagnostic.children_truncated());
}

// =============================================================================
// Edge cases
// =============================================================================

#[test]
fn empty_message_is_preserved() {
    let diagnostic = error(code(1500), "");

    assert_eq!(diagnostic.message(), "");
    assert!(diagnostic.is_bare());
}

#[test]
fn empty_label_message_is_preserved() {
    let mut diagnostic = error(code(1501), "invalid");

    diagnostic.set_primary_label(
        span(1, 1, 2),
        "",
    );

    assert_eq!(
        diagnostic.primary_label()
            .expect("primary")
            .message(),
        ""
    );
}

#[test]
fn unicode_diagnostic_messages_are_preserved() {
    let message = "量子ゲート inválido — خطأ — ошибка";

    let diagnostic = error(code(1502), message);

    assert_eq!(diagnostic.message(), message);
}

#[test]
fn unicode_label_messages_are_preserved() {
    let message = "qubit inválido — 量子";

    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1503),
        "invalid",
    )
    .primary(span(1, 1, 2), message)
    .build();

    assert_eq!(
        diagnostic.primary_label()
            .expect("primary")
            .message(),
        message
    );
}

#[test]
fn unicode_notes_and_help_are_preserved() {
    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1504),
        "invalid",
    )
    .note("nota: tamaño incorrecto")
    .help("修正してください")
    .build();

    assert_eq!(
        diagnostic.notes()[0],
        "nota: tamaño incorrecto"
    );
    assert_eq!(
        diagnostic.helps()[0],
        "修正してください"
    );
}

// =============================================================================
// Clone/equality contracts
// =============================================================================

#[test]
fn diagnostic_clone_preserves_complete_value() {
    let original = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1600),
        "invalid",
    )
    .primary(span(1, 10, 12), "primary")
    .secondary(span(1, 20, 22), "secondary")
    .note("note")
    .help("help")
    .build();

    let cloned = original.clone();

    assert_eq!(original, cloned);
}

#[test]
fn bag_clone_preserves_complete_value() {
    let mut original = DiagnosticBag::with_max_diagnostics(3);

    original.push(error(code(1601), "one"));
    original.push(warning(code(1602), "two"));
    original.push(note(code(1603), "three"));

    let cloned = original.clone();

    assert_eq!(original, cloned);
}

#[test]
fn truncated_bag_clone_preserves_truncation_state() {
    let mut original = DiagnosticBag::with_max_diagnostics(1);

    original.push(error(code(1604), "one"));
    assert!(!original.push(error(code(1605), "overflow")));

    let cloned = original.clone();

    assert_eq!(cloned.len(), 1);
    assert!(cloned.is_truncated());
    assert_eq!(original, cloned);
}

// =============================================================================
// Contract-level regression tests
// =============================================================================

#[test]
fn diagnostics_do_not_require_format_specific_types() {
    // This test intentionally constructs diagnostics exclusively from the
    // generic frontend contracts. If this file ever requires an OpenQASM AST,
    // parser token, Quantum IR operation, or backend type, the architectural
    // boundary has been violated.
    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1700),
        "generic frontend error",
    )
    .primary(span(1, 0, 1), "source")
    .build();

    assert_eq!(diagnostic.code(), code(1700));
}

#[test]
fn diagnostics_are_data_and_do_not_perform_external_side_effects() {
    // Construction and rendering operate entirely on values supplied by the
    // caller. This test intentionally exercises the complete public path
    // without providing filesystem/network/process/hardware capabilities.
    let diagnostic = DiagnosticBuilder::new(
        DiagnosticSeverity::Error,
        code(1701),
        "side-effect-free",
    )
    .primary(span(1, 0, 1), "source")
    .note("note")
    .help("help")
    .build();

    let rendered = render_plain(&diagnostic);

    assert!(rendered.contains("side-effect-free"));
}

#[test]
fn diagnostics_have_stable_machine_identity_independent_of_message() {
    let first = error(code(1702), "message version one");
    let second = error(code(1702), "message version two");

    assert_eq!(first.code(), second.code());
    assert_ne!(first.message(), second.message());
}

#[test]
fn diagnostic_code_is_not_derived_from_message_text() {
    let first = error(code(1703), "alpha");
    let second = error(code(1703), "completely different text");

    assert_eq!(first.code(), second.code());
}

#[test]
fn bag_can_represent_multiple_distinct_diagnostics_with_same_code() {
    let mut bag = DiagnosticBag::with_max_diagnostics(4);

    bag.push(
        DiagnosticBuilder::new(
            DiagnosticSeverity::Error,
            code(1704),
            "same code",
        )
        .primary(span(1, 10, 11), "first location")
        .build(),
    );

    bag.push(
        DiagnosticBuilder::new(
            DiagnosticSeverity::Error,
            code(1704),
            "same code",
        )
        .primary(span(1, 20, 21), "second location")
        .build(),
    );

    assert_eq!(bag.len(), 2);
    assert_ne!(
        bag.diagnostics()[0].primary_span(),
        bag.diagnostics()[1].primary_span()
    );
}

// =============================================================================
// Final production contract smoke test
// =============================================================================

#[test]
fn production_diagnostic_contract_is_complete() {
    let mut bag = DiagnosticBag::with_max_diagnostics(16);

    let diagnostic = DiagnosticBuilder::with_max_children(
        DiagnosticSeverity::Error,
        code(1800),
        "quantum operation is invalid",
        8,
    )
    .primary(
        span(1, 42, 44),
        "invalid operation",
    )
    .secondary(
        span(1, 10, 14),
        "qubit declared here",
    )
    .note(
        "the operation requires operands with compatible widths",
    )
    .help(
        "use registers with matching dimensions",
    )
    .build();

    assert!(bag.push(diagnostic));

    // Structured API.
    assert_eq!(bag.error_count(), 1);
    assert!(bag.has_errors());
    assert!(!bag.has_warnings());
    assert!(!bag.is_truncated());

    let stored = &bag.diagnostics()[0];

    assert_eq!(stored.code(), code(1800));
    assert_eq!(stored.severity(), DiagnosticSeverity::Error);
    assert_eq!(
        stored.primary_span(),
        Some(span(1, 42, 44))
    );
    assert_eq!(stored.labels().len(), 2);
    assert_eq!(stored.notes().len(), 1);
    assert_eq!(stored.helps().len(), 1);
    assert!(!stored.children_truncated());

    // Deterministic ordering.
    let sorted = bag.sorted();
    assert_eq!(sorted, bag.diagnostics());

    // Deterministic rendering.
    let first_render = render_plain(&sorted[0]);
    let second_render = render_plain(&sorted[0]);

    assert_eq!(first_render, second_render);

    // Stable machine-readable identity appears in human rendering.
    assert!(first_render.contains("QF1800"));
}