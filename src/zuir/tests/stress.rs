
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Scalability and Stress Tests
//!
//! Path: src/zuir/tests/stress.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later
//! Safety: unsafe Rust is forbidden.
//!
//! STATUS
//!
//! Proposed new test module. It is not integrated until the repository's
//! canonical ZUIR test-module owner registers it and the complete crate
//! compiles and passes the required test commands.
//!
//! PURPOSE
//!
//! Exercise the scalability and robustness of the existing ZUIR
//! serialization implementation using dynamically sized computational
//! payloads. Cover:
//!
//! - Growth across several payload sizes.
//! - Canonical serialization repeatability.
//! - Serialization and decoding round trips.
//! - Preservation of operation order and payload content.
//! - Incremental writing to a destination.
//! - Explicit output-budget failures.
//! - An opt-in, environment-configurable larger workload.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/serialization/canonical.rs
//! - src/zuir/serialization/schema.rs
//! - src/zuir/serialization/encode.rs
//! - src/zuir/serialization/decode.rs
//! - src/zuir/serialization/validation.rs
//! - src/zuir/tests/serialization.rs
//! - src/zuir/tests/property.rs
//! - src/zuir/tests/determinism.rs
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/antlr/ZamaniLexer.g4
//! - grammar/antlr/ZamaniParser.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//!
//! OWNS
//!
//! - Stress-test fixtures and workload construction.
//! - Regression assertions about the exercised workload.
//! - Test-only configuration for the optional large workload.
//!
//! DOES_NOT_OWN
//!
//! - The canonical serializer, document codec, or fingerprint algorithm.
//! - The ZUIR schema, type system, IR builder, or verifier.
//! - Source lexing, parsing, AST construction, or semantic analysis.
//! - Domain-specific semantics for quantum, classical, HDL, AI, or other
//!   dialects.
//! - Target discovery, hardware allocation, scheduling, or runtime execution.
//! - Production resource policies or universal language limits.
//!
//! DEPENDENCIES
//!
//! Uses the existing ZUIR serialization facade, canonical serialization
//! module, serde_json, and standard-library APIs. No additional crate is
//! required by this test file.
//!
//! GRAMMAR BOUNDARY
//!
//! The grammar defines source syntax. Semantic analysis defines meaning.
//! These tests use structured serialization payloads; they do not prove
//! that source text parses, that an IR is semantically valid, or that a
//! backend can execute a represented computation.
//!
//! SCALABILITY AND POCO-REAF
//!
//! Workload sizes are test-runner choices, not language-level limits.
//! No universal limit is introduced for operations, qubits, processors,
//! register width, tensor rank, memory, devices, nodes, or program size.
//!
//! The optional large workload can be configured with the
//! ZAMANI_ZUIR_STRESS_ITEMS environment variable. The value controls only
//! the number of synthetic operations in that test.
//!
//! No finite stress test proves literal infinite scalability. Actual
//! processing remains subject to available memory, execution time,
//! address-space limits, configured budgets, and implementation behavior.
//!
//! DETERMINISM
//!
//! The same payload and serialization options must produce the same
//! canonical bytes. Ordered operation sequences must retain their order.
//! These assertions do not establish semantic equivalence of arbitrary
//! programs or prove that fingerprints are collision-free.
//!
//! ERROR BEHAVIOR
//!
//! A configured output-budget failure is an expected, structured error.
//! It must not be interpreted as proof that the payload is semantically
//! invalid or that the language has a universal size ceiling.
//!
//! INTEGRATION
//!
//! 1. Confirm the actual src/zuir/mod.rs and test-module layout.
//! 2. Register this module exactly once in the existing test tree.
//! 3. Use the repository's existing serialization facade and canonical
//!    writer; do not implement a duplicate serializer here.
//! 4. Run normal tests before the optional large stress test.
//!
//! FREEZE CRITERIA
//!
//! - Imports match the actual integrated repository revision.
//! - The test module is registered exactly once.
//! - All ordinary stress tests pass on Rust 1.97 or later.
//! - The optional large workload passes at documented test-runner sizes.
//! - Formatting, linting, and repository-level tests pass.
//! - No unsafe code, hidden truncation, or hardware-size assumptions exist.

use std::io::{self, Write};

use serde_json::{json, Value};

use crate::zuir::serialization::{
    canonical_json_bytes,
    decode_document,
    encode_document,
    CanonicalOptions,
};

use crate::zuir::serialization::canonical::{
    write_canonical_json,
    CanonicalError,
};

// -----------------------------------------------------------------------------
// Test workload construction
// -----------------------------------------------------------------------------

/// Build a structured payload containing `count` synthetic operations.
///
/// The operation names and attributes are serialization fixtures, not a
/// closed list of operations supported by the ZUIR core.
///
/// `count` is supplied by the test. It is not a universal IR limit.
fn make_payload(count: usize) -> Result<Value, std::collections::TryReserveError> {
    let mut operations = Vec::new();
    operations.try_reserve_exact(count)?;

    for index in 0..count {
        operations.push(json!({
            "ordinal": index,
            "dialect": "extension.stress",
            "operation": "extension.stress.process",
            "operands": [
                format!("value::{index}"),
                format!("value::input::{index}")
            ],
            "results": [
                format!("value::output::{index}")
            ],
            "attributes": {
                "enabled": index % 2 == 0,
                "logical_index": index,
                "resource_requirement": {
                    "kind": "symbolic",
                    "expression": format!("resource::{index}")
                }
            }
        }));
    }

    Ok(json!({
        "schema": "zamani.zuir.stress-fixture",
        "schema_version": {
            "major": 1,
            "minor": 0,
            "patch": 0
        },
        "logical_program": {
            "target": null,
            "resource_requirements": {
                "memory": "symbolic:required_memory",
                "compute": "symbolic:required_compute"
            },
            "operations": operations
        },
        "metadata": {
            "purpose": "serialization scalability regression",
            "unicode": "Zamani — quantum λ — 界"
        }
    }))
}

/// Serialize a value using the production canonical JSON implementation.
///
/// This helper deliberately uses the canonical API rather than
/// `serde_json::to_vec`, because the tested contract is ZUIR's canonical
/// representation.
fn canonical_bytes(value: &Value) -> Result<Vec<u8>, CanonicalError> {
    canonical_json_bytes(value, CanonicalOptions::unbounded())
}

/// Encode and decode a payload using the production document codec.
fn document_round_trip(payload: &Value) -> Result<Value, String> {
    let encoded = encode_document(payload)
        .map_err(|error| format!("document encoding failed: {error}"))?;

    let decoded = decode_document(encoded.as_bytes())
        .map_err(|error| format!("document decoding failed: {error}"))?;

    Ok(decoded.payload().clone())
}

// -----------------------------------------------------------------------------
// Ordinary CI stress tests
// -----------------------------------------------------------------------------

#[test]
fn dynamically_sized_payloads_round_trip_at_increasing_workloads() {
    // These are representative CI workloads, not language or implementation
    // maximums. Larger workloads are available through the ignored test below.
    let workload_sizes: &[usize] = &[0, 1, 17, 257, 1024, 4096];

    for &count in workload_sizes {
        let payload = make_payload(count)
            .unwrap_or_else(|error| {
                panic!(
                    "could not reserve fixture storage for {count} operations: \
                     {error}"
                )
            });

        let decoded = document_round_trip(&payload)
            .unwrap_or_else(|error| {
                panic!(
                    "document round trip failed at {count} operations: {error}"
                )
            });

        assert_eq!(
            decoded, payload,
            "payload changed during round trip at {count} operations"
        );

        let operations = decoded["logical_program"]["operations"]
            .as_array()
            .expect("operation collection must remain an array");

        assert_eq!(
            operations.len(),
            count,
            "operation count changed at workload size {count}"
        );
    }
}

#[test]
fn canonical_bytes_are_repeatable_for_growing_payloads() {
    let workload_sizes: &[usize] = &[0, 1, 17, 257, 1024];

    for &count in workload_sizes {
        let payload = make_payload(count)
            .unwrap_or_else(|error| {
                panic!("fixture allocation failed at {count}: {error}")
            });

        let first = canonical_bytes(&payload)
            .unwrap_or_else(|error| {
                panic!("first canonical render failed at {count}: {error}")
            });

        let second = canonical_bytes(&payload)
            .unwrap_or_else(|error| {
                panic!("second canonical render failed at {count}: {error}")
            });

        assert_eq!(
            first, second,
            "canonical output changed at workload size {count}"
        );
    }
}

#[test]
fn document_encoding_is_deterministic_after_round_trip() {
    let payload = make_payload(1024)
        .expect("fixture allocation should succeed");

    let first = encode_document(&payload)
        .expect("initial document encoding should succeed");

    let decoded = decode_document(first.as_bytes())
        .expect("initial document decoding should succeed");

    let second = encode_document(decoded.payload())
        .expect("re-encoding decoded payload should succeed");

    assert_eq!(
        first.as_bytes(),
        second.as_bytes(),
        "encoding the same decoded payload must produce the same document"
    );
}

#[test]
fn canonical_writer_matches_the_canonical_byte_api() {
    let payload = make_payload(1024)
        .expect("fixture allocation should succeed");

    let expected = canonical_bytes(&payload)
        .expect("canonical byte API should succeed");

    let mut actual = Vec::new();

    let report = write_canonical_json(
        &payload,
        &mut actual,
        CanonicalOptions::unbounded(),
    )
    .expect("canonical writer should succeed");

    assert_eq!(
        actual, expected,
        "streaming and collecting APIs must agree"
    );

    assert_eq!(
        report.output_bytes,
        expected.len(),
        "reported byte count must match the emitted output"
    );

    assert!(
        report.nodes_visited > 0,
        "a non-empty JSON payload must visit at least its root node"
    );
}

#[test]
fn serialization_preserves_operation_sequence_and_content() {
    let payload = make_payload(257)
        .expect("fixture allocation should succeed");

    let decoded = document_round_trip(&payload)
        .expect("document round trip should succeed");

    let original = payload["logical_program"]["operations"]
        .as_array()
        .expect("original operations must be an array");

    let restored = decoded["logical_program"]["operations"]
        .as_array()
        .expect("restored operations must be an array");

    assert_eq!(original.len(), restored.len());

    for (index, (before, after)) in
        original.iter().zip(restored.iter()).enumerate()
    {
        assert_eq!(
            before, after,
            "operation content or order changed at index {index}"
        );
    }
}

#[test]
fn configured_output_budget_fails_without_becoming_a_semantic_verdict() {
    let payload = make_payload(32)
        .expect("fixture allocation should succeed");

    let restricted = CanonicalOptions::unbounded()
        .with_max_output_bytes(Some(1));

    let error = canonical_json_bytes(&payload, restricted)
        .expect_err("one output byte must not fit this payload");

    assert!(
        matches!(error, CanonicalError::OutputLimitExceeded { .. }),
        "expected a structured output-budget error, got {error}"
    );

    // The same payload must still serialize when the caller explicitly
    // chooses no additional canonical-serialization budget.
    canonical_json_bytes(&payload, CanonicalOptions::unbounded())
        .expect("budget exhaustion must not make the payload invalid");
}

// -----------------------------------------------------------------------------
// Writer failure propagation
// -----------------------------------------------------------------------------

/// A writer that accepts a finite number of bytes and then fails.
///
/// The allowance is a test fixture, not a format or hardware limit.
struct LimitedWriter {
    remaining: usize,
}

impl LimitedWriter {
    fn new(allowed_bytes: usize) -> Self {
        Self {
            remaining: allowed_bytes,
        }
    }
}

impl Write for LimitedWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "intentional stress-test writer failure",
            ));
        }

        let accepted = buffer.len().min(self.remaining);
        self.remaining -= accepted;

        Ok(accepted)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn canonical_writer_propagates_destination_failure() {
    let payload = make_payload(32)
        .expect("fixture allocation should succeed");

    let mut writer = LimitedWriter::new(1);

    let result = write_canonical_json(
        &payload,
        &mut writer,
        CanonicalOptions::unbounded(),
    );

    assert!(
        matches!(result, Err(CanonicalError::Io(_))),
        "writer failure must not be reported as successful serialization"
    );
}

// -----------------------------------------------------------------------------
// Opt-in larger workload
// -----------------------------------------------------------------------------

/// Run a larger stress workload on demand.
///
/// This test is ignored in ordinary CI because its workload is controlled by
/// the caller and can be substantially larger than the regular test corpus.
///
/// Environment variable:
///
///     ZAMANI_ZUIR_STRESS_ITEMS
///
/// Example:
///
///     ZAMANI_ZUIR_STRESS_ITEMS=100000 cargo test -- --ignored \
///         dynamically_sized_payloads_support_opt_in_large_stress
///
/// The default applies only to this optional test. No maximum is imposed by
/// this harness; choose a value appropriate for the available test runner.
/// Large values may require substantial memory and time.
#[test]
#[ignore = "opt-in workload; set ZAMANI_ZUIR_STRESS_ITEMS for the target runner"]
fn dynamically_sized_payloads_support_opt_in_large_stress() {
    let count = match std::env::var("ZAMANI_ZUIR_STRESS_ITEMS") {
        Ok(value) => value.parse::<usize>().unwrap_or_else(|error| {
            panic!(
                "ZAMANI_ZUIR_STRESS_ITEMS must be a valid usize, got \
                 {value:?}: {error}"
            )
        }),
        Err(std::env::VarError::NotPresent) => 100_000,
        Err(error) => {
            panic!("could not read ZAMANI_ZUIR_STRESS_ITEMS: {error}")
        }
    };

    let payload = make_payload(count)
        .unwrap_or_else(|error| {
            panic!(
                "could not reserve operation storage for {count} items: {error}"
            )
        });

    let encoded = encode_document(&payload)
        .unwrap_or_else(|error| {
            panic!("encoding failed for {count} operations: {error}")
        });

    let decoded = decode_document(encoded.as_bytes())
        .unwrap_or_else(|error| {
            panic!("decoding failed for {count} operations: {error}")
        });

    assert_eq!(
        decoded.payload(),
        &payload,
        "large payload changed during document round trip"
    );

    let first = canonical_bytes(&payload)
        .expect("first canonical rendering should succeed");

    let second = canonical_bytes(decoded.payload())
        .expect("second canonical rendering should succeed");

    assert_eq!(
        first, second,
        "large payload canonical output must be repeatable"
    );

    assert_eq!(
        decoded.payload()["logical_program"]["operations"]
            .as_array()
            .expect("operations must remain an array")
            .len(),
        count,
        "large workload lost operations"
    );
}
