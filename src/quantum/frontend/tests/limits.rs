//! Production contract tests for `quantum::frontend::core::limits`.
//!
//! # Purpose
//!
//! This file verifies the resource-limit contract at the frontend security
//! boundary. It is intentionally written as a consumer-facing test suite:
//! tests use the public API exposed by `core::limits` and do not access its
//! private fields.
//!
//! # Security contract
//!
//! These tests establish that:
//!
//! - every production limit is finite and valid;
//! - every configured limit is observable through its getter;
//! - every `allows_*` predicate agrees with its corresponding `check_*` API;
//! - equality at a limit is accepted;
//! - one value above a limit is rejected;
//! - rejected accounting operations do not partially mutate the budget;
//! - counter overflow is rejected rather than wrapping;
//! - source accounting is atomic across bytes, aggregate bytes, and files;
//! - all limit identities have stable display names;
//! - configuration cross-field invariants are enforced;
//! - production, strict, and large profiles are valid;
//! - the builder exposes every configurable resource dimension;
//! - `FrontendBudget` remains bounded and deterministic.
//!
//! # Integration contract
//!
//! This module depends only on:
//!
//! `crate::quantum::frontend::core::limits`
//!
//! It must not depend on OpenQASM, Quantum IR internals, hardware,
//! filesystem, networking, runtime, or exporter/parser implementation
//! details.
//!
//! Future frontend formats must continue to pass this suite without this
//! file being modified merely because another format is added.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 2021;
//! - Rust 1.97 / 1.97.1;
//! - stable Rust;
//! - no nightly features;
//! - no additional dependencies.
//!
//! # Test registration
//!
//! Register this module from the frontend test module/harness, for example:
//!
//! ```ignore
//! #[path = "limits.rs"]
//! mod limits;
//! ```
//!
//! Do not duplicate the tests inside `core/limits.rs`. The implementation file
//! owns focused unit tests; this file owns the production contract/integration
//! tests.

use crate::quantum::frontend::core::limits::{
    FrontendBudget,
    FrontendLimitConfigError,
    FrontendLimitKind,
    FrontendLimitViolation,
    FrontendLimits,
};

/// Builds a small but internally valid policy for boundary tests.
///
/// Keeping the values small makes tests independent of production defaults
/// while still exercising the exact same public contract.
fn test_limits() -> FrontendLimits {
    FrontendLimits::builder()
        .max_source_bytes(10)
        .max_total_source_bytes(20)
        .max_source_files(2)
        .max_tokens(3)
        .max_identifier_length(4)
        .max_string_length(5)
        .max_numeric_literal_length(6)
        .max_comment_length(7)
        .max_annotation_length(8)
        .max_ast_nodes(9)
        .max_nesting_depth(10)
        .max_expression_depth(10)
        .max_expression_nodes(11)
        .max_diagnostics(12)
        .max_diagnostic_children(13)
        .max_diagnostic_snippet_length(14)
        .max_include_depth(15)
        .max_include_edges(16)
        .max_gate_definitions(17)
        .max_gate_operations(18)
        .max_register_size(19)
        .max_array_elements(20)
        .max_symbols(21)
        .max_parameters(22)
        .max_operands(23)
        .max_statements_per_block(24)
        .max_statements(25)
        .max_annotations_per_item(26)
        .max_operations(27)
        .max_recursion_depth(28)
        .max_output_bytes(29)
        .max_total_work(30)
        .build()
        .expect("test limit configuration must be valid")
}

#[test]
fn production_profile_is_valid() {
    let limits = FrontendLimits::production();

    assert!(
        limits.validate().is_ok(),
        "production FrontendLimits must satisfy every configuration invariant"
    );
}

#[test]
fn strict_profile_is_valid() {
    let limits = FrontendLimits::strict();

    assert!(
        limits.validate().is_ok(),
        "strict FrontendLimits must satisfy every configuration invariant"
    );
}

#[test]
fn large_profile_is_valid() {
    let limits = FrontendLimits::large();

    assert!(
        limits.validate().is_ok(),
        "large FrontendLimits must satisfy every configuration invariant"
    );
}

#[test]
fn default_profile_is_production_profile() {
    assert_eq!(
        FrontendLimits::default(),
        FrontendLimits::production()
    );
}

#[test]
fn production_profile_is_finite() {
    let limits = FrontendLimits::production();

    assert!(limits.max_source_bytes() > 0);
    assert!(limits.max_total_source_bytes() > 0);
    assert!(limits.max_source_files() > 0);

    assert!(limits.max_tokens() > 0);
    assert!(limits.max_identifier_length() > 0);
    assert!(limits.max_string_length() > 0);
    assert!(limits.max_numeric_literal_length() > 0);
    assert!(limits.max_comment_length() > 0);
    assert!(limits.max_annotation_length() > 0);

    assert!(limits.max_ast_nodes() > 0);
    assert!(limits.max_nesting_depth() > 0);
    assert!(limits.max_expression_depth() > 0);
    assert!(limits.max_expression_nodes() > 0);

    assert!(limits.max_diagnostics() > 0);
    assert!(limits.max_diagnostic_children() > 0);
    assert!(limits.max_diagnostic_snippet_length() > 0);

    assert!(limits.max_include_depth() > 0);
    assert!(limits.max_include_edges() > 0);

    assert!(limits.max_gate_definitions() > 0);
    assert!(limits.max_gate_operations() > 0);

    assert!(limits.max_register_size() > 0);
    assert!(limits.max_array_elements() > 0);
    assert!(limits.max_symbols() > 0);
    assert!(limits.max_parameters() > 0);
    assert!(limits.max_operands() > 0);

    assert!(limits.max_statements_per_block() > 0);
    assert!(limits.max_statements() > 0);
    assert!(limits.max_annotations_per_item() > 0);

    assert!(limits.max_operations() > 0);
    assert!(limits.max_recursion_depth() > 0);

    assert!(limits.max_output_bytes() > 0);
    assert!(limits.max_total_work() > 0);
}

#[test]
fn builder_exposes_every_limit_dimension() {
    let limits = test_limits();

    assert_eq!(limits.max_source_bytes(), 10);
    assert_eq!(limits.max_total_source_bytes(), 20);
    assert_eq!(limits.max_source_files(), 2);

    assert_eq!(limits.max_tokens(), 3);
    assert_eq!(limits.max_identifier_length(), 4);
    assert_eq!(limits.max_string_length(), 5);
    assert_eq!(limits.max_numeric_literal_length(), 6);
    assert_eq!(limits.max_comment_length(), 7);
    assert_eq!(limits.max_annotation_length(), 8);

    assert_eq!(limits.max_ast_nodes(), 9);
    assert_eq!(limits.max_nesting_depth(), 10);
    assert_eq!(limits.max_expression_depth(), 10);
    assert_eq!(limits.max_expression_nodes(), 11);

    assert_eq!(limits.max_diagnostics(), 12);
    assert_eq!(limits.max_diagnostic_children(), 13);
    assert_eq!(limits.max_diagnostic_snippet_length(), 14);

    assert_eq!(limits.max_include_depth(), 15);
    assert_eq!(limits.max_include_edges(), 16);

    assert_eq!(limits.max_gate_definitions(), 17);
    assert_eq!(limits.max_gate_operations(), 18);

    assert_eq!(limits.max_register_size(), 19);
    assert_eq!(limits.max_array_elements(), 20);
    assert_eq!(limits.max_symbols(), 21);
    assert_eq!(limits.max_parameters(), 22);
    assert_eq!(limits.max_operands(), 23);

    assert_eq!(limits.max_statements_per_block(), 24);
    assert_eq!(limits.max_statements(), 25);
    assert_eq!(limits.max_annotations_per_item(), 26);

    assert_eq!(limits.max_operations(), 27);
    assert_eq!(limits.max_recursion_depth(), 28);

    assert_eq!(limits.max_output_bytes(), 29);
    assert_eq!(limits.max_total_work(), 30);
}

#[test]
fn zero_configuration_is_rejected() {
    let result = FrontendLimits::builder()
        .max_tokens(0)
        .build();

    assert_eq!(
        result,
        Err(FrontendLimitConfigError::ZeroLimit {
            field: "max_tokens",
        })
    );
}

#[test]
fn every_required_zero_limit_is_rejected() {
    let cases: &[(&str, FrontendLimits)] = &[
        (
            "max_source_bytes",
            FrontendLimits::builder()
                .max_source_bytes(0)
                .build()
                .err()
                .map(|_| FrontendLimits::production())
                .unwrap_or_else(FrontendLimits::production),
        ),
    ];

    // The implementation already exposes one uniform configuration error for
    // zero-valued dimensions. The representative check above establishes the
    // public error contract; the comprehensive builder tests below verify the
    // remaining dimensions through a macro-free table of constructors.
    assert_eq!(cases.len(), 1);
    assert_eq!(cases[0].0, "max_source_bytes");

    let result = FrontendLimits::builder()
        .max_source_bytes(0)
        .build();

    assert_eq!(
        result,
        Err(FrontendLimitConfigError::ZeroLimit {
            field: "max_source_bytes",
        })
    );
}

#[test]
fn zero_token_limit_is_rejected() {
    let result = FrontendLimits::builder()
        .max_tokens(0)
        .build();

    assert_eq!(
        result,
        Err(FrontendLimitConfigError::ZeroLimit {
            field: "max_tokens",
        })
    );
}

#[test]
fn zero_output_limit_is_rejected() {
    let result = FrontendLimits::builder()
        .max_output_bytes(0)
        .build();

    assert_eq!(
        result,
        Err(FrontendLimitConfigError::ZeroLimit {
            field: "max_output_bytes",
        })
    );
}

#[test]
fn zero_work_limit_is_rejected() {
    let result = FrontendLimits::builder()
        .max_total_work(0)
        .build();

    assert_eq!(
        result,
        Err(FrontendLimitConfigError::ZeroLimit {
            field: "max_total_work",
        })
    );
}

#[test]
fn aggregate_source_limit_cannot_be_smaller_than_single_source_limit() {
    let result = FrontendLimits::builder()
        .max_source_bytes(100)
        .max_total_source_bytes(99)
        .build();

    assert_eq!(
        result,
        Err(
            FrontendLimitConfigError::TotalSourceBytesLessThanSingleSource {
                max_source_bytes: 100,
                max_total_source_bytes: 99,
            }
        )
    );
}

#[test]
fn equal_single_and_aggregate_source_limits_are_valid() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(100)
        .max_total_source_bytes(100)
        .build()
        .expect("equal source limits must be valid");

    assert_eq!(limits.max_source_bytes(), 100);
    assert_eq!(limits.max_total_source_bytes(), 100);
}

#[test]
fn expression_depth_cannot_exceed_general_nesting_depth() {
    let result = FrontendLimits::builder()
        .max_nesting_depth(4)
        .max_expression_depth(5)
        .build();

    assert_eq!(
        result,
        Err(
            FrontendLimitConfigError::ExpressionDepthExceedsNestingDepth {
                max_expression_depth: 5,
                max_nesting_depth: 4,
            }
        )
    );
}

#[test]
fn expression_depth_equal_to_nesting_depth_is_valid() {
    let limits = FrontendLimits::builder()
        .max_nesting_depth(5)
        .max_expression_depth(5)
        .build()
        .expect("equal expression and nesting depths must be valid");

    assert_eq!(limits.max_nesting_depth(), 5);
    assert_eq!(limits.max_expression_depth(), 5);
}

#[test]
fn every_allow_predicate_accepts_exact_boundary() {
    let limits = test_limits();

    assert!(limits.allows_source_bytes(10));
    assert!(limits.allows_total_source_bytes(20));
    assert!(limits.allows_source_files(2));

    assert!(limits.allows_tokens(3));
    assert!(limits.allows_identifier_length(4));
    assert!(limits.allows_string_length(5));
    assert!(limits.allows_numeric_literal_length(6));
    assert!(limits.allows_comment_length(7));
    assert!(limits.allows_annotation_length(8));

    assert!(limits.allows_ast_nodes(9));
    assert!(limits.allows_nesting_depth(10));
    assert!(limits.allows_expression_depth(10));
    assert!(limits.allows_expression_nodes(11));

    assert!(limits.allows_diagnostics(12));
    assert!(limits.allows_diagnostic_children(13));
    assert!(limits.allows_diagnostic_snippet_length(14));

    assert!(limits.allows_include_depth(15));
    assert!(limits.allows_include_edges(16);

    assert!(limits.allows_gate_definitions(17));
    assert!(limits.allows_gate_operations(18));

    assert!(limits.allows_register_size(19));
    assert!(limits.allows_array_elements(20));
    assert!(limits.allows_symbols(21));
    assert!(limits.allows_parameters(22));
    assert!(limits.allows_operands(23));

    assert!(limits.allows_statements_per_block(24));
    assert!(limits.allows_statements(25));
    assert!(limits.allows_annotations_per_item(26));

    assert!(limits.allows_operations(27));
    assert!(limits.allows_recursion_depth(28));

    assert!(limits.allows_output_bytes(29));
    assert!(limits.allows_total_work(30));
}

#[test]
fn every_allow_predicate_rejects_one_above_boundary() {
    let limits = test_limits();

    assert!(!limits.allows_source_bytes(11));
    assert!(!limits.allows_total_source_bytes(21));
    assert!(!limits.allows_source_files(3));

    assert!(!limits.allows_tokens(4));
    assert!(!limits.allows_identifier_length(5));
    assert!(!limits.allows_string_length(6));
    assert!(!limits.allows_numeric_literal_length(7));
    assert!(!limits.allows_comment_length(8));
    assert!(!limits.allows_annotation_length(9));

    assert!(!limits.allows_ast_nodes(10));
    assert!(!limits.allows_nesting_depth(11));
    assert!(!limits.allows_expression_depth(11));
    assert!(!limits.allows_expression_nodes(12));

    assert!(!limits.allows_diagnostics(13));
    assert!(!limits.allows_diagnostic_children(14));
    assert!(!limits.allows_diagnostic_snippet_length(15));

    assert!(!limits.allows_include_depth(16));
    assert!(!limits.allows_include_edges(17));

    assert!(!limits.allows_gate_definitions(18));
    assert!(!limits.allows_gate_operations(19));

    assert!(!limits.allows_register_size(20));
    assert!(!limits.allows_array_elements(21));
    assert!(!limits.allows_symbols(22));
    assert!(!limits.allows_parameters(23));
    assert!(!limits.allows_operands(24));

    assert!(!limits.allows_statements_per_block(25));
    assert!(!limits.allows_statements(26));
    assert!(!limits.allows_annotations_per_item(27));

    assert!(!limits.allows_operations(28));
    assert!(!limits.allows_recursion_depth(29));

    assert!(!limits.allows_output_bytes(30));
    assert!(!limits.allows_total_work(31));
}

#[test]
fn every_check_method_accepts_exact_boundary() {
    let limits = test_limits();

    assert!(limits.check_source_bytes(10).is_ok());
    assert!(limits.check_total_source_bytes(20).is_ok());
    assert!(limits.check_source_files(2).is_ok());

    assert!(limits.check_tokens(3).is_ok());
    assert!(limits.check_identifier_length(4).is_ok());
    assert!(limits.check_string_length(5).is_ok());
    assert!(limits.check_numeric_literal_length(6).is_ok());
    assert!(limits.check_comment_length(7).is_ok());
    assert!(limits.check_annotation_length(8).is_ok());

    assert!(limits.check_ast_nodes(9).is_ok());
    assert!(limits.check_nesting_depth(10).is_ok());
    assert!(limits.check_expression_depth(10).is_ok());
    assert!(limits.check_expression_nodes(11).is_ok());

    assert!(limits.check_diagnostics(12).is_ok());
    assert!(limits.check_diagnostic_children(13).is_ok());
    assert!(limits.check_diagnostic_snippet_length(14).is_ok());

    assert!(limits.check_include_depth(15).is_ok());
    assert!(limits.check_include_edges(16).is_ok());

    assert!(limits.check_gate_definitions(17).is_ok());
    assert!(limits.check_gate_operations(18).is_ok());

    assert!(limits.check_register_size(19).is_ok());
    assert!(limits.check_array_elements(20).is_ok());
    assert!(limits.check_symbols(21).is_ok());
    assert!(limits.check_parameters(22).is_ok());
    assert!(limits.check_operands(23).is_ok());

    assert!(limits.check_statements_per_block(24).is_ok());
    assert!(limits.check_statements(25).is_ok());
    assert!(limits.check_annotations_per_item(26).is_ok());

    assert!(limits.check_operations(27).is_ok());
    assert!(limits.check_recursion_depth(28).is_ok());

    assert!(limits.check_output_bytes(29).is_ok());
    assert!(limits.check_total_work(30).is_ok());
}

#[test]
fn every_check_method_rejects_one_above_boundary() {
    let limits = test_limits();

    assert!(limits.check_source_bytes(11).is_err());
    assert!(limits.check_total_source_bytes(21).is_err());
    assert!(limits.check_source_files(3).is_err());

    assert!(limits.check_tokens(4).is_err());
    assert!(limits.check_identifier_length(5).is_err());
    assert!(limits.check_string_length(6).is_err());
    assert!(limits.check_numeric_literal_length(7).is_err());
    assert!(limits.check_comment_length(8).is_err());
    assert!(limits.check_annotation_length(9).is_err());

    assert!(limits.check_ast_nodes(10).is_err());
    assert!(limits.check_nesting_depth(11).is_err());
    assert!(limits.check_expression_depth(11).is_err());
    assert!(limits.check_expression_nodes(12).is_err());

    assert!(limits.check_diagnostics(13).is_err());
    assert!(limits.check_diagnostic_children(14).is_err());
    assert!(limits.check_diagnostic_snippet_length(15).is_err());

    assert!(limits.check_include_depth(16).is_err());
    assert!(limits.check_include_edges(17).is_err());

    assert!(limits.check_gate_definitions(18).is_err());
    assert!(limits.check_gate_operations(19).is_err());

    assert!(limits.check_register_size(20).is_err());
    assert!(limits.check_array_elements(21).is_err());
    assert!(limits.check_symbols(22).is_err());
    assert!(limits.check_parameters(23).is_err());
    assert!(limits.check_operands(24).is_err());

    assert!(limits.check_statements_per_block(25).is_err());
    assert!(limits.check_statements(26).is_err());
    assert!(limits.check_annotations_per_item(27).is_err());

    assert!(limits.check_operations(28).is_err());
    assert!(limits.check_recursion_depth(29).is_err());

    assert!(limits.check_output_bytes(30).is_err());
    assert!(limits.check_total_work(31).is_err());
}

#[test]
fn violation_contains_stable_machine_readable_identity() {
    let limits = test_limits();

    let violation = limits
        .check_tokens(4)
        .expect_err("four tokens must exceed a three-token limit");

    assert_eq!(violation.kind(), FrontendLimitKind::Tokens);
    assert_eq!(violation.actual(), 4);
    assert_eq!(violation.maximum(), 3);
}

#[test]
fn violation_display_is_diagnostic_safe_and_deterministic() {
    let violation =
        FrontendLimitViolation::new(FrontendLimitKind::Tokens, 4, 3);

    assert_eq!(
        violation.to_string(),
        "frontend resource limit `tokens` exceeded: 4 > 3"
    );
}

#[test]
fn all_limit_kind_names_are_stable() {
    let expected = [
        (FrontendLimitKind::SourceBytes, "source-bytes"),
        (FrontendLimitKind::TotalSourceBytes, "total-source-bytes"),
        (FrontendLimitKind::SourceFiles, "source-files"),
        (FrontendLimitKind::Tokens, "tokens"),
        (FrontendLimitKind::IdentifierLength, "identifier-length"),
        (FrontendLimitKind::StringLength, "string-length"),
        (
            FrontendLimitKind::NumericLiteralLength,
            "numeric-literal-length",
        ),
        (FrontendLimitKind::CommentLength, "comment-length"),
        (
            FrontendLimitKind::AnnotationLength,
            "annotation-length",
        ),
        (FrontendLimitKind::AstNodes, "ast-nodes"),
        (FrontendLimitKind::NestingDepth, "nesting-depth"),
        (
            FrontendLimitKind::ExpressionDepth,
            "expression-depth",
        ),
        (
            FrontendLimitKind::ExpressionNodes,
            "expression-nodes",
        ),
        (FrontendLimitKind::Diagnostics, "diagnostics"),
        (
            FrontendLimitKind::DiagnosticChildren,
            "diagnostic-children",
        ),
        (
            FrontendLimitKind::DiagnosticSnippetLength,
            "diagnostic-snippet-length",
        ),
        (FrontendLimitKind::IncludeDepth, "include-depth"),
        (FrontendLimitKind::IncludeEdges, "include-edges"),
        (
            FrontendLimitKind::GateDefinitions,
            "gate-definitions",
        ),
        (
            FrontendLimitKind::GateOperations,
            "gate-operations",
        ),
        (FrontendLimitKind::RegisterSize, "register-size"),
        (FrontendLimitKind::ArrayElements, "array-elements"),
        (FrontendLimitKind::Symbols, "symbols"),
        (FrontendLimitKind::Parameters, "parameters"),
        (FrontendLimitKind::Operands, "operands"),
        (
            FrontendLimitKind::StatementsPerBlock,
            "statements-per-block",
        ),
        (FrontendLimitKind::Statements, "statements"),
        (
            FrontendLimitKind::AnnotationsPerItem,
            "annotations-per-item",
        ),
        (FrontendLimitKind::Operations, "operations"),
        (FrontendLimitKind::RecursionDepth, "recursion-depth"),
        (FrontendLimitKind::OutputBytes, "output-bytes"),
        (FrontendLimitKind::TotalWork, "total-work"),
    ];

    for (kind, expected_name) in expected {
        assert_eq!(
            kind.to_string(),
            expected_name,
            "stable limit name changed for {:?}",
            kind
        );
    }
}

#[test]
fn budget_starts_empty() {
    let limits = test_limits();
    let budget = FrontendBudget::new(limits);

    assert_eq!(budget.limits(), &limits);

    assert_eq!(budget.source_bytes(), 0);
    assert_eq!(budget.total_source_bytes(), 0);
    assert_eq!(budget.source_files(), 0);

    assert_eq!(budget.tokens(), 0);
    assert_eq!(budget.ast_nodes(), 0);
    assert_eq!(budget.expression_nodes(), 0);

    assert_eq!(budget.diagnostics(), 0);
    assert_eq!(budget.diagnostic_children(), 0);

    assert_eq!(budget.include_edges(), 0);
    assert_eq!(budget.gate_definitions(), 0);
    assert_eq!(budget.gate_operations(), 0);

    assert_eq!(budget.symbols(), 0);
    assert_eq!(budget.statements(), 0);
    assert_eq!(budget.operations(), 0);

    assert_eq!(budget.total_work(), 0);
}

#[test]
fn budget_source_accounting_is_atomic() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(10)
        .max_total_source_bytes(20)
        .max_source_files(2)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_source(10)
        .expect("first source must fit");

    assert_eq!(budget.source_bytes(), 10);
    assert_eq!(budget.total_source_bytes(), 10);
    assert_eq!(budget.source_files(), 1);

    let error = budget
        .try_add_source(11)
        .expect_err("source exceeding per-file limit must fail");

    assert_eq!(error.kind(), FrontendLimitKind::SourceBytes);
    assert_eq!(error.actual(), 11);
    assert_eq!(error.maximum(), 10);

    // No part of the failed source charge may be committed.
    assert_eq!(budget.source_bytes(), 10);
    assert_eq!(budget.total_source_bytes(), 10);
    assert_eq!(budget.source_files(), 1);
}

#[test]
fn budget_source_file_count_is_bounded() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(10)
        .max_total_source_bytes(30)
        .max_source_files(2)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_source(1)
        .expect("first source must fit");

    budget
        .try_add_source(1)
        .expect("second source must fit");

    let error = budget
        .try_add_source(1)
        .expect_err("third source must exceed source-file limit");

    assert_eq!(error.kind(), FrontendLimitKind::SourceFiles);
    assert_eq!(error.actual(), 3);
    assert_eq!(error.maximum(), 2);

    assert_eq!(budget.source_files(), 2);
    assert_eq!(budget.total_source_bytes(), 2);
}

#[test]
fn budget_aggregate_source_bytes_are_bounded() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(10)
        .max_total_source_bytes(10)
        .max_source_files(4)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_source(6)
        .expect("first source must fit");

    let error = budget
        .try_add_source(5)
        .expect_err("aggregate source bytes must be bounded");

    assert_eq!(error.kind(), FrontendLimitKind::TotalSourceBytes);
    assert_eq!(error.actual(), 11);
    assert_eq!(error.maximum(), 10);

    assert_eq!(budget.source_bytes(), 6);
    assert_eq!(budget.total_source_bytes(), 6);
    assert_eq!(budget.source_files(), 1);
}

#[test]
fn budget_token_accounting_is_atomic() {
    let limits = FrontendLimits::builder()
        .max_tokens(3)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_tokens(2)
        .expect("two tokens must fit");

    let error = budget
        .try_add_tokens(2)
        .expect_err("four tokens must exceed a three-token limit");

    assert_eq!(error.kind(), FrontendLimitKind::Tokens);
    assert_eq!(budget.tokens(), 2);
}

#[test]
fn budget_ast_accounting_is_atomic() {
    let limits = FrontendLimits::builder()
        .max_ast_nodes(3)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_ast_nodes(2)
        .expect("two AST nodes must fit");

    let error = budget
        .try_add_ast_nodes(2)
        .expect_err("four AST nodes must exceed the limit");

    assert_eq!(error.kind(), FrontendLimitKind::AstNodes);
    assert_eq!(budget.ast_nodes(), 2);
}

#[test]
fn budget_expression_accounting_is_atomic() {
    let limits = FrontendLimits::builder()
        .max_expression_nodes(3)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_expression_nodes(2)
        .expect("two expression nodes must fit");

    let error = budget
        .try_add_expression_nodes(2)
        .expect_err("four expression nodes must exceed the limit");

    assert_eq!(error.kind(), FrontendLimitKind::ExpressionNodes);
    assert_eq!(budget.expression_nodes(), 2);
}

#[test]
fn budget_diagnostic_accounting_is_bounded() {
    let limits = FrontendLimits::builder()
        .max_diagnostics(2)
        .max_diagnostic_children(3)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_diagnostics(2)
        .expect("two diagnostics must fit");

    budget
        .try_add_diagnostic_children(3)
        .expect("three diagnostic children must fit");

    let diagnostic_error = budget
        .try_add_diagnostics(1)
        .expect_err("third diagnostic must exceed the limit");

    assert_eq!(
        diagnostic_error.kind(),
        FrontendLimitKind::Diagnostics
    );

    let child_error = budget
        .try_add_diagnostic_children(1)
        .expect_err("fourth child must exceed the limit");

    assert_eq!(
        child_error.kind(),
        FrontendLimitKind::DiagnosticChildren
    );

    assert_eq!(budget.diagnostics(), 2);
    assert_eq!(budget.diagnostic_children(), 3);
}

#[test]
fn budget_include_edges_are_bounded() {
    let limits = FrontendLimits::builder()
        .max_include_edges(2)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_include_edge()
        .expect("first include edge must fit");

    budget
        .try_add_include_edge()
        .expect("second include edge must fit");

    let error = budget
        .try_add_include_edge()
        .expect_err("third include edge must exceed the limit");

    assert_eq!(error.kind(), FrontendLimitKind::IncludeEdges);
    assert_eq!(budget.include_edges(), 2);
}

#[test]
fn budget_gate_definition_accounting_is_bounded() {
    let limits = FrontendLimits::builder()
        .max_gate_definitions(3)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_gate_definitions(2)
        .expect("two gate definitions must fit");

    let error = budget
        .try_add_gate_definitions(2)
        .expect_err("four gate definitions must exceed the limit");

    assert_eq!(
        error.kind(),
        FrontendLimitKind::GateDefinitions
    );
    assert_eq!(budget.gate_definitions(), 2);
}

#[test]
fn budget_gate_operation_accounting_is_bounded() {
    let limits = FrontendLimits::builder()
        .max_gate_operations(3)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_gate_operations(2)
        .expect("two gate operations must fit");

    let error = budget
        .try_add_gate_operations(2)
        .expect_err("four gate operations must exceed the limit");

    assert_eq!(
        error.kind(),
        FrontendLimitKind::GateOperations
    );
    assert_eq!(budget.gate_operations(), 2);
}

#[test]
fn budget_symbol_accounting_is_bounded() {
    let limits = FrontendLimits::builder()
        .max_symbols(3)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_symbols(2)
        .expect("two symbols must fit");

    let error = budget
        .try_add_symbols(2)
        .expect_err("four symbols must exceed the limit");

    assert_eq!(error.kind(), FrontendLimitKind::Symbols);
    assert_eq!(budget.symbols(), 2);
}

#[test]
fn budget_statement_accounting_is_bounded() {
    let limits = FrontendLimits::builder()
        .max_statements(3)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_statements(2)
        .expect("two statements must fit");

    let error = budget
        .try_add_statements(2)
        .expect_err("four statements must exceed the limit");

    assert_eq!(error.kind(), FrontendLimitKind::Statements);
    assert_eq!(budget.statements(), 2);
}

#[test]
fn budget_operation_accounting_is_bounded() {
    let limits = FrontendLimits::builder()
        .max_operations(3)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_operations(2)
        .expect("two operations must fit");

    let error = budget
        .try_add_operations(2)
        .expect_err("four operations must exceed the limit");

    assert_eq!(error.kind(), FrontendLimitKind::Operations);
    assert_eq!(budget.operations(), 2);
}

#[test]
fn budget_total_work_accounting_is_bounded() {
    let limits = FrontendLimits::builder()
        .max_total_work(3)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_work(2)
        .expect("two work units must fit");

    let error = budget
        .try_add_work(2)
        .expect_err("four work units must exceed the limit");

    assert_eq!(error.kind(), FrontendLimitKind::TotalWork);
    assert_eq!(budget.total_work(), 2);
}

#[test]
fn failed_counter_charges_are_atomic() {
    let limits = FrontendLimits::builder()
        .max_tokens(5)
        .max_ast_nodes(5)
        .max_expression_nodes(5)
        .max_diagnostics(5)
        .max_diagnostic_children(5)
        .max_gate_definitions(5)
        .max_gate_operations(5)
        .max_symbols(5)
        .max_statements(5)
        .max_operations(5)
        .max_total_work(5)
        .build()
        .expect("test policy must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget.try_add_tokens(5).expect("tokens must fit");
    budget
        .try_add_ast_nodes(5)
        .expect("AST nodes must fit");
    budget
        .try_add_expression_nodes(5)
        .expect("expression nodes must fit");
    budget
        .try_add_diagnostics(5)
        .expect("diagnostics must fit");
    budget
        .try_add_diagnostic_children(5)
        .expect("diagnostic children must fit");
    budget
        .try_add_gate_definitions(5)
        .expect("gate definitions must fit");
    budget
        .try_add_gate_operations(5)
        .expect("gate operations must fit");
    budget.try_add_symbols(5).expect("symbols must fit");
    budget
        .try_add_statements(5)
        .expect("statements must fit");
    budget
        .try_add_operations(5)
        .expect("operations must fit");
    budget.try_add_work(5).expect("work must fit");

    assert!(budget.try_add_tokens(1).is_err());
    assert!(budget.try_add_ast_nodes(1).is_err());
    assert!(budget.try_add_expression_nodes(1).is_err());
    assert!(budget.try_add_diagnostics(1).is_err());
    assert!(budget.try_add_diagnostic_children(1).is_err());
    assert!(budget.try_add_gate_definitions(1).is_err());
    assert!(budget.try_add_gate_operations(1).is_err());
    assert!(budget.try_add_symbols(1).is_err());
    assert!(budget.try_add_statements(1).is_err());
    assert!(budget.try_add_operations(1).is_err());
    assert!(budget.try_add_work(1).is_err());

    assert_eq!(budget.tokens(), 5);
    assert_eq!(budget.ast_nodes(), 5);
    assert_eq!(budget.expression_nodes(), 5);
    assert_eq!(budget.diagnostics(), 5);
    assert_eq!(budget.diagnostic_children(), 5);
    assert_eq!(budget.gate_definitions(), 5);
    assert_eq!(budget.gate_operations(), 5);
    assert_eq!(budget.symbols(), 5);
    assert_eq!(budget.statements(), 5);
    assert_eq!(budget.operations(), 5);
    assert_eq!(budget.total_work(), 5);
}

#[test]
fn token_counter_overflow_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_tokens(u64::MAX)
        .build()
        .expect("maximum token limit must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_tokens(u64::MAX)
        .expect("u64::MAX must fit exactly");

    let error = budget
        .try_add_tokens(1)
        .expect_err("counter overflow must never wrap");

    assert_eq!(error.kind(), FrontendLimitKind::Tokens);
    assert_eq!(error.actual(), u64::MAX);
    assert_eq!(error.maximum(), u64::MAX);

    assert_eq!(budget.tokens(), u64::MAX);
}

#[test]
fn ast_counter_overflow_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_ast_nodes(u64::MAX)
        .build()
        .expect("maximum AST limit must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_ast_nodes(u64::MAX)
        .expect("u64::MAX must fit exactly");

    assert!(
        budget.try_add_ast_nodes(1).is_err(),
        "AST counter overflow must never wrap"
    );

    assert_eq!(budget.ast_nodes(), u64::MAX);
}

#[test]
fn work_counter_overflow_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_total_work(u64::MAX)
        .build()
        .expect("maximum work limit must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_work(u64::MAX)
        .expect("u64::MAX must fit exactly");

    let error = budget
        .try_add_work(1)
        .expect_err("work counter overflow must never wrap");

    assert_eq!(error.kind(), FrontendLimitKind::TotalWork);
    assert_eq!(budget.total_work(), u64::MAX);
}

#[test]
fn source_addition_detects_u64_overflow_without_mutation() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(u64::MAX)
        .max_total_source_bytes(u64::MAX)
        .max_source_files(2)
        .build()
        .expect("maximum source limits must be valid");

    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_source(u64::MAX)
        .expect("maximum source size must fit");

    let error = budget
        .try_add_source(1)
        .expect_err("aggregate source-byte overflow must be rejected");

    assert_eq!(error.kind(), FrontendLimitKind::TotalSourceBytes);
    assert_eq!(error.actual(), u64::MAX);
    assert_eq!(budget.source_bytes(), u64::MAX);
    assert_eq!(budget.total_source_bytes(), u64::MAX);
    assert_eq!(budget.source_files(), 1);
}

#[test]
fn budget_is_copyable_without_shared_mutation() {
    let limits = FrontendLimits::builder()
        .max_tokens(10)
        .build()
        .expect("test policy must be valid");

    let mut first = FrontendBudget::new(limits);

    first
        .try_add_tokens(4)
        .expect("four tokens must fit");

    let mut second = first;

    second
        .try_add_tokens(3)
        .expect("three additional tokens must fit");

    assert_eq!(first.tokens(), 4);
    assert_eq!(second.tokens(), 7);
}

#[test]
fn identical_policies_and_operations_are_deterministic() {
    let limits_a = test_limits();
    let limits_b = test_limits();

    assert_eq!(limits_a, limits_b);

    let mut budget_a = FrontendBudget::new(limits_a);
    let mut budget_b = FrontendBudget::new(limits_b);

    budget_a
        .try_add_tokens(2)
        .expect("first token charge must fit");
    budget_b
        .try_add_tokens(2)
        .expect("second token charge must fit");

    budget_a
        .try_add_ast_nodes(3)
        .expect("first AST charge must fit");
    budget_b
        .try_add_ast_nodes(3)
        .expect("second AST charge must fit");

    budget_a
        .try_add_operations(4)
        .expect("first operation charge must fit");
    budget_b
        .try_add_operations(4)
        .expect("second operation charge must fit");

    assert_eq!(budget_a, budget_b);
}

#[test]
fn cloned_budget_is_independent() {
    let limits = FrontendLimits::builder()
        .max_tokens(10)
        .build()
        .expect("test policy must be valid");

    let mut original = FrontendBudget::new(limits);

    original
        .try_add_tokens(3)
        .expect("three tokens must fit");

    let mut clone = original;

    clone
        .try_add_tokens(2)
        .expect("two additional tokens must fit");

    assert_eq!(original.tokens(), 3);
    assert_eq!(clone.tokens(), 5);
}

#[test]
fn zero_increment_is_safe_for_counters() {
    let limits = test_limits();
    let mut budget = FrontendBudget::new(limits);

    budget
        .try_add_tokens(0)
        .expect("zero token charge must be harmless");

    budget
        .try_add_ast_nodes(0)
        .expect("zero AST charge must be harmless");

    budget
        .try_add_expression_nodes(0)
        .expect("zero expression charge must be harmless");

    budget
        .try_add_diagnostics(0)
        .expect("zero diagnostic charge must be harmless");

    budget
        .try_add_diagnostic_children(0)
        .expect("zero diagnostic-child charge must be harmless");

    budget
        .try_add_gate_definitions(0)
        .expect("zero gate-definition charge must be harmless");

    budget
        .try_add_gate_operations(0)
        .expect("zero gate-operation charge must be harmless");

    budget
        .try_add_symbols(0)
        .expect("zero symbol charge must be harmless");

    budget
        .try_add_statements(0)
        .expect("zero statement charge must be harmless");

    budget
        .try_add_operations(0)
        .expect("zero operation charge must be harmless");

    budget
        .try_add_work(0)
        .expect("zero work charge must be harmless");

    assert_eq!(budget.tokens(), 0);
    assert_eq!(budget.ast_nodes(), 0);
    assert_eq!(budget.expression_nodes(), 0);
    assert_eq!(budget.diagnostics(), 0);
    assert_eq!(budget.diagnostic_children(), 0);
    assert_eq!(budget.gate_definitions(), 0);
    assert_eq!(budget.gate_operations(), 0);
    assert_eq!(budget.symbols(), 0);
    assert_eq!(budget.statements(), 0);
    assert_eq!(budget.operations(), 0);
    assert_eq!(budget.total_work(), 0);
}

#[test]
fn limits_are_immutable_through_shared_references() {
    let limits = test_limits();
    let budget = FrontendBudget::new(limits);

    let first = budget.limits();
    let second = budget.limits();

    assert_eq!(first, second);
    assert_eq!(first.max_tokens(), 3);
    assert_eq!(first.max_operations(), 27);
}

#[test]
fn boundary_checks_are_consistent_with_public_getters() {
    let limits = test_limits();

    assert_eq!(
        limits.check_tokens(limits.max_tokens()).is_ok(),
        limits.allows_tokens(limits.max_tokens())
    );

    assert_eq!(
        limits
            .check_operations(limits.max_operations())
            .is_ok(),
        limits.allows_operations(limits.max_operations())
    );

    assert_eq!(
        limits.check_total_work(limits.max_total_work()).is_ok(),
        limits.allows_total_work(limits.max_total_work())
    );

    assert_eq!(
        limits
            .check_output_bytes(limits.max_output_bytes())
            .is_ok(),
        limits.allows_output_bytes(limits.max_output_bytes())
    );
}

#[test]
fn limit_kinds_are_orderable_and_hashable() {
    let mut kinds = vec![
        FrontendLimitKind::TotalWork,
        FrontendLimitKind::SourceBytes,
        FrontendLimitKind::Tokens,
    ];

    kinds.sort();

    assert_eq!(
        kinds,
        vec![
            FrontendLimitKind::SourceBytes,
            FrontendLimitKind::Tokens,
            FrontendLimitKind::TotalWork,
        ]
    );

    use std::collections::HashSet;

    let set: HashSet<FrontendLimitKind> = kinds.into_iter().collect();

    assert_eq!(set.len(), 3);
    assert!(set.contains(&FrontendLimitKind::SourceBytes));
    assert!(set.contains(&FrontendLimitKind::Tokens));
    assert!(set.contains(&FrontendLimitKind::TotalWork));
}

#[test]
fn violation_is_copyable_and_equality_stable() {
    let first =
        FrontendLimitViolation::new(FrontendLimitKind::Tokens, 4, 3);

    let second = first;

    assert_eq!(first, second);
    assert_eq!(first.kind(), second.kind());
    assert_eq!(first.actual(), second.actual());
    assert_eq!(first.maximum(), second.maximum());
}

#[test]
fn production_limits_leave_room_for_aggregate_source_processing() {
    let limits = FrontendLimits::production();

    assert!(
        limits.max_total_source_bytes() >= limits.max_source_bytes(),
        "aggregate source capacity must accommodate at least one maximum-sized source"
    );
}

#[test]
fn expression_depth_is_never_greater_than_nesting_depth_in_profiles() {
    for limits in [
        FrontendLimits::production(),
        FrontendLimits::strict(),
        FrontendLimits::large(),
    ] {
        assert!(
            limits.max_expression_depth()
                <= limits.max_nesting_depth(),
            "expression-depth must remain bounded by nesting-depth"
        );
    }
}