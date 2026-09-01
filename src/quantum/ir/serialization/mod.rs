//! Zamani Quantum IR — Serialization Subsystem
//!
//! Production-grade public serialization boundary for the canonical Zamani
//! Quantum Intermediate Representation.
//!
//! # Purpose
//!
//! This module is the orchestration and public API layer for Quantum IR
//! serialization.
//!
//! It intentionally does NOT implement:
//!
//! - quantum semantics;
//! - gate semantics;
//! - qubit semantics;
//! - hardware;
//! - topology;
//! - routing;
//! - scheduling;
//! - optimization;
//! - simulation;
//! - QEC;
//! - frontend parsing;
//! - backend execution.
//!
//! Those responsibilities belong to their owning IR subsystems.
//!
//! # Architecture
//!
//! ```text
//!                  Zamani semantic IR
//!                         │
//!                         ▼
//!              ┌────────────────────┐
//!              │ serialization/mod  │
//!              │ public API/facade  │
//!              └─────────┬──────────┘
//!                        │
//!          ┌─────────────┼──────────────┐
//!          ▼             ▼              ▼
//!      encoder.rs    canonical.rs   decoder.rs
//!          │             │              │
//!          └─────────────┼──────────────┘
//!                        ▼
//!                 canonical bytes
//!                        │
//!          ┌─────────────┼──────────────┐
//!          ▼             ▼              ▼
//!        hashing      storage        transport
//! ```
//!
//! # Canonical contract
//!
//! A canonical serialized document consists of:
//!
//! ```text
//! ┌────────────────────────────────────────────┐
//! │ magic                                      │
//! │ serialization format version               │
//! │ semantic IR version                        │
//! │ payload length                             │
//! │ payload checksum                           │
//! │ canonical payload                          │
//! └────────────────────────────────────────────┘
//! ```
//!
//! The canonical framing is implemented by [`canonical`].
//!
//! This module only coordinates the framing with the semantic object codec.
//!
//! # Scalability
//!
//! This module contains NO fixed quantum-machine capacity.
//!
//! In particular it does not define:
//!
//! - maximum qubits;
//! - maximum logical qubits;
//! - maximum physical qubits;
//! - maximum gates;
//! - maximum operations;
//! - maximum registers;
//! - maximum circuit depth;
//! - maximum topology size;
//! - maximum hardware size.
//!
//! Serialization sizes are determined by the actual serialized object and by
//! an explicit caller-selected resource policy.
//!
//! Therefore the architecture is:
//!
//! ```text
//! one qubit
//!     │
//!     ├── same schema
//!     │
//!     ▼
//! millions/billions/... of resources
//!     │
//!     └── same schema
//! ```
//!
//! "Infinity" is interpreted correctly as:
//!
//! > no fixed quantum-machine-size limit in the semantic or serialization
//! > architecture; concrete executions remain finite and constrained only by
//! > representable identifiers, available storage/address space, and explicit
//! > resource policies.
//!
//! # Security
//!
//! Serialized Quantum IR is untrusted input whenever it crosses a persistence,
//! transport, cache, IPC, plugin, network, or user-controlled boundary.
//!
//! The serialization subsystem therefore guarantees that:
//!
//! - document framing is validated before semantic decoding;
//! - serialization format versions are checked;
//! - semantic IR versions are checked;
//! - lengths are checked before allocation;
//! - `u64` wire lengths are converted to `usize` only through checked
//!   conversions;
//! - collection limits are checked before allocation;
//! - field limits are checked before allocation;
//! - nesting limits are enforced;
//! - malformed booleans are rejected;
//! - malformed UTF-8 is rejected;
//! - invalid discriminants are rejected by owning codecs;
//! - checksum mismatches are rejected;
//! - truncated documents are rejected;
//! - trailing bytes are rejected;
//! - semantic codecs cannot silently ignore unread payload data;
//! - no unsafe code is used.
//!
//! # Determinism
//!
//! The serialization boundary is deterministic:
//!
//! ```text
//! same semantic IR
//! + same semantic IR version
//! + same serialization format
//! + same codec implementation
//! = identical canonical bytes
//! ```
//!
//! Deterministic serialization is required for:
//!
//! - canonical hashing;
//! - content-addressed storage;
//! - compiler caches;
//! - distributed compilation;
//! - provenance;
//! - reproducible builds;
//! - artifact identity;
//! - equality testing;
//! - cross-process transport.
//!
//! This module never arbitrarily sorts semantic sequences. Ordering belongs to
//! the owning IR type.
//!
//! # Versioning
//!
//! Three different concepts must remain independent:
//!
//! ```text
//! serialization format version
//!             ≠
//! semantic Quantum IR version
//!             ≠
//! compiler version
//! ```
//!
//! The serialization format version is owned by [`canonical`].
//!
//! The semantic IR version is owned by `quantum::ir::identity::IrVersion`.
//!
//! Migration and compatibility policy belongs to [`compatibility`].
//!
//! # Quantum identity boundary
//!
//! Quantum identities remain owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module never defines duplicate qubit identity types.
//!
//! # Integration rule
//!
//! Higher-level IR modules implement [`IrEncode`] and [`IrDecode`] locally.
//!
//! They should depend on:
//!
//! ```text
//! quantum::ir::serialization::Encoder
//! quantum::ir::serialization::Decoder
//! quantum::ir::serialization::IrEncode
//! quantum::ir::serialization::IrDecode
//! quantum::ir::serialization::SerializationError
//! ```
//!
//! They must NOT depend on:
//!
//! - hardware;
//! - routing;
//! - scheduling;
//! - backend implementations;
//! - simulators;
//! - frontend implementations.
//!
//! # Rust
//!
//! Compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler
//! enforced.

#![forbid(unsafe_code)]

// ============================================================================
// Submodules
// ============================================================================
//
// These modules have strictly separated responsibilities.
//
// canonical.rs
//     Document framing and canonical wire primitives.
//
// encoder.rs
//     Semantic payload encoding.
//
// decoder.rs
//     Semantic payload decoding.
//
// schema.rs
//     Serialization schema identity and schema-level contracts.
//
// compatibility.rs
//     Version compatibility and migration policy.
//
// serialization.rs
//     LEGACY implementation. It must NOT be declared here.
//
// Once this mod.rs is integrated, the old
// `src/quantum/ir/serialization/serialization.rs` should not be compiled.
// It is retained temporarily only as migration/reference material.

pub mod canonical;
pub mod compatibility;
pub mod decoder;
pub mod encoder;
pub mod schema;

// ============================================================================
// Stable public re-exports
// ============================================================================

pub use canonical::{
    decode_document,
    decode_document_with_limits,
    encode_document,
    CanonicalDocument,
    CanonicalError,
    DecodeLimits,
    FORMAT_VERSION,
    HEADER_LEN,
    MAGIC,
};

pub use decoder::Decoder;

pub use encoder::{
    EncodeLimits,
    Encoder,
};

pub use schema::*;

// ============================================================================
// Compatibility names
// ============================================================================
//
// Existing IR code uses SerializationError and SerializationResult.
// Keep those names stable while the actual canonical structural error type
// remains owned by canonical.rs.
//
// This prevents every downstream IR module from needing a simultaneous API
// migration.

/// Canonical error type used by the Quantum IR serialization subsystem.
pub type SerializationError = CanonicalError;

/// Standard result type for Quantum IR serialization operations.
pub type SerializationResult<T> = Result<T, SerializationError>;

// ============================================================================
// Codec traits
// ============================================================================

/// Trait implemented by a semantic Quantum IR object that can be encoded into
/// the canonical serialization payload.
///
/// # Ownership
///
/// The implementing IR module owns:
///
/// - semantic field ordering;
/// - semantic validation;
/// - operation/type-specific encoding;
/// - extension handling.
///
/// This subsystem owns only the serialization mechanism.
///
/// # Determinism
///
/// Implementations MUST encode fields in a deterministic semantic order.
///
/// Implementations MUST NOT:
///
/// - use memory addresses;
/// - use pointer values;
/// - use process IDs;
/// - use random values;
/// - depend on unordered hash-map iteration;
/// - omit unknown semantic information silently.
///
/// # Scalability
///
/// Implementations MUST NOT introduce fixed machine-size assumptions such as:
///
/// ```text
/// 32 qubits
/// 64 qubits
/// 128 qubits
/// 4096 operations
/// ```
///
/// Resource restrictions belong to explicit `EncodeLimits` or higher-level
/// compiler policies.
///
/// # Errors
///
/// An implementation must return [`SerializationError`] rather than silently
/// truncating or ignoring information.
pub trait IrEncode {
    /// Encodes this semantic object into the canonical payload encoder.
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()>;
}

/// Trait implemented by a semantic Quantum IR object that can be reconstructed
/// from the canonical serialization payload.
///
/// # Security
///
/// Implementations MUST treat the input as untrusted.
///
/// Lengths must be read through [`Decoder`] rather than interpreted manually.
///
/// # Semantic validation
///
/// Successful decoding establishes structural validity only.
///
/// Callers must still perform semantic IR validation through the canonical
/// validation subsystem after reconstruction.
pub trait IrDecode: Sized {
    /// Decodes one semantic object from the canonical payload.
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self>;
}

// ============================================================================
// Serialized artifact
// ============================================================================

/// Owned canonical Quantum IR serialization artifact.
///
/// This wrapper represents the exact bytes exchanged between:
///
/// - persistence;
/// - caches;
/// - compiler stages;
/// - distributed compilation;
/// - transport;
/// - hashing;
/// - artifact storage.
///
/// The wrapper contains no semantic interpretation.
///
/// # Important
///
/// The bytes are already canonical serialized bytes. They MUST NOT be modified
/// after construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializedIr {
    bytes: Vec<u8>,
}

impl SerializedIr {
    /// Constructs an artifact from canonical serialized bytes.
    ///
    /// The complete document is structurally validated before ownership is
    /// accepted.
    ///
    /// This prevents arbitrary bytes from being labelled as canonical IR.
    pub fn from_bytes(bytes: Vec<u8>) -> SerializationResult<Self> {
        decode_document(&bytes)?;

        Ok(Self { bytes })
    }

    /// Constructs an artifact using an explicit decode/resource policy.
    ///
    /// This is useful when the caller deliberately chooses a larger or smaller
    /// deployment-specific policy.
    pub fn from_bytes_with_limits(
        bytes: Vec<u8>,
        limits: DecodeLimits,
    ) -> SerializationResult<Self> {
        decode_document_with_limits(&bytes, limits)?;

        Ok(Self { bytes })
    }

    /// Borrows the exact canonical bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the serialized document size.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the serialized document is empty.
    ///
    /// A valid canonical IR document can never be empty, so this is primarily
    /// a convenience API.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consumes the wrapper and returns the exact canonical bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl AsRef<[u8]> for SerializedIr {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

// ============================================================================
// Encoding API
// ============================================================================

/// Serializes an IR object using the current semantic IR version.
///
/// This is the normal production entry point.
///
/// The result is deterministic for a deterministic semantic object.
pub fn serialize<T: IrEncode>(
    value: &T,
) -> SerializationResult<Vec<u8>> {
    serialize_with_version(value, crate::quantum::ir::identity::IrVersion::CURRENT)
}

/// Serializes an IR object using an explicitly selected semantic IR version.
///
/// The caller is responsible for selecting a version whose semantic contract
/// is actually implemented by the object codec.
///
/// This function deliberately does not inspect the semantic object because
/// semantic ownership belongs to the implementing IR type.
pub fn serialize_with_version<T: IrEncode>(
    value: &T,
    ir_version: crate::quantum::ir::identity::IrVersion,
) -> SerializationResult<Vec<u8>> {
    let mut encoder = Encoder::default();

    value.encode(&mut encoder)?;

    let payload = encoder.into_bytes();

    encode_document(ir_version, &payload)
}

/// Serializes an IR object with an explicit encoder resource policy.
///
/// This is the preferred API for services, embedded systems, distributed
/// workers, and other environments where allocation policy is externally
/// controlled.
pub fn serialize_with_limits<T: IrEncode>(
    value: &T,
    limits: EncodeLimits,
) -> SerializationResult<Vec<u8>> {
    serialize_with_version_and_limits(
        value,
        crate::quantum::ir::identity::IrVersion::CURRENT,
        limits,
    )
}

/// Serializes an IR object with both an explicit semantic version and explicit
/// encoding resource policy.
pub fn serialize_with_version_and_limits<T: IrEncode>(
    value: &T,
    ir_version: crate::quantum::ir::identity::IrVersion,
    limits: EncodeLimits,
) -> SerializationResult<Vec<u8>> {
    let mut encoder = Encoder::with_limits(limits)?;

    value.encode(&mut encoder)?;

    let payload = encoder.into_bytes();

    encode_document(ir_version, &payload)
}

/// Serializes an IR object and returns an owned canonical artifact wrapper.
pub fn serialize_artifact<T: IrEncode>(
    value: &T,
) -> SerializationResult<SerializedIr> {
    let bytes = serialize(value)?;

    SerializedIr::from_bytes(bytes)
}

/// Serializes an IR object with an explicit encoding policy and returns an
/// owned canonical artifact wrapper.
pub fn serialize_artifact_with_limits<T: IrEncode>(
    value: &T,
    limits: EncodeLimits,
) -> SerializationResult<SerializedIr> {
    let bytes = serialize_with_limits(value, limits)?;

    SerializedIr::from_bytes_with_limits(
        bytes,
        DecodeLimits::default(),
    )
}

// ============================================================================
// Decoding API
// ============================================================================

/// Deserializes a canonical Quantum IR document using the default decode
/// policy.
///
/// Structural validation occurs before the semantic codec is invoked.
pub fn deserialize<T: IrDecode>(
    document: &[u8],
) -> SerializationResult<T> {
    deserialize_with_limits(document, DecodeLimits::default())
}

/// Deserializes a canonical Quantum IR document with an explicit resource
/// policy.
///
/// This is the preferred entry point for large programs and resource-aware
/// compilation services.
pub fn deserialize_with_limits<T: IrDecode>(
    document: &[u8],
    limits: DecodeLimits,
) -> SerializationResult<T> {
    let canonical = decode_document_with_limits(document, limits)?;

    let encoded_version = canonical.ir_version();

    //
    // The canonical framing layer validates the wire representation.
    // The public facade additionally enforces the current semantic compatibility
    // contract before invoking an object decoder.
    //
    let current_version =
        crate::quantum::ir::identity::IrVersion::CURRENT;

    if !encoded_version.is_compatible_with(current_version) {
        return Err(SerializationError::UnsupportedIrVersion {
            version: encoded_version,
        });
    }

    let mut decoder = Decoder::with_limits(canonical.payload(), limits);

    let value = T::decode(&mut decoder)?;

    //
    // This is critical:
    //
    // successful decoding of a prefix is not successful decoding of a document.
    //
    // Requiring finish() prevents an object codec from accidentally accepting:
    //
    // valid_object + malicious_extra_payload
    //
    // as a valid object.
    //
    decoder.finish()?;

    Ok(value)
}

/// Decodes only the canonical document envelope.
///
/// This API is useful to storage, cache, transport, and compatibility layers
/// that need to inspect version and payload framing without constructing a
/// semantic IR object.
pub fn inspect(
    document: &[u8],
) -> SerializationResult<CanonicalDocument<'_>> {
    decode_document(document)
}

/// Decodes only the canonical document envelope using an explicit resource
/// policy.
pub fn inspect_with_limits(
    document: &[u8],
    limits: DecodeLimits,
) -> SerializationResult<CanonicalDocument<'_>> {
    decode_document_with_limits(document, limits)
}

// ============================================================================
// Validation helpers
// ============================================================================

/// Validates the structural canonical serialization of a document.
///
/// This does NOT perform semantic Quantum IR validation.
///
/// Use the canonical IR validation subsystem after deserialization for:
///
/// - type correctness;
/// - operation correctness;
/// - control-flow correctness;
/// - resource correctness;
/// - timing correctness;
/// - pulse correctness;
/// - model-specific invariants.
pub fn validate_document(
    document: &[u8],
) -> SerializationResult<()> {
    decode_document(document)?;
    Ok(())
}

/// Validates structural canonical serialization using an explicit policy.
pub fn validate_document_with_limits(
    document: &[u8],
    limits: DecodeLimits,
) -> SerializationResult<()> {
    decode_document_with_limits(document, limits)?;
    Ok(())
}

// ============================================================================
// Canonical version helpers
// ============================================================================

/// Returns the serialization framing version.
#[must_use]
pub const fn format_version() -> u16 {
    FORMAT_VERSION
}

/// Returns the fixed canonical document header size.
#[must_use]
pub const fn header_len() -> usize {
    HEADER_LEN
}

/// Returns the canonical document magic.
#[must_use]
pub const fn magic() -> [u8; 4] {
    MAGIC
}

/// Returns the current semantic Quantum IR version.
#[must_use]
pub const fn current_ir_version(
) -> crate::quantum::ir::identity::IrVersion {
    crate::quantum::ir::identity::IrVersion::CURRENT
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestObject {
        version: crate::quantum::ir::identity::IrVersion,
        logical_qubit: crate::quantum::ir::qubit::QubitId,
        physical_qubit: crate::quantum::ir::qubit::PhysicalQubitId,
    }

    impl IrEncode for TestObject {
        fn encode(
            &self,
            encoder: &mut Encoder,
        ) -> SerializationResult<()> {
            //
            // The exact primitive methods belong to encoder.rs.
            //
            // Keeping the semantic object responsible for field ordering
            // means this facade never needs to be edited when a new IR type
            // appears.
            //
            encoder.write_ir_version(self.version)?;
            encoder.write_qubit_id(self.logical_qubit)?;
            encoder.write_physical_qubit_id(self.physical_qubit)?;

            Ok(())
        }
    }

    impl IrDecode for TestObject {
        fn decode(
            decoder: &mut Decoder<'_>,
        ) -> SerializationResult<Self> {
            Ok(Self {
                version: decoder.read_ir_version()?,
                logical_qubit: decoder.read_qubit_id()?,
                physical_qubit: decoder.read_physical_qubit_id()?,
            })
        }
    }

    #[test]
    fn canonical_constants_are_stable() {
        assert_eq!(MAGIC, *b"ZQIR");
        assert_eq!(HEADER_LEN, 24);
        assert_eq!(FORMAT_VERSION, 1);
    }

    #[test]
    fn current_ir_version_is_available() {
        assert_eq!(
            current_ir_version(),
            crate::quantum::ir::identity::IrVersion::CURRENT
        );
    }

    #[test]
    fn artifact_rejects_invalid_bytes() {
        let result = SerializedIr::from_bytes(vec![0, 1, 2, 3]);

        assert!(result.is_err());
    }

    #[test]
    fn empty_document_is_invalid() {
        assert!(validate_document(&[]).is_err());
    }

    #[test]
    fn serialized_artifact_exposes_exact_bytes() {
        let artifact = SerializedIr::from_bytes(vec![
            b'Z', b'Q', b'I', b'R',
            1, 0,
            1, 0,
            0, 0,
            0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ]);

        assert!(artifact.is_err());
    }
}