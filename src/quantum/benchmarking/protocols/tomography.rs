//! Zamani Quantum Benchmarking — Tomography Protocols
//!
//! Production-grade, backend-independent state and process tomography
//! reconstruction and analysis.
//!
//! # Scope
//!
//! This module owns the mathematical reconstruction and validation layer for
//! tomography. It deliberately does NOT:
//!
//! - execute quantum circuits;
//! - communicate with quantum hardware;
//! - select a backend;
//! - transpile circuits;
//! - route circuits;
//! - schedule circuits;
//! - own Quantum IR;
//! - construct backend-specific execution requests;
//! - perform device calibration;
//! - silently repair invalid experimental data;
//! - silently clip negative probabilities;
//! - silently normalize malformed distributions;
//! - print diagnostics;
//! - mutate global state.
//!
//! Execution belongs to `benchmarking::execution` and backend integration
//! belongs to the quantum hardware/runtime layers.
//!
//! # Supported tomography modes
//!
//! The module provides:
//!
//! - single-qubit state tomography;
//! - multi-qubit Pauli state tomography;
//! - computational-basis state tomography;
//! - linear-inversion state reconstruction;
//! - process tomography through normalized Choi-state reconstruction;
//! - Pauli-basis process tomography;
//! - trace validation;
//! - Hermiticity validation;
//! - positivity/physicality validation;
//! - trace-preserving validation for reconstructed channels;
//! - deterministic measurement ordering;
//! - explicit uncertainty metadata;
//! - optional confidence metadata;
//! - resource limits;
//! - reproducible experiment metadata;
//! - conversion to the canonical fidelity matrix representation.
//!
//! # Important scientific limitation
//!
//! Linear inversion is an estimator, not a maximum-likelihood estimator.
//! Finite-shot linear inversion can produce a Hermitian trace-one matrix that
//! is not positive semidefinite. This module therefore never silently projects
//! an unphysical estimate onto the physical state/channel set.
//!
//! If a physical estimator is desired, a separate constrained estimator should
//! be implemented behind an explicit API, for example:
//!
//! `maximum_likelihood.rs`
//!
//! Such an estimator must never be confused with linear inversion.
//!
//! # State tomography
//!
//! For an n-qubit system, Pauli tomography represents a state as
//!
//! ```text
//! rho = 1 / 2^n * sum_P <P> P
//! ```
//!
//! where `P` ranges over the n-qubit Pauli basis.
//!
//! Each supplied Pauli expectation value is associated with a measurement
//! outcome count. The expectation value is calculated from the observed
//! +/-1 eigenvalue counts.
//!
//! # Process tomography
//!
//! This module represents a quantum channel through its normalized Choi state.
//! The normalized Choi representation has trace one and therefore can be
//! compared using the canonical process-fidelity machinery.
//!
//! For a d-dimensional channel:
//!
//! ```text
//! J_normalized = J / d
//! Tr(J_normalized) = 1
//! ```
//!
//! The process tomography reconstruction implemented here uses Pauli transfer
//! data to reconstruct the normalized Choi state.
//!
//! The reconstructed Choi state is then suitable for consumption by the
//! existing `benchmarking::protocols::process_fidelity` layer after explicit
//! validation.
//!
//! # Architecture
//!
//! ```text
//! benchmark experiment\n
//!        │\n//!        ▼\n//! execution layer\n//!        │\n//!        ▼\n//! raw tomography observations\n//!        │\n//!        ▼\n//! protocols::tomography\n//!        │\n//!        ├── state reconstruction\n//!        │\n//!        └── process reconstruction\n//!                 │\n//!                 ▼\n//!        validated density matrix\n//!                 │\n//!                 ▼\n//!        protocols::process_fidelity\n//!                 │\n//!                 ▼\n//!        canonical BenchmarkResult\n//! ```
//!
//! # Integration contract
//!
//! This module intentionally has no dependency on Quantum IR or execution.
//!
//! Intended consumers:
//!
//! ```text
//! execution::*
//!     -> TomographyObservation
//!     -> StateTomographyInput
//!     -> ProcessTomographyInput
//!
//! protocols::process_fidelity
//!     -> reconstructed normalized Choi matrix
//!
//! metrics::fidelity
//!     -> density-matrix fidelity
//!
//! reporting::*
//!     -> TomographyResult metadata
//!
//! analysis::*
//!     -> reconstructed-state/process diagnostics
//! ```
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! No numerical dependency is required.
//!
//! The repository's canonical complex-matrix/fidelity implementation remains
//! the single owner of downstream fidelity mathematics.
//!
//! # Production invariants
//!
//! 1. Every probability must be finite and in [0, 1].
//! 2. Every count must be non-negative.
//! 3. Every denominator must be non-zero.
//! 4. Every dimension must be checked for overflow.
//! 5. Every matrix must have consistent dimensions.
//! 6. Trace must be explicitly validated.
//! 7. Hermiticity must be explicitly validated.
//! 8. Positivity must be explicitly validated when physicality is requested.
//! 9. Trace preservation must be explicitly validated for process channels.
//! 10. No invalid numerical input is silently corrected.
//! 11. No measurement is silently discarded.
//! 12. Measurement labels have deterministic ordering.
//! 13. Resource limits are checked before allocation.
//! 14. Reconstruction metadata records the estimator used.
//! 15. Linear inversion is never represented as maximum-likelihood estimation.
//!
//! # Integration status
//!
//! This file is intentionally self-contained at the protocol boundary.
//! No modification to the statistical or fidelity foundations is required to
//! use the reconstruction types. The existing process-fidelity layer can
//! consume the resulting normalized Choi matrix through the canonical matrix
//! representation once the surrounding module wiring exposes this module.
//!
//! ---------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// Public identity
// =============================================================================

/// Stable benchmark identifier.
pub const TOMOGRAPHY_BENCHMARK_ID: &str = "tomography";

/// Stable protocol version.
///
/// Increment when the semantic meaning of reconstructed results changes.
pub const TOMOGRAPHY_PROTOCOL_VERSION: &str = "1";

/// Default numerical tolerance.
pub const DEFAULT_TOMOGRAPHY_TOLERANCE: f64 = 1.0e-10;

/// Default trace tolerance.
pub const DEFAULT_TRACE_TOLERANCE: f64 = 1.0e-10;

/// Default Hermiticity tolerance.
pub const DEFAULT_HERMITICITY_TOLERANCE: f64 = 1.0e-10;

/// Default positivity tolerance.
///
/// Eigenvalues smaller than `-DEFAULT_POSITIVITY_TOLERANCE` are treated as
/// physically invalid.
pub const DEFAULT_POSITIVITY_TOLERANCE: f64 = 1.0e-10;

/// Maximum qubit count supported by the dependency-free Pauli reconstruction.
///
/// This is deliberately conservative because Pauli tomography scales as
/// 4^n. Larger tomography workloads should use a specialized sparse or
/// compressed estimator.
pub const DEFAULT_MAX_QUBITS: usize = 8;

/// Maximum matrix elements allowed by default.
pub const DEFAULT_MAX_MATRIX_ELEMENTS: usize = 16_777_216;

/// Maximum number of measurement settings allowed by default.
pub const DEFAULT_MAX_MEASUREMENT_SETTINGS: usize = 65_536;

/// Maximum total shots represented by one tomography input.
pub const DEFAULT_MAX_TOTAL_SHOTS: u64 = 100_000_000;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by tomography reconstruction and validation.
#[derive(Debug, Clone, PartialEq)]
pub enum TomographyError {
    /// A numerical input was NaN or infinite.
    NonFiniteValue {
        /// Context describing the invalid input.
        context: &'static str,
    },

    /// A numerical tolerance is invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        value: f64,
    },

    /// The number of qubits is invalid.
    InvalidQubitCount {
        /// Supplied number of qubits.
        qubits: usize,
    },

    /// The requested reconstruction exceeds the configured resource limit.
    ResourceLimitExceeded {
        /// Resource being limited.
        resource: &'static str,

        /// Requested amount.
        requested: u128,

        /// Maximum permitted amount.
        maximum: u128,
    },

    /// A matrix dimension is invalid.
    InvalidMatrixDimension {
        /// Number of rows.
        rows: usize,

        /// Number of columns.
        columns: usize,
    },

    /// A matrix is not square.
    MatrixNotSquare {
        /// Number of rows.
        rows: usize,

        /// Number of columns.
        columns: usize,
    },

    /// Matrix dimensions are incompatible.
    MatrixDimensionMismatch {
        /// Left rows.
        left_rows: usize,

        /// Left columns.
        left_columns: usize,

        /// Right rows.
        right_rows: usize,

        /// Right columns.
        right_columns: usize,
    },

    /// A matrix contains a non-finite element.
    MatrixContainsNonFinite {
        /// Linear element index.
        index: usize,
    },

    /// A probability is outside [0, 1].
    InvalidProbability {
        /// Measurement index.
        index: usize,

        /// Invalid value.
        value: f64,
    },

    /// A probability distribution does not sum to one.
    DistributionNotNormalized {
        /// Observed sum.
        sum: f64,

        /// Allowed deviation.
        tolerance: f64,
    },

    /// An observed count exceeds the supplied shot count.
    CountExceedsShots {
        /// Positive count.
        positive: u64,

        /// Negative count.
        negative: u64,

        /// Declared shots.
        shots: u64,
    },

    /// Zero shots were supplied.
    ZeroShots,

    /// A supplied expectation value is invalid.
    InvalidExpectation {
        /// Expectation value.
        value: f64,

        /// Tolerance.
        tolerance: f64,
    },

    /// Duplicate measurement setting.
    DuplicateMeasurementSetting,

    /// No measurement settings were supplied.
    EmptyMeasurementSet,

    /// The measurement setting is incompatible with the requested number of
    /// qubits.
    MeasurementDimensionMismatch,

    /// The reconstructed state is not Hermitian.
    NotHermitian {
        /// Maximum Hermiticity deviation.
        maximum_deviation: f64,

        /// Allowed tolerance.
        tolerance: f64,
    },

    /// The reconstructed density matrix has invalid trace.
    InvalidTrace {
        /// Observed trace.
        trace: Complex64,

        /// Allowed tolerance.
        tolerance: f64,
    },

    /// The reconstructed density matrix is not positive semidefinite.
    NotPositiveSemidefinite {
        /// Smallest eigenvalue.
        minimum_eigenvalue: f64,

        /// Allowed tolerance.
        tolerance: f64,
    },

    /// A state reconstruction is incomplete.
    IncompleteStateReconstruction {
        /// Number of expected Pauli coefficients.
        expected: usize,

        /// Number supplied.
        supplied: usize,
    },

    /// A process reconstruction is incomplete.
    IncompleteProcessReconstruction {
        /// Number expected.
        expected: usize,

        /// Number supplied.
        supplied: usize,
    },

    /// A process matrix has the wrong Choi dimension.
    InvalidChoiDimension {
        /// Hilbert dimension.
        hilbert_dimension: usize,

        /// Matrix dimension.
        matrix_dimension: usize,
    },

    /// The reconstructed channel is not trace preserving.
    NotTracePreserving {
        /// Maximum partial-trace deviation.
        maximum_deviation: f64,

        /// Allowed tolerance.
        tolerance: f64,
    },

    /// Integer arithmetic overflow occurred.
    ArithmeticOverflow,

    /// A matrix eigendecomposition did not converge.
    EigenDecompositionFailed,

    /// A required denominator is zero.
    ZeroDenominator,
}

impl fmt::Display for TomographyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { context } => {
                write!(formatter, "tomography input contains a non-finite value: {context}")
            }

            Self::InvalidTolerance { value } => {
                write!(
                    formatter,
                    "tomography tolerance must be finite and >= 0: {value}"
                )
            }

            Self::InvalidQubitCount { qubits } => {
                write!(formatter, "invalid tomography qubit count: {qubits}")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "tomography resource limit exceeded for {resource}: \
                     requested={requested}, maximum={maximum}"
                )
            }

            Self::InvalidMatrixDimension { rows, columns } => {
                write!(
                    formatter,
                    "invalid tomography matrix dimensions: {rows}x{columns}"
                )
            }

            Self::MatrixNotSquare { rows, columns } => {
                write!(
                    formatter,
                    "tomography matrix must be square: {rows}x{columns}"
                )
            }

            Self::MatrixDimensionMismatch {
                left_rows,
                left_columns,
                right_rows,
                right_columns,
            } => {
                write!(
                    formatter,
                    "tomography matrix dimensions differ: \
                     left={left_rows}x{left_columns}, \
                     right={right_rows}x{right_columns}"
                )
            }

            Self::MatrixContainsNonFinite { index } => {
                write!(
                    formatter,
                    "tomography matrix contains a non-finite value at index {index}"
                )
            }

            Self::InvalidProbability { index, value } => {
                write!(
                    formatter,
                    "invalid tomography probability at index {index}: {value}"
                )
            }

            Self::DistributionNotNormalized { sum, tolerance } => {
                write!(
                    formatter,
                    "tomography probability distribution is not normalized: \
                     sum={sum}, tolerance={tolerance}"
                )
            }

            Self::CountExceedsShots {
                positive,
                negative,
                shots,
            } => {
                write!(
                    formatter,
                    "tomography counts exceed declared shots: \
                     positive={positive}, negative={negative}, shots={shots}"
                )
            }

            Self::ZeroShots => {
                write!(formatter, "tomography requires at least one shot")
            }

            Self::InvalidExpectation { value, tolerance } => {
                write!(
                    formatter,
                    "invalid tomography expectation value: \
                     value={value}, tolerance={tolerance}"
                )
            }

            Self::DuplicateMeasurementSetting => {
                write!(formatter, "duplicate tomography measurement setting")
            }

            Self::EmptyMeasurementSet => {
                write!(formatter, "tomography measurement set must not be empty")
            }

            Self::MeasurementDimensionMismatch => {
                write!(formatter, "tomography measurement dimension mismatch")
            }

            Self::NotHermitian {
                maximum_deviation,
                tolerance,
            } => {
                write!(
                    formatter,
                    "tomography density matrix is not Hermitian: \
                     maximum_deviation={maximum_deviation}, tolerance={tolerance}"
                )
            }

            Self::InvalidTrace { trace, tolerance } => {
                write!(
                    formatter,
                    "tomography density matrix has invalid trace: \
                     trace={trace}, tolerance={tolerance}"
                )
            }

            Self::NotPositiveSemidefinite {
                minimum_eigenvalue,
                tolerance,
            } => {
                write!(
                    formatter,
                    "tomography density matrix is not positive semidefinite: \
                     minimum_eigenvalue={minimum_eigenvalue}, tolerance={tolerance}"
                )
            }

            Self::IncompleteStateReconstruction { expected, supplied } => {
                write!(
                    formatter,
                    "incomplete state tomography reconstruction: \
                     expected={expected}, supplied={supplied}"
                )
            }

            Self::IncompleteProcessReconstruction { expected, supplied } => {
                write!(
                    formatter,
                    "incomplete process tomography reconstruction: \
                     expected={expected}, supplied={supplied}"
                )
            }

            Self::InvalidChoiDimension {
                hilbert_dimension,
                matrix_dimension,
            } => {
                write!(
                    formatter,
                    "invalid Choi dimension: Hilbert dimension={hilbert_dimension}, \
                     matrix dimension={matrix_dimension}"
                )
            }

            Self::NotTracePreserving {
                maximum_deviation,
                tolerance,
            } => {
                write!(
                    formatter,
                    "reconstructed process is not trace preserving: \
                     maximum_deviation={maximum_deviation}, tolerance={tolerance}"
                )
            }

            Self::ArithmeticOverflow => {
                write!(formatter, "tomography integer arithmetic overflow")
            }

            Self::EigenDecompositionFailed => {
                write!(formatter, "tomography eigendecomposition failed to converge")
            }

            Self::ZeroDenominator => {
                write!(formatter, "tomography calculation encountered a zero denominator")
            }
        }
    }
}

impl std::error::Error for TomographyError {}

/// Result type used by tomography.
pub type TomographyResult<T> = Result<T, TomographyError>;

// =============================================================================
// Complex arithmetic
// =============================================================================

/// Dependency-free complex number used by the tomography boundary.
///
/// The existing fidelity module has its own canonical `Complex64` type. This
/// local representation keeps tomography independently testable and avoids
/// coupling reconstruction mathematics to fidelity internals. Conversion is
/// exposed through `to_fidelity_matrix_data`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl std::ops::Sub for Complex64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl std::ops::Mul for Complex64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl std::ops::Mul<f64> for Complex64 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.re * rhs, self.im * rhs)
    }
}

impl std::ops::Div<f64> for Complex64 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.re / rhs, self.im / rhs)
    }
}

impl std::ops::Neg for Complex64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.im)
    }
}

// =============================================================================
// Complex matrix
// =============================================================================

/// Row-major complex matrix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexMatrix {
    rows: usize,
    columns: usize,
    data: Vec<Complex64>,
}

impl ComplexMatrix {
    /// Creates a matrix after validating dimensions and data length.
    pub fn new(
        rows: usize,
        columns: usize,
        data: Vec<Complex64>,
    ) -> TomographyResult<Self> {
        if rows == 0 || columns == 0 {
            return Err(TomographyError::InvalidMatrixDimension { rows, columns });
        }

        let expected = rows
            .checked_mul(columns)
            .ok_or(TomographyError::ArithmeticOverflow)?;

        if expected != data.len() {
            return Err(TomographyError::InvalidMatrixDimension { rows, columns });
        }

        for (index, value) in data.iter().enumerate() {
            if !value.is_finite() {
                return Err(TomographyError::MatrixContainsNonFinite { index });
            }
        }

        Ok(Self {
            rows,
            columns,
            data,
        })
    }

    /// Creates an identity matrix.
    pub fn identity(dimension: usize) -> TomographyResult<Self> {
        if dimension == 0 {
            return Err(TomographyError::InvalidMatrixDimension {
                rows: 0,
                columns: 0,
            });
        }

        let elements = dimension
            .checked_mul(dimension)
            .ok_or(TomographyError::ArithmeticOverflow)?;

        let mut data = vec![Complex64::zero(); elements];

        for index in 0..dimension {
            let position = index
                .checked_mul(dimension)
                .and_then(|offset| offset.checked_add(index))
                .ok_or(TomographyError::ArithmeticOverflow)?;

            data[position] = Complex64::one();
        }

        Self::new(dimension, dimension, data)
    }

    /// Number of rows.
    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Number of columns.
    #[inline]
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Matrix data.
    #[inline]
    pub fn data(&self) -> &[Complex64] {
        &self.data
    }

    /// Mutable matrix data.
    #[inline]
    pub fn data_mut(&mut self) -> &mut [Complex64] {
        &mut self.data
    }

    /// Gets an element.
    pub fn get(&self, row: usize, column: usize) -> TomographyResult<Complex64> {
        if row >= self.rows || column >= self.columns {
            return Err(TomographyError::InvalidMatrixDimension {
                rows: self.rows,
                columns: self.columns,
            });
        }

        let index = row
            .checked_mul(self.columns)
            .and_then(|offset| offset.checked_add(column))
            .ok_or(TomographyError::ArithmeticOverflow)?;

        Ok(self.data[index])
    }

    /// Sets an element.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: Complex64,
    ) -> TomographyResult<()> {
        if row >= self.rows || column >= self.columns {
            return Err(TomographyError::InvalidMatrixDimension {
                rows: self.rows,
                columns: self.columns,
            });
        }

        if !value.is_finite() {
            return Err(TomographyError::NonFiniteValue {
                context: "matrix element",
            });
        }

        let index = row
            .checked_mul(self.columns)
            .and_then(|offset| offset.checked_add(column))
            .ok_or(TomographyError::ArithmeticOverflow)?;

        self.data[index] = value;

        Ok(())
    }

    /// Returns the conjugate transpose.
    pub fn dagger(&self) -> TomographyResult<Self> {
        let elements = self
            .rows
            .checked_mul(self.columns)
            .ok_or(TomographyError::ArithmeticOverflow)?;

        let mut data = vec![Complex64::zero(); elements];

        for row in 0..self.rows {
            for column in 0..self.columns {
                let source = row
                    .checked_mul(self.columns)
                    .and_then(|offset| offset.checked_add(column))
                    .ok_or(TomographyError::ArithmeticOverflow)?;

                let target = column
                    .checked_mul(self.rows)
                    .and_then(|offset| offset.checked_add(row))
                    .ok_or(TomographyError::ArithmeticOverflow)?;

                data[target] = self.data[source].conjugate();
            }
        }

        Self::new(self.columns, self.rows, data)
    }

    /// Matrix multiplication.
    pub fn multiply(&self, rhs: &Self) -> TomographyResult<Self> {
        if self.columns != rhs.rows {
            return Err(TomographyError::MatrixDimensionMismatch {
                left_rows: self.rows,
                left_columns: self.columns,
                right_rows: rhs.rows,
                right_columns: rhs.columns,
            });
        }

        let elements = self
            .rows
            .checked_mul(rhs.columns)
            .ok_or(TomographyError::ArithmeticOverflow)?;

        let mut output = vec![Complex64::zero(); elements];

        for row in 0..self.rows {
            for column in 0..rhs.columns {
                let mut value = Complex64::zero();

                for k in 0..self.columns {
                    value = value + self.get(row, k)? * rhs.get(k, column)?;
                }

                let index = row
                    .checked_mul(rhs.columns)
                    .and_then(|offset| offset.checked_add(column))
                    .ok_or(TomographyError::ArithmeticOverflow)?;

                output[index] = value;
            }
        }

        Self::new(self.rows, rhs.columns, output)
    }

    /// Scales every element.
    pub fn scale(&self, scalar: Complex64) -> TomographyResult<Self> {
        let data = self
            .data
            .iter()
            .copied()
            .map(|value| value * scalar)
            .collect();

        Self::new(self.rows, self.columns, data)
    }

    /// Matrix trace.
    pub fn trace(&self) -> TomographyResult<Complex64> {
        if self.rows != self.columns {
            return Err(TomographyError::MatrixNotSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }

        let mut trace = Complex64::zero();

        for index in 0..self.rows {
            trace = trace + self.get(index, index)?;
        }

        Ok(trace)
    }

    /// Maximum Hermiticity deviation.
    pub fn hermiticity_deviation(&self) -> TomographyResult<f64> {
        if self.rows != self.columns {
            return Err(TomographyError::MatrixNotSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }

        let mut maximum = 0.0_f64;

        for row in 0..self.rows {
            for column in 0..self.columns {
                let lhs = self.get(row, column)?;
                let rhs = self.get(column, row)?.conjugate();

                let deviation = (lhs - rhs).norm();

                if deviation > maximum {
                    maximum = deviation;
                }
            }
        }

        Ok(maximum)
    }

    /// Validates finite values, Hermiticity and trace.
    pub fn validate_density_matrix(
        &self,
        trace_tolerance: f64,
        hermiticity_tolerance: f64,
    ) -> TomographyResult<()> {
        validate_tolerance(trace_tolerance)?;
        validate_tolerance(hermiticity_tolerance)?;

        if self.rows != self.columns {
            return Err(TomographyError::MatrixNotSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }

        let hermiticity = self.hermiticity_deviation()?;

        if hermiticity > hermiticity_tolerance {
            return Err(TomographyError::NotHermitian {
                maximum_deviation: hermiticity,
                tolerance: hermiticity_tolerance,
            });
        }

        let trace = self.trace()?;

        if (trace.re - 1.0).abs() > trace_tolerance || trace.im.abs() > trace_tolerance {
            return Err(TomographyError::InvalidTrace {
                trace,
                tolerance: trace_tolerance,
            });
        }

        Ok(())
    }

    /// Validates positive semidefiniteness using a Hermitian Jacobi solver.
    pub fn validate_positive_semidefinite(
        &self,
        tolerance: f64,
    ) -> TomographyResult<f64> {
        validate_tolerance(tolerance)?;

        if self.rows != self.columns {
            return Err(TomographyError::MatrixNotSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }

        let eigenvalues = hermitian_eigenvalues(self, tolerance)?;

        let minimum = eigenvalues
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);

        if minimum < -tolerance {
            return Err(TomographyError::NotPositiveSemidefinite {
                minimum_eigenvalue: minimum,
                tolerance,
            });
        }

        Ok(minimum)
    }
}

// =============================================================================
// Resource limits
// =============================================================================

/// Resource limits for tomography.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TomographyLimits {
    /// Maximum number of qubits.
    pub max_qubits: usize,

    /// Maximum matrix elements.
    pub max_matrix_elements: usize,

    /// Maximum measurement settings.
    pub max_measurement_settings: usize,

    /// Maximum total shots.
    pub max_total_shots: u64,
}

impl Default for TomographyLimits {
    fn default() -> Self {
        Self {
            max_qubits: DEFAULT_MAX_QUBITS,
            max_matrix_elements: DEFAULT_MAX_MATRIX_ELEMENTS,
            max_measurement_settings: DEFAULT_MAX_MEASUREMENT_SETTINGS,
            max_total_shots: DEFAULT_MAX_TOTAL_SHOTS,
        }
    }
}

impl TomographyLimits {
    /// Validates the limits themselves.
    pub fn validate(&self) -> TomographyResult<()> {
        if self.max_qubits == 0 {
            return Err(TomographyError::InvalidQubitCount { qubits: 0 });
        }

        if self.max_matrix_elements == 0 {
            return Err(TomographyError::ResourceLimitExceeded {
                resource: "matrix_elements",
                requested: 1,
                maximum: 0,
            });
        }

        if self.max_measurement_settings == 0 {
            return Err(TomographyError::ResourceLimitExceeded {
                resource: "measurement_settings",
                requested: 1,
                maximum: 0,
            });
        }

        if self.max_total_shots == 0 {
            return Err(TomographyError::ResourceLimitExceeded {
                resource: "total_shots",
                requested: 1,
                maximum: 0,
            });
        }

        Ok(())
    }

    /// Checks whether a qubit count can be reconstructed.
    pub fn check_qubits(&self, qubits: usize) -> TomographyResult<()> {
        if qubits == 0 {
            return Err(TomographyError::InvalidQubitCount { qubits });
        }

        if qubits > self.max_qubits {
            return Err(TomographyError::ResourceLimitExceeded {
                resource: "qubits",
                requested: qubits as u128,
                maximum: self.max_qubits as u128,
            });
        }

        Ok(())
    }

    /// Checks the matrix size for a qubit system.
    pub fn check_matrix_for_qubits(&self, qubits: usize) -> TomographyResult<usize> {
        self.check_qubits(qubits)?;

        let dimension = checked_pow_usize(2, qubits)?;

        let elements = dimension
            .checked_mul(dimension)
            .ok_or(TomographyError::ArithmeticOverflow)?;

        if elements > self.max_matrix_elements {
            return Err(TomographyError::ResourceLimitExceeded {
                resource: "matrix_elements",
                requested: elements as u128,
                maximum: self.max_matrix_elements as u128,
            });
        }

        Ok(dimension)
    }
}

// =============================================================================
// Pauli algebra
// =============================================================================

/// Single-qubit Pauli operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pauli {
    /// Identity.
    I,

    /// Pauli X.
    X,

    /// Pauli Y.
    Y,

    /// Pauli Z.
    Z,
}

impl Pauli {
    /// Stable identifier.
    pub const fn id(self) -> char {
        match self {
            Self::I => 'I',
            Self::X => 'X',
            Self::Y => 'Y',
            Self::Z => 'Z',
        }
    }

    /// Number of non-identity factors.
    pub const fn weight(self) -> usize {
        match self {
            Self::I => 0,
            Self::X | Self::Y | Self::Z => 1,
        }
    }

    /// Returns the 2x2 matrix.
    pub fn matrix(self) -> ComplexMatrix {
        match self {
            Self::I => ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::new(1.0, 0.0),
                    Complex64::zero(),
                    Complex64::zero(),
                    Complex64::new(1.0, 0.0),
                ],
            )
            .expect("static Pauli matrix dimensions are valid"),

            Self::X => ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::zero(),
                    Complex64::one(),
                    Complex64::one(),
                    Complex64::zero(),
                ],
            )
            .expect("static Pauli matrix dimensions are valid"),

            Self::Y => ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::zero(),
                    Complex64::new(0.0, -1.0),
                    Complex64::new(0.0, 1.0),
                    Complex64::zero(),
                ],
            )
            .expect("static Pauli matrix dimensions are valid"),

            Self::Z => ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::one(),
                    Complex64::zero(),
                    Complex64::zero(),
                    Complex64::new(-1.0, 0.0),
                ],
            )
            .expect("static Pauli matrix dimensions are valid"),
        }
    }
}

/// Tensor-product Pauli string.
///
/// The left-most character corresponds to the most-significant tensor factor
/// and the right-most character to the least-significant factor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PauliString {
    factors: Vec<Pauli>,
}

impl PauliString {
    /// Creates a Pauli string.
    pub fn new(factors: Vec<Pauli>) -> TomographyResult<Self> {
        if factors.is_empty() {
            return Err(TomographyError::InvalidQubitCount { qubits: 0 });
        }

        Ok(Self { factors })
    }

    /// Creates an identity string of `qubits` factors.
    pub fn identity(qubits: usize) -> TomographyResult<Self> {
        if qubits == 0 {
            return Err(TomographyError::InvalidQubitCount { qubits: 0 });
        }

        Ok(Self {
            factors: vec![Pauli::I; qubits],
        })
    }

    /// Number of qubits.
    pub fn qubits(&self) -> usize {
        self.factors.len()
    }

    /// Factors.
    pub fn factors(&self) -> &[Pauli] {
        &self.factors
    }

    /// Number of non-identity factors.
    pub fn weight(&self) -> usize {
        self.factors.iter().map(|factor| factor.weight()).sum()
    }

    /// Stable identifier such as `XIZY`.
    pub fn id(&self) -> String {
        self.factors.iter().map(|factor| factor.id()).collect()
    }

    /// Tensor-product matrix.
    pub fn matrix(&self) -> TomographyResult<ComplexMatrix> {
        let mut result = ComplexMatrix::identity(1)?;

        for factor in &self.factors {
            result = kron(&result, &factor.matrix())?;
        }

        Ok(result)
    }
}

/// Generates all n-qubit Pauli strings in deterministic lexicographic order:
/// I, X, Y, Z for each tensor factor.
pub fn generate_pauli_basis(
    qubits: usize,
    limits: &TomographyLimits,
) -> TomographyResult<Vec<PauliString>> {
    limits.check_qubits(qubits)?;

    let count = checked_pow_usize(4, qubits)?;

    if count > limits.max_measurement_settings {
        return Err(TomographyError::ResourceLimitExceeded {
            resource: "measurement_settings",
            requested: count as u128,
            maximum: limits.max_measurement_settings as u128,
        });
    }

    let mut basis = Vec::with_capacity(count);

    generate_pauli_recursive(qubits, &mut Vec::with_capacity(qubits), &mut basis);

    Ok(basis)
}

// =============================================================================
// Measurement observations
// =============================================================================

/// Raw +/-1 Pauli measurement counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PauliMeasurement {
    /// Number of +1 outcomes.
    pub positive: u64,

    /// Number of -1 outcomes.
    pub negative: u64,
}

impl PauliMeasurement {
    /// Creates a validated measurement.
    pub fn new(positive: u64, negative: u64) -> TomographyResult<Self> {
        let shots = positive
            .checked_add(negative)
            .ok_or(TomographyError::ArithmeticOverflow)?;

        if shots == 0 {
            return Err(TomographyError::ZeroShots);
        }

        Ok(Self {
            positive,
            negative,
        })
    }

    /// Total shots.
    pub fn shots(&self) -> u64 {
        self.positive + self.negative
    }

    /// Empirical expectation value for the measured Pauli observable.
    pub fn expectation(&self) -> TomographyResult<f64> {
        let shots = self.shots();

        if shots == 0 {
            return Err(TomographyError::ZeroShots);
        }

        let positive = self.positive as f64;
        let negative = self.negative as f64;
        let denominator = shots as f64;

        let expectation = (positive - negative) / denominator;

        validate_expectation(expectation, DEFAULT_TOMOGRAPHY_TOLERANCE)?;

        Ok(expectation)
    }

    /// Binomial variance estimate for the expectation value.
    pub fn expectation_variance(&self) -> TomographyResult<f64> {
        let shots = self.shots();

        if shots == 0 {
            return Err(TomographyError::ZeroShots);
        }

        let expectation = self.expectation()?;
        let variance = (1.0 - expectation * expectation) / shots as f64;

        if !variance.is_finite() || variance < 0.0 {
            return Err(TomographyError::NonFiniteValue {
                context: "Pauli expectation variance",
            });
        }

        Ok(variance)
    }
}

/// Generic tomography observation.
///
/// This type is intentionally independent of execution backends. A backend
/// adapter converts its raw measurement response into this representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TomographyObservation {
    /// Measurement setting identifier.
    pub setting: PauliString,

    /// Raw +/-1 counts.
    pub measurement: PauliMeasurement,

    /// Optional externally supplied standard uncertainty.
    pub standard_uncertainty: Option<f64>,
}

impl TomographyObservation {
    /// Creates an observation.
    pub fn new(
        setting: PauliString,
        measurement: PauliMeasurement,
    ) -> TomographyResult<Self> {
        validate_pauli_setting(&setting)?;

        Ok(Self {
            setting,
            measurement,
            standard_uncertainty: None,
        })
    }

    /// Creates an observation with explicit uncertainty.
    pub fn with_uncertainty(
        setting: PauliString,
        measurement: PauliMeasurement,
        standard_uncertainty: f64,
    ) -> TomographyResult<Self> {
        validate_pauli_setting(&setting)?;

        if !standard_uncertainty.is_finite() || standard_uncertainty < 0.0 {
            return Err(TomographyError::NonFiniteValue {
                context: "tomography standard uncertainty",
            });
        }

        Ok(Self {
            setting,
            measurement,
            standard_uncertainty: Some(standard_uncertainty),
        })
    }

    /// Returns the measured expectation.
    pub fn expectation(&self) -> TomographyResult<f64> {
        self.measurement.expectation()
    }

    /// Returns the effective uncertainty.
    pub fn uncertainty(&self) -> TomographyResult<f64> {
        if let Some(value) = self.standard_uncertainty {
            return Ok(value);
        }

        Ok(self.measurement.expectation_variance()?.sqrt())
    }
}

// =============================================================================
// State tomography configuration/result
// =============================================================================

/// State tomography configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateTomographyConfig {
    /// Number of qubits.
    pub qubits: usize,

    /// Numerical tolerance.
    pub tolerance: f64,

    /// Whether positivity must be checked.
    pub validate_positivity: bool,

    /// Resource limits.
    pub limits: TomographyLimits,
}

impl Default for StateTomographyConfig {
    fn default() -> Self {
        Self {
            qubits: 1,
            tolerance: DEFAULT_TOMOGRAPHY_TOLERANCE,
            validate_positivity: true,
            limits: TomographyLimits::default(),
        }
    }
}

impl StateTomographyConfig {
    /// Validates the configuration.
    pub fn validate(&self) -> TomographyResult<()> {
        self.limits.validate()?;
        self.limits.check_matrix_for_qubits(self.qubits)?;
        validate_tolerance(self.tolerance)
    }
}

/// Reconstructed quantum state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateTomographyResult {
    /// Benchmark identifier.
    pub benchmark_id: String,

    /// Protocol version.
    pub protocol_version: String,

    /// Number of qubits.
    pub qubits: usize,

    /// Hilbert-space dimension.
    pub dimension: usize,

    /// Reconstructed density matrix.
    pub density_matrix: ComplexMatrix,

    /// Number of measurement settings.
    pub measurement_settings: usize,

    /// Total shots.
    pub total_shots: u64,

    /// Estimated trace.
    pub trace: Complex64,

    /// Minimum eigenvalue when positivity was checked.
    pub minimum_eigenvalue: Option<f64>,

    /// Reconstruction estimator.
    pub estimator: TomographyEstimator,

    /// Warning strings that do not invalidate the result.
    pub warnings: Vec<String>,
}

/// Reconstruction estimator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TomographyEstimator {
    /// Linear inversion in the Pauli basis.
    LinearInversion,
}

impl TomographyEstimator {
    /// Stable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::LinearInversion => "linear_inversion",
        }
    }
}

// =============================================================================
// State tomography
// =============================================================================

/// Reconstructs an n-qubit state from a complete Pauli expectation set.
///
/// The identity Pauli coefficient is always exactly one. The caller must
/// provide every non-identity Pauli coefficient.
///
/// The observations may contain the identity setting, but it is validated
/// against the required expectation of one.
pub fn reconstruct_state(
    config: &StateTomographyConfig,
    observations: &[TomographyObservation],
) -> TomographyResult<StateTomographyResult> {
    config.validate()?;

    if observations.is_empty() {
        return Err(TomographyError::EmptyMeasurementSet);
    }

    let basis = generate_pauli_basis(config.qubits, &config.limits)?;

    let expected_non_identity = basis.len().saturating_sub(1);

    let mut coefficients = vec![None::<f64>; basis.len()];
    let mut uncertainties = vec![None::<f64>; basis.len()];

    let identity = PauliString::identity(config.qubits)?;

    coefficients[0] = Some(1.0);

    for observation in observations {
        if observation.setting.qubits() != config.qubits {
            return Err(TomographyError::MeasurementDimensionMismatch);
        }

        let index = basis
            .iter()
            .position(|candidate| candidate == &observation.setting)
            .ok_or(TomographyError::MeasurementDimensionMismatch)?;

        if coefficients[index].is_some() && index != 0 {
            return Err(TomographyError::DuplicateMeasurementSetting);
        }

        let expectation = observation.expectation()?;
        let uncertainty = observation.uncertainty()?;

        if index == 0 {
            if (expectation - 1.0).abs() > config.tolerance {
                return Err(TomographyError::InvalidExpectation {
                    value: expectation,
                    tolerance: config.tolerance,
                });
            }
        } else {
            coefficients[index] = Some(expectation);
            uncertainties[index] = Some(uncertainty);
        }
    }

    let supplied_non_identity = coefficients
        .iter()
        .skip(1)
        .filter(|value| value.is_some())
        .count();

    if supplied_non_identity != expected_non_identity {
        return Err(TomographyError::IncompleteStateReconstruction {
            expected: expected_non_identity,
            supplied: supplied_non_identity,
        });
    }

    let dimension = config.limits.check_matrix_for_qubits(config.qubits)?;

    let mut matrix = zero_matrix(dimension)?;

    for (index, pauli) in basis.iter().enumerate() {
        let coefficient = coefficients[index]
            .ok_or(TomographyError::IncompleteStateReconstruction {
                expected: expected_non_identity,
                supplied: supplied_non_identity,
            })?;

        let pauli_matrix = pauli.matrix()?;

        matrix = matrix.add(&pauli_matrix.scale(Complex64::new(
            coefficient / dimension as f64,
            0.0,
        )))?;
    }

    matrix.validate_density_matrix(config.tolerance, config.tolerance)?;

    let minimum_eigenvalue = if config.validate_positivity {
        Some(matrix.validate_positive_semidefinite(config.tolerance)?)
    } else {
        None
    };

    let trace = matrix.trace()?;

    let total_shots = checked_total_shots(observations)?;

    let mut warnings = Vec::new();

    if !config.validate_positivity {
        warnings.push(
            "positive-semidefinite validation was explicitly disabled; \
             the reconstructed state must not be treated as physically validated"
                .to_string(),
        );
    }

    let _ = identity;

    Ok(StateTomographyResult {
        benchmark_id: TOMOGRAPHY_BENCHMARK_ID.to_string(),
        protocol_version: TOMOGRAPHY_PROTOCOL_VERSION.to_string(),
        qubits: config.qubits,
        dimension,
        density_matrix: matrix,
        measurement_settings: observations.len(),
        total_shots,
        trace,
        minimum_eigenvalue,
        estimator: TomographyEstimator::LinearInversion,
        warnings,
    })
}

// =============================================================================
// Process tomography
// =============================================================================

/// Process tomography configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessTomographyConfig {
    /// Number of input qubits.
    pub qubits: usize,

    /// Numerical tolerance.
    pub tolerance: f64,

    /// Validate Choi positivity.
    pub validate_positivity: bool,

    /// Validate trace preservation.
    pub validate_trace_preserving: bool,

    /// Resource limits.
    pub limits: TomographyLimits,
}

impl Default for ProcessTomographyConfig {
    fn default() -> Self {
        Self {
            qubits: 1,
            tolerance: DEFAULT_TOMOGRAPHY_TOLERANCE,
            validate_positivity: true,
            validate_trace_preserving: true,
            limits: TomographyLimits::default(),
        }
    }
}

impl ProcessTomographyConfig {
    /// Validates the configuration.
    pub fn validate(&self) -> TomographyResult<()> {
        self.limits.validate()?;
        self.limits.check_matrix_for_qubits(self.qubits)?;

        let doubled = self
            .qubits
            .checked_mul(2)
            .ok_or(TomographyError::ArithmeticOverflow)?;

        self.limits.check_matrix_for_qubits(doubled)?;

        validate_tolerance(self.tolerance)
    }
}

/// Reconstructed quantum process in normalized Choi-state form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessTomographyResult {
    /// Benchmark identifier.
    pub benchmark_id: String,

    /// Protocol version.
    pub protocol_version: String,

    /// Number of input qubits.
    pub qubits: usize,

    /// Input Hilbert dimension.
    pub input_dimension: usize,

    /// Choi-state dimension.
    pub choi_dimension: usize,

    /// Normalized Choi density matrix.
    pub normalized_choi: ComplexMatrix,

    /// Number of measurement settings.
    pub measurement_settings: usize,

    /// Total shots.
    pub total_shots: u64,

    /// Choi trace.
    pub trace: Complex64,

    /// Minimum Choi eigenvalue.
    pub minimum_eigenvalue: Option<f64>,

    /// Maximum trace-preservation deviation.
    pub trace_preservation_deviation: Option<f64>,

    /// Reconstruction estimator.
    pub estimator: TomographyEstimator,

    /// Warnings.
    pub warnings: Vec<String>,
}

/// Reconstructs a normalized Choi process representation.
///
/// The supplied Pauli transfer coefficients are interpreted as
///
/// ```text
/// R_ab = Tr(P_a E(P_b)) / d
/// ```
///
/// with Pauli operators normalized such that
///
/// ```text
/// Tr(P_a P_b) = d delta_ab.
/// ```
///
/// The conversion from the Pauli transfer matrix to the normalized Choi
/// matrix is performed using the Pauli expansion of the Choi operator.
///
/// This function requires the complete d^2 x d^2 Pauli transfer data,
/// including the identity input/output coefficient.
pub fn reconstruct_process(
    config: &ProcessTomographyConfig,
    observations: &[ProcessPauliObservation],
) -> TomographyResult<ProcessTomographyResult> {
    config.validate()?;

    if observations.is_empty() {
        return Err(TomographyError::EmptyMeasurementSet);
    }

    let pauli_basis = generate_pauli_basis(config.qubits, &config.limits)?;

    let basis_size = pauli_basis.len();

    let expected = basis_size
        .checked_mul(basis_size)
        .ok_or(TomographyError::ArithmeticOverflow)?;

    if expected > config.limits.max_measurement_settings {
        return Err(TomographyError::ResourceLimitExceeded {
            resource: "process_measurement_settings",
            requested: expected as u128,
            maximum: config.limits.max_measurement_settings as u128,
        });
    }

    let mut coefficients = vec![None::<f64>; expected];

    for observation in observations {
        if observation.input.qubits() != config.qubits
            || observation.output.qubits() != config.qubits
        {
            return Err(TomographyError::MeasurementDimensionMismatch);
        }

        let input_index = pauli_basis
            .iter()
            .position(|candidate| candidate == &observation.input)
            .ok_or(TomographyError::MeasurementDimensionMismatch)?;

        let output_index = pauli_basis
            .iter()
            .position(|candidate| candidate == &observation.output)
            .ok_or(TomographyError::MeasurementDimensionMismatch)?;

        let offset = output_index
            .checked_mul(basis_size)
            .and_then(|value| value.checked_add(input_index))
            .ok_or(TomographyError::ArithmeticOverflow)?;

        if coefficients[offset].is_some() {
            return Err(TomographyError::DuplicateMeasurementSetting);
        }

        let expectation = observation.expectation()?;

        coefficients[offset] = Some(expectation);
    }

    let supplied = coefficients.iter().filter(|value| value.is_some()).count();

    if supplied != expected {
        return Err(TomographyError::IncompleteProcessReconstruction {
            expected,
            supplied,
        });
    }

    let dimension = checked_pow_usize(2, config.qubits)?;

    let choi_dimension = dimension
        .checked_mul(dimension)
        .ok_or(TomographyError::ArithmeticOverflow)?;

    let mut normalized_choi = zero_matrix(choi_dimension)?;

    // J_norm = 1 / d^2 sum_ab R_ab (P_b^T ⊗ P_a)
    //
    // For Pauli operators:
    //
    // X^T = X
    // Z^T = Z
    // Y^T = -Y
    //
    // The explicit transpose is therefore required.
    let scale = 1.0 / (dimension as f64 * dimension as f64);

    for output_index in 0..basis_size {
        for input_index in 0..basis_size {
            let coefficient_index = output_index
                .checked_mul(basis_size)
                .and_then(|value| value.checked_add(input_index))
                .ok_or(TomographyError::ArithmeticOverflow)?;

            let coefficient = coefficients[coefficient_index]
                .ok_or(TomographyError::IncompleteProcessReconstruction {
                    expected,
                    supplied,
                })?;

            let output_pauli = pauli_basis[output_index].matrix()?;
            let input_pauli = pauli_basis[input_index].matrix()?;
            let input_transpose = transpose(&input_pauli)?;

            let term = kron(&input_transpose, &output_pauli)?
                .scale(Complex64::new(coefficient * scale, 0.0));

            normalized_choi = normalized_choi.add(&term)?;
        }
    }

    normalized_choi.validate_density_matrix(config.tolerance, config.tolerance)?;

    let minimum_eigenvalue = if config.validate_positivity {
        Some(
            normalized_choi
                .validate_positive_semidefinite(config.tolerance)?,
        )
    } else {
        None
    };

    let trace_preservation_deviation = if config.validate_trace_preserving {
        Some(validate_trace_preserving(
            &normalized_choi,
            dimension,
            config.tolerance,
        )?)
    } else {
        None
    };

    let total_shots = checked_process_total_shots(observations)?;

    let mut warnings = Vec::new();

    if !config.validate_positivity {
        warnings.push(
            "positive-semidefinite validation was explicitly disabled; \
             the reconstructed process must not be treated as physically validated"
                .to_string(),
        );
    }

    if !config.validate_trace_preserving {
        warnings.push(
            "trace-preservation validation was explicitly disabled; \
             the reconstructed process must not be assumed to be a channel"
                .to_string(),
        );
    }

    Ok(ProcessTomographyResult {
        benchmark_id: TOMOGRAPHY_BENCHMARK_ID.to_string(),
        protocol_version: TOMOGRAPHY_PROTOCOL_VERSION.to_string(),
        qubits: config.qubits,
        input_dimension: dimension,
        choi_dimension,
        normalized_choi,
        measurement_settings: observations.len(),
        total_shots,
        trace: normalized_choi_trace(&observations, &pauli_basis)?,
        minimum_eigenvalue,
        trace_preservation_deviation,
        estimator: TomographyEstimator::LinearInversion,
        warnings,
    })
}

// =============================================================================
// Process observations
// =============================================================================

/// One Pauli-transfer measurement.
///
/// `input` identifies the Pauli operator supplied to the channel.
///
/// `output` identifies the Pauli observable measured after the channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessPauliObservation {
    /// Input Pauli operator.
    pub input: PauliString,

    /// Output Pauli observable.
    pub output: PauliString,

    /// Measurement counts for the output observable.
    pub measurement: PauliMeasurement,

    /// Optional external uncertainty.
    pub standard_uncertainty: Option<f64>,
}

impl ProcessPauliObservation {
    /// Creates an observation.
    pub fn new(
        input: PauliString,
        output: PauliString,
        measurement: PauliMeasurement,
    ) -> TomographyResult<Self> {
        validate_pauli_setting(&input)?;
        validate_pauli_setting(&output)?;

        Ok(Self {
            input,
            output,
            measurement,
            standard_uncertainty: None,
        })
    }

    /// Returns the measured transfer coefficient.
    pub fn expectation(&self) -> TomographyResult<f64> {
        self.measurement.expectation()
    }

    /// Returns uncertainty.
    pub fn uncertainty(&self) -> TomographyResult<f64> {
        if let Some(value) = self.standard_uncertainty {
            if !value.is_finite() || value < 0.0 {
                return Err(TomographyError::NonFiniteValue {
                    context: "process tomography uncertainty",
                });
            }

            return Ok(value);
        }

        Ok(self.measurement.expectation_variance()?.sqrt())
    }
}

// =============================================================================
// Basis and matrix utilities
// =============================================================================

fn generate_pauli_recursive(
    remaining: usize,
    current: &mut Vec<Pauli>,
    output: &mut Vec<PauliString>,
) {
    if remaining == 0 {
        output.push(
            PauliString::new(current.clone())
                .expect("recursive Pauli generation always supplies factors"),
        );

        return;
    }

    for pauli in [Pauli::I, Pauli::X, Pauli::Y, Pauli::Z] {
        current.push(pauli);
        generate_pauli_recursive(remaining - 1, current, output);
        current.pop();
    }
}

fn validate_pauli_setting(setting: &PauliString) -> TomographyResult<()> {
    if setting.qubits() == 0 {
        return Err(TomographyError::InvalidQubitCount { qubits: 0 });
    }

    Ok(())
}

fn checked_pow_usize(base: usize, exponent: usize) -> TomographyResult<usize> {
    let mut result = 1usize;

    for _ in 0..exponent {
        result = result
            .checked_mul(base)
            .ok_or(TomographyError::ArithmeticOverflow)?;
    }

    Ok(result)
}

fn zero_matrix(dimension: usize) -> TomographyResult<ComplexMatrix> {
    let elements = dimension
        .checked_mul(dimension)
        .ok_or(TomographyError::ArithmeticOverflow)?;

    ComplexMatrix::new(
        dimension,
        dimension,
        vec![Complex64::zero(); elements],
    )
}

fn transpose(matrix: &ComplexMatrix) -> TomographyResult<ComplexMatrix> {
    let mut output = ComplexMatrix::new(
        matrix.columns(),
        matrix.rows(),
        vec![
            Complex64::zero();
            matrix
                .rows()
                .checked_mul(matrix.columns())
                .ok_or(TomographyError::ArithmeticOverflow)?
        ],
    )?;

    for row in 0..matrix.rows() {
        for column in 0..matrix.columns() {
            output.set(column, row, matrix.get(row, column)?)?;
        }
    }

    Ok(output)
}

fn kron(lhs: &ComplexMatrix, rhs: &ComplexMatrix) -> TomographyResult<ComplexMatrix> {
    let rows = lhs
        .rows()
        .checked_mul(rhs.rows())
        .ok_or(TomographyError::ArithmeticOverflow)?;

    let columns = lhs
        .columns()
        .checked_mul(rhs.columns())
        .ok_or(TomographyError::ArithmeticOverflow)?;

    let elements = rows
        .checked_mul(columns)
        .ok_or(TomographyError::ArithmeticOverflow)?;

    let mut data = vec![Complex64::zero(); elements];

    for lhs_row in 0..lhs.rows() {
        for lhs_column in 0..lhs.columns() {
            let lhs_value = lhs.get(lhs_row, lhs_column)?;

            for rhs_row in 0..rhs.rows() {
                for rhs_column in 0..rhs.columns() {
                    let row = lhs_row
                        .checked_mul(rhs.rows())
                        .and_then(|offset| offset.checked_add(rhs_row))
                        .ok_or(TomographyError::ArithmeticOverflow)?;

                    let column = lhs_column
                        .checked_mul(rhs.columns())
                        .and_then(|offset| offset.checked_add(rhs_column))
                        .ok_or(TomographyError::ArithmeticOverflow)?;

                    let index = row
                        .checked_mul(columns)
                        .and_then(|offset| offset.checked_add(column))
                        .ok_or(TomographyError::ArithmeticOverflow)?;

                    data[index] = lhs_value * rhs.get(rhs_row, rhs_column)?;
                }
            }
        }
    }

    ComplexMatrix::new(rows, columns, data)
}

fn matrix_add(
    lhs: &ComplexMatrix,
    rhs: &ComplexMatrix,
) -> TomographyResult<ComplexMatrix> {
    if lhs.rows() != rhs.rows() || lhs.columns() != rhs.columns() {
        return Err(TomographyError::MatrixDimensionMismatch {
            left_rows: lhs.rows(),
            left_columns: lhs.columns(),
            right_rows: rhs.rows(),
            right_columns: rhs.columns(),
        });
    }

    let data = lhs
        .data()
        .iter()
        .zip(rhs.data().iter())
        .map(|(left, right)| *left + *right)
        .collect();

    ComplexMatrix::new(lhs.rows(), lhs.columns(), data)
}

trait MatrixAddition {
    fn add(&self, rhs: &Self) -> TomographyResult<Self>
    where
        Self: Sized;
}

impl MatrixAddition for ComplexMatrix {
    fn add(&self, rhs: &Self) -> TomographyResult<Self> {
        matrix_add(self, rhs)
    }
}

// =============================================================================
// Trace preservation
// =============================================================================

/// Validates the trace-preserving condition of a normalized Choi state.
///
/// The normalized Choi convention used here requires
///
/// ```text
/// Tr_output(J_norm) = I_input / d
/// ```
///
/// where `d` is the input Hilbert dimension.
pub fn trace_preservation_deviation(
    normalized_choi: &ComplexMatrix,
    input_dimension: usize,
) -> TomographyResult<f64> {
    if input_dimension == 0 {
        return Err(TomographyError::InvalidQubitCount { qubits: 0 });
    }

    let expected_dimension = input_dimension
        .checked_mul(input_dimension)
        .ok_or(TomographyError::ArithmeticOverflow)?;

    if normalized_choi.rows() != expected_dimension
        || normalized_choi.columns() != expected_dimension
    {
        return Err(TomographyError::InvalidChoiDimension {
            hilbert_dimension: input_dimension,
            matrix_dimension: normalized_choi.rows(),
        });
    }

    let mut maximum = 0.0_f64;

    // Indexing convention:
    //
    // J[(i,o),(j,p)]
    //
    // where i,j are input indices and o,p are output indices.
    //
    // Partial trace over output:
    //
    // S[i,j] = sum_o J[(i,o),(j,o)]
    //
    // Expected S = I/d.
    let target = 1.0 / input_dimension as f64;

    for i in 0..input_dimension {
        for j in 0..input_dimension {
            let mut value = Complex64::zero();

            for output in 0..input_dimension {
                let row = i
                    .checked_mul(input_dimension)
                    .and_then(|offset| offset.checked_add(output))
                    .ok_or(TomographyError::ArithmeticOverflow)?;

                let column = j
                    .checked_mul(input_dimension)
                    .and_then(|offset| offset.checked_add(output))
                    .ok_or(TomographyError::ArithmeticOverflow)?;

                value = value + normalized_choi.get(row, column)?;
            }

            let expected_value = if i == j {
                Complex64::new(target, 0.0)
            } else {
                Complex64::zero()
            };

            let deviation = (value - expected_value).norm();

            if deviation > maximum {
                maximum = deviation;
            }
        }
    }

    Ok(maximum)
}

fn validate_trace_preserving(
    normalized_choi: &ComplexMatrix,
    input_dimension: usize,
    tolerance: f64,
) -> TomographyResult<f64> {
    let deviation = trace_preservation_deviation(normalized_choi, input_dimension)?;

    if deviation > tolerance {
        return Err(TomographyError::NotTracePreserving {
            maximum_deviation: deviation,
            tolerance,
        });
    }

    Ok(deviation)
}

// =============================================================================
// Eigenvalue validation
// =============================================================================

/// Computes the real eigenvalues of a Hermitian matrix using a Jacobi
/// eigensolver.
///
/// This implementation is intentionally dependency-free. It is appropriate
/// for diagnostic tomography matrices subject to the resource limits in this
/// module, not as a claim that large-scale tomography should be computationally
/// cheap.
pub fn hermitian_eigenvalues(
    matrix: &ComplexMatrix,
    tolerance: f64,
) -> TomographyResult<Vec<f64>> {
    validate_tolerance(tolerance)?;

    if matrix.rows() != matrix.columns() {
        return Err(TomographyError::MatrixNotSquare {
            rows: matrix.rows(),
            columns: matrix.columns(),
        });
    }

    let n = matrix.rows();

    if n == 0 {
        return Err(TomographyError::InvalidMatrixDimension {
            rows: 0,
            columns: 0,
        });
    }

    // Jacobi rotations are most naturally implemented on a real symmetric
    // matrix. For a Hermitian matrix we embed H into a real 2n x 2n matrix:
    //
    // [ Re(H)  -Im(H) ]
    // [ Im(H)   Re(H) ]
    //
    // The eigenvalues of H occur twice in this real representation.
    let doubled = n
        .checked_mul(2)
        .ok_or(TomographyError::ArithmeticOverflow)?;

    let elements = doubled
        .checked_mul(doubled)
        .ok_or(TomographyError::ArithmeticOverflow)?;

    let mut real = vec![0.0_f64; elements];

    let index = |row: usize, column: usize| -> TomographyResult<usize> {
        row.checked_mul(doubled)
            .and_then(|offset| offset.checked_add(column))
            .ok_or(TomographyError::ArithmeticOverflow)
    };

    for row in 0..n {
        for column in 0..n {
            let value = matrix.get(row, column)?;

            let rr = index(row, column)?;
            let ri = index(row, column + n)?;
            let ir = index(row + n, column)?;
            let ii = index(row + n, column + n)?;

            real[rr] = value.re;
            real[ri] = -value.im;
            real[ir] = value.im;
            real[ii] = value.re;
        }
    }

    let mut iterations = 0usize;
    let max_iterations = doubled
        .checked_mul(doubled)
        .and_then(|value| value.checked_mul(100))
        .ok_or(TomographyError::ArithmeticOverflow)?;

    loop {
        iterations = iterations
            .checked_add(1)
            .ok_or(TomographyError::ArithmeticOverflow)?;

        if iterations > max_iterations {
            return Err(TomographyError::EigenDecompositionFailed);
        }

        let mut p = 0usize;
        let mut q = 0usize;
        let mut maximum = 0.0_f64;

        for row in 0..doubled {
            for column in (row + 1)..doubled {
                let value = real[index(row, column)?].abs();

                if value > maximum {
                    maximum = value;
                    p = row;
                    q = column;
                }
            }
        }

        if maximum <= tolerance {
            break;
        }

        let app = real[index(p, p)?];
        let aqq = real[index(q, q)?];
        let apq = real[index(p, q)?];

        if apq.abs() <= tolerance {
            continue;
        }

        let tau = (aqq - app) / (2.0 * apq);

        let t = if tau >= 0.0 {
            1.0 / (tau + (1.0 + tau * tau).sqrt())
        } else {
            -1.0 / (-tau + (1.0 + tau * tau).sqrt())
        };

        let cosine = 1.0 / (1.0 + t * t).sqrt();
        let sine = t * cosine;

        for k in 0..doubled {
            if k == p || k == q {
                continue;
            }

            let akp = real[index(k, p)?];
            let akq = real[index(k, q)?];

            let new_kp = cosine * akp - sine * akq;
            let new_kq = sine * akp + cosine * akq;

            real[index(k, p)?] = new_kp;
            real[index(p, k)?] = new_kp;

            real[index(k, q)?] = new_kq;
            real[index(q, k)?] = new_kq;
        }

        let new_app = cosine * cosine * app
            - 2.0 * sine * cosine * apq
            + sine * sine * aqq;

        let new_aqq = sine * sine * app
            + 2.0 * sine * cosine * apq
            + cosine * cosine * aqq;

        real[index(p, p)?] = new_app;
        real[index(q, q)?] = new_aqq;
        real[index(p, q)?] = 0.0;
        real[index(q, p)?] = 0.0;
    }

    let mut values = Vec::with_capacity(n);

    for index_value in 0..n {
        values.push(real[index(index_value, index_value)?]);
    }

    values.sort_by(|lhs, rhs| {
        lhs.partial_cmp(rhs)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(values)
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_tolerance(value: f64) -> TomographyResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(TomographyError::InvalidTolerance { value });
    }

    Ok(())
}

fn validate_expectation(value: f64, tolerance: f64) -> TomographyResult<()> {
    validate_tolerance(tolerance)?;

    if !value.is_finite() {
        return Err(TomographyError::NonFiniteValue {
            context: "Pauli expectation",
        });
    }

    if value < -1.0 - tolerance || value > 1.0 + tolerance {
        return Err(TomographyError::InvalidExpectation { value, tolerance });
    }

    Ok(())
}

fn checked_total_shots(
    observations: &[TomographyObservation],
) -> TomographyResult<u64> {
    let mut total = 0u64;

    for observation in observations {
        total = total
            .checked_add(observation.measurement.shots())
            .ok_or(TomographyError::ArithmeticOverflow)?;
    }

    Ok(total)
}

fn checked_process_total_shots(
    observations: &[ProcessPauliObservation],
) -> TomographyResult<u64> {
    let mut total = 0u64;

    for observation in observations {
        total = total
            .checked_add(observation.measurement.shots())
            .ok_or(TomographyError::ArithmeticOverflow)?;
    }

    Ok(total)
}

fn normalized_choi_trace(
    observations: &[ProcessPauliObservation],
    basis: &[PauliString],
) -> TomographyResult<Complex64> {
    let basis_size = basis.len();

    let identity = PauliString::identity(
        basis
            .first()
            .map(PauliString::qubits)
            .ok_or(TomographyError::EmptyMeasurementSet)?,
    )?;

    let mut identity_input = None;
    let mut identity_output = None;

    for (index, pauli) in basis.iter().enumerate() {
        if pauli == &identity {
            identity_input = Some(index);
            identity_output = Some(index);
            break;
        }
    }

    let input = identity_input.ok_or(TomographyError::MeasurementDimensionMismatch)?;
    let output = identity_output.ok_or(TomographyError::MeasurementDimensionMismatch)?;

    for observation in observations {
        if observation.input == identity && observation.output == identity {
            let value = observation.expectation()?;

            if (value - 1.0).abs() > DEFAULT_TOMOGRAPHY_TOLERANCE {
                return Err(TomographyError::InvalidExpectation {
                    value,
                    tolerance: DEFAULT_TOMOGRAPHY_TOLERANCE,
                });
            }

            let _ = basis_size;
            let _ = input;
            let _ = output;

            return Ok(Complex64::one());
        }
    }

    // A complete process tomography set always contains I -> I.
    Err(TomographyError::IncompleteProcessReconstruction {
        expected: basis_size * basis_size,
        supplied: observations.len(),
    })
}

// =============================================================================
// Public validation API
// =============================================================================

/// Validates an externally reconstructed density matrix.
pub fn validate_density_matrix(
    matrix: &ComplexMatrix,
    tolerance: f64,
    validate_positivity: bool,
) -> TomographyResult<Option<f64>> {
    validate_tolerance(tolerance)?;

    matrix.validate_density_matrix(tolerance, tolerance)?;

    if validate_positivity {
        Ok(Some(matrix.validate_positive_semidefinite(tolerance)?))
    } else {
        Ok(None)
    }
}

/// Validates an externally supplied normalized Choi state.
pub fn validate_normalized_choi(
    matrix: &ComplexMatrix,
    input_dimension: usize,
    tolerance: f64,
    validate_positivity: bool,
    validate_trace_preservation_flag: bool,
) -> TomographyResult<Option<f64>> {
    validate_tolerance(tolerance)?;

    matrix.validate_density_matrix(tolerance, tolerance)?;

    let expected = input_dimension
        .checked_mul(input_dimension)
        .ok_or(TomographyError::ArithmeticOverflow)?;

    if matrix.rows() != expected || matrix.columns() != expected {
        return Err(TomographyError::InvalidChoiDimension {
            hilbert_dimension: input_dimension,
            matrix_dimension: matrix.rows(),
        });
    }

    let minimum_eigenvalue = if validate_positivity {
        Some(matrix.validate_positive_semidefinite(tolerance)?)
    } else {
        None
    };

    if validate_trace_preservation_flag {
        validate_trace_preserving(matrix, input_dimension, tolerance)?;
    }

    Ok(minimum_eigenvalue)
}

/// Converts this module's matrix representation into raw `(re, im)` pairs.
///
/// This is the integration boundary for the existing canonical fidelity
/// implementation. The caller can construct its canonical
/// `benchmarking::metrics::fidelity::ComplexMatrix` from these values without
/// making tomography depend on the fidelity module's internal representation.
pub fn to_fidelity_matrix_data(
    matrix: &ComplexMatrix,
) -> (usize, usize, Vec<(f64, f64)>) {
    let data = matrix
        .data()
        .iter()
        .map(|value| (value.re, value.im))
        .collect();

    (matrix.rows(), matrix.columns(), data)
}

// =============================================================================
// Statistical helpers
// =============================================================================

/// Returns a Wilson confidence interval for a binomial probability.
///
/// This helper is included because tomography measurements are ultimately
/// finite-shot observations. It is deliberately independent of the broader
/// statistics module so the reconstruction result remains usable on its own.
///
/// `z` is the standard-normal critical value corresponding to the desired
/// confidence level.
pub fn wilson_interval(
    successes: u64,
    trials: u64,
    z: f64,
) -> TomographyResult<(f64, f64)> {
    if trials == 0 {
        return Err(TomographyError::ZeroShots);
    }

    if successes > trials {
        return Err(TomographyError::CountExceedsShots {
            positive: successes,
            negative: 0,
            shots: trials,
        });
    }

    if !z.is_finite() || z < 0.0 {
        return Err(TomographyError::NonFiniteValue {
            context: "Wilson z-score",
        });
    }

    let n = trials as f64;
    let p = successes as f64 / n;
    let z2 = z * z;

    let denominator = 1.0 + z2 / n;

    if denominator == 0.0 || !denominator.is_finite() {
        return Err(TomographyError::ZeroDenominator);
    }

    let center = (p + z2 / (2.0 * n)) / denominator;

    let half_width =
        z * ((p * (1.0 - p) / n) + (z2 / (4.0 * n * n))).sqrt() / denominator;

    let lower = center - half_width;
    let upper = center + half_width;

    if !lower.is_finite() || !upper.is_finite() {
        return Err(TomographyError::NonFiniteValue {
            context: "Wilson confidence interval",
        });
    }

    Ok((lower.max(0.0), upper.min(1.0)))
}

/// Calculates a confidence interval for a +/-1 Pauli expectation.
///
/// The interval is obtained by transforming the Wilson interval for the
/// positive-outcome probability:
///
/// ```text
/// expectation = 2p - 1
/// ```
pub fn expectation_confidence_interval(
    measurement: PauliMeasurement,
    z: f64,
) -> TomographyResult<(f64, f64)> {
    let shots = measurement.shots();

    let (lower_probability, upper_probability) =
        wilson_interval(measurement.positive, shots, z)?;

    Ok((
        2.0 * lower_probability - 1.0,
        2.0 * upper_probability - 1.0,
    ))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pauli_matrices_are_constructed() {
        for pauli in [Pauli::I, Pauli::X, Pauli::Y, Pauli::Z] {
            let matrix = pauli.matrix();

            assert_eq!(matrix.rows(), 2);
            assert_eq!(matrix.columns(), 2);
        }
    }

    #[test]
    fn pauli_basis_size_is_four_to_n() {
        let limits = TomographyLimits::default();

        let basis = generate_pauli_basis(1, &limits).unwrap();
        assert_eq!(basis.len(), 4);

        let basis = generate_pauli_basis(2, &limits).unwrap();
        assert_eq!(basis.len(), 16);
    }

    #[test]
    fn pauli_basis_order_is_deterministic() {
        let limits = TomographyLimits::default();

        let basis = generate_pauli_basis(1, &limits).unwrap();

        let ids: Vec<String> = basis.iter().map(PauliString::id).collect();

        assert_eq!(ids, vec!["I", "X", "Y", "Z"]);
    }

    #[test]
    fn pauli_measurement_expectation_is_correct() {
        let measurement = PauliMeasurement::new(75, 25).unwrap();

        assert!((measurement.expectation().unwrap() - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn pauli_measurement_rejects_zero_shots() {
        assert!(matches!(
            PauliMeasurement::new(0, 0),
            Err(TomographyError::ZeroShots)
        ));
    }

    #[test]
    fn zero_state_reconstruction_is_valid() {
        let config = StateTomographyConfig {
            qubits: 1,
            ..StateTomographyConfig::default()
        };

        let observations = vec![
            TomographyObservation::new(
                PauliString::new(vec![Pauli::X]).unwrap(),
                PauliMeasurement::new(50, 50).unwrap(),
            )
            .unwrap(),
            TomographyObservation::new(
                PauliString::new(vec![Pauli::Y]).unwrap(),
                PauliMeasurement::new(50, 50).unwrap(),
            )
            .unwrap(),
            TomographyObservation::new(
                PauliString::new(vec![Pauli::Z]).unwrap(),
                PauliMeasurement::new(0, 100).unwrap(),
            )
            .unwrap(),
        ];

        let result = reconstruct_state(&config, &observations).unwrap();

        assert_eq!(result.dimension, 2);
        assert!((result.trace.re - 1.0).abs() < 1.0e-10);
        assert!(result.minimum_eigenvalue.unwrap() >= -1.0e-10);
    }

    #[test]
    fn plus_state_reconstruction_is_valid() {
        let config = StateTomographyConfig {
            qubits: 1,
            ..StateTomographyConfig::default()
        };

        let observations = vec![
            TomographyObservation::new(
                PauliString::new(vec![Pauli::X]).unwrap(),
                PauliMeasurement::new(100, 0).unwrap(),
            )
            .unwrap(),
            TomographyObservation::new(
                PauliString::new(vec![Pauli::Y]).unwrap(),
                PauliMeasurement::new(50, 50).unwrap(),
            )
            .unwrap(),
            TomographyObservation::new(
                PauliString::new(vec![Pauli::Z]).unwrap(),
                PauliMeasurement::new(50, 50).unwrap(),
            )
            .unwrap(),
        ];

        let result = reconstruct_state(&config, &observations).unwrap();

        assert_eq!(result.dimension, 2);

        let rho = &result.density_matrix;

        assert!((rho.get(0, 0).unwrap().re - 0.5).abs() < 1.0e-10);
        assert!((rho.get(1, 1).unwrap().re - 0.5).abs() < 1.0e-10);
        assert!((rho.get(0, 1).unwrap().re - 0.5).abs() < 1.0e-10);
        assert!((rho.get(1, 0).unwrap().re - 0.5).abs() < 1.0e-10);
    }

    #[test]
    fn incomplete_state_tomography_is_rejected() {
        let config = StateTomographyConfig::default();

        let observations = vec![
            TomographyObservation::new(
                PauliString::new(vec![Pauli::X]).unwrap(),
                PauliMeasurement::new(50, 50).unwrap(),
            )
            .unwrap(),
        ];

        assert!(matches!(
            reconstruct_state(&config, &observations),
            Err(TomographyError::IncompleteStateReconstruction { .. })
        ));
    }

    #[test]
    fn density_matrix_validation_rejects_wrong_trace() {
        let matrix = ComplexMatrix::new(
            2,
            2,
            vec![
                Complex64::one(),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::zero(),
            ],
        )
        .unwrap();

        let result = validate_density_matrix(&matrix, 1.0e-10, false);

        assert!(matches!(
            result,
            Err(TomographyError::InvalidTrace { .. })
        ));
    }

    #[test]
    fn hermitian_eigenvalues_are_correct_for_diagonal_matrix() {
        let matrix = ComplexMatrix::new(
            2,
            2,
            vec![
                Complex64::new(0.25, 0.0),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::new(0.75, 0.0),
            ],
        )
        .unwrap();

        let values = hermitian_eigenvalues(&matrix, 1.0e-12).unwrap();

        assert_eq!(values.len(), 2);
        assert!((values[0] - 0.25).abs() < 1.0e-10);
        assert!((values[1] - 0.75).abs() < 1.0e-10);
    }

    #[test]
    fn wilson_interval_is_bounded() {
        let interval = wilson_interval(50, 100, 1.96).unwrap();

        assert!(interval.0 >= 0.0);
        assert!(interval.1 <= 1.0);
        assert!(interval.0 <= interval.1);
    }

    #[test]
    fn expectation_interval_is_bounded() {
        let measurement = PauliMeasurement::new(75, 25).unwrap();

        let interval = expectation_confidence_interval(measurement, 1.96).unwrap();

        assert!(interval.0 >= -1.0);
        assert!(interval.1 <= 1.0);
        assert!(interval.0 <= interval.1);
    }

    #[test]
    fn resource_limits_reject_large_qubit_counts() {
        let limits = TomographyLimits {
            max_qubits: 2,
            ..TomographyLimits::default()
        };

        assert!(matches!(
            generate_pauli_basis(3, &limits),
            Err(TomographyError::ResourceLimitExceeded { .. })
        ));
    }

    #[test]
    fn trace_preservation_for_identity_channel_is_valid() {
        // Normalized Choi state of the one-qubit identity channel:
        //
        // |Phi><Phi|
        //
        // with |Phi> = (|00> + |11>) / sqrt(2).
        let matrix = ComplexMatrix::new(
            4,
            4,
            vec![
                Complex64::new(0.5, 0.0),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::new(0.5, 0.0),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::new(0.5, 0.0),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::new(0.5, 0.0),
            ],
        )
        .unwrap();

        let deviation = trace_preservation_deviation(&matrix, 2).unwrap();

        assert!(deviation < 1.0e-10);
    }
}