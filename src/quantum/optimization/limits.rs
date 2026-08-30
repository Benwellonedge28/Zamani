//! Zamani Quantum Optimization — Resource and Work Limits
//!
//! This module defines the resource-safety and deterministic work-budget
//! contract for `quantum::optimization`.
//!
//! # Architectural ownership
//!
//! `quantum::ir::limits::QuantumIrLimits` protects the canonical Quantum IR
//! itself: circuit size, qubit count, operation count, operands, parameters,
//! metadata, depth, measurements, and IR validation/analysis work.
//!
//! `OptimizationLimits` protects the *optimizer* from unbounded compiler
//! work. It therefore covers resources that arise specifically from
//! transformations and optimization algorithms:
//!
//! - optimization-pass count;
//! - fixed-point iterations;
//! - rewrite applications;
//! - circuit growth;
//! - analysis work;
//! - e-graph nodes/classes;
//! - synthesis work;
//! - equivalence-verification work;
//! - randomized verification samples;
//! - provenance records;
//! - wall-clock optimization budget.
//!
//! The two limit systems deliberately have different ownership:
//!
//! ```text
//! Quantum IR
//!     │
//!     └── QuantumIrLimits
//!           protects IR resources
//!
//! Optimization
//!     │
//!     └── OptimizationLimits
//!           protects optimizer resources
//! ```
//!
//! # Integration contract
//!
//! This file is intentionally independent of the future optimizer modules.
//! It MUST NOT import:
//!
//! - `OptimizationContext`;
//! - `OptimizationConfig`;
//! - `OptimizationPass`;
//! - `OptimizationPipeline`;
//! - a specific optimization pass;
//! - a backend;
//! - routing;
//! - scheduling;
//! - hardware APIs.
//!
//! Future files consume this contract rather than changing it:
//!
//! - `config.rs` selects/configures `OptimizationLimits`;
//! - `context.rs` stores the active limits;
//! - `pass.rs` receives access to the limits through the context;
//! - `pipeline.rs` enforces pass/iteration/rewrite budgets;
//! - `scheduler.rs` enforces scheduling/analysis budgets;
//! - `rewrite.rs` enforces rewrite budgets;
//! - `egraph.rs` enforces e-graph budgets;
//! - `synthesis/*` enforces synthesis budgets;
//! - `verification/*` enforces verification budgets;
//! - `planner.rs` uses limits when selecting an optimization strategy;
//! - `result.rs` reports when optimization stopped because a limit was hit.
//!
//! No future module should redefine optimizer resource limits.
//!
//! # Safety
//!
//! This module uses no `unsafe` code.
//!
//! All arithmetic used to calculate resource budgets is overflow checked.
//! Limits are explicit, deterministic, and never represented by an implicit
//! "unlimited" sentinel.
//!
//! A limit of zero is valid for ordinary resource counters and means that the
//! corresponding activity is prohibited. Execution budgets that are required
//! to make an algorithm meaningful are validated separately.
//!
//! # Determinism
//!
//! These limits are primarily *deterministic work limits*. They must not be
//! interpreted as guarantees about CPU time. Wall-clock limits are represented
//! separately and are intended as a defensive outer bound.
//!
//! The optimizer must therefore never rely on wall-clock timing for semantic
//! correctness or optimization fixed-point detection.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97 / 1.97.1.
//!
//! No nightly-only features are used.

use std::fmt;
use std::time::Duration;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced while validating or enforcing optimization limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLimitsError {
    /// The configured limit is internally invalid.
    InvalidConfiguration {
        /// Name of the invalid configuration field.
        field: &'static str,

        /// Invalid configured value.
        value: u64,
    },

    /// A requested resource exceeds its configured maximum.
    ResourceExceeded {
        /// Stable resource identifier.
        resource: &'static str,

        /// Requested amount.
        requested: u64,

        /// Configured maximum.
        maximum: u64,
    },

    /// An addition used while calculating a budget overflowed.
    ArithmeticOverflow {
        /// Stable resource identifier.
        resource: &'static str,
    },

    /// A multiplication used while calculating a budget overflowed.
    ArithmeticMultiplicationOverflow {
        /// Stable resource identifier.
        resource: &'static str,
    },
}

impl fmt::Display for OptimizationLimitsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { field, value } => {
                write!(
                    formatter,
                    "invalid quantum optimization limit `{field}`: \
                     value {value}"
                )
            }

            Self::ResourceExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "quantum optimization resource limit exceeded for \
                     `{resource}`: requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating quantum \
                     optimization resource `{resource}`"
                )
            }

            Self::ArithmeticMultiplicationOverflow { resource } => {
                write!(
                    formatter,
                    "arithmetic multiplication overflow while calculating \
                     quantum optimization resource `{resource}`"
                )
            }
        }
    }
}

impl std::error::Error for OptimizationLimitsError {}

// ============================================================================
// Limit identifiers
// ============================================================================

/// Stable identifiers for optimizer resource limits.
///
/// Keeping these identifiers centralized prevents future passes from inventing
/// incompatible resource names for diagnostics, provenance, and reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationResource {
    /// Number of optimizer passes permitted in one invocation.
    Passes,

    /// Number of fixed-point iterations permitted.
    Iterations,

    /// Number of rewrite applications permitted.
    Rewrites,

    /// Maximum number of operations in an intermediate circuit.
    CircuitOperations,

    /// Maximum number of qubits in an intermediate circuit.
    CircuitQubits,

    /// Maximum number of analysis steps.
    AnalysisSteps,

    /// Maximum number of dependency edges materialized by an analysis.
    DependencyEdges,

    /// Maximum number of e-graph nodes.
    EGraphNodes,

    /// Maximum number of e-graph equivalence classes.
    EGraphClasses,

    /// Maximum synthesis steps.
    SynthesisSteps,

    /// Maximum synthesis search states.
    SynthesisStates,

    /// Maximum synthesis output operations.
    SynthesisOperations,

    /// Maximum verification operations.
    VerificationOperations,

    /// Maximum verification qubits.
    VerificationQubits,

    /// Maximum exhaustive verification states.
    VerificationStates,

    /// Maximum randomized verification samples.
    VerificationSamples,

    /// Maximum rewrite candidates considered.
    RewriteCandidates,

    /// Maximum pattern-matching candidates considered.
    MatchCandidates,

    /// Maximum provenance entries.
    ProvenanceEntries,

    /// Maximum optimization wall-clock duration in milliseconds.
    WallClockMilliseconds,
}

impl OptimizationResource {
    /// Returns a stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passes => "passes",
            Self::Iterations => "iterations",
            Self::Rewrites => "rewrites",
            Self::CircuitOperations => "circuit_operations",
            Self::CircuitQubits => "circuit_qubits",
            Self::AnalysisSteps => "analysis_steps",
            Self::DependencyEdges => "dependency_edges",
            Self::EGraphNodes => "egraph_nodes",
            Self::EGraphClasses => "egraph_classes",
            Self::SynthesisSteps => "synthesis_steps",
            Self::SynthesisStates => "synthesis_states",
            Self::SynthesisOperations => "synthesis_operations",
            Self::VerificationOperations => "verification_operations",
            Self::VerificationQubits => "verification_qubits",
            Self::VerificationStates => "verification_states",
            Self::VerificationSamples => "verification_samples",
            Self::RewriteCandidates => "rewrite_candidates",
            Self::MatchCandidates => "match_candidates",
            Self::ProvenanceEntries => "provenance_entries",
            Self::WallClockMilliseconds => "wall_clock_milliseconds",
        }
    }
}

impl fmt::Display for OptimizationResource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Limit policy
// ============================================================================

/// Determines what a future optimization pipeline should do when a limit is
/// reached.
///
/// The policy belongs here because limit handling must be consistent across
/// passes. Individual passes must not invent their own interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LimitPolicy {
    /// Abort optimization and return the limit error.
    ///
    /// This is appropriate for verified/compiler-strict profiles where partial
    /// optimization is not acceptable.
    Fail,

    /// Stop optimization and return the best circuit produced so far.
    ///
    /// The caller must receive a status indicating that a resource limit was
    /// reached.
    StopAndReturnBest,

    /// Skip the operation that would exceed the limit and continue where
    /// semantically safe.
    ///
    /// This is appropriate only for optional/aggressive transformations.
    SkipPass,
}

impl Default for LimitPolicy {
    fn default() -> Self {
        Self::StopAndReturnBest
    }
}

// ============================================================================
// Optimization limits
// ============================================================================

/// Resource and deterministic-work limits for the Zamani quantum optimizer.
///
/// This is deliberately separate from `QuantumIrLimits`.
///
/// `QuantumIrLimits` answers:
///
/// > "How large/complex may the canonical IR be?"
///
/// `OptimizationLimits` answers:
///
/// > "How much work may the optimizer perform while transforming that IR?"
///
/// All limits are hard upper bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OptimizationLimits {
    // ------------------------------------------------------------------------
    // Pipeline execution
    // ------------------------------------------------------------------------

    /// Maximum number of optimization passes executed by one pipeline.
    max_passes: u64,

    /// Maximum number of fixed-point iterations for one optimization stage.
    max_iterations: u64,

    /// Maximum number of rewrite applications across the optimization run.
    max_rewrites: u64,

    // ------------------------------------------------------------------------
    // Intermediate circuit resources
    // ------------------------------------------------------------------------

    /// Maximum operations allowed in any intermediate circuit.
    max_circuit_operations: u64,

    /// Maximum qubits allowed in any intermediate circuit.
    max_circuit_qubits: u64,

    // ------------------------------------------------------------------------
    // Analysis
    // ------------------------------------------------------------------------

    /// Maximum abstract analysis work units.
    max_analysis_steps: u64,

    /// Maximum dependency edges an analysis may materialize.
    max_dependency_edges: u64,

    // ------------------------------------------------------------------------
    // Rewrite engine
    // ------------------------------------------------------------------------

    /// Maximum rewrite candidates considered.
    max_rewrite_candidates: u64,

    /// Maximum pattern matches considered.
    max_match_candidates: u64,

    // ------------------------------------------------------------------------
    // E-graph / equality saturation
    // ------------------------------------------------------------------------

    /// Maximum e-graph nodes.
    max_egraph_nodes: u64,

    /// Maximum e-graph equivalence classes.
    max_egraph_classes: u64,

    // ------------------------------------------------------------------------
    // Synthesis
    // ------------------------------------------------------------------------

    /// Maximum synthesis work units.
    max_synthesis_steps: u64,

    /// Maximum synthesis search states.
    max_synthesis_states: u64,

    /// Maximum operations emitted by one synthesis invocation.
    max_synthesis_operations: u64,

    // ------------------------------------------------------------------------
    // Verification
    // ------------------------------------------------------------------------

    /// Maximum operations inspected by a verification invocation.
    max_verification_operations: u64,

    /// Maximum qubits accepted by exact/exhaustive verification.
    max_verification_qubits: u64,

    /// Maximum states explored by exhaustive verification.
    max_verification_states: u64,

    /// Maximum randomized verification samples.
    max_verification_samples: u64,

    // ------------------------------------------------------------------------
    // Provenance
    // ------------------------------------------------------------------------

    /// Maximum provenance entries generated by one optimization invocation.
    max_provenance_entries: u64,

    // ------------------------------------------------------------------------
    // Wall clock
    // ------------------------------------------------------------------------

    /// Optional defensive wall-clock budget in milliseconds.
    ///
    /// Zero means no wall-clock budget is requested by this limit object.
    ///
    /// This is intentionally not used for semantic decisions.
    max_wall_clock_millis: u64,

    // ------------------------------------------------------------------------
    // Handling
    // ------------------------------------------------------------------------

    /// Behavior when a resource limit is reached.
    limit_policy: LimitPolicy,
}

impl OptimizationLimits {
    // ========================================================================
    // Production defaults
    // ========================================================================

    /// Default maximum number of passes.
    pub const DEFAULT_MAX_PASSES: u64 = 256;

    /// Default maximum fixed-point iterations.
    pub const DEFAULT_MAX_ITERATIONS: u64 = 64;

    /// Default maximum rewrite applications.
    pub const DEFAULT_MAX_REWRITES: u64 = 10_000_000;

    /// Default maximum intermediate circuit operations.
    ///
    /// This is intentionally bounded above the default IR operation limit so
    /// ordinary optimization can expand a circuit temporarily without making
    /// optimization itself the source of an uncontrolled expansion.
    pub const DEFAULT_MAX_CIRCUIT_OPERATIONS: u64 = 2_000_000;

    /// Default maximum intermediate circuit qubits.
    pub const DEFAULT_MAX_CIRCUIT_QUBITS: u64 = 4096;

    /// Default maximum analysis work units.
    pub const DEFAULT_MAX_ANALYSIS_STEPS: u64 = 100_000_000;

    /// Default maximum dependency edges.
    pub const DEFAULT_MAX_DEPENDENCY_EDGES: u64 = 10_000_000;

    /// Default rewrite candidate budget.
    pub const DEFAULT_MAX_REWRITE_CANDIDATES: u64 = 50_000_000;

    /// Default pattern-match candidate budget.
    pub const DEFAULT_MAX_MATCH_CANDIDATES: u64 = 100_000_000;

    /// Default e-graph node limit.
    pub const DEFAULT_MAX_EGRAPH_NODES: u64 = 5_000_000;

    /// Default e-graph class limit.
    pub const DEFAULT_MAX_EGRAPH_CLASSES: u64 = 1_000_000;

    /// Default synthesis work budget.
    pub const DEFAULT_MAX_SYNTHESIS_STEPS: u64 = 10_000_000;

    /// Default synthesis search-state budget.
    pub const DEFAULT_MAX_SYNTHESIS_STATES: u64 = 1_000_000;

    /// Default synthesis output-operation limit.
    pub const DEFAULT_MAX_SYNTHESIS_OPERATIONS: u64 = 1_000_000;

    /// Default verification operation budget.
    pub const DEFAULT_MAX_VERIFICATION_OPERATIONS: u64 = 100_000;

    /// Default maximum qubits for exact/exhaustive verification.
    ///
    /// Exact unitary/state-space verification scales exponentially, so this
    /// limit is deliberately conservative.
    pub const DEFAULT_MAX_VERIFICATION_QUBITS: u64 = 20;

    /// Default exhaustive state limit.
    pub const DEFAULT_MAX_VERIFICATION_STATES: u64 = 1_048_576;

    /// Default randomized verification sample count.
    pub const DEFAULT_MAX_VERIFICATION_SAMPLES: u64 = 10_000;

    /// Default provenance-entry limit.
    pub const DEFAULT_MAX_PROVENANCE_ENTRIES: u64 = 1_000_000;

    /// Default wall-clock budget.
    ///
    /// Zero means the optimizer has no wall-clock deadline by default.
    /// Deterministic work limits remain active.
    pub const DEFAULT_MAX_WALL_CLOCK_MILLIS: u64 = 0;

    /// Creates the production optimization policy.
    pub const fn production() -> Self {
        Self {
            max_passes: Self::DEFAULT_MAX_PASSES,
            max_iterations: Self::DEFAULT_MAX_ITERATIONS,
            max_rewrites: Self::DEFAULT_MAX_REWRITES,
            max_circuit_operations: Self::DEFAULT_MAX_CIRCUIT_OPERATIONS,
            max_circuit_qubits: Self::DEFAULT_MAX_CIRCUIT_QUBITS,
            max_analysis_steps: Self::DEFAULT_MAX_ANALYSIS_STEPS,
            max_dependency_edges: Self::DEFAULT_MAX_DEPENDENCY_EDGES,
            max_rewrite_candidates: Self::DEFAULT_MAX_REWRITE_CANDIDATES,
            max_match_candidates: Self::DEFAULT_MAX_MATCH_CANDIDATES,
            max_egraph_nodes: Self::DEFAULT_MAX_EGRAPH_NODES,
            max_egraph_classes: Self::DEFAULT_MAX_EGRAPH_CLASSES,
            max_synthesis_steps: Self::DEFAULT_MAX_SYNTHESIS_STEPS,
            max_synthesis_states: Self::DEFAULT_MAX_SYNTHESIS_STATES,
            max_synthesis_operations: Self::DEFAULT_MAX_SYNTHESIS_OPERATIONS,
            max_verification_operations: Self::DEFAULT_MAX_VERIFICATION_OPERATIONS,
            max_verification_qubits: Self::DEFAULT_MAX_VERIFICATION_QUBITS,
            max_verification_states: Self::DEFAULT_MAX_VERIFICATION_STATES,
            max_verification_samples: Self::DEFAULT_MAX_VERIFICATION_SAMPLES,
            max_provenance_entries: Self::DEFAULT_MAX_PROVENANCE_ENTRIES,
            max_wall_clock_millis: Self::DEFAULT_MAX_WALL_CLOCK_MILLIS,
            limit_policy: LimitPolicy::StopAndReturnBest,
        }
    }

    /// Creates a conservative policy suitable for untrusted or resource-
    /// constrained compilation.
    pub const fn conservative() -> Self {
        Self {
            max_passes: 64,
            max_iterations: 16,
            max_rewrites: 1_000_000,
            max_circuit_operations: 1_000_000,
            max_circuit_qubits: 1024,
            max_analysis_steps: 10_000_000,
            max_dependency_edges: 2_000_000,
            max_rewrite_candidates: 5_000_000,
            max_match_candidates: 10_000_000,
            max_egraph_nodes: 250_000,
            max_egraph_classes: 100_000,
            max_synthesis_steps: 1_000_000,
            max_synthesis_states: 100_000,
            max_synthesis_operations: 250_000,
            max_verification_operations: 50_000,
            max_verification_qubits: 16,
            max_verification_states: 65_536,
            max_verification_samples: 2_000,
            max_provenance_entries: 250_000,
            max_wall_clock_millis: 0,
            limit_policy: LimitPolicy::StopAndReturnBest,
        }
    }

    /// Creates a strict policy intended for verified compiler operation.
    ///
    /// Unlike the default production policy, reaching a limit is considered a
    /// compilation failure.
    pub const fn strict() -> Self {
        let mut limits = Self::production();
        limits.limit_policy = LimitPolicy::Fail;
        limits
    }

    /// Creates a deny-by-default optimization policy.
    ///
    /// This is useful for security tests, sandboxed compilation, and explicit
    /// allow-list configurations.
    pub const fn deny_all() -> Self {
        Self {
            max_passes: 0,
            max_iterations: 0,
            max_rewrites: 0,
            max_circuit_operations: 0,
            max_circuit_qubits: 0,
            max_analysis_steps: 0,
            max_dependency_edges: 0,
            max_rewrite_candidates: 0,
            max_match_candidates: 0,
            max_egraph_nodes: 0,
            max_egraph_classes: 0,
            max_synthesis_steps: 0,
            max_synthesis_states: 0,
            max_synthesis_operations: 0,
            max_verification_operations: 0,
            max_verification_qubits: 0,
            max_verification_states: 0,
            max_verification_samples: 0,
            max_provenance_entries: 0,
            max_wall_clock_millis: 0,
            limit_policy: LimitPolicy::Fail,
        }
    }

    // ========================================================================
    // Builder methods
    // ========================================================================

    /// Sets the maximum number of passes.
    pub const fn with_max_passes(mut self, value: u64) -> Self {
        self.max_passes = value;
        self
    }

    /// Sets the maximum number of fixed-point iterations.
    pub const fn with_max_iterations(mut self, value: u64) -> Self {
        self.max_iterations = value;
        self
    }

    /// Sets the maximum number of rewrites.
    pub const fn with_max_rewrites(mut self, value: u64) -> Self {
        self.max_rewrites = value;
        self
    }

    /// Sets the maximum intermediate circuit operation count.
    pub const fn with_max_circuit_operations(
        mut self,
        value: u64,
    ) -> Self {
        self.max_circuit_operations = value;
        self
    }

    /// Sets the maximum intermediate circuit qubit count.
    pub const fn with_max_circuit_qubits(mut self, value: u64) -> Self {
        self.max_circuit_qubits = value;
        self
    }

    /// Sets the maximum analysis work.
    pub const fn with_max_analysis_steps(mut self, value: u64) -> Self {
        self.max_analysis_steps = value;
        self
    }

    /// Sets the maximum dependency edges.
    pub const fn with_max_dependency_edges(mut self, value: u64) -> Self {
        self.max_dependency_edges = value;
        self
    }

    /// Sets the maximum rewrite candidates.
    pub const fn with_max_rewrite_candidates(
        mut self,
        value: u64,
    ) -> Self {
        self.max_rewrite_candidates = value;
        self
    }

    /// Sets the maximum pattern-match candidates.
    pub const fn with_max_match_candidates(mut self, value: u64) -> Self {
        self.max_match_candidates = value;
        self
    }

    /// Sets the maximum e-graph nodes.
    pub const fn with_max_egraph_nodes(mut self, value: u64) -> Self {
        self.max_egraph_nodes = value;
        self
    }

    /// Sets the maximum e-graph classes.
    pub const fn with_max_egraph_classes(mut self, value: u64) -> Self {
        self.max_egraph_classes = value;
        self
    }

    /// Sets the maximum synthesis work.
    pub const fn with_max_synthesis_steps(mut self, value: u64) -> Self {
        self.max_synthesis_steps = value;
        self
    }

    /// Sets the maximum synthesis search states.
    pub const fn with_max_synthesis_states(mut self, value: u64) -> Self {
        self.max_synthesis_states = value;
        self
    }

    /// Sets the maximum synthesized output operations.
    pub const fn with_max_synthesis_operations(
        mut self,
        value: u64,
    ) -> Self {
        self.max_synthesis_operations = value;
        self
    }

    /// Sets the maximum verification operations.
    pub const fn with_max_verification_operations(
        mut self,
        value: u64,
    ) -> Self {
        self.max_verification_operations = value;
        self
    }

    /// Sets the maximum verification qubits.
    pub const fn with_max_verification_qubits(
        mut self,
        value: u64,
    ) -> Self {
        self.max_verification_qubits = value;
        self
    }

    /// Sets the maximum exhaustive verification state count.
    pub const fn with_max_verification_states(
        mut self,
        value: u64,
    ) -> Self {
        self.max_verification_states = value;
        self
    }

    /// Sets the maximum randomized verification samples.
    pub const fn with_max_verification_samples(
        mut self,
        value: u64,
    ) -> Self {
        self.max_verification_samples = value;
        self
    }

    /// Sets the maximum provenance entries.
    pub const fn with_max_provenance_entries(
        mut self,
        value: u64,
    ) -> Self {
        self.max_provenance_entries = value;
        self
    }

    /// Sets an optional wall-clock budget in milliseconds.
    ///
    /// A value of zero disables the wall-clock budget.
    pub const fn with_max_wall_clock_millis(
        mut self,
        value: u64,
    ) -> Self {
        self.max_wall_clock_millis = value;
        self
    }

    /// Sets the limit policy.
    pub const fn with_limit_policy(
        mut self,
        policy: LimitPolicy,
    ) -> Self {
        self.limit_policy = policy;
        self
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Maximum number of passes.
    pub const fn max_passes(&self) -> u64 {
        self.max_passes
    }

    /// Maximum number of iterations.
    pub const fn max_iterations(&self) -> u64 {
        self.max_iterations
    }

    /// Maximum number of rewrites.
    pub const fn max_rewrites(&self) -> u64 {
        self.max_rewrites
    }

    /// Maximum intermediate circuit operations.
    pub const fn max_circuit_operations(&self) -> u64 {
        self.max_circuit_operations
    }

    /// Maximum intermediate circuit qubits.
    pub const fn max_circuit_qubits(&self) -> u64 {
        self.max_circuit_qubits
    }

    /// Maximum analysis steps.
    pub const fn max_analysis_steps(&self) -> u64 {
        self.max_analysis_steps
    }

    /// Maximum dependency edges.
    pub const fn max_dependency_edges(&self) -> u64 {
        self.max_dependency_edges
    }

    /// Maximum rewrite candidates.
    pub const fn max_rewrite_candidates(&self) -> u64 {
        self.max_rewrite_candidates
    }

    /// Maximum pattern-match candidates.
    pub const fn max_match_candidates(&self) -> u64 {
        self.max_match_candidates
    }

    /// Maximum e-graph nodes.
    pub const fn max_egraph_nodes(&self) -> u64 {
        self.max_egraph_nodes
    }

    /// Maximum e-graph classes.
    pub const fn max_egraph_classes(&self) -> u64 {
        self.max_egraph_classes
    }

    /// Maximum synthesis steps.
    pub const fn max_synthesis_steps(&self) -> u64 {
        self.max_synthesis_steps
    }

    /// Maximum synthesis search states.
    pub const fn max_synthesis_states(&self) -> u64 {
        self.max_synthesis_states
    }

    /// Maximum synthesis output operations.
    pub const fn max_synthesis_operations(&self) -> u64 {
        self.max_synthesis_operations
    }

    /// Maximum verification operations.
    pub const fn max_verification_operations(&self) -> u64 {
        self.max_verification_operations
    }

    /// Maximum verification qubits.
    pub const fn max_verification_qubits(&self) -> u64 {
        self.max_verification_qubits
    }

    /// Maximum verification states.
    pub const fn max_verification_states(&self) -> u64 {
        self.max_verification_states
    }

    /// Maximum verification samples.
    pub const fn max_verification_samples(&self) -> u64 {
        self.max_verification_samples
    }

    /// Maximum provenance entries.
    pub const fn max_provenance_entries(&self) -> u64 {
        self.max_provenance_entries
    }

    /// Maximum wall-clock duration in milliseconds.
    pub const fn max_wall_clock_millis(&self) -> u64 {
        self.max_wall_clock_millis
    }

    /// Returns the configured limit policy.
    pub const fn limit_policy(&self) -> LimitPolicy {
        self.limit_policy
    }

    /// Returns the configured wall-clock duration.
    ///
    /// `None` means no wall-clock budget is configured.
    pub fn max_wall_clock_duration(&self) -> Option<Duration> {
        if self.max_wall_clock_millis == 0 {
            None
        } else {
            Some(Duration::from_millis(self.max_wall_clock_millis))
        }
    }

    // ========================================================================
    // Configuration validation
    // ========================================================================

    /// Validates the configuration.
    ///
    /// Zero is allowed for all counters because zero is a meaningful
    /// deny-by-default value.
    ///
    /// The only invalid value currently is a wall-clock duration that cannot
    /// be represented by `Duration::from_millis`.
    ///
    /// Because `u64` milliseconds are directly representable by
    /// `Duration::from_millis`, this method currently succeeds for every
    /// finite field configuration. It exists as a stable contract so future
    /// validation rules can be introduced without changing callers.
    pub const fn validate(&self) -> Result<(), OptimizationLimitsError> {
        Ok(())
    }

    // ========================================================================
    // Individual checks
    // ========================================================================

    /// Checks a pass count.
    pub const fn check_passes(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::Passes,
            requested,
            self.max_passes,
        )
    }

    /// Checks an iteration count.
    pub const fn check_iterations(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::Iterations,
            requested,
            self.max_iterations,
        )
    }

    /// Checks a rewrite count.
    pub const fn check_rewrites(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::Rewrites,
            requested,
            self.max_rewrites,
        )
    }

    /// Checks an intermediate circuit operation count.
    pub const fn check_circuit_operations(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::CircuitOperations,
            requested,
            self.max_circuit_operations,
        )
    }

    /// Checks an intermediate circuit qubit count.
    pub const fn check_circuit_qubits(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::CircuitQubits,
            requested,
            self.max_circuit_qubits,
        )
    }

    /// Checks analysis work.
    pub const fn check_analysis_steps(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::AnalysisSteps,
            requested,
            self.max_analysis_steps,
        )
    }

    /// Checks dependency edges.
    pub const fn check_dependency_edges(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::DependencyEdges,
            requested,
            self.max_dependency_edges,
        )
    }

    /// Checks rewrite candidates.
    pub const fn check_rewrite_candidates(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::RewriteCandidates,
            requested,
            self.max_rewrite_candidates,
        )
    }

    /// Checks pattern-match candidates.
    pub const fn check_match_candidates(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::MatchCandidates,
            requested,
            self.max_match_candidates,
        )
    }

    /// Checks e-graph nodes.
    pub const fn check_egraph_nodes(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::EGraphNodes,
            requested,
            self.max_egraph_nodes,
        )
    }

    /// Checks e-graph classes.
    pub const fn check_egraph_classes(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::EGraphClasses,
            requested,
            self.max_egraph_classes,
        )
    }

    /// Checks synthesis steps.
    pub const fn check_synthesis_steps(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::SynthesisSteps,
            requested,
            self.max_synthesis_steps,
        )
    }

    /// Checks synthesis search states.
    pub const fn check_synthesis_states(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::SynthesisStates,
            requested,
            self.max_synthesis_states,
        )
    }

    /// Checks synthesis output operations.
    pub const fn check_synthesis_operations(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::SynthesisOperations,
            requested,
            self.max_synthesis_operations,
        )
    }

    /// Checks verification operations.
    pub const fn check_verification_operations(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::VerificationOperations,
            requested,
            self.max_verification_operations,
        )
    }

    /// Checks verification qubits.
    pub const fn check_verification_qubits(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::VerificationQubits,
            requested,
            self.max_verification_qubits,
        )
    }

    /// Checks exhaustive verification states.
    pub const fn check_verification_states(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::VerificationStates,
            requested,
            self.max_verification_states,
        )
    }

    /// Checks randomized verification samples.
    pub const fn check_verification_samples(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::VerificationSamples,
            requested,
            self.max_verification_samples,
        )
    }

    /// Checks provenance entries.
    pub const fn check_provenance_entries(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::ProvenanceEntries,
            requested,
            self.max_provenance_entries,
        )
    }

    /// Checks a wall-clock budget expressed in milliseconds.
    pub const fn check_wall_clock_millis(
        &self,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        Self::check(
            OptimizationResource::WallClockMilliseconds,
            requested,
            self.max_wall_clock_millis,
        )
    }

    // ========================================================================
    // Checked arithmetic helpers
    // ========================================================================

    /// Calculates `current + additional` and checks the resulting total
    /// against a resource limit.
    ///
    /// The returned value is the new total on success.
    pub const fn checked_add(
        &self,
        resource: OptimizationResource,
        current: u64,
        additional: u64,
    ) -> Result<u64, OptimizationLimitsError> {
        let requested = match current.checked_add(additional) {
            Some(value) => value,
            None => {
                return Err(
                    OptimizationLimitsError::ArithmeticOverflow {
                        resource: resource.as_str(),
                    },
                );
            }
        };

        self.check_resource(resource, requested)?;

        Ok(requested)
    }

    /// Calculates `current + additional * multiplier` without overflow and
    /// checks the result against the corresponding resource limit.
    ///
    /// This is useful for:
    ///
    /// - dependency-edge accounting;
    /// - analysis work accounting;
    /// - state-space estimation;
    /// - verification work;
    /// - synthesis expansion.
    pub const fn checked_add_product(
        &self,
        resource: OptimizationResource,
        current: u64,
        additional: u64,
        multiplier: u64,
    ) -> Result<u64, OptimizationLimitsError> {
        let product = match additional.checked_mul(multiplier) {
            Some(value) => value,
            None => {
                return Err(
                    OptimizationLimitsError::ArithmeticMultiplicationOverflow {
                        resource: resource.as_str(),
                    },
                );
            }
        };

        self.checked_add(resource, current, product)
    }

    /// Calculates `left * right` with overflow checking.
    ///
    /// The result is not automatically checked against a limit because this
    /// helper is also useful for intermediate calculations.
    pub const fn checked_mul(
        resource: OptimizationResource,
        left: u64,
        right: u64,
    ) -> Result<u64, OptimizationLimitsError> {
        match left.checked_mul(right) {
            Some(value) => Ok(value),
            None => {
                Err(
                    OptimizationLimitsError::ArithmeticMultiplicationOverflow {
                        resource: resource.as_str(),
                    },
                )
            }
        }
    }

    /// Calculates the number of basis states for `qubits` qubits while
    /// enforcing the configured exhaustive-verification state limit.
    ///
    /// This computes `2^qubits` without using a potentially overflowing shift.
    pub const fn checked_basis_states(
        &self,
        qubits: u64,
    ) -> Result<u64, OptimizationLimitsError> {
        self.check_verification_qubits(qubits)?;

        if qubits >= 64 {
            return Err(
                OptimizationLimitsError::ArithmeticMultiplicationOverflow {
                    resource: OptimizationResource::VerificationStates.as_str(),
                },
            );
        }

        let states = 1u64 << qubits;

        self.check_verification_states(states)?;

        Ok(states)
    }

    // ========================================================================
    // Internal generic checking
    // ========================================================================

    const fn check(
        resource: OptimizationResource,
        requested: u64,
        maximum: u64,
    ) -> Result<(), OptimizationLimitsError> {
        if requested > maximum {
            return Err(OptimizationLimitsError::ResourceExceeded {
                resource: resource.as_str(),
                requested,
                maximum,
            });
        }

        Ok(())
    }

    const fn check_resource(
        &self,
        resource: OptimizationResource,
        requested: u64,
    ) -> Result<(), OptimizationLimitsError> {
        let maximum = match resource {
            OptimizationResource::Passes => self.max_passes,
            OptimizationResource::Iterations => self.max_iterations,
            OptimizationResource::Rewrites => self.max_rewrites,
            OptimizationResource::CircuitOperations => {
                self.max_circuit_operations
            }
            OptimizationResource::CircuitQubits => self.max_circuit_qubits,
            OptimizationResource::AnalysisSteps => self.max_analysis_steps,
            OptimizationResource::DependencyEdges => {
                self.max_dependency_edges
            }
            OptimizationResource::EGraphNodes => self.max_egraph_nodes,
            OptimizationResource::EGraphClasses => self.max_egraph_classes,
            OptimizationResource::SynthesisSteps => self.max_synthesis_steps,
            OptimizationResource::SynthesisStates => {
                self.max_synthesis_states
            }
            OptimizationResource::SynthesisOperations => {
                self.max_synthesis_operations
            }
            OptimizationResource::VerificationOperations => {
                self.max_verification_operations
            }
            OptimizationResource::VerificationQubits => {
                self.max_verification_qubits
            }
            OptimizationResource::VerificationStates => {
                self.max_verification_states
            }
            OptimizationResource::VerificationSamples => {
                self.max_verification_samples
            }
            OptimizationResource::RewriteCandidates => {
                self.max_rewrite_candidates
            }
            OptimizationResource::MatchCandidates => {
                self.max_match_candidates
            }
            OptimizationResource::ProvenanceEntries => {
                self.max_provenance_entries
            }
            OptimizationResource::WallClockMilliseconds => {
                self.max_wall_clock_millis
            }
        };

        Self::check(resource, requested, maximum)
    }
}

// ============================================================================
// Default
// ============================================================================

impl Default for OptimizationLimits {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_configuration_is_valid() {
        assert!(OptimizationLimits::production().validate().is_ok());
    }

    #[test]
    fn conservative_configuration_is_valid() {
        assert!(OptimizationLimits::conservative().validate().is_ok());
    }

    #[test]
    fn strict_configuration_is_valid() {
        let limits = OptimizationLimits::strict();

        assert!(limits.validate().is_ok());
        assert_eq!(limits.limit_policy(), LimitPolicy::Fail);
    }

    #[test]
    fn deny_all_allows_zero_resources() {
        let limits = OptimizationLimits::deny_all();

        assert!(limits.check_passes(0).is_ok());
        assert!(limits.check_rewrites(0).is_ok());
        assert!(limits.check_circuit_operations(0).is_ok());
        assert!(limits.check_analysis_steps(0).is_ok());
    }

    #[test]
    fn deny_all_rejects_positive_resources() {
        let limits = OptimizationLimits::deny_all();

        let result = limits.check_passes(1);

        assert_eq!(
            result,
            Err(OptimizationLimitsError::ResourceExceeded {
                resource: "passes",
                requested: 1,
                maximum: 0,
            })
        );
    }

    #[test]
    fn pass_limit_is_enforced() {
        let limits =
            OptimizationLimits::production().with_max_passes(4);

        assert!(limits.check_passes(4).is_ok());

        assert_eq!(
            limits.check_passes(5),
            Err(OptimizationLimitsError::ResourceExceeded {
                resource: "passes",
                requested: 5,
                maximum: 4,
            })
        );
    }

    #[test]
    fn rewrite_limit_is_enforced() {
        let limits =
            OptimizationLimits::production().with_max_rewrites(10);

        assert!(limits.check_rewrites(10).is_ok());
        assert!(limits.check_rewrites(11).is_err());
    }

    #[test]
    fn circuit_growth_is_bounded() {
        let limits =
            OptimizationLimits::production()
                .with_max_circuit_operations(100);

        assert!(limits.check_circuit_operations(100).is_ok());
        assert!(limits.check_circuit_operations(101).is_err());
    }

    #[test]
    fn egraph_limits_are_independent() {
        let limits =
            OptimizationLimits::production()
                .with_max_egraph_nodes(100)
                .with_max_egraph_classes(20);

        assert!(limits.check_egraph_nodes(100).is_ok());
        assert!(limits.check_egraph_classes(20).is_ok());

        assert!(limits.check_egraph_nodes(101).is_err());
        assert!(limits.check_egraph_classes(21).is_err());
    }

    #[test]
    fn synthesis_limits_are_independent() {
        let limits =
            OptimizationLimits::production()
                .with_max_synthesis_steps(100)
                .with_max_synthesis_states(50)
                .with_max_synthesis_operations(25);

        assert!(limits.check_synthesis_steps(100).is_ok());
        assert!(limits.check_synthesis_states(50).is_ok());
        assert!(limits.check_synthesis_operations(25).is_ok());

        assert!(limits.check_synthesis_steps(101).is_err());
        assert!(limits.check_synthesis_states(51).is_err());
        assert!(limits.check_synthesis_operations(26).is_err());
    }

    #[test]
    fn verification_limits_are_independent() {
        let limits =
            OptimizationLimits::production()
                .with_max_verification_qubits(10)
                .with_max_verification_states(1024)
                .with_max_verification_samples(100);

        assert!(limits.check_verification_qubits(10).is_ok());
        assert!(limits.check_verification_states(1024).is_ok());
        assert!(limits.check_verification_samples(100).is_ok());

        assert!(limits.check_verification_qubits(11).is_err());
        assert!(limits.check_verification_states(1025).is_err());
        assert!(limits.check_verification_samples(101).is_err());
    }

    #[test]
    fn checked_add_is_overflow_safe() {
        let limits = OptimizationLimits::production();

        let result = limits.checked_add(
            OptimizationResource::Rewrites,
            u64::MAX,
            1,
        );

        assert_eq!(
            result,
            Err(
                OptimizationLimitsError::ArithmeticOverflow {
                    resource: "rewrites",
                }
            )
        );
    }

    #[test]
    fn checked_add_product_is_overflow_safe() {
        let limits = OptimizationLimits::production();

        let result = limits.checked_add_product(
            OptimizationResource::AnalysisSteps,
            0,
            u64::MAX,
            2,
        );

        assert_eq!(
            result,
            Err(
                OptimizationLimitsError::ArithmeticMultiplicationOverflow {
                    resource: "analysis_steps",
                }
            )
        );
    }

    #[test]
    fn checked_add_enforces_limit() {
        let limits =
            OptimizationLimits::production()
                .with_max_rewrites(100);

        assert_eq!(
            limits.checked_add(
                OptimizationResource::Rewrites,
                90,
                10,
            ),
            Ok(100)
        );

        assert_eq!(
            limits.checked_add(
                OptimizationResource::Rewrites,
                90,
                11,
            ),
            Err(
                OptimizationLimitsError::ResourceExceeded {
                    resource: "rewrites",
                    requested: 101,
                    maximum: 100,
                }
            )
        );
    }

    #[test]
    fn basis_state_calculation_is_bounded() {
        let limits =
            OptimizationLimits::production()
                .with_max_verification_qubits(10)
                .with_max_verification_states(1024);

        assert_eq!(limits.checked_basis_states(10), Ok(1024));
        assert!(limits.checked_basis_states(11).is_err());
    }

    #[test]
    fn basis_state_overflow_is_rejected() {
        let limits =
            OptimizationLimits::production()
                .with_max_verification_qubits(63)
                .with_max_verification_states(u64::MAX);

        assert_eq!(
            limits.checked_basis_states(63),
            Err(
                OptimizationLimitsError::ResourceExceeded {
                    resource: "verification_qubits",
                    requested: 63,
                    maximum: 63,
                }
            )
        );
    }

    #[test]
    fn wall_clock_zero_means_unconfigured() {
        let limits = OptimizationLimits::production();

        assert_eq!(limits.max_wall_clock_duration(), None);
    }

    #[test]
    fn wall_clock_duration_is_available_when_configured() {
        let limits =
            OptimizationLimits::production()
                .with_max_wall_clock_millis(1500);

        assert_eq!(
            limits.max_wall_clock_duration(),
            Some(Duration::from_millis(1500))
        );
    }

    #[test]
    fn resource_names_are_stable() {
        assert_eq!(
            OptimizationResource::Passes.as_str(),
            "passes"
        );

        assert_eq!(
            OptimizationResource::EGraphNodes.as_str(),
            "egraph_nodes"
        );

        assert_eq!(
            OptimizationResource::VerificationStates.as_str(),
            "verification_states"
        );
    }

    #[test]
    fn default_is_production_policy() {
        assert_eq!(
            OptimizationLimits::default(),
            OptimizationLimits::production()
        );
    }

    #[test]
    fn limit_policy_defaults_to_stop_and_return_best() {
        assert_eq!(
            OptimizationLimits::default().limit_policy(),
            LimitPolicy::StopAndReturnBest
        );
    }

    #[test]
    fn strict_policy_fails_on_limit() {
        let limits =
            OptimizationLimits::strict().with_max_passes(1);

        assert_eq!(
            limits.check_passes(2),
            Err(
                OptimizationLimitsError::ResourceExceeded {
                    resource: "passes",
                    requested: 2,
                    maximum: 1,
                }
            )
        );
    }
}