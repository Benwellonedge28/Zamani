//! # Zamani Native AST — Indexing
//!
//! Production-ready source-level representation of an indexing expression.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! native Zamani AST
//!     │
//!     ├── IndexExpression  ← this module
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── HDL IR
//!     └── future domain IRs
//!     │
//!     ▼
//! target lowering
//! ```
//!
//! ## Responsibility
//!
//! This module owns the **source-level structure of indexing**.
//!
//! For source such as:
//!
//! ```text
//! value[index]
//! ```
//!
//! it represents:
//!
//! ```text
//! IndexExpression
//! ├── base   : NodeId
//! └── index  : NodeId
//! ```
//!
//! Both children are references to nodes owned by the canonical AST graph.
//!
//! This module does NOT determine what `value` means or what indexing means
//! semantically.
//!
//! The same syntax may eventually represent access to:
//!
//! - an array;
//! - a slice;
//! - a collection;
//! - a tensor;
//! - a classical register;
//! - a symbolic resource set;
//! - a quantum resource collection;
//! - a distributed resource collection;
//! - a future computational resource;
//! - a user-defined indexable abstraction.
//!
//! Semantic analysis determines the meaning.
//!
//! ## POCO-REAF
//!
//! Indexing introduces no assumptions about:
//!
//! - number of elements;
//! - number of qubits;
//! - register width;
//! - machine size;
//! - hardware topology;
//! - physical resource count;
//! - processor architecture;
//! - vendor;
//! - backend;
//! - quantum technology;
//! - instruction set.
//!
//! There is therefore no:
//!
//! ```text
//! MAX_INDEX
//! MAX_REGISTER_SIZE
//! MAX_QUBITS
//! MAX_ELEMENTS
//! ```
//!
//! in this module.
//!
//! A source expression such as:
//!
//! ```text
//! resources[i]
//! ```
//!
//! remains valid whether `resources` eventually denotes one resource or an
//! arbitrarily large resource collection, subject only to the semantic and
//! target capabilities of the eventual compilation environment.
//!
//! ## Important distinction
//!
//! This is an AST construct, NOT an IR operation.
//!
//! It must not contain:
//!
//! - resolved types;
//! - resolved symbols;
//! - resolved fields;
//! - resource IDs;
//! - physical qubit IDs;
//! - logical qubit assignments;
//! - memory addresses;
//! - SSA values;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - routing information;
//! - scheduling information;
//! - calibration information;
//! - QEC information;
//! - backend information.
//!
//! Those belong to later compiler phases.
//!
//! ## Existing repository integration
//!
//! The repository already has `ExpressionKind::Index` and maps it to the
//! canonical `CoreNodeKind::IndexExpression`.
//!
//! Therefore this file MUST NOT introduce another top-level `ExpressionKind`
//! or another `NodeKind` taxonomy.
//!
//! The aggregate expression layer remains authoritative for expression
//! classification.
//!
//! This module owns the reusable structural payload and local invariants of
//! indexing.
//!
//! ## Grammar contract
//!
//! The repository grammar defines indexing conceptually as:
//!
//! ```text
//! indexExpr: expression '[' expression ']';
//! ```
//!
//! Consequently:
//!
//! ```text
//! base  = expression before '['
//! index = expression inside '[' and ']'
//! ```
//!
//! No integer-only restriction is imposed here.
//!
//! For example, all of the following can be structurally represented:
//!
//! ```text
//! a[0]
//! a[i]
//! a[i + 1]
//! a[f(x)]
//! a[dimension]
//! register[index]
//! resource[symbolic_index]
//! ```
//!
//! Whether an index is legal is a semantic question.
//!
//! ## Child representation
//!
//! Children use [`NodeId`] rather than recursively embedding expressions.
//!
//! This is important for very large or deeply nested ASTs because the indexing
//! node itself never recursively walks its children.
//!
//! For example:
//!
//! ```text
//! a[b[c[d[e]]]]
//! ```
//!
//! becomes an AST graph containing independently allocated nodes connected by
//! `NodeId` references.
//!
//! Traversal depth is therefore controlled by the AST traversal subsystem,
//! rather than by recursive construction in this module.
//!
//! ## Validation boundary
//!
//! Local validation checks:
//!
//! - the base reference is assigned;
//! - the index reference is assigned;
//! - the base and index are not the indexing node itself;
//! - base and index are distinct;
//! - the common node has the canonical index-expression kind.
//!
//! Local validation does NOT check:
//!
//! - whether the child nodes actually exist in the AST graph;
//! - whether the base is indexable;
//! - whether the index has an integer type;
//! - whether the index is within bounds;
//! - whether dynamic indexing is supported by a target;
//! - whether a quantum resource can be dynamically indexed;
//! - whether a physical resource exists.
//!
//! Those checks require later compiler context.
//!
//! ## Scalability
//!
//! The representation contains exactly two `NodeId` references.
//!
//! It does not contain a fixed-size representation of the indexed object.
//!
//! Arbitrary collection size is therefore represented by the referenced
//! expression/resource rather than encoded in this node.
//!
//! The node itself is O(1) in storage and O(1) in local structural validation.
//!
//! ## Determinism
//!
//! Child order is always:
//!
//! 1. base;
//! 2. index.
//!
//! No unordered collection is used.
//!
//! ## Serialization
//!
//! The structure derives Serde serialization.
//!
//! The enclosing AST serialization subsystem remains responsible for the
//! overall AST schema version and serialized representation version.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - performs no allocation based on untrusted index values;
//! - never dereferences a `NodeId`;
//! - never performs unchecked indexing;
//! - never recursively traverses the AST;
//! - never executes an expression;
//! - contains no global mutable state.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes this node and resolves:
//!
//! ```text
//! base expression
//!       │
//!       ▼
//! resolved type/resource
//!       │
//!       ▼
//! indexing semantics
//!       │
//!       ▼
//! index expression type
//!       │
//!       ▼
//! resulting semantic value/place/reference
//! ```
//!
//! This is where the compiler may determine whether indexing means:
//!
//! - array element access;
//! - slice access;
//! - tensor access;
//! - resource selection;
//! - user-defined indexing;
//! - another language-defined operation.
//!
//! ## Quantum integration
//!
//! Quantum code may eventually use the same node for constructs such as:
//!
//! ```text
//! qubits[i]
//! register[index]
//! resources[symbolic_index]
//! ```
//!
//! The AST does not know whether the indexed value represents a qubit,
//! logical resource, physical resource, register, tensor, or something else.
//!
//! Physical mapping remains downstream.
//!
//! ## ZUIR integration
//!
//! This module does not import ZUIR.
//!
//! Conceptually:
//!
//! ```text
//! IndexExpression
//!       │
//!       ▼
//! semantic resolution
//!       │
//!       ▼
//! semantic access/index operation
//!       │
//!       ▼
//! ZUIR
//! ```
//!
//! The exact ZUIR representation depends on the resolved semantic meaning.
//!
//! ## Visitor integration
//!
//! Visitors must observe children in source order:
//!
//! ```text
//! base → index
//! ```
//!
//! This module provides allocation-free child enumeration.
//!
//! ## Thread safety
//!
//! The type has no mutable global state.
//!
//! Thread-safety follows from its contained fields.
//!
//! Immutable indexing nodes may therefore be shared between compiler phases
//! whenever the underlying AST graph is safely shared.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # File contract
//!
//! **Inputs**
//!
//! - `Node`;
//! - `NodeId`;
//! - canonical `NodeKind` / `CoreNodeKind`;
//! - source `Span`;
//! - source metadata.
//!
//! **Outputs**
//!
//! - `IndexExpression`;
//! - deterministic child enumeration;
//! - local structural validation.
//!
//! **Owned here**
//!
//! - indexing-specific structural representation;
//! - base child reference;
//! - index child reference;
//! - local indexing invariants.
//!
//! **Not owned here**
//!
//! - AST graph storage;
//! - name resolution;
//! - type checking;
//! - resource resolution;
//! - quantum compilation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - calibration;
//! - backend mapping;
//! - execution.
//!
//! **Forbidden dependencies**
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - hardware backends;
//! - quantum backends;
//! - schedulers;
//! - routers;
//! - runtimes.
//!
//! **No-re-edit guarantee**
//!
//! Parser, semantic-analysis, ZUIR, quantum, or backend implementations should
//! consume this public contract without requiring this file to be modified.
//!
//! A new indexable type or computational technology therefore does not require
//! changes here.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version for the indexing-node contract.
///
/// This is deliberately separate from the global AST schema version and from
/// the Zamani language version.
pub const INDEX_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Result type used by indexing-node construction and validation.
pub type IndexExpressionResult<T> = Result<T, IndexExpressionError>;

/// Structural errors produced by an indexing expression.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IndexExpressionError {
    /// The common node does not have the canonical index-expression kind.
    InvalidNodeKind {
        /// Actual node kind found on the common node.
        actual: NodeKind,
    },

    /// The base expression is not assigned.
    InvalidBaseReference {
        /// Invalid base node identity.
        id: NodeId,
    },

    /// The index expression is not assigned.
    InvalidIndexReference {
        /// Invalid index node identity.
        id: NodeId,
    },

    /// The base and index refer to the same node.
    DuplicateChild {
        /// Duplicated child identity.
        id: NodeId,
    },

    /// The indexing node references itself as a child.
    SelfReference {
        /// The indexing node identity.
        id: NodeId,
    },
}

impl fmt::Display for IndexExpressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "index expression has invalid AST node kind: {actual}"
                )
            }

            Self::InvalidBaseReference { id } => {
                write!(
                    formatter,
                    "index expression has invalid base node reference: {id:?}"
                )
            }

            Self::InvalidIndexReference { id } => {
                write!(
                    formatter,
                    "index expression has invalid index node reference: {id:?}"
                )
            }

            Self::DuplicateChild { id } => {
                write!(
                    formatter,
                    "index expression uses the same node for base and index: {id:?}"
                )
            }

            Self::SelfReference { id } => {
                write!(
                    formatter,
                    "index expression cannot reference itself as a child: {id:?}"
                )
            }
        }
    }
}

impl std::error::Error for IndexExpressionError {}

/// Canonical source-level indexing expression.
///
/// The node contains exactly the structural information required to represent:
///
/// ```text
/// base[index]
/// ```
///
/// The actual child nodes are owned by the AST graph and referenced by
/// [`NodeId`].
///
/// # Invariants
///
/// A valid `IndexExpression` satisfies:
///
/// 1. `node.kind()` is `CoreNodeKind::IndexExpression`.
/// 2. `base` is not `NodeId::INVALID`.
/// 3. `index` is not `NodeId::INVALID`.
/// 4. `base != index`.
/// 5. Neither child is `node.id()`.
///
/// The existence of the referenced children is validated by the enclosing AST
/// graph validator, not by this local node.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IndexExpression {
    node: Node,
    base: NodeId,
    index: NodeId,
}

impl IndexExpression {
    /// Creates an indexing expression without performing semantic analysis.
    ///
    /// This constructor establishes the source-level structure only.
    ///
    /// Prefer [`Self::try_new`] when input may originate from an untrusted or
    /// partially recovered parser.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        base: NodeId,
        index: NodeId,
    ) -> Self {
        Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::IndexExpression),
                span,
                NodeMetadata::default(),
            ),
            base,
            index,
        }
    }

    /// Creates an indexing expression with explicit metadata.
    ///
    /// This is the preferred constructor when the parser or AST builder has
    /// already collected source-level metadata.
    #[must_use]
    pub fn with_metadata(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        base: NodeId,
        index: NodeId,
    ) -> Self {
        Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::IndexExpression),
                span,
                metadata,
            ),
            base,
            index,
        }
    }

    /// Checked constructor.
    ///
    /// This validates only local structural invariants. It does not inspect the
    /// global AST graph or perform semantic analysis.
    pub fn try_new(
        id: NodeId,
        span: Span,
        base: NodeId,
        index: NodeId,
    ) -> IndexExpressionResult<Self> {
        let expression = Self::new(id, span, base, index);

        expression.validate_structure()?;

        Ok(expression)
    }

    /// Checked constructor with explicit metadata.
    pub fn try_with_metadata(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        base: NodeId,
        index: NodeId,
    ) -> IndexExpressionResult<Self> {
        let expression =
            Self::with_metadata(id, span, metadata, base, index);

        expression.validate_structure()?;

        Ok(expression)
    }

    /// Returns the common AST node.
    #[must_use]
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Only source-level common metadata can be modified through the `Node`
    /// API. Semantic state cannot be inserted here.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST identity.
    #[must_use]
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the canonical node kind.
    #[must_use]
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the source span.
    #[must_use]
    #[inline]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns source-level metadata.
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable source-level metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the expression being indexed.
    ///
    /// For:
    ///
    /// ```text
    /// value[index]
    /// ```
    ///
    /// this returns the `NodeId` of `value`.
    #[must_use]
    #[inline]
    pub const fn base(&self) -> NodeId {
        self.base
    }

    /// Returns the index expression.
    ///
    /// For:
    ///
    /// ```text
    /// value[index]
    /// ```
    ///
    /// this returns the `NodeId` of `index`.
    #[must_use]
    #[inline]
    pub const fn index(&self) -> NodeId {
        self.index
    }

    /// Replaces the base node reference.
    ///
    /// This is a structural AST transformation. The caller remains responsible
    /// for validating the resulting graph.
    #[inline]
    pub fn replace_base(&mut self, base: NodeId) -> NodeId {
        core::mem::replace(&mut self.base, base)
    }

    /// Replaces the index node reference.
    ///
    /// This is a structural AST transformation. The caller remains responsible
    /// for validating the resulting graph.
    #[inline]
    pub fn replace_index(&mut self, index: NodeId) -> NodeId {
        core::mem::replace(&mut self.index, index)
    }

    /// Returns the two direct children in source order.
    ///
    /// The returned array is fixed at two elements because indexing has exactly
    /// two structural operands by language definition:
    ///
    /// ```text
    /// base[index]
    /// ```
    ///
    /// This is not a computational-resource limit.
    #[must_use]
    #[inline]
    pub const fn child_node_ids(&self) -> [NodeId; 2] {
        [self.base, self.index]
    }

    /// Returns the direct children as an iterator.
    ///
    /// This is allocation-free and non-recursive.
    #[must_use]
    #[inline]
    pub fn children(&self) -> core::array::IntoIter<NodeId, 2> {
        self.child_node_ids().into_iter()
    }

    /// Performs local structural validation.
    ///
    /// This deliberately does not inspect the AST graph because this node does
    /// not own the graph.
    pub fn validate_structure(&self) -> IndexExpressionResult<()> {
        if !matches!(
            self.node.kind(),
            NodeKind::Core(CoreNodeKind::IndexExpression)
        ) {
            return Err(IndexExpressionError::InvalidNodeKind {
                actual: self.node.kind().clone(),
            });
        }

        if self.base == NodeId::INVALID {
            return Err(IndexExpressionError::InvalidBaseReference {
                id: self.base,
            });
        }

        if self.index == NodeId::INVALID {
            return Err(IndexExpressionError::InvalidIndexReference {
                id: self.index,
            });
        }

        if self.base == self.index {
            return Err(IndexExpressionError::DuplicateChild {
                id: self.base,
            });
        }

        if self.base == self.id() {
            return Err(IndexExpressionError::SelfReference {
                id: self.id(),
            });
        }

        if self.index == self.id() {
            return Err(IndexExpressionError::SelfReference {
                id: self.id(),
            });
        }

        Ok(())
    }

    /// Returns the local schema version.
    #[must_use]
    #[inline]
    pub const fn schema_version() -> u16 {
        INDEX_EXPRESSION_SCHEMA_VERSION
    }
}

impl super::super::node::AstNode for IndexExpression {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// A lightweight source-order child iterator for indexing.
///
/// This type is intentionally independent of the AST graph.
///
/// It only exposes:
///
/// ```text
/// base → index
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexExpressionChildren {
    children: [NodeId; 2],
    position: usize,
}

impl IndexExpressionChildren {
    /// Creates a child iterator for an indexing expression.
    #[must_use]
    #[inline]
    pub const fn new(expression: &IndexExpression) -> Self {
        Self {
            children: expression.child_node_ids(),
            position: 0,
        }
    }
}

impl Iterator for IndexExpressionChildren {
    type Item = NodeId;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let child = self.children.get(self.position).copied();

        if child.is_some() {
            self.position += 1;
        }

        child
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.children.len().saturating_sub(self.position);

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for IndexExpressionChildren {}

impl std::iter::FusedIterator for IndexExpressionChildren {}

/// Converts an indexing expression into its direct-child iterator.
impl IntoIterator for &IndexExpression {
    type Item = NodeId;
    type IntoIter = IndexExpressionChildren;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IndexExpressionChildren::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test NodeId must be non-zero")
    }

    fn span() -> Span {
        Span::default()
    }

    #[test]
    fn creates_valid_index_expression() {
        let expression =
            IndexExpression::try_new(
                node_id(1),
                span(),
                node_id(2),
                node_id(3),
            )
            .expect("valid indexing expression");

        assert_eq!(expression.id(), node_id(1));
        assert_eq!(expression.base(), node_id(2));
        assert_eq!(expression.index(), node_id(3));

        assert_eq!(
            expression.kind(),
            &NodeKind::core(CoreNodeKind::IndexExpression)
        );
    }

    #[test]
    fn rejects_invalid_base_reference() {
        let result =
            IndexExpression::try_new(
                node_id(1),
                span(),
                NodeId::INVALID,
                node_id(3),
            );

        assert_eq!(
            result,
            Err(IndexExpressionError::InvalidBaseReference {
                id: NodeId::INVALID
            })
        );
    }

    #[test]
    fn rejects_invalid_index_reference() {
        let result =
            IndexExpression::try_new(
                node_id(1),
                span(),
                node_id(2),
                NodeId::INVALID,
            );

        assert_eq!(
            result,
            Err(IndexExpressionError::InvalidIndexReference {
                id: NodeId::INVALID
            })
        );
    }

    #[test]
    fn rejects_duplicate_children() {
        let result =
            IndexExpression::try_new(
                node_id(1),
                span(),
                node_id(2),
                node_id(2),
            );

        assert_eq!(
            result,
            Err(IndexExpressionError::DuplicateChild {
                id: node_id(2)
            })
        );
    }

    #[test]
    fn rejects_self_reference_as_base() {
        let result =
            IndexExpression::try_new(
                node_id(1),
                span(),
                node_id(1),
                node_id(2),
            );

        assert_eq!(
            result,
            Err(IndexExpressionError::SelfReference {
                id: node_id(1)
            })
        );
    }

    #[test]
    fn rejects_self_reference_as_index() {
        let result =
            IndexExpression::try_new(
                node_id(1),
                span(),
                node_id(2),
                node_id(1),
            );

        assert_eq!(
            result,
            Err(IndexExpressionError::SelfReference {
                id: node_id(1)
            })
        );
    }

    #[test]
    fn child_order_is_source_order() {
        let expression =
            IndexExpression::new(
                node_id(1),
                span(),
                node_id(2),
                node_id(3),
            );

        let children: Vec<NodeId> =
            expression.children().collect();

        assert_eq!(
            children,
            vec![node_id(2), node_id(3)]
        );
    }

    #[test]
    fn into_iterator_is_source_order() {
        let expression =
            IndexExpression::new(
                node_id(1),
                span(),
                node_id(2),
                node_id(3),
            );

        let children: Vec<NodeId> =
            (&expression).into_iter().collect();

        assert_eq!(
            children,
            vec![node_id(2), node_id(3)]
        );
    }

    #[test]
    fn validation_is_idempotent() {
        let expression =
            IndexExpression::new(
                node_id(1),
                span(),
                node_id(2),
                node_id(3),
            );

        assert!(expression.validate_structure().is_ok());
        assert!(expression.validate_structure().is_ok());
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            IndexExpression::schema_version(),
            INDEX_EXPRESSION_SCHEMA_VERSION
        );
    }

    #[test]
    fn serde_round_trip_preserves_structure() {
        let expression =
            IndexExpression::new(
                node_id(1),
                span(),
                node_id(2),
                node_id(3),
            );

        let encoded =
            serde_json::to_string(&expression)
                .expect("serialize indexing expression");

        let decoded: IndexExpression =
            serde_json::from_str(&encoded)
                .expect("deserialize indexing expression");

        assert_eq!(decoded, expression);
    }
}