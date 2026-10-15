
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Basic Block
//!
//! Canonical path: `src/zuir/ir/block.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: `unsafe` Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Represent a target-independent basic block using:
//! - a typed block identity;
//! - an optional parent-region reference during construction;
//! - typed block arguments;
//! - an ordered sequence of operation identities;
//! - an explicit terminator and successor edges.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/syntax-model.md`
//! - `grammar/specification/scalability-model.md`
//!
//! ## OWNS
//!
//! - Block-local identity and structure.
//! - Block arguments and their declared types.
//! - Ordered operation references.
//! - Local terminator shape and explicit successor arguments.
//! - Local validation and deterministic access.
//!
//! ## DOES_NOT_OWN
//!
//! - Operation definitions or operation registries.
//! - Region storage or global ID allocation.
//! - Parsing, AST construction, or source-name resolution.
//! - Type inference, SSA dominance, or def-use indexing.
//! - Global control-flow verification.
//! - Dialect registration or extension execution.
//! - Target-specific layout, scheduling, or runtime execution.
//!
//! ## INTEGRATION
//!
//! `ids.rs` owns `BlockId`, `OperationId`, `RegionId`, `ValueId`, and `TypeId`.
//! `schema/types.rs` owns `QualifiedName`.
//! `schema/attributes.rs` owns `AttributeValue`.
//! `ir/region.rs` stores `BlockId` references in region order.
//! `ir/function.rs` stores `RegionId` references in function bodies.
//! The operation container, when implemented, owns operation payloads.
//! The full verifier must resolve every referenced ID and check ownership,
//! types, dominance, successor argument arity, and dialect-specific rules.
//!
//! `grammar/Zamani.g4` and the grammar specification own source syntax and
//! source meaning. This file represents already-resolved IR structure; it
//! must not duplicate grammar productions or source-level name resolution.
//!
//! ## SCALABILITY
//!
//! Collections are dynamically sized. No maximum block size, operation count,
//! argument count, successor count, hardware count, qubit count, register
//! width, or tensor rank is imposed here. Fallible mutation methods use
//! `try_reserve` where practical. The implementation remains limited by
//! available memory and address space; it does not promise literal infinity.
//!
//! ## VALIDITY BOUNDARY
//!
//! `validate()` checks invariants local to this block. It does not prove that
//! referenced operations, values, types, or successor blocks exist. Call the
//! full-program verifier before optimization, serialization of trusted IR,
//! or lowering.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

use crate::zuir::ids::{BlockId, OperationId, RegionId, TypeId, ValueId};
use crate::zuir::schema::attributes::AttributeValue;
use crate::zuir::schema::types::QualifiedName;

/// Result type for block-local operations.
pub type BlockResult<T> = Result<T, BlockError>;

/// Errors produced by local block construction, mutation, and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockError {
    /// An operation identity is already present in the block.
    DuplicateOperation(OperationId),

    /// A block argument identity is already present in the block.
    DuplicateArgument(ValueId),

    /// An insertion position is outside the valid range.
    InvalidOperationPosition {
        position: usize,
        operation_count: usize,
    },

    /// The block is terminated and ordinary operations cannot be appended.
    BlockAlreadyTerminated,

    /// A terminator was malformed.
    InvalidTerminator(String),

    /// An extension terminator has an invalid qualified name.
    InvalidExtensionName(String),

    /// A complete block must belong to a region.
    MissingParentRegion,

    /// The same switch case value appears more than once.
    DuplicateSwitchCase,

    /// Memory could not be reserved for a collection mutation.
    AllocationFailure {
        collection: &'static str,
    },

    /// A locally invalid block was encountered.
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
                f.write_str("cannot append an operation after block termination")
            }
            Self::InvalidTerminator(reason) => {
                write!(f, "invalid block terminator: {reason}")
            }
            Self::InvalidExtensionName(reason) => {
                write!(f, "invalid extension terminator name: {reason}")
            }
            Self::MissingParentRegion => {
                f.write_str("a complete block must identify its parent region")
            }
            Self::DuplicateSwitchCase => {
                f.write_str("switch terminator contains a duplicate case value")
            }
            Self::AllocationFailure { collection } => {
                write!(f, "unable to reserve memory for block {collection}")
            }
            Self::InvalidBlock(reason) => {
                write!(f, "invalid block: {reason}")
            }
        }
    }
}

impl std::error::Error for BlockError {}

/// One typed value entering a block.
///
/// `value` is the canonical value identity. `ty` refers to the semantic type
/// table. `role` is optional, extension-qualified metadata for dialects that
/// need to describe a domain-specific argument role. It does not override
/// the value's semantic type.
///
/// Logical versus physical quantum resource identities remain owned by the
/// quantum domain. A generic ZUIR block argument must not invent a second
/// quantum identity system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockArgument {
    value: ValueId,
    ty: TypeId,
    role: Option<QualifiedName>,
}

impl BlockArgument {
    /// Construct a block argument from resolved identities.
    pub fn new(
        value: ValueId,
        ty: TypeId,
    ) -> Self {
        Self {
            value,
            ty,
            role: None,
        }
    }

    /// Construct an argument with an extension-qualified role.
    pub fn with_role(
        value: ValueId,
        ty: TypeId,
        role: QualifiedName,
    ) -> BlockResult<Self> {
        role.validate().map_err(|error| {
            BlockError::InvalidTerminator(format!(
                "invalid block argument role: {error}"
            ))
        })?;

        Ok(Self {
            value,
            ty,
            role: Some(role),
        })
    }

    /// Return the argument value identity.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Return the argument's semantic type identity.
    #[must_use]
    pub const fn ty(&self) -> TypeId {
        self.ty
    }

    /// Return the optional extension-qualified role.
    #[must_use]
    pub fn role(&self) -> Option<&QualifiedName> {
        self.role.as_ref()
    }

    /// Revalidate local metadata, including after deserialization.
    pub fn validate(&self) -> BlockResult<()> {
        if let Some(role) = &self.role {
            role.validate().map_err(|error| {
                BlockError::InvalidTerminator(format!(
                    "invalid block argument role: {error}"
                ))
            })?;
        }
        Ok(())
    }
}

/// An explicit control-flow edge.
///
/// The target is a block identity. `arguments` are the values passed to that
/// target's block arguments, in order. Target existence, argument count, and
/// type compatibility are whole-program verifier responsibilities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Successor {
    target: BlockId,
    arguments: Vec<ValueId>,
}

impl Successor {
    /// Construct an edge with no block arguments.
    #[must_use]
    pub fn new(target: BlockId) -> Self {
        Self {
            target,
            arguments: Vec::new(),
        }
    }

    /// Construct an edge with explicitly ordered arguments.
    #[must_use]
    pub fn with_arguments(
        target: BlockId,
        arguments: Vec<ValueId>,
    ) -> Self {
        Self { target, arguments }
    }

    /// Return the target block.
    #[must_use]
    pub const fn target(&self) -> BlockId {
        self.target
    }

    /// Return arguments in their declared order.
    #[must_use]
    pub fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }

    /// Return the number of successor arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }
}

/// A switch case associates a portable, declarative value with a successor.
///
/// The verifier must check that case values are appropriate for the selector
/// type and that the owning dialect permits this switch form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchCase {
    value: AttributeValue,
    successor: Successor,
}

impl SwitchCase {
    /// Construct a case from a declarative value and successor edge.
    #[must_use]
    pub fn new(value: AttributeValue, successor: Successor) -> Self {
        Self { value, successor }
    }

    /// Return the case value.
    #[must_use]
    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    /// Return the successor edge.
    #[must_use]
    pub fn successor(&self) -> &Successor {
        &self.successor
    }
}

/// The local termination/control-flow contract for a block.
///
/// `Open` is a construction state, not a valid terminator for a finalized
/// control-flow block. Extension-defined terminators must be registered and
/// interpreted by the owning dialect; an unknown extension is never given
/// executable semantics by this enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BlockTerminator {
    /// The block is still being assembled.
    Open,

    /// Return from the enclosing callable computation.
    Return {
        /// Ordered returned values. Repeated values are permitted.
        values: Vec<ValueId>,
    },

    /// Unconditional transfer to another block.
    Branch {
        /// Target and ordered edge arguments.
        successor: Successor,
    },

    /// Conditional transfer to either of two blocks.
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
        /// Edge used when no case matches.
        default_successor: Successor,
        /// Ordered cases. Duplicate case values are rejected.
        cases: Vec<SwitchCase>,
    },

    /// End the current path without a normal return.
    Unreachable,

    /// Extension-defined terminator.
    Extension {
        /// Registered dialect operation or terminator name.
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

    /// Number of outgoing edges explicitly represented by the terminator.
    #[must_use]
    pub fn successor_count(&self) -> usize {
        match self {
            Self::Open | Self::Return { .. } | Self::Unreachable => 0,
            Self::Branch { .. } => 1,
            Self::ConditionalBranch { .. } => 2,
            Self::Switch {
                default_successor,
                cases,
                ..
            } => 1 + cases.len(),
            Self::Extension { successors, .. } => successors.len(),
        }
    }

    /// Visit outgoing edges in their declared order without allocating.
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

    /// Validate local terminator shape.
    ///
    /// This does not resolve values, blocks, types, or dialect registrations.
    pub fn validate(&self) -> BlockResult<()> {
        match self {
            Self::Open | Self::Return { .. } | Self::Unreachable => Ok(()),

            Self::Branch { .. } => Ok(()),

            Self::ConditionalBranch { .. } => Ok(()),

            Self::Switch { cases, .. } => {
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

            Self::Extension { name, .. } => {
                name.validate().map_err(|error| {
                    BlockError::InvalidExtensionName(error.to_string())
                })
            }
        }
    }
}

/// Canonical target-independent basic block.
///
/// Operation IDs are stored in semantic order. Operation payloads remain in
/// the operation/program storage layer to avoid recursive ownership and to
/// permit independent operation registries.
///
/// The parent region is optional only during assembly. `validate_complete()`
/// requires it to be set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZuirBlock {
    id: BlockId,
    parent_region: Option<RegionId>,
    arguments: Vec<BlockArgument>,
    operations: Vec<OperationId>,
    terminator: BlockTerminator,
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

    /// Create a block associated with a parent region.
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

    /// Return the optional parent region.
    #[must_use]
    pub const fn parent_region(&self) -> Option<RegionId> {
        self.parent_region
    }

    /// Set the parent region during construction.
    ///
    /// This method records the relationship. The full verifier must confirm
    /// that the parent region also lists this block.
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

    /// Return the number of operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Return the current terminator.
    #[must_use]
    pub fn terminator(&self) -> &BlockTerminator {
        &self.terminator
    }

    /// Whether the block has a non-open terminator.
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

    /// Append an operation reference to the block.
    ///
    /// Operations cannot be appended after a terminator. The operation must
    /// exist in the owning program and must not be owned by another block;
    /// those checks belong to the whole-program verifier.
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

    /// Insert an operation reference at a precise position.
    ///
    /// `position == operation_count()` appends. Larger positions fail rather
    /// than being silently clamped.
    pub fn insert_operation(
        &mut self,
        position: usize,
        operation: OperationId,
    ) -> BlockResult<()> {
        if self.is_terminated() {
            return Err(BlockError::BlockAlreadyTerminated);
        }

        if self.operations.contains(&operation) {
            return Err(BlockError::DuplicateOperation(operation));
        }

        if position > self.operations.len() {
            return Err(BlockError::InvalidOperationPosition {
                position,
                operation_count: self.operations.len(),
            });
        }

        self.operations
            .try_reserve(1)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "operations",
            })?;

        self.operations.insert(position, operation);
        Ok(())
    }

    /// Remove an operation reference by identity.
    ///
    /// Returns `true` when an entry was removed. Removing an operation from
    /// this list does not delete the operation from program storage.
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

    /// Replace the terminator explicitly.
    ///
    /// This supports transformations that rewrite control flow. The caller
    /// remains responsible for re-verifying the enclosing control-flow graph.
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

    /// Replace an existing terminator during an explicit IR transformation.
    ///
    /// Unlike `set_terminator`, this method deliberately permits replacing a
    /// closed terminator. The transformation must run full verification after
    /// the rewrite.
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

    /// Validate local invariants, allowing a block still under construction.
    ///
    /// This method does not require a parent region or a closed terminator.
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

    /// Validate a block intended for a complete IR.
    ///
    /// Complete validation requires a parent region and a closed terminator.
    /// The full verifier must still resolve cross-entity references and check
    /// CFG, type, dominance, effect, and dialect-specific invariants.
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
        BlockId::new(value).expect("nonzero block ID")
    }

    fn region_id(value: u128) -> RegionId {
        RegionId::new(value).expect("nonzero region ID")
    }

    fn operation_id(value: u128) -> OperationId {
        OperationId::new(value).expect("nonzero operation ID")
    }

    fn value_id(value: u128) -> ValueId {
        ValueId::new(value).expect("nonzero value ID")
    }

    fn type_id(value: u128) -> TypeId {
        TypeId::new(value).expect("nonzero type ID")
    }

    #[test]
    fn creates_open_block() {
        let block = ZuirBlock::new(block_id(1));
        assert_eq!(block.id(), block_id(1));
        assert!(block.parent_region().is_none());
        assert!(block.terminator().is_open());
        assert!(block.validate().is_ok());
        assert!(block.validate_complete().is_err());
    }

    #[test]
    fn parent_region_can_be_set() {
        let mut block = ZuirBlock::new(block_id(1));
        block.set_parent_region(region_id(9));
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
    fn duplicate_argument_is_rejected() {
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
    fn duplicate_operation_is_rejected() {
        let mut block = ZuirBlock::new(block_id(1));
        block.append_operation(operation_id(3)).unwrap();

        assert_eq!(
            block.append_operation(operation_id(3)),
            Err(BlockError::DuplicateOperation(operation_id(3)))
        );
    }

    #[test]
    fn operation_can_be_inserted_at_position() {
        let mut block = ZuirBlock::new(block_id(1));
        block.append_operation(operation_id(1)).unwrap();
        block.append_operation(operation_id(3)).unwrap();
        block
            .insert_operation(1, operation_id(2))
            .unwrap();

        assert_eq!(
            block.operations(),
            &[operation_id(1), operation_id(2), operation_id(3)]
        );
    }

    #[test]
    fn invalid_operation_position_is_rejected() {
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
    fn operation_can_be_removed_without_deleting_storage() {
        let mut block = ZuirBlock::new(block_id(1));
        block.append_operation(operation_id(1)).unwrap();

        assert!(block.remove_operation(operation_id(1)));
        assert!(!block.remove_operation(operation_id(1)));
        assert!(block.operations().is_empty());
    }

    #[test]
    fn closed_block_rejects_appending_operations() {
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
    fn branch_records_target_and_arguments() {
        let mut block = ZuirBlock::in_region(block_id(1), region_id(7));
        let successor =
            Successor::with_arguments(block_id(2), vec![value_id(8), value_id(9)]);

        block
            .set_terminator(BlockTerminator::Branch {
                successor: successor.clone(),
            })
            .unwrap();

        assert_eq!(block.terminator().successor_count(), 1);
        assert_eq!(successor.target(), block_id(2));
        assert_eq!(
            successor.arguments(),
            &[value_id(8), value_id(9)]
        );
    }

    #[test]
    fn conditional_branch_has_two_explicit_edges() {
        let mut block = ZuirBlock::new(block_id(1));
        block
            .set_terminator(BlockTerminator::ConditionalBranch {
                condition: value_id(1),
                then_successor: Successor::new(block_id(2)),
                else_successor: Successor::new(block_id(3)),
            })
            .unwrap();

        assert_eq!(block.terminator().successor_count(), 2);
    }

    #[test]
    fn same_target_is_allowed_for_both_conditional_edges() {
        let mut block = ZuirBlock::new(block_id(1));
        let target = block_id(2);

        assert!(block
            .set_terminator(BlockTerminator::ConditionalBranch {
                condition: value_id(1),
                then_successor: Successor::new(target),
                else_successor: Successor::new(target),
            })
            .is_ok());
    }

    #[test]
    fn switch_rejects_duplicate_case_values() {
        let mut block = ZuirBlock::new(block_id(1));
        let value = AttributeValue::Integer(4);

        let terminator = BlockTerminator::Switch {
            selector: value_id(1),
            default_successor: Successor::new(block_id(2)),
            cases: vec![
                SwitchCase::new(
                    value.clone(),
                    Successor::new(block_id(3)),
                ),
                SwitchCase::new(
                    value,
                    Successor::new(block_id(4)),
                ),
            ],
        };

        assert_eq!(
            block.set_terminator(terminator),
            Err(BlockError::DuplicateSwitchCase)
        );
    }

    #[test]
    fn complete_validation_requires_closed_terminator_and_parent() {
        let mut block = ZuirBlock::new(block_id(1));
        assert_eq!(
            block.validate_complete(),
            Err(BlockError::MissingParentRegion)
        );

        block.set_parent_region(region_id(2));
        assert!(block.validate_complete().is_err());

        block
            .set_terminator(BlockTerminator::Return { values: Vec::new() })
            .unwrap();

        assert!(block.validate_complete().is_ok());
    }

    #[test]
    fn replacing_terminator_is_explicit() {
        let mut block = ZuirBlock::new(block_id(1));
        block
            .set_terminator(BlockTerminator::Return { values: Vec::new() })
            .unwrap();

        assert!(block
            .set_terminator(BlockTerminator::Unreachable)
            .is_err());

        assert!(block
            .replace_terminator(BlockTerminator::Unreachable)
            .is_ok());
    }

    #[test]
    fn successor_iteration_does_not_change_order() {
        let terminator = BlockTerminator::ConditionalBranch {
            condition: value_id(1),
            then_successor: Successor::new(block_id(2)),
            else_successor: Successor::new(block_id(3)),
        };

        let mut targets = Vec::new();
        terminator.for_each_successor(|edge| targets.push(edge.target()));

        assert_eq!(targets, vec![block_id(2), block_id(3)]);
    }

    #[test]
    fn serde_round_trip_preserves_structure() {
        let mut block = ZuirBlock::in_region(block_id(1), region_id(2));
        block
            .add_argument(BlockArgument::new(value_id(3), type_id(4)))
            .unwrap();
        block.append_operation(operation_id(5)).unwrap();
        block
            .set_terminator(BlockTerminator::Return {
                values: vec![value_id(3)],
            })
            .unwrap();

        let encoded = serde_json::to_string(&block).unwrap();
        let decoded: ZuirBlock = serde_json::from_str(&encoded).unwrap();

        assert_eq!(block, decoded);
        decoded.validate_complete().unwrap();
    }
}
