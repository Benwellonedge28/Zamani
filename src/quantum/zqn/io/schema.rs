//! Zamani Quantum Noise (ZQN) — Persistent Schema Contract.
//!
//! # Ownership
//!
//! This file owns the *logical schema contract* for persisted ZQN documents.
//!
//! It defines:
//!
//! - the stable ZQN document schema identifier;
//! - the relationship between semantic, schema, and compatibility versions;
//! - document-kind classification;
//! - document identity representation;
//! - schema metadata;
//! - provenance-neutral document metadata;
//! - generic, format-neutral document payload representation;
//! - schema envelope validation;
//! - schema expectations;
//! - explicit compatibility requirements;
//! - schema-level validation errors;
//! - resource-safe validation configuration;
//! - forward-compatible handling of unknown metadata fields;
//! - deterministic validation semantics;
//! - integration contracts for the remaining ZQN I/O subsystem.
//!
//! It does NOT own:
//!
//! - ZQN semantic objects;
//! - probability semantics;
//! - quantum-channel mathematics;
//! - fault semantics;
//! - noise-model semantics;
//! - calibration semantics;
//! - characterization semantics;
//! - simulation;
//! - propagation;
//! - target capabilities;
//! - runtime resource accounting;
//! - canonical byte encoding;
//! - JSON/TOML/YAML transport;
//! - compression;
//! - encryption;
//! - digital signatures;
//! - migration implementations;
//! - quantum IR semantics;
//! - qubit identity.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         ZQN semantic objects
//!                                  │
//!                                  ▼
//!                    ┌─────────────────────────┐
//!                    │     ZQN I/O schema      │
//!                    │                         │
//!                    │  THIS FILE              │
//!                    │                         │
//!                    │  document contract      │
//!                    └────────────┬────────────┘
//!                                 │
//!                    ┌────────────┼────────────┐
//!                    ▼            ▼            ▼
//!              serialization deserialization canonical
//!                    │            │            │
//!                    └────────────┼────────────┘
//!                                 ▼
//!                         external persistence
//! ```
//!
//! The schema layer is deliberately below transport formats and above concrete
//! ZQN domain objects.
//!
//! # Critical architectural rule
//!
//! A serialized ZQN document is NOT defined by Rust's in-memory struct layout.
//!
//! The Rust structures in this file are the implementation of the schema
//! contract. Their memory layout is an implementation detail.
//!
//! The persisted representation must instead be defined by explicit field
//! names, explicit version information, and explicit compatibility rules.
//!
//! # Write once, scale everywhere
//!
//! This schema imposes no semantic upper bound on:
//!
//! - qubits;
//! - physical resources;
//! - logical resources;
//! - operations;
//! - circuit depth;
//! - fault count;
//! - channel dimension;
//! - tensor dimension;
//! - distribution size;
//! - calibration count;
//! - observations;
//! - experiments;
//! - nodes;
//! - links;
//! - execution duration;
//! - machine size.
//!
//! "Infinity" means:
//!
//! > no artificial finite machine-size ceiling is encoded by this schema.
//!
//! Every actual document is finite because an actual process, storage medium,
//! address space, transport, compiler invocation, and execution environment
//! are finite.
//!
//! Resource ceilings therefore belong to explicit runtime/resource policy such
//! as `crate::quantum::zqn::core::limits`, not to this schema contract.
//!
//! # Canonical quantum identity
//!
//! This file intentionally does NOT define:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! The canonical quantum identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Higher-level semantic ZQN structures that contain concrete quantum
//! resources must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The persistence schema itself does not need to depend on those types because
//! the payload is deliberately generic. This keeps the schema layer usable for
//! all supported/future quantum modalities rather than forcing every persisted
//! document to be qubit-specific.
//!
//! # Why the payload is format-neutral
//!
//! `serde_json::Value` is used as a format-neutral structured value model.
//!
//! It is NOT a statement that JSON is the only ZQN wire format.
//!
//! The same schema contract can be transported through JSON, TOML, another
//! Serde-compatible representation, or a future binary codec.
//!
//! Concrete transport belongs to `serialization.rs` and `deserialization.rs`.
//!
//! # Unknown fields
//!
//! Forward compatibility requires consumers to be able to encounter fields
//! introduced by a newer producer.
//!
//! Therefore this schema deliberately does not require every metadata field to
//! be known by every reader.
//!
//! Unknown semantic payload fields must NOT be silently interpreted as known
//! fields.
//!
//! The semantic compatibility layer is responsible for determining whether an
//! unknown field is:
//!
//! - ignorable metadata;
//! - an optional extension;
//! - required by the document kind;
//! - incompatible with the consumer.
//!
//! # Version ownership
//!
//! This file consumes the authoritative version definitions from:
//!
//! ```text
//! crate::quantum::zqn::core::version
//! ```
//!
//! It does not create competing semantic/schema/compatibility version types.
//!
//! In particular, the following remain authoritative:
//!
//! ```text
//! ZqnVersion
//! ZqnSchemaVersion
//! ZqnCompatibilityVersion
//! ZqnVersionMetadata
//! ZQN_VERSION_METADATA
//! ```
//!
//! # Version separation
//!
//! Three version concepts must remain separate:
//!
//! ```text
//! semantic version
//!     = meaning/API contract
//!
//! schema version
//!     = persisted structural contract
//!
//! compatibility version
//!     = consumer/producer compatibility guarantees
//! ```
//!
//! A change in one dimension must not be silently represented as a change in
//! another.
//!
//! # Integration contract
//!
//! Later files should consume this file as follows:
//!
//! ```text
//! io/schema.rs
//!     │
//!     ├──► io/serialization.rs
//!     ├──► io/deserialization.rs
//!     ├──► io/canonical.rs
//!     └──► io/compatibility.rs
//! ```
//!
//! Domain modules may construct schema documents through the public types
//! defined here.
//!
//! This file must not import those domain modules. That keeps the dependency
//! direction acyclic.
//!
//! # Serialization contract
//!
//! This file defines what must be serialized.
//!
//! It does NOT define how bytes are produced.
//!
//! `serialization.rs` owns:
//!
//! - encoding;
//! - format selection;
//! - serializer configuration;
//! - byte/string production;
//! - transport-specific errors.
//!
//! `deserialization.rs` owns:
//!
//! - parsing;
//! - decoding;
//! - envelope construction;
//! - structural validation;
//! - malformed-input handling.
//!
//! `canonical.rs` owns:
//!
//! - canonical ordering;
//! - canonical byte representation;
//! - canonical hashing input.
//!
//! `compatibility.rs` owns:
//!
//! - migration classification;
//! - compatibility matrices;
//! - historical schema handling;
//! - migration dispatch.
//!
//! None of those responsibilities are duplicated here.
//!
//! # Determinism
//!
//! Schema validation is deterministic.
//!
//! It must not depend on:
//!
//! - clocks;
//! - random numbers;
//! - environment variables;
//! - filesystem state;
//! - network state;
//! - process IDs;
//! - memory addresses;
//! - thread scheduling;
//! - global mutable state.
//!
//! # Security
//!
//! The schema layer treats persisted documents as untrusted input.
//!
//! It must reject structurally invalid metadata and versions without panicking.
//!
//! It must never:
//!
//! - execute code from payloads;
//! - invoke external processes;
//! - access the network;
//! - access the filesystem;
//! - allocate based on attacker-controlled dimensions without downstream
//!   resource policy;
//! - reinterpret unknown semantic fields as trusted known fields.
//!
//! Expensive payload validation belongs to the semantic subsystem and must be
//! performed under explicit `ZqnLimits`/runtime policy.
//!
//! # No unsafe
//!
//! This file intentionally uses no `unsafe` code.
//!
//! The compiler is instructed to reject unsafe code.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. one stable document schema identifier exists;
//! 2. schema, semantic and compatibility versions remain distinct;
//! 3. schema validation is deterministic;
//! 4. malformed versions return errors;
//! 5. empty required identifiers are rejected;
//! 6. no machine-size ceiling exists;
//! 7. no vendor-specific type exists;
//! 8. no second qubit identity exists;
//! 9. no concrete transport format is required;
//! 10. unknown extension metadata can survive schema processing;
//! 11. domain payload semantics remain outside this file;
//! 12. canonical byte encoding remains outside this file;
//! 13. migration implementation remains outside this file;
//! 14. validation does not perform I/O;
//! 15. validation does not use global state;
//! 16. validation never panics on malformed external metadata;
//! 17. later I/O files can depend on this file without modifying its contract;
//! 18. future quantum modalities can use the schema without redesigning the
//!     envelope;
//! 19. larger quantum machines do not require changes to this file;
//! 20. no unsafe code is present.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::quantum::zqn::core::version::{
    Compatibility,
    VersionError,
    VersionRequirement,
    ZqnCompatibilityVersion,
    ZqnSchemaVersion,
    ZqnVersion,
    ZqnVersionMetadata,
    ZQN_VERSION_METADATA,
};

// =============================================================================
// Stable schema identity
// =============================================================================

/// Stable identifier for the ZQN persisted-document schema.
///
/// This identifies the *schema family*, not a particular semantic object.
///
/// The identifier must remain stable across backward-compatible schema
/// evolution.
///
/// If a fundamentally different persistence family is ever introduced, it
/// should receive a distinct schema identifier rather than silently changing
/// the meaning of this one.
pub const ZQN_DOCUMENT_SCHEMA_ID: &str = "zamani.quantum.zqn";

/// Current persisted-document schema version.
///
/// The authoritative version number is owned by `core::version`.
///
/// This constant exists as a schema-layer convenience alias so callers do not
/// need to duplicate or reinterpret the version contract.
pub const ZQN_DOCUMENT_SCHEMA_VERSION: ZqnSchemaVersion = ZQN_VERSION_METADATA.schema;

/// Current ZQN compatibility contract.
///
/// This is the compatibility level required/provided by the current schema
/// implementation.
pub const ZQN_DOCUMENT_COMPATIBILITY_VERSION: ZqnCompatibilityVersion =
    ZQN_VERSION_METADATA.compatibility;

/// Current ZQN semantic version associated with the schema implementation.
pub const ZQN_DOCUMENT_SEMANTIC_VERSION: ZqnVersion = ZQN_VERSION_METADATA.semantic;

// =============================================================================
// Schema field names
// =============================================================================
//
// These constants centralize persisted field names.
//
// They are deliberately not generated dynamically.
//
// Changing a persisted field name is a schema change and therefore must be
// deliberate and versioned.

/// Top-level schema identifier field.
pub const FIELD_SCHEMA_ID: &str = "schema_id";

/// Top-level schema version field.
pub const FIELD_SCHEMA_VERSION: &str = "schema_version";

/// Top-level semantic version field.
pub const FIELD_SEMANTIC_VERSION: &str = "semantic_version";

/// Top-level compatibility version field.
pub const FIELD_COMPATIBILITY_VERSION: &str = "compatibility_version";

/// Top-level document kind field.
pub const FIELD_DOCUMENT_KIND: &str = "document_kind";

/// Top-level document identity field.
pub const FIELD_DOCUMENT_ID: &str = "document_id";

/// Top-level metadata field.
pub const FIELD_METADATA: &str = "metadata";

/// Top-level payload field.
pub const FIELD_PAYLOAD: &str = "payload";

// =============================================================================
// Schema metadata field names
// =============================================================================

/// Metadata producer identifier.
pub const FIELD_METADATA_PRODUCER: &str = "producer";

/// Metadata description.
pub const FIELD_METADATA_DESCRIPTION: &str = "description";

/// Metadata labels.
pub const FIELD_METADATA_LABELS: &str = "labels";

/// Metadata extension map.
pub const FIELD_METADATA_EXTENSIONS: &str = "extensions";

// =============================================================================
// Document kind
// =============================================================================

/// Classification of a persisted ZQN document.
///
/// This is intentionally broad and extensible.
///
/// It identifies the role of the document rather than imposing a concrete
/// Rust payload type. Concrete payload semantics remain owned by the relevant
/// ZQN subsystem.
///
/// The enum is `non_exhaustive` so adding a new document kind does not require
/// breaking downstream exhaustive matches.
///
/// Serialization is explicit and stable through Serde's `snake_case` names.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ZqnDocumentKind {
    /// A complete noise-model document.
    NoiseModel,

    /// A channel/process description.
    Channel,

    /// A fault-model description.
    FaultModel,

    /// A probability/distribution document.
    ProbabilityModel,

    /// A calibration snapshot.
    Calibration,

    /// A characterization result.
    Characterization,

    /// A simulation/reproducibility configuration.
    Simulation,

    /// An error/uncertainty propagation result.
    Propagation,

    /// A target capability/noise requirement document.
    Target,

    /// An observation/result document.
    Observation,

    /// A generic extension document.
    Extension,
}

impl ZqnDocumentKind {
    /// Returns the stable serialized identifier for this document kind.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoiseModel => "noise_model",
            Self::Channel => "channel",
            Self::FaultModel => "fault_model",
            Self::ProbabilityModel => "probability_model",
            Self::Calibration => "calibration",
            Self::Characterization => "characterization",
            Self::Simulation => "simulation",
            Self::Propagation => "propagation",
            Self::Target => "target",
            Self::Observation => "observation",
            Self::Extension => "extension",
        }
    }
}

impl fmt::Display for ZqnDocumentKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Document identity
// =============================================================================

/// Stable identifier for a persisted ZQN document.
///
/// This is a ZQN document identity, NOT a quantum-resource identity.
///
/// It must never be confused with:
///
/// ```text
/// QubitId
/// PhysicalQubitId
/// OperationId
/// ```
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct ZqnDocumentId(String);

impl ZqnDocumentId {
    /// Creates a document identifier.
    ///
    /// Empty identifiers are rejected.
    pub fn new(value: impl Into<String>) -> Result<Self, SchemaError> {
        let value = value.into();

        if value.is_empty() {
            return Err(SchemaError::EmptyDocumentId);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ZqnDocumentId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ZqnDocumentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Metadata
// =============================================================================

/// Schema-level document metadata.
///
/// Metadata is deliberately separate from the semantic payload.
///
/// This permits provenance, labels and extension information to evolve without
/// changing the semantic object representation.
///
/// Unknown metadata extensions are preserved as structured values.
///
/// The metadata object must remain semantically non-authoritative: it must not
/// override schema/version fields in the envelope.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ZqnDocumentMetadata {
    /// Optional producer identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,

    /// Optional human-readable description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Stable application-defined labels.
    ///
    /// A `Map` is used instead of `HashMap` so callers have deterministic
    /// key ordering independent of randomized hash state.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub labels: Map<String, Value>,

    /// Forward-compatible metadata extensions.
    ///
    /// Extensions are not interpreted by this schema module.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub extensions: Map<String, Value>,
}

impl ZqnDocumentMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces a label.
    ///
    /// Returns the previous value, if one existed.
    pub fn insert_label(
        &mut self,
        key: impl Into<String>,
        value: Value,
    ) -> Option<Value> {
        self.labels.insert(key.into(), value)
    }

    /// Adds or replaces an extension.
    ///
    /// Returns the previous value, if one existed.
    ///
    /// Extension values are intentionally opaque to this layer.
    pub fn insert_extension(
        &mut self,
        key: impl Into<String>,
        value: Value,
    ) -> Option<Value> {
        self.extensions.insert(key.into(), value)
    }

    /// Returns whether no metadata has been supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.producer.is_none()
            && self.description.is_none()
            && self.labels.is_empty()
            && self.extensions.is_empty()
    }

    /// Validates schema-level metadata.
    ///
    /// This validates only structural properties owned by this module.
    ///
    /// It does not inspect arbitrary extension payload semantics.
    pub fn validate(&self) -> Result<(), SchemaError> {
        if matches!(self.producer.as_deref(), Some("")) {
            return Err(SchemaError::EmptyMetadataField {
                field: FIELD_METADATA_PRODUCER,
            });
        }

        if matches!(self.description.as_deref(), Some("")) {
            return Err(SchemaError::EmptyMetadataField {
                field: FIELD_METADATA_DESCRIPTION,
            });
        }

        validate_metadata_keys(&self.labels, FIELD_METADATA_LABELS)?;
        validate_metadata_keys(
            &self.extensions,
            FIELD_METADATA_EXTENSIONS,
        )?;

        Ok(())
    }
}

// =============================================================================
// Generic payload
// =============================================================================

/// Format-neutral ZQN semantic payload.
///
/// This wrapper deliberately prevents the schema layer from becoming coupled
/// to a particular ZQN semantic subsystem.
///
/// The payload may contain:
///
/// - an object;
/// - an array;
/// - a scalar;
///
/// but the concrete document kind determines what semantic structure is valid.
///
/// `schema.rs` performs only structural checks. Domain-specific validation is
/// performed by the relevant ZQN subsystem.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ZqnDocumentPayload(Value);

impl ZqnDocumentPayload {
    /// Creates a payload from a structured Serde value.
    #[must_use]
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    /// Creates an empty object payload.
    #[must_use]
    pub fn object() -> Self {
        Self(Value::Object(Map::new()))
    }

    /// Creates an array payload.
    #[must_use]
    pub fn array(values: Vec<Value>) -> Self {
        Self(Value::Array(values))
    }

    /// Returns a reference to the underlying value.
    #[must_use]
    pub fn as_value(&self) -> &Value {
        &self.0
    }

    /// Returns a mutable reference to the underlying value.
    ///
    /// Mutation is allowed because this type represents a construction-time
    /// payload. Validation must be performed before persistence.
    #[must_use]
    pub fn as_value_mut(&mut self) -> &mut Value {
        &mut self.0
    }

    /// Consumes the payload and returns the underlying value.
    #[must_use]
    pub fn into_value(self) -> Value {
        self.0
    }

    /// Returns whether the payload is a JSON object.
    #[must_use]
    pub fn is_object(&self) -> bool {
        self.0.is_object()
    }

    /// Returns whether the payload is a JSON array.
    #[must_use]
    pub fn is_array(&self) -> bool {
        self.0.is_array()
    }

    /// Returns whether the payload is null.
    #[must_use]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl From<Value> for ZqnDocumentPayload {
    fn from(value: Value) -> Self {
        Self::new(value)
    }
}

// =============================================================================
// Complete document envelope
// =============================================================================

/// Complete versioned ZQN persistence envelope.
///
/// This is the central schema contract.
///
/// The envelope contains enough information for a reader to determine:
///
/// 1. which schema family it is reading;
/// 2. which structural schema version it uses;
/// 3. which semantic ZQN version produced it;
/// 4. which compatibility contract it expects;
/// 5. what kind of document it contains;
/// 6. which document instance is being read;
/// 7. what schema metadata accompanies it;
/// 8. where the domain-specific payload is stored.
///
/// The payload itself remains opaque to this layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZqnDocument {
    /// Stable schema-family identifier.
    pub schema_id: String,

    /// Persisted structural schema version.
    pub schema_version: ZqnSchemaVersion,

    /// ZQN semantic version associated with the document.
    pub semantic_version: ZqnVersion,

    /// Compatibility contract advertised by the producer.
    pub compatibility_version: ZqnCompatibilityVersion,

    /// Semantic role of the document.
    pub document_kind: ZqnDocumentKind,

    /// Stable document identity.
    pub document_id: ZqnDocumentId,

    /// Schema-level metadata.
    #[serde(default)]
    pub metadata: ZqnDocumentMetadata,

    /// Domain-specific payload.
    pub payload: ZqnDocumentPayload,
}

impl ZqnDocument {
    /// Creates a document using the current ZQN schema contract.
    ///
    /// The caller supplies the document identity, kind, metadata and payload.
    ///
    /// Current implementation version metadata is inserted automatically.
    pub fn new(
        document_id: ZqnDocumentId,
        document_kind: ZqnDocumentKind,
        metadata: ZqnDocumentMetadata,
        payload: ZqnDocumentPayload,
    ) -> Result<Self, SchemaError> {
        let document = Self {
            schema_id: ZQN_DOCUMENT_SCHEMA_ID.to_owned(),
            schema_version: ZQN_DOCUMENT_SCHEMA_VERSION,
            semantic_version: ZQN_DOCUMENT_SEMANTIC_VERSION,
            compatibility_version: ZQN_DOCUMENT_COMPATIBILITY_VERSION,
            document_kind,
            document_id,
            metadata,
            payload,
        };

        document.validate()?;

        Ok(document)
    }

    /// Returns the schema family identifier.
    #[must_use]
    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> ZqnSchemaVersion {
        self.schema_version
    }

    /// Returns the semantic version.
    #[must_use]
    pub const fn semantic_version(&self) -> ZqnVersion {
        self.semantic_version
    }

    /// Returns the compatibility version.
    #[must_use]
    pub const fn compatibility_version(
        &self,
    ) -> ZqnCompatibilityVersion {
        self.compatibility_version
    }

    /// Returns the document kind.
    #[must_use]
    pub const fn document_kind(&self) -> ZqnDocumentKind {
        self.document_kind
    }

    /// Returns the document identifier.
    #[must_use]
    pub fn document_id(&self) -> &ZqnDocumentId {
        &self.document_id
    }

    /// Returns the metadata.
    #[must_use]
    pub fn metadata(&self) -> &ZqnDocumentMetadata {
        &self.metadata
    }

    /// Returns the payload.
    #[must_use]
    pub fn payload(&self) -> &ZqnDocumentPayload {
        &self.payload
    }

    /// Returns mutable metadata.
    ///
    /// Callers must revalidate the document before persistence after mutation.
    #[must_use]
    pub fn metadata_mut(&mut self) -> &mut ZqnDocumentMetadata {
        &mut self.metadata
    }

    /// Returns mutable payload.
    ///
    /// Callers must revalidate the document before persistence after mutation.
    #[must_use]
    pub fn payload_mut(&mut self) -> &mut ZqnDocumentPayload {
        &mut self.payload
    }

    /// Validates this document against the current schema contract.
    pub fn validate(&self) -> Result<(), SchemaError> {
        self.validate_against(SchemaExpectation::current())
    }

    /// Validates this document against an explicit schema expectation.
    ///
    /// This is the preferred entry point for deserialization because the
    /// consumer may support a deliberately selected range of schemas.
    pub fn validate_against(
        &self,
        expectation: SchemaExpectation,
    ) -> Result<(), SchemaError> {
        validate_document(self, expectation)
    }
}

// =============================================================================
// Schema expectation
// =============================================================================

/// Explicit schema expectations for a consumer.
///
/// This prevents compatibility from being guessed from version numbers.
///
/// A consumer should construct an expectation from its supported contract and
/// pass it to document validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaExpectation {
    /// Expected schema family.
    pub schema_id: &'static str,

    /// Highest structural schema version understood by this consumer.
    pub schema_version: ZqnSchemaVersion,

    /// Compatibility contract available to this consumer.
    pub compatibility_version: ZqnCompatibilityVersion,

    /// Optional semantic-version requirement.
    pub semantic_requirement: Option<VersionRequirement>,
}

impl SchemaExpectation {
    /// Returns the expectation for this ZQN implementation.
    #[must_use]
    pub const fn current() -> Self {
        Self {
            schema_id: ZQN_DOCUMENT_SCHEMA_ID,
            schema_version: ZQN_DOCUMENT_SCHEMA_VERSION,
            compatibility_version: ZQN_DOCUMENT_COMPATIBILITY_VERSION,
            semantic_requirement: None,
        }
    }

    /// Requires a particular semantic version relationship.
    #[must_use]
    pub const fn with_semantic_requirement(
        mut self,
        requirement: VersionRequirement,
    ) -> Self {
        self.semantic_requirement = Some(requirement);
        self
    }
}

// =============================================================================
// Validation
// =============================================================================

/// Validates a complete ZQN document.
///
/// This function performs only schema-level validation.
///
/// It does NOT:
///
/// - allocate quantum-state representations;
/// - inspect channel matrices;
/// - validate probability distributions;
/// - validate physical calibration;
/// - validate QEC semantics;
/// - access hardware;
/// - access the filesystem;
/// - access the network.
pub fn validate_document(
    document: &ZqnDocument,
    expectation: SchemaExpectation,
) -> Result<(), SchemaError> {
    if document.schema_id.is_empty() {
        return Err(SchemaError::EmptySchemaId);
    }

    if document.schema_id != expectation.schema_id {
        return Err(SchemaError::SchemaIdMismatch {
            expected: expectation.schema_id.to_owned(),
            actual: document.schema_id.clone(),
        });
    }

    validate_schema_version(
        document.schema_version,
        expectation.schema_version,
    )?;

    if let Some(requirement) = expectation.semantic_requirement {
        if !requirement.matches(document.semantic_version) {
            return Err(SchemaError::SemanticVersionRequirementNotSatisfied {
                version: document.semantic_version,
                requirement,
            });
        }
    }

    validate_compatibility(
        document.compatibility_version,
        expectation.compatibility_version,
    )?;

    if document.document_id.as_str().is_empty() {
        return Err(SchemaError::EmptyDocumentId);
    }

    document.metadata.validate()?;

    validate_payload_shape(document.document_kind, &document.payload)?;

    Ok(())
}

/// Validates a schema version against a consumer's supported schema version.
///
/// Current policy:
///
/// ```text
/// same schema major
/// AND
/// document minor <= consumer minor
/// ```
///
/// The compatibility layer remains authoritative for more sophisticated
/// migration decisions.
pub fn validate_schema_version(
    actual: ZqnSchemaVersion,
    supported: ZqnSchemaVersion,
) -> Result<(), SchemaError> {
    if actual.compatible_with(supported) {
        Ok(())
    } else {
        Err(SchemaError::SchemaVersionMismatch {
            expected: supported,
            actual,
        })
    }
}

/// Validates a compatibility contract.
///
/// A producer can be consumed when the consumer's compatibility contract is
/// able to satisfy the producer's declared requirement.
pub fn validate_compatibility(
    producer: ZqnCompatibilityVersion,
    consumer: ZqnCompatibilityVersion,
) -> Result<(), SchemaError> {
    if consumer.satisfies(producer) {
        Ok(())
    } else {
        Err(SchemaError::CompatibilityMismatch {
            required: producer,
            supported: consumer,
        })
    }
}

/// Validates the shape of a payload at the schema level.
///
/// Domain semantics remain outside this function.
///
/// Object payloads are required for known structured ZQN document kinds.
/// `Extension` permits any JSON value because its semantics are intentionally
/// delegated to an extension owner.
pub fn validate_payload_shape(
    kind: ZqnDocumentKind,
    payload: &ZqnDocumentPayload,
) -> Result<(), SchemaError> {
    if matches!(kind, ZqnDocumentKind::Extension) {
        return Ok(());
    }

    if payload.is_null() {
        return Err(SchemaError::NullPayload { kind });
    }

    if !payload.is_object() {
        return Err(SchemaError::PayloadMustBeObject { kind });
    }

    Ok(())
}

// =============================================================================
// Schema envelope helpers
// =============================================================================

/// Builds the explicit top-level field representation used by generic
/// serialization implementations.
///
/// This helper exists so `serialization.rs` does not need to duplicate field
/// naming policy.
///
/// It does NOT produce bytes.
///
/// The returned object is a structured data model only.
pub fn document_to_value(
    document: &ZqnDocument,
) -> Result<Value, SchemaError> {
    document.validate()?;

    let mut root = Map::new();

    root.insert(
        FIELD_SCHEMA_ID.to_owned(),
        Value::String(document.schema_id.clone()),
    );

    root.insert(
        FIELD_SCHEMA_VERSION.to_owned(),
        Value::String(document.schema_version.to_string()),
    );

    root.insert(
        FIELD_SEMANTIC_VERSION.to_owned(),
        Value::String(document.semantic_version.to_string()),
    );

    root.insert(
        FIELD_COMPATIBILITY_VERSION.to_owned(),
        Value::String(document.compatibility_version.to_string()),
    );

    root.insert(
        FIELD_DOCUMENT_KIND.to_owned(),
        Value::String(document.document_kind.as_str().to_owned()),
    );

    root.insert(
        FIELD_DOCUMENT_ID.to_owned(),
        Value::String(document.document_id.as_str().to_owned()),
    );

    let metadata = serde_json::to_value(&document.metadata)
        .map_err(SchemaError::MetadataEncoding)?;

    root.insert(FIELD_METADATA.to_owned(), metadata);

    root.insert(
        FIELD_PAYLOAD.to_owned(),
        document.payload.as_value().clone(),
    );

    Ok(Value::Object(root))
}

// =============================================================================
// Schema-level parsing helpers
// =============================================================================

/// Parses a document-kind identifier.
///
/// Unknown identifiers are rejected at this layer.
///
/// Forward-compatible extension document kinds should use `extension` until a
/// concrete enum variant is added under an explicit schema-version change.
pub fn parse_document_kind(
    value: &str,
) -> Result<ZqnDocumentKind, SchemaError> {
    match value {
        "noise_model" => Ok(ZqnDocumentKind::NoiseModel),
        "channel" => Ok(ZqnDocumentKind::Channel),
        "fault_model" => Ok(ZqnDocumentKind::FaultModel),
        "probability_model" => Ok(ZqnDocumentKind::ProbabilityModel),
        "calibration" => Ok(ZqnDocumentKind::Calibration),
        "characterization" => Ok(ZqnDocumentKind::Characterization),
        "simulation" => Ok(ZqnDocumentKind::Simulation),
        "propagation" => Ok(ZqnDocumentKind::Propagation),
        "target" => Ok(ZqnDocumentKind::Target),
        "observation" => Ok(ZqnDocumentKind::Observation),
        "extension" => Ok(ZqnDocumentKind::Extension),
        _ => Err(SchemaError::UnknownDocumentKind {
            value: value.to_owned(),
        }),
    }
}

impl FromStr for ZqnDocumentKind {
    type Err = SchemaError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_document_kind(value)
    }
}

// =============================================================================
// Version parsing façade
// =============================================================================

/// Parses a semantic ZQN version through the authoritative core version
/// implementation.
///
/// This façade exists so deserialization code can depend on the schema module
/// without duplicating parsing policy.
pub fn parse_semantic_version(
    value: &str,
) -> Result<ZqnVersion, SchemaError> {
    value
        .parse::<ZqnVersion>()
        .map_err(SchemaError::Version)
}

/// Parses a persisted schema version through the authoritative core version
/// implementation.
pub fn parse_schema_version(
    value: &str,
) -> Result<ZqnSchemaVersion, SchemaError> {
    value
        .parse::<ZqnSchemaVersion>()
        .map_err(SchemaError::Version)
}

/// Parses a compatibility version through the authoritative core version
/// implementation.
pub fn parse_compatibility_version(
    value: &str,
) -> Result<ZqnCompatibilityVersion, SchemaError> {
    value
        .parse::<ZqnCompatibilityVersion>()
        .map_err(SchemaError::Version)
}

// =============================================================================
// Metadata validation
// =============================================================================

fn validate_metadata_keys(
    values: &Map<String, Value>,
    field: &'static str,
) -> Result<(), SchemaError> {
    for key in values.keys() {
        if key.is_empty() {
            return Err(SchemaError::EmptyMetadataKey { field });
        }
    }

    Ok(())
}

// =============================================================================
// Schema errors
// =============================================================================

/// Schema-layer error.
///
/// This type contains only errors owned by the schema contract.
///
/// Transport-specific, domain-specific and runtime-specific errors must be
/// represented by their respective layers.
#[derive(Debug)]
pub enum SchemaError {
    /// The schema identifier was empty.
    EmptySchemaId,

    /// The document identifier was empty.
    EmptyDocumentId,

    /// A required metadata string was empty.
    EmptyMetadataField {
        /// Metadata field name.
        field: &'static str,
    },

    /// A metadata key was empty.
    EmptyMetadataKey {
        /// Metadata collection containing the invalid key.
        field: &'static str,
    },

    /// The document uses another schema family.
    SchemaIdMismatch {
        /// Expected schema identifier.
        expected: String,

        /// Actual schema identifier.
        actual: String,
    },

    /// The document schema version is not understood.
    SchemaVersionMismatch {
        /// Highest supported version.
        expected: ZqnSchemaVersion,

        /// Actual document version.
        actual: ZqnSchemaVersion,
    },

    /// The producer's compatibility contract is not supported.
    CompatibilityMismatch {
        /// Required producer compatibility contract.
        required: ZqnCompatibilityVersion,

        /// Consumer compatibility contract.
        supported: ZqnCompatibilityVersion,
    },

    /// The semantic version does not satisfy a consumer requirement.
    SemanticVersionRequirementNotSatisfied {
        /// Actual semantic version.
        version: ZqnVersion,

        /// Required relationship.
        requirement: VersionRequirement,
    },

    /// The document kind is unknown.
    UnknownDocumentKind {
        /// Serialized document-kind value.
        value: String,
    },

    /// A known document kind has a null payload.
    NullPayload {
        /// Document kind.
        kind: ZqnDocumentKind,
    },

    /// A known document kind requires an object payload.
    PayloadMustBeObject {
        /// Document kind.
        kind: ZqnDocumentKind,
    },

    /// A version could not be parsed.
    Version(VersionError),

    /// Metadata could not be represented as a generic structured value.
    MetadataEncoding(serde_json::Error),
}

impl fmt::Display for SchemaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySchemaId => {
                formatter.write_str("ZQN schema identifier must not be empty")
            }

            Self::EmptyDocumentId => {
                formatter.write_str("ZQN document identifier must not be empty")
            }

            Self::EmptyMetadataField { field } => {
                write!(formatter, "ZQN metadata field `{field}` must not be empty")
            }

            Self::EmptyMetadataKey { field } => {
                write!(formatter, "ZQN metadata `{field}` contains an empty key")
            }

            Self::SchemaIdMismatch { expected, actual } => {
                write!(
                    formatter,
                    "ZQN schema identifier mismatch: expected `{expected}`, \
                     found `{actual}`"
                )
            }

            Self::SchemaVersionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "ZQN schema version mismatch: supported `{expected}`, \
                     document `{actual}`"
                )
            }

            Self::CompatibilityMismatch {
                required,
                supported,
            } => {
                write!(
                    formatter,
                    "ZQN compatibility mismatch: document requires `{required}`, \
                     consumer supports `{supported}`"
                )
            }

            Self::SemanticVersionRequirementNotSatisfied {
                version,
                requirement,
            } => {
                write!(
                    formatter,
                    "ZQN semantic version `{version}` does not satisfy \
                     requirement `{requirement:?}`"
                )
            }

            Self::UnknownDocumentKind { value } => {
                write!(
                    formatter,
                    "unknown ZQN document kind `{value}`"
                )
            }

            Self::NullPayload { kind } => {
                write!(
                    formatter,
                    "ZQN `{kind}` document payload must not be null"
                )
            }

            Self::PayloadMustBeObject { kind } => {
                write!(
                    formatter,
                    "ZQN `{kind}` document payload must be an object"
                )
            }

            Self::Version(error) => {
                write!(formatter, "invalid ZQN version: {error}")
            }

            Self::MetadataEncoding(error) => {
                write!(
                    formatter,
                    "failed to construct structured ZQN metadata: {error}"
                )
            }
        }
    }
}

impl std::error::Error for SchemaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Version(error) => Some(error),
            Self::MetadataEncoding(error) => Some(error),
            _ => None,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn document() -> ZqnDocument {
        let id = ZqnDocumentId::new("test-document")
            .expect("test identifier must be valid");

        let mut metadata = ZqnDocumentMetadata::new();

        metadata.insert_label(
            "purpose",
            Value::String("schema-test".to_owned()),
        );

        ZqnDocument::new(
            id,
            ZqnDocumentKind::NoiseModel,
            metadata,
            ZqnDocumentPayload::new(serde_json::json!({
                "model": "test"
            })),
        )
        .expect("test document must be valid")
    }

    #[test]
    fn schema_identifier_is_stable() {
        assert_eq!(
            ZQN_DOCUMENT_SCHEMA_ID,
            "zamani.quantum.zqn"
        );
    }

    #[test]
    fn current_document_uses_current_schema() {
        let value = document();

        assert_eq!(
            value.schema_id,
            ZQN_DOCUMENT_SCHEMA_ID
        );

        assert_eq!(
            value.schema_version,
            ZQN_DOCUMENT_SCHEMA_VERSION
        );

        assert_eq!(
            value.semantic_version,
            ZQN_DOCUMENT_SEMANTIC_VERSION
        );

        assert_eq!(
            value.compatibility_version,
            ZQN_DOCUMENT_COMPATIBILITY_VERSION
        );
    }

    #[test]
    fn document_kind_has_stable_identifier() {
        assert_eq!(
            ZqnDocumentKind::NoiseModel.as_str(),
            "noise_model"
        );

        assert_eq!(
            ZqnDocumentKind::Channel.as_str(),
            "channel"
        );

        assert_eq!(
            ZqnDocumentKind::Extension.as_str(),
            "extension"
        );
    }

    #[test]
    fn document_kind_round_trips_through_parser() {
        for kind in [
            ZqnDocumentKind::NoiseModel,
            ZqnDocumentKind::Channel,
            ZqnDocumentKind::FaultModel,
            ZqnDocumentKind::ProbabilityModel,
            ZqnDocumentKind::Calibration,
            ZqnDocumentKind::Characterization,
            ZqnDocumentKind::Simulation,
            ZqnDocumentKind::Propagation,
            ZqnDocumentKind::Target,
            ZqnDocumentKind::Observation,
            ZqnDocumentKind::Extension,
        ] {
            let parsed = kind
                .as_str()
                .parse::<ZqnDocumentKind>()
                .expect("stable kind must parse");

            assert_eq!(parsed, kind);
        }
    }

    #[test]
    fn empty_document_id_is_rejected() {
        let result = ZqnDocumentId::new("");

        assert!(matches!(
            result,
            Err(SchemaError::EmptyDocumentId)
        ));
    }

    #[test]
    fn empty_schema_id_is_rejected() {
        let mut value = document();

        value.schema_id.clear();

        let result = value.validate();

        assert!(matches!(
            result,
            Err(SchemaError::EmptySchemaId)
        ));
    }

    #[test]
    fn wrong_schema_id_is_rejected() {
        let mut value = document();

        value.schema_id = "another.schema".to_owned();

        let result = value.validate();

        assert!(matches!(
            result,
            Err(SchemaError::SchemaIdMismatch { .. })
        ));
    }

    #[test]
    fn incompatible_schema_version_is_rejected() {
        let mut value = document();

        value.schema_version = ZqnSchemaVersion::new(
            value.schema_version.major().saturating_add(1),
            0,
        );

        let result = value.validate();

        assert!(matches!(
            result,
            Err(SchemaError::SchemaVersionMismatch { .. })
        ));
    }

    #[test]
    fn compatible_lower_minor_schema_is_accepted() {
        let mut value = document();

        value.schema_version = ZqnSchemaVersion::new(
            ZQN_DOCUMENT_SCHEMA_VERSION.major(),
            0,
        );

        assert!(value.validate().is_ok());
    }

    #[test]
    fn incompatible_compatibility_major_is_rejected() {
        let mut value = document();

        value.compatibility_version =
            ZqnCompatibilityVersion::new(
                value.compatibility_version.major().saturating_add(1),
                0,
            );

        let result = value.validate();

        assert!(matches!(
            result,
            Err(SchemaError::CompatibilityMismatch { .. })
        ));
    }

    #[test]
    fn null_known_payload_is_rejected() {
        let result = validate_payload_shape(
            ZqnDocumentKind::Channel,
            &ZqnDocumentPayload::new(Value::Null),
        );

        assert!(matches!(
            result,
            Err(SchemaError::NullPayload {
                kind: ZqnDocumentKind::Channel
            })
        ));
    }

    #[test]
    fn scalar_known_payload_is_rejected() {
        let result = validate_payload_shape(
            ZqnDocumentKind::Channel,
            &ZqnDocumentPayload::new(Value::String(
                "invalid".to_owned(),
            )),
        );

        assert!(matches!(
            result,
            Err(SchemaError::PayloadMustBeObject {
                kind: ZqnDocumentKind::Channel
            })
        ));
    }

    #[test]
    fn extension_payload_can_have_arbitrary_shape() {
        assert!(
            validate_payload_shape(
                ZqnDocumentKind::Extension,
                &ZqnDocumentPayload::new(
                    Value::String("extension".to_owned())
                ),
            )
            .is_ok()
        );

        assert!(
            validate_payload_shape(
                ZqnDocumentKind::Extension,
                &ZqnDocumentPayload::new(Value::Null),
            )
            .is_ok()
        );
    }

    #[test]
    fn metadata_empty_state_is_valid() {
        let metadata = ZqnDocumentMetadata::new();

        assert!(metadata.is_empty());
        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn empty_metadata_key_is_rejected() {
        let mut metadata = ZqnDocumentMetadata::new();

        metadata.insert_label("", Value::Null);

        assert!(matches!(
            metadata.validate(),
            Err(SchemaError::EmptyMetadataKey {
                field: FIELD_METADATA_LABELS
            })
        ));
    }

    #[test]
    fn metadata_extensions_are_preserved() {
        let mut metadata = ZqnDocumentMetadata::new();

        metadata.insert_extension(
            "future_field",
            serde_json::json!({
                "version": 2
            }),
        );

        let value = serde_json::to_value(&metadata)
            .expect("metadata should serialize");

        assert_eq!(
            value["extensions"]["future_field"]["version"],
            Value::from(2)
        );
    }

    #[test]
    fn document_to_value_contains_required_envelope_fields() {
        let value = document_to_value(&document())
            .expect("document must convert to structured value");

        let object = value
            .as_object()
            .expect("document must be an object");

        assert!(object.contains_key(FIELD_SCHEMA_ID));
        assert!(object.contains_key(FIELD_SCHEMA_VERSION));
        assert!(object.contains_key(FIELD_SEMANTIC_VERSION));
        assert!(object.contains_key(FIELD_COMPATIBILITY_VERSION));
        assert!(object.contains_key(FIELD_DOCUMENT_KIND));
        assert!(object.contains_key(FIELD_DOCUMENT_ID));
        assert!(object.contains_key(FIELD_METADATA));
        assert!(object.contains_key(FIELD_PAYLOAD));
    }

    #[test]
    fn document_to_value_preserves_payload() {
        let value = document_to_value(&document())
            .expect("document must convert to structured value");

        assert_eq!(
            value["payload"]["model"],
            Value::String("test".to_owned())
        );
    }

    #[test]
    fn schema_expectation_is_current() {
        let expectation = SchemaExpectation::current();

        assert_eq!(
            expectation.schema_id,
            ZQN_DOCUMENT_SCHEMA_ID
        );

        assert_eq!(
            expectation.schema_version,
            ZQN_DOCUMENT_SCHEMA_VERSION
        );

        assert_eq!(
            expectation.compatibility_version,
            ZQN_DOCUMENT_COMPATIBILITY_VERSION
        );
    }

    #[test]
    fn semantic_requirement_can_be_applied() {
        let expectation =
            SchemaExpectation::current().with_semantic_requirement(
                VersionRequirement::AtLeast(ZqnVersion::new(
                    1, 0, 0,
                )),
            );

        assert!(
            document()
                .validate_against(expectation)
                .is_ok()
        );
    }

    #[test]
    fn unknown_document_kind_is_rejected() {
        let result =
            "future_unknown_kind".parse::<ZqnDocumentKind>();

        assert!(matches!(
            result,
            Err(SchemaError::UnknownDocumentKind { .. })
        ));
    }

    #[test]
    fn document_identity_is_not_quantum_resource_identity() {
        let id = ZqnDocumentId::new("document-1")
            .expect("identifier must be valid");

        assert_eq!(id.as_str(), "document-1");
    }

    #[test]
    fn validation_is_repeatable() {
        let value = document();

        for _ in 0..100 {
            assert!(value.validate().is_ok());
        }
    }

    #[test]
    fn no_machine_size_is_encoded_in_schema() {
        let value = document();

        // The schema contract validates the envelope without inspecting or
        // imposing any quantity on the semantic payload.
        assert!(value.validate().is_ok());
    }

    #[test]
    fn schema_version_string_round_trips() {
        let version = ZQN_DOCUMENT_SCHEMA_VERSION;

        let encoded = version.to_string();

        let decoded = parse_schema_version(&encoded)
            .expect("schema version must parse");

        assert_eq!(decoded, version);
    }

    #[test]
    fn semantic_version_string_round_trips() {
        let version = ZQN_DOCUMENT_SEMANTIC_VERSION;

        let encoded = version.to_string();

        let decoded = parse_semantic_version(&encoded)
            .expect("semantic version must parse");

        assert_eq!(decoded, version);
    }

    #[test]
    fn compatibility_version_string_round_trips() {
        let version = ZQN_DOCUMENT_COMPATIBILITY_VERSION;

        let encoded = version.to_string();

        let decoded = parse_compatibility_version(&encoded)
            .expect("compatibility version must parse");

        assert_eq!(decoded, version);
    }

    #[test]
    fn malformed_schema_version_returns_error() {
        assert!(
            parse_schema_version("not-a-version").is_err()
        );
    }

    #[test]
    fn malformed_semantic_version_returns_error() {
        assert!(
            parse_semantic_version("not-a-version").is_err()
        );
    }

    #[test]
    fn malformed_compatibility_version_returns_error() {
        assert!(
            parse_compatibility_version("not-a-version").is_err()
        );
    }
}