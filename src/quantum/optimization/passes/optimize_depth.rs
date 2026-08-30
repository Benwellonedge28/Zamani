//! Zamani Quantum Optimization — Logical Depth Optimization Pass.
//!
//! Production-grade depth-objective optimization over the canonical
//! `crate::quantum::ir::QuantumCircuit`.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir::QuantumCircuit
//!                                    │
//!                                    ▼
//!                    optimization::passes::optimize_depth
//!                                    │
//!                  ┌─────────────────┴─────────────────┐
//!                  │                                   │
//!                  ▼                                   ▼
//!          analysis::depth                    local transformations
//!                  │                          ┌────────┴────────┐
//!                  │                          ▼                 ▼
//!                  │                    commutation        cancellation
//!                  │                          │                 │
//!                  └──────────────┬───────────┴─────────────────┘
//!                                 ▼
//!                       candidate depth comparison
//!                                 │
//!                    ┌────────────┴────────────┐
//!                    │                         │
//!                    ▼                         ▼
//!              improvement                 no improvement
//!                    │                         │
//!                    ▼                         ▼
//!             atomic commit                 discard
//!                    │
//!                    ▼
//!              optimized Quantum IR
//! ```
//!
//! This file owns the **depth optimization policy and candidate acceptance
//! protocol**.
//!
//! It does NOT own:
//!
//! - logical depth calculation;
//! - commutation semantics;
//! - gate cancellation semantics;
//! - gate fusion semantics;
//! - routing;
//! - physical scheduling;
//! - hardware timing;
//! - hardware topology;
//! - QPU execution;
//! - quantum algorithms;
//! - error correction;
//! - benchmarking;
//! - another Quantum IR.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Why this pass is composite
//!
//! Logical depth cannot generally be reduced by simply "sorting" gates.
//!
//! The canonical depth analysis already exposes independent operations as
//! parallel even when they occur sequentially in the source operation vector.
//!
//! Therefore genuine depth reduction requires exact transformations such as:
//!
//! - commuting operations toward useful partners;
//! - exposing exact cancellation;
//! - eliminating redundant operations;
//! - eventually gate fusion and algebraic synthesis.
//!
//! This pass coordinates transformations already owned by specialized passes
//! and accepts a candidate only when the canonical depth metric improves.
//!
//! This prevents `optimize_depth.rs` from duplicating gate semantics.
//!
//! # Current transformation set
//!
//! The production implementation intentionally uses:
//!
//! 1. `local::commutation`
//! 2. `local::cancellation`
//!
//! in a bounded fixed-point search.
//!
//! This creates the following optimization pattern:
//!
//! ```text
//! C
//! │
//! ├── commutation
//! │
//! ├── cancellation
//! │
//! └── depth analysis
//!       │
//!       ├── lower → accept
//!       └── not lower → discard
//! ```
//!
//! Future depth-aware passes can be inserted into the candidate pipeline
//! without changing the depth acceptance contract.
//!
//! # Semantic safety
//!
//! Candidate circuits are produced by exact transformation passes.
//!
//! A candidate is never committed merely because it has a lower gate count.
//!
//! The acceptance criterion is explicitly:
//!
//! ```text
//! candidate_depth < current_depth
//! ```
//!
//! Therefore this pass cannot knowingly trade increased logical depth for
//! reduced gate count.
//!
//! A candidate with equal depth is rejected by default. This conservative
//! policy prevents the depth pass from becoming an uncontrolled secondary
//! gate-count optimizer and avoids accepting equal-depth permutations merely
//! because they happen to be different.
//!
//! # Transactionality
//!
//! The input circuit is never modified while a candidate is being constructed.
//!
//! The algorithm is:
//!
//! ```text
//! original
//!    │
//!    ├── clone
//!    │
//!    ▼
//! candidate
//!    │
//!    ├── exact transformations
//!    │
//!    ├── validation
//!    │
//!    ├── depth analysis
//!    │
//!    └── acceptance test
//!          │
//!          ├── reject → original remains untouched
//!          │
//!          └── accept → atomic replacement
//! ```
//!
//! This is particularly important for compiler correctness and for future
//! verified optimization.
//!
//! # Fixed-point behavior
//!
//! Each iteration must strictly reduce logical depth before its candidate is
//! committed.
//!
//! Because logical depth is a non-negative integer, successful iterations are
//! monotonically decreasing:
//!
//! ```text
//! d0 > d1 > d2 > ... >= 0
//! ```
//!
//! Consequently the accepted transformation sequence cannot oscillate.
//!
//! The optimizer still has an explicit iteration budget because candidate
//! generation itself consumes resources and because future transformations may
//! be substantially more expensive than the current implementation.
//!
//! # Scaling
//!
//! Let:
//!
//! - `N` = number of operations;
//! - `A` = total qubit operands;
//! - `I` = accepted optimization iterations.
//!
//! The depth analysis is approximately:
//!
//! ```text
//! O(N + A)
//! ```
//!
//! The current commutation stage is potentially quadratic in its configured
//! search window, while cancellation is linear.
//!
//! Therefore total work is approximately:
//!
//! ```text
//! O(I * (commutation_work + N + A))
//! ```
//!
//! There is intentionally no hard-coded circuit-size ceiling.
//!
//! Scaling is governed by:
//!
//! - canonical Quantum IR limits;
//! - `OptimizationLimits`;
//! - `OptimizationContext` resource accounting;
//! - configured iteration budget;
//! - available memory;
//! - available CPU;
//! - host address-space limits.
//!
//! "Infinity" therefore means no artificial algorithmic maximum imposed by
//! this pass. Actual physical and configured resources remain authoritative.
//!
//! # Determinism
//!
//! The current implementation is deterministic:
//!
//! - transformation passes are deterministic;
//! - candidate order is deterministic;
//! - depth comparison is deterministic;
//! - no random numbers are used;
//! - no global mutable state is used;
//! - no threads are created here.
//!
//! # Parallelism
//!
//! This pass deliberately does not spawn threads.
//!
//! Parallel optimization belongs to `optimization::scheduler`.
//!
//! Candidate passes themselves remain `Send + Sync` through the
//! `OptimizationPass` contract.
//!
//! # Resource accounting
//!
//! Every optimization iteration is charged through the shared
//! `OptimizationContext`.
//!
//! The pass does not create a second resource-limit system.
//!
//! # Integration contract
//!
//! ## `analysis::depth`
//!
//! This module consumes:
//!
//! ```text
//! DepthAnalysis::analyze_validated
//! ```
//!
//! It does not reimplement depth calculation.
//!
//! ## `local::commutation`
//!
//! This pass uses `CommutationPass` only through its public
//! `OptimizationPass` interface.
//!
//! Commutation remains responsible for proving exact operation movement.
//!
//! ## `local::cancellation`
//!
//! This pass uses `CancellationPass` only through its public
//! `OptimizationPass` interface.
//!
//! Cancellation remains responsible for exact cancellation semantics.
//!
//! ## `context`
//!
//! The invocation context owns:
//!
//! - cancellation;
//! - resource budgets;
//! - rewrite accounting;
//! - iteration accounting.
//!
//! This pass never creates global optimizer state.
//!
//! ## `pass`
//!
//! This module implements `OptimizationPass`.
//!
//! Its metadata declares:
//!
//! - composite optimization;
//! - circuit scope;
//! - depth changes;
//! - gate-count changes;
//! - operation reordering;
//! - commutation usage;
//! - deterministic execution;
//! - semantic preservation;
//! - large-circuit support;
//! - fixed-point safety.
//!
//! ## `pipeline`
//!
//! The intended position is after normalization and before routing:
//!
//! ```text
//! normalize
//!     ↓
//! parameter simplification
//!     ↓
//! local simplification
//!     ↓
//! optimize_depth
//!     ↓
//! algebraic/synthesis optimization
//!     ↓
//! routing
//!     ↓
//! scheduling
//! ```
//!
//! The pipeline may also run this pass again after other logical
//! transformations because every successful invocation is monotonic with
//! respect to accepted depth.
//!
//! ## `planner`
//!
//! The planner should select this pass for:
//!
//! - `OptimizationLevel::Od`;
//! - `OptimizationProfile::MinimumDepth`;
//! - target profiles whose objective prioritizes logical depth;
//! - balanced profiles when depth has sufficient weight.
//!
//! The pass itself does not inspect the profile.
//!
//! ## `cost`
//!
//! `CostMetric::Depth` remains owned by `optimization::cost`.
//!
//! This pass uses the canonical depth metric directly because its acceptance
//! condition is a strict semantic depth comparison.
//!
//! The cost model may consume the before/after circuit later for multi-objective
//! decisions.
//!
//! ## `targets`
//!
//! This pass is hardware-independent.
//!
//! Target-specific timing and topology do not belong here.
//!
//! Physical depth may change after routing, so this pass must not claim to
//! minimize physical execution time.
//!
//! ## `routing`
//!
//! Routing occurs downstream.
//!
//! Routing may introduce operations and therefore may increase physical/logical
//! depth. A later optimization stage may recalculate logical depth if the
//! compiler policy requires it.
//!
//! ## `scheduling`
//!
//! Hardware scheduling owns execution timing and pulse overlap.
//!
//! This pass optimizes logical circuit depth only.
//!
//! ## `verification`
//!
//! Exact transformations are used, but whole-pipeline semantic verification
//! remains owned by the verification subsystem.
//!
//! A verified compiler profile should run its configured equivalence checker
//! after this pass or after the complete optimization pipeline.
//!
//! ## `benchmarking`
//!
//! Benchmarking must consume this pass's before/after statistics externally.
//!
//! This module must not depend on benchmarking.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe code is required or permitted.

#![forbid(unsafe_code)]

use crate::quantum::ir::QuantumCircuit;

use super::super::analysis::depth::DepthAnalysis;
use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationStage,
    PassIdentifier,
};
use super::super::local::cancellation::CancellationPass;
use super::super::local::commutation::CommutationPass;
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

/// Stable machine-readable identifier.
///
/// This identifier is part of optimizer provenance and should remain stable
/// across Rust type/file refactoring.
pub const PASS_ID: &str = "passes.optimize_depth";

/// Stable human-readable pass name.
pub const PASS_NAME: &str = "Logical Circuit Depth Optimization";

/// Public behavior/schema version of this pass.
pub const PASS_VERSION: u32 = 1;

/// Configuration for depth optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DepthOptimizationConfig {
    /// Maximum number of accepted depth-reducing iterations.
    ///
    /// Zero disables transformation while still allowing the pass to perform
    /// validation and depth measurement.
    pub max_iterations: usize,

    /// Whether exact commutation should be attempted.
    pub enable_commutation: bool,

    /// Whether exact local cancellation should be attempted.
    pub enable_cancellation: bool,
}

impl Default for DepthOptimizationConfig {
    fn default() -> Self {
        Self {
            max_iterations: usize::MAX,
            enable_commutation: true,
            enable_cancellation: true,
        }
    }
}

impl DepthOptimizationConfig {
    /// Creates the production configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_iterations: usize::MAX,
            enable_commutation: true,
            enable_cancellation: true,
        }
    }

    /// Creates a conservative configuration that performs no transformations.
    ///
    /// This is useful for diagnostics and planner tests.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            max_iterations: 0,
            enable_commutation: false,
            enable_cancellation: false,
        }
    }

    /// Sets the maximum number of accepted iterations.
    #[must_use]
    pub const fn with_max_iterations(
        mut self,
        max_iterations: usize,
    ) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Enables or disables exact commutation.
    #[must_use]
    pub const fn with_commutation(
        mut self,
        enabled: bool,
    ) -> Self {
        self.enable_commutation = enabled;
        self
    }

    /// Enables or disables exact cancellation.
    #[must_use]
    pub const fn with_cancellation(
        mut self,
        enabled: bool,
    ) -> Self {
        self.enable_cancellation = enabled;
        self
    }

    /// Returns whether the configuration has any transformation enabled.
    #[must_use]
    pub const fn transformation_enabled(self) -> bool {
        self.max_iterations != 0
            && (self.enable_commutation || self.enable_cancellation)
    }
}

/// Immutable result metrics from one depth optimization invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DepthOptimizationStatistics {
    /// Logical depth before optimization.
    pub depth_before: usize,

    /// Logical depth after optimization.
    pub depth_after: usize,

    /// Number of accepted depth-reducing iterations.
    pub iterations: u64,

    /// Number of candidate transformations attempted.
    pub candidates_attempted: u64,

    /// Number of candidates rejected because they did not improve depth.
    pub candidates_rejected: u64,

    /// Number of operations before optimization.
    pub operations_before: u64,

    /// Number of operations after optimization.
    pub operations_after: u64,
}

impl DepthOptimizationStatistics {
    /// Returns whether logical depth was reduced.
    #[must_use]
    pub const fn changed(self) -> bool {
        self.depth_after < self.depth_before
    }

    /// Returns the absolute depth reduction.
    #[must_use]
    pub const fn depth_reduction(self) -> usize {
        self.depth_before - self.depth_after
    }
}

/// Production logical-depth optimization pass.
///
/// The pass itself is stateless. Invocation-specific state belongs to the
/// supplied `OptimizationContext`.
#[derive(Debug, Clone)]
pub struct OptimizeDepth {
    metadata: PassMetadata,
    config: DepthOptimizationConfig,
}

impl OptimizeDepth {
    /// Constructs the production depth optimizer.
    pub fn new() -> Result<Self, PassMetadataError> {
        Self::with_config(DepthOptimizationConfig::default())
    }

    /// Constructs the depth optimizer with an explicit configuration.
    pub fn with_config(
        config: DepthOptimizationConfig,
    ) -> Result<Self, PassMetadataError> {
        let identifier = PassIdentifier::from_static(PASS_ID)?;

        let metadata = PassMetadata::new(
            identifier,
            PASS_NAME,
            PassKind::Composite,
        )?
        .with_description(
            "Optimizes canonical logical circuit depth using exact, \
             resource-bounded local transformations and strict \
             depth-improvement acceptance.",
        )?
        .with_scope(PassScope::Circuit)
        .with_complexity(PassComplexity::Quadratic)
        .with_determinism(PassDeterminism::Deterministic)
        .with_capabilities([
            PassCapability::ReordersOperations,
            PassCapability::RemovesOperations,
            PassCapability::ChangesDepth,
            PassCapability::ChangesGateCount,
            PassCapability::ChangesTwoQubitCount,
            PassCapability::UsesCommutation,
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

    /// Returns the stable human-readable pass name.
    #[must_use]
    pub const fn pass_name() -> &'static str {
        PASS_NAME
    }

    /// Returns the pass behavior version.
    #[must_use]
    pub const fn pass_version() -> u32 {
        PASS_VERSION
    }

    /// Returns the active configuration.
    #[must_use]
    pub const fn config(&self) -> DepthOptimizationConfig {
        self.config
    }

    /// Optimizes a circuit using a standalone optimization context.
    ///
    /// Production compiler pipelines should normally invoke `run()` so that
    /// the caller's shared context and resource budgets are respected.
    pub fn optimize(
        &self,
        circuit: &mut QuantumCircuit,
    ) -> Result<PassOutcome, OptimizationError> {
        let mut context = OptimizationContext::standalone();

        self.run(circuit, &mut context)
    }

    /// Analyzes logical depth using the canonical analysis implementation.
    fn analyze_depth(
        circuit: &QuantumCircuit,
    ) -> Result<DepthAnalysis, OptimizationError> {
        DepthAnalysis::analyze_validated(circuit)
            .map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::Analysis,
                    format!(
                        "{PASS_ID}: logical depth analysis failed: {error}"
                    ),
                )
            })
    }

    /// Converts a platform counter into the optimizer's `u64` accounting type.
    fn checked_u64(
        value: usize,
        what: &'static str,
    ) -> Result<u64, OptimizationError> {
        u64::try_from(value).map_err(|_| {
            OptimizationError::internal(
                OptimizationStage::Analysis,
                format!(
                    "{PASS_ID}: {what} cannot be represented by \
                     optimizer u64 accounting",
                ),
            )
        })
    }

    /// Builds one candidate by applying the currently enabled exact local
    /// optimization stages.
    ///
    /// The supplied circuit is cloned before transformation. The caller's
    /// circuit is therefore untouched by candidate construction.
    fn build_candidate(
        &self,
        source: &QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<QuantumCircuit, OptimizationError> {
        let mut candidate = source.clone();

        context.check_cancelled().map_err(|error| {
            OptimizationError::resource_limit(
                OptimizationStage::Rewrite,
                format!(
                    "{PASS_ID}: candidate construction cancelled: {error}"
                ),
            )
        })?;

        if self.config.enable_commutation {
            let pass = CommutationPass::new().map_err(|error| {
                OptimizationError::internal(
                    OptimizationStage::Rewrite,
                    format!(
                        "{PASS_ID}: failed to construct commutation \
                         sub-pass: {error}"
                    ),
                )
            })?;

            pass.run(&mut candidate, context)?;
        }

        context.check_cancelled().map_err(|error| {
            OptimizationError::resource_limit(
                OptimizationStage::Rewrite,
                format!(
                    "{PASS_ID}: candidate construction cancelled after \
                     commutation: {error}"
                ),
            )
        })?;

        if self.config.enable_cancellation {
            let pass = CancellationPass::new();

            pass.run(&mut candidate, context)?;
        }

        candidate.validate().map_err(|error| {
            OptimizationError::rewrite_postcondition_failed(
                None,
                format!(
                    "{PASS_ID}: candidate transformation produced invalid \
                     canonical Quantum IR: {error}"
                ),
            )
        })?;

        Ok(candidate)
    }

    /// Executes the depth optimization.
    fn run_impl(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<
        (
            PassOutcome,
            DepthOptimizationStatistics,
        ),
        OptimizationError,
    > {
        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "{PASS_ID}: input Quantum IR validation failed: {error}"
                ),
            )
        })?;

        context.check_cancelled().map_err(|error| {
            OptimizationError::resource_limit(
                OptimizationStage::Analysis,
                format!(
                    "{PASS_ID}: depth optimization cancelled before \
                     analysis: {error}"
                ),
            )
        })?;

        let operations_before =
            Self::checked_u64(
                circuit.len(),
                "operation count before depth optimization",
            )?;

        let initial_analysis = Self::analyze_depth(circuit)?;

        let mut current_depth = initial_analysis.depth();

        let mut iterations = 0u64;
        let mut candidates_attempted = 0u64;
        let mut candidates_rejected = 0u64;

        let max_iterations = self.config.max_iterations;

        if !self.config.transformation_enabled()
            || circuit.is_empty()
            || current_depth == 0
        {
            let statistics = DepthOptimizationStatistics {
                depth_before: current_depth,
                depth_after: current_depth,
                iterations: 0,
                candidates_attempted: 0,
                candidates_rejected: 0,
                operations_before,
                operations_after: operations_before,
            };

            return Ok((
                PassOutcome::unchanged(
                    operations_before,
                    operations_before,
                ),
                statistics,
            ));
        }

        while iterations
            < Self::checked_u64(
                max_iterations,
                "maximum depth optimization iterations",
            )?
        {
            context.check_cancelled().map_err(|error| {
                OptimizationError::resource_limit(
                    OptimizationStage::Rewrite,
                    format!(
                        "{PASS_ID}: depth optimization cancelled during \
                         iteration {iterations}: {error}"
                    ),
                )
            })?;

            context
                .charge_one(
                    super::super::limits::OptimizationResource::Iterations,
                )
                .map_err(|error| {
                    OptimizationError::resource_limit(
                        OptimizationStage::Rewrite,
                        format!(
                            "{PASS_ID}: iteration budget exhausted: {error}"
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

            let candidate =
                self.build_candidate(circuit, context)?;

            let candidate_analysis =
                Self::analyze_depth(&candidate)?;

            let candidate_depth =
                candidate_analysis.depth();

            // Strict acceptance criterion.
            //
            // Equal-depth candidates are deliberately rejected. This keeps
            // this pass a depth optimizer rather than an accidental generic
            // gate-reordering pass.
            if candidate_depth >= current_depth {
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

            context.check_cancelled().map_err(|error| {
                OptimizationError::resource_limit(
                    OptimizationStage::Rewrite,
                    format!(
                        "{PASS_ID}: depth optimization cancelled before \
                         candidate commit: {error}"
                    ),
                )
            })?;

            // Candidate is already validated. Because it is a clone of the
            // canonical circuit with only exact transformations applied,
            // replacing the caller's circuit preserves all circuit-level
            // metadata, identity, IR version and resource policy.
            *circuit = candidate;

            current_depth = candidate_depth;

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

        let final_analysis = Self::analyze_depth(circuit)?;
        let final_depth = final_analysis.depth();

        if final_depth > initial_analysis.depth() {
            return Err(
                OptimizationError::internal(
                    OptimizationStage::Rewrite,
                    format!(
                        "{PASS_ID}: internal invariant violation: \
                         accepted optimization increased depth from \
                         {} to {}",
                        initial_analysis.depth(),
                        final_depth
                    ),
                ),
            );
        }

        let operations_after =
            Self::checked_u64(
                circuit.len(),
                "operation count after depth optimization",
            )?;

        let statistics = DepthOptimizationStatistics {
            depth_before: initial_analysis.depth(),
            depth_after: final_depth,
            iterations,
            candidates_attempted,
            candidates_rejected,
            operations_before,
            operations_after,
        };

        if !statistics.changed() {
            return Ok((
                PassOutcome::no_improvement(
                    operations_before,
                    operations_after,
                )
                .with_iterations(iterations),
                statistics,
            ));
        }

        let outcome =
            PassOutcome::changed(
                operations_before,
                operations_after,
            )
            .with_change(PassChange::Changed)
            .with_iterations(iterations);

        Ok((outcome, statistics))
    }
}

impl OptimizationPass for OptimizeDepth {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        let (outcome, _) =
            self.run_impl(circuit, context)?;

        Ok(outcome)
    }

    fn execution_policy(&self) -> PassExecutionPolicy {
        PassExecutionPolicy::StopWhenStable
    }
}

impl Default for OptimizeDepth {
    fn default() -> Self {
        Self::new()
            .expect("production depth optimization metadata must be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_configuration_is_enabled() {
        let config = DepthOptimizationConfig::default();

        assert!(config.transformation_enabled());
        assert!(config.enable_commutation);
        assert!(config.enable_cancellation);
    }

    #[test]
    fn disabled_configuration_has_no_transformations() {
        let config = DepthOptimizationConfig::disabled();

        assert!(!config.transformation_enabled());
        assert_eq!(config.max_iterations, 0);
    }

    #[test]
    fn statistics_report_depth_reduction_only_when_depth_decreases() {
        let unchanged = DepthOptimizationStatistics {
            depth_before: 4,
            depth_after: 4,
            iterations: 0,
            candidates_attempted: 1,
            candidates_rejected: 1,
            operations_before: 10,
            operations_after: 10,
        };

        assert!(!unchanged.changed());
        assert_eq!(unchanged.depth_reduction(), 0);

        let changed = DepthOptimizationStatistics {
            depth_before: 10,
            depth_after: 6,
            iterations: 2,
            candidates_attempted: 2,
            candidates_rejected: 0,
            operations_before: 20,
            operations_after: 14,
        };

        assert!(changed.changed());
        assert_eq!(changed.depth_reduction(), 4);
    }
}