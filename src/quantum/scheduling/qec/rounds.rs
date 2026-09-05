//! Zamani Quantum Scheduling — QEC Round Model
//!
//! This module defines the scheduler-independent representation of quantum
//! error-correction (QEC) rounds.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "What constitutes one QEC round, what other rounds must precede it,
//! > which canonical qubits and operations participate, and what temporal
//! > requirements constrain the round?"
//!
//! It does NOT decide:
//!
//! - how a round is physically scheduled;
//! - which physical machine is used;
//! - how logical qubits are routed;
//! - how stabilizers are synthesized;
//! - how syndrome data is decoded;
//! - how hardware is contacted;
//! - how pulses are generated;
//! - how resources are discovered;
//! - how a scheduler chooses ASAP/ALAP/list scheduling;
//! - how a QEC code is decoded.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! quantum::ir
//! quantum::routing
//! quantum::hardware
//! quantum::scheduling::planners
//! quantum::scheduling::constraints
//! quantum::scheduling::qec::syndrome
//! quantum::scheduling::qec::stabilizer
//! quantum::runtime
//! ```
//!
//! # Why this file exists
//!
//! A QEC round is not itself a schedule.
//!
//! A round is a semantic scheduling unit containing:
//!
//! - stable round identity;
//! - deterministic sequence position;
//! - participating logical qubits;
//! - participating physical qubits when mapping is already known;
//! - canonical IR operations belonging to the round;
//! - dependencies on previous rounds;
//! - timing requirements;
//! - round-level metadata;
//! - optional repetition information.
//!
//! The generic scheduler later converts those requirements into actual
//! operation start times and resource reservations.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::qubit
//! quantum::ir::core::identity
//!          │
//!          ▼
//! scheduling::types
//!          │
//!          ▼
//! scheduling::qec::rounds
//!          │
//!     ┌────┼──────────────┐
//!     ▼    ▼              ▼
//! syndrome stabilizer   constraints
//!          │              │
//!          └──────┬───────┘
//!                 ▼
//!             planners
//!                 │
//!                 ▼
//!              schedule
//! ```
//!
//! This module deliberately has no dependency on a planner or scheduler
//! algorithm. That keeps it independently implementable and stable.
//!
//! # Canonical identity rule
//!
//! Logical and physical qubits MUST use the canonical IR identities:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Operations MUST use:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! This file MUST NOT define another `QubitId`, `PhysicalQubitId`, or
//! `OperationId`.
//!
//! # No machine-size assumptions
//!
//! This module intentionally contains no:
//!
//! - maximum QEC distance;
//! - maximum number of rounds;
//! - fixed number of ancillas;
//! - fixed number of stabilizers;
//! - fixed number of data qubits;
//! - fixed number of syndrome measurements;
//! - fixed number of operations;
//! - fixed topology;
//! - fixed operation arity;
//! - fixed hardware architecture;
//! - fixed number of QPUs.
//!
//! A round may contain any number of qubits and operations permitted by the
//! concrete compilation request and available resources.
//!
//! "Infinity" means that this module imposes no artificial finite machine-size
//! ceiling. A concrete Rust process remains bounded by available memory,
//! address space, CPU time, explicit compiler policy, and target resources.
//!
//! # Important scalability rule
//!
//! QEC systems frequently repeat structurally identical rounds.
//!
//! Therefore this module supports both:
//!
//! 1. explicitly represented rounds;
//! 2. compact repeated round specifications.
//!
//! A caller does NOT need to allocate one large `Vec<Round>` merely to express
//! a potentially very large repeated QEC sequence.
//!
//! Materialization, when required, belongs to the consumer and should be
//! bounded by explicit compilation/resource policy.
//!
//! # Determinism
//!
//! Collections preserve insertion order for semantic payloads while validation
//! uses deterministic checks.
//!
//! No wall-clock state, global mutable state, or hidden randomness is used.
//!
//! # Thread safety
//!
//! All structures are ordinary owned values with no interior mutability.
//!
//! They can therefore be moved between threads and shared through ordinary
//! Rust ownership mechanisms when wrapped appropriately by higher-level code.
//!
//! # Safety
//!
//! This module is safe Rust only.
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`;
//! - no external dependencies.
//!
//! The compiler-enforced attributes below make the no-unsafe requirement
//! explicit.
//!
//! # Integration contract
//!
//! `RoundSpec` is the principal type consumed by:
//!
//! ```text
//! qec/syndrome.rs
//! qec/stabilizer.rs
//! constraints/custom.rs
//! constraints/qubit.rs
//! constraints/measurement.rs
//! constraints/reset.rs
//! planners/*
//! adapters/qec.rs
//! verification/*
//! ```
//!
//! The generic scheduler MUST interpret round dependencies as precedence
//! constraints. It MUST NOT infer missing dependencies from round numbering.
//!
//! A round with ordinal `N` is not automatically dependent on ordinal `N-1`.
//!
//! This is intentional because QEC protocols can contain:
//!
//! - parallel rounds;
//! - branching;
//! - recovery barriers;
//! - asynchronous measurements;
//! - pipelined syndrome extraction;
//! - distributed QEC;
//! - repeated but partially overlapping procedures.
//!
//! Dependencies must therefore be explicit.
//!
//! # Integration with scheduling
//!
//! The intended flow is:
//!
//! ```text
//! QEC subsystem
//!      │
//!      ▼
//! RoundSpec / RoundPlan
//!      │
//!      ▼
//! qec adapter
//!      │
//!      ▼
//! scheduling constraints/dependencies
//!      │
//!      ▼
//! generic planner
//!      │
//!      ▼
//! ScheduleResult
//! ```
//!
//! `rounds.rs` never creates `ScheduledOperation` values and never assigns
//! actual start times to operations.
//!
//! # Integration with routing
//!
//! Logical qubits may be supplied without physical qubits.
//!
//! Physical qubits may be supplied when routing has already occurred.
//!
//! Both may be present when the QEC subsystem needs to retain provenance.
//!
//! The presence of a physical qubit in this structure does NOT mean routing is
//! owned by this module.
//!
//! # Integration with hardware
//!
//! Hardware-specific timing and resource information MUST NOT be encoded here.
//!
//! For example, this file must never assume:
//!
//! ```text
//! measurement = 1 microsecond
//! reset = 500 nanoseconds
//! four neighbours
//! eight channels
//! distance <= 100
//! ```
//!
//! Such values are supplied by the target/hardware adapter and transformed
//! into scheduling constraints.
//!
//! # Integration with QEC
//!
//! `RoundSpec` is deliberately code-family neutral.
//!
//! Surface-code, color-code, repetition-code, subsystem-code, LDPC,
//! bosonic-code, modular, networked, and future QEC systems can all express
//! their round-level scheduling requirements through the same representation.
//!
//! Code-specific semantics belong in `qec/stabilizer.rs`, `qec/syndrome.rs`,
//! or future specialized modules.
//!
//! # Validation philosophy
//!
//! This module validates only properties that can be checked locally without
//! knowing the target hardware or full schedule.
//!
//! It checks:
//!
//! - non-conflicting identity declarations;
//! - unique qubits within a round;
//! - unique physical qubits within a round;
//! - unique operations within a round;
//! - unique round dependencies;
//! - valid dependency self-reference;
//! - valid timing windows;
//! - valid repetition counts;
//! - deterministic round-plan consistency.
//!
//! It does NOT check:
//!
//! - whether hardware supports an operation;
//! - whether resources are available;
//! - whether two rounds overlap physically;
//! - whether a target can satisfy a timing requirement;
//! - whether a full schedule is semantically equivalent.
//!
//! Those checks belong to the appropriate scheduling/hardware/verification
//! subsystem.
//!
//! ============================================================================
//! Compiler-enforced safety boundary
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::HashSet;
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use super::super::types::{Duration, TimePoint};

// ============================================================================
// Round identity
// ============================================================================

/// Stable identity for a QEC round.
///
/// `RoundId` is a semantic identity, not a vector index and not a hardware
/// address.
///
/// The scheduler or QEC compiler owns allocation of identities.
///
/// Zero is a valid representable value; callers may choose to reserve it as
/// a sentinel in their own allocation policy, but this type does not impose
/// such a policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoundId(u64);

impl RoundId {
    /// Creates a round identity from an explicit stable value.
    ///
    /// This does not allocate or register the identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns whether the identity is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns the next representable identity.
    ///
    /// This does not allocate the returned identity.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for RoundId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<RoundId> for u64 {
    fn from(value: RoundId) -> Self {
        value.value()
    }
}

impl fmt::Display for RoundId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "round:{}", self.0)
    }
}

// ============================================================================
// Round ordinal
// ============================================================================

/// Logical sequence position of a QEC round.
///
/// An ordinal describes ordering metadata only.
///
/// It MUST NOT be interpreted as an implicit dependency.
///
/// For example, round 7 does not automatically depend on round 6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoundOrdinal(u64);

impl RoundOrdinal {
    /// First ordinal.
    pub const FIRST: Self = Self(0);

    /// Creates an ordinal.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric ordinal.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next representable ordinal.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for RoundOrdinal {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<RoundOrdinal> for u64 {
    fn from(value: RoundOrdinal) -> Self {
        value.value()
    }
}

impl fmt::Display for RoundOrdinal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// ============================================================================
// Round kind
// ============================================================================

/// Semantic category of a QEC round.
///
/// This classification is intentionally technology-neutral.
///
/// A code-specific implementation may use `Custom` without modifying the
/// scheduler core.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RoundKind {
    /// General syndrome-extraction round.
    SyndromeExtraction,

    /// Initialization/preparation round.
    Preparation,

    /// Recovery/correction round.
    Recovery,

    /// Measurement/readout round.
    Measurement,

    /// Synchronization/barrier round.
    Synchronization,

    /// Calibration-related round.
    Calibration,

    /// Verification/check round.
    Verification,

    /// Application-specific QEC round.
    Custom(String),
}

impl RoundKind {
    /// Returns a stable machine-readable classification.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::SyndromeExtraction => "syndrome_extraction",
            Self::Preparation => "preparation",
            Self::Recovery => "recovery",
            Self::Measurement => "measurement",
            Self::Synchronization => "synchronization",
            Self::Calibration => "calibration",
            Self::Verification => "verification",
            Self::Custom(value) => value.as_str(),
        }
    }
}

impl Default for RoundKind {
    fn default() -> Self {
        Self::SyndromeExtraction
    }
}

impl fmt::Display for RoundKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Round dependency kind
// ============================================================================

/// Semantic reason why one QEC round must follow another.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RoundDependencyKind {
    /// All predecessor work must complete first.
    Completion,

    /// A measurement result must be available before this round can proceed.
    Measurement,

    /// Classical processing/decoding must complete first.
    Classical,

    /// Recovery/correction must complete first.
    Recovery,

    /// Synchronization between round participants is required.
    Synchronization,

    /// A target/resource transition must complete first.
    Resource,

    /// Application-specific dependency.
    Custom,
}

impl RoundDependencyKind {
    /// Returns a stable machine-readable classification.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completion => "completion",
            Self::Measurement => "measurement",
            Self::Classical => "classical",
            Self::Recovery => "recovery",
            Self::Synchronization => "synchronization",
            Self::Resource => "resource",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for RoundDependencyKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Round dependency
// ============================================================================

/// Explicit precedence relationship between two QEC rounds.
///
/// Dependencies are explicit because ordinal position alone cannot correctly
/// represent parallel, pipelined, branching, or distributed QEC protocols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoundDependency {
    predecessor: RoundId,
    kind: RoundDependencyKind,
}

impl RoundDependency {
    /// Creates a dependency on a predecessor round.
    ///
    /// Self-dependencies are rejected during `validate`.
    #[must_use]
    pub const fn new(predecessor: RoundId, kind: RoundDependencyKind) -> Self {
        Self { predecessor, kind }
    }

    /// Returns the predecessor round.
    #[must_use]
    pub const fn predecessor(self) -> RoundId {
        self.predecessor
    }

    /// Returns the dependency kind.
    #[must_use]
    pub const fn kind(self) -> RoundDependencyKind {
        self.kind
    }
}

// ============================================================================
// Round timing requirements
// ============================================================================

/// Local temporal requirements belonging to a QEC round.
///
/// These are requirements, not a schedule.
///
/// Actual placement is owned by the generic scheduling planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RoundTiming {
    /// Earliest permitted start.
    release_time: Option<TimePoint>,

    /// Latest permitted completion.
    deadline: Option<TimePoint>,

    /// Minimum time the round itself must occupy when a duration is known.
    minimum_duration: Option<Duration>,

    /// Maximum permitted round duration.
    ///
    /// This is optional because some QEC protocols or abstract IR stages may
    /// not have a known upper bound until target lowering.
    maximum_duration: Option<Duration>,

    /// Minimum spacing after the completion of this round before the next
    /// dependent round may begin.
    minimum_successor_spacing: Option<Duration>,
}

impl RoundTiming {
    /// Creates unconstrained round timing.
    #[must_use]
    pub const fn unconstrained() -> Self {
        Self {
            release_time: None,
            deadline: None,
            minimum_duration: None,
            maximum_duration: None,
            minimum_successor_spacing: None,
        }
    }

    /// Sets an earliest start time.
    #[must_use]
    pub const fn with_release_time(mut self, value: TimePoint) -> Self {
        self.release_time = Some(value);
        self
    }

    /// Sets a completion deadline.
    #[must_use]
    pub const fn with_deadline(mut self, value: TimePoint) -> Self {
        self.deadline = Some(value);
        self
    }

    /// Sets a minimum duration.
    #[must_use]
    pub const fn with_minimum_duration(mut self, value: Duration) -> Self {
        self.minimum_duration = Some(value);
        self
    }

    /// Sets a maximum duration.
    #[must_use]
    pub const fn with_maximum_duration(mut self, value: Duration) -> Self {
        self.maximum_duration = Some(value);
        self
    }

    /// Sets the minimum spacing before a dependent successor.
    #[must_use]
    pub const fn with_minimum_successor_spacing(mut self, value: Duration) -> Self {
        self.minimum_successor_spacing = Some(value);
        self
    }

    /// Returns the earliest permitted start.
    #[must_use]
    pub const fn release_time(self) -> Option<TimePoint> {
        self.release_time
    }

    /// Returns the completion deadline.
    #[must_use]
    pub const fn deadline(self) -> Option<TimePoint> {
        self.deadline
    }

    /// Returns the minimum duration.
    #[must_use]
    pub const fn minimum_duration(self) -> Option<Duration> {
        self.minimum_duration
    }

    /// Returns the maximum duration.
    #[must_use]
    pub const fn maximum_duration(self) -> Option<Duration> {
        self.maximum_duration
    }

    /// Returns the minimum successor spacing.
    #[must_use]
    pub const fn minimum_successor_spacing(self) -> Option<Duration> {
        self.minimum_successor_spacing
    }

    /// Validates the local temporal requirements.
    ///
    /// This does not determine whether the target can satisfy them.
    pub fn validate(&self) -> Result<(), RoundError> {
        if let (Some(minimum), Some(maximum)) =
            (self.minimum_duration, self.maximum_duration)
        {
            if minimum > maximum {
                return Err(RoundError::InvalidTiming {
                    reason: "minimum duration exceeds maximum duration",
                });
            }
        }

        if let (Some(release), Some(deadline)) = (self.release_time, self.deadline) {
            if deadline < release {
                return Err(RoundError::InvalidTiming {
                    reason: "deadline precedes release time",
                });
            }
        }

        Ok(())
    }
}

impl Default for RoundTiming {
    fn default() -> Self {
        Self::unconstrained()
    }
}

// ============================================================================
// Repetition specification
// ============================================================================

/// Describes optional repetition of a structurally identical QEC round.
///
/// The count is intentionally represented as `u64` rather than `usize` so the
/// semantic model does not depend on host pointer width.
///
/// A count of zero means no instances are requested.
///
/// This structure does not materialize instances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoundRepetition {
    count: u64,
    ordinal_stride: u64,
}

impl RoundRepetition {
    /// Creates a repetition specification.
    ///
    /// `ordinal_stride` controls how ordinals advance between materialized
    /// instances.
    ///
    /// A stride of zero is allowed because ordinal semantics are independent
    /// metadata; callers that require monotonically increasing ordinals should
    /// validate that policy at the owning plan layer.
    #[must_use]
    pub const fn new(count: u64, ordinal_stride: u64) -> Self {
        Self {
            count,
            ordinal_stride,
        }
    }

    /// Returns the number of requested instances.
    #[must_use]
    pub const fn count(self) -> u64 {
        self.count
    }

    /// Returns the ordinal stride.
    #[must_use]
    pub const fn ordinal_stride(self) -> u64 {
        self.ordinal_stride
    }

    /// Returns whether the repetition is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.count == 0
    }

    /// Returns the final zero-based instance offset.
    ///
    /// Returns `None` when the multiplication would overflow `u64`.
    #[must_use]
    pub const fn checked_final_offset(self) -> Option<u64> {
        if self.count == 0 {
            return Some(0);
        }

        match (self.count - 1).checked_mul(self.ordinal_stride) {
            Some(value) => Some(value),
            None => None,
        }
    }
}

// ============================================================================
// Round specification
// ============================================================================

/// Complete semantic description of one QEC scheduling round.
///
/// This is the principal type exported by this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundSpec {
    id: RoundId,
    ordinal: RoundOrdinal,
    kind: RoundKind,
    label: Option<String>,

    /// Logical qubits participating in this round.
    logical_qubits: Vec<QubitId>,

    /// Physical qubits participating when physical mapping is already known.
    ///
    /// This list is optional because routing may occur after QEC planning.
    physical_qubits: Vec<PhysicalQubitId>,

    /// Canonical IR operations belonging to this round.
    operations: Vec<OperationId>,

    /// Explicit round dependencies.
    dependencies: Vec<RoundDependency>,

    /// Local temporal requirements.
    timing: RoundTiming,

    /// Optional compact repetition description.
    repetition: Option<RoundRepetition>,
}

impl RoundSpec {
    /// Creates an empty round specification.
    ///
    /// The caller subsequently adds participants, operations, and dependencies.
    #[must_use]
    pub fn new(
        id: RoundId,
        ordinal: RoundOrdinal,
        kind: RoundKind,
    ) -> Self {
        Self {
            id,
            ordinal,
            kind,
            label: None,
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
            operations: Vec::new(),
            dependencies: Vec::new(),
            timing: RoundTiming::default(),
            repetition: None,
        }
    }

    /// Returns the stable round identity.
    #[must_use]
    pub const fn id(&self) -> RoundId {
        self.id
    }

    /// Returns the semantic sequence ordinal.
    #[must_use]
    pub const fn ordinal(&self) -> RoundOrdinal {
        self.ordinal
    }

    /// Returns the round kind.
    #[must_use]
    pub const fn kind(&self) -> &RoundKind {
        &self.kind
    }

    /// Returns the optional human-readable label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Sets a diagnostic/human-readable label.
    ///
    /// Labels are metadata and MUST NOT be used as semantic identity.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Replaces the round timing requirements.
    #[must_use]
    pub const fn with_timing(mut self, timing: RoundTiming) -> Self {
        self.timing = timing;
        self
    }

    /// Sets compact repetition information.
    #[must_use]
    pub const fn with_repetition(mut self, repetition: RoundRepetition) -> Self {
        self.repetition = Some(repetition);
        self
    }

    /// Removes repetition information.
    #[must_use]
    pub const fn without_repetition(mut self) -> Self {
        self.repetition = None;
        self
    }

    /// Returns logical qubits participating in this round.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns physical qubits participating in this round.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns canonical IR operations belonging to this round.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns explicit round dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[RoundDependency] {
        &self.dependencies
    }

    /// Returns round timing requirements.
    #[must_use]
    pub const fn timing(&self) -> &RoundTiming {
        &self.timing
    }

    /// Returns repetition information.
    #[must_use]
    pub const fn repetition(&self) -> Option<RoundRepetition> {
        self.repetition
    }

    /// Adds a logical qubit.
    ///
    /// Duplicate logical qubits are rejected immediately.
    pub fn add_logical_qubit(
        &mut self,
        qubit: QubitId,
    ) -> Result<(), RoundError> {
        if self.logical_qubits.contains(&qubit) {
            return Err(RoundError::DuplicateLogicalQubit { qubit });
        }

        self.logical_qubits.push(qubit);
        Ok(())
    }

    /// Adds a physical qubit.
    ///
    /// Duplicate physical qubits are rejected immediately.
    pub fn add_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> Result<(), RoundError> {
        if self.physical_qubits.contains(&qubit) {
            return Err(RoundError::DuplicatePhysicalQubit { qubit });
        }

        self.physical_qubits.push(qubit);
        Ok(())
    }

    /// Adds a canonical IR operation.
    ///
    /// An operation may belong to only one position within this round.
    pub fn add_operation(
        &mut self,
        operation: OperationId,
    ) -> Result<(), RoundError> {
        if self.operations.contains(&operation) {
            return Err(RoundError::DuplicateOperation { operation });
        }

        self.operations.push(operation);
        Ok(())
    }

    /// Adds an explicit predecessor dependency.
    ///
    /// Self-dependencies are rejected immediately.
    pub fn add_dependency(
        &mut self,
        dependency: RoundDependency,
    ) -> Result<(), RoundError> {
        if dependency.predecessor == self.id {
            return Err(RoundError::SelfDependency { round: self.id });
        }

        if self.dependencies.contains(&dependency) {
            return Err(RoundError::DuplicateDependency {
                predecessor: dependency.predecessor,
                kind: dependency.kind,
            });
        }

        self.dependencies.push(dependency);
        Ok(())
    }

    /// Validates the complete local round specification.
    pub fn validate(&self) -> Result<(), RoundError> {
        self.timing.validate()?;

        if let Some(repetition) = self.repetition {
            if repetition.checked_final_offset().is_none() {
                return Err(RoundError::RepetitionOverflow {
                    round: self.id,
                });
            }
        }

        validate_unique(&self.logical_qubits)
            .map_err(|_| RoundError::DuplicateLogicalQubit {
                qubit: self.logical_qubits
                    .first()
                    .copied()
                    .expect("duplicate validation cannot identify an empty collection"),
            })?;

        validate_unique(&self.physical_qubits)
            .map_err(|_| RoundError::DuplicatePhysicalQubit {
                qubit: self.physical_qubits
                    .first()
                    .copied()
                    .expect("duplicate validation cannot identify an empty collection"),
            })?;

        validate_unique(&self.operations)
            .map_err(|_| RoundError::DuplicateOperation {
                operation: self.operations
                    .first()
                    .copied()
                    .expect("duplicate validation cannot identify an empty collection"),
            })?;

        let mut dependencies = HashSet::with_capacity(self.dependencies.len());

        for dependency in &self.dependencies {
            if dependency.predecessor == self.id {
                return Err(RoundError::SelfDependency { round: self.id });
            }

            if !dependencies.insert(*dependency) {
                return Err(RoundError::DuplicateDependency {
                    predecessor: dependency.predecessor,
                    kind: dependency.kind,
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Round plan
// ============================================================================

/// A collection of QEC rounds forming a scheduling-level plan.
///
/// `RoundPlan` stores explicitly represented rounds.
///
/// Repeated rounds can remain compact inside an individual `RoundSpec` through
/// `RoundRepetition` until materialization is actually required.
///
/// The plan does not perform scheduling.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RoundPlan {
    rounds: Vec<RoundSpec>,
}

impl RoundPlan {
    /// Creates an empty plan.
    #[must_use]
    pub const fn new() -> Self {
        Self { rounds: Vec::new() }
    }

    /// Returns the number of explicitly represented round specifications.
    ///
    /// This is NOT necessarily the number of materialized QEC executions when
    /// repetition is used.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rounds.len()
    }

    /// Returns whether the explicit plan contains no round specifications.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rounds.is_empty()
    }

    /// Returns the explicit round specifications.
    #[must_use]
    pub fn rounds(&self) -> &[RoundSpec] {
        &self.rounds
    }

    /// Returns a mutable view of the explicit rounds.
    ///
    /// Mutation remains caller-owned. Call `validate` after modifications.
    pub fn rounds_mut(&mut self) -> &mut [RoundSpec] {
        &mut self.rounds
    }

    /// Adds a round specification.
    ///
    /// Round IDs must be unique within a plan.
    pub fn push(&mut self, round: RoundSpec) -> Result<(), RoundError> {
        if self.rounds.iter().any(|existing| existing.id == round.id) {
            return Err(RoundError::DuplicateRound { round: round.id });
        }

        round.validate()?;
        self.rounds.push(round);
        Ok(())
    }

    /// Returns a round by stable identity.
    #[must_use]
    pub fn get(&self, id: RoundId) -> Option<&RoundSpec> {
        self.rounds.iter().find(|round| round.id == id)
    }

    /// Returns a mutable round by stable identity.
    ///
    /// Call `validate` on the plan after mutation.
    pub fn get_mut(&mut self, id: RoundId) -> Option<&mut RoundSpec> {
        self.rounds.iter_mut().find(|round| round.id == id)
    }

    /// Validates all rounds and all dependencies that refer to rounds known
    /// within this plan.
    ///
    /// Dependency cycles are intentionally NOT detected here because cycle
    /// detection belongs to the scheduler dependency graph. A round plan can
    /// legitimately be assembled incrementally before all rounds are present.
    pub fn validate(&self) -> Result<(), RoundError> {
        let mut ids = HashSet::with_capacity(self.rounds.len());

        for round in &self.rounds {
            if !ids.insert(round.id) {
                return Err(RoundError::DuplicateRound { round: round.id });
            }

            round.validate()?;
        }

        for round in &self.rounds {
            for dependency in &round.dependencies {
                if !ids.contains(&dependency.predecessor) {
                    return Err(RoundError::UnknownPredecessor {
                        round: round.id,
                        predecessor: dependency.predecessor,
                    });
                }
            }
        }

        Ok(())
    }

    /// Returns explicit round dependencies as `(predecessor, successor)` pairs.
    ///
    /// This is deliberately a projection rather than a scheduler graph. The
    /// generic dependency subsystem remains responsible for constructing its
    /// canonical graph representation.
    #[must_use]
    pub fn dependency_edges(&self) -> Vec<(RoundId, RoundId)> {
        let mut edges = Vec::new();

        for round in &self.rounds {
            for dependency in &round.dependencies {
                edges.push((dependency.predecessor, round.id));
            }
        }

        edges
    }

    /// Returns the total number of materialized instances represented by
    /// this plan, when that count can be represented by `u64`.
    ///
    /// A plan containing repeated rounds may therefore describe substantially
    /// more executions than `len()` suggests.
    #[must_use]
    pub fn materialized_round_count(&self) -> Option<u64> {
        let mut total = 0_u64;

        for round in &self.rounds {
            let count = match round.repetition {
                Some(repetition) => repetition.count(),
                None => 1,
            };

            total = total.checked_add(count)?;
        }

        Some(total)
    }
}

// ============================================================================
// Validation helper
// ============================================================================

/// Validates that a collection contains no duplicate values.
///
/// This helper is intentionally generic and local to this module.
fn validate_unique<T>(values: &[T]) -> Result<(), ()>
where
    T: Eq + std::hash::Hash,
{
    let mut seen = HashSet::with_capacity(values.len());

    for value in values {
        if !seen.insert(value) {
            return Err(());
        }
    }

    Ok(())
}

// ============================================================================
// Round error
// ============================================================================

/// Structured local error for invalid QEC round descriptions.
///
/// These errors describe the round model itself.
///
/// Conversion into the scheduler-wide `SchedulingError` belongs to
/// `qec` adapters or the scheduler boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RoundError {
    /// Two explicit rounds use the same stable identity.
    DuplicateRound {
        /// Conflicting round identity.
        round: RoundId,
    },

    /// The same logical qubit occurs more than once within a round.
    DuplicateLogicalQubit {
        /// Conflicting logical qubit.
        qubit: QubitId,
    },

    /// The same physical qubit occurs more than once within a round.
    DuplicatePhysicalQubit {
        /// Conflicting physical qubit.
        qubit: PhysicalQubitId,
    },

    /// The same operation occurs more than once within a round.
    DuplicateOperation {
        /// Conflicting operation.
        operation: OperationId,
    },

    /// A round depends on itself.
    SelfDependency {
        /// Invalid round.
        round: RoundId,
    },

    /// The same dependency is declared more than once.
    DuplicateDependency {
        /// Predecessor round.
        predecessor: RoundId,

        /// Dependency category.
        kind: RoundDependencyKind,
    },

    /// A dependency refers to a round not present in the plan.
    UnknownPredecessor {
        /// Successor round containing the dependency.
        round: RoundId,

        /// Missing predecessor.
        predecessor: RoundId,
    },

    /// The local timing requirements contradict one another.
    InvalidTiming {
        /// Stable reason category.
        reason: &'static str,
    },

    /// A repetition offset cannot be represented.
    RepetitionOverflow {
        /// Round whose repetition specification overflowed.
        round: RoundId,
    },
}

impl fmt::Display for RoundError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateRound { round } => {
                write!(formatter, "duplicate QEC round identity: {round}")
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(formatter, "duplicate logical qubit in QEC round: {qubit:?}")
            }

            Self::DuplicatePhysicalQubit { qubit } => {
                write!(
                    formatter,
                    "duplicate physical qubit in QEC round: {qubit:?}"
                )
            }

            Self::DuplicateOperation { operation } => {
                write!(formatter, "duplicate operation in QEC round: {operation:?}")
            }

            Self::SelfDependency { round } => {
                write!(formatter, "QEC round cannot depend on itself: {round}")
            }

            Self::DuplicateDependency {
                predecessor,
                kind,
            } => {
                write!(
                    formatter,
                    "duplicate QEC round dependency from {predecessor} ({kind})"
                )
            }

            Self::UnknownPredecessor {
                round,
                predecessor,
            } => {
                write!(
                    formatter,
                    "QEC round {round} depends on unknown predecessor {predecessor}"
                )
            }

            Self::InvalidTiming { reason } => {
                write!(formatter, "invalid QEC round timing: {reason}")
            }

            Self::RepetitionOverflow { round } => {
                write!(
                    formatter,
                    "QEC round repetition metadata overflows its representable ordinal offset: {round}"
                )
            }
        }
    }
}

impl std::error::Error for RoundError {}

// ============================================================================
// Public result alias
// ============================================================================

/// Result type for QEC round-model operations.
pub type RoundResult<T> = Result<T, RoundError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn round(id: u64, ordinal: u64) -> RoundSpec {
        RoundSpec::new(
            RoundId::new(id),
            RoundOrdinal::new(ordinal),
            RoundKind::SyndromeExtraction,
        )
    }

    #[test]
    fn round_identity_is_stable() {
        let id = RoundId::new(42);

        assert_eq!(id.value(), 42);
        assert_eq!(id.to_string(), "round:42");
        assert_eq!(id.checked_next(), Some(RoundId::new(43)));
    }

    #[test]
    fn round_ordinal_is_not_an_implicit_dependency() {
        let first = round(1, 0);
        let second = round(2, 1);

        assert!(first.dependencies().is_empty());
        assert!(second.dependencies().is_empty());
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut value = round(1, 0);

        let result = value.add_dependency(RoundDependency::new(
            RoundId::new(1),
            RoundDependencyKind::Completion,
        ));

        assert!(matches!(
            result,
            Err(RoundError::SelfDependency {
                round: RoundId(1)
            })
        ));
    }

    #[test]
    fn duplicate_logical_qubit_is_rejected() {
        let mut value = round(1, 0);
        let qubit = QubitId::from(7_u64);

        assert!(value.add_logical_qubit(qubit).is_ok());
        assert!(matches!(
            value.add_logical_qubit(qubit),
            Err(RoundError::DuplicateLogicalQubit { .. })
        ));
    }

    #[test]
    fn duplicate_operation_is_rejected() {
        let mut value = round(1, 0);
        let operation = OperationId::new(10);

        assert!(value.add_operation(operation).is_ok());
        assert!(matches!(
            value.add_operation(operation),
            Err(RoundError::DuplicateOperation { .. })
        ));
    }

    #[test]
    fn timing_rejects_inverted_duration_bounds() {
        let timing = RoundTiming::unconstrained()
            .with_minimum_duration(Duration::new(20))
            .with_maximum_duration(Duration::new(10));

        assert!(matches!(
            timing.validate(),
            Err(RoundError::InvalidTiming { .. })
        ));
    }

    #[test]
    fn timing_rejects_deadline_before_release() {
        let timing = RoundTiming::unconstrained()
            .with_release_time(TimePoint::new(20))
            .with_deadline(TimePoint::new(10));

        assert!(matches!(
            timing.validate(),
            Err(RoundError::InvalidTiming { .. })
        ));
    }

    #[test]
    fn repetition_can_remain_compact() {
        let repetition = RoundRepetition::new(1_000_000, 1);

        assert_eq!(repetition.count(), 1_000_000);
        assert_eq!(repetition.checked_final_offset(), Some(999_999));
    }

    #[test]
    fn repetition_detects_overflow() {
        let repetition = RoundRepetition::new(u64::MAX, 2);

        assert_eq!(repetition.checked_final_offset(), None);
    }

    #[test]
    fn plan_requires_known_predecessors() {
        let mut plan = RoundPlan::new();

        let mut second = round(2, 1);

        second
            .add_dependency(RoundDependency::new(
                RoundId::new(1),
                RoundDependencyKind::Completion,
            ))
            .expect("dependency should be accepted locally");

        assert!(plan.push(second).is_ok());

        assert!(matches!(
            plan.validate(),
            Err(RoundError::UnknownPredecessor {
                round: RoundId(2),
                predecessor: RoundId(1)
            })
        ));
    }

    #[test]
    fn plan_accepts_explicit_dependency() {
        let mut plan = RoundPlan::new();

        plan.push(round(1, 0))
            .expect("first round should be valid");

        let mut second = round(2, 1);

        second
            .add_dependency(RoundDependency::new(
                RoundId::new(1),
                RoundDependencyKind::Measurement,
            ))
            .expect("dependency should be accepted");

        plan.push(second)
            .expect("second round should be valid");

        assert!(plan.validate().is_ok());
        assert_eq!(
            plan.dependency_edges(),
            vec![(RoundId::new(1), RoundId::new(2))]
        );
    }

    #[test]
    fn materialized_count_is_checked() {
        let mut plan = RoundPlan::new();

        plan.push(
            round(1, 0).with_repetition(RoundRepetition::new(10, 1)),
        )
        .expect("round should be valid");

        plan.push(round(2, 10))
            .expect("round should be valid");

        assert_eq!(plan.materialized_round_count(), Some(11));
    }

    #[test]
    fn empty_plan_is_valid() {
        let plan = RoundPlan::new();

        assert!(plan.is_empty());
        assert_eq!(plan.len(), 0);
        assert_eq!(plan.materialized_round_count(), Some(0));
        assert!(plan.validate().is_ok());
    }
}