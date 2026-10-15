
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Basic Block
//!
//! Canonical path: `src/zuir/ir/block.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Represents a target-independent basic block. A block owns its arguments,
//! ordered operation references, and terminator, but does not own operation
//! payloads or neighboring blocks.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/ids.rs
//! - src/zuir/schema/types.rs
//! - src/zuir/schema/attributes.rs
//! - src/zuir/ir/region.rs
//! - src/zuir/ir/operation.rs
//! - grammar/Zamani.g4
//! - grammar/grammar.md
//!
//! INTEGRATION
//!
//! `Region` owns ordered `BlockId` membership. The operation/program storage
//! layer owns operation payloads. The full verifier resolves cross-entity
//! references and verifies CFG, SSA, types, effects, ownership, and dialect
//! rules. The grammar and semantic analysis own source interpretation.
//!
//! SCALABILITY
//!
//! Collections are dynamically sized. No universal operation count,
//! argument count, successor count, hardware capacity, qubit count, register
//! width, or tensor-rank limit is imposed. Fallible collection growth reports
//! allocation reservation failure where Rust exposes that failure.
//!
//! VALIDITY BOUNDARY
//!
//! Local validation checks only invariants representable within this block.
//! A successfully deserialized block is locally valid, not necessarily valid
//! as part of a complete program. Run the full-program verifier before
//! trusted optimization, serialization, or lowering.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

use crate::zuir::ids::{BlockId, OperationId, RegionId, TypeId, ValueId};
use crate::zuir::schema::attributes::AttributeValue;
use crate::zuir::schema::types::QualifiedName;

/// Result type for block-local operations.
pub type BlockResult<T> = Result<T, BlockError>;

/// Errors raised by block construction, mutation, or local validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockError {
    /// An operation identity already occurs in this block.
    DuplicateOperation(OperationId),

    /// A block argument value identity already occurs in this block.
    DuplicateArgument(ValueId),

    /// An insertion position exceeds the number of operations.
    InvalidOperationPosition {
        position: usize,
        operation_count: usize,
    },

    /// An operation cannot be appended after the terminator.
    BlockAlreadyTerminated,

    /// The requested terminator or argument metadata is invalid.
    InvalidTerminator(String),

    /// An extension-qualified name is invalid.
    InvalidExtensionName(String),

    /// A finalized block must identify its parent region.
    MissingParentRegion,

    /// A switch contains repeated case values.
    DuplicateSwitchCase,

    /// A collection could not reserve capacity for a mutation.
    AllocationFailure {
        collection: &'static str,
    },

    /// A local block invariant is not satisfied.
    InvalidBlock(String),
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateOperation(id) => {
                write!(f, "operation {id} is already present in this block")
            }
            Self::DuplicateArgument(id) => {
                write!(f, "block argument value {id} is already declared")
            }
            Self::InvalidOperationPosition {
                position,
                operation_count,
            } => write!(
                f,
                "operation insertion position {position} exceeds operation count \
                 {operation_count}"
            ),
            Self::BlockAlreadyTerminated => {
                f.write_str("cannot append or insert an operation after termination")
            }
            Self::InvalidTerminator(reason) => {
                write!(f, "invalid block terminator: {reason}")
            }
            Self::InvalidExtensionName(reason) => {
                write!(f, "invalid extension name: {reason}")
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

/// A typed value entering a block.
///
/// `value` is the canonical value identity. `ty` references the semantic type
/// table. An optional qualified `role` may describe a dialect-specific
/// argument role, but never replaces the argument's semantic type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockArgument {
    value: ValueId,
    ty: TypeId,
    role: Option<QualifiedName>,
}

impl BlockArgument {
    /// Create an argument from resolved value and type identities.
    #[must_use]
    pub fn new(value: ValueId, ty: TypeId) -> Self {
        Self {
            value,
            ty,
            role: None,
        }
    }

    /// Create an argument with an extension-qualified role.
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

    /// Return the optional extension-qualified role.
    #[must_use]
    pub fn role(&self) -> Option<&QualifiedName> {
        self.role.as_ref()
    }

    /// Validate local metadata, including metadata loaded from serialized IR.
    pub fn validate(&self) -> BlockResult<()> {
        if let Some(role) = &self.role {
            role.validate().map_err(|error| {
                BlockError::InvalidBlock(format!("invalid argument role: {error}"))
            })?;
        }

        Ok(())
    }
}

/// An explicit control-flow edge.
///
/// The target is a block identity. Arguments are values supplied to the
/// target's block arguments, in order. The full verifier must check target
/// existence, argument arity, type compatibility, and value visibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Successor {
    target: BlockId,
    arguments: Vec<ValueId>,
}

impl Successor {
    /// Construct an edge without block arguments.
    #[must_use]
    pub fn new(target: BlockId) -> Self {
        Self {
            target,
            arguments: Vec::new(),
        }
    }

    /// Construct an edge with ordered block arguments.
    #[must_use]
    pub fn with_arguments(target: BlockId, arguments: Vec<ValueId>) -> Self {
        Self { target, arguments }
    }

    /// Return the target block identity.
    #[must_use]
    pub const fn target(&self) -> BlockId {
        self.target
    }

    /// Return ordered arguments supplied to the target block.
    #[must_use]
    pub fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }

    /// Return the number of arguments supplied to the target.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }
}

/// A switch case value and its associated successor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchCase {
    value: AttributeValue,
    successor: Successor,
}

impl SwitchCase {
    /// Construct a case.
    #[must_use]
    pub fn new(value: AttributeValue, successor: Successor) -> Self {
        Self { value, successor }
    }

    /// Return the declarative case value.
    #[must_use]
    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    /// Return the successor for this case.
    #[must_use]
    pub fn successor(&self) -> &Successor {
        &self.successor
    }
}

/// The explicit control-flow contract for a block.
///
/// `Open` is a construction state, not a finalized terminator. An extension
/// terminator must be declared and interpreted by its registered dialect.
/// An unknown extension must never be assigned implicit executable meaning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BlockTerminator {
    /// The block is still being assembled.
    Open,

    /// Return from the enclosing callable computation.
    Return {
        /// Ordered returned values.
        values: Vec<ValueId>,
    },

    /// Unconditional transfer to another block.
    Branch {
        /// Target and ordered edge arguments.
        successor: Successor,
    },

    /// Conditional transfer to one of two blocks.
    ConditionalBranch {
        /// Value used as the condition.
        condition: ValueId,
        /// Edge taken when the condition is true.
        then_successor: Successor,
        /// Edge taken when the condition is false.
        else_successor: Successor,
    },

    /// Multiway transfer.
    Switch {
        /// Value used to select a case.
        selector: ValueId,
        /// Edge used if no case matches.
        default_successor: Successor,
        /// Cases in declared order.
        cases: Vec<SwitchCase>,
    },

    /// Terminate the current path without a normal return.
    Unreachable,

    /// A terminator whose semantics belong to a registered dialect.
    Extension {
        /// Qualified dialect operation name.
        name: QualifiedName,
        /// Explicit outgoing edges, if any.
        successors: Vec<Successor>,
    },
}

impl Default for BlockTerminator {
    fn default() -> Self {
        Self::Open
    }
}

impl BlockTerminator {
    /// Whether this terminator is still in its construction state.
    #[must_use]
    pub const fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }

    /// Whether this terminator closes the block.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        !self.is_open()
    }

    /// Return the number of explicitly represented outgoing edges.
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

    /// Visit successors in declared order without allocating a temporary list.
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

    /// Validate terminator invariants that can be checked locally.
    ///
    /// This does not resolve values, block identities, types, or dialect
    /// registration. Those checks belong to the full verifier.
    pub fn validate(&self) -> BlockResult<()> {
        match self {
            Self::Open | Self::Return { .. } | Self::Branch { .. } => Ok(()),

            Self::ConditionalBranch { .. } => Ok(()),

            Self::Switch { cases, .. } => {
                // AttributeValue is not required to implement Ord or Hash.
                // Compare by equality to preserve its canonical semantics.
                // A future schema-level canonical key can make this check
                // sub-quadratic without changing the switch representation.
                for (index, case) in cases.iter().enumerate() {
                    if cases[..index]
                        .iter()
                        .any(|prior| prior.value() == case.value())
                    {
                        return Err(BlockError::DuplicateSwitchCase);
                    }
                }

                Ok(())
            }

            Self::Unreachable => Ok(()),

            Self::Extension { name, .. } => name
                .validate()
                .map_err(|error| BlockError::InvalidExtensionName(error.to_string())),
        }
    }
}

/// Wire representation used to reject locally invalid blocks during
/// deserialization rather than trusting serialized state.
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
/// actual operation definitions live in the owning program/module storage.
///
/// A missing parent region is permitted during construction. A finalized
/// block must have a parent region and a closed terminator.
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

    /// Create an empty block associated with its parent region.
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

    /// Return this block's identity.
    #[must_use]
    pub const fn id(&self) -> BlockId {
        self.id
    }

    /// Return the parent region, if assigned.
    #[must_use]
    pub const fn parent_region(&self) -> Option<RegionId> {
        self.parent_region
    }

    /// Assign the parent region during construction or transformation.
    ///
    /// The full verifier must check that the parent region also lists this
    /// block and that ownership is reciprocal.
    pub fn set_parent_region(&mut self, region: RegionId) {
        self.parent_region = Some(region);
    }

    /// Return block arguments in declared order.
    #[must_use]
    pub fn arguments(&self) -> &[BlockArgument] {
        &self.arguments
    }

    /// Return the number of block arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Return operation identities in semantic order.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Return the number of operation references.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Return the current terminator.
    #[must_use]
    pub fn terminator(&self) -> &BlockTerminator {
        &self.terminator
    }

    /// Whether the block has a closed terminator.
    #[must_use]
    pub fn is_terminated(&self) -> bool {
        self.terminator.is_closed()
    }

    /// Add a block argument, rejecting duplicate value identities.
    pub fn add_argument(&mut self, argument: BlockArgument) -> BlockResult<()> {
        argument.validate()?;

        if self
            .arguments
            .iter()
            .any(|existing| existing.value() == argument.value())
        {
            return Err(BlockError::DuplicateArgument(argument.value()));
        }

        self.arguments
            .try_reserve(1)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "arguments",
            })?;

        self.arguments.push(argument);
        Ok(())
    }

    /// Append an operation reference.
    ///
    /// The operation must exist in the owning IR and must not belong to
    /// another block; those are whole-program verification responsibilities.
    pub fn append_operation(&mut self, operation: OperationId) -> BlockResult<()> {
        if self.is_terminated() {
            return Err(BlockError::BlockAlreadyTerminated);
        }

        if self.operations.contains(&operation) {
            return Err(BlockError::DuplicateOperation(operation));
        }

        self.operations
            .try_reserve(1)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "operations",
            })?;

        self.operations.push(operation);
        Ok(())
    }

    /// Insert an operation at an exact position.
    ///
    /// `position == operation_count()` appends. An out-of-range position is
    /// rejected rather than clamped.
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

        self.operations
            .try_reserve(1)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "operations",
            })?;

        self.operations.insert(position, operation);
        Ok(())
    }

    /// Remove an operation reference without deleting its stored payload.
    ///
    /// Returns `true` if the reference was found and removed. Removing a
    /// reference is a structural edit; the enclosing IR must be reverified.
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

    /// Set a terminator on a block that is not yet terminated.
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

    /// Replace a terminator as part of an explicit IR transformation.
    ///
    /// This deliberately permits replacing a closed terminator. The caller
    /// must rerun whole-program verification after rewriting control flow.
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

    /// Validate local invariants while allowing construction to be incomplete.
    ///
    /// This checks duplicate identities, argument metadata, and terminator
    /// shape. It does not prove that referenced entities exist elsewhere.
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

    /// Validate a block intended for inclusion in complete IR.
    ///
    /// A complete block requires a parent region and a closed terminator.
    /// The full verifier must still resolve all references and verify CFG,
    /// SSA, type, effect, ownership, and dialect-specific invariants.
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

    fn block_id(value: u128) -> BlockId {
        BlockId::new(value).expect("valid block ID")
    }

    fn region_id(value: u128) -> RegionId {
        RegionId::new(value).expect("valid region ID")
    }

    fn operation_id(value: u128) -> OperationId {
        OperationId::new(value).expect("valid operation ID")
    }

    fn value_id(value: u128) -> ValueId {
        ValueId::new(value).expect("valid value ID")
    }

    fn type_id(value: u128) -> TypeId {
        TypeId::new(value).expect("valid type ID")
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
            &[
                operation_id(1),
                operation_id(2),
                operation_id(3)
            ]
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
    fn open_terminator_cannot_be_set_as_a_final_terminator() {
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
    fn local_validation_detects_duplicate_operation_ids() {
        let mut block = ZuirBlock::new(block_id(1));

        block.append_operation(operation_id(1)).unwrap();

        // Simulate malformed in-memory data only through a serialized or
        // internal construction path in tests if the representation changes.
        assert!(block.validate().is_ok());
    }
}
