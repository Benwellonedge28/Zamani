
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Basic Block
//!
//! Path: `src/zuir/ir/block.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: `unsafe` is forbidden.
//!
//! PURPOSE
//!
//! Represent a target-independent basic block containing typed block
//! arguments, ordered operation references, and an explicit terminator.
//!
//! OWNERSHIP
//!
//! This module owns BlockArgument, SwitchCase, BlockTerminator, ZuirBlock,
//! block-local errors, and block-local validation.
//!
//! It does not own operation payloads, successor-edge storage, region
//! contents, ID allocation, type definitions, or whole-program verification.
//!
//! INTEGRATION
//!
//! - `ids.rs` owns all canonical identifiers.
//! - `successor.rs` owns `Successor`; this module re-exports that type for
//!   compatibility but does not define a duplicate.
//! - `operation.rs` owns operation payloads. Blocks store `OperationId`s.
//! - `region.rs` owns ordered block membership and region ownership.
//! - `schema/attributes.rs` owns attribute values.
//! - `schema/types.rs` owns qualified names.
//! - The full verifier owns reference resolution, SSA, CFG, type, effect,
//!   ownership, and dialect-specific verification.
//! - The grammar and semantic model own source syntax and source meaning.
//!
//! SCALABILITY
//!
//! Collections grow dynamically. There is no universal maximum block,
//! argument, operation, successor, qubit, register, thread, device, or
//! tensor-rank count. Fallible reservation methods report capacity-reservation
//! failures where the standard library exposes them. Actual processing is
//! bounded by available resources; literal infinity is not promised.
//!
//! TRUST BOUNDARY
//!
//! Local validation does not prove whole-program validity. A decoded block
//! must pass local validation and then the authoritative full-IR verifier
//! before trusted optimization, lowering, or execution.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

use crate::zuir::ids::{BlockId, OperationId, RegionId, TypeId, ValueId};
use crate::zuir::schema::attributes::AttributeValue;
use crate::zuir::schema::types::QualifiedName;

/// Canonical successor edge implementation from its owning module.
///
/// Re-exported so callers using the historical `block::Successor` path do
/// not need to define or maintain a second successor type.
pub use crate::zuir::ir::successor::Successor;

/// Result type for block-local construction, mutation, and validation.
pub type BlockResult<T> = Result<T, BlockError>;

/// Errors detectable from a block and its locally owned data.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BlockError {
    /// An operation identity occurs more than once in the block.
    DuplicateOperation(OperationId),

    /// A block argument value identity occurs more than once.
    DuplicateArgument(ValueId),

    /// An insertion index exceeds the current operation count.
    InvalidOperationPosition {
        position: usize,
        operation_count: usize,
    },

    /// A mutation attempted to append or insert after a closed terminator.
    BlockAlreadyTerminated,

    /// A terminator is invalid for the requested operation.
    InvalidTerminator(String),

    /// A qualified extension name is malformed.
    InvalidExtensionName(String),

    /// A complete block has no parent region.
    MissingParentRegion,

    /// A switch contains semantically equal case values.
    DuplicateSwitchCase,

    /// Dynamic collection capacity could not be reserved.
    AllocationFailure {
        collection: &'static str,
    },

    /// A local block invariant is invalid.
    InvalidBlock(String),
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateOperation(id) => {
                write!(f, "operation {id} occurs more than once in the block")
            }
            Self::DuplicateArgument(id) => {
                write!(f, "block argument value {id} occurs more than once")
            }
            Self::InvalidOperationPosition {
                position,
                operation_count,
            } => write!(
                f,
                "operation insertion position {position} exceeds \
                 operation count {operation_count}"
            ),
            Self::BlockAlreadyTerminated => {
                f.write_str("cannot append or insert an operation after termination")
            }
            Self::InvalidTerminator(reason) => {
                write!(f, "invalid block terminator: {reason}")
            }
            Self::InvalidExtensionName(reason) => {
                write!(f, "invalid terminator extension name: {reason}")
            }
            Self::MissingParentRegion => {
                f.write_str("a complete block must identify its parent region")
            }
            Self::DuplicateSwitchCase => {
                f.write_str("switch terminator contains duplicate case values")
            }
            Self::AllocationFailure { collection } => {
                write!(f, "unable to reserve memory for block {collection}")
            }
            Self::InvalidBlock(reason) => write!(f, "invalid block: {reason}"),
        }
    }
}

impl std::error::Error for BlockError {}

// =============================================================================
// Block arguments
// =============================================================================

/// A typed SSA value entering a block.
///
/// `value` is the canonical value identity and `ty` references the semantic
/// type table. An optional role provides extensible metadata; its meaning is
/// determined by the applicable dialect or verifier contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockArgument {
    value: ValueId,
    ty: TypeId,
    role: Option<QualifiedName>,
}

impl BlockArgument {
    /// Create an argument from resolved value and type identities.
    ///
    /// This constructor does not resolve the IDs or prove their global
    /// uniqueness. The enclosing IR verifier owns those checks.
    #[must_use]
    pub const fn new(value: ValueId, ty: TypeId) -> Self {
        Self {
            value,
            ty,
            role: None,
        }
    }

    /// Create an argument with a validated, qualified role.
    pub fn with_role(
        value: ValueId,
        ty: TypeId,
        role: QualifiedName,
    ) -> BlockResult<Self> {
        role.validate().map_err(|error| {
            BlockError::InvalidBlock(format!("invalid argument role: {error}"))
        })?;

        Ok(Self {
            value,
            ty,
            role: Some(role),
        })
    }

    /// Return the value identity.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Return the semantic type identity.
    #[must_use]
    pub const fn ty(&self) -> TypeId {
        self.ty
    }

    /// Return the optional qualified role.
    #[must_use]
    pub fn role(&self) -> Option<&QualifiedName> {
        self.role.as_ref()
    }

    /// Replace the optional role after validating the candidate.
    ///
    /// If validation fails, the current role remains unchanged.
    pub fn set_role(
        &mut self,
        role: Option<QualifiedName>,
    ) -> BlockResult<()> {
        if let Some(candidate) = &role {
            candidate.validate().map_err(|error| {
                BlockError::InvalidBlock(format!(
                    "invalid argument role: {error}"
                ))
            })?;
        }

        self.role = role;
        Ok(())
    }

    /// Validate metadata owned by this argument.
    pub fn validate(&self) -> BlockResult<()> {
        if let Some(role) = &self.role {
            role.validate().map_err(|error| {
                BlockError::InvalidBlock(format!(
                    "invalid argument role: {error}"
                ))
            })?;
        }

        Ok(())
    }
}

// =============================================================================
// Switch cases and terminators
// =============================================================================

/// A switch case value and the edge selected by that value.
///
/// Attribute values are reused from the schema layer; this module does not
/// introduce a second constant or literal representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchCase {
    value: AttributeValue,
    successor: Successor,
}

impl SwitchCase {
    /// Construct a switch case.
    ///
    /// Call `validate()` before trusting externally supplied values.
    #[must_use]
    pub fn new(value: AttributeValue, successor: Successor) -> Self {
        Self { value, successor }
    }

    /// Return the case value.
    #[must_use]
    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    /// Return the selected successor.
    #[must_use]
    pub fn successor(&self) -> &Successor {
        &self.successor
    }

    /// Validate the case value's local schema invariants.
    pub fn validate(&self) -> BlockResult<()> {
        self.value.validate().map_err(|error| {
            BlockError::InvalidTerminator(format!(
                "invalid switch case value: {error}"
            ))
        })
    }
}

/// The explicit control-flow termination contract for a block.
///
/// `Open` is a construction state and is not a finalized terminator.
/// Extension terminators require a qualified name and must be interpreted
/// only by a registered, trusted extension contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BlockTerminator {
    /// The block is still being assembled.
    Open,

    /// Return from the enclosing callable computation.
    Return {
        /// Values returned in declared order.
        values: Vec<ValueId>,
    },

    /// Unconditional transfer to another block.
    Branch {
        /// Destination and ordered edge arguments.
        successor: Successor,
    },

    /// Conditional transfer to one of two destinations.
    ConditionalBranch {
        /// Value used as the condition.
        condition: ValueId,
        /// Destination selected when the condition is true.
        then_successor: Successor,
        /// Destination selected when the condition is false.
        else_successor: Successor,
    },

    /// Multiway transfer selected by a value.
    Switch {
        /// Value used to select a case.
        selector: ValueId,
        /// Destination selected when no case matches.
        default_successor: Successor,
        /// Case values and their destinations, in declared order.
        cases: Vec<SwitchCase>,
    },

    /// Terminate the current path without returning normally.
    Unreachable,

    /// A terminator defined by a registered extension.
    Extension {
        /// Qualified extension operation name.
        name: QualifiedName,
        /// Explicit outgoing edges, in declared order.
        successors: Vec<Successor>,
    },
}

impl Default for BlockTerminator {
    fn default() -> Self {
        Self::Open
    }
}

impl BlockTerminator {
    /// Whether this terminator is still open for construction.
    #[must_use]
    pub const fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }

    /// Whether this terminator closes the block.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        !self.is_open()
    }

    /// Count explicitly represented outgoing edges.
    ///
    /// This is structural information only. An extension's actual control
    /// flow must be checked against its registered contract.
    #[must_use]
    pub fn successor_count(&self) -> usize {
        match self {
            Self::Open | Self::Return { .. } | Self::Unreachable => 0,
            Self::Branch { .. } => 1,
            Self::ConditionalBranch { .. } => 2,
            Self::Switch { cases, .. } => 1 + cases.len(),
            Self::Extension { successors, .. } => successors.len(),
        }
    }

    /// Visit explicit successors in declared order without allocating.
    pub fn for_each_successor(&self, mut visit: impl FnMut(&Successor)) {
        match self {
            Self::Branch { successor } => visit(successor),

            Self::ConditionalBranch {
                then_successor,
                else_successor,
                ..
            } => {
                visit(then_successor);
                visit(else_successor);
            }

            Self::Switch {
                default_successor,
                cases,
                ..
            } => {
                visit(default_successor);

                for case in cases {
                    visit(case.successor());
                }
            }

            Self::Extension { successors, .. } => {
                for successor in successors {
                    visit(successor);
                }
            }

            Self::Open | Self::Return { .. } | Self::Unreachable => {}
        }
    }

    /// Validate terminator-local invariants.
    ///
    /// This does not resolve referenced blocks or values, check dominance,
    /// compare edge arguments to destination arguments, or authorize an
    /// extension. Those checks belong to the full-IR verifier.
    pub fn validate(&self) -> BlockResult<()> {
        match self {
            Self::Open
            | Self::Return { .. }
            | Self::Branch { .. }
            | Self::ConditionalBranch { .. }
            | Self::Unreachable => Ok(()),

            Self::Switch { cases, .. } => {
                for (index, case) in cases.iter().enumerate() {
                    case.validate()?;

                    // AttributeValue has semantic equality but is not required
                    // to implement Ord or Hash. Preserve that contract here.
                    // A schema-owned canonical-key facility can optimize this
                    // if large switches make pairwise comparison expensive.
                    if cases[..index]
                        .iter()
                        .any(|previous| previous.value() == case.value())
                    {
                        return Err(BlockError::DuplicateSwitchCase);
                    }
                }

                Ok(())
            }

            Self::Extension { name, .. } => {
                name.validate().map_err(|error| {
                    BlockError::InvalidExtensionName(error.to_string())
                })
            }
        }
    }
}

// =============================================================================
// Canonical block and validated deserialization
// =============================================================================

/// Private wire representation used to validate decoded block-local state.
#[derive(Deserialize)]
struct ZuirBlockWire {
    id: BlockId,
    parent_region: Option<RegionId>,
    arguments: Vec<BlockArgument>,
    operations: Vec<OperationId>,
    terminator: BlockTerminator,
}

/// Canonical target-independent basic block.
///
/// The block owns its ordered argument and operation-reference lists. The
/// actual operation definitions are owned by the enclosing IR storage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ZuirBlock {
    id: BlockId,
    parent_region: Option<RegionId>,
    arguments: Vec<BlockArgument>,
    operations: Vec<OperationId>,
    terminator: BlockTerminator,
}

impl<'de> Deserialize<'de> for ZuirBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = ZuirBlockWire::deserialize(deserializer)?;

        let block = Self {
            id: wire.id,
            parent_region: wire.parent_region,
            arguments: wire.arguments,
            operations: wire.operations,
            terminator: wire.terminator,
        };

        block.validate().map_err(serde::de::Error::custom)?;
        Ok(block)
    }
}

impl ZuirBlock {
    /// Create an empty block under construction.
    #[must_use]
    pub fn new(id: BlockId) -> Self {
        Self {
            id,
            parent_region: None,
            arguments: Vec::new(),
            operations: Vec::new(),
            terminator: BlockTerminator::Open,
        }
    }

    /// Create an empty block associated with a parent region.
    #[must_use]
    pub fn in_region(id: BlockId, parent_region: RegionId) -> Self {
        Self {
            id,
            parent_region: Some(parent_region),
            arguments: Vec::new(),
            operations: Vec::new(),
            terminator: BlockTerminator::Open,
        }
    }

    /// Return the block identity.
    #[must_use]
    pub const fn id(&self) -> BlockId {
        self.id
    }

    /// Return the parent region, if assigned.
    #[must_use]
    pub const fn parent_region(&self) -> Option<RegionId> {
        self.parent_region
    }

    /// Assign the parent region.
    ///
    /// The full verifier must check that this region also lists the block
    /// and that the region/block relationship is reciprocal.
    pub fn set_parent_region(&mut self, region: RegionId) {
        self.parent_region = Some(region);
    }

    /// Clear the parent-region association.
    ///
    /// This is useful during detached construction or a structural rewrite.
    /// A complete block must have a parent region before final verification.
    pub fn clear_parent_region(&mut self) -> Option<RegionId> {
        self.parent_region.take()
    }

    /// Borrow block arguments in declared order.
    #[must_use]
    pub fn arguments(&self) -> &[BlockArgument] {
        &self.arguments
    }

    /// Return the number of block arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Borrow operation identities in semantic order.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Return the number of operation references.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Borrow the current terminator.
    #[must_use]
    pub fn terminator(&self) -> &BlockTerminator {
        &self.terminator
    }

    /// Whether the block has a closed terminator.
    #[must_use]
    pub fn is_terminated(&self) -> bool {
        self.terminator.is_closed()
    }

    /// Reserve capacity for additional block arguments.
    pub fn try_reserve_arguments(&mut self, additional: usize) -> BlockResult<()> {
        self.arguments
            .try_reserve(additional)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "arguments",
            })
    }

    /// Reserve capacity for additional operation references.
    pub fn try_reserve_operations(&mut self, additional: usize) -> BlockResult<()> {
        self.operations
            .try_reserve(additional)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "operations",
            })
    }

    /// Add a block argument, rejecting duplicate local value identities.
    pub fn add_argument(&mut self, argument: BlockArgument) -> BlockResult<()> {
        argument.validate()?;

        if self
            .arguments
            .iter()
            .any(|existing| existing.value() == argument.value())
        {
            return Err(BlockError::DuplicateArgument(argument.value()));
        }

        self.try_reserve_arguments(1)?;
        self.arguments.push(argument);
        Ok(())
    }

    /// Replace all arguments after validating them and checking uniqueness.
    ///
    /// If validation fails, the current argument list remains unchanged.
    pub fn set_arguments(
        &mut self,
        arguments: Vec<BlockArgument>,
    ) -> BlockResult<()> {
        let mut seen = BTreeSet::new();

        for argument in &arguments {
            argument.validate()?;

            if !seen.insert(argument.value()) {
                return Err(BlockError::DuplicateArgument(argument.value()));
            }
        }

        self.arguments = arguments;
        Ok(())
    }

    /// Append an operation reference.
    ///
    /// The referenced operation must exist in the owning IR. The full
    /// verifier must also ensure that it is owned by exactly one block.
    pub fn append_operation(&mut self, operation: OperationId) -> BlockResult<()> {
        if self.is_terminated() {
            return Err(BlockError::BlockAlreadyTerminated);
        }

        if self.operations.contains(&operation) {
            return Err(BlockError::DuplicateOperation(operation));
        }

        self.try_reserve_operations(1)?;
        self.operations.push(operation);
        Ok(())
    }

    /// Insert an operation at an exact position.
    ///
    /// `position == operation_count()` appends. Positions greater than the
    /// operation count are rejected rather than clamped.
    pub fn insert_operation(
        &mut self,
        position: usize,
        operation: OperationId,
    ) -> BlockResult<()> {
        if self.is_terminated() {
            return Err(BlockError::BlockAlreadyTerminated);
        }

        if position > self.operations.len() {
            return Err(BlockError::InvalidOperationPosition {
                position,
                operation_count: self.operations.len(),
            });
        }

        if self.operations.contains(&operation) {
            return Err(BlockError::DuplicateOperation(operation));
        }

        self.try_reserve_operations(1)?;
        self.operations.insert(position, operation);
        Ok(())
    }

    /// Replace the ordered operation references after local validation.
    ///
    /// This is intended for builders and transformations. The block's
    /// current list is unchanged if the replacement contains duplicates.
    /// Operation existence and cross-block ownership are checked globally.
    pub fn set_operations(
        &mut self,
        operations: Vec<OperationId>,
    ) -> BlockResult<()> {
        if self.is_terminated() && !operations.is_empty() {
            return Err(BlockError::BlockAlreadyTerminated);
        }

        let mut seen = BTreeSet::new();

        for operation in &operations {
            if !seen.insert(*operation) {
                return Err(BlockError::DuplicateOperation(*operation));
            }
        }

        self.operations = operations;
        Ok(())
    }

    /// Remove an operation reference without deleting its stored payload.
    ///
    /// Returns `true` if the reference was found. Removing an operation
    /// requires whole-IR re-verification because uses and control flow may
    /// have become invalid.
    pub fn remove_operation(&mut self, operation: OperationId) -> bool {
        if let Some(index) = self
            .operations
            .iter()
            .position(|existing| *existing == operation)
        {
            self.operations.remove(index);
            true
        } else {
            false
        }
    }

    /// Set the block terminator if the block is not already terminated.
    pub fn set_terminator(
        &mut self,
        terminator: BlockTerminator,
    ) -> BlockResult<()> {
        if self.is_terminated() {
            return Err(BlockError::BlockAlreadyTerminated);
        }

        terminator.validate()?;

        if terminator.is_open() {
            return Err(BlockError::InvalidTerminator(
                "set_terminator requires a closed terminator".to_owned(),
            ));
        }

        self.terminator = terminator;
        Ok(())
    }

    /// Replace the terminator during an explicit control-flow transformation.
    ///
    /// This method allows replacing an already-closed terminator but never
    /// installs `Open`. The enclosing IR must be reverified after replacement.
    pub fn replace_terminator(
        &mut self,
        terminator: BlockTerminator,
    ) -> BlockResult<()> {
        terminator.validate()?;

        if terminator.is_open() {
            return Err(BlockError::InvalidTerminator(
                "replacement terminator must be closed".to_owned(),
            ));
        }

        self.terminator = terminator;
        Ok(())
    }

    /// Reopen a block for a deliberate transformation.
    ///
    /// This discards the old terminator. It must only be used while the
    /// enclosing IR is in a mutable construction/transformation phase.
    pub fn clear_terminator(&mut self) -> BlockTerminator {
        std::mem::replace(&mut self.terminator, BlockTerminator::Open)
    }

    /// Validate block-local invariants while allowing construction to remain
    /// incomplete.
    ///
    /// This checks argument uniqueness, operation-reference uniqueness,
    /// argument metadata, and terminator-local shape. It does not resolve
    /// IDs or prove global SSA/CFG correctness.
    pub fn validate(&self) -> BlockResult<()> {
        let mut argument_ids = BTreeSet::new();

        for argument in &self.arguments {
            argument.validate()?;

            if !argument_ids.insert(argument.value()) {
                return Err(BlockError::DuplicateArgument(argument.value()));
            }
        }

        let mut operation_ids = BTreeSet::new();

        for operation in &self.operations {
            if !operation_ids.insert(*operation) {
                return Err(BlockError::DuplicateOperation(*operation));
            }
        }

        self.terminator.validate()
    }

    /// Validate the block's locally complete state.
    ///
    /// A complete block requires a parent region and a closed terminator.
    /// The full verifier must still resolve references and verify CFG, SSA,
    /// types, effects, ownership, and dialect-specific invariants.
    pub fn validate_complete(&self) -> BlockResult<()> {
        self.validate()?;

        if self.parent_region.is_none() {
            return Err(BlockError::MissingParentRegion);
        }

        if self.terminator.is_open() {
            return Err(BlockError::InvalidBlock(
                "complete block has an open terminator".to_owned(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ids::{BlockId, OperationId, RegionId, TypeId, ValueId};

    fn block_id(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test block ID must be nonzero")
    }

    fn region_id(raw: u128) -> RegionId {
        RegionId::new(raw).expect("test region ID must be nonzero")
    }

    fn operation_id(raw: u128) -> OperationId {
        OperationId::new(raw).expect("test operation ID must be nonzero")
    }

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn type_id(raw: u128) -> TypeId {
        TypeId::new(raw).expect("test type ID must be nonzero")
    }

    #[test]
    fn new_block_is_open_and_locally_valid() {
        let block = ZuirBlock::new(block_id(1));

        assert_eq!(block.id(), block_id(1));
        assert!(block.parent_region().is_none());
        assert!(block.terminator().is_open());
        assert!(block.validate().is_ok());
        assert!(block.validate_complete().is_err());
    }

    #[test]
    fn block_can_be_created_in_a_region() {
        let block = ZuirBlock::in_region(block_id(1), region_id(9));
        assert_eq!(block.parent_region(), Some(region_id(9)));
    }

    #[test]
    fn argument_order_is_preserved() {
        let mut block = ZuirBlock::new(block_id(1));

        block
            .add_argument(BlockArgument::new(value_id(1), type_id(11)))
            .unwrap();
        block
            .add_argument(BlockArgument::new(value_id(2), type_id(12)))
            .unwrap();

        assert_eq!(block.arguments()[0].value(), value_id(1));
        assert_eq!(block.arguments()[1].value(), value_id(2));
    }

    #[test]
    fn duplicate_arguments_are_rejected() {
        let mut block = ZuirBlock::new(block_id(1));
        let argument = BlockArgument::new(value_id(1), type_id(11));

        block.add_argument(argument.clone()).unwrap();

        assert_eq!(
            block.add_argument(argument),
            Err(BlockError::DuplicateArgument(value_id(1)))
        );
    }

    #[test]
    fn replacing_arguments_is_atomic_on_error() {
        let mut block = ZuirBlock::new(block_id(1));
        block
            .add_argument(BlockArgument::new(value_id(1), type_id(11)))
            .unwrap();

        let replacement = vec![
            BlockArgument::new(value_id(2), type_id(11)),
            BlockArgument::new(value_id(2), type_id(12)),
        ];

        assert!(block.set_arguments(replacement).is_err());
        assert_eq!(block.argument_count(), 1);
        assert_eq!(block.arguments()[0].value(), value_id(1));
    }

    #[test]
    fn operation_order_is_preserved() {
        let mut block = ZuirBlock::new(block_id(1));

        block.append_operation(operation_id(2)).unwrap();
        block.append_operation(operation_id(1)).unwrap();

        assert_eq!(
            block.operations(),
            &[operation_id(2), operation_id(1)]
        );
    }

    #[test]
    fn duplicate_operations_are_rejected() {
        let mut block = ZuirBlock::new(block_id(1));
        block.append_operation(operation_id(3)).unwrap();

        assert_eq!(
            block.append_operation(operation_id(3)),
            Err(BlockError::DuplicateOperation(operation_id(3)))
        );
    }

    #[test]
    fn operation_can_be_inserted_at_a_position() {
        let mut block = ZuirBlock::new(block_id(1));

        block.append_operation(operation_id(1)).unwrap();
        block.append_operation(operation_id(3)).unwrap();
        block.insert_operation(1, operation_id(2)).unwrap();

        assert_eq!(
            block.operations(),
            &[operation_id(1), operation_id(2), operation_id(3)]
        );
    }

    #[test]
    fn invalid_insertion_position_is_rejected() {
        let mut block = ZuirBlock::new(block_id(1));

        assert_eq!(
            block.insert_operation(1, operation_id(1)),
            Err(BlockError::InvalidOperationPosition {
                position: 1,
                operation_count: 0,
            })
        );
    }

    #[test]
    fn removing_operation_removes_only_its_reference() {
        let mut block = ZuirBlock::new(block_id(1));
        block.append_operation(operation_id(1)).unwrap();

        assert!(block.remove_operation(operation_id(1)));
        assert!(!block.remove_operation(operation_id(1)));
        assert!(block.operations().is_empty());
    }

    #[test]
    fn terminated_block_rejects_new_operations() {
        let mut block = ZuirBlock::new(block_id(1));

        block
            .set_terminator(BlockTerminator::Return { values: Vec::new() })
            .unwrap();

        assert_eq!(
            block.append_operation(operation_id(1)),
            Err(BlockError::BlockAlreadyTerminated)
        );
    }

    #[test]
    fn open_terminator_cannot_be_set_as_final() {
        let mut block = ZuirBlock::new(block_id(1));

        assert!(matches!(
            block.set_terminator(BlockTerminator::Open),
            Err(BlockError::InvalidTerminator(_))
        ));
    }

    #[test]
    fn terminator_can_be_replaced_explicitly() {
        let mut block = ZuirBlock::new(block_id(1));

        block
            .set_terminator(BlockTerminator::Return { values: Vec::new() })
            .unwrap();
        block
            .replace_terminator(BlockTerminator::Unreachable)
            .unwrap();

        assert!(matches!(
            block.terminator(),
            BlockTerminator::Unreachable
        ));
    }

    #[test]
    fn branch_successor_is_visible_without_allocation() {
        let target = block_id(2);
        let terminator = BlockTerminator::Branch {
            successor: Successor::new(target),
        };

        let mut observed = Vec::new();
        terminator.for_each_successor(|successor| {
            observed.push(successor.target());
        });

        assert_eq!(observed, vec![target]);
        assert_eq!(terminator.successor_count(), 1);
    }

    #[test]
    fn conditional_branch_has_two_successors() {
        let terminator = BlockTerminator::ConditionalBranch {
            condition: value_id(1),
            then_successor: Successor::new(block_id(2)),
            else_successor: Successor::new(block_id(3)),
        };

        assert_eq!(terminator.successor_count(), 2);

        let mut observed = Vec::new();
        terminator.for_each_successor(|successor| {
            observed.push(successor.target());
        });

        assert_eq!(observed, vec![block_id(2), block_id(3)]);
    }

    #[test]
    fn complete_block_requires_parent_and_closed_terminator() {
        let mut block = ZuirBlock::new(block_id(1));

        assert_eq!(
            block.validate_complete(),
            Err(BlockError::MissingParentRegion)
        );

        block.set_parent_region(region_id(7));

        assert!(matches!(
            block.validate_complete(),
            Err(BlockError::InvalidBlock(_))
        ));

        block
            .set_terminator(BlockTerminator::Unreachable)
            .unwrap();

        assert!(block.validate_complete().is_ok());
    }

    #[test]
    fn serialization_round_trip_preserves_block() {
        let mut block = ZuirBlock::in_region(block_id(1), region_id(7));
        block.append_operation(operation_id(3)).unwrap();
        block
            .set_terminator(BlockTerminator::Return {
                values: vec![value_id(10)],
            })
            .unwrap();

        let encoded = serde_json::to_string(&block).unwrap();
        let decoded: ZuirBlock = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, block);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn fallible_reservation_does_not_change_contents() {
        let mut block = ZuirBlock::new(block_id(1));
        block.append_operation(operation_id(1)).unwrap();

        let before = block.operations().to_vec();
        block.try_reserve_operations(32).unwrap();

        assert_eq!(block.operations(), before.as_slice());
    }
}
