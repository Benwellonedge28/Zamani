//! Zamani Quantum Intermediate Representation — Gates
//!
//! Canonical representation of quantum operations.
//!
//! This module deliberately keeps gates hardware-independent. Backend-specific
//! concerns such as native gate sets, pulse schedules, calibration, routing,
//! and physical qubit mapping belong to later compiler stages.
//!
//! The `Gate` type is shared by:
//!
//! - Quantum circuit construction
//! - Peephole optimization
//! - Cancellation
//! - T-gate reduction
//! - Routing
//! - Scheduling
//! - Quantum simulation
//! - Backend lowering
//!
//! The representation is intentionally explicit so optimization passes can
//! reason about qubits, parameters, measurements, and barriers without
//! depending on textual gate names.

use std::f64::consts::PI;
use std::fmt;

// -----------------------------------------------------------------------------
// Gate kind
// -----------------------------------------------------------------------------

/// Canonical quantum operation kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateKind {
    // -------------------------------------------------------------------------
    // Primitive / Clifford gates
    // -------------------------------------------------------------------------

    Identity,
    X,
    Y,
    Z,
    H,

    S,
    Sdg,

    T,
    Tdg,

    // -------------------------------------------------------------------------
    // Two-qubit gates
    // -------------------------------------------------------------------------

    CX,
    CZ,
    CY,

    SWAP,

    // -------------------------------------------------------------------------
    // Three-qubit gates
    // -------------------------------------------------------------------------

    CCX,
    CSWAP,

    // -------------------------------------------------------------------------
    // Parameterized single-qubit gates
    // -------------------------------------------------------------------------

    RX,
    RY,
    RZ,

    Phase,
    U1,
    U2,
    U3,

    // -------------------------------------------------------------------------
    // Parameterized multi-qubit gates
    // -------------------------------------------------------------------------

    CRX,
    CRY,
    CRZ,

    // -------------------------------------------------------------------------
    // Circuit-control operations
    // -------------------------------------------------------------------------

    Barrier,
    Measure,
    Reset,
}

impl GateKind {
    /// Returns the canonical textual name.
    pub fn as_str(&self) -> &'static str {
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
            Self::CZ => "cz",
            Self::CY => "cy",

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

    /// Returns whether this gate requires one or more angles.
    pub fn is_parameterized(&self) -> bool {
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

    /// Returns whether the operation is a measurement.
    pub fn is_measurement(&self) -> bool {
        matches!(self, Self::Measure)
    }

    /// Returns whether the operation is a barrier.
    pub fn is_barrier(&self) -> bool {
        matches!(self, Self::Barrier)
    }

    /// Returns whether the operation is an identity.
    pub fn is_identity(&self) -> bool {
        matches!(self, Self::Identity)
    }

    /// Returns whether the gate is self-inverse.
    pub fn is_self_inverse(&self) -> bool {
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

    /// Returns whether the gate belongs to the Clifford group.
    pub fn is_clifford(&self) -> bool {
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

    /// Returns whether this is a T-family gate.
    pub fn is_t_gate(&self) -> bool {
        matches!(self, Self::T | Self::Tdg)
    }

    /// Returns the expected number of qubit operands.
    ///
    /// `None` means variable-width, such as barriers.
    pub fn expected_qubits(&self) -> Option<usize> {
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

            Self::CCX
            | Self::CSWAP => Some(3),

            Self::Barrier => None,
        }
    }
}

impl fmt::Display for GateKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// -----------------------------------------------------------------------------
// Gate parameters
// -----------------------------------------------------------------------------

/// Parameters attached to a quantum gate.
///
/// Angles are expressed in radians.
#[derive(Debug, Clone, PartialEq)]
pub enum GateParameter {
    /// Single angle.
    Angle(f64),

    /// Two angles, used by U2.
    TwoAngles {
        theta: f64,
        phi: f64,
    },

    /// Three angles, used by U3.
    ThreeAngles {
        theta: f64,
        phi: f64,
        lambda: f64,
    },
}

impl GateParameter {
    /// Validates all numerical parameters.
    pub fn validate(&self) -> Result<(), GateError> {
        let valid = match self {
            Self::Angle(angle) => angle.is_finite(),

            Self::TwoAngles { theta, phi } => {
                theta.is_finite() && phi.is_finite()
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

    /// Returns the first angle if present.
    pub fn first_angle(&self) -> f64 {
        match self {
            Self::Angle(angle) => *angle,

            Self::TwoAngles { theta, .. } => *theta,

            Self::ThreeAngles { theta, .. } => *theta,
        }
    }
}

// -----------------------------------------------------------------------------
// Gate errors
// -----------------------------------------------------------------------------

/// Errors produced while constructing or validating gates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateError {
    /// The gate received the wrong number of qubits.
    InvalidQubitCount {
        gate: GateKind,
        expected: usize,
        actual: usize,
    },

    /// A qubit occurs more than once.
    DuplicateQubit {
        qubit: usize,
    },

    /// A required parameter is missing.
    MissingParameter {
        gate: GateKind,
    },

    /// A parameter was supplied to a gate that does not accept one.
    UnexpectedParameter {
        gate: GateKind,
    },

    /// A parameter is NaN or infinite.
    InvalidParameter,

    /// A classical target was supplied to a non-measurement operation.
    InvalidClassicalTarget {
        gate: GateKind,
    },

    /// A measurement has no classical destination.
    MissingClassicalTarget,

    /// A barrier has an invalid configuration.
    InvalidBarrier,
}

impl fmt::Display for GateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                write!(f, "qubit {qubit} appears more than once")
            }

            Self::MissingParameter { gate } => {
                write!(f, "gate `{gate}` requires a parameter")
            }

            Self::UnexpectedParameter { gate } => {
                write!(
                    f,
                    "gate `{gate}` does not accept a parameter"
                )
            }

            Self::InvalidParameter => {
                write!(f, "gate parameter must be finite")
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
                write!(f, "invalid barrier configuration")
            }
        }
    }
}

impl std::error::Error for GateError {}

// -----------------------------------------------------------------------------
// Gate
// -----------------------------------------------------------------------------

/// Canonical quantum operation.
#[derive(Debug, Clone, PartialEq)]
pub struct Gate {
    kind: GateKind,
    qubits: Vec<usize>,
    parameter: Option<GateParameter>,
    classical_target: Option<usize>,
}

impl Gate {
    /// Creates a non-parameterized gate.
    pub fn new(
        kind: GateKind,
        qubits: Vec<usize>,
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
        qubits: Vec<usize>,
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

    /// Creates a measurement operation.
    pub fn measurement(
        qubit: usize,
        classical_bit: usize,
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

    /// Creates a barrier over a set of qubits.
    pub fn barrier(
        qubits: Vec<usize>,
    ) -> Result<Self, GateError> {
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
    // Common constructors
    // -------------------------------------------------------------------------

    pub fn id(
        qubit: usize,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::Identity, vec![qubit])
    }

    pub fn x(
        qubit: usize,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::X, vec![qubit])
    }

    pub fn y(
        qubit: usize,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::Y, vec![qubit])
    }

    pub fn z(
        qubit: usize,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::Z, vec![qubit])
    }

    pub fn h(
        qubit: usize,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::H, vec![qubit])
    }

    pub fn s(
        qubit: usize,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::S, vec![qubit])
    }

    pub fn sdg(
        qubit: usize,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::Sdg, vec![qubit])
    }

    pub fn t(
        qubit: usize,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::T, vec![qubit])
    }

    pub fn tdg(
        qubit: usize,
    ) -> Result<Self, GateError> {
        Self::new(GateKind::Tdg, vec![qubit])
    }

    pub fn cx(
        control: usize,
        target: usize,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::CX,
            vec![control, target],
        )
    }

    pub fn cy(
        control: usize,
        target: usize,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::CY,
            vec![control, target],
        )
    }

    pub fn cz(
        control: usize,
        target: usize,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::CZ,
            vec![control, target],
        )
    }

    pub fn swap(
        first: usize,
        second: usize,
    ) -> Result<Self, GateError> {
        Self::new(
            GateKind::SWAP,
            vec![first, second],
        )
    }

    pub fn ccx(
        control_a: usize,
        control_b: usize,
        target: usize,
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

    // -------------------------------------------------------------------------
    // Rotation constructors
    // -------------------------------------------------------------------------

    pub fn rx(
        qubit: usize,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::RX,
            vec![qubit],
            GateParameter::Angle(angle),
        )
    }

    pub fn ry(
        qubit: usize,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::RY,
            vec![qubit],
            GateParameter::Angle(angle),
        )
    }

    pub fn rz(
        qubit: usize,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::RZ,
            vec![qubit],
            GateParameter::Angle(angle),
        )
    }

    pub fn phase(
        qubit: usize,
        angle: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::Phase,
            vec![qubit],
            GateParameter::Angle(angle),
        )
    }

    pub fn u1(
        qubit: usize,
        lambda: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::U1,
            vec![qubit],
            GateParameter::Angle(lambda),
        )
    }

    pub fn u2(
        qubit: usize,
        phi: f64,
        lambda: f64,
    ) -> Result<Self, GateError> {
        Self::parameterized(
            GateKind::U2,
            vec![qubit],
            GateParameter::TwoAngles {
                theta: PI / 2.0,
                phi,
            },
        )
        .and_then(|gate| {
            let mut gate = gate;

            gate.parameter =
                Some(GateParameter::TwoAngles {
                    theta: lambda,
                    phi,
                });

            gate.validate()?;

            Ok(gate)
        })
    }

    pub fn u3(
        qubit: usize,
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
    // Accessors
    // -------------------------------------------------------------------------

    pub fn kind(&self) -> GateKind {
        self.kind
    }

    pub fn qubits(&self) -> &[usize] {
        &self.qubits
    }

    pub fn parameter(
        &self,
    ) -> Option<&GateParameter> {
        self.parameter.as_ref()
    }

    pub fn classical_target(
        &self,
    ) -> Option<usize> {
        self.classical_target
    }

    /// Returns the first rotation angle if one exists.
    pub fn angle(&self) -> Option<f64> {
        self.parameter
            .as_ref()
            .map(GateParameter::first_angle)
    }

    // -------------------------------------------------------------------------
    // Classification
    // -------------------------------------------------------------------------

    pub fn is_measurement(&self) -> bool {
        self.kind.is_measurement()
    }

    pub fn is_barrier(&self) -> bool {
        self.kind.is_barrier()
    }

    pub fn is_identity(&self) -> bool {
        self.kind.is_identity()
    }

    pub fn is_t_gate(&self) -> bool {
        self.kind.is_t_gate()
    }

    pub fn is_clifford(&self) -> bool {
        self.kind.is_clifford()
    }

    pub fn is_parameterized(&self) -> bool {
        self.kind.is_parameterized()
    }

    pub fn is_self_inverse(&self) -> bool {
        self.kind.is_self_inverse()
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates the complete gate.
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

    #[test]
    fn creates_x_gate() {
        let gate =
            Gate::x(0)
                .expect("X gate should be valid");

        assert_eq!(gate.kind(), GateKind::X);
        assert_eq!(gate.qubits(), &[0]);
        assert!(gate.is_clifford());
        assert!(gate.is_self_inverse());
    }

    #[test]
    fn creates_two_qubit_gate() {
        let gate =
            Gate::cx(0, 1)
                .expect("CX gate should be valid");

        assert_eq!(gate.kind(), GateKind::CX);
        assert_eq!(gate.qubits(), &[0, 1]);
        assert!(gate.is_self_inverse());
    }

    #[test]
    fn rejects_duplicate_qubits() {
        let result =
            Gate::cx(0, 0);

        assert!(matches!(
            result,
            Err(GateError::DuplicateQubit {
                qubit: 0
            })
        ));
    }

    #[test]
    fn rejects_wrong_qubit_count() {
        let result =
            Gate::new(GateKind::CX, vec![0]);

        assert!(matches!(
            result,
            Err(
                GateError::InvalidQubitCount {
                    gate: GateKind::CX,
                    expected: 2,
                    actual: 1
                }
            )
        ));
    }

    #[test]
    fn creates_rotation() {
        let gate =
            Gate::rz(0, PI / 4.0)
                .expect("RZ should be valid");

        assert_eq!(gate.kind(), GateKind::RZ);
        assert!(gate.is_parameterized());
        assert_eq!(
            gate.angle(),
            Some(PI / 4.0)
        );
    }

    #[test]
    fn rejects_nan_rotation() {
        let result =
            Gate::rz(0, f64::NAN);

        assert_eq!(
            result,
            Err(GateError::InvalidParameter)
        );
    }

    #[test]
    fn creates_measurement() {
        let gate =
            Gate::measurement(0, 0)
                .expect("measurement should be valid");

        assert!(gate.is_measurement());
        assert_eq!(
            gate.classical_target(),
            Some(0)
        );
    }

    #[test]
    fn measurement_requires_target() {
        let gate = Gate {
            kind: GateKind::Measure,
            qubits: vec![0],
            parameter: None,
            classical_target: None,
        };

        assert_eq!(
            gate.validate(),
            Err(GateError::MissingClassicalTarget)
        );
    }

    #[test]
    fn barrier_can_cover_multiple_qubits() {
        let barrier =
            Gate::barrier(vec![0, 1, 2])
                .expect("barrier should be valid");

        assert!(barrier.is_barrier());
        assert_eq!(
            barrier.qubits(),
            &[0, 1, 2]
        );
    }

    #[test]
    fn t_gate_is_not_clifford() {
        let gate =
            Gate::t(0)
                .expect("T should be valid");

        assert!(gate.is_t_gate());
        assert!(!gate.is_clifford());
    }

    #[test]
    fn t_dagger_is_t_family() {
        let gate =
            Gate::tdg(0)
                .expect("Tdg should be valid");

        assert!(gate.is_t_gate());
    }

    #[test]
    fn identity_is_self_inverse() {
        let gate =
            Gate::id(0)
                .expect("identity should be valid");

        assert!(gate.is_identity());
        assert!(gate.is_self_inverse());
    }

    #[test]
    fn gate_kind_names_are_canonical() {
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
    fn gate_parameter_validation() {
        let parameter =
            GateParameter::Angle(PI / 4.0);

        assert!(parameter.validate().is_ok());
    }

    #[test]
    fn gate_parameter_rejects_infinity() {
        let parameter =
            GateParameter::Angle(f64::INFINITY);

        assert_eq!(
            parameter.validate(),
            Err(GateError::InvalidParameter)
        );
    }

    #[test]
    fn gate_is_cloneable_and_deterministic() {
        let gate =
            Gate::h(0)
                .expect("H should be valid");

        assert_eq!(gate.clone(), gate);
    }
}