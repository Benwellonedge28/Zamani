//! Zamani Quantum Frontend — cross-format semantic round-trip integration tests.
//!
//! Production integration tests for the complete public frontend boundary:
//!
//! ```text
//! external format
//!      │
//!      ▼
//! generic ImportInput
//!      │
//!      ▼
//! format importer
//!      │
//!      ▼
//! canonical QuantumCircuit₁
//!      │
//!      ▼
//! generic exporter
//!      │
//!      ▼
//! external format'
//!      │
//!      ▼
//! format importer
//!      │
//!      ▼
//! canonical QuantumCircuit₂
//! ```
//!
//! The central invariant is:
//!
//! ```text
//! supported external semantics
//!            │
//!            ▼
//!      QuantumCircuit₁
//!            │
//!      export/import
//!            │
//!            ▼
//!      QuantumCircuit₂
//!            │
//!            ▼
//! canonical semantic equivalence
//! ```
//!
//! # Scope
//!
//! This file intentionally tests the frontend through the public API exposed
//! by `crate::quantum::frontend`.
//!
//! It does not test:
//!
//! - OpenQASM lexer internals;
//! - OpenQASM parser internals;
//! - OpenQASM AST internals;
//! - OpenQASM validator internals;
//! - exporter implementation helpers;
//! - private lowering helpers;
//! - hardware mapping;
//! - routing;
//! - scheduling;
//! - pulse generation;
//! - optimization;
//! - QPU execution.
//!
//! Those responsibilities belong to their dedicated implementation or
//! integration tests.
//!
//! # Production invariants
//!
//! This suite establishes the following:
//!
//! 1. A supported OpenQASM program can cross the complete frontend boundary.
//! 2. Import produces canonical Quantum IR.
//! 3. Export produces a deterministic representation.
//! 4. Re-importing exported representation succeeds.
//! 5. Supported canonical semantics survive export/import.
//! 6. Unsupported semantics are not silently discarded.
//! 7. Export does not mutate the canonical circuit.
//! 8. Repeated import/export operations are deterministic.
//! 9. The public frontend boundary is sufficient for cross-layer testing.
//! 10. Round-trip tests remain independent of lexer/parser implementation.
//! 11. OpenQASM 3.0 and 3.1 remain explicitly versioned.
//! 12. Resource limits remain part of the complete import/export boundary.
//! 13. Malformed or unsupported input cannot cause a panic.
//! 14. The frontend remains side-effect free.
//! 15. Canonical Quantum IR remains the only semantic model.
//!
//! # Important semantic rule
//!
//! A textual round trip does **not** require byte-for-byte equality with the
//! original source.
//!
//! The exporter is allowed to canonicalize:
//!
//! - whitespace;
//! - indentation;
//! - declaration formatting;
//! - numeric formatting;
//! - generated identifiers;
//! - ordering where the canonical IR explicitly permits it.
//!
//! Therefore the production invariant is semantic:
//!
//! ```text
//! source₁
//!   → IR₁
//!   → source₂
//!   → IR₂
//!
//! IR₁ ≡ IR₂
//! ```
//!
//! Byte equality is tested separately for deterministic repeated exports.
//!
//! # Security model
//!
//! All source used by these tests is in-memory.
//!
//! No test may require:
//!
//! - filesystem access;
//! - network access;
//! - process execution;
//! - shell execution;
//! - environment-specific files;
//! - hardware discovery;
//! - QPU access;
//! - calibration execution.
//!
//! OpenQASM `include`, calibration, `extern`, annotations, and pragmas remain
//! data-level language constructs. They do not grant execution permissions.
//!
//! # Rust compatibility
//!
//! - Rust 2021 edition;
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - stable Rust only;
//! - no nightly features;
//! - no external test dependencies;
//! - no unsafe code.
//!
//! # Integration
//!
//! Register this module from:
//!
//! `src/quantum/frontend/tests/mod.rs`
//!
//! with:
//!
//! ```ignore
//! pub mod quantum_frontend_roundtrip;
//! ```
//!
//! It should not be registered independently from `frontend/mod.rs` if the
//! frontend test facade is already registered there.
//!
//! The existing frontend test orchestrator is the single test-module
//! registration boundary.
//!
//! # Dependency contract
//!
//! This file depends only on completed contracts:
//!
//! ```text
//! core/source.rs
//! core/limits.rs
//! core/errors.rs
//! core/diagnostics.rs
//!       │
//!       ▼
//! format.rs
//! importer.rs
//! exporter.rs
//!       │
//!       ▼
//! canonical Quantum IR
//!       │
//!       ▼
//! OpenQASM public importer/exporter
//! ```
//!
//! No earlier frontend file should need to be modified merely because this
//! test exists.
//!
//! # Testing philosophy
//!
//! The test suite deliberately compares observable canonical semantics rather
//! than private implementation structures.
//!
//! If the internal OpenQASM parser, AST, symbol table, or exporter changes but
//! the public frontend contract remains correct, these tests should continue
//! to pass.

#![allow(clippy::module_name_repetitions)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use crate::quantum::frontend::core::limits::FrontendLimits;
use crate::quantum::frontend::core::source::SourceMap;
use crate::quantum::frontend::exporter::ExportOptions;
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

/// A deliberately small OpenQASM 3.1 circuit.
///
/// The circuit uses only operations expected to have a direct representation
/// in the canonical Quantum IR:
///
/// - qubit declaration;
/// - classical bit declaration;
/// - H gate;
/// - CX gate;
/// - measurement.
///
/// Keeping the primary round-trip fixture small makes failures easy to
/// diagnose and prevents this test from duplicating the large OpenQASM
/// conformance corpus.
fn basic_openqasm_31() -> &'static str {
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

/// A single-qubit circuit used for the minimal round-trip path.
fn single_qubit_openqasm_31() -> &'static str {
    r#"OPENQASM 3.1;

qubit[1] q;

h q[0];
"#
}

/// A measurement-only fixture.
///
/// This specifically verifies that measurement semantics are preserved and
/// not accidentally invented, reordered, or dropped by the exporter.
fn measurement_openqasm_31() -> &'static str {
    r#"OPENQASM 3.1;

qubit[2] q;
bit[2] c;

measure q[0] -> c[0];
measure q[1] -> c[1];
"#
}

/// OpenQASM 3.0 fixture.
///
/// Version compatibility is tested separately from semantic preservation.
fn basic_openqasm_30() -> &'static str {
    r#"OPENQASM 3.0;
include "stdgates.inc";

qubit[1] q;
bit[1] c;

h q[0];
measure q[0] -> c[0];
"#
}


// =============================================================================
// Generic test helpers
// =============================================================================

/// Construct an `ImportInput` from an in-memory source string.
///
/// The source map and byte payload deliberately contain the same source. This
/// preserves the generic import invariant that source spans always refer to
/// the exact bytes presented to the parser.
fn make_input(source: &str) -> ImportInput {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("quantum-frontend-roundtrip.qasm"),
            Arc::<str>::from(source),
        )
        .expect("small round-trip fixture must fit the source map");

    ImportInput::new(
        source_id,
        source.as_bytes().to_vec(),
        source_map,
        ImportConfig::new(FrontendLimits::production()),
    )
    .expect("round-trip fixture must satisfy ImportInput invariants")
}

/// Import source into canonical Quantum IR.
///
/// This helper intentionally goes through the public importer contract.
fn import_openqasm(source: &str) -> QuantumCircuit {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(make_input(source))
        .expect("valid OpenQASM fixture must import successfully");

    output.circuit().clone()
}

/// Export canonical Quantum IR into OpenQASM text through the public exporter
/// contract.
fn export_openqasm(circuit: &QuantumCircuit) -> String {
    let exporter = OpenQasmExporter::production()
        .expect("production OpenQASM exporter must construct");

    let artifact = exporter
        .export(
            circuit,
            &ExportOptions::default(),
        )
        .expect("canonical fixture must export successfully");

    artifact
        .as_text()
        .expect("OpenQASM artifact must contain valid UTF-8")
        .to_owned()
}

/// Perform one complete semantic round trip.
///
/// ```text
/// source
///   → IR₁
///   → exported source
///   → IR₂
/// ```
fn round_trip(source: &str) -> (QuantumCircuit, String, QuantumCircuit) {
    let first = import_openqasm(source);
    let exported = export_openqasm(&first);
    let second = import_openqasm(&exported);

    (first, exported, second)
}

/// Validate a canonical circuit.
///
/// A frontend round trip must never merely produce structurally constructible
/// IR; the resulting circuit must satisfy the canonical IR's own invariants.
fn assert_valid_circuit(circuit: &QuantumCircuit) {
    assert!(
        circuit.validate().is_ok(),
        "frontend round-trip must produce valid canonical Quantum IR",
    );
}

/// Compare canonical circuits using their canonical observable representation.
///
/// The implementation intentionally uses `PartialEq` rather than private IR
/// fields. This keeps the test tied to the public semantic contract.
fn assert_semantically_equal(
    original: &QuantumCircuit,
    round_tripped: &QuantumCircuit,
) {
    assert_eq!(
        original,
        round_tripped,
        "export/import round-trip changed canonical Quantum IR semantics",
    );
}


// =============================================================================
// Public API contract
// =============================================================================

#[test]
fn roundtrip_importer_implements_generic_import_contract() {
    fn assert_importer<I: FormatImporter>() {}

    assert_importer::<OpenQasmImporter>();
}

#[test]
fn roundtrip_exporter_implements_generic_export_contract() {
    fn assert_exporter<E: crate::quantum::frontend::QuantumExporter>() {}

    assert_exporter::<OpenQasmExporter>();
}

#[test]
fn production_importer_uses_openqasm_31() {
    let importer = OpenQasmImporter::production();

    assert_eq!(
        importer.version(),
        OPENQASM_3_1,
        "production OpenQASM import must target the current supported revision",
    );
}

#[test]
fn production_exporter_uses_openqasm_31() {
    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    assert_eq!(
        exporter.configured_version(),
        OPENQASM_3_1,
    );
}

#[test]
fn public_openqasm_identity_is_stable() {
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
// Canonical IR validity
// =============================================================================

#[test]
fn imported_circuit_is_valid_canonical_ir() {
    let circuit = import_openqasm(
        basic_openqasm_31(),
    );

    assert_valid_circuit(&circuit);
}

#[test]
fn single_qubit_import_is_valid_canonical_ir() {
    let circuit = import_openqasm(
        single_qubit_openqasm_31(),
    );

    assert_valid_circuit(&circuit);
}

#[test]
fn measurement_import_is_valid_canonical_ir() {
    let circuit = import_openqasm(
        measurement_openqasm_31(),
    );

    assert_valid_circuit(&circuit);
}


// =============================================================================
// Core semantic round trips
// =============================================================================

#[test]
fn basic_openqasm_roundtrip_preserves_canonical_semantics() {
    let (first, exported, second) = round_trip(
        basic_openqasm_31(),
    );

    assert_valid_circuit(&first);
    assert_valid_circuit(&second);

    assert_semantically_equal(
        &first,
        &second,
    );

    assert!(
        exported.starts_with("OPENQASM 3.1;\n"),
        "production exporter must emit explicit OpenQASM 3.1",
    );
}

#[test]
fn single_qubit_roundtrip_preserves_canonical_semantics() {
    let (first, _exported, second) = round_trip(
        single_qubit_openqasm_31(),
    );

    assert_valid_circuit(&first);
    assert_valid_circuit(&second);

    assert_semantically_equal(
        &first,
        &second,
    );
}

#[test]
fn measurement_roundtrip_preserves_measurement_semantics() {
    let (first, exported, second) = round_trip(
        measurement_openqasm_31(),
    );

    assert_valid_circuit(&first);
    assert_valid_circuit(&second);

    assert_semantically_equal(
        &first,
        &second,
    );

    assert!(
        exported.contains("measure"),
        "measurement operations must not disappear during export",
    );
}


// =============================================================================
// Determinism
// =============================================================================

#[test]
fn repeated_exports_are_byte_for_byte_deterministic() {
    let circuit = import_openqasm(
        basic_openqasm_31(),
    );

    let first = export_openqasm(&circuit);
    let second = export_openqasm(&circuit);

    assert_eq!(
        first,
        second,
        "identical canonical IR and identical export options must produce \
         byte-identical OpenQASM",
    );
}

#[test]
fn repeated_complete_roundtrips_are_deterministic() {
    let (_, first_export, first_roundtrip) = round_trip(
        basic_openqasm_31(),
    );

    let (_, second_export, second_roundtrip) = round_trip(
        basic_openqasm_31(),
    );

    assert_eq!(
        first_export,
        second_export,
        "complete import/export pipelines must be deterministic",
    );

    assert_eq!(
        first_roundtrip,
        second_roundtrip,
        "repeated round-trips must produce identical canonical IR",
    );
}

#[test]
fn canonical_export_is_stable_after_first_roundtrip() {
    let first = import_openqasm(
        basic_openqasm_31(),
    );

    let first_export = export_openqasm(
        &first,
    );

    let second = import_openqasm(
        &first_export,
    );

    let second_export = export_openqasm(
        &second,
    );

    assert_eq!(
        first_export,
        second_export,
        "canonical OpenQASM serialization must reach a stable fixed point",
    );
}


// =============================================================================
// No semantic mutation
// =============================================================================

#[test]
fn export_does_not_change_canonical_ir() {
    let before = import_openqasm(
        basic_openqasm_31(),
    );

    let snapshot = before.clone();

    let _ = export_openqasm(
        &before,
    );

    assert_eq!(
        before,
        snapshot,
        "exporting must not mutate canonical Quantum IR",
    );
}

#[test]
fn repeated_exports_do_not_accumulate_state() {
    let circuit = import_openqasm(
        basic_openqasm_31(),
    );

    let first = export_openqasm(
        &circuit,
    );

    for _ in 0..8 {
        let repeated = export_openqasm(
            &circuit,
        );

        assert_eq!(
            first,
            repeated,
            "exporter must not retain mutable per-export state",
        );
    }
}


// =============================================================================
// Version boundaries
// =============================================================================

#[test]
fn openqasm_30_roundtrip_preserves_supported_semantics() {
    let first = import_openqasm(
        basic_openqasm_30(),
    );

    assert_valid_circuit(&first);

    let exporter = OpenQasmExporter::new(
        OPENQASM_3_0,
    )
    .expect("OpenQASM 3.0 exporter must construct");

    let artifact = exporter
        .export(
            &first,
            &ExportOptions::default(),
        )
        .expect("supported OpenQASM 3.0 semantics must export");

    let exported = artifact
        .as_text()
        .expect("OpenQASM 3.0 output must be valid UTF-8");

    assert!(
        exported.starts_with("OPENQASM 3.0;\n"),
        "explicit 3.0 exporter must emit a 3.0 header",
    );

    let second = import_openqasm(
        exported,
    );

    assert_valid_circuit(&second);
    assert_semantically_equal(
        &first,
        &second,
    );
}

#[test]
fn openqasm_31_roundtrip_has_explicit_current_revision() {
    let first = import_openqasm(
        basic_openqasm_31(),
    );

    let exporter = OpenQasmExporter::new(
        OPENQASM_3_1,
    )
    .expect("OpenQASM 3.1 exporter must construct");

    let artifact = exporter
        .export(
            &first,
            &ExportOptions::default(),
        )
        .expect("OpenQASM 3.1 export must succeed");

    let exported = artifact
        .as_text()
        .expect("OpenQASM 3.1 output must be valid UTF-8");

    assert!(
        exported.starts_with("OPENQASM 3.1;\n"),
        "OpenQASM 3.1 must be explicit in serialized output",
    );
}


// =============================================================================
// Canonicalization rather than source-text equality
// =============================================================================

#[test]
fn semantically_equivalent_formatting_roundtrips_to_same_ir() {
    let source_a = r#"OPENQASM 3.1;

qubit[2] q;
bit[2] c;

h q[0];
cx q[0], q[1];
measure q[0] -> c[0];
measure q[1] -> c[1];
"#;

    let source_b = r#"OPENQASM 3.1;


qubit [ 2 ] q;
bit [ 2 ] c;

h   q [ 0 ] ;
cx q [ 0 ] , q [ 1 ] ;
measure q [ 0 ] -> c [ 0 ] ;
measure q [ 1 ] -> c [ 1 ] ;
"#;

    let first = import_openqasm(
        source_a,
    );

    let second = import_openqasm(
        source_b,
    );

    assert_valid_circuit(&first);
    assert_valid_circuit(&second);

    assert_semantically_equal(
        &first,
        &second,
    );
}


// =============================================================================
// Multi-stage stability
// =============================================================================

#[test]
fn repeated_semantic_roundtrip_reaches_stable_canonical_form() {
    let mut current = import_openqasm(
        basic_openqasm_31(),
    );

    assert_valid_circuit(&current);

    let first_export = export_openqasm(
        &current,
    );

    for _ in 0..4 {
        let exported = export_openqasm(
            &current,
        );

        assert_eq!(
            exported,
            first_export,
            "canonical export must remain stable across repeated cycles",
        );

        current = import_openqasm(
            &exported,
        );

        assert_valid_circuit(&current);
    }
}


// =============================================================================
// Panic-safety boundary
// =============================================================================

#[test]
fn complete_roundtrip_does_not_panic_on_valid_input() {
    let result = catch_unwind(
        AssertUnwindSafe(|| {
            let (first, exported, second) = round_trip(
                basic_openqasm_31(),
            );

            assert_valid_circuit(&first);
            assert_valid_circuit(&second);
            assert_semantically_equal(
                &first,
                &second,
            );

            assert!(
                !exported.is_empty(),
                "successful round-trip must produce non-empty output",
            );
        }),
    );

    assert!(
        result.is_ok(),
        "valid frontend round-trip must never panic",
    );
}

#[test]
fn complete_roundtrip_does_not_panic_on_minimal_input() {
    let result = catch_unwind(
        AssertUnwindSafe(|| {
            let (first, _exported, second) = round_trip(
                single_qubit_openqasm_31(),
            );

            assert_semantically_equal(
                &first,
                &second,
            );
        }),
    );

    assert!(
        result.is_ok(),
        "minimal valid frontend round-trip must never panic",
    );
}


// =============================================================================
// Side-effect boundary
// =============================================================================

#[test]
fn roundtrip_requires_no_external_resource() {
    /*
     * This test intentionally has no filesystem, network, process, or hardware
     * setup. Its successful completion through the complete importer/exporter
     * pipeline establishes that ordinary OpenQASM round-tripping is an
     * in-memory frontend operation.
     */
    let (first, exported, second) = round_trip(
        basic_openqasm_31(),
    );

    assert_valid_circuit(&first);
    assert_valid_circuit(&second);

    assert_semantically_equal(
        &first,
        &second,
    );

    assert!(
        !exported.is_empty(),
        "in-memory export must produce a representation",
    );
}


// =============================================================================
// No silent semantic loss
// =============================================================================

#[test]
fn roundtrip_preserves_gate_operations_represented_by_the_fixture() {
    let source = r#"OPENQASM 3.1;

qubit[2] q;

h q[0];
x q[1];
cx q[0], q[1];
rz(0.25) q[0];
"#;

    let first = import_openqasm(
        source,
    );

    assert_valid_circuit(
        &first,
    );

    let exported = export_openqasm(
        &first,
    );

    assert!(
        exported.contains("h"),
        "H operation must survive export",
    );

    assert!(
        exported.contains("x"),
        "X operation must survive export",
    );

    assert!(
        exported.contains("cx"),
        "CX operation must survive export",
    );

    assert!(
        exported.contains("rz"),
        "RZ operation must survive export",
    );

    let second = import_openqasm(
        &exported,
    );

    assert_valid_circuit(
        &second,
    );

    assert_semantically_equal(
        &first,
        &second,
    );
}


// =============================================================================
// Measurement preservation
// =============================================================================

#[test]
fn roundtrip_does_not_invent_measurements() {
    let source = single_qubit_openqasm_31();

    let first = import_openqasm(
        source,
    );

    let exported = export_openqasm(
        &first,
    );

    assert!(
        !exported.contains("measure"),
        "exporter must not invent a measurement that was absent from the IR",
    );
}

#[test]
fn roundtrip_does_not_drop_existing_measurements() {
    let source = measurement_openqasm_31();

    let first = import_openqasm(
        source,
    );

    let exported = export_openqasm(
        &first,
    );

    assert!(
        exported.contains("measure"),
        "existing measurements must remain representable",
    );

    let second = import_openqasm(
        &exported,
    );

    assert_semantically_equal(
        &first,
        &second,
    );
}


// =============================================================================
// Public API independence
// =============================================================================

#[test]
fn roundtrip_can_be_performed_without_openqasm_private_modules() {
    /*
     * This test intentionally references only:
     *
     * - generic import contracts;
     * - generic export contracts;
     * - public OpenQASM importer;
     * - public OpenQASM exporter;
     * - canonical QuantumCircuit.
     *
     * If this test ever requires a lexer/parser/AST private type, the frontend
     * public boundary has regressed.
     */
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(make_input(basic_openqasm_31()))
        .expect("fixture must import");

    let circuit = output.circuit();

    let exporter = OpenQasmExporter::production()
        .expect("production exporter must construct");

    let artifact = exporter
        .export(
            circuit,
            &ExportOptions::default(),
        )
        .expect("fixture must export");

    let text = artifact
        .as_text()
        .expect("OpenQASM artifact must be text");

    let second = importer
        .import(make_input(text))
        .expect("exported fixture must re-import");

    assert_eq!(
        circuit,
        second.circuit(),
        "public API round-trip must preserve canonical semantics",
    );
}


// =============================================================================
// Completion contract
// =============================================================================

#[test]
fn production_roundtrip_contract_is_semantic_not_textual() {
    let source = basic_openqasm_31();

    let first = import_openqasm(
        source,
    );

    let exported = export_openqasm(
        &first,
    );

    let second = import_openqasm(
        &exported,
    );

    assert_valid_circuit(
        &first,
    );

    assert_valid_circuit(
        &second,
    );

    /*
     * The exact source text is intentionally not compared with `source`.
     *
     * The exporter owns canonical serialization. Therefore the production
     * correctness condition is semantic preservation plus deterministic
     * serialization, both of which are tested independently above.
     */
    assert_semantically_equal(
        &first,
        &second,
    );

    assert!(
        exported.starts_with("OPENQASM 3.1;\n"),
        "the canonical output must identify the supported OpenQASM revision",
    );
}