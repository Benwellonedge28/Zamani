
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Semantic Dependency Graph
//!
//! Path: `src/frontend/semantic/traversal/dependency_graph.rs`
//!
//! ## PURPOSE
//!
//! Own a deterministic, target-independent graph of dependencies between
//! semantic entities. The graph is reusable by symbol resolution, type
//! analysis, effects, capabilities, contracts, imports, and incremental
//! analysis.
//!
//! ## AUTHORITY
//!
//! - `grammar/Zamani.g4`: canonical source entry point.
//! - `grammar/antlr/ZamaniParser.g4`: parser composition boundary.
//! - `grammar/specification/semantic-model.md`: normative semantic meaning.
//! - `src/frontend/ast/node/node_id.rs`: AST identity contract.
//! - `src/frontend/ast/node/traversal/children.rs`: canonical AST child
//!   enumeration contract.
//!
//! ## OWNS
//!
//! - Dependency relationships and their kinds.
//! - Deterministic graph storage.
//! - Dependency-first topological ordering.
//! - Strongly connected component discovery.
//! - Cycle reporting.
//! - Graph mutation and accounting.
//!
//! ## DOES NOT OWN
//!
//! - AST storage or child enumeration.
//! - Name resolution, type inference, or type compatibility.
//! - The legality of a particular cycle.
//! - Resource feasibility or hardware discovery.
//! - Classical, quantum, HDL, or other domain-specific rules.
//! - IR generation, scheduling, routing, or runtime execution.
//!
//! ## EDGE DIRECTION
//!
//! `dependent -> dependency` means the dependent entity requires the
//! dependency entity to be processed first.
//!
//! Consequently, topological results put dependencies before dependents.
//! Dependency kinds annotate relationships; distinct kinds between the same
//! pair of nodes do not create duplicate ordering prerequisites.
//!
//! ## SCALABILITY
//!
//! No language-level node, edge, depth, domain, or hardware limit is imposed.
//! Storage and runtime are naturally bounded by host resources and the
//! representational capacity of Rust's collection and integer types.
//! Count overflow is reported rather than wrapped.
//!
//! This graph is iterative: graph depth does not consume one Rust call-stack
//! frame per vertex. Memory is O(V + E), where E counts typed relationships.
//!
//! ## DETERMINISM
//!
//! Ordered maps and sets define stable node, edge, component, and ready-queue
//! ordering for the same graph. Node and dependency-kind types must implement
//! `Ord` with a stable ordering.
//!
//! ## FREEZE CRITERIA
//!
//! This file is ready to freeze when its tests pass, its module is declared
//! by `traversal/mod.rs`, and consumers agree on node identity and edge-kind
//! types before depending on them.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

/// Failure to update graph accounting without integer wraparound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DependencyGraphError {
    /// The host's `usize` cannot represent the next edge count.
    EdgeCountOverflow,

    /// Internal accounting would underflow, indicating an invariant failure.
    EdgeCountUnderflow,
}

impl fmt::Display for DependencyGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EdgeCountOverflow => {
                f.write_str("dependency graph edge count overflow")
            }
            Self::EdgeCountUnderflow => {
                f.write_str("dependency graph edge count invariant underflow")
            }
        }
    }
}

impl std::error::Error for DependencyGraphError {}

/// A deterministic directed dependency graph.
///
/// `N` is the caller's canonical identity type for graph vertices.
/// `K` describes the kind of each dependency relationship.
///
/// Prefer semantic entity IDs for semantic dependencies. Use AST `NodeId`
/// only when the graph specifically models dependencies between AST nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DependencyGraph<N, K> {
    /// `dependent -> dependency -> relationship kinds`.
    dependencies: BTreeMap<N, BTreeMap<N, BTreeSet<K>>>,
    /// Number of distinct `(dependent, dependency, kind)` relationships.
    edge_count: usize,
}

impl<N, K> Default for DependencyGraph<N, K> {
    fn default() -> Self {
        Self {
            dependencies: BTreeMap::new(),
            edge_count: 0,
        }
    }
}

/// Dependency-first topological-order result.
///
/// If `complete` is false, `order` contains only vertices that could be
/// emitted by Kahn's algorithm. `remaining` contains vertices blocked by
/// cycles, including vertices downstream of a cycle. `cycles` contains the
/// actual cyclic strongly connected components.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopologicalOrder<N> {
    /// Vertices emitted in dependency-first order.
    pub order: Vec<N>,
    /// Whether every vertex was ordered.
    pub complete: bool,
    /// Vertices not emitted because they depend, directly or indirectly,
    /// on a cycle.
    pub remaining: Vec<N>,
    /// Cyclic strongly connected components.
    pub cycles: Vec<Vec<N>>,
}

/// A graph snapshot suitable for diagnostics and tests.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DependencyGraphStatistics {
    /// Number of vertices.
    pub nodes: usize,
    /// Number of distinct typed dependency relationships.
    pub edges: usize,
    /// Number of vertices with at least one dependency.
    pub nodes_with_dependencies: usize,
}

impl<N, K> DependencyGraph<N, K>
where
    N: Ord + Clone,
    K: Ord + Clone,
{
    /// Creates an empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a vertex. Adding an existing vertex is idempotent.
    pub fn add_node(&mut self, node: N) -> bool {
        if self.dependencies.contains_key(&node) {
            return false;
        }

        self.dependencies.insert(node, BTreeMap::new());
        true
    }

    /// Returns whether the vertex exists.
    #[must_use]
    pub fn contains_node(&self, node: &N) -> bool {
        self.dependencies.contains_key(node)
    }

    /// Returns the number of vertices.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns the number of distinct typed dependency relationships.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Returns vertices in deterministic order.
    pub fn nodes(&self) -> impl Iterator<Item = &N> {
        self.dependencies.keys()
    }

    /// Returns a vertex's dependencies and their relationship kinds.
    ///
    /// The returned map is read-only and ordered. A missing vertex returns
    /// `None`; an existing vertex without dependencies returns an empty map.
    #[must_use]
    pub fn dependencies_of(
        &self,
        dependent: &N,
    ) -> Option<&BTreeMap<N, BTreeSet<K>>> {
        self.dependencies.get(dependent)
    }

    /// Returns direct dependents of `dependency` in deterministic order.
    ///
    /// An unknown vertex has no dependents and returns an empty vector.
    #[must_use]
    pub fn dependents_of(&self, dependency: &N) -> Vec<&N> {
        self.dependencies
            .iter()
            .filter_map(|(dependent, dependencies)| {
                dependencies.contains_key(dependency).then_some(dependent)
            })
            .collect()
    }

    /// Adds the relationship `dependent -> dependency` with the given kind.
    ///
    /// Both vertices are added automatically. Adding an existing typed
    /// relationship is idempotent and returns `Ok(false)`. A new relationship
    /// returns `Ok(true)`.
    ///
    /// If accounting would overflow, the graph remains unchanged.
    pub fn add_dependency(
        &mut self,
        dependent: N,
        dependency: N,
        kind: K,
    ) -> Result<bool, DependencyGraphError> {
        let already_exists = self
            .dependencies
            .get(&dependent)
            .and_then(|entries| entries.get(&dependency))
            .is_some_and(|kinds| kinds.contains(&kind));

        if already_exists {
            return Ok(false);
        }

        let next_count = self
            .edge_count
            .checked_add(1)
            .ok_or(DependencyGraphError::EdgeCountOverflow)?;

        self.dependencies
            .entry(dependent.clone())
            .or_default()
            .entry(dependency.clone())
            .or_default()
            .insert(kind);

        self.dependencies.entry(dependency).or_default();
        self.edge_count = next_count;
        Ok(true)
    }

    /// Removes one typed relationship.
    ///
    /// Returns `true` only if the relationship existed. Vertices are retained,
    /// even if they become isolated. This makes incremental edits predictable.
    pub fn remove_dependency(
        &mut self,
        dependent: &N,
        dependency: &N,
        kind: &K,
    ) -> Result<bool, DependencyGraphError> {
        let mut removed = false;
        let mut remove_pair = false;

        if let Some(entries) = self.dependencies.get_mut(dependent) {
            if let Some(kinds) = entries.get_mut(dependency) {
                removed = kinds.remove(kind);
                remove_pair = kinds.is_empty();
            }

            if remove_pair {
                entries.remove(dependency);
            }
        }

        if removed {
            self.edge_count = self
                .edge_count
                .checked_sub(1)
                .ok_or(DependencyGraphError::EdgeCountUnderflow)?;
        }

        Ok(removed)
    }

    /// Removes a vertex and every incoming or outgoing relationship.
    ///
    /// Returns `Ok(false)` if the vertex was absent. Vertices that depended
    /// on the removed vertex remain present but no longer have that edge;
    /// callers performing incremental analysis must invalidate those
    /// dependents before trusting prior semantic results.
    pub fn remove_node(
        &mut self,
        node: &N,
    ) -> Result<bool, DependencyGraphError> {
        let Some(outgoing) = self.dependencies.get(node) else {
            return Ok(false);
        };

        let mut removed_edges = 0_usize;

        for kinds in outgoing.values() {
            removed_edges = removed_edges
                .checked_add(kinds.len())
                .ok_or(DependencyGraphError::EdgeCountOverflow)?;
        }

        // Count and remove incoming relationships. Skip the node itself:
        // its self-edge was already included in its outgoing relationships.
        for (dependent, entries) in self.dependencies.iter_mut() {
            if dependent == node {
                continue;
            }

            if let Some(kinds) = entries.remove(node) {
                removed_edges = removed_edges
                    .checked_add(kinds.len())
                    .ok_or(DependencyGraphError::EdgeCountOverflow)?;
            }
        }

        self.dependencies.remove(node);

        self.edge_count = self
            .edge_count
            .checked_sub(removed_edges)
            .ok_or(DependencyGraphError::EdgeCountUnderflow)?;

        Ok(true)
    }

    /// Returns deterministic graph statistics.
    #[must_use]
    pub fn statistics(&self) -> DependencyGraphStatistics {
        DependencyGraphStatistics {
            nodes: self.dependencies.len(),
            edges: self.edge_count,
            nodes_with_dependencies: self
                .dependencies
                .values()
                .filter(|entries| !entries.is_empty())
                .count(),
        }
    }

    /// Computes a deterministic dependency-first topological ordering.
    ///
    /// The algorithm is iterative and O(V + E), excluding ordered-collection
    /// costs. If the graph contains cycles, `complete` is false. The caller
    /// must not treat a partial order as proof of successful semantic analysis.
    #[must_use]
    pub fn topological_order(&self) -> TopologicalOrder<N> {
        let mut remaining_dependency_count: BTreeMap<N, usize> = self
            .dependencies
            .iter()
            .map(|(node, dependencies)| (node.clone(), dependencies.len()))
            .collect();

        let mut dependents: BTreeMap<N, BTreeSet<N>> = self
            .dependencies
            .keys()
            .cloned()
            .map(|node| (node, BTreeSet::new()))
            .collect();

        for (dependent, dependencies) in &self.dependencies {
            for dependency in dependencies.keys() {
                dependents
                    .entry(dependency.clone())
                    .or_default()
                    .insert(dependent.clone());
            }
        }

        let mut ready: BTreeSet<N> = remaining_dependency_count
            .iter()
            .filter_map(|(node, count)| {
                (*count == 0).then_some(node.clone())
            })
            .collect();

        let mut order = Vec::with_capacity(self.node_count());

        while let Some(node) = ready.iter().next().cloned() {
            ready.remove(&node);
            order.push(node.clone());

            if let Some(children) = dependents.get(&node) {
                for dependent in children {
                    if let Some(count) =
                        remaining_dependency_count.get_mut(dependent)
                    {
                        // A zero count here would indicate a broken internal
                        // invariant. Avoid underflow even in that situation.
                        if *count == 0 {
                            continue;
                        }

                        *count -= 1;
                        if *count == 0 {
                            ready.insert(dependent.clone());
                        }
                    }
                }
            }
        }

        let ordered: BTreeSet<N> = order.iter().cloned().collect();
        let remaining: Vec<N> = self
            .dependencies
            .keys()
            .filter(|node| !ordered.contains(*node))
            .cloned()
            .collect();

        let cycles = self.cycles();

        TopologicalOrder {
            complete: remaining.is_empty(),
            order,
            remaining,
            cycles,
        }
    }

    /// Returns the graph's cyclic strongly connected components.
    ///
    /// A component is cyclic if it contains multiple vertices or consists
    /// of a single vertex with a self-dependency. Every component and its
    /// members are returned in deterministic order.
    ///
    /// The algorithm is iterative Kosaraju: it does not recurse with graph
    /// depth and runs in O(V + E) graph operations, plus ordered-map costs.
    #[must_use]
    pub fn cycles(&self) -> Vec<Vec<N>> {
        let (adjacency, reverse) = self.adjacency_pair();
        let mut visited = BTreeSet::new();
        let mut finish_order = Vec::with_capacity(self.node_count());

        // First pass: iterative DFS over dependent -> dependency edges.
        for root in self.dependencies.keys() {
            if visited.contains(root) {
                continue;
            }

            visited.insert(root.clone());
            let root_neighbors = adjacency.get(root).cloned().unwrap_or_default();
            let mut stack: Vec<(N, Vec<N>, usize)> =
                vec![(root.clone(), root_neighbors, 0)];

            while let Some((node, neighbors, next_index)) = stack.last_mut() {
                if *next_index < neighbors.len() {
                    let neighbor = neighbors[*next_index].clone();
                    *next_index += 1;

                    if visited.insert(neighbor.clone()) {
                        let next_neighbors = adjacency
                            .get(&neighbor)
                            .cloned()
                            .unwrap_or_default();

                        stack.push((neighbor, next_neighbors, 0));
                    }
                } else {
                    let (finished, _, _) =
                        stack.pop().expect("nonempty DFS stack");
                    finish_order.push(finished);
                }
            }
        }

        // Second pass: traverse the reversed graph in reverse finish order.
        visited.clear();
        let mut cyclic_components = Vec::new();

        for root in finish_order.into_iter().rev() {
            if !visited.insert(root.clone()) {
                continue;
            }

            let mut component = Vec::new();
            let mut stack = vec![root.clone()];

            while let Some(node) = stack.pop() {
                component.push(node.clone());

                if let Some(neighbors) = reverse.get(&node) {
                    for neighbor in neighbors.iter().rev() {
                        if visited.insert(neighbor.clone()) {
                            stack.push(neighbor.clone());
                        }
                    }
                }
            }

            component.sort();

            let is_cycle = component.len() > 1
                || self
                    .dependencies
                    .get(&component[0])
                    .is_some_and(|entries| entries.contains_key(&component[0]));

            if is_cycle {
                cyclic_components.push(component);
            }
        }

        cyclic_components.sort();
        cyclic_components
    }

    /// Builds adjacency lists in both directions.
    ///
    /// Relationship kinds are intentionally omitted here: graph algorithms
    /// need dependency topology, not domain-specific interpretations.
    fn adjacency_pair(&self) -> (BTreeMap<N, Vec<N>>, BTreeMap<N, Vec<N>>) {
        let mut adjacency: BTreeMap<N, Vec<N>> = self
            .dependencies
            .keys()
            .cloned()
            .map(|node| (node, Vec::new()))
            .collect();

        let mut reverse = adjacency.clone();

        for (dependent, dependencies) in &self.dependencies {
            for dependency in dependencies.keys() {
                adjacency
                    .entry(dependent.clone())
                    .or_default()
                    .push(dependency.clone());

                reverse
                    .entry(dependency.clone())
                    .or_default()
                    .push(dependent.clone());
            }
        }

        // BTreeMap iteration already gives deterministic source order; sorting
        // also makes the contract explicit for adjacency lists.
        for neighbors in adjacency.values_mut() {
            neighbors.sort();
        }
        for neighbors in reverse.values_mut() {
            neighbors.sort();
        }

        (adjacency, reverse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Graph = DependencyGraph<&'static str, &'static str>;

    fn edge(
        graph: &mut Graph,
        dependent: &'static str,
        dependency: &'static str,
        kind: &'static str,
    ) {
        graph
            .add_dependency(dependent, dependency, kind)
            .expect("edge insertion should succeed");
    }

    #[test]
    fn empty_graph_is_complete() {
        let graph = Graph::new();
        let result = graph.topological_order();

        assert!(result.complete);
        assert!(result.order.is_empty());
        assert!(result.remaining.is_empty());
        assert!(result.cycles.is_empty());
    }

    #[test]
    fn dependency_is_ordered_before_dependent() {
        let mut graph = Graph::new();
        edge(&mut graph, "function", "type", "requires-type");
        edge(&mut graph, "type", "declaration", "resolves-to");

        let result = graph.topological_order();

        assert!(result.complete);
        assert_eq!(
            result.order,
            vec!["declaration", "type", "function"]
        );
    }

    #[test]
    fn ordering_is_deterministic_for_independent_nodes() {
        let mut graph = Graph::new();
        graph.add_node("z");
        graph.add_node("a");
        graph.add_node("m");

        assert_eq!(
            graph.topological_order().order,
            vec!["a", "m", "z"]
        );
    }

    #[test]
    fn duplicate_typed_edges_are_idempotent() {
        let mut graph = Graph::new();

        assert_eq!(
            graph.add_dependency("f", "t", "type").unwrap(),
            true
        );
        assert_eq!(
            graph.add_dependency("f", "t", "type").unwrap(),
            false
        );
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn distinct_kinds_share_one_ordering_prerequisite() {
        let mut graph = Graph::new();
        edge(&mut graph, "consumer", "provider", "type");
        edge(&mut graph, "consumer", "provider", "effect");

        assert_eq!(graph.edge_count(), 2);

        let result = graph.topological_order();
        assert!(result.complete);
        assert_eq!(result.order, vec!["provider", "consumer"]);
    }

    #[test]
    fn self_dependency_is_a_cycle() {
        let mut graph = Graph::new();
        edge(&mut graph, "alias", "alias", "type-alias");

        let result = graph.topological_order();

        assert!(!result.complete);
        assert_eq!(result.cycles, vec![vec!["alias"]]);
        assert_eq!(result.remaining, vec!["alias"]);
    }

    #[test]
    fn multi_node_cycle_is_reported_as_one_component() {
        let mut graph = Graph::new();
        edge(&mut graph, "a", "b", "depends");
        edge(&mut graph, "b", "c", "depends");
        edge(&mut graph, "c", "a", "depends");

        assert_eq!(graph.cycles(), vec![vec!["a", "b", "c"]]);
    }

    #[test]
    fn separate_cycles_are_reported_deterministically() {
        let mut graph = Graph::new();
        edge(&mut graph, "b", "a", "depends");
        edge(&mut graph, "a", "b", "depends");
        edge(&mut graph, "y", "x", "depends");
        edge(&mut graph, "x", "y", "depends");

        assert_eq!(
            graph.cycles(),
            vec![vec!["a", "b"], vec!["x", "y"]]
        );
    }

    #[test]
    fn acyclic_vertices_are_ordered_even_when_another_component_cycles() {
        let mut graph = Graph::new();
        graph.add_node("independent");
        edge(&mut graph, "a", "b", "depends");
        edge(&mut graph, "b", "a", "depends");
        edge(&mut graph, "consumer", "a", "depends");

        let result = graph.topological_order();

        assert!(!result.complete);
        assert_eq!(result.order, vec!["independent"]);
        assert_eq!(result.cycles, vec![vec!["a", "b"]]);
        assert_eq!(result.remaining, vec!["a", "b", "consumer"]);
    }

    #[test]
    fn removing_one_kind_preserves_other_relationship_kinds() {
        let mut graph = Graph::new();
        edge(&mut graph, "consumer", "provider", "type");
        edge(&mut graph, "consumer", "provider", "effect");

        assert!(graph
            .remove_dependency(&"consumer", &"provider", &"type")
            .unwrap());

        let kinds = graph
            .dependencies_of(&"consumer")
            .unwrap()
            .get("provider")
            .unwrap();

        assert_eq!(kinds, &BTreeSet::from(["effect"]));
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn removing_a_node_removes_incoming_and_outgoing_edges() {
        let mut graph = Graph::new();
        edge(&mut graph, "a", "b", "type");
        edge(&mut graph, "b", "c", "type");
        edge(&mut graph, "c", "b", "effect");
        edge(&mut graph, "b", "b", "self");

        assert!(graph.remove_node(&"b").unwrap());
        assert!(!graph.contains_node(&"b"));
        assert_eq!(graph.edge_count(), 0);
        assert_eq!(graph.node_count(), 2);
        assert!(graph.dependencies_of(&"a").unwrap().is_empty());
        assert!(graph.dependencies_of(&"c").unwrap().is_empty());
    }

    #[test]
    fn dependency_queries_are_deterministic() {
        let mut graph = Graph::new();
        edge(&mut graph, "consumer", "z", "type");
        edge(&mut graph, "consumer", "a", "effect");
        edge(&mut graph, "consumer", "m", "contract");

        let dependencies = graph
            .dependencies_of(&"consumer")
            .unwrap()
            .keys()
            .copied()
            .collect::<Vec<_>>();

        assert_eq!(dependencies, vec!["a", "m", "z"]);
        assert_eq!(
            graph.dependents_of(&"a").into_iter().copied().collect::<Vec<_>>(),
            vec!["consumer"]
        );
    }

    #[test]
    fn statistics_count_typed_edges_and_nonisolated_nodes() {
        let mut graph = Graph::new();
        graph.add_node("isolated");
        edge(&mut graph, "consumer", "provider", "type");
        edge(&mut graph, "consumer", "provider", "effect");

        assert_eq!(
            graph.statistics(),
            DependencyGraphStatistics {
                nodes: 3,
                edges: 2,
                nodes_with_dependencies: 1,
            }
        );
    }

    #[test]
    fn long_dependency_chain_does_not_use_recursive_graph_walks() {
        let mut graph = DependencyGraph::<u64, &'static str>::new();
        let count = 20_000_u64;

        for node in 1..count {
            graph
                .add_dependency(node, node - 1, "chain")
                .expect("edge insertion should succeed");
        }

        let result = graph.topological_order();

        assert!(result.complete);
        assert_eq!(result.order.len(), count as usize);
        assert_eq!(result.order.first(), Some(&0));
        assert_eq!(result.order.last(), Some(&(count - 1)));
        assert!(graph.cycles().is_empty());
    }
}
