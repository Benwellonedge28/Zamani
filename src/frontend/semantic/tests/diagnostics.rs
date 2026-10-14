// src/frontend/semantic/tests/diagnostics.rs
//
// Zamani semantic diagnostics — integration and conformance tests.
//
// Rust edition: 2021
// Minimum Rust version: 1.97
//
// Safety: unsafe Rust is forbidden.
//
// Authority:
//   grammar/Zamani.g4
//   grammar/grammar.md
//   grammar/spec/diagnostics.md
//   grammar/spec/semantics.md
//   src/frontend/ast/node/source/span.rs
//   src/frontend/semantic/diagnostics/diagnostic.rs
//   src/frontend/semantic/diagnostics/codes.rs
//   src/frontend/semantic/diagnostics/labels.rs
//   src/frontend/semantic/diagnostics/ordering.rs
//
// Ownership:
//   This file owns integration tests for the diagnostic contract.
//   It does not define diagnostic types, source spans, parser behavior,
//   semantic rules, target feasibility, or diagnostic collection policy.
//
// Integration:
//   The tests use the canonical AST Span and the existing semantic
//   diagnostics API. They intentionally do not depend on an unfinished
//   semantic analyzer implementation.
//
// Test registration:
//   Include this file from the semantic test module with:
//
//       #[path = "diagnostics.rs"]
//       mod diagnostics;
//
//   The parent test module must be compiled by the crate's test configuration.
//   If diagnostics tests are instead registered directly in diagnostics/mod.rs,
//   adjust the module path to avoid compiling this file twice.
//
// Scalability:
//   Tests use data-driven cases and do not establish language-level limits.
//   Large stress tests must use explicit test-runner budgets and should be
//   kept separate from ordinary unit tests.
//
// Important:
//   Passing these tests establishes diagnostic-contract conformance only.
//   It does not certify the semantic pipeline, parser, grammar, or compiler
//   as a whole.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::frontend::ast::node::source::source_id::SourceId;
use crate::frontend::ast::node::source::span::{SourceOffset, Span};

use crate::frontend::semantic::diagnostics::{
    deduplicate_diagnostics, sort_diagnostics, Diagnostic, DiagnosticCategory,
    DiagnosticCode, DiagnosticPhase, DiagnosticSeverity, DiagnosticValue,
    FixIt, FixItEdit, RecoveryClassification,
};

// -----------------------------------------------------------------------------
// Shared test constructors
// -----------------------------------------------------------------------------

/// Construct a canonical half-open source span.
///
/// Coordinates are UTF-8 byte offsets, not character indices or line numbers.
/// Zero-width ranges are valid and are used for EOF and insertion locations.
fn span(source_id: u64, start: u64, end: u64) -> Span {
    Span::new(
        SourceId::from_raw(source_id),
        SourceOffset::from_raw(start),
        SourceOffset::from_raw(end),
    )
    .expect("test spans must satisfy start <= end")
}

/// Construct a diagnostic without depending on a particular language rule.
fn diagnostic(
    code: &str,
    phase: DiagnosticPhase,
    category: DiagnosticCategory,
    severity: DiagnosticSeverity,
    message: &str,
    primary_span: Option<Span>,
) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::from(code),
        phase,
        category,
        severity,
        message,
        primary_span,
    )
}

/// Construct a representative semantic type error.
fn type_error(source_id: u64, start: u64, end: u64) -> Diagnostic {
    diagnostic(
        "ZM-SEM-TYPE-001",
        DiagnosticPhase::TypeAnalysis,
        DiagnosticCategory::Type,
        DiagnosticSeverity::Error,
        "incompatible types",
        Some(span(source_id, start, end)),
    )
}

// -----------------------------------------------------------------------------
// Stable identity and structured diagnostic contract
// -----------------------------------------------------------------------------

#[test]
fn diagnostic_code_is_independent_of_human_readable_message() {
    let item = diagnostic(
        "ZM-SEM-NAME-001",
        DiagnosticPhase::NameResolution,
        DiagnosticCategory::NameResolution,
        DiagnosticSeverity::Error,
        "identifier could not be resolved",
        Some(span(1, 4, 9)),
    );

    assert_eq!(item.code().as_str(), "ZM-SEM-NAME-001");
    assert_eq!(item.message(), "identifier could not be resolved");
    assert!(item.has_valid_code());

    let changed_message = diagnostic(
        "ZM-SEM-NAME-001",
        DiagnosticPhase::NameResolution,
        DiagnosticCategory::NameResolution,
        DiagnosticSeverity::Error,
        "name is not defined in this scope",
        Some(span(1, 4, 9)),
    );

    assert_eq!(item.code(), changed_message.code());
    assert_ne!(item.message(), changed_message.message());
    assert!(!item.same_report_identity(&changed_message));
}

#[test]
fn empty_diagnostic_code_is_detectable_without_panicking() {
    let item = diagnostic(
        "",
        DiagnosticPhase::SemanticFinalization,
        DiagnosticCategory::Internal,
        DiagnosticSeverity::Error,
        "invalid diagnostic producer output",
        None,
    );

    assert!(!item.has_valid_code());
}

#[test]
fn unknown_nonempty_diagnostic_codes_remain_representable() {
    let extension_code = "EXAMPLE-SEM-FUTURE-001";

    let item = diagnostic(
        extension_code,
        DiagnosticPhase::Extension("example.domain".to_owned()),
        DiagnosticCategory::Extension("example.domain".to_owned()),
        DiagnosticSeverity::Warning,
        "an extension reported a condition",
        None,
    );

    assert_eq!(item.code().as_str(), extension_code);
    assert!(item.has_valid_code());
}

#[test]
fn error_severity_does_not_implicitly_determine_recovery() {
    let recoverable = type_error(1, 2, 5)
        .with_recovery(RecoveryClassification::Recoverable);

    let unrecoverable = type_error(1, 2, 5)
        .with_recovery(RecoveryClassification::Unrecoverable);

    assert!(recoverable.is_error());
    assert!(unrecoverable.is_error());

    assert_eq!(
        recoverable.recovery(),
        RecoveryClassification::Recoverable
    );
    assert_eq!(
        unrecoverable.recovery(),
        RecoveryClassification::Unrecoverable
    );

    assert_ne!(recoverable.recovery(), unrecoverable.recovery());
}

#[test]
fn missing_source_span_is_supported_for_session_level_diagnostics() {
    let item = diagnostic(
        "ZM-SEM-ANALYSIS-002",
        DiagnosticPhase::SemanticFinalization,
        DiagnosticCategory::Analysis,
        DiagnosticSeverity::Error,
        "configured analysis budget exhausted",
        None,
    );

    assert_eq!(item.primary_span(), None);
    assert!(item.is_error());
}

// -----------------------------------------------------------------------------
// Canonical source-span contract
// -----------------------------------------------------------------------------

#[test]
fn source_spans_use_half_open_byte_ranges() {
    let item = span(1, 5, 12);

    assert_eq!(item.start_raw(), 5);
    assert_eq!(item.end_raw(), 12);
    assert_eq!(item.source().as_raw(), 1);

    // The range is [5, 12), so its length is seven bytes.
    assert_eq!(item.end_raw() - item.start_raw(), 7);
}

#[test]
fn zero_width_spans_are_valid_for_eof_and_insertions() {
    let eof = span(1, 42, 42);
    let insertion = span(1, 0, 0);

    assert_eq!(eof.start_raw(), eof.end_raw());
    assert_eq!(insertion.start_raw(), insertion.end_raw());

    let item = diagnostic(
        "ZM-SEM-STRUCTURE-001",
        DiagnosticPhase::AstValidation,
        DiagnosticCategory::Structural,
        DiagnosticSeverity::Error,
        "required construct is missing",
        Some(eof),
    );

    assert_eq!(item.primary_span(), Some(eof));
}

#[test]
fn span_constructor_rejects_reversed_ranges() {
    let result = Span::new(
        SourceId::from_raw(1),
        SourceOffset::from_raw(9),
        SourceOffset::from_raw(3),
    );

    assert!(result.is_err());
}

#[test]
fn spans_from_different_source_units_remain_distinct() {
    let first = span(1, 0, 3);
    let second = span(2, 0, 3);

    assert_ne!(first, second);

    let first_error = type_error(1, 0, 3);
    let second_error = type_error(2, 0, 3);

    assert!(!first_error.same_report_identity(&second_error));
}

#[test]
fn unicode_source_locations_are_byte_based() {
    // "λ" occupies two UTF-8 bytes.
    let source = "aλb";

    assert_eq!(source.len(), 4);
    assert_eq!(&source[0..1], "a");
    assert_eq!(&source[1..3], "λ");
    assert_eq!(&source[3..4], "b");

    let lambda_span = span(1, 1, 3);

    assert_eq!(lambda_span.start_raw(), 1);
    assert_eq!(lambda_span.end_raw(), 3);
    assert_eq!(
        &source[lambda_span.start_raw() as usize..lambda_span.end_raw() as usize],
        "λ"
    );
}

#[test]
fn large_representable_offsets_are_not_reduced_to_machine_width() {
    let start = u64::MAX - 1;
    let end = u64::MAX;
    let location = span(9, start, end);

    assert_eq!(location.start_raw(), start);
    assert_eq!(location.end_raw(), end);
    assert_eq!(location.source().as_raw(), 9);
}

// -----------------------------------------------------------------------------
// Primary locations, secondary labels, notes, help and fixes
// -----------------------------------------------------------------------------

#[test]
fn diagnostics_preserve_primary_and_secondary_source_locations() {
    let declaration = span(1, 3, 8);
    let use_site = span(1, 20, 25);

    let item = diagnostic(
        "ZM-SEM-NAME-002",
        DiagnosticPhase::NameResolution,
        DiagnosticCategory::NameResolution,
        DiagnosticSeverity::Error,
        "duplicate declaration",
        Some(use_site),
    )
    .with_label(declaration, "previous declaration is here");

    assert_eq!(item.primary_span(), Some(use_site));
    assert_eq!(item.labels().len(), 1);
    assert_eq!(item.labels()[0].span(), declaration);
    assert_eq!(
        item.labels()[0].message(),
        "previous declaration is here"
    );
}

#[test]
fn secondary_labels_can_refer_to_another_source_file() {
    let declaration = span(11, 5, 13);
    let use_site = span(24, 2, 7);

    let item = diagnostic(
        "ZM-SEM-NAME-001",
        DiagnosticPhase::NameResolution,
        DiagnosticCategory::NameResolution,
        DiagnosticSeverity::Error,
        "unresolved imported symbol",
        Some(use_site),
    )
    .with_label(declaration, "related declaration");

    assert_eq!(item.primary_span(), Some(use_site));
    assert_eq!(item.labels()[0].span(), declaration);
    assert_ne!(
        item.primary_span().expect("primary location exists").source(),
        item.labels()[0].span().source()
    );
}

#[test]
fn notes_and_help_are_structured_fields() {
    let item = type_error(1, 10, 14)
        .with_note("the expression has a different type")
        .with_note("the expected type comes from the function signature")
        .with_help("provide a value of the expected type");

    assert_eq!(item.notes().len(), 2);
    assert_eq!(
        item.notes()[0],
        "the expression has a different type"
    );
    assert_eq!(
        item.help(),
        Some("provide a value of the expected type")
    );
}

#[test]
fn fix_its_preserve_edits_without_mutating_source() {
    let original_source = String::from("invoke(value");
    let expected_source = original_source.clone();

    let fix = FixIt::new(
        "insert a closing delimiter",
        vec![FixItEdit::new(span(1, 12, 12), ")")],
        true,
    );

    let item = diagnostic(
        "ZM-SEM-STRUCTURE-001",
        DiagnosticPhase::AstValidation,
        DiagnosticCategory::Structural,
        DiagnosticSeverity::Error,
        "closing delimiter is missing",
        Some(span(1, 12, 12)),
    )
    .with_fix(fix);

    assert_eq!(original_source, expected_source);
    assert_eq!(item.fixes().len(), 1);
    assert!(item.fixes()[0].is_safe());
    assert_eq!(
        item.fixes()[0].title(),
        "insert a closing delimiter"
    );
    assert_eq!(item.fixes()[0].edits().len(), 1);
    assert_eq!(item.fixes()[0].edits()[0].replacement(), ")");
    assert_eq!(item.fixes()[0].edits()[0].span(), span(1, 12, 12));
}

#[test]
fn empty_fix_its_are_representable_but_not_mistaken_for_edits() {
    let fix = FixIt::new("explain the correction", Vec::new(), false);

    assert!(fix.is_empty());
    assert!(!fix.is_safe());
    assert!(fix.edits().is_empty());
}

// -----------------------------------------------------------------------------
// Structured metadata and extensibility
// -----------------------------------------------------------------------------

#[test]
fn metadata_is_typed_and_preserves_large_unsigned_values() {
    let required_amount = u128::MAX;

    let item = diagnostic(
        "ZM-SEM-RESOURCE-001",
        DiagnosticPhase::ResourceAnalysis,
        DiagnosticCategory::Resource,
        DiagnosticSeverity::Information,
        "symbolic resource requirement retained",
        None,
    )
    .with_metadata("required_amount", required_amount)
    .with_metadata("is_symbolic", true)
    .with_metadata("resource_kind", "compute.units");

    assert_eq!(
        item.metadata_value("required_amount"),
        Some(&DiagnosticValue::Unsigned(required_amount))
    );

    assert_eq!(
        item.metadata_value("is_symbolic"),
        Some(&DiagnosticValue::Boolean(true))
    );

    assert_eq!(
        item.metadata_value("resource_kind"),
        Some(&DiagnosticValue::Text(
            "compute.units".to_owned()
        ))
    );
}

#[test]
fn metadata_objects_have_deterministic_key_order() {
    let item = type_error(1, 1, 2)
        .with_metadata("zeta", "last")
        .with_metadata("alpha", "first")
        .with_metadata("middle", "between");

    let keys: Vec<&str> = item
        .metadata()
        .keys()
        .map(String::as_str)
        .collect();

    assert_eq!(keys, vec!["alpha", "middle", "zeta"]);
}

#[test]
fn nested_metadata_values_remain_structured() {
    let mut object = DiagnosticValue::object();

    object.insert("domain", DiagnosticValue::from("quantum"));
    object.insert("portable", DiagnosticValue::from(true));

    let item = diagnostic(
        "ZM-SEM-FEATURE-001",
        DiagnosticPhase::DomainAnalysis,
        DiagnosticCategory::Domain,
        DiagnosticSeverity::Error,
        "required semantic feature is unsupported",
        None,
    )
    .with_metadata_value("feature", object);

    let feature = item
        .metadata_value("feature")
        .expect("feature metadata must exist");

    assert_eq!(
        feature.get("domain"),
        Some(&DiagnosticValue::Text("quantum".to_owned()))
    );
    assert_eq!(
        feature.get("portable"),
        Some(&DiagnosticValue::Boolean(true))
    );
}

#[test]
fn metadata_insertion_replaces_only_the_named_key() {
    let item = type_error(1, 1, 2)
        .with_metadata("attempt", 1_i128)
        .with_metadata("attempt", 2_i128)
        .with_metadata("other", "retained");

    assert_eq!(
        item.metadata_value("attempt"),
        Some(&DiagnosticValue::Integer(2))
    );

    assert_eq!(
        item.metadata_value("other"),
        Some(&DiagnosticValue::Text("retained".to_owned()))
    );
}

// -----------------------------------------------------------------------------
// Deterministic ordering and conservative deduplication
// -----------------------------------------------------------------------------

#[test]
fn diagnostics_sort_by_canonical_source_location() {
    let mut items = vec![
        type_error(1, 30, 35),
        type_error(1, 2, 5),
        type_error(1, 12, 16),
    ];

    sort_diagnostics(&mut items);

    let locations: Vec<(u64, u64)> = items
        .iter()
        .map(|item| {
            let location = item.primary_span().expect("source span exists");
            (location.start_raw(), location.end_raw())
        })
        .collect();

    assert_eq!(locations, vec![(2, 5), (12, 16), (30, 35)]);
}

#[test]
fn source_less_diagnostics_sort_before_source_located_diagnostics() {
    let source_less = diagnostic(
        "ZM-SEM-ANALYSIS-002",
        DiagnosticPhase::SemanticFinalization,
        DiagnosticCategory::Analysis,
        DiagnosticSeverity::Error,
        "analysis budget exhausted",
        None,
    );

    let source_located = type_error(1, 0, 1);
    let mut items = vec![source_located, source_less];

    sort_diagnostics(&mut items);

    assert!(items[0].primary_span().is_none());
    assert!(items[1].primary_span().is_some());
}

#[test]
fn ordering_is_repeatable_for_identical_inputs() {
    let original = vec![
        type_error(2, 30, 35),
        type_error(1, 12, 16).with_note("second note"),
        type_error(1, 12, 16).with_note("first note"),
        type_error(1, 2, 5),
    ];

    let mut first = original.clone();
    let mut second = original;

    sort_diagnostics(&mut first);
    sort_diagnostics(&mut second);

    assert_eq!(first, second);
}

#[test]
fn distinct_codes_phases_severities_and_locations_are_not_collapsed() {
    let base = type_error(1, 1, 2);

    let different_code = diagnostic(
        "ZM-SEM-TYPE-002",
        DiagnosticPhase::TypeAnalysis,
        DiagnosticCategory::Type,
        DiagnosticSeverity::Error,
        "incompatible types",
        Some(span(1, 1, 2)),
    );

    let different_phase = diagnostic(
        "ZM-SEM-TYPE-001",
        DiagnosticPhase::DomainAnalysis,
        DiagnosticCategory::Type,
        DiagnosticSeverity::Error,
        "incompatible types",
        Some(span(1, 1, 2)),
    );

    let different_severity = diagnostic(
        "ZM-SEM-TYPE-001",
        DiagnosticPhase::TypeAnalysis,
        DiagnosticCategory::Type,
        DiagnosticSeverity::Warning,
        "incompatible types",
        Some(span(1, 1, 2)),
    );

    let different_location = type_error(1, 8, 9);

    let mut items = vec![
        base,
        different_code,
        different_phase,
        different_severity,
        different_location,
    ];

    deduplicate_diagnostics(&mut items);

    assert_eq!(items.len(), 5);
}

#[test]
fn duplicate_reports_are_removed_without_collapsing_distinct_causes() {
    let first = type_error(1, 2, 5);
    let exact_duplicate = first.clone();
    let separate_location = type_error(1, 7, 10);

    let mut items = vec![first, exact_duplicate, separate_location];

    deduplicate_diagnostics(&mut items);

    assert_eq!(items.len(), 2);
    assert_ne!(
        items[0].primary_span(),
        items[1].primary_span()
    );
}

#[test]
fn deduplication_is_independent_of_insertion_order() {
    let first = type_error(1, 2, 5);
    let duplicate = first.clone();
    let second = type_error(1, 9, 12);
    let third = diagnostic(
        "ZM-SEM-NAME-001",
        DiagnosticPhase::NameResolution,
        DiagnosticCategory::NameResolution,
        DiagnosticSeverity::Error,
        "unresolved name",
        Some(span(1, 15, 19)),
    );

    let mut left = vec![
        first.clone(),
        second.clone(),
        duplicate.clone(),
        third.clone(),
    ];

    let mut right = vec![third, duplicate, second, first];

    deduplicate_diagnostics(&mut left);
    deduplicate_diagnostics(&mut right);

    assert_eq!(left, right);
}

#[test]
fn supplemental_details_do_not_change_report_identity() {
    let base = type_error(1, 2, 5);

    let enriched = type_error(1, 2, 5)
        .with_note("additional context")
        .with_help("review the expected type")
        .with_metadata("expected", "integer");

    assert!(base.same_report_identity(&enriched));

    let mut items = vec![base, enriched];

    deduplicate_diagnostics(&mut items);

    assert_eq!(items.len(), 1);
}

// -----------------------------------------------------------------------------
// Cross-domain diagnostic boundaries
// -----------------------------------------------------------------------------

#[test]
fn target_feasibility_is_distinct_from_source_semantic_validity() {
    let source_error = diagnostic(
        "ZM-SEM-TYPE-001",
        DiagnosticPhase::TypeAnalysis,
        DiagnosticCategory::Type,
        DiagnosticSeverity::Error,
        "source expression has incompatible types",
        Some(span(1, 4, 8)),
    );

    let target_issue = diagnostic(
        "ZM-TARGET-CAPABILITY-001",
        DiagnosticPhase::TargetPlanning,
        DiagnosticCategory::Capability,
        DiagnosticSeverity::Error,
        "selected target lacks a required capability",
        None,
    );

    assert_ne!(source_error.phase(), target_issue.phase());
    assert_ne!(source_error.category(), target_issue.category());
    assert_ne!(source_error.code(), target_issue.code());

    // This test verifies taxonomy separation. It does not claim that a
    // particular target has actually been queried or found unavailable.
}

#[test]
fn quantum_ir_and_physical_realization_failures_remain_distinct() {
    let semantic_issue = diagnostic(
        "ZM-SEM-DOMAIN-QUANTUM-001",
        DiagnosticPhase::DomainAnalysis,
        DiagnosticCategory::Domain,
        DiagnosticSeverity::Error,
        "quantum operation violates a semantic rule",
        Some(span(1, 3, 11)),
    );

    let routing_issue = diagnostic(
        "ZM-ROUTE-001",
        DiagnosticPhase::TargetPlanning,
        DiagnosticCategory::Resource,
        DiagnosticSeverity::Error,
        "physical realization could not satisfy a routing constraint",
        None,
    );

    assert_ne!(semantic_issue.phase(), routing_issue.phase());
    assert_ne!(semantic_issue.code(), routing_issue.code());
}

#[test]
fn resource_requirements_are_not_compiler_budget_failures() {
    let requirement = diagnostic(
        "ZM-SEM-RESOURCE-001",
        DiagnosticPhase::ResourceAnalysis,
        DiagnosticCategory::Resource,
        DiagnosticSeverity::Error,
        "a declared resource constraint is unsatisfied",
        Some(span(1, 0, 20)),
    );

    let budget = diagnostic(
        "ZM-SEM-ANALYSIS-002",
        DiagnosticPhase::SemanticFinalization,
        DiagnosticCategory::Analysis,
        DiagnosticSeverity::Error,
        "the configured analysis budget was exhausted",
        None,
    );

    assert_ne!(requirement.category(), budget.category());
    assert_ne!(requirement.phase(), budget.phase());
    assert_ne!(requirement.code(), budget.code());
}

#[test]
fn extension_diagnostics_do_not_require_new_core_domain_variants() {
    let extension = diagnostic(
        "EXAMPLE-SEM-TENSOR-001",
        DiagnosticPhase::Extension("tensor.shape".to_owned()),
        DiagnosticCategory::Extension("tensor.shape".to_owned()),
        DiagnosticSeverity::Error,
        "extension-defined shape constraint is invalid",
        Some(span(3, 7, 15)),
    );

    assert_eq!(
        extension.phase(),
        &DiagnosticPhase::Extension("tensor.shape".to_owned())
    );

    assert_eq!(
        extension.category(),
        &DiagnosticCategory::Extension("tensor.shape".to_owned())
    );

    assert!(extension.has_valid_code());
}

// -----------------------------------------------------------------------------
// Error recovery and presentation
// -----------------------------------------------------------------------------

#[test]
fn recoverable_diagnostics_retain_source_and_recovery_information() {
    let location = span(1, 8, 8);

    let item = diagnostic(
        "ZM-SEM-STRUCTURE-001",
        DiagnosticPhase::AstValidation,
        DiagnosticCategory::Structural,
        DiagnosticSeverity::Error,
        "required expression is missing",
        Some(location),
    )
    .with_recovery(RecoveryClassification::Recoverable)
    .with_help("insert an expression")
    .with_metadata("recovered", true);

    assert_eq!(item.primary_span(), Some(location));
    assert_eq!(item.recovery(), RecoveryClassification::Recoverable);
    assert_eq!(item.metadata_value("recovered"), Some(&DiagnosticValue::Boolean(true)));
}

#[test]
fn diagnostic_display_does_not_replace_structured_fields() {
    let item = type_error(1, 1, 4)
        .with_note("a related type constraint failed")
        .with_help("check the declared type");

    let rendered = item.to_string();

    assert!(rendered.contains("ZM-SEM-TYPE-001"));
    assert!(rendered.contains("incompatible types"));
    assert!(rendered.contains("note:"));
    assert!(rendered.contains("help:"));

    assert_eq!(item.code().as_str(), "ZM-SEM-TYPE-001");
    assert_eq!(item.message(), "incompatible types");
    assert_eq!(
        item.notes(),
        &["a related type constraint failed".to_owned()]
    );
    assert_eq!(item.help(), Some("check the declared type"));
}

// -----------------------------------------------------------------------------
// Data-driven conformance matrix
// -----------------------------------------------------------------------------

#[test]
fn diagnostic_phases_and_categories_are_explicit_for_core_semantic_failures() {
    struct Case {
        code: &'static str,
        phase: DiagnosticPhase,
        category: DiagnosticCategory,
    }

    let cases = vec![
        Case {
            code: "ZM-SEM-STRUCTURE-001",
            phase: DiagnosticPhase::AstValidation,
            category: DiagnosticCategory::Structural,
        },
        Case {
            code: "ZM-SEM-NAME-001",
            phase: DiagnosticPhase::NameResolution,
            category: DiagnosticCategory::NameResolution,
        },
        Case {
            code: "ZM-SEM-TYPE-001",
            phase: DiagnosticPhase::TypeAnalysis,
            category: DiagnosticCategory::Type,
        },
        Case {
            code: "ZM-SEM-EFFECT-001",
            phase: DiagnosticPhase::EffectAnalysis,
            category: DiagnosticCategory::Effect,
        },
        Case {
            code: "ZM-SEM-CAPABILITY-001",
            phase: DiagnosticPhase::CapabilityAnalysis,
            category: DiagnosticCategory::Capability,
        },
        Case {
            code: "ZM-SEM-RESOURCE-001",
            phase: DiagnosticPhase::ResourceAnalysis,
            category: DiagnosticCategory::Resource,
        },
        Case {
            code: "ZM-SEM-CONTRACT-001",
            phase: DiagnosticPhase::ContractAnalysis,
            category: DiagnosticCategory::Contract,
        },
        Case {
            code: "ZM-SEM-DOMAIN-001",
            phase: DiagnosticPhase::DomainAnalysis,
            category: DiagnosticCategory::Domain,
        },
        Case {
            code: "ZM-SEM-ANALYSIS-002",
            phase: DiagnosticPhase::SemanticFinalization,
            category: DiagnosticCategory::Analysis,
        },
    ];

    for case in cases {
        let item = diagnostic(
            case.code,
            case.phase.clone(),
            case.category.clone(),
            DiagnosticSeverity::Error,
            "conformance test diagnostic",
            None,
        );

        assert_eq!(item.code().as_str(), case.code);
        assert_eq!(item.phase(), &case.phase);
        assert_eq!(item.category(), &case.category);
        assert!(item.is_error());
    }
}

// -----------------------------------------------------------------------------
// Bounded stress checks for algorithmic and allocation regressions
// -----------------------------------------------------------------------------

#[test]
fn a_large_diagnostic_batch_sorts_deterministically() {
    // This is a test workload, not a language-level maximum.
    // Larger workloads belong in a dedicated stress or benchmark target.
    let count = 2_048_usize;
    let mut items = Vec::with_capacity(count);

    for index in (0..count).rev() {
        let offset = u64::try_from(index)
            .expect("the test index fits in the canonical offset");

        items.push(diagnostic(
            "ZM-SEM-NAME-001",
            DiagnosticPhase::NameResolution,
            DiagnosticCategory::NameResolution,
            DiagnosticSeverity::Error,
            "unresolved name",
            Some(span(1, offset, offset)),
        ));
    }

    sort_diagnostics(&mut items);

    assert_eq!(items.len(), count);

    let mut previous = 0_u64;

    for (index, item) in items.iter().enumerate() {
        let location = item.primary_span().expect("source span exists");

        if index > 0 {
            assert!(location.start_raw() >= previous);
        }

        previous = location.start_raw();
    }

    let snapshot = items.clone();

    sort_diagnostics(&mut items);

    assert_eq!(items, snapshot);
}

#[test]
fn a_large_diagnostic_batch_deduplicates_exact_duplicates() {
    // Keep the ordinary test finite. This tests behavior, not a global limit.
    let unique_count = 512_usize;
    let mut items = Vec::with_capacity(unique_count * 2);

    for index in 0..unique_count {
        let offset = u64::try_from(index)
            .expect("the test index fits in the canonical offset");

        let item = diagnostic(
            "ZM-SEM-NAME-001",
            DiagnosticPhase::NameResolution,
            DiagnosticCategory::NameResolution,
            DiagnosticSeverity::Error,
            "unresolved name",
            Some(span(1, offset, offset)),
        );

        items.push(item.clone());
        items.push(item);
    }

    deduplicate_diagnostics(&mut items);

    assert_eq!(items.len(), unique_count);
}

#[test]
fn no_diagnostic_is_lost_when_its_location_differs() {
    let count = 256_usize;
    let mut items = Vec::with_capacity(count);

    for index in 0..count {
        let offset = u64::try_from(index)
            .expect("the test index fits in the canonical offset");

        items.push(diagnostic(
            "ZM-SEM-NAME-001",
            DiagnosticPhase::NameResolution,
            DiagnosticCategory::NameResolution,
            DiagnosticSeverity::Error,
            "unresolved name",
            Some(span(1, offset, offset)),
        ));
    }

    deduplicate_diagnostics(&mut items);

    assert_eq!(items.len(), count);
}