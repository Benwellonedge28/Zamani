
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! Target-independent HDL reset declaration models.
//!
//! This module represents reset intent without selecting physical devices,
//! generating reset networks, scheduling hardware, or defining HDLIR.
//!
//! The canonical frontend owns source syntax, AST construction, and source
//! spans. The canonical semantic validator owns semantic correctness,
//! reference resolution, and cross-declaration consistency. This module
//! provides a loss-preserving model for reset declarations and their
//! source-level properties.
//!
//! Reset property names remain open-ended because the grammar permits
//! identifier-based properties. Unknown properties must be preserved rather
//! than rejected merely because this module does not recognize them.
//!
//! All HDLIR implementation belongs under `src/hdl/hdlir/`.

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::NodeId;
use crate::frontend::semantic::domains::hdl::HdlId;

/// A source-level HDL reset property name.
///
/// Names are not represented by a closed enum: the HDL grammar permits
/// property identifiers that may be introduced by future language versions.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HdlResetPropertyName(String);

impl HdlResetPropertyName {
    /// Creates a property name after validating its identifier spelling.
    ///
    /// This validation checks the lexical shape of an identifier. It does
    /// not determine whether the property is semantically supported.
    pub fn new(value: impl Into<String>) -> Result<Self, HdlResetModelError> {
        let value = value.into();

        if !is_identifier(&value) {
            return Err(HdlResetModelError::InvalidIdentifier {
                role: "reset property",
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the original property spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Classifies the property when its spelling is known.
    ///
    /// An unrecognized property returns `None`; its original spelling is
    /// retained by this value.
    pub fn standard_kind(&self) -> Option<StandardResetProperty> {
        StandardResetProperty::from_name(self.as_str())
    }
}

impl fmt::Display for HdlResetPropertyName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A recognized reset property spelling.
///
/// This enum is only a classification aid, not the complete set of legal
/// property names. Unknown property names remain valid model values.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StandardResetProperty {
    /// The reset is asserted when its signal is logically high.
    ActiveHigh,

    /// The reset is asserted when its signal is logically low.
    ActiveLow,

    /// Reset assertion or behavior is synchronous.
    Synchronous,

    /// Reset assertion or behavior is asynchronous.
    Asynchronous,

    /// The clock associated with the reset.
    Clock,

    /// A synchronizer associated with the reset.
    Synchronizer,

    /// Requests synchronous deassertion.
    DeassertSynchronously,

    /// A parent reset relationship.
    Parent,

    /// A dependency relationship.
    DependsOn,

    /// A reset domain relationship.
    Domain,

    /// A synchronizer stage count or stage-related property.
    Stages,

    /// A reset value property.
    Value,
}

impl StandardResetProperty {
    /// Returns the standard property associated with a spelling.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "active_high" => Some(Self::ActiveHigh),
            "active_low" => Some(Self::ActiveLow),
            "synchronous" => Some(Self::Synchronous),
            "asynchronous" => Some(Self::Asynchronous),
            "clock" => Some(Self::Clock),
            "synchronizer" => Some(Self::Synchronizer),
            "deassert_synchronously" => Some(Self::DeassertSynchronously),
            "parent" => Some(Self::Parent),
            "depends_on" => Some(Self::DependsOn),
            "domain" => Some(Self::Domain),
            "stages" => Some(Self::Stages),
            "value" => Some(Self::Value),
            _ => None,
        }
    }
}

/// The reset assertion behavior expressed by a recognized property.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResetAssertionMode {
    /// Reset behavior is synchronized to a clock.
    Synchronous,

    /// Reset assertion does not require clock synchronization.
    Asynchronous,
}

/// The active polarity expressed by a recognized property.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResetPolarity {
    /// The reset is asserted by a logical high.
    ActiveHigh,

    /// The reset is asserted by a logical low.
    ActiveLow,
}

/// One property attached to an HDL reset declaration.
///
/// `value_expression` refers to a canonical frontend AST node when the
/// grammar supplies an expression. A missing expression is preserved as
/// `None`; interpretation and validation belong to the semantic layer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlResetProperty {
    node_id: NodeId,
    name: HdlResetPropertyName,
    value_expression: Option<NodeId>,
}

impl HdlResetProperty {
    /// Creates a reset property without a value expression.
    pub fn flag(
        node_id: NodeId,
        name: HdlResetPropertyName,
    ) -> Self {
        Self {
            node_id,
            name,
            value_expression: None,
        }
    }

    /// Creates a reset property with a canonical AST value expression.
    pub fn expression(
        node_id: NodeId,
        name: HdlResetPropertyName,
        value_expression: NodeId,
    ) -> Self {
        Self {
            node_id,
            name,
            value_expression: Some(value_expression),
        }
    }

    /// Returns the canonical AST node identifying this property.
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Returns this property's original name.
    pub fn name(&self) -> &HdlResetPropertyName {
        &self.name
    }

    /// Returns the AST node for its value expression, if present.
    pub fn value_expression(&self) -> Option<NodeId> {
        self.value_expression
    }

    /// Returns the recognized property classification, if any.
    pub fn standard_kind(&self) -> Option<StandardResetProperty> {
        self.name.standard_kind()
    }
}

/// A source-level HDL reset declaration.
///
/// The property vector preserves source ordering and duplicate property
/// names. Whether duplicates conflict, override one another, or are invalid
/// is a semantic question and must be decided by the canonical validator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlResetDeclaration {
    source_node: NodeId,
    name: HdlId,
    type_expression: Option<NodeId>,
    properties: Vec<HdlResetProperty>,
}

impl HdlResetDeclaration {
    /// Creates a declaration and checks its local structural invariants.
    ///
    /// This constructor does not resolve references, check clock-domain
    /// compatibility, determine physical implementation, or replace the
    /// canonical semantic validator.
    pub fn new(
        source_node: NodeId,
        name: HdlId,
        type_expression: Option<NodeId>,
        properties: Vec<HdlResetProperty>,
    ) -> Result<Self, HdlResetModelError> {
        if !is_identifier(name.as_str()) {
            return Err(HdlResetModelError::InvalidIdentifier {
                role: "reset declaration",
                value: name.as_str().to_owned(),
            });
        }

        let mut seen_nodes = HashSet::new();

        if !seen_nodes.insert(source_node) {
            return Err(HdlResetModelError::DuplicateAstNode {
                node_id: source_node,
            });
        }

        if let Some(node_id) = type_expression {
            if !seen_nodes.insert(node_id) {
                return Err(HdlResetModelError::DuplicateAstNode { node_id });
            }
        }

        for property in &properties {
            if !seen_nodes.insert(property.node_id) {
                return Err(HdlResetModelError::DuplicateAstNode {
                    node_id: property.node_id,
                });
            }

            if let Some(node_id) = property.value_expression {
                if !seen_nodes.insert(node_id) {
                    return Err(HdlResetModelError::DuplicateAstNode { node_id });
                }
            }
        }

        Ok(Self {
            source_node,
            name,
            type_expression,
            properties,
        })
    }

    /// Returns the declaration's canonical source AST node.
    pub fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Returns the shared semantic identifier for this declaration.
    pub fn name(&self) -> &HdlId {
        &self.name
    }

    /// Returns the optional type expression's AST node.
    pub fn type_expression(&self) -> Option<NodeId> {
        self.type_expression
    }

    /// Returns all properties in their original order, including duplicates.
    pub fn properties(&self) -> &[HdlResetProperty] {
        &self.properties
    }

    /// Iterates over properties with the requested exact spelling.
    pub fn properties_named<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a HdlResetProperty> + 'a {
        self.properties
            .iter()
            .filter(move |property| property.name.as_str() == name)
    }

    /// Iterates over properties of a recognized standard kind.
    pub fn properties_of_kind(
        &self,
        kind: StandardResetProperty,
    ) -> impl Iterator<Item = &HdlResetProperty> {
        self.properties
            .iter()
            .filter(move |property| property.standard_kind() == Some(kind))
    }

    /// Iterates over all value-expression references for a named property.
    pub fn property_values<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = NodeId> + 'a {
        self.properties_named(name)
            .filter_map(HdlResetProperty::value_expression)
    }

    /// Returns recognized synchronous/asynchronous declarations.
    ///
    /// This preserves all matching properties rather than silently choosing
    /// one when multiple properties are present.
    pub fn assertion_modes(&self) -> impl Iterator<Item = ResetAssertionMode> + '_ {
        self.properties.iter().filter_map(|property| {
            match property.standard_kind() {
                Some(StandardResetProperty::Synchronous) => {
                    Some(ResetAssertionMode::Synchronous)
                }
                Some(StandardResetProperty::Asynchronous) => {
                    Some(ResetAssertionMode::Asynchronous)
                }
                _ => None,
            }
        })
    }

    /// Returns recognized active-high/active-low declarations.
    pub fn polarities(&self) -> impl Iterator<Item = ResetPolarity> + '_ {
        self.properties.iter().filter_map(|property| {
            match property.standard_kind() {
                Some(StandardResetProperty::ActiveHigh) => {
                    Some(ResetPolarity::ActiveHigh)
                }
                Some(StandardResetProperty::ActiveLow) => {
                    Some(ResetPolarity::ActiveLow)
                }
                _ => None,
            }
        })
    }

    /// Returns AST references from `clock` properties.
    pub fn clock_references(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.property_values("clock")
    }

    /// Returns AST references from `parent` properties.
    pub fn parent_references(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.property_values("parent")
    }

    /// Returns AST references from `depends_on` properties.
    pub fn dependency_references(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.property_values("depends_on")
    }

    /// Returns AST references from `domain` properties.
    pub fn domain_references(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.property_values("domain")
    }
}

/// Structural errors detected by the reset declaration model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlResetModelError {
    /// A declaration or property name is not a valid identifier.
    InvalidIdentifier {
        /// The kind of name that failed validation.
        role: &'static str,

        /// The original spelling supplied by the caller.
        value: String,
    },

    /// An AST node identifier was reused within one declaration model.
    DuplicateAstNode {
        /// The duplicated canonical AST node identifier.
        node_id: NodeId,
    },
}

impl fmt::Display for HdlResetModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { role, value } => {
                write!(formatter, "invalid {role} identifier: {value:?}")
            }
            Self::DuplicateAstNode { node_id } => {
                write!(formatter, "duplicate AST node identifier: {node_id:?}")
            }
        }
    }
}

impl Error for HdlResetModelError {}

/// Checks the basic ASCII identifier form used by the current HDL grammar.
///
/// This is intentionally a lexical check, not a keyword or semantic check.
/// If the authoritative grammar's identifier rule changes, this helper
/// must be updated to match it rather than becoming an independent authority.
fn is_identifier(value: &str) -> bool {
    let mut chars = value.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }

    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Adapt this helper to the actual NodeId constructor if the repository's
    // canonical NodeId API differs.
    fn node(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn property_name(value: &str) -> HdlResetPropertyName {
        HdlResetPropertyName::new(value).expect("valid property identifier")
    }

    fn reset_id(value: &str) -> HdlId {
        HdlId::new(value).expect("valid reset identifier")
    }

    #[test]
    fn preserves_unknown_future_property_names() {
        let property = property_name("vendor_extension");

        assert_eq!(property.as_str(), "vendor_extension");
        assert_eq!(property.standard_kind(), None);
    }

    #[test]
    fn preserves_property_order_and_duplicates() {
        let declaration = HdlResetDeclaration::new(
            node(1),
            reset_id("system_reset"),
            None,
            vec![
                HdlResetProperty::flag(node(2), property_name("active_low")),
                HdlResetProperty::flag(node(3), property_name("active_low")),
                HdlResetProperty::flag(node(4), property_name("vendor_extension")),
            ],
        )
        .expect("distinct AST nodes should be accepted");

        let names: Vec<_> = declaration
            .properties()
            .iter()
            .map(|property| property.name().as_str())
            .collect();

        assert_eq!(
            names,
            vec!["active_low", "active_low", "vendor_extension"]
        );
    }

    #[test]
    fn classifies_known_reset_modes_and_polarities() {
        let declaration = HdlResetDeclaration::new(
            node(1),
            reset_id("system_reset"),
            None,
            vec![
                HdlResetProperty::flag(node(2), property_name("asynchronous")),
                HdlResetProperty::flag(node(3), property_name("active_low")),
            ],
        )
        .expect("distinct AST nodes should be accepted");

        assert_eq!(
            declaration.assertion_modes().collect::<Vec<_>>(),
            vec![ResetAssertionMode::Asynchronous]
        );
        assert_eq!(
            declaration.polarities().collect::<Vec<_>>(),
            vec![ResetPolarity::ActiveLow]
        );
    }

    #[test]
    fn extracts_relationship_expression_references() {
        let declaration = HdlResetDeclaration::new(
            node(1),
            reset_id("system_reset"),
            None,
            vec![
                HdlResetProperty::expression(
                    node(2),
                    property_name("clock"),
                    node(20),
                ),
                HdlResetProperty::expression(
                    node(3),
                    property_name("parent"),
                    node(21),
                ),
                HdlResetProperty::expression(
                    node(4),
                    property_name("depends_on"),
                    node(22),
                ),
                HdlResetProperty::expression(
                    node(5),
                    property_name("domain"),
                    node(23),
                ),
            ],
        )
        .expect("distinct AST nodes should be accepted");

        assert_eq!(declaration.clock_references().collect::<Vec<_>>(), vec![node(20)]);
        assert_eq!(declaration.parent_references().collect::<Vec<_>>(), vec![node(21)]);
        assert_eq!(
            declaration.dependency_references().collect::<Vec<_>>(),
            vec![node(22)]
        );
        assert_eq!(declaration.domain_references().collect::<Vec<_>>(), vec![node(23)]);
    }

    #[test]
    fn rejects_malformed_names_without_normalizing_them() {
        assert!(HdlResetPropertyName::new("").is_err());
        assert!(HdlResetPropertyName::new("reset mode").is_err());
        assert!(HdlResetPropertyName::new("reset\n").is_err());

        // The shared identifier type may accept broader non-empty names.
        // The reset declaration model must still reject a non-identifier.
        let name = HdlId::new("reset name")
            .expect("the shared ID type permits non-empty names");

        assert!(HdlResetDeclaration::new(
            node(1),
            name,
            None,
            Vec::new(),
        )
        .is_err());
    }

    #[test]
    fn rejects_reused_ast_node_identifiers() {
        let result = HdlResetDeclaration::new(
            node(1),
            reset_id("system_reset"),
            None,
            vec![
                HdlResetProperty::flag(node(2), property_name("active_high")),
                HdlResetProperty::flag(node(2), property_name("synchronous")),
            ],
        );

        assert!(matches!(
            result,
            Err(HdlResetModelError::DuplicateAstNode { .. })
        ));
    }
}
