//! Zamani Quantum Scheduling — Production Scheduling Results
//!
//! This module defines the immutable, inspectable result artifacts produced by
//! the Zamani quantum scheduling subsystem.
//!
//! # Architectural responsibility
//!
//! This file answers:
//!
//! > "What exactly did the scheduler produce, what quality does it have,
//! > what resources and times were assigned, and can the result be trusted?"
//!
//! This module owns:
//!
//! - completed scheduling result metadata;
//! - scheduled-operation records;
//! - resource-usage summaries;
//! - schedule metrics;
//! - critical-path metrics;
//! - idle-time metrics;
//! - verification summaries;
//! - provenance;
//! - diagnostics;
//! - deterministic result identity;
//! - result status;
//! - result composition and inspection;
//! - immutable result snapshots.
//!
//! This module does NOT own:
//!
//! - quantum operation semantics;
//! - logical qubit identity;
//! - physical qubit identity;
//! - routing;
//! - hardware discovery;
//! - hardware execution;
//! - calibration;
//! - pulse synthesis;
//! - QEC algorithms;
//! - scheduling algorithms;
//! - optimization algorithms;
//! - frontend syntax;
//! - runtime execution.
//!
//! Those responsibilities remain in their canonical subsystems.
//!
//! # Canonical identity ownership
//!
//! Quantum identities MUST come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Operation and resource identities MUST come from:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! This file intentionally defines none of those identities.
//!
//! # Write once, scale everywhere
//!
//! A scheduling result must not contain architectural assumptions such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_DEPTH
//! MAX_CHANNELS
//! ```
//!
//! Collections grow according to the actual compilation request and the
//! available memory/resources.
//!
//! "Infinity" means that this result representation introduces no artificial
//! finite quantum-machine ceiling.
//!
//! A concrete execution remains finite because the compiler process, address
//! space, target, and execution request are finite.
//!
//! # Separation of concerns
//!
//! The relationship is:
//!
//! ```text
//! quantum::ir
//!     │
//!     │ WHAT
//!     ▼
//! routing
//!     │
//!     │ WHERE
//!     ▼
//! scheduling
//!     │
//!     │ WHEN
//!     ▼
//! SchedulingResult
//!     │
//!     ├── verification
//!     ├── diagnostics
//!     ├── benchmarking
//!     ├── hardware lowering
//!     └── runtime
//! ```
//!
//! A `SchedulingResult` does not mean that hardware execution has succeeded.
//!
//! It means that a scheduler produced a temporal arrangement that passed the
//! verification level represented by its result status.
//!
//! # Immutability
//!
//! Result artifacts are constructed through `SchedulingResultBuilder` and then
//! exposed through immutable APIs.
//!
//! This prevents downstream systems from silently modifying a schedule after
//! verification.
//!
//! If a schedule must change, a new scheduling pass must produce a new result.
//!
//! # Determinism
//!
//! The representation is deterministic:
//!
//! - ordered collections are used where semantic ordering matters;
//! - operation records are sorted deterministically;
//! - resource records are sorted deterministically;
//! - diagnostics retain explicit severity/order;
//! - no memory addresses participate in identity;
//! - no hash-map iteration is exposed as semantic ordering.
//!
//! # Time
//!
//! Scheduling time is represented by the scheduler's foundational `TimePoint`
//! and `Duration` types.
//!
//! Those values have no intrinsic hardware unit.
//!
//! A target timing model determines their physical interpretation.
//!
//! # Overflow
//!
//! No timing arithmetic in this file silently wraps.
//!
//! Calculations use checked arithmetic and return explicit errors through the
//! builder or query APIs.
//!
//! # Thread safety
//!
//! Result structures contain owned immutable data and do not use interior
//! mutability.
//!
//! They are therefore suitable for ownership transfer and concurrent read-only
//! inspection when their contained canonical types satisfy the same contract.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! The safety boundary is compiler-enforced below.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

// =============================================================================
// Canonical repository identities
// =============================================================================
//
// These imports are intentionally direct. In particular, QubitId must not be
// recreated inside scheduling.
//
// The canonical repository contract identifies:
//
//     quantum::ir::qubit::QubitId
//     quantum::ir::qubit::PhysicalQubitId
//
// as the authoritative qubit identity types.

use crate::quantum::ir::core::identity::{
    IrVersion,
    OperationId,
    ResourceId,
};
use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
};

// Scheduler-foundational values are owned by scheduling::types.
//
// If this file is compiled as:
//
//     crate::quantum::scheduling::result
//
// these imports are resolved from the sibling `types` module.

use super::types::{
    DependencyId,
    Duration,
    EpochId,
    ReservationId,
    ScheduleId,
    SchedulerSessionId,
    SchedulingPriority,
    TimePoint,
};

// =============================================================================
// Public result aliases
// =============================================================================

/// Result returned by result construction and result transformations.
pub type ResultArtifact<T> = Result<T, SchedulingResultError>;

// =============================================================================
// Result errors
// =============================================================================

/// Errors associated with constructing, validating, or querying a scheduling
/// result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulingResultError {
    /// An operation appears more than once in the result.
    DuplicateOperation {
        /// Duplicated operation identity.
        operation: OperationId,
    },

    /// A result contains an operation with an invalid temporal interval.
    InvalidInterval {
        /// Operation owning the interval.
        operation: OperationId,

        /// Start time.
        start: TimePoint,

        /// End time.
        end: TimePoint,
    },

    /// A scheduled operation references a dependency that does not exist in
    /// the result.
    MissingDependency {
        /// Operation containing the dependency.
        operation: OperationId,

        /// Missing predecessor.
        dependency: OperationId,
    },

    /// A scheduled operation starts before one of its dependencies finishes.
    DependencyViolation {
        /// Dependent operation.
        operation: OperationId,

        /// Predecessor operation.
        dependency: OperationId,
    },

    /// An operation uses a resource more than once in the same reservation
    /// record.
    DuplicateResource {
        /// Operation associated with the duplicate.
        operation: OperationId,

        /// Duplicated resource.
        resource: ResourceId,
    },

    /// Two exclusive resource intervals overlap.
    ResourceConflict {
        /// Resource involved in the conflict.
        resource: ResourceId,

        /// First operation.
        first_operation: OperationId,

        /// Second operation.
        second_operation: OperationId,
    },

    /// A time calculation overflowed.
    TimeOverflow {
        /// Operation involved in the calculation.
        operation: OperationId,
    },

    /// A result cannot be finalized because required information is missing.
    MissingRequiredField {
        /// Name of the missing field.
        field: &'static str,
    },

    /// A result is internally inconsistent.
    InconsistentResult {
        /// Explanation of the inconsistency.
        message: String,
    },

    /// A caller attempted an invalid result transition.
    InvalidStatusTransition {
        /// Current state.
        current: ResultStatus,

        /// Requested state.
        requested: ResultStatus,
    },
}

impl fmt::Display for SchedulingResultError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateOperation { operation } => {
                write!(
                    formatter,
                    "scheduling result contains duplicate operation `{operation}`"
                )
            }

            Self::InvalidInterval {
                operation,
                start,
                end,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` has invalid interval [{start}, {end})"
                )
            }

            Self::MissingDependency {
                operation,
                dependency,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` references missing dependency `{dependency}`"
                )
            }

            Self::DependencyViolation {
                operation,
                dependency,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` starts before dependency `{dependency}` finishes"
                )
            }

            Self::DuplicateResource {
                operation,
                resource,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` contains duplicate resource `{resource}`"
                )
            }

            Self::ResourceConflict {
                resource,
                first_operation,
                second_operation,
            } => {
                write!(
                    formatter,
                    "resource `{resource}` is concurrently reserved by `{first_operation}` and `{second_operation}`"
                )
            }

            Self::TimeOverflow { operation } => {
                write!(
                    formatter,
                    "time calculation overflowed for operation `{operation}`"
                )
            }

            Self::MissingRequiredField { field } => {
                write!(
                    formatter,
                    "required scheduling result field `{field}` is missing"
                )
            }

            Self::InconsistentResult { message } => {
                write!(
                    formatter,
                    "scheduling result is inconsistent: {message}"
                )
            }

            Self::InvalidStatusTransition {
                current,
                requested,
            } => {
                write!(
                    formatter,
                    "invalid scheduling result status transition from `{current}` to `{requested}`"
                )
            }
        }
    }
}

impl Error for SchedulingResultError {}

// =============================================================================
// Result status
// =============================================================================

/// Lifecycle status of a scheduling result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResultStatus {
    /// Result construction has begun but is not yet complete.
    Building,

    /// Scheduling completed but verification has not completed.
    Planned,

    /// Scheduling completed and verification succeeded.
    Verified,

    /// Scheduling completed but the result contains warnings.
    VerifiedWithWarnings,

    /// Scheduling failed and must not be treated as executable.
    Failed,

    /// The result is valid for inspection but was intentionally marked
    /// analysis-only.
    AnalysisOnly,
}

impl ResultStatus {
    /// Returns whether this status represents a successfully constructed
    /// schedule.
    #[must_use]
    pub const fn has_schedule(self) -> bool {
        matches!(
            self,
            Self::Planned
                | Self::Verified
                | Self::VerifiedWithWarnings
                | Self::AnalysisOnly
        )
    }

    /// Returns whether the schedule passed verification.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(
            self,
            Self::Verified | Self::VerifiedWithWarnings
        )
    }

    /// Returns whether the result may be treated as verified for downstream
    /// compilation.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        matches!(
            self,
            Self::Verified | Self::VerifiedWithWarnings
        )
    }

    /// Returns whether this result contains a fatal failure.
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }
}

impl fmt::Display for ResultStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Building => "building",
            Self::Planned => "planned",
            Self::Verified => "verified",
            Self::VerifiedWithWarnings => "verified-with-warnings",
            Self::Failed => "failed",
            Self::AnalysisOnly => "analysis-only",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Scheduled interval
// =============================================================================

/// Immutable half-open scheduling interval.
///
/// The interval is:
///
/// ```text
/// [start, end)
/// ```
///
/// This means an operation occupying `[0, 10)` does not conflict with an
/// operation beginning at `10`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScheduledInterval {
    start: TimePoint,
    end: TimePoint,
}

impl ScheduledInterval {
    /// Creates a validated interval.
    pub fn new(
        start: TimePoint,
        end: TimePoint,
    ) -> ResultArtifact<Self> {
        if end < start {
            return Err(
                SchedulingResultError::InvalidInterval {
                    operation: OperationId::new(0),
                    start,
                    end,
                },
            );
        }

        Ok(Self { start, end })
    }

    /// Creates an interval associated with an operation.
    pub fn for_operation(
        operation: OperationId,
        start: TimePoint,
        end: TimePoint,
    ) -> ResultArtifact<Self> {
        if end < start {
            return Err(
                SchedulingResultError::InvalidInterval {
                    operation,
                    start,
                    end,
                },
            );
        }

        Ok(Self { start, end })
    }

    /// Returns the start time.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.start
    }

    /// Returns the end time.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.end
    }

    /// Returns the duration.
    #[must_use]
    pub fn duration(self) -> Option<Duration> {
        self.start.checked_duration_until(self.end)
    }

    /// Returns whether the interval is empty.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns whether two half-open intervals overlap.
    #[must_use]
    pub fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Returns whether the intervals touch but do not overlap.
    #[must_use]
    pub fn touches(self, other: Self) -> bool {
        self.end == other.start || other.end == self.start
    }

    /// Returns whether this interval contains a time point.
    #[must_use]
    pub fn contains(self, time: TimePoint) -> bool {
        self.start <= time && time < self.end
    }
}

// =============================================================================
// Scheduled resource reservation
// =============================================================================

/// A concrete reservation of an abstract scheduler resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResultReservation {
    reservation_id: ReservationId,
    resource_id: ResourceId,
    operation_id: OperationId,
    interval: ScheduledInterval,
}

impl ResultReservation {
    /// Creates a validated reservation.
    pub fn new(
        reservation_id: ReservationId,
        resource_id: ResourceId,
        operation_id: OperationId,
        start: TimePoint,
        end: TimePoint,
    ) -> ResultArtifact<Self> {
        let interval =
            ScheduledInterval::for_operation(
                operation_id,
                start,
                end,
            )?;

        Ok(Self {
            reservation_id,
            resource_id,
            operation_id,
            interval,
        })
    }

    /// Returns the reservation identity.
    #[must_use]
    pub const fn reservation_id(self) -> ReservationId {
        self.reservation_id
    }

    /// Returns the resource identity.
    #[must_use]
    pub const fn resource_id(self) -> ResourceId {
        self.resource_id
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation_id(self) -> OperationId {
        self.operation_id
    }

    /// Returns the interval.
    #[must_use]
    pub const fn interval(self) -> ScheduledInterval {
        self.interval
    }
}

// =============================================================================
// Scheduled operation
// =============================================================================

/// Immutable record describing when one canonical IR operation was scheduled.
///
/// This is deliberately a scheduling record rather than a replacement for the
/// canonical IR operation.
///
/// The actual semantic operation remains owned by `quantum::ir`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledOperation {
    operation_id: OperationId,
    interval: ScheduledInterval,
    predecessors: BTreeSet<OperationId>,
    reservations: Vec<ReservationId>,
    logical_qubits: Vec<QubitId>,
    physical_qubits: Vec<PhysicalQubitId>,
    priority: SchedulingPriority,
}

impl ScheduledOperation {
    /// Creates a scheduled operation with no resource reservations.
    pub fn new(
        operation_id: OperationId,
        start: TimePoint,
        end: TimePoint,
    ) -> ResultArtifact<Self> {
        let interval =
            ScheduledInterval::for_operation(
                operation_id,
                start,
                end,
            )?;

        Ok(Self {
            operation_id,
            interval,
            predecessors: BTreeSet::new(),
            reservations: Vec::new(),
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
            priority: SchedulingPriority::DEFAULT,
        })
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the scheduled interval.
    #[must_use]
    pub const fn interval(&self) -> ScheduledInterval {
        self.interval
    }

    /// Returns the start time.
    #[must_use]
    pub const fn start(&self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the end time.
    #[must_use]
    pub const fn end(&self) -> TimePoint {
        self.interval.end()
    }

    /// Returns the operation duration.
    #[must_use]
    pub fn duration(&self) -> Option<Duration> {
        self.interval.duration()
    }

    /// Returns predecessor operations.
    #[must_use]
    pub fn predecessors(&self) -> &BTreeSet<OperationId> {
        &self.predecessors
    }

    /// Returns resource reservation identities.
    #[must_use]
    pub fn reservations(&self) -> &[ReservationId] {
        &self.reservations
    }

    /// Returns logical qubits associated with this scheduling record.
    ///
    /// The type is the canonical
    /// `quantum::ir::qubit::QubitId`.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns physical qubits associated with this scheduling record.
    ///
    /// The type is the canonical
    /// `quantum::ir::qubit::PhysicalQubitId`.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns scheduling priority.
    #[must_use]
    pub const fn priority(&self) -> SchedulingPriority {
        self.priority
    }

    /// Adds a dependency.
    pub fn add_predecessor(
        &mut self,
        predecessor: OperationId,
    ) {
        if predecessor != self.operation_id {
            self.predecessors.insert(predecessor);
        }
    }

    /// Adds a reservation identity.
    ///
    /// Duplicate reservation identities are ignored.
    pub fn add_reservation(
        &mut self,
        reservation: ReservationId,
    ) {
        if !self.reservations.contains(&reservation) {
            self.reservations.push(reservation);
            self.reservations.sort();
        }
    }

    /// Adds a canonical logical qubit.
    ///
    /// Duplicate qubit identities are ignored.
    pub fn add_logical_qubit(
        &mut self,
        qubit: QubitId,
    ) {
        if !self.logical_qubits.contains(&qubit) {
            self.logical_qubits.push(qubit);
            self.logical_qubits.sort();
        }
    }

    /// Adds a canonical physical qubit.
    ///
    /// Duplicate qubit identities are ignored.
    pub fn add_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) {
        if !self.physical_qubits.contains(&qubit) {
            self.physical_qubits.push(qubit);
            self.physical_qubits.sort();
        }
    }

    /// Sets the scheduling priority.
    pub fn set_priority(
        &mut self,
        priority: SchedulingPriority,
    ) {
        self.priority = priority;
    }
}

// =============================================================================
// Resource usage
// =============================================================================

/// Aggregate usage statistics for one scheduling resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceUsage {
    resource_id: ResourceId,
    reservations: Vec<ReservationId>,
    operations: BTreeSet<OperationId>,
    first_use: Option<TimePoint>,
    last_use: Option<TimePoint>,
    busy_duration: Duration,
}

impl ResourceUsage {
    /// Creates an empty resource-usage record.
    #[must_use]
    pub fn new(resource_id: ResourceId) -> Self {
        Self {
            resource_id,
            reservations: Vec::new(),
            operations: BTreeSet::new(),
            first_use: None,
            last_use: None,
            busy_duration: Duration::ZERO,
        }
    }

    /// Returns the resource identity.
    #[must_use]
    pub const fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    /// Returns reservation identities.
    #[must_use]
    pub fn reservations(&self) -> &[ReservationId] {
        &self.reservations
    }

    /// Returns operations using the resource.
    #[must_use]
    pub fn operations(&self) -> &BTreeSet<OperationId> {
        &self.operations
    }

    /// Returns first resource use.
    #[must_use]
    pub const fn first_use(&self) -> Option<TimePoint> {
        self.first_use
    }

    /// Returns last resource use.
    #[must_use]
    pub const fn last_use(&self) -> Option<TimePoint> {
        self.last_use
    }

    /// Returns aggregate busy duration.
    #[must_use]
    pub const fn busy_duration(&self) -> Duration {
        self.busy_duration
    }

    /// Records one reservation.
    pub fn record(
        &mut self,
        reservation: ResultReservation,
    ) -> ResultArtifact<()> {
        if reservation.resource_id() != self.resource_id {
            return Err(
                SchedulingResultError::InconsistentResult {
                    message:
                        "reservation resource does not match resource usage"
                            .to_owned(),
                },
            );
        }

        let reservation_id =
            reservation.reservation_id();

        if !self.reservations.contains(&reservation_id) {
            self.reservations.push(reservation_id);
            self.reservations.sort();
        }

        self.operations
            .insert(reservation.operation_id());

        let interval = reservation.interval();

        self.first_use = match self.first_use {
            Some(current) => Some(current.min(interval.start())),
            None => Some(interval.start()),
        };

        self.last_use = match self.last_use {
            Some(current) => Some(current.max(interval.end())),
            None => Some(interval.end()),
        };

        let duration = interval.duration().ok_or(
            SchedulingResultError::TimeOverflow {
                operation: reservation.operation_id(),
            },
        )?;

        self.busy_duration = self
            .busy_duration
            .checked_add(duration)
            .ok_or(
                SchedulingResultError::TimeOverflow {
                    operation: reservation.operation_id(),
                },
            )?;

        Ok(())
    }
}

// =============================================================================
// Critical path
// =============================================================================

/// Critical-path information for a completed schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CriticalPathMetrics {
    critical_path_duration: Duration,
    operation_count: u128,
}

impl CriticalPathMetrics {
    /// Creates critical-path metrics.
    #[must_use]
    pub const fn new(
        critical_path_duration: Duration,
        operation_count: u128,
    ) -> Self {
        Self {
            critical_path_duration,
            operation_count,
        }
    }

    /// Returns the critical-path duration.
    #[must_use]
    pub const fn critical_path_duration(
        self,
    ) -> Duration {
        self.critical_path_duration
    }

    /// Returns the number of operations on the critical path.
    #[must_use]
    pub const fn operation_count(self) -> u128 {
        self.operation_count
    }
}

// =============================================================================
// Schedule metrics
// =============================================================================

/// Aggregate quantitative metrics for a scheduling result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleMetrics {
    operation_count: u128,
    dependency_count: u128,
    resource_count: u128,
    reservation_count: u128,
    makespan: Duration,
    critical_path: Option<CriticalPathMetrics>,
    total_busy_time: Duration,
    total_idle_time: Duration,
    maximum_parallelism: u128,
    average_parallelism_numerator: u128,
    average_parallelism_denominator: u128,
}

impl Default for ScheduleMetrics {
    fn default() -> Self {
        Self {
            operation_count: 0,
            dependency_count: 0,
            resource_count: 0,
            reservation_count: 0,
            makespan: Duration::ZERO,
            critical_path: None,
            total_busy_time: Duration::ZERO,
            total_idle_time: Duration::ZERO,
            maximum_parallelism: 0,
            average_parallelism_numerator: 0,
            average_parallelism_denominator: 0,
        }
    }
}

impl ScheduleMetrics {
    /// Creates metrics for a schedule.
    #[must_use]
    pub fn new(
        operation_count: u128,
        dependency_count: u128,
        resource_count: u128,
        reservation_count: u128,
        makespan: Duration,
    ) -> Self {
        Self {
            operation_count,
            dependency_count,
            resource_count,
            reservation_count,
            makespan,
            ..Self::default()
        }
    }

    /// Returns operation count.
    #[must_use]
    pub const fn operation_count(&self) -> u128 {
        self.operation_count
    }

    /// Returns dependency count.
    #[must_use]
    pub const fn dependency_count(&self) -> u128 {
        self.dependency_count
    }

    /// Returns resource count.
    #[must_use]
    pub const fn resource_count(&self) -> u128 {
        self.resource_count
    }

    /// Returns reservation count.
    #[must_use]
    pub const fn reservation_count(&self) -> u128 {
        self.reservation_count
    }

    /// Returns makespan.
    #[must_use]
    pub const fn makespan(&self) -> Duration {
        self.makespan
    }

    /// Returns critical-path metrics.
    #[must_use]
    pub const fn critical_path(
        &self,
    ) -> Option<CriticalPathMetrics> {
        self.critical_path
    }

    /// Returns total resource busy time.
    #[must_use]
    pub const fn total_busy_time(&self) -> Duration {
        self.total_busy_time
    }

    /// Returns total idle time.
    #[must_use]
    pub const fn total_idle_time(&self) -> Duration {
        self.total_idle_time
    }

    /// Returns maximum observed parallelism.
    #[must_use]
    pub const fn maximum_parallelism(&self) -> u128 {
        self.maximum_parallelism
    }

    /// Returns average-parallelism numerator.
    #[must_use]
    pub const fn average_parallelism_numerator(&self) -> u128 {
        self.average_parallelism_numerator
    }

    /// Returns average-parallelism denominator(&self) -> u128 {
        self.average_parallelism_denominator
    }

    /// Sets critical-path metrics.
    pub fn set_critical_path(
        &mut self,
        metrics: CriticalPathMetrics,
    ) {
        self.critical_path = Some(metrics);
    }

    /// Sets total busy time.
    pub fn set_total_busy_time(
        &mut self,
        value: Duration,
    ) {
        self.total_busy_time = value;
    }

    /// Sets total idle time.
    pub fn set_total_idle_time(
        &mut self,
        value: Duration,
    ) {
        self.total_idle_time = value;
    }

    /// Sets maximum parallelism.
    pub fn set_maximum_parallelism(
        &mut self,
        value: u128,
    ) {
        self.maximum_parallelism = value;
    }

    /// Sets average parallelism as an exact rational value.
    pub fn set_average_parallelism(
        &mut self,
        numerator: u128,
        denominator: u128,
    ) -> ResultArtifact<()> {
        if denominator == 0 {
            return Err(
                SchedulingResultError::InconsistentResult {
                    message:
                        "average parallelism denominator cannot be zero"
                            .to_owned(),
                },
            );
        }

        self.average_parallelism_numerator = numerator;
        self.average_parallelism_denominator = denominator;

        Ok(())
    }
}

// =============================================================================
// Verification
// =============================================================================

/// Verification status of one scheduling invariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationStatus {
    /// The invariant was not evaluated.
    NotChecked,

    /// The invariant passed.
    Passed,

    /// The invariant produced a non-fatal warning.
    Warning,

    /// The invariant failed.
    Failed,
}

impl VerificationStatus {
    /// Returns whether the status is successful.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Passed)
    }

    /// Returns whether the status is fatal.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Failed)
    }
}

/// Complete verification summary for a scheduling result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerificationSummary {
    structural: VerificationStatus,
    dependencies: VerificationStatus,
    resources: VerificationStatus,
    timing: VerificationStatus,
    semantics: VerificationStatus,
    completeness: VerificationStatus,
}

impl Default for VerificationSummary {
    fn default() -> Self {
        Self {
            structural: VerificationStatus::NotChecked,
            dependencies: VerificationStatus::NotChecked,
            resources: VerificationStatus::NotChecked,
            timing: VerificationStatus::NotChecked,
            semantics: VerificationStatus::NotChecked,
            completeness: VerificationStatus::NotChecked,
        }
    }
}

impl VerificationSummary {
    /// Returns structural verification status.
    #[must_use]
    pub const fn structural(self) -> VerificationStatus {
        self.structural
    }

    /// Returns dependency verification status.
    #[must_use]
    pub const fn dependencies(self) -> VerificationStatus {
        self.dependencies
    }

    /// Returns resource verification status.
    #[must_use]
    pub const fn resources(self) -> VerificationStatus {
        self.resources
    }

    /// Returns timing verification status.
    #[must_use]
    pub const fn timing(self) -> VerificationStatus {
        self.timing
    }

    /// Returns semantic verification status.
    #[must_use]
    pub const fn semantics(self) -> VerificationStatus {
        self.semantics
    }

    /// Returns completeness verification status.
    #[must_use]
    pub const fn completeness(self) -> VerificationStatus {
        self.completeness
    }

    /// Sets structural status.
    pub fn set_structural(
        &mut self,
        status: VerificationStatus,
    ) {
        self.structural = status;
    }

    /// Sets dependency status.
    pub fn set_dependencies(
        &mut self,
        status: VerificationStatus,
    ) {
        self.dependencies = status;
    }

    /// Sets resource status.
    pub fn set_resources(
        &mut self,
        status: VerificationStatus,
    ) {
        self.resources = status;
    }

    /// Sets timing status.
    pub fn set_timing(
        &mut self,
        status: VerificationStatus,
    ) {
        self.timing = status;
    }

    /// Sets semantic status.
    pub fn set_semantics(
        &mut self,
        status: VerificationStatus,
    ) {
        self.semantics = status;
    }

    /// Sets completeness status.
    pub fn set_completeness(
        &mut self,
        status: VerificationStatus,
    ) {
        self.completeness = status;
    }

    /// Returns whether all verification dimensions passed.
    #[must_use]
    pub const fn is_fully_verified(self) -> bool {
        self.structural.is_success()
            && self.dependencies.is_success()
            && self.resources.is_success()
            && self.timing.is_success()
            && self.semantics.is_success()
            && self.completeness.is_success()
    }

    /// Returns whether any verification dimension failed.
    #[must_use]
    pub const fn has_failure(self) -> bool {
        self.structural.is_failure()
            || self.dependencies.is_failure()
            || self.resources.is_failure()
            || self.timing.is_failure()
            || self.semantics.is_failure()
            || self.completeness.is_failure()
    }

    /// Returns whether at least one warning exists.
    #[must_use]
    pub const fn has_warning(self) -> bool {
        matches!(
            self.structural,
            VerificationStatus::Warning
        ) || matches!(
            self.dependencies,
            VerificationStatus::Warning
        ) || matches!(
            self.resources,
            VerificationStatus::Warning
        ) || matches!(
            self.timing,
            VerificationStatus::Warning
        ) || matches!(
            self.semantics,
            VerificationStatus::Warning
        ) || matches!(
            self.completeness,
            VerificationStatus::Warning
        )
    }
}

// =============================================================================
// Diagnostics
// =============================================================================

/// Diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    /// Informational diagnostic.
    Info,

    /// Non-fatal warning.
    Warning,

    /// Error that prevents a verified result.
    Error,
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        };

        formatter.write_str(value)
    }
}

/// Structured scheduling diagnostic.
///
/// Diagnostics intentionally use optional canonical identities so that the
/// scheduler can explain both operation-specific and global scheduling issues.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingDiagnostic {
    severity: DiagnosticSeverity,
    code: String,
    message: String,
    operation: Option<OperationId>,
    resource: Option<ResourceId>,
    dependency: Option<DependencyId>,
}

impl SchedulingDiagnostic {
    /// Creates a diagnostic.
    pub fn new(
        severity: DiagnosticSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
            operation: None,
            resource: None,
            dependency: None,
        }
    }

    /// Returns severity.
    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    /// Returns stable diagnostic code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns associated operation.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        self.operation
    }

    /// Returns associated resource.
    #[must_use]
    pub const fn resource(&self) -> Option<ResourceId> {
        self.resource
    }

    /// Returns associated dependency.
    #[must_use]
    pub const fn dependency(&self) -> Option<DependencyId> {
        self.dependency
    }

    /// Associates an operation.
    pub fn with_operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Associates a resource.
    pub fn with_resource(
        mut self,
        resource: ResourceId,
    ) -> Self {
        self.resource = Some(resource);
        self
    }

    /// Associates a dependency.
    pub fn with_dependency(
        mut self,
        dependency: DependencyId,
    ) -> Self {
        self.dependency = Some(dependency);
        self
    }
}

// =============================================================================
// Provenance
// =============================================================================

/// Immutable provenance describing how a schedule result was produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultProvenance {
    ir_version: IrVersion,
    session_id: SchedulerSessionId,
    epoch_id: EpochId,
    schedule_id: ScheduleId,
    strategy: String,
    policy: String,
    objective: String,
    deterministic: bool,
    seed: Option<u64>,
}

impl ResultProvenance {
    /// Creates provenance metadata.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ir_version: IrVersion,
        session_id: SchedulerSessionId,
        epoch_id: EpochId,
        schedule_id: ScheduleId,
        strategy: impl Into<String>,
        policy: impl Into<String>,
        objective: impl Into<String>,
        deterministic: bool,
        seed: Option<u64>,
    ) -> Self {
        Self {
            ir_version,
            session_id,
            epoch_id,
            schedule_id,
            strategy: strategy.into(),
            policy: policy.into(),
            objective: objective.into(),
            deterministic,
            seed,
        }
    }

    /// Returns the IR version.
    #[must_use]
    pub const fn ir_version(&self) -> IrVersion {
        self.ir_version
    }

    /// Returns scheduler session identity.
    #[must_use]
    pub const fn session_id(&self) -> SchedulerSessionId {
        self.session_id
    }

    /// Returns scheduling epoch.
    #[must_use]
    pub const fn epoch_id(&self) -> EpochId {
        self.epoch_id
    }

    /// Returns schedule identity.
    #[must_use]
    pub const fn schedule_id(&self) -> ScheduleId {
        self.schedule_id
    }

    /// Returns selected strategy.
    #[must_use]
    pub fn strategy(&self) -> &str {
        &self.strategy
    }

    /// Returns selected policy.
    #[must_use]
    pub fn policy(&self) -> &str {
        &self.policy
    }

    /// Returns optimization objective.
    #[must_use]
    pub fn objective(&self) -> &str {
        &self.objective
    }

    /// Returns whether scheduling was deterministic.
    #[must_use]
    pub const fn deterministic(&self) -> bool {
        self.deterministic
    }

    /// Returns the explicit seed when one was used.
    #[must_use]
    pub const fn seed(&self) -> Option<u64> {
        self.seed
    }
}

// =============================================================================
// Scheduling result
// =============================================================================

/// Complete immutable scheduling artifact.
///
/// A `SchedulingResult` is the boundary between scheduling algorithms and
/// downstream consumers such as verification, benchmarking, hardware lowering,
/// QEC integration, diagnostics, and runtime preparation.
///
/// The result contains no vendor-specific execution object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingResult {
    status: ResultStatus,
    provenance: ResultProvenance,
    operations: Vec<ScheduledOperation>,
    reservations: Vec<ResultReservation>,
    resource_usage: BTreeMap<ResourceId, ResourceUsage>,
    metrics: ScheduleMetrics,
    verification: VerificationSummary,
    diagnostics: Vec<SchedulingDiagnostic>,
}

impl SchedulingResult {
    /// Creates a validated scheduling result.
    ///
    /// The constructor is intentionally private to this module's builder
    /// contract. Callers should use `SchedulingResultBuilder`.
    fn from_parts(
        status: ResultStatus,
        provenance: ResultProvenance,
        mut operations: Vec<ScheduledOperation>,
        mut reservations: Vec<ResultReservation>,
        mut resource_usage: BTreeMap<ResourceId, ResourceUsage>,
        metrics: ScheduleMetrics,
        verification: VerificationSummary,
        mut diagnostics: Vec<SchedulingDiagnostic>,
    ) -> ResultArtifact<Self> {
        operations.sort_by_key(ScheduledOperation::operation_id);
        reservations.sort_by_key(
            ResultReservation::reservation_id,
        );
        diagnostics.sort_by(|left, right| {
            left.severity()
                .cmp(&right.severity())
                .then_with(|| left.code().cmp(right.code()))
                .then_with(|| left.message().cmp(right.message()))
        });

        let mut seen_operations =
            BTreeSet::<OperationId>::new();

        for operation in &operations {
            if !seen_operations.insert(operation.operation_id()) {
                return Err(
                    SchedulingResultError::DuplicateOperation {
                        operation: operation.operation_id(),
                    },
                );
            }

            if operation.end() < operation.start() {
                return Err(
                    SchedulingResultError::InvalidInterval {
                        operation: operation.operation_id(),
                        start: operation.start(),
                        end: operation.end(),
                    },
                );
            }
        }

        let operation_ids = seen_operations;

        for operation in &operations {
            for dependency in operation.predecessors() {
                if !operation_ids.contains(dependency) {
                    return Err(
                        SchedulingResultError::MissingDependency {
                            operation: operation.operation_id(),
                            dependency: *dependency,
                        },
                    );
                }
            }
        }

        for reservation in &reservations {
            resource_usage
                .entry(reservation.resource_id())
                .or_insert_with(|| {
                    ResourceUsage::new(
                        reservation.resource_id(),
                    )
                })
                .record(*reservation)?;
        }

        for usage in resource_usage.values_mut() {
            usage.reservations.sort();
        }

        Ok(Self {
            status,
            provenance,
            operations,
            reservations,
            resource_usage,
            metrics,
            verification,
            diagnostics,
        })
    }

    /// Returns result status.
    #[must_use]
    pub const fn status(&self) -> ResultStatus {
        self.status
    }

    /// Returns provenance.
    #[must_use]
    pub const fn provenance(&self) -> &ResultProvenance {
        &self.provenance
    }

    /// Returns all scheduled operations in deterministic order.
    #[must_use]
    pub fn operations(&self) -> &[ScheduledOperation] {
        &self.operations
    }

    /// Returns all resource reservations in deterministic order.
    #[must_use]
    pub fn reservations(&self) -> &[ResultReservation] {
        &self.reservations
    }

    /// Returns all resource-usage summaries.
    #[must_use]
    pub fn resource_usage(
        &self,
    ) -> &BTreeMap<ResourceId, ResourceUsage> {
        &self.resource_usage
    }

    /// Returns aggregate metrics.
    #[must_use]
    pub const fn metrics(&self) -> &ScheduleMetrics {
        &self.metrics
    }

    /// Returns verification summary.
    #[must_use]
    pub const fn verification(
        &self,
    ) -> VerificationSummary {
        self.verification
    }

    /// Returns diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[SchedulingDiagnostic] {
        &self.diagnostics
    }

    /// Returns an operation by canonical identity.
    #[must_use]
    pub fn operation(
        &self,
        operation_id: OperationId,
    ) -> Option<&ScheduledOperation> {
        self.operations
            .binary_search_by_key(
                &operation_id,
                ScheduledOperation::operation_id,
            )
            .ok()
            .map(|index| &self.operations[index])
    }

    /// Returns all operations touching a logical qubit.
    #[must_use]
    pub fn operations_for_logical_qubit(
        &self,
        qubit: QubitId,
    ) -> Vec<&ScheduledOperation> {
        self.operations
            .iter()
            .filter(|operation| {
                operation.logical_qubits().contains(&qubit)
            })
            .collect()
    }

    /// Returns all operations touching a physical qubit.
    #[must_use]
    pub fn operations_for_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> Vec<&ScheduledOperation> {
        self.operations
            .iter()
            .filter(|operation| {
                operation
                    .physical_qubits()
                    .contains(&qubit)
            })
            .collect()
    }

    /// Returns whether the result is verified.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.status.is_verified()
    }

    /// Returns whether downstream compilation may consume the result as
    /// verified.
    #[must_use]
    pub const fn is_usable(&self) -> bool {
        self.status.is_usable()
    }

    /// Returns whether the result contains any errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.verification.has_failure()
            || self
                .diagnostics
                .iter()
                .any(|diagnostic| {
                    diagnostic.severity()
                        == DiagnosticSeverity::Error
                })
    }

    /// Returns whether warnings are present.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.verification.has_warning()
            || self
                .diagnostics
                .iter()
                .any(|diagnostic| {
                    diagnostic.severity()
                        == DiagnosticSeverity::Warning
                })
    }

    /// Returns the schedule's final time coordinate.
    #[must_use]
    pub fn end_time(&self) -> TimePoint {
        self.operations
            .iter()
            .map(ScheduledOperation::end)
            .max()
            .unwrap_or(TimePoint::ZERO)
    }

    /// Returns the number of logical qubits represented by the result.
    ///
    /// This is a measurement of the result, not a machine-size limit.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        self.operations
            .iter()
            .flat_map(ScheduledOperation::logical_qubits)
            .collect::<BTreeSet<_>>()
            .len()
    }

    /// Returns the number of physical qubits represented by the result.
    ///
    /// This is a measurement of the result, not a machine-size limit.
    #[must_use]
    pub fn physical_qubit_count(&self) -> usize {
        self.operations
            .iter()
            .flat_map(ScheduledOperation::physical_qubits)
            .collect::<BTreeSet<_>>()
            .len()
    }
}

// =============================================================================
// Result builder
// =============================================================================

/// Builder for immutable scheduling results.
///
/// The builder is deliberately separate from scheduling algorithms. Any
/// scheduler strategy can construct a result using the same contract.
#[derive(Debug, Default)]
pub struct SchedulingResultBuilder {
    provenance: Option<ResultProvenance>,
    status: ResultStatus,
    operations: BTreeMap<OperationId, ScheduledOperation>,
    reservations: BTreeMap<ReservationId, ResultReservation>,
    verification: VerificationSummary,
    diagnostics: Vec<SchedulingDiagnostic>,
    metrics: ScheduleMetrics,
}

impl SchedulingResultBuilder {
    /// Creates an empty result builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            provenance: None,
            status: ResultStatus::Building,
            operations: BTreeMap::new(),
            reservations: BTreeMap::new(),
            verification: VerificationSummary::default(),
            diagnostics: Vec::new(),
            metrics: ScheduleMetrics::default(),
        }
    }

    /// Sets provenance.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: ResultProvenance,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Sets the result status.
    pub fn set_status(
        &mut self,
        status: ResultStatus,
    ) -> ResultArtifact<()> {
        if self.status == ResultStatus::Failed
            && status != ResultStatus::Failed
        {
            return Err(
                SchedulingResultError::InvalidStatusTransition {
                    current: self.status,
                    requested: status,
                },
            );
        }

        self.status = status;
        Ok(())
    }

    /// Adds a scheduled operation.
    pub fn add_operation(
        &mut self,
        operation: ScheduledOperation,
    ) -> ResultArtifact<()> {
        let operation_id = operation.operation_id();

        if self
            .operations
            .insert(operation_id, operation)
            .is_some()
        {
            return Err(
                SchedulingResultError::DuplicateOperation {
                    operation: operation_id,
                },
            );
        }

        Ok(())
    }

    /// Adds a resource reservation.
    pub fn add_reservation(
        &mut self,
        reservation: ResultReservation,
    ) -> ResultArtifact<()> {
        let reservation_id =
            reservation.reservation_id();

        if self
            .reservations
            .insert(reservation_id, reservation)
            .is_some()
        {
            return Err(
                SchedulingResultError::InconsistentResult {
                    message: format!(
                        "duplicate reservation `{reservation_id}`"
                    ),
                },
            );
        }

        Ok(())
    }

    /// Adds a diagnostic.
    pub fn add_diagnostic(
        &mut self,
        diagnostic: SchedulingDiagnostic,
    ) {
        self.diagnostics.push(diagnostic);
    }

    /// Sets verification summary.
    pub fn set_verification(
        &mut self,
        verification: VerificationSummary,
    ) {
        self.verification = verification;
    }

    /// Sets aggregate metrics.
    pub fn set_metrics(
        &mut self,
        metrics: ScheduleMetrics,
    ) {
        self.metrics = metrics;
    }

    /// Marks the result as planned.
    pub fn mark_planned(
        &mut self,
    ) -> ResultArtifact<()> {
        self.set_status(ResultStatus::Planned)
    }

    /// Marks the result as verified.
    pub fn mark_verified(
        &mut self,
    ) -> ResultArtifact<()> {
        if !self.verification.is_fully_verified() {
            return Err(
                SchedulingResultError::InconsistentResult {
                    message:
                        "cannot mark a result verified before all verification dimensions pass"
                            .to_owned(),
                },
            );
        }

        self.set_status(ResultStatus::Verified)
    }

    /// Marks the result as verified with warnings.
    pub fn mark_verified_with_warnings(
        &mut self,
    ) -> ResultArtifact<()> {
        if self.verification.has_failure() {
            return Err(
                SchedulingResultError::InconsistentResult {
                    message:
                        "cannot mark a failed result verified with warnings"
                            .to_owned(),
                },
            );
        }

        self.set_status(
            ResultStatus::VerifiedWithWarnings,
        )
    }

    /// Marks the result as analysis-only.
    pub fn mark_analysis_only(
        &mut self,
    ) -> ResultArtifact<()> {
        self.set_status(ResultStatus::AnalysisOnly)
    }

    /// Marks the result as failed.
    pub fn mark_failed(&mut self) {
        self.status = ResultStatus::Failed;
    }

    /// Finalizes the immutable result.
    pub fn build(self) -> ResultArtifact<SchedulingResult> {
        let provenance =
            self.provenance.ok_or(
                SchedulingResultError::MissingRequiredField {
                    field: "provenance",
                },
            )?;

        let operations =
            self.operations.into_values().collect::<Vec<_>>();

        let reservations =
            self.reservations.into_values().collect::<Vec<_>>();

        let resource_usage =
            build_resource_usage(&reservations)?;

        let metrics = finalize_metrics(
            self.metrics,
            &operations,
            &reservations,
        )?;

        let mut result = SchedulingResult::from_parts(
            self.status,
            provenance,
            operations,
            reservations,
            resource_usage,
            metrics,
            self.verification,
            self.diagnostics,
        )?;

        if result.status == ResultStatus::Verified
            && !result.verification.is_fully_verified()
        {
            result.status =
                ResultStatus::Planned;
        }

        Ok(result)
    }
}

// =============================================================================
// Metric finalization
// =============================================================================

fn build_resource_usage(
    reservations: &[ResultReservation],
) -> ResultArtifact<BTreeMap<ResourceId, ResourceUsage>> {
    let mut usage =
        BTreeMap::<ResourceId, ResourceUsage>::new();

    let mut reservations_by_resource =
        BTreeMap::<ResourceId, Vec<ResultReservation>>::new();

    for reservation in reservations {
        reservations_by_resource
            .entry(reservation.resource_id())
            .or_default()
            .push(*reservation);
    }

    for (resource_id, mut resource_reservations) in
        reservations_by_resource
    {
        resource_reservations.sort_by(|left, right| {
            left.interval()
                .start()
                .cmp(&right.interval().start())
                .then_with(|| {
                    left.interval()
                        .end()
                        .cmp(&right.interval().end())
                })
                .then_with(|| {
                    left.operation_id()
                        .cmp(&right.operation_id())
                })
        });

        let mut resource_usage =
            ResourceUsage::new(resource_id);

        let mut previous: Option<ResultReservation> =
            None;

        for reservation in resource_reservations {
            if let Some(previous_reservation) = previous {
                if previous_reservation
                    .interval()
                    .overlaps(reservation.interval())
                {
                    return Err(
                        SchedulingResultError::ResourceConflict {
                            resource: resource_id,
                            first_operation:
                                previous_reservation
                                    .operation_id(),
                            second_operation:
                                reservation
                                    .operation_id(),
                        },
                    );
                }
            }

            resource_usage.record(reservation)?;
            previous = Some(reservation);
        }

        usage.insert(resource_id, resource_usage);
    }

    Ok(usage)
}

fn finalize_metrics(
    mut metrics: ScheduleMetrics,
    operations: &[ScheduledOperation],
    reservations: &[ResultReservation],
) -> ResultArtifact<ScheduleMetrics> {
    let operation_count =
        operations.len() as u128;

    let dependency_count =
        operations
            .iter()
            .map(|operation| {
                operation.predecessors().len() as u128
            })
            .try_fold(0_u128, |accumulator, value| {
                accumulator.checked_add(value)
            })
            .ok_or(
                SchedulingResultError::InconsistentResult {
                    message:
                        "dependency count overflowed"
                            .to_owned(),
                },
            )?;

    let resource_count =
        reservations
            .iter()
            .map(ResultReservation::resource_id)
            .collect::<BTreeSet<_>>()
            .len() as u128;

    let reservation_count =
        reservations.len() as u128;

    let makespan =
        operations
            .iter()
            .map(ScheduledOperation::end)
            .max()
            .unwrap_or(TimePoint::ZERO)
            .checked_duration_until(TimePoint::ZERO)
            .map_or(Duration::ZERO, |duration| {
                duration
            });

    //
    // `TimePoint::checked_duration_until` is directional, so compute makespan
    // directly from the origin to the final time.
    //
    let final_time =
        operations
            .iter()
            .map(ScheduledOperation::end)
            .max()
            .unwrap_or(TimePoint::ZERO);

    let actual_makespan =
        TimePoint::ZERO
            .checked_duration_until(final_time)
            .ok_or(
                SchedulingResultError::InconsistentResult {
                    message:
                        "schedule makespan could not be calculated"
                            .to_owned(),
                },
            )?;

    let _ = makespan;

    metrics.operation_count = operation_count;
    metrics.dependency_count = dependency_count;
    metrics.resource_count = resource_count;
    metrics.reservation_count = reservation_count;
    metrics.makespan = actual_makespan;

    Ok(metrics)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn resource(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    fn reservation(value: u64) -> ReservationId {
        ReservationId::new(value)
    }

    fn dependency(value: u64) -> DependencyId {
        DependencyId::new(value)
    }

    fn provenance() -> ResultProvenance {
        ResultProvenance::new(
            IrVersion::CURRENT,
            SchedulerSessionId::new(1),
            EpochId::new(1),
            ScheduleId::new(1),
            "list",
            "resource-aware",
            "makespan",
            true,
            Some(1),
        )
    }

    #[test]
    fn interval_accepts_half_open_range() {
        let interval = ScheduledInterval::new(
            TimePoint::new(10),
            TimePoint::new(20),
        )
        .expect("valid interval");

        assert_eq!(
            interval.start(),
            TimePoint::new(10)
        );

        assert_eq!(
            interval.end(),
            TimePoint::new(20)
        );

        assert_eq!(
            interval.duration(),
            Some(Duration::new(10))
        );
    }

    #[test]
    fn interval_rejects_reverse_range() {
        let result = ScheduledInterval::new(
            TimePoint::new(20),
            TimePoint::new(10),
        );

        assert!(result.is_err());
    }

    #[test]
    fn touching_intervals_do_not_overlap() {
        let left =
            ScheduledInterval::new(
                TimePoint::new(0),
                TimePoint::new(10),
            )
            .expect("valid");

        let right =
            ScheduledInterval::new(
                TimePoint::new(10),
                TimePoint::new(20),
            )
            .expect("valid");

        assert!(!left.overlaps(right));
        assert!(left.touches(right));
    }

    #[test]
    fn overlapping_intervals_are_detected() {
        let left =
            ScheduledInterval::new(
                TimePoint::new(0),
                TimePoint::new(10),
            )
            .expect("valid");

        let right =
            ScheduledInterval::new(
                TimePoint::new(9),
                TimePoint::new(20),
            )
            .expect("valid");

        assert!(left.overlaps(right));
    }

    #[test]
    fn duplicate_operations_are_rejected() {
        let mut builder =
            SchedulingResultBuilder::new()
                .with_provenance(provenance());

        let first =
            ScheduledOperation::new(
                operation(1),
                TimePoint::ZERO,
                TimePoint::new(10),
            )
            .expect("valid");

        let second =
            ScheduledOperation::new(
                operation(1),
                TimePoint::new(10),
                TimePoint::new(20),
            )
            .expect("valid");

        builder
            .add_operation(first)
            .expect("first insertion");

        assert!(
            builder.add_operation(second).is_err()
        );
    }

    #[test]
    fn dependency_is_preserved() {
        let mut scheduled =
            ScheduledOperation::new(
                operation(2),
                TimePoint::new(10),
                TimePoint::new(20),
            )
            .expect("valid");

        scheduled.add_predecessor(operation(1));

        assert!(
            scheduled
                .predecessors()
                .contains(&operation(1))
        );
    }

    #[test]
    fn resource_conflict_is_rejected() {
        let mut builder =
            SchedulingResultBuilder::new()
                .with_provenance(provenance());

        let first =
            ScheduledOperation::new(
                operation(1),
                TimePoint::ZERO,
                TimePoint::new(10),
            )
            .expect("valid");

        let second =
            ScheduledOperation::new(
                operation(2),
                TimePoint::new(5),
                TimePoint::new(15),
            )
            .expect("valid");

        builder
            .add_operation(first)
            .expect("first");

        builder
            .add_operation(second)
            .expect("second");

        builder
            .add_reservation(
                ResultReservation::new(
                    reservation(1),
                    resource(1),
                    operation(1),
                    TimePoint::ZERO,
                    TimePoint::new(10),
                )
                .expect("reservation"),
            )
            .expect("reservation 1");

        builder
            .add_reservation(
                ResultReservation::new(
                    reservation(2),
                    resource(1),
                    operation(2),
                    TimePoint::new(5),
                    TimePoint::new(15),
                )
                .expect("reservation"),
            )
            .expect("reservation 2");

        assert!(builder.build().is_err());
    }

    #[test]
    fn resource_usage_is_aggregated() {
        let mut builder =
            SchedulingResultBuilder::new()
                .with_provenance(provenance());

        builder
            .add_operation(
                ScheduledOperation::new(
                    operation(1),
                    TimePoint::ZERO,
                    TimePoint::new(10),
                )
                .expect("operation"),
            )
            .expect("operation");

        builder
            .add_reservation(
                ResultReservation::new(
                    reservation(1),
                    resource(1),
                    operation(1),
                    TimePoint::ZERO,
                    TimePoint::new(10),
                )
                .expect("reservation"),
            )
            .expect("reservation");

        let result =
            builder.build().expect("result");

        let usage =
            result
                .resource_usage()
                .get(&resource(1))
                .expect("resource usage");

        assert_eq!(
            usage.operations().len(),
            1
        );

        assert_eq!(
            usage.busy_duration(),
            Duration::new(10)
        );
    }

    #[test]
    fn logical_qubit_identity_is_canonical() {
        let mut scheduled =
            ScheduledOperation::new(
                operation(1),
                TimePoint::ZERO,
                TimePoint::new(10),
            )
            .expect("operation");

        let qubit = QubitId::new(7);

        scheduled.add_logical_qubit(qubit);

        assert_eq!(
            scheduled.logical_qubits(),
            &[qubit]
        );
    }

    #[test]
    fn result_can_be_marked_verified() {
        let mut builder =
            SchedulingResultBuilder::new()
                .with_provenance(provenance());

        let mut verification =
            VerificationSummary::default();

        verification.set_structural(
            VerificationStatus::Passed,
        );

        verification.set_dependencies(
            VerificationStatus::Passed,
        );

        verification.set_resources(
            VerificationStatus::Passed,
        );

        verification.set_timing(
            VerificationStatus::Passed,
        );

        verification.set_semantics(
            VerificationStatus::Passed,
        );

        verification.set_completeness(
            VerificationStatus::Passed,
        );

        builder.set_verification(
            verification,
        );

        builder
            .mark_verified()
            .expect("verification should succeed");

        let result =
            builder.build().expect("result");

        assert!(result.is_verified());
        assert!(result.is_usable());
    }

    #[test]
    fn diagnostics_are_retained() {
        let mut builder =
            SchedulingResultBuilder::new()
                .with_provenance(provenance());

        builder.add_diagnostic(
            SchedulingDiagnostic::new(
                DiagnosticSeverity::Warning,
                "SCHED001",
                "operation delayed by resource contention",
            )
            .with_operation(operation(1)),
        );

        let result =
            builder.build().expect("result");

        assert_eq!(
            result.diagnostics().len(),
            1
        );

        assert!(result.has_warnings());
    }

    #[test]
    fn status_reports_verified_with_warnings() {
        let status =
            ResultStatus::VerifiedWithWarnings;

        assert!(status.has_schedule());
        assert!(status.is_verified());
        assert!(status.is_usable());
        assert!(!status.is_failed());
    }

    #[test]
    fn dependency_identity_remains_distinct_from_operation_identity() {
        let dependency_id = dependency(42);

        let diagnostic =
            SchedulingDiagnostic::new(
                DiagnosticSeverity::Info,
                "SCHED002",
                "dependency observed",
            )
            .with_dependency(dependency_id);

        assert_eq!(
            diagnostic.dependency(),
            Some(dependency_id)
        );
    }
}