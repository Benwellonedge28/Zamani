
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Borrowing Contracts
//!
//! Path: `src/classical/memory/borrowing.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide a classical-facing representation and deterministic index of
//! borrow events, validated against the canonical ZUIR ownership contracts.
//!
//! # Authority
//!
//! - `src/zuir/ids.rs` owns canonical value, region, operation and block IDs.
//! - `src/zuir/ir/ownership.rs` owns ownership contracts and ownership-use
//!   events.
//! - `grammar/memory/borrowing.g4` owns borrowing-related source syntax.
//! - `grammar/memory/memory.g4` owns memory-place and memory-operation syntax.
//! - `grammar/memory/lifetimes.g4` owns lifetime syntax.
//! - `grammar/specification/semantic-model.md` owns source-level semantics.
//!
//! # Responsibilities
//!
//! This module validates locally checkable borrow invariants:
//!
//! - the source and borrowed reference are distinct semantic values;
//! - both values have canonical ownership contracts;
//! - the source contract permits the requested borrow mode;
//! - the reference contract is marked as borrowed with the matching mode;
//! - the reference contract's declaration region agrees with the borrow;
//! - each borrowed reference has at most one indexed borrow declaration;
//! - canonical ownership-use events are recorded for borrow creation.
//!
//! # Does not own
//!
//! This module does not implement a second ownership system, reference type
//! system, lifetime inference engine, alias analysis, control-flow verifier,
//! memory allocator, runtime, physical address model or target scheduler.
//!
//! In particular, recording a borrow does not prove that its lifetime is
//! valid, that two references do not alias illegally, or that every path
//! respects the borrow. Those properties require the canonical semantic
//! verifier, region relationships, memory-place analysis and control-flow
//! graph.
//!
//! # Scalability and POCO-REAF
//!
//! No universal limits are imposed on hardware counts, memory capacity,
//! borrow count, region count or program size. Index storage grows with the
//! number of recorded entries and remains subject to actual resource
//! availability. Ordered maps provide deterministic iteration.
//!
//! # Transaction contract
//!
//! `record_borrow` checks the record and both ownership contracts before
//! changing either index. It records the canonical ownership-use event
//! before inserting the borrow record. No fallible local validation remains
//! after the canonical event has been recorded.
//!
//! This is not a general cross-index transaction mechanism. Callers must
//! avoid concurrent mutation of the same semantic analysis state.
//!
//! # Integration
//!
//! - `memory/mod.rs` declares `pub mod borrowing;`.
//! - `memory/ownership.rs` exposes the canonical ownership adapter.
//! - The semantic analyzer constructs borrow records after resolving source
//!   expressions and reference values to canonical ValueIds.
//! - Lifetime and alias verifiers consume the borrow index alongside
//!   region information, memory places and control-flow information.
//! - Lowering preserves established borrow and lifetime obligations.
//! - Runtime and backend code must not treat this index as proof of safety.
//!
//! # Freeze rule
//!
//! Do not add a parallel ValueId, RegionId, reference type, ownership kind,
//! or lifetime engine here. Changes to semantic invariants require an
//! explicit review of the canonical ownership verifier and normative
//! semantic specification.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::zuir::ids::{BlockId, OperationId, RegionId, ValueId};

use super::ownership::{
    BorrowMutability,
    OwnershipContract,
    OwnershipError,
    OwnershipIndex,
    OwnershipKind,
    OwnershipResult,
    OwnershipUse,
    OwnershipUseKind,
};

/// Result type for borrowing-contract operations.
pub type BorrowResult<T> = Result<T, BorrowError>;

/// Errors raised by local borrowing-contract validation and indexing.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BorrowError {
    /// The source value has no canonical ownership contract.
    #[error("borrow source {0:?} has no ownership contract")]
    MissingSourceContract(ValueId),

    /// The resulting borrowed reference has no canonical ownership contract.
    #[error("borrowed reference {0:?} has no ownership contract")]
    MissingReferenceContract(ValueId),

    /// A value cannot be its own newly created borrowed reference.
    #[error("borrow source and borrowed reference must be distinct values")]
    SelfBorrow,

    /// The source ownership contract does not permit the requested mode.
    #[error(
        "source value {value:?} does not permit a {mutability:?} borrow"
    )]
    SourceNotBorrowable {
        value: ValueId,
        mutability: BorrowMutability,
    },

    /// The resulting value is not classified as a compatible borrow.
    #[error(
        "reference value {value:?} does not have the expected {mutability:?} \
         borrowed ownership contract"
    )]
    ReferenceContractMismatch {
        value: ValueId,
        mutability: BorrowMutability,
    },

    /// The borrow's region and the reference contract disagree.
    #[error(
        "borrowed reference {value:?} has declaration region \
         {contract_region:?}, but the borrow specifies {borrow_region:?}"
    )]
    RegionMismatch {
        value: ValueId,
        contract_region: Option<RegionId>,
        borrow_region: Option<RegionId>,
    },

    /// A borrow record for this resulting reference already exists.
    #[error("borrow for reference value {0:?} is already recorded")]
    DuplicateBorrow(ValueId),

    /// The borrow index contains an entry inconsistent with its key.
    #[error(
        "borrow index key {key:?} does not match reference {record_reference:?}"
    )]
    ReferenceKeyMismatch {
        key: ValueId,
        record_reference: ValueId,
    },

    /// A borrow record refers to a value without its canonical contract.
    #[error("borrow record references undeclared value {0:?}")]
    UndeclaredValue(ValueId),

    /// An error from canonical ownership validation or indexing.
    #[error(transparent)]
    Ownership(#[from] OwnershipError),
}

/// A single borrowing event in canonical semantic identity space.
///
/// `source` identifies the value from which the borrow originates.
/// `reference` identifies the semantic value representing the borrowed
/// reference. These are distinct ValueIds.
///
/// `region` identifies the region in which the reference contract is
/// declared. `None` means that the contract does not identify a specific
/// region here; it does not mean that the borrow has an infinite lifetime.
///
/// The operation and block identify the source-level semantic operation
/// after lowering into the representation used by the canonical verifier.
/// They do not imply execution order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BorrowRecord {
    /// Value from which the borrow originates.
    pub source: ValueId,

    /// Semantic value representing the resulting borrowed reference.
    pub reference: ValueId,

    /// Operation responsible for creating the borrow.
    pub operation: OperationId,

    /// Block containing the operation.
    pub block: BlockId,

    /// Region in which the reference contract is declared, if known.
    pub region: Option<RegionId>,

    /// Access mode requested by the borrow.
    pub mutability: BorrowMutability,
}

impl BorrowRecord {
    /// Construct a borrow record and validate its identity-level invariants.
    ///
    /// This constructor does not establish that the source exists, that the
    /// source permits borrowing, or that the reference outlives its uses.
    /// Those checks require canonical contracts and semantic context.
    pub fn new(
        source: ValueId,
        reference: ValueId,
        operation: OperationId,
        block: BlockId,
        region: Option<RegionId>,
        mutability: BorrowMutability,
    ) -> BorrowResult<Self> {
        if source == reference {
            return Err(BorrowError::SelfBorrow);
        }

        Ok(Self {
            source,
            reference,
            operation,
            block,
            region,
            mutability,
        })
    }

    /// Validate this record against canonical ownership contracts.
    ///
    /// This checks local contract consistency only. It does not perform
    /// alias analysis, control-flow analysis or lifetime inference.
    pub fn validate_against(
        &self,
        ownership: &OwnershipIndex,
    ) -> BorrowResult<()> {
        if self.source == self.reference {
            return Err(BorrowError::SelfBorrow);
        }

        let source_contract = ownership
            .contract(self.source)
            .ok_or(BorrowError::MissingSourceContract(self.source))?;

        let reference_contract = ownership
            .contract(self.reference)
            .ok_or(BorrowError::MissingReferenceContract(self.reference))?;

        source_contract.validate()?;
        reference_contract.validate()?;

        validate_source_capability(source_contract, self.mutability)?;
        validate_reference_contract(reference_contract, self)?;

        if reference_contract.declared_in != self.region {
            return Err(BorrowError::RegionMismatch {
                value: self.reference,
                contract_region: reference_contract.declared_in,
                borrow_region: self.region,
            });
        }

        Ok(())
    }

    /// Convert this record into the canonical ownership-use event.
    ///
    /// The event is associated with the source value, because borrowing uses
    /// the source without transferring ownership.
    pub fn to_ownership_use(&self) -> OwnershipResult<OwnershipUse> {
        let kind = match self.mutability {
            BorrowMutability::Shared => OwnershipUseKind::BorrowShared,
            BorrowMutability::Exclusive => OwnershipUseKind::BorrowExclusive,
        };

        OwnershipUse::new(self.source, self.operation, self.block, kind)
    }
}

/// Check that the source contract allows the requested borrowing mode.
fn validate_source_capability(
    contract: &OwnershipContract,
    mutability: BorrowMutability,
) -> BorrowResult<()> {
    let permitted = match mutability {
        BorrowMutability::Shared => contract.capabilities.may_borrow_shared,
        BorrowMutability::Exclusive => {
            contract.capabilities.may_borrow_exclusive
        }
    };

    if !permitted {
        return Err(BorrowError::SourceNotBorrowable {
            value: contract.value,
            mutability,
        });
    }

    Ok(())
}

/// Check that the resulting value has the correct borrowed ownership kind.
fn validate_reference_contract(
    contract: &OwnershipContract,
    borrow: &BorrowRecord,
) -> BorrowResult<()> {
    let expected = OwnershipKind::Borrowed {
        mutability: borrow.mutability,
    };

    if contract.kind != expected {
        return Err(BorrowError::ReferenceContractMismatch {
            value: borrow.reference,
            mutability: borrow.mutability,
        });
    }

    Ok(())
}

/// Deterministic index of borrow declarations keyed by resulting reference.
///
/// Each reference value may have at most one borrow-creation record in this
/// index. Multiple references may borrow the same source; whether their
/// lifetimes and access modes may overlap is a separate semantic check.
///
/// The canonical OwnershipIndex remains the authority for ownership
/// contracts and ownership-use events.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BorrowIndex {
    records: BTreeMap<ValueId, BorrowRecord>,
}

impl BorrowIndex {
    /// Construct an empty borrow index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of indexed borrow records.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Whether the index contains no borrow records.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Look up the borrow record for a resulting reference value.
    pub fn get(&self, reference: ValueId) -> Option<&BorrowRecord> {
        self.records.get(&reference)
    }

    /// Iterate over records in deterministic reference-ID order.
    pub fn iter(&self) -> impl Iterator<Item = (&ValueId, &BorrowRecord)> {
        self.records.iter()
    }

    /// Record a borrow after validating both canonical ownership contracts.
    ///
    /// A failed validation leaves this borrow index unchanged. The canonical
    /// ownership-use event is recorded only after all local checks pass.
    ///
    /// # Errors
    ///
    /// Returns an error if the borrow is malformed, either contract is
    /// missing, the source prohibits the requested access, the reference
    /// contract is incompatible, the region differs, or the reference has
    /// already been indexed.
    pub fn record_borrow(
        &mut self,
        ownership: &mut OwnershipIndex,
        borrow: BorrowRecord,
    ) -> BorrowResult<()> {
        if self.records.contains_key(&borrow.reference) {
            return Err(BorrowError::DuplicateBorrow(borrow.reference));
        }

        borrow.validate_against(ownership)?;

        // Prepare the canonical event before mutating either index.
        let usage = borrow.to_ownership_use()?;

        // Canonical OwnershipIndex performs its own source-contract check.
        // If it fails, this BorrowIndex remains unchanged.
        ownership.record_use(usage)?;

        // No fallible local operation remains after the canonical event
        // succeeds, so the two indexes cannot diverge through a later
        // validation error in this method.
        self.records.insert(borrow.reference, borrow);

        Ok(())
    }

    /// Validate every indexed record against the supplied ownership index.
    ///
    /// This is useful after deserialization or when validating a completed
    /// semantic model. It does not prove whole-program borrow safety.
    pub fn validate_against(
        &self,
        ownership: &OwnershipIndex,
    ) -> BorrowResult<()> {
        for (key, record) in &self.records {
            if *key != record.reference {
                return Err(BorrowError::ReferenceKeyMismatch {
                    key: *key,
                    record_reference: record.reference,
                });
            }

            record.validate_against(ownership)?;
        }

        Ok(())
    }

    /// Remove a record from this local index.
    ///
    /// This operation does not remove the corresponding canonical ownership
    /// event, because OwnershipIndex intentionally has no event-removal API.
    /// Use this only when rebuilding the indexes together or discarding the
    /// whole analysis result.
    pub fn remove_local(
        &mut self,
        reference: ValueId,
    ) -> Option<BorrowRecord> {
        self.records.remove(&reference)
    }

    /// Clear all records from this local index.
    ///
    /// Canonical ownership-use events are not removed. Rebuild or discard
    /// the corresponding OwnershipIndex when resetting an analysis.
    pub fn clear_local(&mut self) {
        self.records.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{BlockId, OperationId};

    use super::super::ownership::{
        exclusive_borrow,
        move_only,
        shared_borrow,
    };

    fn value(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn region(raw: u128) -> RegionId {
        RegionId::new(raw).expect("test region ID must be nonzero")
    }

    fn operation(raw: u128) -> OperationId {
        OperationId::new(raw).expect("test operation ID must be nonzero")
    }

    fn block(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test block ID must be nonzero")
    }

    fn setup_shared_borrow(
        source: ValueId,
        reference: ValueId,
        scope: Option<RegionId>,
    ) -> OwnershipIndex {
        let mut ownership = OwnershipIndex::new();

        ownership
            .declare(move_only(source, None).expect("valid source contract"))
            .expect("source declaration succeeds");

        ownership
            .declare(
                shared_borrow(reference, scope)
                    .expect("valid borrowed reference contract"),
            )
            .expect("reference declaration succeeds");

        ownership
    }

    #[test]
    fn record_is_constructed_for_distinct_values() {
        let record = BorrowRecord::new(
            value(1),
            value(2),
            operation(3),
            block(4),
            None,
            BorrowMutability::Shared,
        )
        .expect("distinct values form a record");

        assert_eq!(record.source, value(1));
        assert_eq!(record.reference, value(2));
        assert_eq!(record.mutability, BorrowMutability::Shared);
    }

    #[test]
    fn self_borrow_is_rejected() {
        assert_eq!(
            BorrowRecord::new(
                value(1),
                value(1),
                operation(2),
                block(3),
                None,
                BorrowMutability::Shared,
            ),
            Err(BorrowError::SelfBorrow)
        );
    }

    #[test]
    fn valid_shared_borrow_is_recorded_in_both_indexes() {
        let source = value(10);
        let reference = value(11);
        let mut ownership = setup_shared_borrow(source, reference, None);
        let mut borrows = BorrowIndex::new();

        let record = BorrowRecord::new(
            source,
            reference,
            operation(12),
            block(13),
            None,
            BorrowMutability::Shared,
        )
        .expect("valid record");

        borrows
            .record_borrow(&mut ownership, record)
            .expect("valid borrow is recorded");

        assert_eq!(borrows.len(), 1);
        assert!(borrows.get(reference).is_some());
        assert_eq!(borrows.get(reference).unwrap().source, source);

        assert_eq!(ownership.uses_of(source).len(), 1);
        assert_eq!(
            ownership.uses_of(source)[0].kind,
            OwnershipUseKind::BorrowShared
        );

        borrows
            .validate_against(&ownership)
            .expect("record and contracts agree");
    }

    #[test]
    fn valid_exclusive_borrow_records_exclusive_use() {
        let source = value(20);
        let reference = value(21);
        let mut ownership = OwnershipIndex::new();

        ownership
            .declare(move_only(source, None).expect("source contract"))
            .expect("source declaration");

        ownership
            .declare(
                exclusive_borrow(reference, None).expect("reference contract"),
            )
            .expect("reference declaration");

        let mut borrows = BorrowIndex::new();

        let record = BorrowRecord::new(
            source,
            reference,
            operation(22),
            block(23),
            None,
            BorrowMutability::Exclusive,
        )
        .expect("valid record");

        borrows
            .record_borrow(&mut ownership, record)
            .expect("exclusive borrow is recorded");

        assert_eq!(
            ownership.uses_of(source)[0].kind,
            OwnershipUseKind::BorrowExclusive
        );
    }

    #[test]
    fn undeclared_source_is_rejected_without_mutation() {
        let source = value(30);
        let reference = value(31);
        let mut ownership = OwnershipIndex::new();

        ownership
            .declare(
                shared_borrow(reference, None).expect("reference contract"),
            )
            .expect("reference declaration");

        let mut borrows = BorrowIndex::new();
        let record = BorrowRecord::new(
            source,
            reference,
            operation(32),
            block(33),
            None,
            BorrowMutability::Shared,
        )
        .expect("structurally valid record");

        assert!(matches!(
            borrows.record_borrow(&mut ownership, record),
            Err(BorrowError::MissingSourceContract(id)) if id == source
        ));

        assert!(borrows.is_empty());
        assert!(ownership.uses_of(source).is_empty());
    }

    #[test]
    fn incompatible_reference_contract_is_rejected() {
        let source = value(40);
        let reference = value(41);
        let mut ownership = OwnershipIndex::new();

        ownership
            .declare(move_only(source, None).expect("source contract"))
            .expect("source declaration");

        ownership
            .declare(
                exclusive_borrow(reference, None).expect("exclusive contract"),
            )
            .expect("reference declaration");

        let mut borrows = BorrowIndex::new();
        let record = BorrowRecord::new(
            source,
            reference,
            operation(42),
            block(43),
            None,
            BorrowMutability::Shared,
        )
        .expect("structurally valid record");

        assert!(matches!(
            borrows.record_borrow(&mut ownership, record),
            Err(BorrowError::ReferenceContractMismatch { value, .. })
                if value == reference
        ));

        assert!(borrows.is_empty());
        assert!(ownership.uses_of(source).is_empty());
    }

    #[test]
    fn reference_region_must_match_the_borrow_region() {
        let source = value(50);
        let reference = value(51);
        let declared_region = region(52);
        let other_region = region(53);
        let mut ownership =
            setup_shared_borrow(source, reference, Some(declared_region));
        let mut borrows = BorrowIndex::new();

        let record = BorrowRecord::new(
            source,
            reference,
            operation(54),
            block(55),
            Some(other_region),
            BorrowMutability::Shared,
        )
        .expect("structurally valid record");

        assert!(matches!(
            borrows.record_borrow(&mut ownership, record),
            Err(BorrowError::RegionMismatch { value, .. })
                if value == reference
        ));

        assert!(borrows.is_empty());
        assert!(ownership.uses_of(source).is_empty());
    }

    #[test]
    fn duplicate_borrow_declaration_is_rejected() {
        let source = value(60);
        let reference = value(61);
        let mut ownership = setup_shared_borrow(source, reference, None);
        let mut borrows = BorrowIndex::new();

        let first = BorrowRecord::new(
            source,
            reference,
            operation(62),
            block(63),
            None,
            BorrowMutability::Shared,
        )
        .expect("first record");

        borrows
            .record_borrow(&mut ownership, first)
            .expect("first borrow succeeds");

        let second = BorrowRecord::new(
            source,
            reference,
            operation(64),
            block(65),
            None,
            BorrowMutability::Shared,
        )
        .expect("second record");

        assert!(matches!(
            borrows.record_borrow(&mut ownership, second),
            Err(BorrowError::DuplicateBorrow(id)) if id == reference
        ));

        assert_eq!(borrows.len(), 1);
        assert_eq!(ownership.uses_of(source).len(), 1);
    }

    #[test]
    fn ordered_index_is_stable_across_insertion_order() {
        let mut first = BorrowIndex::new();
        let mut second = BorrowIndex::new();

        let records = [
            BorrowRecord::new(
                value(70),
                value(73),
                operation(80),
                block(90),
                None,
                BorrowMutability::Shared,
            )
            .expect("record A"),
            BorrowRecord::new(
                value(71),
                value(72),
                operation(81),
                block(91),
                None,
                BorrowMutability::Exclusive,
            )
            .expect("record B"),
        ];

        first.records.insert(records[0].reference, records[0].clone());
        first.records.insert(records[1].reference, records[1].clone());

        second.records.insert(records[1].reference, records[1].clone());
        second.records.insert(records[0].reference, records[0].clone());

        assert_eq!(
            first.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
            second.iter().map(|(id, _)| *id).collect::<Vec<_>>()
        );
    }

    #[test]
    fn empty_index_is_valid() {
        let ownership = OwnershipIndex::new();
        let borrows = BorrowIndex::new();

        assert!(borrows.is_empty());
        assert_eq!(borrows.len(), 0);
        borrows
            .validate_against(&ownership)
            .expect("empty index is valid");
    }
}
