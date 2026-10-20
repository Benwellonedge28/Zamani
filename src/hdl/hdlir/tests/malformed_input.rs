
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Malformed-input and resilience tests for HDLIR artifact decoding.
//!
//! Path: `src/hdl/hdlir/tests/malformed_input.rs`
//! Rust edition: 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Exercise the existing HDLIR artifact decoder against malformed framing,
//! hostile length declarations, invalid metadata, unsupported required
//! definitions, corrupted payloads, truncation, and trailing data.
//!
//! # Authority
//!
//! These tests follow:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/serialization/format.rs`
//! - `src/hdl/hdlir/serialization/encode.rs`
//! - `src/hdl/hdlir/serialization/decode.rs`
//! - `src/hdl/hdlir/version.rs`
//!
//! The grammar owns source syntax, the frontend owns source-level semantics,
//! ZUIR owns the universal computational representation, and HDLIR owns the
//! hardware-domain representation. These tests exercise the artifact envelope;
//! they do not claim to verify a decoded HDLIR graph.
//!
//! # Scope
//!
//! The artifact decoder validates framing, metadata, configured compatibility,
//! declared payload integrity, and size budgets. Its payload remains opaque.
//! Graph-level invalid IDs, dangling references, and invalid operation
//! relationships must be tested through the canonical payload codec and
//! graph verifier once those APIs are implemented and registered.
//!
//! # Scalability and security
//!
//! No hardware-capacity ceilings are introduced. Numeric limits used below
//! are deliberately small, per-test decoder budgets—not language or hardware
//! limits. Hostile lengths are tested with explicit budgets before allocation.
//!
//! These tests use deterministic mutations, checked framing arithmetic,
//! standard-library safe Rust, and no pointer-derived identities.
//!
//! Passing these tests does not establish HDLIR graph validity, synthesis
//! correctness, target feasibility, authenticity, or physical safety.

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::hdl::hdlir::serialization::decode::{
    decode_artifact, DecodeError, DecodeOptions,
};
use crate::hdl::hdlir::serialization::encode::{
    encode_artifact, EncodeOptions, HDLIR_ARTIFACT_MAGIC, FRAME_PREFIX_LEN,
};
use crate::hdl::hdlir::serialization::format::{
    ArtifactFormat, DefinitionRequirement,
};
use crate::hdl::hdlir::version::HdlirVersion;

const TEST_SCHEMA: &str = "zamani.hdlir.schema.test";
const TEST_DIALECT: &str = "zamani.hdlir.dialect.test";
const TEST_FEATURE: &str = "zamani.hdlir.feature.test";

/// Encode a valid baseline artifact using the production encoder.
fn valid_artifact(payload: &[u8]) -> Vec<u8> {
    encode_artifact(
        &ArtifactFormat::new(),
        payload,
        EncodeOptions::new(),
    )
    .expect("baseline test artifact must encode")
}

/// Read an unsigned big-endian 64-bit integer from a known-valid test slice.
fn read_u64_be(bytes: &[u8]) -> u64 {
    let encoded: [u8; 8] = bytes
        .try_into()
        .expect("test framing field must contain eight bytes");

    u64::from_be_bytes(encoded)
}

/// Return the metadata header and payload of a complete, valid test artifact.
fn split_artifact(bytes: &[u8]) -> (&[u8], &[u8]) {
    assert!(bytes.len() >= FRAME_PREFIX_LEN);
    assert_eq!(&bytes[..8], HDLIR_ARTIFACT_MAGIC);

    let header_len = usize::try_from(read_u64_be(&bytes[8..16]))
        .expect("test header length must fit usize");
    let payload_len = usize::try_from(read_u64_be(&bytes[16..24]))
        .expect("test payload length must fit usize");

    let header_start = FRAME_PREFIX_LEN;
    let header_end = header_start
        .checked_add(header_len)
        .expect("test header end must be representable");
    let payload_end = header_end
        .checked_add(payload_len)
        .expect("test payload end must be representable");

    assert_eq!(
        payload_end,
        bytes.len(),
        "baseline artifact must contain exactly one complete frame"
    );

    (&bytes[header_start..header_end], &bytes[header_end..payload_end])
}

/// Frame caller-supplied metadata and payload bytes.
///
/// This helper intentionally does not validate the metadata. It permits tests
/// to supply malformed JSON or metadata that the production encoder correctly
/// refuses to construct.
fn raw_artifact(header: &[u8], payload: &[u8]) -> Vec<u8> {
    let header_len =
        u64::try_from(header.len()).expect("test header length must fit u64");
    let payload_len = u64::try_from(payload.len())
        .expect("test payload length must fit u64");

    let capacity = FRAME_PREFIX_LEN
        .checked_add(header.len())
        .and_then(|length| length.checked_add(payload.len()))
        .expect("test artifact length must be representable");

    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(capacity)
        .expect("small test artifact allocation must succeed");

    bytes.extend_from_slice(HDLIR_ARTIFACT_MAGIC);
    bytes.extend_from_slice(&header_len.to_be_bytes());
    bytes.extend_from_slice(&payload_len.to_be_bytes());
    bytes.extend_from_slice(header);
    bytes.extend_from_slice(payload);

    bytes
}

/// Replace the JSON metadata while preserving the original payload bytes.
fn rewrite_json_header(
    artifact: &[u8],
    update: impl FnOnce(&mut serde_json::Value),
) -> Vec<u8> {
    let (header, payload) = split_artifact(artifact);
    let mut metadata: serde_json::Value =
        serde_json::from_slice(header).expect("baseline metadata must be JSON");

    update(&mut metadata);

    let rewritten_header =
        serde_json::to_vec(&metadata).expect("test metadata must serialize");

    raw_artifact(&rewritten_header, payload)
}

/// Create options with explicit per-test budgets.
///
/// These values protect the tests from accidental large allocations. They do
/// not represent universal limits on HDL designs or serialized artifacts.
fn bounded_decode_options() -> DecodeOptions {
    DecodeOptions::new()
        .with_max_header_bytes(Some(4 * 1024))
        .with_max_payload_bytes(Some(4 * 1024))
        .with_max_artifact_bytes(Some(8 * 1024))
}

#[test]
fn rejects_invalid_magic() {
    let mut bytes = valid_artifact(b"test payload");
    bytes[0] ^= 1;

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::InvalidMagic)
    ));
}

#[test]
fn rejects_every_truncated_prefix_of_a_valid_artifact() {
    let bytes = valid_artifact(b"payload with more than one byte");

    for end in 0..bytes.len() {
        let result = decode_artifact(&bytes[..end], &bounded_decode_options());

        assert!(
            result.is_err(),
            "truncated artifact unexpectedly decoded at prefix length {end}"
        );
    }
}

#[test]
fn rejects_malformed_metadata_json() {
    let bytes = raw_artifact(b"{not valid JSON", b"payload");

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::InvalidMetadata(_))
    ));
}

#[test]
fn rejects_unknown_metadata_fields() {
    let baseline = valid_artifact(b"payload");

    let bytes = rewrite_json_header(&baseline, |metadata| {
        metadata
            .as_object_mut()
            .expect("baseline metadata must be an object")
            .insert(
                "unrecognized_test_field".to_owned(),
                serde_json::Value::Bool(true),
            );
    });

    // ArtifactFormat declares deny_unknown_fields: forward extensions must
    // not silently alter the meaning of an artifact this reader accepts.
    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::InvalidMetadata(_))
    ));
}

#[test]
fn rejects_an_unknown_artifact_format_identifier() {
    let baseline = valid_artifact(b"payload");

    let bytes = rewrite_json_header(&baseline, |metadata| {
        metadata["format_id"] = serde_json::Value::String(
            "zamani.hdlir.unknown".to_owned(),
        );
    });

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::InvalidFormat(_))
    ));
}

#[test]
fn rejects_an_unparseable_format_version() {
    let baseline = valid_artifact(b"payload");

    let bytes = rewrite_json_header(&baseline, |metadata| {
        metadata["format_version"] =
            serde_json::Value::String("not-a-version".to_owned());
    });

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::InvalidMetadata(_))
    ));
}

#[test]
fn rejects_an_incompatible_format_major_version() {
    let mut format = ArtifactFormat::new();
    format.set_format_version(HdlirVersion::new(2, 0, 0));

    let bytes = encode_artifact(&format, b"payload", EncodeOptions::new())
        .expect("test artifact with a valid version field must encode");

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::IncompatibleFormatVersion(_))
    ));
}

#[test]
fn rejects_malformed_integrity_metadata() {
    let baseline = valid_artifact(b"payload");

    let bytes = rewrite_json_header(&baseline, |metadata| {
        metadata["integrity"]["digest_hex"] =
            serde_json::Value::String("abc".to_owned());
    });

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::InvalidFormat(_))
    ));
}

#[test]
fn rejects_payload_corruption_when_integrity_is_declared() {
    let mut bytes = valid_artifact(b"payload whose integrity is checked");
    let last = bytes.len() - 1;
    bytes[last] ^= 1;

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::IntegrityMismatch { .. })
    ));
}

#[test]
fn rejects_artifacts_missing_required_integrity_metadata() {
    let bytes = encode_artifact(
        &ArtifactFormat::new(),
        b"payload",
        EncodeOptions::new().with_payload_integrity(false),
    )
    .expect("test artifact without integrity metadata must encode");

    let options =
        DecodeOptions::new().with_required_payload_integrity(true);

    assert!(matches!(
        decode_artifact(&bytes, &options),
        Err(DecodeError::IntegrityRequired)
    ));
}

#[test]
fn rejects_trailing_bytes_after_a_complete_artifact() {
    let mut bytes = valid_artifact(b"payload");
    bytes.extend_from_slice(b"unexpected trailing bytes");

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::TrailingData)
    ));
}

#[test]
fn rejects_a_declared_payload_longer_than_the_available_input() {
    let mut bytes = valid_artifact(b"payload");
    let declared_payload_len = read_u64_be(&bytes[16..24]);

    bytes[16..24].copy_from_slice(
        &declared_payload_len
            .checked_add(1)
            .expect("test payload length must be representable")
            .to_be_bytes(),
    );

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::Truncated { .. })
    ));
}

#[test]
fn rejects_a_header_that_exceeds_its_configured_budget_before_reading_it() {
    let bytes = raw_artifact(b"", b"");
    let mut hostile_prefix = bytes;
    hostile_prefix[8..16].copy_from_slice(&4096_u64.to_be_bytes());

    let options = DecodeOptions::new().with_max_header_bytes(Some(64));

    assert!(matches!(
        decode_artifact(&hostile_prefix, &options),
        Err(DecodeError::LimitExceeded {
            resource: "metadata header",
            ..
        })
    ));
}

#[test]
fn rejects_a_payload_that_exceeds_its_configured_budget_before_reading_it() {
    let bytes = raw_artifact(b"", b"");
    let mut hostile_prefix = bytes;
    hostile_prefix[16..24].copy_from_slice(&4096_u64.to_be_bytes());

    let options = DecodeOptions::new().with_max_payload_bytes(Some(64));

    assert!(matches!(
        decode_artifact(&hostile_prefix, &options),
        Err(DecodeError::LimitExceeded {
            resource: "payload",
            ..
        })
    ));
}

#[test]
fn rejects_an_artifact_that_exceeds_its_total_size_budget() {
    let bytes = valid_artifact(b"payload");
    let options = DecodeOptions::new().with_max_artifact_bytes(Some(1));

    assert!(matches!(
        decode_artifact(&bytes, &options),
        Err(DecodeError::LimitExceeded {
            resource: "complete artifact",
            ..
        })
    ));
}

#[test]
fn rejects_framing_length_arithmetic_overflow() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(HDLIR_ARTIFACT_MAGIC);
    bytes.extend_from_slice(&u64::MAX.to_be_bytes());
    bytes.extend_from_slice(&1_u64.to_be_bytes());

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::SizeOverflow)
    ));
}

#[test]
fn rejects_an_unregistered_required_feature() {
    let mut format = ArtifactFormat::new();
    format
        .require_feature(TEST_FEATURE)
        .expect("test feature identifier must be valid");

    let bytes = encode_artifact(&format, b"payload", EncodeOptions::new())
        .expect("artifact with a valid required feature must encode");

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::UnsupportedRequiredFeature(feature))
            if feature == TEST_FEATURE
    ));

    let options = DecodeOptions::new().with_supported_feature(TEST_FEATURE);

    assert!(decode_artifact(&bytes, &options).is_ok());
}

#[test]
fn rejects_an_unregistered_required_schema() {
    let mut format = ArtifactFormat::new();
    format
        .add_schema(
            DefinitionRequirement::new(
                TEST_SCHEMA,
                HdlirVersion::new(1, 0, 0),
                true,
            )
            .expect("test schema requirement must be valid"),
        )
        .expect("test schema requirement must be unique");

    let bytes = encode_artifact(&format, b"payload", EncodeOptions::new())
        .expect("artifact with a valid schema requirement must encode");

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::UnsupportedRequiredDefinition {
            kind: "schema",
            name,
            ..
        }) if name == TEST_SCHEMA
    ));
}

#[test]
fn rejects_an_unregistered_required_dialect() {
    let mut format = ArtifactFormat::new();
    format
        .add_dialect(
            DefinitionRequirement::new(
                TEST_DIALECT,
                HdlirVersion::new(1, 0, 0),
                true,
            )
            .expect("test dialect requirement must be valid"),
        )
        .expect("test dialect requirement must be unique");

    let bytes = encode_artifact(&format, b"payload", EncodeOptions::new())
        .expect("artifact with a valid dialect requirement must encode");

    assert!(matches!(
        decode_artifact(&bytes, &DecodeOptions::new()),
        Err(DecodeError::UnsupportedRequiredDefinition {
            kind: "dialect",
            name,
            ..
        }) if name == TEST_DIALECT
    ));
}

#[test]
fn deterministic_single_bit_mutations_never_panic_the_decoder() {
    let original = valid_artifact(b"deterministic malformed-input corpus");
    let options = bounded_decode_options();

    // Exercise the framing, metadata, and payload with the same reproducible
    // corpus on every run. A mutation may be rejected or remain valid if it
    // changes a non-invalidating field; the invariant under test is no panic.
    for index in 0..original.len() {
        for bit in 0..u8::BITS {
            let mut mutated = original.clone();
            mutated[index] ^= 1_u8 << bit;

            let outcome = catch_unwind(AssertUnwindSafe(|| {
                decode_artifact(&mutated, &options)
            }));

            assert!(
                outcome.is_ok(),
                "decoder panicked after flipping bit {bit} at byte {index}"
            );
        }
    }
}
