//! # Zamani Frontend AST — Interface Declaration
//!
//! Production-ready source-level representation of a Zamani `interface`
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
//! Native Zamani AST
//!     │
//!     ├── InterfaceDeclaration
//!     │       ├── Node
//!     │       ├── name
//!     │       ├── generic parameters
//!     │       ├── extends references
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
//! `InterfaceDeclaration` represents the source-level structure of a Zamani
//! interface without embedding implementation details from any particular
//! machine, backend, computational domain, or execution technology.
//!
//! An interface is a source-language abstraction. It may describe contracts
//! used by classical programs, resource-oriented programs, hybrid programs,
//! accelerator-oriented programs, quantum-related programs, HDL-facing
//! abstractions, or future computational domains.
//!
//! The interface AST does not determine how an implementation will eventually
//! execute.
//!
//! ## Domain neutrality
//!
//! This type deliberately contains no knowledge of:
//!
//! - CPUs;
//! - GPUs;
//! - TPUs;
//! - FPGAs;
//! - QPUs;
//! - qubit counts;
//! - physical qubits;
//! - hardware topology;
//! - vendor APIs;
//! - backend identifiers;
//! - instruction sets;
//! - gate sets;
//! - routing;
//! - scheduling;
//! - calibration;
//! - quantum error correction;
//! - resilience implementation;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - runtime handles;
//! - execution jobs.
//!
//! Those concerns belong to later compiler layers.
//!
//! ## POCO-REAF
//!
//! The declaration contains no machine-size or hardware-size assumptions.
//! The same source-level interface can therefore participate in programs
//! compiled for different scales and computational technologies.
//!
//! ```text
//! Program once
//!      │
//!      ▼
//! InterfaceDeclaration
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
//! The repository grammar defines the interface declaration as:
//
//! ```text
//! interfaceDecl
//!     : modifier* 'interface' IDENTIFIER genericParameters? extendsClause?
//!       '{' interfaceMember* '}'
//!     ;
//! ```
//!
//! The grammar therefore establishes the following source-level structure:
//!
//! ```text
//! modifiers
//! name
//! generic parameters?
//! extends references?
//! interface members*
//! ```
//!
//! This file represents the structural portions that belong directly to the
//! interface declaration:
//!
//! - name;
//! - generic parameter node references;
//! - extended-type node references;
//! - member node references.
//!
//! Modifiers should use the repository's canonical modifier/attribute
//! representation rather than introducing an interface-specific modifier enum
//! here.
//!
//! ## Interface members
//!
//! The grammar currently permits interface members including method forms with
//! different source modifiers such as `default`, `static`, `private`, and
//! `async`.
//!
//! This declaration deliberately does not introduce a closed:
//!
//! ```text
//! enum InterfaceMember
//! ```
//!
//! Instead, members are represented by `NodeId` references to authoritative
//! AST nodes. This keeps the interface declaration open to future member
//! constructs without requiring this file to be redesigned whenever the
//! language grows.
//!
//! ## Canonical AST graph
//!
//! Child constructs are not duplicated inside this declaration.
//!
//! Generic parameters, extended types, and interface members are references to
//! authoritative AST nodes in the enclosing AST graph/store.
//!
//! This provides the invariant:
//!
//! > Every AST node has one authoritative representation.
//!
//! The underlying AST storage can consequently evolve independently toward
//! arenas, indexed stores, persistent structures, incremental storage, or other
//! safe representations without changing the semantic contract of this type.
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
//! - error correction;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - external quantum-language ASTs.
//!
//! ## Structural versus semantic validation
//!
//! This file validates only declaration-local invariants.
//!
//! It validates:
//!
//! - the interface name is non-empty;
//! - the embedded node has `CoreNodeKind::Interface`.
//!
//! It intentionally does not validate:
//!
//! - whether an extended type exists;
//! - whether an extended type is actually an interface;
//! - whether inheritance is cyclic;
//! - whether generic bounds are satisfied;
//! - whether members are legal;
//! - whether member signatures conflict;
//! - whether overloads are valid;
//! - whether visibility rules are satisfied;
//! - whether an interface can be lowered to a particular target.
//!
//! Those checks require the complete AST graph and semantic model.
//!
//! ## Source ordering
//!
//! All collections preserve source order.
//!
//! This guarantees deterministic behavior for:
//!
//! - diagnostics;
//! - serialization;
//! - source reconstruction;
//! - IDE tooling;
//! - visitors;
//! - traversal;
//! - reproducible compilation.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed by this type for:
//!
//! - interface declarations;
//! - generic parameters;
//! - extended interfaces/types;
//! - interface members;
//! - nested declarations;
//! - source size;
//! - computational resources.
//!
//! `Vec<NodeId>` grows according to available memory and externally configured
//! compiler resource policies.
//!
//! Compiler safety limits, if required for hostile input, belong in the
//! configurable validation/resource-policy layer rather than in this AST node.
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
//! The declaration derives Serde serialization and contains only source-level
//! information plus canonical node references.
//!
//! Overall AST serialization versioning and compatibility policies belong to
//! the AST serialization subsystem.
//!
//! ## Thread safety
//!
//! The declaration owns ordinary Rust values and has no global mutable state.
//! Immutable instances can therefore be consumed concurrently when the
//! containing AST graph and surrounding compiler infrastructure are safely
//! shared.
//!
//! ## Security
//!
//! This implementation performs:
//!
//! - no unchecked indexing;
//! - no pointer operations;
//! - no recursion;
//! - no hidden global allocation;
//! - no unsafe operations.
//!
//! Cross-node validation must be performed by the structural AST validator.
//! Configurable compiler limits must protect against pathological untrusted
//! input sizes.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021.
//!
//! No nightly features are required.
//! No `unsafe` code is used.

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version for the source-level interface declaration contract.
///
/// This version is independent from:
///
/// - the Zamani language version;
/// - compiler version;
/// - serialized AST version;
/// - extension versions.
pub const INTERFACE_DECLARATION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identifier for an interface declaration.
pub const INTERFACE_DECLARATION_KIND_NAME: &str = "zamani:interface";

/// A source-level Zamani interface declaration.
///
/// The declaration owns the source-level structure of the interface itself.
/// Child constructs remain authoritative AST nodes referenced through
/// [`NodeId`].
///
/// # Representation
///
/// ```text
/// InterfaceDeclaration
/// ├── Node
/// ├── name
/// ├── generic_parameters
/// ├── extends
/// └── members
/// ```
///
/// Every collection preserves source order.
///
/// # Semantic boundary
///
/// The node references stored by this type have no resolved semantic meaning
/// by themselves.
///
/// For example, an ID in `extends` does not by itself prove that the referenced
/// declaration is an interface. Semantic analysis resolves that relationship.
///
/// Likewise, an ID in `members` identifies an AST node but does not determine
/// whether the member is a valid interface member. That determination belongs
/// to semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    /// Common source-level identity, kind, source span, and metadata.
    node: Node,

    /// Exact source spelling of the interface name.
    ///
    /// Name resolution is deliberately deferred to semantic analysis.
    name: String,

    /// Generic parameter nodes in source order.
    generic_parameters: Vec<NodeId>,

    /// Type references introduced by the interface's `extends` clause.
    ///
    /// The grammar permits zero or more comma-separated identifiers through
    /// the shared `extendsClause` production.
    extends: Vec<NodeId>,

    /// Interface-member nodes in source order.
    ///
    /// Members are referenced rather than duplicated so that the AST has one
    /// authoritative representation of each child node.
    members: Vec<NodeId>,
}

impl InterfaceDeclaration {
    /// Creates a new source-level interface declaration.
    ///
    /// This constructor performs declaration-local validation only.
    ///
    /// It does not:
    ///
    /// - resolve names;
    /// - resolve types;
    /// - validate inheritance;
    /// - validate member legality;
    /// - perform type checking;
    /// - perform capability analysis;
    /// - select a target;
    /// - allocate hardware resources;
    /// - lower to ZUIR.
    ///
    /// # Parser integration
    ///
    /// The parser should:
    ///
    /// 1. allocate the interface `NodeId`;
    /// 2. parse the interface name;
    /// 3. parse generic parameters;
    /// 4. parse the optional `extends` clause;
    /// 5. parse interface members in source order;
    /// 6. allocate canonical node IDs for child nodes;
    /// 7. construct this declaration.
    ///
    /// # Errors
    ///
    /// Returns [`InterfaceDeclarationError::EmptyName`] when `name` is empty.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        extends: Vec<NodeId>,
        members: Vec<NodeId>,
    ) -> Result<Self, InterfaceDeclarationError> {
        let name = name.into();

        validate_name(&name)?;

        Ok(Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::Interface),
                span,
                metadata,
            ),
            name,
            generic_parameters,
            extends,
            members,
        })
    }

    /// Creates a new interface declaration with default node metadata.
    ///
    /// The caller must provide the node identity and source span explicitly.
    /// This prevents the declaration from silently manufacturing source
    /// information or identity.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        extends: Vec<NodeId>,
        members: Vec<NodeId>,
    ) -> Result<Self, InterfaceDeclarationError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            name,
            generic_parameters,
            extends,
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
    /// This is intended for controlled AST transformations.
    ///
    /// Semantic analysis should not use this method to inject resolved
    /// semantic state into the source AST.
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

    /// Returns the source span of the complete interface declaration.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns source-level node metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable access to source-level node metadata.
    ///
    /// Metadata must remain source/tooling metadata and must not become an
    /// implicit semantic or backend side channel.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the source spelling of the interface name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Replaces the source-level interface name.
    ///
    /// This operation is intended for parser recovery or controlled AST
    /// transformations. Semantic name resolution remains downstream.
    ///
    /// # Errors
    ///
    /// Returns [`InterfaceDeclarationError::EmptyName`] if the replacement
    /// name is empty.
    pub fn replace_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<String, InterfaceDeclarationError> {
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
    /// Structural validation should be run after mutation.
    #[inline]
    pub fn generic_parameters_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.generic_parameters
    }

    /// Replaces the generic-parameter references.
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

    /// Returns whether this interface declares generic parameters.
    #[inline]
    #[must_use]
    pub fn is_generic(&self) -> bool {
        !self.generic_parameters.is_empty()
    }

    /// Returns the source-level `extends` references in source order.
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

    /// Replaces the `extends` references.
    ///
    /// Returns the previous collection.
    pub fn replace_extends(&mut self, extends: Vec<NodeId>) -> Vec<NodeId> {
        core::mem::replace(&mut self.extends, extends)
    }

    /// Returns the number of extended-type references.
    #[inline]
    #[must_use]
    pub fn extends_count(&self) -> usize {
        self.extends.len()
    }

    /// Returns whether the interface has an `extends` clause.
    #[inline]
    #[must_use]
    pub fn has_extends(&self) -> bool {
        !self.extends.is_empty()
    }

    /// Returns interface members in source order.
    #[inline]
    #[must_use]
    pub fn members(&self) -> &[NodeId] {
        &self.members
    }

    /// Returns mutable access to interface-member references.
    ///
    /// Structural validation should be rerun after mutation.
    #[inline]
    pub fn members_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.members
    }

    /// Replaces interface-member references.
    ///
    /// Returns the previous collection.
    pub fn replace_members(&mut self, members: Vec<NodeId>) -> Vec<NodeId> {
        core::mem::replace(&mut self.members, members)
    }

    /// Returns the number of interface members.
    #[inline]
    #[must_use]
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Returns whether this interface has no members.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Returns whether this interface has any inheritance relationship.
    ///
    /// This is a structural query only. It does not mean that the referenced
    /// declarations are valid interfaces or that the inheritance relationship
    /// is legal.
    #[inline]
    #[must_use]
    pub fn has_relationships(&self) -> bool {
        !self.extends.is_empty()
    }

    /// Returns the declaration schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        INTERFACE_DECLARATION_SCHEMA_VERSION
    }

    /// Returns the stable source-level declaration identifier.
    #[inline]
    #[must_use]
    pub const fn stable_kind_name() -> &'static str {
        INTERFACE_DECLARATION_KIND_NAME
    }

    /// Validates invariants that can be checked without access to the complete
    /// AST graph.
    ///
    /// Cross-node validation is deliberately excluded.
    pub fn validate_local(&self) -> Result<(), InterfaceDeclarationError> {
        validate_name(&self.name)?;

        if self.node.kind().as_core() != Some(CoreNodeKind::Interface) {
            return Err(InterfaceDeclarationError::InvalidNodeKind {
                expected: CoreNodeKind::Interface,
                actual: self.node.kind().clone(),
            });
        }

        Ok(())
    }

    /// Returns whether all declaration-local invariants are satisfied.
    ///
    /// This does not establish semantic validity of referenced nodes.
    #[inline]
    #[must_use]
    pub fn is_locally_valid(&self) -> bool {
        self.validate_local().is_ok()
    }

    /// Visits every directly referenced child node in deterministic source
    /// order.
    ///
    /// The order is:
    ///
    /// 1. generic parameters;
    /// 2. `extends` references;
    /// 3. interface members.
    ///
    /// No AST-store lookup is performed.
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

        for id in &self.members {
            visitor(*id);
        }
    }

    /// Returns a snapshot of every directly referenced node ID in deterministic
    /// source-oriented relationship order.
    ///
    /// The returned vector contains IDs only; ownership of referenced AST
    /// nodes remains with the enclosing AST graph/store.
    pub fn referenced_nodes(&self) -> Vec<NodeId> {
        let capacity = self
            .generic_parameters
            .len()
            .saturating_add(self.extends.len())
            .saturating_add(self.members.len());

        let mut result = Vec::with_capacity(capacity);

        self.for_each_reference(|id| result.push(id));

        result
    }

    /// Returns the total number of direct AST references owned by this
    /// declaration.
    ///
    /// This is a structural count, not a language or hardware limit.
    #[inline]
    #[must_use]
    pub fn reference_count(&self) -> usize {
        self.generic_parameters
            .len()
            .saturating_add(self.extends.len())
            .saturating_add(self.members.len())
    }
}

impl AstNode for InterfaceDeclaration {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl fmt::Display for InterfaceDeclaration {
    /// Formats a bounded declaration-level representation.
    ///
    /// It deliberately does not recursively print referenced nodes or metadata
    /// because those structures can be arbitrarily large.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "interface {}", self.name)?;

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

        write!(
            formatter,
            " {{ {} member(s) }}",
            self.members.len()
        )
    }
}

/// Errors produced by declaration-local interface validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterfaceDeclarationError {
    /// The interface has an empty source-level name.
    EmptyName,

    /// The embedded canonical node has the wrong node kind.
    InvalidNodeKind {
        /// The node kind required by this declaration.
        expected: CoreNodeKind,

        /// The node kind actually stored in the node.
        actual: NodeKind,
    },
}

impl fmt::Display for InterfaceDeclarationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str(
                    "interface declaration name must not be empty",
                )
            }

            Self::InvalidNodeKind { expected, actual } => {
                write!(
                    formatter,
                    "interface declaration has invalid node kind: \
                     expected zamani:{}, found {}",
                    expected.name(),
                    actual
                )
            }
        }
    }
}

impl std::error::Error for InterfaceDeclarationError {}

/// Validates the declaration-local name invariant.
///
/// This deliberately checks only that a name exists.
///
/// Lexical identifier legality belongs to the lexer/parser and language
/// grammar. Unicode identifier policy, reserved words, normalization, and
/// namespace resolution must not be duplicated here.
fn validate_name(
    name: &str,
) -> Result<(), InterfaceDeclarationError> {
    if name.is_empty() {
        return Err(InterfaceDeclarationError::EmptyName);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::node_id::NodeId;
    use super::super::super::source::SourceId;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value)
            .expect("test node ID must be non-zero")
    }

    fn span() -> Span {
        Span::point(SourceId::from_raw(1), 0.into())
    }

    #[test]
    fn creates_valid_interface_declaration() {
        let declaration = InterfaceDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2)],
            vec![node_id(3), node_id(4)],
            vec![node_id(5), node_id(6)],
        )
        .expect("interface declaration should be valid");

        assert_eq!(declaration.name(), "Example");
        assert_eq!(
            declaration.kind().as_core(),
            Some(CoreNodeKind::Interface)
        );
        assert!(declaration.is_locally_valid());
        assert!(declaration.is_generic());
        assert!(declaration.has_extends());
        assert_eq!(declaration.member_count(), 2);
    }

    #[test]
    fn preserves_reference_order() {
        let declaration = InterfaceDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2), node_id(3)],
            vec![node_id(4), node_id(5)],
            vec![node_id(6), node_id(7), node_id(8)],
        )
        .expect("interface declaration should be valid");

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
            ]
        );
    }

    #[test]
    fn for_each_reference_is_deterministic() {
        let declaration = InterfaceDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2)],
            vec![node_id(3)],
            vec![node_id(4)],
        )
        .expect("interface declaration should be valid");

        let mut visited = Vec::new();

        declaration.for_each_reference(|id| {
            visited.push(id);
        });

        assert_eq!(
            visited,
            vec![
                node_id(2),
                node_id(3),
                node_id(4),
            ]
        );
    }

    #[test]
    fn reference_count_matches_all_relationships() {
        let declaration = InterfaceDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2), node_id(3)],
            vec![node_id(4)],
            vec![
                node_id(5),
                node_id(6),
                node_id(7),
            ],
        )
        .expect("interface declaration should be valid");

        assert_eq!(declaration.reference_count(), 6);
    }

    #[test]
    fn empty_name_is_rejected() {
        let result = InterfaceDeclaration::without_metadata(
            node_id(1),
            span(),
            "",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(InterfaceDeclarationError::EmptyName)
        );
    }

    #[test]
    fn empty_interface_is_valid() {
        let declaration = InterfaceDeclaration::without_metadata(
            node_id(1),
            span(),
            "EmptyInterface",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect("empty interface should be valid");

        assert!(declaration.is_empty());
        assert!(!declaration.has_extends());
        assert!(!declaration.has_relationships());
        assert_eq!(declaration.reference_count(), 0);
    }

    #[test]
    fn replacing_name_preserves_structure() {
        let mut declaration =
            InterfaceDeclaration::without_metadata(
                node_id(1),
                span(),
                "Before",
                vec![node_id(2)],
                vec![node_id(3)],
                vec![node_id(4), node_id(5)],
            )
            .expect("interface declaration should be valid");

        let previous = declaration
            .replace_name("After")
            .expect("replacement name should be valid");

        assert_eq!(previous, "Before");
        assert_eq!(declaration.name(), "After");
        assert_eq!(
            declaration.generic_parameter_count(),
            1
        );
        assert_eq!(declaration.extends_count(), 1);
        assert_eq!(declaration.member_count(), 2);
    }

    #[test]
    fn schema_contract_is_stable() {
        assert_eq!(
            InterfaceDeclaration::schema_version(),
            INTERFACE_DECLARATION_SCHEMA_VERSION
        );

        assert_eq!(
            InterfaceDeclaration::stable_kind_name(),
            INTERFACE_DECLARATION_KIND_NAME
        );
    }

    #[test]
    fn display_is_bounded_to_declaration_level() {
        let declaration = InterfaceDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2)],
            vec![node_id(3), node_id(4)],
            vec![node_id(5), node_id(6)],
        )
        .expect("interface declaration should be valid");

        assert_eq!(
            declaration.to_string(),
            "interface Example \
             <1 generic parameter(s)> \
             extends 2 \
             { 2 member(s) }"
                .replace("             ", "")
        );
    }

    #[test]
    fn ast_node_contract_is_delegated_to_node() {
        let declaration = InterfaceDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect("interface declaration should be valid");

        let ast_node: &dyn AstNode = &declaration;

        assert_eq!(ast_node.id(), node_id(1));
        assert_eq!(
            ast_node.kind().as_core(),
            Some(CoreNodeKind::Interface)
        );
    }

    #[test]
    fn replacing_relationships_preserves_source_order() {
        let mut declaration =
            InterfaceDeclaration::without_metadata(
                node_id(1),
                span(),
                "Example",
                vec![node_id(2)],
                vec![node_id(3)],
                vec![node_id(4)],
            )
            .expect("interface declaration should be valid");

        let old_generics = declaration
            .replace_generic_parameters(vec![
                node_id(10),
                node_id(11),
            ]);

        let old_extends =
            declaration.replace_extends(vec![
                node_id(12),
                node_id(13),
            ]);

        let old_members =
            declaration.replace_members(vec![
                node_id(14),
                node_id(15),
            ]);

        assert_eq!(old_generics, vec![node_id(2)]);
        assert_eq!(old_extends, vec![node_id(3)]);
        assert_eq!(old_members, vec![node_id(4)]);

        assert_eq!(
            declaration.referenced_nodes(),
            vec![
                node_id(10),
                node_id(11),
                node_id(12),
                node_id(13),
                node_id(14),
                node_id(15),
            ]
        );
    }

    #[test]
    fn generic_and_relationship_queries_are_consistent() {
        let declaration = InterfaceDeclaration::without_metadata(
            node_id(1),
            span(),
            "Example",
            vec![node_id(2)],
            vec![node_id(3)],
            vec![node_id(4)],
        )
        .expect("interface declaration should be valid");

        assert!(declaration.is_generic());
        assert_eq!(
            declaration.generic_parameter_count(),
            declaration.generic_parameters().len()
        );

        assert!(declaration.has_extends());
        assert_eq!(
            declaration.extends_count(),
            declaration.extends().len()
        );

        assert_eq!(
            declaration.member_count(),
            declaration.members().len()
        );
    }
}