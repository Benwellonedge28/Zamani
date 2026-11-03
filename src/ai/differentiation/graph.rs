
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

//! # Zamani AI — Differentiation Graph
//!
//! Path: src/ai/differentiation/graph.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides a target-independent dependency graph for automatic
//! differentiation.
//!
//! The graph records the dependencies between differentiable values
//! and computational operations. It supports deterministic graph
//! traversal, structural validation, dependency analysis, and
//! topological ordering.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Differentiation graph structure.
//! - Graph node and edge identities.
//! - Dependency relationships.
//! - Structural graph validation.
//! - Stable topological ordering.
//! - Dependency and ancestor queries.
//! - Cycle detection.
//!
//! This module does not own:
//!
//! - Source parsing or AST construction.
//! - Semantic name resolution.
//! - Mathematical differentiability.
//! - Differentiation rule definitions.
//! - Forward-mode propagation.
//! - Reverse-mode propagation.
//! - Tensor storage or tensor arithmetic.
//! - Model graph construction.
//! - ZUIR construction.
//! - Hardware selection.
//! - Scheduling or execution.
//!
//! ## POCO-REAF
//!
//! The graph does not impose a fixed maximum number of nodes,
//! edges, inputs, outputs, or dependencies.
//!
//! Its representational limits are those of the chosen identifier
//! types, the Rust collection implementations, and available resources.
//!
//! A resource limit is not a semantic graph-size limit.
//!
//! ## Integration
//!
//! `rules.rs` supplies differentiation rule identities and contracts.
//!
//! `forward.rs` and `reverse.rs` consume the graph's dependency
//! information through their respective execution interfaces.
//!
//! `validation.rs` may use this module for structural validation.
//!
//! The canonical AI model graph remains authoritative for model
//! structure. This module represents differentiation dependencies,
//! not a replacement model graph.
//!
//! Graph nodes and edges are declarative. They do not execute
//! mathematical operations.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::hash::Hash;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Identifies a node in a differentiation graph.
///
/// The identifier is generic to allow integration with the canonical
/// identifier types already used by the surrounding AI subsystem.
///
/// An identifier must implement `Ord` so graph traversal and diagnostic
/// ordering are deterministic.
///
/// It must implement `Hash` so callers can use it with other canonical
/// identifier-based systems.
pub trait DifferentiationNodeId:
    Clone + Eq + Ord + Hash + fmt::Debug + fmt::Display
{
}

impl<T> DifferentiationNodeId for T where
    T: Clone + Eq + Ord + Hash + fmt::Debug + fmt::Display
{
}

/// Identifies an operation represented in the graph.
///
/// Operation identities are supplied by the model or IR integration
/// layer. This module does not allocate operation identities.
pub trait DifferentiationOperationId:
    Clone + Eq + Ord + Hash + fmt::Debug + fmt::Display
{
}

impl<T> DifferentiationOperationId for T where
    T: Clone + Eq + Ord + Hash + fmt::Debug + fmt::Display
{
}

// ============================================================================
// NODE DESCRIPTIONS
// ============================================================================

/// Describes the semantic role of a differentiation graph node.
///
/// The variants describe graph structure only. They do not assert
/// that the represented operation is mathematically differentiable.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DifferentiationNodeKind<OperationId> {
    /// A graph input or independent differentiation variable.
    Input,

    /// A constant or externally supplied value.
    Constant,

    /// A computational operation.
    ///
    /// The operation identity is resolved through the authoritative
    /// model-operation or IR registry.
    Operation(OperationId),

    /// A graph output.
    Output,

    /// An explicitly represented intermediate value.
    Intermediate,
}

/// A node in the differentiation dependency graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DifferentiationNode<NodeId, OperationId> {
    id: NodeId,
    kind: DifferentiationNodeKind<OperationId>,
}

impl<NodeId, OperationId> DifferentiationNode<NodeId, OperationId> {
    /// Creates a graph node.
    ///
    /// Structural validation is performed by the graph when the
    /// node is inserted.
    #[must_use]
    pub const fn new(
        id: NodeId,
        kind: DifferentiationNodeKind<OperationId>,
    ) -> Self {
        Self { id, kind }
    }

    /// Returns the node identity.
    #[must_use]
    pub const fn id(&self) -> &NodeId {
        &self.id
    }

    /// Returns the node kind.
    #[must_use]
    pub const fn kind(&self) -> &DifferentiationNodeKind<OperationId> {
        &self.kind
    }

    /// Consumes the node and returns its identity and kind.
    #[must_use]
    pub fn into_parts(self) -> (NodeId, DifferentiationNodeKind<OperationId>) {
        (self.id, self.kind)
    }
}

// ============================================================================
// DEPENDENCY EDGES
// ============================================================================

/// Describes the role of a dependency edge.
///
/// The edge direction is always:
///
///     dependency -> dependent
///
/// This convention makes topological traversal directly usable for
/// forward computation and dependency analysis.
///
/// Reverse differentiation may traverse the same graph in the opposite
/// direction without constructing a second graph.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DifferentiationDependencyKind {
    /// An ordinary computational dependency.
    Value,

    /// A dependency on a parameter.
    Parameter,

    /// A dependency on a control value.
    Control,

    /// A dependency introduced by an explicit graph transformation.
    Derived,

    /// An extensible dependency category.
    Extension(String),
}

/// A dependency edge between graph nodes.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DifferentiationEdge<NodeId> {
    source: NodeId,
    target: NodeId,
    kind: DifferentiationDependencyKind,
}

impl<NodeId> DifferentiationEdge<NodeId> {
    /// Creates a dependency edge.
    ///
    /// `source` is the dependency.
    ///
    /// `target` is the node that depends on it.
    #[must_use]
    pub fn new(
        source: NodeId,
        target: NodeId,
        kind: DifferentiationDependencyKind,
    ) -> Self {
        Self {
            source,
            target,
            kind,
        }
    }

    /// Returns the dependency node.
    #[must_use]
    pub const fn source(&self) -> &NodeId {
        &self.source
    }

    /// Returns the dependent node.
    #[must_use]
    pub const fn target(&self) -> &NodeId {
        &self.target
    }

    /// Returns the dependency kind.
    #[must_use]
    pub const fn kind(&self) -> &DifferentiationDependencyKind {
        &self.kind
    }
}

// ============================================================================
// GRAPH ERRORS
// ============================================================================

/// Structural errors produced by differentiation graph operations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DifferentiationGraphError<NodeId> {
    /// A node identity already exists.
    DuplicateNode {
        /// Conflicting node identity.
        node: NodeId,
    },

    /// A referenced node does not exist.
    MissingNode {
        /// Missing identity.
        node: NodeId,
    },

    /// An edge would create a cycle.
    CycleDetected {
        /// Dependency source.
        source: NodeId,

        /// Dependent target.
        target: NodeId,
    },

    /// The graph contains a cycle.
    ///
    /// The reported cycle is a deterministic closed path.
    InvalidCycle {
        /// Cycle node identities.
        path: Vec<NodeId>,
    },

    /// A graph node has no valid structural role.
    InvalidNode {
        /// Invalid node identity.
        node: NodeId,

        /// Explanation.
        reason: String,
    },

    /// A graph invariant was violated.
    InvariantViolation {
        /// Explanation.
        reason: String,
    },
}

impl<NodeId> fmt::Display for DifferentiationGraphError<NodeId>
where
    NodeId: fmt::Display + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNode { node } => {
                write!(formatter, "duplicate differentiation node '{node}'")
            }

            Self::MissingNode { node } => {
                write!(formatter, "missing differentiation node '{node}'")
            }

            Self::CycleDetected { source, target } => {
                write!(
                    formatter,
                    "dependency edge '{source}' -> '{target}' \
                     would create a cycle"
                )
            }

            Self::InvalidCycle { path } => {
                write!(
                    formatter,
                    "differentiation graph contains a cycle: {path:?}"
                )
            }

            Self::InvalidNode { node, reason } => {
                write!(
                    formatter,
                    "invalid differentiation node '{node}': {reason}"
                )
            }

            Self::InvariantViolation { reason } => {
                write!(
                    formatter,
                    "differentiation graph invariant violation: {reason}"
                )
            }
        }
    }
}

impl<NodeId> Error for DifferentiationGraphError<NodeId>
where
    NodeId: fmt::Debug + fmt::Display + 'static,
{
}

// ============================================================================
// GRAPH VALIDATION REPORT
// ============================================================================

/// A structural graph validation result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DifferentiationGraphValidation<NodeId> {
    node_count: usize,
    edge_count: usize,
    topological_order: Vec<NodeId>,
}

impl<NodeId> DifferentiationGraphValidation<NodeId> {
    /// Returns the number of nodes.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }

    /// Returns the number of edges.
    #[must_use]
    pub const fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Returns the validated topological order.
    #[must_use]
    pub fn topological_order(&self) -> &[NodeId] {
        &self.topological_order
    }

    /// Consumes the report and returns its topological order.
    #[must_use]
    pub fn into_topological_order(self) -> Vec<NodeId> {
        self.topological_order
    }
}

// ============================================================================
// DIFFERENTIATION GRAPH
// ============================================================================

/// A target-independent differentiation dependency graph.
///
/// The graph stores nodes in deterministic identifier order.
///
/// Dependency edges are stored in deterministic order and duplicate
/// edges are rejected.
///
/// The graph does not own operation implementations or numerical values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DifferentiationGraph<NodeId, OperationId> {
    nodes: BTreeMap<NodeId, DifferentiationNode<NodeId, OperationId>>,

    /// Outgoing edges: dependency -> dependent.
    outgoing: BTreeMap<NodeId, BTreeSet<DifferentiationEdge<NodeId>>>,

    /// Incoming edges: dependent -> dependency.
    incoming: BTreeMap<NodeId, BTreeSet<DifferentiationEdge<NodeId>>>,
}

impl<NodeId, OperationId> Default
    for DifferentiationGraph<NodeId, OperationId>
where
    NodeId: DifferentiationNodeId,
    OperationId: DifferentiationOperationId,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<NodeId, OperationId> DifferentiationGraph<NodeId, OperationId>
where
    NodeId: DifferentiationNodeId,
    OperationId: DifferentiationOperationId,
{
    /// Creates an empty differentiation graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            outgoing: BTreeMap::new(),
            incoming: BTreeMap::new(),
        }
    }

    /// Returns the number of graph nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.outgoing.values().map(BTreeSet::len).sum()
    }

    /// Returns whether the graph contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns whether a node exists.
    #[must_use]
    pub fn contains_node(&self, id: &NodeId) -> bool {
        self.nodes.contains_key(id)
    }

    /// Returns a node by identity.
    #[must_use]
    pub fn node(
        &self,
        id: &NodeId,
    ) -> Option<&DifferentiationNode<NodeId, OperationId>> {
        self.nodes.get(id)
    }

    /// Returns all nodes in deterministic identifier order.
    #[must_use]
    pub fn nodes(
        &self,
    ) -> impl Iterator<Item = &DifferentiationNode<NodeId, OperationId>> {
        self.nodes.values()
    }

    /// Returns all dependency edges in deterministic order.
    #[must_use]
    pub fn edges(
        &self,
    ) -> impl Iterator<Item = &DifferentiationEdge<NodeId>> {
        self.outgoing.values().flat_map(BTreeSet::iter)
    }

    /// Inserts a node.
    ///
    /// The operation is atomic: a duplicate identity does not modify
    /// the graph.
    pub fn add_node(
        &mut self,
        node: DifferentiationNode<NodeId, OperationId>,
    ) -> Result<(), DifferentiationGraphError<NodeId>> {
        let id = node.id.clone();

        if self.nodes.contains_key(&id) {
            return Err(DifferentiationGraphError::DuplicateNode { node: id });
        }

        self.nodes.insert(id.clone(), node);
        self.outgoing.insert(id.clone(), BTreeSet::new());
        self.incoming.insert(id, BTreeSet::new());

        Ok(())
    }

    /// Inserts a dependency edge.
    ///
    /// The edge direction is:
    ///
    ///     dependency -> dependent
    ///
    /// The graph remains unchanged if either endpoint is missing,
    /// the edge is duplicated, or the edge would introduce a cycle.
    pub fn add_dependency(
        &mut self,
        source: NodeId,
        target: NodeId,
        kind: DifferentiationDependencyKind,
    ) -> Result<(), DifferentiationGraphError<NodeId>> {
        if !self.nodes.contains_key(&source) {
            return Err(DifferentiationGraphError::MissingNode {
                node: source,
            });
        }

        if !self.nodes.contains_key(&target) {
            return Err(DifferentiationGraphError::MissingNode {
                node: target,
            });
        }

        if source == target {
            return Err(DifferentiationGraphError::CycleDetected {
                source,
                target,
            });
        }

        let edge = DifferentiationEdge::new(
            source.clone(),
            target.clone(),
            kind,
        );

        if self
            .outgoing
            .get(&source)
            .is_some_and(|edges| edges.contains(&edge))
        {
            return Err(
                DifferentiationGraphError::InvariantViolation {
                    reason: format!(
                        "duplicate dependency edge '{source}' -> '{target}'"
                    ),
                },
            );
        }

        // Adding source -> target introduces a cycle precisely when
        // target can already reach source.
        if self.reachable(&target, &source) {
            return Err(DifferentiationGraphError::CycleDetected {
                source,
                target,
            });
        }

        self.outgoing
            .get_mut(&source)
            .ok_or_else(|| {
                DifferentiationGraphError::InvariantViolation {
                    reason: "outgoing adjacency entry is missing".into(),
                }
            })?
            .insert(edge.clone());

        self.incoming
            .get_mut(&target)
            .ok_or_else(|| {
                DifferentiationGraphError::InvariantViolation {
                    reason: "incoming adjacency entry is missing".into(),
                }
            })?
            .insert(edge);

        Ok(())
    }

    /// Returns the direct dependencies of a node.
    ///
    /// The returned identifiers are ordered deterministically.
    pub fn dependencies(
        &self,
        id: &NodeId,
    ) -> Result<Vec<NodeId>, DifferentiationGraphError<NodeId>> {
        let edges = self.incoming.get(id).ok_or_else(|| {
            DifferentiationGraphError::MissingNode {
                node: id.clone(),
            }
        })?;

        Ok(edges.iter().map(|edge| edge.source.clone()).collect())
    }

    /// Returns the direct dependents of a node.
    ///
    /// The returned identifiers are ordered deterministically.
    pub fn dependents(
        &self,
        id: &NodeId,
    ) -> Result<Vec<NodeId>, DifferentiationGraphError<NodeId>> {
        let edges = self.outgoing.get(id).ok_or_else(|| {
            DifferentiationGraphError::MissingNode {
                node: id.clone(),
            }
        })?;

        Ok(edges.iter().map(|edge| edge.target.clone()).collect())
    }

    /// Returns all nodes reachable from `start`.
    ///
    /// The starting node is not included in the result.
    ///
    /// The result is ordered deterministically.
    pub fn descendants(
        &self,
        start: &NodeId,
    ) -> Result<Vec<NodeId>, DifferentiationGraphError<NodeId>> {
        if !self.nodes.contains_key(start) {
            return Err(DifferentiationGraphError::MissingNode {
                node: start.clone(),
            });
        }

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start.clone());

        while let Some(current) = queue.pop_front() {
            if let Some(edges) = self.outgoing.get(&current) {
                for edge in edges {
                    if visited.insert(edge.target.clone()) {
                        queue.push_back(edge.target.clone());
                    }
                }
            }
        }

        visited.remove(start);

        Ok(visited.into_iter().collect())
    }

    /// Returns all ancestors of a node.
    ///
    /// The starting node is not included in the result.
    ///
    /// The result is ordered deterministically.
    pub fn ancestors(
        &self,
        start: &NodeId,
    ) -> Result<Vec<NodeId>, DifferentiationGraphError<NodeId>> {
        if !self.nodes.contains_key(start) {
            return Err(DifferentiationGraphError::MissingNode {
                node: start.clone(),
            });
        }

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start.clone());

        while let Some(current) = queue.pop_front() {
            if let Some(edges) = self.incoming.get(&current) {
                for edge in edges {
                    if visited.insert(edge.source.clone()) {
                        queue.push_back(edge.source.clone());
                    }
                }
            }
        }

        visited.remove(start);

        Ok(visited.into_iter().collect())
    }

    /// Returns whether `target` is reachable from `source`.
    ///
    /// This method returns false if either endpoint does not exist.
    #[must_use]
    pub fn reachable(
        &self,
        source: &NodeId,
        target: &NodeId,
    ) -> bool {
        if !self.nodes.contains_key(source)
            || !self.nodes.contains_key(target)
        {
            return false;
        }

        if source == target {
            return true;
        }

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        visited.insert(source.clone());
        queue.push_back(source.clone());

        while let Some(current) = queue.pop_front() {
            if let Some(edges) = self.outgoing.get(&current) {
                for edge in edges {
                    if &edge.target == target {
                        return true;
                    }

                    if visited.insert(edge.target.clone()) {
                        queue.push_back(edge.target.clone());
                    }
                }
            }
        }

        false
    }

    /// Computes a deterministic topological ordering.
    ///
    /// Nodes with no remaining dependencies are selected in identifier
    /// order. This produces a stable ordering for the same graph.
    ///
    /// A graph containing a cycle returns an explicit error.
    pub fn topological_order(
        &self,
    ) -> Result<Vec<NodeId>, DifferentiationGraphError<NodeId>> {
        let mut indegree: BTreeMap<NodeId, usize> = self
            .nodes
            .keys()
            .cloned()
            .map(|id| {
                let count = self
                    .incoming
                    .get(&id)
                    .map_or(0, BTreeSet::len);

                (id, count)
            })
            .collect();

        let mut ready: BTreeSet<NodeId> = indegree
            .iter()
            .filter_map(|(id, count)| {
                (*count == 0).then_some(id.clone())
            })
            .collect();

        let mut order = Vec::with_capacity(self.nodes.len());

        while let Some(next) = ready.pop_first() {
            order.push(next.clone());

            if let Some(edges) = self.outgoing.get(&next) {
                for edge in edges {
                    let count = indegree
                        .get_mut(&edge.target)
                        .ok_or_else(|| {
                            DifferentiationGraphError::InvariantViolation {
                                reason:
                                    "topological indegree entry is missing"
                                        .into(),
                            }
                        })?;

                    *count = count.checked_sub(1).ok_or_else(|| {
                        DifferentiationGraphError::InvariantViolation {
                            reason:
                                "topological indegree underflow".into(),
                        }
                    })?;

                    if *count == 0 {
                        ready.insert(edge.target.clone());
                    }
                }
            }
        }

        if order.len() != self.nodes.len() {
            return Err(DifferentiationGraphError::InvalidCycle {
                path: self.find_cycle()?,
            });
        }

        Ok(order)
    }

    /// Validates graph invariants and computes its topological order.
    ///
    /// This is structural validation only. It does not prove that
    /// operations are mathematically differentiable.
    pub fn validate(
        &self,
    ) -> Result<
        DifferentiationGraphValidation<NodeId>,
        DifferentiationGraphError<NodeId>,
    > {
        self.validate_adjacency()?;

        let order = self.topological_order()?;

        Ok(DifferentiationGraphValidation {
            node_count: self.node_count(),
            edge_count: self.edge_count(),
            topological_order: order,
        })
    }

    /// Validates that both adjacency maps are internally consistent.
    fn validate_adjacency(
        &self,
    ) -> Result<(), DifferentiationGraphError<NodeId>> {
        if self.nodes.len() != self.outgoing.len()
            || self.nodes.len() != self.incoming.len()
        {
            return Err(
                DifferentiationGraphError::InvariantViolation {
                    reason:
                        "node and adjacency map sizes differ".into(),
                },
            );
        }

        for id in self.nodes.keys() {
            if !self.outgoing.contains_key(id)
                || !self.incoming.contains_key(id)
            {
                return Err(
                    DifferentiationGraphError::InvariantViolation {
                        reason:
                            "a node is missing an adjacency entry".into(),
                    },
                );
            }
        }

        for edge in self.edges() {
            if !self.nodes.contains_key(&edge.source)
                || !self.nodes.contains_key(&edge.target)
            {
                return Err(
                    DifferentiationGraphError::InvariantViolation {
                        reason:
                            "an edge references a missing node".into(),
                    },
                );
            }

            let outgoing_contains = self
                .outgoing
                .get(&edge.source)
                .is_some_and(|edges| edges.contains(edge));

            let incoming_contains = self
                .incoming
                .get(&edge.target)
                .is_some_and(|edges| edges.contains(edge));

            if !outgoing_contains || !incoming_contains {
                return Err(
                    DifferentiationGraphError::InvariantViolation {
                        reason:
                            "edge adjacency maps are inconsistent".into(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Finds a deterministic cycle path.
    ///
    /// The returned path begins and ends at the same node.
    fn find_cycle(
        &self,
    ) -> Result<Vec<NodeId>, DifferentiationGraphError<NodeId>> {
        let mut color: BTreeMap<NodeId, u8> = self
            .nodes
            .keys()
            .cloned()
            .map(|id| (id, 0))
            .collect();

        let mut parent: BTreeMap<NodeId, NodeId> = BTreeMap::new();

        for start in self.nodes.keys() {
            if color.get(start).copied().unwrap_or(0) != 0 {
                continue;
            }

            let mut stack: Vec<(NodeId, Vec<NodeId>, usize)> =
                Vec::new();

            color.insert(start.clone(), 1);

            let neighbors = self
                .outgoing
                .get(start)
                .map(|edges| {
                    edges
                        .iter()
                        .map(|edge| edge.target.clone())
                        .collect()
                })
                .unwrap_or_default();

            stack.push((start.clone(), neighbors, 0));

            while let Some((node, neighbors, index)) = stack.last_mut() {
                if *index >= neighbors.len() {
                    color.insert(node.clone(), 2);
                    stack.pop();
                    continue;
                }

                let next = neighbors[*index].clone();
                *index += 1;

                match color.get(&next).copied().unwrap_or(0) {
                    0 => {
                        parent.insert(next.clone(), node.clone());
                        color.insert(next.clone(), 1);

                        let next_neighbors = self
                            .outgoing
                            .get(&next)
                            .map(|edges| {
                                edges
                                    .iter()
                                    .map(|edge| edge.target.clone())
                                    .collect()
                            })
                            .unwrap_or_default();

                        stack.push((next, next_neighbors, 0));
                    }

                    1 => {
                        let mut path = vec![next.clone()];
                        let mut current = node.clone();

                        while current != next {
                            path.push(current.clone());

                            current = parent
                                .get(&current)
                                .cloned()
                                .ok_or_else(|| {
                                    DifferentiationGraphError::
                                        InvariantViolation {
                                            reason:
                                                "cycle reconstruction \
                                                 failed".into(),
                                        }
                                })?;
                        }

                        path.push(next);
                        path.reverse();

                        return Ok(path);
                    }

                    _ => {}
                }
            }
        }

        Err(
            DifferentiationGraphError::InvariantViolation {
                reason:
                    "topological ordering failed but no cycle was found"
                        .into(),
            },
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    type Graph = DifferentiationGraph<u64, String>;

    fn node(
        id: u64,
        kind: DifferentiationNodeKind<String>,
    ) -> DifferentiationNode<u64, String> {
        DifferentiationNode::new(id, kind)
    }

    fn graph_with_nodes(ids: &[u64]) -> Graph {
        let mut graph = Graph::new();

        for id in ids {
            graph
                .add_node(node(*id, DifferentiationNodeKind::Intermediate))
                .expect("node insertion should succeed");
        }

        graph
    }

    #[test]
    fn creates_empty_graph() {
        let graph = Graph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn inserts_nodes() {
        let mut graph = Graph::new();

        graph
            .add_node(node(1, DifferentiationNodeKind::Input))
            .expect("node insertion should succeed");

        assert!(graph.contains_node(&1));
        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn rejects_duplicate_nodes() {
        let mut graph = graph_with_nodes(&[1]);

        let result = graph.add_node(node(
            1,
            DifferentiationNodeKind::Intermediate,
        ));

        assert!(matches!(
            result,
            Err(DifferentiationGraphError::DuplicateNode { node: 1 })
        ));

        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn inserts_dependency() {
        let mut graph = graph_with_nodes(&[1, 2]);

        graph
            .add_dependency(
                1,
                2,
                DifferentiationDependencyKind::Value,
            )
            .expect("dependency insertion should succeed");

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.dependencies(&2).unwrap(), vec![1]);
        assert_eq!(graph.dependents(&1).unwrap(), vec![2]);
    }

    #[test]
    fn rejects_missing_dependency_endpoint() {
        let mut graph = graph_with_nodes(&[1]);

        let result = graph.add_dependency(
            1,
            2,
            DifferentiationDependencyKind::Value,
        );

        assert!(matches!(
            result,
            Err(DifferentiationGraphError::MissingNode { node: 2 })
        ));

        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn rejects_self_dependency() {
        let mut graph = graph_with_nodes(&[1]);

        let result = graph.add_dependency(
            1,
            1,
            DifferentiationDependencyKind::Value,
        );

        assert!(matches!(
            result,
            Err(DifferentiationGraphError::CycleDetected { .. })
        ));

        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn rejects_cycle() {
        let mut graph = graph_with_nodes(&[1, 2, 3]);

        graph
            .add_dependency(
                1,
                2,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        graph
            .add_dependency(
                2,
                3,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        let result = graph.add_dependency(
            3,
            1,
            DifferentiationDependencyKind::Value,
        );

        assert!(matches!(
            result,
            Err(DifferentiationGraphError::CycleDetected { .. })
        ));

        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn topological_order_respects_dependencies() {
        let mut graph = graph_with_nodes(&[1, 2, 3, 4]);

        graph
            .add_dependency(
                1,
                3,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        graph
            .add_dependency(
                2,
                3,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        graph
            .add_dependency(
                3,
                4,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        let order = graph.topological_order().unwrap();

        let position = |id| {
            order
                .iter()
                .position(|candidate| *candidate == id)
                .unwrap()
        };

        assert!(position(1) < position(3));
        assert!(position(2) < position(3));
        assert!(position(3) < position(4));
    }

    #[test]
    fn topological_order_is_deterministic() {
        let mut graph = graph_with_nodes(&[4, 3, 2, 1]);

        graph
            .add_dependency(
                1,
                3,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        graph
            .add_dependency(
                2,
                3,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        assert_eq!(
            graph.topological_order().unwrap(),
            graph.topological_order().unwrap()
        );
    }

    #[test]
    fn finds_ancestors_and_descendants() {
        let mut graph = graph_with_nodes(&[1, 2, 3, 4]);

        graph
            .add_dependency(
                1,
                2,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        graph
            .add_dependency(
                2,
                3,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        graph
            .add_dependency(
                3,
                4,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        assert_eq!(graph.ancestors(&4).unwrap(), vec![1, 2, 3]);
        assert_eq!(graph.descendants(&1).unwrap(), vec![2, 3, 4]);
    }

    #[test]
    fn validates_graph() {
        let mut graph = graph_with_nodes(&[1, 2]);

        graph
            .add_dependency(
                1,
                2,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        let validation = graph.validate().unwrap();

        assert_eq!(validation.node_count(), 2);
        assert_eq!(validation.edge_count(), 1);
        assert_eq!(validation.topological_order(), &[1, 2]);
    }

    #[test]
    fn supports_fan_out() {
        let mut graph = graph_with_nodes(&[1, 2, 3]);

        graph
            .add_dependency(
                1,
                2,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        graph
            .add_dependency(
                1,
                3,
                DifferentiationDependencyKind::Value,
            )
            .unwrap();

        assert_eq!(graph.dependents(&1).unwrap(), vec![2, 3]);
    }
}
