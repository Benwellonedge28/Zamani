//! Zamani Quantum IR — Program Regions
//!
//! Canonical hardware-independent representation of structured program
//! regions.
//!
//! # Architectural role
//!
//! A `Region` is a structural container inside a `QuantumProgram`.
//!
//! ```text
//! QuantumProgram
//!     └── Region
//!           ├── Block
//!           │     ├── Operation
//!           │     ├── Operation
//!           │     └── ...
//!           ├── Block
//!           └── child Region
//!                 └── ...
//! ```
//!
//! This module owns the structure of a region:
//!
//! - region identity;
//! - region semantic kind;
//! - parent relationship;
//! - child-region references;
//! - ordered block references;
//! - entry block;
//! - exit block;
//! - qubit interface/ports;
//! - local structural validation.
//!
//! This module does NOT own:
//!
//! - concrete blocks;
//! - concrete operations;
//! - operation semantics;
//! - gates;
//! - measurements;
//! - classical expressions;
//! - control-flow algorithms;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware;
//! - calibration;
//! - pulse generation;
//! - simulation;
//! - QEC decoding;
//! - frontend parsing.
//!
//! Those responsibilities belong to their respective IR or downstream
//! subsystems.
//!
//! # Canonical dependencies
//!
//! ```text
//! quantum::ir::identity
//!         │
//!         ├── RegionId
//!         └── BlockId
//!
//! quantum::ir::qubit
//!         │
//!         ├── QubitId
//!         ├── PhysicalQubitId
//!         └── QubitRef
//!
//!                 ▼
//!       quantum::ir::program::region
//!                 │
//!                 ├── Region
//!                 └── RegionQubitPort
//! ```
//!
//! The concrete `Block` type belongs to `program/block.rs`.
//! The concrete `Operation` type belongs to `program/operation.rs`.
//!
//! A region stores their stable identities rather than owning those objects.
//! This avoids recursive ownership and permits the top-level program to use an
//! arena, registry, indexed store, persistent store, or another storage
//! strategy without changing this semantic contract.
//!
//! # Universal quantum-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to any compatible target for which sufficient resources and capabilities
//! exist.
//!
//! A region therefore MUST NOT contain:
//!
//! - a maximum qubit count;
//! - a maximum block count;
//! - a maximum operation count;
//! - a fixed topology;
//! - a fixed hardware architecture;
//! - a vendor-specific device identifier;
//! - a backend-specific instruction;
//! - a scheduler-specific timestamp.
//!
//! Concrete resource limits are policy concerns and belong to the IR limits
//! subsystem or an explicit compilation policy.
//!
//! Hardware capacity belongs to `quantum::hardware`.
//!
//! # Scaling
//!
//! The region representation is intentionally based on dynamically sized
//! collections.
//!
//! There is no semantic constant such as:
//!
//! ```text
//! 64 qubits
//! 4096 blocks
//! 1_000_000 operations
//! ```
//!
//! Such values must never become architectural ceilings.
//!
//! The representation supports any finite region size that the selected
//! compilation environment can represent and that its explicit resource
//! policies permit.
//!
//! "Infinite quantum computers" are not represented as an actually infinite
//! allocation. The semantic model has no fixed architectural ceiling while
//! every concrete IR instance remains finite and resource-checkable.
//!
//! # Determinism
//!
//! Block and child-region collections preserve insertion order.
//!
//! No `HashMap` is used for ordered structural data.
//!
//! No global mutable state is used.
//!
//! No implicit identifier allocation is performed.
//!
//! Therefore the same sequence of API calls produces the same structural
//! ordering.
//!
//! # Atomic mutation
//!
//! Fallible collection growth uses `try_reserve` before mutation.
//!
//! Consequently, an allocation failure does not leave a partially appended
//! element in the region.
//!
//! Structural preconditions are checked before mutation whenever possible.
//!
//! # Parent/child consistency
//!
//! This module stores both:
//!
//! - an optional parent `RegionId`;
//! - an ordered list of child `RegionId`s.
//!
//! Because regions may be stored independently in a program-level registry,
//! this module deliberately does not automatically mutate another `Region`
//! when a parent or child relationship changes.
//!
//! The program/region registry is responsible for maintaining bidirectional
//! graph consistency.
//!
//! Local validation is provided here.
//!
//! Global graph validation belongs to the validation subsystem.
//!
//! # Qubit interface
//!
//! A region may expose logical or physical qubit references at its boundary.
//!
//! Logical qubits use:
//!
//! `quantum::ir::qubit::QubitId`
//!
//! Physical references use:
//!
//! `quantum::ir::qubit::PhysicalQubitId`
//!
//! The region does not decide whether a physical qubit exists, is calibrated,
//! is connected, or is available.
//!
//! Those questions belong to the hardware layer.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! No external crate is required by this module.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::errors::IrSemanticError;
use crate::quantum::ir::identity::{BlockId, RegionId};
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId, QubitRef};

// =============================================================================
// Result type
// =============================================================================

/// Result type for region construction and mutation.
pub type RegionResult<T> = Result<T, IrSemanticError>;

// =============================================================================
// Error construction helpers
// =============================================================================

/// Constructs a canonical IR semantic error associated with a region.
fn region_error<S: Into<String>>(reason: S) -> IrSemanticError {
    IrSemanticError::new("region", reason)
}

/// Converts a failed collection reservation into a canonical IR error.
fn allocation_error(context: &'static str) -> IrSemanticError {
    region_error(format!(
        "unable to reserve memory for {context}"
    ))
}

// =============================================================================
// Region kind
// =============================================================================

/// Semantic classification of an IR region.
///
/// These variants describe the role of a region in a program. They do not
/// describe a hardware instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegionKind {
    /// Root region of a complete program.
    Root,

    /// Generic structured region.
    Generic,

    /// Function body.
    Function,

    /// Gate/circuit-oriented region.
    Circuit,

    /// Pulse-control region.
    Pulse,

    /// Analog quantum region.
    Analog,

    /// Annealing / Ising / QUBO region.
    Annealing,

    /// Conditional region.
    Conditional,

    /// Loop body or loop-associated region.
    Loop,

    /// Repeated-execution region.
    Repeat,

    /// Dynamic-circuit region.
    Dynamic,

    /// Logical/fault-tolerant quantum region.
    Logical,

    /// Classical-only region.
    Classical,

    /// Hybrid quantum/classical region.
    Hybrid,

    /// Implementation region belonging to a higher-level operation.
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
        let name = match self {
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

        formatter.write_str(name)
    }
}

// =============================================================================
// Region boundary qubit port
// =============================================================================

/// A qubit reference exposed through a region boundary.
///
/// A region port identifies a semantic resource reference. It does not allocate
/// or route hardware.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegionQubitPort {
    /// Logical program qubit.
    Logical(QubitId),

    /// Physical target reference.
    ///
    /// Physical references are normally introduced by later compilation
    /// stages. Their presence here does not imply that routing has occurred.
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

    /// Returns the logical qubit when this is a logical port.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::Logical(qubit) => Some(qubit),
            Self::Physical(_) => None,
        }
    }

    /// Returns the physical qubit when this is a physical port.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::Logical(_) => None,
            Self::Physical(qubit) => Some(qubit),
        }
    }

    /// Returns the canonical `QubitRef`.
    #[must_use]
    pub const fn as_qubit_ref(self) -> QubitRef {
        match self {
            Self::Logical(qubit) => QubitRef::Logical(qubit),
            Self::Physical(qubit) => QubitRef::Physical(qubit),
        }
    }

    /// Returns whether this port references a logical qubit.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical(_))
    }

    /// Returns whether this port references a physical qubit.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Physical(_))
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

/// Canonical structured program region.
///
/// A region contains structural references to blocks and child regions.
///
/// It does not own the referenced blocks or operations. The program-level
/// storage layer owns those objects.
///
/// This design avoids recursive ownership and keeps the region representation
/// independent from the storage strategy used by `QuantumProgram`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    /// Stable region identity.
    id: RegionId,

    /// Semantic region classification.
    kind: RegionKind,

    /// Parent region, if any.
    parent: Option<RegionId>,

    /// Ordered block identities.
    blocks: Vec<BlockId>,

    /// Ordered child-region identities.
    child_regions: Vec<RegionId>,

    /// Ordered qubit inputs.
    inputs: Vec<RegionQubitPort>,

    /// Ordered qubit outputs.
    outputs: Vec<RegionQubitPort>,

    /// Entry block.
    entry: Option<BlockId>,

    /// Exit block.
    exit: Option<BlockId>,
}

impl Region {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates an empty region.
    ///
    /// No block is implicitly created.
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

    /// Creates an empty root region.
    #[must_use]
    pub fn root(id: RegionId) -> Self {
        Self::new(id, RegionKind::Root)
    }

    // =========================================================================
    // Identity and kind
    // =========================================================================

    /// Returns the stable region identity.
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
    ///
    /// This operation does not modify blocks, children, or qubit ports.
    pub fn set_kind(&mut self, kind: RegionKind) {
        self.kind = kind;
    }

    // =========================================================================
    // Parent
    // =========================================================================

    /// Returns the parent region identity, if one is recorded.
    #[must_use]
    pub const fn parent(&self) -> Option<RegionId> {
        self.parent
    }

    /// Assigns or clears the parent region reference.
    ///
    /// This method does not modify the parent's child list. A program-level
    /// region registry is responsible for maintaining both sides of the
    /// relationship.
    ///
    /// A region may not be its own parent.
    pub fn set_parent(&mut self, parent: Option<RegionId>) -> RegionResult<()> {
        if parent == Some(self.id) {
            return Err(region_error(
                "a region cannot be its own parent",
            ));
        }

        self.parent = parent;
        Ok(())
    }

    /// Clears the parent relationship.
    pub fn clear_parent(&mut self) {
        self.parent = None;
    }

    // =========================================================================
    // Child regions
    // =========================================================================

    /// Returns child regions in deterministic insertion order.
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

    /// Returns whether the specified region is a direct child.
    #[must_use]
    pub fn contains_child_region(&self, child: RegionId) -> bool {
        self.child_regions.contains(&child)
    }

    /// Adds a child region.
    ///
    /// Duplicate direct-child identities are rejected.
    ///
    /// This method records only the child side of the relationship. The caller
    /// must also set the child's parent through the program-level graph
    /// management API.
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

    /// Removes a direct child-region reference.
    ///
    /// Returns `true` when the reference existed.
    pub fn remove_child_region(&mut self, child: RegionId) -> bool {
        if let Some(index) = self
            .child_regions
            .iter()
            .position(|candidate| *candidate == child)
        {
            self.child_regions.remove(index);
            true
        } else {
            false
        }
    }

    /// Removes all child-region references.
    pub fn clear_child_regions(&mut self) {
        self.child_regions.clear();
    }

    // =========================================================================
    // Blocks
    // =========================================================================

    /// Returns ordered block identities.
    ///
    /// The ordering is semantic program order for the region's block list.
    /// Control-flow successor relationships belong to the block/control-flow
    /// layer.
    #[must_use]
    pub fn blocks(&self) -> &[BlockId] {
        &self.blocks
    }

    /// Returns the number of blocks.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Returns whether the region contains no blocks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Returns whether the region contains the supplied block.
    #[must_use]
    pub fn contains_block(&self, block: BlockId) -> bool {
        self.blocks.contains(&block)
    }

    /// Returns the zero-based position of a block.
    #[must_use]
    pub fn block_position(&self, block: BlockId) -> Option<usize> {
        self.blocks
            .iter()
            .position(|candidate| *candidate == block)
    }

    /// Appends a block to the region.
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

    /// Inserts a block at `index`.
    ///
    /// Existing block order is preserved around the insertion point.
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
                "block insertion index {index} is outside valid range 0..={}",
                self.blocks.len()
            )));
        }

        self.blocks
            .try_reserve(1)
            .map_err(|_| allocation_error("region blocks"))?;

        self.blocks.insert(index, block);

        Ok(())
    }

    /// Removes a block from the region.
    ///
    /// If the removed block is the entry or exit block, the corresponding
    /// boundary reference is cleared.
    ///
    /// The concrete block object is not removed from the program registry.
    pub fn remove_block(&mut self, block: BlockId) -> bool {
        let Some(index) = self
            .blocks
            .iter()
            .position(|candidate| *candidate == block)
        else {
            return false;
        };

        self.blocks.remove(index);

        if self.entry == Some(block) {
            self.entry = None;
        }

        if self.exit == Some(block) {
            self.exit = None;
        }

        true
    }

    /// Removes all block references.
    ///
    /// Entry and exit references are cleared as well.
    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
        self.entry = None;
        self.exit = None;
    }

    // =========================================================================
    // Entry block
    // =========================================================================

    /// Returns the entry block, if one is defined.
    #[must_use]
    pub const fn entry(&self) -> Option<BlockId> {
        self.entry
    }

    /// Sets the entry block.
    ///
    /// The block must already belong to this region.
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

    /// Clears the entry block.
    pub fn clear_entry(&mut self) {
        self.entry = None;
    }

    // =========================================================================
    // Exit block
    // =========================================================================

    /// Returns the exit block, if one is defined.
    #[must_use]
    pub const fn exit(&self) -> Option<BlockId> {
        self.exit
    }

    /// Sets the exit block.
    ///
    /// The block must already belong to this region.
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

    /// Clears the exit block.
    pub fn clear_exit(&mut self) {
        self.exit = None;
    }

    // =========================================================================
    // Region interface: inputs
    // =========================================================================

    /// Returns input qubit ports in deterministic order.
    #[must_use]
    pub fn inputs(&self) -> &[RegionQubitPort] {
        &self.inputs
    }

    /// Returns the number of input ports.
    #[must_use]
    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }

    /// Returns whether the region has no input ports.
    #[must_use]
    pub fn has_no_inputs(&self) -> bool {
        self.inputs.is_empty()
    }

    /// Returns whether the supplied input port already exists.
    #[must_use]
    pub fn has_input(&self, port: RegionQubitPort) -> bool {
        self.inputs.contains(&port)
    }

    /// Adds an input qubit port.
    ///
    /// Duplicate ports are rejected.
    pub fn add_input<P>(&mut self, port: P) -> RegionResult<()>
    where
        P: Into<RegionQubitPort>,
    {
        let port = port.into();

        if self.inputs.contains(&port) {
            return Err(region_error(format!(
                "region {} already contains input port {}",
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

    /// Removes an input port.
    ///
    /// Returns `true` when the port existed.
    pub fn remove_input(&mut self, port: RegionQubitPort) -> bool {
        if let Some(index) = self.inputs.iter().position(|candidate| *candidate == port) {
            self.inputs.remove(index);
            true
        } else {
            false
        }
    }

    /// Removes all input ports.
    pub fn clear_inputs(&mut self) {
        self.inputs.clear();
    }

    // =========================================================================
    // Region interface: outputs
    // =========================================================================

    /// Returns output qubit ports in deterministic order.
    #[must_use]
    pub fn outputs(&self) -> &[RegionQubitPort] {
        &self.outputs
    }

    /// Returns the number of output ports.
    #[must_use]
    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }

    /// Returns whether the region has no output ports.
    #[must_use]
    pub fn has_no_outputs(&self) -> bool {
        self.outputs.is_empty()
    }

    /// Returns whether the supplied output port already exists.
    #[must_use]
    pub fn has_output(&self, port: RegionQubitPort) -> bool {
        self.outputs.contains(&port)
    }

    /// Adds an output qubit port.
    ///
    /// Duplicate ports are rejected.
    pub fn add_output<P>(&mut self, port: P) -> RegionResult<()>
    where
        P: Into<RegionQubitPort>,
    {
        let port = port.into();

        if self.outputs.contains(&port) {
            return Err(region_error(format!(
                "region {} already contains output port {}",
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

    /// Removes an output port.
    ///
    /// Returns `true` when the port existed.
    pub fn remove_output(&mut self, port: RegionQubitPort) -> bool {
        if let Some(index) = self
            .outputs
            .iter()
            .position(|candidate| *candidate == port)
        {
            self.outputs.remove(index);
            true
        } else {
            false
        }
    }

    /// Removes all output ports.
    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
    }

    // =========================================================================
    // Region interface helpers
    // =========================================================================

    /// Adds the same qubit as both an input and output.
    ///
    /// This is useful for regions that operate on an existing qubit without
    /// changing ownership of that semantic resource.
    pub fn add_inout<P>(&mut self, port: P) -> RegionResult<()>
    where
        P: Into<RegionQubitPort> + Copy,
    {
        self.add_input(port)?;
        self.add_output(port)?;

        Ok(())
    }

    /// Returns whether the region has an input/output interface.
    #[must_use]
    pub fn has_interface(&self) -> bool {
        !self.inputs.is_empty() || !self.outputs.is_empty()
    }

    /// Returns all logical qubits appearing in the input interface.
    ///
    /// The returned iterator preserves input-port order.
    pub fn logical_inputs(
        &self,
    ) -> impl Iterator<Item = QubitId> + '_ {
        self.inputs
            .iter()
            .filter_map(|port| port.logical_qubit())
    }

    /// Returns all logical qubits appearing in the output interface.
    ///
    /// The returned iterator preserves output-port order.
    pub fn logical_outputs(
        &self,
    ) -> impl Iterator<Item = QubitId> + '_ {
        self.outputs
            .iter()
            .filter_map(|port| port.logical_qubit())
    }

    /// Returns all physical qubits appearing in the input interface.
    ///
    /// The returned iterator preserves input-port order.
    pub fn physical_inputs(
        &self,
    ) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.inputs
            .iter()
            .filter_map(|port| port.physical_qubit())
    }

    /// Returns all physical qubits appearing in the output interface.
    ///
    /// The returned iterator preserves output-port order.
    pub fn physical_outputs(
        &self,
    ) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.outputs
            .iter()
            .filter_map(|port| port.physical_qubit())
    }

    // =========================================================================
    // Structural predicates
    // =========================================================================

    /// Returns whether the region has both an entry and exit block.
    #[must_use]
    pub const fn has_complete_boundaries(&self) -> bool {
        self.entry.is_some() && self.exit.is_some()
    }

    /// Returns whether the region has exactly one block.
    #[must_use]
    pub fn is_single_block(&self) -> bool {
        self.blocks.len() == 1
    }

    /// Returns whether the region is structurally empty.
    ///
    /// A region is structurally empty when it contains:
    ///
    /// - no blocks;
    /// - no children;
    /// - no inputs;
    /// - no outputs.
    #[must_use]
    pub fn is_structurally_empty(&self) -> bool {
        self.blocks.is_empty()
            && self.child_regions.is_empty()
            && self.inputs.is_empty()
            && self.outputs.is_empty()
            && self.entry.is_none()
            && self.exit.is_none()
    }

    /// Returns the total number of direct structural references held by this
    /// region.
    ///
    /// This is a host-memory count, not a quantum-machine limit.
    #[must_use]
    pub fn structural_reference_count(&self) -> usize {
        self.blocks
            .len()
            .saturating_add(self.child_regions.len())
            .saturating_add(self.inputs.len())
            .saturating_add(self.outputs.len())
            .saturating_add(usize::from(self.parent.is_some()))
            .saturating_add(usize::from(self.entry.is_some()))
            .saturating_add(usize::from(self.exit.is_some()))
    }

    // =========================================================================
    // Local validation
    // =========================================================================

    /// Validates invariants that can be checked using only this region.
    ///
    /// Global checks such as:
    ///
    /// - whether referenced blocks exist;
    /// - whether referenced child regions exist;
    /// - whether parent/child relationships agree globally;
    /// - whether the region participates in a cycle;
    /// - whether block terminators are valid;
    /// - whether operations belong to referenced blocks;
    ///
    /// belong to the global IR validation layer.
    pub fn validate_local(&self) -> RegionResult<()> {
        if self.parent == Some(self.id) {
            return Err(region_error(
                "region cannot reference itself as its parent",
            ));
        }

        if self.child_regions.iter().any(|child| *child == self.id) {
            return Err(region_error(
                "region cannot contain itself as a direct child",
            ));
        }

        if has_duplicates(&self.blocks) {
            return Err(region_error(format!(
                "region {} contains duplicate block identities",
                self.id
            )));
        }

        if has_duplicates(&self.child_regions) {
            return Err(region_error(format!(
                "region {} contains duplicate child-region identities",
                self.id
            )));
        }

        if has_duplicates(&self.inputs) {
            return Err(region_error(format!(
                "region {} contains duplicate input ports",
                self.id
            )));
        }

        if has_duplicates(&self.outputs) {
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

        Ok(())
    }

    // =========================================================================
    // Reservation API
    // =========================================================================

    /// Reserves capacity for additional blocks.
    ///
    /// This method is useful to callers that already know their expected
    /// workload size and want to make allocation policy explicit.
    ///
    /// The supplied amount is a collection capacity request, not a semantic
    /// quantum-machine limit.
    pub fn reserve_blocks(&mut self, additional: usize) -> RegionResult<()> {
        self.blocks
            .try_reserve(additional)
            .map_err(|_| allocation_error("region blocks"))
    }

    /// Reserves capacity for additional child regions.
    pub fn reserve_child_regions(
        &mut self,
        additional: usize,
    ) -> RegionResult<()> {
        self.child_regions
            .try_reserve(additional)
            .map_err(|_| allocation_error("child regions"))
    }

    /// Reserves capacity for additional input ports.
    pub fn reserve_inputs(&mut self, additional: usize) -> RegionResult<()> {
        self.inputs
            .try_reserve(additional)
            .map_err(|_| allocation_error("region input ports"))
    }

    /// Reserves capacity for additional output ports.
    pub fn reserve_outputs(&mut self, additional: usize) -> RegionResult<()> {
        self.outputs
            .try_reserve(additional)
            .map_err(|_| allocation_error("region output ports"))
    }

    // =========================================================================
    // Read-only snapshots
    // =========================================================================

    /// Returns a compact immutable structural view.
    ///
    /// The returned view borrows the region and therefore does not allocate.
    #[must_use]
    pub fn view(&self) -> RegionView<'_> {
        RegionView { region: self }
    }
}

// =============================================================================
// Region view
// =============================================================================

/// Borrowed read-only view over a region.
///
/// This type is intentionally lightweight and allocation-free.
///
/// It is useful for analysis, validation, diagnostics, and compiler passes that
/// must not mutate the region.
#[derive(Debug, Clone, Copy)]
pub struct RegionView<'a> {
    region: &'a Region,
}

impl<'a> RegionView<'a> {
    /// Returns the underlying region identity.
    #[must_use]
    pub const fn id(self) -> RegionId {
        self.region.id()
    }

    /// Returns the region kind.
    #[must_use]
    pub const fn kind(self) -> RegionKind {
        self.region.kind()
    }

    /// Returns the parent.
    #[must_use]
    pub const fn parent(self) -> Option<RegionId> {
        self.region.parent()
    }

    /// Returns the block identities.
    #[must_use]
    pub fn blocks(self) -> &'a [BlockId] {
        self.region.blocks()
    }

    /// Returns the child-region identities.
    #[must_use]
    pub fn child_regions(self) -> &'a [RegionId] {
        self.region.child_regions()
    }

    /// Returns input ports.
    #[must_use]
    pub fn inputs(self) -> &'a [RegionQubitPort] {
        self.region.inputs()
    }

    /// Returns output ports.
    #[must_use]
    pub fn outputs(self) -> &'a [RegionQubitPort] {
        self.region.outputs()
    }

    /// Returns the entry block.
    #[must_use]
    pub const fn entry(self) -> Option<BlockId> {
        self.region.entry()
    }

    /// Returns the exit block.
    #[must_use]
    pub const fn exit(self) -> Option<BlockId> {
        self.region.exit()
    }

    /// Performs local structural validation.
    pub fn validate_local(self) -> RegionResult<()> {
        self.region.validate_local()
    }
}

// =============================================================================
// Duplicate detection
// =============================================================================

/// Returns whether a slice contains duplicate values.
///
/// This implementation deliberately uses pairwise comparison instead of a
/// hash-based collection so validation remains deterministic and requires no
/// additional allocation.
///
/// Validation is not on the hot mutation path; insertion APIs already reject
/// duplicates before mutation.
fn has_duplicates<T>(values: &[T]) -> bool
where
    T: PartialEq,
{
    values
        .iter()
        .enumerate()
        .any(|(index, value)| {
            values[index + 1..]
                .iter()
                .any(|candidate| candidate == value)
        })
}

// =============================================================================
// Default
// =============================================================================

impl Default for Region {
    fn default() -> Self {
        Self::generic(RegionId::new(0))
    }
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for Region {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Region {{ id: {}, kind: {}, blocks: {}, children: {}, inputs: {}, outputs: {}",
            self.id,
            self.kind,
            self.blocks.len(),
            self.child_regions.len(),
            self.inputs.len(),
            self.outputs.len(),
        )?;

        if let Some(entry) = self.entry {
            write!(formatter, ", entry: {entry}")?;
        }

        if let Some(exit) = self.exit {
            write!(formatter, ", exit: {exit}")?;
        }

        if let Some(parent) = self.parent {
            write!(formatter, ", parent: {parent}")?;
        }

        formatter.write_str(" }")
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn region(value: u64) -> Region {
        Region::generic(RegionId::new(value))
    }

    fn block(value: u64) -> BlockId {
        BlockId::new(value)
    }

    fn child(value: u64) -> RegionId {
        RegionId::new(value)
    }

    fn logical(value: usize) -> QubitId {
        QubitId::new(value)
    }

    fn physical(value: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(value)
    }

    #[test]
    fn creates_empty_region() {
        let region = region(1);

        assert_eq!(region.id(), RegionId::new(1));
        assert_eq!(region.kind(), RegionKind::Generic);
        assert_eq!(region.parent(), None);
        assert!(region.blocks().is_empty());
        assert!(region.child_regions().is_empty());
        assert!(region.inputs().is_empty());
        assert!(region.outputs().is_empty());
        assert_eq!(region.entry(), None);
        assert_eq!(region.exit(), None);
    }

    #[test]
    fn preserves_block_order() {
        let mut region = region(1);

        region.add_block(block(10)).expect("block insertion");
        region.add_block(block(20)).expect("block insertion");
        region.add_block(block(30)).expect("block insertion");

        assert_eq!(
            region.blocks(),
            &[block(10), block(20), block(30)]
        );
    }

    #[test]
    fn inserts_block_without_destroying_order() {
        let mut region = region(1);

        region.add_block(block(10)).expect("block insertion");
        region.add_block(block(30)).expect("block insertion");
        region
            .insert_block(1, block(20))
            .expect("block insertion");

        assert_eq!(
            region.blocks(),
            &[block(10), block(20), block(30)]
        );
    }

    #[test]
    fn rejects_duplicate_blocks() {
        let mut region = region(1);

        region.add_block(block(10)).expect("first insertion");

        assert!(region.add_block(block(10)).is_err());
        assert_eq!(region.block_count(), 1);
    }

    #[test]
    fn entry_and_exit_must_belong_to_region() {
        let mut region = region(1);

        assert!(region.set_entry(block(1)).is_err());
        assert!(region.set_exit(block(2)).is_err());

        region.add_block(block(1)).expect("block insertion");
        region.add_block(block(2)).expect("block insertion");

        region.set_entry(block(1)).expect("entry");
        region.set_exit(block(2)).expect("exit");

        assert_eq!(region.entry(), Some(block(1)));
        assert_eq!(region.exit(), Some(block(2)));
    }

    #[test]
    fn removing_boundary_block_clears_boundary() {
        let mut region = region(1);

        region.add_block(block(1)).expect("block insertion");
        region.add_block(block(2)).expect("block insertion");
        region.set_entry(block(1)).expect("entry");
        region.set_exit(block(2)).expect("exit");

        assert!(region.remove_block(block(1)));
        assert!(region.remove_block(block(2)));

        assert_eq!(region.entry(), None);
        assert_eq!(region.exit(), None);
    }

    #[test]
    fn rejects_self_parent() {
        let mut region = region(1);

        assert!(region.set_parent(Some(RegionId::new(1))).is_err());
        assert_eq!(region.parent(), None);
    }

    #[test]
    fn rejects_self_child() {
        let mut region = region(1);

        assert!(region.add_child_region(RegionId::new(1)).is_err());
        assert!(region.child_regions().is_empty());
    }

    #[test]
    fn preserves_child_order() {
        let mut region = region(1);

        region
            .add_child_region(child(2))
            .expect("child insertion");
        region
            .add_child_region(child(3))
            .expect("child insertion");

        assert_eq!(
            region.child_regions(),
            &[child(2), child(3)]
        );
    }

    #[test]
    fn supports_logical_and_physical_ports() {
        let mut region = region(1);

        region
            .add_input(logical(0))
            .expect("logical input");
        region
            .add_output(physical(4))
            .expect("physical output");

        assert_eq!(
            region.inputs(),
            &[RegionQubitPort::Logical(logical(0))]
        );

        assert_eq!(
            region.outputs(),
            &[RegionQubitPort::Physical(physical(4))]
        );
    }

    #[test]
    fn rejects_duplicate_ports() {
        let mut region = region(1);

        region
            .add_input(logical(0))
            .expect("first input");

        assert!(region.add_input(logical(0)).is_err());
        assert_eq!(region.input_count(), 1);

        region
            .add_output(logical(1))
            .expect("first output");

        assert!(region.add_output(logical(1)).is_err());
        assert_eq!(region.output_count(), 1);
    }

    #[test]
    fn inout_adds_both_ports() {
        let mut region = region(1);

        region.add_inout(logical(0)).expect("inout");

        assert_eq!(
            region.inputs(),
            &[RegionQubitPort::Logical(logical(0))]
        );

        assert_eq!(
            region.outputs(),
            &[RegionQubitPort::Logical(logical(0))]
        );
    }

    #[test]
    fn local_validation_accepts_valid_region() {
        let mut region = region(1);

        region.add_block(block(1)).expect("block");
        region.set_entry(block(1)).expect("entry");
        region.set_exit(block(1)).expect("exit");
        region.add_child_region(child(2)).expect("child");
        region.add_input(logical(0)).expect("input");
        region.add_output(logical(0)).expect("output");

        assert!(region.validate_local().is_ok());
    }

    #[test]
    fn display_contains_structural_information() {
        let mut region = region(1);

        region.add_block(block(2)).expect("block");

        let text = region.to_string();

        assert!(text.contains("Region"));
        assert!(text.contains("generic"));
        assert!(text.contains("blocks: 1"));
    }

    #[test]
    fn default_region_is_valid() {
        let region = Region::default();

        assert!(region.validate_local().is_ok());
    }

    #[test]
    fn region_view_is_read_only() {
        let mut region = region(1);

        region.add_block(block(2)).expect("block");

        let view = region.view();

        assert_eq!(view.id(), RegionId::new(1));
        assert_eq!(view.blocks(), &[block(2)]);
        assert!(view.validate_local().is_ok());
    }

    #[test]
    fn structural_reference_count_is_bounded_safely() {
        let mut region = region(1);

        region.add_block(block(1)).expect("block");
        region.add_child_region(child(2)).expect("child");
        region.add_input(logical(0)).expect("input");
        region.add_output(logical(0)).expect("output");
        region.set_parent(Some(RegionId::new(3))).expect("parent");
        region.set_entry(block(1)).expect("entry");
        region.set_exit(block(1)).expect("exit");

        assert_eq!(region.structural_reference_count(), 7);
    }
}