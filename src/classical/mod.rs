
#![forbid(unsafe_code)]

//! # Zamani Classical Computing
//!
//! Canonical path: `src/classical/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden throughout this module tree.
//!
//! ## Purpose
//!
//! This module is the composition root for Zamani's classical-computing
//! subsystem. It registers the existing classical modules and provides
//! stable public paths for classical APIs, algorithms, types, values,
//! memory, collections, expressions, control flow, concurrency,
//! optimization, lowering, validation, interoperability, execution
//! planning, and the ZCIR integration boundary.
//!
//! This file registers modules; it does not implement a compiler,
//! interpreter, semantic analyzer, IR, scheduler, or runtime.
//!
//! ## Architectural authority
//!
//! The architecture must remain consistent with:
//!
//! - `DESIGN.md`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/lexer/tokens.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns source syntax. The frontend and semantic subsystem
//! establish source-language meaning. The canonical IR represents
//! validated computational meaning. Lowering, backends, and runtimes
//! realize supported computations on actual targets.
//!
//! Grammar files do not determine Rust module registration, and registering
//! a Rust module does not establish that the corresponding language feature
//! is implemented.
//!
//! ## Canonical ownership
//!
//! This module does not define competing implementations of:
//!
//! - The source AST or semantic type system.
//! - Source files, source spans, or source maps.
//! - Canonical IR entities or their identity allocators.
//! - Quantum IR entities or quantum/classical interoperability.
//! - Backend selection, physical allocation, scheduling, or execution.
//!
//! The existing ownership boundaries are:
//!
//! - `crate::source_map`: legacy frontend source-location infrastructure.
//! - `crate::quantum::ir`: canonical quantum intermediate representation.
//! - `crate::quantum::ir::classical`: classical representations owned by
//!   the quantum IR.
//! - `crate::zuir`: canonical universal IR and shared IR contracts,
//!   once its module tree is correctly registered and compilable.
//! - `crate::classical::zcir`: classical-facing integration with ZUIR.
//! - Compiler, optimizer, backend, and runtime modules: their respective
//!   compilation and execution responsibilities.
//!
//! ZCIR must not introduce a second IR graph, duplicate entity IDs,
//! independent canonical serialization, or a competing semantic authority.
//!
//! ## POCO-REAF and scalability
//!
//! The classical subsystem must support portable program meaning across
//! supported targets and computational scales.
//!
//! This module imposes no universal maximum on processor counts, thread
//! counts, accelerator counts, memory capacity, collection lengths,
//! operation counts, register widths, tensor ranks, or node counts.
//!
//! Actual compilation and execution remain subject to available resources,
//! representational limits, target capabilities, implementation support,
//! explicit operational budgets, and program requirements.
//!
//! Configured work budgets are operational policies, not implicit changes
//! to language semantics. Unsupported requirements must be reported
//! explicitly rather than silently weakened.
//!
//! POCO-REAF does not mean unlimited physical resources or guarantee that
//! every target supports every computation.
//!
//! ## Dependency direction
//!
//! The intended architecture is:
//!
//! ```text
//! Zamani source and grammar
//!           |
//!           v
//! Frontend AST and semantic model
//!           |
//!           v
//! Canonical computational IR (ZUIR)
//!           |
//!           v
//! Classical integration (ZCIR)
//!           |
//!           v
//! Verification, analysis, legal transformations
//!           |
//!           v
//! Lowering and target feasibility
//!           |
//!           v
//! Backend and runtime
//! ```
//!
//! This is an ownership and responsibility diagram, not a claim that every
//! stage or target is already implemented.
//!
//! Classical adapters may depend on canonical IR contracts. The canonical
//! IR must not depend on ZCIR merely to implement its shared infrastructure.
//! Avoid dependency cycles between classical, quantum, and universal IR.
//!
//! ## Module registration policy
//!
//! Every declared child module must have a corresponding Rust source file
//! or explicitly configured module path. New modules must be registered
//! only after their ownership, public API, dependencies, error behavior,
//! scalability properties, and integration tests have been reviewed.
//!
//! This root owns registration and namespace composition only.
//!
//! ## Integration requirements
//!
//! 1. `src/lib.rs` must declare `pub mod classical;` exactly once.
//! 2. `src/lib.rs` must declare `pub mod zuir;` exactly once when the
//!    ZUIR module tree and all of its dependencies compile.
//! 3. Every module declared by `src/zuir/mod.rs` must exist and compile.
//! 4. ZCIR must reuse canonical ZUIR entities, IDs, verification,
//!    serialization, and pass contracts where those APIs apply.
//! 5. `src/quantum/ir/classical/` must retain its established ownership.
//! 6. The legacy frontend and legacy IR must not be silently treated as
//!    equivalent to the newer semantic model or ZUIR representation.
//! 7. No module may be registered merely to conceal a missing implementation.
//! 8. Public API changes must be checked against existing repository callers.
//!
//! The grammar and semantic specifications define language-level contracts;
//! this module only connects existing Rust implementations to those contracts.
//!
//! ## Freeze criteria
//!
//! This file is ready to freeze when:
//!
//! - Every registered production module resolves.
//! - ZCIR and ZUIR compile together through the actual crate root.
//! - Existing classical and quantum integration paths remain compatible.
//! - The module tree introduces no duplicate canonical types or IDs.
//! - Rust 1.97 compatibility and the no-unsafe policy are maintained.
//! - Formatting, compilation, tests, lint, and relevant integration checks pass.
//!
//! A successful module registration alone does not establish production
//! readiness or full POCO-REAF support.

// ---------------------------------------------------------------------------
// Classical public API and foundational contracts
// ---------------------------------------------------------------------------

/// Stable classical-facing API.
pub mod api;

/// Artifact and consumer compatibility contracts.
pub mod compatibility;

/// Target-independent configuration and operational budgets.
pub mod config;

/// Structured classical diagnostics.
pub mod diagnostics;

/// Classical error taxonomy and result types.
pub mod error;

/// Canonical identity facade for classical consumers.
pub mod ids;

/// Compilation provenance and lineage contracts.
pub mod provenance;

/// Source-span integration using the existing `span.r` file.
#[path = "span.r"]
pub mod span;

// ---------------------------------------------------------------------------
// Classical computational facilities
// ---------------------------------------------------------------------------

/// Generic classical algorithms.
pub mod algorithms;

/// Classical collection implementations and adapters.
pub mod collections;

/// Classical concurrency contracts and primitives.
pub mod concurrency;

/// Classical control-flow planning.
pub mod control_flow;

/// Classical execution planning and capability/resource requirements.
pub mod execution;

/// Classical expression analysis and transformation adapters.
pub mod expressions;

/// Classical memory contracts and memory-related adapters.
pub mod memory;

/// Classical numerical operations and numerical guarantees.
pub mod numeric;

/// Classical optimization planning and legality contracts.
pub mod optimization;

/// Classical type adapters and type-related contracts.
pub mod types;

/// Classical value operations and canonical value adapters.
pub mod values;

// ---------------------------------------------------------------------------
// Integration boundaries
// ---------------------------------------------------------------------------

/// Classical interoperability contracts and adapters.
pub mod interoperability;

/// Target-independent classical lowering adapters.
pub mod lowering;

/// Classical validation aggregation boundary.
pub mod validation;

/// Zamani Classical Intermediate Representation integration.
///
/// ZCIR is the classical-facing boundary over canonical ZUIR. It must not
/// introduce an independent IR graph, type authority, identity allocator,
/// or canonical serialization implementation.
///
/// The ZCIR module tree depends on the actual ZUIR APIs and must not be
/// considered integrated until the crate root and all upstream modules
/// compile successfully.
pub mod zcir;

// ---------------------------------------------------------------------------
// Internal tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
