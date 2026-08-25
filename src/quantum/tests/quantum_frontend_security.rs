//! Zamani Quantum Frontend — security-boundary integration tests.
//!
//! This module verifies the security properties of the complete public
//! quantum-frontend boundary without depending on OpenQASM implementation
//! internals.
//!
//! # Security boundary
//!
//! ```text
//!                         UNTRUSTED INPUT
//!                              │
//!                              ▼
//!                         ImportInput
//!                              │
//!                              ▼
//!                    ┌────────────────────┐
//!                    │ Generic Frontend  │
//!                    │                    │
//!                    │ source validation  │
//!                    │ limits             │
//!                    │ lexer              │
//!                    │ parser             │
//!                    │ validation         │
//!                    │ lowering           │
//!                    └─────────┬──────────┘
//!                              │
//!                              ▼
//!                         Quantum IR
//! ```
//!
//! This test module establishes that the boundary:
//!
//! - does not execute source-level constructs;
//! - does not perform implicit filesystem access;
//! - does not perform implicit network access;
//! - does not spawn processes;
//! - does not access quantum hardware;
//! - does not silently resolve arbitrary includes;
//! - does not silently discard unsupported semantics;
//! - does not panic on adversarial input;
//! - remains deterministic;
//! - respects the source/source-map identity contract;
//! - preserves the generic frontend error boundary;
//! - is safe to reuse from independent compilation requests;
//! - does not depend on global mutable frontend state;
//! - remains compatible with Rust 1.97 / 1.97.1.
//!
//! # Relationship with other frontend tests
//!
//! This file deliberately does **not** replace:
//!
//! - `limits.rs` — individual resource-limit contracts;
//! - `resource_exhaustion.rs` — exhaustive resource exhaustion coverage;
//! - `malformed_inputs.rs` — malformed syntax corpus;
//! - `openqasm_lexer.rs` — lexical conformance;
//! - `openqasm_parser.rs` — grammar conformance;
//! - `openqasm_validation.rs` — semantic correctness;
//! - `openqasm_import.rs` — normal import semantics;
//! - `openqasm_export.rs` — normal export semantics;
//! - `openqasm_roundtrip.rs` — semantic round-trip.
//!
//! Instead, this module verifies the **security properties that span those
//! layers**.
//!
//! # Public-API rule
//!
//! Cross-layer security tests use the public frontend boundary:
//!
//! ```text
//! crate::quantum::frontend
//! ```
//!
//! They intentionally do not depend on:
//!
//! ```text
//! formats::openqasm::lexer
//! formats::openqasm::parser
//! formats::openqasm::ast
//! formats::openqasm::validation
//! ```
//!
//! This is important because OpenQASM is an independently removable frontend
//! format. Security guarantees must belong to the frontend contract rather
//! than to accidental implementation details.
//!
//! # Side-effect rule
//!
//! This test module itself performs no filesystem, network, subprocess,
//! environment mutation, or hardware access.
//!
//! The tests verify side-effect freedom by supplying source constructs that
//! would be dangerous if interpreted as execution permissions.
//!
//! The expected result is either:
//!
//! ```text
//! supported → validated → lowered
//! ```
//!
//! or:
//!
//! ```text
//! unsupported → structured error
//! ```
//!
//! or:
//!
//! ```text
//! invalid → structured error
//! ```
//!
//! Never:
//!
//! ```text
//! source construct → implicit external side effect
//! ```
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
//! # Integration contract
//!
//! This file depends only on contracts that already exist before the security
//! layer:
//!
//! ```text
//! core/source.rs
//!       │
//!       ├───────────────┐
//!       │               │
//! core/limits.rs   core/errors.rs
//!       │               │
//!       └───────┬───────┘
//!               ▼
//!        generic importer
//!               │
//!               ▼
//!       OpenQasmImporter
//! ```
//!
//! It must not require a modification to the OpenQASM lexer/parser/AST merely
//! because this security suite exists.
//!
//! # Production completion criterion
//!
//! This file is complete when all tests below pass under:
//!
//! ```text
//! cargo test
//! cargo test --release
//! cargo clippy -- -D warnings
//! ```
//!
//! using Rust 1.97 / 1.97.1.
//!
//! The parent `tests/mod.rs` must register this module exactly once.

#![allow(clippy::module_name_repetitions)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;
use std::thread;

use crate::quantum::frontend::core::errors::{
    FrontendError,
    FrontendErrorKind,
};
use crate::quantum::frontend::core::limits::FrontendLimits;
use crate::quantum::frontend::core::source::SourceMap;
use crate::quantum::frontend::importer::{
    FormatImporter,
    ImportConfig,
    ImportInput,
    ImportOutput,
};
use crate::quantum::frontend::{
    OpenQasmImporter,
    OPENQASM_3_1,
};


// =============================================================================
// Test input construction
// =============================================================================

/// Creates a normal frontend input using the exact source-map contract used
/// by production callers.
///
/// The source bytes and the source-map text are intentionally constructed from
/// the same string. This prevents tests from bypassing the source identity
/// invariant.
fn input(source: &str) -> ImportInput {
    input_with_limits(source, FrontendLimits::strict())
}

/// Creates an input with explicit limits.
///
/// This helper is intentionally small. Resource-limit testing belongs to
/// `resource_exhaustion.rs`; this module only needs a bounded configuration so
/// security cases remain cheap and deterministic.
fn input_with_limits(
    source: &str,
    limits: FrontendLimits,
) -> ImportInput {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("security-test.qasm"),
            Arc::<str>::from(source),
        )
        .expect(
            "security-test source must fit the frontend SourceMap model",
        );

    ImportInput::new(
        source_id,
        source.as_bytes().to_vec(),
        source_map,
        ImportConfig::new(limits),
    )
    .expect(
        "security-test input must satisfy the generic ImportInput contract",
    )
}


// =============================================================================
// Panic-free execution helpers
// =============================================================================

/// Runs one OpenQASM import while converting an unexpected panic into a test
/// failure.
///
/// The production security invariant is stronger than merely "most malformed
/// programs return errors": untrusted input must not escape the frontend
/// boundary by unwinding.
fn import_without_panic(
    source: &str,
) -> Result<ImportOutput, FrontendError> {
    import_without_panic_with_limits(
        source,
        FrontendLimits::strict(),
    )
}

/// Runs one OpenQASM import with explicit limits.
fn import_without_panic_with_limits(
    source: &str,
    limits: FrontendLimits,
) -> Result<ImportOutput, FrontendError> {
    let importer = OpenQasmImporter::production();
    let request = input_with_limits(source, limits);

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(request)
    }));

    match result {
        Ok(result) => result,

        Err(payload) => {
            panic!(
                "OpenQASM frontend panicked on security input; \
                 panic payload type: {}",
                panic_payload_type(&payload),
            );
        }
    }
}

/// Returns only the panic payload type.
///
/// The actual payload is intentionally not printed. Security tests should not
/// accidentally emit arbitrary attacker-controlled strings into CI logs.
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


// =============================================================================
// Stable result classification
// =============================================================================

/// Represents only the stable externally observable classification required by
/// the security suite.
///
/// Human-readable error messages are deliberately excluded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ErrorClassification {
    kind: FrontendErrorKind,
    code: &'static str,
}

/// Extracts the stable classification from a frontend error.
fn classify_error(
    error: &FrontendError,
) -> ErrorClassification {
    ErrorClassification {
        kind: error.kind(),
        code: error.code().as_str(),
    }
}

/// Runs the same security input twice and requires identical structured
/// failure classification.
fn assert_deterministic_rejection(
    source: &str,
) {
    let first = import_without_panic(source);
    let second = import_without_panic(source);

    let first = first.expect_err(
        "security input must not be accepted",
    );
    let second = second.expect_err(
        "security input must not be accepted on the second run",
    );

    assert_eq!(
        classify_error(&first),
        classify_error(&second),
        "security failure classification must be deterministic",
    );
}


// =============================================================================
// Public contract / type safety
// =============================================================================

/// Compile-time assertion that the production importer can be moved between
/// threads.
///
/// This matters because frontend instances must not depend on thread-local
/// mutable parser state or global execution state.
fn assert_send<T: Send>() {}

/// Compile-time assertion that the production importer can be shared between
/// threads.
///
/// This establishes the intended `Send + Sync` public contract without
/// requiring any unsafe implementation.
fn assert_sync<T: Sync>() {}

#[test]
fn production_importer_is_send_and_sync() {
    assert_send::<OpenQasmImporter>();
    assert_sync::<OpenQasmImporter>();
}


// =============================================================================
// Source/source-map security boundary
// =============================================================================

#[test]
fn source_map_mismatch_is_rejected_before_parsing() {
    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("registered.qasm"),
            Arc::<str>::from(
                "OPENQASM 3.1;\nqubit[1] q;",
            ),
        )
        .expect("test source must fit SourceMap");

    let result = ImportInput::new(
        source_id,
        b"OPENQASM 3.1;\nqubit[2] q;".to_vec(),
        source_map,
        ImportConfig::new(FrontendLimits::strict()),
    );

    let error = result.expect_err(
        "mismatched source bytes and source-map text must be rejected",
    );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::InvalidInput,
        "source-map mismatch must be an invalid-input failure",
    );
}

#[test]
fn unknown_source_id_is_rejected_before_parsing() {
    let source_map = SourceMap::new();

    let result = ImportInput::new(
        crate::quantum::frontend::core::source::SourceId::from_raw(
            u32::MAX,
        ),
        b"OPENQASM 3.1;".to_vec(),
        source_map,
        ImportConfig::new(FrontendLimits::strict()),
    );

    let error = result.expect_err(
        "unknown source IDs must not cross the import boundary",
    );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::InvalidInput,
        "unknown source IDs must be invalid input",
    );
}

#[test]
fn malicious_source_display_name_does_not_execute_or_resolve_paths() {
    let malicious_name =
        "../../../../../definitely-not-a-real-file.qasm";

    let source =
        "OPENQASM 3.1;\nqubit[1] q;\n";

    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from(malicious_name),
            Arc::<str>::from(source),
        )
        .expect("test source must fit SourceMap");

    let request = ImportInput::new(
        source_id,
        source.as_bytes().to_vec(),
        source_map,
        ImportConfig::new(FrontendLimits::strict()),
    )
    .expect("source-name security test input must be valid");

    let importer = OpenQasmImporter::production();

    let result = catch_unwind(AssertUnwindSafe(|| {
        importer.import(request)
    }));

    assert!(
        result.is_ok(),
        "a malicious display name must not cause frontend unwinding",
    );
}


// =============================================================================
// Dangerous source-language constructs
// =============================================================================

#[test]
fn arbitrary_include_is_not_an_implicit_filesystem_permission() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "include \"../../../../etc/passwd\";\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "arbitrary include paths must not become implicit filesystem access",
    );
}

#[test]
fn absolute_include_is_not_an_implicit_filesystem_permission() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "include \"/etc/passwd\";\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "absolute include paths must not be resolved implicitly",
    );
}

#[test]
fn nonexistent_include_does_not_trigger_external_resolution() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "include \"__zamani_security_test_nonexistent__.inc\";\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "unknown includes must not be treated as executable external resources",
    );
}

#[test]
fn extern_declaration_does_not_execute_external_code() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "extern \"sh -c 'touch /tmp/zamani-security-test'\";\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "extern-like execution payloads must never be executed by import",
    );
}

#[test]
fn calibration_like_input_does_not_execute_calibration_code() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "defcal dangerous() {\n",
        "    // security regression payload\n",
        "}\n",
    );

    let result = import_without_panic(source);

    /*
     * Whether this exact construct is syntactically accepted is a format
     * support question. What is not negotiable is that importing it must never
     * execute calibration content.
     *
     * If the implementation supports this construct in a future version, the
     * test must be changed to assert the resulting representation is inert
     * data rather than execution. For the current canonical IR boundary,
     * unsupported calibration constructs must be rejected.
     */
    assert!(
        result.is_err(),
        "unsupported calibration content must not cross the current IR boundary",
    );
}

#[test]
fn pragma_like_input_is_not_execution_permission() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "#pragma exec \"sh -c 'id'\"\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "pragma execution payloads must not execute during import",
    );
}

#[test]
fn annotation_like_input_is_data_not_execution_permission() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "@exec(\"sh -c 'id'\")\n",
        "qubit[1] q;\n",
    );

    let result = import_without_panic(source);

    /*
     * The exact annotation support policy belongs to OpenQASM validation.
     * The security invariant is that annotations cannot cause process,
     * filesystem, network, or hardware effects.
     */
    if let Ok(output) = result {
        /*
         * Successful parsing is acceptable only if the construct remains
         * inert and the canonical circuit remains valid. The test deliberately
         * does not inspect implementation-private AST state.
         */
        assert!(
            output.circuit().validate().is_ok(),
            "successful annotation handling must still produce valid canonical IR",
        );
    }
}


// =============================================================================
// No command execution through source data
// =============================================================================

#[test]
fn command_in_identifier_is_treated_as_source_data() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "qubit[1] `sh -c 'id'`;\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "command-like source must not be executed as an external process",
    );
}

#[test]
fn shell_metacharacters_do_not_cross_the_frontend_boundary() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "qubit[1] q;\n",
        "x q[0];\n",
        "// ; && || $() `rm -rf /` > /tmp/x\n",
    );

    let result = import_without_panic(source);

    /*
     * The source is valid apart from whatever lexical/comment policy the
     * current implementation applies. Either outcome is safe because the
     * shell-looking content occurs only in source data.
     */
    if let Ok(output) = result {
        assert!(
            output.circuit().validate().is_ok(),
            "successful import must produce valid canonical IR",
        );
    }
}


// =============================================================================
// No implicit network permission
// =============================================================================

#[test]
fn network_url_in_include_is_not_resolved() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "include \"https://example.invalid/malicious.inc\";\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "URL-like includes must not trigger network access",
    );
}

#[test]
fn network_command_in_source_is_not_executed() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "include \"tcp://127.0.0.1:1/payload\";\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "network-like source references must not trigger network access",
    );
}


// =============================================================================
// No hardware/QPU execution
// =============================================================================

#[test]
fn hardware_like_source_does_not_execute_a_qpu() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "qubit[1] q;\n",
        "x q[0];\n",
        "// execute_on_qpu\n",
        "// backend=real_hardware\n",
    );

    let result = import_without_panic(source);

    if let Ok(output) = result {
        assert!(
            output.circuit().validate().is_ok(),
            "successful frontend import must remain a valid canonical circuit",
        );
    }
}

#[test]
fn backend_name_in_source_does_not_select_hardware() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "qubit[1] q;\n",
        "x q[0];\n",
        "// backend=real_qpu\n",
        "// device=production_hardware\n",
    );

    let result = import_without_panic(source);

    if let Ok(output) = result {
        assert!(
            output.circuit().validate().is_ok(),
            "frontend import must remain hardware-independent",
        );
    }
}


// =============================================================================
// Panic resistance against security-oriented adversarial input
// =============================================================================

#[test]
fn deeply_nested_delimiters_do_not_panic() {
    let mut source = String::from("OPENQASM 3.1;\n");

    for _ in 0..512 {
        source.push('(');
    }

    source.push('1');

    for _ in 0..512 {
        source.push(')');
    }

    source.push(';');

    let result = import_without_panic(&source);

    assert!(
        result.is_err(),
        "deeply nested malformed input must be rejected",
    );
}

#[test]
fn unterminated_string_does_not_panic() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "include \"../../../../etc/passwd\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "unterminated strings must be rejected",
    );
}

#[test]
fn unterminated_comment_does_not_panic() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "/*",
        "A".repeat(4_096).as_str(),
    );

    let result = import_without_panic(&source);

    assert!(
        result.is_err(),
        "unterminated comments must not escape as panics",
    );
}

#[test]
fn malformed_numeric_payload_does_not_panic() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "qubit[1] q;\n",
        "rx(999999999999999999999999999999999999999999999999999999999999999999)",
        " q[0];\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "pathological numeric literals must be rejected or classified as unsupported",
    );
}

#[test]
fn huge_identifier_payload_is_rejected_without_panic() {
    let identifier = "a".repeat(32_768);

    let source = format!(
        "OPENQASM 3.1;\nqubit[1] {identifier};\n"
    );

    let result = import_without_panic(&source);

    assert!(
        result.is_err(),
        "oversized identifiers must not cross the security boundary",
    );
}

#[test]
fn malformed_security_corpus_is_panic_free() {
    let corpus = [
        "",
        "OPENQASM",
        "OPENQASM 3",
        "OPENQASM 3.1",
        "OPENQASM 3.1;",
        "OPENQASM 3.1;\nqubit",
        "OPENQASM 3.1;\nqubit[",
        "OPENQASM 3.1;\nqubit[-1] q;",
        "OPENQASM 3.1;\nqubit[999999999999999999999999] q;",
        "OPENQASM 3.1;\ninclude \"../../../../etc/passwd\";",
        "OPENQASM 3.1;\ninclude \"https://example.invalid/x\";",
        "OPENQASM 3.1;\nextern \"sh -c id\";",
        "OPENQASM 3.1;\n#pragma exec \"id\"",
        "OPENQASM 3.1;\nqubit[1] q;\nmeasure;",
        "OPENQASM 3.1;\nqubit[1] q;\ninvalid_operation q[0];",
        "OPENQASM 3.1;\nqubit[1] q;\nrx(,) q[0];",
        "OPENQASM 3.1;\nqubit[1] q;\nrx(",
    ];

    for source in corpus {
        let result = import_without_panic(source);

        /*
         * The important property here is absence of panic. Some malformed
         * forms may be classified at different frontend layers depending on
         * the exact grammar production.
         */
        if let Ok(output) = result {
            assert!(
                output.circuit().validate().is_ok(),
                "successful import must always return valid canonical IR",
            );
        }
    }
}


// =============================================================================
// Determinism of security failures
// =============================================================================

#[test]
fn arbitrary_include_failure_is_deterministic() {
    assert_deterministic_rejection(
        concat!(
            "OPENQASM 3.1;\n",
            "include \"../../../../etc/passwd\";\n",
        ),
    );
}

#[test]
fn nonexistent_include_failure_is_deterministic() {
    assert_deterministic_rejection(
        concat!(
            "OPENQASM 3.1;\n",
            "include \"__zamani_security_nonexistent__.inc\";\n",
        ),
    );
}

#[test]
fn malformed_execution_payload_failure_is_deterministic() {
    assert_deterministic_rejection(
        concat!(
            "OPENQASM 3.1;\n",
            "extern \"sh -c 'id'\";\n",
        ),
    );
}

#[test]
fn malformed_syntax_failure_is_deterministic() {
    assert_deterministic_rejection(
        "OPENQASM 3.1;\nqubit[;",
    );
}


// =============================================================================
// Successful-input determinism
// =============================================================================

#[test]
fn benign_import_is_deterministic() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "qubit[2] q;\n",
        "x q[0];\n",
        "h q[1];\n",
        "cx q[0], q[1];\n",
    );

    let first = import_without_panic(source)
        .expect("benign OpenQASM must import successfully");

    let second = import_without_panic(source)
        .expect("same benign OpenQASM must import successfully");

    assert_eq!(
        first.circuit(),
        second.circuit(),
        "same source/configuration must produce identical canonical IR",
    );

    assert_eq!(
        first.format(),
        second.format(),
        "format identity must be deterministic",
    );

    assert_eq!(
        first.version(),
        second.version(),
        "format version must be deterministic",
    );
}


// =============================================================================
// Importer reuse / reentrancy
// =============================================================================

#[test]
fn one_importer_can_process_independent_requests_without_state_leakage() {
    let importer = OpenQasmImporter::production();

    let first = importer
        .import(input(
            concat!(
                "OPENQASM 3.1;\n",
                "qubit[1] a;\n",
                "x a[0];\n",
            ),
        ))
        .expect("first import must succeed");

    let second = importer
        .import(input(
            concat!(
                "OPENQASM 3.1;\n",
                "qubit[2] b;\n",
                "h b[0];\n",
                "cx b[0], b[1];\n",
            ),
        ))
        .expect("second import must succeed");

    assert_ne!(
        first.circuit(),
        second.circuit(),
        "independent requests must not leak circuit state",
    );
}

#[test]
fn failed_import_does_not_poison_subsequent_imports() {
    let importer = OpenQasmImporter::production();

    let failed = importer.import(input(
        concat!(
            "OPENQASM 3.1;\n",
            "include \"../../../../etc/passwd\";\n",
        ),
    ));

    assert!(
        failed.is_err(),
        "security payload must be rejected",
    );

    let successful = importer.import(input(
        concat!(
            "OPENQASM 3.1;\n",
            "qubit[1] q;\n",
            "x q[0];\n",
        ),
    ));

    assert!(
        successful.is_ok(),
        "a failed request must not poison a reusable importer",
    );
}


// =============================================================================
// Concurrent independent requests
// =============================================================================

#[test]
fn concurrent_imports_are_independent_and_panic_free() {
    let importer = Arc::new(OpenQasmImporter::production());

    let sources = [
        concat!(
            "OPENQASM 3.1;\n",
            "qubit[1] q;\n",
            "x q[0];\n",
        ),
        concat!(
            "OPENQASM 3.1;\n",
            "qubit[2] q;\n",
            "h q[0];\n",
            "cx q[0], q[1];\n",
        ),
        concat!(
            "OPENQASM 3.1;\n",
            "qubit[1] q;\n",
            "include \"../../../../etc/passwd\";\n",
        ),
        concat!(
            "OPENQASM 3.1;\n",
            "qubit[1] q;\n",
            "rx(,) q[0];\n",
        ),
    ];

    let handles = sources
        .into_iter()
        .map(|source| {
            let importer = Arc::clone(&importer);

            thread::spawn(move || {
                let result = catch_unwind(AssertUnwindSafe(|| {
                    importer.import(input(source))
                }));

                match result {
                    Ok(result) => result,

                    Err(payload) => {
                        panic!(
                            "concurrent OpenQASM import panicked; \
                             payload type: {}",
                            panic_payload_type(&payload),
                        );
                    }
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        let result = handle
            .join()
            .expect("frontend worker thread must not panic");

        if let Ok(output) = result {
            assert!(
                output.circuit().validate().is_ok(),
                "successful concurrent import must return valid canonical IR",
            );
        }
    }
}


// =============================================================================
// No silent semantic loss at the security boundary
// =============================================================================

#[test]
fn unsupported_external_reference_does_not_become_successful_empty_circuit() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "include \"../../../../definitely-unsupported.inc\";\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "unsupported external source references must not be silently discarded",
    );
}

#[test]
fn invalid_operation_does_not_become_successful_import() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "qubit[1] q;\n",
        "totally_unknown_gate q[0];\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "unknown operations must not silently disappear",
    );
}

#[test]
fn invalid_measurement_does_not_become_successful_import() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "qubit[1] q;\n",
        "bit[1] c;\n",
        "measure q[0] -> c[99];\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "invalid measurement destinations must not be silently repaired",
    );
}


// =============================================================================
// Diagnostic/error boundary
// =============================================================================

#[test]
fn malformed_security_input_uses_structured_frontend_error() {
    let result = import_without_panic(
        "OPENQASM 3.1;\nqubit[;",
    );

    let error = result.expect_err(
        "malformed input must produce a structured error",
    );

    assert!(
        !error.code().as_str().is_empty(),
        "frontend errors must expose stable machine-readable codes",
    );

    assert!(
        error.code().is_well_formed(),
        "frontend error codes must use the stable machine-readable syntax",
    );
}

#[test]
fn security_failure_kind_is_not_derived_from_message_text() {
    let result = import_without_panic(
        "OPENQASM 3.1;\ninclude \"../../../../etc/passwd\";",
    );

    let error = result.expect_err(
        "arbitrary include must be rejected",
    );

    let kind = error.kind();

    assert!(
        matches!(
            kind,
            FrontendErrorKind::Unsupported
                | FrontendErrorKind::Syntax
                | FrontendErrorKind::Semantic
                | FrontendErrorKind::Import
                | FrontendErrorKind::InvalidInput
        ),
        "arbitrary include must remain within the structured frontend failure model",
    );

    /*
     * Deliberately do not inspect `error.to_string()` here. Consumers must use
     * `kind()` and `code()`, never parse human-readable messages.
     */
}


// =============================================================================
// Strict resource boundary smoke tests
// =============================================================================

#[test]
fn strict_source_limit_rejects_before_frontend_parsing() {
    let limits = FrontendLimits::builder()
        .max_source_bytes(16)
        .build()
        .expect("strict test limits must be valid");

    let source = "OPENQASM 3.1;";

    assert!(
        source.len() <= 16,
        "test precondition: source must be within the chosen source limit",
    );

    let oversized = format!(
        "{source}{}",
        "x".repeat(32),
    );

    let mut source_map = SourceMap::new();

    let source_id = source_map
        .add(
            Arc::<str>::from("oversized.qasm"),
            Arc::<str>::from(&oversized),
        )
        .expect("test source must fit SourceMap");

    let result = ImportInput::new(
        source_id,
        oversized.as_bytes().to_vec(),
        source_map,
        ImportConfig::new(limits),
    );

    let error = result.expect_err(
        "oversized source must be rejected before parsing",
    );

    assert_eq!(
        error.kind(),
        FrontendErrorKind::LimitExceeded,
        "source-size exhaustion must use the structured limit-exceeded kind",
    );
}


// =============================================================================
// Source text containing Unicode and adversarial byte patterns
// =============================================================================

#[test]
fn unicode_source_does_not_break_security_boundary() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "// Unicode security corpus: ",
        "λ 漢字 العربية বাংলা हिन्दी 😀\n",
        "qubit[1] q;\n",
        "x q[0];\n",
    );

    let result = import_without_panic(source);

    if let Ok(output) = result {
        assert!(
            output.circuit().validate().is_ok(),
            "Unicode source must not weaken canonical IR validation",
        );
    }
}

#[test]
fn control_characters_do_not_panic() {
    let source = concat!(
        "OPENQASM 3.1;\n",
        "qubit[1] q;\n",
        "\0\1\2\3\4\5\6\7",
        "x q[0];\n",
    );

    let result = import_without_panic(source);

    assert!(
        result.is_err(),
        "control-character source must be rejected or classified safely",
    );
}

#[test]
fn unusual_line_endings_do_not_cross_security_boundary() {
    let source = concat!(
        "OPENQASM 3.1;\r",
        "qubit[1] q;\r",
        "x q[0];\r",
    );

    let result = import_without_panic(source);

    if let Ok(output) = result {
        assert!(
            output.circuit().validate().is_ok(),
            "accepted unusual line endings must still produce valid IR",
        );
    }
}


// =============================================================================
// No state leakage between source identities
// =============================================================================

#[test]
fn distinct_source_ids_do_not_share_import_state() {
    let importer = OpenQasmImporter::production();

    let first_source =
        "OPENQASM 3.1;\nqubit[1] first;\nx first[0];\n";

    let second_source =
        "OPENQASM 3.1;\nqubit[1] second;\nh second[0];\n";

    let first = importer
        .import(input(first_source))
        .expect("first source must import");

    let second = importer
        .import(input(second_source))
        .expect("second source must import");

    assert_ne!(
        first.circuit(),
        second.circuit(),
        "source-local state must not leak between independent imports",
    );
}


// =============================================================================
// Security corpus determinism
// =============================================================================

#[test]
fn security_corpus_has_stable_classification() {
    let corpus = [
        concat!(
            "OPENQASM 3.1;\n",
            "include \"../../../../etc/passwd\";\n",
        ),
        concat!(
            "OPENQASM 3.1;\n",
            "include \"https://example.invalid/payload\";\n",
        ),
        concat!(
            "OPENQASM 3.1;\n",
            "qubit[;\n",
        ),
        concat!(
            "OPENQASM 3.1;\n",
            "qubit[1] q;\n",
            "totally_unknown_gate q[0];\n",
        ),
    ];

    for source in corpus {
        assert_deterministic_rejection(source);
    }
}


// =============================================================================
// Final invariant smoke test
// =============================================================================

#[test]
fn production_frontend_security_boundary_smoke_test() {
    let benign = concat!(
        "OPENQASM 3.1;\n",
        "qubit[2] q;\n",
        "h q[0];\n",
        "cx q[0], q[1];\n",
    );

    let imported = import_without_panic(benign)
        .expect(
            "benign OpenQASM must cross the security boundary successfully",
        );

    assert!(
        imported.circuit().validate().is_ok(),
        "successful import must always expose valid canonical Quantum IR",
    );

    let malicious = concat!(
        "OPENQASM 3.1;\n",
        "include \"../../../../etc/passwd\";\n",
    );

    let rejected = import_without_panic(malicious);

    assert!(
        rejected.is_err(),
        "dangerous external-source reference must not cross the security boundary",
    );
}