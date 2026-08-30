//! Zamani Quantum Optimization — Optimization Pass Scheduler
//!
//! Production-grade scheduler for logical optimization passes.
//!
//! # Architectural position
//!
//! This file schedules *optimization passes*. It does NOT schedule quantum
//! operations for execution on a QPU. Quantum execution scheduling belongs to
//! `crate::quantum::scheduling`.
//!
//! The optimization architecture is:
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                         optimizer
//!                              │
//!                    ┌─────────┴─────────┐
//!                    │                   │
//!                    ▼                   ▼
//!                 planner             registry
//!                    │                   │
//!                    └─────────┬─────────┘
//!                              ▼
//!                         scheduler
//!                              │
//!                    ┌─────────┴─────────┐
//!                    │                   │
//!                    ▼                   ▼
//!               dependency DAG       execution waves
//!                    │                   │
//!                    └─────────┬─────────┘
//!                              ▼
//!                           pipeline
//!                              │
//!                              ▼
//!                       optimized quantum::ir
//! ```
//!
//! # Critical ownership boundary
//!
//! `scheduler.rs` owns:
//!
//! - pass dependency analysis;
//! - pass ordering;
//! - deterministic scheduling;
//! - scheduling waves;
//! - dependency/conflict detection;
//! - scheduler-local structural limits;
//! - schedule diagnostics;
//! - schedule validation.
//!
//! It does NOT own:
//!
//! - `QuantumCircuit`;
//! - quantum execution timing;
//! - hardware topology;
//! - QPU execution;
//! - routing;
//! - backend APIs;
//! - optimization algorithms;
//! - analysis implementations;
//! - pass registration;
//! - pipeline execution;
//! - final optimization results;
//! - benchmarking;
//! - QEC.
//!
//! # Canonical IR rule
//!
//! Optimization passes operate on:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! This scheduler never introduces another circuit representation.
//!
//! # Why scheduling must be conservative
//!
//! Two optimization passes may both be mathematically valid while still being
//! unsafe to execute concurrently because both may mutate the same circuit or
//! because one invalidates an analysis/property required by the other.
//!
//! Therefore this scheduler uses a deliberately conservative rule:
//!
//! - read-only analysis passes may be placed into an analysis wave when their
//!   metadata permits it;
//! - circuit-mutating passes are serialized unless a future explicit
//!   disjoint-region capability is introduced;
//! - a pass that writes/invalidate state is treated as a scheduling barrier;
//! - requirements and invalidations create ordering edges;
//! - original pipeline order remains a valid deterministic tie-breaker.
//!
//! This is safer than pretending that two arbitrary `&mut QuantumCircuit`
//! operations can be parallelized merely because their high-level algorithms
//! appear independent.
//!
//! # Integration with `pass.rs`
//!
//! The scheduler consumes:
//!
//! - `OptimizationPass`;
//! - `PassId`;
//! - `PassCapability`;
//! - `PassChange`;
//! - `PassComplexity`;
//! - `PassDeterminism`;
//! - `PassKind`;
//! - `PassMetadata`;
//! - `PassRequirements`;
//! - `PassEffects`;
//! - `PassScope`.
//!
//! `pass.rs` already defines these scheduler-visible contracts.
//!
//! This file deliberately does not modify that contract.
//!
//! # Integration with `context.rs`
//!
//! `OptimizationContext` owns:
//!
//! - optimizer resource accounting;
//! - pass counters;
//! - iteration counters;
//! - analysis state;
//! - cancellation/deadline state;
//! - deterministic state;
//! - invocation-scoped mutable state.
//!
//! The scheduler does not duplicate those resources.
//!
//! The pipeline/context remain responsible for enforcing global optimizer
//! resource limits while executing the resulting schedule.
//!
//! # Integration with `limits.rs`
//!
//! `OptimizationLimits` already owns optimizer work limits such as:
//!
//! - maximum passes;
//! - maximum iterations;
//! - maximum rewrites;
//! - analysis work;
//! - dependency edges;
//! - candidate counts;
//! - synthesis work;
//! - verification work;
//! - provenance;
//! - wall-clock budget.
//!
//! The scheduler must not create a competing `OptimizationLimits` abstraction.
//!
//! Instead, the scheduler produces a finite schedule and the pipeline/context
//! enforce the configured optimizer limits while executing it.
//!
//! # Integration with `pipeline.rs`
//!
//! The intended pipeline lifecycle is:
//!
//! ```text
//! registry
//!     │
//!     ▼
//! planner
//!     │
//!     ▼
//! scheduler::build_schedule()
//!     │
//!     ▼
//! scheduler::validate_schedule()
//!     │
//!     ▼
//! pipeline executes each scheduled pass
//!     │
//!     ▼
//! context records execution state
//! ```
//!
//! `scheduler.rs` does not call `OptimizationPass::run`.
//!
//! This separation is intentional:
//!
//! - scheduler = decide order;
//! - pipeline = execute order.
//!
//! # Integration with `registry.rs`
//!
//! Registry ownership remains outside this file.
//!
//! The registry supplies an ordered collection of pass references. The
//! scheduler treats that collection as the authoritative planner-selected
//! candidate sequence.
//!
//! A new pass therefore does not require changes to this file.
//!
//! # Integration with `planner.rs`
//!
//! The planner may select:
//!
//! - a subset of passes;
//! - a target-specific sequence;
//! - an optimization profile;
//! - a fixed-point group;
//! - an aggressive sequence.
//!
//! The scheduler then converts that selection into a dependency-safe schedule.
//!
//! The scheduler does not decide which optimization algorithms are desirable.
//!
//! # Integration with analysis modules
//!
//! A pass may declare requirements such as:
//!
//! - dependency analysis;
//! - commutation analysis;
//! - liveness;
//! - depth;
//! - gate counts;
//! - parameter usage;
//!
//! through `AnalysisRequirement`.
//!
//! A pass may invalidate an analysis through `AnalysisInvalidation`.
//!
//! The scheduler uses those declarations to construct ordering constraints.
//!
//! Concrete analysis computation remains owned by the analysis subsystem.
//!
//! # Determinism
//!
//! Schedule construction is deterministic:
//!
//! - input order is preserved as the primary stable order;
//! - dependency edges are deterministic;
//! - ready nodes are selected by original pass index;
//! - wave construction is deterministic;
//! - no hash-map iteration order is exposed as a scheduling decision.
//!
//! # Scaling
//!
//! The scheduler is designed for:
//!
//! - tiny pipelines;
//! - thousands of passes;
//! - generated pass graphs;
//! - very large optimization pipelines;
//! - resource-limited compilation;
//! - future incremental scheduling.
//!
//! It does not impose a circuit-size limit because it never owns the circuit.
//!
//! Scheduling complexity is proportional to the number of passes and declared
//! dependency relationships.
//!
//! The implementation uses iterative algorithms rather than recursive graph
//! traversal so deeply chained pass graphs do not risk stack exhaustion.
//!
//! # Safety
//!
//! This file contains no unsafe code.
//!
//! `#![forbid(unsafe_code)]` is used deliberately.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Important distinction
//!
//! ```text
//! optimization::scheduler
//!     = schedules compiler optimization passes
//!
//! quantum::scheduling
//!     = schedules quantum operations for execution
//! ```
//!
//! These must remain separate indefinitely.
//!
//! # External architectural reference
//!
//! Mature quantum compiler frameworks also treat pass ordering as a semantic
//! compilation concern. For example, pytket explicitly notes that passes can
//! invalidate properties established by earlier passes and therefore require
//! carefully ordered compilation sequences. Zamani's explicit requirement and
//! effect declarations provide a stronger foundation for making that ordering
//! machine-checkable.
//!
//! # No future-file rewrites
//!
//! Adding a new optimization pass should require no change to this file unless
//! the fundamental pass scheduling contract itself changes.
//!
//! A pass only needs to expose correct `PassMetadata` through `OptimizationPass`.
//!
//! The scheduler discovers the pass properties dynamically.

#![forbid(unsafe_code)]

use std::collections::{BTreeSet, VecDeque};
use std::fmt;

use super::pass::{
    OptimizationPass,
    PassCapability,
    PassComplexity,
    PassDeterminism,
    PassKind,
    PassScope,
};

// =============================================================================
// Public result
// =============================================================================

/// Result type returned by scheduler operations.
pub type SchedulerResult<T> = Result<T, SchedulerError>;

// =============================================================================
// Scheduler configuration
// =============================================================================

/// Policy controlling how the optimization-pass scheduler constructs a plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SchedulingPolicy {
    /// Preserve planner order and serialize every transformation.
    ///
    /// This is the safest production default.
    Conservative,

    /// Allow independent read-only analysis passes to share a wave.
    ///
    /// Circuit-transforming passes remain serialized.
    AnalysisParallel,

    /// Aggressively attempt to form independent waves.
    ///
    /// This mode is still conservative with respect to circuit mutation.
    /// It does NOT assume that two arbitrary transformation passes are
    /// thread-safe or region-disjoint.
    Aggressive,
}

impl Default for SchedulingPolicy {
    fn default() -> Self {
        Self::Conservative
    }
}

impl SchedulingPolicy {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conservative => "conservative",
            Self::AnalysisParallel => "analysis_parallel",
            Self::Aggressive => "aggressive",
        }
    }
}

impl fmt::Display for SchedulingPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Scheduler configuration
// =============================================================================

/// Configuration for schedule construction.
///
/// These are scheduling-graph controls, not replacement optimization resource
/// limits. The global optimizer limits remain owned by `OptimizationLimits`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchedulerConfig {
    /// Scheduling strategy.
    policy: SchedulingPolicy,

    /// Maximum number of passes accepted into one schedule.
    ///
    /// Zero means no scheduler-local pass-count ceiling.
    ///
    /// The optimizer's global pass limit remains authoritative during actual
    /// execution.
    max_scheduled_passes: u64,

    /// Maximum dependency edges allowed while constructing the schedule.
    ///
    /// Zero means no scheduler-local edge ceiling.
    ///
    /// The global optimizer dependency-edge limit remains authoritative for
    /// analysis work.
    max_dependency_edges: u64,

    /// Maximum passes permitted in one execution wave.
    ///
    /// Zero means no wave-size ceiling.
    max_wave_size: u64,

    /// Whether the scheduler rejects metadata that claims large-circuit
    /// support for an obviously non-scalable pass.
    ///
    /// This is intentionally disabled by default because complexity metadata
    /// is advisory and planner policy should normally make this decision.
    reject_non_scalable_large_circuit_passes: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            policy: SchedulingPolicy::Conservative,
            max_scheduled_passes: 0,
            max_dependency_edges: 0,
            max_wave_size: 0,
            reject_non_scalable_large_circuit_passes: false,
        }
    }
}

impl SchedulerConfig {
    /// Creates the conservative production configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            policy: SchedulingPolicy::Conservative,
            max_scheduled_passes: 0,
            max_dependency_edges: 0,
            max_wave_size: 0,
            reject_non_scalable_large_circuit_passes: false,
        }
    }

    /// Creates an analysis-parallel configuration.
    #[must_use]
    pub const fn analysis_parallel() -> Self {
        Self {
            policy: SchedulingPolicy::AnalysisParallel,
            max_scheduled_passes: 0,
            max_dependency_edges: 0,
            max_wave_size: 0,
            reject_non_scalable_large_circuit_passes: false,
        }
    }

    /// Creates an aggressive scheduling configuration.
    #[must_use]
    pub const fn aggressive() -> Self {
        Self {
            policy: SchedulingPolicy::Aggressive,
            max_scheduled_passes: 0,
            max_dependency_edges: 0,
            max_wave_size: 0,
            reject_non_scalable_large_circuit_passes: false,
        }
    }

    /// Sets the scheduling policy.
    #[must_use]
    pub const fn with_policy(mut self, policy: SchedulingPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// Sets the scheduler-local pass-count ceiling.
    #[must_use]
    pub const fn with_max_scheduled_passes(
        mut self,
        maximum: u64,
    ) -> Self {
        self.max_scheduled_passes = maximum;
        self
    }

    /// Sets the scheduler-local dependency-edge ceiling.
    #[must_use]
    pub const fn with_max_dependency_edges(
        mut self,
        maximum: u64,
    ) -> Self {
        self.max_dependency_edges = maximum;
        self
    }

    /// Sets the maximum number of passes in a wave.
    #[must_use]
    pub const fn with_max_wave_size(
        mut self,
        maximum: u64,
    ) -> Self {
        self.max_wave_size = maximum;
        self
    }

    /// Enables or disables strict large-circuit metadata validation.
    #[must_use]
    pub const fn reject_non_scalable_large_circuit_passes(
        mut self,
        reject: bool,
    ) -> Self {
        self.reject_non_scalable_large_circuit_passes = reject;
        self
    }

    /// Returns the scheduling policy.
    #[must_use]
    pub const fn policy(self) -> SchedulingPolicy {
        self.policy
    }

    /// Returns the scheduler-local pass limit.
    #[must_use]
    pub const fn max_scheduled_passes(self) -> u64 {
        self.max_scheduled_passes
    }

    /// Returns the scheduler-local dependency-edge limit.
    #[must_use]
    pub const fn max_dependency_edges(self) -> u64 {
        self.max_dependency_edges
    }

    /// Returns the maximum wave size.
    #[must_use]
    pub const fn max_wave_size(self) -> u64 {
        self.max_wave_size
    }

    /// Returns whether strict large-circuit metadata checking is enabled.
    #[must_use]
    pub const fn rejects_non_scalable_large_circuit_passes(
        self,
    ) -> bool {
        self.reject_non_scalable_large_circuit_passes
    }
}

// =============================================================================
// Scheduler errors
// =============================================================================

/// Errors produced while constructing or validating an optimization schedule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerError {
    /// No passes were supplied where a non-empty schedule was required.
    EmptySchedule,

    /// A pass pointer was null. This variant is not normally constructible
    /// from safe Rust and is retained as a semantic error category for future
    /// adapters.
    NullPass,

    /// A pass's metadata failed validation.
    InvalidPassMetadata {
        /// Pass identifier.
        pass_id: String,

        /// Validation message.
        message: String,
    },

    /// A pass identifier appeared more than once.
    DuplicatePassId {
        /// Duplicate identifier.
        pass_id: String,
    },

    /// The scheduler-local pass-count limit was exceeded.
    PassLimitExceeded {
        /// Number requested.
        requested: u64,

        /// Maximum permitted.
        maximum: u64,
    },

    /// The scheduler-local dependency-edge limit was exceeded.
    DependencyEdgeLimitExceeded {
        /// Number requested.
        requested: u64,

        /// Maximum permitted.
        maximum: u64,
    },

    /// The scheduler-local wave-size limit was exceeded.
    WaveSizeLimitExceeded {
        /// Wave index.
        wave: usize,

        /// Number of passes requested.
        requested: u64,

        /// Maximum permitted.
        maximum: u64,
    },

    /// A dependency graph contains a cycle.
    DependencyCycle {
        /// Passes participating in the unresolved cycle.
        pass_ids: Vec<String>,
    },

    /// A pass requires a capability/property that conflicts with its metadata.
    InvalidCapabilityCombination {
        /// Pass identifier.
        pass_id: String,

        /// Human-readable reason.
        message: String,
    },

    /// A pass declares a scope incompatible with a scheduling feature.
    UnsupportedScope {
        /// Pass identifier.
        pass_id: String,

        /// Scope name.
        scope: String,
    },

    /// A pass claims a level of scalability that its declared complexity does
    /// not support under strict validation.
    NonScalablePass {
        /// Pass identifier.
        pass_id: String,

        /// Complexity classification.
        complexity: String,
    },

    /// A schedule contained an invalid dependency index.
    InvalidDependencyIndex {
        /// Pass index.
        pass_index: usize,

        /// Invalid dependency index.
        dependency_index: usize,
    },

    /// A schedule failed internal consistency validation.
    InvalidSchedule {
        /// Human-readable reason.
        message: String,
    },
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySchedule => {
                formatter.write_str(
                    "optimization scheduler received an empty pass set",
                )
            }

            Self::NullPass => {
                formatter.write_str(
                    "optimization scheduler received a null pass",
                )
            }

            Self::InvalidPassMetadata { pass_id, message } => {
                write!(
                    formatter,
                    "invalid optimization pass metadata for `{pass_id}`: {message}"
                )
            }

            Self::DuplicatePassId { pass_id } => {
                write!(
                    formatter,
                    "duplicate optimization pass identifier `{pass_id}`"
                )
            }

            Self::PassLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "optimization scheduler pass limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::DependencyEdgeLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "optimization scheduler dependency-edge limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::WaveSizeLimitExceeded {
                wave,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "optimization scheduler wave {wave} exceeds maximum \
                     wave size: requested {requested}, maximum {maximum}"
                )
            }

            Self::DependencyCycle { pass_ids } => {
                write!(
                    formatter,
                    "optimization pass dependency cycle detected involving: {}",
                    pass_ids.join(", ")
                )
            }

            Self::InvalidCapabilityCombination {
                pass_id,
                message,
            } => {
                write!(
                    formatter,
                    "invalid scheduler capability metadata for `{pass_id}`: {message}"
                )
            }

            Self::UnsupportedScope { pass_id, scope } => {
                write!(
                    formatter,
                    "optimization pass `{pass_id}` has unsupported scheduling \
                     scope `{scope}`"
                )
            }

            Self::NonScalablePass {
                pass_id,
                complexity,
            } => {
                write!(
                    formatter,
                    "optimization pass `{pass_id}` is not considered scalable \
                     for large circuits because its declared complexity is `{complexity}`"
                )
            }

            Self::InvalidDependencyIndex {
                pass_index,
                dependency_index,
            } => {
                write!(
                    formatter,
                    "pass {pass_index} contains invalid dependency index \
                     {dependency_index}"
                )
            }

            Self::InvalidSchedule { message } => {
                write!(
                    formatter,
                    "invalid optimization schedule: {message}"
                )
            }
        }
    }
}

impl std::error::Error for SchedulerError {}

// =============================================================================
// Dependency reason
// =============================================================================

/// Explains why two passes must be ordered.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DependencyReason {
    /// The later pass requires an analysis invalidated by the earlier pass.
    AnalysisInvalidation {
        /// Analysis identifier.
        analysis: String,
    },

    /// A later pass requires a property invalidated by the earlier pass.
    PropertyInvalidation {
        /// Property identifier.
        property: String,
    },

    /// The earlier pass mutates the circuit and no explicit disjoint-region
    /// contract exists.
    CircuitMutation,

    /// One pass reorders operations and therefore acts as a conservative
    /// ordering barrier.
    OperationReordering,

    /// One pass can change qubit usage.
    QubitUsageChange,

    /// One pass introduces/eliminates ancillas.
    AncillaChange,

    /// One pass changes operation arity.
    ArityChange,

    /// One pass changes parameters.
    ParameterChange,

    /// One pass uses target-aware transformations.
    TargetAwareTransformation,

    /// One pass is stochastic.
    StochasticTransformation,

    /// One pass is a synthesis/resynthesis pass that may substantially replace
    /// circuit structure.
    Resynthesis,

    /// Original planner order is retained as a deterministic conservative
    /// barrier.
    StableInputOrder,
}

impl DependencyReason {
    /// Returns a stable identifier.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AnalysisInvalidation { .. } => "analysis_invalidation",
            Self::PropertyInvalidation { .. } => "property_invalidation",
            Self::CircuitMutation => "circuit_mutation",
            Self::OperationReordering => "operation_reordering",
            Self::QubitUsageChange => "qubit_usage_change",
            Self::AncillaChange => "ancilla_change",
            Self::ArityChange => "arity_change",
            Self::ParameterChange => "parameter_change",
            Self::TargetAwareTransformation => "target_aware_transformation",
            Self::StochasticTransformation => "stochastic_transformation",
            Self::Resynthesis => "resynthesis",
            Self::StableInputOrder => "stable_input_order",
        }
    }
}

impl fmt::Display for DependencyReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AnalysisInvalidation { analysis } => {
                write!(
                    formatter,
                    "{}({analysis})",
                    self.as_str()
                )
            }

            Self::PropertyInvalidation { property } => {
                write!(
                    formatter,
                    "{}({property})",
                    self.as_str()
                )
            }

            other => formatter.write_str(other.as_str()),
        }
    }
}

// =============================================================================
// Dependency edge
// =============================================================================

/// One directed dependency edge.
///
/// `before -> after` means `before` must be executed before `after`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DependencyEdge {
    before: usize,
    after: usize,
    reason: DependencyReason,
}

impl DependencyEdge {
    /// Creates an edge.
    #[must_use]
    pub const fn new(
        before: usize,
        after: usize,
        reason: DependencyReason,
    ) -> Self {
        Self {
            before,
            after,
            reason,
        }
    }

    /// Returns the predecessor index.
    #[must_use]
    pub const fn before(&self) -> usize {
        self.before
    }

    /// Returns the successor index.
    #[must_use]
    pub const fn after(&self) -> usize {
        self.after
    }

    /// Returns the dependency reason.
    #[must_use]
    pub const fn reason(&self) -> &DependencyReason {
        &self.reason
    }
}

// =============================================================================
// Schedule node
// =============================================================================

/// One scheduled pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledPass {
    /// Original pass index supplied by the planner.
    index: usize,

    /// Stable pass identifier.
    pass_id: String,

    /// Stable pass name.
    name: String,

    /// Declared complexity.
    complexity: PassComplexity,

    /// Declared determinism.
    determinism: PassDeterminism,

    /// Declared scope.
    scope: PassScope,

    /// Declared kind.
    kind: PassKind,

    /// Whether the pass may mutate circuit semantics.
    semantic_preserving: bool,

    /// Whether the pass is considered safe for fixed-point repetition.
    fixed_point_safe: bool,
}

impl ScheduledPass {
    /// Returns the planner-order index.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the pass identifier.
    #[must_use]
    pub fn pass_id(&self) -> &str {
        &self.pass_id
    }

    /// Returns the pass name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns declared complexity.
    #[must_use]
    pub const fn complexity(&self) -> PassComplexity {
        self.complexity
    }

    /// Returns declared determinism.
    #[must_use]
    pub const fn determinism(&self) -> PassDeterminism {
        self.determinism
    }

    /// Returns declared scope.
    #[must_use]
    pub const fn scope(&self) -> PassScope {
        self.scope
    }

    /// Returns declared pass kind.
    #[must_use]
    pub const fn kind(&self) -> PassKind {
        self.kind
    }

    /// Returns whether the pass declares semantic preservation.
    #[must_use]
    pub const fn semantic_preserving(&self) -> bool {
        self.semantic_preserving
    }

    /// Returns whether fixed-point repetition is declared safe.
    #[must_use]
    pub const fn fixed_point_safe(&self) -> bool {
        self.fixed_point_safe
    }
}

// =============================================================================
// Schedule wave
// =============================================================================

/// A deterministic wave of passes.
///
/// A wave means that the scheduler has determined that the listed passes do
/// not have an ordering dependency *under the scheduler's conservative
/// metadata model*.
///
/// It does NOT by itself authorize concurrent execution on one mutable
/// `QuantumCircuit`.
///
/// The pipeline/execution layer must still honor its execution policy.
///
/// In the current architecture, transformation passes remain serialized.
/// Analysis-only waves are the primary future candidate for parallel execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleWave {
    /// Stable wave number.
    index: usize,

    /// Passes in deterministic execution order within the wave.
    passes: Vec<ScheduledPass>,

    /// Whether the scheduler considers this wave safe for read-only parallel
    /// analysis execution.
    parallel_analysis_eligible: bool,
}

impl ScheduleWave {
    /// Returns the wave index.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns passes in this wave.
    #[must_use]
    pub fn passes(&self) -> &[ScheduledPass] {
        &self.passes
    }

    /// Returns true when the wave consists solely of scheduler-recognized
    /// analysis-only passes.
    #[must_use]
    pub const fn parallel_analysis_eligible(&self) -> bool {
        self.parallel_analysis_eligible
    }

    /// Returns the number of passes in this wave.
    #[must_use]
    pub fn len(&self) -> usize {
        self.passes.len()
    }

    /// Returns whether the wave is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.passes.is_empty()
    }
}

// =============================================================================
// Complete schedule
// =============================================================================

/// Complete deterministic optimization-pass schedule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizationSchedule {
    /// Scheduled waves.
    waves: Vec<ScheduleWave>,

    /// Directed dependency edges.
    edges: Vec<DependencyEdge>,

    /// Number of input passes.
    pass_count: usize,

    /// Number of dependency edges.
    edge_count: usize,
}

impl OptimizationSchedule {
    /// Creates a schedule from validated components.
    fn new(
        waves: Vec<ScheduleWave>,
        edges: Vec<DependencyEdge>,
        pass_count: usize,
    ) -> Self {
        Self {
            edge_count: edges.len(),
            waves,
            pass_count,
            edges,
        }
    }

    /// Returns all execution waves.
    #[must_use]
    pub fn waves(&self) -> &[ScheduleWave] {
        &self.waves
    }

    /// Returns all dependency edges.
    #[must_use]
    pub fn edges(&self) -> &[DependencyEdge] {
        &self.edges
    }

    /// Returns the number of scheduled passes.
    #[must_use]
    pub const fn pass_count(&self) -> usize {
        self.pass_count
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub const fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Returns the number of waves.
    #[must_use]
    pub fn wave_count(&self) -> usize {
        self.waves.len()
    }

    /// Returns whether the schedule contains no passes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pass_count == 0
    }

    /// Returns all passes in execution order.
    #[must_use]
    pub fn ordered_passes(&self) -> Vec<&ScheduledPass> {
        let mut result = Vec::with_capacity(self.pass_count);

        for wave in &self.waves {
            for pass in &wave.passes {
                result.push(pass);
            }
        }

        result
    }

    /// Validates internal schedule invariants.
    pub fn validate(&self) -> SchedulerResult<()> {
        if self.pass_count == 0 {
            if !self.waves.is_empty() {
                return Err(SchedulerError::InvalidSchedule {
                    message:
                        "empty schedule contains execution waves".to_string(),
                });
            }

            return Ok(());
        }

        let mut seen = BTreeSet::new();

        for wave in &self.waves {
            for pass in &wave.passes {
                if pass.index >= self.pass_count {
                    return Err(SchedulerError::InvalidSchedule {
                        message: format!(
                            "scheduled pass index {} is outside pass count {}",
                            pass.index,
                            self.pass_count
                        ),
                    });
                }

                if !seen.insert(pass.index) {
                    return Err(SchedulerError::InvalidSchedule {
                        message: format!(
                            "pass index {} appears more than once",
                            pass.index
                        ),
                    });
                }
            }
        }

        if seen.len() != self.pass_count {
            return Err(SchedulerError::InvalidSchedule {
                message: format!(
                    "schedule contains {} passes but expected {}",
                    seen.len(),
                    self.pass_count
                ),
            });
        }

        for edge in &self.edges {
            if edge.before >= self.pass_count {
                return Err(SchedulerError::InvalidDependencyIndex {
                    pass_index: edge.after,
                    dependency_index: edge.before,
                });
            }

            if edge.after >= self.pass_count {
                return Err(SchedulerError::InvalidDependencyIndex {
                    pass_index: edge.before,
                    dependency_index: edge.after,
                });
            }

            if edge.before == edge.after {
                return Err(SchedulerError::InvalidSchedule {
                    message: format!(
                        "pass {} depends on itself",
                        edge.before
                    ),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Scheduler
// =============================================================================

/// Production optimization-pass scheduler.
///
/// The scheduler is stateless and reusable across optimizer invocations.
#[derive(Debug, Clone, Copy)]
pub struct OptimizationScheduler {
    config: SchedulerConfig,
}

impl Default for OptimizationScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationScheduler {
    /// Creates the conservative production scheduler.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: SchedulerConfig::production(),
        }
    }

    /// Creates a scheduler with an explicit configuration.
    #[must_use]
    pub const fn with_config(config: SchedulerConfig) -> Self {
        Self { config }
    }

    /// Returns the scheduler configuration.
    #[must_use]
    pub const fn config(&self) -> SchedulerConfig {
        self.config
    }

    /// Builds a deterministic schedule from planner-selected passes.
    ///
    /// The input order is preserved as the primary deterministic tie-breaker.
    pub fn schedule<'a, I>(
        &self,
        passes: I,
    ) -> SchedulerResult<OptimizationSchedule>
    where
        I: IntoIterator<Item = &'a dyn OptimizationPass>,
    {
        let pass_list: Vec<&'a dyn OptimizationPass> =
            passes.into_iter().collect();

        self.validate_input(&pass_list)?;

        if pass_list.is_empty() {
            return Ok(OptimizationSchedule::new(
                Vec::new(),
                Vec::new(),
                0,
            ));
        }

        let edges = self.build_dependency_graph(&pass_list)?;
        let waves = self.build_waves(&pass_list, &edges)?;

        let schedule =
            OptimizationSchedule::new(waves, edges, pass_list.len());

        schedule.validate()?;

        Ok(schedule)
    }

    /// Validates planner-selected passes without constructing a schedule.
    pub fn validate_passes<'a, I>(
        &self,
        passes: I,
    ) -> SchedulerResult<()>
    where
        I: IntoIterator<Item = &'a dyn OptimizationPass>,
    {
        let pass_list: Vec<&'a dyn OptimizationPass> =
            passes.into_iter().collect();

        self.validate_input(&pass_list)
    }

    // =========================================================================
    // Input validation
    // =========================================================================

    fn validate_input(
        &self,
        passes: &[&dyn OptimizationPass],
    ) -> SchedulerResult<()> {
        let count = u64::try_from(passes.len()).map_err(|_| {
            SchedulerError::PassLimitExceeded {
                requested: u64::MAX,
                maximum: self.config.max_scheduled_passes,
            }
        })?;

        if self.config.max_scheduled_passes != 0
            && count > self.config.max_scheduled_passes
        {
            return Err(SchedulerError::PassLimitExceeded {
                requested: count,
                maximum: self.config.max_scheduled_passes,
            });
        }

        let mut identifiers = BTreeSet::new();

        for pass in passes {
            let metadata = pass.metadata();

            if let Err(error) = pass.validate() {
                return Err(SchedulerError::InvalidPassMetadata {
                    pass_id: pass.id().as_str().to_string(),
                    message: error.to_string(),
                });
            }

            if !identifiers.insert(pass.id().as_str().to_string()) {
                return Err(SchedulerError::DuplicatePassId {
                    pass_id: pass.id().as_str().to_string(),
                });
            }

            self.validate_capability_contract(*pass)?;

            if self
                .config
                .rejects_non_scalable_large_circuit_passes()
                && !metadata.supports_large_circuits()
            {
                return Err(SchedulerError::NonScalablePass {
                    pass_id: pass.id().as_str().to_string(),
                    complexity: metadata.complexity().as_str().to_string(),
                });
            }
        }

        Ok(())
    }

    fn validate_capability_contract(
        &self,
        pass: &dyn OptimizationPass,
    ) -> SchedulerResult<()> {
        let id = pass.id().as_str().to_string();

        let capabilities = pass.capabilities();

        let has_analysis_only =
            capabilities.contains(&PassCapability::AnalysisOnly);

        if pass.kind() == PassKind::Analysis && !has_analysis_only {
            return Err(
                SchedulerError::InvalidCapabilityCombination {
                    pass_id: id,
                    message:
                        "analysis passes must declare AnalysisOnly"
                            .to_string(),
                },
            );
        }

        if has_analysis_only
            && capabilities.iter().any(|capability| {
                matches!(
                    capability,
                    PassCapability::RemovesOperations
                        | PassCapability::AddsOperations
                        | PassCapability::ReplacesOperations
                        | PassCapability::ReordersOperations
                        | PassCapability::ChangesQubitUsage
                        | PassCapability::ChangesArity
                        | PassCapability::IntroducesAncillas
                        | PassCapability::EliminatesAncillas
                )
            })
        {
            return Err(
                SchedulerError::InvalidCapabilityCombination {
                    pass_id: id,
                    message:
                        "a pass marked AnalysisOnly cannot also declare \
                         circuit-mutating capabilities"
                            .to_string(),
                },
            );
        }

        if pass.determinism() == PassDeterminism::Seeded
            && !pass.has_capability(PassCapability::UsesRandomness)
        {
            return Err(
                SchedulerError::InvalidCapabilityCombination {
                    pass_id: pass.id().as_str().to_string(),
                    message:
                        "seeded passes must declare UsesRandomness"
                            .to_string(),
                },
            );
        }

        if pass.determinism() == PassDeterminism::Nondeterministic
            && pass.semantic_preserving()
            && !pass.has_capability(PassCapability::Approximate)
        {
            return Err(
                SchedulerError::InvalidCapabilityCombination {
                    pass_id: pass.id().as_str().to_string(),
                    message:
                        "nondeterministic semantic-preserving passes must \
                         declare Approximate"
                            .to_string(),
                },
            );
        }

        if pass.scope() == PassScope::Operation
            && pass.has_capability(PassCapability::ChangesQubitUsage)
        {
            return Err(
                SchedulerError::InvalidCapabilityCombination {
                    pass_id: pass.id().as_str().to_string(),
                    message:
                        "operation-scoped passes cannot declare global \
                         qubit-usage changes"
                            .to_string(),
                },
            );
        }

        Ok(())
    }

    // =========================================================================
    // Dependency graph
    // =========================================================================

    fn build_dependency_graph(
        &self,
        passes: &[&dyn OptimizationPass],
    ) -> SchedulerResult<Vec<DependencyEdge>> {
        let mut edges = Vec::new();

        /*
         * The scheduler deliberately uses pairwise metadata analysis rather
         * than maintaining a second dependency language.
         *
         * For a normal optimizer pipeline the number of passes is small enough
         * that O(P²) metadata comparison is inexpensive compared with actually
         * optimizing a quantum circuit.
         *
         * For very large generated pass sets, planner/registry should normally
         * reduce the candidate set before this stage. The explicit edge limit
         * prevents accidental graph explosion.
         */

        for after in 0..passes.len() {
            for before in 0..after {
                let before_pass = passes[before];
                let after_pass = passes[after];

                let reasons =
                    self.dependency_reasons(before_pass, after_pass);

                for reason in reasons {
                    self.push_edge(
                        &mut edges,
                        before,
                        after,
                        reason,
                    )?;
                }
            }
        }

        /*
         * Stable input order is itself a conservative dependency for any
         * transformation pair that could otherwise be reordered safely only
         * through a future explicit disjoint-region contract.
         *
         * This means the scheduler never silently changes the planner's
         * intended sequence merely because two transformations happen to have
         * different metadata today.
         */
        if self.config.policy() == SchedulingPolicy::Conservative {
            for after in 0..passes.len() {
                for before in 0..after {
                    if self.requires_stable_order_barrier(
                        passes[before],
                        passes[after],
                    ) {
                        self.push_edge(
                            &mut edges,
                            before,
                            after,
                            DependencyReason::StableInputOrder,
                        )?;
                    }
                }
            }
        }

        Ok(edges)
    }

    fn push_edge(
        &self,
        edges: &mut Vec<DependencyEdge>,
        before: usize,
        after: usize,
        reason: DependencyReason,
    ) -> SchedulerResult<()> {
        if before == after {
            return Err(SchedulerError::InvalidSchedule {
                message: format!(
                    "attempted to create self-dependency for pass {before}"
                ),
            });
        }

        if edges.iter().any(|edge| {
            edge.before == before && edge.after == after
        }) {
            return Ok(());
        }

        let next_count = edges.len().checked_add(1).ok_or_else(|| {
            SchedulerError::DependencyEdgeLimitExceeded {
                requested: u64::MAX,
                maximum: self.config.max_dependency_edges,
            }
        })?;

        let next_count_u64 = u64::try_from(next_count).map_err(|_| {
            SchedulerError::DependencyEdgeLimitExceeded {
                requested: u64::MAX,
                maximum: self.config.max_dependency_edges,
            }
        })?;

        if self.config.max_dependency_edges != 0
            && next_count_u64 > self.config.max_dependency_edges
        {
            return Err(
                SchedulerError::DependencyEdgeLimitExceeded {
                    requested: next_count_u64,
                    maximum: self.config.max_dependency_edges,
                },
            );
        }

        edges.push(DependencyEdge::new(
            before,
            after,
            reason,
        ));

        Ok(())
    }

    fn dependency_reasons(
        &self,
        before: &dyn OptimizationPass,
        after: &dyn OptimizationPass,
    ) -> Vec<DependencyReason> {
        let mut reasons = Vec::new();

        /*
         * A pass that changes the circuit can invalidate analyses/properties
         * required by a later pass.
         */
        for invalidation in before.effects().invalidated_analyses() {
            for requirement in after.requirements().analyses() {
                if invalidation.identifier()
                    == requirement.identifier()
                {
                    reasons.push(
                        DependencyReason::AnalysisInvalidation {
                            analysis: invalidation.identifier().to_string(),
                        },
                    );
                }
            }
        }

        for invalidation in before.effects().invalidated_properties() {
            for requirement in after.requirements().properties() {
                if invalidation.identifier()
                    == requirement.identifier()
                {
                    reasons.push(
                        DependencyReason::PropertyInvalidation {
                            property: invalidation
                                .identifier()
                                .as_str()
                                .to_string(),
                        },
                    );
                }
            }
        }

        /*
         * Circuit-mutating passes cannot be treated as commutative merely
         * because their metadata does not mention an analysis.
         *
         * This is the critical safety rule preventing the scheduler from
         * accidentally parallelizing two transformations that both receive
         * `&mut QuantumCircuit`.
         */
        if self.mutates_circuit(before) {
            reasons.push(DependencyReason::CircuitMutation);
        }

        if before.has_capability(
            PassCapability::ReordersOperations,
        ) {
            reasons.push(
                DependencyReason::OperationReordering,
            );
        }

        if before.has_capability(
            PassCapability::ChangesQubitUsage,
        ) {
            reasons.push(
                DependencyReason::QubitUsageChange,
            );
        }

        if before.has_capability(
            PassCapability::IntroducesAncillas,
        ) || before.has_capability(
            PassCapability::EliminatesAncillas,
        ) {
            reasons.push(
                DependencyReason::AncillaChange,
            );
        }

        if before.has_capability(
            PassCapability::ChangesArity,
        ) {
            reasons.push(
                DependencyReason::ArityChange,
            );
        }

        if before.has_capability(
            PassCapability::ChangesParameters,
        ) {
            reasons.push(
                DependencyReason::ParameterChange,
            );
        }

        if before.has_capability(
            PassCapability::TargetAware,
        ) {
            reasons.push(
                DependencyReason::TargetAwareTransformation,
            );
        }

        if before.has_capability(
            PassCapability::UsesRandomness,
        ) {
            reasons.push(
                DependencyReason::StochasticTransformation,
            );
        }

        if before.has_capability(
            PassCapability::UsesSynthesis,
        ) {
            reasons.push(
                DependencyReason::Resynthesis,
            );
        }

        reasons
    }

    fn requires_stable_order_barrier(
        &self,
        before: &dyn OptimizationPass,
        after: &dyn OptimizationPass,
    ) -> bool {
        /*
         * Analysis-only passes are the one category for which reordering is
         * generally safe, subject to requirements and cache semantics.
         */
        if self.is_analysis_only(before)
            && self.is_analysis_only(after)
        {
            return false;
        }

        /*
         * A transformation pass must remain in planner order unless an
         * explicit future disjoint-region capability exists.
         *
         * The current PassCapability contract deliberately does not contain
         * such a capability, so the conservative answer is true.
         */
        true
    }

    fn mutates_circuit(
        &self,
        pass: &dyn OptimizationPass,
    ) -> bool {
        if pass.has_capability(PassCapability::AnalysisOnly) {
            return false;
        }

        pass.has_capability(PassCapability::RemovesOperations)
            || pass.has_capability(PassCapability::AddsOperations)
            || pass.has_capability(
                PassCapability::ReplacesOperations,
            )
            || pass.has_capability(
                PassCapability::ReordersOperations,
            )
            || pass.has_capability(
                PassCapability::FusesOperations,
            )
            || pass.has_capability(
                PassCapability::DecomposesOperations,
            )
            || pass.has_capability(
                PassCapability::ChangesArity,
            )
            || pass.has_capability(
                PassCapability::IntroducesAncillas,
            )
            || pass.has_capability(
                PassCapability::EliminatesAncillas,
            )
            || pass.has_capability(
                PassCapability::ChangesQubitUsage,
            )
            || pass.has_capability(
                PassCapability::ChangesParameters,
            )
            || pass.kind() != PassKind::Analysis
    }

    fn is_analysis_only(
        &self,
        pass: &dyn OptimizationPass,
    ) -> bool {
        pass.kind() == PassKind::Analysis
            && pass.has_capability(
                PassCapability::AnalysisOnly,
            )
            && !self.mutates_circuit(pass)
    }

    // =========================================================================
    // Wave construction
    // =========================================================================

    fn build_waves(
        &self,
        passes: &[&dyn OptimizationPass],
        edges: &[DependencyEdge],
    ) -> SchedulerResult<Vec<ScheduleWave>> {
        let count = passes.len();

        let mut indegree = vec![0usize; count];

        let mut outgoing: Vec<Vec<usize>> =
            (0..count).map(|_| Vec::new()).collect();

        for edge in edges {
            if edge.before >= count {
                return Err(
                    SchedulerError::InvalidDependencyIndex {
                        pass_index: edge.after,
                        dependency_index: edge.before,
                    },
                );
            }

            if edge.after >= count {
                return Err(
                    SchedulerError::InvalidDependencyIndex {
                        pass_index: edge.before,
                        dependency_index: edge.after,
                    },
                );
            }

            indegree[edge.after] =
                indegree[edge.after]
                    .checked_add(1)
                    .ok_or_else(|| {
                        SchedulerError::InvalidSchedule {
                            message:
                                "dependency indegree overflow"
                                    .to_string(),
                        }
                    })?;

            outgoing[edge.before].push(edge.after);
        }

        /*
         * A BTreeSet makes the ready queue deterministic regardless of hash-map
         * implementation details.
         */
        let mut ready = BTreeSet::new();

        for index in 0..count {
            if indegree[index] == 0 {
                ready.insert(index);
            }
        }

        let mut waves = Vec::new();
        let mut scheduled = 0usize;

        while !ready.is_empty() {
            let mut current_wave = Vec::new();

            /*
             * Take a snapshot of currently ready passes. We then filter it
             * through the scheduling policy without mutating the dependency
             * graph during selection.
             */
            let ready_snapshot: Vec<usize> =
                ready.iter().copied().collect();

            for index in ready_snapshot {
                if !self.can_share_wave(
                    &current_wave,
                    index,
                    passes,
                ) {
                    continue;
                }

                if self.config.max_wave_size() != 0 {
                    let current_len =
                        u64::try_from(current_wave.len())
                            .map_err(|_| {
                                SchedulerError::WaveSizeLimitExceeded {
                                    wave: waves.len(),
                                    requested: u64::MAX,
                                    maximum:
                                        self.config.max_wave_size(),
                                }
                            })?;

                    if current_len
                        >= self.config.max_wave_size()
                    {
                        break;
                    }
                }

                current_wave.push(index);
            }

            /*
             * There must always be progress. If no pass can share the wave,
             * select the earliest ready pass alone.
             */
            if current_wave.is_empty() {
                if let Some(index) =
                    ready.iter().next().copied()
                {
                    current_wave.push(index);
                } else {
                    return Err(
                        SchedulerError::InvalidSchedule {
                            message:
                                "scheduler failed to select a ready pass"
                                    .to_string(),
                        },
                    );
                }
            }

            /*
             * Remove selected nodes from the ready set and release their
             * successors.
             */
            for index in &current_wave {
                ready.remove(index);
            }

            for index in &current_wave {
                scheduled = scheduled.checked_add(1).ok_or_else(|| {
                    SchedulerError::InvalidSchedule {
                        message:
                            "scheduled pass counter overflow"
                                .to_string(),
                    }
                })?;

                for successor in &outgoing[*index] {
                    indegree[*successor] =
                        indegree[*successor]
                            .checked_sub(1)
                            .ok_or_else(|| {
                                SchedulerError::InvalidSchedule {
                                    message:
                                        "dependency indegree underflow"
                                            .to_string(),
                                }
                            })?;

                    if indegree[*successor] == 0 {
                        ready.insert(*successor);
                    }
                }
            }

            let mut scheduled_passes = Vec::with_capacity(
                current_wave.len(),
            );

            for index in &current_wave {
                scheduled_passes.push(
                    self.scheduled_pass(passes[*index], *index),
                );
            }

            let parallel_analysis_eligible =
                scheduled_passes.iter().all(|pass| {
                    let original = passes[pass.index()];
                    self.is_analysis_only(original)
                });

            let wave_number = waves.len();

            waves.push(ScheduleWave {
                index: wave_number,
                passes: scheduled_passes,
                parallel_analysis_eligible,
            });
        }

        if scheduled != count {
            let mut unresolved = Vec::new();

            for index in 0..count {
                if indegree[index] != 0 {
                    unresolved.push(
                        passes[index].id().as_str().to_string(),
                    );
                }
            }

            return Err(SchedulerError::DependencyCycle {
                pass_ids: unresolved,
            });
        }

        Ok(waves)
    }

    fn can_share_wave(
        &self,
        current_wave: &[usize],
        candidate: usize,
        passes: &[&dyn OptimizationPass],
    ) -> bool {
        if current_wave.is_empty() {
            return true;
        }

        let candidate_pass = passes[candidate];

        match self.config.policy() {
            SchedulingPolicy::Conservative => false,

            SchedulingPolicy::AnalysisParallel
            | SchedulingPolicy::Aggressive => {
                /*
                 * Current architecture permits only analysis-only parallel
                 * waves. This is intentionally strict until a future pass
                 * capability can prove region disjointness.
                 */
                if !self.is_analysis_only(candidate_pass) {
                    return false;
                }

                for existing in current_wave {
                    if !self.is_analysis_only(
                        passes[*existing],
                    ) {
                        return false;
                    }

                    if !self.analysis_passes_can_share(
                        passes[*existing],
                        candidate_pass,
                    ) {
                        return false;
                    }
                }

                true
            }
        }
    }

    fn analysis_passes_can_share(
        &self,
        first: &dyn OptimizationPass,
        second: &dyn OptimizationPass,
    ) -> bool {
        /*
         * Both passes are read-only with respect to the circuit. However,
         * analysis caches live inside OptimizationContext and are mutable.
         *
         * Therefore this method only claims logical independence. Actual
         * concurrent execution still requires the pipeline/context layer to
         * provide independent analysis storage or synchronization.
         *
         * The current scheduler deliberately does not make that assumption.
         */
        if !self.is_analysis_only(first)
            || !self.is_analysis_only(second)
        {
            return false;
        }

        /*
         * If either pass invalidates something required by the other, they
         * cannot share a wave.
         */
        for invalidation in first.effects().invalidated_analyses() {
            if second
                .requirements()
                .analyses()
                .iter()
                .any(|requirement| {
                    requirement.identifier()
                        == invalidation.identifier()
                })
            {
                return false;
            }
        }

        for invalidation in second.effects().invalidated_analyses() {
            if first
                .requirements()
                .analyses()
                .iter()
                .any(|requirement| {
                    requirement.identifier()
                        == invalidation.identifier()
                })
            {
                return false;
            }
        }

        for invalidation in first.effects().invalidated_properties() {
            if second
                .requirements()
                .properties()
                .iter()
                .any(|requirement| {
                    requirement.identifier()
                        == invalidation.identifier()
                })
            {
                return false;
            }
        }

        for invalidation in second.effects().invalidated_properties() {
            if first
                .requirements()
                .properties()
                .iter()
                .any(|requirement| {
                    requirement.identifier()
                        == invalidation.identifier()
                })
            {
                return false;
            }
        }

        true
    }

    fn scheduled_pass(
        &self,
        pass: &dyn OptimizationPass,
        index: usize,
    ) -> ScheduledPass {
        ScheduledPass {
            index,
            pass_id: pass.id().as_str().to_string(),
            name: pass.name().to_string(),
            complexity: pass.complexity(),
            determinism: pass.determinism(),
            scope: pass.scope(),
            kind: pass.kind(),
            semantic_preserving: pass.semantic_preserving(),
            fixed_point_safe: pass.fixed_point_safe(),
        }
    }
}

// =============================================================================
// Standalone dependency graph builder
// =============================================================================

/// Builds only the dependency graph without creating execution waves.
///
/// This is useful for diagnostics, planner introspection, visualization, and
/// future incremental compilation.
pub fn build_dependency_graph<'a, I>(
    passes: I,
) -> SchedulerResult<Vec<DependencyEdge>>
where
    I: IntoIterator<Item = &'a dyn OptimizationPass>,
{
    OptimizationScheduler::new().build_dependency_graph(
        &passes.into_iter().collect::<Vec<_>>(),
    )
}

// =============================================================================
// Schedule validation helpers
// =============================================================================

/// Returns true when every pass in a schedule is strictly deterministic.
#[must_use]
pub fn schedule_is_deterministic(
    schedule: &OptimizationSchedule,
) -> bool {
    schedule
        .waves()
        .iter()
        .flat_map(|wave| wave.passes())
        .all(|pass| {
            pass.determinism() == PassDeterminism::Deterministic
        })
}

/// Returns true when every pass is reproducible with a seed if necessary.
#[must_use]
pub fn schedule_is_reproducible(
    schedule: &OptimizationSchedule,
) -> bool {
    schedule
        .waves()
        .iter()
        .flat_map(|wave| wave.passes())
        .all(|pass| {
            pass.determinism()
                .is_reproducible_with_seed()
        })
}

/// Returns the highest complexity declared by any scheduled pass.
#[must_use]
pub fn maximum_schedule_complexity(
    schedule: &OptimizationSchedule,
) -> Option<PassComplexity> {
    schedule
        .waves()
        .iter()
        .flat_map(|wave| wave.passes())
        .map(ScheduledPass::complexity)
        .max_by_key(|complexity| complexity.rank())
}

/// Returns the number of analysis-only waves.
#[must_use]
pub fn analysis_wave_count(
    schedule: &OptimizationSchedule,
) -> usize {
    schedule
        .waves()
        .iter()
        .filter(|wave| wave.parallel_analysis_eligible())
        .count()
}

/// Returns whether a schedule contains any stochastic pass.
#[must_use]
pub fn schedule_uses_randomness(
    schedule: &OptimizationSchedule,
) -> bool {
    schedule
        .waves()
        .iter()
        .flat_map(|wave| wave.passes())
        .any(|pass| {
            pass.determinism() == PassDeterminism::Seeded
                || pass.determinism()
                    == PassDeterminism::Nondeterministic
        })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::QuantumCircuit;
    use crate::quantum::optimization::errors::{
        OptimizationError,
        OptimizationResult,
    };
    use crate::quantum::optimization::pass::{
        AnalysisInvalidation,
        AnalysisRequirement,
        PassExecutionResult,
        PassMetadata,
        PassOutcome,
        PassRequirements,
        PassEffects,
        PropertyInvalidation,
        PropertyRequirement,
    };
    use crate::quantum::optimization::context::OptimizationContext;

    struct TestPass {
        metadata: PassMetadata,
    }

    impl TestPass {
        fn new(
            id: &str,
            kind: PassKind,
        ) -> Self {
            let pass_id =
                crate::quantum::optimization::errors::PassIdentifier::new(
                    id,
                )
                .expect("test pass identifier must be valid");

            let metadata =
                PassMetadata::new(
                    pass_id,
                    id,
                    kind,
                )
                .expect("test metadata must be valid");

            Self { metadata }
        }

        fn analysis(id: &str) -> Self {
            Self::new(id, PassKind::Analysis)
                .with_analysis_capability()
        }

        fn transform(id: &str) -> Self {
            Self::new(id, PassKind::LocalRewrite)
                .with_transform_capabilities()
        }
    }

    impl TestPass {
        fn with_analysis_capability(
            mut self,
        ) -> Self {
            self.metadata = self
                .metadata
                .with_capability(
                    PassCapability::AnalysisOnly,
                );
            self
        }

        fn with_transform_capabilities(
            mut self,
        ) -> Self {
            self.metadata = self
                .metadata
                .with_capabilities([
                    PassCapability::RemovesOperations,
                    PassCapability::ChangesGateCount,
                ]);
            self
        }

        fn require_analysis(
            mut self,
            identifier: &str,
        ) -> Self {
            let requirement =
                AnalysisRequirement::new(identifier)
                    .expect("analysis identifier valid");

            let mut requirements =
                PassRequirements::new();

            requirements
                .require_analysis(requirement);

            self.metadata =
                self.metadata
                    .with_requirements(requirements);

            self
        }

        fn invalidate_analysis(
            mut self,
            identifier: &str,
        ) -> Self {
            let invalidation =
                AnalysisInvalidation::new(identifier)
                    .expect("analysis identifier valid");

            let mut effects =
                PassEffects::new();

            effects.invalidate_analysis(
                invalidation,
            );

            self.metadata =
                self.metadata.with_effects(effects);

            self
        }

        fn require_property(
            mut self,
            identifier: &str,
        ) -> Self {
            let requirement =
                PropertyRequirement::new(identifier)
                    .expect("property identifier valid");

            let mut requirements =
                PassRequirements::new();

            requirements
                .require_property(requirement);

            self.metadata =
                self.metadata
                    .with_requirements(requirements);

            self
        }

        fn invalidate_property(
            mut self,
            identifier: &str,
        ) -> Self {
            let invalidation =
                PropertyInvalidation::new(identifier)
                    .expect("property identifier valid");

            let mut effects =
                PassEffects::new();

            effects.invalidate_property(
                invalidation,
            );

            self.metadata =
                self.metadata.with_effects(effects);

            self
        }
    }

    impl OptimizationPass for TestPass {
        fn metadata(&self) -> &PassMetadata {
            &self.metadata
        }

        fn run(
            &self,
            circuit: &mut QuantumCircuit,
            _context: &mut OptimizationContext,
        ) -> PassExecutionResult {
            let operations =
                circuit.operations().len();

            let operations =
                u64::try_from(operations)
                    .map_err(|_| {
                        OptimizationError::internal(
                            "test operation count overflow",
                        )
                    })?;

            Ok(PassOutcome::unchanged(
                operations,
                operations,
            ))
        }
    }

    #[test]
    fn empty_schedule_is_valid() {
        let scheduler =
            OptimizationScheduler::new();

        let schedule =
            scheduler
                .schedule(
                    std::iter::empty::<&dyn OptimizationPass>(),
                )
                .expect("empty schedule should be valid");

        assert!(schedule.is_empty());
        assert_eq!(schedule.pass_count(), 0);
    }

    #[test]
    fn transformation_passes_are_serialized() {
        let first =
            TestPass::transform("test.first");

        let second =
            TestPass::transform("test.second");

        let scheduler =
            OptimizationScheduler::new();

        let schedule =
            scheduler
                .schedule([
                    &first as &dyn OptimizationPass,
                    &second as &dyn OptimizationPass,
                ])
                .expect("schedule should succeed");

        assert_eq!(schedule.wave_count(), 2);
        assert_eq!(schedule.pass_count(), 2);
    }

    #[test]
    fn analysis_passes_can_share_analysis_parallel_wave() {
        let first =
            TestPass::analysis("test.analysis_a");

        let second =
            TestPass::analysis("test.analysis_b");

        let scheduler =
            OptimizationScheduler::with_config(
                SchedulerConfig::analysis_parallel(),
            );

        let schedule =
            scheduler
                .schedule([
                    &first as &dyn OptimizationPass,
                    &second as &dyn OptimizationPass,
                ])
                .expect("schedule should succeed");

        assert_eq!(schedule.wave_count(), 1);
        assert_eq!(
            schedule.waves()[0]
                .parallel_analysis_eligible(),
            true
        );
    }

    #[test]
    fn analysis_invalidation_creates_dependency() {
        let first =
            TestPass::analysis("test.producer")
                .invalidate_analysis("depth");

        let second =
            TestPass::analysis("test.consumer")
                .require_analysis("depth");

        let scheduler =
            OptimizationScheduler::with_config(
                SchedulerConfig::analysis_parallel(),
            );

        let schedule =
            scheduler
                .schedule([
                    &first as &dyn OptimizationPass,
                    &second as &dyn OptimizationPass,
                ])
                .expect("schedule should succeed");

        assert_eq!(schedule.wave_count(), 2);

        assert!(
            schedule.edges().iter().any(|edge| {
                edge.before() == 0
                    && edge.after() == 1
            })
        );
    }

    #[test]
    fn property_invalidation_creates_dependency() {
        let first =
            TestPass::analysis("test.property_producer")
                .invalidate_property("normalized");

        let second =
            TestPass::analysis("test.property_consumer")
                .require_property("normalized");

        let scheduler =
            OptimizationScheduler::with_config(
                SchedulerConfig::analysis_parallel(),
            );

        let schedule =
            scheduler
                .schedule([
                    &first as &dyn OptimizationPass,
                    &second as &dyn OptimizationPass,
                ])
                .expect("schedule should succeed");

        assert_eq!(schedule.wave_count(), 2);
    }

    #[test]
    fn deterministic_order_is_preserved() {
        let first =
            TestPass::transform("test.a");

        let second =
            TestPass::transform("test.b");

        let third =
            TestPass::transform("test.c");

        let scheduler =
            OptimizationScheduler::new();

        let schedule =
            scheduler
                .schedule([
                    &first as &dyn OptimizationPass,
                    &second as &dyn OptimizationPass,
                    &third as &dyn OptimizationPass,
                ])
                .expect("schedule should succeed");

        let ids: Vec<&str> = schedule
            .ordered_passes()
            .into_iter()
            .map(ScheduledPass::pass_id)
            .collect();

        assert_eq!(
            ids,
            vec![
                "test.a",
                "test.b",
                "test.c",
            ]
        );
    }

    #[test]
    fn schedule_is_deterministic_for_deterministic_passes() {
        let first =
            TestPass::transform("test.deterministic_a");

        let second =
            TestPass::transform("test.deterministic_b");

        let scheduler =
            OptimizationScheduler::new();

        let schedule =
            scheduler
                .schedule([
                    &first as &dyn OptimizationPass,
                    &second as &dyn OptimizationPass,
                ])
                .expect("schedule should succeed");

        assert!(
            schedule_is_deterministic(&schedule)
        );

        assert!(
            schedule_is_reproducible(&schedule)
        );
    }

    #[test]
    fn schedule_reports_maximum_complexity() {
        let first =
            TestPass::transform("test.linear")
                .with_complexity(
                    PassComplexity::Linear,
                );

        let second =
            TestPass::transform("test.quadratic")
                .with_complexity(
                    PassComplexity::Quadratic,
                );

        let scheduler =
            OptimizationScheduler::new();

        let schedule =
            scheduler
                .schedule([
                    &first as &dyn OptimizationPass,
                    &second as &dyn OptimizationPass,
                ])
                .expect("schedule should succeed");

        assert_eq!(
            maximum_schedule_complexity(&schedule),
            Some(PassComplexity::Quadratic)
        );
    }
}