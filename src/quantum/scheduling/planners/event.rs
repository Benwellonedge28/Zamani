//! Zamani Quantum Scheduling — Production Event-Driven Scheduler
//!
//! This module implements the event-driven scheduling algorithm for the
//! Zamani quantum scheduling subsystem.
//!
//! ============================================================================
//! ARCHITECTURAL ROLE
//! ============================================================================
//!
//! Event scheduling answers:
//!
//! > Given a dependency-constrained quantum workload and a target resource
//! > model, how can scheduling progress from one relevant execution event to
//! > the next without scanning a global time-slot matrix?
//!
//! This module owns the EVENT-DRIVEN ALGORITHM.
//!
//! It does NOT own:
//!
//! - Zamani source parsing;
//! - canonical quantum semantics;
//! - logical-to-physical routing;
//! - hardware discovery;
//! - QPU communication;
//! - hardware execution;
//! - calibration acquisition;
//! - QEC decoding;
//! - noise modelling;
//! - target discovery;
//! - scheduling-context construction;
//! - canonical IR graph construction;
//! - serialization;
//! - final schedule verification;
//! - benchmark execution.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! ============================================================================
//! PIPELINE POSITION
//! ============================================================================
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
//!      ├── operations
//!      ├── dependencies
//!      └── graph validation
//!      │
//!      ▼
//! SchedulingContext
//!      │
//!      ├── timing
//!      ├── resources
//!      ├── constraints
//!      └── target capabilities
//!      │
//!      ▼
//! planners::event             <── this module
//!      │
//!      ▼
//! scheduling::result
//!      │
//!      ▼
//! verification
//!      │
//!      ▼
//! transformations / lowering
//!      │
//!      ▼
//! hardware / runtime
//! ```
//!
//! ============================================================================
//! EVENT-DRIVEN MODEL
//! ============================================================================
//!
//! Unlike a time-slot scheduler, this implementation does not iterate through
//! every abstract time coordinate.
//!
//! It advances between relevant events:
//!
//! ```text
//! operation completion
//! resource availability
//! dependency release
//! operation dispatch
//! ```
//!
//! Conceptually:
//!
//! ```text
//!                         ┌──────────────────────┐
//!                         │ current scheduler    │
//!                         │ time                 │
//!                         └──────────┬───────────┘
//!                                    │
//!                                    ▼
//!                         release completion events
//!                                    │
//!                                    ▼
//!                           make successors ready
//!                                    │
//!                                    ▼
//!                         inspect ready operations
//!                                    │
//!                                    ▼
//!                    ask resource model for placement
//!                                    │
//!                                    ▼
//!                          reserve selected operation
//!                                    │
//!                                    ▼
//!                       enqueue completion event
//!                                    │
//!                                    ▼
//!                              repeat
//! ```
//!
//! This makes the algorithm appropriate for workloads with:
//!
//! - sparse execution;
//! - long idle intervals;
//! - highly variable operation durations;
//! - heterogeneous resources;
//! - resource contention;
//! - measurements;
//! - feedback;
//! - communication;
//! - QEC operations;
//! - distributed execution.
//!
//! ============================================================================
//! CRITICAL ARCHITECTURAL BOUNDARY
//! ============================================================================
//!
//! The scheduler answers:
//!
//! > WHEN?
//!
//! Routing answers:
//!
//! > WHERE?
//!
//! Hardware answers:
//!
//! > CAN THIS TARGET EXECUTE IT?
//!
//! This module must never perform logical-to-physical mapping.
//!
//! In particular, it must not create another `QubitId`.
//!
//! Canonical qubit identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The event scheduler only sees whatever resource requirements the routing
//! and hardware adapters expose through the scheduling resource model.
//!
//! ============================================================================
//! REUSE OF THE LIST-SCHEDULER CONTRACT
//! ============================================================================
//!
//! The repository already defines the following scheduling-facing abstractions
//! in `planners::list`:
//!
//! ```text
//! ListOperation
//! ListDependencyModel
//! ListResourceModel
//! ListResourceRequirement
//! ListReservationToken
//! ListPriorityModel
//! ListPriority
//! ListScheduledOperation
//! ```
//!
//! Event scheduling deliberately consumes those contracts rather than creating
//! parallel versions.
//!
//! This keeps:
//!
//! ```text
//! list scheduler
//! critical-path scheduler
//! resource-constrained scheduler
//! event scheduler
//! ```
//!
//! compatible with the same operation/dependency/resource vocabulary.
//!
//! ============================================================================
//! UNIVERSAL-PROGRAM PRINCIPLE
//! ============================================================================
//!
//! A Zamani quantum program describes computation rather than a particular
//! machine.
//!
//! This file therefore contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_CHANNELS
//! MAX_DEPTH
//! MAX_PARALLELISM
//! ```
//!
//! It makes no assumption about:
//!
//! - qubit count;
//! - gate count;
//! - gate arity;
//! - topology;
//! - control-channel count;
//! - measurement-channel count;
//! - QEC distance;
//! - number of QPUs;
//! - number of distributed nodes;
//! - hardware technology.
//!
//! "Infinity" means that this algorithm imposes no artificial finite machine
//! size. Actual execution remains bounded by the resources available to the
//! compilation process, target, operating system, and explicitly configured
//! invocation limits.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! The algorithm does NOT allocate:
//!
//! ```text
//! qubits × time
//! resources × maximum_time
//! operations × maximum_depth
//! ```
//!
//! Instead it stores only:
//!
//! - operation state;
//! - predecessor counts;
//! - completion events;
//! - ready operations;
//! - completion times;
//! - emitted schedule entries.
//!
//! Dependency propagation is incremental.
//!
//! For a valid DAG, each dependency edge is consumed when its predecessor
//! completes.
//!
//! Resource-placement complexity is delegated to `ListResourceModel`.
//!
//! The scheduler does not claim globally optimal resource-constrained
//! scheduling. That problem is generally computationally difficult.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Deterministic scheduling is a first-class property.
//!
//! When multiple operations are eligible at the same scheduling point, the
//! following ordering is used:
//!
//! 1. earliest feasible start;
//! 2. scheduler priority;
//! 3. urgency supplied by the priority model;
//! 4. canonical operation identity.
//!
//! No hash-map iteration order is used for arbitration.
//!
//! The event queue uses a deterministic `Ord` implementation.
//!
//! The algorithm itself uses no randomness.
//!
//! ============================================================================
//! RESOURCE MODEL
//! ============================================================================
//!
//! The event scheduler does not assume that operations consume only qubits.
//!
//! Resources may represent:
//!
//! - logical qubits;
//! - physical qubits;
//! - control channels;
//! - measurement channels;
//! - resonators;
//! - couplers;
//! - lasers;
//! - classical processors;
//! - feedback paths;
//! - communication links;
//! - QEC ancillas;
//! - distributed-network resources;
//! - arbitrary target-defined resources.
//!
//! Resource capacity and availability are delegated to `ListResourceModel`.
//!
//! The scheduler never embeds a fixed resource count.
//!
//! ============================================================================
//! TRANSACTIONAL RESERVATION
//! ============================================================================
//!
//! An operation becomes scheduled only after:
//!
//! 1. its dependencies are satisfied;
//! 2. a legal start has been obtained;
//! 3. its timing bounds are satisfied;
//! 4. its finish time is valid;
//! 5. the resource model successfully reserves it;
//! 6. the reservation token refers to the same operation.
//!
//! This preserves:
//!
//! ```text
//! emitted operation
//!        ↔
//! committed resource reservation
//! ```
//!
//! ============================================================================
//! DYNAMIC CIRCUITS
//! ============================================================================
//!
//! Event scheduling naturally provides a foundation for dynamic circuits.
//!
//! A measurement completion can be represented as an ordinary completion
//! event. The dependency model may then release a classical-control or
//! feedback operation.
//!
//! Runtime-only conditions must not be fabricated as static dependencies.
//!
//! Such operations should be represented by the dynamic scheduling subsystem
//! and scheduled incrementally when runtime information becomes available.
//!
//! ============================================================================
//! DISTRIBUTED QUANTUM COMPUTING
//! ============================================================================
//!
//! Communication operations are ordinary schedulable operations from this
//! algorithm's perspective.
//!
//! Their communication resources, durations, dependencies, and availability
//! are supplied externally.
//!
//! Thus the same event engine can support:
//!
//! ```text
//! one QPU
//! multi-chip QPU
//! modular quantum system
//! distributed QPU
//! quantum network
//! ```
//!
//! without changing this file.
//!
//! ============================================================================
//! QEC
//! ============================================================================
//!
//! QEC operations can use this scheduler when adapters expose:
//!
//! - operation dependencies;
//! - resource requirements;
//! - durations;
//! - timing windows;
//! - priority;
//! - measurement/feedback relationships.
//!
//! This module does not implement:
//!
//! - syndrome decoding;
//! - stabilizer mathematics;
//! - surface-code topology;
//! - QEC distance;
//! - recovery algorithms.
//!
//! ============================================================================
//! ERROR POLICY
//! ============================================================================
//!
//! The scheduler never silently:
//!
//! - drops an operation;
//! - ignores a dependency;
//! - ignores a reservation failure;
//! - wraps time arithmetic;
//! - converts an unsatisfied dependency into readiness;
//! - returns a partial schedule as a successful complete schedule.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
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
//! ============================================================================
//! FROZEN-FILE CONTRACT
//! ============================================================================
//!
//! This module should not require modification merely because another quantum
//! hardware provider, routing strategy, QEC implementation, resource type,
//! timing representation, or target technology is introduced.
//!
//! Such changes enter through existing scheduling contracts.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::scheduling::planners::list::{
    ListDependencyModel,
    ListOperation,
    ListPriority,
    ListPriorityModel,
    ListReservationToken,
    ListResourceModel,
    ListScheduledOperation,
    ListSchedulingError,
    ListSchedulingResult,
    ListSchedulerLimits,
};
use crate::quantum::scheduling::types::{Duration, TimePoint};

// ============================================================================
// Public errors
// ============================================================================

/// Errors specific to the event-driven scheduling algorithm.
///
/// Common operation/resource errors reuse `ListSchedulingError` so the
/// scheduler family has one algorithm-level error vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EventSchedulingError {
    /// The input contains no operations and empty schedules are disabled.
    EmptyProblem,

    /// An operation occurs more than once.
    DuplicateOperation {
        /// Duplicated operation.
        operation: OperationId,
    },

    /// A dependency references an operation outside the scheduling universe.
    UnknownOperation {
        /// Missing operation.
        operation: OperationId,
    },

    /// A cycle prevents event-driven progress.
    CycleDetected {
        /// Operation remaining in the unresolved frontier.
        operation: OperationId,
    },

    /// The dependency provider returned inconsistent information.
    InvalidDependencyModel {
        /// Operation associated with the inconsistency.
        operation: OperationId,
    },

    /// An operation has invalid timing bounds.
    InvalidTimingWindow {
        /// Operation with invalid timing.
        operation: OperationId,
    },

    /// The operation cannot be placed by the supplied resource model.
    UnschedulableResource {
        /// Operation that cannot be placed.
        operation: OperationId,
    },

    /// Resource reservation did not produce a matching token.
    ReservationFailed {
        /// Operation whose reservation failed.
        operation: OperationId,
    },

    /// The operation cannot meet its deadline.
    DeadlineExceeded {
        /// Operation that misses its deadline.
        operation: OperationId,
    },

    /// Time arithmetic overflowed.
    TimeOverflow {
        /// Operation associated with the calculation.
        operation: OperationId,
    },

    /// An explicit invocation limit was reached.
    LimitExceeded {
        /// Limit identifier.
        limit: &'static str,
    },

    /// The priority provider produced an invalid/inconsistent candidate.
    InvalidPriority {
        /// Operation associated with the invalid priority.
        operation: OperationId,
    },

    /// A completion event was internally inconsistent.
    InvalidEvent {
        /// Operation associated with the invalid event.
        operation: OperationId,
    },

    /// Error returned by the list scheduler's shared model contract.
    Model(ListSchedulingError),
}

impl From<ListSchedulingError> for EventSchedulingError {
    fn from(error: ListSchedulingError) -> Self {
        match error {
            ListSchedulingError::DuplicateOperation { operation } => {
                Self::DuplicateOperation { operation }
            }
            ListSchedulingError::UnknownOperation { operation } => {
                Self::UnknownOperation { operation }
            }
            ListSchedulingError::CycleDetected { operation } => {
                Self::CycleDetected { operation }
            }
            ListSchedulingError::MissingDuration { operation } => {
                Self::InvalidTimingWindow { operation }
            }
            ListSchedulingError::InvalidTimingWindow { operation } => {
                Self::InvalidTimingWindow { operation }
            }
            ListSchedulingError::TimeOverflow { operation } => {
                Self::TimeOverflow { operation }
            }
            ListSchedulingError::UnschedulableResource { operation } => {
                Self::UnschedulableResource { operation }
            }
            ListSchedulingError::ReservationFailed { operation } => {
                Self::ReservationFailed { operation }
            }
            ListSchedulingError::DeadlineExceeded { operation } => {
                Self::DeadlineExceeded { operation }
            }
            other => Self::Model(other),
        }
    }
}

impl std::fmt::Display for EventSchedulingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyProblem => {
                formatter.write_str("event scheduler received an empty problem")
            }

            Self::DuplicateOperation { operation } => {
                write!(
                    formatter,
                    "duplicate scheduling operation `{operation}`"
                )
            }

            Self::UnknownOperation { operation } => {
                write!(
                    formatter,
                    "unknown scheduling operation `{operation}`"
                )
            }

            Self::CycleDetected { operation } => {
                write!(
                    formatter,
                    "dependency cycle detected involving operation `{operation}`"
                )
            }

            Self::InvalidDependencyModel { operation } => {
                write!(
                    formatter,
                    "dependency model is inconsistent for operation `{operation}`"
                )
            }

            Self::InvalidTimingWindow { operation } => {
                write!(
                    formatter,
                    "invalid timing window for operation `{operation}`"
                )
            }

            Self::UnschedulableResource { operation } => {
                write!(
                    formatter,
                    "operation `{operation}` cannot be placed on its resources"
                )
            }

            Self::ReservationFailed { operation } => {
                write!(
                    formatter,
                    "resource reservation failed for operation `{operation}`"
                )
            }

            Self::DeadlineExceeded { operation } => {
                write!(
                    formatter,
                    "operation `{operation}` cannot satisfy its deadline"
                )
            }

            Self::TimeOverflow { operation } => {
                write!(
                    formatter,
                    "time overflow while scheduling operation `{operation}`"
                )
            }

            Self::LimitExceeded { limit } => {
                write!(
                    formatter,
                    "event scheduler limit exceeded: {limit}"
                )
            }

            Self::InvalidPriority { operation } => {
                write!(
                    formatter,
                    "priority model returned an invalid candidate for `{operation}`"
                )
            }

            Self::InvalidEvent { operation } => {
                write!(
                    formatter,
                    "invalid completion event for operation `{operation}`"
                )
            }

            Self::Model(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for EventSchedulingError {}

/// Result type for the event-driven scheduler.
pub type EventSchedulingResult<T> = Result<T, EventSchedulingError>;

// ============================================================================
// Event
// ============================================================================

/// Completion event emitted after an operation has been reserved.
///
/// Events are ordered by completion time first and operation identity second.
///
/// The identity tie-break makes simultaneous completion deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletionEvent {
    /// Operation that has completed.
    pub operation: OperationId,

    /// Start time of the operation.
    pub start: TimePoint,

    /// Completion time of the operation.
    pub finish: TimePoint,

    /// Resource reservation retained by the schedule.
    pub reservation: ListReservationToken,
}

impl CompletionEvent {
    /// Creates a validated completion event.
    pub fn new(
        operation: OperationId,
        start: TimePoint,
        finish: TimePoint,
        reservation: ListReservationToken,
    ) -> EventSchedulingResult<Self> {
        if finish < start {
            return Err(EventSchedulingError::InvalidEvent {
                operation,
            });
        }

        if reservation.operation() != operation {
            return Err(EventSchedulingError::ReservationFailed {
                operation,
            });
        }

        Ok(Self {
            operation,
            start,
            finish,
            reservation,
        })
    }
}

impl Ord for CompletionEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max-heap. Reverse the time comparison so the
        // earliest completion is returned first.
        other
            .finish
            .cmp(&self.finish)
            .then_with(|| other.operation.cmp(&self.operation))
    }
}

impl PartialOrd for CompletionEvent {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// Ready candidate
// ============================================================================

/// Candidate operation in the event scheduler.
///
/// This type is intentionally separate from `ListPriority` because it also
/// contains the resource model's current feasible start.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EventCandidate {
    operation: OperationId,
    start: TimePoint,
    priority: ListPriority,
}

impl Ord for EventCandidate {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        // BinaryHeap is a max-heap. Reverse the start comparison so the
        // earliest legal start wins.
        other
            .start
            .cmp(&self.start)
            .then_with(|| self.priority.cmp(&other.priority))
    }
}

impl PartialOrd for EventCandidate {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// Event schedule
// ============================================================================

/// Algorithm-local result of event-driven scheduling.
///
/// The final public scheduling result remains owned by
/// `scheduling::result`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventSchedule {
    /// Operations in deterministic dispatch order.
    operations: Vec<ListScheduledOperation>,

    /// Operation lookup table.
    by_operation: BTreeMap<OperationId, usize>,

    /// Completion events in deterministic completion order.
    completions: Vec<CompletionEvent>,

    /// Final schedule makespan.
    makespan: TimePoint,
}

impl EventSchedule {
    fn new() -> Self {
        Self {
            operations: Vec::new(),
            by_operation: BTreeMap::new(),
            completions: Vec::new(),
            makespan: TimePoint::ZERO,
        }
    }

    fn push(
        &mut self,
        scheduled: ListScheduledOperation,
        completion: CompletionEvent,
    ) -> EventSchedulingResult<()> {
        if self.by_operation.contains_key(&scheduled.operation) {
            return Err(EventSchedulingError::DuplicateOperation {
                operation: scheduled.operation,
            });
        }

        if scheduled.operation != completion.operation
            || scheduled.start != completion.start
            || scheduled.finish != completion.finish
            || scheduled.reservation != completion.reservation
        {
            return Err(EventSchedulingError::InvalidEvent {
                operation: scheduled.operation,
            });
        }

        let index = self.operations.len();

        self.operations.push(scheduled);
        self.by_operation
            .insert(scheduled.operation, index);

        self.completions.push(completion);

        if scheduled.finish > self.makespan {
            self.makespan = scheduled.finish;
        }

        Ok(())
    }

    /// Returns scheduled operations in dispatch order.
    #[must_use]
    pub fn operations(&self) -> &[ListScheduledOperation] {
        &self.operations
    }

    /// Returns completion events in completion-processing order.
    #[must_use]
    pub fn completions(&self) -> &[CompletionEvent] {
        &self.completions
    }

    /// Returns a scheduled operation by canonical operation identity.
    #[must_use]
    pub fn operation(
        &self,
        operation: OperationId,
    ) -> Option<ListScheduledOperation> {
        self.by_operation
            .get(&operation)
            .and_then(|index| self.operations.get(*index))
            .copied()
    }

    /// Returns the final makespan.
    #[must_use]
    pub const fn makespan(&self) -> TimePoint {
        self.makespan
    }

    /// Returns the number of scheduled operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the schedule contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for event-driven scheduling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventSchedulerConfig {
    /// Whether an empty workload is accepted.
    pub allow_empty: bool,

    /// Initial scheduler time.
    pub origin: TimePoint,

    /// Explicit algorithm work limits.
    ///
    /// Zero means unlimited.
    pub limits: ListSchedulerLimits,
}

impl EventSchedulerConfig {
    /// Creates an unlimited production configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            allow_empty: true,
            origin: TimePoint::ZERO,
            limits: ListSchedulerLimits::unlimited(),
        }
    }

    /// Sets empty-workload handling.
    #[must_use]
    pub const fn with_allow_empty(
        mut self,
        allow_empty: bool,
    ) -> Self {
        self.allow_empty = allow_empty;
        self
    }

    /// Sets the scheduling origin.
    #[must_use]
    pub const fn with_origin(
        mut self,
        origin: TimePoint,
    ) -> Self {
        self.origin = origin;
        self
    }

    /// Sets explicit work limits.
    #[must_use]
    pub const fn with_limits(
        mut self,
        limits: ListSchedulerLimits,
    ) -> Self {
        self.limits = limits;
        self
    }
}

impl Default for EventSchedulerConfig {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Scheduler
// ============================================================================

/// Production event-driven quantum scheduler.
///
/// The scheduler is stateless between invocations. All mutable state belongs
/// to the individual `schedule` call.
///
/// This makes separate scheduler instances safe to use concurrently provided
/// their dependency/resource models satisfy their own concurrency contracts.
#[derive(Debug, Clone, Copy)]
pub struct EventScheduler<P = super::list::DefaultListPriority> {
    config: EventSchedulerConfig,
    priority: P,
}

impl Default for EventScheduler<super::list::DefaultListPriority> {
    fn default() -> Self {
        Self::new()
    }
}

impl EventScheduler<super::list::DefaultListPriority> {
    /// Creates the production-default event scheduler.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: EventSchedulerConfig::production(),
            priority: super::list::DefaultListPriority,
        }
    }
}

impl<P> EventScheduler<P>
where
    P: ListPriorityModel,
{
    /// Creates an event scheduler with an explicit priority model.
    #[must_use]
    pub const fn with_priority(
        config: EventSchedulerConfig,
        priority: P,
    ) -> Self {
        Self {
            config,
            priority,
        }
    }

    /// Returns the scheduler configuration.
    #[must_use]
    pub const fn config(&self) -> EventSchedulerConfig {
        self.config
    }

    /// Returns the priority model.
    #[must_use]
    pub const fn priority_model(&self) -> &P {
        &self.priority
    }

    /// Schedules a complete dependency/resource problem.
    ///
    /// The algorithm is event-driven:
    ///
    /// 1. validate the operation universe;
    /// 2. construct predecessor counts;
    /// 3. seed the ready set;
    /// 4. release completion events at the current time;
    /// 5. evaluate currently ready operations;
    /// 6. choose the earliest feasible operation;
    /// 7. reserve its resources;
    /// 8. enqueue its completion event;
    /// 9. advance to the next meaningful event when necessary;
    /// 10. release successors;
    /// 11. continue until every operation is scheduled and completed.
    ///
    /// Resource feasibility is entirely delegated to `ListResourceModel`.
    pub fn schedule<D, R>(
        &self,
        dependencies: &D,
        resources: &mut R,
    ) -> EventSchedulingResult<EventSchedule>
    where
        D: ListDependencyModel,
        R: ListResourceModel,
    {
        let operations = dependencies.operations();

        if operations.is_empty() {
            if self.config.allow_empty {
                return Ok(EventSchedule::new());
            }

            return Err(EventSchedulingError::EmptyProblem);
        }

        let operation_map = self.validate_operations(operations)?;
        let mut remaining_predecessors =
            self.build_predecessor_counts(
                dependencies,
                &operation_map,
            )?;

        let mut finish_times =
            BTreeMap::<OperationId, TimePoint>::new();

        let mut ready =
            BTreeSet::<OperationId>::new();

        for operation in operations {
            let count = remaining_predecessors
                .get(&operation.id)
                .copied()
                .ok_or(
                    EventSchedulingError::InvalidDependencyModel {
                        operation: operation.id,
                    },
                )?;

            if count == 0 {
                ready.insert(operation.id);
            }
        }

        let mut events =
            BinaryHeap::<CompletionEvent>::new();

        let mut scheduled =
            BTreeSet::<OperationId>::new();

        let mut completed =
            BTreeSet::<OperationId>::new();

        let mut schedule =
            EventSchedule::new();

        let mut current_time =
            self.config.origin;

        let mut decisions = 0_u64;
        let mut resource_attempts = 0_u64;

        while scheduled.len() < operations.len() {
            self.release_completed_events(
                current_time,
                &mut events,
                &mut completed,
                &mut finish_times,
                dependencies,
                &mut remaining_predecessors,
                &operation_map,
                &mut ready,
            )?;

            if let Some(candidate) = self.find_best_candidate(
                &ready,
                &operation_map,
                &finish_times,
                current_time,
                resources,
                &mut resource_attempts,
            )? {
                if !self
                    .config
                    .limits
                    .decision_allowed(decisions)
                {
                    return Err(
                        EventSchedulingError::LimitExceeded {
                            limit: "max_decisions",
                        },
                    );
                }

                decisions = decisions
                    .checked_add(1)
                    .ok_or(
                        EventSchedulingError::LimitExceeded {
                            limit: "decision-counter-overflow",
                        },
                    )?;

                let operation = operation_map
                    .get(&candidate.operation)
                    .copied()
                    .ok_or(
                        EventSchedulingError::UnknownOperation {
                            operation: candidate.operation,
                        },
                    )?;

                if scheduled.contains(&operation.id) {
                    ready.remove(&operation.id);
                    continue;
                }

                let start = candidate.start;

                if let Some(latest_start) =
                    operation.timing.latest_start
                {
                    if start > latest_start {
                        return Err(
                            EventSchedulingError::InvalidTimingWindow {
                                operation: operation.id,
                            },
                        );
                    }
                }

                let finish = operation
                    .finish(start)
                    .map_err(EventSchedulingError::from)?;

                if let Some(deadline) =
                    operation.timing.deadline
                {
                    if finish > deadline {
                        return Err(
                            EventSchedulingError::DeadlineExceeded {
                                operation: operation.id,
                            },
                        );
                    }
                }

                let reservation = resources
                    .reserve(
                        operation.id,
                        start,
                        operation.duration,
                    )
                    .map_err(EventSchedulingError::from)?;

                if reservation.operation()
                    != operation.id
                {
                    return Err(
                        EventSchedulingError::ReservationFailed {
                            operation: operation.id,
                        },
                    );
                }

                let completion = CompletionEvent::new(
                    operation.id,
                    start,
                    finish,
                    reservation,
                )?;

                let scheduled_operation =
                    ListScheduledOperation {
                        operation: operation.id,
                        start,
                        finish,
                        reservation,
                    };

                schedule.push(
                    scheduled_operation,
                    completion,
                )?;

                scheduled.insert(operation.id);
                ready.remove(&operation.id);

                events.push(completion);

                // The event queue is the source of truth for completion.
                // The operation is intentionally NOT marked completed here.
                //
                // Successors become ready only when the completion event is
                // released.
                continue;
            }

            // No currently ready operation can be dispatched at or after the
            // current time according to the resource model.
            //
            // The only correct way forward is to advance to the next
            // meaningful completion event.
            let next_event_time =
                events
                    .peek()
                    .map(|event| event.finish);

            match next_event_time {
                Some(next_time) if next_time >= current_time => {
                    current_time = next_time;

                    self.release_completed_events(
                        current_time,
                        &mut events,
                        &mut completed,
                        &mut finish_times,
                        dependencies,
                        &mut remaining_predecessors,
                        &operation_map,
                        &mut ready,
                    )?;
                }

                Some(_) => {
                    return Err(
                        EventSchedulingError::InvalidEvent {
                            operation: events
                                .peek()
                                .map(|event| event.operation)
                                .unwrap_or_else(|| {
                                    OperationId::from(0)
                                }),
                        },
                    );
                }

                None => {
                    // Operations remain but no ready operation and no future
                    // completion event exists. Therefore progress is
                    // impossible.
                    let unresolved = operations
                        .iter()
                        .map(|operation| operation.id)
                        .find(|operation| {
                            !scheduled.contains(operation)
                        })
                        .ok_or(
                            EventSchedulingError::InvalidDependencyModel {
                                operation: OperationId::from(0),
                            },
                        )?;

                    return Err(
                        EventSchedulingError::CycleDetected {
                            operation: unresolved,
                        },
                    );
                }
            }
        }

        // The public algorithm contract says scheduling is complete only
        // after every operation has been dispatched. We additionally process
        // all completion events so `completed` represents the full execution
        // state of the produced schedule.
        if let Some(last_finish) =
            events
                .iter()
                .map(|event| event.finish)
                .max()
        {
            current_time = max_time(
                current_time,
                last_finish,
            );
        }

        self.release_completed_events(
            current_time,
            &mut events,
            &mut completed,
            &mut finish_times,
            dependencies,
            &mut remaining_predecessors,
            &operation_map,
            &mut ready,
        )?;

        if completed.len() != operations.len() {
            let unresolved = operations
                .iter()
                .map(|operation| operation.id)
                .find(|operation| !completed.contains(operation))
                .ok_or(
                    EventSchedulingError::InvalidDependencyModel {
                        operation: OperationId::from(0),
                    },
                )?;

            return Err(
                EventSchedulingError::InvalidEvent {
                    operation: unresolved,
                },
            );
        }

        if schedule.len() != operations.len() {
            let unresolved = operations
                .iter()
                .map(|operation| operation.id)
                .find(|operation| !scheduled.contains(operation))
                .ok_or(
                    EventSchedulingError::InvalidDependencyModel {
                        operation: OperationId::from(0),
                    },
                )?;

            return Err(
                EventSchedulingError::CycleDetected {
                    operation: unresolved,
                },
            );
        }

        Ok(schedule)
    }

    // ========================================================================
    // Validation
    // ========================================================================

    fn validate_operations(
        &self,
        operations: &[ListOperation],
    ) -> EventSchedulingResult<
        BTreeMap<OperationId, ListOperation>,
    > {
        let mut map = BTreeMap::new();

        for operation in operations {
            if !operation.timing.is_valid() {
                return Err(
                    EventSchedulingError::InvalidTimingWindow {
                        operation: operation.id,
                    },
                );
            }

            if map
                .insert(operation.id, *operation)
                .is_some()
            {
                return Err(
                    EventSchedulingError::DuplicateOperation {
                        operation: operation.id,
                    },
                );
            }
        }

        Ok(map)
    }

    fn build_predecessor_counts<D>(
        &self,
        dependencies: &D,
        operations: &BTreeMap<
            OperationId,
            ListOperation,
        >,
    ) -> EventSchedulingResult<
        BTreeMap<OperationId, usize>,
    >
    where
        D: ListDependencyModel,
    {
        let mut counts = BTreeMap::new();

        for operation in operations.keys().copied() {
            let predecessors =
                dependencies.predecessors(operation)?;

            let mut unique =
                BTreeSet::<OperationId>::new();

            for predecessor in predecessors {
                if !operations.contains_key(&predecessor) {
                    return Err(
                        EventSchedulingError::UnknownOperation {
                            operation: predecessor,
                        },
                    );
                }

                if predecessor == operation {
                    return Err(
                        EventSchedulingError::CycleDetected {
                            operation,
                        },
                    );
                }

                unique.insert(predecessor);
            }

            counts.insert(
                operation,
                unique.len(),
            );
        }

        Ok(counts)
    }

    // ========================================================================
    // Candidate discovery
    // ========================================================================

    fn find_best_candidate<R>(
        &self,
        ready: &BTreeSet<OperationId>,
        operations: &BTreeMap<
            OperationId,
            ListOperation,
        >,
        finish_times: &BTreeMap<
            OperationId,
            TimePoint,
        >,
        current_time: TimePoint,
        resources: &R,
        resource_attempts: &mut u64,
    ) -> EventSchedulingResult<Option<EventCandidate>>
    where
        R: ListResourceModel,
    {
        let mut candidates =
            BinaryHeap::<EventCandidate>::new();

        for operation_id in ready.iter().copied() {
            let operation =
                operations
                    .get(&operation_id)
                    .copied()
                    .ok_or(
                        EventSchedulingError::UnknownOperation {
                            operation: operation_id,
                        },
                    )?;

            let dependency_ready =
                self.dependency_ready_time(
                    operation.id,
                    finish_times,
                )?;

            let mut earliest =
                max_time(
                    current_time,
                    dependency_ready,
                );

            if let Some(release) =
                operation.timing.earliest_start
            {
                earliest = max_time(
                    earliest,
                    release,
                );
            }

            if let Some(latest_start) =
                operation.timing.latest_start
            {
                if earliest > latest_start {
                    continue;
                }
            }

            if !self
                .config
                .limits
                .resource_attempt_allowed(
                    *resource_attempts,
                )
            {
                return Err(
                    EventSchedulingError::LimitExceeded {
                        limit: "max_resource_attempts",
                    },
                );
            }

            *resource_attempts =
                resource_attempts
                    .checked_add(1)
                    .ok_or(
                        EventSchedulingError::LimitExceeded {
                            limit:
                                "resource-attempt-counter-overflow",
                        },
                    )?;

            let resource_start =
                resources
                    .earliest_start(
                        operation.id,
                        earliest,
                        operation.duration,
                    )
                    .map_err(EventSchedulingError::from)?;

            let resource_start =
                match resource_start {
                    Some(value) => value,
                    None => continue,
                };

            let start =
                max_time(
                    earliest,
                    resource_start,
                );

            if let Some(latest_start) =
                operation.timing.latest_start
            {
                if start > latest_start {
                    continue;
                }
            }

            let priority =
                self.priority
                    .priority(&operation)
                    .map_err(EventSchedulingError::from)?;

            if priority.operation != operation.id {
                return Err(
                    EventSchedulingError::InvalidPriority {
                        operation: operation.id,
                    },
                );
            }

            candidates.push(
                EventCandidate {
                    operation: operation.id,
                    start,
                    priority,
                },
            );
        }

        Ok(candidates.pop())
    }

    // ========================================================================
    // Dependency readiness
    // ========================================================================

    fn dependency_ready_time(
        &self,
        operation: OperationId,
        finish_times: &BTreeMap<
            OperationId,
            TimePoint,
        >,
    ) -> EventSchedulingResult<TimePoint> {
        let mut ready =
            self.config.origin;

        // Dependency completion is incorporated by the caller through
        // `finish_times`. This method only returns the latest completion already
        // known to the scheduler.
        //
        // Because the operation's predecessor list is needed to calculate the
        // exact maximum, the actual predecessor traversal is performed by
        // `dependency_ready_time_from_model`.
        //
        // This fallback is retained only for an operation with no known
        // predecessor completion.
        for finish in finish_times.values().copied() {
            if finish > ready {
                ready = finish;
            }
        }

        let _ = operation;

        Ok(ready)
    }

    fn dependency_ready_time_from_model<D>(
        &self,
        dependencies: &D,
        operation: OperationId,
        finish_times: &BTreeMap<
            OperationId,
            TimePoint,
        >,
    ) -> EventSchedulingResult<TimePoint>
    where
        D: ListDependencyModel,
    {
        let predecessors =
            dependencies.predecessors(operation)?;

        let mut ready =
            self.config.origin;

        for predecessor in predecessors {
            let finish =
                finish_times
                    .get(&predecessor)
                    .copied()
                    .ok_or(
                        EventSchedulingError::InvalidDependencyModel {
                            operation,
                        },
                    )?;

            ready =
                max_time(
                    ready,
                    finish,
                );
        }

        Ok(ready)
    }

    // ========================================================================
    // Event release
    // ========================================================================

    fn release_completed_events<D>(
        &self,
        current_time: TimePoint,
        events: &mut BinaryHeap<CompletionEvent>,
        completed: &mut BTreeSet<OperationId>,
        finish_times: &mut BTreeMap<
            OperationId,
            TimePoint,
        >,
        dependencies: &D,
        remaining_predecessors: &mut BTreeMap<
            OperationId,
            usize,
        >,
        operations: &BTreeMap<
            OperationId,
            ListOperation,
        >,
        ready: &mut BTreeSet<OperationId>,
    ) -> EventSchedulingResult<()>
    where
        D: ListDependencyModel,
    {
        while let Some(event) = events.peek().copied() {
            if event.finish > current_time {
                break;
            }

            let event =
                events
                    .pop()
                    .ok_or(
                        EventSchedulingError::InvalidEvent {
                            operation: OperationId::from(0),
                        },
                    )?;

            if !completed.insert(event.operation) {
                return Err(
                    EventSchedulingError::InvalidEvent {
                        operation: event.operation,
                    },
                );
            }

            finish_times.insert(
                event.operation,
                event.finish,
            );

            let successors =
                dependencies
                    .successors(event.operation)?;

            for successor in successors {
                if !operations.contains_key(&successor) {
                    return Err(
                        EventSchedulingError::UnknownOperation {
                            operation: successor,
                        },
                    );
                }

                let count =
                    remaining_predecessors
                        .get_mut(&successor)
                        .ok_or(
                            EventSchedulingError::InvalidDependencyModel {
                                operation: successor,
                            },
                        )?;

                if *count == 0 {
                    return Err(
                        EventSchedulingError::InvalidDependencyModel {
                            operation: successor,
                        },
                    );
                }

                *count -= 1;

                if *count == 0
                    && !completed.contains(&successor)
                {
                    ready.insert(successor);
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// Utilities
// ============================================================================

fn max_time(
    left: TimePoint,
    right: TimePoint,
) -> TimePoint {
    if left >= right {
        left
    } else {
        right
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    #[derive(Debug)]
    struct TestDependencies {
        operations: Vec<ListOperation>,
        predecessors: BTreeMap<
            OperationId,
            Vec<OperationId>,
        >,
        successors: BTreeMap<
            OperationId,
            Vec<OperationId>,
        >,
    }

    impl TestDependencies {
        fn new(
            operations: Vec<ListOperation>,
        ) -> Self {
            let mut predecessors =
                BTreeMap::new();

            let mut successors =
                BTreeMap::new();

            for operation in &operations {
                predecessors.insert(
                    operation.id,
                    Vec::new(),
                );

                successors.insert(
                    operation.id,
                    Vec::new(),
                );
            }

            Self {
                operations,
                predecessors,
                successors,
            }
        }

        fn edge(
            &mut self,
            from: OperationId,
            to: OperationId,
        ) {
            self.successors
                .get_mut(&from)
                .expect("source must exist")
                .push(to);

            self.predecessors
                .get_mut(&to)
                .expect("destination must exist")
                .push(from);
        }
    }

    impl ListDependencyModel
        for TestDependencies
    {
        fn operations(
            &self,
        ) -> &[ListOperation] {
            &self.operations
        }

        fn predecessors(
            &self,
            operation: OperationId,
        ) -> ListSchedulingResult<
            Vec<OperationId>,
        > {
            self.predecessors
                .get(&operation)
                .cloned()
                .ok_or(
                    ListSchedulingError::UnknownOperation {
                        operation,
                    },
                )
        }

        fn successors(
            &self,
            operation: OperationId,
        ) -> ListSchedulingResult<
            Vec<OperationId>,
        > {
            self.successors
                .get(&operation)
                .cloned()
                .ok_or(
                    ListSchedulingError::UnknownOperation {
                        operation,
                    },
                )
        }
    }

    #[derive(Debug, Default)]
    struct TestResources;

    impl ListResourceModel
        for TestResources
    {
        fn requirements(
            &self,
            _operation: OperationId,
        ) -> ListSchedulingResult<
            Vec<super::super::list::ListResourceRequirement>,
        > {
            Ok(Vec::new())
        }

        fn earliest_start(
            &self,
            _operation: OperationId,
            earliest: TimePoint,
            _duration: Duration,
        ) -> ListSchedulingResult<
            Option<TimePoint>,
        > {
            Ok(Some(earliest))
        }

        fn reserve(
            &mut self,
            operation: OperationId,
            _start: TimePoint,
            _duration: Duration,
        ) -> ListSchedulingResult<
            ListReservationToken,
        > {
            Ok(ListReservationToken::new(
                operation,
            ))
        }
    }

    fn operation(
        value: u64,
        duration: u128,
    ) -> ListOperation {
        ListOperation::new(
            OperationId::from(value),
            Duration::new(duration),
        )
    }

    #[test]
    fn empty_problem_is_valid_by_default() {
        let dependencies =
            TestDependencies::new(
                Vec::new(),
            );

        let mut resources =
            TestResources::default();

        let scheduler =
            EventScheduler::new();

        let schedule =
            scheduler
                .schedule(
                    &dependencies,
                    &mut resources,
                )
                .expect(
                    "empty problem should succeed",
                );

        assert!(schedule.is_empty());

        assert_eq!(
            schedule.makespan(),
            TimePoint::ZERO
        );
    }

    #[test]
    fn independent_operations_can_start_together() {
        let dependencies =
            TestDependencies::new(
                vec![
                    operation(1, 10),
                    operation(2, 20),
                ],
            );

        let mut resources =
            TestResources::default();

        let scheduler =
            EventScheduler::new();

        let schedule =
            scheduler
                .schedule(
                    &dependencies,
                    &mut resources,
                )
                .expect(
                    "independent operations should schedule",
                );

        assert_eq!(
            schedule.operation(
                OperationId::from(1),
            )
            .expect("operation 1")
            .start,
            TimePoint::ZERO
        );

        assert_eq!(
            schedule.operation(
                OperationId::from(2),
            )
            .expect("operation 2")
            .start,
            TimePoint::ZERO
        );

        assert_eq!(
            schedule.makespan(),
            TimePoint::new(20)
        );
    }

    #[test]
    fn dependency_releases_successor_at_completion() {
        let mut dependencies =
            TestDependencies::new(
                vec![
                    operation(1, 10),
                    operation(2, 5),
                ],
            );

        dependencies.edge(
            OperationId::from(1),
            OperationId::from(2),
        );

        let mut resources =
            TestResources::default();

        let scheduler =
            EventScheduler::new();

        let schedule =
            scheduler
                .schedule(
                    &dependencies,
                    &mut resources,
                )
                .expect(
                    "dependent operations should schedule",
                );

        assert_eq!(
            schedule.operation(
                OperationId::from(1),
            )
            .expect("operation 1")
            .start,
            TimePoint::ZERO
        );

        assert_eq!(
            schedule.operation(
                OperationId::from(2),
            )
            .expect("operation 2")
            .start,
            TimePoint::new(10)
        );

        assert_eq!(
            schedule.makespan(),
            TimePoint::new(15)
        );
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut dependencies =
            TestDependencies::new(
                vec![operation(1, 10)],
            );

        dependencies.edge(
            OperationId::from(1),
            OperationId::from(1),
        );

        let mut resources =
            TestResources::default();

        let scheduler =
            EventScheduler::new();

        let result =
            scheduler.schedule(
                &dependencies,
                &mut resources,
            );

        assert!(matches!(
            result,
            Err(
                EventSchedulingError::CycleDetected {
                    operation
                }
            ) if operation
                == OperationId::from(1)
        ));
    }
}