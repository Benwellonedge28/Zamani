//! Zamani Quantum IR — Program Operand Model
//!
//! Canonical, hardware-independent representation of operands used by
//! program-level Quantum IR operations.
//!
//! # Architectural role
//!
//! `program::operand` defines the stable vocabulary for values consumed by
//! and referenced by IR operations.
//!
//! An operand answers:
//!
//! > What semantic IR value/resource does this operation refer to?
//!
//! It does NOT answer:
//!
//! - where a resource physically exists;
//! - which hardware qubit implements a logical qubit;
//! - which backend register stores a classical value;
//! - which DAC/channel implements a pulse;
//! - when an operand is evaluated;
//! - how an operand is optimized;
//! - how routing is performed;
//! - how scheduling is performed;
//! - how a simulator stores quantum state;
//! - how a QPU executes the operation.
//!
//! Those responsibilities belong to downstream IR, compilation, hardware,
//! simulator, routing, scheduling, and backend subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to compatible machines of different sizes and architectures.
//!
//! Consequently, this module contains NO semantic constants such as:
//!
//! ```text
//! MAX_QUBITS = 64
//! MAX_OPERANDS = 32
//! MAX_REGISTERS = 4096
//! ```
//!
//! Operand count is determined by the program and the operation definition.
//!
//! A machine may have one resource, thousands of resources, millions of
//! resources, or another finite amount supported by the target and execution
//! environment.
//!
//! Concrete limits belong to explicit resource/security policies such as
//! `QuantumIrLimits`, not to the semantic operand model.
//!
//! # Canonical identity boundaries
//!
//! Quantum operands use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Physical qubit identity, where required by a later compiled representation,
//! uses:
//!
//! ```text
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Classical operands use:
//!
//! ```text
//! quantum::ir::classical::ClassicalBitId
//! ```
//!
//! Other IR resources use the stable identities from `identity.rs`.
//!
//! This module MUST NOT duplicate any of those identity types.
//!
//! # Semantic versus physical operands
//!
//! The canonical source-level/program-level representation should normally
//! use logical resources:
//!
//! ```text
//! Logical Qubit
//! Classical Bit
//! Symbolic Parameter
//! IR Value
//! ```
//!
//! Physical resources may appear only when a downstream compiled IR explicitly
//! requires them.
//!
//! This distinction prevents routing and hardware concerns from leaking into
//! the canonical semantic program.
//!
//! # Operand categories
//!
//! The universal operand model supports references to:
//!
//! - logical qubits;
//! - physical qubits;
//! - classical bits;
//! - SSA/value identities;
//! - symbolic parameters;
//! - operations;
//! - pulses;
//! - waveforms;
//! - channels;
//! - frames;
//! - schedules;
//! - abstract resources;
//! - capability requirements;
//! - extensions;
//! - functions;
//! - modules;
//! - regions;
//! - blocks;
//! - symbols;
//! - types;
//! - attributes.
//!
//! Not every operation is allowed to consume every operand category.
//!
//! The operand model represents the vocabulary.
//!
//! Operation definitions and whole-program validation determine whether a
//! particular operand is legal for a particular operation.
//!
//! # Operand identity versus literal value
//!
//! An operand is a reference to an IR entity.
//!
//! It is intentionally different from a literal:
//!
//! ```text
//! Operand::Value(ValueId(...))
//! ```
//!
//! means:
//!
//! > use the IR value identified by this `ValueId`
//!
//! whereas a literal such as:
//!
//! ```text
//! 0.3
//! 20ns
//! true
//! ```
//!
//! belongs to the IR value/parameter/expression systems.
//!
//! This separation is important for SSA-like dataflow, symbolic parameters,
//! dynamic control, serialization, hashing, and compiler transformations.
//!
//! # Determinism
//!
//! Operand order is semantically significant.
//!
//! For example:
//!
//! ```text
//! CX(q0, q1)
//! ```
//!
//! is not represented as an unordered set of qubits because operand position
//! may carry semantic meaning.
//!
//! Therefore `OperandList` preserves insertion order.
//!
//! Uniqueness is checked explicitly when required by an operation contract.
//!
//! # Scalability
//!
//! This file does not eagerly allocate storage for a quantum/classical
//! register merely because an operand refers to it.
//!
//! A program can therefore reference:
//!
//! ```text
//! q0
//! q1_000_000
//! qN
//! ```
//!
//! without this module imposing an architectural resource ceiling.
//!
//! The actual representable range is bounded only by the identifier types and
//! host/runtime constraints, while explicit compilation limits are enforced by
//! the appropriate policy layer.
//!
//! # Validation boundary
//!
//! This module validates only local operand invariants that can be established
//! without the enclosing program.
//!
//! It can validate:
//!
//! - operand structure;
//! - operand-list structure;
//! - explicit uniqueness requirements;
//! - invalid local combinations;
//! - overflow-safe collection operations.
//!
//! It cannot determine whether:
//!
//! - a `QubitId` is declared in a particular program;
//! - a `ValueId` dominates its use;
//! - a `ClassicalBitId` belongs to a particular register;
//! - an `OperationId` exists;
//! - a resource is supported by hardware;
//! - a capability exists on a target.
//!
//! Those checks belong to program-wide validation and downstream compilation
//! stages.
//!
//! # Serialization boundary
//!
//! `operand.rs` owns the semantic representation only.
//!
//! Canonical binary/text serialization belongs to `serialization.rs`.
//!
//! No serializer implementation is embedded here.
//!
//! # Hashing boundary
//!
//! Operands derive deterministic equality, ordering and hashing traits.
//!
//! Canonical content hashing remains owned by `hash.rs`.
//!
//! This module does not introduce a second hashing scheme.
//!
//! # Integration contracts
//!
//! `quantum::ir::qubit`
//!     Supplies canonical logical and physical qubit identities.
//!
//! `quantum::ir::classical`
//!     Supplies canonical classical-bit identities.
//!
//! `quantum::ir::identity`
//!     Supplies stable identities for operations, values, parameters, pulses,
//!     waveforms, channels, frames, schedules, resources, capabilities,
//!     extensions and other IR entities.
//!
//! `program::operation`
//!     Consumes `Operand` and `OperandList`.
//!
//! `program::result`
//!     May use `ValueId` and operand references when describing produced
//!     values.
//!
//! `program::block`
//!     Stores operations whose operands reference values in the program.
//!
//! `program::region`
//!     Provides structural scope for operand definitions and uses.
//!
//! `program::program`
//!     Owns the complete namespace in which operand identities are resolved.
//!
//! `validation`
//!     Performs program-wide operand resolution and semantic legality checks.
//!
//! `serialization`
//!     Encodes and decodes operands using the canonical IR schema.
//!
//! `hash`
//!     Includes operands in canonical semantic hashing.
//!
//! `optimization`
//!     May rewrite operand references but must preserve operand semantics.
//!
//! `routing`
//!     May transform logical-qubit placement without redefining
//!     `QubitId`.
//!
//! `scheduling`
//!     May use resource/channel/frame operands but must not change their
//!     semantic identity.
//!
//! `hardware`
//!     Resolves abstract resources into physical implementation details.
//!
//! `frontend`
//!     Lowers source-language references into canonical operands.
//!
//! # Important integration rule
//!
//! This file MUST NOT import:
//!
//! ```text
//! quantum::frontend
//! quantum::optimization
//! quantum::routing
//! quantum::scheduling
//! quantum::hardware
//! quantum::simulator
//! quantum::qec
//! backend implementations
//! ```
//!
//! The dependency direction remains:
//!
//! ```text
//! frontend
//!     │
//!     ▼
//! quantum::ir::program::operand
//!     │
//!     ├── optimization
//!     ├── routing
//!     ├── scheduling
//!     ├── hardware
//!     └── backend
//! ```
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
//! -----------------------------------------------------------------------------
//! Ownership contract
//! -----------------------------------------------------------------------------
//!
//! This file OWNS:
//!
//! - `Operand`;
//! - `OperandList`;
//! - `OperandKind`;
//! - operand-local errors;
//! - operand-local validation;
//! - operand-local deterministic collection behavior.
//!
//! This file DOES NOT OWN:
//!
//! - operation semantics;
//! - result semantics;
//! - program namespaces;
//! - qubit identity definitions;
//! - classical-bit identity definitions;
//! - hardware mappings;
//! - routing;
//! - scheduling;
//! - serialization;
//! - hashing.
//!
//! Once this contract is established, downstream files may evolve internally
//! without requiring this file to be redesigned.
//!
//! -----------------------------------------------------------------------------
//! No unsafe code.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;
use std::ops::Index;

use super::super::classical::ClassicalBitId;
use super::super::identity::{
    AttributeId,
    BlockId,
    CapabilityId,
    ChannelId,
    ExtensionId,
    FrameId,
    FunctionId,
    ModuleId,
    NamespaceId,
    OperationId,
    ParameterId,
    ProgramId,
    PulseId,
    RegionId,
    ResourceId,
    ScheduleId,
    TypeId,
    ValueId,
    WaveformId,
};
use super::super::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Operand result
// =============================================================================

/// Result type used by operand-local constructors and validation.
pub type OperandResult<T> = Result<T, OperandError>;

// =============================================================================
// Operand kind
// =============================================================================

/// Broad semantic classification of an operand.
///
/// This classification is deliberately independent from the concrete
/// `Operand` representation and is useful to validators, analyses and
/// diagnostics that only need to know what namespace an operand belongs to.
///
/// No hardware assumptions are encoded here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OperandKind {
    /// Logical quantum resource.
    LogicalQubit,

    /// Physical quantum resource.
    ///
    /// This should normally occur only in a target-lowered IR.
    PhysicalQubit,

    /// Logical classical bit.
    ClassicalBit,

    /// SSA-like or otherwise named IR value.
    Value,

    /// Symbolic/runtime parameter.
    Parameter,

    /// Previously defined IR operation.
    Operation,

    /// Pulse semantic object.
    Pulse,

    /// Waveform semantic object.
    Waveform,

    /// Abstract control/acquisition channel.
    Channel,

    /// Abstract control frame.
    Frame,

    /// Semantic schedule object.
    Schedule,

    /// Abstract resource requirement/reference.
    Resource,

    /// Capability requirement/reference.
    Capability,

    /// Extensible semantic object.
    Extension,

    /// IR program.
    Program,

    /// IR module.
    Module,

    /// IR namespace.
    Namespace,

    /// IR region.
    Region,

    /// IR block.
    Block,

    /// IR function.
    Function,

    /// IR type declaration.
    Type,

    /// IR attribute declaration.
    Attribute,
}

impl OperandKind {
    /// Returns a stable textual name for diagnostics and serialization
    /// schemas.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogicalQubit => "logical_qubit",
            Self::PhysicalQubit => "physical_qubit",
            Self::ClassicalBit => "classical_bit",
            Self::Value => "value",
            Self::Parameter => "parameter",
            Self::Operation => "operation",
            Self::Pulse => "pulse",
            Self::Waveform => "waveform",
            Self::Channel => "channel",
            Self::Frame => "frame",
            Self::Schedule => "schedule",
            Self::Resource => "resource",
            Self::Capability => "capability",
            Self::Extension => "extension",
            Self::Program => "program",
            Self::Module => "module",
            Self::Namespace => "namespace",
            Self::Region => "region",
            Self::Block => "block",
            Self::Function => "function",
            Self::Type => "type",
            Self::Attribute => "attribute",
        }
    }

    /// Returns whether this operand identifies a quantum resource.
    #[must_use]
    pub const fn is_quantum(self) -> bool {
        matches!(
            self,
            Self::LogicalQubit | Self::PhysicalQubit
        )
    }

    /// Returns whether this operand identifies a classical resource/value.
    #[must_use]
    pub const fn is_classical(self) -> bool {
        matches!(
            self,
            Self::ClassicalBit | Self::Value | Self::Parameter
        )
    }

    /// Returns whether this operand identifies a control/resource object.
    #[must_use]
    pub const fn is_resource_related(self) -> bool {
        matches!(
            self,
            Self::Channel
                | Self::Frame
                | Self::Schedule
                | Self::Resource
                | Self::Capability
        )
    }

    /// Returns whether this operand identifies program structure.
    #[must_use]
    pub const fn is_structural(self) -> bool {
        matches!(
            self,
            Self::Program
                | Self::Module
                | Self::Namespace
                | Self::Region
                | Self::Block
                | Self::Function
        )
    }
}

// =============================================================================
// Operand
// =============================================================================

/// Canonical program-level IR operand.
///
/// An `Operand` is a strongly typed reference to a semantic IR entity.
///
/// The enum intentionally does not contain raw integers, strings, pointers,
/// machine addresses, or backend-specific handles.
///
/// This prevents accidental confusion between:
///
/// ```text
/// q0
/// c0
/// value0
/// parameter0
/// operation0
/// ```
///
/// and arbitrary integer values.
///
/// # Logical versus physical quantum operands
///
/// `LogicalQubit` is the normal canonical-program representation.
///
/// `PhysicalQubit` is available because a later target-lowered IR may need to
/// make physical placement explicit.
///
/// The presence of `PhysicalQubit` in this vocabulary does NOT make physical
/// qubits part of the source-level semantic contract.
///
/// # Literal values
///
/// Literal values do not belong directly in `Operand`.
///
/// Use the canonical value/parameter representation and reference the
/// resulting `ValueId` or `ParameterId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Operand {
    /// Reference to a logical Zamani qubit.
    LogicalQubit(QubitId),

    /// Reference to a physical qubit in a target-lowered representation.
    PhysicalQubit(PhysicalQubitId),

    /// Reference to a logical classical bit.
    ClassicalBit(ClassicalBitId),

    /// Reference to an IR-produced or declared value.
    Value(ValueId),

    /// Reference to a symbolic/runtime parameter.
    Parameter(ParameterId),

    /// Reference to another IR operation.
    Operation(OperationId),

    /// Reference to a semantic pulse object.
    Pulse(PulseId),

    /// Reference to a semantic waveform object.
    Waveform(WaveformId),

    /// Reference to an abstract control/acquisition channel.
    Channel(ChannelId),

    /// Reference to an abstract control frame.
    Frame(FrameId),

    /// Reference to a semantic schedule.
    Schedule(ScheduleId),

    /// Reference to an abstract resource.
    Resource(ResourceId),

    /// Reference to a capability requirement.
    Capability(CapabilityId),

    /// Reference to an extensible semantic object.
    Extension(ExtensionId),

    /// Reference to a complete IR program.
    Program(ProgramId),

    /// Reference to an IR module.
    Module(ModuleId),

    /// Reference to an IR namespace.
    Namespace(NamespaceId),

    /// Reference to an IR region.
    Region(RegionId),

    /// Reference to an IR block.
    Block(BlockId),

    /// Reference to an IR function.
    Function(FunctionId),

    /// Reference to an IR type declaration.
    Type(TypeId),

    /// Reference to an IR attribute declaration.
    Attribute(AttributeId),
}

impl Operand {
    /// Returns the semantic kind of this operand.
    #[must_use]
    pub const fn kind(self) -> OperandKind {
        match self {
            Self::LogicalQubit(_) => OperandKind::LogicalQubit,
            Self::PhysicalQubit(_) => OperandKind::PhysicalQubit,
            Self::ClassicalBit(_) => OperandKind::ClassicalBit,
            Self::Value(_) => OperandKind::Value,
            Self::Parameter(_) => OperandKind::Parameter,
            Self::Operation(_) => OperandKind::Operation,
            Self::Pulse(_) => OperandKind::Pulse,
            Self::Waveform(_) => OperandKind::Waveform,
            Self::Channel(_) => OperandKind::Channel,
            Self::Frame(_) => OperandKind::Frame,
            Self::Schedule(_) => OperandKind::Schedule,
            Self::Resource(_) => OperandKind::Resource,
            Self::Capability(_) => OperandKind::Capability,
            Self::Extension(_) => OperandKind::Extension,
            Self::Program(_) => OperandKind::Program,
            Self::Module(_) => OperandKind::Module,
            Self::Namespace(_) => OperandKind::Namespace,
            Self::Region(_) => OperandKind::Region,
            Self::Block(_) => OperandKind::Block,
            Self::Function(_) => OperandKind::Function,
            Self::Type(_) => OperandKind::Type,
            Self::Attribute(_) => OperandKind::Attribute,
        }
    }

    /// Returns whether this operand is a logical quantum operand.
    #[must_use]
    pub const fn is_logical_qubit(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether this operand is a physical quantum operand.
    #[must_use]
    pub const fn is_physical_qubit(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns whether this operand is any quantum operand.
    #[must_use]
    pub const fn is_quantum(self) -> bool {
        self.kind().is_quantum()
    }

    /// Returns whether this operand is classical.
    #[must_use]
    pub const fn is_classical(self) -> bool {
        self.kind().is_classical()
    }

    /// Returns whether this operand is a resource/control reference.
    #[must_use]
    pub const fn is_resource_related(self) -> bool {
        self.kind().is_resource_related()
    }

    /// Returns whether this operand is a structural program reference.
    #[must_use]
    pub const fn is_structural(self) -> bool {
        self.kind().is_structural()
    }

    /// Returns the logical qubit if this operand is one.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the physical qubit if this operand is one.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the classical bit if this operand is one.
    #[must_use]
    pub const fn classical_bit(self) -> Option<ClassicalBitId> {
        match self {
            Self::ClassicalBit(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the value identity if this operand is one.
    #[must_use]
    pub const fn value(self) -> Option<ValueId> {
        match self {
            Self::Value(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the parameter identity if this operand is one.
    #[must_use]
    pub const fn parameter(self) -> Option<ParameterId> {
        match self {
            Self::Parameter(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the operation identity if this operand is one.
    #[must_use]
    pub const fn operation(self) -> Option<OperationId> {
        match self {
            Self::Operation(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the pulse identity if this operand is one.
    #[must_use]
    pub const fn pulse(self) -> Option<PulseId> {
        match self {
            Self::Pulse(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the waveform identity if this operand is one.
    #[must_use]
    pub const fn waveform(self) -> Option<WaveformId> {
        match self {
            Self::Waveform(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the channel identity if this operand is one.
    #[must_use]
    pub const fn channel(self) -> Option<ChannelId> {
        match self {
            Self::Channel(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the frame identity if this operand is one.
    #[must_use]
    pub const fn frame(self) -> Option<FrameId> {
        match self {
            Self::Frame(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the schedule identity if this operand is one.
    #[must_use]
    pub const fn schedule(self) -> Option<ScheduleId> {
        match self {
            Self::Schedule(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the resource identity if this operand is one.
    #[must_use]
    pub const fn resource(self) -> Option<ResourceId> {
        match self {
            Self::Resource(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the capability identity if this operand is one.
    #[must_use]
    pub const fn capability(self) -> Option<CapabilityId> {
        match self {
            Self::Capability(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the extension identity if this operand is one.
    #[must_use]
    pub const fn extension(self) -> Option<ExtensionId> {
        match self {
            Self::Extension(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the program identity if this operand is one.
    #[must_use]
    pub const fn program(self) -> Option<ProgramId> {
        match self {
            Self::Program(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the module identity if this operand is one.
    #[must_use]
    pub const fn module(self) -> Option<ModuleId> {
        match self {
            Self::Module(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the namespace identity if this operand is one.
    #[must_use]
    pub const fn namespace(self) -> Option<NamespaceId> {
        match self {
            Self::Namespace(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the region identity if this operand is one.
    #[must_use]
    pub const fn region(self) -> Option<RegionId> {
        match self {
            Self::Region(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the block identity if this operand is one.
    #[must_use]
    pub const fn block(self) -> Option<BlockId> {
        match self {
            Self::Block(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the function identity if this operand is one.
    #[must_use]
    pub const fn function(self) -> Option<FunctionId> {
        match self {
            Self::Function(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the type identity if this operand is one.
    #[must_use]
    pub const fn type_id(self) -> Option<TypeId> {
        match self {
            Self::Type(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the attribute identity if this operand is one.
    #[must_use]
    pub const fn attribute(self) -> Option<AttributeId> {
        match self {
            Self::Attribute(id) => Some(id),
            _ => None,
        }
    }

    /// Returns whether this operand refers to an operation with the supplied
    /// identity.
    ///
    /// This is useful when validating operation-reference graphs.
    #[must_use]
    pub const fn references_operation(
        self,
        operation: OperationId,
    ) -> bool {
        match self {
            Self::Operation(candidate) => candidate.value() == operation.value(),
            _ => false,
        }
    }
}

// =============================================================================
// Operand conversions
// =============================================================================

impl From<QubitId> for Operand {
    fn from(id: QubitId) -> Self {
        Self::LogicalQubit(id)
    }
}

impl From<PhysicalQubitId> for Operand {
    fn from(id: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(id)
    }
}

impl From<ClassicalBitId> for Operand {
    fn from(id: ClassicalBitId) -> Self {
        Self::ClassicalBit(id)
    }
}

impl From<ValueId> for Operand {
    fn from(id: ValueId) -> Self {
        Self::Value(id)
    }
}

impl From<ParameterId> for Operand {
    fn from(id: ParameterId) -> Self {
        Self::Parameter(id)
    }
}

impl From<OperationId> for Operand {
    fn from(id: OperationId) -> Self {
        Self::Operation(id)
    }
}

impl From<PulseId> for Operand {
    fn from(id: PulseId) -> Self {
        Self::Pulse(id)
    }
}

impl From<WaveformId> for Operand {
    fn from(id: WaveformId) -> Self {
        Self::Waveform(id)
    }
}

impl From<ChannelId> for Operand {
    fn from(id: ChannelId) -> Self {
        Self::Channel(id)
    }
}

impl From<FrameId> for Operand {
    fn from(id: FrameId) -> Self {
        Self::Frame(id)
    }
}

impl From<ScheduleId> for Operand {
    fn from(id: ScheduleId) -> Self {
        Self::Schedule(id)
    }
}

impl From<ResourceId> for Operand {
    fn from(id: ResourceId) -> Self {
        Self::Resource(id)
    }
}

impl From<CapabilityId> for Operand {
    fn from(id: CapabilityId) -> Self {
        Self::Capability(id)
    }
}

impl From<ExtensionId> for Operand {
    fn from(id: ExtensionId) -> Self {
        Self::Extension(id)
    }
}

impl From<ProgramId> for Operand {
    fn from(id: ProgramId) -> Self {
        Self::Program(id)
    }
}

impl From<ModuleId> for Operand {
    fn from(id: ModuleId) -> Self {
        Self::Module(id)
    }
}

impl From<NamespaceId> for Operand {
    fn from(id: NamespaceId) -> Self {
        Self::Namespace(id)
    }
}

impl From<RegionId> for Operand {
    fn from(id: RegionId) -> Self {
        Self::Region(id)
    }
}

impl From<BlockId> for Operand {
    fn from(id: BlockId) -> Self {
        Self::Block(id)
    }
}

impl From<FunctionId> for Operand {
    fn from(id: FunctionId) -> Self {
        Self::Function(id)
    }
}

impl From<TypeId> for Operand {
    fn from(id: TypeId) -> Self {
        Self::Type(id)
    }
}

impl From<AttributeId> for Operand {
    fn from(id: AttributeId) -> Self {
        Self::Attribute(id)
    }
}

// =============================================================================
// Operand display
// =============================================================================

impl fmt::Display for Operand {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::LogicalQubit(id) => write!(formatter, "{id}"),
            Self::PhysicalQubit(id) => write!(formatter, "{id}"),
            Self::ClassicalBit(id) => write!(formatter, "{id}"),
            Self::Value(id) => write!(formatter, "{id}"),
            Self::Parameter(id) => write!(formatter, "{id}"),
            Self::Operation(id) => write!(formatter, "{id}"),
            Self::Pulse(id) => write!(formatter, "{id}"),
            Self::Waveform(id) => write!(formatter, "{id}"),
            Self::Channel(id) => write!(formatter, "{id}"),
            Self::Frame(id) => write!(formatter, "{id}"),
            Self::Schedule(id) => write!(formatter, "{id}"),
            Self::Resource(id) => write!(formatter, "{id}"),
            Self::Capability(id) => write!(formatter, "{id}"),
            Self::Extension(id) => write!(formatter, "{id}"),
            Self::Program(id) => write!(formatter, "{id}"),
            Self::Module(id) => write!(formatter, "{id}"),
            Self::Namespace(id) => write!(formatter, "{id}"),
            Self::Region(id) => write!(formatter, "{id}"),
            Self::Block(id) => write!(formatter, "{id}"),
            Self::Function(id) => write!(formatter, "{id}"),
            Self::Type(id) => write!(formatter, "{id}"),
            Self::Attribute(id) => write!(formatter, "{id}"),
        }
    }
}

// =============================================================================
// Operand errors
// =============================================================================

/// Errors local to operand construction and manipulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperandError {
    /// An operand list exceeded the representable host collection size.
    CollectionSizeOverflow,

    /// A duplicate operand was found where uniqueness was explicitly
    /// required.
    DuplicateOperand {
        operand: Operand,
    },

    /// A duplicate logical qubit was found.
    DuplicateLogicalQubit {
        qubit: QubitId,
    },

    /// A duplicate physical qubit was found.
    DuplicatePhysicalQubit {
        qubit: PhysicalQubitId,
    },

    /// A duplicate classical bit was found.
    DuplicateClassicalBit {
        bit: ClassicalBitId,
    },

    /// An operation referenced itself in a context that forbids
    /// self-reference.
    SelfReferentialOperation {
        operation: OperationId,
    },

    /// The requested index is outside the operand list.
    IndexOutOfBounds {
        index: usize,
        len: usize,
    },
}

impl fmt::Display for OperandError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::CollectionSizeOverflow => {
                write!(
                    formatter,
                    "operand collection size exceeds the host representable collection size"
                )
            }

            Self::DuplicateOperand { operand } => {
                write!(
                    formatter,
                    "duplicate operand {operand}"
                )
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "duplicate logical qubit operand {qubit}"
                )
            }

            Self::DuplicatePhysicalQubit { qubit } => {
                write!(
                    formatter,
                    "duplicate physical qubit operand {qubit}"
                )
            }

            Self::DuplicateClassicalBit { bit } => {
                write!(
                    formatter,
                    "duplicate classical bit operand {bit}"
                )
            }

            Self::SelfReferentialOperation { operation } => {
                write!(
                    formatter,
                    "operation {operation} cannot contain itself as an operand"
                )
            }

            Self::IndexOutOfBounds { index, len } => {
                write!(
                    formatter,
                    "operand index {index} is outside operand list of length {len}"
                )
            }
        }
    }
}

impl std::error::Error for OperandError {}

// =============================================================================
// Operand list
// =============================================================================

/// Ordered collection of operation operands.
///
/// `OperandList` intentionally uses `Vec<Operand>` because operand order can
/// be semantically significant.
///
/// It does not impose an artificial maximum operand count.
///
/// Examples of valid conceptual operations include:
///
/// ```text
/// X(q0)
/// CX(q0, q1)
/// global_operation(q0, q1, ..., qN)
/// distributed_operation(node0, q0, node1, q1, ...)
/// ```
///
/// The actual operation contract determines whether a particular number and
/// kind of operands are valid.
///
/// # Determinism
///
/// Iteration order is insertion order.
///
/// # Scalability
///
/// The collection grows only as the operation's actual operand list grows.
/// No fixed quantum-machine size is encoded here.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct OperandList {
    operands: Vec<Operand>,
}

impl OperandList {
    /// Creates an empty operand list.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an operand list from an iterator.
    ///
    /// This constructor preserves operand order and does not silently remove
    /// duplicates.
    #[must_use]
    pub fn from_iter<I>(operands: I) -> Self
    where
        I: IntoIterator<Item = Operand>,
    {
        Self {
            operands: operands.into_iter().collect(),
        }
    }

    /// Returns the number of operands.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operands.len()
    }

    /// Returns whether the list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operands.is_empty()
    }

    /// Returns the operand at `index`.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Operand> {
        self.operands.get(index)
    }

    /// Returns the operand at `index` as a copied value.
    pub fn get_copy(
        &self,
        index: usize,
    ) -> OperandResult<Operand> {
        self.get(index)
            .copied()
            .ok_or(OperandError::IndexOutOfBounds {
                index,
                len: self.len(),
            })
    }

    /// Returns the first operand.
    #[must_use]
    pub fn first(&self) -> Option<&Operand> {
        self.operands.first()
    }

    /// Returns the last operand.
    #[must_use]
    pub fn last(&self) -> Option<&Operand> {
        self.operands.last()
    }

    /// Returns the underlying ordered slice.
    #[must_use]
    pub fn as_slice(&self) -> &[Operand] {
        &self.operands
    }

    /// Returns an iterator over operands in semantic order.
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, Operand> {
        self.operands.iter()
    }

    /// Returns a mutable iterator.
    ///
    /// Mutation is intentionally exposed only through an explicit mutable
    /// borrow of the collection.
    pub fn iter_mut(
        &mut self,
    ) -> std::slice::IterMut<'_, Operand> {
        self.operands.iter_mut()
    }

    /// Appends an operand.
    ///
    /// This method does not reject duplicates because some operations can
    /// legitimately reference the same semantic resource multiple times.
    ///
    /// Operation-specific validation determines whether duplicates are legal.
    pub fn push(
        &mut self,
        operand: Operand,
    ) {
        self.operands.push(operand);
    }

    /// Attempts to append an operand while explicitly checking whether the
    /// host vector can grow.
    ///
    /// This method is useful at hostile/untrusted input boundaries.
    pub fn try_push(
        &mut self,
        operand: Operand,
    ) -> OperandResult<()> {
        if self.operands.len() == self.operands.capacity() {
            // `Vec::try_reserve` is stable and avoids treating allocation
            // failure as an implicit semantic condition.
            self.operands
                .try_reserve(1)
                .map_err(|_| OperandError::CollectionSizeOverflow)?;
        }

        self.operands.push(operand);

        Ok(())
    }

    /// Appends many operands after reserving their required capacity.
    ///
    /// No fixed maximum is imposed.
    pub fn try_extend<I>(
        &mut self,
        operands: I,
    ) -> OperandResult<()>
    where
        I: IntoIterator<Item = Operand>,
    {
        let iterator = operands.into_iter();

        let (lower, upper) = iterator.size_hint();

        let reserve_amount = upper.unwrap_or(lower);

        if reserve_amount > 0 {
            self.operands
                .try_reserve(reserve_amount)
                .map_err(|_| OperandError::CollectionSizeOverflow)?;
        }

        for operand in iterator {
            self.operands.push(operand);
        }

        Ok(())
    }

    /// Inserts an operand at a semantic position.
    ///
    /// This operation may shift later operands and should therefore be used
    /// only by transformations that intentionally change operand ordering.
    pub fn insert(
        &mut self,
        index: usize,
        operand: Operand,
    ) -> OperandResult<()> {
        if index > self.operands.len() {
            return Err(OperandError::IndexOutOfBounds {
                index,
                len: self.operands.len(),
            });
        }

        if self.operands.len() == self.operands.capacity() {
            self.operands
                .try_reserve(1)
                .map_err(|_| OperandError::CollectionSizeOverflow)?;
        }

        self.operands.insert(index, operand);

        Ok(())
    }

    /// Removes an operand at `index`.
    pub fn remove(
        &mut self,
        index: usize,
    ) -> OperandResult<Operand> {
        if index >= self.operands.len() {
            return Err(OperandError::IndexOutOfBounds {
                index,
                len: self.operands.len(),
            });
        }

        Ok(self.operands.remove(index))
    }

    /// Replaces the operand at `index`.
    pub fn replace(
        &mut self,
        index: usize,
        operand: Operand,
    ) -> OperandResult<Operand> {
        if index >= self.operands.len() {
            return Err(OperandError::IndexOutOfBounds {
                index,
                len: self.operands.len(),
            });
        }

        Ok(std::mem::replace(
            &mut self.operands[index],
            operand,
        ))
    }

    /// Clears all operands.
    pub fn clear(&mut self) {
        self.operands.clear();
    }

    /// Returns the number of logical-qubit operands.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        self.operands
            .iter()
            .filter(|operand| operand.is_logical_qubit())
            .count()
    }

    /// Returns the number of physical-qubit operands.
    #[must_use]
    pub fn physical_qubit_count(&self) -> usize {
        self.operands
            .iter()
            .filter(|operand| operand.is_physical_qubit())
            .count()
    }

    /// Returns the number of quantum operands.
    #[must_use]
    pub fn quantum_operand_count(&self) -> usize {
        self.operands
            .iter()
            .filter(|operand| operand.is_quantum())
            .count()
    }

    /// Returns the number of classical operands.
    #[must_use]
    pub fn classical_operand_count(&self) -> usize {
        self.operands
            .iter()
            .filter(|operand| operand.is_classical())
            .count()
    }

    /// Returns the first logical qubit operand.
    #[must_use]
    pub fn first_logical_qubit(&self) -> Option<QubitId> {
        self.operands.iter().find_map(|operand| {
            operand.logical_qubit()
        })
    }

    /// Returns all logical qubits in operand order.
    ///
    /// This allocates only the returned result, not storage hidden inside the
    /// operand collection itself.
    #[must_use]
    pub fn logical_qubits(&self) -> Vec<QubitId> {
        self.operands
            .iter()
            .filter_map(|operand| operand.logical_qubit())
            .collect()
    }

    /// Returns all physical qubits in operand order.
    #[must_use]
    pub fn physical_qubits(&self) -> Vec<PhysicalQubitId> {
        self.operands
            .iter()
            .filter_map(|operand| operand.physical_qubit())
            .collect()
    }

    /// Returns whether the list contains the supplied operand.
    #[must_use]
    pub fn contains(
        &self,
        operand: Operand,
    ) -> bool {
        self.operands.contains(&operand)
    }

    /// Returns the number of occurrences of an operand.
    #[must_use]
    pub fn count(
        &self,
        operand: Operand,
    ) -> usize {
        self.operands
            .iter()
            .filter(|candidate| **candidate == operand)
            .count()
    }

    /// Returns whether every operand has a unique identity.
    ///
    /// This is a generic identity uniqueness check. Some quantum operations
    /// may intentionally permit repeated operands, so callers must choose
    /// whether this invariant is appropriate.
    #[must_use]
    pub fn has_unique_operands(&self) -> bool {
        let mut seen = BTreeSet::new();

        self.operands
            .iter()
            .all(|operand| seen.insert(*operand))
    }

    /// Validates that no operand occurs more than once.
    pub fn validate_unique_operands(
        &self,
    ) -> OperandResult<()> {
        let mut seen = BTreeSet::new();

        for operand in &self.operands {
            if !seen.insert(*operand) {
                return Err(OperandError::DuplicateOperand {
                    operand: *operand,
                });
            }
        }

        Ok(())
    }

    /// Validates that logical qubits occur at most once.
    ///
    /// Non-qubit operands are ignored.
    pub fn validate_unique_logical_qubits(
        &self,
    ) -> OperandResult<()> {
        let mut seen = BTreeSet::new();

        for operand in &self.operands {
            if let Some(qubit) = operand.logical_qubit() {
                if !seen.insert(qubit) {
                    return Err(
                        OperandError::DuplicateLogicalQubit { qubit },
                    );
                }
            }
        }

        Ok(())
    }

    /// Validates that physical qubits occur at most once.
    ///
    /// Non-physical-qubit operands are ignored.
    pub fn validate_unique_physical_qubits(
        &self,
    ) -> OperandResult<()> {
        let mut seen = BTreeSet::new();

        for operand in &self.operands {
            if let Some(qubit) = operand.physical_qubit() {
                if !seen.insert(qubit) {
                    return Err(
                        OperandError::DuplicatePhysicalQubit { qubit },
                    );
                }
            }
        }

        Ok(())
    }

    /// Validates that classical-bit operands occur at most once.
    ///
    /// Non-classical-bit operands are ignored.
    pub fn validate_unique_classical_bits(
        &self,
    ) -> OperandResult<()> {
        let mut seen = BTreeSet::new();

        for operand in &self.operands {
            if let Some(bit) = operand.classical_bit() {
                if !seen.insert(bit) {
                    return Err(
                        OperandError::DuplicateClassicalBit { bit },
                    );
                }
            }
        }

        Ok(())
    }

    /// Validates that no operand contains the supplied operation as a
    /// self-reference.
    pub fn validate_no_self_reference(
        &self,
        operation: OperationId,
    ) -> OperandResult<()> {
        if self
            .operands
            .iter()
            .any(|operand| operand.references_operation(operation))
        {
            return Err(
                OperandError::SelfReferentialOperation { operation },
            );
        }

        Ok(())
    }

    /// Validates all generic local invariants.
    ///
    /// This intentionally does not validate program-wide namespace membership.
    pub fn validate(&self) -> OperandResult<()> {
        // An operand is a strongly typed enum, so there is no invalid enum
        // discriminant reachable through safe Rust. The explicit method is
        // retained as the stable local-validation boundary for future
        // operand-local invariants.
        Ok(())
    }

    /// Returns an iterator over only logical-qubit operands.
    pub fn logical_qubit_iter(
        &self,
    ) -> impl Iterator<Item = QubitId> + '_ {
        self.operands
            .iter()
            .filter_map(|operand| operand.logical_qubit())
    }

    /// Returns an iterator over only classical-bit operands.
    pub fn classical_bit_iter(
        &self,
    ) -> impl Iterator<Item = ClassicalBitId> + '_ {
        self.operands
            .iter()
            .filter_map(|operand| operand.classical_bit())
    }

    /// Returns an iterator over only value operands.
    pub fn value_iter(
        &self,
    ) -> impl Iterator<Item = ValueId> + '_ {
        self.operands.iter().filter_map(|operand| operand.value())
    }

    /// Returns an iterator over only parameter operands.
    pub fn parameter_iter(
        &self,
    ) -> impl Iterator<Item = ParameterId> + '_ {
        self.operands
            .iter()
            .filter_map(|operand| operand.parameter())
    }

    /// Returns an iterator over only operation operands.
    pub fn operation_iter(
        &self,
    ) -> impl Iterator<Item = OperationId> + '_ {
        self.operands
            .iter()
            .filter_map(|operand| operand.operation())
    }

    /// Returns an iterator over all quantum operands.
    pub fn quantum_iter(
        &self,
    ) -> impl Iterator<Item = Operand> + '_ {
        self.operands
            .iter()
            .copied()
            .filter(|operand| operand.is_quantum())
    }

    /// Returns an iterator over all resource-related operands.
    pub fn resource_iter(
        &self,
    ) -> impl Iterator<Item = Operand> + '_ {
        self.operands
            .iter()
            .copied()
            .filter(|operand| operand.is_resource_related())
    }
}

impl From<Vec<Operand>> for OperandList {
    fn from(operands: Vec<Operand>) -> Self {
        Self { operands }
    }
}

impl From<OperandList> for Vec<Operand> {
    fn from(list: OperandList) -> Self {
        list.operands
    }
}

impl AsRef<[Operand]> for OperandList {
    fn as_ref(&self) -> &[Operand] {
        self.as_slice()
    }
}

impl Index<usize> for OperandList {
    type Output = Operand;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        &self.operands[index]
    }
}

impl IntoIterator for OperandList {
    type Item = Operand;
    type IntoIter = std::vec::IntoIter<Operand>;

    fn into_iter(self) -> Self::IntoIter {
        self.operands.into_iter()
    }
}

impl<'a> IntoIterator for &'a OperandList {
    type Item = &'a Operand;
    type IntoIter = std::slice::Iter<'a, Operand>;

    fn into_iter(self) -> Self::IntoIter {
        self.operands.iter()
    }
}

impl<'a> IntoIterator for &'a mut OperandList {
    type Item = &'a mut Operand;
    type IntoIter = std::slice::IterMut<'a, Operand>;

    fn into_iter(self) -> Self::IntoIter {
        self.operands.iter_mut()
    }
}

// =============================================================================
// Specialized constructors
// =============================================================================

impl OperandList {
    /// Creates an operand list containing one logical qubit.
    #[must_use]
    pub fn single_qubit(
        qubit: QubitId,
    ) -> Self {
        Self::from_iter([Operand::LogicalQubit(qubit)])
    }

    /// Creates an operand list containing two logical qubits.
    ///
    /// This is a convenience constructor only; it is NOT a universal
    /// two-qubit assumption.
    #[must_use]
    pub fn two_qubits(
        first: QubitId,
        second: QubitId,
    ) -> Self {
        Self::from_iter([
            Operand::LogicalQubit(first),
            Operand::LogicalQubit(second),
        ])
    }

    /// Creates an operand list from logical qubits.
    ///
    /// The number of qubits is determined by the iterator and is not
    /// artificially bounded.
    pub fn from_qubits<I>(
        qubits: I,
    ) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::from_iter(
            qubits
                .into_iter()
                .map(Operand::LogicalQubit),
        )
    }

    /// Creates an operand list from classical bits.
    pub fn from_classical_bits<I>(
        bits: I,
    ) -> Self
    where
        I: IntoIterator<Item = ClassicalBitId>,
    {
        Self::from_iter(
            bits.into_iter()
                .map(Operand::ClassicalBit),
        )
    }

    /// Creates an operand list from IR values.
    pub fn from_values<I>(
        values: I,
    ) -> Self
    where
        I: IntoIterator<Item = ValueId>,
    {
        Self::from_iter(
            values.into_iter()
                .map(Operand::Value),
        )
    }

    /// Creates an operand list from symbolic parameters.
    pub fn from_parameters<I>(
        parameters: I,
    ) -> Self
    where
        I: IntoIterator<Item = ParameterId>,
    {
        Self::from_iter(
            parameters
                .into_iter()
                .map(Operand::Parameter),
        )
    }
}

// =============================================================================
// Operand validation helpers
// =============================================================================

/// Validates that all operands are logical qubits and that they are unique.
///
/// This is useful for operations whose local contract requires a unique set
/// of logical qubits.
///
/// It does not impose a maximum number of qubits.
pub fn validate_unique_logical_qubit_operands(
    operands: &OperandList,
) -> OperandResult<()> {
    let mut seen = BTreeSet::new();

    for operand in operands {
        let Some(qubit) = operand.logical_qubit() else {
            continue;
        };

        if !seen.insert(qubit) {
            return Err(
                OperandError::DuplicateLogicalQubit { qubit },
            );
        }
    }

    Ok(())
}

/// Validates that every operand is a logical qubit.
///
/// Empty lists are accepted because arity is an operation-level concern.
pub fn validate_logical_qubit_operands(
    operands: &OperandList,
) -> OperandResult<()> {
    for operand in operands {
        if !operand.is_logical_qubit() {
            return Err(OperandError::DuplicateOperand {
                operand: *operand,
            });
        }
    }

    Ok(())
}

/// Returns all logical qubits referenced by an operand list in operand order.
///
/// Duplicate qubits are retained because this function is a projection, not
/// a uniqueness validator.
#[must_use]
pub fn logical_qubits(
    operands: &OperandList,
) -> Vec<QubitId> {
    operands.logical_qubits()
}

/// Returns all classical bits referenced by an operand list in operand order.
#[must_use]
pub fn classical_bits(
    operands: &OperandList,
) -> Vec<ClassicalBitId> {
    operands
        .classical_bit_iter()
        .collect()
}

/// Returns all IR values referenced by an operand list in operand order.
#[must_use]
pub fn values(
    operands: &OperandList,
) -> Vec<ValueId> {
    operands.value_iter().collect()
}

/// Returns all symbolic parameters referenced by an operand list in operand
/// order.
#[must_use]
pub fn parameters(
    operands: &OperandList,
) -> Vec<ParameterId> {
    operands.parameter_iter().collect()
}

/// Returns all referenced operations in operand order.
#[must_use]
pub fn operations(
    operands: &OperandList,
) -> Vec<OperationId> {
    operands.operation_iter().collect()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_qubit_uses_canonical_qubit_id() {
        let qubit = QubitId::new(42);
        let operand = Operand::from(qubit);

        assert_eq!(
            operand.kind(),
            OperandKind::LogicalQubit
        );

        assert_eq!(
            operand.logical_qubit(),
            Some(qubit)
        );
    }

    #[test]
    fn physical_and_logical_qubits_are_distinct() {
        let logical = Operand::from(QubitId::new(7));
        let physical = Operand::from(PhysicalQubitId::new(7));

        assert_ne!(logical, physical);
        assert!(logical.is_logical_qubit());
        assert!(physical.is_physical_qubit());
    }

    #[test]
    fn operand_order_is_preserved() {
        let q0 = QubitId::new(0);
        let q1 = QubitId::new(1);

        let operands = OperandList::from_qubits([q1, q0]);

        assert_eq!(
            operands.logical_qubits(),
            vec![q1, q0]
        );
    }

    #[test]
    fn duplicate_logical_qubits_are_detectable() {
        let q0 = QubitId::new(0);

        let operands = OperandList::from_qubits([q0, q0]);

        assert!(matches!(
            operands.validate_unique_logical_qubits(),
            Err(OperandError::DuplicateLogicalQubit { qubit })
                if qubit == q0
        ));
    }

    #[test]
    fn different_operand_kinds_do_not_collide() {
        let q0 = Operand::from(QubitId::new(0));
        let c0 = Operand::from(ClassicalBitId::new(0));
        let v0 = Operand::from(ValueId::new(0));
        let p0 = Operand::from(ParameterId::new(0));

        assert_ne!(q0, c0);
        assert_ne!(q0, v0);
        assert_ne!(q0, p0);
        assert_ne!(c0, v0);
        assert_ne!(c0, p0);
        assert_ne!(v0, p0);
    }

    #[test]
    fn operation_self_reference_is_detectable() {
        let operation = OperationId::new(10);

        let operands = OperandList::from_iter([
            Operand::Operation(operation),
        ]);

        assert!(matches!(
            operands.validate_no_self_reference(operation),
            Err(
                OperandError::SelfReferentialOperation {
                    operation: found
                }
            ) if found == operation
        ));
    }

    #[test]
    fn operation_different_reference_is_allowed() {
        let operation = OperationId::new(10);
        let other = OperationId::new(11);

        let operands = OperandList::from_iter([
            Operand::Operation(other),
        ]);

        assert!(
            operands
                .validate_no_self_reference(operation)
                .is_ok()
        );
    }

    #[test]
    fn unique_operand_validation_is_deterministic() {
        let operands = OperandList::from_iter([
            Operand::from(QubitId::new(3)),
            Operand::from(QubitId::new(1)),
            Operand::from(QubitId::new(2)),
        ]);

        assert!(
            operands
                .validate_unique_operands()
                .is_ok()
        );
    }

    #[test]
    fn resource_categories_are_classified() {
        assert!(
            Operand::from(ChannelId::new(1))
                .is_resource_related()
        );

        assert!(
            Operand::from(FrameId::new(1))
                .is_resource_related()
        );

        assert!(
            Operand::from(ResourceId::new(1))
                .is_resource_related()
        );

        assert!(
            Operand::from(CapabilityId::new(1))
                .is_resource_related()
        );
    }

    #[test]
    fn structural_categories_are_classified() {
        assert!(
            Operand::from(ProgramId::new(1))
                .is_structural()
        );

        assert!(
            Operand::from(ModuleId::new(1))
                .is_structural()
        );

        assert!(
            Operand::from(RegionId::new(1))
                .is_structural()
        );

        assert!(
            Operand::from(BlockId::new(1))
                .is_structural()
        );
    }

    #[test]
    fn typed_accessors_return_only_matching_namespace() {
        let operand = Operand::from(QubitId::new(5));

        assert_eq!(
            operand.logical_qubit(),
            Some(QubitId::new(5))
        );

        assert_eq!(
            operand.classical_bit(),
            None
        );

        assert_eq!(
            operand.value(),
            None
        );

        assert_eq!(
            operand.parameter(),
            None
        );
    }

    #[test]
    fn operand_list_supports_large_logical_namespaces_without_eager_register_storage() {
        let large_id = QubitId::new(usize::MAX);

        let operands = OperandList::single_qubit(large_id);

        assert_eq!(
            operands.first_logical_qubit(),
            Some(large_id)
        );
    }

    #[test]
    fn list_mutation_preserves_semantic_order() {
        let q0 = QubitId::new(0);
        let q1 = QubitId::new(1);
        let q2 = QubitId::new(2);

        let mut operands =
            OperandList::from_qubits([q0, q2]);

        operands
            .insert(
                1,
                Operand::from(q1),
            )
            .expect("valid insertion index");

        assert_eq!(
            operands.logical_qubits(),
            vec![q0, q1, q2]
        );
    }

    #[test]
    fn replacement_returns_previous_operand() {
        let q0 = QubitId::new(0);
        let q1 = QubitId::new(1);

        let mut operands =
            OperandList::single_qubit(q0);

        let previous = operands
            .replace(
                0,
                Operand::from(q1),
            )
            .expect("valid operand index");

        assert_eq!(
            previous,
            Operand::from(q0)
        );

        assert_eq!(
            operands[0],
            Operand::from(q1)
        );
    }

    #[test]
    fn empty_operand_list_is_valid_locally() {
        let operands = OperandList::new();

        assert!(operands.is_empty());
        assert_eq!(operands.len(), 0);
        assert!(operands.validate().is_ok());
    }

    #[test]
    fn from_iter_preserves_duplicate_operands() {
        let q0 = QubitId::new(0);

        let operands = OperandList::from_qubits([q0, q0]);

        assert_eq!(
            operands.count(Operand::from(q0)),
            2
        );
    }

    #[test]
    fn display_is_stable() {
        assert_eq!(
            Operand::from(QubitId::new(3)).to_string(),
            "q3"
        );

        assert_eq!(
            Operand::from(ClassicalBitId::new(4)).to_string(),
            "c4"
        );

        assert_eq!(
            Operand::from(ValueId::new(5)).to_string(),
            "value5"
        );

        assert_eq!(
            Operand::from(OperationId::new(6)).to_string(),
            "op6"
        );
    }

    #[test]
    fn classification_names_are_stable() {
        assert_eq!(
            OperandKind::LogicalQubit.as_str(),
            "logical_qubit"
        );

        assert_eq!(
            OperandKind::PhysicalQubit.as_str(),
            "physical_qubit"
        );

        assert_eq!(
            OperandKind::ClassicalBit.as_str(),
            "classical_bit"
        );
    }
}