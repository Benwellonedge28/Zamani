
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Liveness Analysis
//!
//! Path: `src/zuir/analysis/liveness.rs`
//! Rust edition: 2021
//! Minimum intended Rust version: 1.97.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Compute deterministic, region-local value liveness over the canonical
//! ZUIR control-flow graph.
//!
//! ## Ownership
//!
//! This module owns only derived liveness facts:
//! - block live-in sets;
//! - block live-out sets;
//! - block-local upward-exposed uses and definitions;
//! - the fixed-point solver and its result/error API.
//!
//! It does not own IR entities, identifiers, CFG construction, def-use
//! indexing, dominance, ownership verification, effect verification,
//! quantum resource allocation, or target-specific register allocation.
//!
//! ## Authority and integration
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/operand.rs`
//! - `src/zuir/ir/result.rs`
//! - `src/zuir/analysis/cfg.rs`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! This analysis consumes already-lowered ZUIR. It must not parse grammar
//! constructs, infer source semantics, or introduce a competing quantum IR.
//!
//! ## Semantics
//!
//! A value is live-in to a block if its value may be used along some
//! execution path beginning in that block before being redefined in that
//! path. A value is live-out if it is needed after the block along at least
//! one outgoing control-flow edge.
//!
//! For each block B:
//!
//!   live_in[B]  = use[B] union (live_out[B] - def[B])
//!
//!   live_out[B] = union over edges B -> S of:
//!       (live_in[S] - block_arguments[S]) union edge_arguments[B -> S]
//!
//! Block arguments are definitions in the destination block. The values
//! supplied to those arguments are uses on the corresponding predecessor
//! edge. This distinction is essential for SSA-style IR.
//!
//! ## Scope and trust boundary
//!
//! The analysis covers every block declared in the supplied region, including
//! unreachable blocks. It requires a control-flow region accepted by the
//! canonical CFG builder. A supplied operation collection must include every
//! operation referenced by those blocks.
//!
//! Successful analysis does not prove that the whole program is valid.
//! Run the authoritative verifier before trusting IR for optimization,
//! lowering, code generation, or execution.
//!
//! ## Determinism and invalidation
//!
//! Results use ordered maps and sets. Worklist order follows reverse region
//! block order. The fixed-point result is independent of the order in which
//! equivalent set members are inserted.
//!
//! This is an immutable snapshot. Recompute after changing operations,
//! operands, results, block arguments, terminators, successor edges, or region
//! membership. The pass manager owns analysis invalidation.
//!
//! ## Scalability
//!
//! No fixed limit is imposed on blocks, operations, values, successors,
//! qubits, processors, devices, tensor dimensions, or network size.
//! The solver is iterative, not recursively traversed. Runtime and memory
//! requirements grow with the input graph and live-value sets.
//!
//! Ordered standard-library collections do not provide a stable, generally
//! fallible reservation API. Therefore allocator exhaustion cannot be
//! universally recovered from. Actual scale is bounded by available
//! resources; literal infinity on finite hardware is not promised.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::analysis::cfg::{CfgError, ControlFlowGraph};
use crate::zuir::ids::{BlockId, OperationId, ValueId};
use crate::zuir::ir::block::{BlockTerminator, ZuirBlock};
use crate::zuir::ir::operation::ZuirOperation;
use crate::zuir::ir::region::ZuirRegion;

/// Result type for liveness analysis.
pub type LivenessResult<T> = Result<T, LivenessError>;

/// Errors detected while constructing or solving liveness information.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LivenessError {
    /// The canonical CFG could not be built.
    ControlFlow(CfgError),

    /// The supplied block collection contains duplicate identities.
    DuplicateBlock(BlockId),

    /// The supplied operation collection contains duplicate identities.
    DuplicateOperation(OperationId),

    /// A block refers to an operation absent from the supplied collection.
    MissingOperation {
        block: BlockId,
        operation: OperationId,
    },

    /// The block's local structure could not be analyzed safely.
    InvalidBlock {
        block: BlockId,
        reason: String,
    },
}

impl LivenessError {
    /// Return a stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::ControlFlow(_) => "ZUIR-LIVENESS-CFG",
            Self::DuplicateBlock(_) => "ZUIR-LIVENESS-DUPLICATE-BLOCK",
            Self::DuplicateOperation(_) => "ZUIR-LIVENESS-DUPLICATE-OPERATION",
            Self::MissingOperation { .. } => "ZUIR-LIVENESS-MISSING-OPERATION",
            Self::InvalidBlock { .. } => "ZUIR-LIVENESS-INVALID-BLOCK",
        }
    }
}

impl fmt::Display for LivenessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ControlFlow(error) => {
                write!(f, "cannot analyze liveness: {error}")
            }
            Self::DuplicateBlock(block) => {
                write!(f, "duplicate block identity {block}")
            }
            Self::DuplicateOperation(operation) => {
                write!(f, "duplicate operation identity {operation}")
            }
            Self::MissingOperation { block, operation } => write!(
                f,
                "block {block} references missing operation {operation}"
            ),
            Self::InvalidBlock { block, reason } => {
                write!(f, "cannot analyze block {block}: {reason}")
            }
        }
    }
}

impl std::error::Error for LivenessError {}

impl From<CfgError> for LivenessError {
    fn from(error: CfgError) -> Self {
        Self::ControlFlow(error)
    }
}

/// Liveness facts for one block.
///
/// The sets contain canonical `ValueId`s. They are ordered for deterministic
/// inspection and stable tests.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BlockLiveness {
    live_in: BTreeSet<ValueId>,
    live_out: BTreeSet<ValueId>,
    upward_exposed_uses: BTreeSet<ValueId>,
    definitions: BTreeSet<ValueId>,
}

impl BlockLiveness {
    /// Values needed on entry to this block.
    #[must_use]
    pub fn live_in(&self) -> &BTreeSet<ValueId> {
        &self.live_in
    }

    /// Values needed after this block along at least one outgoing edge.
    #[must_use]
    pub fn live_out(&self) -> &BTreeSet<ValueId> {
        &self.live_out
    }

    /// Values used before any definition in this block.
    #[must_use]
    pub fn upward_exposed_uses(&self) -> &BTreeSet<ValueId> {
        &self.upward_exposed_uses
    }

    /// Values defined by block arguments or operation results in this block.
    #[must_use]
    pub fn definitions(&self) -> &BTreeSet<ValueId> {
        &self.definitions
    }

    /// Whether the value is live on block entry.
    #[must_use]
    pub fn is_live_in(&self, value: ValueId) -> bool {
        self.live_in.contains(&value)
    }

    /// Whether the value is live after this block.
    #[must_use]
    pub fn is_live_out(&self, value: ValueId) -> bool {
        self.live_out.contains(&value)
    }
}

/// Immutable liveness snapshot for a single control-flow region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivenessAnalysis {
    region: crate::zuir::ids::RegionId,
    blocks: BTreeMap<BlockId, BlockLiveness>,
    iterations: usize,
}

impl LivenessAnalysis {
    /// Return the analyzed region identity.
    #[must_use]
    pub const fn region(&self) -> crate::zuir::ids::RegionId {
        self.region
    }

    /// Return all block facts in deterministic block-ID order.
    #[must_use]
    pub fn blocks(&self) -> &BTreeMap<BlockId, BlockLiveness> {
        &self.blocks
    }

    /// Return the facts for a block, if that block belongs to this snapshot.
    #[must_use]
    pub fn block(&self, block: BlockId) -> Option<&BlockLiveness> {
        self.blocks.get(&block)
    }

    /// Return the number of fixed-point sweeps performed.
    ///
    /// This is informational, not a semantic property or an execution limit.
    #[must_use]
    pub const fn iterations(&self) -> usize {
        self.iterations
    }

    /// Return the live-in set for a block.
    #[must_use]
    pub fn live_in(&self, block: BlockId) -> Option<&BTreeSet<ValueId>> {
        self.block(block).map(BlockLiveness::live_in)
    }

    /// Return the live-out set for a block.
    #[must_use]
    pub fn live_out(&self, block: BlockId) -> Option<&BTreeSet<ValueId>> {
        self.block(block).map(BlockLiveness::live_out)
    }

    /// Return whether a value is live at a block's entry.
    #[must_use]
    pub fn is_live_in(&self, block: BlockId, value: ValueId) -> bool {
        self.block(block)
            .is_some_and(|facts| facts.is_live_in(value))
    }

    /// Return whether a value is live after a block.
    #[must_use]
    pub fn is_live_out(&self, block: BlockId, value: ValueId) -> bool {
        self.block(block)
            .is_some_and(|facts| facts.is_live_out(value))
    }

    /// Return the number of distinct values live at block entry.
    ///
    /// `None` means the block is not part of this snapshot.
    #[must_use]
    pub fn live_in_count(&self, block: BlockId) -> Option<usize> {
        self.live_in(block).map(BTreeSet::len)
    }

    /// Return the number of distinct values live after a block.
    ///
    /// `None` means the block is not part of this snapshot.
    #[must_use]
    pub fn live_out_count(&self, block: BlockId) -> Option<usize> {
        self.live_out(block).map(BTreeSet::len)
    }
}

/// Analyze liveness for a control-flow region.
///
/// `region` supplies ordered block membership. `blocks` supplies the canonical
/// block objects for that region and any other regions in the same IR scope.
/// `operations` supplies the operation objects referenced by those blocks.
///
/// Every operation referenced by a block in `region` must appear in
/// `operations`. Duplicate block or operation identities are rejected rather
/// than silently overwritten.
///
/// `entry` is optional because this analysis computes facts for every block,
/// including unreachable blocks. If the caller needs entry reachability for
/// another purpose, use the CFG API directly.
///
/// # Errors
///
/// Returns a structured error if the CFG is invalid, block/operation
/// identities are ambiguous, or a referenced operation is missing.
pub fn analyze(
    region: &ZuirRegion,
    blocks: &[ZuirBlock],
    operations: &[ZuirOperation],
) -> LivenessResult<LivenessAnalysis> {
    analyze_with_entry(region, blocks, operations, None)
}

/// Analyze liveness while also validating an explicitly supplied CFG entry.
///
/// Liveness itself does not require reachability from an entry. Supplying
/// `Some(entry)` asks the canonical CFG builder to validate that entry.
pub fn analyze_with_entry(
    region: &ZuirRegion,
    blocks: &[ZuirBlock],
    operations: &[ZuirOperation],
    entry: Option<BlockId>,
) -> LivenessResult<LivenessAnalysis> {
    let mut blocks_by_id = BTreeMap::<BlockId, &ZuirBlock>::new();

    for block in blocks {
        if blocks_by_id.insert(block.id(), block).is_some() {
            return Err(LivenessError::DuplicateBlock(block.id()));
        }
    }

    let mut operations_by_id =
        BTreeMap::<OperationId, &ZuirOperation>::new();

    for operation in operations {
        if operations_by_id
            .insert(operation.id(), operation)
            .is_some()
        {
            return Err(LivenessError::DuplicateOperation(operation.id()));
        }
    }

    // The canonical CFG owns terminator interpretation and edge validation.
    let cfg = ControlFlowGraph::build(region, blocks, entry)?;

    // Keep only blocks that belong to this region. Extra blocks in the caller's
    // storage belong to other regions and must not contaminate this analysis.
    let mut region_blocks = BTreeMap::<BlockId, &ZuirBlock>::new();

    for block_id in region.blocks() {
        let block = blocks_by_id.get(block_id).copied().ok_or_else(|| {
            LivenessError::InvalidBlock {
                block: *block_id,
                reason: "region references a block absent from block storage"
                    .to_owned(),
            }
        })?;

        region_blocks.insert(*block_id, block);
    }

    // Compute the block-local use and definition sets.
    let mut facts = BTreeMap::<BlockId, BlockLiveness>::new();

    for (block_id, block) in &region_blocks {
        let mut local = BlockLiveness::default();

        // Block arguments are definitions at the beginning of the block.
        for argument in block.arguments() {
            local.definitions.insert(argument.value());
        }

        // Operation operands are uses. Operation results are definitions.
        for operation_id in block.operations() {
            let operation = operations_by_id
                .get(operation_id)
                .copied()
                .ok_or(LivenessError::MissingOperation {
                    block: *block_id,
                    operation: *operation_id,
                })?;

            for operand in operation.operands() {
                let value = operand.value();

                if !local.definitions.contains(&value) {
                    local.upward_exposed_uses.insert(value);
                }
            }

            for result in operation.results() {
                local.definitions.insert(result.value());
            }
        }

        // Explicit terminator operands are uses at the end of the block.
        // Successor-edge arguments are processed separately below.
        match block.terminator() {
            BlockTerminator::Open => {
                return Err(LivenessError::InvalidBlock {
                    block: *block_id,
                    reason: "liveness requires a finalized block terminator"
                        .to_owned(),
                });
            }
            BlockTerminator::Return { values } => {
                for value in values {
                    if !local.definitions.contains(value) {
                        local.upward_exposed_uses.insert(*value);
                    }
                }
            }
            BlockTerminator::ConditionalBranch { condition, .. } => {
                if !local.definitions.contains(condition) {
                    local.upward_exposed_uses.insert(*condition);
                }
            }
            BlockTerminator::Switch { selector, .. } => {
                if !local.definitions.contains(selector) {
                    local.upward_exposed_uses.insert(*selector);
                }
            }
            BlockTerminator::Branch { .. }
            | BlockTerminator::Unreachable
            | BlockTerminator::Extension { .. } => {}
        }

        facts.insert(*block_id, local);
    }

    // Solve the backward data-flow equations until no set changes.
    //
    // Iteration is used instead of recursion, so long control-flow chains and
    // loops do not consume call-stack space. The equations are monotone over
    // finite sets of values in the supplied IR, so the iteration converges.
    let mut iterations = 0usize;
    let mut reverse_order = region.blocks().to_vec();
    reverse_order.reverse();

    loop {
        let mut changed = false;
        iterations = iterations.saturating_add(1);

        for block_id in &reverse_order {
            let block_facts = facts.get(block_id).ok_or_else(|| {
                LivenessError::InvalidBlock {
                    block: *block_id,
                    reason: "missing local liveness facts".to_owned(),
                }
            })?;

            let mut new_live_out = BTreeSet::<ValueId>::new();

            for edge in cfg.successors(*block_id) {
                let successor_facts =
                    facts.get(&edge.target()).ok_or_else(|| {
                        LivenessError::InvalidBlock {
                            block: edge.target(),
                            reason: "successor is missing liveness facts"
                                .to_owned(),
                        }
                    })?;

                // Destination block arguments are local definitions. They
                // must not propagate as if they were the same SSA values in
                // the predecessor. Instead, propagate the edge arguments.
                for value in successor_facts.live_in() {
                    if !region_blocks
                        .get(&edge.target())
                        .is_some_and(|target| {
                            target
                                .arguments()
                                .iter()
                                .any(|argument| argument.value() == *value)
                        })
                    {
                        new_live_out.insert(*value);
                    }
                }

                for value in edge.arguments() {
                    new_live_out.insert(*value);
                }
            }

            let mut new_live_in = block_facts.upward_exposed_uses.clone();

            for value in &new_live_out {
                if !block_facts.definitions.contains(value) {
                    new_live_in.insert(*value);
                }
            }

            let current = facts.get_mut(block_id).ok_or_else(|| {
                LivenessError::InvalidBlock {
                    block: *block_id,
                    reason: "missing mutable liveness facts".to_owned(),
                }
            })?;

            if current.live_in != new_live_in
                || current.live_out != new_live_out
            {
                current.live_in = new_live_in;
                current.live_out = new_live_out;
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    Ok(LivenessAnalysis {
        region: region.id(),
        blocks: facts,
        iterations,
    })
}

/// Return the union of two ordered value sets.
///
/// This helper intentionally does not impose a capacity or domain-specific
/// limit. Allocation behavior follows the standard collection implementation.
#[must_use]
pub fn union_values(
    left: &BTreeSet<ValueId>,
    right: &BTreeSet<ValueId>,
) -> BTreeSet<ValueId> {
    left.union(right).copied().collect()
}

/// Return `values` minus the definitions in `definitions`.
#[must_use]
pub fn subtract_definitions(
    values: &BTreeSet<ValueId>,
    definitions: &BTreeSet<ValueId>,
) -> BTreeSet<ValueId> {
    values.difference(definitions).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ids::ValueId;

    fn value(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    #[test]
    fn union_is_deterministic_and_deduplicates_values() {
        let left = [value(1), value(2)].into_iter().collect();
        let right = [value(2), value(3)].into_iter().collect();

        let result = union_values(&left, &right);

        assert_eq!(
            result.into_iter().collect::<Vec<_>>(),
            vec![value(1), value(2), value(3)]
        );
    }

    #[test]
    fn subtract_definitions_removes_only_defined_values() {
        let values = [value(1), value(2), value(3)]
            .into_iter()
            .collect();
        let definitions = [value(2)].into_iter().collect();

        let result = subtract_definitions(&values, &definitions);

        assert_eq!(
            result.into_iter().collect::<Vec<_>>(),
            vec![value(1), value(3)]
        );
    }

    #[test]
    fn default_block_facts_are_empty() {
        let facts = BlockLiveness::default();

        assert!(facts.live_in().is_empty());
        assert!(facts.live_out().is_empty());
        assert!(facts.upward_exposed_uses().is_empty());
        assert!(facts.definitions().is_empty());
        assert!(!facts.is_live_in(value(1)));
        assert!(!facts.is_live_out(value(1)));
    }

    #[test]
    fn diagnostic_codes_are_stable() {
        assert_eq!(
            LivenessError::DuplicateBlock(
                crate::zuir::ids::BlockId::new(1)
                    .expect("test block ID must be nonzero")
            )
            .code(),
            "ZUIR-LIVENESS-DUPLICATE-BLOCK"
        );
    }
}
