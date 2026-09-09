//! # Zamani Native AST — Enum Declaration
//!
//! Production-ready source-level representation of an enum declaration.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//!    parser
//!      │
//!      ▼
//! ┌──────────────────────────────┐
//! │ Native Zamani AST            │
//! │                              │
//! │ EnumDeclaration              │
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//!      structural validation
//!                │
//!                ▼
//!        semantic analysis
//!                │
//!                ▼
//!          Semantic Model
//!                │
//!                ▼
//!               ZUIR
//!                │
//!        ┌───────┼────────┐
//!        ▼       ▼        ▼
//!    classical quantum   future
//!      domain   domain   domains
//! ```
//!
//! `EnumDeclaration` represents **source-language structure only**.
//!
//! It does not represent:
//!
//! - resolved types;
//! - resolved symbols;
//! - runtime values;
//! - memory layouts;
//! - ABI layouts;
//! - quantum hardware;
//! - physical qubits;
//! - quantum gates;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - backend instructions;
//! - scheduling;
//! - routing;
//! - error correction;
//! - calibration;
//! - execution state.
//!
//! Those concerns belong to later compiler phases.
//!
//! ## POCO-REAF
//!
//! An enum is represented independently of the machine on which it may later
//! be compiled or executed.
//!
//! The declaration therefore contains no:
//!
//! - fixed resource count;
//! - fixed machine size;
//! - fixed integer representation for discriminants;
//! - fixed ABI;
//! - fixed memory layout;
//! - target architecture;
//! - vendor information.
//!
//! A program can consequently be compiled for different targets without
//! changing the native AST representation.
//!
//! ## Source grammar contract
//!
//! The repository grammar describes the enum conceptually as:
//!
//! ```text
//! EnumDecl     = "enum" IDENT [ "<" TypeParams ">" ]
//!                "{" { EnumVariant } "}" ;
//!
//! EnumVariant  = IDENT [ EnumVariantKind ] [ "," ] ;
//! ```
//!
//! with variant forms including unit, tuple and struct-like variants.
//!
//! This AST node deliberately does not duplicate those variant definitions.
//! Instead, `variants` contains [`NodeId`] references to canonical variant
//! nodes owned by the AST graph.
//!
//! This allows the eventual `enum_variant.rs` implementation to evolve
//! independently without changing the identity or storage contract of this
//! declaration.
//!
//! ## Canonical graph model
//!
//! ```text
//! EnumDeclaration
//! ├── Node
//! ├── name
//! ├── generic_parameters ──► NodeId ──► GenericParameter
//! └── variants ─────────────► NodeId ──► EnumVariant
//!                                      │
//!                                      ├── unit
//!                                      ├── tuple fields
//!                                      └── struct fields
//! ```
//!
//! The referenced nodes are validated by the AST-store/structural-validation
//! layer. This file performs only validation that can be established locally.
//!
//! ## Why `NodeId` references are used
//!
//! The native AST uses graph-oriented identity rather than duplicating child
//! structures inside every parent declaration.
//!
//! This provides:
//!
//! - one authoritative node identity;
//! - deterministic traversal;
//! - compatibility with side-table semantic information;
//! - incremental compilation support;
//! - source mapping through child nodes;
//! - avoidance of duplicated AST ownership;
//! - extensibility;
//! - large-program scalability.
//!
//! `NodeId` does not represent a quantum resource, machine resource, symbol,
//! semantic type, backend object, or runtime object. It is solely AST identity.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational AST infrastructure:
//!
//! - [`Node`];
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`CoreNodeKind`];
//! - [`NodeMetadata`];
//! - [`Span`];
//! - the Rust standard library;
//! - `serde`.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - execution;
//! - runtime;
//! - backend providers;
//! - OpenQASM;
//! - QIR;
//! - LLVM;
//! - MLIR.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! ```text
//! parser
//!   │
//!   ├── allocate NodeId
//!   ├── parse enum name
//!   ├── parse generic parameter nodes
//!   ├── parse enum variant nodes
//!   ├── preserve source Span
//!   └── construct EnumDeclaration
//! ```
//!
//! The parser owns syntax recognition. It must not resolve field types,
//! generic substitutions, discriminants, layouts, or target representations.
//!
//! ### Structural validation
//!
//! ```text
//! EnumDeclaration
//!       │
//!       ├── validate local invariants
//!       │
//!       └── AST graph validator
//!               ├── NodeId exists
//!               ├── referenced node is GenericParameter
//!               └── referenced node is EnumVariant
//! ```
//!
//! ### Semantic analysis
//!
//! ```text
//! EnumDeclaration
//!       │
//!       ├── resolve name
//!       ├── resolve generic parameters
//!       ├── resolve variant names
//!       ├── resolve variant field types
//!       ├── detect duplicate names
//!       ├── determine semantic type
//!       └── produce semantic representation
//! ```
//!
//! This file must not perform those operations.
//!
//! ### ZUIR
//!
//! The enum declaration is lowered through the semantic model:
//!
//! ```text
//! EnumDeclaration
//!       │
//!       ▼
//! Semantic Model
//!       │
//!       ▼
//! ZUIR type/declaration semantics
//! ```
//!
//! The AST itself does not depend on ZUIR.
//!
//! ### Visitor/traversal
//!
//! Visitors and traversal infrastructure should visit:
//!
//! 1. the enum declaration;
//! 2. generic-parameter references;
//! 3. variant references;
//! 4. the concrete referenced nodes when traversing the AST graph.
//!
//! This file exposes deterministic child-reference iteration through
//! [`EnumDeclaration::referenced_nodes`].
//!
//! ## Determinism
//!
//! Source ordering is significant and therefore preserved exactly:
//!
//! - generic parameters remain in parser/source order;
//! - variants remain in parser/source order;
//! - `referenced_nodes()` preserves declaration order.
//!
//! No hash-based collection is used for source-ordered children.
//!
//! ## Scalability
//!
//! There is no language-level limit on:
//!
//! - number of enum declarations;
//! - number of generic parameters;
//! - number of variants;
//! - number of fields inside variants;
//! - program size.
//!
//! Collections are dynamically sized.
//!
//! Operational limits must be supplied by compiler configuration/validation
//! policy rather than embedded into this AST node.
//!
//! ## Security
//!
//! This type accepts compiler input that may ultimately originate from an
//! untrusted source.
//!
//! Local validation therefore rejects malformed empty names and invalid node
//! kinds without panicking.
//!
//! Cross-node validation belongs to the AST graph validator, where resource
//! limits can also be applied explicitly.
//!
//! No unchecked arithmetic, pointer arithmetic, raw pointers, global mutable
//! state, or `unsafe` code is used.
//!
//! ## Serialization
//!
//! The structure derives `Serialize` and `Deserialize` when the repository's
//! AST serialization layer enables `serde`.
//!
//! The serialization layer is responsible for:
//!
//! - AST schema versioning;
//! - compatibility;
//! - unknown-field policy;
//! - integrity checking;
//! - whole-graph reference validation.
//!
//! This node does not embed a serialization protocol version into its data.
//!
//! ## Rust compatibility
//!
//! Target compiler:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//!
//! Edition compatibility follows the repository's Cargo configuration.
//!
//! No `unsafe` code is permitted.

// -----------------------------------------------------------------------------
// Imports
// -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::fmt;

use super::super::metadata::NodeMetadata;
use super::super::node::AstNode;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

// -----------------------------------------------------------------------------
// Schema identity
// -----------------------------------------------------------------------------

/// In-memory schema version for [`EnumDeclaration`].
///
/// This version is independent from:
///
/// - Zamani language version;
/// - compiler version;
/// - serialized AST format version;
/// - extension version.
///
/// Increment it only when the public in-memory structure or its contract
/// changes incompatibly.
pub const ENUM_DECLARATION_SCHEMA_VERSION: u16 = 1;

/// Stable native AST kind identifier for enum declarations.
pub const ENUM_DECLARATION_KIND_NAME: &str = "zamani:enum";

// -----------------------------------------------------------------------------
// EnumDeclaration
// -----------------------------------------------------------------------------

/// A source-level enum declaration.
///
/// An enum consists of:
///
/// - a common [`Node`] containing identity, kind, source span and metadata;
/// - a source-level name;
/// - zero or more generic-parameter node references;
/// - zero or more enum-variant node references.
///
/// The actual variant definitions are deliberately owned by separate AST
/// nodes. This prevents `EnumDeclaration` from becoming coupled to the
/// representation of unit, tuple or struct-like variants.
///
/// # Invariants
///
/// A locally valid declaration satisfies:
///
/// 1. `name` is non-empty;
/// 2. `node.kind()` is `CoreNodeKind::Enum`;
/// 3. generic-parameter references contain no duplicate `NodeId`s;
/// 4. variant references contain no duplicate `NodeId`s;
/// 5. the declaration itself contains no semantic/backend state.
///
/// Cross-node invariants, including whether a referenced node really is a
/// generic parameter or enum variant, are checked by AST graph validation.
///
/// # Important semantic boundary
///
/// This type does **not** determine:
///
/// - whether a variant name is unique;
/// - whether a variant field type is valid;
/// - whether recursive variants are legal;
/// - discriminant representation;
/// - enum layout;
/// - ABI;
/// - niche optimization;
/// - memory representation.
///
/// Those are semantic/type-system/target concerns.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnumDeclaration {
    /// Common AST identity, source span and metadata.
    node: Node,

    /// Source spelling of the enum identifier.
    name: String,

    /// Generic-parameter nodes in source order.
    generic_parameters: Vec<NodeId>,

    /// Enum-variant nodes in source order.
    variants: Vec<NodeId>,
}

impl EnumDeclaration {
    /// Creates an enum declaration.
    ///
    /// This constructor establishes the structural representation but does
    /// not perform semantic analysis.
    ///
    /// # Errors
    ///
    /// Returns [`EnumDeclarationError::EmptyName`] when `name` is empty.
    ///
    /// Returns [`EnumDeclarationError::InvalidNodeKind`] when `node` is not
    /// classified as [`CoreNodeKind::Enum`].
    ///
    /// Duplicate child references are rejected because they make the source
    /// declaration graph ambiguous.
    pub fn new(
        node: Node,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        variants: Vec<NodeId>,
    ) -> Result<Self, EnumDeclarationError> {
        let name = name.into();

        validate_name(&name)?;

        if node.kind().as_core() != Some(CoreNodeKind::Enum) {
            return Err(EnumDeclarationError::InvalidNodeKind {
                actual: node.kind_owned(),
            });
        }

        validate_unique_ids(
            &generic_parameters,
            EnumReferenceRelation::GenericParameter,
        )?;

        validate_unique_ids(&variants, EnumReferenceRelation::Variant)?;

        Ok(Self {
            node,
            name,
            generic_parameters,
            variants,
        })
    }

    /// Creates an enum declaration using default node metadata.
    ///
    /// The caller must still supply the AST identity and source span.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        variants: Vec<NodeId>,
    ) -> Result<Self, EnumDeclarationError> {
        let node = Node::without_metadata(
            id,
            NodeKind::core(CoreNodeKind::Enum),
            span,
        );

        Self::new(
            node,
            name,
            generic_parameters,
            variants,
        )
    }

    /// Returns the embedded common AST node.
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the embedded common AST node.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns this declaration's stable AST identity.
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns this declaration's node kind.
    #[inline]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns this declaration's source span.
    #[inline]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns this declaration's metadata.
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable access to this declaration's metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the enum's source-level name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Replaces the enum's source-level name.
    ///
    /// The new name must be non-empty.
    ///
    /// Lexical identifier legality remains the responsibility of the parser
    /// and/or lexical validation layer.
    pub fn replace_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<String, EnumDeclarationError> {
        let name = name.into();
        validate_name(&name)?;

        Ok(std::mem::replace(&mut self.name, name))
    }

    /// Returns generic-parameter references in source order.
    #[inline]
    pub fn generic_parameters(&self) -> &[NodeId] {
        &self.generic_parameters
    }

    /// Returns mutable access to generic-parameter references.
    ///
    /// Callers changing this collection are responsible for preserving the
    /// uniqueness invariant. Use [`Self::replace_generic_parameters`] when
    /// validation is desired.
    #[inline]
    pub fn generic_parameters_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.generic_parameters
    }

    /// Replaces generic-parameter references after validating uniqueness.
    pub fn replace_generic_parameters(
        &mut self,
        generic_parameters: Vec<NodeId>,
    ) -> Result<Vec<NodeId>, EnumDeclarationError> {
        validate_unique_ids(
            &generic_parameters,
            EnumReferenceRelation::GenericParameter,
        )?;

        Ok(std::mem::replace(
            &mut self.generic_parameters,
            generic_parameters,
        ))
    }

    /// Returns the number of generic parameters.
    #[inline]
    pub fn generic_parameter_count(&self) -> usize {
        self.generic_parameters.len()
    }

    /// Returns whether the enum declares at least one generic parameter.
    #[inline]
    pub fn is_generic(&self) -> bool {
        !self.generic_parameters.is_empty()
    }

    /// Returns enum-variant references in source order.
    #[inline]
    pub fn variants(&self) -> &[NodeId] {
        &self.variants
    }

    /// Returns mutable access to variant references.
    ///
    /// Callers changing this collection are responsible for preserving the
    /// uniqueness invariant. Use [`Self::replace_variants`] when validation
    /// is desired.
    #[inline]
    pub fn variants_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.variants
    }

    /// Replaces variant references after validating uniqueness.
    pub fn replace_variants(
        &mut self,
        variants: Vec<NodeId>,
    ) -> Result<Vec<NodeId>, EnumDeclarationError> {
        validate_unique_ids(&variants, EnumReferenceRelation::Variant)?;

        Ok(std::mem::replace(&mut self.variants, variants))
    }

    /// Returns the number of variants.
    #[inline]
    pub fn variant_count(&self) -> usize {
        self.variants.len()
    }

    /// Returns whether the enum has no variants.
    ///
    /// Whether empty enums are legal in the Zamani language is a semantic
    /// language rule and is therefore deliberately not rejected here.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.variants.is_empty()
    }

    /// Returns the AST schema version for this declaration.
    #[inline]
    pub const fn schema_version() -> u16 {
        ENUM_DECLARATION_SCHEMA_VERSION
    }

    /// Returns the stable qualified AST kind name.
    #[inline]
    pub const fn stable_kind_name() -> &'static str {
        ENUM_DECLARATION_KIND_NAME
    }

    /// Validates invariants that can be established without access to the
    /// complete AST node store.
    ///
    /// Cross-node references are intentionally not resolved here.
    pub fn validate_local(&self) -> Result<(), EnumDeclarationError> {
        validate_name(&self.name)?;

        if self.node.kind().as_core() != Some(CoreNodeKind::Enum) {
            return Err(EnumDeclarationError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        validate_unique_ids(
            &self.generic_parameters,
            EnumReferenceRelation::GenericParameter,
        )?;

        validate_unique_ids(
            &self.variants,
            EnumReferenceRelation::Variant,
        )?;

        Ok(())
    }

    /// Returns `true` when all local invariants are satisfied.
    ///
    /// This does not prove that referenced `NodeId`s exist in an AST store.
    #[inline]
    pub fn is_locally_valid(&self) -> bool {
        self.validate_local().is_ok()
    }

    /// Returns every direct child-node reference in deterministic source
    /// order.
    ///
    /// Generic parameters precede variants because that mirrors the logical
    /// declaration structure.
    pub fn referenced_nodes(&self) -> Vec<NodeId> {
        let mut result = Vec::with_capacity(
            self.generic_parameters.len()
                .saturating_add(self.variants.len()),
        );

        result.extend(self.generic_parameters.iter().copied());
        result.extend(self.variants.iter().copied());

        result
    }

    /// Visits every direct child reference without allocating a combined
    /// collection.
    ///
    /// This is useful for large ASTs where avoiding an intermediate `Vec` is
    /// preferable.
    pub fn for_each_reference<F>(&self, mut visitor: F)
    where
        F: FnMut(NodeId),
    {
        for id in &self.generic_parameters {
            visitor(*id);
        }

        for id in &self.variants {
            visitor(*id);
        }
    }

    /// Returns the relation represented by a particular direct child
    /// reference.
    ///
    /// This helper avoids requiring traversal code to infer the relation from
    /// collection position.
    pub fn reference_relation(
        &self,
        id: NodeId,
    ) -> Option<EnumReferenceRelation> {
        if self.generic_parameters.contains(&id) {
            Some(EnumReferenceRelation::GenericParameter)
        } else if self.variants.contains(&id) {
            Some(EnumReferenceRelation::Variant)
        } else {
            None
        }
    }
}

impl AstNode for EnumDeclaration {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl fmt::Display for EnumDeclaration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("enum ")?;
        formatter.write_str(&self.name)?;

        if !self.generic_parameters.is_empty() {
            formatter.write_str("<")?;

            for (index, parameter) in
                self.generic_parameters.iter().enumerate()
            {
                if index != 0 {
                    formatter.write_str(", ")?;
                }

                write!(formatter, "{parameter}")?;
            }

            formatter.write_str(">")?;
        }

        formatter.write_str(" {")?;

        if !self.variants.is_empty() {
            formatter.write_str(" ")?;

            for (index, variant) in self.variants.iter().enumerate() {
                if index != 0 {
                    formatter.write_str(", ")?;
                }

                write!(formatter, "{variant}")?;
            }

            formatter.write_str(" ")?;
        }

        formatter.write_str("}")
    }
}

// -----------------------------------------------------------------------------
// Validation errors
// -----------------------------------------------------------------------------

/// Errors that can be established while validating an [`EnumDeclaration`]
/// without accessing the complete AST graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnumDeclarationError {
    /// The enum name is empty.
    EmptyName,

    /// The embedded node is not classified as an enum declaration.
    InvalidNodeKind {
        /// Actual node kind supplied by the caller.
        actual: NodeKind,
    },

    /// A direct child reference occurs more than once.
    DuplicateReference {
        /// Relationship represented by the duplicated reference.
        relation: EnumReferenceRelation,

        /// Duplicated AST node identity.
        node_id: NodeId,
    },
}

impl fmt::Display for EnumDeclarationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("enum declaration name must not be empty")
            }

            Self::InvalidNodeKind { actual } => write!(
                formatter,
                "enum declaration requires node kind {}, found {}",
                CoreNodeKind::Enum.name(),
                actual
            ),

            Self::DuplicateReference {
                relation,
                node_id,
            } => write!(
                formatter,
                "enum declaration contains duplicate {} reference {}",
                relation,
                node_id
            ),
        }
    }
}

impl std::error::Error for EnumDeclarationError {}

// -----------------------------------------------------------------------------
// Reference relation
// -----------------------------------------------------------------------------

/// Identifies the role of a direct child-node reference of an enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnumReferenceRelation {
    /// Reference to a generic parameter node.
    GenericParameter,

    /// Reference to an enum variant node.
    Variant,
}

impl fmt::Display for EnumReferenceRelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenericParameter => formatter.write_str("generic-parameter"),
            Self::Variant => formatter.write_str("variant"),
        }
    }
}

// -----------------------------------------------------------------------------
// Local helpers
// -----------------------------------------------------------------------------

/// Validates the minimal structural requirements of an enum name.
///
/// This intentionally does not implement the full Zamani identifier grammar.
/// Identifier syntax belongs to the lexer/parser contract so that this AST
/// layer does not accidentally duplicate or diverge from the language grammar.
fn validate_name(name: &str) -> Result<(), EnumDeclarationError> {
    if name.is_empty() {
        Err(EnumDeclarationError::EmptyName)
    } else {
        Ok(())
    }
}

/// Ensures that a source-order child-reference collection contains no
/// duplicate AST identities.
///
/// A small local O(n²) check is deliberately used here rather than introducing
/// a `HashSet` solely for construction-time validation. This keeps the node
/// independent of hashing implementation details and preserves a minimal
/// dependency surface.
///
/// Large-scale validation should use the AST validation layer, where the
/// compiler can select a more appropriate strategy for the complete graph.
fn validate_unique_ids(
    ids: &[NodeId],
    relation: EnumReferenceRelation,
) -> Result<(), EnumDeclarationError> {
    for (index, id) in ids.iter().enumerate() {
        if ids[..index].contains(id) {
            return Err(EnumDeclarationError::DuplicateReference {
                relation,
                node_id: *id,
            });
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::node_id::NodeId;

    fn id(value: u64) -> NodeId {
        NodeId::new(value).expect("test IDs must be non-zero")
    }

    fn span() -> Span {
        Span::default()
    }

    fn declaration() -> EnumDeclaration {
        EnumDeclaration::without_metadata(
            id(1),
            span(),
            "Example",
            vec![id(2), id(3)],
            vec![id(4), id(5), id(6)],
        )
        .expect("test enum should be valid")
    }

    #[test]
    fn creates_valid_enum() {
        let value = declaration();

        assert_eq!(value.name(), "Example");
        assert_eq!(value.generic_parameter_count(), 2);
        assert_eq!(value.variant_count(), 3);
        assert!(value.is_generic());
        assert!(!value.is_empty());
        assert!(value.is_locally_valid());
    }

    #[test]
    fn uses_canonical_enum_kind() {
        let value = declaration();

        assert_eq!(
            value.kind().as_core(),
            Some(CoreNodeKind::Enum)
        );
    }

    #[test]
    fn exposes_stable_schema_information() {
        assert_eq!(
            EnumDeclaration::schema_version(),
            ENUM_DECLARATION_SCHEMA_VERSION
        );

        assert_eq!(
            EnumDeclaration::stable_kind_name(),
            "zamani:enum"
        );
    }

    #[test]
    fn rejects_empty_name() {
        let result = EnumDeclaration::without_metadata(
            id(1),
            span(),
            "",
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(EnumDeclarationError::EmptyName)
        );
    }

    #[test]
    fn preserves_generic_parameter_order() {
        let value = EnumDeclaration::without_metadata(
            id(1),
            span(),
            "Example",
            vec![id(7), id(3), id(9)],
            Vec::new(),
        )
        .expect("enum should be valid");

        assert_eq!(
            value.generic_parameters(),
            &[id(7), id(3), id(9)]
        );
    }

    #[test]
    fn preserves_variant_order() {
        let value = EnumDeclaration::without_metadata(
            id(1),
            span(),
            "Example",
            Vec::new(),
            vec![id(8), id(2), id(6)],
        )
        .expect("enum should be valid");

        assert_eq!(
            value.variants(),
            &[id(8), id(2), id(6)]
        );
    }

    #[test]
    fn preserves_reference_order() {
        let value = declaration();

        assert_eq!(
            value.referenced_nodes(),
            vec![
                id(2),
                id(3),
                id(4),
                id(5),
                id(6)
            ]
        );
    }

    #[test]
    fn reference_iteration_is_deterministic() {
        let value = declaration();
        let mut actual = Vec::new();

        value.for_each_reference(|node_id| {
            actual.push(node_id);
        });

        assert_eq!(
            actual,
            vec![
                id(2),
                id(3),
                id(4),
                id(5),
                id(6)
            ]
        );
    }

    #[test]
    fn identifies_reference_relation() {
        let value = declaration();

        assert_eq!(
            value.reference_relation(id(2)),
            Some(EnumReferenceRelation::GenericParameter)
        );

        assert_eq!(
            value.reference_relation(id(4)),
            Some(EnumReferenceRelation::Variant)
        );

        assert_eq!(
            value.reference_relation(id(99)),
            None
        );
    }

    #[test]
    fn rejects_duplicate_generic_reference() {
        let result = EnumDeclaration::without_metadata(
            id(1),
            span(),
            "Example",
            vec![id(2), id(2)],
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(EnumDeclarationError::DuplicateReference {
                relation: EnumReferenceRelation::GenericParameter,
                node_id: id(2),
            })
        );
    }

    #[test]
    fn rejects_duplicate_variant_reference() {
        let result = EnumDeclaration::without_metadata(
            id(1),
            span(),
            "Example",
            Vec::new(),
            vec![id(2), id(2)],
        );

        assert_eq!(
            result,
            Err(EnumDeclarationError::DuplicateReference {
                relation: EnumReferenceRelation::Variant,
                node_id: id(2),
            })
        );
    }

    #[test]
    fn empty_enum_is_structurally_representable() {
        let value = EnumDeclaration::without_metadata(
            id(1),
            span(),
            "Empty",
            Vec::new(),
            Vec::new(),
        )
        .expect("empty enum should remain structurally representable");

        assert!(value.is_empty());
        assert!(value.is_locally_valid());
    }

    #[test]
    fn generic_enum_is_detected() {
        let value = EnumDeclaration::without_metadata(
            id(1),
            span(),
            "OptionLike",
            vec![id(2)],
            vec![id(3), id(4)],
        )
        .expect("enum should be valid");

        assert!(value.is_generic());
        assert_eq!(value.generic_parameter_count(), 1);
    }

    #[test]
    fn name_can_be_replaced() {
        let mut value = declaration();

        let previous = value
            .replace_name("Renamed")
            .expect("replacement should be valid");

        assert_eq!(previous, "Example");
        assert_eq!(value.name(), "Renamed");
    }

    #[test]
    fn empty_replacement_name_is_rejected() {
        let mut value = declaration();

        let result = value.replace_name("");

        assert_eq!(
            result,
            Err(EnumDeclarationError::EmptyName)
        );

        assert_eq!(value.name(), "Example");
    }

    #[test]
    fn display_is_deterministic() {
        let value = declaration();

        assert_eq!(
            value.to_string(),
            "enum Example<2, 3> { 4, 5, 6 }"
        );
    }

    #[test]
    fn enum_implements_ast_node_contract() {
        let value = declaration();

        assert_eq!(AstNode::id(&value), id(1));
        assert_eq!(
            AstNode::kind(&value).as_core(),
            Some(CoreNodeKind::Enum)
        );
    }

    #[test]
    fn local_validation_does_not_require_child_nodes() {
        // Child IDs are intentionally not resolved here. The AST graph
        // validator owns that responsibility.
        let value = EnumDeclaration::without_metadata(
            id(1),
            span(),
            "Example",
            vec![id(10_000)],
            vec![id(20_000)],
        )
        .expect("unresolved references are still structurally representable");

        assert!(value.is_locally_valid());
    }

    #[test]
    fn serde_round_trip_preserves_declaration() {
        let value = declaration();

        let encoded =
            serde_json::to_string(&value).expect("serialization should work");

        let decoded: EnumDeclaration =
            serde_json::from_str(&encoded)
                .expect("deserialization should work");

        assert_eq!(decoded, value);
    }
}