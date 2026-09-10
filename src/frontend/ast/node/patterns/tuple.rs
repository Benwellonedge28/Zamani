//! # Zamani Frontend AST — Tuple Pattern
//!
//! Source-level tuple-pattern representation for the native Zamani AST.
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
//! Native Zamani AST
//!     │
//!     ├── Pattern
//!     │    └── TuplePattern  ← this module
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
//!     ▼
//! domain / target lowering
//! ```
//!
//! ## Purpose
//!
//! `TuplePattern` represents the **source-level structure** of a tuple
//! pattern. It owns:
//!
//! - the common [`Node`] metadata;
//! - the ordered [`NodeId`] references to child patterns.
//!
//! It does not own the child nodes themselves. The enclosing AST graph owns
//! node storage and is responsible for resolving the referenced [`NodeId`]s.
//!
//! This design is intentional. It prevents:
//!
//! - duplicated AST nodes;
//! - recursive Rust object ownership;
//! - unnecessary cloning;
//! - coupling between sibling AST nodes;
//! - machine-size assumptions;
//! - fixed tuple arity limits.
//!
//! ## Example
//!
//! A source pattern such as:
//!
//! ```text
//! (first, second, _)
//! ```
//!
//! is represented conceptually as:
//!
//! ```text
//! TuplePattern
//! ├── NodeId(first)
//! ├── NodeId(second)
//! └── NodeId(wildcard)
//! ```
//!
//! The referenced nodes remain independently owned by the AST graph.
//!
//! ## Grammar integration
//!
//! The native Zamani grammar currently describes tuple patterns conceptually
//! as:
//!
//! ```text
//! tuplePattern: '(' pattern (',' pattern)* ')';
//! ```
//!
//! Therefore this node preserves source order and requires at least one child.
//!
//! Syntax disambiguation remains the parser's responsibility. This type does
//! not decide whether a parenthesized source construct is an expression,
//! tuple expression, or tuple pattern.
//!
//! ## Semantic boundary
//!
//! This type deliberately does **not** perform:
//!
//! - type compatibility checking;
//! - tuple arity checking against a scrutinee;
//! - destructuring semantics;
//! - binding analysis;
//! - exhaustiveness analysis;
//! - reachability analysis;
//! - constant evaluation;
//! - quantum semantics;
//! - hardware mapping.
//!
//! Those responsibilities belong to semantic analysis and later compiler
//! layers.
//!
//! ## Quantum / POCO-REAF boundary
//!
//! A tuple pattern is domain-neutral. It can therefore participate in source
//! programs involving:
//!
//! - classical data;
//! - quantum/classical hybrid data;
//! - generic resources;
//! - distributed computation;
//! - accelerator computation;
//! - future computational domains.
//!
//! No qubit count, machine size, topology, backend, vendor, instruction set,
//! or hardware representation is encoded here.
//!
//! Consequently, this type does not prevent a Zamani program from following
//! the POCO-REAF model:
//!
//! ```text
//! Program Once
//!     │
//!     ▼
//! source-level AST
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! resource / capability discovery
//!     │
//!     ▼
//! target realization
//! ```
//!
//! ## Scalability
//!
//! Tuple arity is represented by a dynamically sized `Vec<NodeId>`.
//!
//! There is deliberately no constant such as:
//!
//! ```text
//! MAX_TUPLE_ELEMENTS
//! ```
//!
//! The practical limit is determined by available resources and by the
//! compiler's externally configurable resource-safety policy.
//!
//! This module does not impose a language-semantic machine-size limit.
//!
//! ## Dependency contract
//!
//! This module may depend on:
//!
//! - the canonical [`Node`];
//! - [`AstNode`];
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`CoreNodeKind`];
//! - [`NodeMetadata`];
//! - [`Span`];
//! - Rust's standard library;
//! - `serde` for AST serialization.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - hardware;
//! - backend providers;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - calibration;
//! - runtime execution.
//!
//! ## Ownership
//!
//! This file owns the representation of a tuple-pattern node.
//!
//! It does not own:
//!
//! - the AST graph;
//! - referenced child nodes;
//! - symbol tables;
//! - type information;
//! - semantic bindings;
//! - resource allocation;
//! - execution state.
//!
//! ## Determinism
//!
//! Child order is preserved exactly.
//!
//! The sequence:
//!
//! ```text
//! [a, b, c]
//! ```
//!
//! is never normalized, sorted, deduplicated, or otherwise reordered by this
//! type.
//!
//! This is essential because source ordering is part of the syntactic
//! structure.
//!
//! ## Safety
//!
//! This implementation uses safe Rust only and contains no `unsafe` code.
//!
//! Target Rust: Rust 1.97 / Rust 1.97.1.

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// AST schema version for [`TuplePattern`].
///
/// This is independent from the Zamani language version, compiler version,
/// serialization format version, and extension versions.
pub const TUPLE_PATTERN_SCHEMA_VERSION: u16 = 1;

/// The native node kind used by tuple patterns.
///
/// `CoreNodeKind` currently provides the generic `Pattern` classification
/// rather than a separate `TuplePattern` variant. Keeping the classification
/// generic avoids coupling this file to a closed enumeration of every possible
/// pattern form.
pub const TUPLE_PATTERN_NODE_KIND: NodeKind =
    NodeKind::Core(CoreNodeKind::Pattern);

/// Errors produced while constructing a tuple pattern.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TuplePatternError {
    /// A tuple pattern must contain at least one pattern according to the
    /// native Zamani tuple-pattern grammar.
    Empty,

    /// The supplied common node does not have the node classification required
    /// by this implementation.
    InvalidNodeKind {
        /// The actual node kind encountered.
        actual: NodeKind,
    },
}

impl core::fmt::Display for TuplePatternError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty => {
                formatter.write_str("tuple pattern must contain at least one child pattern")
            }
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "invalid node kind for tuple pattern: {actual}"
                )
            }
        }
    }
}

impl std::error::Error for TuplePatternError {}

/// Source-level tuple pattern.
///
/// Child patterns are represented by stable [`NodeId`] references. The AST
/// graph, not this node, owns the actual child nodes.
///
/// # Invariants
///
/// A valid `TuplePattern` satisfies:
///
/// 1. `node.kind()` is [`CoreNodeKind::Pattern`].
/// 2. `children` is non-empty.
/// 3. Child order exactly matches source order.
/// 4. Every child identifier is treated as an opaque AST reference.
/// 5. No semantic information is stored in the node.
/// 6. No hardware or target information is stored in the node.
///
/// The existence and classification of the referenced child nodes are
/// validated by graph-level structural validation, not by this type alone.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TuplePattern {
    /// Common AST identity, source span, node kind and metadata.
    node: Node,

    /// Ordered child-pattern references.
    ///
    /// The vector has no fixed capacity or semantic maximum.
    children: Vec<NodeId>,
}

impl TuplePattern {
    /// Returns the AST schema version for this node.
    #[must_use]
    #[inline]
    pub const fn schema_version() -> u16 {
        TUPLE_PATTERN_SCHEMA_VERSION
    }

    /// Constructs a tuple pattern from its complete AST components.
    ///
    /// The child nodes are not copied into this structure. Only their stable
    /// identifiers are retained.
    ///
    /// # Errors
    ///
    /// Returns [`TuplePatternError::Empty`] when no child pattern is supplied.
    ///
    /// # Complexity
    ///
    /// Construction is `O(n)` because the supplied child identifiers must be
    /// retained in an owned vector.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        children: Vec<NodeId>,
    ) -> Result<Self, TuplePatternError> {
        if children.is_empty() {
            return Err(TuplePatternError::Empty);
        }

        Ok(Self {
            node: Node::new(
                id,
                TUPLE_PATTERN_NODE_KIND.clone(),
                span,
                metadata,
            ),
            children,
        })
    }

    /// Constructs a tuple pattern with default metadata.
    ///
    /// This is the normal convenience constructor for parser code when no
    /// additional metadata is attached to the syntax node.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        children: Vec<NodeId>,
    ) -> Result<Self, TuplePatternError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            children,
        )
    }

    /// Constructs a tuple pattern from an already-created common [`Node`].
    ///
    /// This constructor is useful for parser/builders that construct common
    /// node metadata independently from the pattern-specific payload.
    ///
    /// The node must have [`CoreNodeKind::Pattern`] classification.
    ///
    /// # Errors
    ///
    /// Returns [`TuplePatternError::InvalidNodeKind`] when the node kind does
    /// not identify a native pattern.
    ///
    /// Returns [`TuplePatternError::Empty`] when no child patterns are
    /// supplied.
    #[must_use]
    pub fn from_node(
        node: Node,
        children: Vec<NodeId>,
    ) -> Result<Self, TuplePatternError> {
        if node.kind() != &TUPLE_PATTERN_NODE_KIND {
            return Err(TuplePatternError::InvalidNodeKind {
                actual: node.kind_owned(),
            });
        }

        if children.is_empty() {
            return Err(TuplePatternError::Empty);
        }

        Ok(Self {
            node,
            children,
        })
    }

    /// Returns the common AST node.
    #[must_use]
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Only common source metadata can be changed through the [`Node`] API.
    /// Child relationships remain controlled by the tuple-pattern API.
    #[must_use]
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node identifier.
    #[must_use]
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the source span.
    #[must_use]
    #[inline]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns the source-level node kind.
    #[must_use]
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns node metadata.
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable node metadata.
    #[must_use]
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns an iterator over child-pattern identifiers.
    ///
    /// The iterator preserves source order.
    ///
    /// The returned iterator borrows the tuple pattern and therefore does not
    /// allocate.
    #[must_use]
    #[inline]
    pub fn children(&self) -> impl ExactSizeIterator<Item = NodeId> + DoubleEndedIterator + '_ {
        self.children.iter().copied()
    }

    /// Returns the canonical AST child-node iterator.
    ///
    /// This name aligns with the AST graph/traversal APIs used by other
    /// repository nodes.
    #[must_use]
    #[inline]
    pub fn child_node_ids(
        &self,
    ) -> impl ExactSizeIterator<Item = NodeId> + DoubleEndedIterator + '_ {
        self.children()
    }

    /// Returns the number of direct child patterns.
    ///
    /// This is the syntactic tuple arity. It must not be confused with the
    /// semantic type arity of a value.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Returns whether the tuple has no children.
    ///
    /// A valid `TuplePattern` can never be empty because construction rejects
    /// empty tuples. This method exists for generic collection-style APIs and
    /// therefore always returns `false` for a valid instance.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns the child identifier at `index`.
    ///
    /// This operation does not resolve the identifier against the AST graph.
    #[must_use]
    #[inline]
    pub fn child(&self, index: usize) -> Option<NodeId> {
        self.children.get(index).copied()
    }

    /// Returns the first child-pattern identifier.
    #[must_use]
    #[inline]
    pub fn first(&self) -> Option<NodeId> {
        self.children.first().copied()
    }

    /// Returns the last child-pattern identifier.
    #[must_use]
    #[inline]
    pub fn last(&self) -> Option<NodeId> {
        self.children.last().copied()
    }

    /// Appends one child-pattern reference.
    ///
    /// This does not impose a tuple-size limit. Compiler-level resource
    /// policies may impose configurable limits elsewhere.
    ///
    /// The operation is deterministic and preserves source-order insertion.
    #[inline]
    pub fn push_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    /// Appends multiple child-pattern references in the supplied order.
    ///
    /// The operation does not sort, deduplicate, or otherwise normalize the
    /// identifiers.
    #[inline]
    pub fn extend_children<I>(&mut self, children: I)
    where
        I: IntoIterator<Item = NodeId>,
    {
        self.children.extend(children);
    }

    /// Removes and returns the last child-pattern reference.
    ///
    /// This method is intended for AST builder/transform infrastructure.
    #[must_use]
    #[inline]
    pub fn pop_child(&mut self) -> Option<NodeId> {
        self.children.pop()
    }

    /// Returns an immutable slice of child identifiers.
    ///
    /// This is useful for zero-copy validation and serialization adapters.
    #[must_use]
    #[inline]
    pub fn child_slice(&self) -> &[NodeId] {
        &self.children
    }

    /// Returns mutable access to the child identifier slice.
    ///
    /// Structural validation must be rerun after mutation.
    ///
    /// This method deliberately exposes identifiers rather than concrete AST
    /// nodes so the ownership model remains graph-based.
    #[must_use]
    #[inline]
    pub fn child_slice_mut(&mut self) -> &mut [NodeId] {
        &mut self.children
    }

    /// Replaces all child references while preserving their supplied order.
    ///
    /// # Errors
    ///
    /// Returns [`TuplePatternError::Empty`] if `children` is empty.
    pub fn replace_children(
        &mut self,
        children: Vec<NodeId>,
    ) -> Result<Vec<NodeId>, TuplePatternError> {
        if children.is_empty() {
            return Err(TuplePatternError::Empty);
        }

        Ok(std::mem::replace(
            &mut self.children,
            children,
        ))
    }

    /// Validates the local structural invariants of this tuple pattern.
    ///
    /// This function intentionally performs only checks that can be completed
    /// without access to the surrounding AST graph.
    ///
    /// It does **not** verify that the child identifiers exist. Graph-level
    /// validation owns that responsibility.
    pub fn validate_structure(&self) -> Result<(), TuplePatternError> {
        if self.node.kind() != &TUPLE_PATTERN_NODE_KIND {
            return Err(TuplePatternError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        if self.children.is_empty() {
            return Err(TuplePatternError::Empty);
        }

        Ok(())
    }

    /// Returns `true` when the local structural invariants are satisfied.
    ///
    /// This is equivalent to checking [`Self::validate_structure`] without
    /// exposing the error.
    #[must_use]
    #[inline]
    pub fn is_structurally_valid(&self) -> bool {
        self.node.kind() == &TUPLE_PATTERN_NODE_KIND
            && !self.children.is_empty()
    }
}

impl AstNode for TuplePattern {
    /// Returns the common AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl TuplePattern {
    /// Returns the complete ordered child sequence through the canonical
    /// `AstNode` integration surface.
    ///
    /// This convenience method is intentionally separate from `children()` so
    /// generic AST traversal code can use the same conceptual API for every
    /// node with child references.
    #[must_use]
    #[inline]
    pub fn children_vec(&self) -> Vec<NodeId> {
        self.children.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn span() -> Span {
        Span::default()
    }

    #[test]
    fn constructs_non_empty_tuple_pattern() {
        let pattern = TuplePattern::without_metadata(
            node_id(1),
            span(),
            vec![node_id(2), node_id(3), node_id(4)],
        )
        .expect("non-empty tuple pattern should construct");

        assert_eq!(pattern.len(), 3);
        assert_eq!(
            pattern.children().collect::<Vec<_>>(),
            vec![node_id(2), node_id(3), node_id(4)]
        );
        assert_eq!(
            pattern.kind(),
            &TUPLE_PATTERN_NODE_KIND
        );
        assert!(pattern.is_structurally_valid());
    }

    #[test]
    fn rejects_empty_tuple_pattern() {
        let result = TuplePattern::without_metadata(
            node_id(1),
            span(),
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(TuplePatternError::Empty)
        );
    }

    #[test]
    fn preserves_source_order() {
        let pattern = TuplePattern::without_metadata(
            node_id(1),
            span(),
            vec![
                node_id(10),
                node_id(20),
                node_id(30),
                node_id(40),
            ],
        )
        .expect("valid tuple pattern");

        assert_eq!(
            pattern.child_node_ids().collect::<Vec<_>>(),
            vec![
                node_id(10),
                node_id(20),
                node_id(30),
                node_id(40),
            ]
        );
    }

    #[test]
    fn supports_single_element_tuple_pattern() {
        let pattern = TuplePattern::without_metadata(
            node_id(1),
            span(),
            vec![node_id(2)],
        )
        .expect("single-element tuple pattern should construct");

        assert_eq!(pattern.len(), 1);
        assert_eq!(pattern.first(), Some(node_id(2)));
        assert_eq!(pattern.last(), Some(node_id(2)));
    }

    #[test]
    fn child_lookup_is_bounds_safe() {
        let pattern = TuplePattern::without_metadata(
            node_id(1),
            span(),
            vec![node_id(2)],
        )
        .expect("valid tuple pattern");

        assert_eq!(pattern.child(0), Some(node_id(2)));
        assert_eq!(pattern.child(1), None);
    }

    #[test]
    fn push_child_preserves_order() {
        let mut pattern = TuplePattern::without_metadata(
            node_id(1),
            span(),
            vec![node_id(2)],
        )
        .expect("valid tuple pattern");

        pattern.push_child(node_id(3));
        pattern.push_child(node_id(4));

        assert_eq!(
            pattern.children().collect::<Vec<_>>(),
            vec![
                node_id(2),
                node_id(3),
                node_id(4),
            ]
        );
    }

    #[test]
    fn extend_children_preserves_order() {
        let mut pattern = TuplePattern::without_metadata(
            node_id(1),
            span(),
            vec![node_id(2)],
        )
        .expect("valid tuple pattern");

        pattern.extend_children([
            node_id(3),
            node_id(4),
            node_id(5),
        ]);

        assert_eq!(
            pattern.children().collect::<Vec<_>>(),
            vec![
                node_id(2),
                node_id(3),
                node_id(4),
                node_id(5),
            ]
        );
    }

    #[test]
    fn pop_child_removes_only_last_child() {
        let mut pattern = TuplePattern::without_metadata(
            node_id(1),
            span(),
            vec![
                node_id(2),
                node_id(3),
                node_id(4),
            ],
        )
        .expect("valid tuple pattern");

        assert_eq!(
            pattern.pop_child(),
            Some(node_id(4))
        );

        assert_eq!(
            pattern.children().collect::<Vec<_>>(),
            vec![
                node_id(2),
                node_id(3),
            ]
        );
    }

    #[test]
    fn from_node_accepts_pattern_kind() {
        let node = Node::new(
            node_id(1),
            TUPLE_PATTERN_NODE_KIND.clone(),
            span(),
            NodeMetadata::default(),
        );

        let pattern = TuplePattern::from_node(
            node,
            vec![node_id(2)],
        )
        .expect("pattern-kind node should be accepted");

        assert_eq!(pattern.id(), node_id(1));
        assert_eq!(pattern.len(), 1);
    }

    #[test]
    fn from_node_rejects_wrong_kind() {
        let node = Node::new(
            node_id(1),
            NodeKind::Core(CoreNodeKind::Literal),
            span(),
            NodeMetadata::default(),
        );

        let result = TuplePattern::from_node(
            node,
            vec![node_id(2)],
        );

        assert!(matches!(
            result,
            Err(TuplePatternError::InvalidNodeKind { .. })
        ));
    }

    #[test]
    fn replace_children_preserves_new_order() {
        let mut pattern = TuplePattern::without_metadata(
            node_id(1),
            span(),
            vec![node_id(2), node_id(3)],
        )
        .expect("valid tuple pattern");

        let old = pattern
            .replace_children(vec![
                node_id(7),
                node_id(8),
                node_id(9),
            ])
            .expect("replacement must be non-empty");

        assert_eq!(
            old,
            vec![node_id(2), node_id(3)]
        );

        assert_eq!(
            pattern.children().collect::<Vec<_>>(),
            vec![
                node_id(7),
                node_id(8),
                node_id(9),
            ]
        );
    }

    #[test]
    fn replace_children_rejects_empty_and_preserves_original() {
        let mut pattern = TuplePattern::without_metadata(
            node_id(1),
            span(),
            vec![node_id(2), node_id(3)],
        )
        .expect("valid tuple pattern");

        let result = pattern.replace_children(Vec::new());

        assert_eq!(
            result,
            Err(TuplePatternError::Empty)
        );

        assert_eq!(
            pattern.children().collect::<Vec<_>>(),
            vec![node_id(2), node_id(3)]
        );
    }

    #[test]
    fn child_slice_is_zero_copy_view() {
        let pattern = TuplePattern::without_metadata(
            node_id(1),
            span(),
            vec![node_id(2), node_id(3)],
        )
        .expect("valid tuple pattern");

        assert_eq!(
            pattern.child_slice(),
            &[node_id(2), node_id(3)]
        );
    }

    #[test]
    fn ast_node_trait_exposes_common_identity() {
        let pattern = TuplePattern::without_metadata(
            node_id(100),
            span(),
            vec![node_id(200)],
        )
        .expect("valid tuple pattern");

        let ast_node: &dyn AstNode = &pattern;

        assert_eq!(ast_node.id(), node_id(100));
        assert_eq!(
            ast_node.kind(),
            &TUPLE_PATTERN_NODE_KIND
        );
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            TuplePattern::schema_version(),
            TUPLE_PATTERN_SCHEMA_VERSION
        );
    }

    #[test]
    fn large_dynamic_arity_has_no_ast_level_fixed_limit() {
        let count = 10_000usize;

        let children: Vec<NodeId> = (0..count)
            .map(|index| NodeId::new(index as u64 + 1))
            .collect();

        let pattern = TuplePattern::without_metadata(
            NodeId::new(count as u64 + 1),
            span(),
            children,
        )
        .expect("large tuple pattern should construct");

        assert_eq!(pattern.len(), count);
        assert_eq!(
            pattern.child(9_999),
            Some(NodeId::new(10_000))
        );
    }
}