//! Zamani Quantum Optimization — Algebra Subsystem
//!
//! Stable public boundary for the mathematical algebras used by the quantum
//! optimization subsystem.
//!
//! # Architectural role
//!
//! This module is the single entry point for optimization mathematics that
//! operates on Zamani's canonical Quantum IR.
//!
//! The dependency direction is:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                     quantum::frontend
//!                              │
//!                              ▼
//!                        quantum::ir
//!                              │
//!                              ▼
//!                  quantum::optimization
//!                              │
//!                              ▼
//!                optimization::algebra
//!                              │
//!          ┌───────────┬───────┼───────────┐
//!          │           │       │           │
//!          ▼           ▼       ▼           ▼
//!        Pauli      Clifford  Diagonal  PhasePolynomial
//!          │           │       │           │
//!          └───────────┴───────┼───────────┘
//!                              │
//!                              ▼
//!                         optimization
//!                              │
//!                              ▼
//!                           routing
//!                              │
//!                              ▼
//!                         scheduling
//!                              │
//!                              ▼
//!                           hardware
//! ```
//!
//! # Canonical representation rule
//!
//! This module MUST NOT define:
//!
//! - `QuantumGate`;
//! - `QuantumOperation`;
//! - `QuantumCircuit`;
//! - another qubit representation;
//! - another parameter representation;
//! - another IR;
//! - backend-specific gate representations.
//!
//! All circuit and operation semantics originate from:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! The algebra subsystem only provides mathematical representations and
//! transformations that interpret the canonical IR.
//!
//! # Algebra modules
//!
//! The subsystem currently contains four foundational algebras:
//!
//! ## `pauli`
//!
//! Exact Pauli-string and Pauli-product algebra.
//!
//! Used by:
//!
//! - Clifford optimization;
//! - stabilizer reasoning;
//! - Pauli-frame transformations;
//! - phase-polynomial optimization;
//! - Clifford+T optimization;
//! - observable/operator canonicalization;
//! - future Pauli-product synthesis.
//!
//! ## `clifford`
//!
//! Exact Clifford conjugation and tableau-style reasoning.
//!
//! Used by:
//!
//! - Clifford circuit simplification;
//! - stabilizer transformations;
//! - Clifford synthesis;
//! - Clifford+T optimization;
//! - future symplectic optimization.
//!
//! ## `diagonal`
//!
//! Computational-basis diagonal operation analysis.
//!
//! Used by:
//!
//! - diagonal gate reordering;
//! - phase-region extraction;
//! - phase fusion;
//! - phase-polynomial conversion;
//! - diagonal synthesis.
//!
//! ## `phase_polynomial`
//!
//! Exact symbolic phase-polynomial representation and optimization.
//!
//! Used by:
//!
//! - CNOT-dihedral optimization;
//! - CNOT + RZ optimization;
//! - phase-gadget optimization;
//! - T-count reduction;
//! - T-depth reduction;
//! - Clifford+T optimization;
//! - diagonal circuit synthesis.
//!
//! # Stable boundary
//!
//! This file deliberately exposes modules rather than glob-re-exporting all
//! their types.
//!
//! This is important because several mathematical domains legitimately contain
//! types with the same conceptual name. For example, both the Pauli algebra
//! and the Clifford tableau implementation may expose a `Pauli` type, but
//! those types serve different representation contracts.
//!
//! Therefore callers should write:
//!
//! ```text
//! crate::quantum::optimization::algebra::pauli::Pauli
//! crate::quantum::optimization::algebra::clifford::Pauli
//! ```
//!
//! rather than relying on ambiguous wildcard exports.
//!
//! This prevents namespace collisions as the algebra subsystem grows.
//!
//! # Integration contract
//!
//! The following contracts are intentionally established here so downstream
//! files can be implemented without requiring this file to be redesigned.
//!
//! ## `quantum::ir`
//!
//! Algebra modules may consume:
//!
//! ```text
//! crate::quantum::ir::Gate
//! crate::quantum::ir::GateKind
//! crate::quantum::ir::Parameter
//! crate::quantum::ir::QubitId
//! ```
//!
//! They must not replace those types.
//!
//! ## Local optimization
//!
//! `optimization::local` may consume this subsystem for:
//!
//! - Pauli identities;
//! - Clifford identities;
//! - diagonal commutation;
//! - phase simplification.
//!
//! ## Rewrite engine
//!
//! `optimization::rewrite` may use algebraic facts to determine whether a
//! rewrite is semantically valid.
//!
//! The algebra layer does NOT decide whether a rewrite may cross a circuit
//! boundary. That remains the responsibility of the circuit-level rewrite
//! system.
//!
//! ## Synthesis
//!
//! `optimization::synthesis` may consume:
//!
//! - Pauli products;
//! - Clifford tableaux;
//! - diagonal descriptions;
//! - phase polynomials.
//!
//! Algebra modules must not depend on synthesis. This keeps the dependency
//! direction acyclic.
//!
//! ## Fault-tolerant optimization
//!
//! `optimization::fault_tolerant` may consume Pauli, Clifford, and
//! phase-polynomial representations for:
//!
//! - T-count optimization;
//! - T-depth optimization;
//! - Clifford+T normalization;
//! - phase-gadget optimization;
//! - logical resource estimation.
//!
//! The algebra layer must remain independent of fault-tolerant policy.
//!
//! ## Verification
//!
//! `optimization::verification` may use algebraic representations to perform:
//!
//! - exact Clifford equivalence;
//! - Pauli equivalence;
//! - phase-polynomial comparison;
//! - diagonal equivalence checks.
//!
//! Verification must remain the owner of the final semantic decision.
//!
//! # No circular dependencies
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization::algebra
//!      │
//!      ├── local
//!      ├── rewrite
//!      ├── synthesis
//!      ├── fault_tolerant
//!      └── verification
//! ```
//!
//! The algebra subsystem must NOT depend on:
//!
//! - optimization pipeline scheduling;
//! - optimization planners;
//! - optimization pass registry;
//! - routing;
//! - hardware topology;
//! - hardware APIs;
//! - scheduling;
//! - runtime;
//! - QPU execution;
//! - benchmarking orchestration;
//! - frontend parsers;
//! - algorithms such as QAOA/VQE/Grover.
//!
//! This keeps the mathematical layer reusable and independently testable.
//!
//! # Scalability
//!
//! No artificial global qubit-count limit is imposed by this module.
//!
//! Individual algebra implementations are responsible for representing their
//! mathematical structures efficiently and for checking allocation/indexing
//! operations.
//!
//! Resource limits belong to the optimization subsystem's resource-policy
//! layer rather than this mathematical namespace.
//!
//! Therefore the architecture permits:
//!
//! ```text
//! tiny circuit
//!     │
//!     ▼
//! small algebra representation
//! ```
//!
//! and, given sufficient memory/address space:
//!
//! ```text
//! extremely large circuit
//!     │
//!     ▼
//! packed / scalable algebra representation
//! ```
//!
//! No API in this module assumes that a circuit has a practical fixed maximum
//! number of qubits.
//!
//! The phrase "infinity" is necessarily interpreted as "until the available
//! computational resources or explicitly configured compiler limits are
//! exhausted." No software representation can literally support an infinite
//! finite-memory object.
//!
//! # Resource-safety contract
//!
//! Algebra implementations must:
//!
//! - use checked arithmetic where overflow is possible;
//! - validate indices before indexing;
//! - avoid unchecked allocation-size calculations;
//! - preserve symbolic parameters exactly;
//! - avoid silently approximating exact algebra;
//! - avoid constructing exponentially sized matrices unless explicitly
//!   requested by a bounded higher-level operation;
//! - return structured errors on resource exhaustion;
//! - never use `unsafe`.
//!
//! # Numerical semantics
//!
//! Exact algebra and approximate numerical optimization are deliberately
//! separated.
//!
//! This layer should preserve exact symbolic information whenever possible.
//!
//! In particular:
//!
//! ```text
//! θ
//! θ + φ
//! θ - φ
//! 2θ
//! -θ
//! ```
//!
//! must not silently become floating-point approximations merely because an
//! optimization pass is inspecting them.
//!
//! Approximate synthesis or numerical equivalence belongs to explicit
//! higher-level components with an explicit tolerance.
//!
//! # Global phase
//!
//! Algebra modules must not silently discard global phase.
//!
//! A higher-level equivalence policy may explicitly request:
//!
//! ```text
//! exact unitary equivalence
//! ```
//!
//! or:
//!
//! ```text
//! equivalence modulo global phase
//! ```
//!
//! Those are different semantic contracts.
//!
//! The algebra layer therefore preserves phase information whenever its
//! representation requires it.
//!
//! # Rust compatibility
//!
//! This module is intentionally compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.
//!
//! Rust 1.97.1 is specifically supported because it is the requested Zamani
//! compiler baseline.
//!
//! # Safety
//!
//! This module explicitly forbids unsafe Rust.
//!
//! The lint is applied at the algebra module boundary so accidental unsafe
//! additions to this subsystem are rejected by the compiler.
//!
//! # Public API policy
//!
//! The four child modules are the stable algebra namespace.
//!
//! New algebra implementations should be added as independent sibling files
//! and exposed from this boundary exactly once.
//!
//! Existing public APIs in the child modules should not be duplicated here.
//!
//! This prevents `mod.rs` from becoming a second implementation layer.
//!
//! # Future extension
//!
//! The architecture deliberately leaves room for future algebraic modules such
//! as:
//!
//! - `symplectic`;
//! - tensor/network algebra;
//! - stabilizer algebra;
//! - Lie-algebra representations;
//! - operator algebra;
//! - polynomial/ring representations;
//! - approximate unitary invariants;
//! - Pauli-transfer representations;
//! - logical-operator algebra.
//!
//! Such modules should follow the same rules:
//!
//! ```text
//! canonical Quantum IR
//!          │
//!          ▼
//!     algebra module
//!          │
//!          ▼
//! optimization consumer
//! ```
//!
//! They must not introduce a second circuit IR.
//!
//! # Important maintenance rule
//!
//! This file is deliberately complete for the algebra modules currently
//! present in the repository.
//!
//! The four existing modules are declared here directly:
//!
//! ```text
//! pauli
//! clifford
//! diagonal
//! phase_polynomial
//! ```
//!
//! Downstream implementation files can therefore depend on this module's
//! stable namespace without requiring any changes to this file.
//!
//! When a genuinely new algebra family is introduced, its module declaration
//! belongs here as an architectural API change. Existing consumers of the
//! four foundational algebras do not require modification.
//!
//! # Testing contract
//!
//! Unit tests for mathematical algorithms belong inside their respective
//! algebra modules.
//!
//! Cross-algebra tests belong in the optimization integration-test layer.
//!
//! This module intentionally contains only lightweight boundary tests.
//!
//! The purpose is to verify that the expected algebra namespaces are actually
//! part of the compilation unit.
//!
//! # Example
//!
//! ```text
//! use crate::quantum::optimization::algebra;
//!
//! let pauli = algebra::pauli::Pauli::identity(8)?;
//! let _ = pauli;
//! # Ok::<(), algebra::pauli::PauliError>(())
//! ```
//!
//! The exact concrete APIs remain owned by their respective modules.

#![deny(unsafe_code)]

// =============================================================================
// Foundational algebra modules
// =============================================================================

/// Exact Pauli-string and Pauli-product algebra.
///
/// This is the lowest-level operator algebra used by the optimizer.
///
/// It is independent of circuits, routing, scheduling, and hardware.
pub mod pauli;

/// Exact Clifford conjugation and tableau algebra.
///
/// This module uses Pauli mathematics to represent the action of Clifford
/// operations without constructing dense unitary matrices.
pub mod clifford;

/// Computational-basis diagonal-operation algebra.
///
/// This module provides semantic classification and manipulation of diagonal
/// logical operations without owning circuit rewrites.
pub mod diagonal;

/// Exact symbolic phase-polynomial algebra.
///
/// This module represents affine Boolean parity phases and their coefficients
/// using Zamani's canonical parameter representation.
pub mod phase_polynomial;

// =============================================================================
// Stable namespace prelude
// =============================================================================

/// Stable namespace containing the foundational algebra modules.
///
/// The prelude intentionally exports modules rather than wildcarding their
/// contents. This prevents collisions between mathematically distinct types
/// such as the `Pauli` representations used by the Pauli and Clifford
/// subsystems.
///
/// Example:
///
/// ```text
/// use crate::quantum::optimization::algebra::prelude;
///
/// let _ = prelude::pauli;
/// let _ = prelude::clifford;
/// let _ = prelude::diagonal;
/// let _ = prelude::phase_polynomial;
/// ```
pub mod prelude {
    pub use super::{
        clifford,
        diagonal,
        pauli,
        phase_polynomial,
    };
}

// =============================================================================
// Algebra subsystem metadata
// =============================================================================

/// Stable semantic version of the algebra-module boundary.
///
/// This is NOT the overall Zamani compiler version. It identifies the public
/// organization/contract of `optimization::algebra`.
///
/// Incrementing this value is an architectural API decision, not something
/// individual optimization passes should modify.
pub const ALGEBRA_API_VERSION: u32 = 1;

/// Returns the number of foundational algebra modules currently exposed.
///
/// This is intentionally a constant rather than a dynamically generated value
/// so it remains allocation-free and deterministic.
#[must_use]
pub const fn foundational_module_count() -> usize {
    4
}

// =============================================================================
// Compile-time boundary tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foundational_module_count_is_stable() {
        assert_eq!(foundational_module_count(), 4);
    }

    #[test]
    fn algebra_api_version_is_nonzero() {
        assert!(ALGEBRA_API_VERSION > 0);
    }

    #[test]
    fn foundational_modules_are_exposed() {
        let _ = prelude::pauli;
        let _ = prelude::clifford;
        let _ = prelude::diagonal;
        let _ = prelude::phase_polynomial;
    }
}