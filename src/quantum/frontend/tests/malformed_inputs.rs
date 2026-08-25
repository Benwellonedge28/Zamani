//! Zamani Quantum Frontend — malformed/adversarial input tests.
//!
//! Production security and robustness tests for:
//!
//! ```text
//! untrusted bytes/source
//!        │
//!        ▼
//!     ImportInput
//!        │
//!        ▼
//!   OpenQasmImporter
//!        │
//!        ├── lexical failure
//!        ├── syntax failure
//!        ├── semantic failure
//!        ├── unsupported-feature failure
//!        └── resource-limit failure
//! ```
//!
//! # Purpose
//!
//! This module verifies that malformed or adversarial OpenQASM input:
//!
//! - never causes a panic;
//! - never causes an infinite parser loop;
//! - never bypasses `FrontendLimits`;
//! - never silently becomes valid source;
//! - never silently discards malformed constructs;
//! - never produces a successful invalid canonical Quantum IR;
//! - produces deterministic structured frontend failures;
//! - remains bounded by explicitly configured resource limits;
//! - preserves the generic `ImportInput` source-map boundary;
//! - remains independent of exporter, optimizer, hardware, runtime, and QPU
//!   execution.
//!
//! This file is intentionally different from:
//!
//! - `openqasm_lexer.rs` — lexical correctness;
//! - `openqasm_parser.rs` — grammar correctness;
//! - `openqasm_validation.rs` — semantic correctness;
//! - `openqasm_import.rs` — normal end-to-end import;
//! - `limits.rs` — generic limit implementation contract;
//! - `contracts.rs` — generic frontend API contracts;
//! - `openqasm_roundtrip.rs` — semantic round-trip behavior.
//!
//! This module specifically asks:
//!
//! > What happens when an attacker gives the frontend deliberately bad,
//! > truncated, pathological, oversized, deeply nested, or otherwise hostile
//! > input?
//!
//! # Security invariants
//!
//! Every test in this module is built around these invariants:
//!
//! 1. malformed source must fail deterministically;
//! 2. malformed source must never panic;
//! 3. malformed source must not be accepted merely because parsing recovered;
//! 4. resource exhaustion must become a structured limit failure;
//! 5. configured limits must not be bypassed by another frontend phase;
//! 6. invalid UTF-8 must not be represented by a false UTF-8 source map;
//! 7. source-map identity must remain consistent;
//! 8. no source construct is permission to perform I/O;
//! 9. no source construct is permission to execute code;
//! 10. no source construct is permission to access hardware;
//! 11. no malformed input may produce an invalid `QuantumCircuit`.
//!
//! # Resource-safety rule
//!
//! Tests intentionally use small custom limits wherever possible.
//!
//! This is important: adversarial tests must not depend on allocating tens of
//! megabytes merely to prove that a one-byte-over-limit input is rejected.
//! Instead, a deliberately tiny valid `FrontendLimits` configuration is used
//! to exercise the same production enforcement paths.
//!
//! # Determinism rule
//!
//! For a fixed:
//!
//! - source;
//! - source identity;
//! - `FrontendLimits`;
//! - importer configuration;
//!
//! the frontend must produce the same success/failure class and the same
//! structured error identity across repeated executions.
//!
//! Human-readable diagnostic wording may evolve only under the normal stable
//! diagnostic contract; these tests therefore avoid depending on incidental
//! message text.
//!
//! # Rust compatibility
//!
//! - Rust 2021;
//! - Rust 1.97 / 1.97.1;
//! - stable Rust only;
//! - no nightly features;
//! - no additional test dependencies;
//! - no unsafe code.
//!
//! # Integration
//!
//! Register this file from the frontend test harness:
//!
//! ```ignore
//! #[cfg(test)]
//! #[path = "tests/malformed_inputs.rs"]
//! mod malformed_inputs;
//! ```
//!
//! The exact registration location must match the repository's existing
//! frontend test-module wiring. No production module should import this file.
//!
//! # Architectural boundary
//!
//! These tests deliberately use the public frontend boundary:
//!
//! ```text
//! SourceMap
//!     │
//!     ▼
//! ImportInput
//!     │
//!     ▼
//! OpenQasmImporter
//!     │
//!     ▼
//! FrontendError / ImportOutput
//! ```
//!
//! OpenQASM lexer/parser/AST internals are not required. This means these
//! tests remain valid if the implementation is internally refactored while
//! retaining the public frontend contract.

#![allow(clippy::module_name_repetitions)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use crate::quantum::frontend::core::errors::FrontendErrorKind;
use crate::quantum::frontend::core::limits::FrontendLimits;
use crate::quantum::frontend::core::source::{SourceId, SourceMap};
use crate::quantum::frontend::importer::{
    FormatImporter,
    ImportConfig,
    ImportInput,
};
use crate::quantum::frontend::OpenQasmImporter;


// =============================================================================
// Test policy
// =============================================================================

/// Small deterministic limits used by adversarial tests.
///
/// These limits are intentionally much smaller than the production profile.
/// The purpose is to exercise the exact production limit mechanisms without
/// requiring large CI allocations.
///
/// The configuration remains internally valid according to the public
/// `FrontendLimits` builder contract.
fn adversarial_limits() -> FrontendLimits {
    FrontendLimits::builder()
        .max_source_bytes(4 * 1024)
        .max_total_source_bytes(8 * 1024)
        .max_source_files(4)
        .max_tokens(256)
        .max_identifier_length(64)
        .max_string_length(128)
        .max_numeric_literal_length(64)
        .max_comment_length(128)
        .max_annotation_length(128)
        .max_ast_nodes(512)
        .max_nesting_depth(32)
        .max_expression_depth(16)
        .max_expression_nodes(256)
        .max_diagnostics(32)
        .max_diagnostic_children(8)
        .max_diagnostic_snippet_length(512)
        .max_include_depth(8)
        .max_include_edges(16)
        .max_gate_definitions(16)
        .max_gate_operations(64)
        .max_register_size(64)
        .max_array_elements(64)
        .max_symbols(128)
        .max_parameters(16)
        .max_operands(16)
        .max_statements_per_block(64)
        .max_statements(128)
        .max_annotations_per_item(8)
        .max_operations(128)
        .max_recursion_depth(32)
        .max_output_bytes(8 * 1024)
        .max_total_work(10_000)
        .build()
        .expect("adversarial frontend limits must be internally valid")
}


// =============================================================================
// Source/input helpers
// =============================================================================

/// Builds an `ImportInput` whose source map truthfully contains the supplied
/// UTF-8 source.
///
/// This helper is intentionally the same boundary used by the normal frontend
/// integration tests: parser offsets must always refer to the source registered
/// in `SourceMap`.
fn input(
    source: &str,
) -> ImportInput {
    input_with_limits(
        source,
        adversarial_limits(),
    )
}

/// Builds an `ImportInput` with explicit frontend limits.
fn input_with_limits(
    source: &str,
    limits: FrontendLimits,
) -> ImportInput {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("malformed-input.qasm"),
            Arc::<str>::from(source),
        )
        .expect("test source must be valid UTF-8 and fit SourceMap");

    ImportInput::new(
        source_id,
        source.as_bytes().to_vec(),
        source_map,
        ImportConfig::new(limits),
    )
    .expect("test input must satisfy the generic ImportInput contract")
}

/// Builds an input while allowing the test to control the displayed source
/// name.
fn named_input(
    name: &str,
    source: &str,
    limits: FrontendLimits,
) -> ImportInput {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from(name),
            Arc::<str>::from(source),
        )
        .expect("test source must fit SourceMap");

    ImportInput::new(
        source_id,
        source.as_bytes().to_vec(),
        source_map,
        ImportConfig::new(limits),
    )
    .expect("test input must satisfy ImportInput invariants")
}


// =============================================================================
// Import execution helpers
// =============================================================================

/// Runs the importer and asserts that it does not panic.
///
/// A malformed-input test must fail the test suite if the frontend panics.
/// `catch_unwind` is used deliberately here because the production contract
/// requires malformed external input to cross the frontend boundary as a
/// structured result rather than unwinding.
///
/// `AssertUnwindSafe` is appropriate because the closure contains only local,
/// immutable test-owned state and no shared mutable compiler state.
fn import_without_panic(
    source: &str,
) -> Result<
    crate::quantum::frontend::importer::ImportOutput,
    crate::quantum::frontend::core::errors::FrontendError,
> {
    import_without_panic_with_limits(
        source,
        adversarial_limits(),
    )
}

/// Same as `import_without_panic`, but with explicit limits.
fn import_without_panic_with_limits(
    source: &str,
    limits: FrontendLimits,
) -> Result<
    crate::quantum::frontend::importer::ImportOutput,
    crate::quantum::frontend::core::errors::FrontendError,
> {
    let importer = OpenQasmImporter::production();
    let import_input = input_with_limits(source, limits);

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(import_input)
    }));

    match result {
        Ok(result) => result,

        Err(payload) => {
            panic!(
                "OpenQASM frontend panicked while processing malformed input; \
                 panic payload type: {}",
                panic_payload_type(&payload),
            );
        }
    }
}

/// Converts the panic payload into a stable type description without exposing
/// its contents. Malformed source must not make the test output depend on
/// potentially enormous or sensitive panic strings.
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

/// Runs an importer closure repeatedly and proves that its high-level result
/// classification is deterministic.
///
/// We intentionally compare only stable semantic categories rather than error
/// display strings.
fn assert_deterministic_failure(
    source: &str,
) {
    let first = import_without_panic(source);
    let second = import_without_panic(source);

    match (first, second) {
        (Err(first), Err(second)) => {
            assert_eq!(
                first.kind(),
                second.kind(),
                "same malformed input must produce the same frontend error kind",
            );

            assert_eq!(
                first.code(),
                second.code(),
                "same malformed input must produce the same stable frontend error code",
            );
        }

        (Ok(_), Ok(_)) => {
            panic!(
                "input expected to fail was accepted on both deterministic runs"
            );
        }

        (Ok(_), Err(error)) => {
            panic!(
                "same malformed input was accepted once and rejected once: \
                 second result was {}",
                error.kind(),
            );
        }

        (Err(error), Ok(_)) => {
            panic!(
                "same malformed input was rejected once and accepted once: \
                 first result was {}",
                error.kind(),
            );
        }
    }
}


// =============================================================================
// Empty and truncated input
// =============================================================================

#[test]
fn empty_source_is_rejected_without_panicking() {
    let result = import_without_panic("");

    assert!(
        result.is_err(),
        "empty OpenQASM source must not be accepted",
    );
}

#[test]
fn whitespace_only_source_is_rejected_without_panicking() {
    let result = import_without_panic(" \n\t\r\n ");

    assert!(
        result.is_err(),
        "whitespace-only source must not be accepted",
    );
}

#[test]
fn comment_only_source_is_rejected_without_panicking() {
    let result = import_without_panic(
        r#"// deliberately incomplete program
// no version declaration
"#,
    );

    assert!(
        result.is_err(),
        "comment-only source must not be accepted",
    );
}

#[test]
fn version_prefix_without_terminator_is_rejected() {
    let result = import_without_panic("OPENQASM");

    assert!(
        result.is_err(),
        "truncated version declaration must be rejected",
    );
}

#[test]
fn incomplete_version_literal_is_rejected() {
    let result = import_without_panic("OPENQASM 3");

    assert!(
        result.is_err(),
        "incomplete version literal must be rejected",
    );
}

#[test]
fn incomplete_version_statement_is_rejected() {
    let result = import_without_panic("OPENQASM 3.1");

    assert!(
        result.is_err(),
        "version declaration without semicolon must be rejected",
    );
}

#[test]
fn truncated_declaration_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit",
    );

    assert!(
        result.is_err(),
        "truncated qubit declaration must be rejected",
    );
}

#[test]
fn truncated_register_size_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[",
    );

    assert!(
        result.is_err(),
        "unterminated register-size expression must be rejected",
    );
}

#[test]
fn truncated_operation_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[1] q;\nh",
    );

    assert!(
        result.is_err(),
        "truncated gate invocation must be rejected",
    );
}

#[test]
fn truncated_measurement_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[1] q;\nbit[1] c;\nmeasure q[0]",
    );

    assert!(
        result.is_err(),
        "truncated measurement must be rejected",
    );
}


// =============================================================================
// Delimiters and lexical corruption
// =============================================================================

#[test]
fn unterminated_string_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\ninclude \"stdgates.inc;\n",
    );

    assert!(
        result.is_err(),
        "unterminated string literal must be rejected",
    );
}

#[test]
fn unterminated_block_comment_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\n/* unterminated",
    );

    assert!(
        result.is_err(),
        "unterminated block comment must be rejected",
    );
}

#[test]
fn unterminated_annotation_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\n@",
    );

    assert!(
        result.is_err(),
        "truncated annotation must not be accepted",
    );
}

#[test]
fn unmatched_open_parenthesis_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[1] q;\nrx(pi q[0];\n",
    );

    assert!(
        result.is_err(),
        "unmatched opening parenthesis must be rejected",
    );
}

#[test]
fn unmatched_close_parenthesis_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[1] q;\nrx(pi) q[0]);\n",
    );

    assert!(
        result.is_err(),
        "unmatched closing parenthesis must be rejected",
    );
}

#[test]
fn unmatched_open_bracket_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[1] q;\nx q[0;\n",
    );

    assert!(
        result.is_err(),
        "unmatched opening bracket must be rejected",
    );
}

#[test]
fn unmatched_close_bracket_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[1] q;\nx q[0]];\n",
    );

    assert!(
        result.is_err(),
        "unmatched closing bracket must be rejected",
    );
}

#[test]
fn unmatched_open_brace_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nif (true) {\n",
    );

    assert!(
        result.is_err(),
        "unterminated block must be rejected",
    );
}

#[test]
fn unmatched_close_brace_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\n}\n",
    );

    assert!(
        result.is_err(),
        "unexpected closing brace must be rejected",
    );
}


// =============================================================================
// Invalid literals
// =============================================================================

#[test]
fn malformed_decimal_literal_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nconst int x = 1.2.3;\n",
    );

    assert!(
        result.is_err(),
        "malformed numeric literal must be rejected",
    );
}

#[test]
fn malformed_exponent_literal_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nconst float x = 1e+;\n",
    );

    assert!(
        result.is_err(),
        "malformed exponent must be rejected",
    );
}

#[test]
fn malformed_hexadecimal_literal_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nconst int x = 0x;\n",
    );

    assert!(
        result.is_err(),
        "incomplete hexadecimal literal must be rejected",
    );
}

#[test]
fn malformed_binary_literal_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nconst int x = 0b;\n",
    );

    assert!(
        result.is_err(),
        "incomplete binary literal must be rejected",
    );
}

#[test]
fn malformed_duration_literal_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[1] q;\ndelay 1 q[0];\n",
    );

    /*
     * This is intentionally not asserted as a particular error kind because
     * the exact semantic interpretation of an invalid duration is owned by
     * the OpenQASM validator. The invariant is simply that malformed timing
     * syntax cannot become successful canonical IR.
     */
    assert!(
        result.is_err(),
        "invalid duration usage must be rejected or explicitly unsupported",
    );
}


// =============================================================================
// Invalid declarations
// =============================================================================

#[test]
fn declaration_without_identifier_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[1];\n",
    );

    assert!(
        result.is_err(),
        "declaration without an identifier must be rejected",
    );
}

#[test]
fn zero_sized_qubit_register_is_rejected_or_explicitly_unsupported() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[0] q;\n",
    );

    assert!(
        result.is_err(),
        "zero-sized quantum register must not produce a successful circuit",
    );
}

#[test]
fn negative_qubit_register_size_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[-1] q;\n",
    );

    assert!(
        result.is_err(),
        "negative register size must be rejected",
    );
}

#[test]
fn enormous_qubit_register_is_bounded() {
    let limits = adversarial_limits();

    let result = import_without_panic_with_limits(
        "OPENQASM 3.1;\nqubit[999999999999999999999999999999] q;\n",
        limits,
    );

    assert!(
        result.is_err(),
        "an enormous register must not become an unbounded allocation",
    );
}

#[test]
fn duplicate_qubit_declaration_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[1] q;
qubit[1] q;
"#,
    );

    assert!(
        result.is_err(),
        "duplicate declarations must be rejected",
    );
}

#[test]
fn duplicate_classical_declaration_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
bit[1] c;
bit[1] c;
"#,
    );

    assert!(
        result.is_err(),
        "duplicate declarations must be rejected",
    );
}


// =============================================================================
// Unknown identifiers and malformed operations
// =============================================================================

#[test]
fn unknown_qubit_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[1] q;
x missing[0];
"#,
    );

    assert!(
        result.is_err(),
        "unknown quantum identifier must be rejected",
    );
}

#[test]
fn unknown_classical_register_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[1] q;
bit[1] c;
measure q[0] -> missing[0];
"#,
    );

    assert!(
        result.is_err(),
        "unknown classical identifier must be rejected",
    );
}

#[test]
fn out_of_range_qubit_index_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[1] q;
x q[1];
"#,
    );

    assert!(
        result.is_err(),
        "out-of-range quantum index must be rejected",
    );
}

#[test]
fn out_of_range_classical_index_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[1] q;
bit[1] c;
measure q[0] -> c[1];
"#,
    );

    assert!(
        result.is_err(),
        "out-of-range classical index must be rejected",
    );
}

#[test]
fn unknown_gate_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[1] q;
definitely_not_a_gate q[0];
"#,
    );

    assert!(
        result.is_err(),
        "unknown gate must not be silently accepted",
    );
}

#[test]
fn malformed_gate_argument_list_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[2] q;
cx q[0];
"#,
    );

    assert!(
        result.is_err(),
        "wrong gate arity must be rejected",
    );
}

#[test]
fn malformed_gate_parameter_list_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[1] q;
rx q[0];
"#,
    );

    assert!(
        result.is_err(),
        "missing required gate parameters must be rejected",
    );
}

#[test]
fn malformed_measurement_target_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[1] q;
measure q[0] -> q[0];
"#,
    );

    assert!(
        result.is_err(),
        "measurement into a quantum target must be rejected",
    );
}


// =============================================================================
// Broadcast/resource-shape attacks
// =============================================================================

#[test]
fn incompatible_broadcast_dimensions_are_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[2] a;
qubit[3] b;
cx a, b;
"#,
    );

    assert!(
        result.is_err(),
        "incompatible gate broadcast dimensions must be rejected",
    );
}

#[test]
fn huge_broadcast_request_is_bounded() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(8 * 1024)
        .max_total_source_bytes(16 * 1024)
        .max_source_files(4)
        .max_tokens(1_024)
        .max_identifier_length(128)
        .max_string_length(128)
        .max_numeric_literal_length(128)
        .max_comment_length(128)
        .max_annotation_length(128)
        .max_ast_nodes(2_048)
        .max_nesting_depth(64)
        .max_expression_depth(32)
        .max_expression_nodes(1_024)
        .max_diagnostics(64)
        .max_diagnostic_children(8)
        .max_diagnostic_snippet_length(512)
        .max_include_depth(8)
        .max_include_edges(32)
        .max_gate_definitions(32)
        .max_gate_operations(128)
        .max_register_size(128)
        .max_array_elements(128)
        .max_symbols(256)
        .max_parameters(32)
        .max_operands(32)
        .max_statements_per_block(128)
        .max_statements(256)
        .max_annotations_per_item(8)
        .max_operations(256)
        .max_recursion_depth(64)
        .max_output_bytes(16 * 1024)
        .max_total_work(20_000)
        .build()
        .expect("broadcast attack limits must be valid");

    let source = r#"OPENQASM 3.1;
qubit[128] a;
qubit[128] b;
cx a, b;
"#;

    let result = import_without_panic_with_limits(
        source,
        limits,
    );

    /*
     * Depending on the importer representation, equal dimensions may be
     * representable. Therefore this test does not require rejection solely
     * because the registers are large.
     *
     * The security property is that the operation remains bounded by the
     * configured operation/work limits and never causes a panic.
     */
    if let Ok(output) = result {
        assert!(
            output.circuit().validate().is_ok(),
            "successful bounded broadcast import must still produce valid IR",
        );
    }
}


// =============================================================================
// Deep nesting
// =============================================================================

/// Constructs a syntactically nested classical-control program.
///
/// The generated source is intentionally limited by the test itself; it is
/// not intended to be a benchmark. Its purpose is to cross the configured
/// parser nesting boundary deterministically.
fn deeply_nested_if(
    depth: usize,
) -> String {
    let mut source = String::from(
        "OPENQASM 3.1;\n",
    );

    source.push_str(
        "bit[1] c;\n",
    );

    for _ in 0..depth {
        source.push_str(
            "if (true) {\n",
        );
    }

    source.push_str(
        ";\n",
    );

    for _ in 0..depth {
        source.push_str(
            "}\n",
        );
    }

    source
}

#[test]
fn deeply_nested_control_flow_does_not_panic() {
    let source = deeply_nested_if(256);

    let result = import_without_panic_with_limits(
        &source,
        adversarial_limits(),
    );

    assert!(
        result.is_err(),
        "nesting beyond the configured adversarial depth must not be accepted",
    );
}

#[test]
fn moderately_nested_control_flow_is_still_bounded() {
    let source = deeply_nested_if(64);

    let result = import_without_panic_with_limits(
        &source,
        adversarial_limits(),
    );

    /*
     * The test intentionally does not require a particular failure mechanism:
     * a parser may reject the construct syntactically, a validator may reject
     * it semantically, or the configured nesting/recursion limit may reject it.
     *
     * The invariant is that it cannot escape into an unbounded execution path.
     */
    if let Err(error) = result {
        assert!(
            matches!(
                error.kind(),
                FrontendErrorKind::Lexical
                    | FrontendErrorKind::Syntax
                    | FrontendErrorKind::Semantic
                    | FrontendErrorKind::LimitExceeded
                    | FrontendErrorKind::Unsupported
                    | FrontendErrorKind::Import
                    | FrontendErrorKind::Lowering
                    | FrontendErrorKind::Internal
            ),
            "frontend must return a structured error kind",
        );
    }
}


// =============================================================================
// Expression-depth attacks
// =============================================================================

/// Builds a deeply nested parenthesized expression.
///
/// This intentionally avoids allocating recursively in Rust; the generated
/// source is a flat `String`, allowing the parser itself to be the component
/// under test.
fn deeply_nested_expression(
    depth: usize,
) -> String {
    let mut source = String::from(
        "OPENQASM 3.1;\n",
    );

    source.push_str(
        "qubit[1] q;\n",
    );

    source.push_str(
        "rx(",
    );

    for _ in 0..depth {
        source.push('(');
    }

    source.push_str("0");

    for _ in 0..depth {
        source.push(')');
    }

    source.push_str(
        ") q[0];\n",
    );

    source
}

#[test]
fn deeply_nested_expression_does_not_panic() {
    let source = deeply_nested_expression(512);

    let result = import_without_panic_with_limits(
        &source,
        adversarial_limits(),
    );

    assert!(
        result.is_err(),
        "expression nesting beyond the configured depth must not be accepted",
    );
}


// =============================================================================
// Identifier and literal exhaustion
// =============================================================================

#[test]
fn oversized_identifier_is_rejected_before_unbounded_symbol_growth() {
    let limits = adversarial_limits();

    let identifier = "q".repeat(
        usize::try_from(
            limits.max_identifier_length(),
        )
        .expect("test limit must fit usize")
            + 1,
    );

    let source = format!(
        "OPENQASM 3.1;\nqubit[1] {identifier};\n",
    );

    let result = import_without_panic_with_limits(
        &source,
        limits,
    );

    assert!(
        result.is_err(),
        "identifier longer than max_identifier_length must be rejected",
    );
}

#[test]
fn oversized_numeric_literal_is_rejected_or_not_accepted() {
    let limits = adversarial_limits();

    let digits = "9".repeat(
        usize::try_from(
            limits.max_numeric_literal_length(),
        )
        .expect("test limit must fit usize")
            + 1,
    );

    let source = format!(
        "OPENQASM 3.1;\nconst int x = {digits};\n",
    );

    let result = import_without_panic_with_limits(
        &source,
        limits,
    );

    assert!(
        result.is_err(),
        "oversized numeric literal must not be accepted",
    );
}

#[test]
fn oversized_comment_is_bounded() {
    let limits = adversarial_limits();

    let comment = "x".repeat(
        usize::try_from(
            limits.max_comment_length(),
        )
        .expect("test limit must fit usize")
            + 1,
    );

    let source = format!(
        "OPENQASM 3.1;\n// {comment}\nqubit[1] q;\nx q[0];\n",
    );

    let result = import_without_panic_with_limits(
        &source,
        limits,
    );

    /*
     * The comment itself must never force unbounded comment storage. The
     * importer may reject at the lexical limit or otherwise reject the source.
     * It must not panic.
     */
    assert!(
        result.is_err(),
        "comment exceeding the configured lexical limit must be rejected",
    );
}

#[test]
fn oversized_string_literal_is_bounded() {
    let limits = adversarial_limits();

    let content = "x".repeat(
        usize::try_from(
            limits.max_string_length(),
        )
        .expect("test limit must fit usize")
            + 1,
    );

    let source = format!(
        "OPENQASM 3.1;\ninclude \"{content}\";\n",
    );

    let result = import_without_panic_with_limits(
        &source,
        limits,
    );

    assert!(
        result.is_err(),
        "oversized string literal must be bounded",
    );
}


// =============================================================================
// Token/AST/statement exhaustion
// =============================================================================

#[test]
fn excessive_statement_count_is_bounded() {
    let limits = adversarial_limits();

    let mut source = String::from(
        "OPENQASM 3.1;\nqubit[1] q;\n",
    );

    /*
     * Repeated operations are intentionally syntactically valid so that this
     * exercises statement/token/operation limits rather than merely a parser
     * syntax error.
     */
    for _ in 0..512 {
        source.push_str(
            "x q[0];\n",
        );
    }

    let result = import_without_panic_with_limits(
        &source,
        limits,
    );

    assert!(
        result.is_err(),
        "input exceeding statement/operation/token budgets must be rejected",
    );
}

#[test]
fn excessive_operation_count_is_bounded() {
    let limits = adversarial_limits();

    let mut source = String::from(
        "OPENQASM 3.1;\nqubit[1] q;\n",
    );

    for _ in 0..512 {
        source.push_str(
            "h q[0];\n",
        );
    }

    let result = import_without_panic_with_limits(
        &source,
        limits,
    );

    assert!(
        result.is_err(),
        "operation expansion must remain bounded",
    );
}

#[test]
fn excessive_tokens_are_bounded() {
    let limits = adversarial_limits();

    let mut source = String::from(
        "OPENQASM 3.1;\nqubit[1] q;\n",
    );

    for _ in 0..512 {
        source.push_str(
            "x q[0];\n",
        );
    }

    let result = import_without_panic_with_limits(
        &source,
        limits,
    );

    assert!(
        result.is_err(),
        "token exhaustion must become a frontend failure",
    );
}


// =============================================================================
// Include/import boundary attacks
// =============================================================================

#[test]
fn arbitrary_include_path_is_not_a_permission_to_access_filesystem() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
include "../../../../../../etc/passwd";
"#,
    );

    assert!(
        result.is_err(),
        "arbitrary include paths must not be silently accepted by the \\
         production OpenQASM importer",
    );
}

#[test]
fn absolute_include_path_is_rejected_or_explicitly_unsupported() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
include "/etc/passwd";
"#,
    );

    assert!(
        result.is_err(),
        "absolute include paths must not become implicit filesystem access",
    );
}

#[test]
fn include_with_path_traversal_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
include "../secrets.qasm";
"#,
    );

    assert!(
        result.is_err(),
        "path traversal must not be accepted as an implicit include",
    );
}

#[test]
fn malformed_include_string_is_rejected() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
include "../unterminated;
"#,
    );

    assert!(
        result.is_err(),
        "malformed include must be rejected",
    );
}


// =============================================================================
// Unsupported/execution-sensitive constructs
// =============================================================================

#[test]
fn extern_declaration_never_becomes_execution_permission() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
extern foo(int);
"#,
    );

    /*
     * The exact policy may be "unsupported" or "invalid". Both are safe.
     * Successful import would require the frontend to have a complete,
     * side-effect-free representation of the declaration.
     */
    assert!(
        result.is_err(),
        "extern declarations must not silently become executable behavior",
    );
}

#[test]
fn calibration_constructs_do_not_execute_during_import() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
cal {
}
"#,
    );

    assert!(
        result.is_err(),
        "calibration source must never be executed by the frontend",
    );
}

#[test]
fn defcal_constructs_do_not_execute_during_import() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
defcal foo $0 {
}
"#,
    );

    assert!(
        result.is_err(),
        "defcal source must never execute during frontend import",
    );
}

#[test]
fn malformed_pragma_does_not_escape_as_execution() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
#pragma execute arbitrary-command
"#,
    );

    assert!(
        result.is_err(),
        "pragma data must never become process/network/hardware execution",
    );
}


// =============================================================================
// Version corruption
// =============================================================================

#[test]
fn malformed_version_keyword_is_rejected() {
    let result = import_without_panic(
        "OPENQASX 3.1;\n",
    );

    assert!(
        result.is_err(),
        "misspelled language version keyword must be rejected",
    );
}

#[test]
fn unsupported_major_version_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 999.0;\nqubit[1] q;\n",
    );

    let error = result.expect_err(
        "unsupported OpenQASM major version must be rejected",
    );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::Unsupported,
    );
}

#[test]
fn unsupported_future_minor_version_is_rejected() {
    let result = import_without_panic(
        "OPENQASM 3.999;\nqubit[1] q;\n",
    );

    let error = result.expect_err(
        "unsupported future OpenQASM minor version must not be \
         silently accepted",
    );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::Unsupported,
    );
}


// =============================================================================
// Invalid UTF-8 / byte-boundary safety
// =============================================================================

#[test]
fn malformed_utf8_cannot_cross_the_source_map_boundary() {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("malformed-utf8.qasm"),
            Arc::<str>::from("placeholder"),
        )
        .expect("placeholder source must be valid UTF-8");

    /*
     * `ImportInput` requires the registered source to correspond to the bytes
     * used by the importer. Because `SourceMap` stores UTF-8 text, malformed
     * UTF-8 must be rejected at this boundary rather than represented by a
     * dishonest source file.
     */
    let malformed_bytes = vec![
        b'O',
        b'P',
        b'E',
        b'N',
        0xff,
        0xfe,
        b';',
    ];

    let result = ImportInput::new(
        source_id,
        malformed_bytes,
        source_map,
        ImportConfig::new(adversarial_limits()),
    );

    assert!(
        result.is_err(),
        "malformed UTF-8 must not cross the generic source-map boundary",
    );
}

#[test]
fn source_map_identity_mismatch_is_rejected() {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("truth.qasm"),
            Arc::<str>::from(
                "OPENQASM 3.1;\nqubit[1] q;\n",
            ),
        )
        .expect("source must be valid");

    let result = ImportInput::new(
        source_id,
        b"OPENQASM 3.1;\nqubit[999] attacker_controlled;\n"
            .to_vec(),
        source_map,
        ImportConfig::new(adversarial_limits()),
    );

    assert!(
        result.is_err(),
        "source bytes and source-map text must never disagree",
    );
}


// =============================================================================
// Error classification
// =============================================================================

#[test]
fn malformed_syntax_is_classified_as_parse_failure() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[1] q\n",
    );

    let error = result.expect_err(
        "missing declaration terminator must fail",
    );

    assert!(
        error.kind().is_parse_failure(),
        "malformed syntax must remain classified as a parse failure",
    );
}

#[test]
fn malformed_semantics_are_classified_as_semantic_failure_when_reached() {
    let result = import_without_panic(
        r#"OPENQASM 3.1;
qubit[1] q;
x missing[0];
"#,
    );

    let error = result.expect_err(
        "unknown identifier must fail",
    );

    assert!(
        error.kind().is_semantic_failure()
            || error.kind().is_parse_failure(),
        "malformed program must never become successful IR",
    );
}

#[test]
fn resource_exhaustion_is_classified_as_limit_failure_when_limit_is_reached() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(128)
        .max_total_source_bytes(256)
        .max_source_files(2)
        .max_tokens(32)
        .max_identifier_length(32)
        .max_string_length(32)
        .max_numeric_literal_length(32)
        .max_comment_length(32)
        .max_annotation_length(32)
        .max_ast_nodes(64)
        .max_nesting_depth(8)
        .max_expression_depth(8)
        .max_expression_nodes(32)
        .max_diagnostics(8)
        .max_diagnostic_children(4)
        .max_diagnostic_snippet_length(128)
        .max_include_depth(4)
        .max_include_edges(4)
        .max_gate_definitions(4)
        .max_gate_operations(8)
        .max_register_size(8)
        .max_array_elements(8)
        .max_symbols(16)
        .max_parameters(4)
        .max_operands(4)
        .max_statements_per_block(8)
        .max_statements(16)
        .max_annotations_per_item(4)
        .max_operations(8)
        .max_recursion_depth(8)
        .max_output_bytes(256)
        .max_total_work(64)
        .build()
        .expect("limit configuration must be valid");

    let source = format!(
        "OPENQASM 3.1;\nqubit[1] q;\n{}",
        "x q[0];\n".repeat(32),
    );

    let result = import_without_panic_with_limits(
        &source,
        limits,
    );

    let error = result.expect_err(
        "the deliberately tiny frontend budget must be exhausted",
    );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::LimitExceeded,
        "resource exhaustion must cross the structured limit-error boundary",
    );
}


// =============================================================================
// Deterministic rejection
// =============================================================================

#[test]
fn empty_input_rejection_is_deterministic() {
    assert_deterministic_failure("");
}

#[test]
fn malformed_version_rejection_is_deterministic() {
    assert_deterministic_failure(
        "OPENQASM 3.999;\nqubit[1] q;\n",
    );
}

#[test]
fn malformed_syntax_rejection_is_deterministic() {
    assert_deterministic_failure(
        "OPENQASM 3.1;\nqubit[1] q\n",
    );
}

#[test]
fn malformed_semantics_rejection_is_deterministic() {
    assert_deterministic_failure(
        r#"OPENQASM 3.1;
qubit[1] q;
x unknown[0];
"#,
    );
}

#[test]
fn oversized_identifier_rejection_is_deterministic() {
    let limits = adversarial_limits();

    let identifier = "x".repeat(
        usize::try_from(
            limits.max_identifier_length(),
        )
        .expect("test limit must fit usize")
            + 1,
    );

    let source = format!(
        "OPENQASM 3.1;\nqubit[1] {identifier};\n",
    );

    let first = import_without_panic_with_limits(
        &source,
        limits,
    );

    let second = import_without_panic_with_limits(
        &source,
        limits,
    );

    match (first, second) {
        (Err(first), Err(second)) => {
            assert_eq!(
                first.kind(),
                second.kind(),
            );

            assert_eq!(
                first.code(),
                second.code(),
            );

            assert!(
                first.kind().is_limit_failure()
                    || first.kind().is_parse_failure()
                    || first.kind().is_semantic_failure(),
                "oversized identifier must fail through a structured frontend category",
            );
        }

        _ => {
            panic!(
                "oversized identifier must deterministically fail"
            );
        }
    }
}


// =============================================================================
// Repeated hostile corpus
// =============================================================================

/// A compact corpus of malformed programs.
///
/// Keeping these cases as plain `&str` values makes it easy to add regression
/// cases without changing the test harness.
fn malformed_corpus() -> &'static [&'static str] {
    &[
        "",
        " ",
        "OPENQASM",
        "OPENQASM 3",
        "OPENQASM 3.1",
        "OPENQASM 3.1\n",
        "OPENQASX 3.1;",
        "OPENQASM 4.0;",
        "OPENQASM 3.999;",
        "OPENQASM 3.1; qubit",
        "OPENQASM 3.1; qubit[",
        "OPENQASM 3.1; qubit[-1] q;",
        "OPENQASM 3.1; qubit[0] q;",
        "OPENQASM 3.1; x;",
        "OPENQASM 3.1; x q;",
        "OPENQASM 3.1; x unknown[0];",
        "OPENQASM 3.1; measure;",
        "OPENQASM 3.1; measure q[0];",
        "OPENQASM 3.1; /*",
        "OPENQASM 3.1; include \"",
        "OPENQASM 3.1; if (true) {",
        "OPENQASM 3.1; }",
        "OPENQASM 3.1; (",
        "OPENQASM 3.1; [",
        "OPENQASM 3.1; ]",
        "OPENQASM 3.1; rx( q[0];",
    ]
}

#[test]
fn every_malformed_corpus_entry_is_panic_free() {
    for (index, source) in malformed_corpus().iter().enumerate() {
        let result = catch_unwind(AssertUnwindSafe(|| {
            let importer = OpenQasmImporter::production();

            let import_input = input(source);

            importer.import(import_input)
        }));

        assert!(
            result.is_ok(),
            "malformed corpus entry {index} caused frontend panic: {:?}",
            source,
        );

        let result = result
            .expect("panic result already asserted to be absent");

        assert!(
            result.is_err(),
            "malformed corpus entry {index} was unexpectedly accepted: {:?}",
            source,
        );
    }
}

#[test]
fn malformed_corpus_is_deterministic() {
    for (index, source) in malformed_corpus().iter().enumerate() {
        let first = import_without_panic(source);
        let second = import_without_panic(source);

        match (first, second) {
            (Err(first), Err(second)) => {
                assert_eq!(
                    first.kind(),
                    second.kind(),
                    "corpus entry {index} changed error kind between runs",
                );

                assert_eq!(
                    first.code(),
                    second.code(),
                    "corpus entry {index} changed error code between runs",
                );
            }

            _ => {
                panic!(
                    "malformed corpus entry {index} did not deterministically reject: {:?}",
                    source,
                );
            }
        }
    }
}


// =============================================================================
// Public-boundary safety
// =============================================================================

#[test]
fn production_importer_remains_the_only_execution_boundary_under_test() {
    fn assert_generic_importer<I: FormatImporter>() {}

    assert_generic_importer::<OpenQasmImporter>();

    /*
     * This compile-time assertion is intentional. If OpenQASM stops
     * implementing the generic importer contract, malformed-input tests should
     * fail at compilation rather than silently becoming implementation-local
     * parser tests.
     */
}

#[test]
fn malformed_input_never_returns_successful_invalid_ir() {
    let malformed_sources = [
        "OPENQASM 3.1;\nqubit[1] q;\nx unknown[0];\n",
        "OPENQASM 3.1;\nqubit[1] q;\ncx q[0];\n",
        "OPENQASM 3.1;\nqubit[1] q;\nmeasure q[0] -> q[0];\n",
        "OPENQASM 3.1;\nqubit[-1] q;\n",
        "OPENQASM 3.1;\nqubit[1] q;\n",
    ];

    for source in malformed_sources {
        let result = import_without_panic(source);

        if let Ok(output) = result {
            /*
             * A source may be syntactically/semantically valid despite looking
             * suspicious to a generic malformed corpus. Therefore, if import
             * succeeds, the only acceptable outcome is a valid canonical IR.
             */
            assert!(
                output.circuit().validate().is_ok(),
                "successful frontend result must always be valid canonical Quantum IR",
            );
        }
    }
}


// =============================================================================
// Source-name/path safety
// =============================================================================

#[test]
fn hostile_source_name_does_not_change_parser_behavior() {
    let source = r#"OPENQASM 3.1;
qubit[1] q;
x unknown[0];
"#;

    let safe = named_input(
        "program.qasm",
        source,
        adversarial_limits(),
    );

    let hostile = named_input(
        "../../../../../../etc/passwd",
        source,
        adversarial_limits(),
    );

    let importer = OpenQasmImporter::production();

    let safe_result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(safe)
    }))
    .expect("safe source name must not panic");

    let hostile_result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(hostile)
    }))
    .expect("hostile source name must not panic");

    assert!(
        safe_result.is_err(),
        "invalid program must remain invalid with a normal source name",
    );

    assert!(
        hostile_result.is_err(),
        "invalid program must remain invalid with a hostile source name",
    );

    assert_eq!(
        safe_result
            .expect_err("safe source should fail")
            .kind(),
        hostile_result
            .expect_err("hostile source should fail")
            .kind(),
        "source display name must not alter semantic classification",
    );
}


// =============================================================================
// Final production invariant
// =============================================================================

#[test]
fn malformed_input_contract_is_closed() {
    /*
     * This test intentionally summarizes the security contract in executable
     * assertions. It is a guard against future maintainers weakening this
     * module into ordinary parser examples.
     */
    let malformed = [
        "",
        "OPENQASM",
        "OPENQASM 3",
        "OPENQASM 3.1",
        "OPENQASX 3.1;",
        "OPENQASM 4.0;",
        "OPENQASM 3.1; qubit[",
        "OPENQASM 3.1; /*",
        "OPENQASM 3.1; include \"",
        "OPENQASM 3.1; qubit[-1] q;",
        "OPENQASM 3.1; x unknown[0];",
        "OPENQASM 3.1; cx q[0];",
    ];

    for source in malformed {
        let result = catch_unwind(AssertUnwindSafe(|| {
            OpenQasmImporter::production()
                .import(input(source))
        }));

        assert!(
            result.is_ok(),
            "frontend must not panic on malformed input: {source:?}",
        );

        let result = result
            .expect("panic already ruled out");

        assert!(
            result.is_err(),
            "malformed input must not be accepted: {source:?}",
        );
    }
}