//! Zamani Quantum Scheduling — Serialization Boundary
//!
//! Production-grade public serialization facade for the canonical Zamani
//! quantum scheduling subsystem.
//!
//! # Purpose
//!
//! This module is the stable orchestration boundary for scheduling
//! serialization. It connects:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! routing / optimization
//!      │
//!      ▼
//! quantum::scheduling
//!      │
//!      ▼
//! ScheduleResult / scheduling schema
//!      │
//!      ▼
//! scheduling::serialization::mod
//!      │
//!      ├───────────────┐
//!      ▼               ▼
//!   encode           decode
//!      │               │
//!      └───────┬───────┘
//!              ▼
//!       canonical artifact
//! ```
//!
//! This module deliberately does NOT own:
//!
//! - quantum semantics;
//! - `QuantumCircuit`;
//! - `QuantumOperation`;
//! - `Gate`;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - routing;
//! - optimization;
//! - scheduling algorithms;
//! - hardware discovery;
//! - QEC algorithms;
//! - simulation;
//! - runtime execution;
//! - vendor SDKs;
//! - authentication;
//! - encryption;
//! - compression.
//!
//! Those responsibilities remain with their owning subsystems.
//!
//! # Design goals
//!
//! The serialization facade is designed for:
//!
//! - one-qubit programs;
//! - large quantum programs;
//! - multi-QPU systems;
//! - distributed quantum systems;
//! - QEC schedules;
//! - dynamic circuits;
//! - resource-aware schedules;
//! - deterministic builds;
//! - compiler caches;
//! - checkpointing;
//! - distributed compilation;
//! - artifact transport;
//! - long-term persistence.
//!
//! There is intentionally no machine-size constant in this module.
//!
//! In particular, this module does not define:
//!
//! - maximum qubits;
//! - maximum physical qubits;
//! - maximum operations;
//! - maximum resources;
//! - maximum schedule depth;
//! - maximum QEC rounds;
//! - maximum channels;
//! - maximum distributed nodes.
//!
//! "Scale to infinity" is interpreted as:
//!
//! > the serialization architecture imposes no artificial finite quantum
//! > machine-size ceiling; concrete serialized objects remain finite and are
//! > ultimately constrained by available memory, address space, storage,
//! > transport capacity, and explicit caller-selected resource policies.
//!
//! # Canonical quantum identity boundary
//!
//! Quantum identities are owned by the canonical Quantum IR:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module never creates replacement qubit identity types.
//!
//! Scheduling serialization may represent those identities using stable wire
//! representations defined by `schema.rs`, but semantic ownership remains with
//! `quantum::ir::qubit`.
//!
//! # Version separation
//!
//! The following versions are independent:
//!
//! ```text
//! Zamani language version
//!        ≠
//! Quantum IR version
//!        ≠
//! scheduling schema version
//!        ≠
//! binary serialization format version
//!        ≠
//! scheduler algorithm version
//!        ≠
//! hardware version
//!        ≠
//! calibration version
//! ```
//!
//! This prevents changing a scheduler implementation from silently changing
//! the interpretation of previously persisted scheduling artifacts.
//!
//! # Security model
//!
//! Serialized schedules must be treated as untrusted whenever they cross:
//!
//! - a filesystem boundary;
//! - a cache boundary;
//! - IPC;
//! - a network;
//! - a plugin boundary;
//! - a distributed worker boundary;
//! - a checkpoint boundary;
//! - a user-controlled input boundary.
//!
//! The public facade therefore follows this invariant:
//!
//! ```text
//! bytes
//!   │
//!   ▼
//! framing validation
//!   │
//!   ▼
//! version validation
//!   │
//!   ▼
//! resource/length validation
//!   │
//!   ▼
//! payload decoding
//!   │
//!   ▼
//! schema validation
//!   │
//!   ▼
//! semantic scheduling validation
//! ```
//!
//! No decoded schedule is executable merely because deserialization succeeded.
//!
//! # Determinism
//!
//! Canonical serialization must satisfy:
//!
//! ```text
//! same schedule
//! + same schema version
//! + same serialization format
//! + same encoder implementation
//! = identical canonical bytes
//! ```
//!
//! This is required for:
//!
//! - reproducible compilation;
//! - content-addressed storage;
//! - cache keys;
//! - distributed compilation;
//! - provenance;
//! - regression testing;
//! - artifact comparison.
//!
//! Semantic collection ordering remains owned by the semantic type. This facade
//! does not arbitrarily reorder schedule operations, dependencies, reservations,
//! resources, or qubits.
//!
//! # No-unsafe policy
//!
//! The complete scheduling serialization boundary is memory-safe.
//!
//! No operation in this module requires unsafe Rust.
//!
//! The compiler enforces that requirement.
//!
//! # Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! is intentionally present below.
//!
//! # Module ownership
//!
//! ```text
//! schema.rs
//!     WHAT a scheduling artifact means.
//!
//! encode.rs
//!     HOW a scheduling document becomes bytes.
//!
//! decode.rs
//!     HOW bytes become a scheduling document.
//!
//! mod.rs
//!     HOW those components are exposed as one stable public API.
//! ```
//!
//! The facade must remain intentionally thin.
//!
//! ---------------------------------------------------------------------------
//! Integration contract
//! ---------------------------------------------------------------------------
//!
//! Normal producer:
//!
//! ```text
//! quantum::scheduling::ScheduleResult
//!             │
//!             ▼
//! scheduling schema conversion
//!             │
//!             ▼
//! serialization::encode
//!             │
//!             ▼
//! SerializedSchedule
//! ```
//!
//! Normal consumer:
//!
//! ```text
//! bytes
//!   │
//!   ▼
//! serialization::decode
//!   │
//!   ▼
//! scheduling schema
//!   │
//!   ▼
//! scheduling validation
//!   │
//!   ▼
//! quantum::scheduling
//! ```
//!
//! Hardware, routing, QEC, runtime, and frontend code must not directly depend
//! on the binary framing implementation.
//!
//! ---------------------------------------------------------------------------
//! Compatibility with the existing Quantum IR serialization architecture
//! ---------------------------------------------------------------------------
//!
//! The repository's canonical Quantum IR serialization already establishes a
//! public facade around canonical encoding/decoding, explicit versioning,
//! checked lengths, deterministic serialization, and untrusted-input
//! validation. The scheduling serialization facade intentionally follows that
//! architecture rather than creating an incompatible serialization model.
//!
//! ---------------------------------------------------------------------------
//! Migration policy
//! ---------------------------------------------------------------------------
//!
//! Older scheduling serialization implementations may temporarily remain in
//! the repository as migration/reference material, but they must not be
//! re-exported from this module once the production implementation is active.
//!
//! The facade should expose exactly one canonical scheduling serialization
//! contract.
//!
//! ---------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// ============================================================================
// Submodules
// ============================================================================
//
// Each submodule owns exactly one serialization concern.
//
// schema.rs
//     Versioned scheduling data contract.
//
// encode.rs
//     Canonical encoding mechanism.
//
// decode.rs
//     Canonical decoding mechanism.
//
// Keeping these boundaries independent means that changing a schema field,
// adding a new decoder limit, or improving an encoder implementation does not
// require rewriting this facade.
//
// ============================================================================

pub mod decode;
pub mod encode;
pub mod schema;

// ============================================================================
// Stable public re-exports
// ============================================================================
//
// Re-export only stable public contracts.
//
// Internal implementation details should remain behind their owning module.
// This keeps downstream code independent of the physical file layout.
//
// ============================================================================

pub use decode::{
    decode,
    decode_with_limits,
    DecodeError,
    DecodeLimits,
};

pub use encode::{
    encode,
    encode_canonical_json,
    encode_to_writer,
    canonical_payload_digest,
    EncodeError,
    EncodeOptions,
    EncodedArtifact,
};

pub use schema::{
    SchemaCompatibility,
    SchemaVersion,
    SerializedClassicalDependencyId,
    SerializedDependencyId,
    SerializedId,
    SerializedOperationId,
    SerializedPhysicalQubitId,
    SerializedQubitId,
    SerializedReservationId,
    SerializedResourceId,
    SerializedTime,
    SerializedTimeInterval,
    TimeUnit,
    SCHEDULING_SCHEMA_MAJOR,
    SCHEDULING_SCHEMA_MINOR,
    SCHEDULING_SCHEMA_PATCH,
    SCHEDULING_SCHEMA_VERSION,
};

// ============================================================================
// Stable aliases
// ============================================================================
//
// These aliases provide one vocabulary for callers while allowing the
// underlying implementation to remain owned by the schema/codec modules.
//
// ============================================================================

/// Stable scheduling serialization error.
///
/// This is intentionally an alias rather than a second error hierarchy.
/// Encoding and decoding errors remain owned by their respective mechanisms.
pub type SerializationError = SerializationErrorKind;

/// Unified scheduling serialization error.
///
/// Keeping the variants explicit prevents callers from depending on
/// implementation-specific strings.
#[derive(Debug)]
pub enum SerializationErrorKind {
    /// Encoding failed.
    Encode(EncodeError),

    /// Decoding failed.
    Decode(DecodeError),
}

impl std::fmt::Display for SerializationErrorKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encode(error) => write!(formatter, "{error}"),
            Self::Decode(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for SerializationErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Encode(error) => Some(error),
            Self::Decode(error) => Some(error),
        }
    }
}

impl From<EncodeError> for SerializationErrorKind {
    fn from(error: EncodeError) -> Self {
        Self::Encode(error)
    }
}

impl From<DecodeError> for SerializationErrorKind {
    fn from(error: DecodeError) -> Self {
        Self::Decode(error)
    }
}

/// Standard scheduling serialization result.
pub type SerializationResult<T> = Result<T, SerializationError>;

// ============================================================================
// Canonical serialized artifact
// ============================================================================

/// Owned canonical serialized scheduling artifact.
///
/// This wrapper represents bytes that have already passed the structural
/// canonical serialization contract.
///
/// The wrapper is intentionally opaque with respect to semantic scheduling
/// data. Semantic interpretation belongs to the decoder and schema layers.
///
/// # Invariants
///
/// A `SerializedSchedule` created by [`SerializedSchedule::from_bytes`] has
/// passed the canonical decoder's default structural checks.
///
/// A value created by [`SerializedSchedule::from_bytes_with_limits`] has
/// passed the caller-provided decode policy.
///
/// The contained bytes are never modified by this type.
///
/// # Scalability
///
/// No scheduler-size limit is embedded in this wrapper.
///
/// The actual byte vector is necessarily finite because Rust values are finite,
/// but there is no artificial qubit, operation, resource, or schedule-depth
/// ceiling.
///
/// # Security
///
/// This wrapper does not make bytes executable or trusted semantic input.
/// Consumers must still decode and semantically validate the resulting
/// scheduling object before execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedSchedule {
    bytes: Vec<u8>,
}

impl SerializedSchedule {
    /// Constructs a serialized schedule from canonical bytes.
    ///
    /// The bytes are structurally validated before the wrapper is returned.
    ///
    /// This is the safe default for persisted, transported, cached, or
    /// user-supplied scheduling artifacts.
    pub fn from_bytes(bytes: Vec<u8>) -> SerializationResult<Self> {
        decode(&bytes)?;

        Ok(Self { bytes })
    }

    /// Constructs a serialized schedule using an explicit decoding policy.
    ///
    /// This API is intended for environments where allocation and input-size
    /// policies are deliberately controlled by the caller.
    pub fn from_bytes_with_limits(
        bytes: Vec<u8>,
        limits: DecodeLimits,
    ) -> SerializationResult<Self> {
        decode_with_limits(&bytes, limits)?;

        Ok(Self { bytes })
    }

    /// Borrows the exact serialized bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the number of serialized bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the artifact contains zero bytes.
    ///
    /// A valid canonical scheduling artifact is expected to be non-empty.
    /// This method exists as a conventional container API.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consumes the artifact and returns its exact bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl AsRef<[u8]> for SerializedSchedule {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

// ============================================================================
// Canonical encode facade
// ============================================================================

/// Encodes a scheduling schema value into the canonical serialized artifact.
///
/// This is the preferred high-level encoding entry point.
///
/// The supplied value is responsible for representing valid scheduling
/// semantics. The encoding layer is responsible for deterministic
/// representation and framing.
///
/// # Determinism
///
/// For deterministic schema values and deterministic encoder behavior, the
/// result is deterministic.
///
/// # Security
///
/// Encoding never executes schedule operations and never contacts hardware.
///
/// # Scalability
///
/// No machine-size assumptions are introduced here.
///
/// # Integration
///
/// ```text
/// ScheduleResult
///      │
///      ▼
/// schema representation
///      │
///      ▼
/// serialization::encode
///      │
///      ▼
/// Vec<u8>
/// ```
pub fn serialize<T>(value: &T) -> SerializationResult<Vec<u8>>
where
    T: serde::Serialize,
{
    encode(value).map_err(SerializationErrorKind::Encode)
}

/// Encodes a scheduling schema value with explicit encoder options.
pub fn serialize_with_options<T>(
    value: &T,
    options: EncodeOptions,
) -> SerializationResult<EncodedArtifact>
where
    T: serde::Serialize,
{
    encode::encode_with_options(value, options)
        .map_err(SerializationErrorKind::Encode)
}

/// Encodes canonical JSON without the binary scheduling frame.
///
/// This is useful for:
///
/// - diagnostics;
/// - human inspection;
/// - interoperability;
/// - hashing;
/// - debugging;
/// - schema inspection.
///
/// It is not a replacement for the complete framed scheduling artifact.
pub fn serialize_canonical_json<T>(
    value: &T,
) -> SerializationResult<Vec<u8>>
where
    T: serde::Serialize,
{
    encode_canonical_json(value)
        .map_err(SerializationErrorKind::Encode)
}

/// Computes the canonical payload digest.
///
/// The digest is over the canonical payload, not over transport-specific
/// framing.
///
/// This makes the digest stable across transport/storage mechanisms that use
/// the same canonical payload.
pub fn canonical_digest<T>(
    value: &T,
) -> SerializationResult<[u8; 32]>
where
    T: serde::Serialize,
{
    canonical_payload_digest(value)
        .map_err(SerializationErrorKind::Encode)
}

/// Encodes directly into a caller-owned writer.
///
/// The serialization layer does not open files, sockets, or other resources by
/// itself. The caller owns the destination and therefore owns its I/O policy.
///
/// This is important for:
///
/// - large schedules;
//! - streaming persistence;
//! - distributed transport;
//! - bounded-memory services;
//! - embedded environments.
///
/// The implementation must never assume that the complete artifact can be
/// held in memory merely because this facade exists.
pub fn serialize_to_writer<T, W>(
    value: &T,
    writer: &mut W,
) -> SerializationResult<encode::EncodeStatistics>
where
    T: serde::Serialize,
    W: std::io::Write,
{
    encode_to_writer(value, writer)
        .map_err(SerializationErrorKind::Encode)
}

// ============================================================================
// Canonical decode facade
// ============================================================================

/// Decodes a complete scheduling serialization artifact into the requested
/// schema type.
///
/// Structural validation is performed before semantic reconstruction.
///
/// # Security
///
/// Input is treated as untrusted.
///
/// # Integration
///
/// ```text
/// bytes
///   │
///   ▼
/// deserialize::<SchedulingDocument>()
///   │
///   ▼
/// schema object
///   │
///   ▼
/// scheduling semantic validation
/// ```
pub fn deserialize<T>(
    document: &[u8],
) -> SerializationResult<T>
where
    T: serde::de::DeserializeOwned,
{
    decode(document)
        .map_err(SerializationErrorKind::Decode)
}

/// Decodes a scheduling artifact with an explicit resource/input policy.
///
/// This is the preferred API for services and distributed workers that must
/// enforce deployment-specific resource budgets.
pub fn deserialize_with_limits<T>(
    document: &[u8],
    limits: DecodeLimits,
) -> SerializationResult<T>
where
    T: serde::de::DeserializeOwned,
{
    decode_with_limits(document, limits)
        .map_err(SerializationErrorKind::Decode)
}

// ============================================================================
// Artifact constructors
// ============================================================================

/// Serializes a schema value and validates the resulting artifact before
/// returning the owned wrapper.
pub fn serialize_artifact<T>(
    value: &T,
) -> SerializationResult<SerializedSchedule>
where
    T: serde::Serialize,
{
    let bytes = serialize(value)?;

    SerializedSchedule::from_bytes(bytes)
}

/// Serializes a schema value with explicit encoder options and validates the
/// resulting artifact before returning the owned wrapper.
pub fn serialize_artifact_with_options<T>(
    value: &T,
    options: EncodeOptions,
) -> SerializationResult<SerializedSchedule>
where
    T: serde::Serialize,
{
    let artifact = serialize_with_options(value, options)?;

    SerializedSchedule::from_bytes(artifact.bytes)
}

/// Serializes a schema value using explicit decode limits for final artifact
/// validation.
///
/// The encoding operation itself remains governed by `EncodeOptions`; the
/// decode limits are used to establish the resource policy under which the
/// resulting artifact is accepted as a canonical serialized schedule.
pub fn serialize_artifact_with_policies<T>(
    value: &T,
    encode_options: EncodeOptions,
    decode_limits: DecodeLimits,
) -> SerializationResult<SerializedSchedule>
where
    T: serde::Serialize,
{
    let artifact = serialize_with_options(value, encode_options)?;

    SerializedSchedule::from_bytes_with_limits(
        artifact.bytes,
        decode_limits,
    )
}

// ============================================================================
// Schema compatibility facade
// ============================================================================

/// Returns the compatibility classification between two scheduling schema
/// versions.
///
/// This function deliberately performs only schema-level classification.
/// Semantic migration remains owned by the compatibility/migration layer if
/// one is introduced later.
#[must_use]
pub const fn schema_compatibility(
    supported: SchemaVersion,
    encountered: SchemaVersion,
) -> SchemaCompatibility {
    SchemaCompatibility::classify(supported, encountered)
}

/// Returns the current scheduling schema version.
///
/// This is a facade helper so callers do not need to know where the schema
/// version constant is physically defined.
#[must_use]
pub const fn current_schema_version() -> SchemaVersion {
    SCHEDULING_SCHEMA_VERSION
}

// ============================================================================
// Canonical identity helpers
// ============================================================================
//
// These helpers intentionally operate on stable wire representations.
//
// They do not create replacement `QubitId` or `PhysicalQubitId` types.
//
// Semantic reconstruction must remain inside the owning quantum::ir::qubit
// subsystem.
//
// ============================================================================

/// Returns the stable serialized representation of a logical qubit identity.
///
/// The canonical semantic identity remains
/// `crate::quantum::ir::qubit::QubitId`.
///
/// This helper accepts a caller-supplied stable numeric representation instead
/// of attempting to reinterpret the canonical IR type inside serialization.
#[must_use]
pub const fn serialize_qubit_id(value: u64) -> SerializedQubitId {
    SerializedId::new(value)
}

/// Returns the stable serialized representation of a physical qubit identity.
///
/// The canonical semantic identity remains
/// `crate::quantum::ir::qubit::PhysicalQubitId`.
///
/// No physical-qubit capacity is implied by this representation.
#[must_use]
pub const fn serialize_physical_qubit_id(
    value: u64,
) -> SerializedPhysicalQubitId {
    SerializedId::new(value)
}

// ============================================================================
// Semantic validation boundary
// ============================================================================
//
// Serialization validation and scheduling validation are intentionally
// different operations.
//
// Serialization validation answers:
//
//     "Are these bytes a structurally valid scheduling artifact?"
//
// Scheduling validation answers:
//
//     "Does this schedule make semantic and physical sense for its target?"
//
// Hardware validation answers:
//
//     "Can this target execute the schedule?"
//
// Runtime validation answers:
//
//     "Is execution currently permitted and possible?"
//
// Keeping these distinct prevents serialization from acquiring dependencies on
// hardware, routing, runtime, or execution.
//
// ============================================================================

/// Marker describing the validation boundary of a decoded artifact.
///
/// This is intentionally lightweight. It does not claim that the schedule is
/// executable.
///
/// A caller should treat a successfully decoded object as structurally valid,
/// not automatically hardware-valid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationLevel {
    /// Bytes and serialization framing are valid.
    Serialized,

    /// Schema structure is valid.
    Schema,

    /// Scheduling semantics have been validated.
    Scheduling,

    /// Hardware-target compatibility has been validated.
    TargetCompatible,

    /// The artifact has passed every validation stage required by the caller.
    ExecutionReady,
}

impl ValidationLevel {
    /// Returns whether scheduling semantic validation has been reached.
    #[must_use]
    pub const fn includes_scheduling(self) -> bool {
        matches!(
            self,
            Self::Scheduling
                | Self::TargetCompatible
                | Self::ExecutionReady
        )
    }

    /// Returns whether target compatibility has been reached.
    #[must_use]
    pub const fn includes_target(self) -> bool {
        matches!(
            self,
            Self::TargetCompatible | Self::ExecutionReady
        )
    }

    /// Returns whether execution readiness has been established.
    #[must_use]
    pub const fn is_execution_ready(self) -> bool {
        matches!(self, Self::ExecutionReady)
    }
}

// ============================================================================
// Serialization provenance
// ============================================================================
//
// Provenance belongs to the scheduling artifact rather than to the binary
// encoder implementation.
//
// The actual provenance structure belongs to schema.rs when implemented.
// This facade intentionally provides no duplicate provenance model.
//
// ============================================================================

// ============================================================================
// API stability notes
// ============================================================================
//
// Public callers should prefer:
//
//     serialize
//     serialize_with_options
//     serialize_canonical_json
//     serialize_to_writer
//     deserialize
//     deserialize_with_limits
//     serialize_artifact
//     current_schema_version
//
// They should not depend on:
//
//     private fields
//     implementation-specific framing offsets
//     encoder internals
//     decoder internals
//     schema implementation details
//
// This gives the scheduling serialization subsystem freedom to evolve from:
//
//     in-memory encoding
//
// to:
//
//     streaming encoding
//
// to:
//
//     chunked distributed encoding
//
// without changing scheduling semantics.
//
// ============================================================================

// ============================================================================
// Compile-time integration assertions
// ============================================================================
//
// These assertions intentionally test only public structural contracts that
// this facade owns.
//
// Detailed schema, encoding, and decoding tests belong to their owning files
// and test modules.
//
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_schema_version_is_self_consistent() {
        let version = current_schema_version();

        assert_eq!(version.major, SCHEDULING_SCHEMA_MAJOR);
        assert_eq!(version.minor, SCHEDULING_SCHEMA_MINOR);
        assert_eq!(version.patch, SCHEDULING_SCHEMA_PATCH);
    }

    #[test]
    fn exact_schema_version_is_exact() {
        let version = current_schema_version();

        assert_eq!(
            schema_compatibility(version, version),
            SchemaCompatibility::Exact
        );
    }

    #[test]
    fn serialized_id_is_stable() {
        let logical = serialize_qubit_id(42);
        let physical = serialize_physical_qubit_id(99);

        assert_eq!(logical.get(), 42);
        assert_eq!(physical.get(), 99);
    }

    #[test]
    fn validation_levels_have_expected_ordering_semantics() {
        assert!(!ValidationLevel::Serialized.includes_scheduling());
        assert!(ValidationLevel::Scheduling.includes_scheduling());
        assert!(ValidationLevel::TargetCompatible.includes_target());
        assert!(ValidationLevel::ExecutionReady.is_execution_ready());
    }

    #[test]
    fn serialized_schedule_borrowing_is_zero_copy() {
        let bytes = vec![1_u8, 2_u8, 3_u8];

        // The constructor validates canonical framing, so arbitrary bytes are
        // expected to fail. This test therefore checks the container API using
        // a manually constructed value inside this private test module.
        let artifact = SerializedSchedule { bytes };

        assert_eq!(artifact.len(), 3);
        assert_eq!(artifact.as_bytes(), &[1, 2, 3]);
        assert!(!artifact.is_empty());
    }

    #[test]
    fn serialized_schedule_round_trip_container() {
        let artifact = SerializedSchedule {
            bytes: vec![7_u8, 8_u8, 9_u8],
        };

        let bytes = artifact.clone().into_bytes();

        assert_eq!(bytes, vec![7_u8, 8_u8, 9_u8]);
        assert_eq!(artifact.as_ref(), &[7_u8, 8_u8, 9_u8]);
    }
}