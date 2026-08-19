//! Zamani Quantum Intermediate Representation — Gates
//!
//! Hardware-independent representation of quantum operations.
//!
//! Logical qubits are represented by `QubitId` rather than raw integers.
//! Physical-qubit mapping belongs to routing/backend lowering.
//!
//! Measurement has a richer semantic representation in `measurement.rs`.
//! `GateKind::Measure` exists here so the circuit IR can represent a lowered
//! measurement operation.

use std::f64::consts::PI;
use std::fmt;

use super::measurement::ClassicalBitId;
use super::qubits::QubitId;

// -----------------------------------------------------------------------------
// Gate kind
// -----------------------------------------------------------------------------

/// Canonical quantum operation kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateKind {
    Identity,

    X,
    Y,
    Z,
    H,

    S,
    Sdg,

    T,
    Tdg,

    CX,
    CY,
    CZ,

    SWAP,

    CCX,
    CSWAP,

    RX,
    RY,
    RZ,

    Phase,
    U1,
    U2,
    U3,

    CRX,
    CRY,
    CRZ,

    Barrier,
    Measure,
    Reset,
}

impl GateKind {
    /// Canonical textual representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "id",

            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::H => "h",

            Self::S => "s",
            Self::Sdg => "sdg",

            Self::T => "t",
            Self::Tdg => "tdg",

            Self::CX => "cx",
            Self::CY => "cy",
            Self::CZ => "cz",

            Self::SWAP => "swap",

            Self::CCX => "ccx",
            Self::CSWAP => "cswap",

            Self::RX => "rx",
            Self::RY => "ry",
            Self::RZ => "rz",

            Self::Phase => "phase",
            Self::U1 => "u1",
            Self::U2 => "u2",
            Self::U3 => "u3",

            Self::CRX => "crx",
            Self::CRY => "cry",
            Self::CRZ => "crz",

            Self::Barrier => "barrier",
            Self::Measure => "measure",
            Self::Reset => "reset",
        }
    }

    /// Whether this gate accepts parameters.
    pub const fn is_parameterized(self) -> bool {
        matches!(
            self,
            Self::RX
                | Self::RY
                | Self::RZ
                | Self::Phase
                | Self::U1
                | Self::U2
                | Self::U3
                | Self::CRX
                | Self::CRY
                | Self::CRZ
        )
    }

    /// Whether this operation is a measurement.
    pub const fn is_measurement(self) -> bool {
        matches!(self, Self::Measure)
    }

    /// Whether this operation is a barrier.
    pub const fn is_barrier(self) -> bool {
        matches!(self, Self::Barrier)
    }

    /// Whether this operation is identity.
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::Identity)
    }

    /// Whether this operation is a reset.
    pub const fn is_reset(self) -> bool {
        matches!(self, Self::Reset)
    }

    /// Whether the operation belongs to the T family.
    pub const fn is_t_gate(self) -> bool {
        matches!(self, Self::T | Self::Tdg)
    }

    /// Whether the operation is self-inverse.
    pub const fn is_self_inverse(self) -> bool {
        matches!(
            self,
            Self::Identity
                | Self::X
                | Self::Y
                | Self::Z
                | Self::H
                | Self::CX
                | Self::CY
                | Self::CZ
                | Self::SWAP
                | Self::CCX
                | Self::CSWAP
        )
    }

    /// Whether the gate is Clifford.
    pub const fn is_clifford(self) -> bool {
        matches!(
            self,
            Self::Identity
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
                | Self::CCX
                | Self::CSWAP
        )
    }

    /// Number of required qubit operands.
    ///
    /// `None` means variable width, which currently applies to barriers.
    pub const fn expected_qubits(self) -> Option<usize> {
        match self {
            Self::Identity
            | Self::X
            | Self::Y
            | Self::Z
            | Self::H
            | Self::S
            | Self::Sdg
            | Self::T
            | Self::Tdg
            | Self::RX
            | Self::RY
            | Self::RZ
            | Self::Phase
            | Self::U1
            | Self::U2
            | Self::U3
            | Self::Measure
            | Self::Reset => Some(1),

            Self::CX
            | Self::CY
            | Self::CZ
            | Self::SWAP
            | Self::CRX
            | Self::CRY
            | Self::CRZ => Some(2),

            Self::CCX | Self::CSWAP => Some(3),

            Self::Barrier => None,
        }
    }
}

impl fmt::Display for GateKind {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// -----------------------------------------------------------------------------
// Gate parameters
// -----------------------------------------------------------------------------

/// Numerical parameters attached to parameterized gates.
///
/// All angles are expressed in radians.
#[derive(Debug, Clone, PartialEq)]
pub enum GateParameter {
    /// One parameter.
    Angle(f64),

    /// Two parameters.
    TwoAngles {
        theta: f64,
        phi: f64,
    },

    /// Three parameters.
    ThreeAngles {
        theta: f64,
        phi: f64,
        lambda: f64,
    },
}

impl GateParameter {
    /// Validates that all parameters are finite.
    pub fn validate(&self) -> Result<(), GateError> {
        let valid = match self {
            Self::Angle(angle) => angle.is_finite(),

            Self::TwoAngles { theta, phi } => {
                theta.is_finite()
                    && phi.is_finite()
            }

            Self::ThreeAngles {
                theta,
                phi,
                lambda,
            } => {
                theta.is_finite()
                    && phi.is_finite()
                    && lambda.is_finite()
            }
        };

        if valid {
            Ok(())
        } else {
            Err(GateError::InvalidParameter)
        }
    }

    /// Returns the first angle.
    pub fn first_angle(&self) -> f64 {
        match self {
            Self::Angle(angle) => *angle,

            Self::TwoAngles { theta, .. } => *theta,

            Self::ThreeAngles { theta, .. } => *theta,
        }
    }

    /// Returns all angles in order.
    pub fn angles(&self) -> Vec<f64> {
        match self {
            Self::Angle(angle) => vec![*angle],

            Self::TwoAngles { theta, phi } => {
                vec![*theta, *phi]
            }

            Self::ThreeAngles {
                theta,
                phi,
                lambda,
            } => vec![*theta, *phi, *lambda],
        }
    }
}

// -----------------------------------------------------------------------------
// Gate errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateError {
    InvalidQubitCount {
        gate: GateKind,
        expected: usize,
        actual: usize,
    },

    DuplicateQubit {
        qubit: QubitId,
    },

    MissingParameter {
        gate: GateKind,
    },

    UnexpectedParameter {
        gate: GateKind,
    },

    InvalidParameter,

    InvalidClassicalTarget {
        gate: GateKind,
    },

    MissingClassicalTarget,

    InvalidBarrier,
}

impl fmt::Display for GateError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidQubitCount {
                gate,
                expected,
                actual,
            } => write!(
                f,
                "gate `{gate}` expects {expected} qubits but received {actual}"
            ),

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "qubit {qubit} appears more than once"
                )
            }

            Self::MissingParameter { gate } => {
                write!(
                    f,
                    "gate `{gate}` requires a parameter"
                )
            }

            Self::UnexpectedParameter { gate } => {
                write!(
                    f,
                    "gate `{gate}` does not accept parameters"
                )
            }

            Self::InvalidParameter => {
                write!(
                    f,
                    "gate parameter must be finite"
                )
            }

            Self::InvalidClassicalTarget { gate } => {
                write!(
                    f,
                    "gate `{gate}` cannot target a classical bit"
                )
            }

            Self::MissingClassicalTarget => {
                write!(
                    f,
                    "measurement requires a classical target"
                )
            }

            Self::InvalidBarrier => {
                write!(f, "invalid barrier")
            }
        }
    }
}

impl std::error::Error for GateError {}

// -----------------------------------------------------------------------------
// Gate
// -----------------------------------------------------------------------------

/// Canonical quantum operation.
///
/// Qubits are logical `QubitId`s. Physical hardware mapping is intentionally
/// excluded from this type.
#[derive(Debug, Clone, PartialEq)]
pub struct Gate {
    kind: GateKind,
    qubits: Vec<QubitId>,
    parameter: Option<GateParameter>,
    classical_target: Option<ClassicalBitId>,
}

impl Gate {
    /// Creates a non-parameterized gate.
    pub fn new(
        kind: GateKind,
        qubits: Vec<QubitId>,
    ) -> Result<Self, GateError> {
        let gate = Self {
            kind,
            qubits,
            parameter: None,
            classical_target: None,
        };

        gate.validate()?;

        Ok(gate)
    }

    /// Creates a parameterized gate.
    pub fn parameterized(
        kind: GateKind,
        qubits: Vec<QubitId>,
        parameter: GateParameter,
    ) -> Result<Self, GateError> {
        let gate = Self {
            kind,
            qubits,
            parameter: Some(parameter),
            classical_target: None,
        };

        gate.validate()?;

        Ok(gate)
    }

    /// Creates a measurement gate.
    ///
    /// Rich basis/mode semantics are represented by `Measurement`.
    pub fn measurement(
        qubit: QubitId,
        classical_bit: ClassicalBitId,
    ) -> Result<Self, GateError> {
        let gate = Self {
            kind: GateKind::Measure,
            qubits: vec![qubit],
            parameter: None,
            classical_target: Some(classical_bit),
        };

        gate.validate()?;

        Ok(gate)
    }

    /// Creates a barrier.
    pub fn barrier(
        qubits: Vec<QubitId>,
    ) -> Result<Self, GateError> {
        if qubits.is_empty() {
            return Err(GateError::InvalidBarrier);
        }

        let gate = Self {
            kind: GateKind::Barrier,
            qubits,
            parameter: None,
            classical_target: None,
        };

        gate.validate()?;

        Ok(gate)
    }

    // -------------------------------------------------------------------------
    // Single-qubit constructors
    // -------------------------------------------------------------------------

    pub fn id(
        qubit: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::Identity,
            vec![qubit],
        )
    }

    pub fn x(
        qubit: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::X, vec![qubit])
    }

    pub fn y(
        qubit: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::Y, vec![qubit])
    }

    pub fn z(
        qubit: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::Z, vec![qubit])
    }

    pub fn h(
        qubit: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::H, vec![qubit])
    }

    pub fn s(
        qubit: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::S, vec![qubit])
    }

    pub fn sdg(
        qubit: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::Sdg, vec![qubit])
    }

    pub fn t(
        qubit: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::T, vec![qubit])
    }

    pub fn tdg(
        qubit: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::Tdg, vec![qubit])
    }

    // -------------------------------------------------------------------------
    // Two-qubit constructors
    // -------------------------------------------------------------------------

    pub fn cx(
        control: QubitId,
        target: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::CX,
            vec![control, target],
        )
    }

    pub fn cy(
        control: QubitId,
        target: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::CY,
            vec![control, target],
        )
    }

    pub fn cz(
        control: QubitId,
        target: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::CZ,
            vec![control, target],
        )
    }

    pub fn swap(
        first: QubitId,
        second: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::SWAP,
            vec![first, second],
        )
    }

    // -------------------------------------------------------------------------
    // Three-qubit constructors
    // -------------------------------------------------------------------------

    pub fn ccx(
        control_a: QubitId,
        control_b: QubitId,
        target: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::CCX,
            vec![
                control_a,
                control_b,
                target,
            ],
        )
    }

    pub fn cswap(
        control: QubitId,
        target_a: QubitId,
        target_b: QubitId,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::CSWAP,
            vec![
                control,
                target_a,
                target_b,
            ],
        )
    }

    // -------------------------------------------------------------------------
    // Rotation constructors
    // -------------------------------------------------------------------------

    pub fn rx(
        qubit: QubitId,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::RX,
            vec![qubit],
            GateParameter::Angle(angle),
        )
    }

    pub fn ry(
        qubit: QubitId,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::RY,
            vec![qubit],
            GateParameter::Angle(angle),
        )
    }

    pub fn rz(
        qubit: QubitId,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::RZ,
            vec![qubit],
            GateParameter::Angle(angle),
        )
    }

    pub fn phase(
        qubit: QubitId,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::Phase,
            vec![qubit],
            GateParameter::Angle(angle),
        )
    }

    pub fn u1(
        qubit: QubitId,
        lambda: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::U1,
            vec![qubit],
            GateParameter::Angle(lambda),
        )
    }

    pub fn u2(
        qubit: QubitId,
        phi: f64,
        lambda: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::U2,
            vec![qubit],
            GateParameter::TwoAngles {
                theta: PI / 2.0,
                phi: phi + lambda * 0.0,
            },
        )
    }

    pub fn u3(
        qubit: QubitId,
        theta: f64,
        phi: f64,
        lambda: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::U3,
            vec![qubit],
            GateParameter::ThreeAngles {
                theta,
                phi,
                lambda,
            },
        )
    }

    // -------------------------------------------------------------------------
    // Controlled rotations
    // -------------------------------------------------------------------------

    pub fn crx(
        control: QubitId,
        target: QubitId,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::CRX,
            vec![control, target],
            GateParameter::Angle(angle),
        )
    }

    pub fn cry(
        control: QubitId,
        target: QubitId,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::CRY,
            vec![control, target],
            GateParameter::Angle(angle),
        )
    }

    pub fn crz(
        control: QubitId,
        target: QubitId,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::CRZ,
            vec![control, target],
            GateParameter::Angle(angle),
        )
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    pub const fn kind(&self) -> GateKind {
        self.kind
    }

    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }

    pub fn parameter(
        &self,
    ) -> Option<&GateParameter> {
        self.parameter.as_ref()
    }

    pub const fn classical_target(
        &self,
    ) -> Option<ClassicalBitId> {
        self.classical_target
    }

    pub fn angle(&self) -> Option<f64> {
        self.parameter
            .as_ref()
            .map(GateParameter::first_angle)
    }

    pub fn angles(&self) -> Vec<f64> {
        self.parameter
            .as_ref()
            .map(GateParameter::angles)
            .unwrap_or_default()
    }

    pub const fn is_measurement(&self) -> bool {
        self.kind.is_measurement()
    }

    pub const fn is_barrier(&self) -> bool {
        self.kind.is_barrier()
    }

    pub const fn is_identity(&self) -> bool {
        self.kind.is_identity()
    }

    pub const fn is_reset(&self) -> bool {
        self.kind.is_reset()
    }

    pub const fn is_t_gate(&self) -> bool {
        self.kind.is_t_gate()
    }

    pub const fn is_clifford(&self) -> bool {
        self.kind.is_clifford()
    }

    pub const fn is_parameterized(&self) -> bool {
        self.kind.is_parameterized()
    }

    pub const fn is_self_inverse(&self) -> bool {
        self.kind.is_self_inverse()
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    pub fn validate(&self) -> Result<(), GateError> {
        self.validate_qubits()?;
        self.validate_parameter()?;
        self.validate_classical_target()?;

        Ok(())
    }

    fn validate_qubits(&self) -> Result<(), GateError> {
        if let Some(expected) =
            self.kind.expected_qubits()
        {
            if self.qubits.len() != expected {
                return Err(
                    GateError::InvalidQubitCount {
                        gate: self.kind,
                        expected,
                        actual: self.qubits.len(),
                    },
                );
            }
        } else if self.qubits.is_empty() {
            return Err(
                GateError::InvalidQubitCount {
                    gate: self.kind,
                    expected: 1,
                    actual: 0,
                },
            );
        }

        for (index, qubit) in
            self.qubits.iter().enumerate()
        {
            if self.qubits[index + 1..]
                .contains(qubit)
            {
                return Err(
                    GateError::DuplicateQubit {
                        qubit: *qubit,
                    },
                );
            }
        }

        Ok(())
    }

    fn validate_parameter(&self) -> Result<(), GateError> {
        if self.kind.is_parameterized() {
            let parameter =
                self.parameter.as_ref().ok_or(
                    GateError::MissingParameter {
                        gate: self.kind,
                    },
                )?;

            parameter.validate()?;
        } else if self.parameter.is_some() {
            return Err(
                GateError::UnexpectedParameter {
                    gate: self.kind,
                },
            );
        }

        Ok(())
    }

    fn validate_classical_target(
        &self,
    ) -> Result<(), GateError> {
        if self.kind == GateKind::Measure {
            if self.classical_target.is_none() {
                return Err(
                    GateError::MissingClassicalTarget,
                );
            }
        } else if self.classical_target.is_some() {
            return Err(
                GateError::InvalidClassicalTarget {
                    gate: self.kind,
                },
            );
        }

        Ok(())
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

    fn c(index: usize) -> ClassicalBitId {
        ClassicalBitId::new(index)
    }

    #[test]
    fn creates_x_gate() {
        let gate =
            Gate::x(q(0)).unwrap();

        assert_eq!(gate.kind(), GateKind::X);
        assert_eq!(gate.qubits(), &[q(0)]);
        assert!(gate.is_clifford());
        assert!(gate.is_self_inverse());
    }

    #[test]
    fn creates_cx_gate() {
        let gate =
            Gate::cx(q(0), q(1)).unwrap();

        assert_eq!(gate.kind(), GateKind::CX);
        assert_eq!(
            gate.qubits(),
            &[q(0), q(1)]
        );
    }

    #[test]
    fn rejects_duplicate_qubits() {
        let result =
            Gate::cx(q(0), q(0));

        assert_eq!(
            result,
            Err(GateError::DuplicateQubit {
                qubit: q(0),
            })
        );
    }

    #[test]
    fn rejects_wrong_qubit_count() {
        let result =
            Gate::new(
                GateKind::CX,
                vec![q(0)],
            );

        assert_eq!(
            result,
            Err(
                GateError::InvalidQubitCount {
                    gate: GateKind::CX,
                    expected: 2,
                    actual: 1,
                }
            )
        );
    }

    #[test]
    fn creates_rz() {
        let angle = PI / 4.0;

        let gate =
            Gate::rz(q(0), angle)
                .unwrap();

        assert!(gate.is_parameterized());
        assert_eq!(
            gate.angle(),
            Some(angle)
        );
    }

    #[test]
    fn rejects_nan_parameter() {
        let result =
            Gate::rz(q(0), f64::NAN);

        assert_eq!(
            result,
            Err(GateError::InvalidParameter)
        );
    }

    #[test]
    fn creates_measurement() {
        let gate =
            Gate::measurement(
                q(0),
                c(0),
            )
            .unwrap();

        assert!(gate.is_measurement());

        assert_eq!(
            gate.classical_target(),
            Some(c(0))
        );
    }

    #[test]
    fn measurement_requires_classical_target() {
        let gate = Gate {
            kind: GateKind::Measure,
            qubits: vec![q(0)],
            parameter: None,
            classical_target: None,
        };

        assert_eq!(
            gate.validate(),
            Err(
                GateError::MissingClassicalTarget
            )
        );
    }

    #[test]
    fn creates_barrier() {
        let gate =
            Gate::barrier(vec![
                q(0),
                q(1),
                q(2),
            ])
            .unwrap();

        assert!(gate.is_barrier());

        assert_eq!(
            gate.qubits().len(),
            3
        );
    }

    #[test]
    fn rejects_empty_barrier() {
        let result =
            Gate::barrier(Vec::new());

        assert_eq!(
            result,
            Err(GateError::InvalidBarrier)
        );
    }

    #[test]
    fn t_gate_is_not_clifford() {
        let gate =
            Gate::t(q(0)).unwrap();

        assert!(gate.is_t_gate());
        assert!(!gate.is_clifford());
    }

    #[test]
    fn tdg_is_t_family() {
        let gate =
            Gate::tdg(q(0)).unwrap();

        assert!(gate.is_t_gate());
    }

    #[test]
    fn identity_is_self_inverse() {
        let gate =
            Gate::id(q(0)).unwrap();

        assert!(gate.is_identity());
        assert!(gate.is_self_inverse());
    }

    #[test]
    fn parameter_angles_are_exposed() {
        let gate =
            Gate::u3(
                q(0),
                0.1,
                0.2,
                0.3,
            )
            .unwrap();

        assert_eq!(
            gate.angles(),
            vec![0.1, 0.2, 0.3]
        );
    }

    #[test]
    fn gate_names_are_canonical() {
        assert_eq!(
            GateKind::CX.as_str(),
            "cx"
        );

        assert_eq!(
            GateKind::Tdg.as_str(),
            "tdg"
        );

        assert_eq!(
            GateKind::RZ.as_str(),
            "rz"
        );
    }

    #[test]
    fn cswap_has_three_qubits() {
        let gate =
            Gate::cswap(
                q(0),
                q(1),
                q(2),
            )
            .unwrap();

        assert_eq!(
            gate.kind(),
            GateKind::CSWAP
        );

        assert_eq!(
            gate.qubits().len(),
            3
        );
    }
}