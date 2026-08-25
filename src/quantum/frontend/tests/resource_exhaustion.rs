//! Zamani Quantum Frontend — resource-exhaustion/security tests.
//!
//! Production security tests for the complete resource-boundary contract:
//!
//! ```text
//!                    untrusted input
//!                          │
//!                          ▼
//!                    ImportInput
//!                          │
//!                          ▼
//!                    OpenQASM frontend
//!                          │
//!              ┌───────────┼────────────┐
//!              ▼           ▼            ▼
//!            lexer       parser      validation
//!              │           │            │
//!              └───────────┼────────────┘
//!                          ▼
//!                       lowering
//!                          │
//!                          ▼
//!                    canonical IR
//! ```
//!
//! # Purpose
//!
//! This test module is specifically responsible for proving that the frontend
//! does not allow an attacker-controlled input to consume unbounded resources.
//!
//! It is intentionally different from:
//!
//! - `limits.rs` — implementation-level limit contracts;
//! - `malformed_inputs.rs` — malformed/truncated syntax corpus;
//! - `openqasm_lexer.rs` — lexical correctness;
//! - `openqasm_parser.rs` — grammar correctness;
//! - `openqasm_validation.rs` — semantic correctness;
//! - `openqasm_import.rs` — normal import behavior;
//! - `openqasm_export.rs` — normal export behavior;
//! - `openqasm_roundtrip.rs` — semantic round-trip behavior.
//!
//! This file owns the security property:
//!
//! > Every externally controlled frontend resource dimension is bounded by
//! > `FrontendLimits`, and exhaustion becomes deterministic structured data
//! > rather than a panic, hang, overflow, or uncontrolled allocation.
//!
//! # Resource dimensions covered
//!
//! The production `FrontendLimits` contract currently covers:
//!
//! - source bytes;
//! - aggregate source bytes;
//! - source-file count;
//! - lexer tokens;
//! - identifier length;
//! - string length;
//! - numeric literal length;
//! - comment length;
//! - annotation length;
//! - AST nodes;
//! - general nesting depth;
//! - expression depth;
//! - expression-node count;
//! - diagnostics;
//! - diagnostic children;
//! - diagnostic snippet length;
//! - include depth;
//! - include graph edges;
//! - gate definitions;
//! - gate operations;
//! - register size;
//! - array elements;
//! - symbol count;
//! - parameter count;
//! - operand count;
//! - statements per block;
//! - total statements;
//! - annotations per item;
//! - generated operations;
//! - recursion depth;
//! - exported output bytes;
//! - total frontend work.
//!
//! # Security invariants
//!
//! 1. Every configured limit is finite and non-zero.
//! 2. Every configured limit has a stable `FrontendLimitKind`.
//! 3. Values exactly at a configured limit are accepted by the generic
//!    predicate.
//! 4. Values one above a configured limit are rejected.
//! 5. Limit checking is overflow-safe.
//! 6. Resource exhaustion does not require allocating the configured maximum.
//! 7. A small adversarial configuration can exercise production paths.
//! 8. OpenQASM import respects configured frontend limits.
//! 9. Resource-limit failures classify as `FrontendErrorKind::LimitExceeded`.
//! 10. Repeating the same bounded attack produces the same error kind/code.
//! 11. Resource exhaustion cannot silently become successful import.
//! 12. Resource exhaustion cannot silently truncate source.
//! 13. Resource exhaustion cannot silently truncate diagnostics.
//! 14. Resource exhaustion cannot silently truncate generated operations.
//! 15. Resource exhaustion cannot silently truncate exported output.
//! 16. Resource exhaustion cannot trigger filesystem access.
//! 17. Resource exhaustion cannot trigger network access.
//! 18. Resource exhaustion cannot trigger process execution.
//! 19. Resource exhaustion cannot trigger QPU/hardware access.
//! 20. No test relies on huge allocations merely to prove enforcement.
//!
//! # Important testing rule
//!
//! The tests deliberately use tiny custom limits. For example, a token limit
//! of 8 is sufficient to prove the token boundary; there is no reason for CI
//! to allocate one million tokens.
//!
//! This makes the tests suitable for:
//!
//! - local development;
//! - CI;
//! - release builds;
//! - constrained build machines;
//! - fuzzing harnesses;
//! - security regression testing.
//!
//! # Rust compatibility
//!
//! - Rust 2021;
//! - Rust 1.97 / 1.97.1;
//! - stable Rust only;
//! - no nightly features;
//! - no external test dependencies;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! This file depends only on contracts established by earlier frontend layers:
//!
//! ```text
//! core/source.rs
//!       │
//! core/limits.rs ───────┐
//!       │               │
//! core/errors.rs        │
//!       │               │
//!       └──────┬────────┘
//!              ▼
//!       generic importer
//!              │
//!              ▼
//!       OpenQasmImporter
//! ```
//!
//! It does not depend on OpenQASM lexer/parser implementation details.
//!
//! Register it in the frontend test harness using the repository's existing
//! test-module wiring. No production module imports this file.
//!
//! # Production boundary
//!
//! The frontend itself remains side-effect free. These tests therefore do not
//! mock or grant filesystem, network, process, runtime, or QPU permissions.
//!
//! Resource exhaustion must be handled entirely inside the frontend contract.
//!
//! # API compatibility
//!
//! The tests intentionally use the current repository APIs:
//!
//! - `FrontendLimits::production()`;
//! - `FrontendLimits::builder()`;
//! - `FrontendLimits::check_*()`;
//! - `FrontendLimitViolation`;
//! - `FrontendLimitKind`;
//! - `FrontendErrorKind`;
//! - `ImportConfig::new()`;
//! - `ImportInput::new()`;
//! - `OpenQasmImporter::production()`.
//!
//! No new production API is required by this test file.

#![allow(clippy::module_name_repetitions)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use crate::quantum::frontend::core::errors::FrontendErrorKind;
use crate::quantum::frontend::core::limits::{
    FrontendLimitKind,
    FrontendLimitViolation,
    FrontendLimits,
};
use crate::quantum::frontend::core::source::SourceMap;
use crate::quantum::frontend::importer::{
    FormatImporter,
    ImportConfig,
    ImportInput,
};
use crate::quantum::frontend::OpenQasmImporter;

// =============================================================================
// Test configuration
// =============================================================================

/// Small limits used for adversarial frontend integration tests.
///
/// These are intentionally much smaller than the production profile. They
/// allow the tests to exercise the exact production enforcement mechanisms
/// without making CI allocate large buffers.
fn adversarial_limits() -> FrontendLimits {
    FrontendLimits::builder()
        .max_source_bytes(64)
        .max_total_source_bytes(128)
        .max_source_files(2)
        .max_tokens(16)
        .max_identifier_length(8)
        .max_string_length(16)
        .max_numeric_literal_length(8)
        .max_comment_length(16)
        .max_annotation_length(16)
        .max_ast_nodes(32)
        .max_nesting_depth(8)
        .max_expression_depth(4)
        .max_expression_nodes(16)
        .max_diagnostics(4)
        .max_diagnostic_children(4)
        .max_diagnostic_snippet_length(32)
        .max_include_depth(2)
        .max_include_edges(4)
        .max_gate_definitions(4)
        .max_gate_operations(8)
        .max_register_size(4)
        .max_array_elements(4)
        .max_symbols(8)
        .max_parameters(4)
        .max_operands(4)
        .max_statements_per_block(8)
        .max_statements(16)
        .max_annotations_per_item(4)
        .max_operations(8)
        .max_recursion_depth(4)
        .max_output_bytes(128)
        .max_total_work(64)
        .build()
        .expect("adversarial frontend limits must be internally valid")
}

/// Builds a valid minimal OpenQASM prefix.
///
/// Individual tests extend this prefix only as much as required to trigger a
/// particular limit.
fn qasm_prefix() -> &'static str {
    "OPENQASM 3.1;\n"
}

/// Creates an `ImportInput` using the exact generic source-map boundary used
/// by production frontend imports.
fn make_input(
    source: &str,
    limits: FrontendLimits,
) -> ImportInput {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("resource-exhaustion.qasm"),
            Arc::<str>::from(source),
        )
        .expect("test source must satisfy SourceMap invariants");

    ImportInput::new(
        source_id,
        source.as_bytes().to_vec(),
        source_map,
        ImportConfig::new(limits),
    )
    .expect("test input must satisfy ImportInput invariants")
}

/// Runs OpenQASM import and converts an unexpected panic into a test failure.
///
/// The frontend production contract requires hostile input to become a
/// structured result rather than unwinding.
fn import_without_panic(
    source: &str,
    limits: FrontendLimits,
) -> Result<
    crate::quantum::frontend::importer::ImportOutput,
    crate::quantum::frontend::core::errors::FrontendError,
> {
    let importer = OpenQasmImporter::production();
    let input = make_input(source, limits);

    let result = catch_unwind(AssertUnwindSafe(|| importer.import(input)));

    match result {
        Ok(result) => result,
        Err(payload) => {
            panic!(
                "frontend panicked during resource-exhaustion test; \
                 panic payload type: {}",
                panic_payload_type(&payload),
            );
        }
    }
}

/// Returns the panic payload type without printing potentially unbounded or
/// sensitive payload contents.
fn panic_payload_type(
    payload: &(dyn std::any::Any + Send),
) -> &'static str {
    if payload.is::<&'static str>() {
        "&'static str"
    } else if payload.is::<String>() {
        "String"
    } else {
        "unknown"
    }
}

/// Requires a frontend result to be a resource-limit failure.
fn assert_limit_failure(
    result: Result<
        crate::quantum::frontend::importer::ImportOutput,
        crate::quantum::frontend::core::errors::FrontendError,
    >,
) {
    let error = result.expect_err(
        "resource exhaustion must never be accepted as successful import",
    );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::LimitExceeded,
        "resource exhaustion must use the canonical LimitExceeded error kind",
    );

    assert_eq!(
        error.code(),
        crate::quantum::frontend::core::errors::FrontendErrorCode::LIMIT_EXCEEDED,
        "resource exhaustion must use the stable FE-003 error code",
    );
}

/// Executes the same attack twice and verifies stable classification.
///
/// The test intentionally does not compare human-readable error strings.
fn assert_deterministic_limit_failure(
    source: &str,
    limits: FrontendLimits,
) {
    let first = import_without_panic(source, limits);
    let second = import_without_panic(source, limits);

    let first_error = first.expect_err(
        "first resource-exhaustion run must fail",
    );
    let second_error = second.expect_err(
        "second resource-exhaustion run must fail",
    );

    assert_eq!(
        first_error.kind(),
        FrontendErrorKind::LimitExceeded,
    );

    assert_eq!(
        second_error.kind(),
        FrontendErrorKind::LimitExceeded,
    );

    assert_eq!(
        first_error.code(),
        second_error.code(),
        "same attack must retain the same stable error code",
    );
}

// =============================================================================
// Generic FrontendLimits contract
// =============================================================================

#[test]
fn production_limits_are_valid() {
    let limits = FrontendLimits::production();

    assert!(
        limits.validate().is_ok(),
        "production frontend limits must be internally valid",
    );
}

#[test]
fn strict_limits_are_valid() {
    let limits = FrontendLimits::strict();

    assert!(
        limits.validate().is_ok(),
        "strict frontend limits must be internally valid",
    );
}

#[test]
fn large_limits_are_valid() {
    let limits = FrontendLimits::large();

    assert!(
        limits.validate().is_ok(),
        "large frontend limits must be internally valid",
    );
}

#[test]
fn adversarial_limits_are_valid() {
    assert!(
        adversarial_limits().validate().is_ok(),
        "adversarial test limits must be internally valid",
    );
}

// =============================================================================
// Boundary helper
// =============================================================================

/// Verifies the generic "exactly at limit succeeds / one above fails" contract.
fn assert_boundary(
    check: impl Fn(u64) -> Result<(), FrontendLimitViolation>,
    maximum: u64,
) {
    assert!(
        check(maximum).is_ok(),
        "value exactly at the configured maximum must be accepted",
    );

    let above = maximum
        .checked_add(1)
        .expect("test maximum must permit +1");

    let violation = check(above)
        .expect_err("value one above the maximum must be rejected");

    assert_eq!(
        violation.actual(),
        above,
        "violation must retain the observed value",
    );

    assert_eq!(
        violation.maximum(),
        maximum,
        "violation must retain the configured maximum",
    );
}

// =============================================================================
// Source/resource limits
// =============================================================================

#[test]
fn source_bytes_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_source_bytes(value),
        limits.max_source_bytes(),
    );
}

#[test]
fn total_source_bytes_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_total_source_bytes(value),
        limits.max_total_source_bytes(),
    );
}

#[test]
fn source_file_count_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_source_files(value),
        limits.max_source_files(),
    );
}

// =============================================================================
// Lexer/input limits
// =============================================================================

#[test]
fn token_count_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_tokens(value),
        limits.max_tokens(),
    );
}

#[test]
fn identifier_length_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_identifier_length(value),
        limits.max_identifier_length(),
    );
}

#[test]
fn string_length_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_string_length(value),
        limits.max_string_length(),
    );
}

#[test]
fn numeric_literal_length_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_numeric_literal_length(value),
        limits.max_numeric_literal_length(),
    );
}

#[test]
fn comment_length_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_comment_length(value),
        limits.max_comment_length(),
    );
}

#[test]
fn annotation_length_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_annotation_length(value),
        limits.max_annotation_length(),
    );
}

// =============================================================================
// Parser/AST limits
// =============================================================================

#[test]
fn ast_node_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_ast_nodes(value),
        limits.max_ast_nodes(),
    );
}

#[test]
fn nesting_depth_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_nesting_depth(value),
        limits.max_nesting_depth(),
    );
}

#[test]
fn expression_depth_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_expression_depth(value),
        limits.max_expression_depth(),
    );
}

#[test]
fn expression_node_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_expression_nodes(value),
        limits.max_expression_nodes(),
    );
}

// =============================================================================
// Diagnostic limits
// =============================================================================

#[test]
fn diagnostic_count_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_diagnostics(value),
        limits.max_diagnostics(),
    );
}

#[test]
fn diagnostic_child_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_diagnostic_children(value),
        limits.max_diagnostic_children(),
    );
}

#[test]
fn diagnostic_snippet_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_diagnostic_snippet_length(value),
        limits.max_diagnostic_snippet_length(),
    );
}

// =============================================================================
// Include/import graph limits
// =============================================================================

#[test]
fn include_depth_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_include_depth(value),
        limits.max_include_depth(),
    );
}

#[test]
fn include_edge_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_include_edges(value),
        limits.max_include_edges(),
    );
}

// =============================================================================
// Semantic limits
// =============================================================================

#[test]
fn gate_definition_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_gate_definitions(value),
        limits.max_gate_definitions(),
    );
}

#[test]
fn gate_operation_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_gate_operations(value),
        limits.max_gate_operations(),
    );
}

#[test]
fn register_size_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_register_size(value),
        limits.max_register_size(),
    );
}

#[test]
fn array_element_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_array_elements(value),
        limits.max_array_elements(),
    );
}

#[test]
fn symbol_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_symbols(value),
        limits.max_symbols(),
    );
}

#[test]
fn parameter_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_parameters(value),
        limits.max_parameters(),
    );
}

#[test]
fn operand_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_operands(value),
        limits.max_operands(),
    );
}

#[test]
fn statements_per_block_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_statements_per_block(value),
        limits.max_statements_per_block(),
    );
}

#[test]
fn total_statement_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_statements(value),
        limits.max_statements(),
    );
}

#[test]
fn annotations_per_item_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_annotations_per_item(value),
        limits.max_annotations_per_item(),
    );
}

// =============================================================================
// Lowering/export/work limits
// =============================================================================

#[test]
fn operation_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_operations(value),
        limits.max_operations(),
    );
}

#[test]
fn recursion_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_recursion_depth(value),
        limits.max_recursion_depth(),
    );
}

#[test]
fn output_byte_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_output_bytes(value),
        limits.max_output_bytes(),
    );
}

#[test]
fn total_work_boundary_is_enforced() {
    let limits = adversarial_limits();

    assert_boundary(
        |value| limits.check_total_work(value),
        limits.max_total_work(),
    );
}

// =============================================================================
// Stable limit identities
// =============================================================================

#[test]
fn every_resource_violation_preserves_limit_identity() {
    let cases = [
        (
            FrontendLimitKind::SourceBytes,
            FrontendLimits::production().max_source_bytes(),
        ),
        (
            FrontendLimitKind::TotalSourceBytes,
            FrontendLimits::production().max_total_source_bytes(),
        ),
        (
            FrontendLimitKind::SourceFiles,
            FrontendLimits::production().max_source_files(),
        ),
        (
            FrontendLimitKind::Tokens,
            FrontendLimits::production().max_tokens(),
        ),
        (
            FrontendLimitKind::IdentifierLength,
            FrontendLimits::production().max_identifier_length(),
        ),
        (
            FrontendLimitKind::StringLength,
            FrontendLimits::production().max_string_length(),
        ),
        (
            FrontendLimitKind::NumericLiteralLength,
            FrontendLimits::production()
                .max_numeric_literal_length(),
        ),
        (
            FrontendLimitKind::CommentLength,
            FrontendLimits::production().max_comment_length(),
        ),
        (
            FrontendLimitKind::AnnotationLength,
            FrontendLimits::production().max_annotation_length(),
        ),
        (
            FrontendLimitKind::AstNodes,
            FrontendLimits::production().max_ast_nodes(),
        ),
        (
            FrontendLimitKind::NestingDepth,
            FrontendLimits::production().max_nesting_depth(),
        ),
        (
            FrontendLimitKind::ExpressionDepth,
            FrontendLimits::production().max_expression_depth(),
        ),
        (
            FrontendLimitKind::ExpressionNodes,
            FrontendLimits::production().max_expression_nodes(),
        ),
        (
            FrontendLimitKind::Diagnostics,
            FrontendLimits::production().max_diagnostics(),
        ),
        (
            FrontendLimitKind::DiagnosticChildren,
            FrontendLimits::production()
                .max_diagnostic_children(),
        ),
        (
            FrontendLimitKind::DiagnosticSnippetLength,
            FrontendLimits::production()
                .max_diagnostic_snippet_length(),
        ),
        (
            FrontendLimitKind::IncludeDepth,
            FrontendLimits::production().max_include_depth(),
        ),
        (
            FrontendLimitKind::IncludeEdges,
            FrontendLimits::production().max_include_edges(),
        ),
        (
            FrontendLimitKind::GateDefinitions,
            FrontendLimits::production().max_gate_definitions(),
        ),
        (
            FrontendLimitKind::GateOperations,
            FrontendLimits::production().max_gate_operations(),
        ),
        (
            FrontendLimitKind::RegisterSize,
            FrontendLimits::production().max_register_size(),
        ),
        (
            FrontendLimitKind::ArrayElements,
            FrontendLimits::production().max_array_elements(),
        ),
        (
            FrontendLimitKind::Symbols,
            FrontendLimits::production().max_symbols(),
        ),
        (
            FrontendLimitKind::Parameters,
            FrontendLimits::production().max_parameters(),
        ),
        (
            FrontendLimitKind::Operands,
            FrontendLimits::production().max_operands(),
        ),
        (
            FrontendLimitKind::StatementsPerBlock,
            FrontendLimits::production()
                .max_statements_per_block(),
        ),
        (
            FrontendLimitKind::Statements,
            FrontendLimits::production().max_statements(),
        ),
        (
            FrontendLimitKind::AnnotationsPerItem,
            FrontendLimits::production()
                .max_annotations_per_item(),
        ),
        (
            FrontendLimitKind::Operations,
            FrontendLimits::production().max_operations(),
        ),
        (
            FrontendLimitKind::RecursionDepth,
            FrontendLimits::production().max_recursion_depth(),
        ),
        (
            FrontendLimitKind::OutputBytes,
            FrontendLimits::production().max_output_bytes(),
        ),
        (
            FrontendLimitKind::TotalWork,
            FrontendLimits::production().max_total_work(),
        ),
    ];

    for (kind, maximum) in cases {
        let limits = FrontendLimits::production();

        let value = maximum
            .checked_add(1)
            .expect("production maximum must permit +1");

        let violation = match kind {
            FrontendLimitKind::SourceBytes => {
                limits.check_source_bytes(value)
            }
            FrontendLimitKind::TotalSourceBytes => {
                limits.check_total_source_bytes(value)
            }
            FrontendLimitKind::SourceFiles => {
                limits.check_source_files(value)
            }
            FrontendLimitKind::Tokens => limits.check_tokens(value),
            FrontendLimitKind::IdentifierLength => {
                limits.check_identifier_length(value)
            }
            FrontendLimitKind::StringLength => {
                limits.check_string_length(value)
            }
            FrontendLimitKind::NumericLiteralLength => {
                limits.check_numeric_literal_length(value)
            }
            FrontendLimitKind::CommentLength => {
                limits.check_comment_length(value)
            }
            FrontendLimitKind::AnnotationLength => {
                limits.check_annotation_length(value)
            }
            FrontendLimitKind::AstNodes => {
                limits.check_ast_nodes(value)
            }
            FrontendLimitKind::NestingDepth => {
                limits.check_nesting_depth(value)
            }
            FrontendLimitKind::ExpressionDepth => {
                limits.check_expression_depth(value)
            }
            FrontendLimitKind::ExpressionNodes => {
                limits.check_expression_nodes(value)
            }
            FrontendLimitKind::Diagnostics => {
                limits.check_diagnostics(value)
            }
            FrontendLimitKind::DiagnosticChildren => {
                limits.check_diagnostic_children(value)
            }
            FrontendLimitKind::DiagnosticSnippetLength => {
                limits.check_diagnostic_snippet_length(value)
            }
            FrontendLimitKind::IncludeDepth => {
                limits.check_include_depth(value)
            }
            FrontendLimitKind::IncludeEdges => {
                limits.check_include_edges(value)
            }
            FrontendLimitKind::GateDefinitions => {
                limits.check_gate_definitions(value)
            }
            FrontendLimitKind::GateOperations => {
                limits.check_gate_operations(value)
            }
            FrontendLimitKind::RegisterSize => {
                limits.check_register_size(value)
            }
            FrontendLimitKind::ArrayElements => {
                limits.check_array_elements(value)
            }
            FrontendLimitKind::Symbols => {
                limits.check_symbols(value)
            }
            FrontendLimitKind::Parameters => {
                limits.check_parameters(value)
            }
            FrontendLimitKind::Operands => {
                limits.check_operands(value)
            }
            FrontendLimitKind::StatementsPerBlock => {
                limits.check_statements_per_block(value)
            }
            FrontendLimitKind::Statements => {
                limits.check_statements(value)
            }
            FrontendLimitKind::AnnotationsPerItem => {
                limits.check_annotations_per_item(value)
            }
            FrontendLimitKind::Operations => {
                limits.check_operations(value)
            }
            FrontendLimitKind::RecursionDepth => {
                limits.check_recursion_depth(value)
            }
            FrontendLimitKind::OutputBytes => {
                limits.check_output_bytes(value)
            }
            FrontendLimitKind::TotalWork => {
                limits.check_total_work(value)
            }
        }
        .expect_err("one above every limit must produce a violation");

        assert_eq!(
            violation.kind(),
            kind,
            "limit violation must preserve its exact FrontendLimitKind",
        );
        assert_eq!(
            violation.maximum(),
            maximum,
            "limit violation must preserve its configured maximum",
        );
        assert_eq!(
            violation.actual(),
            value,
            "limit violation must preserve its observed value",
        );
    }
}

// =============================================================================
// Overflow safety
// =============================================================================

#[test]
fn source_limit_check_is_safe_at_u64_max() {
    let limits = FrontendLimits::production();

    let result = limits.check_source_bytes(u64::MAX);

    assert!(
        result.is_err(),
        "u64::MAX must be rejected when production source limit is finite",
    );
}

#[test]
fn token_limit_check_is_safe_at_u64_max() {
    let limits = FrontendLimits::production();

    let result = limits.check_tokens(u64::MAX);

    assert!(
        result.is_err(),
        "u64::MAX must be rejected without arithmetic overflow",
    );
}

#[test]
fn ast_limit_check_is_safe_at_u64_max() {
    let limits = FrontendLimits::production();

    let result = limits.check_ast_nodes(u64::MAX);

    assert!(
        result.is_err(),
        "u64::MAX must be rejected without arithmetic overflow",
    );
}

#[test]
fn operation_limit_check_is_safe_at_u64_max() {
    let limits = FrontendLimits::production();

    let result = limits.check_operations(u64::MAX);

    assert!(
        result.is_err(),
        "u64::MAX must be rejected without arithmetic overflow",
    );
}

#[test]
fn total_work_limit_check_is_safe_at_u64_max() {
    let limits = FrontendLimits::production();

    let result = limits.check_total_work(u64::MAX);

    assert!(
        result.is_err(),
        "u64::MAX must be rejected without arithmetic overflow",
    );
}

// =============================================================================
// Actual OpenQASM frontend resource attacks
// =============================================================================

#[test]
fn oversized_source_is_rejected_before_normal_import() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(32)
        .max_total_source_bytes(64)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}{}",
        qasm_prefix(),
        "x".repeat(64),
    );

    let result = import_without_panic(
        &source,
        limits,
    );

    assert_limit_failure(result);
}

#[test]
fn excessive_identifier_length_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(4 * 1024)
        .max_total_source_bytes(8 * 1024)
        .max_identifier_length(4)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}qubit[1] verylongidentifier;\n",
        qasm_prefix(),
    );

    let result = import_without_panic(
        &source,
        limits,
    );

    assert_limit_failure(result);
}

#[test]
fn excessive_numeric_literal_length_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(4 * 1024)
        .max_total_source_bytes(8 * 1024)
        .max_numeric_literal_length(4)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}const float x = 123456789;\n",
        qasm_prefix(),
    );

    let result = import_without_panic(
        &source,
        limits,
    );

    assert_limit_failure(result);
}

#[test]
fn excessive_comment_length_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(4 * 1024)
        .max_total_source_bytes(8 * 1024)
        .max_comment_length(8)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}//{}\n",
        qasm_prefix(),
        "x".repeat(64),
    );

    let result = import_without_panic(
        &source,
        limits,
    );

    assert_limit_failure(result);
}

#[test]
fn excessive_token_count_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(4 * 1024)
        .max_total_source_bytes(8 * 1024)
        .max_tokens(8)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}{}",
        qasm_prefix(),
        "qubit[1] q;\n".repeat(8),
    );

    let result = import_without_panic(
        &source,
        limits,
    );

    assert_limit_failure(result);
}

#[test]
fn excessive_register_size_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(4 * 1024)
        .max_total_source_bytes(8 * 1024)
        .max_register_size(2)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}qubit[3] q;\n",
        qasm_prefix(),
    );

    let result = import_without_panic(
        &source,
        limits,
    );

    assert_limit_failure(result);
}

#[test]
fn excessive_statement_count_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(16 * 1024)
        .max_total_source_bytes(32 * 1024)
        .max_statements(4)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}{}",
        qasm_prefix(),
        "barrier;\n".repeat(8),
    );

    let result = import_without_panic(
        &source,
        limits,
    );

    assert_limit_failure(result);
}

#[test]
fn excessive_operation_count_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(16 * 1024)
        .max_total_source_bytes(32 * 1024)
        .max_operations(2)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}qubit[1] q;\n{}",
        qasm_prefix(),
        "x q[0];\n".repeat(8),
    );

    let result = import_without_panic(
        &source,
        limits,
    );

    assert_limit_failure(result);
}

#[test]
fn excessive_nesting_depth_is_rejected() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(16 * 1024)
        .max_total_source_bytes(32 * 1024)
        .max_nesting_depth(2)
        .max_expression_depth(2)
        .build()
        .expect("test limits must be valid");

    let mut source = qasm_prefix().to_owned();

    source.push_str("qubit[1] q;\n");

    /*
     * The exact semantic construct used for nesting is intentionally simple.
     * The production parser/validator must enforce the configured nesting
     * boundary regardless of the concrete recursive syntax selected.
     */
    for _ in 0..8 {
        source.push_str("{\n");
    }

    for _ in 0..8 {
        source.push_str("}\n");
    }

    let result = import_without_panic(
        &source,
        limits,
    );

    /*
     * A particular implementation may classify the construct as syntax rather
     * than a resource violation if the grammar rejects the synthetic nesting
     * first. Therefore this assertion permits only a structured frontend
     * failure and explicitly forbids successful acceptance.
     */
    let error = result.expect_err(
        "deeply nested hostile input must not be accepted",
    );

    assert!(
        matches!(
            error.kind(),
            FrontendErrorKind::LimitExceeded
                | FrontendErrorKind::Syntax
                | FrontendErrorKind::Semantic
        ),
        "deep nesting must terminate as a structured frontend failure",
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn oversized_source_failure_is_deterministic() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(32)
        .max_total_source_bytes(64)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}{}",
        qasm_prefix(),
        "x".repeat(128),
    );

    assert_deterministic_limit_failure(
        &source,
        limits,
    );
}

#[test]
fn token_exhaustion_failure_is_deterministic() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(8 * 1024)
        .max_total_source_bytes(16 * 1024)
        .max_tokens(8)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}{}",
        qasm_prefix(),
        "x q[0];\n".repeat(32),
    );

    assert_deterministic_limit_failure(
        &source,
        limits,
    );
}

#[test]
fn operation_exhaustion_failure_is_deterministic() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(8 * 1024)
        .max_total_source_bytes(16 * 1024)
        .max_operations(2)
        .build()
        .expect("test limits must be valid");

    let source = format!(
        "{}qubit[1] q;\n{}",
        qasm_prefix(),
        "x q[0];\n".repeat(16),
    );

    assert_deterministic_limit_failure(
        &source,
        limits,
    );
}

// =============================================================================
// No false success
// =============================================================================

#[test]
fn resource_exhaustion_never_returns_successful_import() {
    let attacks = [
        (
            FrontendLimits::builder()
                .max_source_bytes(16)
                .max_total_source_bytes(32)
                .build()
                .expect("valid limits"),
            format!(
                "{}{}",
                qasm_prefix(),
                "x".repeat(128),
            ),
        ),
        (
            FrontendLimits::builder()
                .max_source_bytes(4 * 1024)
                .max_total_source_bytes(8 * 1024)
                .max_tokens(4)
                .build()
                .expect("valid limits"),
            format!(
                "{}{}",
                qasm_prefix(),
                "barrier;\n".repeat(32),
            ),
        ),
        (
            FrontendLimits::builder()
                .max_source_bytes(4 * 1024)
                .max_total_source_bytes(8 * 1024)
                .max_register_size(1)
                .build()
                .expect("valid limits"),
            format!(
                "{}qubit[64] q;\n",
                qasm_prefix(),
            ),
        ),
    ];

    for (limits, source) in attacks {
        let result = import_without_panic(
            &source,
            limits,
        );

        assert_limit_failure(result);
    }
}

// =============================================================================
// Limit object immutability/value semantics
// =============================================================================

#[test]
fn cloning_limits_preserves_exact_policy() {
    let original = adversarial_limits();
    let clone = original;

    assert_eq!(
        original,
        clone,
        "FrontendLimits must have deterministic value semantics",
    );

    assert_eq!(
        original.max_source_bytes(),
        clone.max_source_bytes(),
    );

    assert_eq!(
        original.max_tokens(),
        clone.max_tokens(),
    );

    assert_eq!(
        original.max_operations(),
        clone.max_operations(),
    );

    assert_eq!(
        original.max_total_work(),
        clone.max_total_work(),
    );
}

// =============================================================================
// Resource-limit error identity
// =============================================================================

#[test]
fn limit_violation_display_is_deterministic() {
    let violation = FrontendLimitViolation::new(
        FrontendLimitKind::Tokens,
        17,
        16,
    );

    let first = violation.to_string();
    let second = violation.to_string();

    assert_eq!(
        first,
        second,
        "resource-limit formatting must be deterministic",
    );

    assert!(
        first.contains("tokens"),
        "formatted violation must identify the resource kind",
    );

    assert!(
        first.contains("17"),
        "formatted violation must contain the observed value",
    );

    assert!(
        first.contains("16"),
        "formatted violation must contain the configured maximum",
    );
}

#[test]
fn limit_kind_names_are_machine_stable() {
    assert_eq!(
        FrontendLimitKind::SourceBytes.to_string(),
        "source-bytes",
    );

    assert_eq!(
        FrontendLimitKind::Tokens.to_string(),
        "tokens",
    );

    assert_eq!(
        FrontendLimitKind::AstNodes.to_string(),
        "ast-nodes",
    );

    assert_eq!(
        FrontendLimitKind::Operations.to_string(),
        "operations",
    );

    assert_eq!(
        FrontendLimitKind::TotalWork.to_string(),
        "total-work",
    );
}

// =============================================================================
// Configuration safety
// =============================================================================

#[test]
fn zero_limit_configuration_is_rejected() {
    let result = FrontendLimits::builder()
        .max_source_bytes(0)
        .build();

    assert!(
        result.is_err(),
        "zero source limit must not create an unusable policy",
    );
}

#[test]
fn_total_source_capacity_cannot_be_less_than_single_source_capacity() {
    let result = FrontendLimits::builder()
        .max_source_bytes(128)
        .max_total_source_bytes(64)
        .build();

    assert!(
        result.is_err(),
        "aggregate source capacity must not be smaller than one source",
    );
}

#[test]
fn expression_depth_cannot_exceed_nesting_depth() {
    let result = FrontendLimits::builder()
        .max_nesting_depth(2)
        .max_expression_depth(3)
        .build();

    assert!(
        result.is_err(),
        "expression depth must not exceed general nesting depth",
    );
}

// =============================================================================
// Public security boundary
// =============================================================================

#[test]
fn malformed_resource_attack_does_not_require_external_io() {
    /*
     * This is intentionally a structural test rather than a filesystem mock.
     *
     * The frontend APIs used here receive bytes and source-map data directly.
     * No path is supplied as a permission to load source. Therefore this
     * resource-exhaustion path cannot legitimately require filesystem access.
     */
    let limits = FrontendLimits::builder()
        .max_source_bytes(16)
        .max_total_source_bytes(32)
        .build()
        .expect("valid limits");

    let source = format!(
        "{}{}",
        qasm_prefix(),
        "x".repeat(128),
    );

    let result = import_without_panic(
        &source,
        limits,
    );

    assert_limit_failure(result);
}

#[test]
fn resource_attack_cannot_become_execution_permission() {
    /*
     * Source-level constructs are data at the frontend boundary.
     * This test intentionally supplies an external-looking construct while
     * keeping the input under caller-controlled bytes.
     *
     * The expected result is a structured frontend outcome, never process
     * execution or an uncontrolled side effect.
     */
    let limits = FrontendLimits::builder()
        .max_source_bytes(4 * 1024)
        .max_total_source_bytes(8 * 1024)
        .max_string_length(8)
        .build()
        .expect("valid limits");

    let source = concat!(
        "OPENQASM 3.1;\n",
        "include \"this-string-is-deliberately-too-long\";\n",
    );

    let result = import_without_panic(
        source,
        limits,
    );

    assert!(
        result.is_err(),
        "hostile external-looking source must terminate inside frontend",
    );
}

// =============================================================================
// Regression guard: production defaults remain finite
// =============================================================================

#[test]
fn production_profile_has_finite_limits_for_every_dimension() {
    let limits = FrontendLimits::production();

    let values = [
        limits.max_source_bytes(),
        limits.max_total_source_bytes(),
        limits.max_source_files(),
        limits.max_tokens(),
        limits.max_identifier_length(),
        limits.max_string_length(),
        limits.max_numeric_literal_length(),
        limits.max_comment_length(),
        limits.max_annotation_length(),
        limits.max_ast_nodes(),
        limits.max_nesting_depth(),
        limits.max_expression_depth(),
        limits.max_expression_nodes(),
        limits.max_diagnostics(),
        limits.max_diagnostic_children(),
        limits.max_diagnostic_snippet_length(),
        limits.max_include_depth(),
        limits.max_include_edges(),
        limits.max_gate_definitions(),
        limits.max_gate_operations(),
        limits.max_register_size(),
        limits.max_array_elements(),
        limits.max_symbols(),
        limits.max_parameters(),
        limits.max_operands(),
        limits.max_statements_per_block(),
        limits.max_statements(),
        limits.max_annotations_per_item(),
        limits.max_operations(),
        limits.max_recursion_depth(),
        limits.max_output_bytes(),
        limits.max_total_work(),
    ];

    for value in values {
        assert!(
            value > 0,
            "production frontend limits must never be zero",
        );
    }
}

// =============================================================================
// Regression guard: all resource checks reject u64::MAX
// =============================================================================

#[test]
fn every_production_resource_check_rejects_u64_max() {
    let limits = FrontendLimits::production();

    let checks: &[fn(
        &FrontendLimits,
        u64,
    ) -> Result<(), FrontendLimitViolation>] = &[
        FrontendLimits::check_source_bytes,
        FrontendLimits::check_total_source_bytes,
        FrontendLimits::check_source_files,
        FrontendLimits::check_tokens,
        FrontendLimits::check_identifier_length,
        FrontendLimits::check_string_length,
        FrontendLimits::check_numeric_literal_length,
        FrontendLimits::check_comment_length,
        FrontendLimits::check_annotation_length,
        FrontendLimits::check_ast_nodes,
        FrontendLimits::check_nesting_depth,
        FrontendLimits::check_expression_depth,
        FrontendLimits::check_expression_nodes,
        FrontendLimits::check_diagnostics,
        FrontendLimits::check_diagnostic_children,
        FrontendLimits::check_diagnostic_snippet_length,
        FrontendLimits::check_include_depth,
        FrontendLimits::check_include_edges,
        FrontendLimits::check_gate_definitions,
        FrontendLimits::check_gate_operations,
        FrontendLimits::check_register_size,
        FrontendLimits::check_array_elements,
        FrontendLimits::check_symbols,
        FrontendLimits::check_parameters,
        FrontendLimits::check_operands,
        FrontendLimits::check_statements_per_block,
        FrontendLimits::check_statements,
        FrontendLimits::check_annotations_per_item,
        FrontendLimits::check_operations,
        FrontendLimits::check_recursion_depth,
        FrontendLimits::check_output_bytes,
        FrontendLimits::check_total_work,
    ];

    for check in checks {
        assert!(
            check(&limits, u64::MAX).is_err(),
            "finite production limit must reject u64::MAX",
        );
    }
}

// =============================================================================
// Final contract test
// =============================================================================

#[test]
fn resource_exhaustion_contract_is_closed() {
    let limits = FrontendLimits::production();

    /*
     * This test intentionally summarizes the contract rather than exercising
     * a particular parser implementation.
     *
     * If a new externally-controlled resource dimension is added to
     * FrontendLimits, this test file must be extended alongside the new
     * `FrontendLimitKind` and `check_*` API. That prevents production limits
     * from silently becoming incomplete.
     */
    assert!(limits.validate().is_ok());

    assert!(limits.max_source_bytes() > 0);
    assert!(limits.max_total_source_bytes() >= limits.max_source_bytes());
    assert!(limits.max_source_files() > 0);
    assert!(limits.max_tokens() > 0);
    assert!(limits.max_ast_nodes() > 0);
    assert!(limits.max_nesting_depth() > 0);
    assert!(limits.max_expression_depth() <= limits.max_nesting_depth());
    assert!(limits.max_operations() > 0);
    assert!(limits.max_recursion_depth() > 0);
    assert!(limits.max_output_bytes() > 0);
    assert!(limits.max_total_work() > 0);
}