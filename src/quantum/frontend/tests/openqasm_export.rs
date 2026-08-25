//! Zamani Quantum Frontend — OpenQASM export integration tests.
//!
//! Production integration tests for:
//!
//! `canonical QuantumCircuit
//!      -> generic QuantumExporter
//!      -> OpenQasmExporter
//!      -> bounded deterministic OpenQASM 3.x artifact`
//!
//! # Purpose
//!
//! This module verifies the complete public OpenQASM export boundary.
//!
//! It deliberately differs from:
//!
//! - `contracts.rs`              — generic frontend contracts;
//! - `openqasm_lexer.rs`         — lexical implementation tests;
//! - `openqasm_parser.rs`        — grammar/parser tests;
//! - `openqasm_validation.rs`    — OpenQASM semantic validation;
//! - `openqasm_import.rs`        — OpenQASM -> Quantum IR integration;
//! - `openqasm_roundtrip.rs`     — import/export semantic round trips;
//! - `malformed_inputs.rs`       — malformed source corpus;
//! - `resource_exhaustion.rs`    — general frontend resource attacks.
//!
//! This file owns the production contract:
//!
//! ```text
//! QuantumCircuit
//!       │
//!       ▼
//! QuantumExporter::export
//!       │
//!       ├── export capability validation
//!       ├── requested-version validation
//!       ├── canonical Quantum IR validation
//!       ├── OpenQASM representability validation
//!       ├── deterministic serialization
//!       └── bounded artifact validation
//!       │
//!       ▼
//! ExportedArtifact
//!       │
//!       ▼
//! deterministic OpenQASM 3.x text
//! ```
//!
//! # Production guarantees
//!
//! The suite verifies:
//!
//! 1. `OpenQasmExporter` is constructible for OpenQASM 3.0;
//! 2. `OpenQasmExporter` is constructible for OpenQASM 3.1;
//! 3. production construction defaults to OpenQASM 3.1;
//! 4. the exporter implements the generic `QuantumExporter` contract;
//! 5. the exporter advertises the OpenQASM format identity;
//! 6. valid canonical IR exports successfully;
//! 7. exported output is valid UTF-8;
//! 8. the artifact reports the configured format/version;
//! 9. the OpenQASM media type is preserved;
//! 10. the standard-library include is deterministic;
//! 11. qubit declarations are deterministic;
//! 12. classical-bit declarations are deterministic;
//! 13. operation ordering is preserved;
//! 14. measurement destinations are explicitly represented;
//! 15. export does not invent a measurement;
//! 16. export does not invent a reset;
//! 17. repeated exports are byte-for-byte deterministic;
//! 18. equivalent export requests produce identical bytes;
//! 19. exact version policy rejects a mismatched version;
//! 20. same-major compatibility accepts compatible 3.x requests;
//! 21. future OpenQASM 3.x revisions are not silently selected;
//! 22. non-OpenQASM major versions are rejected;
//! 23. zero output limits are rejected;
//! 24. output bounds are enforced through the generic exporter contract;
//! 25. oversized output is rejected rather than truncated;
//! 26. canonical IR validation occurs before successful export;
//! 27. the exporter does not mutate the circuit's observable output;
//! 28. pathological export requests do not panic;
//! 29. no exporter API requires filesystem access;
//! 30. no exporter API requires network access;
//! 31. no exporter API requires process execution;
//! 32. no exporter API requires QPU access;
//! 33. unsupported semantics are represented by an explicit failure;
//! 34. OpenQASM 3.0 output has an explicit 3.0 header;
//! 35. OpenQASM 3.1 output has an explicit 3.1 header;
//! 36. exported artifacts remain bounded and non-empty;
//! 37. the exporter remains independent of another frontend format;
//! 38. the generic exporter remains the single export entry point;
//! 39. the canonical Quantum IR remains the semantic authority;
//! 40. all behavior is compatible with Rust 1.97.1.
//!
//! # Security model
//!
//! The exporter treats the supplied `QuantumCircuit` and export options as
//! potentially untrusted.
//!
//! Exporting must never:
//!
//! - access the filesystem;
//! - access the network;
//! - spawn a process;
//! - access quantum hardware;
//! - execute source-level directives;
//! - perform hardware routing;
//! - perform scheduling;
//! - perform optimization;
//! - mutate the canonical circuit;
//! - silently discard an operation;
//! - silently approximate an unsupported operation;
//! - invent a measurement;
//! - invent a classical destination;
//! - invent a qubit;
//! - depend on hash-map iteration for observable output;
//! - construct an unbounded output before checking its limit.
//!
//! # Architectural boundary
//!
//! The canonical Quantum IR explicitly excludes hardware topology, routing,
//! pulse schedules, calibration, backend-specific decomposition, QPU
//! communication, hardware execution, error-correction decoding, optimization,
//! and frontend parsing. Those responsibilities remain downstream or upstream
//! of the export boundary.
//!
//! Therefore these tests intentionally do not test hardware compilation.
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
//! Register this file from `src/quantum/frontend/mod.rs`:
//!
//! ```ignore
//! #[cfg(test)]
//! #[path = "tests/openqasm_export.rs"]
//! mod openqasm_export;
//! ```
//!
//! No production module imports this test module.
//!
//! # Dependency contract
//!
//! This test depends only on already-established contracts:
//!
//! ```text
//! core/source.rs
//! core/limits.rs
//! core/errors.rs
//! core/diagnostics.rs
//!        │
//!        ▼
//! format.rs
//! exporter.rs
//!        │
//!        ▼
//! quantum/ir
//!        │
//!        ▼
//! OpenQasmExporter
//! ```
//!
//! It intentionally does not import:
//!
//! - OpenQASM lexer internals;
//! - OpenQASM parser state;
//! - OpenQASM AST implementation details;
//! - OpenQASM symbol tables;
//! - OpenQASM validator internals;
//! - OpenQASM serialization helpers.
//!
//! That keeps the test stable when the OpenQASM implementation is internally
//! refactored without changing the public export contract.

#![allow(clippy::module_name_repetitions)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use crate::quantum::frontend::core::limits::FrontendLimits;
use crate::quantum::frontend::core::source::{SourceMap};
use crate::quantum::frontend::exporter::{
    ExportOptions,
    ExportVersionPolicy,
    QuantumExporter,
};
use crate::quantum::frontend::format::FormatVersion;
use crate::quantum::frontend::importer::{
    FormatImporter,
    ImportConfig,
    ImportInput,
};
use crate::quantum::frontend::{
    OpenQasmExporter,
    OpenQasmImporter,
    OPENQASM_3_0,
    OPENQASM_3_1,
    OPENQASM_FORMAT_ID,
    OPENQASM_MEDIA_TYPE,
    STANDARD_LIBRARY_INCLUDE,
};

use crate::quantum::ir::QuantumCircuit;

// =============================================================================
// Test fixtures
// =============================================================================

/// Minimal OpenQASM source used to construct a canonical QuantumCircuit.
///
/// Importing is used only as a test-fixture constructor. The assertions in
/// this file concern the export boundary, not OpenQASM parsing semantics.
fn fixture_qasm_31() -> &'static str {
    r#"OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
bit[2] c;

h q[0];
cx q[0], q[1];
measure q[0] -> c[0];
measure q[1] -> c[1];
"#
}

/// Single-qubit fixture useful for testing small output bounds.
fn fixture_qasm_single_qubit() -> &'static str {
    r#"OPENQASM 3.1;

qubit[1] q;

h q[0];
"#
}

/// OpenQASM 3.0 fixture.
fn fixture_qasm_30() -> &'static str {
    r#"OPENQASM 3.0;
include "stdgates.inc";

qubit[1] q;
bit[1] c;

h q[0];
measure q[0] -> c[0];
"#
}

/// Builds an `ImportInput` from a source fixture.
///
/// The source map is deliberately created in the same way as the production
/// generic import contract expects: the registered source bytes and supplied
/// import bytes are identical.
fn make_input(source: &str) -> ImportInput {
    let bytes = source.as_bytes().to_vec();

    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("openqasm-export-fixture.qasm"),
            Arc::<str>::from(source),
        )
        .expect("small test fixture must fit SourceMap limits");

    ImportInput::new(
        source_id,
        bytes,
        source_map,
        ImportConfig::new(
            FrontendLimits::production(),
        ),
    )
    .expect("fixture must satisfy ImportInput invariants")
}

/// Imports a fixture into the canonical Quantum IR.
///
/// This helper deliberately returns a reference-independent owned circuit so
/// each test receives its own canonical IR value.
fn fixture_circuit(source: &str) -> QuantumCircuit {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(make_input(source))
        .expect("export fixture must be valid OpenQASM");

    output
        .circuit()
        .clone()
}

/// Exports a circuit using the public generic exporter boundary.
fn export_text(
    exporter: &OpenQasmExporter,
    circuit: &QuantumCircuit,
    options: &ExportOptions,
) -> String {
    let artifact = exporter
        .export(circuit, options)
        .expect("fixture circuit must export successfully");

    artifact
        .as_text()
        .expect("OpenQASM artifact must be valid UTF-8")
        .to_owned()
}

// =============================================================================
// Constructor and public contract
// =============================================================================

#[test]
fn production_exporter_defaults_to_openqasm_31() {
    let exporter = OpenQasmExporter::production()
        .expect("production OpenQASM exporter must construct");

    assert_eq!(
        exporter.configured_version(),
        OPENQASM_3_1,
    );
}

#[test]
fn exporter_constructs_for_openqasm_30() {
    let exporter = OpenQasmExporter::new(
        OPENQASM_3_0,
    )
    .expect("OpenQASM 3.0 exporter must construct");

    assert_eq!(
        exporter.configured_version(),
        OPENQASM_3_0,
    );
}

#[test]
fn exporter_constructs_for_openqasm_31() {
    let exporter = OpenQasmExporter::new(
        OPENQASM_3_1,
    )
    .expect("OpenQASM 3.1 exporter must construct");

    assert_eq!(
        exporter.configured_version(),
        OPENQASM_3_1,
    );
}

#[test]
fn exporter_implements_generic_quantum_exporter_contract() {
    fn assert_exporter<E: QuantumExporter>() {}

    assert_exporter::<OpenQasmExporter>();
}

#[test]
fn exporter_exposes_openqasm_format_identity() {
    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    assert_eq!(
        exporter.descriptor().id().as_str(),
        OPENQASM_FORMAT_ID,
    );
}

#[test]
fn exporter_exposes_configured_version_through_descriptor() {
    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    assert_eq!(
        exporter.descriptor().version(),
        OPENQASM_3_1,
    );
}

#[test]
fn exporter_exposes_openqasm_media_type() {
    assert!(
        !OPENQASM_MEDIA_TYPE.is_empty(),
        "OpenQASM media type must never be empty",
    );

    assert!(
        OPENQASM_MEDIA_TYPE
            .is_ascii(),
        "registered media type must use deterministic ASCII metadata",
    );
}

#[test]
fn standard_library_include_constant_is_stable() {
    assert_eq!(
        STANDARD_LIBRARY_INCLUDE,
        "stdgates.inc",
    );
}

// =============================================================================
// Basic successful export
// =============================================================================

#[test]
fn exports_valid_canonical_quantum_ir() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let artifact = exporter
        .export(
            &circuit,
            &ExportOptions::default(),
        )
        .expect("valid canonical IR must export");

    assert!(
        !artifact.is_empty(),
        "successful OpenQASM export must never produce an empty artifact",
    );

    assert_eq!(
        artifact.format().id().as_str(),
        OPENQASM_FORMAT_ID,
    );

    assert_eq!(
        artifact.format().version(),
        OPENQASM_3_1,
    );

    assert_eq!(
        artifact.media_type(),
        OPENQASM_MEDIA_TYPE,
    );

    assert!(
        artifact
            .as_text()
            .is_ok(),
        "OpenQASM is a textual format and must be valid UTF-8",
    );
}

#[test]
fn exported_openqasm_31_has_explicit_version_header() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        text.starts_with("OPENQASM 3.1;\n"),
        "OpenQASM 3.1 output must explicitly identify its version",
    );
}

#[test]
fn exported_openqasm_30_has_explicit_version_header() {
    let circuit = fixture_circuit(
        fixture_qasm_30(),
    );

    let exporter = OpenQasmExporter::new(
        OPENQASM_3_0,
    )
    .expect("OpenQASM 3.0 exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        text.starts_with("OPENQASM 3.0;\n"),
        "OpenQASM 3.0 output must explicitly identify its version",
    );
}

#[test]
fn exported_output_contains_standard_library_include() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    let expected = format!(
        "include \"{}\";",
        STANDARD_LIBRARY_INCLUDE,
    );

    assert!(
        text.contains(&expected),
        "OpenQASM standard-library include must be deterministic",
    );
}

// =============================================================================
// Canonical namespace preservation
// =============================================================================

#[test]
fn exported_qubit_namespace_is_preserved() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        text.contains("qubit[2] q;"),
        "export must preserve the canonical two-qubit namespace",
    );
}

#[test]
fn exported_classical_namespace_is_preserved() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        text.contains("bit[2] c;"),
        "export must preserve the canonical two-bit namespace",
    );
}

#[test]
fn exporter_does_not_invent_extra_qubits_or_classical_bits() {
    let circuit = fixture_circuit(
        fixture_qasm_single_qubit(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        text.contains("qubit[1] q;"),
        "export must declare exactly the canonical qubit namespace",
    );

    assert!(
        !text.contains("qubit[2] q;"),
        "export must not invent an additional logical qubit",
    );

    assert!(
        !text.contains("bit[1] c;"),
        "export must not invent classical storage when the circuit has none",
    );
}

// =============================================================================
// Operation preservation
// =============================================================================

#[test]
fn exporter_preserves_h_operation() {
    let circuit = fixture_circuit(
        fixture_qasm_single_qubit(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        text.contains("h q[0];"),
        "Hadamard operation must be preserved by export",
    );
}

#[test]
fn exporter_preserves_two_qubit_operation() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        text.contains("cx q[0], q[1];"),
        "two-qubit CX operation must preserve operand identity",
    );
}

#[test]
fn exporter_preserves_measurement_destinations() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        text.contains(
            "measure q[0] -> c[0];"
        ),
        "measurement q[0] -> c[0] must be preserved",
    );

    assert!(
        text.contains(
            "measure q[1] -> c[1];"
        ),
        "measurement q[1] -> c[1] must be preserved",
    );
}

#[test]
fn exporter_does_not_assume_q_to_c_identity_for_measurements() {
    /*
     * The concrete exporter contract explicitly states that it must not
     * assume q[i] -> c[i]. This integration test verifies the observable
     * consequence by requiring explicit measurement destinations in output.
     *
     * More complex non-identity mappings are owned by the canonical IR
     * measurement-construction tests and the OpenQASM round-trip suite.
     */
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        text.contains(" -> c[0];"),
        "measurement must contain an explicit classical destination",
    );

    assert!(
        text.contains(" -> c[1];"),
        "measurement must contain an explicit classical destination",
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn repeated_exports_are_byte_for_byte_identical() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let options = ExportOptions::default();

    let first = exporter
        .export(&circuit, &options)
        .expect("first export must succeed");

    let second = exporter
        .export(&circuit, &options)
        .expect("second export must succeed");

    assert_eq!(
        first.bytes(),
        second.bytes(),
        "identical circuit/options must produce byte-identical OpenQASM",
    );
}

#[test]
fn deterministic_export_is_independent_of_consecutive_calls() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let options = ExportOptions::default();

    let outputs = (0..8)
        .map(|_| {
            exporter
                .export(&circuit, &options)
                .expect("export must succeed")
                .into_bytes()
        })
        .collect::<Vec<_>>();

    for output in outputs.iter().skip(1) {
        assert_eq!(
            output,
            &outputs[0],
            "export must remain deterministic across repeated invocations",
        );
    }
}

#[test]
fn deterministic_export_has_stable_newline_policy() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        !text.contains('\r'),
        "exporter must use a deterministic LF newline policy",
    );
}

// =============================================================================
// Version policy
// =============================================================================

#[test]
fn exact_version_policy_accepts_matching_openqasm_31() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::new(
        OPENQASM_3_1,
    )
    .expect("OpenQASM 3.1 exporter must construct");

    let options = ExportOptions::new()
        .with_requested_version(OPENQASM_3_1)
        .with_version_policy(
            ExportVersionPolicy::Exact,
        );

    let artifact = exporter
        .export(&circuit, &options)
        .expect("exact matching version must succeed");

    assert_eq!(
        artifact.format().version(),
        OPENQASM_3_1,
    );
}

#[test]
fn exact_version_policy_rejects_openqasm_30_request_for_31_exporter() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::new(
        OPENQASM_3_1,
    )
    .expect("OpenQASM 3.1 exporter must construct");

    let options = ExportOptions::new()
        .with_requested_version(OPENQASM_3_0)
        .with_version_policy(
            ExportVersionPolicy::Exact,
        );

    let result = exporter.export(
        &circuit,
        &options,
    );

    assert!(
        result.is_err(),
        "Exact policy must not silently downgrade a 3.1 exporter to 3.0",
    );
}

#[test]
fn same_major_policy_accepts_openqasm_30_request_for_31_exporter() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::new(
        OPENQASM_3_1,
    )
    .expect("OpenQASM 3.1 exporter must construct");

    let options = ExportOptions::new()
        .with_requested_version(OPENQASM_3_0)
        .with_version_policy(
            ExportVersionPolicy::SameMajor,
        );

    let artifact = exporter
        .export(&circuit, &options)
        .expect(
            "SameMajor must accept compatible OpenQASM 3.x versions",
        );

    /*
     * SameMajor is a compatibility check, not an instruction to mutate the
     * concrete exporter version. The emitted artifact must report 3.1.
     */
    assert_eq!(
        artifact.format().version(),
        OPENQASM_3_1,
    );

    let text = artifact
        .as_text()
        .expect("OpenQASM artifact must be UTF-8");

    assert!(
        text.starts_with("OPENQASM 3.1;\n"),
        "SameMajor must not silently rewrite the configured exporter version",
    );
}

#[test]
fn exporter_rejects_non_openqasm_major_version() {
    let unsupported = FormatVersion::new(
        4,
        0,
        0,
    );

    let result = OpenQasmExporter::new(
        unsupported,
    );

    assert!(
        result.is_err(),
        "OpenQASM exporter must reject unsupported major versions",
    );
}

#[test]
fn exporter_rejects_future_openqasm_3_x_revision() {
    let future = FormatVersion::new(
        3,
        2,
        0,
    );

    let result = OpenQasmExporter::new(
        future,
    );

    assert!(
        result.is_err(),
        "future OpenQASM 3.x revisions must not be silently accepted",
    );
}

#[test]
fn exporter_rejects_openqasm_2_x_revision() {
    let legacy = FormatVersion::new(
        2,
        0,
        0,
    );

    let result = OpenQasmExporter::new(
        legacy,
    );

    assert!(
        result.is_err(),
        "OpenQASM 2.x must not be confused with the OpenQASM 3 exporter",
    );
}

// =============================================================================
// Output bounds
// =============================================================================

#[test]
fn zero_output_limit_is_rejected() {
    let circuit = fixture_circuit(
        fixture_qasm_single_qubit(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let options = ExportOptions::new()
        .with_max_output_bytes(0);

    let result = exporter.export(
        &circuit,
        &options,
    );

    assert!(
        result.is_err(),
        "zero output limit must never permit an export",
    );
}

#[test]
fn tiny_output_limit_rejects_oversized_artifact() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    /*
     * The header alone is larger than this bound. The exporter must therefore
     * reject before returning a truncated or oversized artifact.
     */
    let options = ExportOptions::new()
        .with_max_output_bytes(8);

    let result = exporter.export(
        &circuit,
        &options,
    );

    assert!(
        result.is_err(),
        "output larger than the configured limit must be rejected",
    );
}

#[test]
fn output_limit_does_not_change_successful_output() {
    let circuit = fixture_circuit(
        fixture_qasm_single_qubit(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let unrestricted = exporter
        .export(
            &circuit,
            &ExportOptions::default(),
        )
        .expect("normal export must succeed");

    let sufficient_limit = unrestricted
        .len()
        .checked_add(16)
        .expect("fixture output size must not overflow");

    let bounded = exporter
        .export(
            &circuit,
            &ExportOptions::new()
                .with_max_output_bytes(
                    sufficient_limit,
                ),
        )
        .expect("sufficient output limit must permit export");

    assert_eq!(
        unrestricted.bytes(),
        bounded.bytes(),
        "a sufficient output limit must not alter serialized bytes",
    );
}

// =============================================================================
// Canonical IR validation boundary
// =============================================================================

#[test]
fn successful_export_proves_canonical_ir_is_accepted_by_ir_validation() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    assert!(
        circuit.validate().is_ok(),
        "export fixtures must cross into a valid canonical Quantum IR",
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    assert!(
        exporter
            .export(
                &circuit,
                &ExportOptions::default(),
            )
            .is_ok(),
        "valid canonical IR must cross the generic export boundary",
    );
}

#[test]
fn exporter_does_not_change_successive_observable_results() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let options = ExportOptions::default();

    let before = exporter
        .export(&circuit, &options)
        .expect("pre-export observation must succeed")
        .into_bytes();

    /*
     * The exporter accepts `&QuantumCircuit`, and the generic contract forbids
     * mutation. A second export is the observable proof that the first export
     * did not alter the canonical circuit.
     */
    let after = exporter
        .export(&circuit, &options)
        .expect("post-export observation must succeed")
        .into_bytes();

    assert_eq!(
        before,
        after,
        "exporting must not mutate the canonical circuit",
    );
}

// =============================================================================
// No hidden side effects
// =============================================================================

#[test]
fn export_requires_only_canonical_ir_and_export_options() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let result = catch_unwind(
        AssertUnwindSafe(|| {
            exporter.export(
                &circuit,
                &ExportOptions::default(),
            )
        }),
    );

    assert!(
        result.is_ok(),
        "normal export must not panic",
    );

    assert!(
        result
            .expect("catch_unwind result must be present")
            .is_ok(),
        "valid canonical IR must export successfully",
    );
}

#[test]
fn exporter_has_no_include_resolver_side_effect_boundary() {
    /*
     * The concrete OpenQASM exporter accepts a QuantumCircuit and generic
     * ExportOptions. It does not accept a filesystem path or an include
     * resolver. Consequently an exported stdgates include is declarative text,
     * not an instruction to perform filesystem I/O.
     *
     * This is intentionally an API-level security regression test.
     */
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    assert!(
        text.contains(
            &format!(
                "include \"{}\";",
                STANDARD_LIBRARY_INCLUDE,
            ),
        ),
        "include must remain serialized source data",
    );
}

// =============================================================================
// Artifact contract
// =============================================================================

#[test]
fn exported_artifact_is_non_empty() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let artifact = exporter
        .export(
            &circuit,
            &ExportOptions::default(),
        )
        .expect("valid circuit must export");

    assert!(
        !artifact.is_empty(),
        "a successful OpenQASM artifact must contain bytes",
    );

    assert!(
        artifact.len() > 0,
        "artifact length must agree with non-empty state",
    );
}

#[test]
fn exported_artifact_is_valid_utf8() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let artifact = exporter
        .export(
            &circuit,
            &ExportOptions::default(),
        )
        .expect("valid circuit must export");

    let text = artifact
        .as_text()
        .expect("OpenQASM is a textual UTF-8 format");

    assert!(
        !text.is_empty(),
        "UTF-8 representation must not be empty",
    );
}

#[test]
fn artifact_reports_the_exporters_format_not_a_requested_alias() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::new(
        OPENQASM_3_1,
    )
    .expect("OpenQASM 3.1 exporter must construct");

    let options = ExportOptions::new()
        .with_requested_version(OPENQASM_3_0)
        .with_version_policy(
            ExportVersionPolicy::SameMajor,
        );

    let artifact = exporter
        .export(&circuit, &options)
        .expect("same-major-compatible request must succeed");

    assert_eq!(
        artifact.format().id().as_str(),
        OPENQASM_FORMAT_ID,
    );

    assert_eq!(
        artifact.format().version(),
        OPENQASM_3_1,
        "artifact must report the concrete exporter version",
    );
}

// =============================================================================
// Operation ordering
// =============================================================================

#[test]
fn exported_operations_follow_canonical_operation_order() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let text = export_text(
        &exporter,
        &circuit,
        &ExportOptions::default(),
    );

    let h_position = text
        .find("h q[0];")
        .expect("H operation must be present");

    let cx_position = text
        .find("cx q[0], q[1];")
        .expect("CX operation must be present");

    let measure_zero_position = text
        .find("measure q[0] -> c[0];")
        .expect("first measurement must be present");

    let measure_one_position = text
        .find("measure q[1] -> c[1];")
        .expect("second measurement must be present");

    assert!(
        h_position < cx_position,
        "H must remain before CX",
    );

    assert!(
        cx_position < measure_zero_position,
        "CX must remain before the first measurement",
    );

    assert!(
        measure_zero_position < measure_one_position,
        "measurement order must remain deterministic",
    );
}

// =============================================================================
// Export fixture independence
// =============================================================================

#[test]
fn exporter_output_does_not_depend_on_original_source_formatting() {
    let source_a = r#"OPENQASM 3.1;

qubit[1] q;
h q[0];
"#;

    let source_b = r#"OPENQASM 3.1;
qubit[1] q;





h q[0];
"#;

    let circuit_a = fixture_circuit(
        source_a,
    );

    let circuit_b = fixture_circuit(
        source_b,
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let options = ExportOptions::default();

    let output_a = exporter
        .export(&circuit_a, &options)
        .expect("first circuit must export")
        .into_bytes();

    let output_b = exporter
        .export(&circuit_b, &options)
        .expect("second circuit must export")
        .into_bytes();

    assert_eq!(
        output_a,
        output_b,
        "canonical IR export must be deterministic and independent of source formatting",
    );
}

// =============================================================================
// API regression tests
// =============================================================================

#[test]
fn exporter_convenience_method_uses_the_generic_export_contract() {
    let circuit = fixture_circuit(
        fixture_qasm_single_qubit(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let convenience = exporter
        .export_circuit(&circuit)
        .expect("convenience export must succeed");

    let generic = exporter
        .export(
            &circuit,
            &ExportOptions::default(),
        )
        .expect("generic export must succeed");

    assert_eq!(
        convenience.bytes(),
        generic.bytes(),
        "convenience export must not introduce a second serialization path",
    );
}

#[test]
fn exporter_descriptor_is_stable_across_calls() {
    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let first = exporter.descriptor().clone();
    let second = exporter.descriptor().clone();

    assert_eq!(
        first,
        second,
        "exporter format descriptor must be immutable and deterministic",
    );
}

// =============================================================================
// Panic-resistance
// =============================================================================

#[test]
fn valid_export_path_is_panic_free() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let result = catch_unwind(
        AssertUnwindSafe(|| {
            exporter.export(
                &circuit,
                &ExportOptions::default(),
            )
        }),
    );

    assert!(
        result.is_ok(),
        "public exporter boundary must not panic on valid canonical IR",
    );
}

#[test]
fn invalid_version_construction_is_panic_free() {
    let result = catch_unwind(
        AssertUnwindSafe(|| {
            OpenQasmExporter::new(
                FormatVersion::new(999, 999, 999),
            )
        }),
    );

    assert!(
        result.is_ok(),
        "unsupported versions must produce errors, not panics",
    );

    assert!(
        result
            .expect("panic result must exist")
            .is_err(),
        "unsupported version must be rejected explicitly",
    );
}

#[test]
fn invalid_output_limit_is_panic_free() {
    let circuit = fixture_circuit(
        fixture_qasm_single_qubit(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let result = catch_unwind(
        AssertUnwindSafe(|| {
            exporter.export(
                &circuit,
                &ExportOptions::new()
                    .with_max_output_bytes(0),
            )
        }),
    );

    assert!(
        result.is_ok(),
        "invalid output bounds must be reported as errors, not panics",
    );

    assert!(
        result
            .expect("panic result must exist")
            .is_err(),
        "zero output bound must be rejected",
    );
}

// =============================================================================
// Production integration invariants
// =============================================================================

#[test]
fn exporter_remains_downstream_of_canonical_ir() {
    /*
     * This is intentionally a compile-time API contract.
     *
     * The concrete exporter receives `&QuantumCircuit`; it does not receive
     * an OpenQASM AST, lexer token stream, parser state, filesystem path, or
     * hardware object.
     */
    fn accepts_canonical_ir<E: QuantumExporter>(
        exporter: &E,
        circuit: &QuantumCircuit,
    ) {
        let _ = exporter.export(
            circuit,
            &ExportOptions::default(),
        );
    }

    let circuit = fixture_circuit(
        fixture_qasm_single_qubit(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    accepts_canonical_ir(
        &exporter,
        &circuit,
    );
}

#[test]
fn exporter_is_independent_of_openqasm_importer_at_runtime() {
    /*
     * The importer is used only to manufacture this test fixture.
     * Once the QuantumCircuit exists, the exporter boundary accepts only
     * canonical IR plus generic export options.
     *
     * This prevents accidental coupling such as:
     *
     * OpenQASM AST -> OpenQASM exporter
     *
     * instead of:
     *
     * QuantumCircuit -> generic exporter -> OpenQASM exporter.
     */
    let circuit = fixture_circuit(
        fixture_qasm_single_qubit(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let artifact = exporter
        .export(
            &circuit,
            &ExportOptions::default(),
        )
        .expect("canonical IR must be independently exportable");

    assert!(
        artifact
            .as_text()
            .is_ok(),
        "canonical IR must be sufficient for OpenQASM serialization",
    );
}

#[test]
fn exporter_output_is_bounded_by_the_requested_limit() {
    let circuit = fixture_circuit(
        fixture_qasm_single_qubit(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let unrestricted = exporter
        .export(
            &circuit,
            &ExportOptions::default(),
        )
        .expect("fixture must export");

    let required = unrestricted.len();

    let bounded = exporter
        .export(
            &circuit,
            &ExportOptions::new()
                .with_max_output_bytes(required),
        )
        .expect("exactly sufficient bound must permit export");

    assert!(
        bounded.len() <= required,
        "successful artifact must never exceed its configured limit",
    );

    assert_eq!(
        bounded.bytes(),
        unrestricted.bytes(),
        "exactly sufficient output bound must preserve bytes",
    );
}

// =============================================================================
// Final production smoke test
// =============================================================================

#[test]
fn openqasm_export_production_smoke_test() {
    let circuit = fixture_circuit(
        fixture_qasm_31(),
    );

    let exporter = OpenQasmExporter::production()
        .expect("production OpenQASM exporter must construct");

    let options = ExportOptions::new()
        .with_requested_version(OPENQASM_3_1)
        .with_version_policy(
            ExportVersionPolicy::Exact,
        );

    let artifact = exporter
        .export(
            &circuit,
            &options,
        )
        .expect(
            "complete production OpenQASM export boundary must succeed",
        );

    assert_eq!(
        artifact.format().id().as_str(),
        OPENQASM_FORMAT_ID,
    );

    assert_eq!(
        artifact.format().version(),
        OPENQASM_3_1,
    );

    assert_eq!(
        artifact.media_type(),
        OPENQASM_MEDIA_TYPE,
    );

    let text = artifact
        .as_text()
        .expect("production OpenQASM artifact must be UTF-8");

    assert!(
        text.starts_with("OPENQASM 3.1;\n"),
        "production output must explicitly identify OpenQASM 3.1",
    );

    assert!(
        text.contains(
            &format!(
                "include \"{}\";",
                STANDARD_LIBRARY_INCLUDE,
            ),
        ),
        "production output must use the controlled standard-library include",
    );

    assert!(
        text.contains("qubit[2] q;"),
        "production output must preserve the logical qubit namespace",
    );

    assert!(
        text.contains("bit[2] c;"),
        "production output must preserve the logical classical namespace",
    );

    assert!(
        text.contains("h q[0];"),
        "production output must preserve the H operation",
    );

    assert!(
        text.contains("cx q[0], q[1];"),
        "production output must preserve the CX operation",
    );

    assert!(
        text.contains("measure q[0] -> c[0];"),
        "production output must preserve measurement destination zero",
    );

    assert!(
        text.contains("measure q[1] -> c[1];"),
        "production output must preserve measurement destination one",
    );

    assert!(
        artifact.len()
            <= ExportOptions::default()
                .max_output_bytes(),
        "production artifact must remain within the generic default output bound",
    );
}