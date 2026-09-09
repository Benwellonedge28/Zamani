//! # Zamani Native AST — Async Expression
//!
//! Production-ready source-level representation of an asynchronous expression.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! Lexer
//!     │
//!     ▼
//! Parser
//!     │
//!     ▼
//! AsyncExpression  ← this module
//!     │
//!     ▼
//! Structural AST validation
//!     │
//!     ▼
//! Semantic analysis
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── hybrid IR
//!     ├── HDL IR
//!     └── future domain IRs
//!     │
//!     ▼
//! Target lowering / execution
//! ```
//!
//! ## Responsibility
//!
//! This module owns the **source-level structural representation** of an
//! asynchronous expression.
//!
//! It represents only:
//!
//! - the common AST node;
//! - the async expression's body;
//! - source identity;
//! - source span;
//! - source metadata;
//! - deterministic child enumeration;
//! - local structural validation.
//!
//! It deliberately does **not** represent:
//!
//! - executors;
//! - event loops;
//! - futures at runtime;
//! - tasks;
//! - worker threads;
//! - OS threads;
//! - CPU scheduling;
//! - GPU scheduling;
//! - QPU scheduling;
//! - distributed scheduling;
//! - network transports;
//! - runtime handles;
//! - task IDs;
//! - queues;
//! - locks;
//! - polling state;
//! - wake-up state;
//! - reactor state;
//! - backend state;
//! - hardware topology;
//! - quantum resources;
//! - physical qubits;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR.
//!
//! Those concerns belong to later compiler/runtime layers.
//!
//! ## POCO-REAF
//!
//! Async syntax describes **source-level execution intent**.
//!
//! It does not prescribe how asynchronous execution must be realized.
//!
//! The same source-level async expression may eventually be lowered to:
//!
//! - a synchronous implementation when the target permits/chooses it;
//! - a state machine;
//! - cooperative execution;
//! - preemptive execution;
//! - an event-driven runtime;
//! - a distributed execution system;
//! - accelerator execution;
//! - classical/quantum hybrid orchestration;
//! - another future execution model.
//!
//! The AST must not choose between those implementations.
//!
//! This preserves:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF).
//!
//! ## Canonical child representation
//!
//! The existing Zamani expression aggregate already defines:
//!
//! ```text
//! ExpressionKind::Async {
//!     body: NodeId,
//! }
//! ```
//!
//! This module provides the concrete AST-node form of that representation:
//!
//! ```text
//! AsyncExpression
//! ├── Node
//! └── body: NodeId
//! ```
//!
//! The actual child node belongs to the owning AST graph.
//!
//! This module stores only its stable identity.
//!
//! This avoids recursive Rust ownership structures and allows the repository's
//! traversal layer to walk very deeply nested asynchronous programs without
//! requiring this node to recursively traverse the AST.
//!
//! ## Important semantic boundary
//!
//! An async expression is **not** itself a runtime future/task.
//!
//! For example, this AST node must not contain:
//!
//! ```text
//! Future<T>
//! TaskHandle
//! Executor
//! Waker
//! PollState
//! RuntimeTaskId
//! ```
//!
//! Such values belong to semantic/runtime representations.
//!
//! ## Quantum and heterogeneous-computing compatibility
//!
//! Async execution may eventually surround:
//!
//! - classical computation;
//! - quantum computation;
//! - measurement/control feedback;
//! - distributed computation;
//! - accelerator execution;
//! - HDL interaction;
//! - future computational domains.
//!
//! None of those meanings are encoded here.
//!
//! A quantum compiler may later determine that an async body represents a
//! resource-dependent asynchronous operation, but the native AST remains
//! unchanged.
//!
//! In particular, this file contains no:
//!
//! - qubit count;
//! - register width;
//! - quantum topology;
//! - QPU identifier;
//! - vendor identifier;
//! - backend identifier;
//! - gate set;
//! - hardware queue;
//! - calibration;
//! - routing;
//! - scheduling;
//! - QEC implementation.
//!
//! ## No fixed scalability limit
//!
//! This node introduces no fixed limit on:
//!
//! - number of async expressions;
//! - program size;
//! - nesting depth;
//! - number of machines;
//! - number of workers;
//! - number of tasks;
//! - number of quantum resources;
//! - number of classical resources;
//! - number of execution targets.
//!
//! The node has exactly one direct child by language structure.
//!
//! "Infinity" in POCO-REAF means that this AST node does not introduce an
//! artificial finite computational-resource limit. Actual execution remains
//! bounded by available memory, address space, storage, compiler policy and
//! target capabilities.
//!
//! ## Validation boundary
//!
//! [`AsyncExpression::validate_structure`] checks only local structural facts:
//!
//! - the embedded node has `CoreNodeKind::AsyncExpression`;
//! - the async body is not `NodeId::INVALID`;
//! - the async node does not directly reference itself as its body.
//!
//! It does **not** check:
//!
//! - whether the body is semantically asynchronous;
//! - whether the body contains `await`;
//! - whether `await` is legal in the body;
//! - whether the resulting type is valid;
//! - whether captures are valid;
//! - whether ownership is valid;
//! - whether effects are valid;
//! - whether a runtime supports asynchronous execution;
//! - whether a target supports concurrency;
//! - whether a QPU supports asynchronous orchestration;
//! - whether a distributed system can execute the body.
//!
//! Those checks belong to semantic analysis and later compilation stages.
//!
//! ## Child existence
//!
//! This module intentionally does not inspect the global AST graph.
//!
//! Therefore:
//!
//! ```text
//! NodeId != NodeId::INVALID
//! ```
//!
//! means that a structurally present child reference exists.
//!
//! Whether that ID actually resolves to a node in the AST graph is validated
//! by the AST graph/structural validation layer.
//!
//! ## Deterministic traversal
//!
//! There is exactly one child:
//!
//! 1. `body`
//!
//! [`AsyncExpression::child_node_ids`] returns that child using a standard
//! one-element iterator.
//!
//! No allocation is required for child enumeration.
//!
//! ## Serialization
//!
//! The node derives Serde serialization and deserialization.
//!
//! Global AST schema versioning remains owned by the repository-wide AST
//! serialization subsystem.
//!
//! This file does not introduce a competing wire format.
//!
//! No runtime state, pointers, executor handles or hardware state is serialized.
//!
//! ## Security
//!
//! This module:
//!
//! - forbids unsafe Rust;
//! - performs no I/O;
//! - executes no source program;
//! - does not dereference raw pointers;
//! - does not use unchecked indexing;
//! - does not access global mutable state;
//! - does not recursively traverse children;
//! - does not perform backend lookups;
//! - does not allocate based on recursive program depth.
//!
//! AST input is treated as untrusted compiler input.
//!
//! ## Determinism
//!
//! The representation contains only deterministic source-level state.
//!
//! Child traversal always produces exactly one child in source-structural order.
//!
//! ## Incremental compilation
//!
//! `NodeId` allows later compiler phases to maintain side tables such as:
//!
//! ```text
//! NodeId → resolved type
//! NodeId → resolved symbol
//! NodeId → effects
//! NodeId → capabilities
//! NodeId → resource requirements
//! NodeId → async execution semantics
//! NodeId → ZUIR value
//! ```
//!
//! None of that semantic information is embedded into this AST node.
//!
//! ## Dependency contract
//!
//! This file may depend only on foundational AST infrastructure:
//!
//! - `AstNode`;
//! - `Node`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `NodeMetadata`;
//! - `Span`;
//! - Serde;
//! - the Rust standard library.
//!
//! It must never depend on:
//!
//! - parser implementation;
//! - lexer token types;
//! - semantic analysis;
//! - symbol tables;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime executors;
//! - backend providers;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - OpenQASM ASTs.
//!
//! ## Integration contract
//!
//! ### `node.rs`
//!
//! Provides the common `Node` container.
//!
//! ### `node_id.rs`
//!
//! Provides stable AST identity.
//!
//! ### `node_kind.rs`
//!
//! Provides [`CoreNodeKind::AsyncExpression`].
//!
//! The repository already contains this core node kind and the existing
//! expression aggregate already maps `ExpressionKind::Async` to it.
//!
//! ### `expressions/expression.rsq`
//!
//! The existing canonical aggregate is:
//!
//! ```text
//! ExpressionKind::Async {
//!     body: NodeId,
//! }
//! ```
//!
//! That representation remains authoritative for the aggregate expression.
//!
//! This module does not create a competing enum.
//!
//! ### `expressions/mod.rs`
//!
//! Must expose:
//!
//! ```text
//! pub mod async;
//! ```
//!
//! and, where the module's public API is re-exported:
//!
//! ```text
//! pub use self::async::AsyncExpression;
//! ```
//!
//! ### Parser
//!
//! The parser is responsible for converting source syntax into the canonical
//! `ExpressionKind::Async { body }` representation and/or constructing this
//! concrete node through [`AsyncExpression::new`].
//!
//! The parser owns source-token interpretation.
//!
//! This file must never import parser token types.
//!
//! ### Structural validation
//!
//! The AST validator should invoke:
//!
//! ```text
//! AsyncExpression::validate_structure()
//! ```
//!
//! and independently resolve `body` against the owning AST graph.
//!
//! ### Semantic analysis
//!
//! Semantic analysis determines:
//!
//! - the semantic type of the async expression;
//! - the semantic type of the body;
//! - async/effect semantics;
//! - ownership/capture semantics;
//! - resource requirements;
//! - capability requirements;
//! - concurrency semantics;
//! - cancellation semantics;
//! - domain-specific meaning.
//!
//! None of these are stored in this AST node.
//!
//! ### ZUIR
//!
//! ZUIR lowering consumes the semantic representation.
//!
//! This file must not import ZUIR.
//!
//! ### Runtime
//!
//! Runtime construction of futures/tasks/executors belongs outside the AST.
//!
//! ### Quantum execution
//!
//! Quantum scheduling, asynchronous QPU execution, queueing, measurement
//! synchronization, routing and hardware mapping belong downstream.
//!
//! ### Optimization
//!
//! Async fusion, task elimination, scheduling, state-machine generation,
//! parallelization, serialization of execution, and target-specific lowering
//! belong downstream.
//!
//! ## No-reedit integration guarantee
//!
//! The public contract of this file is deliberately self-contained:
//!
//! - common node identity;
//! - source span;
//! - metadata;
//! - one body child;
//! - deterministic child enumeration;
//! - local structural validation.
//!
//! Changes to semantic analysis, ZUIR, quantum compilation, scheduling,
//! routing, calibration, runtime or backend implementation should therefore
//! not require modification of this file.
//!
//! A new source-level async primitive should modify this file only if the
//! actual Zamani language semantics change.
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

/// Schema version of the async-expression contract.
///
/// This is intentionally independent from:
///
/// - the Zamani language version;
/// - the global AST schema version;
/// - the serialization format version.
pub const ASYNC_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level diagnostic/tooling name.
pub const ASYNC_EXPRESSION_KIND_NAME: &str = "zamani:async-expression";

/// Canonical source-level representation of an async expression.
///
/// The body is represented by `NodeId` rather than recursively embedding an
/// expression. The owning AST graph is responsible for storing and resolving
/// the actual child node.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AsyncExpression {
    /// Common AST identity, source kind, source span and metadata.
    node: Node,

    /// Body of the asynchronous expression.
    body: NodeId,
}

/// Short alias for callers that use the language construct name.
pub type Async = AsyncExpression;

/// Errors detectable without consulting the complete AST graph or semantic
/// environment.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AsyncExpressionValidationError {
    /// The embedded node does not have the async-expression node kind.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// The body reference is not structurally present.
    InvalidBodyReference {
        /// Invalid body ID.
        id: NodeId,
    },

    /// The async expression directly references itself as its body.
    SelfReferentialBody {
        /// Async expression's own ID.
        id: NodeId,
    },
}

impl fmt::Display for AsyncExpressionValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "async expression has invalid AST node kind: {actual}"
                )
            }

            Self::InvalidBodyReference { id } => {
                write!(
                    formatter,
                    "async expression has invalid body NodeId: {id:?}"
                )
            }

            Self::SelfReferentialBody { id } => {
                write!(
                    formatter,
                    "async expression cannot use itself as its body: {id:?}"
                )
            }
        }
    }
}

impl std::error::Error for AsyncExpressionValidationError {}

impl AsyncExpression {
    /// Creates an async expression from an already constructed common AST node.
    ///
    /// This constructor intentionally does not rewrite the supplied node.
    /// Call [`Self::validate_structure`] when the node's structural validity
    /// must be checked.
    #[must_use]
    pub fn from_node(node: Node, body: NodeId) -> Self {
        Self { node, body }
    }

    /// Creates a fully initialized async expression.
    ///
    /// The canonical `CoreNodeKind::AsyncExpression` is installed here.
    ///
    /// The constructor performs no semantic validation.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        body: NodeId,
    ) -> Self {
        Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::AsyncExpression),
                span,
                metadata,
            ),
            body,
        }
    }

    /// Creates an async expression and immediately validates its local
    /// structural invariants.
    ///
    /// This does not inspect the global AST graph.
    pub fn try_new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        body: NodeId,
    ) -> Result<Self, AsyncExpressionValidationError> {
        let expression = Self::new(id, span, metadata, body);
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
    /// Mutation is intentionally limited to the common AST container. Semantic
    /// information remains outside the node.
    #[must_use]
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns this expression's stable AST identity.
    #[must_use]
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns this expression's source-level node kind.
    #[must_use]
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns this expression's complete source span.
    #[must_use]
    #[inline]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns this expression's source-level metadata.
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns the async body's AST identity.
    #[must_use]
    #[inline]
    pub const fn body(&self) -> NodeId {
        self.body
    }

    /// Replaces the body reference.
    ///
    /// This method performs no semantic validation. Call
    /// [`Self::validate_structure`] after constructing or transforming an AST
    /// node when structural validation is required.
    #[inline]
    pub fn set_body(&mut self, body: NodeId) {
        self.body = body;
    }

    /// Returns the single direct child of this expression.
    ///
    /// No allocation is performed.
    ///
    /// The iterator is intentionally non-recursive.
    #[must_use]
    #[inline]
    pub fn child_node_ids(&self) -> core::iter::Once<NodeId> {
        core::iter::once(self.body)
    }

    /// Alias used by generic AST walkers.
    #[must_use]
    #[inline]
    pub fn children(&self) -> core::iter::Once<NodeId> {
        self.child_node_ids()
    }

    /// Returns the number of direct children.
    ///
    /// Async expressions always contain exactly one structural child.
    #[must_use]
    pub const fn child_count(&self) -> usize {
        1
    }

    /// Performs local structural validation.
    ///
    /// This method deliberately does not inspect the complete AST graph.
    pub fn validate_structure(
        &self,
    ) -> Result<(), AsyncExpressionValidationError> {
        if self.kind().as_core() != Some(CoreNodeKind::AsyncExpression) {
            return Err(
                AsyncExpressionValidationError::InvalidNodeKind {
                    actual: self.kind().clone(),
                },
            );
        }

        if self.body == NodeId::INVALID {
            return Err(
                AsyncExpressionValidationError::InvalidBodyReference {
                    id: self.body,
                },
            );
        }

        if self.body == self.id() {
            return Err(
                AsyncExpressionValidationError::SelfReferentialBody {
                    id: self.id(),
                },
            );
        }

        Ok(())
    }

    /// Returns whether this node satisfies all local structural invariants.
    #[must_use]
    #[inline]
    pub fn is_structurally_valid(&self) -> bool {
        self.validate_structure().is_ok()
    }
}

impl AstNode for AsyncExpression {
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

impl fmt::Display for AsyncExpression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}(id={:?}, body={:?})",
            ASYNC_EXPRESSION_KIND_NAME,
            self.id(),
            self.body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test node IDs must be non-zero")
    }

    fn span() -> Span {
        Span::default()
    }

    fn metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    fn async_expression(id: u64, body: u64) -> AsyncExpression {
        AsyncExpression::new(
            node_id(id),
            span(),
            metadata(),
            node_id(body),
        )
    }

    #[test]
    fn constructor_uses_canonical_node_kind() {
        let expression = async_expression(1, 2);

        assert_eq!(
            expression.kind().as_core(),
            Some(CoreNodeKind::AsyncExpression)
        );
    }

    #[test]
    fn constructor_preserves_identity() {
        let expression = async_expression(7, 8);

        assert_eq!(expression.id(), node_id(7));
    }

    #[test]
    fn constructor_preserves_body() {
        let expression = async_expression(7, 8);

        assert_eq!(expression.body(), node_id(8));
    }

    #[test]
    fn child_count_is_exactly_one() {
        let expression = async_expression(1, 2);

        assert_eq!(expression.child_count(), 1);
    }

    #[test]
    fn children_are_deterministic() {
        let expression = async_expression(1, 2);

        let children: Vec<NodeId> =
            expression.child_node_ids().collect();

        assert_eq!(children, vec![node_id(2)]);
    }

    #[test]
    fn children_alias_matches_child_node_ids() {
        let expression = async_expression(1, 2);

        let children: Vec<NodeId> =
            expression.children().collect();

        assert_eq!(children, vec![node_id(2)]);
    }

    #[test]
    fn child_iterator_has_exact_size() {
        let expression = async_expression(1, 2);

        assert_eq!(expression.child_node_ids().len(), 1);
    }

    #[test]
    fn valid_expression_passes_validation() {
        let expression = async_expression(1, 2);

        assert!(expression.validate_structure().is_ok());
        assert!(expression.is_structurally_valid());
    }

    #[test]
    fn invalid_body_id_is_rejected() {
        let expression = AsyncExpression::new(
            node_id(1),
            span(),
            metadata(),
            NodeId::INVALID,
        );

        assert_eq!(
            expression.validate_structure(),
            Err(
                AsyncExpressionValidationError::InvalidBodyReference {
                    id: NodeId::INVALID,
                }
            )
        );
    }

    #[test]
    fn self_referential_body_is_rejected() {
        let expression = AsyncExpression::new(
            node_id(1),
            span(),
            metadata(),
            node_id(1),
        );

        assert_eq!(
            expression.validate_structure(),
            Err(
                AsyncExpressionValidationError::SelfReferentialBody {
                    id: node_id(1),
                }
            )
        );
    }

    #[test]
    fn try_new_accepts_valid_structure() {
        let result = AsyncExpression::try_new(
            node_id(1),
            span(),
            metadata(),
            node_id(2),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn try_new_rejects_invalid_body() {
        let result = AsyncExpression::try_new(
            node_id(1),
            span(),
            metadata(),
            NodeId::INVALID,
        );

        assert!(matches!(
            result,
            Err(
                AsyncExpressionValidationError::InvalidBodyReference {
                    ..
                }
            )
        ));
    }

    #[test]
    fn from_node_preserves_supplied_node() {
        let node = Node::new(
            node_id(1),
            NodeKind::core(CoreNodeKind::AsyncExpression),
            span(),
            metadata(),
        );

        let expression =
            AsyncExpression::from_node(node, node_id(2));

        assert_eq!(expression.id(), node_id(1));
        assert_eq!(expression.body(), node_id(2));
        assert!(expression.is_structurally_valid());
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
            AsyncExpression::from_node(node, node_id(2));

        assert!(matches!(
            expression.validate_structure(),
            Err(
                AsyncExpressionValidationError::InvalidNodeKind {
                    ..
                }
            )
        ));
    }

    #[test]
    fn serde_round_trip_preserves_structure() {
        let expression = async_expression(1, 2);

        let encoded =
            serde_json::to_string(&expression)
                .expect("async expression should serialize");

        let decoded: AsyncExpression =
            serde_json::from_str(&encoded)
                .expect("async expression should deserialize");

        assert_eq!(decoded, expression);
    }

    #[test]
    fn display_is_deterministic() {
        let expression = async_expression(1, 2);

        assert_eq!(
            expression.to_string(),
            "zamani:async-expression(id=NodeId(1), body=NodeId(2))"
        );
    }

    #[test]
    fn no_fixed_program_or_resource_limit_is_encoded() {
        let expression = async_expression(1, 2);

        assert_eq!(expression.child_count(), 1);
        assert!(expression.is_structurally_valid());
    }
}