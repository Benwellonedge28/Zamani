//! # Zamani Frontend AST — While Statement
//!
//! Canonical source-level representation of the native Zamani `while`
//! statement.
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
//!     ├── WhileStatement  ← this module
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
//!     └── future-domain IR
//!
//! ```
//!
//! ## Purpose
//!
//! This module owns the source-level structure of a `while` loop.
//!
//! A while statement consists of:
//!
//! 1. a condition expression;
//! 2. a body statement/block.
//!
//! Both are represented by opaque [`NodeId`] references into the enclosing
//! AST graph.
//!
//! This module intentionally contains no semantic interpretation of the
//! condition or body.
//!
//! ## POCO-REAF
//!
//! A while loop must describe programmer intent rather than a particular
//! machine.
//!
//! Consequently this type contains no:
//!
//! - CPU identifiers;
//! - GPU identifiers;
//! - FPGA identifiers;
//! - QPU identifiers;
//! - qubit counts;
//! - register counts;
//! - hardware topology;
//! - gate set;
//! - instruction set;
//! - vendor;
//! - backend;
//! - scheduler state;
//! - routing information;
//! - calibration;
//! - QEC implementation;
//! - noise model;
//! - execution queue;
//! - runtime state.
//!
//! The same AST representation can therefore be used regardless of whether
//! the eventual computation is executed on a tiny machine, a large
//! heterogeneous system, a simulator, quantum hardware, distributed
//! infrastructure, or a future computational system.
//!
//! "Infinity" in POCO-REAF means that this AST representation imposes no
//! artificial machine-size or loop-iteration semantic limit. Actual compiler
//! and execution limits are external resource policies.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - the canonical structural representation of a native while statement;
//! - its two source-level child references;
//! - local structural validation;
//! - deterministic child enumeration;
//! - conversion to/from the enclosing [`StatementKind`] representation.
//!
//! This module does NOT own:
//!
//! - AST graph storage;
//! - child node storage;
//! - node-ID allocation;
//! - parsing;
//! - name resolution;
//! - type checking;
//! - control-flow analysis;
//! - termination analysis;
//! - optimization;
//! - resource allocation;
//! - quantum compilation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - calibration;
//! - backend selection;
//! - runtime execution.
//!
//! ## Child ownership
//!
//! The loop does not own its condition or body nodes.
//!
//! ```text
//! WhileStatement
//! ├── condition: NodeId
//! └── body: NodeId
//!
//! AST graph
//! ├── Node(condition)
//! └── Node(body)
//! ```
//!
//! This preserves the repository's existing graph-oriented AST design and
//! avoids recursive Rust ownership structures.
//!
//! ## Structural versus semantic validation
//!
//! This module validates only local structural requirements:
//!
//! - the condition reference is valid;
//! - the body reference is valid;
//! - the supplied common [`Node`] has `CoreNodeKind::WhileStatement`.
//!
//! It does NOT determine whether:
//!
//! - the condition is boolean;
//! - the condition terminates;
//! - the body is reachable;
//! - the loop is finite;
//! - the loop consumes quantum resources;
//! - the loop is executable on a particular backend.
//!
//! Those questions belong to semantic analysis and later compilation phases.
//!
//! ## Traversal
//!
//! Child order is deterministic:
//!
//! ```text
//! 0 → condition
//! 1 → body
//! ```
//!
//! No recursive traversal is performed here.
//!
//! General AST traversal belongs to the traversal/visitor subsystem.
//!
//! ## Scalability
//!
//! This representation has constant local structural size regardless of:
//!
//! - number of loop iterations;
//! - number of qubits;
//! - machine size;
//! - number of hardware resources;
//! - body complexity;
//! - program size.
//!
//! The AST does not unroll a loop merely because its eventual execution may
//! contain many iterations.
//!
//! A symbolic or runtime-dependent condition therefore remains one AST
//! condition node rather than being expanded into a machine-sized structure.
//!
//! ## Determinism
//!
//! This module:
//!
//! - does not allocate node IDs;
//! - does not use global state;
//! - does not use wall-clock time;
//! - does not use randomness;
//! - does not depend on hash-map iteration;
//! - preserves deterministic child ordering.
//!
//! ## Security
//!
//! The type contains no `unsafe` code, raw pointers, I/O, execution hooks, or
//! global mutable state.
//!
//! Invalid child references are rejected structurally rather than being
//! dereferenced.
//!
//! Graph-level validation remains responsible for checking whether referenced
//! IDs actually exist.
//!
//! ## Serialization
//!
//! The structure derives Serde serialization and deserialization using the
//! repository's ordinary AST serialization mechanism.
//!
//! Serialization schema/version management remains the responsibility of the
//! AST serialization subsystem.
//!
//! This module does not create a competing serialization protocol.
//!
//! ## Parser contract
//!
//! The parser must:
//!
//! 1. recognize the native Zamani `while` syntax;
//! 2. allocate the statement's [`NodeId`];
//! 3. construct the condition node;
//! 4. construct the body node;
//! 5. construct the common [`Node`] with
//!    `CoreNodeKind::WhileStatement`;
//! 6. construct [`WhileStatement`];
//! 7. insert all nodes into the AST graph.
//!
//! The parser must not perform semantic type checking or hardware mapping.
//!
//! ## Semantic contract
//!
//! Semantic analysis consumes this node and resolves:
//!
//! - condition expression meaning;
//! - condition type;
//! - name references;
//! - effects;
//! - resource dependencies;
//! - control-flow semantics.
//!
//! The semantic layer must not add those values to this AST structure.
//!
//! ## ZUIR contract
//!
//! The semantic representation of this node eventually lowers to the
//! appropriate ZUIR control-flow representation.
//!
//! This module deliberately does not import ZUIR.
//!
//! ## Quantum contract
//!
//! A while loop can control quantum, classical, hybrid, distributed, or future
//! computations without changing this type.
//!
//! For example, the condition might eventually depend on:
//!
//! - classical data;
//! - a measurement result;
//! - a generic resource state;
//! - a hybrid computation;
//! - a future computational domain.
//!
//! The AST does not need to know which.
//!
//! ## No hidden scalability limits
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_ITERATIONS
//! MAX_QUBITS
//! MAX_MACHINES
//! MAX_RESOURCES
//! MAX_LOOP_DEPTH
//! ```
//!
//! in this representation.
//!
//! Compiler-service protection limits belong to externally supplied compiler
//! resource policies.
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
//! **Public API**
//!
//! - [`WhileStatement`]
//! - [`WhileStatement::new`]
//! - [`WhileStatement::condition`]
//! - [`WhileStatement::body`]
//! - [`WhileStatement::child_count`]
//! - [`WhileStatement::child_node_ids`]
//! - [`WhileStatement::validate_structure`]
//! - [`WhileStatement::node`]
//! - [`WhileStatement::node_mut`]
//!
//! **Dependencies**
//!
//! Only foundational AST types and the enclosing statement module are used.
//!
//! **Forbidden dependencies**
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - scheduler;
//! - router;
//! - optimizer;
//! - runtime;
//! - backend APIs.
//!
//! **Completion criterion**
//!
//! This file is complete when a parser can construct a valid while statement,
//! the AST validator can validate it, visitors can enumerate its children,
//! semantic analysis can consume it, and lowering can identify its meaning
//! without requiring hardware-specific fields to be added here.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::node::AstNode;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};

/// Result type for while-statement construction and local validation.
pub type WhileStatementResult<T> = Result<T, WhileStatementError>;

/// Errors that can be detected locally while constructing or validating a
/// while statement.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WhileStatementError {
    /// The supplied common node has the wrong AST node kind.
    InvalidNodeKind {
        /// The actual supplied node kind.
        actual: NodeKind,
    },

    /// The condition reference is invalid.
    InvalidConditionReference,

    /// The body reference is invalid.
    InvalidBodyReference,
}

impl fmt::Display for WhileStatementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "while statement requires node kind \
                     zamani:while-statement, found {actual}"
                )
            }

            Self::InvalidConditionReference => {
                formatter.write_str(
                    "while statement contains an invalid condition node reference",
                )
            }

            Self::InvalidBodyReference => {
                formatter.write_str(
                    "while statement contains an invalid body node reference",
                )
            }
        }
    }
}

impl std::error::Error for WhileStatementError {}

/// Canonical source-level representation of a Zamani `while` statement.
///
/// The condition and body are references into the enclosing AST graph.
///
/// # Invariants
///
/// A valid [`WhileStatement`] satisfies:
///
/// 1. Its common node kind is `CoreNodeKind::WhileStatement`.
/// 2. Its condition [`NodeId`] is non-zero.
/// 3. Its body [`NodeId`] is non-zero.
/// 4. It contains no semantic or target-specific state.
///
/// The existence and actual node kinds of the referenced IDs are checked by
/// graph-level AST validation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WhileStatement {
    /// Common source-level AST node information.
    node: Node,

    /// Condition expression node.
    condition: NodeId,

    /// Loop body node.
    body: NodeId,
}

impl WhileStatement {
    /// Creates a canonical while statement.
    ///
    /// The supplied [`Node`] must already have
    /// [`CoreNodeKind::WhileStatement`].
    ///
    /// This function performs local structural validation only. It does not
    /// inspect the AST graph or perform semantic analysis.
    pub fn new(
        node: Node,
        condition: NodeId,
        body: NodeId,
    ) -> WhileStatementResult<Self> {
        let statement = Self {
            node,
            condition,
            body,
        };

        statement.validate_structure()?;

        Ok(statement)
    }

    /// Returns the common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Changing the node kind may invalidate this statement's invariant.
    /// Call [`Self::validate_structure`] after structural mutation.
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

    /// Returns the source span.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &super::super::source::Span {
        self.node.span()
    }

    /// Returns source-level metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &super::super::metadata::NodeMetadata {
        self.node.metadata()
    }

    /// Returns the condition expression's node ID.
    #[inline]
    #[must_use]
    pub const fn condition(&self) -> NodeId {
        self.condition
    }

    /// Returns the loop body's node ID.
    #[inline]
    #[must_use]
    pub const fn body(&self) -> NodeId {
        self.body
    }

    /// Replaces the condition reference.
    ///
    /// The supplied ID is validated locally before replacement.
    pub fn replace_condition(
        &mut self,
        condition: NodeId,
    ) -> WhileStatementResult<NodeId> {
        validate_node_id(condition, WhileStatementError::InvalidConditionReference)?;

        Ok(core::mem::replace(&mut self.condition, condition))
    }

    /// Replaces the body reference.
    ///
    /// The supplied ID is validated locally before replacement.
    pub fn replace_body(
        &mut self,
        body: NodeId,
    ) -> WhileStatementResult<NodeId> {
        validate_node_id(body, WhileStatementError::InvalidBodyReference)?;

        Ok(core::mem::replace(&mut self.body, body))
    }

    /// Returns the number of direct child references.
    ///
    /// A while statement always has exactly two direct children:
    ///
    /// 1. condition;
    /// 2. body.
    #[inline]
    #[must_use]
    pub const fn child_count(&self) -> usize {
        2
    }

    /// Returns direct children in deterministic structural/source order.
    ///
    /// The ordering is:
    ///
    /// ```text
    /// [condition, body]
    /// ```
    #[must_use]
    pub fn child_node_ids(&self) -> [NodeId; 2] {
        [self.condition, self.body]
    }

    /// Validates local structural invariants.
    ///
    /// This method does not dereference either [`NodeId`].
    ///
    /// Graph existence and child-node-kind validation belongs to the enclosing
    /// AST validation subsystem.
    pub fn validate_structure(&self) -> WhileStatementResult<()> {
        if self.node.kind().as_core() != Some(CoreNodeKind::WhileStatement) {
            return Err(WhileStatementError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        validate_node_id(
            self.condition,
            WhileStatementError::InvalidConditionReference,
        )?;

        validate_node_id(
            self.body,
            WhileStatementError::InvalidBodyReference,
        )?;

        Ok(())
    }

    /// Returns the schema version of this node representation.
    ///
    /// The global AST serialization subsystem remains responsible for
    /// serialization compatibility.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        1
    }
}

impl AstNode for WhileStatement {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

fn validate_node_id(
    id: NodeId,
    error: WhileStatementError,
) -> WhileStatementResult<()> {
    if id.get() == 0 {
        return Err(error);
    }

    Ok(())
}

impl fmt::Display for WhileStatement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "while(condition={}, body={})",
            self.condition.get(),
            self.body.get()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::metadata::NodeMetadata;
    use crate::frontend::ast::source::Span;

    fn node(id: u64) -> Node {
        Node::without_metadata(
            NodeId::new(id),
            NodeKind::core(CoreNodeKind::WhileStatement),
            Span::default(),
        )
    }

    #[test]
    fn constructs_valid_while_statement() {
        let statement = WhileStatement::new(
            node(1),
            NodeId::new(2),
            NodeId::new(3),
        )
        .expect("valid while statement");

        assert_eq!(statement.id().get(), 1);
        assert_eq!(statement.condition().get(), 2);
        assert_eq!(statement.body().get(), 3);
        assert_eq!(statement.child_count(), 2);
    }

    #[test]
    fn preserves_deterministic_child_order() {
        let statement = WhileStatement::new(
            node(1),
            NodeId::new(10),
            NodeId::new(20),
        )
        .expect("valid while statement");

        assert_eq!(
            statement.child_node_ids(),
            [NodeId::new(10), NodeId::new(20)]
        );
    }

    #[test]
    fn rejects_zero_condition_id() {
        let result = WhileStatement::new(
            node(1),
            NodeId::default(),
            NodeId::new(3),
        );

        assert_eq!(
            result,
            Err(WhileStatementError::InvalidConditionReference)
        );
    }

    #[test]
    fn rejects_zero_body_id() {
        let result = WhileStatement::new(
            node(1),
            NodeId::new(2),
            NodeId::default(),
        );

        assert_eq!(
            result,
            Err(WhileStatementError::InvalidBodyReference)
        );
    }

    #[test]
    fn rejects_wrong_node_kind() {
        let wrong_node = Node::without_metadata(
            NodeId::new(1),
            NodeKind::core(CoreNodeKind::Statement),
            Span::default(),
        );

        let result = WhileStatement::new(
            wrong_node,
            NodeId::new(2),
            NodeId::new(3),
        );

        assert!(matches!(
            result,
            Err(WhileStatementError::InvalidNodeKind { .. })
        ));
    }

    #[test]
    fn accepts_arbitrarily_large_node_ids_supported_by_node_id() {
        let statement = WhileStatement::new(
            node(1),
            NodeId::new(u64::MAX),
            NodeId::new(u64::MAX - 1),
        )
        .expect("large IDs are structural, not machine-size limits");

        assert_eq!(statement.condition().get(), u64::MAX);
        assert_eq!(statement.body().get(), u64::MAX - 1);
    }

    #[test]
    fn replacement_preserves_invariants() {
        let mut statement = WhileStatement::new(
            node(1),
            NodeId::new(2),
            NodeId::new(3),
        )
        .expect("valid while statement");

        let previous = statement
            .replace_condition(NodeId::new(20))
            .expect("valid condition");

        assert_eq!(previous, NodeId::new(2));
        assert_eq!(statement.condition(), NodeId::new(20));

        let previous = statement
            .replace_body(NodeId::new(30))
            .expect("valid body");

        assert_eq!(previous, NodeId::new(3));
        assert_eq!(statement.body(), NodeId::new(30));

        statement
            .validate_structure()
            .expect("statement remains structurally valid");
    }

    #[test]
    fn implements_ast_node_contract() {
        let statement = WhileStatement::new(
            node(1),
            NodeId::new(2),
            NodeId::new(3),
        )
        .expect("valid while statement");

        assert_eq!(
            statement.kind().as_core(),
            Some(CoreNodeKind::WhileStatement)
        );
        assert_eq!(statement.id(), NodeId::new(1));
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(WhileStatement::schema_version(), 1);
    }

    #[test]
    fn display_is_deterministic() {
        let statement = WhileStatement::new(
            node(1),
            NodeId::new(2),
            NodeId::new(3),
        )
        .expect("valid while statement");

        assert_eq!(
            statement.to_string(),
            "while(condition=2, body=3)"
        );
    }

    #[test]
    fn metadata_remains_source_level() {
        let metadata = NodeMetadata::default();

        let ast_node = Node::new(
            NodeId::new(1),
            NodeKind::core(CoreNodeKind::WhileStatement),
            Span::default(),
            metadata.clone(),
        );

        let statement = WhileStatement::new(
            ast_node,
            NodeId::new(2),
            NodeId::new(3),
        )
        .expect("valid while statement");

        assert_eq!(statement.metadata(), &metadata);
    }
}