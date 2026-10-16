
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Branch Planning
//!
//! Path: `src/classical/control_flow/branch.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines target-independent representations of classical conditional branch
//! plans. A plan identifies an already-resolved condition value and the
//! canonical control-flow block selected by each outcome.
//!
//! This module supports:
//!
//! - a single conditional branch;
//! - an optional alternative destination;
//! - ordered `else if` chains;
//! - an optional final `else` destination;
//! - deterministic inspection and decomposition of branch plans;
//! - structured construction errors;
//! - integration with canonical ZUIR identities.
//!
//! # Ownership
//!
//! This module owns only its branch-planning data structures and their local
//! construction invariants.
//!
//! It does not own:
//!
//! - source syntax or AST nodes;
//! - expression evaluation;
//! - condition type checking;
//! - name resolution;
//! - control-flow graph construction;
//! - canonical IR instructions or terminators;
//! - basic-block identity allocation;
//! - reachability or dominance analysis;
//! - optimization, scheduling, or execution;
//! - hardware discovery or resource allocation.
//!
//! # Canonical identity contract
//!
//! `ValueId` and `BlockId` are imported from the classical identity facade.
//! That facade re-exports the canonical identities owned by ZUIR.
//!
//! This module never creates replacement identifiers, interprets IDs as
//! collection indices, or assumes that IDs are contiguous.
//!
//! # Condition contract
//!
//! A `ValueId` identifies the result of a condition expression after the
//! applicable frontend and semantic stages have resolved it.
//!
//! This module does not establish that the value has Boolean type. The
//! semantic validator must establish condition validity before the branch
//! plan is lowered or executed.
//!
//! # Branch ordering
//!
//! Conditional arms are evaluated in source order by downstream consumers.
//! The first arm whose condition evaluates to true is selected. If no arm
//! matches, the optional fallback is selected when present.
//!
//! This module represents that ordering; it does not execute it.
//!
//! # POCO-REAF and scalability
//!
//! There is no fixed language-level limit on the number of conditional arms
//! or branch destinations represented by these structures.
//!
//! Storage is proportional to the number of arms supplied by the caller.
//! Construction can fail if the process cannot allocate the requested
//! storage; this module does not promise infinite physical memory.
//!
//! Operational budgets belong to the compiler configuration and resource
//! management owners, not to universal branch-count constants.
//!
//! # Integration
//!
//! - `src/classical/ids.rs`: canonical `ValueId` and `BlockId` facade.
//! - `src/classical/error.rs`: canonical `ClassicalError` and
//!   `ClassicalResult<T>`.
//! - `src/classical/control_flow/mod.rs`: declares this module.
//! - `src/classical/validation/`: validates condition types, effects,
//!   reachability, and control-flow invariants.
//! - `src/zuir/`: owns canonical control-flow IR and CFG verification.
//! - `grammar/statements/conditionals.g4`: owns statement-level conditional
//!   syntax.
//! - `grammar/expressions/conditionals.g4`: owns value-producing conditional
//!   expression syntax.
//!
//! This module must not become a second AST or IR implementation.
//!
//! # Freeze rule
//!
//! Public structure changes require compatibility review. Changes to the
//! canonical ID or error owners must be handled by those owners rather than
//! duplicated here.

use crate::classical::error::{ClassicalError, ClassicalResult};
use crate::classical::ids::{BlockId, ValueId};

/// One conditional branch with an optional alternative destination.
///
/// The condition and destinations refer to entities owned by the canonical
/// IR. No source expression, IR instruction, or runtime value is copied into
/// this structure.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BranchPlan {
    condition: ValueId,
    then_target: BlockId,
    else_target: Option<BlockId>,
}

impl BranchPlan {
    /// Creates a branch plan.
    ///
    /// This constructor checks only that the caller has supplied the required
    /// structural references. The identifier types guarantee their own
    /// representation invariants.
    ///
    /// It does not verify that the referenced values or blocks exist in a
    /// particular IR, or that the condition has Boolean type.
    #[must_use]
    pub fn new(
        condition: ValueId,
        then_target: BlockId,
        else_target: Option<BlockId>,
    ) -> Self {
        Self {
            condition,
            then_target,
            else_target,
        }
    }

    /// Returns the condition value identifier.
    #[must_use]
    pub fn condition(&self) -> &ValueId {
        &self.condition
    }

    /// Returns the destination selected when the condition is true.
    #[must_use]
    pub fn then_target(&self) -> &BlockId {
        &self.then_target
    }

    /// Returns the optional destination selected when the condition is false.
    ///
    /// `None` means this plan does not explicitly specify a false destination.
    /// The enclosing CFG or lowering contract must determine the appropriate
    /// fall-through or merge behavior.
    #[must_use]
    pub fn else_target(&self) -> Option<&BlockId> {
        self.else_target.as_ref()
    }

    /// Replaces the condition reference and returns the previous identifier.
    ///
    /// The caller must re-run semantic and IR validation after changing a
    /// plan already incorporated into a compilation pipeline.
    pub fn replace_condition(&mut self, condition: ValueId) -> ValueId {
        core::mem::replace(&mut self.condition, condition)
    }

    /// Replaces the true destination and returns the previous identifier.
    pub fn replace_then_target(&mut self, target: BlockId) -> BlockId {
        core::mem::replace(&mut self.then_target, target)
    }

    /// Replaces the optional false destination and returns the previous value.
    pub fn replace_else_target(
        &mut self,
        target: Option<BlockId>,
    ) -> Option<BlockId> {
        core::mem::replace(&mut self.else_target, target)
    }

    /// Consumes the plan and returns its canonical references.
    #[must_use]
    pub fn into_parts(self) -> (ValueId, BlockId, Option<BlockId>) {
        (self.condition, self.then_target, self.else_target)
    }
}

/// One ordered arm in a conditional branch chain.
///
/// Each arm associates one resolved condition value with the block to select
/// when that condition is true.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConditionalArm {
    condition: ValueId,
    target: BlockId,
}

impl ConditionalArm {
    /// Creates an ordered conditional arm.
    #[must_use]
    pub fn new(condition: ValueId, target: BlockId) -> Self {
        Self { condition, target }
    }

    /// Returns the condition value identifier.
    #[must_use]
    pub fn condition(&self) -> &ValueId {
        &self.condition
    }

    /// Returns the destination selected by this arm.
    #[must_use]
    pub fn target(&self) -> &BlockId {
        &self.target
    }

    /// Consumes the arm and returns its canonical references.
    #[must_use]
    pub fn into_parts(self) -> (ValueId, BlockId) {
        (self.condition, self.target)
    }
}

/// An ordered conditional chain with an optional final fallback.
///
/// Arms preserve source order. Consumers must not reorder arms unless the
/// relevant semantic and effect analyses prove that reordering is legal.
///
/// A chain must contain at least one conditional arm. A standalone block
/// without a condition is not a conditional chain and should be represented
/// by the enclosing control-flow model instead.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BranchChain {
    arms: Vec<ConditionalArm>,
    fallback: Option<BlockId>,
}

impl BranchChain {
    /// Constructs a conditional chain.
    ///
    /// The vector's length is determined by the input and is not subject to a
    /// hard-coded language-level branch limit.
    ///
    /// # Errors
    ///
    /// Returns `ClassicalError::InvalidInput` if `arms` is empty.
    ///
    /// This method does not check condition types, block existence, graph
    /// reachability, ownership, or IR validity. Those checks require the
    /// canonical semantic or IR context.
    pub fn new(
        arms: Vec<ConditionalArm>,
        fallback: Option<BlockId>,
    ) -> ClassicalResult<Self> {
        if arms.is_empty() {
            return Err(ClassicalError::invalid_input(
                "a conditional branch chain must contain at least one arm",
            ));
        }

        Ok(Self { arms, fallback })
    }

    /// Returns the ordered conditional arms.
    #[must_use]
    pub fn arms(&self) -> &[ConditionalArm] {
        &self.arms
    }

    /// Returns the optional fallback destination.
    #[must_use]
    pub fn fallback(&self) -> Option<&BlockId> {
        self.fallback.as_ref()
    }

    /// Returns the number of conditional arms.
    #[must_use]
    pub fn arm_count(&self) -> usize {
        self.arms.len()
    }

    /// Returns whether a final fallback destination exists.
    #[must_use]
    pub fn has_fallback(&self) -> bool {
        self.fallback.is_some()
    }

    /// Returns the first conditional arm, which is guaranteed to exist for
    /// values created through `BranchChain::new`.
    #[must_use]
    pub fn first_arm(&self) -> &ConditionalArm {
        // Construction rejects empty chains. The private field prevents
        // external callers from making a valid chain empty.
        &self.arms[0]
    }

    /// Consumes the chain and returns its ordered components.
    #[must_use]
    pub fn into_parts(self) -> (Vec<ConditionalArm>, Option<BlockId>) {
        (self.arms, self.fallback)
    }

    /// Replaces the fallback destination and returns the previous value.
    pub fn replace_fallback(
        &mut self,
        fallback: Option<BlockId>,
    ) -> Option<BlockId> {
        core::mem::replace(&mut self.fallback, fallback)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value_id(value: u128) -> ValueId {
        ValueId::new(value).expect("test value ID must be valid")
    }

    fn block_id(value: u128) -> BlockId {
        BlockId::new(value).expect("test block ID must be valid")
    }

    #[test]
    fn single_branch_preserves_canonical_references() {
        let condition = value_id(1);
        let then_target = block_id(2);
        let else_target = block_id(3);

        let plan = BranchPlan::new(
            condition.clone(),
            then_target.clone(),
            Some(else_target.clone()),
        );

        assert_eq!(plan.condition(), &condition);
        assert_eq!(plan.then_target(), &then_target);
        assert_eq!(plan.else_target(), Some(&else_target));
    }

    #[test]
    fn single_branch_can_omit_an_explicit_else_destination() {
        let plan = BranchPlan::new(value_id(1), block_id(2), None);

        assert!(plan.else_target().is_none());
    }

    #[test]
    fn branch_replacements_return_previous_references() {
        let mut plan =
            BranchPlan::new(value_id(1), block_id(2), Some(block_id(3)));

        assert_eq!(plan.replace_condition(value_id(4)), value_id(1));
        assert_eq!(plan.replace_then_target(block_id(5)), block_id(2));
        assert_eq!(
            plan.replace_else_target(None),
            Some(block_id(3))
        );

        assert_eq!(plan.condition(), &value_id(4));
        assert_eq!(plan.then_target(), &block_id(5));
        assert!(plan.else_target().is_none());
    }

    #[test]
    fn conditional_chain_preserves_source_order() {
        let arms = vec![
            ConditionalArm::new(value_id(1), block_id(10)),
            ConditionalArm::new(value_id(2), block_id(20)),
            ConditionalArm::new(value_id(3), block_id(30)),
        ];

        let chain = BranchChain::new(arms, Some(block_id(40)))
            .expect("a non-empty chain must be valid");

        assert_eq!(chain.arm_count(), 3);
        assert_eq!(chain.arms()[0].condition(), &value_id(1));
        assert_eq!(chain.arms()[1].condition(), &value_id(2));
        assert_eq!(chain.arms()[2].condition(), &value_id(3));
        assert_eq!(chain.fallback(), Some(&block_id(40)));
    }

    #[test]
    fn empty_conditional_chain_is_rejected() {
        let result = BranchChain::new(Vec::new(), None);

        assert!(matches!(
            result,
            Err(ClassicalError::InvalidInput { .. })
        ));
    }

    #[test]
    fn chain_without_fallback_is_valid() {
        let chain = BranchChain::new(
            vec![ConditionalArm::new(value_id(1), block_id(2))],
            None,
        )
        .expect("a non-empty chain may omit its fallback");

        assert_eq!(chain.arm_count(), 1);
        assert!(!chain.has_fallback());
    }

    #[test]
    fn fallback_can_be_replaced() {
        let mut chain = BranchChain::new(
            vec![ConditionalArm::new(value_id(1), block_id(2))],
            Some(block_id(3)),
        )
        .expect("a non-empty chain must be valid");

        assert_eq!(chain.replace_fallback(Some(block_id(4))), Some(block_id(3)));
        assert_eq!(chain.fallback(), Some(&block_id(4)));

        assert_eq!(chain.replace_fallback(None), Some(block_id(4)));
        assert!(!chain.has_fallback());
    }

    #[test]
    fn large_arm_counts_are_not_restricted_by_a_semantic_constant() {
        // This test verifies that the API accepts a caller-provided collection
        // without an arbitrary branch-count ceiling. Allocation remains
        // subject to the host and configured operational budgets.
        let arms = (1_u128..=1_024)
            .map(|id| ConditionalArm::new(value_id(id), block_id(id + 10_000)))
            .collect::<Vec<_>>();

        let chain = BranchChain::new(arms, None)
            .expect("all supplied arms must be preserved");

        assert_eq!(chain.arm_count(), 1_024);
        assert_eq!(chain.arms().first().map(ConditionalArm::condition), Some(&value_id(1)));
        assert_eq!(chain.arms().last().map(ConditionalArm::condition), Some(&value_id(1_024)));
    }

    #[test]
    fn decomposition_preserves_order_and_fallback() {
        let arms = vec![
            ConditionalArm::new(value_id(1), block_id(11)),
            ConditionalArm::new(value_id(2), block_id(22)),
        ];

        let chain = BranchChain::new(arms, Some(block_id(33)))
            .expect("a non-empty chain must be valid");

        let (arms, fallback) = chain.into_parts();

        assert_eq!(arms.len(), 2);
        assert_eq!(arms[0].target(), &block_id(11));
        assert_eq!(arms[1].target(), &block_id(22));
        assert_eq!(fallback, Some(block_id(33)));
    }
}
