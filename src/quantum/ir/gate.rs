//! Canonical, hardware-independent quantum IR gate representation.
//!
//! This module defines the semantic representation and operation-level
//! invariants for quantum gates.
//!
//! Architectural boundary:
//!
//! ```text
//! frontend
//!    │
//!    ▼
//! quantum::ir::Gate
//!    │
//!    ├── optimization
//!    ├── routing
//!    ├── scheduling
//!    ├── error correction
//!    └── hardware
//! ```
//!
//! This module intentionally does NOT contain:
//!
//! - physical topology;
//! - logical-to-physical routing;
//! - pulse schedules;
//! - calibration;
//! - backend execution;
//! - QPU communication;
//! - error-correction decoding;
//! - optimization algorithms.
//!
//! Those concerns belong to downstream quantum subsystems.
//!
//! # Invariants
//!
//! A valid [`Gate`] guarantees:
//!
//! - all logical operands are unique;
//! - operand count matches the gate kind;
//! - parameter count matches the gate kind;
//! - all numerical parameters are finite;
//! - measurement gates have a classical destination;
//! - non-measurement gates do not have classical destinations;
//! - barriers contain at least one operand;
//! - reset operations contain exactly one operand;
//! - logical and physical qubit identities remain distinct;
//! - resource limits can be checked without mutating the gate.
//!
//! Constructors enforce the strongest local invariants available.
//! [`Gate::validate`] remains available because IR can eventually be
//! deserialized or produced by untrusted external tooling.

use std::fmt;

use super::errors::{IrError, IrResult};
use super::limits::QuantumIrLimits;
use super::measurement::Measurement;
use super::parameter::Parameter;
use super::qubits::QubitId;

/// Result type for gate-level operations.
pub type GateResult<T> = Result<T, GateError>;

/// Errors produced while constructing or validating a [`Gate`].
///
/// This type intentionally preserves structured information.  It can be
/// converted into the canonical [`IrError`] through `From<GateError>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateError {
    /// The gate has an invalid number of logical operands.
    InvalidOperandCount {
        gate: GateKind,
        expected: OperandCount,
        actual: usize,
    },

    /// The gate contains duplicate logical qubit operands.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// The gate contains an invalid parameter count.
    InvalidParameterCount {
        gate: GateKind,
        expected: usize,
        actual: usize,
    },

    /// A required classical target is missing.
    MissingClassicalTarget,

    /// A classical target was supplied to an operation that does not support
    /// one.
    UnexpectedClassicalTarget {
        gate: GateKind,
    },

    /// A barrier must contain at least one operand.
    EmptyBarrier,

    /// Reset must target exactly one logical qubit.
    InvalidResetOperandCount {
        actual: usize,
    },

    /// A parameter is not valid for a quantum IR gate.
    InvalidParameter {
        index: usize,
        parameter: Parameter,
    },

    /// The gate's measurement payload is inconsistent with its kind.
    InvalidMeasurement,

    /// The operation exceeds the supplied IR limits.
    LimitExceeded {
        resource: &'static str,
        limit: usize,
        actual: usize,
    },

    /// A qubit is outside the supplied logical namespace.
    UnknownQubit {
        qubit: QubitId,
    },

    /// A classical target is outside the supplied classical namespace.
    UnknownClassicalBit {
        bit: usize,
    },

    /// The gate's internal state is inconsistent.
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
            } => {
                write!(
                    f,
                    "gate {gate:?} requires {expected}, but received {actual} operands"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(f, "gate contains duplicate logical qubit {qubit:?}")
            }

            Self::InvalidParameterCount {
                gate,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "gate {gate:?} requires {expected} parameters, but received {actual}"
                )
            }

            Self::MissingClassicalTarget => {
                write!(f, "measurement gate requires a classical target")
            }

            Self::UnexpectedClassicalTarget { gate } => {
                write!(
                    f,
                    "gate {gate:?} does not accept a classical target"
                )
            }

            Self::EmptyBarrier => {
                write!(f, "barrier must contain at least one qubit")
            }

            Self::InvalidResetOperandCount { actual } => {
                write!(
                    f,
                    "reset requires exactly one qubit, but received {actual}"
                )
            }

            Self::InvalidParameter { index, parameter } => {
                write!(
                    f,
                    "invalid parameter at index {index}: {parameter:?}"
                )
            }

            Self::InvalidMeasurement => {
                write!(f, "invalid measurement payload")
            }

            Self::LimitExceeded {
                resource,
                limit,
                actual,
            } => {
                write!(
                    f,
                    "gate exceeds {resource} limit: maximum {limit}, actual {actual}"
                )
            }

            Self::UnknownQubit { qubit } => {
                write!(f, "unknown logical qubit {qubit:?}")
            }

            Self::UnknownClassicalBit { bit } => {
                write!(f, "unknown classical bit {bit}")
            }

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

/// Describes how many operands a gate requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandCount {
    /// Exactly one operand.
    Exact(usize),

    /// Any number of operands greater than or equal to the supplied minimum.
    AtLeast(usize),
}

impl OperandCount {
    /// Returns whether `actual` satisfies this operand-count requirement.
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

/// Canonical gate kinds supported by the logical quantum IR.
///
/// These describe logical operations only.  A backend may later decompose,
/// synthesize, route, or otherwise transform them for a physical device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateKind {
    // Single-qubit gates.
    I,
    X,
    Y,
    Z,
    H,
    S,
    Sdg,
    T,
    Tdg,
    V,
    Vdg,

    // Parameterized single-qubit gates.
    RX,
    RY,
    RZ,
    Phase,
    U1,
    U2,
    U3,

    // Two-qubit gates.
    CX,
    CY,
    CZ,
    CH,
    SWAP,
    ISWAP,
    ECR,

    // Parameterized two-qubit gates.
    CRX,
    CRY,
    CRZ,

    // Three-qubit gates.
    CCX,
    CSWAP,

    // Non-unitary logical operations.
    Measure,
    Barrier,
    Reset,
}

impl GateKind {
    /// Returns the required number of logical qubit operands.
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

    /// Returns the exact number of parameters required by this gate.
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

    /// Returns whether this gate requires a classical destination.
    #[must_use]
    pub const fn requires_classical_target(self) -> bool {
        matches!(self, Self::Measure)
    }

    /// Returns whether this gate permits a classical destination.
    #[must_use]
    pub const fn permits_classical_target(self) -> bool {
        self.requires_classical_target()
    }

    /// Returns whether this gate is a measurement.
    #[must_use]
    pub const fn is_measurement(self) -> bool {
        matches!(self, Self::Measure)
    }

    /// Returns whether this gate is a barrier.
    #[must_use]
    pub const fn is_barrier(self) -> bool {
        matches!(self, Self::Barrier)
    }

    /// Returns whether this gate resets a logical qubit.
    #[must_use]
    pub const fn is_reset(self) -> bool {
        matches!(self, Self::Reset)
    }

    /// Returns whether this is a parameterized gate.
    #[must_use]
    pub const fn is_parameterized(self) -> bool {
        self.parameter_count() != 0
    }

    /// Returns whether this operation is unitary at the logical IR level.
    ///
    /// Measurement, barrier, and reset are deliberately non-unitary.
    #[must_use]
    pub const fn is_unitary(self) -> bool {
        !matches!(self, Self::Measure | Self::Barrier | Self::Reset)
    }

    /// Returns whether this gate is self-inverse.
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

    /// Returns whether the gate is a Clifford gate.
    ///
    /// Parameterized rotations are not classified as Clifford merely because
    /// some specific parameter values may happen to be Clifford.
    ///
    /// The compiler can perform value-sensitive classification later.
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

/// A canonical logical quantum operation.
///
/// `Gate` is intentionally immutable through its public API.  This prevents
/// callers from constructing a valid gate and then mutating it into an
/// invalid state without validation.
#[derive(Debug, Clone, PartialEq)]
pub struct Gate {
    kind: GateKind,
    qubits: Vec<QubitId>,
    parameters: Vec<Parameter>,
    classical_target: Option<usize>,
    measurement: Option<Measurement>,
}

impl Gate {
    /// Creates a gate from its complete logical representation.
    ///
    /// All local invariants are validated before the gate is returned.
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

    /// Returns the gate kind.
    #[must_use]
    pub const fn kind(&self) -> GateKind {
        self.kind
    }

    /// Returns the logical qubit operands in deterministic order.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Returns the gate parameters in deterministic order.
    #[must_use]
    pub fn parameters(&self) -> &[Parameter] {
        &self.parameters
    }

    /// Returns the optional classical destination.
    #[must_use]
    pub const fn classical_target(&self) -> Option<usize> {
        self.classical_target
    }

    /// Returns the measurement payload, if present.
    #[must_use]
    pub const fn measurement(&self) -> Option<&Measurement> {
        self.measurement.as_ref()
    }

    /// Returns the first logical qubit.
    #[must_use]
    pub fn qubit(&self) -> Option<QubitId> {
        self.qubits.first().copied()
    }

    /// Returns whether this operation is parameterized.
    #[must_use]
    pub fn is_parameterized(&self) -> bool {
        self.kind.is_parameterized()
    }

    /// Returns whether this operation is unitary.
    #[must_use]
    pub fn is_unitary(&self) -> bool {
        self.kind.is_unitary()
    }

    /// Returns whether this operation is a measurement.
    #[must_use]
    pub fn is_measurement(&self) -> bool {
        self.kind.is_measurement()
    }

    /// Returns whether this operation is a barrier.
    #[must_use]
    pub fn is_barrier(&self) -> bool {
        self.kind.is_barrier()
    }

    /// Returns whether this operation is a reset.
    #[must_use]
    pub fn is_reset(&self) -> bool {
        self.kind.is_reset()
    }

    /// Returns the constant numerical parameters.
    ///
    /// Returns `None` when the gate contains symbolic or expression
    /// parameters.  This avoids silently losing symbolic information.
    #[must_use]
    pub fn constant_parameters(&self) -> Option<Vec<f64>> {
        let mut result = Vec::with_capacity(self.parameters.len());

        for parameter in &self.parameters {
            match parameter {
                Parameter::Constant(value) => result.push(*value),
                _ => return None,
            }
        }

        Some(result)
    }

    /// Returns the constant parameters as a slice when all parameters are
    /// constants.
    ///
    /// This method avoids allocation but intentionally exposes the original
    /// parameter slice rather than synthesizing a separate `f64` slice.
    #[must_use]
    pub fn parameters_are_all_constants(&self) -> bool {
        self.parameters
            .iter()
            .all(|parameter| matches!(parameter, Parameter::Constant(_)))
    }

    /// Validates all local gate invariants.
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

        self.validate_unique_qubits()?;

        let expected_parameters = self.kind.parameter_count();

        if self.parameters.len() != expected_parameters {
            return Err(GateError::InvalidParameterCount {
                gate: self.kind,
                expected: expected_parameters,
                actual: self.parameters.len(),
            });
        }

        for (index, parameter) in self.parameters.iter().enumerate() {
            if !parameter.is_finite() {
                return Err(GateError::InvalidParameter {
                    index,
                    parameter: parameter.clone(),
                });
            }
        }

        if self.kind.requires_classical_target() {
            if self.classical_target.is_none() {
                return Err(GateError::MissingClassicalTarget);
            }
        } else if self.classical_target.is_some() {
            return Err(GateError::UnexpectedClassicalTarget {
                gate: self.kind,
            });
        }

        if self.kind.is_measurement() {
            if self.measurement.is_none() {
                return Err(GateError::InvalidMeasurement);
            }
        } else if self.measurement.is_some() {
            return Err(GateError::InvalidStructure {
                message: "measurement payload supplied to non-measurement gate",
            });
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

    /// Validates the gate against resource limits.
    ///
    /// This performs no allocation and does not mutate the gate.
    pub fn validate_with_limits(&self, limits: &QuantumIrLimits) -> GateResult<()> {
        self.validate()?;

        if self.qubits.len() > limits.max_operands() {
            return Err(GateError::LimitExceeded {
                resource: "gate operands",
                limit: limits.max_operands(),
                actual: self.qubits.len(),
            });
        }

        if self.parameters.len() > limits.max_parameters() {
            return Err(GateError::LimitExceeded {
                resource: "gate parameters",
                limit: limits.max_parameters(),
                actual: self.parameters.len(),
            });
        }

        Ok(())
    }

    /// Validates that every logical operand exists in a logical qubit
    /// namespace of `logical_qubits` qubits.
    pub fn validate_in_namespace(&self, logical_qubits: usize) -> GateResult<()> {
        self.validate()?;

        for &qubit in &self.qubits {
            if qubit.index() >= logical_qubits {
                return Err(GateError::UnknownQubit { qubit });
            }
        }

        if let Some(classical_target) = self.classical_target {
            // The caller owns the classical namespace and therefore this
            // method deliberately does not validate its size.
            let _ = classical_target;
        }

        Ok(())
    }

    /// Validates the gate against both resource limits and logical namespace.
    pub fn validate_with_context(
        &self,
        limits: &QuantumIrLimits,
        logical_qubits: usize,
    ) -> GateResult<()> {
        self.validate_with_limits(limits)?;
        self.validate_in_namespace(logical_qubits)
    }

    fn validate_unique_qubits(&self) -> GateResult<()> {
        for (index, &left) in self.qubits.iter().enumerate() {
            for &right in self.qubits.iter().skip(index + 1) {
                if left == right {
                    return Err(GateError::DuplicateQubit { qubit: left });
                }
            }
        }

        Ok(())
    }

    /// Constructs an identity operation.
    pub fn identity(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::I, qubit)
    }

    /// Constructs an X gate.
    pub fn x(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::X, qubit)
    }

    /// Constructs a Y gate.
    pub fn y(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Y, qubit)
    }

    /// Constructs a Z gate.
    pub fn z(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Z, qubit)
    }

    /// Constructs a Hadamard gate.
    pub fn h(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::H, qubit)
    }

    /// Constructs an S gate.
    pub fn s(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::S, qubit)
    }

    /// Constructs an S-dagger gate.
    pub fn sdg(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Sdg, qubit)
    }

    /// Constructs a T gate.
    pub fn t(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::T, qubit)
    }

    /// Constructs a T-dagger gate.
    pub fn tdg(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Tdg, qubit)
    }

    /// Constructs a V gate.
    pub fn v(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::V, qubit)
    }

    /// Constructs a V-dagger gate.
    pub fn vdg(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Vdg, qubit)
    }

    /// Constructs an RX rotation.
    pub fn rx(qubit: QubitId, theta: f64) -> GateResult<Self> {
        Self::parameterized(
            GateKind::RX,
            vec![qubit],
            vec![Parameter::constant(theta)?],
        )
    }

    /// Constructs an RY rotation.
    pub fn ry(qubit: QubitId, theta: f64) -> GateResult<Self> {
        Self::parameterized(
            GateKind::RY,
            vec![qubit],
            vec![Parameter::constant(theta)?],
        )
    }

    /// Constructs an RZ rotation.
    pub fn rz(qubit: QubitId, theta: f64) -> GateResult<Self> {
        Self::parameterized(
            GateKind::RZ,
            vec![qubit],
            vec![Parameter::constant(theta)?],
        )
    }

    /// Constructs a phase gate.
    pub fn phase(qubit: QubitId, theta: f64) -> GateResult<Self> {
        Self::parameterized(
            GateKind::Phase,
            vec![qubit],
            vec![Parameter::constant(theta)?],
        )
    }

    /// Constructs a U1 gate.
    pub fn u1(qubit: QubitId, lambda: f64) -> GateResult<Self> {
        Self::parameterized(
            GateKind::U1,
            vec![qubit],
            vec![Parameter::constant(lambda)?],
        )
    }

    /// Constructs a U2 gate.
    ///
    /// Both supplied parameters are preserved exactly:
    ///
    /// `U2(phi, lambda)`
    pub fn u2(qubit: QubitId, phi: f64, lambda: f64) -> GateResult<Self> {
        Self::parameterized(
            GateKind::U2,
            vec![qubit],
            vec![
                Parameter::constant(phi)?,
                Parameter::constant(lambda)?,
            ],
        )
    }

    /// Constructs a U3 gate.
    ///
    /// All three supplied parameters are preserved exactly.
    pub fn u3(
        qubit: QubitId,
        theta: f64,
        phi: f64,
        lambda: f64,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::U3,
            vec![qubit],
            vec![
                Parameter::constant(theta)?,
                Parameter::constant(phi)?,
                Parameter::constant(lambda)?,
            ],
        )
    }

    /// Constructs a controlled-X gate.
    pub fn cx(control: QubitId, target: QubitId) -> GateResult<Self> {
        Self::two_qubit(GateKind::CX, control, target)
    }

    /// Constructs a controlled-Y gate.
    pub fn cy(control: QubitId, target: QubitId) -> GateResult<Self> {
        Self::two_qubit(GateKind::CY, control, target)
    }

    /// Constructs a controlled-Z gate.
    pub fn cz(control: QubitId, target: QubitId) -> GateResult<Self> {
        Self::two_qubit(GateKind::CZ, control, target)
    }

    /// Constructs a controlled-H gate.
    pub fn ch(control: QubitId, target: QubitId) -> GateResult<Self> {
        Self::two_qubit(GateKind::CH, control, target)
    }

    /// Constructs a SWAP gate.
    pub fn swap(left: QubitId, right: QubitId) -> GateResult<Self> {
        Self::two_qubit(GateKind::SWAP, left, right)
    }

    /// Constructs an iSWAP gate.
    pub fn iswap(left: QubitId, right: QubitId) -> GateResult<Self> {
        Self::two_qubit(GateKind::ISWAP, left, right)
    }

    /// Constructs an echoed cross-resonance gate.
    pub fn ecr(control: QubitId, target: QubitId) -> GateResult<Self> {
        Self::two_qubit(GateKind::ECR, control, target)
    }

    /// Constructs a controlled RX rotation.
    pub fn crx(
        control: QubitId,
        target: QubitId,
        theta: f64,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::CRX,
            vec![control, target],
            vec![Parameter::constant(theta)?],
        )
    }

    /// Constructs a controlled RY rotation.
    pub fn cry(
        control: QubitId,
        target: QubitId,
        theta: f64,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::CRY,
            vec![control, target],
            vec![Parameter::constant(theta)?],
        )
    }

    /// Constructs a controlled RZ rotation.
    pub fn crz(
        control: QubitId,
        target: QubitId,
        theta: f64,
    ) -> GateResult<Self> {
        Self::parameterized(
            GateKind::CRZ,
            vec![control, target],
            vec![Parameter::constant(theta)?],
        )
    }

    /// Constructs a Toffoli gate.
    pub fn ccx(
        control_a: QubitId,
        control_b: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::three_qubit(
            GateKind::CCX,
            control_a,
            control_b,
            target,
        )
    }

    /// Constructs a Fredkin gate.
    pub fn cswap(
        control: QubitId,
        target_a: QubitId,
        target_b: QubitId,
    ) -> GateResult<Self> {
        Self::three_qubit(
            GateKind::CSWAP,
            control,
            target_a,
            target_b,
        )
    }

    /// Constructs a measurement gate.
    pub fn measure(
        qubit: QubitId,
        classical_target: usize,
        measurement: Measurement,
    ) -> GateResult<Self> {
        Self::new(
            GateKind::Measure,
            vec![qubit],
            Vec::new(),
            Some(classical_target),
            Some(measurement),
        )
    }

    /// Constructs a barrier over one or more logical qubits.
    pub fn barrier(qubits: Vec<QubitId>) -> GateResult<Self> {
        Self::new(
            GateKind::Barrier,
            qubits,
            Vec::new(),
            None,
            None,
        )
    }

    /// Constructs a logical reset operation.
    pub fn reset(qubit: QubitId) -> GateResult<Self> {
        Self::simple(GateKind::Reset, qubit)
    }

    /// Creates a simple single-qubit, non-parameterized operation.
    fn simple(kind: GateKind, qubit: QubitId) -> GateResult<Self> {
        Self::new(
            kind,
            vec![qubit],
            Vec::new(),
            None,
            None,
        )
    }

    /// Creates a two-qubit, non-parameterized operation.
    fn two_qubit(
        kind: GateKind,
        first: QubitId,
        second: QubitId,
    ) -> GateResult<Self> {
        Self::new(
            kind,
            vec![first, second],
            Vec::new(),
            None,
            None,
        )
    }

    /// Creates a three-qubit, non-parameterized operation.
    fn three_qubit(
        kind: GateKind,
        first: QubitId,
        second: QubitId,
        third: QubitId,
    ) -> GateResult<Self> {
        Self::new(
            kind,
            vec![first, second, third],
            Vec::new(),
            None,
            None,
        )
    }

    /// Creates a parameterized gate.
    fn parameterized(
        kind: GateKind,
        qubits: Vec<QubitId>,
        parameters: Vec<Parameter>,
    ) -> GateResult<Self> {
        Self::new(
            kind,
            qubits,
            parameters,
            None,
            None,
        )
    }
}

impl TryFrom<Gate> for IrResult<Gate> {
    type Error = IrError;

    fn try_from(gate: Gate) -> Result<Self, Self::Error> {
        gate.validate()?;
        Ok(Ok(gate))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    #[test]
    fn parameter_counts_are_exact() {
        assert_eq!(GateKind::RX.parameter_count(), 1);
        assert_eq!(GateKind::U2.parameter_count(), 2);
        assert_eq!(GateKind::U3.parameter_count(), 3);
        assert_eq!(GateKind::X.parameter_count(), 0);
    }

    #[test]
    fn u2_preserves_both_parameters() {
        let gate = Gate::u2(q(0), 1.0, 2.0).unwrap();

        assert_eq!(gate.parameters().len(), 2);

        assert_eq!(
            gate.parameters()[0],
            Parameter::Constant(1.0)
        );

        assert_eq!(
            gate.parameters()[1],
            Parameter::Constant(2.0)
        );
    }

    #[test]
    fn u3_preserves_all_parameters() {
        let gate = Gate::u3(q(0), 1.0, 2.0, 3.0).unwrap();

        assert_eq!(gate.parameters().len(), 3);

        assert_eq!(
            gate.parameters()[0],
            Parameter::Constant(1.0)
        );

        assert_eq!(
            gate.parameters()[1],
            Parameter::Constant(2.0)
        );

        assert_eq!(
            gate.parameters()[2],
            Parameter::Constant(3.0)
        );
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let result = Gate::cx(q(0), q(0));

        assert!(matches!(
            result,
            Err(GateError::DuplicateQubit { .. })
        ));
    }

    #[test]
    fn invalid_parameter_count_is_rejected() {
        let result = Gate::new(
            GateKind::RX,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        );

        assert!(matches!(
            result,
            Err(GateError::InvalidParameterCount {
                expected: 1,
                actual: 0,
                ..
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
            Err(GateError::UnexpectedClassicalTarget {
                gate: GateKind::X
            })
        ));
    }

    #[test]
    fn measurement_requires_classical_target() {
        let result = Gate::new(
            GateKind::Measure,
            vec![q(0)],
            Vec::new(),
            None,
            None,
        );

        assert!(matches!(
            result,
            Err(GateError::MissingClassicalTarget)
        ));
    }

    #[test]
    fn empty_barrier_is_rejected() {
        let result = Gate::barrier(Vec::new());

        assert!(matches!(
            result,
            Err(GateError::EmptyBarrier)
        ));
    }

    #[test]
    fn reset_requires_one_qubit() {
        let result = Gate::new(
            GateKind::Reset,
            vec![q(0), q(1)],
            Vec::new(),
            None,
            None,
        );

        assert!(matches!(
            result,
            Err(GateError::InvalidResetOperandCount { actual: 2 })
        ));
    }

    #[test]
    fn namespace_validation_rejects_unknown_qubits() {
        let gate = Gate::cx(q(0), q(2)).unwrap();

        let result = gate.validate_in_namespace(2);

        assert!(matches!(
            result,
            Err(GateError::UnknownQubit { .. })
        ));
    }

    #[test]
    fn namespace_validation_accepts_valid_qubits() {
        let gate = Gate::cx(q(0), q(1)).unwrap();

        assert!(gate.validate_in_namespace(2).is_ok());
    }

    #[test]
    fn self_inverse_classification_is_deterministic() {
        assert!(GateKind::X.is_self_inverse());
        assert!(GateKind::H.is_self_inverse());
        assert!(GateKind::CX.is_self_inverse());
        assert!(!GateKind::T.is_self_inverse());
    }

    #[test]
    fn clifford_classification_does_not_include_toffoli_or_fredkin() {
        assert!(!GateKind::CCX.is_clifford());
        assert!(!GateKind::CSWAP.is_clifford());
    }

    #[test]
    fn non_unitary_operations_are_not_unitary() {
        assert!(!GateKind::Measure.is_unitary());
        assert!(!GateKind::Barrier.is_unitary());
        assert!(!GateKind::Reset.is_unitary());
    }
}