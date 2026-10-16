
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Ownership Adapter
//!
//! Path: `src/classical/memory/ownership.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Expose canonical ownership contracts to classical-computing consumers
//! without creating a competing ownership model.
//!
//! # Authority
//!
//! - `src/zuir/ir/ownership.rs` owns canonical ownership kinds,
//!   capabilities, contracts, use events, and deterministic indexing.
//! - `src/zuir/ids.rs` owns canonical ZUIR identifiers.
//! - `grammar/specification/semantic-model.md` owns source-level
//!   ownership semantics.
//! - The canonical ownership verifier owns whole-program and
//!   control-flow-sensitive correctness checks.
//!
//! # Owns
//!
//! This file owns only the classical-facing adapter and its constructors.
//!
//! # Does not own
//!
//! This file does not define a second ownership kind, ownership contract,
//! ownership index, identifier type, borrow checker, lifetime engine,
//! allocator, runtime, or physical-memory model.
//!
//! # Semantics
//!
//! Ownership describes responsibility for a semantic value's lifetime;
//! it does not identify a physical address or prescribe a storage location.
//! Moving a value transfers ownership and does not necessarily move bytes.
//!
//! This adapter's constructors create locally valid contracts. They do not
//! prove that a program's control flow obeys those contracts.
//!
//! # POCO-REAF and scalability
//!
//! No hardware counts, memory capacities, collection sizes, or tensor ranks
//! are imposed here. Finite identifier representations and actual resource
//! availability remain explicit constraints of their respective owners.
//!
//! # Integration
//!
//! - `memory/mod.rs` declares this module.
//! - `classical/mod.rs` exposes the memory module through the classical
//!   subsystem's existing public module structure.
//! - Classical semantic analysis and lowering use the canonical ValueId
//!   and ownership types.
//! - ZUIR verification remains responsible for checking recorded uses,
//!   moves, borrows, consumption, lifetimes, and control-flow joins.
//! - Quantum-domain resources retain their domain-specific semantics;
//!   this adapter must not make them copyable or implicitly droppable.
//!
//! # Freeze rule
//!
//! Do not introduce locally defined equivalents of canonical ZUIR
//! ownership types. Any semantic change must be made at the authoritative
//! specification and canonical implementation, followed by compatibility
//! and integration review.

use crate::zuir::ids::{RegionId, ValueId};

pub use crate::zuir::ir::ownership::{
    BorrowMutability,
    OwnershipCapabilities,
    OwnershipContract,
    OwnershipError,
    OwnershipIndex,
    OwnershipKind,
    OwnershipResult,
    OwnershipUse,
    OwnershipUseKind,
};

/// Constructs a validated ownership contract for an unrestricted value.
///
/// The canonical capability policy permits copying, moving, borrowing,
/// and dropping. Actual operations must still obey the value's established
/// type and semantic rules.
///
/// # Errors
///
/// Returns an ownership error if canonical contract validation fails.
pub fn unrestricted(
    value: ValueId,
    declared_in: Option<RegionId>,
) -> OwnershipResult<OwnershipContract> {
    OwnershipContract::new(
        value,
        OwnershipKind::Unrestricted,
        OwnershipCapabilities::unrestricted(),
        declared_in,
    )
}

/// Constructs a validated contract for a copyable value.
///
/// Use only when the canonical type and semantic analysis have established
/// that duplication preserves the value's meaning and obligations.
///
/// This constructor does not establish that an arbitrary value is Copy.
pub fn copyable(
    value: ValueId,
    declared_in: Option<RegionId>,
) -> OwnershipResult<OwnershipContract> {
    OwnershipContract::new(
        value,
        OwnershipKind::Copy,
        OwnershipCapabilities::copy(),
        declared_in,
    )
}

/// Constructs a validated contract for a move-only value.
///
/// The value cannot be copied implicitly, but ownership may be transferred.
/// Dropping is permitted under the canonical move-only capability policy.
pub fn move_only(
    value: ValueId,
    declared_in: Option<RegionId>,
) -> OwnershipResult<OwnershipContract> {
    OwnershipContract::new(
        value,
        OwnershipKind::MoveOnly,
        OwnershipCapabilities::move_only(),
        declared_in,
    )
}

/// Constructs a validated affine ownership contract.
///
/// An affine value may be consumed at most once. It need not be consumed
/// on every successful control-flow path.
///
/// The whole-program verifier must still check every relevant use and path.
pub fn affine(
    value: ValueId,
    declared_in: Option<RegionId>,
) -> OwnershipResult<OwnershipContract> {
    OwnershipContract::new(
        value,
        OwnershipKind::Affine,
        OwnershipCapabilities::affine(),
        declared_in,
    )
}

/// Constructs a validated linear ownership contract.
///
/// A linear value must be consumed on every valid execution path and may
/// not be implicitly dropped. This is a local contract declaration, not
/// proof that all control-flow paths satisfy the requirement.
pub fn linear(
    value: ValueId,
    declared_in: Option<RegionId>,
) -> OwnershipResult<OwnershipContract> {
    OwnershipContract::new(
        value,
        OwnershipKind::Linear,
        OwnershipCapabilities::linear(),
        declared_in,
    )
}

/// Constructs a contract describing a shared borrowed value.
///
/// The borrow does not transfer ownership. The canonical verifier remains
/// responsible for proving that its origin remains valid and that no
/// conflicting exclusive access occurs.
pub fn shared_borrow(
    value: ValueId,
    declared_in: Option<RegionId>,
) -> OwnershipResult<OwnershipContract> {
    OwnershipContract::new(
        value,
        OwnershipKind::Borrowed {
            mutability: BorrowMutability::Shared,
        },
        OwnershipCapabilities::shared_borrow(),
        declared_in,
    )
}

/// Constructs a contract describing an exclusive borrowed value.
///
/// The borrow does not transfer ownership. The canonical verifier remains
/// responsible for proving exclusivity, origin validity, reborrowing rules,
/// and the absence of conflicting access throughout the borrow's lifetime.
pub fn exclusive_borrow(
    value: ValueId,
    declared_in: Option<RegionId>,
) -> OwnershipResult<OwnershipContract> {
    OwnershipContract::new(
        value,
        OwnershipKind::Borrowed {
            mutability: BorrowMutability::Exclusive,
        },
        OwnershipCapabilities::exclusive_borrow(),
        declared_in,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn region_id(raw: u128) -> RegionId {
        RegionId::new(raw).expect("test region ID must be nonzero")
    }

    #[test]
    fn constructors_use_canonical_ownership_types() {
        let contract = copyable(value_id(1), None).expect("valid contract");

        assert_eq!(contract.value, value_id(1));
        assert_eq!(contract.kind, OwnershipKind::Copy);
        assert!(contract.capabilities.may_copy);
        assert!(!contract.capabilities.must_consume);
        contract.validate().expect("contract remains valid");
    }

    #[test]
    fn move_only_contract_prohibits_copying() {
        let contract = move_only(value_id(2), None).expect("valid contract");

        assert!(!contract.capabilities.may_copy);
        assert!(contract.capabilities.may_move);
        assert!(contract.capabilities.may_drop);
        contract.validate().expect("contract remains valid");
    }

    #[test]
    fn affine_and_linear_contracts_have_distinct_obligations() {
        let affine_contract =
            affine(value_id(3), None).expect("valid affine contract");
        let linear_contract =
            linear(value_id(4), None).expect("valid linear contract");

        assert!(!affine_contract.capabilities.may_copy);
        assert!(!affine_contract.capabilities.must_consume);
        assert!(affine_contract.capabilities.may_drop);

        assert!(!linear_contract.capabilities.may_copy);
        assert!(linear_contract.capabilities.must_consume);
        assert!(!linear_contract.capabilities.may_drop);
    }

    #[test]
    fn borrowed_contracts_preserve_their_access_modes() {
        let shared =
            shared_borrow(value_id(5), None).expect("valid shared borrow");
        let exclusive =
            exclusive_borrow(value_id(6), None).expect("valid exclusive borrow");

        assert_eq!(
            shared.kind,
            OwnershipKind::Borrowed {
                mutability: BorrowMutability::Shared,
            }
        );
        assert_eq!(
            exclusive.kind,
            OwnershipKind::Borrowed {
                mutability: BorrowMutability::Exclusive,
            }
        );

        assert!(!shared.capabilities.may_move);
        assert!(!exclusive.capabilities.may_move);
    }

    #[test]
    fn declaration_region_is_preserved() {
        let region = region_id(7);
        let contract =
            move_only(value_id(8), Some(region)).expect("valid contract");

        assert_eq!(contract.declared_in, Some(region));
    }

    #[test]
    fn canonical_index_rejects_duplicate_contracts() {
        let mut index = OwnershipIndex::new();
        let value = value_id(9);

        index
            .declare(move_only(value, None).expect("valid contract"))
            .expect("first declaration succeeds");

        let duplicate = move_only(value, None).expect("valid contract");

        assert!(matches!(
            index.declare(duplicate),
            Err(OwnershipError::DuplicateContract(id)) if id == value
        ));
    }

    #[test]
    fn canonical_index_rejects_uses_without_a_contract() {
        use crate::zuir::ids::{BlockId, OperationId};

        let mut index = OwnershipIndex::new();

        let usage = OwnershipUse::new(
            value_id(10),
            OperationId::new(11).expect("valid operation ID"),
            BlockId::new(12).expect("valid block ID"),
            OwnershipUseKind::Read,
        )
        .expect("valid use event");

        assert!(matches!(
            index.record_use(usage),
            Err(OwnershipError::UndeclaredValue(id)) if id == value_id(10)
        ));
    }

    #[test]
    fn repeated_validation_does_not_change_contract_meaning() {
        let contract = linear(value_id(13), None).expect("valid contract");

        contract.validate().expect("first validation");
        contract.validate().expect("second validation");

        assert_eq!(contract.kind, OwnershipKind::Linear);
        assert!(contract.capabilities.must_consume);
    }
}
