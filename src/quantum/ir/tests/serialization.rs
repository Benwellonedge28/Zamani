//! Zamani Quantum IR — Production Serialization Tests
//!
//! This test module verifies the public canonical serialization contract of
//! the Zamani Quantum IR.
//!
//! # Purpose
//!
//! These tests verify that the serialization subsystem:
//!
//! - is deterministic;
//! - round-trips semantic objects without loss;
//! - preserves canonical `quantum::ir::qubit::QubitId`;
//! - preserves canonical `quantum::ir::qubit::PhysicalQubitId`;
//! - preserves semantic IR versions;
//! - produces the documented canonical header;
//! - rejects truncated documents;
//! - rejects trailing bytes;
//! - rejects corrupted payloads;
//! - rejects unsupported semantic IR versions;
//! - enforces explicit resource policies;
//! - does not confuse serialization limits with quantum-machine limits;
//! - supports large collections without fixed quantum-size assumptions;
//! - prevents an `IrDecode` implementation from accepting a payload prefix;
//! - validates `SerializedIr` construction;
//! - remains free of `unsafe` code.
//!
//! # Architectural boundary
//!
//! These tests exercise the serialization API only.
//!
//! They deliberately do NOT test:
//!
//! - hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - simulation;
//! - QEC;
//! - frontend parsing;
//! - backend execution.
//!
//! Those systems consume or produce IR but do not own canonical serialization.
//!
//! # Integration contract
//!
//! This module consumes only the public serialization boundary:
//!
//! ```text
//! crate::quantum::ir::serialization
//! ```
//!
//! Quantum identities are always imported from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! No duplicate `QubitId` or `PhysicalQubitId` exists in this test module.
//!
//! # Scalability
//!
//! The tests use finite sample sizes only as regression fixtures.
//!
//! Those numbers are NOT:
//!
//! - machine limits;
//! - IR limits;
//! - qubit limits;
//! - operation limits;
//! - serialization limits.
//!
//! Production callers select resource policies explicitly.
//!
//! The serialization schema itself contains no fixed quantum-machine size.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use crate::quantum::ir::identity::IrVersion;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::ir::serialization::{
    deserialize,
    deserialize_with_limits,
    inspect,
    inspect_with_limits,
    serialize,
    serialize_artifact,
    serialize_with_limits,
    serialize_with_version,
    validate_document,
    validate_document_with_limits,
    DecodeLimits,
    Encoder,
    IrDecode,
    IrEncode,
    SerializationError,
    SerializedIr,
    FORMAT_VERSION,
    HEADER_LEN,
    MAGIC,
};

// =============================================================================
// Test fixtures
// =============================================================================

/// Small semantic object used to exercise the complete public codec boundary.
///
/// This deliberately uses the canonical quantum identity types from
/// `quantum::ir::qubit`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestObject {
    version: IrVersion,
    logical_qubit: QubitId,
    physical_qubit: PhysicalQubitId,
}

impl TestObject {
    fn new(
        version: IrVersion,
        logical_qubit: QubitId,
        physical_qubit: PhysicalQubitId,
    ) -> Self {
        Self {
            version,
            logical_qubit,
            physical_qubit,
        }
    }
}

impl IrEncode for TestObject {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_ir_version(self.version)?;
        encoder.write_qubit_id(self.logical_qubit)?;
        encoder.write_physical_qubit_id(self.physical_qubit)?;

        Ok(())
    }
}

impl IrDecode for TestObject {
    fn decode(
        decoder: &mut crate::quantum::ir::serialization::Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        Ok(Self {
            version: decoder.read_ir_version()?,
            logical_qubit: decoder.read_qubit_id()?,
            physical_qubit: decoder.read_physical_qubit_id()?,
        })
    }
}

/// Object containing a variable-size collection.
///
/// This is important because fixed-size test fixtures would not exercise the
/// resource-policy and large-program properties of the serialization layer.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ScalableObject {
    values: Vec<QubitId>,
}

impl ScalableObject {
    fn from_count(count: usize) -> Self {
        let mut values = Vec::new();

        for index in 0..count {
            values.push(QubitId::new(index));
        }

        Self { values }
    }
}

impl IrEncode for ScalableObject {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        let count = u64::try_from(self.values.len()).map_err(|_| {
            SerializationError::InvalidObject {
                message: "test collection length cannot be represented as u64",
            }
        })?;

        encoder.write_u64(count);

        for qubit in &self.values {
            encoder.write_qubit_id(*qubit)?;
        }

        Ok(())
    }
}

impl IrDecode for ScalableObject {
    fn decode(
        decoder: &mut crate::quantum::ir::serialization::Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        let count = decoder.read_u64()?;

        let count = usize::try_from(count).map_err(|_| {
            SerializationError::Codec {
                message: "test collection length cannot be represented by host usize"
                    .to_owned(),
            }
        })?;

        let mut values = Vec::new();

        for _ in 0..count {
            values.push(decoder.read_qubit_id()?);
        }

        Ok(Self { values })
    }
}

// =============================================================================
// Test helpers
// =============================================================================

fn sample_object() -> TestObject {
    TestObject::new(
        IrVersion::CURRENT,
        QubitId::new(7),
        PhysicalQubitId::new(11),
    )
}

/// Returns a sample object with a large identity value that remains portable
/// across both 32-bit and 64-bit hosts.
///
/// The value is intentionally below the minimum guaranteed 32-bit `usize`
/// range so the test verifies cross-platform wire stability without requiring
/// a particular host architecture.
fn cross_platform_object() -> TestObject {
    TestObject::new(
        IrVersion::CURRENT,
        QubitId::new(4_000_000_000usize),
        PhysicalQubitId::new(4_000_000_001usize),
    )
}

/// Returns a mutable copy of a valid document.
///
/// The helper is intentionally limited to test fixtures. Production code must
/// never modify canonical serialized artifacts in place.
fn sample_document() -> Vec<u8> {
    serialize(&sample_object()).expect("sample object must serialize")
}

/// Reads a little-endian u16 from a canonical document header.
fn read_u16_at(document: &[u8], offset: usize) -> u16 {
    let end = offset
        .checked_add(2)
        .expect("test offset arithmetic must not overflow");

    let bytes: [u8; 2] = document[offset..end]
        .try_into()
        .expect("test header must contain requested u16");

    u16::from_le_bytes(bytes)
}

/// Reads a little-endian u64 from a canonical document header.
fn read_u64_at(document: &[u8], offset: usize) -> u64 {
    let end = offset
        .checked_add(8)
        .expect("test offset arithmetic must not overflow");

    let bytes: [u8; 8] = document[offset..end]
        .try_into()
        .expect("test header must contain requested u64");

    u64::from_le_bytes(bytes)
}

// =============================================================================
// Basic canonical serialization
// =============================================================================

#[test]
fn serialization_produces_non_empty_canonical_document() {
    let document = sample_document();

    assert!(!document.is_empty());
    assert!(document.len() >= HEADER_LEN);
}

#[test]
fn canonical_document_starts_with_zqir_magic() {
    let document = sample_document();

    assert_eq!(&document[..MAGIC.len()], &MAGIC);
}

#[test]
fn canonical_header_has_documented_length() {
    let document = sample_document();

    assert!(document.len() >= HEADER_LEN);
    assert_eq!(HEADER_LEN, 24);
}

#[test]
fn canonical_header_contains_current_format_version() {
    let document = sample_document();

    let format_version = read_u16_at(&document, 4);

    assert_eq!(format_version, FORMAT_VERSION);
}

#[test]
fn canonical_header_contains_current_ir_version() {
    let document = sample_document();

    let major = read_u16_at(&document, 6);
    let minor = read_u16_at(&document, 8);
    let patch = read_u16_at(&document, 10);

    assert_eq!(major, IrVersion::CURRENT.major());
    assert_eq!(minor, IrVersion::CURRENT.minor());
    assert_eq!(patch, IrVersion::CURRENT.patch());
}

#[test]
fn canonical_header_payload_length_matches_document() {
    let document = sample_document();

    let payload_length = read_u64_at(&document, 12);

    let expected_payload_length = document
        .len()
        .checked_sub(HEADER_LEN)
        .expect("canonical document must contain a header");

    assert_eq!(
        payload_length,
        u64::try_from(expected_payload_length)
            .expect("test payload length must fit u64")
    );
}

#[test]
fn canonical_inspection_returns_exact_payload() {
    let document = sample_document();

    let inspected = inspect(&document)
        .expect("valid canonical document must inspect successfully");

    assert_eq!(
        inspected.ir_version(),
        IrVersion::CURRENT
    );

    assert_eq!(
        inspected.payload().len(),
        document.len() - HEADER_LEN
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn serialization_is_deterministic() {
    let object = sample_object();

    let first = serialize(&object)
        .expect("first serialization must succeed");

    let second = serialize(&object)
        .expect("second serialization must succeed");

    assert_eq!(first, second);
}

#[test]
fn serialization_is_deterministic_across_repeated_runs() {
    let object = cross_platform_object();

    let first = serialize(&object)
        .expect("first serialization must succeed");

    for _ in 0..32 {
        let next = serialize(&object)
            .expect("repeated serialization must succeed");

        assert_eq!(next, first);
    }
}

#[test]
fn equivalent_objects_have_identical_canonical_bytes() {
    let first = sample_object();

    let second = TestObject::new(
        IrVersion::CURRENT,
        QubitId::new(7),
        PhysicalQubitId::new(11),
    );

    assert_eq!(
        serialize(&first).expect("first serialization"),
        serialize(&second).expect("second serialization")
    );
}

// =============================================================================
// Round-trip correctness
// =============================================================================

#[test]
fn test_object_round_trips_without_semantic_loss() {
    let original = sample_object();

    let document = serialize(&original)
        .expect("object must serialize");

    let decoded: TestObject = deserialize(&document)
        .expect("canonical document must deserialize");

    assert_eq!(decoded, original);
}

#[test]
fn logical_qubit_id_round_trips_using_canonical_qubit_module() {
    let original = QubitId::new(4_000_000_000usize);

    let object = TestObject::new(
        IrVersion::CURRENT,
        original,
        PhysicalQubitId::new(19),
    );

    let document = serialize(&object)
        .expect("object must serialize");

    let decoded: TestObject = deserialize(&document)
        .expect("object must deserialize");

    assert_eq!(decoded.logical_qubit, original);
}

#[test]
fn physical_qubit_id_round_trips_using_canonical_qubit_module() {
    let original = PhysicalQubitId::new(4_000_000_001usize);

    let object = TestObject::new(
        IrVersion::CURRENT,
        QubitId::new(19),
        original,
    );

    let document = serialize(&object)
        .expect("object must serialize");

    let decoded: TestObject = deserialize(&document)
        .expect("object must deserialize");

    assert_eq!(decoded.physical_qubit, original);
}

#[test]
fn logical_and_physical_qubit_ids_remain_distinct_types() {
    fn accepts_logical(_: QubitId) {}

    fn accepts_physical(_: PhysicalQubitId) {}

    let logical = QubitId::new(1);
    let physical = PhysicalQubitId::new(1);

    accepts_logical(logical);
    accepts_physical(physical);

    assert_ne!(
        format!("{logical}"),
        format!("{physical}")
    );
}

#[test]
fn large_identity_values_round_trip() {
    let original = cross_platform_object();

    let document = serialize(&original)
        .expect("large identity object must serialize");

    let decoded: TestObject = deserialize(&document)
        .expect("large identity object must deserialize");

    assert_eq!(decoded, original);
}

// =============================================================================
// Version handling
// =============================================================================

#[test]
fn serialization_with_current_version_succeeds() {
    let object = sample_object();

    let document = serialize_with_version(
        &object,
        IrVersion::CURRENT,
    )
    .expect("current version serialization must succeed");

    let inspected = inspect(&document)
        .expect("current version document must inspect");

    assert_eq!(
        inspected.ir_version(),
        IrVersion::CURRENT
    );
}

#[test]
fn future_major_ir_version_is_not_silently_accepted() {
    let object = sample_object();

    let document = serialize_with_version(
        &object,
        IrVersion::new(
            IrVersion::CURRENT.major().saturating_add(1),
            0,
            0,
        ),
    )
    .expect("test document must be constructible");

    let result = deserialize::<TestObject>(&document);

    assert!(matches!(
        result,
        Err(SerializationError::UnsupportedIrVersion { .. })
    ));
}

#[test]
fn future_minor_ir_version_is_not_silently_accepted() {
    let object = sample_object();

    let future_minor = IrVersion::new(
        IrVersion::CURRENT.major(),
        IrVersion::CURRENT.minor().saturating_add(1),
        0,
    );

    let document = serialize_with_version(
        &object,
        future_minor,
    )
    .expect("test document must be constructible");

    let result = deserialize::<TestObject>(&document);

    assert!(matches!(
        result,
        Err(SerializationError::UnsupportedIrVersion { .. })
    ));
}

#[test]
fn future_patch_version_is_not_silently_accepted() {
    let object = sample_object();

    let future_patch = IrVersion::new(
        IrVersion::CURRENT.major(),
        IrVersion::CURRENT.minor(),
        IrVersion::CURRENT.patch().saturating_add(1),
    );

    let document = serialize_with_version(
        &object,
        future_patch,
    )
    .expect("test document must be constructible");

    let result = deserialize::<TestObject>(&document);

    assert!(matches!(
        result,
        Err(SerializationError::UnsupportedIrVersion { .. })
    ));
}

// =============================================================================
// Corruption resistance
// =============================================================================

#[test]
fn truncated_document_is_rejected() {
    let document = sample_document();

    for length in 0..document.len() {
        let truncated = &document[..length];

        assert!(
            inspect(truncated).is_err(),
            "truncated document of length {length} was accepted"
        );
    }
}

#[test]
fn document_shorter_than_header_is_rejected() {
    let document = sample_document();

    for length in 0..HEADER_LEN {
        assert!(
            inspect(&document[..length]).is_err(),
            "document shorter than header was accepted at length {length}"
        );
    }
}

#[test]
fn trailing_bytes_are_rejected() {
    let mut document = sample_document();

    document.push(0);

    let result = inspect(&document);

    assert!(matches!(
        result,
        Err(SerializationError::TrailingBytes { .. })
    ));
}

#[test]
fn multiple_trailing_bytes_are_rejected() {
    let mut document = sample_document();

    document.extend_from_slice(&[0, 1, 2, 3, 4]);

    let result = inspect(&document);

    assert!(matches!(
        result,
        Err(SerializationError::TrailingBytes { .. })
    ));
}

#[test]
fn payload_corruption_is_detected_by_integrity_check() {
    let mut document = sample_document();

    assert!(
        document.len() > HEADER_LEN,
        "test document must have a payload"
    );

    document[HEADER_LEN] ^= 0x01;

    let result = inspect(&document);

    assert!(matches!(
        result,
        Err(SerializationError::ChecksumMismatch { .. })
    ));
}

#[test]
fn payload_corruption_cannot_be_hidden_by_valid_semantics() {
    let mut document = sample_document();

    assert!(
        document.len() > HEADER_LEN,
        "test document must have a payload"
    );

    document[HEADER_LEN] ^= 0xff;

    let result = deserialize::<TestObject>(&document);

    assert!(matches!(
        result,
        Err(SerializationError::ChecksumMismatch { .. })
    ));
}

#[test]
fn invalid_magic_is_rejected_before_payload_processing() {
    let mut document = sample_document();

    document[0] ^= 0xff;

    let result = inspect(&document);

    assert!(matches!(
        result,
        Err(SerializationError::InvalidMagic { .. })
    ));
}

#[test]
fn unsupported_format_version_is_rejected() {
    let mut document = sample_document();

    let future_format = FORMAT_VERSION.saturating_add(1);

    document[4..6].copy_from_slice(
        &future_format.to_le_bytes()
    );

    let result = inspect(&document);

    assert!(matches!(
        result,
        Err(SerializationError::UnsupportedFormatVersion { .. })
    ));
}

#[test]
fn declared_payload_length_cannot_exceed_document() {
    let mut document = sample_document();

    let declared_length = u64::MAX;

    document[12..20].copy_from_slice(
        &declared_length.to_le_bytes()
    );

    let result = inspect(&document);

    assert!(result.is_err());
}

#[test]
fn declared_payload_length_cannot_be_smaller_than_actual_payload() {
    let mut document = sample_document();

    let declared_length = 0_u64;

    document[12..20].copy_from_slice(
        &declared_length.to_le_bytes()
    );

    let result = inspect(&document);

    assert!(result.is_err());
}

// =============================================================================
// Explicit resource-policy tests
// =============================================================================

#[test]
fn default_decode_limits_are_valid() {
    let limits = DecodeLimits::default();

    assert!(
        limits.validate().is_ok(),
        "default decode policy must be internally valid"
    );
}

#[test]
fn conservative_decode_limits_are_valid() {
    let limits = DecodeLimits::conservative();

    assert!(
        limits.validate().is_ok(),
        "conservative decode policy must be internally valid"
    );
}

#[test]
fn zero_document_limit_is_rejected() {
    let limits = DecodeLimits::new(
        0,
        1,
        1,
        1,
        1,
    );

    assert!(limits.validate().is_err());
}

#[test]
fn zero_payload_limit_is_rejected() {
    let limits = DecodeLimits::new(
        1,
        0,
        1,
        1,
        1,
    );

    assert!(limits.validate().is_err());
}

#[test]
fn zero_field_limit_is_rejected() {
    let limits = DecodeLimits::new(
        1,
        1,
        0,
        1,
        1,
    );

    assert!(limits.validate().is_err());
}

#[test]
fn zero_collection_limit_is_rejected() {
    let limits = DecodeLimits::new(
        1,
        1,
        1,
        0,
        1,
    );

    assert!(limits.validate().is_err());
}

#[test]
fn zero_nesting_limit_is_rejected() {
    let limits = DecodeLimits::new(
        1,
        1,
        1,
        1,
        0,
    );

    assert!(limits.validate().is_err());
}

#[test]
fn_payload_limit_cannot_exceed_document_limit() {
    let limits = DecodeLimits::new(
        10,
        11,
        10,
        10,
        10,
    );

    assert!(limits.validate().is_err());
}

#[test]
fn document_size_policy_is_enforced() {
    let document = sample_document();

    let limits = DecodeLimits::new(
        u64::try_from(document.len() - 1)
            .expect("test document length must fit u64"),
        u64::try_from(document.len())
            .expect("test document length must fit u64"),
        u64::MAX,
        u64::MAX,
        u64::MAX,
    );

    let result = inspect_with_limits(&document, limits);

    assert!(matches!(
        result,
        Err(SerializationError::DocumentTooLarge { .. })
    ));
}

#[test]
fn payload_size_policy_is_enforced() {
    let document = sample_document();

    let payload_length = document
        .len()
        .checked_sub(HEADER_LEN)
        .expect("document must contain a payload");

    let limits = DecodeLimits::new(
        u64::try_from(document.len())
            .expect("test document length must fit u64"),
        u64::try_from(payload_length - 1)
            .expect("test payload length must be at least one byte"),
        u64::MAX,
        u64::MAX,
        u64::MAX,
    );

    let result = inspect_with_limits(&document, limits);

    assert!(matches!(
        result,
        Err(SerializationError::PayloadTooLarge { .. })
    ));
}

#[test]
fn explicit_large_policy_allows_larger_documents() {
    let document = sample_document();

    let document_size = u64::try_from(document.len())
        .expect("test document length must fit u64");

    let payload_size = u64::try_from(
        document.len() - HEADER_LEN
    )
    .expect("test payload length must fit u64");

    let limits = DecodeLimits::new(
        document_size,
        payload_size,
        payload_size,
        u64::MAX,
        u64::MAX,
    );

    let inspected = inspect_with_limits(
        &document,
        limits,
    )
    .expect("explicit sufficient policy must allow document");

    assert_eq!(
        inspected.ir_version(),
        IrVersion::CURRENT
    );
}

// =============================================================================
// SerializedIr artifact tests
// =============================================================================

#[test]
fn serialized_ir_accepts_only_structurally_valid_documents() {
    let document = sample_document();

    let artifact = SerializedIr::from_bytes(
        document.clone()
    )
    .expect("valid document must become a SerializedIr");

    assert_eq!(
        artifact.as_bytes(),
        document.as_slice()
    );

    assert_eq!(
        artifact.len(),
        document.len()
    );

    assert!(!artifact.is_empty());
}

#[test]
fn serialized_ir_rejects_corrupted_document() {
    let mut document = sample_document();

    document[HEADER_LEN] ^= 0x01;

    let result = SerializedIr::from_bytes(document);

    assert!(matches!(
        result,
        Err(SerializationError::ChecksumMismatch { .. })
    ));
}

#[test]
fn serialized_ir_into_bytes_preserves_exact_document() {
    let document = sample_document();

    let artifact = SerializedIr::from_bytes(
        document.clone()
    )
    .expect("valid document must become artifact");

    let recovered = artifact.into_bytes();

    assert_eq!(recovered, document);
}

#[test]
fn serialize_artifact_matches_direct_serialization() {
    let object = sample_object();

    let direct = serialize(&object)
        .expect("direct serialization must succeed");

    let artifact = serialize_artifact(&object)
        .expect("artifact serialization must succeed");

    assert_eq!(
        artifact.as_bytes(),
        direct.as_slice()
    );
}

// =============================================================================
// Structural validation API
// =============================================================================

#[test]
fn validate_document_accepts_valid_document() {
    let document = sample_document();

    validate_document(&document)
        .expect("valid canonical document must validate");
}

#[test]
fn validate_document_rejects_corrupted_document() {
    let mut document = sample_document();

    document[HEADER_LEN] ^= 0x01;

    let result = validate_document(&document);

    assert!(result.is_err());
}

#[test]
fn validate_document_rejects_trailing_bytes() {
    let mut document = sample_document();

    document.push(0);

    let result = validate_document(&document);

    assert!(matches!(
        result,
        Err(SerializationError::TrailingBytes { .. })
    ));
}

#[test]
fn validate_document_with_limits_enforces_policy() {
    let document = sample_document();

    let limits = DecodeLimits::new(
        u64::try_from(document.len() - 1)
            .expect("test document length must fit u64"),
        u64::try_from(document.len())
            .expect("test document length must fit u64"),
        u64::MAX,
        u64::MAX,
        u64::MAX,
    );

    let result = validate_document_with_limits(
        &document,
        limits,
    );

    assert!(matches!(
        result,
        Err(SerializationError::DocumentTooLarge { .. })
    ));
}

// =============================================================================
// Decoder-consumption safety
// =============================================================================

/// Decoder which deliberately consumes only the first semantic field.
///
/// The public `deserialize` API must reject this object because the decoder
/// still contains unread payload bytes.
///
/// This prevents a malicious or buggy codec from accepting:
///
/// ```text
/// valid_prefix + attacker_controlled_suffix
/// ```
///
/// as a valid semantic object.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PrefixOnlyObject {
    version: IrVersion,
}

impl IrDecode for PrefixOnlyObject {
    fn decode(
        decoder: &mut crate::quantum::ir::serialization::Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        Ok(Self {
            version: decoder.read_ir_version()?,
        })
    }
}

#[test]
fn decoder_must_consume_entire_payload() {
    let document = serialize(&sample_object())
        .expect("sample object must serialize");

    let result = deserialize::<PrefixOnlyObject>(&document);

    assert!(result.is_err());
}

// =============================================================================
// Large collection / scaling tests
// =============================================================================

#[test]
fn scalable_object_with_empty_collection_round_trips() {
    let original = ScalableObject::from_count(0);

    let document = serialize(&original)
        .expect("empty scalable object must serialize");

    let decoded: ScalableObject = deserialize(&document)
        .expect("empty scalable object must deserialize");

    assert_eq!(decoded, original);
}

#[test]
fn scalable_object_with_many_elements_round_trips() {
    // Regression sample only.
    //
    // This is not a protocol limit or architectural capacity.
    let original = ScalableObject::from_count(8_192);

    let document = serialize_with_limits(
        &original,
        crate::quantum::ir::serialization::EncodeLimits::default(),
    )
    .expect("large scalable object must serialize");

    let decoded: ScalableObject = deserialize_with_limits(
        &document,
        DecodeLimits::conservative(),
    )
    .expect("large scalable object must deserialize");

    assert_eq!(decoded, original);
}

#[test]
fn scalable_object_uses_u64_wire_collection_length() {
    let original = ScalableObject::from_count(1);

    let document = serialize(&original)
        .expect("scalable object must serialize");

    assert!(document.len() >= HEADER_LEN + 8);

    let payload = &document[HEADER_LEN..];

    let count_bytes: [u8; 8] = payload[..8]
        .try_into()
        .expect("collection count must occupy eight bytes");

    let count = u64::from_le_bytes(count_bytes);

    assert_eq!(count, 1);
}

#[test]
fn collection_policy_can_reject_large_collection_before_semantic_completion() {
    let original = ScalableObject::from_count(128);

    let document = serialize(&original)
        .expect("scalable object must serialize");

    let limits = DecodeLimits::new(
        u64::try_from(document.len())
            .expect("document length must fit u64"),
        u64::try_from(document.len() - HEADER_LEN)
            .expect("payload length must fit u64"),
        u64::MAX,
        1,
        u64::MAX,
    );

    let result = deserialize_with_limits::<ScalableObject>(
        &document,
        limits,
    );

    assert!(result.is_err());
}

// =============================================================================
// Canonical envelope invariants
// =============================================================================

#[test]
fn canonical_document_is_exactly_header_plus_payload() {
    let document = sample_document();

    let inspected = inspect(&document)
        .expect("document must inspect");

    let expected = HEADER_LEN
        .checked_add(inspected.payload().len())
        .expect("test document size arithmetic must not overflow");

    assert_eq!(document.len(), expected);
}

#[test]
fn canonical_payload_is_not_empty_for_test_object() {
    let document = sample_document();

    let inspected = inspect(&document)
        .expect("document must inspect");

    assert!(
        !inspected.payload().is_empty(),
        "test fixture must exercise payload serialization"
    );
}

#[test]
fn inspect_does_not_modify_input() {
    let document = sample_document();
    let before = document.clone();

    let _ = inspect(&document)
        .expect("document must inspect");

    assert_eq!(document, before);
}

#[test]
fn deserialize_does_not_require_mutable_input() {
    let document = sample_document();

    let decoded: TestObject = deserialize(&document)
        .expect("immutable serialized input must deserialize");

    assert_eq!(decoded, sample_object());
}

// =============================================================================
// Compatibility / ownership invariants
// =============================================================================

#[test]
fn canonical_qubit_identity_types_are_used_directly() {
    let logical: crate::quantum::ir::qubit::QubitId =
        QubitId::new(17);

    let physical: crate::quantum::ir::qubit::PhysicalQubitId =
        PhysicalQubitId::new(23);

    let object = TestObject::new(
        IrVersion::CURRENT,
        logical,
        physical,
    );

    let document = serialize(&object)
        .expect("canonical qubit identity object must serialize");

    let decoded: TestObject = deserialize(&document)
        .expect("canonical qubit identity object must deserialize");

    assert_eq!(decoded.logical_qubit, logical);
    assert_eq!(decoded.physical_qubit, physical);
}

#[test]
fn serialization_does_not_change_qubit_identity_values() {
    let logical = QubitId::new(123_456);
    let physical = PhysicalQubitId::new(654_321);

    let object = TestObject::new(
        IrVersion::CURRENT,
        logical,
        physical,
    );

    let document = serialize(&object)
        .expect("object must serialize");

    let decoded: TestObject = deserialize(&document)
        .expect("object must deserialize");

    assert_eq!(decoded.logical_qubit, logical);
    assert_eq!(decoded.physical_qubit, physical);
}

// =============================================================================
// Regression test for exact canonical bytes
// =============================================================================

#[test]
fn repeated_canonical_serialization_has_identical_header_and_payload() {
    let object = sample_object();

    let first = serialize(&object)
        .expect("first serialization must succeed");

    let second = serialize(&object)
        .expect("second serialization must succeed");

    assert_eq!(first.len(), second.len());

    assert_eq!(
        &first[..HEADER_LEN],
        &second[..HEADER_LEN]
    );

    assert_eq!(
        &first[HEADER_LEN..],
        &second[HEADER_LEN..]
    );
}

// =============================================================================
// No accidental protocol widening
// =============================================================================

#[test]
fn canonical_magic_is_exactly_four_bytes() {
    assert_eq!(MAGIC.len(), 4);
}

#[test]
fn canonical_header_length_is_stable() {
    assert_eq!(HEADER_LEN, 24);
}

#[test]
fn serialization_format_version_is_non_zero() {
    assert_ne!(FORMAT_VERSION, 0);
}

// =============================================================================
// Test policy
// =============================================================================

#[test]
fn test_limits_are_policy_not_quantum_capacity() {
    let small = DecodeLimits::new(
        1024,
        512,
        256,
        16,
        16,
    );

    let large = DecodeLimits::new(
        1024 * 1024 * 1024,
        1024 * 1024 * 1024,
        1024 * 1024 * 1024,
        u64::MAX,
        u64::MAX,
    );

    assert!(small.validate().is_ok());
    assert!(large.validate().is_ok());

    // The existence of two valid policies demonstrates that the serializer
    // does not encode one fixed machine-size boundary into the IR schema.
    assert_ne!(
        small.max_document_bytes,
        large.max_document_bytes
    );
}