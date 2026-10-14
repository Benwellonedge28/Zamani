
#![forbid(unsafe_code)]

//! # Incremental Semantic Dependency Graph
//!
//! Path: src/frontend/semantic/incremental/dependencies.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Track dependencies between semantic-analysis entities to support precise
//! invalidation, deterministic analysis scheduling, and incremental
//! recomputation.
//!
//! ## Ownership
//!
//! - AST identities belong to `frontend::ast::node::node_id::NodeId`.
//! - This module owns only semantic dependency relationships.
//! - The semantic model owns semantic facts.
//! - The parser and grammar own syntax.
//! - Downstream IRs own their respective computation representations.
//! - Target planning owns hardware feasibility and resource allocation.
//!
//! ## Edge convention
//!
//! `dependent -> prerequisite` means that the dependent fact relies on the
//! prerequisite fact.
//!
//! Invalidation follows reverse edges, from changed prerequisites to their
//! dependents. Topological ordering returns prerequisites first.
//!
//! ## Scalability
//!
//! The graph uses sparse ordered collections. Traversals are iterative;
//! there are no recursive graph walks or fixed language-level graph-size
//! ceilings. Actual operation remains bounded by available memory, time,
//! address space, and representational limits.
//!
//! Operational budgets, when needed, belong to the semantic session.
//! Exhausting a budget must not be reported as successful complete analysis.
//!
//! ## Integration
//!
//! - `incremental/mod.rs` declares and exports this module.
//! - The incremental coordinator owns a `DependencyGraph`.
//! - Semantic passes register the prerequisites they actually read.
//! - Source or semantic-fact changes trigger `invalidate_from`.
//! - The coordinator recomputes affected facts in `topological_order_for`
//!   order.
//! - This module does not inspect AST node variants or grammar productions.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::node_id::NodeId;

/// Sparse, deterministic semantic dependency graph.
///
/// The maps are inverses:
///
/// - `prerequisites[A]` contains the facts read by A.
/// - `dependents[B]` contains the facts that read B.
///
/// Isolated nodes are retained until explicitly removed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DependencyGraph {
    prerequisites: BTreeMap<NodeId, BTreeSet<NodeId>>,
    dependents: BTreeMap<NodeId, BTreeSet<NodeId>>,
}

/// Errors reported by dependency graph operations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DependencyGraphError {
    /// Adding this edge would create a cycle.
    Cycle {
        dependent: NodeId,
        prerequisite: NodeId,
        /// Deterministic path from the proposed prerequisite to the dependent.
        path: Vec<NodeId>,
    },

    /// The graph contains a cycle or an invalid internal relationship.
    ///
    /// Normal mutation methods maintain the graph invariants. This variant
    /// provides a defensive error for validation and future deserialization.
    InvalidGraph {
        nodes: Vec<NodeId>,
        reason: &'static str,
    },
}

impl fmt::Display for DependencyGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cycle {
                dependent,
                prerequisite,
                path,
            } => write!(
                f,
                "dependency from {:?} to {:?} would create a cycle; path: {:?}",
                dependent, prerequisite, path
            ),
            Self::InvalidGraph { nodes, reason } => {
                write!(f, "invalid dependency graph: {reason}; nodes: {nodes:?}")
            }
        }
    }
}

impl Error for DependencyGraphError {}

impl DependencyGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of tracked nodes, including isolated nodes.
    pub fn node_count(&self) -> usize {
        self.prerequisites.len()
    }

    /// Number of distinct edges.
    pub fn edge_count(&self) -> usize {
        self.prerequisites.values().map(BTreeSet::len).sum()
    }

    /// Whether a node is tracked.
    pub fn contains_node(&self, node: NodeId) -> bool {
        self.prerequisites.contains_key(&node)
    }

    /// Register a node without adding dependencies.
    ///
    /// Registration is idempotent.
    pub fn track_node(&mut self, node: NodeId) {
        self.prerequisites.entry(node).or_default();
        self.dependents.entry(node).or_default();
    }

    /// Direct prerequisites in deterministic order.
    pub fn prerequisites_of(&self, node: NodeId) -> Vec<NodeId> {
        self.prerequisites
            .get(&node)
            .map(|nodes| nodes.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Direct dependents in deterministic order.
    pub fn dependents_of(&self, node: NodeId) -> Vec<NodeId> {
        self.dependents
            .get(&node)
            .map(|nodes| nodes.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Record that `dependent` relies on `prerequisite`.
    ///
    /// Returns `true` if a new edge was inserted, or `false` if it already
    /// existed. Rejects cycles without changing the graph.
    pub fn add_dependency(
        &mut self,
        dependent: NodeId,
        prerequisite: NodeId,
    ) -> Result<bool, DependencyGraphError> {
        if self
            .prerequisites
            .get(&dependent)
            .is_some_and(|nodes| nodes.contains(&prerequisite))
        {
            return Ok(false);
        }

        // A new edge dependent -> prerequisite creates a cycle precisely
        // when prerequisite already reaches dependent through prerequisite
        // edges. Check before registering either node, so failure is atomic.
        if let Some(path) =
            self.path_through_prerequisites(prerequisite, dependent)
        {
            return Err(DependencyGraphError::Cycle {
                dependent,
                prerequisite,
                path,
            });
        }

        self.insert_edge(dependent, prerequisite);
        Ok(true)
    }

    /// Atomically replace all direct prerequisites of `dependent`.
    ///
    /// Duplicate inputs are deduplicated. On error, both graph maps and the
    /// set of tracked nodes are restored to their previous state.
    pub fn replace_dependencies<I>(
        &mut self,
        dependent: NodeId,
        prerequisites: I,
    ) -> Result<bool, DependencyGraphError>
    where
        I: IntoIterator<Item = NodeId>,
    {
        let replacement: BTreeSet<NodeId> =
            prerequisites.into_iter().collect();

        let previous = self
            .prerequisites
            .get(&dependent)
            .cloned()
            .unwrap_or_default();

        if self.contains_node(dependent) && previous == replacement {
            return Ok(false);
        }

        // Preserve node membership as well as edges. Failed replacement must
        // not leave newly introduced, isolated nodes behind.
        let original_nodes: BTreeSet<NodeId> =
            self.prerequisites.keys().copied().collect();

        self.track_node(dependent);

        for prerequisite in previous.iter().copied() {
            self.remove_edge(dependent, prerequisite);
        }

        for prerequisite in replacement.iter().copied() {
            if let Err(error) = self.add_dependency(dependent, prerequisite) {
                // Remove all partially installed replacement edges.
                for installed in self.prerequisites_of(dependent) {
                    self.remove_edge(dependent, installed);
                }

                // Restore the original edges. They were valid before this
                // transaction and removing edges cannot create a cycle.
                for prerequisite in previous.iter().copied() {
                    self.insert_edge(dependent, prerequisite);
                }

                // Remove nodes introduced only by the failed transaction.
                let introduced: Vec<NodeId> = self
                    .prerequisites
                    .keys()
                    .filter(|node| !original_nodes.contains(node))
                    .copied()
                    .collect();

                for node in introduced {
                    self.remove_node(node);
                }

                return Err(error);
            }
        }

        Ok(true)
    }

    /// Remove an edge, retaining both endpoint nodes.
    ///
    /// Returns `true` if the edge existed.
    pub fn remove_dependency(
        &mut self,
        dependent: NodeId,
        prerequisite: NodeId,
    ) -> bool {
        self.remove_edge(dependent, prerequisite)
    }

    /// Remove a node and every incident edge.
    ///
    /// Other nodes remain tracked. The caller must invalidate or recompute
    /// affected semantic facts before relying on their previous results.
    pub fn remove_node(&mut self, node: NodeId) -> bool {
        if !self.contains_node(node) {
            return false;
        }

        for prerequisite in self.prerequisites_of(node) {
            self.remove_edge(node, prerequisite);
        }

        for dependent in self.dependents_of(node) {
            self.remove_edge(dependent, node);
        }

        self.prerequisites.remove(&node);
        self.dependents.remove(&node);
        true
    }

    /// Return the changed node and all transitively dependent nodes.
    ///
    /// The graph is not mutated. The result includes `changed` even if that
    /// node has not previously been registered.
    pub fn invalidate_from(&self, changed: NodeId) -> BTreeSet<NodeId> {
        let mut affected = BTreeSet::new();
        let mut pending = VecDeque::new();

        affected.insert(changed);
        pending.push_back(changed);

        while let Some(current) = pending.pop_front() {
            if let Some(children) = self.dependents.get(&current) {
                for child in children {
                    if affected.insert(*child) {
                        pending.push_back(*child);
                    }
                }
            }
        }

        affected
    }

    /// Deterministic topological order of every tracked node.
    ///
    /// Prerequisites precede their dependents. A cycle is an error; a partial
    /// ordering is never returned as if it were complete.
    pub fn topological_order(
        &self,
    ) -> Result<Vec<NodeId>, DependencyGraphError> {
        let mut remaining: BTreeMap<NodeId, usize> = self
            .prerequisites
            .iter()
            .map(|(node, deps)| (*node, deps.len()))
            .collect();

        let mut ready: BTreeSet<NodeId> = remaining
            .iter()
            .filter_map(|(node, count)| {
                (*count == 0).then_some(*node)
            })
            .collect();

        let mut ordered = Vec::with_capacity(remaining.len());

        while let Some(node) = ready.pop_first() {
            ordered.push(node);

            if let Some(children) = self.dependents.get(&node) {
                for dependent in children {
                    if let Some(count) = remaining.get_mut(dependent) {
                        if *count == 0 {
                            return Err(DependencyGraphError::InvalidGraph {
                                nodes: vec![node, *dependent],
                                reason: "inconsistent dependency counts",
                            });
                        }

                        *count -= 1;
                        if *count == 0 {
                            ready.insert(*dependent);
                        }
                    }
                }
            }
        }

        if ordered.len() != remaining.len() {
            let emitted: BTreeSet<NodeId> =
                ordered.iter().copied().collect();

            let nodes = remaining
                .keys()
                .filter(|node| !emitted.contains(node))
                .copied()
                .collect();

            return Err(DependencyGraphError::InvalidGraph {
                nodes,
                reason: "dependency cycle detected",
            });
        }

        Ok(ordered)
    }

    /// Topologically order only `subset`.
    ///
    /// Dependencies outside the subset are treated as already satisfied.
    /// This is intended for scheduling a previously validated invalidation
    /// set. A cycle wholly contained in the subset is still an error.
    pub fn topological_order_for(
        &self,
        subset: &BTreeSet<NodeId>,
    ) -> Result<Vec<NodeId>, DependencyGraphError> {
        let mut remaining: BTreeMap<NodeId, usize> = subset
            .iter()
            .map(|node| {
                let count = self
                    .prerequisites
                    .get(node)
                    .map(|deps| {
                        deps.iter()
                            .filter(|dep| subset.contains(dep))
                            .count()
                    })
                    .unwrap_or(0);

                (*node, count)
            })
            .collect();

        let mut ready: BTreeSet<NodeId> = remaining
            .iter()
            .filter_map(|(node, count)| {
                (*count == 0).then_some(*node)
            })
            .collect();

        let mut ordered = Vec::with_capacity(subset.len());

        while let Some(node) = ready.pop_first() {
            ordered.push(node);

            if let Some(children) = self.dependents.get(&node) {
                for child in children {
                    if !subset.contains(child) {
                        continue;
                    }

                    if let Some(count) = remaining.get_mut(child) {
                        if *count == 0 {
                            return Err(DependencyGraphError::InvalidGraph {
                                nodes: vec![node, *child],
                                reason: "inconsistent subset dependency counts",
                            });
                        }

                        *count -= 1;
                        if *count == 0 {
                            ready.insert(*child);
                        }
                    }
                }
            }
        }

        if ordered.len() != subset.len() {
            let emitted: BTreeSet<NodeId> =
                ordered.iter().copied().collect();

            let nodes = subset
                .iter()
                .filter(|node| !emitted.contains(node))
                .copied()
                .collect();

            return Err(DependencyGraphError::InvalidGraph {
                nodes,
                reason: "cycle detected in requested subset",
            });
        }

        Ok(ordered)
    }

    /// Validate the adjacency-map invariants and graph acyclicity.
    pub fn validate(&self) -> Result<(), DependencyGraphError> {
        for (dependent, prerequisites) in &self.prerequisites {
            if !self.dependents.contains_key(dependent) {
                return Err(DependencyGraphError::InvalidGraph {
                    nodes: vec![*dependent],
                    reason: "node is missing its reverse-map entry",
                });
            }

            for prerequisite in prerequisites {
                if !self
                    .dependents
                    .get(prerequisite)
                    .is_some_and(|items| items.contains(dependent))
                {
                    return Err(DependencyGraphError::InvalidGraph {
                        nodes: vec![*dependent, *prerequisite],
                        reason: "dependency edge is missing its reverse edge",
                    });
                }
            }
        }

        for (prerequisite, dependents) in &self.dependents {
            if !self.prerequisites.contains_key(prerequisite) {
                return Err(DependencyGraphError::InvalidGraph {
                    nodes: vec![*prerequisite],
                    reason: "reverse-map node has no forward-map entry",
                });
            }

            for dependent in dependents {
                if !self
                    .prerequisites
                    .get(dependent)
                    .is_some_and(|items| items.contains(prerequisite))
                {
                    return Err(DependencyGraphError::InvalidGraph {
                        nodes: vec![*dependent, *prerequisite],
                        reason: "reverse edge has no corresponding dependency",
                    });
                }
            }
        }

        self.topological_order().map(|_| ())
    }

    fn insert_edge(&mut self, dependent: NodeId, prerequisite: NodeId) {
        self.track_node(dependent);
        self.track_node(prerequisite);

        self.prerequisites
            .entry(dependent)
            .or_default()
            .insert(prerequisite);

        self.dependents
            .entry(prerequisite)
            .or_default()
            .insert(dependent);
    }

    fn remove_edge(
        &mut self,
        dependent: NodeId,
        prerequisite: NodeId,
    ) -> bool {
        let removed = self
            .prerequisites
            .get_mut(&dependent)
            .is_some_and(|items| items.remove(&prerequisite));

        if removed {
            if let Some(items) = self.dependents.get_mut(&prerequisite) {
                items.remove(&dependent);
            }
        }

        removed
    }

    /// Find a path from `start` to `target` following prerequisite edges.
    ///
    /// Returns a deterministic path including both endpoints.
    fn path_through_prerequisites(
        &self,
        start: NodeId,
        target: NodeId,
    ) -> Option<Vec<NodeId>> {
        let mut pending = VecDeque::new();
        let mut visited = BTreeSet::new();
        let mut predecessor: BTreeMap<NodeId, NodeId> = BTreeMap::new();

        pending.push_back(start);
        visited.insert(start);

        while let Some(current) = pending.pop_front() {
            if current == target {
                let mut path = vec![target];
                let mut cursor = target;

                while cursor != start {
                    cursor = *predecessor.get(&cursor)?;
                    path.push(cursor);
                }

                path.reverse();
                return Some(path);
            }

            if let Some(next_nodes) = self.prerequisites.get(&current) {
                for next in next_nodes {
                    if visited.insert(*next) {
                        predecessor.insert(*next, current);
                        pending.push_back(*next);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> NodeId {
        NodeId::new(value).expect("test IDs must be nonzero")
    }

    #[test]
    fn isolated_nodes_and_duplicate_edges() {
        let mut graph = DependencyGraph::new();
        graph.track_node(id(1));

        assert_eq!(graph.node_count(), 1);
        assert!(graph.add_dependency(id(2), id(1)).unwrap());
        assert!(!graph.add_dependency(id(2), id(1)).unwrap());
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.prerequisites_of(id(2)), vec![id(1)]);
        assert_eq!(graph.dependents_of(id(1)), vec![id(2)]);
        graph.validate().unwrap();
    }

    #[test]
    fn failed_cycle_insertion_does_not_change_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id(2), id(1)).unwrap();
        graph.add_dependency(id(3), id(2)).unwrap();
        let before = graph.clone();

        assert!(matches!(
            graph.add_dependency(id(1), id(3)),
            Err(DependencyGraphError::Cycle { .. })
        ));
        assert_eq!(graph, before);
    }

    #[test]
    fn invalidation_follows_transitive_dependents() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id(2), id(1)).unwrap();
        graph.add_dependency(id(3), id(2)).unwrap();
        graph.add_dependency(id(4), id(1)).unwrap();
        graph.track_node(id(5));

        let expected: BTreeSet<NodeId> =
            [id(1), id(2), id(3), id(4)].into_iter().collect();

        assert_eq!(graph.invalidate_from(id(1)), expected);
    }

    #[test]
    fn topological_order_places_prerequisites_first() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id(3), id(2)).unwrap();
        graph.add_dependency(id(2), id(1)).unwrap();
        graph.track_node(id(4));

        let order = graph.topological_order().unwrap();
        let positions: BTreeMap<NodeId, usize> = order
            .iter()
            .enumerate()
            .map(|(index, node)| (*node, index))
            .collect();

        assert!(positions[&id(1)] < positions[&id(2)]);
        assert!(positions[&id(2)] < positions[&id(3)]);
        assert_eq!(order.len(), 4);
    }

    #[test]
    fn replacement_is_transactional_on_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id(2), id(1)).unwrap();
        graph.add_dependency(id(3), id(2)).unwrap();
        let before = graph.clone();

        assert!(graph.replace_dependencies(id(1), [id(3)]).is_err());
        assert_eq!(graph, before);
    }

    #[test]
    fn replacement_removes_stale_edges_and_deduplicates() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id(3), id(1)).unwrap();
        graph.add_dependency(id(3), id(2)).unwrap();

        assert!(graph.replace_dependencies(id(3), [id(2), id(2)]).unwrap());
        assert_eq!(graph.prerequisites_of(id(3)), vec![id(2)]);
        assert!(graph.dependents_of(id(1)).is_empty());
        graph.validate().unwrap();
    }

    #[test]
    fn failed_replacement_removes_new_isolated_nodes() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id(3), id(2)).unwrap();
        graph.add_dependency(id(2), id(1)).unwrap();
        let before = graph.clone();

        // The new dependency would close a cycle. Node 4 must not leak into
        // the graph when the replacement transaction fails.
        assert!(graph.replace_dependencies(id(1), [id(3), id(4)]).is_err());
        assert_eq!(graph, before);
    }

    #[test]
    fn subset_order_treats_external_dependencies_as_satisfied() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id(2), id(1)).unwrap();
        graph.add_dependency(id(3), id(2)).unwrap();

        let subset: BTreeSet<NodeId> = [id(3), id(2)].into_iter().collect();
        assert_eq!(
            graph.topological_order_for(&subset).unwrap(),
            vec![id(2), id(3)]
        );
    }

    #[test]
    fn removal_cleans_both_adjacency_maps() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id(2), id(1)).unwrap();
        graph.add_dependency(id(3), id(2)).unwrap();

        assert!(graph.remove_node(id(2)));
        assert!(!graph.contains_node(id(2)));
        assert!(graph.contains_node(id(1)));
        assert!(graph.contains_node(id(3)));
        assert!(graph.prerequisites_of(id(3)).is_empty());
        graph.validate().unwrap();
    }

    #[test]
    fn deep_graph_walks_are_iterative() {
        let mut graph = DependencyGraph::new();

        // A finite stress-test workload, not a language capacity limit.
        for value in 1..=2_000 {
            graph.track_node(id(value));
            if value > 1 {
                graph
                    .add_dependency(id(value), id(value - 1))
                    .unwrap();
            }
        }

        assert_eq!(graph.invalidate_from(id(1)).len(), 2_000);
        assert_eq!(graph.topological_order().unwrap().len(), 2_000);
    }
}
