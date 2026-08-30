//! Zamani Quantum Optimization — Configuration Serialization
//!
//! Production serialization boundary for
//! `crate::quantum::optimization::config::OptimizationConfig`.
//!
//! # Architectural role
//!
//! This module serializes and deserializes the optimizer configuration
//! contract. It does not implement optimization algorithms, target
//! resolution, pass planning, circuit transformation, routing, scheduling,
//! hardware execution, or quantum semantics.
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::optimization::config
//!              │
//!              ▼
//! optimization::serialization::config
//!              │
//!       ┌──────┴──────┐
//!       ▼             ▼
//!     JSON           TOML
//!       │             │
//!       └──────┬──────┘
//!              ▼
//!      OptimizationConfig
//! ```
//!
//! # Canonical representation
//!
//! `OptimizationConfig` remains the authoritative in-memory representation.
//!
//! This module deliberately does not create a second optimizer configuration
//! type containing duplicated optimization fields.
//!
//! The serialized document is only an envelope around the canonical
//! configuration:
//!
//! ```text
//! OptimizationConfig
//!        │
//!        ▼
//! OptimizationConfigDocument
//!        │
//!   ┌────┴────┐
//!   ▼         ▼
//! JSON       TOML
//! ```
//!
//! The envelope provides:
//!
//! - stable schema identification;
//! - explicit schema versioning;
//! - forward-compatibility checks;
//! - deterministic canonical serialization;
//! - a stable location for future serialization metadata.
//!
//! # Why an envelope is required
//!
//! Serializing `OptimizationConfig` directly would make the serialized format
//! implicitly depend on the Rust type layout and would provide no reliable way
//! to distinguish:
//!
//! - a valid current configuration;
//! - an obsolete configuration;
//! - a configuration from a future compiler;
//! - an unrelated JSON/TOML document.
//!
//! The envelope therefore contains:
//!
//! ```text
//! schema
//! schema_version
//! config
//! ```
//!
//! # Schema compatibility
//!
//! `CURRENT_SCHEMA_VERSION` identifies the serialization schema, not the Rust
//! compiler version and not the optimizer implementation version.
//!
//! Readers accept the current schema version only.
//!
//! Older schemas are not silently interpreted as current schemas because
//! doing so can change optimizer semantics without the caller knowing.
//!
//! A future schema version is rejected explicitly rather than partially
//! deserialized.
//!
//! Migration of an older schema belongs in an explicit future migration layer.
//! It must never be hidden inside ordinary deserialization.
//!
//! # Determinism
//!
//! Canonical JSON is generated without timestamps, process identifiers,
//! memory addresses, random values, environment information, or filesystem
//! information.
//!
//! The same `OptimizationConfig` therefore produces the same canonical JSON
//! representation for the same dependency versions and schema.
//!
//! The canonical JSON representation is suitable for:
//!
//! - configuration fingerprints;
//! - optimizer provenance;
//! - reproducible compilation;
//! - cache keys;
//! - regression tests;
//! - benchmark metadata;
//! - compiler diagnostics.
//!
//! # TOML
//!
//! TOML is provided as a human-editable configuration interchange format.
//!
//! The configuration is first represented as `serde_json::Value` and then
//! serialized as TOML. This avoids coupling the TOML representation directly
//! to Rust-specific enum layout and also permits the configuration to contain
//! structured enum representations that are inconvenient for direct TOML
//! serialization.
//!
//! The JSON representation remains the canonical machine representation.
//!
//! # Security
//!
//! This module:
//!
//! - performs no filesystem I/O;
//! - performs no network I/O;
//! - executes no external programs;
//! - performs no hardware access;
//! - performs no QPU access;
//! - does not evaluate arbitrary code;
//! - does not allocate unbounded recursive structures intentionally;
//! - does not use `unsafe`;
//! - does not use global mutable state.
//!
//! Callers that receive configuration from untrusted sources should still
//! enforce their own input-size limits before passing arbitrarily large
//! documents to a parser.
//!
//! # Resource scalability
//!
//! Zamani's optimizer is intended to scale from tiny circuits to circuits
//! limited only by available resources.
//!
//! Serialization must therefore introduce no artificial quantum-circuit
//! limit. This module serializes configuration, not circuits, and does not
//! impose a fixed maximum on:
//!
//! - pass lists;
//! - pass identifiers;
//! - configuration metadata;
//! - target identifiers;
//! - rewrite budgets;
//! - optimizer limits.
//!
//! Resource limits governing optimization itself remain owned by
//! `optimization::limits` and the canonical Quantum IR limit system.
//!
//! # Quantum IR boundary
//!
//! Configuration serialization normally does not need to reference a
//! `QubitId`, because optimizer configuration is not a quantum circuit.
//!
//! This is intentional.
//!
//! `quantum::ir::qubit::QubitId` belongs to canonical circuit/IR data
//! structures. It must not be copied into configuration merely to make this
//! serialization module appear quantum-aware.
//!
//! If a future configuration field genuinely contains a qubit identity, that
//! field must use the canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! rather than defining another qubit identifier here.
//!
//! # Integration contract
//!
//! This file is intentionally written so that later optimization files do not
//! need to modify it merely because new optimization passes are introduced.
//!
//! `config.rs` owns the configuration model.
//!
//! This file owns its serialization boundary.
//!
//! `profile.rs` resolves profiles.
//!
//! `targets/target.rs` resolves concrete targets.
//!
//! `pipeline.rs` consumes the configuration.
//!
//! `planner.rs` consumes policy and target information.
//!
//! `context.rs` stores the configuration for an optimization run.
//!
//! `provenance.rs` may record the canonical JSON and fingerprint produced here.
//!
//! `serialization/report.rs` can serialize optimization reports separately.
//!
//! `serialization/provenance.rs` can serialize provenance separately.
//!
//! No optimizer pass should import this module merely to read configuration.
//! Passes should consume the normal `OptimizationConfig` API.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//! No `unsafe` code is used.
//!
//! # External dependencies
//!
//! This module intentionally uses dependencies already present in Zamani:
//!
//! - `serde`;
//! - `serde_json`;
//! - `toml`;
//! - `thiserror`;
//! - `sha2`.
//!
//! No Cargo.toml modification is required for this file.
//!
//! # Public API
//!
//! The primary API is:
//!
//! ```text
//! serialize_json
//! serialize_json_pretty
//! deserialize_json
//! serialize_toml
//! deserialize_toml
//! canonical_json
//! fingerprint
//! ```
//!
//! `OptimizationConfigDocument` is public so higher-level serialization
//! modules can inspect the stable envelope without creating another
//! representation.
//!
//! =============================================================================

use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::quantum::optimization::config::OptimizationConfig;

// =============================================================================
// Schema constants
// =============================================================================

/// Stable schema identifier for serialized optimizer configurations.
///
/// This is deliberately independent of the Rust module path so that moving
/// implementation files does not silently change the persisted format.
pub const CONFIG_SCHEMA: &str = "zamani.quantum.optimization.config";

/// Current serialized configuration schema version.
///
/// Increment this value only when the serialized representation itself changes
/// in a way that requires an explicit compatibility decision.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Serialization format version.
///
/// This identifies the envelope contract rather than the optimizer algorithm
/// version.
pub const FORMAT_VERSION: u32 = 1;

// =============================================================================
// Serialization format
// =============================================================================

/// Supported configuration serialization formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigSerializationFormat {
    /// Canonical compact JSON.
    Json,

    /// Human-readable JSON.
    JsonPretty,

    /// Human-editable TOML.
    Toml,
}

impl ConfigSerializationFormat {
    /// Returns the stable format identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::JsonPretty => "json_pretty",
            Self::Toml => "toml",
        }
    }
}

// =============================================================================
// Serialization envelope
// =============================================================================

/// Stable serialized configuration envelope.
///
/// The optimizer configuration itself remains the canonical
/// `OptimizationConfig`. `Value` is used here as a format-neutral Serde
/// representation so that JSON and TOML can share the same schema boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizationConfigDocument {
    /// Stable schema identifier.
    pub schema: String,

    /// Serialization schema version.
    pub schema_version: u32,

    /// Serialized canonical optimizer configuration.
    pub config: Value,
}

impl OptimizationConfigDocument {
    /// Creates a document from a canonical optimizer configuration.
    ///
    /// Serialization to `serde_json::Value` is performed before the envelope
    /// is constructed so that the same logical representation is used by both
    /// JSON and TOML.
    pub fn from_config(
        config: &OptimizationConfig,
    ) -> Result<Self, ConfigSerializationError> {
        let value = serde_json::to_value(config)
            .map_err(ConfigSerializationError::Json)?;

        Ok(Self {
            schema: CONFIG_SCHEMA.to_owned(),
            schema_version: CURRENT_SCHEMA_VERSION,
            config: value,
        })
    }

    /// Converts the document back into the canonical optimizer configuration.
    ///
    /// The schema is checked before the configuration is deserialized.
    pub fn into_config(self) -> Result<OptimizationConfig, ConfigSerializationError> {
        validate_document_header(&self)?;

        if !self.config.is_object() {
            return Err(ConfigSerializationError::InvalidConfigValue {
                message: "the `config` field must contain a JSON object".to_owned(),
            });
        }

        serde_json::from_value(self.config)
            .map_err(ConfigSerializationError::Json)
    }

    /// Returns the schema identifier.
    pub fn schema(&self) -> &str {
        &self.schema
    }

    /// Returns the serialized schema version.
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns a reference to the format-neutral configuration value.
    pub fn config_value(&self) -> &Value {
        &self.config
    }

    /// Validates the document envelope without constructing the optimizer
    /// configuration.
    ///
    /// This is useful when callers want to reject incompatible documents
    /// before doing the potentially more expensive configuration conversion.
    pub fn validate(&self) -> Result<(), ConfigSerializationError> {
        validate_document_header(self)?;

        if !self.config.is_object() {
            return Err(ConfigSerializationError::InvalidConfigValue {
                message: "the `config` field must contain a JSON object".to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by optimization configuration serialization.
#[derive(Debug, Error)]
pub enum ConfigSerializationError {
    /// The supplied serialized document belongs to a different schema.
    #[error(
        "unsupported optimization configuration schema `{actual}`; expected `{expected}`"
    )]
    UnsupportedSchema {
        /// Actual schema identifier.
        actual: String,

        /// Expected schema identifier.
        expected: &'static str,
    },

    /// The serialized document uses a schema version that this compiler does
    /// not understand.
    #[error(
        "unsupported optimization configuration schema version {actual}; supported version is {supported}"
    )]
    UnsupportedSchemaVersion {
        /// Actual serialized schema version.
        actual: u32,

        /// Current supported schema version.
        supported: u32,
    },

    /// The serialized configuration value has the wrong structural type.
    #[error("invalid optimization configuration value: {message}")]
    InvalidConfigValue {
        /// Explanation of the structural problem.
        message: String,
    },

    /// JSON serialization or deserialization failed.
    #[error("JSON serialization error: {0}")]
    Json(#[source] serde_json::Error),

    /// TOML serialization or deserialization failed.
    #[error("TOML serialization error: {0}")]
    Toml(#[source] toml::ser::Error),

    /// TOML deserialization failed.
    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[source] toml::de::Error),
}

// =============================================================================
// Header validation
// =============================================================================

/// Validates the stable document header.
fn validate_document_header(
    document: &OptimizationConfigDocument,
) -> Result<(), ConfigSerializationError> {
    if document.schema != CONFIG_SCHEMA {
        return Err(ConfigSerializationError::UnsupportedSchema {
            actual: document.schema.clone(),
            expected: CONFIG_SCHEMA,
        });
    }

    if document.schema_version != CURRENT_SCHEMA_VERSION {
        return Err(ConfigSerializationError::UnsupportedSchemaVersion {
            actual: document.schema_version,
            supported: CURRENT_SCHEMA_VERSION,
        });
    }

    Ok(())
}

// =============================================================================
// Document construction
// =============================================================================

/// Converts an optimizer configuration into its stable serialization
/// document.
///
/// This is the common construction path used by all supported formats.
pub fn document_from_config(
    config: &OptimizationConfig,
) -> Result<OptimizationConfigDocument, ConfigSerializationError> {
    OptimizationConfigDocument::from_config(config)
}

/// Converts a validated document into the canonical optimizer configuration.
pub fn config_from_document(
    document: OptimizationConfigDocument,
) -> Result<OptimizationConfig, ConfigSerializationError> {
    document.into_config()
}

// =============================================================================
// JSON serialization
// =============================================================================

/// Serializes an optimizer configuration as canonical compact JSON.
///
/// This is the preferred representation for:
///
/// - compiler caches;
/// - provenance;
/// - fingerprints;
/// - machine-to-machine transport;
/// - reproducible build metadata.
///
/// The output contains no non-deterministic metadata.
pub fn serialize_json(
    config: &OptimizationConfig,
) -> Result<String, ConfigSerializationError> {
    let document = document_from_config(config)?;

    serde_json::to_string(&document)
        .map_err(ConfigSerializationError::Json)
}

/// Serializes an optimizer configuration as human-readable JSON.
///
/// Pretty JSON is intended for:
///
/// - source control;
/// - diagnostics;
/// - configuration inspection;
/// - documentation;
/// - debugging.
pub fn serialize_json_pretty(
    config: &OptimizationConfig,
) -> Result<String, ConfigSerializationError> {
    let document = document_from_config(config)?;

    serde_json::to_string_pretty(&document)
        .map_err(ConfigSerializationError::Json)
}

/// Deserializes an optimizer configuration from JSON.
///
/// The schema envelope is validated before the resulting
/// `OptimizationConfig` is returned.
pub fn deserialize_json(
    input: &str,
) -> Result<OptimizationConfig, ConfigSerializationError> {
    let document: OptimizationConfigDocument =
        serde_json::from_str(input).map_err(ConfigSerializationError::Json)?;

    config_from_document(document)
}

// =============================================================================
// TOML serialization
// =============================================================================

/// Serializes an optimizer configuration as TOML.
///
/// TOML is intended primarily for human-authored Zamani compiler
/// configuration.
///
/// The canonical semantic representation remains the JSON-compatible
/// `OptimizationConfigDocument`.
///
/// Converting through `serde_json::Value` avoids direct coupling between the
/// TOML representation and Rust-specific Serde enum layout.
pub fn serialize_toml(
    config: &OptimizationConfig,
) -> Result<String, ConfigSerializationError> {
    let document = document_from_config(config)?;

    toml::to_string_pretty(&document)
        .map_err(ConfigSerializationError::Toml)
}

/// Deserializes an optimizer configuration from TOML.
///
/// The schema envelope is validated before the configuration is returned.
pub fn deserialize_toml(
    input: &str,
) -> Result<OptimizationConfig, ConfigSerializationError> {
    let document: OptimizationConfigDocument =
        toml::from_str(input).map_err(ConfigSerializationError::TomlDeserialize)?;

    config_from_document(document)
}

// =============================================================================
// Canonical representation
// =============================================================================

/// Returns the canonical compact JSON representation of an optimizer
/// configuration.
///
/// This is deliberately separate from `serialize_json` to make its purpose
/// explicit at call sites that use the representation for identity,
/// reproducibility, or hashing.
///
/// The function currently uses the same stable serialization path as
/// `serialize_json`.
pub fn canonical_json(
    config: &OptimizationConfig,
) -> Result<String, ConfigSerializationError> {
    serialize_json(config)
}

// =============================================================================
// Fingerprinting
// =============================================================================

/// Computes the SHA-256 fingerprint of the canonical serialized configuration.
///
/// The fingerprint covers:
///
/// - schema identifier;
/// - schema version;
/// - complete serialized optimizer configuration.
///
/// It does not include:
///
/// - timestamps;
/// - compiler process identifiers;
/// - machine names;
/// - filesystem paths;
/// - random values;
/// - environment variables.
///
/// This makes the fingerprint appropriate for deterministic provenance and
/// cache identity.
pub fn fingerprint(
    config: &OptimizationConfig,
) -> Result<[u8; 32], ConfigSerializationError> {
    let canonical = canonical_json(config)?;

    let digest = Sha256::digest(canonical.as_bytes());

    let mut result = [0u8; 32];
    result.copy_from_slice(&digest);

    Ok(result)
}

/// Computes the lowercase hexadecimal SHA-256 fingerprint.
///
/// This representation is convenient for:
///
/// - JSON metadata;
/// - logs;
/// - diagnostics;
/// - cache keys;
/// - command-line output.
pub fn fingerprint_hex(
    config: &OptimizationConfig,
) -> Result<String, ConfigSerializationError> {
    let digest = fingerprint(config)?;

    let mut output = String::with_capacity(digest.len() * 2);

    for byte in digest {
        const HEX: &[u8; 16] = b"0123456789abcdef";

        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }

    Ok(output)
}

// =============================================================================
// Format dispatch
// =============================================================================

/// Serializes an optimizer configuration using the requested format.
///
/// This is the single high-level format-dispatch API intended for callers
/// that determine the output format dynamically.
pub fn serialize(
    config: &OptimizationConfig,
    format: ConfigSerializationFormat,
) -> Result<String, ConfigSerializationError> {
    match format {
        ConfigSerializationFormat::Json => serialize_json(config),
        ConfigSerializationFormat::JsonPretty => serialize_json_pretty(config),
        ConfigSerializationFormat::Toml => serialize_toml(config),
    }
}

/// Deserializes an optimizer configuration using the requested format.
///
/// The parser is selected explicitly by the caller; this function never
/// guesses the format from the input.
pub fn deserialize(
    input: &str,
    format: ConfigSerializationFormat,
) -> Result<OptimizationConfig, ConfigSerializationError> {
    match format {
        ConfigSerializationFormat::Json
        | ConfigSerializationFormat::JsonPretty => deserialize_json(input),
        ConfigSerializationFormat::Toml => deserialize_toml(input),
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Parses and validates a JSON configuration document without returning the
/// configuration.
///
/// This is useful when a compiler needs a cheap compatibility check before
/// passing the configuration to another subsystem.
pub fn validate_json(input: &str) -> Result<(), ConfigSerializationError> {
    let document: OptimizationConfigDocument =
        serde_json::from_str(input).map_err(ConfigSerializationError::Json)?;

    document.validate()
}

/// Parses and validates a TOML configuration document without returning the
/// configuration.
pub fn validate_toml(input: &str) -> Result<(), ConfigSerializationError> {
    let document: OptimizationConfigDocument =
        toml::from_str(input).map_err(ConfigSerializationError::TomlDeserialize)?;

    document.validate()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> OptimizationConfig {
        OptimizationConfig::default()
    }

    #[test]
    fn document_uses_current_schema() {
        let config = default_config();
        let document = document_from_config(&config).expect("document creation must succeed");

        assert_eq!(document.schema(), CONFIG_SCHEMA);
        assert_eq!(
            document.schema_version(),
            CURRENT_SCHEMA_VERSION
        );
        assert!(document.config_value().is_object());
    }

    #[test]
    fn json_round_trip_preserves_configuration() {
        let config = default_config();

        let serialized =
            serialize_json(&config).expect("JSON serialization must succeed");

        let restored =
            deserialize_json(&serialized).expect("JSON deserialization must succeed");

        assert_eq!(config, restored);
    }

    #[test]
    fn pretty_json_round_trip_preserves_configuration() {
        let config = default_config();

        let serialized =
            serialize_json_pretty(&config).expect("pretty JSON serialization must succeed");

        let restored =
            deserialize_json(&serialized).expect("pretty JSON must deserialize");

        assert_eq!(config, restored);
    }

    #[test]
    fn canonical_json_is_stable_for_equal_configurations() {
        let first = default_config();
        let second = default_config();

        let first_json =
            canonical_json(&first).expect("canonical JSON must succeed");
        let second_json =
            canonical_json(&second).expect("canonical JSON must succeed");

        assert_eq!(first_json, second_json);
    }

    #[test]
    fn fingerprint_is_stable_for_equal_configurations() {
        let first = default_config();
        let second = default_config();

        let first_hash =
            fingerprint(&first).expect("fingerprint must succeed");
        let second_hash =
            fingerprint(&second).expect("fingerprint must succeed");

        assert_eq!(first_hash, second_hash);
    }

    #[test]
    fn fingerprint_hex_has_sha256_length() {
        let config = default_config();

        let value =
            fingerprint_hex(&config).expect("fingerprint must succeed");

        assert_eq!(value.len(), 64);
        assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    }

    #[test]
    fn wrong_schema_is_rejected() {
        let document = OptimizationConfigDocument {
            schema: "some.other.schema".to_owned(),
            schema_version: CURRENT_SCHEMA_VERSION,
            config: serde_json::json!({}),
        };

        let result = config_from_document(document);

        assert!(matches!(
            result,
            Err(ConfigSerializationError::UnsupportedSchema { .. })
        ));
    }

    #[test]
    fn future_schema_is_rejected() {
        let document = OptimizationConfigDocument {
            schema: CONFIG_SCHEMA.to_owned(),
            schema_version: CURRENT_SCHEMA_VERSION + 1,
            config: serde_json::json!({}),
        };

        let result = config_from_document(document);

        assert!(matches!(
            result,
            Err(ConfigSerializationError::UnsupportedSchemaVersion { .. })
        ));
    }

    #[test]
    fn non_object_configuration_is_rejected() {
        let document = OptimizationConfigDocument {
            schema: CONFIG_SCHEMA.to_owned(),
            schema_version: CURRENT_SCHEMA_VERSION,
            config: serde_json::json!(null),
        };

        let result = config_from_document(document);

        assert!(matches!(
            result,
            Err(ConfigSerializationError::InvalidConfigValue { .. })
        ));
    }

    #[test]
    fn json_validation_rejects_wrong_schema() {
        let input = r#"
        {
            "schema": "wrong.schema",
            "schema_version": 1,
            "config": {}
        }
        "#;

        let result = validate_json(input);

        assert!(matches!(
            result,
            Err(ConfigSerializationError::UnsupportedSchema { .. })
        ));
    }

    #[test]
    fn toml_round_trip_preserves_configuration() {
        let config = default_config();

        let serialized =
            serialize_toml(&config).expect("TOML serialization must succeed");

        let restored =
            deserialize_toml(&serialized).expect("TOML deserialization must succeed");

        assert_eq!(config, restored);
    }

    #[test]
    fn explicit_format_dispatch_works() {
        let config = default_config();

        let json =
            serialize(&config, ConfigSerializationFormat::Json)
                .expect("JSON serialization must succeed");

        let restored =
            deserialize(&json, ConfigSerializationFormat::Json)
                .expect("JSON deserialization must succeed");

        assert_eq!(config, restored);
    }

    #[test]
    fn format_identifiers_are_stable() {
        assert_eq!(
            ConfigSerializationFormat::Json.as_str(),
            "json"
        );

        assert_eq!(
            ConfigSerializationFormat::JsonPretty.as_str(),
            "json_pretty"
        );

        assert_eq!(
            ConfigSerializationFormat::Toml.as_str(),
            "toml"
        );
    }
}