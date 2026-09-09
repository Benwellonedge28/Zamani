//! # Zamani Frontend AST — `break` Statement
//!
//! Canonical source-level representation of the native Zamani `break`
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
//! BreakStatement                    ← this module
//!     │
//!     ▼
//! Native AST structural validation
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
//!     ├── classical lowering
//!     ├── quantum lowering
//!     ├── hybrid lowering
//!     ├── distributed lowering
//!     ├── HDL lowering
//!     └── future-domain lowering
//!     │
//!     ▼
//! Target / runtime / hardware
//! ```
//!
//! ## Purpose
//!
//! This module owns the native Zamani source-level representation of `break`.
//!
//! `break` is a control-flow construct. It expresses the programmer's intent
//! to terminate the nearest applicable enclosing loop or other source-level
//! control construct according to Zamani's semantic rules.
//!
//! This node deliberately does **not** determine:
//!
//! - which loop is dynamically targeted;
//! - whether a loop exists in the enclosing semantic context;
//! - whether a label resolves;
//! - whether a value is legal for the enclosing construct;
//! - whether the enclosing construct is classical or quantum;
//! - how control flow is represented in ZUIR;
//! - how control flow is lowered to a CPU, GPU, FPGA, QPU, simulator, or
//!   distributed system;
//! - scheduling;
//! - routing;
//! - resource allocation;
//! - hardware mapping;
//! - error correction;
//! - resilience;
//! - backend instructions.
//!
//! Those responsibilities belong to later compiler phases.
//!
//! ## POCO-REAF
//!
//! This representation contains no machine-size assumptions.
//!
//! In particular, it does not contain:
//!
//! - maximum loop count;
//! - maximum nesting depth;
//! - maximum number of resources;
//! - maximum qubits;
//! - maximum processors;
//! - physical resource identifiers;
//! - backend identifiers;
//! - vendor identifiers;
//! - hardware topology;
//! - instruction-set information;
//! - scheduler state.
//!
//! Therefore a `break` node can occur in a program that ultimately executes
//! on the smallest supported computational system or on a much larger
//! heterogeneous system without changing the source AST representation.
//!
//! "Infinity" in POCO-REAF means that this AST type imposes no artificial
//! finite computational-resource or machine-size language limit. Actual
//! compilation remains bounded by available resources and explicitly supplied
//! compiler safety policies.
//!
//! ## Canonical representation
//!
//! The repository's existing statement model already defines:
//!
//! ```text
//! Break {
//!     label: Option<NodeId>,
//!     value: Option<NodeId>,
//! }
//! ```
//!
//! This file makes that representation an independently owned AST node while
//! preserving those exact source-level concepts.
//!
//! There must be only one authoritative representation of a native `break`
//! statement after AST migration:
//!
//! ```text
//! BreakStatement
//!       │
//!       ├── optional label ──► AST node
//!       └── optional value ──► AST expression
//! ```
//!
//! The enclosing AST graph owns the referenced nodes.
//!
//! ## Domain neutrality
//!
//! `BreakStatement` is intentionally independent of quantum computation.
//!
//! A `break` may occur around computation involving:
//!
//! - classical values;
//! - quantum resources;
//! - hybrid quantum/classical computation;
//! - accelerators;
//! - distributed resources;
//! - HDL-oriented constructs;
//! - future computational domains.
//!
//! The node does not need to know which domain eventually interprets its
//! surrounding control-flow region.
//!
//! ## Child ordering
//!
//! Direct children are exposed in deterministic source/semantic order:
//!
//! 1. `label`, when present;
//! 2. `value`, when present.
//!
//! The embedded [`Node`] is metadata for this statement and is not itself a
//! child.
//!
//! This ordering must remain stable because traversal, diagnostics, hashing,
//! incremental compilation and tooling may rely on deterministic AST order.
//!
//! ## AST graph boundary
//!
//! Child relationships are represented by [`NodeId`] rather than recursive
//! concrete AST ownership.
//!
//! This permits the enclosing AST graph to own all nodes and allows traversal
//! infrastructure to use explicit worklists where appropriate.
//!
//! This module therefore does not import expression, pattern, loop, semantic,
//! or resource implementations merely to represent a child reference.
//!
//! ## Parser contract
//!
//! The parser is responsible for:
//!
//! 1. recognizing `break`;
//! 2. parsing an optional source-level label;
//! 3. parsing an optional break value where the grammar permits one;
//! 4. allocating the statement `NodeId`;
//! 5. determining the complete source span;
//! 6. constructing the `BreakStatement`;
//! 7. inserting referenced child nodes into the enclosing AST graph.
//!
//! The parser must not perform:
//!
//! - label resolution;
//! - loop-scope resolution;
//! - type checking;
//! - control-flow legality analysis;
//! - quantum analysis;
//! - resource allocation;
//! - scheduling;
//! - routing;
//! - hardware selection.
//!
//! ## Semantic contract
//!
//! Semantic analysis consumes this node and determines:
//!
//! - which enclosing control-flow construct receives the break;
//! - whether an optional label resolves;
//! - whether the break is inside an applicable construct;
//! - whether a value is permitted;
//! - the type of the value;
//! - ownership and lifetime implications;
//! - effects;
//! - resource consequences;
//! - domain-specific control-flow meaning.
//!
//! None of those facts are stored in this AST node.
//!
//! ## ZUIR contract
//!
//! The node is lowered through the semantic model.
//!
//! ```text
//! BreakStatement
//!       │
//!       ▼
//! Semantic control-transfer meaning
//!       │
//!       ▼
//! ZUIR control transfer
//! ```
//!
//! The AST must not import or depend upon ZUIR.
//!
//! ## Quantum contract
//!
//! Quantum code may contain ordinary source-level control flow. For example,
//! a quantum algorithm may iterate over a symbolic resource collection and
//! terminate that loop conditionally.
//!
//! This node does not need a quantum-specific form such as:
//!
//! ```text
//! QuantumBreak
//! ```
//!
//! Such a type would unnecessarily couple the native AST to a particular
//! computational domain.
//!
//! Quantum-specific semantics are resolved downstream.
//!
//! ## Validation boundary
//!
//! Local validation performed here is intentionally structural.
//!
//! It may verify:
//!
//! - the embedded node has `CoreNodeKind::BreakStatement`;
//! - the statement does not directly reference itself;
//! - the node identity is consistent with the AST node contract.
//!
//! It must not verify:
//!
//! - whether the statement occurs inside a loop;
//! - whether a label resolves;
//! - whether a break value has a valid type;
//! - whether the value is allowed by the enclosing construct;
//! - whether the surrounding computation is quantum;
//! - whether the target hardware supports the resulting control flow.
//!
//! Those checks belong to semantic analysis and later stages.
//!
//! ## Scalability
//!
//! This type contains a constant number of optional references and therefore
//! introduces no collection-size limit.
//!
//! It does not define:
//!
//! ```text
//! MAX_BREAKS
//! MAX_LOOPS
//! MAX_NESTING
//! MAX_QUBITS
//! MAX_MACHINES
//! MAX_RESOURCES
//! ```
//!
//! Compiler-service resource limits must be supplied externally through
//! configurable validation/session policies.
//!
//! ## Determinism
//!
//! This type has no:
//!
//! - random state;
//! - timestamps;
//! - memory addresses;
//! - global mutable state;
//! - backend-generated identifiers;
//! - hash-map traversal.
//!
//! Child enumeration is always label followed by value.
//!
//! ## Serialization
//!
//! The structure derives Serde serialization.
//!
//! Global AST schema/version negotiation remains owned by the AST serialization
//! subsystem. This module does not create a competing serialization protocol.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - forbids unsafe code;
//! - performs no I/O;
//! - executes no source code;
//! - performs no unchecked indexing;
//! - performs no pointer arithmetic;
//! - does not recursively traverse the AST;
//! - has no global mutable state.
//!
//! Untrusted serialized/program input must be validated by the AST validation
//! layer before semantic compilation.
//!
//! ## Incremental compilation
//!
//! Semantic information can remain in external side tables keyed by [`NodeId`].
//!
//! For example:
//!
//! ```text
//! NodeId → resolved label
//! NodeId → enclosing control-flow target
//! NodeId → value type
//! NodeId → effects
//! NodeId → resource requirements
//! NodeId → domain information
//! ```
//!
//! This keeps source syntax immutable and supports incremental and parallel
//! compilation.
//!
//! ## No-reedit integration guarantee
//!
//! The public contract of this module depends only on foundational AST
//! concepts:
//!
//! - [`Node`];
//! - [`NodeId`];
//! - [`AstNode`];
//! - [`NodeKind`];
//! - [`CoreNodeKind`].
//!
//! It does not depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - optimization;
//! - scheduling;
//! - routing;
//! - QEC;
//! - calibration;
//! - hardware;
//! - runtime;
//! - backend APIs.
//!
//! Consequently those downstream components can evolve without requiring
//! changes to this file unless the source-language meaning of `break` itself
//! changes.
//!
//! ## Migration contract
//!
//! The existing canonical statement enum contains a `Break` variant with:
//!
//! ```text
//! label: Option<NodeId>
//! value: Option<NodeId>
//! ```
//!
//! During migration, `Statement::Break { label, value }` must map exactly to:
//!
//! ```text
//! BreakStatement::new(..., label, value)
//! ```
//!
//! There must not be two independent semantic representations.
//!
//! After all consumers have migrated, the old inline representation should be
//! removed or reduced to a compatibility wrapper according to the repository's
//! public API compatibility policy.
//!
//! The existing `CoreNodeKind::BreakStatement` remains authoritative.
//!
//! ## File contract
//!
//! **Owns**
//!
//! - the native `BreakStatement` source representation;
//! - optional label reference;
//! - optional break-value reference;
//! - local structural invariants;
//! - deterministic direct-child enumeration.
//!
//! **Does not own**
//!
//! - AST graph storage;
//! - source text;
//! - lexer tokens;
//! - parsing;
//! - semantic resolution;
//! - loop resolution;
//! - type checking;
//! - control-flow graph construction;
//! - ZUIR;
//! - quantum IR;
//! - scheduling;
//! - routing;
//! - QEC;
//! - hardware;
//! - runtime.
//!
//! **Allowed dependencies**
//!
//! - Rust standard library;
//! - Serde;
//! - foundational AST node modules.
//!
//! **Forbidden dependencies**
//!
//! - parser;
//! - lexer;
//! - semantic;
//! - compiler driver;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - backend;
//! - optimizer;
//! - scheduler;
//! - router;
//! - runtime.
//!
//! **Completion criterion**
//!
//! This file is complete when the parser, structural validator, visitors,
//! traversal system, semantic analyzer and lowering system can consume the
//! `BreakStatement` contract without requiring this node to know anything
//! about the target machine or downstream implementation.
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

/// Independent schema version for the native `break` statement contract.
///
/// This is intentionally separate from the overall AST schema version and
/// Zamani language version.
pub const BREAK_STATEMENT_SCHEMA_VERSION: u16 = 1;

/// Stable source-level diagnostic/tooling identity.
pub const BREAK_STATEMENT_KIND_NAME: &str = "zamani:break-statement";

/// Canonical native Zamani `break` statement.
///
/// The node contains only source-level information. References to the optional
/// label and value are owned by the enclosing AST graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BreakStatement {
    /// Common AST identity, classification, source span and metadata.
    node: Node,

    /// Optional source-level label.
    ///
    /// Semantic analysis resolves the label and determines the corresponding
    /// control-flow target.
    label: Option<NodeId>,

    /// Optional source-level break value.
    ///
    /// Whether valued breaks are legal and what type the value has are semantic
    /// questions.
    value: Option<NodeId>,
}

/// Structural validation errors for [`BreakStatement`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BreakStatementValidationError {
    /// The embedded node is not classified as a native break statement.
    InvalidNodeKind {
        /// Actual node kind found on the embedded node.
        actual: NodeKind,
    },

    /// The statement directly references itself.
    SelfReference {
        /// Logical field containing the self-reference.
        field: &'static str,
    },
}

impl fmt::Display for BreakStatementValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "break statement has invalid AST node kind: {actual}"
                )
            }

            Self::SelfReference { field } => {
                write!(
                    formatter,
                    "break statement `{field}` must not reference itself"
                )
            }
        }
    }
}

impl std::error::Error for BreakStatementValidationError {}

impl BreakStatement {
    /// Creates a canonical source-level `break` statement.
    ///
    /// The supplied `Node` must represent
    /// [`CoreNodeKind::BreakStatement`].
    ///
    /// The constructor deliberately does not resolve labels, validate loop
    /// context, infer types, or perform any target-specific analysis.
    #[must_use]
    pub fn new(
        node: Node,
        label: Option<NodeId>,
        value: Option<NodeId>,
    ) -> Self {
        Self {
            node,
            label,
            value,
        }
    }

    /// Returns the embedded canonical AST node.
    #[must_use]
    #[inline]
    pub const fn node(&self) -> &Node {
        &self.node
    }

    /// Returns the optional label node.
    #[must_use]
    #[inline]
    pub const fn label(&self) -> Option<NodeId> {
        self.label
    }

    /// Returns the optional break-value node.
    #[must_use]
    #[inline]
    pub const fn value(&self) -> Option<NodeId> {
        self.value
    }

    /// Returns the number of direct child references.
    ///
    /// This is always in the closed range `0..=2` because the representation
    /// contains exactly two optional child references.
    ///
    /// No program-size or machine-size limit is implied by this method.
    #[must_use]
    #[inline]
    pub const fn child_count(&self) -> usize {
        match (self.label.is_some(), self.value.is_some()) {
            (false, false) => 0,
            (true, false) | (false, true) => 1,
            (true, true) => 2,
        }
    }

    /// Returns the direct children in deterministic source/semantic order.
    ///
    /// Ordering is:
    ///
    /// 1. label;
    /// 2. value.
    ///
    /// No allocation is performed.
    #[must_use]
    #[inline]
    pub fn children(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.label.into_iter().chain(self.value)
    }

    /// Validates local AST structure.
    ///
    /// This method deliberately does not perform semantic control-flow
    /// validation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - the embedded node is not a `BreakStatement`;
    /// - the label references the statement itself;
    /// - the value references the statement itself.
    pub fn validate_structure(
        &self,
    ) -> Result<(), BreakStatementValidationError> {
        let expected = NodeKind::core(CoreNodeKind::BreakStatement);

        if self.node.node_kind() != expected {
            return Err(
                BreakStatementValidationError::InvalidNodeKind {
                    actual: self.node.node_kind(),
                },
            );
        }

        let statement_id = self.node.id();

        if self.label == Some(statement_id) {
            return Err(
                BreakStatementValidationError::SelfReference {
                    field: "label",
                },
            );
        }

        if self.value == Some(statement_id) {
            return Err(
                BreakStatementValidationError::SelfReference {
                    field: "value",
                },
            );
        }

        Ok(())
    }

    /// Returns whether this statement contains a source-level label.
    #[must_use]
    #[inline]
    pub const fn has_label(&self) -> bool {
        self.label.is_some()
    }

    /// Returns whether this statement contains a source-level value.
    #[must_use]
    #[inline]
    pub const fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Returns the stable AST kind name.
    #[must_use]
    #[inline]
    pub const fn kind_name() -> &'static str {
        BREAK_STATEMENT_KIND_NAME
    }

    /// Returns the independent node schema version.
    #[must_use]
    #[inline]
    pub const fn schema_version() -> u16 {
        BREAK_STATEMENT_SCHEMA_VERSION
    }
}

impl AstNode for BreakStatement {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: u64) -> Node {
        /*
         * This helper intentionally uses the repository's canonical Node
         * constructor. Source-span construction belongs to the existing AST
         * source infrastructure rather than this statement node.
         *
         * If NodeId in the current repository is not directly constructible
         * from u64, this helper should be replaced by the repository's
         * canonical deterministic NodeId constructor when these tests are
         * integrated.
         */
        let node_id = NodeId::from_u64(id);
        Node::new(node_id, Default::default())
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(BREAK_STATEMENT_SCHEMA_VERSION, 1);
    }

    #[test]
    fn kind_name_is_stable() {
        assert_eq!(
            BreakStatement::kind_name(),
            "zamani:break-statement"
        );
    }

    #[test]
    fn child_count_tracks_optional_children() {
        let statement = BreakStatement::new(
            node(1),
            None,
            None,
        );

        assert_eq!(statement.child_count(), 0);
    }

    #[test]
    fn child_count_with_label() {
        let statement = BreakStatement::new(
            node(1),
            Some(NodeId::from_u64(2)),
            None,
        );

        assert_eq!(statement.child_count(), 1);
    }

    #[test]
    fn child_count_with_value() {
        let statement = BreakStatement::new(
            node(1),
            None,
            Some(NodeId::from_u64(2)),
        );

        assert_eq!(statement.child_count(), 1);
    }

    #[test]
    fn children_are_deterministic() {
        let label = NodeId::from_u64(2);
        let value = NodeId::from_u64(3);

        let statement = BreakStatement::new(
            node(1),
            Some(label),
            Some(value),
        );

        let children: Vec<NodeId> = statement.children().collect();

        assert_eq!(children, vec![label, value]);
    }

    #[test]
    fn label_precedes_value() {
        let label = NodeId::from_u64(20);
        let value = NodeId::from_u64(30);

        let statement = BreakStatement::new(
            node(10),
            Some(label),
            Some(value),
        );

        let mut children = statement.children();

        assert_eq!(children.next(), Some(label));
        assert_eq!(children.next(), Some(value));
        assert_eq!(children.next(), None);
    }

    #[test]
    fn accessors_preserve_source_references() {
        let label = NodeId::from_u64(2);
        let value = NodeId::from_u64(3);

        let statement = BreakStatement::new(
            node(1),
            Some(label),
            Some(value),
        );

        assert_eq!(statement.label(), Some(label));
        assert_eq!(statement.value(), Some(value));
        assert!(statement.has_label());
        assert!(statement.has_value());
    }

    #[test]
    fn no_children_is_valid() {
        let statement = BreakStatement::new(
            node(1),
            None,
            None,
        );

        assert!(statement.validate_structure().is_ok());
    }

    #[test]
    fn self_label_reference_is_rejected() {
        let statement_id = NodeId::from_u64(1);

        let statement = BreakStatement::new(
            Node::new(statement_id, Default::default()),
            Some(statement_id),
            None,
        );

        let result = statement.validate_structure();

        assert!(matches!(
            result,
            Err(
                BreakStatementValidationError::SelfReference {
                    field: "label"
                }
            )
        ));
    }

    #[test]
    fn self_value_reference_is_rejected() {
        let statement_id = NodeId::from_u64(1);

        let statement = BreakStatement::new(
            Node::new(statement_id, Default::default()),
            None,
            Some(statement_id),
        );

        let result = statement.validate_structure();

        assert!(matches!(
            result,
            Err(
                BreakStatementValidationError::SelfReference {
                    field: "value"
                }
            )
        ));
    }
}