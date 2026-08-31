//! Zamani Quantum IR — Structured Control Namespace.
//!
//! This module is the public namespace boundary for structured control flow
//! in the canonical Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `quantum::ir::control` groups the semantic control-flow facilities used by
//! quantum/classical programs:
//!
//! - conditions and predicates;
//! - conditional branches;
//! - loops and iteration domains;
//! - structured control-flow nodes;
//! - measurement-driven classical feedback;
//! - structured transfers such as `break`, `continue`, and `return`.
//!
//! The namespace is deliberately thin. It does not implement control-flow
//! semantics itself. Semantic ownership belongs to the child modules:
//!
//! ```text
//! control/
//! ├── mod.rs
//! ├── branch.rs
//! ├── condition.rs
//! ├── control_flow.rs
//! └── loop.rs
//! ```
//!
//! # Canonical dependency boundary
//!
//! The intended dependency direction is:
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                           frontend
//!                              │
//!                              ▼
//!                     ┌─────────────────┐
//!                     │ quantum::ir     │
//!                     │                 │
//!                     │ semantic WHAT   │
//!                     └────────┬────────┘
//!                              │
//!                 ┌────────────┼────────────┐
//!                 │            │            │
//!                 ▼            ▼            ▼
//!             condition      branch        loop
//!                 │            │            │
//!                 └────────────┼────────────┘
//!                              ▼
//!                         control_flow
//!                              │
//!                              ▼
//!                  validation / analysis
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!         optimization       routing        scheduling
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                           hardware
//!                              │
//!                              ▼
//!                           backend
//! ```
//!
//! `quantum::ir::control` MUST NOT depend on downstream target-specific
//! subsystems merely to expose its namespace.
//!
//! # Ownership
//!
//! ```text
//! mod.rs
//!     Namespace and public API boundary only.
//!
//! condition.rs
//!     Classical predicates and condition semantics.
//!
//! branch.rs
//!     Conditional branch structure.
//!
//! loop.rs
//!     Loop structure and iteration domains.
//!
//! control_flow.rs
//!     Canonical structured control-flow model, transfers, validation
//!     context/policy, and cross-control-flow structural semantics.
//! ```
//!
//! # What this namespace does NOT own
//!
//! This namespace does not own:
//!
//! - source parsing;
//! - source ASTs;
//! - hardware descriptions;
//! - physical topology;
//! - logical-to-physical routing;
//! - scheduling algorithms;
//! - pulse generation;
//! - calibration;
//! - simulation state;
//! - QPU execution;
//! - optimization algorithms;
//! - QEC decoding;
//! - backend communication;
//! - device-specific limits.
//!
//! Those systems may consume this namespace, but must not become dependencies
//! merely because they need to understand control flow.
//!
//! # Universal-program principle
//!
//! Control flow is semantic. It must describe what the programmer means rather
//! than which machine will execute it.
//!
//! Consequently this namespace contains no:
//!
//! - fixed qubit-count limit;
//! - fixed classical-register limit;
//! - fixed loop-iteration limit;
//! - fixed control-flow depth;
//! - fixed number of branches;
//! - fixed number of operations;
//! - vendor-specific control instructions;
//! - topology assumptions.
//!
//! Finite resource policies belong to validation/execution boundaries.
//!
//! ```text
//! semantic capacity
//!     !=
//! validation policy
//!     !=
//! hardware capacity
//! ```
//!
//! # Logical-qubit identity
//!
//! All logical qubit references exposed by control-flow implementations use the
//! canonical identity owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This namespace MUST NOT introduce a second `QubitId` type.
//!
//! A control-flow construct may refer to a logical qubit, but it must not
//! determine its physical placement.
//!
//! # Operation identity
//!
//! Control-flow constructs reference operations through the canonical
//! `OperationId` identity rather than embedding operation implementations.
//!
//! Therefore adding a new operation class does not require changes to this
//! namespace merely because the operation can appear inside a branch or loop.
//!
//! This permits control flow to contain:
//!
//! - standard gates;
//! - measurements;
//! - resets;
//! - pulse operations;
//! - analog operations;
//! - Hamiltonian evolution;
//! - logical operations;
//! - distributed operations;
//! - vendor/dialect extensions;
//! - future operation kinds.
//!
//! # Scalability
//!
//! "Infinite scalability" is interpreted semantically rather than as a claim
//! that finite Rust memory is infinite.
//!
//! The IR introduces no artificial architectural ceiling. Actual construction
//! and execution remain bounded by:
//!
//! - available memory;
//! - address space;
//! - integer representation;
//! - compiler resources;
//! - explicit service/security policy;
//! - target resources;
//! - runtime resources.
//!
//! These limits must be supplied by the appropriate boundary rather than being
//! encoded into this namespace.
//!
//! # Validation boundary
//!
//! The control-flow implementation contains validation policies and contexts
//! where appropriate. This namespace does not create a competing validation
//! system.
//!
//! In particular, callers should use the repository-wide IR validation layer
//! when validating an entire program and use the control-flow-local validation
//! facilities for control-flow-specific invariants.
//!
//! # Compatibility policy
//!
//! The repository historically contained a flat:
//!
//! ```text
//! quantum::ir::control_flow
//! ```
//!
//! implementation.
//!
//! The canonical long-term location is:
//!
//! ```text
//! quantum::ir::control::control_flow
//! ```
//!
//! The root `quantum::ir` module may retain a compatibility alias while the
//! repository migration is completed. This module itself must not duplicate
//! the old implementation.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Safety
//!
//! Unsafe Rust is explicitly forbidden.
//!
//! The child implementations are likewise expected to remain safe Rust.
//!
//! # API stability rule
//!
//! `mod.rs` should change rarely.
//!
//! Adding a new control-flow implementation should normally mean adding a new
//! child module and explicitly deciding whether its public API belongs in this
//! namespace. Existing public APIs should not be rewritten merely because an
//! unrelated control-flow construct is added.
//!
//! # Integration rule
//!
//! Downstream code should prefer explicit paths when ownership matters:
//!
//! ```text
//! crate::quantum::ir::control::condition
//! crate::quantum::ir::control::branch
//! crate::quantum::ir::control::loop
//! crate::quantum::ir::control::control_flow
//! ```
//!
//! Curated re-exports are provided below for commonly consumed semantic types.
//!
//! # No duplicated definitions
//!
//! This file intentionally contains no definitions of:
//!
//! - conditions;
//! - branches;
//! - loops;
//! - control-flow nodes;
//! - qubit identities;
//! - operation identities;
//! - validation policies;
//! - hardware capabilities.
//!
//! Those concepts have exactly one ownership location.
//!
//! # Cross-module integration contract
//!
//! ```text
//! core::identity::OperationId
//!          │
//!          ▼
//! control::control_flow
//!          │
//!          ├──────────────► control::branch
//!          │
//!          ├──────────────► control::condition
//!          │
//!          └──────────────► control::loop
//!
//! quantum::ir::qubit::QubitId
//!          │
//!          └──────────────► control-flow consumers
//!
//! control namespace
//!          │
//!          ▼
//! program / operation / validation / analysis
//! ```
//!
//! The namespace itself adds no reverse dependency.
//!
//! # Testing contract
//!
//! Tests for semantic behavior belong in the child modules or the repository's
//! integration-test layer. This namespace should only require compilation/API
//! surface tests.
//!
//! Required integration guarantees are:
//!
//! 1. all four child modules compile together;
//! 2. their public APIs are reachable through the documented paths;
//! 3. `QubitId` resolves to `quantum::ir::qubit::QubitId`;
//! 4. `OperationId` remains the canonical operation identity;
//! 5. no unsafe code is introduced;
//! 6. no target-specific dependency is introduced;
//! 7. adding unrelated IR operations does not require modifying this file.
//!
//! # Future extension
//!
//! If Zamani later introduces another fundamentally distinct structured
//! control-flow category, it should be added as another child module rather
//! than enlarging an existing file beyond its responsibility.
//!
//! Examples include future constructs for:
//!
//! - structured parallel control;
//! - asynchronous classical control;
//! - distributed control regions;
//! - quantum network control;
//! - transaction-like reversible control;
//! - fault-tolerant control regions.
//!
//! Such additions must preserve the same semantic boundary.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------
//!
//! Keep this module declarative and intentionally small.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Child modules
// =============================================================================

/// Conditional branch semantics.
///
/// Owns branch structure such as conditional execution and branch-specific
/// validation. It may consume conditions and operation/control-flow
/// references, but it does not own the canonical program operation model.
pub mod branch;

/// Classical predicates used by quantum/classical control flow.
///
/// Owns condition construction and predicate semantics. It does not own
/// hardware capabilities, scheduling, routing, or execution.
pub mod condition;

/// Canonical structured control-flow model.
///
/// Owns control-flow nodes, transfers, validation context/policy, and
/// cross-construct structural semantics.
pub mod control_flow;

/// Loop semantics.
///
/// Owns loop forms, iteration domains, loop variables, and loop-specific
/// structural validation.
pub mod r#loop;

// =============================================================================
// Curated public re-exports
// =============================================================================
//
// IMPORTANT:
//
// Do not blindly glob-re-export every child module.
//
// Explicit re-exports:
// - make ownership visible;
// - prevent accidental API expansion;
// - reduce name collisions;
// - keep the namespace stable;
// - allow child implementations to evolve internally.
//
// If a public symbol does not exist in a child module, this file must not
// invent a replacement for it. Add a re-export only when the child module's
// public API explicitly owns that symbol.

// -----------------------------------------------------------------------------
// Control-flow core
// -----------------------------------------------------------------------------

pub use control_flow::{
    ControlFlowError,
    ControlFlowResult,
    ControlFlowValidationContext,
    ControlFlowValidationPolicy,
    ControlTransfer,
};

// -----------------------------------------------------------------------------
// Branch API
// -----------------------------------------------------------------------------
//
// Branch-specific symbols are intentionally re-exported only when they are
// part of branch.rs's stable public contract.
//
// Keep the module path available even when a particular symbol is not promoted
// to this namespace.
//
// `branch` remains the authoritative path for branch-specific types.

// -----------------------------------------------------------------------------
// Condition API
// -----------------------------------------------------------------------------
//
// Condition-specific symbols remain available through:
//
//     quantum::ir::control::condition
//
// This avoids coupling this namespace to every future predicate type while
// retaining the stable module boundary.

// -----------------------------------------------------------------------------
// Loop API
// -----------------------------------------------------------------------------
//
// Loop-specific symbols remain available through:
//
//     quantum::ir::control::loop
//
// The raw identifier module name is required because `loop` is a Rust keyword.
//
// The canonical Rust path is:
//
//     quantum::ir::control::r#loop
//
// while the conceptual Zamani module remains "loop".

// =============================================================================
// Compatibility helpers
// =============================================================================

/// Returns the canonical module path for documentation, diagnostics, and
/// tooling that needs to identify the structured-control namespace.
///
/// This is deliberately a static string rather than a type-level mechanism so
/// it cannot become another source of identity or versioning.
#[must_use]
pub const fn module_path() -> &'static str {
    "quantum::ir::control"
}

/// Returns the canonical path of the logical-qubit identity used by control
/// flow.
///
/// The function does not create or wrap a qubit identifier. It exists only as
/// a stable diagnostic/tooling contract.
#[must_use]
pub const fn qubit_identity_path() -> &'static str {
    "quantum::ir::qubit::QubitId"
}

/// Returns the canonical path of the operation identity used by control flow.
///
/// Operation implementations remain owned by the program/operation layer.
#[must_use]
pub const fn operation_identity_path() -> &'static str {
    "quantum::ir::identity::OperationId"
}

// =============================================================================
// Compile-time namespace invariants
// =============================================================================
//
// These tests intentionally test only namespace-level guarantees. Semantic
// behavior belongs to the child modules and repository integration tests.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_path_is_canonical() {
        assert_eq!(module_path(), "quantum::ir::control");
    }

    #[test]
    fn logical_qubit_identity_path_is_canonical() {
        assert_eq!(
            qubit_identity_path(),
            "quantum::ir::qubit::QubitId"
        );
    }

    #[test]
    fn operation_identity_path_is_canonical() {
        assert_eq!(
            operation_identity_path(),
            "quantum::ir::identity::OperationId"
        );
    }

    #[test]
    fn unbounded_policy_does_not_define_architectural_limits() {
        let policy = ControlFlowValidationPolicy::unbounded();

        assert_eq!(policy.max_nodes, usize::MAX);
        assert_eq!(policy.max_depth, usize::MAX);
        assert_eq!(policy.max_condition_nodes, usize::MAX);
    }

    #[test]
    fn control_transfer_display_is_stable() {
        assert_eq!(ControlTransfer::Break.to_string(), "break");
        assert_eq!(ControlTransfer::Continue.to_string(), "continue");
        assert_eq!(ControlTransfer::Return.to_string(), "return");
    }
}