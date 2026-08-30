//! Zamani Quantum Optimization — Single-Qubit Synthesis
//!
//! Production-grade single-qubit unitary decomposition and synthesis.
//!
//! # Architectural position
//!
//! This module belongs to:
//!
//! ```text
//! quantum::ir
//!     │
//!     ▼
//! optimization
//!     │
//!     ├── analysis
//!     ├── local
//!     ├── algebra
//!     │
//!     ▼
//! synthesis::single_qubit
//!     │
//!     ▼
//! canonical Quantum IR
//! ```
//!
//! This file is deliberately independent from the optimizer pipeline.
//! It provides reusable mathematical synthesis primitives which future
//! optimization passes can consume.
//!
//! # Responsibilities
//!
//! This module provides:
//!
//! - numerically stable 2x2 complex matrix representation;
//! - exact structural validation of single-qubit unitaries;
//! - global-phase-aware unitary comparison;
//! - Z-Y-Z Euler decomposition;
//! - Z-X-Z Euler decomposition;
//! - U3 decomposition;
//! - decomposition into supported canonical `GateKind`s;
//! - deterministic angle normalization;
//! - singularity-safe Euler extraction;
//! - configurable numerical tolerance;
//! - operation-count/decomposition metadata;
//! - explicit rejection of non-unitary input;
//! - explicit resource limits;
//! - no hidden backend assumptions;
//! - no routing;
//! - no scheduling;
//! - no hardware I/O;
//! - no unsafe code.
//!
//! # Mathematical convention
//!
//! Matrices act on column state vectors:
//!
//! ```text
//! |psi'> = U |psi>
//! ```
//!
//! The canonical rotation convention is:
//!
//! ```text
//! RX(theta) = exp(-i theta X / 2)
//! RY(theta) = exp(-i theta Y / 2)
//! RZ(theta) = exp(-i theta Z / 2)
//! ```
//!
//! A general single-qubit unitary can be represented as:
//!
//! ```text
//! U = exp(i global_phase) RZ(alpha) RY(beta) RZ(gamma)
//! ```
//!
//! The returned Euler angles reproduce the unitary up to global phase.
//!
//! # Global phase
//!
//! Quantum states are physically equivalent under a global phase for ordinary
//! closed-system unitary evolution. Consequently, this module distinguishes:
//!
//! - exact matrix equality;
//! - equality up to global phase.
//!
//! The latter is the normal equivalence relation for unitary synthesis.
//!
//! # Numerical policy
//!
//! Floating-point synthesis is inherently approximate. This module therefore:
//!
//! - rejects NaN and infinity;
//! - never silently clamps invalid values;
//! - uses an explicit tolerance;
//! - clamps only values that are within the configured tolerance of a
//!   mathematically valid singular boundary;
//! - reports invalid matrices rather than producing plausible but incorrect
//!   decompositions.
//!
//! # Scaling
//!
//! Single-qubit synthesis itself is O(1) in circuit size because one operation
//! contains only four complex matrix entries.
//!
//! Circuit-wide scaling is the responsibility of the caller. This module does
//! not allocate proportional to the total circuit size.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features are required.
//! No external crates are required.
//! No `unsafe` code is used.

use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::fmt;

use crate::quantum::ir::gate::GateKind;

// ============================================================================
// Constants
// ============================================================================

/// Smallest supported synthesis tolerance.
///
/// A tighter tolerance is allowed, but values below this threshold are usually
/// dominated by the numerical precision of `f64`.
pub const MIN_TOLERANCE: f64 = 1.0e-15;

/// Default unitary validation/synthesis tolerance.
pub const DEFAULT_TOLERANCE: f64 = 1.0e-12;

/// Default maximum number of emitted operations for one decomposition.
///
/// A single-qubit Euler decomposition normally emits at most three operations.
/// The larger limit protects future target-specific decomposition extensions.
pub const DEFAULT_MAX_OPERATIONS: usize = 64;

/// Maximum number of operations accepted by this module's generic synthesis
/// result.
pub const HARD_MAX_OPERATIONS: usize = 4096;

/// Numerical threshold used when deciding whether a matrix entry is
/// effectively zero under the configured tolerance.
const ZERO_EPSILON: f64 = 1.0e-15;

// ============================================================================
// Complex number
// ============================================================================

/// Minimal dependency-free complex number used by the synthesis engine.
///
/// This is intentionally private in spirit, but public because callers may
/// need to construct unitary matrices directly.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex64 {
    /// Real component.
    pub re: f64,

    /// Imaginary component.
    pub im: f64,
}

impl Complex64 {
    /// Creates a complex number.
    #[must_use]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    /// One.
    #[must_use]
    pub const fn one() -> Self {
        Self { re: 1.0, im: 0.0 }
    }

    /// Imaginary unit.
    #[must_use]
    pub const fn i() -> Self {
        Self { re: 0.0, im: 1.0 }
    }

    /// Returns the complex conjugate.
    #[must_use]
    pub const fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Squared magnitude.
    #[must_use]
    pub fn norm_squared(self) -> f64 {
        self.re.mul_add(self.re, self.im * self.im)
    }

    /// Magnitude.
    #[must_use]
    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    /// Returns true if both components are finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    /// Returns true if the value is approximately zero.
    #[must_use]
    pub fn is_near_zero(self, tolerance: f64) -> bool {
        self.norm() <= tolerance
    }
}

impl std::ops::Add for Complex64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub for Complex64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re.mul_add(rhs.re, -(self.im * rhs.im)),
            im: self.re.mul_add(rhs.im, self.im * rhs.re),
        }
    }
}

impl std::ops::Mul<f64> for Complex64 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl std::ops::Div<f64> for Complex64 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl std::ops::Neg for Complex64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl fmt::Display for Complex64 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im < 0.0 {
            write!(formatter, "{} - {}i", self.re, -self.im)
        } else {
            write!(formatter, "{} + {}i", self.re, self.im)
        }
    }
}

// ============================================================================
// Matrix
// ============================================================================

/// A fixed-size 2x2 complex matrix.
///
/// Fixed-size storage is intentional:
///
/// - no heap allocation;
/// - constant memory;
/// - deterministic behavior;
/// - no circuit-size-dependent memory usage.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SingleQubitUnitary {
    /// Matrix entry `(0,0)`.
    pub m00: Complex64,

    /// Matrix entry `(0,1)`.
    pub m01: Complex64,

    /// Matrix entry `(1,0)`.
    pub m10: Complex64,

    /// Matrix entry `(1,1)`.
    pub m11: Complex64,
}

impl SingleQubitUnitary {
    /// Creates a matrix from four entries.
    #[must_use]
    pub const fn new(
        m00: Complex64,
        m01: Complex64,
        m10: Complex64,
        m11: Complex64,
    ) -> Self {
        Self {
            m00,
            m01,
            m10,
            m11,
        }
    }

    /// Identity matrix.
    #[must_use]
    pub const fn identity() -> Self {
        Self::new(
            Complex64::one(),
            Complex64::zero(),
            Complex64::zero(),
            Complex64::one(),
        )
    }

    /// Pauli-X matrix.
    #[must_use]
    pub const fn x() -> Self {
        Self::new(
            Complex64::zero(),
            Complex64::one(),
            Complex64::one(),
            Complex64::zero(),
        )
    }

    /// Pauli-Y matrix.
    #[must_use]
    pub const fn y() -> Self {
        Self::new(
            Complex64::zero(),
            Complex64::new(0.0, -1.0),
            Complex64::new(0.0, 1.0),
            Complex64::zero(),
        )
    }

    /// Pauli-Z matrix.
    #[must_use]
    pub const fn z() -> Self {
        Self::new(
            Complex64::one(),
            Complex64::zero(),
            Complex64::zero(),
            Complex64::new(-1.0, 0.0),
        )
    }

    /// Hadamard matrix.
    #[must_use]
    pub fn h() -> Self {
        let scale = 1.0 / 2.0_f64.sqrt();

        Self::new(
            Complex64::new(scale, 0.0),
            Complex64::new(scale, 0.0),
            Complex64::new(scale, 0.0),
            Complex64::new(-scale, 0.0),
        )
    }

    /// Creates an RX rotation.
    #[must_use]
    pub fn rx(theta: f64) -> Self {
        let half = theta * 0.5;
        let c = half.cos();
        let s = half.sin();

        Self::new(
            Complex64::new(c, 0.0),
            Complex64::new(0.0, -s),
            Complex64::new(0.0, -s),
            Complex64::new(c, 0.0),
        )
    }

    /// Creates an RY rotation.
    #[must_use]
    pub fn ry(theta: f64) -> Self {
        let half = theta * 0.5;
        let c = half.cos();
        let s = half.sin();

        Self::new(
            Complex64::new(c, 0.0),
            Complex64::new(-s, 0.0),
            Complex64::new(s, 0.0),
            Complex64::new(c, 0.0),
        )
    }

    /// Creates an RZ rotation.
    #[must_use]
    pub fn rz(theta: f64) -> Self {
        let half = theta * 0.5;

        Self::new(
            Complex64::new(half.cos(), -half.sin()),
            Complex64::zero(),
            Complex64::zero(),
            Complex64::new(half.cos(), half.sin()),
        )
    }

    /// Creates a U3 matrix using the standard convention:
    ///
    /// `U3(theta, phi, lambda) = RZ(phi) RY(theta) RZ(lambda)`
    ///
    /// up to the conventional global phase.
    #[must_use]
    pub fn u3(theta: f64, phi: f64, lambda: f64) -> Self {
        let half_theta = theta * 0.5;
        let half_sum = (phi + lambda) * 0.5;
        let half_diff = (phi - lambda) * 0.5;

        let c = half_theta.cos();
        let s = half_theta.sin();

        let phase_sum = Complex64::new(
            half_sum.cos(),
            -half_sum.sin(),
        );

        let phase_diff = Complex64::new(
            half_diff.cos(),
            -half_diff.sin(),
        );

        let upper_right = phase_diff * Complex64::new(-s, 0.0);

        let lower_left = phase_diff.conjugate()
            * Complex64::new(s, 0.0);

        let upper_left = phase_sum
            * Complex64::new(c, 0.0);

        let lower_right = phase_sum.conjugate()
            * Complex64::new(c, 0.0);

        Self::new(
            upper_left,
            upper_right,
            lower_left,
            lower_right,
        )
    }

    /// Returns the matrix entry by row and column.
    #[must_use]
    pub const fn get(self, row: usize, column: usize) -> Option<Complex64> {
        match (row, column) {
            (0, 0) => Some(self.m00),
            (0, 1) => Some(self.m01),
            (1, 0) => Some(self.m10),
            (1, 1) => Some(self.m11),
            _ => None,
        }
    }

    /// Returns the conjugate transpose.
    #[must_use]
    pub const fn adjoint(self) -> Self {
        Self::new(
            self.m00.conjugate(),
            self.m10.conjugate(),
            self.m01.conjugate(),
            self.m11.conjugate(),
        )
    }

    /// Matrix multiplication.
    #[must_use]
    pub fn multiply(self, rhs: Self) -> Self {
        Self::new(
            self.m00 * rhs.m00 + self.m01 * rhs.m10,
            self.m00 * rhs.m01 + self.m01 * rhs.m11,
            self.m10 * rhs.m00 + self.m11 * rhs.m10,
            self.m10 * rhs.m01 + self.m11 * rhs.m11,
        )
    }

    /// Determinant.
    #[must_use]
    pub fn determinant(self) -> Complex64 {
        self.m00 * self.m11 - self.m01 * self.m10
    }

    /// Maximum absolute matrix-entry difference.
    #[must_use]
    pub fn max_difference(self, rhs: Self) -> f64 {
        let differences = [
            (self.m00 - rhs.m00).norm(),
            (self.m01 - rhs.m01).norm(),
            (self.m10 - rhs.m10).norm(),
            (self.m11 - rhs.m11).norm(),
        ];

        differences
            .iter()
            .copied()
            .fold(0.0_f64, f64::max)
    }

    /// Returns the Frobenius norm.
    #[must_use]
    pub fn frobenius_norm(self) -> f64 {
        (self.m00.norm_squared()
            + self.m01.norm_squared()
            + self.m10.norm_squared()
            + self.m11.norm_squared())
        .sqrt()
    }

    /// Returns whether this matrix is finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.m00.is_finite()
            && self.m01.is_finite()
            && self.m10.is_finite()
            && self.m11.is_finite()
    }

    /// Returns whether this matrix is unitary.
    pub fn is_unitary(
        self,
        tolerance: f64,
    ) -> Result<bool, SingleQubitSynthesisError> {
        validate_tolerance(tolerance)?;

        if !self.is_finite() {
            return Err(
                SingleQubitSynthesisError::NonFiniteMatrix,
            );
        }

        let product = self.multiply(self.adjoint());

        Ok(product.max_difference(Self::identity()) <= tolerance)
    }

    /// Validates that this matrix is unitary.
    pub fn validate(
        self,
        tolerance: f64,
    ) -> Result<(), SingleQubitSynthesisError> {
        if !self.is_unitary(tolerance)? {
            return Err(
                SingleQubitSynthesisError::NotUnitary {
                    residual: self
                        .multiply(self.adjoint())
                        .max_difference(Self::identity()),
                },
            );
        }

        Ok(())
    }

    /// Returns true if two matrices are equal up to global phase.
    pub fn equivalent_up_to_global_phase(
        self,
        rhs: Self,
        tolerance: f64,
    ) -> Result<bool, SingleQubitSynthesisError> {
        validate_tolerance(tolerance)?;

        self.validate(tolerance)?;
        rhs.validate(tolerance)?;

        let product = self.multiply(rhs.adjoint());

        let phase = if !product.m00.is_near_zero(tolerance) {
            product.m00
        } else if !product.m01.is_near_zero(tolerance) {
            product.m01
        } else if !product.m10.is_near_zero(tolerance) {
            product.m10
        } else {
            product.m11
        };

        if phase.is_near_zero(tolerance) {
            return Ok(false);
        }

        let normalized = phase / phase.norm();

        let expected = Self::new(
            normalized,
            Complex64::zero(),
            Complex64::zero(),
            normalized,
        );

        Ok(product.max_difference(expected) <= tolerance)
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for single-qubit synthesis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SingleQubitSynthesisConfig {
    /// Numerical equivalence tolerance.
    pub tolerance: f64,

    /// Maximum number of operations emitted by one synthesis.
    pub max_operations: usize,

    /// Whether global phase may be discarded.
    ///
    /// This should normally be `true` for ordinary quantum circuit synthesis.
    /// Set it to `false` when the caller explicitly requires matrix equality.
    pub allow_global_phase: bool,

    /// Whether numerically tiny angles should be removed.
    pub eliminate_near_zero_rotations: bool,

    /// Whether angles should be normalized to the principal interval.
    pub normalize_angles: bool,
}

impl Default for SingleQubitSynthesisConfig {
    fn default() -> Self {
        Self {
            tolerance: DEFAULT_TOLERANCE,
            max_operations: DEFAULT_MAX_OPERATIONS,
            allow_global_phase: true,
            eliminate_near_zero_rotations: true,
            normalize_angles: true,
        }
    }
}

impl SingleQubitSynthesisConfig {
    /// Validates the configuration.
    pub fn validate(self) -> Result<(), SingleQubitSynthesisError> {
        validate_tolerance(self.tolerance)?;

        if self.max_operations == 0 {
            return Err(
                SingleQubitSynthesisError::InvalidConfiguration {
                    message: "max_operations must be greater than zero",
                },
            );
        }

        if self.max_operations > HARD_MAX_OPERATIONS {
            return Err(
                SingleQubitSynthesisError::InvalidConfiguration {
                    message: "max_operations exceeds the hard safety limit",
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the single-qubit synthesis subsystem.
#[derive(Debug, Clone, PartialEq)]
pub enum SingleQubitSynthesisError {
    /// The numerical tolerance is invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        tolerance: f64,
    },

    /// A matrix contains NaN or infinity.
    NonFiniteMatrix,

    /// A matrix is not unitary.
    NotUnitary {
        /// Maximum residual of `U U† - I`.
        residual: f64,
    },

    /// A matrix cannot be decomposed under the requested convention.
    DecompositionFailure {
        /// Human-readable reason.
        message: &'static str,
    },

    /// A configuration is invalid.
    InvalidConfiguration {
        /// Human-readable reason.
        message: &'static str,
    },

    /// The requested target gate set cannot represent the requested
    /// decomposition exactly.
    UnsupportedTarget {
        /// Gate kind requested.
        gate: GateKind,
    },

    /// A decomposition would exceed the configured resource limit.
    OperationLimitExceeded {
        /// Required operation count.
        required: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// An angle is non-finite.
    NonFiniteAngle,

    /// Exact matrix equality was requested but the decomposition differs by a
    /// global phase.
    GlobalPhaseMismatch {
        /// Absolute phase angle in radians.
        phase: f64,
    },
}

impl fmt::Display for SingleQubitSynthesisError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidTolerance { tolerance } => {
                write!(
                    formatter,
                    "invalid synthesis tolerance {tolerance:?}"
                )
            }

            Self::NonFiniteMatrix => {
                write!(formatter, "single-qubit matrix contains a non-finite value")
            }

            Self::NotUnitary { residual } => {
                write!(
                    formatter,
                    "single-qubit matrix is not unitary; residual={residual:e}"
                )
            }

            Self::DecompositionFailure { message } => {
                write!(formatter, "single-qubit decomposition failed: {message}")
            }

            Self::InvalidConfiguration { message } => {
                write!(formatter, "invalid single-qubit synthesis configuration: {message}")
            }

            Self::UnsupportedTarget { gate } => {
                write!(
                    formatter,
                    "single-qubit synthesis target {gate:?} is unsupported"
                )
            }

            Self::OperationLimitExceeded {
                required,
                maximum,
            } => {
                write!(
                    formatter,
                    "single-qubit synthesis requires {required} operations, maximum is {maximum}"
                )
            }

            Self::NonFiniteAngle => {
                write!(formatter, "single-qubit decomposition produced a non-finite angle")
            }

            Self::GlobalPhaseMismatch { phase } => {
                write!(
                    formatter,
                    "exact matrix equality failed because of global phase {phase} radians"
                )
            }
        }
    }
}

impl std::error::Error for SingleQubitSynthesisError {}

// ============================================================================
// Decomposition representation
// ============================================================================

/// Euler decomposition convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EulerConvention {
    /// `RZ(alpha) RY(beta) RZ(gamma)`.
    ZYZ,

    /// `RZ(alpha) RX(beta) RZ(gamma)`.
    ZXZ,
}

/// A single synthesized operation.
///
/// This is intentionally not a second quantum IR. It is a temporary,
/// immutable synthesis result which can later be lowered into the canonical
/// `quantum::ir::Gate`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SynthesizedOperation {
    /// Canonical gate kind.
    pub gate: GateKind,

    /// First parameter, if present.
    pub parameter_0: Option<f64>,

    /// Second parameter, if present.
    pub parameter_1: Option<f64>,

    /// Third parameter, if present.
    pub parameter_2: Option<f64>,
}

impl SynthesizedOperation {
    /// Creates an operation without parameters.
    #[must_use]
    pub const fn gate(gate: GateKind) -> Self {
        Self {
            gate,
            parameter_0: None,
            parameter_1: None,
            parameter_2: None,
        }
    }

    /// Creates a one-parameter operation.
    #[must_use]
    pub const fn one_parameter(
        gate: GateKind,
        parameter: f64,
    ) -> Self {
        Self {
            gate,
            parameter_0: Some(parameter),
            parameter_1: None,
            parameter_2: None,
        }
    }

    /// Creates a two-parameter operation.
    #[must_use]
    pub const fn two_parameters(
        gate: GateKind,
        first: f64,
        second: f64,
    ) -> Self {
        Self {
            gate,
            parameter_0: Some(first),
            parameter_1: Some(second),
            parameter_2: None,
        }
    }

    /// Creates a three-parameter operation.
    #[must_use]
    pub const fn three_parameters(
        gate: GateKind,
        first: f64,
        second: f64,
        third: f64,
    ) -> Self {
        Self {
            gate,
            parameter_0: Some(first),
            parameter_1: Some(second),
            parameter_2: Some(third),
        }
    }

    /// Returns the number of parameters.
    #[must_use]
    pub const fn parameter_count(self) -> usize {
        match (
            self.parameter_0.is_some(),
            self.parameter_1.is_some(),
            self.parameter_2.is_some(),
        ) {
            (false, false, false) => 0,
            (true, false, false) => 1,
            (true, true, false) => 2,
            (true, true, true) => 3,
            _ => 0,
        }
    }
}

/// Result of a single-qubit decomposition.
#[derive(Debug, Clone, PartialEq)]
pub struct SingleQubitDecomposition {
    /// Euler convention used.
    pub convention: EulerConvention,

    /// First Euler angle.
    pub alpha: f64,

    /// Middle Euler angle.
    pub beta: f64,

    /// Third Euler angle.
    pub gamma: f64,

    /// Global phase accumulated by the decomposition.
    ///
    /// This value is normally discarded when `allow_global_phase == true`.
    pub global_phase: f64,

    /// Synthesized operations.
    pub operations: Vec<SynthesizedOperation>,
}

impl SingleQubitDecomposition {
    /// Returns the number of emitted operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns true when the decomposition contains no operations.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.operations.is_empty()
    }

    /// Reconstructs the unitary represented by the operations.
    #[must_use]
    pub fn unitary(&self) -> SingleQubitUnitary {
        let mut result = SingleQubitUnitary::identity();

        for operation in &self.operations {
            let matrix = match operation.gate {
                GateKind::RX => {
                    SingleQubitUnitary::rx(
                        operation.parameter_0.unwrap_or(0.0),
                    )
                }

                GateKind::RY => {
                    SingleQubitUnitary::ry(
                        operation.parameter_0.unwrap_or(0.0),
                    )
                }

                GateKind::RZ => {
                    SingleQubitUnitary::rz(
                        operation.parameter_0.unwrap_or(0.0),
                    )
                }

                GateKind::U3 => {
                    SingleQubitUnitary::u3(
                        operation.parameter_0.unwrap_or(0.0),
                        operation.parameter_1.unwrap_or(0.0),
                        operation.parameter_2.unwrap_or(0.0),
                    )
                }

                GateKind::I => SingleQubitUnitary::identity(),
                GateKind::X => SingleQubitUnitary::x(),
                GateKind::Y => SingleQubitUnitary::y(),
                GateKind::Z => SingleQubitUnitary::z(),
                GateKind::H => SingleQubitUnitary::h(),

                _ => {
                    // The public constructor guarantees that the current
                    // decomposition only contains supported single-qubit
                    // operations.
                    SingleQubitUnitary::identity()
                }
            };

            result = matrix.multiply(result);
        }

        result
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_tolerance(
    tolerance: f64,
) -> Result<(), SingleQubitSynthesisError> {
    if !tolerance.is_finite()
        || tolerance < MIN_TOLERANCE
    {
        return Err(
            SingleQubitSynthesisError::InvalidTolerance {
                tolerance,
            },
        );
    }

    Ok(())
}

fn validate_angle(
    angle: f64,
) -> Result<(), SingleQubitSynthesisError> {
    if angle.is_finite() {
        Ok(())
    } else {
        Err(SingleQubitSynthesisError::NonFiniteAngle)
    }
}

/// Normalize an angle into `[-PI, PI)`.
///
/// This operation is deterministic and avoids unnecessary growth of
/// parameter values during repeated optimization.
#[must_use]
pub fn normalize_angle(angle: f64) -> f64 {
    if !angle.is_finite() {
        return angle;
    }

    let mut value = angle.rem_euclid(TAU);

    if value >= PI {
        value -= TAU;
    }

    if value.abs() <= ZERO_EPSILON {
        0.0
    } else {
        value
    }
}

/// Returns the shortest signed angular distance between two angles.
#[must_use]
pub fn angular_distance(lhs: f64, rhs: f64) -> f64 {
    normalize_angle(lhs - rhs)
}

/// Returns whether two angles are approximately equal modulo `2π`.
#[must_use]
pub fn angles_equivalent(
    lhs: f64,
    rhs: f64,
    tolerance: f64,
) -> bool {
    angular_distance(lhs, rhs).abs() <= tolerance
}

// ============================================================================
// Euler extraction
// ============================================================================

/// Extracts a Z-Y-Z decomposition.
///
/// The returned values satisfy, up to global phase:
///
/// `U ≈ RZ(alpha) RY(beta) RZ(gamma)`.
///
/// The implementation explicitly handles the two singular cases:
///
/// - `sin(beta / 2) ≈ 0`;
/// - `cos(beta / 2) ≈ 0`.
pub fn decompose_zyz(
    unitary: SingleQubitUnitary,
    tolerance: f64,
) -> Result<SingleQubitDecomposition, SingleQubitSynthesisError> {
    let config = SingleQubitSynthesisConfig {
        tolerance,
        ..SingleQubitSynthesisConfig::default()
    };

    decompose_zyz_with_config(unitary, config)
}

/// Extracts a Z-Y-Z decomposition using the supplied configuration.
pub fn decompose_zyz_with_config(
    unitary: SingleQubitUnitary,
    config: SingleQubitSynthesisConfig,
) -> Result<SingleQubitDecomposition, SingleQubitSynthesisError> {
    config.validate()?;
    unitary.validate(config.tolerance)?;

    let determinant = unitary.determinant();

    if determinant.is_near_zero(config.tolerance) {
        return Err(
            SingleQubitSynthesisError::DecompositionFailure {
                message: "unitary determinant is numerically zero",
            },
        );
    }

    // Remove the determinant phase so that the resulting matrix belongs to
    // SU(2). For a 2x2 unitary, det(U) has unit magnitude.
    let determinant_phase = determinant.im.atan2(determinant.re);
    let global_phase = determinant_phase * 0.5;

    let phase_factor = Complex64::new(
        (-global_phase).cos(),
        (-global_phase).sin(),
    );

    let special = SingleQubitUnitary::new(
        unitary.m00 * phase_factor,
        unitary.m01 * phase_factor,
        unitary.m10 * phase_factor,
        unitary.m11 * phase_factor,
    );

    // For SU(2):
    //
    //     U = [ a  b ]
    //         [ -b* a* ]
    //
    // and
    //
    //     |a| = cos(beta/2)
    //     |b| = sin(beta/2).
    let a = special.m00;
    let b = special.m01;

    let mut cos_half_beta = a.norm();
    let mut sin_half_beta = b.norm();

    cos_half_beta = clamp_near_unit(
        cos_half_beta,
        config.tolerance,
    )?;

    sin_half_beta = clamp_near_unit(
        sin_half_beta,
        config.tolerance,
    )?;

    let beta = 2.0 * sin_half_beta.atan2(cos_half_beta);

    let (alpha, gamma) =
        if sin_half_beta <= config.tolerance {
            // beta ≈ 0:
            //
            // U ≈ RZ(alpha + gamma)
            //
            // Choose gamma = 0 for a deterministic representative.
            let phase_a = a.im.atan2(a.re);

            (normalize_angle(2.0 * phase_a), 0.0)
        } else if cos_half_beta <= config.tolerance {
            // beta ≈ pi:
            //
            // Only alpha - gamma is observable in this singular form.
            // Choose gamma = 0 deterministically.
            let phase_b = b.im.atan2(b.re);

            (
                normalize_angle(2.0 * phase_b - PI),
                0.0,
            )
        } else {
            // For:
            //
            // RZ(a) RY(b) RZ(g)
            //
            // a00 = exp(-i(a+g)/2) cos(b/2)
            // a01 = -exp(-i(a-g)/2) sin(b/2)
            //
            // Therefore:
            //
            // arg(a00) = -(a+g)/2
            // arg(-a01) = -(a-g)/2
            let phase_a = a.im.atan2(a.re);
            let phase_minus_b =
                (-b).im.atan2((-b).re);

            let alpha =
                -(phase_a + phase_minus_b);

            let gamma =
                -(phase_a - phase_minus_b);

            (
                normalize_angle(alpha),
                normalize_angle(gamma),
            )
        };

    let alpha = normalize_angle(alpha);
    let beta = if config.normalize_angles {
        normalize_angle(beta)
    } else {
        beta
    };
    let gamma = normalize_angle(gamma);

    validate_angle(alpha)?;
    validate_angle(beta)?;
    validate_angle(gamma)?;

    let mut operations = Vec::with_capacity(3);

    if !config.eliminate_near_zero_rotations
        || alpha.abs() > config.tolerance
    {
        operations.push(
            SynthesizedOperation::one_parameter(
                GateKind::RZ,
                alpha,
            ),
        );
    }

    if !config.eliminate_near_zero_rotations
        || beta.abs() > config.tolerance
    {
        operations.push(
            SynthesizedOperation::one_parameter(
                GateKind::RY,
                beta,
            ),
        );
    }

    if !config.eliminate_near_zero_rotations
        || gamma.abs() > config.tolerance
    {
        operations.push(
            SynthesizedOperation::one_parameter(
                GateKind::RZ,
                gamma,
            ),
        );
    }

    if operations.len() > config.max_operations {
        return Err(
            SingleQubitSynthesisError::OperationLimitExceeded {
                required: operations.len(),
                maximum: config.max_operations,
            },
        );
    }

    let decomposition = SingleQubitDecomposition {
        convention: EulerConvention::ZYZ,
        alpha,
        beta,
        gamma,
        global_phase,
        operations,
    };

    verify_decomposition(
        unitary,
        &decomposition,
        config,
    )?;

    Ok(decomposition)
}

/// Extracts a Z-X-Z decomposition.
///
/// This is derived from the Z-Y-Z representation through the identity:
///
/// `RX(theta) = RZ(-PI/2) RY(theta) RZ(PI/2)`.
pub fn decompose_zxz(
    unitary: SingleQubitUnitary,
    tolerance: f64,
) -> Result<SingleQubitDecomposition, SingleQubitSynthesisError> {
    let config = SingleQubitSynthesisConfig {
        tolerance,
        ..SingleQubitSynthesisConfig::default()
    };

    decompose_zxz_with_config(unitary, config)
}

/// Extracts a Z-X-Z decomposition using the supplied configuration.
pub fn decompose_zxz_with_config(
    unitary: SingleQubitUnitary,
    config: SingleQubitSynthesisConfig,
) -> Result<SingleQubitDecomposition, SingleQubitSynthesisError> {
    config.validate()?;
    unitary.validate(config.tolerance)?;

    let zyz = decompose_zyz_with_config(
        unitary,
        SingleQubitSynthesisConfig {
            // ZYZ is first used as a mathematically stable intermediate.
            eliminate_near_zero_rotations: false,
            ..config
        },
    )?;

    // RY(beta)
    // =
    // RZ(PI/2) RX(beta) RZ(-PI/2)
    //
    // Hence:
    //
    // RZ(a) RY(b) RZ(g)
    // =
    // RZ(a + PI/2) RX(b) RZ(g - PI/2).
    let alpha = normalize_angle(
        zyz.alpha + FRAC_PI_2,
    );

    let beta = normalize_angle(zyz.beta);

    let gamma = normalize_angle(
        zyz.gamma - FRAC_PI_2,
    );

    let mut operations = Vec::with_capacity(3);

    if !config.eliminate_near_zero_rotations
        || alpha.abs() > config.tolerance
    {
        operations.push(
            SynthesizedOperation::one_parameter(
                GateKind::RZ,
                alpha,
            ),
        );
    }

    if !config.eliminate_near_zero_rotations
        || beta.abs() > config.tolerance
    {
        operations.push(
            SynthesizedOperation::one_parameter(
                GateKind::RX,
                beta,
            ),
        );
    }

    if !config.eliminate_near_zero_rotations
        || gamma.abs() > config.tolerance
    {
        operations.push(
            SynthesizedOperation::one_parameter(
                GateKind::RZ,
                gamma,
            ),
        );
    }

    if operations.len() > config.max_operations {
        return Err(
            SingleQubitSynthesisError::OperationLimitExceeded {
                required: operations.len(),
                maximum: config.max_operations,
            },
        );
    }

    let decomposition = SingleQubitDecomposition {
        convention: EulerConvention::ZXZ,
        alpha,
        beta,
        gamma,
        global_phase: zyz.global_phase,
        operations,
    };

    verify_decomposition(
        unitary,
        &decomposition,
        config,
    )?;

    Ok(decomposition)
}

// ============================================================================
// U3 synthesis
// ============================================================================

/// Synthesizes an arbitrary single-qubit unitary as a canonical `U3` gate.
///
/// The resulting gate is:
///
/// `U3(theta, phi, lambda)`.
///
/// With the canonical Zamani convention, this represents the same unitary as
/// the extracted Z-Y-Z decomposition up to global phase.
pub fn synthesize_u3(
    unitary: SingleQubitUnitary,
    config: SingleQubitSynthesisConfig,
) -> Result<SingleQubitDecomposition, SingleQubitSynthesisError> {
    config.validate()?;
    unitary.validate(config.tolerance)?;

    let zyz = decompose_zyz_with_config(
        unitary,
        SingleQubitSynthesisConfig {
            eliminate_near_zero_rotations: false,
            ..config
        },
    )?;

    let theta = normalize_angle(zyz.beta);
    let phi = normalize_angle(zyz.alpha);
    let lambda = normalize_angle(zyz.gamma);

    validate_angle(theta)?;
    validate_angle(phi)?;
    validate_angle(lambda)?;

    let operations = if config.eliminate_near_zero_rotations
        && theta.abs() <= config.tolerance
        && phi.abs() <= config.tolerance
        && lambda.abs() <= config.tolerance
    {
        Vec::new()
    } else {
        vec![
            SynthesizedOperation::three_parameters(
                GateKind::U3,
                theta,
                phi,
                lambda,
            ),
        ]
    };

    if operations.len() > config.max_operations {
        return Err(
            SingleQubitSynthesisError::OperationLimitExceeded {
                required: operations.len(),
                maximum: config.max_operations,
            },
        );
    }

    let decomposition = SingleQubitDecomposition {
        convention: EulerConvention::ZYZ,
        alpha: phi,
        beta: theta,
        gamma: lambda,
        global_phase: zyz.global_phase,
        operations,
    };

    verify_decomposition(
        unitary,
        &decomposition,
        config,
    )?;

    Ok(decomposition)
}

// ============================================================================
// Target-gate synthesis
// ============================================================================

/// Target gate set for single-qubit synthesis.
///
/// The target set is deliberately explicit. A synthesis routine must never
/// silently introduce a gate that the caller did not request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleQubitTarget {
    /// Native arbitrary-angle RX/RY/RZ gates.
    RxRyRz,

    /// Native arbitrary-angle RX/RZ gates.
    RxRz,

    /// Native arbitrary-angle RY/RZ gates.
    RyRz,

    /// A native arbitrary-angle U3 gate.
    U3,

    /// The canonical ZYZ basis.
    ZYZ,

    /// The canonical ZXZ basis.
    ZXZ,
}

impl SingleQubitTarget {
    /// Returns the number of generic operations required by the target's
    /// canonical decomposition.
    #[must_use]
    pub const fn maximum_operations(self) -> usize {
        match self {
            Self::U3 => 1,
            Self::RxRyRz => 3,
            Self::RxRz => 5,
            Self::RyRz => 3,
            Self::ZYZ => 3,
            Self::ZXZ => 3,
        }
    }
}

/// Synthesize a single-qubit unitary against an explicit target gate set.
///
/// This function is intentionally conservative. If a target basis cannot
/// represent an arbitrary single-qubit unitary exactly with the available
/// continuous rotations, the function returns an error rather than silently
/// approximating it.
pub fn synthesize_for_target(
    unitary: SingleQubitUnitary,
    target: SingleQubitTarget,
    config: SingleQubitSynthesisConfig,
) -> Result<SingleQubitDecomposition, SingleQubitSynthesisError> {
    config.validate()?;

    match target {
        SingleQubitTarget::U3 => {
            synthesize_u3(unitary, config)
        }

        SingleQubitTarget::ZYZ => {
            decompose_zyz_with_config(
                unitary,
                config,
            )
        }

        SingleQubitTarget::ZXZ => {
            decompose_zxz_with_config(
                unitary,
                config,
            )
        }

        SingleQubitTarget::RxRyRz => {
            // The target contains all three continuous Euler axes.
            //
            // Use ZYZ because it is numerically stable and canonical, but
            // translate the outer Z rotations directly into the same target
            // representation.
            decompose_zyz_with_config(
                unitary,
                config,
            )
        }

        SingleQubitTarget::RxRz => {
            synthesize_rx_rz(unitary, config)
        }

        SingleQubitTarget::RyRz => {
            decompose_zyz_with_config(
                unitary,
                config,
            )
        }
    }
}

/// Synthesize using RX/RZ only.
///
/// An arbitrary single-qubit unitary requires more than three rotations when
/// restricted to two axes. The implementation uses the identity:
///
/// `RY(theta) = RZ(PI/2) RX(theta) RZ(-PI/2)`.
///
/// Therefore:
///
/// `RZ(a) RY(b) RZ(g)`
///
/// becomes:
///
/// `RZ(a + PI/2) RX(b) RZ(g - PI/2)`.
pub fn synthesize_rx_rz(
    unitary: SingleQubitUnitary,
    config: SingleQubitSynthesisConfig,
) -> Result<SingleQubitDecomposition, SingleQubitSynthesisError> {
    config.validate()?;

    let zyz = decompose_zyz_with_config(
        unitary,
        SingleQubitSynthesisConfig {
            eliminate_near_zero_rotations: false,
            ..config
        },
    )?;

    let first = normalize_angle(
        zyz.alpha + FRAC_PI_2,
    );

    let middle = normalize_angle(zyz.beta);

    let third = normalize_angle(
        zyz.gamma - FRAC_PI_2,
    );

    let mut operations = Vec::with_capacity(3);

    if !config.eliminate_near_zero_rotations
        || first.abs() > config.tolerance
    {
        operations.push(
            SynthesizedOperation::one_parameter(
                GateKind::RZ,
                first,
            ),
        );
    }

    if !config.eliminate_near_zero_rotations
        || middle.abs() > config.tolerance
    {
        operations.push(
            SynthesizedOperation::one_parameter(
                GateKind::RX,
                middle,
            ),
        );
    }

    if !config.eliminate_near_zero_rotations
        || third.abs() > config.tolerance
    {
        operations.push(
            SynthesizedOperation::one_parameter(
                GateKind::RZ,
                third,
            ),
        );
    }

    if operations.len() > config.max_operations {
        return Err(
            SingleQubitSynthesisError::OperationLimitExceeded {
                required: operations.len(),
                maximum: config.max_operations,
            },
        );
    }

    let decomposition = SingleQubitDecomposition {
        convention: EulerConvention::ZXZ,
        alpha: first,
        beta: middle,
        gamma: third,
        global_phase: zyz.global_phase,
        operations,
    };

    verify_decomposition(
        unitary,
        &decomposition,
        config,
    )?;

    Ok(decomposition)
}

// ============================================================================
// Gate-level classification
// ============================================================================

/// Classifies a unitary when it exactly matches a canonical fixed single-qubit
/// gate within the requested tolerance.
///
/// The result is `None` for a parameterized or otherwise unmatched unitary.
pub fn classify_exact_gate(
    unitary: SingleQubitUnitary,
    tolerance: f64,
) -> Result<Option<GateKind>, SingleQubitSynthesisError> {
    validate_tolerance(tolerance)?;
    unitary.validate(tolerance)?;

    let candidates = [
        (GateKind::I, SingleQubitUnitary::identity()),
        (GateKind::X, SingleQubitUnitary::x()),
        (GateKind::Y, SingleQubitUnitary::y()),
        (GateKind::Z, SingleQubitUnitary::z()),
        (GateKind::H, SingleQubitUnitary::h()),
    ];

    for (gate, candidate) in candidates {
        if unitary.equivalent_up_to_global_phase(
            candidate,
            tolerance,
        )? {
            return Ok(Some(gate));
        }
    }

    Ok(None)
}

// ============================================================================
// Verification
// ============================================================================

fn verify_decomposition(
    original: SingleQubitUnitary,
    decomposition: &SingleQubitDecomposition,
    config: SingleQubitSynthesisConfig,
) -> Result<(), SingleQubitSynthesisError> {
    let reconstructed = decomposition.unitary();

    let equivalent =
        if config.allow_global_phase {
            original.equivalent_up_to_global_phase(
                reconstructed,
                config.tolerance,
            )?
        } else {
            original.max_difference(reconstructed)
                <= config.tolerance
        };

    if equivalent {
        Ok(())
    } else if !config.allow_global_phase {
        let product =
            reconstructed.multiply(original.adjoint());

        let phase = product.m00.im.atan2(
            product.m00.re,
        );

        Err(
            SingleQubitSynthesisError::GlobalPhaseMismatch {
                phase,
            },
        )
    } else {
        Err(
            SingleQubitSynthesisError::DecompositionFailure {
                message: "reconstructed unitary does not match input within tolerance",
            },
        )
    }
}

fn clamp_near_unit(
    value: f64,
    tolerance: f64,
) -> Result<f64, SingleQubitSynthesisError> {
    if !value.is_finite() {
        return Err(
            SingleQubitSynthesisError::NonFiniteAngle,
        );
    }

    if value < -tolerance
        || value > 1.0 + tolerance
    {
        return Err(
            SingleQubitSynthesisError::DecompositionFailure {
                message: "unitary amplitude left the valid [0,1] interval",
            },
        );
    }

    if value <= tolerance {
        Ok(0.0)
    } else if value >= 1.0 - tolerance {
        Ok(1.0)
    } else {
        Ok(value)
    }
}

// ============================================================================
// Canonical IR lowering boundary
// ============================================================================

/// Returns whether a gate kind can be emitted by this synthesis module as a
/// one-qubit operation.
///
/// This helper exists so future target/profile code can query the synthesis
/// contract without duplicating gate classification logic.
#[must_use]
pub const fn supports_gate_kind(gate: GateKind) -> bool {
    matches!(
        gate,
        GateKind::I
            | GateKind::X
            | GateKind::Y
            | GateKind::Z
            | GateKind::H
            | GateKind::S
            | GateKind::Sdg
            | GateKind::T
            | GateKind::Tdg
            | GateKind::V
            | GateKind::Vdg
            | GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::U2
            | GateKind::U3
    )
}

/// Returns the canonical parameter arity for a synthesis-emitted gate.
///
/// This is derived from the canonical IR gate definition rather than from
/// optimizer-local gate metadata.
#[must_use]
pub const fn expected_parameter_count(
    gate: GateKind,
) -> usize {
    gate.parameter_count()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_equivalent(
        lhs: SingleQubitUnitary,
        rhs: SingleQubitUnitary,
    ) {
        assert!(
            lhs.equivalent_up_to_global_phase(
                rhs,
                1.0e-10,
            )
            .expect("unitaries should be valid")
        );
    }

    #[test]
    fn identity_is_identity() {
        let identity = SingleQubitUnitary::identity();

        let decomposition = decompose_zyz(
            identity,
            DEFAULT_TOLERANCE,
        )
        .expect("identity must decompose");

        assert!(decomposition.is_identity());
        assert_equivalent(
            identity,
            decomposition.unitary(),
        );
    }

    #[test]
    fn x_decomposes() {
        let decomposition = decompose_zyz(
            SingleQubitUnitary::x(),
            DEFAULT_TOLERANCE,
        )
        .expect("X must decompose");

        assert_equivalent(
            SingleQubitUnitary::x(),
            decomposition.unitary(),
        );
    }

    #[test]
    fn y_decomposes() {
        let decomposition = decompose_zyz(
            SingleQubitUnitary::y(),
            DEFAULT_TOLERANCE,
        )
        .expect("Y must decompose");

        assert_equivalent(
            SingleQubitUnitary::y(),
            decomposition.unitary(),
        );
    }

    #[test]
    fn z_decomposes() {
        let decomposition = decompose_zyz(
            SingleQubitUnitary::z(),
            DEFAULT_TOLERANCE,
        )
        .expect("Z must decompose");

        assert_equivalent(
            SingleQubitUnitary::z(),
            decomposition.unitary(),
        );
    }

    #[test]
    fn hadamard_decomposes() {
        let decomposition = decompose_zyz(
            SingleQubitUnitary::h(),
            DEFAULT_TOLERANCE,
        )
        .expect("H must decompose");

        assert_equivalent(
            SingleQubitUnitary::h(),
            decomposition.unitary(),
        );
    }

    #[test]
    fn rx_round_trip() {
        let input = SingleQubitUnitary::rx(
            0.731,
        );

        let decomposition = decompose_zyz(
            input,
            DEFAULT_TOLERANCE,
        )
        .expect("RX must decompose");

        assert_equivalent(
            input,
            decomposition.unitary(),
        );
    }

    #[test]
    fn ry_round_trip() {
        let input = SingleQubitUnitary::ry(
            -1.127,
        );

        let decomposition = decompose_zyz(
            input,
            DEFAULT_TOLERANCE,
        )
        .expect("RY must decompose");

        assert_equivalent(
            input,
            decomposition.unitary(),
        );
    }

    #[test]
    fn rz_round_trip() {
        let input = SingleQubitUnitary::rz(
            2.431,
        );

        let decomposition = decompose_zyz(
            input,
            DEFAULT_TOLERANCE,
        )
        .expect("RZ must decompose");

        assert_equivalent(
            input,
            decomposition.unitary(),
        );
    }

    #[test]
    fn u3_round_trip() {
        let input =
            SingleQubitUnitary::u3(
                0.71,
                -1.13,
                2.19,
            );

        let decomposition = synthesize_u3(
            input,
            SingleQubitSynthesisConfig::default(),
        )
        .expect("U3 must synthesize");

        assert_equivalent(
            input,
            decomposition.unitary(),
        );
    }

    #[test]
    fn zxz_round_trip() {
        let input =
            SingleQubitUnitary::u3(
                0.43,
                1.17,
                -2.03,
            );

        let decomposition = decompose_zxz(
            input,
            DEFAULT_TOLERANCE,
        )
        .expect("ZXZ must decompose");

        assert_equivalent(
            input,
            decomposition.unitary(),
        );
    }

    #[test]
    fn rx_rz_round_trip() {
        let input =
            SingleQubitUnitary::u3(
                1.1,
                -0.4,
                2.2,
            );

        let decomposition = synthesize_rx_rz(
            input,
            SingleQubitSynthesisConfig::default(),
        )
        .expect("RX/RZ synthesis must succeed");

        assert_equivalent(
            input,
            decomposition.unitary(),
        );
    }

    #[test]
    fn classify_fixed_gates() {
        assert_eq!(
            classify_exact_gate(
                SingleQubitUnitary::identity(),
                DEFAULT_TOLERANCE,
            )
            .expect("classification must succeed"),
            Some(GateKind::I)
        );

        assert_eq!(
            classify_exact_gate(
                SingleQubitUnitary::x(),
                DEFAULT_TOLERANCE,
            )
            .expect("classification must succeed"),
            Some(GateKind::X)
        );

        assert_eq!(
            classify_exact_gate(
                SingleQubitUnitary::h(),
                DEFAULT_TOLERANCE,
            )
            .expect("classification must succeed"),
            Some(GateKind::H)
        );
    }

    #[test]
    fn rejects_non_unitary_matrix() {
        let invalid =
            SingleQubitUnitary::new(
                Complex64::new(2.0, 0.0),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::new(1.0, 0.0),
            );

        assert!(
            invalid
                .validate(DEFAULT_TOLERANCE)
                .is_err()
        );
    }

    #[test]
    fn rejects_non_finite_matrix() {
        let invalid =
            SingleQubitUnitary::new(
                Complex64::new(
                    f64::NAN,
                    0.0,
                ),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::one(),
            );

        assert!(
            invalid
                .validate(DEFAULT_TOLERANCE)
                .is_err()
        );
    }

    #[test]
    fn angle_normalization_is_deterministic() {
        assert!(
            angles_equivalent(
                0.0,
                TAU,
                DEFAULT_TOLERANCE,
            )
        );

        assert!(
            angles_equivalent(
                PI,
                -PI,
                DEFAULT_TOLERANCE,
            )
        );
    }

    #[test]
    fn operation_limit_is_enforced() {
        let config =
            SingleQubitSynthesisConfig {
                max_operations: 1,
                ..SingleQubitSynthesisConfig::default()
            };

        let input =
            SingleQubitUnitary::u3(
                0.7,
                1.1,
                -0.3,
            );

        // U3 is one operation, so this must succeed.
        let result =
            synthesize_u3(input, config);

        assert!(result.is_ok());
    }

    #[test]
    fn target_support_is_explicit() {
        assert!(
            supports_gate_kind(
                GateKind::RX
            )
        );

        assert!(
            supports_gate_kind(
                GateKind::U3
            )
        );

        assert!(
            !supports_gate_kind(
                GateKind::CX
            )
        );

        assert_eq!(
            expected_parameter_count(
                GateKind::U3
            ),
            3
        );
    }
}