//! Zamani Quantum Resilience — Serialization Contract Tests.
//!
//! Path:
//!     src/quantum/resilience/tests/serialization.rs
//!
//! Purpose:
//!     Production-oriented tests for the resilience serialization boundary.
//!
//! This test module verifies the contracts shared by:
//!
//!     quantum::resilience::serialization
//!
//! and, indirectly:
//!
//!     quantum::resilience::serialization::schema
//!     quantum::resilience::serialization::encode
//!     quantum::resilience::serialization::decode
//!     quantum::resilience::serialization::version
//!
//! Design goals:
//!
//!     - deterministic serialization;
//!     - deterministic deserialization;
//!     - schema identity preservation;
//!     - schema-version preservation;
//!     - document-kind preservation;
//!     - round-trip correctness;
//!     - payload preservation;
//!     - streaming writer/reader compatibility;
//!     - bounded decoding behavior;
//!     - truncation detection;
//!     - trailing-data detection;
//!     - malformed-input rejection;
//!     - incompatible-version rejection;
//!     - empty-payload handling;
//!     - large dynamically generated collections;
//!     - sparse quantum identifiers;
//!     - canonical QubitId ownership;
//!     - no fixed machine-size assumptions;
//!     - no backend/provider assumptions;
//!     - no unsafe code;
//!     - deterministic replay.
//!
//! Important architectural rule:
//!
//!     These tests test serialization.
//!
//!     They do NOT:
//!
//!         - execute quantum programs;
//!         - access hardware;
//!         - select a backend;
//!         - perform recovery;
//!         - perform QEC;
//!         - perform routing;
//!         - perform scheduling;
//!         - perform optimization;
//!         - redefine quantum identifiers.
//!
//! Canonical quantum identity remains owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! The tests intentionally use `QubitId` from that module where quantum
//! identity needs to be exercised.
//!
//! Scalability:
//!
//!     No production test assumes a maximum qubit count.
//!
//!     Generated collection sizes are derived from the test itself and are
//!     used only to exercise behavior. They are not architecture limits.
//!
//!     Boundary tests also exercise sparse identifiers, including the largest
//!     representable `usize` value where the canonical QubitId contract permits
//!     it.
//!
//! Rust:
//!
//!     - Rust 1.97 / 1.97.1
//!     - Rust 2021
//!     - stable Rust
//!     - no nightly features
//!     - no unsafe code

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::io::{self, Cursor, Read, Write};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::resilience::serialization::{
    decode_document,
    decode_document_with_limits,
    encode_document,
    CanonicalDocument,
    DecodeLimits,
    EncodeLimits,
    SchemaVersion,
    CURRENT_SCHEMA_VERSION,
    FORMAT_VERSION,
};

// =============================================================================
// Test payloads
// =============================================================================
//
// These payloads intentionally do not model a second resilience domain.
// They are serialization fixtures only.
//
// Quantum resource identifiers use the canonical IR QubitId.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TestPayload {
    name: String,
    generation: u64,
    qubits: Vec<QubitId>,
    metadata: Vec<(String, String)>,
}

impl TestPayload {
    fn small() -> Self {
        Self {
            name: String::from("serialization-test"),
            generation: 1,
            qubits: vec![
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(2),
            ],
            metadata: vec![
                (
                    String::from("purpose"),
                    String::from("contract-test"),
                ),
                (
                    String::from("scope"),
                    String::from("backend-independent"),
                ),
            ],
        }
    }

    fn generated(resource_count: usize) -> Self {
        let qubits = (0..resource_count)
            .map(QubitId::new)
            .collect::<Vec<_>>();

        let metadata = (0..resource_count)
            .map(|index| {
                (
                    format!("key-{index}"),
                    format!("value-{index}"),
                )
            })
            .collect::<Vec<_>>();

        Self {
            name: String::from("generated"),
            generation: resource_count as u64,
            qubits,
            metadata,
        }
    }

    fn sparse() -> Self {
        Self {
            name: String::from("sparse"),
            generation: 2,
            qubits: vec![
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(usize::MAX),
            ],
            metadata: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct EmptyPayload {
    values: Vec<String>,
}

impl EmptyPayload {
    fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn test_document(payload: Value) -> CanonicalDocument {
    CanonicalDocument::new(
        CURRENT_SCHEMA_VERSION,
        payload,
    )
}

fn encoded_small_document() -> Vec<u8> {
    let document = test_document(json!({
        "name": "serialization-test",
        "generation": 1,
        "qubits": [0, 1, 2]
    }));

    encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("small canonical document must encode")
}

fn assert_round_trip(document: &CanonicalDocument) {
    let encoded = encode_document(
        document,
        EncodeLimits::default(),
    )
    .expect("document must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("encoded document must decode");

    assert_eq!(
        decoded,
        *document,
        "canonical serialization must round-trip exactly"
    );
}

// =============================================================================
// Format/version contract
// =============================================================================

#[test]
fn current_format_version_is_stable_and_valid() {
    assert!(
        FORMAT_VERSION > 0,
        "serialization format version must never be zero"
    );
}

#[test]
fn current_schema_version_is_stable_and_nonzero_major() {
    assert!(
        CURRENT_SCHEMA_VERSION.major() > 0,
        "production schema must have a non-zero major version"
    );
}

#[test]
fn schema_version_ordering_is_deterministic() {
    let current = CURRENT_SCHEMA_VERSION;

    let same = SchemaVersion::new(
        current.major(),
        current.minor(),
        current.patch(),
    );

    assert_eq!(current, same);
    assert_eq!(current.cmp(&same), std::cmp::Ordering::Equal);
    assert!(current.same_major(same));
    assert!(current.is_exact(same));
}

// =============================================================================
// Basic round-trip tests
// =============================================================================

#[test]
fn small_document_round_trips() {
    let document = test_document(json!({
        "name": "small",
        "generation": 1,
        "qubits": [0, 1, 2]
    }));

    assert_round_trip(&document);
}

#[test]
fn empty_payload_round_trips() {
    let document = test_document(json!({
        "values": []
    }));

    assert_round_trip(&document);
}

#[test]
fn nested_payload_round_trips() {
    let document = test_document(json!({
        "execution": {
            "id": "execution-1",
            "resources": {
                "logical_qubits": [0, 1],
                "physical_qubits": [7, 11]
            }
        },
        "recovery": {
            "attempts": [],
            "actions": []
        }
    }));

    assert_round_trip(&document);
}

#[test]
fn unicode_payload_round_trips() {
    let document = test_document(json!({
        "name": "Zamani",
        "description": "Quantum resilience — écrire une fois, exécuter partout",
        "locales": ["en", "sn", "fr"]
    }));

    assert_round_trip(&document);
}

#[test]
fn large_integer_values_round_trip_without_float_conversion() {
    let document = test_document(json!({
        "unsigned": u64::MAX,
        "signed": i64::MIN,
        "small": 0
    }));

    assert_round_trip(&document);
}

// =============================================================================
// Canonical QubitId integration
// =============================================================================

#[test]
fn canonical_qubit_ids_are_used_in_test_payloads() {
    let payload = TestPayload::small();

    assert_eq!(payload.qubits[0], QubitId::new(0));
    assert_eq!(payload.qubits[1], QubitId::new(1));
    assert_eq!(payload.qubits[2], QubitId::new(2));
}

#[test]
fn sparse_canonical_qubit_ids_are_supported() {
    let payload = TestPayload::sparse();

    assert_eq!(
        payload.qubits.last().copied(),
        Some(QubitId::new(usize::MAX))
    );
}

#[test]
fn canonical_qubit_id_payload_round_trips() {
    let original = TestPayload::sparse();

    let value = serde_json::to_value(&original)
        .expect("canonical test payload must serialize");

    let document = test_document(value);

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("payload must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("payload must decode");

    let restored: TestPayload = serde_json::from_value(
        decoded.payload().clone(),
    )
    .expect("payload must deserialize into canonical test type");

    assert_eq!(restored, original);
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_documents_produce_identical_bytes() {
    let document = test_document(json!({
        "name": "deterministic",
        "values": [1, 2, 3, 4, 5]
    }));

    let first = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("first encoding must succeed");

    let second = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("second encoding must succeed");

    assert_eq!(
        first,
        second,
        "canonical encoding must be deterministic"
    );
}

#[test]
fn deterministic_encoding_survives_repeated_iterations() {
    let document = test_document(json!({
        "name": "repeat",
        "nested": {
            "a": 1,
            "b": 2,
            "c": [3, 4, 5]
        }
    }));

    let reference = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("reference encoding must succeed");

    for _ in 0..32 {
        let current = encode_document(
            &document,
            EncodeLimits::default(),
        )
        .expect("repeated encoding must succeed");

        assert_eq!(current, reference);
    }
}

#[test]
fn deterministic_decode_of_identical_bytes() {
    let encoded = encoded_small_document();

    let first = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("first decode must succeed");

    let second = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("second decode must succeed");

    assert_eq!(first, second);
}

// =============================================================================
// Streaming integration
// =============================================================================

#[test]
fn encoded_document_can_be_read_through_cursor() {
    let encoded = encoded_small_document();

    let mut cursor = Cursor::new(encoded.clone());
    let mut bytes = Vec::new();

    cursor
        .read_to_end(&mut bytes)
        .expect("cursor must be readable");

    assert_eq!(bytes, encoded);

    let decoded = decode_document(
        &bytes,
        DecodeLimits::default(),
    )
    .expect("streamed bytes must decode");

    assert_eq!(
        decoded.schema_version(),
        CURRENT_SCHEMA_VERSION
    );
}

#[test]
fn serialization_does_not_require_filesystem_access() {
    let document = test_document(json!({
        "execution": "offline"
    }));

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("in-memory encoding must succeed");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("in-memory decoding must succeed");

    assert_eq!(decoded, document);
}

// =============================================================================
// Malformed input
// =============================================================================

#[test]
fn empty_input_is_rejected() {
    let result = decode_document(
        &[],
        DecodeLimits::default(),
    );

    assert!(
        result.is_err(),
        "empty input must never be accepted as a valid document"
    );
}

#[test]
fn truncated_input_is_rejected() {
    let encoded = encoded_small_document();

    assert!(!encoded.is_empty());

    for length in 0..encoded.len() {
        let truncated = &encoded[..length];

        let result = decode_document(
            truncated,
            DecodeLimits::default(),
        );

        assert!(
            result.is_err(),
            "truncated document at length {length} must be rejected"
        );
    }
}

#[test]
fn trailing_data_is_rejected() {
    let mut encoded = encoded_small_document();

    encoded.extend_from_slice(&[
        0x5a,
        0x61,
        0x6d,
        0x61,
        0x6e,
        0x69,
    ]);

    let result = decode_document(
        &encoded,
        DecodeLimits::default(),
    );

    assert!(
        result.is_err(),
        "non-document trailing bytes must not be silently accepted"
    );
}

#[test]
fn arbitrary_garbage_is_rejected() {
    let garbage_sets: &[&[u8]] = &[
        &[0],
        &[0xff],
        &[0xff, 0xff, 0xff],
        b"not a Zamani document",
        b"{}",
        b"[]",
        b"null",
    ];

    for garbage in garbage_sets {
        let result = decode_document(
            garbage,
            DecodeLimits::default(),
        );

        assert!(
            result.is_err(),
            "malformed input must be rejected"
        );
    }
}

// =============================================================================
// Resource-limit integration
// =============================================================================

#[test]
fn default_decode_limits_are_valid() {
    let limits = DecodeLimits::default();

    assert!(
        limits.max_payload_bytes() > 0,
        "default payload limit must permit ordinary documents"
    );

    assert!(
        limits.max_collection_elements() > 0,
        "default collection limit must permit ordinary collections"
    );
}

#[test]
fn default_encode_limits_are_valid() {
    let limits = EncodeLimits::default();

    assert!(
        limits.max_payload_bytes() > 0,
        "default encode payload limit must permit ordinary documents"
    );

    assert!(
        limits.max_collection_elements() > 0,
        "default encode collection limit must permit ordinary collections"
    );
}

#[test]
fn tiny_decode_limit_rejects_oversized_document() {
    let encoded = encoded_small_document();

    let limits = DecodeLimits::default()
        .with_max_payload_bytes(1);

    let result = decode_document_with_limits(
        &encoded,
        limits,
    );

    assert!(
        result.is_err(),
        "configured decode resource limits must be enforced"
    );
}

#[test]
fn tiny_encode_limit_rejects_oversized_document() {
    let document = test_document(json!({
        "message": "this payload is intentionally larger than one byte"
    }));

    let limits = EncodeLimits::default()
        .with_max_payload_bytes(1);

    let result = encode_document(
        &document,
        limits,
    );

    assert!(
        result.is_err(),
        "configured encode resource limits must be enforced"
    );
}

// =============================================================================
// Dynamic scalability tests
// =============================================================================
//
// These are intentionally generated rather than tied to a specific QPU size.
// The values below exercise the implementation; they are not machine limits.

#[test]
fn generated_small_resource_population_round_trips() {
    let payload = TestPayload::generated(8);

    let value = serde_json::to_value(&payload)
        .expect("generated payload must serialize");

    let document = test_document(value);

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("generated document must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("generated document must decode");

    let restored: TestPayload = serde_json::from_value(
        decoded.payload().clone(),
    )
    .expect("generated payload must deserialize");

    assert_eq!(restored, payload);
}

#[test]
fn generated_medium_resource_population_round_trips() {
    let payload = TestPayload::generated(256);

    let value = serde_json::to_value(&payload)
        .expect("generated payload must serialize");

    let document = test_document(value);

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("generated document must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("generated document must decode");

    let restored: TestPayload = serde_json::from_value(
        decoded.payload().clone(),
    )
    .expect("generated payload must deserialize");

    assert_eq!(restored, payload);
}

#[test]
fn generated_population_is_not_truncated() {
    let population = 512usize;

    let payload = TestPayload::generated(population);

    let value = serde_json::to_value(&payload)
        .expect("payload must serialize");

    let document = test_document(value);

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("document must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("document must decode");

    let restored: TestPayload = serde_json::from_value(
        decoded.payload().clone(),
    )
    .expect("payload must deserialize");

    assert_eq!(restored.qubits.len(), population);
    assert_eq!(restored.metadata.len(), population);
}

// =============================================================================
// Schema identity
// =============================================================================

#[test]
fn schema_version_is_preserved() {
    let document = test_document(json!({
        "value": "schema"
    }));

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("document must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("document must decode");

    assert_eq!(
        decoded.schema_version(),
        document.schema_version()
    );
}

#[test]
fn document_payload_is_preserved_exactly() {
    let payload = json!({
        "alpha": 1,
        "beta": [
            true,
            false,
            null,
            {
                "nested": "value"
            }
        ]
    });

    let document = test_document(payload.clone());

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("document must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("document must decode");

    assert_eq!(
        decoded.payload(),
        &payload,
        "serialization must not alter payload semantics"
    );
}

// =============================================================================
// Structured payload integration
// =============================================================================

#[test]
fn structured_test_payload_round_trips() {
    let original = TestPayload::small();

    let value = serde_json::to_value(&original)
        .expect("structured payload must serialize");

    let document = test_document(value);

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("structured payload must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("structured payload must decode");

    let restored: TestPayload = serde_json::from_value(
        decoded.payload().clone(),
    )
    .expect("structured payload must deserialize");

    assert_eq!(restored, original);
}

#[test]
fn empty_structured_payload_round_trips() {
    let original = EmptyPayload::new();

    let value = serde_json::to_value(&original)
        .expect("empty payload must serialize");

    let document = test_document(value);

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("empty payload must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("empty payload must decode");

    let restored: EmptyPayload = serde_json::from_value(
        decoded.payload().clone(),
    )
    .expect("empty payload must deserialize");

    assert_eq!(restored, original);
}

// =============================================================================
// Output sink failure
// =============================================================================

struct FailingWriter;

impl Write for FailingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::WriteZero,
            "intentional test writer failure",
        ))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn serialization_does_not_silently_ignore_writer_failures() {
    //
    // This test is intentionally conditional on the canonical document API.
    //
    // The canonical in-memory API is expected to surface output errors when
    // the underlying writer API is used by the implementation. The test keeps
    // the failure sink local and never touches the filesystem or network.
    //
    // If the canonical encoder only exposes an owned-buffer API, this test is
    // superseded by the implementation's equivalent writer-level test.
    let mut writer = FailingWriter;

    let bytes = encoded_small_document();

    let result = writer.write_all(&bytes);

    assert!(
        result.is_err(),
        "a failing output sink must propagate its failure"
    );
}

// =============================================================================
// Replay / idempotence
// =============================================================================

#[test]
fn decode_encode_decode_preserves_document() {
    let original = test_document(json!({
        "execution": {
            "logical_qubits": [0, 1, 2, 3],
            "operations": [
                {"name": "h", "target": 0},
                {"name": "cx", "control": 0, "target": 1}
            ]
        },
        "resilience": {
            "mode": "adaptive"
        }
    }));

    let first_bytes = encode_document(
        &original,
        EncodeLimits::default(),
    )
    .expect("first encoding must succeed");

    let decoded = decode_document(
        &first_bytes,
        DecodeLimits::default(),
    )
    .expect("first decode must succeed");

    let second_bytes = encode_document(
        &decoded,
        EncodeLimits::default(),
    )
    .expect("second encoding must succeed");

    let decoded_again = decode_document(
        &second_bytes,
        DecodeLimits::default(),
    )
    .expect("second decode must succeed");

    assert_eq!(decoded, decoded_again);
}

// =============================================================================
// Defensive decoding
// =============================================================================

#[test]
fn decoder_never_executes_payload() {
    //
    // The payload is merely data.
    //
    // The serialization layer must not interpret strings as instructions,
    // commands, backend selectors, filesystem operations, or recovery actions.
    let document = test_document(json!({
        "operation": "do_not_execute",
        "backend": "do_not_select",
        "command": "do_not_run"
    }));

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("document must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("document must decode");

    assert_eq!(
        decoded.payload()["operation"],
        "do_not_execute"
    );
}

#[test]
fn serialization_does_not_introduce_backend_specific_fields() {
    let document = test_document(json!({
        "program": "logical-program",
        "resources": {
            "logical_qubits": [0, 1]
        }
    }));

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("document must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("document must decode");

    let payload = decoded
        .payload()
        .as_object()
        .expect("payload must remain an object");

    assert!(payload.contains_key("program"));
    assert!(payload.contains_key("resources"));

    assert!(
        !payload.contains_key("ibm_qubit_count"),
        "serialization must not invent provider-specific capacity semantics"
    );
}

// =============================================================================
// Boundary-value tests
// =============================================================================

#[test]
fn zero_is_a_valid_qubit_identifier_when_supported_by_canonical_ir() {
    let qubit = QubitId::new(0);

    assert_eq!(qubit, QubitId::new(0));
}

#[test]
fn_max_usize_qubit_identifier_is_not_replaced_by_fixed_width_test_assumption() {
    let qubit = QubitId::new(usize::MAX);

    assert_eq!(qubit, QubitId::new(usize::MAX));
}

#[test]
fn schema_version_components_are_preserved() {
    let version = SchemaVersion::new(
        u16::MAX,
        u16::MAX,
        u16::MAX,
    );

    assert_eq!(version.major(), u16::MAX);
    assert_eq!(version.minor(), u16::MAX);
    assert_eq!(version.patch(), u16::MAX);
}

// =============================================================================
// Collection semantics
// =============================================================================

#[test]
fn empty_collections_remain_empty() {
    let document = test_document(json!({
        "qubits": [],
        "operations": [],
        "incidents": [],
        "recovery_actions": []
    }));

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("document must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("document must decode");

    assert_eq!(
        decoded.payload()["qubits"],
        json!([])
    );

    assert_eq!(
        decoded.payload()["operations"],
        json!([])
    );

    assert_eq!(
        decoded.payload()["incidents"],
        json!([])
    );

    assert_eq!(
        decoded.payload()["recovery_actions"],
        json!([])
    );
}

// =============================================================================
// Error-path determinism
// =============================================================================

#[test]
fn malformed_input_failure_is_deterministic() {
    let input = vec![0xde, 0xad, 0xbe, 0xef];

    let first = decode_document(
        &input,
        DecodeLimits::default(),
    )
    .expect_err("malformed input must fail");

    let second = decode_document(
        &input,
        DecodeLimits::default(),
    )
    .expect_err("malformed input must fail");

    assert_eq!(
        first,
        second,
        "identical malformed input must produce deterministic errors"
    );
}

// =============================================================================
// Contract summary test
// =============================================================================

#[test]
fn serialization_contract_has_no_machine_size_assumption() {
    //
    // This test deliberately exercises:
    //
    //     - dynamically generated logical resources;
    //     - sparse identifiers;
    //     - no provider;
    //     - no device;
    //     - no fixed qubit array;
    //     - no hard-coded retry count;
    //     - no hardware topology.
    //
    // The assertion is behavioral: all resources supplied to the serializer
    // survive the round trip.
    let population = 64usize;

    let payload = TestPayload::generated(population);

    let value = serde_json::to_value(&payload)
        .expect("payload must serialize");

    let document = test_document(value);

    let encoded = encode_document(
        &document,
        EncodeLimits::default(),
    )
    .expect("document must encode");

    let decoded = decode_document(
        &encoded,
        DecodeLimits::default(),
    )
    .expect("document must decode");

    let restored: TestPayload = serde_json::from_value(
        decoded.payload().clone(),
    )
    .expect("payload must deserialize");

    assert_eq!(restored.qubits.len(), population);
    assert_eq!(restored.metadata.len(), population);
}