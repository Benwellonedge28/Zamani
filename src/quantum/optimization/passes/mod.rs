//! Zamani Quantum Optimization — Composite Pass Namespace
//!
//! This module is the authoritative module boundary for the high-level,
//! user-facing/composite optimization passes in:
//!
//! `crate::quantum::optimization::passes`
//!
//! # Architectural position
//!
//! The production optimization architecture is:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                     quantum::frontend
//!                              │
//!                              ▼
//!                       quantum::ir
//!                              │
//!                              ▼
//!                  quantum::optimization
//!                              │
//!              ┌───────────────┴────────────────┐
//!              │                                │
//!              ▼                                ▼
//!       optimization::analysis          optimization::local
//!              │                                │
//!              └───────────────┬────────────────┘
//!                              ▼
//!                 optimization::passes
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!       normalize          simplify          objective passes
//!                                              │
//!                        ┌─────────────────────┼─────────────────────┐
//!                        │                     │                     │
//!                        ▼                     ▼                     ▼
//!                     depth                 width             gate count
//!                        │                     │                     │
//!                        └─────────────────────┼─────────────────────┘
//!                                              ▼
//!                                      two-qubit optimization
//!                                              │
//!                                              ▼
//!                                    fault-tolerant optimization
//!                                              │
//!                                              ▼
//!                                      optimized quantum::ir
//!                                              │
//!                                              ▼
//!                                           routing
//!                                              │
//!                                              ▼
//!                                         scheduling
//!                                              │
//!                                              ▼
//!                                          hardware
//! ```
//!
//! This module is deliberately a namespace and composition boundary.
//!
//! It does **not**:
//!
//! - define a second Quantum IR;
//! - define quantum gates;
//! - own circuit semantics;
//! - implement routing;
//! - implement physical scheduling;
//! - communicate with hardware;
//! - execute a QPU;
//! - execute a simulator;
//! - own benchmarking;
//! - own error correction;
//! - own quantum algorithms;
//! - own source-language parsing;
//! - own global optimizer state;
//! - use unsafe code.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Canonical IR rule
//!
//! Every pass declared here operates, directly or indirectly, on the canonical:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! No pass under this namespace may introduce a private replacement such as:
//!
//! ```text
//! optimization::QuantumCircuit
//! optimization::QuantumGate
//! optimization::PeepholeCircuit
//! ```
//!
//! The historical optimization implementations that used local temporary gate
//! representations must remain outside the production architecture. New passes
//! must use the canonical IR and the `OptimizationPass` contract.
//!
//! # Pass ownership
//!
//! The files in this namespace are **composite policy passes**.
//!
//! They coordinate lower-level transformations while keeping the underlying
//! semantic operations in their appropriate subsystems.
//!
//! For example:
//!
//! ```text
//! passes::simplify
//!       │
//!       ├── local::identity
//!       ├── local::cancellation
//!       ├── local::inverse
//!       ├── local::rotation
//!       ├── local::peephole
//!       ├── local::templates
//!       └── local::gate_fusion
//! ```
//!
//! Likewise:
//!
//! ```text
//! passes::optimize_depth
//!       │
//!       ├── analysis::dependency
//!       ├── analysis::depth
//!       ├── local::commutation
//!       └── local::cancellation
//! ```
//!
//! And:
//!
//! ```text
//! passes::optimize_fault_tolerance
//!       │
//!       ├── algebra::clifford
//!       ├── algebra::phase_polynomial
//!       ├── fault_tolerant::t_gate_reduction
//!       ├── fault_tolerant::t_count
//!       └── fault_tolerant::t_depth
//! ```
//!
//! # Why composite passes exist
//!
//! Individual transformations and high-level optimization objectives are
//! intentionally separated.
//!
//! A caller asking for:
//!
//! ```text
//! minimize depth
//! ```
//!
//! should not need to know which exact cancellation, commutation, algebraic,
//! or synthesis transformations are required to achieve that objective.
//!
//! The objective pass owns that policy.
//!
//! The underlying transformation passes own their own semantics.
//!
//! This separation makes it possible to evolve optimization algorithms without
//! changing the public optimization API.
//!
//! # Current pass families
//!
//! The production namespace currently contains:
//!
//! | Module | Responsibility |
//! |---|---|
//! | `normalize` | Canonical logical-circuit normalization |
//! | `simplify` | General conservative circuit simplification |
//! | `optimize_depth` | Logical depth reduction |
//! | `optimize_width` | Logical width/resource reduction |
//! | `optimize_gate_count` | Total operation/gate-count reduction |
//! | `optimize_two_qubit` | Two-qubit-operation reduction |
//! | `optimize_fault_tolerance` | Fault-tolerant resource optimization |
//!
//! Additional future passes should be added as independent modules when they
//! represent a stable optimization objective or reusable composite strategy.
//!
//! Examples include:
//!
//! ```text
//! optimize_error
//! optimize_duration
//! optimize_measurements
//! optimize_ancillas
//! optimize_energy
//! optimize_logical_cost
//! optimize_physical_cost
//! optimize_pareto
//! optimize_multi_objective
//! ```
//!
//! Such future modules must follow the same contracts defined by `pass.rs`,
//! `context.rs`, `cost.rs`, `pipeline.rs`, `registry.rs`, and `planner.rs`.
//!
//! # Stable identifiers
//!
//! Pass identifiers are deliberately kept as stable strings in the individual
//! pass implementations and in `profile.rs`.
//!
//! This module does **not** duplicate those identifiers.
//!
//! That is intentional.
//!
//! There must be one authoritative identifier for each pass. Duplicating
//! identifiers here would allow registry/profile/provenance mismatches.
//!
//! Consumers that need stable identifiers should use the identifier exported
//! by the concrete pass or the canonical identifier from `profile.rs`.
//!
//! # Pass lifecycle
//!
//! Every pass exposed through this namespace is intended to participate in the
//! following lifecycle:
//!
//! ```text
//! registry
//!    │
//!    ▼
//! planner
//!    │
//!    ▼
//! pipeline
//!    │
//!    ├── validate pass metadata
//!    │
//!    ├── obtain required analyses
//!    │
//!    ├── establish OptimizationContext
//!    │
//!    ├── execute pass
//!    │
//!    ├── validate PassOutcome
//!    │
//!    ├── invalidate affected analyses
//!    │
//!    ├── record statistics
//!    │
//!    └── record provenance
//!    │
//!    ▼
//! next pass
//! ```
//!
//! The pass manager/pipeline owns ordering and fixed-point behavior.
//!
//! Individual passes own only their transformation policy.
//!
//! # Resource scaling
//!
//! These modules must not impose arbitrary circuit-size limits such as:
//!
//! ```text
//! maximum 1,000,000 gates
//! maximum 100,000 qubits
//! ```
//!
//! unless a particular algorithm has an explicit mathematical/resource reason
//! to do so and the limit is supplied through the optimizer's configured
//! resource policy.
//!
//! Scaling is controlled by:
//!
//! - `OptimizationLimits`;
//! - `OptimizationContext`;
//! - target capabilities;
//! - pass complexity;
//! - configured iteration/rewrite budgets;
//! - available memory;
//! - available CPU;
//! - host address-space limits;
//! - verification policy;
//! - planner decisions.
//!
//! Therefore "scale from tiny to infinity" means that this module introduces no
//! artificial global circuit ceiling. Actual execution remains bounded by the
//! resources available to the compilation process and by explicit caller
//! policy.
//!
//! Expensive algorithms must advertise their complexity through `PassMetadata`
//! rather than silently creating an independent resource-management system.
//!
//! # Determinism
//!
//! The module itself contains no mutable global state and creates no threads.
//!
//! Deterministic behavior is the default.
//!
//! If a future composite pass invokes randomized optimization, the underlying
//! pass must declare its determinism policy through `OptimizationPass` metadata
//! and obtain randomness through `OptimizationContext`.
//!
//! No ambient process-global random state may be introduced here.
//!
//! # Verification
//!
//! Composite passes must preserve the semantic-equivalence policy established
//! by the optimizer configuration.
//!
//! They must not silently weaken:
//!
//! ```text
//! exact equivalence
//! ```
//!
//! into:
//!
//! ```text
//! approximate equivalence
//! ```
//!
//! or:
//!
//! ```text
//! measurement equivalence
//! ```
//!
//! without an explicit configuration permitting that policy.
//!
//! Whole-pipeline semantic verification remains owned by:
//!
//! `crate::quantum::optimization::verification`
//!
//! # Target independence
//!
//! These logical objective passes must remain independent of concrete hardware
//! APIs.
//!
//! Target information may enter through:
//!
//! ```text
//! OptimizationContext
//!       │
//!       ▼
//! OptimizationTarget
//!       │
//!       ▼
//! CostModel / target capabilities
//! ```
//!
//! but no pass in this module may call a hardware backend directly.
//!
//! Hardware topology and physical routing remain owned by the routing/hardware
//! subsystems.
//!
//! # Ordering contract
//!
//! The planner is authoritative for the actual pass ordering.
//!
//! The conceptual ordering is:
//!
//! ```text
//! normalize
//!     ↓
//! parameter normalization/folding
//!     ↓
//! simplify
//!     ↓
//! objective optimization
//!     ↓
//! algebraic optimization
//!     ↓
//! synthesis/decomposition
//!     ↓
//! final logical optimization
//!     ↓
//! routing
//!     ↓
//! scheduling
//! ```
//!
//! This module must not hard-code a universal pipeline by executing passes from
//! `mod.rs` itself.
//!
//! That responsibility belongs to `pipeline.rs` and `planner.rs`.
//!
//! This distinction is critical because optimization objectives can conflict.
//!
//! For example:
//!
//! ```text
//! fewer gates
//!       ≠
//! lower depth
//!       ≠
//! fewer two-qubit gates
//!       ≠
//! lower T-count
//!       ≠
//! lower T-depth
//!       ≠
//! lower physical error
//! ```
//!
//! The planner and cost model must determine which objective is dominant.
//!
//! # Integration with `profile.rs`
//!
//! `profile.rs` already defines stable identifiers for the composite passes,
//! including:
//!
//! ```text
//! passes.optimize_depth
//! passes.optimize_width
//! passes.optimize_gate_count
//! passes.optimize_two_qubit
//! passes.optimize_fault_tolerance
//! ```
//!
//! This module intentionally does not redefine those constants.
//!
//! The profile layer describes policy.
//!
//! The registry layer resolves policy identifiers into pass implementations.
//!
//! The planner decides when they should run.
//!
//! # Integration with `registry.rs`
//!
//! `registry.rs` should resolve the stable pass identifiers to concrete pass
//! factories/types.
//!
//! Conceptually:
//!
//! ```text
//! "passes.optimize_depth"
//!          │
//!          ▼
//! passes::optimize_depth
//! ```
//!
//! The registry may therefore import concrete types from these modules without
//! this module depending upward on the registry.
//!
//! This creates the correct dependency direction:
//!
//! ```text
//! passes ──► pass contract
//! registry ──► passes
//! planner ──► registry
//! pipeline ──► registry/planner
//! ```
//!
//! rather than:
//!
//! ```text
//! passes ──► registry ──► passes
//! ```
//!
//! which would create unnecessary coupling.
//!
//! # Integration with `planner.rs`
//!
//! The planner should treat these modules as objective-level candidates.
//!
//! It may select:
//!
//! ```text
//! normalize
//! simplify
//! optimize_gate_count
//! optimize_depth
//! optimize_two_qubit
//! optimize_width
//! optimize_fault_tolerance
//! ```
//!
//! based on:
//!
//! - `OptimizationConfig`;
//! - optimization level;
//! - optimization profile;
//! - optimization objective;
//! - target capabilities;
//! - circuit characteristics;
//! - analysis results;
//! - resource limits;
//! - deterministic/reproducible policy;
//! - verification requirements.
//!
//! The planner must not assume every pass is suitable for every circuit.
//!
//! # Integration with `pipeline.rs`
//!
//! The pipeline owns:
//!
//! - pass sequencing;
//! - pass invocation;
//! - fixed-point iteration;
//! - pass failure handling;
//! - resource-limit handling;
//! - analysis invalidation;
//! - provenance;
//! - final verification.
//!
//! `passes/mod.rs` only exposes the pass namespace.
//!
//! # Integration with `analysis`
//!
//! Objective passes may consume analyses such as:
//!
//! ```text
//! dependency
//! commutation
//! depth
//! width
//! critical_path
//! gate_counts
//! qubit_use
//! liveness
//! parameter_usage
//! entanglement
//! ```
//!
//! They must request those analyses through the optimizer's established
//! analysis/context contracts rather than recomputing global information in
//! every objective pass.
//!
//! # Integration with `local`
//!
//! Local transformations remain reusable building blocks.
//!
//! Composite passes may use:
//!
//! ```text
//! local::identity
//! local::inverse
//! local::cancellation
//! local::rotation
//! local::commutation
//! local::peephole
//! local::templates
//! local::gate_fusion
//! ```
//!
//! The composite pass must not copy their gate semantics into its own file.
//!
//! # Integration with `algebra`
//!
//! Algebraic transformations remain reusable specialized engines.
//!
//! Composite passes may coordinate:
//!
//! ```text
//! algebra::pauli
//! algebra::clifford
//! algebra::phase_polynomial
//! algebra::diagonal
//! algebra::symplectic
//! ```
//!
//! # Integration with `synthesis`
//!
//! Synthesis remains responsible for converting equivalent operations into
//! representations appropriate for the selected target/cost model.
//!
//! Composite passes may request synthesis but must not reimplement arbitrary
//! unitary decomposition.
//!
//! # Integration with `fault_tolerant`
//!
//! Fault-tolerant optimization is a separate objective family.
//!
//! `optimize_fault_tolerance` may coordinate:
//!
//! ```text
//! Clifford optimization
//! phase-polynomial optimization
//! T-count reduction
//! T-depth reduction
//! logical resource estimation
//! ```
//!
//! Ordinary gate-count optimization must not silently substitute for
//! fault-tolerant optimization.
//!
//! # Integration with routing
//!
//! These passes operate on logical circuits.
//!
//! Routing remains downstream:
//!
//! ```text
//! logical optimization
//!       ↓
//! routing
//!       ↓
//! physical optimization where required
//!       ↓
//! scheduling
//! ```
//!
//! Some compilation strategies may deliberately invoke another logical
//! optimization stage after routing. That orchestration belongs to the parent
//! pipeline/planner, not this module.
//!
//! # Integration with scheduling
//!
//! `optimize_depth` calculates logical depth where appropriate.
//!
//! It must not claim to optimize wall-clock execution time.
//!
//! Physical duration, pulse overlap, delays, dynamical decoupling, and timing
//! constraints belong to scheduling/hardware.
//!
//! # Integration with benchmarking
//!
//! Benchmarking consumes optimization results.
//!
//! This module must never depend on benchmarking.
//!
//! The intended direction is:
//!
//! ```text
//! optimization
//!      │
//!      ▼
//! OptimizationResult
//!      │
//!      ▼
//! benchmarking
//! ```
//!
//! This prevents circular architecture and allows benchmarking to compare
//! multiple optimization configurations.
//!
//! # Integration with error correction
//!
//! Fault-tolerant cost objectives may consume logical-resource information, but
//! QEC code semantics remain in `quantum::error_correction`.
//!
//! This module must not implement:
//!
//! - stabilizer codes;
//! - surface codes;
//! - syndrome extraction;
//! - decoding;
//! - logical-qubit construction.
//!
//! # Integration with algorithms
//!
//! Quantum algorithms construct canonical circuits.
//!
//! The direction is:
//!
//! ```text
//! algorithms
//!      ↓
//! quantum::ir
//!      ↓
//! optimization::passes
//! ```
//!
//! The optimization subsystem must not embed VQE, QAOA, Grover, Shor, or other
//! algorithm definitions.
//!
//! # Public API policy
//!
//! The preferred public paths are explicit:
//!
//! ```text
//! crate::quantum::optimization::passes::normalize
//! crate::quantum::optimization::passes::simplify
//! crate::quantum::optimization::passes::optimize_depth
//! crate::quantum::optimization::passes::optimize_width
//! crate::quantum::optimization::passes::optimize_gate_count
//! crate::quantum::optimization::passes::optimize_two_qubit
//! crate::quantum::optimization::passes::optimize_fault_tolerance
//! ```
//!
//! Concrete pass types should remain owned by their implementation modules.
//!
//! This module should not flatten every implementation symbol into one huge
//! namespace. Keeping the module boundary explicit makes future additions
//! backwards-compatible and avoids name collisions.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden for this module.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! The child optimization passes are likewise expected to use safe Rust only.
//!
//! # Module declaration policy
//!
//! The declarations below are intentionally explicit.
//!
//! Rust requires the module tree to be declared by the parent module. Keeping
//! these declarations here means adding a new composite pass requires only:
//!
//! 1. adding the new implementation file;
//! 2. declaring it here;
//! 3. registering it with `registry.rs`;
//! 4. assigning it to appropriate profiles/planner policies.
//!
//! Existing passes do not need to be edited merely because another independent
//! pass is introduced.
//!
//! This is the intended integration boundary for the user's "finish one file
//! before moving to the next file" development strategy.
//!
//! # Current implementation inventory
//!
//! This namespace intentionally declares only files that currently exist in
//! the repository:
//!
//! ```text
//! normalize.rs
//! simplify.rs
//! optimize_depth.rs
//! optimize_width.rs
//! optimize_gate_count.rs
//! optimize_two_qubit.rs
//! optimize_fault_tolerance.rs
//! ```
//!
//! Future modules must not be declared until their implementation contracts are
//! complete enough to compile and satisfy the pass framework.
//!
//! # No hidden execution
//!
//! Merely importing this module must never execute optimization.
//!
//! In particular, `mod.rs` must not:
//!
//! - construct a global optimizer;
//! - allocate a global registry;
//! - optimize a circuit;
//! - spawn worker threads;
//! - initialize hardware;
//! - access the filesystem;
//! - access the network;
//! - access environment variables;
//! - generate random numbers.
//!
//! Construction and execution are explicit operations owned by the caller,
//! registry, planner, and pipeline.
//!
//! # Architectural invariant
//!
//! The most important invariant in this file is:
//!
//! ```text
//! passes/mod.rs = namespace + composition boundary
//! ```
//!
//! It is not:
//!
//! ```text
//! passes/mod.rs = optimizer implementation
//! ```
//!
//! Keeping that distinction makes the subsystem scalable, testable,
//! deterministic, and independently extensible.

// No unsafe code is permitted in this module or its direct implementation.
#![forbid(unsafe_code)]

// =============================================================================
// Composite optimization passes
// =============================================================================

/// Canonical logical-circuit normalization.
///
/// This pass establishes the representation expected by subsequent
/// optimization stages before more aggressive transformations are attempted.
pub mod normalize;

/// General-purpose logical circuit simplification.
///
/// This is the conservative composite simplification stage that coordinates
/// reusable local transformations.
pub mod simplify;

/// Logical circuit-depth optimization.
///
/// This pass optimizes logical depth and deliberately does not own physical
/// hardware scheduling or wall-clock timing.
pub mod optimize_depth;

/// Logical circuit-width/resource optimization.
///
/// This pass targets logical qubit/ancilla pressure and relies on canonical
/// liveness/width analyses.
pub mod optimize_width;

/// Total logical gate/operation-count optimization.
///
/// This pass optimizes overall operation cost according to the configured
/// logical cost policy.
pub mod optimize_gate_count;

/// Two-qubit-operation optimization.
///
/// This pass is specialized for reducing expensive entangling operations and
/// related logical two-qubit cost.
pub mod optimize_two_qubit;

/// Fault-tolerant resource optimization.
///
/// This pass coordinates Clifford/T/phase-polynomial and logical-resource
/// optimization without owning QEC semantics.
pub mod optimize_fault_tolerance;

// =============================================================================
// Stable pass-family inventory
// =============================================================================

/// Stable category of a high-level composite optimization pass.
///
/// This enum is intentionally independent of concrete implementation types.
/// It allows diagnostics, documentation, and future registry/planner code to
/// describe the available objective families without duplicating pass metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassFamily {
    /// Canonical circuit normalization.
    Normalize,

    /// General logical simplification.
    Simplify,

    /// Logical depth optimization.
    Depth,

    /// Logical width/resource optimization.
    Width,

    /// Total gate/operation-count optimization.
    GateCount,

    /// Two-qubit-operation optimization.
    TwoQubit,

    /// Fault-tolerant resource optimization.
    FaultTolerance,
}

impl PassFamily {
    /// Returns the stable optimization namespace identifier.
    ///
    /// These values intentionally match the stable identifiers used by the
    /// optimizer profile/planner layer.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Normalize => "normalize",
            Self::Simplify => "simplify",
            Self::Depth => "passes.optimize_depth",
            Self::Width => "passes.optimize_width",
            Self::GateCount => "passes.optimize_gate_count",
            Self::TwoQubit => "passes.optimize_two_qubit",
            Self::FaultTolerance => "passes.optimize_fault_tolerance",
        }
    }

    /// Returns the Rust module path relative to
    /// `crate::quantum::optimization::passes`.
    #[must_use]
    pub const fn module_path(self) -> &'static str {
        match self {
            Self::Normalize => "normalize",
            Self::Simplify => "simplify",
            Self::Depth => "optimize_depth",
            Self::Width => "optimize_width",
            Self::GateCount => "optimize_gate_count",
            Self::TwoQubit => "optimize_two_qubit",
            Self::FaultTolerance => "optimize_fault_tolerance",
        }
    }

    /// Returns the human-readable objective family.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Normalize => "canonical logical-circuit normalization",
            Self::Simplify => "general logical circuit simplification",
            Self::Depth => "logical circuit-depth optimization",
            Self::Width => "logical circuit-width/resource optimization",
            Self::GateCount => "total logical gate/operation-count optimization",
            Self::TwoQubit => "logical two-qubit-operation optimization",
            Self::FaultTolerance => "fault-tolerant resource optimization",
        }
    }

    /// Returns every currently available composite pass family.
    ///
    /// The ordering is documentation-oriented and is not a promise that the
    /// planner must execute passes in this order. Pipeline ordering belongs to
    /// `planner.rs` and `pipeline.rs`.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Normalize,
            Self::Simplify,
            Self::Depth,
            Self::Width,
            Self::GateCount,
            Self::TwoQubit,
            Self::FaultTolerance,
        ]
    }

    /// Returns whether the pass is normally a prerequisite/normalization
    /// family rather than an optimization objective.
    #[must_use]
    pub const fn is_normalization(self) -> bool {
        matches!(self, Self::Normalize)
    }

    /// Returns whether this family is an objective optimizer.
    #[must_use]
    pub const fn is_objective_optimizer(self) -> bool {
        matches!(
            self,
            Self::Depth
                | Self::Width
                | Self::GateCount
                | Self::TwoQubit
                | Self::FaultTolerance
        )
    }
}

// =============================================================================
// Compile-time inventory tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::PassFamily;

    #[test]
    fn all_current_composite_pass_families_are_present() {
        let families = PassFamily::all();

        assert_eq!(families.len(), 7);

        assert!(families.contains(&PassFamily::Normalize));
        assert!(families.contains(&PassFamily::Simplify));
        assert!(families.contains(&PassFamily::Depth));
        assert!(families.contains(&PassFamily::Width));
        assert!(families.contains(&PassFamily::GateCount));
        assert!(families.contains(&PassFamily::TwoQubit));
        assert!(families.contains(&PassFamily::FaultTolerance));
    }

    #[test]
    fn pass_family_identifiers_are_stable() {
        assert_eq!(PassFamily::Normalize.id(), "normalize");
        assert_eq!(PassFamily::Simplify.id(), "simplify");
        assert_eq!(
            PassFamily::Depth.id(),
            "passes.optimize_depth"
        );
        assert_eq!(
            PassFamily::Width.id(),
            "passes.optimize_width"
        );
        assert_eq!(
            PassFamily::GateCount.id(),
            "passes.optimize_gate_count"
        );
        assert_eq!(
            PassFamily::TwoQubit.id(),
            "passes.optimize_two_qubit"
        );
        assert_eq!(
            PassFamily::FaultTolerance.id(),
            "passes.optimize_fault_tolerance"
        );
    }

    #[test]
    fn module_paths_are_stable() {
        assert_eq!(
            PassFamily::Normalize.module_path(),
            "normalize"
        );
        assert_eq!(
            PassFamily::Simplify.module_path(),
            "simplify"
        );
        assert_eq!(
            PassFamily::Depth.module_path(),
            "optimize_depth"
        );
        assert_eq!(
            PassFamily::Width.module_path(),
            "optimize_width"
        );
        assert_eq!(
            PassFamily::GateCount.module_path(),
            "optimize_gate_count"
        );
        assert_eq!(
            PassFamily::TwoQubit.module_path(),
            "optimize_two_qubit"
        );
        assert_eq!(
            PassFamily::FaultTolerance.module_path(),
            "optimize_fault_tolerance"
        );
    }

    #[test]
    fn_only_normalize_is_marked_as_normalization() {
        assert!(PassFamily::Normalize.is_normalization());

        assert!(!PassFamily::Simplify.is_normalization());
        assert!(!PassFamily::Depth.is_normalization());
        assert!(!PassFamily::Width.is_normalization());
        assert!(!PassFamily::GateCount.is_normalization());
        assert!(!PassFamily::TwoQubit.is_normalization());
        assert!(!PassFamily::FaultTolerance.is_normalization());
    }

    #[test]
    fn objective_pass_classification_is_correct() {
        assert!(!PassFamily::Normalize.is_objective_optimizer());
        assert!(!PassFamily::Simplify.is_objective_optimizer());

        assert!(PassFamily::Depth.is_objective_optimizer());
        assert!(PassFamily::Width.is_objective_optimizer());
        assert!(PassFamily::GateCount.is_objective_optimizer());
        assert!(PassFamily::TwoQubit.is_objective_optimizer());
        assert!(PassFamily::FaultTolerance.is_objective_optimizer());
    }

    #[test]
    fn every_family_has_a_description() {
        for family in PassFamily::all() {
            assert!(!family.description().is_empty());
        }
    }
}