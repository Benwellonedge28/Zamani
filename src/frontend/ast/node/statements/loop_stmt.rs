//! # Zamani Frontend AST — Loop Statements
//!
//! Production-ready, source-level representation of native Zamani loop
//! statements.
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
//! LoopStatement
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
//!     └── future-domain IRs
//!     │
//!     ▼
//! Target/resource lowering
//!     │
//!     ▼
//! Execution
//! ```
//!
//! ## Purpose
//!
//! This module owns the native Zamani AST representation of source-level
//! looping constructs.
//!
//! It deliberately represents **programmer intent**, not the eventual
//! implementation of the loop.
//!
//! A loop may eventually execute as:
//!
//! - classical control flow;
//! - quantum/classical hybrid control flow;
//! - distributed control flow;
//! - accelerator control flow;
//! - generated hardware control;
//! - simulator control flow;
//! - another computational model introduced in the future.
//!
//! This module does not decide which realization is selected.
//!
//! ## POCO-REAF
//!
//! Loop representation is independent of:
//!
//! - machine size;
//! - CPU count;
//! - GPU count;
//! - FPGA resources;
//! - QPU resources;
//! - qubit count;
//! - register width;
//! - hardware topology;
//! - processor architecture;
//! - vendor;
//! - backend;
//! - instruction set;
//! - scheduling;
//! - routing;
//! - calibration;
//! - error correction;
//! - resilience;
//! - runtime state.
//!
//! Therefore a source loop can participate in:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! without the AST being redesigned for a different machine size.
//!
//! ## Supported native forms
//!
//! The current native AST contract distinguishes:
//!
//! ```text
//! while condition { body }
//!
//! for pattern in iterable { body }
//! ```
//!
//! These are intentionally separate source constructs because their source
//! semantics are different.
//!
//! `while` owns:
//!
//! - a condition expression;
//! - a body node.
//!
//! `for` owns:
//!
//! - a binding pattern;
//! - an iterable/range expression;
//! - a body node.
//!
//! The semantic layer determines the precise meaning of those children.
//!
//! ## Critical architectural boundary
//!
//! This module does NOT perform:
//!
//! - type checking;
//! - name resolution;
//! - borrow checking;
//! - ownership analysis;
//! - termination proofs;
//! - resource allocation;
//! - quantum resource allocation;
//! - quantum routing;
//! - scheduling;
//! - loop unrolling;
//! - hardware mapping;
//! - QEC;
//! - noise analysis;
//! - backend selection;
//! - target optimization.
//!
//! Those responsibilities belong to later compiler phases.
//!
//! ## Quantum neutrality
//!
//! A loop may contain quantum operations or quantum/classical control.
//!
//! For example:
//!
//! ```text
//! while condition {
//!     operation(...);
//! }
//! ```
//!
//! or:
//!
//! ```text
//! for q in resources {
//!     operation(q);
//! }
//! ```
//!
//! This node does not need to know whether `operation` eventually becomes:
//!
//! - a quantum operation;
//! - a classical operation;
//! - a distributed operation;
//! - a hardware operation;
//! - a future-domain operation.
//!
//! Measurement-dependent control flow, resource legality, reversibility,
//! termination, scheduling and hardware feasibility are resolved downstream.
//!
//! ## Source graph model
//!
//! Children are represented by [`NodeId`] references.
//!
//! The surrounding AST graph owns the referenced nodes.
//!
//! ```text
//! LoopStatement
//! ├── Node
//! └── LoopKind
//!     ├── While
//!     │   ├── condition ──► AST node
//!     │   └── body ───────► AST node
//!     │
//!     └── For
//!         ├── pattern ────► AST node
//!         ├── iterable ───► AST node
//!         └── body ───────► AST node
//! ```
//!
//! This avoids recursively embedding AST nodes and permits traversal
//! infrastructure to use explicit worklists when handling extremely large or
//! deeply nested programs.
//!
//! ## Deterministic child order
//!
//! Children are exposed in source/structural order.
//!
//! For `while`:
//!
//! 1. condition;
//! 2. body.
//!
//! For `for`:
//!
//! 1. pattern;
//! 2. iterable;
//! 3. body.
//!
//! No hash-map iteration is involved.
//!
//! ## Scalability
//!
//! This module introduces no semantic maximum for:
//!
//! - number of loops;
//! - number of loop iterations;
//! - nesting depth;
//! - quantum resources;
//! - classical resources;
//! - machines;
//! - processors;
//! - qubits;
//! - registers;
//! - loop body size;
//! - program size.
//!
//! Runtime iteration counts are not stored in this AST node.
//!
//! Any compiler-service safety limits must be supplied by an explicit,
//! configurable validation/resource policy outside this representation.
//!
//! The term "infinity" therefore means that this node introduces no artificial
//! finite computational or machine-size restriction. Actual execution remains
//! bounded by available resources and the target's capabilities.
//!
//! ## Dependency boundary
//!
//! This module may depend only on:
//!
//! - [`Node`];
//! - [`AstNode`];
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`CoreNodeKind`];
//! - Serde;
//! - the Rust standard library.
//!
//! It must never depend on:
//!
//! - parser implementation;
//! - lexer tokens;
//! - semantic analysis;
//! - symbol tables;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum backends;
//! - hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime.
//!
//! ## Structural validation boundary
//!
//! Local validation checks only facts that can be established without looking
//! into the referenced AST graph.
//!
//! It verifies:
//!
//! - the embedded node kind matches the loop variant;
//! - required child references exist;
//! - no child directly references the loop node itself.
//!
//! It does NOT verify:
//!
//! - that `condition` is boolean;
//! - that `pattern` is legal;
//! - that `iterable` is iterable;
//! - that `body` is a statement/block;
//! - that the loop terminates;
//! - that a quantum operation is legal;
//! - that the target can execute the loop.
//!
//! Those checks require the AST graph, semantic information, or downstream
//! compilation information.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - forbids `unsafe`;
//! - performs no I/O;
//! - executes no program;
//! - performs no pointer operations;
//! - performs no unchecked indexing;
//! - performs no global mutation;
//! - performs no recursive traversal.
//!
//! Malformed graphs are handled by the AST validation layer.
//!
//! ## Serialization
//!
//! The types derive Serde serialization.
//!
//! Global AST serialization/version negotiation remains owned by the AST
//! serialization subsystem.
//!
//! This file does not invent a second global serialization format.
//!
//! ## Incremental compilation
//!
//! Semantic information may be stored externally using [`NodeId`] keys.
//!
//! For example:
//!
//! ```text
//! NodeId → resolved condition type
//! NodeId → resolved iterator type
//! NodeId → control-flow facts
//! NodeId → effects
//! NodeId → resource requirements
//! NodeId → capabilities
//! NodeId → lowering information
//! ```
//!
//! The source AST remains immutable from the perspective of semantic analysis.
//!
//! ## Integration contract
//!
//! ### `node.rs`
//!
//! Provides the common [`Node`] and [`AstNode`] contracts.
//!
//! ### `node_id.rs`
//!
//! Provides opaque AST identity.
//!
//! ### `node_kind.rs`
//!
//! Provides:
//!
//! - [`CoreNodeKind::WhileStatement`];
//! - [`CoreNodeKind::ForStatement`].
//!
//! ### `statement.rs`
//!
//! This module should be the canonical owner of concrete loop data.
//!
//! `StatementKind::While` and `StatementKind::For` should delegate to
//! [`LoopStatement`] rather than maintaining a second independent copy of the
//! loop representation.
//!
//! ### Parser
//!
//! The parser:
//!
//! 1. allocates the node ID;
//! 2. determines the source span;
//! 3. parses the loop condition/pattern/iterable;
//! 4. parses the body;
//! 5. constructs the appropriate `LoopStatement`;
//! 6. inserts referenced children into the AST graph.
//!
//! The parser must not perform semantic or hardware analysis.
//!
//! ### Structural validation
//!
//! The graph validator first validates this node locally and then validates
//! every referenced child against the complete AST graph.
//!
//! ### Semantic analysis
//!
//! Semantic analysis resolves:
//!
//! - names;
//! - types;
//! - iteration semantics;
//! - condition semantics;
//! - control-flow semantics;
//! - ownership;
//! - effects;
//! - capabilities;
//! - resource requirements;
//! - domain semantics;
//! - termination-related properties where required.
//!
//! ### ZUIR
//!
//! Lowering converts the semantic meaning of the loop into the appropriate
//! ZUIR control-flow representation.
//!
//! This module must never import ZUIR.
//!
//! ### Quantum compiler
//!
//! Quantum operations appearing inside a loop remain ordinary AST child
//! constructs.
//!
//! The quantum compiler determines whether the resulting control flow can be:
//!
//! - executed directly;
//! - transformed;
//! - partially evaluated;
//! - represented using classical control;
//! - lowered to a quantum-domain representation;
//! - rejected because of semantic/target constraints.
//!
//! None of those decisions belong here.
//!
//! ## No-reedit guarantee
//!
//! The public contract of this file depends only on foundational AST concepts
//! and the source-level distinction between `while` and `for`.
//!
//! Changes to:
//!
//! - quantum IR;
//! - ZUIR;
//! - hardware;
//! - routing;
//! - scheduling;
//! - QEC;
//! - resilience;
//! - calibration;
//! - backends;
//! - target optimization
//!
//! must not require changes to this file.
//!
//! A change should require reopening this file only if Zamani's actual
//! source-language loop syntax/semantics changes.
//!
//! ## Rust compatibility
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

use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};

/// Independent schema version for the native loop-node contract.
///
/// This is deliberately separate from the global AST schema and Zamani
/// language version.
pub const LOOP_STATEMENT_SCHEMA_VERSION: u16 = 1;

/// Stable source-level diagnostic/tooling identity.
pub const LOOP_STATEMENT_KIND_NAME: &str = "zamani:loop-statement";

/// The source-level form of a loop.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LoopKind {
    /// Conditional loop.
    ///
    /// Semantics of the condition are resolved later.
    While {
        /// Condition expression.
        condition: NodeId,

        /// Loop body.
        body: NodeId,
    },

    /// Iterator/range loop.
    ///
    /// The pattern receives the semantic iteration value.
    For {
        /// Loop binding pattern.
        pattern: NodeId,

        /// Iterable/range expression.
        iterable: NodeId,

        /// Loop body.
        body: NodeId,
    },
}

impl LoopKind {
    /// Returns the canonical native node kind represented by this loop form.
    #[must_use]
    pub const fn core_node_kind(&self) -> CoreNodeKind {
        match self {
            Self::While { .. } => CoreNodeKind::WhileStatement,
            Self::For { .. } => CoreNodeKind::ForStatement,
        }
    }

    /// Returns the number of direct child references.
    #[must_use]
    pub const fn child_count(&self) -> usize {
        match self {
            Self::While { .. } => 2,
            Self::For { .. } => 3,
        }
    }

    /// Returns the direct child references in deterministic source order.
    ///
    /// This is a caller-owned snapshot. AST traversal infrastructure that
    /// requires zero-allocation traversal should use its dedicated traversal
    /// API instead of repeatedly materializing this vector.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        match self {
            Self::While { condition, body } => {
                vec![*condition, *body]
            }

            Self::For {
                pattern,
                iterable,
                body,
            } => {
                vec![*pattern, *iterable, *body]
            }
        }
    }

    /// Returns `true` when this is a `while` loop.
    #[must_use]
    pub const fn is_while(&self) -> bool {
        matches!(self, Self::While { .. })
    }

    /// Returns `true` when this is a `for` loop.
    #[must_use]
    pub const fn is_for(&self) -> bool {
        matches!(self, Self::For { .. })
    }

    /// Returns the `while` condition, when this is a `while` loop.
    #[must_use]
    pub const fn while_condition(&self) -> Option<NodeId> {
        match self {
            Self::While { condition, .. } => Some(*condition),
            Self::For { .. } => None,
        }
    }

    /// Returns the `for` binding pattern, when this is a `for` loop.
    #[must_use]
    pub const fn for_pattern(&self) -> Option<NodeId> {
        match self {
            Self::While { .. } => None,
            Self::For { pattern, .. } => Some(*pattern),
        }
    }

    /// Returns the `for` iterable, when this is a `for` loop.
    #[must_use]
    pub const fn for_iterable(&self) -> Option<NodeId> {
        match self {
            Self::While { .. } => None,
            Self::For { iterable, .. } => Some(*iterable),
        }
    }

    /// Returns the loop body.
    #[must_use]
    pub const fn body(&self) -> NodeId {
        match self {
            Self::While { body, .. } | Self::For { body, .. } => *body,
        }
    }
}

/// Errors produced by local loop structural validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoopStatementValidationError {
    /// The embedded node kind does not match the loop variant.
    InvalidNodeKind {
        /// Expected node kind.
        expected: CoreNodeKind,

        /// Actual node kind.
        actual: NodeKind,
    },

    /// A child points back to this loop node.
    SelfReference {
        /// Logical role of the offending child.
        role: &'static str,
    },
}

impl fmt::Display for LoopStatementValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { expected, actual } => {
                write!(
                    formatter,
                    "loop statement has invalid AST node kind: expected \
                     zamani:{}, found {}",
                    expected.name(),
                    actual,
                )
            }

            Self::SelfReference { role } => {
                write!(
                    formatter,
                    "loop statement {role} references itself",
                )
            }
        }
    }
}

impl std::error::Error for LoopStatementValidationError {}

/// Canonical source-level loop statement.
///
/// All child relationships use [`NodeId`]. The enclosing AST graph owns the
/// referenced nodes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoopStatement {
    /// Common AST identity, classification, span and metadata.
    node: Node,

    /// Concrete source-level loop form.
    kind: LoopKind,
}

impl LoopStatement {
    /// Constructs a loop statement from an already-created canonical [`Node`].
    ///
    /// The caller is responsible for providing a node whose kind matches
    /// `kind`.
    #[must_use]
    pub fn from_node(node: Node, kind: LoopKind) -> Self {
        Self { node, kind }
    }

    /// Constructs a canonical `while` loop.
    #[must_use]
    pub fn while_loop(
        id: NodeId,
        span: crate::frontend::ast::node::source::Span,
        metadata: crate::frontend::ast::node::metadata::NodeMetadata,
        condition: NodeId,
        body: NodeId,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::WhileStatement),
            span,
            metadata,
        );

        Self::from_node(
            node,
            LoopKind::While { condition, body },
        )
    }

    /// Constructs a canonical `for` loop.
    #[must_use]
    pub fn for_loop(
        id: NodeId,
        span: crate::frontend::ast::node::source::Span,
        metadata: crate::frontend::ast::node::metadata::NodeMetadata,
        pattern: NodeId,
        iterable: NodeId,
        body: NodeId,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::ForStatement),
            span,
            metadata,
        );

        Self::from_node(
            node,
            LoopKind::For {
                pattern,
                iterable,
                body,
            },
        )
    }

    /// Returns the loop form.
    #[must_use]
    pub const fn kind(&self) -> &LoopKind {
        &self.kind
    }

    /// Returns whether this is a `while` loop.
    #[must_use]
    pub const fn is_while(&self) -> bool {
        self.kind.is_while()
    }

    /// Returns whether this is a `for` loop.
    #[must_use]
    pub const fn is_for(&self) -> bool {
        self.kind.is_for()
    }

    /// Returns the `while` condition when present.
    #[must_use]
    pub const fn while_condition(&self) -> Option<NodeId> {
        self.kind.while_condition()
    }

    /// Returns the `for` pattern when present.
    #[must_use]
    pub const fn for_pattern(&self) -> Option<NodeId> {
        self.kind.for_pattern()
    }

    /// Returns the `for` iterable when present.
    #[must_use]
    pub const fn for_iterable(&self) -> Option<NodeId> {
        self.kind.for_iterable()
    }

    /// Returns the loop body.
    #[must_use]
    pub const fn body(&self) -> NodeId {
        self.kind.body()
    }

    /// Returns the number of direct children.
    #[must_use]
    pub const fn child_count(&self) -> usize {
        self.kind.child_count()
    }

    /// Returns direct children in deterministic source order.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        self.kind.child_node_ids()
    }

    /// Returns this node's independent schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        LOOP_STATEMENT_SCHEMA_VERSION
    }

    /// Returns the stable source-level kind name.
    #[must_use]
    pub const fn kind_name() -> &'static str {
        LOOP_STATEMENT_KIND_NAME
    }

    /// Performs local structural validation.
    ///
    /// This method intentionally does not dereference child IDs.
    pub fn validate_structure(
        &self,
    ) -> Result<(), LoopStatementValidationError> {
        let expected =
            NodeKind::core(self.kind.core_node_kind());

        if self.node.kind() != &expected {
            return Err(
                LoopStatementValidationError::InvalidNodeKind {
                    expected: self.kind.core_node_kind(),
                    actual: self.node.kind_owned(),
                },
            );
        }

        let id = self.node.id();

        match &self.kind {
            LoopKind::While { condition, body } => {
                if *condition == id {
                    return Err(
                        LoopStatementValidationError::SelfReference {
                            role: "condition",
                        },
                    );
                }

                if *body == id {
                    return Err(
                        LoopStatementValidationError::SelfReference {
                            role: "body",
                        },
                    );
                }
            }

            LoopKind::For {
                pattern,
                iterable,
                body,
            } => {
                if *pattern == id {
                    return Err(
                        LoopStatementValidationError::SelfReference {
                            role: "pattern",
                        },
                    );
                }

                if *iterable == id {
                    return Err(
                        LoopStatementValidationError::SelfReference {
                            role: "iterable",
                        },
                    );
                }

                if *body == id {
                    return Err(
                        LoopStatementValidationError::SelfReference {
                            role: "body",
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Returns the embedded canonical AST node.
    #[must_use]
    pub const fn as_node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the embedded canonical AST node.
    #[must_use]
    pub fn as_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Replaces the loop kind.
    ///
    /// The node's `NodeKind` is updated atomically with the logical loop form
    /// so the object cannot intentionally be left with a stale kind through
    /// this API.
    pub fn set_kind(&mut self, kind: LoopKind) {
        self.node
            .replace_kind(NodeKind::core(kind.core_node_kind()));

        self.kind = kind;
    }

    /// Replaces the `while` condition.
    ///
    /// Returns `false` when this is not a `while` loop.
    pub fn set_while_condition(
        &mut self,
        condition: NodeId,
    ) -> bool {
        match &mut self.kind {
            LoopKind::While {
                condition: current,
                ..
            } => {
                *current = condition;
                true
            }

            LoopKind::For { .. } => false,
        }
    }

    /// Replaces the loop body.
    pub fn set_body(&mut self, body: NodeId) {
        match &mut self.kind {
            LoopKind::While {
                body: current, ..
            }
            | LoopKind::For {
                body: current, ..
            } => {
                *current = body;
            }
        }
    }

    /// Replaces the `for` pattern.
    ///
    /// Returns `false` when this is not a `for` loop.
    pub fn set_for_pattern(
        &mut self,
        pattern: NodeId,
    ) -> bool {
        match &mut self.kind {
            LoopKind::While { .. } => false,

            LoopKind::For {
                pattern: current,
                ..
            } => {
                *current = pattern;
                true
            }
        }
    }

    /// Replaces the `for` iterable.
    ///
    /// Returns `false` when this is not a `for` loop.
    pub fn set_for_iterable(
        &mut self,
        iterable: NodeId,
    ) -> bool {
        match &mut self.kind {
            LoopKind::While { .. } => false,

            LoopKind::For {
                iterable: current,
                ..
            } => {
                *current = iterable;
                true
            }
        }
    }
}

impl AstNode for LoopStatement {
    fn node(&self) -> &Node {
        &self.node
    }

    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::metadata::NodeMetadata;
    use crate::frontend::ast::node::source::Span;

    fn id(value: u64) -> NodeId {
        NodeId::new(value).expect("test NodeId must be non-zero")
    }

    #[test]
    fn while_loop_has_while_node_kind() {
        let statement = LoopStatement::while_loop(
            id(1),
            Span::default(),
            NodeMetadata::default(),
            id(2),
            id(3),
        );

        assert_eq!(
            statement.kind(),
            &LoopKind::While {
                condition: id(2),
                body: id(3),
            }
        );

        assert_eq!(
            statement.kind(),
            &LoopKind::While {
                condition: id(2),
                body: id(3),
            }
        );

        assert_eq!(
            statement.as_node().kind(),
            &NodeKind::core(CoreNodeKind::WhileStatement)
        );
    }

    #[test]
    fn for_loop_has_for_node_kind() {
        let statement = LoopStatement::for_loop(
            id(1),
            Span::default(),
            NodeMetadata::default(),
            id(2),
            id(3),
            id(4),
        );

        assert_eq!(
            statement.as_node().kind(),
            &NodeKind::core(CoreNodeKind::ForStatement)
        );
    }

    #[test]
    fn while_children_are_condition_then_body() {
        let statement = LoopStatement::while_loop(
            id(1),
            Span::default(),
            NodeMetadata::default(),
            id(2),
            id(3),
        );

        assert_eq!(
            statement.child_node_ids(),
            vec![id(2), id(3)]
        );
    }

    #[test]
    fn for_children_are_pattern_iterable_then_body() {
        let statement = LoopStatement::for_loop(
            id(1),
            Span::default(),
            NodeMetadata::default(),
            id(2),
            id(3),
            id(4),
        );

        assert_eq!(
            statement.child_node_ids(),
            vec![id(2), id(3), id(4)]
        );
    }

    #[test]
    fn while_self_reference_is_rejected() {
        let statement = LoopStatement::while_loop(
            id(1),
            Span::default(),
            NodeMetadata::default(),
            id(1),
            id(2),
        );

        assert_eq!(
            statement.validate_structure(),
            Err(
                LoopStatementValidationError::SelfReference {
                    role: "condition",
                }
            )
        );
    }

    #[test]
    fn for_self_reference_is_rejected() {
        let statement = LoopStatement::for_loop(
            id(1),
            Span::default(),
            NodeMetadata::default(),
            id(2),
            id(1),
            id(3),
        );

        assert_eq!(
            statement.validate_structure(),
            Err(
                LoopStatementValidationError::SelfReference {
                    role: "iterable",
                }
            )
        );
    }

    #[test]
    fn mismatched_node_kind_is_rejected() {
        let node = Node::new(
            id(1),
            NodeKind::core(CoreNodeKind::ForStatement),
            Span::default(),
            NodeMetadata::default(),
        );

        let statement = LoopStatement::from_node(
            node,
            LoopKind::While {
                condition: id(2),
                body: id(3),
            },
        );

        assert!(matches!(
            statement.validate_structure(),
            Err(
                LoopStatementValidationError::InvalidNodeKind {
                    expected: CoreNodeKind::WhileStatement,
                    ..
                }
            )
        ));
    }

    #[test]
    fn set_kind_keeps_node_kind_in_sync() {
        let mut statement = LoopStatement::while_loop(
            id(1),
            Span::default(),
            NodeMetadata::default(),
            id(2),
            id(3),
        );

        statement.set_kind(LoopKind::For {
            pattern: id(4),
            iterable: id(5),
            body: id(6),
        });

        assert!(statement.is_for());
        assert_eq!(
            statement.as_node().kind(),
            &NodeKind::core(CoreNodeKind::ForStatement)
        );
        assert_eq!(
            statement.child_node_ids(),
            vec![id(4), id(5), id(6)]
        );
    }

    #[test]
    fn valid_while_passes_validation() {
        let statement = LoopStatement::while_loop(
            id(1),
            Span::default(),
            NodeMetadata::default(),
            id(2),
            id(3),
        );

        assert_eq!(statement.validate_structure(), Ok(()));
    }

    #[test]
    fn valid_for_passes_validation() {
        let statement = LoopStatement::for_loop(
            id(1),
            Span::default(),
            NodeMetadata::default(),
            id(2),
            id(3),
            id(4),
        );

        assert_eq!(statement.validate_structure(), Ok(()));
    }
}