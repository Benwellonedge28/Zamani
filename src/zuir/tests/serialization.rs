
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! ZUIR serialization conformance and integration tests.
//!
//! Path: src/zuir/tests/serialization.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is prohibited.
//!
//! PURPOSE
//!
//! Exercise the public ZUIR serialization API, including canonical JSON,
//! versioned document encoding and decoding, schema compatibility,
//! validation budgets, reader/writer integration, malformed input, and
//! deterministic handling of dynamically sized payloads.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/serialization/mod.rs
//! - src/zuir/serialization/schema.rs
//! - src/zuir/serialization/encode.rs
//! - src/zuir/serialization/decode.rs
//! - src/zuir/serialization/canonical.rs
//! - src/zuir/serialization/validation.rs
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/antlr/ZamaniLexer.g4
//! - grammar/antlr/ZamaniParser.g4
//! - grammar/grammar.md
//! - grammar/specification/semantic-model.md
//!
//! OWNERSHIP
//!
//! This file owns serialization conformance tests only. It does not define
//! a second serializer, IR schema, source parser, semantic model, or IR
//! verifier.
//!
//! GRAMMAR BOUNDARY
//!
//! Grammar files own source syntax. Semantic analysis owns source meaning.
//! ZUIR owns canonical computational meaning. Serialization transports a
//! representation; successful decoding alone does not prove that a payload
//! is a valid or executable ZUIR program.
//!
//! SCALABILITY
//!
//! Tests use dynamically sized values and configurable budgets. They must
//! not encode universal limits on operations, qubits, processors, registers,
//! tensor rank, memory, devices, or topology.
//!
//! A finite test cannot establish literal infinite scalability. Stress cases
//! provide regression coverage for the exercised workload only.
//!
//! FREEZE CRITERIA
//!
//! - The serialization facade exports the APIs imported below.
//! - The parent ZUIR module includes this test module under cfg(test).
//! - The crate compiles on the declared stable Rust baseline.
//! - All tests pass against the repository's real implementations.
//! - No unsafe code or duplicate serialization implementation is introduced.

use std::io::{self, Cursor, Read, Write};

use serde_json::{json, Value};

use crate::zuir::serialization::{
    canonical_json_bytes,
    canonical_json_string,
    decode_document,
    decode_document_from_reader,
    decode_document_from_reader_with_options,
    decode_document_with_options,
    encode_document,
    encode_document_to_writer,
    encode_document_to_writer_with_options,
    encode_document_with_options,
    validate_payload,
    validate_payload_unbounded,
    CanonicalOptions,
    DecodeError,
    DecodeOptions,
    EncodeError,
    EncodeOptions,
    FieldCardinality,
    FieldDescriptor,
    FieldId,
    SchemaDescriptor,
    SchemaId,
    SchemaKind,
    SchemaVersion,
    UnknownFieldPolicy,
    ValidationError,
    ValidationLimits,
    WireKind,
    CURRENT_SCHEMA_VERSION,
    DOCUMENT_SCHEMA_ID,
};

// ---------------------------------------------------------------------------
// Test support
// ---------------------------------------------------------------------------

fn example_payload() -> Value {
    json!({
        "program": {
            "name": "portable-example",
            "domains": [
                "classical",
                "quantum",
                "hdl",
                "distributed"
            ],
            "operations": [
                {
                    "dialect": "core",
                    "name": "example.compute",
                    "operands": [0, 1],
                    "results": [2]
                },
                {
                    "dialect": "extension.example",
                    "name": "example.future_operation",
                    "attributes": {
                        "preserve_unknown_data": true
                    }
                }
            ]
        },
        "metadata": {
            "description": "Serialization test payload",
            "unicode": "Zamani — quantum λ — 界",
            "nullable": null
        }
    })
}

fn make_document(payload: &Value) -> Vec<u8> {
    encode_document(payload)
        .expect("test payload should encode")
        .into_bytes()
}

fn schema_with_fields(
    fields: Vec<FieldDescriptor>,
    unknown_fields: UnknownFieldPolicy,
) -> SchemaDescriptor {
    SchemaDescriptor::new(
        SchemaId::new("zamani.tests.serialization.payload")
            .expect("test schema identifier must be valid"),
        SchemaKind::Payload,
        SchemaVersion::new(1, 0, 0),
        crate::zuir::serialization::CompatibilityPolicy::Exact,
        unknown_fields,
        false,
        fields,
    )
}

fn field(
    id: u32,
    name: &str,
    kind: WireKind,
    cardinality: FieldCardinality,
) -> FieldDescriptor {
    FieldDescriptor::new(
        FieldId::new(id).expect("field identifiers must be nonzero"),
        name,
        kind,
        cardinality,
        true,
        true,
    )
    .expect("test field descriptor should be valid")
}

fn rewrite_document(
    document: &[u8],
    mutate: impl FnOnce(&mut Value),
) -> Vec<u8> {
    let mut value: Value =
        serde_json::from_slice(document).expect("encoded envelope is JSON");

    mutate(&mut value);

    serde_json::to_vec(&value).expect("modified envelope should encode")
}

// A destination that deterministically fails. This verifies that writer
// failures propagate rather than being reported as successful serialization.
struct FailingWriter;

impl Write for FailingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "intentional serialization-test failure",
        ))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// A source that deterministically fails during reader-based decoding.
struct FailingReader;

impl Read for FailingReader {
    fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "intentional deserialization-test failure",
        ))
    }
}

// ---------------------------------------------------------------------------
// Canonical JSON
// ---------------------------------------------------------------------------

#[test]
fn canonical_json_sorts_object_keys_deterministically() {
    let first: Value =
        serde_json::from_str(r#"{"z":3,"a":1,"m":2}"#)
            .expect("valid JSON");

    let second: Value =
        serde_json::from_str(r#"{"m":2,"z":3,"a":1}"#)
            .expect("valid JSON");

    let first_bytes = canonical_json_bytes(
        &first,
        CanonicalOptions::unbounded(),
    )
    .expect("canonical rendering should succeed");

    let second_bytes = canonical_json_bytes(
        &second,
        CanonicalOptions::unbounded(),
    )
    .expect("canonical rendering should succeed");

    assert_eq!(first_bytes, second_bytes);
    assert_eq!(
        first_bytes,
        br#"{"a":1,"m":2,"z":3}"#
    );
}

#[test]
fn canonical_json_preserves_array_order() {
    let first = json!({"operations": [1, 2, 3]});
    let second = json!({"operations": [3, 2, 1]});

    let first_bytes = canonical_json_bytes(
        &first,
        CanonicalOptions::unbounded(),
    )
    .expect("canonical rendering should succeed");

    let second_bytes = canonical_json_bytes(
        &second,
        CanonicalOptions::unbounded(),
    )
    .expect("canonical rendering should succeed");

    assert_ne!(
        first_bytes,
        second_bytes,
        "canonicalization must not reorder semantic sequences"
    );
}

#[test]
fn canonical_json_handles_unicode_and_escaping() {
    let input = json!({
        "unicode": "λ 界",
        "escaped": "quote: \" newline:\n backslash:\\"
    });

    let rendered = canonical_json_string(
        &input,
        CanonicalOptions::unbounded(),
    )
    .expect("valid strings should render");

    let reparsed: Value =
        serde_json::from_str(&rendered).expect("canonical JSON is valid");

    assert_eq!(reparsed, input);
}

#[test]
fn canonical_json_supports_scalar_and_empty_values() {
    let cases = [
        (Value::Null, "null"),
        (json!(true), "true"),
        (json!(false), "false"),
        (json!([]), "[]"),
        (json!({}), "{}"),
    ];

    for (input, expected) in cases {
        let rendered = canonical_json_string(
            &input,
            CanonicalOptions::unbounded(),
        )
        .expect("supported JSON value should render");

        assert_eq!(rendered, expected);
    }
}

#[test]
fn canonical_json_enforces_node_depth_and_output_budgets() {
    let input = json!({
        "outer": {
            "inner": [1, 2, 3]
        }
    });

    let node_limited = canonical_json_bytes(
        &input,
        CanonicalOptions::unbounded().with_max_nodes(Some(2)),
    );

    assert!(
        node_limited.is_err(),
        "exceeding the node budget must fail explicitly"
    );

    let depth_limited = canonical_json_bytes(
        &input,
        CanonicalOptions::unbounded().with_max_depth(Some(1)),
    );

    assert!(
        depth_limited.is_err(),
        "exceeding the depth budget must fail explicitly"
    );

    let exact = canonical_json_bytes(
        &json!("x"),
        CanonicalOptions::unbounded()
            .with_max_output_bytes(Some(3)),
    )
    .expect("three bytes should fit");

    assert_eq!(exact, br#""x""#);

    assert!(
        canonical_json_bytes(
            &json!("x"),
            CanonicalOptions::unbounded()
                .with_max_output_bytes(Some(2)),
        )
        .is_err(),
        "one byte over budget must fail"
    );
}

// ---------------------------------------------------------------------------
// Encoding and document envelope
// ---------------------------------------------------------------------------

#[test]
fn encoder_emits_the_required_versioned_envelope() {
    let payload = example_payload();
    let artifact = encode_document(&payload)
        .expect("valid payload should encode");

    let envelope: Value = serde_json::from_slice(artifact.as_bytes())
        .expect("encoded artifact must be valid JSON");

    assert_eq!(
        envelope["schema_id"],
        DOCUMENT_SCHEMA_ID
    );

    assert_eq!(
        envelope["schema_version"]["major"],
        CURRENT_SCHEMA_VERSION.major()
    );

    assert_eq!(
        envelope["schema_version"]["minor"],
        CURRENT_SCHEMA_VERSION.minor()
    );

    assert_eq!(
        envelope["schema_version"]["patch"],
        CURRENT_SCHEMA_VERSION.patch()
    );

    assert!(
        envelope["format_version"].is_object(),
        "format version must be explicit"
    );

    assert_eq!(envelope["payload"], payload);
}

#[test]
fn encoder_is_deterministic_for_equivalent_payloads() {
    let first: Value =
        serde_json::from_str(r#"{"z":9,"a":{"y":2,"b":1}}"#)
            .expect("valid JSON");

    let second: Value =
        serde_json::from_str(r#"{"a":{"b":1,"y":2},"z":9}"#)
            .expect("valid JSON");

    let first_bytes = encode_document(&first)
        .expect("first payload should encode")
        .into_bytes();

    let second_bytes = encode_document(&second)
        .expect("second payload should encode")
        .into_bytes();

    assert_eq!(first_bytes, second_bytes);
}

#[test]
fn encoder_preserves_ordered_payload_sequences() {
    let first = json!({"operations": ["prepare", "measure", "return"]});
    let second = json!({"operations": ["measure", "prepare", "return"]});

    let first_bytes = encode_document(&first)
        .expect("first payload should encode")
        .into_bytes();

    let second_bytes = encode_document(&second)
        .expect("second payload should encode")
        .into_bytes();

    assert_ne!(first_bytes, second_bytes);
}

#[test]
fn encoder_obeys_exact_and_exceeded_output_budgets() {
    let payload = example_payload();

    let unrestricted = encode_document(&payload)
        .expect("payload should encode");

    let exact_limit = unrestricted.len();

    let exact_options = EncodeOptions::default()
        .with_max_output_bytes(Some(exact_limit));

    let exact = encode_document_with_options(&payload, &exact_options)
        .expect("artifact fitting the exact budget should encode");

    assert_eq!(exact.len(), exact_limit);

    let insufficient_options = EncodeOptions::default()
        .with_max_output_bytes(Some(
            exact_limit.saturating_sub(1),
        ));

    let error = encode_document_with_options(
        &payload,
        &insufficient_options,
    )
    .expect_err("undersized output budget must fail");

    assert!(
        matches!(error, EncodeError::OutputBudgetExceeded { .. }),
        "expected OutputBudgetExceeded, got {error:?}"
    );
}

#[test]
fn encoder_propagates_writer_failures() {
    let payload = example_payload();
    let mut writer = FailingWriter;

    let error = encode_document_to_writer(&payload, &mut writer)
        .expect_err("writer failure must propagate");

    assert!(
        matches!(error, EncodeError::Io(_)),
        "expected I/O error, got {error:?}"
    );
}

#[test]
fn bounded_writer_encoding_does_not_publish_an_artifact_on_failure() {
    let payload = example_payload();

    let options = EncodeOptions::default()
        .with_max_output_bytes(Some(1));

    let mut destination = Vec::new();

    let error = encode_document_to_writer_with_options(
        &payload,
        &options,
        &mut destination,
    )
    .expect_err("insufficient budget must fail");

    assert!(matches!(
        error,
        EncodeError::OutputBudgetExceeded { .. }
    ));

    assert!(
        destination.is_empty(),
        "the encoder must stage the artifact before writing"
    );
}

// ---------------------------------------------------------------------------
// Decoding and round trips
// ---------------------------------------------------------------------------

#[test]
fn encode_decode_round_trip_preserves_payload_meaning() {
    let payload = example_payload();

    let artifact = encode_document(&payload)
        .expect("payload should encode");

    let decoded = decode_document(artifact.as_bytes())
        .expect("encoded artifact should decode");

    assert_eq!(decoded.payload(), &payload);
    assert_eq!(
        decoded.schema_id().as_str(),
        DOCUMENT_SCHEMA_ID
    );
    assert_eq!(
        decoded.schema_version(),
        CURRENT_SCHEMA_VERSION
    );
}

#[test]
fn reader_and_slice_decoding_agree() {
    let payload = example_payload();
    let document = make_document(&payload);

    let from_slice = decode_document(&document)
        .expect("slice decoding should succeed");

    let mut reader = Cursor::new(document.as_slice());

    let from_reader = decode_document_from_reader(&mut reader)
        .expect("reader decoding should succeed");

    assert_eq!(from_slice, from_reader);
}

#[test]
fn decoder_accepts_an_input_exactly_at_its_byte_budget() {
    let document = make_document(&example_payload());

    let options = DecodeOptions::default()
        .with_max_input_bytes(Some(document.len()));

    let decoded = decode_document_with_options(&document, &options)
        .expect("input at the exact limit should be accepted");

    assert_eq!(decoded.payload(), &example_payload());
}

#[test]
fn decoder_rejects_an_input_over_its_byte_budget() {
    let document = make_document(&example_payload());

    let options = DecodeOptions::default()
        .with_max_input_bytes(Some(
            document.len().saturating_sub(1),
        ));

    let error = decode_document_with_options(&document, &options)
        .expect_err("input one byte over the limit must fail");

    assert!(matches!(
        error,
        DecodeError::InputBudgetExceeded { .. }
    ));
}

#[test]
fn reader_decoder_enforces_the_input_budget() {
    let document = make_document(&example_payload());

    let options = DecodeOptions::default()
        .with_max_input_bytes(Some(
            document.len().saturating_sub(1),
        ));

    let mut reader = Cursor::new(document);

    let error = decode_document_from_reader_with_options(
        &mut reader,
        &options,
    )
    .expect_err("reader must not bypass the input budget");

    assert!(matches!(
        error,
        DecodeError::InputBudgetExceeded { .. }
    ));
}

#[test]
fn decoder_propagates_reader_failures() {
    let mut reader = FailingReader;

    let error = decode_document_from_reader(&mut reader)
        .expect_err("reader I/O failures must propagate");

    assert!(
        matches!(error, DecodeError::Io(_)),
        "expected reader I/O error, got {error:?}"
    );
}

#[test]
fn decoder_rejects_malformed_json() {
    let error = decode_document(b"{not-json")
        .expect_err("malformed JSON must be rejected");

    assert!(
        matches!(error, DecodeError::Json(_)),
        "expected JSON error, got {error:?}"
    );
}

#[test]
fn decoder_rejects_trailing_non_whitespace_data() {
    let mut document = make_document(&json!({"value": 1}));
    document.extend_from_slice(b" trailing");

    assert!(
        decode_document(&document).is_err(),
        "trailing non-whitespace must not be silently accepted"
    );
}

#[test]
fn decoder_rejects_unknown_envelope_fields() {
    let document = make_document(&json!({"value": 1}));

    let modified = rewrite_document(&document, |envelope| {
        envelope["unexpected_extension"] = json!(true);
    });

    let error = decode_document(&modified)
        .expect_err("unknown envelope fields must be rejected");

    assert!(matches!(error, DecodeError::Json(_)));
}

#[test]
fn decoder_rejects_an_unexpected_schema_identity() {
    let document = make_document(&json!({"value": 1}));

    let modified = rewrite_document(&document, |envelope| {
        envelope["schema_id"] =
            json!("zamani.zuir.serialization.unrecognized");
    });

    let error = decode_document(&modified)
        .expect_err("unexpected schema must be rejected");

    assert!(
        matches!(error, DecodeError::UnexpectedSchema { .. }),
        "expected schema mismatch, got {error:?}"
    );
}

#[test]
fn decoder_rejects_an_unsupported_format_version() {
    let document = make_document(&json!({"value": 1}));

    let modified = rewrite_document(&document, |envelope| {
        envelope["format_version"]["major"] = json!(u32::MAX);
    });

    let error = decode_document(&modified)
        .expect_err("unsupported format version must be rejected");

    assert!(matches!(
        error,
        DecodeError::UnsupportedFormatVersion { .. }
    ));
}

#[test]
fn decoder_rejects_an_incompatible_schema_major_version() {
    let document = make_document(&json!({"value": 1}));

    let modified = rewrite_document(&document, |envelope| {
        envelope["schema_version"]["major"] = json!(u32::MAX);
    });

    let error = decode_document(&modified)
        .expect_err("incompatible schema version must be rejected");

    assert!(matches!(
        error,
        DecodeError::IncompatibleSchemaVersion { .. }
    ));
}

#[test]
fn decoder_preserves_payload_array_order() {
    let payload = json!({
        "operations": [
            "prepare",
            "entangle",
            "measure",
            "return"
        ]
    });

    let document = make_document(&payload);

    let decoded = decode_document(&document)
        .expect("document should decode");

    assert_eq!(
        decoded.payload()["operations"],
        json!([
            "prepare",
            "entangle",
            "measure",
            "return"
        ])
    );
}

// ---------------------------------------------------------------------------
// Schema and payload validation
// ---------------------------------------------------------------------------

#[test]
fn validation_accepts_a_payload_matching_its_schema() {
    let schema = schema_with_fields(
        vec![
            field(
                1,
                "name",
                WireKind::String,
                FieldCardinality::Required,
            ),
            field(
                2,
                "enabled",
                WireKind::Bool,
                FieldCardinality::Optional,
            ),
        ],
        UnknownFieldPolicy::Reject,
    );

    let payload = json!({
        "name": "portable-operation",
        "enabled": true
    });

    let report = validate_payload_unbounded(&payload, &schema)
        .expect("payload should satisfy its schema");

    assert!(report.nodes_visited > 0);
    assert_eq!(report.unknown_fields, 0);
}

#[test]
fn validation_rejects_a_missing_required_field() {
    let schema = schema_with_fields(
        vec![field(
            1,
            "required",
            WireKind::String,
            FieldCardinality::Required,
        )],
        UnknownFieldPolicy::Reject,
    );

    let error = validate_payload_unbounded(&json!({}), &schema)
        .expect_err("required field must be present");

    assert!(matches!(
        error,
        ValidationError::MissingRequiredField { .. }
    ));
}

#[test]
fn validation_rejects_an_unknown_field_when_policy_requires_it() {
    let schema = schema_with_fields(
        vec![],
        UnknownFieldPolicy::Reject,
    );

    let error = validate_payload_unbounded(
        &json!({"unknown": "must not be silently ignored"}),
        &schema,
    )
    .expect_err("unknown field must be rejected");

    assert!(matches!(
        error,
        ValidationError::UnknownField { .. }
    ));
}

#[test]
fn validation_rejects_a_payload_with_the_wrong_wire_kind() {
    let schema = schema_with_fields(
        vec![field(
            1,
            "enabled",
            WireKind::Bool,
            FieldCardinality::Required,
        )],
        UnknownFieldPolicy::Reject,
    );

    let error = validate_payload_unbounded(
        &json!({"enabled": "true"}),
        &schema,
    )
    .expect_err("string is not a boolean");

    assert!(matches!(
        error,
        ValidationError::WrongFieldKind { .. }
    ));
}

#[test]
fn validation_enforces_configured_node_budgets() {
    let schema = schema_with_fields(
        vec![],
        UnknownFieldPolicy::Preserve,
    );

    let payload = json!({
        "values": [1, 2, 3, 4, 5]
    });

    let limits = ValidationLimits::unbounded()
        .with_max_nodes(2);

    let error = validate_payload(&payload, &schema, &limits)
        .expect_err("node budget should be enforced");

    assert!(matches!(
        error,
        ValidationError::BudgetExceeded { .. }
    ));
}

#[test]
fn validation_does_not_confuse_unknown_fields_with_known_semantics() {
    let schema = schema_with_fields(
        vec![field(
            1,
            "name",
            WireKind::String,
            FieldCardinality::Required,
        )],
        UnknownFieldPolicy::Preserve,
    );

    let payload = json!({
        "name": "example",
        "future_extension": {
            "opaque": true,
            "version": 7
        }
    });

    let report = validate_payload_unbounded(&payload, &schema)
        .expect("preserved extension data should be accepted");

    assert_eq!(report.unknown_fields, 1);
    assert_eq!(
        payload["future_extension"]["opaque"],
        json!(true)
    );
}

// ---------------------------------------------------------------------------
// Dynamically sized payload regression
// ---------------------------------------------------------------------------

#[test]
fn dynamically_sized_payload_round_trips_without_fixed_entity_limits() {
    // This is a finite regression workload, not a universal size limit.
    // The number of elements is derived from the test workload and stored in
    // a dynamically sized Vec rather than a fixed-size machine-oriented array.
    let operation_count = 4096_usize;

    let operations: Vec<Value> = (0..operation_count)
        .map(|index| {
            json!({
                "ordinal": index,
                "dialect": "extension.stress",
                "operation": "extension.stress.process",
                "attributes": {
                    "logical_index": index,
                    "enabled": index % 2 == 0
                }
            })
        })
        .collect();

    let payload = json!({
        "module": "stress",
        "operations": operations
    });

    let first = encode_document(&payload)
        .expect("finite stress payload should encode");

    let decoded = decode_document(first.as_bytes())
        .expect("finite stress payload should decode");

    assert_eq!(decoded.payload(), &payload);

    let second = encode_document(decoded.payload())
        .expect("decoded payload should re-encode");

    assert_eq!(
        first.as_bytes(),
        second.as_bytes(),
        "re-encoding must be deterministic"
    );
    assert_eq!(
        decoded.payload()["operations"]
            .as_array()
            .expect("operations must remain an array")
            .len(),
        operation_count
    );
}

// ---------------------------------------------------------------------------
// Portable contract regression
// ---------------------------------------------------------------------------

#[test]
fn serialization_round_trip_does_not_require_a_concrete_hardware_target() {
    let payload = json!({
        "resource_requirements": {
            "qubits": {
                "minimum": "symbolic:n",
                "kind": "requirement"
            },
            "memory": {
                "minimum": "symbolic:required_memory",
                "kind": "requirement"
            },
            "capabilities": [
                "quantum.measurement",
                "tensor.compute"
            ]
        },
        "logical_computation": {
            "dialect": "core",
            "operation": "example.portable_computation",
            "physical_device": null,
            "physical_allocation": null
        }
    });

    let document = make_document(&payload);
    let decoded = decode_document(&document)
        .expect("portable payload should decode");

    assert_eq!(decoded.payload(), &payload);
    assert!(decoded.payload()["logical_computation"]
        ["physical_device"]
        .is_null());
    assert!(decoded.payload()["resource_requirements"]
        ["qubits"]["minimum"]
        .is_string());
}
