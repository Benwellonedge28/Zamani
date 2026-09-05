//! Zamani Quantum Scheduling — Serialization Schema
//!
//! Production-grade, versioned, target-independent schema contracts for the
//! quantum scheduling subsystem.
//!
//! # Purpose
//!
//! This module defines the stable wire/data schema used to persist, transport,
//! inspect, cache, reproduce, and exchange scheduling information.
//!
//! It intentionally does NOT perform encoding or decoding.
//!
//! The ownership boundary is:
//!
//! ```text
//! serialization/schema.rs
//!        │
//!        ├── defines versioned schema contracts
//!        ├── defines serialized data shapes
//!        ├── defines compatibility rules
//!        ├── defines validation metadata
//!        └── defines forward/backward compatibility boundaries
//!        │
//!        ├──────────────► serialization/encode.rs
//!        └──────────────► serialization/decode.rs
//! ```
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! scheduling adapters
//!      │
//!      ▼
//! scheduling
//!      │
//!      ▼
//! ScheduleResult
//!      │
//!      ▼
//! serialization/schema
//!      │
//!      ├── encode
//!      └── decode
//! ```
//!
//! The schema is deliberately downstream of scheduling semantics.
//!
//! It must not become a second definition of:
//!
//! - `QuantumCircuit`;
//! - `QuantumOperation`;
//! - `Gate`;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - hardware topology;
//! - scheduling algorithms;
//! - resource allocation;
//! - QEC algorithms.
//!
//! Those concepts remain owned by their canonical subsystems.
//!
//! # Canonical quantum identity
//!
//! Logical and physical qubit identities are owned exclusively by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The schema deliberately stores their stable serialized numeric
//! representation rather than defining replacement Rust identity types.
//!
//! This is intentional:
//!
//! 1. the canonical IR remains authoritative;
//! 2. serialization does not create duplicate identity types;
//! 3. the wire format remains stable across Rust implementation changes;
//! 4. host architecture does not affect serialized identity width;
//! 5. adapters can reconstruct canonical `QubitId` / `PhysicalQubitId` values
//!    through their owning constructors.
//!
//! See `quantum::ir::qubit` for the canonical identity contract.
//!
//! # Semantic identity
//!
//! Semantic identifiers are serialized as fixed-width unsigned integers.
//!
//! They are identifiers, NOT:
//!
//! - vector indices;
//! - qubit counts;
//! - resource capacities;
//! - memory addresses;
//! - host pointers;
//! - machine-size limits.
//!
//! `u64` is used because it is stable across 32-bit and 64-bit hosts and is
//! independent of `usize`.
//!
//! # Scalability
//!
//! There is deliberately no:
//!
//! - maximum qubit count;
//! - maximum physical-qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - maximum schedule depth;
//! - maximum number of dependencies;
//! - maximum number of reservations;
//! - maximum number of timing intervals;
//! - maximum QEC rounds;
//! - maximum distributed nodes;
//! - maximum channel count.
//!
//! Dynamic collection fields therefore represent arbitrary finite schedules
//! subject only to the resources and explicit limits of the caller.
//!
//! In Zamani terminology, "scale to infinity" means:
//!
//! > the schema introduces no artificial finite machine-size ceiling.
//!
//! A real serialized object is still necessarily finite and bounded by:
//!
//! - available memory;
//! - address space;
//! - storage;
//! - transport limits;
//! - explicit caller limits;
//! - operating-system constraints.
//!
//! Such limits must never be hidden in this schema.
//!
//! # Compatibility philosophy
//!
//! Schema compatibility is conservative.
//!
//! ```text
//! major
//!   breaking structural/semantic changes
//!
//! minor
//!   additive backward-compatible changes
//!
//! patch
//!   corrections that do not alter interpretation
//! ```
//!
//! Unknown major versions MUST be rejected.
//!
//! Unknown future minor versions MUST NOT be silently interpreted unless the
//! decoder has explicitly established that the encountered additions are
//! safely ignorable.
//!
//! Unknown fields should be preserved by higher-level formats where the chosen
//! encoding permits preservation, but this Rust schema itself does not promise
//! lossless preservation of unknown implementation-specific fields.
//!
//! # Determinism
//!
//! Schema values must be capable of deterministic serialization.
//!
//! Collections representing semantically unordered sets should be sorted by
//! their canonical semantic key before deterministic encoding.
//!
//! The schema itself does not impose a particular encoder.
//!
//! `serialization/encode.rs` owns canonical ordering during encoding.
//!
//! # Security
//!
//! This module contains no executable deserialization logic.
//!
//! Decoders MUST validate:
//!
//! - schema version;
//! - enum discriminants;
//! - identifier domains;
//! - integer ranges;
//! - interval consistency;
//! - cross-reference validity;
//! - duplicate semantic identities where forbidden;
//! - resource capacities;
//! - timing relationships;
//! - object counts against caller-provided limits.
//!
//! A serialized schedule MUST NEVER be trusted merely because it conforms to
//! the Rust type shape.
//!
//! # Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `serde` derives are intentionally avoided in this schema file so the
//! schema remains independent of any particular serialization framework.
//!
//! The encode/decode layer may implement framework-specific conversions.
//!
//! # Dependency policy
//!
//! This file must remain close to dependency-free.
//!
//! It may depend on:
//!
//! - `std`;
//! - canonical primitive representations;
//! - scheduler-independent schema vocabulary.
//!
//! It must NOT depend on:
//!
//! - a hardware vendor SDK;
//! - a runtime;
//! - a simulator;
//! - a particular QPU;
//! - a scheduling algorithm;
//! - routing implementation details;
//! - QEC implementation details.
//!
//! # Integration contract
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling::adapters::ir
//!      │
//!      ▼
//! scheduling::ir
//!      │
//!      ▼
//! scheduling::planners
//!      │
//!      ▼
//! ScheduleResult
//!      │
//!      ▼
//! scheduling::serialization::schema
//!      │
//!      ├── encode.rs
//!      └── decode.rs
//! ```
//!
//! Hardware-specific information is represented only as target metadata and
//! opaque identifiers. The scheduler schema does not embed vendor SDK types.
//!
//! # Ownership rule
//!
//! Every field in this schema must have one owning subsystem.
//!
//! The schema is a representation boundary, not the owner of the represented
//! semantics.
//!
//! This rule prevents a later implementation of `resources`, `timing`,
//! `verification`, or `hardware` from forcing this file to redefine domain
//! logic.
//!
//! # Public API stability
//!
//! This module is intended to be a stable contract.
//!
//! Adding optional/additive schema fields should normally be a minor schema
//! revision.
//!
//! Removing, renaming, changing the meaning, or changing the representation of
//! an existing required field is a major schema revision.
//!
//! ---------------------------------------------------------------------------
//! No-unsafe policy
//! ---------------------------------------------------------------------------
//!
//! The scheduler serialization subsystem is explicitly memory-safe.
//!
//! There is no reason for schema representation to require unsafe Rust.
//!
//! ---------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

// =============================================================================
// Schema version
// =============================================================================

/// Current scheduling schema major version.
///
/// Major changes are reserved for incompatible structural or semantic changes.
pub const SCHEDULING_SCHEMA_MAJOR: u16 = 1;

/// Current scheduling schema minor version.
///
/// Minor changes are additive and intended to remain backward compatible.
pub const SCHEDULING_SCHEMA_MINOR: u16 = 0;

/// Current scheduling schema patch version.
///
/// Patch changes correct implementation/schema defects without changing the
/// semantic contract.
pub const SCHEDULING_SCHEMA_PATCH: u16 = 0;

/// Complete current scheduling schema version.
pub const SCHEDULING_SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(
    SCHEDULING_SCHEMA_MAJOR,
    SCHEDULING_SCHEMA_MINOR,
    SCHEDULING_SCHEMA_PATCH,
);

/// Version of the serialized scheduling schema.
///
/// This version is intentionally independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - Quantum IR version;
/// - hardware version;
/// - backend version;
/// - calibration version;
/// - scheduler algorithm version.
///
/// A schedule may therefore identify all of those independently through
/// metadata while retaining one stable serialization contract.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct SchemaVersion {
    /// Incompatible structural/semantic version.
    pub major: u16,

    /// Backward-compatible additive version.
    pub minor: u16,

    /// Non-semantic correction version.
    pub patch: u16,
}

impl SchemaVersion {
    /// Constructs a schema version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns whether two versions have the same compatibility domain.
    ///
    /// A major-version mismatch is potentially breaking.
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns whether `other` can potentially be interpreted by an
    /// implementation supporting `self`.
    ///
    /// This deliberately performs only the conservative major-version check.
    /// Decoder policy must perform any additional minor/feature checks.
    pub const fn potentially_compatible_with(self, other: Self) -> bool {
        self.major == other.major
    }
}

impl Default for SchemaVersion {
    fn default() -> Self {
        SCHEDULING_SCHEMA_VERSION
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// =============================================================================
// Compatibility policy
// =============================================================================

/// Compatibility classification for a serialized scheduling schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchemaCompatibility {
    /// Exact schema version.
    Exact,

    /// Older schema within the same major version.
    OlderMinor,

    /// Newer schema within the same major version.
    NewerMinor,

    /// Patch-level difference within the same major/minor version.
    PatchDifference,

    /// Major version differs and therefore requires an explicit migration.
    IncompatibleMajor,
}

impl SchemaCompatibility {
    /// Classifies two schema versions.
    pub const fn classify(
        supported: SchemaVersion,
        encountered: SchemaVersion,
    ) -> Self {
        if supported.major != encountered.major {
            return Self::IncompatibleMajor;
        }

        if supported.major == encountered.major
            && supported.minor == encountered.minor
            && supported.patch == encountered.patch
        {
            return Self::Exact;
        }

        if supported.major == encountered.major
            && supported.minor == encountered.minor
        {
            return Self::PatchDifference;
        }

        if encountered.minor < supported.minor {
            Self::OlderMinor
        } else {
            Self::NewerMinor
        }
    }

    /// Returns whether this classification is structurally compatible without
    /// requiring a major-version migration.
    pub const fn same_major(self) -> bool {
        !matches!(self, Self::IncompatibleMajor)
    }
}

// =============================================================================
// Stable semantic identity representations
// =============================================================================

/// Stable serialized semantic identity.
///
/// This is the wire representation of an owning subsystem's semantic ID.
///
/// It MUST NOT be interpreted as an array index or capacity.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct SerializedId(pub u64);

impl SerializedId {
    /// Constructs an ID representation.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the stable numeric representation.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Canonical quantum operation identity in serialized form.
///
/// The semantic owner remains:
///
/// `crate::quantum::ir::core::identity::OperationId`
pub type SerializedOperationId = SerializedId;

/// Canonical resource identity in serialized form.
///
/// The semantic owner remains the scheduler/resource or canonical IR identity
/// subsystem according to the consuming resource contract.
pub type SerializedResourceId = SerializedId;

/// Scheduler dependency identity in serialized form.
pub type SerializedDependencyId = SerializedId;

/// Scheduler reservation identity in serialized form.
pub type SerializedReservationId = SerializedId;

/// Logical qubit identity in serialized form.
///
/// The canonical Rust identity remains:
///
/// `crate::quantum::ir::qubit::QubitId`
pub type SerializedQubitId = SerializedId;

/// Physical qubit identity in serialized form.
///
/// The canonical Rust identity remains:
///
/// `crate::quantum::ir::qubit::PhysicalQubitId`
pub type SerializedPhysicalQubitId = SerializedId;

/// Classical dependency identity in serialized form.
pub type SerializedClassicalDependencyId = SerializedId;

// =============================================================================
// Time
// =============================================================================

/// Canonical serialized time representation.
///
/// The representation is a signed integer plus an explicit unit.
///
/// Signed values are permitted because relative timing calculations may require
/// signed intermediate values. Validation must reject negative values wherever
/// the consuming semantic field requires a non-negative duration/time point.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct SerializedTime {
    /// Numeric magnitude.
    pub value: i128,

    /// Unit used by the value.
    pub unit: TimeUnit,
}

impl SerializedTime {
    /// Constructs a serialized time value.
    pub const fn new(value: i128, unit: TimeUnit) -> Self {
        Self { value, unit }
    }
}

/// Explicit time units.
///
/// No hardware timing grid is hard-coded here.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimeUnit {
    /// Seconds.
    Seconds,

    /// Milliseconds.
    Milliseconds,

    /// Microseconds.
    Microseconds,

    /// Nanoseconds.
    Nanoseconds,

    /// Picoseconds.
    Picoseconds,

    /// Femtoseconds.
    Femtoseconds,

    /// Target-defined timing ticks.
    ///
    /// The target identity and timing-model metadata must identify what one
    /// tick means.
    TargetTicks,

    /// Arbitrary rational timing representation identified externally.
    Rational,
}

/// Serialized time interval.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerializedTimeInterval {
    /// Inclusive/exact start according to scheduling semantics.
    pub start: SerializedTime,

    /// End of the interval.
    pub end: SerializedTime,
}

impl SerializedTimeInterval {
    /// Constructs an interval.
    pub const fn new(start: SerializedTime, end: SerializedTime) -> Self {
        Self { start, end }
    }
}

// =============================================================================
// Schema envelope
// =============================================================================

/// Top-level serialized scheduling document.
///
/// This is the stable root object passed between encode/decode layers.
///
/// It intentionally stores scheduler data as separate sections so future
/// schema additions can be introduced without changing the ownership of the
/// scheduling domain objects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchedulingDocument {
    /// Schema version.
    pub schema: SchemaVersion,

    /// Document kind.
    pub kind: DocumentKind,

    /// Stable document identity.
    pub document_id: SerializedId,

    /// Creation metadata.
    pub metadata: DocumentMetadata,

    /// Serialized schedule.
    pub schedule: SerializedSchedule,

    /// Optional verification information.
    pub verification: Option<SerializedVerification>,

    /// Optional reproducibility information.
    pub reproducibility: Option<ReproducibilityMetadata>,
}

/// Supported scheduling document kinds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DocumentKind {
    /// Complete schedule.
    Schedule,

    /// Schedule plus verification result.
    VerifiedSchedule,

    /// Schedule intended for transport between scheduling components.
    SchedulingArtifact,

    /// A diagnostic snapshot rather than an executable schedule.
    DiagnosticSnapshot,
}

/// Top-level document metadata.
///
/// All fields are optional because metadata availability depends on the
/// compilation/execution boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentMetadata {
    /// Optional stable human-readable name.
    pub name: Option<String>,

    /// Optional Zamani language version.
    pub zamani_version: Option<String>,

    /// Optional compiler version.
    pub compiler_version: Option<String>,

    /// Optional Quantum IR version.
    pub quantum_ir_version: Option<String>,

    /// Optional scheduler implementation version.
    pub scheduler_version: Option<String>,

    /// Optional target identifier.
    pub target_id: Option<String>,

    /// Optional target technology.
    pub target_technology: Option<String>,

    /// Optional hardware/provider identifier.
    pub provider_id: Option<String>,

    /// Optional calibration snapshot identifier.
    pub calibration_id: Option<String>,

    /// Optional routing artifact identifier.
    pub routing_artifact_id: Option<String>,

    /// Optional QEC artifact identifier.
    pub qec_artifact_id: Option<String>,

    /// Optional ZQN/noise-model identifier.
    pub zqn_model_id: Option<String>,
}

// =============================================================================
// Schedule
// =============================================================================

/// Serialized schedule.
///
/// The schedule is an ordered collection of scheduled operations plus the
/// resource reservations required to execute them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedSchedule {
    /// Stable schedule identity.
    pub schedule_id: SerializedId,

    /// Optional source program identity.
    pub source_program_id: Option<SerializedId>,

    /// Optional source circuit identity.
    pub source_circuit_id: Option<SerializedId>,

    /// Scheduled operations.
    pub operations: Vec<SerializedScheduledOperation>,

    /// Resource reservations.
    pub reservations: Vec<SerializedReservation>,

    /// Dependency edges represented in the schedule.
    pub dependencies: Vec<SerializedDependency>,

    /// Schedule-level timing information.
    pub timing: SerializedScheduleTiming,

    /// Target information used to produce the schedule.
    pub target: SerializedTargetReference,

    /// Scheduling policy metadata.
    pub policy: SerializedPolicyReference,

    /// Objective metadata.
    pub objective: SerializedObjectiveReference,
}

/// Scheduled operation.
///
/// This structure records scheduling information without redefining the
/// canonical quantum operation itself.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedScheduledOperation {
    /// Canonical source operation identity.
    pub operation_id: SerializedOperationId,

    /// Logical qubit operands.
    ///
    /// These refer to canonical `quantum::ir::qubit::QubitId` identities in
    /// serialized form.
    pub logical_qubits: Vec<SerializedQubitId>,

    /// Physical qubit operands, if mapping has occurred.
    ///
    /// These refer to canonical
    /// `quantum::ir::qubit::PhysicalQubitId` identities.
    pub physical_qubits: Vec<SerializedPhysicalQubitId>,

    /// Operation classification.
    pub class: SerializedOperationClass,

    /// Optional operation start.
    pub start: SerializedTime,

    /// Operation duration.
    pub duration: SerializedTime,

    /// Operation completion time.
    pub end: SerializedTime,

    /// Resources required by the operation.
    pub resource_requirements: Vec<SerializedResourceRequirement>,

    /// Optional runtime condition.
    pub condition: Option<SerializedCondition>,

    /// Optional operation provenance.
    pub provenance: Option<SerializedProvenance>,

    /// Optional scheduler metadata.
    pub metadata: Vec<SerializedKeyValue>,
}

/// Scheduling operation classes.
///
/// This is scheduling classification only. It is NOT a second quantum gate
/// model.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SerializedOperationClass {
    /// Ordinary quantum operation.
    Quantum,

    /// Measurement.
    Measurement,

    /// Reset.
    Reset,

    /// Barrier/synchronization.
    Synchronization,

    /// Delay/padding.
    Delay,

    /// Classical computation.
    Classical,

    /// Classical feedback.
    Feedback,

    /// Communication.
    Communication,

    /// QEC operation.
    ErrorCorrection,

    /// Target-defined operation.
    Custom,
}

// =============================================================================
// Dependencies
// =============================================================================

/// Serialized dependency edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedDependency {
    /// Stable dependency identity.
    pub dependency_id: SerializedDependencyId,

    /// Predecessor operation.
    pub predecessor: SerializedOperationId,

    /// Successor operation.
    pub successor: SerializedOperationId,

    /// Dependency kind.
    pub kind: SerializedDependencyKind,

    /// Optional latency between predecessor completion and successor readiness.
    pub latency: Option<SerializedTime>,

    /// Whether the dependency is resolved only at runtime.
    pub dynamic: bool,
}

/// Dependency kinds understood by the scheduling layer.
///
/// These do not redefine quantum semantics; they classify scheduling
/// precedence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SerializedDependencyKind {
    /// Quantum data dependency.
    QuantumData,

    /// Classical data dependency.
    ClassicalData,

    /// Measurement dependency.
    Measurement,

    /// Control/condition dependency.
    Control,

    /// Reset dependency.
    Reset,

    /// Resource-induced ordering.
    Resource,

    /// Communication dependency.
    Communication,

    /// QEC dependency.
    ErrorCorrection,

    /// User/compiler-defined precedence.
    Custom,
}

// =============================================================================
// Resources
// =============================================================================

/// Serialized resource reservation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedReservation {
    /// Stable reservation identity.
    pub reservation_id: SerializedReservationId,

    /// Resource being reserved.
    pub resource_id: SerializedResourceId,

    /// Operation consuming the resource.
    pub operation_id: SerializedOperationId,

    /// Reserved interval.
    pub interval: SerializedTimeInterval,

    /// Reservation mode.
    pub mode: SerializedReservationMode,

    /// Amount/capacity consumed.
    ///
    /// This is a quantity, not a fixed hardware maximum.
    pub quantity: SerializedQuantity,
}

/// Serialized resource requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedResourceRequirement {
    /// Resource identity.
    pub resource_id: SerializedResourceId,

    /// Required quantity.
    pub quantity: SerializedQuantity,

    /// Reservation mode.
    pub mode: SerializedReservationMode,
}

/// Resource reservation mode.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SerializedReservationMode {
    /// Resource is exclusively occupied.
    Exclusive,

    /// Resource can be shared up to capacity.
    Shared,

    /// Resource is consumed and later replenished by its owner.
    Consumable,

    /// Resource is reusable after the reservation interval.
    Reusable,
}

/// Explicit quantity.
///
/// Quantities are not constrained by this schema.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SerializedQuantity(pub u64);

// =============================================================================
// Conditions / dynamic scheduling
// =============================================================================

/// Serialized runtime condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedCondition {
    /// Classical dependency IDs.
    pub dependencies: Vec<SerializedClassicalDependencyId>,

    /// Opaque expression representation.
///
/// The canonical classical expression remains owned by the canonical IR.
    pub expression: String,
}

/// Serialized classical dependency.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedClassicalDependency {
    /// Stable dependency identity.
    pub id: SerializedClassicalDependencyId,

    /// Producing operation.
    pub producer: SerializedOperationId,

    /// Optional readiness latency.
    pub readiness_latency: Option<SerializedTime>,
}

// =============================================================================
// Timing
// =============================================================================

/// Schedule-level timing summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedScheduleTiming {
    /// Earliest scheduled time.
    pub start: Option<SerializedTime>,

    /// Final completion time.
    pub end: Option<SerializedTime>,

    /// Total makespan.
    pub makespan: Option<SerializedTime>,

    /// Number of timing layers/depth units if calculated.
    ///
    /// This is an observed metric, not a machine-size bound.
    pub depth: Option<u64>,

    /// Timing resolution used by the target.
    pub resolution: Option<SerializedTimingResolution>,

    /// Alignment requirements applied to the schedule.
    pub alignment: Vec<SerializedAlignment>,
}

/// Timing resolution.
///
/// The actual target timing contract remains owned by the hardware subsystem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedTimingResolution {
    /// Unit used for the resolution.
    pub unit: TimeUnit,

    /// Numerator of the resolution.
    pub numerator: u64,

    /// Denominator of the resolution.
    pub denominator: u64,

    /// Optional target-defined name.
    pub target_name: Option<String>,
}

/// Timing alignment requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedAlignment {
    /// Resource or operation to which the alignment applies.
    pub subject_id: SerializedId,

    /// Alignment period.
    pub period: SerializedTime,

    /// Optional phase offset.
    pub phase: Option<SerializedTime>,
}

// =============================================================================
// Target reference
// =============================================================================

/// Reference to the compilation target.
///
/// This is intentionally descriptive rather than a serialized hardware
/// implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedTargetReference {
    /// Stable target identity.
    pub target_id: String,

    /// Optional provider identity.
    pub provider_id: Option<String>,

    /// Optional technology name.
    pub technology: Option<String>,

    /// Optional topology artifact identity.
    pub topology_id: Option<String>,

    /// Optional capability artifact identity.
    pub capabilities_id: Option<String>,

    /// Optional timing-model artifact identity.
    pub timing_model_id: Option<String>,

    /// Optional resource-model artifact identity.
    pub resource_model_id: Option<String>,

    /// Optional availability snapshot identity.
    pub availability_id: Option<String>,
}

// =============================================================================
// Policies / objectives
// =============================================================================

/// Serialized scheduling policy reference.
///
/// The concrete policy remains owned by `scheduling::policies`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedPolicyReference {
    /// Stable policy name.
    pub name: String,

    /// Optional policy version.
    pub version: Option<String>,

    /// Optional policy-specific configuration encoded as key/value metadata.
    pub parameters: Vec<SerializedKeyValue>,
}

/// Serialized optimization objective.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedObjectiveReference {
    /// Objective name.
    pub name: String,

    /// Optional objective version.
    pub version: Option<String>,

    /// Objective direction.
    pub direction: SerializedObjectiveDirection,

    /// Optional objective weights.
    pub weights: Vec<SerializedObjectiveWeight>,
}

/// Objective direction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SerializedObjectiveDirection {
    /// Lower score is better.
    Minimize,

    /// Higher score is better.
    Maximize,
}

/// Weight belonging to a multi-objective scheduling configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedObjectiveWeight {
    /// Objective component name.
    pub name: String,

    /// Fixed-point representation.
///
/// The scale is supplied explicitly so no floating-point serialization is
/// required for deterministic persistence.
    pub numerator: i128,

    /// Positive scale denominator.
    pub denominator: u64,
}

// =============================================================================
// Verification
// =============================================================================

/// Serialized verification report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedVerification {
    /// Overall verification status.
    pub status: SerializedVerificationStatus,

    /// Individual verification checks.
    pub checks: Vec<SerializedVerificationCheck>,

    /// Optional verifier version.
    pub verifier_version: Option<String>,
}

/// Verification status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SerializedVerificationStatus {
    /// All required checks passed.
    Passed,

    /// One or more checks failed.
    Failed,

    /// Verification was not complete.
    Incomplete,

    /// Verification was intentionally skipped.
    Skipped,
}

/// Individual verification check.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedVerificationCheck {
    /// Check name.
    pub name: String,

    /// Check result.
    pub status: SerializedVerificationCheckStatus,

    /// Optional diagnostic.
    pub message: Option<String>,

    /// Optional operation involved.
    pub operation_id: Option<SerializedOperationId>,

    /// Optional resource involved.
    pub resource_id: Option<SerializedResourceId>,
}

/// Individual verification result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SerializedVerificationCheckStatus {
    Passed,
    Failed,
    Skipped,
}

// =============================================================================
// Provenance
// =============================================================================

/// Serialized operation provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedProvenance {
    /// Source operation identity.
    pub source_operation_id: Option<SerializedOperationId>,

    /// Source file/document identity.
    pub source_document_id: Option<SerializedId>,

    /// Optional source location.
    pub source_location: Option<SerializedSourceLocation>,

    /// Transformation chain.
    pub transformations: Vec<SerializedTransformation>,
}

/// Source location.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedSourceLocation {
    /// Optional source URI/path.
    pub source: Option<String>,

    /// One-based line.
    pub line: Option<u64>,

    /// One-based column.
    pub column: Option<u64>,
}

/// Transformation applied before or during scheduling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedTransformation {
    /// Transformation name.
    pub name: String,

    /// Optional implementation version.
    pub version: Option<String>,

    /// Optional transformation identifier.
    pub id: Option<SerializedId>,
}

// =============================================================================
// Reproducibility
// =============================================================================

/// Metadata needed to reproduce a scheduling decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReproducibilityMetadata {
    /// Whether deterministic mode was requested.
    pub deterministic: bool,

    /// Optional explicit random seed.
    pub seed: Option<u64>,

    /// Optional canonical input digest.
    pub input_digest: Option<String>,

    /// Optional target snapshot digest.
    pub target_digest: Option<String>,

    /// Optional scheduler configuration digest.
    pub configuration_digest: Option<String>,

    /// Optional complete compilation artifact digest.
    pub artifact_digest: Option<String>,
}

// =============================================================================
// Generic metadata
// =============================================================================

/// Generic metadata entry.
///
/// This is intentionally simple and extensible. Domain semantics must not be
/// encoded through arbitrary keys when a dedicated schema field is required.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerializedKeyValue {
    /// Metadata key.
    pub key: String,

    /// Metadata value.
    pub value: String,
}

// =============================================================================
// Schema limits supplied by the decoder
// =============================================================================

/// Explicit decoder resource limits.
///
/// These are caller-supplied safety controls, NOT schema-level machine limits.
///
/// `None` means that this particular limit was not imposed by the caller.
///
/// The schema never supplies an implicit maximum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaDecodeLimits {
    /// Maximum number of operations the caller is willing to materialize.
    pub max_operations: Option<u64>,

    /// Maximum number of dependencies.
    pub max_dependencies: Option<u64>,

    /// Maximum number of reservations.
    pub max_reservations: Option<u64>,

    /// Maximum number of resources referenced.
    pub max_resources: Option<u64>,

    /// Maximum number of logical qubits referenced.
    pub max_logical_qubits: Option<u64>,

    /// Maximum number of physical qubits referenced.
    pub max_physical_qubits: Option<u64>,

    /// Maximum number of metadata entries.
    pub max_metadata_entries: Option<u64>,

    /// Maximum serialized string length in bytes.
    pub max_string_bytes: Option<u64>,
}

impl Default for SchemaDecodeLimits {
    fn default() -> Self {
        Self {
            max_operations: None,
            max_dependencies: None,
            max_reservations: None,
            max_resources: None,
            max_logical_qubits: None,
            max_physical_qubits: None,
            max_metadata_entries: None,
            max_string_bytes: None,
        }
    }
}

// =============================================================================
// Validation error vocabulary
// =============================================================================

/// Schema-level validation error.
///
/// This type describes malformed schema data. Semantic scheduler errors remain
/// owned by `scheduling::errors`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SchemaValidationError {
    /// Unsupported major schema version.
    UnsupportedMajorVersion {
        supported: SchemaVersion,
        encountered: SchemaVersion,
    },

    /// Invalid version relationship.
    InvalidVersion,

    /// Required string was empty.
    EmptyRequiredField {
        field: &'static str,
    },

    /// Required identifier was invalid for the consuming contract.
    InvalidIdentifier {
        field: &'static str,
        value: u64,
    },

    /// Invalid time interval.
    InvalidInterval {
        field: &'static str,
    },

    /// End precedes start.
    EndBeforeStart {
        field: &'static str,
    },

    /// Zero denominator.
    ZeroDenominator {
        field: &'static str,
    },

    /// Invalid objective weight.
    InvalidObjectiveWeight {
        field: &'static str,
    },

    /// Cross-reference points at a nonexistent operation.
    MissingOperationReference {
        operation_id: SerializedOperationId,
    },

    /// Cross-reference points at a nonexistent resource.
    MissingResourceReference {
        resource_id: SerializedResourceId,
    },

    /// Duplicate semantic identity where uniqueness is required.
    DuplicateIdentity {
        id: SerializedId,
    },

    /// A caller-provided decode limit was exceeded.
    DecodeLimitExceeded {
        field: &'static str,
        limit: u64,
    },
}

impl fmt::Display for SchemaValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMajorVersion {
                supported,
                encountered,
            } => write!(
                formatter,
                "unsupported scheduling schema major version: supported {}, encountered {}",
                supported, encountered
            ),

            Self::InvalidVersion => {
                write!(formatter, "invalid scheduling schema version")
            }

            Self::EmptyRequiredField { field } => {
                write!(formatter, "required schema field `{field}` is empty")
            }

            Self::InvalidIdentifier { field, value } => {
                write!(
                    formatter,
                    "invalid identifier `{value}` for schema field `{field}`"
                )
            }

            Self::InvalidInterval { field } => {
                write!(formatter, "invalid time interval in `{field}`")
            }

            Self::EndBeforeStart { field } => {
                write!(formatter, "end precedes start in `{field}`")
            }

            Self::ZeroDenominator { field } => {
                write!(formatter, "zero denominator in `{field}`")
            }

            Self::InvalidObjectiveWeight { field } => {
                write!(formatter, "invalid objective weight in `{field}`")
            }

            Self::MissingOperationReference { operation_id } => {
                write!(
                    formatter,
                    "missing operation reference {}",
                    operation_id.get()
                )
            }

            Self::MissingResourceReference { resource_id } => {
                write!(
                    formatter,
                    "missing resource reference {}",
                    resource_id.get()
                )
            }

            Self::DuplicateIdentity { id } => {
                write!(formatter, "duplicate semantic identity {}", id.get())
            }

            Self::DecodeLimitExceeded { field, limit } => {
                write!(
                    formatter,
                    "schema decode limit exceeded for `{field}`: {}",
                    limit
                )
            }
        }
    }
}

impl std::error::Error for SchemaValidationError {}

// =============================================================================
// Lightweight structural validation
// =============================================================================

impl SerializedTimeInterval {
    /// Validates interval ordering.
    ///
    /// This method intentionally does not perform unit conversion. Unit
    /// normalization belongs to the timing subsystem.
    pub fn validate(&self) -> Result<(), SchemaValidationError> {
        if self.start.unit != self.end.unit {
            return Err(SchemaValidationError::InvalidInterval {
                field: "interval",
            });
        }

        if self.end.value < self.start.value {
            return Err(SchemaValidationError::EndBeforeStart {
                field: "interval",
            });
        }

        Ok(())
    }
}

impl SerializedObjectiveWeight {
    /// Validates an objective weight.
    pub fn validate(&self) -> Result<(), SchemaValidationError> {
        if self.denominator == 0 {
            return Err(SchemaValidationError::ZeroDenominator {
                field: "objective.weight.denominator",
            });
        }

        Ok(())
    }
}

impl SerializedTimingResolution {
    /// Validates the rational timing resolution representation.
    pub fn validate(&self) -> Result<(), SchemaValidationError> {
        if self.denominator == 0 {
            return Err(SchemaValidationError::ZeroDenominator {
                field: "timing.resolution.denominator",
            });
        }

        if self.numerator == 0 {
            return Err(SchemaValidationError::InvalidInterval {
                field: "timing.resolution",
            });
        }

        Ok(())
    }
}

impl SchedulingDocument {
    /// Validates the schema version against the currently supported major
    /// version.
    ///
    /// This is deliberately structural validation only. Full schedule
    /// verification belongs to `scheduling::verification`.
    pub fn validate_schema(&self) -> Result<(), SchemaValidationError> {
        if self.schema.major != SCHEDULING_SCHEMA_MAJOR {
            return Err(SchemaValidationError::UnsupportedMajorVersion {
                supported: SCHEDULING_SCHEMA_VERSION,
                encountered: self.schema,
            });
        }

        if self.metadata.target_id.as_deref() == Some("") {
            return Err(SchemaValidationError::EmptyRequiredField {
                field: "metadata.target_id",
            });
        }

        if let Some(resolution) = self.schedule.timing.resolution {
            resolution.validate()?;
        }

        for alignment in &self.schedule.timing.alignment {
            if alignment.period.value <= 0 {
                return Err(SchemaValidationError::InvalidInterval {
                    field: "timing.alignment.period",
                });
            }

            if let Some(phase) = alignment.phase {
                if phase.unit != alignment.period.unit {
                    return Err(SchemaValidationError::InvalidInterval {
                        field: "timing.alignment.phase",
                    });
                }
            }
        }

        for operation in &self.schedule.operations {
            if operation.duration.value < 0 {
                return Err(SchemaValidationError::InvalidInterval {
                    field: "operation.duration",
                });
            }

            if operation.start.unit != operation.duration.unit
                || operation.end.unit != operation.start.unit
            {
                return Err(SchemaValidationError::InvalidInterval {
                    field: "operation",
                });
            }

            if operation.end.value < operation.start.value {
                return Err(SchemaValidationError::EndBeforeStart {
                    field: "operation",
                });
            }
        }

        for reservation in &self.schedule.reservations {
            reservation.interval.validate()?;
        }

        for weight in &self.objective_weights() {
            weight.validate()?;
        }

        Ok(())
    }

    fn objective_weights(&self) -> Vec<&SerializedObjectiveWeight> {
        self.schedule
            .objective
            .weights
            .iter()
            .collect()
    }
}

// =============================================================================
// Decode-limit helpers
// =============================================================================

impl SchemaDecodeLimits {
    /// Checks an operation count against the caller's explicit limit.
    pub fn check_operations(
        &self,
        count: usize,
    ) -> Result<(), SchemaValidationError> {
        self.check_count("operations", count, self.max_operations)
    }

    /// Checks a dependency count.
    pub fn check_dependencies(
        &self,
        count: usize,
    ) -> Result<(), SchemaValidationError> {
        self.check_count("dependencies", count, self.max_dependencies)
    }

    /// Checks a reservation count.
    pub fn check_reservations(
        &self,
        count: usize,
    ) -> Result<(), SchemaValidationError> {
        self.check_count("reservations", count, self.max_reservations)
    }

    fn check_count(
        &self,
        field: &'static str,
        count: usize,
        limit: Option<u64>,
    ) -> Result<(), SchemaValidationError> {
        if let Some(limit) = limit {
            if (count as u64) > limit {
                return Err(SchemaValidationError::DecodeLimitExceeded {
                    field,
                    limit,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Compile-time schema assertions
// =============================================================================

const _: () = {
    assert!(SCHEDULING_SCHEMA_MAJOR > 0);
};

// =============================================================================
// Integration notes for serialization/encode.rs
// =============================================================================
//
// `encode.rs` must:
//
// 1. accept `SchedulingDocument`;
// 2. never infer machine limits from this schema;
// 3. emit `SCHEDULING_SCHEMA_VERSION` unless explicitly encoding a migration;
// 4. serialize semantic IDs as their stable numeric representation;
// 5. preserve canonical logical/physical qubit identity values;
// 6. apply deterministic ordering where required;
// 7. never serialize Rust memory addresses or pointers;
// 8. never serialize `usize` as a semantic identity;
// 9. never serialize vendor SDK objects;
// 10. never silently truncate identifiers or quantities;
// 11. reject impossible numeric conversions;
// 12. preserve optional fields according to the selected wire format;
// 13. include enough metadata for reproducibility when available;
// 14. remain independent of a particular scheduler algorithm.
//
// The encoder must not need to change merely because a new scheduling planner
// is added.
//
// =============================================================================
// Integration notes for serialization/decode.rs
// =============================================================================
//
// `decode.rs` must:
//
// 1. parse the schema envelope;
// 2. reject unsupported major versions;
// 3. apply caller-provided `SchemaDecodeLimits`;
// 4. validate all structural fields;
// 5. validate cross references;
// 6. reject malformed intervals;
// 7. reject zero denominators;
// 8. reject integer overflow during representation conversion;
// 9. reconstruct canonical `quantum::ir::qubit::QubitId` values through the
//    canonical IR API rather than defining replacement qubit types;
// 10. reconstruct canonical
//     `quantum::ir::qubit::PhysicalQubitId` values through the canonical IR API;
// 11. never assume that an ID is a vector index;
// 12. never silently discard unknown required semantics;
// 13. return structured schema errors;
// 14. pass the validated representation to the scheduler adapters;
// 15. leave semantic schedule verification to `scheduling::verification`.
//
// =============================================================================
// Integration notes for scheduling/ir
// =============================================================================
//
// `scheduling::ir` remains the owner of the scheduler's internal operation and
// dependency representation.
//
// The schema stores references to those objects but does not redefine them.
//
// In particular:
//
//     quantum::ir::qubit::QubitId
//     quantum::ir::qubit::PhysicalQubitId
//
// remain authoritative.
//
// This is consistent with the repository's existing scheduling IR contract,
// which explicitly prohibits duplicate qubit identity definitions.
//
// =============================================================================
// Integration notes for resources
// =============================================================================
//
// `scheduling::resources` owns:
//
// - resource semantics;
// - capacity;
// - reservation behavior;
// - resource availability.
//
// This schema stores serialized resource IDs and reservation facts only.
//
// It does not define:
//
//     MAX_RESOURCES
//     MAX_CHANNELS
//     MAX_QUBITS
//
// or any equivalent fixed hardware constant.
//
// =============================================================================
// Integration notes for timing
// =============================================================================
//
// `scheduling::timing` owns:
//
// - duration semantics;
// - time arithmetic;
// - target timing resolution;
// - alignment;
// - timing constraints.
//
// This schema stores explicit serialized timing values.
//
// Unit conversion belongs to the timing subsystem.
//
// =============================================================================
// Integration notes for hardware
// =============================================================================
//
// `scheduling::adapters::hardware` converts target/hardware capabilities into
// scheduler-owned timing/resource information.
//
// This schema must never import a hardware backend.
//
// Vendor-specific information belongs in target metadata or the owning
// hardware subsystem.
//
// =============================================================================
// Integration notes for routing
// =============================================================================
//
// Routing answers:
//
//     WHERE?
//
// Scheduling answers:
//
//     WHEN?
//
// The schema preserves both logical and physical qubit references so a
// serialized schedule can retain the mapping provenance without making the
// scheduler the owner of routing.
//
// Logical identity:
//
//     quantum::ir::qubit::QubitId
//
// Physical identity:
//
//     quantum::ir::qubit::PhysicalQubitId
//
// =============================================================================
// Integration notes for QEC
// =============================================================================
//
// QEC may identify operations as:
//
//     SerializedOperationClass::ErrorCorrection
//
// and dependencies as:
//
//     SerializedDependencyKind::ErrorCorrection
//
// This schema does not implement stabilizer codes, syndrome extraction,
// decoders, surface-code geometry, or fixed QEC distances.
//
// =============================================================================
// Integration notes for ZQN
// =============================================================================
//
// ZQN/noise information may be referenced through:
//
//     DocumentMetadata::zqn_model_id
//
// and target metadata.
//
// The schema does not embed a noise model.
//
// This prevents scheduling serialization from becoming coupled to a particular
// noise representation.
//
// =============================================================================
// Integration notes for verification
// =============================================================================
//
// A schedule can optionally contain `SerializedVerification`.
//
// The presence of a verification report does NOT itself prove that the schedule
// is safe.
//
// The verifier remains authoritative for semantic validation.
//
// A decoder must therefore not interpret:
//
//     status = Passed
//
// as permission to bypass its own structural checks.
//
// =============================================================================
// Integration notes for benchmarking
// =============================================================================
//
// The benchmark subsystem can consume:
//
// - makespan;
// - depth;
// - reservations;
// - resource utilization derived from reservations;
// - operation timing;
// - verification status;
// - reproducibility metadata.
//
// Benchmarking must not modify the schema's semantic ownership.
//
// =============================================================================
// Integration notes for diagnostics
// =============================================================================
//
// Diagnostics may use:
//
//     SerializedKeyValue
//
// and provenance information.
//
// Machine-readable diagnostic contracts that become stable scheduler semantics
// should eventually receive dedicated schema fields rather than accumulating
// arbitrary keys.
//
// =============================================================================
// Integration notes for distributed scheduling
// =============================================================================
//
// Distributed scheduling may represent communication operations as:
//
//     SerializedOperationClass::Communication
//
// and dependencies as:
//
//     SerializedDependencyKind::Communication
//
// Resource reservations can reference communication resources.
//
// The schema contains no fixed node count, link count, topology dimension, or
// network size.
//
// =============================================================================
// Forward-compatibility rule
// =============================================================================
//
// Future schema versions may add fields such as:
//
// - distributed execution domains;
// - richer dynamic-circuit conditions;
// - pulse-level scheduling metadata;
// - fault-tolerant logical-resource mappings;
// - quantum-network synchronization;
// - heterogeneous timing domains;
// - new optimization metrics.
//
// Such additions should normally be additive within the same major version.
//
// Existing fields must retain their documented semantics.
//
// =============================================================================
// Migration rule
// =============================================================================
//
// A future incompatible schema must introduce a new major version:
//
//     1.x.x
//       │
//       ▼
//     2.x.x
//
// Migration belongs in a dedicated migration layer, not in this schema file.
//
// The schema file must remain the definition of the destination/source data
// contracts, not a growing collection of historical migration algorithms.
//
// =============================================================================
// Testing contract
// =============================================================================
//
// Unit tests for this file should cover:
//
// 1. schema-version construction;
// 2. schema-version comparison;
// 3. compatibility classification;
// 4. interval validation;
// 5. resolution validation;
// 6. objective-weight validation;
// 7. malformed operation timing;
// 8. caller decode limits;
// 9. empty required metadata;
// 10. large identifiers;
// 11. zero identifiers where permitted;
// 12. deterministic equality;
//
// Property tests should verify that valid serialized intervals never report
// `EndBeforeStart` and that invalid interval ordering is always rejected.
//
// Integration tests belong under:
//
//     scheduling/tests/integration/
//     scheduling/tests/determinism/
//     scheduling/tests/scalability/
//
// and should exercise:
//
//     canonical IR
//          ↓
//     routing
//          ↓
//     scheduling
//          ↓
//     ScheduleResult
//          ↓
//     SchedulingDocument
//          ↓
//     encode
//          ↓
//     decode
//          ↓
//     canonical/scheduler representation
//
// =============================================================================
// Production invariants
// =============================================================================
//
// The following invariants are mandatory:
//
// 1. No `unsafe`.
// 2. No machine-size constants.
// 3. No fixed qubit count.
// 4. No fixed resource count.
// 5. No fixed operation count.
// 6. No fixed schedule depth.
// 7. No fixed QEC distance.
// 8. No fixed communication-node count.
// 9. No vendor-specific types.
// 10. No hardware I/O.
// 11. No scheduler algorithm implementation.
// 12. No duplicate canonical qubit identity.
// 13. No `usize` semantic identities.
// 14. No pointer serialization.
// 15. No silent integer truncation.
// 16. No implicit timing-unit conversion.
// 17. No implicit compatibility across major versions.
// 18. No hidden decode resource limits.
// 19. No executable behavior during schema construction.
// 20. No global mutable state.
// 21. Schema representation remains independent of scheduler policy.
// 22. Schema representation remains independent of routing implementation.
// 23. Schema representation remains independent of QEC implementation.
// 24. Schema representation remains independent of hardware provider.
// 25. Schema representation remains usable for tiny and extremely large
//     schedules, subject only to actual available resources and explicit caller
//     limits.
//
// =============================================================================
// End of schema contract
// =============================================================================