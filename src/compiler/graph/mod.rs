
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Compilation Graph
//!
//! Path: `src/compiler/graph/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only. Unsafe Rust is forbidden.
//!
//! ## 1. Purpose
//!
//! Provides the public composition boundary for Zamani's compiler-wide
//! compilation graph subsystem.
//!
//! The subsystem coordinates relationships between compiler entities
//! owned by authoritative repository components.
//!
//! It provides:
//!
//! - A domain-neutral structural compilation graph.
//! - Stable graph and node identities.
//! - Canonical references to externally owned entities.
//! - Typed, extensible graph relationships.
//! - Incremental graph construction.
//! - Structural and delegated semantic validation.
//! - Immutable graph snapshots.
//! - Transactional graph transformations.
//! - A read-only facade over canonical compilation dependencies.
//!
//! The compilation graph is an integration and coordination model.
//!
//! It is NOT:
//!
//! - A programming-language AST.
//! - A semantic model.
//! - Another intermediate representation.
//! - Another compilation-unit dependency authority.
//! - A pipeline-stage scheduler.
//! - A physical hardware graph.
//! - A runtime execution graph.
//!
//! ## 2. Architectural ownership
//!
//! Each child module has one authoritative responsibility.
//!
//! | Module | Ownership |
//! |--------|-----------|
//! | model | Graph identity, nodes, entity references and structural invariants |
//! | edges | Typed directed relationships and adjacency indexes |
//! | builder | Incremental construction and graph composition |
//! | validation | Structural validation and delegated semantic verification |
//! | snapshot | Immutable validated graph snapshots |
//! | transformation | Transactional immutable graph transformations |
//! | dependency | Read-only integration with canonical compilation dependencies |
//!
//! No child module should reimplement another child's authoritative
//! data structures or algorithms.
//!
//! ## 3. Canonical repository authorities
//!
//! ### Frontend and semantic authority
//!
//! `crate::frontend`
//!
//! Owns source parsing, AST construction, name resolution, types,
//! ownership, effects, requirements, capabilities and semantic meaning.
//!
//! The graph only references established canonical entities.
//!
//! ### Compilation-unit dependency authority
//!
//! `crate::compiler::context::dependency_context`
//!
//! Owns compilation-unit identities, dependency relationships,
//! dependency traversal and dependency-first ordering.
//!
//! `dependency.rs` exposes a read-only integration facade.
//!
//! This module MUST NOT create a competing dependency store.
//!
//! ### Dependency analysis
//!
//! `crate::compiler::analysis::dependency`
//!
//! Consumes the canonical DependencyContext and provides compiler-facing
//! dependency analysis.
//!
//! ### Pipeline ordering
//!
//! `crate::compiler::pipeline::ordering`
//!
//! Owns pipeline-stage dependency validation, topological ordering
//! and dependency generations.
//!
//! Pipeline ordering MUST NOT be inferred from arbitrary graph edges.
//!
//! ### Parallel compilation
//!
//! `crate::compiler::parallel::dependencies`
//!
//! Owns dependency-aware compiler work relationships.
//!
//! The compilation graph does not own worker scheduling or execution.
//!
//! ### Incremental compilation
//!
//! `crate::compiler::incremental`
//!
//! Owns fingerprints, cache validation and dependency-driven invalidation.
//!
//! Graph snapshots do not independently define cache identity.
//!
//! ### Workload planning
//!
//! `crate::compiler::planning`
//!
//! Owns abstract feasibility, partitioning, data movement,
//! routing and scheduling plans.
//!
//! This graph does not allocate physical resources.
//!
//! ### Canonical representations
//!
//! The graph may reference entities owned by:
//!
//! - `crate::zuir`
//! - `crate::classical`
//! - `crate::quantum::ir`
//! - `crate::hdl::hdlir`
//! - `crate::ai`
//! - `crate::hybrid`
//! - `crate::distributed`
//! - Other registered or future computational domains.
//!
//! The graph MUST NOT copy or redefine their semantics.
//!
//! ### Backend and runtime
//!
//! `crate::backend` owns backend realization.
//!
//! `crate::runtime` owns runtime execution and physical resource use.
//!
//! Neither responsibility belongs to this module.
//!
//! ## 4. Graph relationship semantics
//!
//! Graph relationships are directed and typed.
//!
//! Structural validity does not imply semantic validity.
//!
//! The meaning of an edge must be established by its authoritative
//! semantic or compiler subsystem.
//!
//! In particular:
//!
//! - Compilation dependencies use DependencyContext.
//! - Pipeline prerequisites use pipeline::ordering.
//! - Parallel work dependencies use parallel::dependencies.
//! - Cache invalidation uses incremental::invalidation.
//! - Partitioning relationships use planning::partitioning.
//!
//! A general compilation graph relationship MUST NOT automatically
//! impose compilation order, execution order or physical placement.
//!
//! Cycles are not universally invalid.
//!
//! Cycle legality depends on the relationship semantics and the
//! authoritative subsystem responsible for interpreting that relationship.
//!
//! ## 5. Scalability
//!
//! This module imposes no arbitrary universal maximum on:
//!
//! - Nodes.
//! - Edges.
//! - Computational domains.
//! - Graph relationships.
//! - Compilation units.
//! - Modules.
//! - Functions.
//! - Backend providers.
//! - Metadata entries.
//! - Hardware categories.
//! - Distributed participants.
//! - Quantum resources.
//! - Tensor dimensions.
//! - Compiler operations.
//!
//! The subsystem uses dynamically sized collections through its
//! underlying implementations.
//!
//! Actual capacity is determined by:
//!
//! - Available memory.
//! - Address-space capacity.
//! - Collection representation.
//! - Allocation success.
//! - Processing capacity.
//! - Execution time.
//! - Explicit operational budgets.
//!
//! Explicit operational budgets are policies, not universal semantic
//! or architectural ceilings.
//!
//! Resource exhaustion MUST be reported distinctly from semantic
//! invalidity.
//!
//! No implementation can guarantee literal infinite memory or execution.
//!
//! ## 6. Determinism
//!
//! Deterministic graph behavior is provided by the child modules.
//!
//! The public composition boundary must not introduce:
//!
//! - Randomized graph ordering.
//! - Pointer-based identities.
//! - Machine-specific identifiers.
//! - Unstable iteration guarantees.
//! - Implicit timestamps.
//! - Nondeterministic graph mutation.
//!
//! Ordered collections and canonical declaration ordering are preserved.
//!
//! ## 7. Immutability and concurrency
//!
//! Completed graphs, edge sets and snapshots are immutable through
//! their public interfaces.
//!
//! Construction occurs through CompilationGraphBuilder.
//!
//! Changes occur through GraphTransformation, producing a new validated
//! graph rather than modifying an existing graph.
//!
//! The composition module creates no global mutable state, locks,
//! worker threads or background execution.
//!
//! Thread-safety follows the concrete child types and their contained
//! values.
//!
//! ## 8. Serialization and compatibility
//!
//! Graph model, edge-set and snapshot schemas have independent versions.
//!
//! Their versioning must not be conflated with:
//!
//! - Zamani language version.
//! - Grammar version.
//! - Semantic-model version.
//! - ZUIR version.
//! - Domain IR versions.
//! - Compiler API version.
//! - Backend ABI version.
//! - Runtime version.
//!
//! Deserialization and snapshot construction must retain the validation
//! guarantees of their owning modules.
//!
//! Structural validity does not establish semantic trust or target
//! feasibility.
//!
//! ## 9. Grammar integration
//!
//! The graph subsystem is downstream of the frontend.
//!
//! Relevant language specifications include:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/MANIFEST.yaml`
//! - `grammar/compile/compile.g4`
//! - `grammar/modules/dependencies.g4`
//! - `grammar/distributed/partitioning.g4`
//! - `grammar/hybrid/hybrid.g4`
//! - `grammar/core/requirements.g4`
//! - `grammar/core/capabilities.g4`
//! - `grammar/core/constraints.g4`
//! - `grammar/core/hints.g4`
//!
//! Grammar declarations describe source-level intent.
//!
//! They do not directly instantiate compiler graph nodes or edges.
//!
//! The frontend and semantic authorities must first resolve and validate
//! the relevant declarations.
//!
//! ## 10. Public integration
//!
//! The compiler composition root must declare:
//!
//! `pub mod graph;`
//!
//! Downstream modules should consume the narrowest appropriate API.
//!
//! They should not depend on private graph implementation details.
//!
//! ## 11. Compatibility
//!
//! Existing child-module APIs remain authoritative.
//!
//! This module only composes and exports them.
//!
//! New computational domains and relationship types must not require
//! changing this composition root merely to become representable.


// ============================================================================
// CHILD MODULES
// ============================================================================

/// Incremental graph construction and composition.
///
/// Owns `CompilationGraphBuilder`, `GraphBuilderConfig` and
/// `BuiltCompilationGraph`.
///
/// Does not own graph-node or edge semantics.
pub mod builder;

/// Read-only integration with canonical compilation-unit dependencies.
///
/// The authoritative dependency store remains
/// `compiler::context::dependency_context`.
pub mod dependency;

/// Typed directed graph relationships and adjacency indexes.
///
/// Does not own semantic dependency discovery or pipeline ordering.
pub mod edges;

/// Structural graph model, graph identity, nodes and canonical entity
/// references.
pub mod model;

/// Immutable validated graph snapshots.
///
/// Does not own content fingerprints or incremental cache persistence.
pub mod snapshot;

/// Immutable transactional graph transformations.
///
/// Does not mutate the original graph or independently define IR semantics.
pub mod transformation;

/// Structural validation and delegated semantic validation.
///
/// Does not replace frontend or domain-specific verification.
pub mod validation;


// ============================================================================
// MODULE API VERSION
// ============================================================================

/// Public API version of the compiler graph composition boundary.
///
/// This version describes the module's public composition and export
/// contract. It is independent of child-module schema versions.
pub const COMPILATION_GRAPH_API_VERSION: u16 = 1;

/// Canonical module name.
pub const COMPILATION_GRAPH_MODULE_NAME: &str =
    "zamani.compiler.graph";


// ============================================================================
// MODEL EXPORTS
// ============================================================================

pub use model::{
    CanonicalEntityRef,
    CompilationGraph,
    CompilationGraphId,
    CompilationGraphNode,
    CompilationGraphNodeId,
    CompilationGraphNodeKind,
    GraphModelError,
    GraphModelResult,
    COMPILATION_GRAPH_MODEL_API_VERSION,
    COMPILATION_GRAPH_SCHEMA_VERSION,
};


// ============================================================================
// EDGE EXPORTS
// ============================================================================

pub use edges::{
    CompilationGraphEdge,
    CompilationGraphEdgeId,
    CompilationGraphEdgeKind,
    CompilationGraphEdgeSet,
    GraphEdgesError,
    GraphEdgesResult,
    COMPILATION_GRAPH_EDGES_API_VERSION,
    COMPILATION_GRAPH_EDGES_SCHEMA_VERSION,
};


// ============================================================================
// BUILDER EXPORTS
// ============================================================================

pub use builder::{
    BuiltCompilationGraph,
    CompilationGraphBuilder,
    GraphBuilderConfig,
    GraphBuilderError,
    GraphBuilderResult,
    COMPILATION_GRAPH_BUILDER_API_VERSION,
};


// ============================================================================
// VALIDATION EXPORTS
// ============================================================================

pub use validation::{
    CompilationGraphSemanticResolver,
    CompilationGraphValidationReport,
    CompilationGraphValidator,
    GraphValidationError,
    GraphValidationResult,
    validate_compilation_graph,
    validate_compilation_graph_with_resolver,
    COMPILATION_GRAPH_VALIDATION_API_VERSION,
    COMPILATION_GRAPH_VALIDATION_SCHEMA_VERSION,
};


// ============================================================================
// SNAPSHOT EXPORTS
// ============================================================================

pub use snapshot::{
    CompilationGraphSnapshot,
    GraphSnapshotError,
    GraphSnapshotResult,
    COMPILATION_GRAPH_SNAPSHOT_API_VERSION,
    COMPILATION_GRAPH_SNAPSHOT_SCHEMA_VERSION,
};


// ============================================================================
// TRANSFORMATION EXPORTS
// ============================================================================

pub use transformation::{
    GraphEdit,
    GraphTransformation,
    GraphTransformationError,
    GraphTransformationResult,
    TransformedCompilationGraph,
    COMPILATION_GRAPH_TRANSFORMATION_API_VERSION,
};


// ============================================================================
// CANONICAL DEPENDENCY INTEGRATION EXPORTS
// ============================================================================

pub use dependency::{
    DependencyGraph,
    DEPENDENCY_GRAPH_API_VERSION,
    DEPENDENCY_GRAPH_MODULE_NAME,
    GraphBuilder,
    GraphEdge,
    GraphEdgeKind,
    GraphError,
    GraphUnit,
    GraphUnitId,
    GraphUnitKind,
};


// ============================================================================
// PUBLIC API TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The composition API and all independently versioned components
    /// must expose valid nonzero versions.
    #[test]
    fn graph_api_versions_are_nonzero() {
        assert_ne!(COMPILATION_GRAPH_API_VERSION, 0);

        assert_ne!(COMPILATION_GRAPH_MODEL_API_VERSION, 0);
        assert_ne!(COMPILATION_GRAPH_SCHEMA_VERSION, 0);

        assert_ne!(COMPILATION_GRAPH_EDGES_API_VERSION, 0);
        assert_ne!(COMPILATION_GRAPH_EDGES_SCHEMA_VERSION, 0);

        assert_ne!(COMPILATION_GRAPH_BUILDER_API_VERSION, 0);

        assert_ne!(COMPILATION_GRAPH_VALIDATION_API_VERSION, 0);
        assert_ne!(COMPILATION_GRAPH_VALIDATION_SCHEMA_VERSION, 0);

        assert_ne!(COMPILATION_GRAPH_SNAPSHOT_API_VERSION, 0);
        assert_ne!(COMPILATION_GRAPH_SNAPSHOT_SCHEMA_VERSION, 0);

        assert_ne!(COMPILATION_GRAPH_TRANSFORMATION_API_VERSION, 0);
        assert_ne!(DEPENDENCY_GRAPH_API_VERSION, 0);
    }

    /// The public module identity must remain stable.
    #[test]
    fn graph_module_identity_is_stable() {
        assert_eq!(
            COMPILATION_GRAPH_MODULE_NAME,
            "zamani.compiler.graph"
        );
    }

    /// The canonical dependency integration must remain separately
    /// identifiable from the structural compilation graph.
    #[test]
    fn dependency_facade_is_separate_from_structural_graph() {
        assert_eq!(
            DEPENDENCY_GRAPH_MODULE_NAME,
            "zamani.compiler.graph.dependency"
        );
    }

    /// The public API must expose the structural graph model without
    /// requiring consumers to access private implementation details.
    #[test]
    fn structural_graph_types_are_publicly_composed() {
        fn accepts_graph(_: &CompilationGraph) {}
        fn accepts_edges(_: &CompilationGraphEdgeSet) {}
        fn accepts_snapshot(_: &CompilationGraphSnapshot) {}

        let _ = accepts_graph;
        let _ = accepts_edges;
        let _ = accepts_snapshot;
    }

    /// The public API must expose the builder and transformation
    /// contracts independently.
    #[test]
    fn graph_construction_and_transformation_apis_are_public() {
        fn accepts_builder(_: &CompilationGraphBuilder) {}
        fn accepts_config(_: &GraphBuilderConfig) {}
        fn accepts_transformation(_: &GraphTransformation) {}

        let _ = accepts_builder;
        let _ = accepts_config;
        let _ = accepts_transformation;
    }

    /// Semantic validation must remain delegated to an explicit
    /// authoritative resolver.
    #[test]
    fn semantic_validation_is_an_explicit_contract() {
        fn accepts_resolver<T: CompilationGraphSemanticResolver>(_: &T) {}

        let _ = accepts_resolver::<
            dyn CompilationGraphSemanticResolver
        >;
    }
}
