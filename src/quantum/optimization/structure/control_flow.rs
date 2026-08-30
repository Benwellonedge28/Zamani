//! Zamani Quantum Optimization — Control-Flow Structure
//!
//! Production-grade control-flow representation for quantum optimization.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! quantum::frontend
//!      |
//!      v
//! quantum::ir::QuantumCircuit
//!      |
//!      v
//! quantum::optimization::structure
//!      |
//!      +-----------------------------+
//!      |                             |
//!      v                             v
//! structure::block             control_flow
//!      |                             |
//!      +-------------+---------------+
//!                    |
//!                    v
//!              optimization passes
//!                    |
//!                    v
//!               routing
//!                    |
//!                    v
//!               scheduling
//!                    |
//!                    v
//!                hardware
//! ```
//!
//! # Purpose
//!
//! This module represents **control-flow structure around quantum regions**.
//!
//! It does NOT define another quantum IR.
//!
//! The authoritative quantum representation remains:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! and the optimizer's canonical access layer remains:
//!
//! `crate::quantum::optimization::circuit::CircuitView`
//!
//! This module provides an optimizer-owned structural representation for:
//!
//! - straight-line regions;
//! - conditional branches;
//! - branch joins;
//! - loops;
//! - loop back-edges;
//! - loop exits;
//! - control-flow entry/exit boundaries;
//! - nested control flow;
//! - explicit control-flow edges;
//! - classical-dependency boundaries;
//! - optimization eligibility;
//! - conservative transformation barriers;
//! - deterministic traversal;
//! - structural validation;
//! - scalable graph construction.
//!
//! # Critical semantic rule
//!
//! A quantum circuit containing `Measure`, `Barrier`, or `Reset` does NOT,
//! by itself, contain a classical branch or loop.
//!
//! The canonical Quantum IR currently represents quantum operations and
//! non-unitary operations, while higher-level classical control-flow semantics
//! must be supplied explicitly by the compiler/frontend/structural layer.
//!
//! Consequently this module never guesses that:
//!
//! ```text
//! measure
//! gate
//! gate
//! ```
//!
//! means:
//!
//! ```text
//! if measurement == 1 {
//!     gate
//!     gate
//! }
//! ```
//!
//! Such an interpretation would be unsound.
//!
//! Instead, the frontend/lowering layer, `structure::conditional`, or
//! `structure::loop` supplies explicit `ControlFlowEdge`/`ControlFlowNode`
//! information.
//!
//! # Integration contract
//!
//! This module is intentionally independent of:
//!
//! - `OptimizationPipeline`;
//! - `OptimizationPass`;
//! - `OptimizationContext`;
//! - routing;
//! - scheduling;
//! - hardware;
//! - QPU execution;
//! - benchmarking;
//! - QEC implementation;
//! - a particular optimization algorithm.
//!
//! It consumes stable structural concepts from:
//!
//! - `optimization::circuit`;
//! - `optimization::structure::block`;
//! - `quantum::ir`.
//!
//! Future modules consume this module:
//!
//! - `structure::region.rs`;
//! - `structure::conditional.rs`;
//! - `structure::loop.rs`;
//! - `analysis::dependency.rs`;
//! - `analysis::liveness.rs`;
//! - `analysis::commutation.rs`;
//! - `passes::optimize_depth.rs`;
//! - `passes::optimize_width.rs`;
//! - `planner.rs`;
//! - `pipeline.rs`;
//! - verification modules.
//!
//! No later optimizer module should need to modify the fundamental control-flow
//! graph representation merely because a new optimization pass is added.
//!
//! # Design principles
//!
//! 1. No duplicate quantum IR.
//! 2. No unsafe code.
//! 3. No global mutable state.
//! 4. No hidden classical-control assumptions.
//! 5. Conservative semantics by default.
//! 6. Explicit branch and loop structure.
//! 7. Deterministic node and edge ordering.
//! 8. Invocation-local IDs.
//! 9. Checked arithmetic.
//! 10. Bounded graph growth.
//! 11. Immutable graph after construction.
//! 12. Validation before optimizer consumption.
//! 13. Support for arbitrarily large workloads subject to available resources.
//! 14. No recursion required for ordinary graph construction/traversal.
//! 15. Explicit support for nested control flow.
//!
//! # Scaling
//!
//! There is no artificial "maximum circuit size" in this module.
//!
//! Actual scalability is bounded by:
//!
//! - available memory;
//! - available CPU;
//! - canonical Quantum IR limits;
//! - optimizer limits;
//! - the number of control-flow nodes/edges supplied by the caller.
//!
//! All size arithmetic is checked.
//!
//! The graph uses contiguous `Vec` storage and invocation-local integer IDs.
//! This gives predictable memory behavior and avoids hash-map overhead in the
//! common deterministic traversal path.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe`.
//!
//! # Example
//!
//! ```ignore
//! let graph = ControlFlowGraph::builder()
//!     .add_node(ControlFlowNode::entry())
//!     .add_node(ControlFlowNode::straight_line(block))
//!     .add_node(ControlFlowNode::conditional_branch(branch))
//!     .add_node(ControlFlowNode::merge())
//!     .add_edge(ControlFlowEdge::normal(entry, body))
//!     .add_edge(ControlFlowEdge::branch(branch, then_node, condition))
//!     .add_edge(ControlFlowEdge::branch(branch, else_node, condition))
//!     .add_edge(ControlFlowEdge::join(then_node, merge))
//!     .add_edge(ControlFlowEdge::join(else_node, merge))
//!     .finish()?;
//!
//! graph.validate()?;
//! ```
//!
//! The exact frontend/control-flow lowering representation is deliberately
//! outside this module.

use std::fmt;
use std::ops::Range;

use super::block::{
    BlockDescriptor,
    BlockId,
    BlockKind,
};

use crate::quantum::optimization::circuit::{
    OperationId,
    RegionId,
};

// ============================================================================
// Public result type
// ============================================================================

/// Result type for control-flow structure operations.
pub type ControlFlowResult<T> = Result<T, ControlFlowError>;

// ============================================================================
// IDs
// ============================================================================

/// Invocation-local identifier for a control-flow node.
///
/// A node ID is never a globally persistent program identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControlFlowNodeId(usize);

impl ControlFlowNodeId {
    /// Creates a node ID from an invocation-local index.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the invocation-local index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for ControlFlowNodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "cf_node{}", self.0)
    }
}

/// Invocation-local identifier for a control-flow edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControlFlowEdgeId(usize);

impl ControlFlowEdgeId {
    /// Creates an edge ID.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the invocation-local index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for ControlFlowEdgeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "cf_edge{}", self.0)
    }
}

// ============================================================================
// Control-flow kinds
// ============================================================================

/// Semantic kind of a control-flow node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlFlowNodeKind {
    /// Entry into the complete control-flow graph.
    Entry,

    /// Exit from the complete control-flow graph.
    Exit,

    /// A normal straight-line quantum region.
    StraightLine,

    /// A region representing the body of a conditional branch.
    ConditionalBranch,

    /// A branch-selection point.
    ConditionalDispatch,

    /// A merge point after conditional branches.
    ConditionalMerge,

    /// A loop header.
    LoopHeader,

    /// A loop body.
    LoopBody,

    /// A loop condition check.
    LoopCondition,

    /// A loop exit.
    LoopExit,

    /// A nested control-flow region.
    ControlFlowRegion,

    /// A semantic boundary that cannot be crossed by ordinary rewrites.
    Barrier,

    /// An explicit user/compiler-defined control-flow region.
    UserDefined,

    /// Unknown structure.
    Unknown,
}

impl ControlFlowNodeKind {
    /// Returns true when the node is an entry node.
    #[must_use]
    pub const fn is_entry(self) -> bool {
        matches!(self, Self::Entry)
    }

    /// Returns true when the node is an exit node.
    #[must_use]
    pub const fn is_exit(self) -> bool {
        matches!(self, Self::Exit)
    }

    /// Returns true when the node represents a branch.
    #[must_use]
    pub const fn is_branch(self) -> bool {
        matches!(
            self,
            Self::ConditionalBranch | Self::ConditionalDispatch
        )
    }

    /// Returns true when the node represents a merge.
    #[must_use]
    pub const fn is_merge(self) -> bool {
        matches!(self, Self::ConditionalMerge)
    }

    /// Returns true when the node represents a loop.
    #[must_use]
    pub const fn is_loop(self) -> bool {
        matches!(
            self,
            Self::LoopHeader
                | Self::LoopBody
                | Self::LoopCondition
                | Self::LoopExit
        )
    }

    /// Returns true when ordinary local transformations must not cross this
    /// node without an explicit semantic proof.
    #[must_use]
    pub const fn is_transformation_boundary(self) -> bool {
        matches!(
            self,
            Self::Entry
                | Self::Exit
                | Self::ConditionalDispatch
                | Self::ConditionalMerge
                | Self::LoopHeader
                | Self::LoopCondition
                | Self::LoopExit
                | Self::Barrier
                | Self::ControlFlowRegion
                | Self::Unknown
        )
    }
}

// ============================================================================
// Edge kinds
// ============================================================================

/// Semantic kind of a control-flow edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlFlowEdgeKind {
    /// Ordinary sequential flow.
    Normal,

    /// True/selected branch.
    BranchTrue,

    /// False/unselected branch.
    BranchFalse,

    /// Multi-way branch selected by a condition.
    BranchCase,

    /// Flow joining after branches.
    Join,

    /// Loop entry.
    LoopEntry,

    /// Loop back-edge.
    LoopBack,

    /// Loop exit.
    LoopExit,

    /// Exceptional/abnormal compiler-level control flow.
    Abnormal,

    /// Explicit user/compiler-defined edge.
    UserDefined,

    /// Unknown edge semantics.
    Unknown,
}

impl ControlFlowEdgeKind {
    /// Returns whether the edge is a branch edge.
    #[must_use]
    pub const fn is_branch(self) -> bool {
        matches!(
            self,
            Self::BranchTrue
                | Self::BranchFalse
                | Self::BranchCase
        )
    }

    /// Returns whether the edge is a loop back-edge.
    #[must_use]
    pub const fn is_loop_back(self) -> bool {
        matches!(self, Self::LoopBack)
    }

    /// Returns whether the edge joins control-flow paths.
    #[must_use]
    pub const fn is_join(self) -> bool {
        matches!(self, Self::Join)
    }

    /// Returns whether traversing this edge changes the iteration of a loop.
    #[must_use]
    pub const fn is_loop_related(self) -> bool {
        matches!(
            self,
            Self::LoopEntry
                | Self::LoopBack
                | Self::LoopExit
        )
    }
}

// ============================================================================
// Conditions
// ============================================================================

/// A reference to classical information controlling a quantum branch.
///
/// This module deliberately does not define the complete classical expression
/// language. It only provides stable optimizer metadata for dependency tracking.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ControlCondition {
    /// A single classical bit must be one.
    BitSet {
        /// Classical-bit index in the owning IR/program namespace.
        bit: usize,
    },

    /// A single classical bit must be zero.
    BitClear {
        /// Classical-bit index.
        bit: usize,
    },

    /// A classical value must equal a specified unsigned integer.
    RegisterEquals {
        /// Classical register identifier/name supplied by the frontend.
        register: String,

        /// Required value.
        value: u64,
    },

    /// A classical value must not equal a specified unsigned integer.
    RegisterNotEquals {
        /// Classical register identifier/name.
        register: String,

        /// Forbidden value.
        value: u64,
    },

    /// An opaque compiler-defined condition.
    ///
    /// The optimizer must treat this conservatively unless another component
    /// proves equivalence.
    Opaque {
        /// Stable invocation-local condition identifier.
        id: u64,
    },

    /// Logical conjunction.
    All(Vec<ControlCondition>),

    /// Logical disjunction.
    Any(Vec<ControlCondition>),

    /// Logical negation.
    Not(Box<ControlCondition>),

    /// Unconditional flow.
    Always,
}

impl ControlCondition {
    /// Returns an unconditional condition.
    #[must_use]
    pub const fn always() -> Self {
        Self::Always
    }

    /// Returns whether this condition is unconditionally true.
    #[must_use]
    pub const fn is_always(&self) -> bool {
        matches!(self, Self::Always)
    }

    /// Returns the number of leaf conditions.
    ///
    /// This method is iterative to avoid stack growth for deeply nested
    /// expressions.
    #[must_use]
    pub fn complexity(&self) -> usize {
        let mut count = 0usize;
        let mut stack = vec![self];

        while let Some(condition) = stack.pop() {
            match condition {
                Self::All(values) | Self::Any(values) => {
                    for value in values {
                        stack.push(value);
                    }
                }

                Self::Not(value) => {
                    stack.push(value);
                }

                _ => {
                    count = count.saturating_add(1);
                }
            }
        }

        count
    }

    /// Returns whether the condition references the supplied classical bit.
    ///
    /// Traversal is iterative.
    #[must_use]
    pub fn references_bit(&self, bit: usize) -> bool {
        let mut stack = vec![self];

        while let Some(condition) = stack.pop() {
            match condition {
                Self::BitSet { bit: value }
                | Self::BitClear { bit: value } => {
                    if *value == bit {
                        return true;
                    }
                }

                Self::All(values) | Self::Any(values) => {
                    for value in values {
                        stack.push(value);
                    }
                }

                Self::Not(value) => {
                    stack.push(value);
                }

                _ => {}
            }
        }

        false
    }

    /// Returns a conservative set of classical register names referenced by
    /// this condition.
    ///
    /// The returned vector is deterministic and duplicate-free.
    #[must_use]
    pub fn referenced_registers(&self) -> Vec<&str> {
        let mut result = Vec::new();
        let mut stack = vec![self];

        while let Some(condition) = stack.pop() {
            match condition {
                Self::RegisterEquals { register, .. }
                | Self::RegisterNotEquals { register, .. } => {
                    if !result.iter().any(|existing| *existing == register) {
                        result.push(register.as_str());
                    }
                }

                Self::All(values) | Self::Any(values) => {
                    for value in values {
                        stack.push(value);
                    }
                }

                Self::Not(value) => {
                    stack.push(value);
                }

                _ => {}
            }
        }

        result
    }
}

// ============================================================================
// Node metadata
// ============================================================================

/// Metadata describing which optimizer region/block a control-flow node owns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ControlFlowSource {
    /// Optional optimizer block.
    block: Option<BlockId>,

    /// Optional optimizer region.
    region: Option<RegionId>,

    /// Optional first operation.
    first_operation: Option<OperationId>,

    /// Optional exclusive operation end.
    end_operation: Option<OperationId>,
}

impl ControlFlowSource {
    /// Creates an empty source descriptor.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            block: None,
            region: None,
            first_operation: None,
            end_operation: None,
        }
    }

    /// Creates a source descriptor for a block.
    #[must_use]
    pub const fn from_block(block: BlockId) -> Self {
        Self {
            block: Some(block),
            region: None,
            first_operation: None,
            end_operation: None,
        }
    }

    /// Associates a region.
    #[must_use]
    pub const fn with_region(
        mut self,
        region: RegionId,
    ) -> Self {
        self.region = Some(region);
        self
    }

    /// Associates the first operation.
    #[must_use]
    pub const fn with_first_operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.first_operation = Some(operation);
        self
    }

    /// Associates the exclusive final operation.
    #[must_use]
    pub const fn with_end_operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.end_operation = Some(operation);
        self
    }

    /// Returns the block.
    #[must_use]
    pub const fn block(self) -> Option<BlockId> {
        self.block
    }

    /// Returns the region.
    #[must_use]
    pub const fn region(self) -> Option<RegionId> {
        self.region
    }

    /// Returns the first operation.
    #[must_use]
    pub const fn first_operation(self) -> Option<OperationId> {
        self.first_operation
    }

    /// Returns the exclusive end operation.
    #[must_use]
    pub const fn end_operation(self) -> Option<OperationId> {
        self.end_operation
    }
}

// ============================================================================
// Nodes
// ============================================================================

/// Immutable control-flow node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlFlowNode {
    id: ControlFlowNodeId,
    kind: ControlFlowNodeKind,
    source: ControlFlowSource,
    condition: Option<ControlCondition>,
    loop_id: Option<u64>,
    depth: usize,
    optimization_barrier: bool,
}

impl ControlFlowNode {
    /// Creates a node with no source metadata.
    #[must_use]
    pub fn new(kind: ControlFlowNodeKind) -> Self {
        Self {
            id: ControlFlowNodeId::new(0),
            kind,
            source: ControlFlowSource::empty(),
            condition: None,
            loop_id: None,
            depth: 0,
            optimization_barrier: kind.is_transformation_boundary(),
        }
    }

    /// Creates an entry node.
    #[must_use]
    pub fn entry() -> Self {
        Self::new(ControlFlowNodeKind::Entry)
    }

    /// Creates an exit node.
    #[must_use]
    pub fn exit() -> Self {
        Self::new(ControlFlowNodeKind::Exit)
    }

    /// Creates a straight-line node from an optimizer block.
    #[must_use]
    pub fn straight_line(block: BlockDescriptor) -> Self {
        let mut node = Self::new(ControlFlowNodeKind::StraightLine);
        node.source = ControlFlowSource::from_block(block.id());
        node.optimization_barrier = !block.is_optimizable()
            || !block.kind().permits_unitary_local_rewrites();
        node
    }

    /// Creates a conditional dispatch node.
    #[must_use]
    pub fn conditional_dispatch(
        condition: ControlCondition,
    ) -> Self {
        let mut node =
            Self::new(ControlFlowNodeKind::ConditionalDispatch);
        node.condition = Some(condition);
        node
    }

    /// Creates a conditional branch node.
    #[must_use]
    pub fn conditional_branch(
        condition: ControlCondition,
    ) -> Self {
        let mut node =
            Self::new(ControlFlowNodeKind::ConditionalBranch);
        node.condition = Some(condition);
        node
    }

    /// Creates a conditional merge node.
    #[must_use]
    pub fn conditional_merge() -> Self {
        Self::new(ControlFlowNodeKind::ConditionalMerge)
    }

    /// Creates a loop header.
    #[must_use]
    pub fn loop_header(loop_id: u64) -> Self {
        let mut node =
            Self::new(ControlFlowNodeKind::LoopHeader);
        node.loop_id = Some(loop_id);
        node
    }

    /// Creates a loop body.
    #[must_use]
    pub fn loop_body(loop_id: u64) -> Self {
        let mut node =
            Self::new(ControlFlowNodeKind::LoopBody);
        node.loop_id = Some(loop_id);
        node
    }

    /// Creates a loop condition.
    #[must_use]
    pub fn loop_condition(
        loop_id: u64,
        condition: ControlCondition,
    ) -> Self {
        let mut node =
            Self::new(ControlFlowNodeKind::LoopCondition);
        node.loop_id = Some(loop_id);
        node.condition = Some(condition);
        node
    }

    /// Creates a loop exit.
    #[must_use]
    pub fn loop_exit(loop_id: u64) -> Self {
        let mut node =
            Self::new(ControlFlowNodeKind::LoopExit);
        node.loop_id = Some(loop_id);
        node
    }

    /// Assigns the invocation-local node ID.
    #[must_use]
    pub const fn with_id(
        mut self,
        id: ControlFlowNodeId,
    ) -> Self {
        self.id = id;
        self
    }

    /// Associates source metadata.
    #[must_use]
    pub const fn with_source(
        mut self,
        source: ControlFlowSource,
    ) -> Self {
        self.source = source;
        self
    }

    /// Associates a nesting depth.
    #[must_use]
    pub const fn with_depth(
        mut self,
        depth: usize,
    ) -> Self {
        self.depth = depth;
        self
    }

    /// Marks/unmarks the node as an optimization barrier.
    ///
    /// Unmarking is only a structural declaration. Transformation passes still
    /// need to prove semantic legality.
    #[must_use]
    pub const fn with_optimization_barrier(
        mut self,
        barrier: bool,
    ) -> Self {
        self.optimization_barrier = barrier;
        self
    }

    /// Returns the node ID.
    #[must_use]
    pub const fn id(&self) -> ControlFlowNodeId {
        self.id
    }

    /// Returns the node kind.
    #[must_use]
    pub const fn kind(&self) -> ControlFlowNodeKind {
        self.kind
    }

    /// Returns source metadata.
    #[must_use]
    pub const fn source(&self) -> ControlFlowSource {
        self.source
    }

    /// Returns the optional condition.
    #[must_use]
    pub fn condition(&self) -> Option<&ControlCondition> {
        self.condition.as_ref()
    }

    /// Returns the optional loop identifier.
    #[must_use]
    pub const fn loop_id(&self) -> Option<u64> {
        self.loop_id
    }

    /// Returns nesting depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Returns whether this node is an optimization barrier.
    #[must_use]
    pub const fn is_optimization_barrier(&self) -> bool {
        self.optimization_barrier
    }
}

// ============================================================================
// Edges
// ============================================================================

/// Immutable control-flow edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlFlowEdge {
    id: ControlFlowEdgeId,
    from: ControlFlowNodeId,
    to: ControlFlowNodeId,
    kind: ControlFlowEdgeKind,
    condition: Option<ControlCondition>,
    loop_id: Option<u64>,
}

impl ControlFlowEdge {
    /// Creates an unconditional normal edge.
    #[must_use]
    pub const fn normal(
        from: ControlFlowNodeId,
        to: ControlFlowNodeId,
    ) -> Self {
        Self {
            id: ControlFlowEdgeId::new(0),
            from,
            to,
            kind: ControlFlowEdgeKind::Normal,
            condition: None,
            loop_id: None,
        }
    }

    /// Creates a true branch edge.
    #[must_use]
    pub fn branch_true(
        from: ControlFlowNodeId,
        to: ControlFlowNodeId,
        condition: ControlCondition,
    ) -> Self {
        Self {
            id: ControlFlowEdgeId::new(0),
            from,
            to,
            kind: ControlFlowEdgeKind::BranchTrue,
            condition: Some(condition),
            loop_id: None,
        }
    }

    /// Creates a false branch edge.
    #[must_use]
    pub fn branch_false(
        from: ControlFlowNodeId,
        to: ControlFlowNodeId,
        condition: ControlCondition,
    ) -> Self {
        Self {
            id: ControlFlowEdgeId::new(0),
            from,
            to,
            kind: ControlFlowEdgeKind::BranchFalse,
            condition: Some(condition),
            loop_id: None,
        }
    }

    /// Creates a join edge.
    #[must_use]
    pub const fn join(
        from: ControlFlowNodeId,
        to: ControlFlowNodeId,
    ) -> Self {
        Self {
            id: ControlFlowEdgeId::new(0),
            from,
            to,
            kind: ControlFlowEdgeKind::Join,
            condition: None,
            loop_id: None,
        }
    }

    /// Creates a loop-entry edge.
    #[must_use]
    pub fn loop_entry(
        from: ControlFlowNodeId,
        to: ControlFlowNodeId,
        loop_id: u64,
    ) -> Self {
        Self {
            id: ControlFlowEdgeId::new(0),
            from,
            to,
            kind: ControlFlowEdgeKind::LoopEntry,
            condition: None,
            loop_id: Some(loop_id),
        }
    }

    /// Creates a loop back-edge.
    #[must_use]
    pub fn loop_back(
        from: ControlFlowNodeId,
        to: ControlFlowNodeId,
        loop_id: u64,
    ) -> Self {
        Self {
            id: ControlFlowEdgeId::new(0),
            from,
            to,
            kind: ControlFlowEdgeKind::LoopBack,
            condition: None,
            loop_id: Some(loop_id),
        }
    }

    /// Creates a loop-exit edge.
    #[must_use]
    pub fn loop_exit(
        from: ControlFlowNodeId,
        to: ControlFlowNodeId,
        loop_id: u64,
    ) -> Self {
        Self {
            id: ControlFlowEdgeId::new(0),
            from,
            to,
            kind: ControlFlowEdgeKind::LoopExit,
            condition: None,
            loop_id: Some(loop_id),
        }
    }

    /// Creates an arbitrary edge.
    #[must_use]
    pub fn new(
        from: ControlFlowNodeId,
        to: ControlFlowNodeId,
        kind: ControlFlowEdgeKind,
    ) -> Self {
        Self {
            id: ControlFlowEdgeId::new(0),
            from,
            to,
            kind,
            condition: None,
            loop_id: None,
        }
    }

    /// Associates an edge condition.
    #[must_use]
    pub fn with_condition(
        mut self,
        condition: ControlCondition,
    ) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Associates a loop identifier.
    #[must_use]
    pub const fn with_loop_id(
        mut self,
        loop_id: u64,
    ) -> Self {
        self.loop_id = Some(loop_id);
        self
    }

    /// Assigns the invocation-local edge ID.
    #[must_use]
    pub const fn with_id(
        mut self,
        id: ControlFlowEdgeId,
    ) -> Self {
        self.id = id;
        self
    }

    /// Returns the edge ID.
    #[must_use]
    pub const fn id(&self) -> ControlFlowEdgeId {
        self.id
    }

    /// Returns the source node.
    #[must_use]
    pub const fn from(&self) -> ControlFlowNodeId {
        self.from
    }

    /// Returns the destination node.
    #[must_use]
    pub const fn to(&self) -> ControlFlowNodeId {
        self.to
    }

    /// Returns the edge kind.
    #[must_use]
    pub const fn kind(&self) -> ControlFlowEdgeKind {
        self.kind
    }

    /// Returns the optional condition.
    #[must_use]
    pub fn condition(&self) -> Option<&ControlCondition> {
        self.condition.as_ref()
    }

    /// Returns the optional loop identifier.
    #[must_use]
    pub const fn loop_id(&self) -> Option<u64> {
        self.loop_id
    }
}

// ============================================================================
// Graph errors
// ============================================================================

/// Errors produced by control-flow graph construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlFlowError {
    /// The graph contains no entry node.
    MissingEntry,

    /// The graph contains more than one entry node.
    MultipleEntries,

    /// The graph contains no exit node.
    MissingExit,

    /// The graph contains more than one exit node.
    MultipleExits,

    /// A node ID referenced by an edge does not exist.
    UnknownNode {
        /// Missing node.
        node: ControlFlowNodeId,
    },

    /// A node was added twice with an incompatible identity.
    DuplicateNode {
        /// Duplicated node.
        node: ControlFlowNodeId,
    },

    /// An edge was added twice with the same identity.
    DuplicateEdge {
        /// Duplicated edge.
        edge: ControlFlowEdgeId,
    },

    /// An edge points to itself where self edges are not permitted.
    SelfEdge {
        /// Edge involved.
        edge: ControlFlowEdgeId,
    },

    /// A required branch has no branch condition.
    MissingBranchCondition {
        /// Node involved.
        node: ControlFlowNodeId,
    },

    /// A loop-related edge has no loop identifier.
    MissingLoopId {
        /// Edge involved.
        edge: ControlFlowEdgeId,
    },

    /// A loop back-edge targets a node that is not a loop header/body/condition.
    InvalidLoopTarget {
        /// Edge involved.
        edge: ControlFlowEdgeId,
    },

    /// A branch node has an invalid successor configuration.
    InvalidBranchStructure {
        /// Node involved.
        node: ControlFlowNodeId,

        /// Number of outgoing edges.
        outgoing: usize,
    },

    /// A merge node has too few incoming paths.
    InvalidMergeStructure {
        /// Node involved.
        node: ControlFlowNodeId,

        /// Number of incoming edges.
        incoming: usize,
    },

    /// The graph contains unreachable nodes.
    UnreachableNode {
        /// Unreachable node.
        node: ControlFlowNodeId,
    },

    /// A loop back-edge does not participate in a structurally valid loop.
    InvalidLoopStructure {
        /// Loop identifier.
        loop_id: u64,
    },

    /// A node depth is inconsistent with its graph structure.
    InvalidDepth {
        /// Node involved.
        node: ControlFlowNodeId,
    },

    /// A source operation range is invalid.
    InvalidOperationRange {
        /// Start operation.
        start: usize,

        /// End operation.
        end: usize,
    },

    /// Control-flow graph size arithmetic overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// The graph exceeded an explicit construction limit.
    ResourceLimitExceeded {
        /// Resource name.
        resource: &'static str,

        /// Requested amount.
        requested: usize,

        /// Maximum amount.
        maximum: usize,
    },

    /// The builder was consumed before completion.
    BuilderConsumed,

    /// An operation requires a node that has not yet been registered.
    MissingNodeForOperation {
        /// Operation identifier.
        operation: OperationId,
    },
}

impl fmt::Display for ControlFlowError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MissingEntry => {
                formatter.write_str(
                    "control-flow graph has no entry node",
                )
            }

            Self::MultipleEntries => {
                formatter.write_str(
                    "control-flow graph has multiple entry nodes",
                )
            }

            Self::MissingExit => {
                formatter.write_str(
                    "control-flow graph has no exit node",
                )
            }

            Self::MultipleExits => {
                formatter.write_str(
                    "control-flow graph has multiple exit nodes",
                )
            }

            Self::UnknownNode { node } => {
                write!(
                    formatter,
                    "control-flow edge references unknown node {node}"
                )
            }

            Self::DuplicateNode { node } => {
                write!(
                    formatter,
                    "control-flow node {node} was registered more than once"
                )
            }

            Self::DuplicateEdge { edge } => {
                write!(
                    formatter,
                    "control-flow edge {edge} was registered more than once"
                )
            }

            Self::SelfEdge { edge } => {
                write!(
                    formatter,
                    "control-flow edge {edge} is a forbidden self-edge"
                )
            }

            Self::MissingBranchCondition { node } => {
                write!(
                    formatter,
                    "branch node {node} has no condition"
                )
            }

            Self::MissingLoopId { edge } => {
                write!(
                    formatter,
                    "loop edge {edge} has no loop identifier"
                )
            }

            Self::InvalidLoopTarget { edge } => {
                write!(
                    formatter,
                    "loop back-edge {edge} has an invalid target"
                )
            }

            Self::InvalidBranchStructure {
                node,
                outgoing,
            } => {
                write!(
                    formatter,
                    "branch node {node} has invalid outgoing-edge count {outgoing}"
                )
            }

            Self::InvalidMergeStructure {
                node,
                incoming,
            } => {
                write!(
                    formatter,
                    "merge node {node} has invalid incoming-edge count {incoming}"
                )
            }

            Self::UnreachableNode { node } => {
                write!(
                    formatter,
                    "control-flow node {node} is unreachable"
                )
            }

            Self::InvalidLoopStructure { loop_id } => {
                write!(
                    formatter,
                    "loop {loop_id} has invalid control-flow structure"
                )
            }

            Self::InvalidDepth { node } => {
                write!(
                    formatter,
                    "control-flow node {node} has invalid nesting depth"
                )
            }

            Self::InvalidOperationRange { start, end } => {
                write!(
                    formatter,
                    "invalid operation range {start}..{end}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "control-flow {resource} limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::BuilderConsumed => {
                formatter.write_str(
                    "control-flow graph builder has already been consumed",
                )
            }

            Self::MissingNodeForOperation { operation } => {
                write!(
                    formatter,
                    "no control-flow node is associated with operation {operation}"
                )
            }
        }
    }
}

impl std::error::Error for ControlFlowError {}

// ============================================================================
// Graph
// ============================================================================

/// Immutable, validated control-flow graph.
///
/// The graph owns structural metadata only. It does not own quantum gates.
#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    nodes: Vec<ControlFlowNode>,
    edges: Vec<ControlFlowEdge>,
    outgoing: Vec<Vec<ControlFlowEdgeId>>,
    incoming: Vec<Vec<ControlFlowEdgeId>>,
    entry: ControlFlowNodeId,
    exit: ControlFlowNodeId,
}

impl ControlFlowGraph {
    /// Starts a graph builder.
    #[must_use]
    pub fn builder() -> ControlFlowGraphBuilder {
        ControlFlowGraphBuilder::new()
    }

    /// Creates a graph from already validated vectors.
    ///
    /// This remains private to the builder path so that adjacency indexes
    /// cannot become inconsistent.
    fn new(
        nodes: Vec<ControlFlowNode>,
        edges: Vec<ControlFlowEdge>,
        entry: ControlFlowNodeId,
        exit: ControlFlowNodeId,
    ) -> ControlFlowResult<Self> {
        let outgoing = build_adjacency(
            nodes.len(),
            &edges,
            true,
        )?;

        let incoming = build_adjacency(
            nodes.len(),
            &edges,
            false,
        )?;

        let graph = Self {
            nodes,
            edges,
            outgoing,
            incoming,
            entry,
            exit,
        };

        graph.validate()?;

        Ok(graph)
    }

    /// Returns the number of nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns the entry node.
    #[must_use]
    pub const fn entry(&self) -> ControlFlowNodeId {
        self.entry
    }

    /// Returns the exit node.
    #[must_use]
    pub const fn exit(&self) -> ControlFlowNodeId {
        self.exit
    }

    /// Returns a node.
    #[must_use]
    pub fn node(
        &self,
        id: ControlFlowNodeId,
    ) -> Option<&ControlFlowNode> {
        self.nodes.get(id.index())
    }

    /// Returns an edge.
    #[must_use]
    pub fn edge(
        &self,
        id: ControlFlowEdgeId,
    ) -> Option<&ControlFlowEdge> {
        self.edges.get(id.index())
    }

    /// Returns all nodes in deterministic ID order.
    #[must_use]
    pub fn nodes(&self) -> &[ControlFlowNode] {
        &self.nodes
    }

    /// Returns all edges in deterministic ID order.
    #[must_use]
    pub fn edges(&self) -> &[ControlFlowEdge] {
        &self.edges
    }

    /// Returns outgoing edge IDs.
    #[must_use]
    pub fn outgoing_edges(
        &self,
        node: ControlFlowNodeId,
    ) -> Option<&[ControlFlowEdgeId]> {
        self.outgoing
            .get(node.index())
            .map(Vec::as_slice)
    }

    /// Returns incoming edge IDs.
    #[must_use]
    pub fn incoming_edges(
        &self,
        node: ControlFlowNodeId,
    ) -> Option<&[ControlFlowEdgeId]> {
        self.incoming
            .get(node.index())
            .map(Vec::as_slice)
    }

    /// Returns outgoing successor nodes.
    pub fn successors(
        &self,
        node: ControlFlowNodeId,
    ) -> Vec<ControlFlowNodeId> {
        self.outgoing
            .get(node.index())
            .map(|edges| {
                edges
                    .iter()
                    .filter_map(|edge_id| {
                        self.edge(*edge_id).map(
                            |edge| edge.to(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns incoming predecessor nodes.
    pub fn predecessors(
        &self,
        node: ControlFlowNodeId,
    ) -> Vec<ControlFlowNodeId> {
        self.incoming
            .get(node.index())
            .map(|edges| {
                edges
                    .iter()
                    .filter_map(|edge_id| {
                        self.edge(*edge_id).map(
                            |edge| edge.from(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns whether a node is a transformation barrier.
    #[must_use]
    pub fn is_transformation_barrier(
        &self,
        node: ControlFlowNodeId,
    ) -> bool {
        self.node(node)
            .map(ControlFlowNode::is_optimization_barrier)
            .unwrap_or(true)
    }

    /// Returns all branch nodes.
    pub fn branch_nodes(
        &self,
    ) -> impl Iterator<Item = &ControlFlowNode> {
        self.nodes
            .iter()
            .filter(|node| node.kind().is_branch())
    }

    /// Returns all loop nodes.
    pub fn loop_nodes(
        &self,
    ) -> impl Iterator<Item = &ControlFlowNode> {
        self.nodes
            .iter()
            .filter(|node| node.kind().is_loop())
    }

    /// Returns all merge nodes.
    pub fn merge_nodes(
        &self,
    ) -> impl Iterator<Item = &ControlFlowNode> {
        self.nodes
            .iter()
            .filter(|node| node.kind().is_merge())
    }

    /// Returns nodes associated with an optimizer block.
    pub fn nodes_for_block(
        &self,
        block: BlockId,
    ) -> impl Iterator<Item = &ControlFlowNode> {
        self.nodes.iter().filter(move |node| {
            node.source().block() == Some(block)
        })
    }

    /// Returns nodes associated with an operation.
    pub fn node_for_operation(
        &self,
        operation: OperationId,
    ) -> Option<&ControlFlowNode> {
        self.nodes.iter().find(|node| {
            let source = node.source();

            match (
                source.first_operation(),
                source.end_operation(),
            ) {
                (Some(start), Some(end)) => {
                    operation.index() >= start.index()
                        && operation.index() < end.index()
                }

                (Some(start), None) => {
                    operation.index() == start.index()
                }

                _ => false,
            }
        })
    }

    /// Performs deterministic breadth-first traversal.
    ///
    /// No recursion is used.
    pub fn reachable_from_entry(
        &self,
    ) -> Vec<ControlFlowNodeId> {
        let mut visited =
            vec![false; self.nodes.len()];
        let mut queue = Vec::new();
        let mut result = Vec::new();

        visited[self.entry.index()] = true;
        queue.push(self.entry);

        let mut cursor = 0usize;

        while cursor < queue.len() {
            let node = queue[cursor];
            cursor += 1;

            result.push(node);

            if let Some(edges) =
                self.outgoing.get(node.index())
            {
                for edge_id in edges {
                    if let Some(edge) = self.edge(*edge_id) {
                        let target = edge.to();

                        if !visited[target.index()] {
                            visited[target.index()] = true;
                            queue.push(target);
                        }
                    }
                }
            }
        }

        result
    }

    /// Returns true when every graph node is reachable from entry.
    #[must_use]
    pub fn is_fully_reachable(&self) -> bool {
        self.reachable_from_entry().len()
            == self.nodes.len()
    }

    /// Validates the complete graph.
    pub fn validate(&self) -> ControlFlowResult<()> {
        if self.nodes.is_empty() {
            return Err(ControlFlowError::MissingEntry);
        }

        let entries = self
            .nodes
            .iter()
            .filter(|node| node.kind().is_entry())
            .count();

        if entries == 0 {
            return Err(ControlFlowError::MissingEntry);
        }

        if entries > 1 {
            return Err(ControlFlowError::MultipleEntries);
        }

        let exits = self
            .nodes
            .iter()
            .filter(|node| node.kind().is_exit())
            .count();

        if exits == 0 {
            return Err(ControlFlowError::MissingExit);
        }

        if exits > 1 {
            return Err(ControlFlowError::MultipleExits);
        }

        for node in &self.nodes {
            if node.id().index() >= self.nodes.len() {
                return Err(
                    ControlFlowError::UnknownNode {
                        node: node.id(),
                    },
                );
            }

            if let (
                Some(start),
                Some(end),
            ) = (
                node.source().first_operation(),
                node.source().end_operation(),
            ) {
                if start.index() > end.index() {
                    return Err(
                        ControlFlowError::InvalidOperationRange {
                            start: start.index(),
                            end: end.index(),
                        },
                    );
                }
            }

            if node.kind().is_branch()
                && node.condition().is_none()
            {
                return Err(
                    ControlFlowError::MissingBranchCondition {
                        node: node.id(),
                    },
                );
            }
        }

        for edge in &self.edges {
            let from = edge.from();
            let to = edge.to();

            if from.index() >= self.nodes.len() {
                return Err(
                    ControlFlowError::UnknownNode {
                        node: from,
                    },
                );
            }

            if to.index() >= self.nodes.len() {
                return Err(
                    ControlFlowError::UnknownNode {
                        node: to,
                    },
                );
            }

            if from == to
                && !edge.kind().is_loop_back()
            {
                return Err(
                    ControlFlowError::SelfEdge {
                        edge: edge.id(),
                    },
                );
            }

            if edge.kind().is_loop_related()
                && edge.loop_id().is_none()
            {
                return Err(
                    ControlFlowError::MissingLoopId {
                        edge: edge.id(),
                    },
                );
            }

            if edge.kind().is_loop_back() {
                let target =
                    &self.nodes[to.index()];

                if !target.kind().is_loop() {
                    return Err(
                        ControlFlowError::InvalidLoopTarget {
                            edge: edge.id(),
                        },
                    );
                }
            }
        }

        for node in &self.nodes {
            let outgoing = self
                .outgoing_edges(node.id())
                .map_or(0, |edges| edges.len());

            let incoming = self
                .incoming_edges(node.id())
                .map_or(0, |edges| edges.len());

            match node.kind() {
                ControlFlowNodeKind::Entry => {
                    if incoming != 0 {
                        return Err(
                            ControlFlowError::InvalidBranchStructure {
                                node: node.id(),
                                outgoing,
                            },
                        );
                    }

                    if outgoing == 0 {
                        return Err(
                            ControlFlowError::InvalidBranchStructure {
                                node: node.id(),
                                outgoing,
                            },
                        );
                    }
                }

                ControlFlowNodeKind::Exit => {
                    if outgoing != 0 {
                        return Err(
                            ControlFlowError::InvalidBranchStructure {
                                node: node.id(),
                                outgoing,
                            },
                        );
                    }

                    if incoming == 0 {
                        return Err(
                            ControlFlowError::InvalidMergeStructure {
                                node: node.id(),
                                incoming,
                            },
                        );
                    }
                }

                ControlFlowNodeKind::ConditionalDispatch => {
                    if outgoing < 2 {
                        return Err(
                            ControlFlowError::InvalidBranchStructure {
                                node: node.id(),
                                outgoing,
                            },
                        );
                    }
                }

                ControlFlowNodeKind::ConditionalMerge => {
                    if incoming < 2 {
                        return Err(
                            ControlFlowError::InvalidMergeStructure {
                                node: node.id(),
                                incoming,
                            },
                        );
                    }
                }

                _ => {}
            }
        }

        if !self.is_fully_reachable() {
            let mut reachable =
                vec![false; self.nodes.len()];

            for node in self.reachable_from_entry() {
                reachable[node.index()] = true;
            }

            for node in &self.nodes {
                if !reachable[node.id().index()] {
                    return Err(
                        ControlFlowError::UnreachableNode {
                            node: node.id(),
                        },
                    );
                }
            }
        }

        validate_loop_structure(self)?;

        Ok(())
    }

    /// Returns the graph's structural depth.
    ///
    /// This is the maximum node nesting depth.
    #[must_use]
    pub fn max_depth(&self) -> usize {
        self.nodes
            .iter()
            .map(ControlFlowNode::depth)
            .max()
            .unwrap_or(0)
    }

    /// Returns the number of loop back-edges.
    #[must_use]
    pub fn loop_back_edge_count(&self) -> usize {
        self.edges
            .iter()
            .filter(|edge| edge.kind().is_loop_back())
            .count()
    }

    /// Returns whether an operation is inside a transformation barrier.
    ///
    /// This is deliberately conservative: unknown control-flow structure is
    /// treated as protected.
    #[must_use]
    pub fn operation_is_protected(
        &self,
        operation: OperationId,
    ) -> bool {
        self.node_for_operation(operation)
            .map_or(true, |node| {
                node.is_optimization_barrier()
            })
    }
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for [`ControlFlowGraph`].
///
/// The builder assigns deterministic IDs based on insertion order.
#[derive(Debug, Default)]
pub struct ControlFlowGraphBuilder {
    nodes: Vec<ControlFlowNode>,
    edges: Vec<ControlFlowEdge>,
    max_nodes: Option<usize>,
    max_edges: Option<usize>,
    consumed: bool,
}

impl ControlFlowGraphBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            max_nodes: None,
            max_edges: None,
            consumed: false,
        }
    }

    /// Sets the maximum number of nodes.
    ///
    /// This is an optional optimizer-local safety limit. No limit is applied
    /// unless explicitly supplied.
    pub fn with_max_nodes(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_nodes = Some(maximum);
        self
    }

    /// Sets the maximum number of edges.
    pub fn with_max_edges(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_edges = Some(maximum);
        self
    }

    /// Adds a node and returns its assigned ID.
    pub fn add_node(
        &mut self,
        node: ControlFlowNode,
    ) -> ControlFlowResult<ControlFlowNodeId> {
        self.ensure_not_consumed()?;

        if let Some(maximum) = self.max_nodes {
            let requested =
                self.nodes
                    .len()
                    .checked_add(1)
                    .ok_or(
                        ControlFlowError::ArithmeticOverflow {
                            calculation: "control-flow node count",
                        },
                    )?;

            if requested > maximum {
                return Err(
                    ControlFlowError::ResourceLimitExceeded {
                        resource: "nodes",
                        requested,
                        maximum,
                    },
                );
            }
        }

        let id =
            ControlFlowNodeId::new(self.nodes.len());

        self.nodes.push(node.with_id(id));

        Ok(id)
    }

    /// Adds an edge and returns its assigned ID.
    pub fn add_edge(
        &mut self,
        edge: ControlFlowEdge,
    ) -> ControlFlowResult<ControlFlowEdgeId> {
        self.ensure_not_consumed()?;

        if let Some(maximum) = self.max_edges {
            let requested =
                self.edges
                    .len()
                    .checked_add(1)
                    .ok_or(
                        ControlFlowError::ArithmeticOverflow {
                            calculation: "control-flow edge count",
                        },
                    )?;

            if requested > maximum {
                return Err(
                    ControlFlowError::ResourceLimitExceeded {
                        resource: "edges",
                        requested,
                        maximum,
                    },
                );
            }
        }

        let id =
            ControlFlowEdgeId::new(self.edges.len());

        self.edges.push(edge.with_id(id));

        Ok(id)
    }

    /// Adds a complete block node.
    pub fn add_block(
        &mut self,
        block: BlockDescriptor,
    ) -> ControlFlowResult<ControlFlowNodeId> {
        self.add_node(
            ControlFlowNode::straight_line(block),
        )
    }

    /// Finishes the graph.
    pub fn finish(
        &mut self,
    ) -> ControlFlowResult<ControlFlowGraph> {
        self.ensure_not_consumed()?;
        self.consumed = true;

        let entry = find_unique_entry(&self.nodes)?;
        let exit = find_unique_exit(&self.nodes)?;

        ControlFlowGraph::new(
            std::mem::take(&mut self.nodes),
            std::mem::take(&mut self.edges),
            entry,
            exit,
        )
    }

    fn ensure_not_consumed(
        &self,
    ) -> ControlFlowResult<()> {
        if self.consumed {
            Err(ControlFlowError::BuilderConsumed)
        } else {
            Ok(())
        }
    }
}

// ============================================================================
// Adjacency
// ============================================================================

fn build_adjacency(
    node_count: usize,
    edges: &[ControlFlowEdge],
    outgoing: bool,
) -> ControlFlowResult<Vec<Vec<ControlFlowEdgeId>>> {
    let mut adjacency =
        vec![Vec::<ControlFlowEdgeId>::new(); node_count];

    for edge in edges {
        let index = if outgoing {
            edge.from().index()
        } else {
            edge.to().index()
        };

        if index >= node_count {
            return Err(
                ControlFlowError::UnknownNode {
                    node: if outgoing {
                        edge.from()
                    } else {
                        edge.to()
                    },
                },
            );
        }

        adjacency[index].push(edge.id());
    }

    Ok(adjacency)
}

// ============================================================================
// Entry / exit helpers
// ============================================================================

fn find_unique_entry(
    nodes: &[ControlFlowNode],
) -> ControlFlowResult<ControlFlowNodeId> {
    let mut found = None;

    for node in nodes {
        if node.kind().is_entry() {
            if found.is_some() {
                return Err(
                    ControlFlowError::MultipleEntries,
                );
            }

            found = Some(node.id());
        }
    }

    found.ok_or(ControlFlowError::MissingEntry)
}

fn find_unique_exit(
    nodes: &[ControlFlowNode],
) -> ControlFlowResult<ControlFlowNodeId> {
    let mut found = None;

    for node in nodes {
        if node.kind().is_exit() {
            if found.is_some() {
                return Err(
                    ControlFlowError::MultipleExits,
                );
            }

            found = Some(node.id());
        }
    }

    found.ok_or(ControlFlowError::MissingExit)
}

// ============================================================================
// Loop validation
// ============================================================================

fn validate_loop_structure(
    graph: &ControlFlowGraph,
) -> ControlFlowResult<()> {
    for edge in graph.edges() {
        if !edge.kind().is_loop_back() {
            continue;
        }

        let loop_id = edge
            .loop_id()
            .ok_or(
                ControlFlowError::MissingLoopId {
                    edge: edge.id(),
                },
            )?;

        let source =
            graph.node(edge.from()).ok_or(
                ControlFlowError::UnknownNode {
                    node: edge.from(),
                },
            )?;

        let target =
            graph.node(edge.to()).ok_or(
                ControlFlowError::UnknownNode {
                    node: edge.to(),
                },
            )?;

        if source.loop_id() != Some(loop_id)
            || target.loop_id() != Some(loop_id)
        {
            return Err(
                ControlFlowError::InvalidLoopStructure {
                    loop_id,
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// Block-to-control-flow helpers
// ============================================================================

/// Creates a conservative straight-line control-flow graph from a sequence of
/// optimizer blocks.
///
/// This helper is useful before first-class frontend control flow is lowered.
/// Each block becomes a node and blocks are connected sequentially.
///
/// The caller must provide at least one block.
pub fn graph_from_blocks(
    blocks: &[BlockDescriptor],
) -> ControlFlowResult<ControlFlowGraph> {
    if blocks.is_empty() {
        return Err(ControlFlowError::MissingEntry);
    }

    let mut builder =
        ControlFlowGraphBuilder::new();

    let entry =
        builder.add_node(ControlFlowNode::entry())?;

    let mut previous = entry;

    for block in blocks {
        let node = builder.add_block(block.clone())?;

        builder.add_edge(
            ControlFlowEdge::normal(
                previous,
                node,
            ),
        )?;

        previous = node;
    }

    let exit =
        builder.add_node(ControlFlowNode::exit())?;

    builder.add_edge(
        ControlFlowEdge::normal(
            previous,
            exit,
        ),
    )?;

    builder.finish()
}

// ============================================================================
// Operation protection
// ============================================================================

/// Describes why an operation is protected from ordinary movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationProtection {
    /// No control-flow protection was identified.
    None,

    /// Operation is inside a conditional branch.
    Conditional,

    /// Operation is inside a loop.
    Loop,

    /// Operation belongs to a control-flow boundary.
    ControlFlowBoundary,

    /// Operation belongs to an explicit optimizer barrier.
    Barrier,

    /// Control-flow structure is unknown, therefore transformation is
    /// conservatively forbidden.
    Unknown,
}

impl OperationProtection {
    /// Returns whether ordinary local movement should be rejected.
    #[must_use]
    pub const fn forbids_unproven_movement(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Classifies the control-flow protection around an operation.
#[must_use]
pub fn operation_protection(
    graph: &ControlFlowGraph,
    operation: OperationId,
) -> OperationProtection {
    let Some(node) =
        graph.node_for_operation(operation)
    else {
        return OperationProtection::Unknown;
    };

    if node.kind()
        == ControlFlowNodeKind::Barrier
    {
        return OperationProtection::Barrier;
    }

    if node.is_optimization_barrier() {
        return OperationProtection::ControlFlowBoundary;
    }

    if node.kind().is_branch() {
        return OperationProtection::Conditional;
    }

    if node.kind().is_loop() {
        return OperationProtection::Loop;
    }

    match node.kind() {
        ControlFlowNodeKind::Unknown
        | ControlFlowNodeKind::ControlFlowRegion => {
            OperationProtection::Unknown
        }

        _ => OperationProtection::None,
    }
}

// ============================================================================
// Structural queries
// ============================================================================

/// Returns whether two nodes can be treated as being in the same straight-line
/// optimization region.
///
/// This is deliberately conservative.
#[must_use]
pub fn same_optimization_region(
    graph: &ControlFlowGraph,
    first: ControlFlowNodeId,
    second: ControlFlowNodeId,
) -> bool {
    let Some(a) = graph.node(first) else {
        return false;
    };

    let Some(b) = graph.node(second) else {
        return false;
    };

    if a.is_optimization_barrier()
        || b.is_optimization_barrier()
    {
        return false;
    }

    if a.depth() != b.depth() {
        return false;
    }

    match (a.source().region(), b.source().region()) {
        (Some(left), Some(right)) => left == right,

        (None, None) => {
            matches!(
                (
                    a.kind(),
                    b.kind()
                ),
                (
                    ControlFlowNodeKind::StraightLine,
                    ControlFlowNodeKind::StraightLine
                )
            )
        }

        _ => false,
    }
}

/// Returns whether a node lies on a control-flow boundary.
#[must_use]
pub fn is_control_flow_boundary(
    graph: &ControlFlowGraph,
    node: ControlFlowNodeId,
) -> bool {
    graph
        .node(node)
        .map_or(true, |value| {
            value.kind()
                .is_transformation_boundary()
        })
}

// ============================================================================
// Range validation helper
// ============================================================================

/// Validates a half-open operation range.
pub fn validate_operation_range(
    range: &Range<usize>,
) -> ControlFlowResult<()> {
    if range.start > range.end {
        return Err(
            ControlFlowError::InvalidOperationRange {
                start: range.start,
                end: range.end,
            },
        );
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_graph() -> ControlFlowGraph {
        let mut builder =
            ControlFlowGraphBuilder::new();

        let entry =
            builder
                .add_node(ControlFlowNode::entry())
                .expect("entry");

        let body =
            builder
                .add_node(
                    ControlFlowNode::new(
                        ControlFlowNodeKind::StraightLine,
                    ),
                )
                .expect("body");

        let exit =
            builder
                .add_node(ControlFlowNode::exit())
                .expect("exit");

        builder
            .add_edge(
                ControlFlowEdge::normal(
                    entry,
                    body,
                ),
            )
            .expect("entry edge");

        builder
            .add_edge(
                ControlFlowEdge::normal(
                    body,
                    exit,
                ),
            )
            .expect("exit edge");

        builder.finish().expect("valid graph")
    }

    #[test]
    fn simple_graph_is_valid() {
        let graph = simple_graph();

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
        assert_eq!(graph.entry().index(), 0);
        assert_eq!(graph.exit().index(), 2);
        assert!(graph.is_fully_reachable());
    }

    #[test]
    fn branch_graph_is_valid() {
        let mut builder =
            ControlFlowGraphBuilder::new();

        let entry =
            builder
                .add_node(ControlFlowNode::entry())
                .expect("entry");

        let dispatch =
            builder
                .add_node(
                    ControlFlowNode::conditional_dispatch(
                        ControlCondition::BitSet {
                            bit: 0,
                        },
                    ),
                )
                .expect("dispatch");

        let then_node =
            builder
                .add_node(
                    ControlFlowNode::conditional_branch(
                        ControlCondition::BitSet {
                            bit: 0,
                        },
                    ),
                )
                .expect("then");

        let else_node =
            builder
                .add_node(
                    ControlFlowNode::conditional_branch(
                        ControlCondition::BitClear {
                            bit: 0,
                        },
                    ),
                )
                .expect("else");

        let merge =
            builder
                .add_node(
                    ControlFlowNode::conditional_merge(),
                )
                .expect("merge");

        let exit =
            builder
                .add_node(ControlFlowNode::exit())
                .expect("exit");

        builder
            .add_edge(
                ControlFlowEdge::normal(
                    entry,
                    dispatch,
                ),
            )
            .expect("entry");

        builder
            .add_edge(
                ControlFlowEdge::branch_true(
                    dispatch,
                    then_node,
                    ControlCondition::BitSet {
                        bit: 0,
                    },
                ),
            )
            .expect("true");

        builder
            .add_edge(
                ControlFlowEdge::branch_false(
                    dispatch,
                    else_node,
                    ControlCondition::BitClear {
                        bit: 0,
                    },
                ),
            )
            .expect("false");

        builder
            .add_edge(
                ControlFlowEdge::join(
                    then_node,
                    merge,
                ),
            )
            .expect("then join");

        builder
            .add_edge(
                ControlFlowEdge::join(
                    else_node,
                    merge,
                ),
            )
            .expect("else join");

        builder
            .add_edge(
                ControlFlowEdge::normal(
                    merge,
                    exit,
                ),
            )
            .expect("exit");

        let graph =
            builder.finish().expect("valid branch");

        assert_eq!(
            graph.branch_nodes().count(),
            3
        );

        assert_eq!(
            graph.merge_nodes().count(),
            1
        );

        assert!(graph.is_fully_reachable());
    }

    #[test]
    fn loop_graph_is_valid() {
        let mut builder =
            ControlFlowGraphBuilder::new();

        let entry =
            builder
                .add_node(ControlFlowNode::entry())
                .expect("entry");

        let header =
            builder
                .add_node(
                    ControlFlowNode::loop_header(1),
                )
                .expect("header");

        let body =
            builder
                .add_node(
                    ControlFlowNode::loop_body(1),
                )
                .expect("body");

        let condition =
            builder
                .add_node(
                    ControlFlowNode::loop_condition(
                        1,
                        ControlCondition::BitSet {
                            bit: 0,
                        },
                    ),
                )
                .expect("condition");

        let exit_loop =
            builder
                .add_node(
                    ControlFlowNode::loop_exit(1),
                )
                .expect("loop exit");

        let exit =
            builder
                .add_node(ControlFlowNode::exit())
                .expect("exit");

        builder
            .add_edge(
                ControlFlowEdge::normal(
                    entry,
                    header,
                ),
            )
            .expect("entry");

        builder
            .add_edge(
                ControlFlowEdge::loop_entry(
                    header,
                    body,
                    1,
                ),
            )
            .expect("loop entry");

        builder
            .add_edge(
                ControlFlowEdge::normal(
                    body,
                    condition,
                ),
            )
            .expect("condition");

        builder
            .add_edge(
                ControlFlowEdge::loop_back(
                    condition,
                    header,
                    1,
                ),
            )
            .expect("back");

        builder
            .add_edge(
                ControlFlowEdge::loop_exit(
                    condition,
                    exit_loop,
                    1,
                ),
            )
            .expect("loop exit");

        builder
            .add_edge(
                ControlFlowEdge::normal(
                    exit_loop,
                    exit,
                ),
            )
            .expect("exit");

        let graph =
            builder.finish().expect("valid loop");

        assert_eq!(
            graph.loop_back_edge_count(),
            1
        );

        assert!(graph.is_fully_reachable());
    }

    #[test]
    fn condition_complexity_is_iterative() {
        let condition =
            ControlCondition::All(vec![
                ControlCondition::BitSet { bit: 0 },
                ControlCondition::Any(vec![
                    ControlCondition::BitClear { bit: 1 },
                    ControlCondition::Not(
                        Box::new(
                            ControlCondition::BitSet {
                                bit: 2,
                            },
                        ),
                    ),
                ]),
            ]);

        assert_eq!(
            condition.complexity(),
            4
        );

        assert!(condition.references_bit(2));
        assert!(!condition.references_bit(99));
    }

    #[test]
    fn block_graph_is_sequential() {
        let blocks = vec![
            BlockDescriptor::new(
                BlockId::new(0),
                0..2,
                BlockKind::StraightLine,
            )
            .expect("block 0"),
            BlockDescriptor::new(
                BlockId::new(1),
                2..4,
                BlockKind::StraightLine,
            )
            .expect("block 1"),
        ];

        let graph =
            graph_from_blocks(&blocks)
                .expect("graph");

        assert_eq!(
            graph.node_count(),
            4
        );

        assert_eq!(
            graph.edge_count(),
            3
        );
    }

    #[test]
    fn invalid_branch_without_condition_is_rejected() {
        let mut builder =
            ControlFlowGraphBuilder::new();

        builder
            .add_node(ControlFlowNode::entry())
            .expect("entry");

        builder
            .add_node(
                ControlFlowNode::new(
                    ControlFlowNodeKind::ConditionalDispatch,
                ),
            )
            .expect("dispatch");

        builder
            .add_node(ControlFlowNode::exit())
            .expect("exit");

        assert!(matches!(
            builder.finish(),
            Err(
                ControlFlowError::MissingBranchCondition {
                    ..
                }
            )
        ));
    }

    #[test]
    fn operation_protection_is_conservative() {
        let mut builder =
            ControlFlowGraphBuilder::new();

        let entry =
            builder
                .add_node(ControlFlowNode::entry())
                .expect("entry");

        let body =
            builder
                .add_node(
                    ControlFlowNode::new(
                        ControlFlowNodeKind::Unknown,
                    )
                    .with_source(
                        ControlFlowSource::empty()
                            .with_first_operation(
                                OperationId::new(0),
                            )
                            .with_end_operation(
                                OperationId::new(1),
                            ),
                    ),
                )
                .expect("body");

        let exit =
            builder
                .add_node(ControlFlowNode::exit())
                .expect("exit");

        builder
            .add_edge(
                ControlFlowEdge::normal(
                    entry,
                    body,
                ),
            )
            .expect("entry");

        builder
            .add_edge(
                ControlFlowEdge::normal(
                    body,
                    exit,
                ),
            )
            .expect("exit");

        let graph =
            builder.finish().expect("graph");

        assert_eq!(
            operation_protection(
                &graph,
                OperationId::new(0),
            ),
            OperationProtection::Unknown
        );
    }

    #[test]
    fn builder_limits_are_enforced() {
        let mut builder =
            ControlFlowGraphBuilder::new()
                .with_max_nodes(1);

        builder
            .add_node(ControlFlowNode::entry())
            .expect("first node");

        assert!(matches!(
            builder.add_node(
                ControlFlowNode::exit()
            ),
            Err(
                ControlFlowError::ResourceLimitExceeded {
                    resource: "nodes",
                    ..
                }
            )
        ));
    }

    #[test]
    fn deterministic_ids_follow_insertion_order() {
        let mut builder =
            ControlFlowGraphBuilder::new();

        let first =
            builder
                .add_node(ControlFlowNode::entry())
                .expect("first");

        let second =
            builder
                .add_node(ControlFlowNode::exit())
                .expect("second");

        assert_eq!(first.index(), 0);
        assert_eq!(second.index(), 1);
    }
}