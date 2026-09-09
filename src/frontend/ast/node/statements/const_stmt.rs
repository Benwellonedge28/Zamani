//! # Zamani Native AST — Constant Statement
//!
//! Source-level representation of a `const` declaration statement.
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
//! ConstStatement  ← this module
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
//!     ├── HDL IR
//!     └── future domain IRs
//! ```
//!
//! ## Responsibility
//!
//! This module represents the source-level structure and intent of a Zamani
//! `const` statement.
//!
//! It deliberately does **not** represent:
//!
//! - a CPU register;
//! - a GPU register;
//! - a hardware memory location;
//! - a quantum register;
//! - a physical qubit;
//! - a logical qubit;
//! - a backend allocation;
//! - a resource assignment;
//! - a scheduler allocation;
//! - a routing decision;
//! - a calibration;
//! - a QEC implementation;
//! - a runtime value;
//! - a resolved symbol;
//! - a resolved type;
//! - a ZUIR value;
//! - a QIR value;
//! - an LLVM value;
//! - an MLIR operation;
//! - a vendor-specific instruction.
//!
//! Those concerns belong to later compilation stages.
//!
//! ## POCO-REAF
//!
//! A constant declaration is source-level information only. Its eventual
//! realization may occur on any computational target supported by later
//! compiler stages.
//!
//! The representation therefore contains no:
//!
//! - fixed machine size;
//! - fixed register size;
//! - fixed memory size;
//! - fixed qubit count;
//! - fixed topology;
//! - fixed instruction set;
//! - fixed vendor;
//! - fixed backend;
//! - fixed computational domain.
//!
//! The same AST representation can consequently participate in compilation
//! for tiny, large, distributed, heterogeneous, quantum, classical,
//! accelerator, or future computational systems, subject only to the
//! capabilities and resources available downstream.
//!
//! ## Child representation
//!
//! Child nodes are represented by [`NodeId`] rather than recursively embedded
//! AST objects.
//!
//! ```text
//! ConstStatement
//! ├── Node
//! ├── type_annotation ──► Type AST node
//! ├── value ─────────────► Expression AST node
//! ├── attributes ───────► Attribute AST nodes
//! └── annotations ──────► Annotation AST nodes
//! ```
//!
//! This keeps this module independent of concrete expression, type,
//! attribute, and annotation implementations.
//!
//! ## Structural versus semantic validation
//!
//! This module validates only invariants that are local to this AST node.
//!
//! It does **not** perform:
//!
//! - name resolution;
//! - type resolution;
//! - constant folding;
//! - compile-time evaluation;
//! - type compatibility checking;
//! - ownership checking;
//! - lifetime checking;
//! - capability checking;
//! - resource allocation;
//! - quantum analysis;
//! - hardware validation.
//!
//! Those belong to subsequent compiler phases.
//!
//! ## Dependency contract
//!
//! This file may depend only on:
//!
//! - [`Node`];
//! - [`AstNode`];
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
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - execution;
//! - backend providers.
//!
//! ## Determinism
//!
//! Child nodes are exposed in deterministic source-structure order:
//!
//! ```text
//! type annotation
//! value
//! attributes
//! annotations
//! ```
//!
//! No hash-map iteration or runtime-dependent ordering is involved.
//!
//! ## Scalability
//!
//! This type intentionally contains no constants such as:
//!
//! ```text
//! MAX_CONSTS
//! MAX_ATTRIBUTES
//! MAX_ANNOTATIONS
//! MAX_VALUE_SIZE
//! MAX_TYPE_SIZE
//! MAX_MACHINES
//! MAX_QUBITS
//! ```
//!
//! AST scalability is governed by the compiler's configurable resource and
//! safety policies, not by this source-level node.
//!
//! ## Security
//!
//! This module contains no `unsafe` code and performs no unchecked indexing.
//! It does not recursively walk referenced nodes, so malformed child graphs
//! cannot cause recursive traversal inside this type.
//!
//! Resource limits for hostile input belong to the configurable AST validation
//! and compiler-policy layers.
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
//! ## Parser integration contract
//!
//! The parser is responsible for:
//!
//! 1. allocating a [`NodeId`];
//! 2. constructing the common [`Node`];
//! 3. parsing the `const` keyword;
//! 4. parsing the source identifier;
//! 5. optionally parsing a type annotation;
//! 6. parsing the required initializer expression;
//! 7. constructing this node;
//! 8. inserting referenced child nodes into the canonical AST store.
//!
//! The parser must not resolve the identifier or determine its eventual
//! computational representation.
//!
//! ## Structural validation integration
//!
//! The AST structural validator must:
//!
//! 1. validate the embedded [`Node`];
//! 2. verify the node kind is `CoreNodeKind::ConstStatement`;
//! 3. verify the name is non-empty;
//! 4. verify the type annotation reference, when present, exists;
//! 5. verify the value reference exists;
//! 6. recursively validate referenced nodes through the canonical AST store.
//!
//! The local [`ConstStatement::validate_structure`] method intentionally does
//! not inspect the global AST store because doing so would introduce an
//! unwanted dependency on the AST container.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes an immutable, structurally validated
//! `ConstStatement` and determines:
//!
//! - the declaration's symbol;
//! - its declared or inferred type;
//! - whether the initializer is semantically constant;
//! - constant-evaluation requirements;
//! - effects;
//! - capabilities;
//! - resource requirements;
//! - ownership/lifetime semantics where applicable.
//!
//! None of those concepts are stored in this AST node.
//!
//! ## ZUIR integration
//!
//! This module must not import ZUIR.
//!
//! Semantic lowering determines how a constant becomes a ZUIR construct.
//! Depending on the program and domain, the resulting semantic value may
//! eventually participate in:
//!
//! - classical computation;
//! - quantum/classical control;
//! - resource descriptions;
//! - distributed computation;
//! - accelerator execution;
//! - HDL generation;
//! - future computational domains.
//!
//! The AST does not choose among those realizations.
//!
//! ## Visitor/traversal integration
//!
//! [`ConstStatement::child_node_ids`] is the canonical local child enumeration.
//!
//! Traversal order is deterministic:
//!
//! ```text
//! type_annotation → value → attributes → annotations
//! ```
//!
//! The method returns an iterator rather than allocating a temporary child
//! vector, which keeps repeated traversal allocation-free at this node.
//!
//! ## Serialization integration
//!
//! The type derives Serde serialization and deserialization.
//!
//! AST schema versioning belongs to the centralized AST serialization layer.
//! It is deliberately not duplicated inside this node.
//!
//! ## Migration contract
//!
//! This is the canonical native AST representation of a `const` statement.
//!
//! The legacy AST must map its constant-declaration representation to exactly
//! one `ConstStatement`.
//!
//! The misspelled legacy `declations/` directory must not become a second
//! authoritative representation for constant statements.
//!
//! ```text
//! legacy const declaration
//!          │
//!          ▼
//! ConstStatement
//!          │
//!          ▼
//! semantic model
//! ```
//!
//! ## Integration files
//!
//! This file is intended to be registered by:
//!
//! ```text
//! src/frontend/ast/node/statements/mod.rs
//! ```
//!
//! and then exposed by the appropriate parent module:
//!
//! ```text
//! src/frontend/ast/node/mod.rs
//! ```
//!
//! Neither of those modules should duplicate the implementation below.
//!
//! The parser should depend on the public `ConstStatement` API. Semantic
//! analysis should consume it through the AST abstraction. ZUIR lowering must
//! remain downstream.
//!
//! ## Important distinction
//!
//! `CoreNodeKind::Constant` and `CoreNodeKind::ConstStatement` are not
//! interchangeable concepts.
//!
//! `ConstStatement` is the source-language statement represented here.
//! `Constant` may represent a different declaration/category elsewhere in the
//! native AST. This file therefore uses the repository's dedicated
//! `CoreNodeKind::ConstStatement` classification.

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Source-level constant statement.
///
/// A constant statement always has a source-level name and an initializer
/// expression. An explicit type annotation is optional and is resolved later
/// by semantic analysis.
///
/// This type represents source structure, not compile-time evaluation itself.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConstStatement {
    /// Common AST identity, classification, source span and metadata.
    node: Node,

    /// Original source identifier spelling.
    ///
    /// Name resolution is deliberately deferred to semantic analysis.
    name: String,

    /// Optional explicit source-level type annotation.
    ///
    /// The referenced node must be a type AST node.
    type_annotation: Option<NodeId>,

    /// Required initializer expression.
    ///
    /// The referenced node must be an expression AST node.
    value: NodeId,

    /// Source-level attributes attached to the statement.
    ///
    /// Attribute interpretation is performed by later compiler stages.
    attributes: Vec<NodeId>,

    /// Source-level annotations attached to the statement.
    ///
    /// Annotation interpretation is performed by later compiler stages.
    annotations: Vec<NodeId>,
}

/// Alias for code that prefers the shorter `Const` name.
///
/// `ConstStatement` remains the canonical public type because it precisely
/// describes the AST construct.
pub type Const = ConstStatement;

impl ConstStatement {
    /// Constructs a constant statement from an existing canonical [`Node`].
    ///
    /// The supplied node must be classified as
    /// `CoreNodeKind::ConstStatement`.
    ///
    /// Use [`Self::try_from_node`] when the node kind cannot be guaranteed by
    /// the caller.
    ///
    /// # Panics
    ///
    /// This constructor does not panic. It is retained as a convenience for
    /// construction paths where the parser has already established the node
    /// classification.
    ///
    /// For untrusted or generic construction use [`Self::try_from_node`].
    #[must_use]
    pub fn from_node(
        node: Node,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        value: NodeId,
    ) -> Self {
        Self {
            node,
            name: name.into(),
            type_annotation,
            value,
            attributes: Vec::new(),
            annotations: Vec::new(),
        }
    }

    /// Safely constructs a constant statement from an existing node.
    ///
    /// This validates the two local constructor invariants that can be checked
    /// without access to the global AST store:
    ///
    /// - the node kind is `ConstStatement`;
    /// - the source name is non-empty.
    pub fn try_from_node(
        node: Node,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        value: NodeId,
    ) -> Result<Self, ConstStatementError> {
        let name = name.into();

        if !matches!(
            node.kind().as_core(),
            Some(CoreNodeKind::ConstStatement)
        ) {
            return Err(ConstStatementError::WrongNodeKind {
                expected: CoreNodeKind::ConstStatement,
                actual: node.kind_owned(),
            });
        }

        if name.is_empty() {
            return Err(ConstStatementError::EmptyName);
        }

        if value == NodeId::INVALID {
            return Err(ConstStatementError::InvalidValueId);
        }

        if matches!(type_annotation, Some(NodeId::INVALID)) {
            return Err(ConstStatementError::InvalidTypeAnnotationId);
        }

        Ok(Self {
            node,
            name,
            type_annotation,
            value,
            attributes: Vec::new(),
            annotations: Vec::new(),
        })
    }

    /// Creates a constant statement from foundational AST components.
    ///
    /// The node receives the repository's dedicated
    /// `CoreNodeKind::ConstStatement` classification.
    ///
    /// Construction validates all locally knowable invariants and therefore
    /// returns a `Result` instead of hiding invalid AST state behind a panic.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        value: NodeId,
    ) -> Result<Self, ConstStatementError> {
        let name = name.into();

        if name.is_empty() {
            return Err(ConstStatementError::EmptyName);
        }

        if value == NodeId::INVALID {
            return Err(ConstStatementError::InvalidValueId);
        }

        if matches!(type_annotation, Some(NodeId::INVALID)) {
            return Err(ConstStatementError::InvalidTypeAnnotationId);
        }

        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::ConstStatement),
            span,
            metadata,
        );

        Ok(Self::from_node(
            node,
            name,
            type_annotation,
            value,
        ))
    }

    /// Creates a constant statement with default node metadata.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        value: NodeId,
    ) -> Result<Self, ConstStatementError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            name,
            type_annotation,
            value,
        )
    }

    /// Returns the source identifier spelling.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Replaces the source identifier.
    ///
    /// This operation does not perform semantic name validation. Lexer/parser
    /// identifier legality belongs to the language frontend.
    ///
    /// The only local invariant enforced here is that the identifier cannot be
    /// empty.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), ConstStatementError> {
        let name = name.into();

        if name.is_empty() {
            return Err(ConstStatementError::EmptyName);
        }

        self.name = name;
        Ok(())
    }

    /// Returns the optional explicit type annotation.
    #[inline]
    #[must_use]
    pub const fn type_annotation(&self) -> Option<NodeId> {
        self.type_annotation
    }

    /// Replaces the optional explicit type annotation.
    ///
    /// `None` removes the annotation.
    pub fn set_type_annotation(
        &mut self,
        type_annotation: Option<NodeId>,
    ) -> Result<(), ConstStatementError> {
        if matches!(type_annotation, Some(NodeId::INVALID)) {
            return Err(ConstStatementError::InvalidTypeAnnotationId);
        }

        self.type_annotation = type_annotation;
        Ok(())
    }

    /// Returns the required initializer expression node.
    #[inline]
    #[must_use]
    pub const fn value(&self) -> NodeId {
        self.value
    }

    /// Replaces the initializer expression.
    ///
    /// The value must always reference a real AST node. Whether that node is
    /// semantically a valid constant expression is decided later.
    pub fn set_value(
        &mut self,
        value: NodeId,
    ) -> Result<(), ConstStatementError> {
        if value == NodeId::INVALID {
            return Err(ConstStatementError::InvalidValueId);
        }

        self.value = value;
        Ok(())
    }

    /// Returns attributes in source order.
    #[inline]
    #[must_use]
    pub fn attributes(&self) -> &[NodeId] {
        &self.attributes
    }

    /// Returns annotations in source order.
    #[inline]
    #[must_use]
    pub fn annotations(&self) -> &[NodeId] {
        &self.annotations
    }

    /// Appends an attribute while preserving source order.
    pub fn push_attribute(
        &mut self,
        attribute: NodeId,
    ) -> Result<(), ConstStatementError> {
        if attribute == NodeId::INVALID {
            return Err(ConstStatementError::InvalidAttributeId);
        }

        self.attributes.push(attribute);
        Ok(())
    }

    /// Appends an annotation while preserving source order.
    pub fn push_annotation(
        &mut self,
        annotation: NodeId,
    ) -> Result<(), ConstStatementError> {
        if annotation == NodeId::INVALID {
            return Err(ConstStatementError::InvalidAnnotationId);
        }

        self.annotations.push(annotation);
        Ok(())
    }

    /// Removes all attributes.
    ///
    /// This is intentionally explicit rather than replacing the vector with a
    /// new allocation.
    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    /// Removes all annotations.
    pub fn clear_annotations(&mut self) {
        self.annotations.clear();
    }

    /// Returns the deterministic local child-node sequence.
    ///
    /// The order is:
    ///
    /// 1. type annotation, when present;
    /// 2. value;
    /// 3. attributes in source order;
    /// 4. annotations in source order.
    ///
    /// This method does not allocate a temporary vector.
    pub fn child_node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.type_annotation
            .into_iter()
            .chain(std::iter::once(self.value))
            .chain(self.attributes.iter().copied())
            .chain(self.annotations.iter().copied())
    }

    /// Validates invariants that can be checked without access to the global
    /// AST store.
    ///
    /// Cross-node validation, including checking that child IDs actually exist
    /// and point to nodes of the appropriate kinds, belongs to the AST
    /// validation layer.
    pub fn validate_structure(&self) -> Result<(), ConstStatementError> {
        if !matches!(
            self.node.kind().as_core(),
            Some(CoreNodeKind::ConstStatement)
        ) {
            return Err(ConstStatementError::WrongNodeKind {
                expected: CoreNodeKind::ConstStatement,
                actual: self.node.kind_owned(),
            });
        }

        if self.name.is_empty() {
            return Err(ConstStatementError::EmptyName);
        }

        if self.value == NodeId::INVALID {
            return Err(ConstStatementError::InvalidValueId);
        }

        if matches!(self.type_annotation, Some(NodeId::INVALID)) {
            return Err(ConstStatementError::InvalidTypeAnnotationId);
        }

        if self.attributes.iter().any(|id| *id == NodeId::INVALID) {
            return Err(ConstStatementError::InvalidAttributeId);
        }

        if self.annotations.iter().any(|id| *id == NodeId::INVALID) {
            return Err(ConstStatementError::InvalidAnnotationId);
        }

        Ok(())
    }
}

impl AstNode for ConstStatement {
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

/// Errors produced by local constant-statement construction or validation.
///
/// These errors deliberately contain no semantic-analysis concepts. They
/// describe only malformed source-AST structure.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConstStatementError {
    /// A constant statement cannot have an empty source identifier.
    EmptyName,

    /// The referenced initializer expression does not have a valid AST ID.
    InvalidValueId,

    /// The optional type annotation contains an invalid AST ID.
    InvalidTypeAnnotationId,

    /// An attribute reference contains an invalid AST ID.
    InvalidAttributeId,

    /// An annotation reference contains an invalid AST ID.
    InvalidAnnotationId,

    /// The supplied common node has the wrong AST classification.
    WrongNodeKind {
        /// Required classification.
        expected: CoreNodeKind,

        /// Actual classification.
        actual: NodeKind,
    },
}

impl core::fmt::Display for ConstStatementError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str(
                    "constant statement name must not be empty",
                )
            }

            Self::InvalidValueId => {
                formatter.write_str(
                    "constant statement value must reference a valid AST node",
                )
            }

            Self::InvalidTypeAnnotationId => {
                formatter.write_str(
                    "constant statement type annotation must reference a valid AST node",
                )
            }

            Self::InvalidAttributeId => {
                formatter.write_str(
                    "constant statement attribute must reference a valid AST node",
                )
            }

            Self::InvalidAnnotationId => {
                formatter.write_str(
                    "constant statement annotation must reference a valid AST node",
                )
            }

            Self::WrongNodeKind { expected, actual } => {
                write!(
                    formatter,
                    "constant statement requires node kind `{}`, found `{}`",
                    expected.name(),
                    actual
                )
            }
        }
    }
}

impl std::error::Error for ConstStatementError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test NodeId must be valid")
    }

    fn test_span() -> Span {
        Span::default()
    }

    #[test]
    fn constructs_valid_const_statement() {
        let statement = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "answer",
            Some(test_node_id(2)),
            test_node_id(3),
        )
        .expect("valid constant statement");

        assert_eq!(statement.name(), "answer");
        assert_eq!(
            statement.type_annotation(),
            Some(test_node_id(2))
        );
        assert_eq!(statement.value(), test_node_id(3));
        assert_eq!(
            statement.kind().as_core(),
            Some(CoreNodeKind::ConstStatement)
        );
    }

    #[test]
    fn rejects_empty_name() {
        let result = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "",
            None,
            test_node_id(2),
        );

        assert_eq!(
            result,
            Err(ConstStatementError::EmptyName)
        );
    }

    #[test]
    fn rejects_invalid_value_id() {
        let result = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "answer",
            None,
            NodeId::INVALID,
        );

        assert_eq!(
            result,
            Err(ConstStatementError::InvalidValueId)
        );
    }

    #[test]
    fn rejects_invalid_type_annotation_id() {
        let result = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "answer",
            Some(NodeId::INVALID),
            test_node_id(2),
        );

        assert_eq!(
            result,
            Err(ConstStatementError::InvalidTypeAnnotationId)
        );
    }

    #[test]
    fn child_order_is_deterministic() {
        let mut statement = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "answer",
            Some(test_node_id(2)),
            test_node_id(3),
        )
        .expect("valid constant statement");

        statement
            .push_attribute(test_node_id(4))
            .expect("valid attribute");

        statement
            .push_attribute(test_node_id(5))
            .expect("valid attribute");

        statement
            .push_annotation(test_node_id(6))
            .expect("valid annotation");

        let children: Vec<NodeId> =
            statement.child_node_ids().collect();

        assert_eq!(
            children,
            vec![
                test_node_id(2),
                test_node_id(3),
                test_node_id(4),
                test_node_id(5),
                test_node_id(6),
            ]
        );
    }

    #[test]
    fn child_order_without_type_annotation_is_deterministic() {
        let mut statement = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "answer",
            None,
            test_node_id(2),
        )
        .expect("valid constant statement");

        statement
            .push_attribute(test_node_id(3))
            .expect("valid attribute");

        statement
            .push_annotation(test_node_id(4))
            .expect("valid annotation");

        let children: Vec<NodeId> =
            statement.child_node_ids().collect();

        assert_eq!(
            children,
            vec![
                test_node_id(2),
                test_node_id(3),
                test_node_id(4),
            ]
        );
    }

    #[test]
    fn rejects_invalid_attribute_id() {
        let mut statement = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "answer",
            None,
            test_node_id(2),
        )
        .expect("valid constant statement");

        assert_eq!(
            statement.push_attribute(NodeId::INVALID),
            Err(ConstStatementError::InvalidAttributeId)
        );
    }

    #[test]
    fn rejects_invalid_annotation_id() {
        let mut statement = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "answer",
            None,
            test_node_id(2),
        )
        .expect("valid constant statement");

        assert_eq!(
            statement.push_annotation(NodeId::INVALID),
            Err(ConstStatementError::InvalidAnnotationId)
        );
    }

    #[test]
    fn changing_value_preserves_statement_identity() {
        let mut statement = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "answer",
            None,
            test_node_id(2),
        )
        .expect("valid constant statement");

        let original_id = statement.id();

        statement
            .set_value(test_node_id(3))
            .expect("valid replacement value");

        assert_eq!(statement.id(), original_id);
        assert_eq!(statement.value(), test_node_id(3));
    }

    #[test]
    fn local_structure_validation_succeeds() {
        let statement = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "answer",
            Some(test_node_id(2)),
            test_node_id(3),
        )
        .expect("valid constant statement");

        assert!(statement.validate_structure().is_ok());
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let mut statement = ConstStatement::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            "answer",
            Some(test_node_id(2)),
            test_node_id(3),
        )
        .expect("valid constant statement");

        statement
            .push_attribute(test_node_id(4))
            .expect("valid attribute");

        statement
            .push_annotation(test_node_id(5))
            .expect("valid annotation");

        let encoded =
            serde_json::to_string(&statement)
                .expect("serialization must succeed");

        let decoded: ConstStatement =
            serde_json::from_str(&encoded)
                .expect("deserialization must succeed");

        assert_eq!(decoded, statement);
    }
}