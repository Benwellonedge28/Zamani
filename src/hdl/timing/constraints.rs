
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL timing-constraint model
//!
//! Path: `src/hdl/timing/constraints.rs`
//! Rust edition: 2021; minimum supported Rust version: 1.97.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/timing.g4`
//! - `grammar/hdl/clocks.g4`
//! - `grammar/hdl/clocking.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/timing/clock.rs`
//! - `src/hdl/timing/reset.rs`
//!
//! The grammar owns timing syntax. The canonical AST owns source nodes and
//! expressions. The semantic pipeline owns name resolution, expression
//! evaluation, dimensional analysis, unit conversion, and semantic errors.
//!
//! ## Owns
//!
//! - A loss-conscious, source-ordered view of timing declarations and items.
//! - Explicit distinctions between ordinary properties, comparisons,
//!   requirements, preferences, hints, assertions, relations, and scopes.
//! - References to canonical AST expression nodes.
//! - Structural checks for item identities and scope relationships.
//! - Open-ended timing property and scope names.
//!
//! ## Does not own
//!
//! - A parser, replacement AST, expression evaluator, or source-map type.
//! - Physical static timing analysis, timing closure, scheduling, placement,
//!   routing, synthesis, device discovery, or hardware allocation.
//! - A second requirements/capabilities/resource model.
//! - HDLIR structures, builders, passes, verification, or lowering.
//!   All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Semantics and units
//!
//! Expression values remain canonical AST references. A unit-bearing value,
//! such as `10ns`, must be preserved by the referenced expression. This
//! module does not parse units from source text, infer dimensions, convert
//! units, or decide whether two quantities are comparable.
//!
//! A requirement, preference, and hint are different kinds of intent.
//! A preference or hint must not be treated as a hard requirement. An
//! ordinary property is not automatically classified as either one.
//!
//! ## Scalability
//!
//! There are no language-level maximums for declarations, timing items,
//! scopes, paths, or constraints. Collections grow with available memory.
//! Callers may impose configurable operational budgets; an incomplete or
//! budget-exhausted analysis must not be reported as complete.
//!
//! ## Integration
//!
//! The AST adapter maps timing declaration and item nodes to these types,
//! retaining source order, canonical `NodeId`s, expression references,
//! qualified references, and parent-scope relationships. The semantic
//! validator resolves references and validates units and contradictions.
//! `clock.rs` remains the owner of clock declarations; this module may refer
//! to clocks by source-level reference without redefining them.
//!
//! ## Freeze criteria
//!
//! Freeze this API after the AST adapter is loss-preserving and grammar
//! conformance tests verify all supported timing item kinds. Any grammar
//! change that adds an item kind must add an explicit model representation
//! or a documented extension representation before the adapter is accepted.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use crate::frontend::ast::node::NodeId;

/// Version of the public in-memory timing-constraint model.
///
/// This is not the Zamani language, grammar, or HDLIR version.
pub const HDL_TIMING_CONSTRAINT_MODEL_VERSION: u16 = 1;

/// A source-level timing name.
///
/// The value is preserved exactly. It may be a property name, timing
/// declaration name, scope name, or other identifier accepted by the
/// canonical parser. This type deliberately does not define a closed
/// vocabulary of timing properties.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimingName(String);

impl TimingName {
    /// Construct a name without silently normalizing its spelling.
    ///
    /// This performs basic structural validation, not canonical lexer or
    /// language-level name validation.
    pub fn new(value: impl Into<String>) -> Result<Self, TimingConstraintError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(TimingConstraintError::InvalidName {
                value,
                reason: "name must not be empty",
            });
        }

        if value.chars().any(char::is_whitespace) {
            return Err(TimingConstraintError::InvalidName {
                value,
                reason: "name must not contain whitespace",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(TimingConstraintError::InvalidName {
                value,
                reason: "name must not contain control characters",
            });
        }

        Ok(Self(value))
    }

    /// Return the original spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TimingName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// The source-level strength of a timing item.
///
/// This classification records author intent. It does not establish
/// whether the constraint is satisfiable or whether a target can implement it.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimingIntent {
    /// An explicit requirement that must be satisfied for a successful
    /// realization that claims conformance to this requirement.
    Requirement,

    /// A preferred outcome that may be relaxed according to the applicable
    /// optimization policy.
    Preference,

    /// A non-binding implementation hint.
    Hint,

    /// An ordinary timing property or relational constraint whose strength
    /// has not been assigned by this model.
    Unclassified,
}

/// A relational operator appearing in a timing constraint.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimingComparison {
    /// Strictly less than.
    LessThan,
    /// Less than or equal to.
    LessThanOrEqual,
    /// Equal to.
    Equal,
    /// Not equal to.
    NotEqual,
    /// Greater than or equal to.
    GreaterThanOrEqual,
    /// Strictly greater than.
    GreaterThan,
}

/// The structural kind of a timing item.
///
/// This is not an evaluation result. For example, `Assertion` records that
/// the source contains an assertion; it does not mean the assertion passed.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimingItemKind {
    /// An ordinary named property, such as `period = expression`.
    Property,

    /// A named comparison, such as `latency <= expression`.
    Comparison(TimingComparison),

    /// A requirement expression.
    Requirement,

    /// A preference expression.
    Preference,

    /// A hint expression.
    Hint,

    /// A timing assertion expression.
    Assertion,

    /// A relationship between two source-level endpoints.
    Relation {
        /// Source endpoint spelling, preserved as supplied by the adapter.
        from: TimingName,
        /// Destination endpoint spelling, preserved as supplied by the adapter.
        to: TimingName,
    },

    /// A named timing window.
    Window,

    /// A named timing path.
    Path,

    /// A timing exception, which may be anonymous in the grammar.
    Exception,

    /// A named timing budget.
    Budget,

    /// A timing-analysis block.
    Analysis,

    /// A named nested timing block.
    NamedBlock,

    /// An attribute attached to timing intent.
    Attribute,

    /// An extension-defined item kind not yet represented by a dedicated
    /// variant. The original extension name is preserved.
    Extension(TimingName),
}

impl TimingItemKind {
    /// Return the intent strength implied by this structural kind.
    ///
    /// Property and comparison items remain unclassified here: their meaning
    /// must not be guessed from the property spelling alone.
    #[must_use]
    pub const fn intent(&self) -> TimingIntent {
        match self {
            Self::Requirement => TimingIntent::Requirement,
            Self::Preference => TimingIntent::Preference,
            Self::Hint => TimingIntent::Hint,
            _ => TimingIntent::Unclassified,
        }
    }

    /// Return whether this item can contain nested timing items.
    #[must_use]
    pub const fn is_scope(&self) -> bool {
        matches!(
            self,
            Self::Relation { .. }
                | Self::Window
                | Self::Path
                | Self::Exception
                | Self::Budget
                | Self::Analysis
                | Self::NamedBlock
        )
    }
}

/// A single timing item referring to canonical AST nodes.
///
/// Items are stored in source order. `parent_scope` references the AST node
/// of a containing item when the grammar nests this item in a block.
/// Expression nodes are references, not copied or re-parsed expressions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimingConstraintItem {
    node_id: NodeId,
    parent_scope: Option<NodeId>,
    name: Option<TimingName>,
    expression: Option<NodeId>,
    kind: TimingItemKind,
}

impl TimingConstraintItem {
    fn new(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        name: Option<TimingName>,
        expression: Option<NodeId>,
        kind: TimingItemKind,
    ) -> Self {
        Self {
            node_id,
            parent_scope,
            name,
            expression,
            kind,
        }
    }

    /// Construct an ordinary timing property assignment.
    pub fn property(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        name: TimingName,
        expression: NodeId,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            Some(name),
            Some(expression),
            TimingItemKind::Property,
        )
    }

    /// Construct a named comparison constraint.
    pub fn comparison(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        name: TimingName,
        operator: TimingComparison,
        expression: NodeId,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            Some(name),
            Some(expression),
            TimingItemKind::Comparison(operator),
        )
    }

    /// Construct a `require` timing expression.
    pub fn requirement(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        expression: NodeId,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            None,
            Some(expression),
            TimingItemKind::Requirement,
        )
    }

    /// Construct a `prefer` timing expression.
    pub fn preference(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        expression: NodeId,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            None,
            Some(expression),
            TimingItemKind::Preference,
        )
    }

    /// Construct a `hint` timing expression.
    pub fn hint(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        expression: NodeId,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            None,
            Some(expression),
            TimingItemKind::Hint,
        )
    }

    /// Construct a timing assertion expression.
    pub fn assertion(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        expression: NodeId,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            None,
            Some(expression),
            TimingItemKind::Assertion,
        )
    }

    /// Construct a relation between two source-level endpoints.
    ///
    /// The relation's body, if any, is represented by additional items whose
    /// `parent_scope` refers to this relation's `node_id`.
    pub fn relation(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        from: TimingName,
        to: TimingName,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            None,
            None,
            TimingItemKind::Relation { from, to },
        )
    }

    /// Construct a named timing scope, such as a window, path, budget, or
    /// nested named block.
    pub fn named_scope(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        name: TimingName,
        kind: TimingScopeKind,
    ) -> Self {
        let kind = match kind {
            TimingScopeKind::Window => TimingItemKind::Window,
            TimingScopeKind::Path => TimingItemKind::Path,
            TimingScopeKind::Budget => TimingItemKind::Budget,
            TimingScopeKind::NamedBlock => TimingItemKind::NamedBlock,
        };

        Self::new(node_id, parent_scope, Some(name), None, kind)
    }

    /// Construct a timing exception scope, optionally named.
    pub fn exception(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        name: Option<TimingName>,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            name,
            None,
            TimingItemKind::Exception,
        )
    }

    /// Construct a timing-analysis scope.
    pub fn analysis(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            None,
            None,
            TimingItemKind::Analysis,
        )
    }

    /// Construct an attribute reference.
    ///
    /// The attribute node remains owned by the canonical AST.
    pub fn attribute(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            None,
            None,
            TimingItemKind::Attribute,
        )
    }

    /// Construct an extension-defined timing item.
    ///
    /// Use this only when the active grammar/extension defines a timing item
    /// that does not yet have a dedicated variant. The adapter must preserve
    /// the original expression reference, if the extension has one.
    pub fn extension(
        node_id: NodeId,
        parent_scope: Option<NodeId>,
        name: TimingName,
        expression: Option<NodeId>,
    ) -> Self {
        Self::new(
            node_id,
            parent_scope,
            Some(name.clone()),
            expression,
            TimingItemKind::Extension(name),
        )
    }

    /// Return the canonical AST node identifying this item.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Return the parent scope's AST node, if nested.
    #[must_use]
    pub const fn parent_scope(&self) -> Option<NodeId> {
        self.parent_scope
    }

    /// Return the item's optional name.
    #[must_use]
    pub fn name(&self) -> Option<&TimingName> {
        self.name.as_ref()
    }

    /// Return the canonical AST expression node, if this item has one.
    #[must_use]
    pub const fn expression(&self) -> Option<NodeId> {
        self.expression
    }

    /// Return the structural kind.
    #[must_use]
    pub const fn kind(&self) -> &TimingItemKind {
        &self.kind
    }

    /// Return the classified intent strength.
    #[must_use]
    pub const fn intent(&self) -> TimingIntent {
        self.kind.intent()
    }

    /// Return whether this item may contain nested timing items.
    #[must_use]
    pub const fn is_scope(&self) -> bool {
        self.kind.is_scope()
    }
}

/// Kinds accepted by [`TimingConstraintItem::named_scope`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimingScopeKind {
    /// A named timing window.
    Window,
    /// A named timing path.
    Path,
    /// A named timing budget.
    Budget,
    /// A nested, named timing block.
    NamedBlock,
}

/// A complete source-level timing declaration.
///
/// `target` is a logical source-level target reference, not a physical device
/// or target-selection decision. The canonical semantic layer resolves it.
/// Items remain in source order; duplicate names and repeated properties are
/// preserved for semantic validation instead of silently overwritten.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimingConstraintSet {
    source_node: NodeId,
    name: TimingName,
    target: Option<TimingName>,
    items: Vec<TimingConstraintItem>,
}

impl TimingConstraintSet {
    /// Construct a timing declaration and validate local structural
    /// invariants.
    ///
    /// This constructor does not resolve names, validate units, prove
    /// satisfiability, or decide target feasibility.
    pub fn new(
        source_node: NodeId,
        name: TimingName,
        target: Option<TimingName>,
        items: Vec<TimingConstraintItem>,
    ) -> Result<Self, TimingConstraintError> {
        let mut item_nodes = BTreeSet::new();
        item_nodes.insert(source_node);

        let mut by_node = BTreeMap::new();
        for item in &items {
            if !item_nodes.insert(item.node_id) {
                return Err(TimingConstraintError::DuplicateAstNode {
                    node: item.node_id,
                });
            }

            by_node.insert(item.node_id, item);
        }

        // A parent must exist in this declaration and must be a scope.
        // This prevents dangling or non-container scope references.
        for item in &items {
            if let Some(parent_id) = item.parent_scope {
                let Some(parent) = by_node.get(&parent_id) else {
                    return Err(TimingConstraintError::UnknownParentScope {
                        item: item.node_id,
                        parent: parent_id,
                    });
                };

                if !parent.is_scope() {
                    return Err(TimingConstraintError::ParentIsNotScope {
                        item: item.node_id,
                        parent: parent_id,
                    });
                }
            }
        }

        // Detect parent cycles without recursive traversal. This is an
        // integrity check over a finite declaration, not a language depth cap.
        for item in &items {
            let mut seen = BTreeSet::new();
            let mut current = Some(item.node_id);

            while let Some(node_id) = current {
                if !seen.insert(node_id) {
                    return Err(TimingConstraintError::ScopeCycle { node: node_id });
                }

                current = by_node
                    .get(&node_id)
                    .and_then(|current_item| current_item.parent_scope);
            }
        }

        // A referenced expression is a distinct canonical AST node. Reject
        // collisions with declaration/item IDs and with other expression
        // references in this flattened model rather than silently aliasing
        // structurally different nodes.
        for item in &items {
            if let Some(expression) = item.expression {
                if !item_nodes.insert(expression) {
                    return Err(TimingConstraintError::DuplicateAstNode {
                        node: expression,
                    });
                }
            }
        }

        Ok(Self {
            source_node,
            name,
            target,
            items,
        })
    }

    /// Return the canonical AST node identifying this declaration.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Return the timing declaration's name.
    #[must_use]
    pub fn name(&self) -> &TimingName {
        &self.name
    }

    /// Return its optional logical target reference.
    #[must_use]
    pub fn target(&self) -> Option<&TimingName> {
        self.target.as_ref()
    }

    /// Return all items in original source order.
    #[must_use]
    pub fn items(&self) -> &[TimingConstraintItem] {
        &self.items
    }

    /// Iterate over all items with the specified name.
    ///
    /// Repeated names are deliberately retained.
    pub fn items_named<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a TimingConstraintItem> + 'a {
        self.items
            .iter()
            .filter(move |item| item.name().is_some_and(|n| n.as_str() == name))
    }

    /// Iterate over items of the requested structural kind.
    pub fn items_of_kind(
        &self,
        kind: &TimingItemKind,
    ) -> impl Iterator<Item = &TimingConstraintItem> {
        self.items.iter().filter(move |item| item.kind() == kind)
    }

    /// Iterate over items directly contained in a particular scope.
    ///
    /// Pass `None` to obtain top-level items.
    pub fn children_of(
        &self,
        parent: Option<NodeId>,
    ) -> impl Iterator<Item = &TimingConstraintItem> {
        self.items
            .iter()
            .filter(move |item| item.parent_scope() == parent)
    }

    /// Iterate over hard requirement items.
    pub fn requirements(&self) -> impl Iterator<Item = &TimingConstraintItem> {
        self.items
            .iter()
            .filter(|item| item.intent() == TimingIntent::Requirement)
    }

    /// Iterate over preference items.
    pub fn preferences(&self) -> impl Iterator<Item = &TimingConstraintItem> {
        self.items
            .iter()
            .filter(|item| item.intent() == TimingIntent::Preference)
    }

    /// Iterate over hint items.
    pub fn hints(&self) -> impl Iterator<Item = &TimingConstraintItem> {
        self.items
            .iter()
            .filter(|item| item.intent() == TimingIntent::Hint)
    }
}

/// Structural construction failures for timing constraints.
///
/// These are model-construction errors, not user-facing semantic diagnostics.
/// The frontend adapter should translate them to the repository's canonical
/// diagnostic contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimingConstraintError {
    /// A supplied name was structurally invalid.
    InvalidName {
        /// Original rejected spelling.
        value: String,
        /// Stable explanation of the structural problem.
        reason: &'static str,
    },

    /// A canonical AST node identity was repeated in the flattened model.
    DuplicateAstNode {
        /// Repeated node identity.
        node: NodeId,
    },

    /// An item's parent scope does not exist in the declaration.
    UnknownParentScope {
        /// Child item.
        item: NodeId,
        /// Missing parent item.
        parent: NodeId,
    },

    /// An item's parent exists but cannot contain child items.
    ParentIsNotScope {
        /// Child item.
        item: NodeId,
        /// Parent item that is not a scope.
        parent: NodeId,
    },

    /// The parent-scope relation contains a cycle.
    ScopeCycle {
        /// Node where the cycle was detected.
        node: NodeId,
    },
}

impl fmt::Display for TimingConstraintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName { value, reason } => {
                write!(formatter, "invalid timing name `{value}`: {reason}")
            }
            Self::DuplicateAstNode { node } => {
                write!(formatter, "duplicate AST node identity {node}")
            }
            Self::UnknownParentScope { item, parent } => write!(
                formatter,
                "timing item {item} refers to missing parent scope {parent}"
            ),
            Self::ParentIsNotScope { item, parent } => write!(
                formatter,
                "timing item {item} refers to non-scope parent {parent}"
            ),
            Self::ScopeCycle { node } => {
                write!(formatter, "cycle in timing scope relation at node {node}")
            }
        }
    }
}

impl Error for TimingConstraintError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(value: u64) -> NodeId {
        NodeId::new(value).expect("test node IDs must be nonzero")
    }

    fn name(value: &str) -> TimingName {
        TimingName::new(value).expect("test timing name must be valid")
    }

    #[test]
    fn preserves_open_world_property_names() {
        let set = TimingConstraintSet::new(
            node(1),
            name("system_timing"),
            None,
            vec![
                TimingConstraintItem::property(
                    node(2),
                    None,
                    name("future_property"),
                    node(3),
                ),
            ],
        )
        .expect("valid timing declaration");

        assert_eq!(
            set.items_named("future_property").count(),
            1
        );
        assert_eq!(
            set.items()[0].kind(),
            &TimingItemKind::Property
        );
    }

    #[test]
    fn preserves_repeated_properties_in_source_order() {
        let set = TimingConstraintSet::new(
            node(1),
            name("timing"),
            None,
            vec![
                TimingConstraintItem::property(
                    node(2),
                    None,
                    name("period"),
                    node(3),
                ),
                TimingConstraintItem::property(
                    node(4),
                    None,
                    name("period"),
                    node(5),
                ),
            ],
        )
        .expect("repeated properties must remain available to semantic validation");

        assert_eq!(set.items_named("period").count(), 2);
        assert_eq!(set.items()[0].expression(), Some(node(3)));
        assert_eq!(set.items()[1].expression(), Some(node(5)));
    }

    #[test]
    fn distinguishes_requirement_preference_and_hint() {
        let set = TimingConstraintSet::new(
            node(1),
            name("timing"),
            None,
            vec![
                TimingConstraintItem::requirement(node(2), None, node(3)),
                TimingConstraintItem::preference(node(4), None, node(5)),
                TimingConstraintItem::hint(node(6), None, node(7)),
            ],
        )
        .expect("valid timing declaration");

        assert_eq!(set.requirements().count(), 1);
        assert_eq!(set.preferences().count(), 1);
        assert_eq!(set.hints().count(), 1);
    }

    #[test]
    fn retains_comparison_operator_without_evaluating_it() {
        let item = TimingConstraintItem::comparison(
            node(2),
            None,
            name("latency"),
            TimingComparison::LessThanOrEqual,
            node(3),
        );

        assert_eq!(
            item.kind(),
            &TimingItemKind::Comparison(TimingComparison::LessThanOrEqual)
        );
        assert_eq!(item.expression(), Some(node(3)));
    }

    #[test]
    fn supports_nested_scopes_and_queries_children() {
        let set = TimingConstraintSet::new(
            node(1),
            name("timing"),
            None,
            vec![
                TimingConstraintItem::named_scope(
                    node(2),
                    None,
                    name("critical_path"),
                    TimingScopeKind::Path,
                ),
                TimingConstraintItem::property(
                    node(3),
                    Some(node(2)),
                    name("latency"),
                    node(4),
                ),
            ],
        )
        .expect("valid nested timing scope");

        assert_eq!(set.children_of(None).count(), 1);
        assert_eq!(set.children_of(Some(node(2))).count(), 1);
    }

    #[test]
    fn rejects_missing_parent_scope() {
        let result = TimingConstraintSet::new(
            node(1),
            name("timing"),
            None,
            vec![TimingConstraintItem::property(
                node(2),
                Some(node(99)),
                name("latency"),
                node(3),
            )],
        );

        assert!(matches!(
            result,
            Err(TimingConstraintError::UnknownParentScope { .. })
        ));
    }

    #[test]
    fn rejects_parent_that_is_not_a_scope() {
        let result = TimingConstraintSet::new(
            node(1),
            name("timing"),
            None,
            vec![
                TimingConstraintItem::property(
                    node(2),
                    None,
                    name("period"),
                    node(3),
                ),
                TimingConstraintItem::property(
                    node(4),
                    Some(node(2)),
                    name("latency"),
                    node(5),
                ),
            ],
        );

        assert!(matches!(
            result,
            Err(TimingConstraintError::ParentIsNotScope { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_ast_node_identities() {
        let result = TimingConstraintSet::new(
            node(1),
            name("timing"),
            None,
            vec![
                TimingConstraintItem::property(
                    node(2),
                    None,
                    name("period"),
                    node(3),
                ),
                TimingConstraintItem::property(
                    node(2),
                    None,
                    name("latency"),
                    node(4),
                ),
            ],
        );

        assert!(matches!(
            result,
            Err(TimingConstraintError::DuplicateAstNode { .. })
        ));
    }

    #[test]
    fn rejects_invalid_names_without_normalizing_them() {
        assert!(TimingName::new("").is_err());
        assert!(TimingName::new("latency budget").is_err());
        assert!(TimingName::new("period\n").is_err());

        let preserved = TimingName::new("vendor.future_metric")
            .expect("extension names should remain open-world");
        assert_eq!(preserved.as_str(), "vendor.future_metric");
    }
}
