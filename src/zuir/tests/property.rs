#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Property and Invariant Tests
//!
//! Path: `src/zuir/tests/property.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: Unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Exercise general properties of the existing ZUIR public contracts across
//! generated inputs. These tests complement example-based unit tests and
//! cross-module integration tests.
//!
//! The suite focuses on:
//! - deterministic, domain-separated fingerprints;
//! - unambiguous fingerprint field framing;
//! - stable sequence and ordered-map encoding;
//! - typed identifier validity, round trips, and boundary values;
//! - canonical serialization repeatability and ordering;
//! - preservation of meaningful sequence order;
//! - deterministic behavior over reproducible generated cases.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/fingerprint/mod.rs`
//! - `src/zuir/fingerprint/stable_hash.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/serialization/canonical.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/tests/determinism.rs`
//! - `src/zuir/tests/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//!
//! OWNS
//!
//! - Reproducible generated-case test inputs.
//! - Cross-input property assertions for the existing public APIs.
//! - Regression tests for ordering, identity, and deterministic encoding.
//!
//! DOES_NOT_OWN
//!
//! - Production fingerprint or serialization implementations.
//! - IR construction, verification, or semantic equivalence.
//! - Source parsing, grammar rules, AST construction, or name resolution.
//! - Type inference, dialect registration, or lowering.
//! - Cache storage, cache invalidation, or runtime behavior.
//! - Hardware discovery, physical resource allocation, or scheduling.
//!
//! DEPENDENCIES
//!
//! - `crate::zuir::fingerprint`
//! - `crate::zuir::ids`
//! - `crate::zuir::serialization::canonical`
//! - Existing `serde_json` dependency.
//!
//! This file deliberately does not require `proptest`, `quickcheck`, a
//! random-number generator, or a new production dependency. Generated cases
//! are deterministic, so failures are reproducible without recording seeds.
//!
//! SCALABILITY AND POCO-REAF
//!
//! No test establishes a universal maximum program size, operation count,
//! qubit count, tensor rank, device count, node count, or register width.
//!
//! Finite test workloads are test-runner inputs, not language or IR limits.
//! Test sizes must remain appropriate for the CI resource budget. Larger
//! stress workloads belong in the dedicated stress-test suite.
//!
//! Finite integer representations are tested at their boundaries.
//! Exhaustion, overflow, and invalid input must be reported rather than
//! silently wrapping, truncating, or reusing identities.
//!
//! DETERMINISM
//!
//! Identical canonical inputs and configuration must produce identical
//! fingerprints or serialized bytes. Different domains, meaningful field
//! values, or semantically ordered sequences must remain distinguishable
//! where the relevant API contract requires it.
//!
//! A fingerprint is not proof of semantic equivalence. These tests validate
//! encoding contracts, not arbitrary program equivalence.
//!
//! INTEGRATION
//!
//! Register this module exactly once in the repository's canonical ZUIR
//! test module. Do not introduce a second ZUIR root or duplicate an existing
//! test-module declaration.
//!
//! The repository's `src/zuir/mod.rs` and test-module wiring must be repaired
//! or confirmed before treating this file as integrated.
//!
//! FREEZE CRITERIA
//!
//! - Every imported API exists and is public in the integration revision.
//! - This module is registered exactly once.
//! - All tests pass on the declared Rust baseline.
//! - The tests use production implementations, not duplicate algorithms.
//! - No unsafe code, fixed hardware limits, or hidden semantic assumptions
//!   are introduced.
//! - Formatting, linting, and repository-level tests pass.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::zuir::fingerprint::{
    encode_sequence,
    encode_string_map,
    fingerprint,
    Fingerprint,
    FingerprintError,
    StableHashBuilder,
};
use crate::zuir::ids::{
    BlockId,
    FunctionId,
    OperationId,
    ProgramId,
    ValueId,
};
use crate::zuir::serialization::canonical::{
    write_canonical_json,
    CanonicalError,
    CanonicalOptions,
};

// -----------------------------------------------------------------------------
// Shared helpers
// -----------------------------------------------------------------------------

/// Construct a digest through the canonical production fingerprint API.
fn digest<T: crate::zuir::fingerprint::StableEncode + ?Sized>(
    domain: &str,
    value: &T,
) -> Result<Fingerprint, FingerprintError> {
    fingerprint(domain, value)
}

/// Serialize a value using the production canonical JSON writer.
///
/// `CanonicalOptions::unbounded()` selects the serializer's explicit
/// unbounded option. It does not imply that process memory or execution time
/// is infinite.
fn canonical_bytes(value: &Value) -> Result<Vec<u8>, CanonicalError> {
    let mut output = Vec::new();

    write_canonical_json(
        value,
        &mut output,
        CanonicalOptions::unbounded(),
    )?;

    Ok(output)
}

/// Construct an object in the supplied insertion order.
///
/// Canonical output must not depend on object construction history.
fn object_from_entries(entries: &[(&str, Value)]) -> Value {
    let mut object = Map::new();

    for (key, value) in entries {
        object.insert((*key).to_owned(), value.clone());
    }

    Value::Object(object)
}

/// Generate reproducible, nonzero numeric cases without randomness.
///
/// The generated sequence is test data only. It does not define an IR limit,
/// an identifier allocation policy, or a hardware capacity.
fn generated_nonzero_values(count: usize) -> Vec<u128> {
    (1..=count)
        .map(|value| value as u128)
        .collect()
}

// -----------------------------------------------------------------------------
// Fingerprint properties
// -----------------------------------------------------------------------------

#[test]
fn identical_inputs_always_produce_identical_fingerprints() {
    let cases = [
        "",
        "zamani",
        "classical",
        "quantum",
        "hdl",
        "tensor",
        "distributed",
        "networking",
        "cryptography",
        "future::dialect::operation",
        "λειτουργία",
        "量子计算",
        "مرحبا",
    ];

    for value in cases {
        let first = digest("zuir.property.repeatability", value)
            .expect("valid input must be fingerprintable");

        let second = digest("zuir.property.repeatability", value)
            .expect("identical input must be fingerprintable");

        assert_eq!(
            first, second,
            "fingerprint changed for input {value:?}"
        );
    }
}

#[test]
fn generated_string_inputs_are_repeatable() {
    let alphabet = [
        'a', 'Z', '0', '_', ':', '.', '-', 'λ', '量', '\n', '\t', ' ',
    ];

    for case_index in 0_u64..256 {
        let mut value = String::new();
        let mut state = case_index.wrapping_add(1);

        for _ in 0..(case_index as usize % 37) {
            // Deterministic test-data generation. The generator does not
            // participate in production compilation or fingerprinting.
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);

            let index = (state as usize) % alphabet.len();
            value.push(alphabet[index]);
        }

        let first = digest("zuir.property.generated-string", &value)
            .expect("generated string must be fingerprintable");

        let second = digest("zuir.property.generated-string", &value)
            .expect("generated string must be fingerprintable");

        assert_eq!(first, second);
    }
}

#[test]
fn fingerprint_domain_separates_equal_content() {
    let value = "same semantic payload";

    let classical = digest("zamani.classical.v1", value)
        .expect("valid domain");

    let quantum = digest("zamani.quantum.v1", value)
        .expect("valid domain");

    let hardware = digest("zamani.hdl.v1", value)
        .expect("valid domain");

    assert_ne!(classical, quantum);
    assert_ne!(classical, hardware);
    assert_ne!(quantum, hardware);
}

#[test]
fn changing_content_changes_the_expected_fingerprint() {
    let cases = [
        ("", "a"),
        ("a", "b"),
        ("add", "sub"),
        ("quantum.measure", "quantum.reset"),
        ("hdl.signal", "hdl.clock"),
        ("tensor.shape", "tensor.layout"),
    ];

    for (left, right) in cases {
        let left_digest = digest("zuir.property.content", left)
            .expect("left value must be fingerprintable");

        let right_digest = digest("zuir.property.content", right)
            .expect("right value must be fingerprintable");

        assert_ne!(
            left_digest, right_digest,
            "distinct test inputs unexpectedly produced equal digests"
        );
    }
}

#[test]
fn fingerprinting_rejects_invalid_domains() {
    for domain in ["", "\n", "invalid\tdomain", "domain\rname"] {
        let result = digest(domain, "payload");

        assert!(
            matches!(result, Err(FingerprintError::InvalidDomain)),
            "invalid domain should be rejected: {domain:?}"
        );
    }
}

#[test]
fn fingerprint_encoding_distinguishes_field_boundaries() {
    let mut first = StableHashBuilder::new("zuir.property.framing")
        .expect("valid domain");

    first
        .field_str("left", "ab")
        .expect("first field must encode");

    first
        .field_str("right", "c")
        .expect("second field must encode");

    let mut second = StableHashBuilder::new("zuir.property.framing")
        .expect("valid domain");

    second
        .field_str("left", "a")
        .expect("first field must encode");

    second
        .field_str("right", "bc")
        .expect("second field must encode");

    assert_ne!(
        first.finish(),
        second.finish(),
        "field boundaries must not collapse distinct field sequences"
    );
}

#[test]
fn fingerprint_encoding_distinguishes_field_names() {
    let mut first = StableHashBuilder::new("zuir.property.field-name")
        .expect("valid domain");

    first
        .field_str("operation", "compute")
        .expect("field must encode");

    let mut second = StableHashBuilder::new("zuir.property.field-name")
        .expect("valid domain");

    second
        .field_str("description", "compute")
        .expect("field must encode");

    assert_ne!(first.finish(), second.finish());
}

#[test]
fn integer_fingerprints_are_repeatable_at_boundaries() {
    let cases = [
        0_u128,
        1,
        u8::MAX as u128,
        u32::MAX as u128,
        u64::MAX as u128,
        u128::MAX,
    ];

    for value in cases {
        let first = digest("zuir.property.integer", &value)
            .expect("integer must be fingerprintable");

        let second = digest("zuir.property.integer", &value)
            .expect("same integer must be fingerprintable");

        assert_eq!(first, second);
    }
}

#[test]
fn fingerprint_output_has_the_declared_digest_size() {
    use crate::zuir::fingerprint::FINGERPRINT_BYTES;

    for value in ["", "a", "Zamani", "quantum.measure", "未来の操作"] {
        let result = digest("zuir.property.digest-size", value)
            .expect("valid input");

        assert_eq!(result.as_bytes().len(), FINGERPRINT_BYTES);
        assert_eq!(result.to_hex().len(), FINGERPRINT_BYTES * 2);
    }
}

// -----------------------------------------------------------------------------
// Ordered sequence and map properties
// -----------------------------------------------------------------------------

#[test]
fn sequence_encoding_is_repeatable() {
    let cases: Vec<Vec<u64>> = vec![
        vec![],
        vec![0],
        vec![1, 2],
        vec![3, 2, 1],
        vec![u64::MAX, 0, 1],
        (0_u64..128).collect(),
    ];

    for values in cases {
        let mut first =
            StableHashBuilder::new("zuir.property.sequence")
                .expect("valid domain");

        encode_sequence("items", &values, &mut first)
            .expect("sequence must encode");

        let mut second =
            StableHashBuilder::new("zuir.property.sequence")
                .expect("valid domain");

        encode_sequence("items", &values, &mut second)
            .expect("same sequence must encode");

        assert_eq!(first.finish(), second.finish());
    }
}

#[test]
fn sequence_encoding_preserves_order() {
    let forward = [10_u64, 20, 30];
    let reverse = [30_u64, 20, 10];

    let mut first =
        StableHashBuilder::new("zuir.property.ordered-sequence")
            .expect("valid domain");

    encode_sequence("operands", &forward, &mut first)
        .expect("forward sequence must encode");

    let mut second =
        StableHashBuilder::new("zuir.property.ordered-sequence")
            .expect("valid domain");

    encode_sequence("operands", &reverse, &mut second)
        .expect("reverse sequence must encode");

    assert_ne!(
        first.finish(),
        second.finish(),
        "operand or pipeline order must not be discarded"
    );
}

#[test]
fn sequence_encoding_preserves_length_and_empty_sequences() {
    let empty: [u64; 0] = [];
    let one = [0_u64];
    let two = [0_u64, 0];

    let encode = |values: &[u64]| {
        let mut builder =
            StableHashBuilder::new("zuir.property.sequence-length")
                .expect("valid domain");

        encode_sequence("values", values, &mut builder)
            .expect("sequence must encode");

        builder.finish()
    };

    let empty_digest = encode(&empty);
    let one_digest = encode(&one);
    let two_digest = encode(&two);

    assert_ne!(empty_digest, one_digest);
    assert_ne!(one_digest, two_digest);
    assert_ne!(empty_digest, two_digest);
}

#[test]
fn ordered_map_encoding_is_independent_of_insertion_order() {
    let mut first = BTreeMap::new();
    first.insert("zeta".to_owned(), 3_u64);
    first.insert("alpha".to_owned(), 1_u64);
    first.insert("middle".to_owned(), 2_u64);

    let mut second = BTreeMap::new();
    second.insert("middle".to_owned(), 2_u64);
    second.insert("alpha".to_owned(), 1_u64);
    second.insert("zeta".to_owned(), 3_u64);

    let encode = |values: &BTreeMap<String, u64>| {
        let mut builder =
            StableHashBuilder::new("zuir.property.ordered-map")
                .expect("valid domain");

        encode_string_map("attributes", values, &mut builder)
            .expect("map must encode");

        builder.finish()
    };

    assert_eq!(encode(&first), encode(&second));
}

#[test]
fn ordered_map_encoding_detects_value_changes() {
    let mut first = BTreeMap::new();
    first.insert("precision".to_owned(), 32_u64);

    let mut second = BTreeMap::new();
    second.insert("precision".to_owned(), 64_u64);

    let encode = |values: &BTreeMap<String, u64>| {
        let mut builder =
            StableHashBuilder::new("zuir.property.map-content")
                .expect("valid domain");

        encode_string_map("attributes", values, &mut builder)
            .expect("map must encode");

        builder.finish()
    };

    assert_ne!(encode(&first), encode(&second));
}

// -----------------------------------------------------------------------------
// Typed identifier properties
// -----------------------------------------------------------------------------

#[test]
fn typed_identifiers_round_trip_nonzero_values() {
    for value in generated_nonzero_values(512) {
        let program = ProgramId::new(value)
            .expect("generated identifier must be nonzero");

        assert_eq!(program.get(), value);
        assert_eq!(u128::from(program), value);

        let encoded = serde_json::to_string(&program)
            .expect("program identifier must serialize");

        let decoded: ProgramId = serde_json::from_str(&encoded)
            .expect("valid program identifier must deserialize");

        assert_eq!(decoded, program);
    }
}

#[test]
fn typed_identifiers_accept_the_maximum_nonzero_value() {
    let program = ProgramId::new(u128::MAX)
        .expect("u128::MAX is nonzero");

    let value = ValueId::new(u128::MAX)
        .expect("u128::MAX is nonzero");

    assert_eq!(program.get(), u128::MAX);
    assert_eq!(value.get(), u128::MAX);
}

#[test]
fn typed_identifiers_reject_zero() {
    assert!(ProgramId::new(0).is_err());
    assert!(BlockId::new(0).is_err());
    assert!(FunctionId::new(0).is_err());
    assert!(OperationId::new(0).is_err());
    assert!(ValueId::new(0).is_err());
}

#[test]
fn identifier_categories_keep_their_type_identity() {
    let program = ProgramId::new(7).expect("nonzero");
    let block = BlockId::new(7).expect("nonzero");
    let operation = OperationId::new(7).expect("nonzero");
    let value = ValueId::new(7).expect("nonzero");

    // These IDs share a numeric payload but belong to different Rust types.
    // No implicit cross-category conversion is introduced by this test.
    assert_eq!(program.get(), block.get());
    assert_eq!(program.get(), operation.get());
    assert_eq!(program.get(), value.get());

    assert_ne!(format!("{program:?}"), format!("{block:?}"));
    assert_ne!(format!("{program:?}"), format!("{operation:?}"));
    assert_ne!(format!("{program:?}"), format!("{value:?}"));
}

#[test]
fn generated_identifier_values_are_orderable_and_hashable() {
    use std::collections::BTreeSet;
    use std::collections::HashSet;

    let ids: Vec<OperationId> = generated_nonzero_values(256)
        .into_iter()
        .map(|value| OperationId::new(value).expect("nonzero"))
        .collect();

    let ordered: BTreeSet<_> = ids.iter().copied().collect();
    let hashed: HashSet<_> = ids.iter().copied().collect();

    assert_eq!(ordered.len(), ids.len());
    assert_eq!(hashed.len(), ids.len());

    for pair in ids.windows(2) {
        assert!(
            pair[0] < pair[1],
            "generated identifiers should retain numeric ordering"
        );
    }
}

// -----------------------------------------------------------------------------
// Canonical serialization properties
// -----------------------------------------------------------------------------

#[test]
fn canonical_serialization_is_repeatable() {
    let values = [
        serde_json::json!(null),
        serde_json::json!(true),
        serde_json::json!(0),
        serde_json::json!(""),
        serde_json::json!("Zamani"),
        serde_json::json!([]),
        serde_json::json!({}),
        serde_json::json!({
            "dialect": "quantum",
            "operation": "measure",
            "effects": ["measurement", "classical-result"]
        }),
    ];

    for value in values {
        let first = canonical_bytes(&value)
            .expect("first serialization must succeed");

        let second = canonical_bytes(&value)
            .expect("second serialization must succeed");

        assert_eq!(first, second);
    }
}

#[test]
fn canonical_serialization_is_independent_of_object_insertion_order() {
    let first = object_from_entries(&[
        ("zeta", Value::from(3)),
        ("alpha", Value::from(1)),
        ("middle", Value::from(2)),
    ]);

    let second = object_from_entries(&[
        ("middle", Value::from(2)),
        ("zeta", Value::from(3)),
        ("alpha", Value::from(1)),
    ]);

    assert_eq!(
        canonical_bytes(&first).expect("first object must serialize"),
        canonical_bytes(&second).expect("second object must serialize")
    );
}

#[test]
fn canonical_serialization_preserves_array_order() {
    let first = serde_json::json!({
        "pipeline": ["core.canonicalize", "core.simplify"]
    });

    let second = serde_json::json!({
        "pipeline": ["core.simplify", "core.canonicalize"]
    });

    assert_ne!(
        canonical_bytes(&first).expect("first value must serialize"),
        canonical_bytes(&second).expect("second value must serialize")
    );
}

#[test]
fn canonical_serialization_round_trips_json_values() {
    let values = [
        serde_json::json!(null),
        serde_json::json!(false),
        serde_json::json!(42),
        serde_json::json!("unicode: λ, 量子, مرحبا"),
        serde_json::json!([1, "two", true, null]),
        serde_json::json!({
            "empty": "",
            "nested": {
                "effects": ["memory.read", "memory.write"],
                "verified": true
            }
        }),
    ];

    for value in values {
        let bytes = canonical_bytes(&value)
            .expect("canonical serialization must succeed");

        let text = std::str::from_utf8(&bytes)
            .expect("canonical JSON must be valid UTF-8");

        let decoded: Value = serde_json::from_str(text)
            .expect("canonical output must be valid JSON");

        assert_eq!(decoded, value);
    }
}

#[test]
fn canonical_serialization_preserves_escaped_content() {
    let value = serde_json::json!({
        "text": "quote: \" slash: \\ newline:\n tab:\t null:\u{0000}"
    });

    let bytes = canonical_bytes(&value)
        .expect("control characters must be serialized safely");

    let text = std::str::from_utf8(&bytes)
        .expect("canonical output must be UTF-8");

    let decoded: Value = serde_json::from_str(text)
        .expect("escaped canonical JSON must parse");

    assert_eq!(decoded, value);
}

#[test]
fn canonical_serialization_handles_generated_nested_values() {
    for case_index in 0_u64..128 {
        let value = serde_json::json!({
            "case": case_index,
            "domain": format!("extension::domain::{case_index}"),
            "operations": [
                {
                    "name": "core.compute",
                    "ordinal": case_index
                },
                {
                    "name": "future.operation",
                    "ordinal": case_index.saturating_add(1)
                }
            ],
            "constraints": {
                "shape": format!("dimension_{case_index}"),
                "target_independent": true
            }
        });

        let first = canonical_bytes(&value)
            .expect("generated nested value must serialize");

        let second = canonical_bytes(&value)
            .expect("identical nested value must serialize");

        assert_eq!(first, second);

        let decoded: Value = serde_json::from_slice(&first)
            .expect("canonical bytes must be valid JSON");

        assert_eq!(decoded, value);
    }
}

// -----------------------------------------------------------------------------
// Cross-component properties
// -----------------------------------------------------------------------------

#[test]
fn canonical_bytes_and_fingerprints_are_repeatable_together() {
    for case_index in 0_u64..128 {
        let value = serde_json::json!({
            "schema": 1,
            "case": case_index,
            "domain": "zuir.property.cross-component",
            "operation": format!("extension.operation.{case_index}"),
            "attributes": {
                "deterministic": true,
                "portable": true
            }
        });

        let bytes = canonical_bytes(&value)
            .expect("value must serialize canonically");

        let first = digest(
            "zuir.property.canonical-document",
            bytes.as_slice(),
        )
        .expect("canonical bytes must be fingerprintable");

        let second = digest(
            "zuir.property.canonical-document",
            bytes.as_slice(),
        )
        .expect("identical canonical bytes must be fingerprintable");

        assert_eq!(first, second);

        let decoded: Value = serde_json::from_slice(&bytes)
            .expect("canonical document must decode");

        assert_eq!(decoded, value);
    }
}

#[test]
fn canonical_document_changes_are_visible_to_fingerprinting() {
    let first = serde_json::json!({
        "operation": "core.add",
        "operands": ["lhs", "rhs"]
    });

    let second = serde_json::json!({
        "operation": "core.sub",
        "operands": ["lhs", "rhs"]
    });

    let first_bytes = canonical_bytes(&first)
        .expect("first document must serialize");

    let second_bytes = canonical_bytes(&second)
        .expect("second document must serialize");

    let first_fingerprint = digest(
        "zuir.property.document",
        first_bytes.as_slice(),
    )
    .expect("first document must fingerprint");

    let second_fingerprint = digest(
        "zuir.property.document",
        second_bytes.as_slice(),
    )
    .expect("second document must fingerprint");

    assert_ne!(first_fingerprint, second_fingerprint);
}

#[test]
fn fingerprint_domains_do_not_replace_schema_versioning() {
    let content = "same canonical representation";

    let version_one = digest("zuir.document.schema.v1", content)
        .expect("valid domain");

    let version_two = digest("zuir.document.schema.v2", content)
        .expect("valid domain");

    assert_ne!(
        version_one, version_two,
        "schema-specific fingerprint domains must remain distinct"
    );
}