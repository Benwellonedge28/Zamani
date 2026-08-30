//! Zamani Quantum Optimization — Gate Count Optimization Pass.
//!
//! Production-grade, backend-independent optimization of logical quantum
//! operation count over Zamani's canonical Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir::QuantumCircuit
//!                                    │
//!                                    ▼
//!                 optimization::passes::optimize_gate_count
//!                                    │
//!                         candidate generation
//!                                    │
//!                                    ▼
//!                       optimization::passes::simplify
//!                                    │
//!                   ┌────────────────┴────────────────┐
//!                   │                                 │
//!                   ▼                                 ▼
//!             exact rewrites                    exact validation
//!                   │                                 │
//!                   └────────────────┬────────────────┘
//!                                    ▼
//!                         canonical operation count
//!                                    │
//!                      ┌─────────────┴─────────────┐
//!                      │                           │
//!                      ▼                           ▼
//!                 strict decrease             no decrease
//!                      │                           │
//!                      ▼                           ▼
//!               atomic candidate             reject candidate
//!                      │
//!                      ▼
//!              optimized Quantum IR
//! ```
//!
//! # Purpose
//!
//! `OptimizeGateCount` owns the **gate-count objective and candidate
//! acceptance policy**.
//!
//! It does not own the individual quantum identities used to reduce the
//! circuit. Those transformations remain implemented by the optimization
//! subsystem's specialized passes.
//!
//! This separation is critical because gate-count optimization is an objective,
//! while cancellation, identity elimination, rotation fusion, commutation,
//! peephole rewriting, templates, and gate fusion are transformation
//! mechanisms.
//!
//! # Canonical representation
//!
//! The only circuit representation accepted by this pass is:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! This file intentionally does NOT define:
//!
//! - `QuantumGate`;
//! - `QuantumOperation`;
//! - another circuit representation;
//! - another qubit representation;
//! - another parameter representation.
//!
//! The canonical Quantum IR owns all quantum operation semantics.
//!
//! # Gate-count definition
//!
//! For this pass, logical gate count is the number of canonical Quantum IR
//! operations:
//!
//! ```text
//! circuit.len()
//! ```
//!
//! This is deliberately different from specialized metrics such as:
//!
//! - two-qubit gate count;
//! - T-count;
//! - circuit depth;
//! - T-depth;
//! - physical gate count;
//! - pulse count;
//! - execution duration.
//!
//! Those metrics belong to the common optimization cost/analysis subsystem.
//!
//! A circuit with fewer total operations is accepted by this pass even if one
//! of its other resource dimensions becomes worse. Multi-objective decisions
//! belong to the planner/cost model and must not be silently introduced here.
//!
//! # Strict monotonicity
//!
//! A candidate is accepted only when:
//!
//! ```text
//! candidate_gate_count < current_gate_count
//! ```
//!
//! Equal-size candidates are rejected.
//!
//! Larger candidates are rejected.
//!
//! This gives the pass a monotonic termination property:
//!
//! ```text
//! N0 > N1 > N2 > ... >= 0
//! ```
//!
//! Therefore successful iterations cannot oscillate.
//!
//! # Why this pass is composite
//!
//! The gate-count objective cannot safely be implemented by hard-coding a
//! collection of gate identities here.
//!
//! The canonical transformation pipeline already has specialized ownership
//! for:
//!
//! - identity elimination;
//! - inverse cancellation;
//! - self-inverse cancellation;
//! - rotation simplification;
//! - commutation;
//! - peephole rewriting;
//! - registered templates;
//! - gate fusion.
//!
//! `OptimizeGateCount` therefore delegates candidate generation to the
//! canonical simplification composite.
//!
//! This avoids creating a second source of truth for quantum identities.
//!
//! # Candidate transaction model
//!
//! The original circuit is never modified while a candidate is being built.
//!
//! ```text
//! original
//!    │
//!    ├── clone
//!    ▼
//! candidate
//!    │
//!    ├── exact simplification
//!    ├── validation
//!    ├── count operations
//!    └── strict comparison
//!           │
//!           ├── reject → original unchanged
//!           │
//!           └── accept → replace original
//! ```
//!
//! This means a candidate that fails validation or fails the objective cannot
//! partially corrupt the caller's circuit.
//!
//! The pipeline-level transaction policy remains authoritative for the whole
//! optimization pipeline.
//!
//! # Fixed-point behavior
//!
//! One invocation performs bounded fixed-point optimization.
//!
//! Each accepted iteration strictly reduces operation count.
//!
//! The loop stops when:
//!
//! 1. no candidate improves the gate count;
//! 2. the configured pass iteration limit is reached;
//! 3. the shared optimization resource budget is exhausted;
//! 4. cancellation is requested;
//! 5. a transformation reports an error.
//!
//! Because accepted iterations are strictly decreasing, this pass cannot
//! oscillate between equal-size or larger circuits.
//!
//! # Scaling
//!
//! This pass imposes no artificial maximum circuit size.
//!
//! Its working complexity is determined by:
//!
//! - the delegated simplification passes;
//! - circuit size;
//! - number of accepted iterations;
//! - canonical IR limits;
//! - optimization limits;
//! - available memory;
//! - available CPU;
//! - configured resource budgets.
//!
//! The implementation deliberately does not use recursive optimization.
//!
//! For a circuit with `N` operations and `I` accepted iterations, candidate
//! construction is approximately:
//!
//! ```text
//! O(I * simplification_work(N))
//! ```
//!
//! Candidate cloning requires O(N) additional memory for the active candidate.
//!
//! This is intentional. It provides a strong transaction boundary without
//! introducing unsafe memory manipulation.
//!
//! "Infinity" therefore means:
//!
//! > no artificial algorithmic circuit-size ceiling imposed by this pass;
//! > actual available memory, address space, IR limits, and optimizer resource
//! > policies remain authoritative.
//!
//! # Determinism
//!
//! The pass is deterministic.
//!
//! It does not use:
//!
//! - random numbers;
//! - wall-clock time;
//! - environment variables;
//! - filesystem state;
//! - network state;
//! - hardware state;
//! - global mutable state.
//!
//! Delegated transformations are responsible for their own determinism
//! guarantees through the common `OptimizationPass` contract.
//!
//! # Parallelism
//!
//! This pass does not spawn threads.
//!
//! Parallel pass scheduling belongs to:
//!
//! `optimization::scheduler`
//!
//! Candidate mutation is intentionally sequential because each accepted
//! candidate changes the objective baseline for the next iteration.
//!
//! # Semantic safety
//!
//! This pass never accepts a candidate merely because it has fewer operations.
//!
//! The candidate must first be produced by the canonical exact optimization
//! framework and must successfully validate as canonical Quantum IR.
//!
//! Whole-pipeline semantic equivalence remains owned by:
//!
//! `optimization::verification`
//!
//! A verified compilation profile should perform configured semantic
//! verification after this pass or after the complete optimization pipeline.
//!
//! # Measurement/reset/barrier semantics
//!
//! This pass does not directly remove measurements, resets, or barriers.
//!
//! It delegates transformations to the specialized local passes, which own
//! their respective semantic boundaries.
//!
//! In particular, an operation-count decrease is never sufficient authority
//! for this file to bypass a semantic boundary.
//!
//! # Target independence
//!
//! This pass is logical and backend-independent.
//!
//! It does not inspect:
//!
//! - hardware topology;
//! - coupling maps;
//! - native gate sets;
//! - pulse durations;
//! - calibration;
//! - QPU APIs.
//!
//! Target-aware optimization belongs to the target/synthesis/routing layers.
//!
//! # Multi-objective optimization
//!
//! This pass intentionally optimizes one objective:
//!
//! `CostMetric::GateCount`
//!
//! The common `optimization::cost` subsystem owns multi-objective comparisons.
//!
//! A caller that wants, for example:
//!
//! ```text
//! minimize gate count
//! subject to two-qubit count not increasing
//! ```
//!
//! should express that policy through the planner/cost system rather than
//! modifying this pass's acceptance rule.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! Input and output are always:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! No optimization-local IR is created.
//!
//! ## `optimization::pass`
//!
//! Implements:
//!
//! `OptimizationPass`
//!
//! Metadata declares:
//!
//! - composite optimization;
//! - circuit scope;
//! - deterministic execution;
//! - gate-count changes;
//! - operation removal;
//! - operation replacement;
//! - semantic preservation;
//! - large-circuit support;
//! - fixed-point safety.
//!
//! ## `optimization::context`
//!
//! Uses the invocation-scoped `OptimizationContext` for:
//!
//! - cancellation;
//! - iteration budgets;
//! - resource accounting;
//! - future shared optimizer services.
//!
//! This pass never creates process-global optimization state.
//!
//! ## `optimization::passes::simplify`
//!
//! Candidate generation is delegated to the canonical `Simplify` composite.
//!
//! `OptimizeGateCount` does not reach into private implementation details of
//! individual local passes.
//!
//! This makes the gate-count objective independent from the concrete set of
//! local transformations.
//!
//! ## `optimization::local`
//!
//! The local transformation family currently includes the canonical local
//! simplification stages:
//!
//! - identity;
//! - inverse;
//! - cancellation;
//! - rotation;
//! - commutation;
//! - peephole;
//! - templates;
//! - gate fusion.
//!
//! Those transformations remain owned by `optimization::local`.
//!
//! ## `optimization::cost`
//!
//! The common cost system owns:
//!
//! - `CostMetric::GateCount`;
//! - multi-objective cost vectors;
//! - weighted objectives;
//! - lexicographic objectives;
//! - Pareto comparisons.
//!
//! This pass uses the exact canonical operation count for its strict local
//! acceptance condition and does not duplicate the common cost-vector model.
//!
//! ## `optimization::statistics`
//!
//! This pass exposes compact pass-local statistics through
//! `GateCountOptimizationStatistics`.
//!
//! Global optimizer statistics remain owned by the shared statistics/context
//! infrastructure.
//!
//! ## `optimization::verification`
//!
//! This pass does not implement a second equivalence engine.
//!
//! Exact transformation passes and canonical validation protect local
//! correctness; whole-pipeline semantic verification remains external.
//!
//! ## `optimization::planner`
//!
//! The planner should select this pass for:
//!
//! - gate-count objectives;
//! - `OptimizationProfile::MinimumGateCount`;
//! - `OptimizationLevel::O2` / `O3` where appropriate;
//! - preprocessing before fault-tolerant optimization;
//! - generic logical simplification when gate count is the primary objective.
//!
//! ## `optimization::pipeline`
//!
//! Recommended logical position:
//!
//! ```text
//! normalize
//!     ↓
//! parameter simplification
//!     ↓
//! local simplification
//!     ↓
//! optimize_gate_count
//!     ↓
//! algebraic optimization
//!     ↓
//! synthesis
//!     ↓
//! routing
//!     ↓
//! scheduling
//! ```
//!
//! The planner may run this pass again after transformations that create new
//! gate-count opportunities.
//!
//! ## `routing`
//!
//! Routing remains downstream.
//!
//! This pass optimizes logical operation count before physical routing.
//!
//! Routing may introduce additional operations. A later optimization stage may
//! therefore be appropriate if the compiler policy permits post-routing
//! optimization.
//!
//! ## `scheduling`
//!
//! Scheduling owns execution timing and physical scheduling.
//!
//! This pass does not claim to minimize execution duration.
//!
//! ## `hardware`
//!
//! No hardware API is accessed.
//!
//! ## `benchmarking`
//!
//! Benchmarking consumes before/after statistics externally.
//!
//! This module does not depend on benchmarking.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe code;
//! - no additional dependencies.
//!
//! # Safety
//!
//! This module explicitly forbids unsafe Rust.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe operations are required.

#![forbid(unsafe_code)]

use crate::quantum::ir::QuantumCircuit;

use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationStage,
    PassIdentifier,
};
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassChange,
    PassComplexity,
    PassDeterminism,
    PassExecutionPolicy,
    PassKind,
    PassMetadata,
    PassMetadataError,
    PassOutcome,
    PassScope,
};
use super::simplify::Simplify;

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable machine-readable pass identifier.
///
/// This identifier is part of optimization provenance and must remain stable
/// across Rust type/file refactoring.
pub const PASS_ID: &str = "passes.optimize_gate_count";

/// Stable human-readable pass name.
pub const PASS_NAME: &str = "Logical Quantum Gate Count Optimization";

/// Public behavior/schema version of this pass.
pub const PASS_VERSION: u32 = 1;

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for gate-count optimization.
///
/// The configuration intentionally contains policy rather than individual gate
/// identities. Gate identities remain owned by the specialized optimization
/// passes.
///
/// This makes the file stable as additional local optimization algorithms are
/// added to Zamani.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GateCountOptimizationConfig {
    /// Maximum number of accepted optimization iterations.
    ///
    /// `0` disables transformation while retaining validation and measurement.
    ///
    /// `usize::MAX` means that this pass itself imposes no iteration ceiling;
    /// shared optimizer limits remain authoritative.
    pub max_iterations: usize,

    /// Whether the canonical `Simplify` composite should be used for candidate
    /// generation.
    ///
    /// This exists primarily for diagnostics/planner control. When disabled,
    /// the pass performs validation and gate-count measurement but does not
    /// transform the circuit.
    pub enable_simplification: bool,
}

impl GateCountOptimizationConfig {
    /// Creates the production configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_iterations: usize::MAX,
            enable_simplification: true,
        }
    }

    /// Creates a disabled configuration.
    ///
    /// The pass still validates and reports the circuit's gate count.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            max_iterations: 0,
            enable_simplification: false,
        }
    }

    /// Sets the maximum number of accepted optimization iterations.
    #[must_use]
    pub const fn with_max_iterations(
        mut self,
        max_iterations: usize,
    ) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Enables or disables candidate simplification.
    #[must_use]
    pub const fn with_simplification(
        mut self,
        enabled: bool,
    ) -> Self {
        self.enable_simplification = enabled;
        self
    }

    /// Returns whether transformations are enabled.
    #[must_use]
    pub const fn transformation_enabled(self) -> bool {
        self.enable_simplification && self.max_iterations != 0
    }
}

impl Default for GateCountOptimizationConfig {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Immutable statistics from one gate-count optimization invocation.
///
/// The values are intentionally compact and use `u64` for consistency with
/// the existing optimization pass result/accounting layer.
///
/// Conversion from `usize` is checked before statistics are emitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateCountOptimizationStatistics {
    /// Logical operation count before optimization.
    pub gate_count_before: u64,

    /// Logical operation count after optimization.
    pub gate_count_after: u64,

    /// Number of accepted optimization iterations.
    pub iterations: u64,

    /// Number of candidates constructed and evaluated.
    pub candidates_attempted: u64,

    /// Number of candidates rejected because they did not strictly reduce
    /// operation count.
    pub candidates_rejected: u64,
}

impl GateCountOptimizationStatistics {
    /// Returns true if gate count was reduced.
    #[must_use]
    pub const fn changed(self) -> bool {
        self.gate_count_after < self.gate_count_before
    }

    /// Returns the absolute number of operations removed.
    #[must_use]
    pub const fn gate_count_reduction(self) -> u64 {
        self.gate_count_before - self.gate_count_after
    }
}

// =============================================================================
// Pass
// =============================================================================

/// Production logical gate-count optimization pass.
///
/// The pass itself is stateless between invocations.
///
/// Invocation-specific state is owned by `OptimizationContext`.
#[derive(Debug, Clone)]
pub struct OptimizeGateCount {
    metadata: PassMetadata,
    config: GateCountOptimizationConfig,
}

impl OptimizeGateCount {
    /// Constructs the production gate-count optimizer.
    pub fn new() -> Result<Self, PassMetadataError> {
        Self::with_config(GateCountOptimizationConfig::default())
    }

    /// Constructs the optimizer with an explicit configuration.
    pub fn with_config(
        config: GateCountOptimizationConfig,
    ) -> Result<Self, PassMetadataError> {
        let identifier = PassIdentifier::from_static(PASS_ID)?;

        let metadata = PassMetadata::new(
            identifier,
            PASS_NAME,
            PassKind::Composite,
        )?
        .with_description(
            "Optimizes canonical logical quantum operation count using \
             exact simplification candidates and strict gate-count \
             improvement acceptance.",
        )?
        .with_scope(PassScope::Circuit)
        .with_complexity(PassComplexity::Quadratic)
        .with_determinism(PassDeterminism::Deterministic)
        .with_capabilities([
            PassCapability::RemovesOperations,
            PassCapability::ChangesGateCount,
            PassCapability::ChangesTwoQubitCount,
            PassCapability::ReordersOperations,
        ])
        .with_semantic_preservation(true)
        .supports_empty_circuit(true)
        .supports_single_operation(true)
        .supports_large_circuits(true)
        .requires_target(false)
        .requires_verification(false)
        .fixed_point_safe(true);

        metadata.validate()?;

        Ok(Self {
            metadata,
            config,
        })
    }

    /// Returns the stable pass identifier.
    #[must_use]
    pub const fn pass_id() -> &'static str {
        PASS_ID
    }

    /// Returns the stable pass name.
    #[must_use]
    pub const fn pass_name() -> &'static str {
        PASS_NAME
    }

    /// Returns the behavior/schema version.
    #[must_use]
    pub const fn pass_version() -> u32 {
        PASS_VERSION
    }

    /// Returns the active configuration.
    #[must_use]
    pub const fn config(&self) -> GateCountOptimizationConfig {
        self.config
    }

    /// Runs the pass using a standalone optimizer context.
    ///
    /// Compiler pipelines should normally call `run()` directly so that the
    /// shared context and resource budgets are preserved.
    pub fn optimize(
        &self,
        circuit: &mut QuantumCircuit,
    ) -> Result<PassOutcome, OptimizationError> {
        let mut context = OptimizationContext::standalone();

        self.run(circuit, &mut context)
    }

    /// Converts a platform operation count into the optimizer's `u64`
    /// accounting representation.
    fn checked_u64(
        value: usize,
        what: &'static str,
    ) -> Result<u64, OptimizationError> {
        u64::try_from(value).map_err(|_| {
            OptimizationError::internal(
                OptimizationStage::Analysis,
                format!(
                    "{PASS_ID}: {what} cannot be represented by \
                     optimizer u64 accounting"
                ),
            )
        })
    }

    /// Validates the canonical input circuit.
    fn validate_input(
        circuit: &QuantumCircuit,
    ) -> Result<(), OptimizationError> {
        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "{PASS_ID}: input Quantum IR validation failed: {error}"
                ),
            )
        })
    }

    /// Checks whether optimizer cancellation has been requested.
    fn check_cancelled(
        context: &mut OptimizationContext,
        stage: OptimizationStage,
        message: String,
    ) -> Result<(), OptimizationError> {
        context.check_cancelled().map_err(|error| {
            OptimizationError::resource_limit(
                stage,
                format!("{message}: {error}"),
            )
        })
    }

    /// Builds one optimization candidate.
    ///
    /// The source circuit remains untouched while the candidate is built.
    ///
    /// The candidate uses the canonical `Simplify` composite, which owns the
    /// actual exact local transformations.
    fn build_candidate(
        &self,
        source: &QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<QuantumCircuit, OptimizationError> {
        let mut candidate = source.clone();

        Self::check_cancelled(
            context,
            OptimizationStage::Rewrite,
            format!("{PASS_ID}: candidate construction cancelled"),
        )?;

        let simplify = Simplify::new().map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::Rewrite,
                format!(
                    "{PASS_ID}: failed to construct canonical \
                     simplification sub-pass: {error}"
                ),
            )
        })?;

        simplify.run(&mut candidate, context)?;

        Self::check_cancelled(
            context,
            OptimizationStage::Rewrite,
            format!(
                "{PASS_ID}: candidate construction cancelled after \
                 simplification"
            ),
        )?;

        candidate.validate().map_err(|error| {
            OptimizationError::rewrite_postcondition_failed(
                None,
                format!(
                    "{PASS_ID}: simplification candidate violates canonical \
                     Quantum IR invariants: {error}"
                ),
            )
        })?;

        Ok(candidate)
    }

    /// Executes the complete gate-count optimization.
    fn run_impl(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<
        (
            PassOutcome,
            GateCountOptimizationStatistics,
        ),
        OptimizationError,
    > {
        Self::validate_input(circuit)?;

        Self::check_cancelled(
            context,
            OptimizationStage::Analysis,
            format!(
                "{PASS_ID}: optimization cancelled before gate-count \
                 analysis"
            ),
        )?;

        let initial_count = Self::checked_u64(
            circuit.len(),
            "gate count before optimization",
        )?;

        let max_iterations = Self::checked_u64(
            self.config.max_iterations,
            "maximum gate-count optimization iterations",
        )?;

        let mut current_count = initial_count;
        let mut iterations = 0u64;
        let mut candidates_attempted = 0u64;
        let mut candidates_rejected = 0u64;

        // Empty and single-operation circuits cannot benefit from the
        // simplification strategy in any meaningful gate-count sense unless a
        // future transformation explicitly changes that assumption.
        //
        // We nevertheless validate and report them normally.
        if !self.config.transformation_enabled()
            || circuit.len() <= 1
        {
            let statistics = GateCountOptimizationStatistics {
                gate_count_before: initial_count,
                gate_count_after: initial_count,
                iterations: 0,
                candidates_attempted: 0,
                candidates_rejected: 0,
            };

            return Ok((
                PassOutcome::unchanged(
                    initial_count,
                    initial_count,
                ),
                statistics,
            ));
        }

        while iterations < max_iterations {
            Self::check_cancelled(
                context,
                OptimizationStage::Rewrite,
                format!(
                    "{PASS_ID}: optimization cancelled during iteration \
                     {iterations}"
                ),
            )?;

            context
                .charge_one(
                    super::super::limits::OptimizationResource::Iterations,
                )
                .map_err(|error| {
                    OptimizationError::resource_limit(
                        OptimizationStage::Rewrite,
                        format!(
                            "{PASS_ID}: optimization iteration budget \
                             exhausted: {error}"
                        ),
                    )
                })?;

            candidates_attempted =
                candidates_attempted.checked_add(1).ok_or_else(|| {
                    OptimizationError::internal(
                        OptimizationStage::Rewrite,
                        format!(
                            "{PASS_ID}: candidate counter overflow"
                        ),
                    )
                })?;

            let candidate = self.build_candidate(circuit, context)?;

            let candidate_count = Self::checked_u64(
                candidate.len(),
                "candidate gate count",
            )?;

            // Strict gate-count acceptance.
            //
            // This is the central invariant of this pass:
            //
            //     candidate_count < current_count
            //
            // Equal or larger candidates are discarded.
            if candidate_count >= current_count {
                candidates_rejected =
                    candidates_rejected
                        .checked_add(1)
                        .ok_or_else(|| {
                            OptimizationError::internal(
                                OptimizationStage::Rewrite,
                                format!(
                                    "{PASS_ID}: rejected-candidate \
                                     counter overflow"
                                ),
                            )
                        })?;

                break;
            }

            Self::check_cancelled(
                context,
                OptimizationStage::Rewrite,
                format!(
                    "{PASS_ID}: optimization cancelled before candidate \
                     commit"
                ),
            )?;

            // The candidate has:
            //
            // 1. been produced by canonical optimization passes;
            // 2. passed canonical IR validation;
            // 3. strictly fewer operations than the current circuit.
            //
            // Replacing the caller's circuit is therefore the atomic commit
            // point for this iteration.
            *circuit = candidate;

            current_count = candidate_count;

            iterations =
                iterations.checked_add(1).ok_or_else(|| {
                    OptimizationError::internal(
                        OptimizationStage::Rewrite,
                        format!(
                            "{PASS_ID}: accepted-iteration counter overflow"
                        ),
                    )
                })?;
        }

        // Always validate the final committed circuit. This is cheap compared
        // with transformation work and protects the public pass boundary.
        circuit.validate().map_err(|error| {
            OptimizationError::rewrite_postcondition_failed(
                None,
                format!(
                    "{PASS_ID}: final optimized Quantum IR validation \
                     failed: {error}"
                ),
            )
        })?;

        let final_count = Self::checked_u64(
            circuit.len(),
            "gate count after optimization",
        )?;

        // Monotonicity is a hard internal invariant. A bug here must fail
        // rather than silently returning a worse circuit.
        if final_count > initial_count {
            return Err(OptimizationError::internal(
                OptimizationStage::Rewrite,
                format!(
                    "{PASS_ID}: internal invariant violation: accepted \
                     optimization increased gate count from {initial_count} \
                     to {final_count}"
                ),
            ));
        }

        let statistics = GateCountOptimizationStatistics {
            gate_count_before: initial_count,
            gate_count_after: final_count,
            iterations,
            candidates_attempted,
            candidates_rejected,
        };

        if !statistics.changed() {
            return Ok((
                PassOutcome::no_improvement(
                    initial_count,
                    final_count,
                )
                .with_iterations(iterations),
                statistics,
            ));
        }

        let outcome = PassOutcome::changed(
            initial_count,
            final_count,
        )
        .with_change(PassChange::Changed)
        .with_iterations(iterations);

        Ok((outcome, statistics))
    }
}

// =============================================================================
// OptimizationPass implementation
// =============================================================================

impl OptimizationPass for OptimizeGateCount {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        let (outcome, _) = self.run_impl(circuit, context)?;

        Ok(outcome)
    }

    fn execution_policy(&self) -> PassExecutionPolicy {
        PassExecutionPolicy::StopWhenStable
    }
}

impl Default for OptimizeGateCount {
    fn default() -> Self {
        Self::new()
            .expect(
                "production gate-count optimization metadata \
                 must be valid",
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
    fn production_configuration_is_enabled() {
        let config = GateCountOptimizationConfig::default();

        assert!(config.transformation_enabled());
        assert!(config.enable_simplification);
        assert_eq!(config.max_iterations, usize::MAX);
    }

    #[test]
    fn disabled_configuration_has_no_transformations() {
        let config = GateCountOptimizationConfig::disabled();

        assert!(!config.transformation_enabled());
        assert!(!config.enable_simplification);
        assert_eq!(config.max_iterations, 0);
    }

    #[test]
    fn configuration_can_disable_simplification() {
        let config = GateCountOptimizationConfig::default()
            .with_simplification(false);

        assert!(!config.transformation_enabled());
        assert!(!config.enable_simplification);
    }

    #[test]
    fn configuration_can_bound_iterations() {
        let config =
            GateCountOptimizationConfig::default()
                .with_max_iterations(17);

        assert!(config.transformation_enabled());
        assert_eq!(config.max_iterations, 17);
    }

    #[test]
    fn statistics_report_only_strict_reductions_as_changes() {
        let unchanged = GateCountOptimizationStatistics {
            gate_count_before: 20,
            gate_count_after: 20,
            iterations: 0,
            candidates_attempted: 1,
            candidates_rejected: 1,
        };

        assert!(!unchanged.changed());
        assert_eq!(unchanged.gate_count_reduction(), 0);

        let changed = GateCountOptimizationStatistics {
            gate_count_before: 20,
            gate_count_after: 13,
            iterations: 2,
            candidates_attempted: 2,
            candidates_rejected: 0,
        };

        assert!(changed.changed());
        assert_eq!(changed.gate_count_reduction(), 7);
    }

    #[test]
    fn statistics_are_monotonic() {
        let statistics = GateCountOptimizationStatistics {
            gate_count_before: 100,
            gate_count_after: 1,
            iterations: 10,
            candidates_attempted: 10,
            candidates_rejected: 0,
        };

        assert!(statistics.changed());
        assert_eq!(statistics.gate_count_reduction(), 99);
        assert!(
            statistics.gate_count_after
                <= statistics.gate_count_before
        );
    }

    #[test]
    fn pass_constants_are_stable() {
        assert_eq!(
            OptimizeGateCount::pass_id(),
            "passes.optimize_gate_count"
        );

        assert_eq!(
            OptimizeGateCount::pass_name(),
            "Logical Quantum Gate Count Optimization"
        );

        assert_eq!(
            OptimizeGateCount::pass_version(),
            1
        );
    }

    #[test]
    fn production_pass_constructs() {
        let pass = OptimizeGateCount::new();

        assert!(pass.is_ok());

        let pass = pass.expect("production metadata should be valid");

        assert_eq!(
            pass.metadata().id().as_str(),
            PASS_ID
        );
    }
}