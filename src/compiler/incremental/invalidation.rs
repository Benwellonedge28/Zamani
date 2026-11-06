#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Incremental Invalidation
//!
//! Path: src/compiler/incremental/invalidation.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Provide the compiler-facing invalidation-planning boundary for
//! incremental compilation.
//!
//! This module adapts the compiler's incremental-compilation workflow
//! to the authoritative ZUIR dependency graph and invalidation planner.
//!
//! It allows compiler orchestration to:
//!
//! - Plan invalidation for one changed compilation artifact.
//! - Plan invalidation for multiple changed artifacts.
//! - Determine all transitive dependents of changed artifacts.
//! - Obtain deterministic invalidation evidence.
//! - Detect stale plans after dependency-graph mutations.
//! - Plan invalidation before removing compilation artifacts.
//! - Validate graph consistency at explicit trust boundaries.
//!
//! ARCHITECTURAL AUTHORITIES
//!
//! src/zuir/incremental/dependencies.rs
//!     Owns logical dependency identities, dependency edges,
//!     graph mutation, reverse dependencies, and graph traversal.
//!
//! src/zuir/incremental/invalidation.rs
//!     Owns the canonical invalidation algorithm, invalidation plans,
//!     direct dependency evidence, and graph-generation validation.
//!
//! src/zuir/incremental/cache_keys.rs
//!     Owns canonical ZUIR cache-key construction.
//!
//! src/zuir/fingerprint/stable_hash.rs
//!     Owns canonical stable fingerprint construction.
//!
//! src/compiler/incremental/fingerprints.rs
//!     Owns compiler-specific fingerprint composition.
//!
//! src/compiler/incremental/cache.rs
//!     Owns compiler-facing cache integration.
//!
//! src/frontend/semantic/incremental/
//!     Owns semantic-fact dependency tracking and semantic invalidation.
//!
//! src/compiler/context/dependency_context.rs
//!     Owns immutable compilation-unit dependency snapshots,
//!     dependency validation, and compilation ordering.
//!
//! src/compiler/parallel_build.rs
//!     Owns parallel execution of compilation jobs.
//!
//! OWNERSHIP
//!
//! THIS MODULE OWNS:
//!
//! - The compiler-facing invalidation-planning API.
//! - Compiler-facing naming and documentation.
//! - Adaptation of compiler invalidation requests to the canonical
//!   ZUIR invalidation planner.
//! - Explicit invalidation planning for removed compilation artifacts.
//! - Compiler-level integration tests.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Dependency identities.
//! - Dependency graph construction.
//! - Dependency graph mutation.
//! - Dependency discovery.
//! - Source parsing.
//! - Grammar interpretation.
//! - Semantic dependency discovery.
//! - Semantic-fact invalidation.
//! - Content fingerprinting.
//! - Cache-key construction.
//! - Cache storage or eviction.
//! - Artifact deletion.
//! - Artifact rebuilding.
//! - Compilation scheduling.
//! - Parallel execution.
//! - IR definitions.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime execution.
//!
//! DEPENDENCY DIRECTION
//!
//! Frontend / semantic dependency discovery
//!                 |
//!                 v
//!       DependencyGraph construction
//!                 |
//!                 v
//!       Compiler dependency context
//!                 |
//!                 v
//!       Compiler invalidation API
//!                 |
//!                 v
//!       Canonical ZUIR invalidation planner
//!                 |
//!                 v
//!       Deterministic invalidation plan
//!                 |
//!          +------+-------+
//!          |              |
//!          v              v
//!     Cache handling   Rebuild planning
//!          |              |
//!          v              v
//!   compiler cache   dependency-aware build
//!                         |
//!                         v
//!                  parallel_build.rs
//!
//! DEPENDENCY DIRECTION RULE
//!
//! This module depends on the canonical ZUIR incremental subsystem.
//! The ZUIR incremental subsystem must not depend on this compiler module.
//!
//! INVALIDATION SEMANTICS
//!
//! If artifact A depends on artifact B, a change to B may invalidate A.
//!
//! For a set of changed roots, the invalidation plan contains:
//!
//! - Every explicitly changed root.
//! - Every transitive dependent of those roots.
//! - Direct dependency-edge evidence supporting the plan.
//!
//! Every artifact appears at most once.
//!
//! The canonical ZUIR planner handles cycles and overlapping dependency
//! paths without unbounded traversal.
//!
//! PLAN ORDERING
//!
//! The invalidated artifact list is deterministic and sorted by logical
//! dependency key.
//!
//! It is NOT a dependency-first compilation order.
//!
//! Rebuild ordering must be obtained from the authoritative compilation
//! dependency context or another explicitly dependency-aware scheduler.
//!
//! This module must never treat lexical key ordering as a valid substitute
//! for compilation ordering.
//!
//! GRAPH FRESHNESS
//!
//! Every invalidation plan records the dependency-graph generation
//! against which it was created.
//!
//! Before applying a plan, the caller must validate that generation.
//!
//! This is optimistic freshness detection, not synchronization.
//!
//! If dependency graph mutation can occur concurrently, the caller must
//! hold the appropriate external synchronization across:
//!
//! 1. Plan freshness validation.
//! 2. Cache invalidation.
//! 3. Related graph changes.
//!
//! This module does not introduce a second graph lock.
//!
//! REMOVED ARTIFACTS
//!
//! A removed artifact must be invalidated while its dependency relationships
//! are still available.
//!
//! The required order is:
//!
//! 1. Build an invalidation plan for the existing artifact.
//! 2. Validate the plan immediately before applying it.
//! 3. Invalidate affected cached artifacts.
//! 4. Remove the artifact from the dependency graph.
//! 5. Update the authoritative dependency context.
//!
//! Removing the graph node before planning may destroy the reverse-edge
//! information required to invalidate its dependents.
//!
//! This module only creates the plan. It does not mutate the graph or cache.
//!
//! CACHE INTEGRATION
//!
//! The compiler coordinator consumes the invalidation plan and applies
//! invalidation to the compiler cache.
//!
//! Cache storage remains owned by backend/cache.rs.
//!
//! Semantic cache invalidation remains owned by the semantic incremental
//! subsystem.
//!
//! ZUIR artifact invalidation remains represented by canonical ZUIR
//! dependency identities.
//!
//! Cache invalidation must not be confused with deleting dependency edges.
//!
//! FINGERPRINT INTEGRATION
//!
//! Fingerprints identify content and compilation inputs.
//!
//! Dependency keys identify logical artifacts.
//!
//! Invalidation plans identify artifacts that may no longer be reusable.
//!
//! These are distinct responsibilities.
//!
//! This module does not calculate or modify fingerprints.
//!
//! SCALABILITY
//!
//! This module introduces no fixed limits on:
//!
//! - Compilation artifacts.
//! - Dependency roots.
//! - Dependency edges.
//! - Transitive dependents.
//! - Compilation units.
//! - Modules.
//! - Functions.
//! - Computing domains.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - Quantum processors.
//! - Distributed nodes.
//! - Tensor dimensions.
//! - IR operations.
//!
//! The underlying graph and invalidation algorithms use dynamically
//! growing collections and iterative traversal.
//!
//! Actual memory and execution limits are determined by available
//! resources and applicable configurable compilation budgets.
//!
//! Literal infinite physical capacity is not an implementation guarantee.
//!
//! DETERMINISM
//!
//! This module relies on the canonical ZUIR planner for deterministic
//! ordering and invalidation evidence.
//!
//! It must not introduce randomized iteration, wall-clock ordering,
//! pointer-based identity, or machine-specific invalidation behavior.
//!
//! ERROR HANDLING
//!
//! Errors from the canonical dependency graph and invalidation planner
//! are propagated without being silently ignored.
//!
//! In particular:
//!
//! - Unknown changed roots are errors.
//! - Empty changed-root collections are errors.
//! - Stale plans are rejected.
//! - Dependency graph invariant violations are reported.
//! - Generation exhaustion is reported by the graph owner.
//!
//! No partial invalidation plan is returned after a planning failure.
//!
//! SECURITY
//!
//! This module does not deserialize or trust externally supplied
//! invalidation plans.
//!
//! A persisted or externally received dependency graph must be validated
//! by its authoritative graph owner before being used for planning.
//!
//! INTEGRATION CONTRACT
//!
//! The compiler coordinator must:
//!
//! 1. Obtain the authoritative dependency graph.
//! 2. Identify changed logical artifacts.
//! 3. Call this module's planner.
//! 4. Inspect the returned invalidation plan.
//! 5. Validate graph freshness immediately before applying the plan.
//! 6. Invalidate affected compiler cache entries.
//! 7. Schedule rebuilding using dependency-aware ordering.
//! 8. Publish new valid artifacts only after successful verification.
//!
//! FREEZE CRITERIA
//!
//! - No duplicate dependency graph exists.
//! - No duplicate invalidation algorithm exists.
//! - All changed roots are validated.
//! - Transitive dependents are included.
//! - Cycles cannot cause unbounded traversal.
//! - Multiple roots are handled deterministically.
//! - Stale plans are rejected.
//! - Removed artifacts can be planned before graph removal.
//! - The graph remains unchanged during planning.
//! - No fixed hardware or graph-size ceilings are introduced.
//! - Unit and integration tests pass.
//! - Compiler module declarations expose the implementation correctly.

use std::error::Error;

use crate::zuir::incremental::{
    DependencyGraph,
    DependencyKey,
    InvalidationCause,
    InvalidationError,
    InvalidationPlan,
    InvalidationPlanner,
    InvalidationResult,
};

// ============================================================================
// PUBLIC COMPILER API
// ============================================================================

/// Compiler-facing invalidation plan.
///
/// This is the canonical ZUIR invalidation plan, not a second plan type.
///
/// The alias preserves one authoritative representation across compiler
/// and ZUIR incremental compilation.
pub type CompilerInvalidationPlan = InvalidationPlan;

/// Compiler-facing invalidation cause.
///
/// The canonical ZUIR type retains the dependency-edge evidence.
pub type CompilerInvalidationCause = InvalidationCause;

/// Compiler-facing invalidation error.
pub type CompilerInvalidationError = InvalidationError;

/// Compiler-facing invalidation result.
pub type CompilerInvalidationResult<T> = InvalidationResult<T>;

// ============================================================================
// COMPILER INVALIDATION PLANNER
// ============================================================================

/// Stateless compiler-facing adapter for canonical invalidation planning.
///
/// This type intentionally contains no dependency graph, cache, lock,
/// fingerprint state, or mutable compilation state.
///
/// The authoritative graph is supplied by the caller.
///
/// Cloning or sharing this planner does not duplicate graph state.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompilerInvalidationPlanner {
    canonical: InvalidationPlanner,
}

impl CompilerInvalidationPlanner {
    /// Create a compiler invalidation planner.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            canonical: InvalidationPlanner::new(),
        }
    }

    /// Plan invalidation for one changed artifact.
    ///
    /// The changed artifact and all of its transitive dependents are
    /// included in the returned plan.
    ///
    /// The graph is not modified.
    ///
    /// # Errors
    ///
    /// Returns an error if the changed artifact is not registered
    /// in the supplied dependency graph.
    pub fn plan_changed(
        &self,
        graph: &DependencyGraph,
        changed: &DependencyKey,
    ) -> CompilerInvalidationResult<CompilerInvalidationPlan> {
        self.canonical.plan(graph, changed)
    }

    /// Plan invalidation for multiple changed artifacts.
    ///
    /// The canonical planner:
    ///
    /// - Deduplicates changed roots.
    /// - Validates every root.
    /// - Includes every root in the result.
    /// - Includes all transitive dependents.
    /// - Handles dependency cycles.
    /// - Produces deterministic output.
    ///
    /// The input iterator is consumed exactly once.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - No changed roots are supplied.
    /// - Any changed root is unknown.
    /// - The canonical dependency graph reports an error.
    pub fn plan_changed_many<I>(
        &self,
        graph: &DependencyGraph,
        changed_roots: I,
    ) -> CompilerInvalidationResult<CompilerInvalidationPlan>
    where
        I: IntoIterator<Item = DependencyKey>,
    {
        self.canonical.plan_many(graph, changed_roots)
    }

    /// Plan invalidation for a compilation artifact that is about to
    /// be removed.
    ///
    /// This operation must be called before removing the artifact from
    /// the dependency graph.
    ///
    /// The returned plan includes the artifact and its transitive
    /// dependents while their dependency relationships still exist.
    ///
    /// This method does not remove the artifact or invalidate any cache.
    ///
    /// The caller is responsible for applying the plan, then removing
    /// the artifact from the graph under the appropriate synchronization.
    ///
    /// # Errors
    ///
    /// Returns an error if the artifact is not currently registered.
    pub fn plan_removal(
        &self,
        graph: &DependencyGraph,
        artifact: &DependencyKey,
    ) -> CompilerInvalidationResult<CompilerInvalidationPlan> {
        self.plan_changed(graph, artifact)
    }

    /// Plan invalidation for several changed artifacts and validate
    /// the dependency graph's internal invariants first.
    ///
    /// This is intended for trust boundaries, diagnostic builds,
    /// graph restoration, and explicit consistency checks.
    ///
    /// Full graph validation may be more expensive than ordinary
    /// incremental planning.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The graph contains inconsistent forward/reverse edges.
    /// - No changed roots are supplied.
    /// - Any changed root is unknown.
    /// - The canonical planner reports an error.
    pub fn plan_changed_many_checked<I>(
        &self,
        graph: &DependencyGraph,
        changed_roots: I,
    ) -> CompilerInvalidationResult<CompilerInvalidationPlan>
    where
        I: IntoIterator<Item = DependencyKey>,
    {
        self.canonical
            .plan_many_checked(graph, changed_roots)
    }

    /// Verify that an invalidation plan still matches the current
    /// dependency graph generation.
    ///
    /// This must be performed immediately before applying a plan.
    ///
    /// It does not acquire a lock and does not prevent another thread
    /// from mutating the graph after the check.
    ///
    /// Callers requiring atomic freshness validation and cache mutation
    /// must provide external synchronization.
    ///
    /// # Errors
    ///
    /// Returns a stale-plan error when the graph has changed since
    /// the plan was created.
    pub fn ensure_current(
        &self,
        graph: &DependencyGraph,
        plan: &CompilerInvalidationPlan,
    ) -> CompilerInvalidationResult<()> {
        self.canonical.ensure_current(graph, plan)
    }

    /// Return whether an invalidation plan still matches the graph.
    ///
    /// This is a convenience query. It is not a substitute for
    /// `ensure_current` immediately before applying a plan.
    #[must_use]
    pub fn is_current(
        &self,
        graph: &DependencyGraph,
        plan: &CompilerInvalidationPlan,
    ) -> bool {
        plan.is_current_for(graph)
    }
}

// ============================================================================
// DEFAULT CONSTRUCTION
// ============================================================================

impl From<InvalidationPlanner> for CompilerInvalidationPlanner {
    fn from(canonical: InvalidationPlanner) -> Self {
        Self { canonical }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(value: &str) -> DependencyKey {
        DependencyKey::new(value)
            .expect("test dependency key must be valid")
    }

    fn add_dependency(
        graph: &mut DependencyGraph,
        dependent: &str,
        prerequisite: &str,
    ) {
        graph
            .add_dependency(
                key(dependent),
                key(prerequisite),
                crate::zuir::incremental::DependencyKind::Transformation,
            )
            .expect("test dependency insertion must succeed");
    }

    #[test]
    fn changed_prerequisite_invalidates_transitive_dependents() {
        let mut graph = DependencyGraph::new();

        add_dependency(&mut graph, "application", "library");
        add_dependency(&mut graph, "library", "foundation");

        let planner = CompilerInvalidationPlanner::new();

        let plan = planner
            .plan_changed(&graph, &key("foundation"))
            .expect("invalidation planning must succeed");

        assert_eq!(
            plan.invalidated(),
            &[
                key("application"),
                key("foundation"),
                key("library"),
            ]
        );

        assert_eq!(plan.changed_roots(), &[key("foundation")]);
        assert_eq!(plan.len(), 3);
        assert!(!plan.is_empty());
    }

    #[test]
    fn multiple_changed_roots_are_deduplicated() {
        let mut graph = DependencyGraph::new();

        add_dependency(&mut graph, "application", "library");
        add_dependency(&mut graph, "application", "runtime");

        let planner = CompilerInvalidationPlanner::new();

        let plan = planner
            .plan_changed_many(
                &graph,
                vec![
                    key("library"),
                    key("runtime"),
                    key("library"),
                ],
            )
            .expect("multi-root invalidation must succeed");

        assert_eq!(
            plan.changed_roots(),
            &[key("library"), key("runtime")]
        );

        assert_eq!(
            plan.invalidated(),
            &[
                key("application"),
                key("library"),
                key("runtime"),
            ]
        );

        assert_eq!(plan.len(), 3);
    }

    #[test]
    fn overlapping_dependency_paths_do_not_duplicate_artifacts() {
        let mut graph = DependencyGraph::new();

        add_dependency(&mut graph, "application", "library_a");
        add_dependency(&mut graph, "application", "library_b");
        add_dependency(&mut graph, "library_a", "foundation");
        add_dependency(&mut graph, "library_b", "foundation");

        let planner = CompilerInvalidationPlanner::new();

        let plan = planner
            .plan_changed(&graph, &key("foundation"))
            .expect("invalidation planning must succeed");

        assert_eq!(
            plan.invalidated(),
            &[
                key("application"),
                key("foundation"),
                key("library_a"),
                key("library_b"),
            ]
        );

        assert_eq!(plan.len(), 4);
    }

    #[test]
    fn dependency_cycles_do_not_cause_unbounded_traversal() {
        let mut graph = DependencyGraph::new();

        add_dependency(&mut graph, "a", "b");
        add_dependency(&mut graph, "b", "c");
        add_dependency(&mut graph, "c", "a");

        let planner = CompilerInvalidationPlanner::new();

        let plan = planner
            .plan_changed(&graph, &key("a"))
            .expect("cycle-safe invalidation must succeed");

        assert_eq!(
            plan.invalidated(),
            &[key("a"), key("b"), key("c")]
        );

        assert_eq!(plan.len(), 3);
    }

    #[test]
    fn unknown_changed_artifact_is_rejected() {
        let graph = DependencyGraph::new();
        let planner = CompilerInvalidationPlanner::new();

        let result = planner.plan_changed(&graph, &key("missing"));

        assert!(result.is_err());
    }

    #[test]
    fn empty_changed_root_collection_is_rejected() {
        let graph = DependencyGraph::new();
        let planner = CompilerInvalidationPlanner::new();

        let result = planner.plan_changed_many(
            &graph,
            Vec::<DependencyKey>::new(),
        );

        assert!(matches!(
            result,
            Err(InvalidationError::EmptyRoots)
        ));
    }

    #[test]
    fn graph_is_not_modified_by_invalidation_planning() {
        let mut graph = DependencyGraph::new();

        add_dependency(&mut graph, "application", "library");

        let generation_before = graph.generation();
        let node_count_before = graph.node_count();
        let edge_count_before = graph.edge_count();

        let planner = CompilerInvalidationPlanner::new();

        let _plan = planner
            .plan_changed(&graph, &key("library"))
            .expect("invalidation planning must succeed");

        assert_eq!(graph.generation(), generation_before);
        assert_eq!(graph.node_count(), node_count_before);
        assert_eq!(graph.edge_count(), edge_count_before);
    }

    #[test]
    fn stale_plans_are_rejected_after_graph_mutation() {
        let mut graph = DependencyGraph::new();

        add_dependency(&mut graph, "application", "library");

        let planner = CompilerInvalidationPlanner::new();

        let plan = planner
            .plan_changed(&graph, &key("library"))
            .expect("invalidation planning must succeed");

        assert!(planner.is_current(&graph, &plan));

        graph
            .add_dependency(
                key("application"),
                key("runtime"),
                crate::zuir::incremental::DependencyKind::Configuration,
            )
            .expect("graph mutation must succeed");

        assert!(!planner.is_current(&graph, &plan));

        assert!(matches!(
            planner.ensure_current(&graph, &plan),
            Err(InvalidationError::StalePlan { .. })
        ));
    }

    #[test]
    fn removal_is_planned_before_the_graph_node_is_removed() {
        let mut graph = DependencyGraph::new();

        add_dependency(&mut graph, "application", "library");

        let planner = CompilerInvalidationPlanner::new();

        let plan = planner
            .plan_removal(&graph, &key("library"))
            .expect("removal invalidation planning must succeed");

        assert!(plan.contains(&key("library")));
        assert!(plan.contains(&key("application")));

        // Planning must not remove the artifact or its relationships.
        assert!(graph.contains_node(&key("library")));
        assert!(graph.contains_node(&key("application")));

        // The caller may remove the graph node only after applying the plan.
        planner
            .ensure_current(&graph, &plan)
            .expect("plan must remain current before application");

        graph
            .remove_node(&key("library"))
            .expect("graph removal must succeed");

        assert!(!graph.contains_node(&key("library")));
    }

    #[test]
    fn checked_planning_validates_graph_and_returns_a_plan() {
        let mut graph = DependencyGraph::new();

        add_dependency(&mut graph, "application", "library");

        let planner = CompilerInvalidationPlanner::new();

        let plan = planner
            .plan_changed_many_checked(
                &graph,
                vec![key("library")],
            )
            .expect("checked planning must succeed");

        assert_eq!(
            plan.invalidated(),
            &[key("application"), key("library")]
        );
    }

    #[test]
    fn invalidation_evidence_is_available_to_compiler_consumers() {
        let mut graph = DependencyGraph::new();

        add_dependency(&mut graph, "application", "library");

        let planner = CompilerInvalidationPlanner::new();

        let plan = planner
            .plan_changed(&graph, &key("library"))
            .expect("invalidation planning must succeed");

        assert!(
            plan.causes().iter().any(|cause| {
                cause.dependent() == &key("application")
                    && cause.prerequisite() == &key("library")
            })
        );
    }

    #[test]
    fn current_plan_has_no_stale_graph_generation() {
        let mut graph = DependencyGraph::new();

        graph
            .add_node(key("source"))
            .expect("node insertion must succeed");

        let planner = CompilerInvalidationPlanner::new();

        let plan = planner
            .plan_changed(&graph, &key("source"))
            .expect("planning must succeed");

        assert!(plan.is_current_for(&graph));
        assert!(planner.is_current(&graph, &plan));
    }
}