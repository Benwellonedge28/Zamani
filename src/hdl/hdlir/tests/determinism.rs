#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! HDLIR determinism and reproducibility integration tests.
//!
//! Path: `src/hdl/hdlir/tests/determinism.rs`
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Verify deterministic behavior of the HDLIR fingerprint protocol and
//! versioned artifact serialization APIs that currently exist in the
//! repository.
//!
//! # Authority and ownership
//!
//! These tests follow:
//!
//! - `grammar/spec/hdl.md`
//! - `src/hdl/hdlir/fingerprint.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/serialization/format.rs`
//! - `src/hdl/hdlir/serialization/encode.rs`
//! - `src/hdl/hdlir/serialization/decode.rs`
//!
//! The grammar and HDL specification own hardware-language semantics.
//! The fingerprint module owns its framed digest protocol. The serialization
//! modules own artifact metadata and framing. This file owns cross-module
//! regression tests for those existing contracts.
//!
//! # Important scope boundary
//!
//! The current HDLIR module root does not yet expose a complete, verified
//! hardware-design graph or a canonical graph-serialization API. These tests
//! therefore do not claim to prove that arbitrary HDLIR graphs, transformations,
//! or compiler pipelines are deterministic.
//!
//! Graph-level determinism tests must be added when the canonical graph,
//! canonicalization rules, and semantic fingerprint entry point are implemented.
//!
//! # Scalability and safety
//!
//! No hardware-capacity ceiling is introduced here. Test data is generated
//! from the values needed by each test. The tests do not use unsafe Rust,
//! pointer-derived identities, process-random hashers, or host-endian encoding.
//!
//! Successful tests establish only the contracts exercised by these tests.
//! They do not establish target feasibility, synthesis correctness, or
//! production readiness of the complete HDLIR subsystem.

use std::collections::BTreeMap;

use crate::hdl::hdlir::fingerprint::{
    FingerprintBuilder,
    HdlirFingerprint,
    HDLIR_FINGERPRINT_ALGORITHM,
};
use crate::hdl::hdlir::serialization::decode::{
    decode_artifact,
    DecodeOptions,
};
use crate::hdl::hdlir::serialization::encode::{
    encode_artifact,
    EncodeOptions,
};
use crate::hdl::hdlir::serialization::format::{
    ArtifactFormat,
    DefinitionRequirement,
};
use crate::hdl::hdlir::version::HdlirVersion;

/// Build a fingerprint from explicitly canonicalized manifest fields.
///
/// `BTreeMap` supplies deterministic key ordering. Callers must still choose
/// stable field names and include every version/configuration input relevant
/// to the identity they intend to calculate.
///
/// This helper tests the existing fingerprint protocol. It is not a substitute
/// for the future canonical HDLIR graph fingerprint implementation.
fn fingerprint_manifest(
    domain: &str,
    fields: &BTreeMap<&str, String>,
) -> HdlirFingerprint {
    let mut builder =
        FingerprintBuilder::new(domain).expect("test fingerprint domain is valid");

    builder
        .write_str("algorithm", HDLIR_FINGERPRINT_ALGORITHM)
        .expect("algorithm identifier is a valid field");

    for (tag, value) in fields {
        builder
            .write_str(tag, value)
            .expect("manifest field tag is valid");
    }

    builder.finish()
}

/// Construct equivalent artifact metadata using different insertion orders.
///
/// The canonical serialization contract requires metadata collections to be
/// encoded deterministically regardless of the order in which callers register
/// equivalent schema and dialect requirements.
fn artifact_format(reverse_insertion_order: bool) -> ArtifactFormat {
    let mut format = ArtifactFormat::new();

    let schemas = [
        ("hardware.module", HdlirVersion::new(1, 0, 0)),
        ("hardware.signal", HdlirVersion::new(1, 0, 0)),
        ("hardware.memory", HdlirVersion::new(2, 1, 0)),
    ];

    let dialects = [
        ("hardware.behavioral", HdlirVersion::new(1, 0, 0)),
        ("hardware.structural", HdlirVersion::new(1, 2, 0)),
    ];

    if reverse_insertion_order {
        for (name, version) in schemas.iter().rev() {
            format
                .add_schema(
                    DefinitionRequirement::new(*name, *version, true)
                        .expect("test schema requirement is valid"),
                )
                .expect("schema requirement must be unique");
        }

        for (name, version) in dialects.iter().rev() {
            format
                .add_dialect(
                    DefinitionRequirement::new(*name, *version, true)
                        .expect("test dialect requirement is valid"),
                )
                .expect("dialect requirement must be unique");
        }
    } else {
        for (name, version) in schemas {
            format
                .add_schema(
                    DefinitionRequirement::new(name, version, true)
                        .expect("test schema requirement is valid"),
                )
                .expect("schema requirement must be unique");
        }

        for (name, version) in dialects {
            format
                .add_dialect(
                    DefinitionRequirement::new(name, version, true)
                        .expect("test dialect requirement is valid"),
                )
                .expect("dialect requirement must be unique");
        }
    }

    format
}

#[test]
fn identical_fingerprint_inputs_produce_identical_digests() {
    let fields = BTreeMap::from([
        ("design.name", String::from("portable_top")),
        ("schema.version", String::from("1.0.0")),
        ("configuration.profile", String::from("deterministic")),
    ]);

    let first = fingerprint_manifest("zamani.hdlir.test.design.v1", &fields);
    let second = fingerprint_manifest("zamani.hdlir.test.design.v1", &fields);

    assert_eq!(first, second);
    assert_eq!(first.as_bytes(), second.as_bytes());
    assert_eq!(first.to_hex(), second.to_hex());
    assert_eq!(first.to_string(), second.to_string());
}

#[test]
fn fingerprint_is_independent_of_equivalent_field_insertion_order() {
    let first_fields = BTreeMap::from([
        ("design.name", String::from("portable_top")),
        ("schema.version", String::from("1.0.0")),
        ("pass.version", String::from("2")),
        ("configuration.profile", String::from("deterministic")),
    ]);

    let second_fields = BTreeMap::from([
        ("configuration.profile", String::from("deterministic")),
        ("pass.version", String::from("2")),
        ("schema.version", String::from("1.0.0")),
        ("design.name", String::from("portable_top")),
    ]);

    assert_eq!(
        fingerprint_manifest("zamani.hdlir.test.design.v1", &first_fields),
        fingerprint_manifest("zamani.hdlir.test.design.v1", &second_fields),
        "equivalent manifest fields must have the same canonical ordering"
    );
}

#[test]
fn fingerprint_domain_separates_different_content_kinds() {
    let fields = BTreeMap::from([
        ("design.name", String::from("portable_top")),
        ("schema.version", String::from("1.0.0")),
    ]);

    let design = fingerprint_manifest("zamani.hdlir.test.design.v1", &fields);
    let module = fingerprint_manifest("zamani.hdlir.test.module.v1", &fields);

    assert_ne!(
        design, module,
        "the same fields in different fingerprint domains must not collide by construction"
    );
}

#[test]
fn fingerprint_changes_when_schema_version_changes() {
    let first_fields = BTreeMap::from([
        ("design.name", String::from("portable_top")),
        ("schema.version", String::from("1.0.0")),
    ]);

    let second_fields = BTreeMap::from([
        ("design.name", String::from("portable_top")),
        ("schema.version", String::from("2.0.0")),
    ]);

    assert_ne!(
        fingerprint_manifest("zamani.hdlir.test.design.v1", &first_fields),
        fingerprint_manifest("zamani.hdlir.test.design.v1", &second_fields),
        "a schema-version change must affect a manifest fingerprint when schema version is included"
    );
}

#[test]
fn fingerprint_changes_when_pass_version_changes() {
    let first_fields = BTreeMap::from([
        ("design.name", String::from("portable_top")),
        ("pass.name", String::from("canonicalize")),
        ("pass.version", String::from("1")),
    ]);

    let second_fields = BTreeMap::from([
        ("design.name", String::from("portable_top")),
        ("pass.name", String::from("canonicalize")),
        ("pass.version", String::from("2")),
    ]);

    assert_ne!(
        fingerprint_manifest("zamani.hdlir.test.pass-input.v1", &first_fields),
        fingerprint_manifest("zamani.hdlir.test.pass-input.v1", &second_fields),
        "pass versions must participate in fingerprints for caches whose results depend on the pass version"
    );
}

#[test]
fn equivalent_metadata_insertion_orders_produce_identical_artifacts() {
    let payload = b"canonical HDLIR test payload";

    let first = encode_artifact(
        &artifact_format(false),
        payload,
        EncodeOptions::default(),
    )
    .expect("first artifact should encode");

    let second = encode_artifact(
        &artifact_format(true),
        payload,
        EncodeOptions::default(),
    )
    .expect("second artifact should encode");

    assert_eq!(
        first, second,
        "equivalent metadata must not serialize differently because of insertion order"
    );
}

#[test]
fn repeated_artifact_encoding_is_byte_for_byte_deterministic() {
    let format = artifact_format(false);
    let payload = b"stable payload bytes";

    let first = encode_artifact(
        &format,
        payload,
        EncodeOptions::default(),
    )
    .expect("first encoding should succeed");

    let second = encode_artifact(
        &format,
        payload,
        EncodeOptions::default(),
    )
    .expect("second encoding should succeed");

    assert_eq!(first, second);
}

#[test]
fn artifact_round_trip_preserves_payload_and_canonical_encoding() {
    let format = artifact_format(false);
    let payload = [
        0_u8, 1, 2, 3, 0, 127, 128, 254, 255,
    ];

    let original = encode_artifact(
        &format,
        &payload,
        EncodeOptions::default(),
    )
    .expect("original artifact should encode");

    let decoded = decode_artifact(&original, &DecodeOptions::default())
        .expect("encoded artifact should decode");

    assert_eq!(decoded.payload(), payload);
    assert_eq!(decoded.format(), &format);

    let reencoded = encode_artifact(
        decoded.format(),
        decoded.payload(),
        EncodeOptions::default(),
    )
    .expect("decoded artifact should re-encode");

    assert_eq!(
        original, reencoded,
        "decoding and re-encoding a supported canonical artifact must preserve its bytes"
    );
}

#[test]
fn artifact_bytes_change_when_payload_changes() {
    let format = ArtifactFormat::new();

    let first = encode_artifact(
        &format,
        b"payload A",
        EncodeOptions::default(),
    )
    .expect("first artifact should encode");

    let second = encode_artifact(
        &format,
        b"payload B",
        EncodeOptions::default(),
    )
    .expect("second artifact should encode");

    assert_ne!(
        first, second,
        "different payloads must not produce the same complete artifact bytes"
    );
}

#[test]
fn artifact_bytes_change_when_required_schema_version_changes() {
    let mut first_format = ArtifactFormat::new();
    first_format
        .add_schema(
            DefinitionRequirement::new(
                "hardware.module",
                HdlirVersion::new(1, 0, 0),
                true,
            )
            .expect("first schema requirement is valid"),
        )
        .expect("first schema requirement should be added");

    let mut second_format = ArtifactFormat::new();
    second_format
        .add_schema(
            DefinitionRequirement::new(
                "hardware.module",
                HdlirVersion::new(2, 0, 0),
                true,
            )
            .expect("second schema requirement is valid"),
        )
        .expect("second schema requirement should be added");

    let payload = b"same payload";

    let first = encode_artifact(
        &first_format,
        payload,
        EncodeOptions::default(),
    )
    .expect("first artifact should encode");

    let second = encode_artifact(
        &second_format,
        payload,
        EncodeOptions::default(),
    )
    .expect("second artifact should encode");

    assert_ne!(
        first, second,
        "artifact metadata must preserve required schema-version differences"
    );
}

#[test]
fn deterministic_fingerprint_has_stable_text_representation() {
    let mut builder = FingerprintBuilder::new("zamani.hdlir.test.stable-text.v1")
        .expect("fingerprint domain is valid");

    builder
        .write_str("name", "portable_top")
        .expect("field should be accepted");
    builder
        .write_u64("schema.major", 1)
        .expect("numeric field should be accepted");
    builder
        .write_bool("canonical", true)
        .expect("boolean field should be accepted");

    let fingerprint = builder.finish();
    let text = fingerprint.to_string();

    assert_eq!(fingerprint.to_hex(), text);
    assert_eq!(text.len(), 64);
    assert!(
        text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "fingerprint text must use lowercase hexadecimal"
    );
}