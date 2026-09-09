//! # Zamani Frontend AST — Class Declaration
//!
//! Production-ready source-level representation of a Zamani `class`
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
//!     ├── ClassDeclaration
//!     │       ├── Node
//!     │       ├── name
//!     │       ├── generic parameters
//!     │       ├── extends references
//!     │       ├── implements references
//!     │       ├── permits references
//!     │       └── member references
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
//! domain / target / runtime lowering
//! ```
//!
//! ## Purpose
//!
//! `ClassDeclaration` represents the **source-language meaning and structure**
//! of a class declaration without embedding implementation details from any
//! particular execution target.
//!
//! A class may therefore participate in:
//!
//! - ordinary classical programs;
//! - generic programs;
//! - resource-oriented programs;
//! - hybrid programs;
//! - quantum-related source abstractions;
//! - accelerator-facing source abstractions;
//! - HDL-facing source abstractions;
//! - future computational domains.
//!
//! The class AST does not know how the class will eventually be implemented.
//!
//! ## Domain neutrality
//!
//! This type must never contain:
//!
//! - CPU identifiers;
//! - GPU identifiers;
//! - FPGA identifiers;
//! - QPU identifiers;
//! - qubit counts;
//! - physical qubit identifiers;
//! - hardware topology;
//! - backend/provider identifiers;
//! - instruction sets;
//! - gate sets;
//! - scheduling information;
//! - routing information;
//! - calibration data;
//! - error-correction implementation;
//! - resilience implementation;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - runtime handles;
//! - execution jobs.
//!
//! Those concerns belong to later compilation layers.
//!
//! ## POCO-REAF
//!
//! The declaration contains no machine-size assumptions. A source class can
//! therefore be compiled for different target sizes and technologies without
//! changing its AST representation.
//!
//! ```text
//! Program once
//!      │
//!      ▼
//! ClassDeclaration
//!      │
//!      ▼
//! semantic resolution
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ├── small target
//!      ├── large target
//!      ├── distributed target
//!      ├── heterogeneous target
//!      ├── quantum target
//!      └── future target
//! ```
//!
//! ## Grammar integration
//!
//! The repository grammar currently describes a class using the conceptual
//! structure:
//!
//! ```text
//! classDecl:
//!     modifiers?
//!     'class'
//!     IDENTIFIER
//!     genericParameters?
//!     extendsClause?
//!     implementsClause?
//!     permitsClause?
//!     '{'
//!     classBody
//!     '}'
//! ;
//!
//! extendsClause:
//!     'extends' IDENTIFIER (',' IDENTIFIER)*
//! ;
//!
//! implementsClause:
//!     'implements' IDENTIFIER (',' IDENTIFIER)*
//! ;
//!
//! permitsClause:
//!     'permits' IDENTIFIER (',' IDENTIFIER)*
//! ;
//!
//! classBody:
//!     classMember*
//! ;
//! ```
//!
//! The AST therefore preserves the semantically relevant declaration
//! relationships as ordered `NodeId` references.
//!
//! Modifiers are intentionally not represented by a private class-specific
//! modifier enum. They must use the repository's canonical modifier/attribute
//! representation when that source construct is exposed by the native AST.
//!
//! ## Canonical-node graph
//!
//! `ClassDeclaration` does not duplicate child AST nodes.
//!
//! Generic parameters, inherited types, implemented interfaces, permitted
//! types, and class members are references to authoritative AST nodes owned by
//! the enclosing AST graph/store.
//!
//! This preserves the invariant:
//!
//! > Every concrete AST node has one authoritative representation.
//!
//! The AST storage strategy can therefore evolve from vectors to arenas,
//! indexed stores, persistent structures, incremental stores, or other safe
//! representations without changing the declaration's semantic contract.
//!
//! ## Dependency contract
//!
//! This module may depend only on:
//!
//! - `Node`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `NodeMetadata`;
//! - `Span`;
//! - `AstNode`;
//! - Rust standard-library facilities;
//! - `serde`.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - optimization;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - external quantum-language ASTs.
//!
//! ## Structural versus semantic validation
//!
//! This file performs only declaration-local validation.
//!
//! It validates:
//!
//! - non-empty class names;
//! - correct canonical `Class` node kind.
//!
//! It deliberately does not decide:
//!
//! - whether a base type exists;
//! - whether a type is actually a class;
//! - whether an interface is implemented legally;
//! - whether inheritance is cyclic;
//! - whether permits relationships are valid;
//! - whether a member is legal in a class;
//! - whether generic bounds are satisfied;
//! - whether a class is instantiable;
//! - whether a class can be lowered to a particular target.
//!
//! Those decisions require the complete AST graph and semantic model.
//!
//! ## Ordering
//!
//! All relationship collections preserve source order.
//!
//! This is important for:
//!
//! - deterministic diagnostics;
//! - deterministic serialization;
//! - source reconstruction;
//! - tooling;
//! - reproducible compilation;
//! - stable visitor traversal.
//!
//! ## Scalability
//!
//! There is deliberately no fixed maximum for:
//!
//! - class declarations;
//! - generic parameters;
//! - base types;
//! - implemented interfaces;
//! - permitted types;
//! - class members;
//! - nested declarations;
//! - source size;
//! - machine size;
//! - resource count.
//!
//! Collections grow according to available resources and externally configured
//! compiler policies.
//!
//! Any hostile-input limits belong to configurable validation infrastructure,
//! not to this AST declaration.
//!
//! ## Determinism
//!
//! This type does not generate IDs, timestamps, random values, pointers, or
//! memory-address-derived identities.
//!
//! `NodeId`s are supplied by the AST construction layer.
//!
//! ## Serialization
//!
//! The declaration derives Serde serialization. Its serialized representation
//! contains only source-level AST information and canonical node references.
//!
//! AST serialization/versioning infrastructure remains responsible for the
//! overall serialized schema version and compatibility policy.
//!
//! ## Thread safety
//!
//! The type contains ordinary Rust-owned values and no global mutable state.
//! Immutable instances can therefore be shared across compiler phases when
//! their containing AST graph is itself shared safely.
//!
//! ## Security
//!
//! This module performs no unchecked indexing and no pointer operations.
//! Malformed or hostile AST graphs must be handled by the structural validation
//! layer and configurable compiler resource policies.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021.
//!
//! No nightly features and no `unsafe` code are used.

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::AstNode;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version for the source-level class-declaration contract.
///
/// This version is independent of:
///
/// - the Zamani language version;
/// - compiler version;
/// - AST serialization version;
/// - extension versions.
pub const CLASS_DECLARATION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identifier for a class declaration.
pub const CLASS_DECLARATION_KIND_NAME: &str = "zamani:class";

/// A source-level Zamani class declaration.
///
/// The declaration owns the source structure of the class itself while
/// referenced child constructs remain canonical AST nodes identified by
/// [`NodeId`].
///
/// # Representation
///
/// ```text
/// ClassDeclaration
/// ├── Node
/// ├── name
/// ├── generic_parameters
/// ├── extends
/// ├── implements
/// ├── permits
/// └── members
/// ```
///
/// Every collection preserves source order.
///
/// # Semantic boundary
///
/// This structure does not resolve names or types. For example, an entry in
/// `extends` is only a reference to an AST node. Semantic analysis determines
/// what that node means and whether it is a valid base type.
///
/// The same principle applies to implemented interfaces, permitted types,
/// generic parameters, and members.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClassDeclaration {
    /// Common source-level AST identity, classification, span and metadata.
    node: Node,

    /// Source spelling of the class name.
    ///
    /// Name resolution belongs to semantic analysis.
    name: String,

    /// Generic parameter nodes in source order.
    ///
    /// Each ID identifies an authoritative generic-parameter node in the AST
    /// graph.
    generic_parameters: Vec<NodeId>,

    /// Base-type references introduced by `extends`, in source order.
    ///
    /// The grammar currently permits multiple identifiers. The AST therefore
    /// preserves all of them instead of imposing a single-inheritance policy.
    ///
    /// Whether the language semantics permit a particular inheritance shape is
    /// determined later by semantic analysis.
    extends: Vec<NodeId>,

    /// Interface/type references introduced by `implements`, in source order.
    implements: Vec<NodeId>,

    /// Type references introduced by `permits`, in source order.
    ///
    /// The AST preserves these as source-level relationships without deciding
    /// whether the referenced declarations are semantically valid.
    permits: Vec<NodeId>,

    /// Class-member nodes in source order.
    ///
    /// Members remain canonical AST nodes rather than being duplicated inside
    /// this declaration.
    members: Vec<NodeId>,
}

impl ClassDeclaration {
    /// Creates a new source-level class declaration.
    ///
    /// The constructor establishes the declaration-local structural contract.
    /// It does not perform semantic resolution or AST-store validation.
    ///
    /// # Errors
    ///
    /// Returns [`ClassDeclarationError::EmptyName`] if `name` is empty.
    ///
    /// # Parser integration
    ///
    /// The parser should:
    ///
    /// 1. allocate the class `NodeId`;
    /// 2. parse the class name;
    /// 3. parse generic parameters;
    /// 4. parse `extends`;
    /// 5. parse `implements`;
    /// 6. parse `permits`;
    /// 7. parse class members;
    /// 8. allocate canonical IDs for referenced nodes;
    /// 9. construct this declaration.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        extends: Vec<NodeId>,
        implements: Vec<NodeId>,
        permits: Vec<NodeId>,
        members: Vec<NodeId>,
    ) -> Result<Self, ClassDeclarationError> {
        let name = name.into();

        validate_name(&name)?;

        Ok(Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::Class),
                span,
                metadata,
            ),
            name,
            generic_parameters,
            extends,
            implements,
            permits,
            members,
        })
    }

    /// Creates a class declaration with default metadata.
    ///
    /// The caller must still provide the canonical node identity and source
    /// span. Those values are never silently manufactured by this type.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        extends: Vec<NodeId>,
        implements: Vec<NodeId>,
        permits: Vec<NodeId>,
        members: Vec<NodeId>,
    ) -> Result<Self, ClassDeclarationError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            name,
            generic_parameters,
            extends,
            implements,
            permits,
            members,
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

    /// Returns the stable AST node identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the canonical node kind.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the source span.
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
    /// Semantic information must not be placed into metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the source spelling of the class name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Replaces the source-level class name.
    ///
    /// This is intended for parser recovery and controlled AST
    /// transformations. Semantic name resolution remains downstream.
    ///
    /// # Errors
    ///
    /// Returns [`ClassDeclarationError::EmptyName`] for an empty name.
    pub fn replace_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<String, ClassDeclarationError> {
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

    /// Returns mutable access to generic-parameter references.
    ///
    /// Structural validation should be rerun after mutation.
    #[inline]
    pub fn generic_parameters_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.generic_parameters
    }

    /// Replaces generic-parameter references.
    ///
    /// Returns the previous collection.
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
    #[inline]
    #[must_use]
    pub fn generic_parameter_count(&self) -> usize {
        self.generic_parameters.len()
    }

    /// Returns whether the class declares generic parameters.
    #[inline]
    #[must_use]
    pub fn is_generic(&self) -> bool {
        !self.generic_parameters.is_empty()
    }

    /// Returns the source-level `extends` references.
    #[inline]
    #[must_use]
    pub fn extends(&self) -> &[NodeId] {
        &self.extends
    }

    /// Returns mutable access to `extends` references.
    ///
    /// Structural validation should be rerun after mutation.
    #[inline]
    pub fn extends_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.extends
    }

    /// Replaces `extends` references.
    ///
    /// Returns the previous collection.
    pub fn replace_extends(&mut self, extends: Vec<NodeId>) -> Vec<NodeId> {
        core::mem::replace(&mut self.extends, extends)
    }

    /// Returns the number of `extends` references.
    #[inline]
    #[must_use]
    pub fn extends_count(&self) -> usize {
        self.extends.len()
    }

    /// Returns whether the class has an `extends` clause.
    #[inline]
    #[must_use]
    pub fn has_extends(&self) -> bool {
        !self.extends.is_empty()
    }

    /// Returns the source-level `implements` references.
    #[inline]
    #[must_use]
    pub fn implements(&self) -> &[NodeId] {
        &self.implements
    }

    /// Returns mutable access to `implements` references.
    ///
    /// Structural validation should be rerun after mutation.
    #[inline]
    pub fn implements_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.implements
    }

    /// Replaces `implements` references.
    ///
    /// Returns the previous collection.
    pub fn replace_implements(
        &mut self,
        implements: Vec<NodeId>,
    ) -> Vec<NodeId> {
        core::mem::replace(&mut self.implements, implements)
    }

    /// Returns the number of implemented-type references.
    #[inline]
    #[must_use]
    pub fn implements_count(&self) -> usize {
        self.implements.len()
    }

    /// Returns whether the class has an `implements` clause.
    #[inline]
    #[must_use]
    pub fn has_implements(&self) -> bool {
        !self.implements.is_empty()
    }

    /// Returns the source-level `permits` references.
    #[inline]
    #[must_use]
    pub fn permits(&self) -> &[NodeId] {
        &self.permits
    }

    /// Returns mutable access to `permits` references.
    ///
    /// Structural validation should be rerun after mutation.
    #[inline]
    pub fn permits_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.permits
    }

    /// Replaces `permits` references.
    ///
    /// Returns the previous collection.
    pub fn replace_permits(&mut self, permits: Vec<NodeId>) -> Vec<NodeId> {
        core::mem::replace(&mut self.permits, permits)
    }

    /// Returns the number of permitted-type references.
    #[inline]
    #[must_use]
    pub fn permits_count(&self) -> usize {
        self.permits.len()
    }

    /// Returns whether the class has a `permits` clause.
    #[inline]
    #[must_use]
    pub fn has_permits(&self) -> bool {
        !self.permits.is_empty()
    }

    /// Returns class members in source order.
    #[inline]
    #[must_use]
    pub fn members(&self) -> &[NodeId] {
        &self.members
    }

    /// Returns mutable access to class-member references.
    ///
    /// Structural validation should be rerun after mutation.
    #[inline]
    pub fn members_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.members
    }

    /// Replaces class-member references.
    ///
    /// Returns the previous collection.
    pub fn replace_members(&mut self, members: Vec<NodeId>) -> Vec<NodeId> {
        core::mem::replace(&mut self.members, members)
    }

    /// Returns the number of class members.
    #[inline]
    #[must_use]
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Returns whether the class contains no members.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Returns whether this class has any inheritance/interface relationship.
    ///
    /// This is purely a structural query. It does not imply that any
    /// relationship is semantically valid.
    #[inline]
    #[must_use]
    pub fn has_relationships(&self) -> bool {
        !self.extends.is_empty()
            || !self.implements.is_empty()
            || !self.permits.is_empty()
    }

    /// Returns the declaration schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        CLASS_DECLARATION_SCHEMA_VERSION
    }

    /// Returns the stable source-level kind identifier.
    #[inline]
    #[must_use]
    pub const fn stable_kind_name() -> &'static str {
        CLASS_DECLARATION_KIND_NAME
    }

    /// Validates invariants that can be checked without access to the complete
    /// AST graph.
    ///
    /// Cross-node checks intentionally remain outside this method.
    pub fn validate_local(&self) -> Result<(), ClassDeclarationError> {
        validate_name(&self.name)?;

        if self.node.kind().as_core() != Some(CoreNodeKind::Class) {
            return Err(ClassDeclarationError::InvalidNodeKind {
                expected: CoreNodeKind::Class,
                actual: self.node.kind().clone(),
            });
        }

        Ok(())
    }

    /// Returns whether the declaration satisfies local structural invariants.
    ///
    /// This does not mean that referenced nodes exist or that the declaration
    /// is semantically valid.
    #[inline]
    #[must_use]
    pub fn is_locally_valid(&self) -> bool {
        self.validate_local().is_ok()
    }

    /// Visits every referenced child node in deterministic source-oriented
    /// relationship order.
    ///
    /// The order is:
    ///
    /// 1. generic parameters;
    /// 2. `extends`;
    /// 3. `implements`;
    /// 4. `permits`;
    /// 5. members.
    ///
    /// This method performs no AST-store lookup and therefore cannot fail.
    pub fn for_each_reference<F>(&self, mut visitor: F)
    where
        F: FnMut(NodeId),
    {
        for id in &self.generic_parameters {
            visitor(*id);
        }

        for id in &self.extends {
            visitor(*id);
        }

        for id in &self.implements {
            visitor(*id);
        }

        for id in &self.permits {
            visitor(*id);
        }

        for id in &self.members {
            visitor(*id);
        }
    }

    /// Returns all referenced node IDs in deterministic source-oriented
    /// relationship order.
    ///
    /// The returned vector is a snapshot. It does not transfer ownership of
    /// the referenced AST nodes.
    pub fn referenced_nodes(&self) -> Vec<NodeId> {
        let mut result = Vec::with_capacity(
            self.generic_parameters
                .len()
                .saturating_add(self.extends.len())
                .saturating_add(self.implements.len())
                .saturating_add(self.permits.len())
                .saturating_add(self.members.len()),
        );

        self.for_each_reference(|id| result.push(id));

        result
    }

    /// Returns the total number of direct AST references held by the
    /// declaration.
    ///
    /// This is a structural count, not a language or machine limit.
    #[inline]
    #[must_use]
    pub fn reference_count(&self) -> usize {
        self.generic_parameters
            .len()
            .saturating_add(self.extends.len())
            .saturating_add(self.implements.len())
            .saturating_add(self.permits.len())
            .saturating_add(self.members.len())
    }
}

impl AstNode for ClassDeclaration {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl fmt::Display for ClassDeclaration {
    /// Formats a concise deterministic source-level summary.
    ///
    /// The method deliberately does not recursively print members or metadata,
    /// because those structures may be arbitrarily large.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "class {}", self.name)?;

        if !self.generic_parameters.is_empty() {
            write!(
                formatter,
                "<{} generic parameter(s)>",
                self.generic_parameters.len()
            )?;
        }

        if !self.extends.is_empty() {
            write!(
                formatter,
                " extends {}",
                self.extends.len()
            )?;
        }

        if !self.implements.is_empty() {
            write!(
                formatter,
                " implements {}",
                self.implements.len()
            )?;
        }

        if !self.permits.is_empty() {
            write!(
                formatter,
                " permits {}",
                self.permits.len()
            )?;
        }

        write!(formatter, " {{ {} member(s) }}", self.members.len())
    }
}

/// Errors produced by declaration-local class validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClassDeclarationError {
    /// A class declaration has an empty source-level name.
    EmptyName,

    /// The embedded canonical node does not have `CoreNodeKind::Class`.
    InvalidNodeKind {
        /// The node kind required by this declaration.
        expected: CoreNodeKind,

        /// The actual node kind.
        actual: NodeKind,
    },
}

impl fmt::Display for ClassDeclarationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("class declaration name must not be empty")
            }

            Self::InvalidNodeKind { expected, actual } => {
                write!(
                    formatter,
                    "class declaration has invalid node kind: expected zamani:{}, found {}",
                    expected.name(),
                    actual
                )
            }
        }
    }
}

impl std::error::Error for ClassDeclarationError {}

/// Validates the minimal source-level name invariant.
///
/// Lexical identifier legality belongs to the lexer/parser and language
/// grammar. This function intentionally performs only the invariant required
/// for this declaration to exist as a named class node.
fn validate_name(name: &str) -> Result<(), ClassDeclarationError> {
    if name.is_empty() {
        return Err(ClassDeclarationError::EmptyName);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::node_id::NodeId;
    use super::super::super::source::SourceId;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test node ID must be non-zero")
    }

    fn span() -> Span {
        Span::point(SourceId::from_raw(1), 0.into())
    }

    #[test]
    fn creates_valid_class_declaration() {
        let declaration = ClassDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2)],
            vec![node_id(3)],
            vec![node_id(4), node_id(5)],
            vec![node_id(6)],
            vec![node_id(7), node_id(8)],
        )
        .expect("class declaration should be valid");

        assert_eq!(declaration.name(), "Example");
        assert_eq!(declaration.kind().as_core(), Some(CoreNodeKind::Class));
        assert!(declaration.is_locally_valid());
        assert!(declaration.is_generic());
        assert!(declaration.has_extends());
        assert!(declaration.has_implements());
        assert!(declaration.has_permits());
        assert_eq!(declaration.member_count(), 2);
    }

    #[test]
    fn preserves_reference_order() {
        let declaration = ClassDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2), node_id(3)],
            vec![node_id(4), node_id(5)],
            vec![node_id(6)],
            vec![node_id(7)],
            vec![node_id(8), node_id(9)],
        )
        .expect("class declaration should be valid");

        assert_eq!(
            declaration.referenced_nodes(),
            vec![
                node_id(2),
                node_id(3),
                node_id(4),
                node_id(5),
                node_id(6),
                node_id(7),
                node_id(8),
                node_id(9),
            ]
        );
    }

    #[test]
    fn for_each_reference_is_deterministic() {
        let declaration = ClassDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2)],
            vec![node_id(3)],
            vec![node_id(4)],
            vec![node_id(5)],
            vec![node_id(6)],
        )
        .expect("class declaration should be valid");

        let mut visited = Vec::new();

        declaration.for_each_reference(|id| visited.push(id));

        assert_eq!(
            visited,
            vec![
                node_id(2),
                node_id(3),
                node_id(4),
                node_id(5),
                node_id(6),
            ]
        );
    }

    #[test]
    fn reference_count_matches_all_relationships() {
        let declaration = ClassDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2), node_id(3)],
            vec![node_id(4)],
            vec![node_id(5), node_id(6)],
            vec![node_id(7)],
            vec![node_id(8), node_id(9), node_id(10)],
        )
        .expect("class declaration should be valid");

        assert_eq!(declaration.reference_count(), 9);
    }

    #[test]
    fn empty_name_is_rejected() {
        let result = ClassDeclaration::without_metadata(
            node_id(1),
            span(),
            "",
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(ClassDeclarationError::EmptyName)
        );
    }

    #[test]
    fn replacing_name_preserves_other_structure() {
        let mut declaration = ClassDeclaration::without_metadata(
            node_id(1),
            span(),
            "Before",
            vec![node_id(2)],
            vec![node_id(3)],
            vec![node_id(4)],
            vec![node_id(5)],
            vec![node_id(6)],
        )
        .expect("class declaration should be valid");

        let old = declaration
            .replace_name("After")
            .expect("replacement name should be valid");

        assert_eq!(old, "Before");
        assert_eq!(declaration.name(), "After");
        assert_eq!(declaration.generic_parameter_count(), 1);
        assert_eq!(declaration.extends_count(), 1);
        assert_eq!(declaration.implements_count(), 1);
        assert_eq!(declaration.permits_count(), 1);
        assert_eq!(declaration.member_count(), 1);
    }

    #[test]
    fn empty_relationships_are_valid() {
        let declaration = ClassDeclaration::without_metadata(
            node_id(1),
            span(),
            "EmptyClass",
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect("empty class should be valid");

        assert!(declaration.is_empty());
        assert!(!declaration.has_extends());
        assert!(!declaration.has_implements());
        assert!(!declaration.has_permits());
        assert!(!declaration.has_relationships());
        assert_eq!(declaration.reference_count(), 0);
    }

    #[test]
    fn schema_contract_is_stable() {
        assert_eq!(ClassDeclaration::schema_version(), 1);
        assert_eq!(
            ClassDeclaration::stable_kind_name(),
            "zamani:class"
        );
    }

    #[test]
    fn display_is_bounded_to_declaration_level() {
        let declaration = ClassDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2)],
            vec![node_id(3)],
            vec![node_id(4)],
            vec![node_id(5)],
            vec![node_id(6), node_id(7)],
        )
        .expect("class declaration should be valid");

        assert_eq!(
            declaration.to_string(),
            "class Example \
             <1 generic parameter(s)> \
             extends 1 \
             implements 1 \
             permits 1 \
             { 2 member(s) }"
                .replace("             ", "")
        );
    }

    #[test]
    fn ast_node_contract_is_delegated_to_node() {
        let declaration = ClassDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect("class declaration should be valid");

        assert_eq!(
            AstNode::id(&declaration),
            node_id(1)
        );

        assert_eq!(
            AstNode::kind(&declaration).as_core(),
            Some(CoreNodeKind::Class)
        );
    }
}