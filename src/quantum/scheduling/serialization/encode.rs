//! Zamani Quantum Scheduling — Canonical Schedule Encoder
//!
//! This module owns the serialization *mechanism* for scheduling artifacts.
//!
//! # Architectural responsibility
//!
//! This file answers:
//!
//! > "How is an already-structured scheduling document converted into
//! > deterministic, integrity-protected bytes?"
//!
//! It owns:
//!
//! - canonical JSON materialization;
//! - deterministic object-key ordering;
//! - preservation of array ordering;
//! - UTF-8 JSON encoding;
//! - binary framing;
//! - payload length framing;
//! - SHA-256 integrity;
//! - writer-based encoding;
//! - in-memory encoding;
//! - checked integer conversions;
//! - encoder configuration;
//! - explicit output statistics.
//!
//! It does NOT own:
//!
//! - scheduling semantics;
//! - schedule construction;
//! - dependency analysis;
//! - resource allocation;
//! - timing;
//! - routing;
//! - hardware discovery;
//! - QEC;
//! - runtime execution;
//! - schema definition;
//! - schema migration;
//! - decoding;
//! - authentication/signatures;
//! - encryption;
//! - compression.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//! scheduling result
//!       │
//!       ▼
//! serialization::schema
//!       │
//!       │ structured document
//!       ▼
//! serialization::encode       ← THIS FILE
//!       │
//!       ├── canonical JSON
//!       ├── SHA-256
//!       └── binary framing
//!       │
//!       ▼
//! canonical schedule bytes
//!       │
//!       ├── filesystem/storage
//!       ├── cache
//!       ├── network transport
//!       ├── distributed compilation
//!       └── runtime artifact transport
//! ```
//!
//! # Critical separation
//!
//! ```text
//! schema.rs
//!     = WHAT the persisted scheduling document means
//!
//! encode.rs
//!     = HOW the document becomes bytes
//!
//! decode.rs
//!     = HOW bytes become a validated document
//! ```
//!
//! This separation is deliberate.
//!
//! Adding a new scheduling field should normally require changing the schema
//! and its conversion layer, but should NOT require changing this encoder.
//!
//! # Generic design
//!
//! The primary API is generic over any Serde-serializable schema value:
//!
//! ```text
//! T: Serialize
//!     │
//!     ▼
//! canonical JSON
//!     │
//!     ▼
//! framed schedule artifact
//! ```
//!
//! Therefore this encoder is not coupled to:
//!
//! - `ScheduleResult`;
//! - a particular scheduler algorithm;
//! - ASAP;
//! - ALAP;
//! - list scheduling;
//! - RCPSP;
//! - QEC;
//! - distributed scheduling;
//! - a particular hardware target;
//! - a particular quantum technology.
//!
//! The scheduling `schema.rs` module defines the structured document that
//! should normally be supplied as `T`.
//!
//! # Canonical representation
//!
//! Canonicalization follows these rules:
//!
//! 1. JSON object keys are emitted in lexicographic order.
//! 2. JSON array order is preserved exactly.
//! 3. Primitive values retain their Serde/JSON meaning.
//! 4. No semantic sorting of arrays is performed.
//! 5. No floating-point normalization is invented here.
//! 6. The resulting JSON is emitted without insignificant whitespace.
//!
//! Array order is never changed because array order may be semantically
//! meaningful to scheduling.
//!
//! # Integrity
//!
//! The binary artifact contains SHA-256 over the exact canonical JSON payload.
//!
//! Integrity means:
//!
//! ```text
//! corruption detection
//! ```
//!
//! It does NOT mean:
//!
//! ```text
//! authenticity
//! authorization
//! identity
//! ```
//!
//! Digital signatures and authorization belong to a higher security layer.
//!
//! # Scalability
//!
//! This encoder contains no quantum-machine-size constants.
//!
//! There is no:
//!
//! - maximum qubit count;
//! - maximum physical-qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - maximum schedule depth;
//! - maximum QEC-round count;
//! - maximum node count;
//! - maximum communication-link count.
//!
//! The encoder processes the actual serialized representation supplied by the
//! caller and is therefore constrained only by:
//!
//! - available memory;
//! - the selected transport;
//! - the host address space;
//! - the representable serialization framing fields.
//!
//! A caller that requires bounded resource consumption should enforce those
//! limits before or during serialization through the scheduling resource/
//! execution policy layer.
//!
//! This file does not turn a particular machine's resource limits into global
//! architectural limits.
//!
//! # Determinism
//!
//! For deterministic `Serialize` implementations:
//!
//! ```text
//! same schema value
//!      +
//! same serializer configuration
//!      =
//! same canonical payload
//!      =
//! same SHA-256
//!      =
//! same framed bytes
//! ```
//!
//! This makes the representation suitable for:
//!
//! - reproducible builds;
//! - content-addressed artifacts;
//! - distributed compilation;
//! - cache keys;
//! - regression tests;
//! - provenance;
//! - schedule comparison.
//!
//! # Security
//!
//! This module never:
//!
//! - executes decoded operations;
//! - invokes hardware;
//! - accesses the network;
//! - accesses the filesystem implicitly;
//! - invokes external processes;
//! - uses global mutable state;
//! - uses `unsafe`.
//!
//! A caller explicitly provides the destination writer when using
//! [`encode_to_writer`].
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
//! - no unsafe code.
//!
//! # Integration contract
//!
//! This module is intentionally generic so that future changes to
//! `serialization/schema.rs` do not require modifications here.
//!
//! The normal dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling::result
//!      │
//!      ▼
//! scheduling::serialization::schema
//!      │
//!      ▼
//! scheduling::serialization::encode   ← THIS FILE
//!      │
//!      ▼
//! bytes
//! ```
//!
//! `decode.rs` should consume the frame generated here.
//!
//! `schema.rs` should remain responsible for validating schema-level
//! invariants.
//!
//! `errors.rs` remains the canonical scheduling error boundary.
//!
//! No scheduler algorithm should depend directly on this implementation.
//!
//! # Important qubit rule
//!
//! This file intentionally does not define or reinterpret qubit identities.
//!
//! Any scheduling schema that contains qubit identities must use the canonical
//! identities owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The encoder simply serializes the schema representation supplied to it.
//!
//! # No unsafe code
//!
//! The compiler enforces the safety boundary below.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, Write};

use serde::Serialize;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

// =============================================================================
// Binary format contract
// =============================================================================

/// Eight-byte magic identifying a Zamani quantum scheduling serialization
/// artifact.
///
/// The magic is deliberately independent of:
// - compiler version;
/// - Zamani language version;
/// - hardware vendor;
/// - hardware model;
/// - scheduling algorithm.
///
/// The final byte is reserved as a format-family discriminator so that future
/// scheduling serialization formats can coexist without ambiguity.
pub const FORMAT_MAGIC: [u8; 8] = *b"ZAMSCH\0\1";

/// Current binary serialization format major version.
///
/// A major-version change may require a different decoder contract.
pub const FORMAT_VERSION_MAJOR: u16 = 1;

/// Current binary serialization format minor version.
///
/// Minor versions are intended for compatible additions that do not invalidate
/// the basic framing contract.
pub const FORMAT_VERSION_MINOR: u16 = 0;

/// Number of bytes occupied by a SHA-256 digest.
pub const DIGEST_LENGTH: usize = 32;

/// Number of bytes occupied by the fixed binary header before the payload.
///
/// Layout:
///
/// ```text
/// magic                 8 bytes
/// format major          2 bytes
/// format minor          2 bytes
/// payload length        8 bytes
/// payload SHA-256       32 bytes
/// ```
///
/// Total: 52 bytes.
pub const HEADER_LENGTH: usize = 52;

// =============================================================================
// Encoder options
// =============================================================================

/// Configuration controlling canonical schedule encoding.
///
/// The encoder deliberately contains no scheduler-size limits.
///
/// Resource limits belong to the scheduling policy / execution policy layer.
/// This structure only controls the serialization mechanism itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodeOptions {
    /// Whether the binary framing header should be emitted.
    ///
    /// When `true`, [`encode`] and [`encode_to_writer`] produce the complete
    /// Zamani scheduling artifact.
    ///
    /// When `false`, callers should normally use [`encode_canonical_json`]
    /// instead because a headerless binary JSON payload has less explicit
    /// framing information.
    pub framed: bool,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self { framed: true }
    }
}

impl EncodeOptions {
    /// Returns options for complete framed schedule artifacts.
    #[must_use]
    pub const fn framed() -> Self {
        Self { framed: true }
    }

    /// Returns options for payload-only output.
    ///
    /// Prefer [`encode_canonical_json`] when payload-only output is desired.
    #[must_use]
    pub const fn payload_only() -> Self {
        Self { framed: false }
    }
}

// =============================================================================
// Encode errors
// =============================================================================

/// Errors produced by the serialization mechanism.
///
/// Schema semantic errors should normally be reported by `schema.rs` before
/// encoding. These errors are specifically concerned with conversion to and
/// writing of the serialized representation.
#[derive(Debug)]
pub enum EncodeError {
    /// The supplied value could not be converted into a JSON value.
    Serialize(serde_json::Error),

    /// The canonical JSON value could not be encoded as UTF-8 JSON bytes.
    Json(serde_json::Error),

    /// The payload is larger than the length field of the binary framing.
    ///
    /// This is a representation failure, not a quantum-machine-size limit.
    PayloadLengthOverflow {
        /// Actual payload length represented by the host.
        length: usize,
    },

    /// A write operation failed.
    Io(io::Error),

    /// The generated framing/header is internally inconsistent.
    InvalidFrame(String),
}

impl fmt::Display for EncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialize(error) => {
                write!(formatter, "failed to serialize scheduling document: {error}")
            }

            Self::Json(error) => {
                write!(formatter, "failed to encode canonical scheduling JSON: {error}")
            }

            Self::PayloadLengthOverflow { length } => {
                write!(
                    formatter,
                    "scheduling serialization payload length {length} cannot be represented by the wire format"
                )
            }

            Self::Io(error) => {
                write!(
                    formatter,
                    "failed to write scheduling serialization artifact: {error}"
                )
            }

            Self::InvalidFrame(reason) => {
                write!(
                    formatter,
                    "internal scheduling serialization frame error: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialize(error) | Self::Json(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::PayloadLengthOverflow { .. } | Self::InvalidFrame(_) => None,
        }
    }
}

impl From<serde_json::Error> for EncodeError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialize(error)
    }
}

impl From<io::Error> for EncodeError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

// =============================================================================
// Encoded artifact statistics
// =============================================================================

/// Metadata describing the encoded artifact.
///
/// This structure avoids forcing callers to reparse the output merely to learn
/// its size or digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedArtifact {
    /// Complete serialized bytes.
    pub bytes: Vec<u8>,

    /// Number of canonical JSON payload bytes.
    pub payload_length: u64,

    /// SHA-256 digest of the canonical JSON payload.
    pub payload_digest: [u8; DIGEST_LENGTH],

    /// Binary header length.
    ///
    /// This is zero when `framed == false`.
    pub header_length: usize,
}

impl EncodedArtifact {
    /// Returns the total number of bytes in the serialized artifact.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the serialized artifact is empty.
    ///
    /// A valid framed artifact is never empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns the complete SHA-256 digest as lowercase hexadecimal.
    #[must_use]
    pub fn digest_hex(&self) -> String {
        bytes_to_lower_hex(&self.payload_digest)
    }

    /// Returns the payload bytes without the binary header.
    ///
    /// This returns a borrowed slice and therefore does not allocate.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.bytes[self.header_length..]
    }
}

// =============================================================================
// Public canonical JSON API
// =============================================================================

/// Serializes any schema-compatible value into canonical compact JSON.
///
/// This function does not add the binary scheduling envelope.
///
/// Use this when the caller needs:
///
/// - a canonical JSON representation;
/// - a digest input;
/// - an interchange payload;
/// - a debugging artifact.
///
/// For the complete binary scheduling artifact use [`encode`].
///
/// # Determinism
///
/// JSON object keys are canonicalized recursively.
///
/// JSON array ordering is preserved.
///
/// # Errors
///
/// The supplied `Serialize` implementation may reject serialization.
///
/// # Scalability
///
/// No scheduler or quantum-system size is encoded here.
///
/// Memory consumption is proportional to the serialized representation.
pub fn encode_canonical_json<T>(value: &T) -> Result<Vec<u8>, EncodeError>
where
    T: Serialize,
{
    let value = serde_json::to_value(value).map_err(EncodeError::Serialize)?;
    let canonical = canonicalize_json_value(value);

    serde_json::to_vec(&canonical).map_err(EncodeError::Json)
}

// =============================================================================
// Public digest API
// =============================================================================

/// Calculates SHA-256 over canonical scheduling JSON.
///
/// The returned digest is exactly the digest embedded in a framed artifact
/// produced by [`encode`].
pub fn canonical_payload_digest<T>(value: &T) -> Result<[u8; DIGEST_LENGTH], EncodeError>
where
    T: Serialize,
{
    let payload = encode_canonical_json(value)?;
    Ok(sha256(&payload))
}

// =============================================================================
// Public complete encoder
// =============================================================================

/// Encodes a scheduling schema value into a complete Zamani scheduling
/// serialization artifact.
///
/// The result consists of:
///
/// ```text
/// ┌──────────────────────────────────────┐
/// │ fixed binary header                  │
/// ├──────────────────────────────────────┤
/// │ canonical JSON payload               │
/// └──────────────────────────────────────┘
/// ```
///
/// Header:
///
/// ```text
/// magic          [8]
/// major          [2]
/// minor          [2]
/// payload_len    [8]
/// sha256         [32]
/// ```
///
/// Integers are encoded in big-endian order.
///
/// # Why big endian?
///
/// The framing layer is transport-oriented rather than host-memory-oriented.
/// Big-endian representation avoids dependence on host CPU byte order.
///
/// # Important
///
/// The `T` supplied here should normally be a type from
/// `serialization::schema`.
///
/// This function intentionally does not know the concrete scheduling schema.
/// That prevents future schema additions from requiring modifications to the
/// encoder.
pub fn encode<T>(value: &T) -> Result<EncodedArtifact, EncodeError>
where
    T: Serialize,
{
    encode_with_options(value, EncodeOptions::default())
}

/// Encodes a scheduling schema value with explicit serialization options.
pub fn encode_with_options<T>(
    value: &T,
    options: EncodeOptions,
) -> Result<EncodedArtifact, EncodeError>
where
    T: Serialize,
{
    let payload = encode_canonical_json(value)?;
    let payload_length =
        u64::try_from(payload.len()).map_err(|_| EncodeError::PayloadLengthOverflow {
            length: payload.len(),
        })?;

    let digest = sha256(&payload);

    if !options.framed {
        return Ok(EncodedArtifact {
            bytes: payload,
            payload_length,
            payload_digest: digest,
            header_length: 0,
        });
    }

    let mut bytes = Vec::with_capacity(
        HEADER_LENGTH
            .checked_add(payload.len())
            .ok_or_else(|| {
                EncodeError::InvalidFrame(
                    "header plus payload length overflowed host capacity".to_owned(),
                )
            })?,
    );

    write_header(&mut bytes, payload_length, &digest)?;
    bytes.extend_from_slice(&payload);

    debug_assert_eq!(bytes.len(), HEADER_LENGTH + payload.len());

    Ok(EncodedArtifact {
        bytes,
        payload_length,
        payload_digest: digest,
        header_length: HEADER_LENGTH,
    })
}

// =============================================================================
// Writer-based encoder
// =============================================================================

/// Encodes a scheduling document directly into a caller-provided writer.
///
/// This API is important for large schedules because callers do not have to
/// retain the final artifact after writing it.
///
/// The canonicalization step still materializes the canonical JSON payload
/// because deterministic canonical JSON must be produced before its SHA-256
/// digest and framing can be finalized.
///
/// For genuinely streaming, constant-memory transport, a future streaming
/// canonical codec may be added behind the same schema boundary. This function
/// intentionally does not pretend that ordinary Serde serialization is
/// streaming.
pub fn encode_to_writer<T, W>(
    value: &T,
    writer: &mut W,
) -> Result<EncodeStatistics, EncodeError>
where
    T: Serialize,
    W: Write,
{
    let payload = encode_canonical_json(value)?;

    let payload_length =
        u64::try_from(payload.len()).map_err(|_| EncodeError::PayloadLengthOverflow {
            length: payload.len(),
        })?;

    let digest = sha256(&payload);

    write_header(writer, payload_length, &digest)?;
    writer.write_all(&payload)?;

    Ok(EncodeStatistics {
        header_length: HEADER_LENGTH,
        payload_length,
        total_length: HEADER_LENGTH
            .checked_add(payload.len())
            .ok_or_else(|| {
                EncodeError::InvalidFrame(
                    "header plus payload length overflowed host capacity".to_owned(),
                )
            })?,
        payload_digest: digest,
    })
}

// =============================================================================
// Payload-only writer API
// =============================================================================

/// Writes canonical JSON without the binary scheduling frame.
///
/// This is useful when the caller already owns an outer transport envelope.
pub fn encode_canonical_json_to_writer<T, W>(
    value: &T,
    writer: &mut W,
) -> Result<EncodeStatistics, EncodeError>
where
    T: Serialize,
    W: Write,
{
    let payload = encode_canonical_json(value)?;

    let payload_length =
        u64::try_from(payload.len()).map_err(|_| EncodeError::PayloadLengthOverflow {
            length: payload.len(),
        })?;

    let digest = sha256(&payload);

    writer.write_all(&payload)?;

    Ok(EncodeStatistics {
        header_length: 0,
        payload_length,
        total_length: payload.len(),
        payload_digest: digest,
    })
}

// =============================================================================
// Streaming statistics
// =============================================================================

/// Statistics returned by writer-based encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodeStatistics {
    /// Number of bytes occupied by the binary frame header.
    pub header_length: usize,

    /// Number of canonical JSON payload bytes.
    pub payload_length: u64,

    /// Total number of bytes written.
    pub total_length: usize,

    /// SHA-256 digest of the payload.
    pub payload_digest: [u8; DIGEST_LENGTH],
}

impl EncodeStatistics {
    /// Returns the payload digest as lowercase hexadecimal.
    #[must_use]
    pub fn digest_hex(&self) -> String {
        bytes_to_lower_hex(&self.payload_digest)
    }
}

// =============================================================================
// Header encoding
// =============================================================================

/// Writes the canonical binary header.
///
/// Header format:
///
/// ```text
/// offset  size  field
/// ------  ----  ----------------
/// 0       8     magic
/// 8       2     major version
/// 10      2     minor version
/// 12      8     payload length
/// 20      32    SHA-256 digest
/// ```
///
/// All integer fields are big-endian.
///
/// This function is intentionally private because the frame layout must remain
/// controlled by this module rather than being assembled inconsistently by
/// callers.
fn write_header<W>(
    writer: &mut W,
    payload_length: u64,
    digest: &[u8; DIGEST_LENGTH],
) -> Result<(), EncodeError>
where
    W: Write,
{
    writer.write_all(&FORMAT_MAGIC)?;
    writer.write_all(&FORMAT_VERSION_MAJOR.to_be_bytes())?;
    writer.write_all(&FORMAT_VERSION_MINOR.to_be_bytes())?;
    writer.write_all(&payload_length.to_be_bytes())?;
    writer.write_all(digest)?;

    Ok(())
}

// =============================================================================
// Canonical JSON implementation
// =============================================================================

/// Recursively canonicalizes a JSON value.
///
/// Object members are copied into a `BTreeMap`, guaranteeing deterministic
/// lexical key ordering independent of the concrete `serde_json::Map`
/// implementation.
///
/// Arrays are recursively canonicalized but their ordering is preserved.
///
/// This is intentionally a pure function with no global state.
#[must_use]
pub fn canonicalize_json_value(value: Value) -> Value {
    match value {
        Value::Object(object) => {
            let mut ordered = BTreeMap::<String, Value>::new();

            for (key, child) in object {
                ordered.insert(key, canonicalize_json_value(child));
            }

            let mut result = Map::new();

            for (key, child) in ordered {
                result.insert(key, child);
            }

            Value::Object(result)
        }

        Value::Array(values) => {
            Value::Array(
                values
                    .into_iter()
                    .map(canonicalize_json_value)
                    .collect(),
            )
        }

        primitive => primitive,
    }
}

// =============================================================================
// Digest
// =============================================================================

/// Computes SHA-256 for arbitrary bytes.
///
/// SHA-256 is used only for integrity/content identification.
///
/// It is not an authentication mechanism.
#[must_use]
pub fn sha256(bytes: &[u8]) -> [u8; DIGEST_LENGTH] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);

    let digest = hasher.finalize();

    let mut result = [0u8; DIGEST_LENGTH];
    result.copy_from_slice(&digest);

    result
}

// =============================================================================
// Hexadecimal formatting
// =============================================================================

/// Converts bytes to lowercase hexadecimal without external allocation-heavy
/// formatting machinery.
///
/// This function is intentionally deterministic.
#[must_use]
pub fn bytes_to_lower_hex(bytes: &[u8]) -> String {
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
// Convenience API
// =============================================================================

/// Encodes a schema value and returns only the complete artifact bytes.
///
/// This is a convenience wrapper around [`encode`].
pub fn encode_bytes<T>(value: &T) -> Result<Vec<u8>, EncodeError>
where
    T: Serialize,
{
    Ok(encode(value)?.bytes)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Serialize;
    use serde_json::json;

    #[derive(Debug, Serialize)]
    struct TestDocument {
        z: u64,
        a: u64,
        nested: TestNested,
        values: Vec<u64>,
    }

    #[derive(Debug, Serialize)]
    struct TestNested {
        beta: u64,
        alpha: u64,
    }

    #[test]
    fn canonicalizes_object_keys_recursively() {
        let value = json!({
            "z": 1,
            "a": 2,
            "nested": {
                "z": 3,
                "a": 4
            }
        });

        let bytes = encode_canonical_json(&value).expect("canonical encoding");

        assert_eq!(
            String::from_utf8(bytes).expect("valid UTF-8"),
            r#"{"a":2,"nested":{"a":4,"z":3},"z":1}"#
        );
    }

    #[test]
    fn preserves_array_order() {
        let value = json!({
            "values": [3, 1, 2]
        });

        let bytes = encode_canonical_json(&value).expect("canonical encoding");

        assert_eq!(
            String::from_utf8(bytes).expect("valid UTF-8"),
            r#"{"values":[3,1,2]}"#
        );
    }

    #[test]
    fn canonicalization_is_deterministic() {
        let first = TestDocument {
            z: 1,
            a: 2,
            nested: TestNested {
                beta: 4,
                alpha: 3,
            },
            values: vec![9, 2, 7],
        };

        let second = TestDocument {
            z: 1,
            a: 2,
            nested: TestNested {
                beta: 4,
                alpha: 3,
            },
            values: vec![9, 2, 7],
        };

        let first_bytes =
            encode(&first).expect("first encoding").bytes;

        let second_bytes =
            encode(&second).expect("second encoding").bytes;

        assert_eq!(first_bytes, second_bytes);
    }

    #[test]
    fn header_has_expected_length() {
        let value = json!({
            "schedule": "test"
        });

        let artifact = encode(&value).expect("encoding");

        assert_eq!(artifact.header_length, HEADER_LENGTH);
        assert_eq!(
            artifact.bytes.len(),
            HEADER_LENGTH + artifact.payload_length as usize
        );
    }

    #[test]
    fn header_contains_magic() {
        let value = json!({
            "schedule": "test"
        });

        let artifact = encode(&value).expect("encoding");

        assert_eq!(
            &artifact.bytes[..FORMAT_MAGIC.len()],
            FORMAT_MAGIC.as_slice()
        );
    }

    #[test]
    fn digest_matches_payload() {
        let value = json!({
            "schedule": "test"
        });

        let artifact = encode(&value).expect("encoding");

        let expected = sha256(artifact.payload());

        assert_eq!(artifact.payload_digest, expected);
    }

    #[test]
    fn digest_hex_is_lowercase() {
        let digest = [0xff_u8; DIGEST_LENGTH];

        assert_eq!(
            bytes_to_lower_hex(&digest),
            "ff".repeat(DIGEST_LENGTH)
        );
    }

    #[test]
    fn payload_only_encoding_has_no_header() {
        let value = json!({
            "schedule": "test"
        });

        let artifact =
            encode_with_options(&value, EncodeOptions::payload_only())
                .expect("encoding");

        assert_eq!(artifact.header_length, 0);
        assert_eq!(
            artifact.bytes.len(),
            artifact.payload_length as usize
        );
    }

    #[test]
    fn writer_encoding_matches_memory_encoding() {
        let value = json!({
            "b": 2,
            "a": 1,
            "items": [5, 4, 3]
        });

        let memory =
            encode(&value).expect("memory encoding");

        let mut output = Vec::new();

        let statistics =
            encode_to_writer(&value, &mut output)
                .expect("writer encoding");

        assert_eq!(memory.bytes, output);
        assert_eq!(
            statistics.payload_digest,
            memory.payload_digest
        );
        assert_eq!(
            statistics.payload_length,
            memory.payload_length
        );
        assert_eq!(
            statistics.total_length,
            memory.bytes.len()
        );
    }

    #[test]
    fn canonical_payload_digest_matches_encoded_artifact() {
        let value = json!({
            "b": 2,
            "a": 1
        });

        let direct =
            canonical_payload_digest(&value)
                .expect("digest");

        let artifact =
            encode(&value)
                .expect("encoding");

        assert_eq!(direct, artifact.payload_digest);
    }

    #[test]
    fn empty_arrays_remain_empty() {
        let value = json!({
            "values": []
        });

        let bytes =
            encode_canonical_json(&value)
                .expect("encoding");

        assert_eq!(
            String::from_utf8(bytes).expect("UTF-8"),
            r#"{"values":[]}"#
        );
    }

    #[test]
    fn nested_arrays_are_not_reordered() {
        let value = json!({
            "matrix": [
                [3, 2, 1],
                [6, 5, 4]
            ]
        });

        let bytes =
            encode_canonical_json(&value)
                .expect("encoding");

        assert_eq!(
            String::from_utf8(bytes).expect("UTF-8"),
            r#"{"matrix":[[3,2,1],[6,5,4]]}"#
        );
    }

    #[test]
    fn null_and_boolean_values_are_preserved() {
        let value = json!({
            "null_value": null,
            "enabled": true,
            "disabled": false
        });

        let bytes =
            encode_canonical_json(&value)
                .expect("encoding");

        assert_eq!(
            String::from_utf8(bytes).expect("UTF-8"),
            r#"{"disabled":false,"enabled":true,"null_value":null}"#
        );
    }

    #[test]
    fn header_version_is_big_endian() {
        let value = json!({
            "value": 1
        });

        let artifact =
            encode(&value)
                .expect("encoding");

        let major_start = FORMAT_MAGIC.len();
        let minor_start = major_start + 2;

        assert_eq!(
            &artifact.bytes[major_start..major_start + 2],
            FORMAT_VERSION_MAJOR.to_be_bytes()
        );

        assert_eq!(
            &artifact.bytes[minor_start..minor_start + 2],
            FORMAT_VERSION_MINOR.to_be_bytes()
        );
    }

    #[test]
    fn payload_length_is_big_endian() {
        let value = json!({
            "value": 1
        });

        let artifact =
            encode(&value)
                .expect("encoding");

        let start = 12;
        let end = start + 8;

        assert_eq!(
            &artifact.bytes[start..end],
            artifact.payload_length.to_be_bytes()
        );
    }
}