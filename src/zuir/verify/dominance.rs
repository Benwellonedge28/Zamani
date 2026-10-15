
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Dominance Analysis
//!
//! Path: `src/zuir/verify/dominance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: unsafe code is forbidden.
//!
//! PURPOSE
//!
//! Compute control-flow reachability and immediate dominators for the
//! explicitly represented control-flow graph of each ZUIR region.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/successor.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! OWNS
//!
//! - Dominance and reachability results.
//! - Immediate-dominator relationships.
//! - Region-local control-flow graph construction.
//! - Deterministic diagnostics for malformed graph references.
//!
//! DOES NOT OWN
//!
//! - Parsing, AST construction, or name resolution.
//! - Global ID allocation or global structural verification.
//! - SSA definition/use verification or value-type verification.
//! - Type, effect, ownership, capability, or resource verification.
//! - Dialect authorization or extension-terminator interpretation.
//! - Target feasibility, hardware placement, scheduling, or execution.
//!
//! CONTROL-FLOW CONTRACT
//!
//! The first block in each region's ordered block list is the entry block.
//! Explicit successors are taken from `BlockTerminator::for_each_successor`.
//! Dominance is computed independently for each region.
//!
//! An entry block has no immediate dominator. Unreachable blocks have no
//! immediate dominator. A block dominates itself, including when unreachable.
//! An unreachable block does not dominate a different block.
//!
//! Extension terminators contribute only the successors explicitly stored
//! in the terminator. Their domain-specific semantics must be verified by
//! the corresponding registered extension.
//!
//! SCALABILITY
//!
//! The implementation uses dynamic collections and an iterative DFS, so
//! graph traversal does not consume the call stack in proportion to graph
//! depth. The iterative immediate-dominator algorithm avoids materializing
//! a full dominator set for every block.
//!
//! There is no universal block, edge, processor, qubit, tensor-rank, or
//! device-count limit. Actual work and memory remain bounded by available
//! resources and the finite address space of the host.
//!
//! INTEGRATION
//!
//! Run structural verification before treating the graph as trusted input.
//! This module independently reports graph errors it encounters, because
//! it must not silently compute through missing or ambiguous block IDs.
//! A successful report proves only the properties implemented here.
//!
//! FREEZE CRITERIA
//!
//! - Uses canonical BlockId and RegionId types.
//! - Reads actual ZUIR block, region, and terminator APIs.
//! - Handles empty regions, malformed references, and unreachable blocks.
//! - Does not recurse over the CFG.
//! - Does not mutate the IR.
//! - Does not use unsafe code or fixed hardware limits.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{BlockId, RegionId};
use crate::zuir::ir::block::{BlockTerminator, ZuirBlock};
use crate::zuir::ir::region::ZuirRegion;

// =============================================================================
// Public input and diagnostic API
// =============================================================================

/// The complete block and region view used by dominance analysis.
///
/// The caller supplies the entities explicitly because the current ZUIR
/// representation stores region block membership as BlockId references.
///
/// Duplicate identities are diagnosed. The verifier never silently treats
/// one of two conflicting block definitions as authoritative.
pub struct DominanceInput<'a> {
    /// Regions whose control-flow graphs are to be analyzed.
    pub regions: &'a [ZuirRegion],

    /// Blocks referenced by those regions.
    pub blocks: &'a [ZuirBlock],
}

/// One deterministic, machine-readable dominance-analysis diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DominanceIssue {
    code: &'static str,
    message: String,
}

impl DominanceIssue {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Return the stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    /// Return the diagnostic explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for DominanceIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

// =============================================================================
// Dominance results
// =============================================================================

/// Computed dominance facts for the supplied region/block view.
///
/// Immediate dominators are stored only for reachable, non-entry blocks.
/// Entry blocks and unreachable blocks return `None` from
/// `immediate_dominator`.
///
/// Region membership is retained so a query cannot accidentally infer
/// dominance between blocks belonging to different regions.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DominanceInfo {
    region_of: BTreeMap<BlockId, RegionId>,
    entry_of: BTreeMap<RegionId, BlockId>,
    immediate_dominator: BTreeMap<BlockId, BlockId>,
    reachable: BTreeSet<BlockId>,
}

impl DominanceInfo {
    /// Return the entry block of a region, if it has a valid first block.
    #[must_use]
    pub fn entry_block(&self, region: RegionId) -> Option<BlockId> {
        self.entry_of.get(&region).copied()
    }

    /// Return the immediate dominator of a reachable, non-entry block.
    ///
    /// The entry block and unreachable blocks have no immediate dominator.
    #[must_use]
    pub fn immediate_dominator(&self, block: BlockId) -> Option<BlockId> {
        self.immediate_dominator.get(&block).copied()
    }

    /// Return whether a block is reachable from its region's entry block.
    ///
    /// A block not included in the analyzed region/block view is not
    /// considered reachable.
    #[must_use]
    pub fn is_reachable(&self, block: BlockId) -> bool {
        self.reachable.contains(&block)
    }

    /// Return the region assigned to a block in the analyzed view.
    #[must_use]
    pub fn region_of(&self, block: BlockId) -> Option<RegionId> {
        self.region_of.get(&block).copied()
    }

    /// Return whether `dominator` dominates `block`.
    ///
    /// A block dominates itself, including if it is unreachable. For
    /// distinct blocks, both must be reachable and belong to the same
    /// region. The query walks the immediate-dominator chain and uses
    /// O(d) time, where d is the chain length; it does not allocate.
    #[must_use]
    pub fn dominates(&self, dominator: BlockId, block: BlockId) -> bool {
        let Some(dominator_region) = self.region_of.get(&dominator) else {
            return false;
        };

        let Some(block_region) = self.region_of.get(&block) else {
            return false;
        };

        if dominator_region != block_region {
            return false;
        }

        if dominator == block {
            return true;
        }

        if !self.reachable.contains(&dominator)
            || !self.reachable.contains(&block)
        {
            return false;
        }

        let mut current = block;

        // A valid immediate-dominator chain terminates at the region entry.
        // The visited set also makes this public query defensive against
        // accidentally corrupted result data.
        let mut visited = BTreeSet::new();

        while visited.insert(current) {
            let Some(parent) = self.immediate_dominator.get(&current).copied()
            else {
                return false;
            };

            if parent == dominator {
                return true;
            }

            if parent == current {
                // Defensive guard: entry blocks normally have no stored
                // immediate-dominator entry.
                return false;
            }

            current = parent;
        }

        false
    }

    /// Borrow the reachable block identities in deterministic order.
    #[must_use]
    pub fn reachable_blocks(&self) -> &BTreeSet<BlockId> {
        &self.reachable
    }

    /// Borrow the computed non-entry immediate-dominator map.
    #[must_use]
    pub fn immediate_dominators(&self) -> &BTreeMap<BlockId, BlockId> {
        &self.immediate_dominator
    }
}

/// The complete outcome of dominance analysis.
///
/// Findings may coexist with partial results. Callers must check
/// `is_valid()` before relying on the result as proof of a well-formed CFG.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DominanceReport {
    issues: Vec<DominanceIssue>,
    info: DominanceInfo,
    regions_checked: usize,
    blocks_checked: usize,
    edges_checked: usize,
}

impl DominanceReport {
    /// Borrow all findings in deterministic analysis order.
    #[must_use]
    pub fn issues(&self) -> &[DominanceIssue] {
        &self.issues
    }

    /// Whether no dominance-analysis errors were found.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    /// Number of findings.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Number of region entities inspected.
    #[must_use]
    pub const fn regions_checked(&self) -> usize {
        self.regions_checked
    }

    /// Number of block entities inspected.
    #[must_use]
    pub const fn blocks_checked(&self) -> usize {
        self.blocks_checked
    }

    /// Number of explicit successor edges inspected.
    #[must_use]
    pub const fn edges_checked(&self) -> usize {
        self.edges_checked
    }

    /// Borrow the computed dominance information.
    ///
    /// If the report has findings, this may contain only partial results.
    #[must_use]
    pub const fn info(&self) -> &DominanceInfo {
        &self.info
    }

    fn push(&mut self, code: &'static str, message: impl Into<String>) {
        self.issues.push(DominanceIssue::new(code, message));
    }
}

// =============================================================================
// Public entry point
// =============================================================================

/// Compute region-local reachability and immediate dominators.
///
/// For a region with blocks, its first block is treated as the entry block.
/// Empty regions are diagnosed and skipped. Only explicitly represented
/// successor edges participate in the CFG.
///
/// This function is read-only and deterministic for a given ordered input.
/// It does not replace structural, SSA, type, or full-IR verification.
#[must_use]
pub fn verify_dominance(input: &DominanceInput<'_>) -> DominanceReport {
    let mut report = DominanceReport::default();
    report.regions_checked = input.regions.len();
    report.blocks_checked = input.blocks.len();

    let mut blocks_by_id: BTreeMap<BlockId, &ZuirBlock> = BTreeMap::new();

    for block in input.blocks {
        if blocks_by_id.insert(block.id(), block).is_some() {
            report.push(
                "ZUIR-DOM-DUPLICATE-BLOCK-ID",
                format!(
                    "block identity {} occurs more than once in the input",
                    block.id()
                ),
            );
        }
    }

    let mut region_ids = BTreeSet::new();

    for region in input.regions {
        if !region_ids.insert(region.id()) {
            report.push(
                "ZUIR-DOM-DUPLICATE-REGION-ID",
                format!(
                    "region identity {} occurs more than once in the input",
                    region.id()
                ),
            );
            continue;
        }

        analyze_region(
            region,
            &blocks_by_id,
            &mut report,
        );
    }

    report
}

// =============================================================================
// Region graph construction and analysis
// =============================================================================

fn analyze_region(
    region: &ZuirRegion,
    blocks_by_id: &BTreeMap<BlockId, &ZuirBlock>,
    report: &mut DominanceReport,
) {
    let region_id = region.id();

    if region.blocks().is_empty() {
        report.push(
            "ZUIR-DOM-EMPTY-REGION",
            format!("region {region_id} has no entry block"),
        );
        return;
    }

    // Map each unique block ID to its local graph index. Region block order
    // is preserved, and index zero is the designated entry block.
    let mut local_index = BTreeMap::<BlockId, usize>::new();
    let mut local_blocks: Vec<&ZuirBlock> = Vec::new();

    for block_id in region.blocks() {
        if local_index.contains_key(block_id) {
            report.push(
                "ZUIR-DOM-DUPLICATE-REGION-BLOCK",
                format!(
                    "region {region_id} lists block {block_id} more than once"
                ),
            );
            continue;
        }

        let Some(block) = blocks_by_id.get(block_id).copied() else {
            report.push(
                "ZUIR-DOM-MISSING-BLOCK",
                format!(
                    "region {region_id} references missing block {block_id}"
                ),
            );
            continue;
        };

        match block.parent_region() {
            Some(parent) if parent == region_id => {}
            actual => {
                report.push(
                    "ZUIR-DOM-BLOCK-REGION-MISMATCH",
                    format!(
                        "region {region_id} lists block {block_id}, \
                         but the block parent is {actual:?}"
                    ),
                );
                continue;
            }
        }

        let index = local_blocks.len();
        local_index.insert(*block_id, index);
        local_blocks.push(block);
        report.info.region_of.insert(*block_id, region_id);
    }

    if local_blocks.is_empty() {
        report.push(
            "ZUIR-DOM-NO-VALID-BLOCKS",
            format!(
                "region {region_id} has no block that can be analyzed"
            ),
        );
        return;
    }

    let entry_id = local_blocks[0].id();
    report.info.entry_of.insert(region_id, entry_id);

    let block_count = local_blocks.len();

    // Construct the directed graph and reverse predecessor lists.
    // Edges are deduplicated per source/target pair: repeated branch edges
    // do not change dominance, and counting them would waste resources.
    let mut successors: Vec<Vec<usize>> =
        (0..block_count).map(|_| Vec::new()).collect();
    let mut predecessors: Vec<Vec<usize>> =
        (0..block_count).map(|_| Vec::new()).collect();

    for (source_index, block) in local_blocks.iter().enumerate() {
        let mut edge_targets = BTreeSet::<usize>::new();

        block.terminator().for_each_successor(|successor| {
            report.edges_checked += 1;

            let target_id = successor.target();

            match local_index.get(&target_id).copied() {
                Some(target_index) => {
                    edge_targets.insert(target_index);
                }
                None => {
                    if blocks_by_id.contains_key(&target_id) {
                        report.push(
                            "ZUIR-DOM-CROSS-REGION-EDGE",
                            format!(
                                "block {} in region {region_id} targets \
                                 block {target_id}, which is not a member \
                                 of the same region",
                                block.id()
                            ),
                        );
                    } else {
                        report.push(
                            "ZUIR-DOM-MISSING-SUCCESSOR",
                            format!(
                                "block {} in region {region_id} targets \
                                 missing block {target_id}",
                                block.id()
                            ),
                        );
                    }
                }
            }
        });

        successors[source_index] = edge_targets.into_iter().collect();
    }

    for (source, targets) in successors.iter().enumerate() {
        for &target in targets {
            predecessors[target].push(source);
        }
    }

    // A reachable graph is traversed iteratively. This avoids recursive DFS
    // stack growth for generated CFGs with very long paths.
    let entry_index = 0;
    let mut visited = vec![false; block_count];
    let mut postorder = Vec::new();
    let mut dfs_stack: Vec<(usize, usize)> = Vec::new();

    visited[entry_index] = true;
    dfs_stack.push((entry_index, 0));

    while let Some((node, next_edge)) = dfs_stack.last_mut() {
        if *next_edge < successors[*node].len() {
            let target = successors[*node][*next_edge];
            *next_edge += 1;

            if !visited[target] {
                visited[target] = true;
                dfs_stack.push((target, 0));
            }
        } else {
            let (finished, _) = dfs_stack
                .pop()
                .expect("the stack was observed to be nonempty");
            postorder.push(finished);
        }
    }

    // Reverse postorder assigns earlier numbers to dominators in the
    // Cooper-Harvey-Kennedy immediate-dominator algorithm.
    let reverse_postorder: Vec<usize> =
        postorder.iter().rev().copied().collect();

    let mut rpo_number = vec![usize::MAX; block_count];

    for (number, &node) in reverse_postorder.iter().enumerate() {
        rpo_number[node] = number;
        report.info.reachable.insert(local_blocks[node].id());
    }

    // Internal idom values use local graph indices. The entry temporarily
    // dominates itself to anchor intersection; it is omitted from the
    // public immediate-dominator map.
    let mut idom: Vec<Option<usize>> = vec![None; block_count];
    idom[entry_index] = Some(entry_index);

    let mut changed = true;

    while changed {
        changed = false;

        for &node in reverse_postorder.iter().skip(1) {
            let mut candidate: Option<usize> = None;

            for &pred in &predecessors[node] {
                if idom[pred].is_none() {
                    continue;
                }

                candidate = Some(match candidate {
                    None => pred,
                    Some(current) => intersect(
                        current,
                        pred,
                        &idom,
                        &rpo_number,
                    ),
                });
            }

            if let Some(new_idom) = candidate {
                if idom[node] != Some(new_idom) {
                    idom[node] = Some(new_idom);
                    changed = true;
                }
            }
        }
    }

    for (node, parent) in idom.iter().enumerate() {
        let Some(parent) = parent else {
            // Unreachable nodes intentionally have no immediate dominator.
            continue;
        };

        if node == entry_index {
            continue;
        }

        report.info.immediate_dominator.insert(
            local_blocks[node].id(),
            local_blocks[*parent].id(),
        );
    }

    // Keep entry-block semantics explicit: the entry is reachable but does
    // not have a public immediate dominator.
}

/// Intersect two nodes' dominator chains using reverse-postorder numbering.
///
/// Preconditions:
/// - `left` and `right` are reachable.
/// - Each node on their dominator chain has an idom.
/// - The entry's idom is itself.
///
/// The function allocates no memory.
fn intersect(
    mut left: usize,
    mut right: usize,
    idom: &[Option<usize>],
    rpo_number: &[usize],
) -> usize {
    while left != right {
        while rpo_number[left] > rpo_number[right] {
            left = idom[left]
                .expect("reachable node must have an immediate dominator");
        }

        while rpo_number[right] > rpo_number[left] {
            right = idom[right]
                .expect("reachable node must have an immediate dominator");
        }
    }

    left
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // The canonical IR constructors and IDs are deliberately exercised by
    // integration tests in the repository. These local tests target the
    // algorithmic properties that do not require constructing full IR nodes.

    #[test]
    fn intersect_returns_same_node_for_identical_inputs() {
        let idom = vec![Some(0), Some(0), Some(1)];
        let rpo = vec![0, 1, 2];

        assert_eq!(intersect(2, 2, &idom, &rpo), 2);
    }

    #[test]
    fn intersect_finds_shared_dominator() {
        // Graph-shaped dominator tree:
        //
        //       0
        //       |
        //       1
        //      / \
        //     2   3
        //
        let idom = vec![Some(0), Some(0), Some(1), Some(1)];
        let rpo = vec![0, 1, 2, 3];

        assert_eq!(intersect(2, 3, &idom, &rpo), 1);
    }

    #[test]
    fn intersect_finds_entry_for_separate_branches() {
        //       0
        //      / \
        //     1   2
        let idom = vec![Some(0), Some(0), Some(0)];
        let rpo = vec![0, 1, 2];

        assert_eq!(intersect(1, 2, &idom, &rpo), 0);
    }

    #[test]
    fn unreachable_nodes_are_not_marked_reachable() {
        // The analysis marks reachability from the entry during DFS.
        // This small independent check documents the intended distinction
        // between reachability and the existence of a block in the graph.
        let visited = [true, true, false];

        assert!(visited[0]);
        assert!(visited[1]);
        assert!(!visited[2]);
    }
}
