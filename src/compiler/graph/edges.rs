
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Graph Edges.
//!
//! Path: src/compiler/graph/edges.rs
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Defines typed, directed relationships between nodes in a
//! CompilationGraph.
//!
//! This module extends the structural graph model in `model.rs`.
//!
//! It does not define another graph, IR, semantic model, dependency
//! authority, hardware model, scheduler or runtime.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - CompilationGraphEdgeId.
//! - CompilationGraphEdgeKind.
//! - CompilationGraphEdge.
//! - CompilationGraphEdgeSet.
//! - Edge-local validation.
//! - Graph-scoped edge validation.
//! - Deterministic incoming/outgoing indexes.
//!
//! # Non-responsibilities
//!
//! This module does not own:
//!
//! - Source grammar.
//! - AST construction.
//! - Semantic dependency discovery.
//! - Pipeline-stage ordering.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - AI or tensor semantics.
//! - Hybrid execution semantics.
//! - Distributed communication semantics.
//! - Physical routing.
//! - Hardware allocation.
//! - Runtime scheduling.
//! - Backend implementation.
//! - Execution.
//!
//! # Important
//!
//! Structural edge validity does not establish semantic validity.
//!
//! A graph edge may be structurally valid while its relationship is
//! semantically invalid for a particular computation domain.
//!
//! Domain-specific verification remains the responsibility of the
//! authoritative semantic and IR subsystems.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use super::model::{
    CompilationGraph,
    CompilationGraphId,
    CompilationGraphNodeId,
    GraphModelError,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of compiler graph-edge infrastructure.
pub const COMPILATION_GRAPH_EDGES_API_VERSION: u16 = 1;

/// Serialized edge-set schema version.
pub const COMPILATION_GRAPH_EDGES_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for graph-edge operations.
pub type GraphEdgesResult<T> = Result<T, GraphEdgesError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compiler graph-edge operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GraphEdgesError {
    /// An identifier is empty or whitespace-only.
    EmptyIdentifier {
        field: &'static str,
    },

    /// An identifier contains prohibited control characters.
    InvalidIdentifier {
        field: &'static str,
    },

    /// A graph edge identifier is duplicated.
    DuplicateEdge {
        edge_id: String,
    },

    /// The source node does not exist in the associated graph.
    MissingSourceNode {
        edge_id: String,
        node_id: String,
    },

    /// The destination node does not exist in the associated graph.
    MissingTargetNode {
        edge_id: String,
        node_id: String,
    },

    /// An edge set belongs to a different graph.
    GraphIdentityMismatch {
        expected: String,
        actual: String,
    },

    /// An unsupported schema representation was supplied.
    InvalidSchemaVersion {
        version: u16,
    },

    /// The derived edge indexes are inconsistent.
    InvalidEdgeIndex,

    /// The derived outgoing index is inconsistent.
    InvalidOutgoingIndex,

    /// The derived incoming index is inconsistent.
    InvalidIncomingIndex,

    /// Metadata contains an invalid key.
    InvalidMetadataKey,

    /// Metadata contains an invalid value.
    InvalidMetadataValue,

    /// An accounting operation exceeded its representation.
    SizeOverflow,

    /// The edge collection is structurally invalid.
    InvalidStructure {
        description: String,
    },

    /// A graph-model validation error occurred.
    GraphModel {
        source: GraphModelError,
    },
}

impl fmt::Display for GraphEdgesError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::InvalidIdentifier { field } => {
                write!(
                    formatter,
                    "{field} contains prohibited control characters"
                )
            }

            Self::DuplicateEdge { edge_id } => {
                write!(
                    formatter,
                    "duplicate compilation graph edge: {edge_id}"
                )
            }

            Self::MissingSourceNode {
                edge_id,
                node_id,
            } => {
                write!(
                    formatter,
                    "edge {edge_id} references missing source node {node_id}"
                )
            }

            Self::MissingTargetNode {
                edge_id,
                node_id,
            } => {
                write!(
                    formatter,
                    "edge {edge_id} references missing target node {node_id}"
                )
            }

            Self::GraphIdentityMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "edge-set graph identity mismatch: expected {expected}, got {actual}"
                )
            }

            Self::InvalidSchemaVersion { version } => {
                write!(
                    formatter,
                    "invalid graph-edge schema version: {version}"
                )
            }

            Self::InvalidEdgeIndex => {
                formatter.write_str(
                    "compilation graph edge index is inconsistent"
                )
            }

            Self::InvalidOutgoingIndex => {
                formatter.write_str(
                    "compilation graph outgoing adjacency index is inconsistent"
                )
            }

            Self::InvalidIncomingIndex => {
                formatter.write_str(
                    "compilation graph incoming adjacency index is inconsistent"
                )
            }

            Self::InvalidMetadataKey => {
                formatter.write_str(
                    "compilation graph edge metadata contains an invalid key"
                )
            }

            Self::InvalidMetadataValue => {
                formatter.write_str(
                    "compilation graph edge metadata contains an invalid value"
                )
            }

            Self::SizeOverflow => {
                formatter.write_str(
                    "compilation graph edge accounting overflow"
                )
            }

            Self::InvalidStructure { description } => {
                write!(
                    formatter,
                    "invalid compilation graph edge structure: {description}"
                )
            }

            Self::GraphModel { source } => {
                write!(formatter, "graph model error: {source}")
            }
        }
    }
}

impl Error for GraphEdgesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::GraphModel { source } => Some(source),
            _ => None,
        }
    }
}

impl From<GraphModelError> for GraphEdgesError {
    fn from(source: GraphModelError) -> Self {
        Self::GraphModel { source }
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> GraphEdgesResult<()> {
    if value.trim().is_empty() {
        return Err(GraphEdgesError::EmptyIdentifier {
            field,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(GraphEdgesError::InvalidIdentifier {
            field,
        });
    }

    Ok(())
}

fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> GraphEdgesResult<()> {
    for (key, value) in metadata {
        if key.trim().is_empty()
            || key.chars().any(char::is_control)
        {
            return Err(GraphEdgesError::InvalidMetadataKey);
        }

        if value.chars().any(char::is_control) {
            return Err(GraphEdgesError::InvalidMetadataValue);
        }
    }

    Ok(())
}

fn checked_add(
    left: u128,
    right: u128,
) -> GraphEdgesResult<u128> {
    left.checked_add(right)
        .ok_or(GraphEdgesError::SizeOverflow)
}

// ============================================================================
// EDGE IDENTITY
// ============================================================================

/// Unique logical identity of an edge within a compilation graph.
///
/// Edge identities are opaque.
///
/// They are not:
///
/// - Array positions.
/// - Memory addresses.
/// - Hardware identifiers.
/// - Content hashes.
/// - Node identifiers.
///
/// An edge identity is unique within its associated edge set.
///
/// No arbitrary identifier-length ceiling is imposed.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "String")]
pub struct CompilationGraphEdgeId(String);

impl TryFrom<String> for CompilationGraphEdgeId {
    type Error = GraphEdgesError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CompilationGraphEdgeId {
    /// Construct a validated edge identity.
    pub fn new(
        value: impl Into<String>,
    ) -> GraphEdgesResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "compilation graph edge identity",
        )?;

        Ok(Self(value))
    }

    /// Return the edge identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CompilationGraphEdgeId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// EXTENSIBLE EDGE CLASSIFICATION
// ============================================================================

/// Open-world classification of a graph relationship.
///
/// This is intentionally string-backed.
///
/// It is not a closed enumeration of relationship types.
///
/// Examples include:
///
/// - compiler.dependency
/// - compiler.stage_order
/// - compiler.artifact_input
/// - compiler.artifact_output
/// - dependency.data
/// - dependency.control
/// - dependency.effect
/// - dependency.ordering
/// - dependency.completion
/// - dependency.feedback
/// - dependency.synchronization
/// - domain.boundary
/// - domain.interoperation
/// - distributed.communication
/// - distributed.consistency
///
/// These identifiers are examples, not a fixed supported inventory.
///
/// New domains and third-party extensions may define additional kinds
/// without modifying this type.
///
/// The meaning of a kind belongs to its authoritative consumer.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "String")]
pub struct CompilationGraphEdgeKind(String);

impl TryFrom<String> for CompilationGraphEdgeKind {
    type Error = GraphEdgesError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CompilationGraphEdgeKind {
    /// Construct an extensible edge classification.
    pub fn new(
        value: impl Into<String>,
    ) -> GraphEdgesResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "compilation graph edge kind",
        )?;

        Ok(Self(value))
    }

    /// Return the classification identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CompilationGraphEdgeKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// GRAPH EDGE
// ============================================================================

/// A directed, typed relationship between two compilation graph nodes.
///
/// Direction is always:
///
///     source -> target
///
/// This is structural direction only.
///
/// The semantic meaning of that direction depends on the edge kind.
///
/// Parallel edges are permitted.
///
/// Cycles are permitted.
///
/// Self-referential edges are permitted at this structural layer.
///
/// The relevant semantic verifier determines whether those relationships
/// are valid for a particular graph use case.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "CompilationGraphEdgeWire")]
pub struct CompilationGraphEdge {
    id: CompilationGraphEdgeId,
    kind: CompilationGraphEdgeKind,
    source: CompilationGraphNodeId,
    target: CompilationGraphNodeId,
    metadata: BTreeMap<String, String>,
}

/// Wire representation for validated deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CompilationGraphEdgeWire {
    id: CompilationGraphEdgeId,
    kind: CompilationGraphEdgeKind,
    source: CompilationGraphNodeId,
    target: CompilationGraphNodeId,

    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl TryFrom<CompilationGraphEdgeWire> for CompilationGraphEdge {
    type Error = GraphEdgesError;

    fn try_from(
        value: CompilationGraphEdgeWire,
    ) -> Result<Self, Self::Error> {
        Self::new(
            value.id,
            value.kind,
            value.source,
            value.target,
            value.metadata,
        )
    }
}

impl CompilationGraphEdge {
    /// Construct a validated directed graph edge.
    ///
    /// This validates edge-local data.
    ///
    /// It does not establish that source and target nodes exist.
    /// Use `CompilationGraphEdgeSet::new` or
    /// `CompilationGraphEdgeSet::validate_against` for that.
    pub fn new(
        id: CompilationGraphEdgeId,
        kind: CompilationGraphEdgeKind,
        source: CompilationGraphNodeId,
        target: CompilationGraphNodeId,
        metadata: BTreeMap<String, String>,
    ) -> GraphEdgesResult<Self> {
        let edge = Self {
            id,
            kind,
            source,
            target,
            metadata,
        };

        edge.validate()?;

        Ok(edge)
    }

    /// Return the edge identity.
    #[must_use]
    pub fn id(&self) -> &CompilationGraphEdgeId {
        &self.id
    }

    /// Return the relationship classification.
    #[must_use]
    pub fn kind(&self) -> &CompilationGraphEdgeKind {
        &self.kind
    }

    /// Return the source node identity.
    #[must_use]
    pub fn source(&self) -> &CompilationGraphNodeId {
        &self.source
    }

    /// Return the target node identity.
    #[must_use]
    pub fn target(&self) -> &CompilationGraphNodeId {
        &self.target
    }

    /// Return deterministic descriptive metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validate all edge-local invariants.
    pub fn validate(&self) -> GraphEdgesResult<()> {
        validate_identifier(
            self.id.as_str(),
            "compilation graph edge identity",
        )?;

        validate_identifier(
            self.kind.as_str(),
            "compilation graph edge kind",
        )?;

        validate_identifier(
            self.source.as_str(),
            "compilation graph edge source",
        )?;

        validate_identifier(
            self.target.as_str(),
            "compilation graph edge target",
        )?;

        validate_metadata(&self.metadata)?;

        Ok(())
    }

    /// Estimate UTF-8 bytes used by edge-owned strings.
    ///
    /// This is an accounting estimate, not an exact heap-size measurement.
    pub fn estimated_metadata_bytes(
        &self,
    ) -> GraphEdgesResult<u128> {
        let mut total = self.id.as_str().len() as u128;

        total = checked_add(
            total,
            self.kind.as_str().len() as u128,
        )?;

        total = checked_add(
            total,
            self.source.as_str().len() as u128,
        )?;

        total = checked_add(
            total,
            self.target.as_str().len() as u128,
        )?;

        for (key, value) in &self.metadata {
            total = checked_add(
                total,
                key.len() as u128,
            )?;

            total = checked_add(
                total,
                value.len() as u128,
            )?;
        }

        Ok(total)
    }
}

// ============================================================================
// EDGE SET
// ============================================================================

/// Immutable collection of directed edges associated with one graph.
///
/// The edge set stores:
///
/// - The associated graph identity.
/// - Edge declarations in insertion order.
/// - A deterministic edge-identity index.
/// - An outgoing adjacency index.
/// - An incoming adjacency index.
/// - Descriptive metadata.
///
/// The indexes are derived data.
///
/// They are rebuilt during construction and deserialization.
///
/// Edge declaration order is preserved.
///
/// Adjacency results preserve edge declaration order.
///
/// The graph model remains the authoritative owner of nodes.
///
/// This edge set does not copy graph nodes.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "CompilationGraphEdgeSetWire")]
pub struct CompilationGraphEdgeSet {
    graph_id: CompilationGraphId,

    schema_version: u16,

    edges: Vec<CompilationGraphEdge>,

    metadata: BTreeMap<String, String>,

    /// Derived identity-to-position index.
    #[serde(skip)]
    edge_index: BTreeMap<CompilationGraphEdgeId, usize>,

    /// Derived source-node-to-edge-position index.
    #[serde(skip)]
    outgoing_index: BTreeMap<CompilationGraphNodeId, Vec<usize>>,

    /// Derived target-node-to-edge-position index.
    #[serde(skip)]
    incoming_index: BTreeMap<CompilationGraphNodeId, Vec<usize>>,
}

/// Validated serialization representation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CompilationGraphEdgeSetWire {
    graph_id: CompilationGraphId,

    schema_version: u16,

    #[serde(default)]
    edges: Vec<CompilationGraphEdge>,

    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl TryFrom<CompilationGraphEdgeSetWire>
    for CompilationGraphEdgeSet
{
    type Error = GraphEdgesError;

    fn try_from(
        value: CompilationGraphEdgeSetWire,
    ) -> Result<Self, Self::Error> {
        Self::from_parts(
            value.graph_id,
            value.schema_version,
            value.edges,
            value.metadata,
        )
    }
}

impl CompilationGraphEdgeSet {
    /// Construct an edge set associated with a specific graph.
    ///
    /// Construction validates:
    ///
    /// - Graph identity association.
    /// - Edge-local invariants.
    /// - Unique edge identities.
    /// - Source-node existence.
    /// - Target-node existence.
    /// - Metadata validity.
    ///
    /// No cycle prohibition is imposed.
    pub fn new(
        graph: &CompilationGraph,
        edges: Vec<CompilationGraphEdge>,
        metadata: BTreeMap<String, String>,
    ) -> GraphEdgesResult<Self> {
        let edge_set = Self::from_parts(
            graph.id().clone(),
            COMPILATION_GRAPH_EDGES_SCHEMA_VERSION,
            edges,
            metadata,
        )?;

        edge_set.validate_against(graph)?;

        Ok(edge_set)
    }

    /// Construct an edge set from serialized or interchange components.
    ///
    /// This validates local structure and builds derived indexes.
    ///
    /// Endpoint existence requires `validate_against`.
    pub fn from_parts(
        graph_id: CompilationGraphId,
        schema_version: u16,
        edges: Vec<CompilationGraphEdge>,
        metadata: BTreeMap<String, String>,
    ) -> GraphEdgesResult<Self> {
        if schema_version == 0 {
            return Err(
                GraphEdgesError::InvalidSchemaVersion {
                    version: schema_version,
                },
            );
        }

        validate_metadata(&metadata)?;

        let mut edge_index = BTreeMap::new();
        let mut outgoing_index: BTreeMap<
            CompilationGraphNodeId,
            Vec<usize>,
        > = BTreeMap::new();

        let mut incoming_index: BTreeMap<
            CompilationGraphNodeId,
            Vec<usize>,
        > = BTreeMap::new();

        for (position, edge) in edges.iter().enumerate() {
            edge.validate()?;

            if edge_index
                .insert(edge.id.clone(), position)
                .is_some()
            {
                return Err(
                    GraphEdgesError::DuplicateEdge {
                        edge_id: edge.id.to_string(),
                    },
                );
            }

            outgoing_index
                .entry(edge.source.clone())
                .or_default()
                .push(position);

            incoming_index
                .entry(edge.target.clone())
                .or_default()
                .push(position);
        }

        let edge_set = Self {
            graph_id,
            schema_version,
            edges,
            metadata,
            edge_index,
            outgoing_index,
            incoming_index,
        };

        edge_set.validate_structure()?;

        Ok(edge_set)
    }

    // ------------------------------------------------------------------------
    // BASIC ACCESS
    // ------------------------------------------------------------------------

    /// Return the associated graph identity.
    #[must_use]
    pub fn graph_id(&self) -> &CompilationGraphId {
        &self.graph_id
    }

    /// Return the edge-set schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Return all edges in declaration order.
    #[must_use]
    pub fn edges(&self) -> &[CompilationGraphEdge] {
        &self.edges
    }

    /// Return deterministic edge-set metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Return the number of registered edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Return whether the edge set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Check whether an edge identity exists.
    #[must_use]
    pub fn contains_edge(
        &self,
        id: &CompilationGraphEdgeId,
    ) -> bool {
        self.edge_index.contains_key(id)
    }

    /// Retrieve an edge by its identity.
    #[must_use]
    pub fn edge(
        &self,
        id: &CompilationGraphEdgeId,
    ) -> Option<&CompilationGraphEdge> {
        let position = self.edge_index.get(id)?;

        self.edges.get(*position)
    }

    /// Return edge identities in canonical identifier order.
    pub fn edge_ids_canonical_order(
        &self,
    ) -> impl Iterator<Item = &CompilationGraphEdgeId> {
        self.edge_index.keys()
    }

    // ------------------------------------------------------------------------
    // ADJACENCY
    // ------------------------------------------------------------------------

    /// Return all outgoing edges from a node.
    ///
    /// Results preserve edge declaration order.
    ///
    /// An unknown node identity produces an empty iterator.
    pub fn outgoing_edges<'a>(
        &'a self,
        node: &CompilationGraphNodeId,
    ) -> impl Iterator<Item = &'a CompilationGraphEdge> + 'a {
        self.outgoing_index
            .get(node)
            .into_iter()
            .flatten()
            .filter_map(|position| self.edges.get(*position))
    }

    /// Return all incoming edges to a node.
    ///
    /// Results preserve edge declaration order.
    ///
    /// An unknown node identity produces an empty iterator.
    pub fn incoming_edges<'a>(
        &'a self,
        node: &CompilationGraphNodeId,
    ) -> impl Iterator<Item = &'a CompilationGraphEdge> + 'a {
        self.incoming_index
            .get(node)
            .into_iter()
            .flatten()
            .filter_map(|position| self.edges.get(*position))
    }

    /// Return the number of outgoing edges for a node.
    #[must_use]
    pub fn outgoing_count(
        &self,
        node: &CompilationGraphNodeId,
    ) -> usize {
        self.outgoing_index
            .get(node)
            .map_or(0, Vec::len)
    }

    /// Return the number of incoming edges for a node.
    #[must_use]
    pub fn incoming_count(
        &self,
        node: &CompilationGraphNodeId,
    ) -> usize {
        self.incoming_index
            .get(node)
            .map_or(0, Vec::len)
    }

    /// Return whether a node has outgoing edges.
    #[must_use]
    pub fn has_outgoing_edges(
        &self,
        node: &CompilationGraphNodeId,
    ) -> bool {
        self.outgoing_index
            .get(node)
            .is_some_and(|edges| !edges.is_empty())
    }

    /// Return whether a node has incoming edges.
    #[must_use]
    pub fn has_incoming_edges(
        &self,
        node: &CompilationGraphNodeId,
    ) -> bool {
        self.incoming_index
            .get(node)
            .is_some_and(|edges| !edges.is_empty())
    }

    // ------------------------------------------------------------------------
    // VALIDATION
    // ------------------------------------------------------------------------

    /// Validate local edge-set invariants.
    ///
    /// This checks identities, edge-local data and derived indexes.
    ///
    /// It does not establish endpoint existence.
    pub fn validate_structure(&self) -> GraphEdgesResult<()> {
        validate_identifier(
            self.graph_id.as_str(),
            "compilation graph identity",
        )?;

        if self.schema_version == 0 {
            return Err(
                GraphEdgesError::InvalidSchemaVersion {
                    version: self.schema_version,
                },
            );
        }

        validate_metadata(&self.metadata)?;

        if self.edge_index.len() != self.edges.len() {
            return Err(GraphEdgesError::InvalidEdgeIndex);
        }

        let mut expected_outgoing: BTreeMap<
            CompilationGraphNodeId,
            Vec<usize>,
        > = BTreeMap::new();

        let mut expected_incoming: BTreeMap<
            CompilationGraphNodeId,
            Vec<usize>,
        > = BTreeMap::new();

        for (position, edge) in self.edges.iter().enumerate() {
            edge.validate()?;

            match self.edge_index.get(&edge.id) {
                Some(index) if *index == position => {}
                _ => {
                    return Err(GraphEdgesError::InvalidEdgeIndex);
                }
            }

            expected_outgoing
                .entry(edge.source.clone())
                .or_default()
                .push(position);

            expected_incoming
                .entry(edge.target.clone())
                .or_default()
                .push(position);
        }

        if expected_outgoing != self.outgoing_index {
            return Err(GraphEdgesError::InvalidOutgoingIndex);
        }

        if expected_incoming != self.incoming_index {
            return Err(GraphEdgesError::InvalidIncomingIndex);
        }

        Ok(())
    }

    /// Validate edge endpoints against the authoritative graph model.
    ///
    /// This checks:
    ///
    /// - Graph identity.
    /// - Graph structural validity.
    /// - Edge-set structural validity.
    /// - Source-node existence.
    /// - Target-node existence.
    ///
    /// It does not check semantic relationship legality.
    pub fn validate_against(
        &self,
        graph: &CompilationGraph,
    ) -> GraphEdgesResult<()> {
        graph.validate()?;
        self.validate_structure()?;

        if self.graph_id != *graph.id() {
            return Err(
                GraphEdgesError::GraphIdentityMismatch {
                    expected: graph.id().to_string(),
                    actual: self.graph_id.to_string(),
                },
            );
        }

        for edge in &self.edges {
            if !graph.contains_node(&edge.source) {
                return Err(
                    GraphEdgesError::MissingSourceNode {
                        edge_id: edge.id.to_string(),
                        node_id: edge.source.to_string(),
                    },
                );
            }

            if !graph.contains_node(&edge.target) {
                return Err(
                    GraphEdgesError::MissingTargetNode {
                        edge_id: edge.id.to_string(),
                        node_id: edge.target.to_string(),
                    },
                );
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // ACCOUNTING
    // ------------------------------------------------------------------------

    /// Estimate UTF-8 bytes held by edge identities, relationships
    /// and descriptive metadata.
    ///
    /// This excludes allocator overhead, collection capacities,
    /// pointers and index structures.
    ///
    /// It is not an exact heap-size measurement.
    pub fn estimated_metadata_bytes(
        &self,
    ) -> GraphEdgesResult<u128> {
        let mut total = self.graph_id.as_str().len() as u128;

        for edge in &self.edges {
            total = checked_add(
                total,
                edge.estimated_metadata_bytes()?,
            )?;
        }

        for (key, value) in &self.metadata {
            total = checked_add(
                total,
                key.len() as u128,
            )?;

            total = checked_add(
                total,
                value.len() as u128,
            )?;
        }

        Ok(total)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use crate::compiler::graph::model::{
        CanonicalEntityRef,
        CompilationGraphNode,
        CompilationGraphNodeKind,
    };

    fn graph() -> CompilationGraph {
        let nodes = vec![
            node("a"),
            node("b"),
            node("c"),
        ];

        CompilationGraph::new(
            CompilationGraphId::new("test.graph")
                .expect("valid graph identity"),
            nodes,
            BTreeMap::new(),
        )
        .expect("valid graph")
    }

    fn node(id: &str) -> CompilationGraphNode {
        CompilationGraphNode::new(
            CompilationGraphNodeId::new(id)
                .expect("valid node identity"),
            CompilationGraphNodeKind::new(
                "compiler.test.node",
            )
            .expect("valid node kind"),
            CanonicalEntityRef::new(
                "compiler.test",
                "node",
                id,
            )
            .expect("valid entity reference"),
            None,
            BTreeMap::new(),
        )
        .expect("valid node")
    }

    fn edge(
        id: &str,
        kind: &str,
        source: &str,
        target: &str,
    ) -> CompilationGraphEdge {
        CompilationGraphEdge::new(
            CompilationGraphEdgeId::new(id)
                .expect("valid edge identity"),
            CompilationGraphEdgeKind::new(kind)
                .expect("valid edge kind"),
            CompilationGraphNodeId::new(source)
                .expect("valid source"),
            CompilationGraphNodeId::new(target)
                .expect("valid target"),
            BTreeMap::new(),
        )
        .expect("valid edge")
    }

    fn node_id(value: &str) -> CompilationGraphNodeId {
        CompilationGraphNodeId::new(value)
            .expect("valid node identity")
    }

    #[test]
    fn empty_edge_set_is_valid() {
        let graph = graph();

        let edges = CompilationGraphEdgeSet::new(
            &graph,
            Vec::new(),
            BTreeMap::new(),
        )
        .expect("empty edge set is valid");

        assert!(edges.is_empty());
        assert_eq!(edges.edge_count(), 0);
        assert!(edges.validate_against(&graph).is_ok());
    }

    #[test]
    fn directed_relationship_is_preserved() {
        let graph = graph();

        let edges = CompilationGraphEdgeSet::new(
            &graph,
            vec![edge(
                "edge.a.b",
                "dependency.data",
                "a",
                "b",
            )],
            BTreeMap::new(),
        )
        .expect("valid edge set");

        let outgoing: Vec<_> =
            edges.outgoing_edges(&node_id("a")).collect();

        let incoming: Vec<_> =
            edges.incoming_edges(&node_id("b")).collect();

        assert_eq!(outgoing.len(), 1);
        assert_eq!(incoming.len(), 1);

        assert_eq!(outgoing[0].source().as_str(), "a");
        assert_eq!(outgoing[0].target().as_str(), "b");
    }

    #[test]
    fn parallel_edges_are_supported() {
        let graph = graph();

        let edges = CompilationGraphEdgeSet::new(
            &graph,
            vec![
                edge("edge.1", "dependency.data", "a", "b"),
                edge("edge.2", "dependency.control", "a", "b"),
            ],
            BTreeMap::new(),
        )
        .expect("parallel relationships are valid");

        assert_eq!(edges.edge_count(), 2);
        assert_eq!(edges.outgoing_count(&node_id("a")), 2);
        assert_eq!(edges.incoming_count(&node_id("b")), 2);
    }

    #[test]
    fn cycles_are_supported() {
        let graph = graph();

        let edges = CompilationGraphEdgeSet::new(
            &graph,
            vec![
                edge("edge.ab", "dependency.feedback", "a", "b"),
                edge("edge.ba", "dependency.feedback", "b", "a"),
            ],
            BTreeMap::new(),
        )
        .expect("cycles are structurally valid");

        assert_eq!(edges.edge_count(), 2);
    }

    #[test]
    fn self_referential_edges_are_structurally_supported() {
        let graph = graph();

        let edges = CompilationGraphEdgeSet::new(
            &graph,
            vec![edge(
                "edge.self",
                "dependency.feedback",
                "a",
                "a",
            )],
            BTreeMap::new(),
        )
        .expect("self-reference is structurally valid");

        assert_eq!(edges.edge_count(), 1);
        assert_eq!(edges.outgoing_count(&node_id("a")), 1);
        assert_eq!(edges.incoming_count(&node_id("a")), 1);
    }

    #[test]
    fn duplicate_edge_identity_is_rejected() {
        let graph = graph();

        let result = CompilationGraphEdgeSet::new(
            &graph,
            vec![
                edge("duplicate", "dependency.data", "a", "b"),
                edge("duplicate", "dependency.control", "b", "c"),
            ],
            BTreeMap::new(),
        );

        assert!(matches!(
            result,
            Err(GraphEdgesError::DuplicateEdge { .. })
        ));
    }

    #[test]
    fn missing_source_node_is_rejected() {
        let graph = graph();

        let result = CompilationGraphEdgeSet::new(
            &graph,
            vec![edge(
                "edge.invalid",
                "dependency.data",
                "missing",
                "b",
            )],
            BTreeMap::new(),
        );

        assert!(matches!(
            result,
            Err(GraphEdgesError::MissingSourceNode { .. })
        ));
    }

    #[test]
    fn missing_target_node_is_rejected() {
        let graph = graph();

        let result = CompilationGraphEdgeSet::new(
            &graph,
            vec![edge(
                "edge.invalid",
                "dependency.data",
                "a",
                "missing",
            )],
            BTreeMap::new(),
        );

        assert!(matches!(
            result,
            Err(GraphEdgesError::MissingTargetNode { .. })
        ));
    }

    #[test]
    fn different_graph_identity_is_rejected() {
        let graph = graph();

        let other_graph = CompilationGraph::new(
            CompilationGraphId::new("another.graph")
                .expect("valid identity"),
            Vec::new(),
            BTreeMap::new(),
        )
        .expect("valid graph");

        let edge_set = CompilationGraphEdgeSet::new(
            &graph,
            vec![edge(
                "edge.a.b",
                "dependency.data",
                "a",
                "b",
            )],
            BTreeMap::new(),
        )
        .expect("valid edge set");

        assert!(matches!(
            edge_set.validate_against(&other_graph),
            Err(GraphEdgesError::GraphIdentityMismatch { .. })
        ));
    }

    #[test]
    fn adjacency_preserves_declaration_order() {
        let graph = graph();

        let edge_set = CompilationGraphEdgeSet::new(
            &graph,
            vec![
                edge("first", "dependency.data", "a", "b"),
                edge("second", "dependency.data", "a", "c"),
                edge("third", "dependency.data", "a", "b"),
            ],
            BTreeMap::new(),
        )
        .expect("valid edge set");

        let ids: Vec<_> = edge_set
            .outgoing_edges(&node_id("a"))
            .map(|edge| edge.id().as_str())
            .collect();

        assert_eq!(
            ids,
            vec!["first", "second", "third"]
        );
    }

    #[test]
    fn unknown_node_has_no_adjacency() {
        let graph = graph();

        let edge_set = CompilationGraphEdgeSet::new(
            &graph,
            Vec::new(),
            BTreeMap::new(),
        )
        .expect("valid edge set");

        assert_eq!(
            edge_set.outgoing_count(&node_id("unknown")),
            0
        );

        assert_eq!(
            edge_set.incoming_count(&node_id("unknown")),
            0
        );
    }

    #[test]
    fn edge_kind_is_open_world() {
        let kind = CompilationGraphEdgeKind::new(
            "third_party.custom.relationship",
        )
        .expect("custom relationship must be accepted");

        assert_eq!(
            kind.as_str(),
            "third_party.custom.relationship"
        );
    }

    #[test]
    fn invalid_edge_identity_is_rejected() {
        let result = CompilationGraphEdgeId::new("   ");

        assert!(matches!(
            result,
            Err(GraphEdgesError::EmptyIdentifier { .. })
        ));
    }

    #[test]
    fn metadata_control_characters_are_rejected() {
        let mut metadata = BTreeMap::new();
        metadata.insert(
            "description".to_string(),
            "invalid\nvalue".to_string(),
        );

        let result = CompilationGraphEdge::new(
            CompilationGraphEdgeId::new("edge")
                .expect("valid identity"),
            CompilationGraphEdgeKind::new("dependency.data")
                .expect("valid kind"),
            node_id("a"),
            node_id("b"),
            metadata,
        );

        assert!(matches!(
            result,
            Err(GraphEdgesError::InvalidMetadataValue)
        ));
    }

    #[test]
    fn edge_set_serialization_round_trip() {
        let graph = graph();

        let original = CompilationGraphEdgeSet::new(
            &graph,
            vec![edge(
                "edge.a.b",
                "dependency.data",
                "a",
                "b",
            )],
            BTreeMap::new(),
        )
        .expect("valid edge set");

        let encoded = serde_json::to_string(&original)
            .expect("serialization succeeds");

        let decoded: CompilationGraphEdgeSet =
            serde_json::from_str(&encoded)
                .expect("validated deserialization succeeds");

        assert_eq!(decoded.edges(), original.edges());

        assert!(decoded.validate_against(&graph).is_ok());

        assert_eq!(
            decoded.outgoing_count(&node_id("a")),
            1
        );
    }

    #[test]
    fn edge_set_accounting_is_available() {
        let graph = graph();

        let edge_set = CompilationGraphEdgeSet::new(
            &graph,
            vec![edge(
                "edge.a.b",
                "dependency.data",
                "a",
                "b",
            )],
            BTreeMap::new(),
        )
        .expect("valid edge set");

        assert!(
            edge_set.estimated_metadata_bytes().unwrap() > 0
        );
    }
}
