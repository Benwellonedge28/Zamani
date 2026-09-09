//! # Zamani Native AST — Await Expression
//!
//! Production-ready source-level representation of the Zamani `await`
//! expression.
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
//!     ├── AwaitExpression  ← this module
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
//!     ├── HDL IR
//!     └── future domain IRs
//! ```
//!
//! ## Responsibility
//!
//! This module owns the **source-level structural representation** of an
//! `await` expression.
//!
//! Conceptually:
//!
//! ```text
//! await <expression>
//! ```
//!
//! The node records:
//!
//! - canonical AST identity;
//! - source span;
//! - source-level metadata;
//! - the `NodeId` of the expression being awaited.
//!
//! It does **not** determine what the awaited expression means or how its
//! completion is implemented.
//!
//! ## Critical architectural boundary
//!
//! This is a native Zamani AST node, not an execution representation.
//!
//! This module must never contain:
//!
//! - executor state;
//! - scheduler state;
//! - task handles;
//! - thread IDs;
//! - process IDs;
//! - runtime futures;
//! - polling state;
//! - wake queues;
//! - reactor state;
//! - event-loop state;
//! - hardware queue state;
//! - device identifiers;
//! - CPU-specific instructions;
//! - GPU-specific instructions;
//! - QPU-specific instructions;
//! - physical qubit identifiers;
//! - quantum scheduling information;
//! - quantum routing information;
//! - calibration data;
//! - QEC state;
//! - resilience state;
//! - backend identifiers;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - runtime objects.
//!
//! Those concerns belong to later compiler/runtime layers.
//!
//! ## POCO-REAF
//!
//! The `await` expression is represented entirely using source-level AST
//! concepts.
//!
//! Consequently, the same source construct can eventually participate in:
//!
//! - ordinary asynchronous computation;
//! - distributed computation;
//! - accelerator computation;
//! - hybrid classical/quantum computation;
//! - asynchronous resource operations;
//! - future computational domains;
//! - other language-defined asynchronous abstractions.
//!
//! The AST introduces no assumptions about:
//!
//! - machine size;
//! - processor count;
//! - thread count;
//! - task count;
//! - register width;
//! - memory size;
//! - qubit count;
//! - quantum technology;
//! - topology;
//! - vendor;
//! - backend;
//! - runtime implementation.
//!
//! "Infinity" in the POCO-REAF architecture means that this node introduces no
//! artificial finite machine or program-size limit. Actual compilation remains
//! bounded only by available resources and explicitly configured compiler
//! resource policies.
//!
//! ## Child representation
//!
//! The awaited expression is represented by [`NodeId`].
//!
//! ```text
//! AwaitExpression
//! ├── Node
//! └── expression: NodeId
//! ```
//!
//! The AST graph owns the actual child node.
//!
//! This deliberately avoids recursively embedding another concrete expression
//! and allows the repository's traversal system to control recursion and
//! resource usage.
//!
//! ## Why `NodeId` instead of `Box<Expression>`?
//!
//! The canonical frontend AST uses a graph-oriented representation for
//! cross-node relationships. This is particularly important for very large
//! programs and for compiler infrastructure that needs:
//!
//! - centralized node ownership;
//! - stable identities;
//! - side tables;
//! - iterative traversal;
//! - source mapping;
//! - diagnostics;
//! - incremental compilation;
//! - transformation bookkeeping.
//!
//! `AwaitExpression` therefore stores only the child identity.
//!
//! ## Semantic neutrality
//!
//! This node does not assume that the awaited value is a particular runtime
//! future type.
//!
//! For example, semantic analysis may eventually determine that an awaited
//! expression represents:
//!
//! - a language future;
//! - a promise;
//! - a distributed operation;
//! - an asynchronous I/O operation;
//! - an asynchronous resource operation;
//! - a domain extension;
//! - another language-defined awaitable abstraction.
//!
//! None of those interpretations belongs in this file.
//!
//! ## Quantum compatibility
//!
//! Quantum computation must not be forced into a special asynchronous AST.
//!
//! A quantum operation that is semantically asynchronous can be represented
//! through ordinary source constructs and later interpreted by semantic/domain
//! compilation.
//!
//! For example:
//!
//! ```text
//! let result = await computation();
//! ```
//!
//! remains structurally:
//!
//! ```text
//! AwaitExpression
//!     └── expression: NodeId
//! ```
//!
//! The quantum compiler may later determine whether the computation requires:
//!
//! - asynchronous QPU execution;
//! - distributed execution;
//! - classical feedback;
//! - resource synchronization;
//! - remote execution;
//! - another execution strategy.
//!
//! None of those details is represented here.
//!
//! ## No special quantum await node
//!
//! This module must never introduce constructs such as:
//!
//! ```text
//! QuantumAwait
//! QpuAwait
//! QubitAwait
//! HardwareAwait
//! BackendAwait
//! ```
//!
//! Generic source semantics are sufficient.
//!
//! ## Structural validation boundary
//!
//! Local validation is intentionally small.
//!
//! This module may validate:
//!
//! - that the embedded node has `CoreNodeKind::AwaitExpression`;
//! - that the child does not refer to this node itself.
//!
//! This module does **not** validate:
//!
//! - whether the child `NodeId` exists in the AST graph;
//! - whether the awaited expression is awaitable;
//! - whether its type implements an awaitable protocol;
//! - whether awaiting is legal in the enclosing context;
//! - whether the surrounding function is asynchronous;
//! - whether the operation is quantum;
//! - whether a backend supports asynchronous execution;
//! - whether a runtime has an executor;
//! - whether a device supports the operation.
//!
//! Those checks require global AST or semantic context.
//!
//! ## Cycles
//!
//! A direct self-reference is rejected because:
//!
//! ```text
//! AwaitExpression(id = N)
//!     └── expression = N
//! ```
//!
//! would create an immediate structural cycle.
//!
//! Longer graph cycles are not detected here. Global AST graph validation owns
//! graph-wide cycle detection because detecting arbitrary cycles requires the
//! complete AST graph.
//!
//! ## Determinism
//!
//! An await expression has exactly one direct child.
//!
//! Child ordering is therefore always:
//!
//! ```text
//! expression
//! ```
//!
//! No hash-map iteration or unordered traversal is used.
//!
//! ## Scalability
//!
//! This node has exactly one direct child because that is inherent to the
//! source-level `await` construct.
//!
//! It introduces no limits on:
//!
//! - total AST nodes;
//! - number of await expressions;
//! - nesting depth;
//! - number of asynchronous operations;
//! - number of resources;
//! - number of machines;
//! - number of quantum resources;
//! - number of execution targets.
//!
//! Deep nesting is handled by external AST traversal infrastructure rather than
//! recursively by this module.
//!
//! ## Serialization
//!
//! The node derives Serde serialization.
//!
//! Repository-wide AST serialization remains responsible for the overall schema
//! version and compatibility policy.
//!
//! This module exposes a local structural schema version only to document the
//! version of this node's contract.
//!
//! ## Source preservation
//!
//! The complete source span is owned by the common [`Node`].
//!
//! The child expression owns its own source span through its own AST node.
//!
//! This avoids duplicating source-coordinate information.
//!
//! ## Metadata
//!
//! Source-level metadata belongs to [`NodeMetadata`].
//!
//! Metadata must remain source/compiler metadata and must not be used to smuggle
//! backend state into the AST.
//!
//! ## Dependency contract
//!
//! This module may depend on:
//!
//! - [`AstNode`];
//! - [`Node`];
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`CoreNodeKind`];
//! - [`NodeMetadata`];
//! - [`Span`];
//! - Serde;
//! - the Rust standard library.
//!
//! This module must never depend on:
//!
//! - semantic analysis;
//! - semantic types;
//! - symbol tables;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - OpenQASM;
//! - external quantum-language ASTs.
//!
//! ## Parser integration
//!
//! The grammar already defines the source construct:
//!
//! ```text
//! "await" Expression
//! ```
//!
//! The parser should:
//!
//! 1. parse the `await` keyword;
//! 2. parse the operand expression;
//! 3. allocate a fresh `NodeId` for this await node;
//! 4. calculate the complete source span;
//! 5. attach source metadata;
//! 6. construct [`AwaitExpression`];
//! 7. insert the node into the canonical AST graph.
//!
//! The parser must not:
//!
//! - resolve the awaited type;
//! - create runtime futures;
//! - select an executor;
//! - select hardware;
//! - select a quantum backend.
//!
//! ## Expression aggregate integration
//!
//! The repository already has a canonical expression aggregate containing:
//!
//! ```text
//! ExpressionKind::Await { expression: NodeId }
//! ```
//!
//! This file therefore does not introduce another competing expression enum.
//!
//! The aggregate remains the canonical dispatch representation unless and until
//! the repository deliberately migrates it to store `AwaitExpression` directly.
//!
//! During such a migration, this node remains the authoritative structural
//! definition of the dedicated await-expression contract.
//!
//! ## Node-kind integration
//!
//! The repository already provides:
//!
//! ```text
//! CoreNodeKind::AwaitExpression
//! ```
//!
//! This module uses that existing classification rather than introducing a
//! second node-kind taxonomy.
//!
//! ## Visitor integration
//!
//! Generic AST traversal should visit:
//!
//! 1. the `AwaitExpression` node;
//! 2. its `expression` child.
//!
//! The child is represented by `NodeId` and must be resolved by the AST graph
//! owner/traversal system.
//!
//! This module does not invoke visitors recursively.
//!
//! ## Semantic-analysis integration
//!
//! Semantic analysis consumes this structure and determines:
//!
//! - what the child expression denotes;
//! - the child's type;
//! - whether it is awaitable;
//! - what value the await produces;
//! - effects;
//! - capabilities;
//! - resource dependencies;
//! - control-flow implications;
//! - asynchronous semantics;
//! - domain-specific meaning.
//!
//! Semantic analysis must not write semantic information into this AST node.
//!
//! ## ZUIR integration
//!
//! The semantic representation of an await expression is lowered downstream
//! into ZUIR.
//!
//! This module deliberately has no ZUIR dependency.
//!
//! Conceptually:
//!
//! ```text
//! AwaitExpression
//!      │
//!      ▼
//! semantic await operation
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ├── classical domain
//!      ├── quantum domain
//!      ├── distributed domain
//!      └── future domains
//! ```
//!
//! ## Runtime integration
//!
//! Runtime implementation may eventually perform:
//!
//! - polling;
//! - scheduling;
//! - suspension;
//! - resumption;
//! - synchronization;
//! - distributed coordination;
//! - resource waiting.
//!
//! None of that belongs in the AST.
//!
//! ## Hardware integration
//!
//! Hardware mapping may eventually determine that the awaited computation
//! requires a particular resource or backend.
//!
//! That information must be derived downstream from semantic meaning and
//! capabilities.
//!
//! This file remains hardware-independent.
//!
//! ## No hard-coded limits
//!
//! No constants such as:
//!
//! ```text
//! MAX_AWAITS
//! MAX_TASKS
//! MAX_FUTURES
//! MAX_THREADS
//! MAX_DEVICES
//! MAX_QUBITS
//! ```
//!
//! may exist here.
//!
//! Compiler safety/resource limits belong in explicit configurable policy
//! layers.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - executes no program;
//! - does not dereference raw pointers;
//! - performs no unchecked indexing;
//! - does not recursively traverse children;
//! - does not access global mutable state;
//! - does not perform backend communication;
//! - does not allocate based on machine size.
//!
//! AST input is untrusted compiler input, so graph-wide malformed-reference and
//! resource-limit protection belongs to the structural AST validation layer.
//!
//! ## Thread safety
//!
//! No global mutable state is used.
//!
//! Immutable instances can be shared between compiler phases when the
//! constituent AST types satisfy the relevant `Send`/`Sync` requirements.
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
//! =============================================================================
//! Implementation
//! =============================================================================
//
// Rust 1.97 / Rust 1.97.1
// Edition 2021
// No unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Local structural schema version for await expressions.
///
/// This version identifies the structural contract of this node. It is
/// intentionally independent of the Zamani language version and the global AST
/// serialization schema.
pub const AWAIT_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level name for this AST construct.
pub const AWAIT_EXPRESSION_KIND_NAME: &str = "zamani:await-expression";

/// Convenient result type for await-expression operations.
pub type AwaitExpressionResult<T> = Result<T, AwaitExpressionError>;

/// Errors detectable from the local structure of an await expression.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AwaitExpressionError {
    /// The embedded common AST node has the wrong node kind.
    InvalidNodeKind {
        /// Actual node kind found in the common node.
        actual: NodeKind,
    },

    /// The await expression directly refers to itself.
    SelfReferentialExpression,
}

impl fmt::Display for AwaitExpressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "await expression has invalid AST node kind: {actual}"
                )
            }

            Self::SelfReferentialExpression => {
                formatter.write_str(
                    "await expression cannot directly reference itself",
                )
            }
        }
    }
}

impl std::error::Error for AwaitExpressionError {}

/// A source-level Zamani `await` expression.
///
/// Conceptually:
///
/// ```text
/// await expression
/// ```
///
/// The awaited expression is represented by [`NodeId`] rather than recursively
/// embedding another AST node.
///
/// This keeps the node compatible with the canonical graph-oriented AST.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AwaitExpression {
    /// Common source-level AST identity, classification, span and metadata.
    node: Node,

    /// AST identity of the expression being awaited.
    expression: NodeId,
}

/// Short alias for [`AwaitExpression`].
pub type Await = AwaitExpression;

impl AwaitExpression {
    /// Creates an await expression with the canonical await node kind.
    ///
    /// This constructor performs no semantic validation.
    ///
    /// The caller supplies the `NodeId` because AST identity belongs to the AST
    /// construction layer and must never be manufactured implicitly by this
    /// node.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        expression: NodeId,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::AwaitExpression),
            span,
            metadata,
        );

        Self { node, expression }
    }

    /// Creates an await expression from an existing common [`Node`].
    ///
    /// The supplied node is preserved exactly. If its node kind is incorrect,
    /// [`Self::validate_structure`] reports the problem.
    #[must_use]
    pub fn from_node(node: Node, expression: NodeId) -> Self {
        Self { node, expression }
    }

    /// Returns the common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns this node's source span.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns this node's metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable access to this node's metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Replaces this node's metadata.
    ///
    /// The returned value is the previous metadata.
    #[inline]
    pub fn replace_metadata(
        &mut self,
        metadata: NodeMetadata,
    ) -> NodeMetadata {
        self.node.replace_metadata(metadata)
    }

    /// Returns the AST identity of the awaited expression.
    #[inline]
    #[must_use]
    pub fn expression(&self) -> NodeId {
        self.expression
    }

    /// Replaces the awaited expression reference.
    ///
    /// This operation changes only the source-level AST relationship. It does
    /// not resolve or execute the referenced expression.
    ///
    /// The returned value is the previous child ID.
    #[inline]
    pub fn replace_expression(&mut self, expression: NodeId) -> NodeId {
        std::mem::replace(&mut self.expression, expression)
    }

    /// Sets the awaited expression reference.
    #[inline]
    pub fn set_expression(&mut self, expression: NodeId) {
        self.expression = expression;
    }

    /// Consumes this node and returns its common [`Node`] header.
    #[must_use]
    pub fn into_node(self) -> Node {
        self.node
    }

    /// Consumes this node and returns the awaited expression's [`NodeId`].
    #[must_use]
    pub fn into_expression(self) -> NodeId {
        self.expression
    }

    /// Consumes this node and returns `(Node, expression)`.
    #[must_use]
    pub fn into_parts(self) -> (Node, NodeId) {
        (self.node, self.expression)
    }

    /// Returns the canonical native node kind.
    #[inline]
    #[must_use]
    pub const fn expected_node_kind() -> NodeKind {
        NodeKind::core(CoreNodeKind::AwaitExpression)
    }

    /// Returns the number of direct AST children.
    ///
    /// An await expression always has exactly one child.
    #[inline]
    #[must_use]
    pub const fn child_count(&self) -> usize {
        1
    }

    /// Returns the awaited expression's node ID.
    ///
    /// This is an alias useful to generic AST tooling.
    #[inline]
    #[must_use]
    pub const fn child(&self) -> NodeId {
        self.expression
    }

    /// Returns the direct AST children in deterministic source order.
    ///
    /// The iterator:
    ///
    /// - performs no allocation;
    /// - performs no recursive traversal;
    /// - yields exactly one child.
    #[inline]
    pub fn child_node_ids(
        &self,
    ) -> impl Iterator<Item = NodeId> + '_ {
        core::iter::once(self.expression)
    }

    /// Returns the direct children of this node.
    ///
    /// This is an alias for [`Self::child_node_ids`].
    #[inline]
    pub fn children(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.child_node_ids()
    }

    /// Validates local structural invariants.
    ///
    /// This method deliberately does not inspect the global AST graph.
    ///
    /// It therefore cannot determine whether `expression` actually resolves to
    /// an existing AST node.
    ///
    /// It only validates invariants that can be determined from this node
    /// alone.
    pub fn validate_structure(&self) -> AwaitExpressionResult<()> {
        let expected = Self::expected_node_kind();

        if self.node.kind() != &expected {
            return Err(AwaitExpressionError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        if self.expression == self.id() {
            return Err(
                AwaitExpressionError::SelfReferentialExpression,
            );
        }

        Ok(())
    }

    /// Returns whether this node satisfies its local structural invariants.
    #[inline]
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        self.validate_structure().is_ok()
    }

    /// Returns the local schema version of this node contract.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        AWAIT_EXPRESSION_SCHEMA_VERSION
    }

    /// Returns the stable source-level kind name.
    #[inline]
    #[must_use]
    pub const fn kind_name() -> &'static str {
        AWAIT_EXPRESSION_KIND_NAME
    }
}

impl AstNode for AwaitExpression {
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

impl fmt::Display for AwaitExpression {
    /// Formats the structural form of the node.
    ///
    /// Because this node stores a `NodeId` rather than the concrete child AST,
    /// the display representation intentionally reports the child identity
    /// instead of pretending to reconstruct source text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "await <node {:?}>",
            self.expression
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::source::source_id::SourceId;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn span() -> Span {
        Span::new(SourceId::new(1), 0, 8)
    }

    fn metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    fn await_expression(
        id: u64,
        expression: u64,
    ) -> AwaitExpression {
        AwaitExpression::new(
            node_id(id),
            span(),
            metadata(),
            node_id(expression),
        )
    }

    #[test]
    fn constructs_valid_await_expression() {
        let expression = await_expression(1, 2);

        assert_eq!(expression.id(), node_id(1));
        assert_eq!(expression.expression(), node_id(2));
        assert_eq!(expression.child_count(), 1);
    }

    #[test]
    fn uses_canonical_node_kind() {
        let expression = await_expression(1, 2);

        assert_eq!(
            expression.node().kind(),
            &NodeKind::core(CoreNodeKind::AwaitExpression)
        );
    }

    #[test]
    fn expected_node_kind_is_canonical() {
        assert_eq!(
            AwaitExpression::expected_node_kind(),
            NodeKind::core(CoreNodeKind::AwaitExpression)
        );
    }

    #[test]
    fn child_is_the_awaited_expression() {
        let expression = await_expression(10, 20);

        assert_eq!(expression.child(), node_id(20));
        assert_eq!(expression.expression(), node_id(20));
    }

    #[test]
    fn child_iteration_is_deterministic() {
        let expression = await_expression(10, 20);

        let children: Vec<NodeId> =
            expression.child_node_ids().collect();

        assert_eq!(children, vec![node_id(20)]);
    }

    #[test]
    fn child_iterator_has_exactly_one_element() {
        let expression = await_expression(10, 20);

        let mut children = expression.child_node_ids();

        assert_eq!(children.next(), Some(node_id(20)));
        assert_eq!(children.next(), None);
    }

    #[test]
    fn valid_structure_passes_validation() {
        let expression = await_expression(1, 2);

        assert_eq!(expression.validate_structure(), Ok(()));
        assert!(expression.is_structurally_valid());
    }

    #[test]
    fn self_reference_is_rejected() {
        let expression = await_expression(10, 10);

        assert_eq!(
            expression.validate_structure(),
            Err(AwaitExpressionError::SelfReferentialExpression)
        );
    }

    #[test]
    fn wrong_node_kind_is_rejected() {
        let node = Node::new(
            node_id(1),
            NodeKind::core(CoreNodeKind::Identifier),
            span(),
            metadata(),
        );

        let expression =
            AwaitExpression::from_node(node, node_id(2));

        assert!(matches!(
            expression.validate_structure(),
            Err(AwaitExpressionError::InvalidNodeKind { .. })
        ));
    }

    #[test]
    fn node_identity_is_preserved() {
        let expression = await_expression(123, 456);

        assert_eq!(expression.id(), node_id(123));
        assert_eq!(expression.expression(), node_id(456));
    }

    #[test]
    fn source_span_is_preserved() {
        let source_span = Span::new(SourceId::new(7), 11, 19);

        let expression = AwaitExpression::new(
            node_id(1),
            source_span.clone(),
            metadata(),
            node_id(2),
        );

        assert_eq!(expression.span(), &source_span);
    }

    #[test]
    fn metadata_is_preserved() {
        let metadata = NodeMetadata::default();

        let expression = AwaitExpression::new(
            node_id(1),
            span(),
            metadata.clone(),
            node_id(2),
        );

        assert_eq!(expression.metadata(), &metadata);
    }

    #[test]
    fn expression_reference_can_be_replaced() {
        let mut expression = await_expression(1, 2);

        let previous =
            expression.replace_expression(node_id(3));

        assert_eq!(previous, node_id(2));
        assert_eq!(expression.expression(), node_id(3));
    }

    #[test]
    fn expression_reference_can_be_set() {
        let mut expression = await_expression(1, 2);

        expression.set_expression(node_id(4));

        assert_eq!(expression.expression(), node_id(4));
    }

    #[test]
    fn into_parts_preserves_structure() {
        let expression = await_expression(1, 2);

        let (node, child) = expression.into_parts();

        assert_eq!(node.id(), node_id(1));
        assert_eq!(
            node.kind(),
            &NodeKind::core(CoreNodeKind::AwaitExpression)
        );
        assert_eq!(child, node_id(2));
    }

    #[test]
    fn schema_information_is_stable() {
        assert_eq!(
            AwaitExpression::schema_version(),
            AWAIT_EXPRESSION_SCHEMA_VERSION
        );

        assert_eq!(
            AwaitExpression::kind_name(),
            "zamani:await-expression"
        );
    }

    #[test]
    fn ast_node_trait_exposes_common_node() {
        let expression = await_expression(1, 2);

        let node: &dyn AstNode = &expression;

        assert_eq!(node.id(), node_id(1));
        assert_eq!(
            node.kind(),
            &NodeKind::core(CoreNodeKind::AwaitExpression)
        );
    }

    #[test]
    fn serde_round_trip_preserves_structure() {
        let expression = await_expression(100, 200);

        let encoded =
            serde_json::to_string(&expression)
                .expect("await expression must serialize");

        let decoded: AwaitExpression =
            serde_json::from_str(&encoded)
                .expect("await expression must deserialize");

        assert_eq!(expression, decoded);
    }
}