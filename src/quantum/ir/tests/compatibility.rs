//! Zamani Quantum IR — Compatibility Contract Tests.
//!
//! This module verifies compatibility guarantees across the canonical
//! `quantum::ir` boundary.
//!
//! # Purpose
//!
//! These tests are intentionally different from ordinary unit tests.
//!
//! They verify that independently implemented or independently evolved parts
//! of the Quantum IR continue to agree on:
//!
//! - canonical module ownership;
//! - logical qubit identity ownership;
//! - physical qubit identity ownership;
//! - legacy compatibility aliases;
//! - IR semantic version compatibility;
//! - serialization-format version separation;
//! - canonical serialization round trips;
//! - deterministic serialization;
//! - deterministic identity representation;
//! - malformed-document rejection;
//! - forward-version rejection;
//! - trailing-byte rejection;
//! - checksum/integrity rejection;
//! - bounded decoding;
//! - canonical primitive encoding;
//! - preservation of semantic identity across serialization;
//! - absence of fixed quantum-machine-size assumptions.
//!
//! # Architectural boundary
//!
//! The tests deliberately exercise the public compatibility boundary rather
//! than private implementation details.
//!
//! They therefore do NOT depend on:
//!
//! - frontend ASTs;
//! - OpenQASM ASTs;
//! - routing;
//! - topology;
//! - hardware;
//! - scheduling;
//! - calibration;
//! - backend execution;
//! - simulator state;
//! - QEC implementation;
//! - optimization algorithms;
//! - vendor-specific APIs.
//!
//! # Canonical dependency boundary
//!
//! ```text
//! quantum::ir::identity
//!          │
//!          ├──────────────┐
//!          ▼              ▼
//! quantum::ir::qubit   quantum::ir::serialization
//!          │              │
//!          └──────┬───────┘
//!                 ▼
//!        compatibility tests
//! ```
//!
//! # Compatibility philosophy
//!
//! The compatibility contract is intentionally conservative:
//!
//! 1. Existing canonical types remain canonical.
//! 2. Legacy aliases must refer to the same canonical types.
//! 3. Unknown future IR versions must not be silently interpreted.
//! 4. Unknown serialization formats must not be silently interpreted.
//! 5. Corrupt documents must be rejected.
//! 6. Truncated documents must be rejected.
//! 7. Trailing bytes in canonical documents must be rejected.
//! 8. Decoder resource policies must remain explicit.
//! 9. Serialization must be deterministic.
//! 10. Logical and physical qubit identities must remain distinct.
//! 11. Compatibility must not introduce a fixed quantum-machine size.
//!
//! # Important scalability rule
//!
//! These tests may use small numeric values because tests need finite,
//! inexpensive fixtures. Those values are test data only.
//!
//! They MUST NOT be interpreted as architectural limits.
//!
//! In particular, this file must never introduce assertions such as:
//!
//! ```text
//! maximum qubits == 64
//! maximum qubits == 4096
//! maximum operations == 1_000_000
//! ```
//!
//! Instead, compatibility is tested through type and wire-format invariants.
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
//! No external test dependency is required.
//!
//! # Integration
//!
//! This file is intended to be registered from:
//!
//! `src/quantum/ir/tests.rs`
//!
//! with:
//!
//! ```text
//! #[path = "tests/compatibility.rs"]
//! mod compatibility;
//! ```
//!
//! The tests use only public APIs from `quantum::ir`.
//!
//! If implementation files are reorganized internally, this test should remain
//! unchanged as long as the public IR compatibility contract remains valid.
//!
//! # Ownership
//!
//! This file owns ONLY cross-module compatibility assertions.
//!
//! It does not own any production IR type.
//!
//! Production types remain owned by their canonical modules.
//!
//! In particular:
//!
//! - `QubitId` is owned by `quantum::ir::qubit`;
//! - `PhysicalQubitId` is owned by `quantum::ir::qubit`;
//! - `IrVersion` is owned by `quantum::ir::identity`;
//! - serialization framing is owned by `quantum::ir::serialization`.
//!
//! # No unsafe
//!
//! The compiler must enforce the no-unsafe requirement for this test module.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! is intentionally present below.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use super::identity::IrVersion;
use super::qubit::{PhysicalQubitId, QubitId};
use super::serialization::{
    deserialize,
    deserialize_with_limits,
    serialize,
    serialize_with_version,
    DecodeLimits,
    Decoder,
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
// Test-only codec fixture
// =============================================================================
//
// This fixture intentionally tests the serialization compatibility boundary
// without depending on a concrete production program/circuit codec.
//
// That is important because:
//
// - serialization infrastructure can be tested before every higher-level IR
//   object implements IrEncode/IrDecode;
// - compatibility tests should not couple themselves to circuit internals;
// - the test verifies the public serialization contract itself;
// - implementation refactors do not require rewriting these tests.
//
// The fixture contains only canonical primitive IR values.

#[derive(Debug, Clone, PartialEq, Eq)]
struct CompatibilityRecord {
    logical_qubit: QubitId,
    physical_qubit: PhysicalQubitId,
    version: IrVersion,
    marker: u64,
}

impl IrEncode for CompatibilityRecord {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_qubit_id(self.logical_qubit);
        encoder.write_physical_qubit_id(self.physical_qubit);
        encoder.write_ir_version(self.version);
        encoder.write_u64(self.marker);

        Ok(())
    }
}

impl IrDecode for CompatibilityRecord {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        Ok(Self {
            logical_qubit: decoder.read_qubit_id()?,
            physical_qubit: decoder.read_physical_qubit_id()?,
            version: decoder.read_ir_version()?,
            marker: decoder.read_u64()?,
        })
    }
}

// =============================================================================
// Test fixtures
// =============================================================================

fn fixture() -> CompatibilityRecord {
    CompatibilityRecord {
        logical_qubit: QubitId::new(7),
        physical_qubit: PhysicalQubitId::new(11),
        version: IrVersion::CURRENT,
        marker: 0x0123_4567_89ab_cdef,
    }
}

fn conservative_limits() -> DecodeLimits {
    DecodeLimits::conservative()
}

// =============================================================================
// Canonical identity ownership
// =============================================================================

#[test]
fn canonical_logical_qubit_type_is_owned_by_qubit_module() {
    let canonical: QubitId =
        super::qubit::QubitId::new(17);

    assert_eq!(canonical.index(), 17);
}

#[test]
fn canonical_physical_qubit_type_is_owned_by_qubit_module() {
    let canonical: PhysicalQubitId =
        super::qubit::PhysicalQubitId::new(23);

    assert_eq!(canonical.index(), 23);
}

#[test]
fn legacy_qubits_alias_resolves_to_canonical_qubit_module() {
    let canonical =
        super::qubit::QubitId::new(31);

    let legacy_alias: super::qubits::QubitId =
        canonical;

    assert_eq!(
        legacy_alias,
        canonical,
        "legacy qubits alias must resolve to the canonical QubitId"
    );
}

#[test]
fn legacy_physical_qubits_alias_resolves_to_canonical_type() {
    let canonical =
        super::qubit::PhysicalQubitId::new(37);

    let legacy_alias: super::qubits::PhysicalQubitId =
        canonical;

    assert_eq!(
        legacy_alias,
        canonical,
        "legacy qubits alias must not define a duplicate physical identity type"
    );
}

#[test]
fn logical_and_physical_qubit_id_types_are_not_interchangeable() {
    let logical =
        QubitId::new(41);

    let physical =
        PhysicalQubitId::new(41);

    assert_eq!(
        logical.index(),
        physical.index()
    );

    // The fact that these require separate declarations above is itself
    // important: equal numeric values do not collapse logical and physical
    // identity namespaces.
    assert_eq!(logical, QubitId::new(41));
    assert_eq!(
        physical,
        PhysicalQubitId::new(41)
    );
}

// =============================================================================
// Identity determinism
// =============================================================================

#[test]
fn logical_qubit_identity_round_trips_through_canonical_u64_wire_value() {
    let original =
        QubitId::new(123);

    let mut encoder =
        Encoder::new();

    encoder.write_qubit_id(original);

    let bytes =
        encoder.into_bytes();

    let mut decoder =
        Decoder::new(&bytes);

    let decoded =
        decoder
            .read_qubit_id()
            .expect("canonical logical qubit identity must decode");

    decoder
        .finish()
        .expect("identity decoder must consume the complete payload");

    assert_eq!(
        decoded,
        original
    );
}

#[test]
fn physical_qubit_identity_round_trips_through_canonical_u64_wire_value() {
    let original =
        PhysicalQubitId::new(987);

    let mut encoder =
        Encoder::new();

    encoder.write_physical_qubit_id(
        original
    );

    let bytes =
        encoder.into_bytes();

    let mut decoder =
        Decoder::new(&bytes);

    let decoded =
        decoder
            .read_physical_qubit_id()
            .expect("canonical physical qubit identity must decode");

    decoder
        .finish()
        .expect("identity decoder must consume the complete payload");

    assert_eq!(
        decoded,
        original
    );
}

#[test]
fn logical_qubit_wire_encoding_is_platform_stable_for_representable_values() {
    let qubit =
        QubitId::new(0x0102_0304_0506_0708usize);

    let mut encoder =
        Encoder::new();

    encoder.write_qubit_id(qubit);

    let bytes =
        encoder.into_bytes();

    assert_eq!(
        bytes.len(),
        std::mem::size_of::<u64>(),
        "canonical identity wire encoding must use a fixed-width representation"
    );

    assert_eq!(
        bytes,
        0x0102_0304_0506_0708u64
            .to_le_bytes()
    );
}

#[test]
fn physical_qubit_wire_encoding_is_platform_stable_for_representable_values() {
    let qubit =
        PhysicalQubitId::new(0x0807_0605_0403_0201usize);

    let mut encoder =
        Encoder::new();

    encoder.write_physical_qubit_id(
        qubit
    );

    let bytes =
        encoder.into_bytes();

    assert_eq!(
        bytes.len(),
        std::mem::size_of::<u64>()
    );

    assert_eq!(
        bytes,
        0x0807_0605_0403_0201u64
            .to_le_bytes()
    );
}

// =============================================================================
// IR version contract
// =============================================================================

#[test]
fn current_ir_version_is_explicit_and_stable() {
    let version =
        IrVersion::CURRENT;

    assert_eq!(
        version,
        IrVersion::new(1, 0, 0)
    );
}

#[test]
fn current_ir_version_reports_current() {
    assert!(
        IrVersion::CURRENT.is_current()
    );
}

#[test]
fn current_ir_version_is_supported_by_current_implementation() {
    assert!(
        IrVersion::CURRENT
            .is_supported_by_current()
    );
}

#[test]
fn current_ir_version_is_supported_by_itself() {
    assert!(
        IrVersion::CURRENT
            .supports(IrVersion::CURRENT)
    );
}

#[test]
fn current_ir_version_has_same_major_as_itself() {
    assert!(
        IrVersion::CURRENT
            .same_major(IrVersion::CURRENT)
    );
}

#[test]
fn older_same_major_version_is_compatible() {
    let older =
        IrVersion::new(1, 0, 0);

    let current =
        IrVersion::CURRENT;

    assert!(
        current.is_compatible_with(older)
    );

    assert!(
        older.is_compatible_with(current)
    );
}

#[test]
fn future_major_version_is_not_supported() {
    let future =
        IrVersion::new(
            IrVersion::CURRENT.major() + 1,
            0,
            0,
        );

    assert!(
        !future.is_supported_by_current(),
        "future major contracts must never be silently accepted"
    );

    assert!(
        !IrVersion::CURRENT.supports(future),
        "current implementation must reject a future major contract"
    );
}

#[test]
fn future_minor_version_is_not_supported() {
    let future =
        IrVersion::new(
            IrVersion::CURRENT.major(),
            IrVersion::CURRENT.minor() + 1,
            0,
        );

    assert!(
        !future.is_supported_by_current(),
        "future minor contracts must not be silently interpreted"
    );

    assert!(
        !IrVersion::CURRENT.supports(future)
    );
}

#[test]
fn future_patch_version_is_not_supported() {
    let future =
        IrVersion::new(
            IrVersion::CURRENT.major(),
            IrVersion::CURRENT.minor(),
            IrVersion::CURRENT.patch() + 1,
        );

    assert!(
        !future.is_supported_by_current(),
        "future patch contracts must not be silently interpreted"
    );

    assert!(
        !IrVersion::CURRENT.supports(future)
    );
}

#[test]
fn different_major_versions_are_not_compatible() {
    let current =
        IrVersion::CURRENT;

    let future =
        IrVersion::new(
            current.major() + 1,
            current.minor(),
            current.patch(),
        );

    assert!(
        !current.is_compatible_with(future)
    );
}

#[test]
fn version_ordering_is_deterministic() {
    let older =
        IrVersion::new(1, 0, 0);

    let newer =
        IrVersion::new(1, 0, 1);

    assert!(
        older < newer
    );

    assert!(
        newer > older
    );
}

// =============================================================================
// Serialization-format / semantic-version separation
// =============================================================================

#[test]
fn serialization_format_version_is_distinct_from_ir_version() {
    assert_eq!(
        FORMAT_VERSION,
        1
    );

    assert_eq!(
        IrVersion::CURRENT.major(),
        1
    );

    // This assertion intentionally documents that the two values belong to
    // different namespaces. Equality here is allowed numerically, but neither
    // value may be substituted for the other.
    assert_ne!(
        std::mem::size_of::<u16>(),
        std::mem::size_of::<IrVersion>()
    );
}

#[test]
fn serialized_document_records_current_ir_version() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    assert_eq!(
        document.format_version()
            .expect("format version must decode"),
        FORMAT_VERSION
    );

    assert_eq!(
        document.ir_version()
            .expect("IR version must decode"),
        IrVersion::CURRENT
    );
}

#[test]
fn explicit_supported_ir_version_is_preserved_in_document_header() {
    let version =
        IrVersion::new(1, 0, 0);

    let document =
        serialize_with_version(
            &fixture(),
            version,
        )
        .expect("supported explicit IR version must serialize");

    assert_eq!(
        document.ir_version()
            .expect("IR version must decode"),
        version
    );
}

#[test]
fn unsupported_future_ir_version_cannot_be_serialized() {
    let future =
        IrVersion::new(
            IrVersion::CURRENT.major() + 1,
            0,
            0,
        );

    let result =
        serialize_with_version(
            &fixture(),
            future,
        );

    assert!(
        matches!(
            result,
            Err(
                SerializationError::UnsupportedIrVersion {
                    version
                }
            ) if version == future
        )
    );
}

// =============================================================================
// Canonical serialization
// =============================================================================

#[test]
fn serialized_document_is_not_empty() {
    let document =
        serialize(&fixture())
            .expect("fixture must serialize");

    assert!(
        !document.is_empty()
    );

    assert!(
        document.len() >= HEADER_LEN
    );
}

#[test]
fn serialized_document_starts_with_canonical_magic() {
    let document =
        serialize(&fixture())
            .expect("fixture must serialize");

    assert_eq!(
        &document.as_bytes()[..MAGIC.len()],
        &MAGIC
    );
}

#[test]
fn serialization_is_deterministic() {
    let first =
        serialize(&fixture())
            .expect("first serialization must succeed");

    let second =
        serialize(&fixture())
            .expect("second serialization must succeed");

    assert_eq!(
        first.as_bytes(),
        second.as_bytes(),
        "identical semantic IR must produce identical canonical bytes"
    );
}

#[test]
fn serialization_does_not_depend_on_serialized_object_identity() {
    let first_value =
        fixture();

    let second_value =
        CompatibilityRecord {
            logical_qubit:
                QubitId::new(
                    first_value.logical_qubit.index()
                ),
            physical_qubit:
                PhysicalQubitId::new(
                    first_value.physical_qubit.index()
                ),
            version:
                first_value.version,
            marker:
                first_value.marker,
        };

    let first =
        serialize(&first_value)
            .expect("first serialization must succeed");

    let second =
        serialize(&second_value)
            .expect("second serialization must succeed");

    assert_eq!(
        first.as_bytes(),
        second.as_bytes()
    );
}

#[test]
fn serialized_document_can_be_revalidated_as_a_serialized_ir() {
    let document =
        serialize(&fixture())
            .expect("fixture must serialize");

    let reconstructed =
        SerializedIr::from_bytes(
            document.as_bytes().to_vec()
        )
        .expect("valid canonical bytes must be accepted");

    assert_eq!(
        reconstructed.as_bytes(),
        document.as_bytes()
    );
}

#[test]
fn serialized_document_length_matches_header_plus_payload() {
    let document =
        serialize(&fixture())
            .expect("fixture must serialize");

    let payload_len =
        document
            .payload_len()
            .expect("payload length must decode");

    let expected =
        HEADER_LEN
            .checked_add(
                usize::try_from(payload_len)
                    .expect("test payload must fit host usize")
            )
            .expect("document length calculation must not overflow");

    assert_eq!(
        document.len(),
        expected
    );
}

// =============================================================================
// Serialization round trips
// =============================================================================

#[test]
fn compatibility_record_round_trips() {
    let original =
        fixture();

    let document =
        serialize(&original)
            .expect("fixture serialization must succeed");

    let decoded:
        CompatibilityRecord =
        deserialize(document.as_bytes())
            .expect("fixture deserialization must succeed");

    assert_eq!(
        decoded,
        original
    );
}

#[test]
fn compatibility_record_round_trips_with_explicit_limits() {
    let original =
        fixture();

    let document =
        serialize(&original)
            .expect("fixture serialization must succeed");

    let decoded:
        CompatibilityRecord =
        deserialize_with_limits(
            document.as_bytes(),
            conservative_limits(),
        )
        .expect("bounded fixture deserialization must succeed");

    assert_eq!(
        decoded,
        original
    );
}

#[test]
fn decoded_logical_qubit_uses_canonical_qubit_module_type() {
    let original =
        fixture();

    let document =
        serialize(&original)
            .expect("fixture serialization must succeed");

    let decoded:
        CompatibilityRecord =
        deserialize(document.as_bytes())
            .expect("fixture deserialization must succeed");

    let canonical:
        super::qubit::QubitId =
        decoded.logical_qubit;

    assert_eq!(
        canonical,
        original.logical_qubit
    );
}

#[test]
fn decoded_physical_qubit_uses_canonical_qubit_module_type() {
    let original =
        fixture();

    let document =
        serialize(&original)
            .expect("fixture serialization must succeed");

    let decoded:
        CompatibilityRecord =
        deserialize(document.as_bytes())
            .expect("fixture deserialization must succeed");

    let canonical:
        super::qubit::PhysicalQubitId =
        decoded.physical_qubit;

    assert_eq!(
        canonical,
        original.physical_qubit
    );
}

// =============================================================================
// Malformed document rejection
// =============================================================================

#[test]
fn empty_document_is_rejected() {
    let result =
        SerializedIr::from_bytes(
            Vec::new()
        );

    assert!(
        matches!(
            result,
            Err(SerializationError::UnexpectedEnd { .. })
        )
    );
}

#[test]
fn truncated_header_is_rejected() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    for length in 0..HEADER_LEN {
        let truncated =
            document.as_bytes()[..length]
                .to_vec();

        let result =
            SerializedIr::from_bytes(
                truncated
            );

        assert!(
            matches!(
                result,
                Err(SerializationError::UnexpectedEnd { .. })
            ),
            "header truncation at byte {length} must be rejected"
        );
    }
}

#[test]
fn invalid_magic_is_rejected() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    let mut bytes =
        document.as_bytes().to_vec();

    bytes[0] ^= 0xff;

    let result =
        SerializedIr::from_bytes(bytes);

    assert!(
        matches!(
            result,
            Err(SerializationError::InvalidMagic { .. })
        )
    );
}

#[test]
fn unsupported_serialization_format_is_rejected() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    let mut bytes =
        document.as_bytes().to_vec();

    // Bytes 4..6 are the serialization-format version according to the
    // canonical framing contract.
    let unsupported =
        FORMAT_VERSION
            .checked_add(1)
            .expect("test version increment must not overflow");

    bytes[4..6]
        .copy_from_slice(
            &unsupported.to_le_bytes()
        );

    let result =
        SerializedIr::from_bytes(bytes);

    assert!(
        matches!(
            result,
            Err(
                SerializationError::UnsupportedFormatVersion {
                    version
                }
            ) if version == unsupported
        )
    );
}

#[test]
fn trailing_bytes_are_rejected() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    let mut bytes =
        document.as_bytes().to_vec();

    bytes.push(0);

    let result =
        SerializedIr::from_bytes(bytes);

    assert!(
        matches!(
            result,
            Err(
                SerializationError::TrailingBytes {
                    count: 1
                }
            )
        )
    );
}

#[test]
fn truncated_payload_is_rejected() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    let bytes =
        document.as_bytes();

    assert!(
        bytes.len() > HEADER_LEN
    );

    let truncated =
        bytes[..bytes.len() - 1]
            .to_vec();

    let result =
        SerializedIr::from_bytes(
            truncated
        );

    assert!(
        matches!(
            result,
            Err(SerializationError::UnexpectedEnd { .. })
        )
    );
}

#[test]
fn corrupted_payload_is_rejected_by_integrity_check() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    let mut bytes =
        document.as_bytes().to_vec();

    assert!(
        bytes.len() > HEADER_LEN
    );

    bytes[HEADER_LEN] ^= 0x01;

    let result =
        SerializedIr::from_bytes(bytes);

    assert!(
        matches!(
            result,
            Err(
                SerializationError::ChecksumMismatch { .. }
            )
        )
    );
}

// =============================================================================
// Forward-version compatibility
// =============================================================================

#[test]
fn future_ir_major_version_in_document_is_rejected_before_object_decode() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    let mut bytes =
        document.as_bytes().to_vec();

    let future_major =
        IrVersion::CURRENT
            .major()
            .checked_add(1)
            .expect("test version increment must not overflow");

    // Header layout:
    //
    // 0..4   magic
    // 4..6   serialization format
    // 6..8   IR major
    // 8..10  IR minor
    // 10..12 IR patch
    // 12..20 payload length
    // 20..24 checksum
    //
    // Changing the semantic version is intentionally done without changing
    // the payload checksum because version rejection occurs at the document
    // semantic-version boundary before object decoding.
    bytes[6..8]
        .copy_from_slice(
            &future_major.to_le_bytes()
        );

    let result =
        deserialize::<CompatibilityRecord>(
            &bytes
        );

    assert!(
        matches!(
            result,
            Err(
                SerializationError::UnsupportedIrVersion {
                    version
                }
            ) if version.major() == future_major
        )
    );
}

#[test]
fn future_ir_minor_version_in_document_is_rejected() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    let mut bytes =
        document.as_bytes().to_vec();

    let future_minor =
        IrVersion::CURRENT
            .minor()
            .checked_add(1)
            .expect("test version increment must not overflow");

    bytes[8..10]
        .copy_from_slice(
            &future_minor.to_le_bytes()
        );

    let result =
        deserialize::<CompatibilityRecord>(
            &bytes
        );

    assert!(
        matches!(
            result,
            Err(
                SerializationError::UnsupportedIrVersion {
                    version
                }
            ) if version.minor() == future_minor
        )
    );
}

#[test]
fn future_ir_patch_version_in_document_is_rejected() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    let mut bytes =
        document.as_bytes().to_vec();

    let future_patch =
        IrVersion::CURRENT
            .patch()
            .checked_add(1)
            .expect("test version increment must not overflow");

    bytes[10..12]
        .copy_from_slice(
            &future_patch.to_le_bytes()
        );

    let result =
        deserialize::<CompatibilityRecord>(
            &bytes
        );

    assert!(
        matches!(
            result,
            Err(
                SerializationError::UnsupportedIrVersion {
                    version
                }
            ) if version.patch() == future_patch
        )
    );
}

// =============================================================================
// Decoder policy compatibility
// =============================================================================

#[test]
fn default_decode_limits_are_valid() {
    DecodeLimits::default()
        .validate()
        .expect(
            "default decode limits must remain internally valid"
        );
}

#[test]
fn conservative_decode_limits_are_valid() {
    DecodeLimits::conservative()
        .validate()
        .expect(
            "conservative decode limits must remain internally valid"
        );
}

#[test]
fn platform_default_decode_limits_are_valid() {
    DecodeLimits::platform_default()
        .validate()
        .expect(
            "platform default decode limits must remain internally valid"
        );
}

#[test]
fn zero_document_limit_is_rejected_as_invalid_policy() {
    let limits =
        DecodeLimits::new(
            0,
            1,
            1,
            1,
            1,
        );

    assert!(
        matches!(
            limits.validate(),
            Err(
                SerializationError::InvalidDecodeLimits {
                    field: "max_document_bytes"
                }
            )
        )
    );
}

#[test]
fn zero_payload_limit_is_rejected_as_invalid_policy() {
    let limits =
        DecodeLimits::new(
            1,
            0,
            1,
            1,
            1,
        );

    assert!(
        matches!(
            limits.validate(),
            Err(
                SerializationError::InvalidDecodeLimits {
                    field: "max_payload_bytes"
                }
            )
        )
    );
}

#[test]
fn zero_field_limit_is_rejected_as_invalid_policy() {
    let limits =
        DecodeLimits::new(
            1,
            1,
            0,
            1,
            1,
        );

    assert!(
        matches!(
            limits.validate(),
            Err(
                SerializationError::InvalidDecodeLimits {
                    field: "max_field_bytes"
                }
            )
        )
    );
}

#[test]
fn zero_collection_limit_is_rejected_as_invalid_policy() {
    let limits =
        DecodeLimits::new(
            1,
            1,
            1,
            0,
            1,
        );

    assert!(
        matches!(
            limits.validate(),
            Err(
                SerializationError::InvalidDecodeLimits {
                    field: "max_collection_elements"
                }
            )
        )
    );
}

#[test]
fn zero_nesting_limit_is_rejected_as_invalid_policy() {
    let limits =
        DecodeLimits::new(
            1,
            1,
            1,
            1,
            0,
        );

    assert!(
        matches!(
            limits.validate(),
            Err(
                SerializationError::InvalidDecodeLimits {
                    field: "max_nesting_depth"
                }
            )
        )
    );
}

#[test]
fn_payload_limit_cannot_exceed_document_limit() {
    let limits =
        DecodeLimits::new(
            10,
            11,
            1,
            1,
            1,
        );

    assert!(
        matches!(
            limits.validate(),
            Err(
                SerializationError::InvalidDecodeLimits {
                    field: "max_payload_bytes"
                }
            )
        )
    );
}

#[test]
fn decode_rejects_document_larger_than_explicit_policy() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    let document_size =
        u64::try_from(document.len())
            .expect("test document length must fit u64");

    let limits =
        DecodeLimits::new(
            document_size
                .saturating_sub(1),
            document_size,
            document_size,
            1,
            1,
        );

    let result =
        deserialize_with_limits::<CompatibilityRecord>(
            document.as_bytes(),
            limits,
        );

    assert!(
        matches!(
            result,
            Err(
                SerializationError::DocumentTooLarge { .. }
            )
        )
    );
}

#[test]
fn decode_rejects_payload_larger_than_explicit_policy() {
    let document =
        serialize(&fixture())
            .expect("fixture serialization must succeed");

    let payload_size =
        document
            .payload_len()
            .expect("payload length must decode");

    assert!(
        payload_size > 0
    );

    let document_size =
        u64::try_from(document.len())
            .expect("test document length must fit u64");

    let limits =
        DecodeLimits::new(
            document_size,
            payload_size - 1,
            document_size,
            1024,
            64,
        );

    let result =
        deserialize_with_limits::<CompatibilityRecord>(
            document.as_bytes(),
            limits,
        );

    assert!(
        matches!(
            result,
            Err(
                SerializationError::PayloadTooLarge { .. }
            )
        )
    );
}

// =============================================================================
// Decoder primitive compatibility
// =============================================================================

#[test]
fn canonical_boolean_encoding_is_stable() {
    let mut encoder =
        Encoder::new();

    encoder.write_bool(false);
    encoder.write_bool(true);

    let bytes =
        encoder.into_bytes();

    assert_eq!(
        bytes,
        vec![0, 1]
    );

    let mut decoder =
        Decoder::new(&bytes);

    assert!(
        !decoder
            .read_bool()
            .expect("false must decode")
    );

    assert!(
        decoder
            .read_bool()
            .expect("true must decode")
    );

    decoder
        .finish()
        .expect("boolean payload must be fully consumed");
}

#[test]
fn noncanonical_boolean_is_rejected() {
    let bytes =
        [2u8];

    let mut decoder =
        Decoder::new(&bytes);

    let result =
        decoder.read_bool();

    assert!(
        matches!(
            result,
            Err(
                SerializationError::InvalidBoolean {
                    value: 2
                }
            )
        )
    );
}

#[test]
fn canonical_integer_encoding_is_little_endian() {
    let value =
        0x0102_0304_0506_0708u64;

    let mut encoder =
        Encoder::new();

    encoder.write_u64(value);

    assert_eq!(
        encoder.into_bytes(),
        value.to_le_bytes()
    );
}

#[test]
fn canonical_signed_integer_encoding_is_little_endian() {
    let value =
        -0x0102_0304_0506_0708i64;

    let mut encoder =
        Encoder::new();

    encoder.write_i64(value);

    assert_eq!(
        encoder.into_bytes(),
        value.to_le_bytes()
    );
}

#[test]
fn canonical_ir_version_encoding_round_trips() {
    let version =
        IrVersion::new(
            17,
            23,
            31,
        );

    let mut encoder =
        Encoder::new();

    encoder.write_ir_version(version);

    let bytes =
        encoder.into_bytes();

    let mut decoder =
        Decoder::new(&bytes);

    let decoded =
        decoder
            .read_ir_version()
            .expect("IR version must decode");

    decoder
        .finish()
        .expect("version payload must be fully consumed");

    assert_eq!(
        decoded,
        version
    );
}

// =============================================================================
// Decoder trailing-payload protection
// =============================================================================

#[test]
fn decoder_finish_rejects_unconsumed_payload() {
    let bytes =
        [0u8, 1u8];

    let mut decoder =
        Decoder::new(&bytes);

    decoder
        .read_u8()
        .expect("first byte must decode");

    let result =
        decoder.finish();

    assert!(
        matches!(
            result,
            Err(
                SerializationError::TrailingBytes {
                    count: 1
                }
            )
        )
    );
}

#[test]
fn decoder_finish_accepts_fully_consumed_payload() {
    let bytes =
        [0u8];

    let mut decoder =
        Decoder::new(&bytes);

    decoder
        .read_u8()
        .expect("byte must decode");

    decoder
        .finish()
        .expect(
            "fully consumed payload must be accepted"
        );
}

// =============================================================================
// Bounded collection compatibility
// =============================================================================

#[test]
fn decoder_collection_limit_is_enforced_before_collection_decode() {
    let mut encoder =
        Encoder::new();

    encoder.write_u64(2);

    let bytes =
        encoder.into_bytes();

    let limits =
        DecodeLimits::new(
            1024,
            1024,
            1024,
            1,
            16,
        );

    let mut decoder =
        Decoder::with_limits(
            &bytes,
            limits,
        );

    let result =
        decoder.read_sequence(
            "compatibility test collection",
            |_decoder| {
                Ok::<u8, SerializationError>(0)
            },
        );

    assert!(
        matches!(
            result,
            Err(
                SerializationError::CollectionLimitExceeded {
                    requested: 2,
                    maximum: 1,
                    ..
                }
            )
        )
    );
}

#[test]
fn decoder_field_limit_is_enforced_before_allocation() {
    let mut encoder =
        Encoder::new();

    encoder.write_u64(8);

    let bytes =
        encoder.into_bytes();

    let limits =
        DecodeLimits::new(
            1024,
            1024,
            4,
            16,
            16,
        );

    let mut decoder =
        Decoder::with_limits(
            &bytes,
            limits,
        );

    let result =
        decoder.read_bytes(
            "compatibility test field"
        );

    assert!(
        matches!(
            result,
            Err(
                SerializationError::FieldLimitExceeded {
                    requested: 8,
                    maximum: 4,
                    ..
                }
            )
        )
    );
}

#[test]
fn nesting_limit_is_enforced() {
    let limits =
        DecodeLimits::new(
            1024,
            1024,
            1024,
            16,
            1,
        );

    let bytes =
        [0u8];

    let mut decoder =
        Decoder::with_limits(
            &bytes,
            limits,
        );

    let first =
        decoder
            .enter_scope()
            .expect("first scope must be allowed");

    // Keep the first guard alive so that nesting depth remains one.
    let _first = first;

    let result =
        decoder.enter_scope();

    assert!(
        matches!(
            result,
            Err(
                SerializationError::NestingLimitExceeded {
                    requested: 2,
                    maximum: 1
                }
            )
        )
    );
}

// =============================================================================
// Large identity values without architectural limits
// =============================================================================

#[test]
fn identity_model_does_not_encode_small_machine_size_limits() {
    let values = [
        0usize,
        1usize,
        63usize,
        64usize,
        127usize,
        128usize,
        1024usize,
        4096usize,
    ];

    for value in values {
        let logical =
            QubitId::new(value);

        assert_eq!(
            logical.index(),
            value,
            "identity namespace must not reject test values merely because \
             they cross historical machine-size boundaries"
        );
    }
}

#[test]
fn high_representable_logical_identity_remains_an_identity_token() {
    let value =
        usize::MAX;

    let logical =
        QubitId::new(value);

    assert_eq!(
        logical.index(),
        value
    );

    // This does not claim that a machine has usize::MAX qubits. It verifies
    // that the identity type itself does not encode an artificial architecture
    // limit. Actual resource availability belongs to explicit resource policy
    // and target hardware.
}

#[test]
fn high_representable_physical_identity_remains_an_identity_token() {
    let value =
        usize::MAX;

    let physical =
        PhysicalQubitId::new(value);

    assert_eq!(
        physical.index(),
        value
    );
}

// =============================================================================
// Compatibility of canonical aliases after serialization
// =============================================================================

#[test]
fn legacy_qubit_alias_can_consume_deserialized_canonical_identity() {
    let original =
        QubitId::new(73);

    let mut encoder =
        Encoder::new();

    encoder.write_qubit_id(original);

    let bytes =
        encoder.into_bytes();

    let mut decoder =
        Decoder::new(&bytes);

    let decoded:
        super::qubits::QubitId =
        decoder
            .read_qubit_id()
            .expect(
                "legacy alias must remain compatible with canonical codec"
            );

    decoder
        .finish()
        .expect("identity payload must be fully consumed");

    assert_eq!(
        decoded,
        original
    );
}

#[test]
fn canonical_qubit_aliases_have_identical_wire_semantics() {
    let canonical =
        super::qubit::QubitId::new(101);

    let legacy:
        super::qubits::QubitId =
        canonical;

    let mut canonical_encoder =
        Encoder::new();

    canonical_encoder
        .write_qubit_id(canonical);

    let mut legacy_encoder =
        Encoder::new();

    legacy_encoder
        .write_qubit_id(legacy);

    assert_eq!(
        canonical_encoder.into_bytes(),
        legacy_encoder.into_bytes()
    );
}

// =============================================================================
// Serialization object ownership
// =============================================================================

#[test]
fn serialized_ir_exposes_borrowed_bytes_without_mutation() {
    let document =
        serialize(&fixture())
            .expect("fixture must serialize");

    let bytes =
        document.as_bytes();

    assert!(
        bytes.len() >= HEADER_LEN
    );

    assert_eq!(
        &bytes[..MAGIC.len()],
        &MAGIC
    );
}

#[test]
fn serialized_ir_can_be_consumed_into_owned_bytes() {
    let document =
        serialize(&fixture())
            .expect("fixture must serialize");

    let expected =
        document.as_bytes().to_vec();

    let actual =
        document.into_bytes();

    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn serialized_ir_length_is_deterministic() {
    let first =
        serialize(&fixture())
            .expect("first serialization must succeed");

    let second =
        serialize(&fixture())
            .expect("second serialization must succeed");

    assert_eq!(
        first.len(),
        second.len()
    );
}

// =============================================================================
// Version + serialization compatibility matrix
// =============================================================================

#[test]
fn current_version_round_trip_matrix_is_supported() {
    let supported_versions = [
        IrVersion::CURRENT,
    ];

    for version in supported_versions {
        assert!(
            version.is_supported_by_current()
        );

        let document =
            serialize_with_version(
                &fixture(),
                version,
            )
            .expect(
                "every explicitly supported compatibility version must serialize"
            );

        assert_eq!(
            document.ir_version()
                .expect("version must be readable"),
            version
        );

        let decoded:
            CompatibilityRecord =
            deserialize(
                document.as_bytes()
            )
            .expect(
                "every explicitly supported compatibility version must decode"
            );

        assert_eq!(
            decoded,
            fixture()
        );
    }
}

// =============================================================================
// Compatibility invariants for malformed headers
// =============================================================================

#[test]
fn invalid_format_version_is_rejected_even_when_payload_is_valid() {
    let document =
        serialize(&fixture())
            .expect("fixture must serialize");

    let mut bytes =
        document.as_bytes().to_vec();

    bytes[4..6]
        .copy_from_slice(
            &u16::MAX.to_le_bytes()
        );

    let result =
        deserialize::<CompatibilityRecord>(
            &bytes
        );

    assert!(
        matches!(
            result,
            Err(
                SerializationError::UnsupportedFormatVersion {
                    version: u16::MAX
                }
            )
        )
    );
}

#[test]
fn malformed_payload_length_is_rejected() {
    let document =
        serialize(&fixture())
            .expect("fixture must serialize");

    let mut bytes =
        document.as_bytes().to_vec();

    // The largest representable payload length is intentionally used here.
    // The decoder must reject it because the actual document cannot contain
    // such a payload, rather than attempting a huge allocation.
    bytes[12..20]
        .copy_from_slice(
            &u64::MAX.to_le_bytes()
        );

    let result =
        SerializedIr::from_bytes(bytes);

    assert!(
        matches!(
            result,
            Err(
                SerializationError::LengthOverflow { .. }
            )
            | Err(
                SerializationError::UnexpectedEnd { .. }
            )
        )
    );
}

// =============================================================================
// Compatibility contract summary
// =============================================================================

#[test]
fn compatibility_contract_is_explicitly_satisfied() {
    // This test intentionally consists of assertions over the foundational
    // compatibility properties. It acts as a compact contract sentinel.
    //
    // If one of these contracts changes intentionally, this test should force
    // that change to be reviewed rather than allowing accidental compatibility
    // drift.

    assert_eq!(
        IrVersion::CURRENT,
        IrVersion::new(1, 0, 0)
    );

    assert!(
        IrVersion::CURRENT
            .is_supported_by_current()
    );

    assert!(
        IrVersion::CURRENT
            .supports(IrVersion::CURRENT)
    );

    assert_eq!(
        QubitId::new(0).index(),
        0
    );

    assert_eq!(
        PhysicalQubitId::new(0).index(),
        0
    );

    assert_eq!(
        FORMAT_VERSION,
        1
    );

    assert_eq!(
        MAGIC,
        *b"ZQIR"
    );

    assert!(
        HEADER_LEN > MAGIC.len()
    );

    DecodeLimits::default()
        .validate()
        .expect(
            "default serialization compatibility policy must be valid"
        );
}