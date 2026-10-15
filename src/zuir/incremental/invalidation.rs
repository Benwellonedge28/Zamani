#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Incremental Compilation — Invalidation Planning
//!
//! Path: `src/zuir/incremental/invalidation.rs`
//! Edition: Rust 2021
//! Minimum Rust baseline: Rust 1.97 or later.
//! Safety: No unsafe code.
//!
//! PURPOSE
//!
//! Build deterministic invalidation plans when compilation inputs or
//! artifacts change. The planner finds every transitive dependent of one
//! or more changed artifacts and records the graph generation against which
//! the plan was computed.
//!
//! AUTHORITY
//!
//! - `src/zuir/incremental/dependencies.rs` owns graph structure and edges.
//! - `src/zuir/fingerprint/stable_hash.rs` owns content fingerprints.
//! - `src/zuir/serialization/canonical.rs` owns canonical serialization.
//! - `src/zuir/ZUIR-SPEC.md` owns ZUIR representation semantics.
//! - `grammar/DESIGN.md` and `grammar/specification/` own language contracts.
//!
//! OWNS
//!
//! - Multi-root invalidation planning.
//! - Deterministic affected-artifact sets.
//! - Direct dependency-edge evidence for each plan.
//! - Plan generation snapshots and stale-plan detection.
//! - Explicit distinction between invalidation planning and execution.
//!
//! DOES_NOT_OWN
//!
//! - Dependency graph mutation or dependency identity.
//! - Content hashing or cache-key construction.
//! - Cache deletion, artifact rebuilding, or scheduling.
//! - Source parsing, semantic analysis, or IR verification.
//! - Target capability discovery or physical resource allocation.
//!
//! DEPENDENCY DIRECTION
//!
//! This module depends only on `dependencies.rs` and the Rust standard
//! library. It must not depend on cache storage, the pass manager,
//! serialization, domain IRs, or target backends.
//!
//! INTEGRATION
//!
//! - `incremental/mod.rs` declares and re-exports this module.
//! - `dependencies.rs` supplies the authoritative dependency graph.
//! - `cache_keys.rs` decides whether cached content matches current inputs.
//! - The incremental coordinator checks plan freshness before applying it.
//! - `pass/manager.rs` can use the plan to discard dependent analyses.
//! - Frontend and grammar changes must be registered as dependency roots.
//!
//! SEMANTIC CONTRACT
//!
//! If A depends on B, a change to B invalidates A. For several changed
//! roots, the plan is the union of their transitive dependents and the
//! roots themselves. Every artifact appears at most once.
//!
//! The returned artifact list is lexicographically ordered by
//! `DependencyKey`; it is NOT a topological execution order. Rebuild
//! scheduling must use a separate dependency-aware scheduler.
//!
//! SCALABILITY
//!
//! No universal graph-size, operation-count, hardware-count, qubit-count,
//! tensor-rank, or node-count limit is imposed. Collections grow with
//! available resources. Graph traversal is iterative and cycle-safe.
//! As with all Rust collections, allocation remains subject to finite
//! process and platform resources.
//!
//! A plan records the graph generation. A caller must compare that
//! generation with the graph's current generation before applying the
//! plan. This check is optimistic freshness detection, not synchronization.
//! Concurrent mutation requires external synchronization.
//!
//! FREEZE CRITERIA
//!
//! - Unknown roots fail explicitly.
//! - Multiple roots are deduplicated.
//! - Cycles cannot cause infinite traversal.
//! - Overlapping invalidation closures are deduplicated.
//! - Plan contents and evidence are deterministic.
//! - Stale plans can be detected.
//! - The graph is never mutated by planning.
//! - Tests and repository integration pass.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;

use super::dependencies::{
    DependencyEdge, DependencyError, DependencyGraph, DependencyKey, DependencyKind,
    DependencyResult,
};

/// One direct dependency relationship encountered during invalidation.
///
/// If `dependent` depends on `prerequisite`, changing the prerequisite
/// can invalidate the dependent. This evidence is descriptive: it does not
/// assert that the dependent's content has actually changed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidationCause {
    dependent: DependencyKey,
    prerequisite: DependencyKey,
    kind: DependencyKind,
}

impl InvalidationCause {
    /// Return the artifact that may need rebuilding.
    #[must_use]
    pub fn dependent(&self) -> &DependencyKey {
        &self.dependent
    }

    /// Return the prerequisite whose change may invalidate the dependent.
    #[must_use]
    pub fn prerequisite(&self) -> &DependencyKey {
        &self.prerequisite
    }

    /// Return the registered reason for the dependency.
    #[must_use]
    pub fn kind(&self) -> DependencyKind {
        self.kind
    }
}

impl From<&DependencyEdge> for InvalidationCause {
    fn from(edge: &DependencyEdge) -> Self {
        Self {
            dependent: edge.dependent().clone(),
            prerequisite: edge.prerequisite().clone(),
            kind: edge.kind(),
        }
    }
}

/// A deterministic invalidation plan.
///
/// `changed_roots` are the caller-supplied changed artifacts.
/// `invalidated` includes those roots and every transitive dependent.
/// `causes` contains direct edges whose prerequisite is in the affected
/// set and whose dependent is also in the affected set.
///
/// Causes are graph evidence, not a minimal explanation set: a graph may
/// have multiple valid dependency paths to the same artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidationPlan {
    graph_generation: u64,
    changed_roots: Vec<DependencyKey>,
    invalidated: Vec<DependencyKey>,
    causes: Vec<InvalidationCause>,
}

impl InvalidationPlan {
    /// Graph generation observed when the plan was created.
    ///
    /// This is an in-memory freshness token, not a persistent cache key.
    #[must_use]
    pub const fn graph_generation(&self) -> u64 {
        self.graph_generation
    }

    /// Changed roots, sorted and deduplicated.
    #[must_use]
    pub fn changed_roots(&self) -> &[DependencyKey] {
        &self.changed_roots
    }

    /// All affected artifacts, sorted and deduplicated.
    ///
    /// The roots are included because their own cached artifacts may no
    /// longer correspond to the changed inputs.
    #[must_use]
    pub fn invalidated(&self) -> &[DependencyKey] {
        &self.invalidated
    }

    /// Direct dependency relationships supporting the plan.
    ///
    /// Results are sorted and deduplicated. An edge is included only when
    /// both endpoints are in the invalidation set.
    #[must_use]
    pub fn causes(&self) -> &[InvalidationCause] {
        &self.causes
    }

    /// Check whether a particular artifact is in the plan.
    #[must_use]
    pub fn contains(&self, key: &DependencyKey) -> bool {
        self.invalidated
            .binary_search(key)
            .is_ok()
    }

    /// Number of distinct affected artifacts.
    #[must_use]
    pub fn len(&self) -> usize {
        self.invalidated.len()
    }

    /// Whether no artifacts were included.
    ///
    /// A valid plan produced from at least one known root is never empty,
    /// because the changed roots themselves are included.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.invalidated.is_empty()
    }

    /// Return whether the plan still matches the graph's current generation.
    ///
    /// Call this immediately before applying a plan while holding whatever
    /// external synchronization protects the graph and cache coordinator.
    #[must_use]
    pub fn is_current_for(&self, graph: &DependencyGraph) -> bool {
        self.graph_generation == graph.generation()
    }
}

/// Errors produced while constructing an invalidation plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidationError {
    /// The dependency graph rejected a requested lookup or validation.
    Dependency(DependencyError),

    /// Planning requires at least one changed root.
    EmptyRoots,

    /// The graph changed after a plan was built.
    StalePlan {
        planned_generation: u64,
        current_generation: u64,
    },
}

impl fmt::Display for InvalidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dependency(error) => write!(f, "invalidation dependency error: {error}"),
            Self::EmptyRoots => {
                f.write_str("invalidation planning requires at least one changed root")
            }
            Self::StalePlan {
                planned_generation,
                current_generation,
            } => write!(
                f,
                "invalidation plan is stale: planned at graph generation \
                 {planned_generation}, current generation is {current_generation}"
            ),
        }
    }
}

impl Error for InvalidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Dependency(error) => Some(error),
            Self::EmptyRoots | Self::StalePlan { .. } => None,
        }
    }
}

impl From<DependencyError> for InvalidationError {
    fn from(error: DependencyError) -> Self {
        Self::Dependency(error)
    }
}

/// Result type for invalidation planning.
pub type InvalidationResult<T> = Result<T, InvalidationError>;

/// Build invalidation plans from the authoritative dependency graph.
///
/// This type is stateless. Create it once or use its associated functions;
/// no shared mutable state or locks are required.
#[derive(Debug, Clone, Copy, Default)]
pub struct InvalidationPlanner;

impl InvalidationPlanner {
    /// Create a planner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Plan invalidation for one changed artifact.
    ///
    /// The changed artifact and all its transitive dependents are included.
    pub fn plan(
        &self,
        graph: &DependencyGraph,
        changed: &DependencyKey,
    ) -> InvalidationResult<InvalidationPlan> {
        self.plan_many(graph, std::iter::once(changed.clone()))
    }

    /// Plan invalidation for several changed artifacts.
    ///
    /// Roots are sorted and deduplicated. An unknown root is an error:
    /// treating a missing graph node as dependency-free could incorrectly
    /// reuse stale artifacts.
    ///
    /// The graph is read-only throughout planning. If callers share the
    /// graph across threads, they must prevent mutation during this call.
    pub fn plan_many<I>(
        &self,
        graph: &DependencyGraph,
        changed_roots: I,
    ) -> InvalidationResult<InvalidationPlan>
    where
        I: IntoIterator<Item = DependencyKey>,
    {
        let roots: BTreeSet<DependencyKey> = changed_roots.into_iter().collect();

        if roots.is_empty() {
            return Err(InvalidationError::EmptyRoots);
        }

        // Validate every root before returning a plan. This also ensures
        // that an unknown root cannot silently produce a partial result.
        for root in &roots {
            if !graph.contains_node(root) {
                // Reuse the graph's canonical error type and wording.
                graph.dependents_of(root)?;
            }
        }

        let mut affected = roots.clone();
        let mut pending = VecDeque::from_iter(roots.iter().cloned());

        // Traverse reverse edges: prerequisite -> dependent.
        // Mark nodes when discovered so cycles and converging paths cannot
        // cause unbounded work or duplicate queue entries.
        while let Some(changed) = pending.pop_front() {
            for edge in graph.dependents_of(&changed)? {
                let dependent = edge.dependent().clone();
                if affected.insert(dependent.clone()) {
                    pending.push_back(dependent);
                }
            }
        }

        // Gather direct graph evidence for the affected subgraph.
        // This second pass deliberately keeps planning separate from cache
        // mutation and gives consumers inspectable reasons for invalidation.
        let mut causes = BTreeSet::new();

        for dependent in &affected {
            for edge in graph.prerequisites_of(dependent)? {
                if affected.contains(edge.prerequisite()) {
                    causes.insert(InvalidationCause::from(&edge));
                }
            }
        }

        Ok(InvalidationPlan {
            graph_generation: graph.generation(),
            changed_roots: roots.into_iter().collect(),
            invalidated: affected.into_iter().collect(),
            causes: causes.into_iter().collect(),
        })
    }

    /// Reject a plan if its graph generation no longer matches.
    ///
    /// This is a check, not a lock. The caller must hold external
    /// synchronization across freshness validation and plan application
    /// if concurrent graph mutation is possible.
    pub fn ensure_current(
        &self,
        graph: &DependencyGraph,
        plan: &InvalidationPlan,
    ) -> InvalidationResult<()> {
        let current_generation = graph.generation();

        if plan.graph_generation != current_generation {
            return Err(InvalidationError::StalePlan {
                planned_generation: plan.graph_generation,
                current_generation,
            });
        }

        Ok(())
    }

    /// Build a plan and verify graph invariants before returning it.
    ///
    /// This is useful at trust boundaries, after deserializing graph data,
    /// or in diagnostic builds. It performs a full graph validation, so
    /// callers should not invoke it after every small graph mutation on
    /// very large graphs unless that cost is acceptable.
    pub fn plan_many_checked<I>(
        &self,
        graph: &DependencyGraph,
        changed_roots: I,
    ) -> InvalidationResult<InvalidationPlan>
    where
        I: IntoIterator<Item = DependencyKey>,
    {
        graph.validate()?;
        self.plan_many(graph, changed_roots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::incremental::dependencies::{
        DependencyGraph, DependencyKey, DependencyKind,
    };

    fn key(value: &str) -> DependencyKey {
        DependencyKey::new(value).expect("test key must be valid")
    }

    fn edge(
        graph: &mut DependencyGraph,
        dependent: &str,
        prerequisite: &str,
        kind: DependencyKind,
    ) {
        graph
            .add_dependency(key(dependent), key(prerequisite), kind)
            .expect("test dependency insertion must succeed");
    }

    #[test]
    fn includes_root_and_all_transitive_dependents() {
        let mut graph = DependencyGraph::new();
        edge(&mut graph, "semantic", "source", DependencyKind::Source);
        edge(&mut graph, "zuir", "semantic", DependencyKind::Semantic);
        edge(&mut graph, "optimized", "zuir", DependencyKind::Transformation);

        let plan = InvalidationPlanner::new()
            .plan(&graph, &key("source"))
            .expect("planning must succeed");

        assert_eq!(
            plan.invalidated(),
            &[key("optimized"), key("semantic"), key("source"), key("zuir")]
        );
        assert!(plan.contains(&key("source")));
        assert!(plan.contains(&key("optimized")));
    }

    #[test]
    fn supports_multiple_roots_and_overlapping_closures() {
        let mut graph = DependencyGraph::new();
        edge(&mut graph, "middle", "source_a", DependencyKind::Source);
        edge(&mut graph, "middle", "source_b", DependencyKind::Source);
        edge(&mut graph, "output", "middle", DependencyKind::Transformation);

        let planner = InvalidationPlanner::new();
        let plan = planner
            .plan_many(&graph, [key("source_b"), key("source_a"), key("source_a")])
            .expect("planning must succeed");

        assert_eq!(plan.changed_roots(), &[key("source_a"), key("source_b")]);
        assert_eq!(
            plan.invalidated(),
            &[key("middle"), key("output"), key("source_a"), key("source_b")]
        );
    }

    #[test]
    fn handles_cycles_without_infinite_traversal() {
        let mut graph = DependencyGraph::new();
        edge(&mut graph, "a", "b", DependencyKind::Semantic);
        edge(&mut graph, "b", "a", DependencyKind::Semantic);
        edge(&mut graph, "c", "b", DependencyKind::Transformation);

        let plan = InvalidationPlanner::new()
            .plan(&graph, &key("a"))
            .expect("cycle-safe planning must succeed");

        assert_eq!(plan.invalidated(), &[key("a"), key("b"), key("c")]);
    }

    #[test]
    fn preserves_all_dependency_reasons_as_evidence() {
        let mut graph = DependencyGraph::new();
        edge(&mut graph, "zuir", "source", DependencyKind::Source);
        edge(&mut graph, "zuir", "source", DependencyKind::Grammar);

        let plan = InvalidationPlanner::new()
            .plan(&graph, &key("source"))
            .expect("planning must succeed");

        assert_eq!(plan.causes().len(), 2);
        assert!(plan.causes().iter().any(|cause| {
            cause.dependent() == &key("zuir")
                && cause.prerequisite() == &key("source")
                && cause.kind() == DependencyKind::Grammar
        }));
    }

    #[test]
    fn rejects_unknown_roots() {
        let graph = DependencyGraph::new();

        let error = InvalidationPlanner::new()
            .plan(&graph, &key("missing"))
            .expect_err("unknown roots must not be accepted");

        assert!(matches!(
            error,
            InvalidationError::Dependency(DependencyError::UnknownNode { .. })
        ));
    }

    #[test]
    fn rejects_empty_root_set() {
        let graph = DependencyGraph::new();

        let error = InvalidationPlanner::new()
            .plan_many(&graph, std::iter::empty())
            .expect_err("empty root set must be rejected");

        assert_eq!(error, InvalidationError::EmptyRoots);
    }

    #[test]
    fn detects_stale_plan_after_graph_mutation() {
        let mut graph = DependencyGraph::new();
        graph.add_node(key("source")).expect("node insertion");

        let planner = InvalidationPlanner::new();
        let plan = planner
            .plan(&graph, &key("source"))
            .expect("planning must succeed");

        graph.add_node(key("other")).expect("graph mutation");

        let error = planner
            .ensure_current(&graph, &plan)
            .expect_err("plan must become stale");

        assert!(matches!(error, InvalidationError::StalePlan { .. }));
    }

    #[test]
    fn unchanged_graph_keeps_plan_current() {
        let mut graph = DependencyGraph::new();
        graph.add_node(key("source")).expect("node insertion");

        let planner = InvalidationPlanner::new();
        let plan = planner
            .plan(&graph, &key("source"))
            .expect("planning must succeed");

        assert!(plan.is_current_for(&graph));
        planner
            .ensure_current(&graph, &plan)
            .expect("unchanged graph should accept plan");
    }

    #[test]
    fn plan_is_deterministic() {
        let mut graph = DependencyGraph::new();
        edge(&mut graph, "z", "source", DependencyKind::Source);
        edge(&mut graph, "a", "source", DependencyKind::Source);
        edge(&mut graph, "m", "a", DependencyKind::Transformation);
        edge(&mut graph, "m", "z", DependencyKind::Analysis);

        let planner = InvalidationPlanner::new();
        let first = planner
            .plan(&graph, &key("source"))
            .expect("first plan");
        let second = planner
            .plan(&graph, &key("source"))
            .expect("second plan");

        assert_eq!(first, second);
    }

    #[test]
    fn checked_planning_validates_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_node(key("source")).expect("node insertion");

        let plan = InvalidationPlanner::new()
            .plan_many_checked(&graph, [key("source")])
            .expect("valid graph should produce a plan");

        assert_eq!(plan.invalidated(), &[key("source")]);
    }

    #[test]
    fn independent_artifacts_are_not_invalidated() {
        let mut graph = DependencyGraph::new();
        edge(&mut graph, "dependent", "source", DependencyKind::Source);
        graph.add_node(key("unrelated")).expect("node insertion");

        let plan = InvalidationPlanner::new()
            .plan(&graph, &key("source"))
            .expect("planning must succeed");

        assert!(!plan.contains(&key("unrelated")));
        assert_eq!(plan.invalidated(), &[key("dependent"), key("source")]);
    }
}