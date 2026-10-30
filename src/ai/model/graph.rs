
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Model Graph
//!
//! File: src/ai/model/graph.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides an indexed, target-independent graph view over the canonical
//! AI model connections already declared in `model::definition`.
//!
//! A model graph represents source-level computational relationships
//! between model members.
//!
//! It does not represent executable IR, physical hardware topology,
//! memory placement, accelerator connectivity, network topology,
//! scheduling, or runtime execution.
//!
//! ## Architectural ownership
//!
//! Model source grammar:
//!     grammar/ai/models.g4
//!
//! Canonical model definition:
//!     crate::ai::model::definition
//!
//! Canonical member identity:
//!     crate::ai::model::definition::ModelMemberId
//!
//! Canonical connections:
//!     crate::ai::model::definition::ModelConnection
//!
//! Universal computational IR:
//!     crate::zuir
//!
//! Execution planning:
//!     compiler / distributed / runtime
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on:
//! - model members;
//! - graph edges;
//! - model depth;
//! - graph traversal depth;
//! - graph fan-in;
//! - graph fan-out;
//! - connected components.
//!
//! Actual limits are determined by available memory, addressability,
//! implementation resources, and explicit operational policies.
//!
//! ## Important graph semantics
//!
//! - Edges are directed.
//! - Parallel edges are preserved.
//! - Source ordering is preserved.
//! - Cycles are permitted in the model representation.
//! - Topological ordering is available only when the graph is acyclic.
//! - Graph traversal is iterative.
//! - Node identity is always the canonical ModelMemberId.
//!
//! ## Integration
//!
//! ModelGraph borrows an immutable ModelDefinition.
//!
//! It constructs indexes over the existing model members and connections.
//! It does not copy or independently own model declarations.
//!
//! The AI semantic subsystem remains responsible for name resolution,
//! type compatibility, connection legality, and semantic validation.
//!
//! ZUIR remains responsible for universal computational representation.
//!
//! ## Freeze criteria
//!
//! - ModelDefinition validation is successful.
//! - Node identities are unique.
//! - Every connection endpoint resolves.
//! - All graph indexes are consistent.
//! - Traversal is deterministic.
//! - Cycles are reported without corrupting graph state.
//! - Parallel edges are preserved.
//! - All tests pass.
//! - Workspace compilation and lint checks pass.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use super::definition::{
    ModelConnection,
    ModelDefinition,
    ModelDefinitionError,
    ModelMember,
    ModelMemberId,
};

// =============================================================================
// Errors
// =============================================================================

/// Errors encountered while constructing or querying a model graph.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ModelGraphError {
    /// The underlying canonical model definition is structurally invalid.
    InvalidDefinition(ModelDefinitionError),

    /// The requested member does not exist in this graph.
    UnknownMember(ModelMemberId),

    /// An allocation reservation could not be satisfied.
    ///
    /// This is an operational resource failure, not a model semantic error.
    AllocationFailed,

    /// An internal graph index could not be represented.
    ///
    /// This should not occur for a successfully constructed graph under
    /// the platform's addressability constraints.
    IndexOverflow,
}

impl fmt::Display for ModelGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDefinition(error) => {
                write!(f, "invalid AI model definition: {error}")
            }

            Self::UnknownMember(id) => {
                write!(f, "unknown AI model member: {}", id.as_str())
            }

            Self::AllocationFailed => {
                f.write_str("unable to reserve model graph storage")
            }

            Self::IndexOverflow => {
                f.write_str("model graph index overflow")
            }
        }
    }
}

impl std::error::Error for ModelGraphError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidDefinition(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ModelDefinitionError> for ModelGraphError {
    fn from(error: ModelDefinitionError) -> Self {
        Self::InvalidDefinition(error)
    }
}

// =============================================================================
// Topological ordering errors
// =============================================================================

/// Indicates that a model graph cannot be topologically ordered.
///
/// `blocked_members` contains members that were not emitted by Kahn's
/// algorithm because a directed cycle prevents a complete ordering.
///
/// Some blocked members may be downstream of a cycle rather than
/// members of the cycle itself.
///
/// This distinction prevents reporting an inaccurate cycle witness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelGraphCycle {
    blocked_members: Vec<ModelMemberId>,
}

impl ModelGraphCycle {
    /// Returns members that could not be placed in the topological order.
    #[must_use]
    pub fn blocked_members(&self) -> &[ModelMemberId] {
        &self.blocked_members
    }

    /// Returns true when at least one member is blocked.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocked_members.is_empty()
    }
}

impl fmt::Display for ModelGraphCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "model graph contains cyclic dependencies affecting {} members",
            self.blocked_members.len()
        )
    }
}

impl std::error::Error for ModelGraphCycle {}

// =============================================================================
// Graph representation
// =============================================================================

/// An indexed, immutable graph view over a canonical AI model.
///
/// The model definition remains the authoritative source of nodes and edges.
///
/// This structure owns only indexes and adjacency relationships.
///
/// The graph borrows its underlying model, preventing accidental divergence
/// between the graph and its canonical model definition.
///
/// The graph is not an executable intermediate representation.
#[derive(Debug)]
pub struct ModelGraph<'model> {
    model: &'model ModelDefinition,

    /// Canonical member identity -> position in ModelDefinition::members.
    node_indices: BTreeMap<ModelMemberId, usize>,

    /// Outgoing connection indices for each member.
    ///
    /// Connection indices refer directly to ModelDefinition::connections.
    outgoing: Vec<Vec<usize>>,

    /// Incoming connection indices for each member.
    incoming: Vec<Vec<usize>>,
}

// =============================================================================
// Construction
// =============================================================================

impl<'model> ModelGraph<'model> {
    /// Constructs an indexed graph from a canonical model definition.
    ///
    /// The definition is structurally validated before graph indexes
    /// are constructed.
    ///
    /// The graph preserves:
    /// - canonical node identity;
    /// - member source ordering;
    /// - connection source ordering;
    /// - parallel connections;
    /// - directed edge semantics.
    ///
    /// Cycles are not rejected during construction.
    ///
    /// # Errors
    ///
    /// Returns InvalidDefinition when the canonical model definition
    /// violates its structural invariants.
    ///
    /// Returns AllocationFailed when a fallible vector reservation fails.
    pub fn new(
        model: &'model ModelDefinition,
    ) -> Result<Self, ModelGraphError> {
        model.validate()?;

        let node_count = model.members.len();
        let edge_count = model.connections.len();

        // -----------------------------------------------------------------
        // Construct the canonical node index.
        // -----------------------------------------------------------------

        let mut node_indices = BTreeMap::new();

        for (index, member) in model.members.iter().enumerate() {
            node_indices.insert(member.id.clone(), index);
        }

        // -----------------------------------------------------------------
        // Allocate adjacency containers.
        // -----------------------------------------------------------------

        let mut outgoing = Vec::new();
        outgoing
            .try_reserve_exact(node_count)
            .map_err(|_| ModelGraphError::AllocationFailed)?;

        outgoing.resize_with(node_count, Vec::new);

        let mut incoming = Vec::new();
        incoming
            .try_reserve_exact(node_count)
            .map_err(|_| ModelGraphError::AllocationFailed)?;

        incoming.resize_with(node_count, Vec::new);

        // -----------------------------------------------------------------
        // Count degrees before allocating adjacency entries.
        //
        // This avoids repeated growth of the adjacency vectors.
        // -----------------------------------------------------------------

        let mut outgoing_counts = Vec::new();
        outgoing_counts
            .try_reserve_exact(node_count)
            .map_err(|_| ModelGraphError::AllocationFailed)?;

        outgoing_counts.resize(node_count, 0usize);

        let mut incoming_counts = Vec::new();
        incoming_counts
            .try_reserve_exact(node_count)
            .map_err(|_| ModelGraphError::AllocationFailed)?;

        incoming_counts.resize(node_count, 0usize);

        for connection in &model.connections {
            let source = node_indices
                .get(&connection.source.member)
                .ok_or_else(|| {
                    ModelGraphError::UnknownMember(
                        connection.source.member.clone(),
                    )
                })?;

            let destination = node_indices
                .get(&connection.destination.member)
                .ok_or_else(|| {
                    ModelGraphError::UnknownMember(
                        connection.destination.member.clone(),
                    )
                })?;

            outgoing_counts[*source] = outgoing_counts[*source]
                .checked_add(1)
                .ok_or(ModelGraphError::IndexOverflow)?;

            incoming_counts[*destination] = incoming_counts[*destination]
                .checked_add(1)
                .ok_or(ModelGraphError::IndexOverflow)?;
        }

        // -----------------------------------------------------------------
        // Reserve adjacency storage.
        // -----------------------------------------------------------------

        for index in 0..node_count {
            outgoing[index]
                .try_reserve_exact(outgoing_counts[index])
                .map_err(|_| ModelGraphError::AllocationFailed)?;

            incoming[index]
                .try_reserve_exact(incoming_counts[index])
                .map_err(|_| ModelGraphError::AllocationFailed)?;
        }

        // -----------------------------------------------------------------
        // Populate adjacency indexes.
        //
        // Each connection is stored exactly once in its source adjacency
        // and once in its destination adjacency.
        //
        // The original connection index is retained.
        // -----------------------------------------------------------------

        for (edge_index, connection) in
            model.connections.iter().enumerate()
        {
            let source = node_indices
                .get(&connection.source.member)
                .ok_or_else(|| {
                    ModelGraphError::UnknownMember(
                        connection.source.member.clone(),
                    )
                })?;

            let destination = node_indices
                .get(&connection.destination.member)
                .ok_or_else(|| {
                    ModelGraphError::UnknownMember(
                        connection.destination.member.clone(),
                    )
                })?;

            outgoing[*source].push(edge_index);
            incoming[*destination].push(edge_index);
        }

        debug_assert_eq!(
            outgoing.iter().map(Vec::len).sum::<usize>(),
            edge_count
        );

        debug_assert_eq!(
            incoming.iter().map(Vec::len).sum::<usize>(),
            edge_count
        );

        Ok(Self {
            model,
            node_indices,
            outgoing,
            incoming,
        })
    }

    // ---------------------------------------------------------------------
    // Graph metadata
    // ---------------------------------------------------------------------

    /// Returns the number of model members.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.model.members.len()
    }

    /// Returns the number of directed connections.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.model.connections.len()
    }

    /// Returns true when the graph has no members.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.model.members.is_empty()
    }

    /// Returns true when the graph contains no connections.
    #[must_use]
    pub fn has_no_edges(&self) -> bool {
        self.model.connections.is_empty()
    }

    /// Borrows the canonical model definition.
    #[must_use]
    pub fn model(&self) -> &'model ModelDefinition {
        self.model
    }

    // ---------------------------------------------------------------------
    // Node access
    // ---------------------------------------------------------------------

    /// Returns a member by its canonical identity.
    #[must_use]
    pub fn node(
        &self,
        id: &ModelMemberId,
    ) -> Option<&'model ModelMember> {
        self.node_indices
            .get(id)
            .map(|index| &self.model.members[*index])
    }

    /// Returns all graph nodes in canonical model source order.
    pub fn nodes(
        &self,
    ) -> impl Iterator<Item = &'model ModelMember> + '_ {
        self.model.members.iter()
    }

    /// Returns all node identities in canonical model source order.
    pub fn node_ids(
        &self,
    ) -> impl Iterator<Item = &ModelMemberId> + '_ {
        self.model.members.iter().map(|member| &member.id)
    }

    /// Returns the number of incoming edges for a member.
    ///
    /// Parallel edges are counted independently.
    ///
    /// Returns None when the requested member does not exist.
    #[must_use]
    pub fn in_degree(
        &self,
        id: &ModelMemberId,
    ) -> Option<usize> {
        let index = *self.node_indices.get(id)?;
        Some(self.incoming[index].len())
    }

    /// Returns the number of outgoing edges for a member.
    ///
    /// Parallel edges are counted independently.
    ///
    /// Returns None when the requested member does not exist.
    #[must_use]
    pub fn out_degree(
        &self,
        id: &ModelMemberId,
    ) -> Option<usize> {
        let index = *self.node_indices.get(id)?;
        Some(self.outgoing[index].len())
    }

    // ---------------------------------------------------------------------
    // Edge access
    // ---------------------------------------------------------------------

    /// Returns all canonical connections in source order.
    pub fn edges(
        &self,
    ) -> impl Iterator<Item = &'model ModelConnection> + '_ {
        self.model.connections.iter()
    }

    /// Returns a connection by its canonical source-order index.
    #[must_use]
    pub fn edge(
        &self,
        index: usize,
    ) -> Option<&'model ModelConnection> {
        self.model.connections.get(index)
    }

    /// Returns outgoing connections for a member.
    ///
    /// The returned connections preserve their order in the original
    /// ModelDefinition::connections collection.
    ///
    /// Returns None when the member does not exist.
    pub fn outgoing(
        &self,
        id: &ModelMemberId,
    ) -> Option<
        impl Iterator<Item = &'model ModelConnection> + '_,
    > {
        let index = *self.node_indices.get(id)?;

        Some(
            self.outgoing[index]
                .iter()
                .map(move |edge_index| {
                    &self.model.connections[*edge_index]
                }),
        )
    }

    /// Returns incoming connections for a member.
    ///
    /// The returned connections preserve their order in the original
    /// ModelDefinition::connections collection.
    ///
    /// Returns None when the member does not exist.
    pub fn incoming(
        &self,
        id: &ModelMemberId,
    ) -> Option<
        impl Iterator<Item = &'model ModelConnection> + '_,
    > {
        let index = *self.node_indices.get(id)?;

        Some(
            self.incoming[index]
                .iter()
                .map(move |edge_index| {
                    &self.model.connections[*edge_index]
                }),
        )
    }

    // ---------------------------------------------------------------------
    // Graph traversal
    // ---------------------------------------------------------------------

    /// Returns members reachable from a starting member by following
    /// outgoing directed connections.
    ///
    /// The starting member is included in the result.
    ///
    /// Each member is returned at most once.
    ///
    /// Traversal is iterative and does not consume call-stack depth.
    ///
    /// Results are deterministic and follow connection source ordering.
    pub fn reachable_from(
        &self,
        start: &ModelMemberId,
    ) -> Result<Vec<&'model ModelMember>, ModelGraphError> {
        let start_index = *self
            .node_indices
            .get(start)
            .ok_or_else(|| ModelGraphError::UnknownMember(start.clone()))?;

        let node_count = self.node_count();

        let mut visited = Vec::new();
        visited
            .try_reserve_exact(node_count)
            .map_err(|_| ModelGraphError::AllocationFailed)?;

        visited.resize(node_count, false);

        let mut queue = VecDeque::new();
        queue
            .try_reserve(node_count)
            .map_err(|_| ModelGraphError::AllocationFailed)?;

        let mut result = Vec::new();
        result
            .try_reserve(node_count)
            .map_err(|_| ModelGraphError::AllocationFailed)?;

        visited[start_index] = true;
        queue.push_back(start_index);

        while let Some(current) = queue.pop_front() {
            result.push(&self.model.members[current]);

            for edge_index in &self.outgoing[current] {
                let connection = &self.model.connections[*edge_index];

                let next = *self
                    .node_indices
                    .get(&connection.destination.member)
                    .ok_or_else(|| {
                        ModelGraphError::UnknownMember(
                            connection.destination.member.clone(),
                        )
                    })?;

                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }

        Ok(result)
    }

    /// Returns members that can reach the supplied member by following
    /// incoming directed connections in reverse.
    ///
    /// The starting member is included.
    ///
    /// Each member is returned at most once.
    ///
    /// Traversal is iterative and deterministic.
    pub fn ancestors_of(
        &self,
        start: &ModelMemberId,
    ) -> Result<Vec<&'model ModelMember>, ModelGraphError> {
        let start_index = *self
            .node_indices
            .get(start)
            .ok_or_else(|| ModelGraphError::UnknownMember(start.clone()))?;

        let node_count = self.node_count();

        let mut visited = Vec::new();
        visited
            .try_reserve_exact(node_count)
            .map_err(|_| ModelGraphError::AllocationFailed)?;

        visited.resize(node_count, false);

        let mut queue = VecDeque::new();
        queue
            .try_reserve(node_count)
            .map_err(|_| ModelGraphError::AllocationFailed)?;

        let mut result = Vec::new();
        result
            .try_reserve(node_count)
            .map_err(|_| ModelGraphError::AllocationFailed)?;

        visited[start_index] = true;
        queue.push_back(start_index);

        while let Some(current) = queue.pop_front() {
            result.push(&self.model.members[current]);

            for edge_index in &self.incoming[current] {
                let connection = &self.model.connections[*edge_index];

                let previous = *self
                    .node_indices
                    .get(&connection.source.member)
                    .ok_or_else(|| {
                        ModelGraphError::UnknownMember(
                            connection.source.member.clone(),
                        )
                    })?;

                if !visited[previous] {
                    visited[previous] = true;
                    queue.push_back(previous);
                }
            }
        }

        Ok(result)
    }

    // ---------------------------------------------------------------------
    // Topological ordering
    // ---------------------------------------------------------------------

    /// Computes a deterministic topological ordering.
    ///
    /// Kahn's algorithm is used.
    ///
    /// Members with equal readiness are selected by canonical member
    /// identity ordering, not hash-map iteration order.
    ///
    /// Parallel connections are handled independently.
    ///
    /// A directed cycle prevents a complete topological ordering.
    /// In that case ModelGraphCycle is returned.
    ///
    /// This method does not mutate the graph or the underlying model.
    pub fn topological_order(
        &self,
    ) -> Result<Vec<&'model ModelMember>, ModelGraphCycle> {
        let node_count = self.node_count();

        let mut indegree = Vec::new();

        if indegree.try_reserve_exact(node_count).is_err() {
            return Err(ModelGraphCycle {
                blocked_members: self
                    .model
                    .members
                    .iter()
                    .map(|member| member.id.clone())
                    .collect(),
            });
        }

        indegree.extend(
            self.incoming.iter().map(Vec::len)
        );

        let mut ready = BTreeSet::new();

        for (index, degree) in indegree.iter().enumerate() {
            if *degree == 0 {
                ready.insert((
                    self.model.members[index].id.clone(),
                    index,
                ));
            }
        }

        let mut result = Vec::new();

        if result.try_reserve_exact(node_count).is_err() {
            return Err(ModelGraphCycle {
                blocked_members: self
                    .model
                    .members
                    .iter()
                    .map(|member| member.id.clone())
                    .collect(),
            });
        }

        while let Some((_, current)) = ready.pop_first() {
            result.push(&self.model.members[current]);

            for edge_index in &self.outgoing[current] {
                let connection = &self.model.connections[*edge_index];

                let Some(destination) = self
                    .node_indices
                    .get(&connection.destination.member)
                    .copied()
                else {
                    continue;
                };

                // The graph constructor guarantees a consistent index.
                // A zero degree here would indicate internal corruption.
                if indegree[destination] == 0 {
                    continue;
                }

                indegree[destination] -= 1;

                if indegree[destination] == 0 {
                    ready.insert((
                        self.model.members[destination].id.clone(),
                        destination,
                    ));
                }
            }
        }

        if result.len() == node_count {
            return Ok(result);
        }

        let blocked_members = self
            .model
            .members
            .iter()
            .enumerate()
            .filter(|(index, _)| indegree[*index] != 0)
            .map(|(_, member)| member.id.clone())
            .collect();

        Err(ModelGraphCycle { blocked_members })
    }

    // ---------------------------------------------------------------------
    // Structural graph properties
    // ---------------------------------------------------------------------

    /// Returns true when the graph contains a directed cycle.
    ///
    /// This checks whether a complete topological ordering exists.
    ///
    /// It does not mutate the graph.
    pub fn has_cycle(&self) -> bool {
        self.topological_order().is_err()
    }

    /// Returns members that have no incoming connections.
    ///
    /// These are graph sources, not necessarily model input declarations.
    pub fn sources(
        &self,
    ) -> impl Iterator<Item = &'model ModelMember> + '_ {
        self.model
            .members
            .iter()
            .enumerate()
            .filter(|(index, _)| self.incoming[*index].is_empty())
            .map(|(_, member)| member)
    }

    /// Returns members that have no outgoing connections.
    ///
    /// These are graph sinks, not necessarily model output declarations.
    pub fn sinks(
        &self,
    ) -> impl Iterator<Item = &'model ModelMember> + '_ {
        self.model
            .members
            .iter()
            .enumerate()
            .filter(|(index, _)| self.outgoing[*index].is_empty())
            .map(|(_, member)| member)
    }

    /// Returns true when a member has neither incoming nor outgoing edges.
    ///
    /// Returns None when the requested member does not exist.
    #[must_use]
    pub fn is_isolated(
        &self,
        id: &ModelMemberId,
    ) -> Option<bool> {
        let index = *self.node_indices.get(id)?;

        Some(
            self.incoming[index].is_empty()
                && self.outgoing[index].is_empty(),
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::model::definition::{
        ModelEndpoint,
        ModelMemberPayload,
        ModelMemberRole,
    };

    use crate::frontend::semantic::types::ty::{
        QualifiedName,
        SemanticName,
        SemanticType,
    };

    fn name(value: &str) -> SemanticName {
        SemanticName::new(value)
    }

    fn qualified(value: &str) -> QualifiedName {
        QualifiedName::single(value)
    }

    fn id(value: &str) -> ModelMemberId {
        ModelMemberId::new(value).expect("valid member identity")
    }

    fn member(value: &str) -> ModelMember {
        ModelMember::new(
            id(value),
            name(value),
            ModelMemberRole::Input,
            ModelMemberPayload::Typed {
                ty: SemanticType::Unit,
                initializer: None,
            },
        )
    }

    fn connection(
        source: &str,
        source_port: &str,
        destination: &str,
        destination_port: &str,
    ) -> ModelConnection {
        ModelConnection {
            source: ModelEndpoint::new(
                id(source),
                name(source_port),
            ),
            destination: ModelEndpoint::new(
                id(destination),
                name(destination_port),
            ),
        }
    }

    fn model() -> ModelDefinition {
        ModelDefinition::new(qualified("TestModel"))
    }

    fn chain() -> ModelDefinition {
        model()
            .with_member(member("a"))
            .with_member(member("b"))
            .with_member(member("c"))
            .with_connection(connection("a", "out", "b", "in"))
            .with_connection(connection("b", "out", "c", "in"))
    }

    // ---------------------------------------------------------------------
    // Construction
    // ---------------------------------------------------------------------

    #[test]
    fn constructs_empty_graph() {
        let definition = model();
        let graph = ModelGraph::new(&definition).unwrap();

        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
        assert!(graph.has_no_edges());
        assert!(!graph.has_cycle());
    }

    #[test]
    fn constructs_graph_from_canonical_model() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn rejects_invalid_model_definition() {
        let definition = model()
            .with_member(member("a"))
            .with_member(member("a"));

        assert!(matches!(
            ModelGraph::new(&definition),
            Err(ModelGraphError::InvalidDefinition(
                ModelDefinitionError::DuplicateMemberIdentity
            ))
        ));
    }

    // ---------------------------------------------------------------------
    // Node and edge access
    // ---------------------------------------------------------------------

    #[test]
    fn finds_nodes_by_canonical_identity() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        assert!(graph.node(&id("a")).is_some());
        assert!(graph.node(&id("b")).is_some());
        assert!(graph.node(&id("missing")).is_none());
    }

    #[test]
    fn preserves_source_node_order() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        let names = graph
            .nodes()
            .map(|member| member.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn preserves_connection_source_order() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        let sources = graph
            .edges()
            .map(|edge| edge.source.member.as_str().to_owned())
            .collect::<Vec<_>>();

        assert_eq!(sources, vec!["a", "b"]);
    }

    #[test]
    fn calculates_node_degrees() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        assert_eq!(graph.in_degree(&id("a")), Some(0));
        assert_eq!(graph.out_degree(&id("a")), Some(1));

        assert_eq!(graph.in_degree(&id("b")), Some(1));
        assert_eq!(graph.out_degree(&id("b")), Some(1));

        assert_eq!(graph.in_degree(&id("c")), Some(1));
        assert_eq!(graph.out_degree(&id("c")), Some(0));

        assert_eq!(graph.in_degree(&id("missing")), None);
    }

    #[test]
    fn returns_incoming_and_outgoing_edges() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        assert_eq!(
            graph.outgoing(&id("a")).unwrap().count(),
            1
        );

        assert_eq!(
            graph.incoming(&id("c")).unwrap().count(),
            1
        );

        assert!(graph.outgoing(&id("missing")).is_none());
    }

    // ---------------------------------------------------------------------
    // Parallel connections
    // ---------------------------------------------------------------------

    #[test]
    fn preserves_parallel_connections() {
        let definition = model()
            .with_member(member("a"))
            .with_member(member("b"))
            .with_connection(connection("a", "first", "b", "first"))
            .with_connection(connection("a", "second", "b", "second"));

        let graph = ModelGraph::new(&definition).unwrap();

        assert_eq!(graph.edge_count(), 2);
        assert_eq!(graph.out_degree(&id("a")), Some(2));
        assert_eq!(graph.in_degree(&id("b")), Some(2));

        assert_eq!(
            graph.outgoing(&id("a")).unwrap().count(),
            2
        );

        assert!(graph.topological_order().is_ok());
    }

    // ---------------------------------------------------------------------
    // Reachability
    // ---------------------------------------------------------------------

    #[test]
    fn finds_reachable_members() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        let reachable = graph
            .reachable_from(&id("a"))
            .unwrap()
            .into_iter()
            .map(|member| member.id.as_str().to_owned())
            .collect::<Vec<_>>();

        assert_eq!(reachable, vec!["a", "b", "c"]);
    }

    #[test]
    fn reachability_includes_starting_member() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        let reachable = graph
            .reachable_from(&id("b"))
            .unwrap();

        assert_eq!(reachable.len(), 2);
        assert_eq!(reachable[0].id.as_str(), "b");
    }

    #[test]
    fn reverse_traversal_finds_ancestors() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        let ancestors = graph
            .ancestors_of(&id("c"))
            .unwrap()
            .into_iter()
            .map(|member| member.id.as_str().to_owned())
            .collect::<Vec<_>>();

        assert_eq!(ancestors, vec!["c", "b", "a"]);
    }

    #[test]
    fn rejects_unknown_traversal_start() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        assert!(matches!(
            graph.reachable_from(&id("missing")),
            Err(ModelGraphError::UnknownMember(_))
        ));
    }

    // ---------------------------------------------------------------------
    // Topological ordering
    // ---------------------------------------------------------------------

    #[test]
    fn calculates_topological_order_for_dag() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        let order = graph.topological_order().unwrap();

        assert_eq!(
            order
                .iter()
                .map(|member| member.id.as_str())
                .collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );
    }

    #[test]
    fn detects_directed_cycles() {
        let definition = model()
            .with_member(member("a"))
            .with_member(member("b"))
            .with_connection(connection("a", "out", "b", "in"))
            .with_connection(connection("b", "out", "a", "in"));

        let graph = ModelGraph::new(&definition).unwrap();

        assert!(graph.has_cycle());
        assert!(graph.topological_order().is_err());
    }

    #[test]
    fn preserves_cyclic_model_definition() {
        let definition = model()
            .with_member(member("a"))
            .with_member(member("b"))
            .with_connection(connection("a", "out", "b", "in"))
            .with_connection(connection("b", "out", "a", "in"));

        let graph = ModelGraph::new(&definition).unwrap();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn reports_members_blocked_by_cycles() {
        let definition = model()
            .with_member(member("a"))
            .with_member(member("b"))
            .with_member(member("c"))
            .with_member(member("d"))
            .with_connection(connection("a", "out", "b", "in"))
            .with_connection(connection("b", "out", "a", "in"))
            .with_connection(connection("b", "out", "c", "in"))
            .with_connection(connection("c", "out", "d", "in"));

        let graph = ModelGraph::new(&definition).unwrap();

        let cycle = graph.topological_order().unwrap_err();

        assert_eq!(cycle.blocked_members().len(), 4);
    }

    #[test]
    fn topological_order_handles_disconnected_components() {
        let definition = model()
            .with_member(member("a"))
            .with_member(member("b"))
            .with_member(member("c"))
            .with_member(member("d"))
            .with_connection(connection("a", "out", "b", "in"));

        let graph = ModelGraph::new(&definition).unwrap();

        let order = graph.topological_order().unwrap();

        assert_eq!(order.len(), 4);
    }

    // ---------------------------------------------------------------------
    // Sources, sinks and isolated members
    // ---------------------------------------------------------------------

    #[test]
    fn identifies_graph_sources_and_sinks() {
        let definition = chain();
        let graph = ModelGraph::new(&definition).unwrap();

        let sources = graph
            .sources()
            .map(|member| member.id.as_str())
            .collect::<Vec<_>>();

        let sinks = graph
            .sinks()
            .map(|member| member.id.as_str())
            .collect::<Vec<_>>();

        assert_eq!(sources, vec!["a"]);
        assert_eq!(sinks, vec!["c"]);
    }

    #[test]
    fn identifies_isolated_members() {
        let definition = model()
            .with_member(member("connected"))
            .with_member(member("destination"))
            .with_member(member("isolated"))
            .with_connection(connection(
                "connected",
                "out",
                "destination",
                "in",
            ));

        let graph = ModelGraph::new(&definition).unwrap();

        assert_eq!(graph.is_isolated(&id("isolated")), Some(true));
        assert_eq!(graph.is_isolated(&id("connected")), Some(false));
        assert_eq!(graph.is_isolated(&id("missing")), None);
    }

    // ---------------------------------------------------------------------
    // Self connections
    // ---------------------------------------------------------------------

    #[test]
    fn detects_self_loop_as_cycle() {
        let definition = model()
            .with_member(member("a"))
            .with_connection(connection("a", "first", "a", "second"));

        let graph = ModelGraph::new(&definition).unwrap();

        assert!(graph.has_cycle());
    }

    // ---------------------------------------------------------------------
    // Scalability properties
    // ---------------------------------------------------------------------

    #[test]
    fn supports_many_nodes_without_language_level_limits() {
        let mut definition = model();

        let count = 2_000usize;

        for index in 0..count {
            definition = definition.with_member(member(
                &format!("member_{index}")
            ));
        }

        let graph = ModelGraph::new(&definition).unwrap();

        assert_eq!(graph.node_count(), count);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn supports_large_logical_graphs_without_recursive_traversal() {
        let count = 2_000usize;

        let mut definition = model();

        for index in 0..count {
            definition = definition.with_member(member(
                &format!("member_{index}")
            ));
        }

        for index in 0..count - 1 {
            definition = definition.with_connection(connection(
                &format!("member_{index}"),
                "out",
                &format!("member_{}", index + 1),
                "in",
            ));
        }

        let graph = ModelGraph::new(&definition).unwrap();

        assert_eq!(graph.node_count(), count);
        assert_eq!(graph.edge_count(), count - 1);

        let reachable = graph
            .reachable_from(&id("member_0"))
            .unwrap();

        assert_eq!(reachable.len(), count);

        assert!(!graph.has_cycle());
    }

    #[test]
    fn graph_does_not_change_canonical_model() {
        let definition = chain();

        let original_member_count = definition.member_count();
        let original_connection_count = definition.connection_count();

        {
            let graph = ModelGraph::new(&definition).unwrap();

            assert_eq!(graph.node_count(), original_member_count);
            assert_eq!(graph.edge_count(), original_connection_count);
        }

        assert_eq!(definition.member_count(), original_member_count);
        assert_eq!(definition.connection_count(), original_connection_count);
    }
}
