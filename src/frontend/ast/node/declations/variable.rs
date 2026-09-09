//! # Zamani Native AST — Variable Declaration
//!
//! This module defines the source-level representation of a variable
//! declaration in the native Zamani frontend AST.
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
//! VariableDeclaration  ← this module
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
//!
//! ```
//!
//! ## Source-level responsibility
//!
//! This type represents what the programmer wrote when declaring a variable.
//!
//! It deliberately does not represent:
//!
//! - a machine register;
//! - a CPU register;
//! - a GPU register;
//! - a quantum register;
//! - a physical qubit;
//! - a logical qubit;
//! - hardware memory;
//! - a backend allocation;
//! - a scheduler allocation;
//! - a routing decision;
//! - a resource assignment;
//! - a resolved symbol;
//! - a resolved type;
//! - a ZUIR value;
//! - a runtime value.
//!
//! Those concerns belong to later compilation stages.
//!
//! ## POCO-REAF
//!
//! The representation contains no machine-size assumptions.
//!
//! Consequently, a variable can represent a value whose eventual storage or
//! computation is realized on:
//!
//! - a tiny machine;
//! - a large machine;
//! - a distributed system;
//! - a CPU;
//! - a GPU;
//! - a QPU;
//! - an accelerator;
//! - an FPGA;
//! - a future computational architecture.
//!
//! The AST records source intent. Downstream stages determine realization.
//!
//! ## Current grammar contract
//!
//! The current Zamani grammar contains:
//!
//! ```text
//! variableDeclaration
//!     : varKeyword IDENTIFIER (':' typeExpr)? '=' expression ';'
//!     | varKeyword IDENTIFIER ':' typeExpr ';'
//!     ;
//!
//! varKeyword
//!     : 'let'
//!     | 'const'
//!     | 'var'
//!     ;
//! ```
//!
//! This module therefore preserves:
//!
//! - binding kind;
//! - identifier spelling;
//! - optional type annotation;
//! - optional initializer;
//! - source metadata;
//! - attributes/annotations represented by child AST nodes.
//!
//! Semantic rules such as mutability, type compatibility, initialization
//! requirements, lifetime, ownership, borrowing, resource legality, or target
//! feasibility must be implemented by later compiler phases.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational native-AST infrastructure:
//!
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
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - backend providers;
//! - optimization;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - execution.
//!
//! ## Child representation
//!
//! Child AST objects are referenced by [`NodeId`] rather than embedded directly.
//!
//! This avoids recursive ownership structures and keeps this declaration
//! independent from the concrete implementations of expressions and types.
//!
//! The referenced nodes are:
//!
//! ```text
//! VariableDeclaration
//! ├── Node
//! ├── type_annotation ──► Type AST node
//! ├── initializer ──────► Expression AST node
//! ├── attributes ───────► Attribute AST nodes
//! └── annotations ──────► Annotation AST nodes
//! ```
//!
//! The declaration therefore does not need to import the future `Type`,
//! `Expression`, `Attribute`, or `Annotation` concrete types.
//!
//! ## Determinism
//!
//! Child collections use `Vec<NodeId>` because source order is meaningful.
//!
//! No hash-map iteration order is involved.
//!
//! Serialization preserves collection order.
//!
//! ## Scalability
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_VARIABLES
//! MAX_ATTRIBUTES
//! MAX_ANNOTATIONS
//! MAX_INITIALIZER_SIZE
//! MAX_TYPE_SIZE
//! ```
//!
//! AST scalability is governed by available resources and configurable
//! compiler safety policies outside this type.
//!
//! ## Validation boundary
//!
//! [`VariableDeclaration::validate_structure`] performs only local structural
//! validation.
//!
//! It does not:
//!
//! - resolve identifiers;
//! - resolve types;
//! - check type compatibility;
//! - infer types;
//! - check ownership;
//! - check lifetimes;
//! - check mutability semantics;
//! - determine quantum resources;
//! - determine hardware resources.
//!
//! ## Visitor/traversal contract
//!
//! [`VariableDeclaration::child_node_ids`] is the canonical local child
//! enumeration used by future visitor/traversal infrastructure.
//!
//! Traversal must preserve the following order:
//!
//! ```text
//! type annotation
//! initializer
//! attributes
//! annotations
//! ```
//!
//! This ordering is deterministic and source-structure preserving.
//!
//! ## Serialization contract
//!
//! The type derives Serde serialization.
//!
//! AST schema versioning belongs to the AST serialization layer rather than
//! being duplicated inside every individual node.
//!
//! ## Security
//!
//! This type contains no unsafe code and performs no unchecked indexing.
//!
//! It does not recursively traverse child nodes, preventing a malformed child
//! graph from causing recursion inside this local type.
//!
//! Resource limits for hostile input belong to the configurable validation and
//! compiler policy layers.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - no nightly features;
//! - no `unsafe` code.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! The parser:
//!
//! 1. allocates a `NodeId`;
//! 2. creates the declaration's `Node`;
//! 3. parses the binding kind;
//! 4. parses the identifier;
//! 5. optionally parses a type expression;
//! 6. optionally parses an initializer;
//! 7. constructs this type;
//! 8. inserts referenced child nodes into the AST graph.
//!
//! ### Structural validation
//!
//! The validator calls [`VariableDeclaration::validate_structure`] and then
//! validates referenced child nodes through the AST graph.
//!
//! ### Semantic analysis
//!
//! Semantic analysis consumes the immutable validated declaration and resolves:
//!
//! - identifier meaning;
//! - declared type;
//! - inferred type;
//! - initialization semantics;
//! - mutability;
//! - ownership;
//! - effects;
//! - capabilities;
//! - resource requirements.
//!
//! ### ZUIR
//!
//! The declaration is lowered by semantic analysis/lowering infrastructure.
//! This file must not import ZUIR.
//!
//! A variable declaration may eventually become:
//!
//! - a classical value;
//! - a resource handle;
//! - a quantum resource binding;
//! - a distributed value;
//! - an accelerator value;
//! - another future semantic value.
//!
//! That interpretation is intentionally outside the AST.
//!
//! ### Migration
//!
//! The legacy AST representation, if any, must map to exactly one
//! `VariableDeclaration` representation.
//!
//! No second competing variable AST should be introduced.
//!
//! ```text
//! legacy variable declaration
//!          │
//!          ▼
//! VariableDeclaration
//!          │
//!          ▼
//! semantic model
//! ```
//!
//! The misspelled legacy/new `declations` directory must not become a second
//! authoritative declaration location.

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Source-level binding kind used by a Zamani variable declaration.
///
/// This enum represents **language syntax**, not a physical storage class.
///
/// The semantic compiler may later interpret these forms according to the
/// language's type, ownership, mutability, lifetime, resource, and execution
/// rules.
///
/// `#[non_exhaustive]` deliberately prevents downstream code from assuming
/// that the three currently defined forms are the permanent universe of
/// Zamani binding forms.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum VariableBindingKind {
    /// `let` binding.
    Let,

    /// `var` binding.
    Var,

    /// `const` binding.
    Const,
}

impl VariableBindingKind {
    /// Returns the canonical source spelling of this binding kind.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Let => "let",
            Self::Var => "var",
            Self::Const => "const",
        }
    }
}

impl core::fmt::Display for VariableBindingKind {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A source-level Zamani variable declaration.
///
/// The declaration owns only its immediate source-level information.
/// Referenced type and expression nodes remain separate AST nodes identified
/// by [`NodeId`].
///
/// # Invariants
///
/// A structurally valid declaration:
///
/// 1. has a non-empty source identifier;
/// 2. contains a valid common [`Node`] whose kind is `Variable`;
/// 3. preserves the parser's child ordering;
/// 4. does not encode semantic resolution;
/// 5. does not encode target/hardware information.
///
/// Whether an initializer is semantically required is deliberately not
/// enforced here because that is a semantic rule rather than merely a local
/// AST-shape rule.
///
/// # Example
///
/// Conceptually:
///
/// ```text
/// let answer: Integer = expression;
/// ```
///
/// becomes:
///
/// ```text
/// VariableDeclaration
/// ├── binding_kind = Let
/// ├── name = "answer"
/// ├── type_annotation = NodeId(...)
/// └── initializer = NodeId(...)
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VariableDeclaration {
    /// Common identity, classification, source span, and metadata.
    node: Node,

    /// Source-level binding form.
    binding_kind: VariableBindingKind,

    /// Original source identifier spelling.
    ///
    /// Semantic name resolution is intentionally performed later.
    name: String,

    /// Optional source-level type annotation.
    ///
    /// The referenced node must be a type AST node.
    type_annotation: Option<NodeId>,

    /// Optional initializer expression.
    ///
    /// The referenced node must be an expression AST node.
    initializer: Option<NodeId>,

    /// Source-level attributes attached to this declaration.
    ///
    /// Attribute interpretation belongs outside this type.
    attributes: Vec<NodeId>,

    /// Source-level annotations attached to this declaration.
    ///
    /// Annotation interpretation belongs outside this type.
    annotations: Vec<NodeId>,
}

/// Backwards-friendly short name for code that prefers `Variable`.
///
/// `VariableDeclaration` remains the canonical type name because it directly
/// describes the source-language construct represented by this module.
pub type Variable = VariableDeclaration;

impl VariableDeclaration {
    /// Creates a new variable declaration.
    ///
    /// The supplied `Node` must have `CoreNodeKind::Variable`.
    ///
    /// This constructor intentionally accepts an already-created [`Node`] so
    /// the parser/builder remains responsible for deterministic node identity
    /// allocation and source-span construction.
    #[must_use]
    pub fn from_node(
        node: Node,
        binding_kind: VariableBindingKind,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        initializer: Option<NodeId>,
    ) -> Self {
        Self {
            node,
            binding_kind,
            name: name.into(),
            type_annotation,
            initializer,
            attributes: Vec::new(),
            annotations: Vec::new(),
        }
    }

    /// Creates a variable declaration directly from its foundational AST
    /// components.
    ///
    /// The node is classified as the native `Variable` node kind.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        binding_kind: VariableBindingKind,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        initializer: Option<NodeId>,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::Variable),
            span,
            metadata,
        );

        Self::from_node(
            node,
            binding_kind,
            name,
            type_annotation,
            initializer,
        )
    }

    /// Creates a variable declaration with default metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        binding_kind: VariableBindingKind,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        initializer: Option<NodeId>,
    ) -> Self {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            binding_kind,
            name,
            type_annotation,
            initializer,
        )
    }

    /// Returns the binding kind.
    #[must_use]
    pub const fn binding_kind(&self) -> VariableBindingKind {
        self.binding_kind
    }

    /// Returns the original source identifier spelling.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the optional type annotation node.
    #[must_use]
    pub const fn type_annotation(&self) -> Option<NodeId> {
        self.type_annotation
    }

    /// Returns the optional initializer node.
    #[must_use]
    pub const fn initializer(&self) -> Option<NodeId> {
        self.initializer
    }

    /// Returns all attached attribute node IDs in source order.
    #[must_use]
    pub fn attributes(&self) -> &[NodeId] {
        &self.attributes
    }

    /// Returns all attached annotation node IDs in source order.
    #[must_use]
    pub fn annotations(&self) -> &[NodeId] {
        &self.annotations
    }

    /// Sets the source identifier.
    ///
    /// Empty identifiers are rejected because a variable declaration cannot
    /// structurally represent a named binding without a name.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), VariableValidationError> {
        let name = name.into();

        if name.is_empty() {
            return Err(VariableValidationError::EmptyName);
        }

        self.name = name;
        Ok(())
    }

    /// Sets the binding kind.
    pub fn set_binding_kind(&mut self, binding_kind: VariableBindingKind) {
        self.binding_kind = binding_kind;
    }

    /// Sets or removes the type annotation.
    pub fn set_type_annotation(&mut self, type_annotation: Option<NodeId>) {
        self.type_annotation = type_annotation;
    }

    /// Sets or removes the initializer.
    pub fn set_initializer(&mut self, initializer: Option<NodeId>) {
        self.initializer = initializer;
    }

    /// Adds an attribute while preserving source order.
    pub fn push_attribute(&mut self, attribute: NodeId) {
        self.attributes.push(attribute);
    }

    /// Adds an annotation while preserving source order.
    pub fn push_annotation(&mut self, annotation: NodeId) {
        self.annotations.push(annotation);
    }

    /// Removes all attributes.
    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    /// Removes all annotations.
    pub fn clear_annotations(&mut self) {
        self.annotations.clear();
    }

    /// Returns the number of attached attributes.
    #[must_use]
    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }

    /// Returns the number of attached annotations.
    #[must_use]
    pub fn annotation_count(&self) -> usize {
        self.annotations.len()
    }

    /// Returns an iterator over immediate child node IDs.
    ///
    /// Traversal order is deterministic:
    ///
    /// 1. type annotation;
    /// 2. initializer;
    /// 3. attributes;
    /// 4. annotations.
    ///
    /// The declaration itself is not yielded.
    pub fn child_node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.type_annotation
            .into_iter()
            .chain(self.initializer)
            .chain(self.attributes.iter().copied())
            .chain(self.annotations.iter().copied())
    }

    /// Returns the number of immediate child references.
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.type_annotation.is_some() as usize
            + self.initializer.is_some() as usize
            + self.attributes.len()
            + self.annotations.len()
    }

    /// Validates local structural invariants.
    ///
    /// This method intentionally does not inspect the referenced child nodes.
    /// Graph-level validation belongs to the AST structural validation layer.
    pub fn validate_structure(&self) -> Result<(), VariableValidationError> {
        if self.name.is_empty() {
            return Err(VariableValidationError::EmptyName);
        }

        if !self.node.is_kind(&NodeKind::core(CoreNodeKind::Variable)) {
            return Err(VariableValidationError::InvalidNodeKind);
        }

        Ok(())
    }

    /// Returns whether the declaration has an explicit type annotation.
    #[must_use]
    pub const fn has_type_annotation(&self) -> bool {
        self.type_annotation.is_some()
    }

    /// Returns whether the declaration has an initializer.
    #[must_use]
    pub const fn has_initializer(&self) -> bool {
        self.initializer.is_some()
    }

    /// Returns whether this declaration contains source-level attributes or
    /// annotations.
    #[must_use]
    pub fn has_metadata_nodes(&self) -> bool {
        !self.attributes.is_empty() || !self.annotations.is_empty()
    }

    /// Returns the canonical AST node kind.
    #[must_use]
    pub const fn node_kind() -> NodeKind {
        NodeKind::Core(CoreNodeKind::Variable)
    }

    /// Returns a source-oriented description suitable for diagnostics.
    ///
    /// This method intentionally does not include semantic type information,
    /// resolved symbols, or backend information.
    #[must_use]
    pub fn diagnostic_description(&self) -> String {
        let mut result = String::new();

        result.push_str(self.binding_kind.as_str());
        result.push(' ');
        result.push_str(&self.name);

        if self.type_annotation.is_some() {
            result.push_str(": <type>");
        }

        if self.initializer.is_some() {
            result.push_str(" = <expression>");
        }

        result
    }
}

/// Structural validation failures local to a variable declaration.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VariableValidationError {
    /// The source identifier is empty.
    EmptyName,

    /// The embedded common node does not identify itself as a variable node.
    InvalidNodeKind,
}

impl core::fmt::Display for VariableValidationError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str(
                    "variable declaration must contain a non-empty identifier",
                )
            }

            Self::InvalidNodeKind => {
                formatter.write_str(
                    "variable declaration has an invalid AST node kind",
                )
            }
        }
    }
}

impl std::error::Error for VariableValidationError {}

impl AstNode for VariableDeclaration {
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::node_id::NodeIdAllocator;
    use super::super::super::source::{SourceId, SourceOffset};

    fn test_span() -> Span {
        Span::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(10),
        )
        .expect("test span must be valid")
    }

    fn test_id(allocator: &mut NodeIdAllocator) -> NodeId {
        allocator
            .allocate()
            .expect("test allocator must have capacity")
    }

    #[test]
    fn creates_let_variable() {
        let mut allocator = NodeIdAllocator::new();

        let variable = VariableDeclaration::without_metadata(
            test_id(&mut allocator),
            test_span(),
            VariableBindingKind::Let,
            "value",
            None,
            None,
        );

        assert_eq!(variable.binding_kind(), VariableBindingKind::Let);
        assert_eq!(variable.name(), "value");
        assert!(!variable.has_type_annotation());
        assert!(!variable.has_initializer());
        assert!(variable.validate_structure().is_ok());
    }

    #[test]
    fn creates_typed_variable() {
        let mut allocator = NodeIdAllocator::new();

        let variable_id = test_id(&mut allocator);
        let type_id = test_id(&mut allocator);

        let variable = VariableDeclaration::without_metadata(
            variable_id,
            test_span(),
            VariableBindingKind::Let,
            "value",
            Some(type_id),
            None,
        );

        assert_eq!(variable.type_annotation(), Some(type_id));
        assert!(variable.has_type_annotation());
        assert!(!variable.has_initializer());
    }

    #[test]
    fn creates_initialized_variable() {
        let mut allocator = NodeIdAllocator::new();

        let variable_id = test_id(&mut allocator);
        let expression_id = test_id(&mut allocator);

        let variable = VariableDeclaration::without_metadata(
            variable_id,
            test_span(),
            VariableBindingKind::Var,
            "value",
            None,
            Some(expression_id),
        );

        assert_eq!(variable.initializer(), Some(expression_id));
        assert!(variable.has_initializer());
    }

    #[test]
    fn supports_type_and_initializer_together() {
        let mut allocator = NodeIdAllocator::new();

        let variable_id = test_id(&mut allocator);
        let type_id = test_id(&mut allocator);
        let expression_id = test_id(&mut allocator);

        let variable = VariableDeclaration::without_metadata(
            variable_id,
            test_span(),
            VariableBindingKind::Let,
            "value",
            Some(type_id),
            Some(expression_id),
        );

        assert_eq!(variable.type_annotation(), Some(type_id));
        assert_eq!(variable.initializer(), Some(expression_id));
        assert_eq!(variable.child_count(), 2);
    }

    #[test]
    fn preserves_child_traversal_order() {
        let mut allocator = NodeIdAllocator::new();

        let variable_id = test_id(&mut allocator);
        let type_id = test_id(&mut allocator);
        let expression_id = test_id(&mut allocator);
        let attribute_id = test_id(&mut allocator);
        let annotation_id = test_id(&mut allocator);

        let mut variable = VariableDeclaration::without_metadata(
            variable_id,
            test_span(),
            VariableBindingKind::Let,
            "value",
            Some(type_id),
            Some(expression_id),
        );

        variable.push_attribute(attribute_id);
        variable.push_annotation(annotation_id);

        let children: Vec<NodeId> = variable.child_node_ids().collect();

        assert_eq!(
            children,
            vec![
                type_id,
                expression_id,
                attribute_id,
                annotation_id,
            ]
        );
    }

    #[test]
    fn supports_arbitrarily_many_attributes_without_ast_limit() {
        let mut allocator = NodeIdAllocator::new();

        let variable_id = test_id(&mut allocator);

        let mut variable = VariableDeclaration::without_metadata(
            variable_id,
            test_span(),
            VariableBindingKind::Let,
            "value",
            None,
            None,
        );

        for _ in 0..10_000 {
            variable.push_attribute(test_id(&mut allocator));
        }

        assert_eq!(variable.attribute_count(), 10_000);
        assert_eq!(variable.child_count(), 10_000);
        assert!(variable.validate_structure().is_ok());
    }

    #[test]
    fn rejects_empty_name() {
        let mut allocator = NodeIdAllocator::new();

        let variable = VariableDeclaration::without_metadata(
            test_id(&mut allocator),
            test_span(),
            VariableBindingKind::Let,
            "",
            None,
            None,
        );

        assert_eq!(
            variable.validate_structure(),
            Err(VariableValidationError::EmptyName)
        );
    }

    #[test]
    fn binding_kind_has_stable_source_spelling() {
        assert_eq!(VariableBindingKind::Let.as_str(), "let");
        assert_eq!(VariableBindingKind::Var.as_str(), "var");
        assert_eq!(VariableBindingKind::Const.as_str(), "const");
    }

    #[test]
    fn node_kind_is_variable() {
        assert_eq!(
            VariableDeclaration::node_kind(),
            NodeKind::Core(CoreNodeKind::Variable)
        );
    }

    #[test]
    fn implements_ast_node() {
        let mut allocator = NodeIdAllocator::new();
        let id = test_id(&mut allocator);

        let variable = VariableDeclaration::without_metadata(
            id,
            test_span(),
            VariableBindingKind::Let,
            "value",
            None,
            None,
        );

        assert_eq!(variable.id(), id);
        assert_eq!(
            variable.kind(),
            &NodeKind::Core(CoreNodeKind::Variable)
        );
    }

    #[test]
    fn equality_is_deterministic() {
        let mut allocator = NodeIdAllocator::new();

        let id = test_id(&mut allocator);

        let first = VariableDeclaration::without_metadata(
            id,
            test_span(),
            VariableBindingKind::Let,
            "value",
            None,
            None,
        );

        let second = first.clone();

        assert_eq!(first, second);
    }

    #[test]
    fn diagnostic_description_is_source_oriented() {
        let mut allocator = NodeIdAllocator::new();

        let variable = VariableDeclaration::without_metadata(
            test_id(&mut allocator),
            test_span(),
            VariableBindingKind::Let,
            "answer",
            None,
            None,
        );

        assert_eq!(
            variable.diagnostic_description(),
            "let answer"
        );
    }
}