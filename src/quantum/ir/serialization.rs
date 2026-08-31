//! Zamani Quantum IR — Canonical Serialization
//!
//! Production-grade, deterministic, bounded serialization infrastructure for
//! the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! This module owns the transport representation of already-defined Quantum IR
//! objects. It does not define the semantics of gates, qubits, measurements,
//! pulses, waveforms, scheduling, routing, hardware, or execution.
//!
//! The fundamental rule is:
//!
//! ```text
//! semantic IR
//!     │
//!     ▼
//! serialization.rs
//!     │
//!     ├── canonical binary representation
//!     ├── bounded decoding
//!     ├── version framing
//!     ├── integrity checking
//!     └── deterministic encoding
//! ```
//!
//! Serialization must never silently change the meaning of an IR program.
//!
//! # Universal-program principle
//!
//! Serialization does NOT impose a quantum-machine size limit.
//!
//! In particular, this module does not use:
//!
//! ```text
//! 63
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! as a qubit, operation, or machine-size boundary.
//!
//! Collection sizes are represented using `u64` on the wire and converted to
//! the host `usize` only after checked conversion and an explicit decoder
//! resource policy check.
//!
//! Therefore the same serialization contract can represent:
//!
//! - one qubit;
//! - thousands of qubits;
//! - millions of qubits;
//! - arbitrarily large finite programs representable by the selected resource
//!   policy and available storage.
//!
//! "Infinite qubits" is not represented literally. The architectural guarantee
//! is that no fixed qubit count is encoded into the IR schema.
//!
//! # Security boundary
//!
//! Serialized IR is untrusted input.
//!
//! A decoder MUST therefore:
//!
//! - validate the magic/version before interpreting payloads;
//! - reject unsupported versions;
//! - reject truncated input;
//! - reject trailing bytes in canonical documents;
//! - check every length before allocation;
//! - check every integer conversion;
//! - check every arithmetic operation;
//! - enforce explicit decoding budgets;
//! - reject malformed UTF-8;
//! - reject invalid boolean encodings;
//! - reject invalid enum discriminants;
//! - verify payload integrity;
//! - never allocate based solely on attacker-controlled unchecked lengths.
//!
//! This module uses no `unsafe` code.
//!
//! # Canonical representation
//!
//! The canonical document format is:
//!
//! ```text
//! ┌──────────────────────────────────────────────┐
//! │ magic                4 bytes                 │
//! ├──────────────────────────────────────────────┤
//! │ format version      u16 LE                   │
//! ├──────────────────────────────────────────────┤
//! │ IR major            u16 LE                   │
//! │ IR minor            u16 LE                   │
//! │ IR patch            u16 LE                   │
//! ├──────────────────────────────────────────────┤
//! │ payload length      u64 LE                   │
//! ├──────────────────────────────────────────────┤
//! │ payload checksum    u32 LE                   │
//! ├──────────────────────────────────────────────┤
//! │ canonical payload   variable length           │
//! └──────────────────────────────────────────────┘
//! ```
//!
//! All multi-byte integers use little-endian encoding.
//!
//! The payload itself is produced by [`IrEncode`] and consumed by
//! [`IrDecode`].
//!
//! # Important distinction
//!
//! The `format version` identifies this serialization framing protocol.
//!
//! `IrVersion` identifies the semantic Quantum IR contract.
//!
//! They are intentionally separate.
//!
//! ```text
//! serialization format version
//!             ≠
//!          IR version
//!             ≠
//!       compiler version
//!             ≠
//!       Zamani language version
//!             ≠
//!       hardware version
//! ```
//!
//! # Determinism
//!
//! Canonical serialization must be deterministic:
//!
//! ```text
//! same semantic IR
//!       +
//! same IR version
//!       +
//! same serialization format
//!       ───────────────────►
//!       identical bytes
//! ```
//!
//! This makes the format suitable for:
//!
//! - caching;
//! - content addressing;
//! - provenance;
//! - reproducible compilation;
//! - distributed compilation;
//! - benchmark reproducibility;
//! - job identity;
//! - artifact storage;
//! - cross-process transport.
//!
//! Collections whose semantic ordering is significant must be encoded in their
//! semantic order. Maps should be represented by callers using deterministic
//! ordering such as `BTreeMap` or an explicitly sorted sequence.
//!
//! This module does not silently sort arbitrary sequences because sequence
//! ordering may itself be semantic.
//!
//! # Quantum identity boundary
//!
//! The canonical logical and physical qubit identities remain owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module does not define duplicate qubit types.
//!
//! # Architectural boundary
//!
//! This module deliberately does NOT depend on:
//!
//! - hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - simulation;
//! - QEC algorithms;
//! - frontend parsing;
//! - backend execution.
//!
//! Higher-level IR modules implement [`IrEncode`] and [`IrDecode`] when their
//! semantic contracts are frozen.
//!
//! # Integration contract
//!
//! `identity.rs` supplies [`IrVersion`].
//!
//! `qubit.rs` supplies [`QubitId`] and [`PhysicalQubitId`]. This file provides
//! checked codecs for both canonical identities.
//!
//! `program.rs`, `region.rs`, `operation.rs`, `gate.rs`, `measurement.rs`,
//! `pulse.rs`, `waveform.rs`, `channel.rs`, `frame.rs`, `resource.rs`,
//! `capability.rs`, `mapping.rs`, `attribute.rs`, `extension.rs`, and
//! `provenance.rs` can implement [`IrEncode`] and [`IrDecode`] without this
//! module needing to know their internal representations.
//!
//! `hash.rs` can hash the exact bytes returned by [`serialize`].
//!
//! `validation.rs` remains responsible for semantic validation. Successful
//! decoding means only that the serialized representation is structurally
//! valid according to this serialization contract; callers should still run
//! canonical IR validation after reconstructing a semantic object.
//!
//! `mod.rs` should eventually expose this module through:
//!
//! ```text
//! pub mod serialization;
//! ```
//!
//! and selectively re-export its stable public API.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - no nightly features;
//! - no `unsafe`;
//! - no external dependency required by this module.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::convert::TryFrom;
use std::fmt;

use super::identity::IrVersion;
use super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Wire-format constants
// =============================================================================

/// Four-byte magic identifying a canonical Zamani Quantum IR document.
pub const MAGIC: [u8; 4] = *b"ZQIR";

/// Current serialization framing format version.
///
/// This is deliberately independent from [`IrVersion`].
pub const FORMAT_VERSION: u16 = 1;

/// Number of bytes in the fixed document header.
pub const HEADER_LEN: usize = 4 + 2 + 2 + 2 + 2 + 8 + 4;

/// Size of the checksum field in bytes.
const CHECKSUM_LEN: usize = 4;

/// Maximum representable serialized payload length.
///
/// This is a representation boundary, not a default resource policy.
const MAX_U64_USIZE_CONVERSION: u64 = usize::MAX as u64;

// =============================================================================
// Decoder policy
// =============================================================================

/// Explicit resource policy applied while decoding serialized IR.
///
/// This policy protects the decoder from malformed or hostile documents
/// without making the IR architecture itself finite.
///
/// A policy can be deliberately configured for a small embedded system or a
/// large compiler server.
///
/// The values here are policy values, not semantic quantum-machine limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeLimits {
    /// Maximum total serialized document size accepted.
    pub max_document_bytes: u64,

    /// Maximum payload size accepted.
    pub max_payload_bytes: u64,

    /// Maximum byte length of a single byte/string field.
    pub max_field_bytes: u64,

    /// Maximum number of elements in a single collection.
    pub max_collection_elements: u64,

    /// Maximum nesting depth permitted by a recursive codec.
    pub max_nesting_depth: u64,
}

impl DecodeLimits {
    /// Creates an explicit decoding policy.
    pub const fn new(
        max_document_bytes: u64,
        max_payload_bytes: u64,
        max_field_bytes: u64,
        max_collection_elements: u64,
        max_nesting_depth: u64,
    ) -> Self {
        Self {
            max_document_bytes,
            max_payload_bytes,
            max_field_bytes,
            max_collection_elements,
            max_nesting_depth,
        }
    }

    /// Returns a conservative policy suitable for ordinary compiler use.
    ///
    /// These values are safety defaults only. They do not define the maximum
    /// quantum-machine size supported by Zamani.
    pub const fn conservative() -> Self {
        Self {
            max_document_bytes: 256 * 1024 * 1024,
            max_payload_bytes: 256 * 1024 * 1024,
            max_field_bytes: 16 * 1024 * 1024,
            max_collection_elements: 16 * 1024 * 1024,
            max_nesting_depth: 4096,
        }
    }

    /// Returns a policy derived from the available host address space.
    ///
    /// The implementation remains conservative and never treats available
    /// address space as an obligation to allocate that amount.
    pub const fn platform_default() -> Self {
        Self::conservative()
    }

    /// Validates the policy itself.
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

        if self.max_field_bytes == 0 {
            return Err(SerializationError::InvalidDecodeLimits {
                field: "max_field_bytes",
            });
        }

        if self.max_collection_elements == 0 {
            return Err(SerializationError::InvalidDecodeLimits {
                field: "max_collection_elements",
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

        Ok(())
    }
}

impl Default for DecodeLimits {
    fn default() -> Self {
        Self::conservative()
    }
}

// =============================================================================
// Serialization errors
// =============================================================================

/// Errors produced by canonical IR serialization and deserialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    /// The supplied decode limits are internally inconsistent.
    InvalidDecodeLimits {
        /// Invalid policy field.
        field: &'static str,
    },

    /// The document is too large for the selected policy.
    DocumentTooLarge {
        /// Actual or declared document size.
        size: u64,

        /// Maximum accepted size.
        maximum: u64,
    },

    /// The payload is too large for the selected policy.
    PayloadTooLarge {
        /// Actual or declared payload size.
        size: u64,

        /// Maximum accepted size.
        maximum: u64,
    },

    /// The serialized document is shorter than required.
    UnexpectedEnd {
        /// Number of bytes required.
        needed: usize,

        /// Number of bytes available.
        available: usize,
    },

    /// The document magic is invalid.
    InvalidMagic {
        /// Bytes encountered at the document start.
        found: [u8; 4],
    },

    /// The serialization framing version is unsupported.
    UnsupportedFormatVersion {
        /// Version encountered in the document.
        version: u16,
    },

    /// The semantic IR version is unsupported.
    UnsupportedIrVersion {
        /// Version encountered in the document.
        version: IrVersion,
    },

    /// The document contains bytes after the canonical payload.
    TrailingBytes {
        /// Number of trailing bytes.
        count: usize,
    },

    /// The payload checksum does not match.
    ChecksumMismatch {
        /// Expected checksum stored in the document.
        expected: u32,

        /// Computed checksum.
        actual: u32,
    },

    /// A length cannot be represented by the host.
    LengthOverflow {
        /// Context of the conversion.
        context: &'static str,

        /// Wire representation.
        value: u64,
    },

    /// An arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Context of the calculation.
        context: &'static str,
    },

    /// A declared collection exceeds the decoder policy.
    CollectionLimitExceeded {
        /// Collection context.
        context: &'static str,

        /// Requested number of elements.
        requested: u64,

        /// Maximum permitted elements.
        maximum: u64,
    },

    /// A field exceeds the decoder policy.
    FieldLimitExceeded {
        /// Field context.
        context: &'static str,

        /// Requested byte count.
        requested: u64,

        /// Maximum permitted bytes.
        maximum: u64,
    },

    /// A nesting level exceeds the decoder policy.
    NestingLimitExceeded {
        /// Requested depth.
        requested: u64,

        /// Maximum permitted depth.
        maximum: u64,
    },

    /// A boolean used a non-canonical byte representation.
    InvalidBoolean {
        /// Invalid byte.
        value: u8,
    },

    /// An enum/discriminant is unknown.
    InvalidDiscriminant {
        /// Type being decoded.
        type_name: &'static str,

        /// Unknown discriminant.
        value: u64,
    },

    /// UTF-8 decoding failed.
    InvalidUtf8,

    /// The caller supplied an invalid IR object for serialization.
    InvalidObject {
        /// Human-readable reason.
        message: &'static str,
    },

    /// A custom IR codec rejected its own payload.
    Codec {
        /// Codec-specific message.
        message: String,
    },
}

impl fmt::Display for SerializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDecodeLimits { field } => {
                write!(
                    formatter,
                    "invalid serialization decode limit `{field}`"
                )
            }

            Self::DocumentTooLarge {
                size,
                maximum,
            } => {
                write!(
                    formatter,
                    "serialized document is too large: {size} bytes, maximum {maximum}"
                )
            }

            Self::PayloadTooLarge {
                size,
                maximum,
            } => {
                write!(
                    formatter,
                    "serialized payload is too large: {size} bytes, maximum {maximum}"
                )
            }

            Self::UnexpectedEnd {
                needed,
                available,
            } => {
                write!(
                    formatter,
                    "unexpected end of serialized IR: needed {needed} bytes, available {available}"
                )
            }

            Self::InvalidMagic { found } => {
                write!(
                    formatter,
                    "invalid Zamani Quantum IR magic: {:02x?}",
                    found
                )
            }

            Self::UnsupportedFormatVersion { version } => {
                write!(
                    formatter,
                    "unsupported Quantum IR serialization format version {version}"
                )
            }

            Self::UnsupportedIrVersion { version } => {
                write!(
                    formatter,
                    "unsupported Quantum IR version {version}"
                )
            }

            Self::TrailingBytes { count } => {
                write!(
                    formatter,
                    "canonical IR document contains {count} trailing bytes"
                )
            }

            Self::ChecksumMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "serialized IR checksum mismatch: expected {expected:#010x}, got {actual:#010x}"
                )
            }

            Self::LengthOverflow {
                context,
                value,
            } => {
                write!(
                    formatter,
                    "serialized {context} length {value} cannot be represented by this host"
                )
            }

            Self::ArithmeticOverflow { context } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {context}"
                )
            }

            Self::CollectionLimitExceeded {
                context,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "serialized {context} contains {requested} elements, maximum {maximum}"
                )
            }

            Self::FieldLimitExceeded {
                context,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "serialized {context} contains {requested} bytes, maximum {maximum}"
                )
            }

            Self::NestingLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "serialized IR nesting depth {requested} exceeds maximum {maximum}"
                )
            }

            Self::InvalidBoolean { value } => {
                write!(
                    formatter,
                    "invalid serialized boolean value {value}"
                )
            }

            Self::InvalidDiscriminant {
                type_name,
                value,
            } => {
                write!(
                    formatter,
                    "invalid {type_name} discriminant {value}"
                )
            }

            Self::InvalidUtf8 => {
                formatter.write_str("serialized string is not valid UTF-8")
            }

            Self::InvalidObject { message } => {
                write!(
                    formatter,
                    "invalid IR object for serialization: {message}"
                )
            }

            Self::Codec { message } => {
                write!(
                    formatter,
                    "IR serialization codec error: {message}"
                )
            }
        }
    }
}

impl std::error::Error for SerializationError {}

// =============================================================================
// Codec traits
// =============================================================================

/// Canonical encoder implemented by serializable IR objects.
///
/// Implementations MUST:
///
/// - emit deterministic bytes;
/// - use the primitive methods supplied by [`Encoder`];
/// - never emit host-dependent representations;
/// - never use pointer addresses;
/// - never serialize `usize` directly;
/// - preserve semantic sequence ordering;
/// - reject invalid semantic state.
///
/// Implementations SHOULD perform local semantic validation before encoding.
pub trait IrEncode {
    /// Encodes this IR object into the canonical payload representation.
    fn encode(&self, encoder: &mut Encoder) -> Result<(), SerializationError>;
}

/// Canonical decoder implemented by deserializable IR objects.
///
/// Implementations MUST:
///
/// - validate every discriminant;
/// - validate every length through [`Decoder`];
/// - never allocate from an unchecked length;
/// - reject malformed state;
/// - preserve canonical semantics.
///
/// Implementations SHOULD call canonical IR validation after reconstruction.
pub trait IrDecode: Sized {
    /// Decodes one object from the canonical payload.
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, SerializationError>;
}

// =============================================================================
// Canonical document
// =============================================================================

/// Canonically framed serialized Quantum IR document.
///
/// This wrapper owns the exact bytes exchanged between persistence,
/// compilation, caching, and transport layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializedIr {
    bytes: Vec<u8>,
}

impl SerializedIr {
    /// Creates a serialized document from already-validated canonical bytes.
    ///
    /// This constructor performs complete framing validation and therefore
    /// should be used for externally supplied byte sequences.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, SerializationError> {
        validate_document_structure(&bytes)?;

        Ok(Self { bytes })
    }

    /// Creates a serialized document from canonical bytes using explicit
    /// decoder limits.
    pub fn from_bytes_with_limits(
        bytes: Vec<u8>,
        limits: DecodeLimits,
    ) -> Result<Self, SerializationError> {
        limits.validate()?;

        check_document_limit(bytes.len(), limits.max_document_bytes)?;

        validate_document_structure(&bytes)?;

        let payload_length = read_payload_length(&bytes)?;

        if payload_length > limits.max_payload_bytes {
            return Err(SerializationError::PayloadTooLarge {
                size: payload_length,
                maximum: limits.max_payload_bytes,
            });
        }

        Ok(Self { bytes })
    }

    /// Returns the complete serialized document.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consumes the document and returns its bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Returns the total serialized document length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the serialized document is empty.
    ///
    /// A valid canonical document is never empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns the serialization format version.
    pub fn format_version(&self) -> Result<u16, SerializationError> {
        read_format_version(&self.bytes)
    }

    /// Returns the semantic IR version stored in the document.
    pub fn ir_version(&self) -> Result<IrVersion, SerializationError> {
        read_ir_version(&self.bytes)
    }

    /// Returns the payload length.
    pub fn payload_len(&self) -> Result<u64, SerializationError> {
        read_payload_length(&self.bytes)
    }

    /// Returns the payload without copying it.
    pub fn payload(&self) -> Result<&[u8], SerializationError> {
        payload_slice(&self.bytes)
    }
}

// =============================================================================
// Public encode/decode entry points
// =============================================================================

/// Serializes a canonical IR object into the Zamani Quantum IR wire format.
///
/// The resulting bytes are deterministic for a deterministic IR object.
pub fn serialize<T>(
    value: &T,
) -> Result<SerializedIr, SerializationError>
where
    T: IrEncode,
{
    serialize_with_version(value, IrVersion::CURRENT)
}

/// Serializes an IR object while explicitly specifying its semantic IR
/// version.
pub fn serialize_with_version<T>(
    value: &T,
    ir_version: IrVersion,
) -> Result<SerializedIr, SerializationError>
where
    T: IrEncode,
{
    if !ir_version.is_supported_by_current() {
        return Err(
            SerializationError::UnsupportedIrVersion {
                version: ir_version,
            },
        );
    }

    let mut payload = Encoder::new();

    value.encode(&mut payload)?;

    build_document(ir_version, payload.into_bytes())
}

/// Deserializes a canonical Quantum IR document.
///
/// Structural validation is performed before the object codec is invoked.
pub fn deserialize<T>(
    document: &[u8],
) -> Result<T, SerializationError>
where
    T: IrDecode,
{
    deserialize_with_limits(document, DecodeLimits::default())
}

/// Deserializes a canonical Quantum IR document using explicit resource
/// limits.
pub fn deserialize_with_limits<T>(
    document: &[u8],
    limits: DecodeLimits,
) -> Result<T, SerializationError>
where
    T: IrDecode,
{
    limits.validate()?;

    check_document_limit(
        document.len(),
        limits.max_document_bytes,
    )?;

    validate_document_structure(document)?;

    let ir_version = read_ir_version(document)?;

    if !ir_version.is_supported_by_current() {
        return Err(
            SerializationError::UnsupportedIrVersion {
                version: ir_version,
            },
        );
    }

    let payload = payload_slice(document)?;

    if payload.len() as u64 > limits.max_payload_bytes {
        return Err(
            SerializationError::PayloadTooLarge {
                size: payload.len() as u64,
                maximum: limits.max_payload_bytes,
            },
        );
    }

    let mut decoder = Decoder::with_limits(
        payload,
        limits,
    );

    let value = T::decode(&mut decoder)?;

    decoder.finish()?;

    Ok(value)
}

// =============================================================================
// Canonical encoder
// =============================================================================

/// Low-level canonical payload encoder.
///
/// Higher-level IR implementations should use this type rather than directly
/// manipulating byte vectors.
#[derive(Debug, Default)]
pub struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    /// Creates an empty canonical encoder.
    #[must_use]
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Creates an encoder with a requested capacity.
    ///
    /// The capacity is only a local allocation hint. It does not alter the
    /// wire format.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    /// Returns the current encoded length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether no payload bytes have been emitted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns the encoded bytes without consuming the encoder.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consumes the encoder.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Writes a single byte.
    pub fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    /// Writes a canonical boolean.
    pub fn write_bool(&mut self, value: bool) {
        self.write_u8(if value { 1 } else { 0 });
    }

    /// Writes a little-endian `u16`.
    pub fn write_u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a little-endian `u32`.
    pub fn write_u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a little-endian `u64`.
    pub fn write_u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a little-endian `i64`.
    pub fn write_i64(&mut self, value: i64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes the IEEE-754 binary representation of an `f64`.
    ///
    /// Higher-level semantic types should canonicalize NaN policy before using
    /// this method if NaN values are permitted by their semantic contract.
    pub fn write_f64(&mut self, value: f64) {
        self.write_u64(value.to_bits());
    }

    /// Writes a length-prefixed byte sequence.
    pub fn write_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), SerializationError> {
        let length = u64::try_from(bytes.len()).map_err(|_| {
            SerializationError::LengthOverflow {
                context: "byte field",
                value: bytes.len() as u64,
            }
        })?;

        self.write_u64(length);
        self.bytes.extend_from_slice(bytes);

        Ok(())
    }

    /// Writes a UTF-8 string.
    pub fn write_string(
        &mut self,
        value: &str,
    ) -> Result<(), SerializationError> {
        self.write_bytes(value.as_bytes())
    }

    /// Writes a canonical `usize` as `u64`.
    ///
    /// `usize` is never written directly because its width varies by host.
    pub fn write_usize(&mut self, value: usize) {
        self.write_u64(value as u64);
    }

    /// Writes a logical qubit using its canonical IR identity.
    ///
    /// The canonical type remains `quantum::ir::qubit::QubitId`.
    pub fn write_qubit_id(&mut self, qubit: QubitId) {
        self.write_usize(qubit.index());
    }

    /// Writes a physical qubit using its canonical IR identity.
    ///
    /// The canonical type remains
    /// `quantum::ir::qubit::PhysicalQubitId`.
    pub fn write_physical_qubit_id(
        &mut self,
        qubit: PhysicalQubitId,
    ) {
        self.write_usize(qubit.index());
    }

    /// Writes an IR version.
    pub fn write_ir_version(&mut self, version: IrVersion) {
        self.write_u16(version.major());
        self.write_u16(version.minor());
        self.write_u16(version.patch());
    }

    /// Writes a discriminant.
    pub fn write_discriminant(&mut self, value: u64) {
        self.write_u64(value);
    }

    /// Writes a length-prefixed sequence using a caller-provided element
    /// encoder.
    pub fn write_sequence<T, F>(
        &mut self,
        values: &[T],
        mut encode: F,
    ) -> Result<(), SerializationError>
    where
        F: FnMut(&mut Self, &T) -> Result<(), SerializationError>,
    {
        let length = u64::try_from(values.len()).map_err(|_| {
            SerializationError::LengthOverflow {
                context: "sequence",
                value: values.len() as u64,
            }
        })?;

        self.write_u64(length);

        for value in values {
            encode(self, value)?;
        }

        Ok(())
    }

    /// Writes a deterministic map-like sequence.
    ///
    /// The caller is responsible for providing entries in semantic canonical
    /// order. This function intentionally does not sort entries because
    /// sorting may alter semantic sequence information.
    pub fn write_entries<K, V, F>(
        &mut self,
        entries: &[(K, V)],
        mut encode: F,
    ) -> Result<(), SerializationError>
    where
        F: FnMut(
            &mut Self,
            &K,
            &V,
        ) -> Result<(), SerializationError>,
    {
        let length = u64::try_from(entries.len()).map_err(|_| {
            SerializationError::LengthOverflow {
                context: "entry collection",
                value: entries.len() as u64,
            }
        })?;

        self.write_u64(length);

        for (key, value) in entries {
            encode(self, key, value)?;
        }

        Ok(())
    }
}

// =============================================================================
// Canonical decoder
// =============================================================================

/// Bounded canonical payload decoder.
///
/// Every allocation-producing operation checks the wire length against the
/// explicit decoder policy before allocating.
#[derive(Debug)]
pub struct Decoder<'a> {
    bytes: &'a [u8],
    position: usize,
    limits: DecodeLimits,
    nesting_depth: u64,
}

impl<'a> Decoder<'a> {
    /// Creates a decoder with the default resource policy.
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            position: 0,
            limits: DecodeLimits::default(),
            nesting_depth: 0,
        }
    }

    /// Creates a decoder with explicit resource limits.
    pub fn with_limits(
        bytes: &'a [u8],
        limits: DecodeLimits,
    ) -> Self {
        Self {
            bytes,
            position: 0,
            limits,
            nesting_depth: 0,
        }
    }

    /// Returns the current read position.
    #[must_use]
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns the remaining unread bytes.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    /// Returns whether all payload bytes have been consumed.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    /// Finishes decoding and rejects trailing payload bytes.
    pub fn finish(&self) -> Result<(), SerializationError> {
        if self.is_finished() {
            Ok(())
        } else {
            Err(SerializationError::TrailingBytes {
                count: self.remaining(),
            })
        }
    }

    /// Enters one nested decoding scope.
    pub fn enter_scope(
        &mut self,
    ) -> Result<NestingGuard<'_>, SerializationError> {
        let next_depth = self
            .nesting_depth
            .checked_add(1)
            .ok_or(SerializationError::ArithmeticOverflow {
                context: "serialization nesting depth",
            })?;

        if next_depth > self.limits.max_nesting_depth {
            return Err(
                SerializationError::NestingLimitExceeded {
                    requested: next_depth,
                    maximum: self.limits.max_nesting_depth,
                },
            );
        }

        self.nesting_depth = next_depth;

        Ok(NestingGuard { decoder: self })
    }

    /// Reads one byte.
    pub fn read_u8(&mut self) -> Result<u8, SerializationError> {
        let bytes = self.take_exact(1)?;
        Ok(bytes[0])
    }

    /// Reads a canonical boolean.
    pub fn read_bool(&mut self) -> Result<bool, SerializationError> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(SerializationError::InvalidBoolean { value }),
        }
    }

    /// Reads a little-endian `u16`.
    pub fn read_u16(&mut self) -> Result<u16, SerializationError> {
        let bytes = self.take_exact(2)?;

        Ok(u16::from_le_bytes([
            bytes[0],
            bytes[1],
        ]))
    }

    /// Reads a little-endian `u32`.
    pub fn read_u32(&mut self) -> Result<u32, SerializationError> {
        let bytes = self.take_exact(4)?;

        Ok(u32::from_le_bytes([
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
        ]))
    }

    /// Reads a little-endian `u64`.
    pub fn read_u64(&mut self) -> Result<u64, SerializationError> {
        let bytes = self.take_exact(8)?;

        Ok(u64::from_le_bytes([
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
        ]))
    }

    /// Reads a little-endian `i64`.
    pub fn read_i64(&mut self) -> Result<i64, SerializationError> {
        let bytes = self.take_exact(8)?;

        Ok(i64::from_le_bytes([
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
        ]))
    }

    /// Reads an IEEE-754 binary `f64`.
    pub fn read_f64(&mut self) -> Result<f64, SerializationError> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    /// Reads a length-prefixed byte sequence.
    ///
    /// The allocation occurs only after all limits and integer conversions have
    /// been checked.
    pub fn read_bytes(
        &mut self,
        context: &'static str,
    ) -> Result<Vec<u8>, SerializationError> {
        let length = self.read_u64()?;

        self.check_field_length(length, context)?;

        let length = usize_from_u64(
            length,
            context,
        )?;

        let bytes = self.take_exact(length)?;

        Ok(bytes.to_vec())
    }

    /// Reads a UTF-8 string.
    pub fn read_string(
        &mut self,
        context: &'static str,
    ) -> Result<String, SerializationError> {
        let bytes = self.read_bytes(context)?;

        String::from_utf8(bytes)
            .map_err(|_| SerializationError::InvalidUtf8)
    }

    /// Reads a canonical `usize` represented as `u64`.
    pub fn read_usize(
        &mut self,
        context: &'static str,
    ) -> Result<usize, SerializationError> {
        let value = self.read_u64()?;

        usize_from_u64(value, context)
    }

    /// Reads a canonical logical qubit identity.
    ///
    /// The returned type is the canonical
    /// `quantum::ir::qubit::QubitId`.
    pub fn read_qubit_id(
        &mut self,
    ) -> Result<QubitId, SerializationError> {
        let index = self.read_usize("logical qubit identity")?;

        Ok(QubitId::new(index))
    }

    /// Reads a canonical physical qubit identity.
    ///
    /// The returned type is the canonical
    /// `quantum::ir::qubit::PhysicalQubitId`.
    pub fn read_physical_qubit_id(
        &mut self,
    ) -> Result<PhysicalQubitId, SerializationError> {
        let index = self.read_usize("physical qubit identity")?;

        Ok(PhysicalQubitId::new(index))
    }

    /// Reads an IR version.
    pub fn read_ir_version(
        &mut self,
    ) -> Result<IrVersion, SerializationError> {
        Ok(IrVersion::new(
            self.read_u16()?,
            self.read_u16()?,
            self.read_u16()?,
        ))
    }

    /// Reads a discriminant.
    pub fn read_discriminant(
        &mut self,
    ) -> Result<u64, SerializationError> {
        self.read_u64()
    }

    /// Reads a bounded sequence.
    pub fn read_sequence<T, F>(
        &mut self,
        context: &'static str,
        mut decode: F,
    ) -> Result<Vec<T>, SerializationError>
    where
        F: FnMut(
            &mut Self,
        ) -> Result<T, SerializationError>,
    {
        let length = self.read_u64()?;

        self.check_collection_length(
            length,
            context,
        )?;

        let length = usize_from_u64(
            length,
            context,
        )?;

        let mut values = Vec::new();

        values.try_reserve(length).map_err(|_| {
            SerializationError::Codec {
                message: format!(
                    "unable to reserve {length} elements for {context}"
                ),
            }
        })?;

        for _ in 0..length {
            values.push(decode(self)?);
        }

        Ok(values)
    }

    /// Reads a sequence of `IrDecode` objects.
    pub fn read_ir_sequence<T>(
        &mut self,
        context: &'static str,
    ) -> Result<Vec<T>, SerializationError>
    where
        T: IrDecode,
    {
        self.read_sequence(context, |decoder| {
            T::decode(decoder)
        })
    }

    /// Checks a collection length against policy.
    pub fn check_collection_length(
        &self,
        length: u64,
        context: &'static str,
    ) -> Result<(), SerializationError> {
        if length > self.limits.max_collection_elements {
            return Err(
                SerializationError::CollectionLimitExceeded {
                    context,
                    requested: length,
                    maximum: self.limits.max_collection_elements,
                },
            );
        }

        Ok(())
    }

    /// Checks a field length against policy.
    pub fn check_field_length(
        &self,
        length: u64,
        context: &'static str,
    ) -> Result<(), SerializationError> {
        if length > self.limits.max_field_bytes {
            return Err(
                SerializationError::FieldLimitExceeded {
                    context,
                    requested: length,
                    maximum: self.limits.max_field_bytes,
                },
            );
        }

        Ok(())
    }

    fn take_exact(
        &mut self,
        length: usize,
    ) -> Result<&'a [u8], SerializationError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(
                SerializationError::ArithmeticOverflow {
                    context: "decoder position",
                },
            )?;

        if end > self.bytes.len() {
            return Err(
                SerializationError::UnexpectedEnd {
                    needed: length,
                    available: self.remaining(),
                },
            );
        }

        let start = self.position;
        self.position = end;

        Ok(&self.bytes[start..end])
    }
}

// =============================================================================
// Nesting guard
// =============================================================================

/// RAII guard for bounded recursive decoding.
pub struct NestingGuard<'a> {
    decoder: &'a mut Decoder<'a>,
}

impl<'a> Drop for NestingGuard<'a> {
    fn drop(&mut self) {
        self.decoder.nesting_depth =
            self.decoder.nesting_depth.saturating_sub(1);
    }
}

// =============================================================================
// Document construction
// =============================================================================

fn build_document(
    ir_version: IrVersion,
    payload: Vec<u8>,
) -> Result<SerializedIr, SerializationError> {
    let payload_length = u64::try_from(payload.len())
        .map_err(|_| SerializationError::LengthOverflow {
            context: "IR payload",
            value: payload.len() as u64,
        })?;

    let header_plus_payload = HEADER_LEN
        .checked_add(payload.len())
        .ok_or(SerializationError::ArithmeticOverflow {
            context: "serialized document size",
        })?;

    let checksum = checksum32(&payload);

    let mut bytes = Vec::with_capacity(header_plus_payload);

    bytes.extend_from_slice(&MAGIC);
    bytes.extend_from_slice(
        &FORMAT_VERSION.to_le_bytes(),
    );

    bytes.extend_from_slice(
        &ir_version.major().to_le_bytes(),
    );

    bytes.extend_from_slice(
        &ir_version.minor().to_le_bytes(),
    );

    bytes.extend_from_slice(
        &ir_version.patch().to_le_bytes(),
    );

    bytes.extend_from_slice(
        &payload_length.to_le_bytes(),
    );

    bytes.extend_from_slice(
        &checksum.to_le_bytes(),
    );

    bytes.extend_from_slice(&payload);

    Ok(SerializedIr { bytes })
}

// =============================================================================
// Document validation
// =============================================================================

fn validate_document_structure(
    document: &[u8],
) -> Result<(), SerializationError> {
    if document.len() < HEADER_LEN {
        return Err(
            SerializationError::UnexpectedEnd {
                needed: HEADER_LEN,
                available: document.len(),
            },
        );
    }

    let magic = [
        document[0],
        document[1],
        document[2],
        document[3],
    ];

    if magic != MAGIC {
        return Err(
            SerializationError::InvalidMagic {
                found: magic,
            },
        );
    }

    let format_version =
        u16::from_le_bytes([
            document[4],
            document[5],
        ]);

    if format_version != FORMAT_VERSION {
        return Err(
            SerializationError::UnsupportedFormatVersion {
                version: format_version,
            },
        );
    }

    let payload_length =
        u64::from_le_bytes([
            document[12],
            document[13],
            document[14],
            document[15],
            document[16],
            document[17],
            document[18],
            document[19],
        ]);

    let payload_length_usize =
        usize_from_u64(
            payload_length,
            "IR payload",
        )?;

    let expected_length = HEADER_LEN
        .checked_add(payload_length_usize)
        .ok_or(SerializationError::ArithmeticOverflow {
            context: "serialized document length",
        })?;

    if document.len() < expected_length {
        return Err(
            SerializationError::UnexpectedEnd {
                needed: expected_length,
                available: document.len(),
            },
        );
    }

    if document.len() > expected_length {
        return Err(
            SerializationError::TrailingBytes {
                count: document.len() - expected_length,
            },
        );
    }

    let expected_checksum =
        u32::from_le_bytes([
            document[20],
            document[21],
            document[22],
            document[23],
        ]);

    let payload =
        &document[HEADER_LEN..expected_length];

    let actual_checksum = checksum32(payload);

    if expected_checksum != actual_checksum {
        return Err(
            SerializationError::ChecksumMismatch {
                expected: expected_checksum,
                actual: actual_checksum,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Document field accessors
// =============================================================================

fn read_format_version(
    document: &[u8],
) -> Result<u16, SerializationError> {
    if document.len() < HEADER_LEN {
        return Err(
            SerializationError::UnexpectedEnd {
                needed: HEADER_LEN,
                available: document.len(),
            },
        );
    }

    Ok(u16::from_le_bytes([
        document[4],
        document[5],
    ]))
}

fn read_ir_version(
    document: &[u8],
) -> Result<IrVersion, SerializationError> {
    if document.len() < HEADER_LEN {
        return Err(
            SerializationError::UnexpectedEnd {
                needed: HEADER_LEN,
                available: document.len(),
            },
        );
    }

    Ok(IrVersion::new(
        u16::from_le_bytes([
            document[6],
            document[7],
        ]),
        u16::from_le_bytes([
            document[8],
            document[9],
        ]),
        u16::from_le_bytes([
            document[10],
            document[11],
        ]),
    ))
}

fn read_payload_length(
    document: &[u8],
) -> Result<u64, SerializationError> {
    if document.len() < HEADER_LEN {
        return Err(
            SerializationError::UnexpectedEnd {
                needed: HEADER_LEN,
                available: document.len(),
            },
        );
    }

    Ok(u64::from_le_bytes([
        document[12],
        document[13],
        document[14],
        document[15],
        document[16],
        document[17],
        document[18],
        document[19],
    ]))
}

fn payload_slice<'a>(
    document: &'a [u8],
) -> Result<&'a [u8], SerializationError> {
    validate_document_structure(document)?;

    let payload_length =
        read_payload_length(document)?;

    let payload_length =
        usize_from_u64(
            payload_length,
            "IR payload",
        )?;

    let end = HEADER_LEN
        .checked_add(payload_length)
        .ok_or(SerializationError::ArithmeticOverflow {
            context: "IR payload end",
        })?;

    Ok(&document[HEADER_LEN..end])
}

// =============================================================================
// Length helpers
// =============================================================================

fn usize_from_u64(
    value: u64,
    context: &'static str,
) -> Result<usize, SerializationError> {
    if value > MAX_U64_USIZE_CONVERSION {
        return Err(
            SerializationError::LengthOverflow {
                context,
                value,
            },
        );
    }

    Ok(value as usize)
}

fn check_document_limit(
    size: usize,
    maximum: u64,
) -> Result<(), SerializationError> {
    let size = u64::try_from(size)
        .map_err(|_| SerializationError::LengthOverflow {
            context: "IR document",
            value: size as u64,
        })?;

    if size > maximum {
        return Err(
            SerializationError::DocumentTooLarge {
                size,
                maximum,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Checksum
// =============================================================================

/// Computes the deterministic checksum used by the framing layer.
///
/// This checksum is intended to detect accidental corruption and malformed
/// transport data. It is NOT a cryptographic hash and must not be used for
/// security authentication or content-addressed identity.
///
/// Cryptographic identity belongs in `hash.rs`.
fn checksum32(bytes: &[u8]) -> u32 {
    // FNV-1a-style 32-bit checksum.
    //
    // The algorithm is deliberately implemented locally so the canonical
    // serialization framing has no external dependency.
    let mut hash: u32 = 0x811c_9dc5;

    for byte in bytes {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }

    hash
}

// =============================================================================
// Primitive codec implementations
// =============================================================================

impl IrEncode for bool {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_bool(*self);
        Ok(())
    }
}

impl IrDecode for bool {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_bool()
    }
}

impl IrEncode for u8 {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_u8(*self);
        Ok(())
    }
}

impl IrDecode for u8 {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_u8()
    }
}

impl IrEncode for u16 {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_u16(*self);
        Ok(())
    }
}

impl IrDecode for u16 {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_u16()
    }
}

impl IrEncode for u32 {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_u32(*self);
        Ok(())
    }
}

impl IrDecode for u32 {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_u32()
    }
}

impl IrEncode for u64 {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_u64(*self);
        Ok(())
    }
}

impl IrDecode for u64 {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_u64()
    }
}

impl IrEncode for i64 {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_i64(*self);
        Ok(())
    }
}

impl IrDecode for i64 {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_i64()
    }
}

impl IrEncode for f64 {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_f64(*self);
        Ok(())
    }
}

impl IrDecode for f64 {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_f64()
    }
}

impl IrEncode for String {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_string(self)
    }
}

impl IrDecode for String {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_string("string")
    }
}

impl IrEncode for str {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_string(self)
    }
}

impl IrEncode for QubitId {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_qubit_id(*self);
        Ok(())
    }
}

impl IrDecode for QubitId {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_qubit_id()
    }
}

impl IrEncode for PhysicalQubitId {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_physical_qubit_id(*self);
        Ok(())
    }
}

impl IrDecode for PhysicalQubitId {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_physical_qubit_id()
    }
}

impl IrEncode for IrVersion {
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_ir_version(*self);
        Ok(())
    }
}

impl IrDecode for IrVersion {
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_ir_version()
    }
}

// =============================================================================
// Generic collection implementations
// =============================================================================

impl<T> IrEncode for Vec<T>
where
    T: IrEncode,
{
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        encoder.write_sequence(
            self,
            |encoder, value| value.encode(encoder),
        )
    }
}

impl<T> IrDecode for Vec<T>
where
    T: IrDecode,
{
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        decoder.read_ir_sequence("vector")
    }
}

// =============================================================================
// Option implementation
// =============================================================================

impl<T> IrEncode for Option<T>
where
    T: IrEncode,
{
    fn encode(
        &self,
        encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        match self {
            Some(value) => {
                encoder.write_u8(1);
                value.encode(encoder)?;
            }

            None => {
                encoder.write_u8(0);
            }
        }

        Ok(())
    }
}

impl<T> IrDecode for Option<T>
where
    T: IrDecode,
{
    fn decode(
        decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        match decoder.read_u8()? {
            0 => Ok(None),

            1 => Ok(Some(T::decode(decoder)?)),

            value => Err(
                SerializationError::InvalidDiscriminant {
                    type_name: "Option",
                    value: u64::from(value),
                },
            ),
        }
    }
}

// =============================================================================
// Unit implementation
// =============================================================================

impl IrEncode for () {
    fn encode(
        &self,
        _encoder: &mut Encoder,
    ) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl IrDecode for () {
    fn decode(
        _decoder: &mut Decoder<'_>,
    ) -> Result<Self, SerializationError> {
        Ok(())
    }
}

// =============================================================================
// Canonical helper functions
// =============================================================================

/// Encodes an arbitrary value into a payload without document framing.
///
/// This is useful for testing or composing nested codecs. Persistent IR
/// artifacts should normally use [`serialize`].
pub fn encode_payload<T>(
    value: &T,
) -> Result<Vec<u8>, SerializationError>
where
    T: IrEncode,
{
    let mut encoder = Encoder::new();

    value.encode(&mut encoder)?;

    Ok(encoder.into_bytes())
}

/// Decodes an arbitrary payload without document framing.
///
/// Persistent/external input should normally use [`deserialize_with_limits`]
/// instead because framing supplies version and integrity information.
pub fn decode_payload<T>(
    payload: &[u8],
) -> Result<T, SerializationError>
where
    T: IrDecode,
{
    decode_payload_with_limits(
        payload,
        DecodeLimits::default(),
    )
}

/// Decodes an arbitrary payload using explicit limits.
pub fn decode_payload_with_limits<T>(
    payload: &[u8],
    limits: DecodeLimits,
) -> Result<T, SerializationError>
where
    T: IrDecode,
{
    limits.validate()?;

    if payload.len() as u64 > limits.max_payload_bytes {
        return Err(
            SerializationError::PayloadTooLarge {
                size: payload.len() as u64,
                maximum: limits.max_payload_bytes,
            },
        );
    }

    let mut decoder =
        Decoder::with_limits(payload, limits);

    let value = T::decode(&mut decoder)?;

    decoder.finish()?;

    Ok(value)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestObject {
        version: IrVersion,
        logical_qubit: QubitId,
        physical_qubit: PhysicalQubitId,
        enabled: bool,
        count: u64,
        name: String,
        values: Vec<u64>,
        optional: Option<String>,
    }

    impl IrEncode for TestObject {
        fn encode(
            &self,
            encoder: &mut Encoder,
        ) -> Result<(), SerializationError> {
            self.version.encode(encoder)?;
            self.logical_qubit.encode(encoder)?;
            self.physical_qubit.encode(encoder)?;
            self.enabled.encode(encoder)?;
            self.count.encode(encoder)?;
            self.name.encode(encoder)?;
            self.values.encode(encoder)?;
            self.optional.encode(encoder)?;

            Ok(())
        }
    }

    impl IrDecode for TestObject {
        fn decode(
            decoder: &mut Decoder<'_>,
        ) -> Result<Self, SerializationError> {
            Ok(Self {
                version: IrVersion::decode(decoder)?,
                logical_qubit: QubitId::decode(decoder)?,
                physical_qubit:
                    PhysicalQubitId::decode(decoder)?,
                enabled: bool::decode(decoder)?,
                count: u64::decode(decoder)?,
                name: String::decode(decoder)?,
                values: Vec::<u64>::decode(decoder)?,
                optional:
                    Option::<String>::decode(decoder)?,
            })
        }
    }

    fn sample_object() -> TestObject {
        TestObject {
            version: IrVersion::CURRENT,
            logical_qubit: QubitId::new(7),
            physical_qubit:
                PhysicalQubitId::new(42),
            enabled: true,
            count: 123_456,
            name: "Zamani".to_owned(),
            values: vec![1, 2, 3, 5, 8, 13],
            optional: Some(
                "quantum".to_owned(),
            ),
        }
    }

    #[test]
    fn round_trip_preserves_object() {
        let original = sample_object();

        let document =
            serialize(&original)
                .expect("serialization must succeed");

        let decoded: TestObject =
            deserialize(document.as_bytes())
                .expect("deserialization must succeed");

        assert_eq!(decoded, original);
    }

    #[test]
    fn serialization_is_deterministic() {
        let object = sample_object();

        let first =
            serialize(&object)
                .expect("first serialization");

        let second =
            serialize(&object)
                .expect("second serialization");

        assert_eq!(
            first.as_bytes(),
            second.as_bytes()
        );
    }

    #[test]
    fn header_contains_canonical_magic() {
        let object = sample_object();

        let document =
            serialize(&object)
                .expect("serialization");

        assert_eq!(
            &document.as_bytes()[0..4],
            &MAGIC
        );
    }

    #[test]
    fn header_contains_format_version() {
        let object = sample_object();

        let document =
            serialize(&object)
                .expect("serialization");

        assert_eq!(
            document.format_version()
                .expect("format version"),
            FORMAT_VERSION
        );
    }

    #[test]
    fn header_contains_ir_version() {
        let object = sample_object();

        let document =
            serialize(&object)
                .expect("serialization");

        assert_eq!(
            document.ir_version()
                .expect("IR version"),
            IrVersion::CURRENT
        );
    }

    #[test]
    fn invalid_magic_is_rejected() {
        let object = sample_object();

        let mut bytes =
            serialize(&object)
                .expect("serialization")
                .into_bytes();

        bytes[0] = b'X';

        let result =
            SerializedIr::from_bytes(bytes);

        assert!(matches!(
            result,
            Err(SerializationError::InvalidMagic {
                ..
            })
        ));
    }

    #[test]
    fn unsupported_format_version_is_rejected() {
        let object = sample_object();

        let mut bytes =
            serialize(&object)
                .expect("serialization")
                .into_bytes();

        bytes[4] = 0xff;
        bytes[5] = 0xff;

        let result =
            SerializedIr::from_bytes(bytes);

        assert!(matches!(
            result,
            Err(
                SerializationError::UnsupportedFormatVersion {
                    ..
                }
            )
        ));
    }

    #[test]
    fn truncated_document_is_rejected() {
        let object = sample_object();

        let bytes =
            serialize(&object)
                .expect("serialization")
                .into_bytes();

        let truncated =
            bytes[..bytes.len() - 1].to_vec();

        let result =
            SerializedIr::from_bytes(truncated);

        assert!(matches!(
            result,
            Err(
                SerializationError::UnexpectedEnd {
                    ..
                }
            )
        ));
    }

    #[test]
    fn trailing_bytes_are_rejected() {
        let object = sample_object();

        let mut bytes =
            serialize(&object)
                .expect("serialization")
                .into_bytes();

        bytes.push(0);

        let result =
            SerializedIr::from_bytes(bytes);

        assert!(matches!(
            result,
            Err(
                SerializationError::TrailingBytes {
                    ..
                }
            )
        ));
    }

    #[test]
    fn checksum_corruption_is_rejected() {
        let object = sample_object();

        let mut bytes =
            serialize(&object)
                .expect("serialization")
                .into_bytes();

        let last =
            bytes.len() - 1;

        bytes[last] ^= 0xff;

        let result =
            SerializedIr::from_bytes(bytes);

        assert!(matches!(
            result,
            Err(
                SerializationError::ChecksumMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn invalid_boolean_is_rejected() {
        let payload = [2u8];

        let mut decoder =
            Decoder::new(&payload);

        let result =
            decoder.read_bool();

        assert!(matches!(
            result,
            Err(
                SerializationError::InvalidBoolean {
                    value: 2
                }
            )
        ));
    }

    #[test]
    fn invalid_utf8_is_rejected() {
        let payload = [
            3u8, 0, 0, 0, 0, 0, 0, 0,
            0xff, 0xff, 0xff,
        ];

        let mut decoder =
            Decoder::new(&payload);

        let result =
            decoder.read_string("test");

        assert!(matches!(
            result,
            Err(SerializationError::InvalidUtf8)
        ));
    }

    #[test]
    fn field_limits_are_enforced_before_allocation() {
        let payload = [
            100u8, 0, 0, 0, 0, 0, 0, 0,
        ];

        let limits = DecodeLimits::new(
            1024,
            1024,
            8,
            100,
            16,
        );

        let mut decoder =
            Decoder::with_limits(
                &payload,
                limits,
            );

        let result =
            decoder.read_bytes("test field");

        assert!(matches!(
            result,
            Err(
                SerializationError::FieldLimitExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn collection_limits_are_enforced_before_allocation() {
        let payload = [
            100u8, 0, 0, 0, 0, 0, 0, 0,
        ];

        let limits = DecodeLimits::new(
            1024,
            1024,
            1024,
            8,
            16,
        );

        let mut decoder =
            Decoder::with_limits(
                &payload,
                limits,
            );

        let result =
            decoder.read_sequence(
                "test collection",
                |_decoder| {
                    Ok::<u64, SerializationError>(0)
                },
            );

        assert!(matches!(
            result,
            Err(
                SerializationError::CollectionLimitExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn logical_qubit_round_trip_uses_canonical_type() {
        let original =
            QubitId::new(123_456);

        let payload =
            encode_payload(&original)
                .expect("encode");

        let decoded: QubitId =
            decode_payload(&payload)
                .expect("decode");

        assert_eq!(
            decoded,
            original
        );
    }

    #[test]
    fn physical_qubit_round_trip_uses_canonical_type() {
        let original =
            PhysicalQubitId::new(654_321);

        let payload =
            encode_payload(&original)
                .expect("encode");

        let decoded:
            PhysicalQubitId =
            decode_payload(&payload)
                .expect("decode");

        assert_eq!(
            decoded,
            original
        );
    }

    #[test]
    fn large_identity_does_not_create_a_small_machine_boundary() {
        let original =
            QubitId::new(usize::MAX);

        let payload =
            encode_payload(&original)
                .expect("encode");

        let decoded: QubitId =
            decode_payload(&payload)
                .expect("decode");

        assert_eq!(
            decoded,
            original
        );
    }

    #[test]
    fn zero_length_string_is_supported() {
        let original = String::new();

        let payload =
            encode_payload(&original)
                .expect("encode");

        let decoded: String =
            decode_payload(&payload)
                .expect("decode");

        assert_eq!(
            decoded,
            original
        );
    }

    #[test]
    fn empty_vector_is_supported() {
        let original:
            Vec<u64> =
            Vec::new();

        let payload =
            encode_payload(&original)
                .expect("encode");

        let decoded:
            Vec<u64> =
            decode_payload(&payload)
                .expect("decode");

        assert_eq!(
            decoded,
            original
        );
    }

    #[test]
    fn none_option_is_supported() {
        let original:
            Option<u64> =
            None;

        let payload =
            encode_payload(&original)
                .expect("encode");

        let decoded:
            Option<u64> =
            decode_payload(&payload)
                .expect("decode");

        assert_eq!(
            decoded,
            original
        );
    }

    #[test]
    fn some_option_is_supported() {
        let original =
            Some(42u64);

        let payload =
            encode_payload(&original)
                .expect("encode");

        let decoded:
            Option<u64> =
            decode_payload(&payload)
                .expect("decode");

        assert_eq!(
            decoded,
            original
        );
    }

    #[test]
    fn document_payload_length_is_correct() {
        let object = sample_object();

        let document =
            serialize(&object)
                .expect("serialization");

        let payload =
            document
                .payload()
                .expect("payload");

        assert_eq!(
            document
                .payload_len()
                .expect("payload length"),
            payload.len() as u64
        );
    }

    #[test]
    fn conservative_limits_are_self_consistent() {
        DecodeLimits::conservative()
            .validate()
            .expect("default limits must be valid");
    }

    #[test]
    fn invalid_limits_are_rejected() {
        let limits = DecodeLimits::new(
            10,
            20,
            1,
            1,
            1,
        );

        assert!(matches!(
            limits.validate(),
            Err(
                SerializationError::InvalidDecodeLimits {
                    field: "max_payload_bytes"
                }
            )
        ));
    }

    #[test]
    fn checksum_is_deterministic() {
        let first =
            checksum32(b"zamani");

        let second =
            checksum32(b"zamani");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn checksum_changes_when_payload_changes() {
        let first =
            checksum32(b"zamani");

        let second =
            checksum32(b"Zamani");

        assert_ne!(
            first,
            second
        );
    }
}