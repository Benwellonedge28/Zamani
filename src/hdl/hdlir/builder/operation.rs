
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Checked Operation Builder
//!
//! Path: `src/hdl/hdlir/builder/operation.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Implementation candidate; repository integration and compilation
//! must be verified before declaring this module production-ready.
//!
//! ## Purpose
//!
//! Construct hardware-domain operation instances through the existing
//! `HdlirOperation` representation and a validated `OperationContract`.
//!
//! The builder validates:
//!
//! - Operation identity and schema version.
//! - Operand and result arity.
//! - Local result and region identity uniqueness.
//! - Attribute names, declared kinds, and required attributes.
//! - The aggregate number of region references permitted by the contract.
//!
//! Graph-wide verification remains responsible for resolving references,
//! checking type constraints, verifying region shapes and ownership,
//! enforcing dialect-specific rules, and validating operation effects.
//!
//! ## Architectural authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/operations.rs`
//! - `src/hdl/hdlir/ir/values.rs`
//! - `src/hdl/hdlir/ir/regions.rs`
//! - `src/hdl/hdlir/schema/operation.rs`
//! - `src/hdl/hdlir/schema/registry.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax. The frontend owns source-level meaning.
//! ZUIR owns universal computational representation. HDLIR owns
//! hardware-domain operation representation and its construction contracts.
//!
//! ## Ownership
//!
//! This file owns construction orchestration only. It does not own a second
//! operation representation, schema registry, type system, value table,
//! region graph, semantic analyzer, verifier registry, or target allocator.
//!
//! ## Scalability
//!
//! No universal maximum is imposed on operands, results, attributes, or
//! regions. Collections grow according to available implementation resources.
//! Schema arities are checked using `u64`; conversion failures are reported.
//! The builder does not claim to process literally infinite input.
//!
//! ## Determinism
//!
//! Operand, result, and region order is preserved. Attributes use `BTreeMap`
//! ordering. No pointer identity, random value, or hash iteration order is
//! used to establish operation semantics.
//!
//! ## Completion criteria
//!
//! Unit tests, repository compilation, whole-graph verification, integration
//! with the real schema-registration workflow, and the repository's required
//! formatting, test, and lint checks must pass before this module is frozen.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirOperationId,
    HdlirRegionId,
    HdlirTypeId,
    HdlirValueId,
};
use crate::hdl::hdlir::ir::operations::{
    HdlirOperation,
    OperationAttributeValue,
    OperationResult,
};
use crate::hdl::hdlir::schema::operation::{
    AttributeKind,
    OperationContract,
};
use crate::hdl::hdlir::schema::registry::Arity;

/// Checked, single-operation construction context.
///
/// The referenced contract must have been constructed from an operation
/// definition obtained from the appropriate schema-registration workflow.
/// This builder cannot independently prove that a definition was registered.
///
/// The contract is borrowed rather than copied so callers can reuse a
/// validated contract for many operation instances.
#[derive(Debug)]
pub struct OperationBuilder<'contract> {
    id: HdlirOperationId,
    contract: &'contract OperationContract,
    operands: Vec<HdlirValueId>,
    results: Vec<OperationResult>,
    attributes: BTreeMap<String, OperationAttributeValue>,
    regions: Vec<HdlirRegionId>,
}

impl<'contract> OperationBuilder<'contract> {
    /// Create a builder for an operation using a validated operation contract.
    ///
    /// No operation is inserted into a graph by this constructor.
    /// The supplied identity must already have been allocated by the caller.
    ///
    /// # Errors
    ///
    /// This constructor has no input-dependent failure after receiving valid
    /// typed IDs and a reference to an already constructed contract.
    #[must_use]
    pub fn new(
        id: HdlirOperationId,
        contract: &'contract OperationContract,
    ) -> Self {
        Self {
            id,
            contract,
            operands: Vec::new(),
            results: Vec::new(),
            attributes: BTreeMap::new(),
            regions: Vec::new(),
        }
    }

    /// Return the operation identity.
    #[must_use]
    pub const fn id(&self) -> HdlirOperationId {
        self.id
    }

    /// Return the operation's qualified schema name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.contract.definition().name().as_str()
    }

    /// Return the schema version used for this operation.
    #[must_use]
    pub fn schema_version(
        &self,
    ) -> crate::hdl::hdlir::version::HdlirVersion {
        self.contract.definition().version()
    }

    /// Return the number of operands currently supplied.
    #[must_use]
    pub fn operand_count(&self) -> usize {
        self.operands.len()
    }

    /// Return the number of results currently supplied.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Return the number of attributes currently supplied.
    #[must_use]
    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }

    /// Return the number of region references currently supplied.
    #[must_use]
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// Append an operand reference.
    ///
    /// Repeated operand IDs are permitted because an operation may consume
    /// the same logical value more than once.
    ///
    /// Reference existence, dominance, scope, and type compatibility are
    /// checked by the relevant graph and verification components.
    pub fn add_operand(
        &mut self,
        operand: HdlirValueId,
    ) -> &mut Self {
        self.operands.push(operand);
        self
    }

    /// Append multiple operand references while preserving their order.
    pub fn add_operands(
        &mut self,
        operands: impl IntoIterator<Item = HdlirValueId>,
    ) -> &mut Self {
        self.operands.extend(operands);
        self
    }

    /// Declare a typed result.
    ///
    /// Duplicate result identities are rejected during finalization.
    /// The referenced type must be resolved by the type registry and verifier.
    pub fn add_result(
        &mut self,
        result: OperationResult,
    ) -> &mut Self {
        self.results.push(result);
        self
    }

    /// Append multiple typed results while preserving their order.
    pub fn add_results(
        &mut self,
        results: impl IntoIterator<Item = OperationResult>,
    ) -> &mut Self {
        self.results.extend(results);
        self
    }

    /// Insert an attribute declared by the operation contract.
    ///
    /// Unknown attributes and duplicate names are rejected immediately.
    /// The value's category is checked against the declared attribute kind.
    ///
    /// # Errors
    ///
    /// Returns a contract violation for an unknown attribute or incompatible
    /// attribute kind. A duplicate insertion returns a duplicate-entity error.
    pub fn add_attribute(
        &mut self,
        name: impl Into<String>,
        value: OperationAttributeValue,
    ) -> HdlirResult<&mut Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "operation.attribute_name",
                "attribute name must not be empty or whitespace-only",
            ));
        }

        let contract = self.contract.attribute(&name).ok_or_else(|| {
            HdlirError::OperationContractViolation {
                schema: self.contract.definition().name().to_string(),
                operation: self.id.to_string(),
                detail: format!(
                    "attribute `{name}` is not declared by the operation contract"
                ),
            }
        })?;

        if !attribute_kind_matches(contract.kind(), &value) {
            return Err(HdlirError::OperationContractViolation {
                schema: self.contract.definition().name().to_string(),
                operation: self.id.to_string(),
                detail: format!(
                    "attribute `{name}` has a value incompatible with \
                     declared kind {:?}",
                    contract.kind()
                ),
            });
        }

        if self.attributes.contains_key(&name) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR operation attribute",
                identity: name,
                scope: Some(self.id.to_string()),
            });
        }

        self.attributes.insert(name, value);
        Ok(self)
    }

    /// Append a nested-region reference.
    ///
    /// This records an identity reference only. Region existence, ownership,
    /// shape, terminators, and block membership are graph-wide invariants.
    pub fn add_region(
        &mut self,
        region: HdlirRegionId,
    ) -> &mut Self {
        self.regions.push(region);
        self
    }

    /// Append multiple region references while preserving their order.
    pub fn add_regions(
        &mut self,
        regions: impl IntoIterator<Item = HdlirRegionId>,
    ) -> &mut Self {
        self.regions.extend(regions);
        self
    }

    /// Validate the current construction state without consuming the builder.
    ///
    /// This checks the operation contract's locally decidable requirements.
    /// It does not verify external references, type constraints, region shapes,
    /// registered verifier availability, or target feasibility.
    ///
    /// # Errors
    ///
    /// Returns a structured HDLIR error if any locally checked requirement
    /// is violated.
    pub fn validate(&self) -> HdlirResult<()> {
        let definition = self.contract.definition();

        let operand_count = u64::try_from(self.operands.len()).map_err(|_| {
            HdlirError::ConversionOutOfRange {
                value: self.operands.len().to_string(),
                destination: "u64 operand count",
            }
        })?;

        let result_count = u64::try_from(self.results.len()).map_err(|_| {
            HdlirError::ConversionOutOfRange {
                value: self.results.len().to_string(),
                destination: "u64 result count",
            }
        })?;

        if !self.contract.accepts_operand_count(operand_count) {
            return Err(contract_violation(
                self.contract,
                self.id,
                format!(
                    "operand count {operand_count} violates schema arity {}..{:?}",
                    definition.signature().operands().min(),
                    definition.signature().operands().max(),
                ),
            ));
        }

        if !self.contract.accepts_result_count(result_count) {
            return Err(contract_violation(
                self.contract,
                self.id,
                format!(
                    "result count {result_count} violates schema arity {}..{:?}",
                    definition.signature().results().min(),
                    definition.signature().results().max(),
                ),
            ));
        }

        self.validate_attributes()?;
        self.validate_region_count()?;

        // Use the canonical operation constructor to validate local identity
        // uniqueness and attribute-name invariants. No alternate IR model is
        // introduced here.
        let operation = HdlirOperation::new(
            self.id,
            definition.name().clone(),
            definition.version(),
            self.operands.clone(),
            self.results.clone(),
            self.attributes.clone(),
            self.regions.clone(),
        )?;

        operation.validate_against_definition(definition)
    }

    /// Finalize the operation after all locally decidable contract checks pass.
    ///
    /// The returned operation is not a certificate of whole-graph validity.
    /// Before a design is trusted, the owning graph must resolve all operand,
    /// result-type, and region references and run the required verifiers.
    ///
    /// # Errors
    ///
    /// Returns a structured error for invalid arity, attributes, region count,
    /// duplicate result IDs, duplicate region IDs, or schema mismatch.
    pub fn build(self) -> HdlirResult<HdlirOperation> {
        self.validate()?;

        let definition = self.contract.definition();

        HdlirOperation::new(
            self.id,
            definition.name().clone(),
            definition.version(),
            self.operands,
            self.results,
            self.attributes,
            self.regions,
        )
    }

    fn validate_attributes(&self) -> HdlirResult<()> {
        for (name, value) in &self.attributes {
            let attribute_contract =
                self.contract.attribute(name).ok_or_else(|| {
                    contract_violation(
                        self.contract,
                        self.id,
                        format!(
                            "attribute `{name}` is not declared by the operation contract"
                        ),
                    )
                })?;

            if !attribute_kind_matches(attribute_contract.kind(), value) {
                return Err(contract_violation(
                    self.contract,
                    self.id,
                    format!(
                        "attribute `{name}` does not satisfy declared kind {:?}",
                        attribute_contract.kind()
                    ),
                ));
            }
        }

        for attribute_contract in self.contract.attributes().values() {
            if attribute_contract.required()
                && !self.attributes.contains_key(attribute_contract.name())
            {
                return Err(contract_violation(
                    self.contract,
                    self.id,
                    format!(
                        "required attribute `{}` is missing",
                        attribute_contract.name()
                    ),
                ));
            }
        }

        Ok(())
    }

    fn validate_region_count(&self) -> HdlirResult<()> {
        let actual = u64::try_from(self.regions.len()).map_err(|_| {
            HdlirError::ConversionOutOfRange {
                value: self.regions.len().to_string(),
                destination: "u64 region count",
            }
        })?;

        let mut minimum = 0_u64;
        let mut maximum: Option<u64> = Some(0);

        for region_contract in self.contract.regions() {
            minimum = minimum.checked_add(region_contract.arity().min())
                .ok_or_else(|| HdlirError::ArithmeticOverflow {
                    operation: "sum operation region minimum arities",
                    context: Some(
                        self.contract.definition().name().to_string()
                    ),
                })?;

            maximum = match (maximum, region_contract.arity().max()) {
                (Some(left), Some(right)) => {
                    Some(left.checked_add(right).ok_or_else(|| {
                        HdlirError::ArithmeticOverflow {
                            operation: "sum operation region maximum arities",
                            context: Some(
                                self.contract.definition().name().to_string()
                            ),
                        }
                    })?)
                }
                _ => None,
            };
        }

        let within_minimum = actual >= minimum;
        let within_maximum = maximum.is_none_or(|limit| actual <= limit);

        if !within_minimum || !within_maximum {
            return Err(contract_violation(
                self.contract,
                self.id,
                format!(
                    "region count {actual} violates aggregate contract \
                     arity {minimum}..{maximum:?}"
                ),
            ));
        }

        Ok(())
    }
}

fn contract_violation(
    contract: &OperationContract,
    id: HdlirOperationId,
    detail: String,
) -> HdlirError {
    HdlirError::OperationContractViolation {
        schema: contract.definition().name().to_string(),
        operation: id.to_string(),
        detail,
    }
}

/// Check the storage category of an attribute against its declared schema.
///
/// This deliberately checks representation category, not the semantic
/// validity of the payload. Schema-specific decoders and verifiers own
/// payload interpretation.
fn attribute_kind_matches(
    kind: AttributeKind,
    value: &OperationAttributeValue,
) -> bool {
    match (kind, value) {
        (AttributeKind::Boolean, OperationAttributeValue::Boolean(_)) => true,
        (
            AttributeKind::SignedInteger,
            OperationAttributeValue::SignedInteger(_),
        ) => true,
        (
            AttributeKind::UnsignedInteger,
            OperationAttributeValue::UnsignedInteger(_),
        ) => true,
        (AttributeKind::String, OperationAttributeValue::String(_)) => true,
        (
            AttributeKind::QualifiedName,
            OperationAttributeValue::QualifiedName(_),
        ) => true,
        (AttributeKind::Type, OperationAttributeValue::Type(_)) => true,
        (
            AttributeKind::Constant,
            OperationAttributeValue::ConstantPayload { .. },
        ) => true,
        (
            AttributeKind::SymbolicExpression,
            OperationAttributeValue::SymbolicExpressionPayload { .. },
        ) => true,
        (AttributeKind::Array, OperationAttributeValue::Array(_)) => true,
        (
            AttributeKind::Dictionary,
            OperationAttributeValue::Dictionary(_),
        ) => true,
        (
            AttributeKind::Extension,
            OperationAttributeValue::ExtensionPayload { .. },
        ) => true,

        // `OperationAttributeValue::Value` does not have a corresponding
        // declared AttributeKind in the current schema API. Do not silently
        // reinterpret it as another category.
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hdl::hdlir::schema::operation::{
        AttributeContract,
        OperationEffect,
        RegionContract,
        RegionShape,
        TypeConstraint,
        ValueGroupContract,
    };
    use crate::hdl::hdlir::schema::registry::{
        DefinitionStability,
        OperationDefinition,
        OperationSignature,
        QualifiedName,
    };
    use crate::hdl::hdlir::version::HdlirVersion;

    use std::collections::{BTreeMap, BTreeSet};

    fn qualified(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test qualified name must be valid")
    }

    fn operation_definition(
        operands: Arity,
        results: Arity,
    ) -> OperationDefinition {
        let dialect = qualified("structural.core");
        let name = qualified("structural.core.noop");

        OperationDefinition::new(
            name,
            dialect,
            HdlirVersion::new(1, 0, 0),
            DefinitionStability::Stable,
            OperationSignature::new(operands, results),
            BTreeSet::new(),
            "Test operation",
        )
        .expect("test definition must be valid")
    }

    fn contract(
        operand_arity: Arity,
        result_arity: Arity,
        attributes: Vec<AttributeContract>,
        regions: Vec<RegionContract>,
    ) -> OperationContract {
        let operand_groups = if operand_arity.min() == 0
            && operand_arity.max() == Some(0)
        {
            Vec::new()
        } else {
            vec![
                ValueGroupContract::new(
                    "operands",
                    operand_arity,
                    TypeConstraint::Any,
                )
                .expect("operand group must be valid"),
            ]
        };

        let result_groups = if result_arity.min() == 0
            && result_arity.max() == Some(0)
        {
            Vec::new()
        } else {
            vec![
                ValueGroupContract::new(
                    "results",
                    result_arity,
                    TypeConstraint::Any,
                )
                .expect("result group must be valid"),
            ]
        };

        let definition =
            operation_definition(operand_arity, result_arity);

        OperationContract::new(
            definition,
            operand_groups,
            result_groups,
            attributes,
            regions,
            BTreeSet::<OperationEffect>::new(),
            BTreeSet::new(),
        )
        .expect("test operation contract must be valid")
    }

    fn operation_id(raw: u128) -> HdlirOperationId {
        HdlirOperationId::try_from_raw(raw).expect("nonzero operation ID")
    }

    fn value_id(raw: u128) -> HdlirValueId {
        HdlirValueId::try_from_raw(raw).expect("nonzero value ID")
    }

    fn type_id(raw: u128) -> HdlirTypeId {
        HdlirTypeId::try_from_raw(raw).expect("nonzero type ID")
    }

    fn region_id(raw: u128) -> HdlirRegionId {
        HdlirRegionId::try_from_raw(raw).expect("nonzero region ID")
    }

    fn zero_arity() -> Arity {
        Arity::new(0, Some(0)).expect("zero arity must be valid")
    }

    fn exactly_one() -> Arity {
        Arity::new(1, Some(1)).expect("single-item arity must be valid")
    }

    #[test]
    fn builds_a_minimal_operation() {
        let schema = contract(
            zero_arity(),
            zero_arity(),
            Vec::new(),
            Vec::new(),
        );

        let operation = OperationBuilder::new(operation_id(1), &schema)
            .build()
            .expect("minimal operation should build");

        assert_eq!(operation.id(), operation_id(1));
        assert_eq!(operation.name().as_str(), "structural.core.noop");
        assert!(operation.operands().is_empty());
        assert!(operation.results().is_empty());
        assert!(operation.attributes().is_empty());
        assert!(operation.regions().is_empty());
    }

    #[test]
    fn rejects_missing_required_operands() {
        let schema = contract(
            exactly_one(),
            zero_arity(),
            Vec::new(),
            Vec::new(),
        );

        let result =
            OperationBuilder::new(operation_id(1), &schema).build();

        assert!(result.is_err());
    }

    #[test]
    fn accepts_an_operand_when_the_schema_requires_one() {
        let schema = contract(
            exactly_one(),
            zero_arity(),
            Vec::new(),
            Vec::new(),
        );

        let mut builder = OperationBuilder::new(operation_id(1), &schema);
        builder.add_operand(value_id(10));

        let operation = builder.build().expect("operand arity should pass");

        assert_eq!(operation.operands(), &[value_id(10)]);
    }

    #[test]
    fn rejects_undeclared_attributes() {
        let schema = contract(
            zero_arity(),
            zero_arity(),
            Vec::new(),
            Vec::new(),
        );

        let mut builder = OperationBuilder::new(operation_id(1), &schema);

        assert!(
            builder
                .add_attribute(
                    "undeclared",
                    OperationAttributeValue::Boolean(true),
                )
                .is_err()
        );
    }

    #[test]
    fn rejects_attribute_kind_mismatches() {
        let attribute = AttributeContract::new(
            "enabled",
            AttributeKind::Boolean,
            true,
            "Whether the feature is enabled",
        )
        .expect("attribute contract must be valid");

        let schema = contract(
            zero_arity(),
            zero_arity(),
            vec![attribute],
            Vec::new(),
        );

        let mut builder = OperationBuilder::new(operation_id(1), &schema);

        assert!(
            builder
                .add_attribute(
                    "enabled",
                    OperationAttributeValue::String("true".to_owned()),
                )
                .is_err()
        );

        // A rejected insertion must not mutate the builder.
        assert_eq!(builder.attribute_count(), 0);
    }

    #[test]
    fn rejects_missing_required_attributes() {
        let attribute = AttributeContract::new(
            "enabled",
            AttributeKind::Boolean,
            true,
            "Whether the feature is enabled",
        )
        .expect("attribute contract must be valid");

        let schema = contract(
            zero_arity(),
            zero_arity(),
            vec![attribute],
            Vec::new(),
        );

        assert!(
            OperationBuilder::new(operation_id(1), &schema)
                .build()
                .is_err()
        );
    }

    #[test]
    fn preserves_attributes_in_deterministic_key_order() {
        let schema = contract(
            zero_arity(),
            zero_arity(),
            vec![
                AttributeContract::new(
                    "alpha",
                    AttributeKind::Boolean,
                    true,
                    "First flag",
                )
                .expect("valid attribute"),
                AttributeContract::new(
                    "beta",
                    AttributeKind::Boolean,
                    true,
                    "Second flag",
                )
                .expect("valid attribute"),
            ],
            Vec::new(),
        );

        let mut builder = OperationBuilder::new(operation_id(1), &schema);
        builder
            .add_attribute("beta", OperationAttributeValue::Boolean(false))
            .expect("beta attribute should be accepted");
        builder
            .add_attribute("alpha", OperationAttributeValue::Boolean(true))
            .expect("alpha attribute should be accepted");

        let operation = builder.build().expect("attributes should validate");

        let names: Vec<&str> =
            operation.attributes().keys().map(String::as_str).collect();

        assert_eq!(names, vec!["alpha", "beta"]);
    }

    #[test]
    fn rejects_duplicate_result_ids() {
        let schema = contract(
            zero_arity(),
            Arity::new(2, Some(2)).expect("arity must be valid"),
            Vec::new(),
            Vec::new(),
        );

        let mut builder = OperationBuilder::new(operation_id(1), &schema);
        builder
            .add_results([
                OperationResult::new(value_id(10), type_id(1)),
                OperationResult::new(value_id(10), type_id(1)),
            ]);

        assert!(builder.build().is_err());
    }

    #[test]
    fn rejects_duplicate_region_references() {
        let region_contract = RegionContract::new(
            "body",
            Arity::new(2, Some(2)).expect("arity must be valid"),
            RegionShape::Graph,
            false,
            "Operation body regions",
        )
        .expect("region contract must be valid");

        let schema = contract(
            zero_arity(),
            zero_arity(),
            Vec::new(),
            vec![region_contract],
        );

        let mut builder = OperationBuilder::new(operation_id(1), &schema);
        builder.add_regions([region_id(7), region_id(7)]);

        assert!(builder.build().is_err());
    }

    #[test]
    fn rejects_region_count_outside_contract() {
        let region_contract = RegionContract::new(
            "body",
            exactly_one(),
            RegionShape::SingleBlock,
            false,
            "Operation body region",
        )
        .expect("region contract must be valid");

        let schema = contract(
            zero_arity(),
            zero_arity(),
            Vec::new(),
            vec![region_contract],
        );

        assert!(
            OperationBuilder::new(operation_id(1), &schema)
                .build()
                .is_err()
        );
    }

    #[test]
    fn permits_repeated_operand_references() {
        let schema = contract(
            Arity::new(2, Some(2)).expect("arity must be valid"),
            zero_arity(),
            Vec::new(),
            Vec::new(),
        );

        let mut builder = OperationBuilder::new(operation_id(1), &schema);
        builder.add_operands([value_id(9), value_id(9)]);

        let operation = builder
            .build()
            .expect("repeated operand references are permitted");

        assert_eq!(operation.operands(), &[value_id(9), value_id(9)]);
    }

    #[test]
    fn rejects_duplicate_attribute_names_without_replacing_values() {
        let attribute = AttributeContract::new(
            "enabled",
            AttributeKind::Boolean,
            true,
            "Whether the feature is enabled",
        )
        .expect("attribute contract must be valid");

        let schema = contract(
            zero_arity(),
            zero_arity(),
            vec![attribute],
            Vec::new(),
        );

        let mut builder = OperationBuilder::new(operation_id(1), &schema);

        builder
            .add_attribute("enabled", OperationAttributeValue::Boolean(true))
            .expect("first insertion should succeed");

        assert!(
            builder
                .add_attribute(
                    "enabled",
                    OperationAttributeValue::Boolean(false),
                )
                .is_err()
        );

        assert_eq!(builder.attribute_count(), 1);
    }
}
