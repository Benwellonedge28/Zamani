//! Zamani Quantum IR — Canonical Standard Gate Semantics
//!
//! This module defines the canonical, hardware-independent representation of
//! standard logical quantum gates.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! This file answers:
//!
//!     "What standard logical gate does the program request?"
//!
//! It does NOT answer:
//!
//! - where the gate executes;
//! - which physical qubits execute it;
//! - whether physical qubits are connected;
//! - which native instruction implements it;
//! - which pulse implements it;
//! - which calibration is required;
//! - which control channel is used;
//! - when it executes;
//! - how it is routed;
//! - how it is optimized;
//! - how it is scheduled;
//! - how QEC implements it;
//! - how a backend executes it.
//!
//! Those responsibilities belong to:
//!
//!     mapping / routing
//!     scheduling
//!     pulse
//!     hardware
//!     optimization
//!     QEC
//!     backend
//!
//! ============================================================================
//! UNIVERSAL-PROGRAM PRINCIPLE
//! ============================================================================
//!
//! A Zamani program is written once at the semantic level.
//!
//! The same gate representation may therefore eventually be lowered to:
//!
//! - one-qubit machines;
//! - small QPUs;
//! - large QPUs;
//! - distributed quantum systems;
//! - logical/fault-tolerant machines;
//! - simulators;
//! - future quantum architectures.
//!
//! This file contains NO architectural maximum qubit count.
//!
//! `usize` values appearing in this file are container/namespace values or
//! explicit validation-policy values. They are not machine-size limits.
//!
//! ============================================================================
//! STANDARD GATE VS UNIVERSAL OPERATION
//! ============================================================================
//!
//! `GateKind` is the canonical STANDARD GATE vocabulary.
//!
//! It is intentionally finite because a standard dialect must have a stable
//! vocabulary.
//!
//! It is NOT the universe of every operation Zamani can represent.
//!
//! Future/vendor/custom operations must use the extensible operation/dialect
//! layer:
//!
//!     quantum::ir::operation
//!     quantum::ir::dialect
//!     quantum::ir::extension
//!
//! This prevents a new quantum architecture from forcing this file to change.
//!
//! ============================================================================
//! IDENTITY
//! ============================================================================
//!
//! Logical gate operands MUST use:
//!
//!     quantum::ir::qubit::QubitId
//!
//! Physical qubits are deliberately excluded from `Gate`.
//!
//! Physical placement belongs to mapping/routing.
//!
//! ============================================================================
//! PARAMETERS
//! ============================================================================
//!
//! Gate parameters use the canonical:
//!
//!     quantum::ir::parameter::Parameter
//!
//! Parameters may be:
//!
//! - constants;
//! - symbols;
//! - symbolic arithmetic expressions.
//!
//! This file never interprets a generic parameter as hardware-specific data.
//!
//! ============================================================================
//! MEASUREMENT
//! ============================================================================
//!
//! Measurement semantics are owned by:
//!
//!     quantum::ir::measurement
//!
//! `Gate` only attaches the canonical measurement object when the standard
//! `Measure` operation is used.
//!
//! ============================================================================
//! VALIDATION BOUNDARY
//! ============================================================================
//!
//! This module validates invariants that can be determined from one gate:
//!
//! - gate kind;
//! - operand arity;
//! - operand uniqueness;
//! - parameter arity;
//! - parameter validity;
//! - measurement consistency;
//! - classical-target consistency;
//! - barrier non-emptiness;
//! - reset arity.
//!
//! Program-wide validation belongs to `validation.rs`.
//!
//! Hardware capability validation belongs to `hardware`/target validation.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This module contains no fixed machine-size assumptions such as:
//!
//!     8
//!     32
//!     64
//!     127
//!     256
//!     1024
//!     4096
//!
//! A gate may reference any representable logical `QubitId`.
//!
//! Explicit `QuantumIrLimits` are policy/security boundaries, not semantic
//! quantum-machine limits.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Gate operand order is preserved because operand order is semantically
//! meaningful for non-symmetric gates such as CX(control, target).
//!
//! No method in this file silently sorts or rewrites operands.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::limits::QuantumIrLimits;
use crate::quantum::ir::measurement::Measurement;
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// RESULT
// ============================================================================

/// Result type returned by gate construction and local validation.
pub type GateResult<T> = Result<T, GateError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced while constructing or validating a standard logical gate.
///
/// These errors intentionally remain local to the gate layer. Higher-level
/// IR validation may translate them into the canonical `IrError` model.
#[derive(Debug, Clone, PartialEq)]
pub enum GateError {
    /// The number of logical operands does not satisfy the gate contract.
    InvalidOperandCount {
        gate: GateKind,
        expected: OperandCount,
        actual: usize,
    },

    /// Two or more operands reference the same logical qubit.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// The number of parameters does not satisfy the gate contract.
    InvalidParameterCount {
        gate: GateKind,
        expected: usize,
        actual: usize,
    },

    /// A parameter failed canonical parameter validation.
    InvalidParameter {
        index: usize,
        parameter: Parameter,
    },

    /// A measurement gate has no measurement payload.
    MissingMeasurement,

    /// A non-measurement gate contains a measurement payload.
    UnexpectedMeasurement {
        gate: GateKind,
    },

    /// The gate-level classical destination and measurement destination differ.
    MeasurementTargetMismatch {
        gate_target: usize,
        measurement_target: usize,
    },

    /// A measurement gate has no classical destination.
    MissingClassicalTarget,

    /// A non-measurement gate contains a classical destination.
    UnexpectedClassicalTarget {
        gate: GateKind,
        target: usize,
    },

    /// A barrier contains no operands.
    EmptyBarrier,

    /// Reset does not have exactly one logical operand.
    InvalidResetOperandCount {
        actual: usize,
    },

    /// A logical operand is outside the supplied logical namespace.
    UnknownQubit {
        qubit: QubitId,
        logical_qubits: usize,
    },

    /// A classical destination is outside the supplied classical namespace.
    UnknownClassicalBit {
        bit: usize,
        classical_bits: usize,
    },

    /// A gate exceeds an explicit IR policy.
    ///
    /// This is a policy failure, not a semantic gate failure.
    LimitExceeded {
        resource: &'static str,
        limit: usize,
        actual: usize,
    },

    /// The representation is internally inconsistent.
    InvalidStructure {
        message: &'static str,
    },
}

impl fmt::Display for GateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperandCount {
                gate,
                expected,
                actual,
            } => write!(
                formatter,
                "gate {gate:?} requires {expected} operand(s), received {actual}"
            ),

            Self::DuplicateQubit { qubit } => {
                write!(
                    formatter,
                    "gate contains duplicate logical qubit {qubit}"
                )
            }

            Self::InvalidParameterCount {
                gate,
                expected,
                actual,
            } => write!(
                formatter,
                "gate {gate:?} requires {expected} parameter(s), received {actual}"
            ),

            Self::InvalidParameter {
                index,
                parameter,
            } => write!(
                formatter,
                "invalid gate parameter at index {index}: {parameter:?}"
            ),

            Self::MissingMeasurement => {
                formatter.write_str(
                    "measurement gate requires a measurement payload",
                )
            }

            Self::UnexpectedMeasurement { gate } => write!(
                formatter,
                "gate {gate:?} cannot contain a measurement payload"
            ),

            Self::MeasurementTargetMismatch {
                gate_target,
                measurement_target,
            } => write!(
                formatter,
                "measurement classical target mismatch: gate target c{gate_target}, \
                 measurement target c{measurement_target}"
            ),

            Self::MissingClassicalTarget => {
                formatter.write_str(
                    "measurement gate requires a classical destination",
                )
            }

            Self::UnexpectedClassicalTarget { gate, target } => write!(
                formatter,
                "non-measurement gate {gate:?} cannot contain classical target c{target}"
            ),

            Self::EmptyBarrier => {
                formatter.write_str(
                    "barrier requires at least one logical qubit",
                )
            }

            Self::InvalidResetOperandCount { actual } => write!(
                formatter,
                "reset requires exactly one logical qubit, received {actual}"
            ),

            Self::UnknownQubit {
                qubit,
                logical_qubits,
            } => write!(
                formatter,
                "logical qubit {qubit} is outside logical namespace 0..{logical_qubits}"
            ),

            Self::UnknownClassicalBit {
                bit,
                classical_bits,
            } => write!(
                formatter,
                "classical bit c{bit} is outside classical namespace 0..{classical_bits}"
            ),

            Self::LimitExceeded {
                resource,
                limit,
                actual,
            } => write!(
                formatter,
                "gate exceeds {resource} limit: maximum {limit}, actual {actual}"
            ),

            Self::InvalidStructure { message } => {
                write!(
                    formatter,
                    "invalid gate structure: {message}"
                )
            }
        }
    }
}

impl std::error::Error for GateError {}

// ============================================================================
// OPERAND CARDINALITY
// ============================================================================

/// Describes how many logical qubits a standard gate accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperandCount {
    /// Exactly `n` logical operands.
    Exact(usize),

    /// At least `n` logical operands.
    AtLeast(usize),
}

impl OperandCount {
    /// Returns whether the supplied operand count satisfies this contract.
    #[must_use]
    pub const fn accepts(self, actual: usize) -> bool {
        match self {
            Self::Exact(expected) => actual == expected,
            Self::AtLeast(minimum) => actual >= minimum,
        }
    }
}

impl fmt::Display for OperandCount {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Exact(value) => write!(formatter, "{value}"),
            Self::AtLeast(value) => {
                write!(formatter, "at least {value}")
            }
        }
    }
}

// ============================================================================
// STANDARD GATE KIND
// ============================================================================

/// Canonical standard logical gate vocabulary.
///
/// This enum represents the stable standard-gate dialect.
///
/// It intentionally does not represent every possible future quantum
/// operation. Custom, vendor, analog, pulse, logical, distributed and other
/// operations belong to the extensible operation/dialect layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateKind {
    // ------------------------------------------------------------------------
    // Single-qubit fixed gates
    // ------------------------------------------------------------------------

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

    /// S = phase(pi / 2).
    S,

    /// S dagger.
    Sdg,

    /// T = phase(pi / 4).
    T,

    /// T dagger.
    Tdg,

    /// Square-root-X.
    V,

    /// Square-root-X dagger.
    Vdg,

    // ------------------------------------------------------------------------
    // Single-qubit parameterized gates
    // ------------------------------------------------------------------------

    /// Rotation around X.
    RX,

    /// Rotation around Y.
    RY,

    /// Rotation around Z.
    RZ,

    /// Generic phase rotation.
    Phase,

    /// U1(lambda).
    U1,

    /// U2(phi, lambda).
    U2,

    /// U3(theta, phi, lambda).
    U3,

    // ------------------------------------------------------------------------
    // Two-qubit fixed gates
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // Two-qubit parameterized gates
    // ------------------------------------------------------------------------

    /// Controlled-RX(theta).
    CRX,

    /// Controlled-RY(theta).
    CRY,

    /// Controlled-RZ(theta).
    CRZ,

    // ------------------------------------------------------------------------
    // Three-qubit fixed gates
    // ------------------------------------------------------------------------

    /// Toffoli / controlled-controlled-X.
    CCX,

    /// Fredkin / controlled-SWAP.
    CSWAP,

    // ------------------------------------------------------------------------
    // Semantic non-unitary / structural operations
    // ------------------------------------------------------------------------

    /// Logical measurement.
    Measure,

    /// Synchronization barrier.
    Barrier,

    /// Logical reset.
    Reset,
}

impl GateKind {
    /// Returns the number of logical qubits required by this standard gate.
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

    /// Returns whether this standard gate requires parameters.
    #[must_use]
    pub const fn is_parameterized(self) -> bool {
        self.parameter_count() != 0
    }

    /// Returns whether the operation changes quantum state through a unitary
    /// transformation.
    ///
    /// Barriers are deliberately excluded from this method because a barrier
    /// is a synchronization marker, not a quantum transformation.
    #[must_use]
    pub const fn is_unitary(self) -> bool {
        !matches!(
            self,
            Self::Measure | Self::Reset | Self::Barrier
        )
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

    /// Returns whether this operation requires a classical destination.
    #[must_use]
    pub const fn requires_classical_target(self) -> bool {
        matches!(self, Self::Measure)
    }

    /// Returns whether this operation accepts a classical destination.
    #[must_use]
    pub const fn permits_classical_target(self) -> bool {
        self.requires_classical_target()
    }

    /// Returns whether this operation is self-inverse independently of
    /// parameter values.
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

    /// Returns whether this gate is Clifford for all allowed parameter values.
    ///
    /// Parameterized rotations are intentionally excluded because only
    /// particular parameter values are Clifford.
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

    /// Returns whether this operation has no quantum-state effect.
    #[must_use]
    pub const fn is_semantic_marker(self) -> bool {
        matches!(self, Self::Barrier)
    }

    /// Returns whether this operation is non-unitary.
    #[must_use]
    pub const fn is_non_unitary(self) -> bool {
        matches!(self, Self::Measure | Self::Reset)
    }
}

// ============================================================================
// GATE
// ============================================================================

/// Canonical hardware-independent standard logical gate.
///
/// All fields are private. A valid `Gate` therefore cannot be mutated into an
/// invalid representation through ordinary field access.
///
/// Physical information is intentionally absent.
///
/// A `Gate` contains:
///
/// - standard semantic gate kind;
/// - logical qubit operands;
/// - symbolic/numeric parameters;
/// - optional logical classical destination;
/// - optional canonical measurement semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct Gate {
    kind: GateKind,
    qubits: Vec<QubitId>,
    parameters: Vec<Parameter>,
    classical_target: Option<usize>,
    measurement: Option<Measurement>,
}

impl Gate {
    // ========================================================================
    // GENERAL CONSTRUCTION
    // ========================================================================

    /// Constructs a complete gate after validating all local invariants.
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

    /// Constructs a gate without parameters or classical/measurement data.
    pub fn simple(
        kind: GateKind,
        qubits: Vec<QubitId>,
    ) -> GateResult<Self> {
        Self::new(
            kind,
            qubits,
            Vec::new(),
            None,
            None,
        )
    }

    /// Constructs a parameterized standard gate.
    pub fn parameterized(
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

    /// Constructs a measurement gate from the canonical measurement object.
    pub fn from_measurement(
        measurement: Measurement,
    ) -> GateResult<Self> {
        let qubit = measurement.qubit();
        let classical_target =
            measurement.classical_bit().index();

        Self::new(
            GateKind::Measure,
            vec![qubit],
            Vec::new(),
            Some(classical_target),
            Some(measurement),
        )
    }

    // ========================================================================
    // ACCESSORS
    // ========================================================================

    /// Returns the standard semantic gate kind.
    #[must_use]
    pub const fn kind(&self) -> GateKind {
        self.kind
    }

    /// Returns logical qubit operands in semantic/program order.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Returns gate parameters in semantic parameter order.
    #[must_use]
    pub fn parameters(&self) -> &[Parameter] {
        &self.parameters
    }

    /// Returns the logical classical destination, if present.
    #[must_use]
    pub const fn classical_target(&self) -> Option<usize> {
        self.classical_target
    }

    /// Returns the canonical measurement semantics, if present.
    #[must_use]
    pub fn measurement(&self) -> Option<&Measurement> {
        self.measurement.as_ref()
    }

    /// Returns the first logical qubit operand, when one exists.
    #[must_use]
    pub fn qubit(&self) -> Option<QubitId> {
        self.qubits.first().copied()
    }

    /// Returns the number of logical qubit operands.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Returns the number of supplied parameters.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Returns whether the standard gate is parameterized.
    #[must_use]
    pub fn is_parameterized(&self) -> bool {
        self.kind.is_parameterized()
    }

    /// Returns whether this gate is a unitary state transformation.
    #[must_use]
    pub fn is_unitary(&self) -> bool {
        self.kind.is_unitary()
    }

    /// Returns whether this gate is a measurement.
    #[must_use]
    pub fn is_measurement(&self) -> bool {
        self.kind.is_measurement()
    }

    /// Returns whether this gate is a synchronization barrier.
    #[must_use]
    pub fn is_barrier(&self) -> bool {
        self.kind.is_barrier()
    }

    /// Returns whether this gate is a reset.
    #[must_use]
    pub fn is_reset(&self) -> bool {
        self.kind.is_reset()
    }

    /// Returns whether this gate is non-unitary.
    #[must_use]
    pub fn is_non_unitary(&self) -> bool {
        self.kind.is_non_unitary()
    }

    /// Returns whether this gate is a semantic marker.
    #[must_use]
    pub fn is_semantic_marker(&self) -> bool {
        self.kind.is_semantic_marker()
    }

    /// Returns whether all parameters are direct finite constants.
    #[must_use]
    pub fn parameters_are_all_constants(&self) -> bool {
        self.parameters
            .iter()
            .all(Parameter::is_constant)
    }

    /// Returns direct constant parameter values when every parameter is a
    /// direct constant.
    ///
    /// Returns `None` for symbolic or expression parameters.
    #[must_use]
    pub fn constant_parameters(&self) -> Option<Vec<f64>> {
        let mut values =
            Vec::with_capacity(self.parameters.len());

        for parameter in &self.parameters {
            match parameter.as_constant() {
                Some(value) => values.push(value),
                None => return None,
            }
        }

        Some(values)
    }

    // ========================================================================
    // VALIDATION
    // ========================================================================

    /// Validates all invariants that can be established from this gate alone.
    pub fn validate(&self) -> GateResult<()> {
        self.validate_operands()?;
        self.validate_parameters()?;
        self.validate_measurement_contract()?;
        self.validate_classical_contract()?;

        Ok(())
    }

    /// Validates only the logical operand contract.
    pub fn validate_operands(&self) -> GateResult<()> {
        let expected =
            self.kind.operand_count();

        if !expected.accepts(self.qubits.len()) {
            if self.kind.is_barrier() {
                return Err(
                    GateError::EmptyBarrier,
                );
            }

            if self.kind.is_reset() {
                return Err(
                    GateError::InvalidResetOperandCount {
                        actual: self.qubits.len(),
                    },
                );
            }

            return Err(
                GateError::InvalidOperandCount {
                    gate: self.kind,
                    expected,
                    actual: self.qubits.len(),
                },
            );
        }

        self.validate_distinct_operands()
    }

    /// Validates gate parameter arity and parameter semantics.
    pub fn validate_parameters(&self) -> GateResult<()> {
        let expected =
            self.kind.parameter_count();

        let actual =
            self.parameters.len();

        if actual != expected {
            return Err(
                GateError::InvalidParameterCount {
                    gate: self.kind,
                    expected,
                    actual,
                },
            );
        }

        for (index, parameter) in
            self.parameters.iter().enumerate()
        {
            if parameter.validate().is_err() {
                return Err(
                    GateError::InvalidParameter {
                        index,
                        parameter: parameter.clone(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Validates measurement-specific invariants.
    pub fn validate_measurement_contract(
        &self,
    ) -> GateResult<()> {
        if self.kind.requires_classical_target() {
            let measurement =
                self.measurement.as_ref().ok_or(
                    GateError::MissingMeasurement,
                )?;

            let target =
                self.classical_target.ok_or(
                    GateError::MissingClassicalTarget,
                )?;

            let measurement_target =
                measurement.classical_bit().index();

            if target != measurement_target {
                return Err(
                    GateError::MeasurementTargetMismatch {
                        gate_target: target,
                        measurement_target,
                    },
                );
            }

            let qubit =
                self.qubits.first().copied().ok_or(
                    GateError::InvalidStructure {
                        message:
                            "measurement gate has no qubit operand",
                    },
                )?;

            if measurement.qubit() != qubit {
                return Err(
                    GateError::InvalidStructure {
                        message:
                            "measurement payload qubit does not match gate operand",
                    },
                );
            }

            return Ok(());
        }

        if self.measurement.is_some() {
            return Err(
                GateError::UnexpectedMeasurement {
                    gate: self.kind,
                },
            );
        }

        Ok(())
    }

    /// Validates classical-target invariants.
    pub fn validate_classical_contract(
        &self,
    ) -> GateResult<()> {
        if self.kind.permits_classical_target() {
            return Ok(());
        }

        if let Some(target) =
            self.classical_target
        {
            return Err(
                GateError::UnexpectedClassicalTarget {
                    gate: self.kind,
                    target,
                },
            );
        }

        Ok(())
    }

    /// Validates that every logical operand is distinct.
    ///
    /// A `BTreeSet` is used rather than the previous O(n²) pairwise scan.
    ///
    /// This matters for scalable variadic operations and prevents the local
    /// validation cost from growing quadratically with operand count.
    fn validate_distinct_operands(
        &self,
    ) -> GateResult<()> {
        let mut seen =
            BTreeSet::new();

        for &qubit in &self.qubits {
            if !seen.insert(qubit) {
                return Err(
                    GateError::DuplicateQubit {
                        qubit,
                    },
                );
            }
        }

        Ok(())
    }

    // ========================================================================
    // POLICY / NAMESPACE VALIDATION
    // ========================================================================

    /// Validates this gate against explicit IR resource limits.
    ///
    /// These limits are compilation/security policy values and do not define
    /// the maximum quantum-machine size supported by Zamani.
    pub fn validate_with_limits(
        &self,
        limits: &QuantumIrLimits,
    ) -> GateResult<()> {
        self.validate()?;

        let operands =
            self.qubits.len();

        if operands >
            limits.max_operands()
        {
            return Err(
                GateError::LimitExceeded {
                    resource:
                        "gate operands",
                    limit:
                        limits.max_operands(),
                    actual:
                        operands,
                },
            );
        }

        let parameters =
            self.parameters.len();

        if parameters >
            limits.max_parameters()
        {
            return Err(
                GateError::LimitExceeded {
                    resource:
                        "gate parameters",
                    limit:
                        limits.max_parameters(),
                    actual:
                        parameters,
                },
            );
        }

        Ok(())
    }

    /// Validates all logical qubit operands against an explicit logical
    /// namespace size.
    ///
    /// `logical_qubits` is a program namespace size, not hardware capacity.
    pub fn validate_in_namespace(
        &self,
        logical_qubits: usize,
    ) -> GateResult<()> {
        self.validate()?;

        for &qubit in &self.qubits {
            if qubit.index() >= logical_qubits {
                return Err(
                    GateError::UnknownQubit {
                        qubit,
                        logical_qubits,
                    },
                );
            }
        }

        Ok(())
    }

    /// Validates the classical destination against an explicit logical
    /// classical namespace.
    pub fn validate_classical_namespace(
        &self,
        classical_bits: usize,
    ) -> GateResult<()> {
        self.validate()?;

        if let Some(bit) =
            self.classical_target
        {
            if bit >= classical_bits {
                return Err(
                    GateError::UnknownClassicalBit {
                        bit,
                        classical_bits,
                    },
                );
            }
        }

        Ok(())
    }

    /// Performs local, policy and namespace validation.
    pub fn validate_complete(
        &self,
        limits: &QuantumIrLimits,
        logical_qubits: usize,
        classical_bits: usize,
    ) -> GateResult<()> {
        self.validate_with_limits(limits)?;
        self.validate_in_namespace(
            logical_qubits,
        )?;
        self.validate_classical_namespace(
            classical_bits,
        )?;

        Ok(())
    }

    // ========================================================================
    // STANDARD FIXED SINGLE-QUBIT GATES
    // ========================================================================

    /// Creates an identity gate.
    pub fn identity(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::I,
            vec![qubit],
        )
    }

    /// Creates an X gate.
    pub fn x(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::X,
            vec![qubit],
        )
    }

    /// Creates a Y gate.
    pub fn y(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::Y,
            vec![qubit],
        )
    }

    /// Creates a Z gate.
    pub fn z(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::Z,
            vec![qubit],
        )
    }

    /// Creates a Hadamard gate.
    pub fn h(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::H,
            vec![qubit],
        )
    }

    /// Creates an S gate.
    pub fn s(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::S,
            vec![qubit],
        )
    }

    /// Creates an S-dagger gate.
    pub fn sdg(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::Sdg,
            vec![qubit],
        )
    }

    /// Creates a T gate.
    pub fn t(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::T,
            vec![qubit],
        )
    }

    /// Creates a T-dagger gate.
    pub fn tdg(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::Tdg,
            vec![qubit],
        )
    }

    /// Creates a square-root-X gate.
    pub fn v(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::V,
            vec![qubit],
        )
    }

    /// Creates a square-root-X-dagger gate.
    pub fn vdg(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::Vdg,
            vec![qubit],
        )
    }

    // ========================================================================
    // PARAMETERIZED SINGLE-QUBIT GATES
    // ========================================================================

    /// Creates RX(theta).
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

    /// Creates RY(theta).
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

    /// Creates RZ(theta).
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

    /// Creates Phase(theta).
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

    /// Creates U1(lambda).
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

    /// Creates U2(phi, lambda).
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

    /// Creates U3(theta, phi, lambda).
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

    // ========================================================================
    // FIXED TWO-QUBIT GATES
    // ========================================================================

    /// Creates CX(control, target).
    pub fn cx(
        control: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CX,
            vec![control, target],
        )
    }

    /// Creates CY(control, target).
    pub fn cy(
        control: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CY,
            vec![control, target],
        )
    }

    /// Creates CZ(control, target).
    pub fn cz(
        control: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CZ,
            vec![control, target],
        )
    }

    /// Creates CH(control, target).
    pub fn ch(
        control: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CH,
            vec![control, target],
        )
    }

    /// Creates SWAP(first, second).
    pub fn swap(
        first: QubitId,
        second: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::SWAP,
            vec![first, second],
        )
    }

    /// Creates iSWAP(first, second).
    pub fn iswap(
        first: QubitId,
        second: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::ISWAP,
            vec![first, second],
        )
    }

    /// Creates ECR(control, target).
    pub fn ecr(
        control: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::ECR,
            vec![control, target],
        )
    }

    // ========================================================================
    // PARAMETERIZED TWO-QUBIT GATES
    // ========================================================================

    /// Creates CRX(control, target, theta).
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

    /// Creates CRY(control, target, theta).
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

    /// Creates CRZ(control, target, theta).
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

    // ========================================================================
    // THREE-QUBIT GATES
    // ========================================================================

    /// Creates CCX(control_a, control_b, target).
    pub fn ccx(
        control_a: QubitId,
        control_b: QubitId,
        target: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CCX,
            vec![
                control_a,
                control_b,
                target,
            ],
        )
    }

    /// Creates CSWAP(control, target_a, target_b).
    pub fn cswap(
        control: QubitId,
        target_a: QubitId,
        target_b: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::CSWAP,
            vec![
                control,
                target_a,
                target_b,
            ],
        )
    }

    // ========================================================================
    // MEASUREMENT / RESET / BARRIER
    // ========================================================================

    /// Creates a measurement gate from canonical measurement semantics.
    pub fn measure(
        measurement: Measurement,
    ) -> GateResult<Self> {
        Self::from_measurement(
            measurement,
        )
    }

    /// Creates a measurement of `qubit` into logical classical bit `c`.
    pub fn measure_to(
        qubit: QubitId,
        classical_bit: usize,
    ) -> GateResult<Self> {
        let measurement =
            Measurement::new(
                qubit,
                crate::quantum::ir::measurement::ClassicalBitId::new(
                    classical_bit,
                ),
            );

        Self::from_measurement(
            measurement,
        )
    }

    /// Creates a barrier over one or more logical qubits.
    pub fn barrier(
        qubits: Vec<QubitId>,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::Barrier,
            qubits,
        )
    }

    /// Creates a single-qubit logical reset.
    pub fn reset(
        qubit: QubitId,
    ) -> GateResult<Self> {
        Self::simple(
            GateKind::Reset,
            vec![qubit],
        )
    }

    // ========================================================================
    // SEMANTIC ANALYSIS HELPERS
    // ========================================================================

    /// Returns whether this gate operates on `qubit`.
    #[must_use]
    pub fn touches(
        &self,
        qubit: QubitId,
    ) -> bool {
        self.qubits.contains(&qubit)
    }

    /// Returns whether this gate and `other` share at least one logical qubit.
    #[must_use]
    pub fn overlaps(
        &self,
        other: &Self,
    ) -> bool {
        // Iterate over the smaller collection to reduce work for large
        // variadic/standardized gate representations.
        let (smaller, larger) =
            if self.qubits.len()
                <= other.qubits.len()
            {
                (&self.qubits, &other.qubits)
            } else {
                (&other.qubits, &self.qubits)
            };

        smaller
            .iter()
            .any(|qubit| {
                larger.contains(qubit)
            })
    }

    /// Returns whether the two gates have disjoint logical operands.
    ///
    /// This is NOT a commutation proof.
    ///
    /// It only proves that the gates do not reference the same logical qubit.
    #[must_use]
    pub fn is_operand_disjoint(
        &self,
        other: &Self,
    ) -> bool {
        !self.overlaps(other)
    }

    /// Returns a compact deterministic semantic summary.
    #[must_use]
    pub fn semantic_summary(
        &self,
    ) -> GateSummary {
        GateSummary {
            kind: self.kind,
            operand_count:
                self.qubits.len(),
            parameter_count:
                self.parameters.len(),
            has_classical_target:
                self.classical_target.is_some(),
            has_measurement:
                self.measurement.is_some(),
        }
    }
}

// ============================================================================
// GATE SUMMARY
// ============================================================================

/// Allocation-free summary of standard gate semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateSummary {
    /// Standard gate kind.
    pub kind: GateKind,

    /// Number of logical qubit operands.
    pub operand_count: usize,

    /// Number of parameters.
    pub parameter_count: usize,

    /// Whether a logical classical destination exists.
    pub has_classical_target: bool,

    /// Whether canonical measurement semantics are attached.
    pub has_measurement: bool,
}

// ============================================================================
// TESTS
// ============================================================================

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
        let gate =
            Gate::x(q(0))
                .expect("X gate should be valid");

        assert_eq!(
            gate.kind(),
            GateKind::X
        );

        assert_eq!(
            gate.qubits(),
            &[q(0)]
        );

        assert!(gate.is_unitary());
        assert!(!gate.is_parameterized());
    }

    #[test]
    fn barrier_is_not_a_unitary_gate() {
        assert!(
            !GateKind::Barrier.is_unitary()
        );

        assert!(
            GateKind::Barrier.is_semantic_marker()
        );
    }

    #[test]
    fn measurement_is_non_unitary() {
        assert!(
            GateKind::Measure.is_non_unitary()
        );

        assert!(
            !GateKind::Measure.is_unitary()
        );
    }

    #[test]
    fn reset_is_non_unitary() {
        assert!(
            GateKind::Reset.is_non_unitary()
        );

        assert!(
            !GateKind::Reset.is_unitary()
        );
    }

    #[test]
    fn h_gate_is_clifford_and_self_inverse() {
        assert!(
            GateKind::H.is_clifford()
        );

        assert!(
            GateKind::H.is_self_inverse()
        );
    }

    #[test]
    fn parameterized_rx_is_valid() {
        let gate =
            Gate::rx(
                q(0),
                p(0.5),
            )
            .expect(
                "RX should be valid",
            );

        assert_eq!(
            gate.kind(),
            GateKind::RX
        );

        assert_eq!(
            gate.parameter_count(),
            1
        );

        assert!(
            gate.is_parameterized()
        );
    }

    #[test]
    fn symbolic_parameter_is_accepted() {
        let parameter =
            Parameter::symbol(
                "theta",
            )
            .expect(
                "symbol should be valid",
            );

        let gate =
            Gate::rx(
                q(7),
                parameter,
            )
            .expect(
                "symbolic RX should be valid",
            );

        assert!(
            gate.is_parameterized()
        );

        assert!(
            !gate.parameters_are_all_constants()
        );
    }

    #[test]
    fn cx_requires_two_distinct_qubits() {
        assert!(
            Gate::cx(
                q(0),
                q(1)
            )
            .is_ok()
        );

        assert!(matches!(
            Gate::cx(
                q(0),
                q(0)
            ),
            Err(
                GateError::DuplicateQubit { .. }
            )
        ));
    }

    #[test]
    fn ccx_requires_three_distinct_qubits() {
        assert!(
            Gate::ccx(
                q(0),
                q(1),
                q(2)
            )
            .is_ok()
        );

        assert!(matches!(
            Gate::ccx(
                q(0),
                q(1),
                q(1)
            ),
            Err(
                GateError::DuplicateQubit { .. }
            )
        ));
    }

    #[test]
    fn barrier_requires_at_least_one_qubit() {
        assert!(matches!(
            Gate::barrier(
                Vec::new()
            ),
            Err(
                GateError::EmptyBarrier
            )
        ));

        assert!(
            Gate::barrier(
                vec![q(0), q(1)]
            )
            .is_ok()
        );
    }

    #[test]
    fn reset_requires_exactly_one_qubit() {
        assert!(
            Gate::reset(q(0))
                .is_ok()
        );

        let result =
            Gate::simple(
                GateKind::Reset,
                vec![
                    q(0),
                    q(1),
                ],
            );

        assert!(matches!(
            result,
            Err(
                GateError::InvalidResetOperandCount {
                    actual: 2
                }
            )
        ));
    }

    #[test]
    fn measurement_has_classical_target() {
        let gate =
            Gate::measure_to(
                q(0),
                3,
            )
            .expect(
                "measurement should be valid",
            );

        assert!(
            gate.is_measurement()
        );

        assert_eq!(
            gate.classical_target(),
            Some(3)
        );

        assert!(
            gate.measurement()
                .is_some()
        );
    }

    #[test]
    fn measurement_payload_and_gate_target_must_agree() {
        let measurement =
            Measurement::new(
                q(0),
                crate::quantum::ir::measurement::ClassicalBitId::new(3),
            );

        let gate =
            Gate::new(
                GateKind::Measure,
                vec![q(0)],
                Vec::new(),
                Some(4),
                Some(measurement),
            );

        assert!(matches!(
            gate,
            Err(
                GateError::MeasurementTargetMismatch {
                    gate_target: 4,
                    measurement_target: 3
                }
            )
        ));
    }

    #[test]
    fn non_measurement_cannot_have_classical_target() {
        let result =
            Gate::new(
                GateKind::X,
                vec![q(0)],
                Vec::new(),
                Some(0),
                None,
            );

        assert!(matches!(
            result,
            Err(
                GateError::UnexpectedClassicalTarget { .. }
            )
        ));
    }

    #[test]
    fn non_measurement_cannot_have_measurement_payload() {
        let measurement =
            Measurement::new(
                q(0),
                crate::quantum::ir::measurement::ClassicalBitId::new(0),
            );

        let result =
            Gate::new(
                GateKind::X,
                vec![q(0)],
                Vec::new(),
                None,
                Some(measurement),
            );

        assert!(matches!(
            result,
            Err(
                GateError::UnexpectedMeasurement {
                    gate: GateKind::X
                }
            )
        ));
    }

    #[test]
    fn wrong_parameter_count_is_rejected() {
        let result =
            Gate::parameterized(
                GateKind::U3,
                vec![q(0)],
                vec![
                    p(0.1),
                    p(0.2),
                ],
            );

        assert!(matches!(
            result,
            Err(
                GateError::InvalidParameterCount {
                    gate: GateKind::U3,
                    expected: 3,
                    actual: 2
                }
            )
        ));
    }

    #[test]
    fn non_finite_parameter_is_rejected() {
        let result =
            Gate::parameterized(
                GateKind::RX,
                vec![q(0)],
                vec![
                    Parameter::Constant(
                        f64::NAN
                    )
                ],
            );

        assert!(matches!(
            result,
            Err(
                GateError::InvalidParameter {
                    index: 0,
                    ..
                }
            )
        ));
    }

    #[test]
    fn infinity_parameter_is_rejected() {
        let result =
            Gate::parameterized(
                GateKind::RX,
                vec![q(0)],
                vec![
                    Parameter::Constant(
                        f64::INFINITY
                    )
                ],
            );

        assert!(matches!(
            result,
            Err(
                GateError::InvalidParameter {
                    index: 0,
                    ..
                }
            )
        ));
    }

    #[test]
    fn logical_namespace_is_explicit() {
        let gate =
            Gate::x(q(99))
                .expect(
                    "gate itself is structurally valid",
                );

        assert!(matches!(
            gate.validate_in_namespace(99),
            Err(
                GateError::UnknownQubit {
                    qubit,
                    logical_qubits: 99
                }
            ) if qubit == q(99)
        ));

        assert!(
            gate.validate_in_namespace(100)
                .is_ok()
        );
    }

    #[test]
    fn arbitrary_logical_identifier_has_no_small_machine_limit() {
        let high =
            usize::MAX;

        let gate =
            Gate::x(q(high))
                .expect(
                    "QubitId is a semantic identifier",
                );

        assert_eq!(
            gate.qubits()[0]
                .index(),
            high
        );
    }

    #[test]
    fn duplicate_detection_is_correct_for_many_operands() {
        let qubits = vec![
            q(0),
            q(1),
            q(2),
            q(3),
            q(4),
            q(5),
            q(6),
            q(7),
            q(8),
            q(9),
            q(10),
        ];

        let result =
            Gate::barrier(qubits);

        assert!(result.is_ok());
    }

    #[test]
    fn duplicate_detection_rejects_duplicate_anywhere() {
        let result =
            Gate::barrier(vec![
                q(0),
                q(1),
                q(2),
                q(3),
                q(4),
                q(2),
            ]);

        assert!(matches!(
            result,
            Err(
                GateError::DuplicateQubit {
                    qubit
                }
            ) if qubit == q(2)
        ));
    }

    #[test]
    fn disjoint_gates_are_operand_disjoint() {
        let first =
            Gate::x(q(0))
                .expect("valid gate");

        let second =
            Gate::h(q(1))
                .expect("valid gate");

        assert!(
            first.is_operand_disjoint(
                &second
            )
        );
    }

    #[test]
    fn overlapping_gates_are_not_operand_disjoint() {
        let first =
            Gate::x(q(0))
                .expect("valid gate");

        let second =
            Gate::h(q(0))
                .expect("valid gate");

        assert!(
            !first.is_operand_disjoint(
                &second
            )
        );
    }

    #[test]
    fn operand_order_is_preserved() {
        let gate =
            Gate::cx(
                q(7),
                q(2),
            )
            .expect(
                "valid CX",
            );

        assert_eq!(
            gate.qubits(),
            &[q(7), q(2)]
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
            .expect(
                "valid U3",
            );

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

    #[test]
    fn all_standard_fixed_single_qubit_gates_have_zero_parameters() {
        let gates = [
            GateKind::I,
            GateKind::X,
            GateKind::Y,
            GateKind::Z,
            GateKind::H,
            GateKind::S,
            GateKind::Sdg,
            GateKind::T,
            GateKind::Tdg,
            GateKind::V,
            GateKind::Vdg,
        ];

        for gate in gates {
            assert_eq!(
                gate.parameter_count(),
                0
            );

            assert_eq!(
                gate.operand_count(),
                OperandCount::Exact(1)
            );
        }
    }

    #[test]
    fn parameterized_gate_metadata_is_consistent() {
        assert_eq!(
            GateKind::RX.parameter_count(),
            1
        );

        assert_eq!(
            GateKind::RY.parameter_count(),
            1
        );

        assert_eq!(
            GateKind::RZ.parameter_count(),
            1
        );

        assert_eq!(
            GateKind::U2.parameter_count(),
            2
        );

        assert_eq!(
            GateKind::U3.parameter_count(),
            3
        );

        assert!(
            GateKind::RX.is_parameterized()
        );

        assert!(
            GateKind::U3.is_parameterized()
        );
    }
}