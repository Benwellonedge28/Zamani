//! # Zamani Frontend AST — Struct Declaration
//!
//! Production-grade source-level representation of a Zamani `struct`
//! declaration.
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
//! native Zamani AST
//!     │
//!     ├── StructDeclaration  ← this module
//!     │       ├── Node
//!     │       ├── generic parameters (NodeId references)
//!     │       └── fields (NodeId references)
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
//!     ▼
//! domain IR
//!     │
//!     ▼
//! target / runtime / hardware
//! ```
//!
//! ## Design principle
//!
//! A struct declaration is a **source-language construct**. It must describe
//! programmer intent without embedding:
//!
//! - machine size;
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - ASIC architecture;
//! - quantum topology;
//! - qubit count;
//! - physical qubit identifiers;
//! - backend/provider identifiers;
//! - scheduling information;
//! - routing information;
//! - calibration information;
//! - QEC implementation;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - runtime state.
//!
//! Consequently, this type is suitable for ordinary classical structures,
//! resource-oriented structures, hybrid programs, quantum-related source
//! abstractions, HDL-facing source abstractions, and future computational
//! domains.
//!
//! ## POCO-REAF
//!
//! The representation contains no fixed resource cardinality and no target
//! assumptions. A struct can therefore describe a source-level abstraction
//! independently of the size or technology of the eventual machine.
//!
//! ```text
//! Program once
//!      │
//!      ▼
//! StructDeclaration
//!      │
//!      ▼
//! semantic resolution
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ├── tiny target
//!      ├── large target
//!      ├── heterogeneous target
//!      ├── quantum target
//!      ├── distributed target
//!      └── future target
//! ```
//!
//! ## Important AST-graph rule
//!
//! This declaration does **not** duplicate field/type AST nodes.
//!
//! Fields and their types are represented by [`NodeId`] references into the
//! canonical AST store. This follows the repository's existing `Item`
//! architecture, where program/module membership is represented by references
//! to canonical nodes rather than duplicate node ownership.
//!
//! This gives the invariant:
//!
//! > Every concrete AST node has one authoritative representation.
//!
//! The enclosing AST store owns the actual field/type nodes.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational AST infrastructure:
//!
//! - [`super::super::node::Node`];
//! - [`super::super::node_id::NodeId`];
//! - [`super::super::node_kind::{CoreNodeKind, NodeKind}`];
//! - [`super::super::metadata::NodeMetadata`];
//! - [`super::super::source::Span`];
//! - Rust standard-library facilities;
//! - `serde`.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - backend providers;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - resilience;
//! - runtime;
//! - compiler execution state.
//!
//! ## Integration contract
//!
//! Parser:
//!
//! ```text
//! parser
//!   │
//!   ├── allocates declaration NodeId
//!   ├── parses name
//!   ├── parses generic-parameter NodeIds
//!   ├── parses field NodeIds
//!   └── constructs StructDeclaration
//! ```
//!
//! Structural validation:
//!
//! ```text
//! StructDeclaration
//!      │
//!      ├── validates local invariants
//!      └── validates references through AST-store validation
//! ```
//!
//! Semantic analysis:
//!
//! ```text
//! StructDeclaration
//!      │
//!      ├── resolves name
//!      ├── resolves generic parameters
//!      ├── resolves field declarations
//!      └── resolves field types
//! ```
//!
//! ZUIR:
//!
//! ```text
//! StructDeclaration
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR type/data model
//! ```
//!
//! `StructDeclaration` itself must never perform semantic resolution or target
//! lowering.
//!
//! ## Scalability
//!
//! There is no fixed number of:
//!
//! - structs;
//! - fields;
//! - generic parameters;
//! - nested declarations;
//! - source files;
//! - machines;
//! - computational resources.
//!
//! Collections therefore use `Vec<T>` and grow according to available memory.
//!
//! No language-semantic maximum is imposed by this module.
//!
//! Compiler-wide hostile-input/resource limits belong in configurable compiler
//! policy infrastructure.
//!
//! ## Determinism
//!
//! Field and generic-parameter order is represented explicitly by `Vec` order.
//!
//! No hash-map iteration order participates in declaration identity or
//! serialization.
//!
//! Constructors do not generate IDs, timestamps, random values, pointers, or
//! memory-address-derived identities.
//!
//! ## Safety
//!
//! This module contains no `unsafe` code.
//!
//! It performs no unchecked indexing and does not dereference arbitrary
//! pointers.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! The implementation uses stable Rust APIs only.

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version for the source-level struct-declaration contract.
///
/// This version is independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - AST serialization version;
/// - extension versions.
pub const STRUCT_DECLARATION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level name of a struct declaration.
pub const STRUCT_DECLARATION_KIND_NAME: &str = "zamani:struct";

/// A source-level Zamani struct declaration.
///
/// The declaration owns only the structure needed to describe the declaration
/// itself. Referenced generic parameters and fields remain canonical AST nodes
/// identified by [`NodeId`].
///
/// # Invariants
///
/// A structurally valid declaration satisfies:
///
/// 1. `node.kind()` is `CoreNodeKind::Struct`.
/// 2. `name` is non-empty.
/// 3. Every generic-parameter reference is represented exactly once in source
///    order.
/// 4. Every field reference is represented exactly once in source order.
/// 5. No declaration-local list has a fixed machine-dependent capacity.
/// 6. No semantic/backend/hardware information is stored.
/// 7. Node identity is supplied by the AST construction layer.
/// 8. Field/type/node-reference existence is validated by the enclosing AST
///    graph/store.
/// 9. Duplicate field-name detection is a semantic/structural graph concern
///    and must not be inferred from `NodeId`.
///
/// # Why `NodeId` references are used
///
/// The repository's `Item` abstraction already establishes a canonical-node
/// graph model in which a lightweight relationship stores a `NodeId` pointing
/// to the authoritative concrete node. This declaration follows that same
/// principle.
///
/// A future AST store can therefore use:
///
/// - indexed vectors;
/// - arenas;
/// - persistent structures;
/// - incremental stores;
/// - compact graph storage;
/// - other safe storage strategies;
///
/// without changing this declaration's public contract.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StructDeclaration {
    /// Common source-level AST identity, classification, span and metadata.
    node: Node,

    /// Source-level struct name.
    ///
    /// The name is retained in source form. Name resolution is deliberately
    /// performed later by semantic analysis.
    name: String,

    /// Generic-parameter nodes in source order.
    ///
    /// These IDs refer to canonical AST nodes owned by the AST graph.
    generic_parameters: Vec<NodeId>,

    /// Field declaration nodes in source order.
    ///
    /// These IDs refer to canonical AST nodes owned by the AST graph.
    fields: Vec<NodeId>,
}

impl StructDeclaration {
    /// Creates a struct declaration.
    ///
    /// This constructor establishes the local structural contract but does not
    /// perform cross-node semantic validation.
    ///
    /// # Errors
    ///
    /// Returns [`StructDeclarationError::EmptyName`] when `name` is empty.
    ///
    /// # Parser integration
    ///
    /// The parser should allocate `id` before constructing this object and
    /// should already have allocated canonical IDs for generic parameters and
    /// fields.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        fields: Vec<NodeId>,
    ) -> Result<Self, StructDeclarationError> {
        let name = name.into();

        validate_name(&name)?;

        Ok(Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::Struct),
                span,
                metadata,
            ),
            name,
            generic_parameters,
            fields,
        })
    }

    /// Creates a struct declaration using default metadata.
    ///
    /// The caller still supplies the node identity and source span. Those
    /// values are intentionally never fabricated by this type.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        fields: Vec<NodeId>,
    ) -> Result<Self, StructDeclarationError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            name,
            generic_parameters,
            fields,
        )
    }

    /// Returns the common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// This is intended for controlled AST transformation infrastructure.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the declaration's stable AST node identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the declaration's source-level node kind.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the declaration's source span.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns source-level metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable source-level metadata.
    ///
    /// Semantic state must not be stored in metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the source-level struct name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Replaces the source-level struct name.
    ///
    /// This operation is intended for parser recovery or source-level AST
    /// transformation. Semantic symbol resolution must occur later.
    ///
    /// # Errors
    ///
    /// Returns [`StructDeclarationError::EmptyName`] if the supplied name is
    /// empty.
    pub fn replace_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<String, StructDeclarationError> {
        let name = name.into();

        validate_name(&name)?;

        Ok(core::mem::replace(&mut self.name, name))
    }

    /// Returns generic-parameter references in source order.
    #[inline]
    #[must_use]
    pub fn generic_parameters(&self) -> &[NodeId] {
        &self.generic_parameters
    }

    /// Returns mutable generic-parameter references.
    ///
    /// Structural validation must be rerun after modifying this collection.
    #[inline]
    pub fn generic_parameters_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.generic_parameters
    }

    /// Replaces the generic-parameter reference list.
    ///
    /// Returns the previous list.
    pub fn replace_generic_parameters(
        &mut self,
        generic_parameters: Vec<NodeId>,
    ) -> Vec<NodeId> {
        core::mem::replace(
            &mut self.generic_parameters,
            generic_parameters,
        )
    }

    /// Returns the number of generic parameters.
    ///
    /// This is a convenience query only. It does not imply a language-level
    /// maximum.
    #[inline]
    #[must_use]
    pub fn generic_parameter_count(&self) -> usize {
        self.generic_parameters.len()
    }

    /// Returns whether the declaration has generic parameters.
    #[inline]
    #[must_use]
    pub fn is_generic(&self) -> bool {
        !self.generic_parameters.is_empty()
    }

    /// Returns field references in source order.
    #[inline]
    #[must_use]
    pub fn fields(&self) -> &[NodeId] {
        &self.fields
    }

    /// Returns mutable field references.
    ///
    /// Structural validation must be rerun after modifying this collection.
    #[inline]
    pub fn fields_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.fields
    }

    /// Replaces the field-reference list.
    ///
    /// Returns the previous list.
    pub fn replace_fields(&mut self, fields: Vec<NodeId>) -> Vec<NodeId> {
        core::mem::replace(&mut self.fields, fields)
    }

    /// Returns the number of fields.
    ///
    /// There is intentionally no fixed maximum.
    #[inline]
    #[must_use]
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Returns whether the struct contains no fields.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns the declaration schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        STRUCT_DECLARATION_SCHEMA_VERSION
    }

    /// Returns the stable source-level kind identifier.
    #[inline]
    #[must_use]
    pub const fn stable_kind_name() -> &'static str {
        STRUCT_DECLARATION_KIND_NAME
    }

    /// Returns a local structural validation result.
    ///
    /// This method deliberately does **not** validate whether referenced
    /// `NodeId`s exist in an AST store because this type has no ownership or
    /// knowledge of the store.
    ///
    /// Cross-node validation belongs to the enclosing AST validation layer.
    pub fn validate_local(&self) -> Result<(), StructDeclarationError> {
        validate_name(&self.name)?;

        if self.node.kind().as_core() != Some(CoreNodeKind::Struct) {
            return Err(StructDeclarationError::InvalidNodeKind {
                expected: CoreNodeKind::Struct,
                actual: self.node.kind().clone(),
            });
        }

        if self
            .generic_parameters
            .iter()
            .any(|id| *id == self.id())
        {
            return Err(StructDeclarationError::SelfReference {
                declaration: self.id(),
                relation: StructReferenceRelation::GenericParameter,
            });
        }

        if self.fields.iter().any(|id| *id == self.id()) {
            return Err(StructDeclarationError::SelfReference {
                declaration: self.id(),
                relation: StructReferenceRelation::Field,
            });
        }

        Ok(())
    }

    /// Returns whether the declaration is locally structurally valid.
    ///
    /// This is a convenience wrapper around [`Self::validate_local`].
    #[inline]
    #[must_use]
    pub fn is_locally_valid(&self) -> bool {
        self.validate_local().is_ok()
    }

    /// Returns the IDs referenced directly by this declaration.
    ///
    /// The returned collection is newly allocated and is therefore suitable
    /// when the caller needs one combined owned list.
    ///
    /// For zero-allocation traversal, use [`Self::generic_parameters`] and
    /// [`Self::fields`] separately.
    #[must_use]
    pub fn referenced_nodes(&self) -> Vec<NodeId> {
        let mut result = Vec::with_capacity(
            self.generic_parameters.len()
                .saturating_add(self.fields.len()),
        );

        result.extend(self.generic_parameters.iter().copied());
        result.extend(self.fields.iter().copied());

        result
    }

    /// Visits each directly referenced node ID in deterministic source order.
    ///
    /// Generic parameters are visited first, followed by fields.
    ///
    /// This operation does not recursively traverse the AST and therefore
    /// cannot overflow the call stack because of arbitrary AST nesting.
    pub fn for_each_reference<F>(&self, mut visitor: F)
    where
        F: FnMut(NodeId),
    {
        for id in &self.generic_parameters {
            visitor(*id);
        }

        for id in &self.fields {
            visitor(*id);
        }
    }
}

impl fmt::Display for StructDeclaration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "struct {}", self.name)?;

        if !self.generic_parameters.is_empty() {
            write!(
                formatter,
                "<{} generic parameters>",
                self.generic_parameters.len()
            )?;
        }

        write!(
            formatter,
            " ({} field{})",
            self.fields.len(),
            if self.fields.len() == 1 { "" } else { "s" }
        )
    }
}

/// Structural errors specific to a struct declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StructDeclarationError {
    /// A struct name is empty.
    EmptyName,

    /// The declaration's common node has an unexpected kind.
    InvalidNodeKind {
        /// Required source-level kind.
        expected: CoreNodeKind,

        /// Actual node kind.
        actual: NodeKind,
    },

    /// A declaration directly references itself.
    SelfReference {
        /// Identity of the struct declaration.
        declaration: NodeId,

        /// Relationship containing the invalid self-reference.
        relation: StructReferenceRelation,
    },
}

impl fmt::Display for StructDeclarationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                write!(formatter, "struct declaration name must not be empty")
            }

            Self::InvalidNodeKind { expected, actual } => {
                write!(
                    formatter,
                    "invalid struct declaration node kind: expected \
                     {}, found {}",
                    expected.name(),
                    actual
                )
            }

            Self::SelfReference {
                declaration,
                relation,
            } => {
                write!(
                    formatter,
                    "struct declaration {} directly references itself \
                     as a {}",
                    declaration,
                    relation
                )
            }
        }
    }
}

impl std::error::Error for StructDeclarationError {}

/// The relationship through which a struct declaration referenced itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructReferenceRelation {
    /// A generic parameter reference.
    GenericParameter,

    /// A field reference.
    Field,
}

impl fmt::Display for StructReferenceRelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenericParameter => formatter.write_str("generic parameter"),
            Self::Field => formatter.write_str("field"),
        }
    }
}

/// Validates a source-level struct name.
///
/// This intentionally performs only the minimum invariant that this file can
/// establish without depending on the lexer/parser identifier implementation.
///
/// Full identifier legality belongs to the parser/lexer/semantic layer.
///
/// Empty names are rejected because an unnamed declaration cannot be
/// represented as a valid source-level struct declaration.
///
/// Unicode names are permitted here. The parser/lexer remains responsible for
/// deciding which Unicode identifier forms are legal in Zamani source.
fn validate_name(name: &str) -> Result<(), StructDeclarationError> {
    if name.is_empty() {
        return Err(StructDeclarationError::EmptyName);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(StructDeclaration::schema_version(), 1);
    }

    #[test]
    fn stable_kind_name_is_stable() {
        assert_eq!(
            StructDeclaration::stable_kind_name(),
            "zamani:struct"
        );
    }

    #[test]
    fn empty_name_is_rejected() {
        let result = StructDeclaration::without_metadata(
            NodeId::default(),
            Span::default(),
            "",
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(StructDeclarationError::EmptyName)
        );
    }

    #[test]
    fn declaration_uses_struct_node_kind() {
        let declaration = StructDeclaration::without_metadata(
            NodeId::default(),
            Span::default(),
            "Example",
            Vec::new(),
            Vec::new(),
        )
        .expect("valid struct declaration");

        assert_eq!(
            declaration.kind().as_core(),
            Some(CoreNodeKind::Struct)
        );
    }

    #[test]
    fn fields_preserve_source_order() {
        let field_a = NodeId::from_raw(10);
        let field_b = NodeId::from_raw(20);
        let field_c = NodeId::from_raw(30);

        let declaration = StructDeclaration::without_metadata(
            NodeId::from_raw(1),
            Span::default(),
            "Example",
            Vec::new(),
            vec![field_a, field_b, field_c],
        )
        .expect("valid struct declaration");

        assert_eq!(
            declaration.fields(),
            &[field_a, field_b, field_c]
        );
    }

    #[test]
    fn generic_parameters_preserve_source_order() {
        let generic_a = NodeId::from_raw(10);
        let generic_b = NodeId::from_raw(20);

        let declaration = StructDeclaration::without_metadata(
            NodeId::from_raw(1),
            Span::default(),
            "Example",
            vec![generic_a, generic_b],
            Vec::new(),
        )
        .expect("valid struct declaration");

        assert_eq!(
            declaration.generic_parameters(),
            &[generic_a, generic_b]
        );

        assert!(declaration.is_generic());
    }

    #[test]
    fn direct_self_field_reference_is_rejected() {
        let declaration_id = NodeId::from_raw(1);

        let declaration = StructDeclaration::without_metadata(
            declaration_id,
            Span::default(),
            "Example",
            Vec::new(),
            vec![declaration_id],
        )
        .expect("construction itself succeeds");

        assert_eq!(
            declaration.validate_local(),
            Err(StructDeclarationError::SelfReference {
                declaration: declaration_id,
                relation: StructReferenceRelation::Field,
            })
        );
    }

    #[test]
    fn direct_self_generic_reference_is_rejected() {
        let declaration_id = NodeId::from_raw(1);

        let declaration = StructDeclaration::without_metadata(
            declaration_id,
            Span::default(),
            "Example",
            vec![declaration_id],
            Vec::new(),
        )
        .expect("construction itself succeeds");

        assert_eq!(
            declaration.validate_local(),
            Err(StructDeclarationError::SelfReference {
                declaration: declaration_id,
                relation: StructReferenceRelation::GenericParameter,
            })
        );
    }

    #[test]
    fn empty_struct_is_supported() {
        let declaration = StructDeclaration::without_metadata(
            NodeId::from_raw(1),
            Span::default(),
            "Empty",
            Vec::new(),
            Vec::new(),
        )
        .expect("valid empty struct");

        assert!(declaration.is_empty());
        assert_eq!(declaration.field_count(), 0);
        assert!(declaration.is_locally_valid());
    }

    #[test]
    fn reference_iteration_is_deterministic() {
        let generic_a = NodeId::from_raw(10);
        let generic_b = NodeId::from_raw(20);
        let field_a = NodeId::from_raw(30);
        let field_b = NodeId::from_raw(40);

        let declaration = StructDeclaration::without_metadata(
            NodeId::from_raw(1),
            Span::default(),
            "Example",
            vec![generic_a, generic_b],
            vec![field_a, field_b],
        )
        .expect("valid declaration");

        let mut visited = Vec::new();

        declaration.for_each_reference(|id| visited.push(id));

        assert_eq!(
            visited,
            vec![generic_a, generic_b, field_a, field_b]
        );
    }

    #[test]
    fn referenced_nodes_preserve_deterministic_order() {
        let generic = NodeId::from_raw(10);
        let field = NodeId::from_raw(20);

        let declaration = StructDeclaration::without_metadata(
            NodeId::from_raw(1),
            Span::default(),
            "Example",
            vec![generic],
            vec![field],
        )
        .expect("valid declaration");

        assert_eq!(
            declaration.referenced_nodes(),
            vec![generic, field]
        );
    }

    #[test]
    fn display_is_bounded_by_declaration_metadata() {
        let declaration = StructDeclaration::without_metadata(
            NodeId::from_raw(1),
            Span::default(),
            "Example",
            vec![
                NodeId::from_raw(2),
                NodeId::from_raw(3),
            ],
            vec![
                NodeId::from_raw(4),
                NodeId::from_raw(5),
                NodeId::from_raw(6),
            ],
        )
        .expect("valid declaration");

        assert_eq!(
            declaration.to_string(),
            "struct Example <2 generic parameters> (3 fields)"
        );
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let declaration = StructDeclaration::without_metadata(
            NodeId::from_raw(1),
            Span::default(),
            "Example",
            vec![NodeId::from_raw(2)],
            vec![
                NodeId::from_raw(3),
                NodeId::from_raw(4),
            ],
        )
        .expect("valid declaration");

        let encoded =
            serde_json::to_string(&declaration)
                .expect("serialization succeeds");

        let decoded: StructDeclaration =
            serde_json::from_str(&encoded)
                .expect("deserialization succeeds");

        assert_eq!(decoded, declaration);
    }
}