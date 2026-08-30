//! Zamani Quantum Optimization — Diagonal Algebra
//!
//! Production-grade algebra and semantic analysis for operations that are
//! diagonal in the computational basis.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir::Gate
//!                            │
//!                            ▼
//!              optimization::operation
//!                            │
//!                            ▼
//!              optimization::algebra::diagonal
//!                            │
//!          ┌─────────────────┼─────────────────┐
//!          │                 │                 │
//!          ▼                 ▼                 ▼
//!      analysis          rewrite           synthesis
//!          │                 │                 │
//!          └─────────────────┼─────────────────┘
//!                            ▼
//!                    optimized Quantum IR
//! ```
//!
//! This module is intentionally a pure algebra/semantic layer.
//!
//! It does NOT:
//!
//! - define another quantum IR;
//! - own `QuantumCircuit`;
//! - mutate circuits;
//! - perform routing;
//! - perform scheduling;
//! - communicate with hardware;
//! - execute quantum programs;
//! - choose a backend;
//! - own a cost model;
//! - own a global optimization pipeline;
//! - perform QAOA/VQE or other algorithms.
//!
//! Those responsibilities belong to the corresponding Zamani subsystems.
//!
//! # Canonical representation
//!
//! All quantum operations are represented by [`crate::quantum::ir::Gate`].
//! This module only interprets that canonical representation.
//!
//! The current canonical IR provides the following diagonal-capable gate kinds:
//!
//! - `I`
//! - `Z`
//! - `S`
//! - `Sdg`
//! - `T`
//! - `Tdg`
//! - `RZ`
//! - `Phase`
//! - `U1`
//! - `CZ`
//! - `CRZ`
//!
//! The classification is deliberately conservative. A gate is not considered
//! diagonal merely because a particular parameter value happens to make it
//! diagonal. Value-sensitive specialization is exposed separately.
//!
//! # Mathematical contract
//!
//! A gate is computational-basis diagonal when its matrix is of the form
//!
//! ```text
//! D = diag(e^(i φ_0), e^(i φ_1), ..., e^(i φ_(2^n-1)))
//! ```
//!
//! for the gate's logical operands.
//!
//! Diagonal operations therefore preserve computational-basis populations and
//! only modify phases.
//!
//! This property permits:
//!
//! - exact reordering with other independent diagonal operations;
//! - exact reordering with overlapping diagonal operations because diagonal
//!   operators commute;
//! - safe combination of compatible phase generators;
//! - phase-angle normalization;
//! - diagonal-region extraction;
//! - later phase-polynomial and synthesis optimizations.
//!
//! # Important semantic boundary
//!
//! A diagonal operation may commute mathematically with another operation
//! without being freely movable across:
//!
//! - measurement;
//! - reset;
//! - barriers;
//! - classical control;
//! - operations with compiler-visible ordering semantics;
//! - externally observable annotations.
//!
//! Consequently, this module reports algebraic facts but does not authorize
//! arbitrary circuit movement. The enclosing rewrite/commutation pass must
//! enforce circuit-level dependencies and semantic boundaries.
//!
//! # Symbolic parameters
//!
//! Zamani's canonical [`crate::quantum::ir::Parameter`] supports:
//!
//! - finite constants;
//! - symbols;
//! - deterministic arithmetic expressions.
//!
//! This module preserves symbolic parameters. It never approximates symbolic
//! expressions and never silently converts them to floating-point values.
//!
//! # Scalability
//!
//! The implementation avoids constructing a `2^n × 2^n` matrix. Diagonal
//! operations are represented through gate metadata and parameter expressions.
//!
//! Operations therefore remain O(1) with respect to Hilbert-space dimension
//! for the supported primitive gate kinds.
//!
//! No artificial circuit-size ceiling is introduced here. Actual resource
//! limits are enforced by the optimizer's global limits/context subsystem.
//!
//! # Determinism
//!
//! All classification and composition functions are deterministic.
//! No randomness, global state, environment state, or hardware state is used.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features and no additional dependencies are required.
//!
//! # Integration contract
//!
//! `algebra/mod.rs` should expose this module:
//!
//! ```text
//! pub mod diagonal;
//! ```
//!
//! The optimization operation layer can delegate diagonal classification to:
//!
//! ```text
//! diagonal::is_diagonal
//! diagonal::classify
//! ```
//!
//! The rewrite layer can use:
//!
//! ```text
//! diagonal::can_fuse
//! diagonal::fuse
//! diagonal::normalize_parameter
//! ```
//!
//! The phase-polynomial optimizer can use:
//!
//! ```text
//! diagonal::phase_generator
//! diagonal::phase_degree
//! ```
//!
//! Circuit mutation intentionally remains outside this module.

// =============================================================================
// Imports
// =============================================================================

use std::fmt;

use crate::quantum::ir::{
    Gate,
    GateError,
    GateKind,
    Parameter,
    ParameterExpression,
};

// =============================================================================
// Public constants
// =============================================================================

/// Mathematical period of a quantum phase angle.
///
/// All phase angles in this module are expressed in radians.
pub const TWO_PI: f64 = std::f64::consts::TAU;

/// Half-turn.
pub const PI: f64 = std::f64::consts::PI;

/// Quarter-turn.
pub const HALF_PI: f64 = std::f64::consts::FRAC_PI_2;

/// Eighth-turn.
pub const QUARTER_PI: f64 = std::f64::consts::FRAC_PI_4;

// =============================================================================
// Errors
// =============================================================================

/// Errors specific to diagonal algebra.
#[derive(Debug, Clone, PartialEq)]
pub enum DiagonalError {
    /// The supplied gate is not computational-basis diagonal.
    NotDiagonal {
        gate: GateKind,
    },

    /// The supplied gates cannot be fused by this exact fusion operation.
    IncompatibleForFusion {
        left: GateKind,
        right: GateKind,
    },

    /// The gates act on different logical operands.
    QubitMismatch,

    /// A parameter required by the operation is missing.
    MissingParameter,

    /// A parameter could not be combined while preserving the canonical IR
    /// parameter representation.
    ParameterConstruction,

    /// Construction of the canonical gate failed.
    GateConstruction(GateError),

    /// A numerical angle was not finite.
    NonFiniteAngle,

    /// A requested operation is mathematically valid but is not representable
    /// by the current canonical IR gate vocabulary.
    UnsupportedRepresentation {
        gate: GateKind,
    },
}

impl fmt::Display for DiagonalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotDiagonal { gate } => {
                write!(f, "gate {gate:?} is not computational-basis diagonal")
            }

            Self::IncompatibleForFusion { left, right } => {
                write!(
                    f,
                    "diagonal gates {left:?} and {right:?} are not directly fusible"
                )
            }

            Self::QubitMismatch => {
                write!(
                    f,
                    "diagonal gates do not act on the same logical operands"
                )
            }

            Self::MissingParameter => {
                write!(f, "diagonal gate is missing its required parameter")
            }

            Self::ParameterConstruction => {
                write!(
                    f,
                    "failed to construct a canonical diagonal parameter expression"
                )
            }

            Self::GateConstruction(error) => {
                write!(f, "failed to construct canonical diagonal gate: {error}")
            }

            Self::NonFiniteAngle => {
                write!(f, "diagonal phase angle is not finite")
            }

            Self::UnsupportedRepresentation { gate } => {
                write!(
                    f,
                    "diagonal operation {gate:?} has no supported canonical fusion representation"
                )
            }
        }
    }
}

impl std::error::Error for DiagonalError {}

impl From<GateError> for DiagonalError {
    fn from(error: GateError) -> Self {
        Self::GateConstruction(error)
    }
}

/// Result type used by this module.
pub type DiagonalResult<T> = Result<T, DiagonalError>;

// =============================================================================
// Diagonal classification
// =============================================================================

/// Semantic class of a computational-basis diagonal operation.
///
/// The enum is intentionally based on semantic generators rather than
/// hardware-native names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagonalKind {
    /// Mathematical identity.
    Identity,

    /// Single-qubit Z.
    Z,

    /// S phase gate.
    S,

    /// S dagger phase gate.
    Sdg,

    /// T phase gate.
    T,

    /// T dagger phase gate.
    Tdg,

    /// Arbitrary Z rotation.
    RZ,

    /// Arbitrary phase rotation.
    Phase,

    /// Qiskit/OpenQASM-style U1 phase rotation.
    U1,

    /// Controlled-Z.
    CZ,

    /// Controlled arbitrary Z rotation.
    CRZ,
}

impl DiagonalKind {
    /// Returns whether the operation is a single-qubit diagonal generator.
    #[must_use]
    pub const fn is_single_qubit(self) -> bool {
        matches!(
            self,
            Self::Identity
                | Self::Z
                | Self::S
                | Self::Sdg
                | Self::T
                | Self::Tdg
                | Self::RZ
                | Self::Phase
                | Self::U1
        )
    }

    /// Returns whether the operation is a multi-qubit diagonal generator.
    #[must_use]
    pub const fn is_multi_qubit(self) -> bool {
        matches!(self, Self::CZ | Self::CRZ)
    }

    /// Returns the number of logical qubits required by this diagonal kind.
    #[must_use]
    pub const fn arity(self) -> usize {
        match self {
            Self::Identity
            | Self::Z
            | Self::S
            | Self::Sdg
            | Self::T
            | Self::Tdg
            | Self::RZ
            | Self::Phase
            | Self::U1 => 1,

            Self::CZ | Self::CRZ => 2,
        }
    }

    /// Returns whether the operation carries an arbitrary phase parameter.
    #[must_use]
    pub const fn is_parameterized(self) -> bool {
        matches!(self, Self::RZ | Self::Phase | Self::U1 | Self::CRZ)
    }

    /// Returns the canonical fixed phase angle when one exists.
    ///
    /// The returned value is expressed in radians.
    #[must_use]
    pub const fn fixed_angle(self) -> Option<f64> {
        match self {
            Self::Identity => Some(0.0),
            Self::Z => Some(PI),
            Self::S => Some(HALF_PI),
            Self::Sdg => Some(-HALF_PI),
            Self::T => Some(QUARTER_PI),
            Self::Tdg => Some(-QUARTER_PI),

            Self::RZ
            | Self::Phase
            | Self::U1
            | Self::CZ
            | Self::CRZ => None,
        }
    }

    /// Returns whether the fixed operation has a phase period of `2π`.
    #[must_use]
    pub const fn is_phase_periodic(self) -> bool {
        matches!(
            self,
            Self::Identity
                | Self::Z
                | Self::S
                | Self::Sdg
                | Self::T
                | Self::Tdg
                | Self::RZ
                | Self::Phase
                | Self::U1
                | Self::CRZ
        )
    }
}

// =============================================================================
// Gate classification
// =============================================================================

/// Returns the diagonal semantic kind of a canonical gate.
///
/// This is intentionally conservative and purely syntactic.
///
/// A parameterized gate such as `RX(0)` is not classified as diagonal here,
/// even though that particular numerical instance is the identity. Such
/// value-sensitive simplification belongs to parameter-aware optimization.
#[must_use]
pub const fn diagonal_kind(kind: GateKind) -> Option<DiagonalKind> {
    match kind {
        GateKind::I => Some(DiagonalKind::Identity),
        GateKind::Z => Some(DiagonalKind::Z),
        GateKind::S => Some(DiagonalKind::S),
        GateKind::Sdg => Some(DiagonalKind::Sdg),
        GateKind::T => Some(DiagonalKind::T),
        GateKind::Tdg => Some(DiagonalKind::Tdg),
        GateKind::RZ => Some(DiagonalKind::RZ),
        GateKind::Phase => Some(DiagonalKind::Phase),
        GateKind::U1 => Some(DiagonalKind::U1),
        GateKind::CZ => Some(DiagonalKind::CZ),
        GateKind::CRZ => Some(DiagonalKind::CRZ),

        _ => None,
    }
}

/// Returns whether a gate is diagonal in the computational basis.
#[must_use]
pub fn is_diagonal(gate: &Gate) -> bool {
    diagonal_kind(gate.kind()).is_some()
}

/// Returns whether a gate kind is diagonal.
#[must_use]
pub const fn is_diagonal_kind(kind: GateKind) -> bool {
    diagonal_kind(kind).is_some()
}

// =============================================================================
// Phase degree
// =============================================================================

/// Degree of the Boolean phase interaction represented by a diagonal gate.
///
/// This is useful to phase-polynomial and synthesis passes.
///
/// Examples:
///
/// - `Z`, `S`, `T`, `RZ` → degree 1;
/// - `CZ` → degree 2;
/// - `CRZ` → degree 2.
///
/// This does not claim that a gate's full synthesis cost is determined by its
/// degree.
#[must_use]
pub const fn phase_degree(kind: GateKind) -> Option<usize> {
    match diagonal_kind(kind) {
        Some(
            DiagonalKind::Identity
            | DiagonalKind::Z
            | DiagonalKind::S
            | DiagonalKind::Sdg
            | DiagonalKind::T
            | DiagonalKind::Tdg
            | DiagonalKind::RZ
            | DiagonalKind::Phase
            | DiagonalKind::U1,
        ) => Some(1),

        Some(DiagonalKind::CZ | DiagonalKind::CRZ) => Some(2),

        None => None,
    }
}

// =============================================================================
// Parameter view
// =============================================================================

/// Borrowed phase parameter.
///
/// A diagonal operation either has a fixed angle or carries a canonical IR
/// parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhaseParameter<'a> {
    /// Fixed numerical angle.
    Constant(f64),

    /// Borrowed canonical symbolic/numerical parameter.
    Parameter(&'a Parameter),
}

impl<'a> PhaseParameter<'a> {
    /// Returns the parameter as a canonical reference when it is parameterized.
    #[must_use]
    pub const fn as_parameter(self) -> Option<&'a Parameter> {
        match self {
            Self::Constant(_) => None,
            Self::Parameter(parameter) => Some(parameter),
        }
    }

    /// Returns a concrete angle if the parameter is a constant.
    #[must_use]
    pub fn as_constant(self) -> Option<f64> {
        match self {
            Self::Constant(value) => Some(value),
            Self::Parameter(parameter) => parameter.as_constant(),
        }
    }

    /// Returns whether this phase is symbolic.
    #[must_use]
    pub fn is_symbolic(self) -> bool {
        match self {
            Self::Constant(_) => false,
            Self::Parameter(parameter) => parameter.is_symbolic(),
        }
    }
}

/// Returns the phase parameter associated with a diagonal gate.
///
/// Fixed gates return their canonical fixed angle. Parameterized gates return
/// a borrowed parameter from the canonical IR.
///
/// `CZ` is not represented as a single phase parameter because its semantics
/// are conditional on both qubits.
#[must_use]
pub fn phase_parameter(gate: &Gate) -> Option<PhaseParameter<'_>> {
    match diagonal_kind(gate.kind()) {
        Some(
            DiagonalKind::Identity
            | DiagonalKind::Z
            | DiagonalKind::S
            | DiagonalKind::Sdg
            | DiagonalKind::T
            | DiagonalKind::Tdg,
        ) => diagonal_kind(gate.kind())
            .and_then(DiagonalKind::fixed_angle)
            .map(PhaseParameter::Constant),

        Some(
            DiagonalKind::RZ
            | DiagonalKind::Phase
            | DiagonalKind::U1
            | DiagonalKind::CRZ,
        ) => gate
            .parameters()
            .first()
            .map(PhaseParameter::Parameter),

        Some(DiagonalKind::CZ) | None => None,
    }
}

// =============================================================================
// Diagonal metadata
// =============================================================================

/// Complete immutable semantic information about a diagonal gate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiagonalInfo<'a> {
    /// Semantic diagonal kind.
    pub kind: DiagonalKind,

    /// Logical qubit operands.
    pub qubits: &'a [crate::quantum::ir::QubitId],

    /// Phase degree.
    pub degree: usize,

    /// Optional phase parameter.
    pub phase: Option<PhaseParameter<'a>>,

    /// Whether the phase contains symbolic information.
    pub symbolic: bool,
}

impl<'a> DiagonalInfo<'a> {
    /// Builds diagonal metadata for a canonical gate.
    pub fn from_gate(gate: &'a Gate) -> Option<Self> {
        let kind = diagonal_kind(gate.kind())?;
        let phase = phase_parameter(gate);
        let symbolic = phase.map_or(false, PhaseParameter::is_symbolic);

        Some(Self {
            kind,
            qubits: gate.qubits(),
            degree: phase_degree(gate.kind()).unwrap_or(0),
            phase,
            symbolic,
        })
    }

    /// Returns whether this diagonal operation is parameterized.
    #[must_use]
    pub const fn is_parameterized(self) -> bool {
        self.kind.is_parameterized()
    }

    /// Returns whether this operation is a single-qubit diagonal operation.
    #[must_use]
    pub const fn is_single_qubit(self) -> bool {
        self.kind.is_single_qubit()
    }

    /// Returns whether this operation is a two-qubit diagonal operation.
    #[must_use]
    pub const fn is_two_qubit(self) -> bool {
        self.kind.is_multi_qubit()
    }
}

/// Returns complete diagonal metadata for a gate.
#[must_use]
pub fn classify(gate: &Gate) -> Option<DiagonalInfo<'_>> {
    DiagonalInfo::from_gate(gate)
}

// =============================================================================
// Commutation
// =============================================================================

/// Returns whether two diagonal gates commute algebraically.
///
/// Every pair of computational-basis diagonal operators commute:
///
/// ```text
/// D1 D2 = D2 D1
/// ```
///
/// This function reports only the algebraic fact. It does not authorize
/// movement across circuit-level semantic boundaries such as measurement or
/// barriers.
#[must_use]
pub fn commute(left: &Gate, right: &Gate) -> bool {
    is_diagonal(left) && is_diagonal(right)
}

/// Returns whether two gates commute specifically because both are diagonal.
///
/// This is an explicit semantic alias useful to commutation analysis.
#[must_use]
pub fn diagonal_commute(left: &Gate, right: &Gate) -> bool {
    commute(left, right)
}

// =============================================================================
// Fusion compatibility
// =============================================================================

/// Returns whether two diagonal operations can be represented by one of the
/// canonical primitive gate forms without changing the surrounding circuit.
///
/// This is deliberately stricter than mathematical commutativity.
///
/// For example:
///
/// ```text
/// RZ(a) RZ(b) -> RZ(a+b)
/// ```
///
/// is directly representable.
///
/// But:
///
/// ```text
/// Z RZ(a)
/// ```
///
/// is mathematically fusible but is not represented by this function as a
/// direct same-kind fusion unless the rewrite layer explicitly chooses a
/// canonical representation.
///
/// This prevents this low-level algebra module from silently choosing a
/// global normal form.
#[must_use]
pub fn can_fuse(left: &Gate, right: &Gate) -> bool {
    if !is_diagonal(left) || !is_diagonal(right) {
        return false;
    }

    if left.qubits() != right.qubits() {
        return false;
    }

    matches!(
        (left.kind(), right.kind()),
        (GateKind::RZ, GateKind::RZ)
            | (GateKind::Phase, GateKind::Phase)
            | (GateKind::U1, GateKind::U1)
            | (GateKind::CRZ, GateKind::CRZ)
    )
}

/// Returns whether two gates have identical logical operands.
///
/// Operand ordering is intentionally significant for controlled operations.
#[must_use]
pub fn same_operands(left: &Gate, right: &Gate) -> bool {
    left.qubits() == right.qubits()
}

// =============================================================================
// Parameter combination
// =============================================================================

/// Constructs a canonical parameter representing `left + right`.
///
/// No floating-point approximation is introduced.
///
/// Constant + constant remains a constant.
///
/// Symbolic combinations become a canonical `ParameterExpression`.
pub fn add_parameters(
    left: &Parameter,
    right: &Parameter,
) -> DiagonalResult<Parameter> {
    if let (Some(left_value), Some(right_value)) =
        (left.as_constant(), right.as_constant())
    {
        let value = left_value + right_value;

        if !value.is_finite() {
            return Err(DiagonalError::NonFiniteAngle);
        }

        return Parameter::constant(value)
            .map_err(|_| DiagonalError::ParameterConstruction);
    }

    Parameter::expression(ParameterExpression::Add(
        Box::new(left.clone()),
        Box::new(right.clone()),
    ))
    .map_err(|_| DiagonalError::ParameterConstruction)
}

/// Constructs a canonical parameter representing `-value`.
pub fn negate_parameter(value: &Parameter) -> DiagonalResult<Parameter> {
    if let Some(constant) = value.as_constant() {
        let result = -constant;

        if !result.is_finite() {
            return Err(DiagonalError::NonFiniteAngle);
        }

        return Parameter::constant(result)
            .map_err(|_| DiagonalError::ParameterConstruction);
    }

    Parameter::expression(ParameterExpression::Negate(
        Box::new(value.clone()),
    ))
    .map_err(|_| DiagonalError::ParameterConstruction)
}

/// Constructs a canonical parameter representing `value - 2πk` only when
/// `value` is a concrete constant.
///
/// Symbolic parameters are returned unchanged because modulo reduction of an
/// arbitrary symbolic expression is not represented by the current canonical
/// parameter expression vocabulary.
pub fn normalize_parameter(value: &Parameter) -> DiagonalResult<Parameter> {
    let Some(angle) = value.as_constant() else {
        return Ok(value.clone());
    };

    if !angle.is_finite() {
        return Err(DiagonalError::NonFiniteAngle);
    }

    let normalized = normalize_angle(angle);

    Parameter::constant(normalized)
        .map_err(|_| DiagonalError::ParameterConstruction)
}

/// Normalizes a finite phase angle into the interval `[-π, π)`.
///
/// This operation is exact for representable floating-point arithmetic in the
/// sense that it only changes the periodic representative; it does not
/// approximate a symbolic expression.
#[must_use]
pub fn normalize_angle(angle: f64) -> f64 {
    if !angle.is_finite() {
        return angle;
    }

    let mut value = angle.rem_euclid(TWO_PI);

    if value >= PI {
        value -= TWO_PI;
    }

    // Avoid retaining a negative zero as a canonical representation.
    if value == 0.0 {
        0.0
    } else {
        value
    }
}

// =============================================================================
// Direct primitive fusion
// =============================================================================

/// Fuses two directly compatible diagonal gates.
///
/// Currently supported:
///
/// ```text
/// RZ(a) RZ(b)       -> RZ(a+b)
/// Phase(a) Phase(b) -> Phase(a+b)
/// U1(a) U1(b)       -> U1(a+b)
/// CRZ(a) CRZ(b)     -> CRZ(a+b)
/// ```
///
/// The original gates are not modified.
///
/// This function deliberately does not combine fixed phase gates such as `S`
/// and `T` into an arbitrary `RZ`, because selecting that representation is a
/// target/cost-model decision. That decision belongs to the higher-level
/// rewrite/synthesis subsystem.
pub fn fuse(left: &Gate, right: &Gate) -> DiagonalResult<Gate> {
    if !is_diagonal(left) {
        return Err(DiagonalError::NotDiagonal {
            gate: left.kind(),
        });
    }

    if !is_diagonal(right) {
        return Err(DiagonalError::NotDiagonal {
            gate: right.kind(),
        });
    }

    if left.qubits() != right.qubits() {
        return Err(DiagonalError::QubitMismatch);
    }

    let output_kind = match (left.kind(), right.kind()) {
        (GateKind::RZ, GateKind::RZ) => GateKind::RZ,
        (GateKind::Phase, GateKind::Phase) => GateKind::Phase,
        (GateKind::U1, GateKind::U1) => GateKind::U1,
        (GateKind::CRZ, GateKind::CRZ) => GateKind::CRZ,

        (left_kind, right_kind) => {
            return Err(DiagonalError::IncompatibleForFusion {
                left: left_kind,
                right: right_kind,
            });
        }
    };

    let left_parameter = left
        .parameters()
        .first()
        .ok_or(DiagonalError::MissingParameter)?;

    let right_parameter = right
        .parameters()
        .first()
        .ok_or(DiagonalError::MissingParameter)?;

    let parameter = add_parameters(left_parameter, right_parameter)?;

    Gate::new(
        output_kind,
        left.qubits().to_vec(),
        vec![parameter],
        None,
        None,
    )
    .map_err(DiagonalError::GateConstruction)
}

// =============================================================================
// Fixed phase conversion
// =============================================================================

/// Returns the exact phase angle represented by a fixed diagonal gate.
///
/// Parameterized gates return `None`.
#[must_use]
pub fn fixed_phase(gate: &Gate) -> Option<f64> {
    diagonal_kind(gate.kind())
        .and_then(DiagonalKind::fixed_angle)
}

/// Returns the fixed phase angle of a gate after normalization.
///
/// This is primarily useful for canonical rule matching.
#[must_use]
pub fn normalized_fixed_phase(gate: &Gate) -> Option<f64> {
    fixed_phase(gate).map(normalize_angle)
}

// =============================================================================
// Identity detection
// =============================================================================

/// Returns whether a diagonal gate is exactly the identity.
///
/// This detects syntactic identity gates directly.
///
/// It also detects concrete parameterized phase rotations whose angle is a
/// multiple of `2π`.
///
/// Symbolic expressions are never guessed to be zero.
#[must_use]
pub fn is_identity(gate: &Gate) -> bool {
    match gate.kind() {
        GateKind::I => true,

        GateKind::RZ
        | GateKind::Phase
        | GateKind::U1
        | GateKind::CRZ => {
            let Some(parameter) = gate.parameters().first() else {
                return false;
            };

            let Some(angle) = parameter.as_constant() else {
                return false;
            };

            is_zero_mod_period(angle)
        }

        GateKind::Z
        | GateKind::S
        | GateKind::Sdg
        | GateKind::T
        | GateKind::Tdg
        | GateKind::CZ => false,

        _ => false,
    }
}

/// Returns whether an angle is equivalent to zero modulo `2π`.
///
/// A small tolerance is deliberately used only for concrete floating-point
/// values. Symbolic expressions never reach this function.
#[must_use]
pub fn is_zero_mod_period(angle: f64) -> bool {
    if !angle.is_finite() {
        return false;
    }

    let reduced = angle.rem_euclid(TWO_PI);

    reduced == 0.0
        || (TWO_PI - reduced).abs() <=
            f64::EPSILON * 8.0_f64.max(angle.abs())
}

// =============================================================================
// Inverse phase semantics
// =============================================================================

/// Returns the inverse diagonal gate kind for fixed diagonal gates.
///
/// Parameterized gates whose inverse is obtained by negating the parameter
/// return the same gate kind through [`inverse_kind`].
#[must_use]
pub const fn inverse_kind(kind: GateKind) -> Option<GateKind> {
    match kind {
        GateKind::I => Some(GateKind::I),
        GateKind::Z => Some(GateKind::Z),
        GateKind::S => Some(GateKind::Sdg),
        GateKind::Sdg => Some(GateKind::S),
        GateKind::T => Some(GateKind::Tdg),
        GateKind::Tdg => Some(GateKind::T),
        GateKind::RZ => Some(GateKind::RZ),
        GateKind::Phase => Some(GateKind::Phase),
        GateKind::U1 => Some(GateKind::U1),
        GateKind::CZ => Some(GateKind::CZ),
        GateKind::CRZ => Some(GateKind::CRZ),

        _ => None,
    }
}

/// Returns whether the diagonal gate is mathematically self-inverse.
#[must_use]
pub fn is_self_inverse(gate: &Gate) -> bool {
    match gate.kind() {
        GateKind::I
        | GateKind::Z
        | GateKind::CZ => true,

        GateKind::RZ
        | GateKind::Phase
        | GateKind::U1
        | GateKind::CRZ => {
            let Some(parameter) = gate.parameters().first() else {
                return false;
            };

            let Some(angle) = parameter.as_constant() else {
                return false;
            };

            // U^2 = I iff 2θ ≡ 0 mod 2π.
            is_zero_mod_period(2.0 * angle)
        }

        GateKind::S
        | GateKind::Sdg
        | GateKind::T
        | GateKind::Tdg => false,

        _ => false,
    }
}

/// Returns the canonical parameter for the inverse of a parameterized
/// diagonal gate.
///
/// Fixed gates return `None`; their inverse is represented by another
/// `GateKind`.
pub fn inverse_parameter(gate: &Gate) -> DiagonalResult<Option<Parameter>> {
    match gate.kind() {
        GateKind::RZ
        | GateKind::Phase
        | GateKind::U1
        | GateKind::CRZ => {
            let parameter = gate
                .parameters()
                .first()
                .ok_or(DiagonalError::MissingParameter)?;

            Ok(Some(negate_parameter(parameter)?))
        }

        _ => Ok(None),
    }
}

// =============================================================================
// Canonical phase signature
// =============================================================================

/// Compact semantic signature for a diagonal gate.
///
/// The signature is intentionally not a matrix. It records the generator,
/// operand count, phase degree, and parameterization class needed by later
/// optimization layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagonalSignature {
    /// Semantic diagonal generator.
    pub kind: DiagonalKind,

    /// Number of logical operands.
    pub arity: usize,

    /// Boolean phase-polynomial degree.
    pub degree: usize,

    /// Whether the generator has a symbolic parameter.
    pub symbolic: bool,
}

/// Creates a compact signature for a diagonal gate.
#[must_use]
pub fn signature(gate: &Gate) -> Option<DiagonalSignature> {
    let info = classify(gate)?;

    Some(DiagonalSignature {
        kind: info.kind,
        arity: info.qubits.len(),
        degree: info.degree,
        symbolic: info.symbolic,
    })
}

// =============================================================================
// Diagonal region semantics
// =============================================================================

/// Returns whether an operation can participate in a diagonal-only region.
///
/// This is intentionally equivalent to diagonal classification rather than
/// checking whether the operation merely commutes with diagonal operations.
#[must_use]
pub fn is_diagonal_region_member(gate: &Gate) -> bool {
    is_diagonal(gate)
}

/// Returns whether a sequence consists entirely of computational-basis
/// diagonal operations.
///
/// The iterator is consumed exactly once and no collection is allocated.
pub fn is_diagonal_region<'a, I>(gates: I) -> bool
where
    I: IntoIterator<Item = &'a Gate>,
{
    gates.into_iter().all(is_diagonal)
}

// =============================================================================
// Diagonal resource characteristics
// =============================================================================

/// Resource characteristics useful to the generic optimizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagonalResources {
    /// Number of logical operands.
    pub arity: usize,

    /// Boolean phase degree.
    pub phase_degree: usize,

    /// Whether the operation has an arbitrary parameter.
    pub parameterized: bool,

    /// Whether the operation is symbolic.
    pub symbolic: bool,

    /// Whether the operation is fixed and exactly known.
    pub fixed: bool,
}

/// Returns resource characteristics for a diagonal gate.
#[must_use]
pub fn resources(gate: &Gate) -> Option<DiagonalResources> {
    let info = classify(gate)?;

    Some(DiagonalResources {
        arity: info.qubits.len(),
        phase_degree: info.degree,
        parameterized: info.is_parameterized(),
        symbolic: info.symbolic,
        fixed: !info.is_parameterized(),
    })
}

// =============================================================================
// Controlled-phase semantics
// =============================================================================

/// Returns whether a diagonal gate represents a two-qubit controlled phase.
#[must_use]
pub const fn is_controlled_phase(kind: GateKind) -> bool {
    matches!(kind, GateKind::CZ | GateKind::CRZ)
}

/// Returns whether a diagonal gate represents an unconditional single-qubit
/// phase.
#[must_use]
pub const fn is_single_qubit_phase(kind: GateKind) -> bool {
    matches!(
        kind,
        GateKind::Z
            | GateKind::S
            | GateKind::Sdg
            | GateKind::T
            | GateKind::Tdg
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
    )
}

// =============================================================================
// Computational-basis phase semantics
// =============================================================================

/// Returns the phase multiplier for a fixed single-qubit diagonal gate on a
/// computational-basis state.
///
/// `bit` must be either `0` or `1`.
///
/// The result is represented as an angle in radians rather than as a complex
/// number. This avoids unnecessary complex allocation and preserves the
/// algebraic phase representation used by later optimizers.
///
/// For example:
///
/// ```text
/// Z |0> = exp(i*0) |0>
/// Z |1> = exp(i*π) |1>
/// ```
#[must_use]
pub fn basis_phase_angle(
    kind: GateKind,
    bit: bool,
) -> Option<f64> {
    let angle = match kind {
        GateKind::I => 0.0,

        GateKind::Z => {
            if bit {
                PI
            } else {
                0.0
            }
        }

        GateKind::S => {
            if bit {
                HALF_PI
            } else {
                0.0
            }
        }

        GateKind::Sdg => {
            if bit {
                -HALF_PI
            } else {
                0.0
            }
        }

        GateKind::T => {
            if bit {
                QUARTER_PI
            } else {
                0.0
            }
        }

        GateKind::Tdg => {
            if bit {
                -QUARTER_PI
            } else {
                0.0
            }
        }

        _ => return None,
    };

    Some(angle)
}

/// Returns the phase angle contributed by a fixed controlled-Z operation for a
/// computational-basis pair.
///
/// The returned phase is non-zero only for `|11>`.
#[must_use]
pub fn controlled_z_phase_angle(
    kind: GateKind,
    control: bool,
    target: bool,
) -> Option<f64> {
    match kind {
        GateKind::CZ => {
            if control && target {
                Some(PI)
            } else {
                Some(0.0)
            }
        }

        _ => None,
    }
}

// =============================================================================
// Global-phase awareness
// =============================================================================

/// Returns whether a diagonal gate can introduce a non-trivial global phase.
///
/// The answer is conservative.
///
/// `I` and all standard phase/rotation gates represented here preserve the
/// `|0...0>` amplitude exactly, so they do not introduce a standalone global
/// phase under the canonical computational-basis representation.
///
/// This function exists because future diagonal generators may include gates
/// whose canonical representation carries an explicit global phase.
#[must_use]
pub const fn has_explicit_global_phase(kind: GateKind) -> bool {
    match kind {
        GateKind::I
        | GateKind::Z
        | GateKind::S
        | GateKind::Sdg
        | GateKind::T
        | GateKind::Tdg
        | GateKind::RZ
        | GateKind::Phase
        | GateKind::U1
        | GateKind::CZ
        | GateKind::CRZ => false,

        _ => false,
    }
}

// =============================================================================
// Validation
// =============================================================================

/// Validates that a gate is a well-formed diagonal operation according to the
/// current canonical IR gate vocabulary.
///
/// Canonical gate validation is still delegated to `Gate::new`/IR validation;
/// this function only validates diagonal-specific structural requirements.
pub fn validate(gate: &Gate) -> DiagonalResult<()> {
    let Some(kind) = diagonal_kind(gate.kind()) else {
        return Err(DiagonalError::NotDiagonal {
            gate: gate.kind(),
        });
    };

    if gate.qubits().len() != kind.arity() {
        return Err(DiagonalError::UnsupportedRepresentation {
            gate: gate.kind(),
        });
    }

    if kind.is_parameterized() {
        if gate.parameters().len() != 1 {
            return Err(DiagonalError::MissingParameter);
        }

        gate.parameters()[0]
            .validate()
            .map_err(|_| DiagonalError::ParameterConstruction)?;
    } else if !gate.parameters().is_empty() {
        return Err(DiagonalError::UnsupportedRepresentation {
            gate: gate.kind(),
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::QubitId;

    fn q(index: usize) -> QubitId {
        QubitId::new(index).expect("test qubit id must be valid")
    }

    fn constant(value: f64) -> Parameter {
        Parameter::constant(value).expect("test parameter must be finite")
    }

    fn gate(kind: GateKind, qubits: Vec<QubitId>) -> Gate {
        Gate::new(
            kind,
            qubits,
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    fn parameterized(
        kind: GateKind,
        qubits: Vec<QubitId>,
        value: f64,
    ) -> Gate {
        Gate::new(
            kind,
            qubits,
            vec![constant(value)],
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    #[test]
    fn fixed_diagonal_gates_are_classified() {
        assert!(is_diagonal(&gate(
            GateKind::I,
            vec![q(0)]
        )));

        assert!(is_diagonal(&gate(
            GateKind::Z,
            vec![q(0)]
        )));

        assert!(is_diagonal(&gate(
            GateKind::S,
            vec![q(0)]
        )));

        assert!(is_diagonal(&gate(
            GateKind::Sdg,
            vec![q(0)]
        )));

        assert!(is_diagonal(&gate(
            GateKind::T,
            vec![q(0)]
        )));

        assert!(is_diagonal(&gate(
            GateKind::Tdg,
            vec![q(0)]
        )));

        assert!(is_diagonal(&gate(
            GateKind::CZ,
            vec![q(0), q(1)]
        )));
    }

    #[test]
    fn_non_diagonal_gates_are_rejected() {
        assert!(!is_diagonal(&gate(
            GateKind::X,
            vec![q(0)]
        )));

        assert!(!is_diagonal(&gate(
            GateKind::H,
            vec![q(0)]
        )));

        assert!(!is_diagonal(&gate(
            GateKind::CX,
            vec![q(0), q(1)]
        )));
    }

    #[test]
    fn parameterized_diagonal_gates_are_classified() {
        for kind in [
            GateKind::RZ,
            GateKind::Phase,
            GateKind::U1,
            GateKind::CRZ,
        ] {
            let operands = if kind == GateKind::CRZ {
                vec![q(0), q(1)]
            } else {
                vec![q(0)]
            };

            assert!(is_diagonal(&parameterized(
                kind,
                operands,
                0.25,
            )));
        }
    }

    #[test]
    fn diagonal_gates_commute() {
        let left = parameterized(
            GateKind::RZ,
            vec![q(0)],
            0.5,
        );

        let right = gate(
            GateKind::Z,
            vec![q(0)],
        );

        assert!(commute(&left, &right));
    }

    #[test]
    fn incompatible_gates_do_not_fuse() {
        let left = gate(
            GateKind::Z,
            vec![q(0)],
        );

        let right = parameterized(
            GateKind::RZ,
            vec![q(0)],
            0.5,
        );

        assert!(!can_fuse(&left, &right));
    }

    #[test]
    fn same_rz_gates_fuse() {
        let left = parameterized(
            GateKind::RZ,
            vec![q(0)],
            0.25,
        );

        let right = parameterized(
            GateKind::RZ,
            vec![q(0)],
            0.5,
        );

        assert!(can_fuse(&left, &right));

        let fused =
            fuse(&left, &right).expect("RZ gates should fuse");

        assert_eq!(fused.kind(), GateKind::RZ);
        assert_eq!(
            fused.parameters()[0].as_constant(),
            Some(0.75)
        );
    }

    #[test]
    fn symbolic_parameters_are_preserved() {
        let a = Parameter::symbol("a")
            .expect("valid symbol");

        let b = Parameter::symbol("b")
            .expect("valid symbol");

        let fused_parameter =
            add_parameters(&a, &b)
                .expect("symbolic addition should work");

        assert!(fused_parameter.is_symbolic());
    }

    #[test]
    fn angle_normalization_is_periodic() {
        assert_eq!(
            normalize_angle(0.0),
            0.0
        );

        assert_eq!(
            normalize_angle(TWO_PI),
            0.0
        );

        assert_eq!(
            normalize_angle(-TWO_PI),
            0.0
        );

        assert!(
            normalize_angle(3.0 * PI) < PI
        );
    }

    #[test]
    fn fixed_phase_values_are_correct() {
        assert_eq!(
            fixed_phase(&gate(
                GateKind::Z,
                vec![q(0)]
            )),
            Some(PI)
        );

        assert_eq!(
            fixed_phase(&gate(
                GateKind::S,
                vec![q(0)]
            )),
            Some(HALF_PI)
        );

        assert_eq!(
            fixed_phase(&gate(
                GateKind::T,
                vec![q(0)]
            )),
            Some(QUARTER_PI)
        );
    }

    #[test]
    fn fixed_phase_basis_values_are_correct() {
        assert_eq!(
            basis_phase_angle(
                GateKind::Z,
                false
            ),
            Some(0.0)
        );

        assert_eq!(
            basis_phase_angle(
                GateKind::Z,
                true
            ),
            Some(PI)
        );

        assert_eq!(
            basis_phase_angle(
                GateKind::T,
                true
            ),
            Some(QUARTER_PI)
        );
    }

    #[test]
    fn controlled_z_only_phases_11() {
        assert_eq!(
            controlled_z_phase_angle(
                GateKind::CZ,
                false,
                false
            ),
            Some(0.0)
        );

        assert_eq!(
            controlled_z_phase_angle(
                GateKind::CZ,
                true,
                false
            ),
            Some(0.0)
        );

        assert_eq!(
            controlled_z_phase_angle(
                GateKind::CZ,
                false,
                true
            ),
            Some(0.0)
        );

        assert_eq!(
            controlled_z_phase_angle(
                GateKind::CZ,
                true,
                true
            ),
            Some(PI)
        );
    }

    #[test]
    fn identity_is_detected_for_zero_rotation() {
        let identity =
            parameterized(
                GateKind::RZ,
                vec![q(0)],
                0.0,
            );

        assert!(is_identity(&identity));
    }

    #[test]
    fn identity_is_detected_modulo_two_pi() {
        let identity =
            parameterized(
                GateKind::RZ,
                vec![q(0)],
                TWO_PI,
            );

        assert!(is_identity(&identity));
    }

    #[test]
    fn symbolic_identity_is_not_guessed() {
        let parameter =
            Parameter::symbol("theta")
                .expect("valid symbol");

        let gate = Gate::new(
            GateKind::RZ,
            vec![q(0)],
            vec![parameter],
            None,
            None,
        )
        .expect("valid symbolic RZ");

        assert!(!is_identity(&gate));
    }

    #[test]
    fn_parameterized_inverse_negates_parameter() {
        let original =
            parameterized(
                GateKind::RZ,
                vec![q(0)],
                0.75,
            );

        let inverse =
            inverse_parameter(&original)
                .expect("inverse parameter should work")
                .expect("RZ has an inverse parameter");

        assert_eq!(
            inverse.as_constant(),
            Some(-0.75)
        );
    }

    #[test]
    fn fixed_gate_inverse_kind_is_correct() {
        assert_eq!(
            inverse_kind(GateKind::S),
            Some(GateKind::Sdg)
        );

        assert_eq!(
            inverse_kind(GateKind::T),
            Some(GateKind::Tdg)
        );

        assert_eq!(
            inverse_kind(GateKind::CZ),
            Some(GateKind::CZ)
        );
    }

    #[test]
    fn diagonal_degree_is_correct() {
        assert_eq!(
            phase_degree(GateKind::RZ),
            Some(1)
        );

        assert_eq!(
            phase_degree(GateKind::CZ),
            Some(2)
        );

        assert_eq!(
            phase_degree(GateKind::CRZ),
            Some(2)
        );

        assert_eq!(
            phase_degree(GateKind::CX),
            None
        );
    }

    #[test]
    fn diagonal_region_requires_all_diagonal() {
        let first =
            gate(GateKind::Z, vec![q(0)]);

        let second =
            parameterized(
                GateKind::RZ,
                vec![q(0)],
                0.25,
            );

        let third =
            gate(GateKind::H, vec![q(0)]);

        assert!(is_diagonal_region([
            &first,
            &second,
        ]));

        assert!(!is_diagonal_region([
            &first,
            &second,
            &third,
        ]));
    }

    #[test]
    fn self_inverse_detection_is_exact_for_fixed_gates() {
        assert!(is_self_inverse(
            &gate(
                GateKind::Z,
                vec![q(0)]
            )
        ));

        assert!(is_self_inverse(
            &gate(
                GateKind::CZ,
                vec![q(0), q(1)]
            )
        ));

        assert!(!is_self_inverse(
            &gate(
                GateKind::T,
                vec![q(0)]
            )
        ));
    }

    #[test]
    fn parameterized_self_inverse_detection_is_value_sensitive() {
        let gate =
            parameterized(
                GateKind::RZ,
                vec![q(0)],
                PI,
            );

        assert!(is_self_inverse(&gate));
    }

    #[test]
    fn signature_is_compact_and_deterministic() {
        let gate =
            parameterized(
                GateKind::CRZ,
                vec![q(0), q(1)],
                0.5,
            );

        let signature =
            signature(&gate)
                .expect("CRZ should have a signature");

        assert_eq!(
            signature.kind,
            DiagonalKind::CRZ
        );

        assert_eq!(
            signature.arity,
            2
        );

        assert_eq!(
            signature.degree,
            2
        );

        assert!(!signature.symbolic);
    }

    #[test]
    fn resource_metadata_is_correct() {
        let gate =
            parameterized(
                GateKind::CRZ,
                vec![q(0), q(1)],
                0.5,
            );

        let resources =
            resources(&gate)
                .expect("CRZ should have resources");

        assert_eq!(resources.arity, 2);
        assert_eq!(resources.phase_degree, 2);
        assert!(resources.parameterized);
        assert!(!resources.symbolic);
        assert!(!resources.fixed);
    }

    #[test]
    fn validation_accepts_valid_diagonal_gate() {
        let gate =
            parameterized(
                GateKind::RZ,
                vec![q(0)],
                0.5,
            );

        assert!(validate(&gate).is_ok());
    }

    #[test]
    fn validation_rejects_non_diagonal_gate() {
        let gate =
            gate(
                GateKind::H,
                vec![q(0)]
            );

        assert!(matches!(
            validate(&gate),
            Err(DiagonalError::NotDiagonal {
                gate: GateKind::H
            })
        ));
    }
}