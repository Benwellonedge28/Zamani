//! Zamani Quantum Optimization — Optimization Blocks
//!
//! Production-grade logical optimization blocks over the canonical Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir::QuantumCircuit
//!      │
//!      ▼
//! quantum::optimization::circuit::CircuitView
//!      │
//!      ▼
//! quantum::optimization::structure::Block
//!      │
//!      ├── analysis
//!      ├── local optimization
//!      ├── algebraic optimization
//!      ├── synthesis
//!      ├── fault-tolerant optimization
//!      └── verification
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! scheduling
//!      │
//!      ▼
//! hardware
//! ```
//!
//! `Block` is an optimizer-owned structural view. It is NOT another quantum
//! intermediate representation.
//!
//! The authoritative quantum representation remains:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! and optimizer access remains:
//!
//! `crate::quantum::optimization::circuit::CircuitView`
//!
//! # Purpose
//!
//! An optimization block identifies a contiguous, semantically bounded region
//! of a canonical logical circuit that can be inspected by optimization
//! analyses and transformation passes.
//!
//! Blocks provide:
//!
//! - deterministic operation ranges;
//! - invocation-local block identity;
//! - invocation-local region identity;
//! - nested-block relationships;
//! - semantic boundary information;
//! - optimization eligibility information;
//! - read-only operation iteration;
//! - overflow-safe range calculations;
//! - zero-copy views over the canonical circuit;
//! - deterministic subdivision;
//! - explicit boundary semantics;
//! - safe handling of empty blocks;
//! - no dependency on routing;
//! - no dependency on scheduling;
//! - no dependency on hardware;
//! - no dependency on execution;
//! - no dependency on a particular optimization algorithm.
//!
//! # Why blocks exist
//!
//! Quantum optimization cannot safely treat an entire circuit as one
//! unrestricted sequence. Measurements, resets, barriers, classical
//! dependencies, control-flow boundaries, and other semantic boundaries can
//! prevent transformations from crossing a boundary.
//!
//! A block therefore provides the structural unit on which later components
//! can reason:
//!
//! ```text
//! Circuit
//! │
//! ├── Block 0
//! │     ├── operation
//! │     ├── operation
//! │     └── operation
//! │
//! ├── Block 1
//! │     ├── operation
//! │     └── operation
//! │
//! └── Block 2
//!       └── operation
//! ```
//!
//! # Important semantic rule
//!
//! A block boundary is a compiler fact, not merely an array boundary.
//!
//! Creating a block does NOT assert that arbitrary quantum operations may be
//! moved across its boundaries. Transformation passes remain responsible for
//! proving their own semantic preconditions.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - block identity;
//! - block range;
//! - block kind;
//! - block boundary metadata;
//! - block hierarchy metadata;
//! - read-only block traversal.
//!
//! This module does NOT own:
//!
//! - quantum gates;
//! - quantum operations;
//! - circuit mutation;
//! - rewrite rules;
//! - equivalence proofs;
//! - hardware topology;
//! - physical qubit mapping;
//! - scheduling;
//! - backend execution.
//!
//! # Integration contract
//!
//! `structure/block.rs` depends only on the already-established optimizer
//! circuit access layer:
//!
//! ```text
//! optimization::circuit
//!          │
//!          ▼
//! structure::block
//! ```
//!
//! It must NOT depend on:
//!
//! - `OptimizationPipeline`;
//! - `OptimizationPass`;
//! - `OptimizationContext`;
//! - a specific optimization pass;
//! - routing;
//! - scheduling;
//! - hardware;
//! - benchmarking;
//! - execution.
//!
//! Future modules consume this file:
//!
//! - `structure/region.rs` builds region hierarchies from blocks;
//! - `structure/loop.rs` identifies loop bodies and loop-invariant regions;
//! - `structure/conditional.rs` models conditional branches;
//! - `structure/control_flow.rs` builds higher-level control-flow structure;
//! - `analysis/dependency.rs` analyzes dependencies inside blocks;
//! - `analysis/commutation.rs` reasons about movement within blocks;
//! - `analysis/liveness.rs` calculates qubit liveness;
//! - `local/*` applies local rewrites inside eligible blocks;
//! - `rewrite.rs` applies validated transformations within blocks;
//! - `pipeline.rs` schedules block-aware optimization passes;
//! - `planner.rs` chooses block strategies.
//!
//! No later module should need to change the fundamental block representation.
//!
//! # Scaling
//!
//! The block representation itself is O(1) additional memory because it stores
//! only a reference to the canonical circuit plus range/metadata information.
//!
//! Iteration is lazy and O(1) auxiliary memory.
//!
//! Creating a block does not clone gates.
//!
//! This means the representation can scale from tiny circuits to the largest
//! circuits permitted by the canonical IR, process memory, and optimizer
//! resource limits.
//!
//! "Infinite" circuits are therefore not represented by an artificial integer
//! limit here. Actual resource limits remain owned by the IR and optimization
//! limit systems.
//!
//! # Determinism
//!
//! Blocks are deterministic:
//!
//! - operation order is canonical circuit order;
//! - block ranges are half-open `[start, end)`;
//! - block IDs are invocation-local;
//! - hierarchy metadata is explicit;
//! - no hash-based ordering is required;
//! - no randomness is used.
//!
//! # Safety
//!
//! This file contains no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features are required.
//!
//! No external dependencies are required.

use std::fmt;
use std::ops::Range;

use super::super::circuit::{
    CircuitView,
    CircuitViewError,
    OperationId,
    OperationRef,
    RegionId,
};

// ============================================================================
// IDs
// ============================================================================

/// Invocation-local identifier for an optimization block.
///
/// A `BlockId` is valid only for the optimizer invocation that created it.
/// It must never be persisted as a globally stable circuit identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(usize);

impl BlockId {
    /// Creates a block identifier from an invocation-local index.
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

impl fmt::Display for BlockId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "block{}", self.0)
    }
}

// ============================================================================
// Block kind
// ============================================================================

/// Semantic classification of an optimization block.
///
/// The classification is deliberately conservative. A block kind describes
/// structural intent; it does not itself prove that every transformation is
/// legal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockKind {
    /// A normal straight-line logical quantum region.
    StraightLine,

    /// A block known by the caller to contain only unitary operations.
    Unitary,

    /// A block containing measurement semantics.
    Measurement,

    /// A block containing reset semantics.
    Reset,

    /// A block containing both unitary and non-unitary operations.
    Mixed,

    /// A block explicitly delimited by an optimizer barrier.
    BarrierDelimited,

    /// A block representing the body of a control-flow construct.
    ControlFlowBody,

    /// A block representing a conditional branch.
    ConditionalBranch,

    /// A block representing a loop body.
    LoopBody,

    /// A block representing a user/compiler-defined region.
    UserDefined,

    /// A block whose semantic classification is intentionally unknown.
    Unknown,
}

impl Default for BlockKind {
    fn default() -> Self {
        Self::StraightLine
    }
}

impl BlockKind {
    /// Returns true when the block may be treated as a pure unitary region.
    ///
    /// This is intentionally conservative. `Unknown` is false.
    #[must_use]
    pub const fn is_unitary(self) -> bool {
        matches!(self, Self::Unitary)
    }

    /// Returns true when the block explicitly contains measurement semantics.
    #[must_use]
    pub const fn contains_measurement(self) -> bool {
        matches!(self, Self::Measurement | Self::Mixed)
    }

    /// Returns true when the block explicitly contains reset semantics.
    #[must_use]
    pub const fn contains_reset(self) -> bool {
        matches!(self, Self::Reset | Self::Mixed)
    }

    /// Returns true when this block represents control-flow structure.
    #[must_use]
    pub const fn is_control_flow(self) -> bool {
        matches!(
            self,
            Self::ControlFlowBody
                | Self::ConditionalBranch
                | Self::LoopBody
        )
    }

    /// Returns true when this block is structurally eligible for ordinary
    /// local unitary rewrites.
    ///
    /// This does not replace pass-specific semantic checks.
    #[must_use]
    pub const fn permits_unitary_local_rewrites(self) -> bool {
        matches!(
            self,
            Self::StraightLine
                | Self::Unitary
                | Self::BarrierDelimited
                | Self::UserDefined
        )
    }
}

// ============================================================================
// Boundary semantics
// ============================================================================

/// Semantic reason why a block boundary exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockBoundaryKind {
    /// Beginning/end of the complete circuit.
    Circuit,

    /// Boundary supplied by a compiler region.
    Region,

    /// Boundary caused by measurement semantics.
    Measurement,

    /// Boundary caused by reset semantics.
    Reset,

    /// Explicit optimization barrier.
    Barrier,

    /// Classical control-flow boundary.
    ControlFlow,

    /// Conditional branch boundary.
    Conditional,

    /// Loop boundary.
    Loop,

    /// User/compiler-defined semantic boundary.
    UserDefined,

    /// Boundary exists but its semantic reason is not known here.
    Unknown,
}

impl Default for BlockBoundaryKind {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Describes one side of a block boundary.
///
/// The optimizer must never infer that crossing a boundary is legal merely
/// because two blocks are adjacent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockBoundary {
    kind: BlockBoundaryKind,

    /// Whether operations before this boundary may be reordered with
    /// operations after it.
    ///
    /// `false` is the conservative default.
    reorderable: bool,

    /// Whether a transformation is allowed to remove the boundary itself.
    ///
    /// This is normally false and should only be true for explicitly
    /// optimizer-owned boundaries.
    removable: bool,
}

impl BlockBoundary {
    /// Creates a conservative semantic boundary.
    #[must_use]
    pub const fn new(kind: BlockBoundaryKind) -> Self {
        Self {
            kind,
            reorderable: false,
            removable: false,
        }
    }

    /// Creates a boundary with explicit movement/removal policy.
    ///
    /// Callers must only set these flags when the corresponding semantic
    /// guarantees have already been established.
    #[must_use]
    pub const fn with_policy(
        kind: BlockBoundaryKind,
        reorderable: bool,
        removable: bool,
    ) -> Self {
        Self {
            kind,
            reorderable,
            removable,
        }
    }

    /// Returns the boundary kind.
    #[must_use]
    pub const fn kind(self) -> BlockBoundaryKind {
        self.kind
    }

    /// Returns whether reordering across this boundary is permitted.
    #[must_use]
    pub const fn is_reorderable(self) -> bool {
        self.reorderable
    }

    /// Returns whether the boundary itself may be removed by a transformation.
    #[must_use]
    pub const fn is_removable(self) -> bool {
        self.removable
    }
}

// ============================================================================
// Block hierarchy metadata
// ============================================================================

/// Hierarchical relationship of a block to another optimizer region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockHierarchy {
    /// Optional parent block.
    parent: Option<BlockId>,

    /// Optional owning optimizer region.
    region: Option<RegionId>,

    /// Nesting depth.
    depth: usize,
}

impl Default for BlockHierarchy {
    fn default() -> Self {
        Self {
            parent: None,
            region: None,
            depth: 0,
        }
    }
}

impl BlockHierarchy {
    /// Creates root-level hierarchy metadata.
    #[must_use]
    pub const fn root() -> Self {
        Self {
            parent: None,
            region: None,
            depth: 0,
        }
    }

    /// Creates nested hierarchy metadata.
    ///
    /// The caller is responsible for supplying a correct depth.
    #[must_use]
    pub const fn nested(
        parent: BlockId,
        region: Option<RegionId>,
        depth: usize,
    ) -> Self {
        Self {
            parent: Some(parent),
            region,
            depth,
        }
    }

    /// Returns the optional parent block.
    #[must_use]
    pub const fn parent(self) -> Option<BlockId> {
        self.parent
    }

    /// Returns the optional owning region.
    #[must_use]
    pub const fn region(self) -> Option<RegionId> {
        self.region
    }

    /// Returns the nesting depth.
    #[must_use]
    pub const fn depth(self) -> usize {
        self.depth
    }

    /// Returns true when this is a root block.
    #[must_use]
    pub const fn is_root(self) -> bool {
        self.parent.is_none()
    }
}

// ============================================================================
// Block descriptor
// ============================================================================

/// Immutable owned description of an optimization block.
///
/// `BlockDescriptor` contains no circuit data. It is therefore cheap to store
/// in region indexes, planners, diagnostics, and provenance records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockDescriptor {
    id: BlockId,
    range: Range<usize>,
    kind: BlockKind,
    entry: BlockBoundary,
    exit: BlockBoundary,
    hierarchy: BlockHierarchy,
    optimizable: bool,
}

impl BlockDescriptor {
    /// Creates a block descriptor.
    ///
    /// The range must be a valid half-open interval.
    pub fn new(
        id: BlockId,
        range: Range<usize>,
        kind: BlockKind,
    ) -> Result<Self, BlockError> {
        validate_range_shape(&range)?;

        Ok(Self {
            id,
            range,
            kind,
            entry: BlockBoundary::default(),
            exit: BlockBoundary::default(),
            hierarchy: BlockHierarchy::root(),
            optimizable: true,
        })
    }

    /// Returns the block ID.
    #[must_use]
    pub const fn id(&self) -> BlockId {
        self.id
    }

    /// Returns the first operation index.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.range.start
    }

    /// Returns the exclusive operation end index.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.range.end
    }

    /// Returns the operation range.
    #[must_use]
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    /// Returns the operation count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.range.end - self.range.start
    }

    /// Returns whether the block is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.range.start == self.range.end
    }

    /// Returns the block kind.
    #[must_use]
    pub const fn kind(&self) -> BlockKind {
        self.kind
    }

    /// Returns the entry boundary.
    #[must_use]
    pub const fn entry_boundary(&self) -> BlockBoundary {
        self.entry
    }

    /// Returns the exit boundary.
    #[must_use]
    pub const fn exit_boundary(&self) -> BlockBoundary {
        self.exit
    }

    /// Returns hierarchy metadata.
    #[must_use]
    pub const fn hierarchy(&self) -> BlockHierarchy {
        self.hierarchy
    }

    /// Returns whether ordinary optimization passes may consider the block.
    #[must_use]
    pub const fn is_optimizable(&self) -> bool {
        self.optimizable
    }

    /// Sets the entry boundary.
    #[must_use]
    pub const fn with_entry_boundary(
        mut self,
        boundary: BlockBoundary,
    ) -> Self {
        self.entry = boundary;
        self
    }

    /// Sets the exit boundary.
    #[must_use]
    pub const fn with_exit_boundary(
        mut self,
        boundary: BlockBoundary,
    ) -> Self {
        self.exit = boundary;
        self
    }

    /// Sets hierarchy metadata.
    #[must_use]
    pub const fn with_hierarchy(
        mut self,
        hierarchy: BlockHierarchy,
    ) -> Self {
        self.hierarchy = hierarchy;
        self
    }

    /// Marks the block as optimization-ineligible.
    #[must_use]
    pub const fn non_optimizable(
        mut self,
    ) -> Self {
        self.optimizable = false;
        self
    }

    /// Returns true when another block is fully contained in this block.
    #[must_use]
    pub fn contains_range(
        &self,
        range: &Range<usize>,
    ) -> bool {
        range.start >= self.range.start
            && range.end <= self.range.end
            && range.start <= range.end
    }

    /// Returns true when this block overlaps another range.
    #[must_use]
    pub fn overlaps_range(
        &self,
        range: &Range<usize>,
    ) -> bool {
        self.range.start < range.end
            && range.start < self.range.end
    }

    /// Returns true when two blocks are directly adjacent.
    #[must_use]
    pub fn is_adjacent_to(
        &self,
        other: &Self,
    ) -> bool {
        self.range.end == other.range.start
            || other.range.end == self.range.start
    }
}

// ============================================================================
// Block errors
// ============================================================================

/// Errors produced by block construction or traversal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockError {
    /// A range has start greater than end.
    InvalidRange {
        /// Range start.
        start: usize,

        /// Range end.
        end: usize,
    },

    /// A range extends beyond the supplied circuit.
    RangeOutOfBounds {
        /// Range start.
        start: usize,

        /// Range end.
        end: usize,

        /// Circuit operation count.
        circuit_len: usize,
    },

    /// Arithmetic overflow occurred while deriving a range.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// The supplied parent block does not contain the child block.
    InvalidParent {
        /// Child block.
        child: BlockId,

        /// Parent block.
        parent: BlockId,
    },

    /// A requested subdivision point is outside this block.
    InvalidSplitPoint {
        /// Split position.
        position: usize,

        /// Block start.
        start: usize,

        /// Block end.
        end: usize,
    },

    /// A requested operation does not belong to this block.
    OperationNotInBlock {
        /// Operation identifier.
        operation: OperationId,

        /// Block identifier.
        block: BlockId,
    },

    /// The underlying canonical circuit access layer failed.
    CircuitView(CircuitViewError),
}

impl fmt::Display for BlockError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRange { start, end } => {
                write!(
                    formatter,
                    "invalid optimization block range {start}..{end}"
                )
            }

            Self::RangeOutOfBounds {
                start,
                end,
                circuit_len,
            } => {
                write!(
                    formatter,
                    "optimization block range {start}..{end} exceeds \
                     circuit length {circuit_len}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidParent { child, parent } => {
                write!(
                    formatter,
                    "{child} is not contained by parent {parent}"
                )
            }

            Self::InvalidSplitPoint {
                position,
                start,
                end,
            } => {
                write!(
                    formatter,
                    "split point {position} is outside block range \
                     {start}..={end}"
                )
            }

            Self::OperationNotInBlock { operation, block } => {
                write!(
                    formatter,
                    "{operation} does not belong to {block}"
                )
            }

            Self::CircuitView(error) => {
                write!(formatter, "optimization circuit view error: {error}")
            }
        }
    }
}

impl std::error::Error for BlockError {}

impl From<CircuitViewError> for BlockError {
    fn from(error: CircuitViewError) -> Self {
        Self::CircuitView(error)
    }
}

// ============================================================================
// Block
// ============================================================================

/// Zero-copy immutable optimization block.
///
/// `Block` borrows the canonical circuit through `CircuitView`.
///
/// No gate or operation is copied when a block is created.
#[derive(Debug, Clone, Copy)]
pub struct Block<'a> {
    view: CircuitView<'a>,
    descriptor: BlockDescriptor,
}

impl<'a> Block<'a> {
    /// Creates a block over a validated circuit view.
    pub fn new(
        view: CircuitView<'a>,
        descriptor: BlockDescriptor,
    ) -> Result<Self, BlockError> {
        validate_range_against_circuit(
            &descriptor.range,
            view.len(),
        )?;

        Ok(Self {
            view,
            descriptor,
        })
    }

    /// Creates a whole-circuit block.
    ///
    /// This operation is O(1) and does not copy the circuit.
    pub fn whole_circuit(
        view: CircuitView<'a>,
        id: BlockId,
    ) -> Result<Self, BlockError> {
        let range = 0..view.len();

        let descriptor = BlockDescriptor::new(
            id,
            range,
            BlockKind::StraightLine,
        )?
        .with_entry_boundary(
            BlockBoundary::new(BlockBoundaryKind::Circuit),
        )
        .with_exit_boundary(
            BlockBoundary::new(BlockBoundaryKind::Circuit),
        );

        Self::new(view, descriptor)
    }

    /// Creates a block from an operation range.
    pub fn from_range(
        view: CircuitView<'a>,
        id: BlockId,
        range: Range<usize>,
        kind: BlockKind,
    ) -> Result<Self, BlockError> {
        let descriptor =
            BlockDescriptor::new(id, range, kind)?;

        Self::new(view, descriptor)
    }

    /// Returns the immutable canonical circuit view.
    #[must_use]
    pub const fn view(&self) -> CircuitView<'a> {
        self.view
    }

    /// Returns the block descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &BlockDescriptor {
        &self.descriptor
    }

    /// Returns the block identifier.
    #[must_use]
    pub const fn id(&self) -> BlockId {
        self.descriptor.id
    }

    /// Returns the region identifier, if any.
    #[must_use]
    pub const fn region(&self) -> Option<RegionId> {
        self.descriptor.hierarchy.region()
    }

    /// Returns the parent block identifier, if any.
    #[must_use]
    pub const fn parent(&self) -> Option<BlockId> {
        self.descriptor.hierarchy.parent()
    }

    /// Returns the hierarchy depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.descriptor.hierarchy.depth()
    }

    /// Returns the block kind.
    #[must_use]
    pub const fn kind(&self) -> BlockKind {
        self.descriptor.kind
    }

    /// Returns the first operation index.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.descriptor.range.start
    }

    /// Returns the exclusive operation end index.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.descriptor.range.end
    }

    /// Returns the number of operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.end() - self.start()
    }

    /// Returns whether the block contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start() == self.end()
    }

    /// Returns whether the block is marked optimization-eligible.
    #[must_use]
    pub const fn is_optimizable(&self) -> bool {
        self.descriptor.optimizable
    }

    /// Returns the entry boundary.
    #[must_use]
    pub const fn entry_boundary(&self) -> BlockBoundary {
        self.descriptor.entry
    }

    /// Returns the exit boundary.
    #[must_use]
    pub const fn exit_boundary(&self) -> BlockBoundary {
        self.descriptor.exit
    }

    /// Returns whether the block contains an operation index.
    #[must_use]
    pub fn contains_index(
        &self,
        index: usize,
    ) -> bool {
        index >= self.start() && index < self.end()
    }

    /// Returns whether the block contains an operation ID.
    #[must_use]
    pub fn contains_operation(
        &self,
        operation: OperationId,
    ) -> bool {
        self.contains_index(operation.index())
    }

    /// Converts an absolute operation index into a block-relative index.
    pub fn relative_index(
        &self,
        index: usize,
    ) -> Result<usize, BlockError> {
        if !self.contains_index(index) {
            return Err(BlockError::OperationNotInBlock {
                operation: OperationId::new(index),
                block: self.id(),
            });
        }

        Ok(index - self.start())
    }

    /// Returns an operation by absolute circuit index.
    pub fn operation(
        &self,
        index: usize,
    ) -> Result<OperationRef<'a>, BlockError> {
        if !self.contains_index(index) {
            return Err(BlockError::OperationNotInBlock {
                operation: OperationId::new(index),
                block: self.id(),
            });
        }

        self.view.operation(index).map_err(BlockError::from)
    }

    /// Returns an operation by block-relative index.
    pub fn operation_at(
        &self,
        relative_index: usize,
    ) -> Result<OperationRef<'a>, BlockError> {
        let absolute = self
            .start()
            .checked_add(relative_index)
            .ok_or(BlockError::ArithmeticOverflow {
                calculation: "block operation index",
            })?;

        self.operation(absolute)
    }

    /// Returns the first operation.
    pub fn first(
        &self,
    ) -> Result<Option<OperationRef<'a>>, BlockError> {
        if self.is_empty() {
            return Ok(None);
        }

        self.operation(self.start()).map(Some)
    }

    /// Returns the final operation.
    pub fn last(
        &self,
    ) -> Result<Option<OperationRef<'a>>, BlockError> {
        if self.is_empty() {
            return Ok(None);
        }

        let index = self
            .end()
            .checked_sub(1)
            .ok_or(BlockError::ArithmeticOverflow {
                calculation: "last block operation index",
            })?;

        self.operation(index).map(Some)
    }

    /// Iterates lazily over operations in canonical circuit order.
    ///
    /// The iterator performs no allocation and does not copy gates.
    pub fn iter(
        &self,
    ) -> BlockOperationIter<'a> {
        BlockOperationIter {
            view: self.view,
            next: self.start(),
            end: self.end(),
        }
    }

    /// Creates a nested child block.
    ///
    /// The child must be completely contained in this block.
    pub fn child(
        &self,
        id: BlockId,
        range: Range<usize>,
        kind: BlockKind,
    ) -> Result<Self, BlockError> {
        validate_range_shape(&range)?;

        if !self.descriptor.contains_range(&range) {
            return Err(BlockError::InvalidParent {
                child: id,
                parent: self.id(),
            });
        }

        let next_depth = self
            .depth()
            .checked_add(1)
            .ok_or(BlockError::ArithmeticOverflow {
                calculation: "block nesting depth",
            })?;

        let descriptor = BlockDescriptor::new(
            id,
            range,
            kind,
        )?
        .with_hierarchy(BlockHierarchy::nested(
            self.id(),
            self.region(),
            next_depth,
        ));

        Self::new(self.view, descriptor)
    }

    /// Creates a child block that uses a different region identifier.
    pub fn child_in_region(
        &self,
        id: BlockId,
        region: RegionId,
        range: Range<usize>,
        kind: BlockKind,
    ) -> Result<Self, BlockError> {
        validate_range_shape(&range)?;

        if !self.descriptor.contains_range(&range) {
            return Err(BlockError::InvalidParent {
                child: id,
                parent: self.id(),
            });
        }

        let next_depth = self
            .depth()
            .checked_add(1)
            .ok_or(BlockError::ArithmeticOverflow {
                calculation: "block nesting depth",
            })?;

        let descriptor = BlockDescriptor::new(
            id,
            range,
            kind,
        )?
        .with_hierarchy(BlockHierarchy::nested(
            self.id(),
            Some(region),
            next_depth,
        ));

        Self::new(self.view, descriptor)
    }

    /// Splits the block into two adjacent child blocks.
    ///
    /// The split point is relative to the absolute circuit operation index.
    ///
    /// Empty child blocks are permitted. This is useful for representing
    /// insertion boundaries and empty control-flow regions without inventing
    /// sentinel operations.
    pub fn split(
        &self,
        left_id: BlockId,
        right_id: BlockId,
        position: usize,
    ) -> Result<(Self, Self), BlockError> {
        if position < self.start()
            || position > self.end()
        {
            return Err(BlockError::InvalidSplitPoint {
                position,
                start: self.start(),
                end: self.end(),
            });
        }

        let left = self.child(
            left_id,
            self.start()..position,
            self.kind(),
        )?;

        let right = self.child(
            right_id,
            position..self.end(),
            self.kind(),
        )?;

        Ok((left, right))
    }

    /// Returns a descriptor for the block.
    #[must_use]
    pub fn to_descriptor(&self) -> BlockDescriptor {
        self.descriptor.clone()
    }

    /// Returns the canonical circuit operation count.
    #[must_use]
    pub fn circuit_operation_count(&self) -> usize {
        self.view.len()
    }

    /// Returns the canonical logical-qubit count.
    #[must_use]
    pub fn circuit_qubit_count(&self) -> usize {
        self.view.num_qubits()
    }

    /// Returns the canonical classical-bit count.
    #[must_use]
    pub fn circuit_classical_bit_count(&self) -> usize {
        self.view.num_classical_bits()
    }
}

// ============================================================================
// Iterator
// ============================================================================

/// Lazy, allocation-free iterator over operations in a block.
#[derive(Debug, Clone, Copy)]
pub struct BlockOperationIter<'a> {
    view: CircuitView<'a>,
    next: usize,
    end: usize,
}

impl<'a> Iterator for BlockOperationIter<'a> {
    type Item = Result<OperationRef<'a>, BlockError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.end {
            return None;
        }

        let index = self.next;

        self.next = match self.next.checked_add(1) {
            Some(next) => next,
            None => {
                return Some(Err(BlockError::ArithmeticOverflow {
                    calculation: "block iterator index",
                }));
            }
        };

        Some(
            self.view
                .operation(index)
                .map_err(BlockError::from),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.next);
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for BlockOperationIter<'a> {}

impl<'a> std::iter::FusedIterator for BlockOperationIter<'a> {}

// ============================================================================
// Block collection validation
// ============================================================================

/// Validates that a collection of block descriptors is deterministic and
/// structurally non-overlapping.
///
/// This function does not allocate and does not sort the descriptors.
///
/// The caller must therefore provide descriptors in monotonically increasing
/// range order.
///
/// Nested blocks should not be passed to this function as siblings; use the
/// parent/child hierarchy instead.
pub fn validate_sibling_blocks(
    blocks: &[BlockDescriptor],
    circuit_len: usize,
) -> Result<(), BlockError> {
    let mut previous_end = 0usize;

    for block in blocks {
        validate_range_against_circuit(
            &block.range,
            circuit_len,
        )?;

        if block.start() < previous_end {
            return Err(BlockError::InvalidRange {
                start: block.start(),
                end: block.end(),
            });
        }

        previous_end = block.end();
    }

    Ok(())
}

/// Returns the number of operations covered by a sibling block collection.
///
/// The function uses checked arithmetic so pathological inputs cannot wrap
/// around `usize`.
pub fn covered_operation_count(
    blocks: &[BlockDescriptor],
) -> Result<usize, BlockError> {
    let mut total = 0usize;

    for block in blocks {
        total = total
            .checked_add(block.len())
            .ok_or(BlockError::ArithmeticOverflow {
                calculation: "total block operation count",
            })?;
    }

    Ok(total)
}

// ============================================================================
// Range helpers
// ============================================================================

fn validate_range_shape(
    range: &Range<usize>,
) -> Result<(), BlockError> {
    if range.start > range.end {
        return Err(BlockError::InvalidRange {
            start: range.start,
            end: range.end,
        });
    }

    Ok(())
}

fn validate_range_against_circuit(
    range: &Range<usize>,
    circuit_len: usize,
) -> Result<(), BlockError> {
    validate_range_shape(range)?;

    if range.end > circuit_len {
        return Err(BlockError::RangeOutOfBounds {
            start: range.start,
            end: range.end,
            circuit_len,
        });
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor(
        id: usize,
        start: usize,
        end: usize,
    ) -> BlockDescriptor {
        BlockDescriptor::new(
            BlockId::new(id),
            start..end,
            BlockKind::StraightLine,
        )
        .expect("test range must be valid")
    }

    #[test]
    fn block_descriptor_accepts_empty_range() {
        let block = descriptor(0, 2, 2);

        assert!(block.is_empty());
        assert_eq!(block.len(), 0);
        assert_eq!(block.start(), 2);
        assert_eq!(block.end(), 2);
    }

    #[test]
    fn block_descriptor_rejects_inverted_range() {
        let result = BlockDescriptor::new(
            BlockId::new(0),
            5..2,
            BlockKind::StraightLine,
        );

        assert!(matches!(
            result,
            Err(BlockError::InvalidRange {
                start: 5,
                end: 2,
            })
        ));
    }

    #[test]
    fn descriptor_contains_range() {
        let block = descriptor(0, 2, 10);

        assert!(block.contains_range(&(2..10)));
        assert!(block.contains_range(&(3..7)));
        assert!(block.contains_range(&(5..5)));
        assert!(!block.contains_range(&(1..5)));
        assert!(!block.contains_range(&(5..11)));
    }

    #[test]
    fn descriptor_detects_overlap() {
        let block = descriptor(0, 2, 10);

        assert!(block.overlaps_range(&(1..3)));
        assert!(block.overlaps_range(&(5..12)));
        assert!(block.overlaps_range(&(2..10)));
        assert!(!block.overlaps_range(&(10..12)));
        assert!(!block.overlaps_range(&(0..2)));
    }

    #[test]
    fn adjacent_blocks_are_detected() {
        let left = descriptor(0, 0, 4);
        let right = descriptor(1, 4, 8);

        assert!(left.is_adjacent_to(&right));
    }

    #[test]
    fn root_hierarchy_is_root() {
        let hierarchy = BlockHierarchy::root();

        assert!(hierarchy.is_root());
        assert_eq!(hierarchy.parent(), None);
        assert_eq!(hierarchy.region(), None);
        assert_eq!(hierarchy.depth(), 0);
    }

    #[test]
    fn nested_hierarchy_is_not_root() {
        let hierarchy = BlockHierarchy::nested(
            BlockId::new(4),
            Some(RegionId::new(7)),
            3,
        );

        assert!(!hierarchy.is_root());
        assert_eq!(
            hierarchy.parent(),
            Some(BlockId::new(4))
        );
        assert_eq!(
            hierarchy.region(),
            Some(RegionId::new(7))
        );
        assert_eq!(hierarchy.depth(), 3);
    }

    #[test]
    fn boundary_is_conservative_by_default() {
        let boundary =
            BlockBoundary::new(BlockBoundaryKind::Measurement);

        assert_eq!(
            boundary.kind(),
            BlockBoundaryKind::Measurement
        );
        assert!(!boundary.is_reorderable());
        assert!(!boundary.is_removable());
    }

    #[test]
    fn boundary_policy_is_explicit() {
        let boundary = BlockBoundary::with_policy(
            BlockBoundaryKind::UserDefined,
            true,
            true,
        );

        assert!(boundary.is_reorderable());
        assert!(boundary.is_removable());
    }

    #[test]
    fn unitary_classification_is_conservative() {
        assert!(BlockKind::Unitary.is_unitary());
        assert!(!BlockKind::Unknown.is_unitary());
        assert!(!BlockKind::Mixed.is_unitary());
    }

    #[test]
    fn measurement_classification_is_explicit() {
        assert!(
            BlockKind::Measurement.contains_measurement()
        );
        assert!(
            BlockKind::Mixed.contains_measurement()
        );
        assert!(
            !BlockKind::Unitary.contains_measurement()
        );
    }

    #[test]
    fn reset_classification_is_explicit() {
        assert!(BlockKind::Reset.contains_reset());
        assert!(BlockKind::Mixed.contains_reset());
        assert!(!BlockKind::Unitary.contains_reset());
    }

    #[test]
    fn control_flow_classification_is_explicit() {
        assert!(
            BlockKind::ControlFlowBody.is_control_flow()
        );
        assert!(
            BlockKind::ConditionalBranch.is_control_flow()
        );
        assert!(
            BlockKind::LoopBody.is_control_flow()
        );
        assert!(
            !BlockKind::Unitary.is_control_flow()
        );
    }

    #[test]
    fn sibling_validation_accepts_ordered_non_overlapping_blocks() {
        let blocks = vec![
            descriptor(0, 0, 4),
            descriptor(1, 4, 7),
            descriptor(2, 7, 12),
        ];

        assert!(
            validate_sibling_blocks(&blocks, 12).is_ok()
        );
    }

    #[test]
    fn sibling_validation_rejects_overlap() {
        let blocks = vec![
            descriptor(0, 0, 5),
            descriptor(1, 4, 8),
        ];

        assert!(matches!(
            validate_sibling_blocks(&blocks, 10),
            Err(BlockError::InvalidRange {
                start: 4,
                end: 8,
            })
        ));
    }

    #[test]
    fn sibling_validation_rejects_out_of_bounds() {
        let blocks = vec![
            descriptor(0, 0, 5),
            descriptor(1, 5, 11),
        ];

        assert!(matches!(
            validate_sibling_blocks(&blocks, 10),
            Err(BlockError::RangeOutOfBounds {
                start: 5,
                end: 11,
                circuit_len: 10,
            })
        ));
    }

    #[test]
    fn covered_operation_count_is_checked() {
        let blocks = vec![
            descriptor(0, 0, 4),
            descriptor(1, 4, 9),
        ];

        assert_eq!(
            covered_operation_count(&blocks)
                .expect("count must succeed"),
            9
        );
    }

    #[test]
    fn empty_block_is_a_valid_structural_object() {
        let block = descriptor(5, 7, 7);

        assert!(block.is_empty());
        assert_eq!(block.len(), 0);
        assert!(!block.overlaps_range(&(7..8)));
        assert!(block.contains_range(&(7..7)));
    }

    #[test]
    fn operation_id_membership_uses_invocation_local_index() {
        let block = descriptor(2, 10, 20);

        assert!(block.contains_operation(
            OperationId::new(10)
        ));
        assert!(block.contains_operation(
            OperationId::new(19)
        ));
        assert!(!block.contains_operation(
            OperationId::new(20)
        ));
        assert!(!block.contains_operation(
            OperationId::new(9)
        ));
    }
}