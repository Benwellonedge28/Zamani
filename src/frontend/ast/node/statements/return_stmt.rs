//! # Zamani Frontend AST — `return` Statement
//!
//! Production-ready source-level representation of a Zamani `return`
//! statement.
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
//! ReturnStatement
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
//! ## Purpose
//!
//! This module owns the native Zamani AST representation of the source-level
//! `return` statement.
//!
//! A return statement expresses control-flow termination of the current
//! callable and, optionally, the source-level expression whose value is
//! returned.
//!
//! This node represents programmer intent only. It does not determine:
//!
//! - calling convention;
//! - ABI;
//! - stack layout;
//! - register allocation;
//! - machine instruction selection;
//! - CPU/GPU/QPU behavior;
//! - quantum measurement behavior;
//! - resource placement;
//! - scheduling;
//! - routing;
//! - hardware mapping;
//! - backend execution;
//! - runtime storage;
//! - ZUIR representation.
//!
//! Those concerns belong to later compiler stages.
//!
//! ## POCO-REAF
//!
//! The node intentionally contains no:
//!
//! - machine size;
//! - processor architecture;
//! - register count;
//! - stack size;
//! - qubit count;
//! - quantum topology;
//! - hardware identifier;
//! - vendor;
//! - backend;
//! - instruction set;
//! - execution queue;
//! - physical resource;
//! - calibration;
//! - QEC implementation;
//! - runtime address;
//! - memory address;
//! - fixed resource capacity.
//!
//! Therefore a source program containing:
//!
//! ```text
//! return value;
//! ```
//!
//! has the same AST representation regardless of whether the eventual
//! computation executes on a classical processor, quantum system, simulator,
//! accelerator, distributed system, or future computational substrate.
//!
//! This preserves Zamani's:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF) architectural objective.
//!
//! ## Language contract
//!
//! The repository grammar currently defines:
//!
//! ```text
//! returnStatement: 'return' expression? ';' ;
//! ```
//!
//! Consequently this node supports exactly two source-level structural forms:
//!
//! ```text
//! return;
//! return expression;
//! ```
//!
//! The optional expression is represented by [`NodeId`].
//!
//! The node does not decide whether returning a value is legal in the
//! surrounding callable. That is semantic analysis.
//!
//! ## Child representation
//!
//! Relationships to other AST nodes are represented using [`NodeId`].
//!
//! ```text
//! ReturnStatement
//! └── value ──► Expression AST node (optional)
//! ```
//!
//! The enclosing AST graph owns the referenced expression node.
//!
//! This avoids recursively embedding concrete AST objects and permits the
//! traversal layer to control recursion explicitly.
//!
//! ## Traversal order
//!
//! The optional returned expression is the only direct child.
//!
//! ```text
//! return;
//!     └── no children
//!
//! return expression;
//!     └── value
//! ```
//!
//! The embedded [`Node`] is metadata, not a child.
//!
//! ## Dependency boundary
//!
//! This module may depend only on:
//!
//! - [`Node`];
//! - [`NodeId`];
//! - [`AstNode`];
//! - [`NodeKind`];
//! - [`CoreNodeKind`];
//! - [`NodeMetadata`];
//! - [`Span`];
//! - Serde;
//! - the Rust standard library.
//!
//! It must never depend on:
//!
//! - lexer implementation;
//! - parser implementation;
//! - semantic analysis;
//! - symbol tables;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - OpenQASM AST;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend providers.
//!
//! ## Semantic boundary
//!
//! This node does not determine:
//!
//! - which callable is being returned from;
//! - the declared return type;
//! - whether a value is required;
//! - whether a value is forbidden;
//! - whether the value's type matches the callable;
//! - ownership or borrowing;
//! - resource lifetime;
//! - quantum-resource legality;
//! - effect legality;
//! - capability requirements.
//!
//! These are semantic-analysis responsibilities.
//!
//! ## ZUIR boundary
//!
//! The semantic layer consumes this node and determines the corresponding
//! universal computational semantics.
//!
//! Conceptually:
//!
//! ```text
//! ReturnStatement
//!       │
//!       ▼
//! Semantic analysis
//!       │
//!       ▼
//! return/control-flow semantic
//!       │
//!       ▼
//! ZUIR
//! ```
//!
//! This file intentionally contains no ZUIR dependency.
//!
//! ## Quantum integration
//!
//! The returned expression may ultimately refer to a quantum-related semantic
//! value, measurement result, resource handle, hybrid value, or another
//! domain-specific result.
//!
//! For example:
//!
//! ```text
//! return measure(q);
//! ```
//!
//! remains an ordinary source-level return statement.
//!
//! This node does not know whether `measure(q)` eventually becomes:
//!
//! - a quantum measurement operation;
//! - a simulator operation;
//! - a hybrid execution boundary;
//! - a hardware instruction sequence;
//! - another future computational operation.
//!
//! That meaning is resolved downstream.
//!
//! ## Scalability
//!
//! This node introduces no artificial computational-size limit.
//!
//! There is no:
//!
//! ```text
//! MAX_RETURN_VALUES
//! MAX_RETURN_DEPTH
//! MAX_QUBITS
//! MAX_REGISTERS
//! MAX_MACHINE_SIZE
//! ```
//!
//! The node contains at most one optional child reference because that is a
//! property of the source-language grammar, not a machine limitation.
//!
//! Large programs contain arbitrarily many independently allocated return
//! nodes in the surrounding AST graph, subject only to available resources and
//! explicitly configured compiler safety policies.
//!
//! ## Determinism
//!
//! The node:
//!
//! - does not allocate `NodeId`s itself;
//! - does not use global mutable state;
//! - does not use random state;
//! - does not use memory addresses;
//! - does not use hash-map iteration;
//! - does not contain timestamps;
//! - preserves the source-level optional-value distinction.
//!
//! The caller owns deterministic `NodeId` allocation.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe` code;
//! - forbids unsafe code;
//! - performs no I/O;
//! - executes no source program;
//! - performs no pointer operations;
//! - performs no unchecked indexing;
//! - performs no recursive traversal;
//! - has no global mutable state.
//!
//! Malformed cross-node references are validated by the owning AST graph
//! validator.
//!
//! ## Serialization
//!
//! The node derives Serde serialization.
//!
//! Global AST serialization and schema negotiation remain owned by the AST
//! serialization subsystem.
//!
//! This node contains only source-level information.
//!
//! ## Incremental compilation
//!
//! Semantic information remains outside the node and can therefore be keyed
//! by [`NodeId`].
//!
//! For example:
//!
//! ```text
//! NodeId → enclosing callable
//! NodeId → resolved return type
//! NodeId → control-flow information
//! NodeId → ownership information
//! NodeId → effects
//! NodeId → capabilities
//! NodeId → resource requirements
//! ```
//!
//! This allows later compiler phases to evolve independently from the source
//! AST.
//!
//! ## Parser integration
//!
//! The parser must:
//!
//! 1. recognize the `return` keyword;
//! 2. determine the complete source span;
//! 3. allocate a `NodeId` through the canonical allocator;
//! 4. parse the optional expression;
//! 5. create the expression AST node when present;
//! 6. create this `ReturnStatement`;
//! 7. insert the referenced expression into the owning AST graph.
//!
//! The parser must not pass lexer tokens into this module.
//!
//! ## Structural validation
//!
//! Local validation verifies only invariants owned by this node:
//!
//! - the embedded node has `CoreNodeKind::ReturnStatement`;
//! - an optional value does not reference this return node itself.
//!
//! Graph-level validation must additionally verify that the referenced value
//! exists and is an expression node.
//!
//! Semantic validity is deliberately outside this module.
//!
//! ## No duplicate semantic ownership
//!
//! The repository currently contains a `StatementKind::Return` representation
//! in `statement.rs` with:
//!
//! ```text
//! value: Option<NodeId>
//! ```
//!
//! This file must become the concrete source-level return node.
//!
//! `statement.rs` should eventually delegate to this representation rather
//! than maintaining an independent authoritative return-node structure.
//!
//! This avoids two AST definitions drifting apart.
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
//! - canonical return-statement node;
//! - optional returned-expression reference;
//! - return-specific structural validation;
//! - deterministic child enumeration.
//!
//! **Does not own**
//!
//! - AST graph storage;
//! - node ID allocation;
//! - parsing;
//! - name resolution;
//! - type checking;
//! - control-flow analysis;
//! - ownership analysis;
//! - effects;
//! - capabilities;
//! - resources;
//! - ZUIR;
//! - quantum IR;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware;
//! - runtime.
//!
//! **Inputs**
//!
//! - caller-owned `NodeId`;
//! - source `Span`;
//! - source metadata;
//! - optional expression `NodeId`.
//!
//! **Outputs**
//!
//! - canonical `ReturnStatement`;
//! - deterministic child enumeration;
//! - local structural validation results.
//!
//! **Thread safety**
//!
//! No global mutable state is used. Immutable instances may be shared by
//! read-only compiler phases when their contained types satisfy the required
//! thread-safety bounds.
//!
//! **Ownership**
//!
//! The node owns only the optional `NodeId` reference. The actual expression
//! node remains owned by the enclosing AST graph.
//!
//! **Completion criterion**
//!
//! This file is complete when parser, structural validation, traversal,
//! semantic analysis and lowering can consume the canonical return statement
//! without adding backend-specific fields to this node.
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

/// Independent schema version for the native return-statement contract.
///
/// This is deliberately separate from the Zamani language version and the
/// global AST serialization version.
pub const RETURN_STATEMENT_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identity used by diagnostics and tooling.
pub const RETURN_STATEMENT_KIND_NAME: &str = "zamani:return-statement";

/// Canonical source-level representation of a Zamani `return` statement.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReturnStatement {
    /// Common AST identity, source span, classification and metadata.
    node: Node,

    /// Optional expression returned by the statement.
    ///
    /// `None` represents:
    ///
    /// ```text
    /// return;
    /// ```
    ///
    /// `Some(id)` represents:
    ///
    /// ```text
    /// return expression;
    /// ```
    ///
    /// Whether either form is semantically legal is determined later.
    value: Option<NodeId>,
}

/// Local structural validation failures for [`ReturnStatement`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReturnStatementValidationError {
    /// The embedded node does not have `ReturnStatement` classification.
    InvalidNodeKind {
        /// Actual node classification.
        actual: NodeKind,
    },

    /// The optional return value references the return node itself.
    SelfReference,
}

impl fmt::Display for ReturnStatementValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "return statement has invalid AST node kind: {actual}"
                )
            }

            Self::SelfReference => {
                formatter.write_str(
                    "return statement value references the return statement itself",
                )
            }
        }
    }
}

impl std::error::Error for ReturnStatementValidationError {}

impl ReturnStatement {
    /// Constructs a return statement from an existing canonical [`Node`].
    ///
    /// This constructor does not allocate a new node identity.
    ///
    /// The caller is responsible for ensuring that `node` is intended to
    /// represent `CoreNodeKind::ReturnStatement`.
    #[must_use]
    pub fn from_node(
        node: Node,
        value: Option<NodeId>,
    ) -> Self {
        Self { node, value }
    }

    /// Constructs a canonical return statement.
    ///
    /// The supplied `id` is used unchanged. ID allocation belongs to the
    /// enclosing parser/builder/AST graph.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        value: Option<NodeId>,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::ReturnStatement),
            span,
            metadata,
        );

        Self::from_node(node, value)
    }

    /// Constructs a return statement with default source metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        value: Option<NodeId>,
    ) -> Self {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            value,
        )
    }

    /// Returns the optional returned expression.
    #[must_use]
    pub const fn value(&self) -> Option<NodeId> {
        self.value
    }

    /// Replaces the optional returned expression.
    ///
    /// Passing `None` represents a bare `return;`.
    pub fn set_value(
        &mut self,
        value: Option<NodeId>,
    ) {
        self.value = value;
    }

    /// Returns whether this is a bare `return;` statement.
    #[must_use]
    pub const fn is_bare(&self) -> bool {
        self.value.is_none()
    }

    /// Returns whether this statement returns an expression.
    #[must_use]
    pub const fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Returns the number of direct AST children.
    ///
    /// This is always zero or one according to the source grammar.
    #[must_use]
    pub const fn child_count(&self) -> usize {
        if self.value.is_some() {
            1
        } else {
            0
        }
    }

    /// Returns the optional returned expression as a child reference.
    ///
    /// `None` means that this node has no direct AST children.
    #[must_use]
    pub const fn value_child(&self) -> Option<NodeId> {
        self.value
    }

    /// Returns all direct child node IDs in deterministic source order.
    ///
    /// The returned vector is a caller-owned snapshot.
    ///
    /// For a return statement there is at most one child: the returned
    /// expression.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        match self.value {
            Some(value) => vec![value],
            None => Vec::new(),
        }
    }

    /// Returns the independent schema version for this node contract.
    #[must_use]
    pub const fn schema_version() -> u16 {
        RETURN_STATEMENT_SCHEMA_VERSION
    }

    /// Returns the stable source-level kind name.
    #[must_use]
    pub const fn kind_name() -> &'static str {
        RETURN_STATEMENT_KIND_NAME
    }

    /// Performs local structural validation.
    ///
    /// This function deliberately does not inspect the owning AST graph.
    /// Graph existence and node-category checks belong to the AST validation
    /// subsystem.
    pub fn validate_structure(
        &self,
    ) -> Result<(), ReturnStatementValidationError> {
        let expected =
            NodeKind::core(CoreNodeKind::ReturnStatement);

        if self.node.kind() != &expected {
            return Err(
                ReturnStatementValidationError::InvalidNodeKind {
                    actual: self.node.kind_owned(),
                },
            );
        }

        if self.value == Some(self.node.id()) {
            return Err(
                ReturnStatementValidationError::SelfReference,
            );
        }

        Ok(())
    }

    /// Returns immutable access to the common AST node.
    #[must_use]
    pub fn as_node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Identity and node classification remain encapsulated by [`Node`].
    #[must_use]
    pub fn as_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl AstNode for ReturnStatement {
    /// Returns the embedded canonical AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the embedded canonical AST node.
    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test NodeId must be non-zero")
    }

    #[test]
    fn constructs_bare_return() {
        let statement = ReturnStatement::without_metadata(
            node_id(1),
            Span::default(),
            None,
        );

        assert_eq!(statement.value(), None);
        assert!(statement.is_bare());
        assert!(!statement.has_value());
        assert_eq!(statement.child_count(), 0);
        assert!(statement.child_node_ids().is_empty());
    }

    #[test]
    fn constructs_return_with_value() {
        let statement = ReturnStatement::without_metadata(
            node_id(1),
            Span::default(),
            Some(node_id(2)),
        );

        assert_eq!(statement.value(), Some(node_id(2)));
        assert!(!statement.is_bare());
        assert!(statement.has_value());
        assert_eq!(statement.child_count(), 1);
        assert_eq!(statement.child_node_ids(), vec![node_id(2)]);
    }

    #[test]
    fn uses_return_statement_node_kind() {
        let statement = ReturnStatement::without_metadata(
            node_id(1),
            Span::default(),
            None,
        );

        assert_eq!(
            statement.kind(),
            &NodeKind::core(CoreNodeKind::ReturnStatement)
        );
    }

    #[test]
    fn preserves_node_identity() {
        let statement = ReturnStatement::without_metadata(
            node_id(42),
            Span::default(),
            None,
        );

        assert_eq!(statement.id(), node_id(42));
    }

    #[test]
    fn setter_supports_bare_return() {
        let mut statement = ReturnStatement::without_metadata(
            node_id(1),
            Span::default(),
            Some(node_id(2)),
        );

        statement.set_value(None);

        assert!(statement.is_bare());
        assert_eq!(statement.child_count(), 0);
    }

    #[test]
    fn setter_supports_return_value() {
        let mut statement = ReturnStatement::without_metadata(
            node_id(1),
            Span::default(),
            None,
        );

        statement.set_value(Some(node_id(9)));

        assert_eq!(statement.value(), Some(node_id(9)));
        assert_eq!(statement.child_node_ids(), vec![node_id(9)]);
    }

    #[test]
    fn rejects_self_reference() {
        let statement = ReturnStatement::without_metadata(
            node_id(1),
            Span::default(),
            Some(node_id(1)),
        );

        assert_eq!(
            statement.validate_structure(),
            Err(ReturnStatementValidationError::SelfReference)
        );
    }

    #[test]
    fn validates_correct_node_kind() {
        let statement = ReturnStatement::without_metadata(
            node_id(1),
            Span::default(),
            None,
        );

        assert_eq!(statement.validate_structure(), Ok(()));
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            ReturnStatement::schema_version(),
            RETURN_STATEMENT_SCHEMA_VERSION
        );

        assert_eq!(
            ReturnStatement::kind_name(),
            "zamani:return-statement"
        );
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let statement = ReturnStatement::without_metadata(
            node_id(1),
            Span::default(),
            Some(node_id(2)),
        );

        let encoded =
            serde_json::to_string(&statement)
                .expect("return statement should serialize");

        let decoded: ReturnStatement =
            serde_json::from_str(&encoded)
                .expect("return statement should deserialize");

        assert_eq!(decoded, statement);
        assert_eq!(decoded.child_node_ids(), vec![node_id(2)]);
    }
}