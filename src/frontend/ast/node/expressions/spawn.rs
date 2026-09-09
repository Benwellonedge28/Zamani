//! # Zamani Native AST — Spawn Expression
//!
//! Production-ready source-level representation of a `spawn` expression.
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
//! SpawnExpression                 ← this module
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
//!     ├── distributed IR
//!     ├── accelerator IR
//!     └── future domain IRs
//!     │
//!     ▼
//! Target lowering / execution
//! ```
//!
//! ## Responsibility
//!
//! This module owns only the source-level structure of a spawn expression.
//!
//! A spawn expression consists of:
//!
//! ```text
//! SpawnExpression
//! ├── Node
//! └── expression: NodeId
//! ```
//!
//! The referenced expression is owned by the canonical AST graph.
//!
//! This node therefore stores the child's stable `NodeId` rather than
//! recursively embedding another AST node.
//!
//! ## Semantic boundary
//!
//! `spawn` describes source-level execution intent.
//!
//! It does NOT describe the mechanism by which the spawned computation will
//! eventually execute.
//!
//! This module must never contain:
//!
//! - executor objects;
//! - runtime task handles;
//! - futures;
//! - worker threads;
//! - thread IDs;
//! - process IDs;
//! - queues;
//! - event loops;
//! - reactor state;
//! - polling state;
//! - wake-up state;
//! - scheduler state;
//! - CPU affinity;
//! - GPU streams;
//! - QPU queues;
//! - distributed workers;
//! - network transports;
//! - backend jobs;
//! - hardware identifiers;
//! - quantum topology;
//! - physical qubits;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - ZUIR values.
//!
//! Those concerns belong to later compilation/runtime layers.
//!
//! ## POCO-REAF
//!
//! The purpose of this representation is to preserve the programmer's intent
//! independently of the eventual execution mechanism.
//!
//! The same source-level spawn expression may eventually be lowered to:
//!
//! - synchronous execution when spawning provides no observable distinction;
//! - cooperative concurrency;
//! - preemptive concurrency;
//! - an event-driven runtime;
//! - distributed execution;
//! - accelerator execution;
//! - classical/quantum orchestration;
//! - another future execution model.
//!
//! The AST does not choose among those implementations.
//!
//! This preserves:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF).
//!
//! ## Quantum compatibility
//!
//! A spawned expression may eventually contain or invoke:
//!
//! - classical computation;
//! - quantum computation;
//! - measurement/control feedback;
//! - hybrid computation;
//! - distributed computation;
//! - accelerator computation;
//! - future computational domains.
//!
//! None of those domain-specific semantics are stored here.
//!
//! In particular, this node does not contain:
//!
//! - qubit counts;
//! - quantum register widths;
//! - physical qubit identifiers;
//! - QPU identifiers;
//! - gate sets;
//! - topology;
//! - routing decisions;
//! - scheduling decisions;
//! - calibration;
//! - QEC implementation;
//! - resilience implementation.
//!
//! Those decisions are derived downstream from semantic meaning, capabilities,
//! resources, target constraints, and available hardware.
//!
//! ## Graph-oriented representation
//!
//! The repository's native AST uses `NodeId` references for child nodes.
//!
//! This is important for scalability because it prevents each expression from
//! recursively owning its complete subtree.
//!
//! ```text
//! AST graph
//!
//!       SpawnExpression
//!             │
//!             │ NodeId
//!             ▼
//!       child expression
//! ```
//!
//! The AST graph/traversal subsystem owns graph resolution and graph-wide
//! validation.
//!
//! This module performs only local validation.
//!
//! ## Structural validation boundary
//!
//! [`SpawnExpression::validate_structure`] checks:
//!
//! 1. the embedded node has `CoreNodeKind::SpawnExpression`;
//! 2. the child is not the spawn node itself.
//!
//! It deliberately does not determine whether the child `NodeId` resolves to
//! an existing node. That requires access to the complete AST graph.
//!
//! It also does not perform semantic validation such as:
//!
//! - whether the expression is spawnable;
//! - whether spawning is legal in the enclosing function;
//! - whether ownership/captures are valid;
//! - whether the expression is asynchronous;
//! - whether the resulting type is valid;
//! - whether concurrency effects are permitted;
//! - whether the target supports concurrency;
//! - whether the target has sufficient resources.
//!
//! Those checks belong to semantic analysis and later compilation stages.
//!
//! ## Cycle handling
//!
//! Direct self-reference is rejected:
//!
//! ```text
//! SpawnExpression(id = N)
//!     └── expression = N
//! ```
//!
//! Longer cycles are intentionally not checked here.
//!
//! Graph-wide cycle detection belongs to the AST graph validation layer because
//! it requires access to all nodes.
//!
//! ## Deterministic traversal
//!
//! A spawn expression has exactly one direct child:
//!
//! ```text
//! expression
//! ```
//!
//! [`SpawnExpression::child_node_ids`] returns that child in deterministic
//! source-structural order.
//!
//! No allocation is required for child enumeration.
//!
//! ## Scalability
//!
//! This module introduces no artificial limits on:
//!
//! - number of spawn expressions;
//! - number of AST nodes;
//! - number of nested expressions;
//! - number of spawned computations;
//! - number of machines;
//! - number of workers;
//! - number of devices;
//! - number of quantum resources;
//! - number of execution targets.
//!
//! There is no `MAX_SPAWNS`, `MAX_TASKS`, `MAX_THREADS`, `MAX_DEVICES`,
//! `MAX_QUBITS`, or similar language-level constant.
//!
//! "Infinity" in POCO-REAF means that this node introduces no artificial
//! computational-resource ceiling. Actual compilation and execution remain
//! bounded by available memory, address space, storage, compiler policy, and
//! target capabilities.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - executes no program;
//! - dereferences no raw pointers;
//! - performs no unchecked indexing;
//! - accesses no global mutable state;
//! - does not recursively traverse the AST;
//! - does not communicate with backends;
//! - does not inspect hardware;
//! - does not allocate based on program nesting depth.
//!
//! AST data must be considered untrusted compiler input.
//!
//! Graph-wide resource limits and malformed-reference protection belong to the
//! configurable AST validation policy.
//!
//! ## Serialization
//!
//! The node derives Serde serialization and deserialization.
//!
//! The repository-wide AST serialization subsystem owns the global schema and
//! compatibility policy.
//!
//! This file deliberately does not introduce a competing serialization format.
//!
//! Only source-level AST structure is serialized.
//!
//! No runtime task state, executor handle, queue, worker, hardware resource,
//! pointer, or backend state is serialized.
//!
//! ## Determinism
//!
//! The representation contains only deterministic source-level information:
//!
//! - node identity;
//! - node kind;
//! - source span;
//! - metadata;
//! - child identity.
//!
//! Child enumeration always produces exactly one child in the same order.
//!
//! No hash-map iteration, global counter, timestamp, random value, memory
//! address, or execution state is involved.
//!
//! ## Incremental compilation
//!
//! Later compiler phases may maintain side tables keyed by `NodeId`, for example:
//!
//! ```text
//! NodeId → resolved symbol
//! NodeId → resolved type
//! NodeId → effects
//! NodeId → capabilities
//! NodeId → resource requirements
//! NodeId → concurrency semantics
//! NodeId → ZUIR value
//! ```
//!
//! Those values are intentionally not embedded into this AST node.
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
//! - Rust standard-library facilities.
//!
//! It must never depend on:
//!
//! - lexer token types;
//! - parser implementation;
//! - semantic analysis;
//! - symbol tables;
//! - semantic types;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
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
//! Supplies the canonical [`Node`] container and [`AstNode`] trait.
//!
//! `Node` owns:
//!
//! - `NodeId`;
//! - `NodeKind`;
//! - `Span`;
//! - `NodeMetadata`.
//!
//! ### `node_id.rs`
//!
//! Supplies stable AST identity.
//!
//! The current repository intentionally does not use a `NodeId::INVALID`
//! sentinel. A `NodeId` is constructed through its checked constructor.
//!
//! Therefore this module does not invent or depend on an invalid sentinel.
//!
//! ### `node_kind.rs`
//!
//! The repository already provides:
//!
//! ```text
//! CoreNodeKind::SpawnExpression
//! ```
//!
//! This implementation uses that canonical classification.
//!
//! It does not create a second node-kind enum.
//!
//! ### `expressions/expression.rs`
//!
//! The expression aggregate is the canonical dispatch representation.
//!
//! The corresponding source-level form is expected to remain structurally
//! equivalent to:
//!
//! ```text
//! ExpressionKind::Spawn { expression: NodeId }
//! ```
//!
//! This concrete node does not define another expression enum.
//!
//! ### `expressions/mod.rs`
//!
//! The expressions module must expose this file using the Rust raw identifier
//! syntax because `async` is a Rust keyword but `spawn` itself is not:
//!
//! ```text
//! pub mod spawn;
//! ```
//!
//! Where public re-exports are used:
//!
//! ```text
//! pub use self::spawn::SpawnExpression;
//! ```
//!
//! ### Parser
//!
//! The parser is responsible for:
//!
//! 1. recognizing the `spawn` syntax;
//! 2. parsing the spawned expression;
//! 3. allocating a fresh `NodeId`;
//! 4. calculating the complete source span;
//! 5. attaching source metadata;
//! 6. constructing `SpawnExpression` or the canonical
//!    `ExpressionKind::Spawn` representation;
//! 7. inserting the node into the AST graph.
//!
//! The parser must not:
//!
//! - create runtime tasks;
//! - select an executor;
//! - allocate hardware;
//! - select a quantum backend;
//! - perform scheduling;
//! - perform routing;
//! - perform QEC.
//!
//! ### Structural validation
//!
//! The AST validation layer should:
//!
//! 1. call [`SpawnExpression::validate_structure`];
//! 2. verify that the child `NodeId` resolves in the AST graph;
//! 3. perform graph-wide cycle checks;
//! 4. apply configurable resource/depth policies.
//!
//! This division keeps local validation independent from the AST container.
//!
//! ### Semantic analysis
//!
//! Semantic analysis determines:
//!
//! - what the spawned expression denotes;
//! - its type;
//! - ownership/capture requirements;
//! - concurrency effects;
//! - capabilities;
//! - resource requirements;
//! - lifetime semantics;
//! - cancellation semantics;
//! - synchronization semantics;
//! - domain-specific meaning.
//!
//! None of those semantic results belong in this AST node.
//!
//! ### ZUIR
//!
//! The semantic representation of spawning is lowered to ZUIR downstream.
//!
//! This module deliberately does not import ZUIR.
//!
//! ### Quantum compilation
//!
//! If the spawned expression eventually contains quantum computation, downstream
//! compilation may determine:
//!
//! - quantum resource requirements;
//! - classical/quantum synchronization;
//! - execution placement;
//! - measurement feedback;
//! - resource allocation;
//! - routing;
//! - scheduling;
//! - error correction;
//! - backend realization.
//!
//! None of these modify the native AST.
//!
//! ### Runtime
//!
//! Runtime layers may represent a spawned computation as a task, future,
//! coroutine, actor, process, distributed job, or another execution mechanism.
//!
//! Those are runtime concepts, not AST concepts.
//!
//! ## No-reedit integration guarantee
//!
//! The stable public contract of this file consists only of:
//!
//! - common AST node;
//! - source identity;
//! - source span;
//! - source metadata;
//! - one child expression reference;
//! - deterministic child enumeration;
//! - local structural validation.
//!
//! Changes to:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum compilation;
//! - hardware;
//! - scheduling;
//! - routing;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend providers;
//!
//! must not require this file to be reopened.
//!
//! This file should change only when the source-level semantics of the Zamani
//! `spawn` construct itself change.
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

/// Local structural schema version for the spawn-expression contract.
///
/// This is deliberately independent of the global AST schema version and
/// Zamani language version.
pub const SPAWN_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level name for this AST construct.
pub const SPAWN_EXPRESSION_KIND_NAME: &str = "zamani:spawn-expression";

/// Convenient result type for spawn-expression operations.
pub type SpawnExpressionResult<T> = Result<T, SpawnExpressionError>;

/// Errors detectable from the local structure of a spawn expression.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SpawnExpressionError {
    /// The common AST node has the wrong node kind.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// The spawned expression directly refers to this spawn node.
    SelfReferentialExpression,
}

impl fmt::Display for SpawnExpressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "spawn expression has invalid AST node kind: {actual}"
                )
            }

            Self::SelfReferentialExpression => {
                formatter.write_str(
                    "spawn expression cannot directly reference itself",
                )
            }
        }
    }
}

impl std::error::Error for SpawnExpressionError {}

/// A source-level Zamani `spawn` expression.
///
/// Conceptually:
///
/// ```text
/// spawn expression
/// ```
///
/// The expression being spawned is represented by [`NodeId`] rather than being
/// recursively embedded.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpawnExpression {
    /// Common source-level AST identity, classification, span and metadata.
    node: Node,

    /// AST identity of the expression being spawned.
    expression: NodeId,
}

/// Short alias for [`SpawnExpression`].
pub type Spawn = SpawnExpression;

impl SpawnExpression {
    /// Creates a spawn expression with the canonical node kind.
    ///
    /// This constructor performs no semantic validation.
    ///
    /// The caller supplies the node identity because identity allocation is
    /// owned by the AST construction layer.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        expression: NodeId,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::SpawnExpression),
            span,
            metadata,
        );

        Self { node, expression }
    }

    /// Creates a spawn expression from an existing common [`Node`].
    ///
    /// The supplied node is preserved exactly.
    ///
    /// If its kind is not `CoreNodeKind::SpawnExpression`,
    /// [`Self::validate_structure`] reports the mismatch.
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

    /// Returns the stable AST node identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the source span of this spawn expression.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns this node's source-level metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable access to source-level metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Replaces this node's metadata.
    ///
    /// Returns the previous metadata.
    #[inline]
    pub fn replace_metadata(
        &mut self,
        metadata: NodeMetadata,
    ) -> NodeMetadata {
        self.node.replace_metadata(metadata)
    }

    /// Returns the AST identity of the spawned expression.
    #[inline]
    #[must_use]
    pub fn expression(&self) -> NodeId {
        self.expression
    }

    /// Replaces the spawned-expression reference.
    ///
    /// This changes only the source-level AST relationship.
    ///
    /// It does not resolve, execute, schedule, or otherwise interpret the
    /// referenced expression.
    ///
    /// Returns the previous child ID.
    #[inline]
    pub fn replace_expression(&mut self, expression: NodeId) -> NodeId {
        std::mem::replace(&mut self.expression, expression)
    }

    /// Sets the spawned-expression reference.
    #[inline]
    pub fn set_expression(&mut self, expression: NodeId) {
        self.expression = expression;
    }

    /// Consumes this node and returns its common [`Node`] header.
    #[must_use]
    pub fn into_node(self) -> Node {
        self.node
    }

    /// Consumes this node and returns the spawned expression's [`NodeId`].
    #[must_use]
    pub fn into_expression(self) -> NodeId {
        self.expression
    }

    /// Consumes this node and returns `(Node, expression)`.
    #[must_use]
    pub fn into_parts(self) -> (Node, NodeId) {
        (self.node, self.expression)
    }

    /// Returns the canonical node kind expected by this AST node.
    #[inline]
    #[must_use]
    pub const fn expected_node_kind() -> NodeKind {
        NodeKind::core(CoreNodeKind::SpawnExpression)
    }

    /// Returns the number of direct AST children.
    ///
    /// A spawn expression has exactly one child by source-language structure.
    #[inline]
    #[must_use]
    pub const fn child_count(&self) -> usize {
        1
    }

    /// Returns the spawned expression's node ID.
    ///
    /// This is a generic child-access alias useful to AST tooling.
    #[inline]
    #[must_use]
    pub const fn child(&self) -> NodeId {
        self.expression
    }

    /// Returns the direct child node IDs in deterministic source order.
    ///
    /// No allocation or recursive traversal occurs.
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
    /// This method intentionally does not inspect the complete AST graph.
    ///
    /// Therefore it does not determine whether `expression` actually resolves
    /// to an existing AST node.
    ///
    /// The AST graph validator owns that responsibility.
    pub fn validate_structure(&self) -> SpawnExpressionResult<()> {
        let expected = Self::expected_node_kind();

        if self.node.kind() != &expected {
            return Err(SpawnExpressionError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        if self.expression == self.id() {
            return Err(
                SpawnExpressionError::SelfReferentialExpression,
            );
        }

        Ok(())
    }

    /// Returns whether the node satisfies its local structural invariants.
    #[inline]
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        self.validate_structure().is_ok()
    }

    /// Returns the local structural schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        SPAWN_EXPRESSION_SCHEMA_VERSION
    }

    /// Returns the stable source-level construct name.
    #[inline]
    #[must_use]
    pub const fn kind_name() -> &'static str {
        SPAWN_EXPRESSION_KIND_NAME
    }
}

impl AstNode for SpawnExpression {
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

impl fmt::Display for SpawnExpression {
    /// Formats the structural representation of the node.
    ///
    /// The child is represented by its `NodeId`; this method deliberately does
    /// not pretend to reconstruct source text from the local node alone.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "spawn <node {:?}>",
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
        NodeId::new(value).expect("test NodeId must be non-zero")
    }

    fn span() -> Span {
        Span::new(SourceId::new(1), 0, 10)
    }

    fn metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    fn spawn_expression(
        id: u64,
        expression: u64,
    ) -> SpawnExpression {
        SpawnExpression::new(
            node_id(id),
            span(),
            metadata(),
            node_id(expression),
        )
    }

    #[test]
    fn constructs_valid_spawn_expression() {
        let expression = spawn_expression(1, 2);

        assert_eq!(expression.id(), node_id(1));
        assert_eq!(expression.expression(), node_id(2));
        assert_eq!(expression.child_count(), 1);
    }

    #[test]
    fn uses_canonical_node_kind() {
        let expression = spawn_expression(1, 2);

        assert_eq!(
            expression.node().kind(),
            &NodeKind::core(CoreNodeKind::SpawnExpression)
        );
    }

    #[test]
    fn expected_node_kind_is_canonical() {
        assert_eq!(
            SpawnExpression::expected_node_kind(),
            NodeKind::core(CoreNodeKind::SpawnExpression)
        );
    }

    #[test]
    fn child_is_the_spawned_expression() {
        let expression = spawn_expression(10, 20);

        assert_eq!(expression.child(), node_id(20));
        assert_eq!(expression.expression(), node_id(20));
    }

    #[test]
    fn child_iteration_is_deterministic() {
        let expression = spawn_expression(10, 20);

        let children: Vec<NodeId> =
            expression.child_node_ids().collect();

        assert_eq!(children, vec![node_id(20)]);
    }

    #[test]
    fn child_iterator_has_exactly_one_element() {
        let expression = spawn_expression(10, 20);

        let mut children = expression.child_node_ids();

        assert_eq!(children.next(), Some(node_id(20)));
        assert_eq!(children.next(), None);
    }

    #[test]
    fn valid_structure_passes_validation() {
        let expression = spawn_expression(1, 2);

        assert_eq!(expression.validate_structure(), Ok(()));
        assert!(expression.is_structurally_valid());
    }

    #[test]
    fn self_reference_is_rejected() {
        let expression = spawn_expression(10, 10);

        assert_eq!(
            expression.validate_structure(),
            Err(SpawnExpressionError::SelfReferentialExpression)
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
            SpawnExpression::from_node(node, node_id(2));

        assert!(matches!(
            expression.validate_structure(),
            Err(SpawnExpressionError::InvalidNodeKind { .. })
        ));
    }

    #[test]
    fn node_identity_is_preserved() {
        let expression = spawn_expression(123, 456);

        assert_eq!(expression.id(), node_id(123));
        assert_eq!(expression.expression(), node_id(456));
    }

    #[test]
    fn source_span_is_preserved() {
        let source_span =
            Span::new(SourceId::new(7), 11, 19);

        let expression = SpawnExpression::new(
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

        let expression = SpawnExpression::new(
            node_id(1),
            span(),
            metadata.clone(),
            node_id(2),
        );

        assert_eq!(expression.metadata(), &metadata);
    }

    #[test]
    fn expression_reference_can_be_replaced() {
        let mut expression = spawn_expression(1, 2);

        let previous =
            expression.replace_expression(node_id(3));

        assert_eq!(previous, node_id(2));
        assert_eq!(expression.expression(), node_id(3));
    }

    #[test]
    fn expression_reference_can_be_set() {
        let mut expression = spawn_expression(1, 2);

        expression.set_expression(node_id(4));

        assert_eq!(expression.expression(), node_id(4));
    }

    #[test]
    fn into_parts_preserves_structure() {
        let expression = spawn_expression(1, 2);

        let (node, child) = expression.into_parts();

        assert_eq!(node.id(), node_id(1));
        assert_eq!(
            node.kind(),
            &NodeKind::core(CoreNodeKind::SpawnExpression)
        );
        assert_eq!(child, node_id(2));
    }

    #[test]
    fn schema_information_is_stable() {
        assert_eq!(
            SpawnExpression::schema_version(),
            SPAWN_EXPRESSION_SCHEMA_VERSION
        );

        assert_eq!(
            SpawnExpression::kind_name(),
            "zamani:spawn-expression"
        );
    }

    #[test]
    fn ast_node_trait_exposes_common_node() {
        let expression = spawn_expression(1, 2);

        let node: &dyn AstNode = &expression;

        assert_eq!(node.id(), node_id(1));
        assert_eq!(
            node.kind(),
            &NodeKind::core(CoreNodeKind::SpawnExpression)
        );
    }

    #[test]
    fn serde_round_trip_preserves_structure() {
        let expression = spawn_expression(100, 200);

        let encoded =
            serde_json::to_string(&expression)
                .expect("spawn expression must serialize");

        let decoded: SpawnExpression =
            serde_json::from_str(&encoded)
                .expect("spawn expression must deserialize");

        assert_eq!(expression, decoded);
    }

    #[test]
    fn display_is_structural_and_deterministic() {
        let expression = spawn_expression(100, 200);

        assert_eq!(
            expression.to_string(),
            "spawn <node NodeId(200)>"
        );
    }
}