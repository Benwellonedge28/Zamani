#![forbid(unsafe_code)]

//! # ZUIR definition-use index
//!
//! Derived index of value definitions and use sites for ZUIR.
//!
//! ## Ownership
//!
//! - `operation.rs` owns operation operands and results.
//! - `block.rs` owns block arguments, operation order, and terminators.
//! - `successor.rs` owns control-flow edge targets and edge arguments.
//! - `values/results.rs` remains the owner of operation-local result
//!   collections.
//! - This module owns only the derived definition-use index.
//!
//! ## Scope
//!
//! The index detects duplicate definitions, duplicate operation and block
//! identities, invalid block-to-operation references, and unresolved uses.
//! It does not prove dominance, type compatibility, region visibility,
//! ownership, effect ordering, or dialect legality. Those are responsibilities
//! of the whole-IR verifier.
//!
//! ## Lifecycle
//!
//! The index is a snapshot. Rebuild or invalidate it after any relevant IR
//! mutation. Register definitions not included in `build()`—such as function
//! arguments—before calling `verify_resolved()`.
//!
//! ## Scalability
//!
//! No arbitrary machine-size or IR-size limit is imposed. Collections grow
//! dynamically, use counts are checked, and use-list growth uses fallible
//! reservation. Standard ordered-map/set allocation does not expose a stable
//! fallible reservation API, so process-level allocation failure cannot be
//! universally recovered from.
//!
//! ## Determinism
//!
//! Definitions are iterated in `ValueId` order. Use lists are maintained in
//! `UseSite` order as uses are inserted.

use serde::{Deserialize, Serialize};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{BlockId, FunctionId, OperationId, TypeId, ValueId};
use crate::zuir::ir::block::{BlockTerminator, ZuirBlock};
use crate::zuir::ir::operation::ZuirOperation;

/// Result type used by this module.
pub type UseDefResult<T> = Result<T, UseDefError>;

/// The owner and position of a value definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DefinitionKind {
    /// An operation result.
    OperationResult {
        operation: OperationId,
        result_index: usize,
    },

    /// A block argument.
    BlockArgument {
        block: BlockId,
        argument_index: usize,
    },

    /// A function parameter.
    ///
    /// `build()` does not infer these because it receives blocks and
    /// operations rather than function objects. Register them through
    /// `define()` before resolution verification.
    FunctionArgument {
        function: FunctionId,
        argument_index: usize,
    },
}

/// One value's definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueDefinition {
    value: ValueId,
    ty: TypeId,
    kind: DefinitionKind,
}

impl ValueDefinition {
    /// Return the defined value ID.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Return the definition's type ID.
    #[must_use]
    pub const fn ty(&self) -> TypeId {
        self.ty
    }

    /// Return the definition owner and position.
    #[must_use]
    pub const fn kind(&self) -> DefinitionKind {
        self.kind
    }
}

/// A location where a value is consumed or referenced.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum UseSite {
    /// An operation operand.
    OperationOperand {
        operation: OperationId,
        operand_index: usize,
    },

    /// A returned value.
    ReturnValue {
        block: BlockId,
        value_index: usize,
    },

    /// A conditional branch condition.
    BranchCondition {
        block: BlockId,
    },

    /// A switch selector.
    SwitchSelector {
        block: BlockId,
    },

    /// An argument passed over a control-flow edge.
    ///
    /// `successor_index` is the successor's position in the terminator's
    /// defined ordering. `argument_index` is the argument's position on that
    /// edge.
    SuccessorArgument {
        block: BlockId,
        successor_index: usize,
        argument_index: usize,
    },
}

/// One use of a value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueUse {
    value: ValueId,
    site: UseSite,
}

impl ValueUse {
    /// Create a use record.
    ///
    /// This records a use site; it does not validate that the definition
    /// exists or is visible at that site.
    #[must_use]
    pub const fn new(value: ValueId, site: UseSite) -> Self {
        Self { value, site }
    }

    /// Return the referenced value.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Return the use site.
    #[must_use]
    pub const fn site(&self) -> &UseSite {
        &self.site
    }
}

/// Definition-use index errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseDefError {
    /// A value has more than one definition.
    DuplicateDefinition {
        value: ValueId,
        first: DefinitionKind,
        second: DefinitionKind,
    },

    /// An operation identity occurs more than once.
    DuplicateOperation(OperationId),

    /// A block identity occurs more than once.
    DuplicateBlock(BlockId),

    /// A block references an operation absent from the supplied collection.
    MissingOperationReference {
        block: BlockId,
        operation: OperationId,
    },

    /// One operation is assigned to multiple blocks.
    OperationInMultipleBlocks {
        operation: OperationId,
        first: BlockId,
        second: BlockId,
    },

    /// A use refers to a value without an indexed definition.
    UndefinedValue(ValueId),

    /// A definition map key disagrees with the stored definition.
    InconsistentDefinitionKey(ValueId),

    /// A use map key disagrees with the recorded use value.
    InconsistentUseKey(ValueId),

    /// Memory reservation for a use list failed.
    AllocationFailure {
        collection: &'static str,
    },

    /// The total use count cannot fit in `usize`.
    CountOverflow,
}

impl fmt::Display for UseDefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateDefinition {
                value,
                first,
                second,
            } => write!(
                f,
                "value {value} has multiple definitions: {first:?} and {second:?}"
            ),
            Self::DuplicateOperation(id) => {
                write!(f, "duplicate operation identity {id}")
            }
            Self::DuplicateBlock(id) => {
                write!(f, "duplicate block identity {id}")
            }
            Self::MissingOperationReference { block, operation } => write!(
                f,
                "block {block} references missing operation {operation}"
            ),
            Self::OperationInMultipleBlocks {
                operation,
                first,
                second,
            } => write!(
                f,
                "operation {operation} is listed in both block {first} and block {second}"
            ),
            Self::UndefinedValue(value) => {
                write!(f, "use of undefined value {value}")
            }
            Self::InconsistentDefinitionKey(value) => {
                write!(f, "inconsistent definition index key for value {value}")
            }
            Self::InconsistentUseKey(value) => {
                write!(f, "inconsistent use index key for value {value}")
            }
            Self::AllocationFailure { collection } => {
                write!(f, "unable to reserve memory for {collection}")
            }
            Self::CountOverflow => {
                f.write_str("total use count exceeds the usize range")
            }
        }
    }
}

impl std::error::Error for UseDefError {}

/// Deterministic, derived definition-use index.
///
/// This structure is not canonical serialized IR state. Rebuild it from the
/// authoritative IR after decoding or mutation.
#[derive(Debug, Clone, Default)]
pub struct UseDefIndex {
    definitions: BTreeMap<ValueId, ValueDefinition>,
    uses: BTreeMap<ValueId, Vec<ValueUse>>,
    operation_ids: BTreeSet<OperationId>,
    block_ids: BTreeSet<BlockId>,
}

impl UseDefIndex {
    /// Create an empty index.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Build an index from the supplied blocks and operations.
    ///
    /// The method indexes block arguments, operation results, operation
    /// operands, terminator values, and successor-edge arguments. It checks
    /// structural identity/reference relationships available through these
    /// inputs.
    ///
    /// It intentionally does not call `verify_resolved()`: the caller may
    /// still need to register function arguments or other definitions owned
    /// by an enclosing scope.
    ///
    /// The whole-IR verifier must separately check dominance, types, scope,
    /// ownership, effects, and dialect-specific invariants.
    pub fn build(
        blocks: &[ZuirBlock],
        operations: &[ZuirOperation],
    ) -> UseDefResult<Self> {
        let mut index = Self::new();
        let mut operations_by_id: BTreeMap<OperationId, &ZuirOperation> =
            BTreeMap::new();

        // Register blocks and their arguments.
        for block in blocks {
            if !index.block_ids.insert(block.id()) {
                return Err(UseDefError::DuplicateBlock(block.id()));
            }

            for (argument_index, argument) in block.arguments().iter().enumerate() {
                index.define(
                    argument.value(),
                    argument.ty(),
                    DefinitionKind::BlockArgument {
                        block: block.id(),
                        argument_index,
                    },
                )?;
            }
        }

        // Register operations and their result definitions.
        for operation in operations {
            let operation_id = operation.id();

            if !index.operation_ids.insert(operation_id) {
                return Err(UseDefError::DuplicateOperation(operation_id));
            }

            operations_by_id.insert(operation_id, operation);

            for (result_index, result) in operation.results().iter().enumerate() {
                index.define(
                    result.value(),
                    result.ty(),
                    DefinitionKind::OperationResult {
                        operation: operation_id,
                        result_index,
                    },
                )?;
            }
        }

        // Check operation ownership and index terminator uses.
        let mut operation_owners: BTreeMap<OperationId, BlockId> = BTreeMap::new();

        for block in blocks {
            for operation_id in block.operations() {
                if !operations_by_id.contains_key(operation_id) {
                    return Err(UseDefError::MissingOperationReference {
                        block: block.id(),
                        operation: *operation_id,
                    });
                }

                if let Some(first_block) = operation_owners.get(operation_id) {
                    return Err(UseDefError::OperationInMultipleBlocks {
                        operation: *operation_id,
                        first: *first_block,
                        second: block.id(),
                    });
                }

                operation_owners.insert(*operation_id, block.id());
            }

            Self::index_terminator_uses(
                &mut index,
                block.id(),
                block.terminator(),
            )?;
        }

        // Index operands after all operation and block definitions have been
        // registered. Repeated operands intentionally produce separate uses.
        for operation in operations {
            for (operand_index, operand) in operation.operands().iter().enumerate() {
                index.record_use(ValueUse::new(
                    operand.value(),
                    UseSite::OperationOperand {
                        operation: operation.id(),
                        operand_index,
                    },
                ))?;
            }
        }

        index.verify_internal_consistency()?;
        Ok(index)
    }

    /// Register a definition.
    ///
    /// Use this for function parameters or other definitions outside the
    /// block/operation collections passed to `build()`.
    pub fn define(
        &mut self,
        value: ValueId,
        ty: TypeId,
        kind: DefinitionKind,
    ) -> UseDefResult<()> {
        if let Some(existing) = self.definitions.get(&value) {
            return Err(UseDefError::DuplicateDefinition {
                value,
                first: existing.kind,
                second: kind,
            });
        }

        self.definitions.insert(
            value,
            ValueDefinition { value, ty, kind },
        );

        Ok(())
    }

    /// Record a use while maintaining deterministic use-site order.
    ///
    /// A use can be recorded before its definition; call `verify_resolved()`
    /// after all definitions in the scope have been registered.
    pub fn record_use(&mut self, value_use: ValueUse) -> UseDefResult<()> {
        let value = value_use.value();

        match self.uses.entry(value) {
            Entry::Occupied(mut entry) => {
                let uses = entry.get_mut();

                uses.try_reserve(1).map_err(|_| {
                    UseDefError::AllocationFailure {
                        collection: "value use list",
                    }
                })?;

                let position = uses
                    .binary_search_by(|existing| existing.site.cmp(value_use.site()))
                    .unwrap_or_else(|position| position);

                uses.insert(position, value_use);
            }
            Entry::Vacant(entry) => {
                let mut uses = Vec::new();

                uses.try_reserve(1).map_err(|_| {
                    UseDefError::AllocationFailure {
                        collection: "value use list",
                    }
                })?;

                uses.push(value_use);
                entry.insert(uses);
            }
        }

        Ok(())
    }

    /// Look up a value's definition.
    #[must_use]
    pub fn definition(&self, value: ValueId) -> Option<&ValueDefinition> {
        self.definitions.get(&value)
    }

    /// Return a value's uses in deterministic order, or an empty slice.
    #[must_use]
    pub fn uses_of(&self, value: ValueId) -> &[ValueUse] {
        self.uses.get(&value).map_or(&[], Vec::as_slice)
    }

    /// Iterate through definitions in `ValueId` order.
    pub fn definitions(
        &self,
    ) -> impl Iterator<Item = (&ValueId, &ValueDefinition)> {
        self.definitions.iter()
    }

    /// Iterate through all uses, grouped by `ValueId` and ordered by use site.
    pub fn all_uses(&self) -> impl Iterator<Item = &ValueUse> {
        self.uses.values().flat_map(|uses| uses.iter())
    }

    /// Number of indexed definitions.
    #[must_use]
    pub fn definition_count(&self) -> usize {
        self.definitions.len()
    }

    /// Number of values that have one or more indexed uses.
    #[must_use]
    pub fn used_value_count(&self) -> usize {
        self.uses.len()
    }

    /// Total number of indexed uses, or `None` on integer overflow.
    #[must_use]
    pub fn use_count(&self) -> Option<usize> {
        self.uses
            .values()
            .try_fold(0usize, |total, uses| total.checked_add(uses.len()))
    }

    /// Number of distinct indexed operation identities.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operation_ids.len()
    }

    /// Number of distinct indexed block identities.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.block_ids.len()
    }

    /// Ensure that every recorded use refers to a registered definition.
    ///
    /// Call only after the enclosing scope has registered all externally
    /// owned definitions.
    pub fn verify_resolved(&self) -> UseDefResult<()> {
        self.verify_internal_consistency()?;

        for value_use in self.all_uses() {
            if !self.definitions.contains_key(&value_use.value()) {
                return Err(UseDefError::UndefinedValue(value_use.value()));
            }
        }

        Ok(())
    }

    /// Verify the index's internal key/value invariants.
    ///
    /// This does not verify the underlying IR's dominance, types, or control
    /// flow. It only verifies the index representation itself.
    pub fn verify_internal_consistency(&self) -> UseDefResult<()> {
        for (key, definition) in &self.definitions {
            if *key != definition.value {
                return Err(UseDefError::InconsistentDefinitionKey(*key));
            }
        }

        for (key, uses) in &self.uses {
            let mut previous_site: Option<&UseSite> = None;

            for value_use in uses {
                if *key != value_use.value() {
                    return Err(UseDefError::InconsistentUseKey(*key));
                }

                if let Some(previous) = previous_site {
                    if previous > value_use.site() {
                        return Err(UseDefError::InconsistentUseKey(*key));
                    }
                }

                previous_site = Some(value_use.site());
            }
        }

        Ok(())
    }

    /// Remove all definitions, uses, and tracked identities.
    pub fn clear(&mut self) {
        self.definitions.clear();
        self.uses.clear();
        self.operation_ids.clear();
        self.block_ids.clear();
    }

    fn index_terminator_uses(
        index: &mut Self,
        block: BlockId,
        terminator: &BlockTerminator,
    ) -> UseDefResult<()> {
        match terminator {
            BlockTerminator::Open | BlockTerminator::Unreachable => {}

            BlockTerminator::Return { values } => {
                for (value_index, value) in values.iter().copied().enumerate() {
                    index.record_use(ValueUse::new(
                        value,
                        UseSite::ReturnValue { block, value_index },
                    ))?;
                }
            }

            BlockTerminator::Branch { successor } => {
                Self::index_successor_uses(
                    index,
                    block,
                    0,
                    successor.arguments(),
                )?;
            }

            BlockTerminator::ConditionalBranch {
                condition,
                then_successor,
                else_successor,
            } => {
                index.record_use(ValueUse::new(
                    *condition,
                    UseSite::BranchCondition { block },
                ))?;

                Self::index_successor_uses(
                    index,
                    block,
                    0,
                    then_successor.arguments(),
                )?;

                Self::index_successor_uses(
                    index,
                    block,
                    1,
                    else_successor.arguments(),
                )?;
            }

            BlockTerminator::Switch {
                selector,
                default_successor,
                cases,
            } => {
                index.record_use(ValueUse::new(
                    *selector,
                    UseSite::SwitchSelector { block },
                ))?;

                Self::index_successor_uses(
                    index,
                    block,
                    0,
                    default_successor.arguments(),
                )?;

                for (case_index, case) in cases.iter().enumerate() {
                    let successor_index = case_index
                        .checked_add(1)
                        .ok_or(UseDefError::CountOverflow)?;

                    Self::index_successor_uses(
                        index,
                        block,
                        successor_index,
                        case.successor().arguments(),
                    )?;
                }
            }

            BlockTerminator::Extension { successors, .. } => {
                for (successor_index, successor) in successors.iter().enumerate() {
                    Self::index_successor_uses(
                        index,
                        block,
                        successor_index,
                        successor.arguments(),
                    )?;
                }
            }
        }

        Ok(())
    }

    fn index_successor_uses(
        index: &mut Self,
        block: BlockId,
        successor_index: usize,
        arguments: &[ValueId],
    ) -> UseDefResult<()> {
        for (argument_index, value) in arguments.iter().copied().enumerate() {
            index.record_use(ValueUse::new(
                value,
                UseSite::SuccessorArgument {
                    block,
                    successor_index,
                    argument_index,
                },
            ))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn ty(raw: u128) -> TypeId {
        TypeId::new(raw).expect("test type ID must be nonzero")
    }

    fn block(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test block ID must be nonzero")
    }

    fn function(raw: u128) -> FunctionId {
        FunctionId::new(raw).expect("test function ID must be nonzero")
    }

    fn operation(raw: u128) -> OperationId {
        OperationId::new(raw).expect("test operation ID must be nonzero")
    }

    #[test]
    fn registers_and_retrieves_a_definition() {
        let mut index = UseDefIndex::new();

        index
            .define(
                value(1),
                ty(1),
                DefinitionKind::FunctionArgument {
                    function: function(1),
                    argument_index: 0,
                },
            )
            .unwrap();

        let definition = index.definition(value(1)).unwrap();

        assert_eq!(definition.value(), value(1));
        assert_eq!(definition.ty(), ty(1));
        assert_eq!(
            definition.kind(),
            DefinitionKind::FunctionArgument {
                function: function(1),
                argument_index: 0,
            }
        );
    }

    #[test]
    fn rejects_duplicate_value_definitions() {
        let mut index = UseDefIndex::new();

        index
            .define(
                value(1),
                ty(1),
                DefinitionKind::FunctionArgument {
                    function: function(1),
                    argument_index: 0,
                },
            )
            .unwrap();

        let error = index
            .define(
                value(1),
                ty(2),
                DefinitionKind::BlockArgument {
                    block: block(1),
                    argument_index: 0,
                },
            )
            .unwrap_err();

        assert!(matches!(
            error,
            UseDefError::DuplicateDefinition { value: found, .. }
                if found == value(1)
        ));
    }

    #[test]
    fn preserves_repeated_uses_and_counts_them() {
        let mut index = UseDefIndex::new();

        index
            .define(
                value(1),
                ty(1),
                DefinitionKind::FunctionArgument {
                    function: function(1),
                    argument_index: 0,
                },
            )
            .unwrap();

        index
            .record_use(ValueUse::new(
                value(1),
                UseSite::OperationOperand {
                    operation: operation(10),
                    operand_index: 1,
                },
            ))
            .unwrap();

        index
            .record_use(ValueUse::new(
                value(1),
                UseSite::OperationOperand {
                    operation: operation(10),
                    operand_index: 0,
                },
            ))
            .unwrap();

        assert_eq!(index.uses_of(value(1)).len(), 2);
        assert_eq!(index.use_count(), Some(2));
        assert_eq!(index.used_value_count(), 1);
        assert!(index.verify_resolved().is_ok());

        assert_eq!(
            index.uses_of(value(1))[0].site(),
            &UseSite::OperationOperand {
                operation: operation(10),
                operand_index: 0,
            }
        );
    }

    #[test]
    fn unresolved_use_is_reported_after_construction() {
        let mut index = UseDefIndex::new();

        index
            .record_use(ValueUse::new(
                value(99),
                UseSite::ReturnValue {
                    block: block(1),
                    value_index: 0,
                },
            ))
            .unwrap();

        assert_eq!(
            index.verify_resolved(),
            Err(UseDefError::UndefinedValue(value(99)))
        );
    }

    #[test]
    fn clear_removes_all_indexed_state() {
        let mut index = UseDefIndex::new();

        index
            .define(
                value(1),
                ty(1),
                DefinitionKind::FunctionArgument {
                    function: function(1),
                    argument_index: 0,
                },
            )
            .unwrap();

        index
            .record_use(ValueUse::new(
                value(1),
                UseSite::ReturnValue {
                    block: block(1),
                    value_index: 0,
                },
            ))
            .unwrap();

        index.clear();

        assert_eq!(index.definition_count(), 0);
        assert_eq!(index.used_value_count(), 0);
        assert_eq!(index.use_count(), Some(0));
        assert_eq!(index.operation_count(), 0);
        assert_eq!(index.block_count(), 0);
        assert!(index.verify_resolved().is_ok());
    }

    #[test]
    fn empty_index_is_consistent() {
        let index = UseDefIndex::new();

        assert_eq!(index.definition_count(), 0);
        assert_eq!(index.use_count(), Some(0));
        assert!(index.verify_internal_consistency().is_ok());
        assert!(index.verify_resolved().is_ok());
    }
}