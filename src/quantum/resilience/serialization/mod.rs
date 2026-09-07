//! Zamani Quantum Resilience — Serialization Boundary
//!
//! Production-grade serialization facade for:
//!
//! `crate::quantum::resilience`
//!
//! # Purpose
//!
//! This module is the stable public boundary between resilience semantic
//! objects and their persisted/transmitted representation.
//!
//! It owns:
//!
//! - serialization API composition;
//! - deserialization API composition;
//! - codec traits;
//! - serialized-artifact ownership;
//! - resource-policy propagation;
//! - schema/version negotiation at the facade boundary;
//! - deterministic serialization contracts;
//! - structural validation before semantic reconstruction.
//!
//! It does NOT own:
//!
//! - resilience domain semantics;
//! - fault semantics;
//! - incident diagnosis;
//! - recovery policy;
//! - hardware semantics;
//! - routing;
//! - scheduling;
//! - QEC algorithms;
//! - optimization;
//! - backend/provider implementations;
//! - transport protocols;
//! - persistence backends.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                    Zamani Quantum Program
//!                             │
//!                             ▼
//!                   Canonical Quantum IR
//!                             │
//!                             ▼
//!                    Quantum Resilience
//!                             │
//!          ┌──────────────────┼──────────────────┐
//!          │                  │                  │
//!          ▼                  ▼                  ▼
//!       detection         recovery           telemetry
//!          │                  │                  │
//!          └──────────────────┼──────────────────┘
//!                             ▼
//!                resilience::serialization
//!                             │
//!              ┌──────────────┼──────────────┐
//!              ▼              ▼              ▼
//!           schema        canonical       compatibility
//!              │              │              │
//!              └──────────────┼──────────────┘
//!                             ▼
//!                 deterministic bytes
//!                             │
//!            ┌────────────────┼────────────────┐
//!            ▼                ▼                ▼
//!         storage          transport         cache
//! ```
//!
//! # Critical ownership rule
//!
//! Serialization must never create competing quantum identity systems.
//!
//! When a serialized resilience object refers to a quantum resource, its
//! semantic codec must use the canonical identities owned by the IR subsystem:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The serialization layer does not define:
//!
//! ```text
//! ResilienceQubitId
//! ResilienceLogicalQubitId
//! ResiliencePhysicalQubitId
//! ```
//!
//! Existing resilience resource modelling follows this same ownership rule.
//! Serialization therefore preserves those identities rather than translating
//! them into a second identity domain.
//!
//! # Separation from Quantum IR serialization
//!
//! There are two related but distinct serialization boundaries:
//!
//! ```text
//! quantum::ir::serialization
//!     │
//!     └── canonical Quantum IR objects
//!
//! quantum::resilience::serialization
//!     │
//!     └── resilience state, incidents, plans, telemetry,
//!         checkpoints, provenance and related objects
//! ```
//!
//! A resilience object containing a reference to canonical Quantum IR must
//! preserve that reference according to the IR identity/serialization
//! contract. Resilience must not redefine the IR wire format.
//!
//! # Write once, scale everywhere
//!
//! This module contains no fixed quantum-machine capacity.
//!
//! It must not assume:
//!
//! - a fixed number of qubits;
//! - a fixed number of logical qubits;
//! - a fixed number of physical qubits;
//! - a fixed number of operations;
//! - a fixed circuit depth;
//! - a fixed topology size;
//! - a fixed number of resources;
//! - a fixed number of recovery actions;
//! - a fixed number of telemetry records;
//! - a fixed number of incidents;
//! - a fixed number of backends.
//!
//! Concrete resource limits are explicit policy values supplied by the caller.
//!
//! Therefore the semantic architecture is:
//!
//! ```text
//! one resource
//!      │
//!      ▼
//! same schema
//!      │
//!      ▼
//! arbitrary finite resource population
//!      │
//!      ▼
//! same schema
//! ```
//!
//! "Infinity" means that the architecture has no artificial machine-size
//! constant. Every actual execution remains finite because concrete systems
//! have finite address space, memory, storage, transport and execution
//! resources.
//!
//! # Security model
//!
//! Serialized resilience data must be treated as untrusted whenever it crosses:
//!
//! - disk;
//! - network;
//! - IPC;
//! - cache;
//! - plugin boundaries;
//! - process boundaries;
//! - distributed-worker boundaries;
//! - user-controlled input;
//! - checkpoint storage.
//!
//! The serialization boundary therefore requires:
//!
//! - magic validation;
//! - format-version validation;
//! - schema-version validation;
//! - length validation before allocation;
//! - checked integer conversions;
//! - explicit collection limits;
//! - explicit byte limits;
//! - explicit nesting limits;
//! - UTF-8 validation;
//! - discriminant validation;
//! - checksum/integrity validation;
//! - truncation detection;
//! - trailing-data detection;
//! - deterministic decoding;
//! - no silent field loss;
//! - no unsafe code.
//!
//! A structurally valid document is not automatically a semantically valid
//! resilience state. Semantic validation remains the responsibility of the
//! owning resilience subsystem.
//!
//! # Determinism
//!
//! Canonical serialization MUST satisfy:
//!
//! ```text
//! same semantic value
//! + same schema version
//! + same format version
//! + same codec
//! + same serialization policy
//! = identical canonical bytes
//! ```
//!
//! Semantic sequence ordering is owned by the semantic object. This module
//! never sorts an object's fields merely to make serialization deterministic.
//!
//! In particular, codecs must never depend on:
//!
//! - memory addresses;
//! - pointer identity;
//! - process IDs;
//! - thread IDs;
//! - wall-clock time unless it is an actual semantic field;
//! - random values;
//! - unordered-map iteration order;
//! - platform-dependent formatting.
//!
//! # Versioning
//!
//! These concepts remain separate:
//!
//! ```text
//! serialization format version
//!             ≠
//! resilience schema version
//!             ≠
//! Quantum IR version
//!             ≠
//! compiler version
//!             ≠
//! application version
//! ```
//!
//! `schema.rs` owns resilience serialization schema identity.
//!
//! `canonical.rs` owns the binary document framing contract.
//!
//! `compatibility.rs` owns compatibility and migration decisions.
//!
//! The Quantum IR subsystem remains authoritative for Quantum IR versions.
//!
//! # Rust
//!
//! Requirements:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! Unsafe code is compiler-forbidden below.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

// ============================================================================
// Submodules
// ============================================================================
//
// These modules form the complete resilience serialization subsystem.
//
// schema.rs
//     Owns schema identity, version metadata and schema-level constants.
//
// canonical.rs
//     Owns canonical document framing, integrity validation and structural
//     document validation.
//
// encoder.rs
//     Owns primitive and bounded payload encoding.
//
// decoder.rs
//     Owns primitive and bounded payload decoding.
//
// compatibility.rs
//     Owns schema compatibility and migration policy.
//
// The semantic resilience modules remain owners of their semantic structures.

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
};

pub use compatibility::{
    check_compatibility,
    Compatibility,
    CompatibilityError,
};

pub use decoder::{
    DecodeLimits,
    Decoder,
};

pub use encoder::{
    EncodeLimits,
    Encoder,
};

pub use schema::{
    current_schema_version,
    is_current_schema_version,
    SchemaVersion,
    CURRENT_SCHEMA_VERSION,
    FORMAT_VERSION,
};

// ============================================================================
// Serialization error
// ============================================================================

/// Error returned by the resilience serialization boundary.
///
/// The error deliberately describes serialization-layer failures rather than
/// attempting to model every possible semantic resilience failure.
///
/// Semantic modules may translate this error into their own domain errors.
///
/// The representation is intentionally owned by this public facade so all
/// serialization child modules share one stable error contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    /// The serialized document is shorter than the required framing.
    TruncatedDocument,

    /// The serialized document contains bytes after the expected document.
    TrailingBytes,

    /// The document magic does not identify a Zamani resilience document.
    InvalidMagic,

    /// The binary serialization format is not supported.
    UnsupportedFormatVersion {
        /// Version found in the document.
        version: u16,

        /// Highest format version understood by this implementation.
        supported: u16,
    },

    /// The resilience schema is not supported.
    UnsupportedSchemaVersion {
        /// Schema version found in the document.
        version: SchemaVersion,
    },

    /// The document declares a payload larger than the permitted limit.
    PayloadTooLarge {
        /// Requested payload size.
        size: u64,

        /// Permitted payload size.
        limit: u64,
    },

    /// A collection declares more elements than permitted.
    CollectionTooLarge {
        /// Declared element count.
        count: u64,

        /// Permitted element count.
        limit: u64,
    },

    /// A nesting depth exceeds the active decode policy.
    NestingTooDeep {
        /// Observed nesting depth.
        depth: u32,

        /// Permitted nesting depth.
        limit: u32,
    },

    /// A field exceeds its configured byte limit.
    FieldTooLarge {
        /// Declared field size.
        size: u64,

        /// Permitted field size.
        limit: u64,
    },

    /// A wire integer cannot be represented by the host type required by the
    /// current operation.
    IntegerOverflow,

    /// A wire value is not a valid boolean representation.
    InvalidBoolean {
        /// Raw boolean byte.
        value: u8,
    },

    /// A wire value is not a valid enum/discriminant.
    InvalidDiscriminant {
        /// Semantic type name.
        type_name: &'static str,

        /// Raw discriminant.
        value: u32,
    },

    /// A length or structural field is inconsistent with the document.
    InvalidLength,

    /// A byte sequence expected to be UTF-8 is malformed.
    InvalidUtf8,

    /// Integrity verification failed.
    IntegrityMismatch,

    /// The payload could not be structurally decoded.
    InvalidPayload,

    /// The payload contains data that the selected semantic decoder did not
    /// consume.
    UnconsumedPayload,

    /// The requested schema migration is not implemented.
    MigrationUnavailable {
        /// Source schema.
        from: SchemaVersion,

        /// Requested destination schema.
        to: SchemaVersion,
    },

    /// The caller supplied an invalid serialization policy.
    InvalidLimits,

    /// A semantic codec rejected an operation.
    CodecError {
        /// Stable codec-level message.
        message: String,
    },

    /// A semantic version is incompatible with the requested operation.
    IncompatibleSemanticVersion {
        /// Human-readable version descriptor.
        message: String,
    },
}

impl fmt::Display for SerializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TruncatedDocument => {
                formatter.write_str("truncated resilience serialization document")
            }
            Self::TrailingBytes => {
                formatter.write_str("trailing bytes after resilience serialization document")
            }
            Self::InvalidMagic => {
                formatter.write_str("invalid resilience serialization magic")
            }
            Self::UnsupportedFormatVersion {
                version,
                supported,
            } => write!(
                formatter,
                "unsupported resilience serialization format version {version}; \
                 maximum supported version is {supported}"
            ),
            Self::UnsupportedSchemaVersion { version } => {
                write!(formatter, "unsupported resilience schema version {version}")
            }
            Self::PayloadTooLarge { size, limit } => {
                write!(
                    formatter,
                    "serialized payload size {size} exceeds configured limit {limit}"
                )
            }
            Self::CollectionTooLarge { count, limit } => {
                write!(
                    formatter,
                    "serialized collection size {count} exceeds configured limit {limit}"
                )
            }
            Self::NestingTooDeep { depth, limit } => {
                write!(
                    formatter,
                    "serialization nesting depth {depth} exceeds configured limit {limit}"
                )
            }
            Self::FieldTooLarge { size, limit } => {
                write!(
                    formatter,
                    "serialized field size {size} exceeds configured limit {limit}"
                )
            }
            Self::IntegerOverflow => {
                formatter.write_str("serialized integer cannot be represented")
            }
            Self::InvalidBoolean { value } => {
                write!(formatter, "invalid serialized boolean value {value}")
            }
            Self::InvalidDiscriminant { type_name, value } => {
                write!(
                    formatter,
                    "invalid serialized discriminant {value} for {type_name}"
                )
            }
            Self::InvalidLength => {
                formatter.write_str("invalid serialized length")
            }
            Self::InvalidUtf8 => {
                formatter.write_str("invalid UTF-8 in serialized data")
            }
            Self::IntegrityMismatch => {
                formatter.write_str("serialized document integrity verification failed")
            }
            Self::InvalidPayload => {
                formatter.write_str("invalid serialized resilience payload")
            }
            Self::UnconsumedPayload => {
                formatter.write_str("semantic decoder did not consume the complete payload")
            }
            Self::MigrationUnavailable { from, to } => {
                write!(
                    formatter,
                    "resilience schema migration from {from} to {to} is unavailable"
                )
            }
            Self::InvalidLimits => {
                formatter.write_str("invalid serialization resource limits")
            }
            Self::CodecError { message } => {
                write!(formatter, "resilience serialization codec error: {message}")
            }
            Self::IncompatibleSemanticVersion { message } => {
                write!(
                    formatter,
                    "incompatible semantic version: {message}"
                )
            }
        }
    }
}

impl std::error::Error for SerializationError {}

/// Result type used throughout resilience serialization.
pub type SerializationResult<T> = Result<T, SerializationError>;

// ============================================================================
// Codec traits
// ============================================================================

/// Encodes a resilience semantic value into the canonical resilience payload.
///
/// # Ownership
///
/// The implementing semantic module owns:
///
/// - field ordering;
/// - semantic representation;
/// - semantic invariants;
/// - optional/required field decisions;
/// - references to canonical IR objects.
///
/// The serialization subsystem owns:
///
/// - primitive wire representation;
/// - length encoding;
/// - bounds enforcement;
/// - document framing;
/// - schema versioning;
/// - integrity validation.
///
/// # Determinism
///
/// Implementations must produce deterministic bytes.
///
/// A semantic collection whose ordering is meaningful must be encoded in its
/// semantic order. This trait does not authorize arbitrary sorting.
///
/// # Quantum identities
///
/// If the object contains a quantum identity, its implementation must use the
/// canonical identity types from:
///
/// ```text
/// crate::quantum::ir::qubit
/// ```
///
/// No resilience-specific quantum identity may be introduced by an encoder.
pub trait ResilienceEncode {
    /// Encode the semantic value into `encoder`.
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()>;
}

/// Decodes a resilience semantic value from a canonical payload.
///
/// # Security
///
/// Implementations must treat all decoder input as untrusted.
///
/// They must use `Decoder` methods for lengths and bounded allocations.
///
/// # Completeness
///
/// The top-level serialization facade calls `Decoder::finish()` after the
/// semantic value is reconstructed. Therefore a decoder cannot silently accept
/// an object followed by malicious or accidental extra payload.
pub trait ResilienceDecode: Sized {
    /// Reconstruct the semantic value from `decoder`.
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self>;
}

// ============================================================================
// Serialized artifact
// ============================================================================

/// Immutable owned canonical resilience serialization artifact.
///
/// The artifact owns the exact canonical bytes exchanged with storage,
/// transport, cache, checkpoint, provenance and distributed execution layers.
///
/// The artifact contains no semantic interpretation.
///
/// # Security
///
/// Construction validates the complete canonical envelope before the bytes
/// are accepted as a `SerializedResilience` value.
#[derive(Clone, PartialEq, Eq)]
pub struct SerializedResilience {
    bytes: Vec<u8>,
}

impl fmt::Debug for SerializedResilience {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SerializedResilience")
            .field("len", &self.bytes.len())
            .finish()
    }
}

impl SerializedResilience {
    /// Construct an artifact from canonical resilience bytes.
    ///
    /// The document is fully structurally validated before ownership is
    /// accepted.
    pub fn from_bytes(bytes: Vec<u8>) -> SerializationResult<Self> {
        decode_document(&bytes)?;

        Ok(Self { bytes })
    }

    /// Construct an artifact using explicit resource limits.
    pub fn from_bytes_with_limits(
        bytes: Vec<u8>,
        limits: DecodeLimits,
    ) -> SerializationResult<Self> {
        decode_document_with_limits(&bytes, limits)?;

        Ok(Self { bytes })
    }

    /// Borrow the exact canonical serialized bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the serialized byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Return whether the underlying byte representation is empty.
    ///
    /// A valid canonical resilience document is never empty. This method is
    /// provided for conventional container ergonomics.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consume the artifact and return its exact canonical bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl AsRef<[u8]> for SerializedResilience {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

// ============================================================================
// Encoding facade
// ============================================================================

/// Serialize a resilience object using the current schema and default limits.
///
/// This is the normal application-level serialization entry point.
pub fn serialize<T: ResilienceEncode>(
    value: &T,
) -> SerializationResult<Vec<u8>> {
    serialize_with_limits(value, EncodeLimits::default())
}

/// Serialize a resilience object with an explicit resource policy.
///
/// The policy controls resource consumption; it does not change semantic
/// meaning.
pub fn serialize_with_limits<T: ResilienceEncode>(
    value: &T,
    limits: EncodeLimits,
) -> SerializationResult<Vec<u8>> {
    let mut encoder = Encoder::with_limits(limits)?;

    value.encode(&mut encoder)?;

    let payload = encoder.into_bytes();

    encode_document(CURRENT_SCHEMA_VERSION, &payload)
}

/// Serialize using an explicitly selected schema version.
///
/// The selected version must be supported by the semantic codec and
/// compatibility layer.
pub fn serialize_with_schema<T: ResilienceEncode>(
    value: &T,
    schema_version: SchemaVersion,
) -> SerializationResult<Vec<u8>> {
    serialize_with_schema_and_limits(
        value,
        schema_version,
        EncodeLimits::default(),
    )
}

/// Serialize using an explicit schema version and resource policy.
pub fn serialize_with_schema_and_limits<T: ResilienceEncode>(
    value: &T,
    schema_version: SchemaVersion,
    limits: EncodeLimits,
) -> SerializationResult<Vec<u8>> {
    check_compatibility(
        schema_version,
        CURRENT_SCHEMA_VERSION,
    )?;

    let mut encoder = Encoder::with_limits(limits)?;

    value.encode(&mut encoder)?;

    let payload = encoder.into_bytes();

    encode_document(schema_version, &payload)
}

/// Serialize a resilience object into an immutable owned artifact.
pub fn serialize_artifact<T: ResilienceEncode>(
    value: &T,
) -> SerializationResult<SerializedResilience> {
    let bytes = serialize(value)?;

    SerializedResilience::from_bytes(bytes)
}

/// Serialize a resilience object into an immutable artifact with explicit
/// limits.
pub fn serialize_artifact_with_limits<T: ResilienceEncode>(
    value: &T,
    limits: EncodeLimits,
) -> SerializationResult<SerializedResilience> {
    let bytes = serialize_with_limits(value, limits)?;

    /*
     * The same policy must be sufficient to validate the artifact that was
     * just produced. We therefore derive the decoder policy from the encoder
     * policy instead of silently switching to an unrelated default.
     */
    let decode_limits = DecodeLimits::from_encode_limits(limits)?;

    SerializedResilience::from_bytes_with_limits(bytes, decode_limits)
}

// ============================================================================
// Decoding facade
// ============================================================================

/// Deserialize a resilience object from a canonical document using default
/// resource limits.
pub fn deserialize<T: ResilienceDecode>(
    document: &[u8],
) -> SerializationResult<T> {
    deserialize_with_limits(document, DecodeLimits::default())
}

/// Deserialize a resilience object with an explicit resource policy.
///
/// Structural validation occurs before the semantic decoder is invoked.
pub fn deserialize_with_limits<T: ResilienceDecode>(
    document: &[u8],
    limits: DecodeLimits,
) -> SerializationResult<T> {
    let canonical = decode_document_with_limits(document, limits)?;

    let schema_version = canonical.schema_version();

    /*
     * Compatibility is checked before semantic decoding. This is important:
     *
     * untrusted bytes
     *      ↓
     * framing validation
     *      ↓
     * schema compatibility
     *      ↓
     * semantic decoding
     *
     * not:
     *
     * untrusted bytes
     *      ↓
     * semantic allocation
     *      ↓
     * compatibility check
     */
    check_compatibility(
        schema_version,
        CURRENT_SCHEMA_VERSION,
    )?;

    let mut decoder =
        Decoder::with_limits(canonical.payload(), limits);

    let value = T::decode(&mut decoder)?;

    /*
     * A semantic decoder is successful only if it consumes the entire payload.
     *
     * This prevents:
     *
     * valid_object + attacker_payload
     *
     * from being accepted as a valid object.
     */
    decoder.finish()?;

    Ok(value)
}

/// Deserialize using an explicitly permitted schema version.
///
/// This API is useful for migration/import tools that intentionally accept a
/// non-current but compatible schema.
pub fn deserialize_with_schema<T: ResilienceDecode>(
    document: &[u8],
    expected_schema: SchemaVersion,
) -> SerializationResult<T> {
    deserialize_with_schema_and_limits(
        document,
        expected_schema,
        DecodeLimits::default(),
    )
}

/// Deserialize using an explicit schema and resource policy.
pub fn deserialize_with_schema_and_limits<T: ResilienceDecode>(
    document: &[u8],
    expected_schema: SchemaVersion,
    limits: DecodeLimits,
) -> SerializationResult<T> {
    let canonical = decode_document_with_limits(document, limits)?;

    let actual_schema = canonical.schema_version();

    if actual_schema != expected_schema {
        return Err(SerializationError::UnsupportedSchemaVersion {
            version: actual_schema,
        });
    }

    let mut decoder =
        Decoder::with_limits(canonical.payload(), limits);

    let value = T::decode(&mut decoder)?;

    decoder.finish()?;

    Ok(value)
}

// ============================================================================
// Inspection / validation
// ============================================================================

/// Inspect the canonical envelope without constructing a semantic resilience
/// object.
///
/// This is appropriate for:
///
/// - caches;
//! - storage;
/// - transport;
/// - compatibility negotiation;
/// - provenance;
/// - checkpoint discovery.
pub fn inspect(
    document: &[u8],
) -> SerializationResult<CanonicalDocument<'_>> {
    decode_document(document)
}

/// Inspect the canonical envelope using an explicit decode policy.
pub fn inspect_with_limits(
    document: &[u8],
    limits: DecodeLimits,
) -> SerializationResult<CanonicalDocument<'_>> {
    decode_document_with_limits(document, limits)
}

/// Validate the complete structural serialization envelope.
///
/// This does not perform semantic resilience validation.
pub fn validate_document(
    document: &[u8],
) -> SerializationResult<()> {
    decode_document(document)?;

    Ok(())
}

/// Validate the complete structural serialization envelope using explicit
/// limits.
pub fn validate_document_with_limits(
    document: &[u8],
    limits: DecodeLimits,
) -> SerializationResult<()> {
    decode_document_with_limits(document, limits)?;

    Ok(())
}

/// Determine whether a document's schema can be consumed by the current
/// resilience serialization implementation.
///
/// This performs envelope validation and compatibility validation but does not
/// decode a semantic resilience object.
pub fn is_compatible(document: &[u8]) -> bool {
    match inspect(document) {
        Ok(document) => {
            check_compatibility(
                document.schema_version(),
                CURRENT_SCHEMA_VERSION,
            )
            .is_ok()
        }
        Err(_) => false,
    }
}

// ============================================================================
// Version / format helpers
// ============================================================================

/// Return the current resilience serialization schema version.
#[must_use]
pub const fn current_schema() -> SchemaVersion {
    CURRENT_SCHEMA_VERSION
}

/// Return the serialization binary format version.
#[must_use]
pub const fn format_version() -> u16 {
    FORMAT_VERSION
}

// ============================================================================
// Integration contracts
// ============================================================================
//
// The following contracts are normative for the remaining serialization files.
//
// -----------------------------------------------------------------------------
// schema.rs
// -----------------------------------------------------------------------------
//
// MUST provide:
//
//     pub struct SchemaVersion
//     pub const CURRENT_SCHEMA_VERSION: SchemaVersion
//     pub const FORMAT_VERSION: u16
//     pub const fn current_schema_version() -> SchemaVersion
//     pub const fn is_current_schema_version(...) -> bool
//
// SchemaVersion must be:
// - deterministic;
// - copyable;
// - orderable;
// - serializable by canonical primitives;
// - independent from Quantum IR version;
// - independent from compiler version.
//
// -----------------------------------------------------------------------------
// canonical.rs
// -----------------------------------------------------------------------------
//
// MUST provide:
//
//     pub struct CanonicalDocument<'a>
//     pub fn encode_document(...)
//     pub fn decode_document(...)
//     pub fn decode_document_with_limits(...)
//
// CanonicalDocument MUST expose:
//
//     schema_version()
//     payload()
//
// The canonical document layer MUST:
// - validate magic;
// - validate format version;
// - validate schema framing;
// - validate declared payload length;
// - validate integrity;
// - reject truncation;
// - reject trailing bytes;
// - perform checked conversions;
// - enforce DecodeLimits.
//
// -----------------------------------------------------------------------------
// encoder.rs
// -----------------------------------------------------------------------------
//
// MUST provide:
//
//     pub struct Encoder
//     pub struct EncodeLimits
//     Encoder::default()
//     Encoder::with_limits(...)
//     Encoder::into_bytes()
//
// Encoder MUST:
// - never use unsafe;
// - enforce configured limits;
// - reject integer overflow;
// - encode deterministic primitives;
// - never depend on machine size;
// - never encode pointer addresses;
// - never silently truncate.
//
// It SHOULD provide canonical methods for:
//
//     bool
//     u8/u16/u32/u64/u128
//     i8/i16/i32/i64/i128
//     usize/u size where explicitly defined by the wire contract
//     byte slices
//     strings
//     optional values
//     sequences
//     maps with explicit semantic ordering
//     SchemaVersion
//     QubitId
//     PhysicalQubitId
//     ResourceId
//
// Quantum identity methods must use the canonical types from:
//
//     crate::quantum::ir::qubit
//
// -----------------------------------------------------------------------------
// decoder.rs
// -----------------------------------------------------------------------------
//
// MUST provide:
//
//     pub struct Decoder<'a>
//     pub struct DecodeLimits
//     Decoder::with_limits(...)
//     Decoder::finish()
//
// Decoder MUST:
// - never allocate based on untrusted lengths before checking limits;
// - perform checked integer conversion;
// - reject malformed booleans;
// - reject malformed UTF-8;
// - reject invalid discriminants;
// - enforce collection limits;
// - enforce field-size limits;
// - enforce nesting limits;
// - expose bounded byte/string reads;
// - expose canonical QubitId/PhysicalQubitId decoding;
// - never use unsafe.
//
// -----------------------------------------------------------------------------
// compatibility.rs
// -----------------------------------------------------------------------------
//
// MUST provide:
//
//     pub enum Compatibility
//     pub enum CompatibilityError
//     pub fn check_compatibility(from, to)
//
// Compatibility MUST remain separate from:
// - binary format compatibility;
// - Quantum IR compatibility;
// - compiler compatibility;
// - hardware compatibility.
//
// It may later provide migration APIs, but migration must never silently
// reinterpret incompatible semantic state.
//
// -----------------------------------------------------------------------------
// Semantic resilience modules
// -----------------------------------------------------------------------------
//
// A semantic resilience module implementing ResilienceEncode / ResilienceDecode
// owns its own field ordering and meaning.
//
// Examples:
//
//     model/fault.rs
//     model/incident.rs
//     model/resource.rs
//     planning/plan.rs
//     recovery/recovery.rs
//     checkpoint/snapshot.rs
//     telemetry/event.rs
//
// Those modules must NOT define a second wire format.
//
// -----------------------------------------------------------------------------
// Quantum IR integration
// -----------------------------------------------------------------------------
//
// If a resilience object contains:
//
//     QubitId
//     PhysicalQubitId
//     ResourceId
//     QuantumCircuit
//     QuantumOperation
//
// the owning subsystem remains responsible for semantic ownership.
//
// Resilience serialization must call the canonical IR serialization/identity
// contract rather than defining duplicate types.
//
// In particular:
//
//     crate::quantum::ir::qubit::QubitId
//     crate::quantum::ir::qubit::PhysicalQubitId
//
// remain authoritative.
//
// -----------------------------------------------------------------------------
// Checkpoint integration
// -----------------------------------------------------------------------------
//
// checkpoint/ must use this serialization boundary for:
//
//     checkpoint metadata
//     manifests
//     integrity information
//     recovery state
//     provenance
//
// It must not define an independent resilience wire format.
//
// -----------------------------------------------------------------------------
// Telemetry integration
// -----------------------------------------------------------------------------
//
// telemetry/ may serialize events, metrics, health observations and traces
// through this boundary.
//
// Telemetry transport formats such as OpenTelemetry, JSON, protobuf or other
// external formats belong to adapters/exporters rather than this core module.
//
// -----------------------------------------------------------------------------
// Storage integration
// -----------------------------------------------------------------------------
//
// Storage receives:
//
//     SerializedResilience
//
// and must not mutate its bytes.
//
// Content-addressed storage may hash:
//
//     SerializedResilience::as_bytes()
//
// directly.
//
// -----------------------------------------------------------------------------
// Distributed integration
// -----------------------------------------------------------------------------
//
// Distributed coordination may transport:
//
//     SerializedResilience
//
// but serialization must remain unaware of the transport implementation.
//
// -----------------------------------------------------------------------------
// Security integration
// -----------------------------------------------------------------------------
//
// Serialization validates structure and integrity.
//
// It does NOT establish:
// - authorization;
// - identity;
// - trust;
// - backend authenticity;
// - telemetry authenticity.
//
// Those remain higher-level security responsibilities.
//
// -----------------------------------------------------------------------------
// Scalability integration
// -----------------------------------------------------------------------------
//
// No serializer may introduce a semantic constant such as:
//
//     MAX_QUBITS
//     MAX_LOGICAL_QUBITS
//     MAX_PHYSICAL_QUBITS
//     MAX_GATES
//     MAX_OPERATIONS
//
// Resource limits are explicit DecodeLimits / EncodeLimits or higher-level
// policy values.
//
// A deployment can therefore choose a limit appropriate to its resources
// without changing the semantic schema or source program.
//
// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestValue {
        number: u64,
    }

    impl ResilienceEncode for TestValue {
        fn encode(
            &self,
            encoder: &mut Encoder,
        ) -> SerializationResult<()> {
            encoder.write_u64(self.number)?;

            Ok(())
        }
    }

    impl ResilienceDecode for TestValue {
        fn decode(
            decoder: &mut Decoder<'_>,
        ) -> SerializationResult<Self> {
            Ok(Self {
                number: decoder.read_u64()?,
            })
        }
    }

    #[test]
    fn current_schema_is_exposed() {
        assert_eq!(
            current_schema(),
            CURRENT_SCHEMA_VERSION
        );
    }

    #[test]
    fn current_format_version_is_exposed() {
        assert_eq!(
            format_version(),
            FORMAT_VERSION
        );
    }

    #[test]
    fn serialization_round_trip_is_structurally_valid() {
        let value = TestValue {
            number: 42,
        };

        let bytes = serialize(&value)
            .expect("test serialization must succeed");

        validate_document(&bytes)
            .expect("serialized document must validate");

        let decoded =
            deserialize::<TestValue>(&bytes)
                .expect("test deserialization must succeed");

        assert_eq!(decoded.number, value.number);
    }

    #[test]
    fn artifact_contains_exact_serialized_bytes() {
        let value = TestValue {
            number: 123,
        };

        let bytes = serialize(&value)
            .expect("test serialization must succeed");

        let artifact =
            SerializedResilience::from_bytes(bytes.clone())
                .expect("canonical bytes must form an artifact");

        assert_eq!(artifact.as_bytes(), bytes.as_slice());
        assert_eq!(artifact.len(), bytes.len());
        assert!(!artifact.is_empty());
    }

    #[test]
    fn malformed_document_is_rejected() {
        let result =
            SerializedResilience::from_bytes(vec![0, 1, 2, 3]);

        assert!(result.is_err());
    }

    #[test]
    fn empty_document_is_rejected() {
        assert!(validate_document(&[]).is_err());
    }

    #[test]
    fn compatibility_check_is_false_for_invalid_documents() {
        assert!(!is_compatible(&[]));
    }
}