//! Zamani Quantum Scheduling — Generic Constraint Framework
//!
//! This module defines the foundational constraint contract used by the
//! scheduling subsystem.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "Is a proposed scheduling decision admissible under the supplied
//! > constraints, and if not, why?"
//!
//! It owns:
//!
//! - the generic scheduler `Constraint` trait;
//! - constraint identity;
//! - constraint categories and severity;
//! - constraint evaluation context;
//! - candidate scheduling decisions;
//! - immutable scheduling-state views;
//! - structured constraint violations;
//! - deterministic constraint collections;
//! - constraint evaluation reports;
//! - composition of multiple constraints;
//! - short-circuit and collect-all evaluation;
//! - constraint explanations;
//! - enable/disable state at the constraint layer;
//! - constraint ordering and deterministic evaluation;
//! - generic constraint applicability;
//! - conversion-independent constraint diagnostics.
//!
//! It does NOT own:
//!
//! - quantum operation semantics;
//! - quantum gate definitions;
//! - quantum circuit representation;
//! - logical qubit identity;
//! - physical qubit identity;
//! - hardware discovery;
//! - routing;
//! - scheduling algorithms;
//! - resource calendars;
//! - hardware calibration;
//! - QEC algorithms;
//! - runtime execution;
//! - serialization formats;
//! - vendor-specific APIs.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical identity boundary
//!
//! Logical and physical qubit identities are imported from the canonical IR:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Scheduler operation and resource identities are imported from the canonical
//! repository identity model through `scheduling::types`.
//!
//! This module MUST NOT define another `QubitId`, `PhysicalQubitId`,
//! `OperationId`, or `ResourceId`.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::qubit
//! quantum::ir::core::identity
//!           │
//!           ▼
//! scheduling::types
//!           │
//!           ▼
//! scheduling::constraints::constraint
//!           │
//!     ┌─────┼──────────────────┐
//!     ▼     ▼                  ▼
//!  qubit  channel          measurement
//!  reset   control       communication
//!     │     │                  │
//!     └─────┼──────────────────┘
//!           ▼
//!       planners
//!           │
//!           ▼
//!       verification
//! ```
//!
//! The generic contract intentionally knows nothing about those specialized
//! constraint implementations.
//!
//! # Constraint versus timing constraint
//!
//! `scheduling::timing::constraints` owns temporal constraint semantics.
//!
//! This module owns the generic mechanism by which temporal constraints,
//! resource constraints, qubit constraints, control constraints, QEC
//! constraints, communication constraints, and future constraints participate
//! in scheduling.
//!
//! Therefore:
//!
//! ```text
//! timing::constraints
//!        │
//!        ▼
//! generic Constraint
//!
//! resources/*
//!        │
//!        ▼
//! generic Constraint
//!
//! qec/*
//!        │
//!        ▼
//! generic Constraint
//! ```
//!
//! This separation prevents `constraint.rs` from becoming a second timing or
//! hardware subsystem.
//!
//! # Candidate versus state
//!
//! A constraint evaluates a candidate scheduling decision against an immutable
//! scheduling-state snapshot.
//!
//! Conceptually:
//!
//! ```text
//! candidate operation
//!        +
//! proposed placement
//!        +
//! resource claims
//!        +
//! current scheduling state
//!        +
//! constraint
//!        │
//!        ▼
//!    evaluation
//!        │
//!   ┌────┴────┐
//!   ▼         ▼
//!  valid   violation
//! ```
//!
//! The constraint itself must not mutate scheduler state.
//!
//! Reservation and state mutation belong to the planner/resource subsystem.
//!
//! # Scalability
//!
//! This file contains no machine-size constants.
//!
//! There is no:
//!
//! - maximum qubit count;
//! - maximum physical qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - maximum constraint count;
//! - maximum schedule depth;
//! - maximum operation arity;
//! - maximum QEC distance;
//! - maximum communication-node count;
//! - fixed topology;
//! - fixed number of channels;
//! - fixed number of constraints.
//!
//! Collections grow according to the resources available to the compilation
//! invocation.
//!
//! The architecture's meaning of "infinity" is:
//!
//! > no artificial finite machine-size ceiling is encoded by this module.
//!
//! A concrete compilation remains bounded by available memory, CPU time,
//! explicit compiler limits, target resources, and operating-system resources.
//!
//! # Determinism
//!
//! Constraint collections use deterministic ordering.
//!
//! A constraint implementation must not depend on hidden global state,
//! wall-clock time, or implicit randomness.
//!
//! If a constraint requires stochastic behaviour, the stochastic execution
//! context must be supplied explicitly by the owning subsystem.
//!
//! # Safety
//!
//! This module is safe Rust only.
//!
//! Rust 1.97 / Rust 1.97.1.
//!
//! Rust 2021 edition.
//!
//! Stable Rust.
//!
//! No nightly features.
//!
//! No `unsafe`.
//!
//! The compiler-enforced attributes below make that requirement explicit.
//!
//! # Thread safety
//!
//! Constraint implementations should preferably be immutable and shareable.
//!
//! The trait therefore requires `Send + Sync`.
//!
//! This allows planners to evaluate independent constraints concurrently when
//! an implementation chooses to do so, without requiring this module to own
//! synchronization.
//!
//! # Mutation rule
//!
//! A `Constraint` MUST NOT mutate:
//!
//! - the candidate;
//! - the scheduling state;
//! - resources;
//! - the quantum IR;
//! - hardware state;
//! - global state.
//!
//! Constraint evaluation is observational.
//!
//! # Error rule
//!
//! Constraint failures are represented by `ConstraintViolation` in this
//! module. The scheduler's canonical `SchedulingError::ConstraintViolation`
//! remains the higher-level error representation.
//!
//! Conversion belongs at the scheduler boundary and does not belong here.
//!
//! This avoids making the generic constraint contract dependent on planners or
//! error aggregation policy.
//!
//! # Integration contract
//!
//! Specialized constraint modules should implement:
//!
//! ```text
//! Constraint
//! ```
//!
//! Examples:
//!
//! ```text
//! scheduling::constraints::qubit
//! scheduling::constraints::channel
//! scheduling::constraints::measurement
//! scheduling::constraints::reset
//! scheduling::constraints::control
//! scheduling::constraints::communication
//! scheduling::constraints::custom
//! ```
//!
//! They consume `ConstraintContext` and return either success or a structured
//! `ConstraintViolation`.
//!
//! Planners use `ConstraintSet` to evaluate all applicable constraints.
//!
//! Verification can independently use the same constraints against an already
//! constructed schedule.
//!
//! This is important: the same constraint semantics should be usable both
//! during planning and after planning.
//!
//! # Production invariant
//!
//! A successful constraint evaluation means only:
//!
//! > "This particular constraint found no violation for this candidate and
//! > supplied state."
//!
//! It does NOT mean that the entire schedule is valid.
//!
//! The complete scheduler must evaluate all applicable constraints and then
//! run the dedicated verification subsystem.
//!
//! # No vendor assumptions
//!
//! This module contains no vendor names, SDKs, backend identifiers, or device
//! assumptions.
//!
//! Hardware-specific constraints should be implemented in specialized modules
//! or adapters and exposed through this generic contract.
//!
//! # No hard-coded operation arity
//!
//! Candidate qubits are represented as slices rather than fixed one- or
//! two-qubit fields.
//!
//! This allows:
//!
//! - single-qubit operations;
//! - two-qubit operations;
//! - multi-qubit operations;
//! - collective operations;
//! - future architectures with different operation arity.
//!
//! # No hard-coded resource count
//!
//! Resource claims are represented as a dynamically sized slice.
//!
//! There is no assumption that a machine has:
//!
//! - one control channel;
//! - two control channels;
//! - eight channels;
//! - a fixed number of measurement resources.
//!
//! # Future extension
//!
//! New constraint kinds should normally be added as a new `ConstraintKind`
//! variant only when callers need to classify the constraint semantically.
//!
//! The core `Constraint` trait itself should not need modification merely to
//! add a new concrete constraint implementation.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use super::super::types::{
    Duration,
    ReservationId,
    TimePoint,
};

use crate::quantum::ir::core::identity::{
    OperationId,
    ResourceId,
};

// ============================================================================
// Constraint identity
// ============================================================================

/// Stable identifier for a scheduling constraint.
///
/// A constraint identifier is a semantic identifier for a constraint instance,
/// not a collection index and not a hardware address.
///
/// The value has no imposed maximum.
///
/// Allocation of identifiers belongs to the subsystem constructing the
/// constraint set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConstraintId(u64);

impl ConstraintId {
    /// Creates a constraint identifier from an explicitly supplied value.
    ///
    /// This does not allocate or register the identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns whether this is the zero identifier.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns the next representable identifier.
    ///
    /// This performs no allocation.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for ConstraintId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ConstraintId> for u64 {
    fn from(value: ConstraintId) -> Self {
        value.value()
    }
}

impl fmt::Display for ConstraintId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "constraint:{}", self.0)
    }
}

// ============================================================================
// Constraint kind
// ============================================================================

/// Semantic category of a scheduling constraint.
///
/// The enum is intentionally broad enough to cover current and future
/// quantum architectures without encoding a particular technology.
///
/// Concrete implementations may use `Custom` when no existing category is
/// appropriate.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstraintKind {
    /// Generic constraint with no more specific category.
    Generic,

    /// Quantum/logical/physical qubit occupancy constraint.
    Qubit,

    /// Control or drive channel constraint.
    Channel,

    /// Measurement/readout constraint.
    Measurement,

    /// Reset constraint.
    Reset,

    /// Classical control or feedback constraint.
    Control,

    /// Communication or network constraint.
    Communication,

    /// Temporal constraint.
    Timing,

    /// Resource capacity constraint.
    Resource,

    /// Alignment/grid constraint.
    Alignment,

    /// QEC-specific scheduling constraint.
    Qec,

    /// Target capability constraint.
    Capability,

    /// Deadline or scheduling-window constraint.
    Deadline,

    /// User/plugin-defined constraint.
    Custom,
}

impl ConstraintKind {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Qubit => "qubit",
            Self::Channel => "channel",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::Control => "control",
            Self::Communication => "communication",
            Self::Timing => "timing",
            Self::Resource => "resource",
            Self::Alignment => "alignment",
            Self::Qec => "qec",
            Self::Capability => "capability",
            Self::Deadline => "deadline",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ConstraintKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Constraint severity
// ============================================================================

/// Severity of a constraint violation.
///
/// Severity is diagnostic and policy metadata.
///
/// A scheduler must not silently treat an error-level constraint as advisory.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstraintSeverity {
    /// Informational condition.
    Info,

    /// Advisory condition that may influence optimization.
    Warning,

    /// A condition that makes the candidate inadmissible.
    Error,

    /// A condition indicating a fundamental invariant failure.
    Critical,
}

impl ConstraintSeverity {
    /// Returns whether this severity represents an admissibility failure.
    #[must_use]
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Error | Self::Critical)
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
        }
    }
}

impl fmt::Display for ConstraintSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Constraint evaluation mode
// ============================================================================

/// Determines how a constraint collection reports failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstraintEvaluationMode {
    /// Stop at the first blocking violation.
    FirstFailure,

    /// Evaluate every applicable constraint and collect all violations.
    CollectAll,
}

impl Default for ConstraintEvaluationMode {
    fn default() -> Self {
        Self::FirstFailure
    }
}

// ============================================================================
// Constraint phase
// ============================================================================

/// Phase in which a constraint is expected to be evaluated.
///
/// This allows the same constraint framework to serve both planning and
/// verification without making constraints dependent on either subsystem.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstraintPhase {
    /// Candidate has not yet been committed.
    Planning,

    /// Candidate is being checked before reservation.
    PreCommit,

    /// Candidate has been placed and is being checked as part of schedule
    /// construction.
    PostCommit,

    /// A completed schedule is being independently verified.
    Verification,

    /// Runtime/dynamic scheduling is evaluating a new event.
    Runtime,
}

impl ConstraintPhase {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planning => "planning",
            Self::PreCommit => "pre_commit",
            Self::PostCommit => "post_commit",
            Self::Verification => "verification",
            Self::Runtime => "runtime",
        }
    }
}

// ============================================================================
// Resource claim
// ============================================================================

/// A resource requested by a scheduling candidate.
///
/// The quantity is abstract and has no hardware-specific unit.
///
/// Examples include capacity units of:
///
/// - a control channel;
/// - a measurement resource;
/// - a communication link;
/// - a classical processor;
/// - an ancilla pool;
/// - another capacity-limited resource.
///
/// A quantity of zero is representable but normally has no scheduling effect.
///
/// The interpretation of the quantity belongs to the resource subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConstraintResourceClaim {
    resource: ResourceId,
    quantity: u128,
}

impl ConstraintResourceClaim {
    /// Creates a resource claim.
    #[must_use]
    pub const fn new(resource: ResourceId, quantity: u128) -> Self {
        Self {
            resource,
            quantity,
        }
    }

    /// Returns the resource identity.
    #[must_use]
    pub const fn resource(self) -> ResourceId {
        self.resource
    }

    /// Returns the requested capacity.
    #[must_use]
    pub const fn quantity(self) -> u128 {
        self.quantity
    }

    /// Returns whether the requested quantity is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.quantity == 0
    }
}

// ============================================================================
// Existing reservation view
// ============================================================================

/// Immutable view of an existing reservation relevant to constraint
/// evaluation.
///
/// This is intentionally a view rather than the resource subsystem's concrete
/// reservation type.
///
/// It prevents this generic constraint module from becoming coupled to the
/// resource-calendar implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstraintReservationView {
    reservation: ReservationId,
    operation: Option<OperationId>,
    resource: ResourceId,
    start: TimePoint,
    duration: Duration,
    quantity: u128,
}

impl ConstraintReservationView {
    /// Creates an immutable reservation view.
    #[must_use]
    pub const fn new(
        reservation: ReservationId,
        operation: Option<OperationId>,
        resource: ResourceId,
        start: TimePoint,
        duration: Duration,
        quantity: u128,
    ) -> Self {
        Self {
            reservation,
            operation,
            resource,
            start,
            duration,
            quantity,
        }
    }

    /// Returns the reservation identity.
    #[must_use]
    pub const fn reservation(self) -> ReservationId {
        self.reservation
    }

    /// Returns the operation occupying the reservation, if known.
    #[must_use]
    pub const fn operation(self) -> Option<OperationId> {
        self.operation
    }

    /// Returns the resource identity.
    #[must_use]
    pub const fn resource(self) -> ResourceId {
        self.resource
    }

    /// Returns the reservation start.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.start
    }

    /// Returns the reservation duration.
    #[must_use]
    pub const fn duration(self) -> Duration {
        self.duration
    }

    /// Returns the requested quantity.
    #[must_use]
    pub const fn quantity(self) -> u128 {
        self.quantity
    }

    /// Returns the reservation end, if representable.
    #[must_use]
    pub const fn checked_end(self) -> Option<TimePoint> {
        self.start.checked_add(self.duration)
    }
}

// ============================================================================
// Candidate scheduling decision
// ============================================================================

/// Immutable candidate scheduling decision.
///
/// This represents what a planner proposes, not what the scheduler has
/// already committed.
///
/// It deliberately contains no quantum operation object. The canonical
/// operation remains owned by `quantum::ir`.
///
/// The scheduler adapter supplies the canonical operation identity and
/// canonical qubit identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulingCandidate<'a> {
    operation: OperationId,
    logical_qubits: &'a [QubitId],
    physical_qubits: &'a [PhysicalQubitId],
    resource_claims: &'a [ConstraintResourceClaim],
    start: TimePoint,
    duration: Duration,
}

impl<'a> SchedulingCandidate<'a> {
    /// Creates a candidate scheduling decision.
    #[must_use]
    pub const fn new(
        operation: OperationId,
        logical_qubits: &'a [QubitId],
        physical_qubits: &'a [PhysicalQubitId],
        resource_claims: &'a [ConstraintResourceClaim],
        start: TimePoint,
        duration: Duration,
    ) -> Self {
        Self {
            operation,
            logical_qubits,
            physical_qubits,
            resource_claims,
            start,
            duration,
        }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns logical qubits touched by the operation.
    #[must_use]
    pub const fn logical_qubits(&self) -> &'a [QubitId] {
        self.logical_qubits
    }

    /// Returns physical qubits touched by the operation.
    #[must_use]
    pub const fn physical_qubits(&self) -> &'a [PhysicalQubitId] {
        self.physical_qubits
    }

    /// Returns resource claims made by the candidate.
    #[must_use]
    pub const fn resource_claims(&self) -> &'a [ConstraintResourceClaim] {
        self.resource_claims
    }

    /// Returns proposed start time.
    #[must_use]
    pub const fn start(&self) -> TimePoint {
        self.start
    }

    /// Returns proposed duration.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the proposed finish time.
    #[must_use]
    pub const fn checked_end(&self) -> Option<TimePoint> {
        self.start.checked_add(self.duration)
    }

    /// Returns whether the candidate has zero duration.
    #[must_use]
    pub const fn is_zero_duration(&self) -> bool {
        self.duration.is_zero()
    }
}

// ============================================================================
// Constraint state
// ============================================================================

/// Immutable scheduling state visible to a constraint.
///
/// This is intentionally a compact generic view rather than a resource
/// calendar, dependency graph, or planner state implementation.
///
/// Specialized subsystems can construct this view from their own structures.
///
/// The candidate is always evaluated separately from this state.
#[derive(Debug, Clone, Copy)]
pub struct ConstraintState<'a> {
    reservations: &'a [ConstraintReservationView],
    completed_operations: &'a [OperationId],
    unavailable_resources: &'a [ResourceId],
}

impl<'a> ConstraintState<'a> {
    /// Creates an immutable scheduling-state view.
    #[must_use]
    pub const fn new(
        reservations: &'a [ConstraintReservationView],
        completed_operations: &'a [OperationId],
        unavailable_resources: &'a [ResourceId],
    ) -> Self {
        Self {
            reservations,
            completed_operations,
            unavailable_resources,
        }
    }

    /// Returns current resource reservations visible to the constraint.
    #[must_use]
    pub const fn reservations(&self) -> &'a [ConstraintReservationView] {
        self.reservations
    }

    /// Returns operations known to be complete.
    #[must_use]
    pub const fn completed_operations(&self) -> &'a [OperationId] {
        self.completed_operations
    }

    /// Returns resources known to be unavailable.
    #[must_use]
    pub const fn unavailable_resources(&self) -> &'a [ResourceId] {
        self.unavailable_resources
    }

    /// Tests whether an operation is recorded as completed.
    #[must_use]
    pub fn is_operation_completed(&self, operation: OperationId) -> bool {
        self.completed_operations
            .iter()
            .any(|candidate| *candidate == operation)
    }

    /// Tests whether a resource is unavailable.
    #[must_use]
    pub fn is_resource_unavailable(&self, resource: ResourceId) -> bool {
        self.unavailable_resources
            .iter()
            .any(|candidate| *candidate == resource)
    }
}

// ============================================================================
// Constraint context
// ============================================================================

/// Complete immutable context supplied to one constraint evaluation.
///
/// The context deliberately contains no mutable scheduler state.
///
/// It can therefore safely be shared among independent constraint evaluations.
#[derive(Debug, Clone, Copy)]
pub struct ConstraintContext<'a> {
    candidate: &'a SchedulingCandidate<'a>,
    state: &'a ConstraintState<'a>,
    phase: ConstraintPhase,
}

impl<'a> ConstraintContext<'a> {
    /// Creates a constraint evaluation context.
    #[must_use]
    pub const fn new(
        candidate: &'a SchedulingCandidate<'a>,
        state: &'a ConstraintState<'a>,
        phase: ConstraintPhase,
    ) -> Self {
        Self {
            candidate,
            state,
            phase,
        }
    }

    /// Returns the candidate being evaluated.
    #[must_use]
    pub const fn candidate(&self) -> &'a SchedulingCandidate<'a> {
        self.candidate
    }

    /// Returns the immutable scheduling state.
    #[must_use]
    pub const fn state(&self) -> &'a ConstraintState<'a> {
        self.state
    }

    /// Returns the current evaluation phase.
    #[must_use]
    pub const fn phase(&self) -> ConstraintPhase {
        self.phase
    }
}

// ============================================================================
// Constraint applicability
// ============================================================================

/// Determines whether a constraint applies to a particular candidate/context.
///
/// Applicability is separate from evaluation so that specialized constraints
/// can avoid unnecessary work.
///
/// A constraint MUST still remain correct if `applies` returns `true` for a
/// broader set of candidates than strictly necessary.
///
/// Implementations should prefer conservative applicability checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintApplicability {
    /// Constraint must be evaluated.
    Applicable,

    /// Constraint does not apply to this candidate.
    NotApplicable,
}

impl ConstraintApplicability {
    /// Returns whether evaluation is required.
    #[must_use]
    pub const fn is_applicable(self) -> bool {
        matches!(self, Self::Applicable)
    }
}

// ============================================================================
// Constraint violation
// ============================================================================

/// Structured explanation of a constraint violation.
///
/// This is deliberately independent of `SchedulingError`.
///
/// The higher-level scheduler can convert this into
/// `SchedulingError::ConstraintViolation` while preserving the richer
/// diagnostic information for reporting and telemetry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintViolation {
    constraint: ConstraintId,
    kind: ConstraintKind,
    severity: ConstraintSeverity,
    operation: Option<OperationId>,
    resource: Option<ResourceId>,
    logical_qubit: Option<QubitId>,
    physical_qubit: Option<PhysicalQubitId>,
    requested_start: Option<TimePoint>,
    requested_duration: Option<Duration>,
    reason: String,
}

impl ConstraintViolation {
    /// Creates a structured violation.
    #[must_use]
    pub fn new(
        constraint: ConstraintId,
        kind: ConstraintKind,
        severity: ConstraintSeverity,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            constraint,
            kind,
            severity,
            operation: None,
            resource: None,
            logical_qubit: None,
            physical_qubit: None,
            requested_start: None,
            requested_duration: None,
            reason: reason.into(),
        }
    }

    /// Returns the constraint identity.
    #[must_use]
    pub const fn constraint(&self) -> ConstraintId {
        self.constraint
    }

    /// Returns the semantic constraint kind.
    #[must_use]
    pub const fn kind(&self) -> ConstraintKind {
        self.kind
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> ConstraintSeverity {
        self.severity
    }

    /// Returns the operation involved, if known.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        self.operation
    }

    /// Returns the resource involved, if known.
    #[must_use]
    pub const fn resource(&self) -> Option<ResourceId> {
        self.resource
    }

    /// Returns the logical qubit involved, if known.
    #[must_use]
    pub const fn logical_qubit(&self) -> Option<QubitId> {
        self.logical_qubit
    }

    /// Returns the physical qubit involved, if known.
    #[must_use]
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        self.physical_qubit
    }

    /// Returns the requested start time, if known.
    #[must_use]
    pub const fn requested_start(&self) -> Option<TimePoint> {
        self.requested_start
    }

    /// Returns the requested duration, if known.
    #[must_use]
    pub const fn requested_duration(&self) -> Option<Duration> {
        self.requested_duration
    }

    /// Returns the stable explanation.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Attaches an operation identity.
    #[must_use]
    pub const fn with_operation(mut self, operation: OperationId) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Attaches a resource identity.
    #[must_use]
    pub const fn with_resource(mut self, resource: ResourceId) -> Self {
        self.resource = Some(resource);
        self
    }

    /// Attaches a logical qubit identity.
    #[must_use]
    pub const fn with_logical_qubit(mut self, qubit: QubitId) -> Self {
        self.logical_qubit = Some(qubit);
        self
    }

    /// Attaches a physical qubit identity.
    #[must_use]
    pub const fn with_physical_qubit(mut self, qubit: PhysicalQubitId) -> Self {
        self.physical_qubit = Some(qubit);
        self
    }

    /// Attaches the requested placement.
    #[must_use]
    pub const fn with_timing(
        mut self,
        start: TimePoint,
        duration: Duration,
    ) -> Self {
        self.requested_start = Some(start);
        self.requested_duration = Some(duration);
        self
    }
}

impl fmt::Display for ConstraintViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} [{}]: {}",
            self.constraint,
            self.kind,
            self.reason
        )
    }

impl std::error::Error for ConstraintViolation {}

// ============================================================================
// Constraint trait
// ============================================================================

/// Generic scheduling constraint contract.
///
/// Implementations must be:
///
/// - deterministic unless explicitly documented otherwise;
/// - side-effect free;
/// - thread-safe;
/// - independent of scheduler algorithm;
/// - independent of hardware vendor;
/// - independent of machine size;
/// - independent of the concrete resource-calendar implementation.
///
/// The trait is intentionally object-safe so a scheduler can hold heterogeneous
/// constraints through `Box<dyn Constraint>`, while specialized systems may
/// use generic/static dispatch where performance requires it.
pub trait Constraint: Send + Sync {
    /// Returns the stable identity of this constraint.
    fn id(&self) -> ConstraintId;

    /// Returns the semantic category.
    fn kind(&self) -> ConstraintKind;

    /// Returns a stable human-readable name.
    fn name(&self) -> &str;

    /// Returns the severity assigned to violations.
    fn severity(&self) -> ConstraintSeverity {
        ConstraintSeverity::Error
    }

    /// Determines whether this constraint applies.
    ///
    /// The default implementation applies the constraint universally.
    fn applies(&self, _context: &ConstraintContext<'_>) -> ConstraintApplicability {
        ConstraintApplicability::Applicable
    }

    /// Evaluates the constraint.
    ///
    /// `Ok(())` means this constraint is satisfied.
    ///
    /// `Err(ConstraintViolation)` means the candidate violates this constraint.
    fn evaluate(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation>;

    /// Returns whether this constraint is enabled.
    ///
    /// Disabled constraints are not evaluated by `ConstraintSet`.
    ///
    /// The default is enabled.
    fn is_enabled(&self) -> bool {
        true
    }

    /// Returns whether this constraint can be evaluated in the supplied phase.
    ///
    /// The default applies the constraint in every phase.
    fn supports_phase(&self, _phase: ConstraintPhase) -> bool {
        true
    }
}

// ============================================================================
// Constraint metadata
// ============================================================================

/// Immutable metadata describing a constraint without evaluating it.
///
/// This is useful for diagnostics, scheduling reports, configuration UIs, and
/// plugin inspection without exposing implementation-specific state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintMetadata {
    id: ConstraintId,
    kind: ConstraintKind,
    name: String,
    severity: ConstraintSeverity,
    enabled: bool,
}

impl ConstraintMetadata {
    /// Builds metadata from a constraint.
    #[must_use]
    pub fn from_constraint(constraint: &dyn Constraint) -> Self {
        Self {
            id: constraint.id(),
            kind: constraint.kind(),
            name: constraint.name().to_owned(),
            severity: constraint.severity(),
            enabled: constraint.is_enabled(),
        }
    }

    /// Returns the constraint ID.
    #[must_use]
    pub const fn id(&self) -> ConstraintId {
        self.id
    }

    /// Returns the constraint kind.
    #[must_use]
    pub const fn kind(&self) -> ConstraintKind {
        self.kind
    }

    /// Returns the constraint name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns severity.
    #[must_use]
    pub const fn severity(&self) -> ConstraintSeverity {
        self.severity
    }

    /// Returns whether enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }
}

// ============================================================================
// Constraint evaluation report
// ============================================================================

/// Result of evaluating a set of scheduling constraints.
///
/// The report separates:
///
/// - evaluated constraints;
/// - skipped constraints;
/// - violations.
///
/// This is important for diagnostics and deterministic scheduler analysis.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConstraintEvaluationReport {
    evaluated: u64,
    skipped: u64,
    violations: Vec<ConstraintViolation>,
}

impl ConstraintEvaluationReport {
    /// Creates an empty report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of evaluated constraints.
    #[must_use]
    pub const fn evaluated(&self) -> u64 {
        self.evaluated
    }

    /// Returns the number of skipped/not-applicable constraints.
    #[must_use]
    pub const fn skipped(&self) -> u64 {
        self.skipped
    }

    /// Returns all collected violations.
    #[must_use]
    pub fn violations(&self) -> &[ConstraintViolation] {
        &self.violations
    }

    /// Returns whether the report contains any violation.
    #[must_use]
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }

    /// Returns whether at least one blocking violation exists.
    #[must_use]
    pub fn has_blocking_violations(&self) -> bool {
        self.violations
            .iter()
            .any(|violation| violation.severity().is_blocking())
    }

    /// Returns the first violation, if any.
    #[must_use]
    pub fn first_violation(&self) -> Option<&ConstraintViolation> {
        self.violations.first()
    }

    fn record_evaluated(&mut self) {
        self.evaluated = self.evaluated.saturating_add(1);
    }

    fn record_skipped(&mut self) {
        self.skipped = self.skipped.saturating_add(1);
    }

    fn record_violation(&mut self, violation: ConstraintViolation) {
        self.violations.push(violation);
    }
}

// ============================================================================
// Constraint-set errors
// ============================================================================

/// Structural error produced when constructing a constraint collection.
///
/// Evaluation failures are represented by `ConstraintViolation`, not this
/// error type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintSetError {
    /// Two constraints have the same stable identifier.
    DuplicateId(ConstraintId),
}

impl fmt::Display for ConstraintSetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId(id) => {
                write!(formatter, "duplicate scheduling constraint identifier: {id}")
            }
        }
    }
}

impl std::error::Error for ConstraintSetError {}

// ============================================================================
// Constraint set
// ============================================================================

/// Deterministic collection of heterogeneous scheduling constraints.
///
/// `ConstraintSet` owns constraint objects but does not own scheduling state.
///
/// Constraints are ordered by `ConstraintId`.
///
/// This gives deterministic evaluation independent of insertion order.
///
/// The collection has no fixed capacity or machine-size limit.
pub struct ConstraintSet {
    constraints: Vec<Box<dyn Constraint>>,
}

impl ConstraintSet {
    /// Creates an empty constraint set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    /// Returns the number of registered constraints.
    #[must_use]
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Returns whether no constraints are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    /// Adds a constraint while preserving deterministic ID ordering.
    ///
    /// Duplicate identifiers are rejected.
    pub fn insert(
        &mut self,
        constraint: Box<dyn Constraint>,
    ) -> Result<(), ConstraintSetError> {
        let id = constraint.id();

        if self
            .constraints
            .binary_search_by_key(&id, |candidate| candidate.id())
            .is_ok()
        {
            return Err(ConstraintSetError::DuplicateId(id));
        }

        let position = match self
            .constraints
            .binary_search_by_key(&id, |candidate| candidate.id())
        {
            Ok(position) | Err(position) => position,
        };

        self.constraints.insert(position, constraint);
        Ok(())
    }

    /// Removes a constraint by identifier.
    pub fn remove(&mut self, id: ConstraintId) -> Option<Box<dyn Constraint>> {
        let position = self
            .constraints
            .binary_search_by_key(&id, |candidate| candidate.id())
            .ok()?;

        Some(self.constraints.remove(position))
    }

    /// Finds a constraint by identifier.
    #[must_use]
    pub fn get(&self, id: ConstraintId) -> Option<&dyn Constraint> {
        let position = self
            .constraints
            .binary_search_by_key(&id, |candidate| candidate.id())
            .ok()?;

        Some(self.constraints[position].as_ref())
    }

    /// Returns all constraints in deterministic order.
    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = &dyn Constraint> {
        self.constraints.iter().map(Box::as_ref)
    }

    /// Returns metadata for all registered constraints.
    #[must_use]
    pub fn metadata(&self) -> Vec<ConstraintMetadata> {
        self.iter()
            .map(ConstraintMetadata::from_constraint)
            .collect()
    }

    /// Evaluates constraints according to the requested evaluation mode.
    ///
    /// Disabled and inapplicable constraints are skipped.
    ///
    /// Constraints are evaluated in deterministic `ConstraintId` order.
    #[must_use]
    pub fn evaluate(
        &self,
        context: &ConstraintContext<'_>,
        mode: ConstraintEvaluationMode,
    ) -> ConstraintEvaluationReport {
        let mut report = ConstraintEvaluationReport::new();

        for constraint in &self.constraints {
            if !constraint.is_enabled()
                || !constraint.supports_phase(context.phase())
                || !constraint.applies(context).is_applicable()
            {
                report.record_skipped();
                continue;
            }

            report.record_evaluated();

            match constraint.evaluate(context) {
                Ok(()) => {}
                Err(mut violation) => {
                    if violation.severity() == ConstraintSeverity::Error
                        && constraint.severity() != ConstraintSeverity::Error
                    {
                        violation = ConstraintViolation::new(
                            violation.constraint(),
                            violation.kind(),
                            constraint.severity(),
                            violation.reason().to_owned(),
                        )
                        .with_optional_context(&violation);
                    }

                    let blocking = violation.severity().is_blocking();

                    report.record_violation(violation);

                    if mode == ConstraintEvaluationMode::FirstFailure && blocking {
                        break;
                    }
                }
            }
        }

        report
    }

    /// Returns whether every applicable enabled constraint accepts the
    /// candidate.
    #[must_use]
    pub fn accepts(&self, context: &ConstraintContext<'_>) -> bool {
        !self
            .evaluate(context, ConstraintEvaluationMode::FirstFailure)
            .has_blocking_violations()
    }
}

impl Default for ConstraintSet {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Constraint-set inspection
// ============================================================================

/// Immutable summary of a constraint set.
///
/// This is intentionally inexpensive to construct and contains no implementation
/// objects.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConstraintSetSummary {
    total: u64,
    enabled: u64,
    disabled: u64,
    kinds: BTreeSet<ConstraintKind>,
}

impl ConstraintSetSummary {
    /// Builds a summary from a constraint set.
    #[must_use]
    pub fn from_set(set: &ConstraintSet) -> Self {
        let mut summary = Self::default();

        for constraint in set.iter() {
            summary.total = summary.total.saturating_add(1);

            if constraint.is_enabled() {
                summary.enabled = summary.enabled.saturating_add(1);
            } else {
                summary.disabled = summary.disabled.saturating_add(1);
            }

            summary.kinds.insert(constraint.kind());
        }

        summary
    }

    /// Returns total registered constraints.
    #[must_use]
    pub const fn total(&self) -> u64 {
        self.total
    }

    /// Returns enabled constraints.
    #[must_use]
    pub const fn enabled(&self) -> u64 {
        self.enabled
    }

    /// Returns disabled constraints.
    #[must_use]
    pub const fn disabled(&self) -> u64 {
        self.disabled
    }

    /// Returns semantic categories represented by the set.
    #[must_use]
    pub fn kinds(&self) -> &BTreeSet<ConstraintKind> {
        &self.kinds
    }
}

// ============================================================================
// Built-in generic constraints
// ============================================================================

/// Validates that a candidate has a representable finish time.
///
/// This is deliberately a generic arithmetic invariant rather than a hardware
/// timing constraint.
///
/// It is useful as a baseline guard before specialized timing constraints are
/// evaluated.
#[derive(Debug, Clone, Copy)]
pub struct RepresentableEndConstraint {
    id: ConstraintId,
}

impl RepresentableEndConstraint {
    /// Creates the constraint.
    #[must_use]
    pub const fn new(id: ConstraintId) -> Self {
        Self { id }
    }
}

impl Constraint for RepresentableEndConstraint {
    fn id(&self) -> ConstraintId {
        self.id
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Timing
    }

    fn name(&self) -> &str {
        "representable_end"
    }

    fn evaluate(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        if context.candidate().checked_end().is_some() {
            return Ok(());
        }

        Err(
            ConstraintViolation::new(
                self.id,
                self.kind(),
                self.severity(),
                "candidate start time plus duration exceeds the representable schedule coordinate",
            )
            .with_operation(context.candidate().operation())
            .with_timing(
                context.candidate().start(),
                context.candidate().duration(),
            ),
        )
    }
}

/// Validates that every requested resource has a non-zero claim when present.
///
/// This does not determine resource capacity. That belongs to
/// `resources/*`.
///
/// It only prevents meaningless resource reservations from entering the
/// scheduling pipeline.
#[derive(Debug, Clone, Copy)]
pub struct NonZeroResourceClaimConstraint {
    id: ConstraintId,
}

impl NonZeroResourceClaimConstraint {
    /// Creates the constraint.
    #[must_use]
    pub const fn new(id: ConstraintId) -> Self {
        Self { id }
    }
}

impl Constraint for NonZeroResourceClaimConstraint {
    fn id(&self) -> ConstraintId {
        self.id
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Resource
    }

    fn name(&self) -> &str {
        "non_zero_resource_claim"
    }

    fn evaluate(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        for claim in context.candidate().resource_claims() {
            if claim.is_zero() {
                return Err(
                    ConstraintViolation::new(
                        self.id,
                        self.kind(),
                        self.severity(),
                        "resource claim quantity must be non-zero",
                    )
                    .with_operation(context.candidate().operation())
                    .with_resource(claim.resource()),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Constraint helpers
// ============================================================================

impl ConstraintViolation {
    /// Copies contextual fields from another violation.
    ///
    /// This is used internally when a collection applies policy-level severity
    /// metadata without losing diagnostic context.
    #[must_use]
    fn with_optional_context(mut self, source: &Self) -> Self {
        self.operation = source.operation;
        self.resource = source.resource;
        self.logical_qubit = source.logical_qubit;
        self.physical_qubit = source.physical_qubit;
        self.requested_start = source.requested_start;
        self.requested_duration = source.requested_duration;
        self
    }
}

// ============================================================================
// Constraint comparison helpers
// ============================================================================

/// Deterministic ordering helper for violations.
///
/// Violations are primarily ordered by:
///
/// 1. severity;
/// 2. constraint ID;
/// 3. operation identity;
/// 4. resource identity.
///
/// This does not define scheduler semantics. It only provides deterministic
/// diagnostic ordering.
pub fn compare_violations(
    left: &ConstraintViolation,
    right: &ConstraintViolation,
) -> Ordering {
    right
        .severity()
        .cmp(&left.severity())
        .then_with(|| left.constraint().cmp(&right.constraint()))
        .then_with(|| left.operation().cmp(&right.operation()))
        .then_with(|| left.resource().cmp(&right.resource()))
        .then_with(|| left.kind().cmp(&right.kind()))
}

// ============================================================================
// Constraint validation helpers
// ============================================================================

/// Validates a candidate's basic structural invariants.
///
/// This helper does not evaluate hardware, topology, timing policy, or
/// resource capacity.
///
/// It is useful before invoking specialized constraints.
///
/// Errors are returned as generic `ConstraintViolation` values so callers can
/// integrate the helper into the normal constraint pipeline.
pub fn validate_candidate_structure(
    constraint_id: ConstraintId,
    candidate: &SchedulingCandidate<'_>,
) -> Result<(), ConstraintViolation> {
    if candidate.logical_qubits().len() != candidate.physical_qubits().len()
        && !candidate.physical_qubits().is_empty()
    {
        return Err(
            ConstraintViolation::new(
                constraint_id,
                ConstraintKind::Qubit,
                ConstraintSeverity::Error,
                "logical and physical qubit mappings have incompatible lengths",
            )
            .with_operation(candidate.operation()),
        );
    }

    if candidate.checked_end().is_none() {
        return Err(
            ConstraintViolation::new(
                constraint_id,
                ConstraintKind::Timing,
                ConstraintSeverity::Error,
                "candidate end time is not representable",
            )
            .with_operation(candidate.operation())
            .with_timing(candidate.start(), candidate.duration()),
        );
    }

    Ok(())
}

// ============================================================================
// Resource-overlap helper
// ============================================================================

/// Tests whether two half-open scheduling intervals overlap.
///
/// Intervals are interpreted as:
///
/// ```text
/// [start, end)
/// ```
///
/// Therefore an operation ending exactly when another starts does not overlap.
///
/// This helper is independent of resource capacity and is suitable for
/// exclusive-resource constraints.
#[must_use]
pub fn intervals_overlap(
    first_start: TimePoint,
    first_duration: Duration,
    second_start: TimePoint,
    second_duration: Duration,
) -> bool {
    let Some(first_end) = first_start.checked_add(first_duration) else {
        return true;
    };

    let Some(second_end) = second_start.checked_add(second_duration) else {
        return true;
    };

    first_start < second_end && second_start < first_end
}

/// Finds an existing reservation that overlaps a candidate resource claim.
///
/// This helper intentionally performs a linear scan over the supplied
/// immutable reservation view.
///
/// Resource calendars should use their own indexed/interval data structures
/// when scale requires it; this generic constraint module must not impose one
/// calendar implementation on every scheduler architecture.
#[must_use]
pub fn find_overlapping_reservation(
    candidate: &SchedulingCandidate<'_>,
    claim: ConstraintResourceClaim,
    reservations: &[ConstraintReservationView],
) -> Option<ConstraintReservationView> {
    reservations.iter().copied().find(|reservation| {
        reservation.resource() == claim.resource()
            && intervals_overlap(
                candidate.start(),
                candidate.duration(),
                reservation.start(),
                reservation.duration(),
            )
    })
}

// ============================================================================
// Qubit helpers
// ============================================================================

/// Tests whether a candidate contains a logical qubit.
///
/// This uses the canonical `quantum::ir::qubit::QubitId`.
#[must_use]
pub fn candidate_uses_logical_qubit(
    candidate: &SchedulingCandidate<'_>,
    qubit: QubitId,
) -> bool {
    candidate
        .logical_qubits()
        .iter()
        .any(|candidate_qubit| *candidate_qubit == qubit)
}

/// Tests whether a candidate contains a physical qubit.
///
/// This uses the canonical `quantum::ir::qubit::PhysicalQubitId`.
#[must_use]
pub fn candidate_uses_physical_qubit(
    candidate: &SchedulingCandidate<'_>,
    qubit: PhysicalQubitId,
) -> bool {
    candidate
        .physical_qubits()
        .iter()
        .any(|candidate_qubit| *candidate_qubit == qubit)
}

// ============================================================================
// Resource helpers
// ============================================================================

/// Returns the total quantity claimed for one resource.
///
/// Checked addition is used so the helper never wraps on overflow.
///
/// `None` means the total is not representable.
#[must_use]
pub fn checked_resource_quantity(
    candidate: &SchedulingCandidate<'_>,
    resource: ResourceId,
) -> Option<u128> {
    candidate
        .resource_claims()
        .iter()
        .filter(|claim| claim.resource() == resource)
        .try_fold(0_u128, |total, claim| {
            total.checked_add(claim.quantity())
        })
}

// ============================================================================
// Public module tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::core::identity::ResourceId;

    #[derive(Debug)]
    struct AlwaysAccept {
        id: ConstraintId,
    }

    impl Constraint for AlwaysAccept {
        fn id(&self) -> ConstraintId {
            self.id
        }

        fn kind(&self) -> ConstraintKind {
            ConstraintKind::Generic
        }

        fn name(&self) -> &str {
            "always_accept"
        }

        fn evaluate(
            &self,
            _context: &ConstraintContext<'_>,
        ) -> Result<(), ConstraintViolation> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct AlwaysReject {
        id: ConstraintId,
    }

    impl Constraint for AlwaysReject {
        fn id(&self) -> ConstraintId {
            self.id
        }

        fn kind(&self) -> ConstraintKind {
            ConstraintKind::Generic
        }

        fn name(&self) -> &str {
            "always_reject"
        }

        fn evaluate(
            &self,
            context: &ConstraintContext<'_>,
        ) -> Result<(), ConstraintViolation> {
            Err(
                ConstraintViolation::new(
                    self.id,
                    self.kind(),
                    ConstraintSeverity::Error,
                    "candidate rejected by test constraint",
                )
                .with_operation(context.candidate().operation()),
            )
        }
    }

    fn test_candidate<'a>(
        logical: &'a [QubitId],
        physical: &'a [PhysicalQubitId],
        claims: &'a [ConstraintResourceClaim],
    ) -> SchedulingCandidate<'a> {
        SchedulingCandidate::new(
            OperationId::from(1_u64),
            logical,
            physical,
            claims,
            TimePoint::ZERO,
            Duration::new(10),
        )
    }

    #[test]
    fn constraint_id_is_stable() {
        let id = ConstraintId::new(42);

        assert_eq!(id.value(), 42);
        assert_eq!(id.to_string(), "constraint:42");
        assert_eq!(id.checked_next(), Some(ConstraintId::new(43)));
    }

    #[test]
    fn candidate_end_is_checked() {
        let candidate = SchedulingCandidate::new(
            OperationId::from(1_u64),
            &[],
            &[],
            &[],
            TimePoint::new(u128::MAX),
            Duration::new(1),
        );

        assert_eq!(candidate.checked_end(), None);
    }

    #[test]
    fn zero_duration_intervals_do_not_overlap() {
        assert!(!intervals_overlap(
            TimePoint::new(10),
            Duration::ZERO,
            TimePoint::new(10),
            Duration::new(5),
        ));
    }

    #[test]
    fn_touching_intervals_do_not_overlap() {
        assert!(!intervals_overlap(
            TimePoint::new(0),
            Duration::new(10),
            TimePoint::new(10),
            Duration::new(10),
        ));
    }

    #[test]
    fn overlapping_intervals_are_detected() {
        assert!(intervals_overlap(
            TimePoint::new(0),
            Duration::new(10),
            TimePoint::new(5),
            Duration::new(10),
        ));
    }

    #[test]
    fn resource_quantity_is_checked() {
        let resource = ResourceId::from(7_u64);

        let claims = [
            ConstraintResourceClaim::new(resource, 3),
            ConstraintResourceClaim::new(resource, 4),
        ];

        let candidate = test_candidate(&[], &[], &claims);

        assert_eq!(checked_resource_quantity(&candidate, resource), Some(7));
    }

    #[test]
    fn constraint_set_is_deterministically_ordered() {
        let mut set = ConstraintSet::new();

        set.insert(Box::new(AlwaysAccept {
            id: ConstraintId::new(20),
        }))
        .expect("first insertion must succeed");

        set.insert(Box::new(AlwaysAccept {
            id: ConstraintId::new(10),
        }))
        .expect("second insertion must succeed");

        let ids: Vec<ConstraintId> = set.iter().map(Constraint::id).collect();

        assert_eq!(
            ids,
            vec![ConstraintId::new(10), ConstraintId::new(20)]
        );
    }

    #[test]
    fn duplicate_constraint_ids_are_rejected() {
        let mut set = ConstraintSet::new();

        set.insert(Box::new(AlwaysAccept {
            id: ConstraintId::new(1),
        }))
        .expect("first insertion must succeed");

        let result = set.insert(Box::new(AlwaysAccept {
            id: ConstraintId::new(1),
        }));

        assert_eq!(
            result,
            Err(ConstraintSetError::DuplicateId(
                ConstraintId::new(1)
            ))
        );
    }

    #[test]
    fn first_failure_stops_on_blocking_violation() {
        let mut set = ConstraintSet::new();

        set.insert(Box::new(AlwaysReject {
            id: ConstraintId::new(1),
        }))
        .expect("insertion must succeed");

        set.insert(Box::new(AlwaysAccept {
            id: ConstraintId::new(2),
        }))
        .expect("insertion must succeed");

        let candidate = test_candidate(&[], &[], &[]);
        let state = ConstraintState::new(&[], &[], &[]);
        let context =
            ConstraintContext::new(
                &candidate,
                &state,
                ConstraintPhase::Planning,
            );

        let report =
            set.evaluate(&context, ConstraintEvaluationMode::FirstFailure);

        assert_eq!(report.evaluated(), 1);
        assert_eq!(report.skipped(), 0);
        assert_eq!(report.violations().len(), 1);
    }

    #[test]
    fn collect_all_reports_all_failures() {
        let mut set = ConstraintSet::new();

        set.insert(Box::new(AlwaysReject {
            id: ConstraintId::new(1),
        }))
        .expect("insertion must succeed");

        set.insert(Box::new(AlwaysReject {
            id: ConstraintId::new(2),
        }))
        .expect("insertion must succeed");

        let candidate = test_candidate(&[], &[], &[]);
        let state = ConstraintState::new(&[], &[], &[]);
        let context =
            ConstraintContext::new(
                &candidate,
                &state,
                ConstraintPhase::Planning,
            );

        let report =
            set.evaluate(&context, ConstraintEvaluationMode::CollectAll);

        assert_eq!(report.evaluated(), 2);
        assert_eq!(report.violations().len(), 2);
    }

    #[test]
    fn representable_end_constraint_accepts_valid_candidate() {
        let constraint =
            RepresentableEndConstraint::new(ConstraintId::new(1));

        let candidate = test_candidate(&[], &[], &[]);
        let state = ConstraintState::new(&[], &[], &[]);
        let context =
            ConstraintContext::new(
                &candidate,
                &state,
                ConstraintPhase::Planning,
            );

        assert!(constraint.evaluate(&context).is_ok());
    }

    #[test]
    fn non_zero_resource_constraint_rejects_zero_claim() {
        let resource = ResourceId::from(1_u64);

        let claims = [
            ConstraintResourceClaim::new(resource, 0),
        ];

        let candidate = test_candidate(&[], &[], &claims);
        let state = ConstraintState::new(&[], &[], &[]);
        let context =
            ConstraintContext::new(
                &candidate,
                &state,
                ConstraintPhase::Planning,
            );

        let constraint =
            NonZeroResourceClaimConstraint::new(ConstraintId::new(1));

        assert!(constraint.evaluate(&context).is_err());
    }
}