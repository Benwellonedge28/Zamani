#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Native Frontend — Semantic Analysis
//!
//! Path: `src/frontend/semantic/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! This module is the public orchestration boundary for semantic analysis
//! over Zamani's canonical frontend AST.
//!
//! It organizes the semantic API, configuration, invocation context,
//! per-analysis session, pipeline, semantic model, symbols, types, effects,
//! capabilities, resources, contracts, domain validation, general validation,
//! diagnostics, traversal, incremental analysis, and tests.
//!
//! This file defines module ownership and the stable public API boundary.
//! It does not duplicate the implementation owned by those modules.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! The implementation must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/frontend/ast/node/`
//! - `src/frontend/parser/`
//! - The canonical downstream IR interfaces, including `quantum::ir`.
//!
//! The grammar and normative specifications define language meaning.
//! The canonical AST owns source structure and node identity.
//! This subsystem determines source-level semantic meaning.
//! Downstream IR and target systems determine how that meaning is realized.
//!
//! ## OWNS
//!
//! - The semantic subsystem's module hierarchy.
//! - The semantic subsystem's public API re-exports.
//! - The boundary between semantic analysis and other compiler stages.
//! - Test-only registration of the semantic conformance suite.
//!
//! ## DOES_NOT_OWN
//!
//! This module does not implement:
//!
//! - Lexing, parsing, or AST construction.
//! - A second AST or an alternative semantic type system.
//! - Name resolution, type inference, or individual validation algorithms.
//! - Physical resource allocation or target-feasibility decisions.
//! - Classical, quantum, or HDL intermediate representations.
//! - Quantum routing, pulse generation, scheduling, or error correction.
//! - Backend code generation, device discovery, or runtime execution.
//! - The legacy analyzer in `src/semantic.rs`.
//!
//! Each responsibility belongs to its designated implementation module.
//!
//! ## TARGET-INDEPENDENT SEMANTICS
//!
//! The semantic layer must describe what a program means independently of
//! the particular machine on which it will execute.
//!
//! It must support the shared language semantics needed by classical,
//! quantum, HDL, hybrid, AI/data, concurrent, distributed, and future
//! computational domains through explicit domain interfaces.
//!
//! A domain-specific validator may add domain rules, but it must reuse the
//! canonical AST, symbols, type system, diagnostics, and semantic model.
//! It must not create a competing source-language semantic authority.
//!
//! Quantum validation must preserve `quantum::ir` as the canonical quantum
//! IR boundary. Physical qubit mapping and pulse-level realization remain
//! downstream responsibilities.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! No language-level hardware ceiling is defined by this module.
//! In particular, the semantic architecture must not introduce arbitrary
//! maximum counts for processors, qubits, accelerators, nodes, memory,
//! threads, register widths, tensor dimensions, or devices.
//!
//! Symbolic resource quantities and constraints must remain representable
//! until the responsible semantic or downstream stage is authorized to
//! evaluate them.
//!
//! Compiler operational budgets are distinct from language semantics.
//! They may be configurable to protect the compiler from resource exhaustion,
//! but exhausting a budget must produce an explicit incomplete, aborted,
//! or failed analysis outcome. Incomplete analysis must never certify a
//! program as semantically valid.
//!
//! No finite implementation can process literally infinite input. The
//! intended guarantee is that the language imposes no arbitrary universal
//! hardware ceiling and that analysis scales subject to available resources,
//! representation limits, and explicitly configured operational policies.
//!
//! ## ERROR AND DETERMINISM CONTRACT
//!
//! Ordinary source errors should be represented by structured diagnostics.
//! Infrastructure or integration failures must remain distinguishable from
//! source-language errors.
//!
//! Unresolved, invalid, tainted, unsupported, and incomplete semantic facts
//! must not silently become valid facts.
//!
//! Equivalent inputs, configuration, and extension snapshots must produce
//! equivalent semantic outcomes and deterministically ordered diagnostics
//! wherever the public contract requires determinism.
//!
//! No unsafe Rust is permitted in this subsystem.
//!
//! ## LEGACY COMPATIBILITY
//!
//! `src/semantic.rs` remains the legacy semantic implementation during
//! migration. It must not be silently replaced, copied wholesale, or treated
//! as a second permanent source of semantic truth.
//!
//! Migration must be driven by verified consumers, compatibility tests,
//! and conformance tests against the canonical frontend AST. Compiler
//! entry points should move to this subsystem only after the native path
//! and its downstream interfaces have been verified.
//!
//! ## MODULE INTEGRATION CONTRACT
//!
//! Each subdirectory owns its own `mod.rs` aggregation boundary. This file
//! declares the subsystem-level modules; it does not redeclare their
//! internal files or reimplement their public interfaces.
//!
//! The semantic API accepts the canonical program type from
//! `crate::frontend::ast::node::program::Program`.
//! The API, configuration, context, session, pipeline, model, diagnostics,
//! and domain modules must agree on their documented contracts.
//!
//! The parent frontend must expose this module exactly once. The crate root
//! must expose the frontend module before this path can be used by other
//! crate modules.
//!
//! ## FREEZE CRITERIA
//!
//! This file may be frozen when:
//!
//! 1. Every declared module exists and compiles.
//! 2. Every child directory correctly exports its intended public modules.
//! 3. The frontend exposes one canonical AST and semantic entry point.
//! 4. Public re-exports resolve to the existing API implementation.
//! 5. The legacy analyzer remains compatible until migration gates pass.
//! 6. Domain validation shares core semantic infrastructure.
//! 7. The test suite compiles and its required tests pass.
//! 8. No unsafe code or universal machine-capacity ceiling is introduced.
//! 9. Incomplete analysis cannot be reported as valid.
//! 10. Formatting, linting, and supported-toolchain checks pass.

// -----------------------------------------------------------------------------
// Public semantic API
// -----------------------------------------------------------------------------
//
// `api.rs` owns the public service and result contracts. Re-export only that
// established API here; subsystem-specific implementation types remain
// available through their owning modules.

pub mod api;

pub use api::{
    analyze,
    SemanticApiError,
    SemanticDiagnostic,
    SemanticModelState,
    SemanticResult,
    SemanticService,
    SemanticStatus,
    SEMANTIC_API_VERSION,
};

// -----------------------------------------------------------------------------
// Configuration and analysis lifecycle
// -----------------------------------------------------------------------------
//
// Configuration, shared context, invocation-local state, and pass ordering
// have distinct ownership. Keeping these boundaries separate allows
// independent analyses to remain isolated and deterministic.

pub mod config;
pub mod context;
pub mod session;
pub mod pipeline;

// -----------------------------------------------------------------------------
// Canonical target-independent semantic model
// -----------------------------------------------------------------------------

pub mod model;

// -----------------------------------------------------------------------------
// Shared semantic infrastructure
// -----------------------------------------------------------------------------

pub mod symbols;
pub mod types;
pub mod effects;
pub mod capabilities;
pub mod resources;
pub mod contracts;

// -----------------------------------------------------------------------------
// Computational-domain validation
// -----------------------------------------------------------------------------
//
// The domains facade owns the registration and export of its domain-specific
// validators and extension registry. Domain validators must reuse the shared
// semantic infrastructure rather than create parallel semantic models.

pub mod domains;

// -----------------------------------------------------------------------------
// General validation and shared compiler services
// -----------------------------------------------------------------------------

pub mod validation;
pub mod diagnostics;
pub mod traversal;
pub mod incremental;

// -----------------------------------------------------------------------------
// Test suite
// -----------------------------------------------------------------------------
//
// Tests are compiled only for test builds. The tests module owns registration
// of the focused test files; this production module does not duplicate them.

#[cfg(test)]
mod tests;