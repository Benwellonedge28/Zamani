//! Zamani Quantum IR — Explicit Version Migration Engine
//!
//! This module owns the EXECUTION of explicitly registered Quantum IR
//! migrations.
//!
//! # Architectural role
//!
//! `compatibility/migration.rs` is deliberately narrower than the rest of the
//! compatibility subsystem:
//
//! ```text
//! core::version
//!       │
//!       │ version meaning
//!       ▼
//! compatibility
//!       │
//!       │ compatibility decision
//!       ▼
//! migration
//!       │
//!       │ explicit transformation
//!       ▼
//! canonical IR
//! ```
//!
//! This module answers:
//
//! > Given an explicitly supported source representation and target
//! > representation, can a registered, deterministic migration transform the
//! > representation without silently losing semantic information?
//!
//! It does NOT:
//!
//! - define what an IR version means;
//! - define serialization framing;
//! - define quantum semantics;
//! - define gate semantics;
//! - define qubit identity;
//! - define hardware;
//! - define routing;
//! - define scheduling;
//! - define optimization;
//! - define QEC;
//! - define backend execution;
//! - define frontend syntax.
//!
//! # Critical safety principle
//!
//! A migration is NOT inferred merely because two versions have the same major
//! number.
//!
//! Version compatibility and migration are different concepts:
//
//! ```text
//! compatible
//!     ≠
//! migration implementation exists
//! ```
//!
//! An older IR may be directly readable without migration. A breaking version
//! may require an explicit registered migration. An unknown version must never
//! be silently converted.
//!
//! # No fabricated historical migrations
//!
//! The repository currently defines the canonical IR version as 1.0.0 but does
//! not provide a complete historical schema catalogue containing concrete
//! transformations for older major versions.
//!
//! Consequently this module intentionally does NOT contain fake migrations such
//! as:
//
//! ```text
//! 0.1.0 -> 1.0.0
//! 1.0.0 -> 2.0.0
//! ```
//!
//! until the corresponding semantic schemas and transformations actually
//! exist.
//!
//! A migration step must be registered with an explicit implementation before
//! it can execute.
//!
//! This prevents the extremely dangerous situation where a compiler silently
//! reinterprets old or future quantum semantics.
//!
//! # Losslessness
//!
//! By default:
//
//! ```text
//! unknown fields       -> preserved or rejected
//! unknown extensions   -> preserved or rejected
//! semantic information -> MUST NOT be discarded
//! ```
//!
//! Lossy migration requires an explicit policy and an explicit declaration from
//! the migration step.
//!
//! # Determinism
//!
//! Migration is deterministic:
//
//! ```text
//! same source bytes
//! + same source version
//! + same target version
//! + same migration registry
//! + same policy
//! = same result
//! ```
//!
//! Migration steps must not depend on:
//
//! - wall-clock time;
//! - random state;
//! - process IDs;
//! - memory addresses;
//! - hash-map iteration order;
//! - hardware;
//! - global mutable state.
//!
//! # Scaling
//!
//! No quantum-machine size is represented here.
//
//! This module does not define:
//
//! - maximum qubits;
//! - maximum operations;
//! - maximum registers;
//! - maximum circuit depth;
//! - maximum topology;
//! - maximum hardware size.
//!
//! Execution/resource limits are caller-selected and apply to migration work,
//! not to the semantic capacity of Zamani Quantum IR.
//!
//! # Transactional semantics
//!
//! Migration operates transactionally:
//
//! ```text
//! source
//!   │
//!   ▼
//! validate migration path
//!   │
//!   ▼
//! clone immutable source payload
//!   │
//!   ▼
//! step 1
//!   │
//!   ▼
//! step 2
//!   │
//!   ▼
//! target
//! ```
//!
//! If any step fails, no partially migrated result is returned.
//!
//! # Integration
//!
//! The intended pipeline is:
//
//! ```text
//! serialized document
//!        │
//!        ▼
//! serialization framing
//!        │
//!        ▼
//! semantic version
//!        │
//!        ▼
//! serialization::compatibility
//!        │
//!        ├── directly readable ──────────────┐
//!        │                                    │
//!        └── migration required               │
//!                 │                           │
//!                 ▼                           │
//!       compatibility::migration              │
//!                 │                           │
//!                 ▼                           │
//!           canonical payload                 │
//!                 │                           │
//!                 └──────────────┬────────────┘
//!                                ▼
//!                         semantic decoder
//!                                │
//!                                ▼
//!                         semantic validation
//! ```
//!
//! # Rust
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The module explicitly forbids unsafe code.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::core::version::IrVersion;

// =============================================================================
// Constants
// =============================================================================

/// Default maximum number of migration steps permitted in one migration.
///
/// This is a safety limit on migration graph traversal, NOT a quantum-machine
/// limit.
///
/// The value is intentionally small because a migration path should normally
/// contain only a bounded number of explicit schema transitions. Callers that
/// operate a deliberately long migration chain can select another policy.
pub const DEFAULT_MAX_STEPS: usize = 1024;

/// Default maximum source/working payload size in bytes.
///
/// This is a resource/security policy, not an IR capacity limit.
///
/// `None` can be used when the caller deliberately delegates allocation limits
/// to an enclosing resource policy.
pub const DEFAULT_MAX_PAYLOAD_BYTES: Option<usize> = Some(256 * 1024 * 1024);

/// Stable identifier for the migration engine implementation.
pub const MIGRATION_ENGINE_ID: &str = "zamani.quantum.ir.compatibility.migration";

/// Current migration-engine contract version.
///
/// This is NOT the semantic Quantum IR version.
///
/// The canonical semantic IR version remains owned by `core::version::IrVersion`.
pub const MIGRATION_ENGINE_VERSION: u16 = 1;

// =============================================================================
// Migration payload
// =============================================================================

/// A versioned semantic migration payload.
///
/// The payload is deliberately opaque to this module.
///
/// The owner of a concrete serialization/schema is responsible for interpreting
/// its bytes. The migration engine only transports them between explicitly
/// registered transformations.
///
/// This separation prevents migration.rs from becoming a second serializer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationPayload {
    version: IrVersion,
    bytes: Vec<u8>,
}

impl MigrationPayload {
    /// Creates a migration payload.
    ///
    /// No semantic validation is performed here.
    ///
    /// The caller must obtain the bytes from a trusted serializer/decoder
    /// boundary or explicitly validate them before migration.
    #[must_use]
    pub fn new(version: IrVersion, bytes: Vec<u8>) -> Self {
        Self { version, bytes }
    }

    /// Returns the semantic IR version represented by this payload.
    #[must_use]
    pub const fn version(&self) -> IrVersion {
        self.version
    }

    /// Returns the payload bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the payload size in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the payload contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consumes the payload and returns its bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Returns an owned copy with a different semantic version.
    ///
    /// This operation MUST only be used by an actual migration step.
    #[must_use]
    pub(crate) fn with_version(
        self,
        version: IrVersion,
    ) -> Self {
        Self {
            version,
            bytes: self.bytes,
        }
    }

    /// Returns an owned copy with replacement bytes and a semantic version.
    ///
    /// This is restricted to migration-step implementations.
    #[must_use]
    pub(crate) fn from_parts(
        version: IrVersion,
        bytes: Vec<u8>,
    ) -> Self {
        Self { version, bytes }
    }
}

// =============================================================================
// Migration direction
// =============================================================================

/// Direction of a migration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MigrationDirection {
    /// Upgrade an older semantic representation.
    Upgrade,

    /// Downgrade a newer semantic representation.
    ///
    /// Downgrades are never assumed to be lossless.
    Downgrade,
}

impl MigrationDirection {
    /// Determines the direction from source and target versions.
    pub const fn between(
        source: IrVersion,
        target: IrVersion,
    ) -> Option<Self> {
        if source < target {
            Some(Self::Upgrade)
        } else if source > target {
            Some(Self::Downgrade)
        } else {
            None
        }
    }
}

impl fmt::Display for MigrationDirection {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Upgrade => formatter.write_str("upgrade"),
            Self::Downgrade => formatter.write_str("downgrade"),
        }
    }
}

// =============================================================================
// Loss policy
// =============================================================================

/// Policy controlling semantic information loss.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LossPolicy {
    /// Any semantic information loss is forbidden.
    Forbid,

    /// Loss is permitted only when the migration step explicitly declares the
    /// affected information as non-semantic.
    AllowDeclared,

    /// Loss is explicitly permitted.
    ///
    /// This mode is intended for tools that intentionally perform lossy
    /// conversion. It must never be the implicit default.
    Allow,
}

impl Default for LossPolicy {
    fn default() -> Self {
        Self::Forbid
    }
}

// =============================================================================
// Unknown information policy
// =============================================================================

/// Policy for unknown fields carried by a migration representation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnknownFieldPolicy {
    /// Preserve unknown fields.
    Preserve,

    /// Reject when unknown fields are encountered.
    Reject,

    /// Explicitly discard unknown fields.
    ///
    /// This is a lossy operation.
    Discard,
}

impl Default for UnknownFieldPolicy {
    fn default() -> Self {
        Self::Preserve
    }
}

/// Policy for unknown extensions carried by a migration representation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnknownExtensionPolicy {
    /// Preserve unknown extensions.
    Preserve,

    /// Reject unknown extensions.
    Reject,

    /// Explicitly discard unknown extensions.
    ///
    /// This is a lossy operation.
    Discard,
}

impl Default for UnknownExtensionPolicy {
    fn default() -> Self {
        Self::Preserve
    }
}

// =============================================================================
// Migration options
// =============================================================================

/// Caller-selected migration resource and compatibility policy.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MigrationOptions {
    /// Maximum number of migration steps allowed.
    ///
    /// This protects against accidental or malicious migration graphs.
    pub max_steps: usize,

    /// Maximum working payload size.
    ///
    /// `None` means no limit is imposed by this layer.
    pub max_payload_bytes: Option<usize>,

    /// Policy for semantic information loss.
    pub loss_policy: LossPolicy,

    /// Policy for unknown fields.
    pub unknown_fields: UnknownFieldPolicy,

    /// Policy for unknown extensions.
    pub unknown_extensions: UnknownExtensionPolicy,

    /// Whether downgrade migrations are allowed.
    pub allow_downgrade: bool,
}

impl Default for MigrationOptions {
    fn default() -> Self {
        Self {
            max_steps: DEFAULT_MAX_STEPS,
            max_payload_bytes: DEFAULT_MAX_PAYLOAD_BYTES,
            loss_policy: LossPolicy::Forbid,
            unknown_fields: UnknownFieldPolicy::Preserve,
            unknown_extensions: UnknownExtensionPolicy::Preserve,
            allow_downgrade: false,
        }
    }
}

impl MigrationOptions {
    /// Validates the resource policy itself.
    pub fn validate(self) -> Result<(), MigrationError> {
        if self.max_steps == 0 {
            return Err(MigrationError::InvalidOptions {
                reason: "max_steps must be greater than zero",
            });
        }

        Ok(())
    }

    /// Returns whether the supplied payload length is permitted.
    #[must_use]
    pub fn payload_size_allowed(
        self,
        length: usize,
    ) -> bool {
        match self.max_payload_bytes {
            Some(limit) => length <= limit,
            None => true,
        }
    }
}

// =============================================================================
// Migration metadata
// =============================================================================

/// Stable identity for a registered migration step.
///
/// The identifier is a semantic registry key, not a memory address or process
/// local handle.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct MigrationId(String);

impl MigrationId {
    /// Creates a migration identifier.
    ///
    /// Empty identifiers are rejected because they cannot provide useful
    /// provenance.
    pub fn new<S: Into<String>>(
        value: S,
    ) -> Result<Self, MigrationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(MigrationError::InvalidMigrationId);
        }

        Ok(Self(value))
    }

    /// Returns the stable identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MigrationId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Metadata describing an explicitly registered migration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationMetadata {
    /// Stable migration identifier.
    pub id: MigrationId,

    /// Human-readable description.
    pub description: String,

    /// Source semantic IR version.
    pub from: IrVersion,

    /// Target semantic IR version.
    pub to: IrVersion,

    /// Whether the step may lose information.
    pub potentially_lossy: bool,

    /// Whether the step requires explicit downgrade permission.
    pub is_downgrade: bool,
}

impl MigrationMetadata {
    /// Constructs migration metadata.
    pub fn new(
        id: MigrationId,
        description: String,
        from: IrVersion,
        to: IrVersion,
        potentially_lossy: bool,
    ) -> Result<Self, MigrationError> {
        if from == to {
            return Err(MigrationError::InvalidMigration {
                id: id.to_string(),
                reason: "source and target versions must differ",
            });
        }

        let is_downgrade = from > to;

        Ok(Self {
            id,
            description,
            from,
            to,
            potentially_lossy,
            is_downgrade,
        })
    }

    /// Returns the direction of the migration.
    #[must_use]
    pub const fn direction(&self) -> MigrationDirection {
        if self.from < self.to {
            MigrationDirection::Upgrade
        } else {
            MigrationDirection::Downgrade
        }
    }
}

// =============================================================================
// Migration context
// =============================================================================

/// Immutable context supplied to every migration step.
#[derive(Clone, Copy, Debug)]
pub struct MigrationContext<'a> {
    /// Caller-selected migration options.
    pub options: &'a MigrationOptions,

    /// Migration engine version.
    pub engine_version: u16,

    /// Current step number, starting at one.
    pub step_number: usize,

    /// Total planned step count.
    pub total_steps: usize,
}

impl<'a> MigrationContext<'a> {
    /// Returns whether loss is permitted for this migration.
    #[must_use]
    pub const fn loss_allowed(self) -> bool {
        matches!(
            self.options.loss_policy,
            LossPolicy::Allow | LossPolicy::AllowDeclared
        )
    }

    /// Returns whether an explicit downgrade is allowed.
    #[must_use]
    pub const fn downgrade_allowed(self) -> bool {
        self.options.allow_downgrade
    }
}

// =============================================================================
// Migration step trait
// =============================================================================

/// A single deterministic version transformation.
///
/// Implementations are expected to live beside the schema/version transition
/// they own.
///
/// A migration step MUST:
//
//! 1. accept only its declared source version;
//! 2. produce exactly its declared target version;
//! 3. never reinterpret a different version;
//! 4. never silently discard semantic information;
//! 5. be deterministic;
//! 6. avoid global mutable state;
//! 7. avoid hardware/backend dependencies;
//! 8. avoid `unsafe`;
//! 9. respect the supplied resource policy;
//! 10. return an error instead of partially succeeding.
///
/// A migration step should perform schema-aware transformations. This generic
/// engine deliberately does not attempt to inspect the payload itself.
pub trait MigrationStep: Send + Sync {
    /// Returns immutable migration metadata.
    fn metadata(&self) -> &MigrationMetadata;

    /// Executes the migration.
    fn migrate(
        &self,
        source: &MigrationPayload,
        context: MigrationContext<'_>,
    ) -> Result<MigrationPayload, MigrationError>;
}

// =============================================================================
// Function-backed migration step
// =============================================================================

type MigrationFunction = fn(
    &MigrationPayload,
    MigrationContext<'_>,
) -> Result<MigrationPayload, MigrationError>;

/// A lightweight migration step backed by a function.
///
/// This is useful for small, deterministic migrations and keeps the registry
/// independent of any particular schema representation.
pub struct FunctionMigration {
    metadata: MigrationMetadata,
    function: MigrationFunction,
}

impl FunctionMigration {
    /// Creates a function-backed migration.
    pub fn new(
        metadata: MigrationMetadata,
        function: MigrationFunction,
    ) -> Self {
        Self {
            metadata,
            function,
        }
    }
}

impl MigrationStep for FunctionMigration {
    fn metadata(&self) -> &MigrationMetadata {
        &self.metadata
    }

    fn migrate(
        &self,
        source: &MigrationPayload,
        context: MigrationContext<'_>,
    ) -> Result<MigrationPayload, MigrationError> {
        (self.function)(source, context)
    }
}

// =============================================================================
// Migration path
// =============================================================================

/// An ordered migration path.
///
/// A path is immutable after construction and contains only explicitly
/// registered steps.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationPath {
    source: IrVersion,
    target: IrVersion,
    steps: Vec<MigrationId>,
}

impl MigrationPath {
    /// Creates an empty path for an exact-version conversion.
    #[must_use]
    pub fn identity(version: IrVersion) -> Self {
        Self {
            source: version,
            target: version,
            steps: Vec::new(),
        }
    }

    /// Creates a migration path.
    pub fn new(
        source: IrVersion,
        target: IrVersion,
        steps: Vec<MigrationId>,
    ) -> Result<Self, MigrationError> {
        if source == target && !steps.is_empty() {
            return Err(MigrationError::InvalidPath {
                reason: "an identity path cannot contain migration steps",
            });
        }

        if source != target && steps.is_empty() {
            return Err(MigrationError::InvalidPath {
                reason: "a non-identity path must contain at least one step",
            });
        }

        Ok(Self {
            source,
            target,
            steps,
        })
    }

    /// Returns the source version.
    #[must_use]
    pub const fn source(&self) -> IrVersion {
        self.source
    }

    /// Returns the target version.
    #[must_use]
    pub const fn target(&self) -> IrVersion {
        self.target
    }

    /// Returns the number of migration steps.
    #[must_use]
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Returns whether this is an identity path.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Returns the migration step identifiers.
    #[must_use]
    pub fn steps(&self) -> &[MigrationId] {
        &self.steps
    }
}

// =============================================================================
// Migration registry
// =============================================================================

/// Deterministic registry of explicit migration steps.
///
/// `BTreeMap` is deliberately used instead of `HashMap` so registry traversal
/// and diagnostics do not depend on hash-map iteration order.
///
/// Registration is expected to happen during compiler/service initialization,
/// not during migration execution.
#[derive(Default)]
pub struct MigrationRegistry {
    steps: BTreeMap<MigrationKey, Box<dyn MigrationStep>>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct MigrationKey {
    from: IrVersion,
    to: IrVersion,
}

impl MigrationRegistry {
    /// Creates an empty migration registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            steps: BTreeMap::new(),
        }
    }

    /// Registers one migration step.
    ///
    /// Two different implementations may not own the same version transition.
    pub fn register(
        &mut self,
        step: Box<dyn MigrationStep>,
    ) -> Result<(), MigrationError> {
        let metadata = step.metadata();

        if metadata.from == metadata.to {
            return Err(MigrationError::InvalidMigration {
                id: metadata.id.to_string(),
                reason: "source and target versions must differ",
            });
        }

        let key = MigrationKey {
            from: metadata.from,
            to: metadata.to,
        };

        if self.steps.contains_key(&key) {
            return Err(MigrationError::DuplicateMigration {
                from: metadata.from,
                to: metadata.to,
            });
        }

        self.steps.insert(key, step);

        Ok(())
    }

    /// Returns whether a direct migration exists.
    #[must_use]
    pub fn contains(
        &self,
        from: IrVersion,
        to: IrVersion,
    ) -> bool {
        self.steps.contains_key(&MigrationKey { from, to })
    }

    /// Returns metadata for a direct migration.
    #[must_use]
    pub fn metadata(
        &self,
        from: IrVersion,
        to: IrVersion,
    ) -> Option<&MigrationMetadata> {
        self.steps
            .get(&MigrationKey { from, to })
            .map(|step| step.metadata())
    }

    /// Returns the number of registered migration steps.
    #[must_use]
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Returns whether the registry contains no migrations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Builds a deterministic migration path.
    ///
    /// The search uses breadth-first traversal over explicitly registered
    /// migrations. Because every edge is explicit and traversal is deterministic,
    /// the resulting path is deterministic for a fixed registry.
    pub fn plan(
        &self,
        source: IrVersion,
        target: IrVersion,
        options: MigrationOptions,
    ) -> Result<MigrationPath, MigrationError> {
        options.validate()?;

        if source == target {
            return Ok(MigrationPath::identity(source));
        }

        if source > target && !options.allow_downgrade {
            return Err(MigrationError::DowngradeNotAllowed {
                from: source,
                to: target,
            });
        }

        #[derive(Clone, Copy)]
        struct QueueEntry {
            version: IrVersion,
        }

        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::BTreeSet::new();
        let mut predecessor: BTreeMap<IrVersion, (IrVersion, MigrationId)> =
            BTreeMap::new();

        queue.push_back(QueueEntry { version: source });
        visited.insert(source);

        let mut examined_edges = 0usize;

        while let Some(entry) = queue.pop_front() {
            if entry.version == target {
                break;
            }

            for (key, step) in &self.steps {
                if key.from != entry.version {
                    continue;
                }

                examined_edges = examined_edges.saturating_add(1);

                if examined_edges > options.max_steps {
                    return Err(MigrationError::PathSearchLimitExceeded {
                        max_steps: options.max_steps,
                    });
                }

                if key.to < key.from && !options.allow_downgrade {
                    continue;
                }

                if !visited.insert(key.to) {
                    continue;
                }

                predecessor.insert(
                    key.to,
                    (
                        key.from,
                        step.metadata().id.clone(),
                    ),
                );

                queue.push_back(QueueEntry { version: key.to });
            }
        }

        if !visited.contains(&target) {
            return Err(MigrationError::NoMigrationPath {
                from: source,
                to: target,
            });
        }

        let mut reversed = Vec::new();
        let mut cursor = target;

        while cursor != source {
            let (previous, migration_id) = predecessor
                .get(&cursor)
                .ok_or(MigrationError::CorruptMigrationPath)?;

            reversed.push(migration_id.clone());
            cursor = *previous;

            if reversed.len() > options.max_steps {
                return Err(MigrationError::PathSearchLimitExceeded {
                    max_steps: options.max_steps,
                });
            }
        }

        reversed.reverse();

        MigrationPath::new(source, target, reversed)
    }

    /// Executes a previously planned migration path.
    pub fn execute(
        &self,
        source: MigrationPayload,
        path: &MigrationPath,
        options: MigrationOptions,
    ) -> Result<MigrationPayload, MigrationError> {
        options.validate()?;

        if source.version() != path.source() {
            return Err(MigrationError::SourceVersionMismatch {
                expected: path.source(),
                actual: source.version(),
            });
        }

        if !options.payload_size_allowed(source.len()) {
            return Err(MigrationError::PayloadTooLarge {
                size: source.len(),
                limit: options.max_payload_bytes,
            });
        }

        if path.len() > options.max_steps {
            return Err(MigrationError::StepLimitExceeded {
                steps: path.len(),
                limit: options.max_steps,
            });
        }

        if path.is_empty() {
            return Ok(source);
        }

        let mut current = source;

        for (index, migration_id) in path.steps().iter().enumerate() {
            let step = self
                .find_by_id(migration_id)
                .ok_or_else(|| MigrationError::MigrationNotRegistered {
                    id: migration_id.clone(),
                })?;

            let metadata = step.metadata();

            if metadata.from != current.version() {
                return Err(MigrationError::StepSourceVersionMismatch {
                    id: migration_id.clone(),
                    expected: metadata.from,
                    actual: current.version(),
                });
            }

            if metadata.is_downgrade && !options.allow_downgrade {
                return Err(MigrationError::DowngradeNotAllowed {
                    from: metadata.from,
                    to: metadata.to,
                });
            }

            if metadata.potentially_lossy {
                match options.loss_policy {
                    LossPolicy::Forbid => {
                        return Err(MigrationError::LossNotAllowed {
                            id: migration_id.clone(),
                        });
                    }

                    LossPolicy::AllowDeclared
                    | LossPolicy::Allow => {}
                }
            }

            let context = MigrationContext {
                options: &options,
                engine_version: MIGRATION_ENGINE_VERSION,
                step_number: index + 1,
                total_steps: path.len(),
            };

            let next = step.migrate(&current, context)?;

            if next.version() != metadata.to {
                return Err(MigrationError::StepTargetVersionMismatch {
                    id: migration_id.clone(),
                    expected: metadata.to,
                    actual: next.version(),
                });
            }

            if !options.payload_size_allowed(next.len()) {
                return Err(MigrationError::PayloadTooLarge {
                    size: next.len(),
                    limit: options.max_payload_bytes,
                });
            }

            current = next;
        }

        if current.version() != path.target() {
            return Err(MigrationError::TargetVersionMismatch {
                expected: path.target(),
                actual: current.version(),
            });
        }

        Ok(current)
    }

    /// Plans and executes a migration in one transactional operation.
    pub fn migrate(
        &self,
        source: MigrationPayload,
        target: IrVersion,
        options: MigrationOptions,
    ) -> Result<MigrationResult, MigrationError> {
        options.validate()?;

        let source_version = source.version();

        if source_version == target {
            return Ok(MigrationResult {
                payload: source,
                path: MigrationPath::identity(target),
            });
        }

        let path = self.plan(
            source_version,
            target,
            options,
        )?;

        let payload = self.execute(
            source,
            &path,
            options,
        )?;

        Ok(MigrationResult {
            payload,
            path,
        })
    }

    fn find_by_id(
        &self,
        id: &MigrationId,
    ) -> Option<&dyn MigrationStep> {
        self.steps
            .values()
            .find(|step| step.metadata().id == *id)
            .map(|step| step.as_ref())
    }
}

// =============================================================================
// Migration result
// =============================================================================

/// Successful migration result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationResult {
    /// Migrated payload.
    pub payload: MigrationPayload,

    /// Exact migration path used.
    pub path: MigrationPath,
}

impl MigrationResult {
    /// Returns the resulting version.
    #[must_use]
    pub const fn version(&self) -> IrVersion {
        self.payload.version()
    }

    /// Returns whether any migration steps were executed.
    #[must_use]
    pub fn migrated(&self) -> bool {
        !self.path.is_empty()
    }

    /// Consumes the result and returns the migrated payload.
    #[must_use]
    pub fn into_payload(self) -> MigrationPayload {
        self.payload
    }
}

// =============================================================================
// Migration errors
// =============================================================================

/// Errors produced by the migration subsystem.
///
/// Errors are deterministic and contain no backend/runtime state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MigrationError {
    /// Migration options are internally invalid.
    InvalidOptions {
        reason: &'static str,
    },

    /// A migration identifier is empty.
    InvalidMigrationId,

    /// A migration declaration is invalid.
    InvalidMigration {
        id: String,
        reason: &'static str,
    },

    /// A duplicate source-to-target transition was registered.
    DuplicateMigration {
        from: IrVersion,
        to: IrVersion,
    },

    /// The caller attempted to downgrade without explicit permission.
    DowngradeNotAllowed {
        from: IrVersion,
        to: IrVersion,
    },

    /// No explicitly registered path exists.
    NoMigrationPath {
        from: IrVersion,
        to: IrVersion,
    },

    /// The migration path data became internally inconsistent.
    CorruptMigrationPath,

    /// The migration graph exceeded the configured traversal budget.
    PathSearchLimitExceeded {
        max_steps: usize,
    },

    /// The source payload version differs from the path's declared source.
    SourceVersionMismatch {
        expected: IrVersion,
        actual: IrVersion,
    },

    /// The payload is larger than the configured migration budget.
    PayloadTooLarge {
        size: usize,
        limit: Option<usize>,
    },

    /// The planned path contains too many steps.
    StepLimitExceeded {
        steps: usize,
        limit: usize,
    },

    /// The migration implementation was not registered.
    MigrationNotRegistered {
        id: MigrationId,
    },

    /// A migration step received an unexpected source version.
    StepSourceVersionMismatch {
        id: MigrationId,
        expected: IrVersion,
        actual: IrVersion,
    },

    /// A migration step returned an unexpected target version.
    StepTargetVersionMismatch {
        id: MigrationId,
        expected: IrVersion,
        actual: IrVersion,
    },

    /// A migration that can lose information was not authorized.
    LossNotAllowed {
        id: MigrationId,
    },

    /// The final payload version differs from the requested target.
    TargetVersionMismatch {
        expected: IrVersion,
        actual: IrVersion,
    },

    /// A migration implementation failed.
    StepFailed {
        id: MigrationId,
        message: String,
    },
}

impl fmt::Display for MigrationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidOptions { reason } => {
                write!(formatter, "invalid migration options: {reason}")
            }

            Self::InvalidMigrationId => {
                formatter.write_str("migration identifier cannot be empty")
            }

            Self::InvalidMigration { id, reason } => {
                write!(
                    formatter,
                    "invalid migration '{id}': {reason}"
                )
            }

            Self::DuplicateMigration { from, to } => {
                write!(
                    formatter,
                    "migration from {from} to {to} is already registered"
                )
            }

            Self::DowngradeNotAllowed { from, to } => {
                write!(
                    formatter,
                    "downgrade from {from} to {to} is not allowed"
                )
            }

            Self::NoMigrationPath { from, to } => {
                write!(
                    formatter,
                    "no registered migration path from {from} to {to}"
                )
            }

            Self::CorruptMigrationPath => {
                formatter.write_str("corrupt migration path")
            }

            Self::PathSearchLimitExceeded { max_steps } => {
                write!(
                    formatter,
                    "migration path search exceeded the limit of {max_steps} steps"
                )
            }

            Self::SourceVersionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "migration source version mismatch: expected {expected}, got {actual}"
                )
            }

            Self::PayloadTooLarge { size, limit } => {
                match limit {
                    Some(limit) => write!(
                        formatter,
                        "migration payload of {size} bytes exceeds limit {limit}"
                    ),

                    None => write!(
                        formatter,
                        "migration payload of {size} bytes is not permitted"
                    ),
                }
            }

            Self::StepLimitExceeded { steps, limit } => {
                write!(
                    formatter,
                    "migration path contains {steps} steps, exceeding limit {limit}"
                )
            }

            Self::MigrationNotRegistered { id } => {
                write!(
                    formatter,
                    "migration '{id}' is not registered"
                )
            }

            Self::StepSourceVersionMismatch {
                id,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "migration '{id}' expected source version {expected}, got {actual}"
                )
            }

            Self::StepTargetVersionMismatch {
                id,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "migration '{id}' produced version {actual}, expected {expected}"
                )
            }

            Self::LossNotAllowed { id } => {
                write!(
                    formatter,
                    "migration '{id}' may lose semantic information and loss is forbidden"
                )
            }

            Self::TargetVersionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "migration target version mismatch: expected {expected}, got {actual}"
                )
            }

            Self::StepFailed { id, message } => {
                write!(
                    formatter,
                    "migration '{id}' failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for MigrationError {}

// =============================================================================
// Standard safe migration helpers
// =============================================================================

/// Creates a lossless version-transition function for a payload whose wire
/// representation is already semantically identical between the two versions.
///
/// This MUST only be registered when the schema owner has established that the
/// representation is genuinely unchanged.
///
/// It is deliberately not automatically registered.
pub fn lossless_retag(
    source: &MigrationPayload,
    context: MigrationContext<'_>,
    target: IrVersion,
) -> Result<MigrationPayload, MigrationError> {
    if source.version() == target {
        return Ok(source.clone());
    }

    if context.step_number == 0 || context.step_number > context.total_steps {
        return Err(MigrationError::StepFailed {
            id: MigrationId::new("lossless-retag").map_err(|_| {
                MigrationError::InvalidMigrationId
            })?,
            message: "invalid migration context".to_owned(),
        });
    }

    if !context.options.payload_size_allowed(source.len()) {
        return Err(MigrationError::PayloadTooLarge {
            size: source.len(),
            limit: context.options.max_payload_bytes,
        });
    }

    Ok(MigrationPayload::from_parts(
        target,
        source.bytes().to_vec(),
    ))
}

/// Creates metadata for a migration that is explicitly lossless.
///
/// The function does not register the migration.
pub fn lossless_metadata(
    id: MigrationId,
    description: String,
    from: IrVersion,
    to: IrVersion,
) -> Result<MigrationMetadata, MigrationError> {
    MigrationMetadata::new(
        id,
        description,
        from,
        to,
        false,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn version(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> IrVersion {
        IrVersion::new(major, minor, patch)
    }

    fn make_step(
        id: &str,
        from: IrVersion,
        to: IrVersion,
    ) -> Box<dyn MigrationStep> {
        let migration_id =
            MigrationId::new(id).expect("valid migration id");

        let metadata = MigrationMetadata::new(
            migration_id,
            format!("test migration {from} -> {to}"),
            from,
            to,
            false,
        )
        .expect("valid metadata");

        Box::new(FunctionMigration::new(
            metadata,
            move |source, context| {
                lossless_retag(
                    source,
                    context,
                    to,
                )
            },
        ))
    }

    #[test]
    fn identity_migration_does_not_change_payload() {
        let v = version(1, 0, 0);

        let payload = MigrationPayload::new(
            v,
            vec![1, 2, 3, 4],
        );

        let registry = MigrationRegistry::new();

        let result = registry
            .migrate(
                payload.clone(),
                v,
                MigrationOptions::default(),
            )
            .expect("identity migration must succeed");

        assert_eq!(result.payload, payload);
        assert!(!result.migrated());
        assert!(result.path.is_empty());
    }

    #[test]
    fn direct_migration_is_planned_and_executed() {
        let from = version(1, 0, 0);
        let to = version(2, 0, 0);

        let mut registry = MigrationRegistry::new();

        registry
            .register(make_step(
                "test.1_to_2",
                from,
                to,
            ))
            .expect("registration must succeed");

        let payload = MigrationPayload::new(
            from,
            vec![10, 20, 30],
        );

        let options = MigrationOptions {
            allow_downgrade: false,
            ..MigrationOptions::default()
        };

        let result = registry
            .migrate(
                payload,
                to,
                options,
            )
            .expect("registered migration must execute");

        assert_eq!(result.version(), to);
        assert_eq!(
            result.payload.bytes(),
            &[10, 20, 30]
        );
        assert_eq!(result.path.len(), 1);
    }

    #[test]
    fn multi_step_path_is_deterministic() {
        let v1 = version(1, 0, 0);
        let v2 = version(1, 1, 0);
        let v3 = version(2, 0, 0);

        let mut registry = MigrationRegistry::new();

        registry
            .register(make_step(
                "test.1_to_1_1",
                v1,
                v2,
            ))
            .expect("registration must succeed");

        registry
            .register(make_step(
                "test.1_1_to_2",
                v2,
                v3,
            ))
            .expect("registration must succeed");

        let path = registry
            .plan(
                v1,
                v3,
                MigrationOptions::default(),
            )
            .expect("path must exist");

        assert_eq!(
            path.steps()
                .iter()
                .map(MigrationId::as_str)
                .collect::<Vec<_>>(),
            vec![
                "test.1_to_1_1",
                "test.1_1_to_2",
            ]
        );
    }

    #[test]
    fn unknown_transition_is_rejected() {
        let registry = MigrationRegistry::new();

        let result = registry.plan(
            version(0, 0, 0),
            version(1, 0, 0),
            MigrationOptions::default(),
        );

        assert!(matches!(
            result,
            Err(MigrationError::NoMigrationPath { .. })
        ));
    }

    #[test]
    fn downgrade_is_rejected_by_default() {
        let from = version(2, 0, 0);
        let to = version(1, 0, 0);

        let mut registry = MigrationRegistry::new();

        registry
            .register(make_step(
                "test.2_to_1",
                from,
                to,
            ))
            .expect("registration must succeed");

        let result = registry.plan(
            from,
            to,
            MigrationOptions::default(),
        );

        assert!(matches!(
            result,
            Err(MigrationError::DowngradeNotAllowed { .. })
        ));
    }

    #[test]
    fn downgrade_can_be_explicitly_enabled() {
        let from = version(2, 0, 0);
        let to = version(1, 0, 0);

        let mut registry = MigrationRegistry::new();

        registry
            .register(make_step(
                "test.2_to_1",
                from,
                to,
            ))
            .expect("registration must succeed");

        let options = MigrationOptions {
            allow_downgrade: true,
            ..MigrationOptions::default()
        };

        let result = registry
            .migrate(
                MigrationPayload::new(
                    from,
                    vec![7, 8],
                ),
                to,
                options,
            )
            .expect("explicit downgrade must succeed");

        assert_eq!(result.version(), to);
    }

    #[test]
    fn payload_limit_is_enforced_before_execution() {
        let from = version(1, 0, 0);
        let to = version(2, 0, 0);

        let mut registry = MigrationRegistry::new();

        registry
            .register(make_step(
                "test.limit",
                from,
                to,
            ))
            .expect("registration must succeed");

        let options = MigrationOptions {
            max_payload_bytes: Some(2),
            ..MigrationOptions::default()
        };

        let result = registry.migrate(
            MigrationPayload::new(
                from,
                vec![1, 2, 3],
            ),
            to,
            options,
        );

        assert!(matches!(
            result,
            Err(MigrationError::PayloadTooLarge { .. })
        ));
    }

    #[test]
    fn duplicate_transitions_are_rejected() {
        let from = version(1, 0, 0);
        let to = version(2, 0, 0);

        let mut registry = MigrationRegistry::new();

        registry
            .register(make_step(
                "test.first",
                from,
                to,
            ))
            .expect("first registration must succeed");

        let result = registry.register(
            make_step(
                "test.second",
                from,
                to,
            ),
        );

        assert!(matches!(
            result,
            Err(MigrationError::DuplicateMigration { .. })
        ));
    }

    #[test]
    fn wrong_source_version_is_rejected() {
        let from = version(1, 0, 0);
        let to = version(2, 0, 0);

        let mut registry = MigrationRegistry::new();

        registry
            .register(make_step(
                "test.version",
                from,
                to,
            ))
            .expect("registration must succeed");

        let path = registry
            .plan(
                from,
                to,
                MigrationOptions::default(),
            )
            .expect("path must exist");

        let payload = MigrationPayload::new(
            version(9, 9, 9),
            vec![],
        );

        let result = registry.execute(
            payload,
            &path,
            MigrationOptions::default(),
        );

        assert!(matches!(
            result,
            Err(MigrationError::SourceVersionMismatch { .. })
        ));
    }

    #[test]
    fn step_target_version_is_verified() {
        let from = version(1, 0, 0);
        let declared_to = version(2, 0, 0);
        let actual_to = version(3, 0, 0);

        let migration_id =
            MigrationId::new("test.bad_target")
                .expect("valid id");

        let metadata = MigrationMetadata::new(
            migration_id,
            "bad target".to_owned(),
            from,
            declared_to,
            false,
        )
        .expect("valid metadata");

        let mut registry = MigrationRegistry::new();

        registry
            .register(Box::new(
                FunctionMigration::new(
                    metadata,
                    move |source, _context| {
                        Ok(MigrationPayload::from_parts(
                            actual_to,
                            source.bytes().to_vec(),
                        ))
                    },
                ),
            ))
            .expect("registration must succeed");

        let result = registry.migrate(
            MigrationPayload::new(
                from,
                vec![1],
            ),
            declared_to,
            MigrationOptions::default(),
        );

        assert!(matches!(
            result,
            Err(MigrationError::StepTargetVersionMismatch { .. })
        ));
    }

    #[test]
    fn lossy_migration_is_rejected_by_default() {
        let from = version(1, 0, 0);
        let to = version(2, 0, 0);

        let id =
            MigrationId::new("test.lossy")
                .expect("valid id");

        let metadata = MigrationMetadata::new(
            id,
            "lossy test".to_owned(),
            from,
            to,
            true,
        )
        .expect("valid metadata");

        let mut registry = MigrationRegistry::new();

        registry
            .register(Box::new(
                FunctionMigration::new(
                    metadata,
                    move |source, _context| {
                        Ok(MigrationPayload::from_parts(
                            to,
                            source.bytes().to_vec(),
                        ))
                    },
                ),
            ))
            .expect("registration must succeed");

        let result = registry.migrate(
            MigrationPayload::new(
                from,
                vec![1],
            ),
            to,
            MigrationOptions::default(),
        );

        assert!(matches!(
            result,
            Err(MigrationError::LossNotAllowed { .. })
        ));
    }

    #[test]
    fn loss_policy_can_explicitly_allow_declared_loss() {
        let from = version(1, 0, 0);
        let to = version(2, 0, 0);

        let id =
            MigrationId::new("test.lossy.allowed")
                .expect("valid id");

        let metadata = MigrationMetadata::new(
            id,
            "lossy test".to_owned(),
            from,
            to,
            true,
        )
        .expect("valid metadata");

        let mut registry = MigrationRegistry::new();

        registry
            .register(Box::new(
                FunctionMigration::new(
                    metadata,
                    move |source, _context| {
                        Ok(MigrationPayload::from_parts(
                            to,
                            source.bytes().to_vec(),
                        ))
                    },
                ),
            ))
            .expect("registration must succeed");

        let options = MigrationOptions {
            loss_policy: LossPolicy::AllowDeclared,
            ..MigrationOptions::default()
        };

        let result = registry.migrate(
            MigrationPayload::new(
                from,
                vec![1],
            ),
            to,
            options,
        );

        assert!(result.is_ok());
    }
}