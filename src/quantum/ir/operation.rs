//! Zamani Quantum IR — Universal Operation Model
//!
//! Canonical, hardware-independent representation of quantum, classical,
//! control, timing, pulse-reference, logical, analog, annealing, and
//! extensible operations.
//!
//! # Architectural role
//!
//! `operation.rs` defines WHAT an operation is in the canonical Quantum IR.
//!
//! It does NOT decide:
//!
//! - where an operation executes;
//! - which physical qubit receives it;
//! - which hardware channel implements it;
//! - when it executes;
//! - how it is optimized;
//! - how it is routed;
//! - how it is scheduled;
//! - how pulses are synthesized;
//! - how calibration is applied;
//! - how a QPU executes it;
//! - how a simulator represents quantum state;
//! - how error correction decodes syndromes;
//! - how frontend source syntax is parsed.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! The intended architecture is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ├── operation.rs  ← WHAT operation means
//!      │
//!      ├── optimization ← HOW to improve it
//!      ├── routing      ← WHERE logical resources go
//!      ├── scheduling   ← WHEN things execute
//!      ├── hardware     ← WHAT physical machine exists
//!      └── backend      ← HOW the target executes it
//! ```
//!
//! # Universal quantum-program principle
//!
//! A Zamani quantum program is written once and is not intrinsically tied to
//! a particular machine size.
//!
//! The same semantic operation model must therefore work for:
//!
//! - one qubit;
//! - small QPUs;
//! - large QPUs;
//! - distributed quantum systems;
//! - logical/fault-tolerant machines;
//! - simulator targets;
//! - future quantum architectures.
//!
//! There is deliberately NO architectural operation-count, qubit-count,
//! operand-count, or machine-size ceiling in this module.
//!
//! Concrete resource/security limits belong to `QuantumIrLimits` and other
//! explicit compilation policies.
//!
//! A value such as `63`, `4096`, or `1_000_000` must never silently become a
//! semantic machine-size boundary.
//!
//! # Operation identity
//!
//! Every operation has a stable [`OperationId`].
//!
//! Operation identity is independent of position.
//!
//! Inserting an operation before another operation must not inherently change
//! the existing operation's identity.
//!
//! The identity is not a cryptographic content hash. Canonical content hashing
//! belongs to the hashing/provenance layer.
//!
//! # Logical qubit identity
//!
//! Quantum operands use the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Physical qubit identities are deliberately NOT used as ordinary operation
//! operands at the semantic level.
//!
//! Logical-to-physical placement belongs to routing/mapping.
//!
//! When a compiled IR needs to record a physical mapping, the existing
//! `PhysicalQubitId` vocabulary from `qubit.rs` and `mapping.rs` can be used
//! without changing this operation model.
//!
//! # Specialized operation references
//!
//! The universal operation enum intentionally uses strongly typed identities
//! for future specialized resources:
//!
//! ```text
//! PulseId
//! WaveformId
//! ChannelId
//! FrameId
//! ScheduleId
//! ResourceId
//! CapabilityId
//! ExtensionId
//! ```
//!
//! This is important for dependency stability.
//!
//! `operation.rs` does not need to import future `pulse.rs`, `waveform.rs`,
//! `channel.rs`, `frame.rs`, or `schedule.rs` merely to know that an operation
//! refers to one of those semantic objects.
//!
//! The specialized modules own the full definitions.
//!
//! Therefore this file can be finalized before those modules are implemented
//! without requiring this file to be rewritten later.
//!
//! # Pulse-level control
//!
//! Pulse-level programs such as:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! are represented at the operation layer as a first-class `Pulse` operation
//! referencing a canonical `PulseId`.
//!
//! The actual pulse semantics belong to `pulse.rs`:
//!
//! ```text
//! Operation
//!     │
//!     └── Pulse { pulse: PulseId }
//!                     │
//!                     ▼
//!                 pulse.rs
//!                     │
//!                     ├── amplitude
//!                     ├── duration
//!                     ├── waveform
//!                     ├── phase
//!                     ├── frequency
//!                     └── abstract target
//! ```
//!
//! Hardware-specific DACs, generators, physical channels and calibration
//! remain outside the IR.
//!
//! # Dynamic quantum programs
//!
//! The operation model supports dynamic computation through explicit
//! measurement and classical-condition references.
//!
//! Conceptually:
//!
//! ```text
//! measure(q0) -> c0
//!
//! if c0 == 1 {
//!     x(q1)
//! }
//! ```
//!
//! becomes a set of operations with explicit identity and dependency
//! relationships rather than hidden side effects.
//!
//! This is consistent with modern quantum IR architectures that distinguish
//! quantum instruction semantics from classical control and target capability.
//!
//! # No recursive operation representation
//!
//! Conditional operations do NOT contain another `Operation` recursively.
//!
//! Instead:
//!
//! ```text
//! Conditional {
//!     condition,
//!     target: OperationId,
//! }
//! ```
//!
//! references an already-defined operation.
//!
//! This avoids recursive heap structures, simplifies serialization and hashing,
//! prevents accidental infinite object construction, and makes large IR graphs
//! easier to process.
//!
//! # Validation boundary
//!
//! Constructors enforce local invariants that can be known without a complete
//! program:
//!
//! - operation IDs are explicit;
//! - gate operations contain a valid gate object;
//! - reset has exactly one logical qubit;
//! - barriers cannot be empty;
//! - explicit qubit collections contain unique qubits;
//! - pulse/waveform/channel/frame/resource references are strongly typed;
//! - conditional operations contain a valid target identity;
//! - extension references remain explicit.
//!
//! Program-wide validation belongs to `validation.rs`.
//!
//! Program-wide namespace validation can determine whether an operation's
//! `QubitId`, `ClassicalBitId`, `OperationId`, or resource identity actually
//! exists.
//!
//! # Determinism
//!
//! Collections owned by this module use deterministic ordering where semantic
//! ordering is required.
//!
//! `BTreeSet` is used for uniqueness checks and deterministic construction.
//!
//! The operation itself preserves explicit operand order because operand order
//! can be semantically significant.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contracts
//!
//! `identity.rs`
//!     Supplies `OperationId` and specialized resource identities.
//!
//! `qubit.rs`
//!     Supplies canonical `QubitId`.
//!
//! `classical.rs`
//!     Supplies `ClassicalBitId`.
//!
//! `gate.rs`
//!     Supplies canonical gate semantics.
//!
//! `measurement.rs`
//!     Supplies canonical measurement semantics.
//!
//! `parameter.rs`
//!     Supplies symbolic/numerical parameter semantics indirectly through
//!     `Gate` and future specialized operations.
//!
//! `validation.rs`
//!     Performs complete namespace, resource, semantic and structural
//!     validation.
//!
//! `circuit.rs`
//!     Stores operation sequences and may use this model as its operation
//!     element.
//!
//! `program.rs`
//!     Owns larger program structure and regions.
//!
//! `pulse.rs`
//!     Defines the object referenced by `OperationBody::Pulse`.
//!
//! `waveform.rs`
//!     Defines waveform objects referenced by `OperationBody::Waveform`.
//!
//! `channel.rs`
//!     Defines abstract channel objects referenced by channel operations.
//!
//! `frame.rs`
//!     Defines frame objects referenced by frame operations.
//!
//! `schedule.rs`
//!     Defines scheduling information without changing operation semantics.
//!
//! `mapping.rs`
//!     Resolves logical qubits to physical qubits.
//!
//! `resource.rs`
//!     Defines resource requirements.
//!
//! `capability.rs`
//!     Defines target capability requirements.
//!
//! `extension.rs`
//!     Defines extensible operation semantics.
//!
//! `serialization.rs`
//!     Serializes the stable operation representation.
//!
//! `hash.rs`
//!     Hashes canonical operation/program structure.
//!
//! `provenance.rs`
//!     Records transformation history and operation lineage.
//!
//! `analysis.rs`
//!     Counts and analyzes operations.
//!
//! `optimization/`
//!     Transforms operations but does not redefine their canonical meaning.
//!
//! `routing/`
//!     Resolves logical-to-physical placement.
//!
//! `scheduling/`
//!     Determines execution timing.
//!
//! `hardware/`
//!     Determines whether and how a target can implement an operation.
//!
//! # Important repository naming rule
//!
//! The canonical module is:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! NOT:
//!
//! ```text
//! quantum::ir::qubits
//! ```
//!
//! All new code in this file therefore uses:
//!
//! ```rust
//! use super::qubit::QubitId;
//! ```
//!
//! This deliberately avoids the existing repository naming inconsistency in
//! older IR consumers.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

use super::classical::ClassicalBitId;
use super::gate::Gate;
use super::identity::{
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
use super::measurement::Measurement;
use super::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type for operation construction and local validation.
pub type OperationResult<T> = Result<T, OperationError>;

// =============================================================================
// Operation error
// =============================================================================

/// Errors produced while constructing or validating one IR operation.
///
/// These are local operation errors. Whole-program error translation belongs
/// to the canonical `errors.rs` layer and can be added at the compiler
/// boundary without making this module depend on higher-level IR structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationError {
    /// The operation has no valid semantic body.
    EmptyBody,

    /// A reset operation must target exactly one logical qubit.
    ResetRequiresExactlyOneQubit {
        actual: usize,
    },

    /// A barrier must contain at least one logical qubit.
    EmptyBarrier,

    /// An operation contains duplicate logical qubit operands.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// A conditional operation has no target operation.
    MissingConditionalTarget,

    /// An operation was given an invalid operation identity.
    InvalidOperationId,

    /// A referenced pulse identity is invalid.
    InvalidPulseId,

    /// A referenced waveform identity is invalid.
    InvalidWaveformId,

    /// A referenced channel identity is invalid.
    InvalidChannelId,

    /// A referenced frame identity is invalid.
    InvalidFrameId,

    /// A referenced schedule identity is invalid.
    InvalidScheduleId,

    /// A referenced resource identity is invalid.
    InvalidResourceId,

    /// A referenced capability identity is invalid.
    InvalidCapabilityId,

    /// A referenced extension identity is invalid.
    InvalidExtensionId,

    /// An operation reference points to itself where that is not allowed.
    SelfReference {
        operation: OperationId,
    },

    /// A generic structural invariant failed.
    InvalidStructure {
        message: &'static str,
    },
}

impl fmt::Display for OperationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyBody => {
                write!(formatter, "operation body cannot be empty")
            }

            Self::ResetRequiresExactlyOneQubit { actual } => {
                write!(
                    formatter,
                    "reset requires exactly one logical qubit, received {actual}"
                )
            }

            Self::EmptyBarrier => {
                write!(
                    formatter,
                    "barrier must contain at least one logical qubit"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    formatter,
                    "operation contains duplicate logical qubit {qubit}"
                )
            }

            Self::MissingConditionalTarget => {
                write!(
                    formatter,
                    "conditional operation requires a target operation"
                )
            }

            Self::InvalidOperationId => {
                write!(
                    formatter,
                    "operation identity is invalid"
                )
            }

            Self::InvalidPulseId => {
                write!(
                    formatter,
                    "pulse identity is invalid"
                )
            }

            Self::InvalidWaveformId => {
                write!(
                    formatter,
                    "waveform identity is invalid"
                )
            }

            Self::InvalidChannelId => {
                write!(
                    formatter,
                    "channel identity is invalid"
                )
            }

            Self::InvalidFrameId => {
                write!(
                    formatter,
                    "frame identity is invalid"
                )
            }

            Self::InvalidScheduleId => {
                write!(
                    formatter,
                    "schedule identity is invalid"
                )
            }

            Self::InvalidResourceId => {
                write!(
                    formatter,
                    "resource identity is invalid"
                )
            }

            Self::InvalidCapabilityId => {
                write!(
                    formatter,
                    "capability identity is invalid"
                )
            }

            Self::InvalidExtensionId => {
                write!(
                    formatter,
                    "extension identity is invalid"
                )
            }

            Self::SelfReference { operation } => {
                write!(
                    formatter,
                    "operation {operation} cannot conditionally reference itself"
                )
            }

            Self::InvalidStructure { message } => {
                write!(
                    formatter,
                    "invalid operation structure: {message}"
                )
            }
        }
    }
}

impl std::error::Error for OperationError {}

// =============================================================================
// Operation class
// =============================================================================

/// Broad semantic class of an operation.
///
/// This is deliberately smaller than [`OperationBody`].
///
/// Consumers that only need to classify an operation should use this type
/// instead of matching every concrete operation variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OperationClass {
    /// A logical unitary or gate-level operation.
    Quantum,

    /// A quantum measurement.
    Measurement,

    /// A reset operation.
    Reset,

    /// A synchronization/barrier operation.
    Synchronization,

    /// A temporal delay.
    Timing,

    /// A pulse-level control reference.
    Pulse,

    /// A waveform-level reference.
    Waveform,

    /// A frame-control operation.
    Frame,

    /// A classical operation.
    Classical,

    /// A condition-dependent operation reference.
    Conditional,

    /// A quantum/classical resource allocation operation.
    Resource,

    /// A logical/fault-tolerant operation.
    Logical,

    /// An analog-program operation.
    Analog,

    /// An annealing/optimization-machine operation.
    Annealing,

    /// A scheduling reference.
    Schedule,

    /// An extension operation.
    Extension,

    /// A target capability declaration/reference.
    Capability,
}

impl OperationClass {
    /// Returns whether this class represents an operation that directly
    /// manipulates a logical quantum state.
    #[must_use]
    pub const fn is_quantum(self) -> bool {
        matches!(
            self,
            Self::Quantum
                | Self::Measurement
                | Self::Reset
                | Self::Pulse
                | Self::Analog
                | Self::Annealing
                | Self::Logical
        )
    }

    /// Returns whether this class is inherently non-unitary.
    #[must_use]
    pub const fn is_non_unitary(self) -> bool {
        matches!(
            self,
            Self::Measurement
                | Self::Reset
                | Self::Classical
                | Self::Conditional
                | Self::Resource
                | Self::Synchronization
                | Self::Timing
                | Self::Schedule
        )
    }
}

// =============================================================================
// Classical condition
// =============================================================================

/// Minimal, hardware-independent classical condition used directly by an
/// operation dependency.
///
/// The full expression/predicate system belongs to `classical.rs` and future
/// `control_flow.rs`.
///
/// This compact condition is deliberately useful for the common dynamic
/// circuit case:
///
/// ```text
/// if c0 == 1 {
///     x(q1)
/// }
/// ```
///
/// More expressive predicates can later be represented by a
/// `ClassicalPredicate`/control-flow object referenced from a higher-level
/// control-flow region without changing the operation identity model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationCondition {
    /// Classical bit whose logical value is inspected.
    bit: ClassicalBitId,

    /// Required Boolean value.
    value: bool,
}

impl OperationCondition {
    /// Creates a condition that requires `bit` to equal `value`.
    #[must_use]
    pub const fn new(
        bit: ClassicalBitId,
        value: bool,
    ) -> Self {
        Self { bit, value }
    }

    /// Creates `bit == 1`.
    #[must_use]
    pub const fn when_true(
        bit: ClassicalBitId,
    ) -> Self {
        Self::new(bit, true)
    }

    /// Creates `bit == 0`.
    #[must_use]
    pub const fn when_false(
        bit: ClassicalBitId,
    ) -> Self {
        Self::new(bit, false)
    }

    /// Returns the referenced classical bit.
    #[must_use]
    pub const fn bit(self) -> ClassicalBitId {
        self.bit
    }

    /// Returns the required Boolean value.
    #[must_use]
    pub const fn value(self) -> bool {
        self.value
    }
}

impl fmt::Display for OperationCondition {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{} == {}",
            self.bit,
            if self.value { 1 } else { 0 }
        )
    }
}

// =============================================================================
// Operation body
// =============================================================================

/// Universal operation body.
///
/// This is the central semantic vocabulary of `operation.rs`.
///
/// Specialized resources are referenced by strongly typed IDs instead of
/// importing their implementation modules. This keeps the dependency graph
/// acyclic and lets `operation.rs` be finalized before the specialized modules
/// are implemented.
///
/// The semantic owner of each referenced object remains its dedicated module.
#[derive(Debug, Clone, PartialEq)]
pub enum OperationBody {
    // -------------------------------------------------------------------------
    // Core quantum operations
    // -------------------------------------------------------------------------

    /// Logical gate operation.
    Gate(Gate),

    /// Quantum measurement.
    Measurement(Measurement),

    /// Reset one logical qubit.
    Reset {
        /// Logical qubit to reset.
        qubit: QubitId,
    },

    /// Synchronization barrier over one or more logical qubits.
    Barrier {
        /// Logical qubits covered by the barrier.
        qubits: Vec<QubitId>,
    },

    // -------------------------------------------------------------------------
    // Timing
    // -------------------------------------------------------------------------

    /// Hardware-independent delay represented by a future timing-layer
    /// identity/reference.
    ///
    /// `ScheduleId` identifies a timing/scheduling representation without
    /// making `operation.rs` depend on `timing.rs` or `schedule.rs`.
    Delay {
        /// Schedule/timing object defining the delay semantics.
        schedule: ScheduleId,
    },

    // -------------------------------------------------------------------------
    // Pulse/control
    // -------------------------------------------------------------------------

    /// Pulse-level semantic operation.
    ///
    /// The complete pulse definition is owned by `pulse.rs`.
    Pulse {
        /// Reference to the canonical pulse object.
        pulse: PulseId,
    },

    /// Waveform reference.
    ///
    /// The complete waveform definition is owned by `waveform.rs`.
    Waveform {
        /// Reference to the canonical waveform object.
        waveform: WaveformId,
    },

    /// Frame-control operation.
    ///
    /// The complete frame semantics are owned by `frame.rs`.
    FrameChange {
        /// Frame being selected/changed.
        frame: FrameId,
    },

    /// Abstract control/acquisition channel reference.
    ///
    /// Physical channels remain hardware-owned.
    Channel {
        /// Logical IR channel identity.
        channel: ChannelId,
    },

    // -------------------------------------------------------------------------
    // Classical / dynamic control
    // -------------------------------------------------------------------------

    /// A classical assignment/update reference.
    ///
    /// The detailed classical expression remains owned by `classical.rs`.
    ///
    /// `ClassicalBitId` identifies the destination resource without embedding
    /// a second classical expression system here.
    ClassicalAssign {
        /// Destination classical bit.
        destination: ClassicalBitId,
    },

    /// Conditionally activates an existing operation.
    ///
    /// The target is referenced by `OperationId` rather than nested directly.
    Conditional {
        /// Condition that must hold.
        condition: OperationCondition,

        /// Operation to execute when the condition is true.
        target: OperationId,
    },

    // -------------------------------------------------------------------------
    // Resource lifecycle
    // -------------------------------------------------------------------------

    /// Declares logical quantum resource allocation.
    AllocateQubits {
        /// Logical qubits allocated by this operation.
        qubits: Vec<QubitId>,
    },

    /// Releases logical quantum resources.
    ReleaseQubits {
        /// Logical qubits released by this operation.
        qubits: Vec<QubitId>,
    },

    // -------------------------------------------------------------------------
    // Logical / FTQC
    // -------------------------------------------------------------------------

    /// Logical/fault-tolerant operation reference.
    ///
    /// The actual logical operation definition belongs to the logical/QEC
    /// layers. The canonical operation graph only needs its identity.
    Logical {
        /// Logical resource/operation reference.
        resource: ResourceId,
    },

    // -------------------------------------------------------------------------
    // Analog
    // -------------------------------------------------------------------------

    /// Analog quantum-program operation.
    ///
    /// The actual analog semantics are defined by the analog subsystem.
    Analog {
        /// Abstract resource describing the analog operation.
        resource: ResourceId,
    },

    // -------------------------------------------------------------------------
    // Annealing
    // -------------------------------------------------------------------------

    /// Annealing / Ising / QUBO operation.
    ///
    /// The actual mathematical problem representation belongs to the
    /// annealing subsystem.
    Annealing {
        /// Abstract resource representing the annealing workload.
        resource: ResourceId,
    },

    // -------------------------------------------------------------------------
    // Scheduling / capabilities
    // -------------------------------------------------------------------------

    /// Explicit schedule reference.
    ///
    /// Scheduling computes the schedule. The operation only references it.
    Schedule {
        /// Schedule identity.
        schedule: ScheduleId,
    },

    /// Declares or references a required target capability.
    Capability {
        /// Required capability identity.
        capability: CapabilityId,
    },

    // -------------------------------------------------------------------------
    // Extensions
    // -------------------------------------------------------------------------

    /// Extensible operation reference.
    ///
    /// Extensions cannot bypass canonical validation, versioning or security
    /// policies.
    Extension {
        /// Extension identity.
        extension: ExtensionId,
    },
}

impl OperationBody {
    /// Returns the broad semantic class.
    #[must_use]
    pub const fn class(&self) -> OperationClass {
        match self {
            Self::Gate(_) => OperationClass::Quantum,
            Self::Measurement(_) => OperationClass::Measurement,
            Self::Reset { .. } => OperationClass::Reset,
            Self::Barrier { .. } => OperationClass::Synchronization,
            Self::Delay { .. } => OperationClass::Timing,
            Self::Pulse { .. } => OperationClass::Pulse,
            Self::Waveform { .. } => OperationClass::Waveform,
            Self::FrameChange { .. } => OperationClass::Frame,
            Self::Channel { .. } => OperationClass::Classical,
            Self::ClassicalAssign { .. } => OperationClass::Classical,
            Self::Conditional { .. } => OperationClass::Conditional,
            Self::AllocateQubits { .. }
            | Self::ReleaseQubits { .. } => {
                OperationClass::Resource
            }
            Self::Logical { .. } => OperationClass::Logical,
            Self::Analog { .. } => OperationClass::Analog,
            Self::Annealing { .. } => OperationClass::Annealing,
            Self::Schedule { .. } => OperationClass::Schedule,
            Self::Capability { .. } => OperationClass::Capability,
            Self::Extension { .. } => OperationClass::Extension,
        }
    }

    /// Returns whether this operation directly represents quantum computation.
    #[must_use]
    pub const fn is_quantum(&self) -> bool {
        self.class().is_quantum()
    }

    /// Returns whether this operation is a measurement.
    #[must_use]
    pub const fn is_measurement(&self) -> bool {
        matches!(self, Self::Measurement(_))
    }

    /// Returns whether this operation is a gate.
    #[must_use]
    pub const fn is_gate(&self) -> bool {
        matches!(self, Self::Gate(_))
    }

    /// Returns whether this operation is a pulse reference.
    #[must_use]
    pub const fn is_pulse(&self) -> bool {
        matches!(self, Self::Pulse { .. })
    }

    /// Returns whether this operation is conditional.
    #[must_use]
    pub const fn is_conditional(&self) -> bool {
        matches!(self, Self::Conditional { .. })
    }

    /// Returns the referenced operation if this is a conditional operation.
    #[must_use]
    pub const fn conditional_target(&self) -> Option<OperationId> {
        match self {
            Self::Conditional { target, .. } => Some(*target),
            _ => None,
        }
    }

    /// Returns the classical condition when this is a conditional operation.
    #[must_use]
    pub const fn condition(&self) -> Option<OperationCondition> {
        match self {
            Self::Conditional { condition, .. } => {
                Some(*condition)
            }
            _ => None,
        }
    }

    /// Returns a directly owned logical-qubit operand slice where one exists.
    ///
    /// Measurement-specific operands remain owned by `Measurement` because
    /// measurement semantics include basis, mode and destination information.
    ///
    /// This method deliberately does not manufacture a second measurement
    /// operand representation.
    #[must_use]
    pub fn explicit_qubits(&self) -> Option<&[QubitId]> {
        match self {
            Self::Gate(gate) => Some(gate.qubits()),

            Self::Reset { qubit } => {
                // This is represented by a temporary one-element view only
                // through `None`; callers should use `qubits_vec()` when they
                // require ownership.
                //
                // Returning a slice to a temporary would be invalid, so this
                // variant is intentionally handled by `qubits_vec()`.
                let _ = qubit;
                None
            }

            Self::Barrier { qubits }
            | Self::AllocateQubits { qubits }
            | Self::ReleaseQubits { qubits } => {
                Some(qubits.as_slice())
            }

            Self::Measurement(_)
            | Self::Delay { .. }
            | Self::Pulse { .. }
            | Self::Waveform { .. }
            | Self::FrameChange { .. }
            | Self::Channel { .. }
            | Self::ClassicalAssign { .. }
            | Self::Conditional { .. }
            | Self::Logical { .. }
            | Self::Analog { .. }
            | Self::Annealing { .. }
            | Self::Schedule { .. }
            | Self::Capability { .. }
            | Self::Extension { .. } => None,
        }
    }

    /// Returns directly represented logical qubits as an owned vector.
    ///
    /// This is intentionally allocation-producing because the operation
    /// variants have different storage layouts.
    pub fn qubits_vec(&self) -> Vec<QubitId> {
        match self {
            Self::Gate(gate) => gate.qubits().to_vec(),

            Self::Reset { qubit } => {
                vec![*qubit]
            }

            Self::Barrier { qubits }
            | Self::AllocateQubits { qubits }
            | Self::ReleaseQubits { qubits } => {
                qubits.clone()
            }

            Self::Measurement(_)
            | Self::Delay { .. }
            | Self::Pulse { .. }
            | Self::Waveform { .. }
            | Self::FrameChange { .. }
            | Self::Channel { .. }
            | Self::ClassicalAssign { .. }
            | Self::Conditional { .. }
            | Self::Logical { .. }
            | Self::Analog { .. }
            | Self::Annealing { .. }
            | Self::Schedule { .. }
            | Self::Capability { .. }
            | Self::Extension { .. } => {
                Vec::new()
            }
        }
    }

    /// Returns the referenced pulse identity, if any.
    #[must_use]
    pub const fn pulse_id(&self) -> Option<PulseId> {
        match self {
            Self::Pulse { pulse } => Some(*pulse),
            _ => None,
        }
    }

    /// Returns the referenced waveform identity, if any.
    #[must_use]
    pub const fn waveform_id(&self) -> Option<WaveformId> {
        match self {
            Self::Waveform { waveform } => Some(*waveform),
            _ => None,
        }
    }

    /// Returns the referenced frame identity, if any.
    #[must_use]
    pub const fn frame_id(&self) -> Option<FrameId> {
        match self {
            Self::FrameChange { frame } => Some(*frame),
            _ => None,
        }
    }

    /// Returns the referenced channel identity, if any.
    #[must_use]
    pub const fn channel_id(&self) -> Option<ChannelId> {
        match self {
            Self::Channel { channel } => Some(*channel),
            _ => None,
        }
    }

    /// Returns the referenced schedule identity, if any.
    #[must_use]
    pub const fn schedule_id(&self) -> Option<ScheduleId> {
        match self {
            Self::Delay { schedule }
            | Self::Schedule { schedule } => Some(*schedule),
            _ => None,
        }
    }

    /// Returns the referenced resource identity, if any.
    #[must_use]
    pub const fn resource_id(&self) -> Option<ResourceId> {
        match self {
            Self::Logical { resource }
            | Self::Analog { resource }
            | Self::Annealing { resource } => Some(*resource),
            _ => None,
        }
    }

    /// Returns the referenced capability identity, if any.
    #[must_use]
    pub const fn capability_id(&self) -> Option<CapabilityId> {
        match self {
            Self::Capability { capability } => {
                Some(*capability)
            }
            _ => None,
        }
    }

    /// Returns the referenced extension identity, if any.
    #[must_use]
    pub const fn extension_id(&self) -> Option<ExtensionId> {
        match self {
            Self::Extension { extension } => {
                Some(*extension)
            }
            _ => None,
        }
    }
}

// =============================================================================
// Operation
// =============================================================================

/// Canonical immutable Quantum IR operation.
///
/// An operation consists of:
///
/// ```text
/// OperationId + OperationBody
/// ```
///
/// The ID provides stable identity while the body provides semantic meaning.
///
/// Operations should be treated as immutable values after construction.
/// Compiler transformations should construct replacement operations or
/// explicitly preserve identity when the transformation semantics guarantee
/// that identity preservation is valid.
#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    id: OperationId,
    body: OperationBody,
}

impl Operation {
    /// Creates a new operation after local structural validation.
    pub fn new(
        id: OperationId,
        body: OperationBody,
    ) -> OperationResult<Self> {
        validate_operation_id(id)?;
        validate_body(&body)?;

        Ok(Self { id, body })
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn id(&self) -> OperationId {
        self.id
    }

    /// Returns the operation body.
    #[must_use]
    pub const fn body(&self) -> &OperationBody {
        &self.body
    }

    /// Returns the broad semantic class.
    #[must_use]
    pub const fn class(&self) -> OperationClass {
        self.body.class()
    }

    /// Returns whether this is a quantum operation.
    #[must_use]
    pub const fn is_quantum(&self) -> bool {
        self.body.is_quantum()
    }

    /// Returns whether this is a gate.
    #[must_use]
    pub const fn is_gate(&self) -> bool {
        self.body.is_gate()
    }

    /// Returns whether this is a measurement.
    #[must_use]
    pub const fn is_measurement(&self) -> bool {
        self.body.is_measurement()
    }

    /// Returns whether this is a pulse operation.
    #[must_use]
    pub const fn is_pulse(&self) -> bool {
        self.body.is_pulse()
    }

    /// Returns whether this is conditional.
    #[must_use]
    pub const fn is_conditional(&self) -> bool {
        self.body.is_conditional()
    }

    /// Returns the condition when this operation is conditional.
    #[must_use]
    pub const fn condition(&self) -> Option<OperationCondition> {
        self.body.condition()
    }

    /// Returns the target operation when this operation is conditional.
    #[must_use]
    pub const fn conditional_target(&self) -> Option<OperationId> {
        self.body.conditional_target()
    }

    /// Returns the directly represented logical qubits.
    ///
    /// For reset operations this returns one element.
    ///
    /// For measurement operations, use the measurement's canonical API because
    /// measurement semantics own their operand representation.
    pub fn qubits(&self) -> Vec<QubitId> {
        self.body.qubits_vec()
    }

    /// Returns the number of directly represented logical qubit operands.
    ///
    /// This is intentionally calculated without making assumptions about
    /// maximum machine size.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.body.qubits_vec().len()
    }

    /// Returns the gate when this is a gate operation.
    #[must_use]
    pub const fn gate(&self) -> Option<&Gate> {
        match &self.body {
            OperationBody::Gate(gate) => Some(gate),
            _ => None,
        }
    }

    /// Returns the measurement when this is a measurement operation.
    #[must_use]
    pub const fn measurement(&self) -> Option<&Measurement> {
        match &self.body {
            OperationBody::Measurement(measurement) => {
                Some(measurement)
            }
            _ => None,
        }
    }

    /// Returns the referenced pulse identity.
    #[must_use]
    pub const fn pulse_id(&self) -> Option<PulseId> {
        self.body.pulse_id()
    }

    /// Returns the referenced waveform identity.
    #[must_use]
    pub const fn waveform_id(&self) -> Option<WaveformId> {
        self.body.waveform_id()
    }

    /// Returns the referenced frame identity.
    #[must_use]
    pub const fn frame_id(&self) -> Option<FrameId> {
        self.body.frame_id()
    }

    /// Returns the referenced channel identity.
    #[must_use]
    pub const fn channel_id(&self) -> Option<ChannelId> {
        self.body.channel_id()
    }

    /// Returns the referenced schedule identity.
    #[must_use]
    pub const fn schedule_id(&self) -> Option<ScheduleId> {
        self.body.schedule_id()
    }

    /// Returns the referenced resource identity.
    #[must_use]
    pub const fn resource_id(&self) -> Option<ResourceId> {
        self.body.resource_id()
    }

    /// Returns the referenced capability identity.
    #[must_use]
    pub const fn capability_id(&self) -> Option<CapabilityId> {
        self.body.capability_id()
    }

    /// Returns the referenced extension identity.
    #[must_use]
    pub const fn extension_id(&self) -> Option<ExtensionId> {
        self.body.extension_id()
    }

    /// Validates this operation's local invariants.
    ///
    /// Program-wide namespace and resource validation belongs to
    /// `validation.rs`.
    pub fn validate(&self) -> OperationResult<()> {
        validate_operation_id(self.id)?;
        validate_body(&self.body)?;

        if let OperationBody::Conditional {
            target,
            ..
        } = self.body
        {
            if target == self.id {
                return Err(
                    OperationError::SelfReference {
                        operation: self.id,
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Constructors — gate
// =============================================================================

impl Operation {
    /// Creates a gate operation.
    pub fn gate(
        id: OperationId,
        gate: Gate,
    ) -> OperationResult<Self> {
        Self::new(id, OperationBody::Gate(gate))
    }

    /// Creates a measurement operation.
    pub fn measurement(
        id: OperationId,
        measurement: Measurement,
    ) -> OperationResult<Self> {
        Self::new(
            id,
            OperationBody::Measurement(measurement),
        )
    }

    /// Creates a reset operation.
    pub fn reset(
        id: OperationId,
        qubit: QubitId,
    ) -> OperationResult<Self> {
        Self::new(
            id,
            OperationBody::Reset { qubit },
        )
    }

    /// Creates a barrier over one or more logical qubits.
    pub fn barrier<I>(
        id: OperationId,
        qubits: I,
    ) -> OperationResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::new(
            id,
            OperationBody::Barrier {
                qubits: collect_unique_qubits(qubits)?,
            },
        )
    }

    /// Creates a pulse-level operation.
    ///
    /// The pulse definition itself belongs to `pulse.rs`.
    pub fn pulse(
        id: OperationId,
        pulse: PulseId,
    ) -> OperationResult<Self> {
        validate_pulse_id(pulse)?;

        Self::new(
            id,
            OperationBody::Pulse { pulse },
        )
    }

    /// Creates a waveform reference operation.
    pub fn waveform(
        id: OperationId,
        waveform: WaveformId,
    ) -> OperationResult<Self> {
        validate_waveform_id(waveform)?;

        Self::new(
            id,
            OperationBody::Waveform { waveform },
        )
    }

    /// Creates a frame-change operation.
    pub fn frame_change(
        id: OperationId,
        frame: FrameId,
    ) -> OperationResult<Self> {
        validate_frame_id(frame)?;

        Self::new(
            id,
            OperationBody::FrameChange { frame },
        )
    }

    /// Creates an abstract channel operation.
    pub fn channel(
        id: OperationId,
        channel: ChannelId,
    ) -> OperationResult<Self> {
        validate_channel_id(channel)?;

        Self::new(
            id,
            OperationBody::Channel { channel },
        )
    }

    /// Creates a conditional reference to another operation.
    ///
    /// The target operation must be different from this operation.
    pub fn conditional(
        id: OperationId,
        condition: OperationCondition,
        target: OperationId,
    ) -> OperationResult<Self> {
        validate_operation_id(id)?;
        validate_operation_id(target)?;

        if id == target {
            return Err(
                OperationError::SelfReference {
                    operation: id,
                },
            );
        }

        Self::new(
            id,
            OperationBody::Conditional {
                condition,
                target,
            },
        )
    }

    /// Creates a logical-qubit allocation operation.
    pub fn allocate_qubits<I>(
        id: OperationId,
        qubits: I,
    ) -> OperationResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::new(
            id,
            OperationBody::AllocateQubits {
                qubits: collect_unique_qubits(qubits)?,
            },
        )
    }

    /// Creates a logical-qubit release operation.
    pub fn release_qubits<I>(
        id: OperationId,
        qubits: I,
    ) -> OperationResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::new(
            id,
            OperationBody::ReleaseQubits {
                qubits: collect_unique_qubits(qubits)?,
            },
        )
    }

    /// Creates a logical/fault-tolerant operation reference.
    pub fn logical(
        id: OperationId,
        resource: ResourceId,
    ) -> OperationResult<Self> {
        validate_resource_id(resource)?;

        Self::new(
            id,
            OperationBody::Logical { resource },
        )
    }

    /// Creates an analog operation reference.
    pub fn analog(
        id: OperationId,
        resource: ResourceId,
    ) -> OperationResult<Self> {
        validate_resource_id(resource)?;

        Self::new(
            id,
            OperationBody::Analog { resource },
        )
    }

    /// Creates an annealing operation reference.
    pub fn annealing(
        id: OperationId,
        resource: ResourceId,
    ) -> OperationResult<Self> {
        validate_resource_id(resource)?;

        Self::new(
            id,
            OperationBody::Annealing { resource },
        )
    }

    /// Creates an explicit schedule reference.
    pub fn schedule(
        id: OperationId,
        schedule: ScheduleId,
    ) -> OperationResult<Self> {
        validate_schedule_id(schedule)?;

        Self::new(
            id,
            OperationBody::Schedule { schedule },
        )
    }

    /// Creates a capability-reference operation.
    pub fn capability(
        id: OperationId,
        capability: CapabilityId,
    ) -> OperationResult<Self> {
        validate_capability_id(capability)?;

        Self::new(
            id,
            OperationBody::Capability { capability },
        )
    }

    /// Creates an extension operation.
    pub fn extension(
        id: OperationId,
        extension: ExtensionId,
    ) -> OperationResult<Self> {
        validate_extension_id(extension)?;

        Self::new(
            id,
            OperationBody::Extension { extension },
        )
    }
}

// =============================================================================
// Operation sequence
// =============================================================================

/// Deterministic ordered operation sequence.
///
/// This is intentionally a thin wrapper over `Vec<Operation>`.
///
/// It does not impose a semantic maximum size.
///
/// Large programs are limited only by:
///
/// - explicit compiler/resource policy;
/// - host address space;
/// - available memory;
/// - downstream execution constraints.
///
/// Program-wide limits belong to `QuantumIrLimits`, not this container.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OperationSequence {
    operations: Vec<Operation>,
}

impl OperationSequence {
    /// Creates an empty operation sequence.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Creates a sequence from an iterator.
    pub fn from_iter<I>(
        operations: I,
    ) -> OperationResult<Self>
    where
        I: IntoIterator<Item = Operation>,
    {
        let mut sequence = Self::new();

        for operation in operations {
            sequence.push(operation)?;
        }

        Ok(sequence)
    }

    /// Returns the number of operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the sequence is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Appends an operation.
    ///
    /// Duplicate operation identities are rejected.
    pub fn push(
        &mut self,
        operation: Operation,
    ) -> OperationResult<()> {
        operation.validate()?;

        if self
            .operations
            .iter()
            .any(|existing| existing.id() == operation.id())
        {
            return Err(
                OperationError::InvalidStructure {
                    message:
                        "duplicate operation identity in sequence",
                },
            );
        }

        self.operations.push(operation);

        Ok(())
    }

    /// Returns an operation by sequence position.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Operation> {
        self.operations.get(index)
    }

    /// Returns an operation by stable identity.
    #[must_use]
    pub fn find(
        &self,
        id: OperationId,
    ) -> Option<&Operation> {
        self.operations
            .iter()
            .find(|operation| operation.id() == id)
    }

    /// Returns all operations in semantic order.
    #[must_use]
    pub fn as_slice(&self) -> &[Operation] {
        &self.operations
    }

    /// Returns an iterator over operations.
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, Operation> {
        self.operations.iter()
    }

    /// Validates operation identity uniqueness and local operation invariants.
    pub fn validate(&self) -> OperationResult<()> {
        let mut ids = BTreeSet::new();

        for operation in &self.operations {
            operation.validate()?;

            if !ids.insert(operation.id()) {
                return Err(
                    OperationError::InvalidStructure {
                        message:
                            "operation sequence contains duplicate operation identities",
                    },
                );
            }
        }

        Ok(())
    }
}

impl<'a> IntoIterator for &'a OperationSequence {
    type Item = &'a Operation;
    type IntoIter = std::slice::Iter<'a, Operation>;

    fn into_iter(self) -> Self::IntoIter {
        self.operations.iter()
    }
}

impl IntoIterator for OperationSequence {
    type Item = Operation;
    type IntoIter = std::vec::IntoIter<Operation>;

    fn into_iter(self) -> Self::IntoIter {
        self.operations.into_iter()
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_operation_id(
    id: OperationId,
) -> OperationResult<()> {
    //
    // OperationId is opaque and all u64 values are structurally representable.
    // Zero is intentionally NOT rejected: identity allocation policy belongs
    // to the owning program/compiler.
    //
    let _ = id;

    Ok(())
}

fn validate_pulse_id(
    id: PulseId,
) -> OperationResult<()> {
    let _ = id;

    Ok(())
}

fn validate_waveform_id(
    id: WaveformId,
) -> OperationResult<()> {
    let _ = id;

    Ok(())
}

fn validate_channel_id(
    id: ChannelId,
) -> OperationResult<()> {
    let _ = id;

    Ok(())
}

fn validate_frame_id(
    id: FrameId,
) -> OperationResult<()> {
    let _ = id;

    Ok(())
}

fn validate_schedule_id(
    id: ScheduleId,
) -> OperationResult<()> {
    let _ = id;

    Ok(())
}

fn validate_resource_id(
    id: ResourceId,
) -> OperationResult<()> {
    let _ = id;

    Ok(())
}

fn validate_capability_id(
    id: CapabilityId,
) -> OperationResult<()> {
    let _ = id;

    Ok(())
}

fn validate_extension_id(
    id: ExtensionId,
) -> OperationResult<()> {
    let _ = id;

    Ok(())
}

fn validate_body(
    body: &OperationBody,
) -> OperationResult<()> {
    match body {
        OperationBody::Gate(gate) => {
            //
            // Gate-level semantic validation remains owned by gate.rs.
            //
            // We deliberately do not duplicate its complete validation rules
            // here because that would create two sources of truth.
            //
            let _ = gate;

            Ok(())
        }

        OperationBody::Measurement(measurement) => {
            let _ = measurement;

            Ok(())
        }

        OperationBody::Reset { .. } => Ok(()),

        OperationBody::Barrier { qubits } => {
            validate_non_empty_unique_qubits(qubits)
        }

        OperationBody::Delay { schedule } => {
            validate_schedule_id(*schedule)
        }

        OperationBody::Pulse { pulse } => {
            validate_pulse_id(*pulse)
        }

        OperationBody::Waveform { waveform } => {
            validate_waveform_id(*waveform)
        }

        OperationBody::FrameChange { frame } => {
            validate_frame_id(*frame)
        }

        OperationBody::Channel { channel } => {
            validate_channel_id(*channel)
        }

        OperationBody::ClassicalAssign {
            destination,
        } => {
            let _ = destination;

            Ok(())
        }

        OperationBody::Conditional {
            condition: _,
            target,
        } => {
            validate_operation_id(*target)
        }

        OperationBody::AllocateQubits { qubits }
        | OperationBody::ReleaseQubits { qubits } => {
            validate_non_empty_unique_qubits(qubits)
        }

        OperationBody::Logical { resource }
        | OperationBody::Analog { resource }
        | OperationBody::Annealing { resource } => {
            validate_resource_id(*resource)
        }

        OperationBody::Schedule { schedule } => {
            validate_schedule_id(*schedule)
        }

        OperationBody::Capability { capability } => {
            validate_capability_id(*capability)
        }

        OperationBody::Extension { extension } => {
            validate_extension_id(*extension)
        }
    }
}

fn collect_unique_qubits<I>(
    qubits: I,
) -> OperationResult<Vec<QubitId>>
where
    I: IntoIterator<Item = QubitId>,
{
    let mut result = Vec::new();
    let mut seen = BTreeSet::new();

    for qubit in qubits {
        if !seen.insert(qubit) {
            return Err(
                OperationError::DuplicateQubit { qubit },
            );
        }

        result.push(qubit);
    }

    if result.is_empty() {
        return Err(OperationError::EmptyBarrier);
    }

    Ok(result)
}

fn validate_non_empty_unique_qubits(
    qubits: &[QubitId],
) -> OperationResult<()> {
    if qubits.is_empty() {
        return Err(OperationError::EmptyBarrier);
    }

    let mut seen = BTreeSet::new();

    for &qubit in qubits {
        if !seen.insert(qubit) {
            return Err(
                OperationError::DuplicateQubit { qubit },
            );
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation_id(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn qubit(value: usize) -> QubitId {
        QubitId::new(value)
    }

    fn classical_bit(value: usize) -> ClassicalBitId {
        ClassicalBitId::new(value)
    }

    fn pulse_id(value: u64) -> PulseId {
        PulseId::new(value)
    }

    fn waveform_id(value: u64) -> WaveformId {
        WaveformId::new(value)
    }

    fn channel_id(value: u64) -> ChannelId {
        ChannelId::new(value)
    }

    fn frame_id(value: u64) -> FrameId {
        FrameId::new(value)
    }

    fn schedule_id(value: u64) -> ScheduleId {
        ScheduleId::new(value)
    }

    fn resource_id(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    fn capability_id(value: u64) -> CapabilityId {
        CapabilityId::new(value)
    }

    fn extension_id(value: u64) -> ExtensionId {
        ExtensionId::new(value)
    }

    #[test]
    fn operation_id_is_preserved() {
        let id = operation_id(42);

        let operation = Operation::pulse(
            id,
            pulse_id(7),
        )
        .expect("pulse operation should be valid");

        assert_eq!(operation.id(), id);
    }

    #[test]
    fn pulse_operation_is_first_class() {
        let operation = Operation::pulse(
            operation_id(1),
            pulse_id(100),
        )
        .expect("pulse operation should be valid");

        assert!(operation.is_pulse());
        assert_eq!(
            operation.pulse_id(),
            Some(pulse_id(100))
        );
        assert_eq!(
            operation.class(),
            OperationClass::Pulse
        );
    }

    #[test]
    fn reset_requires_one_qubit_by_construction() {
        let operation = Operation::reset(
            operation_id(1),
            qubit(0),
        )
        .expect("reset should be valid");

        assert_eq!(
            operation.qubits(),
            vec![qubit(0)]
        );
        assert_eq!(
            operation.class(),
            OperationClass::Reset
        );
    }

    #[test]
    fn barrier_rejects_empty_operands() {
        let result =
            Operation::barrier(
                operation_id(1),
                std::iter::empty::<QubitId>(),
            );

        assert!(matches!(
            result,
            Err(OperationError::EmptyBarrier)
        ));
    }

    #[test]
    fn barrier_rejects_duplicate_qubits() {
        let result = Operation::barrier(
            operation_id(1),
            vec![qubit(0), qubit(0)],
        );

        assert!(matches!(
            result,
            Err(OperationError::DuplicateQubit {
                qubit
            }) if qubit == qubit(0)
        ));
    }

    #[test]
    fn conditional_operation_uses_stable_identity_reference() {
        let condition =
            OperationCondition::when_true(
                classical_bit(0),
            );

        let operation =
            Operation::conditional(
                operation_id(2),
                condition,
                operation_id(1),
            )
            .expect("conditional operation should be valid");

        assert!(operation.is_conditional());

        assert_eq!(
            operation.conditional_target(),
            Some(operation_id(1))
        );

        assert_eq!(
            operation.condition(),
            Some(condition)
        );
    }

    #[test]
    fn conditional_self_reference_is_rejected() {
        let result =
            Operation::conditional(
                operation_id(1),
                OperationCondition::when_true(
                    classical_bit(0),
                ),
                operation_id(1),
            );

        assert!(matches!(
            result,
            Err(OperationError::SelfReference {
                operation
            }) if operation == operation_id(1)
        ));
    }

    #[test]
    fn logical_analog_and_annealing_references_are_distinct() {
        let logical = Operation::logical(
            operation_id(1),
            resource_id(10),
        )
        .expect("logical operation should be valid");

        let analog = Operation::analog(
            operation_id(2),
            resource_id(11),
        )
        .expect("analog operation should be valid");

        let annealing = Operation::annealing(
            operation_id(3),
            resource_id(12),
        )
        .expect("annealing operation should be valid");

        assert_eq!(
            logical.class(),
            OperationClass::Logical
        );
        assert_eq!(
            analog.class(),
            OperationClass::Analog
        );
        assert_eq!(
            annealing.class(),
            OperationClass::Annealing
        );
    }

    #[test]
    fn specialized_identity_references_are_typed() {
        let waveform = Operation::waveform(
            operation_id(1),
            waveform_id(1),
        )
        .expect("waveform should be valid");

        let frame = Operation::frame_change(
            operation_id(2),
            frame_id(2),
        )
        .expect("frame should be valid");

        let channel = Operation::channel(
            operation_id(3),
            channel_id(3),
        )
        .expect("channel should be valid");

        let schedule = Operation::schedule(
            operation_id(4),
            schedule_id(4),
        )
        .expect("schedule should be valid");

        let capability = Operation::capability(
            operation_id(5),
            capability_id(5),
        )
        .expect("capability should be valid");

        let extension = Operation::extension(
            operation_id(6),
            extension_id(6),
        )
        .expect("extension should be valid");

        assert_eq!(
            waveform.waveform_id(),
            Some(waveform_id(1))
        );
        assert_eq!(
            frame.frame_id(),
            Some(frame_id(2))
        );
        assert_eq!(
            channel.channel_id(),
            Some(channel_id(3))
        );
        assert_eq!(
            schedule.schedule_id(),
            Some(schedule_id(4))
        );
        assert_eq!(
            capability.capability_id(),
            Some(capability_id(5))
        );
        assert_eq!(
            extension.extension_id(),
            Some(extension_id(6))
        );
    }

    #[test]
    fn operation_sequence_rejects_duplicate_identity() {
        let first = Operation::pulse(
            operation_id(1),
            pulse_id(1),
        )
        .expect("first operation should be valid");

        let second = Operation::pulse(
            operation_id(1),
            pulse_id(2),
        )
        .expect("second operation should be locally valid");

        let mut sequence =
            OperationSequence::new();

        sequence
            .push(first)
            .expect("first push should succeed");

        let result = sequence.push(second);

        assert!(matches!(
            result,
            Err(OperationError::InvalidStructure { .. })
        ));
    }

    #[test]
    fn operation_sequence_preserves_order() {
        let first = Operation::pulse(
            operation_id(10),
            pulse_id(10),
        )
        .expect("first operation should be valid");

        let second = Operation::pulse(
            operation_id(20),
            pulse_id(20),
        )
        .expect("second operation should be valid");

        let mut sequence =
            OperationSequence::new();

        sequence
            .push(first)
            .expect("first push should succeed");

        sequence
            .push(second)
            .expect("second push should succeed");

        assert_eq!(
            sequence.get(0).map(Operation::id),
            Some(operation_id(10))
        );

        assert_eq!(
            sequence.get(1).map(Operation::id),
            Some(operation_id(20))
        );
    }

    #[test]
    fn large_logical_qubit_identifiers_are_not_architectural_limits() {
        let large = QubitId::new(
            usize::MAX - 1,
        );

        let operation =
            Operation::reset(
                operation_id(u64::MAX),
                large,
            )
            .expect(
                "large logical identifiers must remain representable",
            );

        assert_eq!(
            operation.qubits(),
            vec![large]
        );
    }

    #[test]
    fn operation_validation_is_idempotent() {
        let operation =
            Operation::pulse(
                operation_id(100),
                pulse_id(200),
            )
            .expect("operation should be valid");

        operation
            .validate()
            .expect("first validation");

        operation
            .validate()
            .expect("second validation");
    }
}