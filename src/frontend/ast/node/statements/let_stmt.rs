//! # Zamani Frontend AST — `let` Statement
//!
//! Production-ready source-level representation of a Zamani `let` binding.
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
//! LetStatement  ← this module
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
//! This module owns the native Zamani AST representation of a source-level
//! `let` binding.
//!
//! A `let` statement records what the programmer wrote. It does not decide
//! what the binding eventually becomes at runtime.
//!
//! The same source construct may eventually participate in:
//!
//! - classical computation;
//! - quantum computation;
//! - hybrid quantum/classical computation;
//! - distributed computation;
//! - accelerator computation;
//! - HDL-oriented computation;
//! - future computational domains.
//!
//! Those interpretations belong to later compiler stages.
//!
//! ## POCO-REAF
//!
//! This node intentionally contains no:
//!
//! - machine size;
//! - CPU register;
//! - GPU register;
//! - FPGA resource;
//! - QPU resource;
//! - physical qubit;
//! - logical qubit;
//! - qubit count;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - scheduler allocation;
//! - routing decision;
//! - calibration;
//! - QEC implementation;
//! - resilience implementation;
//! - runtime value;
//! - memory address.
//!
//! Consequently, the source-level binding can participate in the Zamani
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF) model.
//!
//! The compiler determines the eventual realization from the semantic meaning,
//! available capabilities, resources, target constraints, and execution
//! environment.
//!
//! ## Source-level responsibility
//!
//! This type owns exactly:
//!
//! - canonical AST node identity;
//! - canonical AST node kind;
//! - source span;
//! - source metadata;
//! - the `let` binding identity;
//! - the original source name;
//! - an optional source-level type annotation;
//! - an optional initializer;
//! - source-level attributes;
//! - source-level annotations.
//!
//! It does NOT own:
//!
//! - symbol resolution;
//! - inferred types;
//! - ownership state;
//! - lifetime state;
//! - borrow state;
//! - resource allocation;
//! - capability resolution;
//! - effects after semantic analysis;
//! - ZUIR values;
//! - backend values.
//!
//! ## Child representation
//!
//! Child AST nodes are represented by [`NodeId`].
//!
//! This is intentional. The native AST is graph-oriented rather than a deeply
//! recursively owned Rust object graph.
//!
//! ```text
//! LetStatement
//! ├── Node
//! ├── type_annotation ──► Type AST node
//! ├── initializer ──────► Expression AST node
//! ├── attributes ───────► Attribute AST nodes
//! └── annotations ──────► Annotation AST nodes
//! ```
//!
//! The graph itself is owned by the surrounding AST/program container.
//!
//! This node therefore does not need to import concrete expression, type,
//! attribute, or annotation implementations.
//!
//! ## Traversal order
//!
//! Direct children are exposed in deterministic source-oriented order:
//!
//! 1. type annotation;
//! 2. initializer;
//! 3. attributes in source order;
//! 4. annotations in source order.
//!
//! The `Node` itself is not a child.
//!
//! ## Grammar integration
//!
//! The current repository grammar represents variable bindings through a
//! `variableDeclaration` production whose keyword alternatives include `let`.
//! The canonical future mapping for the native AST is:
//!
//! ```text
//! let IDENTIFIER (':' typeExpr)? ('=' expression)? ';'
//!             │
//!             ▼
//!       LetStatement
//! ```
//!
//! The parser must perform the keyword-to-AST mapping. This file must never
//! import lexer token types.
//!
//! If the grammar remains intentionally unified around `variableDeclaration`,
//! the parser should select `LetStatement` when the parsed binding keyword is
//! `let`, while `var` and `const` are represented by their respective canonical
//! statement/declaration nodes.
//!
//! There must not be two independent authoritative AST representations for a
//! `let` statement.
//!
//! ## Dependency boundary
//!
//! This module may depend only on foundational AST infrastructure:
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
//! - OpenQASM AST;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend providers.
//!
//! ## Validation boundary
//!
//! [`LetStatement::validate_structure`] performs only local structural
//! validation.
//!
//! It verifies:
//!
//! - the node has `CoreNodeKind::LetStatement`;
//! - the source name is non-empty;
//! - child references do not point to the statement itself;
//! - extension identities used by local metadata are not interpreted here.
//!
//! It does NOT verify:
//!
//! - whether the name resolves;
//! - whether the name is unique;
//! - whether the type exists;
//! - whether the initializer has the correct type;
//! - whether initialization is required by the language;
//! - whether the binding may be reassigned;
//! - ownership;
//! - borrowing;
//! - lifetimes;
//! - quantum-resource legality;
//! - resource capacity;
//! - hardware feasibility.
//!
//! Those are semantic or later compilation concerns.
//!
//! ## Scalability
//!
//! This node introduces no finite machine or program-size limit.
//!
//! It does not contain:
//!
//! ```text
//! MAX_LET_STATEMENTS
//! MAX_VARIABLES
//! MAX_ATTRIBUTES
//! MAX_ANNOTATIONS
//! MAX_QUANTUM_RESOURCES
//! MAX_QUBITS
//! MAX_MACHINE_SIZE
//! ```
//!
//! Dynamic collections grow according to available resources and the
//! configurable compiler/resource policy.
//!
//! "Infinity" here means that this AST node introduces no artificial language
//! limit. Real execution remains bounded by available memory, address space,
//! storage, compiler policy, and target capabilities.
//!
//! ## Security
//!
//! This module:
//!
//! - contains no `unsafe` code;
//! - forbids unsafe code at the module level;
//! - performs no I/O;
//! - executes no source program;
//! - performs no unchecked indexing;
//! - performs no pointer operations;
//! - performs no recursive child traversal;
//! - has no global mutable state;
//! - performs no backend lookup.
//!
//! Malformed child graphs are validated by the AST graph validator.
//!
//! ## Determinism
//!
//! Child collections use `Vec<NodeId>` because source order is meaningful.
//!
//! No hash-map iteration is used for child traversal.
//!
//! The node itself contains no timestamps, memory addresses, random state,
//! thread-local state, or backend-generated identifiers.
//!
//! ## Serialization
//!
//! The node derives Serde serialization.
//!
//! Global AST serialization/versioning remains owned by the AST serialization
//! subsystem. This node must not invent a competing global schema/versioning
//! system.
//!
//! Serialization contains only source-level AST information.
//!
//! ## Incremental compilation
//!
//! Semantic information remains outside this node and can therefore be keyed
//! by [`NodeId`]:
//!
//! ```text
//! NodeId → resolved symbol
//! NodeId → resolved type
//! NodeId → mutability result
//! NodeId → ownership result
//! NodeId → effects
//! NodeId → capabilities
//! NodeId → resource requirements
//! ```
//!
//! This separation permits incremental compilation and parallel read-only
//! compiler phases without mutating source syntax.
//!
//! ## Integration contracts
//!
//! ### `node.rs`
//!
//! Supplies [`Node`] and [`AstNode`].
//!
//! ### `node_id.rs`
//!
//! Supplies stable node identity.
//!
//! ### `node_kind.rs`
//!
//! Supplies [`CoreNodeKind::LetStatement`].
//!
//! ### `source`
//!
//! Supplies [`Span`].
//!
//! ### `metadata`
//!
//! Supplies [`NodeMetadata`].
//!
//! ### Parser
//!
//! The parser must:
//!
//! 1. allocate a `NodeId`;
//! 2. construct the `Node` using `CoreNodeKind::LetStatement`;
//! 3. parse the source identifier;
//! 4. parse an optional type annotation;
//! 5. parse an optional initializer according to the grammar;
//! 6. collect attributes/annotations;
//! 7. construct this node;
//! 8. insert referenced child nodes into the owning AST graph.
//!
//! The parser must not pass lexer token types into this module.
//!
//! ### Structural validation
//!
//! The AST validator must first call [`LetStatement::validate_structure`] and
//! then validate the referenced nodes through the AST graph.
//!
//! ### Semantic analysis
//!
//! Semantic analysis consumes the immutable validated node and resolves:
//!
//! - identifier meaning;
//! - declared/inferred type;
//! - initialization rules;
//! - binding semantics;
//! - ownership;
//! - lifetimes;
//! - effects;
//! - capabilities;
//! - resource requirements;
//! - domain semantics.
//!
//! ### ZUIR
//!
//! Semantic lowering determines what this binding means in ZUIR.
//!
//! It may eventually represent a:
//!
//! - classical value;
//! - quantum resource handle;
//! - hybrid value;
//! - distributed value;
//! - accelerator value;
//! - future domain value.
//!
//! This module intentionally has no ZUIR dependency.
//!
//! ### Quantum compiler
//!
//! A `let` binding can hold or refer to quantum-related semantic objects, but
//! this AST node does not know that.
//!
//! For example:
//!
//! ```text
//! let result = measure(q);
//! ```
//!
//! remains a normal source-level `let` statement.
//!
//! Measurement semantics are resolved downstream.
//!
//! ### Optimization
//!
//! Constant propagation, dead binding elimination, SSA construction, copy
//! propagation, resource analysis, and other transformations belong downstream.
//!
//! ### Hardware
//!
//! Hardware allocation, registers, physical resources, QPU mappings, memory
//! placement, routing, scheduling, calibration, and backend instruction
//! selection remain downstream.
//!
//! ## No-reedit guarantee
//!
//! This file's public contract is intentionally independent of:
//!
//! - semantic representation;
//! - ZUIR representation;
//! - quantum IR;
//! - QEC;
//! - routing;
//! - scheduling;
//! - calibration;
//! - runtime;
//! - backend implementation.
//!
//! Those systems may evolve without requiring this node to be reopened unless
//! the source-language contract of `let` itself changes.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no `unsafe`.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Independent schema version for the native `let` statement contract.
///
/// This is deliberately distinct from:
///
/// - the Zamani language version;
/// - the global AST schema version;
/// - the serialized AST format version.
///
/// The AST serialization layer owns global version negotiation.
pub const LET_STATEMENT_SCHEMA_VERSION: u16 = 1;

/// Stable source-level diagnostic/tooling identity.
pub const LET_STATEMENT_KIND_NAME: &str = "zamani:let-statement";

/// Canonical source-level `let` statement.
///
/// A `LetStatement` represents the source construct rather than its eventual
/// machine realization.
///
/// All relationships to other AST nodes are expressed using [`NodeId`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LetStatement {
    /// Common AST identity, classification, source span, and metadata.
    node: Node,

    /// Original source identifier spelling.
    ///
    /// Name resolution is deliberately performed by semantic analysis.
    name: String,

    /// Optional source-level type annotation.
    ///
    /// The referenced node must resolve to a type AST node during structural
    /// graph validation.
    type_annotation: Option<NodeId>,

    /// Optional initializer expression.
    ///
    /// The referenced node must resolve to an expression AST node during
    /// structural graph validation.
    initializer: Option<NodeId>,

    /// Source-level attributes in source order.
    attributes: Vec<NodeId>,

    /// Source-level annotations in source order.
    annotations: Vec<NodeId>,
}

/// Errors produced by local structural validation of a [`LetStatement`].
///
/// These errors deliberately describe AST structure rather than semantic
/// legality.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LetStatementValidationError {
    /// The declaration name is empty.
    EmptyName,

    /// The embedded node is not classified as `LetStatement`.
    InvalidNodeKind {
        /// The actual node kind.
        actual: NodeKind,
    },

    /// A child reference points back to this statement.
    SelfReference {
        /// The offending child role.
        role: &'static str,
    },

    /// An attribute references this statement itself.
    SelfAttributeReference,

    /// An annotation references this statement itself.
    SelfAnnotationReference,
}

impl core::fmt::Display for LetStatementValidationError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("let statement has an empty binding name")
            }

            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "let statement has invalid AST node kind: {actual}"
                )
            }

            Self::SelfReference { role } => {
                write!(
                    formatter,
                    "let statement {role} references itself"
                )
            }

            Self::SelfAttributeReference => {
                formatter.write_str(
                    "let statement contains itself as an attribute",
                )
            }

            Self::SelfAnnotationReference => {
                formatter.write_str(
                    "let statement contains itself as an annotation",
                )
            }
        }
    }
}

impl std::error::Error for LetStatementValidationError {}

impl LetStatement {
    /// Creates a `let` statement from an already constructed canonical node.
    ///
    /// The caller is responsible for supplying a node whose kind is
    /// `CoreNodeKind::LetStatement`.
    ///
    /// This constructor does not perform semantic validation and does not
    /// allocate a new `NodeId`.
    #[must_use]
    pub fn from_node(
        node: Node,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        initializer: Option<NodeId>,
    ) -> Self {
        Self {
            node,
            name: name.into(),
            type_annotation,
            initializer,
            attributes: Vec::new(),
            annotations: Vec::new(),
        }
    }

    /// Creates a canonical `let` statement from foundational AST components.
    ///
    /// The resulting node is classified as
    /// `CoreNodeKind::LetStatement`.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        initializer: Option<NodeId>,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::LetStatement),
            span,
            metadata,
        );

        Self::from_node(
            node,
            name,
            type_annotation,
            initializer,
        )
    }

    /// Creates a `let` statement with default node metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        initializer: Option<NodeId>,
    ) -> Self {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            name,
            type_annotation,
            initializer,
        )
    }

    /// Returns the original source identifier spelling.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Replaces the source identifier after validating that it is non-empty.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), LetStatementValidationError> {
        let name = name.into();

        if name.is_empty() {
            return Err(LetStatementValidationError::EmptyName);
        }

        self.name = name;
        Ok(())
    }

    /// Sets or removes the source-level type annotation.
    pub fn set_type_annotation(
        &mut self,
        type_annotation: Option<NodeId>,
    ) {
        self.type_annotation = type_annotation;
    }

    /// Returns the optional type annotation node.
    #[must_use]
    pub const fn type_annotation(&self) -> Option<NodeId> {
        self.type_annotation
    }

    /// Sets or removes the initializer.
    pub fn set_initializer(
        &mut self,
        initializer: Option<NodeId>,
    ) {
        self.initializer = initializer;
    }

    /// Returns the optional initializer node.
    #[must_use]
    pub const fn initializer(&self) -> Option<NodeId> {
        self.initializer
    }

    /// Adds an attribute while preserving source order.
    pub fn push_attribute(&mut self, attribute: NodeId) {
        self.attributes.push(attribute);
    }

    /// Adds an annotation while preserving source order.
    pub fn push_annotation(&mut self, annotation: NodeId) {
        self.annotations.push(annotation);
    }

    /// Returns the attributes in source order.
    #[must_use]
    pub fn attributes(&self) -> &[NodeId] {
        &self.attributes
    }

    /// Returns the annotations in source order.
    #[must_use]
    pub fn annotations(&self) -> &[NodeId] {
        &self.annotations
    }

    /// Returns the independent schema version for this node contract.
    #[must_use]
    pub const fn schema_version() -> u16 {
        LET_STATEMENT_SCHEMA_VERSION
    }

    /// Returns the stable source-level kind name.
    #[must_use]
    pub const fn kind_name() -> &'static str {
        LET_STATEMENT_KIND_NAME
    }

    /// Returns all direct child node IDs in deterministic traversal order.
    ///
    /// The statement itself is not included.
    ///
    /// Order:
    ///
    /// 1. type annotation;
    /// 2. initializer;
    /// 3. attributes;
    /// 4. annotations.
    ///
    /// A fresh `Vec` is intentionally returned so callers cannot mutate the
    /// statement's internal child collections through this API.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        let capacity = usize::from(self.type_annotation.is_some())
            + usize::from(self.initializer.is_some())
            + self.attributes.len()
            + self.annotations.len();

        let mut children = Vec::with_capacity(capacity);

        if let Some(type_annotation) = self.type_annotation {
            children.push(type_annotation);
        }

        if let Some(initializer) = self.initializer {
            children.push(initializer);
        }

        children.extend(self.attributes.iter().copied());
        children.extend(self.annotations.iter().copied());

        children
    }

    /// Performs local structural validation.
    ///
    /// This method deliberately does not inspect the referenced AST graph.
    /// Graph-level type/category validation belongs to the owning AST
    /// validation subsystem.
    pub fn validate_structure(
        &self,
    ) -> Result<(), LetStatementValidationError> {
        let expected = NodeKind::core(CoreNodeKind::LetStatement);

        if self.node.kind() != &expected {
            return Err(
                LetStatementValidationError::InvalidNodeKind {
                    actual: self.node.kind_owned(),
                },
            );
        }

        if self.name.is_empty() {
            return Err(LetStatementValidationError::EmptyName);
        }

        let id = self.node.id();

        if self.type_annotation == Some(id) {
            return Err(
                LetStatementValidationError::SelfReference {
                    role: "type annotation",
                },
            );
        }

        if self.initializer == Some(id) {
            return Err(
                LetStatementValidationError::SelfReference {
                    role: "initializer",
                },
            );
        }

        if self.attributes.iter().any(|child| *child == id) {
            return Err(
                LetStatementValidationError::SelfAttributeReference,
            );
        }

        if self.annotations.iter().any(|child| *child == id) {
            return Err(
                LetStatementValidationError::SelfAnnotationReference,
            );
        }

        Ok(())
    }

    /// Returns whether the statement has a type annotation.
    #[must_use]
    pub const fn has_type_annotation(&self) -> bool {
        self.type_annotation.is_some()
    }

    /// Returns whether the statement has an initializer.
    #[must_use]
    pub const fn has_initializer(&self) -> bool {
        self.initializer.is_some()
    }

    /// Returns whether the statement contains attributes.
    #[must_use]
    pub fn has_attributes(&self) -> bool {
        !self.attributes.is_empty()
    }

    /// Returns whether the statement contains annotations.
    #[must_use]
    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }

    /// Returns the number of direct AST child references.
    ///
    /// This is intentionally calculated from the current collections rather
    /// than bounded by a compile-time maximum.
    #[must_use]
    pub fn child_count(&self) -> usize {
        usize::from(self.type_annotation.is_some())
            + usize::from(self.initializer.is_some())
            + self.attributes.len()
            + self.annotations.len()
    }

    /// Returns an immutable reference to the common AST node.
    #[must_use]
    pub fn as_node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Only metadata can normally be changed through the `Node` API without
    /// changing the source node's structural identity.
    #[must_use]
    pub fn as_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl AstNode for LetStatement {
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
        /*
         * `NodeId` construction is intentionally kept behind the repository's
         * canonical constructor. If the repository's NodeId API changes, this
         * helper is the only test-local construction point that needs to move.
         *
         * The production implementation itself never constructs IDs.
         */
        NodeId::from_u64(value)
    }

    #[test]
    fn new_creates_let_statement_kind() {
        let statement = LetStatement::without_metadata(
            node_id(1),
            Span::default(),
            "answer",
            None,
            None,
        );

        assert_eq!(
            statement.kind(),
            &NodeKind::core(CoreNodeKind::LetStatement)
        );
        assert_eq!(statement.name(), "answer");
    }

    #[test]
    fn node_identity_is_preserved() {
        let id = node_id(42);

        let statement = LetStatement::without_metadata(
            id,
            Span::default(),
            "value",
            None,
            None,
        );

        assert_eq!(statement.id(), id);
    }

    #[test]
    fn empty_name_is_rejected_by_setter() {
        let mut statement = LetStatement::without_metadata(
            node_id(1),
            Span::default(),
            "value",
            None,
            None,
        );

        assert_eq!(
            statement.set_name(""),
            Err(LetStatementValidationError::EmptyName)
        );

        assert_eq!(statement.name(), "value");
    }

    #[test]
    fn empty_name_fails_structural_validation() {
        let statement = LetStatement::without_metadata(
            node_id(1),
            Span::default(),
            "",
            None,
            None,
        );

        assert_eq!(
            statement.validate_structure(),
            Err(LetStatementValidationError::EmptyName)
        );
    }

    #[test]
    fn valid_statement_passes_structural_validation() {
        let statement = LetStatement::without_metadata(
            node_id(1),
            Span::default(),
            "value",
            Some(node_id(2)),
            Some(node_id(3)),
        );

        assert_eq!(statement.validate_structure(), Ok(()));
    }

    #[test]
    fn child_order_is_deterministic() {
        let mut statement = LetStatement::without_metadata(
            node_id(1),
            Span::default(),
            "value",
            Some(node_id(2)),
            Some(node_id(3)),
        );

        statement.push_attribute(node_id(4));
        statement.push_attribute(node_id(5));
        statement.push_annotation(node_id(6));
        statement.push_annotation(node_id(7));

        assert_eq!(
            statement.child_node_ids(),
            vec![
                node_id(2),
                node_id(3),
                node_id(4),
                node_id(5),
                node_id(6),
                node_id(7),
            ]
        );
    }

    #[test]
    fn child_count_matches_child_ids() {
        let mut statement = LetStatement::without_metadata(
            node_id(1),
            Span::default(),
            "value",
            Some(node_id(2)),
            Some(node_id(3)),
        );

        statement.push_attribute(node_id(4));
        statement.push_annotation(node_id(5));

        assert_eq!(
            statement.child_count(),
            statement.child_node_ids().len()
        );
    }

    #[test]
    fn self_reference_is_rejected() {
        let id = node_id(1);

        let statement = LetStatement::without_metadata(
            id,
            Span::default(),
            "value",
            Some(id),
            None,
        );

        assert_eq!(
            statement.validate_structure(),
            Err(LetStatementValidationError::SelfReference {
                role: "type annotation",
            })
        );
    }

    #[test]
    fn self_initializer_reference_is_rejected() {
        let id = node_id(1);

        let statement = LetStatement::without_metadata(
            id,
            Span::default(),
            "value",
            None,
            Some(id),
        );

        assert_eq!(
            statement.validate_structure(),
            Err(LetStatementValidationError::SelfReference {
                role: "initializer",
            })
        );
    }

    #[test]
    fn self_attribute_reference_is_rejected() {
        let id = node_id(1);

        let mut statement = LetStatement::without_metadata(
            id,
            Span::default(),
            "value",
            None,
            None,
        );

        statement.push_attribute(id);

        assert_eq!(
            statement.validate_structure(),
            Err(LetStatementValidationError::SelfAttributeReference)
        );
    }

    #[test]
    fn self_annotation_reference_is_rejected() {
        let id = node_id(1);

        let mut statement = LetStatement::without_metadata(
            id,
            Span::default(),
            "value",
            None,
            None,
        );

        statement.push_annotation(id);

        assert_eq!(
            statement.validate_structure(),
            Err(LetStatementValidationError::SelfAnnotationReference)
        );
    }

    #[test]
    fn optional_components_are_reported_correctly() {
        let statement = LetStatement::without_metadata(
            node_id(1),
            Span::default(),
            "value",
            Some(node_id(2)),
            Some(node_id(3)),
        );

        assert!(statement.has_type_annotation());
        assert!(statement.has_initializer());
        assert!(!statement.has_attributes());
        assert!(!statement.has_annotations());
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            LetStatement::schema_version(),
            LET_STATEMENT_SCHEMA_VERSION
        );

        assert_eq!(
            LetStatement::kind_name(),
            "zamani:let-statement"
        );
    }

    #[test]
    fn ast_node_trait_exposes_common_identity() {
        let id = node_id(99);

        let statement = LetStatement::without_metadata(
            id,
            Span::default(),
            "value",
            None,
            None,
        );

        assert_eq!(AstNode::id(&statement), id);
        assert_eq!(
            AstNode::kind(&statement),
            &NodeKind::core(CoreNodeKind::LetStatement)
        );
    }

    #[test]
    fn serde_round_trip_preserves_source_structure() {
        let mut statement = LetStatement::without_metadata(
            node_id(1),
            Span::default(),
            "value",
            Some(node_id(2)),
            Some(node_id(3)),
        );

        statement.push_attribute(node_id(4));
        statement.push_annotation(node_id(5));

        let encoded =
            serde_json::to_string(&statement).expect("serialize let statement");

        let decoded: LetStatement =
            serde_json::from_str(&encoded).expect("deserialize let statement");

        assert_eq!(decoded, statement);
        assert_eq!(decoded.child_node_ids(), statement.child_node_ids());
    }
}