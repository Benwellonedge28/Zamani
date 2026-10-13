
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Analysis — Dependency Cycle Analysis
//!
//! Path: `src/frontend/semantic/symbols/cycles.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Analyze directed dependencies and report cyclic components deterministically.
//! The graph is generic over canonical dependency identities: callers may use
//! module paths, semantic symbol IDs, type-definition IDs, or other identities
//! already owned by the relevant semantic subsystem.
//!
//! ## Ownership
//!
//! This module owns:
//! - dependency edge and dependency-kind representations;
//! - deterministic directed dependency graph storage;
//! - iterative strongly connected component analysis;
//! - cycle classification under an explicit policy;
//! - a reproducible witness cycle for each forbidden cyclic component.
//!
//! This module does not own:
//! - AST nodes, source spans, or AST identity allocation;
//! - the canonical symbol table, lexical scopes, or name resolution;
//! - module/import/export registration;
//! - type inference, alias resolution, or constant evaluation;
//! - diagnostic rendering or source-map lookup;
//! - target selection, hardware allocation, IR generation, or execution.
//!
//! ## Integration contract
//!
//! - `table.rs` remains the canonical owner of symbol records.
//! - `scopes.rs` remains the canonical owner of lexical scopes.
//! - `resolution.rs` remains the canonical owner of name resolution.
//! - `imports.rs` remains the canonical owner of module/import/export records.
//! - `declarations.rs` supplies declaration and type-dependency edges.
//! - `types/aliases.rs` supplies alias/type-definition dependencies.
//! - `traversal/dependency_graph.rs`, if introduced, may adapt canonical AST
//!   relationships into this graph but must not create another symbol table.
//! - The semantic pipeline applies the returned cycle reports and translates
//!   them into the repository's canonical diagnostic types.
//!
//! ## Cycle semantics
//!
//! A cycle is not automatically an error. The policy explicitly identifies
//! dependency kinds that are forbidden to participate in a cycle. For example,
//! circular type aliases may be forbidden while recursive function calls may
//! be permitted. The normative language specification remains authoritative.
//!
//! An edge whose endpoints belong to the same strongly connected component
//! participates in a cycle. A forbidden edge in such a component makes that
//! component forbidden under the supplied policy.
//!
//! ## Determinism and scalability
//!
//! Ordered collections make node, edge, component, and report ordering stable
//! for the same graph and policy. Both graph passes and witness construction
//! are iterative, avoiding recursion-depth failure on deeply nested graphs.
//! No fixed graph-size, path-depth, domain-count, or machine-capacity ceiling
//! is defined here. Work and storage grow with the graph and available
//! resources. Allocation failure may still abort at the allocator boundary;
//! this module does not claim that finite machines can analyze infinite input.
//!
//! Operational budgets, if needed, belong to semantic configuration/session
//! policy. An interrupted or incomplete analysis must not be reported as a
//! complete, valid result.
//!
//! ## Source and grammar boundary
//!
//! The canonical grammar parses imports and exports; it does not decide their
//! semantic cycle policy. The caller must preserve canonical declaration
//! identities and may attach source-origin descriptions to edges. Mapping an
//! origin back to an exact source span belongs to declaration collection and
//! diagnostic integration.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;

/// A named dependency category.
///
/// The name is an open identifier rather than a closed enumeration, allowing
/// future language features and computational domains to introduce dependency
/// categories without changing this module.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyKind(String);

impl DependencyKind {
    /// Construct a dependency kind from a non-empty identifier.
    ///
    /// The identifier is preserved as supplied. Grammar-level identifier
    /// normalization, if any, must occur before this constructor is called.
    pub fn new(value: impl Into<String>) -> Result<Self, CycleGraphError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CycleGraphError::EmptyDependencyKind);
        }

        Ok(Self(value))
    }

    /// Return the dependency kind's canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Kind for an import dependency.
    #[must_use]
    pub fn import() -> Self {
        Self("import".to_owned())
    }

    /// Kind for a re-export dependency.
    #[must_use]
    pub fn reexport() -> Self {
        Self("reexport".to_owned())
    }

    /// Kind for a type-alias dependency.
    #[must_use]
    pub fn type_alias() -> Self {
        Self("type_alias".to_owned())
    }

    /// Kind for a type-definition dependency.
    #[must_use]
    pub fn type_definition() -> Self {
        Self("type_definition".to_owned())
    }

    /// Kind for a compile-time constant dependency.
    #[must_use]
    pub fn constant_value() -> Self {
        Self("constant_value".to_owned())
    }

    /// Kind for a function-call dependency.
    ///
    /// This kind is not forbidden by the default policy, because recursive
    /// functions may be legal. A language-specific policy may override that.
    #[must_use]
    pub fn function_call() -> Self {
        Self("function_call".to_owned())
    }
}

impl fmt::Display for DependencyKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One directed dependency edge.
///
/// `from` depends on `to`. `origin` is optional caller-supplied provenance,
/// such as a declaration ID or a source-location description. It is not
/// interpreted as source text and does not replace canonical source spans.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyEdge<N> {
    from: N,
    to: N,
    kind: DependencyKind,
    origin: Option<String>,
}

impl<N> DependencyEdge<N> {
    /// Construct an edge between two canonical dependency identities.
    #[must_use]
    pub fn new(
        from: N,
        to: N,
        kind: DependencyKind,
        origin: Option<String>,
    ) -> Self {
        Self {
            from,
            to,
            kind,
            origin,
        }
    }

    /// Return the dependency source.
    #[must_use]
    pub const fn from(&self) -> &N {
        &self.from
    }

    /// Return the dependency target.
    #[must_use]
    pub const fn to(&self) -> &N {
        &self.to
    }

    /// Return the dependency category.
    #[must_use]
    pub const fn kind(&self) -> &DependencyKind {
        &self.kind
    }

    /// Return optional caller-supplied provenance.
    #[must_use]
    pub fn origin(&self) -> Option<&str> {
        self.origin.as_deref()
    }
}

/// Deterministic directed dependency graph.
///
/// The graph stores identities and relationships only. It does not allocate
/// semantic IDs or duplicate symbol, module, scope, or AST records.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DependencyGraph<N> {
    nodes: BTreeSet<N>,
    edges: BTreeSet<DependencyEdge<N>>,
}

impl<N> Default for DependencyGraph<N>
where
    N: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<N> DependencyGraph<N>
where
    N: Ord + Clone,
{
    /// Create an empty dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: BTreeSet::new(),
            edges: BTreeSet::new(),
        }
    }

    /// Return the number of unique graph nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of unique dependency edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Insert an isolated node.
    ///
    /// Returns `true` if this identity was not already present.
    pub fn add_node(&mut self, node: N) -> bool {
        self.nodes.insert(node)
    }

    /// Insert a directed dependency edge and its endpoint nodes.
    ///
    /// Exact duplicate edges are stored only once. Edges with different kinds
    /// or different provenance remain distinct.
    pub fn add_edge(&mut self, edge: DependencyEdge<N>) -> bool {
        self.nodes.insert(edge.from.clone());
        self.nodes.insert(edge.to.clone());
        self.edges.insert(edge)
    }

    /// Iterate over nodes in deterministic identity order.
    pub fn nodes(&self) -> impl Iterator<Item = &N> {
        self.nodes.iter()
    }

    /// Iterate over edges in deterministic order.
    pub fn edges(&self) -> impl Iterator<Item = &DependencyEdge<N>> {
        self.edges.iter()
    }

    /// Analyze all cyclic strongly connected components under `policy`.
    ///
    /// The result includes permitted cycles for inspection. The caller must
    /// check [`CycleAnalysis::is_valid`] before accepting the analysis as
    /// cycle-valid.
    pub fn analyze(&self, policy: &CyclePolicy) -> CycleAnalysis<N> {
        let adjacency = self.adjacency();
        let reverse_adjacency = self.reverse_adjacency();
        let mut visited = BTreeSet::new();
        let mut finish_order = Vec::with_capacity(self.nodes.len());

        // First Kosaraju pass: iterative depth-first finishing order.
        for root in &self.nodes {
            if visited.contains(root) {
                continue;
            }

            visited.insert(root.clone());
            let mut stack: Vec<(N, usize)> = vec![(root.clone(), 0)];

            while !stack.is_empty() {
                let next_node = {
                    let (node, next_index) = stack
                        .last_mut()
                        .expect("stack is checked as non-empty");

                    let neighbors = adjacency.get(node).map(Vec::as_slice).unwrap_or(&[]);

                    if *next_index < neighbors.len() {
                        let neighbor = neighbors[*next_index].clone();
                        *next_index += 1;
                        Some(neighbor)
                    } else {
                        None
                    }
                };

                if let Some(neighbor) = next_node {
                    if visited.insert(neighbor.clone()) {
                        stack.push((neighbor, 0));
                    }
                } else if let Some((finished, _)) = stack.pop() {
                    finish_order.push(finished);
                }
            }
        }

        // Second Kosaraju pass: collect strongly connected components
        // iteratively on the reversed graph.
        visited.clear();
        let mut components = Vec::new();

        for root in finish_order.iter().rev() {
            if !visited.insert(root.clone()) {
                continue;
            }

            let mut component = Vec::new();
            let mut stack = vec![root.clone()];

            while let Some(node) = stack.pop() {
                component.push(node.clone());

                if let Some(neighbors) = reverse_adjacency.get(&node) {
                    // Reverse push order so the smallest identity is visited
                    // first when popped from the LIFO stack.
                    for neighbor in neighbors.iter().rev() {
                        if visited.insert(neighbor.clone()) {
                            stack.push(neighbor.clone());
                        }
                    }
                }
            }

            component.sort();
            components.push(component);
        }

        // Stable component ordering independent of DFS implementation details.
        components.sort();

        let mut cycles = Vec::new();

        for component in components {
            let component_nodes: BTreeSet<N> = component.iter().cloned().collect();
            let internal_edges: Vec<DependencyEdge<N>> = self
                .edges
                .iter()
                .filter(|edge| {
                    component_nodes.contains(&edge.from)
                        && component_nodes.contains(&edge.to)
                })
                .cloned()
                .collect();

            let has_self_loop = internal_edges
                .iter()
                .any(|edge| edge.from == edge.to);

            if component.len() == 1 && !has_self_loop {
                continue;
            }

            let forbidden_edges: Vec<&DependencyEdge<N>> = internal_edges
                .iter()
                .filter(|edge| policy.is_forbidden(edge.kind()))
                .collect();

            let disposition = if forbidden_edges.is_empty() {
                CycleDisposition::Permitted
            } else {
                CycleDisposition::Forbidden
            };

            let witness = forbidden_edges
                .first()
                .and_then(|edge| self.build_witness(edge, &component_nodes))
                .unwrap_or_default();

            let forbidden_kinds = internal_edges
                .iter()
                .filter(|edge| policy.is_forbidden(edge.kind()))
                .map(|edge| edge.kind.clone())
                .collect();

            cycles.push(DependencyCycle {
                nodes: component,
                edges: internal_edges,
                witness,
                forbidden_kinds,
                disposition,
            });
        }

        cycles.sort_by(|left, right| left.nodes.cmp(&right.nodes));

        CycleAnalysis { cycles }
    }

    fn adjacency(&self) -> BTreeMap<N, Vec<N>> {
        let mut adjacency: BTreeMap<N, Vec<N>> = self
            .nodes
            .iter()
            .cloned()
            .map(|node| (node, Vec::new()))
            .collect();

        for edge in &self.edges {
            adjacency
                .entry(edge.from.clone())
                .or_default()
                .push(edge.to.clone());
        }

        for neighbors in adjacency.values_mut() {
            neighbors.sort();
            neighbors.dedup();
        }

        adjacency
    }

    fn reverse_adjacency(&self) -> BTreeMap<N, Vec<N>> {
        let mut adjacency: BTreeMap<N, Vec<N>> = self
            .nodes
            .iter()
            .cloned()
            .map(|node| (node, Vec::new()))
            .collect();

        for edge in &self.edges {
            adjacency
                .entry(edge.to.clone())
                .or_default()
                .push(edge.from.clone());
        }

        for neighbors in adjacency.values_mut() {
            neighbors.sort();
            neighbors.dedup();
        }

        adjacency
    }

    fn build_witness(
        &self,
        forbidden_edge: &DependencyEdge<N>,
        component_nodes: &BTreeSet<N>,
    ) -> Option<Vec<DependencyEdge<N>>> {
        if forbidden_edge.from == forbidden_edge.to {
            return Some(vec![forbidden_edge.clone()]);
        }

        let mut outgoing: BTreeMap<N, Vec<DependencyEdge<N>>> = BTreeMap::new();

        for edge in &self.edges {
            if component_nodes.contains(&edge.from)
                && component_nodes.contains(&edge.to)
            {
                outgoing
                    .entry(edge.from.clone())
                    .or_default()
                    .push(edge.clone());
            }
        }

        for edges in outgoing.values_mut() {
            edges.sort();
        }

        // Find a deterministic path from the forbidden edge's target back to
        // its source. Strong connectivity guarantees such a path exists.
        let start = forbidden_edge.to.clone();
        let target = forbidden_edge.from.clone();
        let mut queue = VecDeque::new();
        let mut parent: BTreeMap<N, Option<DependencyEdge<N>>> = BTreeMap::new();

        parent.insert(start.clone(), None);
        queue.push_back(start.clone());

        while let Some(node) = queue.pop_front() {
            if node == target {
                break;
            }

            if let Some(edges) = outgoing.get(&node) {
                for edge in edges {
                    if parent.contains_key(&edge.to) {
                        continue;
                    }

                    parent.insert(edge.to.clone(), Some(edge.clone()));
                    queue.push_back(edge.to.clone());
                }
            }
        }

        if !parent.contains_key(&target) {
            return None;
        }

        let mut return_path = Vec::new();
        let mut current = target;

        while current != start {
            let edge = parent.get(&current)?.as_ref()?.clone();
            current = edge.from.clone();
            return_path.push(edge);
        }

        return_path.reverse();

        let mut witness = Vec::with_capacity(return_path.len() + 1);
        witness.push(forbidden_edge.clone());
        witness.extend(return_path);
        Some(witness)
    }
}

/// Policy for determining which dependency kinds may participate in cycles.
///
/// The default policy forbids cycles involving imports, re-exports, type
/// aliases, type definitions, or constant-value dependencies. These defaults
/// are conservative integration defaults, not a substitute for the normative
/// Zamani language rules. A language specification may permit or prohibit
/// particular cases differently.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CyclePolicy {
    forbidden_kinds: BTreeSet<DependencyKind>,
}

impl Default for CyclePolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl CyclePolicy {
    /// Create a policy with the standard semantic dependency categories.
    #[must_use]
    pub fn new() -> Self {
        let mut policy = Self {
            forbidden_kinds: BTreeSet::new(),
        };

        policy.forbid(DependencyKind::import());
        policy.forbid(DependencyKind::reexport());
        policy.forbid(DependencyKind::type_alias());
        policy.forbid(DependencyKind::type_definition());
        policy.forbid(DependencyKind::constant_value());

        policy
    }

    /// Mark a dependency kind as forbidden in a cycle.
    pub fn forbid(&mut self, kind: DependencyKind) -> bool {
        self.forbidden_kinds.insert(kind)
    }

    /// Allow a dependency kind to participate in cycles.
    ///
    /// Use this only when the language rules permit the corresponding cycle.
    pub fn allow(&mut self, kind: &DependencyKind) -> bool {
        self.forbidden_kinds.remove(kind)
    }

    /// Return whether the kind is forbidden in a cycle.
    #[must_use]
    pub fn is_forbidden(&self, kind: &DependencyKind) -> bool {
        self.forbidden_kinds.contains(kind)
    }

    /// Iterate over forbidden dependency kinds in deterministic order.
    pub fn forbidden_kinds(&self) -> impl Iterator<Item = &DependencyKind> {
        self.forbidden_kinds.iter()
    }
}

/// Classification of a cyclic strongly connected component.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CycleDisposition {
    /// The component contains no edge forbidden by the supplied policy.
    Permitted,
    /// The component contains at least one edge forbidden by the policy.
    Forbidden,
}

/// A cyclic strongly connected component and a reproducible witness cycle.
///
/// `nodes` and `edges` describe the entire cyclic component. `witness`
/// describes one cycle containing a forbidden edge, when the disposition is
/// `Forbidden`. A permitted cycle has an empty witness because it has no
/// forbidden edge to anchor a policy-specific witness.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DependencyCycle<N> {
    nodes: Vec<N>,
    edges: Vec<DependencyEdge<N>>,
    witness: Vec<DependencyEdge<N>>,
    forbidden_kinds: BTreeSet<DependencyKind>,
    disposition: CycleDisposition,
}

impl<N> DependencyCycle<N> {
    /// Return the component's nodes in deterministic order.
    #[must_use]
    pub fn nodes(&self) -> &[N] {
        &self.nodes
    }

    /// Return all edges internal to the component in deterministic order.
    #[must_use]
    pub fn edges(&self) -> &[DependencyEdge<N>] {
        &self.edges
    }

    /// Return the policy-specific witness cycle, if one exists.
    #[must_use]
    pub fn witness(&self) -> &[DependencyEdge<N>] {
        &self.witness
    }

    /// Return the dependency kinds that make this cycle forbidden.
    #[must_use]
    pub fn forbidden_kinds(&self) -> &BTreeSet<DependencyKind> {
        &self.forbidden_kinds
    }

    /// Return the policy classification of this component.
    #[must_use]
    pub const fn disposition(&self) -> CycleDisposition {
        self.disposition
    }

    /// Return whether this component is forbidden by the policy.
    #[must_use]
    pub const fn is_forbidden(&self) -> bool {
        matches!(self.disposition, CycleDisposition::Forbidden)
    }
}

/// Completed result of dependency-cycle analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CycleAnalysis<N> {
    cycles: Vec<DependencyCycle<N>>,
}

impl<N> CycleAnalysis<N> {
    /// Return all cyclic components in deterministic order.
    #[must_use]
    pub fn cycles(&self) -> &[DependencyCycle<N>] {
        &self.cycles
    }

    /// Return only components forbidden by the supplied policy.
    pub fn forbidden_cycles(&self) -> impl Iterator<Item = &DependencyCycle<N>> {
        self.cycles.iter().filter(|cycle| cycle.is_forbidden())
    }

    /// Return whether no forbidden cycle was found.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.cycles.iter().all(|cycle| !cycle.is_forbidden())
    }

    /// Return whether any cyclic component was found, including permitted ones.
    #[must_use]
    pub fn has_cycles(&self) -> bool {
        !self.cycles.is_empty()
    }
}

/// Construction error for dependency-cycle inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CycleGraphError {
    /// A dependency category was empty or whitespace-only.
    EmptyDependencyKind,
}

impl fmt::Display for CycleGraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDependencyKind => {
                formatter.write_str("dependency kind must not be empty")
            }
        }
    }
}

impl Error for CycleGraphError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn kind(value: &str) -> DependencyKind {
        DependencyKind::new(value).expect("non-empty dependency kind")
    }

    fn edge(from: &str, to: &str, kind: DependencyKind) -> DependencyEdge<String> {
        DependencyEdge::new(
            from.to_owned(),
            to.to_owned(),
            kind,
            None,
        )
    }

    #[test]
    fn empty_dependency_kind_is_rejected() {
        assert_eq!(
            DependencyKind::new("  "),
            Err(CycleGraphError::EmptyDependencyKind)
        );
    }

    #[test]
    fn acyclic_graph_has_no_cycles() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(edge("a", "b", DependencyKind::import()));
        graph.add_edge(edge("b", "c", DependencyKind::import()));

        let result = graph.analyze(&CyclePolicy::new());

        assert!(!result.has_cycles());
        assert!(result.is_valid());
    }

    #[test]
    fn forbidden_two_node_cycle_has_a_witness() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(edge("a", "b", DependencyKind::import()));
        graph.add_edge(edge("b", "a", DependencyKind::import()));

        let result = graph.analyze(&CyclePolicy::new());

        assert!(!result.is_valid());
        assert_eq!(result.forbidden_cycles().count(), 1);

        let cycle = &result.cycles()[0];
        assert!(cycle.is_forbidden());
        assert_eq!(cycle.nodes().len(), 2);
        assert_eq!(cycle.witness().len(), 2);
    }

    #[test]
    fn self_dependency_is_detected() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(edge("alias", "alias", DependencyKind::type_alias()));

        let result = graph.analyze(&CyclePolicy::new());

        assert!(!result.is_valid());
        assert_eq!(result.cycles().len(), 1);
        assert_eq!(result.cycles()[0].witness().len(), 1);
    }

    #[test]
    fn recursive_function_calls_are_permitted_by_default() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(edge("f", "g", DependencyKind::function_call()));
        graph.add_edge(edge("g", "f", DependencyKind::function_call()));

        let result = graph.analyze(&CyclePolicy::new());

        assert!(result.has_cycles());
        assert!(result.is_valid());
        assert_eq!(
            result.cycles()[0].disposition(),
            CycleDisposition::Permitted
        );
    }

    #[test]
    fn policy_can_forbid_recursive_function_calls() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(edge("f", "f", DependencyKind::function_call()));

        let mut policy = CyclePolicy::new();
        policy.forbid(DependencyKind::function_call());

        let result = graph.analyze(&policy);

        assert!(!result.is_valid());
        assert!(result.cycles()[0].is_forbidden());
    }

    #[test]
    fn mixed_cycle_is_forbidden_when_it_contains_a_forbidden_edge() {
        let mut graph = DependencyGraph::new();
        graph.add_edge(edge("f", "T", DependencyKind::function_call()));
        graph.add_edge(edge("T", "f", DependencyKind::type_alias()));

        let result = graph.analyze(&CyclePolicy::new());

        assert!(!result.is_valid());
        assert_eq!(result.cycles()[0].forbidden_kinds().len(), 1);
        assert_eq!(
            result.cycles()[0].forbidden_kinds().iter().next().map(DependencyKind::as_str),
            Some("type_alias")
        );
    }

    #[test]
    fn isolated_nodes_are_not_cycles() {
        let mut graph = DependencyGraph::new();
        graph.add_node("standalone".to_owned());

        let result = graph.analyze(&CyclePolicy::new());

        assert!(!result.has_cycles());
        assert!(result.is_valid());
    }

    #[test]
    fn duplicate_edges_do_not_duplicate_graph_relationships() {
        let mut graph = DependencyGraph::new();
        let dependency = edge("a", "b", DependencyKind::import());

        assert!(graph.add_edge(dependency.clone()));
        assert!(!graph.add_edge(dependency));
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn results_are_deterministic_across_insertion_orders() {
        fn build(reverse: bool) -> CycleAnalysis<String> {
            let mut graph = DependencyGraph::new();
            let mut edges = vec![
                edge("a", "b", DependencyKind::type_alias()),
                edge("b", "c", DependencyKind::type_alias()),
                edge("c", "a", DependencyKind::type_alias()),
                edge("x", "y", DependencyKind::function_call()),
                edge("y", "x", DependencyKind::function_call()),
            ];

            if reverse {
                edges.reverse();
            }

            for dependency in edges {
                graph.add_edge(dependency);
            }

            graph.analyze(&CyclePolicy::new())
        }

        assert_eq!(build(false), build(true));
    }

    #[test]
    fn provenance_is_preserved_on_edges() {
        let dependency = DependencyEdge::new(
            "left".to_owned(),
            "right".to_owned(),
            DependencyKind::import(),
            Some("declaration-17".to_owned()),
        );

        assert_eq!(dependency.origin(), Some("declaration-17"));
    }
}
