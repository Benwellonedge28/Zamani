#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Compiler provenance and transformation lineage.
//!
//! This module records where compiler artifacts came from and which
//! transformations produced them. It is deliberately independent of
//! the frontend, semantic model, IR implementations, backends, and runtime.
//!
//! # Responsibilities
//!
//! - Identify source inputs, compiler artifacts, and transformation records.
//! - Preserve parent-to-child lineage.
//! - Record transformation identity, version, and configuration fingerprint.
//! - Record optional source locations and descriptive metadata.
//! - Validate graph structure and reject dangling edges and cycles.
//! - Provide deterministic iteration through ordered collections.
//!
//! # Non-responsibilities
//!
//! - Defining program semantics or IR.
//! - Computing cryptographic hashes.
//! - Reading files, environment variables, clocks, or Git metadata.
//! - Allocating hardware or scheduling runtime work.
//! - Claiming that provenance is a cryptographic attestation.
//!
//! Hashes and timestamps are supplied by the caller. This module records
//! those values but does not independently verify them.
//!
//! # Scalability
//!
//! No fixed number of artifacts, transformations, inputs, or metadata
//! entries is imposed. Storage is bounded by available memory and the
//! representational limits of the host platform. Operations that require
//! traversing the graph are proportional to the graph size.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;

/// Stable identifier for a provenance graph node.
///
/// Identifiers are caller-supplied strings. They should be globally unique
/// within the graph and should remain stable when the graph is serialized.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceNodeId(String);

impl ProvenanceNodeId {
    /// Creates an identifier.
    ///
    /// Empty and whitespace-only identifiers are rejected. No arbitrary
    /// maximum length is imposed.
    pub fn new(value: impl Into<String>) -> Result<Self, ProvenanceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ProvenanceError::EmptyIdentifier);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ProvenanceNodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A source location associated with a provenance node.
///
/// The location is descriptive metadata. It does not replace the canonical
/// source map or frontend span representation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProvenanceSourceLocation {
    /// Source identifier, such as a source-map key or source URI.
    pub source: String,

    /// Optional one-based starting line.
    pub line_start: Option<u64>,

    /// Optional one-based starting column.
    pub column_start: Option<u64>,

    /// Optional one-based ending line.
    pub line_end: Option<u64>,

    /// Optional one-based ending column.
    pub column_end: Option<u64>,
}

impl ProvenanceSourceLocation {
    /// Creates a source location without line or column information.
    pub fn new(source: impl Into<String>) -> Result<Self, ProvenanceError> {
        let source = source.into();

        if source.trim().is_empty() {
            return Err(ProvenanceError::EmptySourceLocation);
        }

        Ok(Self {
            source,
            line_start: None,
            column_start: None,
            line_end: None,
            column_end: None,
        })
    }

    /// Sets a source range.
    ///
    /// Coordinates are one-based. If both endpoints are present, the ending
    /// coordinate must not precede the starting coordinate.
    pub fn with_range(
        mut self,
        line_start: u64,
        column_start: u64,
        line_end: u64,
        column_end: u64,
    ) -> Result<Self, ProvenanceError> {
        if line_start == 0
            || column_start == 0
            || line_end == 0
            || column_end == 0
        {
            return Err(ProvenanceError::InvalidSourceRange);
        }

        if (line_end, column_end) < (line_start, column_start) {
            return Err(ProvenanceError::InvalidSourceRange);
        }

        self.line_start = Some(line_start);
        self.column_start = Some(column_start);
        self.line_end = Some(line_end);
        self.column_end = Some(column_end);

        Ok(self)
    }
}

/// The kind of entity represented by a provenance node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProvenanceNodeKind {
    /// Original source input.
    Source,

    /// Parsed or semantically analyzed program representation.
    Analysis,

    /// Canonical or domain-specific intermediate representation.
    IntermediateRepresentation,

    /// Output of a compiler transformation.
    TransformationOutput,

    /// Generated backend artifact.
    BackendArtifact,

    /// Linked or packaged artifact.
    Package,

    /// A caller-defined category for extensibility.
    Custom(String),
}

/// Information describing a compiler transformation.
///
/// A transformation record identifies the operation that produced an
/// artifact. It does not itself implement or execute that transformation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformationInfo {
    /// Stable transformation name.
    pub name: String,

    /// Transformation implementation or contract version.
    pub version: String,

    /// Optional fingerprint of the effective transformation configuration.
    ///
    /// The fingerprint is opaque. Its algorithm and correctness are the
    /// responsibility of the caller.
    pub configuration_fingerprint: Option<String>,
}

impl TransformationInfo {
    /// Creates transformation metadata.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();
        let version = version.into();

        if name.trim().is_empty() {
            return Err(ProvenanceError::EmptyTransformationName);
        }

        if version.trim().is_empty() {
            return Err(ProvenanceError::EmptyTransformationVersion);
        }

        Ok(Self {
            name,
            version,
            configuration_fingerprint: None,
        })
    }

    /// Sets the opaque configuration fingerprint.
    pub fn with_configuration_fingerprint(
        mut self,
        fingerprint: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let fingerprint = fingerprint.into();

        if fingerprint.trim().is_empty() {
            return Err(ProvenanceError::EmptyFingerprint);
        }

        self.configuration_fingerprint = Some(fingerprint);
        Ok(self)
    }
}

/// A node in the provenance graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceNode {
    /// Stable graph-local identifier.
    pub id: ProvenanceNodeId,

    /// Category of the represented entity.
    pub kind: ProvenanceNodeKind,

    /// Optional source location.
    pub source_location: Option<ProvenanceSourceLocation>,

    /// Optional transformation information.
    ///
    /// This is expected for transformation-produced nodes, but it remains
    /// optional to support source and externally supplied artifact nodes.
    pub transformation: Option<TransformationInfo>,

    /// Optional caller-supplied content digest.
    ///
    /// The value is opaque. This module does not claim that it is a secure
    /// digest or validate its algorithm.
    pub content_digest: Option<String>,

    /// Extensible metadata.
    ///
    /// A sorted map provides deterministic iteration and serialization order.
    pub metadata: BTreeMap<String, String>,
}

impl ProvenanceNode {
    /// Creates a provenance node.
    pub fn new(
        id: ProvenanceNodeId,
        kind: ProvenanceNodeKind,
    ) -> Self {
        Self {
            id,
            kind,
            source_location: None,
            transformation: None,
            content_digest: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Attaches source-location information.
    #[must_use]
    pub fn with_source_location(
        mut self,
        location: ProvenanceSourceLocation,
    ) -> Self {
        self.source_location = Some(location);
        self
    }

    /// Attaches transformation information.
    #[must_use]
    pub fn with_transformation(
        mut self,
        transformation: TransformationInfo,
    ) -> Self {
        self.transformation = Some(transformation);
        self
    }

    /// Attaches an opaque content digest.
    pub fn with_content_digest(
        mut self,
        digest: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let digest = digest.into();

        if digest.trim().is_empty() {
            return Err(ProvenanceError::EmptyDigest);
        }

        self.content_digest = Some(digest);
        Ok(self)
    }

    /// Adds or replaces metadata.
    ///
    /// Empty keys are rejected. Empty values are allowed because they may
    /// carry meaning in a caller-defined metadata schema.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, ProvenanceError> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(ProvenanceError::EmptyMetadataKey);
        }

        Ok(self.metadata.insert(key, value.into()))
    }
}

/// Relationship between two provenance nodes.
///
/// Edges point from an input or parent artifact to a derived artifact.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProvenanceEdgeKind {
    /// The destination was derived from the source.
    DerivedFrom,

    /// The source was consumed as an input to the destination.
    InputTo,

    /// The destination was generated by a transformation.
    TransformedInto,

    /// The destination was linked into a package.
    LinkedInto,

    /// A caller-defined relationship.
    Custom(String),
}

/// A directed provenance edge.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProvenanceEdge {
    /// Parent or input node.
    pub from: ProvenanceNodeId,

    /// Child or derived node.
    pub to: ProvenanceNodeId,

    /// Relationship type.
    pub kind: ProvenanceEdgeKind,
}

impl ProvenanceEdge {
    /// Creates an edge.
    #[must_use]
    pub fn new(
        from: ProvenanceNodeId,
        to: ProvenanceNodeId,
        kind: ProvenanceEdgeKind,
    ) -> Self {
        Self { from, to, kind }
    }
}

/// A directed acyclic graph recording compiler artifact lineage.
///
/// Nodes and edges are stored in ordered collections to make iteration
/// deterministic. The graph rejects cycles and dangling references.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProvenanceGraph {
    nodes: BTreeMap<ProvenanceNodeId, ProvenanceNode>,
    edges: BTreeSet<ProvenanceEdge>,
}

impl ProvenanceGraph {
    /// Creates an empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns whether the graph contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns a node by identifier.
    #[must_use]
    pub fn node(&self, id: &ProvenanceNodeId) -> Option<&ProvenanceNode> {
        self.nodes.get(id)
    }

    /// Iterates over nodes in deterministic identifier order.
    pub fn nodes(
        &self,
    ) -> impl Iterator<Item = (&ProvenanceNodeId, &ProvenanceNode)> {
        self.nodes.iter()
    }

    /// Iterates over edges in deterministic order.
    pub fn edges(&self) -> impl Iterator<Item = &ProvenanceEdge> {
        self.edges.iter()
    }

    /// Inserts a node.
    ///
    /// Duplicate identifiers are rejected rather than silently replacing
    /// existing lineage information.
    pub fn add_node(
        &mut self,
        node: ProvenanceNode,
    ) -> Result<(), ProvenanceError> {
        let id = node.id.clone();

        if self.nodes.contains_key(&id) {
            return Err(ProvenanceError::DuplicateNode(id));
        }

        self.nodes.insert(id, node);
        Ok(())
    }

    /// Inserts a directed edge.
    ///
    /// Both endpoints must already exist. An edge that would create a cycle
    /// is rejected. Duplicate edges are rejected.
    pub fn add_edge(
        &mut self,
        edge: ProvenanceEdge,
    ) -> Result<(), ProvenanceError> {
        if !self.nodes.contains_key(&edge.from) {
            return Err(ProvenanceError::UnknownNode(edge.from));
        }

        if !self.nodes.contains_key(&edge.to) {
            return Err(ProvenanceError::UnknownNode(edge.to));
        }

        if edge.from == edge.to {
            return Err(ProvenanceError::CycleDetected);
        }

        if self.edges.contains(&edge) {
            return Err(ProvenanceError::DuplicateEdge);
        }

        // Adding from -> to creates a cycle exactly when `from` is already
        // reachable from `to`.
        if self.is_reachable(&edge.to, &edge.from) {
            return Err(ProvenanceError::CycleDetected);
        }

        self.edges.insert(edge);
        Ok(())
    }

    /// Returns the direct parents of a node.
    ///
    /// The returned references follow deterministic identifier order.
    pub fn parents(
        &self,
        id: &ProvenanceNodeId,
    ) -> Result<Vec<&ProvenanceNode>, ProvenanceError> {
        self.require_node(id)?;

        let mut result = Vec::new();

        for edge in &self.edges {
            if &edge.to == id {
                if let Some(node) = self.nodes.get(&edge.from) {
                    result.push(node);
                }
            }
        }

        Ok(result)
    }

    /// Returns the direct children of a node.
    ///
    /// The returned references follow deterministic identifier order.
    pub fn children(
        &self,
        id: &ProvenanceNodeId,
    ) -> Result<Vec<&ProvenanceNode>, ProvenanceError> {
        self.require_node(id)?;

        let mut result = Vec::new();

        for edge in &self.edges {
            if &edge.from == id {
                if let Some(node) = self.nodes.get(&edge.to) {
                    result.push(node);
                }
            }
        }

        Ok(result)
    }

    /// Returns all ancestors of a node.
    ///
    /// The node itself is not included. Traversal is iterative to avoid
    /// recursion-depth dependence on graph size.
    pub fn ancestors(
        &self,
        id: &ProvenanceNodeId,
    ) -> Result<BTreeSet<ProvenanceNodeId>, ProvenanceError> {
        self.require_node(id)?;

        let mut visited = BTreeSet::new();
        let mut pending = VecDeque::new();
        pending.push_back(id.clone());

        while let Some(current) = pending.pop_front() {
            for edge in &self.edges {
                if edge.to == current && visited.insert(edge.from.clone()) {
                    pending.push_back(edge.from.clone());
                }
            }
        }

        visited.remove(id);
        Ok(visited)
    }

    /// Returns all descendants of a node.
    ///
    /// The node itself is not included. Traversal is iterative.
    pub fn descendants(
        &self,
        id: &ProvenanceNodeId,
    ) -> Result<BTreeSet<ProvenanceNodeId>, ProvenanceError> {
        self.require_node(id)?;

        let mut visited = BTreeSet::new();
        let mut pending = VecDeque::new();
        pending.push_back(id.clone());

        while let Some(current) = pending.pop_front() {
            for edge in &self.edges {
                if edge.from == current && visited.insert(edge.to.clone()) {
                    pending.push_back(edge.to.clone());
                }
            }
        }

        visited.remove(id);
        Ok(visited)
    }

    /// Validates graph invariants.
    ///
    /// The graph's mutation APIs already enforce these invariants. This
    /// method is useful after deserialization or when importing graph data
    /// from another subsystem.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        for (id, node) in &self.nodes {
            if id != &node.id {
                return Err(ProvenanceError::NodeKeyMismatch);
            }
        }

        for edge in &self.edges {
            if !self.nodes.contains_key(&edge.from) {
                return Err(ProvenanceError::UnknownNode(edge.from.clone()));
            }

            if !self.nodes.contains_key(&edge.to) {
                return Err(ProvenanceError::UnknownNode(edge.to.clone()));
            }

            if edge.from == edge.to {
                return Err(ProvenanceError::CycleDetected);
            }
        }

        if self.contains_cycle() {
            return Err(ProvenanceError::CycleDetected);
        }

        Ok(())
    }

    /// Checks whether `target` is reachable from `start`.
    fn is_reachable(
        &self,
        start: &ProvenanceNodeId,
        target: &ProvenanceNodeId,
    ) -> bool {
        if start == target {
            return true;
        }

        let mut visited = BTreeSet::new();
        let mut pending = VecDeque::new();
        pending.push_back(start.clone());

        while let Some(current) = pending.pop_front() {
            if !visited.insert(current.clone()) {
                continue;
            }

            for edge in &self.edges {
                if edge.from == current {
                    if &edge.to == target {
                        return true;
                    }

                    if !visited.contains(&edge.to) {
                        pending.push_back(edge.to.clone());
                    }
                }
            }
        }

        false
    }

    /// Returns whether the graph contains a cycle.
    fn contains_cycle(&self) -> bool {
        // Kahn's algorithm. A DAG's topological traversal visits every node.
        let mut in_degree: BTreeMap<ProvenanceNodeId, usize> = self
            .nodes
            .keys()
            .cloned()
            .map(|id| (id, 0))
            .collect();

        for edge in &self.edges {
            if let Some(degree) = in_degree.get_mut(&edge.to) {
                *degree += 1;
            }
        }

        let mut ready = VecDeque::new();

        for (id, degree) in &in_degree {
            if *degree == 0 {
                ready.push_back(id.clone());
            }
        }

        let mut visited_count = 0usize;

        while let Some(current) = ready.pop_front() {
            visited_count += 1;

            for edge in &self.edges {
                if edge.from == current {
                    if let Some(degree) = in_degree.get_mut(&edge.to) {
                        *degree -= 1;

                        if *degree == 0 {
                            ready.push_back(edge.to.clone());
                        }
                    }
                }
            }
        }

        visited_count != self.nodes.len()
    }

    fn require_node(
        &self,
        id: &ProvenanceNodeId,
    ) -> Result<(), ProvenanceError> {
        if self.nodes.contains_key(id) {
            Ok(())
        } else {
            Err(ProvenanceError::UnknownNode(id.clone()))
        }
    }
}

/// Errors produced by provenance construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceError {
    EmptyIdentifier,
    EmptySourceLocation,
    InvalidSourceRange,
    EmptyTransformationName,
    EmptyTransformationVersion,
    EmptyFingerprint,
    EmptyDigest,
    EmptyMetadataKey,
    DuplicateNode(ProvenanceNodeId),
    DuplicateEdge,
    UnknownNode(ProvenanceNodeId),
    NodeKeyMismatch,
    CycleDetected,
}

impl fmt::Display for ProvenanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str("provenance identifier must not be empty")
            }
            Self::EmptySourceLocation => {
                formatter.write_str("provenance source location must not be empty")
            }
            Self::InvalidSourceRange => {
                formatter.write_str("provenance source range is invalid")
            }
            Self::EmptyTransformationName => {
                formatter.write_str("transformation name must not be empty")
            }
            Self::EmptyTransformationVersion => {
                formatter.write_str("transformation version must not be empty")
            }
            Self::EmptyFingerprint => {
                formatter.write_str("configuration fingerprint must not be empty")
            }
            Self::EmptyDigest => {
                formatter.write_str("content digest must not be empty")
            }
            Self::EmptyMetadataKey => {
                formatter.write_str("provenance metadata key must not be empty")
            }
            Self::DuplicateNode(id) => {
                write!(formatter, "duplicate provenance node: {id}")
            }
            Self::DuplicateEdge => {
                formatter.write_str("duplicate provenance edge")
            }
            Self::UnknownNode(id) => {
                write!(formatter, "unknown provenance node: {id}")
            }
            Self::NodeKeyMismatch => {
                formatter.write_str("provenance node key does not match its identifier")
            }
            Self::CycleDetected => {
                formatter.write_str("provenance graph must be acyclic")
            }
        }
    }
}

impl Error for ProvenanceError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> ProvenanceNodeId {
        ProvenanceNodeId::new(value).expect("test identifier should be valid")
    }

    fn node(value: &str, kind: ProvenanceNodeKind) -> ProvenanceNode {
        ProvenanceNode::new(id(value), kind)
    }

    fn edge(from: &str, to: &str) -> ProvenanceEdge {
        ProvenanceEdge::new(
            id(from),
            id(to),
            ProvenanceEdgeKind::DerivedFrom,
        )
    }

    #[test]
    fn accepts_empty_graph() {
        let graph = ProvenanceGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn inserts_nodes_and_edges() {
        let mut graph = ProvenanceGraph::new();

        graph
            .add_node(node("source", ProvenanceNodeKind::Source))
            .expect("source node should be inserted");

        graph
            .add_node(node(
                "ir",
                ProvenanceNodeKind::IntermediateRepresentation,
            ))
            .expect("IR node should be inserted");

        graph
            .add_edge(edge("source", "ir"))
            .expect("lineage edge should be inserted");

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_node_ids() {
        let mut graph = ProvenanceGraph::new();

        graph
            .add_node(node("same", ProvenanceNodeKind::Source))
            .expect("first node should be inserted");

        let result = graph.add_node(node("same", ProvenanceNodeKind::Package));

        assert!(matches!(
            result,
            Err(ProvenanceError::DuplicateNode(_))
        ));
    }

    #[test]
    fn rejects_edges_with_unknown_endpoints() {
        let mut graph = ProvenanceGraph::new();

        graph
            .add_node(node("known", ProvenanceNodeKind::Source))
            .expect("node should be inserted");

        let result = graph.add_edge(edge("known", "missing"));

        assert!(matches!(
            result,
            Err(ProvenanceError::UnknownNode(_))
        ));
    }

    #[test]
    fn rejects_self_edges() {
        let mut graph = ProvenanceGraph::new();

        graph
            .add_node(node("node", ProvenanceNodeKind::Source))
            .expect("node should be inserted");

        assert_eq!(
            graph.add_edge(edge("node", "node")),
            Err(ProvenanceError::CycleDetected)
        );
    }

    #[test]
    fn rejects_edges_that_create_cycles() {
        let mut graph = ProvenanceGraph::new();

        for name in ["a", "b", "c"] {
            graph
                .add_node(node(name, ProvenanceNodeKind::Analysis))
                .expect("node should be inserted");
        }

        graph.add_edge(edge("a", "b")).expect("edge should be inserted");
        graph.add_edge(edge("b", "c")).expect("edge should be inserted");

        assert_eq!(
            graph.add_edge(edge("c", "a")),
            Err(ProvenanceError::CycleDetected)
        );
    }

    #[test]
    fn reports_ancestors_and_descendants() {
        let mut graph = ProvenanceGraph::new();

        for name in ["source", "parsed", "ir", "artifact"] {
            graph
                .add_node(node(name, ProvenanceNodeKind::Analysis))
                .expect("node should be inserted");
        }

        graph
            .add_edge(edge("source", "parsed"))
            .expect("edge should be inserted");
        graph
            .add_edge(edge("parsed", "ir"))
            .expect("edge should be inserted");
        graph
            .add_edge(edge("ir", "artifact"))
            .expect("edge should be inserted");

        let ancestors = graph
            .ancestors(&id("artifact"))
            .expect("node should exist");

        assert!(ancestors.contains(&id("source")));
        assert!(ancestors.contains(&id("parsed")));
        assert!(ancestors.contains(&id("ir")));
        assert!(!ancestors.contains(&id("artifact")));

        let descendants = graph
            .descendants(&id("source"))
            .expect("node should exist");

        assert!(descendants.contains(&id("parsed")));
        assert!(descendants.contains(&id("ir")));
        assert!(descendants.contains(&id("artifact")));
        assert!(!descendants.contains(&id("source")));
    }

    #[test]
    fn rejects_empty_identifiers() {
        assert_eq!(
            ProvenanceNodeId::new("  "),
            Err(ProvenanceError::EmptyIdentifier)
        );
    }

    #[test]
    fn validates_source_ranges() {
        let location = ProvenanceSourceLocation::new("source.zm")
            .expect("source should be valid");

        assert!(location.with_range(2, 1, 3, 1).is_ok());
        assert_eq!(
            location.with_range(3, 1, 2, 1),
            Err(ProvenanceError::InvalidSourceRange)
        );
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut provenance_node =
            node("node", ProvenanceNodeKind::Source);

        provenance_node
            .insert_metadata("z", "last")
            .expect("metadata should be inserted");
        provenance_node
            .insert_metadata("a", "first")
            .expect("metadata should be inserted");

        let keys: Vec<&str> = provenance_node
            .metadata
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(keys, vec!["a", "z"]);
    }

    #[test]
    fn duplicate_edges_are_rejected() {
        let mut graph = ProvenanceGraph::new();

        graph
            .add_node(node("a", ProvenanceNodeKind::Source))
            .expect("node should be inserted");
        graph
            .add_node(node("b", ProvenanceNodeKind::Analysis))
            .expect("node should be inserted");

        graph.add_edge(edge("a", "b")).expect("edge should be inserted");

        assert_eq!(
            graph.add_edge(edge("a", "b")),
            Err(ProvenanceError::DuplicateEdge)
        );
    }
}