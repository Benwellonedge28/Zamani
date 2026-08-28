//! Zamani Quantum Hardware — Serialization Conformance Tests
//!
//! Production-grade integration and conformance tests for
//! `crate::quantum::hardware::serialization`.
//!
//! # Responsibility
//!
//! This module verifies the public serialization contract used by the Zamani
//! quantum hardware abstraction layer.
//!
//! It verifies:
//!
//! - serialization/deserialization round trips;
//! - deterministic canonical JSON;
//! - recursive object-key canonicalization;
//! - preservation of array ordering;
//! - versioned serialization envelopes;
//! - envelope schema validation;
//! - semantic document schema validation;
//! - serialization-format validation;
//! - serialization-format version validation;
//! - semantic schema-version validation;
//! - bounded serialization;
//! - bounded deserialization;
//! - JSON nesting limits;
//! - pre-parser depth protection through public APIs;
//! - fingerprints;
//! - payload fingerprint semantics;
//! - fingerprint verification;
//! - tamper detection;
//! - case-insensitive fingerprint verification;
//! - malformed fingerprints;
//! - optional fingerprints;
//! - serialized-document metadata;
//! - UTF-8 output;
//! - malformed JSON;
//! - non-object envelopes;
//! - malformed envelopes;
//! - invalid schema identifiers;
//! - invalid schema versions;
//! - invalid serialization options;
//! - deterministic output;
//! - `Send`/`Sync` suitability;
//! - public API compatibility.
//!
//! # Architectural boundary
//!
//! This test module intentionally tests the public serialization boundary
//! rather than private implementation details.
//!
//! It does NOT:
//!
//! - access private serializer functions;
//! - access private fields of semantic hardware types;
//! - depend on provider implementations;
//! - depend on network transports;
//! - depend on credentials;
//! - depend on authentication;
//! - depend on benchmarking;
//! - depend on Danga;
//! - depend on quantum compilation;
//! - depend on a particular quantum provider;
//! - duplicate the production serializer implementation.
//!
//! This is important because the production serializer is intentionally
//! provider-neutral and independent from the semantic hardware modules.
//!
//! # Integration contract
//!
//! Production code lives at:
//!
//! `crate::quantum::hardware::serialization`
//!
//! This test module consumes that public API exactly as a downstream hardware
//! consumer would.
//!
//! The dependency direction is:
//!
//! ```text
//! semantic hardware models
//!          |
//!          v
//! serialization.rs
//!          |
//!          v
//! persistence / cache / transport / audit
//! ```
//!
//! Tests therefore import the serializer through its public module path:
//!
//! ```text
//! crate::quantum::hardware::serialization
//! ```
//!
//! No production source file needs to be modified to satisfy these tests.
//!
//! # Mounting
//!
//! This file is intended to be mounted by `hardware/mod.rs` with:
//!
//! ```text
//! #[cfg(test)]
//! #[path = "tests/serialization.rs"]
//! mod serialization_tests;
//! ```
//!
//! Alternatively, it may be included from an existing hardware test module.
//!
//! The tests themselves contain no module-composition logic.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are used.
//!
//! # Security
//!
//! Serialized hardware data is untrusted input.
//!
//! These tests verify that public deserialization APIs reject:
//!
//! - oversized documents;
//! - excessive nesting;
//! - malformed JSON;
//! - malformed envelopes;
//! - wrong envelope schema;
//! - wrong envelope version;
//! - wrong semantic schema;
//! - wrong semantic version;
//! - invalid fingerprints;
//! - tampered fingerprints;
//! - invalid serialization options.
//!
//! The tests intentionally avoid private parser internals. Security behavior
//! must be observable through the same public API used by real callers.
//!
//! # Reproducibility
//!
//! Serialization must be deterministic so that serialized hardware metadata
//! can safely participate in:
//!
//! - cache keys;
//! - hardware snapshots;
//! - benchmark provenance;
//! - audit records;
//! - registry persistence;
//! - provider metadata exchange;
//! - reproducibility records.
//!
//! # Important semantic rule
//!
//! The fingerprint produced by the serializer identifies the canonical
//! semantic payload, not the entire envelope.
//!
//! Therefore changing envelope metadata without changing the payload does not
//! alter the semantic payload fingerprint.
//!
//! # Completion rule
//!
//! This file is complete when the complete public serialization contract is
//! covered without depending on private implementation details.
//!
//! Future semantic hardware modules should implement `Serialize` and
//! `Deserialize` and use the same public serialization APIs. These tests must
//! not need to be rewritten merely because another hardware subsystem is
//! implemented.
//!
//! -----------------------------------------------------------------------------
//! Imports
//! -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::quantum::hardware::serialization::{
    canonicalize_json,
    canonicalize_json_with_depth,
    deserialize_document,
    deserialize_document_with_options,
    deserialize_envelope,
    fingerprint_bytes,
    fingerprint_json,
    fingerprint_str,
    serialize_document,
    serialize_document_metadata,
    serialize_document_with_options,
    verify_fingerprint,
    DocumentEnvelope,
    SerializedDocument,
    SerializationError,
    SerializationFormat,
    SerializationOptions,
    DEFAULT_MAX_JSON_DEPTH,
    FINGERPRINT_HEX_LENGTH,
    MAX_FORMAT_ID_LENGTH,
    MAX_SCHEMA_ID_LENGTH,
    MAX_SCHEMA_VERSION,
    MAX_SERIALIZED_DOCUMENT_BYTES,
    SERIALIZATION_SCHEMA_ID,
    SERIALIZATION_SCHEMA_VERSION,
};

// =============================================================================
// Test constants
// =============================================================================

const TEST_SCHEMA_ID: &str = "zamani.quantum.hardware.tests.serialization";
const TEST_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Test models
// =============================================================================

/// Small deterministic semantic document used throughout the tests.
///
/// The test model intentionally represents generic hardware metadata rather
/// than a provider-specific type. This verifies that the serializer remains a
/// generic representation boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ExampleDocument {
    name: String,
    provider: String,
    qubits: u64,
    native_gates: Vec<String>,
    enabled: bool,
}

/// Nested document used to exercise recursive JSON canonicalization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct NestedDocument {
    metadata: ExampleMetadata,
}

/// Nested semantic metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ExampleMetadata {
    architecture: String,
    revision: String,
    firmware: String,
}

/// Serialize-only model that cannot be deserialized.
///
/// It is useful for verifying that serialization errors remain structured.
#[derive(Debug, Serialize)]
struct SerializeOnlyDocument {
    value: String,
}

/// Deserialize-only model that is not directly used for serialization.
#[derive(Debug, Deserialize, PartialEq, Eq)]
struct DeserializeOnlyDocument {
    value: String,
}

// =============================================================================
// Test helpers
// =============================================================================

fn example_document() -> ExampleDocument {
    ExampleDocument {
        name: "local-test-qpu".to_owned(),
        provider: "local".to_owned(),
        qubits: 8,
        native_gates: vec![
            "cx".to_owned(),
            "h".to_owned(),
            "rz".to_owned(),
        ],
        enabled: true,
    }
}

fn nested_document() -> NestedDocument {
    NestedDocument {
        metadata: ExampleMetadata {
            architecture: "zamani-test".to_owned(),
            revision: "A0".to_owned(),
            firmware: "1.0.0".to_owned(),
        },
    }
}

/// Construct a complete valid envelope with a representative payload.
fn valid_envelope() -> DocumentEnvelope {
    let payload = serde_json::to_value(example_document())
        .expect("test document must serialize to JSON");

    DocumentEnvelope::new(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        payload,
        true,
    )
    .expect("test envelope must be valid")
}

/// Compile-time assertion for thread-safety.
fn assert_send_sync<T: Send + Sync>() {}

/// Return a valid 64-character hexadecimal fingerprint.
fn valid_fingerprint() -> String {
    "00".repeat(FINGERPRINT_HEX_LENGTH / 2)
}

// =============================================================================
// Public API / concurrency contract
// =============================================================================

#[test]
fn public_serialization_types_are_send_and_sync() {
    assert_send_sync::<SerializationFormat>();
    assert_send_sync::<SerializationOptions>();
    assert_send_sync::<SerializationError>();
    assert_send_sync::<DocumentEnvelope>();
    assert_send_sync::<SerializedDocument>();
}

// =============================================================================
// Constants and format contract
// =============================================================================

#[test]
fn serialization_schema_constants_are_stable() {
    assert_eq!(
        SERIALIZATION_SCHEMA_ID,
        "zamani.quantum.hardware.serialization"
    );

    assert_eq!(SERIALIZATION_SCHEMA_VERSION, 1);
}

#[test]
fn production_limits_are_nonzero_and_consistent() {
    assert!(MAX_SERIALIZED_DOCUMENT_BYTES > 0);
    assert!(MAX_SCHEMA_ID_LENGTH > 0);
    assert!(MAX_FORMAT_ID_LENGTH > 0);
    assert!(DEFAULT_MAX_JSON_DEPTH > 0);
    assert!(FINGERPRINT_HEX_LENGTH > 0);
    assert!(MAX_SCHEMA_VERSION > 0);
}

#[test]
fn json_format_has_stable_identifier_and_version() {
    assert_eq!(SerializationFormat::Json.as_str(), "json");
    assert_eq!(SerializationFormat::Json.version(), 1);
    assert_eq!(
        SerializationFormat::Json.to_string(),
        "json"
    );
}

// =============================================================================
// Serialization options
// =============================================================================

#[test]
fn production_options_are_valid() {
    SerializationOptions::production()
        .validate()
        .expect("production options must always be valid");
}

#[test]
fn default_options_equal_production_options() {
    assert_eq!(
        SerializationOptions::default(),
        SerializationOptions::production()
    );
}

#[test]
fn zero_document_limit_is_rejected() {
    let options = SerializationOptions {
        max_document_bytes: 0,
        max_json_depth: DEFAULT_MAX_JSON_DEPTH,
        include_fingerprint: true,
    };

    let error = options
        .validate()
        .expect_err("zero document limit must fail");

    assert!(matches!(
        error,
        SerializationError::InvalidLimit {
            field: "max_document_bytes",
            value: 0,
        }
    ));
}

#[test]
fn zero_json_depth_limit_is_rejected() {
    let options = SerializationOptions {
        max_document_bytes: MAX_SERIALIZED_DOCUMENT_BYTES,
        max_json_depth: 0,
        include_fingerprint: true,
    };

    let error = options
        .validate()
        .expect_err("zero depth limit must fail");

    assert!(matches!(
        error,
        SerializationError::InvalidLimit {
            field: "max_json_depth",
            value: 0,
        }
    ));
}

#[test]
fn_document_limit_cannot_exceed_production_ceiling() {
    let options = SerializationOptions {
        max_document_bytes: MAX_SERIALIZED_DOCUMENT_BYTES + 1,
        max_json_depth: DEFAULT_MAX_JSON_DEPTH,
        include_fingerprint: true,
    };

    let error = options
        .validate()
        .expect_err("document limit above ceiling must fail");

    assert!(matches!(
        error,
        SerializationError::LimitTooLarge {
            field: "max_document_bytes",
            ..
        }
    ));
}

#[test]
fn json_depth_limit_has_no_artificial_lower_than_one_requirement() {
    let options = SerializationOptions {
        max_document_bytes: MAX_SERIALIZED_DOCUMENT_BYTES,
        max_json_depth: 1,
        include_fingerprint: true,
    };

    options
        .validate()
        .expect("depth one is a valid configured limit");
}

// =============================================================================
// Basic round trips
// =============================================================================

#[test]
fn generic_round_trip_is_lossless() {
    let original = example_document();

    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &original,
    )
    .expect("serialization must succeed");

    let decoded: ExampleDocument = deserialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &encoded,
    )
    .expect("deserialization must succeed");

    assert_eq!(original, decoded);
}

#[test]
fn nested_round_trip_is_lossless() {
    let original = nested_document();

    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &original,
    )
    .expect("nested serialization must succeed");

    let decoded: NestedDocument = deserialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &encoded,
    )
    .expect("nested deserialization must succeed");

    assert_eq!(original, decoded);
}

#[test]
fn serialized_output_is_valid_utf8() {
    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("serialization must succeed");

    let text = std::str::from_utf8(&encoded)
        .expect("JSON output must be valid UTF-8");

    assert!(!text.is_empty());
    assert!(text.starts_with('{'));
    assert!(text.ends_with('}'));
}

#[test]
fn serialized_output_is_a_complete_envelope() {
    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("serialization must succeed");

    let envelope = deserialize_envelope(
        &encoded,
        SerializationOptions::production(),
    )
    .expect("serialized output must be a valid envelope");

    assert_eq!(
        envelope.schema_id,
        SERIALIZATION_SCHEMA_ID
    );

    assert_eq!(
        envelope.schema_version,
        SERIALIZATION_SCHEMA_VERSION
    );

    assert_eq!(envelope.format, "json");
    assert_eq!(envelope.format_version, 1);

    assert_eq!(
        envelope.document_schema_id,
        TEST_SCHEMA_ID
    );

    assert_eq!(
        envelope.document_schema_version,
        TEST_SCHEMA_VERSION
    );
}

// =============================================================================
// Determinism / canonical JSON
// =============================================================================

#[test]
fn serialization_is_deterministic() {
    let value = example_document();

    let first = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &value,
    )
    .expect("first serialization must succeed");

    let second = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &value,
    )
    .expect("second serialization must succeed");

    assert_eq!(
        first,
        second,
        "identical semantic values must serialize identically"
    );
}

#[test]
fn canonicalization_sorts_object_keys() {
    let value = json!({
        "z": 1,
        "a": 2,
        "m": 3
    });

    let bytes = canonicalize_json(&value)
        .expect("canonicalization must succeed");

    assert_eq!(
        bytes,
        br#"{"a":2,"m":3,"z":1}"#
    );
}

#[test]
fn canonicalization_sorts_nested_object_keys() {
    let value = json!({
        "outer_z": {
            "z": 1,
            "a": 2
        },
        "outer_a": {
            "z": 3,
            "a": 4
        }
    });

    let bytes = canonicalize_json(&value)
        .expect("canonicalization must succeed");

    assert_eq!(
        bytes,
        br#"{"outer_a":{"a":4,"z":3},"outer_z":{"a":2,"z":1}}"#
    );
}

#[test]
fn canonicalization_preserves_array_order() {
    let value = json!([
        3,
        1,
        2,
        {
            "z": 4,
            "a": 5
        }
    ]);

    let bytes = canonicalize_json(&value)
        .expect("canonicalization must succeed");

    assert_eq!(
        bytes,
        br#"[3,1,2,{"a":5,"z":4}]"#
    );
}

#[test]
fn canonicalization_handles_all_json_value_kinds() {
    let value = json!({
        "null": null,
        "bool": true,
        "number": 42,
        "negative": -7,
        "float": 1.5,
        "string": "zamani",
        "array": [null, false, 0],
        "object": {
            "nested": "value"
        }
    });

    let bytes = canonicalize_json(&value)
        .expect("all JSON value kinds must canonicalize");

    let decoded: Value =
        serde_json::from_slice(&bytes)
            .expect("canonical output must remain valid JSON");

    assert_eq!(decoded, value);
}

// =============================================================================
// Canonicalization depth
// =============================================================================

#[test]
fn canonicalization_rejects_depth_above_limit() {
    let value = json!({
        "a": {
            "b": {
                "c": true
            }
        }
    });

    let error = canonicalize_json_with_depth(
        &value,
        2,
    )
    .expect_err("depth three must not fit depth two");

    assert!(matches!(
        error,
        SerializationError::JsonDepthExceeded {
            maximum: 2
        }
    ));
}

#[test]
fn canonicalization_accepts_value_at_depth_limit() {
    let value = json!({
        "a": {
            "b": true
        }
    });

    canonicalize_json_with_depth(
        &value,
        3,
    )
    .expect("value at the configured depth must succeed");
}

#[test]
fn canonicalization_rejects_zero_depth_limit() {
    let value = json!({
        "value": true
    });

    let error = canonicalize_json_with_depth(
        &value,
        0,
    )
    .expect_err("zero depth must fail");

    assert!(matches!(
        error,
        SerializationError::InvalidLimit {
            field: "max_json_depth",
            value: 0
        }
    ));
}

// =============================================================================
// Fingerprints
// =============================================================================

#[test]
fn fingerprint_has_expected_sha256_length() {
    let fingerprint = fingerprint_str("zamani");

    assert_eq!(
        fingerprint.len(),
        FINGERPRINT_HEX_LENGTH
    );

    assert!(
        fingerprint
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    );
}

#[test]
fn fingerprint_is_lowercase_hex() {
    let fingerprint = fingerprint_str("zamani");

    assert_eq!(
        fingerprint,
        fingerprint.to_ascii_lowercase()
    );
}

#[test]
fn fingerprint_matches_known_sha256_vector() {
    assert_eq!(
        fingerprint_str("zamani"),
        "0cd38465086ac2f07f9425c5695dca3f2f8d30f43a68f067c6aea012261992ca"
    );
}

#[test]
fn fingerprint_bytes_and_string_are_equivalent() {
    let value = "zamani";

    assert_eq!(
        fingerprint_bytes(value.as_bytes()),
        fingerprint_str(value)
    );
}

#[test]
fn fingerprint_json_uses_canonical_json() {
    let first = json!({
        "z": 1,
        "a": 2
    });

    let second = json!({
        "a": 2,
        "z": 1
    });

    assert_eq!(
        fingerprint_json(&first)
            .expect("first fingerprint must succeed"),
        fingerprint_json(&second)
            .expect("second fingerprint must succeed")
    );
}

#[test]
fn matching_fingerprint_is_accepted() {
    let bytes = b"zamani";
    let fingerprint = fingerprint_bytes(bytes);

    verify_fingerprint(
        &fingerprint,
        bytes,
    )
    .expect("matching fingerprint must verify");
}

#[test]
fn uppercase_fingerprint_is_accepted() {
    let bytes = b"zamani";
    let fingerprint =
        fingerprint_bytes(bytes).to_ascii_uppercase();

    verify_fingerprint(
        &fingerprint,
        bytes,
    )
    .expect("hexadecimal fingerprint comparison is case-insensitive");
}

#[test]
fn tampered_content_is_rejected() {
    let bytes = b"zamani";
    let fingerprint = fingerprint_bytes(bytes);

    let error = verify_fingerprint(
        &fingerprint,
        b"tampered",
    )
    .expect_err(
        "tampered content must not verify",
    );

    assert!(matches!(
        error,
        SerializationError::FingerprintMismatch { .. }
    ));
}

#[test]
fn malformed_fingerprint_is_rejected() {
    let error = verify_fingerprint(
        "not-a-fingerprint",
        b"zamani",
    )
    .expect_err(
        "malformed fingerprint must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidFingerprint { .. }
    ));
}

#[test]
fn wrong_length_fingerprint_is_rejected() {
    let error = verify_fingerprint(
        "00",
        b"zamani",
    )
    .expect_err(
        "short fingerprint must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidFingerprint { .. }
    ));
}

#[test]
fn non_hex_fingerprint_is_rejected() {
    let fingerprint =
        "g".repeat(FINGERPRINT_HEX_LENGTH);

    let error = verify_fingerprint(
        &fingerprint,
        b"zamani",
    )
    .expect_err(
        "non-hex fingerprint must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidFingerprint { .. }
    ));
}

// =============================================================================
// Payload fingerprint semantics
// =============================================================================

#[test]
fn serialized_metadata_contains_payload_fingerprint() {
    let document = serialize_document_metadata(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("metadata serialization must succeed");

    let fingerprint = document
        .fingerprint()
        .expect("production serialization should include fingerprint");

    assert_eq!(
        fingerprint.len(),
        FINGERPRINT_HEX_LENGTH
    );
}

#[test]
fn_payload_fingerprint_is_not_envelope_fingerprint() {
    let value = example_document();

    let payload =
        serde_json::to_value(&value)
            .expect("test value must serialize");

    let expected =
        fingerprint_json(&payload)
            .expect("payload fingerprint must succeed");

    let document = serialize_document_metadata(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &value,
    )
    .expect("serialization must succeed");

    assert_eq!(
        document.fingerprint(),
        Some(expected.as_str())
    );
}

#[test]
fn changing_semantic_payload_changes_payload_fingerprint() {
    let first = ExampleDocument {
        qubits: 8,
        ..example_document()
    };

    let second = ExampleDocument {
        qubits: 9,
        ..example_document()
    };

    let first_document = serialize_document_metadata(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &first,
    )
    .expect("first document must serialize");

    let second_document = serialize_document_metadata(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &second,
    )
    .expect("second document must serialize");

    assert_ne!(
        first_document.fingerprint(),
        second_document.fingerprint()
    );
}

// =============================================================================
// SerializedDocument metadata contract
// =============================================================================

#[test]
fn serialized_document_metadata_is_consistent() {
    let document = serialize_document_metadata(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("serialization must succeed");

    assert_eq!(
        document.document_schema_id(),
        TEST_SCHEMA_ID
    );

    assert_eq!(
        document.document_schema_version(),
        TEST_SCHEMA_VERSION
    );

    assert!(!document.is_empty());

    assert_eq!(
        document.len(),
        document.as_bytes().len()
    );

    assert_eq!(
        document.as_str()
            .expect("serialized output must be UTF-8")
            .as_bytes(),
        document.as_bytes()
    );
}

#[test]
fn serialized_document_into_bytes_matches_borrowed_bytes() {
    let document = serialize_document_metadata(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("serialization must succeed");

    let borrowed = document.as_bytes().to_vec();
    let owned = document.into_bytes();

    assert_eq!(borrowed, owned);
}

// =============================================================================
// Fingerprint option
// =============================================================================

#[test]
fn fingerprint_can_be_disabled_explicitly() {
    let options = SerializationOptions {
        max_document_bytes: MAX_SERIALIZED_DOCUMENT_BYTES,
        max_json_depth: DEFAULT_MAX_JSON_DEPTH,
        include_fingerprint: false,
    };

    let encoded = serialize_document_with_options(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
        options,
    )
    .expect("serialization without fingerprint must succeed");

    let envelope = deserialize_envelope(
        &encoded,
        options,
    )
    .expect("envelope without fingerprint must remain valid");

    assert!(
        envelope.fingerprint.is_none(),
        "fingerprint must be absent when explicitly disabled"
    );
}

#[test]
fn fingerprint_is_present_by_default() {
    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("production serialization must succeed");

    let envelope = deserialize_envelope(
        &encoded,
        SerializationOptions::production(),
    )
    .expect("production envelope must deserialize");

    assert!(
        envelope.fingerprint.is_some(),
        "production serialization must include integrity fingerprint"
    );
}

// =============================================================================
// Envelope construction
// =============================================================================

#[test]
fn document_envelope_constructor_creates_valid_envelope() {
    let envelope = valid_envelope();

    envelope
        .validate(
            Some(TEST_SCHEMA_ID),
            DEFAULT_MAX_JSON_DEPTH,
        )
        .expect("constructed envelope must validate");

    assert_eq!(
        envelope.schema_id,
        SERIALIZATION_SCHEMA_ID
    );

    assert_eq!(
        envelope.schema_version,
        SERIALIZATION_SCHEMA_VERSION
    );

    assert_eq!(
        envelope.document_schema_id,
        TEST_SCHEMA_ID
    );

    assert_eq!(
        envelope.document_schema_version,
        TEST_SCHEMA_VERSION
    );
}

#[test]
fn document_envelope_can_be_created_without_fingerprint() {
    let payload = json!({
        "value": "zamani"
    });

    let envelope = DocumentEnvelope::new(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        payload,
        false,
    )
    .expect("envelope without fingerprint must be valid");

    assert!(envelope.fingerprint.is_none());

    envelope
        .validate(
            Some(TEST_SCHEMA_ID),
            DEFAULT_MAX_JSON_DEPTH,
        )
        .expect("fingerprint-free envelope must validate");
}

// =============================================================================
// Schema validation
// =============================================================================

#[test]
fn empty_schema_identifier_is_rejected() {
    let error = serialize_document(
        "",
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect_err(
        "empty schema identifier must fail",
    );

    assert!(matches!(
        error,
        SerializationError::EmptySchemaId
    ));
}

#[test]
fn schema_identifier_with_whitespace_is_rejected() {
    for schema in [
        "zamani quantum",
        " zamani.quantum",
        "zamani.quantum ",
        "zamani\tquantum",
        "zamani\nquantum",
    ] {
        let error = serialize_document(
            schema,
            TEST_SCHEMA_VERSION,
            &example_document(),
        )
        .expect_err(
            "schema identifier containing whitespace must fail",
        );

        assert!(
            matches!(
                error,
                SerializationError::InvalidSchemaId
            ),
            "unexpected error for {schema:?}: {error:?}"
        );
    }
}

#[test]
fn schema_identifier_with_unsupported_characters_is_rejected() {
    for schema in [
        "zamani quantum",
        "zamani:quantum",
        "zamani?quantum",
        "zamani#quantum",
        "zamani@quantum",
        "zamani\\quantum",
        "zamani%quantum",
        "量子",
    ] {
        let error = serialize_document(
            schema,
            TEST_SCHEMA_VERSION,
            &example_document(),
        )
        .expect_err(
            "unsupported schema identifier syntax must fail",
        );

        assert!(
            matches!(
                error,
                SerializationError::InvalidSchemaId
            ),
            "unexpected error for {schema:?}: {error:?}"
        );
    }
}

#[test]
fn schema_identifier_accepts_stable_hardware_namespace_syntax() {
    for schema in [
        "zamani.quantum.hardware",
        "zamani.quantum.hardware.backend",
        "zamani.quantum.hardware/v1",
        "provider-1.hardware",
        "provider_1.hardware",
    ] {
        serialize_document(
            schema,
            TEST_SCHEMA_VERSION,
            &example_document(),
        )
        .expect(
            "stable schema identifiers should be accepted"
        );
    }
}

#[test]
fn schema_identifier_length_limit_is_enforced() {
    let accepted =
        "a".repeat(MAX_SCHEMA_ID_LENGTH);

    serialize_document(
        &accepted,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("maximum schema identifier length must be accepted");

    let rejected =
        "a".repeat(MAX_SCHEMA_ID_LENGTH + 1);

    let error = serialize_document(
        &rejected,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect_err(
        "schema identifier above maximum must fail",
    );

    assert!(matches!(
        error,
        SerializationError::SchemaIdTooLong {
            length,
            maximum: MAX_SCHEMA_ID_LENGTH,
        } if length == MAX_SCHEMA_ID_LENGTH + 1
    ));
}

#[test]
fn zero_semantic_schema_version_is_rejected() {
    let error = serialize_document(
        TEST_SCHEMA_ID,
        0,
        &example_document(),
    )
    .expect_err(
        "semantic schema version zero must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidSchemaVersion {
            version: 0
        }
    ));
}

#[test]
fn semantic_schema_version_above_supported_maximum_is_rejected() {
    let error = serialize_document(
        TEST_SCHEMA_ID,
        MAX_SCHEMA_VERSION + 1,
        &example_document(),
    )
    .expect_err(
        "semantic schema version above maximum must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidSchemaVersion { .. }
    ));
}

// =============================================================================
// Schema compatibility
// =============================================================================

#[test]
fn wrong_document_schema_is_rejected() {
    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("serialization must succeed");

    let error = deserialize_document::<ExampleDocument>(
        "zamani.quantum.hardware.other",
        TEST_SCHEMA_VERSION,
        &encoded,
    )
    .expect_err(
        "wrong semantic schema must fail",
    );

    assert!(matches!(
        error,
        SerializationError::SchemaMismatch { .. }
    ));
}

#[test]
fn wrong_document_schema_version_is_rejected() {
    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("serialization must succeed");

    let error = deserialize_document::<ExampleDocument>(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION + 1,
        &encoded,
    )
    .expect_err(
        "wrong semantic schema version must fail",
    );

    assert!(matches!(
        error,
        SerializationError::UnsupportedSchemaVersion {
            ..
        }
    ));
}

#[test]
fn unknown_envelope_schema_is_rejected() {
    let value = json!({
        "schema_id": "zamani.quantum.hardware.unknown",
        "schema_version": SERIALIZATION_SCHEMA_VERSION,
        "format": "json",
        "format_version": 1,
        "document_schema_id": TEST_SCHEMA_ID,
        "document_schema_version": TEST_SCHEMA_VERSION,
        "payload": {
            "name": "local-test-qpu",
            "provider": "local",
            "qubits": 8,
            "native_gates": ["h"],
            "enabled": true
        }
    });

    let bytes = serde_json::to_vec(&value)
        .expect("test envelope must serialize");

    let error = deserialize_envelope(
        &bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "unknown envelope schema must fail",
    );

    assert!(matches!(
        error,
        SerializationError::SchemaMismatch { .. }
    ));
}

#[test]
fn unknown_envelope_schema_version_is_rejected() {
    let mut value = valid_envelope();

    value.schema_version =
        SERIALIZATION_SCHEMA_VERSION + 1;

    let bytes = serde_json::to_vec(&value)
        .expect("test envelope must serialize");

    let error = deserialize_envelope(
        &bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "unknown envelope version must fail",
    );

    assert!(matches!(
        error,
        SerializationError::UnsupportedSchemaVersion {
            ..
        }
    ));
}

#[test]
fn wrong_format_identifier_is_rejected() {
    let mut value = valid_envelope();

    value.format = "yaml".to_owned();

    let bytes = serde_json::to_vec(&value)
        .expect("test envelope must serialize");

    let error = deserialize_envelope(
        &bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "unsupported format must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidFormat
    ));
}

#[test]
fn wrong_format_version_is_rejected() {
    let mut value = valid_envelope();

    value.format_version = 2;

    let bytes = serde_json::to_vec(&value)
        .expect("test envelope must serialize");

    let error = deserialize_envelope(
        &bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "unsupported format version must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidField {
            field: "format_version",
            ..
        }
    ));
}

#[test]
fn malformed_format_identifier_length_is_rejected() {
    let mut value = valid_envelope();

    value.format =
        "x".repeat(MAX_FORMAT_ID_LENGTH + 1);

    let bytes = serde_json::to_vec(&value)
        .expect("test envelope must serialize");

    let error = deserialize_envelope(
        &bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "oversized format identifier must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidField {
            field: "format",
            ..
        }
    ));
}

// =============================================================================
// Envelope structure
// =============================================================================

#[test]
fn non_object_envelope_is_rejected() {
    let bytes = br#"[]"#;

    let error = deserialize_envelope(
        bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "array cannot be a serialization envelope",
    );

    assert!(matches!(
        error,
        SerializationError::RootMustBeObject
    ));
}

#[test]
fn scalar_envelope_is_rejected() {
    for bytes in [
        br#"null"#.as_slice(),
        br#"true"#.as_slice(),
        br#"42"#.as_slice(),
        br#""text""#.as_slice(),
    ] {
        let error = deserialize_envelope(
            bytes,
            SerializationOptions::production(),
        )
        .expect_err(
            "scalar cannot be a serialization envelope",
        );

        assert!(matches!(
            error,
            SerializationError::RootMustBeObject
        ));
    }
}

#[test]
fn malformed_json_is_rejected() {
    let bytes = br#"{"schema_id":"broken""#;

    let error = deserialize_envelope(
        bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "malformed JSON must fail",
    );

    assert!(matches!(
        error,
        SerializationError::Deserialize { .. }
    ));
}

#[test]
fn malformed_envelope_is_rejected() {
    let value = json!({
        "not_an_envelope": true
    });

    let bytes = serde_json::to_vec(&value)
        .expect("test JSON must serialize");

    let error = deserialize_envelope(
        &bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "missing required envelope fields must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidEnvelope { .. }
    ));
}

// =============================================================================
// Fingerprint validation at envelope boundary
// =============================================================================

#[test]
fn invalid_envelope_fingerprint_is_rejected() {
    let mut value = valid_envelope();

    value.fingerprint =
        Some("invalid".to_owned());

    let bytes = serde_json::to_vec(&value)
        .expect("test envelope must serialize");

    let error = deserialize_envelope(
        &bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "invalid fingerprint must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidFingerprint { .. }
    ));
}

#[test]
fn malformed_hex_envelope_fingerprint_is_rejected() {
    let mut value = valid_envelope();

    value.fingerprint =
        Some("g".repeat(FINGERPRINT_HEX_LENGTH));

    let bytes = serde_json::to_vec(&value)
        .expect("test envelope must serialize");

    let error = deserialize_envelope(
        &bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "non-hex fingerprint must fail",
    );

    assert!(matches!(
        error,
        SerializationError::InvalidFingerprint { .. }
    ));
}

#[test]
fn wrong_envelope_fingerprint_is_rejected() {
    let mut value = valid_envelope();

    value.fingerprint =
        Some(valid_fingerprint());

    let bytes = serde_json::to_vec(&value)
        .expect("test envelope must serialize");

    let error = deserialize_envelope(
        &bytes,
        SerializationOptions::production(),
    )
    .expect_err(
        "incorrect fingerprint must fail",
    );

    assert!(matches!(
        error,
        SerializationError::FingerprintMismatch { .. }
    ));
}

#[test]
fn envelope_without_fingerprint_is_allowed() {
    let mut value = valid_envelope();

    value.fingerprint = None;

    let bytes = serde_json::to_vec(&value)
        .expect("test envelope must serialize");

    deserialize_envelope(
        &bytes,
        SerializationOptions::production(),
    )
    .expect(
        "fingerprint-free envelope should be valid when integrity field is absent"
    );
}

// =============================================================================
// Size limits
// =============================================================================

#[test]
fn oversized_deserialization_input_is_rejected_before_semantic_decode() {
    let options = SerializationOptions {
        max_document_bytes: 4,
        max_json_depth: DEFAULT_MAX_JSON_DEPTH,
        include_fingerprint: true,
    };

    let bytes =
        br#"{"schema_id":"too-large"}"#;

    let error = deserialize_document_with_options::<ExampleDocument>(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        bytes,
        options,
    )
    .expect_err(
        "input above configured size limit must fail",
    );

    assert!(matches!(
        error,
        SerializationError::DocumentTooLarge {
            ..
        }
    ));
}

#[test]
fn oversized_serialization_output_is_rejected() {
    let options = SerializationOptions {
        max_document_bytes: 32,
        max_json_depth: DEFAULT_MAX_JSON_DEPTH,
        include_fingerprint: true,
    };

    let error = serialize_document_with_options(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
        options,
    )
    .expect_err(
        "serialized document above configured limit must fail",
    );

    assert!(matches!(
        error,
        SerializationError::DocumentTooLarge {
            ..
        }
    ));
}

// =============================================================================
// JSON nesting security
// =============================================================================

#[test]
fn deeply_nested_json_is_rejected_by_public_deserializer() {
    let bytes =
        br#"{"a":{"b":{"c":{"d":true}}}}"#;

    let options = SerializationOptions {
        max_document_bytes: MAX_SERIALIZED_DOCUMENT_BYTES,
        max_json_depth: 2,
        include_fingerprint: true,
    };

    let error = deserialize_envelope(
        bytes,
        options,
    )
    .expect_err(
        "excessive structural depth must be rejected",
    );

    assert!(matches!(
        error,
        SerializationError::JsonDepthExceeded {
            maximum: 2
        }
    ));
}

#[test]
fn braces_and_brackets_inside_strings_do_not_count_as_structure() {
    let value = json!({
        "text": "{{{{[[[[}}}}]]]]"
    });

    canonicalize_json_with_depth(
        &value,
        3,
    )
    .expect(
        "delimiter characters inside strings are not structural nesting"
    );
}

#[test]
fn nested_payload_respects_configured_depth() {
    let value = json!({
        "level_one": {
            "level_two": true
        }
    });

    canonicalize_json_with_depth(
        &value,
        3,
    )
    .expect(
        "payload at configured depth should succeed"
    );
}

// =============================================================================
// Deserialization safety / type mismatch
// =============================================================================

#[test]
fn valid_envelope_with_wrong_payload_type_is_rejected() {
    let mut value = valid_envelope();

    value.payload = json!({
        "unexpected": "payload"
    });

    value.fingerprint = Some(
        fingerprint_json(&value.payload)
            .expect("test payload fingerprint must succeed")
    );

    let bytes = serde_json::to_vec(&value)
        .expect("test envelope must serialize");

    let error = deserialize_document::<ExampleDocument>(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &bytes,
    )
    .expect_err(
        "payload that does not match target semantic type must fail"
    );

    assert!(matches!(
        error,
        SerializationError::Deserialize { .. }
    ));
}

#[test]
fn deserialization_rejects_non_json_input() {
    let error = deserialize_document::<ExampleDocument>(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        b"not-json",
    )
    .expect_err(
        "non-JSON input must fail"
    );

    assert!(matches!(
        error,
        SerializationError::Deserialize { .. }
    ));
}

// =============================================================================
// Explicit serializer/deserializer options
// =============================================================================

#[test]
fn explicit_production_options_preserve_round_trip() {
    let options =
        SerializationOptions::production();

    let encoded = serialize_document_with_options(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
        options,
    )
    .expect("serialization must succeed");

    let decoded: ExampleDocument =
        deserialize_document_with_options(
            TEST_SCHEMA_ID,
            TEST_SCHEMA_VERSION,
            &encoded,
            options,
        )
        .expect("deserialization must succeed");

    assert_eq!(
        decoded,
        example_document()
    );
}

#[test]
fn disabled_fingerprint_round_trip_is_lossless() {
    let options = SerializationOptions {
        max_document_bytes: MAX_SERIALIZED_DOCUMENT_BYTES,
        max_json_depth: DEFAULT_MAX_JSON_DEPTH,
        include_fingerprint: false,
    };

    let original = example_document();

    let encoded = serialize_document_with_options(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &original,
        options,
    )
    .expect("serialization must succeed");

    let decoded: ExampleDocument =
        deserialize_document_with_options(
            TEST_SCHEMA_ID,
            TEST_SCHEMA_VERSION,
            &encoded,
            options,
        )
        .expect("deserialization must succeed");

    assert_eq!(original, decoded);
}

// =============================================================================
// Serialization error mapping
// =============================================================================

#[test]
fn serialization_failure_remains_structured() {
    /*
     * The serializer currently accepts all fields in this model, so the
     * primary contract we can test without private implementation coupling is
     * that successful values remain serializable and errors use the public
     * structured error type.
     *
     * This test also ensures the public generic API remains usable with
     * arbitrary Serialize implementations.
     */
    let value = SerializeOnlyDocument {
        value: "zamani".to_owned(),
    };

    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &value,
    )
    .expect("serialize-only document must serialize");

    assert!(!encoded.is_empty());
}

#[test]
fn deserialization_target_can_be_independently_typed() {
    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &DeserializeOnlyDocument {
            value: "zamani".to_owned(),
        },
    )
    .expect("test value must serialize");

    let decoded: DeserializeOnlyDocument =
        deserialize_document(
            TEST_SCHEMA_ID,
            TEST_SCHEMA_VERSION,
            &encoded,
        )
        .expect("typed deserialization must succeed");

    assert_eq!(
        decoded,
        DeserializeOnlyDocument {
            value: "zamani".to_owned()
        }
    );
}

// =============================================================================
// Canonical semantic equivalence
// =============================================================================

#[test]
fn semantically_equal_json_with_different_object_order_has_equal_fingerprint() {
    let first = json!({
        "provider": "ibm",
        "backend": "torino",
        "qubits": 133
    });

    let second = json!({
        "qubits": 133,
        "backend": "torino",
        "provider": "ibm"
    });

    assert_eq!(
        fingerprint_json(&first)
            .expect("first fingerprint must succeed"),
        fingerprint_json(&second)
            .expect("second fingerprint must succeed")
    );
}

#[test]
fn arrays_with_different_order_have_different_fingerprints() {
    let first = json!([
        "h",
        "cx",
        "rz"
    ]);

    let second = json!([
        "rz",
        "cx",
        "h"
    ]);

    assert_ne!(
        fingerprint_json(&first)
            .expect("first fingerprint must succeed"),
        fingerprint_json(&second)
            .expect("second fingerprint must succeed")
    );
}

// =============================================================================
// Public envelope validation
// =============================================================================

#[test]
fn envelope_validation_rejects_wrong_expected_schema() {
    let envelope = valid_envelope();

    let error = envelope
        .validate(
            Some("zamani.quantum.hardware.other"),
            DEFAULT_MAX_JSON_DEPTH,
        )
        .expect_err(
            "wrong expected schema must fail"
        );

    assert!(matches!(
        error,
        SerializationError::SchemaMismatch { .. }
    ));
}

#[test]
fn envelope_validation_accepts_no_expected_schema() {
    let envelope = valid_envelope();

    envelope
        .validate(
            None,
            DEFAULT_MAX_JSON_DEPTH,
        )
        .expect(
            "structurally valid envelope should validate without expected schema"
        );
}

#[test]
fn envelope_validation_rejects_zero_depth_limit() {
    let envelope = valid_envelope();

    let error = envelope
        .validate(
            Some(TEST_SCHEMA_ID),
            0,
        )
        .expect_err(
            "zero validation depth must fail"
        );

    assert!(matches!(
        error,
        SerializationError::InvalidLimit {
            field: "max_json_depth",
            value: 0
        }
    ));
}

// =============================================================================
// Regression tests for previously dangerous boundaries
// =============================================================================

#[test]
fn serializer_does_not_silently_accept_wrong_semantic_version() {
    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("serialization must succeed");

    let result =
        deserialize_document::<ExampleDocument>(
            TEST_SCHEMA_ID,
            TEST_SCHEMA_VERSION + 1,
            &encoded,
        );

    assert!(
        result.is_err(),
        "semantic schema version mismatch must never be silently accepted"
    );
}

#[test]
fn serializer_does_not_silently_accept_wrong_envelope_schema() {
    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("serialization must succeed");

    let mut envelope =
        deserialize_envelope(
            &encoded,
            SerializationOptions::production(),
        )
        .expect("generated envelope must be valid");

    envelope.schema_id =
        "attacker.controlled.schema".to_owned();

    let tampered =
        serde_json::to_vec(&envelope)
            .expect("tampered envelope must serialize");

    let result =
        deserialize_envelope(
            &tampered,
            SerializationOptions::production(),
        );

    assert!(
        result.is_err(),
        "unknown envelope schema must never be silently accepted"
    );
}

#[test]
fn serializer_does_not_silently_accept_tampered_payload() {
    let encoded = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &example_document(),
    )
    .expect("serialization must succeed");

    let mut envelope =
        deserialize_envelope(
            &encoded,
            SerializationOptions::production(),
        )
        .expect("generated envelope must be valid");

    envelope.payload["qubits"] =
        json!(999999);

    let tampered =
        serde_json::to_vec(&envelope)
            .expect("tampered envelope must serialize");

    let result =
        deserialize_document::<ExampleDocument>(
            TEST_SCHEMA_ID,
            TEST_SCHEMA_VERSION,
            &tampered,
        );

    assert!(
        matches!(
            result,
            Err(SerializationError::FingerprintMismatch { .. })
        ),
        "payload tampering must be detected by the integrity fingerprint"
    );
}

// =============================================================================
// Integration guarantees
// =============================================================================

#[test]
fn public_serialization_api_is_provider_neutral() {
    /*
     * This test intentionally uses generic semantic metadata rather than an
     * IBM/IonQ/Braket/etc. model.
     *
     * The serializer must remain independent of provider implementations.
     */
    let value = json!({
        "provider": "provider-neutral",
        "backend": "generic",
        "technology": "quantum",
        "capabilities": [
            "measurement",
            "reset"
        ]
    });

    let encoded = serialize_document(
        "zamani.quantum.hardware.provider-neutral",
        1,
        &value,
    )
    .expect("provider-neutral metadata must serialize");

    let decoded: Value =
        deserialize_document(
            "zamani.quantum.hardware.provider-neutral",
            1,
            &encoded,
        )
        .expect("provider-neutral metadata must deserialize");

    assert_eq!(decoded, value);
}

#[test]
fn serialization_boundary_is_independent_of_backend_implementation() {
    /*
     * This is deliberately a generic contract test.
     *
     * The serializer accepts any Serialize/Deserialize pair and therefore does
     * not require backend.rs, provider adapters, credentials, execution, jobs,
     * benchmarking, or Danga to be implemented.
     */
    let value = json!({
        "backend_id": "local-test-qpu",
        "status": "available",
        "qubits": 8
    });

    let encoded = serialize_document(
        "zamani.quantum.hardware.backend.snapshot",
        1,
        &value,
    )
    .expect("backend-shaped metadata must serialize");

    let decoded: Value =
        deserialize_document(
            "zamani.quantum.hardware.backend.snapshot",
            1,
            &encoded,
        )
        .expect("backend-shaped metadata must deserialize");

    assert_eq!(decoded, value);
}

// =============================================================================
// Final deterministic contract
// =============================================================================

#[test]
fn complete_production_serialization_contract_is_deterministic() {
    let value = example_document();

    let first = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &value,
    )
    .expect("first production serialization must succeed");

    let second = serialize_document(
        TEST_SCHEMA_ID,
        TEST_SCHEMA_VERSION,
        &value,
    )
    .expect("second production serialization must succeed");

    assert_eq!(
        first,
        second,
        "production serialization must be byte-for-byte deterministic"
    );

    let envelope =
        deserialize_envelope(
            &first,
            SerializationOptions::production(),
        )
        .expect("production output must be self-validating");

    envelope
        .validate(
            Some(TEST_SCHEMA_ID),
            DEFAULT_MAX_JSON_DEPTH,
        )
        .expect(
            "production envelope must satisfy the complete validation contract"
        );

    let decoded: ExampleDocument =
        deserialize_document(
            TEST_SCHEMA_ID,
            TEST_SCHEMA_VERSION,
            &first,
        )
        .expect(
            "production output must be recoverable as its semantic type"
        );

    assert_eq!(
        decoded,
        value,
        "production serialization must preserve semantic value"
    );
}