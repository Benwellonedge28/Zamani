//! Zamani Quantum IR — Structured Regions and Blocks
//!
//! This module defines the canonical hardware-independent structural container
//! used by the Zamani Quantum IR.
//!
//! # Architectural role
//!
//! `region.rs` owns the STRUCTURE of an IR program:
//!
//! ```text
//! Program
//!   └── Region
//!         ├── Block
//!         │     ├── OperationId
//!         │     ├── OperationId
//!         │     └── ...
//!         └── Region
//!               └── Block
//! ```
//!
//! A region is a structured semantic container. A block is an ordered/basic
//! container of operation identities.
//!
//! This module deliberately does NOT own the concrete operation definitions.
//! `OperationId` comes from `identity.rs`; the concrete `Operation` type belongs
//! to `operation.rs`.
//!
//! This separation is intentional:
//!
//! ```text
//! identity.rs
//!     │
//!     ├── RegionId
//!     ├── BlockId
//!     └── OperationId
//!
//! region.rs
//!     │
//!     ├── Region
//!     └── Block
//!
//! operation.rs
//!     │
//!     └── Operation
//!
//! program.rs
//!     │
//!     └── QuantumProgram
//! ```
//!
//! # Universal quantum-program principle
//!
//! A Zamani program is written once and may ultimately target:
//!
//! - small quantum systems;
//! - large quantum systems;
//! - distributed quantum systems;
//! - superconducting systems;
//! - trapped-ion systems;
//! - neutral-atom systems;
//! - photonic systems;
//! - spin systems;
//! - topological systems;
//! - analog quantum systems;
//! - annealing systems;
//! - logical/fault-tolerant systems;
//! - simulators;
//! - future quantum architectures.
//!
//! A region therefore MUST NOT encode a fixed machine size.
//!
//! In particular, values such as:
//!
//! ```text
//! 63
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! must never be interpreted here as maximum numbers of qubits, blocks,
//! operations, or regions.
//!
//! Concrete resource limits belong to `limits.rs`.
//! Physical capacity belongs to `quantum::hardware`.
//!
//! # Why regions exist
//!
//! Quantum programs are not necessarily flat circuits.
//!
//! A production quantum language must be able to represent structures such as:
//!
//! ```text
//! function
//!     └── region
//!           └── block
//!
//! if condition
//!     ├── then region
//!     └── else region
//!
//! while condition
//!     └── loop region
//!
//! for each qubit
//!     └── body region
//!
//! pulse program
//!     └── pulse region
//!
//! circuit
//!     └── circuit region
//!
//! fault-tolerant logical operation
//!     └── implementation region
//! ```
//!
//! The region model is therefore deliberately more general than
//! `QuantumCircuit`.
//!
//! # Operation ordering
//!
//! Operations inside a block are stored in explicit program order.
//!
//! A later optimizer, dependency analyzer, scheduler, or DAG builder may derive
//! additional dependency information, but `region.rs` does not perform those
//! transformations.
//!
//! This distinction is important:
//!
//! ```text
//! region.rs
//!     = structural/program order
//!
//! optimization/
//!     = semantic transformation
//!
//! routing/
//!     = logical → physical placement
//!
//! scheduling/
//!     = temporal placement
//! ```
//!
//! # Qubit ownership
//!
//! Regions may declare or reference logical/physical qubits through
//! `quantum::ir::qubit`.
//!
//! This module uses:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! quantum::ir::qubit::QubitRef
//! ```
//!
//! It does NOT perform:
//!
//! - routing;
//! - hardware allocation;
//! - topology validation;
//! - calibration;
//! - scheduling;
//! - pulse generation;
//! - QPU communication;
//! - simulation;
//! - error-correction decoding;
//! - optimization.
//!
//! It merely records the structural scope/reference information required by
//! later compiler stages.
//!
//! # Operation references
//!
//! A block stores `OperationId` values instead of concrete `Operation` values.
//!
//! This avoids a dependency cycle:
//!
//! ```text
//! region.rs ──► operation.rs
//! operation.rs ──► region.rs
//! ```
//!
//! and permits `operation.rs` to be implemented independently.
//!
//! The owning program/module registry resolves an `OperationId` to its actual
//! operation definition.
//!
//! # Parent/child representation
//!
//! Regions use identity references for nested regions rather than recursively
//! owning `Region` values.
//!
//! This avoids recursive data structures and permits a program-level arena,
//! registry, or other storage strategy to own the actual region objects.
//!
//! Consequently this module scales from:
//!
//! ```text
//! one region
//! ```
//!
//! to:
//!
//! ```text
//! very large finite region graphs
//! ```
//!
//! subject only to explicit resource policies and available memory/storage.
//!
//! # Determinism
//!
//! Region APIs preserve insertion order.
//!
//! The same sequence of insertion operations produces the same structural
//! ordering.
//!
//! No global allocator, random identity generation, hash-map iteration order,
//! thread-local state, or hidden mutable global state is used.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` makes that requirement compiler-enforced.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//! No external crates are required.

#![forbid(unsafe_code)]

use std::fmt;

use super::errors::IrSemanticError;
use super::identity::{
    BlockId,
    OperationId,
    RegionId,
};
use super::qubit::{
    PhysicalQubitId,
    QubitId,
    QubitRef,
};

// =============================================================================
// Result alias
// =============================================================================

/// Result type used by region construction and mutation APIs.
///
/// `IrSemanticError` comes from the canonical foundational error module rather
/// than defining a second region-specific error hierarchy.
pub type RegionResult<T> = Result<T, IrSemanticError>;

/// Creates a canonical region semantic error.
///
/// Keeping construction here avoids coupling this file to the implementation
/// details of `IrError` while still using the repository's foundational error
/// vocabulary.
fn region_error<S: Into<String>>(reason: S) -> IrSemanticError {
    IrSemanticError::new("region", reason)
}

/// Converts an allocation failure from a fallible collection operation into a
/// canonical IR semantic error.
///
/// `try_reserve` is used throughout this module when callers explicitly request
/// fallible growth. This prevents collection growth from silently becoming an
/// uncontrolled allocation policy.
fn allocation_error(context: &'static str) -> IrSemanticError {
    region_error(format!(
        "unable to reserve memory for {context}"
    ))
}

// =============================================================================
// Region kind
// =============================================================================

/// Semantic role of an IR region.
///
/// This is intentionally a semantic classification, not a hardware/backend
/// instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegionKind {
    /// Root program region.
    Root,

    /// Generic structured region.
    Generic,

    /// Function body.
    Function,

    /// Gate/circuit-oriented region.
    Circuit,

    /// Pulse/control region.
    Pulse,

    /// Analog quantum region.
    Analog,

    /// Annealing/optimization workload region.
    Annealing,

    /// Conditional `if`/`else` region.
    Conditional,

    /// Loop body.
    Loop,

    /// Repeated execution region.
    Repeat,

    /// Measurement/dynamic-circuit region.
    Dynamic,

    /// Fault-tolerant/logical quantum region.
    Logical,

    /// Classical-only region.
    Classical,

    /// Hybrid quantum/classical region.
    Hybrid,

    /// Implementation region for a higher-level operation.
    Implementation,

    /// Extension-defined region.
    Extension,
}

impl Default for RegionKind {
    fn default() -> Self {
        Self::Generic
    }
}

impl fmt::Display for RegionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Root => "root",
            Self::Generic => "generic",
            Self::Function => "function",
            Self::Circuit => "circuit",
            Self::Pulse => "pulse",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Conditional => "conditional",
            Self::Loop => "loop",
            Self::Repeat => "repeat",
            Self::Dynamic => "dynamic",
            Self::Logical => "logical",
            Self::Classical => "classical",
            Self::Hybrid => "hybrid",
            Self::Implementation => "implementation",
            Self::Extension => "extension",
        };

        write!(formatter, "{value}")
    }
}

// =============================================================================
// Block kind
// =============================================================================

/// Semantic role of a block within a region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BlockKind {
    /// Ordinary sequential block.
    Basic,

    /// Entry block of a region.
    Entry,

    /// Exit block of a region.
    Exit,

    /// Branch/conditional block.
    Branch,

    /// Loop header.
    LoopHeader,

    /// Loop body.
    LoopBody,

    /// Loop continuation.
    LoopContinue,

    /// Loop exit.
    LoopExit,

    /// Function entry.
    FunctionEntry,

    /// Function return.
    FunctionExit,

    /// Exception/error path.
    Error,

    /// Extension-defined block.
    Extension,
}

impl Default for BlockKind {
    fn default() -> Self {
        Self::Basic
    }
}

impl fmt::Display for BlockKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Basic => "basic",
            Self::Entry => "entry",
            Self::Exit => "exit",
            Self::Branch => "branch",
            Self::LoopHeader => "loop_header",
            Self::LoopBody => "loop_body",
            Self::LoopContinue => "loop_continue",
            Self::LoopExit => "loop_exit",
            Self::FunctionEntry => "function_entry",
            Self::FunctionExit => "function_exit",
            Self::Error => "error",
            Self::Extension => "extension",
        };

        write!(formatter, "{value}")
    }
}

// =============================================================================
// Region port
// =============================================================================

/// A qubit crossing a region boundary.
///
/// Region ports are semantic references. They do not allocate hardware and do
/// not perform routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegionQubitPort {
    /// Logical qubit entering/leaving the region.
    Logical(QubitId),

    /// Physical qubit reference used by a lower-level representation.
    Physical(PhysicalQubitId),
}

impl RegionQubitPort {
    /// Creates a logical region port.
    #[must_use]
    pub const fn logical(qubit: QubitId) -> Self {
        Self::Logical(qubit)
    }

    /// Creates a physical region port.
    #[must_use]
    pub const fn physical(qubit: PhysicalQubitId) -> Self {
        Self::Physical(qubit)
    }

    /// Returns the logical qubit if this is a logical port.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::Logical(qubit) => Some(qubit),
            Self::Physical(_) => None,
        }
    }

    /// Returns the physical qubit if this is a physical port.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::Logical(_) => None,
            Self::Physical(qubit) => Some(qubit),
        }
    }

    /// Converts the port to the canonical `QubitRef`.
    #[must_use]
    pub const fn as_qubit_ref(self) -> QubitRef {
        match self {
            Self::Logical(qubit) => QubitRef::Logical(qubit),
            Self::Physical(qubit) => QubitRef::Physical(qubit),
        }
    }
}

impl From<QubitId> for RegionQubitPort {
    fn from(qubit: QubitId) -> Self {
        Self::Logical(qubit)
    }
}

impl From<PhysicalQubitId> for RegionQubitPort {
    fn from(qubit: PhysicalQubitId) -> Self {
        Self::Physical(qubit)
    }
}

impl From<QubitRef> for RegionQubitPort {
    fn from(reference: QubitRef) -> Self {
        match reference {
            QubitRef::Logical(qubit) => Self::Logical(qubit),
            QubitRef::Physical(qubit) => Self::Physical(qubit),
        }
    }
}

impl From<RegionQubitPort> for QubitRef {
    fn from(port: RegionQubitPort) -> Self {
        port.as_qubit_ref()
    }
}

// =============================================================================
// Region
// =============================================================================

/// Canonical structured region.
///
/// A region owns structural information:
///
/// - identity;
/// - semantic kind;
/// - optional parent;
/// - child-region references;
/// - block references;
/// - input/output qubit ports;
/// - entry/exit block references.
///
/// It does NOT own the concrete operations referenced by its blocks.
///
/// Concrete operation storage belongs to `program.rs` or another explicitly
/// designated IR owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    id: RegionId,
    kind: RegionKind,
    parent: Option<RegionId>,
    blocks: Vec<BlockId>,
    child_regions: Vec<RegionId>,
    inputs: Vec<RegionQubitPort>,
    outputs: Vec<RegionQubitPort>,
    entry: Option<BlockId>,
    exit: Option<BlockId>,
}

impl Region {
    /// Creates an empty region.
    ///
    /// The region does not automatically allocate a block.
    #[must_use]
    pub fn new(id: RegionId, kind: RegionKind) -> Self {
        Self {
            id,
            kind,
            parent: None,
            blocks: Vec::new(),
            child_regions: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            entry: None,
            exit: None,
        }
    }

    /// Creates an empty generic region.
    #[must_use]
    pub fn generic(id: RegionId) -> Self {
        Self::new(id, RegionKind::Generic)
    }

    /// Creates a root region.
    #[must_use]
    pub fn root(id: RegionId) -> Self {
        Self::new(id, RegionKind::Root)
    }

    /// Returns the region identity.
    #[must_use]
    pub const fn id(&self) -> RegionId {
        self.id
    }

    /// Returns the semantic region kind.
    #[must_use]
    pub const fn kind(&self) -> RegionKind {
        self.kind
    }

    /// Changes the semantic region kind.
    pub fn set_kind(&mut self, kind: RegionKind) {
        self.kind = kind;
    }

    /// Returns the parent region identity.
    #[must_use]
    pub const fn parent(&self) -> Option<RegionId> {
        self.parent
    }

    /// Assigns the parent region identity.
    ///
    /// This records a structural relationship. It does not update the parent
    /// region's child list automatically because the parent may be stored in a
    /// separate registry.
    pub fn set_parent(&mut self, parent: Option<RegionId>) {
        self.parent = parent;
    }

    /// Returns the child region identities in deterministic insertion order.
    #[must_use]
    pub fn child_regions(&self) -> &[RegionId] {
        &self.child_regions
    }

    /// Returns the number of child regions.
    #[must_use]
    pub fn child_region_count(&self) -> usize {
        self.child_regions.len()
    }

    /// Returns whether the region has no child regions.
    #[must_use]
    pub fn has_no_children(&self) -> bool {
        self.child_regions.is_empty()
    }

    /// Adds a child region reference.
    ///
    /// Duplicate child identities are rejected.
    pub fn add_child_region(&mut self, child: RegionId) -> RegionResult<()> {
        if child == self.id {
            return Err(region_error(
                "a region cannot contain itself as a direct child",
            ));
        }

        if self.child_regions.contains(&child) {
            return Err(region_error(format!(
                "region {} already contains child region {}",
                self.id, child
            )));
        }

        self.child_regions
            .try_reserve(1)
            .map_err(|_| allocation_error("child regions"))?;

        self.child_regions.push(child);
        Ok(())
    }

    /// Removes a child region reference.
    ///
    /// Returns `true` when the child was present.
    pub fn remove_child_region(&mut self, child: RegionId) -> bool {
        if let Some(index) = self.child_regions.iter().position(|id| *id == child) {
            self.child_regions.remove(index);
            true
        } else {
            false
        }
    }

    /// Returns block identities in deterministic program order.
    #[must_use]
    pub fn blocks(&self) -> &[BlockId] {
        &self.blocks
    }

    /// Returns the number of blocks.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Returns whether the region has no blocks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Adds a block reference.
    ///
    /// Duplicate block identities are rejected.
    pub fn add_block(&mut self, block: BlockId) -> RegionResult<()> {
        if self.blocks.contains(&block) {
            return Err(region_error(format!(
                "region {} already contains block {}",
                self.id, block
            )));
        }

        self.blocks
            .try_reserve(1)
            .map_err(|_| allocation_error("region blocks"))?;

        self.blocks.push(block);
        Ok(())
    }

    /// Inserts a block at a specific structural position.
    pub fn insert_block(
        &mut self,
        index: usize,
        block: BlockId,
    ) -> RegionResult<()> {
        if self.blocks.contains(&block) {
            return Err(region_error(format!(
                "region {} already contains block {}",
                self.id, block
            )));
        }

        if index > self.blocks.len() {
            return Err(region_error(format!(
                "block insertion index {} is outside region block range 0..={}",
                index,
                self.blocks.len()
            )));
        }

        self.blocks
            .try_reserve(1)
            .map_err(|_| allocation_error("region blocks"))?;

        self.blocks.insert(index, block);
        Ok(())
    }

    /// Removes a block reference.
    ///
    /// The caller is responsible for maintaining the corresponding block
    /// registry. This method only updates this region's structural list.
    pub fn remove_block(&mut self, block: BlockId) -> bool {
        if let Some(index) = self.blocks.iter().position(|id| *id == block) {
            self.blocks.remove(index);

            if self.entry == Some(block) {
                self.entry = None;
            }

            if self.exit == Some(block) {
                self.exit = None;
            }

            true
        } else {
            false
        }
    }

    /// Returns the entry block.
    #[must_use]
    pub const fn entry(&self) -> Option<BlockId> {
        self.entry
    }

    /// Sets the entry block.
    ///
    /// The block must already belong to the region.
    pub fn set_entry(&mut self, block: BlockId) -> RegionResult<()> {
        if !self.blocks.contains(&block) {
            return Err(region_error(format!(
                "cannot set block {} as region {} entry because it is not a member",
                block, self.id
            )));
        }

        self.entry = Some(block);
        Ok(())
    }

    /// Returns the exit block.
    #[must_use]
    pub const fn exit(&self) -> Option<BlockId> {
        self.exit
    }

    /// Sets the exit block.
    ///
    /// The block must already belong to the region.
    pub fn set_exit(&mut self, block: BlockId) -> RegionResult<()> {
        if !self.blocks.contains(&block) {
            return Err(region_error(format!(
                "cannot set block {} as region {} exit because it is not a member",
                block, self.id
            )));
        }

        self.exit = Some(block);
        Ok(())
    }

    /// Returns region input ports.
    #[must_use]
    pub fn inputs(&self) -> &[RegionQubitPort] {
        &self.inputs
    }

    /// Returns region output ports.
    #[must_use]
    pub fn outputs(&self) -> &[RegionQubitPort] {
        &self.outputs
    }

    /// Adds an input qubit port.
    ///
    /// Duplicate ports are rejected.
    pub fn add_input<Q>(&mut self, qubit: Q) -> RegionResult<()>
    where
        Q: Into<RegionQubitPort>,
    {
        let port = qubit.into();

        if self.inputs.contains(&port) {
            return Err(region_error(format!(
                "region {} already contains input qubit {}",
                self.id,
                port.as_qubit_ref()
            )));
        }

        self.inputs
            .try_reserve(1)
            .map_err(|_| allocation_error("region input ports"))?;

        self.inputs.push(port);
        Ok(())
    }

    /// Adds an output qubit port.
    ///
    /// Duplicate ports are rejected.
    pub fn add_output<Q>(&mut self, qubit: Q) -> RegionResult<()>
    where
        Q: Into<RegionQubitPort>,
    {
        let port = qubit.into();

        if self.outputs.contains(&port) {
            return Err(region_error(format!(
                "region {} already contains output qubit {}",
                self.id,
                port.as_qubit_ref()
            )));
        }

        self.outputs
            .try_reserve(1)
            .map_err(|_| allocation_error("region output ports"))?;

        self.outputs.push(port);
        Ok(())
    }

    /// Removes an input qubit port.
    pub fn remove_input<Q>(&mut self, qubit: Q) -> bool
    where
        Q: Into<RegionQubitPort>,
    {
        let port = qubit.into();

        if let Some(index) = self.inputs.iter().position(|candidate| *candidate == port) {
            self.inputs.remove(index);
            true
        } else {
            false
        }
    }

    /// Removes an output qubit port.
    pub fn remove_output<Q>(&mut self, qubit: Q) -> bool
    where
        Q: Into<RegionQubitPort>,
    {
        let port = qubit.into();

        if let Some(index) = self.outputs.iter().position(|candidate| *candidate == port) {
            self.outputs.remove(index);
            true
        } else {
            false
        }
    }

    /// Returns whether a qubit is referenced by the region input boundary.
    #[must_use]
    pub fn has_input<Q>(&self, qubit: Q) -> bool
    where
        Q: Into<RegionQubitPort> + Copy,
    {
        self.inputs.contains(&qubit.into())
    }

    /// Returns whether a qubit is referenced by the region output boundary.
    #[must_use]
    pub fn has_output<Q>(&self, qubit: Q) -> bool
    where
        Q: Into<RegionQubitPort> + Copy,
    {
        self.outputs.contains(&qubit.into())
    }

    /// Returns whether this region has no explicit parent.
    #[must_use]
    pub const fn is_root_candidate(&self) -> bool {
        self.parent.is_none()
    }

    /// Validates the local structural invariants that can be checked without a
    /// program-wide registry.
    ///
    /// This deliberately does not inspect concrete operations, because those
    /// belong to `operation.rs`/`program.rs`.
    pub fn validate_local(&self) -> RegionResult<()> {
        if self.id == RegionId::new(0) {
            // Zero is a valid identity value. Do not reject it.
        }

        if self.blocks.len() != unique_count(&self.blocks) {
            return Err(region_error(format!(
                "region {} contains duplicate block identities",
                self.id
            )));
        }

        if self.child_regions.len() != unique_count(&self.child_regions) {
            return Err(region_error(format!(
                "region {} contains duplicate child-region identities",
                self.id
            )));
        }

        if self.inputs.len() != unique_count(&self.inputs) {
            return Err(region_error(format!(
                "region {} contains duplicate input ports",
                self.id
            )));
        }

        if self.outputs.len() != unique_count(&self.outputs) {
            return Err(region_error(format!(
                "region {} contains duplicate output ports",
                self.id
            )));
        }

        if let Some(entry) = self.entry {
            if !self.blocks.contains(&entry) {
                return Err(region_error(format!(
                    "region {} entry block {} is not a member of the region",
                    self.id, entry
                )));
            }
        }

        if let Some(exit) = self.exit {
            if !self.blocks.contains(&exit) {
                return Err(region_error(format!(
                    "region {} exit block {} is not a member of the region",
                    self.id, exit
                )));
            }
        }

        if self.entry.is_some() && self.blocks.is_empty() {
            return Err(region_error(format!(
                "region {} has an entry block but no blocks",
                self.id
            )));
        }

        if self.exit.is_some() && self.blocks.is_empty() {
            return Err(region_error(format!(
                "region {} has an exit block but no blocks",
                self.id
            )));
        }

        if self.parent == Some(self.id) {
            return Err(region_error(format!(
                "region {} cannot be its own parent",
                self.id
            )));
        }

        if self.child_regions.contains(&self.id) {
            return Err(region_error(format!(
                "region {} cannot contain itself as a child",
                self.id
            )));
        }

        Ok(())
    }
}

// =============================================================================
// Block
// =============================================================================

/// Canonical ordered block within a region.
///
/// A block stores operation identities rather than concrete operations.
///
/// This allows `operation.rs` to evolve independently while preserving this
/// structural contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    id: BlockId,
    kind: BlockKind,
    parent: Option<RegionId>,
    operations: Vec<OperationId>,
    predecessors: Vec<BlockId>,
    successors: Vec<BlockId>,
    qubits: Vec<RegionQubitPort>,
}

impl Block {
    /// Creates an empty basic block.
    #[must_use]
    pub fn new(id: BlockId, kind: BlockKind) -> Self {
        Self {
            id,
            kind,
            parent: None,
            operations: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
            qubits: Vec::new(),
        }
    }

    /// Creates an ordinary basic block.
    #[must_use]
    pub fn basic(id: BlockId) -> Self {
        Self::new(id, BlockKind::Basic)
    }

    /// Returns the block identity.
    #[must_use]
    pub const fn id(&self) -> BlockId {
        self.id
    }

    /// Returns the block kind.
    #[must_use]
    pub const fn kind(&self) -> BlockKind {
        self.kind
    }

    /// Sets the block kind.
    pub fn set_kind(&mut self, kind: BlockKind) {
        self.kind = kind;
    }

    /// Returns the parent region identity.
    #[must_use]
    pub const fn parent(&self) -> Option<RegionId> {
        self.parent
    }

    /// Sets the parent region identity.
    pub fn set_parent(&mut self, parent: Option<RegionId>) {
        self.parent = parent;
    }

    /// Returns operations in exact insertion/program order.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns the operation count.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the block contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Appends an operation to the block.
    ///
    /// The operation identity must not already occur in the same block.
    pub fn append_operation(
        &mut self,
        operation: OperationId,
    ) -> RegionResult<()> {
        if self.operations.contains(&operation) {
            return Err(region_error(format!(
                "block {} already contains operation {}",
                self.id, operation
            )));
        }

        self.operations
            .try_reserve(1)
            .map_err(|_| allocation_error("block operations"))?;

        self.operations.push(operation);
        Ok(())
    }

    /// Inserts an operation at an explicit position.
    pub fn insert_operation(
        &mut self,
        index: usize,
        operation: OperationId,
    ) -> RegionResult<()> {
        if self.operations.contains(&operation) {
            return Err(region_error(format!(
                "block {} already contains operation {}",
                self.id, operation
            )));
        }

        if index > self.operations.len() {
            return Err(region_error(format!(
                "operation insertion index {} is outside block operation range 0..={}",
                index,
                self.operations.len()
            )));
        }

        self.operations
            .try_reserve(1)
            .map_err(|_| allocation_error("block operations"))?;

        self.operations.insert(index, operation);
        Ok(())
    }

    /// Removes an operation identity.
    ///
    /// Returns `true` when the operation existed.
    pub fn remove_operation(&mut self, operation: OperationId) -> bool {
        if let Some(index) = self.operations.iter().position(|id| *id == operation) {
            self.operations.remove(index);
            true
        } else {
            false
        }
    }

    /// Replaces one operation identity with another.
    ///
    /// The replacement must not already exist elsewhere in the same block.
    pub fn replace_operation(
        &mut self,
        old: OperationId,
        new: OperationId,
    ) -> RegionResult<()> {
        let Some(index) = self.operations.iter().position(|id| *id == old) else {
            return Err(region_error(format!(
                "operation {} is not present in block {}",
                old, self.id
            )));
        };

        if old != new && self.operations.contains(&new) {
            return Err(region_error(format!(
                "replacement operation {} already exists in block {}",
                new, self.id
            )));
        }

        self.operations[index] = new;
        Ok(())
    }

    /// Removes all operations from the block.
    ///
    /// Returns the number of removed operation references.
    pub fn clear_operations(&mut self) -> usize {
        let count = self.operations.len();
        self.operations.clear();
        count
    }

    /// Returns predecessor blocks.
    #[must_use]
    pub fn predecessors(&self) -> &[BlockId] {
        &self.predecessors
    }

    /// Returns successor blocks.
    #[must_use]
    pub fn successors(&self) -> &[BlockId] {
        &self.successors
    }

    /// Adds a predecessor relationship.
    ///
    /// This records structural control-flow connectivity. It does not perform
    /// control-flow analysis.
    pub fn add_predecessor(&mut self, predecessor: BlockId) -> RegionResult<()> {
        if predecessor == self.id {
            return Err(region_error(format!(
                "block {} cannot be its own predecessor",
                self.id
            )));
        }

        if self.predecessors.contains(&predecessor) {
            return Ok(());
        }

        self.predecessors
            .try_reserve(1)
            .map_err(|_| allocation_error("block predecessors"))?;

        self.predecessors.push(predecessor);
        Ok(())
    }

    /// Adds a successor relationship.
    ///
    /// This records structural control-flow connectivity.
    pub fn add_successor(&mut self, successor: BlockId) -> RegionResult<()> {
        if successor == self.id {
            return Err(region_error(format!(
                "block {} cannot be its own successor",
                self.id
            )));
        }

        if self.successors.contains(&successor) {
            return Ok(());
        }

        self.successors
            .try_reserve(1)
            .map_err(|_| allocation_error("block successors"))?;

        self.successors.push(successor);
        Ok(())
    }

    /// Removes a predecessor relationship.
    pub fn remove_predecessor(&mut self, predecessor: BlockId) -> bool {
        if let Some(index) = self.predecessors.iter().position(|id| *id == predecessor) {
            self.predecessors.remove(index);
            true
        } else {
            false
        }
    }

    /// Removes a successor relationship.
    pub fn remove_successor(&mut self, successor: BlockId) -> bool {
        if let Some(index) = self.successors.iter().position(|id| *id == successor) {
            self.successors.remove(index);
            true
        } else {
            false
        }
    }

    /// Returns qubit references explicitly associated with this block.
    ///
    /// These are semantic references only. They do not perform routing or
    /// hardware allocation.
    #[must_use]
    pub fn qubits(&self) -> &[RegionQubitPort] {
        &self.qubits
    }

    /// Adds a qubit reference to the block.
    pub fn add_qubit<Q>(&mut self, qubit: Q) -> RegionResult<()>
    where
        Q: Into<RegionQubitPort>,
    {
        let qubit = qubit.into();

        if self.qubits.contains(&qubit) {
            return Err(region_error(format!(
                "block {} already references qubit {}",
                self.id,
                qubit.as_qubit_ref()
            )));
        }

        self.qubits
            .try_reserve(1)
            .map_err(|_| allocation_error("block qubit references"))?;

        self.qubits.push(qubit);
        Ok(())
    }

    /// Removes a qubit reference.
    pub fn remove_qubit<Q>(&mut self, qubit: Q) -> bool
    where
        Q: Into<RegionQubitPort>,
    {
        let qubit = qubit.into();

        if let Some(index) = self.qubits.iter().position(|candidate| *candidate == qubit) {
            self.qubits.remove(index);
            true
        } else {
            false
        }
    }

    /// Returns whether the block references the supplied qubit.
    #[must_use]
    pub fn references_qubit<Q>(&self, qubit: Q) -> bool
    where
        Q: Into<RegionQubitPort> + Copy,
    {
        self.qubits.contains(&qubit.into())
    }

    /// Validates local block invariants.
    pub fn validate_local(&self) -> RegionResult<()> {
        if self.operations.len() != unique_count(&self.operations) {
            return Err(region_error(format!(
                "block {} contains duplicate operation identities",
                self.id
            )));
        }

        if self.predecessors.len() != unique_count(&self.predecessors) {
            return Err(region_error(format!(
                "block {} contains duplicate predecessor identities",
                self.id
            )));
        }

        if self.successors.len() != unique_count(&self.successors) {
            return Err(region_error(format!(
                "block {} contains duplicate successor identities",
                self.id
            )));
        }

        if self.qubits.len() != unique_count(&self.qubits) {
            return Err(region_error(format!(
                "block {} contains duplicate qubit references",
                self.id
            )));
        }

        if self.predecessors.contains(&self.id) {
            return Err(region_error(format!(
                "block {} contains itself as predecessor",
                self.id
            )));
        }

        if self.successors.contains(&self.id) {
            return Err(region_error(format!(
                "block {} contains itself as successor",
                self.id
            )));
        }

        Ok(())
    }
}

// =============================================================================
// Region graph
// =============================================================================

/// A lightweight structural graph containing region and block identities.
///
/// `RegionGraph` is intentionally an identity-level structure.
///
/// It does not own concrete operations and therefore can be used by
/// `program.rs`, validation, analysis, optimization, routing, or scheduling
/// without creating a dependency on concrete operation semantics.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegionGraph {
    regions: Vec<RegionId>,
    blocks: Vec<BlockId>,
}

impl RegionGraph {
    /// Creates an empty region graph.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            regions: Vec::new(),
            blocks: Vec::new(),
        }
    }

    /// Returns region identities.
    #[must_use]
    pub fn regions(&self) -> &[RegionId] {
        &self.regions
    }

    /// Returns block identities.
    #[must_use]
    pub fn blocks(&self) -> &[BlockId] {
        &self.blocks
    }

    /// Adds a region identity.
    pub fn add_region(&mut self, region: RegionId) -> RegionResult<()> {
        if self.regions.contains(&region) {
            return Err(region_error(format!(
                "region graph already contains region {}",
                region
            )));
        }

        self.regions
            .try_reserve(1)
            .map_err(|_| allocation_error("region graph regions"))?;

        self.regions.push(region);
        Ok(())
    }

    /// Adds a block identity.
    pub fn add_block(&mut self, block: BlockId) -> RegionResult<()> {
        if self.blocks.contains(&block) {
            return Err(region_error(format!(
                "region graph already contains block {}",
                block
            )));
        }

        self.blocks
            .try_reserve(1)
            .map_err(|_| allocation_error("region graph blocks"))?;

        self.blocks.push(block);
        Ok(())
    }

    /// Returns whether the graph contains a region.
    #[must_use]
    pub fn contains_region(&self, region: RegionId) -> bool {
        self.regions.contains(&region)
    }

    /// Returns whether the graph contains a block.
    #[must_use]
    pub fn contains_block(&self, block: BlockId) -> bool {
        self.blocks.contains(&block)
    }

    /// Validates identity uniqueness.
    pub fn validate(&self) -> RegionResult<()> {
        if self.regions.len() != unique_count(&self.regions) {
            return Err(region_error(
                "region graph contains duplicate region identities",
            ));
        }

        if self.blocks.len() != unique_count(&self.blocks) {
            return Err(region_error(
                "region graph contains duplicate block identities",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Utility functions
// =============================================================================

/// Returns the number of unique elements in a slice.
///
/// This intentionally uses equality-based validation rather than a hash set so
/// validation remains deterministic and does not require an additional
/// allocation.
fn unique_count<T>(values: &[T]) -> usize
where
    T: PartialEq,
{
    let mut unique = 0usize;

    for index in 0..values.len() {
        let mut duplicate = false;

        for previous in 0..index {
            if values[previous] == values[index] {
                duplicate = true;
                break;
            }
        }

        if !duplicate {
            unique += 1;
        }
    }

    unique
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::identity::{
        BlockId,
        OperationId,
        RegionId,
    };
    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
        QubitRef,
    };

    #[test]
    fn region_can_be_created_without_blocks() {
        let region = Region::generic(RegionId::new(1));

        assert_eq!(region.id(), RegionId::new(1));
        assert_eq!(region.kind(), RegionKind::Generic);
        assert!(region.is_empty());
        assert_eq!(region.block_count(), 0);
        assert!(region.parent().is_none());
    }

    #[test]
    fn root_region_has_no_parent() {
        let region = Region::root(RegionId::new(1));

        assert!(region.is_root_candidate());
        assert_eq!(region.kind(), RegionKind::Root);
    }

    #[test]
    fn region_can_contain_blocks() {
        let mut region = Region::generic(RegionId::new(1));

        region
            .add_block(BlockId::new(10))
            .expect("first block should be accepted");

        region
            .add_block(BlockId::new(11))
            .expect("second block should be accepted");

        assert_eq!(region.block_count(), 2);
        assert_eq!(
            region.blocks(),
            &[BlockId::new(10), BlockId::new(11)]
        );
    }

    #[test]
    fn duplicate_blocks_are_rejected() {
        let mut region = Region::generic(RegionId::new(1));
        let block = BlockId::new(10);

        region
            .add_block(block)
            .expect("first block should be accepted");

        assert!(region.add_block(block).is_err());
    }

    #[test]
    fn entry_and_exit_must_belong_to_region() {
        let mut region = Region::generic(RegionId::new(1));

        assert!(region.set_entry(BlockId::new(10)).is_err());
        assert!(region.set_exit(BlockId::new(10)).is_err());

        region
            .add_block(BlockId::new(10))
            .expect("block should be accepted");

        region
            .set_entry(BlockId::new(10))
            .expect("entry should be accepted");

        region
            .set_exit(BlockId::new(10))
            .expect("exit should be accepted");

        assert_eq!(region.entry(), Some(BlockId::new(10)));
        assert_eq!(region.exit(), Some(BlockId::new(10)));
    }

    #[test]
    fn child_region_cannot_be_self() {
        let mut region = Region::generic(RegionId::new(1));

        assert!(region.add_child_region(RegionId::new(1)).is_err());
    }

    #[test]
    fn parent_cannot_be_self() {
        let mut region = Region::generic(RegionId::new(1));

        region.set_parent(Some(RegionId::new(1)));

        assert!(region.validate_local().is_err());
    }

    #[test]
    fn region_accepts_logical_qubits() {
        let mut region = Region::generic(RegionId::new(1));
        let qubit = QubitId::new(100);

        region
            .add_input(qubit)
            .expect("logical qubit input should be accepted");

        region
            .add_output(qubit)
            .expect("logical qubit output should be accepted");

        assert_eq!(
            region.inputs(),
            &[RegionQubitPort::Logical(qubit)]
        );

        assert_eq!(
            region.outputs(),
            &[RegionQubitPort::Logical(qubit)]
        );
    }

    #[test]
    fn region_accepts_physical_qubits_when_explicitly_represented() {
        let mut region = Region::generic(RegionId::new(1));
        let qubit = PhysicalQubitId::new(7);

        region
            .add_input(qubit)
            .expect("physical qubit input should be accepted");

        assert_eq!(
            region.inputs(),
            &[RegionQubitPort::Physical(qubit)]
        );
    }

    #[test]
    fn qubit_ref_conversion_preserves_identity_domain() {
        let logical = QubitId::new(5);
        let physical = PhysicalQubitId::new(9);

        let logical_ref: QubitRef = RegionQubitPort::logical(logical).into();
        let physical_ref: QubitRef = RegionQubitPort::physical(physical).into();

        assert_eq!(logical_ref, QubitRef::Logical(logical));
        assert_eq!(physical_ref, QubitRef::Physical(physical));
    }

    #[test]
    fn block_preserves_operation_order() {
        let mut block = Block::basic(BlockId::new(1));

        block
            .append_operation(OperationId::new(10))
            .expect("operation should be accepted");

        block
            .append_operation(OperationId::new(20))
            .expect("operation should be accepted");

        block
            .append_operation(OperationId::new(30))
            .expect("operation should be accepted");

        assert_eq!(
            block.operations(),
            &[
                OperationId::new(10),
                OperationId::new(20),
                OperationId::new(30)
            ]
        );
    }

    #[test]
    fn duplicate_operation_in_same_block_is_rejected() {
        let mut block = Block::basic(BlockId::new(1));
        let operation = OperationId::new(10);

        block
            .append_operation(operation)
            .expect("first operation should be accepted");

        assert!(block.append_operation(operation).is_err());
    }

    #[test]
    fn operation_can_be_inserted_at_specific_position() {
        let mut block = Block::basic(BlockId::new(1));

        block
            .append_operation(OperationId::new(10))
            .expect("operation should be accepted");

        block
            .append_operation(OperationId::new(30))
            .expect("operation should be accepted");

        block
            .insert_operation(1, OperationId::new(20))
            .expect("operation insertion should succeed");

        assert_eq!(
            block.operations(),
            &[
                OperationId::new(10),
                OperationId::new(20),
                OperationId::new(30)
            ]
        );
    }

    #[test]
    fn operation_can_be_replaced() {
        let mut block = Block::basic(BlockId::new(1));

        block
            .append_operation(OperationId::new(10))
            .expect("operation should be accepted");

        block
            .replace_operation(
                OperationId::new(10),
                OperationId::new(20),
            )
            .expect("replacement should succeed");

        assert_eq!(
            block.operations(),
            &[OperationId::new(20)]
        );
    }

    #[test]
    fn block_rejects_self_control_edges() {
        let mut block = Block::basic(BlockId::new(1));

        assert!(block.add_predecessor(BlockId::new(1)).is_err());
        assert!(block.add_successor(BlockId::new(1)).is_err());
    }

    #[test]
    fn block_accepts_control_edges() {
        let mut block = Block::basic(BlockId::new(2));

        block
            .add_predecessor(BlockId::new(1))
            .expect("predecessor should be accepted");

        block
            .add_successor(BlockId::new(3))
            .expect("successor should be accepted");

        assert_eq!(
            block.predecessors(),
            &[BlockId::new(1)]
        );

        assert_eq!(
            block.successors(),
            &[BlockId::new(3)]
        );
    }

    #[test]
    fn block_can_reference_qubits() {
        let mut block = Block::basic(BlockId::new(1));
        let q0 = QubitId::new(0);
        let q1 = QubitId::new(1);

        block
            .add_qubit(q0)
            .expect("q0 should be accepted");

        block
            .add_qubit(q1)
            .expect("q1 should be accepted");

        assert_eq!(
            block.qubits(),
            &[
                RegionQubitPort::Logical(q0),
                RegionQubitPort::Logical(q1)
            ]
        );
    }

    #[test]
    fn local_validation_accepts_valid_region() {
        let mut region = Region::new(
            RegionId::new(1),
            RegionKind::Circuit,
        );

        region
            .add_block(BlockId::new(1))
            .expect("block should be accepted");

        region
            .set_entry(BlockId::new(1))
            .expect("entry should be accepted");

        region
            .set_exit(BlockId::new(1))
            .expect("exit should be accepted");

        region
            .add_input(QubitId::new(0))
            .expect("input should be accepted");

        region
            .add_output(QubitId::new(0))
            .expect("output should be accepted");

        region
            .validate_local()
            .expect("region should validate");
    }

    #[test]
    fn local_validation_accepts_empty_region() {
        let region = Region::generic(RegionId::new(1));

        region
            .validate_local()
            .expect("empty region is structurally valid");
    }

    #[test]
    fn block_local_validation_accepts_valid_block() {
        let mut block = Block::basic(BlockId::new(1));

        block
            .append_operation(OperationId::new(1))
            .expect("operation should be accepted");

        block
            .add_qubit(QubitId::new(0))
            .expect("qubit should be accepted");

        block
            .validate_local()
            .expect("block should validate");
    }

    #[test]
    fn region_graph_rejects_duplicate_regions() {
        let mut graph = RegionGraph::new();

        graph
            .add_region(RegionId::new(1))
            .expect("region should be accepted");

        assert!(graph.add_region(RegionId::new(1)).is_err());
    }

    #[test]
    fn region_graph_tracks_regions_and_blocks() {
        let mut graph = RegionGraph::new();

        graph
            .add_region(RegionId::new(1))
            .expect("region should be accepted");

        graph
            .add_block(BlockId::new(1))
            .expect("block should be accepted");

        assert!(graph.contains_region(RegionId::new(1)));
        assert!(graph.contains_block(BlockId::new(1)));
    }

    #[test]
    fn large_identifier_values_are_not_machine_size_limits() {
        let region = Region::generic(RegionId::new(u64::MAX));
        let block_id = BlockId::new(u64::MAX);
        let operation_id = OperationId::new(u64::MAX);

        assert_eq!(region.id().value(), u64::MAX);

        let mut block = Block::basic(block_id);

        block
            .append_operation(operation_id)
            .expect("maximum representable identity must remain valid");

        assert_eq!(block.operations(), &[operation_id]);
    }

    #[test]
    fn physical_and_logical_qubit_domains_remain_distinct() {
        let logical = RegionQubitPort::logical(QubitId::new(7));
        let physical = RegionQubitPort::physical(PhysicalQubitId::new(7));

        assert_ne!(logical, physical);
        assert_ne!(
            logical.as_qubit_ref(),
            physical.as_qubit_ref()
        );
    }

    #[test]
    fn region_kinds_cover_universal_program_structure() {
        let kinds = [
            RegionKind::Root,
            RegionKind::Generic,
            RegionKind::Function,
            RegionKind::Circuit,
            RegionKind::Pulse,
            RegionKind::Analog,
            RegionKind::Annealing,
            RegionKind::Conditional,
            RegionKind::Loop,
            RegionKind::Repeat,
            RegionKind::Dynamic,
            RegionKind::Logical,
            RegionKind::Classical,
            RegionKind::Hybrid,
            RegionKind::Implementation,
            RegionKind::Extension,
        ];

        assert_eq!(kinds.len(), 16);
    }
}