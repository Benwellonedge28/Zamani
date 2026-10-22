
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Integration Boundary
//!
//! Path: `src/hybrid/integration/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! This module is the composition root for adapters that connect the
//! canonical Zamani frontend, semantic model, classical computation,
//! and quantum IR to the hybrid-composition subsystem.
//!
//! It declares and organizes integration modules. It does not implement
//! a second parser, AST, semantic model, intermediate representation,
//! compiler pipeline, scheduler, backend, or runtime.
//!
//! ## Canonical ownership
//!
//! - Source syntax and tokenization:
//!   `crate::frontend`.
//! - Hybrid syntax recognition:
//!   `crate::frontend::parser::domains::hybrid`.
//! - Canonical AST:
//!   the existing frontend AST.
//! - Semantic analysis and validation:
//!   `crate::frontend::semantic`.
//! - Quantum computation semantics:
//!   `crate::quantum::ir`.
//! - Classical computation:
//!   the repository's existing canonical classical facilities.
//! - Hybrid cross-domain integration:
//!   the adapters declared below, together with the canonical
//!   hybrid-composition contracts.
//! - Compilation and target realization:
//!   `crate::compiler` and the existing backend subsystem.
//! - Execution lifecycle:
//!   `crate::runtime`.
//!
//! This module does not transfer ownership of any of these systems.
//!
//! ## Dependency direction
//!
//! The intended architectural flow is:
//!
//! ```text
//! Canonical frontend and AST
//!           |
//!           v
//! Canonical semantic analysis
//!           |
//!           v
//! Hybrid composition contracts
//!           |
//!           +-------------------+
//!           |                   |
//!           v                   v
//!     Classical adapter   Quantum IR adapter
//!           |                   |
//!           +---------+---------+
//!                     |
//!                     v
//!             Compiler integration
//!                     |
//!                     v
//!          Existing backends/runtime
//! ```
//!
//! This diagram documents the ownership boundary. It does not claim
//! that every downstream integration has already been implemented.
//!
//! ## Existing adapters
//!
//! - [`frontend`] bridges hybrid syntax to the canonical AST boundary.
//! - [`semantic`] delegates hybrid semantic validation to the
//!   authoritative frontend semantic implementation.
//! - [`classical`] defines the classical-to-hybrid integration boundary.
//! - [`quantum`] defines the canonical quantum-IR integration boundary.
//!
//! Each adapter retains its own contract, error handling, and integration
//! responsibilities. This module does not reimplement their behavior.
//!
//! ## Compiler and runtime integration
//!
//! The existing compiler hybrid pipeline is:
//!
//! `crate::compiler::hybrid_pipeline`.
//!
//! The existing hybrid runtime entry point is:
//!
//! `crate::runtime::hybrid_kernel`.
//!
//! Dedicated `compiler.rs` and `runtime.rs` adapters are not declared here
//! because those files were not present at the inspected paths. They must
//! be introduced only after their contracts have been reconciled with the
//! actual compiler pipeline, runtime API, and hybrid representation.
//!
//! Do not create a competing pipeline or execution engine merely to fill
//! these integration points.
//!
//! ## Grammar and semantic conformance
//!
//! The integration boundary must conform to:
//!
//! - `grammar/hybrid/hybrid.g4`
//! - `grammar/hybrid/README.md`
//! - `grammar/spec/hybrid.md`
//! - `grammar/specification/semantic-model.md`
//!
//! The grammar defines syntax, and the semantic subsystem defines meaning.
//! Adapters must preserve source provenance, semantic requirements,
//! domain relationships, data dependencies, effects, and synchronization
//! guarantees wherever those facts are represented by their authorities.
//!
//! Syntax recognition alone is not semantic validation. Semantic validity
//! alone is not proof that a target can execute a program.
//!
//! ## POCO-REAF and scalability
//!
//! The integration layer introduces no universal fixed limits on:
//!
//! - computational domains;
//! - operations or graph relationships;
//! - values or dependencies;
//! - logical qubits or classical resources;
//! - accelerators, devices, nodes, or processes;
//! - tensor dimensions or tensor rank;
//! - synchronization events;
//! - hybrid composition depth.
//!
//! It does not introduce fixed-size registries, duplicate resource tables,
//! or machine-specific assumptions into the portable representation.
//!
//! Concrete implementations must use the owning subsystem's explicit
//! operational and security policies. Resource exhaustion must be
//! reported through the established structured error or diagnostic
//! mechanism; it must never silently change program semantics.
//!
//! POCO-REAF means that portable program meaning is independent of
//! accidental target-size assumptions. It does not mean that finite
//! machines can provide infinite memory or execute infinite workloads.
//!
//! ## Compatibility and public API
//!
//! This module exposes named adapter modules rather than glob-re-exporting
//! their contents. Consumers should import the specific adapter contract
//! they require.
//!
//! This avoids ambiguous names, accidental API collisions, and unintended
//! coupling between the hybrid facade and adapter implementation details.
//!
//! Public adapter APIs must be changed deliberately and tested against their
//! existing callers. New adapters must follow the same ownership rules.
//!
//! ## Completion boundary
//!
//! This module is complete when:
//!
//! - Every declared child module exists and compiles.
//! - Every child adapter retains its canonical subsystem ownership.
//! - No duplicate IR, semantic validator, compiler pipeline, or runtime
//!   is introduced through module composition.
//! - The parent hybrid module exposes this module exactly once.
//! - The crate's public module declarations are consistent with the
//!   actual frontend, compiler, and runtime module layout.
//! - Formatting, compilation, tests, and Clippy pass on the supported
//!   Rust baseline.

/// Integration between hybrid syntax and the canonical frontend AST.
///
/// This adapter preserves syntax/source information and delegates AST
/// construction to the existing frontend boundary.
pub mod frontend;

/// Integration between hybrid computation and canonical semantic analysis.
///
/// Semantic rules and validation remain owned by the frontend semantic
/// subsystem.
pub mod semantic;

/// Integration between canonical classical computation and hybrid
/// composition.
///
/// Classical operation and value semantics remain owned by the existing
/// classical facilities.
pub mod classical;

/// Integration between the canonical quantum IR and hybrid composition.
///
/// Quantum operation semantics remain owned by `crate::quantum::ir`.
pub mod quantum;
