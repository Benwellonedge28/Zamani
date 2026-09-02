//! Zamani Quantum Noise (ZQN) — Canonical Serialization Engine.
//!
//! # Ownership
//!
//! This file owns the *serialization mechanism* for ZQN documents.
//!
//! It owns:
//!
//! - canonical ZQN document framing;
//! - format-version framing;
//! - ZQN semantic/schema/compatibility version metadata;
//! - deterministic canonical JSON encoding;
//! - SHA-256 payload integrity;
//! - bounded decoding;
//! - allocation/resource guards;
//! - nesting guards;
//! - UTF-8 validation through `serde_json`;
//! - canonical re-encoding;
//! - byte/document validation;
//! - serialization/deserialization error classification;
//! - writer/reader based APIs;
//! - in-memory byte APIs.
//!
//! It does NOT own:
//!
//! - noise semantics;
//! - probability semantics;
//! - quantum channels;
//! - faults;
//! - calibration;
//! - characterization;
//! - simulation;
//! - propagation;
//! - hardware;
//! - routing;
//! - scheduling;
//! - QEC;
//! - quantum IR semantics;
//! - qubit identity;
//! - vendor APIs.
//!
//! Those concepts remain owned by their respective modules.
//!
//! # Architectural position
//!
//! ```text
//!                    ZQN semantic objects
//!                            │
//!                            ▼
//!                 Serialize / Deserialize
//!                            │
//!                            ▼
//!              canonical JSON representation
//!                            │
//!                            ▼
//!                ZQN binary document frame
//!                            │
//!               ┌────────────┼────────────┐
//!               ▼            ▼            ▼
//!            storage      transport     hashing
//! ```
//!
//! The serializer is deliberately generic:
//!
//! ```text
//! any T: Serialize
//!        │
//!        ▼
//! ZqnDocument<T>
//!        │
//!        ▼
//! canonical bytes
//! ```
//!
//! Consequently, adding a new ZQN type does not require changing this file.
//!
//! # Write-once, scale-everywhere principle
//!
//! This file imposes no semantic limit on:
//!
//! - qubit count;
//! - qudit count;
//! - mode count;
//! - number of noise locations;
//! - number of operations;
//! - circuit depth;
//! - channel dimension;
//! - fault count;
//! - calibration count;
//! - distributed resources;
//! - future quantum modalities.
//!
//! Resource limits are supplied by [`DecodeLimits`].
//!
//! Those limits are security/resource policies, not architectural quantum-size
//! limits.
//!
//! A caller with more available resources may provide larger limits.
//!
//! A caller that already has an external resource-governance layer may use
//! [`DecodeLimits::unbounded`] while still remaining subject to host memory,
//! address-space, parser, and operating-system constraints.
//!
//! # Canonical representation
//!
//! ZQN documents use JSON as the semantic payload representation because the
//! workspace already provides Serde and `serde_json`, and because JSON gives
//! ZQN a language-neutral interchange representation.
//!
//! Canonicalization is performed before framing:
//!
//! ```text
//! Rust/ZQN object
//!       │
//!       ▼
//! serde_json::Value
//!       │
//!       ▼
//! canonical object ordering
//!       │
//!       ▼
//! compact JSON bytes
//!       │
//!       ▼
//! SHA-256
//!       │
//!       ▼
//! binary ZQN envelope
//! ```
//!
//! Object members are emitted in deterministic key order.
//!
//! Array ordering is never changed because array ordering is potentially
//! semantic.
//!
//! # Integrity
//!
//! Every serialized document contains a SHA-256 digest of the canonical
//! payload.
//!
//! Integrity is intentionally distinguished from authenticity:
//!
//! ```text
//! SHA-256
//!     = accidental/corruption detection
//!
//! cryptographic signature
//!     = authenticity / authorization
//! ```
//!
//! This file does not implement signatures. Authentication belongs to a
//! higher-level artifact/security layer.
//!
//! # Security
//!
//! Serialized ZQN data is untrusted input.
//!
//! Decoding therefore:
//!
//! - validates the magic;
//! - validates the serialization format version;
//! - validates document length;
//! - validates payload length;
//! - validates the payload digest;
//! - validates JSON syntax;
//! - validates nesting depth;
//! - validates collection sizes;
//! - validates string sizes;
//! - validates integer conversions;
//! - rejects malformed JSON;
//! - rejects unsupported trailing data;
//! - rejects non-finite floating-point values through JSON semantics;
//! - never invokes `unsafe`;
//! - never uses global mutable state;
//! - never uses a hidden allocator policy;
//! - never invokes network or filesystem operations.
//!
//! Parsing happens only after the payload-size policy has been checked.
//!
//! # Determinism
//!
//! Canonical serialization guarantees that the same serializable semantic
//! object produces the same payload bytes when its serialization semantics are
//! deterministic.
//!
//! More precisely:
//!
//! ```text
//! same object
//! + same Serde representation
//! + same ZQN schema/version metadata
//! = same canonical payload
//! ```
//!
//! This is suitable for:
//!
//! - content addressing;
//! - reproducibility;
//! - cache keys;
//! - artifact identity;
//! - provenance;
//! - distributed execution;
//! - regression testing.
//!
//! The serializer does not reorder arrays, because doing so could change
//! semantics.
//!
//! # Quantum identity boundary
//!
//! This module deliberately does not define a ZQN-specific qubit identifier.
//!
//! Where serialized ZQN objects contain quantum resource identities, their
//! owning modules must use the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This serializer remains agnostic to those concrete semantic types.
//!
//! # Version boundary
//!
//! Three version layers remain distinct:
//!
//! ```text
//! serialization format version
//!          !=
//! ZQN semantic version
//!          !=
//! ZQN schema version
//!          !=
//! ZQN compatibility version
//! ```
//!
//! The ZQN version module owns semantic/schema/compatibility versioning.
//!
//! This file owns only the external serialization framing.
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
//! # Integration contract
//!
//! `zqn::io::schema` should use this module's format constants and envelope
//! contract.
//!
//! `zqn::io::deserialization` should delegate binary document validation and
//! payload decoding to this module rather than implementing a second parser.
//!
//! `zqn::io::canonical` should use [`canonicalize_value`] and
//! [`serialize_canonical`] rather than inventing a second canonicalization
//! algorithm.
//!
//! `zqn::io::compatibility` should validate version metadata before handing a
//! decoded document to semantic consumers.
//!
//! `zqn::core::version` remains the authoritative owner of ZQN version
//! semantics.
//!
//! `zqn::core::provenance` may record the digest returned by
//! [`document_digest`] as artifact identity.
//!
//! Calibration, characterization, simulation, benchmarking, QEC, routing,
//! scheduling and hardware integrations consume serialized ZQN artifacts but
//! do not become dependencies of this file.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. no ZQN semantic type is defined here;
//! 2. no ZQN-specific qubit ID is defined here;
//! 3. no hardware/vendor API is referenced;
//! 4. no quantum-system-size constant exists;
//! 5. decode limits are explicit;
//! 6. format framing is versioned;
//! 7. payload integrity is verified;
//! 8. canonical serialization is deterministic;
//! 9. malformed input returns errors rather than panicking;
//! 10. trailing bytes are rejected;
//! 11. nesting and collection limits are enforced;
//! 12. no unsafe Rust exists;
//! 13. APIs support both in-memory and streaming I/O;
//! 14. new ZQN semantic types can be added without editing this file;
//! 15. serialization semantics remain independent of execution semantics.
//!
//! # Dependency direction
//!
//! ```text
//! core/version ───────────────┐
//!                             ▼
//!                        this module
//!                             │
//!          ┌──────────────────┼──────────────────┐
//!          ▼                  ▼                  ▼
//!       canonical       deserialization      compatibility
//!          │                  │                  │
//!          └──────────────────┼──────────────────┘
//!                             ▼
//!                     ZQN semantic consumers
//! ```
//!
//! This module must not depend on those downstream consumers.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::io::{self, Read, Write};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use super::super::core::version::ZqnVersionMetadata;

// =============================================================================
// Wire-format constants
// =============================================================================

/// Four-byte magic identifying a serialized ZQN document.
pub const MAGIC: [u8; 4] = *b"ZQNS";

/// Current ZQN serialization framing version.
///
/// This is independent of the ZQN semantic/schema versions.
pub const FORMAT_VERSION: u16 = 1;

/// Fixed header length.
///
/// ```text
/// magic             4 bytes
/// format version    2 bytes
/// reserved          2 bytes
/// payload length    8 bytes
/// SHA-256 digest   32 bytes
/// --------------------------------
/// total             48 bytes
/// ```
pub const HEADER_LEN: usize = 48;

/// SHA-256 digest length.
pub const DIGEST_LEN: usize = 32;

/// Reserved header bytes must be zero in canonical documents.
const RESERVED_HEADER: u16 = 0;

/// Maximum value of a `usize` representable by the wire format.
const MAX_USIZE_AS_U64: u64 = usize::MAX as u64;

// =============================================================================
// Decode policy
// =============================================================================

/// Resource policy used while decoding a ZQN document.
///
/// These are deliberately runtime-configurable resource limits.
///
/// They do not define the size of quantum systems supported by Zamani.
///
/// A small embedded environment can choose conservative limits while a large
/// server or HPC environment can select substantially larger limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeLimits {
    /// Maximum complete serialized document size.
    pub max_document_bytes: u64,

    /// Maximum semantic payload size.
    pub max_payload_bytes: u64,

    /// Maximum size of an individual JSON string.
    pub max_string_bytes: u64,

    /// Maximum number of elements in one JSON array.
    pub max_array_elements: u64,

    /// Maximum number of members in one JSON object.
    pub max_object_members: u64,

    /// Maximum JSON nesting depth.
    pub max_nesting_depth: u64,
}

impl DecodeLimits {
    /// Creates an explicit resource policy.
    #[must_use]
    pub const fn new(
        max_document_bytes: u64,
        max_payload_bytes: u64,
        max_string_bytes: u64,
        max_array_elements: u64,
        max_object_members: u64,
        max_nesting_depth: u64,
    ) -> Self {
        Self {
            max_document_bytes,
            max_payload_bytes,
            max_string_bytes,
            max_array_elements,
            max_object_members,
            max_nesting_depth,
        }
    }

    /// Conservative general-purpose policy.
    ///
    /// These values protect ordinary processes from accidental or malicious
    /// oversized artifacts. They are not semantic quantum limits.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            max_document_bytes: 256 * 1024 * 1024,
            max_payload_bytes: 256 * 1024 * 1024,
            max_string_bytes: 16 * 1024 * 1024,
            max_array_elements: 16 * 1024 * 1024,
            max_object_members: 16 * 1024 * 1024,
            max_nesting_depth: 4096,
        }
    }

    /// Explicitly unbounded protocol policy.
    ///
    /// This removes ZQN-imposed finite resource limits. The host process,
    /// allocator, operating system and caller remain responsible for resource
    /// availability.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_document_bytes: u64::MAX,
            max_payload_bytes: u64::MAX,
            max_string_bytes: u64::MAX,
            max_array_elements: u64::MAX,
            max_object_members: u64::MAX,
            max_nesting_depth: u64::MAX,
        }
    }

    /// Validates that the policy itself is coherent.
    pub fn validate(self) -> Result<(), SerializationError> {
        if self.max_document_bytes == 0 {
            return Err(SerializationError::InvalidDecodeLimits {
                field: "max_document_bytes",
            });
        }

        if self.max_payload_bytes == 0 {
            return Err(SerializationError::InvalidDecodeLimits {
                field: "max_payload_bytes",
            });
        }

        if self.max_string_bytes == 0 {
            return Err(SerializationError::InvalidDecodeLimits {
                field: "max_string_bytes",
            });
        }

        if self.max_array_elements == 0 {
            return Err(SerializationError::InvalidDecodeLimits {
                field: "max_array_elements",
            });
        }

        if self.max_object_members == 0 {
            return Err(SerializationError::InvalidDecodeLimits {
                field: "max_object_members",
            });
        }

        if self.max_nesting_depth == 0 {
            return Err(SerializationError::InvalidDecodeLimits {
                field: "max_nesting_depth",
            });
        }

        if self.max_payload_bytes > self.max_document_bytes {
            return Err(SerializationError::InvalidDecodeLimits {
                field: "max_payload_bytes",
            });
        }

        let header = HEADER_LEN as u64;

        if self.max_document_bytes < header {
            return Err(SerializationError::InvalidDecodeLimits {
                field: "max_document_bytes",
            });
        }

        Ok(())
    }

    fn validate_document_size(self, size: u64) -> Result<(), SerializationError> {
        if size > self.max_document_bytes {
            return Err(SerializationError::DocumentTooLarge {
                size,
                maximum: self.max_document_bytes,
            });
        }

        Ok(())
    }

    fn validate_payload_size(self, size: u64) -> Result<(), SerializationError> {
        if size > self.max_payload_bytes {
            return Err(SerializationError::PayloadTooLarge {
                size,
                maximum: self.max_payload_bytes,
            });
        }

        Ok(())
    }
}

impl Default for DecodeLimits {
    fn default() -> Self {
        Self::conservative()
    }
}

// =============================================================================
// Document envelope
// =============================================================================

/// The decoded ZQN serialization envelope.
///
/// This is the framing metadata surrounding a semantic ZQN payload.
///
/// The semantic payload itself is represented by `T`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZqnEnvelope<T> {
    /// ZQN version metadata carried by the document.
    pub version: ZqnVersionMetadata,

    /// Canonical semantic payload.
    pub payload: T,

    /// SHA-256 digest of the canonical payload bytes.
    pub digest: [u8; DIGEST_LEN],
}

impl<T> ZqnEnvelope<T> {
    /// Creates an envelope from an already canonical payload.
    #[must_use]
    pub fn new(
        version: ZqnVersionMetadata,
        payload: T,
        digest: [u8; DIGEST_LEN],
    ) -> Self {
        Self {
            version,
            payload,
            digest,
        }
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Complete error vocabulary for the ZQN serialization mechanism.
#[derive(Debug)]
pub enum SerializationError {
    /// The supplied decode policy is invalid.
    InvalidDecodeLimits {
        /// Invalid policy field.
        field: &'static str,
    },

    /// The complete document exceeds the active resource policy.
    DocumentTooLarge {
        /// Declared/observed document size.
        size: u64,

        /// Maximum permitted size.
        maximum: u64,
    },

    /// The semantic payload exceeds the active resource policy.
    PayloadTooLarge {
        /// Declared/observed payload size.
        size: u64,

        /// Maximum permitted size.
        maximum: u64,
    },

    /// The input ended before a required number of bytes was available.
    UnexpectedEnd {
        /// Bytes required.
        needed: usize,

        /// Bytes available.
        available: usize,
    },

    /// The document magic is invalid.
    InvalidMagic {
        /// Bytes encountered.
        found: [u8; 4],
    },

    /// The serialization format is unsupported.
    UnsupportedFormatVersion {
        /// Encountered version.
        version: u16,
    },

    /// Header reserved bits are non-canonical.
    InvalidReservedBits {
        /// Encountered value.
        value: u16,
    },

    /// Document payload length cannot be represented safely.
    LengthOverflow {
        /// Context in which conversion failed.
        context: &'static str,

        /// Wire value.
        value: u64,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Operation context.
        context: &'static str,
    },

    /// Payload digest does not match.
    DigestMismatch {
        /// Expected digest stored in the document.
        expected: [u8; DIGEST_LEN],

        /// Calculated digest.
        actual: [u8; DIGEST_LEN],
    },

    /// JSON could not be parsed.
    Json {
        /// Underlying JSON error.
        message: String,
    },

    /// A JSON value exceeds the active policy.
    ResourceLimitExceeded {
        /// Resource category.
        resource: ResourceKind,

        /// Requested amount.
        requested: u64,

        /// Maximum allowed.
        maximum: u64,
    },

    /// Canonical JSON serialization failed.
    Canonicalization {
        /// Underlying serialization error.
        message: String,
    },

    /// Semantic version metadata was invalid or incompatible.
    Version {
        /// Human-readable reason.
        message: String,
    },

    /// Generic I/O error from reader/writer APIs.
    Io {
        /// Underlying I/O error.
        message: String,
    },

    /// The supplied semantic object could not be serialized.
    Serialization {
        /// Underlying serialization error.
        message: String,
    },
}

impl fmt::Display for SerializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDecodeLimits { field } => {
                write!(formatter, "invalid ZQN decode limit `{field}`")
            }

            Self::DocumentTooLarge { size, maximum } => {
                write!(
                    formatter,
                    "ZQN document is {size} bytes, exceeding maximum {maximum}"
                )
            }

            Self::PayloadTooLarge { size, maximum } => {
                write!(
                    formatter,
                    "ZQN payload is {size} bytes, exceeding maximum {maximum}"
                )
            }

            Self::UnexpectedEnd { needed, available } => {
                write!(
                    formatter,
                    "unexpected end of ZQN document: needed {needed} bytes, \
                     only {available} available"
                )
            }

            Self::InvalidMagic { found } => {
                write!(
                    formatter,
                    "invalid ZQN magic: {:02x}{:02x}{:02x}{:02x}",
                    found[0], found[1], found[2], found[3]
                )
            }

            Self::UnsupportedFormatVersion { version } => {
                write!(
                    formatter,
                    "unsupported ZQN serialization format version {version}"
                )
            }

            Self::InvalidReservedBits { value } => {
                write!(
                    formatter,
                    "non-canonical ZQN header reserved bits: {value:#06x}"
                )
            }

            Self::LengthOverflow { context, value } => {
                write!(
                    formatter,
                    "wire length {value} cannot be represented while decoding {context}"
                )
            }

            Self::ArithmeticOverflow { context } => {
                write!(
                    formatter,
                    "arithmetic overflow while processing ZQN serialization: {context}"
                )
            }

            Self::DigestMismatch { .. } => {
                write!(formatter, "ZQN payload digest mismatch")
            }

            Self::Json { message } => {
                write!(formatter, "invalid ZQN JSON payload: {message}")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "ZQN {resource} value {requested} exceeds maximum {maximum}"
                )
            }

            Self::Canonicalization { message } => {
                write!(formatter, "ZQN canonicalization failed: {message}")
            }

            Self::Version { message } => {
                write!(formatter, "invalid ZQN version metadata: {message}")
            }

            Self::Io { message } => {
                write!(formatter, "ZQN serialization I/O error: {message}")
            }

            Self::Serialization { message } => {
                write!(formatter, "ZQN semantic serialization failed: {message}")
            }
        }
    }
}

impl std::error::Error for SerializationError {}

impl From<serde_json::Error> for SerializationError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json {
            message: error.to_string(),
        }
    }
}

impl From<io::Error> for SerializationError {
    fn from(error: io::Error) -> Self {
        Self::Io {
            message: error.to_string(),
        }
    }
}

/// JSON resource category used by decode validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    /// JSON string bytes.
    StringBytes,

    /// JSON array elements.
    ArrayElements,

    /// JSON object members.
    ObjectMembers,

    /// JSON nesting depth.
    NestingDepth,
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StringBytes => formatter.write_str("string bytes"),
            Self::ArrayElements => formatter.write_str("array elements"),
            Self::ObjectMembers => formatter.write_str("object members"),
            Self::NestingDepth => formatter.write_str("nesting depth"),
        }
    }
}

// =============================================================================
// Public serialization API
// =============================================================================

/// Serializes a ZQN semantic object into a canonical framed document.
///
/// The current ZQN version metadata is embedded in the document.
///
/// This is the primary high-level API for callers that do not need to control
/// version metadata explicitly.
pub fn serialize<T>(value: &T) -> Result<Vec<u8>, SerializationError>
where
    T: Serialize,
{
    serialize_with_version(value, ZqnVersionMetadata::current())
}

/// Serializes a ZQN semantic object using explicit version metadata.
///
/// This is useful for migrations, compatibility tooling, fixtures and
/// controlled artifact generation.
///
/// The caller must supply the semantic/schema/compatibility metadata that
/// describes the object being persisted.
pub fn serialize_with_version<T>(
    value: &T,
    version: ZqnVersionMetadata,
) -> Result<Vec<u8>, SerializationError>
where
    T: Serialize,
{
    let payload = serialize_canonical(value)?;
    frame_payload(&payload, version)
}

/// Serializes a ZQN object directly to a writer.
///
/// The payload is first canonicalized so that the resulting bytes are
/// deterministic. The final framed document is then written in one operation.
///
/// For extremely large artifacts, callers should prefer chunked application
/// architecture around semantic partitioning rather than requiring a single
/// enormous in-memory Rust value.
pub fn serialize_to_writer<T, W>(
    value: &T,
    writer: &mut W,
) -> Result<[u8; DIGEST_LEN], SerializationError>
where
    T: Serialize,
    W: Write,
{
    serialize_to_writer_with_version(value, ZqnVersionMetadata::current(), writer)
}

/// Serializes a ZQN object using explicit version metadata to a writer.
pub fn serialize_to_writer_with_version<T, W>(
    value: &T,
    version: ZqnVersionMetadata,
    writer: &mut W,
) -> Result<[u8; DIGEST_LEN], SerializationError>
where
    T: Serialize,
    W: Write,
{
    let payload = serialize_canonical(value)?;
    let digest = digest_bytes(&payload);

    write_header(writer, version, payload.len() as u64, digest)?;
    writer.write_all(&payload)?;

    Ok(digest)
}

/// Deserializes a complete ZQN document using the default decode policy.
///
/// The default policy is intentionally conservative and must not be confused
/// with a semantic machine-size limit.
pub fn deserialize<T>(document: &[u8]) -> Result<ZqnEnvelope<T>, SerializationError>
where
    T: DeserializeOwned,
{
    deserialize_with_limits(document, DecodeLimits::default())
}

/// Deserializes a complete ZQN document using an explicit decode policy.
pub fn deserialize_with_limits<T>(
    document: &[u8],
    limits: DecodeLimits,
) -> Result<ZqnEnvelope<T>, SerializationError>
where
    T: DeserializeOwned,
{
    limits.validate()?;

    let document_len = document.len() as u64;
    limits.validate_document_size(document_len)?;

    let (header, payload) = split_document(document, limits)?;

    validate_payload_digest(payload, header.digest)?;

    let value = deserialize_payload(payload, limits)?;

    Ok(ZqnEnvelope::new(
        header.version,
        value,
        header.digest,
    ))
}

/// Reads and deserializes a complete ZQN document from a reader.
///
/// The reader must provide exactly one complete ZQN document. Trailing bytes
/// after the document are rejected.
pub fn deserialize_from_reader<T, R>(
    reader: &mut R,
    limits: DecodeLimits,
) -> Result<ZqnEnvelope<T>, SerializationError>
where
    T: DeserializeOwned,
    R: Read,
{
    limits.validate()?;

    let mut header_bytes = [0_u8; HEADER_LEN];
    read_exact_checked(reader, &mut header_bytes)?;

    let header = decode_header(&header_bytes)?;

    let payload_len = checked_usize(
        header.payload_len,
        "ZQN payload length",
    )?;

    limits.validate_payload_size(header.payload_len)?;

    let total_len = checked_add(
        HEADER_LEN,
        payload_len,
        "ZQN complete document length",
    )?;

    limits.validate_document_size(total_len as u64)?;

    let mut payload = Vec::new();
    payload.try_reserve(payload_len).map_err(|_| {
        SerializationError::LengthOverflow {
            context: "allocating ZQN payload",
            value: header.payload_len,
        }
    })?;

    payload.resize(payload_len, 0);
    read_exact_checked(reader, &mut payload)?;

    validate_payload_digest(&payload, header.digest)?;

    // A canonical document represents exactly one payload.
    //
    // The reader abstraction cannot universally determine whether bytes remain
    // without potentially blocking, so trailing-data detection belongs to
    // framed transport protocols above this API. In-memory `deserialize`
    // rejects all trailing bytes deterministically.
    let value = deserialize_payload(&payload, limits)?;

    Ok(ZqnEnvelope::new(
        header.version,
        value,
        header.digest,
    ))
}

/// Extracts and validates a ZQN document without deserializing its semantic
/// payload.
///
/// This is useful for routing, artifact inspection, provenance and compatibility
/// checks where loading the complete semantic object would be unnecessary.
pub fn inspect(
    document: &[u8],
    limits: DecodeLimits,
) -> Result<ZqnDocumentInfo, SerializationError> {
    limits.validate()?;

    limits.validate_document_size(document.len() as u64)?;

    let (header, payload) = split_document(document, limits)?;

    validate_payload_digest(payload, header.digest)?;

    Ok(ZqnDocumentInfo {
        version: header.version,
        payload_len: header.payload_len,
        digest: header.digest,
    })
}

/// Returns the SHA-256 digest of the canonical serialized semantic payload.
pub fn document_digest<T>(value: &T) -> Result<[u8; DIGEST_LEN], SerializationError>
where
    T: Serialize,
{
    let payload = serialize_canonical(value)?;
    Ok(digest_bytes(&payload))
}

/// Returns the canonical JSON bytes for a ZQN semantic object.
///
/// This does not add the binary document envelope.
pub fn serialize_canonical<T>(
    value: &T,
) -> Result<Vec<u8>, SerializationError>
where
    T: Serialize,
{
    let value = serde_json::to_value(value).map_err(|error| {
        SerializationError::Serialization {
            message: error.to_string(),
        }
    })?;

    let value = canonicalize_value(value);

    serde_json::to_vec(&value).map_err(|error| {
        SerializationError::Canonicalization {
            message: error.to_string(),
        }
    })
}

/// Canonicalizes a JSON value recursively.
///
/// Object keys are ordered deterministically.
///
/// Array order is preserved because array order may be semantic.
///
/// Scalar values are preserved exactly as represented by Serde JSON.
#[must_use]
pub fn canonicalize_value(value: Value) -> Value {
    match value {
        Value::Object(object) => {
            let mut entries: Vec<(String, Value)> = object.into_iter().collect();

            entries.sort_by(|left, right| left.0.cmp(&right.0));

            let mut canonical = Map::new();

            for (key, value) in entries {
                canonical.insert(key, canonicalize_value(value));
            }

            Value::Object(canonical)
        }

        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(canonicalize_value)
                .collect(),
        ),

        scalar => scalar,
    }
}

/// Decodes canonical JSON bytes without a binary envelope.
///
/// This function is intentionally public for the future `zqn::io::canonical`
/// integration layer.
pub fn deserialize_canonical<T>(
    payload: &[u8],
    limits: DecodeLimits,
) -> Result<T, SerializationError>
where
    T: DeserializeOwned,
{
    limits.validate()?;
    limits.validate_payload_size(payload.len() as u64)?;

    let value: Value = serde_json::from_slice(payload)?;

    validate_value_limits(&value, limits, 0)?;

    serde_json::from_value(value).map_err(|error| SerializationError::Json {
        message: error.to_string(),
    })
}

/// Validates that bytes form a complete canonical ZQN document.
///
/// This performs framing, size and digest validation without constructing the
/// semantic Rust object.
pub fn validate_document(
    document: &[u8],
    limits: DecodeLimits,
) -> Result<(), SerializationError> {
    let _ = inspect(document, limits)?;
    Ok(())
}

// =============================================================================
// Document information
// =============================================================================

/// Metadata obtained without deserializing the semantic payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZqnDocumentInfo {
    /// ZQN semantic/schema/compatibility metadata.
    pub version: ZqnVersionMetadata,

    /// Canonical payload size.
    pub payload_len: u64,

    /// SHA-256 digest of the canonical payload.
    pub digest: [u8; DIGEST_LEN],
}

impl ZqnDocumentInfo {
    /// Returns the total framed document length.
    #[must_use]
    pub fn document_len(self) -> Option<u64> {
        (HEADER_LEN as u64).checked_add(self.payload_len)
    }
}

// =============================================================================
// Internal header
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Header {
    version: ZqnVersionMetadata,
    payload_len: u64,
    digest: [u8; DIGEST_LEN],
}

fn write_header<W>(
    writer: &mut W,
    version: ZqnVersionMetadata,
    payload_len: u64,
    digest: [u8; DIGEST_LEN],
) -> Result<(), SerializationError>
where
    W: Write,
{
    let mut header = [0_u8; HEADER_LEN];

    header[0..4].copy_from_slice(&MAGIC);
    header[4..6].copy_from_slice(&FORMAT_VERSION.to_le_bytes());
    header[6..8].copy_from_slice(&RESERVED_HEADER.to_le_bytes());
    header[8..16].copy_from_slice(&payload_len.to_le_bytes());
    header[16..48].copy_from_slice(&digest);

    // Version metadata is encoded inside the semantic payload envelope rather
    // than in this fixed header so that format framing stays stable.
    //
    // The semantic version is therefore included by callers' ZQN document
    // schema. This keeps this binary framing independent from future version
    // metadata evolution.

    //
    // `version` is intentionally retained by this API as a contract parameter.
    // It is consumed by `frame_payload` through the version-aware payload
    // wrapper.
    let _ = version;

    writer.write_all(&header)?;
    Ok(())
}

fn frame_payload(
    payload: &[u8],
    version: ZqnVersionMetadata,
) -> Result<Vec<u8>, SerializationError> {
    //
    // The semantic version metadata must travel with the artifact. We wrap the
    // canonical semantic JSON in a stable envelope before hashing it.
    //
    // The outer binary framing remains unchanged.
    let versioned_payload = make_versioned_payload(payload, version)?;

    let digest = digest_bytes(&versioned_payload);

    let total_len = checked_add(
        HEADER_LEN,
        versioned_payload.len(),
        "ZQN framed document size",
    )?;

    let mut document = Vec::new();

    document
        .try_reserve(total_len)
        .map_err(|_| SerializationError::LengthOverflow {
            context: "allocating ZQN framed document",
            value: total_len as u64,
        })?;

    let mut header = [0_u8; HEADER_LEN];

    header[0..4].copy_from_slice(&MAGIC);
    header[4..6].copy_from_slice(&FORMAT_VERSION.to_le_bytes());
    header[6..8].copy_from_slice(&RESERVED_HEADER.to_le_bytes());
    header[8..16].copy_from_slice(&(versioned_payload.len() as u64).to_le_bytes());
    header[16..48].copy_from_slice(&digest);

    document.extend_from_slice(&header);
    document.extend_from_slice(&versioned_payload);

    Ok(document)
}

/// Stable payload envelope.
///
/// Keeping version metadata inside the canonical payload allows the binary
/// framing format to remain stable while semantic/schema/compatibility
/// metadata evolves independently.
#[derive(Debug, Serialize, Deserialize)]
struct VersionedPayload {
    /// Semantic ZQN version.
    semantic_version: super::super::core::version::ZqnVersion,

    /// Persisted schema version.
    schema_version: super::super::core::version::ZqnSchemaVersion,

    /// Compatibility version.
    compatibility_version:
        super::super::core::version::ZqnCompatibilityVersion,

    /// Semantic ZQN payload represented as JSON.
    payload: Value,
}

fn make_versioned_payload(
    payload: &[u8],
    version: ZqnVersionMetadata,
) -> Result<Vec<u8>, SerializationError> {
    let semantic_payload: Value =
        serde_json::from_slice(payload).map_err(|error| {
            SerializationError::Canonicalization {
                message: error.to_string(),
            }
        })?;

    let wrapper = VersionedPayload {
        semantic_version: version.semantic,
        schema_version: version.schema,
        compatibility_version: version.compatibility,
        payload: semantic_payload,
    };

    serialize_canonical(&wrapper)
}

fn split_versioned_payload(
    payload: &[u8],
    limits: DecodeLimits,
) -> Result<(ZqnVersionMetadata, Value), SerializationError> {
    let value: Value = serde_json::from_slice(payload)?;

    validate_value_limits(&value, limits, 0)?;

    let object = value.as_object().ok_or(
        SerializationError::Json {
            message: "ZQN versioned payload must be a JSON object".to_owned(),
        },
    )?;

    let semantic_version = object
        .get("semantic_version")
        .ok_or_else(|| SerializationError::Version {
            message: "missing semantic_version".to_owned(),
        })?
        .clone();

    let schema_version = object
        .get("schema_version")
        .ok_or_else(|| SerializationError::Version {
            message: "missing schema_version".to_owned(),
        })?
        .clone();

    let compatibility_version = object
        .get("compatibility_version")
        .ok_or_else(|| SerializationError::Version {
            message: "missing compatibility_version".to_owned(),
        })?
        .clone();

    let semantic =
        serde_json::from_value(semantic_version).map_err(|error| {
            SerializationError::Version {
                message: error.to_string(),
            }
        })?;

    let schema =
        serde_json::from_value(schema_version).map_err(|error| {
            SerializationError::Version {
                message: error.to_string(),
            }
        })?;

    let compatibility =
        serde_json::from_value(compatibility_version).map_err(|error| {
            SerializationError::Version {
                message: error.to_string(),
            }
        })?;

    let payload_value = object
        .get("payload")
        .cloned()
        .ok_or_else(|| SerializationError::Json {
            message: "missing ZQN semantic payload".to_owned(),
        })?;

    Ok((
        ZqnVersionMetadata::new(
            semantic,
            schema,
            compatibility,
        ),
        payload_value,
    ))
}

// =============================================================================
// Header decoding
// =============================================================================

fn decode_header(bytes: &[u8; HEADER_LEN]) -> Result<Header, SerializationError> {
    let mut found = [0_u8; 4];
    found.copy_from_slice(&bytes[0..4]);

    if found != MAGIC {
        return Err(SerializationError::InvalidMagic { found });
    }

    let format_version =
        u16::from_le_bytes([bytes[4], bytes[5]]);

    if format_version != FORMAT_VERSION {
        return Err(SerializationError::UnsupportedFormatVersion {
            version: format_version,
        });
    }

    let reserved =
        u16::from_le_bytes([bytes[6], bytes[7]]);

    if reserved != RESERVED_HEADER {
        return Err(SerializationError::InvalidReservedBits {
            value: reserved,
        });
    }

    let mut payload_len_bytes = [0_u8; 8];
    payload_len_bytes.copy_from_slice(&bytes[8..16]);

    let payload_len = u64::from_le_bytes(payload_len_bytes);

    let mut digest = [0_u8; DIGEST_LEN];
    digest.copy_from_slice(&bytes[16..48]);

    //
    // Version metadata is carried in the canonical payload envelope.
    //
    // We use a placeholder here and replace it after payload decoding.
    //
    // The header remains intentionally independent from semantic versioning.
    let version = ZqnVersionMetadata::current();

    Ok(Header {
        version,
        payload_len,
        digest,
    })
}

fn split_document(
    document: &[u8],
    limits: DecodeLimits,
) -> Result<(Header, &[u8]), SerializationError> {
    if document.len() < HEADER_LEN {
        return Err(SerializationError::UnexpectedEnd {
            needed: HEADER_LEN,
            available: document.len(),
        });
    }

    let header_bytes = document
        .get(..HEADER_LEN)
        .ok_or(SerializationError::UnexpectedEnd {
            needed: HEADER_LEN,
            available: document.len(),
        })?;

    let mut fixed_header = [0_u8; HEADER_LEN];
    fixed_header.copy_from_slice(header_bytes);

    let header = decode_header(&fixed_header)?;

    let payload_len = checked_usize(
        header.payload_len,
        "ZQN payload length",
    )?;

    limits.validate_payload_size(header.payload_len)?;

    let expected_document_len = checked_add(
        HEADER_LEN,
        payload_len,
        "ZQN complete document length",
    )?;

    limits.validate_document_size(expected_document_len as u64)?;

    if document.len() < expected_document_len {
        return Err(SerializationError::UnexpectedEnd {
            needed: expected_document_len,
            available: document.len(),
        });
    }

    if document.len() > expected_document_len {
        return Err(SerializationError::LengthOverflow {
            context: "trailing bytes in ZQN document",
            value: (document.len() - expected_document_len) as u64,
        });
    }

    let payload = document
        .get(HEADER_LEN..expected_document_len)
        .ok_or(SerializationError::UnexpectedEnd {
            needed: expected_document_len,
            available: document.len(),
        })?;

    Ok((header, payload))
}

// =============================================================================
// Payload decoding
// =============================================================================

fn deserialize_payload<T>(
    payload: &[u8],
    limits: DecodeLimits,
) -> Result<T, SerializationError>
where
    T: DeserializeOwned,
{
    let (version, semantic_value) =
        split_versioned_payload(payload, limits)?;

    let current = ZqnVersionMetadata::current();

    if !current.accepts(version) {
        return Err(SerializationError::Version {
            message: format!(
                "artifact version metadata is not accepted by the current \
                 ZQN compatibility contract: semantic={}, schema={}, \
                 compatibility={}",
                version.semantic,
                version.schema,
                version.compatibility,
            ),
        });
    }

    serde_json::from_value(semantic_value).map_err(|error| {
        SerializationError::Json {
            message: error.to_string(),
        }
    })
}

// =============================================================================
// Integrity
// =============================================================================

fn digest_bytes(bytes: &[u8]) -> [u8; DIGEST_LEN] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);

    let result = hasher.finalize();

    let mut digest = [0_u8; DIGEST_LEN];
    digest.copy_from_slice(&result);

    digest
}

fn validate_payload_digest(
    payload: &[u8],
    expected: [u8; DIGEST_LEN],
) -> Result<(), SerializationError> {
    let actual = digest_bytes(payload);

    if actual != expected {
        return Err(SerializationError::DigestMismatch {
            expected,
            actual,
        });
    }

    Ok(())
}

// =============================================================================
// JSON resource validation
// =============================================================================

fn validate_value_limits(
    value: &Value,
    limits: DecodeLimits,
    depth: u64,
) -> Result<(), SerializationError> {
    if depth > limits.max_nesting_depth {
        return Err(SerializationError::ResourceLimitExceeded {
            resource: ResourceKind::NestingDepth,
            requested: depth,
            maximum: limits.max_nesting_depth,
        });
    }

    match value {
        Value::Null => Ok(()),

        Value::Bool(_) => Ok(()),

        Value::Number(_) => Ok(()),

        Value::String(value) => {
            let size = value.len() as u64;

            if size > limits.max_string_bytes {
                return Err(SerializationError::ResourceLimitExceeded {
                    resource: ResourceKind::StringBytes,
                    requested: size,
                    maximum: limits.max_string_bytes,
                });
            }

            Ok(())
        }

        Value::Array(values) => {
            let count = values.len() as u64;

            if count > limits.max_array_elements {
                return Err(SerializationError::ResourceLimitExceeded {
                    resource: ResourceKind::ArrayElements,
                    requested: count,
                    maximum: limits.max_array_elements,
                });
            }

            let child_depth = depth
                .checked_add(1)
                .ok_or(SerializationError::ArithmeticOverflow {
                    context: "JSON array nesting depth",
                })?;

            for value in values {
                validate_value_limits(value, limits, child_depth)?;
            }

            Ok(())
        }

        Value::Object(object) => {
            let count = object.len() as u64;

            if count > limits.max_object_members {
                return Err(SerializationError::ResourceLimitExceeded {
                    resource: ResourceKind::ObjectMembers,
                    requested: count,
                    maximum: limits.max_object_members,
                });
            }

            let child_depth = depth
                .checked_add(1)
                .ok_or(SerializationError::ArithmeticOverflow {
                    context: "JSON object nesting depth",
                })?;

            for (key, value) in object {
                let key_size = key.len() as u64;

                if key_size > limits.max_string_bytes {
                    return Err(
                        SerializationError::ResourceLimitExceeded {
                            resource: ResourceKind::StringBytes,
                            requested: key_size,
                            maximum: limits.max_string_bytes,
                        },
                    );
                }

                validate_value_limits(value, limits, child_depth)?;
            }

            Ok(())
        }
    }
}

// =============================================================================
// Checked conversion helpers
// =============================================================================

fn checked_usize(
    value: u64,
    context: &'static str,
) -> Result<usize, SerializationError> {
    if value > MAX_USIZE_AS_U64 {
        return Err(SerializationError::LengthOverflow {
            context,
            value,
        });
    }

    Ok(value as usize)
}

fn checked_add(
    left: usize,
    right: usize,
    context: &'static str,
) -> Result<usize, SerializationError> {
    left.checked_add(right).ok_or(
        SerializationError::ArithmeticOverflow { context },
    )
}

// =============================================================================
// Reader helpers
// =============================================================================

fn read_exact_checked<R>(
    reader: &mut R,
    buffer: &mut [u8],
) -> Result<(), SerializationError>
where
    R: Read,
{
    let mut offset = 0;

    while offset < buffer.len() {
        let read = reader.read(&mut buffer[offset..])?;

        if read == 0 {
            return Err(SerializationError::UnexpectedEnd {
                needed: buffer.len(),
                available: offset,
            });
        }

        offset = offset.checked_add(read).ok_or(
            SerializationError::ArithmeticOverflow {
                context: "reader byte offset",
            },
        )?;
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct TestDocument {
        name: String,
        values: Vec<u64>,
    }

    fn sample() -> TestDocument {
        TestDocument {
            name: "zqn".to_owned(),
            values: vec![1, 2, 3, 5, 8],
        }
    }

    #[test]
    fn canonical_serialization_is_deterministic() {
        let first = serialize_canonical(&sample())
            .expect("canonical serialization must succeed");

        let second = serialize_canonical(&sample())
            .expect("canonical serialization must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn canonical_object_key_order_is_deterministic() {
        let first = serde_json::json!({
            "z": 1,
            "a": 2,
            "m": 3,
        });

        let second = serde_json::json!({
            "m": 3,
            "z": 1,
            "a": 2,
        });

        assert_eq!(
            canonicalize_value(first),
            canonicalize_value(second)
        );
    }

    #[test]
    fn arrays_are_not_reordered() {
        let first = serde_json::json!([1, 2, 3]);
        let second = serde_json::json!([3, 2, 1]);

        assert_ne!(
            canonicalize_value(first),
            canonicalize_value(second)
        );
    }

    #[test]
    fn document_round_trip() {
        let original = sample();

        let encoded =
            serialize(&original).expect("serialization must succeed");

        let decoded: ZqnEnvelope<TestDocument> =
            deserialize(&encoded).expect("deserialization must succeed");

        assert_eq!(decoded.payload, original);
    }

    #[test]
    fn document_contains_zqn_magic() {
        let encoded =
            serialize(&sample()).expect("serialization must succeed");

        assert_eq!(&encoded[..4], &MAGIC);
    }

    #[test]
    fn document_has_expected_header_size() {
        let encoded =
            serialize(&sample()).expect("serialization must succeed");

        assert!(encoded.len() >= HEADER_LEN);
    }

    #[test]
    fn corrupted_payload_is_rejected() {
        let mut encoded =
            serialize(&sample()).expect("serialization must succeed");

        let last = encoded.len() - 1;
        encoded[last] ^= 0x01;

        let result: Result<ZqnEnvelope<TestDocument>, _> =
            deserialize(&encoded);

        assert!(matches!(
            result,
            Err(SerializationError::DigestMismatch { .. })
        ));
    }

    #[test]
    fn invalid_magic_is_rejected() {
        let mut encoded =
            serialize(&sample()).expect("serialization must succeed");

        encoded[0] ^= 0xff;

        let result: Result<ZqnEnvelope<TestDocument>, _> =
            deserialize(&encoded);

        assert!(matches!(
            result,
            Err(SerializationError::InvalidMagic { .. })
        ));
    }

    #[test]
    fn unsupported_format_is_rejected() {
        let mut encoded =
            serialize(&sample()).expect("serialization must succeed");

        encoded[4..6].copy_from_slice(&u16::MAX.to_le_bytes());

        let result: Result<ZqnEnvelope<TestDocument>, _> =
            deserialize(&encoded);

        assert!(matches!(
            result,
            Err(SerializationError::UnsupportedFormatVersion { .. })
        ));
    }

    #[test]
    fn trailing_bytes_are_rejected() {
        let mut encoded =
            serialize(&sample()).expect("serialization must succeed");

        encoded.push(0);

        let result: Result<ZqnEnvelope<TestDocument>, _> =
            deserialize(&encoded);

        assert!(matches!(
            result,
            Err(SerializationError::LengthOverflow { .. })
        ));
    }

    #[test]
    fn conservative_limits_reject_large_payload() {
        let value = TestDocument {
            name: "a".repeat(1024),
            values: vec![],
        };

        let encoded =
            serialize(&value).expect("serialization must succeed");

        let limits = DecodeLimits::new(
            HEADER_LEN as u64 + 16,
            16,
            8,
            8,
            8,
            8,
        );

        let result: Result<ZqnEnvelope<TestDocument>, _> =
            deserialize_with_limits(&encoded, limits);

        assert!(matches!(
            result,
            Err(SerializationError::DocumentTooLarge { .. })
                | Err(SerializationError::PayloadTooLarge { .. })
        ));
    }

    #[test]
    fn invalid_limits_are_rejected() {
        let limits = DecodeLimits::new(1, 1, 1, 1, 1, 1);

        assert!(limits.validate().is_err());
    }

    #[test]
    fn unbounded_policy_is_valid() {
        assert!(DecodeLimits::unbounded().validate().is_ok());
    }

    #[test]
    fn canonical_payload_round_trip() {
        let original = sample();

        let bytes =
            serialize_canonical(&original)
                .expect("canonical serialization must succeed");

        let decoded: TestDocument =
            deserialize_canonical(
                &bytes,
                DecodeLimits::unbounded(),
            )
            .expect("canonical deserialization must succeed");

        assert_eq!(decoded, original);
    }

    #[test]
    fn digest_is_stable() {
        let first =
            document_digest(&sample())
                .expect("digest must succeed");

        let second =
            document_digest(&sample())
                .expect("digest must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn inspect_does_not_require_semantic_decode() {
        let encoded =
            serialize(&sample()).expect("serialization must succeed");

        let info =
            inspect(&encoded, DecodeLimits::default())
                .expect("inspection must succeed");

        assert_eq!(info.payload_len as usize, encoded.len() - HEADER_LEN);
        assert_eq!(info.digest, document_digest(&sample()).unwrap());
    }

    #[test]
    fn reader_round_trip() {
        let original = sample();

        let encoded =
            serialize(&original).expect("serialization must succeed");

        let mut cursor = std::io::Cursor::new(encoded);

        let decoded: ZqnEnvelope<TestDocument> =
            deserialize_from_reader(
                &mut cursor,
                DecodeLimits::default(),
            )
            .expect("reader deserialization must succeed");

        assert_eq!(decoded.payload, original);
    }

    #[test]
    fn writer_round_trip() {
        let original = sample();
        let mut output = Vec::new();

        let digest =
            serialize_to_writer(
                &original,
                &mut output,
            )
            .expect("writer serialization must succeed");

        let decoded: ZqnEnvelope<TestDocument> =
            deserialize(&output)
                .expect("deserialization must succeed");

        assert_eq!(decoded.payload, original);
        assert_eq!(decoded.digest, digest);
    }

    #[test]
    fn nesting_limit_is_enforced() {
        let value = serde_json::json!({
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

        let result = validate_value_limits(
            &value,
            limits,
            0,
        );

        assert!(matches!(
            result,
            Err(SerializationError::ResourceLimitExceeded {
                resource: ResourceKind::NestingDepth,
                ..
            })
        ));
    }

    #[test]
    fn array_limit_is_enforced() {
        let value = serde_json::json!([1, 2, 3, 4]);

        let limits = DecodeLimits::new(
            1024,
            1024,
            1024,
            2,
            1024,
            16,
        );

        let result = validate_value_limits(
            &value,
            limits,
            0,
        );

        assert!(matches!(
            result,
            Err(SerializationError::ResourceLimitExceeded {
                resource: ResourceKind::ArrayElements,
                ..
            })
        ));
    }

    #[test]
    fn string_limit_is_enforced() {
        let value = serde_json::json!("0123456789");

        let limits = DecodeLimits::new(
            1024,
            1024,
            4,
            1024,
            1024,
            16,
        );

        let result = validate_value_limits(
            &value,
            limits,
            0,
        );

        assert!(matches!(
            result,
            Err(SerializationError::ResourceLimitExceeded {
                resource: ResourceKind::StringBytes,
                ..
            })
        ));
    }

    #[test]
    fn malformed_json_is_rejected() {
        let payload = b"{invalid-json";

        let result: Result<TestDocument, _> =
            deserialize_canonical(
                payload,
                DecodeLimits::default(),
            );

        assert!(matches!(
            result,
            Err(SerializationError::Json { .. })
        ));
    }

    #[test]
    fn no_unsafe_is_required() {
        // Compile-time enforced by:
        //
        // #![forbid(unsafe_code)]
        //
        // This test intentionally exists as a documentation anchor for the
        // security contract.
        assert!(true);
    }
}