//! Zamani Quantum Resilience — Serialization Schema Contract.
//!
//! Path:
//!     src/quantum/resilience/serialization/schema.rs
//!
//! # Purpose
//!
//! This module defines the stable, implementation-independent schema contract
//! for serialized quantum-resilience data.
//!
//! It does NOT:
//!
//! - encode bytes;
//! - decode bytes;
//! - define a wire-format implementation;
//! - depend on serde;
//! - depend on a particular serializer;
//! - duplicate resilience domain models;
//! - duplicate ZQN fault semantics;
//! - define logical or physical qubit identifiers;
//! - access hardware;
//! - access the filesystem;
//! - access the network;
//! - access a clock;
//! - use randomness;
//! - perform migration;
//! - perform recovery.
//!
//! Those responsibilities belong to the corresponding serialization and
//! resilience modules.
//!
//! # Architectural position
//!
//! ```text
//!                         canonical domain objects
//!                                  |
//!          +-----------------------+-----------------------+
//!          |                       |                       |
//!          v                       v                       v
//!       resilience               ZQN                     IR
//!          |                       |                       |
//!          +-----------------------+-----------------------+
//!                                  |
//!                                  v
//!                  serialization::schema
//!                                  |
//!                  +---------------+---------------+
//!                  |                               |
//!                  v                               v
//!             serialization::encode        serialization::decode
//!                  |                               |
//!                  v                               v
//!               bytes/storage/transport        domain objects
//! ```
//!
//! `schema.rs` defines *what* a serialized resilience document means.
//!
//! `encode.rs` defines *how* that schema is encoded.
//!
//! `decode.rs` defines *how* encoded data becomes schema/domain values.
//!
//! `version.rs` defines schema-version compatibility and migration policy.
//!
//! # Integration rule
//!
//! Consumers MUST depend on the types in this module rather than inventing
//! serializer-specific schema identifiers.
//!
//! In particular:
//!
//! ```text
//! encode.rs  ---> schema.rs
//! decode.rs  ---> schema.rs
//! version.rs ---> schema.rs
//!
//! model/*        -X-> schema implementation details
//! detection/*    -X-> schema implementation details
//! recovery/*     -X-> schema implementation details
//! planning/*     -X-> schema implementation details
//! ```
//!
//! Domain modules remain semantic. Serialization remains a boundary concern.
//!
//! # Canonical identity rule
//!
//! Serialized quantum resource identities MUST preserve their domain.
//!
//! The canonical logical and physical identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This schema module deliberately does not import or redefine either type.
//!
//! A serialized representation MUST NOT turn:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! into an interchangeable untyped integer merely because an implementation
//! happens to represent them numerically.
//!
//! Encoding/decoding adapters are responsible for preserving the identity
//! domain when converting between domain values and serialized values.
//!
//! # ZQN integration
//!
//! Canonical physical/noise/fault semantics remain owned by ZQN.
//!
//! Serialization MUST preserve the canonical ZQN fault identity and semantic
//! information required by the owning ZQN schema.
//!
//! This module does not recreate `Fault`, `FaultLocation`, fault
//! classifications, or other ZQN semantic structures.
//!
//! # Write once, scale everywhere
//!
//! This schema contains no machine-size constants.
//!
//! It MUST NOT contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_FAULTS
//! MAX_OPERATIONS
//! MAX_BACKENDS
//! MAX_INCIDENTS
//! MAX_EVENTS
//! ```
//!
//! Collection sizes are represented using schema-neutral lengths and are
//! constrained by the caller's resource/limits policy.
//!
//! Therefore the schema imposes no artificial quantum-machine-size ceiling.
//!
//! Actual finite execution remains bounded by:
//!
//! - available memory;
//! - available storage;
//! - execution resources;
//! - configured resource policy;
//! - transport limits;
//! - target hardware capability;
//! - distributed capacity.
//!
//! # Determinism
//!
//! Schema identity and versioning are deterministic.
//!
//! This module MUST NOT derive identity from:
//!
//! - memory addresses;
//! - process identifiers;
//! - system time;
//! - thread identifiers;
//! - random numbers;
//! - hash-map iteration order.
//!
//! Deterministic encoding order is the responsibility of `encode.rs`, but
//! `schema.rs` provides the stable schema identifiers required to make that
//! encoding reproducible.
//!
//! # Security
//!
//! Schema metadata is descriptive data, not authorization.
//!
//! A schema identifier MUST NOT grant:
//!
//! - backend access;
//! - QPU access;
//! - credentials;
//! - filesystem access;
//! - network access;
//! - recovery authority;
//! - execution authority.
//!
//! Untrusted serialized input MUST still be subject to:
//!
//! - resource limits;
//! - structural validation;
//! - schema compatibility checks;
//! - semantic validation;
//! - integrity verification where applicable.
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
//! - no unsafe Rust.
//!
//! This file deliberately uses only stable standard/core facilities.
//!
//! # No unsafe
//!
//! Unsafe Rust is forbidden.
//!
//! The explicit module-level lint below makes accidental introduction of
//! unsafe code a compilation failure.
//!
//! =============================================================================
//! Schema identity
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

/// The namespace owned by this serialization schema.
///
/// The value is intentionally static and immutable because the namespace is
/// part of the persistent compatibility contract.
pub const SCHEMA_NAMESPACE: &str = "zamani.quantum.resilience";

/// The current resilience serialization schema major version.
///
/// Major-version changes indicate an incompatible schema change that cannot be
/// consumed by an implementation supporting only the previous major version.
pub const CURRENT_SCHEMA_MAJOR: u16 = 1;

/// The current resilience serialization schema minor version.
///
/// Minor-version changes represent backward-compatible additions or
/// clarifications under the compatibility rules documented by this module.
pub const CURRENT_SCHEMA_MINOR: u16 = 0;

/// The current resilience serialization schema patch version.
///
/// Patch changes represent corrections that do not alter the semantic schema
/// contract.
pub const CURRENT_SCHEMA_PATCH: u16 = 0;

/// The complete current schema version.
pub const CURRENT_SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(
    CURRENT_SCHEMA_MAJOR,
    CURRENT_SCHEMA_MINOR,
    CURRENT_SCHEMA_PATCH,
);

/// Identifies the schema family for serialized resilience data.
///
/// The namespace is deliberately not a Rust module path. Persisted data must
/// remain understandable if the internal Rust module hierarchy changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaNamespace;

impl SchemaNamespace {
    /// Returns the canonical persisted namespace.
    #[must_use]
    pub const fn as_str() -> &'static str {
        SCHEMA_NAMESPACE
    }
}

impl fmt::Display for SchemaNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(Self::as_str())
    }
}

/// The stable namespace value.
pub const RESILIENCE_SCHEMA_NAMESPACE: SchemaNamespace = SchemaNamespace;

// =============================================================================
// Schema version
// =============================================================================

/// A semantic version of the resilience serialization schema.
///
/// Version numbers describe the persisted schema, not the Rust crate version,
/// compiler version, hardware version, backend version, or Zamani language
/// version.
///
/// # Compatibility semantics
///
/// ```text
/// major
///     incompatible semantic/schema change
///
/// minor
///     backward-compatible schema addition/change
///
/// patch
///     non-semantic correction/clarification
/// ```
///
/// Compatibility with a particular decoder is determined by `version.rs`.
/// This type intentionally does not contain migration logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SchemaVersion {
    /// Creates a schema version.
    ///
    /// The components are plain semantic version components. This constructor
    /// performs no implicit compatibility assumptions.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }

    /// Returns the current schema version.
    #[must_use]
    pub const fn current() -> Self {
        CURRENT_SCHEMA_VERSION
    }

    /// Returns whether this version belongs to the same major schema family.
    ///
    /// This is only a coarse compatibility property. A decoder MUST use the
    /// complete compatibility rules from `serialization::version`.
    #[must_use]
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns whether this version is exactly equal to another version.
    #[must_use]
    pub const fn is_exact(self, other: Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }
}

impl Default for SchemaVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

// =============================================================================
// Document type
// =============================================================================

/// Identifies the kind of resilience object represented by a serialized
/// document.
///
/// This is intentionally a closed semantic enumeration for the schema's
/// first-generation document families.
///
/// New document families MUST be added deliberately as schema changes rather
/// than inferred from arbitrary user strings.
///
/// The variants describe persistence boundaries, not implementation types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum DocumentKind {
    /// A complete resilience execution context/state snapshot.
    ExecutionState = 1,

    /// A resilience incident.
    Incident = 2,

    /// A recovery plan.
    RecoveryPlan = 3,

    /// A recovery state snapshot.
    RecoveryState = 4,

    /// A checkpoint descriptor.
    Checkpoint = 5,

    /// A checkpoint manifest.
    CheckpointManifest = 6,

    /// A telemetry event.
    TelemetryEvent = 7,

    /// A telemetry metric.
    TelemetryMetric = 8,

    /// A telemetry trace.
    TelemetryTrace = 9,

    /// A health observation/state.
    HealthState = 10,

    /// A serialized resilience policy.
    Policy = 11,

    /// A serialized resilience capability snapshot.
    CapabilitySnapshot = 12,

    /// A serialized diagnosis.
    Diagnosis = 13,

    /// A serialized execution result/verification record.
    VerificationResult = 14,

    /// A serialized provenance record.
    Provenance = 15,

    /// A serialized resilience history record.
    HistoryRecord = 16,

    /// A serialized learning/feedback record.
    LearningFeedback = 17,
}

impl DocumentKind {
    /// Returns the stable schema identifier.
    #[must_use]
    pub const fn code(self) -> u16 {
        self as u16
    }

    /// Returns the canonical persisted name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionState => "execution_state",
            Self::Incident => "incident",
            Self::RecoveryPlan => "recovery_plan",
            Self::RecoveryState => "recovery_state",
            Self::Checkpoint => "checkpoint",
            Self::CheckpointManifest => "checkpoint_manifest",
            Self::TelemetryEvent => "telemetry_event",
            Self::TelemetryMetric => "telemetry_metric",
            Self::TelemetryTrace => "telemetry_trace",
            Self::HealthState => "health_state",
            Self::Policy => "policy",
            Self::CapabilitySnapshot => "capability_snapshot",
            Self::Diagnosis => "diagnosis",
            Self::VerificationResult => "verification_result",
            Self::Provenance => "provenance",
            Self::HistoryRecord => "history_record",
            Self::LearningFeedback => "learning_feedback",
        }
    }
}

impl fmt::Display for DocumentKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Schema identifier
// =============================================================================

/// Fully qualified identity of a serialized resilience document.
///
/// This value separates:
///
/// - schema namespace;
/// - document kind;
/// - schema version.
///
/// It is suitable for use by `encode.rs`, `decode.rs`, and `version.rs`.
///
/// It does not contain:
///
/// - backend names;
/// - device identifiers;
/// - qubit counts;
/// - provider identifiers;
/// - credentials;
/// - memory addresses;
/// - timestamps.
///
/// Those belong to domain/provenance data when semantically required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaId {
    namespace: SchemaNamespace,
    kind: DocumentKind,
    version: SchemaVersion,
}

impl SchemaId {
    /// Creates a schema identifier.
    #[must_use]
    pub const fn new(kind: DocumentKind, version: SchemaVersion) -> Self {
        Self {
            namespace: RESILIENCE_SCHEMA_NAMESPACE,
            kind,
            version,
        }
    }

    /// Creates an identifier using the current schema version.
    #[must_use]
    pub const fn current(kind: DocumentKind) -> Self {
        Self::new(kind, CURRENT_SCHEMA_VERSION)
    }

    /// Returns the namespace.
    #[must_use]
    pub const fn namespace(self) -> SchemaNamespace {
        self.namespace
    }

    /// Returns the document kind.
    #[must_use]
    pub const fn kind(self) -> DocumentKind {
        self.kind
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn version(self) -> SchemaVersion {
        self.version
    }
}

impl fmt::Display for SchemaId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}@{}",
            self.namespace,
            self.kind,
            self.version
        )
    }
}

// =============================================================================
// Schema feature flags
// =============================================================================

/// Optional capabilities represented by a serialized schema.
///
/// Feature flags are intentionally independent of implementation versions.
///
/// A decoder must never assume that the presence of a flag grants the
/// corresponding execution capability. It only means that the serialized
/// document may contain the associated data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SchemaFeatures(u64);

impl SchemaFeatures {
    /// No optional features.
    pub const NONE: Self = Self(0);

    /// The serialized document may contain canonical ZQN fault data.
    pub const ZQN_FAULTS: Self = Self(1 << 0);

    /// The serialized document may contain canonical logical-qubit identity
    /// information.
    pub const LOGICAL_QUBITS: Self = Self(1 << 1);

    /// The serialized document may contain canonical physical-qubit identity
    /// information.
    pub const PHYSICAL_QUBITS: Self = Self(1 << 2);

    /// The serialized document may contain topology/resource mappings.
    pub const RESOURCE_MAPPING: Self = Self(1 << 3);

    /// The serialized document may contain QEC-related state.
    pub const QEC_STATE: Self = Self(1 << 4);

    /// The serialized document may contain mitigation information.
    pub const MITIGATION: Self = Self(1 << 5);

    /// The serialized document may contain verification/provenance data.
    pub const VERIFICATION: Self = Self(1 << 6);

    /// The serialized document may contain checkpoint metadata.
    pub const CHECKPOINTS: Self = Self(1 << 7);

    /// The serialized document may contain telemetry.
    pub const TELEMETRY: Self = Self(1 << 8);

    /// The serialized document may contain distributed coordination state.
    pub const DISTRIBUTED: Self = Self(1 << 9);

    /// The serialized document may contain learning/feedback data.
    pub const LEARNING: Self = Self(1 << 10);

    /// Returns the raw feature bitset.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Creates a feature set from raw bits.
    ///
    /// Unknown bits are preserved. This is important for forward-compatible
    /// transport by components that do not understand every future feature.
    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns whether all bits in `other` are present.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns the union of two feature sets.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns whether no features are set.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

// =============================================================================
// Schema flags
// =============================================================================

/// Structural properties of a serialized document.
///
/// These flags describe the document contract and are not execution
/// instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SchemaFlags(u32);

impl SchemaFlags {
    /// No flags.
    pub const NONE: Self = Self(0);

    /// The document contains a deterministic canonical representation.
    pub const DETERMINISTIC: Self = Self(1 << 0);

    /// The document may be safely persisted once integrity requirements have
    /// been satisfied by the surrounding storage layer.
    pub const PERSISTABLE: Self = Self(1 << 1);

    /// The document contains enough metadata for schema-level compatibility
    /// evaluation.
    pub const SELF_DESCRIBING: Self = Self(1 << 2);

    /// The document may contain data introduced by a newer minor schema.
    pub const FORWARD_EXTENSIBLE: Self = Self(1 << 3);

    /// The document includes integrity metadata.
    ///
    /// The flag does NOT define a cryptographic algorithm. Cryptographic
    /// integrity belongs to the surrounding integrity/security layer.
    pub const HAS_INTEGRITY_METADATA: Self = Self(1 << 4);

    /// Returns raw flag bits.
    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Creates flags from raw bits.
    ///
    /// Unknown bits are preserved for forward compatibility.
    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Returns whether all requested flags are present.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns the union of two flag sets.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns whether no flags are set.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

// =============================================================================
// Schema header
// =============================================================================

/// Stable metadata that must precede a serialized resilience document.
///
/// The header intentionally contains only schema metadata.
///
/// Domain payloads are handled by the individual document serializers.
///
/// # Required properties
///
/// A valid encoded document must identify:
///
/// - schema namespace;
/// - document kind;
/// - schema version;
/// - feature set;
/// - schema flags.
///
/// The actual byte representation is owned by `encode.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaHeader {
    schema: SchemaId,
    features: SchemaFeatures,
    flags: SchemaFlags,
}

impl SchemaHeader {
    /// Creates a schema header.
    #[must_use]
    pub const fn new(
        schema: SchemaId,
        features: SchemaFeatures,
        flags: SchemaFlags,
    ) -> Self {
        Self {
            schema,
            features,
            flags,
        }
    }

    /// Creates a header for the current schema.
    #[must_use]
    pub const fn current(
        kind: DocumentKind,
        features: SchemaFeatures,
        flags: SchemaFlags,
    ) -> Self {
        Self::new(SchemaId::current(kind), features, flags)
    }

    /// Returns the schema identity.
    #[must_use]
    pub const fn schema(self) -> SchemaId {
        self.schema
    }

    /// Returns the schema namespace.
    #[must_use]
    pub const fn namespace(self) -> SchemaNamespace {
        self.schema.namespace()
    }

    /// Returns the document kind.
    #[must_use]
    pub const fn kind(self) -> DocumentKind {
        self.schema.kind()
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn version(self) -> SchemaVersion {
        self.schema.version()
    }

    /// Returns optional feature flags.
    #[must_use]
    pub const fn features(self) -> SchemaFeatures {
        self.features
    }

    /// Returns structural schema flags.
    #[must_use]
    pub const fn flags(self) -> SchemaFlags {
        self.flags
    }

    /// Returns whether this document declares deterministic representation.
    #[must_use]
    pub const fn is_deterministic(self) -> bool {
        self.flags.contains(SchemaFlags::DETERMINISTIC)
    }

    /// Returns whether this document declares forward extensibility.
    #[must_use]
    pub const fn is_forward_extensible(self) -> bool {
        self.flags.contains(SchemaFlags::FORWARD_EXTENSIBLE)
    }
}

// =============================================================================
// Field identifiers
// =============================================================================

/// Stable field identifier for schema-level metadata.
///
/// These IDs are deliberately separate from Rust field names.
///
/// Rust field names may change during refactoring without necessarily changing
/// the persisted schema.
///
/// New fields must receive new identifiers rather than reusing identifiers of
/// removed fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum SchemaField {
    /// Schema namespace.
    Namespace = 1,

    /// Document kind.
    DocumentKind = 2,

    /// Schema version.
    SchemaVersion = 3,

    /// Optional feature flags.
    Features = 4,

    /// Structural schema flags.
    Flags = 5,

    /// Payload.
    Payload = 6,

    /// Integrity metadata.
    Integrity = 7,

    /// Encoding identifier.
    Encoding = 8,

    /// Content length.
    ContentLength = 9,

    /// Extension fields.
    Extensions = 10,
}

impl SchemaField {
    /// Returns the stable numeric field identifier.
    #[must_use]
    pub const fn code(self) -> u16 {
        self as u16
    }

    /// Returns the canonical persisted field name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Namespace => "namespace",
            Self::DocumentKind => "document_kind",
            Self::SchemaVersion => "schema_version",
            Self::Features => "features",
            Self::Flags => "flags",
            Self::Payload => "payload",
            Self::Integrity => "integrity",
            Self::Encoding => "encoding",
            Self::ContentLength => "content_length",
            Self::Extensions => "extensions",
        }
    }
}

impl fmt::Display for SchemaField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Encoding identity
// =============================================================================

/// Identifies the representation encoding used for a serialized document.
///
/// The schema contract remains independent from the actual encoder.
///
/// A future implementation may add another encoding without changing the
/// semantic schema merely because the bytes are represented differently.
///
/// Encoding identifiers are therefore transport concerns, not quantum
/// semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum EncodingKind {
    /// Canonical binary encoding.
    Binary = 1,

    /// Canonical textual encoding.
    Text = 2,

    /// Canonical JSON-compatible representation.
    Json = 3,

    /// Canonical CBOR-compatible representation.
    Cbor = 4,
}

impl EncodingKind {
    /// Returns the stable numeric encoding identifier.
    #[must_use]
    pub const fn code(self) -> u16 {
        self as u16
    }

    /// Returns the canonical persisted name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Text => "text",
            Self::Json => "json",
            Self::Cbor => "cbor",
        }
    }
}

impl fmt::Display for EncodingKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Compatibility classification
// =============================================================================

/// Result of schema-level compatibility evaluation.
///
/// This enum intentionally mirrors the resilience compatibility vocabulary
/// defined by `COMPATIBILITY.md` while remaining local to serialization.
///
/// It describes compatibility, not an execution/recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum SchemaCompatibility {
    /// The schema can be consumed without transformation.
    Compatible = 0,

    /// The schema can be consumed after an allowed migration/adaptation.
    CompatibleWithMigration = 1,

    /// The schema can be consumed but contains optional data that the reader
    /// does not fully interpret.
    CompatibleWithDegradation = 2,

    /// Compatibility depends on runtime/policy information outside the schema.
    ConditionallyCompatible = 3,

    /// The schema cannot safely be interpreted.
    Incompatible = 4,

    /// There is insufficient trusted information to decide.
    Unknown = 5,
}

impl SchemaCompatibility {
    /// Returns whether the result permits direct decoding without migration.
    #[must_use]
    pub const fn allows_direct_decode(self) -> bool {
        matches!(
            self,
            Self::Compatible | Self::CompatibleWithDegradation
        )
    }

    /// Returns whether migration is required.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::CompatibleWithMigration)
    }

    /// Returns whether the result must prevent unconditional acceptance.
    #[must_use]
    pub const fn requires_additional_validation(self) -> bool {
        matches!(
            self,
            Self::ConditionallyCompatible | Self::Unknown
        )
    }

    /// Returns whether the schema is definitively incompatible.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }
}

// =============================================================================
// Schema envelope
// =============================================================================

/// Schema envelope metadata.
///
/// This is the semantic contract used by encoders and decoders to frame a
/// resilience document.
///
/// It deliberately does not store payload bytes.
///
/// # Why an envelope exists
///
/// Without an explicit envelope, a decoder may be forced to infer:
///
/// - schema version;
/// - document kind;
/// - encoding;
/// - feature set;
/// - compatibility.
///
/// That creates ambiguity and makes deterministic migration difficult.
///
/// The envelope makes the schema boundary explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaEnvelope {
    header: SchemaHeader,
    encoding: EncodingKind,
}

impl SchemaEnvelope {
    /// Creates an envelope.
    #[must_use]
    pub const fn new(
        header: SchemaHeader,
        encoding: EncodingKind,
    ) -> Self {
        Self { header, encoding }
    }

    /// Returns the current schema envelope for a document kind and encoding.
    #[must_use]
    pub const fn current(
        kind: DocumentKind,
        encoding: EncodingKind,
        features: SchemaFeatures,
        flags: SchemaFlags,
    ) -> Self {
        Self::new(
            SchemaHeader::current(kind, features, flags),
            encoding,
        )
    }

    /// Returns the schema header.
    #[must_use]
    pub const fn header(self) -> SchemaHeader {
        self.header
    }

    /// Returns the schema identity.
    #[must_use]
    pub const fn schema(self) -> SchemaId {
        self.header.schema()
    }

    /// Returns the document kind.
    #[must_use]
    pub const fn kind(self) -> DocumentKind {
        self.header.kind()
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn version(self) -> SchemaVersion {
        self.header.version()
    }

    /// Returns feature flags.
    #[must_use]
    pub const fn features(self) -> SchemaFeatures {
        self.header.features()
    }

    /// Returns schema flags.
    #[must_use]
    pub const fn flags(self) -> SchemaFlags {
        self.header.flags()
    }

    /// Returns the encoding kind.
    #[must_use]
    pub const fn encoding(self) -> EncodingKind {
        self.encoding
    }
}

// =============================================================================
// Extension policy
// =============================================================================

/// Defines how unknown schema fields/features are handled.
///
/// This is a schema policy rather than an execution policy.
///
/// The default should be selected by `decode.rs`/`version.rs` according to
/// the trust and compatibility requirements of the operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnknownFieldPolicy {
    /// Reject unknown fields.
    Reject,

    /// Ignore unknown optional fields while preserving all known semantics.
    IgnoreOptional,

    /// Preserve unknown fields for round-trip forwarding where the encoding
    /// layer supports lossless preservation.
    Preserve,
}

impl Default for UnknownFieldPolicy {
    fn default() -> Self {
        Self::Reject
    }
}

// =============================================================================
// Schema validation result
// =============================================================================

/// A small schema-level validation status.
///
/// Domain semantic validation belongs to the owning subsystem. This type
/// exists so encoders/decoders can distinguish structural schema validation
/// from domain validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchemaValidation {
    /// Schema structure is valid.
    Valid,

    /// Schema is syntactically valid but requires compatibility/migration
    /// evaluation before interpretation.
    RequiresCompatibilityCheck,

    /// Schema contains unsupported optional extensions.
    HasUnsupportedExtensions,
}

impl SchemaValidation {
    /// Returns whether the schema is structurally acceptable.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        matches!(
            self,
            Self::Valid | Self::RequiresCompatibilityCheck
        )
    }

    /// Returns whether compatibility evaluation is still required.
    #[must_use]
    pub const fn requires_compatibility_check(self) -> bool {
        matches!(self, Self::RequiresCompatibilityCheck)
    }
}

// =============================================================================
// Schema constants for document kinds
// =============================================================================

/// Returns the current schema identifier for a document kind.
#[must_use]
pub const fn current_schema_id(kind: DocumentKind) -> SchemaId {
    SchemaId::current(kind)
}

/// Returns the current schema header for a document kind.
///
/// The caller supplies the features and structural flags because those are
/// properties of the concrete serialized payload rather than universal
/// properties of every resilience document.
#[must_use]
pub const fn current_schema_header(
    kind: DocumentKind,
    features: SchemaFeatures,
    flags: SchemaFlags,
) -> SchemaHeader {
    SchemaHeader::current(kind, features, flags)
}

/// Returns the current schema envelope for a document kind and encoding.
#[must_use]
pub const fn current_schema_envelope(
    kind: DocumentKind,
    encoding: EncodingKind,
    features: SchemaFeatures,
    flags: SchemaFlags,
) -> SchemaEnvelope {
    SchemaEnvelope::current(
        kind,
        encoding,
        features,
        flags,
    )
}

// =============================================================================
// Schema invariants
// =============================================================================

/// Validates the schema metadata itself.
///
/// This function deliberately performs only schema-level checks.
///
/// It MUST NOT validate:
///
/// - quantum states;
/// - faults;
/// - qubits;
/// - hardware;
/// - topology;
/// - recovery plans;
/// - policies;
/// - credentials.
///
/// Those belong to their respective domains.
///
/// The function is deterministic and allocation-free.
#[must_use]
pub const fn validate_schema_header(
    header: SchemaHeader,
) -> SchemaValidation {
    let version = header.version();

    // A zero major version is reserved for pre-production/experimental
    // schemas and therefore is not accepted by the production schema
    // contract.
    if version.major() == 0 {
        return SchemaValidation::RequiresCompatibilityCheck;
    }

    if header.namespace().as_str().is_empty() {
        return SchemaValidation::HasUnsupportedExtensions;
    }

    if header.kind().code() == 0 {
        return SchemaValidation::HasUnsupportedExtensions;
    }

    SchemaValidation::Valid
}

/// Returns whether a schema version belongs to the current major family.
///
/// This does not mean that the document is fully compatible.
///
/// `serialization::version` remains authoritative for actual compatibility
/// decisions.
#[must_use]
pub const fn shares_current_major(version: SchemaVersion) -> bool {
    version.major() == CURRENT_SCHEMA_MAJOR
}

/// Returns whether the supplied schema is exactly the current schema.
#[must_use]
pub const fn is_current_schema(version: SchemaVersion) -> bool {
    version.is_exact(CURRENT_SCHEMA_VERSION)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_schema_version_is_stable() {
        let version = SchemaVersion::current();

        assert_eq!(version.major(), CURRENT_SCHEMA_MAJOR);
        assert_eq!(version.minor(), CURRENT_SCHEMA_MINOR);
        assert_eq!(version.patch(), CURRENT_SCHEMA_PATCH);
    }

    #[test]
    fn schema_version_order_is_deterministic() {
        let old = SchemaVersion::new(1, 0, 0);
        let new = SchemaVersion::new(1, 1, 0);

        assert!(old < new);
        assert!(new > old);
    }

    #[test]
    fn schema_version_display_is_stable() {
        let version = SchemaVersion::new(7, 4, 12);

        assert_eq!(version.to_string(), "7.4.12");
    }

    #[test]
    fn schema_id_is_deterministic() {
        let first = SchemaId::new(
            DocumentKind::Incident,
            SchemaVersion::new(1, 0, 0),
        );

        let second = SchemaId::new(
            DocumentKind::Incident,
            SchemaVersion::new(1, 0, 0),
        );

        assert_eq!(first, second);
        assert_eq!(
            first.to_string(),
            "zamani.quantum.resilience:incident@1.0.0"
        );
    }

    #[test]
    fn document_kind_codes_are_non_zero() {
        let kinds = [
            DocumentKind::ExecutionState,
            DocumentKind::Incident,
            DocumentKind::RecoveryPlan,
            DocumentKind::RecoveryState,
            DocumentKind::Checkpoint,
            DocumentKind::CheckpointManifest,
            DocumentKind::TelemetryEvent,
            DocumentKind::TelemetryMetric,
            DocumentKind::TelemetryTrace,
            DocumentKind::HealthState,
            DocumentKind::Policy,
            DocumentKind::CapabilitySnapshot,
            DocumentKind::Diagnosis,
            DocumentKind::VerificationResult,
            DocumentKind::Provenance,
            DocumentKind::HistoryRecord,
            DocumentKind::LearningFeedback,
        ];

        for kind in kinds {
            assert_ne!(kind.code(), 0);
            assert!(!kind.as_str().is_empty());
        }
    }

    #[test]
    fn feature_union_is_deterministic() {
        let first = SchemaFeatures::ZQN_FAULTS;
        let second = SchemaFeatures::PHYSICAL_QUBITS;

        let combined = first.union(second);

        assert!(combined.contains(first));
        assert!(combined.contains(second));
    }

    #[test]
    fn unknown_feature_bits_are_preserved() {
        let unknown = 1_u64 << 63;
        let features = SchemaFeatures::from_bits(unknown);

        assert_eq!(features.bits(), unknown);
    }

    #[test]
    fn unknown_schema_flag_bits_are_preserved() {
        let unknown = 1_u32 << 31;
        let flags = SchemaFlags::from_bits(unknown);

        assert_eq!(flags.bits(), unknown);
    }

    #[test]
    fn current_schema_id_uses_current_version() {
        let id = current_schema_id(DocumentKind::Checkpoint);

        assert_eq!(id.version(), CURRENT_SCHEMA_VERSION);
        assert_eq!(id.kind(), DocumentKind::Checkpoint);
    }

    #[test]
    fn current_header_can_describe_physical_resources() {
        let header = current_schema_header(
            DocumentKind::Checkpoint,
            SchemaFeatures::PHYSICAL_QUBITS
                .union(SchemaFeatures::RESOURCE_MAPPING),
            SchemaFlags::DETERMINISTIC
                .union(SchemaFlags::PERSISTABLE),
        );

        assert_eq!(
            header.kind(),
            DocumentKind::Checkpoint
        );

        assert!(
            header
                .features()
                .contains(SchemaFeatures::PHYSICAL_QUBITS)
        );

        assert!(
            header
                .features()
                .contains(SchemaFeatures::RESOURCE_MAPPING)
        );

        assert!(header.is_deterministic());
    }

    #[test]
    fn schema_envelope_preserves_encoding() {
        let envelope = current_schema_envelope(
            DocumentKind::Incident,
            EncodingKind::Binary,
            SchemaFeatures::ZQN_FAULTS,
            SchemaFlags::DETERMINISTIC,
        );

        assert_eq!(
            envelope.encoding(),
            EncodingKind::Binary
        );

        assert_eq!(
            envelope.kind(),
            DocumentKind::Incident
        );
    }

    #[test]
    fn schema_validation_accepts_current_schema() {
        let header = SchemaHeader::current(
            DocumentKind::ExecutionState,
            SchemaFeatures::NONE,
            SchemaFlags::DETERMINISTIC,
        );

        assert_eq!(
            validate_schema_header(header),
            SchemaValidation::Valid
        );
    }

    #[test]
    fn zero_major_requires_compatibility_check() {
        let header = SchemaHeader::new(
            SchemaId::new(
                DocumentKind::Incident,
                SchemaVersion::new(0, 1, 0),
            ),
            SchemaFeatures::NONE,
            SchemaFlags::NONE,
        );

        assert_eq!(
            validate_schema_header(header),
            SchemaValidation::RequiresCompatibilityCheck
        );
    }

    #[test]
    fn compatibility_classification_is_not_execution_action() {
        assert!(
            SchemaCompatibility::Compatible
                .allows_direct_decode()
        );

        assert!(
            SchemaCompatibility::CompatibleWithMigration
                .requires_migration()
        );

        assert!(
            SchemaCompatibility::Incompatible
                .is_incompatible()
        );

        assert!(
            SchemaCompatibility::Unknown
                .requires_additional_validation()
        );
    }

    #[test]
    fn namespace_is_not_empty() {
        assert!(!SchemaNamespace::as_str().is_empty());
    }

    #[test]
    fn current_schema_has_no_machine_size_parameter() {
        // This test deliberately documents an architectural property:
        // schema identity is independent of machine size.
        let one = current_schema_id(DocumentKind::ExecutionState);
        let another = current_schema_id(DocumentKind::ExecutionState);

        assert_eq!(one, another);
    }

    #[test]
    fn unknown_field_policy_defaults_to_reject() {
        assert_eq!(
            UnknownFieldPolicy::default(),
            UnknownFieldPolicy::Reject
        );
    }

    #[test]
    fn deterministic_flag_is_reported() {
        let flags = SchemaFlags::DETERMINISTIC;

        assert!(flags.contains(SchemaFlags::DETERMINISTIC));
    }

    #[test]
    fn forward_extensible_flag_is_reported() {
        let flags = SchemaFlags::FORWARD_EXTENSIBLE;

        assert!(flags.contains(SchemaFlags::FORWARD_EXTENSIBLE));
    }
}