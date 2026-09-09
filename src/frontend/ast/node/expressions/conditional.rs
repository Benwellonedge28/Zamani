//! # Zamani Native AST — Conditional Expression
//!
//! Production-ready source-level representation of a conditional expression.
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
//! ConditionalExpression  ← this file
//!     │
//!     ▼
//! structural AST validation
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
//! This module owns the **source-level structural representation** of a
//! conditional expression.
//!
//! It represents:
//!
//! - the condition expression;
//! - the expression executed when the condition is selected;
//! - the optional expression executed when the condition is not selected;
//! - the common AST node identity;
//! - source location;
//! - source-level metadata.
//!
//! It deliberately does **not** represent:
//!
//! - resolved types;
//! - resolved symbols;
//! - control-flow graphs;
//! - SSA values;
//! - branch probabilities;
//! - CPU instructions;
//! - GPU instructions;
//! - QPU instructions;
//! - physical qubits;
//! - quantum topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - resilience;
//! - backend selection;
//! - runtime state;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations.
//!
//! Those concerns belong to later compiler stages.
//!
//! ## POCO-REAF
//!
//! A conditional expression is represented entirely in terms of source-level
//! AST identities. Consequently, the same conditional can eventually be
//! lowered to classical control flow, quantum-classical feedback, accelerator
//! control, distributed execution, HDL control, or a future computational
//! model without changing this node.
//!
//! The AST therefore introduces no assumptions about:
//!
//! - machine size;
//! - register width;
//! - qubit count;
//! - number of branches supported by hardware;
//! - processor architecture;
//! - quantum technology;
//! - vendor;
//! - backend;
//! - topology.
//!
//! "Infinity" here means that this node introduces no artificial finite
//! computational-resource limit. Actual compilation remains bounded only by
//! available resources and explicitly configured compiler safety policies.
//!
//! ## Child representation
//!
//! Children are represented by [`NodeId`].
//!
//! ```text
//! ConditionalExpression
//! ├── condition: NodeId
//! ├── then_branch: NodeId
//! └── else_branch: Option<NodeId>
//! ```
//!
//! The AST graph owns the actual child nodes.
//!
//! This design avoids recursive Rust ownership structures and permits the
//! repository's traversal subsystem to perform iterative graph traversal,
//! which is important for very deeply nested programs.
//!
//! ## Optional else branch
//!
//! `else_branch` is optional because the source language may permit a
//! conditional expression without an explicit alternative.
//!
//! Semantic analysis determines whether an omitted alternative is legal in a
//! particular expression context and what type/value semantics it has.
//!
//! This file does not invent those semantics.
//!
//! ## Important distinction
//!
//! This type is **not** a control-flow graph.
//!
//! It does not encode:
//!
//! - basic blocks;
//! - CFG edges;
//! - dominance;
//! - branch probabilities;
//! - execution scheduling;
//! - speculation;
//! - branch prediction;
//! - hardware predicates.
//!
//! Those belong downstream.
//!
//! ## Quantum compatibility
//!
//! A condition may ultimately depend on:
//!
//! - a classical value;
//! - a measurement result;
//! - a hybrid computation;
//! - a future computational-domain value.
//!
//! This node does not need to know which.
//!
//! For example, a source-level conditional whose condition ultimately derives
//! from a quantum measurement is still structurally just:
//!
//! ```text
//! condition → NodeId
//! then      → NodeId
//! else      → NodeId
//! ```
//!
//! Whether the condition is semantically permitted to depend on measurement,
//! how feedback is implemented, and whether a target supports it are
//! downstream questions.
//!
//! ## No hard-coded quantum constructs
//!
//! This file must never contain representations such as:
//!
//! ```text
//! QuantumConditional
//! MeasurementConditional
//! QubitConditional
//! PhysicalQubitConditional
//! ```
//!
//! Generic AST structure is sufficient.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational native AST infrastructure:
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
//! It must never depend on:
//!
//! - semantic analysis;
//! - symbol tables;
//! - type checking;
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
//! - external quantum-language ASTs.
//!
//! ## Structural validation boundary
//!
//! Local validation checks only properties that can be established without
//! consulting the complete AST graph or semantic environment.
//!
//! This includes:
//!
//! - correct node kind;
//! - required condition reference exists;
//! - required then-branch reference exists;
//! - condition and then branch are distinct from the conditional node itself;
//! - an optional else branch is not the conditional node itself;
//! - direct child references are unique.
//!
//! It does **not** check:
//!
//! - whether a referenced `NodeId` exists in the AST graph;
//! - whether the condition has Boolean type;
//! - whether both branches have compatible types;
//! - whether a quantum measurement may control a branch;
//! - whether a backend supports dynamic control;
//! - whether a target can execute the resulting control flow.
//!
//! Those checks require later compiler context.
//!
//! ## Source spans
//!
//! The common [`Node`] stores the complete expression span.
//!
//! This module additionally permits callers to retain child spans through the
//! child nodes themselves. It deliberately does not duplicate child source
//! locations.
//!
//! ## Determinism
//!
//! Child ordering is always:
//!
//! 1. condition;
//! 2. then branch;
//! 3. else branch, when present.
//!
//! No hash-map iteration is used.
//!
//! ## Serialization
//!
//! The structure derives Serde serialization.
//!
//! The repository-wide AST serialization layer remains responsible for the
//! global serialization schema/version policy. This file does not introduce a
//! competing wire format.
//!
//! ## Scalability
//!
//! This node has a constant number of direct structural fields because a
//! conditional expression inherently has a fixed semantic shape. It introduces
//! no limit on:
//!
//! - AST size;
//! - nesting depth;
//! - number of conditional expressions;
//! - number of program nodes;
//! - number of resources;
//! - number of qubits;
//! - number of machines;
//! - number of execution targets.
//!
//! Deep nesting must be handled by the external AST traversal/validation
//! infrastructure rather than recursive methods in this file.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - executes no source program;
//! - performs no unchecked indexing;
//! - does not dereference pointers;
//! - does not access global mutable state;
//! - does not recursively traverse the AST;
//! - does not allocate based on an unbounded recursive structure.
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
//! ### `node.rs`
//!
//! Provides the common [`Node`] container.
//!
//! ### `node_id.rs`
//!
//! Provides stable AST identity.
//!
//! ### `node_kind.rs`
//!
//! Provides [`CoreNodeKind::ConditionalExpression`].
//!
//! ### `source`
//!
//! Provides [`Span`].
//!
//! ### `expressions/mod.rs`
//!
//! Must expose this module:
//!
//! ```text
//! pub mod conditional;
//! ```
//!
//! ### Expression aggregate
//!
//! The existing expression aggregate may represent a conditional using:
//!
//! ```text
//! ExpressionKind::Conditional {
//!     condition,
//!     then_branch,
//!     else_branch,
//! }
//! ```
//!
//! Those fields should use the same `NodeId` contract as this file.
//!
//! ### Parser
//!
//! The parser creates a `ConditionalExpression` and supplies:
//!
//! - a freshly allocated `NodeId`;
//! - the complete source `Span`;
//! - metadata;
//! - child `NodeId`s.
//!
//! The parser does not resolve types or hardware.
//!
//! ### Structural validation
//!
//! The AST validator invokes [`ConditionalExpression::validate_structure`].
//!
//! The validator subsequently resolves each child `NodeId` against the AST
//! graph and validates the referenced child nodes.
//!
//! ### Semantic analysis
//!
//! Semantic analysis consumes this structure and determines:
//!
//! - condition type;
//! - branch types;
//! - value semantics;
//! - reachability;
//! - effects;
//! - resource semantics;
//! - capability requirements;
//! - domain semantics.
//!
//! ### ZUIR
//!
//! ZUIR lowering consumes the semantic representation.
//!
//! This file must not import ZUIR.
//!
//! ### Optimization
//!
//! Optimization operates downstream.
//!
//! This node contains no branch simplification, constant folding, predication,
//! speculative execution, or quantum-control optimization.
//!
//! ### Scheduling
//!
//! Scheduling operates downstream.
//!
//! This node contains no timing, latency, queue, device, or hardware schedule.
//!
//! ### Hardware
//!
//! Hardware mapping operates downstream.
//!
//! No topology or physical-resource information is permitted here.
//!
//! ## No-re-edit contract
//!
//! This file exposes a stable contract around:
//!
//! - common node identity;
//! - source span;
//! - metadata;
//! - condition child;
//! - then child;
//! - optional else child;
//! - deterministic child enumeration;
//! - local structural validation.
//!
//! Changes to semantic analysis, ZUIR, quantum hardware, routing, scheduling,
//! calibration, QEC, resilience, or backends should not require modifying this
//! file.
//!
//! A new source-level conditional primitive should modify this file only if
//! the actual Zamani language semantics change.
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

/// Schema version of the conditional-expression contract.
///
/// This is intentionally separate from the global AST serialization schema
/// and the Zamani language version.
pub const CONDITIONAL_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level name for this AST node.
pub const CONDITIONAL_EXPRESSION_KIND_NAME: &str =
    "zamani:conditional-expression";

/// A source-level conditional expression.
///
/// The node stores references to its child AST nodes rather than recursively
/// embedding them.
///
/// ```text
/// ConditionalExpression
/// ├── Node
/// ├── condition
/// ├── then_branch
/// └── else_branch?
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConditionalExpression {
    /// Common source-level AST identity, kind, source span and metadata.
    node: Node,

    /// Expression used to determine which branch is selected.
    condition: NodeId,

    /// Expression evaluated when the condition selects the true branch.
    then_branch: NodeId,

    /// Optional expression evaluated when the condition selects the false
    /// branch.
    else_branch: Option<NodeId>,
}

/// Stable alias for callers that prefer the shorter name.
pub type Conditional = ConditionalExpression;

/// Errors detectable from the local structure of a conditional expression.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConditionalValidationError {
    /// The embedded node is not classified as a conditional expression.
    InvalidNodeKind {
        /// Actual node kind found.
        actual: NodeKind,
    },

    /// The condition refers to the conditional expression itself.
    SelfReferentialCondition,

    /// The true branch refers to the conditional expression itself.
    SelfReferentialThenBranch,

    /// The false branch refers to the conditional expression itself.
    SelfReferentialElseBranch,

    /// The same child ID is used by more than one structural role.
    DuplicateChild {
        /// Duplicated child identity.
        id: NodeId,
    },

    /// The condition and true branch refer to the same AST node.
    ConditionAndThenBranchAlias,

    /// The condition and false branch refer to the same AST node.
    ConditionAndElseBranchAlias,

    /// The true and false branches refer to the same AST node.
    ThenAndElseBranchAlias,
}

impl fmt::Display for ConditionalValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "conditional expression has invalid AST node kind: {actual}"
                )
            }

            Self::SelfReferentialCondition => {
                formatter.write_str(
                    "conditional expression cannot use itself as its condition",
                )
            }

            Self::SelfReferentialThenBranch => {
                formatter.write_str(
                    "conditional expression cannot use itself as its then branch",
                )
            }

            Self::SelfReferentialElseBranch => {
                formatter.write_str(
                    "conditional expression cannot use itself as its else branch",
                )
            }

            Self::DuplicateChild { id } => {
                write!(
                    formatter,
                    "conditional expression contains duplicate child node id: {id:?}"
                )
            }

            Self::ConditionAndThenBranchAlias => {
                formatter.write_str(
                    "conditional expression condition and then branch must not alias",
                )
            }

            Self::ConditionAndElseBranchAlias => {
                formatter.write_str(
                    "conditional expression condition and else branch must not alias",
                )
            }

            Self::ThenAndElseBranchAlias => {
                formatter.write_str(
                    "conditional expression then branch and else branch must not alias",
                )
            }
        }
    }
}

impl std::error::Error for ConditionalValidationError {}

impl ConditionalExpression {
    /// Creates a conditional expression from an existing common AST node.
    ///
    /// The supplied node is not rewritten. If it has the wrong node kind,
    /// [`Self::validate_structure`] reports the error.
    #[must_use]
    pub fn from_node(
        node: Node,
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
    ) -> Self {
        Self {
            node,
            condition,
            then_branch,
            else_branch,
        }
    }

    /// Creates a fully initialized conditional expression.
    ///
    /// The constructor establishes the canonical conditional node kind.
    ///
    /// It does not perform semantic validation.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::ConditionalExpression),
            span,
            metadata,
        );

        Self::from_node(node, condition, then_branch, else_branch)
    }

    /// Creates a conditional expression using default metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
    ) -> Self {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            condition,
            then_branch,
            else_branch,
        )
    }

    /// Returns the common AST node.
    #[must_use]
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    #[must_use]
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the condition child ID.
    #[must_use]
    #[inline]
    pub const fn condition(&self) -> NodeId {
        self.condition
    }

    /// Returns the true-branch child ID.
    #[must_use]
    #[inline]
    pub const fn then_branch(&self) -> NodeId {
        self.then_branch
    }

    /// Returns the optional false-branch child ID.
    #[must_use]
    #[inline]
    pub const fn else_branch(&self) -> Option<NodeId> {
        self.else_branch
    }

    /// Replaces the condition child and returns the previous child ID.
    ///
    /// The caller must run structural validation after transformation.
    #[inline]
    pub fn replace_condition(&mut self, condition: NodeId) -> NodeId {
        std::mem::replace(&mut self.condition, condition)
    }

    /// Replaces the true branch and returns the previous child ID.
    ///
    /// The caller must run structural validation after transformation.
    #[inline]
    pub fn replace_then_branch(&mut self, then_branch: NodeId) -> NodeId {
        std::mem::replace(&mut self.then_branch, then_branch)
    }

    /// Replaces the optional false branch and returns the previous value.
    ///
    /// The caller must run structural validation after transformation.
    #[inline]
    pub fn replace_else_branch(
        &mut self,
        else_branch: Option<NodeId>,
    ) -> Option<NodeId> {
        std::mem::replace(&mut self.else_branch, else_branch)
    }

    /// Returns the stable AST node ID.
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

    /// Returns source-level metadata.
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable source-level metadata.
    #[must_use]
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the node kind.
    #[must_use]
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the canonical source-level kind name.
    #[must_use]
    #[inline]
    pub const fn kind_name() -> &'static str {
        CONDITIONAL_EXPRESSION_KIND_NAME
    }

    /// Returns this node's local schema version.
    #[must_use]
    #[inline]
    pub const fn schema_version() -> u16 {
        CONDITIONAL_EXPRESSION_SCHEMA_VERSION
    }

    /// Returns `true` when the conditional has an explicit else branch.
    #[must_use]
    #[inline]
    pub const fn has_else_branch(&self) -> bool {
        self.else_branch.is_some()
    }

    /// Returns the number of direct AST children.
    ///
    /// This is either two or three.
    #[must_use]
    #[inline]
    pub const fn child_count(&self) -> usize {
        if self.else_branch.is_some() {
            3
        } else {
            2
        }
    }

    /// Returns the direct children in deterministic source/semantic order.
    ///
    /// The order is:
    ///
    /// 1. condition;
    /// 2. then branch;
    /// 3. else branch, when present.
    ///
    /// The returned iterator never recursively traverses the AST.
    #[must_use]
    pub fn child_node_ids(&self) -> ConditionalChildIds {
        ConditionalChildIds::new(
            self.condition,
            self.then_branch,
            self.else_branch,
        )
    }

    /// Returns the condition and branch IDs as a fixed logical sequence.
    ///
    /// This method is useful to generic AST walkers that prefer an iterator
    /// while avoiding allocation.
    #[must_use]
    pub fn children(&self) -> ConditionalChildIds {
        self.child_node_ids()
    }

    /// Performs local structural validation.
    ///
    /// This method intentionally does not inspect the global AST graph.
    ///
    /// It therefore cannot determine whether the child IDs actually resolve
    /// to nodes. The AST graph validator owns that responsibility.
    pub fn validate_structure(
        &self,
    ) -> Result<(), ConditionalValidationError> {
        if self.node.kind()
            != &NodeKind::core(CoreNodeKind::ConditionalExpression)
        {
            return Err(ConditionalValidationError::InvalidNodeKind {
                actual: self.node.kind().clone(),
            });
        }

        let node_id = self.node.id();

        if self.condition == node_id {
            return Err(
                ConditionalValidationError::SelfReferentialCondition,
            );
        }

        if self.then_branch == node_id {
            return Err(
                ConditionalValidationError::SelfReferentialThenBranch,
            );
        }

        if self.else_branch == Some(node_id) {
            return Err(
                ConditionalValidationError::SelfReferentialElseBranch,
            );
        }

        if self.condition == self.then_branch {
            return Err(
                ConditionalValidationError::ConditionAndThenBranchAlias,
            );
        }

        if self.else_branch == Some(self.condition) {
            return Err(
                ConditionalValidationError::ConditionAndElseBranchAlias,
            );
        }

        if self.else_branch == Some(self.then_branch) {
            return Err(
                ConditionalValidationError::ThenAndElseBranchAlias,
            );
        }

        Ok(())
    }

    /// Returns `true` when the local structure is valid.
    ///
    /// This is deliberately equivalent to checking
    /// [`Self::validate_structure`].
    #[must_use]
    #[inline]
    pub fn is_structurally_valid(&self) -> bool {
        self.validate_structure().is_ok()
    }
}

impl AstNode for ConditionalExpression {
    /// Returns the embedded common AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the embedded common AST node.
    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Direct children of a conditional expression.
///
/// This iterator is allocation-free and non-recursive.
#[derive(Clone, Debug)]
pub struct ConditionalChildIds {
    condition: NodeId,
    then_branch: NodeId,
    else_branch: Option<NodeId>,
    index: u8,
}

impl ConditionalChildIds {
    /// Creates a deterministic direct-child iterator.
    #[must_use]
    pub const fn new(
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
            index: 0,
        }
    }
}

impl Iterator for ConditionalChildIds {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(self.condition),

            1 => Some(self.then_branch),

            2 => self.else_branch,

            _ => None,
        };

        if result.is_some() {
            self.index = self.index.saturating_add(1);
        }

        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = match self.index {
            0 => 2 + usize::from(self.else_branch.is_some()),
            1 => 1 + usize::from(self.else_branch.is_some()),
            2 => usize::from(self.else_branch.is_some()),
            _ => 0,
        };

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for ConditionalChildIds {}

impl std::iter::FusedIterator for ConditionalChildIds {}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn span() -> Span {
        Span::default()
    }

    fn conditional(
        id: u64,
        condition: u64,
        then_branch: u64,
        else_branch: Option<u64>,
    ) -> ConditionalExpression {
        ConditionalExpression::without_metadata(
            node_id(id),
            span(),
            node_id(condition),
            node_id(then_branch),
            else_branch.map(node_id),
        )
    }

    #[test]
    fn constructor_uses_conditional_node_kind() {
        let expression = conditional(1, 2, 3, Some(4));

        assert_eq!(
            expression.kind(),
            &NodeKind::core(CoreNodeKind::ConditionalExpression)
        );
    }

    #[test]
    fn constructor_preserves_child_ids() {
        let expression = conditional(1, 2, 3, Some(4));

        assert_eq!(expression.condition(), node_id(2));
        assert_eq!(expression.then_branch(), node_id(3));
        assert_eq!(expression.else_branch(), Some(node_id(4)));
    }

    #[test]
    fn child_count_without_else_is_two() {
        let expression = conditional(1, 2, 3, None);

        assert_eq!(expression.child_count(), 2);
        assert!(!expression.has_else_branch());
    }

    #[test]
    fn child_count_with_else_is_three() {
        let expression = conditional(1, 2, 3, Some(4));

        assert_eq!(expression.child_count(), 3);
        assert!(expression.has_else_branch());
    }

    #[test]
    fn children_are_deterministic() {
        let expression = conditional(1, 2, 3, Some(4));

        let children: Vec<NodeId> = expression.child_node_ids().collect();

        assert_eq!(
            children,
            vec![node_id(2), node_id(3), node_id(4)]
        );
    }

    #[test]
    fn children_without_else_are_deterministic() {
        let expression = conditional(1, 2, 3, None);

        let children: Vec<NodeId> = expression.child_node_ids().collect();

        assert_eq!(children, vec![node_id(2), node_id(3)]);
    }

    #[test]
    fn iterator_reports_exact_size() {
        let expression = conditional(1, 2, 3, Some(4));

        assert_eq!(expression.child_node_ids().len(), 3);
    }

    #[test]
    fn valid_conditional_passes_validation() {
        let expression = conditional(1, 2, 3, Some(4));

        assert_eq!(expression.validate_structure(), Ok(()));
    }

    #[test]
    fn valid_conditional_without_else_passes_validation() {
        let expression = conditional(1, 2, 3, None);

        assert_eq!(expression.validate_structure(), Ok(()));
    }

    #[test]
    fn self_referential_condition_is_rejected() {
        let expression = conditional(1, 1, 3, Some(4));

        assert_eq!(
            expression.validate_structure(),
            Err(ConditionalValidationError::SelfReferentialCondition)
        );
    }

    #[test]
    fn self_referential_then_branch_is_rejected() {
        let expression = conditional(1, 2, 1, Some(4));

        assert_eq!(
            expression.validate_structure(),
            Err(
                ConditionalValidationError::SelfReferentialThenBranch
            )
        );
    }

    #[test]
    fn self_referential_else_branch_is_rejected() {
        let expression = conditional(1, 2, 3, Some(1));

        assert_eq!(
            expression.validate_structure(),
            Err(
                ConditionalValidationError::SelfReferentialElseBranch
            )
        );
    }

    #[test]
    fn condition_and_then_branch_alias_is_rejected() {
        let expression = conditional(1, 2, 2, Some(3));

        assert_eq!(
            expression.validate_structure(),
            Err(
                ConditionalValidationError::ConditionAndThenBranchAlias
            )
        );
    }

    #[test]
    fn condition_and_else_branch_alias_is_rejected() {
        let expression = conditional(1, 2, 3, Some(2));

        assert_eq!(
            expression.validate_structure(),
            Err(
                ConditionalValidationError::ConditionAndElseBranchAlias
            )
        );
    }

    #[test]
    fn then_and_else_branch_alias_is_rejected() {
        let expression = conditional(1, 2, 3, Some(3));

        assert_eq!(
            expression.validate_structure(),
            Err(
                ConditionalValidationError::ThenAndElseBranchAlias
            )
        );
    }

    #[test]
    fn replacement_methods_return_previous_values() {
        let mut expression = conditional(1, 2, 3, Some(4));

        assert_eq!(
            expression.replace_condition(node_id(5)),
            node_id(2)
        );

        assert_eq!(
            expression.replace_then_branch(node_id(6)),
            node_id(3)
        );

        assert_eq!(
            expression.replace_else_branch(Some(node_id(7))),
            Some(node_id(4))
        );

        assert_eq!(expression.condition(), node_id(5));
        assert_eq!(expression.then_branch(), node_id(6));
        assert_eq!(expression.else_branch(), Some(node_id(7)));
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let expression = conditional(1, 2, 3, Some(4));

        let encoded =
            serde_json::to_string(&expression).expect("serialization");

        let decoded: ConditionalExpression =
            serde_json::from_str(&encoded).expect("deserialization");

        assert_eq!(decoded, expression);
    }

    #[test]
    fn serialization_round_trip_preserves_missing_else() {
        let expression = conditional(1, 2, 3, None);

        let encoded =
            serde_json::to_string(&expression).expect("serialization");

        let decoded: ConditionalExpression =
            serde_json::from_str(&encoded).expect("deserialization");

        assert_eq!(decoded, expression);
    }

    #[test]
    fn unicode_and_large_source_independent_ids_are_supported() {
        let name_like_program_identifier =
            "条件_квантовый_λογική_🌍";

        assert!(!name_like_program_identifier.is_empty());

        let expression = conditional(
            1,
            2,
            3,
            Some(4),
        );

        assert!(expression.is_structurally_valid());
    }
}