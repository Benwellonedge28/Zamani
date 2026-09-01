//! Zamani Quantum IR — Canonical Serialization Core
//!
//! This module defines the canonical, deterministic, bounded wire primitives
//! and document framing used by the Zamani Quantum IR serialization layer.
//!
//! # Architectural responsibility
//!
//! This file owns ONLY:
//!
//! - canonical document framing;
//! - serialization-format versioning;
//! - IR-version framing;
//! - deterministic primitive encoding;
//! - deterministic primitive decoding;
//! - checked length handling;
//! - explicit decode-resource limits;
//! - canonical boolean representation;
//! - canonical UTF-8 handling;
//! - canonical byte/string representation;
//! - canonical logical/physical qubit identity encoding;
//! - integrity checksum calculation;
//! - structural document validation.
//!
//! This file does NOT own:
//!
//! - quantum semantics;
//! - gates;
//! - measurements;
//! - programs;
//! - operations;
//! - regions;
//! - control flow;
//! - pulse semantics;
//! - waveforms;
//! - resources;
//! - hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - simulation;
//! - QEC;
//! - frontend parsing;
//! - backend execution.
//!
//! Higher-level IR objects implement `IrEncode`/`IrDecode` in the parent
//! serialization module and use `CanonicalWriter`/`CanonicalReader` here.
//!
//! # Canonical architecture
//!
//! ```text
//! semantic IR object
//!        │
//!        ▼
//! IrEncode
//!        │
//!        ▼
//! CanonicalWriter
//!        │
//!        ▼
//! canonical payload
//!        │
//!        ▼
//! encode_document()
//!        │
//!        ├── magic
//!        ├── serialization format version
//!        ├── IR semantic version
//!        ├── payload length
//!        ├── payload checksum
//!        └── payload
//!
//! deserialize:
//!
//! bytes
//!   │
//!   ▼
//! decode_document()
//!   │
//!   ├── structural checks
//!   ├── version checks
//!   ├── length checks
//!   ├── checksum verification
//!   └── trailing-byte rejection
//!   │
//!   ▼
//! CanonicalReader
//!   │
//!   ▼
//! IrDecode
//!   │
//!   ▼
//! semantic IR object
//! ```
//!
//! # Scalability
//!
//! There is intentionally NO architectural quantum-machine maximum here.
//!
//! In particular this file never defines:
//!
//! - maximum qubits;
//! - maximum logical qubits;
//! - maximum physical qubits;
//! - maximum gates;
//! - maximum operations;
//! - maximum registers;
//! - maximum topology size.
//!
//! Wire collection lengths are represented as `u64`.
//!
//! Host allocation is controlled separately by `DecodeLimits`.
//!
//! Therefore:
//!
//! ```text
//! semantic capacity != serialization representation limit
//! semantic capacity != host allocation policy
//! semantic capacity != hardware capacity
//! ```
//!
//! "Infinity" means that the schema contains no fixed quantum-machine bound.
//! Every concrete execution remains finite and is bounded by the available
//! resources and explicit compilation/decoding policy.
//!
//! # Security
//!
//! Serialized IR is untrusted input.
//!
//! Decoding therefore:
//!
//! - checks the magic before payload interpretation;
//! - checks the serialization format version;
//! - checks IR-version compatibility;
//! - checks every length before allocation;
//! - checks every `u64 -> usize` conversion;
//! - checks arithmetic overflow;
//! - rejects non-canonical booleans;
//! - rejects invalid UTF-8;
//! - rejects truncated fields;
//! - rejects trailing bytes in canonical documents;
//! - verifies the payload checksum;
//! - never allocates directly from an unchecked attacker-controlled length;
//! - enforces explicit nesting, collection, field, payload, and document limits.
//!
//! No `unsafe` code is used.
//!
//! `#![forbid(unsafe_code)]` makes this requirement compiler-enforced.
//!
//! # Determinism
//!
//! This module never sorts arbitrary semantic sequences.
//!
//! If an IR sequence is semantically ordered, callers MUST encode it in that
//! order.
//!
//! If a map has semantic unorderedness, the owning module MUST provide a
//! deterministic order, normally through `BTreeMap`, `BTreeSet`, or an
//! explicitly sorted canonical representation.
//!
//! This module provides deterministic primitives but does not change semantic
//! ordering.
//!
//! # Qubit identity boundary
//!
//! The canonical identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file never defines a second qubit-ID type.
//!
//! The current `qubit.rs` implementation uses `usize` internally. The wire
//! representation is nevertheless always `u64`, with checked conversion on
//! decode. This keeps the persistent format independent of host pointer width
//! and provides a migration boundary for the eventual `u64` identity model.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - no nightly features;
//! - no `unsafe`;
//! - no external dependency required by this module.
//!
//! # Integration contract
//!
//! The parent `serialization` module should:
//!
//! 1. declare this module:
//!
//! ```text
//! pub mod canonical;
//! ```
//!
//! 2. use `CanonicalWriter` inside `IrEncode` implementations;
//!
//! 3. use `CanonicalReader` inside `IrDecode` implementations;
//!
//! 4. use `encode_document()` from its public `serialize()` function;
//!
//! 5. use `decode_document()` from its public `deserialize()` function;
//!
//! 6. convert `CanonicalError` into the public serialization error type, or
//!    re-export `CanonicalError` if it is intentionally made the common
//!    serialization error contract.
//!
//! `hash.rs` should hash the exact bytes returned by `encode_document()`.
//!
//! `validation.rs` remains responsible for semantic validation after decoding.
//!
//! `identity.rs` remains responsible for `IrVersion`.
//!
//! `quantum::ir::qubit` remains responsible for logical and physical qubit
//! identity.
//!
//! This file must not acquire dependencies on those downstream modules.
//!
//! # Versioning
//!
//! The serialization format version and semantic IR version are deliberately
//! separate.
//!
//! ```text
//! FORMAT_VERSION
//!     !=
//! IrVersion
//!     !=
//! compiler version
//!     !=
//! Zamani language version
//!     !=
//! hardware version
//! ```
//!
//! A change to the byte framing is a serialization-format change.
//!
//! A change to IR semantics is an IR-version change.
//!
//! Compatibility/migration policy belongs in `serialization/compatibility.rs`.
//!
//! A future format version must never be silently decoded as the current
//! format.

#![forbid(unsafe_code)]

use std::convert::TryFrom;
use std::fmt;

use super::super::identity::IrVersion;
use super::super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Format constants
// =============================================================================

/// Four-byte canonical Zamani Quantum IR document marker.
pub const MAGIC: [u8; 4] = *b"ZQIR";

/// Current canonical serialization format version.
///
/// This is NOT the semantic IR version.
pub const FORMAT_VERSION: u16 = 1;

/// Number of bytes in the fixed document header.
///
/// Layout:
///
/// ```text
/// magic             4
/// format version   2
/// IR major         2
/// IR minor         2
/// IR patch         2
/// payload length   8
/// checksum         4
/// ------------------
/// total            24
/// ```
pub const HEADER_LEN: usize = 24;

/// Number of bytes used by the checksum.
pub const CHECKSUM_LEN: usize = 4;

/// Current checksum algorithm identifier.
///
/// The checksum is an integrity mechanism, not a cryptographic signature.
pub const CHECKSUM_ALGORITHM: ChecksumAlgorithm = ChecksumAlgorithm::Crc32c;

// =============================================================================
// Decode limits
// =============================================================================

/// Explicit resource policy for decoding serialized Quantum IR.
///
/// These limits are security/resource controls, NOT quantum-machine limits.
///
/// A caller compiling a larger program can supply a larger policy without
/// changing the IR schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeLimits {
    /// Maximum complete document size.
    pub max_document_bytes: u64,

    /// Maximum payload size.
    pub max_payload_bytes: u64,

    /// Maximum byte size of one byte/string field.
    pub max_field_bytes: u64,

    /// Maximum number of elements in one collection.
    pub max_collection_elements: u64,

    /// Maximum recursive nesting depth.
    pub max_nesting_depth: u64,
}

impl DecodeLimits {
    /// Creates an explicit decoding policy.
    #[must_use]
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

    /// Conservative general-purpose compiler policy.
    ///
    /// These are safety defaults only.
    ///
    /// They do NOT define the maximum size of Zamani programs or quantum
    /// machines.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            max_document_bytes: 256 * 1024 * 1024,
            max_payload_bytes: 256 * 1024 * 1024,
            max_field_bytes: 16 * 1024 * 1024,
            max_collection_elements: 16 * 1024 * 1024,
            max_nesting_depth: 4096,
        }
    }

    /// Default policy.
    #[must_use]
    pub const fn platform_default() -> Self {
        Self::conservative()
    }

    /// Validates that the policy itself is usable.
    pub fn validate(self) -> Result<(), CanonicalError> {
        if self.max_document_bytes == 0 {
            return Err(CanonicalError::InvalidDecodeLimits {
                field: "max_document_bytes",
            });
        }

        if self.max_payload_bytes == 0 {
            return Err(CanonicalError::InvalidDecodeLimits {
                field: "max_payload_bytes",
            });
        }

        if self.max_field_bytes == 0 {
            return Err(CanonicalError::InvalidDecodeLimits {
                field: "max_field_bytes",
            });
        }

        if self.max_collection_elements == 0 {
            return Err(CanonicalError::InvalidDecodeLimits {
                field: "max_collection_elements",
            });
        }

        if self.max_nesting_depth == 0 {
            return Err(CanonicalError::InvalidDecodeLimits {
                field: "max_nesting_depth",
            });
        }

        if self.max_payload_bytes > self.max_document_bytes {
            return Err(CanonicalError::InvalidDecodeLimits {
                field: "max_payload_bytes",
            });
        }

        Ok(())
    }

    fn check_document_size(self, size: u64) -> Result<(), CanonicalError> {
        if size > self.max_document_bytes {
            return Err(CanonicalError::DocumentTooLarge {
                size,
                maximum: self.max_document_bytes,
            });
        }

        Ok(())
    }

    fn check_payload_size(self, size: u64) -> Result<(), CanonicalError> {
        if size > self.max_payload_bytes {
            return Err(CanonicalError::PayloadTooLarge {
                size,
                maximum: self.max_payload_bytes,
            });
        }

        Ok(())
    }

    fn check_field_size(
        self,
        context: &'static str,
        size: u64,
    ) -> Result<(), CanonicalError> {
        if size > self.max_field_bytes {
            return Err(CanonicalError::FieldLimitExceeded {
                context,
                requested: size,
                maximum: self.max_field_bytes,
            });
        }

        Ok(())
    }

    fn check_collection_size(
        self,
        context: &'static str,
        size: u64,
    ) -> Result<(), CanonicalError> {
        if size > self.max_collection_elements {
            return Err(CanonicalError::CollectionLimitExceeded {
                context,
                requested: size,
                maximum: self.max_collection_elements,
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
// Errors
// =============================================================================

/// Structural/canonical serialization errors.
///
/// Semantic IR validation remains the responsibility of `validation.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalError {
    /// Decode policy is internally invalid.
    InvalidDecodeLimits {
        /// Name of the invalid policy field.
        field: &'static str,
    },

    /// Complete document exceeds policy.
    DocumentTooLarge {
        /// Actual document size.
        size: u64,

        /// Maximum allowed size.
        maximum: u64,
    },

    /// Payload exceeds policy.
    PayloadTooLarge {
        /// Actual payload size.
        size: u64,

        /// Maximum allowed size.
        maximum: u64,
    },

    /// Field exceeds policy.
    FieldLimitExceeded {
        /// Field being decoded.
        context: &'static str,

        /// Requested size.
        requested: u64,

        /// Maximum allowed size.
        maximum: u64,
    },

    /// Collection exceeds policy.
    CollectionLimitExceeded {
        /// Collection being decoded.
        context: &'static str,

        /// Requested number of elements.
        requested: u64,

        /// Maximum allowed number of elements.
        maximum: u64,
    },

    /// Nesting exceeds policy.
    NestingLimitExceeded {
        /// Requested depth.
        requested: u64,

        /// Maximum allowed depth.
        maximum: u64,
    },

    /// Input ended before a complete field was available.
    UnexpectedEnd {
        /// Number of bytes required.
        needed: usize,

        /// Number of bytes remaining.
        available: usize,
    },

    /// Wrong document magic.
    InvalidMagic {
        /// Four bytes actually found.
        found: [u8; 4],
    },

    /// Unsupported serialization format version.
    UnsupportedFormatVersion {
        /// Version found.
        version: u16,
    },

    /// Unsupported semantic IR version.
    UnsupportedIrVersion {
        /// Version found.
        version: IrVersion,
    },

    /// Canonical document has bytes after the declared payload.
    TrailingBytes {
        /// Number of trailing bytes.
        count: usize,
    },

    /// Payload checksum mismatch.
    ChecksumMismatch {
        /// Checksum stored in the document.
        expected: u32,

        /// Checksum computed from payload.
        actual: u32,
    },

    /// Wire length cannot be represented by the host.
    LengthOverflow {
        /// Conversion context.
        context: &'static str,

        /// Wire value.
        value: u64,
    },

    /// Arithmetic overflow.
    ArithmeticOverflow {
        /// Operation context.
        context: &'static str,
    },

    /// Boolean encoding is not canonical.
    InvalidBoolean {
        /// Invalid wire byte.
        value: u8,
    },

    /// Unknown discriminant.
    InvalidDiscriminant {
        /// Semantic type.
        type_name: &'static str,

        /// Wire discriminant.
        value: u64,
    },

    /// Invalid UTF-8.
    InvalidUtf8,

    /// Invalid semantic object supplied to canonical encoding.
    InvalidObject {
        /// Static reason.
        message: &'static str,
    },

    /// Invalid qubit identifier for the current host representation.
    QubitIdOverflow {
        /// Logical or physical identifier kind.
        kind: &'static str,

        /// Wire identifier.
        value: u64,
    },
}

impl fmt::Display for CanonicalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDecodeLimits { field } => {
                write!(formatter, "invalid decode limit `{field}`")
            }

            Self::DocumentTooLarge { size, maximum } => {
                write!(
                    formatter,
                    "document size {size} exceeds maximum {maximum}"
                )
            }

            Self::PayloadTooLarge { size, maximum } => {
                write!(
                    formatter,
                    "payload size {size} exceeds maximum {maximum}"
                )
            }

            Self::FieldLimitExceeded {
                context,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "{context} field size {requested} exceeds maximum {maximum}"
                )
            }

            Self::CollectionLimitExceeded {
                context,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "{context} collection size {requested} exceeds maximum {maximum}"
                )
            }

            Self::NestingLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "nesting depth {requested} exceeds maximum {maximum}"
                )
            }

            Self::UnexpectedEnd {
                needed,
                available,
            } => {
                write!(
                    formatter,
                    "unexpected end of canonical data: needed {needed} bytes, available {available}"
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
                    "unsupported canonical serialization format version {version}"
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
                    "canonical document contains {count} trailing bytes"
                )
            }

            Self::ChecksumMismatch { expected, actual } => {
                write!(
                    formatter,
                    "canonical payload checksum mismatch: expected {expected:#010x}, actual {actual:#010x}"
                )
            }

            Self::LengthOverflow { context, value } => {
                write!(
                    formatter,
                    "{context} length {value} cannot be represented by this host"
                )
            }

            Self::ArithmeticOverflow { context } => {
                write!(
                    formatter,
                    "canonical serialization arithmetic overflow while processing {context}"
                )
            }

            Self::InvalidBoolean { value } => {
                write!(
                    formatter,
                    "invalid canonical boolean byte {value:#04x}"
                )
            }

            Self::InvalidDiscriminant { type_name, value } => {
                write!(
                    formatter,
                    "invalid {type_name} discriminant {value}"
                )
            }

            Self::InvalidUtf8 => {
                formatter.write_str("invalid UTF-8 in canonical IR")
            }

            Self::InvalidObject { message } => {
                write!(formatter, "invalid canonical IR object: {message}")
            }

            Self::QubitIdOverflow { kind, value } => {
                write!(
                    formatter,
                    "{kind} identifier {value} cannot be represented by the current host"
                )
            }
        }
    }
}

impl std::error::Error for CanonicalError {}

// =============================================================================
// Checksum
// =============================================================================

/// Integrity checksum algorithm used by the canonical document envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAlgorithm {
    /// CRC-32C / Castagnoli.
    ///
    /// This is an integrity checksum, not a cryptographic authenticator.
    Crc32c,
}

impl ChecksumAlgorithm {
    /// Stable numeric identifier for the algorithm.
    #[must_use]
    pub const fn id(self) -> u8 {
        match self {
            Self::Crc32c => 1,
        }
    }
}

/// Computes CRC-32C/Castagnoli over the supplied bytes.
///
/// Polynomial in reversed representation: `0x82F63B78`.
///
/// This implementation deliberately avoids lookup tables so the algorithm is
/// fully self-contained and does not require a generated static table.
#[must_use]
pub fn checksum(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;

    for &byte in bytes {
        crc ^= byte as u32;

        let mut bit = 0u8;
        while bit < 8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0x82f6_3b78;
            } else {
                crc >>= 1;
            }

            bit += 1;
        }
    }

    !crc
}

// =============================================================================
// Canonical document
// =============================================================================

/// A validated borrowed canonical Quantum IR document.
///
/// The payload borrows directly from the original byte slice, avoiding an
/// additional allocation during structural decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalDocument<'a> {
    /// Serialization format version.
    format_version: u16,

    /// Semantic Quantum IR version.
    ir_version: IrVersion,

    /// Canonical payload.
    payload: &'a [u8],

    /// Stored payload checksum.
    checksum: u32,
}

impl<'a> CanonicalDocument<'a> {
    /// Returns the serialization format version.
    #[must_use]
    pub const fn format_version(self) -> u16 {
        self.format_version
    }

    /// Returns the semantic IR version.
    #[must_use]
    pub const fn ir_version(self) -> IrVersion {
        self.ir_version
    }

    /// Returns the borrowed canonical payload.
    #[must_use]
    pub const fn payload(self) -> &'a [u8] {
        self.payload
    }

    /// Returns the stored checksum.
    #[must_use]
    pub const fn checksum(self) -> u32 {
        self.checksum
    }

    /// Verifies the payload checksum again.
    ///
    /// Normally unnecessary because `decode_document()` already verifies it,
    /// but useful when callers retain the document as a long-lived borrowed
    /// value and want an explicit integrity check at a later boundary.
    pub fn verify_checksum(self) -> Result<(), CanonicalError> {
        let actual = checksum(self.payload);

        if actual != self.checksum {
            return Err(CanonicalError::ChecksumMismatch {
                expected: self.checksum,
                actual,
            });
        }

        Ok(())
    }
}

/// Serializes a complete canonical document envelope around an already
/// canonical payload.
///
/// This function does not interpret the payload.
///
/// The payload MUST already have been generated by the owning `IrEncode`
/// implementation in canonical semantic order.
pub fn encode_document(
    ir_version: IrVersion,
    payload: &[u8],
) -> Result<Vec<u8>, CanonicalError> {
    let payload_len = u64::try_from(payload.len()).map_err(|_| {
        CanonicalError::LengthOverflow {
            context: "payload",
            value: payload.len() as u64,
        }
    })?;

    let total_len = HEADER_LEN
        .checked_add(payload.len())
        .ok_or(CanonicalError::ArithmeticOverflow {
            context: "document length",
        })?;

    let mut output = Vec::new();

    output.extend_from_slice(&MAGIC);
    output.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    output.extend_from_slice(&ir_version.major().to_le_bytes());
    output.extend_from_slice(&ir_version.minor().to_le_bytes());
    output.extend_from_slice(&ir_version.patch().to_le_bytes());
    output.extend_from_slice(&payload_len.to_le_bytes());
    output.extend_from_slice(&checksum(payload).to_le_bytes());
    output.extend_from_slice(payload);

    debug_assert_eq!(output.len(), total_len);

    Ok(output)
}

/// Decodes a complete canonical document using the default resource policy.
pub fn decode_document(
    bytes: &[u8],
) -> Result<CanonicalDocument<'_>, CanonicalError> {
    decode_document_with_limits(bytes, DecodeLimits::default())
}

/// Decodes a canonical document with an explicit resource policy.
pub fn decode_document_with_limits(
    bytes: &[u8],
    limits: DecodeLimits,
) -> Result<CanonicalDocument<'_>, CanonicalError> {
    limits.validate()?;

    let document_len = u64::try_from(bytes.len()).map_err(|_| {
        CanonicalError::LengthOverflow {
            context: "document",
            value: u64::MAX,
        }
    })?;

    limits.check_document_size(document_len)?;

    if bytes.len() < HEADER_LEN {
        return Err(CanonicalError::UnexpectedEnd {
            needed: HEADER_LEN,
            available: bytes.len(),
        });
    }

    let magic = [bytes[0], bytes[1], bytes[2], bytes[3]];

    if magic != MAGIC {
        return Err(CanonicalError::InvalidMagic { found: magic });
    }

    let format_version = u16::from_le_bytes([bytes[4], bytes[5]]);

    if format_version != FORMAT_VERSION {
        return Err(CanonicalError::UnsupportedFormatVersion {
            version: format_version,
        });
    }

    let ir_version = IrVersion::new(
        u16::from_le_bytes([bytes[6], bytes[7]]),
        u16::from_le_bytes([bytes[8], bytes[9]]),
        u16::from_le_bytes([bytes[10], bytes[11]]),
    );

    if !ir_version.is_supported_by_current() {
        return Err(CanonicalError::UnsupportedIrVersion {
            version: ir_version,
        });
    }

    let payload_len_u64 = u64::from_le_bytes([
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15],
        bytes[16],
        bytes[17],
        bytes[18],
        bytes[19],
    ]);

    limits.check_payload_size(payload_len_u64)?;

    let payload_len = checked_usize(
        payload_len_u64,
        "payload",
    )?;

    let expected_document_len = HEADER_LEN
        .checked_add(payload_len)
        .ok_or(CanonicalError::ArithmeticOverflow {
            context: "document length",
        })?;

    if bytes.len() < expected_document_len {
        return Err(CanonicalError::UnexpectedEnd {
            needed: expected_document_len,
            available: bytes.len(),
        });
    }

    if bytes.len() > expected_document_len {
        return Err(CanonicalError::TrailingBytes {
            count: bytes.len() - expected_document_len,
        });
    }

    let expected_checksum = u32::from_le_bytes([
        bytes[20],
        bytes[21],
        bytes[22],
        bytes[23],
    ]);

    let payload = &bytes[HEADER_LEN..expected_document_len];

    let actual_checksum = checksum(payload);

    if actual_checksum != expected_checksum {
        return Err(CanonicalError::ChecksumMismatch {
            expected: expected_checksum,
            actual: actual_checksum,
        });
    }

    Ok(CanonicalDocument {
        format_version,
        ir_version,
        payload,
        checksum: expected_checksum,
    })
}

// =============================================================================
// Canonical writer
// =============================================================================

/// Low-level deterministic writer used by IR object encoders.
///
/// This type deliberately exposes only canonical primitive representations.
///
/// Semantic ordering belongs to the owning IR object.
#[derive(Debug, Default)]
pub struct CanonicalWriter {
    bytes: Vec<u8>,
}

impl CanonicalWriter {
    /// Creates an empty canonical writer.
    #[must_use]
    pub const fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Creates a writer with a caller-provided capacity.
    ///
    /// Capacity is an optimization only and has no semantic meaning.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of encoded bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the writer contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Appends one byte.
    pub fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    /// Appends a canonical boolean.
    ///
    /// `false = 0x00`, `true = 0x01`.
    pub fn write_bool(&mut self, value: bool) {
        self.write_u8(if value { 1 } else { 0 });
    }

    /// Appends little-endian `u16`.
    pub fn write_u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Appends little-endian `u32`.
    pub fn write_u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Appends little-endian `u64`.
    pub fn write_u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Appends little-endian `u128`.
    pub fn write_u128(&mut self, value: u128) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Appends little-endian signed `i8`.
    pub fn write_i8(&mut self, value: i8) {
        self.write_u8(value as u8);
    }

    /// Appends little-endian signed `i16`.
    pub fn write_i16(&mut self, value: i16) {
        self.write_u16(value as u16);
    }

    /// Appends little-endian signed `i32`.
    pub fn write_i32(&mut self, value: i32) {
        self.write_u32(value as u32);
    }

    /// Appends little-endian signed `i64`.
    pub fn write_i64(&mut self, value: i64) {
        self.write_u64(value as u64);
    }

    /// Appends little-endian signed `i128`.
    pub fn write_i128(&mut self, value: i128) {
        self.write_u128(value as u128);
    }

    /// Appends little-endian `f32`.
    ///
    /// Callers must canonicalize NaN semantics at the semantic type layer if
    /// their IR type treats multiple NaN bit patterns as equivalent.
    pub fn write_f32(&mut self, value: f32) {
        self.write_u32(value.to_bits());
    }

    /// Appends little-endian `f64`.
    ///
    /// Callers must canonicalize NaN semantics at the semantic type layer if
    /// their IR type treats multiple NaN bit patterns as equivalent.
    pub fn write_f64(&mut self, value: f64) {
        self.write_u64(value.to_bits());
    }

    /// Writes a length-prefixed byte sequence.
    ///
    /// Length is encoded as `u64`, never `usize`.
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), CanonicalError> {
        let length = u64::try_from(bytes.len()).map_err(|_| {
            CanonicalError::LengthOverflow {
                context: "byte field",
                value: u64::MAX,
            }
        })?;

        self.write_u64(length);
        self.bytes.extend_from_slice(bytes);

        Ok(())
    }

    /// Writes a length-prefixed UTF-8 string.
    pub fn write_str(&mut self, value: &str) -> Result<(), CanonicalError> {
        self.write_bytes(value.as_bytes())
    }

    /// Writes a canonical `QubitId`.
    ///
    /// The wire representation is always `u64`, even though the current
    /// in-memory `QubitId` implementation stores `usize`.
    pub fn write_qubit_id(&mut self, value: QubitId) {
        self.write_u64(value.index() as u64);
    }

    /// Writes a canonical `PhysicalQubitId`.
    pub fn write_physical_qubit_id(
        &mut self,
        value: PhysicalQubitId,
    ) {
        self.write_u64(value.index() as u64);
    }

    /// Appends an already-canonical byte sequence without modification.
    pub fn write_raw(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    /// Returns the canonical bytes.
    ///
    /// This consumes the writer so callers cannot accidentally mutate the
    /// already-finished canonical sequence through the writer.
    #[must_use]
    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }

    /// Borrows the currently accumulated canonical bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

// =============================================================================
// Canonical reader
// =============================================================================

/// Low-level bounded reader for canonical IR payloads.
///
/// All reads are checked before advancing the cursor.
#[derive(Debug)]
pub struct CanonicalReader<'a> {
    bytes: &'a [u8],
    offset: usize,
    limits: DecodeLimits,
    nesting_depth: u64,
}

impl<'a> CanonicalReader<'a> {
    /// Creates a reader using the default policy.
    pub fn new(bytes: &'a [u8]) -> Result<Self, CanonicalError> {
        Self::with_limits(bytes, DecodeLimits::default())
    }

    /// Creates a reader using explicit decoding limits.
    pub fn with_limits(
        bytes: &'a [u8],
        limits: DecodeLimits,
    ) -> Result<Self, CanonicalError> {
        limits.validate()?;

        let byte_len = u64::try_from(bytes.len()).map_err(|_| {
            CanonicalError::LengthOverflow {
                context: "reader payload",
                value: u64::MAX,
            }
        })?;

        limits.check_payload_size(byte_len)?;

        Ok(Self {
            bytes,
            offset: 0,
            limits,
            nesting_depth: 0,
        })
    }

    /// Returns the current byte offset.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the number of unread bytes.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.offset
    }

    /// Returns whether the reader has reached the end.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.offset == self.bytes.len()
    }

    /// Returns the configured decoding policy.
    #[must_use]
    pub const fn limits(&self) -> DecodeLimits {
        self.limits
    }

    /// Enters one recursive IR structure.
    ///
    /// The returned guard decreases the depth when dropped.
    pub fn enter(&mut self) -> Result<NestingGuard<'_>, CanonicalError> {
        let requested = self
            .nesting_depth
            .checked_add(1)
            .ok_or(CanonicalError::ArithmeticOverflow {
                context: "nesting depth",
            })?;

        if requested > self.limits.max_nesting_depth {
            return Err(CanonicalError::NestingLimitExceeded {
                requested,
                maximum: self.limits.max_nesting_depth,
            });
        }

        self.nesting_depth = requested;

        Ok(NestingGuard { reader: self })
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], CanonicalError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(CanonicalError::ArithmeticOverflow {
                context: "reader offset",
            })?;

        if end > self.bytes.len() {
            return Err(CanonicalError::UnexpectedEnd {
                needed: length,
                available: self.remaining(),
            });
        }

        let result = &self.bytes[self.offset..end];
        self.offset = end;

        Ok(result)
    }

    /// Reads one byte.
    pub fn read_u8(&mut self) -> Result<u8, CanonicalError> {
        Ok(self.take(1)?[0])
    }

    /// Reads a canonical boolean.
    pub fn read_bool(&mut self) -> Result<bool, CanonicalError> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(CanonicalError::InvalidBoolean { value }),
        }
    }

    /// Reads little-endian `u16`.
    pub fn read_u16(&mut self) -> Result<u16, CanonicalError> {
        let bytes = self.take(2)?;

        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    /// Reads little-endian `u32`.
    pub fn read_u32(&mut self) -> Result<u32, CanonicalError> {
        let bytes = self.take(4)?;

        Ok(u32::from_le_bytes([
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
        ]))
    }

    /// Reads little-endian `u64`.
    pub fn read_u64(&mut self) -> Result<u64, CanonicalError> {
        let bytes = self.take(8)?;

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

    /// Reads little-endian `u128`.
    pub fn read_u128(&mut self) -> Result<u128, CanonicalError> {
        let bytes = self.take(16)?;

        Ok(u128::from_le_bytes([
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
            bytes[8],
            bytes[9],
            bytes[10],
            bytes[11],
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15],
        ]))
    }

    /// Reads little-endian signed `i8`.
    pub fn read_i8(&mut self) -> Result<i8, CanonicalError> {
        Ok(self.read_u8()? as i8)
    }

    /// Reads little-endian signed `i16`.
    pub fn read_i16(&mut self) -> Result<i16, CanonicalError> {
        Ok(self.read_u16()? as i16)
    }

    /// Reads little-endian signed `i32`.
    pub fn read_i32(&mut self) -> Result<i32, CanonicalError> {
        Ok(self.read_u32()? as i32)
    }

    /// Reads little-endian signed `i64`.
    pub fn read_i64(&mut self) -> Result<i64, CanonicalError> {
        Ok(self.read_u64()? as i64)
    }

    /// Reads little-endian signed `i128`.
    pub fn read_i128(&mut self) -> Result<i128, CanonicalError> {
        Ok(self.read_u128()? as i128)
    }

    /// Reads little-endian `f32`.
    pub fn read_f32(&mut self) -> Result<f32, CanonicalError> {
        Ok(f32::from_bits(self.read_u32()?))
    }

    /// Reads little-endian `f64`.
    pub fn read_f64(&mut self) -> Result<f64, CanonicalError> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    /// Reads a borrowed length-prefixed byte sequence.
    ///
    /// No allocation occurs.
    pub fn read_bytes(
        &mut self,
        context: &'static str,
    ) -> Result<&'a [u8], CanonicalError> {
        let length = self.read_u64()?;

        self.limits.check_field_size(context, length)?;

        let length = checked_usize(length, context)?;

        self.take(length)
    }

    /// Reads a borrowed UTF-8 string.
    ///
    /// No allocation occurs.
    pub fn read_str(
        &mut self,
        context: &'static str,
    ) -> Result<&'a str, CanonicalError> {
        let bytes = self.read_bytes(context)?;

        std::str::from_utf8(bytes)
            .map_err(|_| CanonicalError::InvalidUtf8)
    }

    /// Reads a canonical logical-qubit identifier.
    pub fn read_qubit_id(&mut self) -> Result<QubitId, CanonicalError> {
        let raw = self.read_u64()?;

        decode_qubit_id(raw)
    }

    /// Reads a canonical physical-qubit identifier.
    pub fn read_physical_qubit_id(
        &mut self,
    ) -> Result<PhysicalQubitId, CanonicalError> {
        let raw = self.read_u64()?;

        decode_physical_qubit_id(raw)
    }

    /// Reads exactly `count` bytes without interpreting them.
    pub fn read_raw(
        &mut self,
        count: usize,
    ) -> Result<&'a [u8], CanonicalError> {
        self.take(count)
    }

    /// Requires that the complete payload has been consumed.
    pub fn finish(self) -> Result<(), CanonicalError> {
        if self.offset != self.bytes.len() {
            return Err(CanonicalError::TrailingBytes {
                count: self.bytes.len() - self.offset,
            });
        }

        Ok(())
    }
}

/// Guard returned by [`CanonicalReader::enter`].
pub struct NestingGuard<'a> {
    reader: &'a mut CanonicalReader<'a>,
}

impl<'a> Drop for NestingGuard<'a> {
    fn drop(&mut self) {
        self.reader.nesting_depth -= 1;
    }
}

// =============================================================================
// Checked conversions
// =============================================================================

/// Converts a wire `u64` length into host `usize` safely.
pub fn checked_usize(
    value: u64,
    context: &'static str,
) -> Result<usize, CanonicalError> {
    usize::try_from(value).map_err(|_| CanonicalError::LengthOverflow {
        context,
        value,
    })
}

/// Converts a host `usize` length into canonical wire `u64`.
pub fn checked_u64(
    value: usize,
    context: &'static str,
) -> Result<u64, CanonicalError> {
    u64::try_from(value).map_err(|_| CanonicalError::LengthOverflow {
        context,
        value: u64::MAX,
    })
}

// =============================================================================
// Qubit identity bridge
// =============================================================================

/// Encodes the canonical logical qubit identifier as its stable wire value.
///
/// This function deliberately uses the canonical `quantum::ir::qubit::QubitId`.
///
/// The current in-memory representation uses `usize`; the wire representation
/// is always `u64`.
#[must_use]
pub fn qubit_id_wire_value(value: QubitId) -> u64 {
    value.index() as u64
}

/// Encodes the canonical physical qubit identifier as its stable wire value.
#[must_use]
pub fn physical_qubit_id_wire_value(value: PhysicalQubitId) -> u64 {
    value.index() as u64
}

/// Decodes a canonical logical qubit identifier.
///
/// The conversion is checked because the current in-memory representation is
/// `usize`.
pub fn decode_qubit_id(
    value: u64,
) -> Result<QubitId, CanonicalError> {
    let index = usize::try_from(value).map_err(|_| {
        CanonicalError::QubitIdOverflow {
            kind: "logical qubit",
            value,
        }
    })?;

    Ok(QubitId::new(index))
}

/// Decodes a canonical physical qubit identifier.
pub fn decode_physical_qubit_id(
    value: u64,
) -> Result<PhysicalQubitId, CanonicalError> {
    let index = usize::try_from(value).map_err(|_| {
        CanonicalError::QubitIdOverflow {
            kind: "physical qubit",
            value,
        }
    })?;

    Ok(PhysicalQubitId::new(index))
}

// =============================================================================
// Canonical discriminant helpers
// =============================================================================

/// Writes a canonical enum discriminant.
pub fn write_discriminant(
    writer: &mut CanonicalWriter,
    value: u64,
) {
    writer.write_u64(value);
}

/// Reads a canonical enum discriminant.
pub fn read_discriminant(
    reader: &mut CanonicalReader<'_>,
) -> Result<u64, CanonicalError> {
    reader.read_u64()
}

/// Validates a discriminant against a known inclusive range.
///
/// This helper is intended for closed standard enums. Extensible dialect
/// identifiers should generally use namespace/name encoding instead.
pub fn require_discriminant(
    value: u64,
    type_name: &'static str,
    maximum: u64,
) -> Result<u64, CanonicalError> {
    if value > maximum {
        return Err(CanonicalError::InvalidDiscriminant {
            type_name,
            value,
        });
    }

    Ok(value)
}

// =============================================================================
// Canonical collection helpers
// =============================================================================

/// Writes a collection length using canonical `u64` representation.
pub fn write_collection_len(
    writer: &mut CanonicalWriter,
    length: usize,
    context: &'static str,
) -> Result<(), CanonicalError> {
    writer.write_u64(checked_u64(length, context)?);

    Ok(())
}

/// Reads and validates a collection length.
///
/// This function does not allocate.
pub fn read_collection_len(
    reader: &mut CanonicalReader<'_>,
    context: &'static str,
) -> Result<usize, CanonicalError> {
    let length = reader.read_u64()?;

    reader
        .limits
        .check_collection_size(context, length)?;

    checked_usize(length, context)
}

// =============================================================================
// Canonical text helpers
// =============================================================================

/// Returns whether the supplied string is already valid canonical UTF-8.
///
/// Rust `&str` is always valid UTF-8, so this function exists primarily as a
/// semantic documentation boundary for callers accepting borrowed text.
#[must_use]
pub const fn is_canonical_utf8(_value: &str) -> bool {
    true
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_is_deterministic() {
        let a = checksum(b"zamani");
        let b = checksum(b"zamani");

        assert_eq!(a, b);
    }

    #[test]
    fn checksum_changes_when_payload_changes() {
        assert_ne!(checksum(b"zamani"), checksum(b"Zamani"));
    }

    #[test]
    fn document_round_trip() {
        let payload = b"canonical-zamani-ir";
        let version = IrVersion::CURRENT;

        let bytes =
            encode_document(version, payload).expect("encode");

        let document =
            decode_document(&bytes).expect("decode");

        assert_eq!(document.format_version(), FORMAT_VERSION);
        assert_eq!(document.ir_version(), version);
        assert_eq!(document.payload(), payload);
        assert_eq!(document.checksum(), checksum(payload));
    }

    #[test]
    fn document_rejects_trailing_bytes() {
        let payload = b"payload";

        let mut bytes =
            encode_document(IrVersion::CURRENT, payload)
                .expect("encode");

        bytes.push(0);

        assert!(matches!(
            decode_document(&bytes),
            Err(CanonicalError::TrailingBytes { count: 1 })
        ));
    }

    #[test]
    fn document_rejects_bad_magic() {
        let mut bytes =
            encode_document(IrVersion::CURRENT, b"x")
                .expect("encode");

        bytes[0] = b'X';

        assert!(matches!(
            decode_document(&bytes),
            Err(CanonicalError::InvalidMagic { .. })
        ));
    }

    #[test]
    fn document_rejects_checksum_corruption() {
        let mut bytes =
            encode_document(IrVersion::CURRENT, b"x")
                .expect("encode");

        let payload_index = HEADER_LEN;
        bytes[payload_index] ^= 0xff;

        assert!(matches!(
            decode_document(&bytes),
            Err(CanonicalError::ChecksumMismatch { .. })
        ));
    }

    #[test]
    fn boolean_encoding_is_canonical() {
        let mut writer = CanonicalWriter::new();

        writer.write_bool(false);
        writer.write_bool(true);

        assert_eq!(writer.finish(), vec![0, 1]);
    }

    #[test]
    fn invalid_boolean_is_rejected() {
        let mut reader =
            CanonicalReader::new(&[2]).expect("reader");

        assert!(matches!(
            reader.read_bool(),
            Err(CanonicalError::InvalidBoolean { value: 2 })
        ));
    }

    #[test]
    fn primitive_round_trip() {
        let mut writer = CanonicalWriter::new();

        writer.write_u8(7);
        writer.write_u16(8);
        writer.write_u32(9);
        writer.write_u64(10);
        writer.write_u128(11);
        writer.write_i8(-12);
        writer.write_i16(-13);
        writer.write_i32(-14);
        writer.write_i64(-15);
        writer.write_i128(-16);
        writer.write_f32(17.0);
        writer.write_f64(18.0);

        let bytes = writer.finish();

        let mut reader =
            CanonicalReader::new(&bytes).expect("reader");

        assert_eq!(reader.read_u8().expect("u8"), 7);
        assert_eq!(reader.read_u16().expect("u16"), 8);
        assert_eq!(reader.read_u32().expect("u32"), 9);
        assert_eq!(reader.read_u64().expect("u64"), 10);
        assert_eq!(reader.read_u128().expect("u128"), 11);
        assert_eq!(reader.read_i8().expect("i8"), -12);
        assert_eq!(reader.read_i16().expect("i16"), -13);
        assert_eq!(reader.read_i32().expect("i32"), -14);
        assert_eq!(reader.read_i64().expect("i64"), -15);
        assert_eq!(reader.read_i128().expect("i128"), -16);
        assert_eq!(reader.read_f32().expect("f32"), 17.0);
        assert_eq!(reader.read_f64().expect("f64"), 18.0);

        reader.finish().expect("complete payload");
    }

    #[test]
    fn string_round_trip() {
        let mut writer = CanonicalWriter::new();

        writer
            .write_str("Zamani Quantum IR")
            .expect("write");

        let bytes = writer.finish();

        let mut reader =
            CanonicalReader::new(&bytes).expect("reader");

        assert_eq!(
            reader.read_str("test-string").expect("string"),
            "Zamani Quantum IR"
        );

        reader.finish().expect("complete");
    }

    #[test]
    fn byte_field_is_borrowed() {
        let original = b"abc";

        let mut writer = CanonicalWriter::new();

        writer.write_bytes(original).expect("write");

        let bytes = writer.finish();

        let mut reader =
            CanonicalReader::new(&bytes).expect("reader");

        assert_eq!(
            reader.read_bytes("bytes").expect("read"),
            original
        );
    }

    #[test]
    fn qubit_round_trip_uses_canonical_namespace() {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(11);

        let mut writer = CanonicalWriter::new();

        writer.write_qubit_id(logical);
        writer.write_physical_qubit_id(physical);

        let bytes = writer.finish();

        let mut reader =
            CanonicalReader::new(&bytes).expect("reader");

        assert_eq!(
            reader.read_qubit_id().expect("logical"),
            logical
        );

        assert_eq!(
            reader
                .read_physical_qubit_id()
                .expect("physical"),
            physical
        );

        reader.finish().expect("complete");
    }

    #[test]
    fn collection_limit_is_enforced_before_allocation() {
        let mut writer = CanonicalWriter::new();

        writer.write_u64(1_000);

        let bytes = writer.finish();

        let limits =
            DecodeLimits::new(1024, 1024, 1024, 10, 10);

        let mut reader =
            CanonicalReader::with_limits(&bytes, limits)
                .expect("reader");

        assert!(matches!(
            read_collection_len(&mut reader, "test"),
            Err(CanonicalError::CollectionLimitExceeded {
                requested: 1_000,
                maximum: 10,
                ..
            })
        ));
    }

    #[test]
    fn field_limit_is_enforced_before_read() {
        let mut writer = CanonicalWriter::new();

        writer.write_u64(100);

        let bytes = writer.finish();

        let limits =
            DecodeLimits::new(1024, 1024, 10, 100, 10);

        let mut reader =
            CanonicalReader::with_limits(&bytes, limits)
                .expect("reader");

        assert!(matches!(
            reader.read_bytes("test"),
            Err(CanonicalError::FieldLimitExceeded {
                requested: 100,
                maximum: 10,
                ..
            })
        ));
    }

    #[test]
    fn nesting_limit_is_enforced() {
        let limits =
            DecodeLimits::new(1024, 1024, 1024, 100, 1);

        let mut reader =
            CanonicalReader::with_limits(&[], limits)
                .expect("reader");

        let first = reader.enter().expect("first");

        let second = reader.enter();

        assert!(matches!(
            second,
            Err(CanonicalError::NestingLimitExceeded {
                requested: 2,
                maximum: 1
            })
        ));

        drop(first);
    }

    #[test]
    fn checked_usize_rejects_unrepresentable_values() {
        #[cfg(target_pointer_width = "32")]
        {
            assert!(checked_usize(
                u64::from(u32::MAX) + 1,
                "test"
            )
            .is_err());
        }

        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(
                checked_usize(42, "test").expect("convert"),
                42
            );
        }
    }

    #[test]
    fn canonical_writer_is_deterministic() {
        let mut first = CanonicalWriter::new();
        first.write_u64(1);
        first.write_u64(2);
        first.write_u64(3);

        let mut second = CanonicalWriter::new();
        second.write_u64(1);
        second.write_u64(2);
        second.write_u64(3);

        assert_eq!(first.finish(), second.finish());
    }
}