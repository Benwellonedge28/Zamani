//! Zamani Quantum Optimization — Provenance Serialization
//!
//! Production serialization boundary for the canonical optimization
//! provenance model:
//!
//! `crate::quantum::optimization::provenance::OptimizationProvenanceSnapshot`
//!
//! # Architectural role
//!
//! This module serializes and deserializes optimization provenance. It does
//! not own:
//!
//! - optimization algorithms;
//! - optimization passes;
//! - rewrite rules;
//! - circuit transformation;
//! - quantum IR semantics;
//! - routing;
//! - scheduling;
//! - hardware execution;
//! - benchmarking;
//! - verification algorithms;
//! - hash generation for quantum IR itself.
//!
//! The canonical dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ▼
//! provenance
//!      │
//!      ▼
//! serialization::provenance
//!      │
//! ┌────┴──────────┐
//! ▼               ▼
//! JSON            TOML
//! ```
//!
//! # Canonical ownership
//!
//! `provenance.rs` owns the in-memory provenance model.
//!
//! This file owns the persisted/interchange representation of that model.
//!
//! It deliberately does not create a second provenance model containing
//! duplicated optimizer fields. The serialized document is an envelope around
//! `OptimizationProvenanceSnapshot`.
//!
//! # Why an envelope exists
//!
//! A bare serialized snapshot cannot reliably distinguish:
//!
//! - a current Zamani provenance document;
//! - an older incompatible provenance document;
//! - a future provenance document;
//! - an unrelated JSON/TOML document.
//!
//! The envelope therefore contains:
//!
//! ```text
//! schema
//! schema_version
//! format_version
//! provenance
//! ```
//!
//! # Compatibility policy
//!
//! Schema compatibility is explicit.
//!
//! This module accepts the current schema version only.
//!
//! It does not silently reinterpret future schemas.
//!
//! It does not silently migrate obsolete schemas.
//!
//! Future migrations must be implemented as explicit migration layers.
//!
//! This is essential because provenance is used for reproducibility, audit,
//! cache identity, compiler diagnostics, regression analysis, and benchmarking.
//!
//! # Determinism
//!
//! Canonical JSON is the machine-readable canonical representation.
//!
//! Canonical serialization contains no:
//!
//! - current timestamps;
//! - process IDs;
//! - memory addresses;
//! - filesystem paths;
//! - environment variables;
//! - random values;
//! - network state;
//! - implicit compiler state.
//!
//! The timestamp and other metadata already contained in the provenance
//! snapshot are serialized exactly as supplied.
//!
//! Therefore the same snapshot produces the same canonical JSON and the same
//! SHA-256 fingerprint.
//!
//! # Hashing
//!
//! This module may calculate a fingerprint of the serialized provenance
//! representation. This is different from hashing the quantum circuit itself.
//!
//! `ContentHash` inside the canonical provenance model remains an independently
//! supplied content identity.
//!
//! The fingerprint functions here hash the canonical serialized provenance
//! document itself.
//!
//! # Scaling
//!
//! Zamani is intended to scale from tiny optimization jobs to workloads limited
//! only by available resources.
//!
//! This module therefore provides two classes of API:
//!
//! 1. String APIs for convenient small/medium documents.
//! 2. Reader/writer APIs for large documents where callers should avoid
//!    unnecessary intermediate strings.
//!
//! No fixed quantum-circuit size is imposed here.
//!
//! Memory consumption is proportional to the representation requested by the
//! caller and the underlying Serde format implementation.
//!
//! Callers processing untrusted input can use `deserialize_json_with_limit` or
//! `deserialize_toml_with_limit` to impose an application-specific byte bound.
//!
//! A zero limit means "reject non-empty input"; it does not mean unlimited.
//!
//! `None` means no serialization-layer byte limit.
//!
//! # Security
//!
//! This module:
//!
//! - uses no `unsafe`;
//! - performs no filesystem I/O;
//! - performs no network I/O;
//! - executes no external process;
//! - performs no hardware access;
//! - performs no QPU access;
//! - evaluates no arbitrary code;
//! - uses no global mutable state;
//! - does not generate random identifiers;
//! - does not silently migrate unknown schemas.
//!
//! Reader/writer functions operate only on caller-supplied streams.
//!
//! # Quantum IR boundary
//!
//! This module normally does not need `QubitId`.
//!
//! Provenance serialization operates on an already constructed provenance
//! snapshot. Circuit and qubit semantics belong to `quantum::ir`.
//!
//! If future provenance structures introduce an actual qubit identity, the
//! identity MUST use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! and must never introduce `serialization::provenance::QubitId` or another
//! local quantum identifier.
//!
//! # Integration contract
//!
//! `provenance.rs`
//!     │
//!     └── provides `OptimizationProvenanceSnapshot`
//!             │
//!             ▼
//! `serialization::provenance`
//!     │
//!     ├── serializes the snapshot;
//!     ├── deserializes the snapshot;
//!     ├── validates schema/version;
//!     └── fingerprints the canonical representation.
//!
//! `result.rs` may store an `OptimizationProvenanceSnapshot`.
//!
//! `serialization::report` may embed or reference a provenance document but
//! should not redefine its fields.
//!
//! `serialization::config` serializes optimizer configuration independently.
//!
//! `context.rs` and `pipeline.rs` record provenance but should not depend on
//! this module merely to perform optimization.
//!
//! `benchmarking` may consume serialized provenance as an external artifact,
//! but this module must not depend on benchmarking.
//!
//! `quantum::ir` must not depend on this module.
//!
//! # Future-proofing
//!
//! New optimization passes, analyses, verification methods, target profiles,
//! rewrite rules, and event components are represented by the canonical
//! provenance model's extensible identifiers and records.
//!
//! This serialization boundary does not enumerate individual passes.
//!
//! Therefore adding a new pass such as:
//!
//! ```text
//! local.rotation
//! algebra.phase_polynomial
//! synthesis.two_qubit
//! fault_tolerant.t_depth
//! ```
//!
//! does not require this file to be modified.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Dependencies
//!
//! Dependencies already present in `Cargo.toml`:
//!
//! - `serde`;
//! - `serde_json`;
//! - `toml`;
//! - `sha2`;
//! - `thiserror`.
//!
//! No additional Cargo dependency is required.
//!
//! # Public API
//!
//! Primary APIs:
//!
//! ```text
//! document_from_snapshot
//! snapshot_from_document
//! validate_document
//!
//! serialize_json
//! serialize_json_pretty
//! deserialize_json
//!
//! serialize_json_to_writer
//! serialize_json_pretty_to_writer
//! deserialize_json_from_reader
//! deserialize_json_from_reader_with_limit
//!
//! serialize_toml
//! deserialize_toml
//!
//! serialize_toml_to_writer
//! deserialize_toml_from_reader
//! deserialize_toml_from_reader_with_limit
//!
//! canonical_json
//! fingerprint
//! fingerprint_hex
//! fingerprint_from_json
//! ```
//!
//! # Important distinction
//!
//! `canonical_json()` returns the canonical serialization of provenance.
//!
//! `fingerprint()` returns the SHA-256 content fingerprint of that canonical
//! representation.
//!
//! Neither operation changes the provenance snapshot.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::io::{self, Read, Write};
use thiserror::Error;

use crate::quantum::optimization::provenance::{
    OptimizationProvenanceSnapshot,
};

// =============================================================================
// Schema constants
// =============================================================================

/// Stable schema identifier for serialized optimization provenance.
pub const PROVENANCE_SCHEMA: &str =
    "zamani.quantum.optimization.provenance";

/// Current serialized provenance schema version.
///
/// Increment this only when the serialized representation requires an explicit
/// compatibility decision.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Serialization envelope version.
///
/// This is deliberately separate from the provenance schema version.
pub const FORMAT_VERSION: u32 = 1;

/// Canonical fingerprint algorithm.
///
/// The fingerprint covers the canonical serialized provenance document.
pub const FINGERPRINT_ALGORITHM: &str = "sha256";

// =============================================================================
// Result aliases
// =============================================================================

/// Result returned by provenance serialization operations.
pub type ProvenanceSerializationResult<T> =
    Result<T, ProvenanceSerializationError>;

// =============================================================================
// Serialization format
// =============================================================================

/// Supported provenance serialization formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProvenanceSerializationFormat {
    /// Canonical compact JSON.
    Json,

    /// Human-readable JSON.
    JsonPretty,

    /// Human-readable TOML.
    Toml,
}

impl ProvenanceSerializationFormat {
    /// Returns the stable format identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::JsonPretty => "json_pretty",
            Self::Toml => "toml",
        }
    }

    /// Returns whether the format is canonical.
    ///
    /// Only compact JSON is the canonical machine representation.
    #[must_use]
    pub const fn is_canonical(self) -> bool {
        matches!(self, Self::Json)
    }
}

impl fmt::Display for ProvenanceSerializationFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Serialization limits
// =============================================================================

/// Optional byte limits for serialization/deserialization operations.
///
/// These limits belong to the serialization boundary rather than to quantum
/// semantics.
///
/// `None` means no serialization-layer byte limit.
///
/// This allows applications to choose limits appropriate to their environment
/// without imposing a permanent artificial limit on Zamani itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SerializationLimits {
    /// Maximum input bytes accepted by a reader-based deserializer.
    ///
    /// `None` means unlimited from this layer's perspective.
    pub max_input_bytes: Option<u64>,

    /// Maximum output bytes written by a writer-based serializer.
    ///
    /// `None` means unlimited from this layer's perspective.
    pub max_output_bytes: Option<u64>,
}

impl SerializationLimits {
    /// Creates an unlimited serialization configuration.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_input_bytes: None,
            max_output_bytes: None,
        }
    }

    /// Creates explicit input/output limits.
    #[must_use]
    pub const fn new(
        max_input_bytes: Option<u64>,
        max_output_bytes: Option<u64>,
    ) -> Self {
        Self {
            max_input_bytes,
            max_output_bytes,
        }
    }

    /// Creates a limit where both input and output have the same bound.
    #[must_use]
    pub const fn bounded(max_bytes: u64) -> Self {
        Self {
            max_input_bytes: Some(max_bytes),
            max_output_bytes: Some(max_bytes),
        }
    }
}

// =============================================================================
// Serialization envelope
// =============================================================================

/// Stable serialized provenance envelope.
///
/// The envelope contains the canonical provenance snapshot without duplicating
/// its fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationProvenanceDocument {
    /// Stable schema identifier.
    pub schema: String,

    /// Serialization schema version.
    pub schema_version: u32,

    /// Envelope format version.
    pub format_version: u32,

    /// Canonical optimization provenance snapshot.
    pub provenance: OptimizationProvenanceSnapshot,
}

impl OptimizationProvenanceDocument {
    /// Creates a document from a provenance snapshot.
    #[must_use]
    pub fn from_snapshot(
        snapshot: &OptimizationProvenanceSnapshot,
    ) -> Self {
        Self {
            schema: PROVENANCE_SCHEMA.to_owned(),
            schema_version: CURRENT_SCHEMA_VERSION,
            format_version: FORMAT_VERSION,
            provenance: snapshot.clone(),
        }
    }

    /// Returns the schema identifier.
    #[must_use]
    pub fn schema(&self) -> &str {
        &self.schema
    }

    /// Returns the serialization schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the envelope format version.
    #[must_use]
    pub const fn format_version(&self) -> u32 {
        self.format_version
    }

    /// Returns the contained provenance snapshot.
    #[must_use]
    pub const fn provenance(
        &self,
    ) -> &OptimizationProvenanceSnapshot {
        &self.provenance
    }

    /// Consumes the document and returns its provenance snapshot.
    #[must_use]
    pub fn into_provenance(
        self,
    ) -> OptimizationProvenanceSnapshot {
        self.provenance
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by provenance serialization.
#[derive(Debug, Error)]
pub enum ProvenanceSerializationError {
    /// The serialized document belongs to another schema.
    #[error(
        "unsupported provenance schema `{actual}`; expected `{expected}`"
    )]
    UnsupportedSchema {
        /// Actual schema identifier.
        actual: String,

        /// Expected schema identifier.
        expected: &'static str,
    },

    /// The provenance schema version is unsupported.
    #[error(
        "unsupported provenance schema version {actual}; supported version is {supported}"
    )]
    UnsupportedSchemaVersion {
        /// Actual schema version.
        actual: u32,

        /// Supported schema version.
        supported: u32,
    },

    /// The envelope format version is unsupported.
    #[error(
        "unsupported provenance format version {actual}; supported version is {supported}"
    )]
    UnsupportedFormatVersion {
        /// Actual format version.
        actual: u32,

        /// Supported format version.
        supported: u32,
    },

    /// The provenance payload failed structural validation.
    #[error("invalid provenance document: {message}")]
    InvalidDocument {
        /// Explanation.
        message: String,
    },

    /// JSON serialization or deserialization failed.
    #[error("JSON serialization error: {0}")]
    Json(#[source] serde_json::Error),

    /// TOML serialization failed.
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[source] toml::ser::Error),

    /// TOML deserialization failed.
    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[source] toml::de::Error),

    /// I/O failed while using a caller-supplied reader/writer.
    #[error("provenance serialization I/O error: {0}")]
    Io(#[source] io::Error),

    /// An input/output byte limit was exceeded.
    #[error(
        "provenance serialization byte limit exceeded: actual {actual_bytes}, maximum {maximum_bytes}"
    )]
    ByteLimitExceeded {
        /// Actual bytes observed or attempted.
        actual_bytes: u64,

        /// Maximum permitted bytes.
        maximum_bytes: u64,
    },

    /// A byte counter overflowed.
    #[error("provenance serialization byte counter overflow")]
    ByteCounterOverflow,

    /// A SHA-256 digest operation failed to produce the expected representation.
    #[error("invalid SHA-256 fingerprint representation")]
    InvalidFingerprint,
}

// =============================================================================
// Document validation
// =============================================================================

/// Validates a serialized provenance document envelope.
///
/// This does not perform semantic verification of the optimization itself.
/// Provenance remains observational; semantic verification belongs to the
/// verification subsystem.
pub fn validate_document(
    document: &OptimizationProvenanceDocument,
) -> ProvenanceSerializationResult<()> {
    if document.schema != PROVENANCE_SCHEMA {
        return Err(
            ProvenanceSerializationError::UnsupportedSchema {
                actual: document.schema.clone(),
                expected: PROVENANCE_SCHEMA,
            },
        );
    }

    if document.schema_version != CURRENT_SCHEMA_VERSION {
        return Err(
            ProvenanceSerializationError::UnsupportedSchemaVersion {
                actual: document.schema_version,
                supported: CURRENT_SCHEMA_VERSION,
            },
        );
    }

    if document.format_version != FORMAT_VERSION {
        return Err(
            ProvenanceSerializationError::UnsupportedFormatVersion {
                actual: document.format_version,
                supported: FORMAT_VERSION,
            },
        );
    }

    validate_snapshot_metadata(&document.provenance)?;

    Ok(())
}

/// Validates the provenance metadata that is duplicated at the semantic
/// provenance level.
///
/// The envelope version and the provenance model's own schema version are
/// deliberately checked independently.
fn validate_snapshot_metadata(
    snapshot: &OptimizationProvenanceSnapshot,
) -> ProvenanceSerializationResult<()> {
    if snapshot.metadata.schema_version == 0 {
        return Err(
            ProvenanceSerializationError::InvalidDocument {
                message:
                    "provenance metadata schema version must be non-zero"
                        .to_owned(),
            },
        );
    }

    if snapshot.metadata.provenance_id.as_str().trim().is_empty() {
        return Err(
            ProvenanceSerializationError::InvalidDocument {
                message:
                    "provenance identifier must not be empty".to_owned(),
            },
        );
    }

    if snapshot.metadata.optimizer.name.trim().is_empty() {
        return Err(
            ProvenanceSerializationError::InvalidDocument {
                message:
                    "optimizer name must not be empty".to_owned(),
            },
        );
    }

    if snapshot.metadata.optimizer.version.trim().is_empty() {
        return Err(
            ProvenanceSerializationError::InvalidDocument {
                message:
                    "optimizer version must not be empty".to_owned(),
            },
        );
    }

    Ok(())
}

// =============================================================================
// Snapshot/document conversion
// =============================================================================

/// Converts a provenance snapshot into the stable serialization document.
#[must_use]
pub fn document_from_snapshot(
    snapshot: &OptimizationProvenanceSnapshot,
) -> OptimizationProvenanceDocument {
    OptimizationProvenanceDocument::from_snapshot(snapshot)
}

/// Converts a validated serialization document into its canonical provenance
/// snapshot.
pub fn snapshot_from_document(
    document: OptimizationProvenanceDocument,
) -> ProvenanceSerializationResult<OptimizationProvenanceSnapshot> {
    validate_document(&document)?;

    Ok(document.into_provenance())
}

// =============================================================================
// JSON string serialization
// =============================================================================

/// Serializes provenance into canonical compact JSON.
///
/// This is the preferred representation for:
///
/// - cache keys;
/// - provenance artifacts;
/// - reproducible builds;
/// - machine-to-machine interchange;
/// - fingerprints;
/// - regression testing.
pub fn serialize_json(
    snapshot: &OptimizationProvenanceSnapshot,
) -> ProvenanceSerializationResult<String> {
    let document = document_from_snapshot(snapshot);

    validate_document(&document)?;

    serde_json::to_string(&document)
        .map_err(ProvenanceSerializationError::Json)
}

/// Serializes provenance into human-readable JSON.
///
/// Pretty JSON is intended for:
///
/// - diagnostics;
/// - source control;
/// - debugging;
/// - audit inspection;
/// - documentation.
pub fn serialize_json_pretty(
    snapshot: &OptimizationProvenanceSnapshot,
) -> ProvenanceSerializationResult<String> {
    let document = document_from_snapshot(snapshot);

    validate_document(&document)?;

    serde_json::to_string_pretty(&document)
        .map_err(ProvenanceSerializationError::Json)
}

/// Deserializes canonical or ordinary JSON provenance.
///
/// The schema and format versions are validated before the snapshot is
/// returned.
pub fn deserialize_json(
    input: &str,
) -> ProvenanceSerializationResult<OptimizationProvenanceSnapshot> {
    deserialize_json_with_limit(input, None)
}

/// Deserializes JSON with an optional byte limit.
///
/// The limit is checked before Serde parsing begins.
pub fn deserialize_json_with_limit(
    input: &str,
    max_input_bytes: Option<u64>,
) -> ProvenanceSerializationResult<OptimizationProvenanceSnapshot> {
    check_input_length(input.as_bytes(), max_input_bytes)?;

    let document: OptimizationProvenanceDocument =
        serde_json::from_str(input)
            .map_err(ProvenanceSerializationError::Json)?;

    snapshot_from_document(document)
}

// =============================================================================
// JSON reader/writer APIs
// =============================================================================

/// Serializes canonical compact JSON directly to a caller-supplied writer.
///
/// This avoids requiring the caller to hold the complete serialized document
/// as a second `String`.
pub fn serialize_json_to_writer<W: Write>(
    snapshot: &OptimizationProvenanceSnapshot,
    writer: &mut W,
) -> ProvenanceSerializationResult<()> {
    serialize_json_to_writer_with_limits(
        snapshot,
        writer,
        SerializationLimits::unlimited(),
    )
}

/// Serializes canonical compact JSON directly to a writer with explicit
/// resource limits.
pub fn serialize_json_to_writer_with_limits<W: Write>(
    snapshot: &OptimizationProvenanceSnapshot,
    writer: &mut W,
    limits: SerializationLimits,
) -> ProvenanceSerializationResult<()> {
    let document = document_from_snapshot(snapshot);

    validate_document(&document)?;

    let mut limited =
        LimitedWriter::new(writer, limits.max_output_bytes);

    serde_json::to_writer(&mut limited, &document)
        .map_err(ProvenanceSerializationError::Json)?;

    limited.finish()?;

    Ok(())
}

/// Serializes pretty JSON directly to a caller-supplied writer.
pub fn serialize_json_pretty_to_writer<W: Write>(
    snapshot: &OptimizationProvenanceSnapshot,
    writer: &mut W,
) -> ProvenanceSerializationResult<()> {
    serialize_json_pretty_to_writer_with_limits(
        snapshot,
        writer,
        SerializationLimits::unlimited(),
    )
}

/// Serializes pretty JSON directly to a writer with explicit limits.
pub fn serialize_json_pretty_to_writer_with_limits<W: Write>(
    snapshot: &OptimizationProvenanceSnapshot,
    writer: &mut W,
    limits: SerializationLimits,
) -> ProvenanceSerializationResult<()> {
    let document = document_from_snapshot(snapshot);

    validate_document(&document)?;

    let mut limited =
        LimitedWriter::new(writer, limits.max_output_bytes);

    serde_json::to_writer_pretty(&mut limited, &document)
        .map_err(ProvenanceSerializationError::Json)?;

    limited.finish()?;

    Ok(())
}

/// Deserializes provenance directly from a reader.
///
/// The reader is consumed only as necessary by the JSON parser.
pub fn deserialize_json_from_reader<R: Read>(
    reader: R,
) -> ProvenanceSerializationResult<OptimizationProvenanceSnapshot> {
    deserialize_json_from_reader_with_limits(
        reader,
        SerializationLimits::unlimited(),
    )
}

/// Deserializes provenance from a reader with optional input/output limits.
///
/// Only `max_input_bytes` applies during deserialization.
pub fn deserialize_json_from_reader_with_limits<R: Read>(
    reader: R,
    limits: SerializationLimits,
) -> ProvenanceSerializationResult<OptimizationProvenanceSnapshot> {
    let mut limited =
        LimitedReader::new(reader, limits.max_input_bytes);

    let document: OptimizationProvenanceDocument =
        serde_json::from_reader(&mut limited)
            .map_err(ProvenanceSerializationError::Json)?;

    limited.finish()?;

    snapshot_from_document(document)
}

// =============================================================================
// TOML string serialization
// =============================================================================

/// Serializes provenance into TOML.
///
/// TOML is intended for human inspection and configuration-oriented tooling.
/// Canonical machine fingerprints must use JSON instead.
pub fn serialize_toml(
    snapshot: &OptimizationProvenanceSnapshot,
) -> ProvenanceSerializationResult<String> {
    let document = document_from_snapshot(snapshot);

    validate_document(&document)?;

    toml::to_string_pretty(&document)
        .map_err(ProvenanceSerializationError::TomlSerialize)
}

/// Deserializes provenance from TOML.
pub fn deserialize_toml(
    input: &str,
) -> ProvenanceSerializationResult<OptimizationProvenanceSnapshot> {
    deserialize_toml_with_limit(input, None)
}

/// Deserializes TOML with an optional byte limit.
pub fn deserialize_toml_with_limit(
    input: &str,
    max_input_bytes: Option<u64>,
) -> ProvenanceSerializationResult<OptimizationProvenanceSnapshot> {
    check_input_length(input.as_bytes(), max_input_bytes)?;

    let document: OptimizationProvenanceDocument =
        toml::from_str(input)
            .map_err(ProvenanceSerializationError::TomlDeserialize)?;

    snapshot_from_document(document)
}

// =============================================================================
// TOML reader/writer APIs
// =============================================================================

/// Serializes TOML directly to a writer.
///
/// TOML serialization is currently materialized by the TOML serializer before
/// being written because the `toml` crate's public serializer operates on a
/// serializable value rather than exposing the same direct streaming contract
/// as `serde_json`.
///
/// The caller can still impose an output limit.
pub fn serialize_toml_to_writer<W: Write>(
    snapshot: &OptimizationProvenanceSnapshot,
    writer: &mut W,
) -> ProvenanceSerializationResult<()> {
    serialize_toml_to_writer_with_limits(
        snapshot,
        writer,
        SerializationLimits::unlimited(),
    )
}

/// Serializes TOML to a writer with an explicit output limit.
pub fn serialize_toml_to_writer_with_limits<W: Write>(
    snapshot: &OptimizationProvenanceSnapshot,
    writer: &mut W,
    limits: SerializationLimits,
) -> ProvenanceSerializationResult<()> {
    let encoded = serialize_toml(snapshot)?;

    write_all_limited(
        writer,
        encoded.as_bytes(),
        limits.max_output_bytes,
    )
}

/// Deserializes TOML directly from a reader.
pub fn deserialize_toml_from_reader<R: Read>(
    reader: R,
) -> ProvenanceSerializationResult<OptimizationProvenanceSnapshot> {
    deserialize_toml_from_reader_with_limits(
        reader,
        SerializationLimits::unlimited(),
    )
}

/// Deserializes TOML from a reader with an optional input limit.
pub fn deserialize_toml_from_reader_with_limits<R: Read>(
    reader: R,
    limits: SerializationLimits,
) -> ProvenanceSerializationResult<OptimizationProvenanceSnapshot> {
    let mut input = Vec::new();

    read_all_limited(
        reader,
        &mut input,
        limits.max_input_bytes,
    )?;

    let text = std::str::from_utf8(&input).map_err(|_| {
        ProvenanceSerializationError::InvalidDocument {
            message:
                "TOML provenance input must be valid UTF-8".to_owned(),
        }
    })?;

    deserialize_toml_with_limit(text, None)
}

// =============================================================================
// Canonical JSON
// =============================================================================

/// Returns the canonical compact JSON representation.
///
/// Canonical JSON is the only representation used by the fingerprint APIs.
pub fn canonical_json(
    snapshot: &OptimizationProvenanceSnapshot,
) -> ProvenanceSerializationResult<String> {
    serialize_json(snapshot)
}

/// Writes canonical JSON directly into a writer.
///
/// This is useful for large provenance records when the caller wants to
/// calculate a digest or persist the representation without an intermediate
/// string.
pub fn write_canonical_json<W: Write>(
    snapshot: &OptimizationProvenanceSnapshot,
    writer: &mut W,
) -> ProvenanceSerializationResult<()> {
    serialize_json_to_writer(snapshot, writer)
}

// =============================================================================
// Fingerprints
// =============================================================================

/// Calculates the SHA-256 fingerprint of canonical provenance JSON.
///
/// The returned value is the raw 32-byte SHA-256 digest.
///
/// This hash identifies the serialized provenance representation. It is not a
/// replacement for the canonical input/output IR hashes stored inside the
/// provenance model.
pub fn fingerprint(
    snapshot: &OptimizationProvenanceSnapshot,
) -> ProvenanceSerializationResult<[u8; 32]> {
    let mut hasher = Sha256::new();

    {
        let mut hashing_writer = HashingWriter::new(&mut hasher);

        write_canonical_json(
            snapshot,
            &mut hashing_writer,
        )?;

        hashing_writer.finish()?;
    }

    let digest = hasher.finalize();

    let mut result = [0u8; 32];
    result.copy_from_slice(&digest);

    Ok(result)
}

/// Calculates the lowercase hexadecimal SHA-256 fingerprint of canonical
/// provenance JSON.
///
/// The representation is exactly 64 ASCII hexadecimal characters.
pub fn fingerprint_hex(
    snapshot: &OptimizationProvenanceSnapshot,
) -> ProvenanceSerializationResult<String> {
    let digest = fingerprint(snapshot)?;

    Ok(hex_encode(&digest))
}

/// Calculates the SHA-256 fingerprint of an already serialized JSON document.
///
/// This function does not parse or reinterpret the JSON. It therefore
/// fingerprints the supplied bytes exactly as supplied.
///
/// Use `fingerprint(snapshot)` when a canonical fingerprint is required.
pub fn fingerprint_from_json(
    input: &str,
) -> String {
    let digest = Sha256::digest(input.as_bytes());

    hex_encode(&digest)
}

// =============================================================================
// Utility functions
// =============================================================================

/// Checks a caller-provided input byte limit.
fn check_input_length(
    bytes: &[u8],
    maximum: Option<u64>,
) -> ProvenanceSerializationResult<()> {
    if let Some(maximum) = maximum {
        let actual =
            u64::try_from(bytes.len())
                .map_err(|_| {
                    ProvenanceSerializationError::ByteCounterOverflow
                })?;

        if actual > maximum {
            return Err(
                ProvenanceSerializationError::ByteLimitExceeded {
                    actual_bytes: actual,
                    maximum_bytes: maximum,
                },
            );
        }
    }

    Ok(())
}

/// Writes bytes while enforcing an optional output limit.
fn write_all_limited<W: Write>(
    writer: &mut W,
    bytes: &[u8],
    maximum: Option<u64>,
) -> ProvenanceSerializationResult<()> {
    let actual =
        u64::try_from(bytes.len())
            .map_err(|_| {
                ProvenanceSerializationError::ByteCounterOverflow
            })?;

    if let Some(maximum) = maximum {
        if actual > maximum {
            return Err(
                ProvenanceSerializationError::ByteLimitExceeded {
                    actual_bytes: actual,
                    maximum_bytes: maximum,
                },
            );
        }
    }

    writer
        .write_all(bytes)
        .map_err(ProvenanceSerializationError::Io)
}

/// Reads all bytes from a reader while enforcing an optional limit.
fn read_all_limited<R: Read>(
    mut reader: R,
    destination: &mut Vec<u8>,
    maximum: Option<u64>,
) -> ProvenanceSerializationResult<()> {
    const BUFFER_SIZE: usize = 64 * 1024;

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut total = 0u64;

    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(ProvenanceSerializationError::Io)?;

        if read == 0 {
            break;
        }

        let read_u64 =
            u64::try_from(read)
                .map_err(|_| {
                    ProvenanceSerializationError::ByteCounterOverflow
                })?;

        total = total
            .checked_add(read_u64)
            .ok_or(
                ProvenanceSerializationError::ByteCounterOverflow,
            )?;

        if let Some(maximum) = maximum {
            if total > maximum {
                return Err(
                    ProvenanceSerializationError::ByteLimitExceeded {
                        actual_bytes: total,
                        maximum_bytes: maximum,
                    },
                );
            }
        }

        destination.extend_from_slice(&buffer[..read]);
    }

    Ok(())
}

/// Encodes bytes as lowercase hexadecimal without introducing another runtime
/// dependency.
fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] =
        b"0123456789abcdef";

    let capacity = bytes
        .len()
        .checked_mul(2)
        .unwrap_or(0);

    let mut result = String::with_capacity(capacity);

    for &byte in bytes {
        result.push(HEX[(byte >> 4) as usize] as char);
        result.push(HEX[(byte & 0x0f) as usize] as char);
    }

    result
}

// =============================================================================
// Limited writer
// =============================================================================

/// Writer adapter enforcing an optional byte limit.
///
/// The adapter never uses unsafe code and never buffers the complete output.
struct LimitedWriter<'a, W: Write + ?Sized> {
    inner: &'a mut W,
    maximum: Option<u64>,
    written: u64,
}

impl<'a, W: Write + ?Sized> LimitedWriter<'a, W> {
    fn new(
        inner: &'a mut W,
        maximum: Option<u64>,
    ) -> Self {
        Self {
            inner,
            maximum,
            written: 0,
        }
    }

    fn finish(
        self,
    ) -> ProvenanceSerializationResult<()> {
        Ok(())
    }
}

impl<W: Write + ?Sized> Write for LimitedWriter<'_, W> {
    fn write(
        &mut self,
        buffer: &[u8],
    ) -> io::Result<usize> {
        let requested =
            u64::try_from(buffer.len())
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "serialization byte counter overflow",
                    )
                })?;

        let new_total =
            self.written
                .checked_add(requested)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "serialization byte counter overflow",
                    )
                })?;

        if let Some(maximum) = self.maximum {
            if new_total > maximum {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    format!(
                        "provenance output byte limit exceeded: \
                         attempted {new_total}, maximum {maximum}"
                    ),
                ));
            }
        }

        let written = self.inner.write(buffer)?;

        let written_u64 =
            u64::try_from(written)
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "serialization byte counter overflow",
                    )
                })?;

        self.written =
            self.written
                .checked_add(written_u64)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "serialization byte counter overflow",
                    )
                })?;

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

// =============================================================================
// Limited reader
// =============================================================================

/// Reader adapter enforcing an optional byte limit.
struct LimitedReader<R: Read> {
    inner: R,
    maximum: Option<u64>,
    read: u64,
}

impl<R: Read> LimitedReader<R> {
    fn new(
        inner: R,
        maximum: Option<u64>,
    ) -> Self {
        Self {
            inner,
            maximum,
            read: 0,
        }
    }

    fn finish(
        self,
    ) -> ProvenanceSerializationResult<()> {
        Ok(())
    }
}

impl<R: Read> Read for LimitedReader<R> {
    fn read(
        &mut self,
        buffer: &mut [u8],
    ) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        let allowed_len =
            match self.maximum {
                Some(maximum) => {
                    if self.read >= maximum {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "provenance input byte limit exceeded: \
                                 maximum {maximum}"
                            ),
                        ));
                    }

                    let remaining =
                        maximum - self.read;

                    let remaining_usize =
                        usize::try_from(remaining)
                            .unwrap_or(usize::MAX);

                    buffer.len().min(remaining_usize)
                }

                None => buffer.len(),
            };

        let read =
            self.inner.read(&mut buffer[..allowed_len])?;

        let read_u64 =
            u64::try_from(read)
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "serialization byte counter overflow",
                    )
                })?;

        self.read =
            self.read
                .checked_add(read_u64)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "serialization byte counter overflow",
                    )
                })?;

        Ok(read)
    }
}

// =============================================================================
// Hashing writer
// =============================================================================

/// Streaming writer that feeds every written byte into a SHA-256 hasher.
///
/// This avoids creating a second full canonical JSON string solely to calculate
/// the fingerprint.
struct HashingWriter<'a> {
    hasher: &'a mut Sha256,
    written: u128,
}

impl<'a> HashingWriter<'a> {
    fn new(hasher: &'a mut Sha256) -> Self {
        Self {
            hasher,
            written: 0,
        }
    }

    fn finish(
        self,
    ) -> ProvenanceSerializationResult<()> {
        Ok(())
    }
}

impl Write for HashingWriter<'_> {
    fn write(
        &mut self,
        buffer: &[u8],
    ) -> io::Result<usize> {
        self.hasher.update(buffer);

        let amount =
            u128::try_from(buffer.len())
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "provenance fingerprint byte counter overflow",
                    )
                })?;

        self.written =
            self.written
                .checked_add(amount)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "provenance fingerprint byte counter overflow",
                    )
                })?;

        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::optimization::provenance::{
        OptimizationProvenance,
        ProvenanceLimits,
        ProvenanceMode,
    };

    fn sample_snapshot() -> OptimizationProvenanceSnapshot {
        let provenance = OptimizationProvenance::new(
            ProvenanceMode::Bounded,
            ProvenanceLimits::compact(),
        )
        .expect("create provenance");

        provenance.snapshot()
    }

    #[test]
    fn document_has_stable_schema() {
        let snapshot = sample_snapshot();

        let document =
            document_from_snapshot(&snapshot);

        assert_eq!(
            document.schema(),
            PROVENANCE_SCHEMA
        );

        assert_eq!(
            document.schema_version(),
            CURRENT_SCHEMA_VERSION
        );

        assert_eq!(
            document.format_version(),
            FORMAT_VERSION
        );
    }

    #[test]
    fn document_round_trip_preserves_snapshot() {
        let snapshot = sample_snapshot();

        let document =
            document_from_snapshot(&snapshot);

        let decoded =
            snapshot_from_document(document)
                .expect("decode document");

        assert_eq!(decoded, snapshot);
    }

    #[test]
    fn json_round_trip_preserves_snapshot() {
        let snapshot = sample_snapshot();

        let encoded =
            serialize_json(&snapshot)
                .expect("serialize JSON");

        assert!(!encoded.is_empty());

        let decoded =
            deserialize_json(&encoded)
                .expect("deserialize JSON");

        assert_eq!(decoded, snapshot);
    }

    #[test]
    fn pretty_json_round_trip_preserves_snapshot() {
        let snapshot = sample_snapshot();

        let encoded =
            serialize_json_pretty(&snapshot)
                .expect("serialize pretty JSON");

        assert!(encoded.contains('\n'));

        let decoded =
            deserialize_json(&encoded)
                .expect("deserialize pretty JSON");

        assert_eq!(decoded, snapshot);
    }

    #[test]
    fn toml_round_trip_preserves_snapshot() {
        let snapshot = sample_snapshot();

        let encoded =
            serialize_toml(&snapshot)
                .expect("serialize TOML");

        assert!(!encoded.is_empty());

        let decoded =
            deserialize_toml(&encoded)
                .expect("deserialize TOML");

        assert_eq!(decoded, snapshot);
    }

    #[test]
    fn canonical_json_is_deterministic() {
        let snapshot = sample_snapshot();

        let first =
            canonical_json(&snapshot)
                .expect("canonical JSON");

        let second =
            canonical_json(&snapshot)
                .expect("canonical JSON");

        assert_eq!(first, second);
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let snapshot = sample_snapshot();

        let first =
            fingerprint_hex(&snapshot)
                .expect("fingerprint");

        let second =
            fingerprint_hex(&snapshot)
                .expect("fingerprint");

        assert_eq!(first, second);
        assert_eq!(first.len(), 64);
        assert!(
            first
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
        );
    }

    #[test]
    fn fingerprint_from_canonical_json_matches_snapshot_fingerprint() {
        let snapshot = sample_snapshot();

        let canonical =
            canonical_json(&snapshot)
                .expect("canonical JSON");

        let from_snapshot =
            fingerprint_hex(&snapshot)
                .expect("snapshot fingerprint");

        let from_json =
            fingerprint_from_json(&canonical);

        assert_eq!(
            from_snapshot,
            from_json
        );
    }

    #[test]
    fn writer_json_matches_string_json() {
        let snapshot = sample_snapshot();

        let expected =
            serialize_json(&snapshot)
                .expect("JSON");

        let mut output = Vec::new();

        serialize_json_to_writer(
            &snapshot,
            &mut output,
        )
        .expect("writer JSON");

        let actual =
            String::from_utf8(output)
                .expect("UTF-8");

        assert_eq!(actual, expected);
    }

    #[test]
    fn reader_json_matches_string_json() {
        let snapshot = sample_snapshot();

        let encoded =
            serialize_json(&snapshot)
                .expect("JSON");

        let decoded =
            deserialize_json_from_reader(
                encoded.as_bytes(),
            )
            .expect("reader JSON");

        assert_eq!(decoded, snapshot);
    }

    #[test]
    fn reader_toml_matches_string_toml() {
        let snapshot = sample_snapshot();

        let encoded =
            serialize_toml(&snapshot)
                .expect("TOML");

        let decoded =
            deserialize_toml_from_reader(
                encoded.as_bytes(),
            )
            .expect("reader TOML");

        assert_eq!(decoded, snapshot);
    }

    #[test]
    fn invalid_schema_is_rejected() {
        let snapshot = sample_snapshot();

        let mut document =
            document_from_snapshot(&snapshot);

        document.schema =
            "some.other.schema".to_owned();

        let result =
            validate_document(&document);

        assert!(matches!(
            result,
            Err(
                ProvenanceSerializationError::UnsupportedSchema {
                    ..
                }
            )
        ));
    }

    #[test]
    fn future_schema_version_is_rejected() {
        let snapshot = sample_snapshot();

        let mut document =
            document_from_snapshot(&snapshot);

        document.schema_version =
            CURRENT_SCHEMA_VERSION + 1;

        let result =
            validate_document(&document);

        assert!(matches!(
            result,
            Err(
                ProvenanceSerializationError::UnsupportedSchemaVersion {
                    ..
                }
            )
        ));
    }

    #[test]
    fn future_format_version_is_rejected() {
        let snapshot = sample_snapshot();

        let mut document =
            document_from_snapshot(&snapshot);

        document.format_version =
            FORMAT_VERSION + 1;

        let result =
            validate_document(&document);

        assert!(matches!(
            result,
            Err(
                ProvenanceSerializationError::UnsupportedFormatVersion {
                    ..
                }
            )
        ));
    }

    #[test]
    fn input_limit_is_enforced() {
        let snapshot = sample_snapshot();

        let encoded =
            serialize_json(&snapshot)
                .expect("JSON");

        let result =
            deserialize_json_with_limit(
                &encoded,
                Some(1),
            );

        assert!(matches!(
            result,
            Err(
                ProvenanceSerializationError::ByteLimitExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn reader_input_limit_is_enforced() {
        let snapshot = sample_snapshot();

        let encoded =
            serialize_json(&snapshot)
                .expect("JSON");

        let result =
            deserialize_json_from_reader_with_limits(
                encoded.as_bytes(),
                SerializationLimits::new(
                    Some(1),
                    None,
                ),
            );

        assert!(result.is_err());
    }

    #[test]
    fn output_limit_is_enforced() {
        let snapshot = sample_snapshot();

        let mut output = Vec::new();

        let result =
            serialize_json_to_writer_with_limits(
                &snapshot,
                &mut output,
                SerializationLimits::new(
                    None,
                    Some(1),
                ),
            );

        assert!(result.is_err());
    }

    #[test]
    fn pretty_writer_matches_pretty_string() {
        let snapshot = sample_snapshot();

        let expected =
            serialize_json_pretty(&snapshot)
                .expect("pretty JSON");

        let mut output = Vec::new();

        serialize_json_pretty_to_writer(
            &snapshot,
            &mut output,
        )
        .expect("pretty writer");

        let actual =
            String::from_utf8(output)
                .expect("UTF-8");

        assert_eq!(actual, expected);
    }

    #[test]
    fn toml_writer_produces_valid_document() {
        let snapshot = sample_snapshot();

        let mut output = Vec::new();

        serialize_toml_to_writer(
            &snapshot,
            &mut output,
        )
        .expect("TOML writer");

        let text =
            String::from_utf8(output)
                .expect("UTF-8");

        let decoded =
            deserialize_toml(&text)
                .expect("TOML decode");

        assert_eq!(decoded, snapshot);
    }

    #[test]
    fn sha256_fingerprint_has_expected_length() {
        let snapshot = sample_snapshot();

        let digest =
            fingerprint(&snapshot)
                .expect("fingerprint");

        assert_eq!(digest.len(), 32);
    }

    #[test]
    fn fingerprint_is_lowercase_hex() {
        let snapshot = sample_snapshot();

        let fingerprint =
            fingerprint_hex(&snapshot)
                .expect("fingerprint");

        assert_eq!(fingerprint.len(), 64);

        assert!(
            fingerprint
                .bytes()
                .all(|byte| {
                    byte.is_ascii_digit()
                        || (b'a'..=b'f').contains(&byte)
                })
        );
    }

    #[test]
    fn serialization_format_identifiers_are_stable() {
        assert_eq!(
            ProvenanceSerializationFormat::Json.as_str(),
            "json"
        );

        assert_eq!(
            ProvenanceSerializationFormat::JsonPretty.as_str(),
            "json_pretty"
        );

        assert_eq!(
            ProvenanceSerializationFormat::Toml.as_str(),
            "toml"
        );

        assert!(
            ProvenanceSerializationFormat::Json
                .is_canonical()
        );

        assert!(
            !ProvenanceSerializationFormat::Toml
                .is_canonical()
        );
    }

    #[test]
    fn unlimited_limits_have_no_bounds() {
        let limits =
            SerializationLimits::unlimited();

        assert_eq!(
            limits.max_input_bytes,
            None
        );

        assert_eq!(
            limits.max_output_bytes,
            None
        );
    }

    #[test]
    fn bounded_limits_are_explicit() {
        let limits =
            SerializationLimits::bounded(1024);

        assert_eq!(
            limits.max_input_bytes,
            Some(1024)
        );

        assert_eq!(
            limits.max_output_bytes,
            Some(1024)
        );
    }

    #[test]
    fn no_unsafe_code_is_required_for_streaming() {
        // This test intentionally exercises the public streaming boundary.
        // The implementation uses only safe Rust reader/writer adapters.
        let snapshot = sample_snapshot();

        let mut output = Vec::new();

        write_canonical_json(
            &snapshot,
            &mut output,
        )
        .expect("canonical streaming JSON");

        assert!(!output.is_empty());
    }
}