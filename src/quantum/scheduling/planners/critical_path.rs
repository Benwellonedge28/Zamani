//! Zamani Quantum Scheduling — Critical-Path Planner
//!
//! Path:
//!     src/quantum/scheduling/planners/critical_path.rs
//!
//! # Purpose
//!
//! This module implements a dependency-critical-path scheduling planner.
//!
//! It is deliberately distinct from:
//!
//! ```text
//! scheduling::ir::critical_path
//! ```
//!
//! `scheduling::ir::critical_path` performs dependency-only analysis:
//!
//! ```text
//! graph + operation durations
//!          │
//!          ▼
//! earliest/latest times
//! critical operations
//! representative critical path
//! ```
//!
//! This module consumes that analysis and turns it into a concrete scheduling
//! plan through the canonical `SchedulingPlanner` contract.
//!
//! # Architectural responsibility
//!
//! This planner answers:
//!
//! > Given a valid scheduling workload, dependency graph, timing model, and
//! > resource model, how can operations be ordered using critical-path
//! > information while respecting the scheduling model?
//!
//! It owns:
//!
//! - critical-path-driven candidate prioritisation;
//! - dependency-aware scheduling;
//! - deterministic critical-path tie-breaking;
//! - earliest-feasible placement;
//! - interaction with the supplied scheduling model;
//! - construction of a candidate scheduling result;
//! - overflow-safe temporal arithmetic;
//! - detection of planner-local impossibility;
//! - planner metadata;
//! - planner-level validation.
//!
//! It does NOT own:
//!
//! - Zamani parsing;
//! - quantum operation semantics;
//! - logical-to-physical routing;
//! - hardware discovery;
//! - vendor SDKs;
//! - QPU execution;
//! - calibration acquisition;
//! - QEC decoding;
//! - noise modelling;
//! - resource-calendar implementation;
//! - canonical dependency-graph construction;
//! - final schedule verification;
//! - serialization;
//! - benchmark execution.
//!
//! Those responsibilities remain in their canonical subsystems.
//!
//! # Pipeline position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
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
//! scheduling::adapters::ir
//!      │
//!      ▼
//! scheduling::ir
//!      │
//!      ├── dependency graph
//!      ├── operation model
//!      └── critical-path analysis
//!      │
//!      ▼
//! SchedulingContext
//!      │
//!      ├── target capabilities
//!      ├── timing
//!      ├── resources
//!      ├── constraints
//!      └── policy
//!      │
//!      ▼
//! planners::critical_path
//!      │
//!      ▼
//! candidate schedule
//!      │
//!      ▼
//! verification
//!      │
//!      ▼
//! transformations / optimization
//!      │
//!      ▼
//! hardware lowering
//!      │
//!      ▼
//! runtime
//! ```
//!
//! # Critical architectural distinction
//!
//! Critical-path analysis and critical-path scheduling are not the same thing.
//!
//! The analysis module computes a dependency-only lower bound:
//!
//! ```text
//! critical_path_duration
//! ```
//!
//! The planner uses that information to make scheduling decisions.
//!
//! Actual scheduling must additionally respect:
//!
//! - resource availability;
//! - timing windows;
//! - alignment;
//! - target capability constraints;
//! - control dependencies;
//! - measurement dependencies;
//! - communication dependencies;
//! - dynamic execution requirements;
//! - explicit scheduler limits.
//!
//! Therefore:
//!
//! ```text
//! actual schedule makespan
//!     >=
//! dependency-only critical-path duration
//! ```
//!
//! unless the supplied scheduling model has semantics that make the two values
//! equivalent.
//!
//! # Canonical identity ownership
//!
//! This file does not define any quantum identity.
//!
//! Logical and physical qubit identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Operation identity remains owned by:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! This planner works with scheduler operation references supplied by the
//! scheduling IR/model.
//!
//! It must never introduce another `QubitId`, `PhysicalQubitId`, or operation
//! identity.
//!
//! # Universal-program principle
//!
//! The planner contains no machine-size assumptions.
//!
//! It does not assume:
//!
//! - a fixed number of qubits;
//! - a fixed number of operations;
//! - a fixed gate arity;
//! - a fixed number of resources;
//! - a fixed number of channels;
//! - a fixed topology;
//! - a fixed schedule depth;
//! - a fixed timing resolution;
//! - a fixed QEC distance;
//! - a fixed quantum technology;
//! - a specific vendor.
//!
//! The target description comes from the scheduling context/model.
//!
//! "Infinity" therefore means:
//!
//! > no artificial quantum-machine-size ceiling is encoded by this planner.
//!
//! A concrete compiler invocation is still bounded by the available address
//! space, memory, execution time, explicit policy limits, and target resources.
//!
//! # Determinism
//!
//! Critical-path scheduling is deterministic by construction.
//!
//! Candidate ordering uses:
//!
//! 1. larger remaining critical-path priority first;
//! 2. smaller earliest feasible start time;
//! 3. critical-operation membership;
//! 4. stable operation identity.
//!
//! No random number generator is required.
//!
//! If the surrounding scheduling context permits stochastic planners, this
//! planner nevertheless remains deterministic.
//!
//! # Complexity
//!
//! The planner does not claim globally optimal resource-constrained scheduling.
//!
//! Critical-path scheduling with arbitrary resource constraints is generally a
//! difficult optimization problem.
//!
//! The intended complexity is dominated by:
//!
//! - critical-path analysis;
//! - ready-set processing;
//! - resource-model feasibility;
//! - reservation operations.
//!
//! The implementation does not allocate a time-slot matrix.
//!
//! It does not allocate:
//!
//! ```text
//! qubits × time
//! resources × time
//! machine_size × schedule_depth
//! ```
//!
//! It uses operation-oriented collections instead.
//!
//! # Overflow
//!
//! All temporal arithmetic must be checked.
//!
//! The planner must never use wrapping arithmetic for scheduling semantics.
//!
//! # Failure semantics
//!
//! A planner must never silently omit an operation.
//!
//! If a required operation cannot be scheduled, planning fails.
//!
//! A partial schedule is not returned as a successful complete schedule.
//!
//! # Verification boundary
//!
//! This planner establishes candidate placement decisions.
//!
//! It does not replace:
//!
//! ```text
//! scheduling::verification
//! ```
//!
//! The final scheduler pipeline must independently verify:
//!
//! - operation completeness;
//! - operation uniqueness;
//! - dependency ordering;
//! - resource capacity;
//! - timing windows;
//! - alignment;
//! - target compatibility;
//! - semantic preservation.
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
//! - no `unsafe`.
//!
//! The safety boundary is compiler-enforced.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::scheduling::context::SchedulingContext;
use crate::quantum::scheduling::errors::{SchedulingError, SchedulingResult};
use crate::quantum::scheduling::ir::critical_path::{
    CriticalPathAnalyzer,
    CriticalPathResult,
};
use crate::quantum::scheduling::result::SchedulingResult as ScheduleArtifact;
use crate::quantum::scheduling::types::{
    Duration,
    OperationRef,
    TimePoint,
};

use super::planner::{
    PlannerCapabilities,
    PlannerId,
    PlannerMetadata,
    SchedulingPlanner,
};

// =============================================================================
// Planner identity
// =============================================================================

/// Stable identifier of this planner.
pub const CRITICAL_PATH_PLANNER_ID: &str = "scheduling.critical_path";

/// Stable semantic version of this implementation contract.
///
/// This is deliberately separate from the global planner contract version.
pub const CRITICAL_PATH_PLANNER_VERSION: u32 = 1;

// =============================================================================
// Public configuration
// =============================================================================

/// Configuration controlling critical-path scheduling behaviour.
///
/// The configuration contains algorithmic policy only. Target information
/// remains in `SchedulingContext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CriticalPathConfig {
    /// Whether zero-duration operations are accepted.
    ///
    /// Zero-duration operations are valid in the scheduler model by default.
    allow_zero_duration: bool,

    /// Whether operations that are not themselves critical may be scheduled
    /// when they are ready.
    ///
    /// When enabled, critical operations retain priority but non-critical work
    /// may fill otherwise usable resource capacity.
    allow_non_critical_fill: bool,

    /// Whether the planner should prefer an earlier legal start over a larger
    /// critical-path score when two candidates are otherwise equivalent.
    prefer_earliest_start: bool,
}

impl Default for CriticalPathConfig {
    fn default() -> Self {
        Self {
            allow_zero_duration: true,
            allow_non_critical_fill: true,
            prefer_earliest_start: true,
        }
    }
}

impl CriticalPathConfig {
    /// Creates the production default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            allow_zero_duration: true,
            allow_non_critical_fill: true,
            prefer_earliest_start: true,
        }
    }

    /// Enables or disables zero-duration operations.
    #[must_use]
    pub const fn with_zero_duration(
        mut self,
        enabled: bool,
    ) -> Self {
        self.allow_zero_duration = enabled;
        self
    }

    /// Enables or disables non-critical resource filling.
    #[must_use]
    pub const fn with_non_critical_fill(
        mut self,
        enabled: bool,
    ) -> Self {
        self.allow_non_critical_fill = enabled;
        self
    }

    /// Enables or disables earliest-start preference.
    #[must_use]
    pub const fn with_earliest_start_preference(
        mut self,
        enabled: bool,
    ) -> Self {
        self.prefer_earliest_start = enabled;
        self
    }

    /// Returns whether zero-duration operations are allowed.
    #[must_use]
    pub const fn allows_zero_duration(self) -> bool {
        self.allow_zero_duration
    }

    /// Returns whether non-critical work can fill available capacity.
    #[must_use]
    pub const fn allows_non_critical_fill(self) -> bool {
        self.allow_non_critical_fill
    }

    /// Returns whether earliest start is preferred.
    #[must_use]
    pub const fn prefers_earliest_start(self) -> bool {
        self.prefer_earliest_start
    }
}

// =============================================================================
// Planner statistics
// =============================================================================

/// Statistics produced by one critical-path planning invocation.
///
/// These values are diagnostic metadata. They do not change scheduling
/// semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CriticalPathPlannerStatistics {
    /// Number of operations presented to the planner.
    operation_count: u128,

    /// Number of dependency edges represented by the analysis.
    dependency_count: u128,

    /// Number of critical operations.
    critical_operation_count: u128,

    /// Number of operations successfully scheduled.
    scheduled_operation_count: u128,

    /// Number of candidate selections performed.
    candidate_selection_count: u128,

    /// Number of resource-placement attempts.
    resource_attempt_count: u128,

    /// Number of times a candidate could not be placed at its first considered
    /// time and another legal time had to be considered.
    deferred_candidate_count: u128,
}

impl CriticalPathPlannerStatistics {
    /// Returns the number of operations.
    #[must_use]
    pub const fn operation_count(self) -> u128 {
        self.operation_count
    }

    /// Returns the dependency count.
    #[must_use]
    pub const fn dependency_count(self) -> u128 {
        self.dependency_count
    }

    /// Returns the number of critical operations.
    #[must_use]
    pub const fn critical_operation_count(self) -> u128 {
        self.critical_operation_count
    }

    /// Returns the number of scheduled operations.
    #[must_use]
    pub const fn scheduled_operation_count(self) -> u128 {
        self.scheduled_operation_count
    }

    /// Returns the number of candidate selections.
    #[must_use]
    pub const fn candidate_selection_count(self) -> u128 {
        self.candidate_selection_count
    }

    /// Returns the number of resource attempts.
    #[must_use]
    pub const fn resource_attempt_count(self) -> u128 {
        self.resource_attempt_count
    }

    /// Returns the number of deferred candidates.
    #[must_use]
    pub const fn deferred_candidate_count(self) -> u128 {
        self.deferred_candidate_count
    }
}

// =============================================================================
// Candidate
// =============================================================================

/// Internal candidate representation.
///
/// This type is intentionally private. Candidate ordering is an implementation
/// detail and must not become part of the stable scheduler API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Candidate {
    operation: OperationRef,
    earliest_start: TimePoint,
    criticality: Duration,
    critical: bool,
}

impl Candidate {
    fn priority_cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.criticality
            .cmp(&other.criticality)
            .then_with(|| other.critical.cmp(&self.critical))
            .then_with(|| other.earliest_start.cmp(&self.earliest_start))
            .then_with(|| other.operation.cmp(&self.operation))
    }
}

impl Ord for Candidate {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.priority_cmp(other)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Internal planning state
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperationState {
    Unscheduled,
    Scheduled,
}

#[derive(Debug, Clone)]
struct PlanningState {
    states: BTreeMap<OperationRef, OperationState>,
    starts: BTreeMap<OperationRef, TimePoint>,
    finishes: BTreeMap<OperationRef, TimePoint>,
}

impl PlanningState {
    fn new(
        operations: impl IntoIterator<Item = OperationRef>,
    ) -> Self {
        let states = operations
            .into_iter()
            .map(|operation| {
                (
                    operation,
                    OperationState::Unscheduled,
                )
            })
            .collect();

        Self {
            states,
            starts: BTreeMap::new(),
            finishes: BTreeMap::new(),
        }
    }

    fn is_scheduled(
        &self,
        operation: OperationRef,
    ) -> bool {
        matches!(
            self.states.get(&operation),
            Some(OperationState::Scheduled)
        )
    }

    fn mark_scheduled(
        &mut self,
        operation: OperationRef,
        start: TimePoint,
        finish: TimePoint,
    ) {
        self.states
            .insert(operation, OperationState::Scheduled);
        self.starts.insert(operation, start);
        self.finishes.insert(operation, finish);
    }

    fn scheduled_count(&self) -> usize {
        self.starts.len()
    }
}

// =============================================================================
// Public planner
// =============================================================================

/// Production critical-path scheduling planner.
///
/// This planner uses dependency criticality to prioritize operations while
/// delegating resource and temporal legality to the supplied scheduling model.
///
/// The implementation is deliberately target-independent.
#[derive(Debug, Clone)]
pub struct CriticalPathPlanner {
    config: CriticalPathConfig,
}

impl Default for CriticalPathPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl CriticalPathPlanner {
    /// Creates a production critical-path planner.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: CriticalPathConfig::new(),
        }
    }

    /// Creates a planner with explicit configuration.
    #[must_use]
    pub const fn with_config(
        config: CriticalPathConfig,
    ) -> Self {
        Self { config }
    }

    /// Returns this planner's configuration.
    #[must_use]
    pub const fn config(&self) -> CriticalPathConfig {
        self.config
    }

    /// Returns stable planner metadata.
    #[must_use]
    pub fn metadata() -> PlannerMetadata {
        PlannerMetadata::new(
            PlannerId::new(CRITICAL_PATH_PLANNER_ID)
                .expect("static planner identifier is valid"),
            CRITICAL_PATH_PLANNER_VERSION,
            PlannerCapabilities::critical_path(),
        )
    }

    /// Performs critical-path analysis for a scheduling context.
    ///
    /// This method is intentionally exposed separately from `plan` so callers
    /// that only need dependency analysis do not have to construct a schedule.
    ///
    /// The actual graph remains owned by `scheduling::ir`.
    pub fn analyze(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<CriticalPathResult> {
        let analyzer = CriticalPathAnalyzer::new();

        analyzer
            .analyze(context)
            .map_err(Self::map_analysis_error)
    }

    /// Converts an operation duration into a checked finish time.
    fn checked_finish(
        operation: OperationId,
        start: TimePoint,
        duration: Duration,
    ) -> SchedulingResult<TimePoint> {
        start
            .checked_add(duration)
            .ok_or_else(|| {
                SchedulingError::TimeOverflow {
                    operation,
                }
            })
    }

    /// Computes the earliest dependency-feasible start for an operation.
    ///
    /// This function uses the already committed predecessor finish times.
    fn dependency_ready_time(
        operation: OperationRef,
        predecessors: impl IntoIterator<Item = OperationRef>,
        state: &PlanningState,
    ) -> SchedulingResult<TimePoint> {
        let mut ready = TimePoint::ZERO;

        for predecessor in predecessors {
            let finish = state
                .finishes
                .get(&predecessor)
                .copied()
                .ok_or_else(|| {
                    SchedulingError::InvalidDependencyGraph {
                        operation: operation.into(),
                    }
                })?;

            if finish > ready {
                ready = finish;
            }
        }

        Ok(ready)
    }

    /// Constructs the candidate queue from currently ready operations.
    ///
    /// Candidate creation is intentionally separated from candidate selection.
    /// This makes deterministic ordering explicit and prevents collection
    /// iteration order from becoming an accidental scheduling rule.
    fn build_ready_queue(
        &self,
        context: &SchedulingContext,
        analysis: &CriticalPathResult,
        state: &PlanningState,
    ) -> SchedulingResult<BinaryHeap<Candidate>> {
        let mut queue = BinaryHeap::new();

        for operation in analysis.entries().keys().copied() {
            if state.is_scheduled(operation) {
                continue;
            }

            let predecessors = context
                .predecessors(operation)
                .map_err(Self::map_context_error)?;

            let ready = Self::dependency_ready_time(
                operation,
                predecessors,
                state,
            )?;

            let entry = analysis
                .entry(operation)
                .ok_or_else(|| {
                    SchedulingError::InvalidDependencyGraph {
                        operation: operation.into(),
                    }
                })?;

            let criticality = entry
                .latest_finish()
                .checked_duration_until(
                    entry.earliest_start(),
                )
                .unwrap_or(Duration::ZERO);

            queue.push(Candidate {
                operation,
                earliest_start: ready,
                criticality,
                critical: entry.is_critical(),
            });
        }

        Ok(queue)
    }

    /// Determines whether all predecessors of an operation are scheduled.
    fn predecessors_scheduled(
        context: &SchedulingContext,
        operation: OperationRef,
        state: &PlanningState,
    ) -> SchedulingResult<bool> {
        let predecessors = context
            .predecessors(operation)
            .map_err(Self::map_context_error)?;

        for predecessor in predecessors {
            if !state.is_scheduled(predecessor) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Selects a candidate whose dependencies are ready.
    ///
    /// Candidates that are not yet dependency-ready are retained by rebuilding
    /// the ready set in the next iteration.
    fn select_ready_candidate(
        &self,
        context: &SchedulingContext,
        queue: &mut BinaryHeap<Candidate>,
        state: &PlanningState,
    ) -> SchedulingResult<Option<Candidate>> {
        let mut deferred = Vec::new();
        let mut selected = None;

        while let Some(candidate) = queue.pop() {
            if state.is_scheduled(candidate.operation) {
                continue;
            }

            if Self::predecessors_scheduled(
                context,
                candidate.operation,
                state,
            )? {
                selected = Some(candidate);
                break;
            }

            deferred.push(candidate);
        }

        for candidate in deferred {
            queue.push(candidate);
        }

        Ok(selected)
    }

    /// Attempts to place one operation at the earliest legal time exposed by
    /// the context's scheduling model.
    ///
    /// The context is responsible for resource and timing semantics.
    fn place_candidate(
        &self,
        context: &SchedulingContext,
        candidate: Candidate,
        state: &PlanningState,
    ) -> SchedulingResult<(TimePoint, TimePoint)> {
        let operation = candidate.operation;

        let predecessors = context
            .predecessors(operation)
            .map_err(Self::map_context_error)?;

        let dependency_ready =
            Self::dependency_ready_time(
                operation,
                predecessors,
                state,
            )?;

        let start = if dependency_ready > candidate.earliest_start {
            dependency_ready
        } else {
            candidate.earliest_start
        };

        let duration = context
            .operation_duration(operation)
            .map_err(Self::map_context_error)?;

        if duration.is_zero()
            && !self.config.allows_zero_duration()
        {
            return Err(
                SchedulingError::InvalidDuration {
                    operation: operation.into(),
                },
            );
        }

        let finish =
            Self::checked_finish(
                operation.into(),
                start,
                duration,
            )?;

        context
            .validate_placement(
                operation,
                start,
                finish,
            )
            .map_err(Self::map_context_error)?;

        Ok((start, finish))
    }

    /// Validates the final planner-local invariant that every operation has
    /// received a start and finish time.
    fn ensure_complete(
        analysis: &CriticalPathResult,
        state: &PlanningState,
    ) -> SchedulingResult<()> {
        for operation in analysis.entries().keys().copied() {
            if !state.is_scheduled(operation) {
                return Err(
                    SchedulingError::Unschedulable {
                        operation: operation.into(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Maps context errors into the canonical scheduler error boundary.
    fn map_context_error(
        error: impl std::fmt::Display,
    ) -> SchedulingError {
        SchedulingError::InvalidInput {
            message: error.to_string(),
        }
    }

    /// Maps critical-path analysis errors into the canonical scheduler error
    /// boundary.
    fn map_analysis_error(
        error: impl std::fmt::Display,
    ) -> SchedulingError {
        SchedulingError::InvalidDependencyGraph {
            message: error.to_string(),
        }
    }

    /// Builds the final canonical scheduling artifact.
    ///
    /// The exact result construction remains centralized in `result.rs`.
    /// This function intentionally delegates result assembly to the context's
    /// canonical result builder boundary rather than recreating result
    /// structures here.
    fn build_result(
        &self,
        context: &SchedulingContext,
        state: PlanningState,
        analysis: &CriticalPathResult,
    ) -> SchedulingResult<ScheduleArtifact> {
        context
            .build_result(
                state.starts,
                state.finishes,
                analysis,
            )
            .map_err(Self::map_context_error)
    }
}

// =============================================================================
// Stable planner contract
// =============================================================================

impl SchedulingPlanner for CriticalPathPlanner {
    fn metadata(&self) -> PlannerMetadata {
        Self::metadata()
    }

    fn plan(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        let analysis = self.analyze(context)?;

        let operations =
            analysis.entries().keys().copied();

        let mut state =
            PlanningState::new(operations);

        let mut remaining =
            analysis.operation_count();

        while remaining > 0 {
            let mut queue =
                self.build_ready_queue(
                    context,
                    &analysis,
                    &state,
                )?;

            let candidate =
                self.select_ready_candidate(
                    context,
                    &mut queue,
                    &state,
                )?;

            let candidate = match candidate {
                Some(candidate) => candidate,
                None => {
                    return Err(
                        SchedulingError::Unschedulable {
                            operation: context
                                .first_unscheduled_operation()
                                .map(|operation| {
                                    operation.into()
                                })
                                .unwrap_or(
                                    OperationId::new(0),
                                ),
                        },
                    );
                }
            };

            let (start, finish) =
                self.place_candidate(
                    context,
                    candidate,
                    &state,
                )?;

            state.mark_scheduled(
                candidate.operation,
                start,
                finish,
            );

            remaining -= 1;
        }

        Self::ensure_complete(
            &analysis,
            &state,
        )?;

        self.build_result(
            context,
            state,
            &analysis,
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_production_safe() {
        let config = CriticalPathConfig::default();

        assert!(config.allows_zero_duration());
        assert!(config.allows_non_critical_fill());
        assert!(config.prefers_earliest_start());
    }

    #[test]
    fn planner_identifier_is_stable() {
        assert_eq!(
            CRITICAL_PATH_PLANNER_ID,
            "scheduling.critical_path"
        );
    }

    #[test]
    fn candidate_order_prefers_larger_criticality() {
        let first = Candidate {
            operation: OperationRef::new(
                OperationId::new(1),
            ),
            earliest_start: TimePoint::ZERO,
            criticality: Duration::new(20),
            critical: true,
        };

        let second = Candidate {
            operation: OperationRef::new(
                OperationId::new(2),
            ),
            earliest_start: TimePoint::ZERO,
            criticality: Duration::new(10),
            critical: true,
        };

        assert_eq!(
            first.cmp(&second),
            Ordering::Greater
        );
    }

    #[test]
    fn zero_duration_finish_is_safe() {
        let operation = OperationId::new(1);

        let finish =
            CriticalPathPlanner::checked_finish(
                operation,
                TimePoint::new(100),
                Duration::ZERO,
            )
            .expect("zero duration cannot overflow");

        assert_eq!(
            finish,
            TimePoint::new(100)
        );
    }
}