
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL logical clock model
//!
//! Path: `src/hdl/timing/clock.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.
//! Safety: safe Rust only.
//!
//! ## Status
//!
//! Target-independent semantic adapter contract. This module defines a
//! loss-conscious representation of clock declarations and their properties.
//! It is not, by itself, a complete HDL parser or semantic validator.
//!
//! ## Normative authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/clocks.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/hdl/timing.g4`
//! - `grammar/hdl/reset.g4`
//! - `grammar/hdl/sequential.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/ast/node/node_id.rs`
//!
//! Grammar and specifications own syntax and meaning. The canonical frontend
//! AST owns source structure and expression nodes. The semantic pipeline owns
//! name resolution, type checking, dimensional analysis, and semantic
//! diagnostics.
//!
//! ## Owns
//!
//! - A clock declaration's canonical AST identity and logical name.
//! - An optional reference to its canonical type expression.
//! - An ordered, flat view of clock properties, named blocks, and attributes.
//! - References from property values to canonical AST expression nodes.
//! - A small vocabulary for recognizing standard clock-property names while
//!   retaining arbitrary property names for future extensions.
//! - Structural checks needed to construct a consistent clock view.
//!
//! ## Does not own
//!
//! - Parsing or a replacement AST.
//! - A second expression representation or expression evaluator.
//! - Name resolution or proof that a property expression denotes a clock.
//! - Frequency/period dimensional analysis or conversion.
//! - Clock-domain declarations or clocking relationships as a second model.
//! - Reset semantics, sequential-event validation, or CDC analysis.
//! - Physical oscillators, PLLs, DLLs, clock trees, pins, buffers, or routing.
//! - Target discovery, target feasibility, synthesis, timing closure, or
//!   physical resource allocation.
//! - HDLIR types, builders, verification, passes, serialization, or lowering.
//!   All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Representation and scalability
//!
//! Expressions and source identities reuse the canonical `NodeId`. This file
//! does not copy or reinterpret expression syntax. A symbolic period,
//! frequency, phase, duty cycle, or other property remains an expression
//! reference until the canonical semantic services can establish its meaning.
//!
//! Clock items use a flat ordered sequence. A named block is identified by its
//! AST node, and its children refer to that node. This avoids introducing a
//! second recursively nested syntax tree and preserves source order.
//!
//! There is no language-level maximum for clocks, properties, blocks,
//! frequency, period, or clock-domain count. Standard Rust collections remain
//! bounded by available memory; the owning parser and analysis session must
//! enforce any configured operational budgets. Budget exhaustion must never
//! be reported as successful complete analysis.
//!
//! ## Integration contract
//!
//! The frontend adapter constructs `HdlClock` from the existing AST:
//!
//! - use the clock declaration's canonical `NodeId`;
//! - preserve its source-level name and optional type-expression node;
//! - visit its clock items in source order;
//! - preserve each property value as its canonical expression `NodeId`;
//! - flatten nested named blocks while setting `parent_block` to the
//!   corresponding named-block AST node;
//! - preserve attributes as references to canonical AST nodes.
//!
//! The semantic validator remains responsible for resolving names and
//! interpreting property values. In particular, recognizing `period` or
//! `frequency` here does not establish a unit, value, target capability, or
//! period/frequency consistency.
//!
//! `timing/domain.rs`, `timing/reset.rs`, `timing/constraints.rs`, and
//! `logic/sequential.rs` should consume canonical clock identities or
//! expression references through explicit adapters. They must not create
//! competing clock declarations or redefine source-language meaning.
//!
//! ## Freeze criteria
//!
//! - The AST adapter preserves identities, values, nesting, and source order.
//! - The grammar and semantic validator agree on property meaning.
//! - Symbolic and unknown values remain distinguishable from invalid values.
//! - Structural errors are reported explicitly, not silently repaired.
//! - The parent timing module and HDL composition root register this module.
//! - Unit tests and repository-wide formatting, check, test, and Clippy runs
//!   pass before the API is declared production-ready.

use core::fmt;
use std::collections::BTreeMap;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::domains::hdl::HdlId;

/// Version of the public in-memory logical clock model.
///
/// This is not the Zamani language version, grammar version, or HDLIR version.
pub const HDL_CLOCK_MODEL_VERSION: u16 = 1;

/// A validated source-level clock property name.
///
/// Names are preserved exactly as supplied by the frontend adapter. Standard
/// names can be recognized with [`HdlClockPropertyName::standard_kind`];
/// unknown names remain valid so the model can support future HDL extensions.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HdlClockPropertyName(String);

impl HdlClockPropertyName {
    /// Construct a property name without silently normalizing it.
    ///
    /// The frontend adapter should normally pass the spelling of the
    /// identifier already accepted by the canonical lexer/parser.
    pub fn new(value: impl Into<String>) -> Result<Self, HdlClockModelError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HdlClockModelError::InvalidName {
                field: "clock property",
                reason: "name must not be empty",
            });
        }

        if value.chars().any(char::is_whitespace) {
            return Err(HdlClockModelError::InvalidName {
                field: "clock property",
                reason: "name must be one identifier",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(HdlClockModelError::InvalidName {
                field: "clock property",
                reason: "name must not contain control characters",
            });
        }

        Ok(Self(value))
    }

    /// Return the original property spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Classify a recognized standard property, if any.
    ///
    /// This is a vocabulary helper, not semantic validation. A returned kind
    /// does not imply that the property's expression has a valid type, unit,
    /// value, or relationship to another property.
    #[must_use]
    pub fn standard_kind(&self) -> Option<StandardClockProperty> {
        Some(match self.0.as_str() {
            "period" => StandardClockProperty::Period,
            "frequency" => StandardClockProperty::Frequency,
            "duty_cycle" => StandardClockProperty::DutyCycle,
            "phase" => StandardClockProperty::Phase,
            "jitter" => StandardClockProperty::Jitter,
            "uncertainty" => StandardClockProperty::Uncertainty,
            "skew" => StandardClockProperty::Skew,
            "edge" => StandardClockProperty::Edge,
            "polarity" => StandardClockProperty::Polarity,
            "source" => StandardClockProperty::Source,
            "parent" => StandardClockProperty::Parent,
            "domain" => StandardClockProperty::Domain,
            "enable" => StandardClockProperty::Enable,
            "divide" => StandardClockProperty::Divide,
            "multiply" => StandardClockProperty::Multiply,
            _ => return None,
        })
    }
}

impl fmt::Display for HdlClockPropertyName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Standard clock-property names recognized by this adapter.
///
/// This enum is deliberately non-exhaustive. The property-name model remains
/// open-world, and additional standard properties can be introduced without
/// making unknown property names invalid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum StandardClockProperty {
    /// Requested or declared clock period.
    Period,
    /// Requested or declared clock frequency.
    Frequency,
    /// Duty-cycle intent.
    DutyCycle,
    /// Phase intent.
    Phase,
    /// Jitter intent.
    Jitter,
    /// Timing uncertainty intent.
    Uncertainty,
    /// Clock skew intent.
    Skew,
    /// Edge-selection intent.
    Edge,
    /// Polarity intent.
    Polarity,
    /// Logical source relationship.
    Source,
    /// Logical parent relationship.
    Parent,
    /// Logical clock-domain association.
    Domain,
    /// Clock-enable intent.
    Enable,
    /// Generated-clock division ratio.
    Divide,
    /// Generated-clock multiplication ratio.
    Multiply,
}

/// A single item in a clock declaration's source-level specification.
///
/// Items are kept in source order. A named block is represented by its own
/// item node; children identify that block through `parent_block`. This is a
/// semantic view over canonical AST nodes, not a second AST.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlClockItem {
    node_id: NodeId,
    parent_block: Option<NodeId>,
    kind: HdlClockItemKind,
}

impl HdlClockItem {
    /// Construct a property assignment.
    ///
    /// `node_id` identifies the source property item. `value_expression`
    /// identifies the canonical AST expression used as its value.
    pub fn property(
        node_id: NodeId,
        parent_block: Option<NodeId>,
        name: HdlClockPropertyName,
        value_expression: NodeId,
    ) -> Self {
        Self {
            node_id,
            parent_block,
            kind: HdlClockItemKind::Property {
                name,
                value_expression,
            },
        }
    }

    /// Construct a named block, such as `generated`, `source`, or `gating`.
    ///
    /// The block name is intentionally open-world. The semantic layer decides
    /// whether a block is defined by the active language version or extension.
    pub fn named_block(
        node_id: NodeId,
        parent_block: Option<NodeId>,
        name: HdlId,
    ) -> Result<Self, HdlClockModelError> {
        validate_identifier("clock named block", name.as_str())?;

        Ok(Self {
            node_id,
            parent_block,
            kind: HdlClockItemKind::NamedBlock { name },
        })
    }

    /// Construct a reference to a source-level clock attribute.
    ///
    /// The item node itself is the canonical AST node for the attribute.
    pub fn attribute(node_id: NodeId, parent_block: Option<NodeId>) -> Self {
        Self {
            node_id,
            parent_block,
            kind: HdlClockItemKind::Attribute,
        }
    }

    /// Return this item's canonical AST node identity.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Return the containing named-block AST identity, if nested.
    #[must_use]
    pub const fn parent_block(&self) -> Option<NodeId> {
        self.parent_block
    }

    /// Return the item kind without transferring ownership.
    #[must_use]
    pub const fn kind(&self) -> &HdlClockItemKind {
        &self.kind
    }

    /// Return the named-block identity if this item opens a named block.
    #[must_use]
    pub const fn is_named_block(&self) -> bool {
        matches!(self.kind, HdlClockItemKind::NamedBlock { .. })
    }
}

/// The meaning category of one clock declaration item.
///
/// Values that depend on expressions always refer to canonical AST nodes.
/// No expression is evaluated or converted into a machine-sized number here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlClockItemKind {
    /// A named property assigned a canonical expression.
    Property {
        /// Property name, preserving its source spelling.
        name: HdlClockPropertyName,
        /// Canonical AST node for the property's value expression.
        value_expression: NodeId,
    },

    /// An open-world named block containing other clock items.
    NamedBlock {
        /// Logical name of the block.
        name: HdlId,
    },

    /// An attribute attached to the clock declaration or a named block.
    Attribute,
}

/// A target-independent logical clock declaration.
///
/// This structure intentionally stores references to canonical AST nodes
/// rather than copied expression syntax. It can represent an incomplete or
/// symbolic declaration; completeness and semantic validity are established
/// by the existing frontend semantic pipeline.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlClock {
    source_node: NodeId,
    name: HdlId,
    type_expression: Option<NodeId>,
    items: Vec<HdlClockItem>,
}

impl HdlClock {
    /// Construct a logical clock view and validate its structural references.
    ///
    /// `items` must be in source order. Each item node must be unique within
    /// this declaration, and each `parent_block` must refer to a named-block
    /// item in this same declaration. The constructor does not validate
    /// property meaning, name resolution, units, or target feasibility.
    pub fn new(
        source_node: NodeId,
        name: HdlId,
        type_expression: Option<NodeId>,
        items: Vec<HdlClockItem>,
    ) -> Result<Self, HdlClockModelError> {
        validate_identifier("clock", name.as_str())?;

        let mut item_nodes = BTreeMap::<NodeId, bool>::new();

        for item in &items {
            if item.node_id == source_node {
                return Err(HdlClockModelError::DeclarationNodeReusedAsItem {
                    node: source_node,
                });
            }

            if item_nodes
                .insert(item.node_id, item.is_named_block())
                .is_some()
            {
                return Err(HdlClockModelError::DuplicateAstNode {
                    node: item.node_id,
                });
            }
        }

        for item in &items {
            if let Some(parent) = item.parent_block {
                match item_nodes.get(&parent) {
                    Some(true) => {}
                    Some(false) | None => {
                        return Err(HdlClockModelError::InvalidParentBlock {
                            item: item.node_id,
                            parent,
                        });
                    }
                }

                if parent == item.node_id {
                    return Err(HdlClockModelError::InvalidParentBlock {
                        item: item.node_id,
                        parent,
                    });
                }
            }
        }

        Ok(Self {
            source_node,
            name,
            type_expression,
            items,
        })
    }

    /// Return the canonical AST node identifying this declaration.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Return the logical clock name.
    #[must_use]
    pub const fn name(&self) -> &HdlId {
        &self.name
    }

    /// Return the canonical AST node for the optional declared type expression.
    #[must_use]
    pub const fn type_expression(&self) -> Option<NodeId> {
        self.type_expression
    }

    /// Return all declaration items in source order.
    #[must_use]
    pub fn items(&self) -> &[HdlClockItem] {
        &self.items
    }

    /// Iterate over property assignments without allocating a new collection.
    pub fn properties(
        &self,
    ) -> impl Iterator<Item = (&HdlClockItem, &HdlClockPropertyName, NodeId)> {
        self.items.iter().filter_map(|item| {
            if let HdlClockItemKind::Property {
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

    /// Iterate over named blocks without allocating a new collection.
    pub fn named_blocks(
        &self,
    ) -> impl Iterator<Item = (&HdlClockItem, &HdlId)> {
        self.items.iter().filter_map(|item| {
            if let HdlClockItemKind::NamedBlock { name } = &item.kind {
                Some((item, name))
            } else {
                None
            }
        })
    }

    /// Find a property by its exact source-level name.
    ///
    /// The iterator is used because the grammar's open property syntax does
    /// not make duplicate-property policy a responsibility of this model.
    /// The semantic validator decides whether repeated properties are legal.
    pub fn properties_named<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = (&'a HdlClockItem, NodeId)> + 'a {
        self.properties().filter_map(move |(item, property_name, value)| {
            (property_name.as_str() == name).then_some((item, value))
        })
    }
}

/// Structural errors detected while constructing the logical clock view.
///
/// These errors indicate malformed adapter input or an inconsistent model.
/// They are not substitutes for source-level semantic diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlClockModelError {
    /// A name was empty, malformed, or otherwise unsuitable for its field.
    InvalidName {
        /// Name field being validated.
        field: &'static str,
        /// Stable, concise explanation of the invalid input.
        reason: &'static str,
    },

    /// A clock item reused an AST node identity already present in this clock.
    DuplicateAstNode {
        /// Repeated canonical AST identity.
        node: NodeId,
    },

    /// An item reused the declaration's own AST identity.
    DeclarationNodeReusedAsItem {
        /// Canonical declaration identity.
        node: NodeId,
    },

    /// An item referred to a parent that is not a named block in this clock.
    InvalidParentBlock {
        /// Item whose parent reference is invalid.
        item: NodeId,
        /// Referenced parent identity.
        parent: NodeId,
    },
}

impl fmt::Display for HdlClockModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName { field, reason } => {
                write!(formatter, "invalid {field} name: {reason}")
            }
            Self::DuplicateAstNode { node } => {
                write!(formatter, "duplicate AST node in clock declaration: {node:?}")
            }
            Self::DeclarationNodeReusedAsItem { node } => {
                write!(
                    formatter,
                    "clock declaration AST node is reused as an item: {node:?}"
                )
            }
            Self::InvalidParentBlock { item, parent } => {
                write!(
                    formatter,
                    "clock item {item:?} refers to invalid named-block parent {parent:?}"
                )
            }
        }
    }
}

impl std::error::Error for HdlClockModelError {}

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), HdlClockModelError> {
    if value.trim().is_empty() {
        return Err(HdlClockModelError::InvalidName {
            field,
            reason: "name must not be empty",
        });
    }

    if value.chars().any(char::is_whitespace) {
        return Err(HdlClockModelError::InvalidName {
            field,
            reason: "name must be one identifier",
        });
    }

    if value.chars().any(char::is_control) {
        return Err(HdlClockModelError::InvalidName {
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
        HdlId::new(value).expect("test identifier must be valid")
    }

    fn property_name(value: &str) -> HdlClockPropertyName {
        HdlClockPropertyName::new(value).expect("test property name must be valid")
    }

    #[test]
    fn recognizes_standard_properties_without_rejecting_extensions() {
        assert_eq!(
            property_name("period").standard_kind(),
            Some(StandardClockProperty::Period)
        );
        assert_eq!(
            property_name("frequency").standard_kind(),
            Some(StandardClockProperty::Frequency)
        );
        assert_eq!(property_name("vendor_extension").standard_kind(), None);
    }

    #[test]
    fn preserves_symbolic_expression_references_and_source_order() {
        let clock = HdlClock::new(
            node(1),
            identifier("system"),
            None,
            vec![
                HdlClockItem::property(
                    node(2),
                    None,
                    property_name("frequency"),
                    node(3),
                ),
                HdlClockItem::property(
                    node(4),
                    None,
                    property_name("period"),
                    node(5),
                ),
            ],
        )
        .expect("valid clock view");

        let properties: Vec<_> = clock
            .properties()
            .map(|(_, name, expression)| (name.as_str(), expression))
            .collect();

        assert_eq!(properties, vec![("frequency", node(3)), ("period", node(5))]);
    }

    #[test]
    fn supports_nested_blocks_without_creating_a_second_nested_tree() {
        let clock = HdlClock::new(
            node(1),
            identifier("derived"),
            None,
            vec![
                HdlClockItem::named_block(node(2), None, identifier("generated"))
                    .expect("valid block"),
                HdlClockItem::property(
                    node(3),
                    Some(node(2)),
                    property_name("source"),
                    node(4),
                ),
            ],
        )
        .expect("valid clock view");

        assert_eq!(clock.items().len(), 2);
        assert_eq!(clock.items()[1].parent_block(), Some(node(2)));
        assert_eq!(clock.named_blocks().count(), 1);
    }

    #[test]
    fn rejects_duplicate_item_node_ids() {
        let result = HdlClock::new(
            node(1),
            identifier("clk"),
            None,
            vec![
                HdlClockItem::attribute(node(2), None),
                HdlClockItem::attribute(node(2), None),
            ],
        );

        assert!(matches!(
            result,
            Err(HdlClockModelError::DuplicateAstNode { node: repeated })
                if repeated == node(2)
        ));
    }

    #[test]
    fn rejects_a_parent_that_is_not_a_named_block() {
        let result = HdlClock::new(
            node(1),
            identifier("clk"),
            None,
            vec![
                HdlClockItem::attribute(node(2), None),
                HdlClockItem::attribute(node(3), Some(node(2))),
            ],
        );

        assert!(matches!(
            result,
            Err(HdlClockModelError::InvalidParentBlock {
                item,
                parent
            }) if item == node(3) && parent == node(2)
        ));
    }

    #[test]
    fn property_lookup_preserves_repeated_properties_for_semantic_validation() {
        let clock = HdlClock::new(
            node(1),
            identifier("clk"),
            None,
            vec![
                HdlClockItem::property(
                    node(2),
                    None,
                    property_name("period"),
                    node(3),
                ),
                HdlClockItem::property(
                    node(4),
                    None,
                    property_name("period"),
                    node(5),
                ),
            ],
        )
        .expect("model preserves source declarations");

        assert_eq!(clock.properties_named("period").count(), 2);
    }

    #[test]
    fn rejects_invalid_property_names() {
        assert!(HdlClockPropertyName::new("").is_err());
        assert!(HdlClockPropertyName::new("clock period").is_err());
        assert!(HdlClockPropertyName::new("period\n").is_err());
    }
}
