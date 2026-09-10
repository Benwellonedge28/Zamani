//! Versioned binary serialization for the native Zamani AST.
//!
//! # Architectural position
//!
//! This module belongs to:
//!
//! ```text
//! src/frontend/ast/node/serialization/
//! ```
//!
//! It owns the **binary transport/container format** for serializable native
//! AST values. It does not own AST node definitions, semantic analysis, ZUIR,
//! quantum IR, hardware mapping, routing, scheduling, calibration, QEC,
//! execution, or backend integration.
//!
//! The intended pipeline is:
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! parser
//!      |
//!      v
//! native Zamani AST
//!      |
//!      v
//! binary serialization
//!      |
//!      v
//! persisted/inter-process bytes
//!      |
//!      v
//! binary deserialization
//!      |
//!      v
//! native Zamani AST
//!      |
//!      v
//! structural validation
//!      |
//!      v
//! semantic analysis
//!      |
//!      v
//! ZUIR
//! ```
//!
//! # Design goals
//!
//! This implementation provides:
//!
//! - deterministic framing;
//! - explicit format versioning;
//! - explicit payload length;
//! - corruption detection;
//! - strict trailing-byte rejection;
//! - configurable decoding limits;
//! - no hidden AST-size limits;
//! - no hard-coded qubit limits;
//! - no hard-coded machine limits;
//! - no target/backend knowledge;
//! - no unsafe code;
//! - Rust 1.97 / 1.97.1 compatibility;
//! - generic `Serialize`/`Deserialize` support;
//! - forward rejection for unsupported format versions;
//! - explicit distinction between framing and payload errors;
//! - stable binary envelope semantics.
//!
//! # Why the payload uses serde
//!
//! The native AST already uses `serde` structural serialization. This module
//! deliberately does not duplicate every AST node in a second binary-specific
//! representation.
//!
//! The binary format therefore consists of a small, stable binary envelope
//! containing the serialized AST payload.
//!
//! This keeps the serialization boundary extensible:
//!
//! ```text
//! new AST node
//!      |
//!      +--> Serialize
//!      |
//!      +--> Deserialize
//!      |
//!      +--> existing binary container
//! ```
//!
//! Adding a new AST construct therefore does not require modifying this file.
//!
//! The payload representation is JSON bytes because `serde_json` is already a
//! repository dependency. The outer representation is nevertheless explicitly
//! binary/framed and suitable for files, caches, compiler IPC, and transport.
//!
//! The format is intentionally not advertised as a compact hardware encoding.
//! It is a stable AST serialization container. A future independently specified
//! compact codec can be added without changing the native AST.
//!
//! # Binary format
//!
//! The current format is:
//!
//! ```text
//! +----------------------+
//! | magic       8 bytes  |
//! +----------------------+
//! | version     2 bytes  |
//! +----------------------+
//! | flags       2 bytes  |
//! +----------------------+
//! | payload_len 8 bytes  |
//! +----------------------+
//! | checksum   32 bytes  |
//! +----------------------+
//! | payload    N bytes   |
//! +----------------------+
//! ```
//!
//! Integer fields are encoded as unsigned little-endian integers.
//!
//! The checksum is SHA-256 over the payload only.
//!
//! The checksum provides corruption detection. It does **not** provide
//! authenticity, authorization, signatures, or trust.
//!
//! Authentication/signing belongs to a higher-level package/distribution or
//! security layer.
//!
//! # Scalability
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_AST_BYTES
//! MAX_AST_NODES
//! MAX_QUANTUM_BITS
//! MAX_QUBITS
//! MAX_REGISTER_SIZE
//! MAX_PROGRAM_SIZE
//! ```
//!
//! in this module.
//!
//! `u64` is used for the serialized payload length because the format must not
//! inherit the host `usize` width.
//!
//! Applications processing untrusted input may supply a
//! [`BinaryDecodeLimits`] value with an explicit maximum payload size.
//!
//! Such a limit is an operational/security policy, not a language or AST
//! semantic limit.
//!
//! # Determinism
//!
//! Determinism depends on the serialized AST representation being deterministic.
//! The native AST should therefore use deterministic structures for unordered
//! data. For example, the existing metadata implementation uses `BTreeMap` for
//! deterministic object ordering.
//!
//! This module itself introduces no timestamps, process IDs, memory addresses,
//! random values, thread IDs, host paths, or backend state.
//!
//! # Integration contract
//!
//! This module may depend on:
//!
//! - Rust standard library;
//! - `serde`;
//! - `serde_json`;
//! - `sha2`.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - QEC;
//! - routing;
//! - scheduling;
//! - hardware;
//! - backend APIs;
//! - runtime execution;
//! - compiler global mutable state.
//!
//! Consumers include the AST serialization module and compiler persistence/
//! caching infrastructure.
//!
//! AST node definitions depend only on their own serialization derives; they
//! do not depend on this module.
//!
//! # Rust compatibility
//!
//! Target compiler:
//!
//! ```text
//! Rust 1.97 / Rust 1.97.1
//! ```
//!
//! No `unsafe` code is used.

use std::fmt;
use std::io::{self, Cursor, Read};

use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::{Digest, Sha256};

/// Eight-byte magic value identifying a Zamani AST binary container.
///
/// The bytes are deliberately ASCII-compatible for straightforward hex dumps
/// and diagnostics while remaining unambiguous as a binary file signature.
pub const BINARY_MAGIC: [u8; 8] = *b"ZASTBIN\0";

/// Current binary-container format version.
///
/// This is the version of the **container format**, not:
//!
//! - the Zamani language version;
//! - the AST schema version;
//! - a node representation version;
//! - the compiler version;
//! - an extension version;
//! - a semantic-model version;
//! - a ZUIR version.
pub const BINARY_FORMAT_VERSION: u16 = 1;

/// No optional binary-container flags are currently defined.
///
/// Unknown flags are rejected by the decoder rather than silently ignored.
/// This prevents a future producer from accidentally communicating semantics
/// to an old decoder that it does not understand.
pub const BINARY_FLAGS_NONE: u16 = 0;

/// Number of bytes occupied by the fixed binary header.
pub const BINARY_HEADER_LEN: usize = 52;

/// Number of bytes in the SHA-256 payload digest.
pub const CHECKSUM_LEN: usize = 32;

/// Configuration controlling binary decoding.
///
/// No limit is enabled by default.
///
/// This is intentional: the AST format itself has no artificial program-size
/// ceiling. Applications that deserialize untrusted data should configure an
/// explicit limit appropriate to their resource policy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BinaryDecodeLimits {
    /// Optional maximum serialized payload size in bytes.
    ///
    /// `None` means that this codec imposes no payload-size limit.
    ///
    /// This is a security/resource policy and must not be interpreted as an
    /// AST or language semantic limit.
    pub max_payload_bytes: Option<u64>,
}

impl BinaryDecodeLimits {
    /// Creates unlimited decoding limits.
    #[inline]
    pub const fn unlimited() -> Self {
        Self {
            max_payload_bytes: None,
        }
    }

    /// Creates a decoder policy with a maximum payload size.
    ///
    /// The value is expressed in bytes and is checked before allocation.
    #[inline]
    pub const fn with_max_payload_bytes(max_payload_bytes: u64) -> Self {
        Self {
            max_payload_bytes: Some(max_payload_bytes),
        }
    }

    #[inline]
    fn check_payload_length(self, length: u64) -> Result<(), BinaryDecodeError> {
        if let Some(maximum) = self.max_payload_bytes {
            if length > maximum {
                return Err(BinaryDecodeError::PayloadLimitExceeded {
                    declared: length,
                    maximum,
                });
            }
        }

        Ok(())
    }
}

/// Binary-container header.
///
/// The header is kept as a value type so callers can inspect validated framing
/// information without having to deserialize the payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BinaryHeader {
    /// Container magic.
    pub magic: [u8; 8],

    /// Container format version.
    pub version: u16,

    /// Container flags.
    pub flags: u16,

    /// Serialized payload length in bytes.
    pub payload_len: u64,

    /// SHA-256 digest of the payload.
    pub checksum: [u8; CHECKSUM_LEN],
}

impl BinaryHeader {
    /// Creates a header for a serialized payload.
    fn for_payload(payload: &[u8]) -> Self {
        let digest = Sha256::digest(payload);
        let mut checksum = [0u8; CHECKSUM_LEN];
        checksum.copy_from_slice(&digest);

        Self {
            magic: BINARY_MAGIC,
            version: BINARY_FORMAT_VERSION,
            flags: BINARY_FLAGS_NONE,
            payload_len: payload.len() as u64,
            checksum,
        }
    }

    /// Returns whether the header has the canonical magic.
    #[inline]
    pub const fn has_valid_magic(&self) -> bool {
        self.magic == BINARY_MAGIC
    }

    /// Returns the expected total encoded size when representable as `usize`.
    ///
    /// This is a checked conversion because serialized lengths are expressed
    /// as `u64` independently of the host architecture.
    #[inline]
    pub fn encoded_len(&self) -> Option<usize> {
        usize::try_from(self.payload_len)
            .ok()
            .and_then(|payload| BINARY_HEADER_LEN.checked_add(payload))
    }
}

/// Errors produced while encoding an AST into the binary container.
#[derive(Debug)]
pub enum BinaryEncodeError {
    /// The serde representation could not be produced.
    Serialize(serde_json::Error),

    /// The serialized payload length could not be represented by the wire
    /// format.
    PayloadTooLarge {
        /// Actual host-side serialized payload length.
        length: usize,
    },
}

impl fmt::Display for BinaryEncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialize(error) => {
                write!(formatter, "failed to serialize AST payload: {error}")
            }
            Self::PayloadTooLarge { length } => write!(
                formatter,
                "serialized AST payload length {length} cannot be represented by the binary format"
            ),
        }
    }
}

impl std::error::Error for BinaryEncodeError {}

impl From<serde_json::Error> for BinaryEncodeError {
    #[inline]
    fn from(error: serde_json::Error) -> Self {
        Self::Serialize(error)
    }
}

/// Errors produced while decoding the binary AST container.
#[derive(Debug)]
pub enum BinaryDecodeError {
    /// The byte stream ended before the fixed header was complete.
    TruncatedHeader,

    /// The byte stream ended before the declared payload was complete.
    TruncatedPayload {
        /// Number of payload bytes required by the header.
        expected: u64,

        /// Number of payload bytes actually available.
        available: usize,
    },

    /// The file/container does not contain the expected magic.
    InvalidMagic {
        /// Bytes encountered at the beginning of the input.
        found: [u8; 8],
    },

    /// The container version is not supported.
    UnsupportedVersion {
        /// Version found in the input.
        found: u16,

        /// Highest version supported by this implementation.
        supported: u16,
    },

    /// Unknown container flags were encountered.
    UnsupportedFlags {
        /// Flags found in the input.
        found: u16,

        /// Flags recognized by this implementation.
        supported: u16,
    },

    /// The declared payload exceeds the caller's configured resource policy.
    PayloadLimitExceeded {
        /// Payload size declared by the container.
        declared: u64,

        /// Maximum accepted payload size.
        maximum: u64,
    },

    /// The declared payload length cannot be represented by the host allocator.
    HostSizeOverflow {
        /// Declared wire payload length.
        length: u64,
    },

    /// The payload checksum does not match.
    ChecksumMismatch {
        /// Expected digest from the header.
        expected: [u8; CHECKSUM_LEN],

        /// Calculated digest.
        actual: [u8; CHECKSUM_LEN],
    },

    /// Bytes remained after the complete container.
    TrailingBytes {
        /// Number of bytes after the declared payload.
        count: usize,
    },

    /// The serialized payload is not valid for the configured serde format.
    Deserialize(serde_json::Error),

    /// An I/O error occurred while reading a stream.
    Io(io::Error),
}

impl fmt::Display for BinaryDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TruncatedHeader => {
                formatter.write_str("truncated Zamani AST binary header")
            }

            Self::TruncatedPayload { expected, available } => write!(
                formatter,
                "truncated Zamani AST binary payload: expected {expected} bytes, found {available}"
            ),

            Self::InvalidMagic { found } => {
                write!(formatter, "invalid Zamani AST binary magic: {found:02x?}")
            }

            Self::UnsupportedVersion { found, supported } => write!(
                formatter,
                "unsupported Zamani AST binary format version {found}; supported version is {supported}"
            ),

            Self::UnsupportedFlags { found, supported } => write!(
                formatter,
                "unsupported Zamani AST binary flags 0x{found:04x}; supported flags are 0x{supported:04x}"
            ),

            Self::PayloadLimitExceeded {
                declared,
                maximum,
            } => write!(
                formatter,
                "Zamani AST binary payload of {declared} bytes exceeds configured limit of {maximum} bytes"
            ),

            Self::HostSizeOverflow { length } => write!(
                formatter,
                "Zamani AST binary payload length {length} cannot be represented by the host"
            ),

            Self::ChecksumMismatch { expected, actual } => write!(
                formatter,
                "Zamani AST binary payload checksum mismatch: expected {expected:02x?}, calculated {actual:02x?}"
            ),

            Self::TrailingBytes { count } => {
                write!(formatter, "{count} trailing bytes after Zamani AST binary container")
            }

            Self::Deserialize(error) => {
                write!(formatter, "failed to deserialize AST payload: {error}")
            }

            Self::Io(error) => write!(formatter, "failed to read AST binary container: {error}"),
        }
    }
}

impl std::error::Error for BinaryDecodeError {}

impl From<io::Error> for BinaryDecodeError {
    #[inline]
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Encodes a serializable AST value into the Zamani binary container.
///
/// The function is generic over the AST type and therefore does not need to
/// know about declarations, expressions, quantum resources, or future AST
/// extensions.
///
/// # Determinism
///
/// Determinism follows the serialized value's serde representation. The binary
/// framing itself is deterministic.
///
/// # Scalability
///
/// There is no application-defined maximum payload size here. The practical
/// limit is the resources available to the process and the limits of the wire
/// representation.
///
/// # Example
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// # #[derive(Debug, Serialize, Deserialize, PartialEq)]
/// # struct Example {
/// #     value: String,
/// # }
/// # let value = Example {
/// #     value: "Zamani".to_owned(),
/// # };
/// let encoded = zamani::frontend::ast::node::serialization::binary::encode(&value)
///     .expect("AST serialization");
/// # let decoded: Example =
/// #     zamani::frontend::ast::node::serialization::binary::decode(&encoded)
/// #         .expect("AST deserialization");
/// # assert_eq!(value, decoded);
/// ```
pub fn encode<T>(value: &T) -> Result<Vec<u8>, BinaryEncodeError>
where
    T: Serialize,
{
    let payload = serde_json::to_vec(value)?;

    encode_payload(&payload)
}

/// Encodes an already serialized AST payload into the binary container.
///
/// This is useful when another serialization layer already owns the serde
/// operation and the binary layer should only provide framing/integrity.
pub fn encode_payload(payload: &[u8]) -> Result<Vec<u8>, BinaryEncodeError> {
    let payload_len = u64::try_from(payload.len())
        .map_err(|_| BinaryEncodeError::PayloadTooLarge {
            length: payload.len(),
        })?;

    let header = BinaryHeader {
        payload_len,
        ..BinaryHeader::for_payload(payload)
    };

    let total_len = BINARY_HEADER_LEN
        .checked_add(payload.len())
        .ok_or(BinaryEncodeError::PayloadTooLarge {
            length: payload.len(),
        })?;

    let mut output = Vec::with_capacity(total_len);

    write_header(&mut output, &header);
    output.extend_from_slice(payload);

    Ok(output)
}

/// Decodes a complete binary AST container into a native AST value.
///
/// The default decoder imposes no payload-size limit.
///
/// For untrusted input, prefer [`decode_with_limits`].
pub fn decode<T>(bytes: &[u8]) -> Result<T, BinaryDecodeError>
where
    T: DeserializeOwned,
{
    decode_with_limits(bytes, BinaryDecodeLimits::unlimited())
}

/// Decodes a complete binary AST container using explicit resource limits.
///
/// The complete input must contain exactly one binary container. Trailing bytes
/// are rejected so concatenation, accidental truncation, or protocol framing
/// mistakes cannot silently change the interpreted input.
pub fn decode_with_limits<T>(
    bytes: &[u8],
    limits: BinaryDecodeLimits,
) -> Result<T, BinaryDecodeError>
where
    T: DeserializeOwned,
{
    let payload = decode_payload_with_limits(bytes, limits)?;

    serde_json::from_slice(payload).map_err(BinaryDecodeError::Deserialize)
}

/// Decodes and validates the binary envelope, returning the serialized payload.
///
/// This function does not interpret the payload as an AST. It is therefore
/// useful for generic transport/storage layers.
pub fn decode_payload(bytes: &[u8]) -> Result<&[u8], BinaryDecodeError> {
    decode_payload_with_limits(bytes, BinaryDecodeLimits::unlimited())
}

/// Decodes and validates the binary envelope with explicit limits.
pub fn decode_payload_with_limits(
    bytes: &[u8],
    limits: BinaryDecodeLimits,
) -> Result<&[u8], BinaryDecodeError> {
    if bytes.len() < BINARY_HEADER_LEN {
        return Err(BinaryDecodeError::TruncatedHeader);
    }

    let header = read_header(bytes)?;

    if !header.has_valid_magic() {
        return Err(BinaryDecodeError::InvalidMagic {
            found: header.magic,
        });
    }

    if header.version != BINARY_FORMAT_VERSION {
        return Err(BinaryDecodeError::UnsupportedVersion {
            found: header.version,
            supported: BINARY_FORMAT_VERSION,
        });
    }

    if header.flags != BINARY_FLAGS_NONE {
        return Err(BinaryDecodeError::UnsupportedFlags {
            found: header.flags,
            supported: BINARY_FLAGS_NONE,
        });
    }

    limits.check_payload_length(header.payload_len)?;

    let payload_len =
        usize::try_from(header.payload_len).map_err(|_| BinaryDecodeError::HostSizeOverflow {
            length: header.payload_len,
        })?;

    let required_len = BINARY_HEADER_LEN
        .checked_add(payload_len)
        .ok_or(BinaryDecodeError::HostSizeOverflow {
            length: header.payload_len,
        })?;

    if bytes.len() < required_len {
        return Err(BinaryDecodeError::TruncatedPayload {
            expected: header.payload_len,
            available: bytes.len().saturating_sub(BINARY_HEADER_LEN),
        });
    }

    if bytes.len() > required_len {
        return Err(BinaryDecodeError::TrailingBytes {
            count: bytes.len() - required_len,
        });
    }

    let payload = &bytes[BINARY_HEADER_LEN..required_len];

    let actual_digest = Sha256::digest(payload);
    let mut actual = [0u8; CHECKSUM_LEN];
    actual.copy_from_slice(&actual_digest);

    if actual != header.checksum {
        return Err(BinaryDecodeError::ChecksumMismatch {
            expected: header.checksum,
            actual,
        });
    }

    Ok(payload)
}

/// Decodes a binary AST container from a reader.
///
/// This method is appropriate when AST bytes arrive from a file, socket, pipe,
/// or another stream rather than an already materialized byte slice.
///
/// Because a stream has no inherent end-of-container marker beyond the declared
/// payload length, this function reads exactly one complete container.
///
/// The caller's configured payload limit is enforced before allocating the
/// payload buffer.
pub fn decode_reader<T, R>(
    reader: &mut R,
    limits: BinaryDecodeLimits,
) -> Result<T, BinaryDecodeError>
where
    T: DeserializeOwned,
    R: Read,
{
    let header = read_header_from_reader(reader)?;

    if !header.has_valid_magic() {
        return Err(BinaryDecodeError::InvalidMagic {
            found: header.magic,
        });
    }

    if header.version != BINARY_FORMAT_VERSION {
        return Err(BinaryDecodeError::UnsupportedVersion {
            found: header.version,
            supported: BINARY_FORMAT_VERSION,
        });
    }

    if header.flags != BINARY_FLAGS_NONE {
        return Err(BinaryDecodeError::UnsupportedFlags {
            found: header.flags,
            supported: BINARY_FLAGS_NONE,
        });
    }

    limits.check_payload_length(header.payload_len)?;

    let payload_len =
        usize::try_from(header.payload_len).map_err(|_| BinaryDecodeError::HostSizeOverflow {
            length: header.payload_len,
        })?;

    let mut payload = Vec::new();
    payload
        .try_reserve_exact(payload_len)
        .map_err(|_| BinaryDecodeError::HostSizeOverflow {
            length: header.payload_len,
        })?;
    payload.resize(payload_len, 0);

    reader.read_exact(&mut payload)?;

    let actual_digest = Sha256::digest(&payload);
    let mut actual = [0u8; CHECKSUM_LEN];
    actual.copy_from_slice(&actual_digest);

    if actual != header.checksum {
        return Err(BinaryDecodeError::ChecksumMismatch {
            expected: header.checksum,
            actual,
        });
    }

    serde_json::from_slice(&payload).map_err(BinaryDecodeError::Deserialize)
}

/// Reads a validated binary header from an in-memory byte slice.
///
/// This function only reads the fixed-size header. Payload validation remains
/// the responsibility of [`decode_payload_with_limits`].
pub fn read_header(bytes: &[u8]) -> Result<BinaryHeader, BinaryDecodeError> {
    if bytes.len() < BINARY_HEADER_LEN {
        return Err(BinaryDecodeError::TruncatedHeader);
    }

    let mut magic = [0u8; 8];
    magic.copy_from_slice(&bytes[0..8]);

    let version = u16::from_le_bytes([bytes[8], bytes[9]]);
    let flags = u16::from_le_bytes([bytes[10], bytes[11]]);

    let mut payload_len_bytes = [0u8; 8];
    payload_len_bytes.copy_from_slice(&bytes[12..20]);
    let payload_len = u64::from_le_bytes(payload_len_bytes);

    let mut checksum = [0u8; CHECKSUM_LEN];
    checksum.copy_from_slice(&bytes[20..52]);

    Ok(BinaryHeader {
        magic,
        version,
        flags,
        payload_len,
        checksum,
    })
}

/// Returns the fixed-size binary header representation.
///
/// This is primarily exposed for integration tests, diagnostics, and streaming
/// implementations.
pub fn encode_header(header: &BinaryHeader) -> [u8; BINARY_HEADER_LEN] {
    let mut output = [0u8; BINARY_HEADER_LEN];

    output[0..8].copy_from_slice(&header.magic);
    output[8..10].copy_from_slice(&header.version.to_le_bytes());
    output[10..12].copy_from_slice(&header.flags.to_le_bytes());
    output[12..20].copy_from_slice(&header.payload_len.to_le_bytes());
    output[20..52].copy_from_slice(&header.checksum);

    output
}

/// Writes a binary header to an output vector.
fn write_header(output: &mut Vec<u8>, header: &BinaryHeader) {
    output.extend_from_slice(&encode_header(header));
}

/// Reads a binary header directly from a reader.
///
/// Exactly [`BINARY_HEADER_LEN`] bytes are consumed.
fn read_header_from_reader<R>(reader: &mut R) -> Result<BinaryHeader, BinaryDecodeError>
where
    R: Read,
{
    let mut bytes = [0u8; BINARY_HEADER_LEN];

    match reader.read_exact(&mut bytes) {
        Ok(()) => read_header(&bytes),
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => {
            Err(BinaryDecodeError::TruncatedHeader)
        }
        Err(error) => Err(BinaryDecodeError::Io(error)),
    }
}

/// Returns the checksum of an AST payload.
///
/// This function is provided for callers that need to compare or cache
/// serialized payloads without constructing the complete binary container.
#[inline]
pub fn payload_checksum(payload: &[u8]) -> [u8; CHECKSUM_LEN] {
    let digest = Sha256::digest(payload);
    let mut checksum = [0u8; CHECKSUM_LEN];
    checksum.copy_from_slice(&digest);
    checksum
}

/// Returns the checksum of a complete binary AST container.
///
/// The container must first be structurally valid. This function intentionally
/// returns a validation error rather than attempting to guess which bytes are
/// the payload.
pub fn container_checksum(bytes: &[u8]) -> Result<[u8; CHECKSUM_LEN], BinaryDecodeError> {
    let payload = decode_payload(bytes)?;
    Ok(payload_checksum(payload))
}

/// Validates a binary AST container without deserializing its payload.
///
/// This is useful for caches and transport layers that only need to establish
/// framing and integrity before handing the payload to another subsystem.
pub fn validate(bytes: &[u8]) -> Result<BinaryHeader, BinaryDecodeError> {
    let payload = decode_payload(bytes)?;
    let header = read_header(bytes)?;

    // `decode_payload` already checks this, but keeping the comparison here
    // makes this function's postcondition explicit.
    debug_assert_eq!(header.payload_len as usize, payload.len());

    Ok(header)
}

/// Validates a binary AST container using explicit resource limits.
pub fn validate_with_limits(
    bytes: &[u8],
    limits: BinaryDecodeLimits,
) -> Result<BinaryHeader, BinaryDecodeError> {
    let payload = decode_payload_with_limits(bytes, limits)?;
    let header = read_header(bytes)?;

    debug_assert_eq!(
        usize::try_from(header.payload_len).ok(),
        Some(payload.len())
    );

    Ok(header)
}

/// Reads one binary container from an in-memory cursor.
///
/// This helper is useful for tests and small in-memory consumers. It does not
/// provide a different serialization format.
pub fn decode_cursor<T>(
    cursor: &mut Cursor<&[u8]>,
    limits: BinaryDecodeLimits,
) -> Result<T, BinaryDecodeError>
where
    T: DeserializeOwned,
{
    decode_reader(cursor, limits)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    struct TestAst {
        id: u64,
        name: String,
        values: Vec<u64>,
    }

    fn test_ast() -> TestAst {
        TestAst {
            id: 42,
            name: "Zamani".to_owned(),
            values: vec![1, 2, 3, 5, 8, 13],
        }
    }

    #[test]
    fn round_trip_preserves_value() {
        let original = test_ast();

        let encoded = encode(&original).expect("encoding must succeed");
        let decoded: TestAst = decode(&encoded).expect("decoding must succeed");

        assert_eq!(decoded, original);
    }

    #[test]
    fn format_has_expected_magic() {
        let encoded = encode(&test_ast()).expect("encoding must succeed");

        assert_eq!(&encoded[..BINARY_MAGIC.len()], &BINARY_MAGIC);
    }

    #[test]
    fn header_is_deterministic() {
        let first = encode(&test_ast()).expect("first encoding");
        let second = encode(&test_ast()).expect("second encoding");

        assert_eq!(first, second);
    }

    #[test]
    fn checksum_detects_payload_corruption() {
        let mut encoded = encode(&test_ast()).expect("encoding must succeed");

        let payload_start = BINARY_HEADER_LEN;
        encoded[payload_start] ^= 0x01;

        let error = decode::<TestAst>(&encoded).expect_err("corruption must fail");

        assert!(matches!(
            error,
            BinaryDecodeError::ChecksumMismatch { .. }
        ));
    }

    #[test]
    fn checksum_detects_header_payload_length_tampering() {
        let mut encoded = encode(&test_ast()).expect("encoding must succeed");

        // Change payload length while keeping the actual payload unchanged.
        let original_length = u64::from_le_bytes([
            encoded[12],
            encoded[13],
            encoded[14],
            encoded[15],
            encoded[16],
            encoded[17],
            encoded[18],
            encoded[19],
        ]);

        let altered_length = original_length.saturating_add(1);
        encoded[12..20].copy_from_slice(&altered_length.to_le_bytes());

        let error = decode::<TestAst>(&encoded).expect_err("tampering must fail");

        assert!(matches!(
            error,
            BinaryDecodeError::TruncatedPayload { .. }
        ));
    }

    #[test]
    fn rejects_trailing_bytes() {
        let mut encoded = encode(&test_ast()).expect("encoding must succeed");
        encoded.push(0);

        let error = decode::<TestAst>(&encoded).expect_err("trailing bytes must fail");

        assert!(matches!(
            error,
            BinaryDecodeError::TrailingBytes { count: 1 }
        ));
    }

    #[test]
    fn rejects_truncated_header() {
        let encoded = vec![0u8; BINARY_HEADER_LEN - 1];

        let error = decode::<TestAst>(&encoded).expect_err("header truncation must fail");

        assert!(matches!(error, BinaryDecodeError::TruncatedHeader));
    }

    #[test]
    fn rejects_truncated_payload() {
        let encoded = encode(&test_ast()).expect("encoding must succeed");
        let truncated = &encoded[..encoded.len() - 1];

        let error = decode::<TestAst>(truncated).expect_err("payload truncation must fail");

        assert!(matches!(
            error,
            BinaryDecodeError::TruncatedPayload { .. }
        ));
    }

    #[test]
    fn rejects_invalid_magic() {
        let mut encoded = encode(&test_ast()).expect("encoding must succeed");
        encoded[0] ^= 0xff;

        let error = decode::<TestAst>(&encoded).expect_err("invalid magic must fail");

        assert!(matches!(error, BinaryDecodeError::InvalidMagic { .. }));
    }

    #[test]
    fn rejects_unsupported_version() {
        let mut encoded = encode(&test_ast()).expect("encoding must succeed");
        encoded[8..10].copy_from_slice(&2u16.to_le_bytes());

        let error = decode::<TestAst>(&encoded).expect_err("unsupported version must fail");

        assert!(matches!(
            error,
            BinaryDecodeError::UnsupportedVersion {
                found: 2,
                supported: BINARY_FORMAT_VERSION
            }
        ));
    }

    #[test]
    fn rejects_unknown_flags() {
        let mut encoded = encode(&test_ast()).expect("encoding must succeed");
        encoded[10..12].copy_from_slice(&1u16.to_le_bytes());

        let error = decode::<TestAst>(&encoded).expect_err("unknown flags must fail");

        assert!(matches!(
            error,
            BinaryDecodeError::UnsupportedFlags {
                found: 1,
                supported: BINARY_FLAGS_NONE
            }
        ));
    }

    #[test]
    fn configured_payload_limit_is_enforced_before_deserialization() {
        let encoded = encode(&test_ast()).expect("encoding must succeed");

        let limit = BinaryDecodeLimits::with_max_payload_bytes(
            u64::try_from(encoded.len() - BINARY_HEADER_LEN)
                .expect("test payload length fits u64")
                .saturating_sub(1),
        );

        let error =
            decode_with_limits::<TestAst>(&encoded, limit).expect_err("limit must be enforced");

        assert!(matches!(
            error,
            BinaryDecodeError::PayloadLimitExceeded { .. }
        ));
    }

    #[test]
    fn unlimited_mode_accepts_payload() {
        let encoded = encode(&test_ast()).expect("encoding must succeed");

        let decoded: TestAst = decode_with_limits(
            &encoded,
            BinaryDecodeLimits::unlimited(),
        )
        .expect("unlimited decoding must succeed");

        assert_eq!(decoded, test_ast());
    }

    #[test]
    fn payload_helpers_round_trip() {
        let payload = serde_json::to_vec(&test_ast()).expect("payload serialization");
        let encoded = encode_payload(&payload).expect("container encoding");

        let recovered =
            decode_payload(&encoded).expect("container decoding");

        assert_eq!(recovered, payload.as_slice());
    }

    #[test]
    fn payload_checksum_is_stable() {
        let payload = serde_json::to_vec(&test_ast()).expect("payload serialization");

        assert_eq!(payload_checksum(&payload), payload_checksum(&payload));
    }

    #[test]
    fn container_checksum_matches_header_checksum() {
        let encoded = encode(&test_ast()).expect("encoding must succeed");
        let header = read_header(&encoded).expect("header");

        assert_eq!(
            container_checksum(&encoded).expect("checksum"),
            header.checksum
        );
    }

    #[test]
    fn reader_round_trip_works() {
        let encoded = encode(&test_ast()).expect("encoding must succeed");
        let mut cursor = Cursor::new(encoded.as_slice());

        let decoded: TestAst =
            decode_cursor(&mut cursor, BinaryDecodeLimits::unlimited())
                .expect("reader decoding");

        assert_eq!(decoded, test_ast());
    }

    #[test]
    fn header_encoding_round_trips() {
        let encoded = encode(&test_ast()).expect("encoding must succeed");
        let header = read_header(&encoded).expect("header");

        let reencoded = encode_header(&header);

        assert_eq!(&encoded[..BINARY_HEADER_LEN], &reencoded);
    }

    #[test]
    fn header_reports_encoded_length() {
        let encoded = encode(&test_ast()).expect("encoding must succeed");
        let header = read_header(&encoded).expect("header");

        assert_eq!(header.encoded_len(), Some(encoded.len()));
    }

    #[test]
    fn zero_length_payload_is_supported() {
        let encoded = encode_payload(&[]).expect("empty payload encoding");

        let payload = decode_payload(&encoded).expect("empty payload decoding");

        assert!(payload.is_empty());
    }

    #[test]
    fn validation_does_not_deserialize_payload() {
        let payload = b"not-an-AST-but-valid-container-payload";
        let encoded = encode_payload(payload).expect("container encoding");

        let header = validate(&encoded).expect("container must validate");

        assert_eq!(header.payload_len, payload.len() as u64);
    }

    #[test]
    fn binary_format_does_not_impose_ast_semantic_limits() {
        let value = TestAst {
            id: u64::MAX,
            name: "arbitrarily represented resource-independent program".to_owned(),
            values: vec![u64::MAX, 0, 1],
        };

        let encoded = encode(&value).expect("encoding must succeed");
        let decoded: TestAst = decode(&encoded).expect("decoding must succeed");

        assert_eq!(decoded, value);
    }
}