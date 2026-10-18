
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Conformance Tests
//!
//! Canonical path: `src/classical/zcir/tests/conformance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety policy: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Exercise the public conformance contract of the classical-facing ZCIR
//! boundary. These tests verify that ZCIR reuses canonical ZUIR entities,
//! preserves identity and representation, rejects invalid local mutations,
//! and remains independent of arbitrary hardware-size assumptions.
//!
//! ## Architectural authority
//!
//! - `src/classical/zcir/ir/mod.rs`
//! - `src/classical/zcir/verify/mod.rs`
//! - `src/classical/zcir/lowering/to_zuir.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/verify/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! ## Ownership
//!
//! This file owns only conformance tests. It does not define production
//! IR entities, verification algorithms, source-language semantics,
//! hardware abstractions, or execution behavior.
//!
//! ## Test scope
//!
//! The suite covers:
//!
//! - Canonical type and identity reuse.
//! - Public verifier and adapter API compatibility.
//! - Local block invariants and duplicate rejection.
//! - Collection growth under an explicit test workload.
//! - Deterministic serialization and round-trip preservation.
//! - Rejection of malformed qualified names.
//! - The distinction between local validation and full verification.
//!
//! Grammar parsing, frontend AST construction, semantic analysis,
//! cross-target compilation, and runtime execution belong to their
//! respective integration and conformance suites.
//!
//! ## Scalability policy
//!
//! A finite test workload is an operational test budget, not a universal
//! limit on program size, operation count, hardware, memory, or execution.
//! No test establishes literal infinite capacity.
//!
//! ## Integration
//!
//! Register this file from the valid ZCIR root or from
//! `src/classical/zcir/tests/mod.rs`. It must not define duplicate IR types.
//!
//! Run the repository's format, check, test, and Clippy gates after
//! registering the module.

use crate::classical::zcir::ir;
use crate::classical::zcir::lowering::to_zuir;
use crate::classical::zcir::verify;
use crate::zuir;

/// Assert that two optional public API types are identical.
///
/// This is a compile-time conformance check: assigning one optional value
/// to the other requires both types to be the same Rust type.
macro_rules! assert_same_type {
    ($canonical:ty, $facade:ty) => {{
        let canonical: Option<$canonical> = None;
        let facade: Option<$facade> = canonical;
        assert!(facade.is_none());
    }};
}

#[test]
fn zcir_block_is_the_canonical_zuir_block_type() {
    assert_same_type!(
        zuir::ir::ZuirBlock,
        ir::ZuirBlock
    );

    assert_same_type!(
        zuir::ids::BlockId,
        ir::BlockId
    );
}

#[test]
fn zcir_operation_and_value_identities_are_canonical() {
    assert_same_type!(
        zuir::ids::OperationId,
        ir::OperationId
    );

    assert_same_type!(
        zuir::ids::ValueId,
        ir::ValueId
    );

    assert_same_type!(
        zuir::ids::FunctionId,
        ir::FunctionId
    );

    assert_same_type!(
        zuir::ids::RegionId,
        ir::RegionId
    );

    assert_same_type!(
        zuir::ids::ModuleId,
        ir::ModuleId
    );

    assert_same_type!(
        zuir::ids::ProgramId,
        ir::ProgramId
    );
}

#[test]
fn zcir_facade_uses_canonical_full_verification_types() {
    assert_same_type!(
        zuir::verify::FullVerificationInput<'static>,
        verify::FullVerificationInput<'static>
    );

    assert_same_type!(
        zuir::verify::FullVerificationReport,
        verify::FullVerificationReport
    );

    assert_same_type!(
        zuir::verify::StructuralInput<'static>,
        verify::StructuralInput<'static>
    );

    assert_same_type!(
        zuir::verify::TypeVerificationInput<'static>,
        verify::TypeVerificationInput<'static>
    );
}

#[test]
fn zcir_verification_entry_points_match_the_canonical_api() {
    let _: fn(
        &verify::FullVerificationInput<'_>,
    ) -> verify::FullVerificationReport = verify::verify_full;

    let _: fn(
        &verify::StructuralInput<'_>,
    ) -> zuir::verify::structural::StructuralReport =
        verify::verify_structural;

    let _: fn(
        &verify::ControlFlowInput<'_>,
    ) -> zuir::verify::control_flow::ControlFlowReport =
        verify::verify_control_flow;

    let _: fn(
        &verify::DominanceInput<'_>,
    ) -> zuir::verify::dominance::DominanceReport =
        verify::verify_dominance;

    let _: fn(
        &verify::TypeVerificationInput<'_>,
    ) -> zuir::verify::types::TypeVerificationReport =
        verify::verify_types;

    let _: fn(
        &verify::EffectVerificationInput<'_>,
    ) -> zuir::verify::effects::EffectVerificationReport =
        verify::verify_effects;

    let _: fn(
        &verify::OwnershipVerificationInput<'_>,
    ) -> zuir::verify::ownership::OwnershipVerificationReport =
        verify::verify_ownership;

    let _: fn(
        &[verify::ResourceIntentEntry<'_>],
    ) -> zuir::verify::resources::ResourceVerificationReport =
        verify::verify_resources;
}

#[test]
fn newly_constructed_block_preserves_its_canonical_identity() {
    let id = zuir::ids::BlockId::new(1)
        .expect("nonzero test block identity");

    let block = ir::ZuirBlock::new(id);

    assert_eq!(block.id(), id);
    assert_eq!(block.operation_count(), 0);
    assert_eq!(block.argument_count(), 0);
    assert!(block.operations().is_empty());

    block
        .validate()
        .expect("a new empty block must satisfy local invariants");
}

#[test]
fn identity_adapter_preserves_the_original_block() {
    let id = zuir::ids::BlockId::new(2)
        .expect("nonzero test block identity");

    let block = ir::ZuirBlock::new(id);
    let converted = to_zuir::to_zuir(block);

    assert_eq!(converted.id(), id);
    assert_eq!(converted.operation_count(), 0);
    assert_eq!(converted.argument_count(), 0);

    converted
        .validate()
        .expect("identity conversion must preserve local validity");
}

#[test]
fn borrowed_identity_adapter_does_not_replace_the_entity() {
    let id = zuir::ids::BlockId::new(3)
        .expect("nonzero test block identity");

    let block = ir::ZuirBlock::new(id);
    let borrowed = to_zuir::as_zuir(&block);

    assert_eq!(borrowed.id(), id);
    assert_eq!(block.id(), id);
}

#[test]
fn mutable_identity_adapter_preserves_the_entity_identity() {
    let id = zuir::ids::BlockId::new(4)
        .expect("nonzero test block identity");

    let mut block = ir::ZuirBlock::new(id);

    {
        let borrowed = to_zuir::as_zuir_mut(&mut block);
        assert_eq!(borrowed.id(), id);
    }

    assert_eq!(block.id(), id);

    block
        .validate()
        .expect("identity-preserving access must not invalidate the block");
}

#[test]
fn duplicate_operation_references_are_rejected() {
    let block_id = zuir::ids::BlockId::new(5)
        .expect("nonzero test block identity");

    let operation_id = zuir::ids::OperationId::new(10)
        .expect("nonzero test operation identity");

    let mut block = ir::ZuirBlock::new(block_id);

    block
        .insert_operation(0, operation_id)
        .expect("the first operation reference must be accepted");

    let duplicate = block.insert_operation(1, operation_id);

    assert!(
        duplicate.is_err(),
        "a block must not contain duplicate operation identities"
    );

    assert_eq!(block.operation_count(), 1);
    assert_eq!(block.operations(), &[operation_id]);
}

#[test]
fn invalid_operation_insertion_position_is_rejected() {
    let block_id = zuir::ids::BlockId::new(6)
        .expect("nonzero test block identity");

    let operation_id = zuir::ids::OperationId::new(11)
        .expect("nonzero test operation identity");

    let mut block = ir::ZuirBlock::new(block_id);

    let result = block.insert_operation(1, operation_id);

    assert!(
        result.is_err(),
        "insertion beyond the current operation count must fail"
    );

    assert_eq!(block.operation_count(), 0);
    assert!(block.operations().is_empty());
}

#[test]
fn invalid_module_names_are_rejected() {
    use zuir::ir::module::ModuleName;

    let invalid_names = [
        "",
        " leading",
        "trailing ",
        "two words",
        "module/name",
        "module..nested",
    ];

    for candidate in invalid_names {
        assert!(
            ModuleName::new(candidate).is_err(),
            "invalid module name was accepted: {candidate:?}"
        );
    }

    assert!(
        ModuleName::new("classical_numeric")
            .is_ok(),
        "a valid module name must be accepted"
    );
}

#[test]
fn invalid_dialect_names_are_rejected() {
    use zuir::ir::program::DialectName;

    let invalid_names = [
        "",
        " Classical",
        "Classical",
        "classical.",
        "classical..numeric",
        "classical/numeric",
    ];

    for candidate in invalid_names {
        assert!(
            DialectName::new(candidate).is_err(),
            "invalid dialect name was accepted: {candidate:?}"
        );
    }

    assert!(
        DialectName::new("classical.numeric").is_ok(),
        "a valid qualified dialect name must be accepted"
    );
}

#[test]
fn block_serialization_is_deterministic() {
    let block_id = zuir::ids::BlockId::new(7)
        .expect("nonzero test block identity");

    let operation_id = zuir::ids::OperationId::new(12)
        .expect("nonzero test operation identity");

    let mut block = ir::ZuirBlock::new(block_id);

    block
        .insert_operation(0, operation_id)
        .expect("test operation reference must be accepted");

    let first = serde_json::to_string(&block)
        .expect("block serialization must succeed");

    let second = serde_json::to_string(&block)
        .expect("repeated block serialization must succeed");

    assert_eq!(
        first, second,
        "unchanged IR must serialize deterministically"
    );
}

#[test]
fn block_serialization_round_trip_preserves_identity_and_contents() {
    let block_id = zuir::ids::BlockId::new(8)
        .expect("nonzero test block identity");

    let operation_ids = [
        zuir::ids::OperationId::new(20)
            .expect("nonzero operation identity"),
        zuir::ids::OperationId::new(21)
            .expect("nonzero operation identity"),
        zuir::ids::OperationId::new(22)
            .expect("nonzero operation identity"),
    ];

    let mut original = ir::ZuirBlock::new(block_id);

    for (position, operation_id) in operation_ids.iter().copied().enumerate() {
        original
            .insert_operation(position, operation_id)
            .expect("unique operation reference must be accepted");
    }

    let encoded = serde_json::to_string(&original)
        .expect("block serialization must succeed");

    let decoded: ir::ZuirBlock = serde_json::from_str(&encoded)
        .expect("valid serialized block must deserialize");

    assert_eq!(decoded.id(), original.id());
    assert_eq!(decoded.operations(), original.operations());
    assert_eq!(decoded.operation_count(), original.operation_count());
    assert_eq!(decoded.argument_count(), original.argument_count());

    decoded
        .validate()
        .expect("round-tripped block must satisfy local invariants");
}

#[test]
fn collection_growth_has_no_small_fixed_semantic_ceiling() {
    // This is a deliberately bounded regression workload. Its size is
    // chosen for test practicality and is not a ZCIR operation limit.
    const TEST_OPERATION_BUDGET: usize = 2_048;

    let block_id = zuir::ids::BlockId::new(9)
        .expect("nonzero test block identity");

    let mut block = ir::ZuirBlock::new(block_id);

    block
        .try_reserve_operations(TEST_OPERATION_BUDGET)
        .expect("test workload reservation should succeed");

    for index in 0..TEST_OPERATION_BUDGET {
        let raw_id = (index as u128) + 1;

        let operation_id = zuir::ids::OperationId::new(raw_id)
            .expect("generated operation identity must be nonzero");

        block
            .insert_operation(index, operation_id)
            .expect("unique operation reference must be accepted");
    }

    assert_eq!(block.operation_count(), TEST_OPERATION_BUDGET);

    let mut seen = std::collections::BTreeSet::new();

    for operation_id in block.operations() {
        assert!(
            seen.insert(*operation_id),
            "operation identities must remain unique"
        );
    }

    assert_eq!(seen.len(), TEST_OPERATION_BUDGET);

    block
        .validate()
        .expect("the resulting block must satisfy local invariants");
}

#[test]
fn zero_workload_does_not_require_a_special_hardware_assumption() {
    let block_id = zuir::ids::BlockId::new(10)
        .expect("nonzero test block identity");

    let block = ir::ZuirBlock::new(block_id);

    assert!(block.operations().is_empty());
    assert_eq!(block.operation_count(), 0);
    assert_eq!(block.argument_count(), 0);

    block
        .validate()
        .expect("an empty construction-state block is locally valid");
}

#[test]
fn verification_reports_are_not_replaced_by_boolean_shortcuts() {
    // A clean local block check is not a substitute for whole-program
    // verification. This test pins the report-based public API so callers
    // can distinguish validity, incompleteness, and invalidity.
    let _: fn(
        &verify::FullVerificationInput<'_>,
    ) -> verify::FullVerificationReport = verify::verify_full;

    let _: fn(
        &verify::StructuralInput<'_>,
    ) -> zuir::verify::structural::StructuralReport =
        verify::verify_structural;

    let _: fn(
        &verify::TypeVerificationInput<'_>,
    ) -> zuir::verify::types::TypeVerificationReport =
        verify::verify_types;
}
