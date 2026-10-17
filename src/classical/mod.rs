
#![forbid(unsafe_code)]

//! Zamani Classical Computing subsystem.
//!
//! Path: `src/classical/mod.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden throughout this module tree.
//!
//! # Purpose
//!
//! Provide the canonical Rust module boundary for classical computation in
//! Zamani. This file registers existing implementations and establishes
//! stable paths for classical APIs, values, types, collections, algorithms,
//! memory, expressions, control flow, concurrency, validation, optimization,
//! lowering, interoperability, and execution planning.
//!
//! This module is a composition root. It is not an implementation of a
//! compiler, interpreter, semantic analyzer, intermediate representation,
//! scheduler, runtime, or hardware abstraction layer.
//!
//! # Architectural authority
//!
//! Source syntax and grammar composition are owned by the grammar subsystem,
//! including `grammar/Zamani.g4` and the canonical ANTLR grammar boundaries.
//!
//! Language meaning and portability are governed by the applicable language
//! specifications, including:
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! These specifications distinguish source semantics from target realization.
//! Grammar defines syntax; it does not define runtime implementation details.
//!
//! # Canonical ownership
//!
//! This module does not introduce a competing AST, semantic type system,
//! source-map implementation, identity allocator, or intermediate
//! representation.
//!
//! Existing repository boundaries remain authoritative:
//!
//! - The frontend owns source AST and semantic analysis.
//! - `crate::source_map` owns canonical source locations and source files.
//! - `crate::zuir` owns the universal computation IR and its shared contracts,
//!   where that subsystem is available and integrated.
//! - `crate::quantum::ir` retains its existing canonical quantum IR.
//! - `crate::quantum::ir::classical` retains its existing classical
//!   computation representations used by that IR.
//! - Compiler, optimizer, backend, and runtime subsystems own their respective
//!   compilation and execution responsibilities.
//!
//! Classical modules may provide adapters, analysis, policies, algorithms,
//! validation, and target-independent planning. They must not silently
//! replace canonical types or redefine source-language semantics.
//!
//! # POCO-REAF and scalability
//!
//! The classical subsystem must support programs across different machine
//! sizes and supported computational targets without imposing arbitrary,
//! universal semantic ceilings.
//!
//! This module defines no maximum for processor counts, thread counts,
//! accelerator counts, memory capacity, collection length, tensor rank,
//! operation counts, or distributed-node counts.
//!
//! Finite operational budgets and actual target limits remain valid when
//! explicit, configurable where appropriate, and reported as operational or
//! feasibility constraints rather than language-semantic restrictions.
//!
//! No implementation can guarantee literally infinite execution: successful
//! realization depends on available resources, target capabilities, program
//! requirements, and the semantics that must be preserved.
//!
//! # Dependency direction
//!
//! The intended architecture is:
//!
//! source -> frontend AST -> semantic model -> canonical IR -> optimization
//! -> lowering and target planning -> backend/runtime execution
//!
//! Module registration here must not reverse that dependency direction.
//! In particular, validation and planning modules must not become substitutes
//! for the canonical frontend semantic model or canonical IR.
//!
//! # Module contracts
//!
//! Each registered module owns its documented responsibilities. This root
//! owns module registration and the `crate::classical::*` namespace only.
//!
//! New child modules must be added only after their file exists, its ownership
//! contract is documented, and its dependency direction has been reviewed.
//! Do not register placeholder files as implementations.
//!
//! # Integration
//!
//! The crate root must declare `pub mod classical;` for downstream code to
//! access this subsystem through `crate::classical`.
//!
//! The `span` module currently uses the existing `span.r` path because that
//! is the filename in the repository. Rename that file to `span.rs` in a
//! separate, reviewed change if the repository is ready to correct the path.
//!
//! # Verification status
//!
//! This module root registers the existing Rust module boundaries. Its
//! registration alone does not establish that every child module compiles or
//! that the subsystem is production-ready. The complete workspace must pass
//! the required build, test, formatting, lint, and integration checks.

/// Stable public API for classical subsystem configuration and
/// compatibility operations.
pub mod api;

/// Generic classical algorithms.
pub mod algorithms;

/// Classical collections and collection adapters.
pub mod collections;

/// Artifact and consumer compatibility contracts.
pub mod compatibility;

/// Classical concurrency contracts and primitives.
pub mod concurrency;

/// Target-independent classical configuration and operational budgets.
pub mod config;

/// Classical control-flow planning.
pub mod control_flow;

/// Structured classical diagnostics and diagnostic adapters.
pub mod diagnostics;

/// Classical error taxonomy and result types.
pub mod error;

/// Classical execution planning and capability/resource requirements.
pub mod execution;

/// Classical expression analysis and transformation adapters.
pub mod expressions;

/// Canonical identity facade for classical consumers.
pub mod ids;

/// Classical interoperability contracts and adapters.
pub mod interoperability;

/// Target-independent classical lowering adapters.
pub mod lowering;

/// Classical memory contracts and memory-related adapters.
pub mod memory;

/// Classical numerical operations and numerical guarantees.
pub mod numeric;

/// Classical optimization planning and legality contracts.
pub mod optimization;

/// Compilation provenance and lineage contracts.
pub mod provenance;

/// Classical type adapters and type-related contracts.
pub mod types;

/// Classical value operations and canonical value adapters.
pub mod values;

/// Classical validation aggregation boundary.
pub mod validation;

/// Source-span integration using the existing repository filename.
///
/// `span.r` currently contains Rust source but does not have the conventional
/// `.rs` extension. This explicit path keeps the module discoverable without
/// inventing a second source-span implementation.
#[path = "span.r"]
pub mod span;

#[cfg(test)]
mod tests;
