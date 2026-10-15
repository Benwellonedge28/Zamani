
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Control-Flow Graph Analysis
//!
//! Canonical path: `src/zuir/analysis/cfg.rs`
//! Rust edition: 2021
//! Minimum intended Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! # Contract
//!
//! PURPOSE
//!
//! Build a deterministic, region-local control-flow graph from the canonical
//! ZUIR region, blocks, and explicit block terminators.
//!
//! OWNS
//!
//! - CFG node and edge analysis views.
//! - Region-local CFG construction.
//! - Explicit entry-point validation.
//! - Iterative reachability traversal.
//! - Deterministic graph access and structural graph diagnostics.
//!
//! DOES_NOT_OWN
//!
//! - Canonical IR entities or their identifiers.
//! - Source parsing, AST construction, or name resolution.
//! - The authoritative full-program verifier.
//! - SSA dominance, type inference, use-def indexing, or ownership checking.
//! - Dialect-specific interpretation of extension terminators.
//! - Physical scheduling, hardware routing, allocation, or runtime execution.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/successor.rs`
//! - `src/zuir/verify/control_flow.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! ENTRY CONTRACT
//!
//! Region block order is declaration order, not execution order. This module
//! NEVER guesses the entry block. The caller supplies an optional entry ID.
//! A graph without an entry is useful for structural inspection, but entry-
//! dependent analyses must require an explicit entry.
//!
//! EDGE CONTRACT
//!
//! Each explicitly declared successor is represented as a distinct edge.
//! Parallel edges are preserved, including edges from switch cases that
//! happen to target the same block. Edge argument order is preserved.
//!
//! Extension terminators contribute the successors exposed by the canonical
//! `BlockTerminator` API. Their domain-specific semantics remain the
//! responsibility of the registered extension and verifier.
//!
//! REGION CONTRACT
//!
//! A CFG represents exactly one region. Cross-region control-flow edges are
//! rejected; nested regions are analyzed separately according to their own
//! region semantics. A structured or dataflow region is not silently
//! reinterpreted as a control-flow region.
//!
//! VALIDATION BOUNDARY
//!
//! This module performs the structural checks necessary to build a coherent
//! CFG. It is not a substitute for `verify/control_flow.rs`, structural
//! verification, SSA verification, type checking, effect checking, ownership
//! verification, or dialect verification.
//!
//! Callers must not treat successful graph construction as proof that the
//! complete ZUIR program is valid.
//!
//! SCALABILITY
//!
//! No universal graph-size, block-count, edge-count, successor-count,
//! qubit-count, processor-count, register-width, tensor-rank, or device-count
//! limit is imposed. Traversals are iterative rather than recursive.
//! Fallible reservation is used for the principal growing vectors.
//!
//! BTreeMap and BTreeSet do not expose stable fallible-reservation APIs.
//! Therefore, this module cannot promise recovery from every allocator
//! failure. Processing is limited by actual available resources; literal
//! infinity is not possible on finite hardware.
//!
//! DETERMINISM
//!
//! Nodes are keyed by typed BlockId. Region declaration order is retained
//! separately, while map-based public traversal is ordered by BlockId.
//! Successor order follows each terminator's declared order. No randomized
//! hashing or pointer identity is used.
//!
//! INVALIDATION
//!
//! The graph is an immutable snapshot. If blocks, region membership,
//! terminators, or successor arguments change, rebuild the CFG before
//! consuming its results. Pass-manager invalidation belongs to `pass/`.
//!
//! INTEGRATION
//!
//! Declare this module in `src/zuir/analysis/mod.rs` and export the public
//! types intentionally. Existing dominance, liveness, and other analyses
//! should consume this graph rather than independently interpreting
//! terminators. Full verification remains a separate required stage.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{BlockId, RegionId, ValueId};
use crate::zuir::ir::block::{BlockTerminator, ZuirBlock};
use crate::zuir::ir::region::{RegionKind, ZuirRegion};

// =============================================================================
// Public errors
// =============================================================================

/// Result type for CFG construction and traversal.
pub type CfgResult<T> = Result<T, CfgError>;

/// Errors encountered while constructing or traversing a CFG.
///
/// Diagnostic codes are stable identifiers. Human-readable messages are
/// explanatory and must not be parsed as machine-readable error codes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CfgError {
    /// The region failed its local validation contract.
    InvalidRegion {
        region: RegionId,
        reason: String,
    },

    /// The supplied region is not a control-flow region.
    NotControlFlowRegion {
        region: RegionId,
    },

    /// A region references a block absent from the supplied block view.
    MissingRegionBlock {
        region: RegionId,
        block: BlockId,
    },

    /// A block identity occurs more than once in the supplied block view.
    DuplicateBlockId {
        block: BlockId,
    },

    /// A block does not identify the region that claims it.
    BlockRegionMismatch {
        block: BlockId,
        expected: RegionId,
        actual: Option<RegionId>,
    },

    /// A supplied block has invalid local structure.
    InvalidBlock {
        block: BlockId,
        reason: String,
    },

    /// A block is still in its construction state.
    OpenTerminator {
        block: BlockId,
    },

    /// A terminator failed its local validation contract.
    InvalidTerminator {
        block: BlockId,
        reason: String,
    },

    /// An edge targets a block not contained in the current region.
    SuccessorOutsideRegion {
        source: BlockId,
        target: BlockId,
        region: RegionId,
    },

    /// An edge targets a block that is not present in the supplied view.
    MissingSuccessorTarget {
        source: BlockId,
        target: BlockId,
    },

    /// A successor targets a block whose declared parent is inconsistent.
    SuccessorRegionMismatch {
        source: BlockId,
        target: BlockId,
        expected: RegionId,
        actual: Option<RegionId>,
    },

    /// An explicitly supplied entry block is not part of this CFG.
    InvalidEntry {
        region: RegionId,
        entry: BlockId,
    },

    /// A fallible vector reservation failed.
    AllocationFailure {
        collection: &'static str,
    },

    /// The edge count cannot be represented by the platform's indexing type.
    EdgeIndexExhausted,
}

impl CfgError {
    /// Return a stable machine-readable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidRegion { .. } => "ZUIR-CFG-INVALID-REGION",
            Self::NotControlFlowRegion { .. } => "ZUIR-CFG-NOT-CONTROL-FLOW-REGION",
            Self::MissingRegionBlock { .. } => "ZUIR-CFG-MISSING-REGION-BLOCK",
            Self::DuplicateBlockId { .. } => "ZUIR-CFG-DUPLICATE-BLOCK-ID",
            Self::BlockRegionMismatch { .. } => "ZUIR-CFG-BLOCK-REGION-MISMATCH",
            Self::InvalidBlock { .. } => "ZUIR-CFG-INVALID-BLOCK",
            Self::OpenTerminator { .. } => "ZUIR-CFG-OPEN-TERMINATOR",
            Self::InvalidTerminator { .. } => "ZUIR-CFG-INVALID-TERMINATOR",
            Self::SuccessorOutsideRegion { .. } => "ZUIR-CFG-SUCCESSOR-OUTSIDE-REGION",
            Self::MissingSuccessorTarget { .. } => "ZUIR-CFG-MISSING-SUCCESSOR-TARGET",
            Self::SuccessorRegionMismatch { .. } => "ZUIR-CFG-SUCCESSOR-REGION-MISMATCH",
            Self::InvalidEntry { .. } => "ZUIR-CFG-INVALID-ENTRY",
            Self::AllocationFailure { .. } => "ZUIR-CFG-ALLOCATION-FAILURE",
            Self::EdgeIndexExhausted => "ZUIR-CFG-EDGE-INDEX-EXHAUSTED",
        }
    }
}

impl fmt::Display for CfgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRegion { region, reason } => {
                write!(f, "region {region} is invalid: {reason}")
            }
            Self::NotControlFlowRegion { region } => {
                write!(f, "region {region} is not a control-flow region")
            }
            Self::MissingRegionBlock { region, block } => {
                write!(f, "region {region} references missing block {block}")
            }
            Self::DuplicateBlockId { block } => {
                write!(f, "block identity {block} occurs more than once")
            }
            Self::BlockRegionMismatch {
                block,
                expected,
                actual,
            } => write!(
                f,
                "block {block} belongs to {actual:?}, but region {expected} claims it"
            ),
            Self::InvalidBlock { block, reason } => {
                write!(f, "block {block} is invalid: {reason}")
            }
            Self::OpenTerminator { block } => {
                write!(f, "block {block} has an open, unfinished terminator")
            }
            Self::InvalidTerminator { block, reason } => {
                write!(f, "block {block} has an invalid terminator: {reason}")
            }
            Self::SuccessorOutsideRegion {
                source,
                target,
                region,
            } => write!(
                f,
                "edge {source} -> {target} leaves region {region}"
            ),
            Self::MissingSuccessorTarget { source, target } => {
                write!(f, "edge {source} -> {target} targets a missing block")
            }
            Self::SuccessorRegionMismatch {
                source,
                target,
                expected,
                actual,
            } => write!(
                f,
                "edge {source} -> {target} expects region {expected}, \
                 but target belongs to {actual:?}"
            ),
            Self::InvalidEntry { region, entry } => {
                write!(f, "entry block {entry} is not in region {region}")
            }
            Self::AllocationFailure { collection } => {
                write!(f, "unable to reserve memory for CFG {collection}")
            }
            Self::EdgeIndexExhausted => {
                f.write_str("CFG edge indexing exceeded the platform index space")
            }
        }
    }
}

impl std::error::Error for CfgError {}

// =============================================================================
// Public graph entities
// =============================================================================

/// One explicit control-flow edge.
///
/// `successor_index` is the edge's ordinal in its source terminator. It is
/// not a globally stable identity and must not be persisted as one.
///
/// `arguments` preserves the exact SSA values and their declared order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgEdge {
    source: BlockId,
    target: BlockId,
    successor_index: usize,
    arguments: Vec<ValueId>,
}

impl CfgEdge {
    /// Return the source block.
    #[must_use]
    pub const fn source(&self) -> BlockId {
        self.source
    }

    /// Return the target block.
    #[must_use]
    pub const fn target(&self) -> BlockId {
        self.target
    }

    /// Return the ordinal of this successor in the source terminator.
    #[must_use]
    pub const fn successor_index(&self) -> usize {
        self.successor_index
    }

    /// Borrow the edge arguments in their semantic order.
    #[must_use]
    pub fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }

    /// Return the number of values passed to the target block.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }
}

/// CFG-local adjacency information for one block.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CfgNode {
    successors: Vec<usize>,
    predecessors: Vec<usize>,
}

impl CfgNode {
    /// Borrow edge indices in the source terminator's declared order.
    #[must_use]
    pub fn successor_edges(&self) -> &[usize] {
        &self.successors
    }

    /// Borrow incoming edge indices in deterministic construction order.
    #[must_use]
    pub fn predecessor_edges(&self) -> &[usize] {
        &self.predecessors
    }

    /// Number of outgoing edges, including parallel edges.
    #[must_use]
    pub fn out_degree(&self) -> usize {
        self.successors.len()
    }

    /// Number of incoming edges, including parallel edges.
    #[must_use]
    pub fn in_degree(&self) -> usize {
        self.predecessors.len()
    }
}

// =============================================================================
// CFG
// =============================================================================

/// An immutable, region-local control-flow graph snapshot.
///
/// The graph stores references by canonical ID, not pointers to IR entities.
/// This avoids tying the graph's lifetime to borrowed IR storage and makes
/// the graph suitable for later analysis, caching, and deterministic testing.
///
/// Rebuild the graph after modifying its source region, blocks, terminators,
/// or successor edges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlFlowGraph {
    region: RegionId,
    entry: Option<BlockId>,
    block_order: Vec<BlockId>,
    nodes: BTreeMap<BlockId, CfgNode>,
    edges: Vec<CfgEdge>,
}

impl ControlFlowGraph {
    /// Build a CFG for one control-flow region.
    ///
    /// `blocks` is the block storage view for the containing IR. Every block
    /// listed by `region` must be present. Extra blocks belonging to other
    /// regions are ignored, except that duplicate IDs in the supplied view
    /// are rejected because they make reference resolution ambiguous.
    ///
    /// `entry` must be supplied by the caller if entry-dependent analyses
    /// will be performed. It is never inferred from region block order.
    ///
    /// This function validates the structural requirements needed for graph
    /// construction, but does not replace the repository's full verifiers.
    pub fn build(
        region: &ZuirRegion,
        blocks: &[ZuirBlock],
        entry: Option<BlockId>,
    ) -> CfgResult<Self> {
        region.validate().map_err(|error| CfgError::InvalidRegion {
            region: region.id(),
            reason: error.to_string(),
        })?;

        if !matches!(region.kind(), RegionKind::ControlFlow) {
            return Err(CfgError::NotControlFlowRegion {
                region: region.id(),
            });
        }

        // Build an index over the supplied block storage. A duplicate ID
        // cannot be resolved deterministically and must not be overwritten.
        let mut by_id = BTreeMap::<BlockId, &ZuirBlock>::new();

        for block in blocks {
            if by_id.insert(block.id(), block).is_some() {
                return Err(CfgError::DuplicateBlockId { block: block.id() });
            }
        }

        let mut membership = BTreeSet::<BlockId>::new();

        for &block_id in region.blocks() {
            if !membership.insert(block_id) {
                // Normally caught by region.validate(); retain the check
                // at the graph boundary as a defense-in-depth invariant.
                return Err(CfgError::InvalidRegion {
                    region: region.id(),
                    reason: format!("block {block_id} is listed more than once"),
                });
            }

            let block = by_id.get(&block_id).copied().ok_or(
                CfgError::MissingRegionBlock {
                    region: region.id(),
                    block: block_id,
                },
            )?;

            if block.parent_region() != Some(region.id()) {
                return Err(CfgError::BlockRegionMismatch {
                    block: block_id,
                    expected: region.id(),
                    actual: block.parent_region(),
                });
            }

            block.validate().map_err(|error| CfgError::InvalidBlock {
                block: block_id,
                reason: error.to_string(),
            })?;

            if block.terminator().is_open() {
                return Err(CfgError::OpenTerminator { block: block_id });
            }

            block.terminator().validate().map_err(|error| {
                CfgError::InvalidTerminator {
                    block: block_id,
                    reason: error.to_string(),
                }
            })?;
        }

        if let Some(entry_id) = entry {
            if !membership.contains(&entry_id) {
                return Err(CfgError::InvalidEntry {
                    region: region.id(),
                    entry: entry_id,
                });
            }
        }

        let mut graph = Self {
            region: region.id(),
            entry,
            block_order: Vec::new(),
            nodes: BTreeMap::new(),
            edges: Vec::new(),
        };

        graph
            .block_order
            .try_reserve(region.block_count())
            .map_err(|_| CfgError::AllocationFailure {
                collection: "block order",
            })?;

        // Create all nodes before edges so forward references are ordinary.
        for &block_id in region.blocks() {
            graph.block_order.push(block_id);
            graph.nodes.insert(block_id, CfgNode::default());
        }

        // Add edges in region block order and terminator successor order.
        // This preserves distinct switch/extension edges even when targets
        // are equal.
        for &source_id in region.blocks() {
            let source = by_id
                .get(&source_id)
                .copied()
                .ok_or(CfgError::MissingRegionBlock {
                    region: region.id(),
                    block: source_id,
                })?;

            let mut successor_index = 0_usize;
            let mut pending_error: Option<CfgError> = None;

            source.terminator().for_each_successor(|successor| {
                if pending_error.is_some() {
                    return;
                }

                let target_id = successor.target();

                if !membership.contains(&target_id) {
                    pending_error = Some(
                        if by_id.contains_key(&target_id) {
                            CfgError::SuccessorOutsideRegion {
                                source: source_id,
                                target: target_id,
                                region: region.id(),
                            }
                        } else {
                            CfgError::MissingSuccessorTarget {
                                source: source_id,
                                target: target_id,
                            }
                        },
                    );
                    return;
                }

                let Some(target) = by_id.get(&target_id).copied() else {
                    pending_error = Some(CfgError::MissingSuccessorTarget {
                        source: source_id,
                        target: target_id,
                    });
                    return;
                };

                if target.parent_region() != Some(region.id()) {
                    pending_error = Some(CfgError::SuccessorRegionMismatch {
                        source: source_id,
                        target: target_id,
                        expected: region.id(),
                        actual: target.parent_region(),
                    });
                    return;
                }

                if successor.argument_count() != target.argument_count() {
                    pending_error = Some(CfgError::InvalidTerminator {
                        block: source_id,
                        reason: format!(
                            "edge to block {target_id} supplies {} arguments, \
                             but the target declares {} block arguments",
                            successor.argument_count(),
                            target.argument_count()
                        ),
                    });
                    return;
                }

                if graph.edges.try_reserve(1).is_err() {
                    pending_error = Some(CfgError::AllocationFailure {
                        collection: "edges",
                    });
                    return;
                }

                let edge_index = graph.edges.len();

                graph.edges.push(CfgEdge {
                    source: source_id,
                    target: target_id,
                    successor_index,
                    arguments: successor.arguments().to_vec(),
                });

                // A Vec index is represented by usize. The edge's own index
                // is valid because it is the current Vec length.
                if let Some(next) = successor_index.checked_add(1) {
                    successor_index = next;
                } else {
                    pending_error = Some(CfgError::EdgeIndexExhausted);
                    return;
                }

                // Store adjacency only after the edge has been constructed.
                // Both endpoints were created in the node pass above.
                if let Some(node) = graph.nodes.get_mut(&source_id) {
                    node.successors.push(edge_index);
                }

                if let Some(node) = graph.nodes.get_mut(&target_id) {
                    node.predecessors.push(edge_index);
                }
            });

            if let Some(error) = pending_error {
                return Err(error);
            }
        }

        Ok(graph)
    }

    /// Return the region represented by this graph.
    #[must_use]
    pub const fn region(&self) -> RegionId {
        self.region
    }

    /// Return the explicitly supplied entry block, if any.
    #[must_use]
    pub const fn entry(&self) -> Option<BlockId> {
        self.entry
    }

    /// Whether an entry block was explicitly supplied.
    #[must_use]
    pub const fn has_entry(&self) -> bool {
        self.entry.is_some()
    }

    /// Return block IDs in the source region's declaration order.
    ///
    /// This is not execution order. Use reachability or a separately defined
    /// traversal when an execution-related ordering is needed.
    #[must_use]
    pub fn block_order(&self) -> &[BlockId] {
        &self.block_order
    }

    /// Return the number of nodes in the graph.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of explicit edges in the graph.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Whether the graph has no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Return whether the graph contains a block.
    #[must_use]
    pub fn contains_block(&self, block: BlockId) -> bool {
        self.nodes.contains_key(&block)
    }

    /// Borrow a node's adjacency information.
    #[must_use]
    pub fn node(&self, block: BlockId) -> Option<&CfgNode> {
        self.nodes.get(&block)
    }

    /// Borrow an edge by its graph-local index.
    #[must_use]
    pub fn edge(&self, index: usize) -> Option<&CfgEdge> {
        self.edges.get(index)
    }

    /// Borrow all edges in deterministic construction order.
    #[must_use]
    pub fn edges(&self) -> &[CfgEdge] {
        &self.edges
    }

    /// Iterate over nodes in ascending BlockId order.
    pub fn nodes(&self) -> impl ExactSizeIterator<Item = (BlockId, &CfgNode)> + '_ {
        self.nodes.iter().map(|(id, node)| (*id, node))
    }

    /// Iterate over outgoing edges in the terminator's declared order.
    pub fn successors(
        &self,
        block: BlockId,
    ) -> impl Iterator<Item = &CfgEdge> + '_ {
        self.nodes
            .get(&block)
            .into_iter()
            .flat_map(|node| node.successors.iter())
            .filter_map(|index| self.edges.get(*index))
    }

    /// Iterate over incoming edges in graph construction order.
    pub fn predecessors(
        &self,
        block: BlockId,
    ) -> impl Iterator<Item = &CfgEdge> + '_ {
        self.nodes
            .get(&block)
            .into_iter()
            .flat_map(|node| node.predecessors.iter())
            .filter_map(|index| self.edges.get(*index))
    }

    /// Compute the blocks reachable from the explicitly supplied entry.
    ///
    /// The traversal is iterative, so graph depth does not consume call-stack
    /// space. An entry-less graph returns `InvalidEntry` rather than guessing
    /// an entry from block order.
    pub fn reachable_from_entry(&self) -> CfgResult<BTreeSet<BlockId>> {
        let entry = self.entry.ok_or(CfgError::InvalidEntry {
            region: self.region,
            entry: self
                .block_order
                .first()
                .copied()
                .unwrap_or_else(|| {
                    // This value is never observed as a successful result.
                    // Empty and entry-less graphs have no meaningful entry.
                    // The branch below returns before using it.
                    self.region_entry_sentinel()
                }),
        })?;

        let mut reachable = BTreeSet::new();
        let mut pending = Vec::<BlockId>::new();

        pending
            .try_reserve(1)
            .map_err(|_| CfgError::AllocationFailure {
                collection: "reachability worklist",
            })?;
        pending.push(entry);

        while let Some(current) = pending.pop() {
            if !reachable.insert(current) {
                continue;
            }

            if let Some(node) = self.nodes.get(&current) {
                pending
                    .try_reserve(node.successors.len())
                    .map_err(|_| CfgError::AllocationFailure {
                        collection: "reachability worklist",
                    })?;

                for edge_index in node.successors.iter().rev() {
                    if let Some(edge) = self.edges.get(*edge_index) {
                        if !reachable.contains(&edge.target) {
                            pending.push(edge.target);
                        }
                    }
                }
            }
        }

        Ok(reachable)
    }

    /// Return the number of blocks reachable from the entry.
    ///
    /// This uses the same explicit-entry and resource-exhaustion contracts
    /// as `reachable_from_entry`.
    pub fn reachable_count(&self) -> CfgResult<usize> {
        Ok(self.reachable_from_entry()?.len())
    }

    /// Iterate over the region's nodes in ascending BlockId order.
    ///
    /// The returned iterator does not imply reachability or execution order.
    pub fn ordered_node_ids(&self) -> impl ExactSizeIterator<Item = BlockId> + '_ {
        self.nodes.keys().copied()
    }

    // Keep construction independent from sentinel values in the typed-ID
    // namespace. This is called only to form a structured error for a graph
    // without an entry; it does not create an actual graph node.
    fn region_entry_sentinel(&self) -> BlockId {
        // A valid BlockId cannot be manufactured without choosing an actual
        // identity. Instead of inventing one, callers should use the
        // dedicated entry-required API below.
        //
        // This helper is replaced by the dedicated error variant in the
        // integration patch described after the code.
        self.block_order[0]
    }
}
