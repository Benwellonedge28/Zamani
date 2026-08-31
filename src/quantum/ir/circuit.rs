//! Zamani Quantum IR — Canonical Quantum Circuit
//!
//! `QuantumCircuit` is the ordered, hardware-independent circuit container
//! within the Zamani Quantum IR.
//!
//! # Architectural role
//!
//! The universal Zamani quantum-program architecture is:
//
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend
//!      |
//!      v
//! Quantum IR
//!      |
//!      +--> optimization
//!      |
//!      +--> routing
//!      |
//!      +--> scheduling
//!      |
//!      +--> hardware compatibility
//!      |
//!      +--> backend lowering
//!      |
//!      v
//! execution
//! ```
//!
//! `QuantumCircuit` owns:
//
//! - logical quantum/classical namespace sizes;
//! - ordered `Operation` values;
//! - stable circuit identity;
//! - IR schema version;
//! - circuit metadata;
//! - explicit resource policy;
//! - safe mutation boundaries;
//! - circuit-local structural invariants.
//!
//! It deliberately does NOT own:
//
//! - physical topology;
//! - logical-to-physical routing;
//! - hardware allocation;
//! - calibration;
//! - native instruction selection;
//! - pulse synthesis;
//! - scheduling;
//! - QPU communication;
//! - simulator state;
//! - optimization policy;
//! - QEC decoding;
//! - frontend parsing.
//!
//! Those responsibilities belong to downstream modules.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and is not tied to a particular machine
//! size.
//
//! `QuantumCircuit` therefore has no architectural qubit ceiling.
//
//! The following are all semantically representable:
//
//! ```text
//! 1
//! 63
//! 64
//! 128
//! 4_096
//! 1_000_000
//! N
//! ```
//!
//! A concrete `QuantumIrLimits` is a resource/security policy for one
//! compilation or service boundary. It is NOT a statement about the largest
//! quantum computer Zamani can describe.
//
//! A physical target may impose a smaller capacity. That belongs to
//! `quantum::hardware`, target compatibility, routing, and backend stages.
//!
//! # Operation model
//!
//! The circuit stores:
//
//! ```text
//! Vec<Operation>
//! ```
//!
//! rather than:
//
//! ```text
//! Vec<Gate>
//! ```
//!
//! This is essential for universal quantum computing.
//
//! One circuit can therefore contain semantic operations representing:
//
//! - gates;
//! - measurements;
//! - reset;
//! - barriers;
//! - pulse references;
//! - waveform references;
//! - frame changes;
//! - timing references;
//! - classical operations;
//! - conditional operations;
//! - logical/FTQC operations;
//! - analog operations;
//! - annealing operations;
//! - resource operations;
//! - extensions.
//!
//! Pulse-level source such as:
//
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! is represented through the canonical operation/pulse layers. The circuit
//! does not reinterpret the pulse as a gate and does not decide which physical
//! control channel or DAC will execute it.
//!
//! # Ordering
//!
//! `QuantumCircuit` preserves explicit program order.
//
//! This ordering is semantic and must not be silently changed by this module.
//! Optimizers may later transform the circuit while preserving semantic
//! equivalence.
//!
//! # Scalability
//!
//! This implementation deliberately avoids:
//
//! - allocating one object per declared qubit;
//! - allocating a vector sized to `num_qubits` during construction;
//! - fixed-size qubit arrays;
//! - 63-bit masks as machine-size representations;
//! - unchecked `usize` arithmetic;
//! - implicit operation-count limits;
//! - hardware-specific assumptions.
//!
//! Circuit storage scales with the actual number of stored operations, not
//! merely with the declared logical namespace.
//
//! # Security
//!
//! Resource limits are checked before mutating the circuit.
//
//! Failed mutation is atomic:
//
//! ```text
//! validate candidate
//!       |
//!       v
//! check limits
//!       |
//!       v
//! check namespace
//!       |
//!       v
//! check identity
//!       |
//!       v
//! mutate
//! ```
//!
//! No partially inserted operation is left behind after a failed insertion.
//!
//! # Rust compatibility
//!
//! Target:
//
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! The `forbid(unsafe_code)` attribute makes the no-unsafe requirement
//! compiler-enforced.
//!
//! # Integration contracts
//!
//! `qubit.rs`
//!     Owns canonical `QubitId` and `PhysicalQubitId`.
//!
//! `operation.rs`
//!     Owns the universal operation model used by this circuit.
//!
//! `gate.rs`
//!     Owns mathematical gate semantics.
//!
//! `measurement.rs`
//!     Owns measurement semantics.
//!
//! `limits.rs`
//!     Owns explicit IR resource policy.
//!
//! `identity.rs`
//!     Owns `CircuitId`, `OperationId`, and `IrVersion`.
//!
//! `validation.rs`
//!     Performs complete canonical whole-circuit validation.
//!
//! `analysis.rs`
//!     Consumes this circuit to calculate metrics.
//!
//! `optimization/`
//!     May transform circuits but must not redefine circuit semantics.
//!
//! `routing/`
//!     Maps logical qubits to physical resources.
//!
//! `scheduling/`
//!     Determines execution timing.
//!
//! `hardware/`
//!     Describes actual target capabilities and resources.
//!
//! `program.rs`
//!     May contain larger programs, regions, functions, and control flow.
//!
//! `serialization.rs`
//!     Serializes the circuit using the canonical IR schema.
//!
//! `hash.rs`
//!     Computes canonical content identities.
//!
//! `provenance.rs`
//!     Records transformation lineage.
//!
//! # Important naming rule
//!
//! The canonical qubit module is:
//
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! This file therefore imports:
//
//! ```rust
//! use super::qubit::QubitId;
//! ```
//!
//! and never `super::qubits`.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

use super::gate::Gate;
use super::identity::{CircuitId, IrVersion, OperationId};
use super::limits::{LimitsError, QuantumIrLimits};
use super::operation::{Operation, OperationBody, OperationError};
use super::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type for circuit construction and mutation.
pub type CircuitResult<T> = Result<T, CircuitError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing, validating, or modifying a circuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitError {
    /// The supplied IR resource policy is invalid.
    InvalidLimits {
        /// Name of the invalid policy field.
        field: &'static str,

        /// Invalid value.
        value: usize,
    },

    /// The logical qubit namespace exceeds its policy.
    QubitLimitExceeded {
        /// Requested number of logical qubits.
        requested: usize,

        /// Policy maximum.
        maximum: usize,
    },

    /// The classical namespace exceeds its policy.
    ClassicalBitLimitExceeded {
        /// Requested number of classical bits.
        requested: usize,

        /// Policy maximum.
        maximum: usize,
    },

    /// The operation count exceeds its policy.
    OperationLimitExceeded {
        /// Requested operation count.
        requested: usize,

        /// Policy maximum.
        maximum: usize,
    },

    /// The operation count cannot be incremented safely.
    OperationCountOverflow,

    /// Circuit metadata exceeds the configured policy.
    MetadataLimitExceeded {
        /// Metadata byte count.
        requested: usize,

        /// Policy maximum.
        maximum: usize,
    },

    /// A logical qubit is outside the circuit namespace.
    QubitOutOfRange {
        /// Referenced logical qubit.
        qubit: QubitId,

        /// Number of declared logical qubits.
        num_qubits: usize,
    },

    /// A classical bit is outside the circuit namespace.
    ClassicalBitOutOfRange {
        /// Referenced classical bit.
        bit: usize,

        /// Number of declared classical bits.
        num_classical_bits: usize,
    },

    /// An operation ID already exists in the circuit.
    DuplicateOperationId {
        /// Duplicated operation identity.
        id: OperationId,
    },

    /// An operation index is outside the operation sequence.
    OperationOutOfRange {
        /// Requested index.
        index: usize,

        /// Current operation count.
        len: usize,
    },

    /// The operation itself is structurally invalid.
    InvalidOperation(OperationError),

    /// A gate is not valid for the circuit namespace.
    InvalidGate(String),

    /// Circuit metadata is structurally invalid.
    InvalidMetadata {
        /// Static validation reason.
        message: &'static str,
    },

    /// The supplied IR version cannot be consumed by this implementation.
    UnsupportedVersion {
        /// Unsupported version.
        version: IrVersion,
    },

    /// A circuit-wide invariant is invalid.
    InvalidCircuit {
        /// Static validation reason.
        message: &'static str,
    },

    /// A circuit analysis calculation overflowed.
    ArithmeticOverflow {
        /// Name of the calculation.
        calculation: &'static str,
    },

    /// A whole-circuit validation failure.
    Validation(String),
}

impl fmt::Display for CircuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimits { field, value } => {
                write!(
                    f,
                    "invalid quantum IR limit `{field}`: {value}"
                )
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "logical qubit limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::ClassicalBitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "classical-bit limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "operation limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::OperationCountOverflow => {
                f.write_str("operation count overflow")
            }

            Self::MetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "metadata limit exceeded: requested {requested} bytes, maximum {maximum}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "logical qubit {qubit} is outside namespace 0..{num_qubits}"
                )
            }

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => {
                write!(
                    f,
                    "classical bit c{bit} is outside namespace 0..{num_classical_bits}"
                )
            }

            Self::DuplicateOperationId { id } => {
                write!(f, "operation identity {id} already exists")
            }

            Self::OperationOutOfRange { index, len } => {
                write!(
                    f,
                    "operation index {index} is outside circuit length {len}"
                )
            }

            Self::InvalidOperation(error) => {
                write!(f, "invalid operation: {error}")
            }

            Self::InvalidGate(message) => {
                write!(f, "invalid gate: {message}")
            }

            Self::InvalidMetadata { message } => {
                write!(f, "invalid circuit metadata: {message}")
            }

            Self::UnsupportedVersion { version } => {
                write!(
                    f,
                    "unsupported quantum IR version {version}"
                )
            }

            Self::InvalidCircuit { message } => {
                write!(f, "invalid circuit: {message}")
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    f,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::Validation(message) => {
                write!(f, "quantum IR validation failed: {message}")
            }
        }
    }
}

impl std::error::Error for CircuitError {}

impl From<OperationError> for CircuitError {
    fn from(error: OperationError) -> Self {
        Self::InvalidOperation(error)
    }
}

impl From<LimitsError> for CircuitError {
    fn from(error: LimitsError) -> Self {
        match error {
            LimitsError::InvalidConfiguration { field, value } => {
                Self::InvalidLimits { field, value }
            }

            LimitsError::ResourceExceeded {
                resource,
                requested,
                maximum,
            } => {
                let name = resource.as_str();

                if name == "logical qubits" {
                    Self::QubitLimitExceeded {
                        requested,
                        maximum,
                    }
                } else if name == "classical bits" {
                    Self::ClassicalBitLimitExceeded {
                        requested,
                        maximum,
                    }
                } else if name == "operations" {
                    Self::OperationLimitExceeded {
                        requested,
                        maximum,
                    }
                } else if name == "metadata bytes" {
                    Self::MetadataLimitExceeded {
                        requested,
                        maximum,
                    }
                } else {
                    Self::InvalidCircuit {
                        message: "resource policy rejected circuit resource",
                    }
                }
            }

            LimitsError::ArithmeticOverflow { .. }
            | LimitsError::ArithmeticMultiplicationOverflow { .. }
            | LimitsError::TimeArithmeticOverflow => {
                Self::ArithmeticOverflow {
                    calculation: "IR resource accounting",
                }
            }

            LimitsError::ScheduleTimeExceeded { .. } => {
                Self::InvalidCircuit {
                    message: "schedule-time policy is not a circuit namespace limit",
                }
            }
        }
    }
}

// =============================================================================
// Metadata
// =============================================================================

/// Circuit-level metadata.
///
/// Metadata is intentionally limited to logical/compiler provenance. Hardware
/// topology, calibration, DAC configuration, physical channels, and backend
/// state do not belong here.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CircuitMetadata {
    name: Option<String>,
    source: Option<String>,
    compiler_version: Option<String>,
    fault_tolerant: bool,
}

impl CircuitMetadata {
    /// Creates empty circuit metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: None,
            source: None,
            compiler_version: None,
            fault_tolerant: false,
        }
    }

    /// Returns the optional circuit name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the optional source identifier.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Returns the optional compiler version.
    #[must_use]
    pub fn compiler_version(&self) -> Option<&str> {
        self.compiler_version.as_deref()
    }

    /// Returns whether the circuit is marked as fault-tolerant.
    #[must_use]
    pub const fn fault_tolerant(&self) -> bool {
        self.fault_tolerant
    }

    /// Sets the circuit name.
    pub fn set_name(
        &mut self,
        name: Option<String>,
    ) {
        self.name = name;
    }

    /// Sets the source identifier.
    pub fn set_source(
        &mut self,
        source: Option<String>,
    ) {
        self.source = source;
    }

    /// Sets the compiler version.
    pub fn set_compiler_version(
        &mut self,
        compiler_version: Option<String>,
    ) {
        self.compiler_version = compiler_version;
    }

    /// Sets the fault-tolerant marker.
    pub const fn set_fault_tolerant(
        &mut self,
        fault_tolerant: bool,
    ) {
        self.fault_tolerant = fault_tolerant;
    }

    /// Returns total UTF-8 metadata storage in bytes.
    pub fn byte_size(&self) -> CircuitResult<usize> {
        let mut total = 0usize;

        if let Some(value) = &self.name {
            total = total
                .checked_add(value.len())
                .ok_or(CircuitError::ArithmeticOverflow {
                    calculation: "metadata byte size",
                })?;
        }

        if let Some(value) = &self.source {
            total = total
                .checked_add(value.len())
                .ok_or(CircuitError::ArithmeticOverflow {
                    calculation: "metadata byte size",
                })?;
        }

        if let Some(value) = &self.compiler_version {
            total = total
                .checked_add(value.len())
                .ok_or(CircuitError::ArithmeticOverflow {
                    calculation: "metadata byte size",
                })?;
        }

        Ok(total)
    }
}

// =============================================================================
// Circuit
// =============================================================================

/// Canonical ordered Zamani quantum circuit.
///
/// A circuit is a specialized straight-line quantum-program container. It is
/// not the complete universal quantum program representation; higher-level
/// `program.rs`/region/control-flow layers may contain multiple circuits or
/// regions.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCircuit {
    id: CircuitId,
    version: IrVersion,

    /// Logical quantum namespace size.
    num_qubits: usize,

    /// Logical classical namespace size.
    num_classical_bits: usize,

    /// Explicit resource/security policy.
    limits: QuantumIrLimits,

    /// Ordered semantic operations.
    operations: Vec<Operation>,

    /// Circuit metadata.
    metadata: CircuitMetadata,

    /// Next circuit-local operation identity.
    ///
    /// This is not a global allocator. It only provides convenient IDs for
    /// operations created through this circuit.
    next_operation_id: u64,
}

impl QuantumCircuit {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates an empty circuit using the production IR policy.
    pub fn new(
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> CircuitResult<Self> {
        Self::with_limits(
            num_qubits,
            num_classical_bits,
            QuantumIrLimits::production(),
        )
    }

    /// Creates an empty circuit using an explicit resource policy.
    pub fn with_limits(
        num_qubits: usize,
        num_classical_bits: usize,
        limits: QuantumIrLimits,
    ) -> CircuitResult<Self> {
        limits.validate()?;

        check_qubit_namespace(
            &limits,
            num_qubits,
        )?;

        check_classical_namespace(
            &limits,
            num_classical_bits,
        )?;

        Ok(Self {
            id: CircuitId::new(0),
            version: IrVersion::CURRENT,
            num_qubits,
            num_classical_bits,
            limits,
            operations: Vec::new(),
            metadata: CircuitMetadata::default(),
            next_operation_id: 0,
        })
    }

    /// Creates a circuit with explicit identity.
    pub fn with_identity(
        id: CircuitId,
        num_qubits: usize,
        num_classical_bits: usize,
        limits: QuantumIrLimits,
    ) -> CircuitResult<Self> {
        let mut circuit = Self::with_limits(
            num_qubits,
            num_classical_bits,
            limits,
        )?;

        circuit.id = id;

        Ok(circuit)
    }

    /// Creates a circuit from a complete operation sequence.
    ///
    /// The input vector is validated before ownership is transferred into the
    /// circuit.
    pub fn from_operations(
        num_qubits: usize,
        num_classical_bits: usize,
        operations: Vec<Operation>,
    ) -> CircuitResult<Self> {
        Self::from_operations_with_limits(
            num_qubits,
            num_classical_bits,
            operations,
            QuantumIrLimits::production(),
        )
    }

    /// Creates a circuit from operations with an explicit policy.
    pub fn from_operations_with_limits(
        num_qubits: usize,
        num_classical_bits: usize,
        operations: Vec<Operation>,
        limits: QuantumIrLimits,
    ) -> CircuitResult<Self> {
        let mut circuit = Self::with_limits(
            num_qubits,
            num_classical_bits,
            limits,
        )?;

        circuit.replace_operations(operations)?;

        Ok(circuit)
    }

    // =========================================================================
    // Identity and version
    // =========================================================================

    /// Returns the circuit identity.
    #[must_use]
    pub const fn id(&self) -> CircuitId {
        self.id
    }

    /// Returns the IR schema version.
    #[must_use]
    pub const fn version(&self) -> IrVersion {
        self.version
    }

    /// Returns whether the circuit uses the current IR contract.
    #[must_use]
    pub const fn is_current_version(&self) -> bool {
        self.version.is_current()
    }

    /// Sets the circuit IR version after compatibility validation.
    pub fn set_version(
        &mut self,
        version: IrVersion,
    ) -> CircuitResult<()> {
        if !version.is_supported_by_current() {
            return Err(CircuitError::UnsupportedVersion {
                version,
            });
        }

        self.version = version;
        Ok(())
    }

    // =========================================================================
    // Namespace
    // =========================================================================

    /// Returns the logical qubit namespace size.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the logical classical-bit namespace size.
    #[must_use]
    pub const fn num_classical_bits(&self) -> usize {
        self.num_classical_bits
    }

    /// Returns whether a logical qubit belongs to this circuit namespace.
    #[must_use]
    pub const fn contains_qubit(
        &self,
        qubit: QubitId,
    ) -> bool {
        qubit.index() < self.num_qubits
    }

    /// Returns whether a classical bit belongs to this circuit namespace.
    #[must_use]
    pub const fn contains_classical_bit(
        &self,
        bit: usize,
    ) -> bool {
        bit < self.num_classical_bits
    }

    /// Returns the canonical logical qubit identifier at an index.
    ///
    /// This does not allocate a qubit object.
    pub fn qubit(
        &self,
        index: usize,
    ) -> CircuitResult<QubitId> {
        if index >= self.num_qubits {
            return Err(CircuitError::QubitOutOfRange {
                qubit: QubitId::new(index),
                num_qubits: self.num_qubits,
            });
        }

        Ok(QubitId::new(index))
    }

    // =========================================================================
    // Resource policy
    // =========================================================================

    /// Returns the resource policy.
    #[must_use]
    pub const fn limits(&self) -> &QuantumIrLimits {
        &self.limits
    }

    /// Replaces the resource policy after checking the existing circuit.
    ///
    /// The operation sequence is never modified by this method.
    pub fn set_limits(
        &mut self,
        limits: QuantumIrLimits,
    ) -> CircuitResult<()> {
        limits.validate()?;

        check_qubit_namespace(
            &limits,
            self.num_qubits,
        )?;

        check_classical_namespace(
            &limits,
            self.num_classical_bits,
        )?;

        check_operation_namespace(
            &limits,
            self.operations.len(),
        )?;

        self.limits = limits;

        Ok(())
    }

    // =========================================================================
    // Metadata
    // =========================================================================

    /// Returns circuit metadata.
    #[must_use]
    pub const fn metadata(&self) -> &CircuitMetadata {
        &self.metadata
    }

    /// Replaces metadata atomically.
    pub fn set_metadata(
        &mut self,
        metadata: CircuitMetadata,
    ) -> CircuitResult<()> {
        let size = metadata.byte_size()?;

        check_metadata_limit(
            &self.limits,
            size,
        )?;

        self.metadata = metadata;

        Ok(())
    }

    // =========================================================================
    // Operation access
    // =========================================================================

    /// Returns the number of operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Compatibility alias for callers that use `len()`.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the circuit contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns the immutable operation sequence.
    ///
    /// There is intentionally no public unrestricted mutable slice. Mutations
    /// must pass circuit invariants.
    #[must_use]
    pub fn operations(&self) -> &[Operation] {
        self.operations.as_slice()
    }

    /// Returns one operation by index.
    pub fn operation(
        &self,
        index: usize,
    ) -> CircuitResult<&Operation> {
        self.operations.get(index).ok_or(
            CircuitError::OperationOutOfRange {
                index,
                len: self.operations.len(),
            },
        )
    }

    /// Returns the operation with a given identity.
    #[must_use]
    pub fn operation_by_id(
        &self,
        id: OperationId,
    ) -> Option<&Operation> {
        self.operations.iter().find(|operation| operation.id() == id)
    }

    /// Returns the operation position for an identity.
    #[must_use]
    pub fn operation_index(
        &self,
        id: OperationId,
    ) -> Option<usize> {
        self.operations
            .iter()
            .position(|operation| operation.id() == id)
    }

    /// Returns an iterator over operations.
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, Operation> {
        self.operations.iter()
    }

    // =========================================================================
    // Operation identity
    // =========================================================================

    /// Allocates the next circuit-local operation identity.
    ///
    /// This identity is deterministic for a freshly constructed circuit.
    pub fn allocate_operation_id(
        &mut self,
    ) -> CircuitResult<OperationId> {
        let raw = self.next_operation_id;

        let next = raw.checked_add(1).ok_or(
            CircuitError::OperationCountOverflow,
        )?;

        self.next_operation_id = next;

        Ok(OperationId::new(raw))
    }

    /// Returns the next operation identity without consuming it.
    pub fn peek_next_operation_id(
        &self,
    ) -> CircuitResult<OperationId> {
        Ok(OperationId::new(self.next_operation_id))
    }

    fn synchronize_next_operation_id(&mut self) -> CircuitResult<()> {
        let mut next = self.next_operation_id;

        for operation in &self.operations {
            let raw = operation.id().raw();

            if raw >= next {
                next = raw.checked_add(1).ok_or(
                    CircuitError::OperationCountOverflow,
                )?;
            }
        }

        self.next_operation_id = next;

        Ok(())
    }

    // =========================================================================
    // Mutation
    // =========================================================================

    /// Appends one operation after complete circuit-local checks.
    ///
    /// The operation is not inserted until every check succeeds.
    pub fn push(
        &mut self,
        operation: Operation,
    ) -> CircuitResult<OperationId> {
        operation.validate()?;

        self.validate_operation_namespace(&operation)?;

        self.check_operation_capacity(1)?;

        let id = operation.id();

        if self.operation_by_id(id).is_some() {
            return Err(CircuitError::DuplicateOperationId { id });
        }

        self.operations.push(operation);

        self.synchronize_next_operation_id()?;

        Ok(id)
    }

    /// Alias for `push`.
    pub fn add_operation(
        &mut self,
        operation: Operation,
    ) -> CircuitResult<OperationId> {
        self.push(operation)
    }

    /// Appends a gate operation.
    pub fn push_gate(
        &mut self,
        gate: Gate,
    ) -> CircuitResult<OperationId> {
        let id = self.allocate_operation_id()?;

        let operation = Operation::gate(
            id,
            gate,
        )?;

        self.push(operation)
    }

    /// Inserts an operation at an explicit position.
    ///
    /// The existing operation order is preserved relative to all unaffected
    /// operations.
    pub fn insert(
        &mut self,
        index: usize,
        operation: Operation,
    ) -> CircuitResult<OperationId> {
        if index > self.operations.len() {
            return Err(CircuitError::OperationOutOfRange {
                index,
                len: self.operations.len(),
            });
        }

        operation.validate()?;

        self.validate_operation_namespace(&operation)?;

        self.check_operation_capacity(1)?;

        let id = operation.id();

        if self.operation_by_id(id).is_some() {
            return Err(CircuitError::DuplicateOperationId { id });
        }

        self.operations.insert(index, operation);

        self.synchronize_next_operation_id()?;

        Ok(id)
    }

    /// Replaces an operation at a position.
    ///
    /// Replacement preserves the position but requires a unique operation ID.
    pub fn replace(
        &mut self,
        index: usize,
        operation: Operation,
    ) -> CircuitResult<OperationId> {
        let old = self.operation(index)?;

        let old_id = old.id();
        let new_id = operation.id();

        operation.validate()?;
        self.validate_operation_namespace(&operation)?;

        if new_id != old_id
            && self
                .operation_by_id(new_id)
                .is_some()
        {
            return Err(
                CircuitError::DuplicateOperationId {
                    id: new_id,
                },
            );
        }

        self.operations[index] = operation;

        self.synchronize_next_operation_id()?;

        Ok(new_id)
    }

    /// Removes one operation and returns it.
    pub fn remove(
        &mut self,
        index: usize,
    ) -> CircuitResult<Operation> {
        if index >= self.operations.len() {
            return Err(CircuitError::OperationOutOfRange {
                index,
                len: self.operations.len(),
            });
        }

        Ok(self.operations.remove(index))
    }

    /// Removes every operation while retaining circuit namespaces and metadata.
    pub fn clear(&mut self) {
        self.operations.clear();
    }

    /// Replaces the complete operation sequence atomically.
    pub fn replace_operations(
        &mut self,
        operations: Vec<Operation>,
    ) -> CircuitResult<()> {
        check_operation_namespace(
            &self.limits,
            operations.len(),
        )?;

        validate_operation_set(
            self.num_qubits,
            self.num_classical_bits,
            &operations,
        )?;

        self.operations = operations;

        self.synchronize_next_operation_id()?;

        Ok(())
    }

    // =========================================================================
    // Namespace mutation
    // =========================================================================

    /// Changes the logical qubit namespace size.
    ///
    /// Shrinking is permitted only when no existing operation references a
    /// qubit that would become invalid.
    pub fn resize_qubits(
        &mut self,
        new_count: usize,
    ) -> CircuitResult<()> {
        check_qubit_namespace(
            &self.limits,
            new_count,
        )?;

        if new_count < self.num_qubits {
            for operation in &self.operations {
                self.validate_operation_against_qubit_count(
                    operation,
                    new_count,
                )?;
            }
        }

        self.num_qubits = new_count;

        Ok(())
    }

    /// Changes the classical-bit namespace size.
    ///
    /// Shrinking is permitted only when all directly represented classical
    /// references remain valid. Detailed classical expression validation remains
    /// the responsibility of `validation.rs`.
    pub fn resize_classical_bits(
        &mut self,
        new_count: usize,
    ) -> CircuitResult<()> {
        check_classical_namespace(
            &self.limits,
            new_count,
        )?;

        self.num_classical_bits = new_count;

        Ok(())
    }

    // =========================================================================
    // Analysis helpers
    // =========================================================================

    /// Returns the maximum number of directly represented logical qubit
    /// operands in any one operation.
    pub fn max_operands_per_operation(
        &self,
    ) -> CircuitResult<usize> {
        let mut maximum = 0usize;

        for operation in &self.operations {
            let count = operation.qubit_count();

            if count > maximum {
                maximum = count;
            }
        }

        Ok(maximum)
    }

    /// Returns the number of gate operations.
    #[must_use]
    pub fn gate_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| operation.is_gate())
            .count()
    }

    /// Returns the number of measurement operations.
    #[must_use]
    pub fn measurement_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| operation.is_measurement())
            .count()
    }

    /// Returns the number of pulse operations.
    #[must_use]
    pub fn pulse_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| operation.is_pulse())
            .count()
    }

    /// Returns whether at least one pulse-level operation exists.
    #[must_use]
    pub fn contains_pulses(&self) -> bool {
        self.operations.iter().any(Operation::is_pulse)
    }

    /// Returns the number of operations that directly carry logical qubit
    /// operands.
    #[must_use]
    pub fn quantum_operation_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| operation.is_quantum())
            .count()
    }

    /// Computes a conservative sequential circuit depth.
    ///
    /// This method intentionally does not attempt to schedule operations in
    /// parallel. It calculates semantic sequential depth from operation order.
    ///
    /// It uses a sparse map keyed by touched qubits rather than allocating one
    /// depth slot for every declared qubit. Therefore a huge sparse namespace
    /// remains cheap when only a small number of qubits are touched.
    pub fn sequential_depth(&self) -> CircuitResult<usize> {
        let mut last_depth: std::collections::BTreeMap<
            QubitId,
            usize,
        > = std::collections::BTreeMap::new();

        let mut global_depth = 0usize;

        for operation in &self.operations {
            let qubits = operation.qubits();

            if qubits.is_empty() {
                global_depth = global_depth
                    .checked_add(1)
                    .ok_or(
                        CircuitError::ArithmeticOverflow {
                            calculation: "sequential depth",
                        },
                    )?;

                continue;
            }

            let mut start = global_depth;

            for qubit in &qubits {
                if let Some(depth) = last_depth.get(qubit) {
                    if *depth > start {
                        start = *depth;
                    }
                }
            }

            let end = start.checked_add(1).ok_or(
                CircuitError::ArithmeticOverflow {
                    calculation: "sequential depth",
                },
            )?;

            global_depth = global_depth.max(end);

            for qubit in qubits {
                last_depth.insert(qubit, end);
            }
        }

        Ok(global_depth)
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Performs circuit-local validation.
    ///
    /// This is intentionally independent of the full `validation.rs` module so
    /// that `circuit.rs` does not create a dependency cycle.
    ///
    /// The canonical validator may perform additional semantic validation.
    pub fn validate(&self) -> CircuitResult<()> {
        if !self.version.is_supported_by_current() {
            return Err(CircuitError::UnsupportedVersion {
                version: self.version,
            });
        }

        self.limits.validate()?;

        check_qubit_namespace(
            &self.limits,
            self.num_qubits,
        )?;

        check_classical_namespace(
            &self.limits,
            self.num_classical_bits,
        )?;

        check_operation_namespace(
            &self.limits,
            self.operations.len(),
        )?;

        let metadata_size = self.metadata.byte_size()?;

        check_metadata_limit(
            &self.limits,
            metadata_size,
        )?;

        validate_operation_set(
            self.num_qubits,
            self.num_classical_bits,
            &self.operations,
        )?;

        Ok(())
    }

    /// Validates one operation against this circuit namespace.
    fn validate_operation_namespace(
        &self,
        operation: &Operation,
    ) -> CircuitResult<()> {
        self.validate_operation_against_qubit_count(
            operation,
            self.num_qubits,
        )
    }

    fn validate_operation_against_qubit_count(
        &self,
        operation: &Operation,
        qubit_count: usize,
    ) -> CircuitResult<()> {
        for qubit in operation.qubits() {
            if qubit.index() >= qubit_count {
                return Err(CircuitError::QubitOutOfRange {
                    qubit,
                    num_qubits: qubit_count,
                });
            }
        }

        if let Some(condition) = operation.condition() {
            let bit = condition.bit().index();

            if bit >= self.num_classical_bits {
                return Err(
                    CircuitError::ClassicalBitOutOfRange {
                        bit,
                        num_classical_bits:
                            self.num_classical_bits,
                    },
                );
            }
        }

        Ok(())
    }

    fn check_operation_capacity(
        &self,
        additional: usize,
    ) -> CircuitResult<()> {
        let requested = self
            .operations
            .len()
            .checked_add(additional)
            .ok_or(
                CircuitError::OperationCountOverflow,
            )?;

        check_operation_namespace(
            &self.limits,
            requested,
        )
    }
}

// =============================================================================
// IntoIterator
// =============================================================================

impl<'a> IntoIterator for &'a QuantumCircuit {
    type Item = &'a Operation;
    type IntoIter = std::slice::Iter<'a, Operation>;

    fn into_iter(self) -> Self::IntoIter {
        self.operations.iter()
    }
}

// =============================================================================
// Helper functions
// =============================================================================

fn check_qubit_namespace(
    limits: &QuantumIrLimits,
    count: usize,
) -> CircuitResult<()> {
    limits
        .check_qubits(count)
        .map_err(CircuitError::from)
}

fn check_classical_namespace(
    limits: &QuantumIrLimits,
    count: usize,
) -> CircuitResult<()> {
    limits
        .check_classical_bits(count)
        .map_err(CircuitError::from)
}

fn check_operation_namespace(
    limits: &QuantumIrLimits,
    count: usize,
) -> CircuitResult<()> {
    limits
        .check_operations(count)
        .map_err(CircuitError::from)
}

fn check_metadata_limit(
    limits: &QuantumIrLimits,
    bytes: usize,
) -> CircuitResult<()> {
    limits
        .check_metadata_bytes(bytes)
        .map_err(CircuitError::from)
}

fn validate_operation_set(
    num_qubits: usize,
    num_classical_bits: usize,
    operations: &[Operation],
) -> CircuitResult<()> {
    let mut ids = BTreeSet::<OperationId>::new();

    for operation in operations {
        operation.validate()?;

        if !ids.insert(operation.id()) {
            return Err(
                CircuitError::DuplicateOperationId {
                    id: operation.id(),
                },
            );
        }

        for qubit in operation.qubits() {
            if qubit.index() >= num_qubits {
                return Err(
                    CircuitError::QubitOutOfRange {
                        qubit,
                        num_qubits,
                    },
                );
            }
        }

        if let Some(condition) = operation.condition() {
            let bit = condition.bit().index();

            if bit >= num_classical_bits {
                return Err(
                    CircuitError::ClassicalBitOutOfRange {
                        bit,
                        num_classical_bits,
                    },
                );
            }
        }

        validate_operation_body_namespace(
            operation,
            num_qubits,
        )?;
    }

    Ok(())
}

fn validate_operation_body_namespace(
    operation: &Operation,
    num_qubits: usize,
) -> CircuitResult<()> {
    match operation.body() {
        OperationBody::Gate(gate) => {
            for qubit in gate.qubits() {
                if qubit.index() >= num_qubits {
                    return Err(
                        CircuitError::QubitOutOfRange {
                            qubit,
                            num_qubits,
                        },
                    );
                }
            }
        }

        OperationBody::Reset { qubit } => {
            if qubit.index() >= num_qubits {
                return Err(
                    CircuitError::QubitOutOfRange {
                        qubit: *qubit,
                        num_qubits,
                    },
                );
            }
        }

        OperationBody::Barrier { qubits }
        | OperationBody::AllocateQubits { qubits }
        | OperationBody::ReleaseQubits { qubits } => {
            for qubit in qubits {
                if qubit.index() >= num_qubits {
                    return Err(
                        CircuitError::QubitOutOfRange {
                            qubit: *qubit,
                            num_qubits,
                        },
                    );
                }
            }
        }

        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::operation::OperationBody;

    fn limits() -> QuantumIrLimits {
        QuantumIrLimits::production()
    }

    #[test]
    fn empty_circuit_is_valid() {
        let circuit = QuantumCircuit::new(
            2,
            2,
        )
        .expect("valid circuit");

        assert_eq!(circuit.num_qubits(), 2);
        assert_eq!(circuit.num_classical_bits(), 2);
        assert!(circuit.is_empty());
        assert!(circuit.validate().is_ok());
    }

    #[test]
    fn uses_canonical_qubit_namespace() {
        let circuit = QuantumCircuit::new(
            8,
            0,
        )
        .expect("valid circuit");

        assert!(circuit.contains_qubit(QubitId::new(7)));
        assert!(!circuit.contains_qubit(QubitId::new(8)));
    }

    #[test]
    fn sparse_large_namespace_does_not_materialize_qubits() {
        let circuit = QuantumCircuit::with_limits(
            1_000_000,
            0,
            limits(),
        );

        assert!(circuit.is_ok());

        let circuit = circuit.expect("large namespace should be representable");

        assert_eq!(
            circuit.num_qubits(),
            1_000_000
        );
        assert!(circuit.is_empty());
    }

    #[test]
    fn operation_order_is_preserved() {
        let mut circuit = QuantumCircuit::new(
            2,
            0,
        )
        .expect("valid circuit");

        let first = circuit
            .allocate_operation_id()
            .expect("operation id");

        let second = circuit
            .allocate_operation_id()
            .expect("operation id");

        let gate1 = Gate::new(
            super::super::gate::GateKind::X,
            vec![QubitId::new(0)],
            Vec::new(),
        )
        .expect("valid gate");

        let gate2 = Gate::new(
            super::super::gate::GateKind::H,
            vec![QubitId::new(1)],
            Vec::new(),
        )
        .expect("valid gate");

        circuit
            .push(
                Operation::gate(first, gate1)
                    .expect("operation"),
            )
            .expect("push");

        circuit
            .push(
                Operation::gate(second, gate2)
                    .expect("operation"),
            )
            .expect("push");

        assert_eq!(
            circuit.operation_count(),
            2
        );

        assert_eq!(
            circuit
                .operation(0)
                .expect("operation")
                .id(),
            first
        );

        assert_eq!(
            circuit
                .operation(1)
                .expect("operation")
                .id(),
            second
        );
    }

    #[test]
    fn duplicate_operation_ids_are_rejected() {
        let mut circuit = QuantumCircuit::new(
            1,
            0,
        )
        .expect("valid circuit");

        let id = OperationId::new(42);

        let gate = Gate::new(
            super::super::gate::GateKind::X,
            vec![QubitId::new(0)],
            Vec::new(),
        )
        .expect("valid gate");

        let operation = Operation::gate(
            id,
            gate,
        )
        .expect("operation");

        circuit
            .push(operation.clone())
            .expect("first insertion");

        let result = circuit.push(operation);

        assert!(matches!(
            result,
            Err(CircuitError::DuplicateOperationId {
                ..
            })
        ));
    }

    #[test]
    fn out_of_range_qubit_is_rejected() {
        let mut circuit = QuantumCircuit::new(
            1,
            0,
        )
        .expect("valid circuit");

        let id = circuit
            .allocate_operation_id()
            .expect("operation id");

        let gate = Gate::new(
            super::super::gate::GateKind::X,
            vec![QubitId::new(1)],
            Vec::new(),
        )
        .expect("gate construction itself is namespace-independent");

        let operation = Operation::gate(
            id,
            gate,
        )
        .expect("operation");

        let result = circuit.push(operation);

        assert!(matches!(
            result,
            Err(CircuitError::QubitOutOfRange {
                ..
            })
        ));
    }

    #[test]
    fn clear_preserves_namespace() {
        let mut circuit = QuantumCircuit::new(
            32,
            8,
        )
        .expect("valid circuit");

        circuit.clear();

        assert_eq!(
            circuit.num_qubits(),
            32
        );
        assert_eq!(
            circuit.num_classical_bits(),
            8
        );
        assert!(circuit.is_empty());
    }

    #[test]
    fn removing_an_operation_does_not_change_other_identities() {
        let mut circuit = QuantumCircuit::new(
            2,
            0,
        )
        .expect("valid circuit");

        let first_id = circuit
            .allocate_operation_id()
            .expect("id");

        let second_id = circuit
            .allocate_operation_id()
            .expect("id");

        let first_gate = Gate::new(
            super::super::gate::GateKind::X,
            vec![QubitId::new(0)],
            Vec::new(),
        )
        .expect("gate");

        let second_gate = Gate::new(
            super::super::gate::GateKind::Z,
            vec![QubitId::new(1)],
            Vec::new(),
        )
        .expect("gate");

        circuit
            .push(
                Operation::gate(
                    first_id,
                    first_gate,
                )
                .expect("operation"),
            )
            .expect("push");

        circuit
            .push(
                Operation::gate(
                    second_id,
                    second_gate,
                )
                .expect("operation"),
            )
            .expect("push");

        let removed = circuit
            .remove(0)
            .expect("remove");

        assert_eq!(
            removed.id(),
            first_id
        );

        assert_eq!(
            circuit
                .operation(0)
                .expect("remaining operation")
                .id(),
            second_id
        );
    }

    #[test]
    fn namespace_resize_cannot_invalidate_existing_operations() {
        let mut circuit = QuantumCircuit::new(
            2,
            0,
        )
        .expect("valid circuit");

        let id = circuit
            .allocate_operation_id()
            .expect("id");

        let gate = Gate::new(
            super::super::gate::GateKind::X,
            vec![QubitId::new(1)],
            Vec::new(),
        )
        .expect("gate");

        circuit
            .push(
                Operation::gate(id, gate)
                    .expect("operation"),
            )
            .expect("push");

        let result = circuit.resize_qubits(1);

        assert!(result.is_err());

        assert_eq!(
            circuit.num_qubits(),
            2
        );
    }

    #[test]
    fn operation_with_no_qubits_can_exist() {
        let mut circuit = QuantumCircuit::new(
            0,
            0,
        )
        .expect("valid circuit");

        let id = circuit
            .allocate_operation_id()
            .expect("id");

        let operation = Operation::new(
            id,
            OperationBody::Capability {
                capability: super::super::identity::CapabilityId::new(1),
            },
        )
        .expect("operation");

        circuit
            .push(operation)
            .expect("push");

        assert_eq!(
            circuit.operation_count(),
            1
        );
    }
}