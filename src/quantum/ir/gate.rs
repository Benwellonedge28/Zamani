//! Zamani Quantum IR — Canonical Gate Semantics
//!
//! This module defines the hardware-independent semantic representation of
//! logical quantum gates and gate-like primitive operations.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! quantum::ir::gate
//!      │
//!      ├── optimization
//!      ├── routing
//!      ├── scheduling
//!      ├── QEC
//!      ├── hardware compatibility
//!      └── backend lowering
//! ```
//!
//! `gate.rs` answers:
//!
//! > What logical quantum operation does this program request?
//!
//! It does NOT answer:
//!
//! - which physical qubit executes it;
//! - whether two physical qubits are connected;
//! - which native instruction implements it;
//! - which pulse implements it;
//! - which calibration is required;
//! - which control channel is used;
//! - when it executes;
//! - how routing is performed;
//! - how optimization is performed;
//! - how error correction is performed;
//! - how a QPU executes it.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may be compiled for:
//!
//! - one-qubit machines;
//! - small QPUs;
//! - large QPUs;
//! - distributed quantum systems;
//! - fault-tolerant logical machines;
//! - simulators;
//! - future quantum architectures.
//!
//! `Gate` therefore has no architectural maximum qubit count.
//!
//! A concrete `QuantumIrLimits` value may restrict the size of one compilation
//! or validation operation. That is a resource/security policy, not a limit
//! on the quantum computers Zamani can describe.
//!
//! # Logical identity
//!
//! All quantum operands use the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Physical qubit identities are intentionally not embedded in a `Gate`.
//! Logical-to-physical mapping belongs to the mapping/routing layers.
//!
//! # Parameter semantics
//!
//! Gate parameters use the canonical `Parameter` type from `parameter.rs`.
//! This permits:
//!
//! - constants;
//! - symbolic parameters;
//! - arithmetic expressions;
//! - deterministic parameter binding.
//!
//! `gate.rs` does not reinterpret generic parameters as hardware quantities.
//!
//! # Measurement semantics
//!
//! Measurement semantics are owned by `measurement.rs`.
//! A measurement gate may carry a canonical `Measurement` payload.
//! `gate.rs` does not duplicate measurement basis, destructive-mode, or
//! reset-after-measurement semantics.
//!
//! # Rust compatibility
//!
//! - Rust 1.97 / Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::fmt;

use super::errors::{IrError, IrResult};
use super::limits::QuantumIrLimits;
use super::measurement::Measurement;
use super::parameter::Parameter;
use super::qubit::QubitId;

/// Result type for gate construction and validation.
pub type GateResult<T> = Result<T, GateError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating a logical gate.
#[derive(Debug, Clone, PartialEq)]
pub enum GateError {
    /// The number of logical operands does not satisfy the gate contract.
    InvalidOperandCount {
        gate: GateKind,
        expected: OperandCount,
        actual: usize,
    },

    /// The same logical qubit occurs more than once in an operation where
    /// distinct operands are required.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// The number of parameters does not satisfy the gate contract.
    InvalidParameterCount {
        gate: GateKind,
        expected: usize,
        actual: usize,
    },

    /// A gate parameter is invalid.
    InvalidParameter {
        index: usize,
        parameter: Parameter,
    },

    /// A measurement gate was created without a measurement payload.
    MissingMeasurement,

    /// A non-measurement gate was given a measurement payload.
    UnexpectedMeasurement {
        gate: GateKind,
    },

    /// A measurement gate has an inconsistent classical target.
    MeasurementTargetMismatch {
        gate_target: usize,
        measurement_target: usize,
    },

    /// A measurement gate does not have the required classical destination.
    MissingClassicalTarget,

    /// A non-measurement gate was given a classical target.
    UnexpectedClassicalTarget {
        gate: GateKind,
        target: usize,
    },

    /// A barrier contains no operands.
    EmptyBarrier,

    /// Reset must operate on exactly one logical qubit.
    InvalidResetOperandCount {
        actual: usize,
    },

    /// A logical qubit does not exist in the supplied namespace.
    UnknownQubit {
        qubit: QubitId,
        logical_qubits: usize,
    },

    /// A classical destination does not exist in the supplied namespace.
    UnknownClassicalBit {
        bit: usize,
        classical_bits: usize,
    },

    /// The gate exceeds an explicit IR resource policy.
    LimitExceeded {
        resource: &'static str,
        limit: usize,
        actual: usize,
    },

    /// The gate contains an internally inconsistent representation.
    InvalidStructure {
        message: &'static str,
    },
}

impl fmt::Display for GateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperandCount {
                gate,
                expected,
                actual,
            } => write!(
                f,
                "gate {gate:?} requires {expected} operand(s), received {actual}"
            ),

            Self::DuplicateQubit { qubit } => {
                write!(f, "gate contains duplicate logical qubit {qubit}")
            }

            Self::InvalidParameterCount {
                gate,
                expected,
                actual,
            } => write!(
                f,
                "gate {gate:?} requires {expected} parameter(s), received {actual}"
            ),

            Self::InvalidParameter { index, parameter } => write!(
                f,
                "invalid parameter at index {index}: {parameter:?}"
            ),

            Self::MissingMeasurement => {
                f.write_str("measurement gate requires a measurement payload")
            }

            Self::UnexpectedMeasurement { gate } => write!(
                f,
                "gate {gate:?} cannot contain a measurement payload"
            ),

            Self::MeasurementTargetMismatch {
                gate_target,
                measurement_target,
            } => write!(
                f,
                "measurement classical target mismatch: gate target c{gate_target}, \
                 measurement target c{measurement_target}"
            ),

            Self::MissingClassicalTarget => {
                f.write_str("measurement gate requires a classical destination")
            }

            Self::UnexpectedClassicalTarget { gate, target } => write!(
                f,
                "non-measurement gate {gate:?} cannot contain classical target c{target}"
            ),

            Self::EmptyBarrier => {
                f.write_str("barrier requires at least one logical qubit")
            }

            Self::InvalidResetOperandCount { actual } => write!(
                f,
                "reset requires exactly one logical qubit, received {actual}"
            ),

            Self::UnknownQubit {
                qubit,
                logical_qubits,
            } => write!(
                f,
                "logical qubit {qubit} is outside logical namespace 0..{logical_qubits}"
            ),

            Self::UnknownClassicalBit {
                bit,
                classical_bits,
            } => write!(
                f,
                "classical bit c{bit} is outside classical namespace 0..{classical_bits}"
            ),

            Self::LimitExceeded {
                resource,
                limit,
                actual,
            } => write!(
                f,
                "gate exceeds {resource} limit: maximum {limit}, actual {actual}"
            ),

            Self::InvalidStructure { message } => {
                write!(f, "invalid gate structure: {message}")
            }
        }
    }
}

impl std::error::Error for GateError {}

impl From<GateError> for IrError {
    fn from(error: GateError) -> Self {
        Self::Gate(error)
    }
}

// =============================================================================
// Operand cardinality
// =============================================================================

/// Describes the number of logical qubits accepted by a gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperandCount {
    /// Exactly this number of operands.
    Exact(usize),

    /// At least this number of operands.
    AtLeast(usize),
}

impl OperandCount {
    /// Returns whether `actual` satisfies the operand contract.
    #[must_use]
    pub const fn accepts(self, actual: usize) -> bool {
        match self {
            Self::Exact(expected) => actual == expected,
            Self::AtLeast(minimum) => actual >= minimum,
        }
    }
}

impl fmt::Display for OperandCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(value) => write!(f, "{value}"),
            Self::AtLeast(value) => write!(f, "at least {value}"),
        }
    }
}

// =============================================================================
// Gate kind
// =============================================================================

/// Canonical logical gate kinds.
///
/// These are semantic operations, not hardware-native instructions.
///
/// A backend may later:
///
/// - decompose them;
/// - synthesize them;
/// - route them;
/// - replace them with native gates;
/// - lower them to pulses;
/// - encode them into logical fault-tolerant operations.
///
/// Such transformations do not belong in this file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateKind {
    // -------------------------------------------------------------------------
    // Single-qubit fixed gates
    // -------------------------------------------------------------------------

    /// Identity.
    I,

    /// Pauli-X.
    X,

    /// Pauli-Y.
    Y,

    /// Pauli-Z.
    Z,

    /// Hadamard.
    H,

    /// S = phase(pi/2).
    S,

    /// S dagger.
    Sdg,

    /// T = phase(pi/4).
    T,

    /// T dagger.
    Tdg,

    /// Square-root-X.
    V,

    /// Square-root-X dagger.
    Vdg,

    // -------------------------------------------------------------------------
    // Single-qubit parameterized gates
    // -------------------------------------------------------------------------

    /// Rotation around X.
    RX,

    /// Rotation around Y.
    RY,

    /// Rotation around Z.
    RZ,

    /// Generic phase gate.
    Phase,

    /// U1 phase gate.
    U1,

    /// U2 gate.
    U2,

    /// U3 gate.
    U3,

    // -------------------------------------------------------------------------
    // Two-qubit fixed gates
    // -------------------------------------------------------------------------

    /// Controlled-X / CNOT.
    CX,

    /// Controlled-Y.
    CY,

    /// Controlled-Z.
    CZ,

    /// Controlled-Hadamard.
    CH,

    /// SWAP.
    SWAP,

    /// iSWAP.
    ISWAP,

    /// Echoed cross-resonance.
    ECR,

    // -------------------------------------------------------------------------
    // Two-qubit parameterized gates
    // -------------------------------------------------------------------------

    /// Controlled-RX.
    CRX,

    /// Controlled-RY.
    CRY,

    /// Controlled-RZ.
    CRZ,

    // -------------------------------------------------------------------------
    // Three-qubit gates
    // -------------------------------------------------------------------------

    /// Toffoli / controlled-controlled-X.
    CCX,

    /// Fredkin / controlled-SWAP.
    CSWAP,

    // -------------------------------------------------------------------------
    // Non-unitary logical operations
    // -------------------------------------------------------------------------

    /// Logical measurement.
    Measure,

    /// Synchronization barrier.
    Barrier,

    /// Logical reset.
    Reset,
}

impl GateKind {
    /// Returns the logical operand contract.
    #[must_use]
    pub const fn operand_count(self) -> OperandCount {
        match self {
            Self::I
            | Self::X
            | Self::Y
            | Self::Z
            | Self::H
            | Self::S
            | Self::Sdg
            | Self::T
            | Self::Tdg
            | Self::V
            | Self::Vdg
            | Self::RX
            | Self::RY
            | Self::RZ
            | Self::Phase
            | Self::U1
            | Self::U2
            | Self::U3
            | Self::Measure
            | Self::Reset => OperandCount::Exact(1),

            Self::CX
            | Self::CY
            | Self::CZ
            | Self::CH
            | Self::SWAP
            | Self::ISWAP
            | Self::ECR
            | Self::CRX
            | Self::CRY
            | Self::CRZ => OperandCount::Exact(2),

            Self::CCX | Self::CSWAP => OperandCount::Exact(3),

            Self::Barrier => OperandCount::AtLeast(1),
        }
    }

    /// Returns the exact number of scalar parameters required.
    #[must_use]
    pub const fn parameter_count(self) -> usize {
        match self {
            Self::RX
            | Self::RY
            | Self::RZ
            | Self::Phase
            | Self::U1
            | Self::CRX
            | Self::CRY
            | Self::CRZ => 1,

            Self::U2 => 2,

            Self::U3 => 3,

            _ => 0,
        }
    }

    /// Returns whether this gate is parameterized.
    #[must_use]
    pub const fn is_parameterized(self) -> bool {
        self.parameter_count() != 0
    }

    /// Returns whether this operation is logically unitary.
    ///
    /// Barriers do not alter the quantum state and are therefore treated as
    /// unitary-compatible semantic markers for analysis purposes, while
    /// measurement and reset are non-unitary.
    #[must_use]
    pub const fn is_unitary(self) -> bool {
        !matches!(self, Self::Measure | Self::Reset)
    }

    /// Returns whether this operation is a measurement.
    #[must_use]
    pub const fn is_measurement(self) -> bool {
        matches!(self, Self::Measure)
    }

    /// Returns whether this operation is a barrier.
    #[must_use]
    pub const fn is_barrier(self) -> bool {
        matches!(self, Self::Barrier)
    }

    /// Returns whether this operation is a reset.
    #[must_use]
    pub const fn is_reset(self) -> bool {
        matches!(self, Self::Reset)
    }

    /// Returns whether the operation requires a classical destination.
    #[must_use]
    pub const fn requires_classical_target(self) -> bool {
        matches!(self, Self::Measure)
    }

    /// Returns whether the operation permits a classical destination.
    #[must_use]
    pub const fn permits_classical_target(self) -> bool {
        self.requires_classical_target()
    }

    /// Returns whether the gate is self-inverse independently of parameters.
    ///
    /// Parameterized gates are intentionally excluded because self-inverse
    /// behavior can depend on their values.
    #[must_use]
    pub const fn is_self_inverse(self) -> bool {
        matches!(
            self,
            Self::I
                | Self::X
                | Self::Y
                | Self::Z
                | Self::H
                | Self::CX
                | Self::CY
                | Self::CZ
                | Self::CH
                | Self::SWAP
                | Self::CCX
                | Self::CSWAP
        )
    }

    /// Returns whether the gate is Clifford independently of parameters.
    ///
    /// Parameterized gates are deliberately not classified as Clifford here,
    /// because only particular parameter values may be Clifford.
    #[must_use]
    pub const fn is_clifford(self) -> bool {
        matches!(
            self,
            Self::I
                | Self::X
                | Self::Y
                | Self::Z
                | Self::H
                | Self::S
                | Self::Sdg
                | Self::CX
                | Self::CY
                | Self::CZ
                | Self::SWAP
        )
    }
}

// =============================================================================
// Gate
// =============================================================================

/// Canonical hardware-independent logical quantum gate.
///
/// The fields are private so callers cannot mutate a valid gate into an
/// invalid representation without going through a constructor or an explicit
/// validated transformation.
///
/// The structure contains only semantic information.
///
/// Physical information belongs elsewhere.
#[derive(Debug, Clone, PartialEq)]
pub struct Gate {
    kind: GateKind,

    /// Logical quantum operands.
    qubits: Vec<QubitId>,

    /// Canonical symbolic/numeric parameters.
    parameters: Vec<Parameter>,

    /// Compatibility-level classical destination.
    ///
    /// For measurement gates this must agree with
    /// `measurement.classical_bit()`.
    classical_target: Option<usize>,

    /// Canonical measurement semantics.
    measurement: Option<Measurement>,
}

impl Gate {
    // -------------------------------------------------------------------------
    // General construction
    // -------------------------------------------------------------------------

    /// Constructs a complete gate representation.
    ///
    /// All local invariants are checked before the gate is returned.
    pub fn new(
        kind: GateKind,
        qubits: Vec<QubitId>,
        parameters: Vec<Parameter>,
        classical_target: Option<usize>,
        measurement: Option<Measurement>,
    ) -> GateResult<Self> {
        let gate = Self {
            kind,
            qubits,
            parameters,
            classical_target,
            measurement,
        };

        gate.validate()?;

        Ok(gate)
    }

    /// Constructs a gate with no parameters, classical target, or measurement
    /// payload.
    pub fn simple(
        kind: GateKind,
        qubits: Vec<QubitId>,
    ) -> GateResult<Self> {
        Self::new(kind, qubits, Vec::new(), None, None)
    }

    /// Constructs a parameterized gate.
    pub fn parameterized(
        kind: GateKind,
        qubits: Vec<QubitId>,
        parameters: Vec<Parameter>,
    ) -> GateResult<Self> {
        Self::new(kind, qubits, parameters, None, None)
    }

    /// Constructs a measurement gate from the canonical measurement payload.
    pub fn from_measurement(
        measurement: Measurement,
    ) -> GateResult<Self> {
        let qubit = measurement.qubit();
        let classical_target = measurement.classical_bit().index();

        Self::new(
            GateKind::Measure,
            vec![qubit],
            Vec::new(),
            Some(classical_target),
            Some(measurement),
        )
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    /// Returns the semantic gate kind.
    #[must_use]
    pub const fn kind(&self) -> GateKind {
        self.kind
    }

    /// Returns logical qubit operands in program order.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Returns canonical gate parameters in parameter order.
    #[must_use]
    pub fn parameters(&self) -> &[Parameter] {
        &self.parameters
    }

    /// Returns the compatibility-level classical destination.
    #[must_use]
    pub const fn classical_target(&self) -> Option<usize> {
        self.classical_target
    }

    /// Returns the canonical measurement payload.
    #[must_use]
    pub fn measurement(&self) -> Option<&Measurement> {
        self.measurement.as_ref()
    }

    /// Returns the first logical operand.
    #[must_use]
    pub fn qubit(&self) -> Option<QubitId> {
        self.qubits.first().copied()
    }

    /// Returns the number of logical operands.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Returns the number of parameters.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Returns whether the gate is parameterized.
    #[must_use]
    pub fn is_parameterized(&self) -> bool {
        self.kind.is_parameterized()
    }

    /// Returns whether the gate is logically unitary.
    #[must_use]
    pub fn is_unitary(&self) -> bool {
        self.kind.is_unitary()
    }

    /// Returns whether the gate is a measurement.
    #[must_use]
    pub fn is_measurement(&self) -> bool {
        self.kind.is_measurement()
    }

    /// Returns whether the gate is a barrier.
    #[must_use]
    pub fn is_barrier(&self) -> bool {
        self.kind.is_barrier()
    }

    /// Returns whether the gate is a reset.
    #[must_use]
    pub fn is_reset(&self) -> bool {
        self.kind.is_reset()
    }

    /// Returns whether all parameters are direct constants.
    #[must_use]
    pub fn parameters_are_all_constants(&self) -> bool {
        self.parameters
            .iter()
            .all(Parameter::is_constant)
    }

    /// Returns all direct constant parameters when every parameter is a
    /// constant.
    ///
    /// Returns `None` instead of silently discarding symbolic parameters.
    #[must_use]
    pub fn constant_parameters(&self) -> Option<Vec<f64>> {
        let mut values = Vec::with_capacity(self.parameters.len());

        for parameter in &self.parameters {
            match parameter.as_constant() {
                Some(value) => values.push(value),
                None => return None,
            }
        }

        Some(values)
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates all local semantic invariants.
    ///
    /// This does not validate target hardware.
    pub fn validate(&self) -> GateResult<()> {
        let expected_operands = self.kind.operand_count();

        if !expected_operands.accepts(self.qubits.len()) {
            if self.kind.is_barrier() {
                return Err(GateError::EmptyBarrier);
            }

            if self.kind.is_reset() {
                return Err(GateError::InvalidResetOperandCount {
                    actual: self.qubits.len(),
                });
            }

            return Err(GateError::InvalidOperandCount {
                gate: self.kind,
                expected: expected_operands,
                actual: self.qubits.len(),
            });
        }

        self.validate_distinct_operands()?;

        let expected_parameters = self.kind.parameter_count();

        if self.parameters.len() != expected_parameters {
            return Err(GateError::InvalidParameterCount {
                gate: self.kind,
                expected: expected_parameters,
                actual: self.parameters.len(),
            });
        }

        for (index, parameter) in self.parameters.iter().enumerate() {
            if let Err(_) = parameter.validate() {
                return Err(GateError::InvalidParameter {
                    index,
                    parameter: parameter.clone(),
                });
            }
        }

        if self.kind.requires_classical_target() {
            let classical_target = self
                .classical_target
                .ok_or(GateError::MissingClassicalTarget)?;

            let measurement = self
                .measurement
                .as_ref()
                .ok_or(GateError::MissingMeasurement)?;

            let measurement_target =
                measurement.classical_bit().index();

            if classical_target != measurement_target {
                return Err(
                    GateError::MeasurementTargetMismatch {
                        gate_target: classical_target,
                        measurement_target,
                    },
                );
            }

            if measurement.qubit() != self.qubits[0] {
                return Err(GateError::InvalidStructure {
                    message:
                        "measurement payload qubit does not match gate operand",
                });
            }
        } else {
            if let Some(target) = self.classical_target {
                return Err(GateError::UnexpectedClassicalTarget {
                    gate: self.kind,
                    target,
                });
            }

            if self.measurement.is_some() {
                return Err(GateError::UnexpectedMeasurement {
                    gate: self.kind,
                });
            }
        }

        if self.kind.is_barrier() && self.qubits.is_empty() {
            return Err(GateError::EmptyBarrier);
        }

        if self.kind.is_reset() && self.qubits.len() != 1 {
            return Err(GateError::InvalidResetOperandCount {
                actual: self.qubits.len(),
            });
        }

        Ok(())
    }

    /// Validates the gate against explicit IR resource policy.
    ///
    /// This is deliberately separate from semantic validation.
    ///
    /// A limit failure does NOT mean the gate is semantically invalid. It
    /// means the selected compilation/security policy does not currently
    /// permit this representation.
    pub fn validate_with_limits(
        &self,
        limits: &QuantumIrLimits,
    ) -> GateResult<()> {
        self.validate()?;

        let operand_count = self.qubits.len();

        if operand_count > limits.max_operands() {
            return Err(GateError::LimitExceeded {
                resource: "gate operands",
                limit: limits.max_operands(),
                actual: operand_count,
            });
        }

        let parameter_count = self.parameters.len();

        if parameter_count > limits.max_parameters() {
            return Err(GateError::LimitExceeded {
                resource: "gate parameters",
                limit: limits.max_parameters(),
                actual: parameter_count,
            });
        }

        Ok(())
    }

    /// Validates logical operand identifiers against a logical namespace.
    ///
    /// `logical_qubits` is the declared logical namespace size, not a hardware
    /// capacity.
    pub fn validate_in_namespace(
        &self,
        logical_qubits: usize,
    ) -> GateResult<()> {
        self.validate()?;

        for &qubit in &self.qubits {
            if qubit.index() >= logical_qubits {
                return Err(GateError::UnknownQubit {
                    qubit,
                    logical_qubits,
                });
            }
        }

        Ok(())
    }

    /// Validates the gate against both a resource policy and a logical
    /// namespace.
    pub fn validate_with_context(
        &self,
        limits: &QuantumIrLimits,
        logical_qubits: usize,
    ) -> GateResult<()> {
        self.validate_with_limits(limits)?;
        self.validate_in_namespace(logical_qubits)
    }

    /// Validates the classical destination against a classical namespace.
    ///
    /// Non-measurement gates have no classical destination and therefore pass
    /// this check.
    pub fn validate_classical_namespace(
        &self,
        classical_bits: usize,
    ) -> GateResult<()> {
        self.validate()?;

        if let Some(bit) = self.classical_target {
            if bit >= classical_bits {
                return Err(GateError::UnknownClassicalBit {
                    bit,
                    classical_bits,
                });
            }
        }

        Ok(())
    }

    /// Performs complete local/context validation.
    pub fn validate_complete(
        &self,
        limits: &QuantumIrLimits,
        logical_qubits: usize,
        classical_bits: usize,
    ) -> GateResult<()> {
        self.validate_with_limits(limits)?;
        self.validate_in_namespace(logical_qubits)?;
        self.validate_classical_namespace(classical_bits)?;

        Ok(())
    }

    /// Checks that all logical operands are distinct.
    ///
    /// The implementation uses no unchecked arithmetic and does not rely on
    /// fixed qubit counts.
    fn validate_distinct_operands(&self) -> GateResult<()> {
        for index in 0..self.qubits.len() {
            let current = self.qubits[index];

            for other_index in (index + 1)..self.qubits.len() {
                if current == self.qubits[other_index] {
                    return Err(GateError::DuplicateQubit {
                        qubit: current,
                    });
                }
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Fixed single-qubit gates
    // -------------------------------------------------------------------------

    /// Identity gate.
    pub fn identity(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::I, vec![qubit])
    }

    /// X gate.
    pub fn x(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::X, vec![qubit])
    }

    /// Y gate.
    pub fn y(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Y, vec![qubit])
    }

    /// Z gate.
    pub fn z(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Z, vec![qubit])
    }

    /// Hadamard gate.
    pub fn h(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::H, vec![qubit])
    }

    /// S gate.
    pub fn s(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::S, vec![qubit])
    }

    /// S dagger gate.
    pub fn sdg(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Sdg, vec![qubit])
    }

    /// T gate.
    pub fn t(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::T, vec![qubit])
    }

    /// T dagger gate.
    pub fn tdg(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Tdg, vec![qubit])
    }

    /// Square-root-X gate.
    pub fn v(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::V, vec![qubit])
    }

    /// Square-root-X dagger gate.
    pub fn vdg(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Vdg, vec![qubit])
    }

    // -------------------------------------------------------------------------
    // Parameterized single-qubit gates
    // -------------------------------------------------------------------------

    /// RX(theta).
    pub fn rx(
        qubit: QubitId,
        theta: Parameter,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::RX,
            vec![qubit],
            vec![theta],
        )
    }

    /// RY(theta).
    pub fn ry(
        qubit: QubitId,
        theta: Parameter,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::RY,
            vec![qubit],
            vec![theta],
        )
    }

    /// RZ(theta).
    pub fn rz(
        qubit: QubitId,
        theta: Parameter,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::RZ,
            vec![qubit],
            vec![theta],
        )
    }

    /// Phase(theta).
    pub fn phase(
        qubit: QubitId,
        theta: Parameter,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::Phase,
            vec![qubit],
            vec![theta],
        )
    }

    /// U1(lambda).
    pub fn u1(
        qubit: QubitId,
        lambda: Parameter,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::U1,
            vec![qubit],
            vec![lambda],
        )
    }

    /// U2(phi, lambda).
    pub fn u2(
        qubit: QubitId,
        phi: Parameter,
        lambda: Parameter,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::U2,
            vec![qubit],
            vec![phi, lambda],
        )
    }

    /// U3(theta, phi, lambda).
    pub fn u3(
        qubit: QubitId,
        theta: Parameter,
        phi: Parameter,
        lambda: Parameter,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::U3,
            vec![qubit],
            vec![theta, phi, lambda],
        )
    }

    // -------------------------------------------------------------------------
    // Fixed two-qubit gates
    // -------------------------------------------------------------------------

    /// Controlled-X / CNOT.
    pub fn cx(
        control: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CX,
            vec![control, target],
        )
    }

    /// Controlled-Y.
    pub fn cy(
        control: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CY,
            vec![control, target],
        )
    }

    /// Controlled-Z.
    pub fn cz(
        control: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CZ,
            vec![control, target],
        )
    }

    /// Controlled-Hadamard.
    pub fn ch(
        control: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CH,
            vec![control, target],
        )
    }

    /// SWAP.
    pub fn swap(
        first: QubitId,
        second: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::SWAP,
            vec![first, second],
        )
    }

    /// iSWAP.
    pub fn iswap(
        first: QubitId,
        second: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::ISWAP,
            vec![first, second],
        )
    }

    /// Echoed cross-resonance gate.
    pub fn ecr(
        control: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::ECR,
            vec![control, target],
        )
    }

    // -------------------------------------------------------------------------
    // Parameterized two-qubit gates
    // -------------------------------------------------------------------------

    /// Controlled-RX(theta).
    pub fn crx(
        control: QubitId,
        target: QubitId,
        theta: Parameter,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::CRX,
            vec![control, target],
            vec![theta],
        )
    }

    /// Controlled-RY(theta).
    pub fn cry(
        control: QubitId,
        target: QubitId,
        theta: Parameter,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::CRY,
            vec![control, target],
            vec![theta],
        )
    }

    /// Controlled-RZ(theta).
    pub fn crz(
        control: QubitId,
        target: QubitId,
        theta: Parameter,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::CRZ,
            vec![control, target],
            vec![theta],
        )
    }

    // -------------------------------------------------------------------------
    // Three-qubit gates
    // -------------------------------------------------------------------------

    /// Toffoli / CCX.
    pub fn ccx(
        control_a: QubitId,
        control_b: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CCX,
            vec![control_a, control_b, target],
        )
    }

    /// Fredkin / controlled-SWAP.
    pub fn cswap(
        control: QubitId,
        target_a: QubitId,
        target_b: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CSWAP,
            vec![control, target_a, target_b],
        )
    }

    // -------------------------------------------------------------------------
    // Measurement / reset / barrier
    // -------------------------------------------------------------------------

    /// Constructs a measurement gate from the canonical measurement object.
    pub fn measure(
        measurement: Measurement,
    ) -> GateResult<Self> {
        Self::from_measurement(measurement)
    }

    /// Constructs a measurement gate using a classical-bit index.
    ///
    /// The index is converted into the canonical measurement namespace.
    pub fn measure_to(
        qubit: QubitId,
        classical_bit: usize,
    ) -> GateResult<Self> {
        let measurement = Measurement::new(
            qubit,
            super::measurement::ClassicalBitId::new(
                classical_bit,
            ),
        );

        Self::from_measurement(measurement)
    }

    /// Constructs a barrier over one or more logical qubits.
    pub fn barrier(
        qubits: Vec<QubitId>,
    ) -> GateResult<Self> {
        Self::simple(GateKind::Barrier, qubits)
    }

    /// Constructs a single-qubit logical reset.
    pub fn reset(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Reset, vec![qubit])
    }

    // -------------------------------------------------------------------------
    // Deterministic semantic helpers
    // -------------------------------------------------------------------------

    /// Returns whether this gate operates on a particular logical qubit.
    #[must_use]
    pub fn touches(&self, qubit: QubitId) -> bool {
        self.qubits.contains(&qubit)
    }

    /// Returns whether two gates share at least one logical qubit.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.qubits
            .iter()
            .any(|qubit| other.qubits.contains(qubit))
    }

    /// Returns whether this gate can be considered independent of another gate
    /// at the logical operand level.
    ///
    /// This is deliberately only an operand-overlap test. It is NOT a complete
    /// commutation proof.
    #[must_use]
    pub fn is_operand_disjoint(&self, other: &Self) -> bool {
        !self.overlaps(other)
    }

    /// Returns a deterministic semantic summary useful to analysis and
    /// debugging.
    #[must_use]
    pub fn semantic_summary(&self) -> GateSummary {
        GateSummary {
            kind: self.kind,
            operand_count: self.qubits.len(),
            parameter_count: self.parameters.len(),
            has_classical_target: self.classical_target.is_some(),
            has_measurement: self.measurement.is_some(),
        }
    }
}

// =============================================================================
// Gate summary
// =============================================================================

/// Allocation-free summary of gate semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateSummary {
    /// Gate kind.
    pub kind: GateKind,

    /// Number of logical qubit operands.
    pub operand_count: usize,

    /// Number of parameters.
    pub parameter_count: usize,

    /// Whether a classical destination exists.
    pub has_classical_target: bool,

    /// Whether canonical measurement semantics are attached.
    pub has_measurement: bool,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn p(value: f64) -> Parameter {
        Parameter::Constant(value)
    }

    #[test]
    fn x_gate_is_valid() {
        let gate = Gate::x(q(0)).expect("X gate should be valid");

        assert_eq!(gate.kind(), GateKind::X);
        assert_eq!(gate.qubits(), &[q(0)]);
        assert!(gate.is_unitary());
        assert!(!gate.is_parameterized());
    }

    #[test]
    fn h_gate_is_clifford() {
        assert!(GateKind::H.is_clifford());
        assert!(GateKind::H.is_self_inverse());
    }

    #[test]
    fn parameterized_rx_is_valid() {
        let gate = Gate::rx(q(0), p(0.5))
            .expect("RX should be valid");

        assert_eq!(gate.kind(), GateKind::RX);
        assert_eq!(gate.parameter_count(), 1);
        assert!(gate.is_parameterized());
    }

    #[test]
    fn symbolic_parameter_is_accepted() {
        let parameter =
            Parameter::symbol("theta")
                .expect("symbol should be valid");

        let gate =
            Gate::rx(q(7), parameter)
                .expect("symbolic RX should be valid");

        assert!(gate.is_parameterized());
        assert!(!gate.parameters_are_all_constants());
    }

    #[test]
    fn cx_requires_two_distinct_qubits() {
        assert!(Gate::cx(q(0), q(1)).is_ok());
        assert!(matches!(
            Gate::cx(q(0), q(0)),
            Err(GateError::DuplicateQubit { .. })
        ));
    }

    #[test]
    fn ccx_requires_three_distinct_qubits() {
        assert!(
            Gate::ccx(q(0), q(1), q(2)).is_ok()
        );

        assert!(matches!(
            Gate::ccx(q(0), q(1), q(1)),
            Err(GateError::DuplicateQubit { .. })
        ));
    }

    #[test]
    fn barrier_requires_at_least_one_qubit() {
        assert!(matches!(
            Gate::barrier(Vec::new()),
            Err(GateError::EmptyBarrier)
        ));

        assert!(
            Gate::barrier(vec![q(0), q(1)]).is_ok()
        );
    }

    #[test]
    fn reset_requires_exactly_one_qubit() {
        assert!(Gate::reset(q(0)).is_ok());

        let result = Gate::simple(
            GateKind::Reset,
            vec![q(0), q(1)],
        );

        assert!(matches!(
            result,
            Err(GateError::InvalidResetOperandCount { actual: 2 })
        ));
    }

    #[test]
    fn measurement_has_classical_target() {
        let gate = Gate::measure_to(q(0), 3)
            .expect("measurement should be valid");

        assert!(gate.is_measurement());
        assert_eq!(gate.classical_target(), Some(3));
        assert!(gate.measurement().is_some());
    }

    #[test]
    fn measurement_payload_and_gate_target_must_agree() {
        let measurement =
            Measurement::new(
                q(0),
                super::super::measurement::ClassicalBitId::new(3),
            );

        let gate = Gate::new(
            GateKind::Measure,
            vec![q(0)],
            Vec::new(),
            Some(4),
            Some(measurement),
        );

        assert!(matches!(
            gate,
            Err(GateError::MeasurementTargetMismatch {
                gate_target: 4,
                measurement_target: 3
            })
        ));
    }

    #[test]
    fn non_measurement_cannot_have_classical_target() {
        let result = Gate::new(
            GateKind::X,
            vec![q(0)],
            Vec::new(),
            Some(0),
            None,
        );

        assert!(matches!(
            result,
            Err(GateError::UnexpectedClassicalTarget { .. })
        ));
    }

    #[test]
    fn non_measurement_cannot_have_measurement_payload() {
        let measurement =
            Measurement::new(
                q(0),
                super::super::measurement::ClassicalBitId::new(0),
            );

        let result = Gate::new(
            GateKind::X,
            vec![q(0)],
            Vec::new(),
            None,
            Some(measurement),
        );

        assert!(matches!(
            result,
            Err(GateError::UnexpectedMeasurement {
                gate: GateKind::X
            })
        ));
    }

    #[test]
    fn wrong_parameter_count_is_rejected() {
        let result = Gate::parameterized(
            GateKind::U3,
            vec![q(0)],
            vec![p(0.1), p(0.2)],
        );

        assert!(matches!(
            result,
            Err(GateError::InvalidParameterCount {
                gate: GateKind::U3,
                expected: 3,
                actual: 2
            })
        ));
    }

    #[test]
    fn non_finite_parameter_is_rejected() {
        let result = Gate::parameterized(
            GateKind::RX,
            vec![q(0)],
            vec![Parameter::Constant(f64::NAN)],
        );

        assert!(matches!(
            result,
            Err(GateError::InvalidParameter { index: 0, .. })
        ));
    }

    #[test]
    fn logical_namespace_is_explicit() {
        let gate = Gate::x(q(99))
            .expect("gate itself is structurally valid");

        assert!(matches!(
            gate.validate_in_namespace(99),
            Err(GateError::UnknownQubit {
                qubit,
                logical_qubits: 99
            }) if qubit == q(99)
        ));

        assert!(
            gate.validate_in_namespace(100).is_ok()
        );
    }

    #[test]
    fn arbitrary_logical_identifier_has_no_small_machine_limit() {
        let high = usize::MAX;

        let gate = Gate::x(q(high))
            .expect("QubitId is a semantic identifier");

        assert_eq!(
            gate.qubits()[0].index(),
            high
        );
    }

    #[test]
    fn disjoint_gates_are_operand_disjoint() {
        let first =
            Gate::x(q(0)).expect("valid gate");

        let second =
            Gate::h(q(1)).expect("valid gate");

        assert!(
            first.is_operand_disjoint(&second)
        );
    }

    #[test]
    fn overlapping_gates_are_not_operand_disjoint() {
        let first =
            Gate::x(q(0)).expect("valid gate");

        let second =
            Gate::h(q(0)).expect("valid gate");

        assert!(
            !first.is_operand_disjoint(&second)
        );
    }

    #[test]
    fn gate_summary_is_deterministic() {
        let gate =
            Gate::u3(
                q(0),
                p(0.1),
                p(0.2),
                p(0.3),
            )
            .expect("valid U3");

        let summary =
            gate.semantic_summary();

        assert_eq!(
            summary.kind,
            GateKind::U3
        );

        assert_eq!(
            summary.operand_count,
            1
        );

        assert_eq!(
            summary.parameter_count,
            3
        );

        assert!(
            !summary.has_classical_target
        );

        assert!(
            !summary.has_measurement
        );
    }
}