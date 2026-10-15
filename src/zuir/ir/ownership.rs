//! Ownership contracts and ownership-use indexing for ZUIR.
//!
//! # Responsibility
//!
//! This module defines ownership kinds, capabilities, contracts, and recorded
//! ownership-use events. It provides deterministic indexing and local
//! consistency validation.
//!
//! It does not perform control-flow-aware ownership verification. In
//! particular, it does not independently prove that values are consumed on
//! every control-flow path, that borrows do not overlap illegally, or that
//! region captures obey lifetime rules.
//!
//! # Scalability
//!
//! This module imposes no fixed limits on values, operations, blocks, regions,
//! or ownership contracts. Actual resource exhaustion must be handled by the
//! caller.
//!
//! # Integration
//!
//! The ownership verifier should consume this index together with the CFG,
//! operation semantics, region structure, and function signatures.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::zuir::ids::{BlockId, FunctionId, OperationId, RegionId, ValueId};
use crate::zuir::schema::types::QualifiedName;

/// Result type used by this module.
pub type OwnershipResult<T> = Result<T, OwnershipError>;

/// Errors produced by ownership contracts and their index.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OwnershipError {
    #[error("ownership contract for value {0:?} is already declared")]
    DuplicateContract(ValueId),

    #[error("ownership information for value {0:?} has not been declared")]
    UndeclaredValue(ValueId),

    #[error("invalid ownership capabilities: {0}")]
    InvalidCapabilities(String),

    #[error("ownership kind {kind} conflicts with its capabilities: {reason}")]
    InvalidKindCapabilities {
        kind: String,
        reason: String,
    },

    #[error("invalid ownership extension name: {0}")]
    InvalidExtensionName(String),

    #[error(
        "ownership index inconsistency: contract key {key:?} \
         does not match contract value {contract_value:?}"
    )]
    ContractKeyMismatch {
        key: ValueId,
        contract_value: ValueId,
    },

    #[error(
        "ownership index inconsistency: use-index key {key:?} \
         does not match use value {use_value:?}"
    )]
    UseKeyMismatch {
        key: ValueId,
        use_value: ValueId,
    },

    #[error(
        "ownership index contains uses for undeclared value {0:?}"
    )]
    UsesWithoutContract(ValueId),
}

/// Mutability of a borrowed value.
///
/// The precise lifetime and aliasing rules are enforced by the ownership
/// verifier and the region model, not by this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorrowMutability {
    /// Read-only access through this borrow.
    Shared,

    /// Exclusive mutable access through this borrow.
    Exclusive,
}

/// Ownership classification for a ZUIR value.
///
/// Extension kinds allow dialects to define additional ownership models
/// without requiring a universal closed list of domain-specific variants.
///
/// The meaning of an extension must be specified by its owning dialect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnershipKind {
    /// Ordinary values whose ownership is not restricted by this model.
    Unrestricted,

    /// Values that may be duplicated according to their type semantics.
    Copy,

    /// Values that may be transferred but not implicitly duplicated.
    MoveOnly,

    /// Values that must be consumed exactly once on every valid execution
    /// path, subject to the owning dialect's semantics.
    Linear,

    /// Values that may be consumed at most once.
    Affine,

    /// A borrowed reference with an explicit access mode.
    Borrowed {
        mutability: BorrowMutability,
    },

    /// An ownership kind defined by an extension or dialect.
    Extension(QualifiedName),
}

/// Capabilities associated with an ownership contract.
///
/// These flags describe permitted operations. They do not prove that a
/// program's control flow obeys the corresponding rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipCapabilities {
    /// Whether the value may be copied.
    pub may_copy: bool,

    /// Whether ownership may be transferred.
    pub may_move: bool,

    /// Whether a shared borrow may be created.
    pub may_borrow_shared: bool,

    /// Whether an exclusive borrow may be created.
    pub may_borrow_exclusive: bool,

    /// Whether consumption is required on every valid execution path.
    pub must_consume: bool,

    /// Whether the value may be dropped without explicit consumption.
    pub may_drop: bool,
}

impl OwnershipCapabilities {
    /// Capabilities for unrestricted values.
    pub const fn unrestricted() -> Self {
        Self {
            may_copy: true,
            may_move: true,
            may_borrow_shared: true,
            may_borrow_exclusive: true,
            must_consume: false,
            may_drop: true,
        }
    }

    /// Capabilities for copyable values.
    pub const fn copy() -> Self {
        Self {
            may_copy: true,
            may_move: true,
            may_borrow_shared: true,
            may_borrow_exclusive: true,
            must_consume: false,
            may_drop: true,
        }
    }

    /// Capabilities for move-only values.
    pub const fn move_only() -> Self {
        Self {
            may_copy: false,
            may_move: true,
            may_borrow_shared: true,
            may_borrow_exclusive: true,
            must_consume: false,
            may_drop: true,
        }
    }

    /// Capabilities for linear values.
    pub const fn linear() -> Self {
        Self {
            may_copy: false,
            may_move: true,
            may_borrow_shared: true,
            may_borrow_exclusive: true,
            must_consume: true,
            may_drop: false,
        }
    }

    /// Capabilities for affine values.
    pub const fn affine() -> Self {
        Self {
            may_copy: false,
            may_move: true,
            may_borrow_shared: true,
            may_borrow_exclusive: true,
            must_consume: false,
            may_drop: true,
        }
    }

    /// Capabilities for shared borrows.
    pub const fn shared_borrow() -> Self {
        Self {
            may_copy: false,
            may_move: false,
            may_borrow_shared: true,
            may_borrow_exclusive: false,
            must_consume: false,
            may_drop: true,
        }
    }

    /// Capabilities for exclusive borrows.
    pub const fn exclusive_borrow() -> Self {
        Self {
            may_copy: false,
            may_move: false,
            may_borrow_shared: false,
            may_borrow_exclusive: true,
            must_consume: false,
            may_drop: true,
        }
    }

    /// Validate contradictions that can be detected from the capability
    /// flags alone.
    pub fn validate(self) -> OwnershipResult<()> {
        if self.must_consume && self.may_drop {
            return Err(OwnershipError::InvalidCapabilities(
                "a value cannot simultaneously require consumption and allow \
                 implicit dropping"
                    .to_owned(),
            ));
        }

        Ok(())
    }
}

/// Validate the relationship between an ownership kind and its capabilities.
///
/// This prevents a caller from labelling a value `MoveOnly` while granting
/// copying, or labelling a value `Linear` while permitting implicit dropping.
fn validate_kind_capabilities(
    kind: &OwnershipKind,
    capabilities: OwnershipCapabilities,
) -> OwnershipResult<()> {
    let invalid = |reason: &str| OwnershipError::InvalidKindCapabilities {
        kind: format!("{kind:?}"),
        reason: reason.to_owned(),
    };

    match kind {
        OwnershipKind::Unrestricted => {}

        OwnershipKind::Copy => {
            if !capabilities.may_copy {
                return Err(invalid("copy ownership must permit copying"));
            }

            if capabilities.must_consume {
                return Err(invalid(
                    "copy ownership cannot require linear consumption",
                ));
            }
        }

        OwnershipKind::MoveOnly => {
            if capabilities.may_copy {
                return Err(invalid(
                    "move-only ownership must prohibit copying",
                ));
            }
        }

        OwnershipKind::Linear => {
            if capabilities.may_copy
                || !capabilities.must_consume
                || capabilities.may_drop
            {
                return Err(invalid(
                    "linear ownership must prohibit copying, require \
                     consumption, and prohibit implicit dropping",
                ));
            }
        }

        OwnershipKind::Affine => {
            if capabilities.may_copy || capabilities.must_consume {
                return Err(invalid(
                    "affine ownership must prohibit copying and must not \
                     require consumption on every path",
                ));
            }
        }

        OwnershipKind::Borrowed { mutability } => {
            if capabilities.may_copy || capabilities.may_move {
                return Err(invalid(
                    "borrowed values cannot independently grant copy or \
                     move ownership",
                ));
            }

            match mutability {
                BorrowMutability::Shared => {
                    if !capabilities.may_borrow_shared
                        || capabilities.may_borrow_exclusive
                    {
                        return Err(invalid(
                            "shared borrows must permit shared access and \
                             prohibit exclusive borrowing",
                        ));
                    }
                }

                BorrowMutability::Exclusive => {
                    if capabilities.may_borrow_shared
                        || !capabilities.may_borrow_exclusive
                    {
                        return Err(invalid(
                            "exclusive borrows must permit exclusive access \
                             and prohibit shared borrowing",
                        ));
                    }
                }
            }
        }

        OwnershipKind::Extension(name) => {
            name.validate().map_err(|error| {
                OwnershipError::InvalidExtensionName(format!("{error:?}"))
            })?;
        }
    }

    Ok(())
}

/// Ownership contract for a single value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipContract {
    /// Value governed by this contract.
    pub value: ValueId,

    /// Ownership classification.
    pub kind: OwnershipKind,

    /// Permitted ownership operations.
    pub capabilities: OwnershipCapabilities,

    /// Region in which the contract was declared, if region-scoped.
    pub declared_in: Option<RegionId>,
}

impl OwnershipContract {
    /// Construct and validate an ownership contract.
    pub fn new(
        value: ValueId,
        kind: OwnershipKind,
        capabilities: OwnershipCapabilities,
        declared_in: Option<RegionId>,
    ) -> OwnershipResult<Self> {
        validate_kind(&kind)?;
        capabilities.validate()?;
        validate_kind_capabilities(&kind, capabilities)?;

        Ok(Self {
            value,
            kind,
            capabilities,
            declared_in,
        })
    }

    /// Validate this contract.
    pub fn validate(&self) -> OwnershipResult<()> {
        validate_kind(&self.kind)?;
        self.capabilities.validate()?;
        validate_kind_capabilities(&self.kind, self.capabilities)
    }
}

/// Validate kind-specific metadata.
fn validate_kind(kind: &OwnershipKind) -> OwnershipResult<()> {
    if let OwnershipKind::Extension(name) = kind {
        name.validate().map_err(|error| {
            OwnershipError::InvalidExtensionName(format!("{error:?}"))
        })?;
    }

    Ok(())
}

/// Ownership-related use of a value by an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnershipUseKind {
    /// Read the value without transferring ownership.
    Read,

    /// Mutate the value.
    Write,

    /// Transfer ownership to another destination.
    Move,

    /// Create or use a shared borrow.
    BorrowShared,

    /// Create or use an exclusive borrow.
    BorrowExclusive,

    /// Explicitly drop or discard the value.
    Drop,

    /// Capture the value through shared access.
    CaptureShared,

    /// Capture the value through exclusive access.
    CaptureExclusive,

    /// A dialect-defined ownership event.
    Extension(QualifiedName),
}

impl OwnershipUseKind {
    fn validate(&self) -> OwnershipResult<()> {
        if let Self::Extension(name) = self {
            name.validate().map_err(|error| {
                OwnershipError::InvalidExtensionName(format!("{error:?}"))
            })?;
        }

        Ok(())
    }
}

/// Recorded ownership event associated with an operation.
///
/// The enclosing function and region structure can be consulted by a verifier
/// to establish broader control-flow and lifetime properties.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipUse {
    /// Value being used.
    pub value: ValueId,

    /// Operation responsible for the event.
    pub operation: OperationId,

    /// Block containing the operation.
    pub block: BlockId,

    /// Kind of ownership event.
    pub kind: OwnershipUseKind,
}

impl OwnershipUse {
    /// Construct a use event.
    pub fn new(
        value: ValueId,
        operation: OperationId,
        block: BlockId,
        kind: OwnershipUseKind,
    ) -> OwnershipResult<Self> {
        kind.validate()?;

        Ok(Self {
            value,
            operation,
            block,
            kind,
        })
    }
}

/// Deterministic index of ownership contracts and recorded use events.
///
/// Contract uniqueness is enforced per `ValueId`. Multiple use events are
/// permitted for the same value and operation; a higher-level verifier may
/// reject duplicates when the operation semantics require uniqueness.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipIndex {
    contracts: BTreeMap<ValueId, OwnershipContract>,
    uses: BTreeMap<ValueId, Vec<OwnershipUse>>,
}

impl OwnershipIndex {
    /// Create an empty ownership index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Declare an ownership contract.
    ///
    /// A failed declaration leaves the index unchanged.
    pub fn declare(
        &mut self,
        contract: OwnershipContract,
    ) -> OwnershipResult<()> {
        contract.validate()?;

        if self.contracts.contains_key(&contract.value) {
            return Err(OwnershipError::DuplicateContract(contract.value));
        }

        self.contracts.insert(contract.value, contract);
        Ok(())
    }

    /// Record an ownership-use event.
    ///
    /// The referenced value must already have a contract. Events are kept in
    /// deterministic order, independent of insertion order.
    pub fn record_use(&mut self, usage: OwnershipUse) -> OwnershipResult<()> {
        usage.kind.validate()?;

        if !self.contracts.contains_key(&usage.value) {
            return Err(OwnershipError::UndeclaredValue(usage.value));
        }

        let entries = self.uses.entry(usage.value).or_default();
        entries.push(usage);

        entries.sort_by(|left, right| {
            (
                left.block,
                left.operation,
                format!("{:?}", left.kind),
            )
                .cmp(&(
                    right.block,
                    right.operation,
                    format!("{:?}", right.kind),
                ))
        });

        Ok(())
    }

    /// Look up a value's ownership contract.
    pub fn contract(&self, value: ValueId) -> Option<&OwnershipContract> {
        self.contracts.get(&value)
    }

    /// Iterate over all declared contracts in deterministic key order.
    pub fn contracts(
        &self,
    ) -> impl Iterator<Item = (&ValueId, &OwnershipContract)> {
        self.contracts.iter()
    }

    /// Return recorded uses for a value in deterministic order.
    ///
    /// Returns an empty slice when the value has no recorded uses.
    pub fn uses_of(&self, value: ValueId) -> &[OwnershipUse] {
        self.uses.get(&value).map_or(&[], Vec::as_slice)
    }

    /// Iterate over all use lists in deterministic value-key order.
    pub fn all_uses(
        &self,
    ) -> impl Iterator<Item = (&ValueId, &[OwnershipUse])> {
        self.uses
            .iter()
            .map(|(value, uses)| (value, uses.as_slice()))
    }

    /// Number of declared ownership contracts.
    pub fn contract_count(&self) -> usize {
        self.contracts.len()
    }

    /// Number of recorded ownership-use events.
    pub fn use_count(&self) -> usize {
        self.uses.values().map(Vec::len).sum()
    }

    /// Whether no ownership contracts or use events are recorded.
    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty() && self.uses.is_empty()
    }

    /// Validate contracts and all indexed use events.
    pub fn validate(&self) -> OwnershipResult<()> {
        self.verify_internal_consistency()?;

        for contract in self.contracts.values() {
            contract.validate()?;
        }

        for uses in self.uses.values() {
            for usage in uses {
                usage.kind.validate()?;
            }
        }

        Ok(())
    }

    /// Check consistency between index keys, contracts, and use events.
    ///
    /// This verifies structural consistency only. It does not establish
    /// linearity, borrow safety, lifetime validity, or correct use ordering
    /// across control-flow paths.
    pub fn verify_internal_consistency(&self) -> OwnershipResult<()> {
        for (key, contract) in &self.contracts {
            if *key != contract.value {
                return Err(OwnershipError::ContractKeyMismatch {
                    key: *key,
                    contract_value: contract.value,
                });
            }
        }

        for (key, uses) in &self.uses {
            if !self.contracts.contains_key(key) {
                return Err(OwnershipError::UsesWithoutContract(*key));
            }

            for usage in uses {
                if *key != usage.value {
                    return Err(OwnershipError::UseKeyMismatch {
                        key: *key,
                        use_value: usage.value,
                    });
                }
            }
        }

        Ok(())
    }

    /// Remove all contracts and recorded use events.
    pub fn clear(&mut self) {
        self.contracts.clear();
        self.uses.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Keep ID construction centralized because ID representation belongs to
    // the shared ZUIR ID module.
    //
    // If the repository uses non-tuple ID constructors, adapt these helpers
    // to that authoritative ID API rather than introducing local ID types.

    fn value_id(raw: u64) -> ValueId {
        ValueId(raw)
    }

    fn block_id(raw: u64) -> BlockId {
        BlockId(raw)
    }

    fn operation_id(raw: u64) -> OperationId {
        OperationId(raw)
    }

    fn shared_contract(value: ValueId) -> OwnershipContract {
        OwnershipContract::new(
            value,
            OwnershipKind::Borrowed {
                mutability: BorrowMutability::Shared,
            },
            OwnershipCapabilities::shared_borrow(),
            None,
        )
        .expect("shared-borrow contract should be valid")
    }

    #[test]
    fn accepts_a_valid_move_only_contract() {
        let contract = OwnershipContract::new(
            value_id(1),
            OwnershipKind::MoveOnly,
            OwnershipCapabilities::move_only(),
            None,
        );

        assert!(contract.is_ok());
    }

    #[test]
    fn rejects_copy_capability_for_move_only_values() {
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
    fn rejects_droppable_linear_values() {
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
    fn rejects_affine_values_that_require_consumption() {
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
    fn rejects_contradictory_borrow_capabilities() {
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
    fn rejects_duplicate_contracts() {
        let value = value_id(1);
        let mut index = OwnershipIndex::new();

        index
            .declare(shared_contract(value))
            .expect("first declaration should succeed");

        let result = index.declare(shared_contract(value));

        assert_eq!(
            result,
            Err(OwnershipError::DuplicateContract(value))
        );
    }

    #[test]
    fn rejects_uses_of_undeclared_values() {
        let mut index = OwnershipIndex::new();

        let usage = OwnershipUse::new(
            value_id(99),
            operation_id(1),
            block_id(1),
            OwnershipUseKind::Read,
        )
        .expect("read event should be valid");

        assert_eq!(
            index.record_use(usage),
            Err(OwnershipError::UndeclaredValue(value_id(99)))
        );
    }

    #[test]
    fn records_and_returns_uses() {
        let value = value_id(1);
        let mut index = OwnershipIndex::new();

        index
            .declare(shared_contract(value))
            .expect("declaration should succeed");

        index
            .record_use(
                OwnershipUse::new(
                    value,
                    operation_id(2),
                    block_id(1),
                    OwnershipUseKind::Read,
                )
                .expect("use should be valid"),
            )
            .expect("recording should succeed");

        assert_eq!(index.uses_of(value).len(), 1);
        assert_eq!(index.use_count(), 1);
    }

    #[test]
    fn use_order_is_deterministic() {
        let value = value_id(1);
        let mut index = OwnershipIndex::new();

        index
            .declare(shared_contract(value))
            .expect("declaration should succeed");

        for operation in [operation_id(3), operation_id(1), operation_id(2)] {
            index
                .record_use(
                    OwnershipUse::new(
                        value,
                        operation,
                        block_id(1),
                        OwnershipUseKind::Read,
                    )
                    .expect("use should be valid"),
                )
                .expect("recording should succeed");
        }

        let operations: Vec<_> = index
            .uses_of(value)
            .iter()
            .map(|usage| usage.operation)
            .collect();

        assert_eq!(
            operations,
            vec![operation_id(1), operation_id(2), operation_id(3)]
        );
    }

    #[test]
    fn clear_removes_contracts_and_uses() {
        let value = value_id(1);
        let mut index = OwnershipIndex::new();

        index
            .declare(shared_contract(value))
            .expect("declaration should succeed");

        index
            .record_use(
                OwnershipUse::new(
                    value,
                    operation_id(1),
                    block_id(1),
                    OwnershipUseKind::Read,
                )
                .expect("use should be valid"),
            )
            .expect("recording should succeed");

        index.clear();

        assert!(index.is_empty());
        assert_eq!(index.contract_count(), 0);
        assert_eq!(index.use_count(), 0);
    }
}