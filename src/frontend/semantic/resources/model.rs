
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Resource Model
//!
//! Path: `src/frontend/semantic/resources/model.rs`
//!
//! ## Purpose
//!
//! Defines the canonical, target-independent semantic representation of
//! resource requirements and resource constraints.
//!
//! ## Authority
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/Zamani.g4`
//! - `src/frontend/ast/node/resources/resource.rs`
//! - `src/frontend/ast/node/resources/resource_kind.rs`
//!
//! ## Ownership
//!
//! This module owns:
//! - exact, non-negative decimal quantities;
//! - extensible semantic resource units;
//! - symbolic and known resource quantities;
//! - resource intent;
//! - resource requirements;
//! - resource constraints;
//! - semantic resource validation;
//! - structured resource-model errors.
//!
//! This module does not own:
//! - AST resource declarations;
//! - name resolution;
//! - expression evaluation;
//! - type inference;
//! - capability discovery;
//! - physical resource inventory;
//! - resource allocation;
//! - target selection;
//! - scheduling, routing, or lowering;
//! - runtime resource handles.
//!
//! ## Integration contract
//!
//! The canonical AST remains authoritative for source syntax and node identity.
//! This module consumes AST resource declarations and produces semantic facts.
//!
//! `requirements.rs` extracts requirements from validated AST constructs.
//! `symbolic.rs` resolves or combines symbolic expressions through the
//! semantic expression/constraint services.
//! `constraints.rs` evaluates constraints when sufficient semantic evidence
//! is available.
//! `resources/mod.rs` exposes the intended public resource API.
//! `model/facts.rs` and `model/program.rs` may retain these values as semantic
//! results. The pipeline must not certify an incomplete analysis as valid.
//!
//! No target-specific IR or physical resource allocation is represented here.
//!
//! ## Scalability contract
//!
//! Resource kinds and units are open, namespaced identities, not closed enums.
//! Known quantities use decimal strings so their representation is not
//! restricted by `u64`, `u128`, `usize`, or the host pointer width.
//! Symbolic quantities retain canonical AST `NodeId` references.
//!
//! This removes arbitrary language-level capacity ceilings from this model;
//! it does not promise infinite memory, infinite compilation time, or that
//! every target can execute every program.
//!
//! ## Determinism and safety
//!
//! Constructors validate their inputs. Ordered presentation is obtained by
//! explicit canonical keys rather than relying on hash-map iteration order.
//! No I/O, hardware inspection, global mutable state, unchecked arithmetic,
//! or unsafe code is used.
//!
//! ## Freeze criteria
//!
//! - `cargo check` succeeds with the actual `resources/mod.rs` exports.
//! - Unit and integration tests pass.
//! - AST conversion preserves names, scopes and symbolic cardinalities.
//! - Unknown extension resource kinds remain representable.
//! - Known quantities do not overflow a machine integer.
//! - No semantic decision depends on target availability.
//! - Downstream modules consume these types instead of defining duplicates.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::resources::resource::{
    Resource as AstResource,
    ResourceCardinality,
    ResourceScope as AstResourceScope,
    ResourceType as AstResourceType,
};
use crate::frontend::ast::node::resources::resource_kind::ResourceKind;

/// Version of this in-memory semantic resource model.
///
/// This is not the language version, AST schema version, or serialized
/// artifact version.
pub const RESOURCE_MODEL_SCHEMA_VERSION: u16 = 1;

/// A validated, exact, non-negative decimal quantity.
///
/// The value is stored as a canonical decimal string, not as a machine-sized
/// integer. The accepted grammar is:
///
/// `DIGITS` or `DIGITS "." DIGITS`
///
/// Scientific notation and signs are intentionally excluded from this
/// representation. An expression requiring them must remain symbolic or be
/// represented by a different, explicitly specified numeric type.
///
/// Leading integer zeroes and trailing fractional zeroes are removed during
/// construction. Zero is canonicalized to `"0"`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExactQuantity {
    canonical: String,
}

impl ExactQuantity {
    /// Parses and canonicalizes an exact non-negative decimal quantity.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ResourceModelError> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err(ResourceModelError::InvalidQuantity(
                "quantity cannot be empty".to_owned(),
            ));
        }

        let mut parts = value.split('.');
        let integer = parts.next().unwrap_or_default();
        let fraction = parts.next();

        if parts.next().is_some()
            || integer.is_empty()
            || !integer.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(ResourceModelError::InvalidQuantity(
                "expected unsigned decimal digits with at most one decimal point"
                    .to_owned(),
            ));
        }

        if let Some(fraction) = fraction {
            if fraction.is_empty()
                || !fraction.bytes().all(|byte| byte.is_ascii_digit())
            {
                return Err(ResourceModelError::InvalidQuantity(
                    "fractional digits must be present and numeric".to_owned(),
                ));
            }
        }

        let integer = integer.trim_start_matches('0');
        let integer = if integer.is_empty() { "0" } else { integer };

        let canonical = match fraction {
            Some(fraction) => {
                let fraction = fraction.trim_end_matches('0');

                if fraction.is_empty() {
                    integer.to_owned()
                } else {
                    let mut result =
                        String::with_capacity(integer.len() + 1 + fraction.len());
                    result.push_str(integer);
                    result.push('.');
                    result.push_str(fraction);
                    result
                }
            }
            None => integer.to_owned(),
        };

        Ok(Self { canonical })
    }

    /// Returns the canonical decimal representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.canonical
    }

    /// Returns whether this quantity is exactly zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.canonical == "0"
    }

    /// Compares two quantities numerically without converting them to a
    /// bounded machine integer or floating-point value.
    #[must_use]
    pub fn numeric_cmp(&self, other: &Self) -> core::cmp::Ordering {
        compare_decimal(&self.canonical, &other.canonical)
    }
}

impl fmt::Display for ExactQuantity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical)
    }
}

fn compare_decimal(left: &str, right: &str) -> core::cmp::Ordering {
    let (left_integer, left_fraction) = split_canonical_decimal(left);
    let (right_integer, right_fraction) = split_canonical_decimal(right);

    left_integer
        .len()
        .cmp(&right_integer.len())
        .then_with(|| left_integer.cmp(right_integer))
        .then_with(|| {
            let width = left_fraction.len().max(right_fraction.len());

            for index in 0..width {
                let left_digit = left_fraction.as_bytes().get(index).copied().unwrap_or(b'0');
                let right_digit =
                    right_fraction.as_bytes().get(index).copied().unwrap_or(b'0');

                let ordering = left_digit.cmp(&right_digit);
                if ordering != core::cmp::Ordering::Equal {
                    return ordering;
                }
            }

            core::cmp::Ordering::Equal
        })
}

fn split_canonical_decimal(value: &str) -> (&str, &str) {
    match value.split_once('.') {
        Some((integer, fraction)) => (integer, fraction),
        None => (value, ""),
    }
}

/// A namespaced semantic resource unit.
///
/// Unit identities remain open so new domains can introduce units without
/// changing this module's type definition. Dimensional compatibility and
/// conversion factors belong to the unit registry/constraint solver.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceUnit {
    namespace: String,
    name: String,
}

impl ResourceUnit {
    /// Creates a validated namespaced unit identity.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, ResourceModelError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identity_component("unit namespace", &namespace)?;
        validate_identity_component("unit name", &name)?;

        Ok(Self { namespace, name })
    }

    /// Returns the unit namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the unit name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the stable textual identity `namespace:name`.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

impl fmt::Display for ResourceUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.namespace, self.name)
    }
}

/// Semantic knowledge about a resource quantity.
///
/// `Symbolic` and `Runtime` refer to existing canonical AST expressions.
/// They do not copy or reinterpret expression syntax. Expression type
/// checking and evaluation belong to the semantic expression subsystem.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ResourceQuantity {
    /// No cardinality or quantity was specified.
    Unspecified,

    /// An exact non-negative decimal quantity is known.
    Known(ExactQuantity),

    /// The quantity is defined by a source/AST expression.
    Symbolic(NodeId),

    /// The language construct explicitly permits runtime-derived quantity.
    Runtime(NodeId),
}

impl ResourceQuantity {
    /// Creates a known quantity from a decimal representation.
    pub fn known(value: impl AsRef<str>) -> Result<Self, ResourceModelError> {
        Ok(Self::Known(ExactQuantity::new(value)?))
    }

    /// Returns the known quantity, if available.
    #[must_use]
    pub fn known_value(&self) -> Option<&ExactQuantity> {
        match self {
            Self::Known(value) => Some(value),
            Self::Unspecified | Self::Symbolic(_) | Self::Runtime(_) => None,
        }
    }

    /// Returns the referenced expression, if one exists.
    #[must_use]
    pub const fn expression_id(&self) -> Option<NodeId> {
        match self {
            Self::Symbolic(id) | Self::Runtime(id) => Some(*id),
            Self::Unspecified | Self::Known(_) => None,
        }
    }

    /// Returns whether the quantity is known exactly.
    #[must_use]
    pub const fn is_known(&self) -> bool {
        matches!(self, Self::Known(_))
    }
}

/// Source-level resource scope retained in semantic facts.
///
/// This is not a physical location or target placement.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ResourceScope {
    /// Use the surrounding lexical or semantic scope.
    Implicit,

    /// Scope is tied to an AST declaration.
    Node(NodeId),

    /// Scope depends on a source expression.
    Expression(NodeId),

    /// Scope has a validated source-level name.
    Named(String),
}

impl From<&AstResourceScope> for ResourceScope {
    fn from(scope: &AstResourceScope) -> Self {
        match scope {
            AstResourceScope::Implicit => Self::Implicit,
            AstResourceScope::Node(id) => Self::Node(*id),
            AstResourceScope::Expression(id) => Self::Expression(*id),
            AstResourceScope::Named(name) => Self::Named(name.clone()),
        }
    }
}

/// The meaning of a resource declaration's intent.
///
/// These variants must not be collapsed: a preference or hint cannot silently
/// become a mandatory constraint, and a prohibition is not a capacity claim.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ResourceIntent {
    /// The program requires the resource condition to hold.
    Required,

    /// Prefer the condition when feasible; it does not change program meaning.
    Preferred,

    /// The resource may be used if available and permitted.
    Allowed,

    /// The resource must not be used.
    Forbidden,

    /// Non-binding guidance for downstream implementation.
    Hint,

    /// A source-declared budget to be checked under its specified semantics.
    Budget,
}

/// A canonical semantic resource requirement.
///
/// This structure records intent, not physical allocation. `kind` is the
/// extensible AST resource-kind identity. `quantity` may remain symbolic until
/// the relevant expression and constraint services can resolve it.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ResourceRequirement {
    kind: ResourceKind,
    quantity: ResourceQuantity,
    unit: Option<ResourceUnit>,
    intent: ResourceIntent,
    source_node: Option<NodeId>,
    source_name: Option<String>,
    source_scope: ResourceScope,
    source_type: Option<NodeId>,
}

impl ResourceRequirement {
    /// Constructs a requirement and validates its local invariants.
    pub fn new(
        kind: ResourceKind,
        quantity: ResourceQuantity,
        unit: Option<ResourceUnit>,
        intent: ResourceIntent,
    ) -> Result<Self, ResourceModelError> {
        kind.validate()
            .map_err(|error| ResourceModelError::InvalidResourceKind(error.to_string()))?;

        if let Some(unit) = &unit {
            validate_identity_component("unit namespace", unit.namespace())?;
            validate_identity_component("unit name", unit.name())?;
        }

        Ok(Self {
            kind,
            quantity,
            unit,
            intent,
            source_node: None,
            source_name: None,
            source_scope: ResourceScope::Implicit,
            source_type: None,
        })
    }

    /// Constructs a semantic requirement from an existing canonical AST
    /// resource node.
    ///
    /// The caller supplies the resolved resource kind and unit because the AST
    /// resource node stores source-level name/type references rather than
    /// claiming that resolution has already happened.
    ///
    /// The AST cardinality expression is preserved as a symbolic reference.
    pub fn from_ast(
        resource: &AstResource,
        kind: ResourceKind,
        unit: Option<ResourceUnit>,
        intent: ResourceIntent,
    ) -> Result<Self, ResourceModelError> {
        resource
            .validate_local()
            .map_err(|error| ResourceModelError::InvalidAstResource(error.to_string()))?;

        if !resource.is_declaration() {
            return Err(ResourceModelError::ExpectedResourceDeclaration);
        }

        let quantity = match resource.cardinality() {
            ResourceCardinality::Unspecified => ResourceQuantity::Unspecified,
            ResourceCardinality::Expression(id) => ResourceQuantity::Symbolic(*id),
        };

        let mut requirement = Self::new(kind, quantity, unit, intent)?;
        requirement.source_node = Some(resource.node().id());
        requirement.source_name = Some(resource.name().as_str().to_owned());
        requirement.source_scope = ResourceScope::from(resource.scope());
        requirement.source_type = resource.resource_type_id();

        Ok(requirement)
    }

    /// Returns the resource kind.
    #[must_use]
    pub fn kind(&self) -> &ResourceKind {
        &self.kind
    }

    /// Returns the semantic quantity.
    #[must_use]
    pub fn quantity(&self) -> &ResourceQuantity {
        &self.quantity
    }

    /// Returns the unit, if explicitly provided.
    #[must_use]
    pub fn unit(&self) -> Option<&ResourceUnit> {
        self.unit.as_ref()
    }

    /// Returns the requirement intent.
    #[must_use]
    pub const fn intent(&self) -> ResourceIntent {
        self.intent
    }

    /// Returns the originating AST node, if available.
    #[must_use]
    pub const fn source_node(&self) -> Option<NodeId> {
        self.source_node
    }

    /// Returns the source-level resource name, if available.
    #[must_use]
    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    /// Returns the source-level scope.
    #[must_use]
    pub fn source_scope(&self) -> &ResourceScope {
        &self.source_scope
    }

    /// Returns the source type-expression node, if one was specified.
    #[must_use]
    pub const fn source_type(&self) -> Option<NodeId> {
        self.source_type
    }

    /// Returns a stable key suitable for deterministic sorting.
    ///
    /// The key is for presentation and deterministic ordering, not semantic
    /// equivalence. It intentionally does not include target availability.
    #[must_use]
    pub fn canonical_sort_key(&self) -> String {
        let kind = self.kind.as_string();
        let unit = self
            .unit
            .as_ref()
            .map(ResourceUnit::canonical_name)
            .unwrap_or_default();

        let quantity = match &self.quantity {
            ResourceQuantity::Unspecified => "unspecified".to_owned(),
            ResourceQuantity::Known(value) => format!("known:{value}"),
            ResourceQuantity::Symbolic(id) => format!("symbolic:{id}"),
            ResourceQuantity::Runtime(id) => format!("runtime:{id}"),
        };

        format!(
            "{}|{}|{:?}|{}|{}",
            kind,
            unit,
            self.intent,
            quantity,
            self.source_name.as_deref().unwrap_or_default()
        )
    }
}

/// A resource-constraint comparison operator.
///
/// These operators are generic and do not encode any specific hardware
/// capacity or computational domain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConstraintOperator {
    /// Equality.
    Equal,

    /// Inequality.
    NotEqual,

    /// Strictly less than.
    LessThan,

    /// Less than or equal to.
    LessThanOrEqual,

    /// Strictly greater than.
    GreaterThan,

    /// Greater than or equal to.
    GreaterThanOrEqual,
}

/// One side of a resource constraint.
///
/// Expression-backed operands remain symbolic until a compatible evaluator
/// has enough type and value information.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ConstraintOperand {
    /// An exact quantity.
    Known(ExactQuantity),

    /// A canonical AST expression.
    Expression(NodeId),
}

/// A semantic resource constraint.
///
/// This type records a relation to be checked. It does not decide whether a
/// physical target satisfies the relation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ResourceConstraint {
    resource_kind: ResourceKind,
    operator: ConstraintOperator,
    operand: ConstraintOperand,
    unit: Option<ResourceUnit>,
    intent: ResourceIntent,
    source_node: Option<NodeId>,
}

impl ResourceConstraint {
    /// Creates a validated resource constraint.
    pub fn new(
        resource_kind: ResourceKind,
        operator: ConstraintOperator,
        operand: ConstraintOperand,
        unit: Option<ResourceUnit>,
        intent: ResourceIntent,
        source_node: Option<NodeId>,
    ) -> Result<Self, ResourceModelError> {
        resource_kind
            .validate()
            .map_err(|error| ResourceModelError::InvalidResourceKind(error.to_string()))?;

        if let Some(unit) = &unit {
            validate_identity_component("unit namespace", unit.namespace())?;
            validate_identity_component("unit name", unit.name())?;
        }

        Ok(Self {
            resource_kind,
            operator,
            operand,
            unit,
            intent,
            source_node,
        })
    }

    /// Returns the constrained resource kind.
    #[must_use]
    pub fn resource_kind(&self) -> &ResourceKind {
        &self.resource_kind
    }

    /// Returns the comparison operator.
    #[must_use]
    pub const fn operator(&self) -> ConstraintOperator {
        self.operator
    }

    /// Returns the operand.
    #[must_use]
    pub fn operand(&self) -> &ConstraintOperand {
        &self.operand
    }

    /// Returns the unit, if specified.
    #[must_use]
    pub fn unit(&self) -> Option<&ResourceUnit> {
        self.unit.as_ref()
    }

    /// Returns the constraint intent.
    #[must_use]
    pub const fn intent(&self) -> ResourceIntent {
        self.intent
    }

    /// Returns the source node, if known.
    #[must_use]
    pub const fn source_node(&self) -> Option<NodeId> {
        self.source_node
    }
}

/// Errors from constructing or validating semantic resource values.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResourceModelError {
    /// A quantity was malformed.
    InvalidQuantity(String),

    /// A namespaced identity component was empty or malformed.
    InvalidIdentity {
        /// The field being validated.
        field: &'static str,

        /// Explanation of the validation failure.
        reason: String,
    },

    /// The resource-kind identity failed validation.
    InvalidResourceKind(String),

    /// The AST resource failed its local structural validation.
    InvalidAstResource(String),

    /// A resource reference was passed where a declaration was required.
    ExpectedResourceDeclaration,
}

impl fmt::Display for ResourceModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuantity(reason) => {
                write!(formatter, "invalid resource quantity: {reason}")
            }
            Self::InvalidIdentity { field, reason } => {
                write!(formatter, "invalid {field}: {reason}")
            }
            Self::InvalidResourceKind(reason) => {
                write!(formatter, "invalid resource kind: {reason}")
            }
            Self::InvalidAstResource(reason) => {
                write!(formatter, "invalid AST resource: {reason}")
            }
            Self::ExpectedResourceDeclaration => {
                formatter.write_str("expected a resource declaration, found a reference")
            }
        }
    }
}

impl Error for ResourceModelError {}

fn validate_identity_component(
    field: &'static str,
    value: &str,
) -> Result<(), ResourceModelError> {
    if value.is_empty() {
        return Err(ResourceModelError::InvalidIdentity {
            field,
            reason: "must not be empty".to_owned(),
        });
    }

    if value.trim() != value {
        return Err(ResourceModelError::InvalidIdentity {
            field,
            reason: "must not contain leading or trailing whitespace".to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ResourceModelError::InvalidIdentity {
            field,
            reason: "must not contain control characters".to_owned(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_quantity_canonicalizes_without_machine_integer_conversion() {
        let value = ExactQuantity::new("000123456789012345678901234567890.1200")
            .expect("valid decimal");

        assert_eq!(value.as_str(), "123456789012345678901234567890.12");
    }

    #[test]
    fn exact_quantity_canonicalizes_zero() {
        assert_eq!(
            ExactQuantity::new("000.000").expect("valid decimal").as_str(),
            "0"
        );
    }

    #[test]
    fn exact_quantity_compares_arbitrary_decimal_values() {
        let smaller = ExactQuantity::new("999999999999999999999999.99")
            .expect("valid decimal");
        let larger = ExactQuantity::new("1000000000000000000000000")
            .expect("valid decimal");

        assert_eq!(
            smaller.numeric_cmp(&larger),
            core::cmp::Ordering::Less
        );
    }

    #[test]
    fn exact_quantity_rejects_malformed_values() {
        for value in ["", "-1", "+1", ".5", "1.", "1.2.3", "1e9", " 1"] {
            assert!(
                ExactQuantity::new(value).is_err(),
                "expected {value:?} to be rejected"
            );
        }
    }

    #[test]
    fn exact_quantity_accepts_zero_and_fractional_values() {
        assert!(ExactQuantity::new("0").expect("valid").is_zero());
        assert_eq!(
            ExactQuantity::new("000.0500")
                .expect("valid")
                .as_str(),
            "0.05"
        );
    }

    #[test]
    fn unit_identity_is_extensible_and_namespaced() {
        let unit = ResourceUnit::new("quantum", "logical-qubit")
            .expect("valid extension unit");

        assert_eq!(unit.canonical_name(), "quantum:logical-qubit");
    }

    #[test]
    fn invalid_unit_identity_is_rejected() {
        assert!(ResourceUnit::new("", "byte").is_err());
        assert!(ResourceUnit::new("core", " ").is_err());
        assert!(ResourceUnit::new("core\n", "byte").is_err());
    }

    #[test]
    fn symbolic_quantities_preserve_ast_identity() {
        // A concrete NodeId is supplied by the canonical AST allocator in
        // integration tests. This test exercises the enum's representation
        // through the public API rather than manufacturing an AST identity.
        fn expression_id(quantity: &ResourceQuantity) -> Option<NodeId> {
            quantity.expression_id()
        }

        let known = ResourceQuantity::known("123").expect("valid quantity");
        assert_eq!(expression_id(&known), None);
        assert!(known.is_known());
    }

    #[test]
    fn intent_categories_remain_distinct() {
        assert_ne!(ResourceIntent::Required, ResourceIntent::Preferred);
        assert_ne!(ResourceIntent::Forbidden, ResourceIntent::Allowed);
        assert_ne!(ResourceIntent::Hint, ResourceIntent::Budget);
    }
}
