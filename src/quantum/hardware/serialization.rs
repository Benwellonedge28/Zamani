//! Zamani Quantum — Hardware Serialization
//!
//! Production-grade, deterministic, versioned serialization boundary for
//! `crate::quantum::hardware`.
//!
//! # Responsibility
//!
//! This module is the authoritative serialization infrastructure for the
//! hardware subsystem.
//!
//! It provides:
//!
//! - versioned serialization envelopes;
//! - schema identification;
//! - schema-version checking;
//! - deterministic JSON serialization;
//! - bounded deserialization;
//! - payload-size protection;
//! - UTF-8-safe text handling;
//! - SHA-256 content fingerprints;
//! - canonical JSON representation;
//! - serialization/deserialization helpers;
//! - integrity verification;
//! - forward-compatibility policy;
//! - backward-compatibility policy;
//! - explicit unknown-schema rejection;
//! - explicit unsupported-version rejection;
//! - serialization error classification;
//! - safe handling of untrusted provider/device data;
//! - stable persisted representations for hardware state;
//! - serialization contracts consumed by backend, topology,
//!   calibration, capabilities, execution, jobs, providers, registries,
//!   adapters, benchmarking, Danga, and diagnostics.
//!
//! # Non-responsibilities
//!
//! This module does NOT:
//!
//! - communicate with quantum hardware;
//! - communicate with provider APIs;
//! - authenticate;
//! - store credentials;
//! - store API keys;
//! - perform encryption at rest;
//! - sign documents;
//! - verify digital signatures;
//! - perform quantum compilation;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - perform benchmarking;
//! - execute jobs;
//! - define backend capabilities;
//! - define topology semantics;
//! - define calibration semantics;
//! - define provider-specific schemas.
//!
//! Encryption/signatures belong to security/cryptographic layers.
//! Provider-specific serialization belongs to provider adapters.
//!
//! # Architectural position
//!
//! ```text
//!                    Zamani Quantum IR
//!                           |
//!                           v
//!                    Hardware subsystem
//!                           |
//!        +------------------+------------------+
//!        |                  |                  |
//!        v                  v                  v
//!      Backend           Topology         Calibration
//!        |                  |                  |
//!        +------------------+------------------+
//!                           |
//!                           v
//!                    serialization.rs
//!                           |
//!             +-------------+-------------+
//!             |                           |
//!             v                           v
//!        canonical JSON             fingerprints
//!             |                           |
//!             +-------------+-------------+
//!                           |
//!                           v
//!              persistence / transport / cache
//!
//! Provider adapters may serialize provider-native payloads independently.
//! They must not redefine the Zamani hardware serialization envelope.
//! ```
//!
//! # Design rule
//!
//! The most important rule in this module is:
//!
//! > Serialization is a transport/persistence concern, not a semantic
//! > ownership mechanism.
//!
//! A type owns its meaning. This module owns how an already-defined type is
//! represented, versioned, bounded, fingerprinted, and safely recovered.
//!
//! # Determinism
//!
//! Production quantum compilation, benchmarking, caching, reproducibility,
//! and auditability require deterministic serialization.
//!
//! Therefore:
//!
//! - callers should use `BTreeMap`/`BTreeSet` for unordered logical data;
//! - this module uses an explicit envelope;
//! - schema metadata is represented deterministically;
//! - fingerprints are calculated over canonical bytes;
//! - no timestamps are inserted implicitly;
//! - no random values are inserted implicitly;
//! - no machine-local paths are inserted implicitly;
//! - no environment variables are read;
//! - no network state is read.
//!
//! # Fingerprints
//!
//! A fingerprint is an integrity/content identifier.
//!
//! It is NOT:
//!
//! - a digital signature;
//! - proof of authenticity;
//! - proof that a provider supplied the document;
//! - proof that a device is genuine.
//!
//! Authenticity requires a separate signature/attestation layer.
//!
//! # Security
//!
//! Deserialization must be treated as an untrusted-input boundary.
//!
//! The implementation therefore enforces:
//!
//! - maximum serialized document size;
//! - maximum schema identifier size;
//! - maximum schema version range;
//! - valid UTF-8 through Rust `String`/`&str`;
//! - exact schema matching;
//! - optional fingerprint verification;
//! - rejection of malformed JSON;
//! - rejection of missing envelope fields;
//! - rejection of unsupported schema versions.
//!
//! This module never logs serialized payloads because payloads may contain
//! provider metadata that callers do not intend to expose.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! # Dependencies
//!
//! Uses dependencies already present in `Cargo.toml`:
//!
//! - `serde`;
//! - `serde_json`;
//! - `sha2`.
//!
//! No new Cargo dependency is required.
//!
//! # Integration contract
//!
//! This module deliberately does not depend on `backend.rs`,
//! `calibration.rs`, `topology.rs`, `capabilities.rs`, `execution.rs`,
//! `job.rs`, or provider adapters.
//!
//! That is intentional.
//!
//! Every hardware model can independently implement `Serialize` and
//! `Deserialize`, then use this module without creating a circular dependency.
//!
//! Consumers:
//!
//! ```text
//! hardware::backend
//! hardware::capabilities
//! hardware::topology
//! hardware::calibration
//! hardware::execution
//! hardware::job
//! hardware::provider
//! hardware::registry
//! hardware::adapters
//! benchmarking
//! Danga
//! ```
//!
//! The serialization layer therefore remains frozen while the semantic
//! hardware modules evolve.
//!
//! # Compatibility policy
//!
//! `schema_id` identifies the semantic document family.
//!
//! `schema_version` identifies the serialized representation version.
//!
//! A reader MUST:
//!
//! 1. verify the schema identifier;
//! 2. verify the schema version;
//! 3. reject unsupported versions unless an explicit migration exists;
//! 4. deserialize only after those checks.
//!
//! Unknown schemas must never be guessed.
//!
//! Unknown future versions must never be silently interpreted as the current
//! version.
//!
//! # Example
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use zamani_compiler::quantum::hardware::serialization::{
//!     deserialize_document,
//!     serialize_document,
//! };
//!
//! #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
//! struct Example {
//!     value: String,
//! }
//!
//! let value = Example {
//!     value: "zamani".to_owned(),
//! };
//!
//! let encoded = serialize_document(
//!     "zamani.quantum.hardware.example",
//!     1,
//!     &value,
//! ).expect("serialization must succeed");
//!
//! let decoded: Example = deserialize_document(
//!     "zamani.quantum.hardware.example",
//!     1,
//!     &encoded,
//! ).expect("deserialization must succeed");
//!
//! assert_eq!(value, decoded);
//! ```
//!
//! # Stability
//!
//! The following public types and functions form the stable serialization
//! boundary:
//!
//! - `SERIALIZATION_SCHEMA_ID`;
//! - `SERIALIZATION_SCHEMA_VERSION`;
//! - `MAX_SERIALIZED_DOCUMENT_BYTES`;
//! - `MAX_SCHEMA_ID_LENGTH`;
//! - `SerializationFormat`;
//! - `SerializationError`;
//! - `DocumentEnvelope`;
//! - `SerializedDocument`;
//! - `serialize_document`;
//! - `serialize_document_with_options`;
//! - `deserialize_document`;
//! - `deserialize_document_with_options`;
//! - `canonicalize_json`;
//! - `fingerprint_bytes`;
//! - `fingerprint_str`;
//! - `verify_fingerprint`.
//!
//! Existing hardware modules can therefore integrate against this API without
//! requiring this file to be changed later.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable identifier for this serialization infrastructure.
pub const SERIALIZATION_SCHEMA_ID: &str = "zamani.quantum.hardware.serialization";

/// Version of the serialization infrastructure contract.
pub const SERIALIZATION_SCHEMA_VERSION: u16 = 1;

/// Maximum serialized document size accepted by the safe default APIs.
///
/// This is deliberately bounded because hardware/provider payloads are
/// untrusted external input. Applications requiring larger documents should
/// use a streaming/transport layer rather than weakening this core bound.
pub const MAX_SERIALIZED_DOCUMENT_BYTES: usize = 64 * 1024 * 1024;

/// Maximum schema identifier length in bytes.
pub const MAX_SCHEMA_ID_LENGTH: usize = 512;

/// Maximum serialized format identifier length.
pub const MAX_FORMAT_ID_LENGTH: usize = 64;

/// Maximum fingerprint length in bytes when represented as hexadecimal text.
pub const FINGERPRINT_HEX_LENGTH: usize = 64;

/// Maximum supported schema version representable by the generic envelope.
///
/// This prevents absurd version values from being accepted as meaningful
/// protocol versions.
pub const MAX_SCHEMA_VERSION: u16 = 10_000;

/// Maximum nesting depth checked while canonicalizing JSON.
///
/// This is a defensive limit against pathological JSON structures.
pub const DEFAULT_MAX_JSON_DEPTH: usize = 256;

// =============================================================================
// Serialization format
// =============================================================================

/// Supported wire representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SerializationFormat {
    /// Canonical JSON representation.
    Json,
}

impl SerializationFormat {
    /// Stable machine-readable format identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
        }
    }

    /// Returns the currently supported format version.
    pub const fn version(self) -> u16 {
        match self {
            Self::Json => 1,
        }
    }
}

impl fmt::Display for SerializationFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Options
// =============================================================================

/// Options controlling serialization behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SerializationOptions {
    /// Maximum encoded document size.
    pub max_document_bytes: usize,

    /// Maximum JSON nesting depth.
    pub max_json_depth: usize,

    /// Whether a content fingerprint is included in the envelope.
    pub include_fingerprint: bool,
}

impl Default for SerializationOptions {
    fn default() -> Self {
        Self {
            max_document_bytes: MAX_SERIALIZED_DOCUMENT_BYTES,
            max_json_depth: DEFAULT_MAX_JSON_DEPTH,
            include_fingerprint: true,
        }
    }
}

impl SerializationOptions {
    /// Creates conservative production defaults.
    pub const fn production() -> Self {
        Self {
            max_document_bytes: MAX_SERIALIZED_DOCUMENT_BYTES,
            max_json_depth: DEFAULT_MAX_JSON_DEPTH,
            include_fingerprint: true,
        }
    }

    /// Validates the options before use.
    pub fn validate(self) -> Result<Self, SerializationError> {
        if self.max_document_bytes == 0 {
            return Err(SerializationError::InvalidLimit {
                field: "max_document_bytes",
                value: self.max_document_bytes,
            });
        }

        if self.max_document_bytes > MAX_SERIALIZED_DOCUMENT_BYTES {
            return Err(SerializationError::LimitTooLarge {
                field: "max_document_bytes",
                value: self.max_document_bytes,
                maximum: MAX_SERIALIZED_DOCUMENT_BYTES,
            });
        }

        if self.max_json_depth == 0 {
            return Err(SerializationError::InvalidLimit {
                field: "max_json_depth",
                value: self.max_json_depth,
            });
        }

        Ok(self)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the authoritative hardware serialization boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    /// A required schema identifier was empty.
    EmptySchemaId,

    /// A schema identifier exceeded the permitted length.
    SchemaIdTooLong {
        length: usize,
        maximum: usize,
    },

    /// The schema identifier contained invalid characters.
    InvalidSchemaId,

    /// A format identifier was invalid.
    InvalidFormat,

    /// A schema version is invalid.
    InvalidSchemaVersion {
        version: u16,
    },

    /// The serialized payload exceeded the configured limit.
    DocumentTooLarge {
        size: usize,
        maximum: usize,
    },

    /// A configured limit was zero.
    InvalidLimit {
        field: &'static str,
        value: usize,
    },

    /// A configured limit exceeded the production maximum.
    LimitTooLarge {
        field: &'static str,
        value: usize,
        maximum: usize,
    },

    /// Serialization itself failed.
    Serialize {
        message: String,
    },

    /// Deserialization itself failed.
    Deserialize {
        message: String,
    },

    /// The outer envelope is malformed.
    InvalidEnvelope {
        message: String,
    },

    /// The caller expected one schema but received another.
    SchemaMismatch {
        expected: String,
        actual: String,
    },

    /// The caller expected one version but received another.
    UnsupportedSchemaVersion {
        schema_id: String,
        expected: u16,
        actual: u16,
    },

    /// The document contains a fingerprint that does not match its payload.
    FingerprintMismatch {
        expected: String,
        actual: String,
    },

    /// A fingerprint has invalid hexadecimal representation.
    InvalidFingerprint {
        fingerprint: String,
    },

    /// JSON nesting exceeded the configured limit.
    JsonDepthExceeded {
        maximum: usize,
    },

    /// The document's JSON root was not an object.
    RootMustBeObject,

    /// An envelope field has an invalid value.
    InvalidField {
        field: &'static str,
        message: String,
    },
}

impl fmt::Display for SerializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySchemaId => {
                formatter.write_str("serialization schema identifier cannot be empty")
            }

            Self::SchemaIdTooLong { length, maximum } => {
                write!(
                    formatter,
                    "serialization schema identifier is {length} bytes; maximum is {maximum}"
                )
            }

            Self::InvalidSchemaId => {
                formatter.write_str("serialization schema identifier contains invalid characters")
            }

            Self::InvalidFormat => {
                formatter.write_str("unsupported serialization format")
            }

            Self::InvalidSchemaVersion { version } => {
                write!(
                    formatter,
                    "invalid serialization schema version {version}"
                )
            }

            Self::DocumentTooLarge { size, maximum } => {
                write!(
                    formatter,
                    "serialized document is {size} bytes; maximum is {maximum}"
                )
            }

            Self::InvalidLimit { field, value } => {
                write!(formatter, "{field} must be greater than zero, got {value}")
            }

            Self::LimitTooLarge {
                field,
                value,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} is {value}; maximum permitted value is {maximum}"
                )
            }

            Self::Serialize { message } => {
                write!(formatter, "serialization failed: {message}")
            }

            Self::Deserialize { message } => {
                write!(formatter, "deserialization failed: {message}")
            }

            Self::InvalidEnvelope { message } => {
                write!(formatter, "invalid serialization envelope: {message}")
            }

            Self::SchemaMismatch { expected, actual } => {
                write!(
                    formatter,
                    "schema mismatch: expected `{expected}`, received `{actual}`"
                )
            }

            Self::UnsupportedSchemaVersion {
                schema_id,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "unsupported schema version for `{schema_id}`: \
                     expected {expected}, received {actual}"
                )
            }

            Self::FingerprintMismatch { expected, actual } => {
                write!(
                    formatter,
                    "serialization fingerprint mismatch: expected `{expected}`, \
                     calculated `{actual}`"
                )
            }

            Self::InvalidFingerprint { fingerprint } => {
                write!(
                    formatter,
                    "invalid hexadecimal fingerprint `{fingerprint}`"
                )
            }

            Self::JsonDepthExceeded { maximum } => {
                write!(
                    formatter,
                    "JSON nesting exceeds maximum depth of {maximum}"
                )
            }

            Self::RootMustBeObject => {
                formatter.write_str("serialized document root must be a JSON object")
            }

            Self::InvalidField { field, message } => {
                write!(
                    formatter,
                    "invalid serialization envelope field `{field}`: {message}"
                )
            }
        }
    }
}

impl std::error::Error for SerializationError {}

// =============================================================================
// Envelope
// =============================================================================

/// Versioned outer envelope used by persisted/transmitted Zamani hardware
/// documents.
///
/// The envelope deliberately contains semantic schema metadata outside the
/// serialized payload.
///
/// This lets readers reject incompatible documents before attempting to
/// deserialize provider/device state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentEnvelope {
    /// Stable semantic schema identifier.
    pub schema_id: String,

    /// Semantic serialized representation version.
    pub schema_version: u16,

    /// Serialization format.
    pub format: String,

    /// Format version.
    pub format_version: u16,

    /// Canonical SHA-256 fingerprint of the payload.
    ///
    /// This is a content fingerprint, not a signature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Actual serialized domain object.
    pub payload: Value,
}

impl DocumentEnvelope {
    /// Creates a validated envelope from a serializable value.
    pub fn new<T: Serialize>(
        schema_id: impl Into<String>,
        schema_version: u16,
        value: &T,
        options: SerializationOptions,
    ) -> Result<Self, SerializationError> {
        options.validate()?;

        let schema_id = schema_id.into();
        validate_schema_id(&schema_id)?;
        validate_schema_version(schema_version)?;

        let payload = serde_json::to_value(value).map_err(|error| {
            SerializationError::Serialize {
                message: error.to_string(),
            }
        })?;

        if !payload.is_object() {
            return Err(SerializationError::RootMustBeObject);
        }

        validate_json_depth(&payload, options.max_json_depth)?;

        let canonical_payload = canonical_json_bytes(&payload)?;
        enforce_size(canonical_payload.len(), options.max_document_bytes)?;

        let fingerprint = if options.include_fingerprint {
            Some(fingerprint_bytes(&canonical_payload))
        } else {
            None
        };

        Ok(Self {
            schema_id,
            schema_version,
            format: SerializationFormat::Json.as_str().to_owned(),
            format_version: SerializationFormat::Json.version(),
            fingerprint,
            payload,
        })
    }

    /// Validates envelope metadata and payload without deserializing the
    /// domain-specific object.
    pub fn validate(
        &self,
        options: SerializationOptions,
    ) -> Result<(), SerializationError> {
        options.validate()?;

        validate_schema_id(&self.schema_id)?;
        validate_schema_version(self.schema_version)?;

        if self.format != SerializationFormat::Json.as_str() {
            return Err(SerializationError::InvalidFormat);
        }

        if self.format_version != SerializationFormat::Json.version() {
            return Err(SerializationError::InvalidField {
                field: "format_version",
                message: format!(
                    "expected {}, received {}",
                    SerializationFormat::Json.version(),
                    self.format_version
                ),
            });
        }

        if !self.payload.is_object() {
            return Err(SerializationError::RootMustBeObject);
        }

        validate_json_depth(&self.payload, options.max_json_depth)?;

        let canonical_payload = canonical_json_bytes(&self.payload)?;
        enforce_size(canonical_payload.len(), options.max_document_bytes)?;

        if let Some(expected) = &self.fingerprint {
            let actual = fingerprint_bytes(&canonical_payload);

            if !constant_time_equal_ascii(expected.as_bytes(), actual.as_bytes()) {
                return Err(SerializationError::FingerprintMismatch {
                    expected: expected.clone(),
                    actual,
                });
            }
        }

        Ok(())
    }

    /// Returns the canonical payload fingerprint.
    pub fn calculated_fingerprint(&self) -> Result<String, SerializationError> {
        let payload = canonical_json_bytes(&self.payload)?;
        Ok(fingerprint_bytes(&payload))
    }
}

/// Serialized document returned by the serialization helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializedDocument {
    /// Canonical UTF-8 JSON bytes.
    bytes: Vec<u8>,
    /// Document fingerprint.
    fingerprint: String,
    /// Schema identifier.
    schema_id: String,
    /// Schema version.
    schema_version: u16,
}

impl SerializedDocument {
    /// Returns the serialized UTF-8 bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the serialized UTF-8 string.
    ///
    /// Canonical JSON generated by this module is always valid UTF-8.
    pub fn as_str(&self) -> &str {
        // `serialize_document` constructs this value from UTF-8 JSON produced
        // by serde_json. Keeping the conversion centralized avoids exposing
        // unchecked UTF-8 assumptions elsewhere.
        std::str::from_utf8(&self.bytes)
            .expect("canonical serde_json output must always be valid UTF-8")
    }

    /// Returns the SHA-256 content fingerprint.
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Returns the schema identifier.
    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    /// Returns the schema version.
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Consumes the wrapper and returns the owned bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

// =============================================================================
// Public serialization API
// =============================================================================

/// Serializes a hardware-domain object using production defaults.
pub fn serialize_document<T: Serialize>(
    schema_id: impl Into<String>,
    schema_version: u16,
    value: &T,
) -> Result<SerializedDocument, SerializationError> {
    serialize_document_with_options(
        schema_id,
        schema_version,
        value,
        SerializationOptions::production(),
    )
}

/// Serializes a hardware-domain object using explicit safety limits.
pub fn serialize_document_with_options<T: Serialize>(
    schema_id: impl Into<String>,
    schema_version: u16,
    value: &T,
    options: SerializationOptions,
) -> Result<SerializedDocument, SerializationError> {
    options.validate()?;

    let envelope = DocumentEnvelope::new(
        schema_id,
        schema_version,
        value,
        options,
    )?;

    let json = canonical_envelope_json(&envelope, options.max_json_depth)?;

    enforce_size(json.len(), options.max_document_bytes)?;

    let fingerprint = fingerprint_bytes(&json);

    Ok(SerializedDocument {
        bytes: json,
        fingerprint,
        schema_id: envelope.schema_id,
        schema_version: envelope.schema_version,
    })
}

/// Deserializes a hardware-domain object using production defaults.
///
/// The schema and version are checked before the domain object is decoded.
pub fn deserialize_document<T: DeserializeOwned>(
    expected_schema_id: &str,
    expected_schema_version: u16,
    bytes: &[u8],
) -> Result<T, SerializationError> {
    deserialize_document_with_options(
        expected_schema_id,
        expected_schema_version,
        bytes,
        SerializationOptions::production(),
    )
}

/// Deserializes a hardware-domain object using explicit safety limits.
pub fn deserialize_document_with_options<T: DeserializeOwned>(
    expected_schema_id: &str,
    expected_schema_version: u16,
    bytes: &[u8],
    options: SerializationOptions,
) -> Result<T, SerializationError> {
    options.validate()?;

    validate_schema_id(expected_schema_id)?;
    validate_schema_version(expected_schema_version)?;

    enforce_size(bytes.len(), options.max_document_bytes)?;

    let envelope: DocumentEnvelope =
        serde_json::from_slice(bytes).map_err(|error| {
            SerializationError::Deserialize {
                message: error.to_string(),
            }
        })?;

    envelope.validate(options)?;

    if envelope.schema_id != expected_schema_id {
        return Err(SerializationError::SchemaMismatch {
            expected: expected_schema_id.to_owned(),
            actual: envelope.schema_id,
        });
    }

    if envelope.schema_version != expected_schema_version {
        return Err(SerializationError::UnsupportedSchemaVersion {
            schema_id: expected_schema_id.to_owned(),
            expected: expected_schema_version,
            actual: envelope.schema_version,
        });
    }

    serde_json::from_value(envelope.payload).map_err(|error| {
        SerializationError::Deserialize {
            message: error.to_string(),
        }
    })
}

/// Parses and validates a serialized envelope without decoding its domain
/// payload.
///
/// This is useful for registries and discovery code that need to inspect
/// schema metadata before deciding which domain type should be loaded.
pub fn inspect_document(
    bytes: &[u8],
    options: SerializationOptions,
) -> Result<DocumentEnvelope, SerializationError> {
    options.validate()?;

    enforce_size(bytes.len(), options.max_document_bytes)?;

    let envelope: DocumentEnvelope =
        serde_json::from_slice(bytes).map_err(|error| {
            SerializationError::Deserialize {
                message: error.to_string(),
            }
        })?;

    envelope.validate(options)?;

    Ok(envelope)
}

// =============================================================================
// Canonical JSON
// =============================================================================

/// Produces canonical JSON bytes for a JSON value.
///
/// Object keys are recursively sorted lexicographically. Arrays preserve their
/// semantic ordering because array order is meaningful in JSON.
pub fn canonical_json_bytes(value: &Value) -> Result<Vec<u8>, SerializationError> {
    let canonical = canonicalize_value(value, 0, DEFAULT_MAX_JSON_DEPTH)?;

    serde_json::to_vec(&canonical).map_err(|error| SerializationError::Serialize {
        message: error.to_string(),
    })
}

/// Produces canonical JSON text for a JSON value.
pub fn canonicalize_json(value: &Value) -> Result<String, SerializationError> {
    let bytes = canonical_json_bytes(value)?;

    String::from_utf8(bytes).map_err(|error| SerializationError::Serialize {
        message: error.to_string(),
    })
}

/// Produces canonical JSON bytes for the complete serialization envelope.
fn canonical_envelope_json(
    envelope: &DocumentEnvelope,
    max_depth: usize,
) -> Result<Vec<u8>, SerializationError> {
    let value = serde_json::to_value(envelope).map_err(|error| {
        SerializationError::Serialize {
            message: error.to_string(),
        }
    })?;

    let canonical = canonicalize_value(&value, 0, max_depth)?;

    serde_json::to_vec(&canonical).map_err(|error| {
        SerializationError::Serialize {
            message: error.to_string(),
        }
    })
}

/// Recursively canonicalizes JSON objects.
///
/// `serde_json::Map` ordering depends on its internal representation. Building
/// a new map from sorted keys gives this module deterministic behavior without
/// depending on the `preserve_order` feature.
fn canonicalize_value(
    value: &Value,
    depth: usize,
    maximum_depth: usize,
) -> Result<Value, SerializationError> {
    if depth > maximum_depth {
        return Err(SerializationError::JsonDepthExceeded {
            maximum: maximum_depth,
        });
    }

    match value {
        Value::Null => Ok(Value::Null),

        Value::Bool(value) => Ok(Value::Bool(*value)),

        Value::Number(value) => Ok(Value::Number(value.clone())),

        Value::String(value) => Ok(Value::String(value.clone())),

        Value::Array(values) => {
            let mut canonical_values = Vec::with_capacity(values.len());

            for value in values {
                canonical_values.push(canonicalize_value(
                    value,
                    depth + 1,
                    maximum_depth,
                )?);
            }

            Ok(Value::Array(canonical_values))
        }

        Value::Object(object) => {
            let mut keys: Vec<&String> = object.keys().collect();
            keys.sort();

            let mut canonical = Map::new();

            for key in keys {
                let value = object
                    .get(key)
                    .ok_or_else(|| SerializationError::InvalidEnvelope {
                        message: format!(
                            "object key `{key}` disappeared during canonicalization"
                        ),
                    })?;

                canonical.insert(
                    key.clone(),
                    canonicalize_value(
                        value,
                        depth + 1,
                        maximum_depth,
                    )?,
                );
            }

            Ok(Value::Object(canonical))
        }
    }
}

// =============================================================================
// Fingerprints
// =============================================================================

/// Calculates a SHA-256 fingerprint and returns lowercase hexadecimal text.
pub fn fingerprint_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    hex_encode(&digest)
}

/// Calculates a SHA-256 fingerprint over UTF-8 text.
pub fn fingerprint_str(value: &str) -> String {
    fingerprint_bytes(value.as_bytes())
}

/// Verifies a hexadecimal SHA-256 fingerprint against bytes.
///
/// Comparison is performed without early-exit equality on the hexadecimal
/// representation.
pub fn verify_fingerprint(
    bytes: &[u8],
    expected: &str,
) -> Result<(), SerializationError> {
    validate_fingerprint(expected)?;

    let actual = fingerprint_bytes(bytes);

    if constant_time_equal_ascii(expected.as_bytes(), actual.as_bytes()) {
        Ok(())
    } else {
        Err(SerializationError::FingerprintMismatch {
            expected: expected.to_owned(),
            actual,
        })
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_schema_id(schema_id: &str) -> Result<(), SerializationError> {
    if schema_id.is_empty() {
        return Err(SerializationError::EmptySchemaId);
    }

    if schema_id.len() > MAX_SCHEMA_ID_LENGTH {
        return Err(SerializationError::SchemaIdTooLong {
            length: schema_id.len(),
            maximum: MAX_SCHEMA_ID_LENGTH,
        });
    }

    // Schema identifiers are deliberately restricted to a conservative
    // machine-readable namespace. This avoids ambiguous persistence keys.
    for character in schema_id.chars() {
        let valid = character.is_ascii_alphanumeric()
            || matches!(
                character,
                '.' | '-' | '_' | '/' | ':'
            );

        if !valid {
            return Err(SerializationError::InvalidSchemaId);
        }
    }

    Ok(())
}

fn validate_schema_version(
    version: u16,
) -> Result<(), SerializationError> {
    if version == 0 || version > MAX_SCHEMA_VERSION {
        return Err(SerializationError::InvalidSchemaVersion {
            version,
        });
    }

    Ok(())
}

fn validate_fingerprint(
    fingerprint: &str,
) -> Result<(), SerializationError> {
    if fingerprint.len() != FINGERPRINT_HEX_LENGTH {
        return Err(SerializationError::InvalidFingerprint {
            fingerprint: fingerprint.to_owned(),
        });
    }

    if !fingerprint
        .bytes()
        .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(SerializationError::InvalidFingerprint {
            fingerprint: fingerprint.to_owned(),
        });
    }

    Ok(())
}

fn enforce_size(
    size: usize,
    maximum: usize,
) -> Result<(), SerializationError> {
    if size > maximum {
        return Err(SerializationError::DocumentTooLarge {
            size,
            maximum,
        });
    }

    Ok(())
}

fn validate_json_depth(
    value: &Value,
    maximum_depth: usize,
) -> Result<(), SerializationError> {
    validate_json_depth_inner(value, 0, maximum_depth)
}

fn validate_json_depth_inner(
    value: &Value,
    depth: usize,
    maximum_depth: usize,
) -> Result<(), SerializationError> {
    if depth > maximum_depth {
        return Err(SerializationError::JsonDepthExceeded {
            maximum: maximum_depth,
        });
    }

    match value {
        Value::Array(values) => {
            for child in values {
                validate_json_depth_inner(
                    child,
                    depth + 1,
                    maximum_depth,
                )?;
            }
        }

        Value::Object(object) => {
            for child in object.values() {
                validate_json_depth_inner(
                    child,
                    depth + 1,
                    maximum_depth,
                )?;
            }
        }

        Value::Null
        | Value::Bool(_)
        | Value::Number(_)
        | Value::String(_) => {}
    }

    Ok(())
}

// =============================================================================
// Constant-time-ish comparison
// =============================================================================

/// Compares two ASCII byte strings without returning early because of a
/// differing byte.
///
/// This is appropriate for fingerprints where avoiding obvious comparison
/// timing differences is desirable. It is not a substitute for a MAC or
/// cryptographic signature.
fn constant_time_equal_ascii(
    left: &[u8],
    right: &[u8],
) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut difference: u8 = 0;

    for index in 0..left.len() {
        difference |= left[index] ^ right[index];
    }

    difference == 0
}

// =============================================================================
// Minimal hexadecimal encoder
// =============================================================================

/// Encodes bytes as lowercase hexadecimal without introducing another crate.
fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut output = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
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

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct Example {
        name: String,
        values: Vec<u64>,
    }

    const EXAMPLE_SCHEMA: &str =
        "zamani.quantum.hardware.test";

    #[test]
    fn production_options_are_valid() {
        assert!(
            SerializationOptions::production()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn schema_validation_rejects_empty_identifier() {
        let result = validate_schema_id("");

        assert_eq!(
            result,
            Err(SerializationError::EmptySchemaId)
        );
    }

    #[test]
    fn schema_validation_rejects_invalid_characters() {
        let result = validate_schema_id("zamani quantum hardware");

        assert_eq!(
            result,
            Err(SerializationError::InvalidSchemaId)
        );
    }

    #[test]
    fn schema_validation_accepts_canonical_identifier() {
        assert!(
            validate_schema_id(
                "zamani.quantum.hardware.backend"
            )
            .is_ok()
        );
    }

    #[test]
    fn version_zero_is_rejected() {
        assert_eq!(
            validate_schema_version(0),
            Err(SerializationError::InvalidSchemaVersion {
                version: 0
            })
        );
    }

    #[test]
    fn serialization_round_trip_is_lossless() {
        let original = Example {
            name: "zamani".to_owned(),
            values: vec![1, 2, 3, 5, 8],
        };

        let serialized = serialize_document(
            EXAMPLE_SCHEMA,
            1,
            &original,
        )
        .expect("serialization must succeed");

        let decoded: Example = deserialize_document(
            EXAMPLE_SCHEMA,
            1,
            serialized.as_bytes(),
        )
        .expect("deserialization must succeed");

        assert_eq!(original, decoded);
    }

    #[test]
    fn serialization_contains_schema_metadata() {
        let original = Example {
            name: "hardware".to_owned(),
            values: vec![42],
        };

        let serialized = serialize_document(
            EXAMPLE_SCHEMA,
            7,
            &original,
        )
        .expect("serialization must succeed");

        let envelope = inspect_document(
            serialized.as_bytes(),
            SerializationOptions::production(),
        )
        .expect("inspection must succeed");

        assert_eq!(envelope.schema_id, EXAMPLE_SCHEMA);
        assert_eq!(envelope.schema_version, 7);
        assert_eq!(
            envelope.format,
            SerializationFormat::Json.as_str()
        );
        assert_eq!(
            envelope.format_version,
            SerializationFormat::Json.version()
        );
        assert!(envelope.fingerprint.is_some());
    }

    #[test]
    fn wrong_schema_is_rejected_before_domain_decode() {
        let original = Example {
            name: "hardware".to_owned(),
            values: vec![1],
        };

        let serialized = serialize_document(
            EXAMPLE_SCHEMA,
            1,
            &original,
        )
        .expect("serialization must succeed");

        let result: Result<Example, SerializationError> =
            deserialize_document(
                "zamani.quantum.hardware.other",
                1,
                serialized.as_bytes(),
            );

        assert!(matches!(
            result,
            Err(SerializationError::SchemaMismatch { .. })
        ));
    }

    #[test]
    fn wrong_version_is_rejected() {
        let original = Example {
            name: "hardware".to_owned(),
            values: vec![1],
        };

        let serialized = serialize_document(
            EXAMPLE_SCHEMA,
            1,
            &original,
        )
        .expect("serialization must succeed");

        let result: Result<Example, SerializationError> =
            deserialize_document(
                EXAMPLE_SCHEMA,
                2,
                serialized.as_bytes(),
            );

        assert!(matches!(
            result,
            Err(SerializationError::UnsupportedSchemaVersion { .. })
        ));
    }

    #[test]
    fn tampering_is_detected() {
        let original = Example {
            name: "hardware".to_owned(),
            values: vec![1, 2, 3],
        };

        let serialized = serialize_document(
            EXAMPLE_SCHEMA,
            1,
            &original,
        )
        .expect("serialization must succeed");

        let mut tampered = serialized.into_bytes();

        let last = tampered
            .iter()
            .position(|byte| *byte == b'3')
            .expect("test payload must contain 3");

        tampered[last] = b'4';

        let result: Result<Example, SerializationError> =
            deserialize_document(
                EXAMPLE_SCHEMA,
                1,
                &tampered,
            );

        assert!(matches!(
            result,
            Err(SerializationError::FingerprintMismatch { .. })
                | Err(SerializationError::Deserialize { .. })
        ));
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let bytes = b"zamani quantum hardware";

        let first = fingerprint_bytes(bytes);
        let second = fingerprint_bytes(bytes);

        assert_eq!(first, second);
        assert_eq!(first.len(), FINGERPRINT_HEX_LENGTH);
    }

    #[test]
    fn fingerprint_changes_when_content_changes() {
        let first =
            fingerprint_bytes(b"zamani hardware");
        let second =
            fingerprint_bytes(b"zamani hardware!");

        assert_ne!(first, second);
    }

    #[test]
    fn fingerprint_verification_succeeds() {
        let bytes = b"zamani";

        let fingerprint = fingerprint_bytes(bytes);

        assert!(
            verify_fingerprint(bytes, &fingerprint).is_ok()
        );
    }

    #[test]
    fn fingerprint_verification_rejects_wrong_value() {
        let bytes = b"zamani";

        let result = verify_fingerprint(
            bytes,
            "0000000000000000000000000000000000000000000000000000000000000000",
        );

        assert!(matches!(
            result,
            Err(SerializationError::FingerprintMismatch { .. })
        ));
    }

    #[test]
    fn invalid_fingerprint_is_rejected() {
        let result = verify_fingerprint(b"zamani", "not-a-fingerprint");

        assert!(matches!(
            result,
            Err(SerializationError::InvalidFingerprint { .. })
        ));
    }

    #[test]
    fn canonical_json_sorts_object_keys() {
        let mut object = Map::new();

        object.insert(
            "z".to_owned(),
            Value::Number(3.into()),
        );

        object.insert(
            "a".to_owned(),
            Value::Number(1.into()),
        );

        object.insert(
            "m".to_owned(),
            Value::Number(2.into()),
        );

        let value = Value::Object(object);

        let canonical =
            canonicalize_json(&value)
                .expect("canonicalization must succeed");

        assert_eq!(
            canonical,
            r#"{"a":1,"m":2,"z":3}"#
        );
    }

    #[test]
    fn arrays_keep_semantic_order() {
        let value = serde_json::json!([
            3,
            1,
            2
        ]);

        let canonical =
            canonicalize_json(&value)
                .expect("canonicalization must succeed");

        assert_eq!(canonical, "[3,1,2]");
    }

    #[test]
    fn nested_objects_are_canonicalized_recursively() {
        let value = serde_json::json!({
            "outer": {
                "z": 3,
                "a": 1
            }
        });

        let canonical =
            canonicalize_json(&value)
                .expect("canonicalization must succeed");

        assert_eq!(
            canonical,
            r#"{"outer":{"a":1,"z":3}}"#
        );
    }

    #[test]
    fn root_arrays_are_rejected_for_domain_documents() {
        let value = vec![1_u64, 2_u64, 3_u64];

        let result = serialize_document(
            EXAMPLE_SCHEMA,
            1,
            &value,
        );

        assert_eq!(
            result,
            Err(SerializationError::RootMustBeObject)
        );
    }

    #[test]
    fn document_size_limit_is_enforced() {
        let original = Example {
            name: "a".repeat(1024),
            values: vec![1, 2, 3],
        };

        let options = SerializationOptions {
            max_document_bytes: 64,
            max_json_depth: DEFAULT_MAX_JSON_DEPTH,
            include_fingerprint: true,
        };

        let result = serialize_document_with_options(
            EXAMPLE_SCHEMA,
            1,
            &original,
            options,
        );

        assert!(matches!(
            result,
            Err(SerializationError::DocumentTooLarge { .. })
        ));
    }

    #[test]
    fn deserialize_size_limit_is_enforced() {
        let bytes = vec![b' '; 1024];

        let options = SerializationOptions {
            max_document_bytes: 64,
            max_json_depth: DEFAULT_MAX_JSON_DEPTH,
            include_fingerprint: true,
        };

        let result: Result<Example, SerializationError> =
            deserialize_document_with_options(
                EXAMPLE_SCHEMA,
                1,
                &bytes,
                options,
            );

        assert!(matches!(
            result,
            Err(SerializationError::DocumentTooLarge { .. })
        ));
    }

    #[test]
    fn json_depth_limit_is_enforced() {
        let mut value = Value::Null;

        for _ in 0..10 {
            value = Value::Array(vec![value]);
        }

        let result = validate_json_depth(&value, 3);

        assert_eq!(
            result,
            Err(SerializationError::JsonDepthExceeded {
                maximum: 3
            })
        );
    }

    #[test]
    fn fingerprint_is_lowercase_hexadecimal() {
        let fingerprint =
            fingerprint_bytes(b"zamani");

        assert!(
            fingerprint
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
        );

        assert!(
            fingerprint
                .bytes()
                .all(|byte| !byte.is_ascii_uppercase())
        );
    }

    #[test]
    fn envelope_validation_recomputes_fingerprint() {
        let original = Example {
            name: "zamani".to_owned(),
            values: vec![7, 11],
        };

        let serialized = serialize_document(
            EXAMPLE_SCHEMA,
            1,
            &original,
        )
        .expect("serialization must succeed");

        let envelope = inspect_document(
            serialized.as_bytes(),
            SerializationOptions::production(),
        )
        .expect("inspection must succeed");

        let calculated = envelope
            .calculated_fingerprint()
            .expect("fingerprint must calculate");

        assert_eq!(
            envelope.fingerprint.as_deref(),
            Some(calculated.as_str())
        );
    }

    #[test]
    fn schema_id_length_is_bounded() {
        let schema_id = "a".repeat(MAX_SCHEMA_ID_LENGTH + 1);

        let result = validate_schema_id(&schema_id);

        assert!(matches!(
            result,
            Err(SerializationError::SchemaIdTooLong { .. })
        ));
    }

    #[test]
    fn invalid_serialization_options_are_rejected() {
        let options = SerializationOptions {
            max_document_bytes: 0,
            max_json_depth: DEFAULT_MAX_JSON_DEPTH,
            include_fingerprint: true,
        };

        assert!(matches!(
            options.validate(),
            Err(SerializationError::InvalidLimit {
                field: "max_document_bytes",
                ..
            })
        ));
    }

    #[test]
    fn excessive_serialization_options_are_rejected() {
        let options = SerializationOptions {
            max_document_bytes:
                MAX_SERIALIZED_DOCUMENT_BYTES + 1,
            max_json_depth: DEFAULT_MAX_JSON_DEPTH,
            include_fingerprint: true,
        };

        assert!(matches!(
            options.validate(),
            Err(SerializationError::LimitTooLarge {
                field: "max_document_bytes",
                ..
            })
        ));
    }

    #[test]
    fn constant_time_comparison_matches_equal_values() {
        assert!(
            constant_time_equal_ascii(
                b"abcdef",
                b"abcdef"
            )
        );
    }

    #[test]
    fn constant_time_comparison_rejects_different_values() {
        assert!(
            !constant_time_equal_ascii(
                b"abcdef",
                b"abcdeg"
            )
        );
    }

    #[test]
    fn constant_time_comparison_rejects_different_lengths() {
        assert!(
            !constant_time_equal_ascii(
                b"abcdef",
                b"abc"
            )
        );
    }
}