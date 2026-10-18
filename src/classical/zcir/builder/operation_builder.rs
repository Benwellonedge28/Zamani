
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Operation Builder
//!
//! Canonical path: `src/classical/zcir/builder/operation_builder.rs`
//! Minimum Rust version: 1.97+
//!
//! ## Purpose
//!
//! Construct canonical ZUIR operations for classical-domain compilation.
//! ZCIR is an integration facade over ZUIR, not an independent operation
//! representation.
//!
//! ## Architectural authority
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/operand.rs`
//! - `src/zuir/ir/result.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/effects.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! ## Ownership
//!
//! ZUIR owns the operation representation, typed identifiers, ordered
//! operands and results, attributes, effects, region references, extension
//! payloads, and operation-local validation.
//!
//! This file owns only the builder API and its construction lifecycle.
//!
//! ## Does not own
//!
//! - Source parsing, AST construction, or semantic analysis.
//! - Type inference or global type compatibility.
//! - ID allocation or global identity uniqueness.
//! - Dialect registration or operation execution.
//! - SSA, dominance, alias, ownership, or whole-program verification.
//! - Resource allocation, scheduling, target selection, or lowering.
//! - A second operation, operand, result, or effect representation.
//!
//! ## Scalability and POCO-REAF
//!
//! This builder introduces no arbitrary maximum operand, result, region,
//! effect, attribute, or operation count. It exposes fallible reservation
//! methods backed by the canonical operation API.
//!
//! Successful construction does not promise infinite memory or processing.
//! Finite identity space, available resources, configured operational
//! budgets, and target capabilities remain explicit constraints.
//!
//! ## Semantic contract
//!
//! Operand and result order is preserved. Repeated operand references are
//! allowed. Result value IDs and nested region IDs must be unique within
//! the operation according to the canonical ZUIR contract.
//!
//! An operation name must be a valid QualifiedName. A valid name does not
//! prove that its operation is registered or that its signature is legal.
//! Unknown operations must not be silently executed, optimized, or lowered.
//!
//! ## Integration
//!
//! `builder/mod.rs` should register this file and re-export
//! `ZcirOperationBuilder` if that public API is desired.
//!
//! The builder returns `crate::zuir::ir::operation::ZuirOperation`,
//! which is the canonical representation. Blocks store its canonical
//! `OperationId`, and regions/functions refer to canonical ZUIR entities.
//!
//! Before optimization, trusted serialization, lowering, or execution,
//! run the authoritative full-IR verifier and the relevant dialect checks.
//!
//! ## Freeze criteria
//!
//! Freeze this file only when its API is checked against the repository's
//! actual canonical types, the parent modules are registered, and formatting,
//! compilation, tests, and linting pass.

use crate::zuir::ids::{OperationId, RegionId};
use crate::zuir::ir::operand::OperationOperand;
use crate::zuir::ir::operation::{
    OpaqueOperationPayload,
    OperationError,
    ZuirOperation,
};
use crate::zuir::ir::result::OperationValueResult;
use crate::zuir::schema::attributes::{
    AttributeKey,
    AttributeValue,
};
use crate::zuir::schema::types::QualifiedName;

/// Canonical operation produced by the ZCIR builder.
///
/// This alias does not create a second operation representation.
pub type BuiltZcirOperation = ZuirOperation;

/// Builder for one canonical, target-independent operation.
///
/// The builder owns the operation under construction. Every mutating method
/// delegates validation to the canonical ZUIR API. `build` consumes the
/// builder and performs a final operation-local validation.
#[derive(Debug, Clone)]
pub struct ZcirOperationBuilder {
    operation: ZuirOperation,
}

impl ZcirOperationBuilder {
    /// Start an operation with a canonical identity and qualified name.
    ///
    /// This validates the name's structure. It does not allocate the ID,
    /// check global uniqueness, or establish dialect registration.
    pub fn new(
        id: OperationId,
        name: QualifiedName,
    ) -> Result<Self, OperationError> {
        let operation = ZuirOperation::new(id, name)?;
        Ok(Self { operation })
    }

    /// Start an operation with its ordered operands and results.
    ///
    /// Both collections are checked by the canonical constructor.
    pub fn with_signature(
        id: OperationId,
        name: QualifiedName,
        operands: Vec<OperationOperand>,
        results: Vec<OperationValueResult>,
    ) -> Result<Self, OperationError> {
        let operation =
            ZuirOperation::with_signature(id, name, operands, results)?;

        Ok(Self { operation })
    }

    /// Return the operation identity.
    #[must_use]
    pub const fn id(&self) -> OperationId {
        self.operation.id()
    }

    /// Borrow the operation's qualified name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        self.operation.name()
    }

    /// Borrow the canonical operation without transferring ownership.
    #[must_use]
    pub const fn as_operation(&self) -> &ZuirOperation {
        &self.operation
    }

    /// Return the number of operands.
    #[must_use]
    pub fn operand_count(&self) -> usize {
        self.operation.operand_count()
    }

    /// Return the number of results.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.operation.result_count()
    }

    /// Return the number of nested-region references.
    #[must_use]
    pub fn region_count(&self) -> usize {
        self.operation.region_count()
    }

    /// Reserve additional operand capacity.
    ///
    /// Reservation does not change the logical operand sequence.
    pub fn try_reserve_operands(
        &mut self,
        additional: usize,
    ) -> Result<(), OperationError> {
        self.operation.try_reserve_operands(additional)
    }

    /// Reserve additional result capacity.
    pub fn try_reserve_results(
        &mut self,
        additional: usize,
    ) -> Result<(), OperationError> {
        self.operation.try_reserve_results(additional)
    }

    /// Reserve additional nested-region reference capacity.
    pub fn try_reserve_regions(
        &mut self,
        additional: usize,
    ) -> Result<(), OperationError> {
        self.operation.try_reserve_regions(additional)
    }

    /// Append one operand while preserving operand order.
    ///
    /// Repeated value references are legal and are not deduplicated.
    pub fn add_operand(
        &mut self,
        operand: OperationOperand,
    ) -> Result<&mut Self, OperationError> {
        self.operation.add_operand(operand)?;
        Ok(self)
    }

    /// Replace all operands after canonical validation.
    ///
    /// The canonical operation retains its previous operands if validation
    /// fails.
    pub fn set_operands(
        &mut self,
        operands: Vec<OperationOperand>,
    ) -> Result<&mut Self, OperationError> {
        self.operation.set_operands(operands)?;
        Ok(self)
    }

    /// Append a typed result.
    ///
    /// The canonical API rejects a result whose value ID duplicates an
    /// existing result in this operation. Program-wide uniqueness remains
    /// the responsibility of the enclosing verifier.
    pub fn add_result(
        &mut self,
        result: OperationValueResult,
    ) -> Result<&mut Self, OperationError> {
        self.operation.add_result(result)?;
        Ok(self)
    }

    /// Replace all results after canonical validation.
    pub fn set_results(
        &mut self,
        results: Vec<OperationValueResult>,
    ) -> Result<&mut Self, OperationError> {
        self.operation.set_results(results)?;
        Ok(self)
    }

    /// Attach an attribute through the canonical attribute API.
    ///
    /// Returns the previous value when an attribute with the same key
    /// existed. Namespace authorization and operation-specific attribute
    /// legality must be checked by dialect verification.
    pub fn set_attribute(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> Result<Option<AttributeValue>, OperationError> {
        self.operation.set_attribute(key, value)
    }

    /// Remove an attribute by its complete qualified key.
    pub fn remove_attribute(
        &mut self,
        key: &AttributeKey,
    ) -> Option<AttributeValue> {
        self.operation.remove_attribute(key)
    }

    /// Attach a nested-region reference.
    ///
    /// This records a reference; it does not construct the region or prove
    /// that it exists or belongs to this operation.
    pub fn add_region(
        &mut self,
        region: RegionId,
    ) -> Result<&mut Self, OperationError> {
        self.operation.add_region(region)?;
        Ok(self)
    }

    /// Replace nested-region references after canonical validation.
    pub fn set_regions(
        &mut self,
        regions: Vec<RegionId>,
    ) -> Result<&mut Self, OperationError> {
        self.operation.set_regions(regions)?;
        Ok(self)
    }

    /// Declare a semantic effect by its qualified name.
    ///
    /// Returns `true` when the effect was newly added, or `false` if it
    /// was already declared. Effects are semantic declarations, not target
    /// capabilities or physical resource allocations.
    pub fn add_effect(
        &mut self,
        effect: QualifiedName,
    ) -> Result<bool, OperationError> {
        self.operation.add_effect(effect)
    }

    /// Replace the complete effect set after canonical validation.
    pub fn set_effects(
        &mut self,
        effects: std::collections::BTreeSet<QualifiedName>,
    ) -> Result<&mut Self, OperationError> {
        self.operation.set_effects(effects)?;
        Ok(self)
    }

    /// Set or clear an opaque extension payload.
    ///
    /// The payload remains uninterpreted unless a trusted extension
    /// contract defines its meaning.
    pub fn set_opaque_payload(
        &mut self,
        payload: Option<OpaqueOperationPayload>,
    ) -> Result<&mut Self, OperationError> {
        self.operation.set_opaque_payload(payload)?;
        Ok(self)
    }

    /// Validate operation-local invariants without consuming the builder.
    ///
    /// This does not perform dialect, type, SSA, dominance, region-ownership,
    /// effect-policy, or whole-program verification.
    pub fn validate(&self) -> Result<(), OperationError> {
        self.operation.validate()
    }

    /// Finish construction and return the canonical ZUIR operation.
    ///
    /// The caller must still insert the operation into its enclosing IR
    /// and run the applicable full-IR and dialect verifiers.
    pub fn build(self) -> Result<BuiltZcirOperation, OperationError> {
        self.operation.validate()?;
        Ok(self.operation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{TypeId, ValueId};

    fn operation_id(raw: u128) -> OperationId {
        OperationId::new(raw).expect("test ID must be nonzero")
    }

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test ID must be nonzero")
    }

    fn type_id(raw: u128) -> TypeId {
        TypeId::new(raw).expect("test ID must be nonzero")
    }

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test qualified name must be valid")
    }

    fn builder() -> ZcirOperationBuilder {
        ZcirOperationBuilder::new(operation_id(1), name("classical::add"))
            .expect("valid operation")
    }

    #[test]
    fn creates_canonical_operation() {
        let builder = builder();

        assert_eq!(builder.id(), operation_id(1));
        assert_eq!(builder.name(), &name("classical::add"));
        assert_eq!(builder.operand_count(), 0);
        assert_eq!(builder.result_count(), 0);
        assert_eq!(builder.region_count(), 0);

        let operation = builder.build().expect("valid operation");
        assert!(operation.validate().is_ok());
    }

    #[test]
    fn preserves_operand_order_and_repeated_references() {
        let mut builder = builder();

        builder
            .add_operand(OperationOperand::new(value_id(10)))
            .expect("first operand")
            .add_operand(OperationOperand::new(value_id(10)))
            .expect("repeated operand")
            .add_operand(OperationOperand::new(value_id(20)))
            .expect("third operand");

        let operation = builder.build().expect("valid operation");

        assert_eq!(operation.operand_count(), 3);
        assert_eq!(operation.operands()[0].value(), value_id(10));
        assert_eq!(operation.operands()[1].value(), value_id(10));
        assert_eq!(operation.operands()[2].value(), value_id(20));
    }

    #[test]
    fn accepts_distinct_typed_results() {
        let mut builder = builder();

        builder
            .add_result(OperationValueResult::new(
                value_id(100),
                type_id(1),
            ))
            .expect("first result")
            .add_result(OperationValueResult::new(
                value_id(200),
                type_id(1),
            ))
            .expect("second result");

        let operation = builder.build().expect("valid operation");

        assert_eq!(operation.result_count(), 2);
        assert_eq!(operation.results()[0].value(), value_id(100));
        assert_eq!(operation.results()[1].value(), value_id(200));
    }

    #[test]
    fn rejects_duplicate_result_value_ids() {
        let mut builder = builder();

        builder
            .add_result(OperationValueResult::new(
                value_id(100),
                type_id(1),
            ))
            .expect("first result");

        let result = builder.add_result(OperationValueResult::new(
            value_id(100),
            type_id(2),
        ));

        assert!(result.is_err());
        assert_eq!(builder.result_count(), 1);
    }

    #[test]
    fn rejects_duplicate_region_references() {
        let mut builder = builder();
        let region = RegionId::new(9).expect("test region ID must be nonzero");

        builder.add_region(region).expect("first region");

        assert!(builder.add_region(region).is_err());
        assert_eq!(builder.region_count(), 1);
    }

    #[test]
    fn reserves_capacity_without_changing_logical_contents() {
        let mut builder = builder();

        builder.try_reserve_operands(32).expect("reserve operands");
        builder.try_reserve_results(32).expect("reserve results");
        builder.try_reserve_regions(16).expect("reserve regions");

        assert_eq!(builder.operand_count(), 0);
        assert_eq!(builder.result_count(), 0);
        assert_eq!(builder.region_count(), 0);
    }

    #[test]
    fn effect_declarations_are_idempotent() {
        let mut builder = builder();

        assert!(
            builder
                .add_effect(name("classical::memory_read"))
                .expect("valid effect")
        );

        assert!(
            !builder
                .add_effect(name("classical::memory_read"))
                .expect("duplicate declaration is valid")
        );

        let operation = builder.build().expect("valid operation");

        assert_eq!(operation.effects().len(), 1);
        assert!(operation.effects().contains(&name("classical::memory_read")));
    }

    #[test]
    fn output_is_the_canonical_zuir_operation() {
        let operation = builder().build().expect("valid operation");

        fn accepts_zuir(_: ZuirOperation) {}

        accepts_zuir(operation);
    }
}
