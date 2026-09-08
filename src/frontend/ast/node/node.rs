//! # Zamani Frontend AST — Canonical Node
//!
//! This module defines the common metadata-bearing identity of every node in
//! the native Zamani Abstract Syntax Tree (AST).
//!
//! ## Architectural position
//!
//! ```text
//! source
//!   │
//!   ▼
//! lexer
//!   │
//!   ▼
//! parser
//!   │
//!   ▼
//! ┌───────────────────────────────┐
//! │ Native Zamani AST             │
//! │                               │
//! │ node.rs  ← this module        │
//! └───────────────┬───────────────┘
//!                 │
//!                 ▼
//!          structural validation
//!                 │
//!                 ▼
//!          semantic analysis
//!                 │
//!                 ▼
//!            semantic model
//!                 │
//!                 ▼
//!                ZUIR
//!                 │
//!        ┌────────┼────────┐
//!        ▼        ▼        ▼
//!     classical quantum   HDL
//!       IR       IR       IR
//! ```
//!
//! ## Purpose
//!
//! [`Node`] is the common source-level identity and metadata container used by
//! concrete AST nodes. It intentionally contains only information that is
//! meaningful at the source/AST layer:
//!
//! - stable AST identity;
//! - source location;
//! - source-level node classification;
//! - AST metadata.
//!
//! It does **not** contain:
//!
//! - resolved symbols;
//! - resolved types;
//! - semantic domains;
//! - backend identifiers;
//! - physical resources;
//! - hardware topology;
//! - qubit mappings;
//! - scheduling information;
//! - calibration information;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - optimization state;
//! - runtime state.
//!
//! Those concepts belong to later compiler layers.
//!
//! ## POCO-REAF
//!
//! The node representation deliberately contains no machine-size information.
//! Consequently, the same source AST model can represent a program intended
//! for one resource, many resources, a simulator, a quantum computer, a
//! heterogeneous machine, or a future computational system.
//!
//! ```text
//! Program Once
//!       │
//!       ▼
//! Canonical AST
//!       │
//!       ▼
//! Semantic analysis
//!       │
//!       ▼
//! ZUIR
//!       │
//!       ▼
//! Target/resource discovery
//!       │
//!       ▼
//! Mapping / optimization / scheduling / execution
//! ```
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational AST infrastructure:
//!
//! - [`super::node_id::NodeId`];
//! - [`super::node_kind::NodeKind`];
//! - [`super::metadata::NodeMetadata`];
//! - [`super::source::Span`];
//! - the Rust standard library;
//! - `serde` for stable serialization.
//!
//! It must never depend on semantic analysis, ZUIR, quantum backends,
//! hardware, optimization, scheduling, routing, execution, or runtime modules.
//!
//! ## Stability contract
//!
//! The fields of [`Node`] are intentionally private. Concrete AST nodes should
//! expose their own domain-specific source structure while delegating common
//! identity/location/metadata behavior to this type.
//!
//! This prevents downstream code from depending on the physical representation
//! of the node and allows internal storage improvements without changing the
//! public AST contract.
//!
//! ## Thread safety
//!
//! [`Node`] contains no mutable global state and does not contain thread-local
//! or process-local execution state. Its thread-safety therefore follows from
//! its field types. ASTs may safely be shared immutably between compiler
//! phases when their constituent types are `Send + Sync`.
//!
//! ## Safety
//!
//! This implementation contains no `unsafe` code.

use serde::{Deserialize, Serialize};
use std::fmt;

use super::metadata::NodeMetadata;
use super::node_id::NodeId;
use super::node_kind::NodeKind;
use super::source::Span;

/// The canonical AST node schema version.
///
/// This is the version of the in-memory node contract, not the Zamani
/// language version and not the serialized file format version.
///
/// These version spaces must remain independent.
pub const AST_NODE_SCHEMA_VERSION: u16 = 1;

/// Common source-level identity and metadata for a Zamani AST node.
///
/// `Node` is intentionally small in semantic responsibility. It identifies
/// and locates a node but does not attempt to represent the meaning resolved
/// by later compiler phases.
///
/// # Invariants
///
/// A valid `Node` satisfies:
///
/// 1. Its [`NodeId`] is valid according to the `NodeId` contract.
/// 2. Its [`NodeKind`] is a valid canonical or registered extension kind.
/// 3. Its [`Span`] belongs to the source infrastructure's valid coordinate
///    system.
/// 4. Its metadata is structurally valid according to `NodeMetadata`.
/// 5. No backend, machine, hardware, or execution state is stored in it.
///
/// Structural relationships between nodes are validated by the AST validation
/// layer rather than by this low-level container.
///
/// # Determinism
///
/// `Node` does not generate IDs itself. ID allocation is delegated to
/// [`NodeId`]'s allocation policy. This is important because a low-level node
/// must not secretly depend on:
///
/// - memory addresses;
/// - pointer identity;
/// - hash-map iteration order;
/// - thread scheduling;
/// - wall-clock time;
/// - random state.
///
/// # Cloning
///
/// Cloning a node clones its logical AST information. It does not create a new
/// identity. This is intentional: [`NodeId`] represents the identity of the
/// represented AST node, not an ownership identity of the Rust value.
///
/// Code that needs a distinct AST node must explicitly allocate a new
/// [`NodeId`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Node {
    id: NodeId,
    kind: NodeKind,
    span: Span,
    metadata: NodeMetadata,
}

impl Node {
    /// Creates a new canonical AST node.
    ///
    /// This constructor does not perform semantic validation. It establishes
    /// the common structural fields and leaves cross-node invariants to the
    /// structural AST validator.
    ///
    /// # Arguments
    ///
    /// * `id` - Stable identity allocated by the AST construction layer.
    /// * `kind` - Canonical or registered extension node kind.
    /// * `span` - Source range associated with the node.
    /// * `metadata` - Source-level metadata associated with the node.
    ///
    /// # Integration
    ///
    /// Parser:
    ///
    /// ```text
    /// parser
    ///   └── allocates NodeId
    ///       └── determines NodeKind
    ///           └── attaches Span
    ///               └── creates Node
    /// ```
    ///
    /// Semantic analysis consumes the resulting node but must not add semantic
    /// state to this object.
    pub fn new(
        id: NodeId,
        kind: NodeKind,
        span: Span,
        metadata: NodeMetadata,
    ) -> Self {
        Self {
            id,
            kind,
            span,
            metadata,
        }
    }

    /// Creates a node with empty/default metadata.
    ///
    /// This is useful for parser paths where metadata is not present.
    ///
    /// The constructor intentionally does not manufacture a source span or
    /// node ID. Those are required inputs because silently manufacturing either
    /// value would make deterministic compilation harder to reason about.
    pub fn without_metadata(
        id: NodeId,
        kind: NodeKind,
        span: Span,
    ) -> Self {
        Self::new(id, kind, span, NodeMetadata::default())
    }

    /// Returns the stable AST node identity.
    #[inline]
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Returns the node classification.
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }

    /// Returns the node classification by value when the classification is
    /// cheaply cloneable.
    ///
    /// This method is provided separately from [`Self::kind`] so callers do
    /// not need to depend on the internal storage representation.
    #[inline]
    pub fn kind_owned(&self) -> NodeKind {
        self.kind.clone()
    }

    /// Returns the source span associated with this node.
    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// Returns the source span by value.
    ///
    /// The source-span type is expected to be cheap to clone. This method keeps
    /// callers independent of the field layout.
    #[inline]
    pub fn span_owned(&self) -> Span {
        self.span.clone()
    }

    /// Returns immutable access to node metadata.
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        &self.metadata
    }

    /// Returns mutable access to node metadata.
    ///
    /// This is intentionally limited to metadata. Callers cannot mutate the
    /// node's identity, kind, or source span through this method.
    ///
    /// Structural AST transformations should create a new node or use the
    /// dedicated AST transformation/fold infrastructure rather than mutating
    /// arbitrary node identity fields.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        &mut self.metadata
    }

    /// Replaces the node metadata and returns the previous metadata.
    ///
    /// This operation does not change node identity or source structure.
    #[inline]
    pub fn replace_metadata(
        &mut self,
        metadata: NodeMetadata,
    ) -> NodeMetadata {
        std::mem::replace(&mut self.metadata, metadata)
    }

    /// Returns whether this node has a particular node kind.
    #[inline]
    pub fn is_kind(&self, kind: &NodeKind) -> bool {
        &self.kind == kind
    }

    /// Returns whether this node belongs to an extension kind.
    ///
    /// This delegates classification to [`NodeKind`] rather than encoding
    /// extension knowledge in this module.
    #[inline]
    pub fn is_extension(&self) -> bool {
        self.kind.is_extension()
    }

    /// Returns whether this node belongs to the native core AST.
    #[inline]
    pub fn is_core(&self) -> bool {
        self.kind.is_core()
    }

    /// Returns the schema version of this node contract.
    #[inline]
    pub const fn schema_version() -> u16 {
        AST_NODE_SCHEMA_VERSION
    }

    /// Replaces the source span.
    ///
    /// This operation is intended for controlled parser recovery and AST
    /// transformation infrastructure. The caller is responsible for ensuring
    /// that the replacement span is structurally valid.
    ///
    /// Semantic information is never derived from this method.
    #[inline]
    pub fn replace_span(&mut self, span: Span) -> Span {
        std::mem::replace(&mut self.span, span)
    }

    /// Returns the previous span while replacing it with `span`.
    ///
    /// This alias exists for transformation code where the operation reads more
    /// naturally as an AST edit.
    #[inline]
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    /// Returns the previous node kind while replacing it with `kind`.
    ///
    /// Changing a node's kind is a structural AST transformation and should
    /// generally be performed by parser recovery or dedicated transformation
    /// infrastructure rather than arbitrary semantic code.
    #[inline]
    pub fn replace_kind(&mut self, kind: NodeKind) -> NodeKind {
        std::mem::replace(&mut self.kind, kind)
    }

    /// Returns a compact diagnostic representation of this node.
    ///
    /// This method intentionally avoids printing metadata because metadata can
    /// contain large extension payloads. Diagnostics should not accidentally
    /// cause an enormous metadata allocation or log amplification.
    pub fn diagnostic_summary(&self) -> NodeDiagnosticSummary {
        NodeDiagnosticSummary {
            id: self.id,
            kind: self.kind.clone(),
            span: self.span.clone(),
        }
    }
}

impl Default for Node {
    /// Creates a structurally neutral node.
    ///
    /// This implementation is provided for generic containers and test
    /// infrastructure. Production parser code should normally use [`Node::new`]
    /// because silently using a default identity/kind/span can hide parser
    /// errors.
    ///
    /// The exact default values are delegated to the foundational AST types.
    fn default() -> Self {
        Self {
            id: NodeId::default(),
            kind: NodeKind::default(),
            span: Span::default(),
            metadata: NodeMetadata::default(),
        }
    }
}

/// Lightweight diagnostic representation of a node.
///
/// This intentionally excludes arbitrary metadata. A diagnostic system should
/// be able to inspect a node without materializing or serializing potentially
/// large extension payloads.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeDiagnosticSummary {
    /// Stable AST identity.
    pub id: NodeId,

    /// Canonical or extension node kind.
    pub kind: NodeKind,

    /// Source location.
    pub span: Span,
}

impl fmt::Display for NodeDiagnosticSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} at {}",
            self.kind,
            self.span
        )
    }
}

/// Trait implemented by concrete AST nodes that contain a [`Node`].
///
/// This trait provides the common integration surface used by:
///
/// - visitors;
/// - traversal;
/// - structural validation;
/// - diagnostics;
/// - source mapping;
/// - AST tooling.
///
/// Concrete AST nodes should delegate these methods to their embedded `Node`.
///
/// # Example
///
/// ```ignore
/// pub struct Function {
///     node: Node,
///     // function-specific source structure...
/// }
///
/// impl AstNode for Function {
///     fn node(&self) -> &Node {
///         &self.node
///     }
///
///     fn node_mut(&mut self) -> &mut Node {
///         &mut self.node
///     }
/// }
/// ```
pub trait AstNode {
    /// Returns the common node container.
    fn node(&self) -> &Node;

    /// Returns mutable access to the common node container.
    fn node_mut(&mut self) -> &mut Node;

    /// Returns the stable AST node identity.
    #[inline]
    fn id(&self) -> NodeId {
        self.node().id()
    }

    /// Returns the node kind.
    #[inline]
    fn kind(&self) -> &NodeKind {
        self.node().kind()
    }

    /// Returns the source span.
    #[inline]
    fn span(&self) -> &Span {
        self.node().span()
    }

    /// Returns node metadata.
    #[inline]
    fn metadata(&self) -> &NodeMetadata {
        self.node().metadata()
    }

    /// Returns mutable node metadata.
    #[inline]
    fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node_mut().metadata_mut()
    }

    /// Returns whether this node has the supplied kind.
    #[inline]
    fn is_kind(&self, kind: &NodeKind) -> bool {
        self.node().is_kind(kind)
    }

    /// Returns whether this node is an extension node.
    #[inline]
    fn is_extension(&self) -> bool {
        self.node().is_extension()
    }

    /// Returns whether this node is a native core node.
    #[inline]
    fn is_core(&self) -> bool {
        self.node().is_core()
    }

    /// Creates a diagnostic summary without exposing the concrete AST type.
    #[inline]
    fn diagnostic_summary(&self) -> NodeDiagnosticSummary {
        self.node().diagnostic_summary()
    }
}

/// Optional helper trait for AST values that can expose their node without
/// permitting mutation.
///
/// This is useful for parallel read-only compiler phases and tooling.
///
/// It deliberately contains no semantic methods.
pub trait AstNodeRef {
    /// Returns the common AST node.
    fn node_ref(&self) -> &Node;
}

impl<T> AstNodeRef for T
where
    T: AstNode,
{
    #[inline]
    fn node_ref(&self) -> &Node {
        self.node()
    }
}

/// Validates the local invariants of a common AST node.
///
/// This function deliberately performs only node-local validation.
///
/// Cross-node checks such as:
///
/// - parent/child relationships;
/// - duplicate declarations;
/// - unresolved names;
/// - type correctness;
/// - resource compatibility;
/// - quantum connectivity;
/// - backend support;
///
/// belong to later validation/semantic layers.
///
/// The function returns `Ok(())` when local invariants are structurally
/// acceptable.
///
/// The concrete `NodeId`, `NodeKind`, `Span`, and `NodeMetadata` types own their
/// own validation contracts. This function therefore delegates to those
/// contracts instead of duplicating them.
///
/// # Integration
///
/// ```text
/// parser
///   │
///   ▼
/// AST construction
///   │
///   ▼
/// validate_node
///   │
///   ▼
/// structural validation
///   │
///   ▼
/// semantic analysis
/// ```
pub fn validate_node(node: &Node) -> Result<(), NodeValidationError> {
    if node.id.is_default() {
        return Err(NodeValidationError::DefaultNodeId);
    }

    node.kind
        .validate()
        .map_err(NodeValidationError::InvalidNodeKind)?;

    node.span
        .validate()
        .map_err(NodeValidationError::InvalidSpan)?;

    node.metadata
        .validate()
        .map_err(NodeValidationError::InvalidMetadata)?;

    Ok(())
}

/// Errors produced by local node validation.
///
/// This error is intentionally AST-specific and contains no semantic or
/// backend-specific error variants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeValidationError {
    /// The node has not received a real identity.
    DefaultNodeId,

    /// The node kind is malformed.
    InvalidNodeKind(String),

    /// The source span is malformed.
    InvalidSpan(String),

    /// Node metadata is malformed.
    InvalidMetadata(String),
}

impl fmt::Display for NodeValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DefaultNodeId => {
                write!(formatter, "AST node has no valid node ID")
            }
            Self::InvalidNodeKind(message) => {
                write!(formatter, "invalid AST node kind: {message}")
            }
            Self::InvalidSpan(message) => {
                write!(formatter, "invalid AST node span: {message}")
            }
            Self::InvalidMetadata(message) => {
                write!(formatter, "invalid AST node metadata: {message}")
            }
        }
    }
}

impl std::error::Error for NodeValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// This test intentionally checks only the common-node contract.
    ///
    /// Concrete kinds, spans and metadata are tested in their respective
    /// modules so this module does not duplicate their test suites.
    #[test]
    fn node_schema_version_is_stable() {
        assert_eq!(Node::schema_version(), AST_NODE_SCHEMA_VERSION);
    }

    #[test]
    fn diagnostic_summary_contains_identity_kind_and_span() {
        let node = Node::default();
        let summary = node.diagnostic_summary();

        assert_eq!(summary.id, node.id());
        assert_eq!(summary.kind, node.kind_owned());
        assert_eq!(summary.span, node.span_owned());
    }

    #[test]
    fn metadata_can_be_replaced_without_changing_identity() {
        let mut node = Node::default();

        let original_id = node.id();
        let original_kind = node.kind_owned();
        let original_span = node.span_owned();

        let replacement = NodeMetadata::default();

        let _old = node.replace_metadata(replacement);

        assert_eq!(node.id(), original_id);
        assert_eq!(node.kind_owned(), original_kind);
        assert_eq!(node.span_owned(), original_span);
    }

    #[test]
    fn span_replacement_does_not_change_identity() {
        let mut node = Node::default();

        let original_id = node.id();
        let original_kind = node.kind_owned();

        let replacement = Span::default();
        node.set_span(replacement);

        assert_eq!(node.id(), original_id);
        assert_eq!(node.kind_owned(), original_kind);
    }

    #[test]
    fn node_clone_preserves_identity() {
        let node = Node::default();
        let clone = node.clone();

        assert_eq!(node, clone);
        assert_eq!(node.id(), clone.id());
    }
}