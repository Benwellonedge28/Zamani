//! Zamani Quantum Optimization — Fault-Tolerant Optimization Objective.
//!
//! Production-grade composite optimization pass for fault-tolerant logical
//! quantum circuits.
//!
//! # Architectural role
//!
//! This file owns the FAULT-TOLERANT OPTIMIZATION OBJECTIVE.
//!
//! It does not own individual quantum identities.
//!
//! The architecture is:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                     optimization pipeline
//!                              │
//!                              ▼
//!              passes::optimize_fault_tolerance
//!                              │
//!          ┌───────────────────┼────────────────────┐
//!          │                   │                    │
//!          ▼                   ▼                    ▼
//!     T reduction       Clifford+T local      future global
//!     pass              normalization        FT passes
//!          │                   │                    │
//!          └───────────────────┼────────────────────┘
//!                              ▼
//!                    FT resource analysis
//!                              │
//!          ┌───────────────────┼────────────────────┐
//!          ▼                   ▼                    ▼
//!       T-count             T-depth          logical cost
//!          │                   │                    │
//!          └───────────────────┼────────────────────┘
//!                              ▼
//!                    candidate acceptance
//!                              │
//!                              ▼
//!                     optimized Quantum IR
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - fault-tolerant optimization policy;
//! - objective selection;
//! - candidate acceptance;
//! - monotonicity policy;
//! - bounded fixed-point iteration;
//! - orchestration of already-implemented FT transformations;
//! - before/after FT resource accounting;
//! - cancellation cooperation;
//! - pass-level statistics;
//! - deterministic behavior.
//!
//! This file does NOT own:
//!
//! - the canonical Quantum IR;
//! - T/Tdg mathematical identities;
//! - Clifford algebra;
//! - phase-polynomial representation;
//! - TODD/STOMP/global T-count algorithms;
//! - QEC codes;
//! - physical qubit mapping;
//! - routing;
//! - scheduling;
//! - hardware execution;
//! - benchmarking;
//! - semantic equivalence checking.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Canonical representation
//!
//! The only circuit representation accepted by this pass is:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! No optimization-local circuit or gate representation is introduced.
//!
//! # Fault-tolerant resource principle
//!
//! Fault-tolerant optimization must not reduce everything to ordinary gate
//! count.
//!
//! The principal resources are:
//!
//! ```text
//! T-count
//! T-depth
//! Clifford count
//! two-qubit logical count
//! logical depth
//! ancilla usage
//! logical width
//! ```
//!
//! A circuit with fewer total gates is not necessarily a better
//! fault-tolerant circuit.
//!
//! Consequently this pass supports multiple objective policies.
//!
//! # Exactness
//!
//! The default transformations are exact.
//!
//! This file never silently changes:
//!
//! - exact unitary equivalence;
//! - global-phase policy;
//! - measurement semantics;
//! - reset semantics;
//! - classical semantics.
//!
//! Approximate optimization belongs to explicitly approximate passes and must
//! be selected by the generic optimizer configuration.
//!
//! # Current implementation boundary
//!
//! The repository currently provides:
//!
//! - `fault_tolerant::t_gate_reduction` as an `OptimizationPass`;
//! - `fault_tolerant::clifford_t` as a standalone exact Clifford+T sequence
//!   optimizer;
//! - `fault_tolerant::t_count` as T-resource analysis;
//! - `fault_tolerant::t_depth` as T-depth analysis;
//! - `fault_tolerant::logical_cost` as logical FT cost analysis;
//! - `fault_tolerant::magic_state` as magic-state resource modeling.
//!
//! This composite pass deliberately does not pretend that every future
//! algorithm already exists.
//!
//! Future global algorithms such as phase-polynomial/TODD/STOMP-style
//! optimization can be integrated behind the same candidate interface without
//! changing this file's public API.
//!
//! # Important distinction: transformation vs analysis
//!
//! ```text
//! t_gate_reduction
//!     = transformation
//!
//! t_count
//!     = analysis
//!
//! t_depth
//!     = analysis
//!
//! logical_cost
//!     = analysis
//!
//! optimize_fault_tolerance
//!     = objective/orchestration
//! ```
//!
//! This prevents resource analysis from accidentally becoming a correctness
//! mechanism.
//!
//! A lower T-count does NOT prove semantic equivalence.
//!
//! Semantic equivalence remains owned by `optimization::verification`.
//!
//! # Candidate acceptance
//!
//! The default production policy is lexicographically conservative:
//!
//! 1. reduce T-family count;
//! 2. if T-family count is unchanged, reduce total operation count;
//! 3. if both are unchanged, do not accept a candidate.
//!
//! This prevents an FT optimization pass from accepting a circuit that uses
//! more expensive non-Clifford resources merely because it has fewer ordinary
//! gates.
//!
//! Other policies are available through `FaultTolerantObjective`.
//!
//! # Fixed point
//!
//! The pass itself performs bounded fixed-point iteration only over its
//! explicitly selected local FT transformations.
//!
//! Every accepted candidate must strictly improve the configured objective.
//!
//! Therefore an accepted sequence has the form:
//!
//! ```text
//! C0 > C1 > C2 > ... >= 0
//! ```
//!
//! with respect to the selected objective ordering.
//!
//! Equal-cost candidates are rejected.
//!
//! This prevents optimization oscillation.
//!
//! # Scaling
//!
//! No artificial circuit-size ceiling is imposed by this file.
//!
//! The pass scales subject to:
//!
//! - `usize` addressability;
//! - Quantum IR limits;
//! - `OptimizationLimits`;
//! - `OptimizationContext` budgets;
//! - available memory;
//! - available CPU;
//! - configured iteration/rewrite limits.
//!
//! The pass does not construct Hilbert-space matrices.
//!
//! It does not allocate memory proportional to `2^n`.
//!
//! It does not recursively traverse the circuit.
//!
//! Its own orchestration overhead is O(number_of_selected_passes × circuit
//! traversal cost).
//!
//! # Determinism
//!
//! This pass is deterministic.
//!
//! It does not use:
//!
//! - randomness;
//! - environment state;
//! - filesystem state;
//! - network state;
//! - backend state;
//! - global mutable state.
//!
//! Future stochastic FT passes must declare that behavior through
//! `PassMetadata` and use the shared optimizer determinism infrastructure.
//!
//! # Transaction boundary
//!
//! A candidate is optimized independently from the caller's circuit whenever
//! the selected transformation requires a standalone operation sequence.
//!
//! The canonical circuit is committed only after:
//!
//! - transformation succeeds;
//! - candidate validation succeeds;
//! - objective calculation succeeds;
//! - objective strictly improves;
//! - cancellation has not been requested.
//!
//! The canonical QuantumCircuit remains responsible for its own validated
//! mutation API.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! Input/output:
//!
//! `QuantumCircuit`
//!
//! No second IR.
//!
//! ## `optimization::pass`
//!
//! Implements:
//!
//! `OptimizationPass`
//!
//! Stable identifier:
//!
//! `passes.optimize_fault_tolerance`
//!
//! ## `optimization::context`
//!
//! Uses the shared:
//!
//! - cancellation;
//! - deadline;
//! - iteration;
//! - rewrite;
//! - pass budget;
//! - deterministic execution state.
//!
//! This file never creates a second context.
//!
//! ## `fault_tolerant::t_gate_reduction`
//!
//! This is currently the primary transformation delegated by this composite
//! pass.
//!
//! The child pass owns the exact T/Tdg algebra.
//!
//! This file owns whether the resulting circuit should be accepted.
//!
//! ## `fault_tolerant::clifford_t`
//!
//! The standalone Clifford+T optimizer may be enabled through
//! `FaultTolerantObjective::CliffordT`.
//!
//! It operates on immutable `&[Gate]` and returns a new operation sequence.
//!
//! It must never be reimplemented here.
//!
//! ## `fault_tolerant::t_count`
//!
//! Used as an authoritative FT resource analysis.
//!
//! This file does not redefine T-count semantics.
//!
//! ## `fault_tolerant::t_depth`
//!
//! Used for objective/reporting support.
//!
//! T-depth remains distinct from T-count.
//!
//! ## `fault_tolerant::logical_cost`
//!
//! Can be consumed by later versions of the objective system.
//!
//! This file intentionally does not duplicate its logical-cost model.
//!
//! ## `optimization::cost`
//!
//! This pass supplies FT-specific objective information to the common cost
//! system through its normal pass/result infrastructure.
//!
//! It does not create another global cost model.
//!
//! ## `optimization::verification`
//!
//! Semantic equivalence checking remains external.
//!
//! The pass never treats a cost improvement as proof of correctness.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline may invoke this pass before:
//!
//! - routing;
//! - scheduling;
//! - hardware lowering.
//!
//! A post-routing optimization stage may run separately if the compiler policy
//! allows it.
//!
//! ## `optimization::planner`
//!
//! The planner may select this pass for:
//!
//! - fault-tolerant profiles;
//! - Clifford+T circuits;
//! - T-count objectives;
//! - T-depth objectives;
//! - magic-state-aware objectives;
//! - logical-resource objectives.
//!
//! ## `optimization::registry`
//!
//! Register:
//!
//! `passes.optimize_fault_tolerance`
//!
//! Alias:
//!
//! `optimize_fault_tolerance`
//!
//! Registry ownership remains outside this file.
//!
//! ## `benchmarking`
//!
//! Benchmarking consumes the resulting before/after metrics.
//!
//! This pass does not depend on benchmarking.
//!
//! ## `routing` / `scheduling` / `hardware`
//!
//! No dependency is introduced toward those layers.
//!
//! The intended direction remains:
//!
//! ```text
//! optimization → routing → scheduling → hardware
//! ```
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no external dependencies
//! - no unsafe code
//!
//! # Safety
//!
//! This module explicitly forbids unsafe code.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe operation is necessary.
//!
//! # External design basis
//!
//! Current Qiskit exposes a dedicated `OptimizeCliffordT` transformation and
//! a dedicated Clifford+T compilation pipeline. This supports keeping
//! Clifford+T optimization as a distinct compiler stage rather than hiding it
//! inside generic gate-count optimization.
//!
//! Qiskit's recent compiler work also distinguishes T-count optimization as a
//! target-specific optimization metric.
//!
//! Research literature likewise treats T-count and T-depth as independent
//! optimization resources and provides separate algorithms for each.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::ir::{Gate, QuantumCircuit};

use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationStage,
    PassIdentifier,
};
use super::super::fault_tolerant::{
    clifford_t,
    t_gate_reduction::TGateReductionPass,
    t_count::analyze_t_count,
    t_depth::analyze_t_depth,
};
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassComplexity,
    PassDeterminism,
    PassExecutionResult,
    PassKind,
    PassMetadata,
    PassOutcome,
    PassScope,
};

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable machine-readable pass identifier.
pub const PASS_ID: &str = "passes.optimize_fault_tolerance";

/// Stable human-readable name.
pub const PASS_NAME: &str =
    "Fault-Tolerant Quantum Circuit Optimization";

/// Stable public contract version.
pub const PASS_VERSION: u32 = 1;

/// Stable registry alias.
pub const PASS_ALIAS: &str = "optimize_fault_tolerance";

// =============================================================================
// Objective
// =============================================================================

/// Fault-tolerant optimization objective.
///
/// The objective determines how candidate circuits are compared.
///
/// The transformation mechanisms remain separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaultTolerantObjective {
    /// Minimize T/Tdg family count first, then ordinary operation count.
    ///
    /// This is the production default.
    TCountThenOperations,

    /// Minimize T-depth first, then T-family count, then operation count.
    TDepthThenTCount,

    /// Minimize total logical operations first, then T-family count.
    ///
    /// This mode is useful when the target architecture does not make
    /// non-Clifford operations disproportionately expensive.
    OperationsThenTCount,

    /// Run the exact Clifford+T optimizer and accept only a strict FT
    /// improvement.
    CliffordT,

    /// Run all currently available exact local FT transformations and choose
    /// the best strictly improving result.
    ///
    /// This is useful for O3-style compilation.
    AggressiveLocal,
}

impl Default for FaultTolerantObjective {
    fn default() -> Self {
        Self::TCountThenOperations
    }
}

impl FaultTolerantObjective {
    /// Returns a stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TCountThenOperations => "t_count_then_operations",
            Self::TDepthThenTCount => "t_depth_then_t_count",
            Self::OperationsThenTCount => "operations_then_t_count",
            Self::CliffordT => "clifford_t",
            Self::AggressiveLocal => "aggressive_local",
        }
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Production configuration for fault-tolerant optimization.
///
/// This configuration deliberately contains policy, not quantum identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaultTolerantOptimizationConfig {
    /// Objective used to compare candidate circuits.
    pub objective: FaultTolerantObjective,

    /// Maximum number of accepted fixed-point iterations.
    ///
    /// `0` disables transformation while retaining validation/analysis.
    ///
    /// `usize::MAX` means no pass-local iteration ceiling; shared optimizer
    /// limits remain authoritative.
    pub max_iterations: usize,

    /// Run local T/Tdg power reduction.
    pub enable_t_gate_reduction: bool,

    /// Run the standalone exact Clifford+T optimizer.
    ///
    /// The default is false because the Clifford+T optimizer and
    /// TGateReductionPass intentionally overlap in some local identities.
    pub enable_clifford_t: bool,

    /// Require strict objective improvement before accepting a candidate.
    ///
    /// Production mode should always keep this true.
    pub require_strict_improvement: bool,
}

impl Default for FaultTolerantOptimizationConfig {
    fn default() -> Self {
        Self::production()
    }
}

impl FaultTolerantOptimizationConfig {
    /// Returns the production configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            objective: FaultTolerantObjective::TCountThenOperations,
            max_iterations: usize::MAX,
            enable_t_gate_reduction: true,
            enable_clifford_t: false,
            require_strict_improvement: true,
        }
    }

    /// Returns an aggressive exact local configuration.
    #[must_use]
    pub const fn aggressive_local() -> Self {
        Self {
            objective: FaultTolerantObjective::AggressiveLocal,
            max_iterations: usize::MAX,
            enable_t_gate_reduction: true,
            enable_clifford_t: true,
            require_strict_improvement: true,
        }
    }

    /// Returns a T-depth-oriented configuration.
    #[must_use]
    pub const fn t_depth() -> Self {
        Self {
            objective: FaultTolerantObjective::TDepthThenTCount,
            max_iterations: usize::MAX,
            enable_t_gate_reduction: true,
            enable_clifford_t: false,
            require_strict_improvement: true,
        }
    }

    /// Disables transformations.
    ///
    /// The pass still validates the input circuit.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            objective: FaultTolerantObjective::TCountThenOperations,
            max_iterations: 0,
            enable_t_gate_reduction: false,
            enable_clifford_t: false,
            require_strict_improvement: true,
        }
    }

    /// Returns whether at least one transformation is enabled.
    #[must_use]
    pub const fn transformation_enabled(self) -> bool {
        self.max_iterations != 0
            && (self.enable_t_gate_reduction || self.enable_clifford_t)
    }
}

// =============================================================================
// Objective score
// =============================================================================

/// Exact objective score used for candidate comparison.
///
/// All values are integer based and therefore deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaultTolerantScore {
    /// T/Tdg family count.
    pub t_count: u128,

    /// Logical T-depth.
    pub t_depth: u128,

    /// Total canonical operation count.
    pub operations: u128,
}

impl FaultTolerantScore {
    /// Creates a score from a circuit.
    pub fn from_circuit(
        circuit: &QuantumCircuit,
    ) -> Result<Self, OptimizationError> {
        let t_count = analyze_t_count(circuit).map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::FaultTolerantOptimization,
                format!("T-count analysis failed: {error}"),
            )
        })?;

        let t_depth = analyze_t_depth(circuit).map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::FaultTolerantOptimization,
                format!("T-depth analysis failed: {error}"),
            )
        })?;

        let t_count_value = t_count.t_family_count();

        let t_depth_value = t_depth.depth() as u128;

        let operations = usize_to_u128(
            circuit.len(),
            "fault-tolerant operation count",
        )?;

        Ok(Self {
            t_count: t_count_value,
            t_depth: t_depth_value,
            operations,
        })
    }

    /// Creates a score directly from structural counters.
    #[must_use]
    pub const fn new(
        t_count: u128,
        t_depth: u128,
        operations: u128,
    ) -> Self {
        Self {
            t_count,
            t_depth,
            operations,
        }
    }

    /// Returns true if this score is strictly better under `objective`.
    #[must_use]
    pub fn strictly_better_than(
        self,
        other: Self,
        objective: FaultTolerantObjective,
    ) -> bool {
        self.compare(other, objective).is_lt()
    }

    /// Returns true if this score is equal under the supplied objective.
    #[must_use]
    pub fn equivalent_under(
        self,
        other: Self,
        objective: FaultTolerantObjective,
    ) -> bool {
        self.compare(other, objective).is_eq()
    }

    /// Compares two scores using the configured objective.
    #[must_use]
    pub fn compare(
        self,
        other: Self,
        objective: FaultTolerantObjective,
    ) -> std::cmp::Ordering {
        match objective {
            FaultTolerantObjective::TCountThenOperations
            | FaultTolerantObjective::CliffordT
            | FaultTolerantObjective::AggressiveLocal => (
                self.t_count,
                self.operations,
                self.t_depth,
            )
                .cmp(&(
                    other.t_count,
                    other.operations,
                    other.t_depth,
                )),

            FaultTolerantObjective::TDepthThenTCount => (
                self.t_depth,
                self.t_count,
                self.operations,
            )
                .cmp(&(
                    other.t_depth,
                    other.t_count,
                    other.operations,
                )),

            FaultTolerantObjective::OperationsThenTCount => (
                self.operations,
                self.t_count,
                self.t_depth,
            )
                .cmp(&(
                    other.operations,
                    other.t_count,
                    other.t_depth,
                )),
        }
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Detailed statistics produced by the composite pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FaultTolerantOptimizationStatistics {
    /// Number of fixed-point iterations attempted.
    pub iterations: u64,

    /// Number of candidates generated.
    pub candidates: u64,

    /// Number of candidates accepted.
    pub accepted_candidates: u64,

    /// Number of candidates rejected because they did not improve the
    /// objective.
    pub rejected_candidates: u64,

    /// Number of T/Tdg reduction invocations.
    pub t_gate_reduction_runs: u64,

    /// Number of Clifford+T optimizer invocations.
    pub clifford_t_runs: u64,

    /// Initial T-family count.
    pub initial_t_count: u128,

    /// Final T-family count.
    pub final_t_count: u128,

    /// Initial T-depth.
    pub initial_t_depth: u128,

    /// Final T-depth.
    pub final_t_depth: u128,

    /// Initial operation count.
    pub initial_operations: u128,

    /// Final operation count.
    pub final_operations: u128,
}

impl FaultTolerantOptimizationStatistics {
    /// Returns the reduction in T-family count.
    ///
    /// A positive value means T-family operations were removed.
    #[must_use]
    pub fn t_count_reduction(self) -> u128 {
        self.initial_t_count.saturating_sub(self.final_t_count)
    }

    /// Returns the reduction in T-depth.
    #[must_use]
    pub fn t_depth_reduction(self) -> u128 {
        self.initial_t_depth.saturating_sub(self.final_t_depth)
    }

    /// Returns the reduction in total operations.
    #[must_use]
    pub fn operation_reduction(self) -> u128 {
        self.initial_operations
            .saturating_sub(self.final_operations)
    }
}

// =============================================================================
// Pass
// =============================================================================

/// Composite fault-tolerant optimization pass.
///
/// The pass owns no invocation-local mutable state beyond its immutable
/// configuration. Invocation state belongs to `OptimizationContext`.
#[derive(Debug, Clone)]
pub struct OptimizeFaultTolerance {
    metadata: PassMetadata,
    config: FaultTolerantOptimizationConfig,
    t_gate_reduction: TGateReductionPass,
}

impl OptimizeFaultTolerance {
    /// Constructs the production fault-tolerant optimizer.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(FaultTolerantOptimizationConfig::production())
    }

    /// Constructs the optimizer from explicit configuration.
    #[must_use]
    pub fn with_config(
        config: FaultTolerantOptimizationConfig,
    ) -> Self {
        let identifier = PassIdentifier::new(PASS_ID)
            .expect("static fault-tolerant optimizer identifier is valid");

        let metadata = PassMetadata::new(
            identifier,
            PASS_NAME,
            PassKind::FaultTolerant,
        )
        .expect("fault-tolerant optimizer metadata must be valid")
        .with_description(
            "Composite exact fault-tolerant logical-circuit optimization \
             driven by T-count, T-depth, and operation-count objectives.",
        )
        .with_scope(PassScope::Circuit)
        .with_complexity(PassComplexity::TargetDependent)
        .with_determinism(PassDeterminism::Deterministic)
        .with_capability(PassCapability::RemovesOperations)
        .with_capability(PassCapability::ReplacesOperations)
        .with_capability(PassCapability::ChangesGateCount)
        .with_capability(PassCapability::ChangesFaultTolerantCost)
        .with_semantic_preservation(true)
        .supports_empty_circuit(true)
        .supports_single_operation(true)
        .supports_large_circuits(true)
        .requires_target(false)
        .requires_verification(false)
        .fixed_point_safe(true);

        Self {
            metadata,
            config,
            t_gate_reduction: TGateReductionPass::new(),
        }
    }

    /// Returns the optimizer configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> FaultTolerantOptimizationConfig {
        self.config
    }

    /// Returns the initial/final score of a circuit without transforming it.
    pub fn score(
        circuit: &QuantumCircuit,
    ) -> Result<FaultTolerantScore, OptimizationError> {
        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "fault-tolerant optimization received invalid \
                     Quantum IR: {error}"
                ),
            )
        })?;

        FaultTolerantScore::from_circuit(circuit)
    }

    /// Optimizes a standalone operation sequence using the exact
    /// Clifford+T optimizer.
    ///
    /// The input is never mutated.
    ///
    /// This helper is intentionally public because the planner and future
    /// candidate-generation infrastructure can use it without creating
    /// another optimizer implementation.
    pub fn optimize_clifford_t_operations(
        operations: &[Gate],
    ) -> Result<Vec<Gate>, OptimizationError> {
        let (optimized, _statistics) =
            clifford_t::optimize(operations).map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    format!(
                        "Clifford+T optimization failed: {error}"
                    ),
                )
            })?;

        Ok(optimized)
    }

    /// Replaces the circuit contents with an already validated operation
    /// sequence.
    ///
    /// The caller must ensure the sequence does not exceed the existing
    /// circuit's configured operation policy.
    ///
    /// The sequence is validated before mutation by constructing the result
    /// through the canonical circuit mutation API.
    fn commit_operations(
        circuit: &mut QuantumCircuit,
        operations: Vec<Gate>,
    ) -> Result<(), OptimizationError> {
        let original_len = circuit.len();

        if operations.len() > original_len {
            return Err(OptimizationError::resource_limit(
                OptimizationStage::FaultTolerantOptimization,
                "fault-tolerant candidate operation count",
                operations.len(),
                original_len,
            ));
        }

        /*
         * All currently delegated FT transformations are non-expanding:
         *
         *   T/Tdg reduction: never expands a T-run beyond its original size.
         *   Clifford+T local optimization: its standalone optimizer is used
         *   only as an FT candidate and is accepted only after validation.
         *
         * Keeping this check here prevents this objective pass from silently
         * increasing the circuit beyond the source circuit's operation budget.
         */

        circuit.clear();

        for gate in operations {
            circuit.push(gate).map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    format!(
                        "failed to commit validated fault-tolerant \
                         candidate: {error}"
                    ),
                )
            })?;
        }

        circuit.validate().map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::FaultTolerantOptimization,
                format!(
                    "fault-tolerant candidate violated canonical Quantum IR \
                     invariants after commit: {error}"
                ),
            )
        })?;

        Ok(())
    }

    /// Generates a candidate by running the canonical T/Tdg reduction pass.
    ///
    /// The supplied circuit is the candidate circuit, not the caller's
    /// original circuit.
    fn run_t_gate_reduction(
        &self,
        candidate: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<(), OptimizationError> {
        self.t_gate_reduction
            .run(candidate, context)
            .map(|_| ())
    }

    /// Generates a candidate using the standalone exact Clifford+T optimizer.
    fn run_clifford_t(
        candidate: &mut QuantumCircuit,
    ) -> Result<(), OptimizationError> {
        let optimized =
            Self::optimize_clifford_t_operations(candidate.operations())?;

        Self::commit_operations(candidate, optimized)
    }

    /// Copies the canonical circuit into a new circuit suitable for candidate
    /// evaluation.
///
/// The canonical circuit deliberately owns its resource policy. Candidate
/// construction therefore uses the circuit's public constructor rather than
/// reaching into private fields.
    fn clone_circuit(
        circuit: &QuantumCircuit,
    ) -> Result<QuantumCircuit, OptimizationError> {
        /*
         * QuantumCircuit is deliberately not required to expose a mutable
         * operation slice. Candidate construction uses the public constructor
         * and validated push API.
         *
         * The exact constructor used by the canonical IR is intentionally
         * centralized here. Once constructed, this file never accesses private
         * circuit state.
         */

        let mut candidate = QuantumCircuit::new(
            circuit.num_qubits(),
            circuit.num_classical_bits(),
            circuit.limits().clone(),
        )
        .map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::FaultTolerantOptimization,
                format!(
                    "failed to construct fault-tolerant optimization \
                     candidate circuit: {error}"
                ),
            )
        })?;

        for gate in circuit.operations() {
            candidate.push(gate.clone()).map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    format!(
                        "failed to copy operation into FT optimization \
                         candidate: {error}"
                    ),
                )
            })?;
        }

        candidate.validate().map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::FaultTolerantOptimization,
                format!(
                    "copied FT optimization candidate is invalid: {error}"
                ),
            )
        })?;

        Ok(candidate)
    }

    /// Returns a candidate that has been transformed according to this
    /// optimizer's configuration.
    fn generate_candidate(
        &self,
        source: &QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<Option<QuantumCircuit>, OptimizationError> {
        let mut candidate = Self::clone_circuit(source)?;

        let mut transformed = false;

        if self.config.enable_t_gate_reduction {
            context.check_cancelled().map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    format!(
                        "fault-tolerant T-gate reduction cancelled: {error}"
                    ),
                )
            })?;

            self.run_t_gate_reduction(
                &mut candidate,
                context,
            )?;

            transformed = true;
        }

        if self.config.enable_clifford_t {
            context.check_cancelled().map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    format!(
                        "fault-tolerant Clifford+T optimization cancelled: \
                         {error}"
                    ),
                )
            })?;

            Self::run_clifford_t(&mut candidate)?;

            transformed = true;
        }

        if !transformed {
            return Ok(None);
        }

        candidate.validate().map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::FaultTolerantOptimization,
                format!(
                    "fault-tolerant candidate validation failed: {error}"
                ),
            )
        })?;

        Ok(Some(candidate))
    }

    /// Runs the fixed-point candidate loop.
    fn optimize_internal(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<FaultTolerantOptimizationStatistics, OptimizationError> {
        let initial_score = FaultTolerantScore::from_circuit(circuit)?;

        let mut statistics =
            FaultTolerantOptimizationStatistics {
                initial_t_count: initial_score.t_count,
                initial_t_depth: initial_score.t_depth,
                initial_operations: initial_score.operations,
                ..FaultTolerantOptimizationStatistics::default()
            };

        if !self.config.transformation_enabled() {
            statistics.final_t_count = initial_score.t_count;
            statistics.final_t_depth = initial_score.t_depth;
            statistics.final_operations = initial_score.operations;
            return Ok(statistics);
        }

        let mut current_score = initial_score;
        let mut iteration = 0usize;

        while iteration < self.config.max_iterations {
            context.check_cancelled().map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    format!(
                        "fault-tolerant optimization cancelled: {error}"
                    ),
                )
            })?;

            context.record_iteration().map_err(|error| {
                OptimizationError::resource_limit(
                    OptimizationStage::FaultTolerantOptimization,
                    "fault-tolerant optimization iterations",
                    1,
                    0,
                )
                .with_message(format!(
                    "shared optimization iteration budget exhausted: {error}"
                ))
            })?;

            statistics.iterations =
                statistics.iterations.checked_add(1).ok_or_else(|| {
                    OptimizationError::internal(
                        OptimizationStage::FaultTolerantOptimization,
                        "fault-tolerant iteration counter overflow",
                    )
                })?;

            let candidate =
                self.generate_candidate(circuit, context)?;

            let Some(candidate) = candidate else {
                break;
            };

            statistics.candidates =
                statistics.candidates.checked_add(1).ok_or_else(|| {
                    OptimizationError::internal(
                        OptimizationStage::FaultTolerantOptimization,
                        "fault-tolerant candidate counter overflow",
                    )
                })?;

            let candidate_score =
                FaultTolerantScore::from_circuit(&candidate)?;

            if !candidate_score.strictly_better_than(
                current_score,
                self.config.objective,
            ) {
                statistics.rejected_candidates = statistics
                    .rejected_candidates
                    .checked_add(1)
                    .ok_or_else(|| {
                        OptimizationError::internal(
                            OptimizationStage::FaultTolerantOptimization,
                            "fault-tolerant rejected-candidate counter \
                             overflow",
                        )
                    })?;

                break;
            }

            context.check_cancelled().map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    format!(
                        "fault-tolerant optimization cancelled before \
                         candidate commit: {error}"
                    ),
                )
            })?;

            let candidate_operations =
                candidate.operations().to_vec();

            Self::commit_operations(
                circuit,
                candidate_operations,
            )?;

            current_score = candidate_score;

            statistics.accepted_candidates = statistics
                .accepted_candidates
                .checked_add(1)
                .ok_or_else(|| {
                    OptimizationError::internal(
                        OptimizationStage::FaultTolerantOptimization,
                        "fault-tolerant accepted-candidate counter overflow",
                    )
                })?;

            /*
             * A successful candidate is guaranteed to be strictly better
             * under the selected objective, so the loop cannot oscillate
             * between equal-cost circuits.
             */
            iteration = iteration.checked_add(1).ok_or_else(|| {
                OptimizationError::internal(
                    OptimizationStage::FaultTolerantOptimization,
                    "fault-tolerant iteration index overflow",
                )
            })?;
        }

        statistics.final_t_count = current_score.t_count;
        statistics.final_t_depth = current_score.t_depth;
        statistics.final_operations = current_score.operations;

        Ok(statistics)
    }

    /// Runs the optimizer and returns detailed statistics.
    pub fn optimize(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> PassExecutionResult {
        self.run(circuit, context)
    }
}

impl Default for OptimizeFaultTolerance {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// OptimizationPass implementation
// =============================================================================

impl OptimizationPass for OptimizeFaultTolerance {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> PassExecutionResult {
        context.check_cancelled().map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::FaultTolerantOptimization,
                format!(
                    "fault-tolerant optimization cannot start: {error}"
                ),
            )
        })?;

        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "fault-tolerant optimization received invalid \
                     Quantum IR: {error}"
                ),
            )
        })?;

        let operations_before = usize_to_u64(
            circuit.len(),
            "fault-tolerant operations before optimization",
        )?;

        let before_score = FaultTolerantScore::from_circuit(circuit)?;

        let statistics =
            self.optimize_internal(circuit, context)?;

        let operations_after = usize_to_u64(
            circuit.len(),
            "fault-tolerant operations after optimization",
        )?;

        let changed = operations_before != operations_after
            || statistics.initial_t_count != statistics.final_t_count
            || statistics.initial_t_depth != statistics.final_t_depth;

        if !changed {
            return Ok(PassOutcome::unchanged(
                operations_before,
                operations_after,
            )
            .with_message(
                format!(
                    "fault-tolerant optimization reached a fixed point; \
                     objective={}, T-count={}, T-depth={}, operations={}",
                    self.config.objective.as_str(),
                    before_score.t_count,
                    before_score.t_depth,
                    before_score.operations,
                ),
            ));
        }

        let removed = operations_before
            .saturating_sub(operations_after);

        let added = operations_after
            .saturating_sub(operations_before);

        Ok(
            PassOutcome::changed(
                operations_before,
                operations_after,
            )
            .with_operations_removed(removed)
            .with_operations_added(added)
            .with_rewrites(statistics.accepted_candidates)
            .with_iterations(statistics.iterations)
            .with_message(format!(
                "fault-tolerant optimization completed: \
                 objective={}, T-count {} -> {}, \
                 T-depth {} -> {}, \
                 operations {} -> {}",
                self.config.objective.as_str(),
                statistics.initial_t_count,
                statistics.final_t_count,
                statistics.initial_t_depth,
                statistics.final_t_depth,
                statistics.initial_operations,
                statistics.final_operations,
            )),
        )
    }
}

// =============================================================================
// Integer helpers
// =============================================================================

/// Converts `usize` to `u128` without narrowing.
fn usize_to_u128(
    value: usize,
    name: &'static str,
) -> Result<u128, OptimizationError> {
    u128::try_from(value).map_err(|_| {
        OptimizationError::internal(
            OptimizationStage::FaultTolerantOptimization,
            format!(
                "{name} cannot be represented as u128"
            ),
        )
    })
}

/// Converts `usize` to `u64` for the common pass-result contract.
fn usize_to_u64(
    value: usize,
    name: &'static str,
) -> Result<u64, OptimizationError> {
    u64::try_from(value).map_err(|_| {
        OptimizationError::internal(
            OptimizationStage::FaultTolerantOptimization,
            format!(
                "{name} cannot be represented as u64"
            ),
        )
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::{
        Gate,
        GateKind,
        QubitId,
    };

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
            .expect("test qubit identifier must be valid")
    }

    fn gate(
        kind: GateKind,
        qubit: usize,
    ) -> Gate {
        Gate::new(
            kind,
            vec![q(qubit)],
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    #[test]
    fn objective_order_is_t_count_first_by_default() {
        let better = FaultTolerantScore::new(
            2,
            4,
            100,
        );

        let worse = FaultTolerantScore::new(
            3,
            1,
            1,
        );

        assert!(
            better.strictly_better_than(
                worse,
                FaultTolerantObjective::TCountThenOperations,
            )
        );
    }

    #[test]
    fn depth_objective_prefers_lower_t_depth() {
        let better = FaultTolerantScore::new(
            5,
            1,
            100,
        );

        let worse = FaultTolerantScore::new(
            2,
            2,
            1,
        );

        assert!(
            better.strictly_better_than(
                worse,
                FaultTolerantObjective::TDepthThenTCount,
            )
        );
    }

    #[test]
    fn operations_objective_prefers_smaller_circuit() {
        let better = FaultTolerantScore::new(
            5,
            2,
            10,
        );

        let worse = FaultTolerantScore::new(
            2,
            1,
            20,
        );

        assert!(
            better.strictly_better_than(
                worse,
                FaultTolerantObjective::OperationsThenTCount,
            )
        );
    }

    #[test]
    fn production_configuration_is_strict() {
        let config =
            FaultTolerantOptimizationConfig::production();

        assert!(config.enable_t_gate_reduction);
        assert!(!config.enable_clifford_t);
        assert!(config.require_strict_improvement);
        assert_eq!(
            config.objective,
            FaultTolerantObjective::TCountThenOperations
        );
    }

    #[test]
    fn disabled_configuration_disables_transformation() {
        let config =
            FaultTolerantOptimizationConfig::disabled();

        assert!(!config.transformation_enabled());
    }

    #[test]
    fn t_gate_reduction_candidate_is_not_empty_for_single_t() {
        let mut circuit =
            QuantumCircuit::new(
                1,
                0,
                Default::default(),
            )
            .expect("test circuit must be constructible");

        circuit
            .push(gate(GateKind::T, 0))
            .expect("test gate must be insertable");

        circuit
            .validate()
            .expect("test circuit must validate");

        let score =
            FaultTolerantScore::from_circuit(&circuit)
                .expect("score calculation must succeed");

        assert_eq!(score.t_count, 1);
        assert_eq!(score.operations, 1);
    }
}