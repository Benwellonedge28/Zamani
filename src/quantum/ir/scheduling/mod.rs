//! Zamani Quantum IR — Scheduling Engine
//!
//! Production-grade, deterministic, hardware-independent scheduling
//! infrastructure for the canonical Zamani Quantum IR.
//!
//! # Architectural role
//!
//! This module answers:
//!
//! > When should each already-defined IR operation occur, subject to the
//! > supplied dependency, resource, timing, and policy constraints?
//!
//! It does NOT define what an operation means.
//!
//! The ownership boundary is:
//!
//! ```text
//! quantum::ir::operation
//!     = WHAT the operation means
//!
//! quantum::ir::qubit
//!     = identity of logical/physical qubits
//!
//! quantum::ir::resource
//!     = semantic resource requirements
//!
//! quantum::ir::timing
//!     = canonical timing semantics
//!
//! quantum::ir::schedule
//!     = representation of the resulting schedule
//!
//! quantum::ir::scheduling
//!     = algorithms and orchestration that determine WHEN operations occur
//!
//! quantum::hardware
//!     = actual target capabilities and physical constraints
//!
//! backend
//!     = target-specific execution
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to targets ranging from very small quantum systems to very large,
//! distributed, or fault-tolerant systems.
//!
//! This module therefore contains:
//!
//! - no fixed qubit count;
//! - no fixed operation count;
//! - no fixed register size;
//! - no fixed topology;
//! - no fixed channel count;
//! - no fixed hardware architecture;
//! - no vendor-specific scheduling rules;
//! - no architecture-specific constants masquerading as limits.
//!
//! Concrete limits are supplied through `QuantumIrLimits`.
//!
//! `QuantumIrLimits` is a deployment/security policy, NOT a statement of the
//! maximum quantum computer Zamani can represent.
//!
//! # Important separation
//!
//! This module does not make a schedule executable merely because it can be
//! scheduled.
//!
//! A successful schedule means only:
//!
//! ```text
//! semantic operations
//!     +
//! scheduling constraints
//!     +
//! supplied resource model
//!     +
//! supplied policy
//!     ↓
//! valid temporal arrangement
//! ```
//!
//! Hardware capability validation remains downstream.
//!
//! # Canonical qubit identity
//!
//! New scheduling code MUST use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! and, when a downstream mapping has already been performed:
//!
//! ```text
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module never defines another qubit identity type.
//!
//! # Scheduling model
//!
//! Scheduling is expressed as a dependency/resource-constrained temporal
//! placement problem:
//!
//! ```text
//! task
//!   ├── operation identity
//!   ├── duration
//!   ├── resources
//!   ├── predecessors
//!   └── optional priority
//!
//!              ↓
//!
//! deterministic list scheduler
//!              ↓
//!
//! canonical Schedule
//! ```
//!
//! The scheduler implemented here is deliberately target-independent.
//!
//! Target-specific schedulers may implement the public `SchedulingStrategy`
//! trait and use the same task/schedule contracts.
//!
//! # Determinism
//!
//! Scheduling MUST be deterministic for identical inputs and policy.
//!
//! The implementation therefore:
//!
//! - uses ordered maps/sets where semantic ordering matters;
//! - never uses hash-map iteration as a scheduling decision;
//! - uses stable `OperationId` ordering as the final tie-breaker;
//! - performs dependency traversal iteratively rather than recursively;
//! - never depends on memory addresses;
//! - never depends on thread timing.
//!
//! # Scalability
//!
//! The scheduler has no semantic size ceiling.
//!
//! Its memory and execution requirements naturally scale with:
//!
//! - number of tasks;
//! - dependency edges;
//! - resource references;
//! - schedule duration;
//! - selected scheduling strategy.
//!
//! A deployment can impose explicit limits using `QuantumIrLimits`.
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_DEPTH
//! ```
//!
//! in this module.
//!
//! # No unsafe
//!
//! This module explicitly forbids unsafe Rust.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.
//!
//! # Relationship with `ir::schedule`
//!
//! `schedule.rs` owns the data structure representing a completed schedule.
//!
//! This module owns the algorithmic contract that constructs that structure.
//!
//! In particular, this module MUST NOT introduce a second `Schedule` type.
//!
//! ```text
//! scheduling/mod.rs
//!       │
//!       │ produces
//!       ▼
//! ir/schedule.rs
//!       │
//!       ▼
//! Schedule
//! ```
//!
//! # Relationship with hardware
//!
//! The scheduler may receive abstract resources such as:
//!
//! - logical qubits;
//! - physical qubits;
//! - channels;
//! - frames;
//! - other semantic resources.
//!
//! It does not discover or select hardware resources.
//!
//! Logical-to-physical mapping belongs to routing/mapping infrastructure.
//!
//! # Relationship with timing
//!
//! This module currently consumes the canonical schedule time/duration types
//! from `ir::schedule` because those types already form the stable schedule
//! representation contract.
//!
//! Future richer timing expressions may be resolved before a task reaches the
//! concrete scheduling stage.
//!
//! # Relationship with operation.rs
//!
//! `operation.rs` owns the semantic operation.
//!
//! The scheduler consumes only its stable `OperationId` and a scheduling
//! descriptor supplied by the caller.
//!
//! This prevents the scheduler from becoming coupled to every operation
//! dialect.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! frontend
//! optimization
//! routing
//! operation/resource analysis
//!        │
//!        ▼
//! SchedulingTask
//! ```
//!
//! Downstream:
//!
//! ```text
//! Schedule
//!   │
//!   ├── validation
//!   ├── pulse lowering
//!   ├── hardware lowering
//!   └── backend
//! ```
//!
//! # Design principle
//!
//! The scheduler is intentionally generic:
//!
//! ```text
//! operation semantics
//!        ≠
//! scheduling policy
//!        ≠
//! hardware topology
//!        ≠
//! backend execution
//! ```
//!
//! This prevents the scheduling layer from becoming the scalability ceiling
//! of the Zamani Quantum IR.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::identity::{IrVersion, OperationId, ScheduleId};
use super::limits::{LimitsError, QuantumIrLimits};
use super::qubit::QubitId;
use super::schedule::{
    Schedule,
    ScheduleDuration,
    ScheduleError,
    ScheduleResource,
    ScheduleTime,
    ScheduledOperation,
};

// =============================================================================
// Public result types
// =============================================================================

/// Result returned by scheduling operations.
pub type SchedulingResult<T> = Result<T, SchedulingError>;

/// Result returned by a scheduling strategy.
pub type StrategyResult<T> = Result<T, SchedulingError>;

// =============================================================================
// Scheduling error
// =============================================================================

/// Errors produced by the scheduling engine.
///
/// These errors describe scheduling concerns. Semantic IR errors remain owned
/// by the corresponding IR modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulingError {
    /// Two scheduling tasks use the same operation identity.
    DuplicateOperation {
        /// Duplicated operation.
        operation: OperationId,
    },

    /// A task references itself as a predecessor.
    SelfDependency {
        /// Operation containing the invalid dependency.
        operation: OperationId,
    },

    /// A task references a dependency that was not supplied.
    UnknownDependency {
        /// Operation containing the dependency.
        operation: OperationId,

        /// Missing predecessor.
        dependency: OperationId,
    },

    /// The dependency graph contains a cycle.
    DependencyCycle,

    /// A task has no valid operation identity.
    InvalidOperationId,

    /// A task's start/end calculation overflowed the canonical time type.
    TimeOverflow {
        /// Operation whose temporal calculation failed.
        operation: OperationId,
    },

    /// The resulting schedule violates the configured policy.
    ScheduleLimit(LimitsError),

    /// The resulting schedule failed canonical schedule validation.
    ScheduleValidation(ScheduleError),

    /// The requested strategy cannot schedule the supplied input.
    StrategyFailure {
        /// Stable diagnostic message.
        message: String,
    },

    /// The selected strategy requires information that was not supplied.
    MissingSchedulingInformation {
        /// Stable diagnostic message.
        message: String,
    },

    /// A strategy is not supported by this scheduler configuration.
    UnsupportedStrategy {
        /// Strategy name.
        strategy: String,
    },

    /// The scheduler received an invalid IR version.
    UnsupportedIrVersion {
        /// Supplied IR version.
        version: IrVersion,
    },
}

impl fmt::Display for SchedulingError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateOperation { operation } => {
                write!(
                    formatter,
                    "scheduling input contains duplicate operation `{operation}`"
                )
            }

            Self::SelfDependency { operation } => {
                write!(
                    formatter,
                    "operation `{operation}` cannot depend on itself"
                )
            }

            Self::UnknownDependency {
                operation,
                dependency,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` depends on unknown operation `{dependency}`"
                )
            }

            Self::DependencyCycle => {
                formatter.write_str(
                    "scheduling dependency graph contains a cycle",
                )
            }

            Self::InvalidOperationId => {
                formatter.write_str(
                    "scheduling task contains an invalid operation identity",
                )
            }

            Self::TimeOverflow { operation } => {
                write!(
                    formatter,
                    "scheduling time calculation overflowed for operation `{operation}`"
                )
            }

            Self::ScheduleLimit(error) => {
                write!(
                    formatter,
                    "scheduling resource limit exceeded: {error}"
                )
            }

            Self::ScheduleValidation(error) => {
                write!(
                    formatter,
                    "generated schedule failed validation: {error}"
                )
            }

            Self::StrategyFailure { message } => {
                write!(
                    formatter,
                    "scheduling strategy failed: {message}"
                )
            }

            Self::MissingSchedulingInformation { message } => {
                write!(
                    formatter,
                    "required scheduling information is missing: {message}"
                )
            }

            Self::UnsupportedStrategy { strategy } => {
                write!(
                    formatter,
                    "unsupported scheduling strategy `{strategy}`"
                )
            }

            Self::UnsupportedIrVersion { version } => {
                write!(
                    formatter,
                    "unsupported Quantum IR version `{version}`"
                )
            }
        }
    }
}

impl Error for SchedulingError {}

impl From<ScheduleError> for SchedulingError {
    fn from(error: ScheduleError) -> Self {
        Self::ScheduleValidation(error)
    }
}

// =============================================================================
// Scheduling priority
// =============================================================================

/// Deterministic scheduling priority.
///
/// The priority is advisory. It never overrides dependency or resource
/// correctness.
///
/// Custom scheduling strategies can use richer policies without changing
/// `SchedulingTask`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchedulingPriority(i64);

impl SchedulingPriority {
    /// Lowest representable priority.
    pub const MIN: Self = Self(i64::MIN);

    /// Default priority.
    pub const DEFAULT: Self = Self(0);

    /// Highest representable priority.
    pub const MAX: Self = Self(i64::MAX);

    /// Creates a priority.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the raw priority.
    #[must_use]
    pub const fn value(self) -> i64 {
        self.0
    }
}

impl Default for SchedulingPriority {
    fn default() -> Self {
        Self::DEFAULT
    }
}

// =============================================================================
// Scheduling task
// =============================================================================

/// Target-independent description of one schedulable IR operation.
///
/// `SchedulingTask` deliberately does not contain the complete `Operation`.
///
/// The operation remains owned by `quantum::ir::operation`.
///
/// This descriptor contains only information required to determine temporal
/// placement.
///
/// # Resource semantics
///
/// Every resource listed here is considered exclusively occupied for the
/// operation's `[start, end)` interval by the default list scheduler.
///
/// If a future hardware model needs shared/preemptible/resource-capacity
/// semantics, it should implement a specialized `SchedulingStrategy` rather
/// than changing the canonical operation model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingTask {
    operation: OperationId,
    duration: ScheduleDuration,
    resources: Vec<ScheduleResource>,
    predecessors: BTreeSet<OperationId>,
    priority: SchedulingPriority,
}

impl SchedulingTask {
    /// Creates a task with no resource or dependency requirements.
    #[must_use]
    pub fn new(
        operation: OperationId,
        duration: ScheduleDuration,
    ) -> Self {
        Self {
            operation,
            duration,
            resources: Vec::new(),
            predecessors: BTreeSet::new(),
            priority: SchedulingPriority::DEFAULT,
        }
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the operation duration.
    #[must_use]
    pub const fn duration(&self) -> ScheduleDuration {
        self.duration
    }

    /// Returns the scheduling priority.
    #[must_use]
    pub const fn priority(&self) -> SchedulingPriority {
        self.priority
    }

    /// Returns all resources in deterministic order.
    #[must_use]
    pub fn resources(&self) -> &[ScheduleResource] {
        &self.resources
    }

    /// Returns all predecessor operation identities.
    #[must_use]
    pub fn predecessors(&self) -> &BTreeSet<OperationId> {
        &self.predecessors
    }

    /// Sets the scheduling priority.
    #[must_use]
    pub fn with_priority(
        mut self,
        priority: SchedulingPriority,
    ) -> Self {
        self.priority = priority;
        self
    }

    /// Adds a resource requirement.
    ///
    /// Duplicate resource references are ignored.
    #[must_use]
    pub fn with_resource(
        mut self,
        resource: ScheduleResource,
    ) -> Self {
        self.add_resource(resource);
        self
    }

    /// Adds a logical-qubit resource.
    ///
    /// Uses the canonical `quantum::ir::qubit::QubitId`.
    #[must_use]
    pub fn with_logical_qubit(
        mut self,
        qubit: QubitId,
    ) -> Self {
        self.add_resource(
            ScheduleResource::LogicalQubit(qubit),
        );
        self
    }

    /// Adds a predecessor dependency.
    ///
    /// Self-dependencies are rejected later by `validate_input()` so that
    /// task construction remains cheap and composable.
    #[must_use]
    pub fn depends_on(
        mut self,
        predecessor: OperationId,
    ) -> Self {
        self.predecessors.insert(predecessor);
        self
    }

    /// Adds a resource without creating duplicates.
    pub fn add_resource(
        &mut self,
        resource: ScheduleResource,
    ) {
        if !self.resources.contains(&resource) {
            self.resources.push(resource);
            self.resources.sort();
        }
    }

    /// Returns whether the task uses the supplied resource.
    #[must_use]
    pub fn uses_resource(
        &self,
        resource: ScheduleResource,
    ) -> bool {
        self.resources.contains(&resource)
    }
}

// =============================================================================
// Scheduling input
// =============================================================================

/// Immutable collection of scheduling tasks.
///
/// `SchedulingInput` owns scheduling metadata only. It does not own or clone
/// the complete canonical program.
///
/// This makes scheduling memory proportional to the scheduling information
/// actually needed by the selected strategy.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SchedulingInput {
    tasks: BTreeMap<OperationId, SchedulingTask>,
    ir_version: IrVersion,
}

impl SchedulingInput {
    /// Creates an empty scheduling input using the current IR version.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            ir_version: IrVersion::CURRENT,
        }
    }

    /// Creates an empty input for an explicit IR version.
    #[must_use]
    pub fn with_version(
        ir_version: IrVersion,
    ) -> Self {
        Self {
            tasks: BTreeMap::new(),
            ir_version,
        }
    }

    /// Returns the IR version carried by this input.
    #[must_use]
    pub const fn ir_version(&self) -> IrVersion {
        self.ir_version
    }

    /// Returns the number of scheduling tasks.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Returns whether there are no scheduling tasks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Returns a task by operation identity.
    #[must_use]
    pub fn get(
        &self,
        operation: OperationId,
    ) -> Option<&SchedulingTask> {
        self.tasks.get(&operation)
    }

    /// Returns tasks in deterministic operation-identity order.
    pub fn tasks(
        &self,
    ) -> impl Iterator<Item = &SchedulingTask> {
        self.tasks.values()
    }

    /// Inserts a task.
    ///
    /// Duplicate identities are rejected.
    pub fn insert(
        &mut self,
        task: SchedulingTask,
    ) -> SchedulingResult<()> {
        let operation = task.operation();

        if operation.value() == 0 {
            return Err(SchedulingError::InvalidOperationId);
        }

        if self.tasks.contains_key(&operation) {
            return Err(
                SchedulingError::DuplicateOperation {
                    operation,
                },
            );
        }

        self.tasks.insert(operation, task);

        Ok(())
    }

    /// Inserts multiple tasks deterministically.
    ///
    /// If any task is invalid, the input remains unchanged.
    pub fn extend<I>(
        &mut self,
        tasks: I,
    ) -> SchedulingResult<()>
    where
        I: IntoIterator<Item = SchedulingTask>,
    {
        let collected: Vec<SchedulingTask> =
            tasks.into_iter().collect();

        let mut seen = BTreeSet::new();

        for task in &collected {
            let operation = task.operation();

            if operation.value() == 0 {
                return Err(
                    SchedulingError::InvalidOperationId,
                );
            }

            if !seen.insert(operation)
                || self.tasks.contains_key(&operation)
            {
                return Err(
                    SchedulingError::DuplicateOperation {
                        operation,
                    },
                );
            }
        }

        for task in collected {
            self.tasks.insert(
                task.operation(),
                task,
            );
        }

        Ok(())
    }

    /// Validates dependencies before scheduling.
    pub fn validate(
        &self,
    ) -> SchedulingResult<()> {
        if !self.ir_version.is_supported_by_current() {
            return Err(
                SchedulingError::UnsupportedIrVersion {
                    version: self.ir_version,
                },
            );
        }

        for task in self.tasks.values() {
            for &dependency in task.predecessors() {
                if dependency == task.operation() {
                    return Err(
                        SchedulingError::SelfDependency {
                            operation: task.operation(),
                        },
                    );
                }

                if !self.tasks.contains_key(&dependency) {
                    return Err(
                        SchedulingError::UnknownDependency {
                            operation: task.operation(),
                            dependency,
                        },
                    );
                }
            }
        }

        self.detect_cycle()
    }

    /// Detects dependency cycles iteratively.
    ///
    /// Iterative traversal is intentional: a program can contain a dependency
    /// graph deeper than the host call stack can safely support.
    fn detect_cycle(
        &self,
    ) -> SchedulingResult<()> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum VisitState {
            Visiting,
            Visited,
        }

        let mut states: BTreeMap<
            OperationId,
            VisitState,
        > = BTreeMap::new();

        for &root in self.tasks.keys() {
            if states.contains_key(&root) {
                continue;
            }

            let mut stack: Vec<(
                OperationId,
                bool,
            )> = Vec::new();

            stack.push((root, false));

            while let Some((
                operation,
                exiting,
            )) = stack.pop()
            {
                if exiting {
                    states.insert(
                        operation,
                        VisitState::Visited,
                    );
                    continue;
                }

                match states.get(&operation) {
                    Some(VisitState::Visiting) => {
                        return Err(
                            SchedulingError::DependencyCycle,
                        );
                    }

                    Some(VisitState::Visited) => {
                        continue;
                    }

                    None => {}
                }

                states.insert(
                    operation,
                    VisitState::Visiting,
                );

                stack.push((operation, true));

                if let Some(task) =
                    self.tasks.get(&operation)
                {
                    for &dependency in
                        task.predecessors().iter().rev()
                    {
                        match states.get(&dependency) {
                            Some(VisitState::Visiting) => {
                                return Err(
                                    SchedulingError::DependencyCycle,
                                );
                            }

                            Some(VisitState::Visited) => {}

                            None => {
                                stack.push((
                                    dependency,
                                    false,
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Scheduling strategy
// =============================================================================

/// Pluggable scheduling algorithm.
///
/// The default implementation below is a deterministic resource-constrained
/// ASAP list scheduler.
///
/// Future algorithms can implement this trait without changing:
///
/// - `SchedulingTask`;
/// - `SchedulingInput`;
/// - `Schedule`;
/// - downstream hardware interfaces.
pub trait SchedulingStrategy {
    /// Returns a stable strategy name.
    fn name(&self) -> &'static str;

    /// Produces a schedule from validated scheduling input.
    fn schedule(
        &self,
        input: &SchedulingInput,
        limits: &QuantumIrLimits,
        schedule_id: ScheduleId,
    ) -> SchedulingResult<Schedule>;
}

// =============================================================================
// Default list scheduling strategy
// =============================================================================

/// Deterministic resource-constrained ASAP list scheduler.
///
/// Algorithm:
///
/// 1. validate the dependency graph;
/// 2. identify ready operations;
/// 3. choose the ready operation using priority and stable identity;
/// 4. calculate the earliest legal start from predecessors;
/// 5. calculate the earliest legal start from occupied resources;
/// 6. place the operation;
/// 7. release its dependency successors;
/// 8. continue until all operations are scheduled.
///
/// The algorithm does not assume a particular quantum architecture.
///
/// It works for:
///
/// - one qubit;
/// - many logical qubits;
/// - mapped physical qubits;
/// - channels;
/// - frames;
/// - hybrid resources;
/// - distributed resource descriptors.
///
/// It is deterministic rather than pretending to be globally optimal for every
/// possible hardware scheduling problem.
#[derive(Debug, Clone, Copy, Default)]
pub struct AsSoonAsPossible;

impl AsSoonAsPossible {
    /// Creates the default deterministic ASAP strategy.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn choose_ready_task(
        &self,
        ready: &BTreeSet<OperationId>,
        input: &SchedulingInput,
    ) -> Option<OperationId> {
        ready
            .iter()
            .copied()
            .max_by(|left, right| {
                let left_priority = input
                    .get(*left)
                    .map(|task| task.priority())
                    .unwrap_or(
                        SchedulingPriority::DEFAULT,
                    );

                let right_priority = input
                    .get(*right)
                    .map(|task| task.priority())
                    .unwrap_or(
                        SchedulingPriority::DEFAULT,
                    );

                left_priority
                    .cmp(&right_priority)
                    // Lower OperationId wins the final deterministic tie.
                    .then_with(|| right.cmp(left))
            })
    }
}

impl SchedulingStrategy for AsSoonAsPossible {
    fn name(&self) -> &'static str {
        "as-soon-as-possible"
    }

    fn schedule(
        &self,
        input: &SchedulingInput,
        limits: &QuantumIrLimits,
        schedule_id: ScheduleId,
    ) -> SchedulingResult<Schedule> {
        input.validate()?;

        let mut schedule_builder =
            super::schedule::ScheduleBuilder::with_version_and_limits(
                schedule_id,
                input.ir_version(),
                limits.clone(),
            );

        if input.is_empty() {
            return schedule_builder
                .finish()
                .map_err(SchedulingError::from);
        }

        let mut successors: BTreeMap<
            OperationId,
            BTreeSet<OperationId>,
        > = BTreeMap::new();

        let mut remaining_predecessors: BTreeMap<
            OperationId,
            BTreeSet<OperationId>,
        > = BTreeMap::new();

        for task in input.tasks() {
            remaining_predecessors.insert(
                task.operation(),
                task.predecessors().clone(),
            );

            successors
                .entry(task.operation())
                .or_default();

            for &dependency in task.predecessors() {
                successors
                    .entry(dependency)
                    .or_default()
                    .insert(task.operation());
            }
        }

        let mut ready = BTreeSet::new();

        for task in input.tasks() {
            if task.predecessors().is_empty() {
                ready.insert(task.operation());
            }
        }

        let mut scheduled_ends: BTreeMap<
            OperationId,
            ScheduleTime,
        > = BTreeMap::new();

        let mut resource_available_at: BTreeMap<
            ScheduleResource,
            ScheduleTime,
        > = BTreeMap::new();

        let mut scheduled_count: usize = 0;

        while !ready.is_empty() {
            let operation = self
                .choose_ready_task(&ready, input)
                .ok_or(
                    SchedulingError::StrategyFailure {
                        message:
                            "ready-set selection unexpectedly failed"
                                .to_owned(),
                    },
                )?;

            ready.remove(&operation);

            let task = input.get(operation).ok_or(
                SchedulingError::MissingSchedulingInformation {
                    message: format!(
                        "task `{operation}` disappeared from scheduling input"
                    ),
                },
            )?;

            let mut earliest_start =
                ScheduleTime::ZERO;

            for &dependency in task.predecessors() {
                let dependency_end =
                    scheduled_ends
                        .get(&dependency)
                        .copied()
                        .ok_or(
                            SchedulingError::StrategyFailure {
                                message: format!(
                                    "dependency `{dependency}` was not scheduled before `{operation}`"
                                ),
                            },
                        )?;

                if dependency_end > earliest_start {
                    earliest_start = dependency_end;
                }
            }

            for &resource in task.resources() {
                if let Some(available) =
                    resource_available_at.get(&resource)
                {
                    if *available > earliest_start {
                        earliest_start = *available;
                    }
                }
            }

            let end = earliest_start
                .checked_add(
                    ScheduleTime::from_attoseconds(
                        task.duration().attoseconds(),
                    ),
                )
                .ok_or(
                    SchedulingError::TimeOverflow {
                        operation,
                    },
                )?;

            let scheduled =
                ScheduledOperation::new(
                    operation,
                    earliest_start,
                    task.duration(),
                );

            let scheduled =
                task.resources()
                    .iter()
                    .copied()
                    .fold(
                        scheduled,
                        |operation, resource| {
                            operation.with_resource(resource)
                        },
                    );

            schedule_builder
                .push(scheduled)
                .map_err(SchedulingError::from)?;

            for &dependency in task.predecessors() {
                schedule_builder
                    .add_dependency(
                        operation,
                        dependency,
                    )
                    .map_err(SchedulingError::from)?;
            }

            scheduled_ends.insert(
                operation,
                end,
            );

            for &resource in task.resources() {
                resource_available_at.insert(
                    resource,
                    end,
                );
            }

            scheduled_count =
                scheduled_count
                    .checked_add(1)
                    .ok_or(
                        SchedulingError::ScheduleLimit(
                            LimitsError::ArithmeticOverflow {
                                resource:
                                    super::limits::ResourceKind::ScheduledOperations,
                            },
                        ),
                    )?;

            if let Some(dependent_operations) =
                successors.get(&operation)
            {
                for &dependent in
                    dependent_operations
                {
                    let predecessors =
                        remaining_predecessors
                            .get_mut(&dependent)
                            .ok_or(
                                SchedulingError::StrategyFailure {
                                    message: format!(
                                        "missing dependency state for `{dependent}`"
                                    ),
                                },
                            )?;

                    predecessors.remove(&operation);

                    if predecessors.is_empty() {
                        ready.insert(dependent);
                    }
                }
            }
        }

        if scheduled_count != input.len() {
            return Err(
                SchedulingError::StrategyFailure {
                    message: format!(
                        "scheduler terminated after {scheduled_count} of {} operations",
                        input.len()
                    ),
                },
            );
        }

        schedule_builder
            .finish()
            .map_err(SchedulingError::from)
    }
}

// =============================================================================
// Scheduling engine
// =============================================================================

/// High-level scheduling engine.
///
/// This is the primary integration point for downstream compiler passes.
///
/// The engine is intentionally stateless.
///
/// That means:
///
/// - no global scheduler state;
/// - no global operation allocator;
/// - no hidden hardware state;
/// - deterministic repeated execution;
/// - safe concurrent use when separate calls use separate inputs/policies.
#[derive(Debug, Clone)]
pub struct SchedulingEngine<S = AsSoonAsPossible>
where
    S: SchedulingStrategy,
{
    strategy: S,
    limits: QuantumIrLimits,
}

impl Default for SchedulingEngine<AsSoonAsPossible> {
    fn default() -> Self {
        Self {
            strategy: AsSoonAsPossible::new(),
            limits: QuantumIrLimits::production(),
        }
    }
}

impl<S> SchedulingEngine<S>
where
    S: SchedulingStrategy,
{
    /// Creates an engine with a custom strategy and policy.
    #[must_use]
    pub fn new(
        strategy: S,
        limits: QuantumIrLimits,
    ) -> Self {
        Self {
            strategy,
            limits,
        }
    }

    /// Returns the selected strategy.
    #[must_use]
    pub const fn strategy(&self) -> &S {
        &self.strategy
    }

    /// Returns the configured scheduling policy.
    #[must_use]
    pub const fn limits(&self) -> &QuantumIrLimits {
        &self.limits
    }

    /// Schedules a validated input.
    pub fn schedule(
        &self,
        input: &SchedulingInput,
        schedule_id: ScheduleId,
    ) -> SchedulingResult<Schedule> {
        input.validate()?;

        let schedule =
            self.strategy.schedule(
                input,
                &self.limits,
                schedule_id,
            )?;

        schedule
            .validate(&self.limits)
            .map_err(SchedulingError::from)?;

        Ok(schedule)
    }

    /// Schedules using the current IR version and a caller-selected policy.
    ///
    /// This is an explicit alias for `schedule`.
    pub fn run(
        &self,
        input: &SchedulingInput,
        schedule_id: ScheduleId,
    ) -> SchedulingResult<Schedule> {
        self.schedule(input, schedule_id)
    }
}

// =============================================================================
// Scheduling builder
// =============================================================================

/// Builder for creating scheduling input.
///
/// This builder exists to make frontend/analysis/routing integration explicit
/// without coupling those subsystems to a particular scheduler algorithm.
#[derive(Debug, Clone)]
pub struct SchedulingInputBuilder {
    input: SchedulingInput,
}

impl Default for SchedulingInputBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedulingInputBuilder {
    /// Creates a builder using the current IR version.
    #[must_use]
    pub fn new() -> Self {
        Self {
            input: SchedulingInput::new(),
        }
    }

    /// Creates a builder for an explicit IR version.
    #[must_use]
    pub fn with_version(
        version: IrVersion,
    ) -> Self {
        Self {
            input: SchedulingInput::with_version(
                version,
            ),
        }
    }

    /// Adds a scheduling task.
    pub fn push(
        &mut self,
        task: SchedulingTask,
    ) -> SchedulingResult<&mut Self> {
        self.input.insert(task)?;
        Ok(self)
    }

    /// Consumes the builder and returns the scheduling input.
    pub fn build(
        self,
    ) -> SchedulingResult<SchedulingInput> {
        self.input.validate()?;
        Ok(self.input)
    }
}

// =============================================================================
// Convenience scheduling API
// =============================================================================

/// Schedules an input using the default deterministic ASAP strategy and the
/// production resource policy.
///
/// This is the simplest stable integration API.
pub fn schedule(
    input: &SchedulingInput,
    schedule_id: ScheduleId,
) -> SchedulingResult<Schedule> {
    SchedulingEngine::default()
        .schedule(input, schedule_id)
}

/// Schedules an input with an explicitly supplied policy.
pub fn schedule_with_limits(
    input: &SchedulingInput,
    schedule_id: ScheduleId,
    limits: QuantumIrLimits,
) -> SchedulingResult<Schedule> {
    SchedulingEngine::new(
        AsSoonAsPossible::new(),
        limits,
    )
    .schedule(input, schedule_id)
}

// =============================================================================
// Test-only/reference invariants
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::qubit::QubitId;

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    #[test]
    fn empty_input_produces_empty_schedule() {
        let input = SchedulingInput::new();

        let schedule =
            schedule(
                &input,
                ScheduleId::new(1),
            )
            .expect("empty input must schedule");

        assert!(schedule.is_empty());
    }

    #[test]
    fn independent_operations_can_run_in_parallel() {
        let q0 =
            QubitId::new(1);
        let q1 =
            QubitId::new(2);

        let mut builder =
            SchedulingInputBuilder::new();

        builder
            .push(
                SchedulingTask::new(
                    operation(1),
                    ScheduleDuration::from_nanoseconds(10)
                        .expect("10ns is representable"),
                )
                .with_logical_qubit(q0),
            )
            .expect("first task");

        builder
            .push(
                SchedulingTask::new(
                    operation(2),
                    ScheduleDuration::from_nanoseconds(20)
                        .expect("20ns is representable"),
                )
                .with_logical_qubit(q1),
            )
            .expect("second task");

        let input =
            builder
                .build()
                .expect("input must validate");

        let result =
            schedule(
                &input,
                ScheduleId::new(1),
            )
            .expect("independent operations must schedule");

        assert_eq!(
            result.operation_count(),
            2
        );

        assert_eq!(
            result
                .operations()[0]
                .start(),
            ScheduleTime::ZERO
        );

        assert_eq!(
            result
                .operations()[1]
                .start(),
            ScheduleTime::ZERO
        );
    }

    #[test]
    fn shared_resource_serializes_operations() {
        let q0 =
            QubitId::new(1);

        let first =
            SchedulingTask::new(
                operation(1),
                ScheduleDuration::from_nanoseconds(10)
                    .expect("10ns"),
            )
            .with_logical_qubit(q0);

        let second =
            SchedulingTask::new(
                operation(2),
                ScheduleDuration::from_nanoseconds(20)
                    .expect("20ns"),
            )
            .with_logical_qubit(q0);

        let mut builder =
            SchedulingInputBuilder::new();

        builder
            .push(first)
            .expect("first task");

        builder
            .push(second)
            .expect("second task");

        let input =
            builder
                .build()
                .expect("input must validate");

        let result =
            schedule(
                &input,
                ScheduleId::new(2),
            )
            .expect("shared resource must serialize");

        let first_end =
            result
                .operations()
                .iter()
                .find(|operation| {
                    operation.operation_id()
                        == operation(1)
                })
                .expect("first operation")
                .end()
                .expect("valid end");

        let second_start =
            result
                .operations()
                .iter()
                .find(|operation| {
                    operation.operation_id()
                        == operation(2)
                })
                .expect("second operation")
                .start();

        assert!(
            second_start >= first_end
        );
    }

    #[test]
    fn dependency_serializes_even_without_shared_resource() {
        let q0 =
            QubitId::new(1);
        let q1 =
            QubitId::new(2);

        let first =
            SchedulingTask::new(
                operation(1),
                ScheduleDuration::from_nanoseconds(10)
                    .expect("10ns"),
            )
            .with_logical_qubit(q0);

        let second =
            SchedulingTask::new(
                operation(2),
                ScheduleDuration::from_nanoseconds(10)
                    .expect("10ns"),
            )
            .with_logical_qubit(q1)
            .depends_on(operation(1));

        let mut builder =
            SchedulingInputBuilder::new();

        builder
            .push(first)
            .expect("first");

        builder
            .push(second)
            .expect("second");

        let input =
            builder
                .build()
                .expect("valid input");

        let result =
            schedule(
                &input,
                ScheduleId::new(3),
            )
            .expect("dependency must schedule");

        let first_end =
            result
                .operations()
                .iter()
                .find(|operation| {
                    operation.operation_id()
                        == operation(1)
                })
                .expect("first operation")
                .end()
                .expect("valid end");

        let second_start =
            result
                .operations()
                .iter()
                .find(|operation| {
                    operation.operation_id()
                        == operation(2)
                })
                .expect("second operation")
                .start();

        assert!(
            second_start >= first_end
        );
    }

    #[test]
    fn cycles_are_rejected_without_recursive_stack_use() {
        let first =
            SchedulingTask::new(
                operation(1),
                ScheduleDuration::ZERO,
            )
            .depends_on(operation(2));

        let second =
            SchedulingTask::new(
                operation(2),
                ScheduleDuration::ZERO,
            )
            .depends_on(operation(1));

        let mut builder =
            SchedulingInputBuilder::new();

        builder
            .push(first)
            .expect("first");

        builder
            .push(second)
            .expect("second");

        let input =
            builder.build();

        assert!(
            matches!(
                input,
                Err(
                    SchedulingError::DependencyCycle
                )
            )
        );
    }

    #[test]
    fn duplicate_resources_are_deduplicated() {
        let q0 =
            QubitId::new(1);

        let task =
            SchedulingTask::new(
                operation(1),
                ScheduleDuration::ZERO,
            )
            .with_logical_qubit(q0)
            .with_logical_qubit(q0);

        assert_eq!(
            task.resources().len(),
            1
        );
    }

    #[test]
    fn scheduling_is_deterministic() {
        let q0 =
            QubitId::new(1);
        let q1 =
            QubitId::new(2);

        let task1 =
            SchedulingTask::new(
                operation(10),
                ScheduleDuration::from_nanoseconds(5)
                    .expect("5ns"),
            )
            .with_logical_qubit(q0);

        let task2 =
            SchedulingTask::new(
                operation(20),
                ScheduleDuration::from_nanoseconds(5)
                    .expect("5ns"),
            )
            .with_logical_qubit(q1);

        let mut first =
            SchedulingInputBuilder::new();

        first
            .push(task1.clone())
            .expect("task1");

        first
            .push(task2.clone())
            .expect("task2");

        let mut second =
            SchedulingInputBuilder::new();

        second
            .push(task2)
            .expect("task2");

        second
            .push(task1)
            .expect("task1");

        let first_input =
            first.build()
                .expect("first input");

        let second_input =
            second.build()
                .expect("second input");

        let first_schedule =
            schedule(
                &first_input,
                ScheduleId::new(10),
            )
            .expect("first schedule");

        let second_schedule =
            schedule(
                &second_input,
                ScheduleId::new(10),
            )
            .expect("second schedule");

        assert_eq!(
            first_schedule,
            second_schedule
        );
    }
}