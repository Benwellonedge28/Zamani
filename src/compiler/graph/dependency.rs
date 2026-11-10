
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Compiler — Canonical Dependency Graph Integration.
//!
//! Path:
//!     src/compiler/graph/dependency.rs
//!
//! Rust:
//!     Edition: 2021
//!     Minimum version: 1.97
//!
//! Safety:
//!     Safe Rust only. Unsafe Rust is forbidden.
//!
//! # 1. Purpose
//!
//! Provides the compiler graph namespace with a stable, read-only
//! integration interface to the canonical compilation dependency graph.
//!
//! This module does not create or store another dependency graph.
//!
//! The authoritative implementation is:
//!
//!     crate::compiler::context::dependency_context
//!
//! # 2. Ownership
//!
//! Canonical graph storage and algorithms belong to:
//!
//!     DependencyContext
//!     DependencyContextBuilder
//!
//! This module owns only:
//!
//! - The compiler graph integration facade.
//! - A stable graph-facing API.
//! - Integration-boundary validation.
//! - Read-only graph queries.
//! - Access to canonical dependency ordering.
//! - Compatibility exports.
//!
//! This module does NOT own:
//!
//! - Source parsing.
//! - AST construction.
//! - Import resolution.
//! - Semantic dependency discovery.
//! - Type inference.
//! - Effect analysis.
//! - Ownership analysis.
//! - Canonical dependency identities.
//! - Canonical dependency edge definitions.
//! - Graph storage.
//! - Graph mutation algorithms.
//! - Incremental fingerprints.
//! - Cache invalidation.
//! - Parallel execution.
//! - Pipeline-stage ordering.
//! - Workload partitioning.
//! - Hardware discovery.
//! - Resource allocation.
//! - Physical scheduling.
//! - Backend selection.
//! - Runtime execution.
//!
//! # 3. Canonical ownership
//!
//! There is exactly one authoritative compilation-unit dependency graph.
//!
//!     frontend / semantic analysis
//!                 |
//!                 v
//!       DependencyContextBuilder
//!                 |
//!                 v
//!          DependencyContext
//!                 |
//!                 +--------------------+
//!                 |                    |
//!                 v                    v
//!       compiler::graph         analysis::dependency
//!                 |
//!                 +--------------------+
//!                 |                    |
//!                 v                    v
//!       parallel::dependencies   incremental integration
//!
//! The graph facade does not introduce another source of truth.
//!
//! # 4. Edge direction
//!
//! Canonical edge direction:
//!
//!     dependent -> dependency
//!
//! Example:
//!
//!     application -> library
//!
//! means that application requires library.
//!
//! Consequently:
//!
//!     library -> application
//!
//! is the dependency-first processing order.
//!
//! # 5. Relationship semantics
//!
//! Only relationships for which:
//!
//!     DependencyKind::requires_ordering()
//!
//! returns true participate in compilation ordering.
//!
//! Non-ordering relationships remain available through the canonical
//! graph and its query interfaces.
//!
//! Data-flow, feedback, synchronization and other non-ordering
//! relationships must not automatically create compilation prerequisites.
//!
//! # 6. Scalability
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Compilation units.
//! - Dependencies.
//! - Dependents.
//! - Modules.
//! - Functions.
//! - Computing domains.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed participants.
//! - Compiler stages.
//! - Operations.
//! - Memory.
//!
//! Collection capacity is determined by the canonical implementation,
//! host representation limits and available resources.
//!
//! Explicit operational budgets remain the responsibility of the
//! compilation context and configuration subsystems.
//!
//! No finite implementation promises infinite physical resources.
//!
//! # 7. Concurrency
//!
//! The canonical DependencyContext is immutable.
//!
//! Its clones share the underlying graph through Arc.
//!
//! This facade adds no locks, global state, threads or mutable caches.
//!
//! # 8. Determinism
//!
//! Deterministic ordering, traversal and graph validation are delegated
//! to the canonical DependencyContext.
//!
//! This module must not introduce randomized ordering or machine-specific
//! dependency identities.
//!
//! # 9. Grammar boundary
//!
//! Source syntax:
//!
//!     grammar/Zamani.g4
//!     grammar/modules/imports.g4
//!     grammar/declarations/domains.g4
//!
//! Semantic authority:
//!
//!     grammar/specification/semantic-model.md
//!     grammar/validation/semantic-boundaries.md
//!
//! Scalability:
//!
//!     grammar/specification/scalability-model.md
//!
//! POCO-REAF:
//!
//!     grammar/specification/poco-reaf.md
//!
//! Grammar syntax does not directly construct this graph.
//!
//! The frontend and semantic authorities establish valid relationships.
//!
//! # 10. Integration
//!
//! Canonical graph:
//!
//!     compiler::context::dependency_context
//!
//! Dependency analysis:
//!
//!     compiler::analysis::dependency
//!
//! Parallel dependency planning:
//!
//!     compiler::parallel::dependencies
//!
//! Pipeline ordering:
//!
//!     compiler::pipeline::ordering
//!
//! Workload partitioning:
//!
//!     compiler::planning::partitioning
//!
//! Incremental compilation:
//!
//!     compiler::incremental
//!
//! Each consumer retains its existing responsibility.
//!
//! # 11. Compatibility
//!
//! Existing DependencyContext and DependencyContextBuilder APIs
//! remain unchanged.
//!
//! The facade is additive and does not require existing consumers
//! to migrate immediately.

use crate::compiler::context::dependency_context::{
    CompilationUnit,
    CompilationUnitId,
    CompilationUnitKind,
    DependencyContext,
    DependencyContextBuilder,
    DependencyEdge,
    DependencyGraphError,
    DependencyKind,
};

/// Public API version of the compiler dependency graph facade.
///
/// Independent of language, grammar, semantic-model, IR and backend
/// versions.
pub const DEPENDENCY_GRAPH_API_VERSION: u16 = 1;

/// Stable compiler graph facade identity.
pub const DEPENDENCY_GRAPH_MODULE_NAME: &str =
    "zamani.compiler.graph.dependency";

// ============================================================================
// CANONICAL TYPE EXPORTS
// ============================================================================

/// Canonical compilation-unit identity.
pub use crate::compiler::context::dependency_context::CompilationUnitId
    as GraphUnitId;

/// Canonical compilation-unit classification.
pub use crate::compiler::context::dependency_context::CompilationUnitKind
    as GraphUnitKind;

/// Canonical compilation-unit description.
pub use crate::compiler::context::dependency_context::CompilationUnit
    as GraphUnit;

/// Canonical typed dependency relationship.
pub use crate::compiler::context::dependency_context::DependencyEdge
    as GraphEdge;

/// Canonical relationship classification.
pub use crate::compiler::context::dependency_context::DependencyKind
    as GraphEdgeKind;

/// Canonical graph error.
pub use crate::compiler::context::dependency_context::DependencyGraphError
    as GraphError;

/// Canonical graph builder.
///
/// The returned builder builds a DependencyContext, which remains the
/// authoritative immutable graph representation.
pub use crate::compiler::context::dependency_context::DependencyContextBuilder
    as GraphBuilder;

// ============================================================================
// GRAPH FACADE
// ============================================================================

/// Immutable compiler-facing view of the canonical dependency graph.
///
/// This type owns no independent graph storage.
///
/// Cloning this type clones the canonical DependencyContext handle.
/// The underlying immutable graph remains shared.
///
/// # Architectural invariant
///
/// Every query delegates to DependencyContext.
///
/// No graph algorithm, dependency identity, relationship definition
/// or ordering implementation is duplicated here.
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    context: DependencyContext,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::empty()
    }
}

impl DependencyGraph {
    /// Creates an empty dependency graph facade.
    ///
    /// No allocation proportional to any predefined graph capacity
    /// is performed.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            context: DependencyContext::new(),
        }
    }

    /// Creates a facade over an existing canonical dependency context.
    ///
    /// The graph is validated before it is exposed through this facade.
    ///
    /// No graph data is copied.
    ///
    /// # Errors
    ///
    /// Returns the canonical graph validation error when the graph
    /// contains invalid endpoints or ordering cycles.
    pub fn from_context(
        context: DependencyContext,
    ) -> Result<Self, DependencyGraphError> {
        context.validate()?;

        Ok(Self { context })
    }

    /// Returns the canonical dependency context.
    ///
    /// This allows existing compiler subsystems to consume the
    /// authoritative context directly.
    #[must_use]
    pub const fn context(&self) -> &DependencyContext {
        &self.context
    }

    /// Consumes the facade and returns the canonical context.
    ///
    /// No graph data is copied.
    #[must_use]
    pub fn into_context(self) -> DependencyContext {
        self.context
    }

    /// Returns the number of compilation units.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.context.unit_count()
    }

    /// Returns the number of unique typed dependency relationships.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.context.edge_count()
    }

    /// Returns whether the graph contains no compilation units.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.context.is_empty()
    }

    /// Returns a compilation unit by identity.
    #[must_use]
    pub fn unit(
        &self,
        id: &CompilationUnitId,
    ) -> Option<&CompilationUnit> {
        self.context.unit(id)
    }

    /// Returns all compilation units in deterministic identity order.
    pub fn units(
        &self,
    ) -> impl ExactSizeIterator<Item = &CompilationUnit> {
        self.context.units()
    }

    /// Returns all compilation-unit identities in deterministic order.
    pub fn unit_ids(
        &self,
    ) -> impl ExactSizeIterator<Item = &CompilationUnitId> {
        self.context.unit_ids()
    }

    /// Returns all typed dependency edges in deterministic order.
    pub fn edges(
        &self,
    ) -> impl ExactSizeIterator<Item = &DependencyEdge> {
        self.context.edges()
    }

    /// Checks whether a compilation unit exists.
    #[must_use]
    pub fn contains_unit(
        &self,
        id: &CompilationUnitId,
    ) -> bool {
        self.context.contains_unit(id)
    }

    // ========================================================================
    // DIRECT RELATIONSHIPS
    // ========================================================================

    /// Returns direct dependencies of a compilation unit.
    ///
    /// All relationship kinds are included.
    ///
    /// Returns None when the unit is unknown.
    #[must_use]
    pub fn dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.context.dependencies_of(id)
    }

    /// Returns direct dependents of a compilation unit.
    ///
    /// All relationship kinds are included.
    ///
    /// Returns None when the unit is unknown.
    #[must_use]
    pub fn dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.context.dependents_of(id)
    }

    /// Returns direct dependencies that impose compile ordering.
    #[must_use]
    pub fn ordering_dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.context.ordering_dependencies_of(id)
    }

    /// Returns direct dependents that have compile-order relationships.
    #[must_use]
    pub fn ordering_dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.context.ordering_dependents_of(id)
    }

    /// Returns all relationship kinds between two compilation units.
    ///
    /// The result is deterministically ordered.
    #[must_use]
    pub fn relationships_between(
        &self,
        dependent: &CompilationUnitId,
        dependency: &CompilationUnitId,
    ) -> Vec<DependencyKind> {
        self.context.relationships_between(
            dependent,
            dependency,
        )
    }

    /// Checks whether a particular typed edge exists.
    #[must_use]
    pub fn contains_edge(
        &self,
        edge: &DependencyEdge,
    ) -> bool {
        self.context.contains_edge(edge)
    }

    // ========================================================================
    // TRANSITIVE RELATIONSHIPS
    // ========================================================================

    /// Returns all transitive dependencies.
    ///
    /// The starting unit is excluded from the result.
    ///
    /// All relationship kinds are followed.
    ///
    /// Traversal is iterative.
    ///
    /// Returns None when the starting unit is unknown.
    pub fn transitive_dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.context.transitive_dependencies_of(id)
    }

    /// Returns all transitive dependents.
    ///
    /// The starting unit is excluded.
    ///
    /// Returns None when the starting unit is unknown.
    pub fn transitive_dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.context.transitive_dependents_of(id)
    }

    /// Returns transitive compile-order dependencies.
    ///
    /// Non-ordering relationships are excluded.
    pub fn transitive_ordering_dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.context.transitive_ordering_dependencies_of(id)
    }

    /// Returns transitive compile-order dependents.
    ///
    /// Non-ordering relationships are excluded.
    pub fn transitive_ordering_dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> Option<Vec<CompilationUnitId>> {
        self.context.transitive_ordering_dependents_of(id)
    }

    // ========================================================================
    // ORDERING
    // ========================================================================

    /// Returns deterministic dependency-first compilation order.
    ///
    /// Only ordering relationships participate.
    ///
    /// The canonical implementation uses iterative Kahn topological
    /// sorting and reports ordering cycles.
    ///
    /// # Errors
    ///
    /// Returns DependencyGraphError::OrderingCycle when an ordering
    /// cycle prevents a complete dependency-first order.
    pub fn dependency_first_order(
        &self,
    ) -> Result<Vec<CompilationUnitId>, DependencyGraphError> {
        self.context.dependency_first_order()
    }

    // ========================================================================
    // VALIDATION
    // ========================================================================

    /// Validates the canonical graph.
    ///
    /// Validation includes:
    ///
    /// - Dependency endpoint validity.
    /// - Compilation-unit registration.
    /// - Ordering-cycle detection.
    ///
    /// Non-ordering cycles are permitted.
    pub fn validate(
        &self,
    ) -> Result<(), DependencyGraphError> {
        self.context.validate()
    }

    // ========================================================================
    // BUILDER INTEGRATION
    // ========================================================================

    /// Creates a canonical builder initialized from this graph.
    ///
    /// The original graph remains immutable.
    ///
    /// Changes made through the returned builder cannot mutate
    /// the existing graph.
    #[must_use]
    pub fn to_builder(&self) -> DependencyContextBuilder {
        self.context.to_builder()
    }

    /// Returns the number of strong references to the shared
    /// canonical graph data.
    ///
    /// Informational only. Must not be used for synchronization.
    #[must_use]
    pub fn shared_reference_count(&self) -> usize {
        self.context.shared_reference_count()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_id(value: &str) -> CompilationUnitId {
        CompilationUnitId::new(value)
            .expect("valid compilation-unit identity")
    }

    fn unit(value: &str) -> CompilationUnit {
        CompilationUnit::new(
            unit_id(value),
            CompilationUnitKind::new("source_module")
                .expect("valid unit kind"),
            value,
        )
        .expect("valid compilation unit")
    }

    fn graph_with_units(
        identifiers: &[&str],
    ) -> DependencyGraph {
        let mut builder = DependencyContextBuilder::new();

        for identifier in identifiers {
            builder
                .add_unit(unit(identifier))
                .expect("unit registration");
        }

        DependencyGraph::from_context(
            builder.build().expect("canonical context"),
        )
        .expect("valid dependency graph")
    }

    #[test]
    fn empty_graph_is_valid() {
        let graph = DependencyGraph::empty();

        assert!(graph.is_empty());
        assert_eq!(graph.unit_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn graph_wraps_canonical_context_without_duplication() {
        let graph = graph_with_units(&["application"]);

        assert_eq!(graph.unit_count(), 1);
        assert!(graph.contains_unit(&unit_id("application")));

        assert_eq!(
            graph.context().unit_count(),
            graph.unit_count()
        );
    }

    #[test]
    fn dependency_direction_is_preserved() {
        let mut builder = DependencyContextBuilder::new();

        builder.add_unit(unit("application")).expect("unit");
        builder.add_unit(unit("library")).expect("unit");

        builder
            .add_dependency(
                unit_id("application"),
                unit_id("library"),
                DependencyKind::ModuleImport,
            )
            .expect("dependency");

        let graph = DependencyGraph::from_context(
            builder.build().expect("context"),
        )
        .expect("graph");

        assert_eq!(
            graph.dependencies_of(&unit_id("application")),
            Some(vec![unit_id("library")])
        );

        assert_eq!(
            graph.dependents_of(&unit_id("library")),
            Some(vec![unit_id("application")])
        );
    }

    #[test]
    fn dependency_first_order_is_correct() {
        let mut builder = DependencyContextBuilder::new();

        for name in ["application", "library", "foundation"] {
            builder.add_unit(unit(name)).expect("unit");
        }

        builder
            .add_dependency(
                unit_id("application"),
                unit_id("library"),
                DependencyKind::ModuleImport,
            )
            .expect("edge");

        builder
            .add_dependency(
                unit_id("library"),
                unit_id("foundation"),
                DependencyKind::ModuleImport,
            )
            .expect("edge");

        let graph = DependencyGraph::from_context(
            builder.build().expect("context"),
        )
        .expect("graph");

        assert_eq!(
            graph.dependency_first_order().expect("order"),
            vec![
                unit_id("foundation"),
                unit_id("library"),
                unit_id("application"),
            ]
        );
    }

    #[test]
    fn non_ordering_relationships_do_not_create_compile_prerequisites() {
        let mut builder = DependencyContextBuilder::new();

        builder.add_unit(unit("first")).expect("unit");
        builder.add_unit(unit("second")).expect("unit");

        builder
            .add_dependency(
                unit_id("first"),
                unit_id("second"),
                DependencyKind::Feedback,
            )
            .expect("edge");

        builder
            .add_dependency(
                unit_id("second"),
                unit_id("first"),
                DependencyKind::Feedback,
            )
            .expect("edge");

        let graph = DependencyGraph::from_context(
            builder.build().expect("context"),
        )
        .expect("graph");

        assert_eq!(
            graph.dependency_first_order().expect("order").len(),
            2
        );

        assert_eq!(
            graph.transitive_dependencies_of(&unit_id("first")),
            Some(vec![unit_id("second")])
        );
    }

    #[test]
    fn unknown_unit_queries_are_explicit() {
        let graph = graph_with_units(&["application"]);

        assert_eq!(
            graph.dependencies_of(&unit_id("missing")),
            None
        );

        assert_eq!(
            graph.dependents_of(&unit_id("missing")),
            None
        );
    }

    #[test]
    fn clones_share_canonical_graph_data() {
        let graph = graph_with_units(&["application"]);
        let clone = graph.clone();

        assert_eq!(
            graph.unit_count(),
            clone.unit_count()
        );

        assert!(
            clone.contains_unit(&unit_id("application"))
        );
    }

    #[test]
    fn builder_modification_does_not_mutate_original_graph() {
        let graph = graph_with_units(&["first"]);

        let mut builder = graph.to_builder();

        builder.add_unit(unit("second")).expect("unit");

        let modified = DependencyGraph::from_context(
            builder.build().expect("context"),
        )
        .expect("graph");

        assert_eq!(graph.unit_count(), 1);
        assert_eq!(modified.unit_count(), 2);
    }

    #[test]
    fn deterministic_unit_iteration_is_preserved() {
        let graph = graph_with_units(&[
            "zeta",
            "alpha",
            "middle",
        ]);

        let identifiers: Vec<String> = graph
            .unit_ids()
            .map(|id| id.as_str().to_owned())
            .collect();

        assert_eq!(
            identifiers,
            vec!["alpha", "middle", "zeta"]
        );
    }

    #[test]
    fn graph_api_version_is_available() {
        assert_eq!(DEPENDENCY_GRAPH_API_VERSION, 1);
        assert_eq!(
            DEPENDENCY_GRAPH_MODULE_NAME,
            "zamani.compiler.graph.dependency"
        );
    }
}
