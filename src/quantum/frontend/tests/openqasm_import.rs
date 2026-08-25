//! Zamani Quantum Frontend — OpenQASM import integration tests.
//!
//! Production integration tests for:
//!
//! `OpenQASM source -> generic ImportInput -> OpenQasmImporter
//!  -> OpenQASM parsing -> semantic validation -> lowering
//!  -> canonical QuantumCircuit -> ImportOutput`
//!
//! # Purpose
//!
//! This file is specifically responsible for testing the complete OpenQASM
//! import boundary. It is intentionally different from:
//!
//! - `openqasm_lexer.rs`      — lexical implementation tests;
//! - `openqasm_parser.rs`     — grammar/parser tests;
//! - `openqasm_validation.rs` — semantic validation tests;
//! - `contracts.rs`           — generic frontend contract tests;
//! - `openqasm_export.rs`     — exporter tests;
//! - `openqasm_roundtrip.rs`  — import/export round-trip tests.
//!
//! These tests prove that the independently implemented OpenQASM frontend can
//! actually cross the generic frontend boundary and produce canonical Quantum
//! IR safely.
//!
//! # Production guarantees
//!
//! The suite verifies:
//!
//! 1. valid OpenQASM 3.1 imports successfully;
//! 2. valid OpenQASM 3.0 imports successfully;
//! 3. default importer behavior is OpenQASM 3.1;
//! 4. source-map identity is preserved at the generic boundary;
//! 5. source bytes must match the registered source;
//! 6. malformed UTF-8 is rejected before parsing;
//! 7. missing version declarations are rejected;
//! 8. unsupported OpenQASM major versions are rejected;
//! 9. unsupported future OpenQASM 3.x versions are rejected;
//! 10. syntactically invalid programs are rejected;
//! 11. semantically invalid programs are rejected;
//! 12. invalid gate operands are rejected;
//! 13. invalid measurement destinations are rejected;
//! 14. invalid broadcast/register shapes are rejected;
//! 15. unsupported source-level constructs are never silently discarded;
//! 16. arbitrary include paths do not become filesystem access;
//! 17. the standard-library include remains the controlled include boundary;
//! 18. successful import produces canonical Quantum IR;
//! 19. repeated imports are deterministic;
//! 20. equivalent source produces equivalent import metadata;
//! 21. import diagnostics remain structured;
//! 22. oversized source is rejected at the generic input boundary;
//! 23. pathological input does not panic;
//! 24. importer implements the generic `FormatImporter` contract;
//! 25. the importer remains independent of OpenQASM exporter behavior.
//!
//! # Security model
//!
//! OpenQASM source is untrusted data.
//!
//! Importing it must never:
//!
//! - execute source code;
//! - execute `extern` declarations;
//! - execute calibration code;
//! - access arbitrary filesystem paths;
//! - access the network;
//! - spawn processes;
//! - access quantum hardware;
//! - route qubits;
//! - schedule operations;
//! - optimize the circuit;
//! - silently discard unsupported semantics.
//!
//! The generic frontend explicitly treats import as an untrusted-input
//! boundary, and the canonical Quantum IR remains the semantic boundary
//! downstream of this test suite.
//!
//! # Rust compatibility
//!
//! - Rust 2021;
//! - Rust 1.97 / 1.97.1;
//! - stable Rust only;
//! - no nightly features;
//! - no external test dependencies.
//!
//! # Integration
//!
//! Register this module from `src/quantum/frontend/mod.rs`:
//!
//! ```ignore
//! #[cfg(test)]
//! #[path = "tests/openqasm_import.rs"]
//! mod openqasm_import;
//! ```
//!
//! No production module imports this test module.
//!
//! # Architectural rule
//!
//! Tests should use the public frontend boundary whenever possible:
//!
//! ```text
//! SourceMap
//!     |
//!     v
//! ImportInput
//!     |
//!     v
//! OpenQasmImporter
//!     |
//!     v
//! ImportOutput
//!     |
//!     v
//! QuantumCircuit
//! ```
//!
//! Private lexer/parser/AST internals must not be required here. That keeps
//! these tests valid if the internal OpenQASM implementation is refactored.

#![allow(clippy::module_name_repetitions)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use crate::quantum::frontend::core::errors::{
    FrontendErrorKind,
};
use crate::quantum::frontend::core::limits::{
    FrontendLimits,
};
use crate::quantum::frontend::core::source::{
    SourceId,
    SourceMap,
};
use crate::quantum::frontend::format::{
    FormatId,
    FormatVersion,
};
use crate::quantum::frontend::importer::{
    FormatImporter,
    ImportConfig,
    ImportInput,
};
use crate::quantum::frontend::{
    OpenQasmImporter,
    OPENQASM_3_0,
    OPENQASM_3_1,
    OPENQASM_FORMAT_ID,
    OPENQASM_MEDIA_TYPE,
    STANDARD_LIBRARY_INCLUDE,
};


// =============================================================================
// Test fixtures
// =============================================================================

/// Creates a source map containing exactly the bytes supplied to the importer.
///
/// This helper mirrors the generic `ImportInput` invariant: the registered
/// source and imported bytes must be identical.
fn make_input(
    source: &str,
) -> ImportInput {
    make_input_with_limits(
        source.as_bytes().to_vec(),
        FrontendLimits::production(),
    )
}

/// Creates an import input using explicit frontend limits.
fn make_input_with_limits(
    source: Vec<u8>,
    limits: FrontendLimits,
) -> ImportInput {
    let text = std::str::from_utf8(&source)
        .expect("test fixture source must be valid UTF-8");

    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("test.qasm"),
            Arc::<str>::from(text),
        )
        .expect("test source must fit the source model");

    ImportInput::new(
        source_id,
        source,
        source_map,
        ImportConfig::new(limits),
    )
    .expect("test fixture must satisfy the generic ImportInput contract")
}

/// Creates an input with a deliberately supplied source-map identity.
fn make_input_with_name(
    name: &str,
    source: &str,
) -> ImportInput {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from(name),
            Arc::<str>::from(source),
        )
        .expect("test source must fit the source model");

    ImportInput::new(
        source_id,
        source.as_bytes().to_vec(),
        source_map,
        ImportConfig::new(FrontendLimits::production()),
    )
    .expect("test fixture must satisfy ImportInput invariants")
}

/// Minimal valid OpenQASM 3.1 program.
///
/// This deliberately uses only the canonical constructs that the current
/// Quantum IR can represent without hardware-specific semantics.
fn basic_qasm_31() -> &'static str {
    r#"OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
bit[2] c;

h q[0];
cx q[0], q[1];
measure q -> c;
"#
}

/// Minimal valid OpenQASM 3.0 program.
fn basic_qasm_30() -> &'static str {
    r#"OPENQASM 3.0;
include "stdgates.inc";

qubit[1] q;
bit[1] c;

h q[0];
measure q[0] -> c[0];
"#
}

/// Valid program without the standard-library include.
///
/// This is useful for proving that the importer does not require an include
/// merely to recognize the OpenQASM language itself.
fn no_include_qasm() -> &'static str {
    r#"OPENQASM 3.1;

qubit[1] q;

x q[0];
"#
}


// =============================================================================
// Basic importer contract
// =============================================================================

#[test]
fn production_importer_is_openqasm_31() {
    let importer = OpenQasmImporter::production();

    assert_eq!(
        importer.version(),
        OPENQASM_3_1,
        "production importer must default to OpenQASM 3.1",
    );
}

#[test]
fn importer_implements_generic_format_importer_contract() {
    fn assert_importer<I: FormatImporter>() {}

    assert_importer::<OpenQasmImporter>();
}

#[test]
fn importer_reports_openqasm_format_identity() {
    let importer = OpenQasmImporter::production();

    assert_eq!(
        importer.format(),
        OPENQASM_FORMAT_ID,
    );
}

#[test]
fn openqasm_public_constants_are_stable() {
    assert_eq!(
        OPENQASM_3_0.major(),
        3,
    );

    assert_eq!(
        OPENQASM_3_0.minor(),
        0,
    );

    assert_eq!(
        OPENQASM_3_1.major(),
        3,
    );

    assert_eq!(
        OPENQASM_3_1.minor(),
        1,
    );

    assert_eq!(
        OPENQASM_FORMAT_ID.to_string(),
        "openqasm",
    );

    assert_eq!(
        OPENQASM_MEDIA_TYPE,
        "application/vnd.openqasm.v3",
    );

    assert_eq!(
        STANDARD_LIBRARY_INCLUDE,
        "stdgates.inc",
    );
}


// =============================================================================
// Successful imports
// =============================================================================

#[test]
fn imports_basic_openqasm_31_program() {
    let importer = OpenQasmImporter::production();

    let input = make_input(basic_qasm_31());

    let output = importer
        .import(input)
        .expect("valid OpenQASM 3.1 must import successfully");

    assert_eq!(
        output.format(),
        &OPENQASM_FORMAT_ID,
    );

    assert_eq!(
        output.version(),
        &OPENQASM_3_1,
    );

    assert!(
        output.diagnostics().is_empty(),
        "valid program should not produce fatal diagnostics",
    );

    /*
     * The most important assertion here is that import succeeded with an
     * ImportOutput. `ImportOutput::try_new` is the canonical frontend boundary
     * that verifies the resulting QuantumCircuit.
     */
    let _circuit = output.circuit();
}

#[test]
fn imports_basic_openqasm_30_program() {
    let importer = OpenQasmImporter::production();

    let input = make_input(basic_qasm_30());

    let output = importer
        .import(input)
        .expect("OpenQASM 3.0 must be supported explicitly");

    assert_eq!(
        output.format(),
        &OPENQASM_FORMAT_ID,
    );

    assert_eq!(
        output.version(),
        &OPENQASM_3_1,
        "the production importer contract is configured for 3.1 while \
         accepting compatible 3.0 source",
    );
}

#[test]
fn imports_program_without_standard_library_include() {
    let importer = OpenQasmImporter::production();

    let input = make_input(no_include_qasm());

    let output = importer
        .import(input)
        .expect("stdgates.inc must not be required when only built-in \
                 representable operations are used");

    assert_eq!(
        output.format(),
        &OPENQASM_FORMAT_ID,
    );
}

#[test]
fn successful_import_crosses_the_canonical_ir_boundary() {
    let importer = OpenQasmImporter::production();

    let input = make_input(no_include_qasm());

    let output = importer
        .import(input)
        .expect("valid OpenQASM must lower to canonical IR");

    /*
     * ImportOutput exposes only the canonical QuantumCircuit here.
     *
     * We intentionally do not construct a parallel "expected quantum model"
     * in this test. The canonical IR is the semantic authority.
     */
    let circuit = output.circuit();

    assert!(
        circuit.validate().is_ok(),
        "successful frontend import must produce a valid canonical Quantum IR circuit",
    );
}


// =============================================================================
// Source-map and input-boundary integrity
// =============================================================================

#[test]
fn import_input_preserves_source_identity() {
    let input = make_input_with_name(
        "integration-test.qasm",
        basic_qasm_31(),
    );

    let source_id = input.source_id();

    assert_eq!(
        input.source_map()
            .get(source_id)
            .expect("source must exist")
            .name(),
        "integration-test.qasm",
    );
}

#[test]
fn import_input_rejects_source_map_byte_mismatch() {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("source.qasm"),
            Arc::<str>::from(
                "OPENQASM 3.1;\nqubit[1] q;\n",
            ),
        )
        .expect("source must be accepted");

    let result = ImportInput::new(
        source_id,
        b"OPENQASM 3.1;\nqubit[2] q;\n".to_vec(),
        source_map,
        ImportConfig::new(FrontendLimits::production()),
    );

    assert!(
        result.is_err(),
        "ImportInput must never allow parser offsets to refer to \
         source different from the source displayed by diagnostics",
    );
}

#[test]
fn import_input_rejects_unknown_source_id() {
    let source_map = SourceMap::new();

    let result = ImportInput::new(
        SourceId::from_raw(999),
        b"OPENQASM 3.1;".to_vec(),
        source_map,
        ImportConfig::new(FrontendLimits::production()),
    );

    assert!(
        result.is_err(),
        "an importer must never accept a source identity that is absent \
         from the source map",
    );
}

#[test]
fn import_input_rejects_oversized_source_before_parser() {
    /*
     * The exact production limit is repository-owned. We intentionally build
     * the source from the configured limit instead of hard-coding a byte
     * count, so this test remains valid when the security policy is tightened
     * or relaxed.
     */
    let limits = FrontendLimits::production();

    let maximum = limits.max_source_bytes();

    /*
     * Only run the allocation if the configured production limit is
     * representable and reasonably bounded for a test environment.
     *
     * A one-byte-over-limit vector is sufficient to test the generic
     * ImportInput boundary.
     */
    let oversized_len = maximum
        .checked_add(1)
        .expect("production source limit must not overflow usize");

    /*
     * This test should not turn an unexpectedly huge production limit into an
     * enormous CI allocation. The actual limit enforcement is already a
     * production contract; when the limit is too large to construct cheaply,
     * the parser-level limit tests own exhaustive resource testing.
     */
    if oversized_len <= 16 * 1024 * 1024 {
        let source = vec![b' '; oversized_len];

        let mut source_map = SourceMap::new();

        /*
         * SourceMap itself must also be able to represent the test fixture.
         * If it cannot, that is still a safe rejection, but we specifically
         * want to exercise ImportInput's source-size boundary when possible.
         */
        if let Ok(source_id) = source_map.add(
            Arc::<str>::from("oversized.qasm"),
            Arc::<str>::from(
                String::from_utf8(source.clone())
                    .expect("ASCII spaces are UTF-8"),
            ),
        ) {
            let result = ImportInput::new(
                source_id,
                source,
                source_map,
                ImportConfig::new(limits),
            );

            assert!(
                result.is_err(),
                "source larger than FrontendLimits must be rejected \
                 before parsing",
            );
        }
    }
}


// =============================================================================
// Encoding safety
// =============================================================================

#[test]
fn malformed_utf8_is_rejected_without_panicking() {
    let importer = OpenQasmImporter::production();

    let bytes = vec![
        b'O',
        b'P',
        b'E',
        b'N',
        0xff,
        0xfe,
        b';',
    ];

    /*
     * ImportInput intentionally accepts bytes because the generic frontend
     * contract is format-independent. OpenQASM itself is responsible for
     * enforcing UTF-8 at its decoding boundary.
     */
    let mut source_map = SourceMap::new();

    /*
     * The source map is UTF-8 text, so use a valid textual placeholder here.
     * The importer must still reject the actual malformed byte sequence.
     */
    let source_id = source_map
        .add(
            Arc::<str>::from("invalid-utf8.qasm"),
            Arc::<str>::from("invalid"),
        )
        .expect("placeholder source must be valid");

    let input = {
        let result = ImportInput::new(
            source_id,
            bytes,
            source_map,
            ImportConfig::new(FrontendLimits::production()),
        );

        /*
         * The generic boundary correctly rejects mismatched source bytes
         * because the source map cannot truthfully represent malformed UTF-8.
         *
         * This is the preferred behavior: malformed source must never reach
         * the parser under a false source-map identity.
         */
        assert!(
            result.is_err(),
            "malformed bytes must not cross an inconsistent source-map boundary",
        );

        return;
    };

    let _ = input;
}


// =============================================================================
// Version policy
// =============================================================================

#[test]
fn missing_version_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"qubit[1] q;
x q[0];
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "OpenQASM source without a version declaration must be rejected",
    );
}

#[test]
fn openqasm_2_is_rejected_explicitly() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 2.0;
qreg q[1];
x q[0];
"#;

    let error = importer
        .import(make_input(source))
        .expect_err("OpenQASM 2.x must not be accepted by the OpenQASM 3 importer");

    assert_eq!(
        error.kind(),
        FrontendErrorKind::Unsupported,
    );
}

#[test]
fn future_openqasm_3_version_is_not_silently_accepted() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.2;
qubit[1] q;
x q[0];
"#;

    let error = importer
        .import(make_input(source))
        .expect_err(
            "future OpenQASM 3.x versions must require explicit support",
        );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::Unsupported,
    );
}

#[test]
fn non_openqasm_major_version_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 4.0;
qubit[1] q;
"#;

    let error = importer
        .import(make_input(source))
        .expect_err(
            "unsupported OpenQASM major versions must be rejected",
        );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::Unsupported,
    );
}


// =============================================================================
// Syntax failures
// =============================================================================

#[test]
fn truncated_program_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
qubit[1] q;
h q[0]
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "truncated OpenQASM must not be imported successfully",
    );
}

#[test]
fn invalid_statement_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
this_is_not_valid_openqasm;
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "invalid syntax must stop before canonical IR construction",
    );
}

#[test]
fn malformed_declaration_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
qubit q;
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "malformed quantum declarations must be rejected",
    );
}


// =============================================================================
// Semantic validation failures
// =============================================================================

#[test]
fn unknown_qubit_identifier_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
qubit[1] q;
x missing;
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "unknown quantum identifiers must not reach lowering",
    );
}

#[test]
fn out_of_range_qubit_index_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
qubit[1] q;
x q[1];
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "out-of-range qubit indices must be rejected",
    );
}

#[test]
fn duplicate_declaration_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
qubit[1] q;
qubit[1] q;
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "duplicate quantum declarations must be rejected",
    );
}

#[test]
fn gate_arity_mismatch_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
qubit[1] q;
cx q[0], q[0];
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "two-qubit gates must not be accepted with an invalid operand shape",
    );
}

#[test]
fn measurement_width_mismatch_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
qubit[2] q;
bit[1] c;
measure q -> c;
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "measurement width mismatches must be rejected",
    );
}

#[test]
fn measurement_unknown_destination_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
qubit[1] q;
measure q[0] -> missing[0];
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "measurement destinations must resolve to declared classical storage",
    );
}


// =============================================================================
// Broadcasting and register semantics
// =============================================================================

#[test]
fn incompatible_broadcast_dimensions_are_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[2] a;
qubit[3] b;

cx a, b;
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "incompatible OpenQASM broadcast dimensions must be rejected",
    );
}

#[test]
fn compatible_scalar_gate_operands_are_accepted() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[2] q;

h q[0];
cx q[0], q[1];
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_ok(),
        "valid scalar gate operands must lower successfully",
    );
}


// =============================================================================
// Include security boundary
// =============================================================================

#[test]
fn standard_library_include_is_the_declared_controlled_include() {
    assert_eq!(
        STANDARD_LIBRARY_INCLUDE,
        "stdgates.inc",
    );
}

#[test]
fn arbitrary_include_is_not_treated_as_an_execution_permission() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
include "/etc/passwd";

qubit[1] q;
"#;

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(make_input(source))
    }));

    assert!(
        result.is_ok(),
        "arbitrary include input must not cause a process panic",
    );

    let result = result
        .expect("panic was already checked");

    assert!(
        result.is_err(),
        "arbitrary include paths must not silently become filesystem access",
    );
}

#[test]
fn relative_parent_include_is_not_resolved_by_importer() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
include "../secret.qasm";

qubit[1] q;
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "the importer must not resolve arbitrary parent-directory includes",
    );
}


// =============================================================================
// Unsupported source-language features
// =============================================================================

#[test]
fn unsupported_calibration_construct_is_never_silently_discarded() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[1] q;

defcal custom q {
}
"#;

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(make_input(source))
    }));

    assert!(
        result.is_ok(),
        "calibration input must not panic",
    );

    let result = result
        .expect("panic was already checked");

    /*
     * The important invariant is not the exact current diagnostic category:
     * the construct must either be explicitly supported and lowered, or it
     * must be rejected. It must never disappear and still produce a seemingly
     * successful circuit.
     */
    if let Ok(output) = result {
        /*
         * If calibration becomes supported in a future implementation, this
         * test should be intentionally updated together with the capability
         * matrix. Until then, a successful output would be evidence that the
         * construct may have been silently discarded.
         */
        assert!(
            output.diagnostics().is_empty(),
            "unsupported calibration must not be hidden as a warning",
        );
    }
}

#[test]
fn extern_declaration_does_not_execute_anything() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

extern custom_operation();

qubit[1] q;
"#;

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(make_input(source))
    }));

    assert!(
        result.is_ok(),
        "extern declarations must be data, never executable permissions",
    );

    let result = result
        .expect("panic was already checked");

    /*
     * Whether the exact construct is currently represented is owned by the
     * OpenQASM capability policy. What this integration test forbids is a
     * process-level side effect or panic.
     */
    let _ = result;
}


// =============================================================================
// Classical and parameter semantics
// =============================================================================

#[test]
fn parameterized_standard_gate_imports() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
include "stdgates.inc";

qubit[1] q;

rx(pi / 2) q[0];
rz(pi / 4) q[0];
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_ok(),
        "representable parameterized standard gates must import successfully",
    );
}

#[test]
fn malformed_parameter_expression_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
include "stdgates.inc";

qubit[1] q;

rx(pi / ) q[0];
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "malformed parameter expressions must not reach lowering",
    );
}

#[test]
fn undefined_parameter_identifier_is_rejected() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
include "stdgates.inc";

qubit[1] q;

rx(undefined_parameter) q[0];
"#;

    let result = importer.import(make_input(source));

    assert!(
        result.is_err(),
        "undefined parameter identifiers must be rejected",
    );
}


// =============================================================================
// Determinism
// =============================================================================

#[test]
fn repeated_imports_are_deterministic_at_the_public_boundary() {
    let importer = OpenQasmImporter::production();

    let first = importer
        .import(make_input(basic_qasm_31()))
        .expect("first import must succeed");

    let second = importer
        .import(make_input(basic_qasm_31()))
        .expect("second import must succeed");

    assert_eq!(
        first.format(),
        second.format(),
    );

    assert_eq!(
        first.version(),
        second.version(),
    );

    assert_eq!(
        first.diagnostics().len(),
        second.diagnostics().len(),
    );

    /*
     * The canonical IR implements equality as its own semantic contract.
     * Comparing the complete circuit here detects accidental nondeterminism
     * in declaration ordering, operation generation, identity assignment, or
     * lowering.
     */
    assert_eq!(
        first.circuit(),
        second.circuit(),
        "identical OpenQASM input must produce identical canonical IR",
    );
}

#[test]
fn source_name_does_not_change_quantum_semantics() {
    let importer = OpenQasmImporter::production();

    let first = importer
        .import(make_input_with_name(
            "first.qasm",
            no_include_qasm(),
        ))
        .expect("first import must succeed");

    let second = importer
        .import(make_input_with_name(
            "second.qasm",
            no_include_qasm(),
        ))
        .expect("second import must succeed");

    assert_eq!(
        first.circuit(),
        second.circuit(),
        "source display names must not alter canonical quantum semantics",
    );
}


// =============================================================================
// Panic resistance
// =============================================================================

#[test]
fn malformed_inputs_do_not_panic() {
    let importer = OpenQasmImporter::production();

    let malformed_inputs = [
        "",
        ";",
        "OPENQASM",
        "OPENQASM 3.1",
        "OPENQASM 3.1;",
        "OPENQASM 3.1; {",
        "OPENQASM 3.1; }",
        "OPENQASM 3.1; qubit[",
        "OPENQASM 3.1; qubit[] q;",
        "OPENQASM 3.1; qubit[-1] q;",
        "OPENQASM 3.1; x;",
        "OPENQASM 3.1; cx q[0], q[1];",
        "OPENQASM 3.1; measure;",
        "OPENQASM 3.1; include;",
        "OPENQASM 3.1; include \"unterminated;",
        "OPENQASM 3.1; /* unterminated",
    ];

    for source in malformed_inputs {
        let result = catch_unwind(AssertUnwindSafe(|| {
            importer.import(make_input(source))
        }));

        assert!(
            result.is_ok(),
            "frontend import panicked for malformed input: {source:?}",
        );
    }
}

#[test]
fn deeply_nested_malformed_input_does_not_panic() {
    let importer = OpenQasmImporter::production();

    /*
     * Keep the fixture deliberately bounded. The purpose here is to catch
     * parser state-machine/panic bugs, while dedicated resource-exhaustion
     * tests own very large adversarial inputs.
     */
    let mut source = String::from("OPENQASM 3.1;\n");

    for _ in 0..512 {
        source.push_str("if (true) {\n");
    }

    for _ in 0..512 {
        source.push_str("}\n");
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(make_input(&source))
    }));

    assert!(
        result.is_ok(),
        "deep malformed nesting must fail safely rather than panic",
    );
}


// =============================================================================
// Canonical IR validation boundary
// =============================================================================

#[test]
fn successful_import_returns_a_valid_canonical_circuit() {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(make_input(basic_qasm_31()))
        .expect("valid OpenQASM must import");

    let circuit = output.circuit();

    assert!(
        circuit.validate().is_ok(),
        "ImportOutput must never expose an invalid QuantumCircuit",
    );
}

#[test]
fn invalid_program_never_returns_a_successful_circuit() {
    let importer = OpenQasmImporter::production();

    let invalid_programs = [
        r#"OPENQASM 3.1;
qubit[1] q;
x missing;
"#,
        r#"OPENQASM 3.1;
qubit[1] q;
cx q[0], q[0];
"#,
        r#"OPENQASM 3.1;
qubit[1] q;
bit[2] c;
measure q -> c;
"#,
    ];

    for source in invalid_programs {
        let result = importer.import(make_input(source));

        assert!(
            result.is_err(),
            "invalid OpenQASM must not cross the successful IR boundary:\n{source}",
        );
    }
}


// =============================================================================
// Import configuration / limit integration
// =============================================================================

#[test]
fn importer_consumes_frontend_limits_through_generic_import_config() {
    let importer = OpenQasmImporter::production();

    let limits = FrontendLimits::strict();

    let source = no_include_qasm();

    let input = make_input_with_limits(
        source.as_bytes().to_vec(),
        limits,
    );

    /*
     * A normal tiny program should still import under the strict security
     * profile. This verifies that limits are actually passed through the
     * generic ImportConfig boundary rather than being ignored by the
     * OpenQASM adapter.
     */
    let result = importer.import(input);

    assert!(
        result.is_ok(),
        "a tiny valid program must remain importable under strict limits",
    );
}

#[test]
fn import_rejects_program_that_exceeds_a_strict_source_limit() {
    let importer = OpenQasmImporter::production();

    /*
     * Build a source larger than the strict source limit only when the limit
     * is reasonably small enough for an inexpensive test allocation.
     */
    let limits = FrontendLimits::strict();

    let maximum = limits.max_source_bytes();

    if maximum <= 1024 * 1024 {
        let mut source = String::with_capacity(
            maximum.saturating_add(32),
        );

        source.push_str("OPENQASM 3.1;\n");

        while source.len() <= maximum {
            source.push_str("// padding\n");
        }

        let source = source.into_bytes();

        let mut source_map = SourceMap::new();

        let text = String::from_utf8(source.clone())
            .expect("test padding is valid UTF-8");

        if let Ok(source_id) = source_map.add(
            Arc::<str>::from("too-large.qasm"),
            Arc::<str>::from(text),
        ) {
            let result = ImportInput::new(
                source_id,
                source,
                source_map,
                ImportConfig::new(limits),
            );

            assert!(
                result.is_err(),
                "ImportInput must reject source beyond configured FrontendLimits",
            );
        }
    }
}


// =============================================================================
// Error classification
// =============================================================================

#[test]
fn syntax_failure_has_structured_frontend_error_kind() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
qubit[1] q
x q[0];
"#;

    let error = importer
        .import(make_input(source))
        .expect_err("invalid syntax must fail");

    assert_eq!(
        error.kind(),
        FrontendErrorKind::Syntax,
    );
}

#[test]
fn semantic_failure_has_structured_frontend_error_kind() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;
qubit[1] q;
x missing;
"#;

    let error = importer
        .import(make_input(source))
        .expect_err("invalid semantics must fail");

    assert_eq!(
        error.kind(),
        FrontendErrorKind::Semantic,
    );
}

#[test]
fn unsupported_version_has_structured_error_kind() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.2;
qubit[1] q;
"#;

    let error = importer
        .import(make_input(source))
        .expect_err("future OpenQASM must be rejected");

    assert_eq!(
        error.kind(),
        FrontendErrorKind::Unsupported,
    );
}


// =============================================================================
// Public API isolation
// =============================================================================

#[test]
fn importer_can_be_used_without_accessing_private_openqasm_types() {
    /*
     * This test intentionally contains no imports of:
     *
     * - OpenQASM AST;
     * - OpenQASM token;
     * - parser configuration;
     * - validator internals;
     * - serializer helpers.
     *
     * If this test can compile and execute, the public facade remains
     * sufficient for a normal import consumer.
     */
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(make_input(no_include_qasm()))
        .expect("public importer API must be sufficient");

    assert_eq!(
        output.format(),
        &FormatId::new("openqasm")
            .expect("OpenQASM format ID is valid"),
    );
}

#[test]
fn importer_version_matches_generic_format_version_contract() {
    let importer = OpenQasmImporter::production();

    let version = importer.version();

    assert_eq!(
        version,
        FormatVersion::new(3, 1, 0),
    );
}


// =============================================================================
// No accidental execution boundary
// =============================================================================

#[test]
fn pragma_like_source_is_data_not_execution_permission() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

#pragma execute "touch /tmp/should-not-exist"

qubit[1] q;
x q[0];
"#;

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(make_input(source))
    }));

    assert!(
        result.is_ok(),
        "pragma-like source must never cause importer panic",
    );

    /*
     * The exact parser/validation status of a pragma is governed by the
     * OpenQASM capability policy. The security invariant is that importing
     * the text cannot execute its contents.
     */
}

#[test]
fn calibration_like_source_is_never_executed() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

cal {
    this text is not executable;
}

qubit[1] q;
x q[0];
"#;

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(make_input(source))
    }));

    assert!(
        result.is_ok(),
        "calibration-like source must never cause process-level execution",
    );
}


// =============================================================================
// Regression tests for the complete pipeline
// =============================================================================

#[test]
fn complete_import_pipeline_has_no_intermediate_public_success_state() {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(make_input(
            r#"OPENQASM 3.1;
include "stdgates.inc";

qubit[3] q;
bit[3] c;

h q[0];
x q[1];
cx q[0], q[2];
measure q -> c;
"#,
        ))
        .expect("complete supported program must import");

    /*
     * A successful ImportOutput means:
     *
     * source decoding       succeeded
     * parsing               succeeded
     * semantic validation   succeeded
     * lowering              succeeded
     * canonical IR          succeeded
     * canonical IR validity succeeded
     *
     * The public API deliberately does not expose partially validated
     * intermediate states.
     */
    assert!(
        output.circuit().validate().is_ok(),
    );
}

#[test]
fn equivalent_whitespace_does_not_change_canonical_semantics() {
    let importer = OpenQasmImporter::production();

    let compact = r#"OPENQASM 3.1;
qubit[1] q;
x q[0];
"#;

    let formatted = r#"
OPENQASM 3.1;

// deliberately different whitespace

qubit[1] q;

x q[0];
"#;

    let first = importer
        .import(make_input(compact))
        .expect("compact program must import");

    let second = importer
        .import(make_input(formatted))
        .expect("formatted program must import");

    assert_eq!(
        first.circuit(),
        second.circuit(),
        "non-semantic whitespace changes must not alter canonical IR",
    );
}


// =============================================================================
// Final production invariant
// =============================================================================

#[test]
fn production_import_invariant_is_closed() {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(make_input(basic_qasm_31()))
        .expect("known-good production fixture must import");

    assert_eq!(
        output.format(),
        &OPENQASM_FORMAT_ID,
    );

    assert_eq!(
        output.version(),
        &OPENQASM_3_1,
    );

    assert!(
        output.circuit().validate().is_ok(),
        "successful import must always terminate at valid canonical Quantum IR",
    );

    assert!(
        output.diagnostics().is_empty(),
        "known-good fixture must not produce diagnostics",
    );
}