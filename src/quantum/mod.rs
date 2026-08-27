//! Zamani Quantum Computing Subsystem
//!
//! This module is the authoritative public boundary for quantum computing in
//! Zamani.
//!
//! # Architectural principle
//!
//! `quantum::mod` owns module composition and public namespace boundaries.
//! It does not own quantum semantics, compiler algorithms, backend
//! implementations, benchmarking mathematics, or source-language parsing.
//!
//! The canonical dependency direction is:
//
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                    quantum::frontend
//!                              │
//!                              ▼
//!                       quantum::ir
//!                              │
//!              ┌───────────────┼────────────────┐
//!              │               │                │
//!              ▼               ▼                ▼
//!          algorithms      optimization      analysis
//!              │               │                │
//!              │          routing/scheduling    │
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              ▼
//!                    error correction
//!                              │
//!                              ▼
//!                         hardware
//!                              │
//!                              ▼
//!                           runtime
//! ```
//!
//! Benchmarking is a consumer/orchestration subsystem:
//
//! ```text
//!                     quantum::benchmarking
//!                            │
//!          ┌─────────────────┼─────────────────┐
//!          │                 │                 │
//!          ▼                 ▼                 ▼
//!       quantum::ir     algorithms        error_correction
//!          │                 │                 │
//!          └─────────────────┼─────────────────┘
//!                            │
//!                            ▼
//!                       hardware/runtime
//! ```
//!
//! The canonical Quantum IR must never depend on benchmarking.
//!
//! # Ownership boundaries
//!
//! ## `frontend`
//!
//! Owns:
//!
//! - source-language parsing;
//! - OpenQASM and future quantum-format adapters;
//! - frontend diagnostics;
//! - lowering into the canonical Quantum IR.
//!
//! ## `ir`
//!
//! Owns:
//!
//! - quantum circuit semantics;
//! - quantum operation representation;
//! - qubit/register representation;
//! - measurement representation;
//! - canonical IR invariants.
//!
//! ## `algorithms`
//!
//! Owns:
//!
//! - backend-independent quantum algorithms;
//! - variational algorithms;
//! - algorithmic circuit construction.
//!
//! ## `optimization`
//!
//! Owns:
//!
//! - logical circuit optimization;
//! - cancellation;
//! - peephole optimization;
//! - T-gate reduction;
//! - future optimization passes.
//!
//! ## `routing`
//!
//! Owns:
//!
//! - hardware-aware routing;
//! - transpilation;
//! - logical-to-physical mapping.
//!
//! ## `scheduling`
//!
//! Owns:
//!
//! - execution scheduling;
//! - operation ordering;
//! - scheduling constraints.
//!
//! ## `error_correction`
//!
//! Owns:
//!
//! - quantum error-correction codes;
//! - encoding/decoding;
//! - syndrome processing;
//! - logical-qubit construction;
//! - QEC execution mechanisms.
//!
//! ## `hardware`
//!
//! Owns:
//!
//! - backend abstractions;
//! - hardware capabilities;
//! - calibration;
//! - topology;
//! - backend-specific execution contracts.
//!
//! ## `benchmarking`
//!
//! Owns:
//!
//! - benchmark specifications;
//! - benchmark experiments;
//! - benchmark workload construction;
//! - benchmark circuit generation;
//! - benchmark execution contracts;
//! - normalized observations;
//! - statistical analysis;
//! - benchmark metrics;
//! - Quantum Volume;
//! - randomized benchmarking;
//! - XEB;
//! - cycle/layer benchmarking;
//! - application benchmarks;
//! - volumetric benchmarking;
//! - QEC benchmarking;
//! - hardware/system benchmarking;
//! - reproducibility;
//! - comparison and regression analysis;
//! - reporting.
//!
//! Benchmarking must consume the other quantum subsystems. It must not
//! reimplement their responsibilities.
//!
//! # Benchmarking boundary
//!
//! The authoritative benchmarking namespace is:
//
//! ```text
//! quantum::benchmarking
//! ```
//!
//! The benchmarking directory contains its own `mod.rs`, so it is declared as
//! a normal Rust child module below. The quantum root must not recreate the
//! benchmarking hierarchy inline.
//!
//! This is important because `benchmarking/mod.rs` is responsible for wiring:
//
//! ```text
//! core
//! generators
//! execution
//! statistics
//! metrics
//! protocols
//! volumetric
//! applications
//! qec
//! hardware
//! analysis
//! reporting
//! registry
//! validation
//! tests
//! ```
//!
//! # Benchmarking architecture
//!
//! The intended production flow is:
//
//! ```text
//! Quantum IR / Algorithm / QEC workload
//!                  │
//!                  ▼
//!          Benchmark specification
//!                  │
//!                  ▼
//!            Workload generator
//!                  │
//!                  ▼
//!          Benchmark experiment
//!                  │
//!                  ▼
//!          Execution contract
//!                  │
//!        ┌─────────┴─────────┐
//!        ▼                   ▼
//!     Simulator            Hardware
//!        │                   │
//!        └─────────┬─────────┘
//!                  ▼
//!          Raw observations
//!                  │
//!                  ▼
//!          Statistical engine
//!                  │
//!                  ▼
//!             Metrics
//!                  │
//!                  ▼
//!          BenchmarkResult
//!                  │
//!       ┌──────────┼──────────┐
//!       ▼          ▼          ▼
//!    Reports    Baselines   Regression
//! ```
//!
//! # Quantum Volume compatibility
//!
//! The historical public path:
//
//! ```text
//! quantum::volume_estimator
//! ```
//!
//! remains available through the compatibility re-export below.
//!
//! The authoritative implementation is:
//
//! ```text
//! quantum::benchmarking::volume_estimator
//! ```
//!
//! `volume_estimator` is intentionally a mathematical/statistical component.
//! It does not own circuit generation, execution, routing, scheduling, or
//! backend selection.
//!
//! The long-term Quantum Volume architecture is:
//
//! ```text
//! benchmarking::generators::qv
//!             │
//!             ▼
//! benchmarking::protocols::quantum_volume
//!             │
//!             ▼
//! benchmarking::execution
//!             │
//!             ▼
//! benchmarking::volume_estimator
//! ```
//!
//! # Public API stability
//!
//! New code should prefer explicit subsystem paths:
//
//! ```text
//! quantum::benchmarking
//! quantum::algorithms
//! quantum::error_correction
//! quantum::frontend
//! quantum::hardware
//! quantum::ir
//! quantum::optimization
//! quantum::routing
//! quantum::scheduling
//! ```
//!
//! Historical flat paths are retained only where they already form part of
//! the repository's public compatibility surface.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Safety and dependency invariants
//!
//! This module:
//!
//! - owns no global quantum state;
//! - performs no unsafe operations;
//! - performs no backend I/O;
//! - performs no benchmark execution;
//! - performs no random generation;
//! - performs no statistical calculations;
//! - performs no source parsing.
//!
//! Those operations belong to their owning subsystems.
//!
//! The module also deliberately avoids introducing dependencies merely for
//! namespace composition.
//!
//! # Module declarations
//!
//! The declarations below are intentionally explicit. This makes the quantum
//! subsystem's public architecture discoverable from one file while leaving
//! implementation ownership inside each child module.
//!
//! # Integration contract
//!
//! Every child module must obey these rules:
//
//! 1. The canonical Quantum IR is `quantum::ir`.
//! 2. Frontends lower toward `quantum::ir`.
//! 3. Algorithms construct or consume canonical quantum representations.
//! 4. Optimization operates on canonical representations.
//! 5. Routing operates after logical construction and before backend execution
//!    where required.
//! 6. Scheduling operates according to backend/execution constraints.
//! 7. Hardware provides backend capabilities, topology and calibration.
//! 8. Error correction owns QEC semantics and mechanisms.
//! 9. Benchmarking measures and orchestrates these components.
//! 10. No lower semantic layer may depend upward on benchmarking.
//!
//! # Lifecycle
//!
//! `init_quantum` and `shutdown_quantum` remain compatibility hooks. They do
//! not own global state. Concrete components are initialized and released by
//! their own constructors and ownership boundaries.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Canonical quantum subsystems
// =============================================================================

/// Backend-independent quantum algorithm construction.
///
/// This module owns quantum algorithms and algorithmic circuit construction.
/// Benchmarking may consume these algorithms but must not redefine them.
pub mod algorithms;

/// Canonical hardware-independent Zamani Quantum IR.
///
/// This is the authoritative semantic representation used throughout the
/// quantum subsystem.
pub mod ir;

/// External quantum-language and format frontends.
///
/// Frontends parse external representations and lower them into `quantum::ir`.
pub mod frontend;

/// Quantum error-correction subsystem.
///
/// QEC semantics and mechanisms remain owned here; benchmarking only measures
/// them through its QEC benchmarking adapters.
pub mod error_correction;

// =============================================================================
// Quantum benchmarking
// =============================================================================

/// Production quantum-computing benchmarking framework.
///
/// This module owns the complete benchmarking architecture, including:
///
/// - benchmark specifications;
/// - workload definitions;
/// - experiment orchestration;
/// - deterministic/random circuit generation;
/// - execution contracts;
/// - observations;
/// - statistics;
/// - metrics;
/// - benchmark protocols;
/// - volumetric benchmarks;
/// - application benchmarks;
/// - QEC benchmarks;
/// - hardware benchmark integration;
/// - comparison;
/// - regression detection;
/// - reporting;
/// - reproducibility.
///
/// The child module's own `mod.rs` is authoritative. Do not recreate its
/// children here.
pub mod benchmarking;

// =============================================================================
// Hardware abstraction
// =============================================================================

/// Quantum hardware abstraction layer.
///
/// The current repository stores the hardware implementation as individual
/// files rather than relying on a directory-level `mod.rs`, so the namespace
/// remains explicitly composed here.
pub mod hardware {
    /// Backend capability and execution abstraction.
    #[path = "backend.rs"]
    pub mod backend;

    /// Hardware calibration metadata and validation.
    #[path = "calibration.rs"]
    pub mod calibration;

    /// Hardware connectivity/topology representation.
    #[path = "topology.rs"]
    pub mod topology;
}

// =============================================================================
// Optimization
// =============================================================================

/// Quantum logical-circuit optimization passes.
pub mod optimization {
    /// Cancellation and local simplification.
    #[path = "cancellation.rs"]
    pub mod cancellation;

    /// Peephole optimization.
    #[path = "peephole.rs"]
    pub mod peephole;

    /// T-gate reduction.
    #[path = "t_gate_reduction.rs"]
    pub mod t_gate_reduction;
}

// =============================================================================
// Routing
// =============================================================================

/// Hardware-connectivity-aware quantum routing and transpilation.
pub mod routing {
    /// Quantum circuit transpilation/routing.
    #[path = "transpiler.rs"]
    pub mod transpiler;
}

// =============================================================================
// Scheduling
// =============================================================================

/// Quantum scheduling subsystem.
pub mod scheduling {
    /// Stabilizer-oriented scheduling.
    #[path = "stabilizer_scheduler.rs"]
    pub mod stabilizer_scheduler;
}

// =============================================================================
// Backwards-compatible exports
// =============================================================================

/// Historical flat path for the Quantum Volume estimator.
///
/// New code should prefer:
///
/// `quantum::benchmarking::volume_estimator`
pub use benchmarking::volume_estimator;

/// Historical flat path for T-gate reduction.
///
/// New code should prefer:
///
/// `quantum::optimization::t_gate_reduction`
pub use optimization::t_gate_reduction;

/// Historical flat path for quantum transpilation.
///
/// New code should prefer:
///
/// `quantum::routing::transpiler`
pub use routing::transpiler;

/// Historical flat path for stabilizer scheduling.
///
/// New code should prefer:
///
/// `quantum::scheduling::stabilizer_scheduler`
pub use scheduling::stabilizer_scheduler;

/// Historical flat path for variational algorithms.
///
/// The authoritative implementation remains:
///
/// `quantum::algorithms::variational`
pub use algorithms::variational;

// =============================================================================
// Controlled quantum prelude
// =============================================================================

/// Stable high-level quantum subsystem prelude.
///
/// The prelude exposes subsystem boundaries rather than leaking implementation
/// internals. This keeps callers insulated from internal file organization.
pub mod prelude {
    pub use super::algorithms;
    pub use super::benchmarking;
    pub use super::error_correction;
    pub use super::frontend;
    pub use super::hardware;
    pub use super::ir;
    pub use super::optimization;
    pub use super::routing;
    pub use super::scheduling;
}

// =============================================================================
// Lifecycle compatibility hooks
// =============================================================================

/// Initializes the quantum subsystem boundary.
///
/// The quantum root deliberately owns no process-global state. Concrete
/// quantum components must initialize themselves through their own APIs.
#[inline]
pub fn init_quantum() {}

/// Shuts down the quantum subsystem boundary.
///
/// Ownership and cleanup remain with the concrete quantum components.
#[inline]
pub fn shutdown_quantum() {}

// =============================================================================
// Architectural smoke tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_quantum_boundaries_are_reachable() {
        let _ = std::any::TypeId::of::<frontend::FrontendLimits>();
        let _ = std::any::TypeId::of::<ir::QuantumCircuit>();
    }

    #[test]
    fn benchmarking_boundary_is_reachable() {
        // The existence of the child module verifies that the quantum root
        // uses the authoritative benchmarking/mod.rs rather than recreating
        // benchmarking as an inline module.
        let _ = std::any::TypeId::of::<
            benchmarking::volume_estimator::QuantumVolumeConfig,
        >();
    }

    #[test]
    fn openqasm_frontend_is_reachable_through_canonical_boundary() {
        assert_eq!(frontend::OPENQASM_FORMAT_ID, "openqasm");
        assert_eq!(frontend::OPENQASM_3_0.major(), 3);
        assert_eq!(frontend::OPENQASM_3_0.minor(), 0);
        assert_eq!(frontend::OPENQASM_3_1.major(), 3);
        assert_eq!(frontend::OPENQASM_3_1.minor(), 1);
    }

    #[test]
    fn lifecycle_hooks_are_safe_noops() {
        init_quantum();
        shutdown_quantum();
    }
}