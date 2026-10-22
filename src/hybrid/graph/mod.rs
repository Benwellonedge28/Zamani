
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Graph Subsystem
//!
//! Path: `src/hybrid/graph/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Compose the canonical graph-model, typed-edge, construction, verification,
//! and read-only analysis facilities for Zamani hybrid computation.
//!
//! The graph represents relationships between computations owned by existing
//! canonical domain representations. It does not duplicate the source AST,
//! semantic model, classical IR, quantum IR, ZUIR, HDLIR, compiler pipeline,
//! backend implementations, or runtime.
//!
//! Hybrid composition supports classical, quantum, accelerator, tensor,
//! AI/ML, HDL/hardware, distributed, networking, and future computational
//! domains through extensible domain contracts and canonical IR references.
//!
//! ## Canonical ownership
//!
//! - `model` owns `HybridGraph`, `HybridNode`, and `CanonicalIrRef`.
//! - `edges` owns `HybridEdgeSet`, `HybridEdge`, `EdgeKind`, and edge
//!   structural validation.
//! - `builder` owns incremental graph and edge-set construction.
//! - `verify` owns whole-graph verification orchestration and the interface
//!   for resolving semantics through authoritative domain implementations.
//! - `analysis` owns read-only graph indexes, statistics, reachability,
//!   connected components, and topological-order analysis.
//! - `crate::hybrid::domain::composition` owns domain-instance composition.
//! - `crate::hybrid::ids` owns graph-scoped hybrid identities.
//! - `crate::hybrid::config` owns explicit operational budgets.
//! - `crate::hybrid::errors` owns structured hybrid errors.
//! - Canonical domain IRs own the meaning of their respective operations.
//!
//! This module owns module composition and the graph subsystem's public
//! exports only. It does not introduce another graph representation,
//! registry, semantic analyzer, scheduler, or execution engine.
//!
//! ## Validation boundaries
//!
//! Graph construction, structural verification, semantic verification, and
//! graph analysis are separate operations with distinct guarantees.
//!
//! A successfully constructed graph is not, by itself, proof that:
//!
//! - every referenced canonical IR entity exists;
//! - every required dependency from the source program is represented;
//! - every domain interface is registered or supported;
//! - every extension relationship is understood;
//! - every dependency cycle is semantically valid;
//! - a target can satisfy the program's requirements;
//! - a backend or runtime can execute the computation.
//!
//! Callers must select the verification level appropriate to their task.
//! Where semantic confidence is required, use the verifier with an
//! implementation of `HybridSemanticResolver` backed by the authoritative
//! semantic and domain facilities.
//!
//! ## Cycle semantics
//!
//! The graph is not universally required to be a directed acyclic graph.
//! Feedback and loop-carried relationships can be valid. Topological-order
//! analysis reports whether such an ordering exists; it does not establish
//! that a cyclic program is invalid.
//!
//! Cycle legality is determined by the relevant language and domain
//! semantics, not by this module's exports.
//!
//! ## Scalability and resource policy
//!
//! This module introduces no fixed universal limits on graph size, node
//! count, edge count, domain count, nesting, or hardware capacity.
//!
//! The underlying implementations use growable collections and can enforce
//! explicitly configured operational budgets through `HybridConfig`.
//! Budget exhaustion is an operational failure, not a restriction on the
//! meaning of a valid Zamani program.
//!
//! No finite machine has infinite resources. Memory, address space, execution
//! time, allocator behavior, and finite identifier representations remain
//! practical constraints and must not be misrepresented as unlimited.
//!
//! ## Determinism
//!
//! Deterministic ordering and canonical views are provided by the owning
//! model, edge, builder, verifier, and analysis APIs. This module does not
//! reorder graph contents or replace identity allocation with content hashes.
//!
//! ## Integration
//!
//! - `crate::hybrid` exposes this graph subsystem.
//! - `crate::hybrid::domain` supplies the domain composition and contracts.
//! - `crate::hybrid::config` and `crate::hybrid::errors` provide shared
//!   operational policy and failure types.
//! - Frontend and semantic adapters translate canonical source and semantic
//!   information into graph inputs without introducing a second AST or
//!   semantic model.
//! - `crate::quantum::ir` and the canonical classical and other domain
//!   representations remain authoritative for operation semantics.
//! - `crate::compiler::hybrid_pipeline` consumes validated hybrid artifacts
//!   through the integration boundary; this module does not replace that
//!   pipeline.
//! - Runtime integration consumes appropriate validated artifacts without
//!   making this graph module a runtime or scheduler.
//! - `grammar/spec/hybrid.md` and `grammar/hybrid/` remain the authorities
//!   for hybrid language composition; this Rust module does not define syntax.
//!
//! ## Completion boundary
//!
//! This module is complete when all declared child modules exist, the
//! exported names match their owning implementations, the parent hybrid
//! module exposes this subsystem as intended, and the crate's formatting,
//! compilation, test, and lint checks pass.

/// Read-only graph analysis and deterministic graph queries.
///
/// Use this module to derive structural information without modifying the
/// canonical graph or assigning physical execution resources.
pub mod analysis;

/// Incremental construction of a graph and its separate edge set.
///
/// The builder establishes the guarantees documented by its API; callers
/// must not interpret construction success as complete semantic verification.
pub mod builder;

/// Canonical typed dependency edges and edge-set validation.
///
/// Edges are stored separately from `HybridGraph` in the current model.
pub mod edges;

/// Canonical graph and node representation.
///
/// Domain operations are referenced through `CanonicalIrRef`; their
/// semantics remain owned by their canonical domain representations.
pub mod model;

/// Whole-graph structural and semantic verification.
///
/// Semantic verification delegates domain-specific questions through the
/// resolver contract rather than implementing a parallel semantic system.
pub mod verify;

// -----------------------------------------------------------------------------
// Stable, intentional public exports
// -----------------------------------------------------------------------------

/// Read-only analysis results and query interface.
pub use analysis::{
    GraphAnalysisSummary,
    HybridGraphAnalysis,
    TopologicalOrder,
};

/// Graph construction results and builder interface.
pub use builder::{
    BuiltHybridGraph,
    HybridGraphBuilder,
};

/// Canonical edge representation, extension identity, and validation report.
pub use edges::{
    EdgeKind,
    EdgeValidationReport,
    ExtensionEdgeKind,
    HybridEdge,
    HybridEdgeSet,
};

/// Canonical graph, node, and external-IR reference representation.
pub use model::{
    CanonicalIrRef,
    HybridGraph,
    HybridNode,
};

/// Whole-graph verification interface and result types.
pub use verify::{
    HybridGraphVerifier,
    HybridSemanticResolver,
    VerificationReport,
};
