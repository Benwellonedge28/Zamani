
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Complexity Analysis
//!
//! Path: `src/classical/zcir/analysis/complexity.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Analyze the structural size and declared abstract costs of a classical
//! computation without tying its meaning to a particular target machine.
//!
//! This module reports:
//! - Total and entry-reachable block and edge counts.
//! - Total and reachable static operation counts.
//! - Caller-supplied abstract work estimates.
//! - Caller-supplied static storage estimates.
//! - Whether the reachable CFG contains a cycle.
//! - A conservative acyclic work upper bound, when one can be established.
//!
//! ## Authority and ownership
//!
//! Canonical identity types belong to `crate::zuir::ids`.
//! Canonical blocks, operations, regions, operands, and terminators belong
//! to ZUIR. Canonical use-def indexing belongs to `crate::zuir::ir::use_def`.
//! The ZCIR control-flow and liveness modules own their respective analyses.
//!
//! This module owns only its input cost facts and derived complexity report.
//! It does not own or duplicate canonical IR, calculate semantic types,
//! perform liveness analysis, or modify the program.
//!
//! ## Cost model
//!
//! `AbstractCost::Exact(n)` is meaningful only in the caller's declared
//! cost unit. Values from different units MUST NOT be combined.
//!
//! `Unknown` means the cost is not established.
//! `Unbounded` means the caller knows no finite bound applies.
//! `Overflow` means exact arithmetic exceeded the representable `u128` range.
//!
//! These states are intentionally distinct. Unknown cost is not zero;
//! overflow is not an exact saturated result.
//!
//! Operation counts are structural counts supplied by the caller. This
//! module does not inspect operations and cannot verify those counts.
//!
//! `static_storage` represents caller-defined static storage cost, not peak
//! runtime memory. Runtime memory requires allocation, lifetime, alias,
//! escape, and execution-path information from other analyses.
//!
//! ## Control-flow interpretation
//!
//! For a nonempty problem, an entry block must be configured. Reachability
//! is computed iteratively. The reachable CFG is acyclic if and only if
//! Kahn's topological elimination removes every reachable block.
//!
//! In an acyclic reachable CFG, summing nonnegative per-block work costs
//! gives a conservative upper bound for one execution through the graph,
//! provided the caller's block costs correctly bound each block visit.
//! It may overestimate work because not every branch is taken.
//!
//! If a reachable cycle exists, runtime work is `Unknown`: cycles may be
//! bounded loops, unbounded loops, or paths whose bounds require additional
//! semantic facts. A cycle is not automatically proof of unbounded execution.
//!
//! ## Scalability
//!
//! There are no universal block, edge, operation, or hardware-count limits.
//! Graph traversal is iterative, not recursive. Counts use checked arithmetic.
//! Ordered collections provide deterministic results. Standard collection
//! allocation may still fail at process level; literal infinite capacity is
//! not promised.
//!
//! ## Invalidation
//!
//! The result is a snapshot. Recompute it when blocks, edges, operation
//! counts, or cost facts change.
//!
//! ## Integration
//!
//! 1. Register this module in `src/classical/zcir/analysis/mod.rs`.
//! 2. Build facts from verified canonical ZUIR operations and terminators.
//! 3. Use canonical `BlockId` values; do not invent local identity types.
//! 4. Reuse the verified CFG boundary where its public API is available.
//! 5. Supply cost estimates in one consistent, documented unit.
//! 6. Do not use complexity estimates alone to justify transformations,
//!    target feasibility, or resource allocation.
//!
//! Grammar and semantic specifications define the meaning of programs.
//! This analysis consumes resolved IR facts and never parses source syntax.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use crate::zuir::ids::BlockId;

/// Result type for complexity-analysis operations.
pub type ComplexityResult<T> = Result<T, ComplexityError>;

/// Errors detected while constructing or analyzing a complexity problem.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ComplexityError {
    /// A block was registered more than once.
    DuplicateBlock {
        block: BlockId,
    },

    /// A fact or edge refers to an unregistered source block.
    UnknownBlock {
        block: BlockId,
    },

    /// An edge targets a block that is not registered.
    UnknownSuccessor {
        source: BlockId,
        destination: BlockId,
    },

    /// A nonempty graph was analyzed without an entry block.
    MissingEntryBlock,

    /// A configured entry block is not registered.
    UnknownEntryBlock {
        block: BlockId,
    },

    /// The entry block was already configured.
    EntryBlockAlreadySet {
        existing: BlockId,
        attempted: BlockId,
    },

    /// A structural count could not be represented.
    CountOverflow,
}

impl fmt::Display for ComplexityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateBlock { block } => {
                write!(f, "duplicate complexity-analysis block {block:?}")
            }
            Self::UnknownBlock { block } => {
                write!(f, "unknown complexity-analysis block {block:?}")
            }
            Self::UnknownSuccessor {
                source,
                destination,
            } => write!(
                f,
                "complexity CFG edge from {source:?} targets unknown block \
                 {destination:?}"
            ),
            Self::MissingEntryBlock => {
                f.write_str("nonempty complexity graph requires an entry block")
            }
            Self::UnknownEntryBlock { block } => {
                write!(f, "complexity entry block {block:?} is not registered")
            }
            Self::EntryBlockAlreadySet {
                existing,
                attempted,
            } => write!(
                f,
                "complexity entry is already {existing:?}; cannot replace it \
                 with {attempted:?}"
            ),
            Self::CountOverflow => {
                f.write_str("structural complexity count exceeds u128")
            }
        }
    }
}

impl std::error::Error for ComplexityError {}

/// An abstract, non-negative cost estimate.
///
/// `Exact` describes an exact number in the caller-defined cost unit.
/// It does not imply that the estimate accurately models physical runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AbstractCost {
    /// A known finite cost.
    Exact(u128),

    /// The cost cannot currently be established.
    Unknown,

    /// The caller knows that no finite upper bound applies.
    Unbounded,

    /// Exact arithmetic exceeded the representable range.
    Overflow,
}

impl AbstractCost {
    /// Return an exact zero cost.
    #[must_use]
    pub const fn zero() -> Self {
        Self::Exact(0)
    }

    /// Add two costs without wrapping or silently saturating.
    ///
    /// Unbounded dominates because the sum is unbounded. Otherwise overflow
    /// dominates unknown, since the result is known to be unrepresentable.
    #[must_use]
    pub const fn checked_add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unbounded, _) | (_, Self::Unbounded) => Self::Unbounded,
            (Self::Overflow, _) | (_, Self::Overflow) => Self::Overflow,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            (Self::Exact(left), Self::Exact(right)) => {
                match left.checked_add(right) {
                    Some(value) => Self::Exact(value),
                    None => Self::Overflow,
                }
            }
        }
    }

    /// Multiply two costs without wrapping or silently saturating.
    #[must_use]
    pub const fn checked_mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::Exact(0), _) | (_, Self::Exact(0)) => Self::Exact(0),
            (Self::Unbounded, _) | (_, Self::Unbounded) => Self::Unbounded,
            (Self::Overflow, _) | (_, Self::Overflow) => Self::Overflow,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            (Self::Exact(left), Self::Exact(right)) => {
                match left.checked_mul(right) {
                    Some(value) => Self::Exact(value),
                    None => Self::Overflow,
                }
            }
        }
    }

    /// Return the exact value, if known and representable.
    #[must_use]
    pub const fn exact(self) -> Option<u128> {
        match self {
            Self::Exact(value) => Some(value),
            Self::Unknown | Self::Unbounded | Self::Overflow => None,
        }
    }

    /// Whether the cost is an exact finite value.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact(_))
    }
}

/// Caller-supplied complexity facts for one block.
///
/// Counts and estimates must refer to the same canonical block and remain
/// consistent with its verified operations. The analysis does not infer
/// instruction semantics from these facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockCost {
    operation_count: u128,
    abstract_work: AbstractCost,
    static_storage: AbstractCost,
}

impl BlockCost {
    /// Create a block cost from explicit facts.
    #[must_use]
    pub const fn new(
        operation_count: u128,
        abstract_work: AbstractCost,
        static_storage: AbstractCost,
    ) -> Self {
        Self {
            operation_count,
            abstract_work,
            static_storage,
        }
    }

    /// Create structural-only facts when operation costs are unavailable.
    #[must_use]
    pub const fn structural_only(operation_count: u128) -> Self {
        Self::new(
            operation_count,
            AbstractCost::Unknown,
            AbstractCost::Unknown,
        )
    }

    /// Return the supplied static operation count.
    #[must_use]
    pub const fn operation_count(self) -> u128 {
        self.operation_count
    }

    /// Return the supplied abstract work estimate.
    #[must_use]
    pub const fn abstract_work(self) -> AbstractCost {
        self.abstract_work
    }

    /// Return the supplied static storage estimate.
    #[must_use]
    pub const fn static_storage(self) -> AbstractCost {
        self.static_storage
    }
}

impl Default for BlockCost {
    fn default() -> Self {
        Self::structural_only(0)
    }
}

#[derive(Debug, Clone, Default)]
struct BlockFacts {
    cost: BlockCost,
    successors: BTreeSet<BlockId>,
}

/// Input facts for deterministic complexity analysis.
///
/// Edges may be added before their destination blocks are registered;
/// every endpoint is validated by `analyze`.
#[derive(Debug, Clone, Default)]
pub struct ComplexityProblem {
    blocks: BTreeMap<BlockId, BlockFacts>,
    entry: Option<BlockId>,
}

impl ComplexityProblem {
    /// Create an empty complexity problem.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a block with caller-supplied complexity facts.
    pub fn add_block(
        &mut self,
        block: BlockId,
        cost: BlockCost,
    ) -> ComplexityResult<()> {
        if self.blocks.contains_key(&block) {
            return Err(ComplexityError::DuplicateBlock { block });
        }

        self.blocks.insert(
            block,
            BlockFacts {
                cost,
                successors: BTreeSet::new(),
            },
        );
        Ok(())
    }

    /// Set the graph entry block.
    ///
    /// The entry must already be registered and can be set only once.
    pub fn set_entry(&mut self, block: BlockId) -> ComplexityResult<()> {
        if !self.blocks.contains_key(&block) {
            return Err(ComplexityError::UnknownEntryBlock { block });
        }

        if let Some(existing) = self.entry {
            return Err(ComplexityError::EntryBlockAlreadySet {
                existing,
                attempted: block,
            });
        }

        self.entry = Some(block);
        Ok(())
    }

    /// Add a directed control-flow edge.
    ///
    /// Duplicate edges are idempotent for this block-level analysis.
    pub fn add_edge(
        &mut self,
        source: BlockId,
        destination: BlockId,
    ) -> ComplexityResult<()> {
        let facts = self
            .blocks
            .get_mut(&source)
            .ok_or(ComplexityError::UnknownBlock { block: source })?;

        facts.successors.insert(destination);
        Ok(())
    }

    /// Return the number of registered blocks.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Whether this problem contains no blocks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Compute deterministic structural and cost analysis.
    ///
    /// The algorithm is iterative. A nonempty graph requires an entry block.
    /// Every successor endpoint is validated before reachability is computed.
    pub fn analyze(&self) -> ComplexityResult<ComplexityAnalysis> {
        if self.blocks.is_empty() {
            return Ok(ComplexityAnalysis::empty());
        }

        let entry = self.entry.ok_or(ComplexityError::MissingEntryBlock)?;

        // Validate all edges, including edges in unreachable blocks.
        let mut edge_count = 0_u128;
        let mut predecessors: BTreeMap<BlockId, BTreeSet<BlockId>> = self
            .blocks
            .keys()
            .copied()
            .map(|block| (block, BTreeSet::new()))
            .collect();

        for (&source, facts) in &self.blocks {
            for &destination in &facts.successors {
                let Some(destination_predecessors) =
                    predecessors.get_mut(&destination)
                else {
                    return Err(ComplexityError::UnknownSuccessor {
                        source,
                        destination,
                    });
                };

                destination_predecessors.insert(source);
                edge_count = edge_count
                    .checked_add(1)
                    .ok_or(ComplexityError::CountOverflow)?;
            }
        }

        // Find reachable blocks iteratively.
        let mut reachable = BTreeSet::new();
        let mut queue = VecDeque::new();

        reachable.insert(entry);
        queue.push_back(entry);

        while let Some(block) = queue.pop_front() {
            let facts = self
                .blocks
                .get(&block)
                .ok_or(ComplexityError::UnknownBlock { block })?;

            for &successor in &facts.successors {
                if reachable.insert(successor) {
                    queue.push_back(successor);
                }
            }
        }

        let mut all_operations = AbstractCost::zero();
        let mut reachable_operations = AbstractCost::zero();
        let mut reachable_work = AbstractCost::zero();
        let mut reachable_storage = AbstractCost::zero();

        for (&block, facts) in &self.blocks {
            let operations = AbstractCost::Exact(facts.cost.operation_count);
            all_operations = all_operations.checked_add(operations);

            if reachable.contains(&block) {
                reachable_operations =
                    reachable_operations.checked_add(operations);
                reachable_work =
                    reachable_work.checked_add(facts.cost.abstract_work);
                reachable_storage =
                    reachable_storage.checked_add(facts.cost.static_storage);
            }
        }

        // Kahn's algorithm on the reachable subgraph. If all reachable
        // blocks are removed, the reachable CFG is acyclic.
        let mut indegree: BTreeMap<BlockId, usize> = BTreeMap::new();

        for &block in &reachable {
            let degree = predecessors
                .get(&block)
                .ok_or(ComplexityError::UnknownBlock { block })?
                .iter()
                .filter(|predecessor| reachable.contains(predecessor))
                .count();

            indegree.insert(block, degree);
        }

        let mut ready: BTreeSet<BlockId> = indegree
            .iter()
            .filter_map(|(&block, &degree)| (degree == 0).then_some(block))
            .collect();

        let mut removed = 0_usize;

        while let Some(block) = ready.pop_first() {
            removed = removed
                .checked_add(1)
                .ok_or(ComplexityError::CountOverflow)?;

            let facts = self
                .blocks
                .get(&block)
                .ok_or(ComplexityError::UnknownBlock { block })?;

            for &successor in &facts.successors {
                if !reachable.contains(&successor) {
                    continue;
                }

                let degree = indegree
                    .get_mut(&successor)
                    .ok_or(ComplexityError::UnknownBlock {
                        block: successor,
                    })?;

                *degree = degree
                    .checked_sub(1)
                    .ok_or(ComplexityError::CountOverflow)?;

                if *degree == 0 {
                    ready.insert(successor);
                }
            }
        }

        let has_reachable_cycle = removed != reachable.len();

        // Acyclic control flow visits each reachable block at most once.
        // The sum is therefore an upper bound if each block's work estimate
        // itself is an upper bound. For cycles, loop bounds are not inferred.
        let acyclic_work_upper_bound = if has_reachable_cycle {
            AbstractCost::Unknown
        } else {
            reachable_work
        };

        Ok(ComplexityAnalysis {
            total_blocks: self.blocks.len(),
            total_edges: edge_count,
            reachable_blocks: reachable.len(),
            unreachable_blocks: self.blocks.len() - reachable.len(),
            all_static_operations: all_operations,
            reachable_static_operations: reachable_operations,
            reachable_one_visit_work: reachable_work,
            reachable_static_storage: reachable_storage,
            acyclic_work_upper_bound,
            has_reachable_cycle,
            entry: Some(entry),
        })
    }
}

/// Immutable result of a complexity analysis.
///
/// Structural counts are independent of the caller's abstract cost unit.
/// Work and storage values preserve uncertainty and arithmetic overflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexityAnalysis {
    total_blocks: usize,
    total_edges: u128,
    reachable_blocks: usize,
    unreachable_blocks: usize,
    all_static_operations: AbstractCost,
    reachable_static_operations: AbstractCost,
    reachable_one_visit_work: AbstractCost,
    reachable_static_storage: AbstractCost,
    acyclic_work_upper_bound: AbstractCost,
    has_reachable_cycle: bool,
    entry: Option<BlockId>,
}

impl ComplexityAnalysis {
    fn empty() -> Self {
        Self {
            total_blocks: 0,
            total_edges: 0,
            reachable_blocks: 0,
            unreachable_blocks: 0,
            all_static_operations: AbstractCost::zero(),
            reachable_static_operations: AbstractCost::zero(),
            reachable_one_visit_work: AbstractCost::zero(),
            reachable_static_storage: AbstractCost::zero(),
            acyclic_work_upper_bound: AbstractCost::zero(),
            has_reachable_cycle: false,
            entry: None,
        }
    }

    /// Total number of blocks, including unreachable blocks.
    #[must_use]
    pub const fn total_blocks(&self) -> usize {
        self.total_blocks
    }

    /// Total number of unique source/destination edges.
    ///
    /// This graph intentionally collapses parallel edges because they do not
    /// change block-level reachability or cycle detection.
    #[must_use]
    pub const fn total_edges(&self) -> u128 {
        self.total_edges
    }

    /// Number of blocks reachable from the configured entry.
    #[must_use]
    pub const fn reachable_blocks(&self) -> usize {
        self.reachable_blocks
    }

    /// Number of blocks unreachable from the configured entry.
    #[must_use]
    pub const fn unreachable_blocks(&self) -> usize {
        self.unreachable_blocks
    }

    /// Sum of static operation counts across all registered blocks.
    #[must_use]
    pub const fn all_static_operations(&self) -> AbstractCost {
        self.all_static_operations
    }

    /// Sum of static operation counts in reachable blocks.
    #[must_use]
    pub const fn reachable_static_operations(&self) -> AbstractCost {
        self.reachable_static_operations
    }

    /// Sum of work estimates for one visit to every reachable block.
    ///
    /// This is a structural aggregate, not necessarily the work of a real
    /// execution path.
    #[must_use]
    pub const fn reachable_one_visit_work(&self) -> AbstractCost {
        self.reachable_one_visit_work
    }

    /// Sum of caller-supplied static storage estimates for reachable blocks.
    ///
    /// This is not a runtime peak-memory estimate.
    #[must_use]
    pub const fn reachable_static_storage(&self) -> AbstractCost {
        self.reachable_static_storage
    }

    /// Acyclic upper bound on work, or `Unknown` when reachable cycles exist.
    #[must_use]
    pub const fn acyclic_work_upper_bound(&self) -> AbstractCost {
        self.acyclic_work_upper_bound
    }

    /// Whether a cycle exists in the entry-reachable CFG.
    #[must_use]
    pub const fn has_reachable_cycle(&self) -> bool {
        self.has_reachable_cycle
    }

    /// Return the configured entry block, if the graph is nonempty.
    #[must_use]
    pub const fn entry(&self) -> Option<BlockId> {
        self.entry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test IDs must be nonzero")
    }

    fn known(operations: u128, work: u128, storage: u128) -> BlockCost {
        BlockCost::new(
            operations,
            AbstractCost::Exact(work),
            AbstractCost::Exact(storage),
        )
    }

    #[test]
    fn empty_problem_has_zero_complexity() {
        let result = ComplexityProblem::new().analyze().unwrap();

        assert_eq!(result.total_blocks(), 0);
        assert_eq!(result.total_edges(), 0);
        assert_eq!(result.all_static_operations(), AbstractCost::Exact(0));
        assert_eq!(result.acyclic_work_upper_bound(), AbstractCost::Exact(0));
        assert!(!result.has_reachable_cycle());
    }

    #[test]
    fn nonempty_problem_requires_entry() {
        let mut problem = ComplexityProblem::new();
        problem.add_block(block(1), known(2, 3, 4)).unwrap();

        assert_eq!(
            problem.analyze(),
            Err(ComplexityError::MissingEntryBlock)
        );
    }

    #[test]
    fn duplicate_blocks_are_rejected() {
        let mut problem = ComplexityProblem::new();
        problem.add_block(block(1), known(1, 1, 1)).unwrap();

        assert_eq!(
            problem.add_block(block(1), known(2, 2, 2)),
            Err(ComplexityError::DuplicateBlock { block: block(1) })
        );
    }

    #[test]
    fn unknown_entry_is_rejected() {
        let mut problem = ComplexityProblem::new();

        assert_eq!(
            problem.set_entry(block(9)),
            Err(ComplexityError::UnknownEntryBlock { block: block(9) })
        );
    }

    #[test]
    fn unknown_successor_is_rejected() {
        let mut problem = ComplexityProblem::new();
        problem.add_block(block(1), known(1, 1, 1)).unwrap();
        problem.set_entry(block(1)).unwrap();
        problem.add_edge(block(1), block(2)).unwrap();

        assert!(matches!(
            problem.analyze(),
            Err(ComplexityError::UnknownSuccessor {
                source,
                destination
            }) if source == block(1) && destination == block(2)
        ));
    }

    #[test]
    fn acyclic_graph_reports_reachability_and_costs() {
        let mut problem = ComplexityProblem::new();
        problem.add_block(block(1), known(2, 5, 8)).unwrap();
        problem.add_block(block(2), known(3, 7, 4)).unwrap();
        problem.add_block(block(3), known(9, 20, 10)).unwrap();
        problem.set_entry(block(1)).unwrap();
        problem.add_edge(block(1), block(2)).unwrap();

        let result = problem.analyze().unwrap();

        assert_eq!(result.total_blocks(), 3);
        assert_eq!(result.total_edges(), 1);
        assert_eq!(result.reachable_blocks(), 2);
        assert_eq!(result.unreachable_blocks(), 1);
        assert_eq!(result.all_static_operations(), AbstractCost::Exact(14));
        assert_eq!(
            result.reachable_static_operations(),
            AbstractCost::Exact(5)
        );
        assert_eq!(
            result.reachable_one_visit_work(),
            AbstractCost::Exact(12)
        );
        assert_eq!(
            result.acyclic_work_upper_bound(),
            AbstractCost::Exact(12)
        );
        assert_eq!(
            result.reachable_static_storage(),
            AbstractCost::Exact(12)
        );
        assert!(!result.has_reachable_cycle());
    }

    #[test]
    fn reachable_cycle_makes_runtime_bound_unknown() {
        let mut problem = ComplexityProblem::new();
        problem.add_block(block(1), known(1, 2, 3)).unwrap();
        problem.add_block(block(2), known(1, 4, 5)).unwrap();
        problem.set_entry(block(1)).unwrap();
        problem.add_edge(block(1), block(2)).unwrap();
        problem.add_edge(block(2), block(1)).unwrap();

        let result = problem.analyze().unwrap();

        assert!(result.has_reachable_cycle());
        assert_eq!(
            result.acyclic_work_upper_bound(),
            AbstractCost::Unknown
        );
        assert_eq!(
            result.reachable_one_visit_work(),
            AbstractCost::Exact(6)
        );
    }

    #[test]
    fn unreachable_cycle_does_not_make_reachable_graph_cyclic() {
        let mut problem = ComplexityProblem::new();
        problem.add_block(block(1), known(1, 2, 3)).unwrap();
        problem.add_block(block(2), known(1, 4, 5)).unwrap();
        problem.add_block(block(3), known(1, 6, 7)).unwrap();
        problem.set_entry(block(1)).unwrap();
        problem.add_edge(block(2), block(3)).unwrap();
        problem.add_edge(block(3), block(2)).unwrap();

        let result = problem.analyze().unwrap();

        assert!(!result.has_reachable_cycle());
        assert_eq!(result.reachable_blocks(), 1);
        assert_eq!(result.unreachable_blocks(), 2);
        assert_eq!(
            result.acyclic_work_upper_bound(),
            AbstractCost::Exact(2)
        );
    }

    #[test]
    fn branch_merge_is_acyclic() {
        let mut problem = ComplexityProblem::new();
        for id in 1..=4 {
            problem
                .add_block(block(id), known(1, id, 1))
                .unwrap();
        }
        problem.set_entry(block(1)).unwrap();
        problem.add_edge(block(1), block(2)).unwrap();
        problem.add_edge(block(1), block(3)).unwrap();
        problem.add_edge(block(2), block(4)).unwrap();
        problem.add_edge(block(3), block(4)).unwrap();

        let result = problem.analyze().unwrap();

        assert!(!result.has_reachable_cycle());
        assert_eq!(result.reachable_blocks(), 4);
        // Summing every reachable block is conservative for either branch.
        assert_eq!(
            result.acyclic_work_upper_bound(),
            AbstractCost::Exact(10)
        );
    }

    #[test]
    fn self_loop_is_a_cycle() {
        let mut problem = ComplexityProblem::new();
        problem.add_block(block(1), known(1, 1, 1)).unwrap();
        problem.set_entry(block(1)).unwrap();
        problem.add_edge(block(1), block(1)).unwrap();

        assert!(problem.analyze().unwrap().has_reachable_cycle());
    }

    #[test]
    fn unknown_cost_does_not_become_zero() {
        let mut problem = ComplexityProblem::new();
        problem
            .add_block(block(1), BlockCost::structural_only(5))
            .unwrap();
        problem.set_entry(block(1)).unwrap();

        let result = problem.analyze().unwrap();

        assert_eq!(
            result.reachable_static_operations(),
            AbstractCost::Exact(5)
        );
        assert_eq!(
            result.reachable_one_visit_work(),
            AbstractCost::Unknown
        );
        assert_eq!(
            result.reachable_static_storage(),
            AbstractCost::Unknown
        );
    }

    #[test]
    fn abstract_cost_addition_reports_overflow() {
        assert_eq!(
            AbstractCost::Exact(u128::MAX).checked_add(AbstractCost::Exact(1)),
            AbstractCost::Overflow
        );
    }

    #[test]
    fn abstract_cost_multiplication_reports_overflow() {
        assert_eq!(
            AbstractCost::Exact(u128::MAX).checked_mul(AbstractCost::Exact(2)),
            AbstractCost::Overflow
        );
    }

    #[test]
    fn cost_states_remain_distinct() {
        assert_eq!(
            AbstractCost::Unknown.checked_add(AbstractCost::Exact(1)),
            AbstractCost::Unknown
        );
        assert_eq!(
            AbstractCost::Unbounded.checked_add(AbstractCost::Exact(1)),
            AbstractCost::Unbounded
        );
        assert_eq!(
            AbstractCost::Overflow.checked_add(AbstractCost::Exact(1)),
            AbstractCost::Overflow
        );
    }

    #[test]
    fn repeated_edge_is_idempotent() {
        let mut problem = ComplexityProblem::new();
        problem.add_block(block(1), known(1, 1, 1)).unwrap();
        problem.add_block(block(2), known(1, 1, 1)).unwrap();
        problem.set_entry(block(1)).unwrap();
        problem.add_edge(block(1), block(2)).unwrap();
        problem.add_edge(block(1), block(2)).unwrap();

        let result = problem.analyze().unwrap();

        assert_eq!(result.total_edges(), 1);
        assert_eq!(result.reachable_blocks(), 2);
    }

    #[test]
    fn insertion_order_does_not_change_analysis() {
        let mut left = ComplexityProblem::new();
        left.add_block(block(1), known(2, 4, 8)).unwrap();
        left.add_block(block(2), known(3, 5, 9)).unwrap();
        left.set_entry(block(1)).unwrap();
        left.add_edge(block(1), block(2)).unwrap();

        let mut right = ComplexityProblem::new();
        right.add_block(block(2), known(3, 5, 9)).unwrap();
        right.add_block(block(1), known(2, 4, 8)).unwrap();
        right.set_entry(block(1)).unwrap();
        right.add_edge(block(1), block(2)).unwrap();

        assert_eq!(left.analyze().unwrap(), right.analyze().unwrap());
    }

    #[test]
    fn entry_can_only_be_set_once() {
        let mut problem = ComplexityProblem::new();
        problem.add_block(block(1), known(0, 0, 0)).unwrap();
        problem.add_block(block(2), known(0, 0, 0)).unwrap();
        problem.set_entry(block(1)).unwrap();

        assert_eq!(
            problem.set_entry(block(2)),
            Err(ComplexityError::EntryBlockAlreadySet {
                existing: block(1),
                attempted: block(2),
            })
        );
    }
}
