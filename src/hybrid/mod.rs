
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing
//!
//! Canonical composition root for Zamani's hybrid-computing subsystem.
//!
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//!
//! ## Purpose
//!
//! This module connects computation across classical, quantum, AI/ML, tensor,
//! accelerator, HDL/hardware, distributed, networking, and future domains.
//!
//! Its architectural objective is:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever` (POCO-REAF).
//!
//! Portability is conditional on preserving program semantics, satisfying
//! required capabilities, respecting explicit constraints, and having
//! sufficient resources and compatible implementations.
//!
//! "Scale to infinity" means that the architecture imposes no arbitrary,
//! universal finite ceiling on machine size or domain composition. It does
//! not mean that a finite compiler or execution environment has infinite
//! memory, time, address space, or processing capacity.
//!
//! ## Authority and ownership
//!
//! This module owns module composition and selected public API exports only.
//! It does not define another:
//!
//! - source language, lexer, parser, or AST;
//! - semantic analyzer, type system, or ownership checker;
//! - universal IR, classical IR, quantum IR, or HDL IR;
//! - compiler pipeline or backend registry;
//! - physical resource allocator, scheduler, router, or QEC system;
//! - runtime, device manager, or vendor integration.
//!
//! The authoritative hybrid language contract is `grammar/spec/hybrid.md`.
//! The grammar architecture is documented in `grammar/hybrid/README.md`.
//! Hybrid syntax is parsed by the existing frontend; semantic facts are
//! supplied by the canonical semantic layer; quantum computation semantics
//! remain owned by `crate::quantum::ir`; and target realization remains the
//! responsibility of the existing compiler, backend, and runtime systems.
//!
//! ## Dependency direction
//!
//! The intended dependency direction is:
//!
//! `frontend -> semantic -> hybrid composition -> compiler planning -> backend/runtime`
//!
//! Hybrid adapters may reference canonical domain representations through
//! explicit interfaces. The hybrid core must not acquire dependencies on
//! vendor SDKs, physical devices, credentials, or target-specific execution
//! mechanisms.
//!
//! ## Scalability and resource policy
//!
//! This module defines no maximum number of domains, nodes, edges, operations,
//! qubits, accelerators, devices, or synchronization events.
//!
//! Operational budgets belong to `config::HybridConfig` and its associated
//! policy types. A configured budget is an operational or security policy,
//! not a restriction on the meaning of a valid Zamani program.
//!
//! Budget exhaustion, unsupported capabilities, invalid contracts, and
//! infeasible target requirements must remain distinguishable failures.
//! Implementations must never silently discard dependencies or weaken
//! program requirements to make an unsupported target appear compatible.
//!
//! ## Compatibility
//!
//! Existing implementations are retained and exposed through their owning
//! modules. This composition root deliberately avoids broad glob re-exports
//! that could create ambiguous names, conceal ownership boundaries, or make
//! internal implementation details part of the public API accidentally.
//!
//! The `api` module is the public entry point for the supported hybrid
//! contract-validation facade. Other functionality remains available through
//! its explicit owning module.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden throughout this module and its descendants.
//! Public APIs must validate external input at their appropriate trust
//! boundaries and return structured errors instead of relying on panics for
//! expected invalid-input or resource-exhaustion conditions.

// ---------------------------------------------------------------------------
// Public API and foundational contracts
// ---------------------------------------------------------------------------

/// Stable public entry point for hybrid contract validation.
pub mod api;

/// Explicit operational, validation, and resource-budget policies.
pub mod config;

/// Domain-independent contracts for cross-domain interaction.
pub mod contract;

/// Structured hybrid diagnostics and source-location association.
pub mod diagnostics;

/// Canonical structured errors and machine-readable error codes.
pub mod errors;

/// Typed identities for hybrid graphs and related entities.
pub mod ids;

/// Representation and contract-version compatibility definitions.
pub mod version;

// ---------------------------------------------------------------------------
// Domain composition and graph representation
// ---------------------------------------------------------------------------

/// Extensible identities, interfaces, registries, and domain composition.
pub mod domain;

/// Canonical cross-domain dependency graph and graph analysis.
pub mod graph;

// ---------------------------------------------------------------------------
// Cross-domain data, control, and synchronization
// ---------------------------------------------------------------------------

/// Data values, exchange contracts, ownership, layout, and transfers.
pub mod data;

/// Control flow, execution dependencies, conditions, and feedback.
pub mod control;

/// Synchronization, ordering, completion, and cancellation contracts.
pub mod sync;

// ---------------------------------------------------------------------------
// Requirements, capabilities, and downstream planning interfaces
// ---------------------------------------------------------------------------

/// Symbolic requirements, capability descriptions, constraints, and
/// feasibility assessment.
pub mod resources;

/// Interfaces and representations for downstream hybrid realization plans.
pub mod planning;

// ---------------------------------------------------------------------------
// Interchange and repository integration
// ---------------------------------------------------------------------------

/// Versioned hybrid representation interchange and validation.
pub mod interchange;

/// Adapters to existing frontend, semantic, classical, quantum, compiler,
/// and runtime implementations.
pub mod integration;

// ---------------------------------------------------------------------------
// Normative invariants and compatibility
// ---------------------------------------------------------------------------

/// Machine-checkable architectural invariants and compatibility rules.
pub mod spec;

// ---------------------------------------------------------------------------
// Deliberately narrow public facade
// ---------------------------------------------------------------------------

pub use api::{ContractValidationReport, HybridApi};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
