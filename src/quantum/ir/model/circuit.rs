//! Zamani Quantum IR — Canonical Circuit Model
//!
//! This module defines the canonical hardware-independent circuit model used
//! by the Zamani Quantum IR.
//!
//! # Architectural role
//!
//! `QuantumCircuit` is a semantic circuit container. It describes a quantum
//! computation as an ordered sequence of canonical `Operation` values over
//! logical quantum and classical namespaces.
//!
//! It deliberately does NOT implement:
//!
//! - hardware allocation;
//! - physical topology;
//! - logical-to-physical routing;
//! - scheduling;
//! - calibration;
//! - pulse synthesis;
//! - backend execution;
//! - simulation;
//! - optimization;
//! - QEC decoding;
//! - frontend parsing;
//! - vendor APIs.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Canonical architecture
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! canonical Quantum IR
//!      │
//!      ├── model::circuit::QuantumCircuit
//!      │
//!      ├── optimization
//!      │
//!      ├── routing
//!      │
//!      ├── scheduling
//!      │
//!      ├── hardware compatibility
//!      │
//!      └── backend lowering
//!      │
//!      ▼
//! execution
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may target different quantum
//! architectures and different machine sizes.
//!
//! Therefore this type has no architectural maximum such as:
//!
//! ```text
//! 32 qubits
//! 64 qubits
//! 128 qubits
//! 4096 qubits
//! ```
//!
//! A circuit may describe any finite logical namespace permitted by the
//! selected `QuantumIrLimits` policy and the resources available to the
//! compilation process.
//!
//! `QuantumIrLimits` is a resource/security policy, not the definition of
//! what Zamani can express.
//!
//! # Logical versus physical resources
//!
//! This circuit owns logical qubit identities:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! It does not assign physical qubits.
//!
//! Physical placement belongs to routing/mapping and hardware integration.
//!
//! # Operation ownership
//!
//! `Operation` owns the semantics of individual instructions.
//!
//! `QuantumCircuit` owns:
//!
//! - operation ordering;
//! - logical namespace declaration;
//! - classical namespace declaration;
//! - circuit identity;
//! - IR version;
//! - explicit resource policy;
//! - circuit-level metadata;
//! - mutation atomicity;
//! - circuit-wide structural invariants.
//!
//! # Mutation safety
//!
//! All mutating operations follow:
//!
//! ```text
//! candidate
//!    │
//!    ▼
//! local operation validation
//!    │
//!    ▼
//! namespace validation
//!    │
//!    ▼
//! identity validation
//!    │
//!    ▼
//! resource-policy validation
//!    │
//!    ▼
//! commit
//! ```
//!
//! A failed mutation does not partially modify the circuit.
//!
//! # Determinism
//!
//! Explicit operation order is preserved.
//!
//! Operation identity is independent from sequence position.
//!
//! The circuit never uses unordered collections for semantic operation
//! ordering.
//!
//! # Scalability
//!
//! Construction does not allocate one object per declared qubit.
//!
//! A circuit declaring a very large logical namespace but containing only a
//! small number of operations stores only the actual operations and metadata.
//!
//! This keeps representation proportional to semantic content rather than
//! merely to declared namespace size.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contracts
//!
//! `crate::quantum::ir::qubit`
//!     Owns canonical `QubitId`.
//!
//! `crate::quantum::ir::operation`
//!     Owns canonical `Operation` and `OperationSequence`.
//!
//! `crate::quantum::ir::identity`
//!     Owns `CircuitId`, `OperationId`, and `IrVersion`.
//!
//! `crate::quantum::ir::limits`
//!     Owns explicit resource policy.
//!
//! `crate::quantum::ir::validation`
//!     Performs complete IR-wide validation beyond circuit-local invariants.
//!
//! `crate::quantum::ir::analysis`
//!     Performs advanced read-only analysis.
//!
//! `crate::quantum::ir::serialization`
//!     Owns canonical serialization.
//!
//! `crate::quantum::ir::hash`
//!     Owns canonical content hashing.
//!
//! `crate::quantum::ir::provenance`
//!     Owns transformation lineage.
//!
//! `crate::quantum::optimization`
//!     May transform this model but must preserve the semantic contract.
//!
//! `crate::quantum::routing`
//!     May map logical resources to physical resources.
//!
//! `crate::quantum::scheduling`
//!     May derive execution timing.
//!
//! `crate::quantum::hardware`
//!     Describes actual target capabilities and resources.
//!
//! # Important repository naming rule
//!
//! The canonical qubit module is:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! Never use `quantum::ir::qubits` for new code.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::identity::{CircuitId, IrVersion, OperationId};
use crate::quantum::ir::limits::{LimitsError, QuantumIrLimits};
use crate::quantum::ir::operation::{Operation, OperationError, OperationSequence};
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type returned by circuit construction and mutation operations.
pub type CircuitResult<T> = Result<T, CircuitError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the canonical circuit model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitError {
    /// The supplied IR version is not supported by this implementation.
    UnsupportedVersion {
        /// Requested IR version.
        version: IrVersion,
    },

    /// The supplied resource policy is invalid.
    InvalidLimits {
        /// Name of the invalid policy field.
        field: &'static str,

        /// Invalid configured value.
        value: usize,
    },

    /// The logical-qubit namespace exceeds its policy.
    QubitLimitExceeded {
        /// Requested logical-qubit count.
        requested: usize,

        /// Maximum permitted by policy.
        maximum: usize,
    },

    /// The classical namespace exceeds its policy.
    ClassicalBitLimitExceeded {
        /// Requested classical-bit count.
        requested: usize,

        /// Maximum permitted by policy.
        maximum: usize,
    },

    /// The operation sequence exceeds its policy.
    OperationLimitExceeded {
        /// Requested operation count.
        requested: usize,

        /// Maximum permitted by policy.
        maximum: usize,
    },

    /// Operation-count arithmetic overflowed.
    OperationCountOverflow,

    /// A logical qubit referenced by an operation is outside this circuit's
    /// logical namespace.
    QubitOutOfRange {
        /// Referenced logical qubit.
        qubit: QubitId,

        /// Number of logical qubits declared by the circuit.
        num_qubits: usize,
    },

    /// An operation identity already exists in the circuit.
    DuplicateOperationId {
        /// Duplicated identity.
        id: OperationId,
    },

    /// An operation sequence index is invalid.
    OperationOutOfRange {
        /// Requested index.
        index: usize,

        /// Number of operations currently present.
        len: usize,
    },

    /// The operation itself violates its local invariants.
    InvalidOperation(OperationError),

    /// A circuit-wide invariant failed.
    InvalidCircuit {
        /// Stable description of the violated invariant.
        message: &'static str,
    },

    /// Metadata would exceed the configured metadata policy.
    MetadataLimitExceeded {
        /// Requested metadata size.
        requested: usize,

        /// Maximum permitted metadata size.
        maximum: usize,
    },

    /// Metadata input is invalid.
    InvalidMetadata {
        /// Stable validation reason.
        message: &'static str,
    },

    /// Arithmetic overflow occurred while calculating circuit statistics.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// A complete circuit validation failed.
    Validation(String),
}

impl fmt::Display for CircuitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion { version } => {
                write!(
                    formatter,
                    "unsupported Zamani Quantum IR version {version}"
                )
            }

            Self::InvalidLimits { field, value } => {
                write!(
                    formatter,
                    "invalid quantum IR limit `{field}`: {value}"
                )
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "logical qubit limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::ClassicalBitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "classical-bit limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "operation limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::OperationCountOverflow => {
                formatter.write_str("operation count overflow")
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is outside circuit namespace 0..{num_qubits}"
                )
            }

            Self::DuplicateOperationId { id } => {
                write!(
                    formatter,
                    "operation identity {id} already exists in circuit"
                )
            }

            Self::OperationOutOfRange { index, len } => {
                write!(
                    formatter,
                    "operation index {index} is outside circuit length {len}"
                )
            }

            Self::InvalidOperation(error) => {
                write!(formatter, "invalid operation: {error}")
            }

            Self::InvalidCircuit { message } => {
                write!(formatter, "invalid circuit: {message}")
            }

            Self::MetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "circuit metadata limit exceeded: requested {requested} bytes, maximum {maximum}"
                )
            }

            Self::InvalidMetadata { message } => {
                write!(
                    formatter,
                    "invalid circuit metadata: {message}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::Validation(message) => {
                write!(
                    formatter,
                    "quantum circuit validation failed: {message}"
                )
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
                match resource.as_str() {
                    "logical qubits" => {
                        Self::QubitLimitExceeded {
                            requested,
                            maximum,
                        }
                    }

                    "classical bits" => {
                        Self::ClassicalBitLimitExceeded {
                            requested,
                            maximum,
                        }
                    }

                    "operations" => {
                        Self::OperationLimitExceeded {
                            requested,
                            maximum,
                        }
                    }

                    _ => {
                        Self::InvalidCircuit {
                            message:
                                "circuit resource policy rejected the requested resource",
                        }
                    }
                }
            }

            LimitsError::ArithmeticOverflow { .. }
            | LimitsError::ArithmeticMultiplicationOverflow { .. }
            | LimitsError::TimeArithmeticOverflow => {
                Self::ArithmeticOverflow {
                    calculation: "quantum IR resource accounting",
                }
            }

            LimitsError::ScheduleTimeExceeded { .. } => {
                Self::InvalidCircuit {
                    message:
                        "schedule-time limits belong to scheduling, not circuit construction",
                }
            }
        }
    }
}

// =============================================================================
// Circuit metadata
// =============================================================================

/// Deterministic circuit-level metadata.
///
/// Metadata is deliberately small and semantic. It does not contain hardware
/// topology, calibration, credentials, backend state, physical allocation, or
/// execution results.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CircuitMetadata {
    name: Option<String>,
    source: Option<String>,
    compiler_version: Option<String>,
    fault_tolerant: bool,
}

impl CircuitMetadata {
    /// Creates empty metadata.
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

    /// Returns the optional compiler-version string.
    #[must_use]
    pub fn compiler_version(&self) -> Option<&str> {
        self.compiler_version.as_deref()
    }

    /// Returns whether the circuit is explicitly marked as fault tolerant.
    #[must_use]
    pub const fn is_fault_tolerant(&self) -> bool {
        self.fault_tolerant
    }

    /// Sets the circuit name.
    ///
    /// The circuit owner performs the metadata-size policy check before
    /// committing this value.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Sets the source identifier.
    pub fn set_source(&mut self, source: Option<String>) {
        self.source = source;
    }

    /// Sets the compiler version.
    pub fn set_compiler_version(
        &mut self,
        compiler_version: Option<String>,
    ) {
        self.compiler_version = compiler_version;
    }

    /// Marks or unmarks the circuit as fault tolerant.
    pub const fn set_fault_tolerant(
        &mut self,
        fault_tolerant: bool,
    ) {
        self.fault_tolerant = fault_tolerant;
    }

    /// Returns deterministic metadata storage size in bytes.
    ///
    /// This is an accounting measure for the explicit metadata strings. It
    /// does not include allocator overhead.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        let mut total = 0usize;

        if let Some(value) = self.name.as_deref() {
            total = total.saturating_add(value.len());
        }

        if let Some(value) = self.source.as_deref() {
            total = total.saturating_add(value.len());
        }

        if let Some(value) = self.compiler_version.as_deref() {
            total = total.saturating_add(value.len());
        }

        total
    }

    /// Validates metadata-local invariants.
    pub fn validate(&self) -> CircuitResult<()> {
        if self
            .name
            .as_deref()
            .is_some_and(|value| value.is_empty())
        {
            return Err(CircuitError::InvalidMetadata {
                message: "circuit name cannot be an empty string",
            });
        }

        if self
            .source
            .as_deref()
            .is_some_and(|value| value.is_empty())
        {
            return Err(CircuitError::InvalidMetadata {
                message: "source identifier cannot be an empty string",
            });
        }

        if self
            .compiler_version
            .as_deref()
            .is_some_and(|value| value.is_empty())
        {
            return Err(CircuitError::InvalidMetadata {
                message: "compiler version cannot be an empty string",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Circuit statistics
// =============================================================================

/// Deterministic, read-only circuit statistics.
///
/// These statistics are intentionally semantic and target-independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CircuitStatistics {
    /// Number of logical qubits declared by the circuit.
    pub logical_qubits: usize,

    /// Number of declared classical bits.
    pub classical_bits: usize,

    /// Number of operations.
    pub operations: usize,

    /// Number of distinct logical qubits referenced by operations.
    pub referenced_qubits: usize,

    /// Maximum referenced logical-qubit index plus one.
    ///
    /// This is `None` when no operation references a logical qubit.
    pub referenced_qubit_span: Option<usize>,
}

// =============================================================================
// Quantum circuit
// =============================================================================

/// Canonical hardware-independent quantum circuit.
///
/// `QuantumCircuit` is a specialized straight-line quantum program model.
/// Higher-level dynamic programs, functions, regions and control flow belong
/// to the larger program/control-flow IR.
///
/// The circuit contains no physical hardware state and no execution state.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCircuit {
    id: CircuitId,
    version: IrVersion,

    /// Number of logical qubits in the circuit namespace.
    num_qubits: usize,

    /// Number of classical bits in the circuit namespace.
    num_classical_bits: usize,

    /// Ordered canonical operations.
    operations: OperationSequence,

    /// Explicit resource/security policy.
    limits: QuantumIrLimits,

    /// Circuit-level semantic metadata.
    metadata: CircuitMetadata,

    /// Next operation identity issued by this circuit-local allocator.
    ///
    /// This is NOT a global allocator and does not determine semantic
    /// operation identity outside this circuit.
    next_operation_id: u64,
}

impl QuantumCircuit {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates an empty circuit using the production IR policy.
    ///
    /// For externally supplied or untrusted counts, prefer
    /// [`Self::try_new_with_limits`].
    pub fn new(
        id: CircuitId,
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> CircuitResult<Self> {
        Self::try_new_with_limits(
            id,
            num_qubits,
            num_classical_bits,
            QuantumIrLimits::production(),
        )
    }

    /// Creates an empty circuit with an explicit resource policy.
    ///
    /// No allocation proportional to `num_qubits` or
    /// `num_classical_bits` is performed.
    pub fn try_new_with_limits(
        id: CircuitId,
        num_qubits: usize,
        num_classical_bits: usize,
        limits: QuantumIrLimits,
    ) -> CircuitResult<Self> {
        if !IrVersion::CURRENT.is_supported_by_current() {
            return Err(CircuitError::UnsupportedVersion {
                version: IrVersion::CURRENT,
            });
        }

        limits.validate()?;

        limits.check_qubits(num_qubits)?;
        limits.check_classical_bits(num_classical_bits)?;

        let circuit = Self {
            id,
            version: IrVersion::CURRENT,
            num_qubits,
            num_classical_bits,
            operations: OperationSequence::new(),
            limits,
            metadata: CircuitMetadata::new(),
            next_operation_id: 0,
        };

        circuit.validate()?;

        Ok(circuit)
    }

    /// Creates an empty circuit with an explicitly selected IR version.
    ///
    /// The version must be supported by this implementation.
    pub fn try_new_versioned(
        id: CircuitId,
        version: IrVersion,
        num_qubits: usize,
        num_classical_bits: usize,
        limits: QuantumIrLimits,
    ) -> CircuitResult<Self> {
        if !version.is_supported_by_current() {
            return Err(CircuitError::UnsupportedVersion { version });
        }

        limits.validate()?;

        limits.check_qubits(num_qubits)?;
        limits.check_classical_bits(num_classical_bits)?;

        let circuit = Self {
            id,
            version,
            num_qubits,
            num_classical_bits,
            operations: OperationSequence::new(),
            limits,
            metadata: CircuitMetadata::new(),
            next_operation_id: 0,
        };

        circuit.validate()?;

        Ok(circuit)
    }

    // =========================================================================
    // Identity and version
    // =========================================================================

    /// Returns the stable circuit identity.
    #[must_use]
    pub const fn id(&self) -> CircuitId {
        self.id
    }

    /// Returns the IR schema version.
    #[must_use]
    pub const fn version(&self) -> IrVersion {
        self.version
    }

    // =========================================================================
    // Namespace
    // =========================================================================

    /// Returns the declared logical-qubit count.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the declared classical-bit count.
    #[must_use]
    pub const fn num_classical_bits(&self) -> usize {
        self.num_classical_bits
    }

    /// Returns whether the circuit declares no logical qubits.
    #[must_use]
    pub const fn has_no_qubits(&self) -> bool {
        self.num_qubits == 0
    }

    /// Returns whether a logical qubit belongs to this circuit namespace.
    #[must_use]
    pub const fn contains_qubit(
        &self,
        qubit: QubitId,
    ) -> bool {
        qubit.index() < self.num_qubits
    }

    /// Returns whether a classical-bit index belongs to this circuit namespace.
    #[must_use]
    pub const fn contains_classical_bit(
        &self,
        bit: usize,
    ) -> bool {
        bit < self.num_classical_bits
    }

    // =========================================================================
    // Policy
    // =========================================================================

    /// Returns the explicit resource policy.
    #[must_use]
    pub const fn limits(&self) -> &QuantumIrLimits {
        &self.limits
    }

    // =========================================================================
    // Metadata
    // =========================================================================

    /// Returns circuit metadata.
    #[must_use]
    pub const fn metadata(&self) -> &CircuitMetadata {
        &self.metadata
    }

    /// Replaces circuit metadata atomically.
    ///
    /// The candidate metadata is validated and checked against the explicit
    /// metadata policy before being committed.
    pub fn set_metadata(
        &mut self,
        metadata: CircuitMetadata,
    ) -> CircuitResult<()> {
        metadata.validate()?;

        let bytes = metadata.byte_len();

        self.check_metadata_bytes(bytes)?;

        self.metadata = metadata;

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Updates the circuit name atomically.
    pub fn set_name(
        &mut self,
        name: Option<String>,
    ) -> CircuitResult<()> {
        let mut candidate = self.metadata.clone();

        candidate.set_name(name);

        self.set_metadata(candidate)
    }

    /// Updates the source identifier atomically.
    pub fn set_source(
        &mut self,
        source: Option<String>,
    ) -> CircuitResult<()> {
        let mut candidate = self.metadata.clone();

        candidate.set_source(source);

        self.set_metadata(candidate)
    }

    /// Updates the compiler version atomically.
    pub fn set_compiler_version(
        &mut self,
        compiler_version: Option<String>,
    ) -> CircuitResult<()> {
        let mut candidate = self.metadata.clone();

        candidate.set_compiler_version(compiler_version);

        self.set_metadata(candidate)
    }

    /// Marks this circuit as fault tolerant.
    pub fn set_fault_tolerant(
        &mut self,
        value: bool,
    ) -> CircuitResult<()> {
        let mut candidate = self.metadata.clone();

        candidate.set_fault_tolerant(value);

        self.set_metadata(candidate)
    }

    // =========================================================================
    // Operations
    // =========================================================================

    /// Returns the number of operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the circuit contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns an operation by sequence position.
    #[must_use]
    pub fn operation(
        &self,
        index: usize,
    ) -> Option<&Operation> {
        self.operations.get(index)
    }

    /// Returns the ordered operation iterator.
    ///
    /// The iterator does not expose mutable access, preserving circuit
    /// invariants.
    pub fn operations(
        &self,
    ) -> impl ExactSizeIterator<Item = &Operation> + DoubleEndedIterator {
        self.operations.iter()
    }

    /// Returns the first operation, if any.
    #[must_use]
    pub fn first_operation(&self) -> Option<&Operation> {
        self.operations.iter().next()
    }

    /// Returns the last operation, if any.
    #[must_use]
    pub fn last_operation(&self) -> Option<&Operation> {
        self.operations.iter().next_back()
    }

    /// Appends an already constructed operation.
    ///
    /// This is the primary circuit mutation API.
    ///
    /// The candidate is fully checked before the sequence is changed.
    pub fn append_operation(
        &mut self,
        operation: Operation,
    ) -> CircuitResult<()> {
        self.validate_operation_for_circuit(&operation)?;

        self.operations
            .push(operation)
            .map_err(CircuitError::from)?;

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Allocates a circuit-local operation identity without inserting an
    /// operation.
    ///
    /// The identity is stable for the returned operation and independent of
    /// sequence position.
    pub fn allocate_operation_id(
        &mut self,
    ) -> CircuitResult<OperationId> {
        let value = self.next_operation_id;

        let next = value
            .checked_add(1)
            .ok_or(CircuitError::OperationCountOverflow)?;

        self.next_operation_id = next;

        Ok(OperationId::new(value))
    }

    /// Appends an operation using the circuit-local identity allocator.
    ///
    /// The caller supplies the semantic operation body.
    pub fn append_with_new_id(
        &mut self,
        body: crate::quantum::ir::operation::OperationBody,
    ) -> CircuitResult<OperationId> {
        let id = self.allocate_operation_id()?;

        let operation = Operation::new(id, body)?;

        self.append_operation(operation)?;

        Ok(id)
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates the complete circuit-local invariant set.
    ///
    /// This validation is deterministic and side-effect free.
    pub fn validate(&self) -> CircuitResult<()> {
        if !self.version.is_supported_by_current() {
            return Err(CircuitError::UnsupportedVersion {
                version: self.version,
            });
        }

        self.limits.validate()?;

        self.limits.check_qubits(self.num_qubits)?;
        self.limits
            .check_classical_bits(self.num_classical_bits)?;
        self.limits
            .check_operations(self.operations.len())?;

        self.metadata.validate()?;

        self.check_metadata_bytes(self.metadata.byte_len())?;

        self.operations
            .validate()
            .map_err(CircuitError::from)?;

        let mut operation_ids = BTreeSet::new();

        for operation in self.operations.iter() {
            if !operation_ids.insert(operation.id()) {
                return Err(CircuitError::DuplicateOperationId {
                    id: operation.id(),
                });
            }

            self.validate_operation_for_namespace(operation)?;
        }

        self.validate_next_operation_identity(&operation_ids)?;

        Ok(())
    }

    /// Validates the candidate operation without modifying the circuit.
    fn validate_operation_for_circuit(
        &self,
        operation: &Operation,
    ) -> CircuitResult<()> {
        operation.validate()?;

        let requested = self
            .operation_count()
            .checked_add(1)
            .ok_or(CircuitError::OperationCountOverflow)?;

        self.limits.check_operations(requested)?;

        self.validate_operation_for_namespace(operation)?;

        if self
            .operations
            .iter()
            .any(|existing| existing.id() == operation.id())
        {
            return Err(CircuitError::DuplicateOperationId {
                id: operation.id(),
            });
        }

        Ok(())
    }

    /// Validates all logical qubit operands of one operation.
    fn validate_operation_for_namespace(
        &self,
        operation: &Operation,
    ) -> CircuitResult<()> {
        for qubit in operation.qubits() {
            if !self.contains_qubit(qubit) {
                return Err(CircuitError::QubitOutOfRange {
                    qubit,
                    num_qubits: self.num_qubits,
                });
            }
        }

        Ok(())
    }

    /// Validates the next local identity allocator against existing IDs.
    ///
    /// This prevents the local allocator from producing an already-used
    /// operation identity after importing or transforming an operation set.
    fn validate_next_operation_identity(
        &self,
        operation_ids: &BTreeSet<OperationId>,
    ) -> CircuitResult<()> {
        let candidate = OperationId::new(self.next_operation_id);

        if operation_ids.contains(&candidate) {
            return Err(CircuitError::InvalidCircuit {
                message:
                    "next circuit-local operation identity collides with an existing operation",
            });
        }

        Ok(())
    }

    /// Checks the configured metadata resource policy.
    ///
    /// `QuantumIrLimits` owns the actual metadata policy. This helper keeps
    /// the translation boundary in one place.
    fn check_metadata_bytes(
        &self,
        bytes: usize,
    ) -> CircuitResult<()> {
        match self.limits.check_metadata_bytes(bytes) {
            Ok(()) => Ok(()),

            Err(LimitsError::ResourceExceeded {
                requested,
                maximum,
                ..
            }) => {
                Err(CircuitError::MetadataLimitExceeded {
                    requested,
                    maximum,
                })
            }

            Err(LimitsError::InvalidConfiguration {
                field,
                value,
            }) => {
                Err(CircuitError::InvalidLimits {
                    field,
                    value,
                })
            }

            Err(_) => {
                Err(CircuitError::ArithmeticOverflow {
                    calculation: "circuit metadata size",
                })
            }
        }
    }

    // =========================================================================
    // Read-only statistics
    // =========================================================================

    /// Computes deterministic circuit statistics.
    ///
    /// The calculation is read-only and does not alter the circuit.
    pub fn statistics(&self) -> CircuitResult<CircuitStatistics> {
        let mut referenced = BTreeSet::new();

        for operation in self.operations.iter() {
            for qubit in operation.qubits() {
                referenced.insert(qubit);
            }
        }

        let referenced_qubit_span = referenced
            .iter()
            .next_back()
            .and_then(|qubit| {
                qubit
                    .index()
                    .checked_add(1)
            });

        if referenced_qubit_span.is_none() && !referenced.is_empty() {
            return Err(CircuitError::ArithmeticOverflow {
                calculation: "referenced logical-qubit span",
            });
        }

        Ok(CircuitStatistics {
            logical_qubits: self.num_qubits,
            classical_bits: self.num_classical_bits,
            operations: self.operation_count(),
            referenced_qubits: referenced.len(),
            referenced_qubit_span,
        })
    }

    /// Returns the set of logical qubits referenced by the circuit.
    ///
    /// The returned set is deterministic and sorted by `QubitId`.
    pub fn referenced_qubits(
        &self,
    ) -> CircuitResult<BTreeSet<QubitId>> {
        let mut result = BTreeSet::new();

        for operation in self.operations.iter() {
            for qubit in operation.qubits() {
                if !self.contains_qubit(qubit) {
                    return Err(CircuitError::QubitOutOfRange {
                        qubit,
                        num_qubits: self.num_qubits,
                    });
                }

                result.insert(qubit);
            }
        }

        Ok(result)
    }

    // =========================================================================
    // Namespace growth
    // =========================================================================

    /// Expands the logical-qubit namespace.
    ///
    /// Existing operations remain unchanged.
    ///
    /// This operation never allocates one object per new qubit.
    pub fn extend_qubits(
        &mut self,
        additional: usize,
    ) -> CircuitResult<()> {
        let new_count = self
            .num_qubits
            .checked_add(additional)
            .ok_or(CircuitError::ArithmeticOverflow {
                calculation: "logical-qubit namespace growth",
            })?;

        self.limits.check_qubits(new_count)?;

        self.num_qubits = new_count;

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Expands the classical-bit namespace.
    ///
    /// Existing operations remain unchanged.
    pub fn extend_classical_bits(
        &mut self,
        additional: usize,
    ) -> CircuitResult<()> {
        let new_count = self
            .num_classical_bits
            .checked_add(additional)
            .ok_or(CircuitError::ArithmeticOverflow {
                calculation: "classical-bit namespace growth",
            })?;

        self.limits
            .check_classical_bits(new_count)?;

        self.num_classical_bits = new_count;

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    // =========================================================================
    // Safe replacement
    // =========================================================================

    /// Replaces an operation at a sequence position atomically.
    ///
    /// The replacement must preserve all circuit-local invariants.
    ///
    /// This method deliberately does not expose a mutable operation reference.
    pub fn replace_operation(
        &mut self,
        index: usize,
        replacement: Operation,
    ) -> CircuitResult<()> {
        let existing = self
            .operation(index)
            .ok_or(CircuitError::OperationOutOfRange {
                index,
                len: self.operation_count(),
            })?;

        let existing_id = existing.id();

        self.validate_operation_for_namespace(&replacement)?;
        replacement.validate()?;

        if replacement.id() != existing_id
            && self
                .operations
                .iter()
                .any(|operation| operation.id() == replacement.id())
        {
            return Err(CircuitError::DuplicateOperationId {
                id: replacement.id(),
            });
        }

        /*
         * OperationSequence intentionally does not expose mutable internals.
         *
         * Replacement therefore cannot be implemented safely by mutating the
         * sequence in-place unless the sequence itself grows an atomic
         * replacement API.
         *
         * The canonical circuit contract consequently treats direct
         * replacement as unsupported until OperationSequence provides that
         * operation atomically.
         *
         * Returning an explicit error is preferable to exposing an
         * `operations_mut()` escape hatch.
         */
        let _ = replacement;

        Err(CircuitError::InvalidCircuit {
            message:
                "atomic operation replacement requires OperationSequence::replace; mutable sequence access is intentionally not exposed",
        })
    }

    // =========================================================================
    // Structural helpers
    // =========================================================================

    /// Returns whether all referenced logical qubits are within the declared
    /// namespace.
    #[must_use]
    pub fn has_valid_qubit_namespace(&self) -> bool {
        self.operations
            .iter()
            .all(|operation| {
                operation
                    .qubits()
                    .into_iter()
                    .all(|qubit| self.contains_qubit(qubit))
            })
    }

    /// Returns whether operation identities are unique.
    #[must_use]
    pub fn has_unique_operation_ids(&self) -> bool {
        let mut ids = BTreeSet::new();

        self.operations
            .iter()
            .all(|operation| ids.insert(operation.id()))
    }

    /// Returns whether the circuit is valid without allocating a diagnostic
    /// string.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> QuantumIrLimits {
        QuantumIrLimits::production()
            .with_max_qubits(128)
            .with_max_classical_bits(128)
            .with_max_operations(1_000)
    }

    #[test]
    fn empty_circuit_is_valid() {
        let circuit = QuantumCircuit::try_new_with_limits(
            CircuitId::new(1),
            1,
            1,
            limits(),
        )
        .expect("circuit construction must succeed");

        assert!(circuit.is_valid());
        assert_eq!(circuit.operation_count(), 0);
        assert!(circuit.is_empty());
    }

    #[test]
    fn namespace_does_not_allocate_per_qubit() {
        let circuit = QuantumCircuit::try_new_with_limits(
            CircuitId::new(2),
            1_000_000,
            1_000_000,
            QuantumIrLimits::unbounded(),
        )
        .expect("large logical namespaces must be representable");

        assert_eq!(circuit.num_qubits(), 1_000_000);
        assert_eq!(circuit.num_classical_bits(), 1_000_000);
        assert!(circuit.is_empty());
    }

    #[test]
    fn qubit_namespace_is_logical() {
        let circuit = QuantumCircuit::try_new_with_limits(
            CircuitId::new(3),
            64,
            0,
            limits(),
        )
        .expect("construction must succeed");

        assert!(circuit.contains_qubit(QubitId::new(0)));
        assert!(circuit.contains_qubit(QubitId::new(63)));
        assert!(!circuit.contains_qubit(QubitId::new(64)));
    }

    #[test]
    fn namespace_growth_is_policy_checked() {
        let mut circuit =
            QuantumCircuit::try_new_with_limits(
                CircuitId::new(4),
                2,
                0,
                limits(),
            )
            .expect("construction must succeed");

        circuit
            .extend_qubits(126)
            .expect("growth to policy maximum must succeed");

        assert_eq!(circuit.num_qubits(), 128);

        assert!(
            circuit.extend_qubits(1).is_err()
        );
    }

    #[test]
    fn metadata_is_atomic() {
        let mut circuit =
            QuantumCircuit::try_new_with_limits(
                CircuitId::new(5),
                1,
                0,
                limits(),
            )
            .expect("construction must succeed");

        circuit
            .set_name(Some("bell".to_owned()))
            .expect("valid metadata must succeed");

        assert_eq!(
            circuit.metadata().name(),
            Some("bell")
        );
    }

    #[test]
    fn statistics_are_deterministic() {
        let circuit =
            QuantumCircuit::try_new_with_limits(
                CircuitId::new(6),
                8,
                8,
                limits(),
            )
            .expect("construction must succeed");

        let statistics = circuit
            .statistics()
            .expect("statistics must succeed");

        assert_eq!(statistics.logical_qubits, 8);
        assert_eq!(statistics.classical_bits, 8);
        assert_eq!(statistics.operations, 0);
        assert_eq!(statistics.referenced_qubits, 0);
        assert_eq!(
            statistics.referenced_qubit_span,
            None
        );
    }

    #[test]
    fn no_mutable_operation_escape_hatch_exists() {
        let circuit =
            QuantumCircuit::try_new_with_limits(
                CircuitId::new(7),
                4,
                4,
                limits(),
            )
            .expect("construction must succeed");

        let _operations = circuit.operations();

        // Deliberately no `operations_mut()` API exists.
        assert!(circuit.is_valid());
    }

    #[test]
    fn large_namespace_is_not_a_hardware_limit() {
        let circuit =
            QuantumCircuit::try_new_with_limits(
                CircuitId::new(8),
                1_000_000,
                1_000_000,
                QuantumIrLimits::unbounded(),
            )
            .expect("large finite namespace must be accepted");

        assert_eq!(
            circuit.num_qubits(),
            1_000_000
        );
    }
}