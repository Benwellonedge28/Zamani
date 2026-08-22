//! Zamani Quantum Intermediate Representation — Circuit
//!
//! Canonical, hardware-independent container for a logical quantum program.
//!
//! # Architectural boundary
//!
//! `QuantumCircuit` owns:
//!
//! - logical qubit namespace size;
//! - logical classical-bit namespace size;
//! - ordered logical quantum operations;
//! - circuit metadata;
//! - IR schema version;
//! - circuit identity;
//! - the resource policy used by the circuit;
//! - safe, atomic mutation boundaries.
//!
//! It deliberately does NOT own:
//!
//! - physical qubit topology;
//! - logical-to-physical routing;
//! - pulse schedules;
//! - calibration;
//! - backend-specific gate decomposition;
//! - QPU communication;
//! - hardware execution;
//! - error-correction decoding;
//! - optimization algorithms;
//! - frontend parsing.
//!
//! Those concerns belong to downstream quantum compiler/backend stages.
//!
//! # Invariants
//!
//! A `QuantumCircuit` created through the public API guarantees:
//!
//! - logical qubit count is within its configured IR limit;
//! - classical-bit count is within its configured IR limit;
//! - operation count is within its configured IR limit;
//! - metadata is within its configured byte limit;
//! - every inserted operation is locally valid;
//! - every inserted operation references valid logical namespaces;
//! - failed mutation never partially modifies the circuit;
//! - callers cannot obtain an unrestricted mutable operation slice;
//! - callers cannot mutate metadata fields without validation;
//! - circuit depth calculations are overflow-safe;
//! - the circuit has an explicit IR version;
//! - the circuit has an explicit circuit identity;
//! - whole-circuit validation is delegated to the canonical validator.
//!
//! # Untrusted IR
//!
//! A circuit can eventually be reconstructed from external sources such as:
//!
//! - deserialization;
//! - replay;
//! - frontend lowering;
//! - generated IR;
//! - optimizer output;
//! - external tools.
//!
//! Therefore `validate()` MUST remain available even though public constructors
//! and mutation methods already enforce local invariants.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No nightly features or external dependencies are required.

use std::fmt;

use super::errors::{
    IrCircuitError,
    IrError,
    IrIdentifierError,
    IrLimitError,
    IrResult,
};
use super::gate::{Gate, GateError};
use super::identity::{
    CircuitId,
    IrVersion,
};
use super::limits::{
    LimitsError,
    QuantumIrLimits,
};
use super::validation::{
    validate_circuit_with_config,
    ValidationConfig,
};
use super::qubits::QubitId;

// -----------------------------------------------------------------------------
// Circuit errors
// -----------------------------------------------------------------------------

/// Errors produced while constructing, validating, or modifying a circuit.
///
/// This is the circuit-local error vocabulary. Public subsystem boundaries can
/// convert it into the canonical [`IrError`] using `From<CircuitError>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitError {
    /// The configured circuit resource policy is invalid.
    InvalidLimits {
        /// The offending limit field.
        field: &'static str,

        /// The invalid value.
        value: usize,
    },

    /// The requested logical qubit count exceeds the circuit's resource policy.
    QubitLimitExceeded {
        /// Requested number of logical qubits.
        requested: usize,

        /// Maximum permitted number.
        maximum: usize,
    },

    /// The requested classical-bit count exceeds the circuit's resource policy.
    ClassicalBitLimitExceeded {
        /// Requested number of classical bits.
        requested: usize,

        /// Maximum permitted number.
        maximum: usize,
    },

    /// The requested operation count exceeds the circuit's resource policy.
    OperationLimitExceeded {
        /// Requested number of operations.
        requested: usize,

        /// Maximum permitted number.
        maximum: usize,
    },

    /// Circuit metadata exceeds its configured byte limit.
    MetadataLimitExceeded {
        /// Requested metadata size in bytes.
        requested: usize,

        /// Maximum permitted bytes.
        maximum: usize,
    },

    /// A logical qubit is outside the circuit's namespace.
    QubitOutOfRange {
        /// Invalid logical qubit.
        qubit: QubitId,

        /// Number of logical qubits in the circuit.
        num_qubits: usize,
    },

    /// A classical bit is outside the circuit's namespace.
    ClassicalBitOutOfRange {
        /// Invalid classical-bit index.
        bit: usize,

        /// Number of classical bits in the circuit.
        num_classical_bits: usize,
    },

    /// A gate has no operands where operands are required.
    MissingOperands,

    /// A gate contains a duplicate logical qubit.
    DuplicateQubit {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// A gate failed its local validation.
    InvalidGate {
        /// Original gate error.
        error: GateError,
    },

    /// An operation index is invalid.
    OperationOutOfRange {
        /// Requested index.
        index: usize,

        /// Current operation count.
        len: usize,
    },

    /// Circuit metadata is structurally invalid.
    InvalidMetadata {
        /// Static reason.
        message: &'static str,
    },

    /// Circuit version is not supported by the current IR implementation.
    UnsupportedVersion {
        /// Version supplied to the circuit.
        version: IrVersion,
    },

    /// The circuit contains an invalid internal combination.
    InvalidCircuit {
        /// Static reason.
        message: &'static str,
    },

    /// An arithmetic operation required by circuit analysis overflowed.
    ArithmeticOverflow {
        /// Name of the calculation.
        calculation: &'static str,
    },

    /// Canonical IR validation failed.
    Validation(IrError),
}

impl fmt::Display for CircuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimits { field, value } => {
                write!(
                    f,
                    "invalid quantum IR limit `{field}`: value {value}"
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
                    "logical qubit {qubit} is outside range 0..{num_qubits}"
                )
            }

            Self::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => {
                write!(
                    f,
                    "classical bit {bit} is outside range 0..{num_classical_bits}"
                )
            }

            Self::MissingOperands => {
                f.write_str("gate has no operands")
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "logical qubit {qubit} appears more than once"
                )
            }

            Self::InvalidGate { error } => {
                write!(f, "invalid gate: {error}")
            }

            Self::OperationOutOfRange { index, len } => {
                write!(
                    f,
                    "operation index {index} is outside circuit length {len}"
                )
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

            Self::Validation(error) => {
                write!(f, "quantum IR validation failed: {error}")
            }
        }
    }
}

impl std::error::Error for CircuitError {}

impl From<GateError> for CircuitError {
    fn from(error: GateError) -> Self {
        Self::InvalidGate { error }
    }
}

impl From<LimitsError> for CircuitError {
    fn from(error: LimitsError) -> Self {
        match error {
            LimitsError::InvalidConfiguration {
                field,
                value,
            } => Self::InvalidLimits { field, value },

            LimitsError::ResourceExceeded {
                resource,
                requested,
                maximum,
            } => match resource {
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

                "metadata bytes" => {
                    Self::MetadataLimitExceeded {
                        requested,
                        maximum,
                    }
                }

                _ => Self::InvalidCircuit {
                    message: "resource limit exceeded",
                },
            },

            LimitsError::ArithmeticOverflow { .. }
            | LimitsError::ArithmeticMultiplicationOverflow {
                ..
            } => Self::ArithmeticOverflow {
                calculation: "IR resource limit",
            },
        }
    }
}

impl From<CircuitError> for IrError {
    fn from(error: CircuitError) -> Self {
        match error {
            CircuitError::InvalidLimits { field, value } => {
                IrError::Limit(IrLimitError::new(
                    field,
                    value,
                    0,
                ))
            }

            CircuitError::QubitLimitExceeded {
                requested,
                maximum,
            } => IrError::Limit(IrLimitError::new(
                "max_qubits",
                requested,
                maximum,
            )),

            CircuitError::ClassicalBitLimitExceeded {
                requested,
                maximum,
            } => IrError::Limit(IrLimitError::new(
                "max_classical_bits",
                requested,
                maximum,
            )),

            CircuitError::OperationLimitExceeded {
                requested,
                maximum,
            } => IrError::Limit(IrLimitError::new(
                "max_operations",
                requested,
                maximum,
            )),

            CircuitError::MetadataLimitExceeded {
                requested,
                maximum,
            } => IrError::Limit(IrLimitError::new(
                "max_metadata_bytes",
                requested,
                maximum,
            )),

            CircuitError::QubitOutOfRange {
                qubit,
                num_qubits,
            } => IrError::Identifier(
                IrIdentifierError::QubitOutOfRange {
                    index: qubit.index(),
                    count: num_qubits,
                },
            ),

            CircuitError::ClassicalBitOutOfRange {
                bit,
                num_classical_bits,
            } => IrError::Identifier(
                IrIdentifierError::ClassicalBitOutOfRange {
                    index: bit,
                    count: num_classical_bits,
                },
            ),

            CircuitError::MissingOperands => {
                IrError::Circuit(
                    IrCircuitError::MissingOperands,
                )
            }

            CircuitError::DuplicateQubit { qubit } => {
                IrError::Circuit(
                    IrCircuitError::MissingOperands,
                )
            }

            CircuitError::InvalidGate { error } => {
                IrError::Gate(error.into())
            }

            CircuitError::OperationOutOfRange {
                index,
                len,
            } => IrError::Identifier(
                IrIdentifierError::OperationOutOfRange {
                    index,
                    count: len,
                },
            ),

            CircuitError::InvalidMetadata { .. } => {
                IrError::Circuit(
                    IrCircuitError::InvalidMetadata,
                )
            }

            CircuitError::UnsupportedVersion {
                ..
            } => {
                IrError::Version(
                    "unsupported quantum IR version",
                )
            }

            CircuitError::InvalidCircuit { .. } => {
                IrError::Circuit(
                    IrCircuitError::InvalidStructure,
                )
            }

            CircuitError::ArithmeticOverflow {
                ..
            } => {
                IrError::Invariant {
                    message: "circuit arithmetic overflow",
                }
            }

            CircuitError::Validation(error) => error,
        }
    }
}

// -----------------------------------------------------------------------------
// Circuit metadata
// -----------------------------------------------------------------------------

/// Metadata associated with a quantum circuit.
///
/// Metadata is deliberately limited to logical/compiler provenance. Hardware
/// configuration, calibration, topology, pulse information, and backend
/// configuration must not be stored here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitMetadata {
    /// Optional human-readable circuit name.
    name: Option<String>,

    /// Optional source language/module identifier.
    source: Option<String>,

    /// Optional compiler version string.
    compiler_version: Option<String>,

    /// Whether the circuit is intended for fault-tolerant execution.
    fault_tolerant: bool,
}

impl Default for CircuitMetadata {
    fn default() -> Self {
        Self {
            name: None,
            source: None,
            compiler_version: None,
            fault_tolerant: false,
        }
    }
}

impl CircuitMetadata {
    /// Creates empty metadata.
    pub const fn new() -> Self {
        Self {
            name: None,
            source: None,
            compiler_version: None,
            fault_tolerant: false,
        }
    }

    /// Returns the optional circuit name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the optional source identifier.
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Returns the optional compiler version.
    pub fn compiler_version(&self) -> Option<&str> {
        self.compiler_version.as_deref()
    }

    /// Returns whether fault-tolerant execution is requested.
    pub const fn fault_tolerant(&self) -> bool {
        self.fault_tolerant
    }

    /// Returns the UTF-8 byte size of all metadata strings.
    ///
    /// Length arithmetic is overflow-safe.
    pub fn byte_size(&self) -> Result<usize, CircuitError> {
        let mut total = 0usize;

        if let Some(value) = &self.name {
            total = total
                .checked_add(value.len())
                .ok_or(
                    CircuitError::ArithmeticOverflow {
                        calculation: "metadata size",
                    },
                )?;
        }

        if let Some(value) = &self.source {
            total = total
                .checked_add(value.len())
                .ok_or(
                    CircuitError::ArithmeticOverflow {
                        calculation: "metadata size",
                    },
                )?;
        }

        if let Some(value) = &self.compiler_version {
            total = total
                .checked_add(value.len())
                .ok_or(
                    CircuitError::ArithmeticOverflow {
                        calculation: "metadata size",
                    },
                )?;
        }

        Ok(total)
    }

    fn with_name(
        mut self,
        name: Option<String>,
    ) -> Self {
        self.name = name;
        self
    }

    fn with_source(
        mut self,
        source: Option<String>,
    ) -> Self {
        self.source = source;
        self
    }

    fn with_compiler_version(
        mut self,
        compiler_version: Option<String>,
    ) -> Self {
        self.compiler_version = compiler_version;
        self
    }

    fn with_fault_tolerant(
        mut self,
        fault_tolerant: bool,
    ) -> Self {
        self.fault_tolerant = fault_tolerant;
        self
    }
}

// -----------------------------------------------------------------------------
// Quantum circuit
// -----------------------------------------------------------------------------

/// Canonical Zamani quantum circuit.
///
/// The circuit owns the logical program representation but deliberately does
/// not know anything about physical hardware.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCircuit {
    id: CircuitId,
    version: IrVersion,
    num_qubits: usize,
    num_classical_bits: usize,
    limits: QuantumIrLimits,
    operations: Vec<Gate>,
    metadata: CircuitMetadata,
}

impl QuantumCircuit {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates a circuit using production IR limits and the current IR version.
    ///
    /// Unlike the previous implementation, construction is fallible. This is
    /// intentional: an invalid resource request must not create an invalid
    /// circuit that is discovered only later.
    pub fn new(
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> Result<Self, CircuitError> {
        Self::try_new_with_limits(
            num_qubits,
            num_classical_bits,
            QuantumIrLimits::production(),
        )
    }

    /// Creates a circuit with an explicit resource policy.
    pub fn try_new_with_limits(
        num_qubits: usize,
        num_classical_bits: usize,
        limits: QuantumIrLimits,
    ) -> Result<Self, CircuitError> {
        limits.validate()?;

        limits.check_qubits(num_qubits)?;
        limits.check_classical_bits(num_classical_bits)?;

        Ok(Self {
            id: CircuitId::new(0),
            version: IrVersion::CURRENT,
            num_qubits,
            num_classical_bits,
            limits,
            operations: Vec::new(),
            metadata: CircuitMetadata::default(),
        })
    }

    /// Creates a circuit with an explicit identity and resource policy.
    pub fn with_identity(
        id: CircuitId,
        num_qubits: usize,
        num_classical_bits: usize,
        limits: QuantumIrLimits,
    ) -> Result<Self, CircuitError> {
        let mut circuit = Self::try_new_with_limits(
            num_qubits,
            num_classical_bits,
            limits,
        )?;

        circuit.id = id;

        Ok(circuit)
    }

    /// Creates a circuit with explicit metadata.
    pub fn with_metadata(
        num_qubits: usize,
        num_classical_bits: usize,
        metadata: CircuitMetadata,
    ) -> Result<Self, CircuitError> {
        let mut circuit =
            Self::try_new_with_limits(
                num_qubits,
                num_classical_bits,
                QuantumIrLimits::production(),
            )?;

        circuit.set_metadata(metadata)?;

        Ok(circuit)
    }

    /// Creates a circuit with explicit metadata and limits.
    pub fn with_metadata_and_limits(
        num_qubits: usize,
        num_classical_bits: usize,
        metadata: CircuitMetadata,
        limits: QuantumIrLimits,
    ) -> Result<Self, CircuitError> {
        let mut circuit =
            Self::try_new_with_limits(
                num_qubits,
                num_classical_bits,
                limits,
            )?;

        circuit.set_metadata(metadata)?;

        Ok(circuit)
    }

    /// Creates a circuit from an existing operation sequence.
    ///
    /// The operation vector is consumed only after the circuit's resource
    /// bounds and every operation have been validated.
    pub fn from_operations(
        num_qubits: usize,
        num_classical_bits: usize,
        operations: Vec<Gate>,
    ) -> Result<Self, CircuitError> {
        let limits =
            QuantumIrLimits::production();

        Self::from_operations_with_limits(
            num_qubits,
            num_classical_bits,
            operations,
            limits,
        )
    }

    /// Creates a circuit from operations under an explicit resource policy.
    pub fn from_operations_with_limits(
        num_qubits: usize,
        num_classical_bits: usize,
        operations: Vec<Gate>,
        limits: QuantumIrLimits,
    ) -> Result<Self, CircuitError> {
        limits.validate()?;

        limits.check_qubits(num_qubits)?;
        limits.check_classical_bits(
            num_classical_bits,
        )?;
        limits.check_operations(
            operations.len(),
        )?;

        let mut circuit =
            Self::try_new_with_limits(
                num_qubits,
                num_classical_bits,
                limits,
            )?;

        // Validate every operation before mutating the circuit.
        for gate in &operations {
            circuit.validate_gate(gate)?;
        }

        circuit.operations = operations;

        Ok(circuit)
    }

    // -------------------------------------------------------------------------
    // Identity and version
    // -------------------------------------------------------------------------

    /// Returns the stable circuit identity.
    pub const fn id(&self) -> CircuitId {
        self.id
    }

    /// Changes the circuit identity.
    ///
    /// Identity is application-controlled and is not generated by the IR.
    pub const fn set_id(
        &mut self,
        id: CircuitId,
    ) {
        self.id = id;
    }

    /// Returns the IR schema/semantic version.
    pub const fn version(&self) -> IrVersion {
        self.version
    }

    /// Changes the IR version after verifying that the current implementation
    /// understands it.
    pub fn set_version(
        &mut self,
        version: IrVersion,
    ) -> Result<(), CircuitError> {
        if !version.is_supported_by_current() {
            return Err(
                CircuitError::UnsupportedVersion {
                    version,
                },
            );
        }

        self.version = version;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Resource policy
    // -------------------------------------------------------------------------

    /// Returns the resource policy owned by this circuit.
    pub const fn limits(
        &self,
    ) -> &QuantumIrLimits {
        &self.limits
    }

    /// Replaces the circuit's resource policy.
    ///
    /// The complete circuit is checked against the new policy before the
    /// replacement occurs. Failure therefore leaves the original policy
    /// untouched.
    pub fn set_limits(
        &mut self,
        limits: QuantumIrLimits,
    ) -> Result<(), CircuitError> {
        limits.validate()?;

        limits.check_qubits(
            self.num_qubits,
        )?;

        limits.check_classical_bits(
            self.num_classical_bits,
        )?;

        limits.check_operations(
            self.operations.len(),
        )?;

        let metadata_size =
            self.metadata.byte_size()?;

        limits.check_metadata_bytes(
            metadata_size,
        )?;

        self.limits = limits;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Namespace accessors
    // -------------------------------------------------------------------------

    /// Returns the number of logical qubits.
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the number of logical classical bits.
    pub const fn num_classical_bits(
        &self,
    ) -> usize {
        self.num_classical_bits
    }

    // -------------------------------------------------------------------------
    // Operation accessors
    // -------------------------------------------------------------------------

    /// Returns the number of operations.
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the circuit has no operations.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns the immutable ordered operation sequence.
    ///
    /// No mutable operation slice is exposed. All mutation must pass through
    /// the circuit's validated mutation API.
    pub fn operations(&self) -> &[Gate] {
        &self.operations
    }

    /// Returns one operation by stable sequence position.
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Gate> {
        self.operations.get(index)
    }

    /// Returns the first operation, if one exists.
    pub fn first(&self) -> Option<&Gate> {
        self.operations.first()
    }

    /// Returns the last operation, if one exists.
    pub fn last(&self) -> Option<&Gate> {
        self.operations.last()
    }

    /// Consumes the circuit and returns its ordered operations.
    pub fn into_operations(
        self,
    ) -> Vec<Gate> {
        self.operations
    }

    // -------------------------------------------------------------------------
    // Metadata
    // -------------------------------------------------------------------------

    /// Returns immutable circuit metadata.
    pub fn metadata(
        &self,
    ) -> &CircuitMetadata {
        &self.metadata
    }

    /// Replaces metadata atomically.
    ///
    /// The existing metadata remains untouched if the new metadata exceeds the
    /// configured resource policy.
    pub fn set_metadata(
        &mut self,
        metadata: CircuitMetadata,
    ) -> Result<(), CircuitError> {
        let size =
            metadata.byte_size()?;

        self.limits
            .check_metadata_bytes(size)?;

        self.metadata = metadata;

        Ok(())
    }

    /// Sets the human-readable circuit name.
    pub fn set_name(
        &mut self,
        name: Option<String>,
    ) -> Result<(), CircuitError> {
        let metadata =
            self.metadata.clone()
                .with_name(name);

        self.set_metadata(metadata)
    }

    /// Sets the logical source-language/module identifier.
    pub fn set_source(
        &mut self,
        source: Option<String>,
    ) -> Result<(), CircuitError> {
        let metadata =
            self.metadata.clone()
                .with_source(source);

        self.set_metadata(metadata)
    }

    /// Sets the compiler version provenance.
    pub fn set_compiler_version(
        &mut self,
        compiler_version: Option<String>,
    ) -> Result<(), CircuitError> {
        let metadata =
            self.metadata
                .clone()
                .with_compiler_version(
                    compiler_version,
                );

        self.set_metadata(metadata)
    }

    /// Sets the fault-tolerant execution intent.
    pub fn set_fault_tolerant(
        &mut self,
        fault_tolerant: bool,
    ) -> Result<(), CircuitError> {
        let metadata =
            self.metadata
                .clone()
                .with_fault_tolerant(
                    fault_tolerant,
                );

        self.set_metadata(metadata)
    }

    // -------------------------------------------------------------------------
    // Safe construction and mutation
    // -------------------------------------------------------------------------

    /// Appends an operation atomically.
    ///
    /// Validation occurs before the vector is modified.
    pub fn push(
        &mut self,
        gate: Gate,
    ) -> Result<(), CircuitError> {
        let next_len =
            self.operations
                .len()
                .checked_add(1)
                .ok_or(
                    CircuitError::ArithmeticOverflow {
                        calculation:
                            "operation count",
                    },
                )?;

        self.limits
            .check_operations(next_len)?;

        self.validate_gate(&gate)?;

        self.operations.push(gate);

        Ok(())
    }

    /// Inserts an operation atomically.
    pub fn insert(
        &mut self,
        index: usize,
        gate: Gate,
    ) -> Result<(), CircuitError> {
        if index > self.operations.len() {
            return Err(
                CircuitError::OperationOutOfRange {
                    index,
                    len: self.operations.len(),
                },
            );
        }

        let next_len =
            self.operations
                .len()
                .checked_add(1)
                .ok_or(
                    CircuitError::ArithmeticOverflow {
                        calculation:
                            "operation count",
                    },
                )?;

        self.limits
            .check_operations(next_len)?;

        self.validate_gate(&gate)?;

        self.operations.insert(
            index,
            gate,
        );

        Ok(())
    }

    /// Replaces an operation atomically.
    ///
    /// The old operation is not removed until the replacement has passed every
    /// validation check.
    pub fn replace(
        &mut self,
        index: usize,
        gate: Gate,
    ) -> Result<Gate, CircuitError> {
        if index >= self.operations.len() {
            return Err(
                CircuitError::OperationOutOfRange {
                    index,
                    len: self.operations.len(),
                },
            );
        }

        self.validate_gate(&gate)?;

        Ok(std::mem::replace(
            &mut self.operations[index],
            gate,
        ))
    }

    /// Removes an operation.
    ///
    /// Removal cannot violate the operation-count limit and therefore requires
    /// no speculative mutation.
    pub fn remove(
        &mut self,
        index: usize,
    ) -> Result<Gate, CircuitError> {
        if index >= self.operations.len() {
            return Err(
                CircuitError::OperationOutOfRange {
                    index,
                    len: self.operations.len(),
                },
            );
        }

        Ok(self.operations.remove(index))
    }

    /// Removes all operations.
    pub fn clear(&mut self) {
        self.operations.clear();
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates the complete circuit against its own resource policy.
    ///
    /// This delegates to the canonical validator rather than maintaining a
    /// second whole-circuit validation implementation.
    pub fn validate(
        &self,
    ) -> IrResult<()> {
        let config =
            ValidationConfig::new(
                self.limits,
            );

        validate_circuit_with_config(
            self,
            &config,
        )
    }

    /// Validates the complete circuit with an explicit validation configuration.
    pub fn validate_with_config(
        &self,
        config: &ValidationConfig,
    ) -> IrResult<()> {
        validate_circuit_with_config(
            self,
            config,
        )
    }

    /// Validates a single gate against this circuit's logical namespaces and
    /// resource policy.
    pub fn validate_gate(
        &self,
        gate: &Gate,
    ) -> Result<(), CircuitError> {
        gate.validate()?;

        if gate.qubits().is_empty() {
            return Err(
                CircuitError::MissingOperands,
            );
        }

        if gate.qubits().len()
            > self.limits.max_operands()
        {
            return Err(
                CircuitError::InvalidCircuit {
                    message:
                        "gate exceeds maximum operand count",
                },
            );
        }

        if gate.parameter_count()
            > self.limits.max_parameters()
        {
            return Err(
                CircuitError::InvalidCircuit {
                    message:
                        "gate exceeds maximum parameter count",
                },
            );
        }

        for qubit in gate.qubits() {
            self.validate_qubit(*qubit)?;
        }

        if let Some(bit) =
            gate.classical_target()
        {
            self.validate_classical_bit(bit)?;
        }

        if let Some(measurement) =
            gate.measurement()
        {
            measurement.validate(
                self.num_qubits,
                self.num_classical_bits,
            )?;
        }

        Ok(())
    }

    /// Validates a logical qubit against this circuit's namespace.
    pub fn validate_qubit(
        &self,
        qubit: QubitId,
    ) -> Result<(), CircuitError> {
        if qubit.index() >= self.num_qubits {
            return Err(
                CircuitError::QubitOutOfRange {
                    qubit,
                    num_qubits: self.num_qubits,
                },
            );
        }

        Ok(())
    }

    /// Validates a classical-bit index against this circuit's namespace.
    pub fn validate_classical_bit(
        &self,
        bit: usize,
    ) -> Result<(), CircuitError> {
        if bit >= self.num_classical_bits {
            return Err(
                CircuitError::ClassicalBitOutOfRange {
                    bit,
                    num_classical_bits:
                        self.num_classical_bits,
                },
            );
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Analysis
    // -------------------------------------------------------------------------

    /// Counts measurement operations.
    pub fn measurement_count(
        &self,
    ) -> usize {
        self.operations
            .iter()
            .filter(|gate| gate.is_measurement())
            .count()
    }

    /// Counts barrier operations.
    pub fn barrier_count(
        &self,
    ) -> usize {
        self.operations
            .iter()
            .filter(|gate| gate.is_barrier())
            .count()
    }

    /// Counts reset operations.
    pub fn reset_count(
        &self,
    ) -> usize {
        self.operations
            .iter()
            .filter(|gate| gate.is_reset())
            .count()
    }

    /// Returns whether the circuit contains measurements.
    pub fn has_measurements(
        &self,
    ) -> bool {
        self.operations
            .iter()
            .any(|gate| gate.is_measurement())
    }

    /// Returns whether the circuit contains barriers.
    pub fn has_barriers(
        &self,
    ) -> bool {
        self.operations
            .iter()
            .any(|gate| gate.is_barrier())
    }

    /// Returns the number of operations touching a logical qubit.
    pub fn qubit_gate_count(
        &self,
        qubit: QubitId,
    ) -> Result<usize, CircuitError> {
        self.validate_qubit(qubit)?;

        Ok(
            self.operations
                .iter()
                .filter(|gate| {
                    gate.qubits().contains(&qubit)
                })
                .count(),
        )
    }

    /// Returns operations touching a logical qubit.
    ///
    /// Ordering is exactly the circuit's operation ordering.
    pub fn operations_on_qubit(
        &self,
        qubit: QubitId,
    ) -> Result<Vec<&Gate>, CircuitError> {
        self.validate_qubit(qubit)?;

        Ok(
            self.operations
                .iter()
                .filter(|gate| {
                    gate.qubits().contains(&qubit)
                })
                .collect(),
        )
    }

    /// Calculates logical circuit depth.
    ///
    /// Depth is hardware-independent. It represents dependency layers over
    /// logical qubits and does not represent physical execution latency.
    ///
    /// The calculation is overflow-safe and bounded by `max_depth`.
    pub fn depth(
        &self,
    ) -> Result<usize, CircuitError> {
        if self.operations.is_empty() {
            return Ok(0);
        }

        if self.num_qubits == 0 {
            return Err(
                CircuitError::InvalidCircuit {
                    message:
                        "non-empty circuit cannot contain zero logical qubits",
                },
            );
        }

        let mut depths =
            vec![0usize; self.num_qubits];

        for gate in &self.operations {
            let mut latest = 0usize;

            for qubit in gate.qubits() {
                let current =
                    depths[qubit.index()];

                if current > latest {
                    latest = current;
                }
            }

            let next =
                latest.checked_add(1).ok_or(
                    CircuitError::ArithmeticOverflow {
                        calculation:
                            "logical circuit depth",
                    },
                )?;

            if next > self.limits.max_depth() {
                return Err(
                    CircuitError::InvalidCircuit {
                        message:
                            "circuit exceeds maximum logical depth",
                    },
                );
            }

            for qubit in gate.qubits() {
                depths[qubit.index()] = next;
            }
        }

        Ok(
            depths
                .into_iter()
                .max()
                .unwrap_or(0),
        )
    }

    // -------------------------------------------------------------------------
    // Deterministic structural helpers
    // -------------------------------------------------------------------------

    /// Returns the number of single-qubit operations.
    pub fn single_qubit_operation_count(
        &self,
    ) -> usize {
        self.operations
            .iter()
            .filter(|gate| {
                gate.qubits().len() == 1
            })
            .count()
    }

    /// Returns the number of two-qubit operations.
    pub fn two_qubit_operation_count(
        &self,
    ) -> usize {
        self.operations
            .iter()
            .filter(|gate| {
                gate.qubits().len() == 2
            })
            .count()
    }

    /// Returns the number of multi-qubit operations.
    pub fn multi_qubit_operation_count(
        &self,
    ) -> usize {
        self.operations
            .iter()
            .filter(|gate| {
                gate.qubits().len() > 2
            })
            .count()
    }

    /// Returns the number of parameterized operations.
    pub fn parameterized_operation_count(
        &self,
    ) -> usize {
        self.operations
            .iter()
            .filter(|gate| gate.is_parameterized())
            .count()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn production_limits() -> QuantumIrLimits {
        QuantumIrLimits::production()
    }

    #[test]
    fn creates_empty_circuit() {
        let circuit =
            QuantumCircuit::new(4, 4)
                .unwrap();

        assert_eq!(
            circuit.num_qubits(),
            4
        );

        assert_eq!(
            circuit.num_classical_bits(),
            4
        );

        assert!(circuit.is_empty());
        assert_eq!(
            circuit.version(),
            IrVersion::CURRENT
        );
        assert_eq!(
            circuit.id(),
            CircuitId::new(0)
        );
    }

    #[test]
    fn construction_enforces_limits() {
        let limits =
            production_limits()
                .with_max_qubits(2)
                .with_max_classical_bits(2);

        let circuit =
            QuantumCircuit::try_new_with_limits(
                3,
                2,
                limits,
            );

        assert!(matches!(
            circuit,
            Err(
                CircuitError::QubitLimitExceeded {
                    requested: 3,
                    maximum: 2,
                }
            )
        ));
    }

    #[test]
    fn push_is_atomic_on_failure() {
        let limits =
            production_limits()
                .with_max_operations(1);

        let mut circuit =
            QuantumCircuit::try_new_with_limits(
                2,
                2,
                limits,
            )
            .unwrap();

        circuit
            .push(
                Gate::x(q(0))
                    .unwrap(),
            )
            .unwrap();

        let result =
            circuit.push(
                Gate::x(q(1))
                    .unwrap(),
            );

        assert!(result.is_err());
        assert_eq!(circuit.len(), 1);
        assert_eq!(
            circuit.get(0)
                .unwrap()
                .qubits(),
            &[q(0)]
        );
    }

    #[test]
    fn rejects_out_of_range_qubit() {
        let mut circuit =
            QuantumCircuit::new(2, 2)
                .unwrap();

        let gate =
            Gate::x(q(2))
                .unwrap();

        let result =
            circuit.push(gate);

        assert!(matches!(
            result,
            Err(
                CircuitError::QubitOutOfRange {
                    ..
                }
            )
        ));

        assert!(circuit.is_empty());
    }

    #[test]
    fn accepts_two_qubit_gate() {
        let mut circuit =
            QuantumCircuit::new(2, 2)
                .unwrap();

        circuit
            .push(
                Gate::cx(
                    q(0),
                    q(1),
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(circuit.len(), 1);
    }

    #[test]
    fn insert_is_atomic() {
        let mut circuit =
            QuantumCircuit::new(2, 2)
                .unwrap();

        circuit
            .push(
                Gate::x(q(0))
                    .unwrap(),
            )
            .unwrap();

        let result =
            circuit.insert(
                1,
                Gate::x(q(2))
                    .unwrap(),
            );

        assert!(result.is_err());
        assert_eq!(circuit.len(), 1);
        assert_eq!(
            circuit.get(0)
                .unwrap()
                .qubits(),
            &[q(0)]
        );
    }

    #[test]
    fn replace_validates_before_mutation() {
        let mut circuit =
            QuantumCircuit::new(2, 2)
                .unwrap();

        circuit
            .push(
                Gate::x(q(0))
                    .unwrap(),
            )
            .unwrap();

        let result =
            circuit.replace(
                0,
                Gate::x(q(2))
                    .unwrap(),
            );

        assert!(result.is_err());
        assert_eq!(
            circuit.get(0)
                .unwrap()
                .qubits(),
            &[q(0)]
        );
    }

    #[test]
    fn remove_preserves_remaining_order() {
        let mut circuit =
            QuantumCircuit::new(3, 3)
                .unwrap();

        circuit
            .push(
                Gate::x(q(0))
                    .unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::x(q(1))
                    .unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::x(q(2))
                    .unwrap(),
            )
            .unwrap();

        circuit.remove(1).unwrap();

        assert_eq!(circuit.len(), 2);

        assert_eq!(
            circuit.get(0)
                .unwrap()
                .qubits(),
            &[q(0)]
        );

        assert_eq!(
            circuit.get(1)
                .unwrap()
                .qubits(),
            &[q(2)]
        );
    }

    #[test]
    fn metadata_mutation_is_limit_checked() {
        let limits =
            production_limits()
                .with_max_metadata_bytes(4);

        let mut circuit =
            QuantumCircuit::try_new_with_limits(
                1,
                1,
                limits,
            )
            .unwrap();

        let result =
            circuit.set_name(Some(
                "too-long".to_owned(),
            ));

        assert!(result.is_err());
        assert_eq!(
            circuit.metadata().name(),
            None
        );
    }

    #[test]
    fn metadata_mutation_is_atomic() {
        let limits =
            production_limits()
                .with_max_metadata_bytes(16);

        let mut circuit =
            QuantumCircuit::try_new_with_limits(
                1,
                1,
                limits,
            )
            .unwrap();

        circuit
            .set_name(Some(
                "valid".to_owned(),
            ))
            .unwrap();

        let result =
            circuit.set_name(Some(
                "this-name-is-too-large"
                    .to_owned(),
            ));

        assert!(result.is_err());

        assert_eq!(
            circuit.metadata().name(),
            Some("valid")
        );
    }

    #[test]
    fn depth_is_deterministic() {
        let mut circuit =
            QuantumCircuit::new(2, 2)
                .unwrap();

        circuit
            .push(
                Gate::x(q(0))
                    .unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::x(q(1))
                    .unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::cx(
                    q(0),
                    q(1),
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            circuit.depth().unwrap(),
            2
        );
    }

    #[test]
    fn depth_of_empty_circuit_is_zero() {
        let circuit =
            QuantumCircuit::new(2, 2)
                .unwrap();

        assert_eq!(
            circuit.depth().unwrap(),
            0
        );
    }

    #[test]
    fn analysis_counts_are_deterministic() {
        let mut circuit =
            QuantumCircuit::new(3, 3)
                .unwrap();

        circuit
            .push(
                Gate::x(q(0))
                    .unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::cx(
                    q(0),
                    q(1),
                )
                .unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::ccx(
                    q(0),
                    q(1),
                    q(2),
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            circuit.single_qubit_operation_count(),
            1
        );

        assert_eq!(
            circuit.two_qubit_operation_count(),
            1
        );

        assert_eq!(
            circuit.multi_qubit_operation_count(),
            1
        );
    }

    #[test]
    fn whole_circuit_validation_uses_canonical_validator() {
        let mut circuit =
            QuantumCircuit::new(2, 2)
                .unwrap();

        circuit
            .push(
                Gate::cx(
                    q(0),
                    q(1),
                )
                .unwrap(),
            )
            .unwrap();

        assert!(
            circuit.validate().is_ok()
        );
    }

    #[test]
    fn version_is_explicit() {
        let circuit =
            QuantumCircuit::new(1, 1)
                .unwrap();

        assert_eq!(
            circuit.version(),
            IrVersion::CURRENT
        );
    }

    #[test]
    fn circuit_identity_can_be_changed_explicitly() {
        let mut circuit =
            QuantumCircuit::new(1, 1)
                .unwrap();

        circuit.set_id(
            CircuitId::new(42)
        );

        assert_eq!(
            circuit.id(),
            CircuitId::new(42)
        );
    }

    #[test]
    fn unsupported_version_is_rejected() {
        let mut circuit =
            QuantumCircuit::new(1, 1)
                .unwrap();

        let result =
            circuit.set_version(
                IrVersion::new(2, 0, 0)
            );

        assert!(matches!(
            result,
            Err(
                CircuitError::UnsupportedVersion {
                    ..
                }
            )
        ));

        assert_eq!(
            circuit.version(),
            IrVersion::CURRENT
        );
    }

    #[test]
    fn from_operations_enforces_operation_limit() {
        let limits =
            production_limits()
                .with_max_operations(1);

        let operations = vec![
            Gate::x(q(0)).unwrap(),
            Gate::x(q(1)).unwrap(),
        ];

        let result =
            QuantumCircuit::from_operations_with_limits(
                2,
                2,
                operations,
                limits,
            );

        assert!(matches!(
            result,
            Err(
                CircuitError::OperationLimitExceeded {
                    requested: 2,
                    maximum: 1,
                }
            )
        ));
    }

    #[test]
    fn clear_removes_all_operations() {
        let mut circuit =
            QuantumCircuit::new(2, 2)
                .unwrap();

        circuit
            .push(
                Gate::x(q(0))
                    .unwrap(),
            )
            .unwrap();

        circuit.clear();

        assert!(circuit.is_empty());
    }

    #[test]
    fn no_mutable_operation_escape_hatch_exists() {
        // This is intentionally a compile-time/API property.
        //
        // There is no `operations_mut()` method. All operation mutations must
        // pass through push/insert/replace/remove/clear.
        let mut circuit =
            QuantumCircuit::new(1, 1)
                .unwrap();

        circuit
            .push(
                Gate::x(q(0))
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(circuit.len(), 1);
    }
}