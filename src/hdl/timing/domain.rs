
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL logical clock-domain model
//!
//! Path: `src/hdl/timing/domain.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Represent source-level logical clock domains and their declared
//! relationships without choosing or allocating physical hardware.
//!
//! ## Normative authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/clocks.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/hdl/timing.g4`
//! - `grammar/hdl/sequential.g4`
//! - `grammar/hdl/reset.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/ast/node/node_id.rs`
//! - `src/hdl/timing/clock.rs`
//!
//! The grammar owns syntax. The canonical AST owns source structure and
//! expression nodes. The semantic pipeline owns name resolution, type and
//! expression checking, clock/reset consistency, and semantic diagnostics.
//!
//! ## Owns
//!
//! - A logical clock-domain declaration's canonical AST identity and name.
//! - An ordered, loss-conscious view of domain properties, named blocks,
//!   and attributes.
//! - References from property values to canonical AST expression nodes.
//! - Open-world property names for language and dialect extensibility.
//! - Structural checks for declaration/item identities and named-block links.
//!
//! ## Does not own
//!
//! - A parser, a replacement AST, or a second expression representation.
//! - Clock or reset declarations as competing sources of semantic truth.
//! - Name resolution or proof that an expression denotes a clock or reset.
//! - Clock-domain crossing analysis or synchronizer insertion.
//! - Timing closure, scheduling, synthesis, placement, or routing.
//! - Physical clocks, clock trees, oscillators, PLLs, DLLs, or clock pins.
//! - Target selection, device discovery, or resource allocation.
//! - HDLIR structures, builders, verifiers, passes, serialization, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Representation and scalability
//!
//! This module reuses the canonical frontend `NodeId` and the shared semantic
//! `HdlId`. Property values remain references to canonical AST expression
//! nodes; they are not evaluated or converted into host integers here.
//!
//! Domain items use an ordered flat collection. Named blocks refer to their
//! containing block by AST identity, avoiding a second nested syntax tree.
//! Property order and repeated property names are preserved for semantic
//! validation.
//!
//! No language-level maximum is imposed on clock domains, properties, named
//! blocks, or clock-domain relationships. Collections are limited only by
//! available resources and any explicitly configured operational budgets
//! enforced by the owning compilation session.
//!
//! ## Integration contract
//!
//! The frontend adapter constructs [`HdlClockDomain`] from the canonical AST.
//! It must preserve the declaration's `NodeId`, source-level name, property
//! order, each property's canonical expression `NodeId`, and each named
//! block's parent relationship.
//!
//! `clock.rs`, `reset.rs`, sequential-process analysis, and future crossing
//! analysis may consume this view through explicit adapters. They must not
//! duplicate domain declarations or treat an unresolved expression as a
//! successfully resolved clock.
//!
//! The existing semantic validator remains authoritative. Adding this model
//! does not by itself enable domain validation; the frontend adapter and
//! semantic input/validation contract must be extended separately.
//!
//! ## Freeze criteria
//!
//! - Canonical AST identities and source ordering are preserved.
//! - Unknown property names remain representable.
//! - Invalid structural links are reported rather than silently repaired.
//! - Domain meaning and reference resolution remain with semantic analysis.
//! - The timing module and HDL composition root register this file.
//! - Unit tests and repository-wide Rust checks pass.

use core::fmt;
use std::collections::BTreeMap;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::domains::hdl::HdlId;

/// Version of the public in-memory clock-domain model.
///
/// This is not the Zamani language, grammar, or HDLIR version.
pub const HDL_CLOCK_DOMAIN_MODEL_VERSION: u16 = 1;

/// A validated source-level clock-domain property name.
///
/// Property names remain open-world. This type validates the basic shape of a
/// single name but does not decide whether the semantic layer supports it.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HdlClockDomainPropertyName(String);

impl HdlClockDomainPropertyName {
    /// Creates a property name without silently normalizing its spelling.
    pub fn new(value: impl Into<String>) -> Result<Self, HdlClockDomainModelError> {
        let value = value.into();
        validate_name("clock-domain property", &value)?;
        Ok(Self(value))
    }

    /// Returns the original property spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HdlClockDomainPropertyName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One item in a clock-domain declaration.
///
/// Items remain in source order. A named block is represented by its own AST
/// node, and child items refer to it through `parent_block`. This is a
/// semantic view over canonical AST nodes, not a second syntax tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlClockDomainItem {
    node_id: NodeId,
    parent_block: Option<NodeId>,
    kind: HdlClockDomainItemKind,
}

impl HdlClockDomainItem {
    /// Creates a property assignment.
    ///
    /// `node_id` identifies the property item. `value_expression` identifies
    /// the canonical AST expression assigned to that property.
    #[must_use]
    pub fn property(
        node_id: NodeId,
        parent_block: Option<NodeId>,
        name: HdlClockDomainPropertyName,
        value_expression: NodeId,
    ) -> Self {
        Self {
            node_id,
            parent_block,
            kind: HdlClockDomainItemKind::Property {
                name,
                value_expression,
            },
        }
    }

    /// Creates a named block.
    ///
    /// Block names are open-world. Whether a block name is supported or has
    /// particular meaning is decided by the semantic layer or an enabled
    /// extension.
    pub fn named_block(
        node_id: NodeId,
        parent_block: Option<NodeId>,
        name: HdlId,
    ) -> Result<Self, HdlClockDomainModelError> {
        validate_name("clock-domain named block", name.as_str())?;

        Ok(Self {
            node_id,
            parent_block,
            kind: HdlClockDomainItemKind::NamedBlock { name },
        })
    }

    /// Creates an attribute item.
    ///
    /// `node_id` identifies the canonical AST node representing the
    /// attribute. Attribute contents remain owned by the canonical AST.
    #[must_use]
    pub fn attribute(node_id: NodeId, parent_block: Option<NodeId>) -> Self {
        Self {
            node_id,
            parent_block,
            kind: HdlClockDomainItemKind::Attribute,
        }
    }

    /// Returns this item's canonical AST node identity.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Returns the containing named-block AST identity, if any.
    #[must_use]
    pub const fn parent_block(&self) -> Option<NodeId> {
        self.parent_block
    }

    /// Returns this item's kind.
    #[must_use]
    pub const fn kind(&self) -> &HdlClockDomainItemKind {
        &self.kind
    }

    /// Returns whether this item declares a named block.
    #[must_use]
    pub const fn is_named_block(&self) -> bool {
        matches!(self.kind, HdlClockDomainItemKind::NamedBlock { .. })
    }
}

/// The category of a clock-domain declaration item.
///
/// Property values refer to canonical AST expressions. No expression
/// evaluation, reference resolution, or target-specific interpretation occurs
/// in this module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlClockDomainItemKind {
    /// A named property assigned a canonical AST expression.
    Property {
        /// The property name, preserving its source spelling.
        name: HdlClockDomainPropertyName,

        /// Canonical AST node identifying the property's value expression.
        value_expression: NodeId,
    },

    /// An open-world named block containing other domain items.
    NamedBlock {
        /// Logical name of the block.
        name: HdlId,
    },

    /// An attribute attached to the declaration or a named block.
    Attribute,
}

/// A target-independent logical clock-domain declaration.
///
/// A clock domain is a semantic timing region, not a physical clock-tree
/// object. This structure can preserve symbolic and unresolved property
/// expressions until the canonical semantic pipeline validates them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlClockDomain {
    source_node: NodeId,
    name: HdlId,
    items: Vec<HdlClockDomainItem>,
}

impl HdlClockDomain {
    /// Creates a clock-domain view and validates its structural invariants.
    ///
    /// `items` must be in source order. Item identities must be unique and
    /// must not reuse the declaration identity. A `parent_block` must refer
    /// to an earlier named-block item in this declaration.
    ///
    /// This constructor does not resolve property expressions, establish
    /// clock-domain compatibility, validate reset behavior, or select
    /// physical hardware.
    pub fn new(
        source_node: NodeId,
        name: HdlId,
        items: Vec<HdlClockDomainItem>,
    ) -> Result<Self, HdlClockDomainModelError> {
        validate_name("clock domain", name.as_str())?;

        let mut item_nodes = BTreeMap::<NodeId, (bool, usize)>::new();

        for (position, item) in items.iter().enumerate() {
            if item.node_id == source_node {
                return Err(
                    HdlClockDomainModelError::DeclarationNodeReusedAsItem {
                        node: source_node,
                    },
                );
            }

            if item_nodes
                .insert(item.node_id, (item.is_named_block(), position))
                .is_some()
            {
                return Err(HdlClockDomainModelError::DuplicateAstNode {
                    node: item.node_id,
                });
            }
        }

        for (position, item) in items.iter().enumerate() {
            let Some(parent) = item.parent_block else {
                continue;
            };

            match item_nodes.get(&parent) {
                Some((true, parent_position)) if *parent_position < position => {}
                Some(_) | None => {
                    return Err(HdlClockDomainModelError::InvalidParentBlock {
                        item: item.node_id,
                        parent,
                    });
                }
            }
        }

        Ok(Self {
            source_node,
            name,
            items,
        })
    }

    /// Returns the canonical AST node identifying the declaration.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Returns the logical domain name.
    #[must_use]
    pub const fn name(&self) -> &HdlId {
        &self.name
    }

    /// Returns every domain item in source order, including repeated
    /// properties and unknown extension properties.
    #[must_use]
    pub fn items(&self) -> &[HdlClockDomainItem] {
        &self.items
    }

    /// Iterates over property assignments without allocating a new collection.
    pub fn properties(
        &self,
    ) -> impl Iterator<
        Item = (
            &HdlClockDomainItem,
            &HdlClockDomainPropertyName,
            NodeId,
        ),
    > {
        self.items.iter().filter_map(|item| {
            if let HdlClockDomainItemKind::Property {
                name,
                value_expression,
            } = &item.kind
            {
                Some((item, name, *value_expression))
            } else {
                None
            }
        })
    }

    /// Iterates over named blocks without allocating a new collection.
    pub fn named_blocks(
        &self,
    ) -> impl Iterator<Item = (&HdlClockDomainItem, &HdlId)> {
        self.items.iter().filter_map(|item| {
            if let HdlClockDomainItemKind::NamedBlock { name } = &item.kind {
                Some((item, name))
            } else {
                None
            }
        })
    }

    /// Iterates over properties with the exact requested spelling.
    ///
    /// An iterator is returned rather than a single value because the grammar
    /// does not establish a universal duplicate-property policy.
    pub fn properties_named<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = (&'a HdlClockDomainItem, NodeId)> + 'a {
        self.properties()
            .filter_map(move |(item, property_name, expression)| {
                (property_name.as_str() == name).then_some((item, expression))
            })
    }

    /// Iterates over expression references attached to `clock` properties.
    ///
    /// These are AST expression identities, not resolved clock identities.
    /// The semantic layer must resolve them before treating them as clocks.
    pub fn clock_references(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.properties_named("clock")
            .map(|(_, expression)| expression)
    }

    /// Iterates over expression references attached to `reset` properties.
    ///
    /// These are AST expression identities, not resolved reset identities.
    pub fn reset_references(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.properties_named("reset")
            .map(|(_, expression)| expression)
    }

    /// Iterates over expression references attached to `parent` properties.
    ///
    /// The semantic layer decides whether each expression denotes a valid
    /// parent domain and whether the resulting relationship is legal.
    pub fn parent_references(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.properties_named("parent")
            .map(|(_, expression)| expression)
    }
}

/// Structural errors detected while constructing a clock-domain view.
///
/// These errors indicate malformed adapter input. They are distinct from
/// semantic diagnostics such as unresolved clocks or incompatible domains.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlClockDomainModelError {
    /// A name is empty, contains whitespace, or contains control characters.
    InvalidName {
        /// The kind of name that failed validation.
        field: &'static str,

        /// The reason validation failed.
        reason: &'static str,
    },

    /// An item repeats an AST node identity already used by another item.
    DuplicateAstNode {
        /// Repeated canonical AST identity.
        node: NodeId,
    },

    /// An item reuses the declaration's own AST identity.
    DeclarationNodeReusedAsItem {
        /// Canonical declaration identity.
        node: NodeId,
    },

    /// An item refers to a missing, non-block, or later named-block item.
    InvalidParentBlock {
        /// The item containing the invalid parent reference.
        item: NodeId,

        /// The referenced parent identity.
        parent: NodeId,
    },
}

impl fmt::Display for HdlClockDomainModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName { field, reason } => {
                write!(formatter, "invalid {field} name: {reason}")
            }
            Self::DuplicateAstNode { node } => {
                write!(
                    formatter,
                    "duplicate AST node in clock-domain declaration: {node:?}"
                )
            }
            Self::DeclarationNodeReusedAsItem { node } => {
                write!(
                    formatter,
                    "clock-domain declaration AST node is reused as an item: {node:?}"
                )
            }
            Self::InvalidParentBlock { item, parent } => {
                write!(
                    formatter,
                    "clock-domain item {item:?} refers to invalid named-block parent {parent:?}"
                )
            }
        }
    }
}

impl std::error::Error for HdlClockDomainModelError {}

fn validate_name(
    field: &'static str,
    value: &str,
) -> Result<(), HdlClockDomainModelError> {
    if value.trim().is_empty() {
        return Err(HdlClockDomainModelError::InvalidName {
            field,
            reason: "name must not be empty",
        });
    }

    if value.chars().any(char::is_whitespace) {
        return Err(HdlClockDomainModelError::InvalidName {
            field,
            reason: "name must be one identifier",
        });
    }

    if value.chars().any(char::is_control) {
        return Err(HdlClockDomainModelError::InvalidName {
            field,
            reason: "name must not contain control characters",
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(value: u64) -> NodeId {
        NodeId::new(value).expect("test node IDs must be non-zero")
    }

    fn identifier(value: &str) -> HdlId {
        HdlId::new(value).expect("test identifier must not be empty")
    }

    fn property_name(value: &str) -> HdlClockDomainPropertyName {
        HdlClockDomainPropertyName::new(value)
            .expect("test property name must be valid")
    }

    #[test]
    fn preserves_unknown_properties_for_future_extensions() {
        let domain = HdlClockDomain::new(
            node(1),
            identifier("control"),
            vec![HdlClockDomainItem::property(
                node(2),
                None,
                property_name("vendor_extension"),
                node(3),
            )],
        )
        .expect("unknown property names remain representable");

        assert_eq!(
            domain
                .properties()
                .map(|(_, name, _)| name.as_str())
                .collect::<Vec<_>>(),
            vec!["vendor_extension"]
        );
    }

    #[test]
    fn preserves_property_order_and_duplicate_names() {
        let domain = HdlClockDomain::new(
            node(1),
            identifier("control"),
            vec![
                HdlClockDomainItem::property(
                    node(2),
                    None,
                    property_name("clock"),
                    node(20),
                ),
                HdlClockDomainItem::property(
                    node(3),
                    None,
                    property_name("clock"),
                    node(21),
                ),
                HdlClockDomainItem::property(
                    node(4),
                    None,
                    property_name("mode"),
                    node(22),
                ),
            ],
        )
        .expect("distinct item nodes should be accepted");

        assert_eq!(domain.properties_named("clock").count(), 2);
        assert_eq!(
            domain.clock_references().collect::<Vec<_>>(),
            vec![node(20), node(21)]
        );
    }

    #[test]
    fn supports_nested_named_blocks_with_flat_ordered_items() {
        let domain = HdlClockDomain::new(
            node(1),
            identifier("control"),
            vec![
                HdlClockDomainItem::named_block(
                    node(2),
                    None,
                    identifier("relationships"),
                )
                .expect("valid named block"),
                HdlClockDomainItem::property(
                    node(3),
                    Some(node(2)),
                    property_name("clock"),
                    node(30),
                ),
                HdlClockDomainItem::named_block(
                    node(4),
                    Some(node(2)),
                    identifier("constraints"),
                )
                .expect("valid nested block"),
                HdlClockDomainItem::property(
                    node(5),
                    Some(node(4)),
                    property_name("phase"),
                    node(31),
                ),
            ],
        )
        .expect("valid flat representation");

        assert_eq!(domain.items().len(), 4);
        assert_eq!(domain.named_blocks().count(), 2);
        assert_eq!(domain.items()[1].parent_block(), Some(node(2)));
        assert_eq!(domain.items()[3].parent_block(), Some(node(4)));
    }

    #[test]
    fn rejects_duplicate_item_node_ids() {
        let result = HdlClockDomain::new(
            node(1),
            identifier("control"),
            vec![
                HdlClockDomainItem::attribute(node(2), None),
                HdlClockDomainItem::attribute(node(2), None),
            ],
        );

        assert!(matches!(
            result,
            Err(HdlClockDomainModelError::DuplicateAstNode { node: repeated })
                if repeated == node(2)
        ));
    }

    #[test]
    fn rejects_reuse_of_the_declaration_node_as_an_item() {
        let result = HdlClockDomain::new(
            node(1),
            identifier("control"),
            vec![HdlClockDomainItem::attribute(node(1), None)],
        );

        assert!(matches!(
            result,
            Err(HdlClockDomainModelError::DeclarationNodeReusedAsItem { node: repeated })
                if repeated == node(1)
        ));
    }

    #[test]
    fn rejects_a_parent_that_is_not_a_named_block() {
        let result = HdlClockDomain::new(
            node(1),
            identifier("control"),
            vec![
                HdlClockDomainItem::attribute(node(2), None),
                HdlClockDomainItem::attribute(node(3), Some(node(2))),
            ],
        );

        assert!(matches!(
            result,
            Err(HdlClockDomainModelError::InvalidParentBlock { item, parent })
                if item == node(3) && parent == node(2)
        ));
    }

    #[test]
    fn rejects_a_parent_that_appears_after_its_child() {
        let result = HdlClockDomain::new(
            node(1),
            identifier("control"),
            vec![
                HdlClockDomainItem::property(
                    node(2),
                    Some(node(3)),
                    property_name("clock"),
                    node(20),
                ),
                HdlClockDomainItem::named_block(
                    node(3),
                    None,
                    identifier("relationships"),
                )
                .expect("valid named block"),
            ],
        );

        assert!(matches!(
            result,
            Err(HdlClockDomainModelError::InvalidParentBlock { item, parent })
                if item == node(2) && parent == node(3)
        ));
    }

    #[test]
    fn rejects_malformed_property_and_domain_names() {
        assert!(HdlClockDomainPropertyName::new("").is_err());
        assert!(HdlClockDomainPropertyName::new("clock domain").is_err());
        assert!(HdlClockDomainPropertyName::new("clock\n").is_err());

        let name = identifier("control domain");

        assert!(HdlClockDomain::new(node(1), name, Vec::new()).is_err());
    }
}
