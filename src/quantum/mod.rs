//! Zamani Quantum subsystem root.
//!
//! This module is the public boundary for all quantum functionality in Zamani.
//! It owns module wiring only; quantum semantics remain owned by the canonical
//! [`ir`] module and external-language parsing remains owned by [`frontend`].
//!
//! # Architectural dependency direction
//!
//! ```text
//! external quantum formats
//!          │
//!          ▼
//!     quantum::frontend
//!          │
//!          ▼
//!      quantum::ir
//!          │
//!     ┌────┴───────────────┐
//!     ▼                    ▼
//! algorithms          compiler-facing
//!     │                    │
//!     └──────────┬─────────┘
//!                ▼
//!       optimization / routing / scheduling
//!                │
//!                ▼
//!       error correction / hardware
//! ```
//!
//! The root must not become a second ownership layer.
//!
//! - `frontend` owns source-language parsing, diagnostics, format adapters,
//!   and lowering boundaries.
//! - `ir` owns canonical logical quantum semantics and IR invariants.
//! - `algorithms` owns backend-independent algorithm construction.
//! - `optimization`, `routing`, and `scheduling` own downstream transformations.
//! - `error_correction` owns error-correction functionality.
//! - `hardware` owns backend/topology/calibration abstractions.
//!
//! A concrete frontend format must never depend on another concrete format.
//! OpenQASM and future formats such as QIR or Quil lower independently toward
//! the canonical Zamani Quantum IR.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1, Rust 2021.
//! No nightly features are required by this module.
//!
//! # Lifecycle
//!
//! The quantum root owns no process-global state. The lifecycle functions are
//! retained as compatibility hooks and deliberately perform no global mutation.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Canonical semantic layers
// =============================================================================

/// Backend-independent quantum algorithm construction.
pub mod algorithms;

/// Canonical hardware-independent Zamani Quantum IR.
///
/// This is the semantic boundary consumed by frontends and downstream
/// quantum compiler stages.
pub mod ir;

/// External quantum-language and format frontends.
///
/// OpenQASM is the first production format. Additional formats such as QIR
/// and Quil belong under `frontend::formats` and remain independently
/// removable.
pub mod frontend;

/// Quantum error-correction subsystem.
pub mod error_correction;

// =============================================================================
// Subsystems stored without directory-level mod.rs
// =============================================================================

/// Quantum circuit benchmarking and resource estimation.
pub mod benchmarking {
    /// Quantum-volume/resource-estimation implementation.
    #[path = "volume_estimator.rs"]
    pub mod volume_estimator;
}

/// Quantum hardware abstractions.
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

/// Downstream logical-circuit optimization passes.
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

/// Hardware-connectivity-aware routing/transpilation.
pub mod routing {
    /// Quantum circuit transpilation/routing.
    #[path = "transpiler.rs"]
    pub mod transpiler;
}

/// Quantum scheduling passes.
pub mod scheduling {
    /// Stabilizer-oriented scheduling.
    #[path = "stabilizer_scheduler.rs"]
    pub mod stabilizer_scheduler;
}

// =============================================================================
// Backwards-compatible paths
// =============================================================================

/// Historical flat path for the volume estimator.
pub use benchmarking::volume_estimator;

/// Historical flat path for T-gate reduction.
pub use optimization::t_gate_reduction;

/// Historical flat path for transpilation.
pub use routing::transpiler;

/// Historical flat path for stabilizer scheduling.
pub use scheduling::stabilizer_scheduler;

/// Historical flat path for variational algorithms.
///
/// The authoritative implementation is now owned by
/// `quantum::algorithms::variational`.
pub use algorithms::variational;

// =============================================================================
// Controlled public prelude
// =============================================================================

/// Stable quantum-root prelude.
///
/// Only subsystem boundaries are promoted here. Concrete parser internals,
/// ASTs, lexer implementation details, and hardware implementation details
/// remain under their owning modules.
pub mod prelude {
    pub use super::algorithms;
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
/// No global state is created here. Concrete quantum components own their
/// initialization through their constructors/configuration APIs.
#[inline]
pub fn init_quantum() {}

/// Shuts down the quantum subsystem boundary.
///
/// Resource ownership belongs to the concrete component that acquired it.
#[inline]
pub fn shutdown_quantum() {}

// =============================================================================
// Architectural smoke tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_boundaries_are_reachable() {
        let _ = std::any::TypeId::of::<frontend::FrontendLimits>();
        let _ = std::any::TypeId::of::<ir::QuantumCircuit>();
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