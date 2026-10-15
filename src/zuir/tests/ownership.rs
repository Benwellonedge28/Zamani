//! ZUIR ownership contract and index conformance tests.
//!
//! Path: src/zuir/tests/ownership.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Test the public ownership-contract and ownership-index APIs defined by
//! `crate::zuir::ir::ownership`.
//!
//! OWNS
//!
//! Regression and conformance tests for ownership kinds, capabilities,
//! contract validation, ownership-use recording, deterministic indexing,
//! error behavior, and dynamic collection growth.
//!
//! DOES NOT OWN
//!
//! This file does not implement ownership semantics, define another ownership
//! model, or replace the production ownership verifier.
//!
//! IMPORTANT VERIFICATION BOUNDARY
//!
//! These tests establish local contract and index behavior only. They do not
//! prove linear consumption on every control-flow path, borrow exclusivity,
//! lifetime validity, region-capture legality, SSA correctness, or
//! interprocedural ownership correctness.
//!
//! INTEGRATION
//!
//! This file requires the owning test module to be registered by the actual
//! ZUIR module hierarchy. It imports the canonical production types directly.
//!
//! SCALABILITY
//!
//! Test workloads use dynamically sized collections. The workload size below
//! is a test parameter, not a ZUIR, language, or hardware limit.
//!
//! FREEZE CRITERIA
//!
//! - Typed identifiers are constructed through their validated public API.
//! - Ownership contracts reject contradictory capabilities.
//! - Invalid index operations do not corrupt existing index state.
//! - Use ordering is deterministic.
//! - Extension-defined ownership names remain extensible.
//! - Tests do not mistake local indexing for whole-program verification.
//! - The test module compiles and passes with the repository's actual module
//!   registration and dependency configuration.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::zuir::ids::{BlockId, OperationId, RegionId, ValueId};
use crate::zuir::ir::ownership::{
    BorrowMutability, OwnershipCapabilities, OwnershipContract, OwnershipError,
    OwnershipIndex, OwnershipKind, OwnershipUse, OwnershipUseKind,
};
use crate::zuir::schema::types::QualifiedName;

// -----------------------------------------------------------------------------
// Test helpers
// -----------------------------------------------------------------------------
//
// Use the canonical typed-ID constructors. Do not construct IDs by accessing
// their internal representation or by defining test-only ID types.

fn value_id(raw: u128) -> ValueId {
    ValueId::new(raw).expect("test value ID must be nonzero")
}

fn block_id(raw: u128) -> BlockId {
    BlockId::new(raw).expect("test block ID must be nonzero")
}

fn operation_id(raw: u128) -> OperationId {
    OperationId::new(raw).expect("test operation ID must be nonzero")
}

fn region_id(raw: u128) -> RegionId {
    RegionId::new(raw).expect("test region ID must be nonzero")
}

fn qualified_name(name: &str) -> QualifiedName {
    QualifiedName::parse(name).expect("test qualified name must be valid")
}

fn unrestricted_contract(value: ValueId) -> OwnershipContract {
    OwnershipContract::new(
        value,
        OwnershipKind::Unrestricted,
        OwnershipCapabilities::unrestricted(),
        None,
    )
    .expect("unrestricted ownership contract must be valid")
}

fn copy_contract(value: ValueId) -> OwnershipContract {
    OwnershipContract::new(
        value,
        OwnershipKind::Copy,
        OwnershipCapabilities::copy(),
        None,
    )
    .expect("copy ownership contract must be valid")
}

fn move_only_contract(value: ValueId) -> OwnershipContract {
    OwnershipContract::new(
        value,
        OwnershipKind::MoveOnly,
        OwnershipCapabilities::move_only(),
        None,
    )
    .expect("move-only ownership contract must be valid")
}

fn linear_contract(value: ValueId) -> OwnershipContract {
    OwnershipContract::new(
        value,
        OwnershipKind::Linear,
        OwnershipCapabilities::linear(),
        None,
    )
    .expect("linear ownership contract must be valid")
}

fn affine_contract(value: ValueId) -> OwnershipContract {
    OwnershipContract::new(
        value,
        OwnershipKind::Affine,
        OwnershipCapabilities::affine(),
        None,
    )
    .expect("affine ownership contract must be valid")
}

fn shared_borrow_contract(value: ValueId) -> OwnershipContract {
    OwnershipContract::new(
        value,
        OwnershipKind::Borrowed {
            mutability: BorrowMutability::Shared,
        },
        OwnershipCapabilities::shared_borrow(),
        None,
    )
    .expect("shared-borrow contract must be valid")
}

fn exclusive_borrow_contract(value: ValueId) -> OwnershipContract {
    OwnershipContract::new(
        value,
        OwnershipKind::Borrowed {
            mutability: BorrowMutability::Exclusive,
        },
        OwnershipCapabilities::exclusive_borrow(),
        None,
    )
    .expect("exclusive-borrow contract must be valid")
}

fn ownership_use(
    value: ValueId,
    operation: OperationId,
    block: BlockId,
    kind: OwnershipUseKind,
) -> OwnershipUse {
    OwnershipUse::new(value, operation, block, kind)
        .expect("test ownership-use event must be structurally valid")
}

fn declare(
    index: &mut OwnershipIndex,
    contract: OwnershipContract,
) {
    index
        .declare(contract)
        .expect("test ownership contract declaration must succeed");
}

// -----------------------------------------------------------------------------
// Ownership capability and contract conformance
// -----------------------------------------------------------------------------

#[test]
fn accepts_each_supported_builtin_ownership_kind() {
    let contracts = [
        unrestricted_contract(value_id(1)),
        copy_contract(value_id(2)),
        move_only_contract(value_id(3)),
        linear_contract(value_id(4)),
        affine_contract(value_id(5)),
        shared_borrow_contract(value_id(6)),
        exclusive_borrow_contract(value_id(7)),
    ];

    for contract in contracts {
        assert!(
            contract.validate().is_ok(),
            "expected valid contract: {contract:?}"
        );
    }
}

#[test]
fn unrestricted_values_can_be_copied_moved_borrowed_and_dropped() {
    let capabilities = OwnershipCapabilities::unrestricted();

    assert!(capabilities.may_copy);
    assert!(capabilities.may_move);
    assert!(capabilities.may_borrow_shared);
    assert!(capabilities.may_borrow_exclusive);
    assert!(!capabilities.must_consume);
    assert!(capabilities.may_drop);
    assert!(capabilities.validate().is_ok());
}

#[test]
fn copy_ownership_requires_copy_capability() {
    let contract = copy_contract(value_id(1));

    assert!(contract.capabilities.may_copy);
    assert!(!contract.capabilities.must_consume);
    assert!(contract.capabilities.may_drop);
}

#[test]
fn move_only_ownership_rejects_copy_capability() {
    let result = OwnershipContract::new(
        value_id(1),
        OwnershipKind::MoveOnly,
        OwnershipCapabilities::copy(),
        None,
    );

    assert!(matches!(
        result,
        Err(OwnershipError::InvalidKindCapabilities { .. })
    ));
}

#[test]
fn linear_ownership_requires_consumption_and_prohibits_implicit_drop() {
    let contract = linear_contract(value_id(1));

    assert!(!contract.capabilities.may_copy);
    assert!(contract.capabilities.may_move);
    assert!(contract.capabilities.must_consume);
    assert!(!contract.capabilities.may_drop);
    assert!(contract.validate().is_ok());
}

#[test]
fn linear_ownership_rejects_implicit_drop() {
    let mut capabilities = OwnershipCapabilities::linear();
    capabilities.may_drop = true;

    let result = OwnershipContract::new(
        value_id(1),
        OwnershipKind::Linear,
        capabilities,
        None,
    );

    assert!(matches!(
        result,
        Err(OwnershipError::InvalidCapabilities(_))
            | Err(OwnershipError::InvalidKindCapabilities { .. })
    ));
}

#[test]
fn linear_ownership_rejects_missing_consumption_requirement() {
    let mut capabilities = OwnershipCapabilities::linear();
    capabilities.must_consume = false;

    let result = OwnershipContract::new(
        value_id(1),
        OwnershipKind::Linear,
        capabilities,
        None,
    );

    assert!(matches!(
        result,
        Err(OwnershipError::InvalidKindCapabilities { .. })
    ));
}

#[test]
fn affine_ownership_allows_drop_but_prohibits_copying() {
    let contract = affine_contract(value_id(1));

    assert!(!contract.capabilities.may_copy);
    assert!(!contract.capabilities.must_consume);
    assert!(contract.capabilities.may_drop);
    assert!(contract.validate().is_ok());
}

#[test]
fn affine_ownership_rejects_required_linear_consumption() {
    let mut capabilities = OwnershipCapabilities::affine();
    capabilities.must_consume = true;
    capabilities.may_drop = false;

    let result = OwnershipContract::new(
        value_id(1),
        OwnershipKind::Affine,
        capabilities,
        None,
    );

    assert!(matches!(
        result,
        Err(OwnershipError::InvalidKindCapabilities { .. })
    ));
}

#[test]
fn shared_borrow_rejects_exclusive_borrow_capability() {
    let result = OwnershipContract::new(
        value_id(1),
        OwnershipKind::Borrowed {
            mutability: BorrowMutability::Shared,
        },
        OwnershipCapabilities::exclusive_borrow(),
        None,
    );

    assert!(matches!(
        result,
        Err(OwnershipError::InvalidKindCapabilities { .. })
    ));
}

#[test]
fn exclusive_borrow_rejects_shared_borrow_capability() {
    let result = OwnershipContract::new(
        value_id(1),
        OwnershipKind::Borrowed {
            mutability: BorrowMutability::Exclusive,
        },
        OwnershipCapabilities::shared_borrow(),
        None,
    );

    assert!(matches!(
        result,
        Err(OwnershipError::InvalidKindCapabilities { .. })
    ));
}

#[test]
fn borrowed_ownership_rejects_independent_copy_or_move_ownership() {
    let mut capabilities = OwnershipCapabilities::shared_borrow();
    capabilities.may_move = true;

    let result = OwnershipContract::new(
        value_id(1),
        OwnershipKind::Borrowed {
            mutability: BorrowMutability::Shared,
        },
        capabilities,
        None,
    );

    assert!(matches!(
        result,
        Err(OwnershipError::InvalidKindCapabilities { .. })
    ));
}

#[test]
fn rejects_contradictory_consumption_and_drop_capabilities() {
    let mut capabilities = OwnershipCapabilities::move_only();
    capabilities.must_consume = true;
    capabilities.may_drop = true;

    assert!(matches!(
        capabilities.validate(),
        Err(OwnershipError::InvalidCapabilities(_))
    ));
}

#[test]
fn supports_extension_defined_ownership_kinds() {
    let name = qualified_name("future_domain::ownership::special");

    let contract = OwnershipContract::new(
        value_id(1),
        OwnershipKind::Extension(name.clone()),
        OwnershipCapabilities::move_only(),
        None,
    )
    .expect("valid extension ownership name must be accepted");

    assert_eq!(contract.kind, OwnershipKind::Extension(name));
    assert!(contract.validate().is_ok());
}

#[test]
fn supports_region_scoped_ownership_contracts() {
    let region = region_id(41);

    let contract = OwnershipContract::new(
        value_id(1),
        OwnershipKind::MoveOnly,
        OwnershipCapabilities::move_only(),
        Some(region),
    )
    .expect("region-scoped ownership contract must be valid");

    assert_eq!(contract.declared_in, Some(region));
    assert!(contract.validate().is_ok());
}

// -----------------------------------------------------------------------------
// Ownership-index contract management
// -----------------------------------------------------------------------------

#[test]
fn new_index_is_empty_and_valid() {
    let index = OwnershipIndex::new();

    assert!(index.is_empty());
    assert_eq!(index.contract_count(), 0);
    assert_eq!(index.use_count(), 0);
    assert!(index.validate().is_ok());
}

#[test]
fn declares_and_retrieves_contracts_by_typed_value_id() {
    let value = value_id(7);
    let contract = move_only_contract(value);
    let mut index = OwnershipIndex::new();

    index
        .declare(contract.clone())
        .expect("first declaration must succeed");

    assert_eq!(index.contract(value), Some(&contract));
    assert_eq!(index.contract_count(), 1);
    assert!(!index.is_empty());
    assert!(index.validate().is_ok());
}

#[test]
fn rejects_duplicate_contracts_without_mutating_the_index() {
    let value = value_id(1);
    let mut index = OwnershipIndex::new();

    declare(&mut index, unrestricted_contract(value));

    let before = index.clone();
    let result = index.declare(unrestricted_contract(value));

    assert_eq!(result, Err(OwnershipError::DuplicateContract(value)));
    assert_eq!(index, before);
}

#[test]
fn missing_contract_lookup_returns_none() {
    let index = OwnershipIndex::new();

    assert!(index.contract(value_id(99)).is_none());
}

#[test]
fn rejects_use_of_undeclared_value_without_mutating_the_index() {
    let mut index = OwnershipIndex::new();
    declare(&mut index, unrestricted_contract(value_id(1)));

    let before = index.clone();
    let undeclared = value_id(99);
    let usage = ownership_use(
        undeclared,
        operation_id(1),
        block_id(1),
        OwnershipUseKind::Read,
    );

    assert_eq!(
        index.record_use(usage),
        Err(OwnershipError::UndeclaredValue(undeclared))
    );
    assert_eq!(index, before);
}

#[test]
fn records_and_retrieves_ownership_use_events() {
    let value = value_id(1);
    let usage = ownership_use(
        value,
        operation_id(2),
        block_id(3),
        OwnershipUseKind::Read,
    );

    let mut index = OwnershipIndex::new();
    declare(&mut index, unrestricted_contract(value));

    index
        .record_use(usage.clone())
        .expect("declared value use must be recorded");

    assert_eq!(index.uses_of(value), &[usage]);
    assert_eq!(index.use_count(), 1);
    assert!(index.validate().is_ok());
}

#[test]
fn returns_an_empty_slice_for_a_value_without_uses() {
    let value = value_id(1);
    let mut index = OwnershipIndex::new();

    declare(&mut index, unrestricted_contract(value));

    assert!(index.uses_of(value).is_empty());
}

#[test]
fn preserves_distinct_ownership_event_kinds() {
    let value = value_id(1);
    let block = block_id(1);

    let events = [
        OwnershipUseKind::Read,
        OwnershipUseKind::Write,
        OwnershipUseKind::Move,
        OwnershipUseKind::BorrowShared,
        OwnershipUseKind::BorrowExclusive,
        OwnershipUseKind::Drop,
        OwnershipUseKind::CaptureShared,
        OwnershipUseKind::CaptureExclusive,
        OwnershipUseKind::Extension(qualified_name(
            "future_domain::ownership_event",
        )),
    ];

    let mut index = OwnershipIndex::new();
    declare(&mut index, unrestricted_contract(value));

    for (offset, kind) in events.into_iter().enumerate() {
        index
            .record_use(ownership_use(
                value,
                operation_id((offset as u128) + 1),
                block,
                kind,
            ))
            .expect("valid ownership event must be recorded");
    }

    assert_eq!(index.uses_of(value).len(), events.len());
    assert_eq!(index.use_count(), events.len());
    assert!(index.validate().is_ok());
}

#[test]
fn permits_multiple_events_for_the_same_operation_when_recorded() {
    // The index intentionally records events; it does not decide whether
    // an operation's semantics permit a particular combination of events.
    // That decision belongs to the dialect-aware ownership verifier.

    let value = value_id(1);
    let operation = operation_id(1);
    let block = block_id(1);

    let mut index = OwnershipIndex::new();
    declare(&mut index, unrestricted_contract(value));

    index
        .record_use(ownership_use(
            value,
            operation,
            block,
            OwnershipUseKind::Read,
        ))
        .expect("first event must be recorded");

    index
        .record_use(ownership_use(
            value,
            operation,
            block,
            OwnershipUseKind::Write,
        ))
        .expect("second event must be recorded");

    assert_eq!(index.uses_of(value).len(), 2);
    assert_eq!(index.use_count(), 2);
}

#[test]
fn contract_iteration_is_deterministic_by_value_id() {
    let mut index = OwnershipIndex::new();

    for raw in [9_u128, 2, 7, 1, 5] {
        declare(&mut index, unrestricted_contract(value_id(raw)));
    }

    let values: Vec<ValueId> = index
        .contracts()
        .map(|(value, _)| *value)
        .collect();

    assert_eq!(
        values,
        vec![
            value_id(1),
            value_id(2),
            value_id(5),
            value_id(7),
            value_id(9),
        ]
    );
}

#[test]
fn use_iteration_is_deterministic_by_value_id() {
    let mut index = OwnershipIndex::new();

    for raw in [8_u128, 3, 6, 1] {
        declare(&mut index, unrestricted_contract(value_id(raw)));

        index
            .record_use(ownership_use(
                value_id(raw),
                operation_id(raw),
                block_id(1),
                OwnershipUseKind::Read,
            ))
            .expect("declared value use must be recorded");
    }

    let values: Vec<ValueId> = index
        .all_uses()
        .map(|(value, _)| *value)
        .collect();

    assert_eq!(
        values,
        vec![
            value_id(1),
            value_id(3),
            value_id(6),
            value_id(8),
        ]
    );
}

#[test]
fn use_order_is_independent_of_insertion_order() {
    let value = value_id(1);
    let block = block_id(1);
    let operation_ids = [operation_id(5), operation_id(2), operation_id(4)];

    let mut first = OwnershipIndex::new();
    let mut second = OwnershipIndex::new();

    declare(&mut first, unrestricted_contract(value));
    declare(&mut second, unrestricted_contract(value));

    for operation in operation_ids {
        first
            .record_use(ownership_use(
                value,
                operation,
                block,
                OwnershipUseKind::Read,
            ))
            .expect("first index insertion must succeed");
    }

    for operation in operation_ids.into_iter().rev() {
        second
            .record_use(ownership_use(
                value,
                operation,
                block,
                OwnershipUseKind::Read,
            ))
            .expect("second index insertion must succeed");
    }

    assert_eq!(first.uses_of(value), second.uses_of(value));
    assert_eq!(
        first
            .uses_of(value)
            .iter()
            .map(|usage| usage.operation)
            .collect::<Vec<_>>(),
        vec![operation_id(2), operation_id(4), operation_id(5)]
    );
}

#[test]
fn validates_a_populated_index() {
    let value = value_id(1);
    let mut index = OwnershipIndex::new();

    declare(&mut index, move_only_contract(value));

    index
        .record_use(ownership_use(
            value,
            operation_id(1),
            block_id(1),
            OwnershipUseKind::Move,
        ))
        .expect("valid move event must be recorded");

    assert!(index.verify_internal_consistency().is_ok());
    assert!(index.validate().is_ok());
}

#[test]
fn clear_removes_contracts_and_uses() {
    let value = value_id(1);
    let mut index = OwnershipIndex::new();

    declare(&mut index, unrestricted_contract(value));

    index
        .record_use(ownership_use(
            value,
            operation_id(1),
            block_id(1),
            OwnershipUseKind::Read,
        ))
        .expect("valid event must be recorded");

    assert!(!index.is_empty());

    index.clear();

    assert!(index.is_empty());
    assert_eq!(index.contract_count(), 0);
    assert_eq!(index.use_count(), 0);
    assert!(index.validate().is_ok());
}

// -----------------------------------------------------------------------------
// Dynamic growth and extension compatibility
// -----------------------------------------------------------------------------

#[test]
fn index_grows_dynamically_for_many_values_and_uses() {
    // This is a regression workload, not a language or implementation limit.
    // Increase it in dedicated stress tests when the CI resource budget allows.
    const TEST_VALUE_COUNT: u128 = 1_024;

    let mut index = OwnershipIndex::new();

    for raw in 1..=TEST_VALUE_COUNT {
        let value = value_id(raw);
        declare(&mut index, unrestricted_contract(value));

        index
            .record_use(ownership_use(
                value,
                operation_id(raw * 2),
                block_id(1),
                OwnershipUseKind::Read,
            ))
            .expect("read event must be recorded");

        index
            .record_use(ownership_use(
                value,
                operation_id((raw * 2) + 1),
                block_id(1),
                OwnershipUseKind::Write,
            ))
            .expect("write event must be recorded");
    }

    assert_eq!(index.contract_count(), TEST_VALUE_COUNT as usize);
    assert_eq!(index.use_count(), (TEST_VALUE_COUNT * 2) as usize);
    assert!(index.validate().is_ok());

    for raw in 1..=TEST_VALUE_COUNT {
        let value = value_id(raw);
        assert_eq!(index.uses_of(value).len(), 2);
    }
}

#[test]
fn accepts_future_domain_event_names_without_changing_the_core_enum() {
    let value = value_id(1);
    let mut index = OwnershipIndex::new();

    declare(&mut index, unrestricted_contract(value));

    let future_event = OwnershipUseKind::Extension(qualified_name(
        "future_domain::resource_transfer",
    ));

    index
        .record_use(ownership_use(
            value,
            operation_id(1),
            block_id(1),
            future_event.clone(),
        ))
        .expect("valid extension event must be recorded");

    assert_eq!(index.uses_of(value).len(), 1);
    assert_eq!(index.uses_of(value)[0].kind, future_event);
    assert!(index.validate().is_ok());
}