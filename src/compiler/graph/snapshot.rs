
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Graph Snapshots.
//!
//! Path: src/compiler/graph/snapshot.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provides an immutable, validated point-in-time representation of a
//! compiler graph and its separately owned edge set.
//!
//! A snapshot groups the canonical graph model and its corresponding
//! canonical edge set without creating another graph representation.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - CompilationGraphSnapshot.
//! - Snapshot schema versioning.
//! - Snapshot-local descriptive metadata.
//! - Snapshot construction and validation.
//! - Snapshot serialization/deserialization validation.
//! - Snapshot-level structural accounting.
//!
//! # Canonical authorities
//!
//! `super::model` owns:
//! - CompilationGraph.
//! - CompilationGraphId.
//! - CompilationGraphNode.
//! - Graph-model validation.
//!
//! `super::edges` owns:
//! - CompilationGraphEdgeSet.
//! - CompilationGraphEdge.
//! - Edge identities and relationships.
//! - Adjacency indexes.
//! - Edge-set validation.
//!
//! `crate::compiler::incremental::fingerprints` owns compiler-level
//! fingerprint integration.
//!
//! # Non-responsibilities
//!
//! This module does not own:
//!
//! - Source grammar, parsing, or AST construction.
//! - Source-level semantic analysis.
//! - Canonical entity resolution.
//! - ZUIR or domain-specific IR.
//! - Dependency discovery.
//! - Semantic edge classification.
//! - Cycle legality.
//! - Pipeline ordering.
//! - Physical resource allocation.
//! - Target selection.
//! - Scheduling or routing.
//! - Backend implementation.
//! - Runtime execution.
//! - Content fingerprint algorithms.
//! - Cache storage or invalidation.
//!
//! # Immutability
//!
//! A snapshot cannot be modified after construction.
//!
//! To represent a changed graph, construct a new graph and a new snapshot.
//!
//! The snapshot owns its graph and edge-set values. Cloning a snapshot
//! creates an independent owned copy of those values.
//!
//! # Validation
//!
//! Successful construction establishes:
//!
//! - A nonzero snapshot schema version.
//! - A structurally valid graph.
//! - A structurally valid edge set.
//! - Matching graph identities.
//! - Existing source and target nodes for every edge.
//! - Valid snapshot metadata.
//!
//! It does not establish semantic validity, target feasibility,
//! backend compatibility, or execution availability.
//!
//! # Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Graph nodes.
//! - Graph edges.
//! - Metadata entries.
//! - Metadata string lengths.
//! - Graph identity lengths.
//! - Snapshot size.
//!
//! Actual operation depends on available memory, address space,
//! allocation behavior, execution time, and operational budgets.
//!
//! Literal infinite memory or computation is not promised.
//!
//! # Determinism
//!
//! The snapshot preserves the declaration order of graph nodes and edges.
//!
//! Snapshot metadata uses BTreeMap for deterministic key ordering.
//!
//! This module does not create timestamps, random identifiers,
//! pointer-based identities, or content fingerprints.
//!
//! # Serialization
//!
//! Deserialization validates the complete local snapshot structure.
//!
//! It rebuilds graph and edge-set indexes through their canonical
//! constructors and validates edge endpoints against the graph.
//!
//! A successfully deserialized snapshot is structurally valid.
//! It is not automatically semantically verified or trusted.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use super::edges::{
    CompilationGraphEdgeSet,
    GraphEdgesError,
};

use super::model::{
    CompilationGraph,
    GraphModelError,
};

// ============================================================================
// API AND SCHEMA VERSION
// ============================================================================

/// Public API version for compiler graph snapshots.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Semantic-model version.
/// - Compiler composition API version.
/// - Graph-model API version.
/// - Graph-edge API version.
/// - ZUIR schema version.
/// - Domain IR versions.
/// - Backend ABI versions.
/// - Runtime versions.
pub const COMPILATION_GRAPH_SNAPSHOT_API_VERSION: u16 = 1;

/// Current serialized snapshot schema version.
///
/// This is a representation version, not a graph-size limit.
pub const COMPILATION_GRAPH_SNAPSHOT_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for compiler graph snapshot operations.
pub type GraphSnapshotResult<T> =
    Result<T, GraphSnapshotError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compiler graph snapshot operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GraphSnapshotError {
    /// The snapshot schema version is zero.
    InvalidSchemaVersion {
        version: u16,
    },

    /// Snapshot metadata contains an invalid key.
    InvalidMetadataKey,

    /// Snapshot metadata contains prohibited control characters.
    InvalidMetadataValue,

    /// Snapshot accounting exceeded its representation.
    SizeOverflow,

    /// The graph model is structurally invalid.
    GraphModel {
        source: GraphModelError,
    },

    /// The graph edge set is structurally invalid or incompatible.
    GraphEdges {
        source: GraphEdgesError,
    },
}

impl fmt::Display for GraphSnapshotError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidSchemaVersion { version } => {
                write!(
                    formatter,
                    "invalid compilation graph snapshot schema version: {version}"
                )
            }

            Self::InvalidMetadataKey => {
                formatter.write_str(
                    "compilation graph snapshot contains an invalid metadata key",
                )
            }

            Self::InvalidMetadataValue => {
                formatter.write_str(
                    "compilation graph snapshot metadata contains prohibited control characters",
                )
            }

            Self::SizeOverflow => {
                formatter.write_str(
                    "compilation graph snapshot accounting overflow",
                )
            }

            Self::GraphModel { source } => {
                write!(
                    formatter,
                    "compilation graph snapshot model error: {source}",
                )
            }

            Self::GraphEdges { source } => {
                write!(
                    formatter,
                    "compilation graph snapshot edge error: {source}",
                )
            }
        }
    }
}

impl Error for GraphSnapshotError {
    fn source(
        &self,
    ) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::GraphModel { source } => Some(source),
            Self::GraphEdges { source } => Some(source),
            _ => None,
        }
    }
}

impl From<GraphModelError> for GraphSnapshotError {
    fn from(
        source: GraphModelError,
    ) -> Self {
        Self::GraphModel {
            source,
        }
    }
}

impl From<GraphEdgesError> for GraphSnapshotError {
    fn from(
        source: GraphEdgesError,
    ) -> Self {
        Self::GraphEdges {
            source,
        }
    }
}

// ============================================================================
// METADATA VALIDATION
// ============================================================================

/// Validate snapshot-local descriptive metadata.
///
/// Empty metadata values are permitted.
///
/// Empty or whitespace-only keys are prohibited.
///
/// Control characters are prohibited in keys and values.
///
/// Metadata is descriptive only. It cannot override canonical graph
/// or edge semantics.
fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> GraphSnapshotResult<()> {
    for (key, value) in metadata {
        if key.trim().is_empty()
            || key.chars().any(char::is_control)
        {
            return Err(
                GraphSnapshotError::InvalidMetadataKey,
            );
        }

        if value.chars().any(char::is_control) {
            return Err(
                GraphSnapshotError::InvalidMetadataValue,
            );
        }
    }

    Ok(())
}

/// Add two accounting quantities without wrapping.
fn checked_add(
    left: u128,
    right: u128,
) -> GraphSnapshotResult<u128> {
    left.checked_add(right)
        .ok_or(GraphSnapshotError::SizeOverflow)
}

// ============================================================================
// SNAPSHOT
// ============================================================================

/// Immutable, validated point-in-time compiler graph snapshot.
///
/// A snapshot groups:
///
/// - One canonical CompilationGraph.
/// - One canonical CompilationGraphEdgeSet.
/// - Snapshot-local descriptive metadata.
/// - A snapshot serialization schema version.
///
/// The graph remains the authoritative owner of nodes.
///
/// The edge set remains the authoritative owner of relationships.
///
/// This type does not copy either representation into a competing
/// snapshot-specific graph model.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "CompilationGraphSnapshotWire")]
pub struct CompilationGraphSnapshot {
    schema_version: u16,

    graph: CompilationGraph,

    edges: CompilationGraphEdgeSet,

    metadata: BTreeMap<String, String>,
}

/// Wire representation used for validated deserialization.
///
/// This type is deliberately private. External callers must construct
/// snapshots through the validated public API.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Deserialize,
)]
struct CompilationGraphSnapshotWire {
    schema_version: u16,

    graph: CompilationGraph,

    edges: CompilationGraphEdgeSet,

    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl TryFrom<CompilationGraphSnapshotWire>
    for CompilationGraphSnapshot
{
    type Error = GraphSnapshotError;

    fn try_from(
        value: CompilationGraphSnapshotWire,
    ) -> Result<Self, Self::Error> {
        Self::new_with_schema_version(
            value.schema_version,
            value.graph,
            value.edges,
            value.metadata,
        )
    }
}

impl CompilationGraphSnapshot {
    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    /// Construct a snapshot using the current snapshot schema version.
    ///
    /// The supplied graph and edge set are validated together.
    ///
    /// The edge set must belong to the supplied graph, and every edge
    /// endpoint must identify a node in that graph.
    ///
    /// The values are moved into the snapshot. No unnecessary clone
    /// is performed.
    pub fn new(
        graph: CompilationGraph,
        edges: CompilationGraphEdgeSet,
    ) -> GraphSnapshotResult<Self> {
        Self::new_with_metadata(
            graph,
            edges,
            BTreeMap::new(),
        )
    }

    /// Construct a snapshot using the current schema version and
    /// caller-supplied descriptive metadata.
    ///
    /// Metadata is not semantic authority.
    pub fn new_with_metadata(
        graph: CompilationGraph,
        edges: CompilationGraphEdgeSet,
        metadata: BTreeMap<String, String>,
    ) -> GraphSnapshotResult<Self> {
        Self::new_with_schema_version(
            COMPILATION_GRAPH_SNAPSHOT_SCHEMA_VERSION,
            graph,
            edges,
            metadata,
        )
    }

    /// Construct a snapshot with an explicit schema version.
    ///
    /// This is intended for controlled serialization, interchange,
    /// and migration.
    ///
    /// A nonzero schema version is structurally accepted. Compatibility
    /// with a particular consumer remains the responsibility of the
    /// appropriate compatibility authority.
    pub fn new_with_schema_version(
        schema_version: u16,
        graph: CompilationGraph,
        edges: CompilationGraphEdgeSet,
        metadata: BTreeMap<String, String>,
    ) -> GraphSnapshotResult<Self> {
        if schema_version == 0 {
            return Err(
                GraphSnapshotError::InvalidSchemaVersion {
                    version: schema_version,
                },
            );
        }

        validate_metadata(&metadata)?;

        let snapshot = Self {
            schema_version,
            graph,
            edges,
            metadata,
        };

        snapshot.validate()?;

        Ok(snapshot)
    }

    /// Create a new snapshot from an existing immutable graph and edge set.
    ///
    /// This operation clones the supplied values.
    ///
    /// Use `new` when ownership can be transferred to avoid cloning.
    pub fn capture(
        graph: &CompilationGraph,
        edges: &CompilationGraphEdgeSet,
    ) -> GraphSnapshotResult<Self> {
        Self::new(
            graph.clone(),
            edges.clone(),
        )
    }

    // ------------------------------------------------------------------------
    // ACCESSORS
    // ------------------------------------------------------------------------

    /// Return the snapshot schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Return whether this snapshot uses the current schema version.
    ///
    /// This does not establish compatibility with every consumer.
    #[must_use]
    pub const fn uses_current_schema(&self) -> bool {
        self.schema_version
            == COMPILATION_GRAPH_SNAPSHOT_SCHEMA_VERSION
    }

    /// Return the canonical graph.
    #[must_use]
    pub const fn graph(&self) -> &CompilationGraph {
        &self.graph
    }

    /// Return the canonical edge set associated with the graph.
    #[must_use]
    pub const fn edges(&self) -> &CompilationGraphEdgeSet {
        &self.edges
    }

    /// Return snapshot-local descriptive metadata.
    #[must_use]
    pub const fn metadata(
        &self,
    ) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Return the graph identity.
    ///
    /// The graph model remains the identity authority.
    #[must_use]
    pub fn graph_id(
        &self,
    ) -> &super::model::CompilationGraphId {
        self.graph.id()
    }

    /// Return the number of graph nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Return the number of graph edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.edge_count()
    }

    /// Return whether both the graph and edge set are empty.
    ///
    /// A graph with nodes but no edges is not empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
            && self.edges.is_empty()
    }

    // ------------------------------------------------------------------------
    // VALIDATION
    // ------------------------------------------------------------------------

    /// Validate the complete snapshot structure.
    ///
    /// This verifies:
    ///
    /// - Snapshot schema version.
    /// - Snapshot metadata.
    /// - Graph-model structure.
    /// - Edge-set structure.
    /// - Graph identity agreement.
    /// - Source-node existence for every edge.
    /// - Target-node existence for every edge.
    ///
    /// It does not perform semantic verification.
    pub fn validate(
        &self,
    ) -> GraphSnapshotResult<()> {
        if self.schema_version == 0 {
            return Err(
                GraphSnapshotError::InvalidSchemaVersion {
                    version: self.schema_version,
                },
            );
        }

        validate_metadata(&self.metadata)?;

        self.graph.validate()?;

        self.edges.validate_against(
            &self.graph,
        )?;

        Ok(())
    }

    /// Return an estimate of UTF-8 bytes represented by this snapshot.
    ///
    /// This includes:
    ///
    /// - Graph model metadata and descriptive strings.
    /// - Edge-set metadata and descriptive strings.
    /// - Snapshot-local metadata.
    ///
    /// It excludes:
    ///
    /// - Allocator overhead.
    /// - Collection capacities.
    /// - Pointer sizes.
    /// - Internal BTreeMap and Vec structures.
    /// - Derived graph and adjacency indexes.
    /// - Serialization framing.
    ///
    /// It is an accounting estimate, not an exact heap measurement.
    pub fn estimated_metadata_bytes(
        &self,
    ) -> GraphSnapshotResult<u128> {
        let mut total =
            self.graph.estimated_metadata_bytes()?;

        total = checked_add(
            total,
            self.edges.estimated_metadata_bytes()?,
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

    // ------------------------------------------------------------------------
    // OWNERSHIP TRANSFER
    // ------------------------------------------------------------------------

    /// Consume the snapshot and return its canonical graph and edge set.
    ///
    /// Snapshot-local metadata and snapshot schema information are discarded.
    ///
    /// No graph or edge representation is reconstructed.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        CompilationGraph,
        CompilationGraphEdgeSet,
    ) {
        (
            self.graph,
            self.edges,
        )
    }

    /// Consume the snapshot and return all owned components.
    #[must_use]
    pub fn into_parts_with_metadata(
        self,
    ) -> (
        u16,
        CompilationGraph,
        CompilationGraphEdgeSet,
        BTreeMap<String, String>,
    ) {
        (
            self.schema_version,
            self.graph,
            self.edges,
            self.metadata,
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::edges::{
        CompilationGraphEdge,
        CompilationGraphEdgeId,
        CompilationGraphEdgeKind,
    };

    use super::super::model::{
        CanonicalEntityRef,
        CompilationGraphId,
        CompilationGraphNode,
        CompilationGraphNodeId,
        CompilationGraphNodeKind,
    };

    fn graph_id() -> CompilationGraphId {
        CompilationGraphId::new(
            "test.compiler.graph",
        )
        .expect("valid graph identity")
    }

    fn node_id(
        value: &str,
    ) -> CompilationGraphNodeId {
        CompilationGraphNodeId::new(value)
            .expect("valid node identity")
    }

    fn node(
        id: &str,
    ) -> CompilationGraphNode {
        CompilationGraphNode::new(
            node_id(id),
            CompilationGraphNodeKind::new(
                "compiler.test.node",
            )
            .expect("valid node kind"),
            CanonicalEntityRef::new(
                "crate::compiler::testing",
                "test.entity",
                id,
            )
            .expect("valid canonical entity reference"),
            None,
            BTreeMap::new(),
        )
        .expect("valid graph node")
    }

    fn graph() -> CompilationGraph {
        CompilationGraph::new(
            graph_id(),
            vec![
                node("node.a"),
                node("node.b"),
            ],
            BTreeMap::new(),
        )
        .expect("valid graph")
    }

    fn edge(
        id: &str,
        source: &str,
        target: &str,
    ) -> CompilationGraphEdge {
        CompilationGraphEdge::new(
            CompilationGraphEdgeId::new(id)
                .expect("valid edge identity"),
            CompilationGraphEdgeKind::new(
                "compiler.test.dependency",
            )
            .expect("valid edge kind"),
            node_id(source),
            node_id(target),
            BTreeMap::new(),
        )
        .expect("valid edge")
    }

    fn edge_set(
        graph: &CompilationGraph,
        edges: Vec<CompilationGraphEdge>,
    ) -> CompilationGraphEdgeSet {
        CompilationGraphEdgeSet::new(
            graph,
            edges,
            BTreeMap::new(),
        )
        .expect("valid edge set")
    }

    #[test]
    fn constructs_valid_snapshot() {
        let graph = graph();

        let edges = edge_set(
            &graph,
            vec![
                edge(
                    "edge.a",
                    "node.a",
                    "node.b",
                ),
            ],
        );

        let snapshot = CompilationGraphSnapshot::new(
            graph,
            edges,
        )
        .expect("valid snapshot");

        assert_eq!(
            snapshot.node_count(),
            2,
        );

        assert_eq!(
            snapshot.edge_count(),
            1,
        );

        assert!(!snapshot.is_empty());

        assert!(snapshot.validate().is_ok());
    }

    #[test]
    fn rejects_zero_snapshot_schema_version() {
        let graph = graph();

        let edges = edge_set(
            &graph,
            Vec::new(),
        );

        let result =
            CompilationGraphSnapshot::new_with_schema_version(
                0,
                graph,
                edges,
                BTreeMap::new(),
            );

        assert!(matches!(
            result,
            Err(
                GraphSnapshotError::InvalidSchemaVersion {
                    version: 0,
                }
            )
        ));
    }

    #[test]
    fn rejects_edge_set_from_another_graph() {
        let first_graph = graph();

        let second_graph = CompilationGraph::new(
            CompilationGraphId::new(
                "another.compiler.graph",
            )
            .expect("valid graph identity"),
            vec![
                node("node.a"),
                node("node.b"),
            ],
            BTreeMap::new(),
        )
        .expect("valid graph");

        let foreign_edges = edge_set(
            &second_graph,
            Vec::new(),
        );

        let result = CompilationGraphSnapshot::new(
            first_graph,
            foreign_edges,
        );

        assert!(matches!(
            result,
            Err(
                GraphSnapshotError::GraphEdges {
                    source: GraphEdgesError::GraphIdentityMismatch {
                        ..
                    }
                }
            )
        ));
    }

    #[test]
    fn preserves_valid_feedback_cycles() {
        let graph = graph();

        let edges = edge_set(
            &graph,
            vec![
                edge(
                    "edge.forward",
                    "node.a",
                    "node.b",
                ),
                edge(
                    "edge.feedback",
                    "node.b",
                    "node.a",
                ),
            ],
        );

        let snapshot = CompilationGraphSnapshot::new(
            graph,
            edges,
        )
        .expect("structurally valid cyclic graph");

        assert_eq!(
            snapshot.edge_count(),
            2,
        );

        assert!(snapshot.validate().is_ok());
    }

    #[test]
    fn snapshot_metadata_is_deterministic() {
        let graph = graph();

        let edges = edge_set(
            &graph,
            Vec::new(),
        );

        let mut metadata = BTreeMap::new();

        metadata.insert(
            "zeta".to_owned(),
            "last".to_owned(),
        );

        metadata.insert(
            "alpha".to_owned(),
            "first".to_owned(),
        );

        let snapshot =
            CompilationGraphSnapshot::new_with_metadata(
                graph,
                edges,
                metadata,
            )
            .expect("valid snapshot");

        let keys: Vec<&String> =
            snapshot.metadata().keys().collect();

        assert_eq!(
            keys,
            vec![
                &"alpha".to_owned(),
                &"zeta".to_owned(),
            ],
        );
    }

    #[test]
    fn rejects_invalid_snapshot_metadata_key() {
        let graph = graph();

        let edges = edge_set(
            &graph,
            Vec::new(),
        );

        let mut metadata = BTreeMap::new();

        metadata.insert(
            "   ".to_owned(),
            "value".to_owned(),
        );

        let result =
            CompilationGraphSnapshot::new_with_metadata(
                graph,
                edges,
                metadata,
            );

        assert_eq!(
            result,
            Err(
                GraphSnapshotError::InvalidMetadataKey,
            ),
        );
    }

    #[test]
    fn rejects_control_characters_in_snapshot_metadata() {
        let graph = graph();

        let edges = edge_set(
            &graph,
            Vec::new(),
        );

        let mut metadata = BTreeMap::new();

        metadata.insert(
            "valid.key".to_owned(),
            "invalid\nvalue".to_owned(),
        );

        let result =
            CompilationGraphSnapshot::new_with_metadata(
                graph,
                edges,
                metadata,
            );

        assert_eq!(
            result,
            Err(
                GraphSnapshotError::InvalidMetadataValue,
            ),
        );
    }

    #[test]
    fn snapshot_capture_preserves_original_values() {
        let graph = graph();

        let edges = edge_set(
            &graph,
            vec![
                edge(
                    "edge.a",
                    "node.a",
                    "node.b",
                ),
            ],
        );

        let snapshot =
            CompilationGraphSnapshot::capture(
                &graph,
                &edges,
            )
            .expect("valid snapshot");

        assert_eq!(
            snapshot.graph(),
            &graph,
        );

        assert_eq!(
            snapshot.edges(),
            &edges,
        );
    }

    #[test]
    fn snapshot_clone_is_equal_and_independent() {
        let graph = graph();

        let edges = edge_set(
            &graph,
            Vec::new(),
        );

        let snapshot = CompilationGraphSnapshot::new(
            graph,
            edges,
        )
        .expect("valid snapshot");

        let cloned = snapshot.clone();

        assert_eq!(
            snapshot,
            cloned,
        );

        assert!(cloned.validate().is_ok());
    }

    #[test]
    fn snapshot_accounting_is_available() {
        let graph = graph();

        let edges = edge_set(
            &graph,
            vec![
                edge(
                    "edge.a",
                    "node.a",
                    "node.b",
                ),
            ],
        );

        let snapshot = CompilationGraphSnapshot::new(
            graph,
            edges,
        )
        .expect("valid snapshot");

        assert!(
            snapshot
                .estimated_metadata_bytes()
                .expect("accounting succeeds")
                > 0
        );
    }

    #[test]
    fn empty_graph_and_edge_set_form_an_empty_snapshot() {
        let graph = CompilationGraph::new(
            graph_id(),
            Vec::new(),
            BTreeMap::new(),
        )
        .expect("valid empty graph");

        let edges = edge_set(
            &graph,
            Vec::new(),
        );

        let snapshot = CompilationGraphSnapshot::new(
            graph,
            edges,
        )
        .expect("valid empty snapshot");

        assert!(snapshot.is_empty());
        assert_eq!(snapshot.node_count(), 0);
        assert_eq!(snapshot.edge_count(), 0);
    }

    #[test]
    fn into_parts_preserves_canonical_values() {
        let graph = graph();

        let edges = edge_set(
            &graph,
            vec![
                edge(
                    "edge.a",
                    "node.a",
                    "node.b",
                ),
            ],
        );

        let snapshot = CompilationGraphSnapshot::new(
            graph.clone(),
            edges.clone(),
        )
        .expect("valid snapshot");

        let (returned_graph, returned_edges) =
            snapshot.into_parts();

        assert_eq!(
            returned_graph,
            graph,
        );

        assert_eq!(
            returned_edges,
            edges,
        );
    }
}
