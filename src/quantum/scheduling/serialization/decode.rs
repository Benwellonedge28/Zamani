//! Zamani Quantum Scheduling — Canonical Schedule Decoder
//!
//! This module owns the serialization *decoding mechanism* for quantum
//! scheduling artifacts.
//!
//! # Architectural responsibility
//!
//! This file answers:
//!
//! > "How do bytes become a validated, integrity-checked scheduling
//! > representation without executing anything?"
//!
//! It owns:
//!
//! - binary framing validation;
//! - format-version validation;
//! - payload-length validation;
//! - optional caller-supplied payload bounds;
//! - SHA-256 integrity verification;
//! - UTF-8 JSON parsing through `serde_json`;
//! - optional canonical-JSON validation;
//! - generic Serde deserialization;
//! - reader-based decoding;
//! - deterministic validation of framing;
//! - explicit decode diagnostics.
//!
//! It does NOT own:
//!
//! - scheduling semantics;
//! - schedule construction;
//! - dependency analysis;
//! - resource allocation;
//! - routing;
//! - hardware discovery;
//! - QEC;
//! - runtime execution;
//! - authentication;
//! - authorization;
//! - encryption;
//! - compression;
//! - schema migration;
//! - scheduler algorithms.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! canonical scheduling artifact
//!          │
//!          ▼
//! serialization::decode       ← THIS FILE
//!          │
//!          ├── framing validation
//!          ├── version validation
//!          ├── length validation
//!          ├── SHA-256 verification
//!          ├── canonical JSON validation
//!          └── Serde deserialization
//!          │
//!          ▼
//! serialization::schema / caller-owned type
//!          │
//!          ▼
//! validated scheduling representation
//! ```
//!
//! # Critical security boundary
//!
//! Decoding is not execution.
//!
//! A decoded scheduling document MUST NOT:
//!
//! - contact a QPU;
//! - invoke a backend;
//! - invoke a process;
//! - perform filesystem execution;
//! - mutate global scheduler state;
//! - execute quantum operations;
//! - execute classical feedback;
//! - perform hardware discovery.
//!
//! The resulting value is data only.
//!
//! A higher-level integration layer is responsible for:
//!
//! 1. schema/domain validation;
//! 2. target compatibility validation;
//! 3. resource validation;
//! 4. semantic validation;
//! 5. authorization;
//! 6. only then execution.
//!
//! # Binary framing
//!
//! `encode.rs` defines the current binary frame as:
//!
//! ```text
//! offset  size    field
//! ------  ------  ----------------
//! 0       8       magic
//! 8       2       format major
//! 10      2       format minor
//! 12      8       payload length
//! 20      32      SHA-256(payload)
//! 52      N       canonical JSON payload
//! ```
//!
//! All integer framing fields are little-endian.
//!
//! The decoder never trusts a payload length merely because it fits in the
//! serialized header.
//!
//! A caller may additionally provide an explicit maximum payload size.
//! There is intentionally no scheduler-wide hard-coded maximum.
//!
//! # Scalability
//!
//! This module contains no limits on:
//!
//! - qubit count;
//! - operation count;
//! - resource count;
//! - schedule depth;
//! - dependency count;
//! - reservation count;
//! - QEC rounds;
//! - distributed nodes;
//! - communication links.
//!
//! The only fixed sizes are properties of the serialization format itself:
//!
//! - the magic field;
//! - version fields;
//! - the length field;
//! - the SHA-256 digest.
//!
//! A caller may impose resource/security limits through `DecodeOptions`.
//!
//! Those limits are invocation policy, not Zamani architectural limits.
//!
//! # Canonical JSON
//!
//! The encoder canonicalizes JSON object keys while preserving array order.
//!
//! When `require_canonical_json` is enabled, this decoder verifies that the
//! payload is already in canonical JSON representation before deserializing it.
//!
//! This prevents accepting multiple byte representations of the same JSON
//! object when deterministic artifacts are required.
//!
//! It also rejects duplicate JSON object keys because canonicalization through
//! `serde_json::Value` produces one semantic representation.
//!
//! # Integrity
//!
//! SHA-256 verifies:
//!
//! ```text
//! bytes received == bytes encoded
//! ```
//!
//! It does NOT establish:
//!
//! - authenticity;
//! - authorship;
//! - authorization;
//! - trustworthiness;
//! - hardware compatibility.
//!
//! Digital signatures and trust policy belong to a higher security boundary.
//!
//! # Versioning
//!
//! Binary format compatibility is separate from scheduling schema compatibility.
//!
//! ```text
//! binary format version
//!         ≠
//! scheduling schema version
//!         ≠
//! Zamani language version
//!         ≠
//! Quantum IR version
//!         ≠
//! hardware version
//! ```
//!
//! This decoder validates the binary format version.
//!
//! The generic decoded object remains responsible for its own schema/domain
//! validation.
//!
//! # Canonical quantum identity
//!
//! This file does not define qubit identities.
//!
//! Any decoded scheduling schema containing qubits must ultimately resolve them
//! through the canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No decoder-local `QubitId` or `PhysicalQubitId` exists here.
//!
//! # Rust contract
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
//! # Integration contract
//!
//! ```text
//! serialization/schema.rs
//!          │
//!          ▼
//! serialization/encode.rs
//!          │
//!          ▼
//! serialized bytes
//!          │
//!          ▼
//! serialization/decode.rs       ← THIS FILE
//!          │
//!          ▼
//! DeserializeOwned target type
//!          │
//!          ▼
//! scheduling/domain validation
//!          │
//!          ▼
//! routing / hardware / QEC / runtime
//! ```
//!
//! `decode.rs` intentionally does not import scheduler algorithms.
//!
//! This keeps the serialization boundary stable when scheduling algorithms are
//! added, removed, optimized, or replaced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::io::{self, Read};

use serde::de::DeserializeOwned;
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::schema::{
    DIGEST_LENGTH,
    FORMAT_MAGIC,
    FORMAT_VERSION_MAJOR,
    FORMAT_VERSION_MINOR,
    HEADER_LENGTH,
};

// =============================================================================
// Decode options
// =============================================================================

/// Configuration controlling schedule-artifact decoding.
///
/// This structure contains only decoding/security policy.
///
/// It does not impose any Zamani-wide machine-size limit.
///
/// # Security
///
/// For untrusted input, callers SHOULD provide an explicit
/// `max_payload_bytes`.
///
/// For trusted local artifacts, `max_payload_bytes = None` permits the
/// representable payload size subject to available resources.
///
/// No arbitrary scheduler-specific default limit is hidden here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeOptions {
    /// Optional maximum accepted JSON payload size in bytes.
    ///
    /// `None` means no decoder-imposed payload limit.
    ///
    /// This is an invocation policy, not a quantum-machine limit.
    pub max_payload_bytes: Option<u64>,

    /// Require the payload to already be in canonical JSON representation.
    ///
    /// Enabled by default because scheduling artifacts are intended to be
    /// reproducible and content-addressable.
    pub require_canonical_json: bool,

    /// Permit a future binary-format minor version.
    ///
    /// This should only be enabled when the caller has independently
    /// established that the future minor version is backward-compatible with
    /// the decoder.
    ///
    /// Disabled by default for conservative compatibility behavior.
    pub allow_future_format_minor: bool,

    /// Reject bytes following the framed payload.
    ///
    /// Enabled by default because a complete scheduling artifact is expected
    /// to occupy exactly one frame.
    pub reject_trailing_bytes: bool,
}

impl Default for DecodeOptions {
    fn default() -> Self {
        Self {
            max_payload_bytes: None,
            require_canonical_json: true,
            allow_future_format_minor: false,
            reject_trailing_bytes: true,
        }
    }
}

impl DecodeOptions {
    /// Creates conservative production defaults.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_payload_bytes: None,
            require_canonical_json: true,
            allow_future_format_minor: false,
            reject_trailing_bytes: true,
        }
    }

    /// Creates options suitable for decoding a trusted local artifact.
    #[must_use]
    pub const fn trusted() -> Self {
        Self {
            max_payload_bytes: None,
            require_canonical_json: true,
            allow_future_format_minor: false,
            reject_trailing_bytes: true,
        }
    }

    /// Creates options suitable for a caller that explicitly controls the
    /// maximum serialized payload size.
    #[must_use]
    pub const fn with_max_payload_bytes(max_payload_bytes: u64) -> Self {
        Self {
            max_payload_bytes: Some(max_payload_bytes),
            require_canonical_json: true,
            allow_future_format_minor: false,
            reject_trailing_bytes: true,
        }
    }

    /// Returns a copy with the payload limit changed.
    #[must_use]
    pub const fn with_payload_limit(self, max_payload_bytes: Option<u64>) -> Self {
        Self {
            max_payload_bytes,
            ..self
        }
    }

    /// Returns a copy with canonical JSON enforcement changed.
    #[must_use]
    pub const fn with_canonical_json_required(self, required: bool) -> Self {
        Self {
            require_canonical_json: required,
            ..self
        }
    }

    /// Returns a copy with future format-minor acceptance changed.
    #[must_use]
    pub const fn with_future_format_minor_allowed(self, allowed: bool) -> Self {
        Self {
            allow_future_format_minor: allowed,
            ..self
        }
    }

    /// Returns a copy with trailing-byte handling changed.
    #[must_use]
    pub const fn with_trailing_bytes_rejected(self, rejected: bool) -> Self {
        Self {
            reject_trailing_bytes: rejected,
            ..self
        }
    }
}

// =============================================================================
// Frame metadata
// =============================================================================

/// Metadata extracted from a valid binary scheduling frame.
///
/// This is intentionally independent from the decoded scheduling schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameMetadata {
    /// Binary format major version.
    pub format_major: u16,

    /// Binary format minor version.
    pub format_minor: u16,

    /// Canonical JSON payload length in bytes.
    pub payload_length: u64,

    /// SHA-256 digest stored in the frame.
    pub payload_digest: [u8; DIGEST_LENGTH],
}

impl FrameMetadata {
    /// Returns the number of bytes occupied by the complete frame.
    ///
    /// Returns `None` if the value cannot be represented as `usize`.
    #[must_use]
    pub fn total_length(self) -> Option<usize> {
        let header = HEADER_LENGTH as u64;

        header
            .checked_add(self.payload_length)
            .and_then(|value| usize::try_from(value).ok())
    }

    /// Returns the stored digest as lowercase hexadecimal.
    #[must_use]
    pub fn digest_hex(self) -> String {
        bytes_to_lower_hex(&self.payload_digest)
    }
}

// =============================================================================
// Decoded artifact
// =============================================================================

/// A successfully decoded scheduling artifact.
///
/// The frame metadata and decoded value are returned together so callers do
/// not have to inspect the serialized representation again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedArtifact<T> {
    /// Decoded scheduling/schema value.
    pub value: T,

    /// Validated binary frame metadata.
    pub frame: FrameMetadata,
}

impl<T> DecodedArtifact<T> {
    /// Creates a decoded artifact.
    #[must_use]
    pub const fn new(value: T, frame: FrameMetadata) -> Self {
        Self { value, frame }
    }

    /// Decomposes the artifact into its decoded value and frame metadata.
    #[must_use]
    pub fn into_parts(self) -> (T, FrameMetadata) {
        (self.value, self.frame)
    }

    /// Returns a reference to the decoded value.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Returns the frame metadata.
    #[must_use]
    pub const fn frame(&self) -> FrameMetadata {
        self.frame
    }
}

// =============================================================================
// Decode errors
// =============================================================================

/// Errors produced while decoding a scheduling serialization artifact.
///
/// These errors describe serialization/framing failures. Domain-specific
/// semantic validation belongs to the scheduling schema and consuming
/// subsystems.
#[derive(Debug)]
pub enum DecodeError {
    /// The input is shorter than the fixed binary header.
    TruncatedHeader {
        /// Number of bytes actually available.
        actual: usize,

        /// Number of bytes required for the header.
        required: usize,
    },

    /// The binary magic does not identify a Zamani scheduling artifact.
    InvalidMagic {
        /// Bytes encountered at the beginning of the input.
        actual: [u8; FORMAT_MAGIC.len()],
    },

    /// The binary format major version is unsupported.
    UnsupportedFormatMajor {
        /// Decoder-supported major version.
        expected: u16,

        /// Encountered major version.
        actual: u16,
    },

    /// A future minor version was encountered and the caller has not
    /// explicitly allowed it.
    UnsupportedFutureFormatMinor {
        /// Decoder-supported minor version.
        supported: u16,

        /// Encountered minor version.
        actual: u16,
    },

    /// The payload length cannot be represented by the host address space.
    PayloadLengthOverflow {
        /// Length declared in the wire format.
        length: u64,
    },

    /// The payload exceeds the caller-provided decoding limit.
    PayloadLimitExceeded {
        /// Declared payload length.
        length: u64,

        /// Caller-provided maximum.
        maximum: u64,
    },

    /// The frame declares more payload bytes than were supplied.
    TruncatedPayload {
        /// Declared payload length.
        expected: u64,

        /// Number of payload bytes actually supplied.
        actual: u64,
    },

    /// Additional bytes followed a complete frame.
    TrailingBytes {
        /// Number of trailing bytes observed when known.
        count: usize,
    },

    /// The payload's SHA-256 digest does not match the frame.
    DigestMismatch {
        /// Digest stored in the frame.
        expected: [u8; DIGEST_LENGTH],

        /// Digest calculated over the received payload.
        actual: [u8; DIGEST_LENGTH],
    },

    /// The payload is not valid JSON.
    Json(serde_json::Error),

    /// The JSON is valid but is not in canonical representation.
    NonCanonicalJson,

    /// The JSON representation could not be converted to the requested Rust
    /// type.
    Deserialize(serde_json::Error),

    /// Reading from an external source failed.
    Io(io::Error),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TruncatedHeader { actual, required } => write!(
                formatter,
                "truncated Zamani scheduling frame header: received {actual} bytes, required {required}"
            ),

            Self::InvalidMagic { actual } => write!(
                formatter,
                "invalid Zamani scheduling frame magic: received {:?}",
                actual
            ),

            Self::UnsupportedFormatMajor { expected, actual } => write!(
                formatter,
                "unsupported Zamani scheduling binary format major version: expected {expected}, received {actual}"
            ),

            Self::UnsupportedFutureFormatMinor { supported, actual } => write!(
                formatter,
                "unsupported future Zamani scheduling binary format minor version: decoder supports {supported}, received {actual}"
            ),

            Self::PayloadLengthOverflow { length } => write!(
                formatter,
                "scheduling payload length {length} cannot be represented by the host address space"
            ),

            Self::PayloadLimitExceeded { length, maximum } => write!(
                formatter,
                "scheduling payload length {length} exceeds caller-provided decoding limit {maximum}"
            ),

            Self::TruncatedPayload { expected, actual } => write!(
                formatter,
                "truncated scheduling payload: expected {expected} bytes, received {actual}"
            ),

            Self::TrailingBytes { count } => write!(
                formatter,
                "unexpected trailing bytes after scheduling frame: {count}"
            ),

            Self::DigestMismatch { expected, actual } => write!(
                formatter,
                "scheduling payload SHA-256 mismatch: expected {}, received {}",
                bytes_to_lower_hex(expected),
                bytes_to_lower_hex(actual)
            ),

            Self::Json(error) => {
                write!(formatter, "invalid scheduling JSON payload: {error}")
            }

            Self::NonCanonicalJson => write!(
                formatter,
                "scheduling JSON payload is valid JSON but is not in canonical representation"
            ),

            Self::Deserialize(error) => write!(
                formatter,
                "failed to deserialize scheduling JSON into the requested representation: {error}"
            ),

            Self::Io(error) => {
                write!(formatter, "failed to read scheduling serialization artifact: {error}")
            }
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) | Self::Deserialize(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::TruncatedHeader { .. }
            | Self::InvalidMagic { .. }
            | Self::UnsupportedFormatMajor { .. }
            | Self::UnsupportedFutureFormatMinor { .. }
            | Self::PayloadLengthOverflow { .. }
            | Self::PayloadLimitExceeded { .. }
            | Self::TruncatedPayload { .. }
            | Self::TrailingBytes { .. }
            | Self::DigestMismatch { .. }
            | Self::NonCanonicalJson => None,
        }
    }
}

impl From<io::Error> for DecodeError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

// =============================================================================
// Public frame inspection
// =============================================================================

/// Reads and validates only the binary frame header.
///
/// This function does not deserialize the JSON payload.
///
/// It is useful when callers need to inspect the payload size and digest before
/// committing to payload allocation.
///
/// # Errors
///
/// Returns an error when the input does not contain a complete, valid header.
pub fn inspect_frame(bytes: &[u8]) -> Result<FrameMetadata, DecodeError> {
    let header = bytes.get(..HEADER_LENGTH).ok_or(DecodeError::TruncatedHeader {
        actual: bytes.len(),
        required: HEADER_LENGTH,
    })?;

    parse_header(header)
}

// =============================================================================
// Public framed decoder
// =============================================================================

/// Decodes a complete framed scheduling artifact.
///
/// This is the normal production entry point.
///
/// The function:
///
/// 1. validates the fixed frame header;
/// 2. validates the format version;
/// 3. validates the declared payload size;
/// 4. extracts the exact payload;
/// 5. rejects unexpected trailing bytes when configured;
/// 6. verifies SHA-256;
/// 7. validates canonical JSON when configured;
/// 8. deserializes the JSON into `T`.
///
/// No scheduling operation is executed.
pub fn decode<T>(bytes: &[u8]) -> Result<DecodedArtifact<T>, DecodeError>
where
    T: DeserializeOwned,
{
    decode_with_options(bytes, DecodeOptions::production())
}

/// Decodes a complete framed scheduling artifact using explicit options.
pub fn decode_with_options<T>(
    bytes: &[u8],
    options: DecodeOptions,
) -> Result<DecodedArtifact<T>, DecodeError>
where
    T: DeserializeOwned,
{
    let frame = inspect_frame(bytes)?;

    validate_payload_limit(frame.payload_length, options.max_payload_bytes)?;

    let payload_length = usize::try_from(frame.payload_length)
        .map_err(|_| DecodeError::PayloadLengthOverflow {
            length: frame.payload_length,
        })?;

    let payload_start = HEADER_LENGTH;

    let payload_end = payload_start
        .checked_add(payload_length)
        .ok_or(DecodeError::PayloadLengthOverflow {
            length: frame.payload_length,
        })?;

    if bytes.len() < payload_end {
        return Err(DecodeError::TruncatedPayload {
            expected: frame.payload_length,
            actual: bytes.len().saturating_sub(payload_start) as u64,
        });
    }

    if options.reject_trailing_bytes && bytes.len() != payload_end {
        return Err(DecodeError::TrailingBytes {
            count: bytes.len() - payload_end,
        });
    }

    let payload = &bytes[payload_start..payload_end];

    verify_digest(payload, &frame)?;

    let value = deserialize_payload(payload, options.require_canonical_json)?;

    Ok(DecodedArtifact::new(value, frame))
}

// =============================================================================
// Public payload-only decoder
// =============================================================================

/// Decodes canonical JSON without a Zamani binary frame.
///
/// This is intentionally separate from [`decode`] so callers cannot
/// accidentally confuse raw JSON with an integrity-protected scheduling
/// artifact.
///
/// No SHA-256 verification is performed because there is no binary frame
/// containing a trusted digest.
///
/// Use [`decode`] for normal persisted/transported scheduling artifacts.
pub fn decode_canonical_json<T>(payload: &[u8]) -> Result<T, DecodeError>
where
    T: DeserializeOwned,
{
    deserialize_payload(payload, true)
}

/// Decodes JSON without requiring canonical byte representation.
///
/// This function is useful for compatibility/interchange inputs that are known
/// to be outside the canonical scheduling artifact format.
///
/// It should not normally be used for reproducibility-sensitive scheduling
/// artifacts.
pub fn decode_json<T>(payload: &[u8]) -> Result<T, DecodeError>
where
    T: DeserializeOwned,
{
    deserialize_payload(payload, false)
}

// =============================================================================
// Public reader-based decoder
// =============================================================================

/// Decodes one complete framed scheduling artifact from a reader.
///
/// The reader is consumed until the complete frame has been read.
///
/// When `reject_trailing_bytes` is enabled, this function performs one
/// additional read after the frame to detect whether another byte follows the
/// frame.
///
/// For network protocols where waiting for EOF is inappropriate, callers
/// should use `reject_trailing_bytes = false` and rely on the framing length.
pub fn decode_from_reader<R, T>(
    reader: &mut R,
) -> Result<DecodedArtifact<T>, DecodeError>
where
    R: Read,
    T: DeserializeOwned,
{
    decode_from_reader_with_options(reader, DecodeOptions::production())
}

/// Decodes one complete framed scheduling artifact from a reader using explicit
/// options.
pub fn decode_from_reader_with_options<R, T>(
    reader: &mut R,
    options: DecodeOptions,
) -> Result<DecodedArtifact<T>, DecodeError>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut header = [0_u8; HEADER_LENGTH];

    read_exact_header(reader, &mut header)?;

    let frame = parse_header(&header)?;

    validate_payload_limit(frame.payload_length, options.max_payload_bytes)?;

    let payload_length = usize::try_from(frame.payload_length)
        .map_err(|_| DecodeError::PayloadLengthOverflow {
            length: frame.payload_length,
        })?;

    let mut payload = Vec::new();

    /*
     * Reserve only the exact declared payload size.
     *
     * There is deliberately no fixed scheduler limit here. The caller may
     * provide `max_payload_bytes` above when decoding untrusted data.
     */
    payload
        .try_reserve_exact(payload_length)
        .map_err(|_| DecodeError::PayloadLengthOverflow {
            length: frame.payload_length,
        })?;

    payload.resize(payload_length, 0_u8);

    if let Err(error) = reader.read_exact(&mut payload) {
        if error.kind() == io::ErrorKind::UnexpectedEof {
            return Err(DecodeError::TruncatedPayload {
                expected: frame.payload_length,
                actual: 0,
            });
        }

        return Err(DecodeError::Io(error));
    }

    verify_digest(&payload, &frame)?;

    let value = deserialize_payload(&payload, options.require_canonical_json)?;

    if options.reject_trailing_bytes {
        let mut trailing = [0_u8; 1];

        match reader.read(&mut trailing) {
            Ok(0) => {}
            Ok(_) => {
                return Err(DecodeError::TrailingBytes { count: 1 });
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                /*
                 * Do not silently treat Interrupted as EOF.
                 *
                 * A higher-level streaming protocol should decide how to
                 * retry interrupted reads.
                 */
                return Err(DecodeError::Io(error));
            }
            Err(error) => return Err(DecodeError::Io(error)),
        }
    }

    Ok(DecodedArtifact::new(value, frame))
}

// =============================================================================
// Header parsing
// =============================================================================

/// Parses a complete fixed-size binary scheduling header.
///
/// This function performs no allocation.
fn parse_header(header: &[u8]) -> Result<FrameMetadata, DecodeError> {
    if header.len() < HEADER_LENGTH {
        return Err(DecodeError::TruncatedHeader {
            actual: header.len(),
            required: HEADER_LENGTH,
        });
    }

    let magic = header
        .get(..FORMAT_MAGIC.len())
        .ok_or(DecodeError::TruncatedHeader {
            actual: header.len(),
            required: HEADER_LENGTH,
        })?;

    let mut actual_magic = [0_u8; FORMAT_MAGIC.len()];
    actual_magic.copy_from_slice(magic);

    if actual_magic != FORMAT_MAGIC {
        return Err(DecodeError::InvalidMagic {
            actual: actual_magic,
        });
    }

    let format_major = read_u16_le(header, 8)?;
    let format_minor = read_u16_le(header, 10)?;
    let payload_length = read_u64_le(header, 12)?;

    if format_major != FORMAT_VERSION_MAJOR {
        return Err(DecodeError::UnsupportedFormatMajor {
            expected: FORMAT_VERSION_MAJOR,
            actual: format_major,
        });
    }

    /*
     * A future minor format may be compatible, but it MUST NOT be silently
     * accepted unless the caller explicitly opts in.
     *
     * `parse_header` itself has no DecodeOptions, so future-minor checking is
     * completed by `validate_format_version`.
     */
    let mut payload_digest = [0_u8; DIGEST_LENGTH];

    let digest_start = 20;
    let digest_end = digest_start + DIGEST_LENGTH;

    payload_digest.copy_from_slice(
        header
            .get(digest_start..digest_end)
            .ok_or(DecodeError::TruncatedHeader {
                actual: header.len(),
                required: HEADER_LENGTH,
            })?,
    );

    Ok(FrameMetadata {
        format_major,
        format_minor,
        payload_length,
        payload_digest,
    })
}

// =============================================================================
// Format-version validation
// =============================================================================

/// Validates a parsed frame against the decoder's supported format version.
fn validate_format_version(
    frame: &FrameMetadata,
    options: DecodeOptions,
) -> Result<(), DecodeError> {
    if frame.format_major != FORMAT_VERSION_MAJOR {
        return Err(DecodeError::UnsupportedFormatMajor {
            expected: FORMAT_VERSION_MAJOR,
            actual: frame.format_major,
        });
    }

    if frame.format_minor > FORMAT_VERSION_MINOR
        && !options.allow_future_format_minor
    {
        return Err(DecodeError::UnsupportedFutureFormatMinor {
            supported: FORMAT_VERSION_MINOR,
            actual: frame.format_minor,
        });
    }

    Ok(())
}

// =============================================================================
// Payload limit validation
// =============================================================================

fn validate_payload_limit(
    payload_length: u64,
    maximum: Option<u64>,
) -> Result<(), DecodeError> {
    if let Some(maximum) = maximum {
        if payload_length > maximum {
            return Err(DecodeError::PayloadLimitExceeded {
                length: payload_length,
                maximum,
            });
        }
    }

    Ok(())
}

// =============================================================================
// Digest validation
// =============================================================================

fn verify_digest(
    payload: &[u8],
    frame: &FrameMetadata,
) -> Result<(), DecodeError> {
    let actual = sha256(payload);

    if actual != frame.payload_digest {
        return Err(DecodeError::DigestMismatch {
            expected: frame.payload_digest,
            actual,
        });
    }

    Ok(())
}

// =============================================================================
// JSON deserialization
// =============================================================================

fn deserialize_payload<T>(
    payload: &[u8],
    require_canonical_json: bool,
) -> Result<T, DecodeError>
where
    T: DeserializeOwned,
{
    /*
     * Parse to `Value` first rather than directly into T when canonical
     * validation is requested.
     *
     * This provides a concrete representation against which the canonical
     * encoding can be compared.
     */
    if require_canonical_json {
        let value: Value =
            serde_json::from_slice(payload).map_err(DecodeError::Json)?;

        let canonical =
            serde_json::to_vec(&value).map_err(DecodeError::Json)?;

        if canonical.as_slice() != payload {
            return Err(DecodeError::NonCanonicalJson);
        }

        /*
         * Deserialize the canonical value into the requested domain type.
         *
         * Serializing `Value` again is intentionally avoided. The parsed value
         * is already owned and can be converted directly through Serde.
         */
        serde_json::from_value(value).map_err(DecodeError::Deserialize)
    } else {
        serde_json::from_slice(payload).map_err(DecodeError::Deserialize)
    }
}

// =============================================================================
// Exact header reading
// =============================================================================

fn read_exact_header<R>(
    reader: &mut R,
    header: &mut [u8; HEADER_LENGTH],
) -> Result<(), DecodeError>
where
    R: Read,
{
    let mut offset = 0_usize;

    while offset < header.len() {
        match reader.read(&mut header[offset..]) {
            Ok(0) => {
                return Err(DecodeError::TruncatedHeader {
                    actual: offset,
                    required: HEADER_LENGTH,
                });
            }

            Ok(read) => {
                offset = offset
                    .checked_add(read)
                    .ok_or(DecodeError::TruncatedHeader {
                        actual: offset,
                        required: HEADER_LENGTH,
                    })?;
            }

            Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                /*
                 * `Read` explicitly permits interruption.
                 *
                 * Retrying here is correct and prevents transient EINTR-like
                 * conditions from becoming false corruption reports.
                 */
                continue;
            }

            Err(error) => return Err(DecodeError::Io(error)),
        }
    }

    Ok(())
}

// =============================================================================
// Checked integer readers
// =============================================================================

fn read_u16_le(bytes: &[u8], offset: usize) -> Result<u16, DecodeError> {
    let end = offset
        .checked_add(2)
        .ok_or(DecodeError::TruncatedHeader {
            actual: bytes.len(),
            required: HEADER_LENGTH,
        })?;

    let field = bytes.get(offset..end).ok_or(DecodeError::TruncatedHeader {
        actual: bytes.len(),
        required: HEADER_LENGTH,
    })?;

    let mut raw = [0_u8; 2];
    raw.copy_from_slice(field);

    Ok(u16::from_le_bytes(raw))
}

fn read_u64_le(bytes: &[u8], offset: usize) -> Result<u64, DecodeError> {
    let end = offset
        .checked_add(8)
        .ok_or(DecodeError::TruncatedHeader {
            actual: bytes.len(),
            required: HEADER_LENGTH,
        })?;

    let field = bytes.get(offset..end).ok_or(DecodeError::TruncatedHeader {
        actual: bytes.len(),
        required: HEADER_LENGTH,
    })?;

    let mut raw = [0_u8; 8];
    raw.copy_from_slice(field);

    Ok(u64::from_le_bytes(raw))
}

// =============================================================================
// SHA-256
// =============================================================================

fn sha256(payload: &[u8]) -> [u8; DIGEST_LENGTH] {
    let mut hasher = Sha256::new();

    hasher.update(payload);

    let digest = hasher.finalize();

    let mut output = [0_u8; DIGEST_LENGTH];
    output.copy_from_slice(&digest);

    output
}

// =============================================================================
// Hex formatting
// =============================================================================

fn bytes_to_lower_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let capacity = bytes.len().saturating_mul(2);
    let mut output = String::with_capacity(capacity);

    for &byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }

    output
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};
    use std::io::Cursor;

    /*
     * A deliberately small test schema.
     *
     * The decoder is generic and therefore must not depend on the scheduler's
     * concrete schema representation.
     */
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct TestDocument {
        name: String,
        operations: Vec<u64>,
    }

    fn test_document() -> TestDocument {
        TestDocument {
            name: String::from("test"),
            operations: vec![0, 1, 2, 3],
        }
    }

    fn frame_for(payload: &[u8]) -> Vec<u8> {
        let digest = sha256(payload);

        let mut output = Vec::with_capacity(HEADER_LENGTH + payload.len());

        output.extend_from_slice(&FORMAT_MAGIC);
        output.extend_from_slice(&FORMAT_VERSION_MAJOR.to_le_bytes());
        output.extend_from_slice(&FORMAT_VERSION_MINOR.to_le_bytes());
        output.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        output.extend_from_slice(&digest);
        output.extend_from_slice(payload);

        output
    }

    #[test]
    fn decodes_canonical_json() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;

        let decoded: TestDocument =
            decode_canonical_json(payload).expect("canonical JSON must decode");

        assert_eq!(decoded, test_document());
    }

    #[test]
    fn rejects_non_canonical_json_when_required() {
        /*
         * The keys are intentionally reversed relative to canonical
         * lexicographic order.
         */
        let payload = br#"{"operations":[0,1,2,3],"name":"test"}"#;

        let result: Result<TestDocument, DecodeError> =
            decode_canonical_json(payload);

        assert!(matches!(result, Err(DecodeError::NonCanonicalJson)));
    }

    #[test]
    fn accepts_non_canonical_json_with_explicit_interchange_api() {
        let payload = br#"{"operations":[0,1,2,3],"name":"test"}"#;

        let decoded: TestDocument =
            decode_json(payload).expect("valid JSON must decode");

        assert_eq!(decoded, test_document());
    }

    #[test]
    fn decodes_complete_frame() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let frame = frame_for(payload);

        let decoded: DecodedArtifact<TestDocument> =
            decode(&frame).expect("frame must decode");

        assert_eq!(decoded.value, test_document());
        assert_eq!(
            decoded.frame.payload_length,
            payload.len() as u64
        );
    }

    #[test]
    fn frame_metadata_is_read_without_payload_allocation() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let frame = frame_for(payload);

        let metadata =
            inspect_frame(&frame).expect("header must decode");

        assert_eq!(
            metadata.payload_length,
            payload.len() as u64
        );
        assert_eq!(
            metadata.format_major,
            FORMAT_VERSION_MAJOR
        );
        assert_eq!(
            metadata.format_minor,
            FORMAT_VERSION_MINOR
        );
    }

    #[test]
    fn rejects_invalid_magic() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let mut frame = frame_for(payload);

        frame[0] ^= 0xff;

        let result: Result<DecodedArtifact<TestDocument>, DecodeError> =
            decode(&frame);

        assert!(matches!(
            result,
            Err(DecodeError::InvalidMagic { .. })
        ));
    }

    #[test]
    fn rejects_major_version_mismatch() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let mut frame = frame_for(payload);

        frame[8..10].copy_from_slice(
            &FORMAT_VERSION_MAJOR.saturating_add(1).to_le_bytes()
        );

        let result: Result<DecodedArtifact<TestDocument>, DecodeError> =
            decode(&frame);

        assert!(matches!(
            result,
            Err(DecodeError::UnsupportedFormatMajor { .. })
        ));
    }

    #[test]
    fn rejects_future_minor_version_by_default() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let mut frame = frame_for(payload);

        frame[10..12].copy_from_slice(
            &FORMAT_VERSION_MINOR.saturating_add(1).to_le_bytes()
        );

        let result: Result<DecodedArtifact<TestDocument>, DecodeError> =
            decode(&frame);

        assert!(matches!(
            result,
            Err(DecodeError::UnsupportedFutureFormatMinor { .. })
        ));
    }

    #[test]
    fn future_minor_version_can_be_explicitly_allowed() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let mut frame = frame_for(payload);

        frame[10..12].copy_from_slice(
            &FORMAT_VERSION_MINOR.saturating_add(1).to_le_bytes()
        );

        let options =
            DecodeOptions::production()
                .with_future_format_minor_allowed(true);

        let decoded: DecodedArtifact<TestDocument> =
            decode_with_options(&frame, options)
                .expect("explicit future-minor opt-in must work");

        assert_eq!(decoded.value, test_document());
    }

    #[test]
    fn rejects_truncated_header() {
        let frame = vec![0_u8; HEADER_LENGTH - 1];

        let result: Result<DecodedArtifact<TestDocument>, DecodeError> =
            decode(&frame);

        assert!(matches!(
            result,
            Err(DecodeError::TruncatedHeader { .. })
        ));
    }

    #[test]
    fn rejects_truncated_payload() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let mut frame = frame_for(payload);

        frame.pop();

        let result: Result<DecodedArtifact<TestDocument>, DecodeError> =
            decode(&frame);

        assert!(matches!(
            result,
            Err(DecodeError::TruncatedPayload { .. })
        ));
    }

    #[test]
    fn rejects_trailing_bytes_by_default() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let mut frame = frame_for(payload);

        frame.extend_from_slice(b"extra");

        let result: Result<DecodedArtifact<TestDocument>, DecodeError> =
            decode(&frame);

        assert!(matches!(
            result,
            Err(DecodeError::TrailingBytes { .. })
        ));
    }

    #[test]
    fn trailing_bytes_can_be_allowed_explicitly() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let mut frame = frame_for(payload);

        /*
         * The decoder is allowed to ignore transport framing after the exact
         * payload when explicitly configured.
         */
        frame.extend_from_slice(b"extra");

        let options =
            DecodeOptions::production()
                .with_trailing_bytes_rejected(false);

        let decoded: DecodedArtifact<TestDocument> =
            decode_with_options(&frame, options)
                .expect("trailing bytes should be allowed explicitly");

        assert_eq!(decoded.value, test_document());
    }

    #[test]
    fn rejects_digest_mismatch() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let mut frame = frame_for(payload);

        let digest_offset = 20;
        frame[digest_offset] ^= 0xff;

        let result: Result<DecodedArtifact<TestDocument>, DecodeError> =
            decode(&frame);

        assert!(matches!(
            result,
            Err(DecodeError::DigestMismatch { .. })
        ));
    }

    #[test]
    fn rejects_payload_tampering() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let mut frame = frame_for(payload);

        let payload_offset = HEADER_LENGTH;
        frame[payload_offset] ^= 0xff;

        let result: Result<DecodedArtifact<TestDocument>, DecodeError> =
            decode(&frame);

        assert!(matches!(
            result,
            Err(DecodeError::DigestMismatch { .. })
        ));
    }

    #[test]
    fn enforces_explicit_payload_limit() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let frame = frame_for(payload);

        let options =
            DecodeOptions::with_max_payload_bytes(
                (payload.len() as u64).saturating_sub(1)
            );

        let result: Result<DecodedArtifact<TestDocument>, DecodeError> =
            decode_with_options(&frame, options);

        assert!(matches!(
            result,
            Err(DecodeError::PayloadLimitExceeded { .. })
        ));
    }

    #[test]
    fn reader_decode_matches_slice_decode() {
        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let frame = frame_for(payload);

        let expected: DecodedArtifact<TestDocument> =
            decode(&frame).expect("slice decode must work");

        let mut reader = Cursor::new(frame);

        let actual: DecodedArtifact<TestDocument> =
            decode_from_reader(&mut reader)
                .expect("reader decode must work");

        assert_eq!(actual, expected);
    }

    #[test]
    fn reader_handles_fragmented_input() {
        struct FragmentedReader {
            chunks: Vec<Vec<u8>>,
            current: usize,
        }

        impl Read for FragmentedReader {
            fn read(
                &mut self,
                buffer: &mut [u8],
            ) -> io::Result<usize> {
                if self.current >= self.chunks.len() {
                    return Ok(0);
                }

                let chunk = &self.chunks[self.current];

                let amount = chunk.len().min(buffer.len());

                buffer[..amount].copy_from_slice(&chunk[..amount]);

                if amount == chunk.len() {
                    self.current += 1;
                } else {
                    self.chunks[self.current] =
                        chunk[amount..].to_vec();
                }

                Ok(amount)
            }
        }

        let payload = br#"{"name":"test","operations":[0,1,2,3]}"#;
        let frame = frame_for(payload);

        let chunks = frame
            .chunks(3)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();

        let mut reader = FragmentedReader {
            chunks,
            current: 0,
        };

        let decoded: DecodedArtifact<TestDocument> =
            decode_from_reader(&mut reader)
                .expect("fragmented input must decode");

        assert_eq!(decoded.value, test_document());
    }

    #[test]
    fn digest_hex_is_stable() {
        let digest = [0xab_u8, 0xcd_u8, 0x00_u8, 0xff_u8];

        assert_eq!(
            bytes_to_lower_hex(&digest),
            "abcd00ff"
        );
    }

    #[test]
    fn metadata_total_length_is_checked() {
        let metadata = FrameMetadata {
            format_major: FORMAT_VERSION_MAJOR,
            format_minor: FORMAT_VERSION_MINOR,
            payload_length: 10,
            payload_digest: [0_u8; DIGEST_LENGTH],
        };

        assert_eq!(
            metadata.total_length(),
            Some(HEADER_LENGTH + 10)
        );
    }
}