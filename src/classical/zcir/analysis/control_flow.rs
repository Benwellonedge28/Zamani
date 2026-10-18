
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Control-Flow Graph and Analysis
//!
//! Path: `src/classical/zcir/analysis/control_flow.rs`
//! Edition: Rust 2021.
//! Minimum intended Rust version: 1.97.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Represent and analyze the control-flow graph (CFG) of a classical
//! computation using canonical ZUIR block identities.
//!
//! This module provides:
//!
//! - Explicit graph construction.
//! - Validation of graph membership and edge endpoints.
//! - Deterministic successor and predecessor queries.
//! - Reachability from a designated entry block.
//! - Reverse postorder of reachable blocks.
//! - Unreachable-block discovery.
//! - Exit-block discovery.
//! - Basic graph statistics.
//!
//! ## Architectural authority
//!
//! - `src/zuir/ids.rs` owns `BlockId`.
//! - `src/zuir/ir/block.rs` owns `BlockTerminator` and block-local data.
//! - `src/zuir/ir/successor.rs` owns successor edges and edge arguments.
//! - `src/zuir/ir/region.rs` owns region membership and region ownership.
//! - `src/zuir/ir/operation.rs` owns operations and nested-region references.
//! - `src/zuir/ir/use_def.rs` owns the canonical definition-use index.
//! - `grammar/Zamani.g4` and the canonical parser grammars own syntax.
//! - `grammar/specification/semantic-model.md` defines language meaning.
//! - `grammar/validation/semantic-boundaries.md` defines semantic boundaries.
//!
//! ## Ownership
//!
//! This module owns a derived, analysis-oriented CFG representation.
//!
//! It does not own canonical blocks, operations, terminators, successor
//! arguments, identity allocation, region storage, or source syntax.
//!
//! The caller or an integration adapter constructs this graph from canonical
//! IR. A future adapter must preserve every semantically relevant edge,
//! including conditional, switch, exceptional, and dialect-defined edges
//! where those constructs are supported.
//!
//! ## Validity boundary
//!
//! A structurally valid graph is not necessarily a valid program.
//!
//! This module does not prove:
//!
//! - That terminators are legal or complete.
//! - That all canonical successor references are represented.
//! - That branch arguments match destination block arguments.
//! - That definitions dominate uses.
//! - That types, effects, captures, ownership, or dialect contracts are valid.
//! - That a target supports the computation.
//!
//! Those properties belong to canonical IR construction and verification.
//!
//! ## Determinism
//!
//! Block membership and unique adjacency are represented using ordered
//! collections. Successor and predecessor queries therefore have stable
//! ordering independent of hash seeds or insertion order.
//!
//! Reverse postorder uses an iterative depth-first traversal. It does not
//! depend on the process call stack and has deterministic successor order.
//!
//! Parallel semantic edges are retained in the edge collection, while
//! adjacency queries represent unique source/destination relationships.
//!
//! ## Scalability
//!
//! There are no configured universal limits on block or edge counts.
//! Collections grow dynamically as resources permit. Graph traversals are
//! iterative and have linear graph complexity, excluding ordered-collection
//! operation costs.
//!
//! Standard collection allocation can still fail at the process level.
//! This module does not promise literal infinity or guarantee recovery from
//! every allocator failure.
//!
//! ## Lifecycle
//!
//! This graph is a snapshot. Rebuild it or invalidate dependent analyses
//! whenever block membership, terminators, successor targets, region
//! structure, or relevant edge semantics change.
//!
//! ## Integration
//!
//! Register this module through `src/classical/zcir/analysis/mod.rs`.
//! Construct graphs from canonical ZUIR blocks and terminators through an
//! explicit adapter once the repository's verified conversion boundary is
//! established. Do not duplicate canonical IR types or infer edge semantics
//! from source text.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::BlockId;

/// Result type for control-flow graph operations.
pub type ControlFlowResult<T> = Result<T, ControlFlowError>;

/// Errors detectable by this graph representation and its local analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ControlFlowError {
    /// A block was inserted more than once.
    DuplicateBlock(BlockId),

    /// An edge references a source block not registered in the graph.
    UnknownSourceBlock(BlockId),

    /// An edge references a destination block not registered in the graph.
    UnknownDestinationBlock(BlockId),

    /// The configured entry block is not registered in the graph.
    UnknownEntryBlock(BlockId),

    /// Analysis requires an entry block, but none has been configured.
    MissingEntryBlock,

    /// A block has already been configured as the entry block.
    EntryBlockAlreadySet {
        existing: BlockId,
        attempted: BlockId,
    },

    /// The graph has no registered blocks.
    EmptyGraph,
}

impl fmt::Display for ControlFlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateBlock(block) => {
                write!(f, "duplicate control-flow block {block}")
            }
            Self::UnknownSourceBlock(block) => {
                write!(f, "unknown control-flow source block {block}")
            }
            Self::UnknownDestinationBlock(block) => {
                write!(f, "unknown control-flow destination block {block}")
            }
            Self::UnknownEntryBlock(block) => {
                write!(f, "control-flow entry block {block} is not registered")
            }
            Self::MissingEntryBlock => {
                f.write_str("control-flow analysis requires an entry block")
            }
            Self::EntryBlockAlreadySet {
                existing,
                attempted,
            } => write!(
                f,
                "control-flow entry block is already {existing}; cannot replace it with {attempted}"
            ),
            Self::EmptyGraph => {
                f.write_str("cannot analyze an empty control-flow graph")
            }
        }
    }
}

impl std::error::Error for ControlFlowError {}

/// Semantic category attached to an explicit control-flow edge.
///
/// These categories describe edge intent, not execution policy. The
/// canonical IR adapter is responsible for assigning them correctly.
/// `Other` preserves a generic edge when the caller has no more specific
/// category available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ControlFlowEdgeKind {
    /// Unconditional branch.
    Branch,

    /// True edge of a conditional branch.
    ConditionalTrue,

    /// False edge of a conditional branch.
    ConditionalFalse,

    /// An individual switch case.
    SwitchCase {
        /// Position of the case in the canonical switch representation.
        case_index: usize,
    },

    /// Default edge of a switch.
    SwitchDefault,

    /// Exceptional or failure-handling transfer.
    Exceptional,

    /// A dialect-defined or otherwise classified edge.
    Other,
}

/// One explicit control-flow edge.
///
/// Multiple edges between the same blocks are permitted. For example,
/// separate switch cases may target the same destination. Each edge remains
/// distinguishable by its kind and its position in the edge collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControlFlowEdge {
    source: BlockId,
    destination: BlockId,
    kind: ControlFlowEdgeKind,
}

impl ControlFlowEdge {
    /// Construct an edge value.
    ///
    /// This constructor does not establish graph membership. Use
    /// `ControlFlowGraph::add_edge` to validate both endpoints.
    #[must_use]
    pub const fn new(
        source: BlockId,
        destination: BlockId,
        kind: ControlFlowEdgeKind,
    ) -> Self {
        Self {
            source,
            destination,
            kind,
        }
    }

    /// Return the source block.
    #[must_use]
    pub const fn source(self) -> BlockId {
        self.source
    }

    /// Return the destination block.
    #[must_use]
    pub const fn destination(self) -> BlockId {
        self.destination
    }

    /// Return the semantic category of the edge.
    #[must_use]
    pub const fn kind(self) -> ControlFlowEdgeKind {
        self.kind
    }
}

/// Derived control-flow graph.
///
/// Block identities are canonical ZUIR `BlockId` values. The graph owns
/// only its derived adjacency and edge collections; it does not own or
/// mutate the canonical IR blocks.
#[derive(Debug, Clone, Default)]
pub struct ControlFlowGraph {
    blocks: BTreeSet<BlockId>,
    successors: BTreeMap<BlockId, BTreeSet<BlockId>>,
    predecessors: BTreeMap<BlockId, BTreeSet<BlockId>>,
    edges: Vec<ControlFlowEdge>,
    entry: Option<BlockId>,
}

impl ControlFlowGraph {
    /// Create an empty graph without an entry block.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a block.
    ///
    /// Duplicate block identities are rejected instead of being silently
    /// merged. Adjacency sets are initialized for deterministic lookup.
    pub fn add_block(&mut self, block: BlockId) -> ControlFlowResult<()> {
        if self.blocks.contains(&block) {
            return Err(ControlFlowError::DuplicateBlock(block));
        }

        self.blocks.insert(block);
        self.successors.insert(block, BTreeSet::new());
        self.predecessors.insert(block, BTreeSet::new());

        Ok(())
    }

    /// Configure the graph entry block.
    ///
    /// The entry must already have been registered. Reconfiguration is
    /// rejected to prevent accidental changes to the graph's interpretation.
    pub fn set_entry(&mut self, block: BlockId) -> ControlFlowResult<()> {
        if !self.blocks.contains(&block) {
            return Err(ControlFlowError::UnknownEntryBlock(block));
        }

        if let Some(existing) = self.entry {
            return Err(ControlFlowError::EntryBlockAlreadySet {
                existing,
                attempted: block,
            });
        }

        self.entry = Some(block);
        Ok(())
    }

    /// Add an explicit edge after validating its endpoints.
    ///
    /// Parallel edges are retained. Successor and predecessor queries expose
    /// unique adjacent blocks, because reachability does not depend on the
    /// number of edges connecting the same pair.
    pub fn add_edge(
        &mut self,
        edge: ControlFlowEdge,
    ) -> ControlFlowResult<()> {
        if !self.blocks.contains(&edge.source) {
            return Err(ControlFlowError::UnknownSourceBlock(edge.source));
        }

        if !self.blocks.contains(&edge.destination) {
            return Err(ControlFlowError::UnknownDestinationBlock(
                edge.destination,
            ));
        }

        self.edges.push(edge);

        if let Some(successors) = self.successors.get_mut(&edge.source) {
            successors.insert(edge.destination);
        }

        if let Some(predecessors) = self.predecessors.get_mut(&edge.destination) {
            predecessors.insert(edge.source);
        }

        Ok(())
    }

    /// Register an edge without assigning a specialized category.
    pub fn connect(
        &mut self,
        source: BlockId,
        destination: BlockId,
    ) -> ControlFlowResult<()> {
        self.add_edge(ControlFlowEdge::new(
            source,
            destination,
            ControlFlowEdgeKind::Other,
        ))
    }

    /// Return the configured entry block, if any.
    #[must_use]
    pub const fn entry(&self) -> Option<BlockId> {
        self.entry
    }

    /// Return whether the graph contains the specified block.
    #[must_use]
    pub fn contains_block(&self, block: BlockId) -> bool {
        self.blocks.contains(&block)
    }

    /// Iterate through registered blocks in canonical identity order.
    pub fn blocks(&self) -> impl Iterator<Item = BlockId> + '_ {
        self.blocks.iter().copied()
    }

    /// Iterate through all explicit edges in insertion order.
    ///
    /// Insertion order is retained for consumers that need to correlate
    /// edges with an input representation. It is not a canonical
    /// serialization order.
    pub fn edges(&self) -> impl Iterator<Item = &ControlFlowEdge> {
        self.edges.iter()
    }

    /// Return unique successors in deterministic identity order.
    #[must_use]
    pub fn successors(&self, block: BlockId) -> Option<&BTreeSet<BlockId>> {
        self.successors.get(&block)
    }

    /// Return unique predecessors in deterministic identity order.
    #[must_use]
    pub fn predecessors(&self, block: BlockId) -> Option<&BTreeSet<BlockId>> {
        self.predecessors.get(&block)
    }

    /// Number of registered blocks.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Number of explicit edges, including parallel edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Return whether the graph has no registered blocks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Compute reachability and reverse postorder from the entry.
    ///
    /// The traversal is iterative and does not recurse through the Rust
    /// call stack. Its deterministic successor order comes from BTreeSet.
    pub fn analyze(&self) -> ControlFlowResult<ControlFlowAnalysis> {
        if self.blocks.is_empty() {
            return Err(ControlFlowError::EmptyGraph);
        }

        let entry = self.entry.ok_or(ControlFlowError::MissingEntryBlock)?;

        if !self.blocks.contains(&entry) {
            return Err(ControlFlowError::UnknownEntryBlock(entry));
        }

        // Iterative DFS with explicit enter/exit states. A block is marked
        // when first discovered, preventing cycles from growing the stack
        // indefinitely.
        let mut visited = BTreeSet::new();
        let mut postorder = Vec::new();
        let mut stack = vec![(entry, false)];

        while let Some((block, exiting)) = stack.pop() {
            if exiting {
                postorder.push(block);
                continue;
            }

            if !visited.insert(block) {
                continue;
            }

            stack.push((block, true));

            if let Some(successors) = self.successors.get(&block) {
                // Reverse insertion into the LIFO stack produces ascending
                // traversal order.
                for successor in successors.iter().rev() {
                    if !visited.contains(successor) {
                        stack.push((*successor, false));
                    }
                }
            }
        }

        postorder.reverse();

        let unreachable = self
            .blocks
            .difference(&visited)
            .copied()
            .collect::<BTreeSet<_>>();

        let exits = visited
            .iter()
            .filter(|block| {
                self.successors
                    .get(block)
                    .is_some_and(BTreeSet::is_empty)
            })
            .copied()
            .collect::<BTreeSet<_>>();

        Ok(ControlFlowAnalysis {
            entry,
            reachable: visited,
            unreachable,
            reverse_postorder: postorder,
            reachable_exits: exits,
        })
    }
}

/// Results of deterministic entry-rooted CFG traversal.
///
/// The sets and sequence are derived snapshots. Recompute after graph
/// mutation; do not treat them as live views of the graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlFlowAnalysis {
    entry: BlockId,
    reachable: BTreeSet<BlockId>,
    unreachable: BTreeSet<BlockId>,
    reverse_postorder: Vec<BlockId>,
    reachable_exits: BTreeSet<BlockId>,
}

impl ControlFlowAnalysis {
    /// Return the analyzed entry block.
    #[must_use]
    pub const fn entry(&self) -> BlockId {
        self.entry
    }

    /// Return all blocks reachable from the entry.
    #[must_use]
    pub fn reachable(&self) -> &BTreeSet<BlockId> {
        &self.reachable
    }

    /// Return all registered blocks not reachable from the entry.
    #[must_use]
    pub fn unreachable(&self) -> &BTreeSet<BlockId> {
        &self.unreachable
    }

    /// Return reachable blocks in reverse postorder.
    #[must_use]
    pub fn reverse_postorder(&self) -> &[BlockId] {
        &self.reverse_postorder
    }

    /// Return reachable blocks with no outgoing graph edges.
    ///
    /// This is a graph-theoretic definition of an exit. It does not establish
    /// that the corresponding canonical terminator is a legal return or
    /// other valid termination construct.
    #[must_use]
    pub fn reachable_exits(&self) -> &BTreeSet<BlockId> {
        &self.reachable_exits
    }

    /// Return the number of reachable blocks.
    #[must_use]
    pub fn reachable_count(&self) -> usize {
        self.reachable.len()
    }

    /// Return the number of unreachable blocks.
    #[must_use]
    pub fn unreachable_count(&self) -> usize {
        self.unreachable.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ids::BlockId;

    fn block(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test block IDs must be nonzero")
    }

    fn graph_with_blocks(ids: &[u128], entry: u128) -> ControlFlowGraph {
        let mut graph = ControlFlowGraph::new();

        for id in ids {
            graph.add_block(block(*id)).expect("unique block");
        }

        graph.set_entry(block(entry)).expect("registered entry");
        graph
    }

    #[test]
    fn empty_graph_cannot_be_analyzed() {
        let graph = ControlFlowGraph::new();
        assert_eq!(graph.analyze(), Err(ControlFlowError::EmptyGraph));
    }

    #[test]
    fn nonempty_graph_requires_entry() {
        let mut graph = ControlFlowGraph::new();
        graph.add_block(block(1)).expect("block");

        assert_eq!(graph.analyze(), Err(ControlFlowError::MissingEntryBlock));
    }

    #[test]
    fn rejects_duplicate_blocks() {
        let mut graph = ControlFlowGraph::new();
        graph.add_block(block(1)).expect("first insertion");

        assert_eq!(
            graph.add_block(block(1)),
            Err(ControlFlowError::DuplicateBlock(block(1)))
        );
    }

    #[test]
    fn rejects_unknown_edge_endpoints() {
        let mut graph = graph_with_blocks(&[1], 1);

        assert_eq!(
            graph.connect(block(2), block(1)),
            Err(ControlFlowError::UnknownSourceBlock(block(2)))
        );

        assert_eq!(
            graph.connect(block(1), block(2)),
            Err(ControlFlowError::UnknownDestinationBlock(block(2)))
        );
    }

    #[test]
    fn computes_reachability_and_unreachable_blocks() {
        let mut graph = graph_with_blocks(&[1, 2, 3, 4], 1);

        graph.connect(block(1), block(2)).expect("edge");
        graph.connect(block(2), block(3)).expect("edge");

        let analysis = graph.analyze().expect("analysis");

        assert_eq!(
            analysis.reachable(),
            &BTreeSet::from([block(1), block(2), block(3)])
        );
        assert_eq!(
            analysis.unreachable(),
            &BTreeSet::from([block(4)])
        );
        assert_eq!(analysis.reachable_count(), 3);
        assert_eq!(analysis.unreachable_count(), 1);
    }

    #[test]
    fn handles_cycles_without_recursive_traversal() {
        let mut graph = graph_with_blocks(&[1, 2, 3], 1);

        graph.connect(block(1), block(2)).expect("edge");
        graph.connect(block(2), block(3)).expect("edge");
        graph.connect(block(3), block(1)).expect("back edge");

        let analysis = graph.analyze().expect("analysis");

        assert_eq!(analysis.reachable_count(), 3);
        assert_eq!(analysis.unreachable_count(), 0);
        assert!(analysis.reachable_exits().is_empty());
    }

    #[test]
    fn parallel_edges_are_preserved_but_adjacency_is_unique() {
        let mut graph = graph_with_blocks(&[1, 2], 1);

        graph
            .add_edge(ControlFlowEdge::new(
                block(1),
                block(2),
                ControlFlowEdgeKind::ConditionalTrue,
            ))
            .expect("first edge");

        graph
            .add_edge(ControlFlowEdge::new(
                block(1),
                block(2),
                ControlFlowEdgeKind::ConditionalFalse,
            ))
            .expect("parallel edge");

        assert_eq!(graph.edge_count(), 2);
        assert_eq!(
            graph.successors(block(1)).expect("known block").len(),
            1
        );
        assert_eq!(
            graph.predecessors(block(2)).expect("known block").len(),
            1
        );
    }

    #[test]
    fn reports_reachable_exits() {
        let mut graph = graph_with_blocks(&[1, 2, 3], 1);

        graph.connect(block(1), block(2)).expect("edge");
        graph.connect(block(1), block(3)).expect("edge");

        let analysis = graph.analyze().expect("analysis");

        assert_eq!(
            analysis.reachable_exits(),
            &BTreeSet::from([block(2), block(3)])
        );
    }

    #[test]
    fn traversal_order_is_deterministic() {
        let mut graph = graph_with_blocks(&[1, 2, 3, 4], 1);

        graph.connect(block(1), block(3)).expect("edge");
        graph.connect(block(1), block(2)).expect("edge");
        graph.connect(block(2), block(4)).expect("edge");

        let first = graph.analyze().expect("first analysis");
        let second = graph.analyze().expect("second analysis");

        assert_eq!(
            first.reverse_postorder(),
            second.reverse_postorder()
        );
        assert_eq!(first, second);
    }

    #[test]
    fn rejects_entry_reconfiguration() {
        let mut graph = graph_with_blocks(&[1, 2], 1);

        assert_eq!(
            graph.set_entry(block(2)),
            Err(ControlFlowError::EntryBlockAlreadySet {
                existing: block(1),
                attempted: block(2),
            })
        );
    }
}
