//! Zamani Quantum Optimization Subsystem
//!
//! Production-grade, backend-independent optimization framework for the
//! canonical Zamani Quantum IR.
//!
//! # Architectural position
//!
//! The optimization subsystem sits strictly between logical Quantum IR and
//! downstream physical compilation stages:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                    quantum::frontend
//!                              │
//!                              ▼
//!                       quantum::ir
//!                              │
//!                              ▼
//!                 quantum::optimization
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!          routing         error correction   analysis
//!             │                │
//!             ▼                ▼
//!         scheduling       hardware
//!             │                │
//!             └────────┬───────┘
//!                      ▼
//!                   runtime
//! ```
//!
//! Optimization owns logical circuit transformation. It does NOT own:
//!
//! - source parsing;
//! - frontend lowering;
//! - quantum algorithms;
//! - hardware APIs;
//! - physical topology;
//! - logical-to-physical routing;
//! - execution scheduling;
//! - QPU communication;
//! - benchmark orchestration;
//! - quantum error-correction semantics;
//! - runtime execution.
//!
//! Those responsibilities remain in their owning quantum subsystems.
//!
//! # Canonical IR rule
//!
//! The authoritative quantum representation is:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! Optimization MUST NOT define a competing `QuantumGate`, `QuantumOperation`,
//! `QuantumCircuit`, `QubitId`, or equivalent semantic representation.
//!
//! In particular, code requiring qubit identifiers MUST use the canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! and not:
//!
//! ```text
//! crate::quantum::ir::qubits::QubitId
//! ```
//!
//! The canonical repository module is `quantum::ir::qubit` because the
//! implementation is `src/quantum/ir/qubit.rs`.
//!
//! # Production architecture
//!
//! The subsystem is divided into independent contracts:
//!
//! ```text
//! config / profile / limits
//!             │
//!             ▼
//!         context
//!             │
//!             ├───────────────┐
//!             ▼               ▼
//!        analyses          targets
//!             │               │
//!             └───────┬───────┘
//!                     ▼
//!                  planner
//!                     │
//!                     ▼
//!                 pipeline
//!                     │
//!       ┌─────────────┼──────────────┐
//!       ▼             ▼              ▼
//!     local        algebra       synthesis
//!       │             │              │
//!       ├─────────────┼──────────────┤
//!       │             ▼              │
//!       │       fault_tolerant       │
//!       │             │              │
//!       └─────────────┼──────────────┘
//!                     ▼
//!                  rewrite
//!                     │
//!                     ▼
//!                  result
//!                     │
//!              ┌──────┴──────┐
//!              ▼             ▼
//!         verification   provenance
//! ```
//!
//! This organization permits tiny circuits and extremely large circuits to
//! use the same contracts. Practical scalability is governed by explicit
//! resource policies rather than an artificial circuit-size ceiling.
//!
//! # Scaling model
//!
//! The optimizer is intentionally resource-driven.
//!
//! It must be able to operate on:
//!
//! - one-operation circuits;
//! - small educational circuits;
//! - thousands of operations;
//! - millions of operations;
//! - very large generated workloads;
//! - workloads approaching the limits of available memory/CPU;
//! - future distributed or partitioned optimization strategies.
//!
//! No module in this namespace should introduce a hidden fixed circuit-size
//! limit merely for convenience.
//!
//! Resource boundaries belong to:
//!
//! - `limits`;
//! - `config`;
//! - `context`;
//! - pipeline limits;
//! - pass-specific budgets;
//! - e-graph limits;
//! - verification limits.
//!
//! A workload that exceeds an explicitly configured resource boundary must
//! terminate according to policy rather than silently exhausting resources.
//!
//! # Determinism
//!
//! Deterministic optimization is a first-class compiler property.
//!
//! The subsystem therefore supports:
//!
//! - deterministic passes;
//! - explicitly seeded stochastic passes;
//! - reproducible configuration;
//! - stable pass identifiers;
//! - stable provenance;
//! - deterministic pipeline ordering;
//! - bounded fixed-point execution.
//!
//! No optimizer module may depend on ambient process-global randomness.
//!
//! # Safety
//!
//! This entire subsystem forbids unsafe Rust.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No optimization transformation requires unsafe Rust.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization foundation
//!      │
//!      ├── analysis
//!      ├── targets
//!      ├── parameter
//!      ├── structure
//!      ├── local
//!      ├── algebra
//!      ├── synthesis
//!      ├── fault_tolerant
//!      ├── stochastic
//!      └── verification
//!              │
//!              ▼
//!          pipeline
//!              │
//!              ▼
//!       optimized quantum::ir
//! ```
//!
//! The optimization subsystem must never introduce an upward dependency from
//! Quantum IR into optimization.
//!
//! # Ownership boundaries
//!
//! ## `canonical`
//!
//! Owns optimization-level canonicalization and normalization.
//!
//! ## `circuit`
//!
//! Provides optimization-specific views/editing facilities over the canonical
//! Quantum IR without defining a second IR.
//!
//! ## `operation`
//!
//! Provides semantic classification of canonical IR operations for optimizer
//! use.
//!
//! ## `analysis`
//!
//! Computes immutable facts about circuits:
//!
//! - dependencies;
//! - qubit use;
//! - liveness;
//! - commutation;
//! - depth;
//! - width;
//! - critical path;
//! - gate counts;
//! - parameter usage;
//! - entanglement/interaction.
//!
//! Analyses do not transform circuits.
//!
//! ## `pass`
//!
//! Defines the stable optimization-pass contract.
//!
//! ## `pipeline`
//!
//! Executes passes, fixed-point loops, validation boundaries, progress
//! detection, limits, and pipeline-level policies.
//!
//! ## `scheduler`
//!
//! Schedules optimization passes and analysis work. It is NOT quantum hardware
//! scheduling.
//!
//! ## `registry`
//!
//! Owns pass discovery and registration.
//!
//! ## `planner`
//!
//! Selects suitable passes and pipelines from circuit characteristics,
//! configuration, target capabilities, objectives, and resource policy.
//!
//! ## `rules` / `pattern` / `matcher` / `rewrite`
//!
//! Own generic rewrite infrastructure.
//!
//! ## `egraph`
//!
//! Provides bounded equality-saturation infrastructure for aggressive
//! optimization.
//!
//! ## `local`
//!
//! Owns local circuit transformations:
//!
//! - identity elimination;
//! - inverse cancellation;
//! - self-inverse cancellation;
//! - rotation fusion;
//! - commutation;
//! - peephole rewriting;
//! - template optimization;
//! - gate fusion.
//!
//! ## `algebra`
//!
//! Owns mathematical representations and transformations such as:
//!
//! - Pauli algebra;
//! - Clifford algebra;
//! - diagonal circuits;
//! - phase polynomials;
//! - symplectic representations.
//!
//! ## `synthesis`
//!
//! Owns unitary/isometry/Clifford/phase and target-compatible synthesis.
//!
//! ## `fault_tolerant`
//!
//! Owns logical resource optimization such as:
//!
//! - Clifford+T optimization;
//! - T-count;
//! - T-depth;
//! - magic-state cost;
//! - logical resource estimation.
//!
//! ## `parameter`
//!
//! Owns symbolic and constant parameter optimization.
//!
//! ## `structure`
//!
//! Owns optimization of logical blocks, regions, loops, conditionals, and
//! control flow while preserving semantic boundaries.
//!
//! ## `stochastic`
//!
//! Owns explicitly stochastic or approximate optimization techniques.
//!
//! ## `targets`
//!
//! Defines optimization targets, legal gate sets, target constraints, and
//! target optimization profiles.
//!
//! Physical topology itself remains outside this subsystem.
//!
//! ## `passes`
//!
//! Provides composite optimization strategies such as:
//!
//! - normalization;
//! - simplification;
//! - gate-count optimization;
//! - depth optimization;
//! - width optimization;
//! - two-qubit optimization;
//! - fault-tolerant optimization.
//!
//! ## `verification`
//!
//! Owns structural and semantic validation/equivalence checking of optimizer
//! transformations.
//!
//! ## `statistics`
//!
//! Owns standardized optimization measurements.
//!
//! ## `provenance`
//!
//! Owns reproducibility and transformation history.
//!
//! ## `result`
//!
//! Owns final optimizer result reporting.
//!
//! ## `serialization`
//!
//! Owns stable serialization adapters for configuration, reports, and
//! provenance.
//!
//! # Integration with quantum::ir
//!
//! The canonical IR currently exposes:
//!
//! ```text
//! quantum::ir::analysis
//! quantum::ir::circuit
//! quantum::ir::errors
//! quantum::ir::gate
//! quantum::ir::identity
//! quantum::ir::limits
//! quantum::ir::measurement
//! quantum::ir::parameter
//! quantum::ir::qubit
//! quantum::ir::validation
//! ```
//!
//! Optimization consumes those facilities. 
//!
//! The optimizer must not duplicate their semantics.
//!
//! # Integration with frontend
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! optimization
//! ```
//!
//! Frontends do not select individual optimization implementation details.
//!
//! # Integration with algorithms
//!
//! ```text
//! quantum::algorithms
//!          │
//!          ▼
//!      quantum::ir
//!          │
//!          ▼
//!    optimization
//! ```
//!
//! Algorithms may construct circuits intended for optimization, but algorithm
//! semantics remain outside this subsystem.
//!
//! # Integration with routing
//!
//! The preferred logical compilation flow is:
//!
//! ```text
//! logical IR
//!     │
//!     ▼
//! optimization
//!     │
//!     ▼
//! routing
//!     │
//!     ▼
//! physical/target-aware optimization where explicitly requested
//!     │
//!     ▼
//! scheduling
//! ```
//!
//! Optimization may consume abstract target cost information but must not own
//! physical topology or routing algorithms.
//!
//! # Integration with scheduling
//!
//! Optimization may expose logical:
//!
//! - dependency information;
//! - depth;
//! - critical path;
//! - parallelism opportunities.
//!
//! Actual execution timing remains owned by `quantum::scheduling`.
//!
//! # Integration with hardware
//!
//! Hardware capabilities are represented to optimization through target
//! abstractions. Optimization must never directly call a QPU provider.
//!
//! ```text
//! hardware capability
//!        │
//!        ▼
//! optimization target
//!        │
//!        ▼
//! optimizer
//! ```
//!
//! # Integration with error correction
//!
//! Fault-tolerant optimization operates on logical circuits and logical
//! resource objectives. Error-correction code semantics remain owned by
//! `quantum::error_correction`.
//!
//! # Integration with benchmarking
//!
//! Benchmarking is a consumer of optimization results, not a dependency of
//! optimization.
//!
//! ```text
//! original circuit
//!       │
//!       ├──── analysis
//!       │
//!       ▼
//! optimization
//!       │
//!       ▼
//! optimized circuit
//!       │
//!       └──── analysis
//!                  │
//!                  ▼
//!             benchmarking
//! ```
//!
//! The quantum root explicitly defines benchmarking as a consumer/orchestration
//! subsystem rather than a dependency of lower semantic layers. 
//!
//! # Integration with reproducibility
//!
//! Every optimizer invocation should be capable of recording:
//!
//! - configuration;
//! - profile;
//! - target;
//! - pass order;
//! - pass identifiers;
//! - deterministic seed when applicable;
//! - input/output fingerprints;
//! - resource limits;
//! - statistics;
//! - verification outcome.
//!
//! # Integration with current repository structure
//!
//! The repository already contains the complete foundation and major optimizer
//! namespaces:
//!
//! ```text
//! optimization/
//! ├── algebra/
//! ├── analysis/
//! ├── canonical.rs
//! ├── circuit.rs
//! ├── config.rs
//! ├── context.rs
//! ├── cost.rs
//! ├── egraph.rs
//! ├── equivalence.rs
//! ├── errors.rs
//! ├── fault_tolerant/
//! ├── limits.rs
//! ├── local/
//! ├── matcher.rs
//! ├── operation.rs
//! ├── parameter/
//! ├── pass.rs
//! ├── passes/
//! ├── pattern.rs
//! ├── pipeline.rs
//! ├── planner.rs
//! ├── profile.rs
//! ├── provenance.rs
//! ├── registry.rs
//! ├── result.rs
//! ├── rewrite.rs
//! ├── rules.rs
//! ├── scheduler.rs
//! ├── serialization/
//! ├── statistics.rs
//! ├── stochastic/
//! ├── structure/
//! ├── synthesis/
//! ├── targets/
//! ├── tests/
//! ├── validation.rs
//! └── verification/
//! ```
//!
//! These files are already present in the repository. 
//!
//! The nested namespaces likewise already have their production module
//! boundaries. For example, local optimization exposes cancellation,
//! commutation, fusion, identity, inverse, peephole, rotation, and templates.
//! 
//!
//! Fault-tolerant optimization contains Clifford+T, logical cost, magic-state,
//! T-count, T-depth, and T-gate reduction components. 
//!
//! Parameter optimization contains binding, constant folding, symbolic
//! simplification, and general simplification. 
//!
//! Target optimization contains constraints, gate sets, target definitions,
//! and target profiles. 
//!
//! Verification contains semantic, structural, randomized, exhaustive, and
//! certificate-based verification. 
//!
//! # Public namespace policy
//!
//! This file is intentionally the namespace boundary rather than a giant
//! centralized type-definition file.
//!
//! Concrete types remain owned by their implementation modules.
//!
//! This has an important consequence:
//!
//! Adding a new optimization pass must NOT require editing unrelated foundation
//! files merely to make its type visible.
//!
//! A new pass should:
//!
//! 1. implement `OptimizationPass`;
//! 2. register itself through `registry`;
//! 3. declare its metadata;
//! 4. declare its analysis requirements;
//! 5. declare its invalidation effects;
//! 6. become selectable by the planner/pipeline.
//!
//! Existing foundation files should remain stable.
//!
//! # Fixed-point optimization
//!
//! Production optimization may require iterative execution.
//!
//! The pipeline therefore owns:
//!
//! - iteration limits;
//! - pass execution limits;
//! - no-progress detection;
//! - cancellation;
//! - time limits;
//! - validation boundaries;
//! - progress fingerprints.
//!
//! This mirrors modern quantum compiler practice in which optimization stages
//! can contain iterative loops rather than being restricted to one linear pass.
//! 12
//!
//! # Multi-objective optimization
//!
//! Optimization must not assume gate count is the only objective.
//!
//! The subsystem supports objectives such as:
//!
//! - gate count;
//! - two-qubit count;
//! - depth;
//! - T-count;
//! - T-depth;
//! - logical error cost;
//! - duration;
//! - width;
//! - target-specific cost.
//!
//! A target can therefore prefer a circuit with more total gates if it has
//! substantially fewer expensive two-qubit operations.
//!
//! # Hardware boundary
//!
//! Modern quantum compiler stacks distinguish abstract/logical optimization
//! from hardware-aware optimization and then separately handle layout/routing,
//! translation, and scheduling. Zamani preserves that ownership separation.
//! 13
//!
//! # Tests
//!
//! The optimization integration tests are deliberately kept under
//! `optimization/tests/` and are wired through that directory's own `mod.rs`.
//!
//! This root module only enables the test namespace under `cfg(test)`.
//!
//! # No legacy competing namespace
//!
//! The old architecture in which optimization contained separate local
//! `QuantumGate` definitions is not permitted in the production architecture.
//!
//! The current local namespace explicitly documents that it must not introduce
//! an alternative quantum IR. 
//!
//! # Module declarations
//!
//! Keep these declarations explicit. They make the complete optimizer
//! architecture discoverable from one file and prevent hidden module ownership.
//!
//! Adding a child module should normally require only this declaration and its
//! own implementation/test integration.
//!
//! No implementation logic belongs in this file.
//!
//! -----------------------------------------------------------------------------
//! Canonical optimization foundation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]

/// Algebraic quantum-circuit representations and transformations.
pub mod algebra;

/// Immutable logical-circuit analyses used by optimization passes.
pub mod analysis;

/// Canonical optimization normalization and canonical-form handling.
pub mod canonical;

/// Optimization-specific circuit views and transactional editing facilities.
pub mod circuit;

/// User/compiler-facing optimization configuration.
pub mod config;

/// Invocation-scoped optimization context and analysis/resource state.
pub mod context;

/// Quantum optimization cost models and objectives.
pub mod cost;

/// Bounded equality-saturation/e-graph optimization infrastructure.
pub mod egraph;

/// Circuit semantic-equivalence policy and equivalence services.
pub mod equivalence;

/// Optimization-specific error vocabulary.
pub mod errors;

/// Fault-tolerant and logical-resource optimization.
pub mod fault_tolerant;

/// Global optimization resource limits and policies.
pub mod limits;

/// Local circuit simplification and rewrite passes.
pub mod local;

/// Pattern matching support.
pub mod matcher;

/// Optimization-level semantic operation classification.
pub mod operation;

/// Symbolic and constant parameter optimization.
pub mod parameter;

/// Stable optimization-pass contract.
pub mod pass;

/// Composite, user-facing optimization strategies.
pub mod passes;

/// Generic rewrite-pattern representation.
pub mod pattern;

/// Optimization pipeline execution engine.
pub mod pipeline;

/// Automatic optimization pipeline planning.
pub mod planner;

/// Named optimization profiles.
pub mod profile;

/// Optimization provenance and reproducibility metadata.
pub mod provenance;

/// Pass registry and pass discovery.
pub mod registry;

/// Final optimization result/reporting types.
pub mod result;

/// Generic circuit rewrite engine.
pub mod rewrite;

/// Rewrite-rule metadata and rule catalog.
pub mod rules;

/// Optimization-pass scheduling and analysis scheduling.
pub mod scheduler;

/// Serialization adapters for optimization artifacts.
pub mod serialization;

/// Standardized optimization statistics.
pub mod statistics;

/// Explicitly stochastic/approximate optimization methods.
pub mod stochastic;

/// Structural/control-flow optimization.
pub mod structure;

/// Unitary, Clifford, phase, isometry, and decomposition synthesis.
pub mod synthesis;

/// Target gate sets, target constraints, and target profiles.
pub mod targets;

/// Optimization-specific structural and semantic validation.
pub mod validation;

/// Semantic, structural, randomized, exhaustive, and certificate verification.
pub mod verification;

// -----------------------------------------------------------------------------
// Integration test namespace
// -----------------------------------------------------------------------------

/// Cross-module optimization tests.
///
/// The `tests/mod.rs` file owns its own test-module composition, keeping this
/// root namespace independent from individual test files.
#[cfg(test)]
mod tests;

// -----------------------------------------------------------------------------
// Stable architectural constants
// -----------------------------------------------------------------------------

/// Stable identifier for the Zamani quantum optimization subsystem.
pub const OPTIMIZATION_SUBSYSTEM_ID: &str = "zamani.quantum.optimization";

/// Major architectural version of the optimization subsystem contract.
///
/// This is intentionally independent from the Zamani language version and
/// from individual optimization-pass versions.
pub const OPTIMIZATION_ARCHITECTURE_VERSION: u32 = 1;

/// Minimum supported Rust major version.
pub const MIN_RUST_MAJOR: u32 = 1;

/// Minimum supported Rust minor version.
pub const MIN_RUST_MINOR: u32 = 97;

/// Whether this subsystem guarantees that it contains no unsafe Rust.
///
/// Kept as a compile-time/documentation contract for integration tooling.
pub const UNSAFE_CODE_ALLOWED: bool = false;

// -----------------------------------------------------------------------------
// Architectural capability description
// -----------------------------------------------------------------------------

/// High-level capabilities exposed by the optimization subsystem.
///
/// This is intentionally a stable description of subsystem capabilities rather
/// than a registry of concrete passes. Concrete pass discovery belongs to
/// `registry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationCapability {
    /// Canonical IR normalization.
    Canonicalization,

    /// Immutable circuit analysis.
    Analysis,

    /// Local circuit rewriting.
    LocalRewrite,

    /// Algebraic optimization.
    AlgebraicRewrite,

    /// Generic pattern/rewrite optimization.
    Rewrite,

    /// Equality-saturation optimization.
    EqualitySaturation,

    /// Parameter optimization.
    ParameterOptimization,

    /// Structural/control-flow optimization.
    StructuralOptimization,

    /// Unitary/Clifford/phase synthesis.
    Synthesis,

    /// Fault-tolerant resource optimization.
    FaultTolerantOptimization,

    /// Target-aware optimization.
    TargetAwareOptimization,

    /// Stochastic/approximate optimization.
    StochasticOptimization,

    /// Semantic equivalence verification.
    Verification,

    /// Reproducibility/provenance.
    Provenance,

    /// Stable result/statistics reporting.
    Reporting,
}

impl OptimizationCapability {
    /// Returns the stable capability identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canonicalization => "canonicalization",
            Self::Analysis => "analysis",
            Self::LocalRewrite => "local_rewrite",
            Self::AlgebraicRewrite => "algebraic_rewrite",
            Self::Rewrite => "rewrite",
            Self::EqualitySaturation => "equality_saturation",
            Self::ParameterOptimization => "parameter_optimization",
            Self::StructuralOptimization => "structural_optimization",
            Self::Synthesis => "synthesis",
            Self::FaultTolerantOptimization => "fault_tolerant_optimization",
            Self::TargetAwareOptimization => "target_aware_optimization",
            Self::StochasticOptimization => "stochastic_optimization",
            Self::Verification => "verification",
            Self::Provenance => "provenance",
            Self::Reporting => "reporting",
        }
    }
}

impl core::fmt::Display for OptimizationCapability {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Returns the complete set of architectural capabilities.
///
/// The returned slice is static and allocation-free.
#[must_use]
pub const fn capabilities() -> &'static [OptimizationCapability] {
    &[
        OptimizationCapability::Canonicalization,
        OptimizationCapability::Analysis,
        OptimizationCapability::LocalRewrite,
        OptimizationCapability::AlgebraicRewrite,
        OptimizationCapability::Rewrite,
        OptimizationCapability::EqualitySaturation,
        OptimizationCapability::ParameterOptimization,
        OptimizationCapability::StructuralOptimization,
        OptimizationCapability::Synthesis,
        OptimizationCapability::FaultTolerantOptimization,
        OptimizationCapability::TargetAwareOptimization,
        OptimizationCapability::StochasticOptimization,
        OptimizationCapability::Verification,
        OptimizationCapability::Provenance,
        OptimizationCapability::Reporting,
    ]
}

// -----------------------------------------------------------------------------
// Integration invariants
// -----------------------------------------------------------------------------

/// Returns true when the optimization subsystem is configured to use the
/// canonical Quantum IR rather than a private optimization IR.
///
/// This is intentionally a constant contract.
#[must_use]
pub const fn uses_canonical_quantum_ir() -> bool {
    true
}

/// Returns the canonical path that optimization modules must use for qubit
/// identifiers.
///
/// This is provided as documentation/tooling metadata; it does not introduce
/// an alias or duplicate qubit type.
#[must_use]
pub const fn canonical_qubit_id_path() -> &'static str {
    "crate::quantum::ir::qubit::QubitId"
}

/// Returns the canonical Quantum IR module path.
#[must_use]
pub const fn canonical_ir_path() -> &'static str {
    "crate::quantum::ir"
}

/// Returns the downstream routing module path.
#[must_use]
pub const fn routing_module_path() -> &'static str {
    "crate::quantum::routing"
}

/// Returns the downstream scheduling module path.
#[must_use]
pub const fn scheduling_module_path() -> &'static str {
    "crate::quantum::scheduling"
}

/// Returns the benchmarking consumer module path.
#[must_use]
pub const fn benchmarking_module_path() -> &'static str {
    "crate::quantum::benchmarking"
}

/// Returns the error-correction module path.
#[must_use]
pub const fn error_correction_module_path() -> &'static str {
    "crate::quantum::error_correction"
}

// -----------------------------------------------------------------------------
// Compile-time architecture tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod architecture_tests {
    use super::*;

    #[test]
    fn subsystem_identity_is_stable() {
        assert_eq!(
            OPTIMIZATION_SUBSYSTEM_ID,
            "zamani.quantum.optimization"
        );
    }

    #[test]
    fn architecture_version_is_supported() {
        assert_eq!(OPTIMIZATION_ARCHITECTURE_VERSION, 1);
    }

    #[test]
    fn unsafe_code_is_forbidden() {
        assert!(!UNSAFE_CODE_ALLOWED);
    }

    #[test]
    fn canonical_ir_is_authoritative() {
        assert!(uses_canonical_quantum_ir());
        assert_eq!(
            canonical_ir_path(),
            "crate::quantum::ir"
        );
    }

    #[test]
    fn canonical_qubit_module_is_correct() {
        assert_eq!(
            canonical_qubit_id_path(),
            "crate::quantum::ir::qubit::QubitId"
        );
    }

    #[test]
    fn all_capabilities_have_stable_identifiers() {
        let all = capabilities();

        assert!(!all.is_empty());

        for capability in all {
            assert!(!capability.as_str().is_empty());
        }
    }

    #[test]
    fn capability_identifiers_are_unique() {
        let all = capabilities();

        for (index, capability) in all.iter().enumerate() {
            for other in all.iter().skip(index + 1) {
                assert_ne!(
                    capability.as_str(),
                    other.as_str()
                );
            }
        }
    }

    #[test]
    fn integration_boundaries_are_stable() {
        assert_eq!(
            routing_module_path(),
            "crate::quantum::routing"
        );

        assert_eq!(
            scheduling_module_path(),
            "crate::quantum::scheduling"
        );

        assert_eq!(
            benchmarking_module_path(),
            "crate::quantum::benchmarking"
        );

        assert_eq!(
            error_correction_module_path(),
            "crate::quantum::error_correction"
        );
    }
}