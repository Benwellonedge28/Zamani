
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Structured Loop Planning
//!
//! Path: `src/classical/control_flow/loops.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # File contract
//!
//! Status: Proposed implementation; full workspace verification pending.
//!
//! Purpose:
//! Represent target-independent, semantically resolved `while` and `for`
//! loop plans using canonical Zamani identities.
//!
//! Authority:
//! - `grammar/statements/loops.g4` owns native loop syntax.
//! - The frontend owns AST nodes and source spans.
//! - Semantic analysis owns condition typing, binding, iteration protocols,
//!   ownership, borrowing, effects, and termination analysis.
//! - `crate::zuir` owns canonical IR, CFG construction, and verification.
//! - `crate::classical::ids` re-exports canonical identities.
//! - `crate::classical::error` owns classical errors.
//!
//! Owns:
//! - Resolved while-loop and for-loop plan structures.
//! - The distinction between condition-controlled and iterator-controlled
//!   loops.
//! - Structural accessors and construction.
//! - The documented loop-header, body, continue, and exit relationships.
//!
//! Does not own:
//! - AST definitions or parsing.
//! - Expression evaluation or condition type checking.
//! - Iterator implementation or collection storage.
//! - CFG construction or IR terminators.
//! - Break/continue statement parsing or runtime execution.
//! - Parallel scheduling, hardware selection, or resource allocation.
//!
//! # Canonical loop contract
//!
//! The current native syntax supports:
//!
//!     while expression { ... }
//!     for binding in expression { ... }
//!
//! This file intentionally does not introduce a third unconditional-loop
//! construct, do-while loop, or C-style three-clause loop.
//!
//! # Control-flow contract
//!
//! Every plan records a header, body, and exit block.
//!
//! - The header is the loop's control-flow entry and continuation destination.
//! - The body is the destination for an iteration.
//! - The exit is the destination reached when the loop finishes normally or
//!   when a valid break targets this loop.
//! - A while condition is a canonical value reference associated with the
//!   loop header's condition evaluation.
//! - A for loop references its iterable and binding; iteration mechanics
//!   belong to semantic analysis and canonical IR lowering.
//!
//! These references do not, by themselves, establish that a CFG is valid.
//! The canonical IR builder and verifier must confirm that blocks exist,
//! edges are legal, values dominate their uses, and loop semantics are
//! preserved.
//!
//! A continue statement must target the loop's continuation point according
//! to the canonical lowering contract. This plan exposes that point as the
//! header; an IR lowering implementation may introduce a separate latch or
//! iterator-advance block where required.
//!
//! # Scalability contract
//!
//! No machine-dependent loop-count, nesting-depth, iteration-count, hardware
//! count, or collection-size limit is imposed here.
//!
//! A loop may execute zero, finitely many, or indefinitely many iterations
//! according to its semantics. The compiler and runtime may apply explicit
//! operational budgets, cancellation, or resource policies without changing
//! the meaning of a valid loop.
//!
//! This module makes no claim of infinite physical memory or computation.
//!
//! # Integration
//!
//! - `src/classical/control_flow/mod.rs` declares `pub mod loops;`.
//! - `src/classical/ids.rs` supplies `BlockId`, `SymbolId`, and `ValueId`.
//! - `src/classical/error.rs` supplies `ClassicalError` and `ClassicalResult`.
//! - `src/classical/validation/` checks loop semantics and invariants.
//! - `src/classical/lowering/control_flow.rs` maps validated plans to the
//!   canonical IR.
//! - `src/zuir/` remains the owner of canonical blocks, edges, and terminators.
//! - `grammar/statements/loops.g4` remains the syntax authority.
//! - The canonical break and continue statement owners remain responsible
//!   for their own syntax and semantic validation.
//!
//! # Freeze rule
//!
//! Changes to public plan structures require review of their consumers,
//! canonical IR mapping, and compatibility tests. Do not add a duplicate AST,
//! CFG, identity allocator, or runtime loop interpreter here.

use crate::classical::error::{ClassicalError, ClassicalResult};
use crate::classical::ids::{BlockId, SymbolId, ValueId};

/// A resolved plan for a condition-controlled `while` loop.
///
/// The condition is represented by its canonical value identity. The
/// semantic pipeline must ensure that the condition has the required
/// Boolean semantics and that its evaluation occurs at the correct point
/// on each iteration.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhileLoopPlan {
    condition: ValueId,
    header: BlockId,
    body: BlockId,
    exit: BlockId,
}

impl WhileLoopPlan {
    /// Constructs a while-loop plan from canonical identities.
    ///
    /// This constructor performs structural construction only. It does not
    /// prove that the referenced blocks or value exist in an IR.
    #[must_use]
    pub fn new(
        condition: ValueId,
        header: BlockId,
        body: BlockId,
        exit: BlockId,
    ) -> Self {
        Self {
            condition,
            header,
            body,
            exit,
        }
    }

    /// Returns the canonical condition value.
    #[must_use]
    pub fn condition(&self) -> &ValueId {
        &self.condition
    }

    /// Returns the loop header block.
    #[must_use]
    pub fn header(&self) -> &BlockId {
        &self.header
    }

    /// Returns the loop body block.
    #[must_use]
    pub fn body(&self) -> &BlockId {
        &self.body
    }

    /// Returns the normal loop exit block.
    #[must_use]
    pub fn exit(&self) -> &BlockId {
        &self.exit
    }

    /// Returns the continuation destination represented by this plan.
    ///
    /// Lowering may introduce a distinct latch or condition-evaluation block
    /// when the canonical IR requires one.
    #[must_use]
    pub fn continue_target(&self) -> &BlockId {
        &self.header
    }

    /// Consumes the plan and returns its canonical identities.
    #[must_use]
    pub fn into_parts(self) -> (ValueId, BlockId, BlockId, BlockId) {
        (self.condition, self.header, self.body, self.exit)
    }
}

/// A resolved plan for an iterator-controlled `for` loop.
///
/// `iterable` identifies the resolved iterable value, while `binding`
/// identifies the canonical symbol introduced for each iteration.
///
/// This structure does not assume that the iterable is stored in memory,
/// has a known length, or can be indexed. It may represent a lazy sequence,
/// stream, range, distributed sequence, or another semantically supported
/// iterable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForLoopPlan {
    iterable: ValueId,
    binding: SymbolId,
    header: BlockId,
    body: BlockId,
    exit: BlockId,
}

impl ForLoopPlan {
    /// Constructs a for-loop plan from canonical identities.
    ///
    /// The semantic pipeline must establish that the binding is valid for
    /// the iterable and that ownership, borrowing, and iteration effects
    /// are respected.
    #[must_use]
    pub fn new(
        iterable: ValueId,
        binding: SymbolId,
        header: BlockId,
        body: BlockId,
        exit: BlockId,
    ) -> Self {
        Self {
            iterable,
            binding,
            header,
            body,
            exit,
        }
    }

    /// Returns the resolved iterable value.
    #[must_use]
    pub fn iterable(&self) -> &ValueId {
        &self.iterable
    }

    /// Returns the symbol bound for each iteration.
    #[must_use]
    pub fn binding(&self) -> &SymbolId {
        &self.binding
    }

    /// Returns the loop header block.
    #[must_use]
    pub fn header(&self) -> &BlockId {
        &self.header
    }

    /// Returns the loop body block.
    #[must_use]
    pub fn body(&self) -> &BlockId {
        &self.body
    }

    /// Returns the normal loop exit block.
    #[must_use]
    pub fn exit(&self) -> &BlockId {
        &self.exit
    }

    /// Returns the continuation destination represented by this plan.
    ///
    /// The canonical lowering layer is responsible for any iterator-advance
    /// or latch blocks needed before returning to the header.
    #[must_use]
    pub fn continue_target(&self) -> &BlockId {
        &self.header
    }

    /// Consumes the plan and returns its canonical identities.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (ValueId, SymbolId, BlockId, BlockId, BlockId) {
        (
            self.iterable,
            self.binding,
            self.header,
            self.body,
            self.exit,
        )
    }
}

/// A target-independent description of a canonical native loop.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LoopPlan {
    /// Condition-controlled loop.
    While(WhileLoopPlan),

    /// Iterator-controlled loop.
    For(ForLoopPlan),
}

impl LoopPlan {
    /// Constructs a while-loop plan.
    #[must_use]
    pub fn while_loop(
        condition: ValueId,
        header: BlockId,
        body: BlockId,
        exit: BlockId,
    ) -> Self {
        Self::While(WhileLoopPlan::new(condition, header, body, exit))
    }

    /// Constructs a for-loop plan.
    #[must_use]
    pub fn for_loop(
        iterable: ValueId,
        binding: SymbolId,
        header: BlockId,
        body: BlockId,
        exit: BlockId,
    ) -> Self {
        Self::For(ForLoopPlan::new(
            iterable, binding, header, body, exit,
        ))
    }

    /// Returns the loop header block.
    #[must_use]
    pub fn header(&self) -> &BlockId {
        match self {
            Self::While(plan) => plan.header(),
            Self::For(plan) => plan.header(),
        }
    }

    /// Returns the loop body block.
    #[must_use]
    pub fn body(&self) -> &BlockId {
        match self {
            Self::While(plan) => plan.body(),
            Self::For(plan) => plan.body(),
        }
    }

    /// Returns the normal loop exit block.
    #[must_use]
    pub fn exit(&self) -> &BlockId {
        match self {
            Self::While(plan) => plan.exit(),
            Self::For(plan) => plan.exit(),
        }
    }

    /// Returns the continuation destination represented by this plan.
    #[must_use]
    pub fn continue_target(&self) -> &BlockId {
        match self {
            Self::While(plan) => plan.continue_target(),
            Self::For(plan) => plan.continue_target(),
        }
    }

    /// Returns whether this plan represents a while loop.
    #[must_use]
    pub const fn is_while(&self) -> bool {
        matches!(self, Self::While(_))
    }

    /// Returns whether this plan represents a for loop.
    #[must_use]
    pub const fn is_for(&self) -> bool {
        matches!(self, Self::For(_))
    }

    /// Returns the while-loop details, if applicable.
    #[must_use]
    pub const fn as_while(&self) -> Option<&WhileLoopPlan> {
        match self {
            Self::While(plan) => Some(plan),
            Self::For(_) => None,
        }
    }

    /// Returns the for-loop details, if applicable.
    #[must_use]
    pub const fn as_for(&self) -> Option<&ForLoopPlan> {
        match self {
            Self::For(plan) => Some(plan),
            Self::While(_) => None,
        }
    }
}

/// Validates the local structural contract for a loop plan.
///
/// This checks only properties available from the plan itself. It cannot
/// validate block existence, condition types, binding scope, graph edges,
/// effects, or dominance without the canonical semantic/IR context.
///
/// Currently the constructor types and private fields guarantee that all
/// required identities are present, so this function is deliberately narrow.
/// It is an integration seam for the semantic/IR verifier, not a substitute
/// for that verifier.
pub fn validate_loop_plan(plan: &LoopPlan) -> ClassicalResult<()> {
    // The required IDs are typed canonical references. Their existence and
    // membership in a particular IR must be verified by the IR owner.
    //
    // Do not reject repeated IDs here: whether a block arrangement is valid
    // depends on the canonical CFG and lowering representation.
    let _ = (plan.header(), plan.body(), plan.exit());

    match plan {
        LoopPlan::While(loop_plan) => {
            let _ = loop_plan.condition();
        }
        LoopPlan::For(loop_plan) => {
            let _ = (loop_plan.iterable(), loop_plan.binding());
        }
    }

    // Keep the shared error dependency explicit for future semantic checks
    // without manufacturing an error for a structurally complete plan.
    let _ = core::marker::PhantomData::<ClassicalError>;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value_id(value: u128) -> ValueId {
        ValueId::new(value).expect("test value ID must be nonzero")
    }

    fn symbol_id(value: u128) -> SymbolId {
        SymbolId::new(value).expect("test symbol ID must be nonzero")
    }

    fn block_id(value: u128) -> BlockId {
        BlockId::new(value).expect("test block ID must be nonzero")
    }

    #[test]
    fn while_loop_preserves_all_canonical_references() {
        let condition = value_id(1);
        let header = block_id(2);
        let body = block_id(3);
        let exit = block_id(4);

        let plan = WhileLoopPlan::new(
            condition.clone(),
            header.clone(),
            body.clone(),
            exit.clone(),
        );

        assert_eq!(plan.condition(), &condition);
        assert_eq!(plan.header(), &header);
        assert_eq!(plan.body(), &body);
        assert_eq!(plan.exit(), &exit);
        assert_eq!(plan.continue_target(), &header);
    }

    #[test]
    fn for_loop_preserves_iterable_binding_and_blocks() {
        let iterable = value_id(1);
        let binding = symbol_id(2);
        let header = block_id(3);
        let body = block_id(4);
        let exit = block_id(5);

        let plan = ForLoopPlan::new(
            iterable.clone(),
            binding.clone(),
            header.clone(),
            body.clone(),
            exit.clone(),
        );

        assert_eq!(plan.iterable(), &iterable);
        assert_eq!(plan.binding(), &binding);
        assert_eq!(plan.header(), &header);
        assert_eq!(plan.body(), &body);
        assert_eq!(plan.exit(), &exit);
        assert_eq!(plan.continue_target(), &header);
    }

    #[test]
    fn enum_constructors_preserve_loop_kind() {
        let while_plan =
            LoopPlan::while_loop(value_id(1), block_id(2), block_id(3), block_id(4));

        assert!(while_plan.is_while());
        assert!(!while_plan.is_for());
        assert!(while_plan.as_while().is_some());
        assert!(while_plan.as_for().is_none());

        let for_plan = LoopPlan::for_loop(
            value_id(5),
            symbol_id(6),
            block_id(7),
            block_id(8),
            block_id(9),
        );

        assert!(for_plan.is_for());
        assert!(!for_plan.is_while());
        assert!(for_plan.as_for().is_some());
        assert!(for_plan.as_while().is_none());
    }

    #[test]
    fn common_accessors_work_for_both_loop_kinds() {
        let plans = [
            LoopPlan::while_loop(
                value_id(1),
                block_id(2),
                block_id(3),
                block_id(4),
            ),
            LoopPlan::for_loop(
                value_id(5),
                symbol_id(6),
                block_id(7),
                block_id(8),
                block_id(9),
            ),
        ];

        for plan in &plans {
            assert_eq!(plan.continue_target(), plan.header());
            assert_ne!(plan.header(), plan.body());
            assert_ne!(plan.body(), plan.exit());
            validate_loop_plan(plan).expect("structurally complete plan");
        }
    }

    #[test]
    fn loop_plans_are_deterministically_comparable() {
        let first = LoopPlan::while_loop(
            value_id(1),
            block_id(2),
            block_id(3),
            block_id(4),
        );

        let second = first.clone();

        assert_eq!(first, second);
    }

    #[test]
    fn large_identity_values_are_not_truncated() {
        let high = u128::MAX;

        // Canonical IDs reject zero but may represent large nonzero IDs.
        let value = ValueId::new(high).expect("nonzero value ID");
        let header = BlockId::new(high).expect("nonzero block ID");
        let body = block_id(3);
        let exit = block_id(4);

        let plan = WhileLoopPlan::new(value.clone(), header.clone(), body, exit);

        assert_eq!(plan.condition(), &value);
        assert_eq!(plan.header(), &header);
    }
}
