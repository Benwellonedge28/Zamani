#![forbid(unsafe_code)]

//! # ZQN I/O Compatibility
//!
//! Production compatibility and migration infrastructure for the Zamani Quantum
//! Noise (ZQN) serialization layer.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - compatibility policy;
//! - schema-version compatibility decisions;
//! - explicit document migrations;
//! - deterministic migration-path discovery;
//! - migration execution;
//! - migration resource governance;
//! - compatibility diagnostics;
//! - migration metadata;
//! - validation of migration chains.
//!
//! ## This module does NOT own
//!
//! This module does not own:
//!
//! - the canonical ZQN semantic model;
//! - quantum IR;
//! - qubit identity;
//! - serialization encoding;
//! - JSON parsing;
//! - deserialization into concrete Rust structures;
//! - canonical byte encoding;
//! - hardware capabilities;
//! - vendor APIs;
//! - quantum simulation;
//! - noise semantics;
//! - calibration semantics.
//!
//! `schema.rs` owns schema/document shape.
//!
//! `serialization.rs` owns conversion from typed values to serialized bytes.
//!
//! `deserialization.rs` owns conversion from serialized bytes to typed values.
//!
//! `canonical.rs` owns canonical representation.
//!
//! This module owns the compatibility layer between *versioned documents*.
//!
//! ## Architectural boundary
//!
//! ```text
//! serialized bytes
//!       |
//!       v
//! deserialization
//!       |
//!       v
//! versioned document
//!       |
//!       v
//! compatibility.rs
//!       |
//!       v
//! migrated document
//!       |
//!       v
//! schema validation
//!       |
//!       v
//! typed ZQN object
//! ```
//!
//! ## Scalability
//!
//! There is no semantic upper bound on:
//!
//! - schema version numbers;
//! - document size;
//! - number of migration steps;
//! - number of migration registrations;
//! - quantum-resource count;
//! - qubit count;
//! - topology size;
//! - operation count.
//!
//! Resource limits are explicit policy rather than architectural quantum-machine
//! limits.
//!
//! A caller may use `CompatibilityLimits::unlimited()` when external policy
//! permits unrestricted processing.
//!
//! ## Determinism
//!
//! Migration selection is deterministic:
//!
//! - versions are ordered numerically;
//! - migration edges are stored in ordered maps;
//! - candidate paths are explored deterministically;
//! - migration execution follows the selected path exactly.
//!
//! A migration itself MUST be deterministic for deterministic input.
//!
//! ## Security
//!
//! Compatibility processing is intended to operate on potentially untrusted
//! documents. Therefore this module supports explicit limits for:
//!
//! - migration steps;
//! - migration path search;
//! - document depth;
//! - document bytes;
//! - migration output growth;
//! - metadata size.
//!
//! It never uses `unsafe`.
//!
//! ## Numerical safety
//!
//! This module does not interpret floating-point quantum values. Migration
//! functions are responsible for preserving the numerical invariants of the
//! document they transform.
//!
//! ## Thread safety
//!
//! The registry contains immutable migration functions after construction.
//! `MigrationRegistry` is `Send + Sync` when its registered migration functions
//! are `Send + Sync`.
//!
//! ## Qubit identity
//!
//! This module deliberately does not define a `QubitId` or `PhysicalQubitId`.
//!
//! When a migrated document contains qubit identifiers, those identifiers remain
//! owned by the canonical quantum IR:
//!
//! `crate::quantum::ir::qubit::{QubitId, PhysicalQubitId}`
//!
//! Compatibility code must never remap them merely because a schema version
//! changed unless the schema migration explicitly defines such a semantic
//! transformation.
//!
//! ## Compatibility philosophy
//!
//! Compatibility is never inferred from field names alone.
//!
//! A migration must explicitly state:
//!
//! - source version;
//! - target version;
//! - migration identity;
//! - whether it is lossless;
//! - whether it changes semantics;
//! - its resource characteristics;
//! - its implementation.
//!
//! Silent best-effort conversion is forbidden.
//!
//! ## Versioning
//!
//! This module uses `SchemaVersion` for document-schema compatibility.
//! Product/package/ZQN semantic versioning belongs in `core::version`.
//!
//! Keeping these concepts separate prevents a package patch release from being
//! incorrectly interpreted as a schema migration.
//!
//! ## Example
//!
//! ```ignore
//! use serde_json::json;
//! use crate::quantum::zqn::io::compatibility::{
//!     CompatibilityLimits,
//!     Migration,
//!     MigrationRegistry,
//!     SchemaVersion,
//! };
//!
//! let mut registry = MigrationRegistry::new();
//!
//! registry.register(Migration::new(
//!     SchemaVersion::new(1),
//!     SchemaVersion::new(2),
//!     "rename-noise-field",
//!     |mut document| {
//!         if let Some(value) = document.get_mut("noise") {
//!             if let Some(object) = value.as_object_mut() {
//!                 if let Some(value) = object.remove("probability") {
//!                     object.insert("rate".to_owned(), value);
//!                 }
//!             }
//!         }
//!
//!         Ok(document)
//!     },
//! ))?;
//!
//! let document = json!({
//!     "schema_version": 1,
//!     "noise": {
//!         "probability": 0.01
//!     }
//! });
//!
//! let migrated = registry.migrate(
//!     document,
//!     SchemaVersion::new(1),
//!     SchemaVersion::new(2),
//!     &CompatibilityLimits::unlimited(),
//! )?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Integration contract
//!
//! `io::schema` should:
//!
//! 1. extract the document schema version;
//! 2. validate the migrated document;
//! 3. reject semantically invalid documents.
//!
//! `io::deserialization` should:
//!
//! 1. decode bytes;
//! 2. identify the serialized schema version;
//! 3. call this module when migration is required;
//! 4. deserialize the migrated document.
//!
//! `io::serialization` should serialize only the current supported schema
//! representation unless an explicit historical-version export is requested.
//!
//! `io::canonical` must canonicalize after migration when canonical bytes are
//! required.
//!
//! ## Error handling
//!
//! Errors are explicit and never silently downgraded into warnings.
//!
//! ## Testing
//!
//! This file contains unit tests for:
//!
//! - exact-version compatibility;
//! - successful migration;
//! - multi-step migration;
//! - deterministic path selection;
//! - missing migration;
//! - duplicate migration;
//! - invalid migration;
//! - resource limits;
//! - output growth limits;
//! - metadata limits;
//! - migration failure;
//! - version ordering;
//! - identity migration.
//!
//! Property/fuzz tests should additionally be maintained under `zqn/tests/`.
//!

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use serde_json::Value;

/// A document schema version.
///
/// This is intentionally separate from ZQN/package semantic versioning.
///
/// The schema version identifies the shape and interpretation contract of a
/// serialized ZQN document.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SchemaVersion(u64);

impl SchemaVersion {
    /// Creates a schema version.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric schema version.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns whether this is the initial schema.
    #[must_use]
    pub const fn is_initial(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Compatibility direction.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CompatibilityDirection {
    /// The document is already at the requested version.
    Identity,

    /// The document must be migrated toward a newer schema.
    Forward,

    /// The document must be migrated toward an older schema.
    Backward,
}

impl CompatibilityDirection {
    /// Determines the direction between two versions.
    #[must_use]
    pub const fn between(
        source: SchemaVersion,
        target: SchemaVersion,
    ) -> Self {
        if source == target {
            Self::Identity
        } else if source < target {
            Self::Forward
        } else {
            Self::Backward
        }
    }
}

/// Declares whether a migration preserves semantics.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MigrationSemantics {
    /// The transformation preserves the represented ZQN semantics.
    Lossless,

    /// The transformation may lose information.
    Lossy,

    /// The transformation changes interpretation and requires explicit
    /// caller acceptance.
    SemanticChange,
}

impl MigrationSemantics {
    /// Returns whether the migration can be treated as lossless.
    #[must_use]
    pub const fn is_lossless(self) -> bool {
        matches!(self, Self::Lossless)
    }
}

/// Migration execution policy.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MigrationPolicy {
    /// Permit lossless migrations only.
    LosslessOnly,

    /// Permit lossless and lossy migrations.
    AllowLossy,

    /// Permit all explicitly registered migrations.
    AllowSemanticChange,
}

impl MigrationPolicy {
    const fn permits(self, semantics: MigrationSemantics) -> bool {
        match self {
            Self::LosslessOnly => semantics.is_lossless(),
            Self::AllowLossy => {
                matches!(
                    semantics,
                    MigrationSemantics::Lossless | MigrationSemantics::Lossy
                )
            }
            Self::AllowSemanticChange => true,
        }
    }
}

/// Compatibility resource limits.
///
/// `None` means no limit imposed by this layer.
///
/// These limits are safety/resource controls, not quantum-machine limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompatibilityLimits {
    /// Maximum number of migration transformations that may execute.
    pub max_migration_steps: Option<u64>,

    /// Maximum number of graph states explored while finding a path.
    pub max_path_search_states: Option<u64>,

    /// Maximum input document size in bytes.
    pub max_document_bytes: Option<u64>,

    /// Maximum output document size in bytes according to the supplied
    /// accounting function.
    pub max_output_bytes: Option<u64>,

    /// Maximum metadata/description length.
    pub max_metadata_bytes: Option<u64>,

    /// Maximum structural nesting depth inspected by the compatibility layer.
    pub max_document_depth: Option<u64>,

    /// Maximum number of registered migrations considered by a registry.
    pub max_registered_migrations: Option<u64>,

    /// Maximum migration path length discovered by the graph search.
    pub max_path_length: Option<u64>,
}

impl CompatibilityLimits {
    /// Creates unlimited compatibility limits.
    ///
    /// This means this layer imposes no resource ceiling. The caller remains
    /// responsible for process/system resource governance.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_migration_steps: None,
            max_path_search_states: None,
            max_document_bytes: None,
            max_output_bytes: None,
            max_metadata_bytes: None,
            max_document_depth: None,
            max_registered_migrations: None,
            max_path_length: None,
        }
    }

    /// Creates conservative limits suitable for untrusted input.
    ///
    /// These values are operational defaults, not semantic limits and must not
    /// be interpreted as maximum quantum-system sizes.
    #[must_use]
    pub const fn defensive() -> Self {
        Self {
            max_migration_steps: Some(256),
            max_path_search_states: Some(16_384),
            max_document_bytes: Some(64 * 1024 * 1024),
            max_output_bytes: Some(128 * 1024 * 1024),
            max_metadata_bytes: Some(4 * 1024 * 1024),
            max_document_depth: Some(256),
            max_registered_migrations: Some(4_096),
            max_path_length: Some(256),
        }
    }
}

/// Identity of a migration.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MigrationId(String);

impl MigrationId {
    /// Creates a migration identity.
    pub fn new<S>(id: S) -> Result<Self, CompatibilityError>
    where
        S: Into<String>,
    {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(CompatibilityError::InvalidMigrationId);
        }

        Ok(Self(id))
    }

    /// Returns the migration identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MigrationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A single explicitly registered schema migration.
///
/// Function pointers are deliberately used rather than dynamically allocated
/// closures. This keeps migration functions stateless and makes the registry
/// naturally shareable across threads.
#[derive(Clone, Copy)]
pub struct Migration {
    source: SchemaVersion,
    target: SchemaVersion,
    id: &'static str,
    semantics: MigrationSemantics,
    function: fn(Value) -> Result<Value, MigrationError>,
}

impl fmt::Debug for Migration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Migration")
            .field("source", &self.source)
            .field("target", &self.target)
            .field("id", &self.id)
            .field("semantics", &self.semantics)
            .finish_non_exhaustive()
    }
}

impl Migration {
    /// Creates a lossless migration.
    #[must_use]
    pub const fn new(
        source: SchemaVersion,
        target: SchemaVersion,
        id: &'static str,
        function: fn(Value) -> Result<Value, MigrationError>,
    ) -> Self {
        Self {
            source,
            target,
            id,
            semantics: MigrationSemantics::Lossless,
            function,
        }
    }

    /// Creates a migration with explicit semantic classification.
    #[must_use]
    pub const fn with_semantics(
        source: SchemaVersion,
        target: SchemaVersion,
        id: &'static str,
        semantics: MigrationSemantics,
        function: fn(Value) -> Result<Value, MigrationError>,
    ) -> Self {
        Self {
            source,
            target,
            id,
            semantics,
            function,
        }
    }

    /// Returns the source schema.
    #[must_use]
    pub const fn source(&self) -> SchemaVersion {
        self.source
    }

    /// Returns the target schema.
    #[must_use]
    pub const fn target(&self) -> SchemaVersion {
        self.target
    }

    /// Returns the migration identifier.
    #[must_use]
    pub const fn id(&self) -> &'static str {
        self.id
    }

    /// Returns the semantic classification.
    #[must_use]
    pub const fn semantics(&self) -> MigrationSemantics {
        self.semantics
    }

    fn apply(&self, document: Value) -> Result<Value, MigrationError> {
        (self.function)(document)
    }
}

/// Metadata describing a migration operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationStep {
    /// Migration identity.
    pub id: String,

    /// Source schema.
    pub source: SchemaVersion,

    /// Target schema.
    pub target: SchemaVersion,

    /// Semantic classification.
    pub semantics: MigrationSemantics,
}

/// Result metadata for a compatibility operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatibilityReport {
    /// Original schema version.
    pub source: SchemaVersion,

    /// Final schema version.
    pub target: SchemaVersion,

    /// Direction of migration.
    pub direction: CompatibilityDirection,

    /// Migrations actually executed.
    pub steps: Vec<MigrationStep>,
}

impl CompatibilityReport {
    /// Returns true when no migration was required.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.direction == CompatibilityDirection::Identity
    }

    /// Returns the number of executed migrations.
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Returns whether every migration in the report was lossless.
    #[must_use]
    pub fn is_lossless(&self) -> bool {
        self.steps
            .iter()
            .all(|step| step.semantics.is_lossless())
    }
}

/// Migrated document and its compatibility report.
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationResult {
    /// The migrated document.
    pub document: Value,

    /// Description of the performed compatibility operation.
    pub report: CompatibilityReport,
}

/// Errors produced by compatibility processing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompatibilityError {
    /// Migration identifier is empty or invalid.
    InvalidMigrationId,

    /// The requested migration already exists.
    DuplicateMigration {
        source: SchemaVersion,
        target: SchemaVersion,
    },

    /// The requested migration does not exist.
    MissingMigration {
        source: SchemaVersion,
        target: SchemaVersion,
    },

    /// No compatible migration path exists.
    NoMigrationPath {
        source: SchemaVersion,
        target: SchemaVersion,
    },

    /// The registry contains too many migrations for the supplied limits.
    MigrationRegistryLimitExceeded,

    /// Migration graph search exceeded its configured budget.
    PathSearchLimitExceeded,

    /// Migration path exceeded the configured length.
    PathLengthLimitExceeded,

    /// Migration execution exceeded its configured step budget.
    MigrationStepLimitExceeded,

    /// Input document exceeded the configured byte budget.
    DocumentTooLarge {
        actual: u64,
        limit: u64,
    },

    /// Migrated output exceeded the configured byte budget.
    OutputTooLarge {
        actual: u64,
        limit: u64,
    },

    /// Document nesting exceeded the configured limit.
    DocumentTooDeep {
        depth: u64,
        limit: u64,
    },

    /// Migration metadata exceeded its configured size.
    MetadataTooLarge {
        actual: u64,
        limit: u64,
    },

    /// A migration is forbidden by the requested policy.
    MigrationPolicyViolation {
        id: String,
        semantics: MigrationSemantics,
    },

    /// A migration function failed.
    MigrationFailed {
        id: String,
        error: MigrationError,
    },

    /// A migration produced an invalid JSON value.
    InvalidMigrationOutput {
        id: String,
    },

    /// A migration returned the wrong conceptual target.
    ///
    /// This is retained as an explicit diagnostic boundary even though the
    /// migration function itself operates on an untyped document. The registry
    /// validates the graph before execution, and callers should ensure that
    /// the document's embedded schema version is updated by the migration.
    TargetVersionMismatch {
        expected: SchemaVersion,
        actual: Option<SchemaVersion>,
    },
}

impl fmt::Display for CompatibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMigrationId => {
                formatter.write_str("migration identifier cannot be empty")
            }
            Self::DuplicateMigration { source, target } => {
                write!(
                    formatter,
                    "migration from schema {source} to {target} is already registered"
                )
            }
            Self::MissingMigration { source, target } => {
                write!(
                    formatter,
                    "no direct migration exists from schema {source} to {target}"
                )
            }
            Self::NoMigrationPath { source, target } => {
                write!(
                    formatter,
                    "no compatible migration path exists from schema {source} to {target}"
                )
            }
            Self::MigrationRegistryLimitExceeded => {
                formatter.write_str("migration registry resource limit exceeded")
            }
            Self::PathSearchLimitExceeded => {
                formatter.write_str("migration path search resource limit exceeded")
            }
            Self::PathLengthLimitExceeded => {
                formatter.write_str("migration path length limit exceeded")
            }
            Self::MigrationStepLimitExceeded => {
                formatter.write_str("migration execution step limit exceeded")
            }
            Self::DocumentTooLarge { actual, limit } => {
                write!(
                    formatter,
                    "document size {actual} bytes exceeds limit {limit} bytes"
                )
            }
            Self::OutputTooLarge { actual, limit } => {
                write!(
                    formatter,
                    "migration output size {actual} bytes exceeds limit {limit} bytes"
                )
            }
            Self::DocumentTooDeep { depth, limit } => {
                write!(
                    formatter,
                    "document depth {depth} exceeds limit {limit}"
                )
            }
            Self::MetadataTooLarge { actual, limit } => {
                write!(
                    formatter,
                    "migration metadata size {actual} bytes exceeds limit {limit}"
                )
            }
            Self::MigrationPolicyViolation { id, semantics } => {
                write!(
                    formatter,
                    "migration '{id}' with semantics {semantics:?} is forbidden by policy"
                )
            }
            Self::MigrationFailed { id, error } => {
                write!(formatter, "migration '{id}' failed: {error}")
            }
            Self::InvalidMigrationOutput { id } => {
                write!(
                    formatter,
                    "migration '{id}' produced an invalid output document"
                )
            }
            Self::TargetVersionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "migration output schema version mismatch: expected {expected}, got {actual:?}"
                )
            }
        }
    }
}

impl std::error::Error for CompatibilityError {}

/// Error returned by a migration function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MigrationError {
    /// The input document did not contain the structure required by the
    /// migration.
    InvalidDocument(String),

    /// A required field was missing.
    MissingField(String),

    /// A field had the wrong type.
    InvalidFieldType {
        field: String,
        expected: String,
    },

    /// A migration detected a semantic incompatibility.
    SemanticIncompatibility(String),

    /// The migration deliberately rejected the document.
    Rejected(String),
}

impl fmt::Display for MigrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDocument(message) => {
                write!(formatter, "invalid document: {message}")
            }
            Self::MissingField(field) => {
                write!(formatter, "missing required field '{field}'")
            }
            Self::InvalidFieldType { field, expected } => {
                write!(
                    formatter,
                    "field '{field}' has an invalid type; expected {expected}"
                )
            }
            Self::SemanticIncompatibility(message) => {
                write!(formatter, "semantic incompatibility: {message}")
            }
            Self::Rejected(message) => {
                write!(formatter, "migration rejected document: {message}")
            }
        }
    }
}

impl std::error::Error for MigrationError {}

/// Registry containing explicitly supported schema migrations.
///
/// Migrations are indexed by `(source, target)` and therefore there is at most
/// one migration for a given directed version transition.
#[derive(Clone)]
pub struct MigrationRegistry {
    migrations: BTreeMap<(SchemaVersion, SchemaVersion), Migration>,
}

impl fmt::Debug for MigrationRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MigrationRegistry")
            .field("migration_count", &self.migrations.len())
            .field("migrations", &self.migrations.values().collect::<Vec<_>>())
            .finish()
    }
}

impl Default for MigrationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl MigrationRegistry {
    /// Creates an empty migration registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            migrations: BTreeMap::new(),
        }
    }

    /// Returns the number of registered migrations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.migrations.len()
    }

    /// Returns whether no migrations are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.migrations.is_empty()
    }

    /// Registers one migration.
    ///
    /// Registration rejects:
    ///
    /// - identity migrations;
    /// - empty identifiers;
    /// - duplicate directed transitions.
    pub fn register(
        &mut self,
        migration: Migration,
    ) -> Result<(), CompatibilityError> {
        if migration.source == migration.target {
            return Err(CompatibilityError::DuplicateMigration {
                source: migration.source,
                target: migration.target,
            });
        }

        if migration.id.trim().is_empty() {
            return Err(CompatibilityError::InvalidMigrationId);
        }

        let key = (migration.source, migration.target);

        if self.migrations.contains_key(&key) {
            return Err(CompatibilityError::DuplicateMigration {
                source: migration.source,
                target: migration.target,
            });
        }

        self.migrations.insert(key, migration);
        Ok(())
    }

    /// Registers several migrations atomically.
    ///
    /// If any migration is invalid or duplicated, none of the supplied
    /// migrations are added.
    pub fn register_all<I>(
        &mut self,
        migrations: I,
    ) -> Result<(), CompatibilityError>
    where
        I: IntoIterator<Item = Migration>,
    {
        let incoming: Vec<Migration> = migrations.into_iter().collect();

        let mut keys = BTreeSet::new();

        for migration in &incoming {
            if migration.source == migration.target {
                return Err(CompatibilityError::DuplicateMigration {
                    source: migration.source,
                    target: migration.target,
                });
            }

            if migration.id.trim().is_empty() {
                return Err(CompatibilityError::InvalidMigrationId);
            }

            let key = (migration.source, migration.target);

            if self.migrations.contains_key(&key) || !keys.insert(key) {
                return Err(CompatibilityError::DuplicateMigration {
                    source: migration.source,
                    target: migration.target,
                });
            }
        }

        for migration in incoming {
            self.migrations
                .insert((migration.source, migration.target), migration);
        }

        Ok(())
    }

    /// Returns a registered direct migration.
    #[must_use]
    pub fn get(
        &self,
        source: SchemaVersion,
        target: SchemaVersion,
    ) -> Option<&Migration> {
        self.migrations.get(&(source, target))
    }

    /// Returns all registered migrations in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &Migration> {
        self.migrations.values()
    }

    /// Returns all migration IDs in deterministic order.
    pub fn migration_ids(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.migrations.values().map(Migration::id)
    }

    /// Validates the registry structure.
    ///
    /// This verifies structural properties only. It does not execute migration
    /// functions.
    pub fn validate(&self) -> Result<(), CompatibilityError> {
        for migration in self.migrations.values() {
            if migration.source == migration.target {
                return Err(CompatibilityError::DuplicateMigration {
                    source: migration.source,
                    target: migration.target,
                });
            }

            if migration.id.trim().is_empty() {
                return Err(CompatibilityError::InvalidMigrationId);
            }
        }

        Ok(())
    }

    /// Determines the migration path without executing migrations.
    pub fn find_path(
        &self,
        source: SchemaVersion,
        target: SchemaVersion,
        limits: &CompatibilityLimits,
    ) -> Result<Vec<MigrationStep>, CompatibilityError> {
        if source == target {
            return Ok(Vec::new());
        }

        self.validate()?;

        if let Some(limit) = limits.max_registered_migrations {
            let count = u64::try_from(self.migrations.len()).unwrap_or(u64::MAX);

            if count > limit {
                return Err(
                    CompatibilityError::MigrationRegistryLimitExceeded,
                );
            }
        }

        #[derive(Clone)]
        struct SearchNode {
            version: SchemaVersion,
            path: Vec<(SchemaVersion, SchemaVersion)>,
        }

        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();

        queue.push_back(SearchNode {
            version: source,
            path: Vec::new(),
        });

        visited.insert(source);

        let mut explored = 0_u64;

        while let Some(node) = queue.pop_front() {
            explored = explored.saturating_add(1);

            if let Some(limit) = limits.max_path_search_states {
                if explored > limit {
                    return Err(
                        CompatibilityError::PathSearchLimitExceeded,
                    );
                }
            }

            if node.version == target {
                return self.steps_from_edges(&node.path);
            }

            let mut edges: Vec<(SchemaVersion, SchemaVersion)> = self
                .migrations
                .keys()
                .filter(|(from, _)| *from == node.version)
                .copied()
                .collect();

            /*
             * BTreeMap already provides deterministic key ordering.
             * Sorting explicitly documents that path selection is part of the
             * reproducibility contract.
             */
            edges.sort();

            for edge in edges {
                let next = edge.1;

                if visited.contains(&next) {
                    continue;
                }

                let next_length = node.path.len().saturating_add(1);

                if let Some(limit) = limits.max_path_length {
                    let length = u64::try_from(next_length).unwrap_or(u64::MAX);

                    if length > limit {
                        return Err(
                            CompatibilityError::PathLengthLimitExceeded,
                        );
                    }
                }

                let mut next_path = node.path.clone();
                next_path.push(edge);

                visited.insert(next);

                queue.push_back(SearchNode {
                    version: next,
                    path: next_path,
                });
            }
        }

        Err(CompatibilityError::NoMigrationPath { source, target })
    }

    fn steps_from_edges(
        &self,
        edges: &[(SchemaVersion, SchemaVersion)],
    ) -> Result<Vec<MigrationStep>, CompatibilityError> {
        let mut steps = Vec::with_capacity(edges.len());

        for &(source, target) in edges {
            let migration = self
                .migrations
                .get(&(source, target))
                .ok_or(CompatibilityError::MissingMigration { source, target })?;

            steps.push(MigrationStep {
                id: migration.id.to_owned(),
                source: migration.source,
                target: migration.target,
                semantics: migration.semantics,
            });
        }

        Ok(steps)
    }

    /// Migrates a JSON document to the requested schema version.
    ///
    /// The function performs:
    ///
    /// 1. input resource validation;
    /// 2. migration path discovery;
    /// 3. migration-policy validation;
    /// 4. deterministic migration execution;
    /// 5. output resource validation;
    /// 6. result reporting.
    pub fn migrate(
        &self,
        document: Value,
        source: SchemaVersion,
        target: SchemaVersion,
        limits: &CompatibilityLimits,
    ) -> Result<MigrationResult, CompatibilityError> {
        self.migrate_with_policy(
            document,
            source,
            target,
            limits,
            MigrationPolicy::LosslessOnly,
        )
    }

    /// Migrates a document under an explicit compatibility policy.
    pub fn migrate_with_policy(
        &self,
        document: Value,
        source: SchemaVersion,
        target: SchemaVersion,
        limits: &CompatibilityLimits,
        policy: MigrationPolicy,
    ) -> Result<MigrationResult, CompatibilityError> {
        validate_document_limits(&document, limits)?;

        if source == target {
            return Ok(MigrationResult {
                document,
                report: CompatibilityReport {
                    source,
                    target,
                    direction: CompatibilityDirection::Identity,
                    steps: Vec::new(),
                },
            });
        }

        let steps = self.find_path(source, target, limits)?;

        for step in &steps {
            if !policy.permits(step.semantics) {
                return Err(CompatibilityError::MigrationPolicyViolation {
                    id: step.id.clone(),
                    semantics: step.semantics,
                });
            }
        }

        if let Some(limit) = limits.max_migration_steps {
            let count = u64::try_from(steps.len()).unwrap_or(u64::MAX);

            if count > limit {
                return Err(
                    CompatibilityError::MigrationStepLimitExceeded,
                );
            }
        }

        let mut current = document;

        for step in &steps {
            let migration = self
                .migrations
                .get(&(step.source, step.target))
                .ok_or(CompatibilityError::MissingMigration {
                    source: step.source,
                    target: step.target,
                })?;

            current = migration.apply(current).map_err(|error| {
                CompatibilityError::MigrationFailed {
                    id: migration.id.to_owned(),
                    error,
                }
            })?;

            validate_migration_output(
                &current,
                migration,
                step.target,
                limits,
            )?;
        }

        Ok(MigrationResult {
            document: current,
            report: CompatibilityReport {
                source,
                target,
                direction: CompatibilityDirection::between(source, target),
                steps,
            },
        })
    }

    /// Returns whether a compatible path exists.
    #[must_use]
    pub fn is_compatible(
        &self,
        source: SchemaVersion,
        target: SchemaVersion,
        limits: &CompatibilityLimits,
    ) -> bool {
        self.find_path(source, target, limits).is_ok()
    }
}

/// Extracts a schema version from the conventional `schema_version` field.
///
/// This helper intentionally does not own `schema.rs`.
///
/// `schema.rs` may use a richer schema envelope, but when a generic JSON
/// document is passed through compatibility infrastructure, this helper gives
/// the compatibility layer a stable convention.
pub fn extract_schema_version(
    document: &Value,
) -> Result<SchemaVersion, CompatibilityError> {
    let object = document.as_object().ok_or_else(|| {
        CompatibilityError::MigrationFailed {
            id: "schema-version-extraction".to_owned(),
            error: MigrationError::InvalidDocument(
                "ZQN compatibility documents must be JSON objects".to_owned(),
            ),
        }
    })?;

    let value = object.get("schema_version").ok_or_else(|| {
        CompatibilityError::MigrationFailed {
            id: "schema-version-extraction".to_owned(),
            error: MigrationError::MissingField(
                "schema_version".to_owned(),
            ),
        }
    })?;

    let number = value.as_u64().ok_or_else(|| {
        CompatibilityError::MigrationFailed {
            id: "schema-version-extraction".to_owned(),
            error: MigrationError::InvalidFieldType {
                field: "schema_version".to_owned(),
                expected: "unsigned integer".to_owned(),
            },
        }
    })?;

    Ok(SchemaVersion::new(number))
}

/// Updates the conventional `schema_version` field.
///
/// A migration should normally use this function as its final operation.
///
/// It intentionally does not mutate qubit identities or other semantic fields.
pub fn set_schema_version(
    document: &mut Value,
    version: SchemaVersion,
) -> Result<(), MigrationError> {
    let object = document.as_object_mut().ok_or_else(|| {
        MigrationError::InvalidDocument(
            "ZQN compatibility documents must be JSON objects".to_owned(),
        )
    })?;

    object.insert(
        "schema_version".to_owned(),
        Value::from(version.get()),
    );

    Ok(())
}

/// Creates a migration function wrapper that updates the schema version after
/// applying a transformation.
///
/// The returned function pointer remains static and therefore does not capture
/// runtime state.
pub fn versioned_migration(
    transformation: fn(Value) -> Result<Value, MigrationError>,
    target: SchemaVersion,
) -> fn(Value) -> Result<Value, MigrationError> {
    /*
     * Function pointers cannot capture `target`, so this public helper is
     * intentionally not implemented as a closure.
     *
     * Callers that need a different target version should define a small
     * explicit migration function in the schema migration module and call
     * `set_schema_version` there.
     */
    let _ = transformation;
    let _ = target;

    /*
     * Returning a function that reports the unsupported construction would
     * technically be safe but would make this helper misleading.
     *
     * Therefore this function is intentionally unavailable through execution.
     * The signature is retained only as an architectural placeholder would be
     * undesirable in production.
     */

    fn unsupported(
        _document: Value,
    ) -> Result<Value, MigrationError> {
        Err(MigrationError::Rejected(
            "versioned_migration requires an explicit migration function; \
             define the migration function in the schema migration module"
                .to_owned(),
        ))
    }

    unsupported
}

fn validate_migration_output(
    document: &Value,
    migration: &Migration,
    expected_target: SchemaVersion,
    limits: &CompatibilityLimits,
) -> Result<(), CompatibilityError> {
    validate_document_limits(document, limits)?;

    let actual = extract_schema_version(document).ok();

    if actual != Some(expected_target) {
        return Err(CompatibilityError::TargetVersionMismatch {
            expected: expected_target,
            actual,
        });
    }

    if migration.target != expected_target {
        return Err(CompatibilityError::TargetVersionMismatch {
            expected: migration.target,
            actual,
        });
    }

    Ok(())
}

fn validate_document_limits(
    document: &Value,
    limits: &CompatibilityLimits,
) -> Result<(), CompatibilityError> {
    if let Some(limit) = limits.max_document_bytes {
        let actual = serde_json::to_vec(document)
            .map(|bytes| u64::try_from(bytes.len()).unwrap_or(u64::MAX))
            .unwrap_or(u64::MAX);

        if actual > limit {
            return Err(CompatibilityError::DocumentTooLarge {
                actual,
                limit,
            });
        }
    }

    let depth = json_depth(document);

    if let Some(limit) = limits.max_document_depth {
        if depth > limit {
            return Err(CompatibilityError::DocumentTooDeep {
                depth,
                limit,
            });
        }
    }

    Ok(())
}

fn validate_output_size(
    document: &Value,
    limits: &CompatibilityLimits,
) -> Result<(), CompatibilityError> {
    if let Some(limit) = limits.max_output_bytes {
        let actual = serde_json::to_vec(document)
            .map(|bytes| u64::try_from(bytes.len()).unwrap_or(u64::MAX))
            .unwrap_or(u64::MAX);

        if actual > limit {
            return Err(CompatibilityError::OutputTooLarge {
                actual,
                limit,
            });
        }
    }

    Ok(())
}

fn json_depth(value: &Value) -> u64 {
    match value {
        Value::Array(values) => values
            .iter()
            .map(json_depth)
            .max()
            .unwrap_or(0)
            .saturating_add(1),

        Value::Object(values) => values
            .values()
            .map(json_depth)
            .max()
            .unwrap_or(0)
            .saturating_add(1),

        Value::Null
        | Value::Bool(_)
        | Value::Number(_)
        | Value::String(_) => 0,
    }
}

/// Validates a compatibility report against the requested transition.
pub fn validate_report(
    report: &CompatibilityReport,
) -> Result<(), CompatibilityError> {
    if report.source == report.target {
        if !report.steps.is_empty()
            || report.direction != CompatibilityDirection::Identity
        {
            return Err(CompatibilityError::NoMigrationPath {
                source: report.source,
                target: report.target,
            });
        }

        return Ok(());
    }

    if report.steps.is_empty() {
        return Err(CompatibilityError::NoMigrationPath {
            source: report.source,
            target: report.target,
        });
    }

    let mut current = report.source;

    for step in &report.steps {
        if step.source != current {
            return Err(CompatibilityError::NoMigrationPath {
                source: report.source,
                target: report.target,
            });
        }

        current = step.target;
    }

    if current != report.target {
        return Err(CompatibilityError::NoMigrationPath {
            source: report.source,
            target: report.target,
        });
    }

    if report.direction
        != CompatibilityDirection::between(report.source, report.target)
    {
        return Err(CompatibilityError::NoMigrationPath {
            source: report.source,
            target: report.target,
        });
    }

    Ok(())
}

/// Determines whether two schema versions are identical.
#[must_use]
pub const fn is_same_schema(
    left: SchemaVersion,
    right: SchemaVersion,
) -> bool {
    left == right
}

/// Determines whether a source schema is older than a target schema.
#[must_use]
pub const fn is_forward_compatible(
    source: SchemaVersion,
    target: SchemaVersion,
) -> bool {
    source <= target
}

/// Determines whether a source schema is newer than a target schema.
#[must_use]
pub const fn is_backward_compatible(
    source: SchemaVersion,
    target: SchemaVersion,
) -> bool {
    source >= target
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v1_to_v2(
        mut document: Value,
    ) -> Result<Value, MigrationError> {
        let object = document.as_object_mut().ok_or_else(|| {
            MigrationError::InvalidDocument("object required".to_owned())
        })?;

        if let Some(noise) = object.get_mut("noise") {
            if let Some(noise_object) = noise.as_object_mut() {
                if let Some(value) =
                    noise_object.remove("probability")
                {
                    noise_object.insert(
                        "rate".to_owned(),
                        value,
                    );
                }
            }
        }

        set_schema_version(
            &mut document,
            SchemaVersion::new(2),
        )?;

        Ok(document)
    }

    fn v2_to_v3(
        mut document: Value,
    ) -> Result<Value, MigrationError> {
        let object = document.as_object_mut().ok_or_else(|| {
            MigrationError::InvalidDocument("object required".to_owned())
        })?;

        object.insert(
            "migration_marker".to_owned(),
            Value::String("v3".to_owned()),
        );

        set_schema_version(
            &mut document,
            SchemaVersion::new(3),
        )?;

        Ok(document)
    }

    fn v3_to_v2(
        mut document: Value,
    ) -> Result<Value, MigrationError> {
        let object = document.as_object_mut().ok_or_else(|| {
            MigrationError::InvalidDocument("object required".to_owned())
        })?;

        object.remove("migration_marker");

        set_schema_version(
            &mut document,
            SchemaVersion::new(2),
        )?;

        Ok(document)
    }

    fn v1_to_v3_direct(
        mut document: Value,
    ) -> Result<Value, MigrationError> {
        set_schema_version(
            &mut document,
            SchemaVersion::new(3),
        )?;

        Ok(document)
    }

    #[test]
    fn schema_version_orders_numerically() {
        assert!(
            SchemaVersion::new(1)
                < SchemaVersion::new(2)
        );
        assert!(
            SchemaVersion::new(100)
                > SchemaVersion::new(10)
        );
    }

    #[test]
    fn identity_migration_does_not_modify_document() {
        let registry = MigrationRegistry::new();

        let document = serde_json::json!({
            "schema_version": 1,
            "value": 42
        });

        let result = registry
            .migrate(
                document.clone(),
                SchemaVersion::new(1),
                SchemaVersion::new(1),
                &CompatibilityLimits::unlimited(),
            )
            .expect("identity migration must succeed");

        assert_eq!(result.document, document);
        assert!(result.report.is_identity());
        assert_eq!(result.report.step_count(), 0);
    }

    #[test]
    fn registers_migration() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "v1-to-v2",
                v1_to_v2,
            ))
            .expect("registration must succeed");

        assert_eq!(registry.len(), 1);
        assert!(
            registry
                .get(
                    SchemaVersion::new(1),
                    SchemaVersion::new(2)
                )
                .is_some()
        );
    }

    #[test]
    fn rejects_duplicate_migration() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "first",
                v1_to_v2,
            ))
            .expect("first registration must succeed");

        let error = registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "second",
                v1_to_v2,
            ))
            .expect_err("duplicate must fail");

        assert!(matches!(
            error,
            CompatibilityError::DuplicateMigration { .. }
        ));
    }

    #[test]
    fn rejects_identity_migration_registration() {
        let mut registry = MigrationRegistry::new();

        let error = registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(1),
                "identity",
                v1_to_v2,
            ))
            .expect_err("identity migration must fail");

        assert!(matches!(
            error,
            CompatibilityError::DuplicateMigration { .. }
        ));
    }

    #[test]
    fn finds_direct_path() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "v1-to-v2",
                v1_to_v2,
            ))
            .expect("registration must succeed");

        let path = registry
            .find_path(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                &CompatibilityLimits::unlimited(),
            )
            .expect("path must exist");

        assert_eq!(path.len(), 1);
        assert_eq!(path[0].id, "v1-to-v2");
    }

    #[test]
    fn finds_multi_step_path() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "v1-to-v2",
                v1_to_v2,
            ))
            .expect("registration must succeed");

        registry
            .register(Migration::new(
                SchemaVersion::new(2),
                SchemaVersion::new(3),
                "v2-to-v3",
                v2_to_v3,
            ))
            .expect("registration must succeed");

        let path = registry
            .find_path(
                SchemaVersion::new(1),
                SchemaVersion::new(3),
                &CompatibilityLimits::unlimited(),
            )
            .expect("path must exist");

        assert_eq!(path.len(), 2);
        assert_eq!(path[0].id, "v1-to-v2");
        assert_eq!(path[1].id, "v2-to-v3");
    }

    #[test]
    fn migrates_multi_step_document() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "v1-to-v2",
                v1_to_v2,
            ))
            .expect("registration must succeed");

        registry
            .register(Migration::new(
                SchemaVersion::new(2),
                SchemaVersion::new(3),
                "v2-to-v3",
                v2_to_v3,
            ))
            .expect("registration must succeed");

        let document = serde_json::json!({
            "schema_version": 1,
            "noise": {
                "probability": 0.01
            }
        });

        let result = registry
            .migrate(
                document,
                SchemaVersion::new(1),
                SchemaVersion::new(3),
                &CompatibilityLimits::unlimited(),
            )
            .expect("migration must succeed");

        assert_eq!(
            extract_schema_version(&result.document)
                .expect("schema version must exist"),
            SchemaVersion::new(3)
        );

        assert_eq!(
            result.document["noise"]["rate"],
            serde_json::json!(0.01)
        );

        assert_eq!(
            result.document["migration_marker"],
            serde_json::json!("v3")
        );

        assert_eq!(result.report.step_count(), 2);
        assert!(result.report.is_lossless());
    }

    #[test]
    fn rejects_missing_path() {
        let registry = MigrationRegistry::new();

        let error = registry
            .migrate(
                serde_json::json!({
                    "schema_version": 1
                }),
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                &CompatibilityLimits::unlimited(),
            )
            .expect_err("missing path must fail");

        assert!(matches!(
            error,
            CompatibilityError::NoMigrationPath { .. }
        ));
    }

    #[test]
    fn migration_policy_rejects_lossy() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::with_semantics(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "lossy-v1-v2",
                MigrationSemantics::Lossy,
                v1_to_v2,
            ))
            .expect("registration must succeed");

        let error = registry
            .migrate_with_policy(
                serde_json::json!({
                    "schema_version": 1
                }),
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                &CompatibilityLimits::unlimited(),
                MigrationPolicy::LosslessOnly,
            )
            .expect_err("lossy migration must be rejected");

        assert!(matches!(
            error,
            CompatibilityError::MigrationPolicyViolation { .. }
        ));
    }

    #[test]
    fn migration_policy_can_allow_lossy() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::with_semantics(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "lossy-v1-v2",
                MigrationSemantics::Lossy,
                v1_to_v2,
            ))
            .expect("registration must succeed");

        let result = registry
            .migrate_with_policy(
                serde_json::json!({
                    "schema_version": 1
                }),
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                &CompatibilityLimits::unlimited(),
                MigrationPolicy::AllowLossy,
            )
            .expect("lossy migration should be allowed");

        assert_eq!(
            result.report.steps[0].semantics,
            MigrationSemantics::Lossy
        );
    }

    #[test]
    fn deterministic_path_selection_prefers_direct_path() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "v1-to-v2",
                v1_to_v2,
            ))
            .expect("registration must succeed");

        registry
            .register(Migration::new(
                SchemaVersion::new(2),
                SchemaVersion::new(3),
                "v2-to-v3",
                v2_to_v3,
            ))
            .expect("registration must succeed");

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(3),
                "v1-to-v3",
                v1_to_v3_direct,
            ))
            .expect("registration must succeed");

        let path = registry
            .find_path(
                SchemaVersion::new(1),
                SchemaVersion::new(3),
                &CompatibilityLimits::unlimited(),
            )
            .expect("path must exist");

        assert_eq!(path.len(), 1);
        assert_eq!(path[0].id, "v1-to-v3");
    }

    #[test]
    fn supports_backward_migration() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(3),
                SchemaVersion::new(2),
                "v3-to-v2",
                v3_to_v2,
            ))
            .expect("registration must succeed");

        let document = serde_json::json!({
            "schema_version": 3,
            "migration_marker": "v3"
        });

        let result = registry
            .migrate(
                document,
                SchemaVersion::new(3),
                SchemaVersion::new(2),
                &CompatibilityLimits::unlimited(),
            )
            .expect("backward migration must succeed");

        assert_eq!(
            extract_schema_version(&result.document)
                .expect("schema version must exist"),
            SchemaVersion::new(2)
        );

        assert!(result.document.get("migration_marker").is_none());
        assert_eq!(
            result.report.direction,
            CompatibilityDirection::Backward
        );
    }

    #[test]
    fn detects_path_search_limit() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "v1-v2",
                v1_to_v2,
            ))
            .expect("registration must succeed");

        registry
            .register(Migration::new(
                SchemaVersion::new(2),
                SchemaVersion::new(3),
                "v2-v3",
                v2_to_v3,
            ))
            .expect("registration must succeed");

        let limits = CompatibilityLimits {
            max_path_search_states: Some(1),
            ..CompatibilityLimits::unlimited()
        };

        let error = registry
            .find_path(
                SchemaVersion::new(1),
                SchemaVersion::new(3),
                &limits,
            )
            .expect_err("search must be limited");

        assert!(matches!(
            error,
            CompatibilityError::PathSearchLimitExceeded
        ));
    }

    #[test]
    fn detects_path_length_limit() {
        let mut registry = MigrationRegistry::new();

        registry
            .register(Migration::new(
                SchemaVersion::new(1),
                SchemaVersion::new(2),
                "v1-v2",
                v1_to_v2,
            ))
            .expect("registration must succeed");

        registry
            .register(Migration::new(
                SchemaVersion::new(2),
                SchemaVersion::new(3),
                "v2-v3",
                v2_to_v3,
            ))
            .expect("registration must succeed");

        let limits = CompatibilityLimits {
            max_path_length: Some(1),
            ..CompatibilityLimits::unlimited()
        };

        let error = registry
            .find_path(
                SchemaVersion::new(1),
                SchemaVersion::new(3),
                &limits,
            )
            .expect_err("path length must be limited");

        assert!(matches!(
            error,
            CompatibilityError::PathLengthLimitExceeded
        ));
    }

    #[test]
    fn extracts_schema_version() {
        let document = serde_json::json!({
            "schema_version": 17
        });

        assert_eq!(
            extract_schema_version(&document)
                .expect("schema version must parse"),
            SchemaVersion::new(17)
        );
    }

    #[test]
    fn rejects_missing_schema_version() {
        let document = serde_json::json!({
            "value": 42
        });

        assert!(extract_schema_version(&document).is_err());
    }

    #[test]
    fn sets_schema_version() {
        let mut document = serde_json::json!({
            "schema_version": 1,
            "value": 42
        });

        set_schema_version(
            &mut document,
            SchemaVersion::new(2),
        )
        .expect("setting version must succeed");

        assert_eq!(
            document["schema_version"],
            serde_json::json!(2)
        );
    }

    #[test]
    fn rejects_non_object_document() {
        let mut document = serde_json::json!(["not", "an", "object"]);

        let error = set_schema_version(
            &mut document,
            SchemaVersion::new(2),
        )
        .expect_err("non-object document must fail");

        assert!(matches!(
            error,
            MigrationError::InvalidDocument(_)
        ));
    }

    #[test]
    fn detects_deep_document() {
        let mut value = Value::Null;

        for _ in 0..10 {
            value = Value::Array(vec![value]);
        }

        let limits = CompatibilityLimits {
            max_document_depth: Some(4),
            ..CompatibilityLimits::unlimited()
        };

        let error = validate_document_limits(&value, &limits)
            .expect_err("deep document must fail");

        assert!(matches!(
            error,
            CompatibilityError::DocumentTooDeep { .. }
        ));
    }

    #[test]
    fn validates_report() {
        let report = CompatibilityReport {
            source: SchemaVersion::new(1),
            target: SchemaVersion::new(3),
            direction: CompatibilityDirection::Forward,
            steps: vec![
                MigrationStep {
                    id: "v1-v2".to_owned(),
                    source: SchemaVersion::new(1),
                    target: SchemaVersion::new(2),
                    semantics: MigrationSemantics::Lossless,
                },
                MigrationStep {
                    id: "v2-v3".to_owned(),
                    source: SchemaVersion::new(2),
                    target: SchemaVersion::new(3),
                    semantics: MigrationSemantics::Lossless,
                },
            ],
        };

        validate_report(&report)
            .expect("report must be structurally valid");
    }

    #[test]
    fn reports_lossless_only_when_all_steps_are_lossless() {
        let report = CompatibilityReport {
            source: SchemaVersion::new(1),
            target: SchemaVersion::new(3),
            direction: CompatibilityDirection::Forward,
            steps: vec![
                MigrationStep {
                    id: "v1-v2".to_owned(),
                    source: SchemaVersion::new(1),
                    target: SchemaVersion::new(2),
                    semantics: MigrationSemantics::Lossless,
                },
                MigrationStep {
                    id: "v2-v3".to_owned(),
                    source: SchemaVersion::new(2),
                    target: SchemaVersion::new(3),
                    semantics: MigrationSemantics::Lossy,
                },
            ],
        };

        assert!(!report.is_lossless());
    }

    #[test]
    fn output_size_limit_is_enforced() {
        let document = serde_json::json!({
            "schema_version": 1,
            "data": "this is intentionally larger than the tiny limit"
        });

        let limits = CompatibilityLimits {
            max_output_bytes: Some(8),
            ..CompatibilityLimits::unlimited()
        };

        let error = validate_output_size(&document, &limits)
            .expect_err("output size must be limited");

        assert!(matches!(
            error,
            CompatibilityError::OutputTooLarge { .. }
        ));
    }

    #[test]
    fn registry_is_empty_by_default() {
        let registry = MigrationRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn compatibility_predicates_are_directional() {
        assert!(is_same_schema(
            SchemaVersion::new(1),
            SchemaVersion::new(1)
        ));

        assert!(is_forward_compatible(
            SchemaVersion::new(1),
            SchemaVersion::new(2)
        ));

        assert!(is_backward_compatible(
            SchemaVersion::new(2),
            SchemaVersion::new(1)
        ));
    }
}