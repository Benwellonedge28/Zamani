
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Operation Contracts
//!
//! Path: `src/hdl/hdlir/schema/operation.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Add detailed, declarative contracts to the operation definitions owned by
//! `schema::registry`. This module describes operand/result groups, type
//! constraints, attributes, regions, effects, and named verifier requirements.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/extensibility.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/schema/registry.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax; the frontend owns source semantics; ZUIR owns the
//! universal computational representation; HDLIR owns hardware-domain
//! operation contracts. Physical realization remains downstream.
//!
//! ## Ownership
//!
//! This module owns declarative constraints for registered operation schemas.
//! It does not own operation instances, concrete type inference, the type
//! registry, verifier execution, source parsing, target discovery, or hardware
//! allocation. A named verifier requirement is a contract identifier, not an
//! executable callback and not proof that a verifier is installed.
//!
//! ## Scalability and determinism
//!
//! No universal operation, operand, result, attribute, or region count is
//! imposed. Arity is declared by each operation schema. Counts use checked
//! arithmetic; ordered groups preserve their declared order; sets and maps
//! use deterministic ordering. Operational budgets are enforced by callers.
//!
//! ## Integration
//!
//! `OperationDefinition`, `OperationSignature`, `Arity`, and `QualifiedName`
//! remain owned by `schema::registry`. This module composes with those types
//! without redefining them. Builders consume these contracts during
//! construction; verifiers interpret them against concrete IR; serialization
//! persists them only through an explicitly versioned artifact contract.
//!
//! ## Completion criteria
//!
//! - Declared group arities exactly agree with the registered signature.
//! - Group labels and attribute names are valid and unique.
//! - Cross-group type references resolve to declared operand groups.
//! - Region and effect contracts are explicit and deterministic.
//! - Missing verifier implementations are reported by the verifier layer.
//! - Invalid contracts return structured errors rather than panicking.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};

use super::registry::{
    Arity, OperationDefinition, QualifiedName,
};

// =============================================================================
// Operand and result contracts
// =============================================================================

/// Describes the required type relationship for an operand or result group.
///
/// These are declarative constraints. The type verifier is responsible for
/// interpreting them against the actual HDLIR type system. In particular,
/// `IntegerLike`, `LogicValue`, and `SignalLike` do not define new types.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum TypeConstraint {
    /// Any type admitted by the operation's other constraints.
    Any,

    /// The value must have this registered type identity.
    Exact(QualifiedName),

    /// The value must have integer-like semantics according to the type system.
    IntegerLike,

    /// The value must represent a supported logic value or logic vector.
    LogicValue,

    /// The value must be a logical hardware signal type.
    SignalLike,

    /// The value must have a type compatible with the named operand group.
    ///
    /// The group label must identify a declared operand group in the same
    /// operation contract. Compatibility, including any symbolic-width
    /// reasoning, is determined by the type verifier.
    SameAsOperandGroup(String),
}

/// A named, ordered group of operands or results.
///
/// Groups support both fixed and variadic portions of an operation signature.
/// A variadic group is represented by an `Arity` with no maximum; it does not
/// impose an implicit universal maximum.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueGroupContract {
    label: String,
    arity: Arity,
    type_constraint: TypeConstraint,
}

impl ValueGroupContract {
    /// Create a named operand or result group.
    pub fn new(
        label: impl Into<String>,
        arity: Arity,
        type_constraint: TypeConstraint,
    ) -> HdlirResult<Self> {
        let label = label.into();
        validate_label("value_group.label", &label)?;

        Ok(Self {
            label,
            arity,
            type_constraint,
        })
    }

    /// Return the group's stable local label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Return the group's arity constraint.
    #[must_use]
    pub const fn arity(&self) -> Arity {
        self.arity
    }

    /// Return the group's type constraint.
    #[must_use]
    pub fn type_constraint(&self) -> &TypeConstraint {
        &self.type_constraint
    }
}

// =============================================================================
// Attribute contracts
// =============================================================================

/// The declared value category of an operation attribute.
///
/// These categories specify the schema contract, not the storage encoding.
/// A serialization format must define the exact canonical representation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AttributeKind {
    /// Boolean value.
    Boolean,

    /// Signed integer value.
    SignedInteger,

    /// Unsigned integer value.
    UnsignedInteger,

    /// Text value.
    String,

    /// Qualified schema name.
    QualifiedName,

    /// Registered type identity.
    Type,

    /// Constant value whose exact representation is defined elsewhere.
    Constant,

    /// Symbolic expression evaluated by an appropriate downstream service.
    SymbolicExpression,

    /// Ordered sequence of attribute values.
    Array,

    /// Keyed aggregate of attribute values.
    Dictionary,

    /// Extension-defined value with separately registered semantics.
    Extension,
}

/// A single named attribute's requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttributeContract {
    name: String,
    kind: AttributeKind,
    required: bool,
    description: String,
}

impl AttributeContract {
    /// Create an attribute contract.
    pub fn new(
        name: impl Into<String>,
        kind: AttributeKind,
        required: bool,
        description: impl Into<String>,
    ) -> HdlirResult<Self> {
        let name = name.into();
        let description = description.into();

        validate_label("attribute.name", &name)?;

        if description.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "attribute.description",
                "description must not be empty",
            ));
        }

        Ok(Self {
            name,
            kind,
            required,
            description,
        })
    }

    /// Return the canonical local attribute name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the required attribute category.
    #[must_use]
    pub const fn kind(&self) -> AttributeKind {
        self.kind
    }

    /// Return whether the attribute must be present.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }

    /// Return the attribute's human-readable description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

// =============================================================================
// Region contracts
// =============================================================================

/// The structural model permitted for an operation region.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum RegionShape {
    /// The region contains exactly one block when present.
    SingleBlock,

    /// The region may contain multiple blocks with defined control flow.
    MultiBlock,

    /// The region is a graph-like body whose semantics are defined by its
    /// owning dialect rather than by a mandatory linear block sequence.
    Graph,

    /// The operation's dialect defines the detailed region structure.
    DialectDefined,
}

/// Contract for one kind of nested operation region.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionContract {
    label: String,
    arity: Arity,
    shape: RegionShape,
    requires_terminator: bool,
    description: String,
}

impl RegionContract {
    /// Create a region contract.
    pub fn new(
        label: impl Into<String>,
        arity: Arity,
        shape: RegionShape,
        requires_terminator: bool,
        description: impl Into<String>,
    ) -> HdlirResult<Self> {
        let label = label.into();
        let description = description.into();

        validate_label("region.label", &label)?;

        if description.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "region.description",
                "description must not be empty",
            ));
        }

        Ok(Self {
            label,
            arity,
            shape,
            requires_terminator,
            description,
        })
    }

    /// Return the region's stable local label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Return the permitted number of regions in this group.
    #[must_use]
    pub const fn arity(&self) -> Arity {
        self.arity
    }

    /// Return the region's structural shape.
    #[must_use]
    pub const fn shape(&self) -> RegionShape {
        self.shape
    }

    /// Return whether the region must end with a legal terminator.
    #[must_use]
    pub const fn requires_terminator(&self) -> bool {
        self.requires_terminator
    }

    /// Return the region's human-readable description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

// =============================================================================
// Effects and verifier requirements
// =============================================================================

/// A semantic effect that an operation may declare.
///
/// This enum describes broad effect categories. Dialects may use separately
/// registered interfaces or extensions for more specific effect semantics.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum OperationEffect {
    /// Reads logical state or memory.
    Read,

    /// Writes logical state or memory.
    Write,

    /// Changes state across a clock or other declared event.
    StateTransition,

    /// Communicates with another logical component.
    Communication,

    /// Performs an externally observable action.
    Observable,

    /// Depends on or changes clock/reset-related behavior.
    ClockOrReset,

    /// Performs memory access whose semantics are defined by its dialect.
    MemoryAccess,

    /// Represents an effect defined by an extension contract.
    ExtensionDefined,
}

/// Named verifier requirement attached to an operation schema.
///
/// The identifier is resolved by the verification subsystem. Merely
/// declaring it does not execute the verifier or certify the operation.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VerifierRequirement {
    name: QualifiedName,
    required: bool,
}

impl VerifierRequirement {
    /// Create a verifier requirement using a registered qualified name.
    #[must_use]
    pub fn new(name: QualifiedName, required: bool) -> Self {
        Self { name, required }
    }

    /// Return the verifier contract identifier.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return whether the verifier must be available for successful
    /// verification of this operation.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

// =============================================================================
// Full operation contract
// =============================================================================

/// A complete declarative contract attached to one registered operation.
///
/// The underlying `OperationDefinition` remains authoritative for the
/// operation's qualified name, dialect, version, stability, interfaces, and
/// overall operand/result arity. This type adds detailed constraints without
/// defining a second operation identity or registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationContract {
    definition: OperationDefinition,
    operand_groups: Vec<ValueGroupContract>,
    result_groups: Vec<ValueGroupContract>,
    attributes: BTreeMap<String, AttributeContract>,
    regions: Vec<RegionContract>,
    effects: BTreeSet<OperationEffect>,
    verifiers: BTreeSet<VerifierRequirement>,
}

impl OperationContract {
    /// Construct and validate a complete operation contract.
    ///
    /// The sum of the operand-group arities must exactly match the registered
    /// operation's operand arity. The same rule applies to result groups.
    /// All group and region labels must be unique within their respective
    /// namespaces. Type references to operand groups must resolve.
    pub fn new(
        definition: OperationDefinition,
        operand_groups: Vec<ValueGroupContract>,
        result_groups: Vec<ValueGroupContract>,
        attributes: Vec<AttributeContract>,
        regions: Vec<RegionContract>,
        effects: BTreeSet<OperationEffect>,
        verifiers: BTreeSet<VerifierRequirement>,
    ) -> HdlirResult<Self> {
        validate_value_groups(
            "operation.operands",
            &operand_groups,
            definition.signature().operands(),
        )?;
        validate_value_groups(
            "operation.results",
            &result_groups,
            definition.signature().results(),
        )?;
        validate_type_references(&operand_groups, &result_groups)?;

        let mut attribute_map = BTreeMap::new();
        for attribute in attributes {
            let key = attribute.name().to_owned();
            if attribute_map.insert(key.clone(), attribute).is_some() {
                return Err(HdlirError::invalid_input(
                    "operation.attributes",
                    format!("duplicate attribute contract `{key}`"),
                ));
            }
        }

        let mut region_labels = BTreeSet::new();
        for region in &regions {
            if !region_labels.insert(region.label().to_owned()) {
                return Err(HdlirError::invalid_input(
                    "operation.regions",
                    format!("duplicate region contract `{}`", region.label()),
                ));
            }
        }

        Ok(Self {
            definition,
            operand_groups,
            result_groups,
            attributes: attribute_map,
            regions,
            effects,
            verifiers,
        })
    }

    /// Return the registered operation definition.
    #[must_use]
    pub fn definition(&self) -> &OperationDefinition {
        &self.definition
    }

    /// Return the ordered operand-group contracts.
    #[must_use]
    pub fn operand_groups(&self) -> &[ValueGroupContract] {
        &self.operand_groups
    }

    /// Return the ordered result-group contracts.
    #[must_use]
    pub fn result_groups(&self) -> &[ValueGroupContract] {
        &self.result_groups
    }

    /// Return attribute contracts in deterministic name order.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, AttributeContract> {
        &self.attributes
    }

    /// Return the ordered region contracts.
    #[must_use]
    pub fn regions(&self) -> &[RegionContract] {
        &self.regions
    }

    /// Return the declared effects in deterministic order.
    #[must_use]
    pub fn effects(&self) -> &BTreeSet<OperationEffect> {
        &self.effects
    }

    /// Return the required or optional verifier contracts in deterministic
    /// order.
    #[must_use]
    pub fn verifiers(&self) -> &BTreeSet<VerifierRequirement> {
        &self.verifiers
    }

    /// Return whether a concrete operand count is permitted by the registered
    /// operation signature.
    #[must_use]
    pub const fn accepts_operand_count(&self, count: u64) -> bool {
        self.definition.signature().operands().accepts(count)
    }

    /// Return whether a concrete result count is permitted by the registered
    /// operation signature.
    #[must_use]
    pub const fn accepts_result_count(&self, count: u64) -> bool {
        self.definition.signature().results().accepts(count)
    }

    /// Return whether the named attribute is part of this operation schema.
    #[must_use]
    pub fn attribute(&self, name: &str) -> Option<&AttributeContract> {
        self.attributes.get(name)
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_label(field: &'static str, label: &str) -> HdlirResult<()> {
    if label.is_empty() {
        return Err(HdlirError::invalid_input(
            field,
            "label must not be empty",
        ));
    }

    let mut bytes = label.bytes();
    let first = bytes.next().ok_or_else(|| {
        HdlirError::invalid_input(field, "label must not be empty")
    })?;

    if !(first.is_ascii_lowercase() || first == b'_')
        || !bytes.all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'_'
                || byte == b'-'
        })
    {
        return Err(HdlirError::invalid_input(
            field,
            format!(
                "label `{label}` must use lowercase ASCII letters, digits, `_` or `-`, \
                 and begin with a lowercase letter or `_`"
            ),
        ));
    }

    Ok(())
}

fn validate_value_groups(
    field: &'static str,
    groups: &[ValueGroupContract],
    expected: Arity,
) -> HdlirResult<()> {
    let mut labels = BTreeSet::new();
    let mut minimum = 0_u64;
    let mut maximum = Some(0_u64);

    for group in groups {
        if !labels.insert(group.label().to_owned()) {
            return Err(HdlirError::invalid_input(
                field,
                format!("duplicate value-group label `{}`", group.label()),
            ));
        }

        minimum = minimum.checked_add(group.arity().min()).ok_or_else(|| {
            HdlirError::ArithmeticOverflow {
                operation: "summing operation group minimum arities",
                context: Some(field.to_owned()),
            }
        })?;

        maximum = match (maximum, group.arity().max()) {
            (Some(total), Some(part)) => Some(total.checked_add(part).ok_or_else(|| {
                HdlirError::ArithmeticOverflow {
                    operation: "summing operation group maximum arities",
                    context: Some(field.to_owned()),
                }
            })?),
            _ => None,
        };
    }

    if minimum != expected.min() || maximum != expected.max() {
        return Err(HdlirError::invalid_input(
            field,
            format!(
                "group arity ({minimum}..={maximum:?}) does not match registered \
                 operation arity ({}..={:?})",
                expected.min(),
                expected.max()
            ),
        ));
    }

    Ok(())
}

fn validate_type_references(
    operands: &[ValueGroupContract],
    results: &[ValueGroupContract],
) -> HdlirResult<()> {
    let operand_labels: BTreeSet<&str> =
        operands.iter().map(ValueGroupContract::label).collect();

    for group in operands.iter().chain(results.iter()) {
        if let TypeConstraint::SameAsOperandGroup(reference) = group.type_constraint() {
            if !operand_labels.contains(reference.as_str()) {
                return Err(HdlirError::invalid_input(
                    "operation.type_constraint",
                    format!(
                        "group `{}` references undeclared operand group `{reference}`",
                        group.label()
                    ),
                ));
            }
        }
    }

    Ok(())
}

// =============================================================================
// Focused tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::version::HdlirVersion;
    use crate::hdl::hdlir::schema::registry::{
        DefinitionStability, OperationSignature,
    };

    fn qualified(name: &str) -> QualifiedName {
        QualifiedName::parse(name).expect("test name must be valid")
    }

    fn operation_definition(
        operands: Arity,
        results: Arity,
    ) -> OperationDefinition {
        let dialect = qualified("hdl.structural");
        OperationDefinition::new(
            qualified("hdl.structural.connect"),
            dialect,
            HdlirVersion::default(),
            DefinitionStability::Core,
            OperationSignature::new(operands, results),
            BTreeSet::new(),
            "Connect compatible logical values.",
        )
        .expect("test operation definition must be valid")
    }

    #[test]
    fn constructs_a_fixed_arity_operation_contract() {
        let definition = operation_definition(
            Arity::new(2, Some(2)).expect("valid arity"),
            Arity::new(0, Some(0)).expect("valid arity"),
        );

        let contract = OperationContract::new(
            definition,
            vec![
                ValueGroupContract::new(
                    "source",
                    Arity::new(1, Some(1)).expect("valid arity"),
                    TypeConstraint::SignalLike,
                )
                .expect("valid group"),
                ValueGroupContract::new(
                    "destination",
                    Arity::new(1, Some(1)).expect("valid arity"),
                    TypeConstraint::SignalLike,
                )
                .expect("valid group"),
            ],
            vec![],
            vec![],
            vec![],
            BTreeSet::from([OperationEffect::Write]),
            BTreeSet::new(),
        )
        .expect("contract must be valid");

        assert!(contract.accepts_operand_count(2));
        assert!(contract.accepts_result_count(0));
        assert_eq!(contract.operand_groups().len(), 2);
        assert!(contract.effects().contains(&OperationEffect::Write));
    }

    #[test]
    fn rejects_group_arity_that_disagrees_with_registry_signature() {
        let definition = operation_definition(
            Arity::new(2, Some(2)).expect("valid arity"),
            Arity::new(0, Some(0)).expect("valid arity"),
        );

        let result = OperationContract::new(
            definition,
            vec![ValueGroupContract::new(
                "source",
                Arity::new(1, Some(1)).expect("valid arity"),
                TypeConstraint::Any,
            )
            .expect("valid group")],
            vec![],
            vec![],
            vec![],
            BTreeSet::new(),
            BTreeSet::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn supports_unbounded_schema_arity_without_a_fixed_maximum() {
        let unbounded = Arity::new(0, None).expect("valid unbounded arity");
        let definition = operation_definition(unbounded, unbounded);

        let contract = OperationContract::new(
            definition,
            vec![ValueGroupContract::new(
                "inputs",
                unbounded,
                TypeConstraint::Any,
            )
            .expect("valid group")],
            vec![ValueGroupContract::new(
                "outputs",
                unbounded,
                TypeConstraint::Any,
            )
            .expect("valid group")],
            vec![],
            vec![],
            BTreeSet::new(),
            BTreeSet::new(),
        )
        .expect("unbounded contract must be valid");

        assert!(contract.accepts_operand_count(u64::MAX));
        assert!(contract.accepts_result_count(u64::MAX));
    }

    #[test]
    fn rejects_duplicate_attribute_names() {
        let definition = operation_definition(
            Arity::new(0, Some(0)).expect("valid arity"),
            Arity::new(0, Some(0)).expect("valid arity"),
        );

        let first = AttributeContract::new(
            "mode",
            AttributeKind::String,
            true,
            "Operation mode.",
        )
        .expect("valid attribute");

        let second = AttributeContract::new(
            "mode",
            AttributeKind::String,
            false,
            "Conflicting duplicate.",
        )
        .expect("individually valid attribute");

        assert!(OperationContract::new(
            definition,
            vec![],
            vec![],
            vec![first, second],
            vec![],
            BTreeSet::new(),
            BTreeSet::new(),
        )
        .is_err());
    }

    #[test]
    fn rejects_unresolved_operand_group_type_references() {
        let definition = operation_definition(
            Arity::new(1, Some(1)).expect("valid arity"),
            Arity::new(1, Some(1)).expect("valid arity"),
        );

        let input = ValueGroupContract::new(
            "input",
            Arity::new(1, Some(1)).expect("valid arity"),
            TypeConstraint::Any,
        )
        .expect("valid group");

        let output = ValueGroupContract::new(
            "output",
            Arity::new(1, Some(1)).expect("valid arity"),
            TypeConstraint::SameAsOperandGroup("missing".to_owned()),
        )
        .expect("valid group");

        assert!(OperationContract::new(
            definition,
            vec![input],
            vec![output],
            vec![],
            vec![],
            BTreeSet::new(),
            BTreeSet::new(),
        )
        .is_err());
    }

    #[test]
    fn rejects_duplicate_region_labels() {
        let definition = operation_definition(
            Arity::new(0, Some(0)).expect("valid arity"),
            Arity::new(0, Some(0)).expect("valid arity"),
        );

        let region = || {
            RegionContract::new(
                "body",
                Arity::new(1, Some(1)).expect("valid arity"),
                RegionShape::SingleBlock,
                true,
                "Operation body.",
            )
            .expect("valid region")
        };

        assert!(OperationContract::new(
            definition,
            vec![],
            vec![],
            vec![],
            vec![region(), region()],
            BTreeSet::new(),
            BTreeSet::new(),
        )
        .is_err());
    }
}
