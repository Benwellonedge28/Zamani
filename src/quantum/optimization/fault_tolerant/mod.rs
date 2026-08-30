//! Zamani Quantum Optimization — Fault-Tolerant Optimization
//!
//! This module is the public boundary for fault-tolerant quantum-circuit
//! optimization and resource analysis.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                       quantum::frontend
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                    quantum::optimization
//!                              │
//!                              ▼
//!              optimization::fault_tolerant
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!        T reduction       T-count          T-depth
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              │
//!                              ▼
//!                  fault-tolerant cost model
//!                              │
//!                              ▼
//!                    optimized Quantum IR
//!                              │
//!                              ▼
//!                          routing
//!                              │
//!                              ▼
//!                         scheduling
//!                              │
//!                              ▼
//!                          hardware
//! ```
//!
//! # Purpose
//!
//! The fault-tolerant subsystem is responsible for transformations and
//! analyses whose correctness or usefulness depends on logical/fault-tolerant
//! quantum resources rather than merely ordinary circuit gate count.
//!
//! Examples include:
//!
//! - exact local T/Tdg reduction;
//! - T-count analysis;
//! - T-depth analysis;
//! - Clifford+T resource accounting;
//! - magic-state resource accounting;
//! - future phase-polynomial T optimization;
//! - future Clifford+T synthesis;
//! - future logical resource estimation.
//!
//! # Ownership boundary
//!
//! This module owns the namespace and integration boundary for
//! fault-tolerant optimization.
//!
//! It does NOT own:
//!
//! - the canonical quantum IR;
//! - qubit allocation;
//! - physical routing;
//! - hardware topology;
//! - pulse scheduling;
//! - QPU execution;
//! - measurement-result processing;
//! - error-correction code definitions;
//! - frontend parsing;
//! - quantum algorithms;
//! - benchmarking orchestration;
//! - generic optimization infrastructure.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Canonical representation
//!
//! Every transformation and analysis exposed by this module operates on the
//! canonical Zamani Quantum IR.
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//! ```
//!
//! No fault-tolerant module may introduce a second `QuantumCircuit`,
//! `QuantumGate`, `QuantumOperation`, `QubitId`, or equivalent private circuit
//! representation.
//!
//! This boundary is deliberate: it ensures that frontend lowering,
//! optimization, routing, scheduling, simulation, verification, benchmarking,
//! and hardware compilation all operate on the same semantic representation.
//!
//! # Exactness policy
//!
//! Fault-tolerant transformations are not automatically allowed to relax
//! semantic equivalence.
//!
//! A transformation that is exact must remain exact.
//!
//! A transformation that is valid only up to global phase, approximate
//! equivalence, measurement equivalence, or another relaxed policy must
//! explicitly participate in the optimization subsystem's configured
//! equivalence policy.
//!
//! This module therefore does not provide a blanket "fault tolerant means
//! approximate" policy.
//!
//! # T-family policy
//!
//! The currently implemented T-family subsystem recognizes explicit canonical
//! `GateKind::T` and `GateKind::Tdg` operations.
//!
//! A generic parameterized rotation such as:
//!
//! ```text
//! RZ(theta)
//! ```
//!
//! must NOT automatically be treated as a T gate merely because a particular
//! runtime value of `theta` happens to equal `pi / 4`.
//!
//! Value-sensitive classification belongs to the algebraic/phase-analysis
//! subsystem.
//!
//! This prevents accidental under-counting of non-Clifford resources.
//!
//! # Resource separation
//!
//! Fault-tolerant optimization deliberately keeps different resources
//! independent.
//!
//! In particular:
//!
//! ```text
//! ordinary gate count != T-count != T-depth
//! ```
//!
//! A transformation may improve one metric while worsening another.
//!
//! The common optimization `CostModel` and `OptimizationObjective` determine
//! which resource trade-offs are desirable.
//!
//! This module therefore does not attempt to define one universal meaning of
//! "best circuit."
//!
//! # Current implementation
//!
//! The current production boundary contains four implemented components:
//!
//! ```text
//! fault_tolerant/
//! ├── mod.rs
//! ├── t_gate_reduction.rs
//! ├── t_count.rs
//! ├── t_depth.rs
//! └── magic_state.rs
//! ```
//!
//! Their responsibilities are intentionally distinct:
//!
//! ```text
//! t_gate_reduction.rs
//!     exact circuit transformation
//!
//! t_count.rs
//!     exact structural T-resource accounting
//!
//! t_depth.rs
//!     exact/conservative T-layer depth analysis
//!
//! magic_state.rs
//!     fault-tolerant magic-state resource modeling
//! ```
//!
//! Future modules must follow the same separation.
//!
//! # Stable module identifiers
//!
//! Public module names are part of the compiler's API surface.
//!
//! Existing consumers should be able to refer to:
//!
//! ```text
//! quantum::optimization::fault_tolerant::t_gate_reduction
//! quantum::optimization::fault_tolerant::t_count
//! quantum::optimization::fault_tolerant::t_depth
//! quantum::optimization::fault_tolerant::magic_state
//! ```
//!
//! Future implementation modules should be added here only when their source
//! files exist and their contracts are independently complete.
//!
//! # Integration with generic optimization
//!
//! The dependency direction is:
//!
//! ```text
//! fault_tolerant
//!      │
//!      ├──► optimization::pass
//!      ├──► optimization::context
//!      ├──► optimization::errors
//!      ├──► optimization::analysis
//!      ├──► optimization::cost
//!      ├──► optimization::validation
//!      └──► quantum::ir
//! ```
//!
//! The fault-tolerant subsystem must NOT introduce a dependency in the
//! opposite direction merely to expose its own implementation details.
//!
//! Generic optimization infrastructure owns:
//!
//! - pass lifecycle;
//! - configuration;
//! - limits;
//! - pipeline execution;
//! - analysis caching;
//! - provenance;
//! - verification policy;
//! - cost comparison;
//! - planner/scheduler behavior.
//!
//! Fault-tolerant modules provide the domain-specific implementation.
//!
//! # Integration with the pass registry
//!
//! The T-gate reduction pass has the stable identifier:
//!
//! ```text
//! fault_tolerant.t_gate_reduction
//! ```
//!
//! Registration belongs to `optimization::registry`.
//!
//! This module exposes the implementation module and stable pass type; it does
//! not create a second registry.
//!
//! A future fault-tolerant transformation should similarly expose its stable
//! implementation through this namespace while registration remains owned by
//! the generic registry.
//!
//! # Integration with the planner
//!
//! `optimization::planner` may select fault-tolerant passes according to:
//!
//! - optimization profile;
//! - target capabilities;
//! - requested objective;
//! - circuit characteristics;
//! - T/non-Clifford resource density;
//! - resource limits;
//! - verification policy.
//!
//! The planner should not depend on private implementation details of a
//! particular pass.
//!
//! # Integration with T-count
//!
//! `t_count` is an analysis/resource-accounting subsystem.
//!
//! It does not rewrite the circuit.
//!
//! A transformation pass may consume its results before and after a rewrite,
//! but the analysis remains independent of the transformation.
//!
//! This distinction prevents code such as:
//!
//! ```text
//! "T-count changed, therefore the rewrite must be correct"
//! ```
//!
//! from becoming an invalid correctness assumption.
//!
//! Correctness is established by the generic verification subsystem.
//!
//! # Integration with T-depth
//!
//! T-depth remains independent from T-count.
//!
//! For example, two T gates on independent qubits may contribute:
//!
//! ```text
//! T-count = 2
//! T-depth = 1
//! ```
//!
//! Therefore neither metric may be implemented as a trivial derivation of the
//! other.
//!
//! T-depth analysis belongs to `t_depth.rs`.
//!
//! # Integration with magic-state resources
//!
//! Magic-state analysis is a resource model, not a circuit transformation.
//!
//! It may consume T-family analysis and fault-tolerant configuration, but it
//! must not directly invoke hardware APIs or QPU execution.
//!
//! Hardware-specific distillation factories, physical error rates, code
//! distances, and scheduling remain outside this module unless they are
//! supplied through an explicit target/resource-model interface.
//!
//! # Integration with phase-polynomial optimization
//!
//! Future global T-count optimization belongs in the algebraic optimization
//! layer, especially:
//!
//! ```text
//! optimization::algebra::phase_polynomial
//! ```
//!
//! The fault-tolerant subsystem may consume and expose fault-tolerant resource
//! metrics for that optimization, but this module should not duplicate the
//! phase-polynomial representation.
//!
//! The intended direction is:
//!
//! ```text
//! canonical IR
//!      │
//!      ├──────────────► T-count analysis
//!      │
//!      └──────────────► phase-polynomial optimization
//!                              │
//!                              ▼
//!                       canonical IR
//! ```
//!
//! This avoids a dependency cycle between resource accounting and algebraic
//! transformation.
//!
//! # Integration with error correction
//!
//! Fault-tolerant optimization is NOT quantum error correction.
//!
//! The optimizer may calculate logical resource requirements such as T-count,
//! T-depth, or magic-state demand, but code construction, syndrome extraction,
//! decoding, lattice surgery, surface codes, repetition codes, LDPC codes,
//! color codes, and other QEC semantics belong to `quantum::error_correction`
//! and related subsystems.
//!
//! QEC-aware cost information may be supplied to optimization through generic
//! cost/target interfaces.
//!
//! # Integration with routing
//!
//! Fault-tolerant optimization normally operates before physical routing:
//!
//! ```text
//! logical circuit
//!      │
//!      ▼
//! fault-tolerant optimization
//!      │
//!      ▼
//! logical optimized circuit
//!      │
//!      ▼
//! routing
//! ```
//!
//! However, target-aware optimization may consume abstract routing/cost
//! information through the generic target layer.
//!
//! This module must never directly implement a routing algorithm.
//!
//! # Integration with scheduling
//!
//! T-depth is a logical resource metric.
//!
//! Physical execution scheduling is a separate responsibility.
//!
//! Therefore:
//!
//! ```text
//! t_depth.rs
//!     │
//!     └──► logical T-depth information
//!
//! scheduling
//!     │
//!     └──► physical execution timing
//! ```
//!
//! They must not be conflated.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may consume:
//!
//! - T-count;
//! - T-depth;
//! - T-family count;
//! - magic-state resource estimates;
//! - before/after optimization statistics.
//!
//! This module does not depend on benchmarking.
//!
//! The dependency direction remains:
//!
//! ```text
//! benchmarking ─────► optimization/fault_tolerant
//! ```
//!
//! rather than:
//!
//! ```text
//! optimization/fault_tolerant ─────► benchmarking
//! ```
//!
//! # Scaling
//!
//! This module imposes no artificial circuit-size ceiling.
//!
//! The practical scalability boundary is determined by:
//!
//! - canonical Quantum IR limits;
//! - generic optimization limits;
//! - pass-specific resource limits;
//! - available memory;
//! - available CPU time;
//! - configured cancellation/deadline policy;
//! - host allocation limits.
//!
//! This is important for Zamani's requirement that the optimizer scale from
//! tiny circuits to the largest circuits the available resources can
//! represent.
//!
//! "Infinity" therefore means:
//!
//! ```text
//! no artificial algorithmic ceiling in the module boundary
//! ```
//!
//! not:
//!
//! ```text
//! impossible mathematical infinite memory/time
//! ```
//!
//! Individual passes remain responsible for declaring their complexity and
//! cooperating with `OptimizationContext` resource limits.
//!
//! # Determinism
//!
//! The currently exposed fault-tolerant components are deterministic for a
//! fixed canonical circuit and fixed optimizer configuration.
//!
//! If a future fault-tolerant pass introduces randomized search, it must use
//! the generic optimization determinism/seed infrastructure and declare its
//! determinism characteristics through `PassMetadata`.
//!
//! No module-level mutable state is permitted.
//!
//! # Thread safety
//!
//! This module contains no global mutable state and does not spawn worker
//! threads.
//!
//! Parallel execution belongs to the generic optimization scheduler.
//!
//! This permits the same fault-tolerant pass implementation to participate in
//! sequential or parallel compiler pipelines without changing this module.
//!
//! # Transactional behavior
//!
//! Transformation modules are responsible for ensuring that failed
//! transformations do not leave the canonical circuit partially modified.
//!
//! `mod.rs` deliberately does not provide a second mutation mechanism.
//!
//! # Error policy
//!
//! Domain-specific analysis errors may remain domain-specific inside an
//! analysis module where that is useful for detailed diagnostics.
//!
//! Transformation passes integrated with `OptimizationPass` must expose
//! failures through the generic optimization error contract.
//!
//! This module must not introduce another global optimizer error type.
//!
//! # Public API stability
//!
//! Re-export only deliberately stable, high-value APIs.
//!
//! Implementation details remain accessible through their named modules when
//! necessary, but users should normally consume fault-tolerant functionality
//! through the generic optimization API.
//!
//! # Rust compatibility
//!
//! This module is intentionally compatible with:
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
//! No unsafe code is permitted in this subsystem boundary.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! The child modules independently enforce the same rule.

#![forbid(unsafe_code)]

// =============================================================================
// Implemented fault-tolerant modules
// =============================================================================

/// Exact local Clifford+T T/Tdg power reduction.
///
/// Stable pass identifier:
///
/// `fault_tolerant.t_gate_reduction`
pub mod t_gate_reduction;

/// Exact structural T-resource accounting.
///
/// This module measures T, Tdg, and related T-family resources without
/// modifying the circuit.
pub mod t_count;

/// Exact/conservative logical T-depth analysis.
///
/// T-depth is deliberately independent from T-count.
pub mod t_depth;

/// Fault-tolerant magic-state resource modeling.
///
/// This module models resource requirements and does not execute QPU work.
pub mod magic_state;

// =============================================================================
// Stable public re-exports
// =============================================================================

/// Production T-gate reduction pass.
///
/// This is the primary fault-tolerant transformation currently exposed by the
/// subsystem.
pub use t_gate_reduction::TGateReductionPass;

/// Stable identifier for the T-gate reduction transformation.
///
/// This alias keeps the subsystem's public API convenient without duplicating
/// the identifier value in multiple implementation files.
pub const T_GATE_REDUCTION_PASS_ID: &str =
    t_gate_reduction::PASS_ID;

// =============================================================================
// Subsystem metadata
// =============================================================================

/// Stable identifier for the fault-tolerant optimization subsystem.
pub const SUBSYSTEM_ID: &str = "quantum.optimization.fault_tolerant";

/// Public API/schema version for this module boundary.
///
/// This version changes only when the public module contract or stable
/// subsystem-level semantics change. Individual transformation versions are
/// maintained by their own modules.
pub const SUBSYSTEM_SCHEMA_VERSION: u32 = 1;

/// Stable name of the subsystem.
pub const SUBSYSTEM_NAME: &str = "Fault-Tolerant Quantum Optimization";

/// Returns the stable subsystem identifier.
///
/// This function is intentionally allocation-free and suitable for diagnostics,
/// provenance, registry inspection, and compiler introspection.
#[must_use]
pub const fn subsystem_id() -> &'static str {
    SUBSYSTEM_ID
}

/// Returns the stable subsystem schema version.
#[must_use]
pub const fn subsystem_schema_version() -> u32 {
    SUBSYSTEM_SCHEMA_VERSION
}

/// Returns the stable human-readable subsystem name.
#[must_use]
pub const fn subsystem_name() -> &'static str {
    SUBSYSTEM_NAME
}

// =============================================================================
// Capability description
// =============================================================================

/// Stable capability identifiers exposed by the currently implemented
/// fault-tolerant subsystem.
///
/// These values are intentionally strings rather than a public Rust enum so
/// future capabilities can be added without making every downstream consumer
/// recompile against a new exhaustive enum variant.
pub mod capabilities {
    /// Exact local T/Tdg power reduction.
    pub const T_GATE_REDUCTION: &str =
        "fault_tolerant.t_gate_reduction";

    /// Structural T-count analysis.
    pub const T_COUNT_ANALYSIS: &str =
        "fault_tolerant.t_count_analysis";

    /// Logical T-depth analysis.
    pub const T_DEPTH_ANALYSIS: &str =
        "fault_tolerant.t_depth_analysis";

    /// Magic-state resource analysis.
    pub const MAGIC_STATE_ANALYSIS: &str =
        "fault_tolerant.magic_state_analysis";

    /// Returns the currently implemented capability identifiers.
    ///
    /// The returned slice is static and allocation-free.
    #[must_use]
    pub const fn implemented() -> &'static [&'static str] {
        &[
            T_GATE_REDUCTION,
            T_COUNT_ANALYSIS,
            T_DEPTH_ANALYSIS,
            MAGIC_STATE_ANALYSIS,
        ]
    }
}

// =============================================================================
// Architectural invariants
// =============================================================================

/// Stable architectural invariants for compiler introspection and tests.
///
/// These are compile-time constants rather than runtime configuration.
pub mod invariants {
    /// Fault-tolerant optimization operates on canonical Quantum IR.
    pub const CANONICAL_IR_ONLY: bool = true;

    /// No unsafe Rust is permitted.
    pub const UNSAFE_CODE_FORBIDDEN: bool = true;

    /// Hardware APIs are outside this subsystem.
    pub const NO_HARDWARE_IO: bool = true;

    /// Routing is outside this subsystem.
    pub const NO_ROUTING_OWNERSHIP: bool = true;

    /// Physical scheduling is outside this subsystem.
    pub const NO_PHYSICAL_SCHEDULING: bool = true;

    /// QEC code implementation is outside this subsystem.
    pub const NO_QEC_CODE_OWNERSHIP: bool = true;

    /// Benchmarking is a consumer rather than a dependency.
    pub const NO_BENCHMARKING_DEPENDENCY: bool = true;

    /// No artificial circuit-size ceiling is imposed by this module boundary.
    pub const NO_ARTIFICIAL_CIRCUIT_SIZE_LIMIT: bool = true;

    /// Current fault-tolerant components are deterministic.
    pub const CURRENT_COMPONENTS_DETERMINISTIC: bool = true;

    /// Global mutable state is not part of the subsystem architecture.
    pub const NO_GLOBAL_MUTABLE_STATE: bool = true;
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsystem_identity_is_stable() {
        assert_eq!(
            subsystem_id(),
            "quantum.optimization.fault_tolerant"
        );

        assert_eq!(
            subsystem_name(),
            "Fault-Tolerant Quantum Optimization"
        );

        assert_eq!(subsystem_schema_version(), 1);
    }

    #[test]
    fn implemented_modules_are_exposed() {
        assert_eq!(
            T_GATE_REDUCTION_PASS_ID,
            "fault_tolerant.t_gate_reduction"
        );

        assert!(
            capabilities::implemented()
                .contains(&capabilities::T_GATE_REDUCTION)
        );

        assert!(
            capabilities::implemented()
                .contains(&capabilities::T_COUNT_ANALYSIS)
        );

        assert!(
            capabilities::implemented()
                .contains(&capabilities::T_DEPTH_ANALYSIS)
        );

        assert!(
            capabilities::implemented()
                .contains(&capabilities::MAGIC_STATE_ANALYSIS)
        );
    }

    #[test]
    fn architectural_invariants_are_enabled() {
        assert!(invariants::CANONICAL_IR_ONLY);
        assert!(invariants::UNSAFE_CODE_FORBIDDEN);
        assert!(invariants::NO_HARDWARE_IO);
        assert!(invariants::NO_ROUTING_OWNERSHIP);
        assert!(invariants::NO_PHYSICAL_SCHEDULING);
        assert!(invariants::NO_QEC_CODE_OWNERSHIP);
        assert!(invariants::NO_BENCHMARKING_DEPENDENCY);
        assert!(invariants::NO_ARTIFICIAL_CIRCUIT_SIZE_LIMIT);
        assert!(invariants::CURRENT_COMPONENTS_DETERMINISTIC);
        assert!(invariants::NO_GLOBAL_MUTABLE_STATE);
    }
}