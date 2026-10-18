
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Liveness Analysis
//!
//! Canonical path: `src/classical/zcir/analysis/liveness.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Compute deterministic, conservative block-level live-in and live-out
//! value sets over a control-flow graph.
//!
//! ## Canonical ownership
//!
//! - `crate::zuir::ids::{BlockId, ValueId}` owns canonical identities.
//! - Canonical ZUIR blocks, operations, operands and terminators own IR data.
//! - `src/classical/zcir/analysis/use_def.rs` integrates definition-use facts.
//! - `src/classical/zcir/analysis/control_flow.rs` may provide CFG analysis.
//! - The optimizer owns transformations and must independently check legality.
//!
//! This module owns only the liveness problem's input facts and computed
//! data-flow result. It does not own or duplicate canonical IR nodes.
//!
//! ## Data-flow equations
//!
//! For each block B:
//!
//!     OUT[B] = union(IN[S]) for every successor S of B
//!     IN[B]  = USE[B] union (OUT[B] - DEF[B])
//!
//! `USE[B]` must contain values used before their first definition in B.
//! `DEF[B]` must contain values defined in B. An upstream operation scan
//! is responsible for constructing these sets with correct instruction
//! ordering semantics.
//!
//! ## Safety and soundness
//!
//! Liveness is a may-analysis: a value is live if it may be needed along
//! some reachable control-flow continuation represented by the graph.
//!
//! A value not found in a computed live set is not, by itself, permission
//! to remove its defining operation. Operations may have observable effects,
//! traps, synchronization behavior, resource contracts or other constraints.
//! Transformations must preserve all applicable semantic contracts.
//!
//! ## Scalability
//!
//! No universal maximum block count or value count is imposed. A deterministic
//! worklist revisits predecessors when a block's live-in set changes.
//! Memory and execution remain bounded by available resources and the size
//! of the supplied graph and fact sets.
//!
//! Ordered maps and sets provide deterministic iteration, at the cost of
//! logarithmic collection operations. This is not a promise of infinite
//! physical capacity.
//!
//! ## Mutation and invalidation
//!
//! A `LivenessAnalysis` is a snapshot of one `LivenessProblem`. Recompute it
//! after changing block uses, definitions, or control-flow edges.
//! This module does not automatically observe IR mutation.
//!
//! ## Integration
//!
//! - Build block facts from canonical operations and terminators.
//! - Use canonical IDs; do not invent local ID types.
//! - Supply successors from the verified CFG or equivalent trusted source.
//! - Run structural verification before treating input facts as valid IR facts.
//! - Register `pub mod liveness;` in `analysis/mod.rs` when the ZCIR module
//!   hierarchy is valid Rust.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use crate::zuir::ids::{BlockId, ValueId};

/// Result type for liveness operations.
pub type LivenessResult<T> = Result<T, LivenessError>;

/// Errors produced while building or solving a liveness problem.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LivenessError {
    /// A block was registered more than once.
    DuplicateBlock {
        block: BlockId,
    },

    /// A fact was added for a block that has not been registered.
    UnknownBlock {
        block: BlockId,
    },

    /// An edge refers to a destination block that was not registered.
    UnknownSuccessor {
        source: BlockId,
        destination: BlockId,
    },
}

impl fmt::Display for LivenessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateBlock { block } => {
                write!(f, "liveness problem contains duplicate block {block}")
            }
            Self::UnknownBlock { block } => {
                write!(f, "liveness fact references unknown block {block}")
            }
            Self::UnknownSuccessor {
                source,
                destination,
            } => {
                write!(
                    f,
                    "control-flow edge from block {source} references \
                     unknown successor {destination}"
                )
            }
        }
    }
}

impl std::error::Error for LivenessError {}

/// Input facts for one block.
///
/// `uses` contains values read before being defined locally. `definitions`
/// contains values defined in the block. `successors` contains outgoing CFG
/// edges. All collections are ordered to make processing deterministic.
#[derive(Debug, Clone, Default)]
struct BlockFacts {
    uses: BTreeSet<ValueId>,
    definitions: BTreeSet<ValueId>,
    successors: BTreeSet<BlockId>,
}

/// Input to the liveness solver.
///
/// The problem contains block-level facts, not a second copy of the IR.
/// Instruction-order-sensitive use/definition extraction is the responsibility
/// of the canonical operation scan that populates this structure.
#[derive(Debug, Clone, Default)]
pub struct LivenessProblem {
    blocks: BTreeMap<BlockId, BlockFacts>,
}

impl LivenessProblem {
    /// Create an empty liveness problem.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a block.
    ///
    /// Duplicate registration is an error so an accidental CFG construction
    /// bug cannot silently replace previously collected facts.
    pub fn add_block(&mut self, block: BlockId) -> LivenessResult<()> {
        if self.blocks.contains_key(&block) {
            return Err(LivenessError::DuplicateBlock { block });
        }

        self.blocks.insert(block, BlockFacts::default());
        Ok(())
    }

    /// Record a value used before its first definition in this block.
    ///
    /// The caller must account for instruction order when extracting facts.
    /// If a value is defined earlier in the same block, it should not be
    /// recorded as a block-level upward-exposed use solely because it is
    /// used later in that block.
    pub fn add_use(
        &mut self,
        block: BlockId,
        value: ValueId,
    ) -> LivenessResult<()> {
        let facts = self
            .blocks
            .get_mut(&block)
            .ok_or(LivenessError::UnknownBlock { block })?;

        facts.uses.insert(value);
        Ok(())
    }

    /// Record a value defined in this block.
    ///
    /// A definition does not automatically imply that the value is live.
    pub fn add_definition(
        &mut self,
        block: BlockId,
        value: ValueId,
    ) -> LivenessResult<()> {
        let facts = self
            .blocks
            .get_mut(&block)
            .ok_or(LivenessError::UnknownBlock { block })?;

        facts.definitions.insert(value);
        Ok(())
    }

    /// Record a directed control-flow edge.
    ///
    /// The source must already be registered. The destination is validated
    /// when `compute` is called, so blocks and edges can be supplied in
    /// different orders.
    ///
    /// Duplicate edges are idempotent for block-level liveness.
    pub fn add_successor(
        &mut self,
        source: BlockId,
        destination: BlockId,
    ) -> LivenessResult<()> {
        let facts = self
            .blocks
            .get_mut(&source)
            .ok_or(LivenessError::UnknownBlock { block: source })?;

        facts.successors.insert(destination);
        Ok(())
    }

    /// Number of registered blocks.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Whether no blocks have been registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Compute a fixed-point liveness solution.
    ///
    /// The algorithm is iterative and does not recurse through the CFG.
    /// Initially every live set is empty. Blocks are processed through a
    /// deterministic worklist, and predecessors are re-enqueued whenever
    /// a block's live-in set changes.
    ///
    /// An empty graph is valid and produces an empty analysis.
    pub fn compute(&self) -> LivenessResult<LivenessAnalysis> {
        let mut predecessors: BTreeMap<BlockId, BTreeSet<BlockId>> =
            self.blocks
                .keys()
                .copied()
                .map(|block| (block, BTreeSet::new()))
                .collect();

        // Validate all CFG endpoints before solving. A malformed edge must
        // never be silently interpreted as a terminal path.
        for (&source, facts) in &self.blocks {
            for &destination in &facts.successors {
                let Some(destination_predecessors) =
                    predecessors.get_mut(&destination)
                else {
                    return Err(LivenessError::UnknownSuccessor {
                        source,
                        destination,
                    });
                };

                destination_predecessors.insert(source);
            }
        }

        let mut live_in: BTreeMap<BlockId, BTreeSet<ValueId>> = self
            .blocks
            .keys()
            .copied()
            .map(|block| (block, BTreeSet::new()))
            .collect();

        let mut live_out = live_in.clone();

        // Reverse block order is a deterministic initial ordering. The
        // worklist algorithm remains correct regardless of this ordering.
        let mut worklist: VecDeque<BlockId> =
            self.blocks.keys().rev().copied().collect();

        let mut queued: BTreeSet<BlockId> =
            self.blocks.keys().copied().collect();

        while let Some(block) = worklist.pop_front() {
            queued.remove(&block);

            let facts = self
                .blocks
                .get(&block)
                .expect("worklist contains only registered blocks");

            let mut new_out = BTreeSet::new();

            for successor in &facts.successors {
                let successor_live_in = live_in
                    .get(successor)
                    .expect("CFG endpoints were validated before solving");

                new_out.extend(successor_live_in.iter().copied());
            }

            let mut new_in = facts.uses.clone();

            for value in &new_out {
                if !facts.definitions.contains(value) {
                    new_in.insert(*value);
                }
            }

            let out_changed = live_out.get(&block) != Some(&new_out);
            let in_changed = live_in.get(&block) != Some(&new_in);

            if out_changed {
                live_out.insert(block, new_out);
            }

            if in_changed {
                live_in.insert(block, new_in);

                // Only predecessors can be affected by a changed live-in set.
                // The queued set prevents redundant pending work items.
                for predecessor in predecessors
                    .get(&block)
                    .expect("every registered block has a predecessor set")
                {
                    if queued.insert(*predecessor) {
                        worklist.push_back(*predecessor);
                    }
                }
            }
        }

        Ok(LivenessAnalysis {
            live_in,
            live_out,
            predecessors,
        })
    }
}

/// Immutable result of liveness analysis.
///
/// Each registered block has a live-in and live-out set, including blocks
/// unreachable from a function entry. Unreachable blocks are still analyzed
/// locally; reachability policy belongs to CFG analysis and verification.
#[derive(Debug, Clone, Default)]
pub struct LivenessAnalysis {
    live_in: BTreeMap<BlockId, BTreeSet<ValueId>>,
    live_out: BTreeMap<BlockId, BTreeSet<ValueId>>,
    predecessors: BTreeMap<BlockId, BTreeSet<BlockId>>,
}

impl LivenessAnalysis {
    /// Obtain the live-in set for a block.
    ///
    /// Returns `None` if the block was not part of the analyzed problem.
    #[must_use]
    pub fn live_in(
        &self,
        block: BlockId,
    ) -> Option<&BTreeSet<ValueId>> {
        self.live_in.get(&block)
    }

    /// Obtain the live-out set for a block.
    ///
    /// Returns `None` if the block was not part of the analyzed problem.
    #[must_use]
    pub fn live_out(
        &self,
        block: BlockId,
    ) -> Option<&BTreeSet<ValueId>> {
        self.live_out.get(&block)
    }

    /// Obtain the predecessors recorded for a block.
    ///
    /// Returns `None` if the block was not part of the analyzed problem.
    #[must_use]
    pub fn predecessors(
        &self,
        block: BlockId,
    ) -> Option<&BTreeSet<BlockId>> {
        self.predecessors.get(&block)
    }

    /// Whether a value is live on entry to a block.
    ///
    /// Returns `None` for an unknown block.
    #[must_use]
    pub fn is_live_in(
        &self,
        block: BlockId,
        value: ValueId,
    ) -> Option<bool> {
        self.live_in(block).map(|values| values.contains(&value))
    }

    /// Whether a value is live on exit from a block.
    ///
    /// Returns `None` for an unknown block.
    #[must_use]
    pub fn is_live_out(
        &self,
        block: BlockId,
        value: ValueId,
    ) -> Option<bool> {
        self.live_out(block).map(|values| values.contains(&value))
    }

    /// Number of blocks represented in this result.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.live_in.len()
    }

    /// Whether the result contains no blocks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.live_in.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test block ID must be nonzero")
    }

    fn value(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    #[test]
    fn empty_problem_produces_empty_analysis() {
        let result = LivenessProblem::new().compute().unwrap();

        assert!(result.is_empty());
        assert_eq!(result.block_count(), 0);
    }

    #[test]
    fn duplicate_block_is_rejected() {
        let mut problem = LivenessProblem::new();
        let entry = block(1);

        problem.add_block(entry).unwrap();

        assert_eq!(
            problem.add_block(entry),
            Err(LivenessError::DuplicateBlock { block: entry })
        );
    }

    #[test]
    fn facts_for_unknown_blocks_are_rejected() {
        let mut problem = LivenessProblem::new();
        let missing = block(7);

        assert_eq!(
            problem.add_use(missing, value(1)),
            Err(LivenessError::UnknownBlock { block: missing })
        );

        assert_eq!(
            problem.add_definition(missing, value(1)),
            Err(LivenessError::UnknownBlock { block: missing })
        );

        assert_eq!(
            problem.add_successor(missing, block(8)),
            Err(LivenessError::UnknownBlock { block: missing })
        );
    }

    #[test]
    fn unknown_successor_is_rejected_during_compute() {
        let entry = block(1);
        let missing = block(2);
        let mut problem = LivenessProblem::new();

        problem.add_block(entry).unwrap();
        problem.add_successor(entry, missing).unwrap();

        assert_eq!(
            problem.compute(),
            Err(LivenessError::UnknownSuccessor {
                source: entry,
                destination: missing,
            })
        );
    }

    #[test]
    fn local_use_is_live_in() {
        let entry = block(1);
        let input = value(10);
        let mut problem = LivenessProblem::new();

        problem.add_block(entry).unwrap();
        problem.add_use(entry, input).unwrap();

        let result = problem.compute().unwrap();

        assert_eq!(result.live_in(entry), Some(&BTreeSet::from([input])));
        assert_eq!(result.live_out(entry), Some(&BTreeSet::new()));
    }

    #[test]
    fn definition_kills_liveness_across_the_block() {
        let entry = block(1);
        let result_block = block(2);
        let input = value(10);
        let mut problem = LivenessProblem::new();

        problem.add_block(entry).unwrap();
        problem.add_block(result_block).unwrap();
        problem.add_successor(entry, result_block).unwrap();

        // The value is defined in entry, so its use in the successor does
        // not make it live on entry to the defining block.
        problem.add_definition(entry, input).unwrap();
        problem.add_use(result_block, input).unwrap();

        let result = problem.compute().unwrap();

        assert_eq!(result.live_in(entry), Some(&BTreeSet::new()));
        assert_eq!(result.live_out(entry), Some(&BTreeSet::from([input])));
        assert_eq!(result.live_in(result_block), Some(&BTreeSet::from([input])));
    }

    #[test]
    fn a_value_used_in_a_successor_propagates_backward() {
        let first = block(1);
        let second = block(2);
        let third = block(3);
        let input = value(10);
        let mut problem = LivenessProblem::new();

        problem.add_block(first).unwrap();
        problem.add_block(second).unwrap();
        problem.add_block(third).unwrap();

        problem.add_successor(first, second).unwrap();
        problem.add_successor(second, third).unwrap();
        problem.add_use(third, input).unwrap();

        let result = problem.compute().unwrap();

        assert!(result.is_live_in(first, input).unwrap());
        assert!(result.is_live_out(first, input).unwrap());
        assert!(result.is_live_in(second, input).unwrap());
        assert!(result.is_live_out(second, input).unwrap());
        assert!(result.is_live_in(third, input).unwrap());
    }

    #[test]
    fn branch_merges_liveness_from_all_successors() {
        let branch = block(1);
        let left = block(2);
        let right = block(3);
        let left_value = value(10);
        let right_value = value(20);
        let mut problem = LivenessProblem::new();

        problem.add_block(branch).unwrap();
        problem.add_block(left).unwrap();
        problem.add_block(right).unwrap();

        problem.add_successor(branch, left).unwrap();
        problem.add_successor(branch, right).unwrap();

        problem.add_use(left, left_value).unwrap();
        problem.add_use(right, right_value).unwrap();

        let result = problem.compute().unwrap();
        let expected = BTreeSet::from([left_value, right_value]);

        assert_eq!(result.live_out(branch), Some(&expected));
        assert_eq!(result.live_in(branch), Some(&expected));
    }

    #[test]
    fn loops_reach_a_fixed_point() {
        let first = block(1);
        let second = block(2);
        let third = block(3);
        let loop_value = value(10);
        let mut problem = LivenessProblem::new();

        problem.add_block(first).unwrap();
        problem.add_block(second).unwrap();
        problem.add_block(third).unwrap();

        problem.add_successor(first, second).unwrap();
        problem.add_successor(second, third).unwrap();
        problem.add_successor(third, second).unwrap();

        problem.add_use(third, loop_value).unwrap();

        let result = problem.compute().unwrap();

        assert!(result.is_live_in(first, loop_value).unwrap());
        assert!(result.is_live_in(second, loop_value).unwrap());
        assert!(result.is_live_out(second, loop_value).unwrap());
        assert!(result.is_live_in(third, loop_value).unwrap());
    }

    #[test]
    fn self_loop_is_supported() {
        let loop_block = block(1);
        let loop_value = value(10);
        let mut problem = LivenessProblem::new();

        problem.add_block(loop_block).unwrap();
        problem.add_successor(loop_block, loop_block).unwrap();
        problem.add_use(loop_block, loop_value).unwrap();

        let result = problem.compute().unwrap();

        assert!(result.is_live_in(loop_block, loop_value).unwrap());
        assert!(result.is_live_out(loop_block, loop_value).unwrap());
    }

    #[test]
    fn unreachable_blocks_are_analyzed_locally() {
        let entry = block(1);
        let unreachable = block(2);
        let local_value = value(10);
        let mut problem = LivenessProblem::new();

        problem.add_block(entry).unwrap();
        problem.add_block(unreachable).unwrap();
        problem.add_use(unreachable, local_value).unwrap();

        let result = problem.compute().unwrap();

        assert_eq!(result.live_in(entry), Some(&BTreeSet::new()));
        assert!(result.is_live_in(unreachable, local_value).unwrap());
    }

    #[test]
    fn repeated_edges_do_not_change_the_result() {
        let first = block(1);
        let second = block(2);
        let input = value(10);
        let mut problem = LivenessProblem::new();

        problem.add_block(first).unwrap();
        problem.add_block(second).unwrap();

        problem.add_successor(first, second).unwrap();
        problem.add_successor(first, second).unwrap();
        problem.add_use(second, input).unwrap();

        let result = problem.compute().unwrap();

        assert!(result.is_live_out(first, input).unwrap());
        assert_eq!(
            result.predecessors(second),
            Some(&BTreeSet::from([first]))
        );
    }

    #[test]
    fn definition_and_use_sets_are_value_sensitive() {
        let entry = block(1);
        let defined = value(10);
        let used = value(20);
        let mut problem = LivenessProblem::new();

        problem.add_block(entry).unwrap();
        problem.add_definition(entry, defined).unwrap();
        problem.add_use(entry, used).unwrap();

        let result = problem.compute().unwrap();

        assert_eq!(
            result.live_in(entry),
            Some(&BTreeSet::from([used]))
        );
        assert_eq!(result.live_out(entry), Some(&BTreeSet::new()));
    }

    #[test]
    fn insertion_order_does_not_change_the_solution() {
        let a = block(1);
        let b = block(2);
        let c = block(3);
        let x = value(10);
        let y = value(20);

        let mut first = LivenessProblem::new();
        first.add_block(a).unwrap();
        first.add_block(b).unwrap();
        first.add_block(c).unwrap();
        first.add_successor(a, b).unwrap();
        first.add_successor(a, c).unwrap();
        first.add_use(b, x).unwrap();
        first.add_use(c, y).unwrap();

        let mut second = LivenessProblem::new();
        second.add_block(c).unwrap();
        second.add_block(b).unwrap();
        second.add_block(a).unwrap();
        second.add_use(c, y).unwrap();
        second.add_use(b, x).unwrap();
        second.add_successor(a, c).unwrap();
        second.add_successor(a, b).unwrap();

        let first_result = first.compute().unwrap();
        let second_result = second.compute().unwrap();

        for block in [a, b, c] {
            assert_eq!(
                first_result.live_in(block),
                second_result.live_in(block)
            );
            assert_eq!(
                first_result.live_out(block),
                second_result.live_out(block)
            );
        }
    }
}
