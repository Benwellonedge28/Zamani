//! Zamani Quantum IR — Validation Subsystem
//!
//! Production-grade validation boundary for the canonical Zamani Quantum IR.
//!
//! # Architectural role
//!
//! This module is the public validation boundary of:
//!
//! ```text
//! quantum::ir::validation
//! ```
//!
//! It coordinates independent validation domains without becoming coupled to
//! quantum hardware, routing, scheduling, optimization, simulation, QEC,
//! frontend parsing, or backend execution.
//!
//! The validation subsystem answers:
//!
//! > Is this canonical Zamani IR structurally, semantically, type-wise,
//! > control-flow-wise, resource-wise, and diagnostically valid under the
//! > supplied validation policy?
//!
//! It does NOT answer:
//!
//! - whether a QPU physically exists;
//! - whether a physical topology can realize the program;
//! - whether routing is possible;
//! - whether a target supports an operation;
//! - which native operation should be selected;
//! - which calibration should be used;
//! - how pulses should be synthesized;
//! - how operations should be scheduled;
//! - how a backend communicates with hardware;
//! - how a simulator executes quantum state;
//! - how QEC is decoded.
//!
//! Those responsibilities remain outside the canonical IR validation layer.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! canonical quantum::ir
//!      │
//!      ▼
//! ┌─────────────────────────────────────┐
//! │ quantum::ir::validation              │
//! │                                     │
//! │  policy                             │
//! │  namespace                          │
//! │  structural                         │
//! │  typing                             │
//! │  semantic                           │
//! │  control flow                       │
//! │  resources                           │
//! │  diagnostics                        │
//! └──────────────────┬──────────────────┘
//!                    │
//!                    ▼
//!             valid canonical IR
//!                    │
//!          ┌─────────┼─────────┐
//!          ▼         ▼         ▼
//!     optimization routing scheduling
//!          │         │         │
//!          └─────────┼─────────┘
//!                    ▼
//!                 hardware
//!                    │
//!                    ▼
//!                 backend
//! ```
//!
//! # Universal-program principle
//!
//! Validation MUST NOT impose an architectural quantum-machine size.
//!
//! In particular, this module must never encode assumptions such as:
//!
//! ```text
//! 32 qubits
//! 64 qubits
//! 128 qubits
//! 4096 qubits
//! ```
//!
//! as language-level limits.
//!
//! A program may describe:
//!
//! ```text
//! 1 qubit
//! 2 qubits
//! N qubits
//! extremely sparse logical namespaces
//! distributed logical resources
//! future quantum architectures
//! ```
//!
//! Concrete limits belong to `QuantumIrLimits` and are explicitly scoped to
//! one validation/compilation/execution context.
//!
//! Therefore:
//!
//! ```text
//! architectural capability != validation policy
//! ```
//!
//! `QuantumIrLimits` answers:
//!
//! > How much work may this particular invocation safely process?
//!
//! It does NOT answer:
//!
//! > What is the maximum quantum computer Zamani can represent?
//!
//! # Scalability
//!
//! Validation must scale according to the resources actually represented and
//! touched by the IR rather than allocating structures proportional to a
//! hypothetical physical machine size.
//!
//! Validation implementations therefore prefer:
//!
//! - sparse resource sets;
//! - canonical IDs;
//! - checked arithmetic;
//! - bounded validation work;
//! - deterministic traversal;
//! - explicit policies;
//! - streaming-friendly validation where possible;
//! - no fixed-size machine bitsets;
//! - no architecture-specific arrays;
//! - no unsafe memory operations.
//!
//! # Trust boundary
//!
//! Validation MUST be performed again when IR crosses a trust boundary,
//! regardless of whether an earlier constructor already validated it.
//!
//! Possible sources include:
//!
//! - Zamani frontend lowering;
//! - deserialization;
//! - generated IR;
//! - optimization passes;
//! - transformation passes;
//! - caches;
//! - replay;
//! - distributed compilation;
//! - external tooling;
//! - future dialects;
//! - plugins.
//!
//! The validator therefore treats canonical IR as potentially untrusted data.
//!
//! # Validation layers
//!
//! The complete validation subsystem is intentionally decomposed:
//!
//! ```text
//! ValidationConfig
//!       │
//!       ▼
//! policy validation
//!       │
//!       ▼
//! structural validation
//!       │
//!       ├── namespaces
//!       ├── identities
//!       ├── operands
//!       ├── operation structure
//!       └── references
//!       │
//!       ▼
//! typing validation
//!       │
//!       ▼
//! semantic validation
//!       │
//!       ├── measurements
//!       ├── classical semantics
//!       ├── quantum semantics
//!       └── cross-object invariants
//!       │
//!       ▼
//! control-flow validation
//!       │
//!       ├── blocks
//!       ├── branches
//!       ├── successors
//!       ├── loops
//!       └── feedback
//!       │
//!       ▼
//! resource validation
//!       │
//!       ├── qubits
//!       ├── classical resources
//!       ├── operands
//!       ├── operations
//!       ├── metadata
//!       └── validation work
//!       │
//!       ▼
//! diagnostics
//! ```
//!
//! # Canonical qubit identity
//!
//! All validation code MUST use the authoritative qubit module:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! There must be no second qubit identity definition inside validation.
//!
//! `qubits` remains only a compatibility alias at the parent IR boundary.
//! New validation code must use `quantum::ir::qubit`.
//!
//! # Module ownership
//!
//! ```text
//! validation/mod.rs
//!     Public validation subsystem boundary and orchestration API.
//!
//! validation/validation.rs
//!     Canonical validation engine and validation configuration.
//!
//! validation/structural.rs
//!     Representation/reference/namespace structure.
//!
//! validation/typing.rs
//!     Canonical IR type correctness.
//!
//! validation/semantic.rs
//!     Quantum/classical semantic invariants.
//!
//! validation/control_flow.rs
//!     Blocks, successors, branches, loops and dynamic control flow.
//!
//! validation/resources.rs
//!     Explicit resource/security policy accounting.
//!
//! validation/diagnostics.rs
//!     Structured validation diagnostics and reporting.
//! ```
//!
//! Each implementation module owns its domain. This module must not duplicate
//! those implementations.
//!
//! # Dependency rule
//!
//! Validation may depend on canonical IR definitions:
//!
//! ```text
//! validation
//!     ├── core IR types
//!     ├── qubit
//!     ├── gate
//!     ├── operation
//!     ├── program
//!     ├── classical
//!     ├── control flow
//!     ├── resources
//!     └── limits
//! ```
//!
//! Validation MUST NOT depend on:
//!
//! ```text
//! hardware
//! routing
//! scheduling
//! optimization
//! simulator
//! QEC implementation
//! backend execution
//! frontend parser
//! ```
//!
//! Downstream components may depend on validation.
//!
//! The dependency direction must never be reversed.
//!
//! # Determinism
//!
//! Validation is required to be deterministic for identical:
//!
//! - canonical IR;
//! - validation configuration;
//! - validation limits;
//! - validator version.
//!
//! Validation must not depend on:
//!
//! - hash-map iteration order;
//! - memory addresses;
//! - wall-clock timing;
//! - thread scheduling;
//! - random state;
//! - backend state.
//!
//! # Error model
//!
//! The validation subsystem must expose the canonical IR error boundary rather
//! than introducing an unrelated error hierarchy for every validation phase.
//!
//! Detailed phase-specific information belongs to the corresponding validation
//! module and/or diagnostic representation.
//!
//! # Compatibility
//!
//! This module deliberately preserves the existing high-level validation API
//! while introducing a directory-based subsystem.
//!
//! Existing consumers should continue to be able to use:
//!
//! ```text
//! quantum::ir::validation::validate_circuit
//! quantum::ir::validation::validate_circuit_with_limits
//! quantum::ir::validation::validate_circuit_with_config
//! quantum::ir::validation::validate_operation
//! quantum::ir::validation::validate_gate
//! quantum::ir::validation::validate_limits
//! ```
//!
//! New code may use the more explicit phase boundaries:
//!
//! ```text
//! quantum::ir::validation::structural
//! quantum::ir::validation::typing
//! quantum::ir::validation::semantic
//! quantum::ir::validation::control_flow
//! quantum::ir::validation::resources
//! quantum::ir::validation::diagnostics
//! ```
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
//! - no unsafe.
//!
//! `forbid(unsafe_code)` is deliberately applied here as a compiler-enforced
//! architectural guarantee.
//!
//! # Integration contract
//!
//! This file owns only:
//!
//! 1. module declarations;
//! 2. public API re-exports;
//! 3. subsystem-level documentation;
//! 4. cross-module validation tests that belong at the validation boundary.
//!
//! It does NOT own:
//!
//! - individual validation algorithms;
//! - gate semantics;
//! - qubit identity;
//! - resource policy definitions;
//! - type definitions;
//! - serialization;
//! - hashing;
//! - hardware compatibility.
//!
//! This makes the file stable even as individual validation algorithms evolve.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Validation implementation modules
// =============================================================================
//
// Keep these declarations explicit. Do not use glob-based module discovery.
// This makes the validation subsystem's compilation surface deterministic and
// auditable.

/// Dynamic control-flow validation.
///
/// Validates blocks, successors, branches, loops, guards and classical
/// feedback relationships that are represented in the canonical IR.
pub mod control_flow;

/// Structured diagnostics and validation reporting.
///
/// This module owns diagnostic objects rather than making the validation
/// engine print or log directly.
pub mod diagnostics;

/// Resource and security-policy validation.
///
/// This module is responsible for applying explicit `QuantumIrLimits` and
/// related resource accounting without turning those limits into language
/// architecture limits.
pub mod resources;

/// Quantum/classical semantic validation.
///
/// This validates meaning-preserving invariants that cannot be established by
/// structural validation alone.
pub mod semantic;

/// Structural/reference validation.
///
/// This validates namespaces, identities, references, operation structure and
/// other representation-level invariants.
pub mod structural;

/// Canonical IR typing validation.
///
/// This validates that operations, operands, results, parameters and values
/// agree with the canonical IR type system.
pub mod typing;

/// Canonical validation engine.
///
/// This module contains the stable high-level validation configuration and
/// compatibility entry points.
pub mod validation;

// =============================================================================
// Canonical high-level validation API
// =============================================================================
//
// Re-export the established validation engine API at this module boundary.
// This preserves:
//
//     quantum::ir::validation::validate_circuit(...)
//
// instead of requiring callers to know that the implementation happens to
// reside in validation/validation.rs.

pub use self::validation::{
    validate_circuit,
    validate_circuit_with_config,
    validate_circuit_with_limits,
    validate_gate,
    validate_limits,
    validate_operation,
    ValidationConfig,
};

// =============================================================================
// Phase-level API
// =============================================================================
//
// These aliases deliberately expose the implementation modules themselves,
// rather than duplicating their types or functions here.
//
// Consumers that need a specific validation phase should use:
//
//     quantum::ir::validation::structural
//     quantum::ir::validation::typing
//     quantum::ir::validation::semantic
//     quantum::ir::validation::control_flow
//     quantum::ir::validation::resources
//     quantum::ir::validation::diagnostics
//
// This preserves ownership boundaries.

// =============================================================================
// Compatibility contract
// =============================================================================

/// Validates the validation subsystem's compile-time integration contract.
///
/// This function intentionally performs no runtime work. It exists as a
/// lightweight integration point for tests, startup self-checks and future
/// validation registry work.
///
/// The important guarantee is that all validation domains are present in the
/// same canonical subsystem and can be referenced through their stable module
/// paths.
///
/// This function is intentionally infallible because module availability is a
/// compile-time property.
#[inline]
pub const fn integration_contract() {
    // The function is intentionally empty.
    //
    // Presence of the modules and public APIs is checked by Rust itself when
    // this module is compiled. Keeping this function const and side-effect
    // free makes it safe for tests and compile-time-oriented callers.
}

// =============================================================================
// Cross-module tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_subsystem_has_all_domains() {
        // Referencing each module here ensures the public module topology
        // remains part of the compile-time integration contract.
        let _ = control_flow::integration_contract;
        let _ = diagnostics::integration_contract;
        let _ = resources::integration_contract;
        let _ = semantic::integration_contract;
        let _ = structural::integration_contract;
        let _ = typing::integration_contract;
        let _ = validation::validate_limits;

        integration_contract();
    }

    #[test]
    fn canonical_validation_api_is_reexported() {
        // These references intentionally verify the compatibility API without
        // constructing a complete quantum program.
        let _ = validate_circuit;
        let _ = validate_circuit_with_config;
        let _ = validate_circuit_with_limits;
        let _ = validate_gate;
        let _ = validate_limits;
        let _ = validate_operation;
        let _ = ValidationConfig::production;
    }
}