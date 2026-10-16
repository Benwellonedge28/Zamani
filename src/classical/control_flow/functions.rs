
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Function Control-Flow Planning
//!
//! Path: `src/classical/control_flow/functions.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Describes the function-level control-flow references needed by the
//! classical compiler integration without defining a competing function
//! declaration, function signature, AST, CFG, or intermediate representation.
//!
//! A plan associates a canonical ZUIR function with its entry block and the
//! blocks that terminate by returning from that function.
//!
//! # Authority and ownership
//!
//! - `grammar/functions/functions.g4` owns function declaration syntax.
//! - The frontend AST owns source-level function declarations and bodies.
//! - Semantic analysis owns name resolution, signature checking, type
//!   checking, effects, contracts, generic resolution, and return validation.
//! - `crate::zuir::ids` owns canonical function and block identities.
//! - `crate::zuir::ir::function` owns canonical ZUIR function declarations,
//!   signatures, and body-region references.
//! - The canonical ZUIR CFG and verifier own blocks, edges, terminators,
//!   reachability, dominance, and full control-flow validity.
//! - `crate::classical::error` owns the classical error vocabulary.
//!
//! This module owns only its target-independent function control-flow plan.
//!
//! # Scope
//!
//! This module supports:
//!
//! - identifying the canonical function associated with a plan;
//! - identifying its entry block;
//! - recording its return-terminating blocks;
//! - deterministic inspection and decomposition;
//! - adding and removing return-block references;
//! - rejecting duplicate return-block registrations;
//! - constructing plans from dynamically sized collections.
//!
//! It does not execute functions or prove that referenced entities exist.
//!
//! # Semantic contract
//!
//! A return block is a canonical block that the caller asserts terminates
//! by returning from the associated function.
//!
//! This plan does not encode the returned value. Return-value presence,
//! return types, result arity, and return-value compatibility are owned by
//! semantic analysis and canonical IR verification.
//!
//! The entry block may also be a return block for a function whose body
//! returns immediately. The plan does not prohibit this arrangement.
//!
//! A function declaration without a body does not necessarily need a
//! control-flow plan. Whether a body is required is determined by the
//! declaration context and semantic rules.
//!
//! # Determinism
//!
//! Return-block references are stored in a `BTreeSet`. Iteration order is
//! deterministic according to the canonical `BlockId` ordering; it does not
//! represent execution order or source order.
//!
//! # POCO-REAF and scalability
//!
//! This module imposes no fixed limit on the number of functions, return
//! blocks, parameters, source statements, or target devices.
//!
//! Storage grows with the supplied return-block set. Operations are subject
//! to ordinary allocation and computational costs. A process can fail to
//! allocate sufficient memory; this is not a language-level size limit.
//!
//! Compiler work budgets and runtime resource policies belong to their
//! respective owners and must not silently change function semantics.
//!
//! # Integration
//!
//! - `src/classical/control_flow/mod.rs` declares and exports this module.
//! - `src/classical/ids.rs` re-exports canonical `FunctionId` and `BlockId`.
//! - `src/classical/error.rs` provides `ClassicalError` and `ClassicalResult`.
//! - `src/classical/validation/` integrates semantic and structural checks.
//! - `src/classical/lowering/control_flow.rs` consumes validated control-flow
//!   plans when translating to canonical ZUIR.
//! - `src/zuir/ir/function.rs` remains the canonical function owner.
//! - The canonical ZUIR CFG builder and verifier establish block membership,
//!   terminator legality, return semantics, and graph correctness.
//! - `grammar/functions/functions.g4` remains the syntax authority.
//!
//! This module must not be used as a replacement for canonical IR verification.
//!
//! # Freeze rule
//!
//! Changes to this public contract require compatibility review of its
//! validation, lowering, and canonical IR consumers.

use std::collections::BTreeSet;

use crate::classical::error::{ClassicalError, ClassicalResult};
use crate::classical::ids::{BlockId, FunctionId};

/// Target-independent control-flow references for a function definition.
///
/// This is an adapter-level plan, not a second representation of the
/// function's signature, body, or canonical ZUIR definition.
///
/// The plan records identities only. It does not own the blocks or function
/// represented by those identities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionControlFlowPlan {
    function: FunctionId,
    entry: BlockId,
    return_blocks: BTreeSet<BlockId>,
}

impl FunctionControlFlowPlan {
    /// Creates a function control-flow plan with no registered return blocks.
    ///
    /// `function` and `entry` must be canonical identifiers belonging to
    /// the same compilation context. Their existence and ownership must be
    /// checked by the canonical IR verifier.
    ///
    /// This constructor is infallible because the supplied identifier types
    /// already enforce their local representation invariants.
    #[must_use]
    pub fn new(function: FunctionId, entry: BlockId) -> Self {
        Self {
            function,
            entry,
            return_blocks: BTreeSet::new(),
        }
    }

    /// Constructs a plan from an iterable of return-block references.
    ///
    /// The iterator may be dynamically sized. No arbitrary semantic limit
    /// is imposed on the number of return blocks.
    ///
    /// Duplicate block identifiers are rejected rather than silently
    /// discarded.
    ///
    /// # Errors
    ///
    /// Returns `ClassicalError::InvalidInput` if the same return block is
    /// supplied more than once.
    ///
    /// This function does not validate block existence, function ownership,
    /// return-value types, or terminator legality.
    pub fn with_return_blocks<I>(
        function: FunctionId,
        entry: BlockId,
        return_blocks: I,
    ) -> ClassicalResult<Self>
    where
        I: IntoIterator<Item = BlockId>,
    {
        let mut plan = Self::new(function, entry);

        for block in return_blocks {
            plan.add_return_block(block)?;
        }

        Ok(plan)
    }

    /// Returns the canonical function identifier.
    #[must_use]
    pub const fn function(&self) -> FunctionId {
        self.function
    }

    /// Returns the function entry-block identifier.
    #[must_use]
    pub const fn entry(&self) -> BlockId {
        self.entry
    }

    /// Returns the registered return blocks in deterministic identifier order.
    ///
    /// The order is for stable inspection, not execution scheduling.
    #[must_use]
    pub fn return_blocks(
        &self,
    ) -> impl ExactSizeIterator<Item = &BlockId> + DoubleEndedIterator {
        self.return_blocks.iter()
    }

    /// Returns the number of distinct registered return blocks.
    #[must_use]
    pub fn return_block_count(&self) -> usize {
        self.return_blocks.len()
    }

    /// Returns whether a block has been registered as a return block.
    #[must_use]
    pub fn contains_return_block(&self, block: &BlockId) -> bool {
        self.return_blocks.contains(block)
    }

    /// Registers a block that terminates by returning from this function.
    ///
    /// This operation checks only for duplicate registration. The caller
    /// remains responsible for ensuring that the block belongs to the
    /// canonical function and has a valid return terminator.
    ///
    /// # Errors
    ///
    /// Returns `ClassicalError::InvalidInput` if the block is already
    /// registered.
    pub fn add_return_block(&mut self, block: BlockId) -> ClassicalResult<()> {
        if !self.return_blocks.insert(block) {
            return Err(ClassicalError::invalid_input(format!(
                "return block {block} is already registered for function {}",
                self.function
            )));
        }

        Ok(())
    }

    /// Removes a registered return block.
    ///
    /// Returns `true` if the block was present and removed, or `false` if
    /// the plan did not contain that block.
    ///
    /// This does not mutate the canonical IR or remove a return terminator.
    pub fn remove_return_block(&mut self, block: &BlockId) -> bool {
        self.return_blocks.remove(block)
    }

    /// Consumes the plan and returns its canonical references.
    ///
    /// Return blocks are returned in deterministic identifier order.
    #[must_use]
    pub fn into_parts(self) -> (FunctionId, BlockId, BTreeSet<BlockId>) {
        (self.function, self.entry, self.return_blocks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn function_id(value: u128) -> FunctionId {
        FunctionId::new(value).expect("test function ID must be nonzero")
    }

    fn block_id(value: u128) -> BlockId {
        BlockId::new(value).expect("test block ID must be nonzero")
    }

    #[test]
    fn new_plan_preserves_canonical_references() {
        let function = function_id(1);
        let entry = block_id(2);

        let plan = FunctionControlFlowPlan::new(function, entry);

        assert_eq!(plan.function(), function);
        assert_eq!(plan.entry(), entry);
        assert_eq!(plan.return_block_count(), 0);
        assert_eq!(plan.return_blocks().count(), 0);
    }

    #[test]
    fn return_blocks_can_be_registered_and_removed() {
        let mut plan =
            FunctionControlFlowPlan::new(function_id(1), block_id(2));

        let first = block_id(3);
        let second = block_id(4);

        assert!(plan.add_return_block(first).is_ok());
        assert!(plan.add_return_block(second).is_ok());
        assert_eq!(plan.return_block_count(), 2);
        assert!(plan.contains_return_block(&first));
        assert!(plan.contains_return_block(&second));

        assert!(plan.remove_return_block(&first));
        assert!(!plan.contains_return_block(&first));
        assert_eq!(plan.return_block_count(), 1);

        assert!(!plan.remove_return_block(&first));
    }

    #[test]
    fn duplicate_return_block_registration_is_rejected() {
        let block = block_id(3);
        let mut plan =
            FunctionControlFlowPlan::new(function_id(1), block_id(2));

        plan.add_return_block(block)
            .expect("first registration should succeed");

        let result = plan.add_return_block(block);

        assert!(matches!(
            result,
            Err(ClassicalError::InvalidInput { .. })
        ));
        assert_eq!(plan.return_block_count(), 1);
    }

    #[test]
    fn bulk_construction_rejects_duplicates() {
        let result = FunctionControlFlowPlan::with_return_blocks(
            function_id(1),
            block_id(2),
            [block_id(3), block_id(4), block_id(3)],
        );

        assert!(matches!(
            result,
            Err(ClassicalError::InvalidInput { .. })
        ));
    }

    #[test]
    fn bulk_construction_preserves_all_distinct_return_blocks() {
        let plan = FunctionControlFlowPlan::with_return_blocks(
            function_id(1),
            block_id(2),
            [block_id(30), block_id(10), block_id(20)],
        )
        .expect("distinct return blocks should be accepted");

        assert_eq!(plan.return_block_count(), 3);

        let actual: Vec<BlockId> = plan.return_blocks().copied().collect();

        assert_eq!(
            actual,
            vec![block_id(10), block_id(20), block_id(30)]
        );
    }

    #[test]
    fn entry_block_may_also_be_a_return_block() {
        let entry = block_id(2);

        let plan = FunctionControlFlowPlan::with_return_blocks(
            function_id(1),
            entry,
            [entry],
        )
        .expect("a function may return directly from its entry block");

        assert_eq!(plan.entry(), entry);
        assert!(plan.contains_return_block(&entry));
    }

    #[test]
    fn return_block_count_is_not_fixed_by_a_semantic_limit() {
        // This is a test input size, not a production limit.
        let return_blocks = (1_u128..=2_048)
            .map(block_id)
            .collect::<Vec<_>>();

        let plan = FunctionControlFlowPlan::with_return_blocks(
            function_id(1),
            block_id(10_000),
            return_blocks,
        )
        .expect("all distinct supplied blocks should be retained");

        assert_eq!(plan.return_block_count(), 2_048);
    }

    #[test]
    fn large_nonzero_identifiers_are_preserved() {
        let function = function_id(u128::MAX);
        let entry = block_id(u128::MAX - 1);
        let returned = block_id(u128::MAX - 2);

        let plan = FunctionControlFlowPlan::with_return_blocks(
            function,
            entry,
            [returned],
        )
        .expect("large representable identifiers should be accepted");

        assert_eq!(plan.function(), function);
        assert_eq!(plan.entry(), entry);
        assert!(plan.contains_return_block(&returned));
    }

    #[test]
    fn decomposition_preserves_plan_contents() {
        let function = function_id(1);
        let entry = block_id(2);
        let first_return = block_id(3);
        let second_return = block_id(4);

        let plan = FunctionControlFlowPlan::with_return_blocks(
            function,
            entry,
            [second_return, first_return],
        )
        .expect("distinct return blocks should be accepted");

        let (actual_function, actual_entry, actual_returns) =
            plan.into_parts();

        assert_eq!(actual_function, function);
        assert_eq!(actual_entry, entry);
        assert!(actual_returns.contains(&first_return));
        assert!(actual_returns.contains(&second_return));
        assert_eq!(actual_returns.len(), 2);
    }

    #[test]
    fn different_functions_have_distinct_plan_identities() {
        let first =
            FunctionControlFlowPlan::new(function_id(1), block_id(10));
        let second =
            FunctionControlFlowPlan::new(function_id(2), block_id(10));

        assert_ne!(first.function(), second.function());
    }

    #[test]
    fn plans_support_clone_and_equality() {
        let plan = FunctionControlFlowPlan::with_return_blocks(
            function_id(1),
            block_id(2),
            [block_id(3), block_id(4)],
        )
        .expect("valid plan");

        assert_eq!(plan.clone(), plan);
    }
}
