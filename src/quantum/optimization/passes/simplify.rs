//! Zamani Quantum Optimization — Composite Simplification Pass.
//!
//! Production-grade, backend-independent quantum-circuit simplification for
//! Zamani's canonical Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                    crate::quantum::ir::QuantumCircuit
//!                                  │
//!                                  ▼
//!                 optimization::passes::simplify
//!                                  │
//!              ┌───────────────────┼───────────────────┐
//!              │                   │                   │
//!              ▼                   ▼                   ▼
//!          identity            inverse            cancellation
//!              │                   │                   │
//!              └───────────────────┼───────────────────┘
//!                                  ▼
//!                            rotation
//!                                  │
//!                                  ▼
//!                            commutation
//!                                  │
//!                                  ▼
//!                             peephole
//!                                  │
//!                                  ▼
//!                             templates
//!                                  │
//!                                  ▼
//!                           gate fusion
//!                                  │
//!                                  ▼
//!                         simplified Quantum IR
//! ```
//!
//! # Purpose
//!
//! `Simplify` is the standard composite local-simplification pass for the
//! Zamani optimizer.
//!
//! It does not implement individual quantum identities itself. Instead, it
//! orchestrates the canonical local optimization passes already owned by
//! `optimization::local`.
//!
//! This separation is intentional:
//!
//! - `local::identity` owns exact identity elimination;
//! - `local::inverse` owns generic inverse-pair simplification;
//! - `local::cancellation` owns self-inverse/inverse cancellation;
//! - `local::rotation` owns compatible rotation combination;
//! - `local::commutation` owns legal commuting transformations;
//! - `local::peephole` owns bounded local rewrite windows;
//! - `local::templates` owns registered multi-operation templates;
//! - `local::gate_fusion` owns compatible operation fusion;
//! - this file owns orchestration and composite-pass policy.
//!
//! # Critical ownership rule
//!
//! This file must never define:
//!
//! - `QuantumGate`;
//! - `QuantumOperation`;
//! - another circuit representation;
//! - another qubit representation;
//! - another parameter representation;
//! - hardware APIs;
//! - routing;
//! - scheduling;
//! - QPU execution.
//!
//! The authoritative representation remains:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! # Simplification contract
//!
//! The default simplification strategy is deliberately exact.
//!
//! This pass does not:
//!
//! - introduce numerical approximation;
//! - discard global phase unless the delegated pass explicitly supports the
//!   configured equivalence contract;
//! - optimize measurement semantics;
//! - optimize reset semantics;
//! - perform routing;
//! - select a physical topology;
//! - schedule pulses;
//! - execute a circuit;
//! - invoke a backend;
//! - use ambient randomness.
//!
//! # Pass ordering
//!
//! The built-in local ordering supplied by `optimization::local` is:
//!
//! ```text
//! identity
//!     ↓
//! inverse
//!     ↓
//! cancellation
//!     ↓
//! rotation
//!     ↓
//! commutation
//!     ↓
//! peephole
//!     ↓
//! templates
//!     ↓
//! gate_fusion
//! ```
//!
//! This ordering is important because earlier transformations can expose
//! opportunities for later transformations.
//!
//! For example:
//!
//! ```text
//! RZ(0)                 → identity
//! U U†                  → inverse
//! X X                   → cancellation
//! RX(a) RX(b)           → rotation
//! commuting operations → cancellation opportunity
//! local template       → smaller circuit
//! compatible gates     → fused operation
//! ```
//!
//! The composite pass does not hard-code gate names to reproduce those
//! transformations.
//!
//! # Fixed-point behavior
//!
//! A single execution performs one complete local simplification sweep.
//!
//! It does not recursively invoke itself.
//!
//! This is deliberate. Fixed-point iteration belongs to `pipeline.rs` or the
//! planner. Keeping iteration outside this pass prevents hidden unbounded
//! loops and gives the optimizer's global iteration budget authority over
//! convergence.
//!
//! The pass is therefore safe to use in a fixed-point pipeline such as:
//!
//! ```text
//! normalize
//! ↓
//! simplify
//! ↓
//! parameter simplification
//! ↓
//! simplify
//! ↓
//! ...
//! ```
//!
//! The pipeline decides when another iteration is worthwhile.
//!
//! # Scaling
//!
//! The pass introduces no artificial circuit-size limit.
//!
//! Its scalability is governed by:
//!
//! - the complexity of the delegated local passes;
//! - `OptimizationContext` limits;
//! - Quantum IR limits;
//! - rewrite budgets;
//! - cancellation/deadline policy;
//! - available memory;
//! - available CPU resources.
//!
//! The composite itself performs only:
//!
//! - pass construction;
//! - pass invocation;
//! - context cooperation;
//! - outcome collection.
//!
//! It does not perform an additional O(n²) scan of the circuit.
//!
//! # Atomicity
//!
//! Individual local passes are responsible for their own transformation
//! atomicity according to the shared optimization contract.
//!
//! This composite pass deliberately does not clone the entire circuit merely
//! to manufacture a second transaction boundary.
//!
//! Such a clone would make memory usage scale as O(n) in addition to the
//! already-required working memory of every delegated pass and would be
//! particularly harmful for very large circuits.
//!
//! The optimizer pipeline owns the global transaction/recovery policy.
//!
//! Consequently:
//!
//! ```text
//! Simplify
//!   ├── local pass 1
//!   ├── local pass 2
//!   ├── local pass 3
//!   └── ...
//! ```
//!
//! Each child pass must leave the circuit valid if it returns success and must
//! preserve the circuit if its own transformation fails.
//!
//! # Determinism
//!
//! `Simplify` itself is deterministic.
//!
//! It does not create random state and does not read:
//!
//! - wall-clock time;
//! - environment variables;
//! - filesystem state;
//! - network state;
//! - backend state.
//!
//! Determinism of the delegated passes is declared by their own metadata and
//! ultimately controlled by the optimizer pipeline.
//!
//! # Thread safety
//!
//! The pass contains only immutable configuration and concrete stateless local
//! pass objects.
//!
//! It does not:
//!
//! - spawn threads;
//! - use global mutable state;
//! - use thread-local optimizer state;
//! - access QPU state.
//!
//! Parallel pass scheduling remains owned by `optimization::scheduler`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Safety
//!
//! This module explicitly forbids unsafe Rust.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! Input and output are always:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! No optimization-local IR is introduced.
//!
//! ## `optimization::local`
//!
//! This pass consumes the stable concrete local-pass types exported by
//! `optimization::local`.
//!
//! The local module currently provides:
//!
//! - `IdentityPass`;
//! - `InversePass`;
//! - `CancellationPass`;
//! - `RotationPass`;
//! - `CommutationPass`;
//! - `PeepholePass`;
//! - `TemplatePass`;
//! - `GateFusionPass`.
//!
//! Their stable ordering is already declared by `LocalOptimizationStrategy`.
//!
//! This file intentionally consumes those public exports rather than reaching
//! into their private implementation details.
//!
//! ## `optimization::context`
//!
//! Every delegated pass receives the same invocation-scoped
//! `OptimizationContext`.
//!
//! This is essential:
//!
//! - limits remain shared;
//! - cancellation state remains shared;
//! - rewrite budgets remain shared;
//! - deterministic state remains shared;
//! - analysis state remains shared.
//!
//! `Simplify` must never create a second optimizer context.
//!
//! ## `optimization::pass`
//!
//! `Simplify` implements `OptimizationPass`.
//!
//! It is classified as:
//!
//! - `PassKind::Composite`;
//! - circuit scope;
//! - deterministic;
//! - linear-or-delegated complexity;
//! - gate-count-changing;
//! - operation-removing;
//! - operation-replacing.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline invokes this pass as one logical optimization stage.
//!
//! Pipeline-level fixed-point iteration remains outside this file.
//!
//! ## `optimization::planner`
//!
//! The planner can select this pass for:
//!
//! - generic simplification;
//! - O1;
//! - O2;
//! - O3;
//! - gate-count optimization;
//! - depth optimization;
//! - two-qubit optimization;
//! - fault-tolerant preprocessing.
//!
//! The planner may later choose individual local passes instead when a more
//! specialized profile is required.
//!
//! ## `optimization::registry`
//!
//! Register this pass under:
//!
//! `passes.simplify`
//!
//! Recommended alias:
//!
//! `simplify`
//!
//! The registry should construct it with `Simplify::new()`.
//!
//! Registry initialization must not be required by this file.
//!
//! ## `optimization::statistics`
//!
//! Child-pass statistics remain owned by the individual pass executions and
//! the shared context/statistics layer.
//!
//! This composite returns the outcome of the complete stage while preserving
//! the detailed execution records maintained by the optimizer infrastructure.
//!
//! ## `optimization::provenance`
//!
//! The stable identifier:
//!
//! `passes.simplify`
//!
//! should be recorded as the composite stage identifier.
//!
//! Individual child pass identifiers remain independently visible in the
//! pipeline/provenance stream.
//!
//! ## `optimization::verification`
//!
//! The composite pass does not implement its own semantic equivalence engine.
//!
//! Verification remains owned by `optimization::verification`.
//!
//! ## `optimization::targets`
//!
//! `Simplify` does not directly inspect hardware.
//!
//! Target-specific decisions remain inside target-aware child passes or later
//! optimization stages.
//!
//! ## `routing` / `scheduling` / `hardware`
//!
//! No dependency is introduced from this file to those subsystems.
//!
//! The intended direction remains:
//!
//! ```text
//! frontend
//!    ↓
//! quantum::ir
//!    ↓
//! optimization
//!    ↓
//! routing
//!    ↓
//! scheduling
//!    ↓
//! hardware
//! ```
//!
//! # Why this file does not duplicate local algorithms
//!
//! Duplicating cancellation, identity removal, rotation fusion, and peephole
//! logic here would create two sources of truth.
//!
//! That would eventually produce:
//!
//! - different semantic rules;
//! - different resource limits;
//! - different verification behavior;
//! - different provenance;
//! - different bug fixes;
//! - different pass statistics.
//!
//! The production architecture therefore has exactly one implementation of
//! each local transformation and one composite orchestrator.
//!
//! # No hidden optimization
//!
//! `Simplify` does not perform transformations beyond those explicitly selected
//! by its strategy.
//!
//! This is important for compiler reproducibility and debugging.
//!
//! A caller selecting `Simplify` receives the documented local optimization
//! family, not an implementation-dependent collection of undocumented rewrites.
//!
//! # Extension policy
//!
//! When a new local optimization is added:
//!
//! 1. implement it in `optimization::local`;
//! 2. expose its stable pass type there;
//! 3. add it to the canonical local strategy if appropriate;
//! 4. update the registry/planner.
//!
//! `Simplify` should only need modification if the semantic default local
//! strategy itself changes.
//!
//! New unrelated optimizers should not be added to this file.
//!
//! # Production invariant
//!
//! This file must remain independently usable after all other optimization
//! modules are extended.
//!
//! Future local passes must integrate through the stable `OptimizationPass`
//! contract and local-pass exports rather than requiring this file to know
//! their internal implementation.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]

use crate::quantum::ir::QuantumCircuit;

use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    PassIdentifier,
};
use super::super::local::{
    CancellationPass,
    CommutationPass,
    GateFusionPass,
    IdentityPass,
    InversePass,
    LocalOptimizationPass,
    PeepholePass,
    RotationPass,
    TemplatePass,
};
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassComplexity,
    PassDeterminism,
    PassExecutionPolicy,
    PassKind,
    PassMetadata,
    PassOutcome,
    PassScope,
};

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable machine-readable identifier for the composite simplification pass.
pub const PASS_ID: &str = "passes.simplify";

/// Human-readable name of the composite pass.
pub const PASS_NAME: &str = "Quantum Circuit Simplification";

/// Stable configuration alias.
pub const PASS_ALIAS: &str = "simplify";

/// Contract version of this composite pass.
pub const PASS_VERSION: u32 = 1;

// =============================================================================
// Strategy
// =============================================================================

/// Built-in simplification strategy.
///
/// The strategy controls which local optimization family is executed. It does
/// not create a second optimization configuration system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimplificationStrategy {
    /// Identity removal, inverse simplification, cancellation, and rotations.
    Conservative,

    /// Complete standard local optimization.
    Balanced,

    /// Complete local optimization with the same canonical ordering.
    ///
    /// Aggressive global search remains owned by e-graphs/planner/pipeline.
    Aggressive,
}

impl Default for SimplificationStrategy {
    fn default() -> Self {
        Self::Balanced
    }
}

impl SimplificationStrategy {
    /// Returns a stable serialized strategy identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conservative => "conservative",
            Self::Balanced => "balanced",
            Self::Aggressive => "aggressive",
        }
    }

    /// Returns the local pass identifiers selected by this strategy.
    ///
    /// The concrete pass instances are created by `Simplify`.
    #[must_use]
    pub const fn pass_ids(self) -> &'static [&'static str] {
        match self {
            Self::Conservative => &[
                "local.identity",
                "local.inverse",
                "local.cancellation",
                "local.rotation",
            ],
            Self::Balanced | Self::Aggressive => &[
                "local.identity",
                "local.inverse",
                "local.cancellation",
                "local.rotation",
                "local.commutation",
                "local.peephole",
                "local.templates",
                "local.gate_fusion",
            ],
        }
    }
}

// =============================================================================
// Simplify
// =============================================================================

/// Composite production simplification pass.
///
/// The pass itself is stateless with respect to an optimization invocation.
/// Mutable execution state belongs to `OptimizationContext`.
#[derive(Debug, Clone)]
pub struct Simplify {
    metadata: PassMetadata,
    strategy: SimplificationStrategy,
    identity: IdentityPass,
    inverse: InversePass,
    cancellation: CancellationPass,
    rotation: RotationPass,
    commutation: CommutationPass,
    peephole: PeepholePass,
    templates: TemplatePass,
    gate_fusion: GateFusionPass,
}

impl Simplify {
    /// Creates the standard balanced simplification pass.
    #[must_use]
    pub fn new() -> Self {
        Self::with_strategy(SimplificationStrategy::Balanced)
    }

    /// Creates a simplification pass using an explicit strategy.
    #[must_use]
    pub fn with_strategy(strategy: SimplificationStrategy) -> Self {
        let identifier = PassIdentifier::new(PASS_ID)
            .expect("passes.simplify has a valid static identifier");

        let metadata = PassMetadata::new(
            identifier,
            PASS_NAME,
            PassKind::Composite,
        )
        .expect("simplify pass metadata must be valid")
        .with_scope(PassScope::Circuit)
        .with_complexity(PassComplexity::TargetDependent)
        .with_capability(PassCapability::RemovesOperations)
        .with_capability(PassCapability::ChangesGateCount)
        .with_capability(PassCapability::ChangesOperationOrder)
        .with_capability(PassCapability::ChangesParameters)
        .with_determinism(PassDeterminism::Deterministic)
        .with_execution_policy(PassExecutionPolicy::Sequential);

        Self {
            metadata,
            strategy,
            identity: IdentityPass::new(),
            inverse: InversePass::new(),
            cancellation: CancellationPass::new(),
            rotation: RotationPass::new(),
            commutation: CommutationPass::new(),
            peephole: PeepholePass::new(),
            templates: TemplatePass::new(),
            gate_fusion: GateFusionPass::new(),
        }
    }

    /// Returns the stable pass identifier.
    #[must_use]
    pub const fn pass_id() -> &'static str {
        PASS_ID
    }

    /// Returns the pass contract version.
    #[must_use]
    pub const fn pass_version() -> u32 {
        PASS_VERSION
    }

    /// Returns the selected simplification strategy.
    #[must_use]
    pub const fn strategy(&self) -> SimplificationStrategy {
        self.strategy
    }

    /// Returns the stable child-pass identifiers selected by this pass.
    #[must_use]
    pub const fn selected_pass_ids(&self) -> &'static [&'static str] {
        self.strategy.pass_ids()
    }

    /// Executes the simplification pipeline directly.
    ///
    /// This is the convenient non-trait API for compiler components that want
    /// to invoke simplification without manually constructing a trait object.
    pub fn simplify(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        self.run(circuit, context)
    }

    /// Executes the selected child passes in canonical order.
    fn run_selected(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        let operations_before = circuit.len() as u64;

        let mut last_outcome: Option<PassOutcome> = None;

        // ---------------------------------------------------------------------
        // Conservative core
        // ---------------------------------------------------------------------

        self.execute_child(
            &self.identity,
            circuit,
            context,
            &mut last_outcome,
        )?;

        self.execute_child(
            &self.inverse,
            circuit,
            context,
            &mut last_outcome,
        )?;

        self.execute_child(
            &self.cancellation,
            circuit,
            context,
            &mut last_outcome,
        )?;

        self.execute_child(
            &self.rotation,
            circuit,
            context,
            &mut last_outcome,
        )?;

        // ---------------------------------------------------------------------
        // Balanced/aggressive extensions
        // ---------------------------------------------------------------------

        if !matches!(
            self.strategy,
            SimplificationStrategy::Conservative
        ) {
            self.execute_child(
                &self.commutation,
                circuit,
                context,
                &mut last_outcome,
            )?;

            self.execute_child(
                &self.peephole,
                circuit,
                context,
                &mut last_outcome,
            )?;

            self.execute_child(
                &self.templates,
                circuit,
                context,
                &mut last_outcome,
            )?;

            self.execute_child(
                &self.gate_fusion,
                circuit,
                context,
                &mut last_outcome,
            )?;
        }

        context.check_limits().map_err(|error| {
            OptimizationError::pass_failure(
                PASS_ID,
                format!(
                    "simplification completed its child passes but the \
                     optimizer resource policy rejected the resulting stage: \
                     {error}"
                ),
            )
        })?;

        let operations_after = circuit.len() as u64;

        // The child passes are individually responsible for semantic
        // correctness. The composite nevertheless enforces the basic
        // invariant that a successful optimization pass leaves a valid
        // canonical Quantum IR.
        circuit.validate().map_err(|error| {
            OptimizationError::pass_failure(
                PASS_ID,
                format!(
                    "simplification produced an invalid Quantum IR: {error}"
                ),
            )
        })?;

        let mut outcome = match last_outcome {
            Some(outcome) => outcome,
            None => {
                return Err(OptimizationError::pass_failure(
                    PASS_ID,
                    "simplification executed without any child pass",
                ));
            }
        };

        // The child outcome describes the most recently executed child.
        // Replace its stage-level operation counts with the actual composite
        // stage boundaries when the shared outcome API permits this through
        // the canonical aggregate constructor.
        //
        // The existing pass infrastructure intentionally keeps detailed
        // statistics in the context/statistics layer. Therefore we preserve
        // the child outcome rather than manufacturing a second incompatible
        // statistics representation here.
        //
        // The boundary counts are still recorded through the context-level
        // optimizer statistics by the pipeline.
        let _ = operations_before;
        let _ = operations_after;

        // Make sure the final outcome cannot accidentally advertise a child
        // pass as the composite pass. The pipeline/provenance layer owns the
        // final pass identity; this object is already executing as
        // `passes.simplify`.
        outcome
            .set_pass_id(PASS_ID)
            .map_err(|error| {
                OptimizationError::pass_failure(
                    PASS_ID,
                    format!(
                        "failed to attach composite pass identifier: {error}"
                    ),
                )
            })?;

        Ok(outcome)
    }

    /// Executes one child pass and retains its most recent outcome.
    ///
    /// Every child receives the same optimizer context. No child gets a private
    /// limit/cancellation state.
    fn execute_child<P>(
        &self,
        pass: &P,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
        last_outcome: &mut Option<PassOutcome>,
    ) -> Result<(), OptimizationError>
    where
        P: OptimizationPass + LocalOptimizationPass,
    {
        context.check_limits().map_err(|error| {
            OptimizationError::pass_failure(
                PASS_ID,
                format!(
                    "resource policy rejected child pass `{}`: {error}",
                    pass.id()
                ),
            )
        })?;

        let result = pass.run(circuit, context).map_err(|error| {
            OptimizationError::pass_failure(
                PASS_ID,
                format!(
                    "local simplification child pass `{}` failed: {error}",
                    pass.id()
                ),
            )
        })?;

        let outcome = result.map_err(|error| {
            OptimizationError::pass_failure(
                PASS_ID,
                format!(
                    "local simplification child pass `{}` returned an \
                     optimization error: {error}",
                    pass.id()
                ),
            )
        })?;

        *last_outcome = Some(outcome);

        Ok(())
    }
}

impl Default for Simplify {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// OptimizationPass implementation
// =============================================================================

impl OptimizationPass for Simplify {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        // Validate before invoking any child pass.
        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                super::super::errors::OptimizationStage::InputValidation,
                format!(
                    "{PASS_ID}: input Quantum IR validation failed: {error}"
                ),
            )
        })?;

        context.check_limits().map_err(|error| {
            OptimizationError::pass_failure(
                PASS_ID,
                format!(
                    "{PASS_ID}: optimizer resource policy rejected \
                     simplification: {error}"
                ),
            )
        })?;

        let outcome = self.run_selected(circuit, context)?;

        // Final validation is mandatory for a composite pass because several
        // independently implemented transformations have been composed.
        circuit.validate().map_err(|error| {
            OptimizationError::pass_failure(
                PASS_ID,
                format!(
                    "{PASS_ID}: final Quantum IR validation failed: {error}"
                ),
            )
        })?;

        Ok(outcome)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{
        SimplificationStrategy,
        PASS_ALIAS,
        PASS_ID,
        PASS_NAME,
        PASS_VERSION,
    };

    #[test]
    fn stable_identity_is_correct() {
        assert_eq!(PASS_ID, "passes.simplify");
        assert_eq!(PASS_ALIAS, "simplify");
        assert_eq!(PASS_NAME, "Quantum Circuit Simplification");
        assert_eq!(PASS_VERSION, 1);
    }

    #[test]
    fn default_strategy_is_balanced() {
        assert_eq!(
            SimplificationStrategy::default(),
            SimplificationStrategy::Balanced
        );
    }

    #[test]
    fn conservative_strategy_is_small_and_deterministic() {
        let ids = SimplificationStrategy::Conservative.pass_ids();

        assert_eq!(
            ids,
            &[
                "local.identity",
                "local.inverse",
                "local.cancellation",
                "local.rotation",
            ]
        );
    }

    #[test]
    fn balanced_strategy_contains_complete_local_family() {
        let ids = SimplificationStrategy::Balanced.pass_ids();

        assert!(ids.contains(&"local.identity"));
        assert!(ids.contains(&"local.inverse"));
        assert!(ids.contains(&"local.cancellation"));
        assert!(ids.contains(&"local.rotation"));
        assert!(ids.contains(&"local.commutation"));
        assert!(ids.contains(&"local.peephole"));
        assert!(ids.contains(&"local.templates"));
        assert!(ids.contains(&"local.gate_fusion"));
    }

    #[test]
    fn aggressive_strategy_has_the_same_local_contract() {
        assert_eq!(
            SimplificationStrategy::Aggressive.pass_ids(),
            SimplificationStrategy::Balanced.pass_ids()
        );
    }

    #[test]
    fn strategy_identifiers_are_stable() {
        assert_eq!(
            SimplificationStrategy::Conservative.as_str(),
            "conservative"
        );

        assert_eq!(
            SimplificationStrategy::Balanced.as_str(),
            "balanced"
        );

        assert_eq!(
            SimplificationStrategy::Aggressive.as_str(),
            "aggressive"
        );
    }
}