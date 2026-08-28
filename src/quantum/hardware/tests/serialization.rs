//! Zamani Quantum — Hardware Serialization
//!
//! Production-grade, provider-neutral serialization boundary for
//! `quantum::hardware`.
//!
//! # Responsibility
//!
//! This module owns representation concerns only:
//!
//! - deterministic JSON serialization;
//! - versioned serialization envelopes;
//! - schema identification;
//! - schema-version checking;
//! - bounded serialization/deserialization;
//! - JSON nesting protection;
//! - canonical JSON representation;
//! - SHA-256 content fingerprints;
//! - envelope integrity verification;
//! - explicit compatibility checks;
//! - safe handling of untrusted serialized hardware/provider data.
//!
//! # Non-responsibilities
//!
//! This module does NOT:
//!
//! - communicate with quantum hardware;
//! - communicate with providers;
//! - authenticate;
//! - store credentials;
//! - store API keys;
//! - perform encryption;
//! - perform digital signatures;
//! - perform quantum compilation;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - perform benchmarking;
//! - execute jobs;
//! - define backend semantics;
//! - define topology semantics;
//! - define capability semantics;
//! - define provider-specific schemas.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural boundary
//!
//! ```text
//! backend.rs
//! topology.rs
//! calibration.rs
//! capabilities.rs
//! execution.rs
//! job.rs
//! provider.rs
//!       │
//!       │ Serialize / Deserialize
//!       ▼
//! serialization.rs
//!       │
//!       ├── canonical JSON
//!       ├── schema/version envelope
//!       └── content fingerprint
//!       │
//!       ▼
//! persistence / cache / transport / audit
//! ```
//!
//! This module intentionally has no dependency on the semantic hardware
//! modules. That prevents circular dependencies and permits this file to be
//! completed and frozen independently.
//!
//! # Determinism
//!
//! Canonical JSON:
//!
//! - sorts object keys recursively;
//! - preserves array order;
//! - contains no insignificant whitespace;
//! - injects no timestamps;
//! - injects no randomness;
//! - reads no environment variables;
//! - reads no filesystem state;
//! - reads no network state.
//!
//! # Security
//!
//! Serialized provider/device data is untrusted input.
//!
//! The safe APIs enforce:
//!
//! - maximum encoded document size;
//! - maximum JSON nesting depth;
//! - schema identifier validation;
//! - schema version validation;
//! - envelope validation;
//! - exact schema matching;
//! - exact version matching;
//! - fingerprint validation.
//!
//! The input-size and structural preflight checks happen before invoking the
//! JSON parser. This is important because rejecting pathological nesting only
//! after parsing would not provide adequate protection against parser-stack
//! exhaustion.
//!
//! # Fingerprints
//!
//! SHA-256 fingerprints identify the exact canonical payload bytes.
//!
//! They are NOT:
//!
//! - signatures;
//! - authentication;
//! - authorization;
//! - proof of provider identity;
//! - proof of hardware authenticity.
//!
//! Authenticity belongs to a separate cryptographic attestation/signature
//! subsystem.
//!
//! # Compatibility
//!
//! There are two independent versions:
//!
//! 1. Envelope schema version.
//! 2. Semantic document schema version.
//!
//! An unknown envelope schema or version is always rejected.
//! An unknown semantic document schema is always rejected when the caller has
//! supplied an expected schema.
//! A semantic version mismatch is always rejected.
//!
//! Migration is deliberately outside this module.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Existing dependencies
//!
//! This implementation uses dependencies already present in `Cargo.toml`:
//!
//! - `serde`;
//! - `serde_json`;
//! - `sha2`.
//!
//! No Cargo.toml modification is required.
//!
//! # Integration contract
//!
//! Semantic hardware modules implement `Serialize`/`Deserialize` and call:
//!
//! - [`serialize_document`];
//! - [`serialize_document_with_options`];
//! - [`deserialize_document`];
//! - [`deserialize_document_with_options`].
//!
//! The serializer never imports those semantic modules.
//!
//! Consequently, completing or changing `backend.rs`, `topology.rs`,
//! `calibration.rs`, `execution.rs`, `job.rs`, provider adapters, benchmarking,
//! or Danga does not require modifying this file merely for integration.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::fmt;

// =============================================================================
// Stable schema constants
// =============================================================================

/// Stable identifier for the serialization envelope.
pub const SERIALIZATION_SCHEMA_ID: &str =
    "zamani.quantum.hardware.serialization";

/// Version of the serialization envelope.
pub const SERIALIZATION_SCHEMA_VERSION: u16 = 1;

/// Maximum complete encoded document accepted by the production defaults.
pub const MAX_SERIALIZED_DOCUMENT_BYTES: usize = 64 * 1024 * 1024;

/// Maximum semantic schema identifier length in bytes.
pub const MAX_SCHEMA_ID_LENGTH: usize = 512;

/// Maximum wire-format identifier length.
pub const MAX_FORMAT_ID_LENGTH: usize = 64;

/// SHA-256 hexadecimal representation length.
pub const FINGERPRINT_HEX_LENGTH: usize = 64;

/// Maximum semantic schema version accepted by this generic envelope.
pub const MAX_SCHEMA_VERSION: u16 = 10_000;

/// Default maximum JSON nesting depth.
pub const DEFAULT_MAX_JSON_DEPTH: usize = 256;

// =============================================================================
// Serialization format
// =============================================================================

/// Supported serialization formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SerializationFormat {
    /// Canonical JSON.
    Json,
}

impl SerializationFormat {
    /// Stable machine-readable format identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
        }
    }

    /// Version of the supported JSON representation.
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

/// Controls serialization and deserialization resource limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SerializationOptions {
    /// Maximum complete serialized envelope size.
    pub max_document_bytes: usize,

    /// Maximum JSON nesting depth.
    pub max_json_depth: usize,

    /// Whether the serialized envelope contains a SHA-256 fingerprint.
    pub include_fingerprint: bool,
}

impl Default for SerializationOptions {
    fn default() -> Self {
        Self::production()
    }
}

impl SerializationOptions {
    /// Conservative production defaults.
    pub const fn production() -> Self {
        Self {
            max_document_bytes: MAX_SERIALIZED_DOCUMENT_BYTES,
            max_json_depth: DEFAULT_MAX_JSON_DEPTH,
            include_fingerprint: true,
        }
    }

    /// Validate configured resource limits.
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

/// Structured errors emitted by the serialization boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    /// Schema identifier is empty.
    EmptySchemaId,

    /// Schema identifier is too long.
    SchemaIdTooLong {
        length: usize,
        maximum: usize,
    },

    /// Schema identifier contains unsupported characters.
    InvalidSchemaId,

    /// Serialization format is unsupported.
    InvalidFormat,

    /// Schema version is invalid.
    InvalidSchemaVersion {
        version: u16,
    },

    /// Serialized input/output exceeds the configured limit.
    DocumentTooLarge {
        size: usize,
        maximum: usize,
    },

    /// A configured limit is zero.
    InvalidLimit {
        field: &'static str,
        value: usize,
    },

    /// A configured limit exceeds the production ceiling.
    LimitTooLarge {
        field: &'static str,
        value: usize,
        maximum: usize,
    },

    /// Serde serialization failed.
    Serialize {
        message: String,
    },

    /// Serde deserialization failed.
    Deserialize {
        message: String,
    },

    /// The envelope structure is invalid.
    InvalidEnvelope {
        message: String,
    },

    /// Expected and actual schema identifiers differ.
    SchemaMismatch {
        expected: String,
        actual: String,
    },

    /// Expected and actual semantic schema versions differ.
    UnsupportedSchemaVersion {
        schema_id: String,
        expected: u16,
        actual: u16,
    },

    /// Payload fingerprint does not match.
    FingerprintMismatch {
        expected: String,
        actual: String,
    },

    /// Fingerprint representation is malformed.
    InvalidFingerprint {
        fingerprint: String,
    },

    /// JSON nesting exceeds the configured maximum.
    JsonDepthExceeded {
        maximum: usize,
    },

    /// A required envelope root is not a JSON object.
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
                formatter.write_str(
                    "serialization schema identifier cannot be empty",
                )
            }

            Self::SchemaIdTooLong { length, maximum } => {
                write!(
                    formatter,
                    "schema identifier is {length} bytes; maximum is {maximum}"
                )
            }

            Self::InvalidSchemaId => {
                formatter.write_str(
                    "serialization schema identifier contains invalid characters",
                )
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
                write!(
                    formatter,
                    "{field} must be greater than zero, got {value}"
                )
            }

            Self::LimitTooLarge {
                field,
                value,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} is {value}; maximum is {maximum}"
                )
            }

            Self::Serialize { message } => {
                write!(formatter, "serialization failed: {message}")
            }

            Self::Deserialize { message } => {
                write!(formatter, "deserialization failed: {message}")
            }

            Self::InvalidEnvelope { message } => {
                write!(
                    formatter,
                    "invalid serialization envelope: {message}"
                )
            }

            Self::SchemaMismatch { expected, actual } => {
                write!(
                    formatter,
                    "schema mismatch: expected {expected}, received {actual}"
                )
            }

            Self::UnsupportedSchemaVersion {
                schema_id,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "unsupported schema version for {schema_id}: \
                     expected {expected}, received {actual}"
                )
            }

            Self::FingerprintMismatch { expected, actual } => {
                write!(
                    formatter,
                    "fingerprint mismatch: expected {expected}, \
                     calculated {actual}"
                )
            }

            Self::InvalidFingerprint { fingerprint } => {
                write!(
                    formatter,
                    "invalid fingerprint: {fingerprint}"
                )
            }

            Self::JsonDepthExceeded { maximum } => {
                write!(
                    formatter,
                    "JSON nesting exceeds maximum depth {maximum}"
                )
            }

            Self::RootMustBeObject => {
                formatter.write_str(
                    "serialization envelope root must be a JSON object",
                )
            }

            Self::InvalidField { field, message } => {
                write!(
                    formatter,
                    "invalid envelope field {field}: {message}"
                )
            }
        }
    }
}

impl std::error::Error for SerializationError {}

// =============================================================================
// Envelope
// =============================================================================

/// Versioned generic hardware serialization envelope.
///
/// The envelope distinguishes the serialization infrastructure version from
/// the semantic document's own version.
///
/// The fingerprint covers the canonical semantic payload only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentEnvelope {
    /// Envelope schema identifier.
    pub schema_id: String,

    /// Envelope schema version.
    pub schema_version: u16,

    /// Serialization format identifier.
    pub format: String,

    /// Serialization format version.
    pub format_version: u16,

    /// Semantic document schema identifier.
    pub document_schema_id: String,

    /// Semantic document schema version.
    pub document_schema_version: u16,

    /// Semantic payload.
    pub payload: Value,

    /// Optional SHA-256 fingerprint of canonical payload bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

impl DocumentEnvelope {
    /// Construct a validated envelope from a JSON payload.
    pub fn new(
        document_schema_id: &str,
        document_schema_version: u16,
        payload: Value,
        include_fingerprint: bool,
    ) -> Result<Self, SerializationError> {
        validate_schema_id(document_schema_id)?;
        validate_schema_version(document_schema_version)?;

        let fingerprint = if include_fingerprint {
            Some(fingerprint_value(&payload)?)
        } else {
            None
        };

        let envelope = Self {
            schema_id: SERIALIZATION_SCHEMA_ID.to_owned(),
            schema_version: SERIALIZATION_SCHEMA_VERSION,
            format: SerializationFormat::Json.as_str().to_owned(),
            format_version: SerializationFormat::Json.version(),
            document_schema_id: document_schema_id.to_owned(),
            document_schema_version,
            payload,
            fingerprint,
        };

        envelope.validate(
            Some(document_schema_id),
            DEFAULT_MAX_JSON_DEPTH,
        )?;

        Ok(envelope)
    }

    /// Validate envelope structure and compatibility.
    ///
    /// If `expected_document_schema_id` is `None`, only structural envelope
    /// validation is performed.
    pub fn validate(
        &self,
        expected_document_schema_id: Option<&str>,
        max_json_depth: usize,
    ) -> Result<(), SerializationError> {
        if self.schema_id != SERIALIZATION_SCHEMA_ID {
            return Err(SerializationError::SchemaMismatch {
                expected: SERIALIZATION_SCHEMA_ID.to_owned(),
                actual: self.schema_id.clone(),
            });
        }

        if self.schema_version != SERIALIZATION_SCHEMA_VERSION {
            return Err(SerializationError::UnsupportedSchemaVersion {
                schema_id: self.schema_id.clone(),
                expected: SERIALIZATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.format.len() > MAX_FORMAT_ID_LENGTH {
            return Err(SerializationError::InvalidField {
                field: "format",
                message: format!(
                    "format identifier exceeds {MAX_FORMAT_ID_LENGTH} bytes"
                ),
            });
        }

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

        validate_schema_id(&self.document_schema_id)?;
        validate_schema_version(self.document_schema_version)?;

        if let Some(expected) = expected_document_schema_id {
            validate_schema_id(expected)?;

            if expected != self.document_schema_id {
                return Err(SerializationError::SchemaMismatch {
                    expected: expected.to_owned(),
                    actual: self.document_schema_id.clone(),
                });
            }
        }

        validate_json_depth(&self.payload, max_json_depth)?;

        if let Some(fingerprint) = &self.fingerprint {
            validate_fingerprint(fingerprint)?;

            let actual = fingerprint_value(&self.payload)?;

            if !fingerprint.eq_ignore_ascii_case(&actual) {
                return Err(
                    SerializationError::FingerprintMismatch {
                        expected: fingerprint.clone(),
                        actual,
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Serialized document
// =============================================================================

/// Complete serialized document together with useful immutable metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializedDocument {
    bytes: Vec<u8>,
    document_schema_id: String,
    document_schema_version: u16,
    fingerprint: Option<String>,
}

impl SerializedDocument {
    fn new(
        bytes: Vec<u8>,
        document_schema_id: String,
        document_schema_version: u16,
        fingerprint: Option<String>,
    ) -> Self {
        Self {
            bytes,
            document_schema_id,
            document_schema_version,
            fingerprint,
        }
    }

    /// Borrow the complete serialized document.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume the wrapper and return the serialized bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Return the serialized document as UTF-8 text.
    pub fn as_str(&self) -> Result<&str, SerializationError> {
        std::str::from_utf8(&self.bytes).map_err(|error| {
            SerializationError::Deserialize {
                message: format!(
                    "serialized document is not valid UTF-8: {error}"
                ),
            }
        })
    }

    /// Number of serialized bytes.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether no serialized bytes exist.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Semantic document schema identifier.
    pub fn document_schema_id(&self) -> &str {
        &self.document_schema_id
    }

    /// Semantic document schema version.
    pub const fn document_schema_version(&self) -> u16 {
        self.document_schema_version
    }

    /// Optional semantic payload fingerprint.
    pub fn fingerprint(&self) -> Option<&str> {
        self.fingerprint.as_deref()
    }
}

// =============================================================================
// Public serialization API
// =============================================================================

/// Serialize a semantic hardware value using production defaults.
///
/// The result is a complete canonical JSON envelope.
pub fn serialize_document<T: Serialize>(
    document_schema_id: &str,
    document_schema_version: u16,
    value: &T,
) -> Result<Vec<u8>, SerializationError> {
    serialize_document_with_options(
        document_schema_id,
        document_schema_version,
        value,
        SerializationOptions::production(),
    )
}

/// Serialize a semantic hardware value with explicit limits.
pub fn serialize_document_with_options<T: Serialize>(
    document_schema_id: &str,
    document_schema_version: u16,
    value: &T,
    options: SerializationOptions,
) -> Result<Vec<u8>, SerializationError> {
    serialize_document_internal(
        document_schema_id,
        document_schema_version,
        value,
        options,
    )
    .map(SerializedDocument::into_bytes)
}

/// Serialize a value and retain metadata about the resulting document.
pub fn serialize_document_metadata<T: Serialize>(
    document_schema_id: &str,
    document_schema_version: u16,
    value: &T,
) -> Result<SerializedDocument, SerializationError> {
    serialize_document_internal(
        document_schema_id,
        document_schema_version,
        value,
        SerializationOptions::production(),
    )
}

/// Deserialize a semantic hardware value using production defaults.
pub fn deserialize_document<T: DeserializeOwned>(
    expected_document_schema_id: &str,
    expected_document_schema_version: u16,
    bytes: &[u8],
) -> Result<T, SerializationError> {
    deserialize_document_with_options(
        expected_document_schema_id,
        expected_document_schema_version,
        bytes,
        SerializationOptions::production(),
    )
}

/// Deserialize a semantic hardware value with explicit limits.
pub fn deserialize_document_with_options<T: DeserializeOwned>(
    expected_document_schema_id: &str,
    expected_document_schema_version: u16,
    bytes: &[u8],
    options: SerializationOptions,
) -> Result<T, SerializationError> {
    options.validate()?;

    validate_schema_id(expected_document_schema_id)?;
    validate_schema_version(expected_document_schema_version)?;

    if bytes.len() > options.max_document_bytes {
        return Err(SerializationError::DocumentTooLarge {
            size: bytes.len(),
            maximum: options.max_document_bytes,
        });
    }

    /*
     * Perform a structural depth scan before serde_json parses the document.
     *
     * This is defense-in-depth: the JSON parser should never be asked to
     * process arbitrarily deep attacker-controlled nesting just so that we can
     * reject it after parsing.
     */
    preflight_json_depth(bytes, options.max_json_depth)?;

    let value = parse_json(bytes)?;

    let envelope = envelope_from_value(value)?;

    envelope.validate(
        Some(expected_document_schema_id),
        options.max_json_depth,
    )?;

    if envelope.document_schema_version != expected_document_schema_version {
        return Err(
            SerializationError::UnsupportedSchemaVersion {
                schema_id: expected_document_schema_id.to_owned(),
                expected: expected_document_schema_version,
                actual: envelope.document_schema_version,
            },
        );
    }

    serde_json::from_value(envelope.payload).map_err(|error| {
        SerializationError::Deserialize {
            message: error.to_string(),
        }
    })
}

/// Deserialize only the generic envelope.
pub fn deserialize_envelope(
    bytes: &[u8],
    options: SerializationOptions,
) -> Result<DocumentEnvelope, SerializationError> {
    options.validate()?;

    if bytes.len() > options.max_document_bytes {
        return Err(SerializationError::DocumentTooLarge {
            size: bytes.len(),
            maximum: options.max_document_bytes,
        });
    }

    preflight_json_depth(bytes, options.max_json_depth)?;

    let value = parse_json(bytes)?;
    let envelope = envelope_from_value(value)?;

    envelope.validate(None, options.max_json_depth)?;

    Ok(envelope)
}

// =============================================================================
// Canonicalization
// =============================================================================

/// Canonicalize JSON using the production nesting limit.
///
/// Object keys are recursively sorted. Array order is preserved.
pub fn canonicalize_json(
    value: &Value,
) -> Result<Vec<u8>, SerializationError> {
    canonicalize_json_with_depth(
        value,
        DEFAULT_MAX_JSON_DEPTH,
    )
}

/// Canonicalize JSON with an explicit maximum nesting depth.
pub fn canonicalize_json_with_depth(
    value: &Value,
    max_json_depth: usize,
) -> Result<Vec<u8>, SerializationError> {
    if max_json_depth == 0 {
        return Err(SerializationError::InvalidLimit {
            field: "max_json_depth",
            value: max_json_depth,
        });
    }

    validate_json_depth(value, max_json_depth)?;

    let canonical = canonicalize_value(value);

    serde_json::to_vec(&canonical).map_err(|error| {
        SerializationError::Serialize {
            message: error.to_string(),
        }
    })
}

// =============================================================================
// Fingerprints
// =============================================================================

/// Calculate a lowercase hexadecimal SHA-256 fingerprint over bytes.
pub fn fingerprint_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);

    let mut output = String::with_capacity(
        FINGERPRINT_HEX_LENGTH,
    );

    for byte in digest {
        use fmt::Write as _;

        // Writing into a String cannot fail.
        let _ = write!(output, "{byte:02x}");
    }

    output
}

/// Calculate a SHA-256 fingerprint over UTF-8 text.
pub fn fingerprint_str(value: &str) -> String {
    fingerprint_bytes(value.as_bytes())
}

/// Calculate a fingerprint over canonical JSON.
pub fn fingerprint_json(
    value: &Value,
) -> Result<String, SerializationError> {
    let canonical = canonicalize_json(value)?;
    Ok(fingerprint_bytes(&canonical))
}

/// Verify a SHA-256 fingerprint against bytes.
pub fn verify_fingerprint(
    expected_fingerprint: &str,
    bytes: &[u8],
) -> Result<(), SerializationError> {
    validate_fingerprint(expected_fingerprint)?;

    let actual = fingerprint_bytes(bytes);

    if expected_fingerprint.eq_ignore_ascii_case(&actual) {
        Ok(())
    } else {
        Err(SerializationError::FingerprintMismatch {
            expected: expected_fingerprint.to_ascii_lowercase(),
            actual,
        })
    }
}

// =============================================================================
// Internal serialization
// =============================================================================

fn serialize_document_internal<T: Serialize>(
    document_schema_id: &str,
    document_schema_version: u16,
    value: &T,
    options: SerializationOptions,
) -> Result<SerializedDocument, SerializationError> {
    options.validate()?;

    validate_schema_id(document_schema_id)?;
    validate_schema_version(document_schema_version)?;

    let payload = serde_json::to_value(value).map_err(|error| {
        SerializationError::Serialize {
            message: error.to_string(),
        }
    })?;

    validate_json_depth(
        &payload,
        options.max_json_depth,
    )?;

    let canonical_payload =
        canonicalize_json_with_depth(
            &payload,
            options.max_json_depth,
        )?;

    let fingerprint = if options.include_fingerprint {
        Some(fingerprint_bytes(&canonical_payload))
    } else {
        None
    };

    let envelope = DocumentEnvelope {
        schema_id: SERIALIZATION_SCHEMA_ID.to_owned(),
        schema_version: SERIALIZATION_SCHEMA_VERSION,
        format: SerializationFormat::Json.as_str().to_owned(),
        format_version: SerializationFormat::Json.version(),
        document_schema_id: document_schema_id.to_owned(),
        document_schema_version,
        payload: canonicalize_value(&payload),
        fingerprint: fingerprint.clone(),
    };

    envelope.validate(
        Some(document_schema_id),
        options.max_json_depth,
    )?;

    let envelope_value =
        serde_json::to_value(&envelope).map_err(|error| {
            SerializationError::Serialize {
                message: error.to_string(),
            }
        })?;

    let bytes = canonicalize_json_with_depth(
        &envelope_value,
        options.max_json_depth,
    )?;

    if bytes.len() > options.max_document_bytes {
        return Err(SerializationError::DocumentTooLarge {
            size: bytes.len(),
            maximum: options.max_document_bytes,
        });
    }

    Ok(SerializedDocument::new(
        bytes,
        document_schema_id.to_owned(),
        document_schema_version,
        fingerprint,
    ))
}

// =============================================================================
// Internal parsing
// =============================================================================

fn parse_json(bytes: &[u8]) -> Result<Value, SerializationError> {
    serde_json::from_slice(bytes).map_err(|error| {
        SerializationError::Deserialize {
            message: error.to_string(),
        }
    })
}

fn envelope_from_value(
    value: Value,
) -> Result<DocumentEnvelope, SerializationError> {
    if !value.is_object() {
        return Err(SerializationError::RootMustBeObject);
    }

    serde_json::from_value(value).map_err(|error| {
        SerializationError::InvalidEnvelope {
            message: error.to_string(),
        }
    })
}

// =============================================================================
// Validation
// =============================================================================

fn validate_schema_id(
    schema_id: &str,
) -> Result<(), SerializationError> {
    if schema_id.is_empty() {
        return Err(SerializationError::EmptySchemaId);
    }

    if schema_id.len() > MAX_SCHEMA_ID_LENGTH {
        return Err(SerializationError::SchemaIdTooLong {
            length: schema_id.len(),
            maximum: MAX_SCHEMA_ID_LENGTH,
        });
    }

    /*
     * Conservative identifier grammar:
     *
     *   A-Z a-z 0-9 . - _ /
     *
     * This keeps schema identifiers stable across:
     *
     * - JSON;
     * - filesystems;
     * - registries;
     * - URLs;
     * - provider transports;
     * - logs;
     * - configuration.
     */
    if !schema_id.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'.' | b'-' | b'_' | b'/'
            )
    }) {
        return Err(SerializationError::InvalidSchemaId);
    }

    Ok(())
}

fn validate_schema_version(
    version: u16,
) -> Result<(), SerializationError> {
    if version == 0 || version > MAX_SCHEMA_VERSION {
        return Err(
            SerializationError::InvalidSchemaVersion {
                version,
            },
        );
    }

    Ok(())
}

fn validate_fingerprint(
    fingerprint: &str,
) -> Result<(), SerializationError> {
    if fingerprint.len() != FINGERPRINT_HEX_LENGTH
        || !fingerprint
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(
            SerializationError::InvalidFingerprint {
                fingerprint: fingerprint.to_owned(),
            },
        );
    }

    Ok(())
}

// =============================================================================
// JSON depth validation
// =============================================================================

fn validate_json_depth(
    value: &Value,
    maximum: usize,
) -> Result<(), SerializationError> {
    if maximum == 0 {
        return Err(SerializationError::InvalidLimit {
            field: "max_json_depth",
            value: maximum,
        });
    }

    fn visit(
        value: &Value,
        depth: usize,
        maximum: usize,
    ) -> bool {
        if depth > maximum {
            return false;
        }

        match value {
            Value::Object(values) => values
                .values()
                .all(|value| {
                    visit(
                        value,
                        depth + 1,
                        maximum,
                    )
                }),

            Value::Array(values) => values
                .iter()
                .all(|value| {
                    visit(
                        value,
                        depth + 1,
                        maximum,
                    )
                }),

            Value::Null
            | Value::Bool(_)
            | Value::Number(_)
            | Value::String(_) => true,
        }
    }

    if visit(value, 1, maximum) {
        Ok(())
    } else {
        Err(
            SerializationError::JsonDepthExceeded {
                maximum,
            },
        )
    }
}

// =============================================================================
// Pre-parser depth protection
// =============================================================================

/// Perform a conservative JSON structural depth scan before invoking serde.
///
/// The scanner understands JSON strings and escapes so braces/brackets inside
/// strings do not affect the calculated nesting depth.
///
/// This is intentionally stricter than attempting to recover malformed JSON.
/// The actual JSON parser remains authoritative for JSON syntax.
fn preflight_json_depth(
    bytes: &[u8],
    maximum: usize,
) -> Result<(), SerializationError> {
    if maximum == 0 {
        return Err(SerializationError::InvalidLimit {
            field: "max_json_depth",
            value: maximum,
        });
    }

    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for &byte in bytes {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }

            if byte == b'\\' {
                escaped = true;
                continue;
            }

            if byte == b'"' {
                in_string = false;
            }

            continue;
        }

        match byte {
            b'"' => {
                in_string = true;
            }

            b'{' | b'[' => {
                depth = depth.saturating_add(1);

                if depth > maximum {
                    return Err(
                        SerializationError::JsonDepthExceeded {
                            maximum,
                        },
                    );
                }
            }

            b'}' | b']' => {
                /*
                 * Do not reject mismatched closing delimiters here.
                 * serde_json is responsible for syntax validation.
                 *
                 * We only use this pass for pre-parser depth protection.
                 */
                depth = depth.saturating_sub(1);
            }

            _ => {}
        }
    }

    Ok(())
}

// =============================================================================
// Canonical JSON
// =============================================================================

fn canonicalize_value(value: &Value) -> Value {
    match value {
        Value::Object(object) => {
            let mut entries: Vec<(&String, &Value)> =
                object.iter().collect();

            entries.sort_by(|left, right| {
                left.0.cmp(right.0)
            });

            let mut result = Map::new();

            for (key, child) in entries {
                result.insert(
                    key.clone(),
                    canonicalize_value(child),
                );
            }

            Value::Object(result)
        }

        Value::Array(values) => {
            Value::Array(
                values
                    .iter()
                    .map(canonicalize_value)
                    .collect(),
            )
        }

        Value::Null => Value::Null,

        Value::Bool(value) => Value::Bool(*value),

        Value::Number(value) => {
            Value::Number(value.clone())
        }

        Value::String(value) => {
            Value::String(value.clone())
        }
    }
}

fn fingerprint_value(
    value: &Value,
) -> Result<String, SerializationError> {
    let canonical = canonicalize_json(value)?;
    Ok(fingerprint_bytes(&canonical))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct Example {
        value: String,
        number: u64,
    }

    fn example() -> Example {
        Example {
            value: "zamani".to_owned(),
            number: 42,
        }
    }

    #[test]
    fn production_options_are_valid() {
        SerializationOptions::production()
            .validate()
            .expect("production defaults must be valid");
    }

    #[test]
    fn round_trip_is_lossless() {
        let value = example();

        let encoded = serialize_document(
            "zamani.quantum.hardware.test",
            1,
            &value,
        )
        .expect("serialization should succeed");

        let decoded: Example =
            deserialize_document(
                "zamani.quantum.hardware.test",
                1,
                &encoded,
            )
            .expect("deserialization should succeed");

        assert_eq!(value, decoded);
    }

    #[test]
    fn canonicalization_sorts_keys_recursively() {
        let value = json!({
            "z": 1,
            "a": {
                "z": true,
                "a": false
            },
            "array": [
                {
                    "z": 1,
                    "a": 2
                }
            ]
        });

        let bytes = canonicalize_json(&value)
            .expect("canonicalization should succeed");

        let text = String::from_utf8(bytes)
            .expect("canonical JSON must be UTF-8");

        assert_eq!(
            text,
            r#"{"a":{"a":false,"z":true},"array":[{"a":2,"z":1}],"z":1}"#
        );
    }

    #[test]
    fn arrays_preserve_semantic_order() {
        let value = json!([3, 1, 2]);

        let bytes = canonicalize_json(&value)
            .expect("canonicalization should succeed");

        assert_eq!(
            bytes,
            br#"[3,1,2]"#
        );
    }

    #[test]
    fn fingerprint_is_sha256() {
        assert_eq!(
            fingerprint_str("zamani"),
            "0cd38465086ac2f07f9425c5695dca3f2f8d30f43a68f067c6aea012261992ca"
        );
    }

    #[test]
    fn matching_fingerprint_is_accepted() {
        let bytes = b"zamani";
        let fingerprint = fingerprint_bytes(bytes);

        verify_fingerprint(
            &fingerprint,
            bytes,
        )
        .expect("fingerprint must verify");
    }

    #[test]
    fn tampered_fingerprint_is_rejected() {
        let fingerprint =
            fingerprint_bytes(b"zamani");

        let error = verify_fingerprint(
            &fingerprint,
            b"tampered",
        )
        .expect_err(
            "tampered content must be rejected",
        );

        assert!(matches!(
            error,
            SerializationError::FingerprintMismatch { .. }
        ));
    }

    #[test]
    fn malformed_fingerprint_is_rejected() {
        let error = verify_fingerprint(
            "not-a-fingerprint",
            b"zamani",
        )
        .expect_err(
            "invalid fingerprint must be rejected",
        );

        assert!(matches!(
            error,
            SerializationError::InvalidFingerprint { .. }
        ));
    }

    #[test]
    fn wrong_schema_is_rejected() {
        let encoded = serialize_document(
            "zamani.quantum.hardware.test",
            1,
            &example(),
        )
        .expect("serialization should succeed");

        let error =
            deserialize_document::<Example>(
                "zamani.quantum.hardware.other",
                1,
                &encoded,
            )
            .expect_err(
                "wrong schema must fail",
            );

        assert!(matches!(
            error,
            SerializationError::SchemaMismatch { .. }
        ));
    }

    #[test]
    fn wrong_semantic_version_is_rejected() {
        let encoded = serialize_document(
            "zamani.quantum.hardware.test",
            1,
            &example(),
        )
        .expect("serialization should succeed");

        let error =
            deserialize_document::<Example>(
                "zamani.quantum.hardware.test",
                2,
                &encoded,
            )
            .expect_err(
                "wrong semantic version must fail",
            );

        assert!(matches!(
            error,
            SerializationError::UnsupportedSchemaVersion { .. }
        ));
    }

    #[test]
    fn invalid_schema_identifier_is_rejected() {
        let error = serialize_document(
            "zamani quantum hardware",
            1,
            &example(),
        )
        .expect_err(
            "invalid schema identifier must fail",
        );

        assert!(matches!(
            error,
            SerializationError::InvalidSchemaId
        ));
    }

    #[test]
    fn empty_schema_identifier_is_rejected() {
        let error = serialize_document(
            "",
            1,
            &example(),
        )
        .expect_err(
            "empty schema identifier must fail",
        );

        assert!(matches!(
            error,
            SerializationError::EmptySchemaId
        ));
    }

    #[test]
    fn zero_schema_version_is_rejected() {
        let error = serialize_document(
            "zamani.quantum.hardware.test",
            0,
            &example(),
        )
        .expect_err(
            "zero schema version must fail",
        );

        assert!(matches!(
            error,
            SerializationError::InvalidSchemaVersion { .. }
        ));
    }

    #[test]
    fn oversized_input_is_rejected_before_parsing() {
        let options = SerializationOptions {
            max_document_bytes: 4,
            max_json_depth: DEFAULT_MAX_JSON_DEPTH,
            include_fingerprint: true,
        };

        let error =
            deserialize_document_with_options::<Example>(
                "zamani.quantum.hardware.test",
                1,
                br#"{"schema_id":"too-large"}"#,
                options,
            )
            .expect_err(
                "oversized input must fail before parsing",
            );

        assert!(matches!(
            error,
            SerializationError::DocumentTooLarge { .. }
        ));
    }

    #[test]
    fn malformed_json_is_rejected() {
        let error =
            deserialize_envelope(
                br#"{"schema_id":"broken""#,
                SerializationOptions::production(),
            )
            .expect_err(
                "malformed JSON must fail",
            );

        assert!(matches!(
            error,
            SerializationError::Deserialize { .. }
        ));
    }

    #[test]
    fn non_object_envelope_is_rejected() {
        let error =
            deserialize_envelope(
                br#"[]"#,
                SerializationOptions::production(),
            )
            .expect_err(
                "array cannot be an envelope",
            );

        assert!(matches!(
            error,
            SerializationError::RootMustBeObject
        ));
    }

    #[test]
    fn malformed_envelope_is_rejected() {
        let value = json!({
            "not_an_envelope": true
        });

        let bytes =
            serde_json::to_vec(&value)
                .expect("test JSON must serialize");

        let error =
            deserialize_envelope(
                &bytes,
                SerializationOptions::production(),
            )
            .expect_err(
                "malformed envelope must fail",
            );

        assert!(matches!(
            error,
            SerializationError::InvalidEnvelope { .. }
        ));
    }

    #[test]
    fn unknown_envelope_schema_is_rejected() {
        let value = json!({
            "schema_id": "other.serialization",
            "schema_version": 1,
            "format": "json",
            "format_version": 1,
            "document_schema_id":
                "zamani.quantum.hardware.test",
            "document_schema_version": 1,
            "payload": {
                "value": "x",
                "number": 1
            }
        });

        let bytes =
            serde_json::to_vec(&value)
                .expect("test JSON must serialize");

        let error =
            deserialize_envelope(
                &bytes,
                SerializationOptions::production(),
            )
            .expect_err(
                "unknown envelope schema must fail",
            );

        assert!(matches!(
            error,
            SerializationError::SchemaMismatch { .. }
        ));
    }

    #[test]
    fn invalid_fingerprint_is_rejected() {
        let value = json!({
            "schema_id":
                SERIALIZATION_SCHEMA_ID,
            "schema_version":
                SERIALIZATION_SCHEMA_VERSION,
            "format": "json",
            "format_version": 1,
            "document_schema_id":
                "zamani.quantum.hardware.test",
            "document_schema_version": 1,
            "payload": {
                "value": "x",
                "number": 1
            },
            "fingerprint": "invalid"
        });

        let bytes =
            serde_json::to_vec(&value)
                .expect("test JSON must serialize");

        let error =
            deserialize_envelope(
                &bytes,
                SerializationOptions::production(),
            )
            .expect_err(
                "invalid fingerprint must fail",
            );

        assert!(matches!(
            error,
            SerializationError::InvalidFingerprint { .. }
        ));
    }

    #[test]
    fn mismatched_fingerprint_is_rejected() {
        let value = json!({
            "schema_id":
                SERIALIZATION_SCHEMA_ID,
            "schema_version":
                SERIALIZATION_SCHEMA_VERSION,
            "format": "json",
            "format_version": 1,
            "document_schema_id":
                "zamani.quantum.hardware.test",
            "document_schema_version": 1,
            "payload": {
                "value": "x",
                "number": 1
            },
            "fingerprint":
                "0000000000000000000000000000000000000000000000000000000000000000"
        });

        let bytes =
            serde_json::to_vec(&value)
                .expect("test JSON must serialize");

        let error =
            deserialize_envelope(
                &bytes,
                SerializationOptions::production(),
            )
            .expect_err(
                "incorrect fingerprint must fail",
            );

        assert!(matches!(
            error,
            SerializationError::FingerprintMismatch { .. }
        ));
    }

    #[test]
    fn depth_limit_is_enforced_during_canonicalization() {
        let value = json!({
            "a": {
                "b": {
                    "c": true
                }
            }
        });

        let error =
            canonicalize_json_with_depth(
                &value,
                2,
            )
            .expect_err(
                "depth three must not fit depth two",
            );

        assert!(matches!(
            error,
            SerializationError::JsonDepthExceeded { .. }
        ));
    }

    #[test]
    fn depth_limit_is_enforced_before_parsing() {
        let bytes =
            br#"{"a":{"b":{"c":{"d":true}}}}"#;

        let error =
            deserialize_envelope(
                bytes,
                SerializationOptions {
                    max_document_bytes:
                        MAX_SERIALIZED_DOCUMENT_BYTES,
                    max_json_depth: 2,
                    include_fingerprint: true,
                },
            )
            .expect_err(
                "excessive input nesting must fail",
            );

        assert!(matches!(
            error,
            SerializationError::JsonDepthExceeded { .. }
        ));
    }

    #[test]
    fn braces_inside_strings_do_not_increase_depth() {
        let bytes =
            br#"{"text":"{{{{[[[[}}}}]]]]"}"#;

        let result =
            preflight_json_depth(
                bytes,
                2,
            );

        assert!(
            result.is_ok(),
            "braces inside strings are not structural depth"
        );
    }

    #[test]
    fn serialized_metadata_is_available() {
        let document =
            serialize_document_metadata(
                "zamani.quantum.hardware.test",
                1,
                &example(),
            )
            .expect(
                "serialization should succeed",
            );

        assert_eq!(
            document.document_schema_id(),
            "zamani.quantum.hardware.test"
        );

        assert_eq!(
            document.document_schema_version(),
            1
        );

        assert!(
            document.fingerprint().is_some()
        );

        assert!(
            !document.is_empty()
        );

        assert_eq!(
            document.len(),
            document.as_bytes().len()
        );

        document
            .as_str()
            .expect(
                "serialized JSON must be UTF-8"
            );
    }

    #[test]
    fn fingerprint_is_payload_fingerprint_not_envelope_fingerprint() {
        let value = example();

        let payload =
            serde_json::to_value(&value)
                .expect(
                    "example must serialize"
                );

        let expected =
            fingerprint_json(&payload)
                .expect(
                    "payload fingerprint must succeed"
                );

        let document =
            serialize_document_metadata(
                "zamani.quantum.hardware.test",
                1,
                &value,
            )
            .expect(
                "serialization should succeed"
            );

        assert_eq!(
            document.fingerprint(),
            Some(expected.as_str())
        );
    }

    #[test]
    fn serialization_is_deterministic() {
        let value = example();

        let first =
            serialize_document(
                "zamani.quantum.hardware.test",
                1,
                &value,
            )
            .expect(
                "first serialization should succeed"
            );

        let second =
            serialize_document(
                "zamani.quantum.hardware.test",
                1,
                &value,
            )
            .expect(
                "second serialization should succeed"
            );

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn no_fingerprint_option_is_supported() {
        let options = SerializationOptions {
            max_document_bytes:
                MAX_SERIALIZED_DOCUMENT_BYTES,
            max_json_depth:
                DEFAULT_MAX_JSON_DEPTH,
            include_fingerprint: false,
        };

        let bytes =
            serialize_document_with_options(
                "zamani.quantum.hardware.test",
                1,
                &example(),
                options,
            )
            .expect(
                "serialization without fingerprint should succeed"
            );

        let envelope =
            deserialize_envelope(
                &bytes,
                options,
            )
            .expect(
                "envelope should remain valid"
            );

        assert!(
            envelope.fingerprint.is_none()
        );
    }

    #[test]
    fn production_limit_cannot_be_raised_by_options() {
        let options = SerializationOptions {
            max_document_bytes:
                MAX_SERIALIZED_DOCUMENT_BYTES + 1,
            max_json_depth:
                DEFAULT_MAX_JSON_DEPTH,
            include_fingerprint: true,
        };

        let error =
            options
                .validate()
                .expect_err(
                    "production maximum must be enforced"
                );

        assert!(matches!(
            error,
            SerializationError::LimitTooLarge { .. }
        ));
    }
}