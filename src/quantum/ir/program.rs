//! Zamani Quantum IR — Universal Quantum Program
//!
//! Canonical top-level representation of a hardware-independent Zamani
//! quantum program.
//!
//! # Architectural role
//!
//! `QuantumProgram` is the universal semantic container for quantum
//! computation in Zamani. It is deliberately broader than
//! `QuantumCircuit`.
//!
//! A circuit is one possible representation of a quantum computation.
//! A program may additionally contain:
//!
//! - logical qubit declarations;
//! - classical resources;
//! - parameters;
//! - ordered operations;
//! - nested regions;
//! - control-flow boundaries;
//! - logical-to-physical mapping references;
//! - resource requirements;
//! - capability requirements;
//! - timing metadata;
//! - pulse/control-program references;
//! - analog-program references;
//! - annealing/program references;
//! - fault-tolerant intent;
//! - provenance;
//! - extensible semantic metadata.
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written once and is independent of the
//! eventual quantum machine.
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! QuantumProgram
//!      │
//!      ├── 1-qubit target
//!      ├── 100-qubit target
//!      ├── 10,000-qubit target
//!      ├── distributed target
//!      ├── logical/FTQC target
//!      ├── simulator
//!      └── future quantum architecture
//! ```
//!
//! The program therefore does NOT contain an architectural maximum such as
//! 63, 4096, or 1_000_000 qubits.
//!
//! Concrete resource limits are explicit policy.
//!
//! Physical capacity is a property of the selected target.
//!
//! Routing determines logical-to-physical placement.
//!
//! Scheduling determines execution time.
//!
//! Optimization transforms the program.
//!
//! Hardware determines what actually exists.
//!
//! Backends determine how the target executes the resulting program.
//!
//! # Architectural boundaries
//!
//! This file owns:
//!
//! - the universal program container;
//! - program identity;
//! - logical resource declarations;
//! - classical resource declarations;
//! - ordered semantic operation records;
//! - program regions;
//! - program-level requirements;
//! - mapping references;
//! - program metadata;
//! - program-level validation of local structural invariants;
//! - deterministic insertion order.
//!
//! This file does NOT own:
//!
//! - hardware topology;
//! - routing algorithms;
//! - physical-qubit allocation algorithms;
//! - calibration;
//! - scheduling algorithms;
//! - pulse compilation;
//! - backend-specific gate decomposition;
//! - QPU communication;
//! - simulator state;
//! - optimization algorithms;
//! - QEC decoding;
//! - frontend parsing.
//!
//! Those concerns belong to their respective Quantum subsystems.
//!
//! # Pulse-level example
//!
//! A source-level function such as:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! must ultimately be representable by the universal program model without
//! forcing the program to know which physical channel, DAC, calibration,
//! device frequency, or hardware instruction will execute it.
//!
//! Conceptually:
//!
//! ```text
//! QuantumProgram
//!   └── Region
//!       └── Pulse semantic operation
//!             ├── logical target = q
//!             ├── amplitude = 0.3
//!             └── duration = 20ns
//! ```
//!
//! The actual channel, calibration, waveform lowering, scheduling, and
//! backend instruction selection occur downstream.
//!
//! # Scalability
//!
//! The representation uses `usize` for collection indexes/counts because
//! these are host-memory quantities, not quantum-machine limits.
//!
//! No fixed qubit-count constant appears in this module.
//!
//! Very large programs are bounded only by:
//!
//! 1. the host representation's addressable memory;
//! 2. explicit `QuantumIrLimits` policy;
//! 3. downstream compiler/resource policies;
//! 4. actual target capabilities.
//!
//! A program may therefore represent any finite machine size supported by
//! the execution environment.
//!
//! "Infinite qubits" is not represented as a physical machine. The semantic
//! model has no fixed architectural ceiling, while every concrete program
//! remains finite and resource-checkable.
//!
//! # Atomic mutation
//!
//! Mutating methods validate inputs before changing program state whenever
//! practical. Resource counters use checked arithmetic.
//!
//! Failed insertions do not partially append an item.
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
//! No `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

use super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Program identity
// =============================================================================

/// Opaque identity of a universal quantum program.
///
/// A program identity is semantic/compiler identity. It is not a hardware
/// job ID and does not identify a cloud-provider submission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProgramId(u64);

impl ProgramId {
    /// Creates a program identifier from a caller-controlled value.
    ///
    /// The constructor does not inspect global uniqueness. Uniqueness is a
    /// responsibility of the caller or higher-level program manager.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl Default for ProgramId {
    fn default() -> Self {
        Self::new(0)
    }
}

impl fmt::Display for ProgramId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "program-{}", self.0)
    }
}

// =============================================================================
// Region identity
// =============================================================================

/// Opaque identity for a program region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegionId(u64);

impl RegionId {
    /// Creates a region identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for RegionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "region-{}", self.0)
    }
}

// =============================================================================
// Operation identity
// =============================================================================

/// Opaque identity for a semantic program operation.
///
/// This is intentionally local to `program.rs` rather than assuming that a
/// future `operation.rs` implementation already exists.
///
/// Once `operation.rs` becomes canonical, this ID can be bridged there without
/// changing the program's structural contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProgramOperationId(u64);

impl ProgramOperationId {
    /// Creates an operation identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ProgramOperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "op-{}", self.0)
    }
}

// =============================================================================
// Program error
// =============================================================================

/// Errors produced by universal quantum-program construction and mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramError {
    /// Program identity is invalid for the requested operation.
    InvalidProgramId,

    /// Region identity is already present.
    DuplicateRegion {
        /// Existing region.
        region: RegionId,
    },

    /// Region identity does not exist.
    UnknownRegion {
        /// Requested region.
        region: RegionId,
    },

    /// Operation identity is already present.
    DuplicateOperation {
        /// Existing operation identity.
        operation: ProgramOperationId,
    },

    /// An operation refers to a logical qubit that has not been declared.
    UnknownLogicalQubit {
        /// Referenced logical qubit.
        qubit: QubitId,
    },

    /// A mapping refers to an undeclared logical qubit.
    UnknownMappingLogicalQubit {
        /// Referenced logical qubit.
        qubit: QubitId,
    },

    /// A program contains the same logical-to-physical mapping key twice.
    DuplicateMapping {
        /// Logical qubit whose mapping already exists.
        logical: QubitId,
    },

    /// The supplied mapping conflicts with an existing physical assignment.
    PhysicalQubitAlreadyMapped {
        /// Physical qubit already assigned.
        physical: PhysicalQubitId,

        /// Logical qubit currently assigned to it.
        existing_logical: QubitId,
    },

    /// An operation is structurally invalid.
    InvalidOperation {
        /// Static explanation.
        message: &'static str,
    },

    /// A region is structurally invalid.
    InvalidRegion {
        /// Static explanation.
        message: &'static str,
    },

    /// Program metadata is invalid.
    InvalidMetadata {
        /// Static explanation.
        message: &'static str,
    },

    /// A resource count exceeds an explicit policy.
    ResourceLimitExceeded {
        /// Resource name.
        resource: &'static str,

        /// Requested amount.
        requested: usize,

        /// Maximum allowed amount.
        maximum: usize,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Calculation description.
        calculation: &'static str,
    },

    /// The program is internally inconsistent.
    InvalidProgram {
        /// Static explanation.
        message: &'static str,
    },
}

impl fmt::Display for ProgramError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProgramId => {
                formatter.write_str("invalid quantum program identifier")
            }

            Self::DuplicateRegion { region } => {
                write!(formatter, "duplicate program region {region}")
            }

            Self::UnknownRegion { region } => {
                write!(formatter, "unknown program region {region}")
            }

            Self::DuplicateOperation { operation } => {
                write!(formatter, "duplicate program operation {operation}")
            }

            Self::UnknownLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "operation references undeclared logical qubit {qubit}"
                )
            }

            Self::UnknownMappingLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "mapping references undeclared logical qubit {qubit}"
                )
            }

            Self::DuplicateMapping { logical } => {
                write!(
                    formatter,
                    "logical qubit {logical} already has a physical mapping"
                )
            }

            Self::PhysicalQubitAlreadyMapped {
                physical,
                existing_logical,
            } => {
                write!(
                    formatter,
                    "physical qubit {physical} is already mapped to logical qubit {existing_logical}"
                )
            }

            Self::InvalidOperation { message } => {
                write!(formatter, "invalid program operation: {message}")
            }

            Self::InvalidRegion { message } => {
                write!(formatter, "invalid program region: {message}")
            }

            Self::InvalidMetadata { message } => {
                write!(formatter, "invalid program metadata: {message}")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "{resource} resource limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidProgram { message } => {
                write!(formatter, "invalid quantum program: {message}")
            }
        }
    }
}

impl std::error::Error for ProgramError {}

// =============================================================================
// Program result
// =============================================================================

/// Result type used by the universal program API.
pub type ProgramResult<T> = Result<T, ProgramError>;

// =============================================================================
// Operation kind
// =============================================================================

/// Semantic class of a universal quantum-program operation.
///
/// These are intentionally technology-independent categories.
///
/// The actual operation payload is represented by `ProgramOperation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProgramOperationKind {
    /// Gate-level quantum operation.
    Gate,

    /// Quantum measurement.
    Measurement,

    /// Quantum reset.
    Reset,

    /// Delay or semantic idle.
    Delay,

    /// Synchronization boundary.
    Synchronization,

    /// Pulse-level control operation.
    Pulse,

    /// Waveform reference/control operation.
    Waveform,

    /// Frame or phase-control operation.
    Frame,

    /// Acquisition/readout operation.
    Acquire,

    /// Classical operation.
    Classical,

    /// Conditional quantum operation.
    Conditional,

    /// Structured control-flow operation.
    ControlFlow,

    /// Logical/fault-tolerant operation.
    Logical,

    /// Analog quantum operation.
    Analog,

    /// Annealing / Ising / QUBO operation.
    Annealing,

    /// Resource allocation/declaration operation.
    Allocation,

    /// Resource release operation.
    Release,

    /// Extension-defined semantic operation.
    Extension,
}

impl fmt::Display for ProgramOperationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Gate => "gate",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::Delay => "delay",
            Self::Synchronization => "synchronization",
            Self::Pulse => "pulse",
            Self::Waveform => "waveform",
            Self::Frame => "frame",
            Self::Acquire => "acquire",
            Self::Classical => "classical",
            Self::Conditional => "conditional",
            Self::ControlFlow => "control-flow",
            Self::Logical => "logical",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Allocation => "allocation",
            Self::Release => "release",
            Self::Extension => "extension",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Program operation
// =============================================================================

/// A technology-independent semantic program operation.
///
/// This type intentionally stores the operation's semantic category and
/// resource references without embedding backend-specific implementation.
///
/// Detailed gate, measurement, pulse, waveform, frame, timing, and control
/// flow payloads are expected to be supplied by their corresponding IR
/// modules as those modules become canonical.
///
/// `ProgramOperation` therefore acts as a stable program-level envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramOperation {
    /// Stable program-local operation identity.
    id: ProgramOperationId,

    /// Semantic operation category.
    kind: ProgramOperationKind,

    /// Logical qubits consumed by the operation.
    logical_qubits: Vec<QubitId>,

    /// Optional classical resources consumed by the operation.
    classical_bits: Vec<usize>,

    /// Optional region containing this operation.
    region: Option<RegionId>,

    /// Optional semantic operation name.
    name: Option<String>,
}

impl ProgramOperation {
    /// Creates a quantum operation with logical-qubit operands.
    ///
    /// Operands are copied into owned storage so callers cannot mutate
    /// program state through an external slice.
    pub fn try_new(
        id: ProgramOperationId,
        kind: ProgramOperationKind,
        logical_qubits: &[QubitId],
    ) -> ProgramResult<Self> {
        if logical_qubits.len() > 0 {
            let mut unique = logical_qubits.to_vec();
            unique.sort_unstable();
            unique.dedup();

            if unique.len() != logical_qubits.len() {
                return Err(ProgramError::InvalidOperation {
                    message: "an operation contains duplicate logical qubit operands",
                });
            }
        }

        Ok(Self {
            id,
            kind,
            logical_qubits: logical_qubits.to_vec(),
            classical_bits: Vec::new(),
            region: None,
            name: None,
        })
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn id(&self) -> ProgramOperationId {
        self.id
    }

    /// Returns the semantic operation category.
    #[must_use]
    pub const fn kind(&self) -> ProgramOperationKind {
        self.kind
    }

    /// Returns logical qubit operands.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns classical-bit operands.
    #[must_use]
    pub fn classical_bits(&self) -> &[usize] {
        &self.classical_bits
    }

    /// Returns the containing region, if any.
    #[must_use]
    pub const fn region(&self) -> Option<RegionId> {
        self.region
    }

    /// Returns the optional semantic name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Adds classical-bit operands.
    ///
    /// Duplicate classical bits are rejected.
    pub fn with_classical_bits(
        mut self,
        classical_bits: &[usize],
    ) -> ProgramResult<Self> {
        let mut unique = classical_bits.to_vec();
        unique.sort_unstable();
        unique.dedup();

        if unique.len() != classical_bits.len() {
            return Err(ProgramError::InvalidOperation {
                message: "an operation contains duplicate classical-bit operands",
            });
        }

        self.classical_bits = classical_bits.to_vec();
        Ok(self)
    }

    /// Associates the operation with a region.
    #[must_use]
    pub const fn with_region(mut self, region: RegionId) -> Self {
        self.region = Some(region);
        self
    }

    /// Gives the operation a semantic name.
    ///
    /// The name is metadata only. It does not select a backend instruction.
    pub fn with_name<S>(mut self, name: S) -> ProgramResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.is_empty() {
            return Err(ProgramError::InvalidOperation {
                message: "operation name cannot be empty",
            });
        }

        self.name = Some(name);
        Ok(self)
    }
}

// =============================================================================
// Program region
// =============================================================================

/// A structured region in a universal quantum program.
///
/// Regions allow future control-flow, function, pulse, logical, analog and
/// other semantic structures to share the same top-level program model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramRegion {
    /// Region identity.
    id: RegionId,

    /// Optional parent region.
    parent: Option<RegionId>,

    /// Optional semantic name.
    name: Option<String>,

    /// Operation identities in deterministic program order.
    operations: Vec<ProgramOperationId>,
}

impl ProgramRegion {
    /// Creates an empty root-capable region.
    pub fn try_new(id: RegionId) -> ProgramResult<Self> {
        Ok(Self {
            id,
            parent: None,
            name: None,
            operations: Vec::new(),
        })
    }

    /// Returns the region identity.
    #[must_use]
    pub const fn id(&self) -> RegionId {
        self.id
    }

    /// Returns the parent region.
    #[must_use]
    pub const fn parent(&self) -> Option<RegionId> {
        self.parent
    }

    /// Returns the semantic name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns operations belonging to this region.
    #[must_use]
    pub fn operations(&self) -> &[ProgramOperationId] {
        &self.operations
    }

    /// Sets the parent region.
    #[must_use]
    pub const fn with_parent(mut self, parent: RegionId) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Sets the region name.
    pub fn with_name<S>(mut self, name: S) -> ProgramResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.is_empty() {
            return Err(ProgramError::InvalidRegion {
                message: "region name cannot be empty",
            });
        }

        self.name = Some(name);
        Ok(self)
    }

    fn push_operation(&mut self, operation: ProgramOperationId) {
        self.operations.push(operation);
    }
}

// =============================================================================
// Resource requirements
// =============================================================================

/// Abstract resources requested by a quantum program.
///
/// These are requirements, not claims about physical hardware.
///
/// Hardware capacity is evaluated later against the selected target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ProgramRequirements {
    /// Minimum logical qubits required.
    logical_qubits: usize,

    /// Minimum classical bits required.
    classical_bits: usize,

    /// Whether pulse-level control is semantically required.
    pulse_control: bool,

    /// Whether dynamic/mid-circuit classical control is required.
    dynamic_control: bool,

    /// Whether analog execution semantics are required.
    analog_execution: bool,

    /// Whether annealing semantics are required.
    annealing_execution: bool,

    /// Whether logical/fault-tolerant semantics are required.
    fault_tolerant_execution: bool,
}

impl ProgramRequirements {
    /// Creates empty requirements.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            logical_qubits: 0,
            classical_bits: 0,
            pulse_control: false,
            dynamic_control: false,
            analog_execution: false,
            annealing_execution: false,
            fault_tolerant_execution: false,
        }
    }

    /// Returns required logical-qubit count.
    #[must_use]
    pub const fn logical_qubits(&self) -> usize {
        self.logical_qubits
    }

    /// Returns required classical-bit count.
    #[must_use]
    pub const fn classical_bits(&self) -> usize {
        self.classical_bits
    }

    /// Returns whether pulse control is required.
    #[must_use]
    pub const fn pulse_control(&self) -> bool {
        self.pulse_control
    }

    /// Returns whether dynamic control is required.
    #[must_use]
    pub const fn dynamic_control(&self) -> bool {
        self.dynamic_control
    }

    /// Returns whether analog execution is required.
    #[must_use]
    pub const fn analog_execution(&self) -> bool {
        self.analog_execution
    }

    /// Returns whether annealing execution is required.
    #[must_use]
    pub const fn annealing_execution(&self) -> bool {
        self.annealing_execution
    }

    /// Returns whether fault-tolerant execution is required.
    #[must_use]
    pub const fn fault_tolerant_execution(&self) -> bool {
        self.fault_tolerant_execution
    }

    /// Sets the required logical-qubit count.
    #[must_use]
    pub const fn with_logical_qubits(
        mut self,
        count: usize,
    ) -> Self {
        self.logical_qubits = count;
        self
    }

    /// Sets the required classical-bit count.
    #[must_use]
    pub const fn with_classical_bits(
        mut self,
        count: usize,
    ) -> Self {
        self.classical_bits = count;
        self
    }

    /// Requires pulse-level control.
    #[must_use]
    pub const fn require_pulse_control(mut self) -> Self {
        self.pulse_control = true;
        self
    }

    /// Requires dynamic control.
    #[must_use]
    pub const fn require_dynamic_control(mut self) -> Self {
        self.dynamic_control = true;
        self
    }

    /// Requires analog execution semantics.
    #[must_use]
    pub const fn require_analog_execution(mut self) -> Self {
        self.analog_execution = true;
        self
    }

    /// Requires annealing execution semantics.
    #[must_use]
    pub const fn require_annealing_execution(mut self) -> Self {
        self.annealing_execution = true;
        self
    }

    /// Requires fault-tolerant execution semantics.
    #[must_use]
    pub const fn require_fault_tolerance(mut self) -> Self {
        self.fault_tolerant_execution = true;
        self
    }
}

// =============================================================================
// Logical-to-physical mapping
// =============================================================================

/// A program-level logical-to-physical mapping.
///
/// This is a mapping record, not a routing algorithm.
///
/// Routing decides the mapping. This structure merely records one when a
/// downstream stage has produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProgramMapping {
    /// Logical program qubit.
    logical: QubitId,

    /// Physical target qubit.
    physical: PhysicalQubitId,
}

impl ProgramMapping {
    /// Creates a logical-to-physical mapping record.
    #[must_use]
    pub const fn new(
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Self {
        Self { logical, physical }
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn logical(self) -> QubitId {
        self.logical
    }

    /// Returns the physical qubit.
    #[must_use]
    pub const fn physical(self) -> PhysicalQubitId {
        self.physical
    }
}

// =============================================================================
// Program metadata
// =============================================================================

/// Hardware-independent program metadata.
///
/// This structure deliberately excludes hardware configuration and
/// calibration.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProgramMetadata {
    /// Optional human-readable program name.
    name: Option<String>,

    /// Optional source/module identity.
    source: Option<String>,

    /// Optional compiler identity.
    compiler: Option<String>,

    /// Optional user-defined semantic tags.
    tags: BTreeMap<String, String>,

    /// Whether the program explicitly requests fault-tolerant semantics.
    fault_tolerant: bool,
}

impl ProgramMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the program name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the source identity.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Returns the compiler identity.
    #[must_use]
    pub fn compiler(&self) -> Option<&str> {
        self.compiler.as_deref()
    }

    /// Returns metadata tags.
    #[must_use]
    pub fn tags(&self) -> &BTreeMap<String, String> {
        &self.tags
    }

    /// Returns whether fault-tolerant semantics were requested.
    #[must_use]
    pub const fn fault_tolerant(&self) -> bool {
        self.fault_tolerant
    }

    /// Sets the program name.
    pub fn set_name<S>(&mut self, name: S) -> ProgramResult<()>
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.is_empty() {
            return Err(ProgramError::InvalidMetadata {
                message: "program name cannot be empty",
            });
        }

        self.name = Some(name);
        Ok(())
    }

    /// Sets the source identity.
    pub fn set_source<S>(&mut self, source: S) -> ProgramResult<()>
    where
        S: Into<String>,
    {
        let source = source.into();

        if source.is_empty() {
            return Err(ProgramError::InvalidMetadata {
                message: "source identity cannot be empty",
            });
        }

        self.source = Some(source);
        Ok(())
    }

    /// Sets the compiler identity.
    pub fn set_compiler<S>(&mut self, compiler: S) -> ProgramResult<()>
    where
        S: Into<String>,
    {
        let compiler = compiler.into();

        if compiler.is_empty() {
            return Err(ProgramError::InvalidMetadata {
                message: "compiler identity cannot be empty",
            });
        }

        self.compiler = Some(compiler);
        Ok(())
    }

    /// Adds or replaces a semantic metadata tag.
    pub fn insert_tag<S1, S2>(
        &mut self,
        key: S1,
        value: S2,
    ) -> ProgramResult<()>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        if key.is_empty() {
            return Err(ProgramError::InvalidMetadata {
                message: "metadata tag key cannot be empty",
            });
        }

        self.tags.insert(key, value);
        Ok(())
    }

    /// Marks the program as fault-tolerant.
    pub fn set_fault_tolerant(&mut self, enabled: bool) {
        self.fault_tolerant = enabled;
    }

    /// Returns UTF-8 metadata size using checked arithmetic.
    pub fn byte_size(&self) -> ProgramResult<usize> {
        let mut total = 0usize;

        if let Some(value) = &self.name {
            total = total
                .checked_add(value.len())
                .ok_or(ProgramError::ArithmeticOverflow {
                    calculation: "program metadata size",
                })?;
        }

        if let Some(value) = &self.source {
            total = total
                .checked_add(value.len())
                .ok_or(ProgramError::ArithmeticOverflow {
                    calculation: "program metadata size",
                })?;
        }

        if let Some(value) = &self.compiler {
            total = total
                .checked_add(value.len())
                .ok_or(ProgramError::ArithmeticOverflow {
                    calculation: "program metadata size",
                })?;
        }

        for (key, value) in &self.tags {
            total = total
                .checked_add(key.len())
                .and_then(|size| size.checked_add(value.len()))
                .ok_or(ProgramError::ArithmeticOverflow {
                    calculation: "program metadata tags size",
                })?;
        }

        Ok(total)
    }
}

// =============================================================================
// Universal quantum program
// =============================================================================

/// Canonical universal Zamani quantum program.
///
/// `QuantumProgram` is the top-level semantic container above a
/// `QuantumCircuit`.
///
/// It can represent workloads that are circuit-oriented, pulse-oriented,
/// dynamic, hybrid, logical/fault-tolerant, analog, annealing-oriented, or
/// extension-defined without making any particular hardware model canonical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumProgram {
    /// Stable program identity.
    id: ProgramId,

    /// Quantum IR schema major version.
    ir_major: u16,

    /// Quantum IR schema minor version.
    ir_minor: u16,

    /// Logical qubit namespace.
    ///
    /// The value is the number of declared logical qubits.
    logical_qubits: usize,

    /// Classical-bit namespace size.
    classical_bits: usize,

    /// Ordered top-level operations.
    operations: Vec<ProgramOperation>,

    /// Structured regions.
    regions: BTreeMap<RegionId, ProgramRegion>,

    /// Logical-to-physical mapping records.
    ///
    /// This does not perform routing.
    mappings: BTreeMap<QubitId, PhysicalQubitId>,

    /// Reverse physical-to-logical mapping.
    ///
    /// Keeping both indexes makes uniqueness checks deterministic and avoids
    /// requiring a scan of the full mapping for every insertion.
    reverse_mappings: BTreeMap<PhysicalQubitId, QubitId>,

    /// Abstract program requirements.
    requirements: ProgramRequirements,

    /// Program metadata.
    metadata: ProgramMetadata,

    /// Next operation identity.
    next_operation_id: u64,

    /// Next region identity.
    next_region_id: u64,
}

impl QuantumProgram {
    /// Current universal-program IR major version.
    pub const IR_MAJOR_VERSION: u16 = 1;

    /// Current universal-program IR minor version.
    pub const IR_MINOR_VERSION: u16 = 0;

    /// Creates an empty universal quantum program.
    ///
    /// The program has no architectural qubit limit. Concrete limits should
    /// be applied by the caller through `QuantumIrLimits` or the canonical
    /// validation layer.
    #[must_use]
    pub fn new(id: ProgramId) -> Self {
        Self {
            id,
            ir_major: Self::IR_MAJOR_VERSION,
            ir_minor: Self::IR_MINOR_VERSION,
            logical_qubits: 0,
            classical_bits: 0,
            operations: Vec::new(),
            regions: BTreeMap::new(),
            mappings: BTreeMap::new(),
            reverse_mappings: BTreeMap::new(),
            requirements: ProgramRequirements::new(),
            metadata: ProgramMetadata::new(),
            next_operation_id: 0,
            next_region_id: 0,
        }
    }

    /// Creates a program with explicit logical and classical namespaces.
    ///
    /// This constructor does not impose a fixed architecture-specific limit.
    pub fn with_resources(
        id: ProgramId,
        logical_qubits: usize,
        classical_bits: usize,
    ) -> Self {
        let mut program = Self::new(id);
        program.logical_qubits = logical_qubits;
        program.classical_bits = classical_bits;
        program.requirements = ProgramRequirements::new()
            .with_logical_qubits(logical_qubits)
            .with_classical_bits(classical_bits);
        program
    }

    /// Returns the program identity.
    #[must_use]
    pub const fn id(&self) -> ProgramId {
        self.id
    }

    /// Returns the IR major version.
    #[must_use]
    pub const fn ir_major(&self) -> u16 {
        self.ir_major
    }

    /// Returns the IR minor version.
    #[must_use]
    pub const fn ir_minor(&self) -> u16 {
        self.ir_minor
    }

    /// Returns the logical-qubit namespace size.
    #[must_use]
    pub const fn logical_qubit_count(&self) -> usize {
        self.logical_qubits
    }

    /// Returns the classical-bit namespace size.
    #[must_use]
    pub const fn classical_bit_count(&self) -> usize {
        self.classical_bits
    }

    /// Returns all program operations in deterministic order.
    #[must_use]
    pub fn operations(&self) -> &[ProgramOperation] {
        &self.operations
    }

    /// Returns the number of operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns all regions.
    #[must_use]
    pub fn regions(&self) -> &BTreeMap<RegionId, ProgramRegion> {
        &self.regions
    }

    /// Returns a region by identity.
    #[must_use]
    pub fn region(&self, id: RegionId) -> Option<&ProgramRegion> {
        self.regions.get(&id)
    }

    /// Returns all logical-to-physical mappings.
    ///
    /// Mapping is immutable through this accessor.
    #[must_use]
    pub fn mappings(&self) -> &BTreeMap<QubitId, PhysicalQubitId> {
        &self.mappings
    }

    /// Returns the physical qubit mapped to a logical qubit.
    #[must_use]
    pub fn physical_for(
        &self,
        logical: QubitId,
    ) -> Option<PhysicalQubitId> {
        self.mappings.get(&logical).copied()
    }

    /// Returns the logical qubit mapped to a physical qubit.
    #[must_use]
    pub fn logical_for(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<QubitId> {
        self.reverse_mappings.get(&physical).copied()
    }

    /// Returns program requirements.
    #[must_use]
    pub const fn requirements(&self) -> ProgramRequirements {
        self.requirements
    }

    /// Returns program metadata.
    #[must_use]
    pub fn metadata(&self) -> &ProgramMetadata {
        &self.metadata
    }

    /// Returns mutable metadata.
    ///
    /// Metadata setters still validate individual values.
    pub fn metadata_mut(&mut self) -> &mut ProgramMetadata {
        &mut self.metadata
    }

    /// Sets the logical-qubit namespace size.
    ///
    /// Shrinking below currently referenced qubits is rejected.
    pub fn set_logical_qubit_count(
        &mut self,
        count: usize,
    ) -> ProgramResult<()> {
        if count < self.logical_qubits {
            let mut offending = None;

            for operation in &self.operations {
                for qubit in operation.logical_qubits() {
                    if qubit.index() >= count {
                        offending = Some(*qubit);
                        break;
                    }
                }

                if offending.is_some() {
                    break;
                }
            }

            if offending.is_none() {
                offending = self
                    .mappings
                    .keys()
                    .find(|qubit| qubit.index() >= count)
                    .copied();
            }

            if let Some(qubit) = offending {
                return Err(ProgramError::UnknownLogicalQubit { qubit });
            }
        }

        self.logical_qubits = count;
        self.requirements = self
            .requirements
            .with_logical_qubits(count);

        Ok(())
    }

    /// Sets the classical-bit namespace size.
    ///
    /// Existing operation operands must remain inside the new namespace.
    pub fn set_classical_bit_count(
        &mut self,
        count: usize,
    ) -> ProgramResult<()> {
        for operation in &self.operations {
            for bit in operation.classical_bits() {
                if *bit >= count {
                    return Err(ProgramError::InvalidProgram {
                        message: "classical namespace would invalidate an existing operation",
                    });
                }
            }
        }

        self.classical_bits = count;
        self.requirements = self
            .requirements
            .with_classical_bits(count);

        Ok(())
    }

    /// Declares one additional logical qubit.
    ///
    /// This method is checked for `usize` overflow.
    pub fn allocate_logical_qubit(&mut self) -> ProgramResult<QubitId> {
        let id = QubitId::new(self.logical_qubits);

        self.logical_qubits = self
            .logical_qubits
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "logical qubit namespace size",
            })?;

        self.requirements = self
            .requirements
            .with_logical_qubits(self.logical_qubits);

        Ok(id)
    }

    /// Declares additional logical qubits.
    ///
    /// The allocation is atomic: if the count cannot be represented, no
    /// program state is changed.
    pub fn allocate_logical_qubits(
        &mut self,
        count: usize,
    ) -> ProgramResult<QubitId> {
        let new_count = self
            .logical_qubits
            .checked_add(count)
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "logical qubit namespace size",
            })?;

        let first = QubitId::new(self.logical_qubits);

        self.logical_qubits = new_count;
        self.requirements = self
            .requirements
            .with_logical_qubits(new_count);

        Ok(first)
    }

    /// Adds one classical bit to the namespace.
    pub fn allocate_classical_bit(&mut self) -> ProgramResult<usize> {
        let bit = self.classical_bits;

        self.classical_bits = self
            .classical_bits
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "classical-bit namespace size",
            })?;

        self.requirements = self
            .requirements
            .with_classical_bits(self.classical_bits);

        Ok(bit)
    }

    /// Adds multiple classical bits.
    pub fn allocate_classical_bits(
        &mut self,
        count: usize,
    ) -> ProgramResult<usize> {
        let new_count = self
            .classical_bits
            .checked_add(count)
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "classical-bit namespace size",
            })?;

        let first = self.classical_bits;

        self.classical_bits = new_count;
        self.requirements = self
            .requirements
            .with_classical_bits(new_count);

        Ok(first)
    }

    /// Creates a new program region.
    ///
    /// Region IDs are generated using checked arithmetic.
    pub fn create_region(&mut self) -> ProgramResult<RegionId> {
        let value = self.next_region_id;

        self.next_region_id = self
            .next_region_id
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "next region identity",
            })?;

        let id = RegionId::new(value);

        if self.regions.contains_key(&id) {
            return Err(ProgramError::DuplicateRegion { region: id });
        }

        let region = ProgramRegion::try_new(id)?;

        self.regions.insert(id, region);

        Ok(id)
    }

    /// Adds an explicitly identified region.
    pub fn add_region(
        &mut self,
        region: ProgramRegion,
    ) -> ProgramResult<()> {
        let id = region.id();

        if self.regions.contains_key(&id) {
            return Err(ProgramError::DuplicateRegion { region: id });
        }

        if let Some(parent) = region.parent() {
            if !self.regions.contains_key(&parent) {
                return Err(ProgramError::UnknownRegion { region: parent });
            }
        }

        self.regions.insert(id, region);

        Ok(())
    }

    /// Creates and inserts a new operation.
    ///
    /// The operation is rejected before insertion if it references an
    /// undeclared logical qubit or classical bit.
    pub fn push_operation(
        &mut self,
        mut operation: ProgramOperation,
    ) -> ProgramResult<ProgramOperationId> {
        for qubit in operation.logical_qubits() {
            if qubit.index() >= self.logical_qubits {
                return Err(ProgramError::UnknownLogicalQubit {
                    qubit: *qubit,
                });
            }
        }

        for bit in operation.classical_bits() {
            if *bit >= self.classical_bits {
                return Err(ProgramError::InvalidOperation {
                    message: "operation references an undeclared classical bit",
                });
            }
        }

        if let Some(region) = operation.region() {
            if !self.regions.contains_key(&region) {
                return Err(ProgramError::UnknownRegion { region });
            }
        }

        let id = operation.id();

        if self.operations.iter().any(|existing| existing.id() == id) {
            return Err(ProgramError::DuplicateOperation {
                operation: id,
            });
        }

        if operation.kind() == ProgramOperationKind::Pulse {
            self.requirements =
                self.requirements.require_pulse_control();
        }

        if operation.kind() == ProgramOperationKind::Conditional
            || operation.kind() == ProgramOperationKind::ControlFlow
        {
            self.requirements =
                self.requirements.require_dynamic_control();
        }

        if operation.kind() == ProgramOperationKind::Analog {
            self.requirements =
                self.requirements.require_analog_execution();
        }

        if operation.kind() == ProgramOperationKind::Annealing {
            self.requirements =
                self.requirements.require_annealing_execution();
        }

        if operation.kind() == ProgramOperationKind::Logical {
            self.requirements =
                self.requirements.require_fault_tolerance();
        }

        if operation.region().is_none() && !self.regions.is_empty() {
            if let Some(root) = self.regions.keys().next().copied() {
                operation = operation.with_region(root);

                if let Some(region) = self.regions.get_mut(&root) {
                    region.push_operation(id);
                }
            }
        } else if let Some(region_id) = operation.region() {
            if let Some(region) = self.regions.get_mut(&region_id) {
                region.push_operation(id);
            }
        }

        self.operations.push(operation);

        self.next_operation_id = self
            .next_operation_id
            .max(id.value().saturating_add(1));

        Ok(id)
    }

    /// Creates a new operation identity.
    ///
    /// The generated identity is checked for overflow.
    pub fn next_operation_id(
        &mut self,
    ) -> ProgramResult<ProgramOperationId> {
        let value = self.next_operation_id;

        self.next_operation_id = self
            .next_operation_id
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "next operation identity",
            })?;

        Ok(ProgramOperationId::new(value))
    }

    /// Creates and appends an operation using an automatically generated ID.
    pub fn append_operation(
        &mut self,
        kind: ProgramOperationKind,
        logical_qubits: &[QubitId],
    ) -> ProgramResult<ProgramOperationId> {
        let id = self.next_operation_id()?;

        let operation =
            ProgramOperation::try_new(id, kind, logical_qubits)?;

        self.push_operation(operation)
    }

    /// Records an explicit logical-to-physical mapping.
    ///
    /// This does not perform routing and does not verify that the physical
    /// qubit exists in hardware.
    ///
    /// It only guarantees that the mapping is internally unambiguous.
    pub fn map_qubit(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> ProgramResult<()> {
        if logical.index() >= self.logical_qubits {
            return Err(ProgramError::UnknownMappingLogicalQubit {
                qubit: logical,
            });
        }

        if self.mappings.contains_key(&logical) {
            return Err(ProgramError::DuplicateMapping { logical });
        }

        if let Some(existing_logical) =
            self.reverse_mappings.get(&physical).copied()
        {
            return Err(ProgramError::PhysicalQubitAlreadyMapped {
                physical,
                existing_logical,
            });
        }

        self.mappings.insert(logical, physical);
        self.reverse_mappings.insert(physical, logical);

        Ok(())
    }

    /// Records multiple mappings atomically.
    ///
    /// If any mapping conflicts or references an unknown logical qubit, the
    /// program is not modified.
    pub fn map_qubits(
        &mut self,
        mappings: &[ProgramMapping],
    ) -> ProgramResult<()> {
        let mut logicals = BTreeMap::new();
        let mut physicals = BTreeMap::new();

        for mapping in mappings {
            if mapping.logical().index() >= self.logical_qubits {
                return Err(ProgramError::UnknownMappingLogicalQubit {
                    qubit: mapping.logical(),
                });
            }

            if self.mappings.contains_key(&mapping.logical())
                || logicals.contains_key(&mapping.logical())
            {
                return Err(ProgramError::DuplicateMapping {
                    logical: mapping.logical(),
                });
            }

            if self
                .reverse_mappings
                .contains_key(&mapping.physical())
                || physicals.contains_key(&mapping.physical())
            {
                let existing = self
                    .reverse_mappings
                    .get(&mapping.physical())
                    .copied()
                    .or_else(|| physicals.get(&mapping.physical()).copied())
                    .unwrap_or(mapping.logical());

                return Err(
                    ProgramError::PhysicalQubitAlreadyMapped {
                        physical: mapping.physical(),
                        existing_logical: existing,
                    },
                );
            }

            logicals.insert(mapping.logical(), mapping.physical());
            physicals.insert(mapping.physical(), mapping.logical());
        }

        for (logical, physical) in logicals {
            self.mappings.insert(logical, physical);
        }

        for (physical, logical) in physicals {
            self.reverse_mappings.insert(physical, logical);
        }

        Ok(())
    }

    /// Removes an existing mapping.
    ///
    /// This does not change logical or physical hardware state. It only
    /// removes the program-level mapping record.
    pub fn unmap_qubit(
        &mut self,
        logical: QubitId,
    ) -> Option<PhysicalQubitId> {
        let physical = self.mappings.remove(&logical)?;

        self.reverse_mappings.remove(&physical);

        Some(physical)
    }

    /// Clears every program-level mapping.
    ///
    /// No hardware operation occurs.
    pub fn clear_mappings(&mut self) {
        self.mappings.clear();
        self.reverse_mappings.clear();
    }

    /// Sets explicit program requirements.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: ProgramRequirements,
    ) -> Self {
        self.requirements = requirements
            .with_logical_qubits(self.logical_qubits)
            .with_classical_bits(self.classical_bits);

        self
    }

    /// Replaces the program requirements after validating namespace counts.
    pub fn set_requirements(
        &mut self,
        requirements: ProgramRequirements,
    ) -> ProgramResult<()> {
        if requirements.logical_qubits() != self.logical_qubits {
            return Err(ProgramError::InvalidProgram {
                message: "program requirements logical-qubit count must match the program namespace",
            });
        }

        if requirements.classical_bits() != self.classical_bits {
            return Err(ProgramError::InvalidProgram {
                message: "program requirements classical-bit count must match the program namespace",
            });
        }

        self.requirements = requirements;

        Ok(())
    }

    /// Marks the program as requiring pulse-level control.
    pub fn require_pulse_control(&mut self) {
        self.requirements =
            self.requirements.require_pulse_control();
    }

    /// Marks the program as requiring dynamic control.
    pub fn require_dynamic_control(&mut self) {
        self.requirements =
            self.requirements.require_dynamic_control();
    }

    /// Marks the program as requiring fault-tolerant execution.
    pub fn require_fault_tolerance(&mut self) {
        self.requirements =
            self.requirements.require_fault_tolerance();

        self.metadata.set_fault_tolerant(true);
    }

    /// Validates local structural invariants.
    ///
    /// This method deliberately does not validate hardware compatibility.
    ///
    /// Hardware compatibility belongs to downstream target validation.
    pub fn validate(&self) -> ProgramResult<()> {
        if self.id.value() == u64::MAX {
            return Err(ProgramError::InvalidProgramId);
        }

        if self.ir_major == 0 {
            return Err(ProgramError::InvalidProgram {
                message: "IR major version must be non-zero",
            });
        }

        if self.requirements.logical_qubits()
            != self.logical_qubits
        {
            return Err(ProgramError::InvalidProgram {
                message: "logical-qubit requirement does not match program namespace",
            });
        }

        if self.requirements.classical_bits()
            != self.classical_bits
        {
            return Err(ProgramError::InvalidProgram {
                message: "classical-bit requirement does not match program namespace",
            });
        }

        let mut operation_ids = BTreeMap::new();

        for operation in &self.operations {
            if operation_ids
                .insert(operation.id(), ())
                .is_some()
            {
                return Err(ProgramError::DuplicateOperation {
                    operation: operation.id(),
                });
            }

            for qubit in operation.logical_qubits() {
                if qubit.index() >= self.logical_qubits {
                    return Err(ProgramError::UnknownLogicalQubit {
                        qubit: *qubit,
                    });
                }
            }

            for bit in operation.classical_bits() {
                if *bit >= self.classical_bits {
                    return Err(ProgramError::InvalidOperation {
                        message: "operation references an undeclared classical bit",
                    });
                }
            }

            if let Some(region) = operation.region() {
                if !self.regions.contains_key(&region) {
                    return Err(ProgramError::UnknownRegion { region });
                }
            }
        }

        for (region_id, region) in &self.regions {
            if let Some(parent) = region.parent() {
                if !self.regions.contains_key(&parent) {
                    return Err(ProgramError::UnknownRegion {
                        region: parent,
                    });
                }

                if parent == *region_id {
                    return Err(ProgramError::InvalidRegion {
                        message: "a region cannot be its own parent",
                    });
                }
            }

            for operation_id in region.operations() {
                if !operation_ids.contains_key(operation_id) {
                    return Err(ProgramError::InvalidRegion {
                        message: "region references an unknown operation",
                    });
                }
            }
        }

        for (logical, physical) in &self.mappings {
            if logical.index() >= self.logical_qubits {
                return Err(
                    ProgramError::UnknownMappingLogicalQubit {
                        qubit: *logical,
                    },
                );
            }

            match self.reverse_mappings.get(physical) {
                Some(mapped_logical) if mapped_logical == logical => {}
                _ => {
                    return Err(ProgramError::InvalidProgram {
                        message: "logical-to-physical mapping indexes are inconsistent",
                    });
                }
            }
        }

        for (physical, logical) in &self.reverse_mappings {
            match self.mappings.get(logical) {
                Some(mapped_physical) if mapped_physical == physical => {}
                _ => {
                    return Err(ProgramError::InvalidProgram {
                        message: "physical-to-logical mapping indexes are inconsistent",
                    });
                }
            }
        }

        self.metadata.byte_size()?;

        Ok(())
    }

    /// Returns whether the program is structurally valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Returns the number of logical qubits that currently have mappings.
    #[must_use]
    pub fn mapped_qubit_count(&self) -> usize {
        self.mappings.len()
    }

    /// Returns whether every declared logical qubit has a physical mapping.
    ///
    /// This is a mapping-state query only. It does not mean that the physical
    /// qubits actually exist or are executable.
    #[must_use]
    pub fn is_fully_mapped(&self) -> bool {
        self.mappings.len() == self.logical_qubits
    }

    /// Returns a deterministic semantic summary of the program.
    ///
    /// This is intentionally not a cryptographic hash. Canonical hashing
    /// belongs in the dedicated `hash.rs` module.
    #[must_use]
    pub fn summary(&self) -> ProgramSummary {
        let mut gate_count = 0usize;
        let mut measurement_count = 0usize;
        let mut pulse_count = 0usize;
        let mut classical_count = 0usize;
        let mut control_flow_count = 0usize;
        let mut logical_count = 0usize;
        let mut analog_count = 0usize;
        let mut annealing_count = 0usize;

        for operation in &self.operations {
            match operation.kind() {
                ProgramOperationKind::Gate => gate_count += 1,
                ProgramOperationKind::Measurement => {
                    measurement_count += 1
                }
                ProgramOperationKind::Pulse => pulse_count += 1,
                ProgramOperationKind::Classical => {
                    classical_count += 1
                }
                ProgramOperationKind::Conditional
                | ProgramOperationKind::ControlFlow => {
                    control_flow_count += 1
                }
                ProgramOperationKind::Logical => logical_count += 1,
                ProgramOperationKind::Analog => analog_count += 1,
                ProgramOperationKind::Annealing => {
                    annealing_count += 1
                }
                ProgramOperationKind::Reset
                | ProgramOperationKind::Delay
                | ProgramOperationKind::Synchronization
                | ProgramOperationKind::Waveform
                | ProgramOperationKind::Frame
                | ProgramOperationKind::Acquire
                | ProgramOperationKind::Allocation
                | ProgramOperationKind::Release
                | ProgramOperationKind::Extension => {}
            }
        }

        ProgramSummary {
            logical_qubits: self.logical_qubits,
            classical_bits: self.classical_bits,
            operations: self.operations.len(),
            regions: self.regions.len(),
            mappings: self.mappings.len(),
            gate_count,
            measurement_count,
            pulse_count,
            classical_count,
            control_flow_count,
            logical_operation_count: logical_count,
            analog_operation_count: analog_count,
            annealing_operation_count: annealing_count,
        }
    }
}

impl Default for QuantumProgram {
    fn default() -> Self {
        Self::new(ProgramId::default())
    }
}

// =============================================================================
// Program summary
// =============================================================================

/// Deterministic, read-only program statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProgramSummary {
    /// Number of declared logical qubits.
    pub logical_qubits: usize,

    /// Number of declared classical bits.
    pub classical_bits: usize,

    /// Number of semantic operations.
    pub operations: usize,

    /// Number of structured regions.
    pub regions: usize,

    /// Number of recorded logical-to-physical mappings.
    pub mappings: usize,

    /// Number of gate operations.
    pub gate_count: usize,

    /// Number of measurements.
    pub measurement_count: usize,

    /// Number of pulse operations.
    pub pulse_count: usize,

    /// Number of classical operations.
    pub classical_count: usize,

    /// Number of control-flow operations.
    pub control_flow_count: usize,

    /// Number of logical/fault-tolerant operations.
    pub logical_operation_count: usize,

    /// Number of analog operations.
    pub analog_operation_count: usize,

    /// Number of annealing operations.
    pub annealing_operation_count: usize,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_program_is_valid() {
        let program = QuantumProgram::new(ProgramId::new(1));

        assert!(program.validate().is_ok());
        assert_eq!(program.logical_qubit_count(), 0);
        assert_eq!(program.classical_bit_count(), 0);
        assert_eq!(program.operation_count(), 0);
    }

    #[test]
    fn logical_qubit_allocation_is_checked() {
        let mut program = QuantumProgram::new(ProgramId::new(2));

        let first = program
            .allocate_logical_qubit()
            .expect("allocation must succeed");

        assert_eq!(first.index(), 0);
        assert_eq!(program.logical_qubit_count(), 1);
    }

    #[test]
    fn operations_require_declared_qubits() {
        let mut program = QuantumProgram::new(ProgramId::new(3));

        let result = program.append_operation(
            ProgramOperationKind::Gate,
            &[QubitId::new(0)],
        );

        assert!(matches!(
            result,
            Err(ProgramError::UnknownLogicalQubit { .. })
        ));
    }

    #[test]
    fn operation_can_be_added_after_qubit_declaration() {
        let mut program = QuantumProgram::new(ProgramId::new(4));

        program
            .allocate_logical_qubit()
            .expect("allocation must succeed");

        let operation = program
            .append_operation(
                ProgramOperationKind::Gate,
                &[QubitId::new(0)],
            )
            .expect("operation must succeed");

        assert_eq!(operation.value(), 0);
        assert_eq!(program.operation_count(), 1);
        assert!(program.validate().is_ok());
    }

    #[test]
    fn duplicate_operands_are_rejected() {
        let result = ProgramOperation::try_new(
            ProgramOperationId::new(0),
            ProgramOperationKind::Gate,
            &[QubitId::new(0), QubitId::new(0)],
        );

        assert!(matches!(
            result,
            Err(ProgramError::InvalidOperation { .. })
        ));
    }

    #[test]
    fn mapping_requires_declared_logical_qubit() {
        let mut program = QuantumProgram::new(ProgramId::new(5));

        let result = program.map_qubit(
            QubitId::new(0),
            PhysicalQubitId::new(10),
        );

        assert!(matches!(
            result,
            Err(ProgramError::UnknownMappingLogicalQubit { .. })
        ));
    }

    #[test]
    fn mapping_is_bijective() {
        let mut program = QuantumProgram::with_resources(
            ProgramId::new(6),
            2,
            0,
        );

        program
            .map_qubit(
                QubitId::new(0),
                PhysicalQubitId::new(10),
            )
            .expect("first mapping must succeed");

        let result = program.map_qubit(
            QubitId::new(1),
            PhysicalQubitId::new(10),
        );

        assert!(matches!(
            result,
            Err(ProgramError::PhysicalQubitAlreadyMapped { .. })
        ));
    }

    #[test]
    fn mapping_can_be_removed() {
        let mut program = QuantumProgram::with_resources(
            ProgramId::new(7),
            1,
            0,
        );

        program
            .map_qubit(
                QubitId::new(0),
                PhysicalQubitId::new(5),
            )
            .expect("mapping must succeed");

        assert_eq!(
            program.physical_for(QubitId::new(0)),
            Some(PhysicalQubitId::new(5))
        );

        assert_eq!(
            program.unmap_qubit(QubitId::new(0)),
            Some(PhysicalQubitId::new(5))
        );

        assert_eq!(
            program.physical_for(QubitId::new(0)),
            None
        );
    }

    #[test]
    fn pulse_operation_sets_pulse_requirement() {
        let mut program = QuantumProgram::with_resources(
            ProgramId::new(8),
            1,
            0,
        );

        program
            .append_operation(
                ProgramOperationKind::Pulse,
                &[QubitId::new(0)],
            )
            .expect("pulse operation must succeed");

        assert!(program.requirements().pulse_control());
    }

    #[test]
    fn dynamic_operation_sets_dynamic_requirement() {
        let mut program = QuantumProgram::with_resources(
            ProgramId::new(9),
            2,
            1,
        );

        program
            .append_operation(
                ProgramOperationKind::Conditional,
                &[QubitId::new(0)],
            )
            .expect("conditional operation must succeed");

        assert!(program.requirements().dynamic_control());
    }

    #[test]
    fn logical_operation_sets_fault_tolerance_requirement() {
        let mut program = QuantumProgram::with_resources(
            ProgramId::new(10),
            1,
            0,
        );

        program
            .append_operation(
                ProgramOperationKind::Logical,
                &[QubitId::new(0)],
            )
            .expect("logical operation must succeed");

        assert!(
            program
                .requirements()
                .fault_tolerant_execution()
        );
    }

    #[test]
    fn region_can_contain_operations() {
        let mut program = QuantumProgram::with_resources(
            ProgramId::new(11),
            1,
            0,
        );

        let region = program
            .create_region()
            .expect("region must be created");

        let operation_id = program
            .next_operation_id()
            .expect("operation id must exist");

        let operation = ProgramOperation::try_new(
            operation_id,
            ProgramOperationKind::Gate,
            &[QubitId::new(0)],
        )
        .expect("operation must be valid")
        .with_region(region);

        program
            .push_operation(operation)
            .expect("operation must be inserted");

        assert_eq!(
            program
                .region(region)
                .expect("region must exist")
                .operations(),
            &[operation_id]
        );

        assert!(program.validate().is_ok());
    }

    #[test]
    fn metadata_is_checked() {
        let mut metadata = ProgramMetadata::new();

        assert!(metadata.set_name("").is_err());
        assert!(metadata.set_name("algorithm").is_ok());
        assert!(metadata
            .insert_tag("domain", "quantum")
            .is_ok());

        assert!(metadata.byte_size().is_ok());
    }

    #[test]
    fn summary_counts_operation_classes() {
        let mut program = QuantumProgram::with_resources(
            ProgramId::new(12),
            2,
            1,
        );

        program
            .append_operation(
                ProgramOperationKind::Gate,
                &[QubitId::new(0)],
            )
            .expect("gate must succeed");

        program
            .append_operation(
                ProgramOperationKind::Measurement,
                &[QubitId::new(0)],
            )
            .expect("measurement must succeed");

        program
            .append_operation(
                ProgramOperationKind::Pulse,
                &[QubitId::new(1)],
            )
            .expect("pulse must succeed");

        let summary = program.summary();

        assert_eq!(summary.gate_count, 1);
        assert_eq!(summary.measurement_count, 1);
        assert_eq!(summary.pulse_count, 1);
        assert_eq!(summary.operations, 3);
    }

    #[test]
    fn mapping_round_trip_is_consistent() {
        let mut program = QuantumProgram::with_resources(
            ProgramId::new(13),
            3,
            0,
        );

        program
            .map_qubits(&[
                ProgramMapping::new(
                    QubitId::new(0),
                    PhysicalQubitId::new(7),
                ),
                ProgramMapping::new(
                    QubitId::new(1),
                    PhysicalQubitId::new(11),
                ),
                ProgramMapping::new(
                    QubitId::new(2),
                    PhysicalQubitId::new(19),
                ),
            ])
            .expect("mappings must succeed");

        assert_eq!(
            program.logical_for(PhysicalQubitId::new(11)),
            Some(QubitId::new(1))
        );

        assert_eq!(
            program.physical_for(QubitId::new(2)),
            Some(PhysicalQubitId::new(19))
        );

        assert!(program.is_fully_mapped());
        assert!(program.validate().is_ok());
    }

    #[test]
    fn program_scales_without_architectural_qubit_constant() {
        let mut program = QuantumProgram::new(ProgramId::new(14));

        let count = 100_000usize;

        program
            .allocate_logical_qubits(count)
            .expect("large logical namespace must be representable");

        assert_eq!(
            program.logical_qubit_count(),
            count
        );

        assert!(program.validate().is_ok());
    }

    #[test]
    fn shrinking_namespace_cannot_invalidate_operations() {
        let mut program = QuantumProgram::with_resources(
            ProgramId::new(15),
            2,
            0,
        );

        program
            .append_operation(
                ProgramOperationKind::Gate,
                &[QubitId::new(1)],
            )
            .expect("operation must succeed");

        let result = program.set_logical_qubit_count(1);

        assert!(result.is_err());
        assert_eq!(program.logical_qubit_count(), 2);
    }

    #[test]
    fn shrinking_classical_namespace_cannot_invalidate_operations() {
        let mut program = QuantumProgram::with_resources(
            ProgramId::new(16),
            1,
            2,
        );

        let operation_id = program
            .next_operation_id()
            .expect("operation ID must exist");

        let operation = ProgramOperation::try_new(
            operation_id,
            ProgramOperationKind::Classical,
            &[QubitId::new(0)],
        )
        .expect("operation must succeed")
        .with_classical_bits(&[1])
        .expect("classical operands must succeed");

        program
            .push_operation(operation)
            .expect("operation must be inserted");

        assert!(program.set_classical_bit_count(1).is_err());
        assert_eq!(program.classical_bit_count(), 2);
    }
}