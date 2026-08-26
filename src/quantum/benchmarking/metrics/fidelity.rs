//! Zamani Quantum Benchmarking — Fidelity Metrics
//!
//! Production-grade, backend-independent fidelity calculations used by the
//! Zamani quantum benchmarking subsystem.
//!
//! # Scope
//!
//! This module contains fidelity mathematics only. It does NOT:
//!
//! - execute quantum circuits;
//! - generate benchmark circuits;
//! - communicate with hardware;
//! - perform tomography experiments;
//! - select a backend;
//! - own Quantum IR;
//! - own benchmark protocols;
//! - print diagnostics;
//! - mutate global state.
//!
//! The intended dependency direction is:
//!
//! ```text
//! Zamani Quantum IR
//!        │
//!        ▼
//! benchmark observation / protocol result
//!        │
//!        ▼
//! benchmarking::metrics::fidelity
//!        │
//!        ▼
//! BenchmarkResult / protocol analysis
//! ```
//!
//! # Fidelity conventions
//!
//! Zamani uses the squared Uhlmann-Jozsa fidelity convention:
//!
//! ```text
//! F(rho, sigma)
//!   = [Tr sqrt(sqrt(rho) sigma sqrt(rho))]^2
//! ```
//!
//! For a pure reference state |psi> and an arbitrary state rho:
//!
//! ```text
//! F(|psi>, rho) = <psi|rho|psi>
//! ```
//!
//! For two pure states:
//!
//! ```text
//! F(|psi>, |phi>) = |<psi|phi>|^2
//! ```
//!
//! The result is always intended to lie in [0, 1].
//!
//! # Production guarantees
//!
//! This module:
//!
//! - rejects NaN and infinite values;
//! - validates dimensions;
//! - validates state-vector normalization;
//! - validates probability distributions;
//! - validates density-matrix dimensions;
//! - rejects negative density-matrix diagonal probabilities;
//! - rejects non-Hermitian density matrices beyond tolerance;
//! - rejects materially non-unit-trace density matrices;
//! - checks numerical bounds;
//! - avoids unchecked integer arithmetic;
//! - avoids external numerical dependencies;
//! - provides explicit tolerances;
//! - does not silently repair invalid scientific input;
//! - does not silently reinterpret a fidelity convention;
//! - exposes the metric definition used by the result.
//!
//! # Integration contract
//!
//! Future modules may use this file as follows:
//!
//! ```text
//! protocols/gate_fidelity.rs
//!     -> fidelity::state_fidelity
//!     -> fidelity::process_fidelity
//!     -> fidelity::average_gate_fidelity
//!
//! protocols/xeb.rs
//!     -> fidelity::classical_distribution_fidelity
//!
//! protocols/mirror.rs
//!     -> fidelity::state_fidelity
//!
//! applications/*.rs
//!     -> fidelity::state_fidelity
//!     -> fidelity::classical_distribution_fidelity
//!
//! reporting/*
//!     -> FidelityResult
//!
//! metrics/mod.rs
//!     -> re-export public fidelity API
//! ```
//!
//! This file intentionally does not depend on those future modules so it can
//! be completed and tested independently.
//!
//! # Scientific note
//!
//! Full mixed-state fidelity requires a matrix square root. This module uses
//! an eigendecomposition-based implementation for Hermitian positive
//! semidefinite density matrices. The implementation is deliberately
//! dependency-free and is intended for benchmarking-scale diagnostic
//! calculations. Large tomography problems should not be interpreted as
//! computationally cheap merely because the API is available.
//!
//! See also:
//! - Quantum Benchmark Zoo fidelity/error definitions.
//! - QASMBench fidelity methodology.
//!
//! Rust compatibility: Rust 1.97.1, Rust 2021.
//! No nightly features required.

use std::fmt;

// =============================================================================
// Public constants
// =============================================================================

/// Default numerical tolerance for scientific input validation.
pub const DEFAULT_FIDELITY_TOLERANCE: f64 = 1.0e-10;

/// Default normalization tolerance for state vectors.
pub const DEFAULT_STATE_NORMALIZATION_TOLERANCE: f64 = 1.0e-10;

/// Default trace tolerance for density matrices.
pub const DEFAULT_TRACE_TOLERANCE: f64 = 1.0e-10;

/// Default Hermiticity tolerance for density matrices.
pub const DEFAULT_HERMITICITY_TOLERANCE: f64 = 1.0e-10;

/// Default positive-semidefinite numerical tolerance.
pub const DEFAULT_POSITIVITY_TOLERANCE: f64 = 1.0e-10;

// =============================================================================
// Error type
// =============================================================================

/// Errors produced by fidelity calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum FidelityError {
    /// A numerical input contained NaN or infinity.
    NonFiniteValue {
        /// Human-readable location of the invalid value.
        context: &'static str,
    },

    /// A state vector was empty.
    EmptyStateVector,

    /// State-vector dimensions differ.
    StateDimensionMismatch {
        /// First state dimension.
        left: usize,

        /// Second state dimension.
        right: usize,
    },

    /// A state vector is not normalized within tolerance.
    StateNotNormalized {
        /// Observed squared norm.
        norm_squared: f64,

        /// Permitted absolute deviation.
        tolerance: f64,
    },

    /// A probability distribution is empty.
    EmptyDistribution,

    /// Probability distributions have different dimensions.
    DistributionDimensionMismatch {
        left: usize,
        right: usize,
    },

    /// A probability is outside [0, 1].
    InvalidProbability {
        index: usize,
        value: f64,
    },

    /// Probabilities do not sum to one.
    DistributionNotNormalized {
        sum: f64,
        tolerance: f64,
    },

    /// A matrix has no dimensions.
    EmptyMatrix,

    /// Matrix dimensions are invalid.
    InvalidMatrixDimension {
        rows: usize,
        columns: usize,
    },

    /// Two matrices have incompatible dimensions.
    MatrixDimensionMismatch {
        left_rows: usize,
        left_columns: usize,
        right_rows: usize,
        right_columns: usize,
    },

    /// Matrix is not square where a square matrix is required.
    MatrixNotSquare {
        rows: usize,
        columns: usize,
    },

    /// Density matrix is not Hermitian.
    NotHermitian {
        maximum_deviation: f64,
        tolerance: f64,
    },

    /// Density matrix trace is not one.
    InvalidTrace {
        trace: Complex64,
        tolerance: f64,
    },

    /// A diagonal density-matrix probability is materially negative.
    NegativeDiagonal {
        index: usize,
        value: f64,
        tolerance: f64,
    },

    /// A density matrix has a materially negative eigenvalue.
    NotPositiveSemidefinite {
        minimum_eigenvalue: f64,
        tolerance: f64,
    },

    /// A matrix square root could not be constructed.
    MatrixSquareRootFailed,

    /// A required eigendecomposition did not converge.
    EigenDecompositionFailed,

    /// A dimension cannot be converted safely to a Hilbert-space dimension.
    DimensionOverflow {
        dimension: usize,
    },

    /// A derived fidelity is outside the mathematically valid range.
    FidelityOutOfRange {
        value: f64,
        tolerance: f64,
    },

    /// Average gate fidelity received an invalid Hilbert-space dimension.
    InvalidHilbertDimension,

    /// Process fidelity and average gate fidelity were requested for
    /// incompatible dimensions.
    ProcessDimensionMismatch {
        process_dimension: usize,
        hilbert_dimension: usize,
    },
}

impl fmt::Display for FidelityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { context } => {
                write!(f, "fidelity input contains a non-finite value: {context}")
            }

            Self::EmptyStateVector => {
                write!(f, "state vector must not be empty")
            }

            Self::StateDimensionMismatch { left, right } => {
                write!(
                    f,
                    "state-vector dimensions differ: left={left}, right={right}"
                )
            }

            Self::StateNotNormalized {
                norm_squared,
                tolerance,
            } => {
                write!(
                    f,
                    "state vector is not normalized: norm_squared={norm_squared}, \
                     tolerance={tolerance}"
                )
            }

            Self::EmptyDistribution => {
                write!(f, "probability distribution must not be empty")
            }

            Self::DistributionDimensionMismatch { left, right } => {
                write!(
                    f,
                    "probability-distribution dimensions differ: left={left}, right={right}"
                )
            }

            Self::InvalidProbability { index, value } => {
                write!(
                    f,
                    "invalid probability at index {index}: {value}; expected [0, 1]"
                )
            }

            Self::DistributionNotNormalized { sum, tolerance } => {
                write!(
                    f,
                    "probability distribution is not normalized: sum={sum}, \
                     tolerance={tolerance}"
                )
            }

            Self::EmptyMatrix => {
                write!(f, "matrix must not be empty")
            }

            Self::InvalidMatrixDimension { rows, columns } => {
                write!(
                    f,
                    "invalid matrix dimensions: {rows}x{columns}"
                )
            }

            Self::MatrixDimensionMismatch {
                left_rows,
                left_columns,
                right_rows,
                right_columns,
            } => {
                write!(
                    f,
                    "matrix dimensions differ: left={left_rows}x{left_columns}, \
                     right={right_rows}x{right_columns}"
                )
            }

            Self::MatrixNotSquare { rows, columns } => {
                write!(f, "matrix must be square: {rows}x{columns}")
            }

            Self::NotHermitian {
                maximum_deviation,
                tolerance,
            } => {
                write!(
                    f,
                    "density matrix is not Hermitian: maximum deviation={maximum_deviation}, \
                     tolerance={tolerance}"
                )
            }

            Self::InvalidTrace { trace, tolerance } => {
                write!(
                    f,
                    "density matrix trace is not one: trace={trace}, tolerance={tolerance}"
                )
            }

            Self::NegativeDiagonal {
                index,
                value,
                tolerance,
            } => {
                write!(
                    f,
                    "density matrix has negative diagonal at {index}: \
                     value={value}, tolerance={tolerance}"
                )
            }

            Self::NotPositiveSemidefinite {
                minimum_eigenvalue,
                tolerance,
            } => {
                write!(
                    f,
                    "density matrix is not positive semidefinite: \
                     minimum eigenvalue={minimum_eigenvalue}, tolerance={tolerance}"
                )
            }

            Self::MatrixSquareRootFailed => {
                write!(f, "density-matrix square root failed")
            }

            Self::EigenDecompositionFailed => {
                write!(f, "Hermitian eigendecomposition failed to converge")
            }

            Self::DimensionOverflow { dimension } => {
                write!(
                    f,
                    "Hilbert-space dimension {dimension} cannot be represented safely"
                )
            }

            Self::FidelityOutOfRange { value, tolerance } => {
                write!(
                    f,
                    "calculated fidelity is outside [0, 1]: value={value}, tolerance={tolerance}"
                )
            }

            Self::InvalidHilbertDimension => {
                write!(f, "Hilbert-space dimension must be greater than zero")
            }

            Self::ProcessDimensionMismatch {
                process_dimension,
                hilbert_dimension,
            } => {
                write!(
                    f,
                    "process dimension {process_dimension} does not match \
                     Hilbert dimension {hilbert_dimension}"
                )
            }
        }
    }
}

impl std::error::Error for FidelityError {}

// =============================================================================
// Complex number
// =============================================================================

/// Dependency-free complex number representation.
///
/// This is intentionally small and immutable. If Zamani later adopts a
/// canonical complex-number type in the quantum IR/runtime, conversion can be
/// added at the integration boundary without changing the fidelity formulas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex64 {
    /// Real component.
    pub re: f64,

    /// Imaginary component.
    pub im: f64,
}

impl Complex64 {
    /// Creates a complex number.
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Zero.
    #[inline]
    pub const fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    /// One.
    #[inline]
    pub const fn one() -> Self {
        Self { re: 1.0, im: 0.0 }
    }

    /// Complex conjugate.
    #[inline]
    pub const fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Squared magnitude.
    #[inline]
    pub fn norm_squared(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Magnitude.
    #[inline]
    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    /// Returns whether both components are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
}

impl std::ops::Add for Complex64 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl std::ops::Sub for Complex64 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl std::ops::Mul for Complex64 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl std::ops::Mul<f64> for Complex64 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.re * rhs, self.im * rhs)
    }
}

impl std::ops::Div<f64> for Complex64 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.re / rhs, self.im / rhs)
    }
}

impl std::ops::Neg for Complex64 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.im)
    }
}

// =============================================================================
// Fidelity metric identity
// =============================================================================

/// Canonical fidelity convention used by Zamani.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FidelityDefinition {
    /// Squared Uhlmann-Jozsa fidelity.
    ///
    /// F = [Tr sqrt(sqrt(rho) sigma sqrt(rho))]^2
    UhlmannSquared,

    /// Fidelity between a pure state and an arbitrary density matrix.
    ///
    /// F = <psi|rho|psi>
    PureStateDensity,

    /// Squared overlap between two normalized pure states.
    ///
    /// F = |<psi|phi>|^2
    PureStateOverlap,

    /// Squared Bhattacharyya coefficient for classical probability
    /// distributions.
    ///
    /// F = (sum_i sqrt(p_i q_i))^2
    ClassicalSquaredBhattacharyya,

    /// Process/Choi-state fidelity.
    Process,

    /// Average gate fidelity derived from process fidelity.
    AverageGate,
}

impl FidelityDefinition {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UhlmannSquared => "uhlmann_squared",
            Self::PureStateDensity => "pure_state_density",
            Self::PureStateOverlap => "pure_state_overlap",
            Self::ClassicalSquaredBhattacharyya => "classical_squared_bhattacharyya",
            Self::Process => "process_fidelity",
            Self::AverageGate => "average_gate_fidelity",
        }
    }
}

// =============================================================================
// Fidelity result
// =============================================================================

/// A validated fidelity measurement.
///
/// This is deliberately richer than returning a bare `f64`, so future
/// `metrics::metric` integration can preserve the scientific meaning of a
/// result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FidelityResult {
    /// Fidelity value in [0, 1].
    pub value: f64,

    /// Metric definition.
    pub definition: FidelityDefinition,

    /// Numerical tolerance used during validation.
    pub tolerance: f64,
}

impl FidelityResult {
    /// Constructs a validated fidelity result.
    pub fn new(
        value: f64,
        definition: FidelityDefinition,
        tolerance: f64,
    ) -> Result<Self, FidelityError> {
        validate_tolerance(tolerance)?;

        if !value.is_finite() {
            return Err(FidelityError::NonFiniteValue {
                context: "fidelity result",
            });
        }

        if value < -tolerance || value > 1.0 + tolerance {
            return Err(FidelityError::FidelityOutOfRange { value, tolerance });
        }

        Ok(Self {
            value: clamp_unit_interval(value),
            definition,
            tolerance,
        })
    }

    /// Returns infidelity `1 - F`.
    pub fn infidelity(self) -> f64 {
        1.0 - self.value
    }

    /// Returns whether the fidelity is numerically perfect.
    pub fn is_perfect(self) -> bool {
        self.value >= 1.0 - self.tolerance
    }

    /// Returns whether the fidelity is numerically zero.
    pub fn is_zero(self) -> bool {
        self.value <= self.tolerance
    }
}

// =============================================================================
// Validation configuration
// =============================================================================

/// Validation tolerances used by density-matrix calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FidelityTolerance {
    /// State-vector normalization tolerance.
    pub normalization: f64,

    /// Density-matrix trace tolerance.
    pub trace: f64,

    /// Density-matrix Hermiticity tolerance.
    pub hermiticity: f64,

    /// Positive-semidefinite tolerance.
    pub positivity: f64,

    /// Final metric tolerance.
    pub result: f64,
}

impl Default for FidelityTolerance {
    fn default() -> Self {
        Self {
            normalization: DEFAULT_STATE_NORMALIZATION_TOLERANCE,
            trace: DEFAULT_TRACE_TOLERANCE,
            hermiticity: DEFAULT_HERMITICITY_TOLERANCE,
            positivity: DEFAULT_POSITIVITY_TOLERANCE,
            result: DEFAULT_FIDELITY_TOLERANCE,
        }
    }
}

impl FidelityTolerance {
    /// Validates all configured tolerances.
    pub fn validate(self) -> Result<Self, FidelityError> {
        validate_tolerance(self.normalization)?;
        validate_tolerance(self.trace)?;
        validate_tolerance(self.hermiticity)?;
        validate_tolerance(self.positivity)?;
        validate_tolerance(self.result)?;

        Ok(self)
    }
}

// =============================================================================
// State-vector validation
// =============================================================================

/// Validates a normalized quantum state vector.
pub fn validate_state_vector(
    state: &[Complex64],
    tolerance: f64,
) -> Result<(), FidelityError> {
    validate_tolerance(tolerance)?;

    if state.is_empty() {
        return Err(FidelityError::EmptyStateVector);
    }

    let mut norm_squared = 0.0;

    for amplitude in state {
        if !amplitude.is_finite() {
            return Err(FidelityError::NonFiniteValue {
                context: "state-vector amplitude",
            });
        }

        norm_squared += amplitude.norm_squared();
    }

    if !norm_squared.is_finite() {
        return Err(FidelityError::NonFiniteValue {
            context: "state-vector norm",
        });
    }

    if (norm_squared - 1.0).abs() > tolerance {
        return Err(FidelityError::StateNotNormalized {
            norm_squared,
            tolerance,
        });
    }

    Ok(())
}

// =============================================================================
// Pure-state fidelity
// =============================================================================

/// Calculates fidelity between two normalized pure states.
///
/// ```text
/// F(|psi>, |phi>) = |<psi|phi>|²
/// ```
///
/// This is the preferred API when both backend and reference data are
/// available as state vectors.
pub fn pure_state_fidelity(
    left: &[Complex64],
    right: &[Complex64],
) -> Result<FidelityResult, FidelityError> {
    pure_state_fidelity_with_tolerance(
        left,
        right,
        DEFAULT_FIDELITY_TOLERANCE,
    )
}

/// Calculates pure-state fidelity using an explicit validation tolerance.
pub fn pure_state_fidelity_with_tolerance(
    left: &[Complex64],
    right: &[Complex64],
    tolerance: f64,
) -> Result<FidelityResult, FidelityError> {
    validate_tolerance(tolerance)?;

    validate_state_vector(left, tolerance)?;
    validate_state_vector(right, tolerance)?;

    if left.len() != right.len() {
        return Err(FidelityError::StateDimensionMismatch {
            left: left.len(),
            right: right.len(),
        });
    }

    let mut overlap = Complex64::zero();

    for index in 0..left.len() {
        overlap = overlap + left[index].conjugate() * right[index];
    }

    let fidelity = overlap.norm_squared();

    FidelityResult::new(
        fidelity,
        FidelityDefinition::PureStateOverlap,
        tolerance,
    )
}

// =============================================================================
// Pure-state / density-matrix fidelity
// =============================================================================

/// Calculates fidelity between a pure reference state and a density matrix.
///
/// ```text
/// F(|psi>, rho) = <psi|rho|psi>
/// ```
///
/// This is substantially cheaper than general mixed-state Uhlmann fidelity
/// and should be preferred whenever one side of the comparison is known to
/// be pure.
pub fn pure_state_density_fidelity(
    state: &[Complex64],
    density_matrix: &ComplexMatrix,
) -> Result<FidelityResult, FidelityError> {
    pure_state_density_fidelity_with_tolerance(
        state,
        density_matrix,
        FidelityTolerance::default(),
    )
}

/// Calculates pure-state/density-matrix fidelity with explicit tolerances.
pub fn pure_state_density_fidelity_with_tolerance(
    state: &[Complex64],
    density_matrix: &ComplexMatrix,
    tolerances: FidelityTolerance,
) -> Result<FidelityResult, FidelityError> {
    let tolerances = tolerances.validate()?;

    validate_state_vector(state, tolerances.normalization)?;
    validate_density_matrix(density_matrix, tolerances)?;

    if state.len() != density_matrix.rows() {
        return Err(FidelityError::StateDimensionMismatch {
            left: state.len(),
            right: density_matrix.rows(),
        });
    }

    let mut result = Complex64::zero();

    for row in 0..state.len() {
        for column in 0..state.len() {
            let contribution = state[row].conjugate()
                * density_matrix[(row, column)]
                * state[column];

            result = result + contribution;
        }
    }

    if result.im.abs() > tolerances.result {
        return Err(FidelityError::NonFiniteValue {
            context: "pure-state/density-matrix fidelity imaginary residual",
        });
    }

    FidelityResult::new(
        result.re,
        FidelityDefinition::PureStateDensity,
        tolerances.result,
    )
}

// =============================================================================
// Classical distribution fidelity
// =============================================================================

/// Calculates squared Bhattacharyya fidelity between two classical
/// probability distributions.
///
/// ```text
/// F(p, q) = (sum_i sqrt(p_i q_i))²
/// ```
///
/// This is useful when a quantum benchmark exposes only measurement
/// distributions rather than reconstructed quantum states.
pub fn classical_distribution_fidelity(
    left: &[f64],
    right: &[f64],
) -> Result<FidelityResult, FidelityError> {
    classical_distribution_fidelity_with_tolerance(
        left,
        right,
        DEFAULT_FIDELITY_TOLERANCE,
    )
}

/// Calculates classical distribution fidelity with explicit tolerance.
pub fn classical_distribution_fidelity_with_tolerance(
    left: &[f64],
    right: &[f64],
    tolerance: f64,
) -> Result<FidelityResult, FidelityError> {
    validate_tolerance(tolerance)?;
    validate_probability_distribution(left, tolerance)?;
    validate_probability_distribution(right, tolerance)?;

    if left.len() != right.len() {
        return Err(FidelityError::DistributionDimensionMismatch {
            left: left.len(),
            right: right.len(),
        });
    }

    let mut coefficient = 0.0;

    for index in 0..left.len() {
        coefficient += (left[index] * right[index]).sqrt();
    }

    let fidelity = coefficient * coefficient;

    FidelityResult::new(
        fidelity,
        FidelityDefinition::ClassicalSquaredBhattacharyya,
        tolerance,
    )
}

/// Validates a classical probability distribution.
pub fn validate_probability_distribution(
    probabilities: &[f64],
    tolerance: f64,
) -> Result<(), FidelityError> {
    validate_tolerance(tolerance)?;

    if probabilities.is_empty() {
        return Err(FidelityError::EmptyDistribution);
    }

    let mut sum = 0.0;

    for (index, probability) in probabilities.iter().enumerate() {
        if !probability.is_finite() {
            return Err(FidelityError::NonFiniteValue {
                context: "probability distribution",
            });
        }

        if *probability < -tolerance || *probability > 1.0 + tolerance {
            return Err(FidelityError::InvalidProbability {
                index,
                value: *probability,
            });
        }

        sum += *probability;
    }

    if !sum.is_finite() {
        return Err(FidelityError::NonFiniteValue {
            context: "probability distribution sum",
        });
    }

    if (sum - 1.0).abs() > tolerance {
        return Err(FidelityError::DistributionNotNormalized {
            sum,
            tolerance,
        });
    }

    Ok(())
}

// =============================================================================
// Complex matrix
// =============================================================================

/// Dependency-free dense complex matrix.
///
/// The matrix is row-major.
///
/// This type is intentionally local to the fidelity metric boundary. It can
/// later be replaced by or converted from Zamani's canonical quantum state
/// representation without changing the public fidelity formulas.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexMatrix {
    rows: usize,
    columns: usize,
    data: Vec<Complex64>,
}

impl ComplexMatrix {
    /// Creates a matrix from row-major data.
    pub fn new(
        rows: usize,
        columns: usize,
        data: Vec<Complex64>,
    ) -> Result<Self, FidelityError> {
        if rows == 0 || columns == 0 {
            return Err(FidelityError::InvalidMatrixDimension { rows, columns });
        }

        let expected = rows
            .checked_mul(columns)
            .ok_or(FidelityError::DimensionOverflow {
                dimension: rows,
            })?;

        if data.len() != expected {
            return Err(FidelityError::InvalidMatrixDimension { rows, columns });
        }

        for value in &data {
            if !value.is_finite() {
                return Err(FidelityError::NonFiniteValue {
                    context: "matrix element",
                });
            }
        }

        Ok(Self {
            rows,
            columns,
            data,
        })
    }

    /// Creates an identity matrix.
    pub fn identity(size: usize) -> Result<Self, FidelityError> {
        if size == 0 {
            return Err(FidelityError::InvalidMatrixDimension {
                rows: 0,
                columns: 0,
            });
        }

        let count = size
            .checked_mul(size)
            .ok_or(FidelityError::DimensionOverflow { dimension: size })?;

        let mut data = vec![Complex64::zero(); count];

        for index in 0..size {
            data[index * size + index] = Complex64::one();
        }

        Ok(Self {
            rows: size,
            columns: size,
            data,
        })
    }

    /// Returns the number of rows.
    #[inline]
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    #[inline]
    pub const fn columns(&self) -> usize {
        self.columns
    }

    /// Returns the matrix dimensions.
    #[inline]
    pub const fn shape(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    /// Returns the underlying row-major elements.
    #[inline]
    pub fn data(&self) -> &[Complex64] {
        &self.data
    }

    /// Returns a mutable view of the underlying row-major elements.
    ///
    /// This is intentionally public so integration adapters can construct
    /// validated matrices without requiring another matrix abstraction.
    #[inline]
    pub fn data_mut(&mut self) -> &mut [Complex64] {
        &mut self.data
    }

    /// Gets an element.
    #[inline]
    pub fn get(&self, row: usize, column: usize) -> Option<Complex64> {
        if row >= self.rows || column >= self.columns {
            return None;
        }

        Some(self[(row, column)])
    }

    /// Returns the conjugate transpose.
    pub fn dagger(&self) -> Self {
        let mut result = Self {
            rows: self.columns,
            columns: self.rows,
            data: vec![
                Complex64::zero();
                self.rows * self.columns
            ],
        };

        for row in 0..self.rows {
            for column in 0..self.columns {
                result[(column, row)] = self[(row, column)].conjugate();
            }
        }

        result
    }

    /// Matrix trace.
    pub fn trace(&self) -> Result<Complex64, FidelityError> {
        if self.rows != self.columns {
            return Err(FidelityError::MatrixNotSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }

        let mut result = Complex64::zero();

        for index in 0..self.rows {
            result = result + self[(index, index)];
        }

        Ok(result)
    }

    /// Matrix multiplication.
    pub fn multiply(
        &self,
        rhs: &Self,
    ) -> Result<Self, FidelityError> {
        if self.columns != rhs.rows {
            return Err(FidelityError::MatrixDimensionMismatch {
                left_rows: self.rows,
                left_columns: self.columns,
                right_rows: rhs.rows,
                right_columns: rhs.columns,
            });
        }

        let size = self
            .rows
            .checked_mul(rhs.columns)
            .ok_or(FidelityError::DimensionOverflow {
                dimension: self.rows,
            })?;

        let mut data = vec![Complex64::zero(); size];

        for row in 0..self.rows {
            for column in 0..rhs.columns {
                let mut value = Complex64::zero();

                for k in 0..self.columns {
                    value = value + self[(row, k)] * rhs[(k, column)];
                }

                data[row * rhs.columns + column] = value;
            }
        }

        Self::new(self.rows, rhs.columns, data)
    }

    /// Scales the matrix by a real scalar.
    pub fn scale(&self, scalar: f64) -> Self {
        let mut result = self.clone();

        for value in &mut result.data {
            *value = *value * scalar;
        }

        result
    }

    /// Returns the maximum element-wise absolute difference between this
    /// matrix and another matrix.
    pub fn max_difference(
        &self,
        rhs: &Self,
    ) -> Result<f64, FidelityError> {
        if self.shape() != rhs.shape() {
            return Err(FidelityError::MatrixDimensionMismatch {
                left_rows: self.rows,
                left_columns: self.columns,
                right_rows: rhs.rows,
                right_columns: rhs.columns,
            });
        }

        let mut maximum = 0.0;

        for index in 0..self.data.len() {
            let difference = self.data[index] - rhs.data[index];
            maximum = maximum.max(difference.norm());
        }

        Ok(maximum)
    }
}

impl std::ops::Index<(usize, usize)> for ComplexMatrix {
    type Output = Complex64;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;
        &self.data[row * self.columns + column]
    }
}

impl std::ops::IndexMut<(usize, usize)> for ComplexMatrix {
    #[inline]
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output {
        let (row, column) = index;
        &mut self.data[index.0 * self.columns + index.1]
    }
}

// =============================================================================
// Density-matrix validation
// =============================================================================

/// Validates a density matrix using production defaults.
pub fn validate_density_matrix(
    density_matrix: &ComplexMatrix,
    tolerances: FidelityTolerance,
) -> Result<(), FidelityError> {
    let tolerances = tolerances.validate()?;

    if density_matrix.rows == 0 || density_matrix.columns == 0 {
        return Err(FidelityError::EmptyMatrix);
    }

    if density_matrix.rows != density_matrix.columns {
        return Err(FidelityError::MatrixNotSquare {
            rows: density_matrix.rows,
            columns: density_matrix.columns,
        });
    }

    // -------------------------------------------------------------------------
    // Finite values
    // -------------------------------------------------------------------------

    for value in &density_matrix.data {
        if !value.is_finite() {
            return Err(FidelityError::NonFiniteValue {
                context: "density-matrix element",
            });
        }
    }

    // -------------------------------------------------------------------------
    // Hermiticity
    // -------------------------------------------------------------------------

    let dagger = density_matrix.dagger();
    let hermiticity_error = density_matrix.max_difference(&dagger)?;

    if hermiticity_error > tolerances.hermiticity {
        return Err(FidelityError::NotHermitian {
            maximum_deviation: hermiticity_error,
            tolerance: tolerances.hermiticity,
        });
    }

    // -------------------------------------------------------------------------
    // Trace
    // -------------------------------------------------------------------------

    let trace = density_matrix.trace()?;

    if !trace.is_finite() {
        return Err(FidelityError::NonFiniteValue {
            context: "density-matrix trace",
        });
    }

    if trace.im.abs() > tolerances.trace
        || (trace.re - 1.0).abs() > tolerances.trace
    {
        return Err(FidelityError::InvalidTrace {
            trace,
            tolerance: tolerances.trace,
        });
    }

    // -------------------------------------------------------------------------
    // Diagonal probability sanity
    // -------------------------------------------------------------------------

    for index in 0..density_matrix.rows {
        let diagonal = density_matrix[(index, index)];

        if diagonal.im.abs() > tolerances.hermiticity {
            return Err(FidelityError::NotHermitian {
                maximum_deviation: diagonal.im.abs(),
                tolerance: tolerances.hermiticity,
            });
        }

        if diagonal.re < -tolerances.positivity {
            return Err(FidelityError::NegativeDiagonal {
                index,
                value: diagonal.re,
                tolerance: tolerances.positivity,
            });
        }
    }

    // -------------------------------------------------------------------------
    // Positive semidefinite validation
    // -------------------------------------------------------------------------
    //
    // A density matrix must be positive semidefinite. Hermiticity plus
    // non-negative diagonal entries is not sufficient. We therefore inspect
    // its eigenvalues.
    //

    let eigenvalues = hermitian_eigenvalues(density_matrix)?;

    let minimum = eigenvalues
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);

    if minimum < -tolerances.positivity {
        return Err(FidelityError::NotPositiveSemidefinite {
            minimum_eigenvalue: minimum,
            tolerance: tolerances.positivity,
        });
    }

    Ok(())
}

// =============================================================================
// Mixed-state Uhlmann fidelity
// =============================================================================

/// Calculates squared Uhlmann-Jozsa fidelity between two density matrices.
///
/// ```text
/// F(rho, sigma)
///   = [Tr sqrt(sqrt(rho) sigma sqrt(rho))]^2
/// ```
///
/// Both matrices are validated as density matrices before calculation.
pub fn density_matrix_fidelity(
    left: &ComplexMatrix,
    right: &ComplexMatrix,
) -> Result<FidelityResult, FidelityError> {
    density_matrix_fidelity_with_tolerance(
        left,
        right,
        FidelityTolerance::default(),
    )
}

/// Calculates mixed-state fidelity with explicit tolerances.
pub fn density_matrix_fidelity_with_tolerance(
    left: &ComplexMatrix,
    right: &ComplexMatrix,
    tolerances: FidelityTolerance,
) -> Result<FidelityResult, FidelityError> {
    let tolerances = tolerances.validate()?;

    validate_density_matrix(left, tolerances)?;
    validate_density_matrix(right, tolerances)?;

    if left.shape() != right.shape() {
        return Err(FidelityError::MatrixDimensionMismatch {
            left_rows: left.rows,
            left_columns: left.columns,
            right_rows: right.rows,
            right_columns: right.columns,
        });
    }

    // sqrt(rho)
    let sqrt_left = hermitian_matrix_square_root(
        left,
        tolerances.positivity,
    )?;

    // sqrt(rho) sigma sqrt(rho)
    let middle = sqrt_left
        .multiply(right)?
        .multiply(&sqrt_left)?;

    // sqrt(sqrt(rho) sigma sqrt(rho))
    let sqrt_middle = hermitian_matrix_square_root(
        &middle,
        tolerances.positivity,
    )?;

    let trace = sqrt_middle.trace()?;

    if !trace.is_finite() {
        return Err(FidelityError::NonFiniteValue {
            context: "Uhlmann fidelity trace",
        });
    }

    if trace.im.abs() > tolerances.result {
        return Err(FidelityError::NonFiniteValue {
            context: "Uhlmann fidelity imaginary residual",
        });
    }

    let fidelity = trace.re * trace.re;

    FidelityResult::new(
        fidelity,
        FidelityDefinition::UhlmannSquared,
        tolerances.result,
    )
}

// =============================================================================
// Process fidelity
// =============================================================================

/// Calculates process fidelity from normalized Choi states.
///
/// The matrices must represent normalized Choi states, i.e. density
/// matrices with trace one.
///
/// For normalized Choi states this is exactly the state fidelity of the
/// corresponding process representations.
pub fn process_fidelity(
    ideal_choi: &ComplexMatrix,
    actual_choi: &ComplexMatrix,
) -> Result<FidelityResult, FidelityError> {
    let result = density_matrix_fidelity(
        ideal_choi,
        actual_choi,
    )?;

    FidelityResult::new(
        result.value,
        FidelityDefinition::Process,
        result.tolerance,
    )
}

// =============================================================================
// Average gate fidelity
// =============================================================================

/// Calculates average gate fidelity from process fidelity.
///
/// For a channel acting on a d-dimensional Hilbert space:
///
/// ```text
/// F_avg = (d * F_pro + 1) / (d + 1)
/// ```
///
/// `process_fidelity` must be the normalized process fidelity corresponding
/// to the channel comparison.
pub fn average_gate_fidelity(
    process_fidelity: f64,
    hilbert_dimension: usize,
) -> Result<FidelityResult, FidelityError> {
    average_gate_fidelity_with_tolerance(
        process_fidelity,
        hilbert_dimension,
        DEFAULT_FIDELITY_TOLERANCE,
    )
}

/// Calculates average gate fidelity with explicit tolerance.
pub fn average_gate_fidelity_with_tolerance(
    process_fidelity: f64,
    hilbert_dimension: usize,
    tolerance: f64,
) -> Result<FidelityResult, FidelityError> {
    validate_tolerance(tolerance)?;

    if hilbert_dimension == 0 {
        return Err(FidelityError::InvalidHilbertDimension);
    }

    let process =
        FidelityResult::new(
            process_fidelity,
            FidelityDefinition::Process,
            tolerance,
        )?;

    let dimension = hilbert_dimension as f64;

    let denominator = dimension + 1.0;

    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(FidelityError::DimensionOverflow {
            dimension: hilbert_dimension,
        });
    }

    let average =
        (dimension * process.value + 1.0) / denominator;

    FidelityResult::new(
        average,
        FidelityDefinition::AverageGate,
        tolerance,
    )
}

/// Converts a qubit count to Hilbert-space dimension.
///
/// This is checked to avoid overflow from `2^n`.
pub fn qubit_count_to_hilbert_dimension(
    qubits: usize,
) -> Result<usize, FidelityError> {
    if qubits >= usize::BITS as usize {
        return Err(FidelityError::DimensionOverflow {
            dimension: qubits,
        });
    }

    Ok(1usize << qubits)
}

// =============================================================================
// Bures metrics
// =============================================================================

/// Calculates the Bures distance from a squared fidelity.
///
/// ```text
/// B = sqrt(2 - 2 sqrt(F))
/// ```
///
/// This is provided separately from fidelity so callers do not accidentally
/// interpret a distance as a fidelity.
pub fn bures_distance(
    fidelity: f64,
) -> Result<f64, FidelityError> {
    validate_unit_interval(fidelity, DEFAULT_FIDELITY_TOLERANCE)?;

    let value =
        (2.0 - 2.0 * fidelity.sqrt()).max(0.0).sqrt();

    if !value.is_finite() {
        return Err(FidelityError::NonFiniteValue {
            context: "Bures distance",
        });
    }

    Ok(value)
}

/// Calculates the Bures angle.
///
/// ```text
/// A = acos(sqrt(F))
/// ```
pub fn bures_angle(
    fidelity: f64,
) -> Result<f64, FidelityError> {
    validate_unit_interval(fidelity, DEFAULT_FIDELITY_TOLERANCE)?;

    Ok(clamp_unit_interval(fidelity).sqrt().acos())
}

// =============================================================================
// Infidelity
// =============================================================================

/// Calculates infidelity.
///
/// ```text
/// 1 - F
/// ```
pub fn infidelity(
    fidelity: f64,
) -> Result<f64, FidelityError> {
    validate_unit_interval(fidelity, DEFAULT_FIDELITY_TOLERANCE)?;

    Ok(1.0 - clamp_unit_interval(fidelity))
}

// =============================================================================
// Internal numerical helpers
// =============================================================================

fn validate_tolerance(
    tolerance: f64,
) -> Result<(), FidelityError> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(FidelityError::NonFiniteValue {
            context: "fidelity tolerance",
        });
    }

    Ok(())
}

fn validate_unit_interval(
    value: f64,
    tolerance: f64,
) -> Result<(), FidelityError> {
    if !value.is_finite() {
        return Err(FidelityError::NonFiniteValue {
            context: "unit-interval metric",
        });
    }

    if value < -tolerance || value > 1.0 + tolerance {
        return Err(FidelityError::FidelityOutOfRange {
            value,
            tolerance,
        });
    }

    Ok(())
}

#[inline]
fn clamp_unit_interval(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

// =============================================================================
// Hermitian eigendecomposition
// =============================================================================

/// Calculates eigenvalues of a Hermitian matrix using a Jacobi rotation
/// algorithm.
///
/// The implementation is intentionally dependency-free. It is designed for
/// small-to-moderate matrices used by state/process characterization and
/// benchmark analysis. It is not presented as a replacement for optimized
/// large-scale linear algebra libraries.
fn hermitian_eigenvalues(
    matrix: &ComplexMatrix,
) -> Result<Vec<f64>, FidelityError> {
    if matrix.rows != matrix.columns {
        return Err(FidelityError::MatrixNotSquare {
            rows: matrix.rows,
            columns: matrix.columns,
        });
    }

    if matrix.rows == 0 {
        return Err(FidelityError::EmptyMatrix);
    }

    let mut working = matrix.clone();

    let n = working.rows;

    const MAX_SWEEPS_FACTOR: usize = 64;

    let max_sweeps = MAX_SWEEPS_FACTOR
        .checked_mul(n.max(1))
        .and_then(|value| value.checked_mul(n.max(1)))
        .ok_or(FidelityError::DimensionOverflow {
            dimension: n,
        })?;

    let convergence_tolerance = 1.0e-13;

    for _ in 0..max_sweeps {
        let mut max_off_diagonal = 0.0;
        let mut pivot_p = 0;
        let mut pivot_q = 0;

        for p in 0..n {
            for q in (p + 1)..n {
                let magnitude = working[(p, q)].norm();

                if magnitude > max_off_diagonal {
                    max_off_diagonal = magnitude;
                    pivot_p = p;
                    pivot_q = q;
                }
            }
        }

        if max_off_diagonal <= convergence_tolerance {
            let mut eigenvalues = Vec::with_capacity(n);

            for index in 0..n {
                let value = working[(index, index)].re;

                if !value.is_finite() {
                    return Err(FidelityError::NonFiniteValue {
                        context: "Hermitian eigenvalue",
                    });
                }

                eigenvalues.push(value);
            }

            eigenvalues.sort_by(|a, b| {
                a.partial_cmp(b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            return Ok(eigenvalues);
        }

        apply_complex_jacobi_rotation(
            &mut working,
            pivot_p,
            pivot_q,
        )?;
    }

    Err(FidelityError::EigenDecompositionFailed)
}

/// Applies a unitary Jacobi rotation to eliminate one off-diagonal complex
/// entry of a Hermitian matrix.
fn apply_complex_jacobi_rotation(
    matrix: &mut ComplexMatrix,
    p: usize,
    q: usize,
) -> Result<(), FidelityError> {
    if p == q {
        return Ok(());
    }

    let app = matrix[(p, p)].re;
    let aqq = matrix[(q, q)].re;
    let apq = matrix[(p, q)];

    let magnitude = apq.norm();

    if magnitude <= 1.0e-15 {
        return Ok(());
    }

    let phase = apq / magnitude;

    let tau = (aqq - app) / (2.0 * magnitude);

    let t = if tau >= 0.0 {
        1.0 / (tau + (1.0 + tau * tau).sqrt())
    } else {
        -1.0 / (-tau + (1.0 + tau * tau).sqrt())
    };

    let c = 1.0 / (1.0 + t * t).sqrt();
    let s_real = c * t;

    let s = phase * s_real;

    // Preserve the diagonal entries required by the unitary similarity
    // transformation.
    let new_app = app - t * magnitude;
    let new_aqq = aqq + t * magnitude;

    matrix[(p, p)] = Complex64::new(new_app, 0.0);
    matrix[(q, q)] = Complex64::new(new_aqq, 0.0);
    matrix[(p, q)] = Complex64::zero();
    matrix[(q, p)] = Complex64::zero();

    // Apply the transformation to the remaining rows/columns.
    for k in 0..matrix.rows {
        if k == p || k == q {
            continue;
        }

        let akp = matrix[(k, p)];
        let akq = matrix[(k, q)];

        let new_kp =
            akp * c - akq * s.conjugate();

        let new_kq =
            akp * s + akq * c;

        matrix[(k, p)] = new_kp;
        matrix[(p, k)] = new_kp.conjugate();

        matrix[(k, q)] = new_kq;
        matrix[(q, k)] = new_kq.conjugate();
    }

    Ok(())
}

/// Calculates a Hermitian positive-semidefinite matrix square root.
fn hermitian_matrix_square_root(
    matrix: &ComplexMatrix,
    positivity_tolerance: f64,
) -> Result<ComplexMatrix, FidelityError> {
    if matrix.rows != matrix.columns {
        return Err(FidelityError::MatrixNotSquare {
            rows: matrix.rows,
            columns: matrix.columns,
        });
    }

    let hermiticity_error =
        matrix.max_difference(&matrix.dagger())?;

    if hermiticity_error > positivity_tolerance {
        return Err(FidelityError::NotHermitian {
            maximum_deviation: hermiticity_error,
            tolerance: positivity_tolerance,
        });
    }

    let (eigenvalues, eigenvectors) =
        hermitian_eigendecomposition(matrix)?;

    let n = matrix.rows;

    let mut diagonal_sqrt =
        ComplexMatrix::new(
            n,
            n,
            vec![Complex64::zero(); n * n],
        )?;

    for index in 0..n {
        let eigenvalue = eigenvalues[index];

        if eigenvalue < -positivity_tolerance {
            return Err(FidelityError::NotPositiveSemidefinite {
                minimum_eigenvalue: eigenvalue,
                tolerance: positivity_tolerance,
            });
        }

        diagonal_sqrt[(index, index)] =
            Complex64::new(
                eigenvalue.max(0.0).sqrt(),
                0.0,
            );
    }

    let first =
        eigenvectors.multiply(&diagonal_sqrt)?;

    first.multiply(&eigenvectors.dagger())
}

/// Hermitian eigendecomposition.
///
/// Returns eigenvalues and a unitary eigenvector matrix whose columns are
/// eigenvectors.
fn hermitian_eigendecomposition(
    matrix: &ComplexMatrix,
) -> Result<(Vec<f64>, ComplexMatrix), FidelityError> {
    if matrix.rows != matrix.columns {
        return Err(FidelityError::MatrixNotSquare {
            rows: matrix.rows,
            columns: matrix.columns,
        });
    }

    if matrix.rows == 0 {
        return Err(FidelityError::EmptyMatrix);
    }

    let n = matrix.rows;

    let mut working = matrix.clone();

    let mut vectors = ComplexMatrix::identity(n)?;

    const MAX_SWEEPS_FACTOR: usize = 64;

    let max_sweeps = MAX_SWEEPS_FACTOR
        .checked_mul(n.max(1))
        .and_then(|value| value.checked_mul(n.max(1)))
        .ok_or(FidelityError::DimensionOverflow {
            dimension: n,
        })?;

    let convergence_tolerance = 1.0e-13;

    for _ in 0..max_sweeps {
        let mut max_off_diagonal = 0.0;
        let mut pivot_p = 0;
        let mut pivot_q = 0;

        for p in 0..n {
            for q in (p + 1)..n {
                let magnitude = working[(p, q)].norm();

                if magnitude > max_off_diagonal {
                    max_off_diagonal = magnitude;
                    pivot_p = p;
                    pivot_q = q;
                }
            }
        }

        if max_off_diagonal <= convergence_tolerance {
            let mut eigenvalues = Vec::with_capacity(n);

            for index in 0..n {
                eigenvalues.push(working[(index, index)].re);
            }

            return Ok((eigenvalues, vectors));
        }

        apply_complex_jacobi_rotation_with_vectors(
            &mut working,
            &mut vectors,
            pivot_p,
            pivot_q,
        )?;
    }

    Err(FidelityError::EigenDecompositionFailed)
}

/// Jacobi rotation with simultaneous eigenvector accumulation.
fn apply_complex_jacobi_rotation_with_vectors(
    matrix: &mut ComplexMatrix,
    vectors: &mut ComplexMatrix,
    p: usize,
    q: usize,
) -> Result<(), FidelityError> {
    if p == q {
        return Ok(());
    }

    let app = matrix[(p, p)].re;
    let aqq = matrix[(q, q)].re;
    let apq = matrix[(p, q)];

    let magnitude = apq.norm();

    if magnitude <= 1.0e-15 {
        return Ok(());
    }

    let phase = apq / magnitude;

    let tau = (aqq - app) / (2.0 * magnitude);

    let t = if tau >= 0.0 {
        1.0 / (tau + (1.0 + tau * tau).sqrt())
    } else {
        -1.0 / (-tau + (1.0 + tau * tau).sqrt())
    };

    let c = 1.0 / (1.0 + t * t).sqrt();
    let s = phase * (c * t);

    let new_app = app - t * magnitude;
    let new_aqq = aqq + t * magnitude;

    matrix[(p, p)] = Complex64::new(new_app, 0.0);
    matrix[(q, q)] = Complex64::new(new_aqq, 0.0);
    matrix[(p, q)] = Complex64::zero();
    matrix[(q, p)] = Complex64::zero();

    for k in 0..n_of(matrix) {
        if k == p || k == q {
            continue;
        }

        let akp = matrix[(k, p)];
        let akq = matrix[(k, q)];

        let new_kp =
            akp * c - akq * s.conjugate();

        let new_kq =
            akp * s + akq * c;

        matrix[(k, p)] = new_kp;
        matrix[(p, k)] = new_kp.conjugate();

        matrix[(k, q)] = new_kq;
        matrix[(q, k)] = new_kq.conjugate();
    }

    // Accumulate the same right-side unitary transformation into the
    // eigenvector matrix.
    for k in 0..n_of(vectors) {
        let vkp = vectors[(k, p)];
        let vkq = vectors[(k, q)];

        vectors[(k, p)] =
            vkp * c - vkq * s.conjugate();

        vectors[(k, q)] =
            vkp * s + vkq * c;
    }

    Ok(())
}

#[inline]
fn n_of(matrix: &ComplexMatrix) -> usize {
    matrix.rows()
}

// =============================================================================
// Convenience APIs
// =============================================================================

/// Calculates fidelity between a pure reference state and an observed state
/// represented either as a state vector or density matrix.
///
/// This enum is useful at future protocol integration boundaries where the
/// backend may expose either representation.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumState {
    /// Normalized pure state.
    Pure(Vec<Complex64>),

    /// Density matrix.
    Mixed(ComplexMatrix),
}

impl QuantumState {
    /// Calculates fidelity against another quantum state.
    pub fn fidelity(
        &self,
        other: &Self,
    ) -> Result<FidelityResult, FidelityError> {
        match (self, other) {
            (Self::Pure(left), Self::Pure(right)) => {
                pure_state_fidelity(left, right)
            }

            (Self::Pure(state), Self::Mixed(matrix)) => {
                pure_state_density_fidelity(state, matrix)
            }

            (Self::Mixed(matrix), Self::Pure(state)) => {
                pure_state_density_fidelity(state, matrix)
            }

            (Self::Mixed(left), Self::Mixed(right)) => {
                density_matrix_fidelity(left, right)
            }
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn zero_state() -> Vec<Complex64> {
        vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
        ]
    }

    fn one_state() -> Vec<Complex64> {
        vec![
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        ]
    }

    fn plus_state() -> Vec<Complex64> {
        let amplitude = 1.0 / 2.0_f64.sqrt();

        vec![
            Complex64::new(amplitude, 0.0),
            Complex64::new(amplitude, 0.0),
        ]
    }

    fn pure_zero_density_matrix() -> ComplexMatrix {
        ComplexMatrix::new(
            2,
            2,
            vec![
                Complex64::new(1.0, 0.0),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::zero(),
            ],
        )
        .unwrap()
    }

    fn maximally_mixed_qubit() -> ComplexMatrix {
        ComplexMatrix::new(
            2,
            2,
            vec![
                Complex64::new(0.5, 0.0),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::new(0.5, 0.0),
            ],
        )
        .unwrap()
    }

    #[test]
    fn zero_state_has_unit_self_fidelity() {
        let result =
            pure_state_fidelity(
                &zero_state(),
                &zero_state(),
            )
            .unwrap();

        assert!((result.value - 1.0).abs() < 1.0e-12);
        assert_eq!(
            result.definition,
            FidelityDefinition::PureStateOverlap
        );
    }

    #[test]
    fn_orthogonal_states_have_zero_fidelity() {
        let result =
            pure_state_fidelity(
                &zero_state(),
                &one_state(),
            )
            .unwrap();

        assert!(result.value.abs() < 1.0e-12);
    }

    #[test]
    fn_zero_and_plus_have_half_fidelity() {
        let result =
            pure_state_fidelity(
                &zero_state(),
                &plus_state(),
            )
            .unwrap();

        assert!((result.value - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn_global_phase_does_not_change_pure_state_fidelity() {
        let state = zero_state();

        let phased = vec![
            Complex64::new(
                0.0_f64.cos(),
                0.0_f64.sin(),
            ),
            Complex64::zero(),
        ];

        let result =
            pure_state_fidelity(&state, &phased)
                .unwrap();

        assert!((result.value - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn rejects_unnormalized_state() {
        let state = vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0),
        ];

        let result =
            pure_state_fidelity(
                &state,
                &zero_state(),
            );

        assert!(matches!(
            result,
            Err(FidelityError::StateNotNormalized { .. })
        ));
    }

    #[test]
    fn classical_identical_distribution_has_unit_fidelity() {
        let distribution = vec![0.25, 0.75];

        let result =
            classical_distribution_fidelity(
                &distribution,
                &distribution,
            )
            .unwrap();

        assert!((result.value - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn classical_disjoint_distributions_have_zero_fidelity() {
        let left = vec![1.0, 0.0];
        let right = vec![0.0, 1.0];

        let result =
            classical_distribution_fidelity(
                &left,
                &right,
            )
            .unwrap();

        assert!(result.value.abs() < 1.0e-12);
    }

    #[test]
    fn rejects_invalid_probability_distribution() {
        let invalid = vec![0.5, 0.25];

        let result =
            validate_probability_distribution(
                &invalid,
                DEFAULT_FIDELITY_TOLERANCE,
            );

        assert!(matches!(
            result,
            Err(FidelityError::DistributionNotNormalized { .. })
        ));
    }

    #[test]
    fn pure_state_density_fidelity_is_correct() {
        let result =
            pure_state_density_fidelity(
                &zero_state(),
                &pure_zero_density_matrix(),
            )
            .unwrap();

        assert!((result.value - 1.0).abs() < 1.0e-12);
        assert_eq!(
            result.definition,
            FidelityDefinition::PureStateDensity
        );
    }

    #[test]
    fn pure_state_against_maximally_mixed_state_is_half() {
        let result =
            pure_state_density_fidelity(
                &zero_state(),
                &maximally_mixed_qubit(),
            )
            .unwrap();

        assert!((result.value - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn density_matrix_self_fidelity_is_one() {
        let state =
            pure_zero_density_matrix();

        let result =
            density_matrix_fidelity(
                &state,
                &state,
            )
            .unwrap();

        assert!((result.value - 1.0).abs() < 1.0e-10);
    }

    #[test]
    fn maximally_mixed_self_fidelity_is_one() {
        let state =
            maximally_mixed_qubit();

        let result =
            density_matrix_fidelity(
                &state,
                &state,
            )
            .unwrap();

        assert!((result.value - 1.0).abs() < 1.0e-10);
    }

    #[test]
    fn orthogonal_pure_density_matrices_have_zero_fidelity() {
        let zero =
            pure_zero_density_matrix();

        let one =
            ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::zero(),
                    Complex64::zero(),
                    Complex64::zero(),
                    Complex64::new(1.0, 0.0),
                ],
            )
            .unwrap();

        let result =
            density_matrix_fidelity(
                &zero,
                &one,
            )
            .unwrap();

        assert!(result.value.abs() < 1.0e-10);
    }

    #[test]
    fn rejects_non_hermitian_density_matrix() {
        let matrix =
            ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::new(0.5, 0.0),
                    Complex64::new(0.0, 0.2),
                    Complex64::new(0.0, 0.0),
                    Complex64::new(0.5, 0.0),
                ],
            )
            .unwrap();

        let result =
            validate_density_matrix(
                &matrix,
                FidelityTolerance::default(),
            );

        assert!(matches!(
            result,
            Err(FidelityError::NotHermitian { .. })
        ));
    }

    #[test]
    fn rejects_invalid_trace() {
        let matrix =
            ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::new(0.25, 0.0),
                    Complex64::zero(),
                    Complex64::zero(),
                    Complex64::new(0.25, 0.0),
                ],
            )
            .unwrap();

        let result =
            validate_density_matrix(
                &matrix,
                FidelityTolerance::default(),
            );

        assert!(matches!(
            result,
            Err(FidelityError::InvalidTrace { .. })
        ));
    }

    #[test]
    fn average_gate_fidelity_of_perfect_channel_is_one() {
        let result =
            average_gate_fidelity(
                1.0,
                2,
            )
            .unwrap();

        assert!((result.value - 1.0).abs() < 1.0e-12);
        assert_eq!(
            result.definition,
            FidelityDefinition::AverageGate
        );
    }

    #[test]
    fn average_gate_fidelity_formula_is_correct() {
        let result =
            average_gate_fidelity(
                0.5,
                2,
            )
            .unwrap();

        let expected =
            (2.0 * 0.5 + 1.0) / 3.0;

        assert!((result.value - expected).abs() < 1.0e-12);
    }

    #[test]
    fn qubit_dimension_conversion_is_checked() {
        assert_eq!(
            qubit_count_to_hilbert_dimension(0)
                .unwrap(),
            1
        );

        assert_eq!(
            qubit_count_to_hilbert_dimension(1)
                .unwrap(),
            2
        );

        assert_eq!(
            qubit_count_to_hilbert_dimension(3)
                .unwrap(),
            8
        );
    }

    #[test]
    fn infidelity_is_complement() {
        let result =
            infidelity(0.875).unwrap();

        assert!((result - 0.125).abs() < 1.0e-12);
    }

    #[test]
    fn perfect_fidelity_has_zero_bures_distance() {
        let result =
            bures_distance(1.0).unwrap();

        assert!(result.abs() < 1.0e-12);
    }

    #[test]
    fn zero_fidelity_has_pi_over_two_bures_angle() {
        let result =
            bures_angle(0.0).unwrap();

        assert!(
            (result - std::f64::consts::FRAC_PI_2).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn quantum_state_dispatches_correctly() {
        let left =
            QuantumState::Pure(zero_state());

        let right =
            QuantumState::Pure(zero_state());

        let result =
            left.fidelity(&right).unwrap();

        assert!((result.value - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn fidelity_result_rejects_non_finite_values() {
        let result =
            FidelityResult::new(
                f64::NAN,
                FidelityDefinition::PureStateOverlap,
                DEFAULT_FIDELITY_TOLERANCE,
            );

        assert!(matches!(
            result,
            Err(FidelityError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn fidelity_result_rejects_values_outside_range() {
        let result =
            FidelityResult::new(
                1.5,
                FidelityDefinition::PureStateOverlap,
                DEFAULT_FIDELITY_TOLERANCE,
            );

        assert!(matches!(
            result,
            Err(FidelityError::FidelityOutOfRange { .. })
        ));
    }

    #[test]
    fn density_matrix_shape_is_preserved() {
        let matrix =
            ComplexMatrix::identity(4)
                .unwrap();

        assert_eq!(
            matrix.shape(),
            (4, 4)
        );
    }

    #[test]
    fn matrix_multiplication_identity_is_correct() {
        let matrix =
            ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::new(1.0, 0.0),
                    Complex64::new(2.0, 0.0),
                    Complex64::new(3.0, 0.0),
                    Complex64::new(4.0, 0.0),
                ],
            )
            .unwrap();

        let identity =
            ComplexMatrix::identity(2)
                .unwrap();

        let result =
            matrix
                .multiply(&identity)
                .unwrap();

        assert_eq!(
            result,
            matrix
        );
    }

    #[test]
    fn density_matrix_fidelity_is_symmetric() {
        let left =
            pure_zero_density_matrix();

        let right =
            maximally_mixed_qubit();

        let forward =
            density_matrix_fidelity(
                &left,
                &right,
            )
            .unwrap();

        let reverse =
            density_matrix_fidelity(
                &right,
                &left,
            )
            .unwrap();

        assert!(
            (forward.value - reverse.value).abs()
                < 1.0e-10
        );
    }

    #[test]
    fn fidelity_never_reports_outside_unit_interval() {
        let result =
            pure_state_fidelity(
                &plus_state(),
                &plus_state(),
            )
            .unwrap();

        assert!(
            result.value >= 0.0
                && result.value <= 1.0
        );
    }
}