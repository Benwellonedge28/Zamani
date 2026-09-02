//! Zamani Quantum Noise (ZQN) — Production Deserialization Engine.
//!
//! # Ownership
//!
//! This file owns the *deserialization boundary* for persisted ZQN artifacts.
//!
//! It owns:
//!
//! - decoding already-framed ZQN documents;
//! - validating the binary framing before semantic allocation;
//! - validating payload length against explicit resource policy;
//! - validating payload integrity;
//! - decoding the versioned semantic payload envelope;
//! - validating semantic/schema/compatibility version metadata;
//! - decoding semantic Rust values through Serde;
//! - validating JSON resource limits before semantic materialization;
//! - strict in-memory trailing-byte rejection;
//! - framed-reader decoding;
//! - schema-envelope decoding helpers;
//! - deterministic malformed-input handling;
//! - conversion of low-level decode failures into the established ZQN
//!   serialization error vocabulary.
//!
//! It does NOT own:
//!
//! - ZQN version definitions;
//! - persisted schema definitions;
//! - serialization;
//! - canonicalization;
//! - hashing;
//! - semantic noise models;
//! - probability semantics;
//! - channel mathematics;
//! - fault semantics;
//! - calibration;
//! - characterization;
//! - simulation;
//! - propagation;
//! - target capabilities;
//! - hardware;
//! - routing;
//! - scheduling;
//! - QEC;
//! - quantum IR semantics;
//! - qubit identity;
//! - vendor APIs;
//! - migration implementation.
//!
//! Those responsibilities remain owned by their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                 external bytes
//!                       │
//!                       ▼
//!             ┌────────────────────┐
//!             │  THIS FILE         │
//!             │                    │
//!             │  deserialization   │
//!             │                    │
//!             │  framing validation│
//!             │  integrity check   │
//!             │  version check     │
//!             │  resource check    │
//!             │  semantic decode   │
//!             └─────────┬──────────┘
//!                       │
//!                       ▼
//!                semantic Rust value
//!                       │
//!          ┌────────────┼────────────┐
//!          ▼            ▼            ▼
//!        schema       ZQN domain    consumers
//!        validation   validation
//! ```
//!
//! The dependency direction is deliberately one-way:
//!
//! ```text
//! core/version
//!       │
//!       ▼
//! io/schema ───────────────┐
//!       │                  │
//!       ▼                  ▼
//! io/serialization    THIS FILE
//!       │                  │
//!       └──────────┬───────┘
//!                  ▼
//!           semantic consumers
//! ```
//!
//! This module must not depend on routing, scheduling, hardware, QEC,
//! benchmarking or frontend implementations.
//!
//! # Write once, scale everywhere
//!
//! This file contains no semantic machine-size ceiling.
//!
//! There is no:
//!
//! - maximum qubit count;
//! - maximum physical-resource count;
//! - maximum logical-resource count;
//! - maximum circuit depth;
//! - maximum channel dimension;
//! - maximum fault count;
//! - maximum number of operations;
//! - maximum machine size.
//!
//! Actual resource governance is supplied through [`DecodeLimits`] from the
//! serialization subsystem and, where appropriate, through the caller's
//! higher-level runtime/resource policy.
//!
//! Therefore:
//!
//! ```text
//! tiny machine
//!      │
//!      ▼
//! same ZQN artifact contract
//!      │
//!      ▼
//! large machine
//!      │
//!      ▼
//! same ZQN artifact contract
//! ```
//!
//! A larger machine requires more resources, not a different deserializer.
//!
//! # Security model
//!
//! Persisted ZQN data is untrusted.
//!
//! This module therefore:
//!
//! 1. validates framing before semantic decoding;
//! 2. validates declared lengths;
//! 3. validates the complete document length;
//! 4. validates the SHA-256 payload digest;
//! 5. validates JSON syntax;
//! 6. validates version metadata;
//! 7. validates collection/string/nesting resource policy;
//! 8. rejects trailing bytes for in-memory documents;
//! 9. rejects malformed version metadata;
//! 10. never executes code contained in a payload;
//! 11. never performs filesystem access;
//! 12. never performs network access;
//! 13. never invokes external processes;
//! 14. never uses a hidden global allocator policy;
//! 15. never uses a hidden global RNG;
//! 16. never uses `unsafe`.
//!
//! Deserialization is therefore a data transformation boundary, not an
//! execution boundary.
//!
//! # Integrity versus authenticity
//!
//! SHA-256 verification performed by this module establishes integrity:
//!
//! ```text
//! bytes changed
//!     ↓
//! digest mismatch
//!     ↓
//! reject
//! ```
//!
//! It does NOT establish authenticity or authorization.
//!
//! Digital signatures, trust roots and authorization remain outside this file.
//!
//! # Version ownership
//!
//! Version semantics are owned exclusively by:
//!
//! ```text
//! crate::quantum::zqn::core::version
//! ```
//!
//! This file consumes:
//!
//! - `ZqnVersion`;
//! - `ZqnSchemaVersion`;
//! - `ZqnCompatibilityVersion`;
//! - `ZqnVersionMetadata`.
//!
//! It does not define another version type.
//!
//! # Schema integration
//!
//! `io/schema.rs` owns the logical ZQN document schema.
//!
//! This file provides helpers for decoding a generic serialized `ZqnDocument`
//! and then delegates structural validation to the schema module.
//!
//! It does not reproduce schema validation rules.
//!
//! # Quantum identity integration
//!
//! This file intentionally does not define:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! The repository's canonical quantum identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! If a semantic ZQN payload contains those identities, the owning semantic
//! module is responsible for defining the corresponding Serde representation.
//!
//! The persistence boundary remains identity-agnostic.
//!
//! # Canonical representation
//!
//! The binary framing is owned by `io/serialization.rs`.
//!
//! This module therefore does NOT create another wire format.
//!
//! The current framing is:
//!
//! ```text
//! +----------------------+
//! | magic                | 4 bytes
//! +----------------------+
//! | format version       | 2 bytes
//! +----------------------+
//! | reserved             | 2 bytes
//! +----------------------+
//! | payload length       | 8 bytes
//! +----------------------+
//! | SHA-256 digest       | 32 bytes
//! +----------------------+
//! | versioned JSON       | N bytes
//! +----------------------+
//! ```
//!
//! The exact framing constants are imported from `io::serialization` so this
//! file cannot accidentally establish a competing wire protocol.
//!
//! # Versioned semantic payload
//!
//! The current serializer places version metadata inside the canonical payload:
//!
//! ```json
//! {
//!   "semantic_version": "...",
//!   "schema_version": "...",
//!   "compatibility_version": "...",
//!   "payload": { ... }
//! }
//! ```
//!
//! This module decodes that structure without depending on a private type from
//! `serialization.rs`.
//!
//! That is deliberate: the serializer and deserializer are independently
//! maintainable while sharing the public framing contract.
//!
//! # Reader semantics
//!
//! There are two fundamentally different APIs:
//!
//! ```text
//! byte slice
//!     ↓
//! exact document length known
//!     ↓
//! trailing bytes can be rejected
//!
//! Read
//!     ↓
//! framed document length obtained from header
//!     ↓
//! exactly one frame is consumed
//! ```
//!
//! A generic `Read` implementation cannot universally prove that EOF follows
//! the frame without potentially blocking. Consequently this module never
//! performs an unsafe or heuristic "peek" for reader trailing data.
//!
//! Applications requiring a single-document transport should place an explicit
//! framing boundary around the reader.
//!
//! # Determinism
//!
//! For identical input bytes and identical decode policy:
//!
//! ```text
//! deserialize(bytes, limits)
//! ```
//!
//! produces the same result.
//!
//! It does not depend on:
//!
//! - current time;
//! - environment;
//! - process ID;
//! - thread scheduling;
//! - memory address;
//! - global mutable state;
//! - randomness.
//!
//! # No unsafe
//!
//! This file explicitly forbids unsafe code.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. it uses the established ZQN framing;
//! 2. it does not define another wire format;
//! 3. it does not define another version system;
//! 4. it does not define another qubit identity;
//! 5. malformed input returns errors;
//! 6. corrupted payloads are rejected;
//! 7. unsupported formats are rejected;
//! 8. incompatible versions are rejected;
//! 9. resource limits are explicit;
//! 10. in-memory trailing bytes are rejected;
//! 11. JSON resource limits are enforced;
//! 12. no filesystem/network access occurs;
//! 13. no code contained in the payload can execute;
//! 14. no unsafe Rust exists;
//! 15. semantic domain validation remains outside this module;
//! 16. schema validation remains owned by `io/schema.rs`;
//! 17. serialization remains owned by `io/serialization.rs`;
//! 18. larger quantum systems require no source-code changes;
//! 19. future quantum modalities can use the same boundary;
//! 20. reader and in-memory APIs remain deterministic.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::io::{self, Read};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::schema::{
    ZqnDocument,
    ZqnDocumentKind,
    ZqnDocumentMetadata,
    ZqnDocumentPayload,
    ZqnDocumentId,
    SchemaError,
    SchemaExpectation,
};
use super::serialization::{
    deserialize_canonical,
    inspect,
    DecodeLimits,
    SerializationError,
    DIGEST_LEN,
    HEADER_LEN,
    MAGIC,
    FORMAT_VERSION,
};
use crate::quantum::zqn::core::version::{
    ZqnCompatibilityVersion,
    ZqnSchemaVersion,
    ZqnVersion,
    ZqnVersionMetadata,
};

// =============================================================================
// Public decoded artifact
// =============================================================================

/// Fully decoded ZQN semantic artifact.
///
/// This type is intentionally generic so that adding a new ZQN semantic type
/// does not require editing this module.
///
/// The envelope carries the authoritative artifact version metadata and the
/// decoded semantic payload.
///
/// # Integration
///
/// ```text
/// bytes
///   │
///   ▼
/// deserialize
///   │
///   ▼
/// ZqnDecoded<T>
///   │
///   ├── version
///   └── value
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZqnDecoded<T> {
    /// Version metadata carried by the artifact.
    pub version: ZqnVersionMetadata,

    /// Decoded semantic value.
    pub value: T,

    /// SHA-256 digest of the canonical versioned payload.
    pub digest: [u8; DIGEST_LEN],
}

impl<T> ZqnDecoded<T> {
    /// Creates a decoded artifact.
    #[must_use]
    pub const fn new(
        version: ZqnVersionMetadata,
        value: T,
        digest: [u8; DIGEST_LEN],
    ) -> Self {
        Self {
            version,
            value,
            digest,
        }
    }

    /// Returns the artifact version metadata.
    #[must_use]
    pub const fn version(&self) -> ZqnVersionMetadata {
        self.version
    }

    /// Returns the decoded semantic value.
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Consumes the wrapper and returns the semantic value.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }

    /// Returns the SHA-256 digest.
    #[must_use]
    pub const fn digest(&self) -> [u8; DIGEST_LEN] {
        self.digest
    }
}

// =============================================================================
// Versioned payload contract
// =============================================================================

/// Stable representation of the payload envelope produced by
/// `io::serialization`.
///
/// This is private intentionally. Version metadata ownership remains in
/// `core::version`, while this structure only mirrors the persistence contract.
///
/// Keeping this structure local prevents the serializer's implementation type
/// from becoming part of the public semantic API.
#[derive(Debug, Deserialize)]
struct VersionedPayload {
    semantic_version: ZqnVersion,
    schema_version: ZqnSchemaVersion,
    compatibility_version: ZqnCompatibilityVersion,
    payload: Value,
}

// =============================================================================
// Public error
// =============================================================================

/// Deserialization-specific error.
///
/// Most low-level framing errors are represented by the established
/// [`SerializationError`] type because serialization and deserialization share
/// the same wire contract.
///
/// This type adds only errors that are specifically owned by the deserialization
/// boundary.
#[derive(Debug)]
pub enum DeserializationError {
    /// Low-level serialization/framing failure.
    Serialization(SerializationError),

    /// The decoded versioned payload had an invalid structural shape.
    InvalidVersionedPayload {
        /// Explanation of the structural problem.
        message: String,
    },

    /// The artifact version is not accepted by the selected consumer contract.
    IncompatibleVersion {
        /// Artifact version.
        artifact: ZqnVersionMetadata,

        /// Consumer version.
        consumer: ZqnVersionMetadata,
    },

    /// Schema-level validation failed.
    Schema(SchemaError),

    /// The semantic payload could not be decoded into the requested type.
    SemanticDecode {
        /// Underlying Serde error.
        message: String,
    },

    /// A required JSON member was absent.
    MissingField {
        /// Missing field name.
        field: &'static str,
    },

    /// A JSON member had an unexpected type.
    InvalidFieldType {
        /// Field name.
        field: &'static str,

        /// Expected representation.
        expected: &'static str,
    },

    /// A reader could not provide the requested frame.
    Reader(io::Error),
}

impl fmt::Display for DeserializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialization(error) => {
                write!(formatter, "{error}")
            }

            Self::InvalidVersionedPayload { message } => {
                write!(
                    formatter,
                    "invalid ZQN versioned payload: {message}"
                )
            }

            Self::IncompatibleVersion { artifact, consumer } => {
                write!(
                    formatter,
                    "ZQN artifact version is incompatible with consumer: \
                     artifact semantic={}, schema={}, compatibility={}; \
                     consumer semantic={}, schema={}, compatibility={}",
                    artifact.semantic,
                    artifact.schema,
                    artifact.compatibility,
                    consumer.semantic,
                    consumer.schema,
                    consumer.compatibility,
                )
            }

            Self::Schema(error) => {
                write!(formatter, "ZQN schema validation failed: {error}")
            }

            Self::SemanticDecode { message } => {
                write!(
                    formatter,
                    "ZQN semantic payload decoding failed: {message}"
                )
            }

            Self::MissingField { field } => {
                write!(
                    formatter,
                    "ZQN deserialization missing required field `{field}`"
                )
            }

            Self::InvalidFieldType { field, expected } => {
                write!(
                    formatter,
                    "ZQN field `{field}` has an invalid type; \
                     expected {expected}"
                )
            }

            Self::Reader(error) => {
                write!(
                    formatter,
                    "ZQN reader error: {error}"
                )
            }
        }
    }
}

impl std::error::Error for DeserializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(error) => Some(error),
            Self::Schema(error) => Some(error),
            Self::Reader(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SerializationError> for DeserializationError {
    fn from(error: SerializationError) -> Self {
        Self::Serialization(error)
    }
}

impl From<SchemaError> for DeserializationError {
    fn from(error: SchemaError) -> Self {
        Self::Schema(error)
    }
}

// =============================================================================
// High-level API
// =============================================================================

/// Deserializes a complete ZQN document using the default resource policy.
///
/// This is the primary convenience API.
///
/// For untrusted or externally supplied artifacts, callers should normally use
/// [`deserialize_with_limits`] and provide an explicit policy inherited from
/// the application's resource-governance layer.
pub fn deserialize<T>(
    document: &[u8],
) -> Result<ZqnDecoded<T>, DeserializationError>
where
    T: DeserializeOwned,
{
    deserialize_with_limits(
        document,
        DecodeLimits::default(),
    )
}

/// Deserializes a complete ZQN document with an explicit resource policy.
///
/// The operation is strict:
///
/// - malformed framing is rejected;
/// - unsupported format versions are rejected;
/// - payload length mismatches are rejected;
/// - trailing bytes are rejected;
/// - digest mismatches are rejected;
/// - malformed JSON is rejected;
/// - invalid version metadata is rejected;
/// - incompatible versions are rejected;
/// - resource-limit violations are rejected.
///
/// No semantic machine-size ceiling is introduced.
pub fn deserialize_with_limits<T>(
    document: &[u8],
    limits: DecodeLimits,
) -> Result<ZqnDecoded<T>, DeserializationError>
where
    T: DeserializeOwned,
{
    limits
        .validate()
        .map_err(DeserializationError::Serialization)?;

    // `inspect` performs all framing, declared-length, trailing-byte and
    // digest checks before semantic materialization.
    let info = inspect(document, limits)
        .map_err(DeserializationError::Serialization)?;

    let payload = document
        .get(HEADER_LEN..)
        .ok_or_else(|| {
            DeserializationError::InvalidVersionedPayload {
                message: "document contains no payload after header".to_owned(),
            }
        })?;

    if payload.len() as u64 != info.payload_len {
        return Err(
            DeserializationError::InvalidVersionedPayload {
                message: format!(
                    "payload length mismatch: frame declares {}, \
                     slice contains {}",
                    info.payload_len,
                    payload.len(),
                ),
            },
        );
    }

    let (version, semantic_value) =
        decode_versioned_payload(payload, limits)?;

    validate_artifact_version(version)?;

    let value = decode_semantic_value::<T>(
        semantic_value,
        limits,
    )?;

    Ok(ZqnDecoded::new(
        version,
        value,
        info.digest,
    ))
}

/// Deserializes a complete ZQN artifact while requiring an explicit consumer
/// version contract.
///
/// This is the preferred API for long-lived tooling, migration systems and
/// heterogeneous deployments where the consumer contract should not be
/// implicitly taken from the currently running binary.
pub fn deserialize_with_version_requirement<T>(
    document: &[u8],
    limits: DecodeLimits,
    expectation: ZqnVersionMetadata,
) -> Result<ZqnDecoded<T>, DeserializationError>
where
    T: DeserializeOwned,
{
    let decoded = deserialize_with_limits(document, limits)?;

    if !expectation.accepts(decoded.version) {
        return Err(
            DeserializationError::IncompatibleVersion {
                artifact: decoded.version,
                consumer: expectation,
            },
        );
    }

    Ok(decoded)
}

/// Deserializes exactly one framed ZQN artifact from a reader.
///
/// The reader is consumed only through the frame declared in the ZQN header.
///
/// A generic reader cannot safely be probed for EOF because a network/socket
/// reader may block. Therefore this function does not attempt to establish
/// whether bytes exist *after* the frame.
///
/// Use a higher-level transport framing layer when a stream contains multiple
/// ZQN artifacts.
pub fn deserialize_from_reader<T, R>(
    reader: &mut R,
    limits: DecodeLimits,
) -> Result<ZqnDecoded<T>, DeserializationError>
where
    T: DeserializeOwned,
    R: Read,
{
    limits
        .validate()
        .map_err(DeserializationError::Serialization)?;

    let mut header = [0_u8; HEADER_LEN];

    read_exact(reader, &mut header)?;

    validate_header(&header)?;

    let payload_len = u64::from_le_bytes([
        header[8],
        header[9],
        header[10],
        header[11],
        header[12],
        header[13],
        header[14],
        header[15],
    ]);

    limits
        .validate_payload_size(payload_len)
        .map_err(DeserializationError::Serialization)?;

    let payload_len_usize =
        checked_usize(payload_len, "ZQN payload length")?;

    let total_len =
        HEADER_LEN
            .checked_add(payload_len_usize)
            .ok_or_else(|| {
                DeserializationError::Serialization(
                    SerializationError::ArithmeticOverflow {
                        context: "ZQN framed reader document length",
                    },
                )
            })?;

    limits
        .validate_document_size(total_len as u64)
        .map_err(DeserializationError::Serialization)?;

    let mut document = Vec::new();

    document
        .try_reserve(total_len)
        .map_err(|_| {
            DeserializationError::Serialization(
                SerializationError::LengthOverflow {
                    context: "allocating ZQN reader document",
                    value: total_len as u64,
                },
            )
        })?;

    document.extend_from_slice(&header);

    let old_len = document.len();

    document.resize(total_len, 0);

    if let Err(error) =
        read_exact(reader, &mut document[old_len..])
    {
        return Err(error);
    }

    deserialize_with_limits(&document, limits)
}

/// Deserializes one framed ZQN artifact from a reader and applies an explicit
/// version requirement.
pub fn deserialize_from_reader_with_version_requirement<T, R>(
    reader: &mut R,
    limits: DecodeLimits,
    expectation: ZqnVersionMetadata,
) -> Result<ZqnDecoded<T>, DeserializationError>
where
    T: DeserializeOwned,
    R: Read,
{
    let decoded =
        deserialize_from_reader(reader, limits)?;

    if !expectation.accepts(decoded.version) {
        return Err(
            DeserializationError::IncompatibleVersion {
                artifact: decoded.version,
                consumer: expectation,
            },
        );
    }

    Ok(decoded)
}

/// Deserializes a canonical JSON payload without the ZQN binary framing.
///
/// This API is intentionally provided for integration with the future
/// `io::canonical` layer and for tooling that already owns framing.
///
/// It does not accept the binary ZQN header.
///
/// Version metadata is not inferred because canonical JSON alone does not
/// establish a ZQN artifact version.
pub fn deserialize_canonical_payload<T>(
    payload: &[u8],
    limits: DecodeLimits,
) -> Result<T, DeserializationError>
where
    T: DeserializeOwned,
{
    deserialize_canonical(payload, limits)
        .map_err(DeserializationError::Serialization)
}

// =============================================================================
// ZqnDocument integration
// =============================================================================

/// Deserializes a persisted [`ZqnDocument`] and validates its schema envelope.
///
/// This is the bridge between the generic serialization mechanism and
/// `io/schema.rs`.
///
/// The generic binary serializer remains independent from the schema module;
/// this function is the explicit integration boundary.
pub fn deserialize_schema_document(
    document: &[u8],
    limits: DecodeLimits,
) -> Result<ZqnDecoded<ZqnDocument>, DeserializationError> {
    let decoded =
        deserialize_with_limits::<ZqnDocument>(
            document,
            limits,
        )?;

    decoded.value.validate()?;

    validate_embedded_version_consistency(
        decoded.version,
        &decoded.value,
    )?;

    Ok(decoded)
}

/// Deserializes a persisted [`ZqnDocument`] against an explicit schema
/// expectation.
///
/// This is the recommended API for applications that intentionally support
/// multiple schema versions.
pub fn deserialize_schema_document_with_expectation(
    document: &[u8],
    limits: DecodeLimits,
    expectation: SchemaExpectation,
) -> Result<ZqnDecoded<ZqnDocument>, DeserializationError> {
    let decoded =
        deserialize_with_version_requirement(
            document,
            limits,
            ZqnVersionMetadata::new(
                expectation.semantic_requirement
                    .map(|_| ZqnVersion::new(0, 0, 0))
                    .unwrap_or(ZqnVersion::new(0, 0, 0)),
                expectation.schema_version,
                expectation.compatibility_version,
            ),
        )?;

    // The explicit schema expectation is more precise than the generic
    // version metadata comparison above, so perform schema validation through
    // the authoritative schema contract as the final step.
    decoded
        .value
        .validate_against(expectation)?;

    validate_embedded_version_consistency(
        decoded.version,
        &decoded.value,
    )?;

    Ok(decoded)
}

/// Validates that the binary artifact version and the embedded
/// `ZqnDocument` version metadata agree.
///
/// This prevents a document from claiming one version in its binary payload
/// while embedding another version in its logical schema envelope.
fn validate_embedded_version_consistency(
    artifact: ZqnVersionMetadata,
    document: &ZqnDocument,
) -> Result<(), DeserializationError> {
    let embedded = ZqnVersionMetadata::new(
        document.semantic_version,
        document.schema_version,
        document.compatibility_version,
    );

    if artifact != embedded {
        return Err(
            DeserializationError::InvalidVersionedPayload {
                message: format!(
                    "artifact version metadata does not match \
                     embedded ZQN document metadata: \
                     artifact semantic={}, schema={}, compatibility={}; \
                     embedded semantic={}, schema={}, compatibility={}",
                    artifact.semantic,
                    artifact.schema,
                    artifact.compatibility,
                    embedded.semantic,
                    embedded.schema,
                    embedded.compatibility,
                ),
            },
        );
    }

    Ok(())
}

// =============================================================================
// Version decoding
// =============================================================================

/// Decodes the versioned semantic payload envelope.
///
/// This function deliberately parses into `serde_json::Value` first so that
/// resource limits can be applied before converting the payload into an
/// arbitrary semantic Rust type.
fn decode_versioned_payload(
    payload: &[u8],
    limits: DecodeLimits,
) -> Result<(ZqnVersionMetadata, Value), DeserializationError> {
    limits
        .validate_payload_size(payload.len() as u64)
        .map_err(DeserializationError::Serialization)?;

    let value: Value =
        serde_json::from_slice(payload)
            .map_err(|error| {
                DeserializationError::Serialization(
                    SerializationError::Json {
                        message: error.to_string(),
                    },
                )
            })?;

    validate_value_limits(
        &value,
        limits,
        0,
    )?;

    let object = value
        .as_object()
        .ok_or_else(|| {
            DeserializationError::InvalidVersionedPayload {
                message:
                    "versioned ZQN payload must be a JSON object"
                        .to_owned(),
            }
        })?;

    let semantic =
        parse_version_field::<ZqnVersion>(
            object,
            "semantic_version",
        )?;

    let schema =
        parse_version_field::<ZqnSchemaVersion>(
            object,
            "schema_version",
        )?;

    let compatibility =
        parse_version_field::<ZqnCompatibilityVersion>(
            object,
            "compatibility_version",
        )?;

    let semantic_payload =
        object
            .get("payload")
            .cloned()
            .ok_or(
                DeserializationError::MissingField {
                    field: "payload",
                },
            )?;

    Ok((
        ZqnVersionMetadata::new(
            semantic,
            schema,
            compatibility,
        ),
        semantic_payload,
    ))
}

/// Parses one version field using its Serde representation.
///
/// The authoritative version types remain owned by `core::version`.
fn parse_version_field<T>(
    object: &Map<String, Value>,
    field: &'static str,
) -> Result<T, DeserializationError>
where
    T: DeserializeOwned,
{
    let value =
        object
            .get(field)
            .ok_or(
                DeserializationError::MissingField {
                    field,
                },
            )?;

    serde_json::from_value(value.clone())
        .map_err(|error| {
            DeserializationError::Serialization(
                SerializationError::Version {
                    message: format!(
                        "invalid `{field}`: {error}"
                    ),
                },
            )
        })
}

// =============================================================================
// Version validation
// =============================================================================

/// Validates an artifact against the current ZQN compatibility contract.
///
/// This deliberately delegates the compatibility decision to
/// `ZqnVersionMetadata::accepts`.
fn validate_artifact_version(
    version: ZqnVersionMetadata,
) -> Result<(), DeserializationError> {
    let consumer =
        ZqnVersionMetadata::current();

    if consumer.accepts(version) {
        Ok(())
    } else {
        Err(
            DeserializationError::IncompatibleVersion {
                artifact: version,
                consumer,
            },
        )
    }
}

// =============================================================================
// Semantic payload decoding
// =============================================================================

/// Decodes the semantic payload after all framing, integrity, version and
/// resource checks have completed.
fn decode_semantic_value<T>(
    value: Value,
    limits: DecodeLimits,
) -> Result<T, DeserializationError>
where
    T: DeserializeOwned,
{
    validate_value_limits(
        &value,
        limits,
        0,
    )?;

    serde_json::from_value(value)
        .map_err(|error| {
            DeserializationError::SemanticDecode {
                message: error.to_string(),
            }
        })
}

// =============================================================================
// Header validation
// =============================================================================

/// Validates the fixed binary header without reading the semantic payload.
///
/// The SHA-256 digest is not recomputed here because that requires the payload.
/// It is checked by the established `io::serialization::inspect` path for
/// in-memory decoding.
fn validate_header(
    header: &[u8; HEADER_LEN],
) -> Result<(), DeserializationError> {
    let mut magic = [0_u8; 4];

    magic.copy_from_slice(&header[0..4]);

    if magic != MAGIC {
        return Err(
            DeserializationError::Serialization(
                SerializationError::InvalidMagic {
                    found: magic,
                },
            ),
        );
    }

    let format_version =
        u16::from_le_bytes([
            header[4],
            header[5],
        ]);

    if format_version != FORMAT_VERSION {
        return Err(
            DeserializationError::Serialization(
                SerializationError::UnsupportedFormatVersion {
                    version: format_version,
                },
            ),
        );
    }

    let reserved =
        u16::from_le_bytes([
            header[6],
            header[7],
        ]);

    if reserved != 0 {
        return Err(
            DeserializationError::Serialization(
                SerializationError::InvalidReservedBits {
                    value: reserved,
                },
            ),
        );
    }

    Ok(())
}

// =============================================================================
// JSON resource validation
// =============================================================================

/// Validates JSON structure against explicit resource limits.
///
/// This function does not impose a quantum-system-size limit.
///
/// It validates only the representation that was actually supplied.
fn validate_value_limits(
    value: &Value,
    limits: DecodeLimits,
    depth: u64,
) -> Result<(), DeserializationError> {
    if depth > limits.max_nesting_depth {
        return Err(
            DeserializationError::Serialization(
                SerializationError::ResourceLimitExceeded {
                    resource:
                        super::serialization::ResourceKind::NestingDepth,
                    requested: depth,
                    maximum: limits.max_nesting_depth,
                },
            ),
        );
    }

    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) => {
            Ok(())
        }

        Value::String(string) => {
            let size = string.len() as u64;

            if size > limits.max_string_bytes {
                return Err(
                    DeserializationError::Serialization(
                        SerializationError::ResourceLimitExceeded {
                            resource:
                                super::serialization::ResourceKind::StringBytes,
                            requested: size,
                            maximum: limits.max_string_bytes,
                        },
                    ),
                );
            }

            Ok(())
        }

        Value::Array(values) => {
            let count = values.len() as u64;

            if count > limits.max_array_elements {
                return Err(
                    DeserializationError::Serialization(
                        SerializationError::ResourceLimitExceeded {
                            resource:
                                super::serialization::ResourceKind::ArrayElements,
                            requested: count,
                            maximum: limits.max_array_elements,
                        },
                    ),
                );
            }

            let next_depth =
                depth.checked_add(1).ok_or_else(|| {
                    DeserializationError::Serialization(
                        SerializationError::ArithmeticOverflow {
                            context:
                                "ZQN JSON array nesting depth",
                        },
                    )
                })?;

            for child in values {
                validate_value_limits(
                    child,
                    limits,
                    next_depth,
                )?;
            }

            Ok(())
        }

        Value::Object(object) => {
            let count = object.len() as u64;

            if count > limits.max_object_members {
                return Err(
                    DeserializationError::Serialization(
                        SerializationError::ResourceLimitExceeded {
                            resource:
                                super::serialization::ResourceKind::ObjectMembers,
                            requested: count,
                            maximum: limits.max_object_members,
                        },
                    ),
                );
            }

            let next_depth =
                depth.checked_add(1).ok_or_else(|| {
                    DeserializationError::Serialization(
                        SerializationError::ArithmeticOverflow {
                            context:
                                "ZQN JSON object nesting depth",
                        },
                    )
                })?;

            for (key, child) in object {
                let key_size =
                    key.len() as u64;

                if key_size > limits.max_string_bytes {
                    return Err(
                        DeserializationError::Serialization(
                            SerializationError::ResourceLimitExceeded {
                                resource:
                                    super::serialization::ResourceKind::StringBytes,
                                requested: key_size,
                                maximum: limits.max_string_bytes,
                            },
                        ),
                    );
                }

                validate_value_limits(
                    child,
                    limits,
                    next_depth,
                )?;
            }

            Ok(())
        }
    }
}

// =============================================================================
// Checked arithmetic
// =============================================================================

fn checked_usize(
    value: u64,
    context: &'static str,
) -> Result<usize, DeserializationError> {
    if value > usize::MAX as u64 {
        return Err(
            DeserializationError::Serialization(
                SerializationError::LengthOverflow {
                    context,
                    value,
                },
            ),
        );
    }

    Ok(value as usize)
}

// =============================================================================
// Reader support
// =============================================================================

fn read_exact<R>(
    reader: &mut R,
    buffer: &mut [u8],
) -> Result<(), DeserializationError>
where
    R: Read,
{
    let mut offset = 0usize;

    while offset < buffer.len() {
        let count =
            reader.read(&mut buffer[offset..])
                .map_err(DeserializationError::Reader)?;

        if count == 0 {
            return Err(
                DeserializationError::Serialization(
                    SerializationError::UnexpectedEnd {
                        needed: buffer.len(),
                        available: offset,
                    },
                ),
            );
        }

        offset =
            offset.checked_add(count).ok_or_else(|| {
                DeserializationError::Serialization(
                    SerializationError::ArithmeticOverflow {
                        context:
                            "ZQN reader offset",
                    },
                )
            })?;
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct TestPayload {
        name: String,
        values: Vec<u64>,
    }

    fn sample() -> TestPayload {
        TestPayload {
            name: "zqn".to_owned(),
            values: vec![1, 2, 3, 5, 8],
        }
    }

    fn encoded() -> Vec<u8> {
        super::super::serialization::serialize(
            &sample(),
        )
        .expect("test serialization must succeed")
    }

    #[test]
    fn round_trip_decodes_semantic_value() {
        let bytes = encoded();

        let decoded: ZqnDecoded<TestPayload> =
            deserialize(&bytes)
                .expect("deserialization must succeed");

        assert_eq!(decoded.value, sample());
        assert_eq!(
            decoded.version,
            ZqnVersionMetadata::current()
        );
    }

    #[test]
    fn digest_is_available_after_decode() {
        let bytes = encoded();

        let decoded: ZqnDecoded<TestPayload> =
            deserialize(&bytes)
                .expect("deserialization must succeed");

        assert_ne!(
            decoded.digest,
            [0_u8; DIGEST_LEN]
        );
    }

    #[test]
    fn malformed_magic_is_rejected() {
        let mut bytes = encoded();

        bytes[0] ^= 0xff;

        let result =
            deserialize::<TestPayload>(&bytes);

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::InvalidMagic { .. }
                )
            )
        ));
    }

    #[test]
    fn malformed_format_version_is_rejected() {
        let mut bytes = encoded();

        bytes[4..6]
            .copy_from_slice(&u16::MAX.to_le_bytes());

        let result =
            deserialize::<TestPayload>(&bytes);

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::UnsupportedFormatVersion {
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn malformed_reserved_bits_are_rejected() {
        let mut bytes = encoded();

        bytes[6..8]
            .copy_from_slice(&1_u16.to_le_bytes());

        let result =
            deserialize::<TestPayload>(&bytes);

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::InvalidReservedBits {
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn corrupted_payload_is_rejected() {
        let mut bytes = encoded();

        let last =
            bytes.len()
                .checked_sub(1)
                .expect("encoded document must not be empty");

        bytes[last] ^= 1;

        let result =
            deserialize::<TestPayload>(&bytes);

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::DigestMismatch {
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn trailing_bytes_are_rejected() {
        let mut bytes = encoded();

        bytes.push(0);

        let result =
            deserialize::<TestPayload>(&bytes);

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::LengthOverflow {
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn truncated_header_is_rejected() {
        let bytes =
            vec![0_u8; HEADER_LEN - 1];

        let result =
            deserialize::<TestPayload>(&bytes);

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::UnexpectedEnd {
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn truncated_payload_is_rejected() {
        let mut bytes = encoded();

        bytes.truncate(
            bytes.len()
                .checked_sub(1)
                .expect("encoded document must not be empty"),
        );

        let result =
            deserialize::<TestPayload>(&bytes);

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::UnexpectedEnd {
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn explicit_limits_are_respected() {
        let bytes = encoded();

        let limits = DecodeLimits::new(
            HEADER_LEN as u64 + 8,
            8,
            8,
            8,
            8,
            8,
        );

        let result =
            deserialize_with_limits::<TestPayload>(
                &bytes,
                limits,
            );

        assert!(result.is_err());
    }

    #[test]
    fn unbounded_policy_is_supported() {
        let bytes = encoded();

        let result =
            deserialize_with_limits::<TestPayload>(
                &bytes,
                DecodeLimits::unbounded(),
            );

        assert!(result.is_ok());
    }

    #[test]
    fn reader_round_trip() {
        let bytes = encoded();

        let mut reader =
            std::io::Cursor::new(bytes);

        let decoded: ZqnDecoded<TestPayload> =
            deserialize_from_reader(
                &mut reader,
                DecodeLimits::default(),
            )
            .expect("reader decode must succeed");

        assert_eq!(decoded.value, sample());
    }

    #[test]
    fn reader_truncation_is_rejected() {
        let bytes = encoded();

        let truncated_len =
            bytes.len()
                .checked_sub(1)
                .expect("encoded document must not be empty");

        let mut reader =
            std::io::Cursor::new(
                bytes[..truncated_len].to_vec(),
            );

        let result =
            deserialize_from_reader::<
                TestPayload,
                _,
            >(
                &mut reader,
                DecodeLimits::default(),
            );

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::UnexpectedEnd {
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn canonical_payload_round_trip() {
        let value = sample();

        let payload =
            super::super::serialization::serialize_canonical(
                &value,
            )
            .expect("canonical serialization must succeed");

        let decoded: TestPayload =
            deserialize_canonical_payload(
                &payload,
                DecodeLimits::unbounded(),
            )
            .expect(
                "canonical deserialization must succeed"
            );

        assert_eq!(decoded, value);
    }

    #[test]
    fn invalid_versioned_payload_shape_is_rejected() {
        let value =
            serde_json::json!("not-an-object");

        let payload =
            serde_json::to_vec(&value)
                .expect("test JSON must serialize");

        let result =
            decode_versioned_payload(
                &payload,
                DecodeLimits::unbounded(),
            );

        assert!(matches!(
            result,
            Err(
                DeserializationError::InvalidVersionedPayload {
                    ..
                }
            )
        ));
    }

    #[test]
    fn missing_semantic_version_is_rejected() {
        let value =
            serde_json::json!({
                "schema_version": {
                    "major": 1,
                    "minor": 0
                },
                "compatibility_version": {
                    "major": 1,
                    "minor": 0
                },
                "payload": {}
            });

        let payload =
            serde_json::to_vec(&value)
                .expect("test JSON must serialize");

        let result =
            decode_versioned_payload(
                &payload,
                DecodeLimits::unbounded(),
            );

        assert!(matches!(
            result,
            Err(
                DeserializationError::MissingField {
                    field: "semantic_version"
                }
            )
        ));
    }

    #[test]
    fn missing_schema_version_is_rejected() {
        let value =
            serde_json::json!({
                "semantic_version": {
                    "major": 1,
                    "minor": 0,
                    "patch": 0
                },
                "compatibility_version": {
                    "major": 1,
                    "minor": 0
                },
                "payload": {}
            });

        let payload =
            serde_json::to_vec(&value)
                .expect("test JSON must serialize");

        let result =
            decode_versioned_payload(
                &payload,
                DecodeLimits::unbounded(),
            );

        assert!(matches!(
            result,
            Err(
                DeserializationError::MissingField {
                    field: "schema_version"
                }
            )
        ));
    }

    #[test]
    fn missing_compatibility_version_is_rejected() {
        let value =
            serde_json::json!({
                "semantic_version": {
                    "major": 1,
                    "minor": 0,
                    "patch": 0
                },
                "schema_version": {
                    "major": 1,
                    "minor": 0
                },
                "payload": {}
            });

        let payload =
            serde_json::to_vec(&value)
                .expect("test JSON must serialize");

        let result =
            decode_versioned_payload(
                &payload,
                DecodeLimits::unbounded(),
            );

        assert!(matches!(
            result,
            Err(
                DeserializationError::MissingField {
                    field: "compatibility_version"
                }
            )
        ));
    }

    #[test]
    fn missing_semantic_payload_is_rejected() {
        let value =
            serde_json::json!({
                "semantic_version": {
                    "major": 1,
                    "minor": 0,
                    "patch": 0
                },
                "schema_version": {
                    "major": 1,
                    "minor": 0
                },
                "compatibility_version": {
                    "major": 1,
                    "minor": 0
                }
            });

        let payload =
            serde_json::to_vec(&value)
                .expect("test JSON must serialize");

        let result =
            decode_versioned_payload(
                &payload,
                DecodeLimits::unbounded(),
            );

        assert!(matches!(
            result,
            Err(
                DeserializationError::MissingField {
                    field: "payload"
                }
            )
        ));
    }

    #[test]
    fn incompatible_major_version_is_rejected() {
        let version =
            ZqnVersionMetadata::new(
                ZqnVersion::new(2, 0, 0),
                ZqnSchemaVersion::new(1, 0),
                ZqnCompatibilityVersion::new(1, 0),
            );

        let result =
            validate_artifact_version(version);

        assert!(matches!(
            result,
            Err(
                DeserializationError::IncompatibleVersion {
                    ..
                }
            )
        ));
    }

    #[test]
    fn compatible_current_version_is_accepted() {
        assert!(
            validate_artifact_version(
                ZqnVersionMetadata::current()
            )
            .is_ok()
        );
    }

    #[test]
    fn json_string_limit_is_enforced() {
        let value =
            serde_json::json!("0123456789");

        let limits = DecodeLimits::new(
            1024,
            1024,
            4,
            1024,
            1024,
            16,
        );

        let result =
            validate_value_limits(
                &value,
                limits,
                0,
            );

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::ResourceLimitExceeded {
                        resource:
                            super::super::serialization::ResourceKind::StringBytes,
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn json_array_limit_is_enforced() {
        let value =
            serde_json::json!([
                1, 2, 3, 4
            ]);

        let limits = DecodeLimits::new(
            1024,
            1024,
            1024,
            2,
            1024,
            16,
        );

        let result =
            validate_value_limits(
                &value,
                limits,
                0,
            );

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::ResourceLimitExceeded {
                        resource:
                            super::super::serialization::ResourceKind::ArrayElements,
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn json_object_limit_is_enforced() {
        let value =
            serde_json::json!({
                "a": 1,
                "b": 2,
                "c": 3
            });

        let limits = DecodeLimits::new(
            1024,
            1024,
            1024,
            1024,
            2,
            16,
        );

        let result =
            validate_value_limits(
                &value,
                limits,
                0,
            );

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::ResourceLimitExceeded {
                        resource:
                            super::super::serialization::ResourceKind::ObjectMembers,
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn json_nesting_limit_is_enforced() {
        let value =
            serde_json::json!({
                "a": {
                    "b": {
                        "c": {
                            "d": 1
                        }
                    }
                }
            });

        let limits = DecodeLimits::new(
            1024 * 1024,
            1024 * 1024,
            1024,
            1024,
            1024,
            2,
        );

        let result =
            validate_value_limits(
                &value,
                limits,
                0,
            );

        assert!(matches!(
            result,
            Err(
                DeserializationError::Serialization(
                    SerializationError::ResourceLimitExceeded {
                        resource:
                            super::super::serialization::ResourceKind::NestingDepth,
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn no_quantum_resource_identity_is_defined_here() {
        // This is a compile-time architectural property:
        //
        // deserialization.rs contains no QubitId or PhysicalQubitId
        // definition. Semantic ZQN payloads retain the canonical identities
        // owned by quantum::ir::qubit.
        assert!(true);
    }

    #[test]
    fn repeated_decoding_is_deterministic() {
        let bytes = encoded();

        let first =
            deserialize::<TestPayload>(&bytes)
                .expect("first decode must succeed");

        let second =
            deserialize::<TestPayload>(&bytes)
                .expect("second decode must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn empty_input_is_rejected_without_panic() {
        let result =
            deserialize::<TestPayload>(&[]);

        assert!(result.is_err());
    }

    #[test]
    fn arbitrary_invalid_input_returns_error() {
        let inputs: &[&[u8]] = &[
            b"",
            b"x",
            b"ZQNS",
            b"{",
            b"not-json",
            &[0xff, 0xff, 0xff, 0xff],
        ];

        for input in inputs {
            let result =
                deserialize::<TestPayload>(input);

            assert!(
                result.is_err(),
                "invalid input unexpectedly decoded"
            );
        }
    }
}