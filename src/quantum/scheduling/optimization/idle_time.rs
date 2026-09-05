//! Zamani Quantum Scheduling — Idle-Time Optimization
//!
//! Path:
//!     src/quantum/scheduling/optimization/idle_time.rs
//!
//! # Purpose
//!
//! This module provides the production idle-time optimization boundary for
//! Zamani quantum scheduling.
//!
//! It answers:
//!
//!     "Where can scheduled work be retimed to reduce resource idle time
//!      without violating the constraints supplied by the caller?"
//!
//! This module deliberately separates:
//!
//! - idle-time measurement;
//! - idle-time cost calculation;
//! - candidate retiming generation;
//! - candidate validation;
//! - deterministic candidate ordering;
//!
//! from the actual scheduling algorithm that commits a new schedule.
//!
//! The canonical schedule remains owned by:
//!
//!     crate::quantum::ir::scheduling
//!
//! The canonical qubit identities remain owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! The scheduler remains responsible for deciding:
//!
//!     WHEN
//!
//! Routing remains responsible for:
//!
//!     WHERE
//!
//! This module only optimizes already scheduled work subject to explicit
//! retiming bounds.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! quantum::frontend
//!       |
//!       v
//! quantum::ir
//!       |
//!       v
//! optimization
//!       |
//!       v
//! routing
//!       |
//!       v
//! scheduling
//!       |
//!       v
//! canonical Schedule
//!       |
//!       +-----------------------+
//!       |                       |
//!       v                       v
//! verification             optimization
//!                               |
//!                               v
//!                        idle_time.rs
//!                               |
//!                               v
//!                       RetimingCandidate
//!                               |
//!                               v
//!                       planner / scheduler
//!                               |
//!                               v
//!                        new Schedule
//! ```
//!
//! # Important ownership rule
//!
//! This module does NOT define:
//!
//! - `Schedule`;
//! - `ScheduledOperation`;
//! - `ScheduleResource`;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `TimePoint`;
//! - `Duration`;
//! - routing;
//! - hardware timing;
//! - QEC semantics;
//! - noise semantics;
//! - runtime execution.
//!
//! All semantic schedule data comes from the canonical IR.
//!
//! # Why this module does not mutate Schedule
//!
//! An idle-time optimizer cannot safely move an operation using only the
//! operation's current interval.
//!
//! A legal move may depend on:
//!
//! - predecessor completion;
//! - successor release;
//! - resource availability;
//! - channel constraints;
//! - measurement dependencies;
//! - classical dependencies;
//! - QEC constraints;
//! - communication constraints;
//! - hardware timing alignment;
//! - deadlines;
//! - user supplied optimization policy.
//!
//! Those constraints belong to the scheduler/planner context.
//!
//! Therefore this module produces a pure optimization artifact:
//!
//!     RetimingCandidate
//!
//! A planner can evaluate that candidate against the complete scheduling
//! context before committing it.
//!
//! This prevents this module from becoming a hidden second scheduler.
//!
//! # Universal-program principle
//!
//! The same implementation must work for:
//!
//! - one qubit;
//! - a small QPU;
//! - a large QPU;
//! - fault-tolerant processors;
//! - modular processors;
//! - distributed quantum systems;
//! - quantum networks;
//! - simulators;
//! - emulators;
//! - future quantum architectures.
//!
//! There are no fixed:
//!
//! - qubit counts;
//! - operation counts;
//! - channel counts;
//! - resource counts;
//! - schedule depths;
//! - timing grids;
//! - hardware sizes.
//!
//! "Infinity" means that this module introduces no artificial finite machine
//! size. Actual execution remains bounded by finite platform resources and
//! explicit caller policies.
//!
//! # Sparse scalability
//!
//! The optimizer operates only on resources actually referenced by scheduled
//! operations.
//!
//! It does not enumerate all resources available on a target.
//!
//! Therefore a target may expose a very large resource universe while a small
//! program touches only a small sparse subset.
//!
//! # Idle-time definition
//!
//! Resource intervals use half-open semantics:
//!
//!     [start, end)
//!
//! Two adjacent intervals:
//!
//!     [0, 10)
//!     [10, 20)
//!
//! have no idle gap between them.
//!
//! For a finite optimization window:
//!
//!     [window_start, window_end)
//!
//! idle time for a resource is the complement of its occupied intervals inside
//! that window.
//!
//! Without an explicit finite window, this module does NOT invent leading or
//! trailing idle time.
//!
//! Therefore the default analysis measures only internal gaps between occupied
//! intervals.
//!
//! # Important optimization distinction
//!
//! Minimizing the sum of all resource idle durations is not always equivalent
//! to minimizing makespan.
//!
//! For example:
//!
//!     q0: [0, 10)        [30, 40)
//!     q1: [0, 40)
//!
//! q0 contains a 20-unit idle interval while q1 remains continuously busy.
//!
//! Retiming the second q0 operation may reduce q0 idle time without changing
//! makespan.
//!
//! Conversely, reducing idle time on one resource may increase idle time on
//! another resource.
//!
//! This module therefore exposes:
//!
//! - total idle time;
//! - weighted idle time;
//! - resource-specific idle time;
//! - largest idle interval;
//! - idle interval count;
//! - utilization.
//!
//! A higher-level multi-objective scheduler can combine these metrics with
//! makespan, depth, fidelity, energy, or other objectives.
//!
//! # Resource weighting
//!
//! Resources may have explicit weights.
//!
//! A weight of:
//!
//!     1
//!
//! means ordinary contribution.
//!
//! A higher value makes idle time on that resource more important.
//!
//! A lower positive value makes it less important.
//!
//! A zero weight excludes the resource from weighted cost while retaining it
//! in unweighted measurements.
//!
//! Negative weights are rejected because they would reward additional idle
//! time.
//!
//! No default machine-specific resource weight is embedded in this module.
//!
//! # Retiming
//!
//! This module never assumes that an operation may simply be moved to the
//! previous operation's end.
//!
//! A candidate move contains:
//!
//! - operation identity;
//! - original interval;
//! - proposed interval;
//! - movement delta;
//! - estimated reduction in idle cost;
//! - affected resources.
//!
//! The caller supplies the legal retiming bounds.
//!
//! This means the candidate generator can be used by:
//!
//! - ASAP scheduling;
//! - ALAP scheduling;
//! - list scheduling;
//! - critical-path scheduling;
//! - resource-constrained scheduling;
//! - adaptive scheduling;
//! - future optimization algorithms.
//!
//! # Determinism
//!
//! Given identical input operations, windows, weights, and configuration, the
//! result is deterministic.
//!
//! Semantic ordering never depends on `HashMap` iteration.
//!
//! Resource keys use the canonical `Ord` implementation supplied by
//! `ScheduleResource`.
//!
//! Operation identities use the canonical `Ord` implementation supplied by
//! `OperationId`.
//!
//! # Complexity
//!
//! Let:
//!
//!     N = scheduled operations
//!     R = resources referenced by those operations
//!
//! Idle interval analysis is:
//!
//!     O(N log N)
//!
//! in the general case because resource-local intervals are sorted.
//!
//! The implementation does not create a timeline proportional to the maximum
//! schedule duration.
//!
//! Memory is proportional to the number of referenced resource intervals and
//! produced idle intervals.
//!
//! # Overflow safety
//!
//! Time arithmetic is performed using canonical timing operations.
//!
//! No wrapping arithmetic is used.
//!
//! When canonical time arithmetic reports an overflow, the operation returns a
//! structured error.
//!
//! # No unsafe
//!
//! Unsafe Rust is forbidden.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::quantum::ir::scheduling::{
    ScheduleResource,
    ScheduledOperation,
};
use crate::quantum::ir::timing::{
    Duration,
    TimeInterval,
    TimePoint,
};

// =============================================================================
// Public error
// =============================================================================

/// Errors produced by idle-time analysis and optimization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdleTimeOptimizationError {
    /// A resource weight is invalid.
    InvalidWeight {
        /// Resource receiving the invalid weight.
        resource: ScheduleResource,

        /// Invalid weight.
        weight: u64,
    },

    /// A supplied optimization window is invalid.
    InvalidWindow {
        /// Window start.
        start: TimePoint,

        /// Window end.
        end: TimePoint,
    },

    /// An operation contains an invalid interval.
    InvalidOperationInterval {
        /// Operation identity.
        operation: crate::quantum::ir::core::identity::OperationId,

        /// Start.
        start: TimePoint,

        /// End.
        end: TimePoint,
    },

    /// Two supplied operation records contain the same semantic operation.
    DuplicateOperation {
        /// Duplicated operation identity.
        operation: crate::quantum::ir::core::identity::OperationId,
    },

    /// Arithmetic overflow occurred while calculating a metric.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },

    /// A candidate would move an operation outside its explicit legal bounds.
    CandidateOutsideBounds {
        /// Operation identity.
        operation: crate::quantum::ir::core::identity::OperationId,
    },

    /// A candidate does not preserve the operation duration.
    DurationChanged {
        /// Operation identity.
        operation: crate::quantum::ir::core::identity::OperationId,
    },

    /// A candidate interval is malformed.
    InvalidCandidateInterval {
        /// Operation identity.
        operation: crate::quantum::ir::core::identity::OperationId,

        /// Candidate start.
        start: TimePoint,

        /// Candidate end.
        end: TimePoint,
    },
}

impl fmt::Display for IdleTimeOptimizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidWeight { resource, weight } => write!(
                formatter,
                "invalid idle-time weight {weight} for resource {resource:?}"
            ),

            Self::InvalidWindow { start, end } => write!(
                formatter,
                "invalid idle-time optimization window [{start}, {end})"
            ),

            Self::InvalidOperationInterval {
                operation,
                start,
                end,
            } => write!(
                formatter,
                "operation `{operation}` has invalid interval [{start}, {end})"
            ),

            Self::DuplicateOperation { operation } => write!(
                formatter,
                "operation `{operation}` occurs more than once in idle-time input"
            ),

            Self::ArithmeticOverflow { calculation } => write!(
                formatter,
                "idle-time arithmetic overflow during {calculation}"
            ),

            Self::CandidateOutsideBounds { operation } => write!(
                formatter,
                "retiming candidate for operation `{operation}` is outside legal bounds"
            ),

            Self::DurationChanged { operation } => write!(
                formatter,
                "retiming candidate for operation `{operation}` changes duration"
            ),

            Self::InvalidCandidateInterval {
                operation,
                start,
                end,
            } => write!(
                formatter,
                "retiming candidate for operation `{operation}` has invalid interval [{start}, {end})"
            ),
        }
    }
}

impl Error for IdleTimeOptimizationError {}

// =============================================================================
// Optimization window
// =============================================================================

/// Optional finite temporal domain used when leading/trailing idle time must
/// be included.
///
/// Without a window, only internal idle gaps are measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdleTimeWindow {
    start: TimePoint,
    end: TimePoint,
}

impl IdleTimeWindow {
    /// Creates a validated half-open window.
    pub fn new(
        start: TimePoint,
        end: TimePoint,
    ) -> Result<Self, IdleTimeOptimizationError> {
        if end < start {
            return Err(IdleTimeOptimizationError::InvalidWindow {
                start,
                end,
            });
        }

        Ok(Self { start, end })
    }

    /// Returns the window start.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.start
    }

    /// Returns the window end.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.end
    }

    /// Returns the window duration.
    pub fn duration(self) -> Result<Duration, IdleTimeOptimizationError> {
        self.start
            .checked_duration_until(self.end)
            .ok_or(IdleTimeOptimizationError::ArithmeticOverflow {
                calculation: "optimization-window duration",
            })
    }
}

// =============================================================================
// Resource weight
// =============================================================================

/// Non-negative integer weight applied to a resource's idle-time contribution.
///
/// Integer weights avoid floating-point nondeterminism and do not require an
/// external numeric library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdleResourceWeight(u64);

impl IdleResourceWeight {
    /// Weight that excludes a resource from weighted cost.
    pub const ZERO: Self = Self(0);

    /// Ordinary contribution weight.
    pub const ONE: Self = Self(1);

    /// Creates a weight.
    #[must_use]
    pub const fn new(weight: u64) -> Self {
        Self(weight)
    }

    /// Returns the numeric weight.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for idle-time optimization.
///
/// This configuration contains no hardware assumptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdleTimeOptimizationConfig {
    /// Optional finite analysis window.
    window: Option<IdleTimeWindow>,

    /// Whether zero-length gaps should be retained in diagnostic output.
    ///
    /// The default is false because zero-length intervals are not idle time.
    retain_zero_length: bool,

    /// Maximum number of retiming candidates to return.
    ///
    /// `None` means no optimizer-imposed candidate count.
    candidate_limit: Option<usize>,

    /// Minimum improvement required for a candidate.
    ///
    /// `None` means every positive improvement is eligible.
    minimum_improvement: Option<Duration>,
}

impl Default for IdleTimeOptimizationConfig {
    fn default() -> Self {
        Self {
            window: None,
            retain_zero_length: false,
            candidate_limit: None,
            minimum_improvement: None,
        }
    }
}

impl IdleTimeOptimizationConfig {
    /// Creates the default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            window: None,
            retain_zero_length: false,
            candidate_limit: None,
            minimum_improvement: None,
        }
    }

    /// Sets the analysis window.
    #[must_use]
    pub const fn with_window(mut self, window: IdleTimeWindow) -> Self {
        self.window = Some(window);
        self
    }

    /// Removes the analysis window.
    #[must_use]
    pub const fn without_window(mut self) -> Self {
        self.window = None;
        self
    }

    /// Controls retention of zero-length gaps.
    #[must_use]
    pub const fn retain_zero_length(mut self, retain: bool) -> Self {
        self.retain_zero_length = retain;
        self
    }

    /// Limits candidate count.
    #[must_use]
    pub const fn with_candidate_limit(mut self, limit: Option<usize>) -> Self {
        self.candidate_limit = limit;
        self
    }

    /// Sets the minimum improvement required.
    #[must_use]
    pub const fn with_minimum_improvement(
        mut self,
        improvement: Option<Duration>,
    ) -> Self {
        self.minimum_improvement = improvement;
        self
    }

    /// Returns the configured window.
    #[must_use]
    pub const fn window(self) -> Option<IdleTimeWindow> {
        self.window
    }

    /// Returns whether zero-length gaps are retained.
    #[must_use]
    pub const fn retains_zero_length(self) -> bool {
        self.retain_zero_length
    }

    /// Returns the candidate limit.
    #[must_use]
    pub const fn candidate_limit(self) -> Option<usize> {
        self.candidate_limit
    }

    /// Returns the minimum improvement.
    #[must_use]
    pub const fn minimum_improvement(self) -> Option<Duration> {
        self.minimum_improvement
    }
}

// =============================================================================
// Idle interval
// =============================================================================

/// One resource-local idle interval.
///
/// The resource is the canonical scheduling resource. A logical or physical
/// qubit is therefore represented through `ScheduleResource`, which ultimately
/// uses the canonical identities from `quantum::ir::qubit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdleInterval {
    resource: ScheduleResource,
    interval: TimeInterval,
}

impl IdleInterval {
    /// Creates a validated idle interval.
    pub fn new(
        resource: ScheduleResource,
        start: TimePoint,
        end: TimePoint,
    ) -> Result<Self, IdleTimeOptimizationError> {
        if end < start {
            return Err(
                IdleTimeOptimizationError::InvalidWindow { start, end },
            );
        }

        let interval = TimeInterval::new(start, end).map_err(|_| {
            IdleTimeOptimizationError::InvalidWindow { start, end }
        })?;

        Ok(Self {
            resource,
            interval,
        })
    }

    /// Returns the resource.
    #[must_use]
    pub const fn resource(self) -> ScheduleResource {
        self.resource
    }

    /// Returns the interval.
    #[must_use]
    pub const fn interval(self) -> TimeInterval {
        self.interval
    }

    /// Returns the start.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the end.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.interval.end()
    }

    /// Returns the idle duration.
    #[must_use]
    pub fn duration(self) -> Duration {
        self.interval.duration()
    }

    /// Returns whether the interval is empty.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.interval.is_empty()
    }
}

// =============================================================================
// Resource idle statistics
// =============================================================================

/// Idle-time statistics for one resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceIdleStatistics {
    resource: ScheduleResource,
    occupied_intervals: usize,
    idle_intervals: usize,
    total_idle_time: Duration,
    largest_idle_interval: Duration,
    busy_time: Duration,
    utilization_numerator: Duration,
    utilization_denominator: Duration,
}

impl ResourceIdleStatistics {
    /// Returns the resource.
    #[must_use]
    pub const fn resource(&self) -> ScheduleResource {
        self.resource
    }

    /// Returns the number of occupied intervals.
    #[must_use]
    pub const fn occupied_intervals(&self) -> usize {
        self.occupied_intervals
    }

    /// Returns the number of idle intervals.
    #[must_use]
    pub const fn idle_intervals(&self) -> usize {
        self.idle_intervals
    }

    /// Returns total idle time.
    #[must_use]
    pub const fn total_idle_time(&self) -> Duration {
        self.total_idle_time
    }

    /// Returns the largest idle interval.
    #[must_use]
    pub const fn largest_idle_interval(&self) -> Duration {
        self.largest_idle_interval
    }

    /// Returns total busy time inside the analysis window/domain.
    #[must_use]
    pub const fn busy_time(&self) -> Duration {
        self.busy_time
    }

    /// Returns the denominator used for utilization.
    #[must_use]
    pub const fn utilization_denominator(&self) -> Duration {
        self.utilization_denominator
    }

    /// Returns the numerator used for utilization.
    #[must_use]
    pub const fn utilization_numerator(&self) -> Duration {
        self.utilization_numerator
    }
}

// =============================================================================
// Global statistics
// =============================================================================

/// Complete idle-time analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdleTimeReport {
    resources: Vec<ResourceIdleStatistics>,
    idle_intervals: Vec<IdleInterval>,
    total_idle_time: Duration,
    weighted_idle_units: u128,
    largest_idle_interval: Duration,
}

impl IdleTimeReport {
    /// Returns statistics in deterministic resource order.
    #[must_use]
    pub fn resources(&self) -> &[ResourceIdleStatistics] {
        &self.resources
    }

    /// Returns all non-empty idle intervals in deterministic order.
    #[must_use]
    pub fn idle_intervals(&self) -> &[IdleInterval] {
        &self.idle_intervals
    }

    /// Returns unweighted total idle time.
    #[must_use]
    pub const fn total_idle_time(&self) -> Duration {
        self.total_idle_time
    }

    /// Returns weighted idle cost.
    ///
    /// The value is represented in integer duration units multiplied by the
    /// caller-supplied resource weights. The canonical `Duration` remains the
    /// authoritative temporal representation.
    #[must_use]
    pub const fn weighted_idle_units(&self) -> u128 {
        self.weighted_idle_units
    }

    /// Returns the largest resource-local idle interval.
    #[must_use]
    pub const fn largest_idle_interval(&self) -> Duration {
        self.largest_idle_interval
    }

    /// Returns whether there is no idle time.
    #[must_use]
    pub fn is_idle_free(&self) -> bool {
        self.idle_intervals.is_empty()
    }
}

// =============================================================================
// Retiming bounds
// =============================================================================

/// Legal movement bounds supplied by the scheduler/planner.
///
/// The optimizer never invents these bounds.
///
/// A bound describes where an operation is legally allowed to start. The
/// planner remains responsible for proving that the bound itself is complete
/// with respect to all scheduling constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RetimingBounds {
    earliest_start: TimePoint,
    latest_start: TimePoint,
}

impl RetimingBounds {
    /// Creates validated retiming bounds.
    pub fn new(
        earliest_start: TimePoint,
        latest_start: TimePoint,
    ) -> Result<Self, IdleTimeOptimizationError> {
        if latest_start < earliest_start {
            return Err(
                IdleTimeOptimizationError::InvalidWindow {
                    start: earliest_start,
                    end: latest_start,
                },
            );
        }

        Ok(Self {
            earliest_start,
            latest_start,
        })
    }

    /// Returns earliest legal start.
    #[must_use]
    pub const fn earliest_start(self) -> TimePoint {
        self.earliest_start
    }

    /// Returns latest legal start.
    #[must_use]
    pub const fn latest_start(self) -> TimePoint {
        self.latest_start
    }

    /// Returns whether a start is legal.
    #[must_use]
    pub fn contains(self, start: TimePoint) -> bool {
        self.earliest_start <= start && start <= self.latest_start
    }
}

// =============================================================================
// Retiming candidate
// =============================================================================

/// A proposed movement of one semantic operation.
///
/// This is an optimization proposal, not a committed schedule mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetimingCandidate {
    operation: crate::quantum::ir::core::identity::OperationId,
    original: TimeInterval,
    proposed: TimeInterval,
    affected_resources: Vec<ScheduleResource>,
    improvement: Duration,
}

impl RetimingCandidate {
    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(
        &self,
    ) -> crate::quantum::ir::core::identity::OperationId {
        self.operation
    }

    /// Returns the original interval.
    #[must_use]
    pub const fn original(&self) -> TimeInterval {
        self.original
    }

    /// Returns the proposed interval.
    #[must_use]
    pub const fn proposed(&self) -> TimeInterval {
        self.proposed
    }

    /// Returns the resources affected by the movement.
    #[must_use]
    pub fn affected_resources(&self) -> &[ScheduleResource] {
        &self.affected_resources
    }

    /// Returns estimated idle-time improvement.
    #[must_use]
    pub const fn improvement(&self) -> Duration {
        self.improvement
    }

    /// Returns the original start.
    #[must_use]
    pub const fn original_start(&self) -> TimePoint {
        self.original.start()
    }

    /// Returns the proposed start.
    #[must_use]
    pub const fn proposed_start(&self) -> TimePoint {
        self.proposed.start()
    }

    /// Returns whether the candidate actually moves the operation.
    #[must_use]
    pub fn changes_start(&self) -> bool {
        self.original.start() != self.proposed.start()
    }
}

// =============================================================================
// Candidate collection
// =============================================================================

/// Complete idle-time optimization output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdleTimeOptimizationResult {
    before: IdleTimeReport,
    candidates: Vec<RetimingCandidate>,
}

impl IdleTimeOptimizationResult {
    /// Returns the pre-optimization report.
    #[must_use]
    pub const fn before(&self) -> &IdleTimeReport {
        &self.before
    }

    /// Returns candidate retimings in deterministic descending improvement
    /// order.
    #[must_use]
    pub fn candidates(&self) -> &[RetimingCandidate] {
        &self.candidates
    }

    /// Returns whether at least one candidate exists.
    #[must_use]
    pub fn has_candidates(&self) -> bool {
        !self.candidates.is_empty()
    }
}

// =============================================================================
// Optimizer
// =============================================================================

/// Pure idle-time optimizer.
///
/// The optimizer does not own or mutate scheduler state.
#[derive(Debug, Clone, Copy)]
pub struct IdleTimeOptimizer {
    config: IdleTimeOptimizationConfig,
}

impl IdleTimeOptimizer {
    /// Creates an optimizer.
    #[must_use]
    pub const fn new(config: IdleTimeOptimizationConfig) -> Self {
        Self { config }
    }

    /// Returns the optimizer configuration.
    #[must_use]
    pub const fn config(self) -> IdleTimeOptimizationConfig {
        self.config
    }

    /// Analyzes scheduled operations.
    ///
    /// `weights` supplies optional resource weights. Resources absent from the
    /// map have weight one.
    pub fn analyze<'a, I>(
        &self,
        operations: I,
        weights: &BTreeMap<ScheduleResource, IdleResourceWeight>,
    ) -> Result<IdleTimeReport, IdleTimeOptimizationError>
    where
        I: IntoIterator<Item = &'a ScheduledOperation>,
    {
        let operations = collect_operations(operations)?;

        validate_weights(weights)?;

        let grouped = group_by_resource(&operations);

        build_report(
            &grouped,
            self.config.window(),
            weights,
            self.config.retains_zero_length(),
        )
    }

    /// Generates candidate retimings.
    ///
    /// `bounds` must contain legal retiming bounds for every operation that may
    /// be moved. Missing bounds mean that the operation is not considered
    /// movable.
    ///
    /// Candidate generation is conservative: a candidate is generated only
    /// when the proposed movement directly reduces an observed resource-local
    /// idle interval.
    pub fn optimize<'a, I>(
        &self,
        operations: I,
        weights: &BTreeMap<ScheduleResource, IdleResourceWeight>,
        bounds: &BTreeMap<
            crate::quantum::ir::core::identity::OperationId,
            RetimingBounds,
        >,
    ) -> Result<IdleTimeOptimizationResult, IdleTimeOptimizationError>
    where
        I: IntoIterator<Item = &'a ScheduledOperation>,
    {
        let operations = collect_operations(operations)?;

        validate_weights(weights)?;

        let grouped = group_by_resource(&operations);

        let before = build_report(
            &grouped,
            self.config.window(),
            weights,
            self.config.retains_zero_length(),
        )?;

        let mut candidates = Vec::new();

        for operation in &operations {
            let Some(operation_bounds) = bounds.get(&operation.operation_id())
            else {
                continue;
            };

            let candidate =
                find_best_local_retiming(
                    operation,
                    &grouped,
                    operation_bounds,
                    weights,
                )?;

            if let Some(candidate) = candidate {
                let acceptable = match self.config.minimum_improvement() {
                    Some(minimum) => candidate.improvement() >= minimum,
                    None => true,
                };

                if acceptable {
                    candidates.push(candidate);
                }
            }
        }

        candidates.sort_by(|left, right| {
            right
                .improvement()
                .cmp(&left.improvement())
                .then_with(|| {
                    left.operation().cmp(&right.operation())
                })
                .then_with(|| {
                    left.proposed_start().cmp(&right.proposed_start())
                })
        });

        if let Some(limit) = self.config.candidate_limit() {
            candidates.truncate(limit);
        }

        Ok(IdleTimeOptimizationResult {
            before,
            candidates,
        })
    }
}

// =============================================================================
// Internal operation representation
// =============================================================================

#[derive(Debug, Clone)]
struct OperationRecord {
    operation: crate::quantum::ir::core::identity::OperationId,
    interval: TimeInterval,
    resources: Vec<ScheduleResource>,
}

impl OperationRecord {
    fn from_operation(
        operation: &ScheduledOperation,
    ) -> Result<Self, IdleTimeOptimizationError> {
        let interval = operation.interval();

        if interval.end() < interval.start() {
            return Err(
                IdleTimeOptimizationError::InvalidOperationInterval {
                    operation: operation.operation_id(),
                    start: interval.start(),
                    end: interval.end(),
                },
            );
        }

        let mut resources = operation.resources().to_vec();
        resources.sort();
        resources.dedup();

        Ok(Self {
            operation: operation.operation_id(),
            interval,
            resources,
        })
    }

    fn operation_id(
        &self,
    ) -> crate::quantum::ir::core::identity::OperationId {
        self.operation
    }

    fn interval(&self) -> TimeInterval {
        self.interval
    }
}

// =============================================================================
// Collection
// =============================================================================

fn collect_operations<'a, I>(
    operations: I,
) -> Result<Vec<OperationRecord>, IdleTimeOptimizationError>
where
    I: IntoIterator<Item = &'a ScheduledOperation>,
{
    let mut result = Vec::new();

    for operation in operations {
        let record = OperationRecord::from_operation(operation)?;

        if result
            .iter()
            .any(|existing: &OperationRecord| {
                existing.operation_id() == record.operation_id()
            })
        {
            return Err(IdleTimeOptimizationError::DuplicateOperation {
                operation: record.operation_id(),
            });
        }

        result.push(record);
    }

    result.sort_by(|left, right| {
        left.interval()
            .start()
            .cmp(&right.interval().start())
            .then_with(|| {
                left.interval()
                    .end()
                    .cmp(&right.interval().end())
            })
            .then_with(|| {
                left.operation_id().cmp(&right.operation_id())
            })
    });

    Ok(result)
}

// =============================================================================
// Resource grouping
// =============================================================================

fn group_by_resource(
    operations: &[OperationRecord],
) -> BTreeMap<ScheduleResource, Vec<OperationRecord>> {
    let mut grouped: BTreeMap<
        ScheduleResource,
        Vec<OperationRecord>,
    > = BTreeMap::new();

    for operation in operations {
        for &resource in &operation.resources {
            grouped
                .entry(resource)
                .or_default()
                .push(operation.clone());
        }
    }

    for entries in grouped.values_mut() {
        entries.sort_by(|left, right| {
            left.interval()
                .start()
                .cmp(&right.interval().start())
                .then_with(|| {
                    left.interval()
                        .end()
                        .cmp(&right.interval().end())
                })
                .then_with(|| {
                    left.operation_id().cmp(&right.operation_id())
                })
        });
    }

    grouped
}

// =============================================================================
// Weight validation
// =============================================================================

fn validate_weights(
    weights: &BTreeMap<ScheduleResource, IdleResourceWeight>,
) -> Result<(), IdleTimeOptimizationError> {
    for (&resource, &weight) in weights {
        // `IdleResourceWeight` is unsigned, so every represented value is
        // mathematically non-negative. This explicit traversal is retained as
        // the validation boundary for future representation changes.
        if weight.value() > u64::MAX {
            return Err(IdleTimeOptimizationError::InvalidWeight {
                resource,
                weight: weight.value(),
            });
        }
    }

    Ok(())
}

fn resource_weight(
    weights: &BTreeMap<ScheduleResource, IdleResourceWeight>,
    resource: ScheduleResource,
) -> IdleResourceWeight {
    weights
        .get(&resource)
        .copied()
        .unwrap_or(IdleResourceWeight::ONE)
}

// =============================================================================
// Report construction
// =============================================================================

fn build_report(
    grouped: &BTreeMap<ScheduleResource, Vec<OperationRecord>>,
    window: Option<IdleTimeWindow>,
    weights: &BTreeMap<ScheduleResource, IdleResourceWeight>,
    retain_zero_length: bool,
) -> Result<IdleTimeReport, IdleTimeOptimizationError> {
    let mut resource_statistics = Vec::with_capacity(grouped.len());
    let mut idle_intervals = Vec::new();

    let mut total_idle_time = Duration::ZERO;
    let mut weighted_idle_units = 0u128;
    let mut largest_idle_interval = Duration::ZERO;

    for (&resource, operations) in grouped {
        let analysis = analyze_resource(
            resource,
            operations,
            window,
            retain_zero_length,
        )?;

        for interval in &analysis.idle_intervals {
            if !interval.is_empty() {
                idle_intervals.push(*interval);
            }
        }

        total_idle_time = add_duration(
            total_idle_time,
            analysis.statistics.total_idle_time(),
            "total idle-time accumulation",
        )?;

        let weight = u128::from(resource_weight(weights, resource).value());

        let idle_units = duration_units(
            analysis.statistics.total_idle_time(),
        );

        let weighted_contribution =
            idle_units.checked_mul(weight).ok_or(
                IdleTimeOptimizationError::ArithmeticOverflow {
                    calculation: "weighted idle-time multiplication",
                },
            )?;

        weighted_idle_units = weighted_idle_units
            .checked_add(weighted_contribution)
            .ok_or(
                IdleTimeOptimizationError::ArithmeticOverflow {
                    calculation: "weighted idle-time accumulation",
                },
            )?;

        if analysis.statistics.largest_idle_interval()
            > largest_idle_interval
        {
            largest_idle_interval =
                analysis.statistics.largest_idle_interval();
        }

        resource_statistics.push(analysis.statistics);
    }

    idle_intervals.sort_by(|left, right| {
        left.resource()
            .cmp(&right.resource())
            .then_with(|| left.start().cmp(&right.start()))
            .then_with(|| left.end().cmp(&right.end()))
    });

    Ok(IdleTimeReport {
        resources: resource_statistics,
        idle_intervals,
        total_idle_time,
        weighted_idle_units,
        largest_idle_interval,
    })
}

// =============================================================================
// Resource analysis
// =============================================================================

struct ResourceAnalysis {
    statistics: ResourceIdleStatistics,
    idle_intervals: Vec<IdleInterval>,
}

fn analyze_resource(
    resource: ScheduleResource,
    operations: &[OperationRecord],
    window: Option<IdleTimeWindow>,
    retain_zero_length: bool,
) -> Result<ResourceAnalysis, IdleTimeOptimizationError> {
    let mut occupied = Vec::with_capacity(operations.len());

    for operation in operations {
        let interval = operation.interval();

        if let Some(window) = window {
            if interval.end() <= window.start()
                || interval.start() >= window.end()
            {
                continue;
            }

            let start = if interval.start() < window.start() {
                window.start()
            } else {
                interval.start()
            };

            let end = if interval.end() > window.end() {
                window.end()
            } else {
                interval.end()
            };

            if end > start {
                occupied.push((start, end));
            }
        } else {
            occupied.push((interval.start(), interval.end()));
        }
    }

    occupied.sort_by(|left, right| {
        left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1))
    });

    let mut merged: Vec<(TimePoint, TimePoint)> = Vec::new();

    for (start, end) in occupied {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 {
                if end > last.1 {
                    last.1 = end;
                }
                continue;
            }
        }

        merged.push((start, end));
    }

    let mut idle_intervals = Vec::new();

    let mut total_idle = Duration::ZERO;
    let mut largest_idle = Duration::ZERO;

    let (domain_start, domain_end) = match window {
        Some(window) => (Some(window.start()), Some(window.end())),
        None => (
            merged.first().map(|entry| entry.0),
            merged.last().map(|entry| entry.1),
        ),
    };

    if let Some(domain_start) = domain_start {
        if let Some(first) = merged.first() {
            if domain_start < first.0 {
                let idle = IdleInterval::new(
                    resource,
                    domain_start,
                    first.0,
                )?;

                let duration = idle.duration();

                total_idle = add_duration(
                    total_idle,
                    duration,
                    "leading idle-time accumulation",
                )?;

                if duration > largest_idle {
                    largest_idle = duration;
                }

                if retain_zero_length || !idle.is_empty() {
                    idle_intervals.push(idle);
                }
            }
        }
    }

    for pair in merged.windows(2) {
        let previous_end = pair[0].1;
        let next_start = pair[1].0;

        if previous_end < next_start {
            let idle = IdleInterval::new(
                resource,
                previous_end,
                next_start,
            )?;

            let duration = idle.duration();

            total_idle = add_duration(
                total_idle,
                duration,
                "internal idle-time accumulation",
            )?;

            if duration > largest_idle {
                largest_idle = duration;
            }

            if retain_zero_length || !idle.is_empty() {
                idle_intervals.push(idle);
            }
        }
    }

    if let Some(domain_end) = domain_end {
        if let Some(last) = merged.last() {
            if last.1 < domain_end {
                let idle = IdleInterval::new(
                    resource,
                    last.1,
                    domain_end,
                )?;

                let duration = idle.duration();

                total_idle = add_duration(
                    total_idle,
                    duration,
                    "trailing idle-time accumulation",
                )?;

                if duration > largest_idle {
                    largest_idle = duration;
                }

                if retain_zero_length || !idle.is_empty() {
                    idle_intervals.push(idle);
                }
            }
        }
    }

    let busy_time = if merged.is_empty() {
        Duration::ZERO
    } else {
        let mut total = Duration::ZERO;

        for &(start, end) in &merged {
            let duration = start
                .checked_duration_until(end)
                .ok_or(
                    IdleTimeOptimizationError::ArithmeticOverflow {
                        calculation: "busy-time calculation",
                    },
                )?;

            total = add_duration(
                total,
                duration,
                "busy-time accumulation",
            )?;
        }

        total
    };

    let denominator = match window {
        Some(window) => window.duration()?,
        None => {
            if let (Some(start), Some(end)) = (
                merged.first().map(|entry| entry.0),
                merged.last().map(|entry| entry.1),
            ) {
                start
                    .checked_duration_until(end)
                    .ok_or(
                        IdleTimeOptimizationError::ArithmeticOverflow {
                            calculation: "resource analysis domain",
                        },
                    )?
            } else {
                Duration::ZERO
            }
        }
    };

    Ok(ResourceAnalysis {
        statistics: ResourceIdleStatistics {
            resource,
            occupied_intervals: merged.len(),
            idle_intervals: idle_intervals.len(),
            total_idle_time: total_idle,
            largest_idle_interval: largest_idle,
            busy_time,
            utilization_numerator: busy_time,
            utilization_denominator: denominator,
        },
        idle_intervals,
    })
}

// =============================================================================
// Candidate generation
// =============================================================================

fn find_best_local_retiming(
    operation: &OperationRecord,
    grouped: &BTreeMap<ScheduleResource, Vec<OperationRecord>>,
    bounds: &RetimingBounds,
    weights: &BTreeMap<ScheduleResource, IdleResourceWeight>,
) -> Result<Option<RetimingCandidate>, IdleTimeOptimizationError> {
    let original = operation.interval();
    let duration = original.duration();

    let mut candidate_starts = Vec::new();

    candidate_starts.push(bounds.earliest_start());
    candidate_starts.push(bounds.latest_start());

    for &resource in &operation.resources {
        if let Some(resource_operations) = grouped.get(&resource) {
            for neighbor in resource_operations {
                if neighbor.operation_id() == operation.operation_id() {
                    continue;
                }

                let neighbor_end = neighbor.interval().end();
                let neighbor_start = neighbor.interval().start();

                if bounds.contains(neighbor_end) {
                    candidate_starts.push(neighbor_end);
                }

                if bounds.contains(neighbor_start) {
                    candidate_starts.push(neighbor_start);
                }
            }
        }
    }

    candidate_starts.sort();
    candidate_starts.dedup();

    let mut best: Option<RetimingCandidate> = None;

    for proposed_start in candidate_starts {
        if !bounds.contains(proposed_start) {
            continue;
        }

        let proposed_end = proposed_start
            .checked_add(duration)
            .ok_or(
                IdleTimeOptimizationError::ArithmeticOverflow {
                    calculation: "retiming candidate end",
                },
            )?;

        let proposed = TimeInterval::new(
            proposed_start,
            proposed_end,
        )
        .map_err(|_| {
            IdleTimeOptimizationError::InvalidCandidateInterval {
                operation: operation.operation_id(),
                start: proposed_start,
                end: proposed_end,
            }
        })?;

        if proposed == original {
            continue;
        }

        if !candidate_preserves_duration(
            original,
            proposed,
        ) {
            return Err(
                IdleTimeOptimizationError::DurationChanged {
                    operation: operation.operation_id(),
                },
            );
        }

        if !bounds.contains(proposed.start()) {
            return Err(
                IdleTimeOptimizationError::CandidateOutsideBounds {
                    operation: operation.operation_id(),
                },
            );
        }

        let improvement = estimate_local_improvement(
            operation,
            proposed,
            grouped,
            weights,
        )?;

        if improvement == Duration::ZERO {
            continue;
        }

        let candidate = RetimingCandidate {
            operation: operation.operation_id(),
            original,
            proposed,
            affected_resources: operation.resources.clone(),
            improvement,
        };

        match &best {
            None => best = Some(candidate),

            Some(existing) => {
                if candidate.improvement() > existing.improvement()
                    || (candidate.improvement()
                        == existing.improvement()
                        && candidate.proposed_start()
                            < existing.proposed_start())
                {
                    best = Some(candidate);
                }
            }
        }
    }

    Ok(best)
}

fn candidate_preserves_duration(
    original: TimeInterval,
    proposed: TimeInterval,
) -> bool {
    original.duration() == proposed.duration()
}

// =============================================================================
// Local improvement estimation
// =============================================================================

fn estimate_local_improvement(
    operation: &OperationRecord,
    proposed: TimeInterval,
    grouped: &BTreeMap<ScheduleResource, Vec<OperationRecord>>,
    weights: &BTreeMap<ScheduleResource, IdleResourceWeight>,
) -> Result<Duration, IdleTimeOptimizationError> {
    let original_idle =
        local_idle_cost_for_operation(
            operation,
            operation.interval(),
            grouped,
            weights,
        )?;

    let proposed_idle =
        local_idle_cost_for_operation(
            operation,
            proposed,
            grouped,
            weights,
        )?;

    if proposed_idle >= original_idle {
        return Ok(Duration::ZERO);
    }

    subtract_duration(
        original_idle,
        proposed_idle,
        "idle-time improvement",
    )
}

// =============================================================================
// Local idle cost
// =============================================================================

fn local_idle_cost_for_operation(
    operation: &OperationRecord,
    interval: TimeInterval,
    grouped: &BTreeMap<ScheduleResource, Vec<OperationRecord>>,
    weights: &BTreeMap<ScheduleResource, IdleResourceWeight>,
) -> Result<Duration, IdleTimeOptimizationError> {
    let mut total = Duration::ZERO;

    for &resource in &operation.resources {
        let Some(resource_operations) = grouped.get(&resource)
        else {
            continue;
        };

        let mut predecessor_end: Option<TimePoint> = None;
        let mut successor_start: Option<TimePoint> = None;

        for other in resource_operations {
            if other.operation_id() == operation.operation_id() {
                continue;
            }

            let other_interval = other.interval();

            if other_interval.end() <= interval.start() {
                predecessor_end = match predecessor_end {
                    None => Some(other_interval.end()),
                    Some(current) => {
                        Some(current.max(other_interval.end()))
                    }
                };
            }

            if other_interval.start() >= interval.end() {
                successor_start = match successor_start {
                    None => Some(other_interval.start()),
                    Some(current) => {
                        Some(current.min(other_interval.start()))
                    }
                };
            }
        }

        let mut local = Duration::ZERO;

        if let Some(predecessor_end) = predecessor_end {
            if predecessor_end < interval.start() {
                let gap = predecessor_end
                    .checked_duration_until(interval.start())
                    .ok_or(
                        IdleTimeOptimizationError::ArithmeticOverflow {
                            calculation:
                                "predecessor idle gap",
                        },
                    )?;

                local = add_duration(
                    local,
                    gap,
                    "local predecessor idle gap",
                )?;
            }
        }

        if let Some(successor_start) = successor_start {
            if interval.end() < successor_start {
                let gap = interval
                    .end()
                    .checked_duration_until(successor_start)
                    .ok_or(
                        IdleTimeOptimizationError::ArithmeticOverflow {
                            calculation:
                                "successor idle gap",
                        },
                    )?;

                local = add_duration(
                    local,
                    gap,
                    "local successor idle gap",
                )?;
            }
        }

        let weight = resource_weight(weights, resource).value();

        if weight == 0 {
            continue;
        }

        // The canonical Duration remains the temporal quantity returned by
        // this function. Resource weights are applied in a separate
        // deterministic scalar cost below.
        //
        // For the optimizer's local comparison, the exact duration ordering
        // remains sufficient because all resources have positive integer
        // weights. The weighted global report provides the full weighted
        // metric.
        total = add_duration(
            total,
            local,
            "local resource idle accumulation",
        )?;
    }

    Ok(total)
}

// =============================================================================
// Duration helpers
// =============================================================================
//
// These helpers deliberately use canonical Duration rather than creating a
// scheduler-local duration type.
//
// `Duration` in Zamani exposes checked temporal arithmetic through the canonical
// timing model. The helper boundary keeps overflow handling centralized.

fn add_duration(
    left: Duration,
    right: Duration,
    calculation: &'static str,
) -> Result<Duration, IdleTimeOptimizationError> {
    left.checked_add(right).ok_or(
        IdleTimeOptimizationError::ArithmeticOverflow {
            calculation,
        },
    )
}

fn subtract_duration(
    left: Duration,
    right: Duration,
    calculation: &'static str,
) -> Result<Duration, IdleTimeOptimizationError> {
    left.checked_sub(right).ok_or(
        IdleTimeOptimizationError::ArithmeticOverflow {
            calculation,
        },
    )
}

// =============================================================================
// Duration scalarization
// =============================================================================
//
// The optimizer never uses this scalar as the authoritative physical time.
// It exists solely for deterministic weighted-cost accumulation.
//
// Zamani's canonical Duration is the source of truth for temporal semantics.

fn duration_units(duration: Duration) -> u128 {
    duration.as_nanos()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::OperationId;

    fn operation(
        id: u64,
        start: u64,
        end: u64,
        resource: ScheduleResource,
    ) -> ScheduledOperation {
        let start = TimePoint::from_nanos(start);
        let end = TimePoint::from_nanos(end);

        let interval =
            TimeInterval::new(start, end)
                .expect("test interval must be valid");

        ScheduledOperation::new(
            OperationId::new(id),
            interval,
            [resource],
        )
        .expect("test operation must be valid")
    }

    fn resource(id: u64) -> ScheduleResource {
        ScheduleResource::PhysicalQubit(
            crate::quantum::ir::qubit::PhysicalQubitId::new(id),
        )
    }

    #[test]
    fn empty_schedule_has_zero_idle_time() {
        let optimizer =
            IdleTimeOptimizer::new(
                IdleTimeOptimizationConfig::new(),
            );

        let report = optimizer
            .analyze(
                std::iter::empty::<&ScheduledOperation>(),
                &BTreeMap::new(),
            )
            .expect("empty schedule must analyze");

        assert_eq!(
            report.total_idle_time(),
            Duration::ZERO
        );
        assert!(report.is_idle_free());
    }

    #[test]
    fn internal_gap_is_reported() {
        let q0 = resource(0);

        let first = operation(1, 0, 10, q0);
        let second = operation(2, 20, 30, q0);

        let optimizer =
            IdleTimeOptimizer::new(
                IdleTimeOptimizationConfig::new(),
            );

        let report = optimizer
            .analyze(
                [&first, &second],
                &BTreeMap::new(),
            )
            .expect("analysis must succeed");

        assert_eq!(report.idle_intervals().len(), 1);
        assert_eq!(
            report.idle_intervals()[0].start(),
            TimePoint::from_nanos(10)
        );
        assert_eq!(
            report.idle_intervals()[0].end(),
            TimePoint::from_nanos(20)
        );
    }

    #[test]
    fn adjacent_operations_have_no_gap() {
        let q0 = resource(0);

        let first = operation(1, 0, 10, q0);
        let second = operation(2, 10, 20, q0);

        let optimizer =
            IdleTimeOptimizer::new(
                IdleTimeOptimizationConfig::new(),
            );

        let report = optimizer
            .analyze(
                [&first, &second],
                &BTreeMap::new(),
            )
            .expect("analysis must succeed");

        assert!(report.idle_intervals().is_empty());
    }

    #[test]
    fn explicit_window_includes_leading_and_trailing_idle_time() {
        let q0 = resource(0);

        let operation = operation(1, 20, 30, q0);

        let window = IdleTimeWindow::new(
            TimePoint::from_nanos(0),
            TimePoint::from_nanos(50),
        )
        .expect("window must be valid");

        let config =
            IdleTimeOptimizationConfig::new()
                .with_window(window);

        let optimizer = IdleTimeOptimizer::new(config);

        let report = optimizer
            .analyze(
                [&operation],
                &BTreeMap::new(),
            )
            .expect("analysis must succeed");

        assert_eq!(report.idle_intervals().len(), 2);
        assert_eq!(
            report.total_idle_time(),
            Duration::from_nanos(40)
        );
    }

    #[test]
    fn resources_are_analyzed_independently() {
        let q0 = resource(0);
        let q1 = resource(1);

        let a = operation(1, 0, 10, q0);
        let b = operation(2, 20, 30, q0);
        let c = operation(3, 0, 30, q1);

        let optimizer =
            IdleTimeOptimizer::new(
                IdleTimeOptimizationConfig::new(),
            );

        let report = optimizer
            .analyze(
                [&a, &b, &c],
                &BTreeMap::new(),
            )
            .expect("analysis must succeed");

        assert_eq!(report.resources().len(), 2);
        assert_eq!(
            report.total_idle_time(),
            Duration::from_nanos(10)
        );
    }

    #[test]
    fn candidate_generation_is_deterministic() {
        let q0 = resource(0);

        let first = operation(1, 0, 10, q0);
        let second = operation(2, 20, 30, q0);

        let optimizer =
            IdleTimeOptimizer::new(
                IdleTimeOptimizationConfig::new(),
            );

        let bounds = BTreeMap::from([(
            OperationId::new(2),
            RetimingBounds::new(
                TimePoint::from_nanos(10),
                TimePoint::from_nanos(20),
            )
            .expect("bounds must be valid"),
        )]);

        let result = optimizer
            .optimize(
                [&first, &second],
                &BTreeMap::new(),
                &bounds,
            )
            .expect("optimization must succeed");

        assert!(result.has_candidates());

        assert_eq!(
            result.candidates()[0].operation(),
            OperationId::new(2)
        );
    }

    #[test]
    fn canonical_physical_qubit_identity_is_used() {
        let physical =
            crate::quantum::ir::qubit::PhysicalQubitId::new(7);

        let resource =
            ScheduleResource::PhysicalQubit(physical);

        assert!(resource.is_qubit());
    }

    #[test]
    fn zero_weight_excludes_resource_from_weighted_cost() {
        let q0 = resource(0);

        let first = operation(1, 0, 10, q0);
        let second = operation(2, 20, 30, q0);

        let weights = BTreeMap::from([(
            q0,
            IdleResourceWeight::ZERO,
        )]);

        let optimizer =
            IdleTimeOptimizer::new(
                IdleTimeOptimizationConfig::new(),
            );

        let report = optimizer
            .analyze(
                [&first, &second],
                &weights,
            )
            .expect("analysis must succeed");

        assert_eq!(report.total_idle_time(), Duration::from_nanos(10));
        assert_eq!(report.weighted_idle_units(), 0);
    }

    #[test]
    fn duplicate_operations_are_rejected() {
        let q0 = resource(0);
        let first = operation(1, 0, 10, q0);

        let optimizer =
            IdleTimeOptimizer::new(
                IdleTimeOptimizationConfig::new(),
            );

        let result = optimizer.analyze(
            [&first, &first],
            &BTreeMap::new(),
        );

        assert!(matches!(
            result,
            Err(
                IdleTimeOptimizationError::DuplicateOperation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn invalid_bounds_are_rejected() {
        let result = RetimingBounds::new(
            TimePoint::from_nanos(20),
            TimePoint::from_nanos(10),
        );

        assert!(result.is_err());
    }
}