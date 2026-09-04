//! Zamani Quantum Scheduling — Configuration Contract
//!
//! Production-grade, target-independent configuration for the top-level
//! quantum scheduling subsystem.
//!
//! # Architectural role
//!
//! This module defines the immutable configuration supplied to a scheduler.
//!
//! Configuration answers:
//!
//! > HOW should the scheduler operate for this invocation?
//!
//! It does NOT answer:
//!
//! > What hardware exists?
//!
//! Hardware capabilities belong to `quantum::hardware`.
//!
//! It does NOT answer:
//!
//! > How many qubits does this machine have?
//!
//! Target capacity belongs to the target/resource model.
//!
//! It does NOT define:
//!
//! - quantum semantics;
//! - QubitId;
//! - PhysicalQubitId;
//! - hardware topology;
//! - gate implementation;
//! - calibration;
//! - noise modelling;
//! - routing;
//! - QEC decoding;
//! - execution;
//! - backend authentication.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may be scheduled against targets of
//! radically different sizes and technologies.
//!
//! Therefore this configuration deliberately contains no machine-size
//! assumptions.
//!
//! A configuration may specify:
//!
//! - scheduling strategy;
//! - scheduling objective;
//! - deterministic behaviour;
//! - reproducibility seed;
//! - verification level;
//! - transformation policy;
//! - diagnostic policy;
//! - planning parallelism;
//! - explicit scheduler limits;
//! - deadline policy;
//! - failure behaviour.
//!
//! It must never contain a hard-coded statement such as:
//!
//! ```text
//! max_qubits = 1024
//! max_channels = 32
//! max_depth = 10000
//! ```
//!
//! Such values belong to explicit deployment policies or target capabilities,
//! never to the scheduler's universal defaults.
//!
//! # Relationship with `QuantumIrLimits`
//!
//! The canonical Quantum IR already owns:
//!
//! ```text
//! quantum::ir::core::limits::QuantumIrLimits
//! ```
//!
//! That type represents an explicit IR/resource/security policy for one
//! compilation or service invocation.
//!
//! `SchedulingConfig` therefore does NOT duplicate IR limits.
//!
//! Instead:
//!
//! ```text
//! QuantumIrLimits
//!        │
//!        │ protects IR processing
//!        ▼
//! scheduler input
//!        │
//!        ├── SchedulingConfig
//!        │
//!        ├── target capabilities
//!        ├── resource model
//!        ├── timing model
//!        └── scheduling policy
//!        │
//!        ▼
//! scheduler
//! ```
//!
//! The integration point for combining these policies belongs in
//! `scheduling::context` / the scheduling adapter layer, not in this
//! foundational configuration object.
//!
//! # Relationship with canonical qubit identity
//!
//! This file intentionally does not import or redefine `QubitId`.
//!
//! Qubit identity remains owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! and physical qubit identity, where applicable, remains owned by:
//!
//! ```text
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Configuration is intentionally independent of the number or identity of
//! qubits.
//!
//! # Dependency direction
//!
//! This file is intentionally low in the scheduling dependency graph.
//!
//! ```text
//! config.rs
//!     │
//!     ├── limits.rs
//!     │
//!     └── standard-library policy types
//! ```
//!
//! Higher-level scheduling modules consume this configuration:
//!
//! ```text
//! config
//!   ↓
//! context
//!   ↓
//! policies
//!   ↓
//! planners
//!   ↓
//! algorithms
//! ```
//!
//! This file must not depend on planners, algorithms, hardware backends,
//! routing implementations, QEC implementations, or runtime implementations.
//!
//! # Immutability
//!
//! A `SchedulingConfig` is an immutable value object after construction.
//!
//! Builder methods consume `self` and return a new configuration.
//!
//! There is:
//!
//! - no global configuration;
//! - no global mutable state;
//! - no singleton;
//! - no thread-local scheduler configuration;
//! - no hidden environment-variable configuration;
//! - no implicit machine discovery.
//!
//! # Determinism
//!
//! Deterministic scheduling is explicitly configurable.
//!
//! When deterministic scheduling is enabled, downstream scheduling algorithms
//! must use stable ordering and an explicit seed where randomness is required.
//!
//! The configuration itself never performs random generation.
//!
//! # Parallelism
//!
//! `parallelism` is an execution-policy hint for scheduler computation.
//!
//! It is NOT a hardware channel count and NOT a quantum parallelism limit.
//!
//! `None` means that the caller has not imposed an explicit scheduler worker
//! limit.
//!
//! # Limits
//!
//! `SchedulingLimits` contains optional scheduler guardrails.
//!
//! An absent limit means:
//!
//! > This scheduling policy does not impose a finite ceiling for this
//! > dimension.
//!
//! It does NOT mean infinite physical resources exist.
//!
//! Physical, operating-system, allocator, process, target, network, and
//! hardware limitations remain authoritative.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Safety
//!
//! This module explicitly forbids unsafe Rust.
//!
//! # Frozen-contract rule
//!
//! Once this file is accepted, downstream scheduling modules should consume
//! these configuration contracts rather than adding ad-hoc configuration
//! fields to individual algorithms.
//!
//! New scheduler behaviour should normally be introduced by extending the
//! relevant policy/objective/strategy abstraction rather than by introducing
//! global flags or machine-specific constants here.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::num::NonZeroU128;
use std::time::Duration;

use super::limits::SchedulingLimits;

// =============================================================================
// Scheduling strategy
// =============================================================================

/// High-level scheduling strategy.
///
/// A strategy describes the general temporal-placement approach. It does not
/// contain the implementation itself.
///
/// The actual algorithms belong to `scheduling::algorithms` and
/// `scheduling::planners`.
///
/// New strategies should be added here only when they represent a stable
/// public scheduling contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingStrategy {
    /// Schedule operations as early as all constraints permit.
    AsSoonAsPossible,

    /// Schedule operations as late as possible while preserving the supplied
    /// execution horizon and dependencies.
    AsLateAsPossible,

    /// Deterministic readiness/list scheduling.
    List,

    /// Critical-path-driven scheduling.
    CriticalPath,

    /// Resource-constrained scheduling.
    ResourceConstrained,

    /// Event-driven scheduling for dynamic/resource-release-heavy workloads.
    EventDriven,

    /// Let the scheduler select an appropriate strategy from the supplied
    /// graph/resource characteristics.
    Adaptive,
}

impl Default for SchedulingStrategy {
    fn default() -> Self {
        Self::List
    }
}

impl fmt::Display for SchedulingStrategy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::AsSoonAsPossible => "asap",
            Self::AsLateAsPossible => "alap",
            Self::List => "list",
            Self::CriticalPath => "critical-path",
            Self::ResourceConstrained => "resource-constrained",
            Self::EventDriven => "event-driven",
            Self::Adaptive => "adaptive",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Scheduling objective
// =============================================================================

/// Primary optimization objective.
///
/// Objectives describe what a scheduler should prefer after correctness
/// constraints have been satisfied.
///
/// An objective is not permission to violate dependencies, resource
/// constraints, timing constraints, or semantic correctness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingObjective {
    /// Produce any valid schedule without requesting an optimization target.
    Feasible,

    /// Minimize total execution duration.
    Makespan,

    /// Minimize temporal circuit depth.
    Depth,

    /// Minimize resource idle time.
    IdleTime,

    /// Prefer schedules expected to improve fidelity.
    Fidelity,

    /// Prefer schedules expected to reduce energy/resource cost.
    Energy,

    /// Use a caller-supplied weighted multi-objective policy.
    MultiObjective,
}

impl Default for SchedulingObjective {
    fn default() -> Self {
        Self::Makespan
    }
}

impl fmt::Display for SchedulingObjective {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Feasible => "feasible",
            Self::Makespan => "makespan",
            Self::Depth => "depth",
            Self::IdleTime => "idle-time",
            Self::Fidelity => "fidelity",
            Self::Energy => "energy",
            Self::MultiObjective => "multi-objective",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Verification policy
// =============================================================================

/// Verification policy for generated schedules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationPolicy {
    /// Do not perform scheduler-owned post-planning verification.
    ///
    /// This mode is intended for specialized analysis workflows. Production
    /// execution pipelines should normally use at least `Standard`.
    Disabled,

    /// Verify structural, dependency, resource, and timing invariants.
    Standard,

    /// Perform standard verification plus semantic preservation checks.
    Strict,

    /// Perform the strongest verification available to the configured
    /// verification subsystem.
    Exhaustive,
}

impl Default for VerificationPolicy {
    fn default() -> Self {
        Self::Strict
    }
}

// =============================================================================
// Transformation policy
// =============================================================================

/// Controls schedule transformations performed after temporal placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransformationPolicy {
    /// Do not automatically transform the schedule.
    None,

    /// Materialize required delays and target timing alignment.
    TimingOnly,

    /// Apply timing transformations and legal padding.
    TimingAndPadding,

    /// Permit timing-aware dynamical decoupling when the target/context
    /// explicitly supports it.
    TimingPaddingAndDecoupling,
}

impl Default for TransformationPolicy {
    fn default() -> Self {
        Self::TimingOnly
    }
}

// =============================================================================
// Failure policy
// =============================================================================

/// Defines what the scheduler should do when a valid schedule cannot be
/// produced under the selected policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FailurePolicy {
    /// Return the first structured error and stop.
    FailFast,

    /// Collect independent planning/verification diagnostics where possible,
    /// then return failure if the schedule is invalid.
    CollectDiagnostics,
}

impl Default for FailurePolicy {
    fn default() -> Self {
        Self::FailFast
    }
}

// =============================================================================
// Diagnostic policy
// =============================================================================

/// Controls scheduler diagnostic generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DiagnosticPolicy {
    /// No scheduler diagnostic events.
    Disabled,

    /// Emit high-level planning diagnostics.
    Basic,

    /// Emit operation/resource explanations.
    Detailed,

    /// Emit detailed diagnostics plus profiling information.
    DetailedAndProfiled,
}

impl Default for DiagnosticPolicy {
    fn default() -> Self {
        Self::Basic
    }
}

// =============================================================================
// Timing mode
// =============================================================================

/// Determines how the scheduler treats temporal information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingMode {
    /// Require all operations reaching concrete scheduling to have resolved
    /// durations.
    Concrete,

    /// Permit symbolic/unresolved timing information when supported by the
    /// supplied context.
    Symbolic,

    /// Resolve timing information through the supplied target/context before
    /// scheduling.
    ResolveFromTarget,

    /// Permit a mixture of concrete and symbolic timing information.
    Hybrid,
}

impl Default for TimingMode {
    fn default() -> Self {
        Self::ResolveFromTarget
    }
}

// =============================================================================
// Parallelism policy
// =============================================================================

/// Scheduler computation parallelism policy.
///
/// This is a host-side scheduling policy and must not be confused with
/// quantum-machine parallelism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParallelismPolicy {
    /// Execute scheduler computation without an explicit worker limit.
    ///
    /// The host/runtime remains responsible for actual available resources.
    Unbounded,

    /// Limit scheduler computation to the specified number of workers.
    Limited(NonZeroU128),

    /// Let the scheduler/runtime choose an appropriate level of host-side
    /// parallelism.
    Adaptive,
}

impl Default for ParallelismPolicy {
    fn default() -> Self {
        Self::Adaptive
    }
}

impl ParallelismPolicy {
    /// Creates a bounded parallelism policy.
    ///
    /// Returns `None` for zero because zero scheduler workers is not a valid
    /// execution policy.
    #[must_use]
    pub fn limited(workers: u128) -> Option<Self> {
        NonZeroU128::new(workers).map(Self::Limited)
    }

    /// Returns the configured worker count when explicitly bounded.
    #[must_use]
    pub const fn workers(self) -> Option<NonZeroU128> {
        match self {
            Self::Unbounded | Self::Adaptive => None,
            Self::Limited(workers) => Some(workers),
        }
    }
}

// =============================================================================
// Reproducibility policy
// =============================================================================

/// Reproducibility configuration.
///
/// Deterministic mode does not itself make a scheduler deterministic if the
/// target description or external resource state changes. The complete input
/// context must also be equivalent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReproducibilityConfig {
    deterministic: bool,
    seed: Option<u64>,
}

impl ReproducibilityConfig {
    /// Deterministic configuration without stochastic seed state.
    #[must_use]
    pub const fn deterministic() -> Self {
        Self {
            deterministic: true,
            seed: None,
        }
    }

    /// Non-deterministic configuration.
    #[must_use]
    pub const fn nondeterministic() -> Self {
        Self {
            deterministic: false,
            seed: None,
        }
    }

    /// Deterministic configuration with an explicit reproducibility seed.
    #[must_use]
    pub const fn with_seed(seed: u64) -> Self {
        Self {
            deterministic: true,
            seed: Some(seed),
        }
    }

    /// Returns whether deterministic scheduling is requested.
    #[must_use]
    pub const fn deterministic_mode(self) -> bool {
        self.deterministic
    }

    /// Returns the explicit seed, if one was supplied.
    #[must_use]
    pub const fn seed(self) -> Option<u64> {
        self.seed
    }
}

impl Default for ReproducibilityConfig {
    fn default() -> Self {
        Self::deterministic()
    }
}

// =============================================================================
// Deadline policy
// =============================================================================

/// Optional scheduler wall-clock deadline.
///
/// This is a host-side planning deadline. It is deliberately separate from a
/// quantum schedule's physical execution duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanningDeadline {
    timeout: Option<Duration>,
}

impl PlanningDeadline {
    /// Creates an unlimited planning deadline.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { timeout: None }
    }

    /// Creates a wall-clock planning deadline.
    ///
    /// A zero duration is valid and means the scheduler must not spend
    /// positive wall-clock time planning.
    #[must_use]
    pub const fn after(timeout: Duration) -> Self {
        Self {
            timeout: Some(timeout),
        }
    }

    /// Returns the configured timeout.
    #[must_use]
    pub const fn timeout(self) -> Option<Duration> {
        self.timeout
    }

    /// Returns whether a finite planning timeout is configured.
    #[must_use]
    pub const fn is_bounded(self) -> bool {
        self.timeout.is_some()
    }
}

impl Default for PlanningDeadline {
    fn default() -> Self {
        Self::unlimited()
    }
}

// =============================================================================
// Scheduler execution mode
// =============================================================================

/// Execution mode for the scheduling engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingMode {
    /// Produce one complete static schedule.
    Static,

    /// Permit runtime events and unresolved conditional timing.
    Dynamic,

    /// Schedule a partitioned/distributed execution plan.
    Distributed,

    /// Permit both static and dynamic scheduling features.
    Hybrid,
}

impl Default for SchedulingMode {
    fn default() -> Self {
        Self::Static
    }
}

// =============================================================================
// SchedulingConfig
// =============================================================================

/// Immutable production configuration for one scheduling invocation.
///
/// # No machine-size assumptions
///
/// This object intentionally contains no:
///
/// - maximum qubit count;
/// - maximum gate count;
/// - fixed topology dimensions;
/// - fixed number of channels;
/// - fixed schedule depth;
/// - fixed number of QPUs;
/// - fixed QEC distance.
///
/// Such properties are supplied by the target/resource/QEC contexts.
///
/// # Thread safety
///
/// `SchedulingConfig` contains only owned immutable values and standard
/// library value types. It does not contain locks, global state, callbacks,
/// raw pointers, or runtime handles.
///
/// A configuration may therefore be safely cloned and supplied to multiple
/// independent scheduler invocations.
///
/// # Configuration precedence
///
/// The scheduler should conceptually apply configuration in this order:
///
/// ```text
/// caller configuration
///       ↓
/// explicit target/context capabilities
///       ↓
/// scheduling policy
///       ↓
/// scheduling algorithm
/// ```
///
/// Configuration never overrides a target capability or semantic correctness
/// requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingConfig {
    strategy: SchedulingStrategy,
    objective: SchedulingObjective,
    mode: SchedulingMode,

    timing_mode: TimingMode,
    verification: VerificationPolicy,
    transformations: TransformationPolicy,
    failure_policy: FailurePolicy,
    diagnostics: DiagnosticPolicy,

    reproducibility: ReproducibilityConfig,
    parallelism: ParallelismPolicy,
    deadline: PlanningDeadline,

    limits: SchedulingLimits,

    /// Whether a caller explicitly requests schedule optimization.
    ///
    /// This is separate from the objective because a caller may request a
    /// feasible schedule while allowing the planner to use an optimization
    /// strategy when useful.
    optimize: bool,

    /// Whether the scheduler may select an algorithm adaptively.
    ///
    /// The strategy remains the declared high-level strategy; this flag only
    /// permits adaptive implementation selection where supported.
    adaptive_algorithm_selection: bool,

    /// Whether scheduler-generated delays may be represented explicitly in
    /// the resulting schedule.
    materialize_delays: bool,

    /// Whether resource availability is allowed to be time-dependent.
    ///
    /// This does not itself create resource availability information; that
    /// information must be supplied by the scheduling context.
    dynamic_resource_availability: bool,

    /// Whether runtime feedback dependencies are permitted.
    allow_dynamic_feedback: bool,

    /// Whether distributed communication dependencies are permitted.
    allow_distributed_communication: bool,
}

impl SchedulingConfig {
    /// Creates a production-oriented default configuration.
    ///
    /// The defaults contain no machine-size assumptions.
    ///
    /// Defaults:
    ///
    /// - list scheduling;
    /// - makespan objective;
    /// - static mode;
    /// - target-resolved timing;
    /// - strict verification;
    /// - timing-only transformations;
    /// - fail-fast errors;
    /// - basic diagnostics;
    /// - deterministic execution;
    /// - adaptive host parallelism;
    /// - no scheduler wall-clock deadline;
    /// - unlimited scheduler guardrails.
    #[must_use]
    pub fn new() -> Self {
        Self {
            strategy: SchedulingStrategy::default(),
            objective: SchedulingObjective::default(),
            mode: SchedulingMode::default(),

            timing_mode: TimingMode::default(),
            verification: VerificationPolicy::default(),
            transformations: TransformationPolicy::default(),
            failure_policy: FailurePolicy::default(),
            diagnostics: DiagnosticPolicy::default(),

            reproducibility: ReproducibilityConfig::default(),
            parallelism: ParallelismPolicy::default(),
            deadline: PlanningDeadline::default(),

            limits: SchedulingLimits::unlimited(),

            optimize: true,
            adaptive_algorithm_selection: false,
            materialize_delays: true,
            dynamic_resource_availability: false,
            allow_dynamic_feedback: false,
            allow_distributed_communication: false,
        }
    }

    /// Creates a configuration with explicitly conservative execution
    /// semantics suitable as a baseline for untrusted compilation services.
    ///
    /// This method still does not impose a machine-size limit. Deployment
    /// limits should be supplied explicitly through `SchedulingLimits`.
    #[must_use]
    pub fn restricted() -> Self {
        Self {
            verification: VerificationPolicy::Strict,
            transformations: TransformationPolicy::TimingOnly,
            failure_policy: FailurePolicy::FailFast,
            diagnostics: DiagnosticPolicy::Basic,
            reproducibility: ReproducibilityConfig::deterministic(),
            parallelism: ParallelismPolicy::Adaptive,
            deadline: PlanningDeadline::unlimited(),
            limits: SchedulingLimits::unlimited(),
            optimize: false,
            adaptive_algorithm_selection: false,
            materialize_delays: true,
            dynamic_resource_availability: false,
            allow_dynamic_feedback: false,
            allow_distributed_communication: false,
            ..Self::new()
        }
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    /// Returns the selected high-level scheduling strategy.
    #[must_use]
    pub const fn strategy(&self) -> SchedulingStrategy {
        self.strategy
    }

    /// Returns the selected optimization objective.
    #[must_use]
    pub const fn objective(&self) -> SchedulingObjective {
        self.objective
    }

    /// Returns the scheduling execution mode.
    #[must_use]
    pub const fn mode(&self) -> SchedulingMode {
        self.mode
    }

    /// Returns the timing interpretation mode.
    #[must_use]
    pub const fn timing_mode(&self) -> TimingMode {
        self.timing_mode
    }

    /// Returns the verification policy.
    #[must_use]
    pub const fn verification(&self) -> VerificationPolicy {
        self.verification
    }

    /// Returns the schedule transformation policy.
    #[must_use]
    pub const fn transformations(&self) -> TransformationPolicy {
        self.transformations
    }

    /// Returns the failure policy.
    #[must_use]
    pub const fn failure_policy(&self) -> FailurePolicy {
        self.failure_policy
    }

    /// Returns the diagnostic policy.
    #[must_use]
    pub const fn diagnostics(&self) -> DiagnosticPolicy {
        self.diagnostics
    }

    /// Returns reproducibility settings.
    #[must_use]
    pub const fn reproducibility(&self) -> ReproducibilityConfig {
        self.reproducibility
    }

    /// Returns host-side scheduler parallelism policy.
    #[must_use]
    pub const fn parallelism(&self) -> ParallelismPolicy {
        self.parallelism
    }

    /// Returns the optional host-side planning deadline.
    #[must_use]
    pub const fn deadline(&self) -> PlanningDeadline {
        self.deadline
    }

    /// Returns the scheduler guardrail policy.
    #[must_use]
    pub const fn limits(&self) -> &SchedulingLimits {
        &self.limits
    }

    /// Returns whether optimization is requested.
    #[must_use]
    pub const fn optimization_enabled(&self) -> bool {
        self.optimize
    }

    /// Returns whether adaptive algorithm selection is enabled.
    #[must_use]
    pub const fn adaptive_algorithm_selection(&self) -> bool {
        self.adaptive_algorithm_selection
    }

    /// Returns whether delays should be materialized explicitly.
    #[must_use]
    pub const fn materialize_delays(&self) -> bool {
        self.materialize_delays
    }

    /// Returns whether time-dependent resource availability may be consumed.
    #[must_use]
    pub const fn dynamic_resource_availability(&self) -> bool {
        self.dynamic_resource_availability
    }

    /// Returns whether runtime feedback dependencies are allowed.
    #[must_use]
    pub const fn allow_dynamic_feedback(&self) -> bool {
        self.allow_dynamic_feedback
    }

    /// Returns whether distributed communication dependencies are allowed.
    #[must_use]
    pub const fn allow_distributed_communication(&self) -> bool {
        self.allow_distributed_communication
    }

    // -------------------------------------------------------------------------
    // Builder methods
    // -------------------------------------------------------------------------

    /// Selects a scheduling strategy.
    #[must_use]
    pub const fn with_strategy(
        mut self,
        strategy: SchedulingStrategy,
    ) -> Self {
        self.strategy = strategy;
        self
    }

    /// Selects the optimization objective.
    #[must_use]
    pub const fn with_objective(
        mut self,
        objective: SchedulingObjective,
    ) -> Self {
        self.objective = objective;
        self
    }

    /// Selects static, dynamic, distributed, or hybrid scheduling mode.
    #[must_use]
    pub const fn with_mode(
        mut self,
        mode: SchedulingMode,
    ) -> Self {
        self.mode = mode;
        self
    }

    /// Selects the timing interpretation mode.
    #[must_use]
    pub const fn with_timing_mode(
        mut self,
        timing_mode: TimingMode,
    ) -> Self {
        self.timing_mode = timing_mode;
        self
    }

    /// Selects the verification policy.
    #[must_use]
    pub const fn with_verification(
        mut self,
        verification: VerificationPolicy,
    ) -> Self {
        self.verification = verification;
        self
    }

    /// Selects the transformation policy.
    #[must_use]
    pub const fn with_transformations(
        mut self,
        transformations: TransformationPolicy,
    ) -> Self {
        self.transformations = transformations;
        self
    }

    /// Selects failure behaviour.
    #[must_use]
    pub const fn with_failure_policy(
        mut self,
        failure_policy: FailurePolicy,
    ) -> Self {
        self.failure_policy = failure_policy;
        self
    }

    /// Selects diagnostic generation.
    #[must_use]
    pub const fn with_diagnostics(
        mut self,
        diagnostics: DiagnosticPolicy,
    ) -> Self {
        self.diagnostics = diagnostics;
        self
    }

    /// Sets reproducibility configuration.
    #[must_use]
    pub const fn with_reproducibility(
        mut self,
        reproducibility: ReproducibilityConfig,
    ) -> Self {
        self.reproducibility = reproducibility;
        self
    }

    /// Requests deterministic scheduling.
    #[must_use]
    pub const fn deterministic(self) -> Self {
        self.with_reproducibility(ReproducibilityConfig::deterministic())
    }

    /// Requests non-deterministic scheduling where the selected algorithm
    /// supports it.
    #[must_use]
    pub const fn nondeterministic(self) -> Self {
        self.with_reproducibility(
            ReproducibilityConfig::nondeterministic(),
        )
    }

    /// Sets an explicit reproducibility seed.
    ///
    /// An explicit seed implies deterministic configuration.
    #[must_use]
    pub const fn with_seed(self, seed: u64) -> Self {
        self.with_reproducibility(
            ReproducibilityConfig::with_seed(seed),
        )
    }

    /// Sets host-side scheduler parallelism.
    #[must_use]
    pub const fn with_parallelism(
        mut self,
        parallelism: ParallelismPolicy,
    ) -> Self {
        self.parallelism = parallelism;
        self
    }

    /// Sets a finite scheduler planning timeout.
    #[must_use]
    pub const fn with_planning_timeout(
        mut self,
        timeout: Duration,
    ) -> Self {
        self.deadline = PlanningDeadline::after(timeout);
        self
    }

    /// Removes the scheduler planning timeout.
    #[must_use]
    pub const fn without_planning_timeout(mut self) -> Self {
        self.deadline = PlanningDeadline::unlimited();
        self
    }

    /// Replaces scheduler guardrails.
    #[must_use]
    pub fn with_limits(
        mut self,
        limits: SchedulingLimits,
    ) -> Self {
        self.limits = limits;
        self
    }

    /// Enables or disables objective optimization.
    #[must_use]
    pub const fn with_optimization(
        mut self,
        enabled: bool,
    ) -> Self {
        self.optimize = enabled;
        self
    }

    /// Enables or disables adaptive implementation selection.
    #[must_use]
    pub const fn with_adaptive_algorithm_selection(
        mut self,
        enabled: bool,
    ) -> Self {
        self.adaptive_algorithm_selection = enabled;
        self
    }

    /// Enables or disables explicit delay materialization.
    #[must_use]
    pub const fn with_delay_materialization(
        mut self,
        enabled: bool,
    ) -> Self {
        self.materialize_delays = enabled;
        self
    }

    /// Enables or disables time-dependent resource availability.
    #[must_use]
    pub const fn with_dynamic_resource_availability(
        mut self,
        enabled: bool,
    ) -> Self {
        self.dynamic_resource_availability = enabled;
        self
    }

    /// Enables or disables runtime feedback scheduling.
    #[must_use]
    pub const fn with_dynamic_feedback(
        mut self,
        enabled: bool,
    ) -> Self {
        self.allow_dynamic_feedback = enabled;
        self
    }

    /// Enables or disables distributed communication scheduling.
    #[must_use]
    pub const fn with_distributed_communication(
        mut self,
        enabled: bool,
    ) -> Self {
        self.allow_distributed_communication = enabled;
        self
    }

    // -------------------------------------------------------------------------
    // Semantic validation
    // -------------------------------------------------------------------------

    /// Validates internal configuration relationships.
    ///
    /// This does not validate the target or program. Those belong to the
    /// scheduling context and validation subsystem.
    ///
    /// The method intentionally returns a small static error type so the
    /// foundational configuration contract does not depend on the broader
    /// scheduling error hierarchy.
    pub fn validate(&self) -> Result<(), ConfigValidationError> {
        if self.strategy == SchedulingStrategy::AsLateAsPossible
            && self.mode == SchedulingMode::Dynamic
        {
            return Err(
                ConfigValidationError::AlapDynamicModeUnsupported,
            );
        }

        if self.transformations
            == TransformationPolicy::TimingPaddingAndDecoupling
            && self.verification == VerificationPolicy::Disabled
        {
            return Err(
                ConfigValidationError::DecouplingRequiresVerification,
            );
        }

        if self.allow_dynamic_feedback
            && self.mode == SchedulingMode::Static
        {
            return Err(
                ConfigValidationError::DynamicFeedbackRequiresDynamicMode,
            );
        }

        if self.allow_distributed_communication
            && self.mode == SchedulingMode::Static
        {
            return Err(
                ConfigValidationError::DistributedCommunicationRequiresDistributedMode,
            );
        }

        if self.adaptive_algorithm_selection
            && self.strategy != SchedulingStrategy::Adaptive
        {
            return Err(
                ConfigValidationError::AdaptiveSelectionRequiresAdaptiveStrategy,
            );
        }

        Ok(())
    }

    /// Returns whether this configuration is internally valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

impl Default for SchedulingConfig {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Configuration validation errors
// =============================================================================

/// Static configuration relationship errors.
///
/// These errors deliberately remain independent of `SchedulingError` so this
/// foundational module can be implemented before the broader scheduling error
/// hierarchy is finalized.
///
/// `errors.rs` can later provide a direct conversion into the public
/// `SchedulingError`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigValidationError {
    /// ALAP requires a known execution horizon and therefore cannot be used
    /// directly as the static dynamic-mode policy.
    AlapDynamicModeUnsupported,

    /// Dynamical decoupling is a semantic schedule transformation and requires
    /// post-transformation verification.
    DecouplingRequiresVerification,

    /// Runtime feedback requires dynamic or hybrid scheduling.
    DynamicFeedbackRequiresDynamicMode,

    /// Distributed communication requires distributed or hybrid scheduling.
    DistributedCommunicationRequiresDistributedMode,

    /// Adaptive implementation selection requires the Adaptive high-level
    /// strategy.
    AdaptiveSelectionRequiresAdaptiveStrategy,
}

impl fmt::Display for ConfigValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::AlapDynamicModeUnsupported => formatter.write_str(
                "ALAP scheduling requires a concrete execution horizon and \
                 cannot be configured directly for dynamic scheduling",
            ),

            Self::DecouplingRequiresVerification => formatter.write_str(
                "dynamical-decoupling transformation requires schedule \
                 verification",
            ),

            Self::DynamicFeedbackRequiresDynamicMode => formatter.write_str(
                "dynamic feedback requires Dynamic or Hybrid scheduling mode",
            ),

            Self::DistributedCommunicationRequiresDistributedMode => {
                formatter.write_str(
                    "distributed communication requires Distributed or \
                     Hybrid scheduling mode",
                )
            }

            Self::AdaptiveSelectionRequiresAdaptiveStrategy => {
                formatter.write_str(
                    "adaptive algorithm selection requires the Adaptive \
                     scheduling strategy",
                )
            }
        }
    }
}

impl std::error::Error for ConfigValidationError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        let config = SchedulingConfig::default();

        assert!(config.is_valid());
        assert_eq!(
            config.strategy(),
            SchedulingStrategy::List
        );
        assert_eq!(
            config.objective(),
            SchedulingObjective::Makespan
        );
        assert_eq!(
            config.mode(),
            SchedulingMode::Static
        );
        assert!(config.reproducibility().deterministic_mode());
        assert!(config.deadline().timeout().is_none());
    }

    #[test]
    fn deterministic_seed_is_preserved() {
        let config = SchedulingConfig::new().with_seed(42);

        assert!(config.reproducibility().deterministic_mode());
        assert_eq!(
            config.reproducibility().seed(),
            Some(42)
        );
    }

    #[test]
    fn zero_parallelism_is_rejected() {
        assert!(ParallelismPolicy::limited(0).is_none());
    }

    #[test]
    fn bounded_parallelism_is_preserved() {
        let policy = ParallelismPolicy::limited(8)
            .expect("8 is non-zero");

        assert_eq!(
            policy.workers().map(NonZeroU128::get),
            Some(8)
        );
    }

    #[test]
    fn planning_timeout_is_preserved() {
        let config = SchedulingConfig::new()
            .with_planning_timeout(Duration::from_secs(5));

        assert_eq!(
            config.deadline().timeout(),
            Some(Duration::from_secs(5))
        );
    }

    #[test]
    fn planning_timeout_can_be_removed() {
        let config = SchedulingConfig::new()
            .with_planning_timeout(Duration::from_secs(5))
            .without_planning_timeout();

        assert_eq!(
            config.deadline().timeout(),
            None
        );
    }

    #[test]
    fn dynamic_feedback_requires_dynamic_mode() {
        let config = SchedulingConfig::new()
            .with_dynamic_feedback(true);

        assert_eq!(
            config.validate(),
            Err(
                ConfigValidationError::DynamicFeedbackRequiresDynamicMode
            )
        );
    }

    #[test]
    fn dynamic_feedback_is_valid_in_dynamic_mode() {
        let config = SchedulingConfig::new()
            .with_mode(SchedulingMode::Dynamic)
            .with_dynamic_feedback(true);

        assert!(config.is_valid());
    }

    #[test]
    fn distributed_communication_requires_distributed_mode() {
        let config = SchedulingConfig::new()
            .with_distributed_communication(true);

        assert_eq!(
            config.validate(),
            Err(
                ConfigValidationError::DistributedCommunicationRequiresDistributedMode
            )
        );
    }

    #[test]
    fn distributed_communication_is_valid_in_hybrid_mode() {
        let config = SchedulingConfig::new()
            .with_mode(SchedulingMode::Hybrid)
            .with_distributed_communication(true);

        assert!(config.is_valid());
    }

    #[test]
    fn adaptive_selection_requires_adaptive_strategy() {
        let config = SchedulingConfig::new()
            .with_adaptive_algorithm_selection(true);

        assert_eq!(
            config.validate(),
            Err(
                ConfigValidationError::AdaptiveSelectionRequiresAdaptiveStrategy
            )
        );
    }

    #[test]
    fn adaptive_selection_is_valid_with_adaptive_strategy() {
        let config = SchedulingConfig::new()
            .with_strategy(SchedulingStrategy::Adaptive)
            .with_adaptive_algorithm_selection(true);

        assert!(config.is_valid());
    }

    #[test]
    fn decoupling_requires_verification() {
        let config = SchedulingConfig::new()
            .with_verification(VerificationPolicy::Disabled)
            .with_transformations(
                TransformationPolicy::TimingPaddingAndDecoupling,
            );

        assert_eq!(
            config.validate(),
            Err(
                ConfigValidationError::DecouplingRequiresVerification
            )
        );
    }

    #[test]
    fn no_machine_size_is_encoded_in_configuration() {
        let config = SchedulingConfig::new();

        assert!(config.is_valid());
        assert!(config.limits().is_unlimited());
    }

    #[test]
    fn builder_methods_compose() {
        let config = SchedulingConfig::new()
            .with_strategy(SchedulingStrategy::ResourceConstrained)
            .with_objective(SchedulingObjective::Fidelity)
            .with_mode(SchedulingMode::Hybrid)
            .with_timing_mode(TimingMode::Hybrid)
            .with_verification(VerificationPolicy::Exhaustive)
            .with_transformations(
                TransformationPolicy::TimingAndPadding,
            )
            .with_failure_policy(FailurePolicy::CollectDiagnostics)
            .with_diagnostics(DiagnosticPolicy::DetailedAndProfiled)
            .with_parallelism(ParallelismPolicy::Adaptive)
            .with_seed(1234)
            .with_optimization(true)
            .with_delay_materialization(true)
            .with_dynamic_resource_availability(true)
            .with_dynamic_feedback(true)
            .with_distributed_communication(true);

        assert!(config.is_valid());
        assert_eq!(
            config.strategy(),
            SchedulingStrategy::ResourceConstrained
        );
        assert_eq!(
            config.objective(),
            SchedulingObjective::Fidelity
        );
        assert_eq!(
            config.mode(),
            SchedulingMode::Hybrid
        );
        assert!(config.dynamic_resource_availability());
        assert!(config.allow_dynamic_feedback());
        assert!(config.allow_distributed_communication());
    }
}