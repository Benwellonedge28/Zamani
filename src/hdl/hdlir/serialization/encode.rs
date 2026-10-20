
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Deterministic HDLIR artifact encoder
//!
//! Path: `src/hdl/hdlir/serialization/encode.rs`
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Encode a versioned HDLIR artifact envelope and an already-encoded payload
//! into a deterministic byte representation.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `grammar/Zamani.g4`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/serialization/format.rs`
//!
//! ## OWNS
//!
//! - Stable framing of artifact metadata and payload bytes.
//! - Deterministic JSON encoding of the existing artifact metadata type.
//! - Optional SHA-256 payload-integrity calculation.
//! - Checked framing arithmetic and explicit encoding budgets.
//! - Safe writing to caller-provided output sinks.
//!
//! ## DOES NOT OWN
//!
//! - HDL source parsing, source-level semantics, or grammar interpretation.
//! - The HDLIR graph or its verification.
//! - Payload-specific serialization or canonicalization.
//! - Schema and dialect registration or compatibility migration.
//! - Physical hardware allocation, synthesis, placement, or routing.
//!
//! ## WIRE CONTRACT
//!
//! ```text
//! +-------------------------+-------------------------------+
//! | Field                   | Encoding                      |
//! +-------------------------+-------------------------------+
//! | Magic                   | 8 bytes: ZHDLIR followed by CR |
//! |                         | LF                            |
//! | Header length           | unsigned 64-bit, big-endian   |
//! | Payload length          | unsigned 64-bit, big-endian   |
//! | Metadata header         | compact UTF-8 JSON            |
//! | Payload                 | exact caller-supplied bytes   |
//! +-------------------------+-------------------------------+
//! ```
//!
//! The header contains `ArtifactFormat`. Its payload integrity digest, when
//! present, is SHA-256 over the exact payload bytes and excludes the header
//! and framing prefix.
//!
//! The framing is a proposed initial wire contract. Before persisted artifacts
//! are released, ratify it in `HDLIR-SPEC.md` and `compatibility.md` and ensure
//! the decoder implements the identical framing.
//!
//! ## SCALABILITY
//!
//! There are no hardware-count, width, module-count, or payload-size constants
//! in this encoder. Optional limits are operational budgets, not HDL language
//! restrictions. The caller can choose limits appropriate to its environment.
//!
//! Finite software remains subject to address-space, allocation, and I/O
//! constraints. All representable framing arithmetic is checked.
//!
//! ## INTEGRATION
//!
//! `serialization/mod.rs` should expose this module and re-export only the
//! approved public functions and types. The decoder must use the same magic,
//! length encoding, header encoding, and integrity coverage.
//!
//! The caller supplies payload bytes produced by the canonical HDLIR payload
//! codec. This module cannot establish that those bytes are canonical,
//! verified, semantically complete, or safe to realize.
//!
//! ## FREEZE CRITERIA
//!
//! - The wire contract is ratified and matched by the decoder.
//! - Metadata and framing tests pass.
//! - Budget exhaustion and sink failures remain explicit.
//! - Round trips preserve the exact payload bytes.
//! - Canonical output is stable for equivalent metadata and payload bytes.
//! - The actual repository module tree compiles and required checks pass.

use core::fmt;
use serde_json::Error as JsonError;
use sha2::{Digest, Sha256};
use std::io::{self, Write};

use crate::hdl::hdlir::serialization::format::{
    ArtifactFormat, FormatError, IntegrityAlgorithm, PayloadIntegrity,
};

/// Eight-byte magic prefix identifying a Zamani HDLIR artifact.
///
/// The format version is carried in the metadata header, not inferred from
/// this magic prefix.
pub const HDLIR_ARTIFACT_MAGIC: &[u8; 8] = b"ZHDLIR\r\n";

/// Number of bytes in the fixed framing prefix.
///
/// This is the magic prefix plus the two fixed-width length fields.
pub const FRAME_PREFIX_LEN: usize = 8 + 8 + 8;

/// Configuration for encoding an HDLIR artifact.
///
/// All limits are optional. `None` means that this encoder imposes no
/// additional limit for that category; it does not promise that the host can
/// allocate or write an artifact of arbitrary size.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncodeOptions {
    max_header_bytes: Option<usize>,
    max_payload_bytes: Option<usize>,
    max_artifact_bytes: Option<usize>,
    include_payload_integrity: bool,
}

impl EncodeOptions {
    /// Create encoding options with no explicit size budgets and with
    /// SHA-256 payload integrity enabled.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_header_bytes: None,
            max_payload_bytes: None,
            max_artifact_bytes: None,
            include_payload_integrity: true,
        }
    }

    /// Set the maximum encoded metadata-header size.
    ///
    /// The limit applies to the compact JSON header, excluding framing and
    /// payload bytes.
    #[must_use]
    pub const fn with_max_header_bytes(mut self, limit: Option<usize>) -> Self {
        self.max_header_bytes = limit;
        self
    }

    /// Set the maximum payload size.
    ///
    /// The limit applies to the exact caller-supplied payload bytes.
    #[must_use]
    pub const fn with_max_payload_bytes(mut self, limit: Option<usize>) -> Self {
        self.max_payload_bytes = limit;
        self
    }

    /// Set the maximum total artifact size, including framing.
    #[must_use]
    pub const fn with_max_artifact_bytes(mut self, limit: Option<usize>) -> Self {
        self.max_artifact_bytes = limit;
        self
    }

    /// Choose whether to add SHA-256 payload integrity metadata.
    ///
    /// If the supplied format already declares integrity metadata, its digest
    /// is recomputed regardless of this setting. This prevents an old digest
    /// from being emitted for different payload bytes.
    #[must_use]
    pub const fn with_payload_integrity(mut self, enabled: bool) -> Self {
        self.include_payload_integrity = enabled;
        self
    }

    /// Return the configured metadata-header limit.
    #[must_use]
    pub const fn max_header_bytes(self) -> Option<usize> {
        self.max_header_bytes
    }

    /// Return the configured payload limit.
    #[must_use]
    pub const fn max_payload_bytes(self) -> Option<usize> {
        self.max_payload_bytes
    }

    /// Return the configured total-artifact limit.
    #[must_use]
    pub const fn max_artifact_bytes(self) -> Option<usize> {
        self.max_artifact_bytes
    }

    /// Return whether integrity metadata is requested by default.
    #[must_use]
    pub const fn includes_payload_integrity(self) -> bool {
        self.include_payload_integrity
    }
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Failures produced while encoding an HDLIR artifact.
#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError {
    /// Artifact metadata violates its declared format contract.
    InvalidFormat(FormatError),

    /// Metadata could not be encoded as JSON.
    MetadataEncoding(JsonError),

    /// A requested operational size budget was exceeded.
    LimitExceeded {
        /// Name of the limited resource.
        resource: &'static str,
        /// Configured maximum.
        limit: usize,
        /// Observed or required size.
        actual: usize,
    },

    /// A size could not be represented in the required integer type.
    LengthNotRepresentable {
        /// Name of the length being converted.
        resource: &'static str,
    },

    /// Checked arithmetic overflowed while computing an encoded size.
    SizeOverflow,

    /// A memory allocation requested by this encoder failed.
    AllocationFailed {
        /// Requested or required capacity, when known.
        requested: usize,
    },

    /// Writing the artifact to the supplied sink failed.
    ///
    /// A sink may have accepted a prefix before returning this error. Callers
    /// must discard or otherwise recover the partial output as appropriate.
    WriteFailed(io::Error),
}

impl fmt::Display for EncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(error) => {
                write!(formatter, "invalid HDLIR artifact format: {error}")
            }
            Self::MetadataEncoding(error) => {
                write!(formatter, "could not encode HDLIR metadata: {error}")
            }
            Self::LimitExceeded {
                resource,
                limit,
                actual,
            } => write!(
                formatter,
                "HDLIR encoding budget exceeded for {resource}: \
                 required {actual} bytes, limit is {limit} bytes"
            ),
            Self::LengthNotRepresentable { resource } => {
                write!(formatter, "{resource} length cannot be represented on this platform")
            }
            Self::SizeOverflow => {
                formatter.write_str("HDLIR artifact size arithmetic overflowed")
            }
            Self::AllocationFailed { requested } => write!(
                formatter,
                "could not allocate {requested} bytes while encoding HDLIR"
            ),
            Self::WriteFailed(error) => {
                write!(formatter, "could not write HDLIR artifact: {error}")
            }
        }
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidFormat(error) => Some(error),
            Self::MetadataEncoding(error) => Some(error),
            Self::WriteFailed(error) => Some(error),
            Self::LimitExceeded { .. }
            | Self::LengthNotRepresentable { .. }
            | Self::SizeOverflow
            | Self::AllocationFailed { .. } => None,
        }
    }
}

/// Prepared artifact metadata and its computed framing lengths.
struct PreparedArtifact {
    header: Vec<u8>,
    header_len: u64,
    payload_len: u64,
    total_len: usize,
}

/// A JSON output buffer that enforces its optional size limit while writing.
///
/// This prevents metadata serialization from growing the header buffer beyond
/// the configured header budget.
struct CappedJsonBuffer {
    bytes: Vec<u8>,
    limit: Option<usize>,
    exceeded: Option<usize>,
    allocation_failed: Option<usize>,
    size_overflowed: bool,
}

impl CappedJsonBuffer {
    fn new(limit: Option<usize>) -> Self {
        Self {
            bytes: Vec::new(),
            limit,
            exceeded: None,
            allocation_failed: None,
            size_overflowed: false,
        }
    }

    fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for CappedJsonBuffer {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        let Some(required_len) = self.bytes.len().checked_add(input.len()) else {
            self.size_overflowed = true;
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "metadata length overflow",
            ));
        };

        if let Some(limit) = self.limit {
            if required_len > limit {
                self.exceeded = Some(required_len);
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "metadata size budget exceeded",
                ));
            }
        }

        if self.bytes.try_reserve(input.len()).is_err() {
            self.allocation_failed = Some(required_len);
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "metadata buffer allocation failed",
            ));
        }

        self.bytes.extend_from_slice(input);
        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Encode an artifact into a newly allocated byte vector.
///
/// The supplied payload must already have been encoded by the canonical
/// payload codec. Its bytes are preserved exactly.
///
/// Metadata is validated before encoding. When integrity is enabled, or the
/// metadata already declares an integrity digest, the digest is recomputed
/// over the supplied payload bytes.
///
/// # Errors
///
/// Returns an error if metadata is invalid, a configured budget is exceeded,
/// a framing length cannot be represented, arithmetic overflows, or a
/// required allocation fails.
pub fn encode_artifact(
    format: &ArtifactFormat,
    payload: &[u8],
    options: EncodeOptions,
) -> Result<Vec<u8>, EncodeError> {
    let prepared = prepare_artifact(format, payload, options)?;

    let mut output = Vec::new();
    output
        .try_reserve_exact(prepared.total_len)
        .map_err(|_| EncodeError::AllocationFailed {
            requested: prepared.total_len,
        })?;

    write_prepared(&mut output, &prepared, payload)?;
    Ok(output)
}

/// Encode an artifact directly into a caller-provided output sink.
///
/// This avoids allocating a second buffer for the complete artifact. The
/// payload itself is a borrowed byte slice, so the caller retains ownership
/// of its payload storage.
///
/// A failed write may leave a partial artifact in the sink. Callers writing
/// to a file should use a temporary file and commit it only after this
/// function succeeds if atomic replacement is required.
///
/// # Errors
///
/// Returns an error if metadata is invalid, a configured budget is exceeded,
/// a framing length cannot be represented, arithmetic overflows, metadata
/// allocation fails, or the sink returns an I/O error.
pub fn encode_artifact_to<W: Write>(
    sink: &mut W,
    format: &ArtifactFormat,
    payload: &[u8],
    options: EncodeOptions,
) -> Result<(), EncodeError> {
    let prepared = prepare_artifact(format, payload, options)?;
    write_prepared(sink, &prepared, payload)
}

fn prepare_artifact(
    format: &ArtifactFormat,
    payload: &[u8],
    options: EncodeOptions,
) -> Result<PreparedArtifact, EncodeError> {
    format.validate().map_err(EncodeError::InvalidFormat)?;

    enforce_limit(
        "payload",
        options.max_payload_bytes,
        payload.len(),
    )?;

    let payload_len =
        u64::try_from(payload.len()).map_err(|_| EncodeError::LengthNotRepresentable {
            resource: "payload",
        })?;

    let mut output_format = format.clone();

    let integrity_required =
        options.include_payload_integrity || format.integrity().is_some();

    if integrity_required {
        let digest = Sha256::digest(payload);
        let digest_hex = hex::encode(digest);
        let integrity =
            PayloadIntegrity::new(IntegrityAlgorithm::Sha256, digest_hex)
                .map_err(EncodeError::InvalidFormat)?;

        output_format.set_integrity(Some(integrity));
    } else {
        output_format.set_integrity(None);
    }

    output_format
        .validate()
        .map_err(EncodeError::InvalidFormat)?;

    let mut json_buffer = CappedJsonBuffer::new(options.max_header_bytes);
    let json_result = serde_json::to_writer(&mut json_buffer, &output_format);

    if let Some(actual) = json_buffer.exceeded {
        return Err(EncodeError::LimitExceeded {
            resource: "metadata header",
            limit: options
                .max_header_bytes
                .expect("exceeded is only set when a limit exists"),
            actual,
        });
    }

    if let Some(requested) = json_buffer.allocation_failed {
        return Err(EncodeError::AllocationFailed { requested });
    }

    if json_buffer.size_overflowed {
        return Err(EncodeError::SizeOverflow);
    }

    json_result.map_err(EncodeError::MetadataEncoding)?;

    let header = json_buffer.into_inner();

    let header_len =
        u64::try_from(header.len()).map_err(|_| EncodeError::LengthNotRepresentable {
            resource: "metadata header",
        })?;

    let total_len = FRAME_PREFIX_LEN
        .checked_add(header.len())
        .and_then(|size| size.checked_add(payload.len()))
        .ok_or(EncodeError::SizeOverflow)?;

    enforce_limit(
        "complete artifact",
        options.max_artifact_bytes,
        total_len,
    )?;

    Ok(PreparedArtifact {
        header,
        header_len,
        payload_len,
        total_len,
    })
}

fn enforce_limit(
    resource: &'static str,
    limit: Option<usize>,
    actual: usize,
) -> Result<(), EncodeError> {
    if let Some(limit) = limit {
        if actual > limit {
            return Err(EncodeError::LimitExceeded {
                resource,
                limit,
                actual,
            });
        }
    }

    Ok(())
}

fn write_prepared<W: Write>(
    sink: &mut W,
    prepared: &PreparedArtifact,
    payload: &[u8],
) -> Result<(), EncodeError> {
    sink.write_all(HDLIR_ARTIFACT_MAGIC)
        .map_err(EncodeError::WriteFailed)?;

    sink.write_all(&prepared.header_len.to_be_bytes())
        .map_err(EncodeError::WriteFailed)?;

    sink.write_all(&prepared.payload_len.to_be_bytes())
        .map_err(EncodeError::WriteFailed)?;

    sink.write_all(&prepared.header)
        .map_err(EncodeError::WriteFailed)?;

    sink.write_all(payload)
        .map_err(EncodeError::WriteFailed)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::serialization::format::ArtifactFormat;

    fn read_u64_be(bytes: &[u8]) -> u64 {
        let array: [u8; 8] = bytes
            .try_into()
            .expect("test input must contain exactly eight bytes");
        u64::from_be_bytes(array)
    }

    fn decode_test_frame(bytes: &[u8]) -> (&[u8], &[u8]) {
        assert!(bytes.len() >= FRAME_PREFIX_LEN);
        assert_eq!(&bytes[..8], HDLIR_ARTIFACT_MAGIC);

        let header_len = usize::try_from(read_u64_be(&bytes[8..16]))
            .expect("test header length must fit usize");
        let payload_len = usize::try_from(read_u64_be(&bytes[16..24]))
            .expect("test payload length must fit usize");

        let header_start = FRAME_PREFIX_LEN;
        let payload_start = header_start
            .checked_add(header_len)
            .expect("test frame length must not overflow");
        let end = payload_start
            .checked_add(payload_len)
            .expect("test frame length must not overflow");

        assert_eq!(end, bytes.len());

        (&bytes[header_start..payload_start], &bytes[payload_start..end])
    }

    #[test]
    fn encoding_is_deterministic_for_identical_inputs() {
        let format = ArtifactFormat::new();
        let payload = b"canonical HDLIR payload";

        let first = encode_artifact(&format, payload, EncodeOptions::default())
            .expect("first encoding should succeed");
        let second = encode_artifact(&format, payload, EncodeOptions::default())
            .expect("second encoding should succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn framing_preserves_payload_bytes_exactly() {
        let format = ArtifactFormat::new();
        let payload = [0_u8, 1, 2, 0, 255, 128, 64];

        let encoded = encode_artifact(&format, &payload, EncodeOptions::default())
            .expect("encoding should succeed");

        let (header_bytes, decoded_payload) = decode_test_frame(&encoded);

        assert_eq!(decoded_payload, payload);

        let decoded_header: ArtifactFormat =
            serde_json::from_slice(header_bytes).expect("header should be valid JSON");

        decoded_header
            .validate()
            .expect("encoded header should satisfy its metadata invariants");
    }

    #[test]
    fn integrity_is_computed_from_exact_payload_bytes() {
        let format = ArtifactFormat::new();
        let payload = b"payload whose exact bytes are protected";

        let encoded = encode_artifact(&format, payload, EncodeOptions::default())
            .expect("encoding should succeed");

        let (header_bytes, decoded_payload) = decode_test_frame(&encoded);
        let decoded_header: ArtifactFormat =
            serde_json::from_slice(header_bytes).expect("header should be valid JSON");

        let integrity = decoded_header
            .integrity()
            .expect("default options enable payload integrity");

        assert_eq!(integrity.algorithm(), IntegrityAlgorithm::Sha256);
        assert_eq!(
            integrity.digest_hex(),
            hex::encode(Sha256::digest(decoded_payload))
        );
    }

    #[test]
    fn existing_integrity_is_recomputed_for_new_payload() {
        let mut format = ArtifactFormat::new();
        let original_payload = b"original";
        let original_digest = hex::encode(Sha256::digest(original_payload));

        let integrity = PayloadIntegrity::new(
            IntegrityAlgorithm::Sha256,
            original_digest,
        )
        .expect("SHA-256 digest should be valid");

        format.set_integrity(Some(integrity));

        let replacement_payload = b"replacement";
        let encoded = encode_artifact(
            &format,
            replacement_payload,
            EncodeOptions::default().with_payload_integrity(false),
        )
        .expect("encoding should recompute declared integrity");

        let (header_bytes, decoded_payload) = decode_test_frame(&encoded);
        let decoded_header: ArtifactFormat =
            serde_json::from_slice(header_bytes).expect("header should be valid JSON");

        assert_eq!(
            decoded_header
                .integrity()
                .expect("integrity should remain declared")
                .digest_hex(),
            hex::encode(Sha256::digest(decoded_payload))
        );
    }

    #[test]
    fn payload_budget_is_enforced_before_encoding() {
        let format = ArtifactFormat::new();
        let payload = b"payload";

        let result = encode_artifact(
            &format,
            payload,
            EncodeOptions::default().with_max_payload_bytes(Some(2)),
        );

        assert!(matches!(
            result,
            Err(EncodeError::LimitExceeded {
                resource: "payload",
                limit: 2,
                actual: 7,
            })
        ));
    }

    #[test]
    fn header_budget_is_enforced_during_json_serialization() {
        let format = ArtifactFormat::new();

        let result = encode_artifact(
            &format,
            b"",
            EncodeOptions::default().with_max_header_bytes(Some(1)),
        );

        assert!(matches!(
            result,
            Err(EncodeError::LimitExceeded {
                resource: "metadata header",
                limit: 1,
                ..
            })
        ));
    }

    #[test]
    fn complete_artifact_budget_includes_framing_and_header() {
        let format = ArtifactFormat::new();
        let payload = b"x";

        let unlimited =
            encode_artifact(&format, payload, EncodeOptions::default())
                .expect("unlimited encoding should succeed");

        let limit = unlimited.len() - 1;
        let result = encode_artifact(
            &format,
            payload,
            EncodeOptions::default().with_max_artifact_bytes(Some(limit)),
        );

        assert!(matches!(
            result,
            Err(EncodeError::LimitExceeded {
                resource: "complete artifact",
                ..
            })
        ));
    }

    #[test]
    fn sink_api_matches_vector_api() {
        let format = ArtifactFormat::new();
        let payload = b"streamed payload";
        let options = EncodeOptions::default();

        let expected =
            encode_artifact(&format, payload, options).expect("encoding should succeed");

        let mut sink = Vec::new();
        encode_artifact_to(&mut sink, &format, payload, options)
            .expect("writing to Vec should succeed");

        assert_eq!(sink, expected);
    }

    #[test]
    fn empty_payload_is_supported() {
        let format = ArtifactFormat::new();

        let encoded = encode_artifact(&format, b"", EncodeOptions::default())
            .expect("empty payload should be supported");

        let (header, payload) = decode_test_frame(&encoded);

        assert!(!header.is_empty());
        assert!(payload.is_empty());
    }
}
