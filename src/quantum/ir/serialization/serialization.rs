//! Zamani Quantum IR — Canonical Serialization Engine
//!
//! This module defines the low-level, deterministic, bounded serialization
//! contract for the Zamani Quantum Intermediate Representation.
//!
//! # Architectural position
//!
//! ```text
//!                 semantic Quantum IR
//!                         │
//!                         ▼
//!             ┌──────────────────────┐
//!             │ canonical serializer │
//!             │       this file      │
//!             └──────────┬───────────┘
//!                        │
//!                        ▼
//!              deterministic bytes
//!                        │
//!          ┌─────────────┼─────────────┐
//!          ▼             ▼             ▼
//!       storage       transport      hashing
//! ```
//!
//! This file owns the serialization *mechanism*.
//!
//! It does NOT own the semantic representation of:
//!
//! - programs;
//! - circuits;
//! - operations;
//! - gates;
//! - measurements;
//! - qubits;
//! - pulse semantics;
//! - waveforms;
//! - timing;
//! - resources;
//! - capabilities;
//! - mappings;
//! - control flow;
//! - QEC;
//! - hardware;
//! - routing;
//! - scheduling;
//! - execution.
//!
//! Those concepts implement [`IrEncode`] and [`IrDecode`] in their owning
//! modules.
//!
//! # Universal-program principle
//!
//! Serialization places no fixed quantum-machine-size limit on Zamani.
//!
//! A serialized program may describe:
//!
//! - one qubit;
//! - many qubits;
//! - very large logical namespaces;
//! - arbitrarily large finite programs;
//! - future quantum architectures;
//! - distributed quantum programs;
//! - analog programs;
//! - pulse programs;
//! - fault-tolerant programs.
//!
//! Actual resource constraints are supplied by [`DecodeLimits`] at decode time.
//!
//! A decode limit is a *security/resource policy*, not an architectural
//! limitation of Zamani Quantum IR.
//!
//! # Determinism
//!
//! Canonical serialization guarantees:
//!
//! ```text
//! same semantic object
//! + same IR version
//! + same serialization format
//! = identical bytes
//! ```
//!
//! This is required for:
//!
//! - reproducible compilation;
//! - content-addressed storage;
//! - cache keys;
//! - distributed compilation;
//! - provenance;
//! - artifact identity;
//! - canonical hashing;
//! - testing;
//! - cross-process transport.
//!
//! This module never sorts arbitrary semantic sequences. Ordering is part of
//! the owning type's semantic contract.
//!
//! # Security
//!
//! Serialized IR is untrusted input.
//!
//! Decoding therefore:
//!
//! - validates the magic;
//! - validates the format version;
//! - validates the semantic IR version;
//! - checks document length;
//! - checks payload length;
//! - checks every allocation-related length;
//! - checks every integer conversion;
//! - checks arithmetic overflow;
//! - rejects malformed booleans;
//! - rejects invalid UTF-8;
//! - rejects invalid discriminants through owning codecs;
//! - enforces nesting limits;
//! - verifies payload integrity;
//! - rejects trailing bytes;
//! - never uses `unsafe`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Quantum identity boundary
//!
//! The canonical quantum identity types remain owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file imports those types directly and never creates duplicate qubit
//! identity definitions.
//!
//! # Version boundary
//!
//! [`FORMAT_VERSION`] describes this binary framing protocol.
//!
//! [`IrVersion`] describes the semantic Quantum IR contract.
//!
//! These are deliberately independent:
//!
//! ```text
//! format version != IR version != compiler version
//!                  != Zamani language version
//!                  != hardware version
//! ```
//!
//! # Extensibility
//!
//! New IR types should implement [`IrEncode`] and [`IrDecode`] in their own
//! modules.
//!
//! This file must not be edited merely because a new quantum operation,
//! hardware architecture, pulse construct, QEC code, or dialect is introduced.
//!
//! That property is intentional.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::convert::TryFrom;
use std::fmt;

use super::identity::IrVersion;
use super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Wire format
// =============================================================================

/// Four-byte magic identifying a Zamani Quantum IR serialized document.
pub const MAGIC: [u8; 4] = *b"ZQIR";

/// Current serialization framing version.
///
/// This version describes only the binary framing contract. It is independent
/// from [`IrVersion`].
pub const FORMAT_VERSION: u16 = 1;

/// Fixed header size in bytes.
///
/// Layout:
///
/// ```text
/// magic              4 bytes
/// format version     2 bytes
/// IR major           2 bytes
/// IR minor           2 bytes
/// IR patch           2 bytes
/// payload length     8 bytes
/// checksum           4 bytes
/// --------------------------
/// total              24 bytes
/// ```
pub const HEADER_LEN: usize = 24;

/// Number of bytes occupied by the payload checksum.
const CHECKSUM_LEN: usize = 4;

// =============================================================================
// Decode policy
// =============================================================================

/// Resource policy applied while decoding serialized IR.
///
/// These limits are deliberately separate from the semantic IR model.
///
/// A small embedded compiler can provide a small policy.
///
/// A large compiler service can provide a substantially larger policy.
///
/// The values do not define how many qubits Zamani supports.
///
/// # Security
///
/// Every allocation-sensitive codec must enforce these limits before
/// allocating.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeLimits {
    /// Maximum complete document size in bytes.
    pub max_document_bytes: u64,

    /// Maximum payload size in bytes.
    pub max_payload_bytes: u64,

    /// Maximum size of one byte/string field.
    pub max_field_bytes: u64,

    /// Maximum number of elements in one collection.
    pub max_collection_elements: u64,

    /// Maximum recursive nesting depth.
    pub max_nesting_depth: u64,
}

impl DecodeLimits {
    /// Creates an explicit decode policy.
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

    /// Returns a conservative general-purpose policy.
    ///
    /// These are security defaults, not Zamani architectural limits.
    pub const fn conservative() -> Self {
        Self {
            max_document_bytes: 256 * 1024 * 1024,
            max_payload_bytes: 256 * 1024 * 1024,
            max_field_bytes: 16 * 1024 * 1024,
            max_collection_elements: 16 * 1024 * 1024,
            max_nesting_depth: 4096,
        }
    }

    /// Returns a policy intended for callers that explicitly want no
    /// protocol-level finite limit.
    ///
    /// Host memory, address-space and collection representation constraints
    /// still apply through checked conversions.
    ///
    /// This should only be used when the surrounding application already
    /// supplies an appropriate resource boundary.
    pub const fn unbounded() -> Self {
        Self {
            max_document_bytes: u64::MAX,
            max_payload_bytes: u64::MAX,
            max_field_bytes: u64::MAX,
            max_collection_elements: u64::MAX,
            max_nesting_depth: u64::MAX,
        }
    }

    /// Validates this policy before it is used.
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

/// Canonical errors produced by this serialization layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    /// Decode policy is invalid.
    InvalidDecodeLimits {
        /// Invalid field name.
        field: &'static str,
    },

    /// Complete document exceeds the selected policy.
    DocumentTooLarge {
        /// Actual/declared size.
        size: u64,

        /// Maximum permitted size.
        maximum: u64,
    },

    /// Payload exceeds the selected policy.
    PayloadTooLarge {
        /// Actual/declared size.
        size: u64,

        /// Maximum permitted size.
        maximum: u64,
    },

    /// Input ended before a required number of bytes was available.
    UnexpectedEnd {
        /// Required bytes.
        needed: usize,

        /// Available bytes.
        available: usize,
    },

    /// Invalid document magic.
    InvalidMagic {
        /// Four bytes encountered.
        found: [u8; 4],
    },

    /// Unsupported serialization format.
    UnsupportedFormatVersion {
        /// Encountered version.
        version: u16,
    },

    /// Unsupported semantic IR version.
    UnsupportedIrVersion {
        /// Encountered semantic version.
        version: IrVersion,
    },

    /// Canonical documents cannot contain extra bytes.
    TrailingBytes {
        /// Number of extra bytes.
        count: usize,
    },

    /// Payload checksum mismatch.
    ChecksumMismatch {
        /// Stored checksum.
        expected: u32,

        /// Calculated checksum.
        actual: u32,
    },

    /// Wire length cannot be represented by the host.
    LengthOverflow {
        /// Field/context.
        context: &'static str,

        /// Wire value.
        value: u64,
    },

    /// Arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Operation context.
        context: &'static str,
    },

    /// Collection count exceeds the active policy.
    CollectionLimitExceeded {
        /// Collection context.
        context: &'static str,

        /// Requested count.
        requested: u64,

        /// Maximum permitted count.
        maximum: u64,
    },

    /// Field length exceeds the active policy.
    FieldLimitExceeded {
        /// Field context.
        context: &'static str,

        /// Requested size.
        requested: u64,

        /// Maximum permitted size.
        maximum: u64,
    },

    /// Nesting exceeds the active policy.
    NestingLimitExceeded {
        /// Requested depth.
        requested: u64,

        /// Maximum permitted depth.
        maximum: u64,
    },

    /// Boolean byte is not canonical.
    InvalidBoolean {
        /// Invalid byte.
        value: u8,
    },

    /// UTF-8 validation failed.
    InvalidUtf8,

    /// Generic invalid IR object.
    InvalidObject {
        /// Static reason.
        message: &'static str,
    },

    /// A user-defined codec rejected data.
    Codec {
        /// Codec-specific message.
        message: String,
    },
}

impl fmt::Display for SerializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDecodeLimits { field } => {
                write!(formatter, "invalid decode limit `{field}`")
            }

            Self::DocumentTooLarge { size, maximum } => {
                write!(
                    formatter,
                    "serialized document is too large: {size} bytes, maximum {maximum}"
                )
            }

            Self::PayloadTooLarge { size, maximum } => {
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
                    "invalid Zamani Quantum IR magic: {:02x}{:02x}{:02x}{:02x}",
                    found[0],
                    found[1],
                    found[2],
                    found[3]
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
                    "unsupported Quantum IR semantic version {version}"
                )
            }

            Self::TrailingBytes { count } => {
                write!(
                    formatter,
                    "canonical serialized IR contains {count} trailing bytes"
                )
            }

            Self::ChecksumMismatch { expected, actual } => {
                write!(
                    formatter,
                    "serialized IR checksum mismatch: expected {expected:#010x}, actual {actual:#010x}"
                )
            }

            Self::LengthOverflow { context, value } => {
                write!(
                    formatter,
                    "wire length {value} cannot be represented on this host while decoding {context}"
                )
            }

            Self::ArithmeticOverflow { context } => {
                write!(
                    formatter,
                    "arithmetic overflow while processing serialized IR: {context}"
                )
            }

            Self::CollectionLimitExceeded {
                context,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "collection `{context}` contains {requested} elements, maximum {maximum}"
                )
            }

            Self::FieldLimitExceeded {
                context,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "field `{context}` contains {requested} bytes, maximum {maximum}"
                )
            }

            Self::NestingLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "serialization nesting depth {requested} exceeds maximum {maximum}"
                )
            }

            Self::InvalidBoolean { value } => {
                write!(
                    formatter,
                    "invalid canonical boolean byte {value:#04x}"
                )
            }

            Self::InvalidUtf8 => {
                write!(formatter, "serialized string is not valid UTF-8")
            }

            Self::InvalidObject { message } => {
                write!(formatter, "invalid IR object: {message}")
            }

            Self::Codec { message } => {
                write!(formatter, "IR codec error: {message}")
            }
        }
    }
}

impl std::error::Error for SerializationError {}

/// Result type used by the serialization subsystem.
pub type SerializationResult<T> = Result<T, SerializationError>;

// =============================================================================
// Codec traits
// =============================================================================

/// Trait implemented by semantic IR objects that have a canonical encoding.
///
/// Implementations belong to the owning semantic module.
///
/// They must:
///
/// - emit fields in a fixed canonical order;
/// - never emit nondeterministic map ordering;
/// - preserve semantic sequence ordering;
/// - use canonical primitive encoders;
/// - reject invalid semantic state before encoding when appropriate.
///
/// The trait deliberately contains no knowledge of hardware or execution.
pub trait IrEncode {
    /// Encodes this semantic object into the canonical payload encoder.
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()>;
}

/// Trait implemented by semantic IR objects that can be reconstructed from
/// canonical serialized data.
///
/// Implementations must use the supplied decoder rather than directly
/// indexing the byte slice.
///
/// This guarantees common bounds, length, UTF-8 and nesting checks.
pub trait IrDecode: Sized {
    /// Decodes one semantic object.
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self>;
}

// =============================================================================
// Document
// =============================================================================

/// Serializes an IR object using the current IR semantic version.
pub fn serialize<T: IrEncode>(value: &T) -> SerializationResult<Vec<u8>> {
    serialize_with_version(value, IrVersion::CURRENT)
}

/// Serializes an IR object using an explicitly supplied semantic IR version.
///
/// The caller must use a version appropriate for the object being encoded.
pub fn serialize_with_version<T: IrEncode>(
    value: &T,
    ir_version: IrVersion,
) -> SerializationResult<Vec<u8>> {
    if !ir_version.is_supported_by_current() {
        return Err(SerializationError::UnsupportedIrVersion {
            version: ir_version,
        });
    }

    let mut payload_encoder = Encoder::new();

    value.encode(&mut payload_encoder)?;

    let payload = payload_encoder.into_bytes();

    let payload_len = u64::try_from(payload.len()).map_err(|_| {
        SerializationError::LengthOverflow {
            context: "serialized payload",
            value: payload.len() as u64,
        }
    })?;

    let document_len = (HEADER_LEN as u64)
        .checked_add(payload_len)
        .ok_or(SerializationError::ArithmeticOverflow {
            context: "document length",
        })?;

    let mut output = Vec::with_capacity(
        HEADER_LEN
            .checked_add(payload.len())
            .ok_or(SerializationError::ArithmeticOverflow {
                context: "output capacity",
            })?,
    );

    output.extend_from_slice(&MAGIC);
    output.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    output.extend_from_slice(&ir_version.major().to_le_bytes());
    output.extend_from_slice(&ir_version.minor().to_le_bytes());
    output.extend_from_slice(&ir_version.patch().to_le_bytes());
    output.extend_from_slice(&payload_len.to_le_bytes());

    let checksum = crc32(&payload);
    output.extend_from_slice(&checksum.to_le_bytes());

    output.extend_from_slice(&payload);

    debug_assert_eq!(
        output.len() as u64,
        document_len,
        "canonical serialization length accounting must be exact"
    );

    Ok(output)
}

/// Deserializes an IR object using the default decode policy.
pub fn deserialize<T: IrDecode>(bytes: &[u8]) -> SerializationResult<T> {
    deserialize_with_limits(bytes, DecodeLimits::default())
}

/// Deserializes an IR object using an explicit decode policy.
pub fn deserialize_with_limits<T: IrDecode>(
    bytes: &[u8],
    limits: DecodeLimits,
) -> SerializationResult<T> {
    limits.validate()?;

    let document_size = u64::try_from(bytes.len()).map_err(|_| {
        SerializationError::LengthOverflow {
            context: "document size",
            value: bytes.len() as u64,
        }
    })?;

    if document_size > limits.max_document_bytes {
        return Err(SerializationError::DocumentTooLarge {
            size: document_size,
            maximum: limits.max_document_bytes,
        });
    }

    if bytes.len() < HEADER_LEN {
        return Err(SerializationError::UnexpectedEnd {
            needed: HEADER_LEN,
            available: bytes.len(),
        });
    }

    let mut decoder = Decoder::new(bytes, limits);

    let magic = decoder.read_fixed::<4>()?;

    if magic != MAGIC {
        return Err(SerializationError::InvalidMagic { found: magic });
    }

    let format_version = decoder.read_u16()?;

    if format_version != FORMAT_VERSION {
        return Err(SerializationError::UnsupportedFormatVersion {
            version: format_version,
        });
    }

    let ir_major = decoder.read_u16()?;
    let ir_minor = decoder.read_u16()?;
    let ir_patch = decoder.read_u16()?;

    let ir_version = IrVersion::new(
        ir_major,
        ir_minor,
        ir_patch,
    );

    if !ir_version.is_supported_by_current() {
        return Err(SerializationError::UnsupportedIrVersion {
            version: ir_version,
        });
    }

    let payload_length = decoder.read_u64()?;
    let expected_checksum = decoder.read_u32()?;

    if payload_length > limits.max_payload_bytes {
        return Err(SerializationError::PayloadTooLarge {
            size: payload_length,
            maximum: limits.max_payload_bytes,
        });
    }

    let payload_length_usize = usize::try_from(payload_length).map_err(|_| {
        SerializationError::LengthOverflow {
            context: "payload length",
            value: payload_length,
        }
    })?;

    let expected_document_size = HEADER_LEN
        .checked_add(payload_length_usize)
        .ok_or(SerializationError::ArithmeticOverflow {
            context: "expected document size",
        })?;

    if expected_document_size > bytes.len() {
        return Err(SerializationError::UnexpectedEnd {
            needed: expected_document_size,
            available: bytes.len(),
        });
    }

    if expected_document_size < bytes.len() {
        return Err(SerializationError::TrailingBytes {
            count: bytes.len() - expected_document_size,
        });
    }

    let payload = decoder.read_exact(payload_length_usize)?;

    let actual_checksum = crc32(payload);

    if actual_checksum != expected_checksum {
        return Err(SerializationError::ChecksumMismatch {
            expected: expected_checksum,
            actual: actual_checksum,
        });
    }

    let mut payload_decoder = Decoder::new(payload, limits);

    let value = T::decode(&mut payload_decoder)?;

    payload_decoder.finish()?;

    decoder.finish()?;

    Ok(value)
}

// =============================================================================
// Encoder
// =============================================================================

/// Deterministic canonical payload encoder.
///
/// This type contains no semantic knowledge of the Quantum IR.
#[derive(Debug, Default)]
pub struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    /// Creates an empty payload encoder.
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
        }
    }

    /// Creates an encoder with a caller-supplied capacity.
    ///
    /// Capacity is an allocation hint only.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    /// Returns the encoded payload without copying it.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Returns the current encoded length.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the payload is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Encodes a boolean using exactly one canonical byte.
    pub fn write_bool(&mut self, value: bool) {
        self.bytes.push(if value { 1 } else { 0 });
    }

    /// Encodes an unsigned 8-bit integer.
    pub fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    /// Encodes an unsigned 16-bit integer.
    pub fn write_u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Encodes an unsigned 32-bit integer.
    pub fn write_u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Encodes an unsigned 64-bit integer.
    pub fn write_u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Encodes a signed 8-bit integer.
    pub fn write_i8(&mut self, value: i8) {
        self.bytes.push(value as u8);
    }

    /// Encodes a signed 16-bit integer.
    pub fn write_i16(&mut self, value: i16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Encodes a signed 32-bit integer.
    pub fn write_i32(&mut self, value: i32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Encodes a signed 64-bit integer.
    pub fn write_i64(&mut self, value: i64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Encodes a 32-bit IEEE floating-point value.
    pub fn write_f32(&mut self, value: f32) {
        self.write_u32(value.to_bits());
    }

    /// Encodes a 64-bit IEEE floating-point value.
    ///
    /// Canonical IR types should ensure that NaN policy is handled by their
    /// owning semantic contract before reaching this primitive.
    pub fn write_f64(&mut self, value: f64) {
        self.write_u64(value.to_bits());
    }

    /// Encodes a byte slice with a u64 length prefix.
    pub fn write_bytes(&mut self, value: &[u8]) -> SerializationResult<()> {
        let length = u64::try_from(value.len()).map_err(|_| {
            SerializationError::LengthOverflow {
                context: "byte field",
                value: value.len() as u64,
            }
        })?;

        self.write_u64(length);
        self.bytes.extend_from_slice(value);

        Ok(())
    }

    /// Encodes a UTF-8 string with a u64 byte length.
    pub fn write_str(&mut self, value: &str) -> SerializationResult<()> {
        self.write_bytes(value.as_bytes())
    }

    /// Encodes an optional value.
    pub fn write_option<T: IrEncode>(
        &mut self,
        value: Option<&T>,
    ) -> SerializationResult<()> {
        match value {
            None => {
                self.write_bool(false);
                Ok(())
            }

            Some(value) => {
                self.write_bool(true);
                value.encode(self)
            }
        }
    }

    /// Encodes a sequence with a u64 element count.
    ///
    /// The sequence order is preserved exactly.
    pub fn write_sequence<T, I>(
        &mut self,
        values: I,
    ) -> SerializationResult<()>
    where
        T: IrEncode,
        I: IntoIterator<Item = T>,
    {
        let values: Vec<T> = values.into_iter().collect();

        let count = u64::try_from(values.len()).map_err(|_| {
            SerializationError::LengthOverflow {
                context: "sequence element count",
                value: values.len() as u64,
            }
        })?;

        self.write_u64(count);

        for value in values {
            value.encode(self)?;
        }

        Ok(())
    }

    /// Encodes a sequence from references without requiring ownership.
    pub fn write_sequence_ref<T, I>(
        &mut self,
        values: I,
    ) -> SerializationResult<()>
    where
        T: IrEncode,
        I: IntoIterator<Item = T>,
    {
        self.write_sequence(values)
    }

    /// Appends raw bytes.
    ///
    /// This is intended for carefully specified extension/dialect payloads.
    pub fn write_raw(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }
}

// =============================================================================
// Decoder
// =============================================================================

/// Bounded canonical payload decoder.
///
/// The decoder maintains its position without exposing direct indexing to
/// callers. This centralizes bounds checking.
pub struct Decoder<'a> {
    bytes: &'a [u8],
    position: usize,
    limits: DecodeLimits,
    depth: u64,
}

impl<'a> Decoder<'a> {
    /// Creates a decoder over a complete payload.
    pub fn new(
        bytes: &'a [u8],
        limits: DecodeLimits,
    ) -> Self {
        Self {
            bytes,
            position: 0,
            limits,
            depth: 0,
        }
    }

    /// Returns the remaining number of bytes.
    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    /// Returns the current byte position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns the active decode limits.
    pub const fn limits(&self) -> DecodeLimits {
        self.limits
    }

    /// Ensures the decoder consumed its complete input.
    pub fn finish(&self) -> SerializationResult<()> {
        if self.position != self.bytes.len() {
            return Err(SerializationError::TrailingBytes {
                count: self.bytes.len() - self.position,
            });
        }

        Ok(())
    }

    /// Reads one byte.
    pub fn read_u8(&mut self) -> SerializationResult<u8> {
        let bytes = self.read_exact(1)?;
        Ok(bytes[0])
    }

    /// Reads a canonical boolean.
    pub fn read_bool(&mut self) -> SerializationResult<bool> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(SerializationError::InvalidBoolean { value }),
        }
    }

    /// Reads a little-endian u16.
    pub fn read_u16(&mut self) -> SerializationResult<u16> {
        Ok(u16::from_le_bytes(self.read_fixed::<2>()?))
    }

    /// Reads a little-endian u32.
    pub fn read_u32(&mut self) -> SerializationResult<u32> {
        Ok(u32::from_le_bytes(self.read_fixed::<4>()?))
    }

    /// Reads a little-endian u64.
    pub fn read_u64(&mut self) -> SerializationResult<u64> {
        Ok(u64::from_le_bytes(self.read_fixed::<8>()?))
    }

    /// Reads a little-endian i8.
    pub fn read_i8(&mut self) -> SerializationResult<i8> {
        Ok(self.read_u8()? as i8)
    }

    /// Reads a little-endian i16.
    pub fn read_i16(&mut self) -> SerializationResult<i16> {
        Ok(i16::from_le_bytes(self.read_fixed::<2>()?))
    }

    /// Reads a little-endian i32.
    pub fn read_i32(&mut self) -> SerializationResult<i32> {
        Ok(i32::from_le_bytes(self.read_fixed::<4>()?))
    }

    /// Reads a little-endian i64.
    pub fn read_i64(&mut self) -> SerializationResult<i64> {
        Ok(i64::from_le_bytes(self.read_fixed::<8>()?))
    }

    /// Reads a 32-bit IEEE floating-point value.
    pub fn read_f32(&mut self) -> SerializationResult<f32> {
        Ok(f32::from_bits(self.read_u32()?))
    }

    /// Reads a 64-bit IEEE floating-point value.
    pub fn read_f64(&mut self) -> SerializationResult<f64> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    /// Reads a fixed-size byte array.
    pub fn read_fixed<const N: usize>(
        &mut self,
    ) -> SerializationResult<[u8; N]> {
        let bytes = self.read_exact(N)?;

        let mut output = [0u8; N];
        output.copy_from_slice(bytes);

        Ok(output)
    }

    /// Reads an exact byte slice.
    pub fn read_exact(
        &mut self,
        length: usize,
    ) -> SerializationResult<&'a [u8]> {
        let end = self.position.checked_add(length).ok_or(
            SerializationError::ArithmeticOverflow {
                context: "decoder position",
            },
        )?;

        if end > self.bytes.len() {
            return Err(SerializationError::UnexpectedEnd {
                needed: end,
                available: self.bytes.len(),
            });
        }

        let start = self.position;
        self.position = end;

        Ok(&self.bytes[start..end])
    }

    /// Reads a u64 length and checks it against the field policy.
    pub fn read_length(
        &mut self,
        context: &'static str,
    ) -> SerializationResult<u64> {
        let length = self.read_u64()?;

        if length > self.limits.max_field_bytes {
            return Err(SerializationError::FieldLimitExceeded {
                context,
                requested: length,
                maximum: self.limits.max_field_bytes,
            });
        }

        Ok(length)
    }

    /// Converts a wire length to host usize after policy checking.
    pub fn length_to_usize(
        &self,
        length: u64,
        context: &'static str,
    ) -> SerializationResult<usize> {
        usize::try_from(length).map_err(|_| {
            SerializationError::LengthOverflow {
                context,
                value: length,
            }
        })
    }

    /// Reads a length-prefixed byte field.
    pub fn read_bytes(
        &mut self,
        context: &'static str,
    ) -> SerializationResult<Vec<u8>> {
        let length = self.read_length(context)?;

        let length = self.length_to_usize(length, context)?;

        Ok(self.read_exact(length)?.to_vec())
    }

    /// Reads a UTF-8 string.
    pub fn read_string(
        &mut self,
        context: &'static str,
    ) -> SerializationResult<String> {
        let bytes = self.read_bytes(context)?;

        String::from_utf8(bytes)
            .map_err(|_| SerializationError::InvalidUtf8)
    }

    /// Reads an optional value.
    pub fn read_option<T: IrDecode>(
        &mut self,
    ) -> SerializationResult<Option<T>> {
        if self.read_bool()? {
            Ok(Some(T::decode(self)?))
        } else {
            Ok(None)
        }
    }

    /// Reads a collection count and checks it against the active policy.
    pub fn read_collection_count(
        &mut self,
        context: &'static str,
    ) -> SerializationResult<usize> {
        let count = self.read_u64()?;

        if count > self.limits.max_collection_elements {
            return Err(SerializationError::CollectionLimitExceeded {
                context,
                requested: count,
                maximum: self.limits.max_collection_elements,
            });
        }

        self.length_to_usize(count, context)
    }

    /// Reads a collection whose elements implement [`IrDecode`].
    ///
    /// Allocation occurs only after the count has passed the policy and host
    /// conversion checks.
    pub fn read_vec<T: IrDecode>(
        &mut self,
        context: &'static str,
    ) -> SerializationResult<Vec<T>> {
        let count = self.read_collection_count(context)?;

        let mut values = Vec::with_capacity(count);

        for _ in 0..count {
            values.push(T::decode(self)?);
        }

        Ok(values)
    }

    /// Enters a nested semantic structure.
    ///
    /// The returned guard restores the previous depth when dropped.
    pub fn enter_scope(&mut self) -> SerializationResult<DecodeScope<'_>> {
        let requested = self.depth.checked_add(1).ok_or(
            SerializationError::ArithmeticOverflow {
                context: "decoder nesting depth",
            },
        )?;

        if requested > self.limits.max_nesting_depth {
            return Err(SerializationError::NestingLimitExceeded {
                requested,
                maximum: self.limits.max_nesting_depth,
            });
        }

        self.depth = requested;

        Ok(DecodeScope { decoder: self })
    }

    /// Returns the current nesting depth.
    pub const fn depth(&self) -> u64 {
        self.depth
    }
}

/// Scope guard for bounded nested decoding.
pub struct DecodeScope<'a> {
    decoder: &'a mut Decoder<'a>,
}

impl Drop for DecodeScope<'_> {
    fn drop(&mut self) {
        self.decoder.depth = self.decoder.depth.saturating_sub(1);
    }
}

// =============================================================================
// Primitive IR identity codecs
// =============================================================================

impl IrEncode for QubitId {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_u64(self.value());
        Ok(())
    }
}

impl IrDecode for QubitId {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        Ok(QubitId::new(decoder.read_u64()?))
    }
}

impl IrEncode for PhysicalQubitId {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_u64(self.value());
        Ok(())
    }
}

impl IrDecode for PhysicalQubitId {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        Ok(PhysicalQubitId::new(decoder.read_u64()?))
    }
}

// =============================================================================
// Primitive standard-library codecs
// =============================================================================

impl IrEncode for bool {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_bool(*self);
        Ok(())
    }
}

impl IrDecode for bool {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_bool()
    }
}

impl IrEncode for u8 {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_u8(*self);
        Ok(())
    }
}

impl IrDecode for u8 {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_u8()
    }
}

impl IrEncode for u16 {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_u16(*self);
        Ok(())
    }
}

impl IrDecode for u16 {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_u16()
    }
}

impl IrEncode for u32 {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_u32(*self);
        Ok(())
    }
}

impl IrDecode for u32 {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_u32()
    }
}

impl IrEncode for u64 {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_u64(*self);
        Ok(())
    }
}

impl IrDecode for u64 {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_u64()
    }
}

impl IrEncode for i8 {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_i8(*self);
        Ok(())
    }
}

impl IrDecode for i8 {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_i8()
    }
}

impl IrEncode for i16 {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_i16(*self);
        Ok(())
    }
}

impl IrDecode for i16 {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_i16()
    }
}

impl IrEncode for i32 {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_i32(*self);
        Ok(())
    }
}

impl IrDecode for i32 {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_i32()
    }
}

impl IrEncode for i64 {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_i64(*self);
        Ok(())
    }
}

impl IrDecode for i64 {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_i64()
    }
}

impl IrEncode for f32 {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_f32(*self);
        Ok(())
    }
}

impl IrDecode for f32 {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_f32()
    }
}

impl IrEncode for f64 {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_f64(*self);
        Ok(())
    }
}

impl IrDecode for f64 {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_f64()
    }
}

impl IrEncode for String {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_str(self)
    }
}

impl IrDecode for String {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_string("string")
    }
}

impl<T: IrEncode> IrEncode for Option<T> {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        match self {
            None => encoder.write_bool(false),

            Some(value) => {
                encoder.write_bool(true);
                value.encode(encoder)?;
            }
        }

        Ok(())
    }
}

impl<T: IrDecode> IrDecode for Option<T> {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        decoder.read_option::<T>()
    }
}

// =============================================================================
// IR version codec
// =============================================================================

impl IrEncode for IrVersion {
    fn encode(&self, encoder: &mut Encoder) -> SerializationResult<()> {
        encoder.write_u16(self.major());
        encoder.write_u16(self.minor());
        encoder.write_u16(self.patch());

        Ok(())
    }
}

impl IrDecode for IrVersion {
    fn decode(decoder: &mut Decoder<'_>) -> SerializationResult<Self> {
        Ok(IrVersion::new(
            decoder.read_u16()?,
            decoder.read_u16()?,
            decoder.read_u16()?,
        ))
    }
}

// =============================================================================
// Canonical checksum
// =============================================================================

/// Calculates the deterministic CRC-32 checksum used by the serialization
/// framing layer.
///
/// This checksum detects accidental corruption and malformed transport data.
///
/// It is NOT a cryptographic authentication mechanism.
///
/// Cryptographic content identity belongs to the dedicated IR hashing layer.
pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;

    for &byte in bytes {
        crc ^= byte as u32;

        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);

            crc = (crc >> 1) ^ (0xedb8_8320u32 & mask);
        }
    }

    !crc
}

// =============================================================================
// Canonical encoding helpers
// =============================================================================

/// Encodes a sequence without collecting it first.
///
/// This is useful for large IR objects where the caller already knows the
/// number of elements and wants to avoid an intermediate collection.
pub fn encode_sequence<T, I>(
    encoder: &mut Encoder,
    count: u64,
    values: I,
) -> SerializationResult<()>
where
    T: IrEncode,
    I: IntoIterator<Item = T>,
{
    encoder.write_u64(count);

    let mut actual = 0u64;

    for value in values {
        value.encode(encoder)?;

        actual = actual.checked_add(1).ok_or(
            SerializationError::ArithmeticOverflow {
                context: "sequence element count",
            },
        )?;
    }

    if actual != count {
        return Err(SerializationError::InvalidObject {
            message: "declared sequence count does not match encoded sequence",
        });
    }

    Ok(())
}

/// Encodes a sequence of references without requiring ownership.
pub fn encode_sequence_refs<'a, T, I>(
    encoder: &mut Encoder,
    count: u64,
    values: I,
) -> SerializationResult<()>
where
    T: IrEncode + 'a,
    I: IntoIterator<Item = &'a T>,
{
    encoder.write_u64(count);

    let mut actual = 0u64;

    for value in values {
        value.encode(encoder)?;

        actual = actual.checked_add(1).ok_or(
            SerializationError::ArithmeticOverflow {
                context: "sequence element count",
            },
        )?;
    }

    if actual != count {
        return Err(SerializationError::InvalidObject {
            message: "declared sequence count does not match encoded sequence",
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestObject {
        id: QubitId,
        enabled: bool,
        name: String,
    }

    impl IrEncode for TestObject {
        fn encode(
            &self,
            encoder: &mut Encoder,
        ) -> SerializationResult<()> {
            self.id.encode(encoder)?;
            self.enabled.encode(encoder)?;
            self.name.encode(encoder)?;

            Ok(())
        }
    }

    impl IrDecode for TestObject {
        fn decode(
            decoder: &mut Decoder<'_>,
        ) -> SerializationResult<Self> {
            Ok(Self {
                id: QubitId::decode(decoder)?,
                enabled: bool::decode(decoder)?,
                name: String::decode(decoder)?,
            })
        }
    }

    #[test]
    fn round_trip_test_object() {
        let original = TestObject {
            id: QubitId::new(123),
            enabled: true,
            name: String::from("zamani"),
        };

        let bytes = serialize(&original).expect("serialization must succeed");

        let decoded: TestObject =
            deserialize(&bytes).expect("deserialization must succeed");

        assert_eq!(decoded, original);
    }

    #[test]
    fn serialization_is_deterministic() {
        let original = TestObject {
            id: QubitId::new(42),
            enabled: true,
            name: String::from("deterministic"),
        };

        let first = serialize(&original).expect("first serialization");
        let second = serialize(&original).expect("second serialization");

        assert_eq!(first, second);
    }

    #[test]
    fn different_values_produce_different_payloads() {
        let first = TestObject {
            id: QubitId::new(1),
            enabled: true,
            name: String::from("a"),
        };

        let second = TestObject {
            id: QubitId::new(2),
            enabled: true,
            name: String::from("a"),
        };

        let first_bytes =
            serialize(&first).expect("first serialization");

        let second_bytes =
            serialize(&second).expect("second serialization");

        assert_ne!(first_bytes, second_bytes);
    }

    #[test]
    fn invalid_magic_is_rejected() {
        let original = TestObject {
            id: QubitId::new(1),
            enabled: true,
            name: String::from("x"),
        };

        let mut bytes =
            serialize(&original).expect("serialization");

        bytes[0] ^= 0xff;

        let result: SerializationResult<TestObject> =
            deserialize(&bytes);

        assert!(matches!(
            result,
            Err(SerializationError::InvalidMagic { .. })
        ));
    }

    #[test]
    fn truncated_document_is_rejected() {
        let original = TestObject {
            id: QubitId::new(1),
            enabled: true,
            name: String::from("x"),
        };

        let mut bytes =
            serialize(&original).expect("serialization");

        bytes.pop();

        let result: SerializationResult<TestObject> =
            deserialize(&bytes);

        assert!(matches!(
            result,
            Err(SerializationError::UnexpectedEnd { .. })
        ));
    }

    #[test]
    fn trailing_bytes_are_rejected() {
        let original = TestObject {
            id: QubitId::new(1),
            enabled: true,
            name: String::from("x"),
        };

        let mut bytes =
            serialize(&original).expect("serialization");

        bytes.push(0);

        let result: SerializationResult<TestObject> =
            deserialize(&bytes);

        assert!(matches!(
            result,
            Err(SerializationError::TrailingBytes { .. })
        ));
    }

    #[test]
    fn checksum_corruption_is_rejected() {
        let original = TestObject {
            id: QubitId::new(1),
            enabled: true,
            name: String::from("x"),
        };

        let mut bytes =
            serialize(&original).expect("serialization");

        let last = bytes.len() - 1;
        bytes[last] ^= 1;

        let result: SerializationResult<TestObject> =
            deserialize(&bytes);

        assert!(matches!(
            result,
            Err(SerializationError::ChecksumMismatch { .. })
        ));
    }

    #[test]
    fn invalid_boolean_is_rejected() {
        let mut decoder =
            Decoder::new(&[2], DecodeLimits::default());

        let result = decoder.read_bool();

        assert!(matches!(
            result,
            Err(SerializationError::InvalidBoolean { value: 2 })
        ));
    }

    #[test]
    fn invalid_utf8_is_rejected() {
        let mut encoder = Encoder::new();

        encoder
            .write_bytes(&[0xff])
            .expect("encoding must succeed");

        let bytes = encoder.into_bytes();

        let mut decoder =
            Decoder::new(&bytes, DecodeLimits::default());

        let result = decoder.read_string("test");

        assert!(matches!(
            result,
            Err(SerializationError::InvalidUtf8)
        ));
    }

    #[test]
    fn decode_collection_limit_is_enforced() {
        let mut encoder = Encoder::new();

        encoder.write_u64(10);

        let bytes = encoder.into_bytes();

        let limits = DecodeLimits::new(
            1024,
            1024,
            1024,
            5,
            16,
        );

        let mut decoder =
            Decoder::new(&bytes, limits);

        let result = decoder.read_collection_count("test");

        assert!(matches!(
            result,
            Err(SerializationError::CollectionLimitExceeded {
                requested: 10,
                maximum: 5,
                ..
            })
        ));
    }

    #[test]
    fn field_limit_is_enforced() {
        let mut encoder = Encoder::new();

        encoder.write_u64(100);

        let bytes = encoder.into_bytes();

        let limits = DecodeLimits::new(
            1024,
            1024,
            10,
            100,
            16,
        );

        let mut decoder =
            Decoder::new(&bytes, limits);

        let result = decoder.read_bytes("test");

        assert!(matches!(
            result,
            Err(SerializationError::FieldLimitExceeded {
                requested: 100,
                maximum: 10,
                ..
            })
        ));
    }

    #[test]
    fn qubit_identity_round_trips() {
        let qubit = QubitId::new(u64::MAX);

        let bytes =
            serialize(&qubit).expect("serialization");

        let decoded: QubitId =
            deserialize(&bytes).expect("deserialization");

        assert_eq!(decoded, qubit);
        assert_eq!(decoded.value(), u64::MAX);
    }

    #[test]
    fn physical_qubit_identity_round_trips() {
        let qubit = PhysicalQubitId::new(u64::MAX);

        let bytes =
            serialize(&qubit).expect("serialization");

        let decoded: PhysicalQubitId =
            deserialize(&bytes).expect("deserialization");

        assert_eq!(decoded, qubit);
        assert_eq!(decoded.value(), u64::MAX);
    }

    #[test]
    fn version_round_trips() {
        let version = IrVersion::new(1, 0, 0);

        let mut encoder = Encoder::new();

        version
            .encode(&mut encoder)
            .expect("version encoding");

        let bytes = encoder.into_bytes();

        let mut decoder =
            Decoder::new(&bytes, DecodeLimits::default());

        let decoded =
            IrVersion::decode(&mut decoder)
                .expect("version decoding");

        assert_eq!(decoded, version);
    }

    #[test]
    fn checksum_is_deterministic() {
        let bytes = b"zamani quantum ir";

        assert_eq!(
            crc32(bytes),
            crc32(bytes)
        );
    }

    #[test]
    fn checksum_changes_when_payload_changes() {
        let first = crc32(b"zamani");
        let second = crc32(b"Zamani");

        assert_ne!(first, second);
    }

    #[test]
    fn unbounded_policy_is_valid() {
        assert!(
            DecodeLimits::unbounded()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn conservative_policy_is_valid() {
        assert!(
            DecodeLimits::conservative()
                .validate()
                .is_ok()
        );
    }
}