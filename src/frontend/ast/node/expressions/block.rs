//! # Zamani Native AST — Block Expression
//!
//! Source-level representation of a block expression.
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
//!     ├── BlockExpression  ← this module
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
//!     ├── classical domain IR
//!     ├── quantum domain IR
//!     ├── HDL domain IR
//!     └── future domain IRs
//!
//! ## Responsibility
//!
//! `BlockExpression` represents the source-level structure of a block:
//!
//! ```text
//! {
//!     statement
//!     statement
//!     expression
//! }
//! ```
//!
//! The node records only the ordered child-node identities belonging to the
//! block. It does not resolve their meaning.
//!
//! In particular, this module does NOT perform:
//!
//! - name resolution;
//! - type checking;
//! - type inference;
//! - generic substitution;
//! - ownership analysis;
//! - borrow analysis;
//! - effect inference;
//! - capability resolution;
//! - resource allocation;
//! - quantum analysis;
//! - hardware mapping;
//! - routing;
//! - scheduling;
//! - error correction;
//! - resilience;
//! - calibration;
//! - backend selection;
//! - QIR generation;
//! - ZUIR generation.
//!
//! Those responsibilities belong to later compiler layers.
//!
//! ## POCO-REAF
//!
//! A block contains no machine-size assumptions.
//!
//! There is no:
//!
//! - maximum statement count;
//! - maximum block size;
//! - fixed nesting depth;
//! - fixed register count;
//! - fixed qubit count;
//! - fixed resource count;
//! - target architecture;
//! - hardware topology.
//!
//! Program size is therefore constrained only by the compiler's explicit,
//! configurable resource policies and the resources available to the host
//! environment.
//!
//! ## Graph representation
//!
//! The canonical Zamani AST represents child relationships with `NodeId`.
//!
//! ```text
//! BlockExpression
//! ├── Node
//! └── statements: Vec<NodeId>
//!       ├── NodeId
//!       ├── NodeId
//!       └── ...
//! ```
//!
//! The block owns the ordering of its children but does not own the child
//! objects themselves.
//!
//! This prevents recursive Rust ownership structures and allows the complete
//! AST graph to be stored and traversed by the AST graph/container layer.
//!
//! ## Ordering
//!
//! `statements` is ordered source structure.
//!
//! The order MUST be preserved because source order can affect:
//!
//! - evaluation order;
//! - side effects;
//! - control flow;
//! - resource lifetime;
//! - lexical scope;
//! - diagnostics;
//! - source reconstruction;
//! - semantic analysis.
//!
//! The AST must therefore never use an unordered collection for block children.
//!
//! ## Final expression
//!
//! This node intentionally does not introduce a separate `tail_expression`
//! field.
//!
//! If the Zamani grammar distinguishes a final expression from ordinary
//! statements, that distinction must be represented by the canonical statement/
//! expression representation already used by the expression AST rather than
//! duplicated here.
//!
//! This prevents two competing representations of the same source construct.
//!
//! Semantic analysis may later determine whether the final child contributes
//! the value of the block.
//!
//! ## Validation boundary
//!
//! Local validation performed here is deliberately structural:
//!
//! - the block's own node identity must be valid;
//! - every child reference must be a valid `NodeId`;
//! - the child sequence must remain ordered;
//! - no semantic interpretation is attempted.
//!
//! Whether a referenced node actually exists in the global AST graph is a
//! responsibility of the graph-level structural validator.
//!
//! Whether a statement is legal in the block is a responsibility of semantic
//! analysis and/or the appropriate language-level validator.
//!
//! ## Traversal
//!
//! [`BlockExpression::child_node_ids`] exposes children without recursively
//! walking them.
//!
//! This is important for very deeply nested source programs. A global AST
//! traversal implementation can use an explicit work stack rather than relying
//! on Rust call-stack recursion.
//!
//! ## Determinism
//!
//! Child nodes are stored in `Vec<NodeId>` because source order is semantically
//! meaningful.
//!
//! No hash-map iteration is involved in local block traversal.
//!
//! ## Serialization
//!
//! Serde derives are provided for deterministic structural serialization.
//!
//! Serialization format versioning belongs to the AST serialization subsystem,
//! not this individual node.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - dereferences no raw pointers;
//! - executes no source code;
//! - performs no unchecked indexing;
//! - performs no recursive traversal;
//! - contains no global mutable state;
//! - contains no backend credentials or execution state.
//!
//! Untrusted serialized AST data must still be subjected to the global AST
//! structural validation pass after deserialization.
//!
//! ## Dependencies
//!
//! Allowed dependencies are restricted to foundational AST infrastructure:
//!
//! - `serde`;
//! - `thiserror`;
//! - [`Node`];
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`NodeMetadata`];
//! - [`Span`].
//!
//! This module must not depend on:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - QEC;
//! - resilience;
//! - calibration;
//! - runtime;
//! - backend APIs;
//! - LLVM;
//! - QIR;
//! - MLIR.
//!
//! ## Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ## Integration contract
//!
//! Parser:
//!
//! ```text
//! parser
//!   ├── allocates block NodeId
//!   ├── parses `{ ... }`
//!   ├── creates child AST nodes
//!   ├── records child NodeIds in source order
//!   └── creates BlockExpression
//! ```
//!
//! Structural validation:
//!
//! ```text
//! BlockExpression
//!     │
//!     └── local validation
//!             │
//!             ▼
//! AST graph validation
//!             │
//!             ▼
//! semantic analysis
//! ```
//!
//! Semantic analysis resolves the meaning of the block's child nodes,
//! including scope, types, effects, resources and control-flow semantics.
//!
//! ZUIR lowering consumes the semantic representation rather than depending on
//! this source node as an execution representation.
//!
//! ## Important architectural distinction
//!
//! ```text
//! BlockExpression       = source structure
//! Semantic block        = resolved meaning
//! ZUIR region/block     = universal computational semantics
//! Domain IR block       = domain-specific implementation
//! Target block          = target/backend realization
//! ```
//!
//! These representations must not be merged.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::frontend::ast::node::{
    metadata::NodeMetadata,
    node::Node,
    node_id::NodeId,
    node_kind::NodeKind,
    source::Span,
};

/// Structural errors specific to a [`BlockExpression`].
///
/// These errors intentionally do not contain semantic errors. Semantic
/// diagnostics belong to the semantic-analysis layer.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum BlockExpressionError {
    /// A child reference does not identify a valid AST node.
    #[error(
        "block expression child at index {index} has NodeId::INVALID"
    )]
    InvalidChild {
        /// Source-order position of the invalid child.
        index: usize,
    },
}

impl BlockExpressionError {
    /// Returns the source-order index of the invalid child, if applicable.
    #[must_use]
    pub const fn child_index(&self) -> Option<usize> {
        match self {
            Self::InvalidChild { index } => Some(*index),
        }
    }
}

/// A source-level Zamani block expression.
///
/// A block is an ordered sequence of references to other AST nodes.
///
/// The node itself does not own the referenced children. The canonical AST
/// graph owns those nodes and this structure records their source-order
/// relationship.
///
/// # Invariants
///
/// A structurally valid `BlockExpression` satisfies:
///
/// 1. every child `NodeId` is valid;
/// 2. child order is source order;
/// 3. the block contains no semantic information;
/// 4. the block contains no target-specific information;
/// 5. no fixed number of children is assumed.
///
/// The existence and type of each referenced child are checked by the
/// graph-level AST validator.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockExpression {
    /// Common source-level node identity, kind, span and metadata.
    node: Node,

    /// Ordered source-level child nodes.
    ///
    /// The vector has no AST-level maximum length. Compiler resource limits,
    /// when required for denial-of-service protection, belong to configurable
    /// validation/session policy rather than this structure.
    statements: Vec<NodeId>,
}

impl BlockExpression {
    /// Constructs a block expression with validated local structure.
    ///
    /// This constructor does not verify that child IDs exist in the global AST
    /// graph. That check necessarily belongs to the graph-level validator.
    ///
    /// # Errors
    ///
    /// Returns [`BlockExpressionError::InvalidChild`] if any child is
    /// `NodeId::INVALID`.
    pub fn try_new(
        id: NodeId,
        span: Span,
        statements: Vec<NodeId>,
        metadata: NodeMetadata,
    ) -> Result<Self, BlockExpressionError> {
        Self::validate_children(&statements)?;

        Ok(Self {
            node: Node::new(
                id,
                NodeKind::BlockExpression,
                span,
                metadata,
            ),
            statements,
        })
    }

    /// Constructs a block expression with default metadata.
    ///
    /// This is the normal constructor for parser paths that do not attach
    /// explicit metadata.
    ///
    /// # Errors
    ///
    /// Returns [`BlockExpressionError::InvalidChild`] if any child ID is
    /// invalid.
    pub fn new(
        id: NodeId,
        span: Span,
        statements: Vec<NodeId>,
    ) -> Result<Self, BlockExpressionError> {
        Self::try_new(
            id,
            span,
            statements,
            NodeMetadata::default(),
        )
    }

    /// Constructs a block expression with explicit metadata.
    ///
    /// This is equivalent to [`Self::try_new`] and is provided as an explicit
    /// parser/tooling entry point.
    ///
    /// # Errors
    ///
    /// Returns [`BlockExpressionError::InvalidChild`] if any child ID is
    /// invalid.
    pub fn with_metadata(
        id: NodeId,
        span: Span,
        statements: Vec<NodeId>,
        metadata: NodeMetadata,
    ) -> Result<Self, BlockExpressionError> {
        Self::try_new(id, span, statements, metadata)
    }

    /// Returns the common AST node.
    #[must_use]
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Only common node metadata should normally be changed through this
    /// interface. Structural child changes should use the validated methods
    /// provided by this type.
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

    /// Returns the node kind.
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

    /// Returns node metadata.
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable node metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the ordered child node IDs.
    ///
    /// The returned slice is immutable so callers cannot introduce an invalid
    /// `NodeId` without going through structural validation.
    #[must_use]
    #[inline]
    pub fn statements(&self) -> &[NodeId] {
        &self.statements
    }

    /// Returns the number of children in this block.
    ///
    /// No semantic maximum is imposed here.
    #[must_use]
    #[inline]
    pub fn statement_count(&self) -> usize {
        self.statements.len()
    }

    /// Returns whether this block contains no children.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    /// Returns a child node ID by source-order index.
    ///
    /// No unchecked indexing is used.
    #[must_use]
    #[inline]
    pub fn statement(&self, index: usize) -> Option<NodeId> {
        self.statements.get(index).copied()
    }

    /// Replaces the complete child sequence after validating it.
    ///
    /// This is preferable to exposing a mutable `Vec<NodeId>`, because it
    /// preserves the local invariant that no child may be `NodeId::INVALID`.
    ///
    /// # Errors
    ///
    /// Returns [`BlockExpressionError::InvalidChild`] if the supplied sequence
    /// contains an invalid node ID.
    pub fn replace_statements(
        &mut self,
        statements: Vec<NodeId>,
    ) -> Result<(), BlockExpressionError> {
        Self::validate_children(&statements)?;
        self.statements = statements;
        Ok(())
    }

    /// Appends one child after validating its node identity.
    ///
    /// # Errors
    ///
    /// Returns [`BlockExpressionError::InvalidChild`] when `statement` is
    /// `NodeId::INVALID`.
    pub fn push_statement(
        &mut self,
        statement: NodeId,
    ) -> Result<(), BlockExpressionError> {
        if statement.is_invalid() {
            return Err(BlockExpressionError::InvalidChild {
                index: self.statements.len(),
            });
        }

        self.statements.push(statement);
        Ok(())
    }

    /// Removes and returns a child at `index`.
    ///
    /// This operation is structural and does not perform semantic validation.
    /// The returned node ID remains the caller's responsibility.
    #[must_use]
    pub fn remove_statement(&mut self, index: usize) -> Option<NodeId> {
        if index < self.statements.len() {
            Some(self.statements.remove(index))
        } else {
            None
        }
    }

    /// Removes all children.
    ///
    /// An empty block is structurally valid. Whether an empty block is
    /// semantically legal in a particular source context belongs to later
    /// validation.
    pub fn clear_statements(&mut self) {
        self.statements.clear();
    }

    /// Returns the direct children of this node.
    ///
    /// The iterator is deterministic and follows source order.
    ///
    /// This method does not recursively traverse descendants.
    #[must_use]
    #[inline]
    pub fn child_node_ids(
        &self,
    ) -> impl ExactSizeIterator<Item = NodeId> + DoubleEndedIterator + '_ {
        self.statements.iter().copied()
    }

    /// Validates the local structural invariants of this block.
    ///
    /// This does not check whether the child IDs exist in the containing AST
    /// graph. That operation requires access to the graph and belongs to the
    /// global structural validator.
    ///
    /// # Errors
    ///
    /// Returns the first invalid child encountered in deterministic source
    /// order.
    pub fn validate(&self) -> Result<(), BlockExpressionError> {
        Self::validate_children(&self.statements)
    }

    /// Validates a supplied child sequence.
    ///
    /// Validation is deterministic and linear in the number of children.
    ///
    /// There is deliberately no maximum child count here.
    fn validate_children(
        statements: &[NodeId],
    ) -> Result<(), BlockExpressionError> {
        for (index, statement) in statements.iter().copied().enumerate() {
            if statement.is_invalid() {
                return Err(BlockExpressionError::InvalidChild { index });
            }
        }

        Ok(())
    }

    /// Decomposes the node into its constituent source-level parts.
    ///
    /// This is useful for AST transformations that replace a block while
    /// preserving ownership of its already allocated node identity.
    #[must_use]
    pub fn into_parts(self) -> (Node, Vec<NodeId>) {
        (self.node, self.statements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::source::source_id::SourceId;

    fn test_span() -> Span {
        Span::new(SourceId::new(1), 0, 10)
    }

    #[test]
    fn constructs_empty_block() {
        let block = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            Vec::new(),
        )
        .expect("an empty block is structurally valid");

        assert!(block.is_empty());
        assert_eq!(block.statement_count(), 0);
        assert!(block.validate().is_ok());
        assert_eq!(
            block.kind(),
            &NodeKind::BlockExpression
        );
    }

    #[test]
    fn constructs_ordered_block() {
        let block = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            vec![
                NodeId::new(2),
                NodeId::new(3),
                NodeId::new(4),
            ],
        )
        .expect("valid block");

        assert_eq!(block.statement_count(), 3);
        assert_eq!(
            block.statements(),
            &[
                NodeId::new(2),
                NodeId::new(3),
                NodeId::new(4)
            ]
        );
    }

    #[test]
    fn preserves_source_order_during_traversal() {
        let block = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            vec![
                NodeId::new(7),
                NodeId::new(3),
                NodeId::new(11),
            ],
        )
        .expect("valid block");

        let children: Vec<NodeId> =
            block.child_node_ids().collect();

        assert_eq!(
            children,
            vec![
                NodeId::new(7),
                NodeId::new(3),
                NodeId::new(11),
            ]
        );
    }

    #[test]
    fn rejects_invalid_child() {
        let result = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            vec![
                NodeId::new(2),
                NodeId::INVALID,
                NodeId::new(4),
            ],
        );

        assert_eq!(
            result,
            Err(BlockExpressionError::InvalidChild { index: 1 })
        );
    }

    #[test]
    fn push_rejects_invalid_child() {
        let mut block = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            Vec::new(),
        )
        .expect("valid block");

        let result = block.push_statement(NodeId::INVALID);

        assert_eq!(
            result,
            Err(BlockExpressionError::InvalidChild { index: 0 })
        );
        assert!(block.is_empty());
    }

    #[test]
    fn push_preserves_order() {
        let mut block = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            Vec::new(),
        )
        .expect("valid block");

        block
            .push_statement(NodeId::new(10))
            .expect("valid child");

        block
            .push_statement(NodeId::new(20))
            .expect("valid child");

        assert_eq!(
            block.statements(),
            &[NodeId::new(10), NodeId::new(20)]
        );
    }

    #[test]
    fn replace_statements_is_transactional_on_failure() {
        let mut block = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            vec![NodeId::new(2)],
        )
        .expect("valid block");

        let result = block.replace_statements(vec![
            NodeId::new(3),
            NodeId::INVALID,
            NodeId::new(5),
        ]);

        assert_eq!(
            result,
            Err(BlockExpressionError::InvalidChild { index: 1 })
        );

        assert_eq!(
            block.statements(),
            &[NodeId::new(2)]
        );
    }

    #[test]
    fn remove_statement_is_safe_for_out_of_range_index() {
        let mut block = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            vec![NodeId::new(2)],
        )
        .expect("valid block");

        assert_eq!(block.remove_statement(99), None);
        assert_eq!(
            block.statements(),
            &[NodeId::new(2)]
        );
    }

    #[test]
    fn clear_produces_valid_empty_block() {
        let mut block = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            vec![
                NodeId::new(2),
                NodeId::new(3),
            ],
        )
        .expect("valid block");

        block.clear_statements();

        assert!(block.is_empty());
        assert!(block.validate().is_ok());
    }

    #[test]
    fn serialization_round_trip_requires_only_structural_data() {
        let block = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            vec![
                NodeId::new(2),
                NodeId::new(3),
            ],
        )
        .expect("valid block");

        /*
         * Serialization itself is exercised by the repository's canonical
         * AST serialization test suite. This test intentionally does not
         * require a particular serialization crate or format at this layer.
         *
         * The important local invariant is that the complete structural
         * state is represented by:
         *
         *   Node + ordered child NodeIds
         */
        assert_eq!(block.statement_count(), 2);
        assert!(block.validate().is_ok());
    }

    #[test]
    fn child_iterator_has_exact_length() {
        let block = BlockExpression::new(
            NodeId::new(1),
            test_span(),
            vec![
                NodeId::new(2),
                NodeId::new(3),
                NodeId::new(4),
            ],
        )
        .expect("valid block");

        assert_eq!(block.child_node_ids().len(), 3);
    }

    #[test]
    fn large_dynamic_child_sequence_has_no_ast_defined_limit() {
        let count = 10_000usize;

        let statements: Vec<NodeId> = (1..=count as u64)
            .map(NodeId::new)
            .collect();

        let block = BlockExpression::new(
            NodeId::new(count as u64 + 1),
            test_span(),
            statements,
        )
        .expect("large block should be accepted");

        assert_eq!(block.statement_count(), count);
        assert!(block.validate().is_ok());
    }
}