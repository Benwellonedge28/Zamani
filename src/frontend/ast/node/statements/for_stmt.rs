//! # Zamani Native AST — For Statement
//!
//! Production-ready source-level representation of a `for` loop.
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
//! ForStatement                         ← this module
//!     │
//!     ▼
//! Native Zamani AST
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
//!     ├── hybrid IR
//!     ├── distributed IR
//!     ├── accelerator IR
//!     └── future-domain IR
//!     │
//!     ▼
//! target lowering / execution
//! ```
//!
//! ## Responsibility
//!
//! This module owns the source-level structure of a native Zamani `for`
//! statement.
//!
//! A `for` statement contains exactly three structural references:
//!
//! ```text
//! ForStatement
//! ├── Node
//! ├── pattern: NodeId
//! ├── iterable: NodeId
//! └── body: NodeId
//! ```
//!
//! The referenced nodes remain owned by the enclosing AST graph.
//!
//! This module does not own or resolve those nodes.
//!
//! ## Domain neutrality
//!
//! The loop itself is domain-neutral.
//!
//! The iterable may eventually represent:
//!
//! - a classical collection;
//! - a symbolic range;
//! - a quantum-resource collection;
//! - a distributed resource set;
//! - an accelerator resource set;
//! - an extension-defined iterable;
//! - a future computational resource.
//!
//! The AST does not determine which interpretation applies.
//!
//! In particular, this type contains no:
//!
//! - qubit count;
//! - physical qubit identifier;
//! - QPU identifier;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - gate set;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC implementation;
//! - noise model;
//! - resilience implementation;
//! - runtime iterator;
//! - CPU/GPU/FPGA instruction;
//! - QIR value;
//! - LLVM value;
//! - MLIR operation.
//!
//! Those concerns belong to later compiler phases.
//!
//! ## POCO-REAF
//!
//! The loop contains no machine-size assumptions.
//!
//! A source program can therefore describe iteration over a resource whose
//! eventual size is determined later by generic parameters, symbolic values,
//! compiler analysis, available resources, or target capabilities.
//!
//! The AST introduces no semantic upper bound on:
//!
//! - iterations;
//! - iterable size;
//! - program size;
//! - loop nesting;
//! - number of resources;
//! - number of machines;
//! - number of quantum resources.
//!
//! Any operational limits belong to compiler/runtime policy and target
//! capabilities, never to this source-level representation.
//!
//! ## Structural validation boundary
//!
//! Local validation checks only facts available from this object:
//!
//! 1. the node has `CoreNodeKind::ForStatement`;
//! 2. the pattern does not directly reference the loop itself;
//! 3. the iterable does not directly reference the loop itself;
//! 4. the body does not directly reference the loop itself.
//!
//! It does NOT check whether the `NodeId`s resolve in the complete AST.
//!
//! Graph-wide validation owns:
//!
//! - reference resolution;
//! - duplicate ownership checks;
//! - graph-cycle detection;
//! - unreachable-node detection;
//! - configurable node/depth/resource policies.
//!
//! Semantic analysis owns:
//!
//! - pattern validity;
//! - iterable typing;
//! - iterator protocol;
//! - binding semantics;
//! - ownership/borrowing;
//! - break/continue legality;
//! - effects;
//! - capabilities;
//! - resource semantics;
//! - domain interpretation;
//! - generic substitution.
//!
//! ## Deterministic child order
//!
//! Direct children are exposed in source-structural order:
//!
//! ```text
//! pattern → iterable → body
//! ```
//!
//! This ordering is stable and does not depend on hash-map iteration.
//!
//! ## Dependency boundary
//!
//! This file depends only on foundational AST infrastructure and Serde.
//!
//! It must never depend on:
//!
//! - parser implementation;
//! - lexer tokens;
//! - semantic analysis;
//! - compiler driver;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - calibration;
//! - runtime;
//! - backend providers;
//! - LLVM;
//! - QIR;
//! - MLIR.
//!
//! ## Serialization
//!
//! The structure derives Serde serialization/deserialization.
//!
//! The repository-wide AST serialization subsystem remains responsible for
//! global schema versioning and compatibility policy.
//!
//! This file does not introduce a second serialization format.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - executes no source program;
//! - uses no raw pointers;
//! - uses no global mutable state;
//! - uses no unchecked indexing;
//! - performs no recursive AST traversal.
//!
//! AST references are treated as untrusted identifiers.
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
//! - no `unsafe`.
//!
//! # File contract
//!
//! **Owns**
//!
//! - source-level `for` statement representation;
//! - its three child references;
//! - local structural validation;
//! - deterministic child enumeration.
//!
//! **Does not own**
//!
//! - AST graph storage;
//! - name resolution;
//! - type checking;
//! - iterator semantics;
//! - resource allocation;
//! - ZUIR;
//! - quantum IR;
//! - scheduling;
//! - routing;
//! - hardware;
//! - runtime execution.
//!
//! **Parser contract**
//!
//! The parser allocates the node ID, calculates the complete source span,
//! attaches metadata, creates the three child references, and inserts all
//! nodes into the AST graph.
//!
//! **Semantic contract**
//!
//! Semantic analysis consumes this node and resolves the pattern and iterable.
//!
//! **ZUIR contract**
//!
//! The semantic model lowers loop control flow to the appropriate ZUIR
//! representation. This module does not import ZUIR.
//!
//! **No-reedit contract**
//!
//! Changes to hardware, quantum backends, QEC, routing, scheduling, calibration,
//! runtime, or target lowering do not require modification of this file.
//!
//! This file should change only when the source-level semantics of Zamani's
//! native `for` construct change.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Local schema version for the `for` statement representation.
///
/// This is intentionally independent of the language version and the global
/// AST serialization schema version.
pub const FOR_STATEMENT_SCHEMA_VERSION: u16 = 1;

/// Stable source-level name for this construct.
pub const FOR_STATEMENT_KIND_NAME: &str = "zamani:for-statement";

/// Result type for local `for` statement operations.
pub type ForStatementResult<T> = Result<T, ForStatementError>;

/// Errors that can be detected without access to the complete AST graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ForStatementError {
    /// The embedded common node has the wrong node kind.
    InvalidNodeKind {
        /// Actual node kind found in the embedded node.
        actual: NodeKind,
    },

    /// The pattern points directly back to the loop itself.
    SelfReferentialPattern,

    /// The iterable points directly back to the loop itself.
    SelfReferentialIterable,

    /// The body points directly back to the loop itself.
    SelfReferentialBody,
}

impl fmt::Display for ForStatementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "for statement has invalid AST node kind: {actual}"
                )
            }

            Self::SelfReferentialPattern => {
                formatter.write_str(
                    "for statement pattern cannot directly reference the loop",
                )
            }

            Self::SelfReferentialIterable => {
                formatter.write_str(
                    "for statement iterable cannot directly reference the loop",
                )
            }

            Self::SelfReferentialBody => {
                formatter.write_str(
                    "for statement body cannot directly reference the loop",
                )
            }
        }
    }
}

impl std::error::Error for ForStatementError {}

/// Canonical source-level `for` statement.
///
/// The three child nodes are referenced by `NodeId` so the enclosing AST graph
/// remains responsible for storage and graph-wide validation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ForStatement {
    /// Common AST identity, node kind, source span and metadata.
    node: Node,

    /// Pattern receiving each item produced by the iterable.
    pattern: NodeId,

    /// Expression producing the value/resource sequence being iterated.
    iterable: NodeId,

    /// Statement/block executed for each iteration.
    body: NodeId,
}

impl ForStatement {
    /// Creates a `for` statement without performing graph-wide validation.
    ///
    /// The caller owns `NodeId` allocation and AST graph insertion.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        pattern: NodeId,
        iterable: NodeId,
        body: NodeId,
    ) -> Self {
        Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::ForStatement),
                span,
                metadata,
            ),
            pattern,
            iterable,
            body,
        }
    }

    /// Creates a `for` statement and immediately performs local structural
    /// validation.
    ///
    /// This does not verify that the referenced child IDs exist in the AST
    /// graph. That requires the graph validator.
    pub fn try_new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        pattern: NodeId,
        iterable: NodeId,
        body: NodeId,
    ) -> ForStatementResult<Self> {
        let statement = Self::new(
            id,
            span,
            metadata,
            pattern,
            iterable,
            body,
        );

        statement.validate_structure()?;

        Ok(statement)
    }

    /// Returns the node's stable AST identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the complete source span of the `for` statement.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns mutable access to the source span.
    ///
    /// Callers changing spans must preserve the repository-wide source-span
    /// invariants.
    #[inline]
    pub fn span_mut(&mut self) -> &mut Span {
        self.node.span_mut()
    }

    /// Returns source metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable source metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the canonical node kind.
    #[inline]
    #[must_use]
    pub fn node_kind(&self) -> NodeKind {
        self.node.node_kind()
    }

    /// Returns the loop binding pattern.
    #[inline]
    #[must_use]
    pub fn pattern(&self) -> NodeId {
        self.pattern
    }

    /// Returns the iterable expression.
    #[inline]
    #[must_use]
    pub fn iterable(&self) -> NodeId {
        self.iterable
    }

    /// Returns the loop body.
    #[inline]
    #[must_use]
    pub fn body(&self) -> NodeId {
        self.body
    }

    /// Replaces the pattern reference.
    ///
    /// This is a structural AST transformation operation. It does not resolve
    /// or validate the replacement node against the enclosing graph.
    #[inline]
    pub fn set_pattern(&mut self, pattern: NodeId) {
        self.pattern = pattern;
    }

    /// Replaces the iterable reference.
    ///
    /// Semantic validation remains the responsibility of later phases.
    #[inline]
    pub fn set_iterable(&mut self, iterable: NodeId) {
        self.iterable = iterable;
    }

    /// Replaces the body reference.
    ///
    /// The caller must run local/graph validation after a transformation when
    /// necessary.
    #[inline]
    pub fn set_body(&mut self, body: NodeId) {
        self.body = body;
    }

    /// Returns the expected canonical node kind.
    #[inline]
    #[must_use]
    pub const fn expected_node_kind() -> NodeKind {
        NodeKind::Core(CoreNodeKind::ForStatement)
    }

    /// Returns the stable source-level kind name.
    #[inline]
    #[must_use]
    pub const fn kind_name() -> &'static str {
        FOR_STATEMENT_KIND_NAME
    }

    /// Returns the local schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        FOR_STATEMENT_SCHEMA_VERSION
    }

    /// Validates structural properties that can be checked without access to
    /// the complete AST graph.
    ///
    /// This deliberately does not perform:
    ///
    /// - name resolution;
    /// - type checking;
    /// - iterator validation;
    /// - resource validation;
    /// - semantic control-flow validation;
    /// - hardware validation;
    /// - target validation.
    pub fn validate_structure(&self) -> ForStatementResult<()> {
        let expected = Self::expected_node_kind();

        if self.node.node_kind() != expected {
            return Err(ForStatementError::InvalidNodeKind {
                actual: self.node.node_kind(),
            });
        }

        let id = self.id();

        if self.pattern == id {
            return Err(ForStatementError::SelfReferentialPattern);
        }

        if self.iterable == id {
            return Err(ForStatementError::SelfReferentialIterable);
        }

        if self.body == id {
            return Err(ForStatementError::SelfReferentialBody);
        }

        Ok(())
    }

    /// Returns the three direct child node IDs in deterministic source order.
    ///
    /// Order:
    ///
    /// 1. pattern;
    /// 2. iterable;
    /// 3. body.
    ///
    /// A fixed-size array is used because this node has exactly three
    /// structurally required children. This avoids heap allocation while
    /// remaining independent of machine size or program size.
    #[must_use]
    pub fn child_node_ids(&self) -> [NodeId; 3] {
        [self.pattern, self.iterable, self.body]
    }

    /// Returns the direct children as an iterator without allocating a `Vec`.
    ///
    /// This method is useful to traversal infrastructure that wants a
    /// zero-heap local child enumeration.
    pub fn children(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.child_node_ids().into_iter()
    }

    /// Rebuilds this node with a new common AST node while preserving its
    /// source-level loop children.
    ///
    /// This is useful to AST transformation infrastructure when replacing the
    /// common node metadata/span/identity without changing loop semantics.
    #[must_use]
    pub fn with_node(&self, node: Node) -> Self {
        Self {
            node,
            pattern: self.pattern,
            iterable: self.iterable,
            body: self.body,
        }
    }
}

impl AstNode for ForStatement {
    /// Returns the canonical node kind.
    #[inline]
    #[must_use]
    fn node_kind(&self) -> NodeKind {
        self.node.node_kind()
    }

    /// Returns the source span.
    #[inline]
    #[must_use]
    fn span(&self) -> &Span {
        self.node.span()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::source::SourceId;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test NodeId must be non-zero")
    }

    fn span() -> Span {
        Span::new(SourceId::new(1), 0, 20)
    }

    #[test]
    fn constructor_preserves_all_fields() {
        let statement = ForStatement::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(2),
            node_id(3),
            node_id(4),
        );

        assert_eq!(statement.id(), node_id(1));
        assert_eq!(statement.pattern(), node_id(2));
        assert_eq!(statement.iterable(), node_id(3));
        assert_eq!(statement.body(), node_id(4));
        assert_eq!(
            statement.node_kind(),
            NodeKind::core(CoreNodeKind::ForStatement)
        );
    }

    #[test]
    fn try_new_accepts_valid_structure() {
        let result = ForStatement::try_new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(2),
            node_id(3),
            node_id(4),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn expected_node_kind_is_canonical() {
        assert_eq!(
            ForStatement::expected_node_kind(),
            NodeKind::Core(CoreNodeKind::ForStatement)
        );
    }

    #[test]
    fn kind_name_is_stable() {
        assert_eq!(
            ForStatement::kind_name(),
            "zamani:for-statement"
        );
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(ForStatement::schema_version(), 1);
    }

    #[test]
    fn child_order_is_pattern_iterable_body() {
        let statement = ForStatement::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(2),
            node_id(3),
            node_id(4),
        );

        assert_eq!(
            statement.child_node_ids(),
            [node_id(2), node_id(3), node_id(4)]
        );

        let children: Vec<_> = statement.children().collect();

        assert_eq!(
            children,
            vec![node_id(2), node_id(3), node_id(4)]
        );
    }

    #[test]
    fn self_referential_pattern_is_rejected() {
        let result = ForStatement::try_new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(1),
            node_id(3),
            node_id(4),
        );

        assert_eq!(
            result,
            Err(ForStatementError::SelfReferentialPattern)
        );
    }

    #[test]
    fn self_referential_iterable_is_rejected() {
        let result = ForStatement::try_new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(2),
            node_id(1),
            node_id(4),
        );

        assert_eq!(
            result,
            Err(ForStatementError::SelfReferentialIterable)
        );
    }

    #[test]
    fn self_referential_body_is_rejected() {
        let result = ForStatement::try_new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(2),
            node_id(3),
            node_id(1),
        );

        assert_eq!(
            result,
            Err(ForStatementError::SelfReferentialBody)
        );
    }

    #[test]
    fn wrong_node_kind_is_rejected() {
        let node = Node::new(
            node_id(1),
            NodeKind::core(CoreNodeKind::WhileStatement),
            span(),
            NodeMetadata::default(),
        );

        let statement = ForStatement {
            node,
            pattern: node_id(2),
            iterable: node_id(3),
            body: node_id(4),
        };

        assert_eq!(
            statement.validate_structure(),
            Err(ForStatementError::InvalidNodeKind {
                actual: NodeKind::core(CoreNodeKind::WhileStatement),
            })
        );
    }

    #[test]
    fn setters_preserve_structural_model() {
        let mut statement = ForStatement::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(2),
            node_id(3),
            node_id(4),
        );

        statement.set_pattern(node_id(5));
        statement.set_iterable(node_id(6));
        statement.set_body(node_id(7));

        assert_eq!(statement.pattern(), node_id(5));
        assert_eq!(statement.iterable(), node_id(6));
        assert_eq!(statement.body(), node_id(7));
    }

    #[test]
    fn serde_round_trip_preserves_statement() {
        let statement = ForStatement::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(2),
            node_id(3),
            node_id(4),
        );

        let encoded =
            serde_json::to_string(&statement)
                .expect("serialization should succeed");

        let decoded: ForStatement =
            serde_json::from_str(&encoded)
                .expect("deserialization should succeed");

        assert_eq!(decoded, statement);
    }

    #[test]
    fn source_span_is_preserved() {
        let statement = ForStatement::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(2),
            node_id(3),
            node_id(4),
        );

        assert_eq!(statement.span(), &span());
    }

    #[test]
    fn metadata_is_preserved() {
        let statement = ForStatement::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(2),
            node_id(3),
            node_id(4),
        );

        assert_eq!(
            statement.metadata(),
            &NodeMetadata::default()
        );
    }

    #[test]
    fn no_semantic_or_hardware_state_is_stored() {
        let statement = ForStatement::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            node_id(2),
            node_id(3),
            node_id(4),
        );

        // The only loop-specific state is source-level structure.
        assert_eq!(statement.child_node_ids().len(), 3);
    }
}