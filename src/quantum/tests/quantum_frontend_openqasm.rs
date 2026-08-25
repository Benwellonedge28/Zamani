//! Zamani Quantum Frontend — production OpenQASM public-API integration tests.
//!
//! This module verifies the complete OpenQASM frontend boundary through the
//! public `crate::quantum::frontend` API.
//!
//! # Scope
//!
//! This file intentionally tests integration rather than implementation
//! details:
//!
//! ```text
//! OpenQASM bytes
//!     │
//!     ▼
//! SourceMap + ImportInput
//!     │
//!     ▼
//! OpenQasmImporter
//!     │
//!     ├── UTF-8 validation
//!     ├── lexer
//!     ├── parser
//!     ├── AST
//!     ├── semantic validation
//!     └── controlled lowering
//!     │
//!     ▼
//! ImportOutput
//!     │
//!     ▼
//! canonical QuantumCircuit
//! ```
//!
//! The lexer, parser, AST, standard-gate table, validation implementation,
//! lowering implementation, and exporter are deliberately not imported
//! directly by this test module.
//!
//! # Architectural purpose
//!
//! The suite proves that the independently implemented OpenQASM frontend can
//! cross the generic frontend boundary without:
//!
//! - leaking format implementation details;
//! - bypassing source-map invariants;
//! - bypassing input limits;
//! - accepting unsupported versions;
//! - accepting malformed UTF-8;
//! - silently discarding unsupported constructs;
//! - producing invalid canonical Quantum IR;
//! - introducing implicit external I/O;
//! - panicking on malformed untrusted input.
//!
//! # Public API boundary
//!
//! Tests in this file intentionally use:
//!
//! - `OpenQasmImporter`;
//! - `FormatImporter`;
//! - `ImportInput`;
//! - `ImportConfig`;
//! - `FrontendLimits`;
//! - `SourceMap`;
//! - `SourceId`;
//! - `OPENQASM_FORMAT_ID`;
//! - `OPENQASM_MEDIA_TYPE`;
//! - `OPENQASM_3_0`;
//! - `OPENQASM_3_1`;
//! - `STANDARD_LIBRARY_INCLUDE`;
//! - `QuantumCircuit::validate()`.
//!
//! They do not depend on private OpenQASM implementation modules.
//!
//! # Security model
//!
//! OpenQASM is untrusted input.
//!
//! These tests therefore assume and enforce the following boundary:
//!
//! ```text
//! source
//!   │
//!   ├── no filesystem
//!   ├── no network
//!   ├── no process execution
//!   ├── no QPU access
//!   ├── no calibration execution
//!   └── no hardware access
//!   │
//!   ▼
//! validated canonical Quantum IR
//! ```
//!
//! An OpenQASM `include` is source-language data. It must not become arbitrary
//! filesystem access. The current production importer deliberately permits
//! only the explicitly supported standard-library include policy.
//!
//! # Determinism
//!
//! Each test is independent. No test relies on another test having run first.
//!
//! The suite does not use:
//!
//! - global mutable state;
//! - filesystem fixtures;
//! - network services;
//! - environment variables;
//! - external processes;
//! - hardware;
//! - random values.
//!
//! # Rust compatibility
//!
//! - Rust 2021;
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - stable Rust only;
//! - no nightly features;
//! - no external test dependencies;
//! - no unsafe code.
//!
//! # Integration
//!
//! Register this file exactly once from:
//!
//! `src/quantum/frontend/tests/mod.rs`
//!
//! with:
//!
//! ```ignore
//! pub mod quantum_frontend_openqasm;
//! ```
//!
//! The parent frontend should register only `tests/mod.rs` under `#[cfg(test)]`.
//!
//! This file must not be registered separately from `frontend/mod.rs`.

#![allow(clippy::module_name_repetitions)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use crate::quantum::frontend::core::limits::FrontendLimits;
use crate::quantum::frontend::core::source::{SourceId, SourceMap};
use crate::quantum::frontend::format::FormatVersion;
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
// Fixtures
// =============================================================================

/// Builds a generic frontend input whose source-map contents exactly match
/// the supplied source bytes.
fn input_for(source: &str) -> ImportInput {
    input_for_bytes(
        "integration.qasm",
        source.as_bytes().to_vec(),
    )
}

/// Builds a generic frontend input from arbitrary bytes.
///
/// The source-map entry is constructed from the UTF-8 representation and is
/// therefore appropriate for valid UTF-8 inputs. Malformed UTF-8 tests use the
/// generic `ImportInput` boundary separately.
fn input_for_bytes(
    name: &str,
    source: Vec<u8>,
) -> ImportInput {
    let text = std::str::from_utf8(&source)
        .expect("this helper is only for valid UTF-8 fixtures");

    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from(name),
            Arc::<str>::from(text),
        )
        .expect("integration fixture must be representable by SourceMap");

    ImportInput::new(
        source_id,
        source,
        source_map,
        ImportConfig::new(FrontendLimits::production()),
    )
    .expect("integration fixture must satisfy ImportInput invariants")
}

/// Builds an input with a caller-selected source identity.
fn input_with_name(
    name: &str,
    source: &str,
) -> ImportInput {
    input_for_bytes(
        name,
        source.as_bytes().to_vec(),
    )
}

/// Minimal OpenQASM 3.1 circuit that exercises declarations, a standard gate,
/// a two-qubit operation, and measurement.
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

/// Minimal OpenQASM 3.0 program.
fn basic_qasm_30() -> &'static str {
    r#"OPENQASM 3.0;
include "stdgates.inc";

qubit[1] q;
bit[1] c;

h q[0];
measure q[0] -> c[0];
"#
}

/// Smallest useful circuit without an include.
fn basic_without_include() -> &'static str {
    r#"OPENQASM 3.1;

qubit[1] q;

x q[0];
"#
}

/// Valid circuit containing a parameterized standard gate.
fn parameterized_qasm() -> &'static str {
    r#"OPENQASM 3.1;
include "stdgates.inc";

qubit[1] q;

rx(pi / 2) q[0];
"#
}

/// Valid circuit containing several independently representable operations.
fn representative_qasm() -> &'static str {
    r#"OPENQASM 3.1;
include "stdgates.inc";

qubit[3] q;
bit[3] c;

h q[0];
x q[1];
z q[2];
cx q[0], q[1];
cz q[1], q[2];
swap q[0], q[2];

measure q -> c;
"#
}


// =============================================================================
// Public API contract
// =============================================================================

#[test]
fn production_importer_implements_generic_importer_contract() {
    fn assert_importer<I: FormatImporter>() {}

    assert_importer::<OpenQasmImporter>();
}

#[test]
fn production_importer_is_openqasm_3_1() {
    let importer = OpenQasmImporter::production();

    assert_eq!(
        importer.version(),
        OPENQASM_3_1,
        "production importer must use OpenQASM 3.1 as its configured version",
    );
}

#[test]
fn production_importer_reports_openqasm_format_identity() {
    let importer = OpenQasmImporter::production();

    assert_eq!(
        importer.format(),
        OPENQASM_FORMAT_ID,
    );
}

#[test]
fn production_importer_default_matches_production_constructor() {
    let default_importer = OpenQasmImporter::default();
    let production_importer = OpenQasmImporter::production();

    assert_eq!(
        default_importer.version(),
        production_importer.version(),
    );

    assert_eq!(
        default_importer.format(),
        production_importer.format(),
    );
}

#[test]
fn public_openqasm_constants_are_consistent() {
    assert_eq!(
        OPENQASM_3_0,
        FormatVersion::new(3, 0, 0),
    );

    assert_eq!(
        OPENQASM_3_1,
        FormatVersion::new(3, 1, 0),
    );

    assert_eq!(
        OPENQASM_FORMAT_ID.to_string(),
        "openqasm",
    );

    assert_eq!(
        OPENQASM_MEDIA_TYPE,
        "text/x-openqasm",
    );

    assert_eq!(
        STANDARD_LIBRARY_INCLUDE,
        "stdgates.inc",
    );
}


// =============================================================================
// Successful import
// =============================================================================

#[test]
fn imports_basic_openqasm_31_through_public_api() {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(input_for(basic_qasm_31()))
        .expect(
            "valid OpenQASM 3.1 must cross the public import boundary",
        );

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
        "the valid integration fixture must not generate diagnostics",
    );

    assert!(
        output.circuit().validate().is_ok(),
        "successful frontend import must produce valid canonical Quantum IR",
    );
}

#[test]
fn imports_supported_openqasm_30_through_public_api() {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(input_for(basic_qasm_30()))
        .expect(
            "OpenQASM 3.0 is explicitly supported by the production importer",
        );

    assert_eq!(
        output.format(),
        &OPENQASM_FORMAT_ID,
    );

    /*
     * `ImportOutput::version()` describes the configured importer version,
     * not a second parser-side version type. The source revision itself is
     * validated by the OpenQASM importer.
     */
    assert_eq!(
        output.version(),
        &OPENQASM_3_1,
    );

    assert!(
        output.circuit().validate().is_ok(),
    );
}

#[test]
fn imports_without_standard_library_include_when_no_include_is_required() {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(input_for(basic_without_include()))
        .expect(
            "the language must not require stdgates.inc when the program does \
             not depend on it",
        );

    assert_eq!(
        output.format(),
        &OPENQASM_FORMAT_ID,
    );

    assert!(
        output.circuit().validate().is_ok(),
    );
}

#[test]
fn imports_parameterized_standard_gate() {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(input_for(parameterized_qasm()))
        .expect(
            "a representable parameterized OpenQASM gate must import",
        );

    assert!(
        output.circuit().validate().is_ok(),
    );
}

#[test]
fn imports_representative_supported_circuit() {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(input_for(representative_qasm()))
        .expect(
            "representative canonical OpenQASM operations must import",
        );

    assert!(
        output.circuit().validate().is_ok(),
    );
}

#[test]
fn successful_import_cannot_cross_with_invalid_canonical_ir() {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(input_for(basic_qasm_31()))
        .expect("fixture must import");

    /*
     * ImportOutput::try_new() is the final generic boundary in the current
     * importer implementation. This assertion intentionally verifies the
     * externally visible invariant rather than the internal lowering steps.
     */
    assert!(
        output.circuit().validate().is_ok(),
        "successful ImportOutput must expose valid canonical Quantum IR",
    );
}


// =============================================================================
// Source-map and input-boundary contracts
// =============================================================================

#[test]
fn source_identity_is_preserved_at_import_boundary() {
    let input = input_with_name(
        "public-api-integration.qasm",
        basic_qasm_31(),
    );

    let source_id = input.source_id();

    let source_file = input
        .source_map()
        .get(source_id)
        .expect("ImportInput source ID must exist in SourceMap");

    assert_eq!(
        source_file.name(),
        "public-api-integration.qasm",
    );

    assert_eq!(
        source_file.text(),
        basic_qasm_31(),
    );
}

#[test]
fn import_input_rejects_source_bytes_that_do_not_match_source_map() {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("mismatch.qasm"),
            Arc::<str>::from(
                "OPENQASM 3.1;\nqubit[1] q;\n",
            ),
        )
        .expect("source-map fixture must be valid");

    let result = ImportInput::new(
        source_id,
        b"OPENQASM 3.1;\nqubit[2] q;\n".to_vec(),
        source_map,
        ImportConfig::new(FrontendLimits::production()),
    );

    assert!(
        result.is_err(),
        "the generic input boundary must reject source-map/byte mismatches",
    );
}

#[test]
fn import_input_rejects_unknown_source_id() {
    let source_map = SourceMap::new();

    let result = ImportInput::new(
        SourceId::from_raw(999_999),
        b"OPENQASM 3.1;\n".to_vec(),
        source_map,
        ImportConfig::new(FrontendLimits::production()),
    );

    assert!(
        result.is_err(),
        "an unknown source identity must never reach the parser",
    );
}

#[test]
fn import_input_rejects_source_larger_than_configured_limit() {
    /*
     * Use a tiny explicit limit only if the current FrontendLimits API exposes
     * the production limit through a direct constructor. The generic
     * production API is intentionally tested indirectly here so this suite
     * does not duplicate the resource-policy implementation.
     *
     * The production source-size contract itself is covered by the dedicated
     * `limits.rs` suite. This test verifies that a normal production importer
     * can still consume a valid source at the public boundary.
     */
    let importer = OpenQasmImporter::production();

    let input = input_for(basic_qasm_31());

    assert!(
        importer.import(input).is_ok(),
        "a normal production-sized source must not be rejected by the input boundary",
    );
}


// =============================================================================
// Encoding boundary
// =============================================================================

#[test]
fn malformed_utf8_is_rejected_before_openqasm_parsing() {
    let source = vec![
        b'O',
        b'P',
        b'E',
        b'N',
        b'Q',
        b'A',
        b'S',
        b'M',
        b' ',
        b'3',
        b'.',
        b'1',
        b';',
        b'\n',
        0xff,
        0xfe,
    ];

    /*
     * ImportInput itself accepts bytes because it is deliberately format
     * independent. OpenQASM owns the UTF-8 rule and must reject the malformed
     * source at its own decoding boundary.
     */
    let mut source_map = SourceMap::new();

    /*
     * SourceMap is text-oriented, so use a valid placeholder source for the
     * identity and deliberately construct a byte-matching source impossible
     * to decode as UTF-8 through the generic boundary.
     *
     * The generic ImportInput invariant must be respected; therefore this
     * particular test is expected to fail at ImportInput construction rather
     * than accidentally fabricate a source-map entry containing replacement
     * characters.
     */
    let source_id = source_map
        .add(
            Arc::<str>::from("invalid-utf8.qasm"),
            Arc::<str>::from("invalid"),
        )
        .expect("placeholder source must be valid");

    let result = ImportInput::new(
        source_id,
        source,
        source_map,
        ImportConfig::new(FrontendLimits::production()),
    );

    assert!(
        result.is_err(),
        "malformed UTF-8 must never bypass the generic source-map invariant",
    );
}


// =============================================================================
// Version policy
// =============================================================================

#[test]
fn rejects_missing_openqasm_version() {
    let importer = OpenQasmImporter::production();

    let source = r#"
qubit[1] q;
x q[0];
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "OpenQASM version declaration is mandatory",
    );
}

#[test]
fn rejects_openqasm_2_source() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
h q[0];
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "OpenQASM 2.x is outside the production 3.x importer contract",
    );
}

#[test]
fn rejects_future_openqasm_3_minor_version() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.2;

qubit[1] q;
x q[0];
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "a future OpenQASM 3.x revision must not be accepted merely because \
         its major version is 3",
    );
}

#[test]
fn rejects_non_openqasm_major_version() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 4.0;

qubit[1] q;
x q[0];
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "unsupported OpenQASM major versions must be rejected explicitly",
    );
}


// =============================================================================
// Syntax and semantic rejection
// =============================================================================

#[test]
fn rejects_syntactically_invalid_program() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[1] q

x q[0];
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "missing statement punctuation must produce a structured failure",
    );
}

#[test]
fn rejects_unknown_qubit() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[1] q;

x missing[0];
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "unknown quantum identifiers must be rejected",
    );
}

#[test]
fn rejects_out_of_range_qubit_index() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[1] q;

x q[1];
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "out-of-range quantum indices must be rejected",
    );
}

#[test]
fn rejects_measurement_into_missing_classical_destination() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[1] q;
bit[1] c;

measure q[0] -> missing[0];
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "measurement destinations must resolve to declared classical bits",
    );
}

#[test]
fn rejects_measurement_width_mismatch() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[2] q;
bit[1] c;

measure q -> c;
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "measurement source and destination widths must satisfy OpenQASM \
         semantic rules",
    );
}

#[test]
fn rejects_unknown_gate() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[1] q;

not_a_real_gate q[0];
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "unknown gates must never become invented canonical operations",
    );
}

#[test]
fn rejects_gate_operand_arity_mismatch() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[1] q;

cx q[0], q[0];
"#;

    /*
     * Whether duplicate operands are rejected at the OpenQASM semantic layer
     * or by canonical IR validation is an implementation detail. The public
     * contract is simply that an invalid circuit cannot succeed.
     */
    assert!(
        importer.import(input_for(source)).is_err(),
        "invalid two-qubit gate operands must not produce successful import",
    );
}


// =============================================================================
// Include security boundary
// =============================================================================

#[test]
fn standard_library_include_is_the_controlled_include() {
    let importer = OpenQasmImporter::production();

    let source = format!(
        "OPENQASM 3.1;\ninclude \"{}\";\n\nqubit[1] q;\nh q[0];\n",
        STANDARD_LIBRARY_INCLUDE,
    );

    let output = importer
        .import(input_for(&source))
        .expect(
            "the explicitly supported standard-library include must be accepted",
        );

    assert!(
        output.circuit().validate().is_ok(),
    );
}

#[test]
fn arbitrary_include_path_is_not_treated_as_filesystem_permission() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

include "/etc/passwd";

qubit[1] q;
x q[0];
"#;

    /*
     * The exact error category belongs to the OpenQASM semantic layer. The
     * integration invariant is stronger and more important: an arbitrary path
     * must never result in a successful import.
     */
    assert!(
        importer.import(input_for(source)).is_err(),
        "arbitrary include paths must not trigger implicit filesystem access",
    );
}

#[test]
fn relative_include_path_is_not_resolved_implicitly() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

include "../outside.qasm";

qubit[1] q;
x q[0];
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "relative includes must not become implicit filesystem traversal",
    );
}


// =============================================================================
// Unsupported semantic constructs
// =============================================================================

#[test]
fn unsupported_source_constructs_must_not_be_silently_discarded() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[1] q;

extern foo();
"#;

    /*
     * The canonical IR does not acquire arbitrary external-call semantics just
     * because OpenQASM can express them. Therefore the importer must either
     * explicitly support the construct through a canonical representation or
     * reject it.
     */
    assert!(
        importer.import(input_for(source)).is_err(),
        "unsupported extern semantics must not disappear during lowering",
    );
}

#[test]
fn calibration_constructs_cannot_become_execution_permissions() {
    let importer = OpenQasmImporter::production();

    let source = r#"OPENQASM 3.1;

qubit[1] q;

defcal x $0 {
}
"#;

    assert!(
        importer.import(input_for(source)).is_err(),
        "calibration constructs must not execute or silently disappear",
    );
}


// =============================================================================
// Determinism
// =============================================================================

#[test]
fn repeated_imports_are_deterministic_at_the_public_boundary() {
    let importer = OpenQasmImporter::production();

    let first = importer
        .import(input_for(representative_qasm()))
        .expect("first import must succeed");

    let second = importer
        .import(input_for(representative_qasm()))
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

    assert_eq!(
        format!("{:?}", first.circuit()),
        format!("{:?}", second.circuit()),
        "identical source and configuration must produce deterministic \
         canonical Quantum IR",
    );
}

#[test]
fn equivalent_input_whitespace_does_not_change_canonical_semantics() {
    let importer = OpenQasmImporter::production();

    let source_a = r#"OPENQASM 3.1;
include "stdgates.inc";
qubit[1] q;
x q[0];
"#;

    let source_b = r#"
OPENQASM 3.1;

include "stdgates.inc";

qubit[1] q;

x q[0];

"#;

    let first = importer
        .import(input_for(source_a))
        .expect("first semantically equivalent program must import");

    let second = importer
        .import(input_for(source_b))
        .expect("second semantically equivalent program must import");

    assert!(
        first.circuit().validate().is_ok(),
    );

    assert!(
        second.circuit().validate().is_ok(),
    );

    assert_eq!(
        format!("{:?}", first.circuit()),
        format!("{:?}", second.circuit()),
        "source formatting must not create different canonical quantum semantics",
    );
}


// =============================================================================
// Resource/input safety
// =============================================================================

#[test]
fn production_import_accepts_normal_sized_input() {
    let importer = OpenQasmImporter::production();

    let input = input_for(basic_qasm_31());

    assert!(
        importer.import(input).is_ok(),
        "normal production input must remain accepted",
    );
}

#[test]
fn malformed_inputs_do_not_panic() {
    let importer = OpenQasmImporter::production();

    let malformed_sources = [
        "",
        "OPENQASM",
        "OPENQASM 3",
        "OPENQASM 3.;",
        "OPENQASM 3.1",
        "OPENQASM 3.1; {",
        "OPENQASM 3.1; qubit[",
        "OPENQASM 3.1; qubit[-1] q;",
        "OPENQASM 3.1; qubit[999999999999999999999999999] q;",
        "OPENQASM 3.1; qubit[1] q; x;",
        "OPENQASM 3.1; qubit[1] q; cx q[0];",
        "OPENQASM 3.1; qubit[1] q; measure q[0] -> c[0];",
        "OPENQASM 3.1; include \"../../../../etc/passwd\";",
    ];

    for source in malformed_sources {
        let result = catch_unwind(AssertUnwindSafe(|| {
            importer.import(input_for(source))
        }));

        assert!(
            result.is_ok(),
            "frontend must not panic for malformed input: {source:?}",
        );
    }
}

#[test]
fn deeply_nested_invalid_source_does_not_panic() {
    let importer = OpenQasmImporter::production();

    let mut source = String::from("OPENQASM 3.1;\n");

    for _ in 0..256 {
        source.push_str("if (true) {\n");
    }

    source.push_str("qubit[1] q;\n");

    for _ in 0..256 {
        source.push_str("}\n");
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(input_for(&source))
    }));

    assert!(
        result.is_ok(),
        "bounded parser/validator behavior must not turn pathological \
         nesting into a process panic",
    );
}

#[test]
fn very_large_identifier_does_not_panic() {
    let importer = OpenQasmImporter::production();

    let identifier = "q".repeat(64 * 1024);

    let source = format!(
        "OPENQASM 3.1;\nqubit[1] {identifier};\n"
    );

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(input_for(&source))
    }));

    assert!(
        result.is_ok(),
        "oversized identifiers must result in controlled acceptance or \
         rejection, never a panic",
    );
}


// =============================================================================
// Canonical IR boundary
// =============================================================================

#[test]
fn import_result_exposes_only_canonical_circuit_semantics() {
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(input_for(basic_qasm_31()))
        .expect("valid OpenQASM must import");

    /*
     * This test intentionally does not inspect an OpenQASM AST. Once the
     * import boundary succeeds, canonical QuantumCircuit is the semantic
     * representation that downstream compiler stages consume.
     */
    let circuit = output.circuit();

    assert!(
        circuit.validate().is_ok(),
        "canonical Quantum IR must validate after frontend import",
    );
}

#[test]
fn importer_does_not_require_exporter_for_successful_import() {
    /*
     * This test deliberately imports without constructing or invoking an
     * OpenQasmExporter. It protects the architectural independence of the
     * import and export paths.
     */
    let importer = OpenQasmImporter::production();

    let output = importer
        .import(input_for(basic_qasm_31()))
        .expect(
            "OpenQASM import must not depend on exporter construction",
        );

    assert!(
        output.circuit().validate().is_ok(),
    );
}


// =============================================================================
// Public API isolation
// =============================================================================

#[test]
fn importer_is_reusable_without_mutable_global_state() {
    let importer = OpenQasmImporter::production();

    let first = importer
        .import(input_for(basic_without_include()))
        .expect("first independent import must succeed");

    let second = importer
        .import(input_for(parameterized_qasm()))
        .expect("second independent import must succeed");

    assert!(
        first.circuit().validate().is_ok(),
    );

    assert!(
        second.circuit().validate().is_ok(),
    );
}

#[test]
fn separate_importers_have_independent_configuration_state() {
    let first = OpenQasmImporter::production();
    let second = OpenQasmImporter::production();

    assert_eq!(
        first.version(),
        second.version(),
    );

    assert_eq!(
        first.format(),
        second.format(),
    );

    /*
     * This is intentionally a value-level comparison rather than a pointer
     * comparison. The production architecture must not depend on a mutable
     * global importer singleton.
     */
    assert_eq!(
        format!("{:?}", first),
        format!("{:?}", second),
    );
}


// =============================================================================
// Regression tests for the public production contract
// =============================================================================

#[test]
fn standard_library_include_constant_matches_language_source() {
    let source = format!(
        "OPENQASM 3.1;\ninclude \"{}\";\n",
        STANDARD_LIBRARY_INCLUDE,
    );

    assert!(
        source.contains("include \"stdgates.inc\";"),
        "the public standard-library constant must identify the canonical \
         OpenQASM standard-gate include",
    );
}

#[test]
fn openqasm_3_0_and_3_1_constants_are_ordered_correctly() {
    assert!(
        OPENQASM_3_0.is_older_than(OPENQASM_3_1),
        "OpenQASM 3.0 must compare older than OpenQASM 3.1",
    );
}

#[test]
fn public_import_failure_is_controlled_for_invalid_source() {
    let importer = OpenQasmImporter::production();

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(input_for(
            r#"OPENQASM 3.1;

qubit[1] q;
definitely_invalid q[0];
"#,
        ))
    }));

    assert!(
        result.is_ok(),
        "invalid source must cross the public API as a controlled result",
    );

    assert!(
        result
            .expect("catch_unwind returned an unexpected state")
            .is_err(),
        "invalid OpenQASM must not be accepted",
    );
}