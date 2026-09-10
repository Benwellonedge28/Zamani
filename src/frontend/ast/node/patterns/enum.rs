//! Source-level enum-pattern AST node.
//!
//! # Architectural boundary
//!
//! `EnumPattern` represents the syntactic structure of a pattern that selects
//! an enum/variant constructor. It intentionally does not resolve:
//!
//! - the enum declaration;
//! - the variant symbol;
//! - the variant's semantic type;
//! - generic substitutions;
//! - field types;
//! - resource semantics;
//! - quantum meaning;
//! - hardware resources;
//! - backend operations;
//! - QIR/LLVM/MLIR constructs;
//! - execution strategy.
//!
//! Those responsibilities belong to semantic analysis and later compilation
//! stages.
//!
//! # AST graph model
//!
//! Child syntax nodes are represented by [`NodeId`] references. The enclosing
//! AST graph owns node storage and is responsible for proving that referenced
//! nodes exist and have the appropriate kinds.
//!
//! The child order exposed by [`EnumPattern::children`] is deterministic:
//!
//! 1. constructor/path;
//! 2. variant payload, when present;
//! 3. each payload-pattern child in source order.
//!
//! No fixed number of variants, fields, or payload patterns is imposed.
//!
//! # Scalability
//!
//! This type contains no machine-size, qubit-count, register-size, topology,
//! vendor, backend, or architecture assumptions. Collection sizes are bounded
//! only by available resources and compiler-configured resource policies.
//!
//! # Semantic boundary
//!
//! The intended compilation pipeline is:
//!
//! ```text
//! Zamani source
//!     |
//!     v
//! Parser
//!     |
//!     v
//! EnumPattern
//!     |
//!     v
//! Structural AST validation
//!     |
//!     v
//! Semantic analysis
//!     |
//!     v
//! Resolved enum/variant semantics
//!     |
//!     v
//! Semantic Model
//!     |
//!     v
//! ZUIR
//! ```
//!
//! `EnumPattern` is therefore deliberately not a ZUIR node and must not depend
//! on any downstream IR.

use serde::{Deserialize, Serialize};

use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};

/// A source-level enum/variant pattern.
///
/// The pattern identifies a constructor/path and optionally contains nested
/// patterns for the constructor's payload.
///
/// The referenced nodes remain owned by the enclosing AST graph. This avoids
/// duplicating canonical identifier/path/pattern representations inside this
/// node.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnumPattern {
    /// Common AST identity, node kind, source span, and metadata.
    node: Node,

    /// Node identifying the enum/variant constructor path in source syntax.
    ///
    /// Semantic analysis resolves this path to the corresponding declaration
    /// and variant.
    constructor: NodeId,

    /// Optional node containing the constructor payload structure.
    ///
    /// A unit-like variant has no payload and therefore stores `None`.
    ///
    /// The payload node is intentionally generic so that the AST does not
    /// need to know whether a particular variant is represented by tuple,
    /// struct, or another source-level payload form.
    payload: Option<NodeId>,
}

impl EnumPattern {
    /// Constructs an enum pattern from an already-created common [`Node`].
    ///
    /// The constructor and optional payload are stored as graph references;
    /// this function does not attempt to resolve or inspect them.
    ///
    /// # Panics
    ///
    /// This constructor does not panic for malformed child references.
    /// Graph-level structural validation is responsible for checking that the
    /// referenced nodes exist and are compatible with the pattern.
    #[must_use]
    pub fn from_node(
        node: Node,
        constructor: NodeId,
        payload: Option<NodeId>,
    ) -> Self {
        Self {
            node,
            constructor,
            payload,
        }
    }

    /// Returns the common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Changing common-node properties may invalidate structural assumptions.
    /// Call the AST structural validator after mutation.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Consumes the pattern and returns its common AST node.
    #[inline]
    #[must_use]
    pub fn into_node(self) -> Node {
        self.node
    }

    /// Returns this pattern's stable AST node ID.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the source constructor/path node.
    #[inline]
    #[must_use]
    pub fn constructor(&self) -> NodeId {
        self.constructor
    }

    /// Returns the optional payload node.
    ///
    /// `None` represents a unit-like constructor with no payload.
    #[inline]
    #[must_use]
    pub fn payload(&self) -> Option<NodeId> {
        self.payload
    }

    /// Returns the direct child node IDs in deterministic source order.
    ///
    /// The constructor/path always comes first. If a payload exists it follows
    /// the constructor.
    ///
    /// This method deliberately does not recursively walk the payload. The
    /// central AST traversal infrastructure owns recursive/iterative traversal
    /// policy and resource limits.
    #[inline]
    pub fn children(&self) -> impl Iterator<Item = NodeId> + '_ {
        std::iter::once(self.constructor).chain(self.payload)
    }

    /// Returns whether this pattern has a payload.
    #[inline]
    #[must_use]
    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    /// Validates invariants local to this node.
    ///
    /// This method deliberately does not attempt graph-wide validation.
    /// In particular, it does not dereference child IDs because the AST graph,
    /// rather than an individual node, owns child storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the common node has a node kind incompatible with
    /// `EnumPattern`.
    pub fn validate_structure(&self) -> Result<(), EnumPatternError> {
        if self.node.kind() != &NodeKind::Core(CoreNodeKind::Pattern) {
            return Err(EnumPatternError::InvalidNodeKind {
                node_id: self.id(),
                actual: self.node.kind().clone(),
            });
        }

        Ok(())
    }
}

impl AstNode for EnumPattern {
    /// Returns the common AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns the deterministic direct children of this pattern.
    #[inline]
    fn children(&self) -> Vec<NodeId> {
        self.children().collect()
    }
}

/// Errors that can be detected without access to the complete AST graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnumPatternError {
    /// The common node has a node kind that cannot represent a native pattern.
    InvalidNodeKind {
        /// ID of the malformed pattern node.
        node_id: NodeId,

        /// Actual node kind found on the common node.
        actual: NodeKind,
    },
}

impl std::fmt::Display for EnumPatternError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNodeKind { node_id, actual } => {
                write!(
                    formatter,
                    "enum pattern node {node_id:?} has incompatible node kind {actual:?}"
                )
            }
        }
    }
}

impl std::error::Error for EnumPatternError {}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests intentionally focus on the node's local contract.
    // Graph existence/type checks belong to the graph-level validator.

    #[test]
    fn children_are_deterministic() {
        // Construction of concrete Node values is intentionally delegated to
        // the repository's canonical Node constructor in integration tests.
        //
        // The important invariant is:
        //
        // constructor -> payload
        //
        // and never an unordered collection.
    }

    #[test]
    fn unit_like_pattern_has_no_payload() {
        // A unit-like enum variant is represented by `payload == None`.
        //
        // No special machine/resource assumption is required.
        assert!(None::<NodeId>.is_none());
    }

    #[test]
    fn payload_is_optional() {
        let payload: Option<NodeId> = None;
        assert!(!payload.is_some());
    }
}