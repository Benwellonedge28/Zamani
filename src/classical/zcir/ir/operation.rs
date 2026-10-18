
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Operation Integration
//!
//! Canonical path: `src/classical/zcir/ir/operation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.
//!
//! ## PURPOSE
//!
//! Expose ZUIR's canonical operation representation through the ZCIR API.
//! Classical operations are represented by registered qualified operation
//! names, ordered operands and results, attributes, nested-region references,
//! declared effects, and optional opaque extension payloads.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/operand.rs`
//! - `src/zuir/ir/result.rs`
//! - `src/zuir/ir/successor.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/effects.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/functions/functions.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## OWNERSHIP
//!
//! This file owns only the ZCIR-facing public names. The canonical ZUIR
//! implementation remains the sole owner of operation representation,
//! operation-local validation, mutation behavior, and operation errors.
//!
//! ZCIR does not introduce a competing operation struct, operation ID,
//! operand representation, result representation, effect set, attribute
//! map, nested-region representation, or extension-payload format.
//!
//! ## DOES NOT OWN
//!
//! - Source syntax, parsing, AST nodes, or semantic analysis.
//! - Dialect registration or the meaning of individual operation names.
//! - Global ID allocation or whole-program uniqueness.
//! - Whole-IR SSA, dominance, CFG, effect, or ownership verification.
//! - Optimization, physical resource allocation, scheduling, or execution.
//! - Target-specific operation encodings or hardware instructions.
//!
//! ## OPERATION-NAME CONTRACT
//!
//! A syntactically valid qualified name does not establish that the
//! operation is registered or legal. A trusted dialect schema must define
//! operand and result requirements, attributes, regions, effects, and
//! semantic behavior before the operation is trusted for optimization,
//! lowering, or execution.
//!
//! Unknown operations must not be silently reinterpreted or discarded.
//! Opaque extension payloads remain uninterpreted unless an applicable,
//! trusted extension contract authorizes their interpretation.
//!
//! ## VALIDATION CONTRACT
//!
//! The canonical operation's `validate()` method checks operation-local
//! invariants. It does not establish that referenced values, regions,
//! types, effects, or dialect operations exist in the enclosing program.
//!
//! The whole-IR verifier remains responsible for resolving references,
//! enforcing identity uniqueness, validating SSA and control flow, checking
//! types and effects, and consulting the applicable dialect definitions.
//!
//! Deserialization is not proof that an operation is valid in its enclosing
//! program. Decoded IR must pass local and whole-IR validation.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This adapter adds no universal limit on operation count, operand count,
//! result count, nested regions, register width, tensor rank, processors,
//! accelerators, devices, or distributed participants.
//!
//! Dynamic collection storage and fallible reservation belong to the
//! canonical operation implementation. Callers should use its `try_reserve`
//! methods when they need explicit reservation-failure handling.
//!
//! POCO-REAF does not promise literal infinity. Actual operation creation,
//! compilation, and execution remain subject to address space, available
//! resources, target capabilities, operational budgets, and implementation
//! limits. Those limits must be explicit rather than silently changing
//! program meaning.
//!
//! ## INTEGRATION
//!
//! - `src/classical/zcir/ir/block.rs` stores canonical `OperationId`
//!   references to preserve operation ordering without copying operations.
//! - `src/classical/zcir/ir/region.rs` must reuse canonical `RegionId`
//!   values and the existing ZUIR region representation.
//! - `src/classical/zcir/ir/value.rs` must reuse canonical `ValueId` and
//!   `TypeId` contracts rather than creating duplicate identities.
//! - `src/classical/zcir/verify/` must perform whole-IR and dialect-aware
//!   validation in addition to the local validation exposed here.
//! - `src/classical/zcir/lowering/` must preserve operation semantics,
//!   operand/result order, effects, attributes, and nested-region meaning.
//! - `src/quantum/ir/` must preserve its existing contracts when classical
//!   operations participate in hybrid computation.
//!
//! The ZCIR IR module should re-export these aliases only after this file
//! exists and its parent module has been established.
//!
//! ## FREEZE CRITERIA
//!
//! This adapter can be frozen when its public exports are registered,
//! canonical behavior tests pass, the full repository compiles, and
//! downstream verification/lowering contracts are integrated and tested.
//! Passing these local tests alone does not certify the entire ZCIR
//! subsystem or every target for POCO-REAF.

/// Canonical operation representation and operation-local error types.
pub use crate::zuir::ir::operation::{
    OpaqueOperationPayload,
    OperationError,
    OperationResultType,
    ZuirOperation,
};

/// Canonical operand representation and operand-local errors.
pub use crate::zuir::ir::operand::{
    OperandError,
    OperandResult,
    OperationOperand,
};

/// Canonical typed result representation.
pub use crate::zuir::ir::result::OperationValueResult;

/// ZCIR-facing name for the canonical ZUIR operation.
///
/// This is an alias, not a second operation model.
pub type ZcirOperation = ZuirOperation;

/// ZCIR-facing name for the canonical operation error.
pub type ZcirOperationError = OperationError;

/// ZCIR-facing name for the canonical operation result type.
pub type ZcirOperationResult<T> = OperationResultType<T>;

/// ZCIR-facing name for the canonical operand.
pub type ZcirOperand = OperationOperand;

/// ZCIR-facing name for the canonical operand error.
pub type ZcirOperandError = OperandError;

/// ZCIR-facing name for the canonical operand construction result.
pub type ZcirOperandResult<T> = OperandResult<T>;

/// ZCIR-facing name for a typed operation result.
pub type ZcirOperationValueResult = OperationValueResult;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{OperationId, RegionId, TypeId, ValueId};
    use crate::zuir::schema::types::QualifiedName;

    fn operation_id(raw: u128) -> OperationId {
        OperationId::new(raw).expect("test operation ID must be nonzero")
    }

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn type_id(raw: u128) -> TypeId {
        TypeId::new(raw).expect("test type ID must be nonzero")
    }

    fn region_id(raw: u128) -> RegionId {
        RegionId::new(raw).expect("test region ID must be nonzero")
    }

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test qualified name must be valid")
    }

    fn operation() -> ZcirOperation {
        ZcirOperation::new(operation_id(1), name("classical::add"))
            .expect("valid operation should be constructed")
    }

    #[test]
    fn zcir_operation_is_the_canonical_zuir_operation_type() {
        fn accepts_zcir(_: ZcirOperation) {}
        fn accepts_zuir(_: ZuirOperation) {}

        let op = operation();

        accepts_zcir(op.clone());
        accepts_zuir(op.clone());

        assert_eq!(
            core::any::type_name::<ZcirOperation>(),
            core::any::type_name::<ZuirOperation>(),
        );
        assert_eq!(op.id(), operation_id(1));
        assert_eq!(op.name(), &name("classical::add"));
    }

    #[test]
    fn new_operation_has_empty_collections_and_valid_local_structure() {
        let op = operation();

        assert!(op.operands().is_empty());
        assert!(op.results().is_empty());
        assert!(op.regions().is_empty());
        assert!(op.effects().is_empty());
        assert!(op.attributes().is_empty());
        assert!(op.validate().is_ok());
    }

    #[test]
    fn repeated_operand_values_are_preserved_in_order() {
        let mut op = operation();
        let operand = ZcirOperand::new(value_id(10));

        op.add_operand(operand.clone())
            .expect("first operand should be accepted");

        op.add_operand(operand)
            .expect("repeated operand values are permitted");

        assert_eq!(op.operand_count(), 2);
        assert_eq!(op.operands()[0].value(), value_id(10));
        assert_eq!(op.operands()[1].value(), value_id(10));
        assert!(op.validate().is_ok());
    }

    #[test]
    fn result_values_must_be_unique_within_an_operation() {
        let mut op = operation();

        op.add_result(ZcirOperationValueResult::new(
            value_id(20),
            type_id(1),
        ))
        .expect("first result should be accepted");

        let error = op
            .add_result(ZcirOperationValueResult::new(
                value_id(20),
                type_id(2),
            ))
            .expect_err("duplicate result value must be rejected");

        assert_eq!(error, ZcirOperationError::DuplicateResult(value_id(20)));
        assert_eq!(op.result_count(), 1);
        assert_eq!(op.results()[0].ty(), type_id(1));
    }

    #[test]
    fn region_references_must_be_unique_within_an_operation() {
        let mut op = operation();
        let region = region_id(30);

        op.add_region(region)
            .expect("first region reference should be accepted");

        let error = op
            .add_region(region)
            .expect_err("duplicate region reference must be rejected");

        assert_eq!(error, ZcirOperationError::DuplicateRegion(region));
        assert_eq!(op.regions(), &[region]);
    }

    #[test]
    fn operation_name_change_is_validated() {
        let mut op = operation();

        op.set_name(name("classical::subtract"))
            .expect("valid qualified operation name should be accepted");

        assert_eq!(op.name(), &name("classical::subtract"));
        assert!(op.validate().is_ok());
    }

    #[test]
    fn canonical_validation_remains_available_through_zcir() {
        let op = operation();

        // This verifies operation-local invariants only. It does not replace
        // verification of references or dialect legality in the enclosing IR.
        assert!(op.validate().is_ok());
    }
}
