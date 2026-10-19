#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Package Definition
//!
//! Path: `src/hdl/design/package.rs`
//! Rust baseline: 1.97 or later.
//!
//! ## Purpose
//!
//! Represents a target-independent HDL package declaration using references
//! to the canonical frontend AST. A package supplies a qualified source-level
//! identity and, optionally, ordered metadata and attribute references.
//!
//! This is a derived design-layer record, not a replacement AST, package
//! registry, semantic symbol table, or intermediate representation.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/hardware-modules.g4`
//! - `grammar/modules/packages.g4`
//! - `src/frontend/ast/node/node_id.rs`
//! - `src/frontend/ast/node/source/`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! The grammar owns package syntax. The canonical AST owns source nodes,
//! their identities, and their source spans. Shared semantic analysis owns
//! name resolution, visibility meaning, dependency resolution, and semantic
//! validation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - a package-specific representation of a qualified source-level name;
//! - the canonical AST identity of the package declaration;
//! - the canonical source span associated with the declaration;
//! - ordered references to package metadata fields and attributes;
//! - local validation of the references recorded by this design-layer value.
//!
//! It does not own the referenced AST nodes or their values.
//!
//! ## Package grammar contract
//!
//! The canonical package grammar permits a package declaration with either a
//! semicolon or a body. A body contains metadata fields and attributes.
//! Metadata values reuse the canonical static attribute-value grammar.
//!
//! Modules, imports, exports, dependencies, ordinary declarations, and package
//! resolution are not owned by this file.
//!
//! ## Scalability
//!
//! There are no artificial limits on package-name segments or body items.
//! Collections grow according to available memory and caller policy.
//! This module does not infer target capacities or physical hardware details.
//!
//! ## Determinism
//!
//! Package-name segments and body items preserve their supplied order.
//! Validation reports the first invalid reference in deterministic source
//! order. No global mutable state or generated identifiers are used.
//!
//! ## Deserialization
//!
//! Deserialization may bypass constructors. Consumers must call `validate`
//! before relying on this record's local invariants.
//!
//! ## HDLIR boundary
//!
//! This file defines no IR, IR builder, verifier, pass, lowering, or
//! serialization format for HDLIR. HDLIR implementation remains exclusively
//! under `src/hdl/hdlir/`.

use core::fmt;
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::source::Span;

/// Schema version for this in-memory HDL package record.
///
/// This is not the Zamani language version, AST serialization version,
/// package release version, or a future HDLIR schema version.
pub const HDL_PACKAGE_SCHEMA_VERSION: u16 = 1;

/// A qualified source-level HDL package name.
///
/// Each element is one name segment. The `::` separator is structural and
/// must not occur inside an individual segment.
///
/// This type does not establish whether the name resolves, whether the
/// package is visible in a particular scope, or whether two declarations
/// refer to the same semantic package.
///
/// Lexical identifier validity remains the responsibility of the canonical
/// lexer and grammar.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PackageName {
    segments: Vec<String>,
}

impl PackageName {
    /// Construct a package name from its ordered identifier segments.
    ///
    /// The caller should pass the segments extracted from the canonical
    /// qualified-name AST, rather than parse source text independently.
    ///
    /// # Errors
    ///
    /// Returns [`PackageNameError`] if the name is empty or a segment is
    /// empty, contains only whitespace, contains surrounding whitespace,
    /// or contains the reserved `::` separator.
    pub fn new<I, S>(segments: I) -> Result<Self, PackageNameError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let name = Self {
            segments: segments.into_iter().map(Into::into).collect(),
        };

        name.validate()?;
        Ok(name)
    }

    /// Return the ordered name segments.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Return the number of segments in this qualified name.
    #[must_use]
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// Return the final segment of this validated name.
    #[must_use]
    pub fn leaf(&self) -> &str {
        // A PackageName created through `new` or validated after
        // deserialization always contains at least one segment.
        self.segments.last().map(String::as_str).unwrap_or("")
    }

    /// Return the canonical structural representation using `::`.
    #[must_use]
    pub fn as_qualified_string(&self) -> String {
        self.segments.join("::")
    }

    /// Validate the invariants, including for deserialized values.
    ///
    /// This does not replace canonical lexical validation.
    ///
    /// # Errors
    ///
    /// Returns the first [`PackageNameError`] in segment order.
    pub fn validate(&self) -> Result<(), PackageNameError> {
        if self.segments.is_empty() {
            return Err(PackageNameError::EmptyName);
        }

        for (index, segment) in self.segments.iter().enumerate() {
            if segment.trim().is_empty() {
                return Err(PackageNameError::EmptySegment { index });
            }

            if segment.trim() != segment {
                return Err(PackageNameError::SurroundingWhitespace { index });
            }

            if segment.contains("::") {
                return Err(PackageNameError::EmbeddedSeparator { index });
            }
        }

        Ok(())
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, segment) in self.segments.iter().enumerate() {
            if index != 0 {
                f.write_str("::")?;
            }
            f.write_str(segment)?;
        }

        Ok(())
    }
}

/// Errors raised while constructing or validating a package name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PackageNameError {
    /// The name contains no segments.
    EmptyName,

    /// A segment is empty or consists only of whitespace.
    EmptySegment {
        /// Zero-based index of the invalid segment.
        index: usize,
    },

    /// A segment contains leading or trailing whitespace.
    SurroundingWhitespace {
        /// Zero-based index of the invalid segment.
        index: usize,
    },

    /// A segment contains the reserved `::` separator.
    EmbeddedSeparator {
        /// Zero-based index of the invalid segment.
        index: usize,
    },
}

impl fmt::Display for PackageNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                f.write_str("HDL package name must contain at least one segment")
            }
            Self::EmptySegment { index } => {
                write!(f, "HDL package name segment {index} is empty")
            }
            Self::SurroundingWhitespace { index } => {
                write!(
                    f,
                    "HDL package name segment {index} has surrounding whitespace"
                )
            }
            Self::EmbeddedSeparator { index } => {
                write!(
                    f,
                    "HDL package name segment {index} contains the reserved `::` separator"
                )
            }
        }
    }
}

impl std::error::Error for PackageNameError {}

/// A package metadata field referencing its canonical AST nodes.
///
/// The key is retained as source-level metadata text. Its lexical validity
/// belongs to the canonical grammar, and its meaning belongs to semantic
/// analysis. The field and value IDs refer to existing AST nodes; this type
/// never copies or evaluates the value.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackageMetadataField {
    field_node: NodeId,
    key: String,
    value_node: NodeId,
}

impl PackageMetadataField {
    /// Construct a metadata-field reference.
    ///
    /// `field_node` identifies the canonical metadata-field AST node;
    /// `value_node` identifies its canonical static attribute-value AST node.
    ///
    /// # Errors
    ///
    /// Returns [`PackageBodyError::EmptyMetadataKey`] if the key is empty
    /// or contains only whitespace. The canonical grammar remains responsible
    /// for deciding whether the key is a valid identifier.
    pub fn new(
        field_node: NodeId,
        key: impl Into<String>,
        value_node: NodeId,
    ) -> Result<Self, PackageBodyError> {
        let field = Self {
            field_node,
            key: key.into(),
            value_node,
        };

        field.validate()?;
        Ok(field)
    }

    /// Return the canonical AST identity of the metadata-field node.
    #[must_use]
    pub const fn field_node(&self) -> NodeId {
        self.field_node
    }

    /// Return the metadata key exactly as retained by the adapter.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Return the canonical AST identity of the metadata value.
    #[must_use]
    pub const fn value_node(&self) -> NodeId {
        self.value_node
    }

    /// Validate this field's local invariants.
    ///
    /// # Errors
    ///
    /// Returns [`PackageBodyError`] if the key is empty or whitespace-only,
    /// or if the field and value refer to the same AST node.
    pub fn validate(&self) -> Result<(), PackageBodyError> {
        if self.key.trim().is_empty() {
            return Err(PackageBodyError::EmptyMetadataKey);
        }

        if self.field_node == self.value_node {
            return Err(PackageBodyError::FieldAndValueShareNode {
                node: self.field_node,
            });
        }

        Ok(())
    }
}

/// An ordered item in a package body.
///
/// The package grammar admits metadata fields and attributes. This enum
/// preserves their original order without defining a second attribute AST.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PackageBodyItem {
    /// A metadata field and its canonical AST references.
    Metadata(PackageMetadataField),

    /// A reference to a canonical AST attribute node.
    Attribute {
        /// Canonical AST identity of the attribute node.
        node: NodeId,
    },
}

impl PackageBodyItem {
    /// Return the primary AST node identifying this body item.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        match self {
            Self::Metadata(field) => field.field_node(),
            Self::Attribute { node } => *node,
        }
    }

    /// Validate this item's local invariants.
    ///
    /// # Errors
    ///
    /// Returns an error if a metadata field violates its local contract.
    pub fn validate(&self) -> Result<(), PackageBodyError> {
        match self {
            Self::Metadata(field) => field.validate(),
            Self::Attribute { .. } => Ok(()),
        }
    }
}

/// Errors raised by package-body construction or validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PackageBodyError {
    /// A metadata key is empty or whitespace-only.
    EmptyMetadataKey,

    /// A metadata field and its value refer to the same AST node.
    FieldAndValueShareNode {
        /// The invalid shared node identity.
        node: NodeId,
    },

    /// The package declaration is incorrectly listed as one of its children.
    DeclarationUsedAsChild {
        /// The child role that refers to the declaration node.
        role: PackageReferenceRole,
        /// The repeated AST node identity.
        node: NodeId,
    },

    /// An AST node is referenced more than once in the package record.
    DuplicateReference {
        /// The role in which the repeated reference was found.
        role: PackageReferenceRole,
        /// The repeated AST node identity.
        node: NodeId,
    },
}

impl fmt::Display for PackageBodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMetadataKey => {
                f.write_str("HDL package metadata key cannot be empty")
            }
            Self::FieldAndValueShareNode { node } => {
                write!(
                    f,
                    "HDL package metadata field and value share AST node {node}"
                )
            }
            Self::DeclarationUsedAsChild { role, node } => {
                write!(
                    f,
                    "HDL package declaration node {node} is also used as {role}"
                )
            }
            Self::DuplicateReference { role, node } => {
                write!(
                    f,
                    "duplicate HDL package AST reference {node} in role {role}"
                )
            }
        }
    }
}

impl std::error::Error for PackageBodyError {}

/// The role of an AST reference recorded by a package design record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageReferenceRole {
    /// A metadata-field AST node.
    MetadataField,

    /// A metadata value AST node.
    MetadataValue,

    /// An attribute AST node.
    Attribute,
}

impl fmt::Display for PackageReferenceRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MetadataField => f.write_str("metadata field"),
            Self::MetadataValue => f.write_str("metadata value"),
            Self::Attribute => f.write_str("attribute"),
        }
    }
}

/// A validated, target-independent HDL package declaration record.
///
/// This record references canonical AST nodes; it does not own their contents.
/// The canonical AST remains the sole authority for source syntax, attribute
/// values, metadata value structure, visibility syntax, and source locations.
///
/// Package resolution, metadata interpretation, visibility rules, version
/// compatibility, and dependency resolution are downstream semantic concerns.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HdlPackage {
    declaration: NodeId,
    name: PackageName,
    span: Span,
    body_items: Vec<PackageBodyItem>,
}

impl HdlPackage {
    /// Construct a package record and validate its local invariants.
    ///
    /// `body_items` must preserve the source order of metadata fields and
    /// attributes. A package declared with a semicolon has an empty body-item
    /// list. The canonical AST still retains all source-level details.
    ///
    /// This constructor does not resolve names, interpret metadata, check
    /// package uniqueness, or inspect the AST store.
    ///
    /// # Errors
    ///
    /// Returns [`HdlPackageError`] for an invalid name or malformed body
    /// references.
    pub fn new(
        declaration: NodeId,
        name: PackageName,
        span: Span,
        body_items: Vec<PackageBodyItem>,
    ) -> Result<Self, HdlPackageError> {
        let package = Self {
            declaration,
            name,
            span,
            body_items,
        };

        package.validate()?;
        Ok(package)
    }

    /// Return the canonical AST identity of this package declaration.
    #[must_use]
    pub const fn declaration_id(&self) -> NodeId {
        self.declaration
    }

    /// Return the qualified package name.
    #[must_use]
    pub fn name(&self) -> &PackageName {
        &self.name
    }

    /// Return the canonical source span of the package declaration.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Return body items in source order.
    #[must_use]
    pub fn body_items(&self) -> &[PackageBodyItem] {
        &self.body_items
    }

    /// Return the number of body items.
    #[must_use]
    pub fn body_item_count(&self) -> usize {
        self.body_items.len()
    }

    /// Iterate over metadata fields in their original relative order.
    #[must_use]
    pub fn metadata_fields(
        &self,
    ) -> impl Iterator<Item = &PackageMetadataField> + '_ {
        self.body_items.iter().filter_map(|item| match item {
            PackageBodyItem::Metadata(field) => Some(field),
            PackageBodyItem::Attribute { .. } => None,
        })
    }

    /// Iterate over canonical attribute-node references in source order.
    #[must_use]
    pub fn attribute_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.body_items.iter().filter_map(|item| match item {
            PackageBodyItem::Metadata(_) => None,
            PackageBodyItem::Attribute { node } => Some(*node),
        })
    }

    /// Validate local invariants, including after deserialization.
    ///
    /// This method cannot verify that referenced IDs exist in the AST store
    /// or that each ID identifies the expected concrete node kind. Those
    /// checks require the canonical AST storage and structural validator.
    ///
    /// # Errors
    ///
    /// Returns the first local invariant violation in deterministic body order.
    pub fn validate(&self) -> Result<(), HdlPackageError> {
        self.name
            .validate()
            .map_err(HdlPackageError::InvalidName)?;

        let mut seen = BTreeSet::new();

        for item in &self.body_items {
            item.validate().map_err(HdlPackageError::InvalidBodyItem)?;

            match item {
                PackageBodyItem::Metadata(field) => {
                    Self::check_reference(
                        self.declaration,
                        &mut seen,
                        field.field_node(),
                        PackageReferenceRole::MetadataField,
                    )?;

                    Self::check_reference(
                        self.declaration,
                        &mut seen,
                        field.value_node(),
                        PackageReferenceRole::MetadataValue,
                    )?;
                }
                PackageBodyItem::Attribute { node } => {
                    Self::check_reference(
                        self.declaration,
                        &mut seen,
                        *node,
                        PackageReferenceRole::Attribute,
                    )?;
                }
            }
        }

        Ok(())
    }

    fn check_reference(
        declaration: NodeId,
        seen: &mut BTreeSet<NodeId>,
        node: NodeId,
        role: PackageReferenceRole,
    ) -> Result<(), HdlPackageError> {
        if node == declaration {
            return Err(HdlPackageError::InvalidBodyItem(
                PackageBodyError::DeclarationUsedAsChild { role, node },
            ));
        }

        if !seen.insert(node) {
            return Err(HdlPackageError::InvalidBodyItem(
                PackageBodyError::DuplicateReference { role, node },
            ));
        }

        Ok(())
    }
}

/// Errors raised while constructing or validating an HDL package record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlPackageError {
    /// The qualified package name is invalid.
    InvalidName(PackageNameError),

    /// A package body item violates its local invariants.
    InvalidBodyItem(PackageBodyError),
}

impl fmt::Display for HdlPackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(error) => {
                write!(f, "invalid HDL package name: {error}")
            }
            Self::InvalidBodyItem(error) => {
                write!(f, "invalid HDL package body: {error}")
            }
        }
    }
}

impl std::error::Error for HdlPackageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidName(error) => Some(error),
            Self::InvalidBodyItem(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_name_preserves_qualified_segments() {
        let name = PackageName::new(["org", "zamani", "hardware"])
            .expect("qualified package name should be valid");

        assert_eq!(name.segment_count(), 3);
        assert_eq!(name.leaf(), "hardware");
        assert_eq!(
            name.as_qualified_string(),
            "org::zamani::hardware"
        );
        assert_eq!(name.to_string(), "org::zamani::hardware");
    }

    #[test]
    fn package_name_rejects_empty_name() {
        assert_eq!(
            PackageName::new(Vec::<String>::new()),
            Err(PackageNameError::EmptyName)
        );
    }

    #[test]
    fn package_name_rejects_empty_qualified_segments() {
        assert_eq!(
            PackageName::new(["org", "", "hardware"]),
            Err(PackageNameError::EmptySegment { index: 1 })
        );

        assert_eq!(
            PackageName::new(["org", "   "]),
            Err(PackageNameError::EmptySegment { index: 1 })
        );
    }

    #[test]
    fn package_name_rejects_surrounding_whitespace() {
        assert_eq!(
            PackageName::new(["org", " hardware"]),
            Err(PackageNameError::SurroundingWhitespace { index: 1 })
        );
    }

    #[test]
    fn package_name_rejects_embedded_separator() {
        assert_eq!(
            PackageName::new(["org::zamani"]),
            Err(PackageNameError::EmbeddedSeparator { index: 0 })
        );
    }

    #[test]
    fn package_name_validation_catches_invalid_deserialized_shape() {
        let invalid = PackageName {
            segments: Vec::new(),
        };

        assert_eq!(invalid.validate(), Err(PackageNameError::EmptyName));
    }

    #[test]
    fn metadata_field_requires_a_nonempty_key() {
        let node = NodeId::first();

        assert_eq!(
            PackageMetadataField::new(node, "  ", NodeId::new(2).unwrap()),
            Err(PackageBodyError::EmptyMetadataKey)
        );
    }

    #[test]
    fn metadata_field_rejects_same_field_and_value_node() {
        let node = NodeId::first();

        assert_eq!(
            PackageMetadataField::new(node, "version", node),
            Err(PackageBodyError::FieldAndValueShareNode { node })
        );
    }

    #[test]
    fn package_body_items_preserve_metadata_and_attribute_order() {
        let field_node = NodeId::new(2).unwrap();
        let value_node = NodeId::new(3).unwrap();
        let attribute_node = NodeId::new(4).unwrap();

        let metadata = PackageMetadataField::new(
            field_node,
            "version",
            value_node,
        )
        .expect("metadata field should be valid");

        let items = [
            PackageBodyItem::Metadata(metadata),
            PackageBodyItem::Attribute {
                node: attribute_node,
            },
        ];

        assert_eq!(items[0].node_id(), field_node);
        assert_eq!(items[1].node_id(), attribute_node);
    }
}