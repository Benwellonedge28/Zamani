//! # Zamani Frontend AST — Continue Statement
//!
//! Canonical source-level representation of a `continue` statement.
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
//!     ├── ContinueStatement
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
//!     ├── classical lowering
//!     ├── quantum lowering
//!     ├── HDL lowering
//!     └── future-domain lowering
//! ```
//!
//! ## Responsibility
//!
//! This file owns the canonical source-level representation of a `continue`
//! statement.
//!
//! A `continue` statement expresses source-level control-flow intent:
//!
//! > transfer control to the continuation point of the nearest applicable
//! > enclosing loop, or to an explicitly labelled applicable construct when
//! > the language provides labelled control flow.
//!
//! This node does **not** determine how that intent is implemented.
//!
//! It therefore contains no:
//!
//! - machine instruction;
//! - CPU branch;
//! - GPU instruction;
//! - quantum operation;
//! - QPU instruction;
//! - physical qubit;
//! - hardware topology;
//! - scheduler state;
//! - routing information;
//! - resource allocation;
//! - QEC information;
//! - backend information;
//! - runtime program counter;
//! - target-specific optimization;
//! - LLVM value;
//! - QIR value;
//! - MLIR operation.
//!
//! Those concerns belong to later compiler layers.
//!
//! ## POCO-REAF
//!
//! This node contains no machine-size assumption.
//!
//! A `continue` statement therefore remains valid whether the enclosing
//! computation eventually executes:
//!
//! - on a tiny machine;
//! - on a large classical machine;
//! - on a simulator;
//! - on a quantum system;
//! - on a heterogeneous system;
//! - on a distributed system;
//! - on future computational hardware.
//!
//! "Infinity" means that this AST node imposes no artificial finite semantic
//! limit. Actual compilation remains bounded only by available resources and
//! explicitly configured compiler/resource-service policies.
//!
//! ## Canonical ownership
//!
//! `ContinueStatement` owns:
//!
//! - the common [`Node`];
//! - the optional source-level label reference.
//!
//! The enclosing AST graph owns the actual child nodes referenced by `NodeId`.
//!
//! This file does not own the AST graph.
//!
//! ## Existing repository integration
//!
//! The repository already defines:
//!
//! - [`Node`];
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`CoreNodeKind::ContinueStatement`];
//! - `StatementKind::Continue`.
//!
//! This file reuses those definitions rather than creating competing
//! representations.
//!
//! The existing statement abstraction currently represents:
//!
//! ```text
//! Continue {
//!     label: Option<NodeId>
//! }
//! ```
//!
//! This dedicated node preserves that information.
//!
//! The current grammar accepts the unlabeled form:
//!
//! ```text
//! continue;
//! ```
//!
//! Therefore parser construction should normally use:
//!
//! ```text
//! ContinueStatement::unlabeled(...)
//! ```
//!
//! Label support remains available to the AST without requiring a future
//! structural redesign if labelled control flow is added to the language.
//!
//! ## Structural versus semantic validation
//!
//! This file performs only local structural validation.
//!
//! It may verify:
//!
//! - the node has `CoreNodeKind::ContinueStatement`;
//! - the optional label reference is structurally present when specified.
//!
//! It must NOT determine whether the statement is actually inside a loop.
//!
//! Whether `continue` is legal at a particular source location is a semantic
//! control-flow question and belongs to semantic analysis.
//!
//! The existing semantic layer already performs this kind of contextual check;
//! the current repository reports `continue outside loop` during semantic
//! processing. The dedicated AST node must remain independent of that logic.
//!
//! ## Child traversal contract
//!
//! Direct children are exposed in deterministic source order.
//!
//! For this node:
//!
//! ```text
//! ContinueStatement
//! └── label [optional]
//! ```
//!
//! The unlabeled form therefore has zero children.
//!
//! No heap allocation is required to enumerate the optional child.
//!
//! ## Parser contract
//!
//! The parser is responsible for:
//!
//! 1. recognizing `continue`;
//! 2. consuming its syntactic terminator;
//! 3. allocating the AST `NodeId`;
//! 4. determining the source span;
//! 5. constructing the common `Node`;
//! 6. constructing this `ContinueStatement`.
//!
//! The parser must not perform:
//!
//! - loop-context semantic validation;
//! - name resolution;
//! - type checking;
//! - resource allocation;
//! - hardware selection;
//! - quantum mapping;
//! - scheduling;
//! - routing;
//! - QEC.
//!
//! ## Semantic contract
//!
//! Semantic analysis consumes this node and determines whether the control
//! transfer is legal in the surrounding control-flow context.
//!
//! The semantic layer may resolve:
//!
//! - whether a target loop exists;
//! - the meaning of a label;
//! - control-flow reachability;
//! - effects and other language semantics.
//!
//! It must not mutate this source AST to attach runtime or backend state.
//!
//! ## ZUIR contract
//!
//! The semantic model lowers the source-level continue operation into the
//! appropriate ZUIR control-flow representation.
//!
//! This file intentionally has no dependency on ZUIR.
//!
//! ## Quantum contract
//!
//! A quantum program may contain ordinary control flow surrounding quantum or
//! hybrid operations. `ContinueStatement` does not need to know whether the
//! loop body contains:
//!
//! - classical computation;
//! - quantum operations;
//! - measurements;
//! - resource operations;
//! - hybrid computation;
//! - distributed operations;
//! - future-domain operations.
//!
//! The semantic and lowering stages determine the eventual representation.
//!
//! ## Hardware contract
//!
//! This node must never encode:
//!
//! - physical qubit IDs;
//! - processor IDs;
//! - device IDs;
//! - coupling graphs;
//! - instruction latency;
//! - scheduling slots;
//! - backend jobs;
//! - target architecture.
//!
//! A control-flow construct remains source-level until later lowering.
//!
//! ## Scalability
//!
//! This implementation contains no:
//!
//! - maximum loop count;
//! - maximum nesting depth;
//! - maximum qubit count;
//! - maximum machine size;
//! - fixed register size;
//! - fixed hardware topology.
//!
//! A `continue` statement itself has constant structural size regardless of
//! how large the enclosing program becomes.
//!
//! The enclosing AST graph and compiler validation infrastructure are
//! responsible for resource-policy enforcement.
//!
//! ## Determinism
//!
//! Construction is deterministic with respect to caller-provided:
//!
//! - `NodeId`;
//! - `NodeKind`;
//! - `Span`;
//! - metadata;
//! - label reference.
//!
//! No global state, randomness, timestamps, memory addresses, or hash-map
//! iteration are used.
//!
//! ## Serialization
//!
//! The node derives Serde serialization through its constituent types.
//!
//! Serialization schema/version policy remains owned by the AST serialization
//! subsystem. This file does not create a competing serialization protocol.
//!
//! ## Security
//!
//! This type:
//!
//! - performs no I/O;
//! - executes no source code;
//! - uses no raw pointers;
//! - uses no `unsafe`;
//! - performs no unchecked indexing;
//! - performs no recursive traversal;
//! - does not allocate based on untrusted dynamic sizes.
//!
//! A malformed `NodeId` is represented by the existing opaque `NodeId` type;
//! graph existence/kind validation belongs to the enclosing AST validator.
//!
//! ## Thread safety
//!
//! The type contains no global mutable state or interior runtime state.
//! Immutable instances can be shared by compiler phases subject to the
//! `Send`/`Sync` properties of their constituent types.
//!
//! ## Rust compatibility
//!
//! This implementation targets:
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
//! - canonical `ContinueStatement` structure;
//! - optional source-level label reference;
//! - local structural validation;
//! - deterministic direct-child enumeration.
//!
//! **Does not own**
//!
//! - AST graph storage;
//! - parser state;
//! - lexical analysis;
//! - semantic loop-context analysis;
//! - symbol resolution;
//! - type checking;
//! - ZUIR;
//! - quantum IR;
//! - scheduling;
//! - routing;
//! - QEC;
//! - hardware;
//! - runtime execution.
//!
//! **Allowed dependencies**
//!
//! - Rust standard library;
//! - Serde;
//! - `Node`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `AstNode`.
//!
//! **Forbidden dependencies**
//!
//! - semantic analysis;
//! - compiler driver;
//! - ZUIR;
//! - quantum IR;
//! - backend APIs;
//! - hardware APIs;
//! - scheduler;
//! - router;
//! - optimizer;
//! - runtime.
//!
//! **Completion criterion**
//!
//! This file is complete when the parser, structural validator, visitors,
//! semantic layer and lowering layer can consume the dedicated node through
//! its stable public contract without adding backend-specific state to it.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use serde::{Deserialize, Serialize};

use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};

/// Schema version for the dedicated continue-statement node.
///
/// This is intentionally independent from the language version and the global
/// AST serialization version.
pub const CONTINUE_STATEMENT_SCHEMA_VERSION: u16 = 1;

/// Errors that can be detected locally while constructing or validating a
/// [`ContinueStatement`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContinueStatementError {
    /// The supplied common node does not identify a continue statement.
    InvalidNodeKind {
        /// The actual node kind supplied by the caller.
        actual: NodeKind,
    },
}

impl core::fmt::Display for ContinueStatementError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "expected `zamani:continue-statement`, found `{actual}`"
                )
            }
        }
    }
}

impl std::error::Error for ContinueStatementError {}

/// Result type for continue-statement construction and local validation.
pub type ContinueStatementResult<T> = Result<T, ContinueStatementError>;

/// Canonical source-level `continue` statement.
///
/// The statement contains no execution state. Its optional `label` is a
/// reference to another source-level AST node and is interpreted by semantic
/// analysis.
///
/// The current Zamani grammar accepts the unlabeled form:
///
/// ```text
/// continue;
/// ```
///
/// The optional label is retained in the AST contract because the existing
/// statement representation already exposes it and because retaining it keeps
/// the node extensible without coupling it to the current grammar's smaller
/// syntax surface.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContinueStatement {
    /// Common AST identity, source span, node kind and metadata.
    node: Node,

    /// Optional source-level label reference.
    ///
    /// The referenced node remains owned by the enclosing AST graph.
    label: Option<NodeId>,
}

impl ContinueStatement {
    /// Constructs a continue statement from an already-created common node.
    ///
    /// The caller owns `NodeId` allocation and source-span determination.
    ///
    /// # Errors
    ///
    /// Returns [`ContinueStatementError::InvalidNodeKind`] if `node` is not
    /// classified as `CoreNodeKind::ContinueStatement`.
    pub fn new(
        node: Node,
        label: Option<NodeId>,
    ) -> ContinueStatementResult<Self> {
        let expected = NodeKind::core(CoreNodeKind::ContinueStatement);

        if node.kind() != &expected {
            return Err(ContinueStatementError::InvalidNodeKind {
                actual: node.kind_owned(),
            });
        }

        Ok(Self { node, label })
    }

    /// Constructs an unlabeled continue statement.
    ///
    /// This is the canonical constructor for the current Zamani grammar.
    #[must_use]
    pub fn unlabeled(node: Node) -> ContinueStatementResult<Self> {
        Self::new(node, None)
    }

    /// Returns the optional source-level label reference.
    ///
    /// This is only an AST reference. It is not a resolved symbol or runtime
    /// control-flow target.
    #[must_use]
    pub const fn label(&self) -> Option<NodeId> {
        self.label
    }

    /// Returns whether this statement has an explicit source-level label.
    #[must_use]
    pub const fn is_labeled(&self) -> bool {
        self.label.is_some()
    }

    /// Replaces the optional source-level label.
    ///
    /// Returns the previous label.
    ///
    /// This operation does not perform semantic label resolution.
    pub fn replace_label(&mut self, label: Option<NodeId>) -> Option<NodeId> {
        core::mem::replace(&mut self.label, label)
    }

    /// Returns the common AST node.
    #[must_use]
    pub const fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Only common node metadata can be modified through [`Node`]'s public
    /// mutation API; semantic information is not stored here.
    #[must_use]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the number of direct child nodes.
    ///
    /// An unlabeled continue has zero children.
    /// A labeled continue has one child.
    #[must_use]
    pub const fn child_count(&self) -> usize {
        if self.label.is_some() {
            1
        } else {
            0
        }
    }

    /// Returns the direct child node, when a label exists.
    ///
    /// This allocation-free accessor is preferred by traversal code that only
    /// needs to inspect the optional child.
    #[must_use]
    pub const fn label_child(&self) -> Option<NodeId> {
        self.label
    }

    /// Returns an allocation-free iterator over direct children.
    ///
    /// Child order is deterministic:
    ///
    /// ```text
    /// label
    /// ```
    ///
    /// for labeled statements, and empty otherwise.
    pub fn children(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.label.iter().copied()
    }

    /// Returns the direct children as a small caller-owned vector.
    ///
    /// This convenience method is intended for APIs that explicitly require
    /// an owned collection. Traversal infrastructure should normally use
    /// [`Self::children`] to avoid allocation.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        self.children().collect()
    }

    /// Performs local structural validation.
    ///
    /// This method verifies the node's local classification only. It does not
    /// verify whether the optional label resolves or whether this statement is
    /// lexically contained by a loop.
    ///
    /// Those checks require the complete AST graph and semantic context.
    pub fn validate_structure(&self) -> ContinueStatementResult<()> {
        let expected = NodeKind::core(CoreNodeKind::ContinueStatement);

        if self.node.kind() != &expected {
            return Err(ContinueStatementError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        Ok(())
    }

    /// Returns the schema version of this node representation.
    #[must_use]
    pub const fn schema_version() -> u16 {
        CONTINUE_STATEMENT_SCHEMA_VERSION
    }
}

impl AstNode for ContinueStatement {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::metadata::NodeMetadata;
    use crate::frontend::ast::node::source::Span;
    use crate::frontend::ast::node::node_id::NodeId;

    fn test_node() -> Node {
        Node::without_metadata(
            NodeId::first(),
            NodeKind::core(CoreNodeKind::ContinueStatement),
            Span::default(),
        )
    }

    #[test]
    fn constructs_unlabeled_continue() {
        let statement =
            ContinueStatement::unlabeled(test_node()).expect("valid continue node");

        assert_eq!(
            statement.kind(),
            &NodeKind::core(CoreNodeKind::ContinueStatement)
        );
        assert_eq!(statement.label(), None);
        assert_eq!(statement.child_count(), 0);
        assert_eq!(statement.children().count(), 0);
    }

    #[test]
    fn constructs_labeled_continue_without_resolving_label() {
        let label = NodeId::new(2).expect("non-zero node ID");

        let statement =
            ContinueStatement::new(test_node(), Some(label))
                .expect("valid continue node");

        assert_eq!(statement.label(), Some(label));
        assert!(statement.is_labeled());
        assert_eq!(statement.child_count(), 1);
        assert_eq!(
            statement.children().collect::<Vec<_>>(),
            vec![label]
        );
    }

    #[test]
    fn rejects_wrong_node_kind() {
        let node = Node::without_metadata(
            NodeId::first(),
            NodeKind::core(CoreNodeKind::BreakStatement),
            Span::default(),
        );

        let result = ContinueStatement::unlabeled(node);

        assert!(matches!(
            result,
            Err(ContinueStatementError::InvalidNodeKind { .. })
        ));
    }

    #[test]
    fn validation_accepts_canonical_kind() {
        let statement =
            ContinueStatement::unlabeled(test_node()).expect("valid continue node");

        statement
            .validate_structure()
            .expect("canonical continue node must validate");
    }

    #[test]
    fn validation_rejects_mutated_node_kind() {
        let mut statement =
            ContinueStatement::unlabeled(test_node()).expect("valid continue node");

        statement
            .node_mut()
            .replace_kind(NodeKind::core(CoreNodeKind::BreakStatement));

        assert!(matches!(
            statement.validate_structure(),
            Err(ContinueStatementError::InvalidNodeKind { .. })
        ));
    }

    #[test]
    fn child_order_is_deterministic() {
        let label = NodeId::new(9).expect("non-zero node ID");

        let statement =
            ContinueStatement::new(test_node(), Some(label))
                .expect("valid continue node");

        assert_eq!(
            statement.child_node_ids(),
            vec![label]
        );
    }

    #[test]
    fn replacing_label_preserves_node_identity() {
        let label_a = NodeId::new(2).expect("non-zero node ID");
        let label_b = NodeId::new(3).expect("non-zero node ID");

        let mut statement =
            ContinueStatement::new(test_node(), Some(label_a))
                .expect("valid continue node");

        let node_id_before = statement.id();

        assert_eq!(
            statement.replace_label(Some(label_b)),
            Some(label_a)
        );
        assert_eq!(statement.label(), Some(label_b));
        assert_eq!(statement.id(), node_id_before);
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let label = NodeId::new(2).expect("non-zero node ID");

        let statement =
            ContinueStatement::new(test_node(), Some(label))
                .expect("valid continue node");

        let encoded =
            serde_json::to_string(&statement).expect("serialize continue statement");

        let decoded: ContinueStatement =
            serde_json::from_str(&encoded).expect("deserialize continue statement");

        assert_eq!(decoded, statement);
        assert_eq!(decoded.label(), Some(label));
        assert_eq!(decoded.kind(), statement.kind());
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            ContinueStatement::schema_version(),
            CONTINUE_STATEMENT_SCHEMA_VERSION
        );
    }

    #[test]
    fn node_identity_is_not_changed_by_clone() {
        let statement =
            ContinueStatement::unlabeled(test_node()).expect("valid continue node");

        let clone = statement.clone();

        assert_eq!(statement.id(), clone.id());
        assert_eq!(statement, clone);
    }
}