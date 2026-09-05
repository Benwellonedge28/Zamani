//! Zamani Quantum Scheduling — Production Profiling
//!
//! This module provides scheduler instrumentation and performance profiling
//! without becoming coupled to any particular scheduling algorithm, hardware
//! provider, routing implementation, QEC implementation, or runtime.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "How much work did scheduling perform, where did the time go, how large
//! > was the scheduling problem, and what scheduling behaviour occurred?"
//!
//! It owns:
//!
//! - host-side profiling measurements;
//! - scheduler phase measurements;
//! - operation/dependency/resource counters;
//! - scheduling decision counters;
//! - conflict and constraint counters;
//! - transformation counters;
//! - verification counters;
//! - dynamic/distributed/QEC profiling counters;
//! - queue/parallelism observations;
//! - optional unique-qubit observation;
//! - peak and aggregate measurements;
//! - profile snapshots;
//! - profile merging;
//! - deterministic profile serialization-friendly values;
//! - lightweight RAII phase timing.
//!
//! It does NOT own:
//!
//! - scheduling algorithms;
//! - scheduling policy;
//! - quantum operation semantics;
//! - routing;
//! - hardware discovery;
//! - hardware execution;
//! - QEC algorithms;
//! - diagnostics trace events;
//! - schedule verification logic;
//! - optimization logic;
//! - serialization format definitions.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! scheduling
//!      │
//!      ├───────────────┐
//!      │               │
//!      ▼               ▼
//! diagnostics::trace   diagnostics::profile
//!                      │
//!                      ▼
//!                 ProfileSnapshot
//!                      │
//!              ┌───────┼────────┐
//!              ▼       ▼        ▼
//!          diagnostics benchmark tooling
//!          explain      analysis
//! ```
//!
//! Profiling is observational. It must never alter scheduling semantics.
//!
//! # Important distinction: host time vs schedule time
//!
//! There are two different clocks in the scheduling system.
//!
//! ## Scheduler time
//!
//! `quantum::scheduling::types::TimePoint` represents the abstract temporal
//! coordinate of the quantum schedule.
//!
//! ## Profiling time
//!
//! `std::time::Instant` measures how long the compiler/scheduler itself took
//! to perform work on the host.
//!
//! These must never be confused.
//!
//! For example:
//!
//! ```text
//! schedule makespan = 400 abstract timing units
//!
//! host planning time = 2.7 ms
//! ```
//!
//! They measure completely different things.
//!
//! # Scalability
//!
//! This module deliberately contains no constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_EVENTS
//! MAX_PROFILE_ENTRIES
//! ```
//!
//! Aggregate counters use `u128` so profiling does not introduce a small
//! machine-size-oriented ceiling into the scheduling architecture.
//!
//! The profile does NOT retain every operation by default.
//!
//! This is intentional.
//!
//! A billion-operation schedule should not require a billion profiling
//! records merely because profiling is enabled.
//!
//! Instead, the default profile contains aggregate statistics.
//!
//! Optional unique-qubit tracking can retain identities when a caller explicitly
//! requests that information.
//!
//! # Concurrency model
//!
//! `Profile` is intentionally an owned mutable accumulator. It contains no
//! global state and no interior mutability.
//!
//! For concurrent scheduling:
//!
//! ```text
//! scheduler worker A ──► Profile A ──┐
//! scheduler worker B ──► Profile B ──┤
//! scheduler worker C ──► Profile C ──┤
//!                                      ▼
//!                                Profile::merge
//!                                      │
//!                                      ▼
//!                              final ProfileSnapshot
//! ```
//!
//! This avoids putting a global mutex on every scheduling decision.
//!
//! A profile can therefore scale with the scheduler rather than becoming a
//! synchronization bottleneck.
//!
//! # Determinism
//!
//! Aggregate counters are deterministic when the scheduling execution itself
//! is deterministic.
//!
//! Host elapsed-time measurements are inherently environment-dependent and
//! therefore MUST NOT participate in schedule identity, semantic equality, or
//! reproducibility decisions.
//!
//! `ProfileSnapshot::semantic_fingerprint_values()` therefore exposes only
//! deterministic profile dimensions.
//!
//! # Overflow
//!
//! Profiling counters use checked arithmetic at the public operation level.
//!
//! If a counter would exceed `u128::MAX`, the operation returns
//! `ProfileError::CounterOverflow` rather than silently wrapping.
//!
//! This is preferable to wrapping because profiling must never report a false
//! value without notifying the caller.
//!
//! # Memory behaviour
//!
//! Aggregate profiling has constant memory with respect to the number of
//! operations, dependencies, resources, and scheduling decisions.
//!
//! Optional unique-qubit tracking requires storage proportional to the number
//! of distinct qubits observed by that profile.
//!
//! No time-slot matrix is allocated.
//!
//! No operation-sized event log is allocated.
//!
//! No machine-size-dependent array is allocated.
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
//! - no unsafe code.
//!
//! # Integration contract
//!
//! The intended consumers are:
//!
//! ```text
//! scheduling::planners
//! scheduling::algorithms
//! scheduling::verification
//! scheduling::transformations
//! scheduling::optimization
//! scheduling::dynamic
//! scheduling::distributed
//! scheduling::qec
//! ```
//!
//! They may record measurements through `Profile`.
//!
//! `diagnostics::trace` may coexist with this module but must not be required
//! for profiling.
//!
//! The benchmarking subsystem may consume `ProfileSnapshot` without depending
//! on scheduler implementation details.
//!
//! # Integration with SchedulingConfig
//!
//! `SchedulingConfig` already defines:
//!
//! ```text
//! DiagnosticPolicy::DetailedAndProfiled
//! ```
//!
//! The scheduler composition layer should enable profiling when that policy is
//! selected. This module intentionally does not import `SchedulingConfig` so
//! that it remains foundational and avoids a dependency cycle.
//!
//! # Integration with canonical qubit identity
//!
//! When qubit identity is required, this module uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! It never defines another `QubitId`.
//!
//! # Integration with scheduling types
//!
//! Scheduler timing values use:
//!
//! ```text
//! crate::quantum::scheduling::types::{Duration, TimePoint}
//! ```
//!
//! Profiling host elapsed time uses `std::time::Duration` internally and is
//! represented separately in the snapshot.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::time::{Duration as HostDuration, Instant};

use crate::quantum::ir::qubit::QubitId;

use super::types::{Duration, OperationId, ResourceId, TimePoint};

// =============================================================================
// Result type
// =============================================================================

/// Result type used by the profiling subsystem.
pub type ProfileResult<T> = Result<T, ProfileError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the profiling subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileError {
    /// A profiling counter would overflow its representable range.
    CounterOverflow {
        /// Logical name of the counter.
        counter: &'static str,
    },

    /// A phase was ended without a corresponding active phase.
    InvalidPhase,

    /// A phase was already closed.
    PhaseAlreadyClosed,

    /// A profile merge would produce an overflow.
    MergeOverflow {
        /// Logical name of the overflowing field.
        field: &'static str,
    },

    /// A supplied operation identifier was invalid for the requested
    /// operation.
    InvalidOperation {
        /// Operation involved.
        operation: OperationId,

        /// Explanation.
        message: &'static str,
    },

    /// A supplied resource identifier was invalid for the requested
    /// operation.
    InvalidResource {
        /// Resource involved.
        resource: ResourceId,

        /// Explanation.
        message: &'static str,
    },
}

impl fmt::Display for ProfileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CounterOverflow { counter } => {
                write!(formatter, "profiling counter `{counter}` overflowed")
            }

            Self::InvalidPhase => {
                formatter.write_str("profiling phase is invalid")
            }

            Self::PhaseAlreadyClosed => {
                formatter.write_str("profiling phase has already been closed")
            }

            Self::MergeOverflow { field } => {
                write!(formatter, "profile merge overflowed field `{field}`")
            }

            Self::InvalidOperation {
                operation,
                message,
            } => {
                write!(
                    formatter,
                    "invalid operation `{operation}` for profiling: {message}"
                )
            }

            Self::InvalidResource {
                resource,
                message,
            } => {
                write!(
                    formatter,
                    "invalid resource `{resource}` for profiling: {message}"
                )
            }
        }
    }
}

impl Error for ProfileError {}

// =============================================================================
// Profiling phase
// =============================================================================

/// Major scheduler phase.
///
/// This enum is intentionally descriptive rather than algorithm-specific.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProfilePhase {
    /// Input normalization and preparation.
    Preparation,

    /// Dependency graph construction.
    DependencyAnalysis,

    /// Resource analysis.
    ResourceAnalysis,

    /// Timing analysis.
    TimingAnalysis,

    /// Constraint analysis.
    ConstraintAnalysis,

    /// Scheduling/planning.
    Planning,

    /// Target-aware resource placement.
    ResourceScheduling,

    /// Temporal alignment.
    Alignment,

    /// Schedule transformations.
    Transformation,

    /// Verification.
    Verification,

    /// Scheduling optimization.
    Optimization,

    /// Dynamic scheduling preparation.
    DynamicScheduling,

    /// Distributed scheduling.
    DistributedScheduling,

    /// QEC-aware scheduling.
    QecScheduling,

    /// Result construction.
    ResultConstruction,

    /// Serialization preparation.
    Serialization,

    /// Other caller-defined profiling work.
    Other,
}

impl fmt::Display for ProfilePhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Preparation => "preparation",
            Self::DependencyAnalysis => "dependency-analysis",
            Self::ResourceAnalysis => "resource-analysis",
            Self::TimingAnalysis => "timing-analysis",
            Self::ConstraintAnalysis => "constraint-analysis",
            Self::Planning => "planning",
            Self::ResourceScheduling => "resource-scheduling",
            Self::Alignment => "alignment",
            Self::Transformation => "transformation",
            Self::Verification => "verification",
            Self::Optimization => "optimization",
            Self::DynamicScheduling => "dynamic-scheduling",
            Self::DistributedScheduling => "distributed-scheduling",
            Self::QecScheduling => "qec-scheduling",
            Self::ResultConstruction => "result-construction",
            Self::Serialization => "serialization",
            Self::Other => "other",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Counter set
// =============================================================================

/// Aggregate scheduler counters.
///
/// All counters are aggregate values rather than per-operation records.
///
/// This is the primary mechanism that allows profiling to remain memory
/// efficient for extremely large schedules.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProfileCounters {
    operations_seen: u128,
    operations_scheduled: u128,
    operations_delayed: u128,
    operations_completed: u128,
    operations_rejected: u128,
    operations_rescheduled: u128,

    dependency_edges: u128,
    dependency_waits: u128,
    dependency_violations: u128,

    resources_seen: u128,
    resource_reservations: u128,
    resource_releases: u128,
    resource_waits: u128,
    resource_conflicts: u128,
    capacity_conflicts: u128,

    constraints_evaluated: u128,
    constraints_satisfied: u128,
    constraints_rejected: u128,
    constraint_conflicts: u128,

    scheduling_decisions: u128,
    scheduling_iterations: u128,
    backtracks: u128,

    ready_queue_insertions: u128,
    ready_queue_removals: u128,
    peak_ready_queue: u128,

    alignment_adjustments: u128,
    inserted_delays: u128,
    inserted_padding: u128,
    transformations: u128,

    verification_checks: u128,
    verification_failures: u128,

    optimization_iterations: u128,
    objective_evaluations: u128,

    dynamic_events: u128,
    conditional_operations: u128,
    feedback_waits: u128,

    communication_operations: u128,
    communication_waits: u128,
    communication_conflicts: u128,

    qec_rounds: u128,
    syndrome_operations: u128,

    planning_failures: u128,
    cancellations: u128,
}

impl ProfileCounters {
    /// Creates an empty counter set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations_seen: 0,
            operations_scheduled: 0,
            operations_delayed: 0,
            operations_completed: 0,
            operations_rejected: 0,
            operations_rescheduled: 0,
            dependency_edges: 0,
            dependency_waits: 0,
            dependency_violations: 0,
            resources_seen: 0,
            resource_reservations: 0,
            resource_releases: 0,
            resource_waits: 0,
            resource_conflicts: 0,
            capacity_conflicts: 0,
            constraints_evaluated: 0,
            constraints_satisfied: 0,
            constraints_rejected: 0,
            constraint_conflicts: 0,
            scheduling_decisions: 0,
            scheduling_iterations: 0,
            backtracks: 0,
            ready_queue_insertions: 0,
            ready_queue_removals: 0,
            peak_ready_queue: 0,
            alignment_adjustments: 0,
            inserted_delays: 0,
            inserted_padding: 0,
            transformations: 0,
            verification_checks: 0,
            verification_failures: 0,
            optimization_iterations: 0,
            objective_evaluations: 0,
            dynamic_events: 0,
            conditional_operations: 0,
            feedback_waits: 0,
            communication_operations: 0,
            communication_waits: 0,
            communication_conflicts: 0,
            qec_rounds: 0,
            syndrome_operations: 0,
            planning_failures: 0,
            cancellations: 0,
        }
    }

    fn increment(
        value: &mut u128,
        counter: &'static str,
    ) -> ProfileResult<()> {
        *value = value
            .checked_add(1)
            .ok_or(ProfileError::CounterOverflow { counter })?;

        Ok(())
    }

    fn add(
        value: &mut u128,
        amount: u128,
        counter: &'static str,
    ) -> ProfileResult<()> {
        *value = value
            .checked_add(amount)
            .ok_or(ProfileError::CounterOverflow { counter })?;

        Ok(())
    }

    /// Records an operation observed by the scheduler.
    pub fn record_operation_seen(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.operations_seen, "operations_seen")
    }

    /// Records a successfully scheduled operation.
    pub fn record_operation_scheduled(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.operations_scheduled,
            "operations_scheduled",
        )
    }

    /// Records an operation that had to be delayed.
    pub fn record_operation_delayed(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.operations_delayed, "operations_delayed")
    }

    /// Records an operation that completed planning.
    pub fn record_operation_completed(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.operations_completed,
            "operations_completed",
        )
    }

    /// Records an operation rejected by scheduling.
    pub fn record_operation_rejected(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.operations_rejected,
            "operations_rejected",
        )
    }

    /// Records an operation scheduled more than once due to rescheduling.
    pub fn record_operation_rescheduled(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.operations_rescheduled,
            "operations_rescheduled",
        )
    }

    /// Adds dependency edges.
    pub fn add_dependency_edges(&mut self, count: u128) -> ProfileResult<()> {
        Self::add(
            &mut self.dependency_edges,
            count,
            "dependency_edges",
        )
    }

    /// Records a dependency-induced wait.
    pub fn record_dependency_wait(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.dependency_waits, "dependency_waits")
    }

    /// Records a dependency violation detected during analysis/verification.
    pub fn record_dependency_violation(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.dependency_violations,
            "dependency_violations",
        )
    }

    /// Records a resource observed by the scheduler.
    pub fn record_resource_seen(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.resources_seen, "resources_seen")
    }

    /// Records a resource reservation.
    pub fn record_resource_reservation(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.resource_reservations,
            "resource_reservations",
        )
    }

    /// Records a resource release.
    pub fn record_resource_release(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.resource_releases, "resource_releases")
    }

    /// Records a wait caused by resource availability.
    pub fn record_resource_wait(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.resource_waits, "resource_waits")
    }

    /// Records a resource conflict.
    pub fn record_resource_conflict(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.resource_conflicts,
            "resource_conflicts",
        )
    }

    /// Records a resource capacity conflict.
    pub fn record_capacity_conflict(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.capacity_conflicts,
            "capacity_conflicts",
        )
    }

    /// Records a constraint evaluation.
    pub fn record_constraint_evaluated(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.constraints_evaluated,
            "constraints_evaluated",
        )
    }

    /// Records a satisfied constraint.
    pub fn record_constraint_satisfied(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.constraints_satisfied,
            "constraints_satisfied",
        )
    }

    /// Records a rejected constraint.
    pub fn record_constraint_rejected(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.constraints_rejected,
            "constraints_rejected",
        )
    }

    /// Records a conflicting constraint.
    pub fn record_constraint_conflict(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.constraint_conflicts,
            "constraint_conflicts",
        )
    }

    /// Records a scheduling decision.
    pub fn record_scheduling_decision(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.scheduling_decisions,
            "scheduling_decisions",
        )
    }

    /// Records a scheduler iteration.
    pub fn record_scheduling_iteration(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.scheduling_iterations,
            "scheduling_iterations",
        )
    }

    /// Records a scheduler backtrack.
    pub fn record_backtrack(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.backtracks, "backtracks")
    }

    /// Records insertion into the ready queue.
    pub fn record_ready_queue_insertion(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.ready_queue_insertions,
            "ready_queue_insertions",
        )
    }

    /// Records removal from the ready queue.
    pub fn record_ready_queue_removal(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.ready_queue_removals,
            "ready_queue_removals",
        )
    }

    /// Records an observed ready-queue size.
    pub fn observe_ready_queue_size(
        &mut self,
        size: u128,
    ) -> ProfileResult<()> {
        if size > self.peak_ready_queue {
            self.peak_ready_queue = size;
        }

        Ok(())
    }

    /// Records a timing alignment adjustment.
    pub fn record_alignment_adjustment(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.alignment_adjustments,
            "alignment_adjustments",
        )
    }

    /// Records an inserted delay.
    pub fn record_inserted_delay(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.inserted_delays, "inserted_delays")
    }

    /// Records inserted padding.
    pub fn record_inserted_padding(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.inserted_padding, "inserted_padding")
    }

    /// Records a schedule transformation.
    pub fn record_transformation(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.transformations, "transformations")
    }

    /// Records a verification check.
    pub fn record_verification_check(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.verification_checks,
            "verification_checks",
        )
    }

    /// Records a verification failure.
    pub fn record_verification_failure(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.verification_failures,
            "verification_failures",
        )
    }

    /// Records an optimization iteration.
    pub fn record_optimization_iteration(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.optimization_iterations,
            "optimization_iterations",
        )
    }

    /// Records an objective evaluation.
    pub fn record_objective_evaluation(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.objective_evaluations,
            "objective_evaluations",
        )
    }

    /// Records a runtime/dynamic scheduling event.
    pub fn record_dynamic_event(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.dynamic_events, "dynamic_events")
    }

    /// Records a conditional operation.
    pub fn record_conditional_operation(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.conditional_operations,
            "conditional_operations",
        )
    }

    /// Records a classical-feedback wait.
    pub fn record_feedback_wait(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.feedback_waits, "feedback_waits")
    }

    /// Records a communication operation.
    pub fn record_communication_operation(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.communication_operations,
            "communication_operations",
        )
    }

    /// Records communication-induced waiting.
    pub fn record_communication_wait(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.communication_waits,
            "communication_waits",
        )
    }

    /// Records a communication resource conflict.
    pub fn record_communication_conflict(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.communication_conflicts,
            "communication_conflicts",
        )
    }

    /// Records a QEC round.
    pub fn record_qec_round(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.qec_rounds, "qec_rounds")
    }

    /// Records a syndrome-related operation.
    pub fn record_syndrome_operation(&mut self) -> ProfileResult<()> {
        Self::increment(
            &mut self.syndrome_operations,
            "syndrome_operations",
        )
    }

    /// Records a planning failure.
    pub fn record_planning_failure(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.planning_failures, "planning_failures")
    }

    /// Records cancellation.
    pub fn record_cancellation(&mut self) -> ProfileResult<()> {
        Self::increment(&mut self.cancellations, "cancellations")
    }

    /// Returns the number of operations observed.
    #[must_use]
    pub const fn operations_seen(&self) -> u128 {
        self.operations_seen
    }

    /// Returns the number of scheduled operations.
    #[must_use]
    pub const fn operations_scheduled(&self) -> u128 {
        self.operations_scheduled
    }

    /// Returns the number of delayed operations.
    #[must_use]
    pub const fn operations_delayed(&self) -> u128 {
        self.operations_delayed
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub const fn dependency_edges(&self) -> u128 {
        self.dependency_edges
    }

    /// Returns the number of resource conflicts.
    #[must_use]
    pub const fn resource_conflicts(&self) -> u128 {
        self.resource_conflicts
    }

    /// Returns the number of scheduling decisions.
    #[must_use]
    pub const fn scheduling_decisions(&self) -> u128 {
        self.scheduling_decisions
    }

    /// Returns the peak ready queue size.
    #[must_use]
    pub const fn peak_ready_queue(&self) -> u128 {
        self.peak_ready_queue
    }

    /// Returns the number of verification failures.
    #[must_use]
    pub const fn verification_failures(&self) -> u128 {
        self.verification_failures
    }

    /// Returns the number of inserted delays.
    #[must_use]
    pub const fn inserted_delays(&self) -> u128 {
        self.inserted_delays
    }
}

// =============================================================================
// Phase measurement
// =============================================================================

/// Host-side elapsed time accumulated for one scheduler phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhaseMeasurement {
    /// Number of completed measurements for this phase.
    count: u128,

    /// Aggregate host elapsed time in nanoseconds.
    elapsed_nanos: u128,
}

impl PhaseMeasurement {
    /// Creates an empty phase measurement.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            count: 0,
            elapsed_nanos: 0,
        }
    }

    fn record(&mut self, elapsed: HostDuration) -> ProfileResult<()> {
        let nanos = elapsed.as_nanos();

        self.count = self
            .count
            .checked_add(1)
            .ok_or(ProfileError::CounterOverflow {
                counter: "phase_count",
            })?;

        self.elapsed_nanos = self
            .elapsed_nanos
            .checked_add(nanos)
            .ok_or(ProfileError::CounterOverflow {
                counter: "phase_elapsed_nanos",
            })?;

        Ok(())
    }

    /// Returns the number of completed measurements.
    #[must_use]
    pub const fn count(self) -> u128 {
        self.count
    }

    /// Returns aggregate elapsed host time.
    #[must_use]
    pub const fn elapsed(self) -> HostDuration {
        nanos_to_duration(self.elapsed_nanos)
    }

    /// Returns aggregate elapsed nanoseconds.
    #[must_use]
    pub const fn elapsed_nanos(self) -> u128 {
        self.elapsed_nanos
    }

    /// Returns average elapsed time when at least one measurement exists.
    #[must_use]
    pub fn average(self) -> Option<HostDuration> {
        if self.count == 0 {
            return None;
        }

        Some(nanos_to_duration(self.elapsed_nanos / self.count))
    }
}

// =============================================================================
// Phase collection
// =============================================================================

/// Collection of profiling measurements by scheduler phase.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PhaseMeasurements {
    preparation: PhaseMeasurement,
    dependency_analysis: PhaseMeasurement,
    resource_analysis: PhaseMeasurement,
    timing_analysis: PhaseMeasurement,
    constraint_analysis: PhaseMeasurement,
    planning: PhaseMeasurement,
    resource_scheduling: PhaseMeasurement,
    alignment: PhaseMeasurement,
    transformation: PhaseMeasurement,
    verification: PhaseMeasurement,
    optimization: PhaseMeasurement,
    dynamic_scheduling: PhaseMeasurement,
    distributed_scheduling: PhaseMeasurement,
    qec_scheduling: PhaseMeasurement,
    result_construction: PhaseMeasurement,
    serialization: PhaseMeasurement,
    other: PhaseMeasurement,
}

impl PhaseMeasurements {
    /// Creates empty phase measurements.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            preparation: PhaseMeasurement::new(),
            dependency_analysis: PhaseMeasurement::new(),
            resource_analysis: PhaseMeasurement::new(),
            timing_analysis: PhaseMeasurement::new(),
            constraint_analysis: PhaseMeasurement::new(),
            planning: PhaseMeasurement::new(),
            resource_scheduling: PhaseMeasurement::new(),
            alignment: PhaseMeasurement::new(),
            transformation: PhaseMeasurement::new(),
            verification: PhaseMeasurement::new(),
            optimization: PhaseMeasurement::new(),
            dynamic_scheduling: PhaseMeasurement::new(),
            distributed_scheduling: PhaseMeasurement::new(),
            qec_scheduling: PhaseMeasurement::new(),
            result_construction: PhaseMeasurement::new(),
            serialization: PhaseMeasurement::new(),
            other: PhaseMeasurement::new(),
        }
    }

    fn record(
        &mut self,
        phase: ProfilePhase,
        elapsed: HostDuration,
    ) -> ProfileResult<()> {
        match phase {
            ProfilePhase::Preparation => {
                self.preparation.record(elapsed)
            }
            ProfilePhase::DependencyAnalysis => {
                self.dependency_analysis.record(elapsed)
            }
            ProfilePhase::ResourceAnalysis => {
                self.resource_analysis.record(elapsed)
            }
            ProfilePhase::TimingAnalysis => {
                self.timing_analysis.record(elapsed)
            }
            ProfilePhase::ConstraintAnalysis => {
                self.constraint_analysis.record(elapsed)
            }
            ProfilePhase::Planning => self.planning.record(elapsed),
            ProfilePhase::ResourceScheduling => {
                self.resource_scheduling.record(elapsed)
            }
            ProfilePhase::Alignment => self.alignment.record(elapsed),
            ProfilePhase::Transformation => {
                self.transformation.record(elapsed)
            }
            ProfilePhase::Verification => self.verification.record(elapsed),
            ProfilePhase::Optimization => {
                self.optimization.record(elapsed)
            }
            ProfilePhase::DynamicScheduling => {
                self.dynamic_scheduling.record(elapsed)
            }
            ProfilePhase::DistributedScheduling => {
                self.distributed_scheduling.record(elapsed)
            }
            ProfilePhase::QecScheduling => {
                self.qec_scheduling.record(elapsed)
            }
            ProfilePhase::ResultConstruction => {
                self.result_construction.record(elapsed)
            }
            ProfilePhase::Serialization => {
                self.serialization.record(elapsed)
            }
            ProfilePhase::Other => self.other.record(elapsed),
        }
    }

    /// Returns measurements for a phase.
    #[must_use]
    pub const fn get(&self, phase: ProfilePhase) -> PhaseMeasurement {
        match phase {
            ProfilePhase::Preparation => self.preparation,
            ProfilePhase::DependencyAnalysis => self.dependency_analysis,
            ProfilePhase::ResourceAnalysis => self.resource_analysis,
            ProfilePhase::TimingAnalysis => self.timing_analysis,
            ProfilePhase::ConstraintAnalysis => self.constraint_analysis,
            ProfilePhase::Planning => self.planning,
            ProfilePhase::ResourceScheduling => self.resource_scheduling,
            ProfilePhase::Alignment => self.alignment,
            ProfilePhase::Transformation => self.transformation,
            ProfilePhase::Verification => self.verification,
            ProfilePhase::Optimization => self.optimization,
            ProfilePhase::DynamicScheduling => self.dynamic_scheduling,
            ProfilePhase::DistributedScheduling => {
                self.distributed_scheduling
            }
            ProfilePhase::QecScheduling => self.qec_scheduling,
            ProfilePhase::ResultConstruction => self.result_construction,
            ProfilePhase::Serialization => self.serialization,
            ProfilePhase::Other => self.other,
        }
    }

    /// Returns all phase measurements in deterministic order.
    #[must_use]
    pub fn entries(&self) -> [(ProfilePhase, PhaseMeasurement); 17] {
        [
            (ProfilePhase::Preparation, self.preparation),
            (
                ProfilePhase::DependencyAnalysis,
                self.dependency_analysis,
            ),
            (ProfilePhase::ResourceAnalysis, self.resource_analysis),
            (ProfilePhase::TimingAnalysis, self.timing_analysis),
            (
                ProfilePhase::ConstraintAnalysis,
                self.constraint_analysis,
            ),
            (ProfilePhase::Planning, self.planning),
            (
                ProfilePhase::ResourceScheduling,
                self.resource_scheduling,
            ),
            (ProfilePhase::Alignment, self.alignment),
            (ProfilePhase::Transformation, self.transformation),
            (ProfilePhase::Verification, self.verification),
            (ProfilePhase::Optimization, self.optimization),
            (
                ProfilePhase::DynamicScheduling,
                self.dynamic_scheduling,
            ),
            (
                ProfilePhase::DistributedScheduling,
                self.distributed_scheduling,
            ),
            (ProfilePhase::QecScheduling, self.qec_scheduling),
            (
                ProfilePhase::ResultConstruction,
                self.result_construction,
            ),
            (ProfilePhase::Serialization, self.serialization),
            (ProfilePhase::Other, self.other),
        ]
    }
}

// =============================================================================
// Optional qubit observation
// =============================================================================

/// Controls whether profiling retains canonical qubit identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QubitTracking {
    /// Do not retain qubit identities.
    Disabled,

    /// Retain distinct logical/physical identity values supplied by callers.
    ///
    /// This is useful when profiling a specific workload, but consumes memory
    /// proportional to the number of unique qubits observed.
    Unique,
}

impl Default for QubitTracking {
    fn default() -> Self {
        Self::Disabled
    }
}

// =============================================================================
// Profile configuration
// =============================================================================

/// Configuration for a profiling session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileConfig {
    /// Whether host-side phase timing is enabled.
    measure_host_time: bool,

    /// Whether canonical qubit identities should be retained.
    qubit_tracking: QubitTracking,
}

impl ProfileConfig {
    /// Creates a lightweight aggregate-only profile configuration.
    #[must_use]
    pub const fn aggregate() -> Self {
        Self {
            measure_host_time: true,
            qubit_tracking: QubitTracking::Disabled,
        }
    }

    /// Creates a profile that also retains unique qubit identities.
    #[must_use]
    pub const fn with_unique_qubits() -> Self {
        Self {
            measure_host_time: true,
            qubit_tracking: QubitTracking::Unique,
        }
    }

    /// Creates a profile with host timing disabled.
    #[must_use]
    pub const fn counters_only() -> Self {
        Self {
            measure_host_time: false,
            qubit_tracking: QubitTracking::Disabled,
        }
    }

    /// Returns whether host-side timing is enabled.
    #[must_use]
    pub const fn measures_host_time(self) -> bool {
        self.measure_host_time
    }

    /// Returns the qubit tracking mode.
    #[must_use]
    pub const fn qubit_tracking(self) -> QubitTracking {
        self.qubit_tracking
    }
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self::aggregate()
    }
}

// =============================================================================
// Profile
// =============================================================================

/// Mutable profiling accumulator.
///
/// A `Profile` belongs to one scheduler execution context.
///
/// It should normally be created at the scheduling composition boundary and
/// passed to planners/algorithms as an optional instrumentation dependency.
///
/// The profile itself does not decide whether profiling is enabled.
#[derive(Debug, Default)]
pub struct Profile {
    config: ProfileConfig,
    counters: ProfileCounters,
    phases: PhaseMeasurements,

    started_at: Option<Instant>,
    host_elapsed_nanos: u128,

    schedule_makespan: Option<TimePoint>,
    scheduled_duration: Option<Duration>,
    idle_duration: Option<Duration>,

    peak_parallel_operations: u128,
    peak_active_resources: u128,

    unique_qubits: Option<BTreeSet<QubitId>>,
}

impl Profile {
    /// Creates a profile using the supplied configuration.
    #[must_use]
    pub fn new(config: ProfileConfig) -> Self {
        let unique_qubits = match config.qubit_tracking() {
            QubitTracking::Disabled => None,
            QubitTracking::Unique => Some(BTreeSet::new()),
        };

        Self {
            config,
            counters: ProfileCounters::new(),
            phases: PhaseMeasurements::new(),
            started_at: None,
            host_elapsed_nanos: 0,
            schedule_makespan: None,
            scheduled_duration: None,
            idle_duration: None,
            peak_parallel_operations: 0,
            peak_active_resources: 0,
            unique_qubits,
        }
    }

    /// Starts the overall host-side profile timer.
    ///
    /// Calling this more than once resets the profile's start point. The
    /// counters themselves are retained.
    pub fn start(&mut self) {
        if self.config.measures_host_time() {
            self.started_at = Some(Instant::now());
        }
    }

    /// Stops the overall host-side profile timer.
    pub fn stop(&mut self) -> ProfileResult<()> {
        if !self.config.measures_host_time() {
            return Ok(());
        }

        if let Some(started_at) = self.started_at.take() {
            let elapsed = started_at.elapsed().as_nanos();

            self.host_elapsed_nanos = self
                .host_elapsed_nanos
                .checked_add(elapsed)
                .ok_or(ProfileError::CounterOverflow {
                    counter: "host_elapsed_nanos",
                })?;
        }

        Ok(())
    }

    /// Returns whether the profile currently has an active overall timer.
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.started_at.is_some()
    }

    /// Begins measuring one scheduler phase.
    ///
    /// The returned guard records elapsed time when dropped.
    ///
    /// If host timing is disabled, the guard becomes a zero-cost logical
    /// lifetime marker.
    pub fn phase(&mut self, phase: ProfilePhase) -> ProfilePhaseGuard {
        let started_at = if self.config.measures_host_time() {
            Some(Instant::now())
        } else {
            None
        };

        ProfilePhaseGuard {
            phase,
            started_at,
            completed: false,
        }
    }

    /// Explicitly records a completed phase measurement.
    pub fn record_phase(
        &mut self,
        phase: ProfilePhase,
        elapsed: HostDuration,
    ) -> ProfileResult<()> {
        self.phases.record(phase, elapsed)
    }

    /// Records an operation observation.
    pub fn record_operation(
        &mut self,
        operation: OperationId,
    ) -> ProfileResult<()> {
        if operation.is_zero() {
            return Err(ProfileError::InvalidOperation {
                operation,
                message: "zero operation identity is not valid for an observed operation",
            });
        }

        self.counters.record_operation_seen()
    }

    /// Records a canonical qubit observation.
    ///
    /// When unique tracking is disabled this operation performs no allocation.
    pub fn observe_qubit(&mut self, qubit: QubitId) {
        if let Some(qubits) = self.unique_qubits.as_mut() {
            qubits.insert(qubit);
        }
    }

    /// Records a resource observation.
    pub fn record_resource(
        &mut self,
        resource: ResourceId,
    ) -> ProfileResult<()> {
        if resource.is_zero() {
            return Err(ProfileError::InvalidResource {
                resource,
                message: "zero resource identity is not valid for an observed resource",
            });
        }

        self.counters.record_resource_seen()
    }

    /// Records a scheduled operation.
    pub fn record_operation_scheduled(&mut self) -> ProfileResult<()> {
        self.counters.record_operation_scheduled()
    }

    /// Records a delayed operation.
    pub fn record_operation_delayed(&mut self) -> ProfileResult<()> {
        self.counters.record_operation_delayed()
    }

    /// Records an operation completion.
    pub fn record_operation_completed(&mut self) -> ProfileResult<()> {
        self.counters.record_operation_completed()
    }

    /// Records a scheduling decision.
    pub fn record_scheduling_decision(&mut self) -> ProfileResult<()> {
        self.counters.record_scheduling_decision()
    }

    /// Records a scheduling iteration.
    pub fn record_scheduling_iteration(&mut self) -> ProfileResult<()> {
        self.counters.record_scheduling_iteration()
    }

    /// Records a resource conflict.
    pub fn record_resource_conflict(&mut self) -> ProfileResult<()> {
        self.counters.record_resource_conflict()
    }

    /// Records a dependency wait.
    pub fn record_dependency_wait(&mut self) -> ProfileResult<()> {
        self.counters.record_dependency_wait()
    }

    /// Records a resource wait.
    pub fn record_resource_wait(&mut self) -> ProfileResult<()> {
        self.counters.record_resource_wait()
    }

    /// Records a timing alignment adjustment.
    pub fn record_alignment_adjustment(&mut self) -> ProfileResult<()> {
        self.counters.record_alignment_adjustment()
    }

    /// Records an inserted delay.
    pub fn record_inserted_delay(&mut self) -> ProfileResult<()> {
        self.counters.record_inserted_delay()
    }

    /// Records a verification check.
    pub fn record_verification_check(&mut self) -> ProfileResult<()> {
        self.counters.record_verification_check()
    }

    /// Records a verification failure.
    pub fn record_verification_failure(&mut self) -> ProfileResult<()> {
        self.counters.record_verification_failure()
    }

    /// Records a dynamic scheduling event.
    pub fn record_dynamic_event(&mut self) -> ProfileResult<()> {
        self.counters.record_dynamic_event()
    }

    /// Records a communication wait.
    pub fn record_communication_wait(&mut self) -> ProfileResult<()> {
        self.counters.record_communication_wait()
    }

    /// Records a QEC round.
    pub fn record_qec_round(&mut self) -> ProfileResult<()> {
        self.counters.record_qec_round()
    }

    /// Observes the current ready queue size.
    pub fn observe_ready_queue_size(
        &mut self,
        size: u128,
    ) -> ProfileResult<()> {
        self.counters.observe_ready_queue_size(size)
    }

    /// Observes the current number of concurrently active operations.
    pub fn observe_parallel_operations(
        &mut self,
        count: u128,
    ) -> ProfileResult<()> {
        if count > self.peak_parallel_operations {
            self.peak_parallel_operations = count;
        }

        Ok(())
    }

    /// Observes the number of currently active resources.
    pub fn observe_active_resources(
        &mut self,
        count: u128,
    ) -> ProfileResult<()> {
        if count > self.peak_active_resources {
            self.peak_active_resources = count;
        }

        Ok(())
    }

    /// Sets the resulting schedule makespan.
    ///
    /// The value is scheduler time, not host elapsed time.
    pub fn set_schedule_makespan(&mut self, makespan: TimePoint) {
        self.schedule_makespan = Some(makespan);
    }

    /// Sets total scheduled execution duration.
    pub fn set_scheduled_duration(&mut self, duration: Duration) {
        self.scheduled_duration = Some(duration);
    }

    /// Sets total scheduler-observed idle duration.
    pub fn set_idle_duration(&mut self, duration: Duration) {
        self.idle_duration = Some(duration);
    }

    /// Returns the mutable aggregate counters.
    pub fn counters_mut(&mut self) -> &mut ProfileCounters {
        &mut self.counters
    }

    /// Returns the aggregate counters.
    #[must_use]
    pub const fn counters(&self) -> &ProfileCounters {
        &self.counters
    }

    /// Returns phase measurements.
    #[must_use]
    pub const fn phases(&self) -> &PhaseMeasurements {
        &self.phases
    }

    /// Returns the number of unique qubits when unique tracking is enabled.
    #[must_use]
    pub fn unique_qubit_count(&self) -> Option<u128> {
        self.unique_qubits
            .as_ref()
            .map(|qubits| qubits.len() as u128)
    }

    /// Returns whether unique-qubit tracking is enabled.
    #[must_use]
    pub const fn tracks_unique_qubits(&self) -> bool {
        matches!(
            self.config.qubit_tracking(),
            QubitTracking::Unique
        )
    }

    /// Returns host elapsed time accumulated so far.
    #[must_use]
    pub fn host_elapsed(&self) -> HostDuration {
        let mut nanos = self.host_elapsed_nanos;

        if let Some(started_at) = self.started_at {
            nanos = nanos.saturating_add(started_at.elapsed().as_nanos());
        }

        nanos_to_duration(nanos)
    }

    /// Returns the schedule makespan.
    #[must_use]
    pub const fn schedule_makespan(&self) -> Option<TimePoint> {
        self.schedule_makespan
    }

    /// Returns total scheduled duration.
    #[must_use]
    pub const fn scheduled_duration(&self) -> Option<Duration> {
        self.scheduled_duration
    }

    /// Returns total idle duration.
    #[must_use]
    pub const fn idle_duration(&self) -> Option<Duration> {
        self.idle_duration
    }

    /// Returns peak parallel operation count.
    #[must_use]
    pub const fn peak_parallel_operations(&self) -> u128 {
        self.peak_parallel_operations
    }

    /// Returns peak active resource count.
    #[must_use]
    pub const fn peak_active_resources(&self) -> u128 {
        self.peak_active_resources
    }

    /// Merges another independent worker profile into this profile.
    ///
    /// This is the preferred integration mechanism for parallel schedulers.
    ///
    /// The two profiles should represent independent work contributing to the
    /// same logical scheduling invocation.
    ///
    /// Host elapsed time is summed.
    ///
    /// Peak values are combined by maximum.
    ///
    /// Schedule-level values are combined conservatively:
    ///
    /// - makespan: maximum;
    /// - scheduled duration: sum;
    /// - idle duration: sum.
    pub fn merge(&mut self, other: &Profile) -> ProfileResult<()> {
        merge_counters(&mut self.counters, &other.counters)?;

        merge_phases(&mut self.phases, &other.phases)?;

        self.host_elapsed_nanos = checked_add(
            self.host_elapsed_nanos,
            other.host_elapsed_nanos,
            "host_elapsed_nanos",
        )?;

        self.peak_parallel_operations = self
            .peak_parallel_operations
            .max(other.peak_parallel_operations);

        self.peak_active_resources =
            self.peak_active_resources.max(other.peak_active_resources);

        self.counters.peak_ready_queue = self
            .counters
            .peak_ready_queue
            .max(other.counters.peak_ready_queue);

        self.schedule_makespan = match (
            self.schedule_makespan,
            other.schedule_makespan,
        ) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (None, value) | (value, None) => value,
        };

        self.scheduled_duration = merge_duration_sum(
            self.scheduled_duration,
            other.scheduled_duration,
        )?;

        self.idle_duration =
            merge_duration_sum(self.idle_duration, other.idle_duration)?;

        if let Some(target) = self.unique_qubits.as_mut() {
            if let Some(source) = other.unique_qubits.as_ref() {
                target.extend(source.iter().copied());
            }
        }

        Ok(())
    }

    /// Finalizes the profile into an immutable snapshot.
    pub fn snapshot(&mut self) -> ProfileResult<ProfileSnapshot> {
        self.stop()?;

        Ok(ProfileSnapshot {
            counters: self.counters.clone(),
            phases: self.phases.clone(),
            host_elapsed_nanos: self.host_elapsed_nanos,
            schedule_makespan: self.schedule_makespan,
            scheduled_duration: self.scheduled_duration,
            idle_duration: self.idle_duration,
            peak_parallel_operations: self.peak_parallel_operations,
            peak_active_resources: self.peak_active_resources,
            unique_qubit_count: self.unique_qubit_count(),
        })
    }
}

// =============================================================================
// RAII phase guard
// =============================================================================

/// RAII guard used to record host elapsed time for one profiling phase.
///
/// The guard owns no mutable reference to the profile, which permits scheduler
/// code to perform work while the guard is alive.
///
/// Call [`ProfilePhaseGuard::finish`] to explicitly attach the measurement to a
/// profile.
///
/// Dropping an unfinished guard does not mutate global state and does not
/// silently modify a profile.
#[derive(Debug)]
pub struct ProfilePhaseGuard {
    phase: ProfilePhase,
    started_at: Option<Instant>,
    completed: bool,
}

impl ProfilePhaseGuard {
    /// Returns the measured phase.
    #[must_use]
    pub const fn phase(&self) -> ProfilePhase {
        self.phase
    }

    /// Finishes this phase and returns its elapsed host time.
    pub fn finish(&mut self) -> ProfileResult<HostDuration> {
        if self.completed {
            return Err(ProfileError::PhaseAlreadyClosed);
        }

        self.completed = true;

        let elapsed = self
            .started_at
            .map(|start| start.elapsed())
            .unwrap_or_else(HostDuration::default);

        Ok(elapsed)
    }

    /// Finishes the guard and records the measurement into the supplied
    /// profile.
    pub fn finish_into(
        &mut self,
        profile: &mut Profile,
    ) -> ProfileResult<()> {
        let elapsed = self.finish()?;
        profile.record_phase(self.phase, elapsed)
    }
}

impl Drop for ProfilePhaseGuard {
    fn drop(&mut self) {
        // Intentionally do nothing.
        //
        // A dropped guard must never silently mutate a profile because it no
        // longer owns a profile reference. Call `finish_into` explicitly.
    }
}

// =============================================================================
// Immutable snapshot
// =============================================================================

/// Immutable profiling result.
///
/// This is suitable for diagnostics, benchmarking, reporting, and
/// serialization adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileSnapshot {
    counters: ProfileCounters,
    phases: PhaseMeasurements,

    host_elapsed_nanos: u128,

    schedule_makespan: Option<TimePoint>,
    scheduled_duration: Option<Duration>,
    idle_duration: Option<Duration>,

    peak_parallel_operations: u128,
    peak_active_resources: u128,

    unique_qubit_count: Option<u128>,
}

impl ProfileSnapshot {
    /// Returns aggregate scheduler counters.
    #[must_use]
    pub const fn counters(&self) -> &ProfileCounters {
        &self.counters
    }

    /// Returns phase measurements.
    #[must_use]
    pub const fn phases(&self) -> &PhaseMeasurements {
        &self.phases
    }

    /// Returns total host profiling time.
    #[must_use]
    pub fn host_elapsed(&self) -> HostDuration {
        nanos_to_duration(self.host_elapsed_nanos)
    }

    /// Returns host elapsed nanoseconds.
    #[must_use]
    pub const fn host_elapsed_nanos(&self) -> u128 {
        self.host_elapsed_nanos
    }

    /// Returns scheduler makespan.
    #[must_use]
    pub const fn schedule_makespan(&self) -> Option<TimePoint> {
        self.schedule_makespan
    }

    /// Returns total scheduled execution duration.
    #[must_use]
    pub const fn scheduled_duration(&self) -> Option<Duration> {
        self.scheduled_duration
    }

    /// Returns total idle duration.
    #[must_use]
    pub const fn idle_duration(&self) -> Option<Duration> {
        self.idle_duration
    }

    /// Returns peak scheduler parallelism.
    #[must_use]
    pub const fn peak_parallel_operations(&self) -> u128 {
        self.peak_parallel_operations
    }

    /// Returns peak active resources.
    #[must_use]
    pub const fn peak_active_resources(&self) -> u128 {
        self.peak_active_resources
    }

    /// Returns unique qubit count when tracking was enabled.
    #[must_use]
    pub const fn unique_qubit_count(&self) -> Option<u128> {
        self.unique_qubit_count
    }

    /// Returns the number of operations observed.
    #[must_use]
    pub const fn operation_count(&self) -> u128 {
        self.counters.operations_seen
    }

    /// Returns the number of operations successfully scheduled.
    #[must_use]
    pub const fn scheduled_operation_count(&self) -> u128 {
        self.counters.operations_scheduled
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub const fn dependency_edge_count(&self) -> u128 {
        self.counters.dependency_edges
    }

    /// Returns the number of resource reservations.
    #[must_use]
    pub const fn resource_reservation_count(&self) -> u128 {
        self.counters.resource_reservations
    }

    /// Returns the number of resource conflicts.
    #[must_use]
    pub const fn resource_conflict_count(&self) -> u128 {
        self.counters.resource_conflicts
    }

    /// Returns the number of inserted delays.
    #[must_use]
    pub const fn inserted_delay_count(&self) -> u128 {
        self.counters.inserted_delays
    }

    /// Returns the number of verification failures.
    #[must_use]
    pub const fn verification_failure_count(&self) -> u128 {
        self.counters.verification_failures
    }

    /// Returns a deterministic collection of profile values suitable for
    /// higher-level fingerprinting.
    ///
    /// Host elapsed time is deliberately excluded because it depends on the
    /// machine running the compiler.
    #[must_use]
    pub fn semantic_fingerprint_values(&self) -> Vec<(&'static str, u128)> {
        vec![
            ("operations_seen", self.counters.operations_seen),
            (
                "operations_scheduled",
                self.counters.operations_scheduled,
            ),
            (
                "operations_delayed",
                self.counters.operations_delayed,
            ),
            (
                "operations_completed",
                self.counters.operations_completed,
            ),
            ("dependency_edges", self.counters.dependency_edges),
            ("dependency_waits", self.counters.dependency_waits),
            ("resources_seen", self.counters.resources_seen),
            (
                "resource_reservations",
                self.counters.resource_reservations,
            ),
            (
                "resource_conflicts",
                self.counters.resource_conflicts,
            ),
            (
                "constraints_evaluated",
                self.counters.constraints_evaluated,
            ),
            (
                "constraints_rejected",
                self.counters.constraints_rejected,
            ),
            (
                "scheduling_decisions",
                self.counters.scheduling_decisions,
            ),
            (
                "scheduling_iterations",
                self.counters.scheduling_iterations,
            ),
            ("backtracks", self.counters.backtracks),
            (
                "peak_ready_queue",
                self.counters.peak_ready_queue,
            ),
            (
                "alignment_adjustments",
                self.counters.alignment_adjustments,
            ),
            ("inserted_delays", self.counters.inserted_delays),
            ("inserted_padding", self.counters.inserted_padding),
            ("transformations", self.counters.transformations),
            (
                "verification_checks",
                self.counters.verification_checks,
            ),
            (
                "verification_failures",
                self.counters.verification_failures,
            ),
            (
                "optimization_iterations",
                self.counters.optimization_iterations,
            ),
            (
                "objective_evaluations",
                self.counters.objective_evaluations,
            ),
            ("dynamic_events", self.counters.dynamic_events),
            (
                "conditional_operations",
                self.counters.conditional_operations,
            ),
            ("feedback_waits", self.counters.feedback_waits),
            (
                "communication_operations",
                self.counters.communication_operations,
            ),
            (
                "communication_waits",
                self.counters.communication_waits,
            ),
            (
                "communication_conflicts",
                self.counters.communication_conflicts,
            ),
            ("qec_rounds", self.counters.qec_rounds),
            (
                "syndrome_operations",
                self.counters.syndrome_operations,
            ),
            ("planning_failures", self.counters.planning_failures),
            ("cancellations", self.counters.cancellations),
            (
                "peak_parallel_operations",
                self.peak_parallel_operations,
            ),
            (
                "peak_active_resources",
                self.peak_active_resources,
            ),
        ]
    }
}

// =============================================================================
// Utility functions
// =============================================================================

fn checked_add(
    left: u128,
    right: u128,
    field: &'static str,
) -> ProfileResult<u128> {
    left.checked_add(right)
        .ok_or(ProfileError::MergeOverflow { field })
}

fn merge_counters(
    target: &mut ProfileCounters,
    source: &ProfileCounters,
) -> ProfileResult<()> {
    target.operations_seen = checked_add(
        target.operations_seen,
        source.operations_seen,
        "operations_seen",
    )?;

    target.operations_scheduled = checked_add(
        target.operations_scheduled,
        source.operations_scheduled,
        "operations_scheduled",
    )?;

    target.operations_delayed = checked_add(
        target.operations_delayed,
        source.operations_delayed,
        "operations_delayed",
    )?;

    target.operations_completed = checked_add(
        target.operations_completed,
        source.operations_completed,
        "operations_completed",
    )?;

    target.operations_rejected = checked_add(
        target.operations_rejected,
        source.operations_rejected,
        "operations_rejected",
    )?;

    target.operations_rescheduled = checked_add(
        target.operations_rescheduled,
        source.operations_rescheduled,
        "operations_rescheduled",
    )?;

    target.dependency_edges = checked_add(
        target.dependency_edges,
        source.dependency_edges,
        "dependency_edges",
    )?;

    target.dependency_waits = checked_add(
        target.dependency_waits,
        source.dependency_waits,
        "dependency_waits",
    )?;

    target.dependency_violations = checked_add(
        target.dependency_violations,
        source.dependency_violations,
        "dependency_violations",
    )?;

    target.resources_seen = checked_add(
        target.resources_seen,
        source.resources_seen,
        "resources_seen",
    )?;

    target.resource_reservations = checked_add(
        target.resource_reservations,
        source.resource_reservations,
        "resource_reservations",
    )?;

    target.resource_releases = checked_add(
        target.resource_releases,
        source.resource_releases,
        "resource_releases",
    )?;

    target.resource_waits = checked_add(
        target.resource_waits,
        source.resource_waits,
        "resource_waits",
    )?;

    target.resource_conflicts = checked_add(
        target.resource_conflicts,
        source.resource_conflicts,
        "resource_conflicts",
    )?;

    target.capacity_conflicts = checked_add(
        target.capacity_conflicts,
        source.capacity_conflicts,
        "capacity_conflicts",
    )?;

    target.constraints_evaluated = checked_add(
        target.constraints_evaluated,
        source.constraints_evaluated,
        "constraints_evaluated",
    )?;

    target.constraints_satisfied = checked_add(
        target.constraints_satisfied,
        source.constraints_satisfied,
        "constraints_satisfied",
    )?;

    target.constraints_rejected = checked_add(
        target.constraints_rejected,
        source.constraints_rejected,
        "constraints_rejected",
    )?;

    target.constraint_conflicts = checked_add(
        target.constraint_conflicts,
        source.constraint_conflicts,
        "constraint_conflicts",
    )?;

    target.scheduling_decisions = checked_add(
        target.scheduling_decisions,
        source.scheduling_decisions,
        "scheduling_decisions",
    )?;

    target.scheduling_iterations = checked_add(
        target.scheduling_iterations,
        source.scheduling_iterations,
        "scheduling_iterations",
    )?;

    target.backtracks = checked_add(
        target.backtracks,
        source.backtracks,
        "backtracks",
    )?;

    target.ready_queue_insertions = checked_add(
        target.ready_queue_insertions,
        source.ready_queue_insertions,
        "ready_queue_insertions",
    )?;

    target.ready_queue_removals = checked_add(
        target.ready_queue_removals,
        source.ready_queue_removals,
        "ready_queue_removals",
    )?;

    target.alignment_adjustments = checked_add(
        target.alignment_adjustments,
        source.alignment_adjustments,
        "alignment_adjustments",
    )?;

    target.inserted_delays = checked_add(
        target.inserted_delays,
        source.inserted_delays,
        "inserted_delays",
    )?;

    target.inserted_padding = checked_add(
        target.inserted_padding,
        source.inserted_padding,
        "inserted_padding",
    )?;

    target.transformations = checked_add(
        target.transformations,
        source.transformations,
        "transformations",
    )?;

    target.verification_checks = checked_add(
        target.verification_checks,
        source.verification_checks,
        "verification_checks",
    )?;

    target.verification_failures = checked_add(
        target.verification_failures,
        source.verification_failures,
        "verification_failures",
    )?;

    target.optimization_iterations = checked_add(
        target.optimization_iterations,
        source.optimization_iterations,
        "optimization_iterations",
    )?;

    target.objective_evaluations = checked_add(
        target.objective_evaluations,
        source.objective_evaluations,
        "objective_evaluations",
    )?;

    target.dynamic_events = checked_add(
        target.dynamic_events,
        source.dynamic_events,
        "dynamic_events",
    )?;

    target.conditional_operations = checked_add(
        target.conditional_operations,
        source.conditional_operations,
        "conditional_operations",
    )?;

    target.feedback_waits = checked_add(
        target.feedback_waits,
        source.feedback_waits,
        "feedback_waits",
    )?;

    target.communication_operations = checked_add(
        target.communication_operations,
        source.communication_operations,
        "communication_operations",
    )?;

    target.communication_waits = checked_add(
        target.communication_waits,
        source.communication_waits,
        "communication_waits",
    )?;

    target.communication_conflicts = checked_add(
        target.communication_conflicts,
        source.communication_conflicts,
        "communication_conflicts",
    )?;

    target.qec_rounds = checked_add(
        target.qec_rounds,
        source.qec_rounds,
        "qec_rounds",
    )?;

    target.syndrome_operations = checked_add(
        target.syndrome_operations,
        source.syndrome_operations,
        "syndrome_operations",
    )?;

    target.planning_failures = checked_add(
        target.planning_failures,
        source.planning_failures,
        "planning_failures",
    )?;

    target.cancellations = checked_add(
        target.cancellations,
        source.cancellations,
        "cancellations",
    )?;

    target.peak_ready_queue = target
        .peak_ready_queue
        .max(source.peak_ready_queue);

    Ok(())
}

fn merge_phases(
    target: &mut PhaseMeasurements,
    source: &PhaseMeasurements,
) -> ProfileResult<()> {
    for (phase, measurement) in source.entries() {
        let destination = match phase {
            ProfilePhase::Preparation => &mut target.preparation,
            ProfilePhase::DependencyAnalysis => {
                &mut target.dependency_analysis
            }
            ProfilePhase::ResourceAnalysis => {
                &mut target.resource_analysis
            }
            ProfilePhase::TimingAnalysis => &mut target.timing_analysis,
            ProfilePhase::ConstraintAnalysis => {
                &mut target.constraint_analysis
            }
            ProfilePhase::Planning => &mut target.planning,
            ProfilePhase::ResourceScheduling => {
                &mut target.resource_scheduling
            }
            ProfilePhase::Alignment => &mut target.alignment,
            ProfilePhase::Transformation => &mut target.transformation,
            ProfilePhase::Verification => &mut target.verification,
            ProfilePhase::Optimization => &mut target.optimization,
            ProfilePhase::DynamicScheduling => {
                &mut target.dynamic_scheduling
            }
            ProfilePhase::DistributedScheduling => {
                &mut target.distributed_scheduling
            }
            ProfilePhase::QecScheduling => &mut target.qec_scheduling,
            ProfilePhase::ResultConstruction => {
                &mut target.result_construction
            }
            ProfilePhase::Serialization => &mut target.serialization,
            ProfilePhase::Other => &mut target.other,
        };

        destination.count = checked_add(
            destination.count,
            measurement.count,
            "phase_count",
        )?;

        destination.elapsed_nanos = checked_add(
            destination.elapsed_nanos,
            measurement.elapsed_nanos,
            "phase_elapsed_nanos",
        )?;
    }

    Ok(())
}

fn merge_duration_sum(
    left: Option<Duration>,
    right: Option<Duration>,
) -> ProfileResult<Option<Duration>> {
    match (left, right) {
        (Some(a), Some(b)) => {
            let value = a
                .value()
                .checked_add(b.value())
                .ok_or(ProfileError::MergeOverflow {
                    field: "duration",
                })?;

            Ok(Some(Duration::new(value)))
        }

        (None, value) | (value, None) => Ok(value),
    }
}

/// Converts nanoseconds to `std::time::Duration` without panicking.
///
/// `std::time::Duration` stores seconds/nanoseconds and has a finite
/// representable range. Profiling internally uses u128, so this conversion
/// can theoretically exceed the host duration representation.
///
/// In that extraordinary case, the value is represented by the largest
/// representable `Duration` rather than wrapping.
fn nanos_to_duration(nanos: u128) -> HostDuration {
    let seconds = nanos / 1_000_000_000;
    let remainder = nanos % 1_000_000_000;

    match u64::try_from(seconds) {
        Ok(seconds) => HostDuration::new(seconds, remainder as u32),

        Err(_) => HostDuration::new(
            u64::MAX,
            999_999_999,
        ),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_profile_is_zero() {
        let profile = Profile::new(ProfileConfig::aggregate());

        assert_eq!(profile.counters().operations_seen(), 0);
        assert_eq!(profile.counters().operations_scheduled(), 0);
        assert_eq!(profile.unique_qubit_count(), None);
    }

    #[test]
    fn operation_recording_is_aggregate() {
        let mut profile = Profile::new(ProfileConfig::aggregate());

        profile
            .counters_mut()
            .record_operation_seen()
            .expect("counter should not overflow");

        profile
            .record_operation_scheduled()
            .expect("counter should not overflow");

        assert_eq!(profile.counters().operations_seen(), 1);
        assert_eq!(profile.counters().operations_scheduled(), 1);
    }

    #[test]
    fn unique_qubit_tracking_is_optional() {
        let mut disabled = Profile::new(ProfileConfig::aggregate());

        disabled.observe_qubit(QubitId::new(1));

        assert_eq!(disabled.unique_qubit_count(), None);

        let mut enabled =
            Profile::new(ProfileConfig::with_unique_qubits());

        enabled.observe_qubit(QubitId::new(1));
        enabled.observe_qubit(QubitId::new(1));
        enabled.observe_qubit(QubitId::new(2));

        assert_eq!(enabled.unique_qubit_count(), Some(2));
    }

    #[test]
    fn peak_values_are_monotonic() {
        let mut profile = Profile::new(ProfileConfig::aggregate());

        profile
            .observe_ready_queue_size(10)
            .expect("observation should succeed");

        profile
            .observe_ready_queue_size(5)
            .expect("observation should succeed");

        profile
            .observe_parallel_operations(7)
            .expect("observation should succeed");

        profile
            .observe_parallel_operations(3)
            .expect("observation should succeed");

        assert_eq!(profile.counters().peak_ready_queue(), 10);
        assert_eq!(profile.peak_parallel_operations(), 7);
    }

    #[test]
    fn profile_merge_combines_workers() {
        let mut first = Profile::new(ProfileConfig::aggregate());
        let mut second = Profile::new(ProfileConfig::aggregate());

        first
            .counters_mut()
            .record_operation_seen()
            .expect("first increment should succeed");

        second
            .counters_mut()
            .record_operation_seen()
            .expect("second increment should succeed");

        first
            .merge(&second)
            .expect("profile merge should succeed");

        assert_eq!(first.counters().operations_seen(), 2);
    }

    #[test]
    fn phase_measurement_records_elapsed_time() {
        let mut profile = Profile::new(ProfileConfig::aggregate());

        let mut guard = profile.phase(ProfilePhase::Planning);

        let _ = guard
            .finish_into(&mut profile)
            .expect("phase measurement should succeed");

        assert_eq!(
            profile
                .phases()
                .get(ProfilePhase::Planning)
                .count(),
            1
        );
    }

    #[test]
    fn counters_only_profile_has_no_host_timer() {
        let mut profile = Profile::new(ProfileConfig::counters_only());

        profile.start();

        assert!(!profile.is_running());

        profile
            .stop()
            .expect("stopping disabled timer should succeed");
    }

    #[test]
    fn snapshot_excludes_host_time_from_semantic_values() {
        let mut profile = Profile::new(ProfileConfig::aggregate());

        profile
            .counters_mut()
            .record_operation_seen()
            .expect("increment should succeed");

        let snapshot = profile
            .snapshot()
            .expect("snapshot should succeed");

        let values = snapshot.semantic_fingerprint_values();

        assert!(values
            .iter()
            .any(|(name, value)| *name == "operations_seen"
                && *value == 1));

        assert!(!values
            .iter()
            .any(|(name, _)| *name == "host_elapsed_nanos"));
    }
}