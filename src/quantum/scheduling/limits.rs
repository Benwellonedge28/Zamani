//! Zamani Quantum Scheduling — Execution and Resource Limits
//!
//! Production-grade, target-independent limits for the scheduling engine.
//!
//! # Architectural role
//!
//! This module defines **policy limits for one scheduling invocation**.
//!
//! It does NOT define:
//!
//! - the maximum number of qubits Zamani supports;
//! - hardware capacity;
//! - topology;
//! - gate semantics;
//! - logical-to-physical mapping;
//! - scheduling algorithms;
//! - timing semantics;
//! - QEC semantics;
//! - backend capabilities;
//! - vendor-specific restrictions.
//!
//! The distinction is fundamental:
//!
//! ```text
//! canonical IR
//!     = what the program means
//!
//! target capabilities
//!     = what a target can provide
//!
//! routing
//!     = where operations execute
//!
//! scheduling
//!     = when operations execute
//!
//! scheduling limits
//!     = how much scheduling work this invocation is permitted to consume
//! ```
//!
//! # Write once, scale everywhere
//!
//! No value in this module is a universal architectural ceiling.
//!
//! In particular, this module MUST NOT contain definitions such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_SCHEDULE_DEPTH
//! ```
//!
//! A production deployment may explicitly choose limits for safety, denial-of-
//! service protection, latency control, memory protection, or service-level
//! policy. A different deployment may choose different limits.
//!
//! An unbounded policy means:
//!
//! > this scheduling policy does not impose an application-level finite
//! > ceiling for that category.
//!
//! It does NOT mean that infinite memory, time, processors, quantum hardware,
//! or address space exist.
//!
//! The actual execution environment remains finite.
//!
//! # Relationship with `quantum::ir::core::limits`
//!
//! The canonical Quantum IR already owns [`QuantumIrLimits`].
//!
//! That policy governs resource consumption while constructing, validating,
//! analysing, transforming, serializing, and otherwise processing canonical
//! IR.
//!
//! This module deliberately does not redefine those limits.
//!
//! The separation is:
//!
//! ```text
//! QuantumIrLimits
//!     │
//!     ├── protects canonical IR processing
//!     │
//!     ▼
//! scheduler input
//!     │
//!     ▼
//! SchedulingLimits
//!     │
//!     └── protects scheduling work
//! ```
//!
//! A higher-level scheduling context/configuration may carry both policies.
//! This module does not create a dependency on `QuantumIrLimits`, which keeps
//! this file independently reusable and prevents the scheduling policy layer
//! from becoming coupled to the complete IR implementation.
//!
//! # No qubit identity
//!
//! This file intentionally does NOT import:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! That is deliberate and correct.
//!
//! Limits concern counts and computational work, not semantic qubit identity.
//!
//! Any scheduler component that actually refers to a logical or physical qubit
//! MUST use the canonical types from:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! No second qubit identity type may be introduced.
//!
//! # Scalable representation
//!
//! Scheduler quantities use `u64` for count-like policy values and `u128` for
//! schedule-time budgets.
//!
//! These are representation widths, not machine-size limits.
//!
//! The scheduler must still use checked arithmetic whenever values are
//! combined.
//!
//! `Option<T>` is used to represent the absence of a policy ceiling.
//!
//! Therefore:
//!
//! ```text
//! Some(value) = this invocation imposes a ceiling
//! None        = no application-level ceiling from this policy
//! ```
//!
//! # Why not `usize`?
//!
//! Semantic scheduling limits should not silently change width merely because
//! the host architecture changes.
//!
//! `u64` provides a stable representation for count policies.
//!
//! Conversion to collection indices is an explicit implementation concern and
//! must use checked conversion.
//!
//! # Why `u128` for schedule time?
//!
//! Scheduling time can represent extremely large logical schedules, especially
//! when composing:
//!
//! - long computations;
//! - QEC rounds;
//! - distributed communication;
//! - modular execution;
//! - waiting windows;
//! - calibration intervals;
//! - fault-tolerant workloads.
//!
//! The time representation here is deliberately unitless. The canonical
//! timing subsystem decides what a time unit means.
//!
//! This prevents this module from hard-coding nanoseconds, picoseconds,
//! hardware ticks, or sample periods.
//!
//! # Limits are not scheduler correctness
//!
//! Passing all limit checks does NOT mean that a schedule is valid.
//!
//! Correctness remains the responsibility of:
//!
//! - dependency validation;
//! - resource validation;
//! - timing validation;
//! - semantic validation;
//! - target capability validation;
//! - final schedule verification.
//!
//! # Determinism
//!
//! `SchedulingLimits` is immutable after construction.
//!
//! It contains no:
//!
//! - global mutable state;
//! - global counters;
//! - global allocators;
//! - hidden environment reads;
//! - thread-local scheduling state;
//! - randomness.
//!
//! The same limits value therefore has deterministic semantics.
//!
//! # Thread safety
//!
//! The type contains only immutable scalar values and is `Send`/`Sync` by
//! construction.
//!
//! It is safe to share one policy between independent scheduler workers.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! The safety boundary is compiler-enforced with `forbid(unsafe_code)`.
//!
//! # Integration contract
//!
//! This module is consumed by:
//!
//! ```text
//! scheduling::config
//! scheduling::context
//! scheduling::planners
//! scheduling::algorithms
//! scheduling::verification
//! scheduling::diagnostics
//! scheduling::plugins
//! ```
//!
//! It should NOT depend on those modules.
//!
//! Dependency direction:
//!
//! ```text
//! scheduling::limits
//!       │
//!       ├── config
//!       ├── context
//!       ├── planners
//!       ├── algorithms
//!       ├── verification
//!       └── plugins
//! ```
//!
//! This one-way dependency allows this file to be completed without requiring
//! later scheduler modules to be implemented first.
//!
//! # Policy composition
//!
//! A production scheduler may have several independent boundaries:
//!
//! ```text
//! IR limits
//!     ↓
//! input acceptance
//!
//! Scheduling limits
//!     ↓
//! scheduling work
//!
//! Target capabilities
//!     ↓
//! physical feasibility
//!
//! Runtime limits
//!     ↓
//! execution
//! ```
//!
//! No one policy should impersonate another.
//!
//! # Failure semantics
//!
//! Limit violations are represented by [`SchedulingLimitError`].
//!
//! The error contains the exact category and observed/allowed quantities so
//! callers can make programmatic decisions without parsing error strings.
//!
//! # Important invariant
//!
//! A value of `None` means "not limited by this policy", not "zero".
//!
//! A value of `Some(0)` means "nothing in this category is permitted".
//!
//! These meanings must never be conflated.
//!
//! # Memory scalability
//!
//! This module does not attempt to predict the exact memory required by every
//! scheduling algorithm. Such predictions are algorithm-specific.
//!
//! Instead, it provides explicit work/resource budgets that schedulers can
//! enforce at appropriate checkpoints.
//!
//! A planner remains responsible for translating its internal state into these
//! accounting categories.
//!
//! # No hard-coded architecture assumptions
//!
//! There are deliberately no assumptions about:
//!
//! - superconducting systems;
//! - trapped ions;
//! - neutral atoms;
//! - photons;
//! - spins;
//! - topological systems;
//! - annealers;
//! - analog quantum systems;
//! - distributed QPUs;
//! - future quantum architectures.
//!
//! Limits describe work, not technology.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::error::Error;
use std::fmt;

// =============================================================================
// Optional policy values
// =============================================================================

/// A scheduling policy ceiling.
///
/// `None` means that this policy does not impose a finite application-level
/// ceiling for the corresponding category.
///
/// `Some(value)` means the category is limited to `value`.
///
/// This wrapper exists to make the distinction explicit rather than using
/// sentinel integers such as zero or `u64::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Limit<T>(Option<T>);

impl<T> Limit<T> {
    /// Creates an explicitly bounded limit.
    #[must_use]
    pub const fn bounded(value: T) -> Self {
        Self(Some(value))
    }

    /// Creates an unbounded policy value.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self(None)
    }

    /// Returns the configured value.
    #[must_use]
    pub const fn get(self) -> Option<T>
    where
        T: Copy,
    {
        self.0
    }

    /// Returns `true` when this policy has no finite application-level
    /// ceiling.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.0.is_none()
    }

    /// Returns `true` when this policy has an explicit ceiling.
    #[must_use]
    pub const fn is_bounded(self) -> bool {
        self.0.is_some()
    }

    /// Returns whether `value` is permitted by this limit.
    #[must_use]
    pub fn allows(self, value: T) -> bool
    where
        T: Copy + PartialOrd,
    {
        match self.0 {
            Some(maximum) => value <= maximum,
            None => true,
        }
    }
}

impl<T> Default for Limit<T> {
    fn default() -> Self {
        Self::unbounded()
    }
}

// =============================================================================
// Scheduling resource categories
// =============================================================================

/// Categories of scheduler work that may be limited.
///
/// These are scheduler-policy categories, not hardware resource types.
///
/// Hardware resources such as physical qubits, measurement channels and
/// couplers belong to the target/resource model. They are not represented here
/// as fixed architectural limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingLimitKind {
    /// Number of schedulable operations accepted by this scheduling invocation.
    Operations,

    /// Number of dependency edges that may be processed.
    Dependencies,

    /// Number of resource requirements processed.
    ResourceRequirements,

    /// Number of resource reservations created.
    Reservations,

    /// Number of temporal intervals represented by the scheduler.
    Intervals,

    /// Number of ready operations simultaneously tracked.
    ReadyOperations,

    /// Number of events tracked by an event-driven scheduler.
    Events,

    /// Number of scheduling iterations.
    Iterations,

    /// Number of planner passes.
    PlannerPasses,

    /// Number of optimization passes performed as part of scheduling.
    OptimizationPasses,

    /// Number of verification checks performed.
    VerificationChecks,

    /// Number of diagnostics emitted.
    Diagnostics,

    /// Number of distributed nodes considered.
    DistributedNodes,

    /// Number of distributed links considered.
    DistributedLinks,

    /// Number of dynamic classical dependencies.
    ClassicalDependencies,

    /// Number of QEC constraints/requirements consumed.
    QecConstraints,

    /// Number of schedule transformations applied.
    Transformations,

    /// Maximum scheduled logical/target depth.
    ScheduleDepth,

    /// Maximum resulting schedule time.
    ScheduleTime,

    /// Maximum number of generated schedule objects.
    Schedules,

    /// Maximum number of candidate schedules retained simultaneously.
    CandidateSchedules,

    /// Maximum number of plugin/strategy invocations.
    PluginInvocations,

    /// Maximum amount of scheduler-owned metadata.
    MetadataBytes,

    /// Maximum amount of scheduler-owned diagnostic storage.
    DiagnosticBytes,

    /// Maximum amount of serialized schedule data.
    SerializedBytes,
}

impl SchedulingLimitKind {
    /// Returns the stable machine-independent name of this category.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operations => "operations",
            Self::Dependencies => "dependencies",
            Self::ResourceRequirements => "resource requirements",
            Self::Reservations => "reservations",
            Self::Intervals => "intervals",
            Self::ReadyOperations => "ready operations",
            Self::Events => "events",
            Self::Iterations => "iterations",
            Self::PlannerPasses => "planner passes",
            Self::OptimizationPasses => "optimization passes",
            Self::VerificationChecks => "verification checks",
            Self::Diagnostics => "diagnostics",
            Self::DistributedNodes => "distributed nodes",
            Self::DistributedLinks => "distributed links",
            Self::ClassicalDependencies => "classical dependencies",
            Self::QecConstraints => "QEC constraints",
            Self::Transformations => "transformations",
            Self::ScheduleDepth => "schedule depth",
            Self::ScheduleTime => "schedule time",
            Self::Schedules => "schedules",
            Self::CandidateSchedules => "candidate schedules",
            Self::PluginInvocations => "plugin invocations",
            Self::MetadataBytes => "metadata bytes",
            Self::DiagnosticBytes => "diagnostic bytes",
            Self::SerializedBytes => "serialized bytes",
        }
    }
}

impl fmt::Display for SchedulingLimitKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Limit error
// =============================================================================

/// Error produced when a scheduling policy is exceeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingLimitError {
    /// A count-like scheduler resource exceeded its configured ceiling.
    CountExceeded {
        /// Resource category.
        kind: SchedulingLimitKind,

        /// Observed/requested value.
        observed: u64,

        /// Configured maximum.
        maximum: u64,
    },

    /// A schedule-time budget was exceeded.
    TimeExceeded {
        /// Observed/requested schedule time.
        observed: u128,

        /// Configured maximum schedule time.
        maximum: u128,
    },

    /// An internal accounting addition would overflow.
    ArithmeticOverflow {
        /// Resource category being accounted.
        kind: SchedulingLimitKind,
    },

    /// A schedule-time accounting operation would overflow.
    TimeArithmeticOverflow,

    /// A configuration invariant is invalid.
    InvalidConfiguration {
        /// Policy field.
        field: &'static str,
    },
}

impl fmt::Display for SchedulingLimitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountExceeded {
                kind,
                observed,
                maximum,
            } => write!(
                formatter,
                "scheduling limit exceeded for {kind}: observed {observed}, maximum {maximum}"
            ),

            Self::TimeExceeded { observed, maximum } => write!(
                formatter,
                "schedule-time limit exceeded: observed {observed}, maximum {maximum}"
            ),

            Self::ArithmeticOverflow { kind } => write!(
                formatter,
                "arithmetic overflow while accounting for scheduling resource `{kind}`"
            ),

            Self::TimeArithmeticOverflow => {
                formatter.write_str(
                    "arithmetic overflow while accounting for schedule time",
                )
            }

            Self::InvalidConfiguration { field } => write!(
                formatter,
                "invalid scheduling limit configuration for `{field}`"
            ),
        }
    }
}

impl Error for SchedulingLimitError {}

// =============================================================================
// Result aliases
// =============================================================================

/// Result type for scheduling-limit operations.
pub type SchedulingLimitResult<T> = Result<T, SchedulingLimitError>;

// =============================================================================
// Scheduling limits
// =============================================================================

/// Explicit per-invocation scheduling resource policy.
///
/// This type contains **no universal machine-size limits**.
///
/// Every field is a policy ceiling that may independently be bounded or
/// unbounded.
///
/// # Default
///
/// [`SchedulingLimits::unbounded`] is intentionally the semantic default for
/// this low-level policy object.
///
/// A higher-level production service may construct a bounded policy according
/// to its security, latency, memory, tenancy, or resource requirements.
///
/// This avoids silently introducing an arbitrary architectural ceiling into
/// the scheduler.
///
/// # Relationship to hardware
///
/// Hardware capacity is not inferred from these values.
///
/// For example, a target may provide 10,000 physical resources while a service
/// policy permits only 1,000 scheduling operations for one invocation.
///
/// Conversely, a policy may permit millions of operations while a small target
/// makes the program physically unschedulable.
///
/// These are different questions.
///
/// # Relationship to `QuantumIrLimits`
///
/// `QuantumIrLimits` governs canonical IR processing.
///
/// `SchedulingLimits` governs scheduling work.
///
/// A scheduler invocation may therefore have both policies simultaneously.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchedulingLimits {
    operations: Limit<u64>,
    dependencies: Limit<u64>,
    resource_requirements: Limit<u64>,
    reservations: Limit<u64>,
    intervals: Limit<u64>,
    ready_operations: Limit<u64>,
    events: Limit<u64>,

    iterations: Limit<u64>,
    planner_passes: Limit<u64>,
    optimization_passes: Limit<u64>,
    verification_checks: Limit<u64>,

    diagnostics: Limit<u64>,
    distributed_nodes: Limit<u64>,
    distributed_links: Limit<u64>,
    classical_dependencies: Limit<u64>,
    qec_constraints: Limit<u64>,
    transformations: Limit<u64>,

    schedule_depth: Limit<u64>,
    schedule_time: Limit<u128>,

    schedules: Limit<u64>,
    candidate_schedules: Limit<u64>,
    plugin_invocations: Limit<u64>,

    metadata_bytes: Limit<u64>,
    diagnostic_bytes: Limit<u64>,
    serialized_bytes: Limit<u64>,
}

impl SchedulingLimits {
    /// Creates an entirely unbounded scheduler policy.
    ///
    /// This does not allocate resources and does not claim that infinite
    /// execution is possible. It only removes application-level ceilings from
    /// this policy object.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            operations: Limit::unbounded(),
            dependencies: Limit::unbounded(),
            resource_requirements: Limit::unbounded(),
            reservations: Limit::unbounded(),
            intervals: Limit::unbounded(),
            ready_operations: Limit::unbounded(),
            events: Limit::unbounded(),

            iterations: Limit::unbounded(),
            planner_passes: Limit::unbounded(),
            optimization_passes: Limit::unbounded(),
            verification_checks: Limit::unbounded(),

            diagnostics: Limit::unbounded(),
            distributed_nodes: Limit::unbounded(),
            distributed_links: Limit::unbounded(),
            classical_dependencies: Limit::unbounded(),
            qec_constraints: Limit::unbounded(),
            transformations: Limit::unbounded(),

            schedule_depth: Limit::unbounded(),
            schedule_time: Limit::unbounded(),

            schedules: Limit::unbounded(),
            candidate_schedules: Limit::unbounded(),
            plugin_invocations: Limit::unbounded(),

            metadata_bytes: Limit::unbounded(),
            diagnostic_bytes: Limit::unbounded(),
            serialized_bytes: Limit::unbounded(),
        }
    }

    /// Creates a bounded policy with every category initially set to zero.
    ///
    /// This is useful when a caller wants to explicitly opt individual
    /// categories into the policy.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            operations: Limit::bounded(0),
            dependencies: Limit::bounded(0),
            resource_requirements: Limit::bounded(0),
            reservations: Limit::bounded(0),
            intervals: Limit::bounded(0),
            ready_operations: Limit::bounded(0),
            events: Limit::bounded(0),

            iterations: Limit::bounded(0),
            planner_passes: Limit::bounded(0),
            optimization_passes: Limit::bounded(0),
            verification_checks: Limit::bounded(0),

            diagnostics: Limit::bounded(0),
            distributed_nodes: Limit::bounded(0),
            distributed_links: Limit::bounded(0),
            classical_dependencies: Limit::bounded(0),
            qec_constraints: Limit::bounded(0),
            transformations: Limit::bounded(0),

            schedule_depth: Limit::bounded(0),
            schedule_time: Limit::bounded(0),

            schedules: Limit::bounded(0),
            candidate_schedules: Limit::bounded(0),
            plugin_invocations: Limit::bounded(0),

            metadata_bytes: Limit::bounded(0),
            diagnostic_bytes: Limit::bounded(0),
            serialized_bytes: Limit::bounded(0),
        }
    }

    // -------------------------------------------------------------------------
    // Individual getters
    // -------------------------------------------------------------------------

    /// Returns the operation limit.
    #[must_use]
    pub const fn operations(&self) -> Limit<u64> {
        self.operations
    }

    /// Returns the dependency limit.
    #[must_use]
    pub const fn dependencies(&self) -> Limit<u64> {
        self.dependencies
    }

    /// Returns the resource-requirement limit.
    #[must_use]
    pub const fn resource_requirements(&self) -> Limit<u64> {
        self.resource_requirements
    }

    /// Returns the reservation limit.
    #[must_use]
    pub const fn reservations(&self) -> Limit<u64> {
        self.reservations
    }

    /// Returns the interval limit.
    #[must_use]
    pub const fn intervals(&self) -> Limit<u64> {
        self.intervals
    }

    /// Returns the ready-operation limit.
    #[must_use]
    pub const fn ready_operations(&self) -> Limit<u64> {
        self.ready_operations
    }

    /// Returns the event limit.
    #[must_use]
    pub const fn events(&self) -> Limit<u64> {
        self.events
    }

    /// Returns the scheduling-iteration limit.
    #[must_use]
    pub const fn iterations(&self) -> Limit<u64> {
        self.iterations
    }

    /// Returns the planner-pass limit.
    #[must_use]
    pub const fn planner_passes(&self) -> Limit<u64> {
        self.planner_passes
    }

    /// Returns the optimization-pass limit.
    #[must_use]
    pub const fn optimization_passes(&self) -> Limit<u64> {
        self.optimization_passes
    }

    /// Returns the verification-check limit.
    #[must_use]
    pub const fn verification_checks(&self) -> Limit<u64> {
        self.verification_checks
    }

    /// Returns the diagnostics limit.
    #[must_use]
    pub const fn diagnostics(&self) -> Limit<u64> {
        self.diagnostics
    }

    /// Returns the distributed-node limit.
    #[must_use]
    pub const fn distributed_nodes(&self) -> Limit<u64> {
        self.distributed_nodes
    }

    /// Returns the distributed-link limit.
    #[must_use]
    pub const fn distributed_links(&self) -> Limit<u64> {
        self.distributed_links
    }

    /// Returns the classical-dependency limit.
    #[must_use]
    pub const fn classical_dependencies(&self) -> Limit<u64> {
        self.classical_dependencies
    }

    /// Returns the QEC-constraint limit.
    #[must_use]
    pub const fn qec_constraints(&self) -> Limit<u64> {
        self.qec_constraints
    }

    /// Returns the transformation limit.
    #[must_use]
    pub const fn transformations(&self) -> Limit<u64> {
        self.transformations
    }

    /// Returns the schedule-depth limit.
    #[must_use]
    pub const fn schedule_depth(&self) -> Limit<u64> {
        self.schedule_depth
    }

    /// Returns the schedule-time limit.
    #[must_use]
    pub const fn schedule_time(&self) -> Limit<u128> {
        self.schedule_time
    }

    /// Returns the number-of-schedules limit.
    #[must_use]
    pub const fn schedules(&self) -> Limit<u64> {
        self.schedules
    }

    /// Returns the candidate-schedule limit.
    #[must_use]
    pub const fn candidate_schedules(&self) -> Limit<u64> {
        self.candidate_schedules
    }

    /// Returns the plugin-invocation limit.
    #[must_use]
    pub const fn plugin_invocations(&self) -> Limit<u64> {
        self.plugin_invocations
    }

    /// Returns the metadata-byte limit.
    #[must_use]
    pub const fn metadata_bytes(&self) -> Limit<u64> {
        self.metadata_bytes
    }

    /// Returns the diagnostic-byte limit.
    #[must_use]
    pub const fn diagnostic_bytes(&self) -> Limit<u64> {
        self.diagnostic_bytes
    }

    /// Returns the serialized-byte limit.
    #[must_use]
    pub const fn serialized_bytes(&self) -> Limit<u64> {
        self.serialized_bytes
    }

    // -------------------------------------------------------------------------
    // Builder methods
    // -------------------------------------------------------------------------

    /// Sets the operation limit.
    #[must_use]
    pub const fn with_operations(mut self, limit: Option<u64>) -> Self {
        self.operations = Limit(limit);
        self
    }

    /// Sets the dependency limit.
    #[must_use]
    pub const fn with_dependencies(mut self, limit: Option<u64>) -> Self {
        self.dependencies = Limit(limit);
        self
    }

    /// Sets the resource-requirement limit.
    #[must_use]
    pub const fn with_resource_requirements(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.resource_requirements = Limit(limit);
        self
    }

    /// Sets the reservation limit.
    #[must_use]
    pub const fn with_reservations(mut self, limit: Option<u64>) -> Self {
        self.reservations = Limit(limit);
        self
    }

    /// Sets the interval limit.
    #[must_use]
    pub const fn with_intervals(mut self, limit: Option<u64>) -> Self {
        self.intervals = Limit(limit);
        self
    }

    /// Sets the ready-operation limit.
    #[must_use]
    pub const fn with_ready_operations(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.ready_operations = Limit(limit);
        self
    }

    /// Sets the event limit.
    #[must_use]
    pub const fn with_events(mut self, limit: Option<u64>) -> Self {
        self.events = Limit(limit);
        self
    }

    /// Sets the iteration limit.
    #[must_use]
    pub const fn with_iterations(mut self, limit: Option<u64>) -> Self {
        self.iterations = Limit(limit);
        self
    }

    /// Sets the planner-pass limit.
    #[must_use]
    pub const fn with_planner_passes(mut self, limit: Option<u64>) -> Self {
        self.planner_passes = Limit(limit);
        self
    }

    /// Sets the optimization-pass limit.
    #[must_use]
    pub const fn with_optimization_passes(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.optimization_passes = Limit(limit);
        self
    }

    /// Sets the verification-check limit.
    #[must_use]
    pub const fn with_verification_checks(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.verification_checks = Limit(limit);
        self
    }

    /// Sets the diagnostics limit.
    #[must_use]
    pub const fn with_diagnostics(mut self, limit: Option<u64>) -> Self {
        self.diagnostics = Limit(limit);
        self
    }

    /// Sets the distributed-node limit.
    #[must_use]
    pub const fn with_distributed_nodes(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.distributed_nodes = Limit(limit);
        self
    }

    /// Sets the distributed-link limit.
    #[must_use]
    pub const fn with_distributed_links(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.distributed_links = Limit(limit);
        self
    }

    /// Sets the classical-dependency limit.
    #[must_use]
    pub const fn with_classical_dependencies(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.classical_dependencies = Limit(limit);
        self
    }

    /// Sets the QEC-constraint limit.
    #[must_use]
    pub const fn with_qec_constraints(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.qec_constraints = Limit(limit);
        self
    }

    /// Sets the transformation limit.
    #[must_use]
    pub const fn with_transformations(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.transformations = Limit(limit);
        self
    }

    /// Sets the schedule-depth limit.
    #[must_use]
    pub const fn with_schedule_depth(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.schedule_depth = Limit(limit);
        self
    }

    /// Sets the schedule-time limit.
    ///
    /// The unit is intentionally abstract and must be interpreted by the
    /// scheduling timing subsystem.
    #[must_use]
    pub const fn with_schedule_time(
        mut self,
        limit: Option<u128>,
    ) -> Self {
        self.schedule_time = Limit(limit);
        self
    }

    /// Sets the number-of-schedules limit.
    #[must_use]
    pub const fn with_schedules(mut self, limit: Option<u64>) -> Self {
        self.schedules = Limit(limit);
        self
    }

    /// Sets the candidate-schedule limit.
    #[must_use]
    pub const fn with_candidate_schedules(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.candidate_schedules = Limit(limit);
        self
    }

    /// Sets the plugin-invocation limit.
    #[must_use]
    pub const fn with_plugin_invocations(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.plugin_invocations = Limit(limit);
        self
    }

    /// Sets the metadata-byte limit.
    #[must_use]
    pub const fn with_metadata_bytes(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.metadata_bytes = Limit(limit);
        self
    }

    /// Sets the diagnostic-byte limit.
    #[must_use]
    pub const fn with_diagnostic_bytes(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.diagnostic_bytes = Limit(limit);
        self
    }

    /// Sets the serialized-byte limit.
    #[must_use]
    pub const fn with_serialized_bytes(
        mut self,
        limit: Option<u64>,
    ) -> Self {
        self.serialized_bytes = Limit(limit);
        self
    }

    // -------------------------------------------------------------------------
    // Generic checking
    // -------------------------------------------------------------------------

    /// Checks a count-like value against the corresponding policy.
    pub fn check_count(
        &self,
        kind: SchedulingLimitKind,
        value: u64,
    ) -> SchedulingLimitResult<()> {
        let limit = self.count_limit(kind);

        if let Some(maximum) = limit.get() {
            if value > maximum {
                return Err(SchedulingLimitError::CountExceeded {
                    kind,
                    observed: value,
                    maximum,
                });
            }
        }

        Ok(())
    }

    /// Checks schedule time against the configured policy.
    pub fn check_schedule_time(
        &self,
        value: u128,
    ) -> SchedulingLimitResult<()> {
        if let Some(maximum) = self.schedule_time.get() {
            if value > maximum {
                return Err(SchedulingLimitError::TimeExceeded {
                    observed: value,
                    maximum,
                });
            }
        }

        Ok(())
    }

    /// Returns the policy limit associated with a count category.
    ///
    /// `ScheduleTime` is intentionally excluded because it uses `u128`.
    #[must_use]
    pub const fn count_limit(
        &self,
        kind: SchedulingLimitKind,
    ) -> Limit<u64> {
        match kind {
            SchedulingLimitKind::Operations => self.operations,
            SchedulingLimitKind::Dependencies => self.dependencies,
            SchedulingLimitKind::ResourceRequirements => {
                self.resource_requirements
            }
            SchedulingLimitKind::Reservations => self.reservations,
            SchedulingLimitKind::Intervals => self.intervals,
            SchedulingLimitKind::ReadyOperations => self.ready_operations,
            SchedulingLimitKind::Events => self.events,
            SchedulingLimitKind::Iterations => self.iterations,
            SchedulingLimitKind::PlannerPasses => self.planner_passes,
            SchedulingLimitKind::OptimizationPasses => {
                self.optimization_passes
            }
            SchedulingLimitKind::VerificationChecks => {
                self.verification_checks
            }
            SchedulingLimitKind::Diagnostics => self.diagnostics,
            SchedulingLimitKind::DistributedNodes => self.distributed_nodes,
            SchedulingLimitKind::DistributedLinks => self.distributed_links,
            SchedulingLimitKind::ClassicalDependencies => {
                self.classical_dependencies
            }
            SchedulingLimitKind::QecConstraints => self.qec_constraints,
            SchedulingLimitKind::Transformations => self.transformations,
            SchedulingLimitKind::ScheduleDepth => self.schedule_depth,
            SchedulingLimitKind::Schedules => self.schedules,
            SchedulingLimitKind::CandidateSchedules => {
                self.candidate_schedules
            }
            SchedulingLimitKind::PluginInvocations => {
                self.plugin_invocations
            }
            SchedulingLimitKind::MetadataBytes => self.metadata_bytes,
            SchedulingLimitKind::DiagnosticBytes => self.diagnostic_bytes,
            SchedulingLimitKind::SerializedBytes => self.serialized_bytes,

            // Schedule time uses u128 and therefore must be checked through
            // `check_schedule_time`.
            SchedulingLimitKind::ScheduleTime => Limit::unbounded(),
        }
    }

    // -------------------------------------------------------------------------
    // Checked accounting helpers
    // -------------------------------------------------------------------------

    /// Adds two count values without permitting integer wraparound.
    pub const fn checked_add_count(
        kind: SchedulingLimitKind,
        current: u64,
        additional: u64,
    ) -> SchedulingLimitResult<u64> {
        match current.checked_add(additional) {
            Some(value) => Ok(value),
            None => Err(SchedulingLimitError::ArithmeticOverflow {
                kind,
            }),
        }
    }

    /// Adds two schedule-time quantities without permitting integer
    /// wraparound.
    pub const fn checked_add_time(
        current: u128,
        additional: u128,
    ) -> SchedulingLimitResult<u128> {
        match current.checked_add(additional) {
            Some(value) => Ok(value),
            None => Err(SchedulingLimitError::TimeArithmeticOverflow),
        }
    }

    /// Multiplies two count values without permitting integer wraparound.
    pub const fn checked_mul_count(
        kind: SchedulingLimitKind,
        left: u64,
        right: u64,
    ) -> SchedulingLimitResult<u64> {
        match left.checked_mul(right) {
            Some(value) => Ok(value),
            None => Err(SchedulingLimitError::ArithmeticOverflow {
                kind,
            }),
        }
    }

    /// Multiplies two schedule-time quantities without permitting integer
    /// wraparound.
    pub const fn checked_mul_time(
        left: u128,
        right: u128,
    ) -> SchedulingLimitResult<u128> {
        match left.checked_mul(right) {
            Some(value) => Ok(value),
            None => Err(SchedulingLimitError::TimeArithmeticOverflow),
        }
    }

    /// Adds a count to an existing accounting value and immediately checks
    /// the resulting value against the policy.
    pub fn account_count(
        &self,
        kind: SchedulingLimitKind,
        current: u64,
        additional: u64,
    ) -> SchedulingLimitResult<u64> {
        let next = Self::checked_add_count(
            kind,
            current,
            additional,
        )?;

        self.check_count(kind, next)?;

        Ok(next)
    }

    /// Adds schedule time to an existing accounting value and immediately
    /// checks the resulting value against the policy.
    pub fn account_time(
        &self,
        current: u128,
        additional: u128,
    ) -> SchedulingLimitResult<u128> {
        let next = Self::checked_add_time(
            current,
            additional,
        )?;

        self.check_schedule_time(next)?;

        Ok(next)
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates the policy's structural invariants.
    ///
    /// The representation currently guarantees that bounded values are
    /// non-negative because all count limits are unsigned.
    ///
    /// This method exists as a stable integration boundary so future policy
    /// fields can add structural validation without changing callers.
    pub const fn validate(&self) -> SchedulingLimitResult<()> {
        // All current fields are structurally valid by construction.
        //
        // Keep this explicit rather than removing the method. `config.rs` and
        // `context.rs` can therefore validate a policy before scheduling,
        // while future fields can introduce checks without changing their
        // integration contract.
        Ok(())
    }

    /// Returns whether this policy contains no finite ceiling.
    #[must_use]
    pub const fn is_unbounded(&self) -> bool {
        self.operations.is_unbounded()
            && self.dependencies.is_unbounded()
            && self.resource_requirements.is_unbounded()
            && self.reservations.is_unbounded()
            && self.intervals.is_unbounded()
            && self.ready_operations.is_unbounded()
            && self.events.is_unbounded()
            && self.iterations.is_unbounded()
            && self.planner_passes.is_unbounded()
            && self.optimization_passes.is_unbounded()
            && self.verification_checks.is_unbounded()
            && self.diagnostics.is_unbounded()
            && self.distributed_nodes.is_unbounded()
            && self.distributed_links.is_unbounded()
            && self.classical_dependencies.is_unbounded()
            && self.qec_constraints.is_unbounded()
            && self.transformations.is_unbounded()
            && self.schedule_depth.is_unbounded()
            && self.schedule_time.is_unbounded()
            && self.schedules.is_unbounded()
            && self.candidate_schedules.is_unbounded()
            && self.plugin_invocations.is_unbounded()
            && self.metadata_bytes.is_unbounded()
            && self.diagnostic_bytes.is_unbounded()
            && self.serialized_bytes.is_unbounded()
    }
}

impl Default for SchedulingLimits {
    fn default() -> Self {
        Self::unbounded()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unbounded_limit_allows_any_representable_value() {
        let limit = Limit::<u64>::unbounded();

        assert!(limit.is_unbounded());
        assert!(!limit.is_bounded());
        assert!(limit.allows(0));
        assert!(limit.allows(u64::MAX));
    }

    #[test]
    fn bounded_limit_rejects_values_above_ceiling() {
        let limit = Limit::bounded(10_u64);

        assert!(limit.is_bounded());
        assert!(!limit.is_unbounded());
        assert!(limit.allows(10));
        assert!(!limit.allows(11));
    }

    #[test]
    fn zero_is_a_real_bound_not_unbounded() {
        let limit = Limit::bounded(0_u64);

        assert!(limit.is_bounded());
        assert!(!limit.is_unbounded());
        assert!(limit.allows(0));
        assert!(!limit.allows(1));
    }

    #[test]
    fn none_means_no_policy_ceiling() {
        let limits = SchedulingLimits::unbounded();

        assert!(limits.is_unbounded());
        assert!(limits.operations().allows(u64::MAX));
        assert!(limits.dependencies().allows(u64::MAX));
        assert!(limits.schedule_time().allows(u128::MAX));
    }

    #[test]
    fn none_policy_is_zero_capability_policy() {
        let limits = SchedulingLimits::none();

        assert!(!limits.is_unbounded());
        assert!(limits.operations().allows(0));
        assert!(!limits.operations().allows(1));
        assert!(limits.schedule_depth().allows(0));
        assert!(!limits.schedule_depth().allows(1));
    }

    #[test]
    fn operation_limit_is_enforced() {
        let limits = SchedulingLimits::unbounded()
            .with_operations(Some(100));

        assert!(limits.check_count(
            SchedulingLimitKind::Operations,
            100
        ).is_ok());

        let error = limits.check_count(
            SchedulingLimitKind::Operations,
            101,
        );

        assert!(matches!(
            error,
            Err(SchedulingLimitError::CountExceeded {
                kind: SchedulingLimitKind::Operations,
                observed: 101,
                maximum: 100,
            })
        ));
    }

    #[test]
    fn schedule_time_limit_is_enforced() {
        let limits = SchedulingLimits::unbounded()
            .with_schedule_time(Some(1_000));

        assert!(limits.check_schedule_time(1_000).is_ok());

        let error = limits.check_schedule_time(1_001);

        assert!(matches!(
            error,
            Err(SchedulingLimitError::TimeExceeded {
                observed: 1_001,
                maximum: 1_000,
            })
        ));
    }

    #[test]
    fn checked_count_addition_detects_overflow() {
        let result = SchedulingLimits::checked_add_count(
            SchedulingLimitKind::Operations,
            u64::MAX,
            1,
        );

        assert!(matches!(
            result,
            Err(SchedulingLimitError::ArithmeticOverflow {
                kind: SchedulingLimitKind::Operations,
            })
        ));
    }

    #[test]
    fn checked_count_multiplication_detects_overflow() {
        let result = SchedulingLimits::checked_mul_count(
            SchedulingLimitKind::Operations,
            u64::MAX,
            2,
        );

        assert!(matches!(
            result,
            Err(SchedulingLimitError::ArithmeticOverflow {
                kind: SchedulingLimitKind::Operations,
            })
        ));
    }

    #[test]
    fn checked_time_addition_detects_overflow() {
        let result = SchedulingLimits::checked_add_time(
            u128::MAX,
            1,
        );

        assert!(matches!(
            result,
            Err(SchedulingLimitError::TimeArithmeticOverflow)
        ));
    }

    #[test]
    fn checked_time_multiplication_detects_overflow() {
        let result = SchedulingLimits::checked_mul_time(
            u128::MAX,
            2,
        );

        assert!(matches!(
            result,
            Err(SchedulingLimitError::TimeArithmeticOverflow)
        ));
    }

    #[test]
    fn account_count_checks_result_after_addition() {
        let limits = SchedulingLimits::unbounded()
            .with_operations(Some(10));

        assert_eq!(
            limits.account_count(
                SchedulingLimitKind::Operations,
                5,
                5,
            ),
            Ok(10)
        );

        assert!(matches!(
            limits.account_count(
                SchedulingLimitKind::Operations,
                5,
                6,
            ),
            Err(SchedulingLimitError::CountExceeded {
                kind: SchedulingLimitKind::Operations,
                observed: 11,
                maximum: 10,
            })
        ));
    }

    #[test]
    fn account_time_checks_result_after_addition() {
        let limits = SchedulingLimits::unbounded()
            .with_schedule_time(Some(100));

        assert_eq!(
            limits.account_time(40, 60),
            Ok(100)
        );

        assert!(matches!(
            limits.account_time(40, 61),
            Err(SchedulingLimitError::TimeExceeded {
                observed: 101,
                maximum: 100,
            })
        ));
    }

    #[test]
    fn generic_count_mapping_is_stable() {
        let limits = SchedulingLimits::unbounded()
            .with_operations(Some(1))
            .with_dependencies(Some(2))
            .with_reservations(Some(3))
            .with_schedule_depth(Some(4));

        assert_eq!(
            limits.count_limit(
                SchedulingLimitKind::Operations
            ).get(),
            Some(1)
        );

        assert_eq!(
            limits.count_limit(
                SchedulingLimitKind::Dependencies
            ).get(),
            Some(2)
        );

        assert_eq!(
            limits.count_limit(
                SchedulingLimitKind::Reservations
            ).get(),
            Some(3)
        );

        assert_eq!(
            limits.count_limit(
                SchedulingLimitKind::ScheduleDepth
            ).get(),
            Some(4)
        );

        // Schedule time is checked through its u128-specific API.
        assert!(limits.count_limit(
            SchedulingLimitKind::ScheduleTime
        ).is_unbounded());
    }

    #[test]
    fn resource_kind_names_are_stable() {
        assert_eq!(
            SchedulingLimitKind::Operations.as_str(),
            "operations"
        );

        assert_eq!(
            SchedulingLimitKind::ScheduleTime.as_str(),
            "schedule time"
        );

        assert_eq!(
            SchedulingLimitKind::QecConstraints.as_str(),
            "QEC constraints"
        );
    }

    #[test]
    fn policy_is_copy_and_deterministic() {
        let limits = SchedulingLimits::unbounded()
            .with_operations(Some(10))
            .with_schedule_time(Some(1_000));

        let copy = limits;

        assert_eq!(limits, copy);
        assert_eq!(
            limits.operations().get(),
            copy.operations().get()
        );
        assert_eq!(
            limits.schedule_time().get(),
            copy.schedule_time().get()
        );
    }

    #[test]
    fn validation_succeeds_for_unbounded_policy() {
        assert!(SchedulingLimits::unbounded().validate().is_ok());
    }

    #[test]
    fn validation_succeeds_for_zero_policy() {
        assert!(SchedulingLimits::none().validate().is_ok());
    }
}