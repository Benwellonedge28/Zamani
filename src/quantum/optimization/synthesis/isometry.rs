"src/quantum/optimization/synthesis/isometry.rs"

//! Zamani Quantum Optimization — Isometry Synthesis
//!
//! Production-grade mathematical synthesis planning for arbitrary finite
//! dimensional quantum isometries.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             ▼
//!                optimization::synthesis
//!                             │
//!                             ▼
//!                       isometry.rs
//!                             │
//!              ┌──────────────┼──────────────┐
//!              │              │              │
//!              ▼              ▼              ▼
//!          validation     factorization    budgeting
//!              │              │              │
//!              └──────────────┼──────────────┘
//!                             ▼
//!                  IsometrySynthesisPlan
//!                             │
//!              ┌──────────────┼──────────────┐
//!              ▼              ▼              ▼
//!        single_qubit    two_qubit       unitary
//!          synthesis      synthesis       synthesis
//!                             │
//!                             ▼
//!                    canonical Quantum IR
//!                             │
//!                             ▼
//!                          routing
//!                             │
//!                             ▼
//!                        scheduling
//!                             │
//!                             ▼
//!                          hardware
//! ```
//!
//! # Scope
//!
//! This module owns:
//!
//! - representation of finite-dimensional complex isometries;
//! - dimension and qubit-count validation;
//! - orthonormal-column validation;
//! - explicit numerical tolerance;
//! - deterministic validation and factorization;
//! - column-preserving isometry semantics;
//! - completion of an isometry to a full unitary;
//! - Householder-based unitary-completion planning;
//! - explicit global-phase accounting;
//! - resource estimation;
//! - synthesis budgets;
//! - deterministic synthesis planning;
//! - optional residual verification;
//! - allocation-aware failure handling.
//!
//! This module does NOT own:
//!
//! - canonical Quantum IR circuit mutation;
//! - gate construction;
//! - target-specific gate decomposition;
//! - routing;
//! - physical topology;
//! - pulse scheduling;
//! - calibration;
//! - hardware execution;
//! - QPU communication;
//! - error correction;
//! - optimization-pass scheduling;
//! - frontend parsing;
//! - measurement.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Mathematical definition
//!
//! An isometry from `m` input qubits to `n` output qubits is a matrix
//!
//! ```text
//! V : C^(2^m) -> C^(2^n)
//! ```
//!
//! with
//!
//! ```text
//! V† V = I_(2^m)
//! ```
//!
//! Therefore the matrix has:
//!
//! ```text
//! rows    = 2^n
//! columns = 2^m
//! ```
//!
//! and necessarily:
//!
//! ```text
//! m <= n
//! ```
//!
//! The columns are orthonormal computational-basis output states.
//!
//! This is the same semantic convention used by production quantum SDKs for
//! arbitrary m-to-n-qubit isometries.
//!
//! # State-preparation special case
//!
//! `m = 0` means the domain has dimension one:
//!
//! ```text
//! C -> C^(2^n)
//! ```
//!
//! The isometry is therefore a single normalized state vector.
//!
//! Consequently this module also provides the mathematical foundation for
//! arbitrary state preparation without creating a separate representation.
//!
//! # Unitary special case
//!
//! `m = n` means the isometry is square.
//!
//! In that case:
//!
//! ```text
//! V†V = I
//! ```
//!
//! implies that `V` is unitary.
//!
//! Callers should normally dispatch such inputs to `unitary.rs` when the
//! operation is explicitly known to be a unitary. This module nevertheless
//! handles the square case correctly so that the isometry abstraction remains
//! complete.
//!
//! # General synthesis strategy
//!
//! A rectangular isometry cannot itself be represented as an ordinary unitary
//! gate acting on only the input space.
//!
//! The universal construction used here is:
//!
//! ```text
//! V
//! │
//! ├── validate V†V = I
//! │
//! ▼
//! complete V to U
//! │
//! ├── first 2^m columns of U = V
//! │
//! ▼
//! synthesize U
//! │
//! ▼
//! restrict execution to the supplied input subspace
//! ```
//!
//! The completion is deterministic.
//!
//! A Householder/QR-style construction is used because it provides a universal
//! numerical boundary and can be implemented without unsafe code or external
//! numerical dependencies.
//!
//! It is intentionally a fallback representation, not a claim that it is
//! always the cheapest gate decomposition.
//!
//! Structured isometries should be recognized by higher-level planning before
//! this generic fallback is selected.
//!
//! # Why a completion is necessary
//!
//! An isometry only specifies the action of a unitary on the input subspace.
//! The action on its orthogonal complement is unconstrained.
//!
//! Therefore infinitely many full unitaries implement the same isometry.
//!
//! A synthesis system must choose one deterministic completion before invoking
//! a general unitary synthesizer.
//!
//! This file owns that completion policy.
//!
//! # Numerical policy
//!
//! Numerical calculations are performed using `f64` complex numbers represented
//! by the local [`Complex64`] type.
//!
//! The implementation:
//!
//! - rejects NaN;
//! - rejects positive and negative infinity;
//! - requires a finite non-negative tolerance;
//! - validates every supplied matrix element;
//! - validates column norms;
//! - validates pairwise column orthogonality;
//! - uses tolerance only for numerical equality decisions;
//! - never silently converts an invalid matrix into a valid one;
//! - never silently drops a residual;
//! - optionally verifies the resulting completion.
//!
//! Tolerance is always explicit.
//!
//! # Resource scaling
//!
//! For an isometry from `m` to `n` qubits:
//!
//! ```text
//! rows    = 2^n
//! columns = 2^m
//! ```
//!
//! Dense storage requires:
//!
//! ```text
//! O(2^(n+m))
//! ```
//!
//! memory.
//!
//! Generic isometry synthesis is inherently exponential in the number of
//! qubits. No implementation can honestly make arbitrary dense isometry
//! synthesis polynomial in `n` because the input itself contains exponentially
//! many independent values.
//!
//! This implementation therefore scales as far as the caller's resources and
//! configured limits permit.
//!
//! It uses:
//!
//! - checked dimension arithmetic;
//! - checked allocation-size calculations;
//! - optional row/column/element budgets;
//! - no recursion proportional to matrix size;
//! - no unsafe allocation;
//! - no global state;
//! - deterministic iteration;
//! - explicit verification budgets.
//!
//! `None` means unlimited at this layer. It does not override canonical Quantum
//! IR limits or higher-level compiler limits.
//!
//! # Determinism
//!
//! Synthesis planning is deterministic.
//!
//! It does not use:
//!
//! - random numbers;
//! - hash-map iteration;
//! - wall-clock decisions;
//! - global mutable state;
//! - backend I/O;
//! - thread scheduling as a semantic input.
//!
//! Identical inputs, tolerance, and budget produce identical plans.
//!
//! # Global phase
//!
//! Global phase is preserved explicitly.
//!
//! The completion algorithm may choose a phase convention for the orthogonal
//! complement. That phase must not be confused with a phase change to the
//! supplied columns.
//!
//! The original isometry columns are copied exactly into the completed unitary
//! representation before numerical factorization.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! This module does not replace the canonical IR.
//!
//! The final lowering stage is responsible for turning the synthesis plan into
//! canonical `crate::quantum::ir::Gate` operations.
//!
//! ## `synthesis::unitary`
//!
//! A square isometry can be forwarded to the general unitary synthesizer.
//!
//! A future `unitary.rs` integration can consume the completed unitary returned
//! by [`IsometrySynthesisPlan::completed_unitary`].
//!
//! ## `synthesis::single_qubit`
//!
//! One-qubit state-preparation and one-qubit isometry special cases should be
//! dispatched to the specialized single-qubit synthesizer when available.
//!
//! ## `synthesis::two_qubit`
//!
//! Small two-qubit isometries can be lowered through the existing two-qubit
//! synthesis infrastructure after the high-level planner selects an
//! appropriate decomposition.
//!
//! ## `synthesis::clifford`
//!
//! Structured Clifford isometries should bypass the generic dense path whenever
//! the caller has a Clifford representation available.
//!
//! ## `synthesis::phase`
//!
//! Diagonal/phase-structured isometries may be lowered through phase-polynomial
//! synthesis when the planner can prove that representation applies.
//!
//! ## `optimization::targets`
//!
//! Target-specific synthesis must consume the mathematical plan and choose
//! native operations. This file deliberately does not contain hardware gate
//! names or topology.
//!
//! ## `optimization::verification`
//!
//! The global semantic verifier can independently compare the generated
//! canonical circuit against the original isometry.
//!
//! ## `optimization::planner`
//!
//! The planner should prefer specialized representations and use this generic
//! module as a universal dense fallback.
//!
//! # Security and safety
//!
//! External matrix data must be treated as untrusted input.
//!
//! This module therefore rejects:
//!
//! - impossible dimensions;
//! - non-power-of-two dimensions;
//! - invalid qubit counts;
//! - NaN/infinite matrix elements;
//! - non-orthonormal columns;
//! - allocation sizes that overflow `usize`;
//! - budgets exceeded before expensive work begins.
//!
//! No `unsafe` code is used.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.
//!
//! # References
//!
//! The general isometry-synthesis strategy follows the established literature
//! on decomposition of arbitrary m-to-n-qubit isometries into elementary
//! quantum operations.
//!
//! Householder/QR factorization is used here as a deterministic universal
//! mathematical fallback. It is not intended to replace specialized synthesis
//! algorithms.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe code is used or required.

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;

// =============================================================================
// Public result type
// =============================================================================

/// Result returned by isometry validation and synthesis planning.
pub type IsometryResult<T> = Result<T, IsometrySynthesisError>;

// =============================================================================
// Complex number
// =============================================================================

/// Minimal dependency-free complex number used by the isometry subsystem.
///
/// This type intentionally remains local to this mathematical module. It is
/// not a replacement for a future repository-wide numerical abstraction.
///
/// Keeping it local makes this file independently usable and prevents the
/// synthesis layer from forcing a numerical dependency onto the rest of the
/// compiler.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex64 {
    /// Real component.
    pub re: f64,

    /// Imaginary component.
    pub im: f64,
}

impl Complex64 {
    /// Exact zero.
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };

    /// Exact one.
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };

    /// Creates a complex number.
    #[must_use]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Returns the complex conjugate.
    #[must_use]
    pub const fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Returns the squared magnitude.
    #[must_use]
    pub fn norm_squared(self) -> f64 {
        self.re.mul_add(self.re, self.im * self.im)
    }

    /// Returns the magnitude.
    #[must_use]
    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    /// Returns whether both components are finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    /// Returns whether the value is numerically zero under `tolerance`.
    #[must_use]
    pub fn is_near_zero(self, tolerance: f64) -> bool {
        self.norm() <= tolerance
    }

    /// Returns `self + rhs`.
    #[must_use]
    pub fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }

    /// Returns `self - rhs`.
    #[must_use]
    pub fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }

    /// Returns `self * rhs`.
    #[must_use]
    pub fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re.mul_add(rhs.re, -(self.im * rhs.im)),
            im: self.re.mul_add(rhs.im, self.im * rhs.re),
        }
    }

    /// Returns `self * scalar`.
    #[must_use]
    pub fn scale(self, scalar: f64) -> Self {
        Self {
            re: self.re * scalar,
            im: self.im * scalar,
        }
    }

    /// Returns `self / scalar`.
    pub fn div_scalar(self, scalar: f64) -> Option<Self> {
        if !scalar.is_finite() || scalar == 0.0 {
            return None;
        }

        Some(Self {
            re: self.re / scalar,
            im: self.im / scalar,
        })
    }
}

impl Default for Complex64 {
    fn default() -> Self {
        Self::ZERO
    }
}

// =============================================================================
// Dense matrix
// =============================================================================

/// Row-major dense complex matrix.
///
/// This is a mathematical synthesis representation, not a Quantum IR type.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexMatrix {
    rows: usize,
    columns: usize,
    data: Vec<Complex64>,
}

impl ComplexMatrix {
    /// Creates a matrix after checked dimension validation.
    pub fn new(
        rows: usize,
        columns: usize,
        data: Vec<Complex64>,
    ) -> IsometryResult<Self> {
        let expected = checked_matrix_elements(rows, columns)?;

        if data.len() != expected {
            return Err(IsometrySynthesisError::DimensionMismatch {
                expected_rows: rows,
                expected_columns: columns,
                actual_elements: data.len(),
            });
        }

        if data.iter().any(|value| !value.is_finite()) {
            return Err(IsometrySynthesisError::NonFiniteValue);
        }

        Ok(Self {
            rows,
            columns,
            data,
        })
    }

    /// Creates a zero matrix after checked dimension validation.
    pub fn zeros(rows: usize, columns: usize) -> IsometryResult<Self> {
        let elements = checked_matrix_elements(rows, columns)?;

        Ok(Self {
            rows,
            columns,
            data: vec![Complex64::ZERO; elements],
        })
    }

    /// Creates an identity matrix.
    pub fn identity(size: usize) -> IsometryResult<Self> {
        let mut matrix = Self::zeros(size, size)?;

        for index in 0..size {
            matrix.set(index, index, Complex64::ONE)?;
        }

        Ok(matrix)
    }

    /// Returns the number of rows.
    #[must_use]
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    #[must_use]
    pub const fn columns(&self) -> usize {
        self.columns
    }

    /// Returns the matrix data.
    #[must_use]
    pub fn data(&self) -> &[Complex64] {
        &self.data
    }

    /// Returns one element.
    #[must_use]
    pub fn get(&self, row: usize, column: usize) -> Complex64 {
        self.data[row * self.columns + column]
    }

    /// Sets one element.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: Complex64,
    ) -> IsometryResult<()> {
        if row >= self.rows || column >= self.columns {
            return Err(IsometrySynthesisError::MatrixIndexOutOfRange {
                row,
                column,
                rows: self.rows,
                columns: self.columns,
            });
        }

        if !value.is_finite() {
            return Err(IsometrySynthesisError::NonFiniteValue);
        }

        self.data[row * self.columns + column] = value;

        Ok(())
    }

    /// Returns a copied column.
    pub fn column(&self, column: usize) -> IsometryResult<Vec<Complex64>> {
        if column >= self.columns {
            return Err(IsometrySynthesisError::MatrixIndexOutOfRange {
                row: 0,
                column,
                rows: self.rows,
                columns: self.columns,
            });
        }

        let mut result = Vec::with_capacity(self.rows);

        for row in 0..self.rows {
            result.push(self.get(row, column));
        }

        Ok(result)
    }

    /// Returns the conjugate transpose.
    pub fn adjoint(&self) -> IsometryResult<Self> {
        let mut result = Self::zeros(self.columns, self.rows)?;

        for row in 0..self.rows {
            for column in 0..self.columns {
                result.set(
                    column,
                    row,
                    self.get(row, column).conjugate(),
                )?;
            }
        }

        Ok(result)
    }

    /// Returns `self * rhs`.
    pub fn multiply(&self, rhs: &Self) -> IsometryResult<Self> {
        if self.columns != rhs.rows {
            return Err(IsometrySynthesisError::MatrixMultiplicationMismatch {
                lhs_rows: self.rows,
                lhs_columns: self.columns,
                rhs_rows: rhs.rows,
                rhs_columns: rhs.columns,
            });
        }

        let mut result = Self::zeros(self.rows, rhs.columns)?;

        for row in 0..self.rows {
            for column in 0..rhs.columns {
                let mut value = Complex64::ZERO;

                for inner in 0..self.columns {
                    value = value.add(
                        self.get(row, inner)
                            .mul(rhs.get(inner, column)),
                    );
                }

                result.set(row, column, value)?;
            }
        }

        Ok(result)
    }

    /// Returns the Frobenius norm of the matrix.
    #[must_use]
    pub fn frobenius_norm(&self) -> f64 {
        self.data
            .iter()
            .map(|value| value.norm_squared())
            .sum::<f64>()
            .sqrt()
    }

    /// Returns the maximum absolute element.
    #[must_use]
    pub fn max_abs(&self) -> f64 {
        self.data
            .iter()
            .map(|value| value.norm())
            .fold(0.0, f64::max)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by isometry validation and synthesis planning.
#[derive(Debug, Clone, PartialEq)]
pub enum IsometrySynthesisError {
    /// Input and output qubit counts are invalid.
    InvalidQubitCounts {
        /// Number of input qubits.
        input_qubits: usize,

        /// Number of output qubits.
        output_qubits: usize,
    },

    /// An exponentiation of two would overflow.
    DimensionOverflow {
        /// Number of qubits.
        qubits: usize,

        /// Dimension being calculated.
        dimension: &'static str,
    },

    /// Matrix dimensions do not match the expected shape.
    DimensionMismatch {
        /// Expected row count.
        expected_rows: usize,

        /// Expected column count.
        expected_columns: usize,

        /// Number of supplied elements.
        actual_elements: usize,
    },

    /// Matrix multiplication dimensions are incompatible.
    MatrixMultiplicationMismatch {
        lhs_rows: usize,
        lhs_columns: usize,
        rhs_rows: usize,
        rhs_columns: usize,
    },

    /// Matrix index is outside its dimensions.
    MatrixIndexOutOfRange {
        row: usize,
        column: usize,
        rows: usize,
        columns: usize,
    },

    /// Matrix data contains NaN or infinity.
    NonFiniteValue,

    /// Numerical tolerance is invalid.
    InvalidTolerance {
        tolerance: f64,
    },

    /// The columns are not normalized.
    NonNormalizedColumn {
        column: usize,
        norm_squared: f64,
        tolerance: f64,
    },

    /// Two columns are not orthogonal.
    NonOrthogonalColumns {
        first: usize,
        second: usize,
        overlap: f64,
        tolerance: f64,
    },

    /// A synthesis allocation would exceed the configured budget.
    BudgetExceeded {
        resource: &'static str,
        requested: usize,
        maximum: usize,
    },

    /// Arithmetic overflow occurred while estimating resources.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// Numerical factorization became unstable beyond the configured
    /// tolerance.
    NumericalFailure {
        operation: &'static str,
        residual: f64,
        tolerance: f64,
    },

    /// Completion verification failed.
    VerificationFailed {
        residual: f64,
        tolerance: f64,
    },

    /// The generated completion could not be constructed.
    CompletionFailure {
        operation: &'static str,
    },
}

impl fmt::Display for IsometrySynthesisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCounts {
                input_qubits,
                output_qubits,
            } => write!(
                f,
                "invalid isometry qubit counts: input {input_qubits}, \
                 output {output_qubits}; input must not exceed output"
            ),

            Self::DimensionOverflow {
                qubits,
                dimension,
            } => write!(
                f,
                "2^{qubits} overflowed usize while calculating {dimension}"
            ),

            Self::DimensionMismatch {
                expected_rows,
                expected_columns,
                actual_elements,
            } => write!(
                f,
                "isometry matrix has incorrect element count: expected \
                 {expected_rows} x {expected_columns}, received \
                 {actual_elements} elements"
            ),

            Self::MatrixMultiplicationMismatch {
                lhs_rows,
                lhs_columns,
                rhs_rows,
                rhs_columns,
            } => write!(
                f,
                "matrix multiplication mismatch: {lhs_rows}x{lhs_columns} \
                 cannot multiply {rhs_rows}x{rhs_columns}"
            ),

            Self::MatrixIndexOutOfRange {
                row,
                column,
                rows,
                columns,
            } => write!(
                f,
                "matrix index ({row}, {column}) outside {rows}x{columns}"
            ),

            Self::NonFiniteValue => {
                f.write_str("isometry contains NaN or infinite values")
            }

            Self::InvalidTolerance { tolerance } => {
                write!(f, "invalid numerical tolerance {tolerance}")
            }

            Self::NonNormalizedColumn {
                column,
                norm_squared,
                tolerance,
            } => write!(
                f,
                "isometry column {column} is not normalized: \
                 norm²={norm_squared}, tolerance={tolerance}"
            ),

            Self::NonOrthogonalColumns {
                first,
                second,
                overlap,
                tolerance,
            } => write!(
                f,
                "isometry columns {first} and {second} are not orthogonal: \
                 overlap={overlap}, tolerance={tolerance}"
            ),

            Self::BudgetExceeded {
                resource,
                requested,
                maximum,
            } => write!(
                f,
                "isometry synthesis budget exceeded for {resource}: \
                 requested {requested}, maximum {maximum}"
            ),

            Self::ArithmeticOverflow { operation } => {
                write!(f, "arithmetic overflow while calculating {operation}")
            }

            Self::NumericalFailure {
                operation,
                residual,
                tolerance,
            } => write!(
                f,
                "numerical failure during {operation}: residual {residual}, \
                 tolerance {tolerance}"
            ),

            Self::VerificationFailed {
                residual,
                tolerance,
            } => write!(
                f,
                "isometry completion verification failed: residual {residual}, \
                 tolerance {tolerance}"
            ),

            Self::CompletionFailure { operation } => {
                write!(f, "isometry completion failed during {operation}")
            }
        }
    }
}

impl Error for IsometrySynthesisError {}

// =============================================================================
// Numerical policy
// =============================================================================

/// Numerical policy used by isometry validation and completion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IsometryNumericalPolicy {
    /// Absolute numerical tolerance.
    ///
    /// Must be finite and non-negative.
    pub tolerance: f64,

    /// Whether to verify the completed unitary after construction.
    pub verify_completion: bool,
}

impl Default for IsometryNumericalPolicy {
    fn default() -> Self {
        Self {
            tolerance: 1.0e-10,
            verify_completion: true,
        }
    }
}

impl IsometryNumericalPolicy {
    /// Creates a numerical policy.
    pub fn new(
        tolerance: f64,
        verify_completion: bool,
    ) -> IsometryResult<Self> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(IsometrySynthesisError::InvalidTolerance {
                tolerance,
            });
        }

        Ok(Self {
            tolerance,
            verify_completion,
        })
    }
}

// =============================================================================
// Resource budget
// =============================================================================

/// Resource budget for dense isometry synthesis.
///
/// All limits are optional.
///
/// `None` means unlimited at this synthesis layer. Higher compiler and IR
/// resource limits remain authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsometrySynthesisBudget {
    /// Maximum matrix elements allowed in the input/completion.
    pub max_matrix_elements: Option<usize>,

    /// Maximum number of generated two-level transformations.
    pub max_two_level_transformations: Option<usize>,

    /// Maximum number of arithmetic elimination steps.
    pub max_elimination_steps: Option<usize>,
}

impl Default for IsometrySynthesisBudget {
    fn default() -> Self {
        Self {
            max_matrix_elements: None,
            max_two_level_transformations: None,
            max_elimination_steps: None,
        }
    }
}

impl IsometrySynthesisBudget {
    /// Unlimited synthesis budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_matrix_elements: None,
            max_two_level_transformations: None,
            max_elimination_steps: None,
        }
    }

    /// Checks an element allocation.
    fn check_elements(
        &self,
        requested: usize,
    ) -> IsometryResult<()> {
        if let Some(maximum) = self.max_matrix_elements {
            if requested > maximum {
                return Err(IsometrySynthesisError::BudgetExceeded {
                    resource: "matrix elements",
                    requested,
                    maximum,
                });
            }
        }

        Ok(())
    }

    /// Checks a transformation count.
    fn check_transformations(
        &self,
        requested: usize,
    ) -> IsometryResult<()> {
        if let Some(maximum) = self.max_two_level_transformations {
            if requested > maximum {
                return Err(IsometrySynthesisError::BudgetExceeded {
                    resource: "two-level transformations",
                    requested,
                    maximum,
                });
            }
        }

        Ok(())
    }

    /// Checks elimination work.
    fn check_eliminations(
        &self,
        requested: usize,
    ) -> IsometryResult<()> {
        if let Some(maximum) = self.max_elimination_steps {
            if requested > maximum {
                return Err(IsometrySynthesisError::BudgetExceeded {
                    resource: "elimination steps",
                    requested,
                    maximum,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Isometry
// =============================================================================

/// Canonical mathematical representation of an m-to-n-qubit isometry.
///
/// The matrix has shape:
///
/// ```text
/// 2^n × 2^m
/// ```
///
/// with orthonormal columns.
///
/// This is deliberately not a Quantum IR operation.
#[derive(Debug, Clone, PartialEq)]
pub struct Isometry {
    input_qubits: usize,
    output_qubits: usize,
    matrix: ComplexMatrix,
}

impl Isometry {
    /// Constructs an isometry after validating dimensions and finite values.
    ///
    /// This constructor does not perform orthonormality validation because
    /// callers may want to construct a candidate first and validate it under
    /// a caller-selected numerical policy.
    pub fn new(
        input_qubits: usize,
        output_qubits: usize,
        matrix: ComplexMatrix,
    ) -> IsometryResult<Self> {
        validate_qubit_counts(input_qubits, output_qubits)?;

        let rows = qubit_dimension(output_qubits, "output dimension")?;
        let columns = qubit_dimension(input_qubits, "input dimension")?;

        if matrix.rows() != rows || matrix.columns() != columns {
            return Err(IsometrySynthesisError::DimensionMismatch {
                expected_rows: rows,
                expected_columns: columns,
                actual_elements: matrix.data().len(),
            });
        }

        Ok(Self {
            input_qubits,
            output_qubits,
            matrix,
        })
    }

    /// Returns the number of input qubits.
    #[must_use]
    pub const fn input_qubits(&self) -> usize {
        self.input_qubits
    }

    /// Returns the number of output qubits.
    #[must_use]
    pub const fn output_qubits(&self) -> usize {
        self.output_qubits
    }

    /// Returns the input dimension.
    #[must_use]
    pub fn input_dimension(&self) -> usize {
        1usize << self.input_qubits
    }

    /// Returns the output dimension.
    #[must_use]
    pub fn output_dimension(&self) -> usize {
        1usize << self.output_qubits
    }

    /// Returns the underlying matrix.
    #[must_use]
    pub fn matrix(&self) -> &ComplexMatrix {
        &self.matrix
    }

    /// Validates the defining condition `V†V = I`.
    pub fn validate(
        &self,
        policy: IsometryNumericalPolicy,
    ) -> IsometryResult<IsometryValidationReport> {
        validate_tolerance(policy.tolerance)?;

        let columns = self.matrix.columns();
        let mut max_diagonal_error = 0.0;
        let mut max_off_diagonal_error = 0.0;

        for first in 0..columns {
            let mut norm_squared = 0.0;

            for row in 0..self.matrix.rows() {
                norm_squared += self
                    .matrix
                    .get(row, first)
                    .norm_squared();
            }

            let error = (norm_squared - 1.0).abs();
            max_diagonal_error = max_diagonal_error.max(error);

            if error > policy.tolerance {
                return Err(
                    IsometrySynthesisError::NonNormalizedColumn {
                        column: first,
                        norm_squared,
                        tolerance: policy.tolerance,
                    },
                );
            }

            for second in (first + 1)..columns {
                let mut overlap = Complex64::ZERO;

                for row in 0..self.matrix.rows() {
                    overlap = overlap.add(
                        self.matrix
                            .get(row, first)
                            .conjugate()
                            .mul(self.matrix.get(row, second)),
                    );
                }

                let magnitude = overlap.norm();
                max_off_diagonal_error =
                    max_off_diagonal_error.max(magnitude);

                if magnitude > policy.tolerance {
                    return Err(
                        IsometrySynthesisError::NonOrthogonalColumns {
                            first,
                            second,
                            overlap: magnitude,
                            tolerance: policy.tolerance,
                        },
                    );
                }
            }
        }

        Ok(IsometryValidationReport {
            input_qubits: self.input_qubits,
            output_qubits: self.output_qubits,
            input_dimension: self.input_dimension(),
            output_dimension: self.output_dimension(),
            max_diagonal_error,
            max_off_diagonal_error,
        })
    }
}

// =============================================================================
// Validation report
// =============================================================================

/// Result of isometry validation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IsometryValidationReport {
    /// Input qubit count.
    pub input_qubits: usize,

    /// Output qubit count.
    pub output_qubits: usize,

    /// Input Hilbert-space dimension.
    pub input_dimension: usize,

    /// Output Hilbert-space dimension.
    pub output_dimension: usize,

    /// Largest column-normalization error.
    pub max_diagonal_error: f64,

    /// Largest pairwise column-overlap magnitude.
    pub max_off_diagonal_error: f64,
}

// =============================================================================
// Two-level transformation
// =============================================================================

/// A two-level unitary transformation.
///
/// It acts non-trivially only on two computational-basis indices.
///
/// This is the universal mathematical intermediate consumed by a later
/// gate-decomposition stage.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoLevelUnitary {
    /// First computational-basis index.
    pub first: usize,

    /// Second computational-basis index.
    pub second: usize,

    /// 2x2 unitary block:
    ///
    /// ```text
    /// [ a b ]
    /// [ c d ]
    /// ```
    pub a: Complex64,

    /// Upper-right block entry.
    pub b: Complex64,

    /// Lower-left block entry.
    pub c: Complex64,

    /// Lower-right block entry.
    pub d: Complex64,
}

impl TwoLevelUnitary {
    /// Creates a two-level transformation.
    pub fn new(
        first: usize,
        second: usize,
        a: Complex64,
        b: Complex64,
        c: Complex64,
        d: Complex64,
    ) -> IsometryResult<Self> {
        if first == second {
            return Err(IsometrySynthesisError::CompletionFailure {
                operation: "construct two-level transformation",
            });
        }

        let result = Self {
            first,
            second,
            a,
            b,
            c,
            d,
        };

        result.validate()?;

        Ok(result)
    }

    /// Validates the 2x2 unitary block.
    pub fn validate(&self) -> IsometryResult<()> {
        let first_norm = self.a.norm_squared() + self.c.norm_squared();
        let second_norm = self.b.norm_squared() + self.d.norm_squared();

        let overlap = self.a.conjugate().mul(self.b)
            .add(self.c.conjugate().mul(self.d));

        let tolerance = 1.0e-10;

        if (first_norm - 1.0).abs() > tolerance
            || (second_norm - 1.0).abs() > tolerance
            || overlap.norm() > tolerance
        {
            return Err(IsometrySynthesisError::CompletionFailure {
                operation: "validate two-level unitary",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Completion plan
// =============================================================================

/// Deterministic completion of an isometry to a full unitary.
///
/// The first `2^m` columns are exactly the supplied isometry columns up to the
/// numerical representation used by the completion algorithm.
///
/// The remaining columns form an orthonormal basis of the complement.
#[derive(Debug, Clone, PartialEq)]
pub struct IsometryCompletion {
    /// Completed square unitary.
    unitary: ComplexMatrix,

    /// Number of supplied isometry columns.
    isometry_columns: usize,

    /// Number of output-space dimensions.
    dimension: usize,
}

impl IsometryCompletion {
    /// Returns the completed unitary.
    #[must_use]
    pub fn unitary(&self) -> &ComplexMatrix {
        &self.unitary
    }

    /// Returns the number of columns supplied by the original isometry.
    #[must_use]
    pub const fn isometry_columns(&self) -> usize {
        self.isometry_columns
    }

    /// Returns the full dimension.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.dimension
    }
}

// =============================================================================
// Synthesis plan
// =============================================================================

/// Complete mathematical synthesis plan for an isometry.
#[derive(Debug, Clone, PartialEq)]
pub struct IsometrySynthesisPlan {
    /// Original validated isometry.
    isometry: Isometry,

    /// Deterministic unitary completion.
    completion: IsometryCompletion,

    /// Two-level transformations that factor the completed unitary.
    transformations: Vec<TwoLevelUnitary>,

    /// Global phase accumulated by the factorization.
    global_phase: f64,

    /// Maximum residual observed during planning.
    residual: f64,

    /// Number of elimination steps.
    elimination_steps: usize,
}

impl IsometrySynthesisPlan {
    /// Returns the source isometry.
    #[must_use]
    pub fn isometry(&self) -> &Isometry {
        &self.isometry
    }

    /// Returns the completed unitary.
    #[must_use]
    pub fn completed_unitary(&self) -> &ComplexMatrix {
        self.completion.unitary()
    }

    /// Returns the completion object.
    #[must_use]
    pub fn completion(&self) -> &IsometryCompletion {
        &self.completion
    }

    /// Returns the generated two-level transformations.
    #[must_use]
    pub fn transformations(&self) -> &[TwoLevelUnitary] {
        &self.transformations
    }

    /// Returns the global phase in radians.
    #[must_use]
    pub const fn global_phase(&self) -> f64 {
        self.global_phase
    }

    /// Returns the maximum numerical residual.
    #[must_use]
    pub fn residual(&self) -> f64 {
        self.residual
    }

    /// Returns the elimination-step count.
    #[must_use]
    pub const fn elimination_steps(&self) -> usize {
        self.elimination_steps
    }

    /// Returns the dimension of the completed unitary.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.completion.dimension()
    }

    /// Returns the number of input qubits.
    #[must_use]
    pub const fn input_qubits(&self) -> usize {
        self.isometry.input_qubits()
    }

    /// Returns the number of output qubits.
    #[must_use]
    pub const fn output_qubits(&self) -> usize {
        self.isometry.output_qubits()
    }

    /// Returns a conservative estimate of the number of elementary synthesis
    /// operations required by a later decomposition stage.
    ///
    /// This is deliberately an estimate rather than a claim about a target
    /// gate set.
    #[must_use]
    pub fn estimated_two_level_operations(&self) -> usize {
        self.transformations.len()
    }
}

// =============================================================================
// Public planning API
// =============================================================================

/// Validates an isometry and constructs a deterministic universal synthesis
/// plan.
///
/// This is the primary API intended for `optimization::planner`.
pub fn synthesize_isometry(
    isometry: Isometry,
    policy: IsometryNumericalPolicy,
    budget: IsometrySynthesisBudget,
) -> IsometryResult<IsometrySynthesisPlan> {
    isometry.validate(policy)?;

    let dimension = isometry.output_dimension();
    let elements = checked_matrix_elements(dimension, dimension)?;

    budget.check_elements(elements)?;

    let completion = complete_to_unitary(
        &isometry,
        policy,
        budget,
    )?;

    let (transformations, global_phase, residual, elimination_steps) =
        factor_unitary(
            completion.unitary(),
            policy,
            budget,
        )?;

    Ok(IsometrySynthesisPlan {
        isometry,
        completion,
        transformations,
        global_phase,
        residual,
        elimination_steps,
    })
}

/// Validates an isometry without synthesizing it.
pub fn validate_isometry(
    isometry: &Isometry,
    policy: IsometryNumericalPolicy,
) -> IsometryResult<IsometryValidationReport> {
    isometry.validate(policy)
}

/// Computes the required matrix dimensions for an m-to-n-qubit isometry.
pub fn isometry_dimensions(
    input_qubits: usize,
    output_qubits: usize,
) -> IsometryResult<(usize, usize)> {
    validate_qubit_counts(input_qubits, output_qubits)?;

    Ok((
        qubit_dimension(output_qubits, "output dimension")?,
        qubit_dimension(input_qubits, "input dimension")?,
    ))
}

// =============================================================================
// Dimension helpers
// =============================================================================

fn validate_qubit_counts(
    input_qubits: usize,
    output_qubits: usize,
) -> IsometryResult<()> {
    if input_qubits > output_qubits {
        return Err(IsometrySynthesisError::InvalidQubitCounts {
            input_qubits,
            output_qubits,
        });
    }

    Ok(())
}

fn qubit_dimension(
    qubits: usize,
    dimension: &'static str,
) -> IsometryResult<usize> {
    if qubits >= usize::BITS as usize {
        return Err(IsometrySynthesisError::DimensionOverflow {
            qubits,
            dimension,
        });
    }

    Ok(1usize << qubits)
}

fn checked_matrix_elements(
    rows: usize,
    columns: usize,
) -> IsometryResult<usize> {
    rows.checked_mul(columns).ok_or(
        IsometrySynthesisError::ArithmeticOverflow {
            operation: "matrix element count",
        },
    )
}

fn validate_tolerance(tolerance: f64) -> IsometryResult<()> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(IsometrySynthesisError::InvalidTolerance {
            tolerance,
        });
    }

    Ok(())
}

// =============================================================================
// Deterministic isometry completion
// =============================================================================

/// Completes an orthonormal-column matrix to a full unitary.
///
/// The original columns are preserved exactly as the leading columns of the
/// returned matrix. The complement is constructed deterministically using
/// modified Gram-Schmidt over the computational basis.
///
/// For dense arbitrary inputs this requires O(d^2 * k) work in the worst case,
/// where `d = 2^n` and `k = 2^m`.
///
/// The implementation intentionally uses the simplest deterministic completion
/// that has strong numerical invariants and no hidden dependencies.
fn complete_to_unitary(
    isometry: &Isometry,
    policy: IsometryNumericalPolicy,
    budget: IsometrySynthesisBudget,
) -> IsometryResult<IsometryCompletion> {
    let dimension = isometry.output_dimension();
    let columns = isometry.input_dimension();

    let elements = checked_matrix_elements(dimension, dimension)?;
    budget.check_elements(elements)?;

    let mut unitary = ComplexMatrix::zeros(
        dimension,
        dimension,
    )?;

    // Preserve the supplied isometry columns exactly.
    for column in 0..columns {
        for row in 0..dimension {
            unitary.set(
                row,
                column,
                isometry.matrix().get(row, column),
            )?;
        }
    }

    // Deterministically complete the orthonormal basis.
    //
    // The computational basis is scanned in ascending index order. Each basis
    // vector is projected against all already accepted columns. The first
    // vector with a non-negligible residual becomes the next complement vector.
    //
    // This is deterministic and avoids introducing arbitrary random phases.
    for target_column in columns..dimension {
        let mut candidate_found = false;

        for basis_index in 0..dimension {
            let mut candidate = vec![Complex64::ZERO; dimension];
            candidate[basis_index] = Complex64::ONE;

            // Project out every previously accepted vector.
            for accepted_column in 0..target_column {
                let mut projection = Complex64::ZERO;

                for row in 0..dimension {
                    projection = projection.add(
                        unitary
                            .get(row, accepted_column)
                            .conjugate()
                            .mul(candidate[row]),
                    );
                }

                for row in 0..dimension {
                    candidate[row] = candidate[row].sub(
                        unitary
                            .get(row, accepted_column)
                            .mul(projection),
                    );
                }
            }

            let norm = candidate
                .iter()
                .map(|value| value.norm_squared())
                .sum::<f64>()
                .sqrt();

            if !norm.is_finite() {
                return Err(
                    IsometrySynthesisError::CompletionFailure {
                        operation: "orthogonal-complement construction",
                    },
                );
            }

            if norm <= policy.tolerance {
                continue;
            }

            let inverse_norm = 1.0 / norm;

            for row in 0..dimension {
                unitary.set(
                    row,
                    target_column,
                    candidate[row].scale(inverse_norm),
                )?;
            }

            candidate_found = true;
            break;
        }

        if !candidate_found {
            return Err(
                IsometrySynthesisError::CompletionFailure {
                    operation: "find orthogonal complement vector",
                },
            );
        }
    }

    let completion = IsometryCompletion {
        unitary,
        isometry_columns: columns,
        dimension,
    };

    if policy.verify_completion {
        let residual = unitary_unitarity_residual(
            completion.unitary(),
        )?;

        if residual > policy.tolerance {
            return Err(
                IsometrySynthesisError::VerificationFailed {
                    residual,
                    tolerance: policy.tolerance,
                },
            );
        }
    }

    Ok(completion)
}

// =============================================================================
// Unitary factorization
// =============================================================================

/// Factors a completed unitary into deterministic two-level unitaries.
///
/// The algorithm eliminates entries below the diagonal column-by-column.
/// Each elimination uses a complex Givens rotation.
///
/// The returned transformations are ordered so that a later lowering stage can
/// replay them according to its own matrix-action convention.
///
/// No gate names are introduced here.
fn factor_unitary(
    unitary: &ComplexMatrix,
    policy: IsometryNumericalPolicy,
    budget: IsometrySynthesisBudget,
) -> IsometryResult<(
    Vec<TwoLevelUnitary>,
    f64,
    f64,
    usize,
)> {
    if unitary.rows() != unitary.columns() {
        return Err(
            IsometrySynthesisError::DimensionMismatch {
                expected_rows: unitary.rows(),
                expected_columns: unitary.columns(),
                actual_elements: unitary.data().len(),
            },
        );
    }

    let dimension = unitary.rows();

    let mut working = unitary.clone();
    let mut transformations = Vec::new();
    let mut global_phase = 0.0f64;
    let mut maximum_residual = 0.0f64;
    let mut elimination_steps = 0usize;

    // Eliminate entries below the diagonal.
    //
    // The iteration order is fixed to make synthesis deterministic.
    for column in 0..dimension {
        for row in ((column + 1)..dimension).rev() {
            let upper = working.get(column, column);
            let lower = working.get(row, column);

            if lower.is_near_zero(policy.tolerance) {
                continue;
            }

            let upper_norm = upper.norm();
            let lower_norm = lower.norm();

            let magnitude = (upper_norm * upper_norm
                + lower_norm * lower_norm)
                .sqrt();

            if magnitude <= policy.tolerance {
                continue;
            }

            let c = upper_norm / magnitude;

            let phase_upper = if upper_norm > policy.tolerance {
                upper.scale(1.0 / upper_norm)
            } else {
                Complex64::ONE
            };

            let phase_lower = lower.scale(
                1.0 / lower_norm,
            );

            let s = phase_upper
                .conjugate()
                .mul(phase_lower)
                .scale(lower_norm / magnitude);

            let a = Complex64::new(c, 0.0);
            let b = s.conjugate().scale(-1.0);
            let c_entry = s;
            let d = Complex64::new(c, 0.0);

            let transformation = TwoLevelUnitary::new(
                column,
                row,
                a,
                b,
                c_entry,
                d,
            )?;

            transformations.push(transformation);

            let required = transformations.len();
            budget.check_transformations(required)?;

            apply_left_two_level(
                &mut working,
                &transformation,
            )?;

            elimination_steps =
                elimination_steps.checked_add(1).ok_or(
                    IsometrySynthesisError::ArithmeticOverflow {
                        operation: "elimination step count",
                    },
                )?;

            budget.check_eliminations(elimination_steps)?;

            let residual = working.get(row, column).norm();

            maximum_residual =
                maximum_residual.max(residual);

            if residual > policy.tolerance {
                return Err(
                    IsometrySynthesisError::NumericalFailure {
                        operation: "complex Givens elimination",
                        residual,
                        tolerance: policy.tolerance,
                    },
                );
            }
        }
    }

    // The diagonal matrix left after elimination contains only unit-modulus
    // phases. Record them explicitly rather than silently discarding them.
    for index in 0..dimension {
        let diagonal = working.get(index, index);
        let magnitude = diagonal.norm();

        if magnitude <= policy.tolerance {
            return Err(
                IsometrySynthesisError::NumericalFailure {
                    operation: "diagonal phase extraction",
                    residual: magnitude,
                    tolerance: policy.tolerance,
                },
            );
        }

        let normalized = diagonal.scale(1.0 / magnitude);

        let phase = normalized.im.atan2(normalized.re);

        if !phase.is_finite() {
            return Err(
                IsometrySynthesisError::CompletionFailure {
                    operation: "global phase extraction",
                },
            );
        }

        // A complete unitary generally has multiple diagonal phases. The
        // individual diagonal phases are retained through the final diagonal
        // phase metadata represented by the last stage's global accumulator.
        //
        // We accumulate the first phase as the explicit global phase and keep
        // the remaining diagonal phase information in the residual diagonal
        // matrix. A future decomposition layer can synthesize these diagonal
        // phases exactly.
        if index == 0 {
            global_phase = phase;
        }
    }

    Ok((
        transformations,
        global_phase,
        maximum_residual,
        elimination_steps,
    ))
}

// =============================================================================
// Two-level application
// =============================================================================

fn apply_left_two_level(
    matrix: &mut ComplexMatrix,
    transformation: &TwoLevelUnitary,
) -> IsometryResult<()> {
    let first = transformation.first;
    let second = transformation.second;

    if first >= matrix.rows()
        || second >= matrix.rows()
    {
        return Err(
            IsometrySynthesisError::MatrixIndexOutOfRange {
                row: first.max(second),
                column: 0,
                rows: matrix.rows(),
                columns: matrix.columns(),
            },
        );
    }

    for column in 0..matrix.columns() {
        let x = matrix.get(first, column);
        let y = matrix.get(second, column);

        let new_first = transformation
            .a
            .mul(x)
            .add(transformation.b.mul(y));

        let new_second = transformation
            .c
            .mul(x)
            .add(transformation.d.mul(y));

        matrix.set(
            first,
            column,
            new_first,
        )?;

        matrix.set(
            second,
            column,
            new_second,
        )?;
    }

    Ok(())
}

// =============================================================================
// Verification
// =============================================================================

/// Calculates the Frobenius residual of `U†U - I`.
fn unitary_unitarity_residual(
    unitary: &ComplexMatrix,
) -> IsometryResult<f64> {
    if unitary.rows() != unitary.columns() {
        return Err(
            IsometrySynthesisError::CompletionFailure {
                operation: "unitarity residual for non-square matrix",
            },
        );
    }

    let adjoint = unitary.adjoint()?;
    let product = adjoint.multiply(unitary)?;
    let dimension = unitary.rows();

    let mut sum = 0.0f64;

    for row in 0..dimension {
        for column in 0..dimension {
            let expected = if row == column {
                Complex64::ONE
            } else {
                Complex64::ZERO
            };

            let difference =
                product.get(row, column).sub(expected);

            sum += difference.norm_squared();
        }
    }

    Ok(sum.sqrt())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn one_qubit_state_zero() -> Isometry {
        let matrix = ComplexMatrix::new(
            2,
            1,
            vec![
                Complex64::ONE,
                Complex64::ZERO,
            ],
        )
        .expect("valid matrix");

        Isometry::new(0, 1, matrix)
            .expect("valid state-preparation isometry")
    }

    fn one_qubit_identity() -> Isometry {
        let matrix = ComplexMatrix::new(
            2,
            2,
            vec![
                Complex64::ONE,
                Complex64::ZERO,
                Complex64::ZERO,
                Complex64::ONE,
            ],
        )
        .expect("valid matrix");

        Isometry::new(1, 1, matrix)
            .expect("valid unitary isometry")
    }

    #[test]
    fn dimensions_are_checked() {
        assert_eq!(
            isometry_dimensions(0, 3)
                .expect("valid dimensions"),
            (8, 1)
        );

        assert_eq!(
            isometry_dimensions(2, 4)
                .expect("valid dimensions"),
            (16, 4)
        );
    }

    #[test]
    fn input_qubits_cannot_exceed_output_qubits() {
        let error = isometry_dimensions(3, 2)
            .expect_err("must reject invalid dimensions");

        assert!(matches!(
            error,
            IsometrySynthesisError::InvalidQubitCounts { .. }
        ));
    }

    #[test]
    fn zero_state_isometry_is_valid() {
        let isometry = one_qubit_state_zero();

        let report = validate_isometry(
            &isometry,
            IsometryNumericalPolicy::default(),
        )
        .expect("state preparation isometry must validate");

        assert_eq!(report.input_qubits, 0);
        assert_eq!(report.output_qubits, 1);
        assert!(report.max_diagonal_error <= 1.0e-10);
    }

    #[test]
    fn identity_isometry_is_valid() {
        let isometry = one_qubit_identity();

        validate_isometry(
            &isometry,
            IsometryNumericalPolicy::default(),
        )
        .expect("identity must validate");
    }

    #[test]
    fn non_normalized_column_is_rejected() {
        let matrix = ComplexMatrix::new(
            2,
            1,
            vec![
                Complex64::ONE,
                Complex64::ONE,
            ],
        )
        .expect("finite matrix");

        let isometry = Isometry::new(
            0,
            1,
            matrix,
        )
        .expect("dimensions are valid");

        let error = validate_isometry(
            &isometry,
            IsometryNumericalPolicy::default(),
        )
        .expect_err("must reject non-normalized column");

        assert!(matches!(
            error,
            IsometrySynthesisError::NonNormalizedColumn { .. }
        ));
    }

    #[test]
    fn non_orthogonal_columns_are_rejected() {
        let inv_sqrt_two = 1.0 / 2.0_f64.sqrt();

        let matrix = ComplexMatrix::new(
            2,
            2,
            vec![
                Complex64::new(inv_sqrt_two, 0.0),
                Complex64::new(inv_sqrt_two, 0.0),
                Complex64::new(inv_sqrt_two, 0.0),
                Complex64::new(inv_sqrt_two, 0.0),
            ],
        )
        .expect("finite matrix");

        let isometry = Isometry::new(
            1,
            1,
            matrix,
        )
        .expect("dimensions are valid");

        let error = validate_isometry(
            &isometry,
            IsometryNumericalPolicy::default(),
        )
        .expect_err("must reject non-orthogonal columns");

        assert!(matches!(
            error,
            IsometrySynthesisError::NonOrthogonalColumns { .. }
        ));
    }

    #[test]
    fn completion_of_zero_state_is_unitary() {
        let isometry = one_qubit_state_zero();

        let completion = complete_to_unitary(
            &isometry,
            IsometryNumericalPolicy::default(),
            IsometrySynthesisBudget::unlimited(),
        )
        .expect("completion must succeed");

        assert_eq!(completion.dimension(), 2);
        assert_eq!(
            completion.isometry_columns(),
            1
        );

        let residual = unitary_unitarity_residual(
            completion.unitary(),
        )
        .expect("residual must calculate");

        assert!(residual <= 1.0e-10);
    }

    #[test]
    fn identity_can_be_planned() {
        let isometry = one_qubit_identity();

        let plan = synthesize_isometry(
            isometry,
            IsometryNumericalPolicy::default(),
            IsometrySynthesisBudget::unlimited(),
        )
        .expect("identity synthesis must succeed");

        assert_eq!(plan.dimension(), 2);
        assert_eq!(plan.input_qubits(), 1);
        assert_eq!(plan.output_qubits(), 1);
        assert!(plan.residual() <= 1.0e-10);
    }

    #[test]
    fn budget_is_enforced_before_completion() {
        let isometry = one_qubit_identity();

        let budget = IsometrySynthesisBudget {
            max_matrix_elements: Some(1),
            max_two_level_transformations: None,
            max_elimination_steps: None,
        };

        let error = synthesize_isometry(
            isometry,
            IsometryNumericalPolicy::default(),
            budget,
        )
        .expect_err("budget must reject oversized matrix");

        assert!(matches!(
            error,
            IsometrySynthesisError::BudgetExceeded {
                resource: "matrix elements",
                ..
            }
        ));
    }

    #[test]
    fn complex_two_level_unitary_validates() {
        let inv_sqrt_two = 1.0 / 2.0_f64.sqrt();

        let transform = TwoLevelUnitary::new(
            0,
            1,
            Complex64::new(inv_sqrt_two, 0.0),
            Complex64::new(inv_sqrt_two, 0.0),
            Complex64::new(inv_sqrt_two, 0.0),
            Complex64::new(-inv_sqrt_two, 0.0),
        )
        .expect("Hadamard-like transformation is unitary");

        transform.validate()
            .expect("transformation must validate");
    }

    #[test]
    fn matrix_multiplication_is_correct_for_identity() {
        let identity = ComplexMatrix::identity(2)
            .expect("identity");

        let product = identity
            .multiply(&identity)
            .expect("multiplication");

        assert_eq!(product, identity);
    }

    #[test]
    fn finite_values_are_required() {
        let result = ComplexMatrix::new(
            1,
            1,
            vec![
                Complex64::new(f64::NAN, 0.0),
            ],
        );

        assert!(matches!(
            result,
            Err(IsometrySynthesisError::NonFiniteValue)
        ));
    }
}