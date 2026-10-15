
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Dominance Analysis
//!
//! Computes dominators, immediate dominators, dominator-tree children, and
//! dominance frontiers for a region-local control-flow graph.
//!
//! The implementation uses the Lengauer–Tarjan algorithm and iterative
//! traversals. It imposes no fixed graph-size or hardware-specific limits.
//!
//! Integration requirement:
//! `ControlFlowGraph` must expose `pub fn entry(&self) -> Option<BlockId>`.
//! Add that accessor to `analysis/cfg.rs`; never infer the entry from block
//! declaration order.
//!
//! This analysis is not a substitute for complete ZUIR verification.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::analysis::cfg::ControlFlowGraph;
use crate::zuir::ids::BlockId;

/// Result type for dominance analysis.
pub type DominanceResult<T> = Result<T, DominanceError>;

/// Errors reported by dominance analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DominanceError {
    /// The CFG has no explicitly declared entry.
    MissingEntry,

    /// The declared entry is absent from the graph.
    InvalidEntry {
        entry: BlockId,
    },

    /// An edge references a node missing from the graph's node index.
    InvalidGraphEdge {
        source: BlockId,
        target: BlockId,
    },

    /// A required vector reservation failed.
    AllocationFailure {
        collection: &'static str,
    },

    /// The graph exceeds the index space supported by this process.
    IndexSpaceExhausted,
}

impl DominanceError {
    /// Return a stable machine-readable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingEntry => "ZUIR-DOM-MISSING-ENTRY",
            Self::InvalidEntry { .. } => "ZUIR-DOM-INVALID-ENTRY",
            Self::InvalidGraphEdge { .. } => "ZUIR-DOM-INVALID-EDGE",
            Self::AllocationFailure { .. } => "ZUIR-DOM-ALLOCATION-FAILURE",
            Self::IndexSpaceExhausted => "ZUIR-DOM-INDEX-SPACE-EXHAUSTED",
        }
    }
}

impl fmt::Display for DominanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEntry => {
                f.write_str("dominance analysis requires an explicit CFG entry")
            }
            Self::InvalidEntry { entry } => {
                write!(f, "CFG entry block {entry} is absent from the graph")
            }
            Self::InvalidGraphEdge { source, target } => {
                write!(f, "CFG edge {source} -> {target} references a missing node")
            }
            Self::AllocationFailure { collection } => {
                write!(f, "unable to reserve memory for dominance {collection}")
            }
            Self::IndexSpaceExhausted => {
                f.write_str("dominance graph exceeds the platform index space")
            }
        }
    }
}

impl std::error::Error for DominanceError {}

fn reserve<T>(
    values: &mut Vec<T>,
    additional: usize,
    collection: &'static str,
) -> DominanceResult<()> {
    values
        .try_reserve(additional)
        .map_err(|_| DominanceError::AllocationFailure { collection })
}

/// Immutable results of dominance analysis.
///
/// Only blocks reachable from the CFG entry participate in dominance and
/// dominance-frontier calculations. Unreachable blocks remain explicitly
/// available through `unreachable_blocks()`.
#[derive(Debug, Clone)]
pub struct DominanceInfo {
    entry: BlockId,
    reachable: BTreeSet<BlockId>,
    unreachable: BTreeSet<BlockId>,
    immediate_dominators: BTreeMap<BlockId, BlockId>,
    children: BTreeMap<BlockId, Vec<BlockId>>,
    frontiers: BTreeMap<BlockId, BTreeSet<BlockId>>,
}

impl DominanceInfo {
    /// Compute dominance information for a CFG.
    ///
    /// Uses Lengauer–Tarjan with iterative DFS and iterative path
    /// compression. Parallel CFG edges are preserved by the CFG, but are
    /// deduplicated where the dominance-frontier algorithm requires distinct
    /// predecessor blocks.
    pub fn compute(cfg: &ControlFlowGraph) -> DominanceResult<Self> {
        let entry = cfg.entry().ok_or(DominanceError::MissingEntry)?;

        if !cfg.contains_block(entry) {
            return Err(DominanceError::InvalidEntry { entry });
        }

        let node_ids: Vec<BlockId> = cfg.ordered_node_ids().collect();
        let node_count = node_ids.len();

        let mut node_index = BTreeMap::<BlockId, usize>::new();
        for (index, &block) in node_ids.iter().enumerate() {
            node_index.insert(block, index);
        }

        let entry_index = *node_index
            .get(&entry)
            .ok_or(DominanceError::InvalidEntry { entry })?;

        // Build index-based adjacency. This avoids repeatedly searching the
        // graph by BlockId during the core algorithm.
        let mut successors = Vec::<Vec<usize>>::new();
        let mut predecessors = Vec::<Vec<usize>>::new();
        reserve(&mut successors, node_count, "successors")?;
        reserve(&mut predecessors, node_count, "predecessors")?;

        for _ in 0..node_count {
            successors.push(Vec::new());
            predecessors.push(Vec::new());
        }

        for &source in &node_ids {
            let source_index = *node_index
                .get(&source)
                .ok_or(DominanceError::IndexSpaceExhausted)?;

            for edge in cfg.successors(source) {
                let target = edge.target();
                let target_index = *node_index.get(&target).ok_or(
                    DominanceError::InvalidGraphEdge { source, target },
                )?;

                successors[source_index].push(target_index);
                predecessors[target_index].push(source_index);
            }
        }

        // Iterative DFS numbering. Zero means unreachable; reachable DFS
        // numbers occupy 1..=reachable_count.
        let mut dfs_number = vec![0_usize; node_count];
        let mut vertex = Vec::<usize>::new();
        let mut parent = Vec::<usize>::new();

        reserve(&mut vertex, node_count.saturating_add(1), "DFS vertices")?;
        reserve(&mut parent, node_count.saturating_add(1), "DFS parents")?;
        vertex.push(usize::MAX);
        parent.push(0);

        dfs_number[entry_index] = 1;
        vertex.push(entry_index);
        parent.push(0);

        let mut stack = Vec::<(usize, usize)>::new();
        reserve(&mut stack, node_count, "DFS stack")?;
        stack.push((entry_index, 0));

        while let Some((current, next_successor)) = stack.last_mut() {
            if *next_successor >= successors[*current].len() {
                stack.pop();
                continue;
            }

            let next = successors[*current][*next_successor];
            *next_successor += 1;

            if dfs_number[next] != 0 {
                continue;
            }

            let next_dfs = vertex.len();
            dfs_number[next] = next_dfs;
            vertex.push(next);
            parent.push(dfs_number[*current]);
            stack.push((next, 0));
        }

        let reachable_count = vertex.len().saturating_sub(1);

        if reachable_count == 0 {
            return Err(DominanceError::InvalidEntry { entry });
        }

        // Convert incoming graph edges to DFS-numbered predecessors.
        let mut pred_dfs = Vec::<Vec<usize>>::new();
        reserve(
            &mut pred_dfs,
            reachable_count.saturating_add(1),
            "DFS predecessors",
        )?;
        pred_dfs.push(Vec::new());

        for dfs in 1..=reachable_count {
            let node = vertex[dfs];
            let mut incoming = Vec::new();

            for &pred_node in &predecessors[node] {
                let pred_number = dfs_number[pred_node];
                if pred_number != 0 {
                    incoming.push(pred_number);
                }
            }

            pred_dfs.push(incoming);
        }

        let mut semi: Vec<usize> = (0..=reachable_count).collect();
        let mut label: Vec<usize> = (0..=reachable_count).collect();
        let mut ancestor = vec![0_usize; reachable_count + 1];
        let mut idom = vec![0_usize; reachable_count + 1];

        let mut bucket = Vec::<Vec<usize>>::new();
        reserve(&mut bucket, reachable_count + 1, "buckets")?;
        for _ in 0..=reachable_count {
            bucket.push(Vec::new());
        }

        let mut path = Vec::<usize>::new();
        reserve(&mut path, reachable_count, "path compression")?;

        // Lengauer–Tarjan reverse pass.
        for w in (2..=reachable_count).rev() {
            for &v in &pred_dfs[w] {
                let u = eval(v, &mut ancestor, &mut label, &semi, &mut path);
                semi[w] = semi[w].min(semi[u]);
            }

            bucket[semi[w]].push(w);

            let p = parent[w];
            ancestor[w] = p;

            let pending = std::mem::take(&mut bucket[p]);
            for v in pending {
                let u = eval(v, &mut ancestor, &mut label, &semi, &mut path);
                idom[v] = if semi[u] < semi[v] { u } else { p };
            }
        }

        // Resolve provisional immediate dominators.
        for w in 2..=reachable_count {
            if idom[w] != semi[w] {
                idom[w] = idom[idom[w]];
            }
        }

        // Translate DFS-numbered results into typed IDs.
        let mut reachable = BTreeSet::new();
        let mut unreachable = BTreeSet::new();

        for (index, &block) in node_ids.iter().enumerate() {
            if dfs_number[index] == 0 {
                unreachable.insert(block);
            } else {
                reachable.insert(block);
            }
        }

        let mut immediate_dominators = BTreeMap::new();
        let mut children = BTreeMap::<BlockId, Vec<BlockId>>::new();
        let mut frontiers = BTreeMap::<BlockId, BTreeSet<BlockId>>::new();

        for &block in &reachable {
            children.insert(block, Vec::new());
            frontiers.insert(block, BTreeSet::new());
        }

        for w in 2..=reachable_count {
            let block = node_ids[vertex[w]];
            let dominator = node_ids[vertex[idom[w]]];

            immediate_dominators.insert(block, dominator);

            let Some(kids) = children.get_mut(&dominator) else {
                return Err(DominanceError::IndexSpaceExhausted);
            };
            kids.push(block);
        }

        for kids in children.values_mut() {
            kids.sort_unstable();
        }

        // Standard dominance-frontier construction. Only reachable
        // predecessors count; duplicate edges from the same predecessor
        // are collapsed before checking whether a block is a join.
        for &block in &reachable {
            let block_index = node_index[&block];
            let mut incoming = Vec::<usize>::new();

            for &pred in &predecessors[block_index] {
                if dfs_number[pred] != 0 {
                    incoming.push(pred);
                }
            }

            incoming.sort_unstable();
            incoming.dedup();

            if incoming.len() < 2 {
                continue;
            }

            let block_dfs = dfs_number[block_index];

            for pred in incoming {
                let mut runner = dfs_number[pred];

                while runner != 0 && runner != idom[block_dfs] {
                    let runner_block = node_ids[vertex[runner]];
                    if let Some(frontier) = frontiers.get_mut(&runner_block) {
                        frontier.insert(block);
                    }

                    let next = idom[runner];
                    if next == runner {
                        break;
                    }
                    runner = next;
                }
            }
        }

        Ok(Self {
            entry,
            reachable,
            unreachable,
            immediate_dominators,
            children,
            frontiers,
        })
    }

    /// Return the explicit CFG entry.
    #[must_use]
    pub const fn entry(&self) -> BlockId {
        self.entry
    }

    /// Return all blocks reachable from the entry.
    #[must_use]
    pub fn reachable_blocks(&self) -> &BTreeSet<BlockId> {
        &self.reachable
    }

    /// Return all blocks unreachable from the entry.
    #[must_use]
    pub fn unreachable_blocks(&self) -> &BTreeSet<BlockId> {
        &self.unreachable
    }

    /// Test whether a block is reachable from the entry.
    #[must_use]
    pub fn is_reachable(&self, block: BlockId) -> bool {
        self.reachable.contains(&block)
    }

    /// Return a block's immediate dominator, or `None` for the entry and
    /// blocks not reachable from the entry.
    #[must_use]
    pub fn immediate_dominator(&self, block: BlockId) -> Option<BlockId> {
        self.immediate_dominators.get(&block).copied()
    }

    /// Borrow all computed immediate dominators.
    #[must_use]
    pub fn immediate_dominators(&self) -> &BTreeMap<BlockId, BlockId> {
        &self.immediate_dominators
    }

    /// Borrow the dominator-tree children of a block.
    #[must_use]
    pub fn children(&self, block: BlockId) -> &[BlockId] {
        self.children.get(&block).map_or(&[], Vec::as_slice)
    }

    /// Borrow the dominance frontier of a block.
    #[must_use]
    pub fn dominance_frontier(&self, block: BlockId) -> Option<&BTreeSet<BlockId>> {
        self.frontiers.get(&block)
    }

    /// Borrow all computed dominance frontiers.
    #[must_use]
    pub fn dominance_frontiers(&self) -> &BTreeMap<BlockId, BTreeSet<BlockId>> {
        &self.frontiers
    }

    /// Return whether `dominator` dominates `block`.
    ///
    /// A block dominates itself when it is reachable. Unreachable blocks
    /// have no dominance relation in this entry-rooted analysis.
    #[must_use]
    pub fn dominates(&self, dominator: BlockId, block: BlockId) -> bool {
        if !self.reachable.contains(&dominator) || !self.reachable.contains(&block) {
            return false;
        }

        let mut current = block;

        loop {
            if current == dominator {
                return true;
            }

            let Some(parent) = self.immediate_dominator(current) else {
                return false;
            };

            current = parent;
        }
    }

    /// Number of reachable blocks analyzed.
    #[must_use]
    pub fn reachable_count(&self) -> usize {
        self.reachable.len()
    }

    /// Number of unreachable blocks excluded from dominance calculations.
    #[must_use]
    pub fn unreachable_count(&self) -> usize {
        self.unreachable.len()
    }
}

/// Evaluate a Lengauer–Tarjan vertex and compress its ancestor path.
///
/// `0` is the sentinel and is never a real DFS number. The iterative
/// implementation avoids recursion on adversarially deep graphs.
fn eval(
    vertex: usize,
    ancestor: &mut [usize],
    label: &mut [usize],
    semi: &[usize],
    path: &mut Vec<usize>,
) -> usize {
    if ancestor[vertex] == 0 {
        return label[vertex];
    }

    path.clear();
    let mut current = vertex;

    while ancestor[current] != 0 && ancestor[ancestor[current]] != 0 {
        path.push(current);
        current = ancestor[current];
    }

    for &node in path.iter().rev() {
        let parent = ancestor[node];

        if semi[label[parent]] < semi[label[node]] {
            label[node] = label[parent];
        }

        ancestor[node] = ancestor[parent];
    }

    label[vertex]
}
