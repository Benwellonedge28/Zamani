
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Graph Model.
//!
//! Path: src/compiler/graph/model.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Defines the canonical structural model for compiler-wide graph
//! composition.
//!
//! A CompilationGraph is a domain-neutral collection of graph nodes
//! referring to entities owned by authoritative repository subsystems.
//!
//! The model provides graph identity, node identity, canonical entity
//! references, descriptive metadata, structural validation and
//! deterministic node lookup.
//!
//! It is deliberately independent of graph-edge semantics.
//!
//! # Canonical ownership
//!
//! This module owns:
//!
//! - CompilationGraph.
//! - CompilationGraphId.
//! - CompilationGraphNodeId.
//! - CompilationGraphNodeKind.
//! - CanonicalEntityRef.
//! - CompilationGraphNode.
//! - Graph-model structural validation.
//!
//! This module does NOT own:
//!
//! - Source grammar or AST.
//! - Semantic analysis.
//! - Compilation-unit dependency discovery.
//! - Pipeline-stage dependency ordering.
//! - ZUIR.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - HybridGraph.
//! - Domain-specific operation semantics.
//! - Graph edge semantics.
//! - Graph traversal algorithms.
//! - Physical resource allocation.
//! - Target selection.
//! - Routing or scheduling.
//! - Backend implementation.
//! - Runtime execution.
//!
//! # Integration authorities
//!
//! Compiler composition:
//!     crate::compiler
//!
//! Compilation-unit dependencies:
//!     crate::compiler::context::dependency_context
//!
//! Pipeline stages:
//!     crate::compiler::pipeline::stage
//!
//! Pipeline dependency ordering:
//!     crate::compiler::pipeline::ordering
//!
//! Canonical universal IR:
//!     crate::zuir
//!
//! Classical computation:
//!     crate::classical
//!
//! Quantum computation:
//!     crate::quantum::ir
//!
//! Hardware description:
//!     crate::hdl::hdlir
//!
//! AI and tensor computation:
//!     crate::ai
//!
//! Hybrid computation:
//!     crate::hybrid::graph
//!
//! Distributed computation:
//!     crate::distributed
//!
//! Backend realization:
//!     crate::backend
//!
//! Runtime execution:
//!     crate::runtime
//!
//! # Graph ownership
//!
//! The compiler graph is an integration and coordination model.
//!
//! It is NOT another representation of the program's semantic IR.
//!
//! A CanonicalEntityRef identifies an entity through its authoritative
//! owner, entity kind and entity identity.
//!
//! The compiler graph never copies the referenced entity's semantics.
//!
//! # Scalability
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Graph nodes.
//! - Computational domains.
//! - Compilation units.
//! - Functions.
//! - Modules.
//! - Pipeline stages.
//! - IR entities.
//! - Backend providers.
//! - Graph metadata entries.
//! - Metadata string lengths.
//! - Identifier lengths.
//!
//! Collections grow according to available resources.
//!
//! The model does not promise literal infinite memory or computation.
//!
//! Actual limits are determined by:
//!
//! - Available memory.
//! - Address-space capacity.
//! - Rust collection representation.
//! - Allocation behavior.
//! - Execution time.
//! - Explicit operational budgets enforced by higher-level owners.
//!
//! Resource exhaustion must not be interpreted as semantic invalidity.
//!
//! # Determinism
//!
//! - Node declaration order is preserved.
//! - Metadata is stored in BTreeMap.
//! - Node lookup uses a deterministic ordered index.
//! - No randomized iteration order is exposed.
//! - Node identities are not content hashes.
//!
//! Canonical content fingerprints belong to the repository's
//! reproducibility and fingerprinting authorities.
//!
//! # Serialization
//!
//! Deserialization validates local model invariants.
//!
//! It does not prove that referenced entities exist or that a
//! compilation graph is semantically complete.
//!
//! Those guarantees require the corresponding integration authorities.
//!
//! # Concurrency
//!
//! CompilationGraph is immutable after construction.
//!
//! Its immutable references can be shared between threads.
//!
//! Mutation must occur through a separate graph builder or a verified
//! transformation that produces a new graph.
//!
//! No global mutable state or internal worker threads are introduced.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::Deserialize;
use serde::Serialize;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the compiler graph model.
///
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Semantic-model version.
/// - Compiler release version.
/// - ZUIR version.
/// - Domain IR versions.
/// - Backend ABI versions.
/// - Runtime versions.
pub const COMPILATION_GRAPH_MODEL_API_VERSION: u16 = 1;

/// Current structural schema version.
///
/// This identifies the graph-model serialization schema.
/// It is not a hardware-capacity limit.
pub const COMPILATION_GRAPH_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for compiler graph model operations.
pub type GraphModelResult<T> = Result<T, GraphModelError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by the compiler graph model.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GraphModelError {
    /// An identifier is empty or whitespace-only.
    EmptyIdentifier {
        field: &'static str,
    },

    /// An identifier contains prohibited control characters.
    InvalidIdentifier {
        field: &'static str,
    },

    /// An invalid graph schema version was supplied.
    InvalidSchemaVersion {
        version: u16,
    },

    /// Two nodes have the same identity.
    DuplicateNode {
        node_id: String,
    },

    /// Graph index and node storage are inconsistent.
    InvalidNodeIndex,

    /// Metadata contains an invalid key.
    InvalidMetadataKey,

    /// Metadata contains prohibited control characters.
    InvalidMetadataValue,

    /// A descriptive label is invalid.
    InvalidLabel,

    /// A metadata accounting operation exceeded its representation.
    SizeOverflow,

    /// A node or graph failed structural validation.
    InvalidStructure {
        description: String,
    },
}

impl fmt::Display for GraphModelError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(
                    formatter,
                    "{field} cannot be empty"
                )
            }

            Self::InvalidIdentifier { field } => {
                write!(
                    formatter,
                    "{field} contains prohibited control characters"
                )
            }

            Self::InvalidSchemaVersion { version } => {
                write!(
                    formatter,
                    "invalid compilation graph schema version: {version}"
                )
            }

            Self::DuplicateNode { node_id } => {
                write!(
                    formatter,
                    "duplicate compilation graph node: {node_id}"
                )
            }

            Self::InvalidNodeIndex => {
                formatter.write_str(
                    "compilation graph node index is inconsistent"
                )
            }

            Self::InvalidMetadataKey => {
                formatter.write_str(
                    "compilation graph metadata contains an invalid key"
                )
            }

            Self::InvalidMetadataValue => {
                formatter.write_str(
                    "compilation graph metadata contains an invalid value"
                )
            }

            Self::InvalidLabel => {
                formatter.write_str(
                    "compilation graph label is invalid"
                )
            }

            Self::SizeOverflow => {
                formatter.write_str(
                    "compilation graph metadata size exceeds its representation"
                )
            }

            Self::InvalidStructure { description } => {
                write!(
                    formatter,
                    "invalid compilation graph structure: {description}"
                )
            }
        }
    }
}

impl Error for GraphModelError {}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validate an extensible logical identifier.
///
/// Identifiers are dynamically sized.
///
/// No arbitrary identifier-length ceiling is imposed.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> GraphModelResult<()> {
    if value.trim().is_empty() {
        return Err(
            GraphModelError::EmptyIdentifier {
                field,
            },
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            GraphModelError::InvalidIdentifier {
                field,
            },
        );
    }

    Ok(())
}

/// Validate descriptive text.
///
/// Empty metadata values are permitted.
/// Control characters are not.
fn validate_text(
    value: &str,
    field: &'static str,
) -> GraphModelResult<()> {
    if value.chars().any(char::is_control) {
        return Err(
            GraphModelError::InvalidIdentifier {
                field,
            },
        );
    }

    Ok(())
}

/// Validate deterministic metadata.
fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> GraphModelResult<()> {
    for (key, value) in metadata {
        if key.trim().is_empty()
            || key.chars().any(char::is_control)
        {
            return Err(
                GraphModelError::InvalidMetadataKey,
            );
        }

        if value.chars().any(char::is_control) {
            return Err(
                GraphModelError::InvalidMetadataValue,
            );
        }
    }

    Ok(())
}

/// Add two accounting quantities without wrapping.
fn checked_add(
    left: u128,
    right: u128,
) -> GraphModelResult<u128> {
    left.checked_add(right)
        .ok_or(GraphModelError::SizeOverflow)
}

// ============================================================================
// GRAPH IDENTITY
// ============================================================================

/// Globally namespaced logical identity of a compilation graph.
///
/// This identity is independent of:
///
/// - Physical machine identity.
/// - Operating system.
/// - Hardware.
/// - Backend.
/// - Runtime.
/// - Compilation worker.
/// - Memory address.
///
/// The identity is not a content hash.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize
)]
#[serde(try_from = "String")]
pub struct CompilationGraphId(String);

impl TryFrom<String> for CompilationGraphId {
    type Error = GraphModelError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CompilationGraphId {
    /// Construct a validated compilation graph identity.
    pub fn new(
        value: impl Into<String>,
    ) -> GraphModelResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "compilation graph identity",
        )?;

        Ok(Self(value))
    }

    /// Return the graph identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CompilationGraphId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// NODE IDENTITY
// ============================================================================

/// Identity of a node within a compilation graph.
///
/// Node identity is scoped by the containing CompilationGraph.
///
/// IDs are opaque logical identities and must not be interpreted as
/// physical addresses or array positions.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize
)]
#[serde(try_from = "String")]
pub struct CompilationGraphNodeId(String);

impl TryFrom<String> for CompilationGraphNodeId {
    type Error = GraphModelError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CompilationGraphNodeId {
    /// Construct a validated graph-node identity.
    pub fn new(
        value: impl Into<String>,
    ) -> GraphModelResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "compilation graph node identity",
        )?;

        Ok(Self(value))
    }

    /// Return the node identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CompilationGraphNodeId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// EXTENSIBLE NODE CLASSIFICATION
// ============================================================================

/// Open-world classification of a compiler graph node.
///
/// This is deliberately a string-backed identifier rather than a closed
/// enumeration of computing domains or compiler operations.
///
/// Examples:
///
/// - compiler.compilation_unit
/// - compiler.pipeline_stage
/// - compiler.transformation
/// - compiler.artifact
/// - compiler.analysis_result
/// - domain.classical.operation
/// - domain.quantum.operation
/// - domain.hdl.component
/// - domain.ai.computation
/// - domain.distributed.computation
///
/// These examples are descriptive, not a closed registry.
///
/// A new domain or provider can introduce a new classification without
/// modifying this type.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize
)]
#[serde(try_from = "String")]
pub struct CompilationGraphNodeKind(String);

impl TryFrom<String> for CompilationGraphNodeKind {
    type Error = GraphModelError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CompilationGraphNodeKind {
    /// Construct an extensible node classification.
    pub fn new(
        value: impl Into<String>,
    ) -> GraphModelResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "compilation graph node kind",
        )?;

        Ok(Self(value))
    }

    /// Return the classification identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CompilationGraphNodeKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// CANONICAL ENTITY REFERENCE
// ============================================================================

/// Reference to an entity owned by another authoritative subsystem.
///
/// The reference consists of:
///
/// - `authority`: the canonical owning subsystem.
/// - `entity_kind`: the kind of entity within that subsystem.
/// - `entity_id`: the identity assigned by that subsystem.
///
/// Example:
///
/// authority  = "compiler.pipeline.stage"
/// entity_kind = "stage"
/// entity_id   = "semantic.validation"
///
/// Another example:
///
/// authority  = "quantum.ir"
/// entity_kind = "operation"
/// entity_id   = "operation-identifier"
///
/// These identifiers are examples only.
///
/// This structure does not resolve or interpret the referenced entity.
///
/// The authoritative subsystem remains responsible for:
///
/// - Entity existence.
/// - Entity validation.
/// - Semantic meaning.
/// - Version compatibility.
/// - Lifecycle.
/// - Access control.
///
/// No domain inventory is hard-coded.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize
)]
#[serde(try_from = "CanonicalEntityRefWire")]
pub struct CanonicalEntityRef {
    authority: String,
    entity_kind: String,
    entity_id: String,
}

/// Validated deserialization representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CanonicalEntityRefWire {
    authority: String,
    entity_kind: String,
    entity_id: String,
}

impl TryFrom<CanonicalEntityRefWire> for CanonicalEntityRef {
    type Error = GraphModelError;

    fn try_from(
        value: CanonicalEntityRefWire,
    ) -> Result<Self, Self::Error> {
        Self::new(
            value.authority,
            value.entity_kind,
            value.entity_id,
        )
    }
}

impl CanonicalEntityRef {
    /// Construct a reference to an authoritative repository entity.
    pub fn new(
        authority: impl Into<String>,
        entity_kind: impl Into<String>,
        entity_id: impl Into<String>,
    ) -> GraphModelResult<Self> {
        let value = Self {
            authority: authority.into(),
            entity_kind: entity_kind.into(),
            entity_id: entity_id.into(),
        };

        value.validate()?;

        Ok(value)
    }

    /// Return the authoritative owning subsystem.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the canonical entity classification.
    #[must_use]
    pub fn entity_kind(&self) -> &str {
        &self.entity_kind
    }

    /// Return the canonical entity identity.
    #[must_use]
    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }

    /// Validate local reference invariants.
    ///
    /// This does not establish that the referenced entity exists.
    pub fn validate(&self) -> GraphModelResult<()> {
        validate_identifier(
            &self.authority,
            "canonical entity authority",
        )?;

        validate_identifier(
            &self.entity_kind,
            "canonical entity kind",
        )?;

        validate_identifier(
            &self.entity_id,
            "canonical entity identity",
        )?;

        Ok(())
    }

    /// Estimate the UTF-8 bytes used by the reference's strings.
    ///
    /// This is accounting information, not an exact heap-size measurement.
    pub fn estimated_bytes(&self) -> GraphModelResult<u128> {
        let total = self.authority.len() as u128;

        let total = checked_add(
            total,
            self.entity_kind.len() as u128,
        )?;

        checked_add(
            total,
            self.entity_id.len() as u128,
        )
    }
}

impl fmt::Display for CanonicalEntityRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}::{}::{}",
            self.authority,
            self.entity_kind,
            self.entity_id,
        )
    }
}

// ============================================================================
// GRAPH NODE
// ============================================================================

/// One compiler graph node.
///
/// A node is a reference-bearing structural entry.
///
/// It does not copy the semantics or implementation of its referenced
/// entity.
///
/// `kind` describes the compiler graph's classification.
///
/// `entity` identifies the authoritative repository entity.
///
/// `label` and `metadata` are descriptive only.
///
/// They must never override canonical semantic information.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(try_from = "CompilationGraphNodeWire")]
pub struct CompilationGraphNode {
    id: CompilationGraphNodeId,
    kind: CompilationGraphNodeKind,
    entity: CanonicalEntityRef,
    label: Option<String>,
    metadata: BTreeMap<String, String>,
}

/// Validated graph-node deserialization representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CompilationGraphNodeWire {
    id: CompilationGraphNodeId,
    kind: CompilationGraphNodeKind,
    entity: CanonicalEntityRef,

    #[serde(default)]
    label: Option<String>,

    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl TryFrom<CompilationGraphNodeWire> for CompilationGraphNode {
    type Error = GraphModelError;

    fn try_from(
        value: CompilationGraphNodeWire,
    ) -> Result<Self, Self::Error> {
        Self::new(
            value.id,
            value.kind,
            value.entity,
            value.label,
            value.metadata,
        )
    }
}

impl CompilationGraphNode {
    /// Construct and validate a compilation graph node.
    pub fn new(
        id: CompilationGraphNodeId,
        kind: CompilationGraphNodeKind,
        entity: CanonicalEntityRef,
        label: Option<String>,
        metadata: BTreeMap<String, String>,
    ) -> GraphModelResult<Self> {
        let value = Self {
            id,
            kind,
            entity,
            label,
            metadata,
        };

        value.validate()?;

        Ok(value)
    }

    /// Return the graph-local node identity.
    #[must_use]
    pub fn id(&self) -> &CompilationGraphNodeId {
        &self.id
    }

    /// Return the node classification.
    #[must_use]
    pub fn kind(&self) -> &CompilationGraphNodeKind {
        &self.kind
    }

    /// Return the referenced canonical entity.
    #[must_use]
    pub fn entity(&self) -> &CanonicalEntityRef {
        &self.entity
    }

    /// Return the optional descriptive label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Return deterministic descriptive metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validate all local node invariants.
    pub fn validate(&self) -> GraphModelResult<()> {
        validate_identifier(
            self.id.as_str(),
            "compilation graph node identity",
        )?;

        validate_identifier(
            self.kind.as_str(),
            "compilation graph node kind",
        )?;

        self.entity.validate()?;

        if let Some(label) = &self.label {
            if label.trim().is_empty()
                || label.chars().any(char::is_control)
            {
                return Err(
                    GraphModelError::InvalidLabel,
                );
            }
        }

        validate_metadata(&self.metadata)?;

        Ok(())
    }

    /// Estimate the UTF-8 bytes held by node metadata and references.
    ///
    /// This is an accounting estimate rather than an exact heap-size
    /// measurement.
    pub fn estimated_metadata_bytes(
        &self,
    ) -> GraphModelResult<u128> {
        let mut total =
            self.id.as_str().len() as u128;

        total = checked_add(
            total,
            self.kind.as_str().len() as u128,
        )?;

        total = checked_add(
            total,
            self.entity.estimated_bytes()?,
        )?;

        if let Some(label) = &self.label {
            total = checked_add(
                total,
                label.len() as u128,
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
// COMPILATION GRAPH
// ============================================================================

/// Canonical structural model of compiler-wide graph composition.
///
/// This model stores nodes and their canonical entity references.
///
/// It deliberately does not embed graph edges.
///
/// Edge ownership belongs to `compiler::graph::edges`.
///
/// Compilation-unit dependencies remain owned by
/// `compiler::context::dependency_context`.
///
/// Pipeline dependencies remain owned by
/// `compiler::pipeline::ordering`.
///
/// This separation prevents multiple competing dependency graphs.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(try_from = "CompilationGraphWire")]
pub struct CompilationGraph {
    id: CompilationGraphId,

    schema_version: u16,

    nodes: Vec<CompilationGraphNode>,

    metadata: BTreeMap<String, String>,

    /// Derived lookup index.
    ///
    /// The index is rebuilt during validated construction and
    /// deserialization.
    ///
    /// It is never serialized as authoritative graph data.
    #[serde(skip)]
    node_index: BTreeMap<CompilationGraphNodeId, usize>,
}

/// Wire representation used for validated deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct CompilationGraphWire {
    id: CompilationGraphId,

    schema_version: u16,

    #[serde(default)]
    nodes: Vec<CompilationGraphNode>,

    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl TryFrom<CompilationGraphWire> for CompilationGraph {
    type Error = GraphModelError;

    fn try_from(
        value: CompilationGraphWire,
    ) -> Result<Self, Self::Error> {
        Self::new_with_schema_version(
            value.id,
            value.schema_version,
            value.nodes,
            value.metadata,
        )
    }
}

impl CompilationGraph {
    /// Construct a graph using the current structural schema version.
    pub fn new(
        id: CompilationGraphId,
        nodes: Vec<CompilationGraphNode>,
        metadata: BTreeMap<String, String>,
    ) -> GraphModelResult<Self> {
        Self::new_with_schema_version(
            id,
            COMPILATION_GRAPH_SCHEMA_VERSION,
            nodes,
            metadata,
        )
    }

    /// Construct a graph with an explicitly supplied schema version.
    ///
    /// This is intended for controlled interchange and migration.
    ///
    /// Version compatibility must be established by the appropriate
    /// compatibility authority.
    pub fn new_with_schema_version(
        id: CompilationGraphId,
        schema_version: u16,
        nodes: Vec<CompilationGraphNode>,
        metadata: BTreeMap<String, String>,
    ) -> GraphModelResult<Self> {
        if schema_version == 0 {
            return Err(
                GraphModelError::InvalidSchemaVersion {
                    version: schema_version,
                },
            );
        }

        validate_metadata(&metadata)?;

        let mut node_index =
            BTreeMap::new();

        for (position, node) in nodes.iter().enumerate() {
            node.validate()?;

            if node_index
                .insert(node.id.clone(), position)
                .is_some()
            {
                return Err(
                    GraphModelError::DuplicateNode {
                        node_id: node.id.to_string(),
                    },
                );
            }
        }

        let graph = Self {
            id,
            schema_version,
            nodes,
            metadata,
            node_index,
        };

        graph.validate()?;

        Ok(graph)
    }

    /// Return the graph identity.
    #[must_use]
    pub fn id(&self) -> &CompilationGraphId {
        &self.id
    }

    /// Return the structural schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Return whether this graph declares the current model schema.
    ///
    /// This does not establish compatibility with every consumer.
    #[must_use]
    pub const fn uses_current_schema(&self) -> bool {
        self.schema_version
            == COMPILATION_GRAPH_SCHEMA_VERSION
    }

    /// Return all graph nodes in declaration order.
    #[must_use]
    pub fn nodes(&self) -> &[CompilationGraphNode] {
        &self.nodes
    }

    /// Return deterministic graph metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Return the number of registered nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return whether the graph contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Check whether a graph node exists.
    #[must_use]
    pub fn contains_node(
        &self,
        id: &CompilationGraphNodeId,
    ) -> bool {
        self.node_index.contains_key(id)
    }

    /// Retrieve a node by its identity.
    ///
    /// Lookup uses the derived ordered index.
    #[must_use]
    pub fn node(
        &self,
        id: &CompilationGraphNodeId,
    ) -> Option<&CompilationGraphNode> {
        let position = self.node_index.get(id)?;

        self.nodes.get(*position)
    }

    /// Return node identities in deterministic identifier order.
    ///
    /// This does not change declaration order in the graph.
    pub fn node_ids_canonical_order(
        &self,
    ) -> impl Iterator<Item = &CompilationGraphNodeId> {
        self.node_index.keys()
    }

    /// Return the declaration position of a node.
    #[must_use]
    pub fn node_position(
        &self,
        id: &CompilationGraphNodeId,
    ) -> Option<usize> {
        self.node_index.get(id).copied()
    }

    /// Validate the complete local graph-model structure.
    ///
    /// This verifies:
    ///
    /// - Graph identity.
    /// - Schema-version representation.
    /// - Node validity.
    /// - Unique node identities.
    /// - Node-index consistency.
    /// - Metadata validity.
    ///
    /// This does not resolve canonical entity references.
    pub fn validate(&self) -> GraphModelResult<()> {
        validate_identifier(
            self.id.as_str(),
            "compilation graph identity",
        )?;

        if self.schema_version == 0 {
            return Err(
                GraphModelError::InvalidSchemaVersion {
                    version: self.schema_version,
                },
            );
        }

        validate_metadata(&self.metadata)?;

        if self.node_index.len() != self.nodes.len() {
            return Err(
                GraphModelError::InvalidNodeIndex,
            );
        }

        let mut seen = BTreeSet::new();

        for (position, node) in self.nodes.iter().enumerate() {
            node.validate()?;

            if !seen.insert(node.id.clone()) {
                return Err(
                    GraphModelError::DuplicateNode {
                        node_id: node.id.to_string(),
                    },
                );
            }

            match self.node_index.get(&node.id) {
                Some(index) if *index == position => {}

                _ => {
                    return Err(
                        GraphModelError::InvalidNodeIndex,
                    );
                }
            }
        }

        Ok(())
    }

    /// Estimate UTF-8 bytes held by graph identities and metadata.
    ///
    /// This is not an exact heap-size measurement.
    ///
    /// It excludes allocator overhead, collection capacities,
    /// pointer sizes and internal collection structures.
    pub fn estimated_metadata_bytes(
        &self,
    ) -> GraphModelResult<u128> {
        let mut total =
            self.id.as_str().len() as u128;

        for node in &self.nodes {
            total = checked_add(
                total,
                node.estimated_metadata_bytes()?,
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

    fn graph_id() -> CompilationGraphId {
        CompilationGraphId::new(
            "test.compilation.graph"
        )
        .expect("valid graph identity")
    }

    fn node_id(
        value: &str,
    ) -> CompilationGraphNodeId {
        CompilationGraphNodeId::new(value)
            .expect("valid node identity")
    }

    fn node_kind(
        value: &str,
    ) -> CompilationGraphNodeKind {
        CompilationGraphNodeKind::new(value)
            .expect("valid node kind")
    }

    fn entity_ref(
        authority: &str,
        kind: &str,
        id: &str,
    ) -> CanonicalEntityRef {
        CanonicalEntityRef::new(
            authority,
            kind,
            id,
        )
        .expect("valid entity reference")
    }

    fn sample_node(
        id: &str,
        authority: &str,
        entity_id: &str,
    ) -> CompilationGraphNode {
        CompilationGraphNode::new(
            node_id(id),
            node_kind("compiler.compilation_unit"),
            entity_ref(
                authority,
                "compilation_unit",
                entity_id,
            ),
            None,
            BTreeMap::new(),
        )
        .expect("valid graph node")
    }

    #[test]
    fn empty_graph_is_valid() {
        let graph = CompilationGraph::new(
            graph_id(),
            Vec::new(),
            BTreeMap::new(),
        )
        .expect("empty graph must be valid");

        assert!(graph.is_empty());
        assert_eq!(graph.node_count(), 0);
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn graph_preserves_node_declaration_order() {
        let graph = CompilationGraph::new(
            graph_id(),
            vec![
                sample_node(
                    "node.b",
                    "compiler.context",
                    "unit.b",
                ),
                sample_node(
                    "node.a",
                    "compiler.context",
                    "unit.a",
                ),
            ],
            BTreeMap::new(),
        )
        .expect("valid graph");

        assert_eq!(
            graph.nodes()[0].id().as_str(),
            "node.b"
        );

        assert_eq!(
            graph.nodes()[1].id().as_str(),
            "node.a"
        );
    }

    #[test]
    fn graph_supports_fast_identity_lookup() {
        let graph = CompilationGraph::new(
            graph_id(),
            vec![
                sample_node(
                    "node.a",
                    "compiler.context",
                    "unit.a",
                ),
                sample_node(
                    "node.b",
                    "compiler.context",
                    "unit.b",
                ),
            ],
            BTreeMap::new(),
        )
        .expect("valid graph");

        let requested = node_id("node.b");

        let node = graph
            .node(&requested)
            .expect("node must exist");

        assert_eq!(
            node.entity().entity_id(),
            "unit.b"
        );
    }

    #[test]
    fn graph_rejects_duplicate_node_identities() {
        let result = CompilationGraph::new(
            graph_id(),
            vec![
                sample_node(
                    "duplicate",
                    "compiler.context",
                    "unit.a",
                ),
                sample_node(
                    "duplicate",
                    "compiler.context",
                    "unit.b",
                ),
            ],
            BTreeMap::new(),
        );

        assert!(matches!(
            result,
            Err(GraphModelError::DuplicateNode { .. })
        ));
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert!(
            CompilationGraphId::new(" ").is_err()
        );

        assert!(
            CompilationGraphNodeId::new("").is_err()
        );

        assert!(
            CompilationGraphNodeKind::new("").is_err()
        );
    }

    #[test]
    fn identifiers_reject_control_characters() {
        assert!(
            CompilationGraphId::new(
                "graph\ninvalid"
            ).is_err()
        );

        assert!(
            CompilationGraphNodeId::new(
                "node\0invalid"
            ).is_err()
        );
    }

    #[test]
    fn canonical_entity_reference_is_extensible() {
        let reference = entity_ref(
            "future.domain.registry",
            "future.computation",
            "entity-001",
        );

        assert_eq!(
            reference.authority(),
            "future.domain.registry"
        );

        assert_eq!(
            reference.entity_kind(),
            "future.computation"
        );
    }

    #[test]
    fn metadata_has_deterministic_order() {
        let mut metadata = BTreeMap::new();

        metadata.insert(
            "zeta".to_owned(),
            "last".to_owned(),
        );

        metadata.insert(
            "alpha".to_owned(),
            "first".to_owned(),
        );

        let graph = CompilationGraph::new(
            graph_id(),
            Vec::new(),
            metadata,
        )
        .expect("valid graph");

        let keys: Vec<&str> = graph
            .metadata()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(
            keys,
            vec!["alpha", "zeta"]
        );
    }

    #[test]
    fn graph_rejects_invalid_schema_version() {
        let result =
            CompilationGraph::new_with_schema_version(
                graph_id(),
                0,
                Vec::new(),
                BTreeMap::new(),
            );

        assert!(matches!(
            result,
            Err(GraphModelError::InvalidSchemaVersion {
                version: 0
            })
        ));
    }

    #[test]
    fn graph_serialization_round_trip() {
        let graph = CompilationGraph::new(
            graph_id(),
            vec![
                sample_node(
                    "node.a",
                    "compiler.context",
                    "unit.a",
                ),
            ],
            BTreeMap::new(),
        )
        .expect("valid graph");

        let serialized =
            serde_json::to_string(&graph)
                .expect("serialization must succeed");

        let restored: CompilationGraph =
            serde_json::from_str(&serialized)
                .expect("validated deserialization must succeed");

        assert_eq!(graph, restored);

        assert!(restored.validate().is_ok());

        assert!(
            restored.contains_node(
                &node_id("node.a")
            )
        );
    }

    #[test]
    fn deserialization_rejects_duplicate_nodes() {
        let serialized = r#"
        {
            "id": "graph",
            "schema_version": 1,
            "nodes": [
                {
                    "id": "duplicate",
                    "kind": "compiler.unit",
                    "entity": {
                        "authority": "compiler.context",
                        "entity_kind": "unit",
                        "entity_id": "unit.a"
                    }
                },
                {
                    "id": "duplicate",
                    "kind": "compiler.unit",
                    "entity": {
                        "authority": "compiler.context",
                        "entity_kind": "unit",
                        "entity_id": "unit.b"
                    }
                }
            ],
            "metadata": {}
        }
        "#;

        let restored: Result<
            CompilationGraph,
            _,
        > = serde_json::from_str(serialized);

        assert!(restored.is_err());
    }

    #[test]
    fn estimated_metadata_bytes_are_available() {
        let graph = CompilationGraph::new(
            graph_id(),
            vec![
                sample_node(
                    "node.a",
                    "compiler.context",
                    "unit.a",
                ),
            ],
            BTreeMap::new(),
        )
        .expect("valid graph");

        assert!(
            graph.estimated_metadata_bytes().is_ok()
        );
    }

    #[test]
    fn canonical_node_order_is_deterministic() {
        let graph = CompilationGraph::new(
            graph_id(),
            vec![
                sample_node(
                    "node.z",
                    "compiler.context",
                    "unit.z",
                ),
                sample_node(
                    "node.a",
                    "compiler.context",
                    "unit.a",
                ),
            ],
            BTreeMap::new(),
        )
        .expect("valid graph");

        let ids: Vec<&str> = graph
            .node_ids_canonical_order()
            .map(CompilationGraphNodeId::as_str)
            .collect();

        assert_eq!(
            ids,
            vec!["node.a", "node.z"]
        );
    }
}
