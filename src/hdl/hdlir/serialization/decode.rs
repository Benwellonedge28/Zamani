
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Artifact Decoder
//!
//! Path: `src/hdl/hdlir/serialization/decode.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Decode the framed HDLIR artifact emitted by `serialization/encode.rs`.
//! Treat all incoming bytes and metadata as untrusted.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `grammar/Zamani.g4`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/serialization/format.rs`
//! - `src/hdl/hdlir/serialization/encode.rs`
//!
//! The grammar owns source syntax. The frontend owns source semantics.
//! ZUIR owns the universal computational representation. HDLIR owns the
//! hardware-domain representation. This module owns artifact framing,
//! untrusted-header validation, configured compatibility checks, payload
//! integrity verification, and safe extraction of payload bytes.
//!
//! ## Does not own
//!
//! - HDL parsing, AST construction, or semantic analysis.
//! - The HDLIR graph or its payload-specific codec.
//! - Schema or dialect registration.
//! - Version migrations.
//! - Synthesis, physical allocation, placement, routing, or execution.
//! - Proof that a decoded payload is valid HDLIR.
//!
//! ## Wire contract
//!
//! The decoder matches `encode.rs` exactly:
//!
//! 1. Eight-byte `ZHDLIR\r\n` magic.
//! 2. Eight-byte unsigned big-endian metadata-header length.
//! 3. Eight-byte unsigned big-endian payload length.
//! 4. UTF-8 JSON containing `ArtifactFormat`.
//! 5. Exact payload bytes.
//!
//! SHA-256, when declared, covers the exact payload bytes only.
//!
//! ## Scalability and security
//!
//! No hardware-capacity ceiling is introduced. Header, payload, and complete
//! artifact limits are optional operational budgets. Length conversions and
//! arithmetic are checked before allocation. Allocations use `try_reserve`.
//!
//! Decoding an artifact requires enough memory for the returned payload.
//! The reader API avoids buffering the entire framed artifact, but it still
//! returns an owned payload. For payloads too large to retain in memory, a
//! future streaming payload-codec API should consume a bounded payload reader
//! directly rather than using this owned-payload API.
//!
//! Reader-based decoding consumes one byte after the payload to verify EOF.
//! Therefore the reader must represent one complete artifact stream. Use the
//! slice-based API when trailing bytes belong to an enclosing protocol.
//!
//! A successful decode establishes framing and metadata validity, configured
//! compatibility, and integrity where declared. It does not establish that
//! the payload is a valid or verified HDLIR graph.

use core::fmt;
use serde_json::Error as JsonError;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Cursor, Read};

use crate::hdl::hdlir::serialization::encode::{
    FRAME_PREFIX_LEN, HDLIR_ARTIFACT_MAGIC,
};
use crate::hdl::hdlir::serialization::format::{
    ArtifactFormat, FormatError, IntegrityAlgorithm,
};
use crate::hdl::hdlir::version::{
    HdlirVersion, HDLIR_FORMAT_VERSION,
};

/// Policy for decoding an HDLIR artifact.
///
/// `None` size limits mean that this type imposes no additional size budget
/// for the corresponding category. The host, allocator, and input source can
/// still impose practical limits.
///
/// The default accepts artifacts with or without a payload digest. If a
/// digest is present, it is always verified. Applications that require
/// integrity metadata should enable `require_payload_integrity`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodeOptions {
    max_header_bytes: Option<usize>,
    max_payload_bytes: Option<usize>,
    max_artifact_bytes: Option<usize>,
    require_payload_integrity: bool,
    supported_features: BTreeSet<String>,
    supported_schemas: BTreeMap<String, HdlirVersion>,
    supported_dialects: BTreeMap<String, HdlirVersion>,
}

impl DecodeOptions {
    /// Create decoder options without explicit size limits.
    ///
    /// The default supported-feature, schema, and dialect sets are empty.
    /// Consequently, artifacts declaring required features, schemas, or
    /// dialects must be decoded with options that explicitly declare support.
    #[must_use]
    pub fn new() -> Self {
        Self {
            max_header_bytes: None,
            max_payload_bytes: None,
            max_artifact_bytes: None,
            require_payload_integrity: false,
            supported_features: BTreeSet::new(),
            supported_schemas: BTreeMap::new(),
            supported_dialects: BTreeMap::new(),
        }
    }

    /// Set the maximum metadata-header size in bytes.
    #[must_use]
    pub fn with_max_header_bytes(mut self, limit: Option<usize>) -> Self {
        self.max_header_bytes = limit;
        self
    }

    /// Set the maximum payload size in bytes.
    #[must_use]
    pub fn with_max_payload_bytes(mut self, limit: Option<usize>) -> Self {
        self.max_payload_bytes = limit;
        self
    }

    /// Set the maximum total artifact size, including framing, in bytes.
    #[must_use]
    pub fn with_max_artifact_bytes(mut self, limit: Option<usize>) -> Self {
        self.max_artifact_bytes = limit;
        self
    }

    /// Require payload integrity metadata.
    ///
    /// When enabled, an artifact without an integrity declaration is rejected.
    /// A declared digest is verified regardless of this setting.
    #[must_use]
    pub const fn with_required_payload_integrity(
        mut self,
        required: bool,
    ) -> Self {
        self.require_payload_integrity = required;
        self
    }

    /// Declare support for a required feature identifier.
    ///
    /// Feature names should match the qualified identifiers used by
    /// `ArtifactFormat`. Duplicate insertion is idempotent.
    #[must_use]
    pub fn with_supported_feature(
        mut self,
        feature: impl Into<String>,
    ) -> Self {
        self.supported_features.insert(feature.into());
        self
    }

    /// Declare support for a schema at a specific version.
    ///
    /// A required artifact schema is accepted only if its declared version
    /// can be read by the supplied supported version.
    #[must_use]
    pub fn with_supported_schema(
        mut self,
        name: impl Into<String>,
        version: HdlirVersion,
    ) -> Self {
        self.supported_schemas.insert(name.into(), version);
        self
    }

    /// Declare support for a dialect at a specific version.
    ///
    /// A required artifact dialect is accepted only if its declared version
    /// can be read by the supplied supported version.
    #[must_use]
    pub fn with_supported_dialect(
        mut self,
        name: impl Into<String>,
        version: HdlirVersion,
    ) -> Self {
        self.supported_dialects.insert(name.into(), version);
        self
    }

    /// Return the configured header-size budget.
    #[must_use]
    pub const fn max_header_bytes(&self) -> Option<usize> {
        self.max_header_bytes
    }

    /// Return the configured payload-size budget.
    #[must_use]
    pub const fn max_payload_bytes(&self) -> Option<usize> {
        self.max_payload_bytes
    }

    /// Return the configured total-artifact budget.
    #[must_use]
    pub const fn max_artifact_bytes(&self) -> Option<usize> {
        self.max_artifact_bytes
    }

    /// Return whether integrity metadata is mandatory.
    #[must_use]
    pub const fn requires_payload_integrity(&self) -> bool {
        self.require_payload_integrity
    }

    /// Return the explicitly supported feature identifiers.
    #[must_use]
    pub fn supported_features(&self) -> &BTreeSet<String> {
        &self.supported_features
    }

    /// Return the explicitly supported schema versions.
    #[must_use]
    pub fn supported_schemas(&self) -> &BTreeMap<String, HdlirVersion> {
        &self.supported_schemas
    }

    /// Return the explicitly supported dialect versions.
    #[must_use]
    pub fn supported_dialects(&self) -> &BTreeMap<String, HdlirVersion> {
        &self.supported_dialects
    }
}

impl Default for DecodeOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// A decoded artifact whose framing, metadata, configured compatibility, and
/// declared payload integrity have been checked.
///
/// The payload remains opaque bytes. It must be decoded by the canonical
/// payload codec and the resulting HDLIR must be verified before use.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedArtifact {
    format: ArtifactFormat,
    payload: Vec<u8>,
    artifact_len: u64,
}

impl DecodedArtifact {
    /// Return the validated artifact metadata.
    #[must_use]
    pub fn format(&self) -> &ArtifactFormat {
        &self.format
    }

    /// Return the decoded payload bytes.
    ///
    /// The payload is returned as an immutable slice so callers cannot
    /// accidentally mutate it while retaining the original validation claim.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Consume the artifact and return its payload bytes.
    ///
    /// Consuming the artifact makes the ownership transfer explicit.
    #[must_use]
    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }

    /// Return the complete framed artifact length, including the prefix.
    #[must_use]
    pub const fn artifact_len(&self) -> u64 {
        self.artifact_len
    }
}

/// Errors produced while decoding an HDLIR artifact.
#[derive(Debug)]
#[non_exhaustive]
pub enum DecodeError {
    /// The input does not begin with the expected HDLIR magic.
    InvalidMagic,

    /// The input ended before a required framing section was complete.
    Truncated {
        /// Section that ended prematurely.
        section: &'static str,
    },

    /// A framing length or total size is not representable.
    LengthNotRepresentable {
        /// Name of the unrepresentable length.
        resource: &'static str,
    },

    /// Checked size arithmetic overflowed.
    SizeOverflow,

    /// A configured operational size budget was exceeded.
    LimitExceeded {
        /// Resource category.
        resource: &'static str,
        /// Configured maximum.
        limit: usize,
        /// Declared or observed size.
        actual: u64,
    },

    /// A required allocation could not be reserved.
    AllocationFailed {
        /// Resource category.
        resource: &'static str,
        /// Requested capacity, if representable.
        requested: usize,
    },

    /// Metadata JSON could not be decoded.
    InvalidMetadata(JsonError),

    /// Decoded metadata failed its own invariants.
    InvalidFormat(FormatError),

    /// The artifact format version is not readable by this decoder.
    IncompatibleFormatVersion(FormatError),

    /// A required feature is not supported by this decoder configuration.
    UnsupportedRequiredFeature(String),

    /// A required schema or dialect is not supported or version-compatible.
    UnsupportedRequiredDefinition {
        /// Either `schema` or `dialect`.
        kind: &'static str,
        /// Required definition identifier.
        name: String,
        /// Version declared by the artifact.
        required_version: HdlirVersion,
        /// Supported version, if a definition with this name was registered.
        supported_version: Option<HdlirVersion>,
    },

    /// The artifact omitted required integrity metadata.
    IntegrityRequired,

    /// The payload does not match its declared integrity digest.
    IntegrityMismatch {
        /// Declared digest algorithm.
        algorithm: &'static str,
    },

    /// Bytes remain after the declared artifact payload.
    TrailingData,

    /// The input source returned an I/O error.
    ReadFailed {
        /// Section being read.
        section: &'static str,
        /// Underlying I/O error.
        source: io::Error,
    },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMagic => {
                formatter.write_str("invalid HDLIR artifact magic")
            }
            Self::Truncated { section } => {
                write!(formatter, "truncated HDLIR artifact {section}")
            }
            Self::LengthNotRepresentable { resource } => {
                write!(formatter, "{resource} length is not representable")
            }
            Self::SizeOverflow => {
                formatter.write_str("HDLIR artifact size arithmetic overflowed")
            }
            Self::LimitExceeded {
                resource,
                limit,
                actual,
            } => write!(
                formatter,
                "HDLIR decode budget exceeded for {resource}: \
                 declared or observed {actual} bytes, limit is {limit} bytes"
            ),
            Self::AllocationFailed {
                resource,
                requested,
            } => write!(
                formatter,
                "could not reserve {requested} bytes for HDLIR {resource}"
            ),
            Self::InvalidMetadata(error) => {
                write!(formatter, "invalid HDLIR metadata JSON: {error}")
            }
            Self::InvalidFormat(error) => {
                write!(formatter, "invalid HDLIR artifact metadata: {error}")
            }
            Self::IncompatibleFormatVersion(error) => {
                write!(formatter, "incompatible HDLIR format version: {error}")
            }
            Self::UnsupportedRequiredFeature(feature) => write!(
                formatter,
                "unsupported required HDLIR feature: {feature}"
            ),
            Self::UnsupportedRequiredDefinition {
                kind,
                name,
                required_version,
                supported_version,
            } => {
                if let Some(supported) = supported_version {
                    write!(
                        formatter,
                        "unsupported required HDLIR {kind} {name} \
                         at version {required_version}; reader supports {supported}"
                    )
                } else {
                    write!(
                        formatter,
                        "unsupported required HDLIR {kind} {name} \
                         at version {required_version}"
                    )
                }
            }
            Self::IntegrityRequired => {
                formatter.write_str("HDLIR payload integrity metadata is required")
            }
            Self::IntegrityMismatch { algorithm } => write!(
                formatter,
                "HDLIR payload does not match its declared {algorithm} digest"
            ),
            Self::TrailingData => {
                formatter.write_str("trailing bytes follow the HDLIR artifact")
            }
            Self::ReadFailed { section, source } => {
                write!(formatter, "could not read HDLIR {section}: {source}")
            }
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidMetadata(error) => Some(error),
            Self::InvalidFormat(error)
            | Self::IncompatibleFormatVersion(error) => Some(error),
            Self::ReadFailed { source, .. } => Some(source),
            Self::InvalidMagic
            | Self::Truncated { .. }
            | Self::LengthNotRepresentable { .. }
            | Self::SizeOverflow
            | Self::LimitExceeded { .. }
            | Self::AllocationFailed { .. }
            | Self::UnsupportedRequiredFeature(_)
            | Self::UnsupportedRequiredDefinition { .. }
            | Self::IntegrityRequired
            | Self::IntegrityMismatch { .. }
            | Self::TrailingData => None,
        }
    }
}

/// Decode one complete in-memory HDLIR artifact.
///
/// This function rejects trailing bytes. Use [`decode_artifact_from`] when
/// reading from a finite stream.
///
/// # Errors
///
/// Returns a structured error for invalid framing, malformed metadata,
/// incompatible required features or definitions, budget exhaustion,
/// allocation failure, missing required integrity metadata, digest mismatch,
/// truncation, or trailing data.
pub fn decode_artifact(
    bytes: &[u8],
    options: &DecodeOptions,
) -> Result<DecodedArtifact, DecodeError> {
    decode_artifact_from(&mut Cursor::new(bytes), options)
}

/// Decode one complete HDLIR artifact from a reader.
///
/// The reader must expose exactly one artifact and then EOF. This function
/// reads the fixed framing prefix first, checks declared lengths and budgets
/// before allocating buffers, then reads the header and payload. It does not
/// buffer the complete framed artifact as one allocation.
///
/// The owned result still retains the complete payload in memory. For a
/// transport that does not naturally terminate at artifact boundaries, frame
/// the input externally or use [`decode_artifact`] on an already bounded slice.
///
/// # Errors
///
/// Returns a structured error for invalid framing, malformed metadata,
/// incompatible required features or definitions, budget exhaustion,
/// allocation failure, missing required integrity metadata, digest mismatch,
/// truncation, trailing data, or I/O failure.
pub fn decode_artifact_from<R: Read>(
    reader: &mut R,
    options: &DecodeOptions,
) -> Result<DecodedArtifact, DecodeError> {
    let mut magic = [0_u8; 8];
    read_exact_section(reader, &mut magic, "magic")?;

    if &magic != HDLIR_ARTIFACT_MAGIC {
        return Err(DecodeError::InvalidMagic);
    }

    let mut header_length_bytes = [0_u8; 8];
    read_exact_section(reader, &mut header_length_bytes, "header length")?;
    let header_len_u64 = u64::from_be_bytes(header_length_bytes);

    let mut payload_length_bytes = [0_u8; 8];
    read_exact_section(reader, &mut payload_length_bytes, "payload length")?;
    let payload_len_u64 = u64::from_be_bytes(payload_length_bytes);

    let prefix_len_u64 = u64::try_from(FRAME_PREFIX_LEN)
        .map_err(|_| DecodeError::LengthNotRepresentable {
            resource: "frame prefix",
        })?;

    let artifact_len = prefix_len_u64
        .checked_add(header_len_u64)
        .and_then(|value| value.checked_add(payload_len_u64))
        .ok_or(DecodeError::SizeOverflow)?;

    enforce_limit(
        "metadata header",
        options.max_header_bytes,
        header_len_u64,
    )?;
    enforce_limit(
        "payload",
        options.max_payload_bytes,
        payload_len_u64,
    )?;
    enforce_limit(
        "complete artifact",
        options.max_artifact_bytes,
        artifact_len,
    )?;

    let header_len = usize::try_from(header_len_u64).map_err(|_| {
        DecodeError::LengthNotRepresentable {
            resource: "metadata header",
        }
    })?;
    let payload_len = usize::try_from(payload_len_u64).map_err(|_| {
        DecodeError::LengthNotRepresentable {
            resource: "payload",
        }
    })?;

    let mut header = Vec::new();
    header
        .try_reserve_exact(header_len)
        .map_err(|_| DecodeError::AllocationFailed {
            resource: "metadata header",
            requested: header_len,
        })?;
    header.resize(header_len, 0);
    read_exact_section(reader, &mut header, "metadata header")?;

    let format: ArtifactFormat =
        serde_json::from_slice(&header).map_err(DecodeError::InvalidMetadata)?;

    validate_format(&format, options)?;

    let mut payload = Vec::new();
    payload
        .try_reserve_exact(payload_len)
        .map_err(|_| DecodeError::AllocationFailed {
            resource: "payload",
            requested: payload_len,
        })?;
    payload.resize(payload_len, 0);
    read_exact_section(reader, &mut payload, "payload")?;

    verify_payload_integrity(&format, &payload, options)?;

    if read_one_after_payload(reader)? {
        return Err(DecodeError::TrailingData);
    }

    Ok(DecodedArtifact {
        format,
        payload,
        artifact_len,
    })
}

fn validate_format(
    format: &ArtifactFormat,
    options: &DecodeOptions,
) -> Result<(), DecodeError> {
    format.validate().map_err(DecodeError::InvalidFormat)?;

    format
        .ensure_version_readable_by(HDLIR_FORMAT_VERSION)
        .map_err(DecodeError::IncompatibleFormatVersion)?;

    for feature in format.required_features() {
        if !options.supported_features.contains(feature) {
            return Err(DecodeError::UnsupportedRequiredFeature(
                feature.clone(),
            ));
        }
    }

    validate_definitions(
        "schema",
        format.schemas(),
        &options.supported_schemas,
    )?;
    validate_definitions(
        "dialect",
        format.dialects(),
        &options.supported_dialects,
    )?;

    if options.require_payload_integrity && format.integrity().is_none() {
        return Err(DecodeError::IntegrityRequired);
    }

    Ok(())
}

fn validate_definitions(
    kind: &'static str,
    requirements: &BTreeMap<
        String,
        crate::hdl::hdlir::serialization::format::DefinitionRequirement,
    >,
    supported: &BTreeMap<String, HdlirVersion>,
) -> Result<(), DecodeError> {
    for (name, requirement) in requirements {
        if !requirement.is_required() {
            continue;
        }

        let Some(&supported_version) = supported.get(name) else {
            return Err(DecodeError::UnsupportedRequiredDefinition {
                kind,
                name: name.clone(),
                required_version: requirement.version(),
                supported_version: None,
            });
        };

        if requirement
            .version()
            .ensure_readable_by(supported_version)
            .is_err()
        {
            return Err(DecodeError::UnsupportedRequiredDefinition {
                kind,
                name: name.clone(),
                required_version: requirement.version(),
                supported_version: Some(supported_version),
            });
        }
    }

    Ok(())
}

fn verify_payload_integrity(
    format: &ArtifactFormat,
    payload: &[u8],
    options: &DecodeOptions,
) -> Result<(), DecodeError> {
    let Some(integrity) = format.integrity() else {
        return if options.require_payload_integrity {
            Err(DecodeError::IntegrityRequired)
        } else {
            Ok(())
        };
    };

    match integrity.algorithm() {
        IntegrityAlgorithm::Sha256 => {
            let actual_digest = Sha256::digest(payload);
            let actual_hex = hex::encode(actual_digest);
            let expected_hex = integrity.digest_hex();

            // Compare every byte rather than returning at the first mismatch.
            // This is not an authenticity check; the digest is public metadata.
            let mut difference = 0_u8;
            for (actual, expected) in
                actual_hex.bytes().zip(expected_hex.bytes())
            {
                difference |= actual ^ expected;
            }

            if actual_hex.len() != expected_hex.len() || difference != 0 {
                return Err(DecodeError::IntegrityMismatch {
                    algorithm: integrity.algorithm().as_str(),
                });
            }
        }
    }

    Ok(())
}

fn enforce_limit(
    resource: &'static str,
    limit: Option<usize>,
    actual: u64,
) -> Result<(), DecodeError> {
    let Some(limit) = limit else {
        return Ok(());
    };

    let limit_u64 = u64::try_from(limit).unwrap_or(u64::MAX);
    if actual > limit_u64 {
        return Err(DecodeError::LimitExceeded {
            resource,
            limit,
            actual,
        });
    }

    Ok(())
}

fn read_exact_section<R: Read>(
    reader: &mut R,
    buffer: &mut [u8],
    section: &'static str,
) -> Result<(), DecodeError> {
    match reader.read_exact(buffer) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => {
            Err(DecodeError::Truncated { section })
        }
        Err(source) => Err(DecodeError::ReadFailed { section, source }),
    }
}

/// Return true if any byte remains after the declared payload.
fn read_one_after_payload<R: Read>(
    reader: &mut R,
) -> Result<bool, DecodeError> {
    let mut byte = [0_u8; 1];

    loop {
        match reader.read(&mut byte) {
            Ok(0) => return Ok(false),
            Ok(_) => return Ok(true),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(source) => {
                return Err(DecodeError::ReadFailed {
                    section: "trailing-data check",
                    source,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::serialization::encode::{
        encode_artifact, EncodeOptions,
    };
    use crate::hdl::hdlir::serialization::format::ArtifactFormat;

    fn encode(payload: &[u8]) -> Vec<u8> {
        encode_artifact(
            &ArtifactFormat::new(),
            payload,
            EncodeOptions::new(),
        )
        .expect("test artifact should encode")
    }

    #[test]
    fn round_trip_preserves_payload_and_metadata() {
        let payload = b"opaque test payload";
        let bytes = encode(payload);

        let decoded =
            decode_artifact(&bytes, &DecodeOptions::new()).expect("decode");

        assert_eq!(decoded.payload(), payload);
        assert_eq!(decoded.format().format_id(), "zamani.hdlir");
        assert_eq!(
            decoded.artifact_len(),
            u64::try_from(bytes.len()).expect("length fits u64")
        );
    }

    #[test]
    fn rejects_invalid_magic() {
        let mut bytes = encode(b"payload");
        bytes[0] ^= 1;

        assert!(matches!(
            decode_artifact(&bytes, &DecodeOptions::new()),
            Err(DecodeError::InvalidMagic)
        ));
    }

    #[test]
    fn rejects_truncated_artifact() {
        let bytes = encode(b"payload");
        let truncated = &bytes[..bytes.len() - 1];

        assert!(matches!(
            decode_artifact(truncated, &DecodeOptions::new()),
            Err(DecodeError::Truncated { .. })
        ));
    }

    #[test]
    fn rejects_trailing_bytes() {
        let mut bytes = encode(b"payload");
        bytes.push(0);

        assert!(matches!(
            decode_artifact(&bytes, &DecodeOptions::new()),
            Err(DecodeError::TrailingData)
        ));
    }

    #[test]
    fn rejects_payload_over_configured_budget() {
        let bytes = encode(b"payload");
        let options =
            DecodeOptions::new().with_max_payload_bytes(Some(2));

        assert!(matches!(
            decode_artifact(&bytes, &options),
            Err(DecodeError::LimitExceeded {
                resource: "payload",
                ..
            })
        ));
    }

    #[test]
    fn rejects_payload_tampering_when_digest_is_present() {
        let mut bytes = encode(b"payload");
        let last = bytes.len() - 1;
        bytes[last] ^= 1;

        assert!(matches!(
            decode_artifact(&bytes, &DecodeOptions::new()),
            Err(DecodeError::IntegrityMismatch { .. })
        ));
    }

    #[test]
    fn requires_integrity_when_configured() {
        let bytes = encode_artifact(
            &ArtifactFormat::new(),
            b"payload",
            EncodeOptions::new().with_payload_integrity(false),
        )
        .expect("test artifact should encode");

        let options =
            DecodeOptions::new().with_required_payload_integrity(true);

        assert!(matches!(
            decode_artifact(&bytes, &options),
            Err(DecodeError::IntegrityRequired)
        ));
    }

    #[test]
    fn rejects_unsupported_required_features() {
        let mut format = ArtifactFormat::new();
        format
            .require_feature("zamani.hdlir.feature.example")
            .expect("valid feature identifier");

        let bytes = encode_artifact(
            &format,
            b"payload",
            EncodeOptions::new(),
        )
        .expect("test artifact should encode");

        assert!(matches!(
            decode_artifact(&bytes, &DecodeOptions::new()),
            Err(DecodeError::UnsupportedRequiredFeature(_))
        ));

        let options = DecodeOptions::new()
            .with_supported_feature("zamani.hdlir.feature.example");

        assert!(decode_artifact(&bytes, &options).is_ok());
    }

    #[test]
    fn rejects_required_schema_when_not_registered() {
        use crate::hdl::hdlir::serialization::format::DefinitionRequirement;

        let mut format = ArtifactFormat::new();
        format
            .add_schema(
                DefinitionRequirement::new(
                    "zamani.hdlir.schema.core",
                    HdlirVersion::new(1, 0, 0),
                    true,
                )
                .expect("valid schema requirement"),
            )
            .expect("schema requirement should be unique");

        let bytes = encode_artifact(
            &format,
            b"payload",
            EncodeOptions::new(),
        )
        .expect("test artifact should encode");

        assert!(matches!(
            decode_artifact(&bytes, &DecodeOptions::new()),
            Err(DecodeError::UnsupportedRequiredDefinition {
                kind: "schema",
                ..
            })
        ));
    }

    #[test]
    fn rejects_artifact_exceeding_total_size_budget() {
        let bytes = encode(b"payload");
        let options =
            DecodeOptions::new().with_max_artifact_bytes(Some(1));

        assert!(matches!(
            decode_artifact(&bytes, &options),
            Err(DecodeError::LimitExceeded {
                resource: "complete artifact",
                ..
            })
        ));
    }
}
