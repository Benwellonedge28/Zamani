//! Zamani Quantum IR — Universal Quantum Instruction Model
//!
//! This module defines the canonical instruction-level semantic vocabulary for
//! Zamani Quantum IR.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! This file answers:
//!
//!     "What kind of quantum/hybrid instruction does the program mean?"
//!
//! It does NOT answer:
//!
//! - which physical machine executes the instruction;
//! - which physical qubit is selected;
//! - which hardware channel is selected;
//! - which native instruction is selected;
//! - which calibration is applied;
//! - how routing is performed;
//! - how scheduling is performed;
//! - how a pulse is synthesized;
//! - how a waveform is sampled;
//! - how a backend executes the instruction;
//! - how a simulator represents quantum state;
//! - how QEC decodes a syndrome;
//! - how an optimization pass transforms the instruction;
//! - how Zamani source syntax is parsed.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//!     Zamani source
//!          │
//!          ▼
//!       frontend
//!          │
//!          ▼
//!   canonical Quantum IR
//!          │
//!          ├── optimization
//!          ├── routing
//!          ├── scheduling
//!          ├── hardware
//!          ├── QEC
//!          ├── simulator
//!          └── backend
//!
//! `instruction.rs` MUST remain independent of all of those downstream
//! implementations.
//!
//! ============================================================================
//! UNIVERSAL-PROGRAM PRINCIPLE
//! ============================================================================
//!
//! A Zamani quantum program is written once at the semantic level.
//!
//! The instruction representation therefore contains no architectural machine
//! size.
//!
//! It must work for:
//!
//! - one qubit;
//! - small quantum processors;
//! - large quantum processors;
//! - distributed quantum systems;
//! - logical/fault-tolerant systems;
//! - pulse-controlled systems;
//! - analog systems;
//! - hybrid quantum/classical systems;
//! - future quantum architectures.
//!
//! No instruction variant assumes:
//!
//! - a fixed qubit count;
//! - a fixed register size;
//! - a fixed operand count;
//! - a fixed topology;
//! - a fixed hardware vendor;
//! - a fixed pulse technology;
//! - a fixed gate set.
//!
//! Concrete resource limits are policy, hardware, runtime, or compiler
//! concerns. They are never encoded as semantic limits here.
//!
//! ============================================================================
//! STANDARD GATES VS UNIVERSAL INSTRUCTIONS
//! ============================================================================
//!
//! `gate.rs` owns the canonical standard logical gate vocabulary.
//!
//! `instruction.rs` deliberately does NOT duplicate `GateKind`.
//!
//! A standard gate is represented by:
//!
//!     Instruction::Gate(Gate)
//!
//! Future/vendor/custom instructions are represented by the extensible
//! instruction variants and/or extension references.
//!
//! Therefore adding a new quantum architecture does not require modifying the
//! standard gate enum.
//!
//! ============================================================================
//! CANONICAL QUBIT IDENTITY
//! ============================================================================
//!
//! Logical quantum operands MUST use:
//!
//!     quantum::ir::qubit::QubitId
//!
//! Physical qubits are not ordinary semantic instruction operands.
//!
//! Logical-to-physical placement belongs to mapping/routing.
//!
//! ============================================================================
//! PULSE-LEVEL CONTROL
//! ============================================================================
//!
//! A Zamani program such as:
//!
//!     fn x_gate(q) {
//!         pulse(amp=0.3, dur=20ns)
//!     }
//!
//! is represented at this layer by a pulse instruction referring to semantic
//! pulse resources.
//!
//! This file does NOT select:
//!
//! - DAC;
//! - ADC;
//! - physical drive line;
//! - oscillator;
//! - carrier;
//! - sample rate;
//! - physical channel;
//! - calibration implementation.
//!
//! Those decisions belong to pulse, hardware, scheduling and backend layers.
//!
//! ============================================================================
//! DYNAMIC CIRCUITS
//! ============================================================================
//!
//! Dynamic execution is represented explicitly.
//!
//! Example:
//!
//!     measure(q0) -> c0
//!
//!     if c0 == 1 {
//!         x(q1)
//!     }
//!
//! This module stores references and semantic relationships. It does not
//! execute the classical condition.
//!
//! ============================================================================
//! NO RECURSIVE INSTRUCTION OBJECTS
//! ============================================================================
//!
//! Conditional instructions reference an existing `OperationId` instead of
//! containing another `Instruction` recursively.
//!
//! This avoids:
//!
//! - recursive object construction;
//! - accidental infinite structures;
//! - unnecessary heap nesting;
//! - serialization ambiguity.
//!
//! Program/region structures own the actual operation graph.
//!
//! ============================================================================
//! EXTENSIBILITY
//! ============================================================================
//!
//! Unknown/future instruction kinds must not silently become no-ops.
//!
//! `ExtensionId` provides an explicit extensibility boundary.
//!
//! An extension can be resolved by the dialect/extension subsystem while the
//! canonical instruction remains structurally valid.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! This module deliberately has no external dependencies.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! `quantum::ir::qubit`
//!     Supplies canonical `QubitId`.
//!
//! `quantum::ir::gate`
//!     Supplies standard logical `Gate` semantics.
//!
//! `quantum::ir::measurement`
//!     Supplies canonical measurement semantics.
//!
//! `quantum::ir::classical`
//!     Supplies logical classical-bit references.
//!
//! `quantum::ir::identity`
//!     Supplies stable resource identities.
//!
//! `quantum::ir::pulse`
//!     Owns pulse semantics referenced by `PulseId`.
//!
//! `quantum::ir::waveform`
//!     Owns waveform semantics referenced by `WaveformId`.
//!
//! `quantum::ir::channel`
//!     Owns abstract channel semantics referenced by `ChannelId`.
//!
//! `quantum::ir::frame`
//!     Owns frame semantics referenced by `FrameId`.
//!
//! `quantum::ir::schedule`
//!     Owns schedule semantics referenced by `ScheduleId`.
//!
//! `quantum::ir::resource`
//!     Owns resource requirements referenced by `ResourceId`.
//!
//! `quantum::ir::capability`
//!     Owns capability requirements referenced by `CapabilityId`.
//!
//! `quantum::ir::extension`
//!     Owns extension definitions referenced by `ExtensionId`.
//!
//! `quantum::ir::operation`
//!     Owns the universal operation container. It may embed/reference this
//!     instruction model.
//!
//! `quantum::ir::program`
//!     Owns program-level ordering, regions and namespaces.
//!
//! `quantum::ir::validation`
//!     Performs whole-program validation.
//!
//! `quantum::ir::serialization`
//!     Owns canonical persistence.
//!
//! `quantum::ir::hash`
//!     Owns canonical content hashing.
//!
//! `quantum::ir::provenance`
//!     Owns transformation lineage.
//!
//! ============================================================================
//! FILE-COMPLETION GUARANTEE
//! ============================================================================
//!
//! This file owns:
//!
//! - instruction classification;
//! - instruction identity;
//! - instruction operands;
//! - standard-gate instruction representation;
//! - measurement instruction representation;
//! - reset instruction representation;
//! - barrier instruction representation;
//! - delay instruction representation;
//! - pulse instruction references;
//! - waveform instruction references;
//! - channel instruction references;
//! - frame instruction references;
//! - conditional instruction references;
//! - classical feedback references;
//! - logical instruction references;
//! - analog instruction references;
//! - annealing instruction references;
//! - distributed instruction references;
//! - resource/capability requirements;
//! - extension references;
//! - local invariants;
//! - deterministic accessors;
//! - local tests.
//!
//! Later IR files should consume this contract rather than changing the
//! semantic meaning of these types.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::classical::ClassicalBitId;
use crate::quantum::ir::gate::Gate;
use crate::quantum::ir::identity::{
    CapabilityId,
    ChannelId,
    ExtensionId,
    FrameId,
    OperationId,
    PulseId,
    ResourceId,
    ScheduleId,
    WaveformId,
};
use crate::quantum::ir::measurement::Measurement;
use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// SCHEMA
// ============================================================================

/// Stable semantic schema identifier for the instruction layer.
pub const INSTRUCTION_SCHEMA_ID: &str = "zamani.quantum.ir.quantum.instruction";

/// Major semantic version of the instruction contract.
///
/// This is intentionally separate from the complete Quantum IR version.
pub const INSTRUCTION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result returned by instruction construction and local validation.
pub type InstructionResult<T> = Result<T, InstructionError>;

// ============================================================================
// ERROR MODEL
// ============================================================================

/// Errors produced by local instruction construction/validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionError {
    /// The instruction identity is invalid according to the local contract.
    InvalidOperationId,

    /// A gate instruction contains an invalid logical-qubit collection.
    InvalidGateOperands,

    /// A measurement instruction does not contain its required measurement
    /// payload.
    MissingMeasurement,

    /// A reset instruction must target at least one logical qubit.
    EmptyReset,

    /// A barrier instruction must target at least one logical qubit.
    EmptyBarrier,

    /// Duplicate logical-qubit operands were supplied where uniqueness is
    /// required.
    DuplicateQubit {
        /// The duplicated logical qubit.
        qubit: QubitId,
    },

    /// A classical destination was repeated.
    DuplicateClassicalBit {
        /// The duplicated classical bit.
        bit: ClassicalBitId,
    },

    /// A conditional instruction does not contain a target operation.
    MissingConditionalTarget,

    /// A conditional instruction directly references itself.
    ConditionalSelfReference {
        /// The operation that attempted the self-reference.
        operation: OperationId,
    },

    /// A resource requirement contains a duplicate identity.
    DuplicateResource {
        /// The duplicated resource identity.
        resource: ResourceId,
    },

    /// A capability requirement contains a duplicate identity.
    DuplicateCapability {
        /// The duplicated capability identity.
        capability: CapabilityId,
    },

    /// An extension reference was duplicated.
    DuplicateExtension {
        /// The duplicated extension identity.
        extension: ExtensionId,
    },

    /// An instruction contains structurally inconsistent data.
    InvalidStructure {
        /// Stable explanation of the local invariant violation.
        message: &'static str,
    },
}

impl fmt::Display for InstructionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidOperationId => {
                write!(formatter, "invalid operation identity")
            }

            Self::InvalidGateOperands => {
                write!(formatter, "invalid gate instruction operands")
            }

            Self::MissingMeasurement => {
                write!(formatter, "measurement instruction requires measurement semantics")
            }

            Self::EmptyReset => {
                write!(formatter, "reset instruction requires at least one logical qubit")
            }

            Self::EmptyBarrier => {
                write!(formatter, "barrier instruction requires at least one logical qubit")
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    formatter,
                    "instruction contains duplicate logical qubit {qubit}"
                )
            }

            Self::DuplicateClassicalBit { bit } => {
                write!(
                    formatter,
                    "instruction contains duplicate classical bit {bit}"
                )
            }

            Self::MissingConditionalTarget => {
                write!(
                    formatter,
                    "conditional instruction requires a target operation"
                )
            }

            Self::ConditionalSelfReference { operation } => {
                write!(
                    formatter,
                    "conditional instruction cannot reference operation {operation} itself"
                )
            }

            Self::DuplicateResource { resource } => {
                write!(
                    formatter,
                    "instruction contains duplicate resource requirement {resource}"
                )
            }

            Self::DuplicateCapability { capability } => {
                write!(
                    formatter,
                    "instruction contains duplicate capability requirement {capability}"
                )
            }

            Self::DuplicateExtension { extension } => {
                write!(
                    formatter,
                    "instruction contains duplicate extension reference {extension}"
                )
            }

            Self::InvalidStructure { message } => {
                write!(
                    formatter,
                    "invalid instruction structure: {message}"
                )
            }
        }
    }
}

impl std::error::Error for InstructionError {}

// ============================================================================
// INSTRUCTION CLASS
// ============================================================================

/// Broad semantic classification of an instruction.
///
/// This is intentionally smaller than [`InstructionKind`].
///
/// Consumers that only need coarse classification should use this type instead
/// of matching every instruction variant.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum InstructionClass {
    /// Standard logical quantum operation.
    Quantum,

    /// Measurement operation.
    Measurement,

    /// State reset/reinitialization.
    Reset,

    /// Synchronization operation.
    Synchronization,

    /// Temporal operation.
    Timing,

    /// Pulse-level control.
    Pulse,

    /// Waveform-level reference.
    Waveform,

    /// Frame-level control.
    Frame,

    /// Classical control/feedback.
    Classical,

    /// Logical/fault-tolerant operation.
    Logical,

    /// Analog/Hamiltonian operation.
    Analog,

    /// Annealing/Ising/QUBO operation.
    Annealing,

    /// Distributed quantum operation.
    Distributed,

    /// Explicit resource operation/reference.
    Resource,

    /// Explicit scheduling reference.
    Schedule,

    /// Extensible/vendor/future instruction.
    Extension,
}

impl fmt::Display for InstructionClass {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let name = match self {
            Self::Quantum => "quantum",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::Synchronization => "synchronization",
            Self::Timing => "timing",
            Self::Pulse => "pulse",
            Self::Waveform => "waveform",
            Self::Frame => "frame",
            Self::Classical => "classical",
            Self::Logical => "logical",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Distributed => "distributed",
            Self::Resource => "resource",
            Self::Schedule => "schedule",
            Self::Extension => "extension",
        };

        formatter.write_str(name)
    }
}

// ============================================================================
// INSTRUCTION KIND
// ============================================================================

/// Stable semantic kind of a quantum/hybrid instruction.
///
/// This enum is deliberately broader than the standard gate vocabulary.
///
/// Standard gates are represented by [`InstructionKind::Gate`].
///
/// Future/vendor instructions use the extensibility variants rather than
/// continually expanding the standard gate enum.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum InstructionKind {
    /// Standard logical gate.
    Gate,

    /// Measurement.
    Measurement,

    /// Logical reset.
    Reset,

    /// Barrier/synchronization.
    Barrier,

    /// Semantic delay.
    Delay,

    /// Pulse execution/reference.
    Pulse,

    /// Waveform reference.
    Waveform,

    /// Frame operation.
    Frame,

    /// Channel operation/reference.
    Channel,

    /// Classical feedback/control operation.
    ClassicalFeedback,

    /// Explicit conditional operation reference.
    Conditional,

    /// Logical/fault-tolerant operation.
    Logical,

    /// Analog/Hamiltonian evolution.
    Analog,

    /// Annealing/Ising/QUBO evolution.
    Annealing,

    /// Distributed quantum operation.
    Distributed,

    /// Resource requirement/reference.
    Resource,

    /// Schedule reference.
    Schedule,

    /// Extensible operation.
    Extension,
}

impl InstructionKind {
    /// Returns the broad semantic class.
    #[must_use]
    pub const fn class(self) -> InstructionClass {
        match self {
            Self::Gate => InstructionClass::Quantum,
            Self::Measurement => InstructionClass::Measurement,
            Self::Reset => InstructionClass::Reset,
            Self::Barrier => InstructionClass::Synchronization,
            Self::Delay => InstructionClass::Timing,
            Self::Pulse => InstructionClass::Pulse,
            Self::Waveform => InstructionClass::Waveform,
            Self::Frame => InstructionClass::Frame,
            Self::Channel => InstructionClass::Pulse,
            Self::ClassicalFeedback => InstructionClass::Classical,
            Self::Conditional => InstructionClass::Classical,
            Self::Logical => InstructionClass::Logical,
            Self::Analog => InstructionClass::Analog,
            Self::Annealing => InstructionClass::Annealing,
            Self::Distributed => InstructionClass::Distributed,
            Self::Resource => InstructionClass::Resource,
            Self::Schedule => InstructionClass::Schedule,
            Self::Extension => InstructionClass::Extension,
        }
    }

    /// Returns whether this instruction directly changes quantum state.
    #[must_use]
    pub const fn is_quantum_state_instruction(self) -> bool {
        matches!(
            self,
            Self::Gate
                | Self::Measurement
                | Self::Reset
                | Self::Pulse
                | Self::Logical
                | Self::Analog
                | Self::Annealing
                | Self::Distributed
        )
    }

    /// Returns whether this instruction can produce a measurement result.
    #[must_use]
    pub const fn can_measure(self) -> bool {
        matches!(
            self,
            Self::Measurement
                | Self::Gate
                | Self::Logical
                | Self::Distributed
        )
    }

    /// Returns whether the instruction is a semantic marker rather than a
    /// quantum-state transformation.
    #[must_use]
    pub const fn is_marker(self) -> bool {
        matches!(
            self,
            Self::Barrier
                | Self::Delay
                | Self::Schedule
                | Self::Resource
        )
    }
}

// ============================================================================
// INSTRUCTION OPERAND
// ============================================================================

/// Typed semantic operand/reference used by extensible instruction forms.
///
/// Standard instructions normally use their strongly typed dedicated fields.
/// `InstructionOperand` exists for future/vendor/custom instruction dialects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InstructionOperand {
    /// Logical qubit.
    Qubit(QubitId),

    /// Classical bit.
    ClassicalBit(ClassicalBitId),

    /// Existing operation.
    Operation(OperationId),

    /// Pulse object.
    Pulse(PulseId),

    /// Waveform object.
    Waveform(WaveformId),

    /// Abstract control channel.
    Channel(ChannelId),

    /// Abstract control frame.
    Frame(FrameId),

    /// Schedule object.
    Schedule(ScheduleId),

    /// Abstract resource requirement.
    Resource(ResourceId),

    /// Capability requirement.
    Capability(CapabilityId),

    /// Extensible object.
    Extension(ExtensionId),
}

impl InstructionOperand {
    /// Returns the referenced logical qubit if this is a qubit operand.
    #[must_use]
    pub const fn qubit(self) -> Option<QubitId> {
        match self {
            Self::Qubit(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the referenced classical bit if this is a classical operand.
    #[must_use]
    pub const fn classical_bit(self) -> Option<ClassicalBitId> {
        match self {
            Self::ClassicalBit(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the referenced operation if this is an operation operand.
    #[must_use]
    pub const fn operation(self) -> Option<OperationId> {
        match self {
            Self::Operation(value) => Some(value),
            _ => None,
        }
    }

    /// Returns whether this is a quantum operand.
    #[must_use]
    pub const fn is_quantum(self) -> bool {
        matches!(self, Self::Qubit(_))
    }
}

// ============================================================================
// CONDITIONAL REFERENCE
// ============================================================================

/// Explicit classical condition attached to an existing operation.
///
/// The condition expression itself remains owned by the classical/control-flow
/// subsystem. This structure only records the instruction-level dependency.
///
/// This avoids duplicating the complete classical expression AST here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConditionalReference {
    condition_id: u64,
    target: OperationId,
}

impl ConditionalReference {
    /// Creates a conditional reference.
    ///
    /// `condition_id` is an opaque identity supplied by the classical/control
    /// subsystem. It is not interpreted by this module.
    pub fn new(
        condition_id: u64,
        target: OperationId,
    ) -> InstructionResult<Self> {
        if condition_id == 0 {
            return Err(InstructionError::InvalidStructure {
                message: "conditional condition identity must be non-zero",
            });
        }

        Ok(Self {
            condition_id,
            target,
        })
    }

    /// Returns the opaque condition identity.
    #[must_use]
    pub const fn condition_id(&self) -> u64 {
        self.condition_id
    }

    /// Returns the target operation.
    #[must_use]
    pub const fn target(&self) -> OperationId {
        self.target
    }

    /// Validates the reference against the owning operation identity.
    pub fn validate_against(
        &self,
        owner: OperationId,
    ) -> InstructionResult<()> {
        if self.target == owner {
            return Err(
                InstructionError::ConditionalSelfReference {
                    operation: owner,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// RESET SEMANTICS
// ============================================================================

/// Reset instruction semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResetInstruction {
    qubits: Vec<QubitId>,
}

impl ResetInstruction {
    /// Creates a reset instruction.
    ///
    /// Multiple qubits are supported. There is deliberately no fixed maximum.
    pub fn new(
        qubits: Vec<QubitId>,
    ) -> InstructionResult<Self> {
        if qubits.is_empty() {
            return Err(InstructionError::EmptyReset);
        }

        validate_unique_qubits(&qubits)?;

        Ok(Self { qubits })
    }

    /// Returns the reset qubits in semantic operand order.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }
}

// ============================================================================
// BARRIER SEMANTICS
// ============================================================================

/// Barrier/synchronization instruction semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BarrierInstruction {
    qubits: Vec<QubitId>,
}

impl BarrierInstruction {
    /// Creates a barrier over one or more logical qubits.
    ///
    /// A barrier does not choose hardware scheduling. It expresses semantic
    /// synchronization intent.
    pub fn new(
        qubits: Vec<QubitId>,
    ) -> InstructionResult<Self> {
        if qubits.is_empty() {
            return Err(InstructionError::EmptyBarrier);
        }

        validate_unique_qubits(&qubits)?;

        Ok(Self { qubits })
    }

    /// Returns the barrier qubits in semantic order.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }
}

// ============================================================================
// DELAY SEMANTICS
// ============================================================================

/// Delay duration expressed in femtoseconds.
///
/// This local representation is intentionally integer-based and deterministic.
/// The canonical program-wide timing layer may later convert it into its own
/// timing representation.
///
/// The value is not a hardware clock period.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct InstructionDuration(u64);

impl InstructionDuration {
    /// Zero duration.
    pub const ZERO: Self = Self(0);

    /// Creates a duration from femtoseconds.
    #[must_use]
    pub const fn from_femtoseconds(value: u64) -> Self {
        Self(value)
    }

    /// Creates a duration from picoseconds.
    pub const fn from_picoseconds(
        value: u64,
    ) -> InstructionResult<Self> {
        value
            .checked_mul(1_000)
            .map(Self)
            .ok_or(InstructionError::InvalidStructure {
                message: "instruction duration overflow",
            })
    }

    /// Creates a duration from nanoseconds.
    pub const fn from_nanoseconds(
        value: u64,
    ) -> InstructionResult<Self> {
        value
            .checked_mul(1_000_000)
            .map(Self)
            .ok_or(InstructionError::InvalidStructure {
                message: "instruction duration overflow",
            })
    }

    /// Creates a duration from microseconds.
    pub const fn from_microseconds(
        value: u64,
    ) -> InstructionResult<Self> {
        value
            .checked_mul(1_000_000_000)
            .map(Self)
            .ok_or(InstructionError::InvalidStructure {
                message: "instruction duration overflow",
            })
    }

    /// Returns femtoseconds.
    #[must_use]
    pub const fn femtoseconds(self) -> u64 {
        self.0
    }

    /// Returns whether this duration is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for InstructionDuration {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{}fs", self.0)
    }
}

// ============================================================================
// DELAY INSTRUCTION
// ============================================================================

/// Hardware-independent delay instruction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DelayInstruction {
    duration: InstructionDuration,
    qubits: Vec<QubitId>,
}

impl DelayInstruction {
    /// Creates a delay over the specified logical qubits.
    ///
    /// An empty qubit set is allowed because a semantic delay can represent a
    /// program-level timing region rather than a qubit-local delay.
    pub fn new(
        duration: InstructionDuration,
        qubits: Vec<QubitId>,
    ) -> InstructionResult<Self> {
        validate_unique_qubits(&qubits)?;

        Ok(Self {
            duration,
            qubits,
        })
    }

    /// Returns the semantic duration.
    #[must_use]
    pub const fn duration(&self) -> InstructionDuration {
        self.duration
    }

    /// Returns the affected logical qubits.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }
}

// ============================================================================
// PULSE INSTRUCTION
// ============================================================================

/// Pulse-level instruction.
///
/// The pulse object itself is owned by `pulse.rs`. This instruction only
/// references it by stable identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PulseInstruction {
    pulse: PulseId,
}

impl PulseInstruction {
    /// Creates a pulse reference.
    #[must_use]
    pub const fn new(pulse: PulseId) -> Self {
        Self { pulse }
    }

    /// Returns the pulse identity.
    #[must_use]
    pub const fn pulse(&self) -> PulseId {
        self.pulse
    }
}

// ============================================================================
// WAVEFORM INSTRUCTION
// ============================================================================

/// Waveform reference instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaveformInstruction {
    waveform: WaveformId,
}

impl WaveformInstruction {
    /// Creates a waveform reference.
    #[must_use]
    pub const fn new(waveform: WaveformId) -> Self {
        Self { waveform }
    }

    /// Returns the waveform identity.
    #[must_use]
    pub const fn waveform(&self) -> WaveformId {
        self.waveform
    }
}

// ============================================================================
// CHANNEL INSTRUCTION
// ============================================================================

/// Abstract control/acquisition channel reference.
///
/// This is deliberately not a physical channel number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChannelInstruction {
    channel: ChannelId,
}

impl ChannelInstruction {
    /// Creates an abstract channel reference.
    #[must_use]
    pub const fn new(channel: ChannelId) -> Self {
        Self { channel }
    }

    /// Returns the channel identity.
    #[must_use]
    pub const fn channel(&self) -> ChannelId {
        self.channel
    }
}

// ============================================================================
// FRAME INSTRUCTION
// ============================================================================

/// Abstract control-frame reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameInstruction {
    frame: FrameId,
}

impl FrameInstruction {
    /// Creates an abstract frame reference.
    #[must_use]
    pub const fn new(frame: FrameId) -> Self {
        Self { frame }
    }

    /// Returns the frame identity.
    #[must_use]
    pub const fn frame(&self) -> FrameId {
        self.frame
    }
}

// ============================================================================
// CLASSICAL FEEDBACK
// ============================================================================

/// Classical feedback instruction.
///
/// The actual expression is owned by the classical/control-flow layer.
/// `condition_id` is intentionally opaque here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassicalFeedbackInstruction {
    condition_id: u64,
    target: OperationId,
}

impl ClassicalFeedbackInstruction {
    /// Creates a classical-feedback reference.
    pub fn new(
        condition_id: u64,
        target: OperationId,
    ) -> InstructionResult<Self> {
        if condition_id == 0 {
            return Err(InstructionError::InvalidStructure {
                message: "classical feedback condition identity must be non-zero",
            });
        }

        Ok(Self {
            condition_id,
            target,
        })
    }

    /// Returns the opaque classical condition identity.
    #[must_use]
    pub const fn condition_id(&self) -> u64 {
        self.condition_id
    }

    /// Returns the operation affected by feedback.
    #[must_use]
    pub const fn target(&self) -> OperationId {
        self.target
    }

    /// Validates against the containing operation.
    pub fn validate_against(
        &self,
        owner: OperationId,
    ) -> InstructionResult<()> {
        if self.target == owner {
            return Err(
                InstructionError::ConditionalSelfReference {
                    operation: owner,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// LOGICAL INSTRUCTION
// ============================================================================

/// Logical/fault-tolerant instruction reference.
///
/// The actual logical operation definition belongs to the logical/QEC dialect.
/// This type only identifies it without coupling canonical instruction.rs to a
/// particular code, lattice, decoder or fault-tolerance scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogicalInstruction {
    extension: ExtensionId,
}

impl LogicalInstruction {
    /// Creates an extensible logical-operation reference.
    #[must_use]
    pub const fn new(extension: ExtensionId) -> Self {
        Self { extension }
    }

    /// Returns the extension defining the logical operation.
    #[must_use]
    pub const fn extension(&self) -> ExtensionId {
        self.extension
    }
}

// ============================================================================
// ANALOG INSTRUCTION
// ============================================================================

/// Analog/Hamiltonian instruction reference.
///
/// The Hamiltonian/analog model itself belongs to the analog/model dialect.
/// This instruction stores the extension object defining the semantic analog
/// evolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnalogInstruction {
    extension: ExtensionId,
}

impl AnalogInstruction {
    /// Creates an analog-operation reference.
    #[must_use]
    pub const fn new(extension: ExtensionId) -> Self {
        Self { extension }
    }

    /// Returns the extension defining the analog operation.
    #[must_use]
    pub const fn extension(&self) -> ExtensionId {
        self.extension
    }
}

// ============================================================================
// ANNEALING INSTRUCTION
// ============================================================================

/// Annealing/Ising/QUBO instruction reference.
///
/// Annealing semantics remain outside the standard circuit/gate vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnnealingInstruction {
    extension: ExtensionId,
}

impl AnnealingInstruction {
    /// Creates an annealing-operation reference.
    #[must_use]
    pub const fn new(extension: ExtensionId) -> Self {
        Self { extension }
    }

    /// Returns the extension defining the annealing operation.
    #[must_use]
    pub const fn extension(&self) -> ExtensionId {
        self.extension
    }
}

// ============================================================================
// DISTRIBUTED INSTRUCTION
// ============================================================================

/// Distributed quantum instruction reference.
///
/// Distributed semantics such as entanglement links, remote operations and
/// teleportation are defined by the distributed dialect/model layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DistributedInstruction {
    extension: ExtensionId,
}

impl DistributedInstruction {
    /// Creates a distributed-operation reference.
    #[must_use]
    pub const fn new(extension: ExtensionId) -> Self {
        Self { extension }
    }

    /// Returns the extension defining the distributed operation.
    #[must_use]
    pub const fn extension(&self) -> ExtensionId {
        self.extension
    }
}

// ============================================================================
// RESOURCE REQUIREMENT
// ============================================================================

/// Explicit abstract resource requirement attached to an instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionResource {
    resource: ResourceId,
}

impl InstructionResource {
    /// Creates a resource requirement.
    #[must_use]
    pub const fn new(resource: ResourceId) -> Self {
        Self { resource }
    }

    /// Returns the resource identity.
    #[must_use]
    pub const fn resource(&self) -> ResourceId {
        self.resource
    }
}

// ============================================================================
// CAPABILITY REQUIREMENT
// ============================================================================

/// Explicit target capability requirement attached to an instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionCapability {
    capability: CapabilityId,
}

impl InstructionCapability {
    /// Creates a capability requirement.
    #[must_use]
    pub const fn new(capability: CapabilityId) -> Self {
        Self { capability }
    }

    /// Returns the capability identity.
    #[must_use]
    pub const fn capability(&self) -> CapabilityId {
        self.capability
    }
}

// ============================================================================
// SCHEDULE REFERENCE
// ============================================================================

/// Reference to a semantic schedule object.
///
/// This does not perform scheduling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduleInstruction {
    schedule: ScheduleId,
}

impl ScheduleInstruction {
    /// Creates a schedule reference.
    #[must_use]
    pub const fn new(schedule: ScheduleId) -> Self {
        Self { schedule }
    }

    /// Returns the schedule identity.
    #[must_use]
    pub const fn schedule(&self) -> ScheduleId {
        self.schedule
    }
}

// ============================================================================
// EXTENSION INSTRUCTION
// ============================================================================

/// Explicit extensible/vendor/future instruction.
///
/// The actual schema is owned by the extension/dialect layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtensionInstruction {
    extension: ExtensionId,
}

impl ExtensionInstruction {
    /// Creates an extension reference.
    #[must_use]
    pub const fn new(extension: ExtensionId) -> Self {
        Self { extension }
    }

    /// Returns the extension identity.
    #[must_use]
    pub const fn extension(&self) -> ExtensionId {
        self.extension
    }
}

// ============================================================================
// INSTRUCTION BODY
// ============================================================================

/// Complete semantic body of an instruction.
///
/// This enum intentionally contains semantic references rather than hardware
/// implementation objects.
///
/// Standard gates remain delegated to `Gate`.
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionBody {
    /// Standard logical gate.
    Gate(Gate),

    /// Canonical measurement semantics.
    Measurement(Measurement),

    /// Logical reset.
    Reset(ResetInstruction),

    /// Barrier/synchronization.
    Barrier(BarrierInstruction),

    /// Semantic delay.
    Delay(DelayInstruction),

    /// Pulse reference.
    Pulse(PulseInstruction),

    /// Waveform reference.
    Waveform(WaveformInstruction),

    /// Abstract channel reference.
    Channel(ChannelInstruction),

    /// Abstract frame reference.
    Frame(FrameInstruction),

    /// Classical feedback.
    ClassicalFeedback(ClassicalFeedbackInstruction),

    /// Conditional reference.
    Conditional(ConditionalReference),

    /// Logical/fault-tolerant instruction.
    Logical(LogicalInstruction),

    /// Analog/Hamiltonian instruction.
    Analog(AnalogInstruction),

    /// Annealing/Ising/QUBO instruction.
    Annealing(AnnealingInstruction),

    /// Distributed quantum instruction.
    Distributed(DistributedInstruction),

    /// Resource requirement/reference.
    Resource(InstructionResource),

    /// Schedule reference.
    Schedule(ScheduleInstruction),

    /// Explicit future/vendor/custom extension.
    Extension(ExtensionInstruction),
}

impl InstructionBody {
    /// Returns the stable semantic kind.
    #[must_use]
    pub const fn kind(&self) -> InstructionKind {
        match self {
            Self::Gate(_) => InstructionKind::Gate,
            Self::Measurement(_) => InstructionKind::Measurement,
            Self::Reset(_) => InstructionKind::Reset,
            Self::Barrier(_) => InstructionKind::Barrier,
            Self::Delay(_) => InstructionKind::Delay,
            Self::Pulse(_) => InstructionKind::Pulse,
            Self::Waveform(_) => InstructionKind::Waveform,
            Self::Channel(_) => InstructionKind::Channel,
            Self::Frame(_) => InstructionKind::Frame,
            Self::ClassicalFeedback(_) => {
                InstructionKind::ClassicalFeedback
            }
            Self::Conditional(_) => InstructionKind::Conditional,
            Self::Logical(_) => InstructionKind::Logical,
            Self::Analog(_) => InstructionKind::Analog,
            Self::Annealing(_) => InstructionKind::Annealing,
            Self::Distributed(_) => InstructionKind::Distributed,
            Self::Resource(_) => InstructionKind::Resource,
            Self::Schedule(_) => InstructionKind::Schedule,
            Self::Extension(_) => InstructionKind::Extension,
        }
    }

    /// Returns the broad semantic class.
    #[must_use]
    pub const fn class(&self) -> InstructionClass {
        self.kind().class()
    }

    /// Returns the standard gate if this is a gate instruction.
    #[must_use]
    pub fn gate(&self) -> Option<&Gate> {
        match self {
            Self::Gate(gate) => Some(gate),
            _ => None,
        }
    }

    /// Returns measurement semantics if this is a measurement instruction.
    #[must_use]
    pub fn measurement(&self) -> Option<&Measurement> {
        match self {
            Self::Measurement(measurement) => Some(measurement),
            _ => None,
        }
    }

    /// Returns logical qubits directly owned by the instruction body.
    ///
    /// The returned vector is newly allocated only for the accessor call.
    /// Semantic storage remains owned by the underlying instruction.
    #[must_use]
    pub fn qubits(&self) -> Vec<QubitId> {
        match self {
            Self::Gate(gate) => gate.qubits().to_vec(),

            Self::Measurement(measurement) => {
                measurement.qubits().to_vec()
            }

            Self::Reset(reset) => reset.qubits().to_vec(),

            Self::Barrier(barrier) => barrier.qubits().to_vec(),

            Self::Delay(delay) => delay.qubits().to_vec(),

            _ => Vec::new(),
        }
    }

    /// Returns whether this body directly references a quantum resource.
    #[must_use]
    pub fn references_qubits(&self) -> bool {
        match self {
            Self::Gate(_)
            | Self::Measurement(_)
            | Self::Reset(_)
            | Self::Barrier(_)
            | Self::Delay(_) => true,

            _ => false,
        }
    }

    /// Returns whether the instruction is non-unitary.
    #[must_use]
    pub const fn is_non_unitary(&self) -> bool {
        matches!(
            self,
            Self::Measurement(_)
                | Self::Reset(_)
                | Self::ClassicalFeedback(_)
                | Self::Conditional(_)
        )
    }
}

// ============================================================================
// INSTRUCTION
// ============================================================================

/// Canonical universal instruction.
///
/// `Instruction` is the stable semantic object consumed by program/circuit IR
/// containers.
///
/// It does not own source locations, scheduling decisions, physical mapping or
/// backend implementation details.
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    id: OperationId,
    body: InstructionBody,
    resources: Vec<InstructionResource>,
    capabilities: Vec<InstructionCapability>,
    extensions: Vec<ExtensionId>,
}

impl Instruction {
    /// Creates a new instruction after checking local invariants.
    pub fn new(
        id: OperationId,
        body: InstructionBody,
    ) -> InstructionResult<Self> {
        let instruction = Self {
            id,
            body,
            resources: Vec::new(),
            capabilities: Vec::new(),
            extensions: Vec::new(),
        };

        instruction.validate()?;

        Ok(instruction)
    }

    /// Creates an instruction with explicit resource requirements.
    pub fn with_resources(
        id: OperationId,
        body: InstructionBody,
        resources: Vec<InstructionResource>,
    ) -> InstructionResult<Self> {
        let mut instruction = Self::new(id, body)?;
        instruction.set_resources(resources)?;
        Ok(instruction)
    }

    /// Creates an instruction with explicit capability requirements.
    pub fn with_capabilities(
        id: OperationId,
        body: InstructionBody,
        capabilities: Vec<InstructionCapability>,
    ) -> InstructionResult<Self> {
        let mut instruction = Self::new(id, body)?;
        instruction.set_capabilities(capabilities)?;
        Ok(instruction)
    }

    /// Creates an instruction with explicit extension references.
    pub fn with_extensions(
        id: OperationId,
        body: InstructionBody,
        extensions: Vec<ExtensionId>,
    ) -> InstructionResult<Self> {
        let mut instruction = Self::new(id, body)?;
        instruction.set_extensions(extensions)?;
        Ok(instruction)
    }

    /// Returns the stable operation identity.
    #[must_use]
    pub const fn id(&self) -> OperationId {
        self.id
    }

    /// Returns the instruction body.
    #[must_use]
    pub fn body(&self) -> &InstructionBody {
        &self.body
    }

    /// Returns the instruction kind.
    #[must_use]
    pub const fn kind(&self) -> InstructionKind {
        match &self.body {
            InstructionBody::Gate(_) => InstructionKind::Gate,
            InstructionBody::Measurement(_) => InstructionKind::Measurement,
            InstructionBody::Reset(_) => InstructionKind::Reset,
            InstructionBody::Barrier(_) => InstructionKind::Barrier,
            InstructionBody::Delay(_) => InstructionKind::Delay,
            InstructionBody::Pulse(_) => InstructionKind::Pulse,
            InstructionBody::Waveform(_) => InstructionKind::Waveform,
            InstructionBody::Channel(_) => InstructionKind::Channel,
            InstructionBody::Frame(_) => InstructionKind::Frame,
            InstructionBody::ClassicalFeedback(_) => {
                InstructionKind::ClassicalFeedback
            }
            InstructionBody::Conditional(_) => InstructionKind::Conditional,
            InstructionBody::Logical(_) => InstructionKind::Logical,
            InstructionBody::Analog(_) => InstructionKind::Analog,
            InstructionBody::Annealing(_) => InstructionKind::Annealing,
            InstructionBody::Distributed(_) => InstructionKind::Distributed,
            InstructionBody::Resource(_) => InstructionKind::Resource,
            InstructionBody::Schedule(_) => InstructionKind::Schedule,
            InstructionBody::Extension(_) => InstructionKind::Extension,
        }
    }

    /// Returns the broad semantic class.
    #[must_use]
    pub const fn class(&self) -> InstructionClass {
        self.kind().class()
    }

    /// Returns whether this instruction is a standard logical gate.
    #[must_use]
    pub const fn is_gate(&self) -> bool {
        matches!(self.body, InstructionBody::Gate(_))
    }

    /// Returns whether this instruction is measurement.
    #[must_use]
    pub const fn is_measurement(&self) -> bool {
        matches!(self.body, InstructionBody::Measurement(_))
    }

    /// Returns whether this instruction is reset.
    #[must_use]
    pub const fn is_reset(&self) -> bool {
        matches!(self.body, InstructionBody::Reset(_))
    }

    /// Returns whether this instruction is a barrier.
    #[must_use]
    pub const fn is_barrier(&self) -> bool {
        matches!(self.body, InstructionBody::Barrier(_))
    }

    /// Returns whether this instruction is pulse-level.
    #[must_use]
    pub const fn is_pulse(&self) -> bool {
        matches!(
            self.body,
            InstructionBody::Pulse(_)
                | InstructionBody::Waveform(_)
                | InstructionBody::Channel(_)
                | InstructionBody::Frame(_)
        )
    }

    /// Returns whether this instruction is an extension.
    #[must_use]
    pub const fn is_extension(&self) -> bool {
        matches!(self.body, InstructionBody::Extension(_))
    }

    /// Returns logical qubit operands.
    ///
    /// This accessor allocates a new vector. For analysis code that needs to
    /// avoid allocation, prefer matching `body()` directly.
    #[must_use]
    pub fn qubits(&self) -> Vec<QubitId> {
        self.body.qubits()
    }

    /// Returns all resource requirements.
    #[must_use]
    pub fn resources(&self) -> &[InstructionResource] {
        &self.resources
    }

    /// Returns all capability requirements.
    #[must_use]
    pub fn capabilities(&self) -> &[InstructionCapability] {
        &self.capabilities
    }

    /// Returns all extension references.
    #[must_use]
    pub fn extensions(&self) -> &[ExtensionId] {
        &self.extensions
    }

    /// Replaces resource requirements after validating uniqueness.
    pub fn set_resources(
        &mut self,
        resources: Vec<InstructionResource>,
    ) -> InstructionResult<()> {
        let mut seen = BTreeSet::new();

        for resource in &resources {
            if !seen.insert(resource.resource()) {
                return Err(
                    InstructionError::DuplicateResource {
                        resource: resource.resource(),
                    },
                );
            }
        }

        self.resources = resources;
        Ok(())
    }

    /// Replaces capability requirements after validating uniqueness.
    pub fn set_capabilities(
        &mut self,
        capabilities: Vec<InstructionCapability>,
    ) -> InstructionResult<()> {
        let mut seen = BTreeSet::new();

        for capability in &capabilities {
            if !seen.insert(capability.capability()) {
                return Err(
                    InstructionError::DuplicateCapability {
                        capability: capability.capability(),
                    },
                );
            }
        }

        self.capabilities = capabilities;
        Ok(())
    }

    /// Replaces extension references after validating uniqueness.
    pub fn set_extensions(
        &mut self,
        extensions: Vec<ExtensionId>,
    ) -> InstructionResult<()> {
        let mut seen = BTreeSet::new();

        for extension in &extensions {
            if !seen.insert(*extension) {
                return Err(
                    InstructionError::DuplicateExtension {
                        extension: *extension,
                    },
                );
            }
        }

        self.extensions = extensions;
        Ok(())
    }

    /// Returns whether the instruction references the supplied logical qubit.
    #[must_use]
    pub fn uses_qubit(
        &self,
        qubit: QubitId,
    ) -> bool {
        self.qubits().iter().any(|candidate| *candidate == qubit)
    }

    /// Returns whether the instruction is a measurement or reset operation.
    #[must_use]
    pub const fn is_state_boundary(&self) -> bool {
        matches!(
            self.body,
            InstructionBody::Measurement(_)
                | InstructionBody::Reset(_)
        )
    }

    /// Validates local structural invariants.
    ///
    /// Namespace existence and target capability checks belong to
    /// `validation.rs` and `quantum::hardware`.
    pub fn validate(&self) -> InstructionResult<()> {
        match &self.body {
            InstructionBody::Gate(gate) => {
                let qubits = gate.qubits();

                validate_unique_qubits(qubits)?;

                if qubits.is_empty() {
                    return Err(InstructionError::InvalidGateOperands);
                }
            }

            InstructionBody::Measurement(measurement) => {
                let qubits = measurement.qubits();

                if qubits.is_empty() {
                    return Err(InstructionError::InvalidStructure {
                        message: "measurement requires at least one logical qubit",
                    });
                }

                validate_unique_qubits(qubits)?;

                validate_measurement_destinations(measurement)?;
            }

            InstructionBody::Reset(reset) => {
                validate_unique_qubits(reset.qubits())?;
            }

            InstructionBody::Barrier(barrier) => {
                validate_unique_qubits(barrier.qubits())?;

                if barrier.qubits().is_empty() {
                    return Err(InstructionError::EmptyBarrier);
                }
            }

            InstructionBody::Delay(delay) => {
                validate_unique_qubits(delay.qubits())?;
            }

            InstructionBody::Conditional(condition) => {
                condition.validate_against(self.id)?;
            }

            InstructionBody::ClassicalFeedback(feedback) => {
                feedback.validate_against(self.id)?;
            }

            InstructionBody::Pulse(_)
            | InstructionBody::Waveform(_)
            | InstructionBody::Channel(_)
            | InstructionBody::Frame(_)
            | InstructionBody::Logical(_)
            | InstructionBody::Analog(_)
            | InstructionBody::Annealing(_)
            | InstructionBody::Distributed(_)
            | InstructionBody::Resource(_)
            | InstructionBody::Schedule(_)
            | InstructionBody::Extension(_) => {}
        }

        validate_resource_requirements(&self.resources)?;
        validate_capability_requirements(&self.capabilities)?;
        validate_extension_references(&self.extensions)?;

        Ok(())
    }
}

// ============================================================================
// MEASUREMENT DESTINATION VALIDATION
// ============================================================================

/// Validates measurement destination uniqueness without imposing a global
/// classical-register size.
fn validate_measurement_destinations(
    measurement: &Measurement,
) -> InstructionResult<()> {
    let mut seen = BTreeSet::new();

    for bit in measurement.classical_bits() {
        if !seen.insert(*bit) {
            return Err(
                InstructionError::DuplicateClassicalBit {
                    bit: *bit,
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// QUANTUM OPERAND VALIDATION
// ============================================================================

/// Validates logical-qubit uniqueness.
///
/// No maximum number of qubits is imposed.
///
/// The complexity is proportional to the number of operands actually supplied.
fn validate_unique_qubits(
    qubits: &[QubitId],
) -> InstructionResult<()> {
    let mut seen = BTreeSet::new();

    for qubit in qubits {
        if !seen.insert(*qubit) {
            return Err(
                InstructionError::DuplicateQubit {
                    qubit: *qubit,
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// RESOURCE VALIDATION
// ============================================================================

fn validate_resource_requirements(
    resources: &[InstructionResource],
) -> InstructionResult<()> {
    let mut seen = BTreeSet::new();

    for resource in resources {
        if !seen.insert(resource.resource()) {
            return Err(
                InstructionError::DuplicateResource {
                    resource: resource.resource(),
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// CAPABILITY VALIDATION
// ============================================================================

fn validate_capability_requirements(
    capabilities: &[InstructionCapability],
) -> InstructionResult<()> {
    let mut seen = BTreeSet::new();

    for capability in capabilities {
        if !seen.insert(capability.capability()) {
            return Err(
                InstructionError::DuplicateCapability {
                    capability: capability.capability(),
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// EXTENSION VALIDATION
// ============================================================================

fn validate_extension_references(
    extensions: &[ExtensionId],
) -> InstructionResult<()> {
    let mut seen = BTreeSet::new();

    for extension in extensions {
        if !seen.insert(*extension) {
            return Err(
                InstructionError::DuplicateExtension {
                    extension: *extension,
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// STANDARD CONSTRUCTORS
// ============================================================================

impl Instruction {
    /// Creates a standard gate instruction.
    pub fn gate(
        id: OperationId,
        gate: Gate,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Gate(gate),
        )
    }

    /// Creates a measurement instruction.
    pub fn measurement(
        id: OperationId,
        measurement: Measurement,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Measurement(measurement),
        )
    }

    /// Creates a reset instruction.
    pub fn reset(
        id: OperationId,
        qubits: Vec<QubitId>,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Reset(
                ResetInstruction::new(qubits)?,
            ),
        )
    }

    /// Creates a barrier instruction.
    pub fn barrier(
        id: OperationId,
        qubits: Vec<QubitId>,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Barrier(
                BarrierInstruction::new(qubits)?,
            ),
        )
    }

    /// Creates a delay instruction.
    pub fn delay(
        id: OperationId,
        duration: InstructionDuration,
        qubits: Vec<QubitId>,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Delay(
                DelayInstruction::new(
                    duration,
                    qubits,
                )?,
            ),
        )
    }

    /// Creates a pulse instruction.
    pub fn pulse(
        id: OperationId,
        pulse: PulseId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Pulse(
                PulseInstruction::new(pulse),
            ),
        )
    }

    /// Creates a waveform instruction.
    pub fn waveform(
        id: OperationId,
        waveform: WaveformId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Waveform(
                WaveformInstruction::new(waveform),
            ),
        )
    }

    /// Creates a channel instruction.
    pub fn channel(
        id: OperationId,
        channel: ChannelId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Channel(
                ChannelInstruction::new(channel),
            ),
        )
    }

    /// Creates a frame instruction.
    pub fn frame(
        id: OperationId,
        frame: FrameId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Frame(
                FrameInstruction::new(frame),
            ),
        )
    }

    /// Creates a classical-feedback instruction.
    pub fn classical_feedback(
        id: OperationId,
        condition_id: u64,
        target: OperationId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::ClassicalFeedback(
                ClassicalFeedbackInstruction::new(
                    condition_id,
                    target,
                )?,
            ),
        )
    }

    /// Creates a conditional instruction.
    pub fn conditional(
        id: OperationId,
        condition_id: u64,
        target: OperationId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Conditional(
                ConditionalReference::new(
                    condition_id,
                    target,
                )?,
            ),
        )
    }

    /// Creates a logical instruction.
    pub fn logical(
        id: OperationId,
        extension: ExtensionId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Logical(
                LogicalInstruction::new(extension),
            ),
        )
    }

    /// Creates an analog instruction.
    pub fn analog(
        id: OperationId,
        extension: ExtensionId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Analog(
                AnalogInstruction::new(extension),
            ),
        )
    }

    /// Creates an annealing instruction.
    pub fn annealing(
        id: OperationId,
        extension: ExtensionId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Annealing(
                AnnealingInstruction::new(extension),
            ),
        )
    }

    /// Creates a distributed instruction.
    pub fn distributed(
        id: OperationId,
        extension: ExtensionId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Distributed(
                DistributedInstruction::new(extension),
            ),
        )
    }

    /// Creates an extension instruction.
    pub fn extension(
        id: OperationId,
        extension: ExtensionId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Extension(
                ExtensionInstruction::new(extension),
            ),
        )
    }

    /// Creates a resource-reference instruction.
    pub fn resource(
        id: OperationId,
        resource: ResourceId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Resource(
                InstructionResource::new(resource),
            ),
        )
    }

    /// Creates a schedule-reference instruction.
    pub fn schedule(
        id: OperationId,
        schedule: ScheduleId,
    ) -> InstructionResult<Self> {
        Self::new(
            id,
            InstructionBody::Schedule(
                ScheduleInstruction::new(schedule),
            ),
        )
    }
}

// ============================================================================
// TRAITS
// ============================================================================

impl fmt::Display for Instruction {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{} {}",
            self.id,
            self.kind().class()
        )
    }
}

impl fmt::Display for InstructionKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let name = match self {
            Self::Gate => "gate",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::Barrier => "barrier",
            Self::Delay => "delay",
            Self::Pulse => "pulse",
            Self::Waveform => "waveform",
            Self::Channel => "channel",
            Self::Frame => "frame",
            Self::ClassicalFeedback => "classical_feedback",
            Self::Conditional => "conditional",
            Self::Logical => "logical",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Distributed => "distributed",
            Self::Resource => "resource",
            Self::Schedule => "schedule",
            Self::Extension => "extension",
        };

        formatter.write_str(name)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::identity::{
        OperationId,
        PulseId,
    };

    #[test]
    fn duration_is_exact_and_deterministic() {
        let duration =
            InstructionDuration::from_nanoseconds(20)
                .expect("20ns must be representable");

        assert_eq!(
            duration.femtoseconds(),
            20_000_000
        );
    }

    #[test]
    fn duration_detects_overflow() {
        let result =
            InstructionDuration::from_nanoseconds(u64::MAX);

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let q = QubitId::new(7);

        let result =
            ResetInstruction::new(vec![q, q]);

        assert!(matches!(
            result,
            Err(
                InstructionError::DuplicateQubit {
                    qubit
                }
            ) if qubit == q
        ));
    }

    #[test]
    fn empty_reset_is_rejected() {
        let result =
            ResetInstruction::new(Vec::new());

        assert!(matches!(
            result,
            Err(InstructionError::EmptyReset)
        ));
    }

    #[test]
    fn empty_barrier_is_rejected() {
        let result =
            BarrierInstruction::new(Vec::new());

        assert!(matches!(
            result,
            Err(InstructionError::EmptyBarrier)
        ));
    }

    #[test]
    fn conditional_self_reference_is_rejected() {
        let operation = OperationId::new(42);

        let condition =
            ConditionalReference::new(
                1,
                operation,
            )
            .expect("condition construction must succeed");

        let result =
            condition.validate_against(operation);

        assert!(matches!(
            result,
            Err(
                InstructionError::ConditionalSelfReference {
                    operation: value
                }
            ) if value == operation
        ));
    }

    #[test]
    fn pulse_instruction_is_hardware_independent() {
        let instruction =
            PulseInstruction::new(
                PulseId::new(10),
            );

        assert_eq!(
            instruction.pulse(),
            PulseId::new(10)
        );
    }

    #[test]
    fn resource_duplicates_are_rejected() {
        let resource =
            InstructionResource::new(
                ResourceId::new(5),
            );

        let result =
            Instruction::with_resources(
                OperationId::new(1),
                InstructionBody::Resource(resource),
                vec![resource, resource],
            );

        assert!(matches!(
            result,
            Err(
                InstructionError::DuplicateResource {
                    resource: value
                }
            ) if value == ResourceId::new(5)
        ));
    }

    #[test]
    fn capability_duplicates_are_rejected() {
        let capability =
            InstructionCapability::new(
                CapabilityId::new(5),
            );

        let result =
            Instruction::with_capabilities(
                OperationId::new(1),
                InstructionBody::Extension(
                    ExtensionInstruction::new(
                        ExtensionId::new(1),
                    ),
                ),
                vec![capability, capability],
            );

        assert!(matches!(
            result,
            Err(
                InstructionError::DuplicateCapability {
                    capability: value
                }
            ) if value == CapabilityId::new(5)
        ));
    }

    #[test]
    fn extension_duplicates_are_rejected() {
        let extension = ExtensionId::new(9);

        let result =
            Instruction::with_extensions(
                OperationId::new(1),
                InstructionBody::Extension(
                    ExtensionInstruction::new(
                        extension,
                    ),
                ),
                vec![extension, extension],
            );

        assert!(matches!(
            result,
            Err(
                InstructionError::DuplicateExtension {
                    extension: value
                }
            ) if value == extension
        ));
    }

    #[test]
    fn instruction_classification_is_stable() {
        let instruction =
            Instruction::pulse(
                OperationId::new(1),
                PulseId::new(2),
            )
            .expect("pulse instruction must construct");

        assert_eq!(
            instruction.kind(),
            InstructionKind::Pulse
        );

        assert_eq!(
            instruction.class(),
            InstructionClass::Pulse
        );

        assert!(instruction.is_pulse());
    }

    #[test]
    fn instruction_identity_is_independent_of_position() {
        let first =
            OperationId::new(100);

        let second =
            OperationId::new(200);

        assert_ne!(first, second);

        let a =
            Instruction::pulse(
                first,
                PulseId::new(1),
            )
            .expect("instruction must construct");

        let b =
            Instruction::pulse(
                second,
                PulseId::new(1),
            )
            .expect("instruction must construct");

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn no_fixed_instruction_operand_limit_exists() {
        let qubits: Vec<QubitId> =
            (0usize..10_000usize)
                .map(QubitId::new)
                .collect();

        let barrier =
            BarrierInstruction::new(qubits)
                .expect("10,000 logical operands must be representable");

        assert_eq!(
            barrier.qubits().len(),
            10_000
        );
    }

    #[test]
    fn delay_can_be_program_level() {
        let delay =
            DelayInstruction::new(
                InstructionDuration::from_nanoseconds(20)
                    .expect("duration must construct"),
                Vec::new(),
            )
            .expect("program-level delay must be valid");

        assert!(delay.qubits().is_empty());
        assert_eq!(
            delay.duration().femtoseconds(),
            20_000_000
        );
    }

    #[test]
    fn instruction_body_reports_kind() {
        let body =
            InstructionBody::Pulse(
                PulseInstruction::new(
                    PulseId::new(1),
                ),
            );

        assert_eq!(
            body.kind(),
            InstructionKind::Pulse
        );

        assert_eq!(
            body.class(),
            InstructionClass::Pulse
        );
    }
}