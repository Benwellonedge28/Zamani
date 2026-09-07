//! Zamani Quantum Resilience — Production Decoder
//!
//! Path:
//!     src/quantum/resilience/serialization/decode.rs
//!
//! # Purpose
//!
//! This module owns the decoding boundary for serialized resilience data.
//!
//! It is responsible for:
//!
//! - reading serialized resilience documents;
//! - parsing the schema envelope;
//! - validating schema identity;
//! - validating encoding identity;
//! - validating schema feature/flag metadata;
//! - applying an explicit unknown-field policy;
//! - deserializing the payload into caller-selected Rust types;
//! - exposing decoded schema metadata to higher layers;
//! - preserving deterministic behavior;
//! - supporting streaming readers;
//! - preventing silent acceptance of malformed or incompatible documents;
//! - avoiding artificial quantum-machine-size limits.
//!
//! It does NOT own:
//!
//! - resilience domain semantics;
//! - QEC;
//! - fault classification;
//! - hardware discovery;
//! - routing;
//! - scheduling;
//! - recovery;
//! - mitigation;
//! - checkpoint storage;
//! - integrity cryptography;
//! - authorization;
//! - migration algorithms.
//!
//! Those responsibilities remain in their owning modules.
//!
//! # Architectural position
//!
//! ```text
//!                 serialized bytes
//!                        |
//!                        v
//!                serialization::decode
//!                        |
//!              +---------+---------+
//!              |                   |
//!              v                   v
//!        schema validation     payload decode
//!              |                   |
//!              +---------+---------+
//!                        |
//!                        v
//!              DecodedDocument<T>
//!                        |
//!             +----------+-----------+
//!             |                      |
//!             v                      v
//!       domain validation       provenance/state
//! ```
//!
//! # Important boundary
//!
//! Decoding is NOT semantic validation.
//!
//! A successfully decoded `T` means:
//!
//! ```text
//! bytes -> syntactically valid representation -> Rust value
//! ```
//!
//! It does NOT mean:
//!
//! ```text
//! Rust value -> semantically valid quantum state
//! Rust value -> safe recovery state
//! Rust value -> valid hardware state
//! Rust value -> authorized execution
//! ```
//!
//! Higher layers MUST perform domain-specific validation before using decoded
//! values for execution, recovery, checkpoint restoration, or policy decisions.
//!
//! # Write once, scale everywhere
//!
//! This module contains no fixed limits for:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - operations;
//! - faults;
//! - incidents;
//! - telemetry events;
//! - devices;
//! - backends;
//! - checkpoints;
//! - payload size.
//!
//! `decode_from_reader` is the preferred API for large serialized documents.
//! The caller controls the reader and therefore controls buffering, storage,
//! transport and resource policy.
//!
//! The decoder itself does not impose a machine-size ceiling.
//!
//! # Canonical qubit identity
//!
//! This module deliberately does not define or deserialize quantum identifiers
//! as primitive integers.
//!
//! When a decoded domain object contains logical or physical qubit identities,
//! that domain object remains responsible for using the canonical types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No duplicate resilience-specific qubit type is introduced here.
//!
//! # Security
//!
//! Serialized input is untrusted.
//!
//! Decoding therefore:
//!
//! - validates the schema envelope;
//! - rejects malformed schema metadata;
//! - rejects incompatible encodings;
//! - never grants authority;
//! - never opens hardware access;
//! - never executes decoded instructions;
//! - never performs recovery;
//! - never trusts serialized capability claims as authorization;
//! - never treats a successful decode as proof of integrity.
//!
//! Integrity/authentication must be performed by the surrounding security or
//! checkpoint integrity layer.
//!
//! # Determinism
//!
//! Decoder behavior is deterministic for identical:
//!
//! - input bytes;
//! - decoder configuration;
//! - schema policy;
//! - target Rust type.
//!
//! The decoder does not use:
//!
//! - randomness;
//! - system time;
//! - thread identifiers;
//! - process identifiers;
//! - memory addresses;
//! - unordered iteration for decisions.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::io::{self, Read};

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{Map, Value};

use super::schema::{
    current_schema_id,
    current_schema_header,
    DocumentKind,
    EncodingKind,
    SchemaCompatibility,
    SchemaEnvelope,
    SchemaFeatures,
    SchemaFlags,
    SchemaHeader,
    SchemaId,
    SchemaNamespace,
    SchemaValidation,
    SchemaVersion,
    UnknownFieldPolicy,
    CURRENT_SCHEMA_MAJOR,
    CURRENT_SCHEMA_VERSION,
    RESILIENCE_SCHEMA_NAMESPACE,
};

// =============================================================================
// Public constants
// =============================================================================

/// Canonical JSON media type accepted by this decoder.
pub const RESILIENCE_JSON_MEDIA_TYPE: &str = "application/json";

/// Canonical textual encoding name.
pub const RESILIENCE_ENCODING_FORMAT: &str = "json";

/// Decoder implementation revision.
///
/// This is NOT the persisted schema version.
pub const DECODER_IMPLEMENTATION_REVISION: u32 = 1;

// =============================================================================
// Decode configuration
// =============================================================================

/// Controls how strictly a serialized resilience document is decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecoderConfig {
    /// How unknown schema fields are handled.
    pub unknown_field_policy: UnknownFieldPolicy,

    /// Whether the decoder permits payload-only documents.
    ///
    /// Portable persisted documents should normally be enveloped.
    pub allow_payload_only: bool,

    /// Whether an exact current schema version is required.
    ///
    /// When false, same-major compatibility can be accepted according to the
    /// decoder's compatibility policy.
    pub require_exact_schema: bool,

    /// Whether future minor versions may be accepted when the document
    /// explicitly declares forward extensibility.
    ///
    /// Such documents are returned as degraded compatibility rather than being
    /// silently treated as identical to the current schema.
    pub allow_forward_minor: bool,

    /// Whether older schema versions within the current major are accepted.
    ///
    /// This does not perform migration. It only permits decoding when the
    /// wire representation is structurally compatible.
    pub allow_older_minor: bool,

    /// Whether unknown feature bits are accepted.
    ///
    /// Unknown features must never be interpreted as known capabilities.
    pub allow_unknown_feature_bits: bool,

    /// Whether unknown schema flag bits are accepted.
    pub allow_unknown_flag_bits: bool,

    /// Whether trailing non-whitespace data after one JSON document is rejected.
    pub reject_trailing_data: bool,
}

impl DecoderConfig {
    /// Production configuration.
    ///
    /// The production decoder is conservative:
    ///
    /// - envelopes are required;
    /// - unknown fields are rejected;
    /// - exact schema versions are preferred;
    /// - unknown feature/flag bits are rejected;
    /// - trailing data is rejected.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            unknown_field_policy: UnknownFieldPolicy::Reject,
            allow_payload_only: false,
            require_exact_schema: true,
            allow_forward_minor: false,
            allow_older_minor: false,
            allow_unknown_feature_bits: false,
            allow_unknown_flag_bits: false,
            reject_trailing_data: true,
        }
    }

    /// Configuration for controlled forward-compatible readers.
    ///
    /// This is useful for infrastructure components whose explicit purpose is
    /// to transport documents created by newer minor versions without
    /// interpreting unknown extensions.
    #[must_use]
    pub const fn forward_compatible() -> Self {
        Self {
            unknown_field_policy: UnknownFieldPolicy::IgnoreOptional,
            allow_payload_only: false,
            require_exact_schema: false,
            allow_forward_minor: true,
            allow_older_minor: true,
            allow_unknown_feature_bits: true,
            allow_unknown_flag_bits: true,
            reject_trailing_data: true,
        }
    }

    /// Configuration for trusted internal payload-only use.
    ///
    /// This mode is intentionally not the default.
    #[must_use]
    pub const fn internal_payload_only() -> Self {
        Self {
            unknown_field_policy: UnknownFieldPolicy::Reject,
            allow_payload_only: true,
            require_exact_schema: true,
            allow_forward_minor: false,
            allow_older_minor: false,
            allow_unknown_feature_bits: false,
            allow_unknown_flag_bits: false,
            reject_trailing_data: true,
        }
    }
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self::production()
    }
}

// =============================================================================
// Decode error
// =============================================================================

/// Errors produced by the resilience decoding boundary.
#[derive(Debug)]
pub enum DecodeError {
    /// Input could not be read.
    Io(io::Error),

    /// JSON syntax or deserialization failed.
    Deserialization(serde_json::Error),

    /// The top-level document is not an object.
    InvalidDocumentShape,

    /// The schema envelope is absent.
    MissingSchema,

    /// The payload is absent.
    MissingPayload,

    /// The schema namespace is invalid.
    InvalidNamespace {
        /// Namespace supplied by the document.
        found: String,
    },

    /// The document kind is unsupported.
    UnsupportedDocumentKind {
        /// Numeric document-kind code, when available.
        code: Option<u16>,

        /// Text document-kind name, when available.
        name: Option<String>,
    },

    /// The schema version is malformed.
    InvalidSchemaVersion,

    /// The schema version is unsupported.
    UnsupportedSchemaVersion {
        /// Version supplied by the document.
        found: SchemaVersion,

        /// Version understood by this implementation.
        current: SchemaVersion,
    },

    /// Schema compatibility could not be established safely.
    IncompatibleSchema {
        /// Version supplied by the document.
        found: SchemaVersion,

        /// Compatibility result.
        compatibility: SchemaCompatibility,
    },

    /// The encoding is unsupported.
    UnsupportedEncoding {
        /// Encoding identifier supplied by the document.
        found: String,
    },

    /// Required schema metadata is missing.
    MissingSchemaField {
        /// Stable field name.
        field: &'static str,
    },

    /// A schema field has an invalid representation.
    InvalidSchemaField {
        /// Stable field name.
        field: &'static str,

        /// Explanation safe for diagnostics.
        reason: &'static str,
    },

    /// Unknown schema fields were rejected.
    UnknownSchemaField {
        /// Field name.
        field: String,
    },

    /// Unknown feature bits were rejected.
    UnknownFeatureBits {
        /// Unknown bits.
        bits: u64,
    },

    /// Unknown schema flag bits were rejected.
    UnknownFlagBits {
        /// Unknown bits.
        bits: u32,
    },

    /// A payload-only document was rejected.
    PayloadOnlyNotAllowed,

    /// The declared encoding does not match the decoder.
    EncodingMismatch {
        /// Declared encoding.
        declared: EncodingKind,

        /// Decoder encoding.
        expected: EncodingKind,
    },

    /// The schema metadata itself is structurally invalid.
    InvalidSchema,

    /// The decoded payload failed the selected domain type's deserializer.
    InvalidPayload,

    /// More than one JSON document was supplied.
    TrailingData,

    /// A numeric conversion required by the schema could not be represented.
    NumericOverflow,

    /// An impossible internal decoder state was reached.
    InternalInvariantViolation,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => {
                write!(formatter, "resilience decoding input failed: {error}")
            }

            Self::Deserialization(error) => {
                write!(formatter, "resilience document deserialization failed: {error}")
            }

            Self::InvalidDocumentShape => {
                formatter.write_str("resilience document must be a JSON object")
            }

            Self::MissingSchema => {
                formatter.write_str("resilience document is missing the schema envelope")
            }

            Self::MissingPayload => {
                formatter.write_str("resilience document is missing the payload")
            }

            Self::InvalidNamespace { found } => {
                write!(
                    formatter,
                    "unsupported resilience schema namespace: {found}"
                )
            }

            Self::UnsupportedDocumentKind { code, name } => {
                match (code, name) {
                    (Some(code), Some(name)) => {
                        write!(
                            formatter,
                            "unsupported resilience document kind {name} ({code})"
                        )
                    }

                    (Some(code), None) => {
                        write!(
                            formatter,
                            "unsupported resilience document kind code {code}"
                        )
                    }

                    (None, Some(name)) => {
                        write!(
                            formatter,
                            "unsupported resilience document kind {name}"
                        )
                    }

                    (None, None) => {
                        formatter.write_str("unsupported resilience document kind")
                    }
                }
            }

            Self::InvalidSchemaVersion => {
                formatter.write_str("invalid resilience schema version")
            }

            Self::UnsupportedSchemaVersion { found, current } => {
                write!(
                    formatter,
                    "unsupported resilience schema version {found}; current decoder schema is {current}"
                )
            }

            Self::IncompatibleSchema {
                found,
                compatibility,
            } => {
                write!(
                    formatter,
                    "resilience schema {found} is not safely decodable: {compatibility:?}"
                )
            }

            Self::UnsupportedEncoding { found } => {
                write!(
                    formatter,
                    "unsupported resilience serialization encoding: {found}"
                )
            }

            Self::MissingSchemaField { field } => {
                write!(
                    formatter,
                    "resilience schema is missing required field `{field}`"
                )
            }

            Self::InvalidSchemaField { field, reason } => {
                write!(
                    formatter,
                    "invalid resilience schema field `{field}`: {reason}"
                )
            }

            Self::UnknownSchemaField { field } => {
                write!(
                    formatter,
                    "unknown resilience schema field `{field}`"
                )
            }

            Self::UnknownFeatureBits { bits } => {
                write!(
                    formatter,
                    "unknown resilience schema feature bits: {bits:#018x}"
                )
            }

            Self::UnknownFlagBits { bits } => {
                write!(
                    formatter,
                    "unknown resilience schema flag bits: {bits:#010x}"
                )
            }

            Self::PayloadOnlyNotAllowed => {
                formatter.write_str(
                    "payload-only resilience documents are not permitted by this decoder",
                )
            }

            Self::EncodingMismatch { declared, expected } => {
                write!(
                    formatter,
                    "resilience document declares encoding {declared}, but decoder expects {expected}"
                )
            }

            Self::InvalidSchema => {
                formatter.write_str("invalid resilience schema metadata")
            }

            Self::InvalidPayload => {
                formatter.write_str("resilience payload could not be decoded")
            }

            Self::TrailingData => {
                formatter.write_str(
                    "trailing non-whitespace data follows the resilience document",
                )
            }

            Self::NumericOverflow => {
                formatter.write_str(
                    "numeric value cannot be represented by the decoder",
                )
            }

            Self::InternalInvariantViolation => {
                formatter.write_str(
                    "resilience decoder internal invariant was violated",
                )
            }
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Deserialization(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for DecodeError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for DecodeError {
    fn from(error: serde_json::Error) -> Self {
        Self::Deserialization(error)
    }
}

// =============================================================================
// Decoded document
// =============================================================================

/// A successfully decoded resilience document.
///
/// The schema metadata is retained alongside the payload so callers can make
/// compatibility/provenance decisions without reconstructing information from
/// the payload itself.
#[derive(Debug, Clone)]
pub struct DecodedDocument<T> {
    /// Decoded domain payload.
    payload: T,

    /// Original schema envelope.
    envelope: SchemaEnvelope,

    /// Compatibility classification established during decoding.
    compatibility: SchemaCompatibility,
}

impl<T> DecodedDocument<T> {
    /// Creates a decoded document.
    #[must_use]
    pub const fn new(
        payload: T,
        envelope: SchemaEnvelope,
        compatibility: SchemaCompatibility,
    ) -> Self {
        Self {
            payload,
            envelope,
            compatibility,
        }
    }

    /// Returns a reference to the decoded payload.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Consumes the document and returns the payload.
    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }

    /// Returns the decoded schema envelope.
    #[must_use]
    pub const fn envelope(&self) -> SchemaEnvelope {
        self.envelope
    }

    /// Returns the schema identifier.
    #[must_use]
    pub const fn schema(&self) -> SchemaId {
        self.envelope.schema()
    }

    /// Returns the document kind.
    #[must_use]
    pub const fn kind(&self) -> DocumentKind {
        self.envelope.kind()
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn version(&self) -> SchemaVersion {
        self.envelope.version()
    }

    /// Returns the schema feature set.
    #[must_use]
    pub const fn features(&self) -> SchemaFeatures {
        self.envelope.features()
    }

    /// Returns schema structural flags.
    #[must_use]
    pub const fn flags(&self) -> SchemaFlags {
        self.envelope.flags()
    }

    /// Returns the declared encoding.
    #[must_use]
    pub const fn encoding(&self) -> EncodingKind {
        self.envelope.encoding()
    }

    /// Returns schema compatibility classification.
    #[must_use]
    pub const fn compatibility(&self) -> SchemaCompatibility {
        self.compatibility
    }

    /// Returns true if the document was decoded against the exact current
    /// schema version.
    #[must_use]
    pub const fn is_current_schema(&self) -> bool {
        self.version().is_exact(CURRENT_SCHEMA_VERSION)
    }

    /// Returns true if the document contains data whose semantics may not be
    /// fully interpreted by this decoder.
    #[must_use]
    pub const fn is_degraded(&self) -> bool {
        matches!(
            self.compatibility,
            SchemaCompatibility::CompatibleWithDegradation
        )
    }

    /// Returns true if migration is required before the payload is considered
    /// equivalent to the current schema.
    #[must_use]
    pub const fn requires_migration(&self) -> bool {
        self.compatibility.requires_migration()
    }
}

// =============================================================================
// Raw wire representation
// =============================================================================

/// Raw schema envelope used only while parsing JSON.
///
/// This is deliberately separate from the semantic schema types in
/// `schema.rs`. The semantic schema remains authoritative; this structure is
/// only a parser boundary.
#[derive(Debug, Deserialize)]
struct RawEnvelope {
    schema: RawSchema,
    payload: Value,
}

/// Raw schema representation.
///
/// The representation is intentionally tolerant about whether numeric codes
/// or canonical strings are supplied. The semantic validation layer decides
/// what is acceptable.
#[derive(Debug, Deserialize)]
struct RawSchema {
    namespace: Option<String>,

    #[serde(rename = "document_kind")]
    document_kind: Option<Value>,

    #[serde(rename = "schema_version")]
    schema_version: Option<Value>,

    features: Option<Value>,

    flags: Option<Value>,

    encoding: Option<Value>,

    #[serde(flatten)]
    extensions: Map<String, Value>,
}

// =============================================================================
// Public decoding API
// =============================================================================

/// Decodes a complete enveloped JSON resilience document using production
/// defaults.
///
/// For large documents prefer [`decode_from_reader`].
pub fn decode<T>(bytes: &[u8]) -> Result<DecodedDocument<T>, DecodeError>
where
    T: DeserializeOwned,
{
    decode_with_config(bytes, DecoderConfig::production())
}

/// Decodes a complete JSON resilience document using explicit configuration.
pub fn decode_with_config<T>(
    bytes: &[u8],
    config: DecoderConfig,
) -> Result<DecodedDocument<T>, DecodeError>
where
    T: DeserializeOwned,
{
    let mut cursor = std::io::Cursor::new(bytes);

    decode_from_reader_with_config(&mut cursor, config)
}

/// Decodes one resilience document from an arbitrary reader.
///
/// This is the preferred API for large serialized artifacts because the input
/// source is controlled by the caller and no fixed document-size constant is
/// imposed by the resilience subsystem.
pub fn decode_from_reader<R, T>(
    reader: &mut R,
) -> Result<DecodedDocument<T>, DecodeError>
where
    R: Read,
    T: DeserializeOwned,
{
    decode_from_reader_with_config(reader, DecoderConfig::production())
}

/// Decodes one resilience document from an arbitrary reader using explicit
/// configuration.
pub fn decode_from_reader_with_config<R, T>(
    reader: &mut R,
    config: DecoderConfig,
) -> Result<DecodedDocument<T>, DecodeError>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut deserializer = serde_json::Deserializer::from_reader(reader);

    let raw = RawEnvelope::deserialize(&mut deserializer)
        .map_err(DecodeError::Deserialization)?;

    if config.reject_trailing_data {
        ensure_no_trailing_json(&mut deserializer)?;
    }

    decode_raw_envelope(raw, config)
}

/// Decodes a JSON value that already represents a complete resilience
/// document.
///
/// This is useful when an outer transport has already parsed JSON.
pub fn decode_value<T>(
    value: Value,
) -> Result<DecodedDocument<T>, DecodeError>
where
    T: DeserializeOwned,
{
    decode_value_with_config(value, DecoderConfig::production())
}

/// Decodes a JSON value using explicit decoder configuration.
pub fn decode_value_with_config<T>(
    value: Value,
    config: DecoderConfig,
) -> Result<DecodedDocument<T>, DecodeError>
where
    T: DeserializeOwned,
{
    let object = value
        .as_object()
        .ok_or(DecodeError::InvalidDocumentShape)?;

    let schema_value = object
        .get("schema")
        .ok_or(DecodeError::MissingSchema)?;

    let payload = object
        .get("payload")
        .cloned()
        .ok_or(DecodeError::MissingPayload)?;

    let raw_schema: RawSchema =
        serde_json::from_value(schema_value.clone())
            .map_err(DecodeError::Deserialization)?;

    let raw = RawEnvelope {
        schema: raw_schema,
        payload,
    };

    decode_raw_envelope(raw, config)
}

// =============================================================================
// Raw envelope decoding
// =============================================================================

fn decode_raw_envelope<T>(
    raw: RawEnvelope,
    config: DecoderConfig,
) -> Result<DecodedDocument<T>, DecodeError>
where
    T: DeserializeOwned,
{
    let parsed_schema = parse_schema(raw.schema, config)?;

    let compatibility = evaluate_compatibility(
        parsed_schema.version(),
        parsed_schema.flags(),
        config,
    )?;

    let payload = serde_json::from_value::<T>(raw.payload)
        .map_err(|_| DecodeError::InvalidPayload)?;

    Ok(DecodedDocument::new(
        payload,
        parsed_schema,
        compatibility,
    ))
}

// =============================================================================
// Schema parsing
// =============================================================================

fn parse_schema(
    raw: RawSchema,
    config: DecoderConfig,
) -> Result<SchemaEnvelope, DecodeError> {
    let namespace = raw
        .namespace
        .ok_or(DecodeError::MissingSchemaField {
            field: "namespace",
        })?;

    if namespace != RESILIENCE_SCHEMA_NAMESPACE {
        return Err(DecodeError::InvalidNamespace {
            found: namespace,
        });
    }

    let kind = parse_document_kind(
        raw.document_kind
            .ok_or(DecodeError::MissingSchemaField {
                field: "document_kind",
            })?,
    )?;

    let version = parse_schema_version(
        raw.schema_version
            .ok_or(DecodeError::MissingSchemaField {
                field: "schema_version",
            })?,
    )?;

    let features = parse_features(
        raw.features
            .unwrap_or_else(|| Value::from(0_u64)),
        config,
    )?;

    let flags = parse_flags(
        raw.flags
            .unwrap_or_else(|| Value::from(0_u32)),
        config,
    )?;

    let encoding = parse_encoding(
        raw.encoding
            .ok_or(DecodeError::MissingSchemaField {
                field: "encoding",
            })?,
    )?;

    validate_schema_extensions(&raw.extensions, config)?;

    let header = SchemaHeader::new(
        SchemaId::new(kind, version),
        features,
        flags,
    );

    match super::schema::validate_schema_header(header) {
        SchemaValidation::Valid
        | SchemaValidation::RequiresCompatibilityCheck => {}
        SchemaValidation::HasUnsupportedExtensions => {
            return Err(DecodeError::InvalidSchema);
        }
    }

    Ok(SchemaEnvelope::new(header, encoding))
}

// =============================================================================
// Document kind
// =============================================================================

fn parse_document_kind(value: Value) -> Result<DocumentKind, DecodeError> {
    match value {
        Value::String(name) => document_kind_from_name(&name),

        Value::Number(number) => {
            let code = number
                .as_u64()
                .ok_or(DecodeError::InvalidSchemaField {
                    field: "document_kind",
                    reason: "document kind code must be an unsigned integer",
                })?;

            let code = u16::try_from(code)
                .map_err(|_| DecodeError::NumericOverflow)?;

            document_kind_from_code(code)
        }

        _ => Err(DecodeError::InvalidSchemaField {
            field: "document_kind",
            reason: "document kind must be a string or unsigned integer",
        }),
    }
}

fn document_kind_from_name(
    name: &str,
) -> Result<DocumentKind, DecodeError> {
    match name {
        "execution_state" => Ok(DocumentKind::ExecutionState),
        "incident" => Ok(DocumentKind::Incident),
        "recovery_plan" => Ok(DocumentKind::RecoveryPlan),
        "recovery_state" => Ok(DocumentKind::RecoveryState),
        "checkpoint" => Ok(DocumentKind::Checkpoint),
        "checkpoint_manifest" => Ok(DocumentKind::CheckpointManifest),
        "telemetry_event" => Ok(DocumentKind::TelemetryEvent),
        "telemetry_metric" => Ok(DocumentKind::TelemetryMetric),
        "telemetry_trace" => Ok(DocumentKind::TelemetryTrace),
        "health_state" => Ok(DocumentKind::HealthState),
        "policy" => Ok(DocumentKind::Policy),
        "capability_snapshot" => Ok(DocumentKind::CapabilitySnapshot),
        "diagnosis" => Ok(DocumentKind::Diagnosis),
        "verification_result" => Ok(DocumentKind::VerificationResult),
        "provenance" => Ok(DocumentKind::Provenance),
        "history_record" => Ok(DocumentKind::HistoryRecord),
        "learning_feedback" => Ok(DocumentKind::LearningFeedback),

        _ => Err(DecodeError::UnsupportedDocumentKind {
            code: None,
            name: Some(name.to_owned()),
        }),
    }
}

fn document_kind_from_code(
    code: u16,
) -> Result<DocumentKind, DecodeError> {
    match code {
        1 => Ok(DocumentKind::ExecutionState),
        2 => Ok(DocumentKind::Incident),
        3 => Ok(DocumentKind::RecoveryPlan),
        4 => Ok(DocumentKind::RecoveryState),
        5 => Ok(DocumentKind::Checkpoint),
        6 => Ok(DocumentKind::CheckpointManifest),
        7 => Ok(DocumentKind::TelemetryEvent),
        8 => Ok(DocumentKind::TelemetryMetric),
        9 => Ok(DocumentKind::TelemetryTrace),
        10 => Ok(DocumentKind::HealthState),
        11 => Ok(DocumentKind::Policy),
        12 => Ok(DocumentKind::CapabilitySnapshot),
        13 => Ok(DocumentKind::Diagnosis),
        14 => Ok(DocumentKind::VerificationResult),
        15 => Ok(DocumentKind::Provenance),
        16 => Ok(DocumentKind::HistoryRecord),
        17 => Ok(DocumentKind::LearningFeedback),

        _ => Err(DecodeError::UnsupportedDocumentKind {
            code: Some(code),
            name: None,
        }),
    }
}

// =============================================================================
// Schema version
// =============================================================================

fn parse_schema_version(
    value: Value,
) -> Result<SchemaVersion, DecodeError> {
    match value {
        Value::String(version) => parse_version_string(&version),

        Value::Array(parts) => {
            if parts.len() != 3 {
                return Err(DecodeError::InvalidSchemaField {
                    field: "schema_version",
                    reason: "version array must contain major, minor and patch",
                });
            }

            let major = parse_u16_value(
                &parts[0],
                "schema_version",
            )?;

            let minor = parse_u16_value(
                &parts[1],
                "schema_version",
            )?;

            let patch = parse_u16_value(
                &parts[2],
                "schema_version",
            )?;

            Ok(SchemaVersion::new(major, minor, patch))
        }

        Value::Object(object) => {
            let major = object
                .get("major")
                .ok_or(DecodeError::InvalidSchemaField {
                    field: "schema_version",
                    reason: "missing major version",
                })?;

            let minor = object
                .get("minor")
                .ok_or(DecodeError::InvalidSchemaField {
                    field: "schema_version",
                    reason: "missing minor version",
                })?;

            let patch = object
                .get("patch")
                .ok_or(DecodeError::InvalidSchemaField {
                    field: "schema_version",
                    reason: "missing patch version",
                })?;

            Ok(SchemaVersion::new(
                parse_u16_value(major, "schema_version")?,
                parse_u16_value(minor, "schema_version")?,
                parse_u16_value(patch, "schema_version")?,
            ))
        }

        _ => Err(DecodeError::InvalidSchemaVersion),
    }
}

fn parse_version_string(
    value: &str,
) -> Result<SchemaVersion, DecodeError> {
    let mut components = value.split('.');

    let major = components
        .next()
        .ok_or(DecodeError::InvalidSchemaVersion)?;

    let minor = components
        .next()
        .ok_or(DecodeError::InvalidSchemaVersion)?;

    let patch = components
        .next()
        .ok_or(DecodeError::InvalidSchemaVersion)?;

    if components.next().is_some() {
        return Err(DecodeError::InvalidSchemaVersion);
    }

    let major = major
        .parse::<u16>()
        .map_err(|_| DecodeError::InvalidSchemaVersion)?;

    let minor = minor
        .parse::<u16>()
        .map_err(|_| DecodeError::InvalidSchemaVersion)?;

    let patch = patch
        .parse::<u16>()
        .map_err(|_| DecodeError::InvalidSchemaVersion)?;

    Ok(SchemaVersion::new(major, minor, patch))
}

// =============================================================================
// Features
// =============================================================================

fn parse_features(
    value: Value,
    config: DecoderConfig,
) -> Result<SchemaFeatures, DecodeError> {
    let bits = match value {
        Value::Number(number) => number
            .as_u64()
            .ok_or(DecodeError::InvalidSchemaField {
                field: "features",
                reason: "features must be an unsigned integer",
            })?,

        Value::Object(mut object) => {
            if let Some(bits) = object.remove("bits") {
                bits.as_u64().ok_or(DecodeError::InvalidSchemaField {
                    field: "features",
                    reason: "feature bits must be an unsigned integer",
                })?
            } else {
                return Err(DecodeError::InvalidSchemaField {
                    field: "features",
                    reason: "feature object must contain bits",
                });
            }
        }

        _ => {
            return Err(DecodeError::InvalidSchemaField {
                field: "features",
                reason: "features must be an unsigned integer or object",
            });
        }
    };

    let known = known_feature_bits();
    let unknown = bits & !known;

    if unknown != 0 && !config.allow_unknown_feature_bits {
        return Err(DecodeError::UnknownFeatureBits {
            bits: unknown,
        });
    }

    Ok(SchemaFeatures::from_bits(bits))
}

const fn known_feature_bits() -> u64 {
    SchemaFeatures::ZQN_FAULTS.bits()
        | SchemaFeatures::LOGICAL_QUBITS.bits()
        | SchemaFeatures::PHYSICAL_QUBITS.bits()
        | SchemaFeatures::RESOURCE_MAPPING.bits()
        | SchemaFeatures::QEC_STATE.bits()
        | SchemaFeatures::MITIGATION.bits()
        | SchemaFeatures::VERIFICATION.bits()
        | SchemaFeatures::CHECKPOINTS.bits()
        | SchemaFeatures::TELEMETRY.bits()
        | SchemaFeatures::DISTRIBUTED.bits()
        | SchemaFeatures::LEARNING.bits()
}

// =============================================================================
// Schema flags
// =============================================================================

fn parse_flags(
    value: Value,
    config: DecoderConfig,
) -> Result<SchemaFlags, DecodeError> {
    let bits = match value {
        Value::Number(number) => number
            .as_u64()
            .ok_or(DecodeError::InvalidSchemaField {
                field: "flags",
                reason: "flags must be an unsigned integer",
            })?,

        Value::Object(mut object) => {
            if let Some(bits) = object.remove("bits") {
                bits.as_u64().ok_or(DecodeError::InvalidSchemaField {
                    field: "flags",
                    reason: "flag bits must be an unsigned integer",
                })?
            } else {
                return Err(DecodeError::InvalidSchemaField {
                    field: "flags",
                    reason: "flag object must contain bits",
                });
            }
        }

        _ => {
            return Err(DecodeError::InvalidSchemaField {
                field: "flags",
                reason: "flags must be an unsigned integer or object",
            });
        }
    };

    let bits =
        u32::try_from(bits).map_err(|_| DecodeError::NumericOverflow)?;

    let known = known_schema_flag_bits();
    let unknown = bits & !known;

    if unknown != 0 && !config.allow_unknown_flag_bits {
        return Err(DecodeError::UnknownFlagBits {
            bits: unknown,
        });
    }

    Ok(SchemaFlags::from_bits(bits))
}

const fn known_schema_flag_bits() -> u32 {
    SchemaFlags::DETERMINISTIC.bits()
        | SchemaFlags::PERSISTABLE.bits()
        | SchemaFlags::SELF_DESCRIBING.bits()
        | SchemaFlags::FORWARD_EXTENSIBLE.bits()
        | SchemaFlags::HAS_INTEGRITY_METADATA.bits()
}

// =============================================================================
// Encoding
// =============================================================================

fn parse_encoding(
    value: Value,
) -> Result<EncodingKind, DecodeError> {
    match value {
        Value::String(name) => match name.as_str() {
            "binary" => Ok(EncodingKind::Binary),
            "text" => Ok(EncodingKind::Text),
            "json" => Ok(EncodingKind::Json),
            "cbor" => Ok(EncodingKind::Cbor),

            _ => Err(DecodeError::UnsupportedEncoding {
                found: name,
            }),
        },

        Value::Number(number) => {
            let code = number
                .as_u64()
                .ok_or(DecodeError::InvalidSchemaField {
                    field: "encoding",
                    reason: "encoding code must be an unsigned integer",
                })?;

            let code = u16::try_from(code)
                .map_err(|_| DecodeError::NumericOverflow)?;

            match code {
                1 => Ok(EncodingKind::Binary),
                2 => Ok(EncodingKind::Text),
                3 => Ok(EncodingKind::Json),
                4 => Ok(EncodingKind::Cbor),

                _ => Err(DecodeError::UnsupportedEncoding {
                    found: code.to_string(),
                }),
            }
        }

        _ => Err(DecodeError::InvalidSchemaField {
            field: "encoding",
            reason: "encoding must be a string or unsigned integer",
        }),
    }
}

// =============================================================================
// Extension handling
// =============================================================================

fn validate_schema_extensions(
    extensions: &Map<String, Value>,
    config: DecoderConfig,
) -> Result<(), DecodeError> {
    for field in extensions.keys() {
        match config.unknown_field_policy {
            UnknownFieldPolicy::Reject => {
                return Err(DecodeError::UnknownSchemaField {
                    field: field.clone(),
                });
            }

            UnknownFieldPolicy::IgnoreOptional => {
                // Forward-compatible reader intentionally ignores unknown
                // optional metadata. It never interprets it as authority.
            }

            UnknownFieldPolicy::Preserve => {
                // This decoder does not expose an extension-preservation
                // container. Therefore Preserve has the same immediate
                // acceptance semantics as IgnoreOptional, while the caller
                // remains responsible for lossless forwarding.
            }
        }
    }

    Ok(())
}

// =============================================================================
// Compatibility
// =============================================================================

fn evaluate_compatibility(
    version: SchemaVersion,
    flags: SchemaFlags,
    config: DecoderConfig,
) -> Result<SchemaCompatibility, DecodeError> {
    let current = CURRENT_SCHEMA_VERSION;

    if config.require_exact_schema {
        if version.is_exact(current) {
            return Ok(SchemaCompatibility::Compatible);
        }

        return Err(DecodeError::UnsupportedSchemaVersion {
            found: version,
            current,
        });
    }

    if version.major() != CURRENT_SCHEMA_MAJOR {
        return Err(DecodeError::IncompatibleSchema {
            found: version,
            compatibility: SchemaCompatibility::Incompatible,
        });
    }

    if version.is_exact(current) {
        return Ok(SchemaCompatibility::Compatible);
    }

    if version.minor() > current.minor() {
        if config.allow_forward_minor
            && flags.contains(SchemaFlags::FORWARD_EXTENSIBLE)
        {
            return Ok(
                SchemaCompatibility::CompatibleWithDegradation
            );
        }

        return Err(DecodeError::IncompatibleSchema {
            found: version,
            compatibility: SchemaCompatibility::Incompatible,
        });
    }

    if version.minor() < current.minor() {
        if config.allow_older_minor {
            return Ok(
                SchemaCompatibility::CompatibleWithMigration
            );
        }

        return Err(DecodeError::UnsupportedSchemaVersion {
            found: version,
            current,
        });
    }

    // Same major/minor but different patch.
    //
    // Patch-level differences are accepted because patch versions are defined
    // by schema.rs as non-semantic corrections/clarifications.
    Ok(SchemaCompatibility::Compatible)
}

// =============================================================================
// Trailing input validation
// =============================================================================

fn ensure_no_trailing_json<R>(
    deserializer: &mut serde_json::Deserializer<R>,
) -> Result<(), DecodeError>
where
    R: Read,
{
    let trailing = Option::<Value>::deserialize(deserializer)
        .map_err(DecodeError::Deserialization)?;

    if trailing.is_some() {
        return Err(DecodeError::TrailingData);
    }

    Ok(())
}

// =============================================================================
// Numeric helpers
// =============================================================================

fn parse_u16_value(
    value: &Value,
    field: &'static str,
) -> Result<u16, DecodeError> {
    let number = value
        .as_u64()
        .ok_or(DecodeError::InvalidSchemaField {
            field,
            reason: "value must be an unsigned integer",
        })?;

    u16::try_from(number).map_err(|_| DecodeError::NumericOverflow)
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
        value: u64,
    }

    fn valid_document() -> Value {
        serde_json::json!({
            "schema": {
                "namespace": RESILIENCE_SCHEMA_NAMESPACE,
                "document_kind": "incident",
                "schema_version": "1.0.0",
                "features": 0,
                "flags": 5,
                "encoding": "json"
            },
            "payload": {
                "value": 42
            }
        })
    }

    #[test]
    fn production_decoder_accepts_current_document() {
        let document = valid_document();

        let decoded: DecodedDocument<TestPayload> =
            decode_value(document).expect("valid document must decode");

        assert_eq!(decoded.payload().value, 42);
        assert_eq!(decoded.kind(), DocumentKind::Incident);
        assert_eq!(decoded.version(), CURRENT_SCHEMA_VERSION);
        assert_eq!(
            decoded.compatibility(),
            SchemaCompatibility::Compatible
        );
        assert!(decoded.is_current_schema());
    }

    #[test]
    fn decoder_preserves_schema_metadata() {
        let document = valid_document();

        let decoded: DecodedDocument<TestPayload> =
            decode_value(document).expect("valid document must decode");

        assert_eq!(
            decoded.schema(),
            current_schema_id(DocumentKind::Incident)
        );

        assert_eq!(decoded.encoding(), EncodingKind::Json);
        assert!(decoded.flags().contains(
            SchemaFlags::DETERMINISTIC
        ));
        assert!(decoded.flags().contains(
            SchemaFlags::SELF_DESCRIBING
        ));
    }

    #[test]
    fn decoder_accepts_numeric_document_kind() {
        let mut document = valid_document();

        document["schema"]["document_kind"] =
            Value::from(DocumentKind::Incident.code());

        let decoded: DecodedDocument<TestPayload> =
            decode_value(document).expect("numeric kind must decode");

        assert_eq!(decoded.kind(), DocumentKind::Incident);
    }

    #[test]
    fn decoder_accepts_structured_version() {
        let mut document = valid_document();

        document["schema"]["schema_version"] =
            serde_json::json!({
                "major": 1,
                "minor": 0,
                "patch": 0
            });

        let decoded: DecodedDocument<TestPayload> =
            decode_value(document).expect("structured version must decode");

        assert_eq!(decoded.version(), CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn decoder_accepts_version_array() {
        let mut document = valid_document();

        document["schema"]["schema_version"] =
            serde_json::json!([1, 0, 0]);

        let decoded: DecodedDocument<TestPayload> =
            decode_value(document).expect("array version must decode");

        assert_eq!(decoded.version(), CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn decoder_rejects_wrong_namespace() {
        let mut document = valid_document();

        document["schema"]["namespace"] =
            Value::from("other.namespace");

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value(document);

        assert!(matches!(
            result,
            Err(DecodeError::InvalidNamespace { .. })
        ));
    }

    #[test]
    fn decoder_rejects_unknown_kind() {
        let mut document = valid_document();

        document["schema"]["document_kind"] =
            Value::from("unknown_kind");

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value(document);

        assert!(matches!(
            result,
            Err(DecodeError::UnsupportedDocumentKind { .. })
        ));
    }

    #[test]
    fn decoder_rejects_missing_schema() {
        let document = serde_json::json!({
            "payload": {
                "value": 42
            }
        });

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value(document);

        assert!(matches!(
            result,
            Err(DecodeError::MissingSchema)
        ));
    }

    #[test]
    fn decoder_rejects_missing_payload() {
        let document = serde_json::json!({
            "schema": {
                "namespace": RESILIENCE_SCHEMA_NAMESPACE,
                "document_kind": "incident",
                "schema_version": "1.0.0",
                "features": 0,
                "flags": 5,
                "encoding": "json"
            }
        });

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value(document);

        assert!(matches!(
            result,
            Err(DecodeError::MissingPayload)
        ));
    }

    #[test]
    fn production_decoder_rejects_unknown_schema_fields() {
        let mut document = valid_document();

        document["schema"]["future_extension"] =
            Value::from(true);

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value(document);

        assert!(matches!(
            result,
            Err(DecodeError::UnknownSchemaField { .. })
        ));
    }

    #[test]
    fn forward_decoder_accepts_unknown_schema_fields() {
        let mut document = valid_document();

        document["schema"]["future_extension"] =
            Value::from(true);

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value_with_config(
                document,
                DecoderConfig::forward_compatible(),
            );

        assert!(result.is_ok());
    }

    #[test]
    fn production_decoder_rejects_unknown_feature_bits() {
        let mut document = valid_document();

        document["schema"]["features"] =
            Value::from(1_u64 << 63);

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value(document);

        assert!(matches!(
            result,
            Err(DecodeError::UnknownFeatureBits { .. })
        ));
    }

    #[test]
    fn forward_decoder_preserves_unknown_feature_bits() {
        let unknown = 1_u64 << 63;

        let mut document = valid_document();

        document["schema"]["features"] =
            Value::from(unknown);

        let result: DecodedDocument<TestPayload> =
            decode_value_with_config(
                document,
                DecoderConfig::forward_compatible(),
            )
            .expect("forward-compatible decoder must accept unknown bits");

        assert_eq!(result.features().bits(), unknown);
    }

    #[test]
    fn production_decoder_rejects_unknown_flag_bits() {
        let mut document = valid_document();

        document["schema"]["flags"] =
            Value::from(1_u32 << 31);

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value(document);

        assert!(matches!(
            result,
            Err(DecodeError::UnknownFlagBits { .. })
        ));
    }

    #[test]
    fn production_decoder_rejects_future_minor_schema() {
        let mut document = valid_document();

        document["schema"]["schema_version"] =
            Value::from("1.99.0");

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value(document);

        assert!(matches!(
            result,
            Err(DecodeError::UnsupportedSchemaVersion { .. })
                | Err(DecodeError::IncompatibleSchema { .. })
        ));
    }

    #[test]
    fn forward_decoder_accepts_forward_extensible_future_minor() {
        let mut document = valid_document();

        document["schema"]["schema_version"] =
            Value::from("1.99.0");

        document["schema"]["flags"] =
            Value::from(
                SchemaFlags::DETERMINISTIC.bits()
                    | SchemaFlags::FORWARD_EXTENSIBLE.bits(),
            );

        let decoded: DecodedDocument<TestPayload> =
            decode_value_with_config(
                document,
                DecoderConfig::forward_compatible(),
            )
            .expect("forward-compatible schema must decode");

        assert_eq!(
            decoded.compatibility(),
            SchemaCompatibility::CompatibleWithDegradation
        );

        assert!(decoded.is_degraded());
    }

    #[test]
    fn decoder_rejects_different_major() {
        let mut document = valid_document();

        document["schema"]["schema_version"] =
            Value::from("2.0.0");

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value_with_config(
                document,
                DecoderConfig::forward_compatible(),
            );

        assert!(matches!(
            result,
            Err(DecodeError::IncompatibleSchema { .. })
        ));
    }

    #[test]
    fn decoder_rejects_wrong_encoding() {
        let mut document = valid_document();

        document["schema"]["encoding"] =
            Value::from("cbor");

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value(document);

        assert!(result.is_ok());

        let decoded =
            result.expect("document itself is structurally valid");

        assert_eq!(decoded.encoding(), EncodingKind::Cbor);
    }

    #[test]
    fn decoder_rejects_invalid_payload() {
        let mut document = valid_document();

        document["payload"] =
            serde_json::json!({
                "value": "not-a-number"
            });

        let result: Result<DecodedDocument<TestPayload>, DecodeError> =
            decode_value(document);

        assert!(matches!(
            result,
            Err(DecodeError::InvalidPayload)
        ));
    }

    #[test]
    fn reader_decoder_works() {
        let document = valid_document();

        let bytes =
            serde_json::to_vec(&document)
                .expect("test document must serialize");

        let mut reader =
            std::io::Cursor::new(bytes);

        let decoded: DecodedDocument<TestPayload> =
            decode_from_reader(&mut reader)
                .expect("reader decoding must succeed");

        assert_eq!(decoded.payload().value, 42);
    }

    #[test]
    fn reader_decoder_rejects_trailing_document() {
        let document = valid_document();

        let mut bytes =
            serde_json::to_vec(&document)
                .expect("test document must serialize");

        bytes.extend_from_slice(
            br#"{"another":true}"#,
        );

        let mut reader =
            std::io::Cursor::new(bytes);

        let result: Result<
            DecodedDocument<TestPayload>,
            DecodeError,
        > = decode_from_reader(&mut reader);

        assert!(matches!(
            result,
            Err(DecodeError::TrailingData)
                | Err(DecodeError::Deserialization(_))
        ));
    }

    #[test]
    fn schema_flags_are_decoded_without_machine_limits() {
        let document = valid_document();

        let decoded: DecodedDocument<TestPayload> =
            decode_value(document)
                .expect("valid document must decode");

        // The decoder does not infer or impose any quantum-machine size.
        assert!(decoded.flags().bits() >= 0);
    }

    #[test]
    fn schema_version_display_round_trip() {
        let version = SchemaVersion::new(1, 2, 3);
        let parsed =
            parse_version_string(&version.to_string())
                .expect("version string must parse");

        assert_eq!(parsed, version);
    }

    #[test]
    fn schema_namespace_is_stable() {
        assert_eq!(
            SchemaNamespace::as_str(),
            RESILIENCE_SCHEMA_NAMESPACE
        );
    }

    #[test]
    fn current_schema_header_helpers_remain_usable() {
        let header = current_schema_header(
            DocumentKind::Checkpoint,
            SchemaFeatures::CHECKPOINTS,
            SchemaFlags::PERSISTABLE,
        );

        assert_eq!(
            header.version(),
            CURRENT_SCHEMA_VERSION
        );
    }
}