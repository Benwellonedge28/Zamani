//! Zamani Quantum Noise (ZQN) — Calibration Snapshots
//!
//! Production-grade, immutable calibration snapshots for the ZQN subsystem.
//!
//! # Purpose
//!
//! This module owns the immutable temporal/identity envelope around a set of
//! calibration facts used by ZQN.
//!
//! A `CalibrationSnapshot` answers:
//!
//! > "Which calibration state, for which quantum resources, was considered
//! > valid during which explicit interval, under which calibration schema and
//! > provenance context?"
//!
//! The snapshot is deliberately an envelope rather than the implementation of
//! every possible calibration parameter. Parameter-specific semantics belong to
//! `calibration/parameter.rs`, device-specific organization belongs to
//! `calibration/device.rs`, and gate/readout-specific calibration belongs to
//! their respective modules.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                         ZQN semantic layer
//!                              │
//!              ┌───────────────┼────────────────┐
//!              │               │                │
//!              ▼               ▼                ▼
//!          calibration      noise model      characterization
//!              │               │                │
//!              ▼               │                │
//!       CalibrationSnapshot    │                │
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              ▼
//!                         ZQN execution
//!                              │
//!                 ┌────────────┴────────────┐
//!                 ▼                         ▼
//!             simulator                  hardware
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - immutable calibration snapshot identity;
//! - explicit validity intervals;
//! - snapshot revision;
//! - calibration schema version;
//! - target/resource scope references;
//! - snapshot status;
//! - snapshot construction;
//! - structural validation;
//! - temporal validity queries;
//! - deterministic resource ordering;
//! - snapshot-level consistency checks;
//! - snapshot fingerprints/materialization hooks;
//! - explicit snapshot lineage references.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - calibration parameter mathematics;
//! - gate calibration mathematics;
//! - readout calibration mathematics;
//! - drift models;
//! - interpolation algorithms;
//! - hardware discovery;
//! - QPU credentials;
//! - target capability definitions;
//! - routing;
//! - scheduling;
//! - quantum state;
//! - simulation;
//! - noise channels;
//! - fault generation;
//! - benchmarking methodology;
//! - characterization protocols;
//! - cryptographic hashing implementation;
//! - cryptographic signatures;
//! - serialization formats;
//! - global registries;
//! - mutable calibration state.
//!
//! # Canonical quantum-resource identity
//!
//! ZQN does not define a replacement `QubitId` or `PhysicalQubitId`.
//!
//! Quantum-resource identity remains owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Therefore this file uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! directly.
//!
//! A `PhysicalQubitId` identifies a physical resource reference. Constructing
//! or storing one does not prove that the corresponding hardware resource
//! exists. Hardware capability/availability validation remains outside this
//! module.
//!
//! # Write once, scale everywhere
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CALIBRATION_ENTRIES
//! MAX_DEVICES
//! MAX_PARAMETERS
//! MAX_SNAPSHOT_SIZE
//! ```
//!
//! A snapshot contains dynamically sized collections and therefore has no
//! semantic machine-size ceiling.
//!
//! Concrete execution remains limited by:
//!
//! - available memory;
//! - address space;
//! - storage;
//! - execution time;
//! - caller resource policy;
//! - hardware capability;
//! - distributed-system capacity.
//!
//! "Infinity" therefore means that ZQN does not encode an artificial finite
//! quantum-machine ceiling.
//!
//! # Immutability
//!
//! Once constructed, a `CalibrationSnapshot` cannot be modified.
//!
//! A changed calibration state must produce a new snapshot.
//!
//! This provides:
//!
//! ```text
//! snapshot A
//!     │
//!     └── immutable forever
//!
//! calibration changes
//!     │
//!     ▼
//! snapshot B
//! ```
//!
//! This property is essential for reproducibility and concurrent execution.
//!
//! # Temporal semantics
//!
//! Physical time is explicit.
//!
//! This file never silently calls `SystemTime::now()` to determine calibration
//! validity.
//!
//! Callers provide an explicit `CalibrationTime` when asking whether a snapshot
//! is valid.
//!
//! This follows the ZQN context contract that physical time must be explicit
//! rather than implicitly captured during construction. The context layer
//! likewise deliberately avoids automatic wall-clock capture. 
//!
//! # Validity interval
//!
//! A snapshot uses a half-open interval:
//!
//! ```text
//! [valid_from, valid_until)
//! ```
//!
//! Therefore:
//!
//! ```text
//! valid_from <= t < valid_until
//! ```
//!
//! is valid when the snapshot has a finite end.
//!
//! An open-ended snapshot has:
//!
//! ```text
//! [valid_from, ∞)
//! ```
//!
//! Open-ended validity is permitted because calibration lifetime is a policy
//! concern, not a semantic machine-size constraint.
//!
//! # Determinism
//!
//! Snapshot construction is deterministic.
//!
//! This file:
//!
//! - does not generate random IDs;
//! - does not read the system clock;
//! - does not read process IDs;
//! - does not read thread IDs;
//! - does not depend on hash-map iteration order;
//! - does not use global mutable state;
//! - does not perform implicit I/O.
//!
//! Resource scopes are stored in deterministic order.
//!
//! # Serialization
//!
//! This module defines an in-memory domain model.
//!
//! It deliberately does not derive `Serialize`/`Deserialize` because the
//! repository's ZQN architecture reserves wire-format ownership for
//! `zqn::io`.
//!
//! The future serialization layer must preserve:
//!
//! - snapshot identity;
//! - revision;
//! - schema version;
//! - validity interval;
//! - status;
//! - scope;
//! - lineage;
//! - all calibration payload references;
//! - deterministic ordering.
//!
//! # Provenance
//!
//! A snapshot may reference an upstream calibration/provenance artifact by
//! opaque identity.
//!
//! This module does not implement provenance or hashing.
//!
//! The existing ZQN provenance layer explicitly separates provenance from
//! calibration payload ownership and prohibits duplicate hashing contracts.
//! 
//!
//! # Metadata
//!
//! Generic metadata is intentionally not reimplemented here.
//!
//! When snapshot metadata is integrated, consumers should use the central
//! `zqn::core::metadata` representation rather than creating a second
//! calibration-specific key/value system.
//!
//! # Integration with calibration/parameter.rs
//!
//! Parameter definitions should reference a `CalibrationSnapshot` by
//! `CalibrationId` and use the snapshot's validity/scope contract.
//!
//! This file does not know parameter names or parameter mathematics.
//!
//! # Integration with calibration/device.rs
//!
//! Device calibration code consumes snapshot scope and validates that the
//! referenced physical resources belong to the target being calibrated.
//!
//! Hardware existence remains a hardware-layer responsibility.
//!
//! # Integration with calibration/gate.rs
//!
//! Gate calibration records associate gate-specific calibrated values with a
//! snapshot ID and explicit resource scope.
//!
//! # Integration with calibration/readout.rs
//!
//! Readout calibration records use the same snapshot envelope and canonical
//! physical-resource identities.
//!
//! # Integration with calibration/drift.rs
//!
//! Drift models may create a sequence of snapshots:
//!
//! ```text
//! snapshot(t0)
//! snapshot(t1)
//! snapshot(t2)
//! ...
//! ```
//!
//! The drift module owns interpolation/evolution semantics; this file owns
//! only the immutable snapshot boundary.
//!
//! # Integration with calibration/interpolation.rs
//!
//! Interpolation consumes two or more compatible snapshots and creates an
//! explicit derived calibration state. It must not mutate either source
//! snapshot.
//!
//! # Integration with noise
//!
//! Noise models may reference a snapshot when deriving calibrated noise
//! parameters.
//!
//! A noise model must never silently use the newest globally registered
//! calibration snapshot.
//!
//! The snapshot must be explicitly supplied or explicitly selected by a
//! higher-level execution policy.
//!
//! # Integration with characterization
//!
//! Characterization experiments produce observations. A characterization
//! subsystem may derive a new calibration snapshot from validated observations.
//!
//! This file does not perform statistical estimation.
//!
//! # Integration with hardware
//!
//! Hardware adapters may import provider-neutral calibration data and construct
//! snapshots after validation.
//!
//! Provider credentials and vendor API objects must never be stored here.
//!
//! # Integration with runtime
//!
//! Runtime execution should carry an explicit snapshot reference or snapshot
//! object through its execution context.
//!
//! Runtime must not mutate a snapshot during execution.
//!
//! # Integration with routing and scheduling
//!
//! Routing and scheduling may query snapshot validity and resource scope before
//! using calibrated error/duration information.
//!
//! This file does not calculate routing or scheduling costs.
//!
//! # Integration with QEC
//!
//! QEC may use a snapshot when converting physical calibration information into
//! physical error/fault parameters.
//!
//! QEC does not own the snapshot representation.
//!
//! # Integration with benchmarking
//!
//! Benchmark results should record which calibration snapshot was active.
//!
//! This prevents benchmark results from becoming ambiguous when calibration
//! changes between runs.
//!
//! # Security
//!
//! Calibration data can be security-sensitive and operationally sensitive.
//!
//! This type therefore stores references and values only as explicitly supplied.
//!
//! It does not:
//!
//! - contain credentials;
//! - contain API keys;
//! - execute metadata;
//! - load files;
//! - access networks;
//! - invoke provider code;
//! - dynamically load libraries;
//! - trust hardware identifiers as authorization.
//!
//! An ID is an identifier, not a capability.
//!
//! # Resource safety
//!
//! No global resource ceiling is encoded here.
//!
//! Validation accepts explicit policy limits through `SnapshotValidationLimits`.
//!
//! This allows:
//!
//! - embedded systems;
//! - desktop systems;
//! - servers;
//! - distributed systems;
//! - fuzzers;
//! - hostile-input boundaries;
//!
//! to choose appropriate limits without changing ZQN semantics.
//!
//! # Rust compatibility
//!
//! This implementation targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - standard library only.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use crate::quantum::zqn::core::ids::CalibrationId;

// =============================================================================
// Schema
// =============================================================================

/// Current in-memory calibration snapshot schema version.
///
/// This is intentionally independent from the Zamani language version,
/// Quantum IR version, compiler version, hardware version, and serialization
/// version.
pub const CALIBRATION_SNAPSHOT_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Time
// =============================================================================

/// Explicit physical time used by calibration validity queries.
///
/// The representation is:
///
/// ```text
/// seconds + nanoseconds
/// ```
///
/// with:
///
/// ```text
/// 0 <= nanoseconds < 1_000_000_000
/// ```
///
/// The type does not depend on a wall clock and therefore remains deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CalibrationTime {
    seconds: i64,
    nanoseconds: u32,
}

impl CalibrationTime {
    /// Number of nanoseconds in one second.
    pub const NANOS_PER_SECOND: u32 = 1_000_000_000;

    /// Creates an explicit calibration time.
    ///
    /// The nanosecond component must be less than one second.
    pub const fn new(
        seconds: i64,
        nanoseconds: u32,
    ) -> Result<Self, SnapshotError> {
        if nanoseconds >= Self::NANOS_PER_SECOND {
            return Err(SnapshotError::InvalidTimestamp {
                seconds,
                nanoseconds,
            });
        }

        Ok(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Creates an integral-second calibration time.
    pub const fn from_seconds(seconds: i64) -> Self {
        Self {
            seconds,
            nanoseconds: 0,
        }
    }

    /// Returns the signed seconds component.
    #[must_use]
    pub const fn seconds(self) -> i64 {
        self.seconds
    }

    /// Returns the nanoseconds component.
    #[must_use]
    pub const fn nanoseconds(self) -> u32 {
        self.nanoseconds
    }

    /// Returns the time as a signed `(seconds, nanoseconds)` pair.
    #[must_use]
    pub const fn components(self) -> (i64, u32) {
        (self.seconds, self.nanoseconds)
    }
}

impl fmt::Display for CalibrationTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{:09}s",
            self.seconds,
            self.nanoseconds
        )
    }
}

// =============================================================================
// Validity interval
// =============================================================================

/// Explicit validity interval for a calibration snapshot.
///
/// The interval is half-open:
///
/// ```text
/// [valid_from, valid_until)
/// ```
///
/// `valid_until == None` means that the interval is open-ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CalibrationValidity {
    valid_from: CalibrationTime,
    valid_until: Option<CalibrationTime>,
}

impl CalibrationValidity {
    /// Creates a finite validity interval.
    pub const fn finite(
        valid_from: CalibrationTime,
        valid_until: CalibrationTime,
    ) -> Result<Self, SnapshotError> {
        if valid_until <= valid_from {
            return Err(SnapshotError::InvalidValidityInterval);
        }

        Ok(Self {
            valid_from,
            valid_until: Some(valid_until),
        })
    }

    /// Creates an open-ended validity interval.
    pub const fn open_ended(valid_from: CalibrationTime) -> Self {
        Self {
            valid_from,
            valid_until: None,
        }
    }

    /// Returns the first valid instant.
    #[must_use]
    pub const fn valid_from(self) -> CalibrationTime {
        self.valid_from
    }

    /// Returns the exclusive end, if one exists.
    #[must_use]
    pub const fn valid_until(self) -> Option<CalibrationTime> {
        self.valid_until
    }

    /// Returns whether the supplied explicit time lies inside the interval.
    #[must_use]
    pub const fn contains(self, time: CalibrationTime) -> bool {
        if time < self.valid_from {
            return false;
        }

        match self.valid_until {
            Some(end) => time < end,
            None => true,
        }
    }

    /// Returns whether this interval overlaps another interval.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        match (self.valid_until, other.valid_until) {
            (Some(a_end), Some(b_end)) => {
                self.valid_from < b_end && other.valid_from < a_end
            }

            (Some(a_end), None) => other.valid_from < a_end,

            (None, Some(b_end)) => self.valid_from < b_end,

            (None, None) => true,
        }
    }
}

// =============================================================================
// Snapshot status
// =============================================================================

/// Lifecycle/validation status of a calibration snapshot.
///
/// Status is descriptive. It does not provide authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CalibrationSnapshotStatus {
    /// Snapshot has been structurally validated and may be considered active
    /// by an explicit caller policy.
    Valid,

    /// Snapshot exists but has not been approved for active use.
    Pending,

    /// Snapshot has been explicitly superseded.
    Superseded,

    /// Snapshot has been explicitly invalidated.
    Invalidated,
}

impl CalibrationSnapshotStatus {
    /// Returns whether the status permits active use under a policy that only
    /// requires lifecycle validity.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Valid)
    }
}

// =============================================================================
// Resource scope
// =============================================================================

/// Quantum resource referenced by a calibration snapshot.
///
/// Logical and physical qubit identity remain owned by the canonical Quantum
/// IR.
///
/// This enum does not claim that a referenced physical qubit exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CalibrationResource {
    /// Logical quantum resource.
    LogicalQubit(QubitId),

    /// Physical hardware quantum resource.
    PhysicalQubit(PhysicalQubitId),
}

impl CalibrationResource {
    /// Returns whether this resource is a physical resource.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns whether this resource is a logical resource.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }
}

// =============================================================================
// Snapshot lineage
// =============================================================================

/// Explicit relationship to another calibration snapshot.
///
/// This is intentionally an identity-only relationship. Transformation
/// semantics belong to the owning calibration subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CalibrationLineage {
    /// This snapshot supersedes another snapshot.
    Supersedes(CalibrationId),

    /// This snapshot was derived from another snapshot.
    DerivedFrom(CalibrationId),

    /// This snapshot replaces an invalidated snapshot.
    Replaces(CalibrationId),
}

// =============================================================================
// Validation limits
// =============================================================================

/// Caller-selected resource policy for snapshot validation.
///
/// These limits are not ZQN semantic limits.
///
/// `None` means that this validation layer does not impose that particular
/// ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotValidationLimits {
    /// Maximum number of scoped resources.
    pub max_resources: Option<u64>,

    /// Maximum number of lineage references.
    ///
    /// The current snapshot representation contains at most one lineage
    /// reference, but this policy is intentionally expressed generically so a
    /// future schema can expand without changing the policy vocabulary.
    pub max_lineage_references: Option<u64>,

    /// Maximum number of UTF-8 bytes in the target identifier.
    pub max_target_identifier_bytes: Option<usize>,

    /// Maximum number of UTF-8 bytes in the schema identifier.
    pub max_schema_identifier_bytes: Option<usize>,

    /// Maximum number of UTF-8 bytes in the revision label.
    pub max_revision_label_bytes: Option<usize>,
}

impl SnapshotValidationLimits {
    /// Returns an unrestricted validation policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_resources: None,
            max_lineage_references: None,
            max_target_identifier_bytes: None,
            max_schema_identifier_bytes: None,
            max_revision_label_bytes: None,
        }
    }

    /// Returns a conservative policy suitable for untrusted input.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            max_resources: Some(1_000_000),
            max_lineage_references: Some(1),
            max_target_identifier_bytes: Some(4096),
            max_schema_identifier_bytes: Some(4096),
            max_revision_label_bytes: Some(4096),
        }
    }
}

impl Default for SnapshotValidationLimits {
    fn default() -> Self {
        Self::conservative()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by calibration snapshot construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotError {
    /// Snapshot ID is invalid in the current API context.
    InvalidSnapshotId,

    /// Schema identifier is empty.
    EmptySchemaIdentifier,

    /// Target identifier is empty.
    EmptyTargetIdentifier,

    /// Revision label is empty when supplied.
    EmptyRevisionLabel,

    /// A validity interval has an invalid ordering.
    InvalidValidityInterval,

    /// A timestamp contains an invalid nanosecond component.
    InvalidTimestamp {
        /// Seconds component.
        seconds: i64,

        /// Nanoseconds component.
        nanoseconds: u32,
    },

    /// A textual field exceeded its explicit validation policy.
    FieldTooLarge {
        /// Name of the field.
        field: &'static str,

        /// Actual UTF-8 byte length.
        actual_bytes: usize,

        /// Maximum allowed by the supplied policy.
        maximum_bytes: usize,
    },

    /// Too many scoped resources were supplied.
    ResourceCountExceeded {
        /// Actual number of resources.
        actual: u64,

        /// Maximum allowed by policy.
        maximum: u64,
    },

    /// Too many lineage references were supplied.
    LineageCountExceeded {
        /// Actual count.
        actual: u64,

        /// Maximum allowed by policy.
        maximum: u64,
    },

    /// The same resource appeared more than once.
    DuplicateResource {
        /// Duplicate resource.
        resource: CalibrationResource,
    },

    /// A lineage relationship points back to the snapshot itself.
    SelfLineage,

    /// A lineage relationship contains an invalid zero/placeholder identity
    /// according to the ID implementation.
    InvalidLineage,

    /// Snapshot schema is newer than this implementation supports.
    UnsupportedSchemaVersion {
        /// Encountered version.
        found: u16,

        /// Maximum supported version.
        supported: u16,
    },
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSnapshotId => {
                formatter.write_str("invalid calibration snapshot ID")
            }

            Self::EmptySchemaIdentifier => {
                formatter.write_str(
                    "calibration snapshot schema identifier must not be empty",
                )
            }

            Self::EmptyTargetIdentifier => {
                formatter.write_str(
                    "calibration snapshot target identifier must not be empty",
                )
            }

            Self::EmptyRevisionLabel => {
                formatter.write_str(
                    "calibration snapshot revision label must not be empty",
                )
            }

            Self::InvalidValidityInterval => {
                formatter.write_str(
                    "calibration snapshot validity interval is invalid",
                )
            }

            Self::InvalidTimestamp {
                seconds,
                nanoseconds,
            } => {
                write!(
                    formatter,
                    "invalid calibration timestamp {}.{:09}s",
                    seconds,
                    nanoseconds
                )
            }

            Self::FieldTooLarge {
                field,
                actual_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "calibration snapshot field `{field}` is \
                     {actual_bytes} bytes; maximum is {maximum_bytes}"
                )
            }

            Self::ResourceCountExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "calibration snapshot contains {actual} resources; \
                     maximum policy is {maximum}"
                )
            }

            Self::LineageCountExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "calibration snapshot contains {actual} lineage \
                     references; maximum policy is {maximum}"
                )
            }

            Self::DuplicateResource { resource } => {
                write!(
                    formatter,
                    "calibration snapshot contains duplicate resource \
                     reference: {resource:?}"
                )
            }

            Self::SelfLineage => {
                formatter.write_str(
                    "calibration snapshot cannot reference itself as lineage",
                )
            }

            Self::InvalidLineage => {
                formatter.write_str(
                    "calibration snapshot contains an invalid lineage reference",
                )
            }

            Self::UnsupportedSchemaVersion {
                found,
                supported,
            } => {
                write!(
                    formatter,
                    "unsupported calibration snapshot schema version \
                     {found}; supported through {supported}"
                )
            }
        }
    }
}

impl std::error::Error for SnapshotError {}

// =============================================================================
// Calibration snapshot
// =============================================================================

/// Immutable, provider-neutral calibration snapshot.
///
/// The snapshot is the stable envelope used by all downstream ZQN calibration
/// consumers.
///
/// Parameter payloads are deliberately represented through opaque identity
/// references rather than by coupling this file to future parameter/device/gate
/// modules.
///
/// This allows `snapshot.rs` to be completed and stabilized before those
/// modules are implemented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalibrationSnapshot {
    /// Snapshot identity.
    id: CalibrationId,

    /// Snapshot schema version.
    schema_version: u16,

    /// Provider-neutral target identity.
    target_id: String,

    /// Calibration schema identifier.
    calibration_schema: String,

    /// Monotonic/explicit revision label supplied by the owning calibration
    /// system.
    revision: Option<String>,

    /// Explicit interval for which the snapshot is valid.
    validity: CalibrationValidity,

    /// Lifecycle state.
    status: CalibrationSnapshotStatus,

    /// Canonical quantum-resource scope.
    resources: Vec<CalibrationResource>,

    /// References to calibration objects/parameter sets associated with this
    /// snapshot.
    ///
    /// These are intentionally opaque IDs so this file does not depend on
    /// future parameter implementation details.
    calibration_objects: Vec<CalibrationId>,

    /// Optional lineage relationship.
    lineage: Option<CalibrationLineage>,
}

impl CalibrationSnapshot {
    /// Creates a new immutable calibration snapshot.
    ///
    /// The constructor performs structural validation but does not access
    /// hardware, storage, clocks, registries, networks, or external services.
    ///
    /// `resources` and `calibration_objects` are consumed so construction does
    /// not require an unnecessary second allocation merely to copy the caller's
    /// data.
    pub fn new(
        id: CalibrationId,
        target_id: impl Into<String>,
        calibration_schema: impl Into<String>,
        revision: Option<String>,
        validity: CalibrationValidity,
        status: CalibrationSnapshotStatus,
        mut resources: Vec<CalibrationResource>,
        mut calibration_objects: Vec<CalibrationId>,
        lineage: Option<CalibrationLineage>,
    ) -> Result<Self, SnapshotError> {
        let target_id = target_id.into();
        let calibration_schema = calibration_schema.into();

        validate_non_empty(
            &target_id,
            SnapshotField::TargetIdentifier,
        )?;

        validate_non_empty(
            &calibration_schema,
            SnapshotField::SchemaIdentifier,
        )?;

        if let Some(revision_value) = revision.as_ref() {
            validate_non_empty(
                revision_value,
                SnapshotField::RevisionLabel,
            )?;
        }

        // Deterministic canonical ordering is part of the snapshot contract.
        //
        // This does not imply semantic ordering. It only makes equality,
        // diagnostics, caching and serialization deterministic.
        resources.sort_unstable();
        calibration_objects.sort_unstable();

        ensure_unique_resources(&resources)?;

        if let Some(lineage_value) = lineage {
            if lineage_value_references(&lineage_value, &id) {
                return Err(SnapshotError::SelfLineage);
            }
        }

        Ok(Self {
            id,
            schema_version: CALIBRATION_SNAPSHOT_SCHEMA_VERSION,
            target_id,
            calibration_schema,
            revision,
            validity,
            status,
            resources,
            calibration_objects,
            lineage,
        })
    }

    /// Returns the snapshot ID.
    #[must_use]
    pub const fn id(&self) -> CalibrationId {
        self.id
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the provider-neutral target identifier.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Returns the calibration schema identifier.
    #[must_use]
    pub fn calibration_schema(&self) -> &str {
        &self.calibration_schema
    }

    /// Returns the optional revision label.
    #[must_use]
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }

    /// Returns the explicit validity interval.
    #[must_use]
    pub const fn validity(&self) -> CalibrationValidity {
        self.validity
    }

    /// Returns the lifecycle status.
    #[must_use]
    pub const fn status(&self) -> CalibrationSnapshotStatus {
        self.status
    }

    /// Returns all scoped resources in deterministic order.
    #[must_use]
    pub fn resources(&self) -> &[CalibrationResource] {
        &self.resources
    }

    /// Returns all associated calibration object IDs in deterministic order.
    #[must_use]
    pub fn calibration_objects(&self) -> &[CalibrationId] {
        &self.calibration_objects
    }

    /// Returns the optional lineage reference.
    #[must_use]
    pub const fn lineage(&self) -> Option<CalibrationLineage> {
        self.lineage
    }

    /// Returns whether the snapshot is temporally valid at the supplied
    /// explicit time.
    #[must_use]
    pub const fn is_temporally_valid_at(
        &self,
        time: CalibrationTime,
    ) -> bool {
        self.validity.contains(time)
    }

    /// Returns whether the snapshot can be used at the supplied explicit time
    /// under the basic ZQN lifecycle policy.
    #[must_use]
    pub const fn is_usable_at(&self, time: CalibrationTime) -> bool {
        self.status.is_usable() && self.validity.contains(time)
    }

    /// Returns whether the snapshot explicitly covers the supplied resource.
    #[must_use]
    pub fn covers_resource(&self, resource: CalibrationResource) -> bool {
        self.resources.binary_search(&resource).is_ok()
    }

    /// Returns whether the snapshot covers a canonical logical qubit.
    #[must_use]
    pub fn covers_logical_qubit(&self, qubit: QubitId) -> bool {
        self.covers_resource(CalibrationResource::LogicalQubit(qubit))
    }

    /// Returns whether the snapshot covers a canonical physical qubit.
    #[must_use]
    pub fn covers_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> bool {
        self.covers_resource(CalibrationResource::PhysicalQubit(qubit))
    }

    /// Returns the number of explicitly scoped resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns the number of calibration-object references.
    #[must_use]
    pub fn calibration_object_count(&self) -> usize {
        self.calibration_objects.len()
    }

    /// Returns whether the snapshot has an explicit physical-resource scope.
    #[must_use]
    pub fn has_physical_scope(&self) -> bool {
        self.resources.iter().any(CalibrationResource::is_physical)
    }

    /// Returns whether the snapshot has a logical-resource scope.
    #[must_use]
    pub fn has_logical_scope(&self) -> bool {
        self.resources.iter().any(CalibrationResource::is_logical)
    }

    /// Validates this snapshot under an explicit resource policy.
    ///
    /// This method performs only local structural validation.
    ///
    /// It does NOT determine whether:
    ///
    /// - the target exists;
    /// - the physical resources exist;
    /// - the calibration is scientifically accurate;
    /// - the calibration is current;
    /// - the caller is authorized;
    /// - the target supports the calibration.
    pub fn validate(
        &self,
        limits: SnapshotValidationLimits,
    ) -> Result<(), SnapshotError> {
        if self.schema_version > CALIBRATION_SNAPSHOT_SCHEMA_VERSION {
            return Err(SnapshotError::UnsupportedSchemaVersion {
                found: self.schema_version,
                supported: CALIBRATION_SNAPSHOT_SCHEMA_VERSION,
            });
        }

        validate_non_empty(
            &self.target_id,
            SnapshotField::TargetIdentifier,
        )?;

        validate_non_empty(
            &self.calibration_schema,
            SnapshotField::SchemaIdentifier,
        )?;

        if let Some(revision) = self.revision.as_ref() {
            validate_non_empty(revision, SnapshotField::RevisionLabel)?;
        }

        validate_optional_string_limit(
            "target_id",
            &self.target_id,
            limits.max_target_identifier_bytes,
        )?;

        validate_optional_string_limit(
            "calibration_schema",
            &self.calibration_schema,
            limits.max_schema_identifier_bytes,
        )?;

        if let Some(revision) = self.revision.as_ref() {
            validate_optional_string_limit(
                "revision",
                revision,
                limits.max_revision_label_bytes,
            )?;
        }

        if let Some(maximum) = limits.max_resources {
            let actual = self.resources.len() as u64;

            if actual > maximum {
                return Err(SnapshotError::ResourceCountExceeded {
                    actual,
                    maximum,
                });
            }
        }

        if let Some(maximum) = limits.max_lineage_references {
            let actual = u64::from(self.lineage.is_some());

            if actual > maximum {
                return Err(SnapshotError::LineageCountExceeded {
                    actual,
                    maximum,
                });
            }
        }

        ensure_unique_resources(&self.resources)?;

        if let Some(lineage) = self.lineage {
            if lineage_value_references(&lineage, &self.id) {
                return Err(SnapshotError::SelfLineage);
            }
        }

        Ok(())
    }

    /// Creates a new snapshot that has the supplied lifecycle status.
    ///
    /// This does not mutate the original snapshot.
    ///
    /// The operation is useful for lifecycle transitions such as:
    ///
    /// ```text
    /// Pending → Valid
    /// Valid → Superseded
    /// Valid → Invalidated
    /// ```
    pub fn with_status(
        &self,
        status: CalibrationSnapshotStatus,
    ) -> Self {
        let mut replacement = self.clone();
        replacement.status = status;
        replacement
    }

    /// Creates a new snapshot with a replacement revision.
    ///
    /// The original snapshot remains unchanged.
    pub fn with_revision(
        &self,
        revision: Option<String>,
    ) -> Result<Self, SnapshotError> {
        if let Some(value) = revision.as_ref() {
            validate_non_empty(value, SnapshotField::RevisionLabel)?;
        }

        let mut replacement = self.clone();
        replacement.revision = revision;
        Ok(replacement)
    }

    /// Creates a new snapshot with a replacement validity interval.
    ///
    /// The original snapshot remains unchanged.
    pub fn with_validity(
        &self,
        validity: CalibrationValidity,
    ) -> Self {
        let mut replacement = self.clone();
        replacement.validity = validity;
        replacement
    }

    /// Returns a deterministic comparison key for this snapshot.
    ///
    /// This is not a cryptographic hash and must not be used as an
    /// authentication primitive.
    ///
    /// The key is useful for deterministic ordering/caching at a higher layer.
    #[must_use]
    pub fn canonical_identity_components(
        &self,
    ) -> CalibrationSnapshotIdentity<'_> {
        CalibrationSnapshotIdentity {
            id: self.id,
            schema_version: self.schema_version,
            target_id: &self.target_id,
            calibration_schema: &self.calibration_schema,
            revision: self.revision.as_deref(),
            validity: self.validity,
            status: self.status,
            resources: &self.resources,
            calibration_objects: &self.calibration_objects,
            lineage: self.lineage,
        }
    }
}

// =============================================================================
// Canonical identity view
// =============================================================================

/// Borrowed canonical identity view of a calibration snapshot.
///
/// This type exists so future hashing/serialization layers can consume stable
/// fields without depending on private struct layout.
///
/// It does not calculate a digest.
#[derive(Debug, Clone, Copy)]
pub struct CalibrationSnapshotIdentity<'a> {
    /// Snapshot identity.
    pub id: CalibrationId,

    /// Snapshot schema.
    pub schema_version: u16,

    /// Target identifier.
    pub target_id: &'a str,

    /// Calibration schema identifier.
    pub calibration_schema: &'a str,

    /// Optional revision.
    pub revision: Option<&'a str>,

    /// Validity interval.
    pub validity: CalibrationValidity,

    /// Lifecycle status.
    pub status: CalibrationSnapshotStatus,

    /// Resource scope.
    pub resources: &'a [CalibrationResource],

    /// Calibration-object references.
    pub calibration_objects: &'a [CalibrationId],

    /// Optional lineage.
    pub lineage: Option<CalibrationLineage>,
}

// =============================================================================
// Internal validation helpers
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnapshotField {
    TargetIdentifier,
    SchemaIdentifier,
    RevisionLabel,
}

impl SnapshotField {
    const fn error(self) -> SnapshotError {
        match self {
            Self::TargetIdentifier => {
                SnapshotError::EmptyTargetIdentifier
            }

            Self::SchemaIdentifier => {
                SnapshotError::EmptySchemaIdentifier
            }

            Self::RevisionLabel => {
                SnapshotError::EmptyRevisionLabel
            }
        }
    }
}

fn validate_non_empty(
    value: &str,
    field: SnapshotField,
) -> Result<(), SnapshotError> {
    if value.trim().is_empty() {
        return Err(field.error());
    }

    Ok(())
}

fn validate_optional_string_limit(
    field: &'static str,
    value: &str,
    maximum: Option<usize>,
) -> Result<(), SnapshotError> {
    if let Some(maximum_bytes) = maximum {
        let actual_bytes = value.len();

        if actual_bytes > maximum_bytes {
            return Err(SnapshotError::FieldTooLarge {
                field,
                actual_bytes,
                maximum_bytes,
            });
        }
    }

    Ok(())
}

fn ensure_unique_resources(
    resources: &[CalibrationResource],
) -> Result<(), SnapshotError> {
    if resources.len() < 2 {
        return Ok(());
    }

    for window in resources.windows(2) {
        if window[0] == window[1] {
            return Err(SnapshotError::DuplicateResource {
                resource: window[0],
            });
        }
    }

    Ok(())
}

fn lineage_value_references(
    lineage: &CalibrationLineage,
    id: &CalibrationId,
) -> bool {
    match lineage {
        CalibrationLineage::Supersedes(reference)
        | CalibrationLineage::DerivedFrom(reference)
        | CalibrationLineage::Replaces(reference) => reference == id,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn calibration_id(value: u64) -> CalibrationId {
        CalibrationId::new(value)
    }

    #[test]
    fn calibration_time_rejects_invalid_nanoseconds() {
        let result = CalibrationTime::new(
            0,
            CalibrationTime::NANOS_PER_SECOND,
        );

        assert!(matches!(
            result,
            Err(SnapshotError::InvalidTimestamp { .. })
        ));
    }

    #[test]
    fn finite_validity_is_half_open() {
        let start = CalibrationTime::from_seconds(10);
        let end = CalibrationTime::from_seconds(20);

        let validity =
            CalibrationValidity::finite(start, end).expect("valid interval");

        assert!(validity.contains(start));
        assert!(validity.contains(CalibrationTime::from_seconds(19)));
        assert!(!validity.contains(end));
        assert!(!validity.contains(CalibrationTime::from_seconds(9)));
    }

    #[test]
    fn open_ended_validity_has_no_explicit_end() {
        let start = CalibrationTime::from_seconds(10);
        let validity = CalibrationValidity::open_ended(start);

        assert_eq!(validity.valid_until(), None);
        assert!(validity.contains(CalibrationTime::from_seconds(10)));
        assert!(validity.contains(CalibrationTime::from_seconds(i64::MAX)));
    }

    #[test]
    fn finite_interval_requires_positive_duration() {
        let time = CalibrationTime::from_seconds(10);

        assert!(matches!(
            CalibrationValidity::finite(time, time),
            Err(SnapshotError::InvalidValidityInterval)
        ));
    }

    #[test]
    fn resource_scope_is_sorted_deterministically() {
        let validity = CalibrationValidity::open_ended(
            CalibrationTime::from_seconds(0),
        );

        let resources = vec![
            CalibrationResource::PhysicalQubit(
                PhysicalQubitId::new(8),
            ),
            CalibrationResource::PhysicalQubit(
                PhysicalQubitId::new(2),
            ),
            CalibrationResource::PhysicalQubit(
                PhysicalQubitId::new(5),
            ),
        ];

        let snapshot = CalibrationSnapshot::new(
            calibration_id(1),
            "target-a",
            "zqn-calibration-v1",
            None,
            validity,
            CalibrationSnapshotStatus::Valid,
            resources,
            Vec::new(),
            None,
        )
        .expect("snapshot should construct");

        assert_eq!(
            snapshot.resources(),
            &[
                CalibrationResource::PhysicalQubit(
                    PhysicalQubitId::new(2)
                ),
                CalibrationResource::PhysicalQubit(
                    PhysicalQubitId::new(5)
                ),
                CalibrationResource::PhysicalQubit(
                    PhysicalQubitId::new(8)
                ),
            ]
        );
    }

    #[test]
    fn duplicate_resources_are_rejected() {
        let validity = CalibrationValidity::open_ended(
            CalibrationTime::from_seconds(0),
        );

        let resource = CalibrationResource::PhysicalQubit(
            PhysicalQubitId::new(3),
        );

        let result = CalibrationSnapshot::new(
            calibration_id(1),
            "target-a",
            "zqn-calibration-v1",
            None,
            validity,
            CalibrationSnapshotStatus::Valid,
            vec![resource, resource],
            Vec::new(),
            None,
        );

        assert!(matches!(
            result,
            Err(SnapshotError::DuplicateResource { .. })
        ));
    }

    #[test]
    fn snapshot_does_not_depend_on_machine_size() {
        let validity = CalibrationValidity::open_ended(
            CalibrationTime::from_seconds(0),
        );

        let resources = (0usize..10_000)
            .map(|index| {
                CalibrationResource::PhysicalQubit(
                    PhysicalQubitId::new(index),
                )
            })
            .collect::<Vec<_>>();

        let snapshot = CalibrationSnapshot::new(
            calibration_id(1),
            "target-large",
            "zqn-calibration-v1",
            None,
            validity,
            CalibrationSnapshotStatus::Valid,
            resources,
            Vec::new(),
            None,
        )
        .expect("large snapshot should construct");

        assert_eq!(snapshot.resource_count(), 10_000);
    }

    #[test]
    fn temporal_usability_requires_valid_status() {
        let time = CalibrationTime::from_seconds(10);
        let validity = CalibrationValidity::open_ended(time);

        let snapshot = CalibrationSnapshot::new(
            calibration_id(1),
            "target-a",
            "zqn-calibration-v1",
            None,
            validity,
            CalibrationSnapshotStatus::Pending,
            Vec::new(),
            Vec::new(),
            None,
        )
        .expect("snapshot should construct");

        assert!(!snapshot.is_usable_at(time));
    }

    #[test]
    fn valid_snapshot_is_usable_inside_interval() {
        let start = CalibrationTime::from_seconds(10);
        let end = CalibrationTime::from_seconds(20);

        let validity =
            CalibrationValidity::finite(start, end).expect("interval");

        let snapshot = CalibrationSnapshot::new(
            calibration_id(1),
            "target-a",
            "zqn-calibration-v1",
            None,
            validity,
            CalibrationSnapshotStatus::Valid,
            Vec::new(),
            Vec::new(),
            None,
        )
        .expect("snapshot should construct");

        assert!(snapshot.is_usable_at(
            CalibrationTime::from_seconds(15)
        ));

        assert!(!snapshot.is_usable_at(end));
    }

    #[test]
    fn physical_qubit_scope_uses_canonical_ir_identity() {
        let validity = CalibrationValidity::open_ended(
            CalibrationTime::from_seconds(0),
        );

        let qubit = PhysicalQubitId::new(7);

        let snapshot = CalibrationSnapshot::new(
            calibration_id(1),
            "target-a",
            "zqn-calibration-v1",
            None,
            validity,
            CalibrationSnapshotStatus::Valid,
            vec![CalibrationResource::PhysicalQubit(qubit)],
            Vec::new(),
            None,
        )
        .expect("snapshot should construct");

        assert!(snapshot.covers_physical_qubit(qubit));
    }

    #[test]
    fn logical_qubit_scope_uses_canonical_ir_identity() {
        let validity = CalibrationValidity::open_ended(
            CalibrationTime::from_seconds(0),
        );

        let qubit = QubitId::new(4);

        let snapshot = CalibrationSnapshot::new(
            calibration_id(1),
            "logical-target",
            "zqn-calibration-v1",
            None,
            validity,
            CalibrationSnapshotStatus::Valid,
            vec![CalibrationResource::LogicalQubit(qubit)],
            Vec::new(),
            None,
        )
        .expect("snapshot should construct");

        assert!(snapshot.covers_logical_qubit(qubit));
    }

    #[test]
    fn self_lineage_is_rejected() {
        let id = calibration_id(42);

        let validity = CalibrationValidity::open_ended(
            CalibrationTime::from_seconds(0),
        );

        let result = CalibrationSnapshot::new(
            id,
            "target-a",
            "zqn-calibration-v1",
            None,
            validity,
            CalibrationSnapshotStatus::Valid,
            Vec::new(),
            Vec::new(),
            Some(CalibrationLineage::DerivedFrom(id)),
        );

        assert!(matches!(
            result,
            Err(SnapshotError::SelfLineage)
        ));
    }

    #[test]
    fn status_transition_does_not_mutate_original() {
        let validity = CalibrationValidity::open_ended(
            CalibrationTime::from_seconds(0),
        );

        let original = CalibrationSnapshot::new(
            calibration_id(1),
            "target-a",
            "zqn-calibration-v1",
            None,
            validity,
            CalibrationSnapshotStatus::Valid,
            Vec::new(),
            Vec::new(),
            None,
        )
        .expect("snapshot should construct");

        let superseded =
            original.with_status(CalibrationSnapshotStatus::Superseded);

        assert_eq!(
            original.status(),
            CalibrationSnapshotStatus::Valid
        );

        assert_eq!(
            superseded.status(),
            CalibrationSnapshotStatus::Superseded
        );
    }

    #[test]
    fn validation_policy_is_explicit() {
        let validity = CalibrationValidity::open_ended(
            CalibrationTime::from_seconds(0),
        );

        let snapshot = CalibrationSnapshot::new(
            calibration_id(1),
            "target-a",
            "zqn-calibration-v1",
            None,
            validity,
            CalibrationSnapshotStatus::Valid,
            Vec::new(),
            Vec::new(),
            None,
        )
        .expect("snapshot should construct");

        assert!(snapshot
            .validate(SnapshotValidationLimits::unlimited())
            .is_ok());
    }

    #[test]
    fn canonical_identity_view_is_stable() {
        let validity = CalibrationValidity::open_ended(
            CalibrationTime::from_seconds(0),
        );

        let snapshot = CalibrationSnapshot::new(
            calibration_id(9),
            "target-a",
            "zqn-calibration-v1",
            Some("revision-1".to_owned()),
            validity,
            CalibrationSnapshotStatus::Valid,
            Vec::new(),
            vec![calibration_id(3), calibration_id(2)],
            None,
        )
        .expect("snapshot should construct");

        let identity = snapshot.canonical_identity_components();

        assert_eq!(identity.id, calibration_id(9));
        assert_eq!(
            identity.calibration_objects,
            &[calibration_id(2), calibration_id(3)]
        );
        assert_eq!(identity.revision, Some("revision-1"));
    }
}