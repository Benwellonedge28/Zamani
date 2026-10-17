
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Target-independent lowering orchestration for classical control flow.
//!
//! Path: `src/classical/lowering/control_flow.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Connect validated classical control-flow plans to an adapter that builds
//! the canonical destination IR. This module supports the existing branch,
//! conditional-chain, loop, and function-control-flow plan types.
//!
//! # Authority
//!
//! - `grammar/statements/conditionals.g4` owns statement conditional syntax.
//! - `grammar/statements/loops.g4` owns native `while` and `for` syntax.
//! - The frontend owns parsed AST nodes and source locations.
//! - Semantic analysis owns name resolution, condition typing, binding,
//!   effects, ownership, borrowing, return validation, and control-transfer
//!   legality.
//! - `crate::classical::control_flow` owns the source-independent planning
//!   types consumed here.
//! - `crate::classical::ids` re-exports canonical ZUIR identities.
//! - `crate::zuir` owns canonical IR entities and their verification.
//!
//! # Ownership
//!
//! This module owns the lowering adapter contract, work-budget accounting,
//! and orchestration of existing plans.
//!
//! It does not own a second AST, CFG, IR, identity allocator, semantic
//! analyzer, runtime, scheduler, or hardware abstraction.
//!
//! # Semantic preservation
//!
//! Lowering must preserve conditional-arm order, loop kind, loop references,
//! function entry, and registered return blocks. No condition, branch,
//! return, or loop may be silently discarded.
//!
//! Passing a plan to this module does not prove that its references exist in
//! the destination IR or that its semantics are valid. The semantic analyzer
//! and canonical IR verifier remain authoritative.
//!
//! # Transaction contract
//!
//! Each `ControlFlowLoweringSink` method must apply one complete plan
//! atomically: on `Err`, the destination representation must be unchanged.
//! This prevents a failed operation from leaving half a branch chain or a
//! partially registered function flow in the destination.
//!
//! # Scalability
//!
//! There is no fixed semantic ceiling on branch-arm count, return-block
//! count, function count, or loop count. Work is accounted for using a
//! caller-supplied optional budget. Exhaustion is reported explicitly.
//! Normal allocation, address-space, execution-time, and target constraints
//! still apply.
//!
//! # Integration
//!
//! `src/classical/lowering/mod.rs` must declare `pub mod control_flow;`.
//! A concrete sink must be implemented in the adapter that owns the actual
//! canonical ZUIR builder. That sink must use the repository's canonical
//! identifiers and verifier; it must not define replacement IR types.
//!
//! This module deliberately does not guess constructors or mutation methods
//! for ZUIR blocks, regions, operations, or terminators.

use crate::classical::control_flow::branch::{BranchChain, BranchPlan};
use crate::classical::control_flow::functions::FunctionControlFlowPlan;
use crate::classical::control_flow::loops::{validate_loop_plan, LoopPlan};
use crate::classical::error::{ClassicalError, ClassicalResult};

/// Explicit operational options for control-flow lowering.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ControlFlowLoweringOptions {
    /// Optional budget for charged lowering work units.
    ///
    /// One unit is charged per conditional arm for a branch chain, one for a
    /// single branch, one for a loop plan, and one plus each registered
    /// return block for a function plan.
    ///
    /// `None` means no work-unit limit is imposed by these options. It does
    /// not promise infinite computation or memory.
    pub work_budget: Option<u64>,
}

/// Adapter to the canonical destination IR builder.
///
/// Implementations must use canonical IR types and identities. They must
/// verify destination references and preserve all semantics represented by
/// each input plan.
///
/// Each method must be atomic with respect to its destination: it either
/// completes the entire requested plan or returns an error without leaving
/// partial changes.
///
/// This trait is an integration boundary, not a replacement IR. The
/// destination-specific implementation belongs with the canonical builder.
pub trait ControlFlowLoweringSink {
    /// Emits one conditional branch plan into the canonical IR.
    ///
    /// The condition must retain its resolved meaning. The optional `else`
    /// target must not be replaced with an invented destination.
    fn emit_branch_plan(&mut self, plan: &BranchPlan) -> ClassicalResult<()>;

    /// Emits a conditional chain, preserving arm order and fallback behavior.
    ///
    /// Implementations must not reorder arms unless a separate, valid
    /// semantic transformation has proved that reordering is legal.
    fn emit_branch_chain(&mut self, plan: &BranchChain) -> ClassicalResult<()>;

    /// Emits a validated while-loop or for-loop plan.
    ///
    /// Implementations must preserve the loop kind, header, body, exit, and
    /// associated condition or iterable/binding references. Any required
    /// latch, iterator-advance, or condition-evaluation blocks must be
    /// created through the canonical builder.
    fn emit_loop_plan(&mut self, plan: &LoopPlan) -> ClassicalResult<()>;

    /// Emits the control-flow associations for one canonical function.
    ///
    /// The function, entry, and return-block references must remain canonical
    /// identifiers. This method does not replace function-signature
    /// validation or canonical terminator verification.
    fn emit_function_flow(
        &mut self,
        plan: &FunctionControlFlowPlan,
    ) -> ClassicalResult<()>;
}

/// A control-flow lowering orchestrator.
///
/// The orchestrator owns neither the canonical IR nor the sink's builder.
/// It retains only the sink and the work-accounting state for this lowering
/// session.
pub struct ControlFlowLowerer<S> {
    sink: S,
    work_budget: Option<u64>,
    charged_work: u64,
}

impl<S> ControlFlowLowerer<S>
where
    S: ControlFlowLoweringSink,
{
    /// Creates a lowering orchestrator around a canonical-IR adapter.
    #[must_use]
    pub fn new(sink: S, options: ControlFlowLoweringOptions) -> Self {
        Self {
            sink,
            work_budget: options.work_budget,
            charged_work: 0,
        }
    }

    /// Returns the number of work units charged in this session.
    #[must_use]
    pub const fn charged_work(&self) -> u64 {
        self.charged_work
    }

    /// Returns the configured work budget, if any.
    #[must_use]
    pub const fn work_budget(&self) -> Option<u64> {
        self.work_budget
    }

    /// Returns a shared reference to the destination adapter.
    #[must_use]
    pub const fn sink(&self) -> &S {
        &self.sink
    }

    /// Returns a mutable reference to the destination adapter.
    ///
    /// Direct sink access is useful for adapter-specific operations. Callers
    /// remain responsible for preserving the sink's transaction contract.
    pub fn sink_mut(&mut self) -> &mut S {
        &mut self.sink
    }

    /// Consumes the orchestrator and returns the destination adapter.
    #[must_use]
    pub fn into_sink(self) -> S {
        self.sink
    }

    /// Lowers one branch plan through the canonical adapter.
    ///
    /// Work is charged before the adapter is called. If the adapter fails,
    /// the attempted work remains charged, but the sink must remain
    /// unchanged as required by [`ControlFlowLoweringSink`].
    pub fn lower_branch(&mut self, plan: &BranchPlan) -> ClassicalResult<()> {
        self.charge_work(1)?;
        self.sink.emit_branch_plan(plan)
    }

    /// Lowers a complete conditional chain through the canonical adapter.
    ///
    /// The entire chain is passed as one transaction so a sink can preserve
    /// source ordering and avoid partial emission on failure.
    pub fn lower_branch_chain(
        &mut self,
        plan: &BranchChain,
    ) -> ClassicalResult<()> {
        let work = u64::try_from(plan.arm_count()).map_err(|_| {
            ClassicalError::RepresentationFailure {
                message: "conditional arm count cannot be represented by the lowering work counter"
                    .to_owned(),
            }
        })?;

        self.charge_work(work)?;
        self.sink.emit_branch_chain(plan)
    }

    /// Lowers one loop plan after its local structural validation.
    ///
    /// This validation does not prove that blocks or values exist in the
    /// destination IR. The canonical verifier must establish those facts.
    pub fn lower_loop(&mut self, plan: &LoopPlan) -> ClassicalResult<()> {
        validate_loop_plan(plan)?;
        self.charge_work(1)?;
        self.sink.emit_loop_plan(plan)
    }

    /// Lowers one function's control-flow associations.
    ///
    /// The cost includes one unit for the function plan and one for every
    /// registered return block. Return blocks are owned by the plan and are
    /// inspected in its deterministic identity order by the sink.
    pub fn lower_function_flow(
        &mut self,
        plan: &FunctionControlFlowPlan,
    ) -> ClassicalResult<()> {
        let return_count = u64::try_from(plan.return_block_count()).map_err(|_| {
            ClassicalError::RepresentationFailure {
                message: "return-block count cannot be represented by the lowering work counter"
                    .to_owned(),
            }
        })?;

        let work = return_count.checked_add(1).ok_or_else(|| {
            ClassicalError::RepresentationFailure {
                message: "function lowering work count overflowed".to_owned(),
            }
        })?;

        self.charge_work(work)?;
        self.sink.emit_function_flow(plan)
    }

    fn charge_work(&mut self, requested: u64) -> ClassicalResult<()> {
        let next = self.charged_work.checked_add(requested).ok_or_else(|| {
            ClassicalError::RepresentationFailure {
                message: "control-flow lowering work counter overflowed".to_owned(),
            }
        })?;

        if let Some(budget) = self.work_budget {
            if next > budget {
                return Err(ClassicalError::ResourceLimitExceeded {
                    resource: "classical.control_flow.lowering_work_units".to_owned(),
                    requested: Some(next),
                    available: Some(budget),
                });
            }
        }

        self.charged_work = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classical::control_flow::branch::ConditionalArm;
    use crate::classical::control_flow::loops::LoopPlan;
    use crate::classical::ids::{BlockId, FunctionId, SymbolId, ValueId};

    #[derive(Default)]
    struct RecordingSink {
        branches: usize,
        chains: usize,
        loops: usize,
        functions: usize,
        fail_next: bool,
    }

    impl RecordingSink {
        fn maybe_fail(&mut self) -> ClassicalResult<()> {
            if self.fail_next {
                self.fail_next = false;
                return Err(ClassicalError::ValidationFailed {
                    message: "test sink rejected the plan".to_owned(),
                });
            }
            Ok(())
        }
    }

    impl ControlFlowLoweringSink for RecordingSink {
        fn emit_branch_plan(&mut self, _plan: &BranchPlan) -> ClassicalResult<()> {
            self.maybe_fail()?;
            self.branches += 1;
            Ok(())
        }

        fn emit_branch_chain(&mut self, _plan: &BranchChain) -> ClassicalResult<()> {
            self.maybe_fail()?;
            self.chains += 1;
            Ok(())
        }

        fn emit_loop_plan(&mut self, _plan: &LoopPlan) -> ClassicalResult<()> {
            self.maybe_fail()?;
            self.loops += 1;
            Ok(())
        }

        fn emit_function_flow(
            &mut self,
            _plan: &FunctionControlFlowPlan,
        ) -> ClassicalResult<()> {
            self.maybe_fail()?;
            self.functions += 1;
            Ok(())
        }
    }

    fn value(value: u128) -> ValueId {
        ValueId::new(value).expect("test value ID must be nonzero")
    }

    fn block(value: u128) -> BlockId {
        BlockId::new(value).expect("test block ID must be nonzero")
    }

    fn symbol(value: u128) -> SymbolId {
        SymbolId::new(value).expect("test symbol ID must be nonzero")
    }

    fn function(value: u128) -> FunctionId {
        FunctionId::new(value).expect("test function ID must be nonzero")
    }

    #[test]
    fn branch_is_forwarded_and_work_is_charged() {
        let sink = RecordingSink::default();
        let mut lowerer = ControlFlowLowerer::new(
            sink,
            ControlFlowLoweringOptions::default(),
        );

        let plan = BranchPlan::new(value(1), block(2), Some(block(3)));
        lowerer.lower_branch(&plan).expect("branch lowering");

        assert_eq!(lowerer.charged_work(), 1);
        assert_eq!(lowerer.sink().branches, 1);
    }

    #[test]
    fn conditional_chain_preserves_all_arms_for_the_sink() {
        let sink = RecordingSink::default();
        let mut lowerer = ControlFlowLowerer::new(
            sink,
            ControlFlowLoweringOptions::default(),
        );

        let chain = BranchChain::new(
            vec![
                ConditionalArm::new(value(1), block(10)),
                ConditionalArm::new(value(2), block(20)),
                ConditionalArm::new(value(3), block(30)),
            ],
            Some(block(40)),
        )
        .expect("non-empty chain");

        lowerer
            .lower_branch_chain(&chain)
            .expect("chain lowering");

        assert_eq!(lowerer.charged_work(), 3);
        assert_eq!(lowerer.sink().chains, 1);
    }

    #[test]
    fn while_loop_is_forwarded() {
        let sink = RecordingSink::default();
        let mut lowerer = ControlFlowLowerer::new(
            sink,
            ControlFlowLoweringOptions::default(),
        );

        let plan = LoopPlan::while_loop(value(1), block(2), block(3), block(4));
        lowerer.lower_loop(&plan).expect("loop lowering");

        assert_eq!(lowerer.charged_work(), 1);
        assert_eq!(lowerer.sink().loops, 1);
    }

    #[test]
    fn for_loop_is_forwarded_without_assuming_iterable_storage() {
        let sink = RecordingSink::default();
        let mut lowerer = ControlFlowLowerer::new(
            sink,
            ControlFlowLoweringOptions::default(),
        );

        let plan = LoopPlan::for_loop(
            value(1),
            symbol(2),
            block(3),
            block(4),
            block(5),
        );

        lowerer.lower_loop(&plan).expect("loop lowering");

        assert_eq!(lowerer.sink().loops, 1);
    }

    #[test]
    fn function_flow_cost_accounts_for_return_blocks() {
        let sink = RecordingSink::default();
        let mut lowerer = ControlFlowLowerer::new(
            sink,
            ControlFlowLoweringOptions::default(),
        );

        let plan = FunctionControlFlowPlan::with_return_blocks(
            function(1),
            block(2),
            [block(3), block(4)],
        )
        .expect("unique return blocks");

        lowerer
            .lower_function_flow(&plan)
            .expect("function flow lowering");

        assert_eq!(lowerer.charged_work(), 3);
        assert_eq!(lowerer.sink().functions, 1);
    }

    #[test]
    fn budget_exhaustion_does_not_call_the_sink() {
        let sink = RecordingSink::default();
        let mut lowerer = ControlFlowLowerer::new(
            sink,
            ControlFlowLoweringOptions {
                work_budget: Some(0),
            },
        );

        let plan = BranchPlan::new(value(1), block(2), None);
        let result = lowerer.lower_branch(&plan);

        assert!(matches!(
            result,
            Err(ClassicalError::ResourceLimitExceeded { .. })
        ));
        assert_eq!(lowerer.charged_work(), 0);
        assert_eq!(lowerer.sink().branches, 0);
    }

    #[test]
    fn failed_sink_call_does_not_charge_a_budget_excessively() {
        let sink = RecordingSink {
            fail_next: true,
            ..RecordingSink::default()
        };
        let mut lowerer = ControlFlowLowerer::new(
            sink,
            ControlFlowLoweringOptions {
                work_budget: Some(1),
            },
        );

        let plan = BranchPlan::new(value(1), block(2), None);
        assert!(lowerer.lower_branch(&plan).is_err());

        // Attempted work is charged even when emission fails.
        assert_eq!(lowerer.charged_work(), 1);
        assert_eq!(lowerer.sink().branches, 0);
    }

    #[test]
    fn repeated_calls_obey_the_same_session_budget() {
        let sink = RecordingSink::default();
        let mut lowerer = ControlFlowLowerer::new(
            sink,
            ControlFlowLoweringOptions {
                work_budget: Some(1),
            },
        );

        let first = BranchPlan::new(value(1), block(2), None);
        let second = BranchPlan::new(value(3), block(4), None);

        lowerer.lower_branch(&first).expect("first branch");
        assert!(lowerer.lower_branch(&second).is_err());

        assert_eq!(lowerer.charged_work(), 1);
        assert_eq!(lowerer.sink().branches, 1);
    }
}
