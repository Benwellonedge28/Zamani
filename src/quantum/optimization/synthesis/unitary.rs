//! Zamani Quantum Optimization — General Unitary Synthesis
//!
//! Production-grade synthesis planning for arbitrary n-qubit unitary
//! operators.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             ▼
//!                 optimization::synthesis
//!                             │
//!                             ▼
//!                  ┌─────────────────────┐
//!                  │     unitary.rs      │
//!                  │                     │
//!                  │ matrix validation   │
//!                  │ resource checking   │
//!                  │ Givens factorization│
//!                  │ phase extraction    │
//!                  │ two-level factors   │
//!                  └──────────┬──────────┘
//!                             │
//!                             ▼
//!                 UnitarySynthesisPlan
//!                             │
//!                             ▼
//!                    decomposition.rs
//!                             │
//!                             ▼
//!                    canonical Gate list
//!                             │
//!                             ▼
//!                       optimization
//!                             │
//!                             ▼
//!                         routing
//!                             │
//!                             ▼
//!                       scheduling
//! ```
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - arbitrary finite-dimensional unitary-matrix representation;
//! - n-qubit dimension validation;
//! - finite-value validation;
//! - unitarity validation;
//! - explicit numerical tolerance;
//! - explicit resource budgets;
//! - deterministic Givens/two-level factorization;
//! - diagonal phase extraction;
//! - global-phase accounting;
//! - two-level unitary representation;
//! - Gray-code metadata required by later lowering;
//! - synthesis complexity accounting;
//! - deterministic synthesis planning;
//! - optional post-factorization verification;
//! - allocation-aware failure handling;
//! - no unsafe code.
//!
//! This module does NOT own:
//!
//! - canonical circuit mutation;
//! - routing;
//! - physical qubit topology;
//! - scheduling;
//! - hardware APIs;
//! - pulse generation;
//! - backend execution;
//! - measurement;
//! - error correction;
//! - target-specific gate decomposition;
//! - optimization-pass scheduling.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Why Givens factorization?
//!
//! A general unitary can be factored into two-level unitary transformations.
//! Each two-level transformation acts non-trivially on only two computational
//! basis states.
//!
//! This gives a universal and deterministic mathematical synthesis boundary:
//!
//! ```text
//! arbitrary U
//!    │
//!    ▼
//! Givens elimination
//!    │
//!    ▼
//! diagonal phases + two-level unitaries
//!    │
//!    ▼
//! target-aware decomposition
//! ```
//!
//! This is not intended to claim that the Givens representation is always
//! the shortest circuit. It is a complete universal fallback representation.
//!
//! For small or structured unitaries, higher-level synthesis algorithms should
//! be preferred by `planner.rs`, such as:
//!
//! - specialized 1-qubit synthesis;
//! - KAK/two-qubit synthesis;
//! - Clifford synthesis;
//! - Quantum Shannon decomposition;
//! - phase-polynomial synthesis;
//! - target-specific synthesis;
//! - approximate synthesis;
//! - tensor-product decomposition.
//!
//! # Complexity
//!
//! Let:
//!
//! ```text
//! d = 2^n
//! ```
//!
//! be the matrix dimension.
//!
//! Matrix storage requires O(d²) memory.
//!
//! The validation/factorization algorithms require polynomial time in `d`,
//! which is therefore exponential in the number of qubits.
//!
//! A generic unitary has exponentially many independent degrees of freedom,
//! so no implementation can honestly promise polynomial resource usage in
//! `n` for arbitrary dense unitaries.
//!
//! The implementation therefore scales "as far as resources allow" by:
//!
//! - using checked arithmetic;
//! - using caller-controlled optional limits;
//! - refusing impossible allocations before allocation;
//! - using fixed-size complex arithmetic;
//! - avoiding recursion over the number of matrix elements;
//! - avoiding unsafe code;
//! - avoiding hidden global state;
//! - supporting `None` as an explicitly unlimited synthesis budget.
//!
//! # Numerical policy
//!
//! Floating-point arithmetic is used only for numerical matrix data.
//!
//! The implementation:
//!
//! - rejects NaN and infinity;
//! - uses an explicit tolerance;
//! - never silently treats a large residual as zero;
//! - only skips an elimination when the element is within tolerance;
//! - checks the final factorization residual when verification is enabled;
//! - preserves global phase explicitly;
//! - never silently drops relative phase.
//!
//! # Canonical IR rule
//!
//! This file deliberately does not define another `QuantumGate`, circuit,
//! qubit, parameter, or hardware representation.
//!
//! Canonical IR remains:
//!
//! - `crate::quantum::ir::Gate`
//! - `crate::quantum::ir::GateKind`
//! - `crate::quantum::ir::Parameter`
//! - `crate::quantum::ir::QubitId`
//!
//! The types in this file are mathematical synthesis-planning types, not a
//! replacement quantum IR.
//!
//! # Integration
//!
//! `decomposition.rs` consumes [`UnitarySynthesisPlan`] and is responsible for
//! lowering:
//!
//! - `TwoLevelUnitary`;
//! - `BasisPhase`;
//! - global phase metadata;
//!
//! into canonical `Gate` operations.
//!
//! `single_qubit.rs` should be used when a two-level factor is actually a
//! one-qubit operation.
//!
//! `two_qubit.rs` should be used when a two-level factor has a native
//! two-qubit realization available.
//!
//! `targets/` determines which lowering strategy is preferable.
//!
//! `verification/semantic.rs` can independently verify the final circuit.
//!
//! `planner.rs` may choose this module as the universal fallback when no
//! more specialized synthesis algorithm applies.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
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

use super::single_qubit::Complex64;
use crate::quantum::ir::gate::GateKind;
use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// Result
// ============================================================================

/// Result type returned by general unitary synthesis.
pub type UnitarySynthesisResult<T> =
    Result<T, UnitarySynthesisError>;

// ============================================================================
// Constants
// ============================================================================

/// Minimum useful numerical tolerance.
///
/// Values below this threshold are accepted only when they are finite and
/// represent an explicit caller choice. The threshold protects the algorithm
/// from requesting distinctions below ordinary f64 numerical resolution.
pub const MIN_TOLERANCE: f64 = 1.0e-15;

/// Default numerical tolerance.
pub const DEFAULT_TOLERANCE: f64 = 1.0e-12;

/// Default maximum number of qubits for a dense general-unitary synthesis.
///
/// This is a policy default, not a mathematical hard limit. Callers can use
/// `None` in [`UnitarySynthesisLimits`] when they deliberately want resource
/// availability to be the only upper bound.
pub const DEFAULT_MAX_QUBITS: usize = 12;

/// Default maximum number of matrix elements.
///
/// This protects ordinary compiler invocations from accidentally constructing
/// enormous dense matrices.
pub const DEFAULT_MAX_MATRIX_ELEMENTS: usize = 1 << 24;

/// Numerical zero test is deliberately smaller than the normal verification
/// tolerance. It is only used for exact structural zero decisions after a
/// tolerance check has already been performed.
const STRUCTURAL_ZERO: f64 = 1.0e-15;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by general unitary synthesis.
#[derive(Debug, Clone, PartialEq)]
pub enum UnitarySynthesisError {
    /// Matrix has no rows.
    EmptyMatrix,

    /// Matrix is not square.
    NonSquare {
        /// Number of rows.
        rows: usize,

        /// Number of columns.
        columns: usize,
    },

    /// Matrix dimension is not a power of two.
    DimensionNotPowerOfTwo {
        /// Matrix dimension.
        dimension: usize,
    },

    /// Matrix dimension is not representable as an n-qubit operator.
    InvalidQubitCount {
        /// Number of qubits.
        qubits: usize,
    },

    /// A matrix contains a non-finite number.
    NonFiniteMatrix {
        /// Flat matrix index.
        index: usize,
    },

    /// The matrix is not unitary.
    NotUnitary {
        /// Maximum observed residual.
        residual: f64,

        /// Configured tolerance.
        tolerance: f64,
    },

    /// Tolerance is invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        tolerance: f64,
    },

    /// A configured limit is invalid.
    InvalidLimit {
        /// Name of the limit.
        resource: &'static str,
    },

    /// A resource limit would be exceeded.
    ResourceLimitExceeded {
        /// Resource name.
        resource: &'static str,

        /// Configured maximum.
        maximum: usize,

        /// Required amount.
        required: usize,
    },

    /// Checked arithmetic overflow.
    ArithmeticOverflow {
        /// Operation that overflowed.
        operation: &'static str,
    },

    /// A vector allocation could not be reserved.
    AllocationFailed {
        /// Requested additional capacity.
        requested: usize,
    },

    /// Numerical factorization failed.
    FactorizationFailure {
        /// Static explanation.
        message: &'static str,
    },

    /// Final reconstruction does not reproduce the input within tolerance.
    VerificationFailed {
        /// Maximum reconstruction residual.
        residual: f64,

        /// Configured tolerance.
        tolerance: f64,
    },

    /// A generated factor is malformed.
    InvalidTwoLevelFactor {
        /// First basis state.
        first: usize,

        /// Second basis state.
        second: usize,
    },

    /// A lowering request requires a target capability not owned by this
    /// module.
    UnsupportedLowering {
        /// Gate kind that would be required.
        gate: GateKind,
    },

    /// The matrix is too large for a dense exact synthesis strategy.
    DenseRepresentationTooLarge {
        /// Matrix dimension.
        dimension: usize,
    },
}

impl fmt::Display for UnitarySynthesisError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyMatrix => {
                write!(formatter, "unitary matrix must not be empty")
            }

            Self::NonSquare { rows, columns } => {
                write!(
                    formatter,
                    "unitary matrix must be square: {rows} rows, {columns} columns"
                )
            }

            Self::DimensionNotPowerOfTwo { dimension } => {
                write!(
                    formatter,
                    "unitary matrix dimension {dimension} is not a power of two"
                )
            }

            Self::InvalidQubitCount { qubits } => {
                write!(
                    formatter,
                    "invalid qubit count for dense unitary synthesis: {qubits}"
                )
            }

            Self::NonFiniteMatrix { index } => {
                write!(
                    formatter,
                    "unitary matrix contains a non-finite value at flat index {index}"
                )
            }

            Self::NotUnitary {
                residual,
                tolerance,
            } => {
                write!(
                    formatter,
                    "matrix is not unitary: residual={residual:e}, tolerance={tolerance:e}"
                )
            }

            Self::InvalidTolerance { tolerance } => {
                write!(
                    formatter,
                    "invalid unitary-synthesis tolerance: {tolerance:e}"
                )
            }

            Self::InvalidLimit { resource } => {
                write!(
                    formatter,
                    "invalid unitary-synthesis limit for {resource}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                maximum,
                required,
            } => {
                write!(
                    formatter,
                    "unitary synthesis exceeds {resource}: maximum={maximum}, required={required}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {operation}"
                )
            }

            Self::AllocationFailed { requested } => {
                write!(
                    formatter,
                    "unable to reserve {requested} additional matrix/synthesis elements"
                )
            }

            Self::FactorizationFailure { message } => {
                write!(
                    formatter,
                    "unitary factorization failed: {message}"
                )
            }

            Self::VerificationFailed {
                residual,
                tolerance,
            } => {
                write!(
                    formatter,
                    "unitary synthesis verification failed: residual={residual:e}, tolerance={tolerance:e}"
                )
            }

            Self::InvalidTwoLevelFactor {
                first,
                second,
            } => {
                write!(
                    formatter,
                    "invalid two-level factor acting on basis states {first} and {second}"
                )
            }

            Self::UnsupportedLowering { gate } => {
                write!(
                    formatter,
                    "unitary factor lowering requires unsupported gate {gate:?}"
                )
            }

            Self::DenseRepresentationTooLarge { dimension } => {
                write!(
                    formatter,
                    "dense unitary representation is too large for dimension {dimension}"
                )
            }
        }
    }
}

impl Error for UnitarySynthesisError {}

// ============================================================================
// Limits
// ============================================================================

/// Explicit resource policy for dense unitary synthesis.
///
/// Every field is optional so the caller can choose between:
///
/// - bounded compilation;
/// - deliberately unlimited synthesis;
/// - a mixture of bounded and unlimited resources.
///
/// `None` means "no limit imposed by this synthesis layer".
///
/// The canonical IR and global compiler limits remain authoritative even when
/// all fields here are `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitarySynthesisLimits {
    /// Maximum number of qubits represented by one dense matrix.
    pub max_qubits: Option<usize>,

    /// Maximum number of scalar matrix elements.
    pub max_matrix_elements: Option<usize>,

    /// Maximum number of two-level factors.
    pub max_two_level_factors: Option<usize>,

    /// Maximum number of explicit basis phases.
    pub max_basis_phases: Option<usize>,

    /// Maximum number of temporary working elements.
    pub max_working_elements: Option<usize>,
}

impl Default for UnitarySynthesisLimits {
    fn default() -> Self {
        Self {
            max_qubits: Some(DEFAULT_MAX_QUBITS),
            max_matrix_elements: Some(
                DEFAULT_MAX_MATRIX_ELEMENTS,
            ),
            max_two_level_factors: None,
            max_basis_phases: None,
            max_working_elements: None,
        }
    }
}

impl UnitarySynthesisLimits {
    /// Unlimited limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_qubits: None,
            max_matrix_elements: None,
            max_two_level_factors: None,
            max_basis_phases: None,
            max_working_elements: None,
        }
    }

    /// Validates the limit configuration.
    pub fn validate(
        self,
    ) -> UnitarySynthesisResult<()> {
        if matches!(self.max_qubits, Some(0)) {
            return Err(
                UnitarySynthesisError::InvalidLimit {
                    resource: "max_qubits",
                },
            );
        }

        if matches!(
            self.max_matrix_elements,
            Some(0)
        ) {
            return Err(
                UnitarySynthesisError::InvalidLimit {
                    resource: "max_matrix_elements",
                },
            );
        }

        if matches!(
            self.max_two_level_factors,
            Some(0)
        ) {
            return Err(
                UnitarySynthesisError::InvalidLimit {
                    resource: "max_two_level_factors",
                },
            );
        }

        if matches!(
            self.max_basis_phases,
            Some(0)
        ) {
            return Err(
                UnitarySynthesisError::InvalidLimit {
                    resource: "max_basis_phases",
                },
            );
        }

        if matches!(
            self.max_working_elements,
            Some(0)
        ) {
            return Err(
                UnitarySynthesisError::InvalidLimit {
                    resource: "max_working_elements",
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for general unitary synthesis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitarySynthesisConfig {
    /// Numerical tolerance.
    pub tolerance: f64,

    /// Resource limits.
    pub limits: UnitarySynthesisLimits,

    /// Whether global phase may be removed from the executable circuit.
    ///
    /// The removed phase remains recorded in [`UnitarySynthesisPlan`].
    pub allow_global_phase: bool,

    /// Whether the input matrix should be validated as unitary.
    ///
    /// This should normally remain `true`.
    ///
    /// Setting it to `false` is intended only for trusted internal pipelines
    /// where validation has already been performed by another layer.
    pub validate_input: bool,

    /// Whether the generated factorization should be independently verified.
    pub verify_output: bool,
}

impl Default for UnitarySynthesisConfig {
    fn default() -> Self {
        Self {
            tolerance: DEFAULT_TOLERANCE,
            limits: UnitarySynthesisLimits::default(),
            allow_global_phase: true,
            validate_input: true,
            verify_output: true,
        }
    }
}

impl UnitarySynthesisConfig {
    /// Creates a configuration with no limits imposed by this module.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            tolerance: DEFAULT_TOLERANCE,
            limits: UnitarySynthesisLimits::unlimited(),
            allow_global_phase: true,
            validate_input: true,
            verify_output: true,
        }
    }

    /// Validates the configuration.
    pub fn validate(
        self,
    ) -> UnitarySynthesisResult<()> {
        validate_tolerance(self.tolerance)?;
        self.limits.validate()
    }
}

// ============================================================================
// Dense matrix
// ============================================================================

/// Dense square complex matrix representing a logical unitary.
///
/// Storage is row-major:
///
/// ```text
/// index(row, column) = row * dimension + column
/// ```
///
/// The type is intentionally independent of the canonical circuit IR.
/// It represents mathematical input to synthesis, not a circuit operation.
#[derive(Debug, Clone, PartialEq)]
pub struct UnitaryMatrix {
    dimension: usize,
    data: Vec<Complex64>,
}

impl UnitaryMatrix {
    /// Creates an identity matrix of the requested dimension.
    ///
    /// This constructor is intended for trusted internal callers. For
    /// externally supplied dimensions, use [`Self::try_identity`].
    pub fn identity(
        dimension: usize,
    ) -> UnitarySynthesisResult<Self> {
        Self::try_identity(dimension, None)
    }

    /// Creates an identity matrix with an explicit element limit.
    pub fn try_identity(
        dimension: usize,
        maximum_elements: Option<usize>,
    ) -> UnitarySynthesisResult<Self> {
        if dimension == 0 {
            return Err(
                UnitarySynthesisError::EmptyMatrix,
            );
        }

        let elements = checked_square(dimension)?;

        if let Some(maximum) = maximum_elements {
            if elements > maximum {
                return Err(
                    UnitarySynthesisError::ResourceLimitExceeded {
                        resource: "matrix elements",
                        maximum,
                        required: elements,
                    },
                );
            }
        }

        let mut data = Vec::new();

        data.try_reserve_exact(elements)
            .map_err(|_| {
                UnitarySynthesisError::AllocationFailed {
                    requested: elements,
                }
            })?;

        data.resize(
            elements,
            Complex64::zero(),
        );

        for index in 0..dimension {
            data[index * dimension + index] =
                Complex64::one();
        }

        Ok(Self {
            dimension,
            data,
        })
    }

    /// Creates a matrix from row-major data.
    pub fn from_rows(
        rows: Vec<Vec<Complex64>>,
    ) -> UnitarySynthesisResult<Self> {
        if rows.is_empty() {
            return Err(
                UnitarySynthesisError::EmptyMatrix,
            );
        }

        let dimension = rows.len();

        for row in &rows {
            if row.len() != dimension {
                return Err(
                    UnitarySynthesisError::NonSquare {
                        rows: dimension,
                        columns: row.len(),
                    },
                );
            }
        }

        let elements = checked_square(dimension)?;

        let mut data = Vec::new();

        data.try_reserve_exact(elements)
            .map_err(|_| {
                UnitarySynthesisError::AllocationFailed {
                    requested: elements,
                }
            })?;

        for row in rows {
            data.extend(row);
        }

        let matrix = Self {
            dimension,
            data,
        };

        matrix.validate_finite()?;

        Ok(matrix)
    }

    /// Creates a matrix from row-major flat data.
    pub fn from_flat(
        dimension: usize,
        data: Vec<Complex64>,
    ) -> UnitarySynthesisResult<Self> {
        if dimension == 0 {
            return Err(
                UnitarySynthesisError::EmptyMatrix,
            );
        }

        let expected = checked_square(dimension)?;

        if data.len() != expected {
            return Err(
                UnitarySynthesisError::NonSquare {
                    rows: dimension,
                    columns: data.len()
                        / dimension.max(1),
                },
            );
        }

        let matrix = Self {
            dimension,
            data,
        };

        matrix.validate_finite()?;

        Ok(matrix)
    }

    /// Returns the matrix dimension.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.dimension
    }

    /// Returns the number of qubits represented by this matrix.
    pub fn qubit_count(
        &self,
    ) -> UnitarySynthesisResult<usize> {
        qubits_for_dimension(self.dimension)
    }

    /// Returns the number of scalar elements.
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.data.len()
    }

    /// Returns an immutable row-major view.
    #[must_use]
    pub fn as_slice(&self) -> &[Complex64] {
        &self.data
    }

    /// Returns one matrix element.
    pub fn get(
        &self,
        row: usize,
        column: usize,
    ) -> Option<Complex64> {
        if row >= self.dimension
            || column >= self.dimension
        {
            return None;
        }

        Some(
            self.data[row * self.dimension + column],
        )
    }

    /// Returns one matrix element without bounds checks performed by this API.
    ///
    /// The caller must provide valid coordinates.
    #[must_use]
    pub fn get_unchecked_by_contract(
        &self,
        row: usize,
        column: usize,
    ) -> Complex64 {
        self.data[row * self.dimension + column]
    }

    /// Sets one matrix element.
    ///
    /// This method is intentionally internal in spirit. Synthesis algorithms
    /// use it while transforming a private working copy.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: Complex64,
    ) -> UnitarySynthesisResult<()> {
        if row >= self.dimension
            || column >= self.dimension
        {
            return Err(
                UnitarySynthesisError::FactorizationFailure {
                    message: "matrix coordinate outside dimension",
                },
            );
        }

        if !value.is_finite() {
            return Err(
                UnitarySynthesisError::NonFiniteMatrix {
                    index: row * self.dimension
                        + column,
                },
            );
        }

        self.data[row * self.dimension + column] =
            value;

        Ok(())
    }

    /// Returns the conjugate transpose.
    pub fn adjoint(&self) -> Self {
        let mut data =
            vec![Complex64::zero(); self.data.len()];

        for row in 0..self.dimension {
            for column in 0..self.dimension {
                data[column * self.dimension + row] =
                    self.data[row * self.dimension + column]
                        .conjugate();
            }
        }

        Self {
            dimension: self.dimension,
            data,
        }
    }

    /// Matrix multiplication.
    pub fn multiply(
        &self,
        rhs: &Self,
    ) -> UnitarySynthesisResult<Self> {
        if self.dimension != rhs.dimension {
            return Err(
                UnitarySynthesisError::FactorizationFailure {
                    message: "matrix dimensions do not match",
                },
            );
        }

        let dimension = self.dimension;
        let elements = checked_square(dimension)?;

        let mut data = Vec::new();

        data.try_reserve_exact(elements)
            .map_err(|_| {
                UnitarySynthesisError::AllocationFailed {
                    requested: elements,
                }
            })?;

        data.resize(
            elements,
            Complex64::zero(),
        );

        for row in 0..dimension {
            for column in 0..dimension {
                let mut value =
                    Complex64::zero();

                for k in 0..dimension {
                    value = value
                        + self.get_unchecked_by_contract(
                            row,
                            k,
                        ) * rhs.get_unchecked_by_contract(
                            k,
                            column,
                        );
                }

                data[row * dimension + column] =
                    value;
            }
        }

        Ok(Self {
            dimension,
            data,
        })
    }

    /// Returns the maximum absolute element-wise difference.
    #[must_use]
    pub fn max_difference(
        &self,
        rhs: &Self,
    ) -> f64 {
        if self.dimension != rhs.dimension {
            return f64::INFINITY;
        }

        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(lhs, rhs)| {
                (*lhs - *rhs).norm()
            })
            .fold(0.0, f64::max)
    }

    /// Returns the Frobenius norm.
    #[must_use]
    pub fn frobenius_norm(&self) -> f64 {
        self.data
            .iter()
            .map(|value| value.norm_squared())
            .sum::<f64>()
            .sqrt()
    }

    /// Validates finite matrix entries.
    pub fn validate_finite(
        &self,
    ) -> UnitarySynthesisResult<()> {
        for (index, value) in
            self.data.iter().enumerate()
        {
            if !value.is_finite() {
                return Err(
                    UnitarySynthesisError::NonFiniteMatrix {
                        index,
                    },
                );
            }
        }

        Ok(())
    }

    /// Computes the maximum residual of:
    ///
    /// ```text
    /// U† U - I
    /// ```
    pub fn unitarity_residual(
        &self,
    ) -> UnitarySynthesisResult<f64> {
        let adjoint = self.adjoint();

        let product =
            adjoint.multiply(self)?;

        let identity =
            UnitaryMatrix::identity(
                self.dimension,
            )?;

        Ok(product.max_difference(
            &identity,
        ))
    }

    /// Validates that the matrix is unitary.
    pub fn validate_unitary(
        &self,
        tolerance: f64,
    ) -> UnitarySynthesisResult<()> {
        validate_tolerance(tolerance)?;
        self.validate_finite()?;

        let residual =
            self.unitarity_residual()?;

        if residual > tolerance {
            return Err(
                UnitarySynthesisError::NotUnitary {
                    residual,
                    tolerance,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Two-level unitary
// ============================================================================

/// A two-level unitary acting only on two computational basis states.
///
/// The full matrix is identity everywhere except the selected basis-state
/// pair.
///
/// ```text
///          first      second
/// first    a          b
/// second   c          d
/// ```
///
/// This is the mathematical primitive produced by the universal factorizer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoLevelUnitary {
    /// First computational-basis index.
    pub first: usize,

    /// Second computational-basis index.
    pub second: usize,

    /// `(first, first)` matrix element.
    pub a: Complex64,

    /// `(first, second)` matrix element.
    pub b: Complex64,

    /// `(second, first)` matrix element.
    pub c: Complex64,

    /// `(second, second)` matrix element.
    pub d: Complex64,
}

impl TwoLevelUnitary {
    /// Creates a two-level unitary.
    pub fn new(
        first: usize,
        second: usize,
        a: Complex64,
        b: Complex64,
        c: Complex64,
        d: Complex64,
    ) -> UnitarySynthesisResult<Self> {
        if first == second {
            return Err(
                UnitarySynthesisError::InvalidTwoLevelFactor {
                    first,
                    second,
                },
            );
        }

        if !a.is_finite()
            || !b.is_finite()
            || !c.is_finite()
            || !d.is_finite()
        {
            return Err(
                UnitarySynthesisError::FactorizationFailure {
                    message: "two-level factor contains a non-finite value",
                },
            );
        }

        Ok(Self {
            first,
            second,
            a,
            b,
            c,
            d,
        })
    }

    /// Returns the 2x2 block as a fixed-size array in row-major order.
    #[must_use]
    pub const fn block(
        self,
    ) -> [Complex64; 4] {
        [
            self.a,
            self.b,
            self.c,
            self.d,
        ]
    }

    /// Returns the conjugate transpose of this factor.
    #[must_use]
    pub const fn adjoint(
        self,
    ) -> Self {
        Self {
            first: self.first,
            second: self.second,
            a: self.a.conjugate(),
            b: self.c.conjugate(),
            c: self.b.conjugate(),
            d: self.d.conjugate(),
        }
    }

    /// Returns whether the selected basis states differ in exactly one bit.
    #[must_use]
    pub fn differs_by_one_bit(
        self,
    ) -> bool {
        let xor =
            self.first ^ self.second;

        xor != 0 && xor.count_ones() == 1
    }

    /// Returns the target qubit when the two states differ by one bit.
    pub fn differing_qubit(
        self,
    ) -> Option<usize> {
        if !self.differs_by_one_bit() {
            return None;
        }

        Some(
            self.first
                .wrapping_xor(self.second)
                .trailing_zeros() as usize,
        )
    }

    /// Returns the control-bit positions on which the two basis states agree
    /// or differ according to the supplied lowering convention.
    ///
    /// The returned vector contains all qubits except the differing target
    /// qubit. The order is ascending and deterministic.
    pub fn control_qubits(
        self,
        num_qubits: usize,
    ) -> UnitarySynthesisResult<Vec<usize>> {
        if num_qubits >= usize::BITS as usize {
            return Err(
                UnitarySynthesisError::InvalidQubitCount {
                    qubits: num_qubits,
                },
            );
        }

        let dimension =
            checked_power_of_two(num_qubits)?;

        if self.first >= dimension
            || self.second >= dimension
        {
            return Err(
                UnitarySynthesisError::InvalidTwoLevelFactor {
                    first: self.first,
                    second: self.second,
                },
            );
        }

        let target =
            self.differing_qubit();

        let mut controls =
            Vec::with_capacity(num_qubits);

        for bit in 0..num_qubits {
            if Some(bit) != target {
                controls.push(bit);
            }
        }

        Ok(controls)
    }

    /// Returns the Hamming distance between the selected basis states.
    #[must_use]
    pub fn hamming_distance(
        self,
    ) -> u32 {
        (self.first ^ self.second)
            .count_ones()
    }

    /// Returns a deterministic Gray-code path connecting the two basis states.
    ///
    /// The returned sequence starts with `first` and ends with `second`.
    /// Consecutive entries differ in exactly one bit.
    ///
    /// This path is metadata for later lowering. It does not mutate the
    /// canonical circuit and does not itself imply a particular hardware
    /// topology.
    pub fn gray_path(
        self,
    ) -> UnitarySynthesisResult<Vec<usize>> {
        let xor =
            self.first ^ self.second;

        let distance =
            xor.count_ones() as usize;

        let mut path =
            Vec::with_capacity(distance + 1);

        path.push(self.first);

        let mut current =
            self.first;

        for bit in 0..usize::BITS {
            if ((xor >> bit) & 1) != 0 {
                current ^= 1usize << bit;
                path.push(current);
            }
        }

        if path.last().copied()
            != Some(self.second)
        {
            return Err(
                UnitarySynthesisError::FactorizationFailure {
                    message: "failed to construct deterministic Gray path",
                },
            );
        }

        Ok(path)
    }
}

// ============================================================================
// Basis phase
// ============================================================================

/// A phase applied to one computational basis state.
///
/// This is a synthesis-planning primitive, not a canonical IR operation.
///
/// `decomposition.rs` is responsible for converting a collection of basis
/// phases into a target-appropriate diagonal/multiplexed circuit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BasisPhase {
    /// Computational-basis state receiving the phase.
    pub basis_state: usize,

    /// Phase in radians.
    pub phase: f64,
}

impl BasisPhase {
    /// Creates a validated basis phase.
    pub fn new(
        basis_state: usize,
        phase: f64,
    ) -> UnitarySynthesisResult<Self> {
        if !phase.is_finite() {
            return Err(
                UnitarySynthesisError::FactorizationFailure {
                    message: "basis phase is not finite",
                },
            );
        }

        Ok(Self {
            basis_state,
            phase: normalize_angle(phase),
        })
    }
}

// ============================================================================
// Global phase
// ============================================================================

/// Global phase metadata.
///
/// A global phase is physically irrelevant for ordinary closed-system quantum
/// evolution but must not be silently forgotten by a compiler.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalPhase {
    /// Phase in radians.
    pub radians: f64,
}

impl GlobalPhase {
    /// Creates a global phase.
    pub fn new(
        radians: f64,
    ) -> UnitarySynthesisResult<Self> {
        if !radians.is_finite() {
            return Err(
                UnitarySynthesisError::FactorizationFailure {
                    message: "global phase is not finite",
                },
            );
        }

        Ok(Self {
            radians: normalize_angle(radians),
        })
    }
}

// ============================================================================
// Plan
// ============================================================================

/// Statistics for a general unitary synthesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitarySynthesisStatistics {
    /// Number of qubits.
    pub qubits: usize,

    /// Matrix dimension.
    pub dimension: usize,

    /// Number of matrix elements.
    pub matrix_elements: usize,

    /// Number of two-level factors.
    pub two_level_factors: usize,

    /// Number of explicit basis phases.
    pub basis_phases: usize,

    /// Whether a non-zero global phase was extracted.
    pub has_global_phase: bool,

    /// Number of elimination steps skipped because the element was already
    /// numerically zero.
    pub skipped_zero_eliminations: usize,

    /// Number of actual Givens eliminations.
    pub performed_eliminations: usize,
}

impl UnitarySynthesisStatistics {
    /// Returns the number of mathematical synthesis factors excluding the
    /// diagonal phase representation.
    #[must_use]
    pub const fn factor_count(
        self,
    ) -> usize {
        self.two_level_factors
    }
}

/// Complete mathematical synthesis plan.
///
/// The plan is deliberately immutable after construction.
///
/// The factor ordering is important:
///
/// ```text
/// U = G1† G2† ... Gm† D
/// ```
///
/// where the stored `two_level_factors` are `[G1†, G2†, ..., Gm†]`.
///
/// Therefore a circuit lowerer that applies gates left-to-right should emit:
///
/// ```text
/// D
/// Gm†
/// ...
/// G2†
/// G1†
/// ```
///
/// The plan itself does not impose this circuit-level lowering order on
/// `decomposition.rs`; it exposes the exact mathematical relationship.
#[derive(Debug, Clone, PartialEq)]
pub struct UnitarySynthesisPlan {
    /// Original matrix dimension.
    pub dimension: usize,

    /// Number of logical qubits.
    pub qubits: usize,

    /// Two-level factors in mathematical product order.
    pub two_level_factors: Vec<TwoLevelUnitary>,

    /// Relative diagonal basis phases.
    ///
    /// When `allow_global_phase` is true, the first diagonal phase is absorbed
    /// into `global_phase`, so this vector normally contains at most `d - 1`
    /// entries.
    pub basis_phases: Vec<BasisPhase>,

    /// Extracted global phase.
    pub global_phase: GlobalPhase,

    /// Synthesis statistics.
    pub statistics: UnitarySynthesisStatistics,
}

impl UnitarySynthesisPlan {
    /// Returns whether the plan is mathematically the identity up to global
    /// phase.
    #[must_use]
    pub fn is_identity_up_to_global_phase(
        &self,
    ) -> bool {
        self.two_level_factors.is_empty()
            && self
                .basis_phases
                .iter()
                .all(|phase| {
                    phase.phase.abs()
                        <= DEFAULT_TOLERANCE
                })
    }

    /// Returns the number of mathematical synthesis factors.
    #[must_use]
    pub fn factor_count(
        &self,
    ) -> usize {
        self.two_level_factors.len()
    }

    /// Returns the number of executable lowering primitives before a target
    /// decomposition is selected.
    #[must_use]
    pub fn primitive_count(
        &self,
    ) -> usize {
        self.two_level_factors.len()
            + self.basis_phases.len()
    }

    /// Returns the two-level factors in execution order.
    ///
    /// The diagonal phase representation must be emitted before these factors.
    pub fn execution_two_level_factors(
        &self,
    ) -> impl DoubleEndedIterator<
        Item = &TwoLevelUnitary,
    > {
        self.two_level_factors.iter().rev()
    }

    /// Reconstructs the matrix represented by the factorization.
    ///
    /// This method includes the extracted global phase.
    ///
    /// It is intentionally available for independent verification and tests.
    pub fn reconstruct(
        &self,
    ) -> UnitarySynthesisResult<UnitaryMatrix> {
        let dimension = self.dimension;

        let mut result =
            UnitaryMatrix::identity(
                dimension,
            )?;

        // Construct the diagonal D.
        //
        // Start with the global phase and then apply the stored relative
        // basis-state phases.
        let global =
            self.global_phase.radians;

        let mut diagonal =
            vec![
                Complex64::new(
                    global.cos(),
                    global.sin(),
                );
                dimension
            ];

        for phase in &self.basis_phases {
            if phase.basis_state >= dimension {
                return Err(
                    UnitarySynthesisError::FactorizationFailure {
                        message: "basis phase outside plan dimension",
                    },
                );
            }

            let value = Complex64::new(
                phase.phase.cos(),
                phase.phase.sin(),
            );

            diagonal[phase.basis_state] =
                Complex64::new(
                    global.cos(),
                    global.sin(),
                ) * value;
        }

        // If global phase is allowed, basis state zero carries only the global
        // phase. If it was not allowed, basis state zero is represented in the
        // same diagonal vector.
        for index in 0..dimension {
            result.set(
                index,
                index,
                diagonal[index],
            )?;
        }

        // Mathematical relation:
        //
        // U = G1† G2† ... Gm† D
        //
        // Starting from D, left-multiply by Gm†, ..., G1†.
        for factor in
            self.two_level_factors.iter().rev()
        {
            apply_two_level_left(
                &mut result,
                *factor,
            )?;
        }

        Ok(result)
    }
}

// ============================================================================
// Public synthesis entry points
// ============================================================================

/// Synthesizes a general dense n-qubit unitary using the default configuration.
pub fn synthesize_unitary(
    matrix: &UnitaryMatrix,
) -> UnitarySynthesisResult<UnitarySynthesisPlan> {
    synthesize_unitary_with_config(
        matrix,
        UnitarySynthesisConfig::default(),
    )
}

/// Synthesizes a general dense n-qubit unitary with an explicit configuration.
pub fn synthesize_unitary_with_config(
    matrix: &UnitaryMatrix,
    config: UnitarySynthesisConfig,
) -> UnitarySynthesisResult<UnitarySynthesisPlan> {
    config.validate()?;

    validate_dimension_against_limits(
        matrix,
        config.limits,
    )?;

    if config.validate_input {
        matrix.validate_unitary(
            config.tolerance,
        )?;
    } else {
        matrix.validate_finite()?;
    }

    let dimension =
        matrix.dimension();

    let qubits =
        qubits_for_dimension(dimension)?;

    let factor_count =
        maximum_two_level_factor_count(
            dimension,
        )?;

    check_optional_limit(
        "two-level factors",
        factor_count,
        config
            .limits
            .max_two_level_factors,
    )?;

    let maximum_phases =
        dimension;

    check_optional_limit(
        "basis phases",
        maximum_phases,
        config.limits.max_basis_phases,
    )?;

    let working_elements =
        matrix.element_count();

    check_optional_limit(
        "working elements",
        working_elements,
        config
            .limits
            .max_working_elements,
    )?;

    let mut working =
        matrix.clone();

    let mut factors =
        Vec::new();

    factors
        .try_reserve_exact(factor_count)
        .map_err(|_| {
            UnitarySynthesisError::AllocationFailed {
                requested: factor_count,
            }
        })?;

    let mut skipped =
        0usize;

    let mut performed =
        0usize;

    /*
     * Givens/QR-style elimination.
     *
     * For each column, eliminate entries below the diagonal from bottom to
     * top. The left multiplication matrix
     *
     *     G = [ c   s ]
     *         [-s*  c*]
     *
     * is selected so that G * [a, b]^T = [r, 0]^T.
     *
     * We store G† because:
     *
     *     Gm ... G2 G1 U = D
     *
     * implies:
     *
     *     U = G1† G2† ... Gm† D.
     */
    if dimension >= 2 {
        for column in 0..(dimension - 1) {
            for row in
                ((column + 1)..dimension).rev()
            {
                let a =
                    working
                        .get_unchecked_by_contract(
                            column,
                            column,
                        );

                let b =
                    working
                        .get_unchecked_by_contract(
                            row,
                            column,
                        );

                if b.norm()
                    <= config.tolerance
                {
                    skipped = skipped
                        .checked_add(1)
                        .ok_or(
                            UnitarySynthesisError::ArithmeticOverflow {
                                operation: "skipped elimination count",
                            },
                        )?;

                    continue;
                }

                let a_norm =
                    a.norm();

                let b_norm =
                    b.norm();

                let radius =
                    a_norm
                        .hypot(b_norm);

                if !radius.is_finite()
                    || radius
                        <= STRUCTURAL_ZERO
                {
                    return Err(
                        UnitarySynthesisError::FactorizationFailure {
                            message: "Givens normalization radius is invalid",
                        },
                    );
                }

                let c =
                    a.conjugate()
                        / radius;

                let s =
                    b.conjugate()
                        / radius;

                // G = [[c, s], [-s*, c*]]
                //
                // Its adjoint, which is stored in the plan, is:
                //
                // G† = [[c*, -s], [s*, c]]
                let factor =
                    TwoLevelUnitary::new(
                        column,
                        row,
                        c.conjugate(),
                        -s,
                        s.conjugate(),
                        c,
                    )?;

                apply_givens_left(
                    &mut working,
                    column,
                    row,
                    c,
                    s,
                )?;

                factors.push(factor);

                performed = performed
                    .checked_add(1)
                    .ok_or(
                        UnitarySynthesisError::ArithmeticOverflow {
                            operation: "performed elimination count",
                        },
                    )?;
            }
        }
    }

    /*
     * The transformed unitary should now be diagonal.
     *
     * For a mathematically exact unitary, a triangular unitary matrix is
     * diagonal. Floating-point error means we explicitly check the residual.
     */
    let diagonal_residual =
        diagonal_residual(&working);

    if diagonal_residual
        > config.tolerance * 8.0
    {
        return Err(
            UnitarySynthesisError::FactorizationFailure {
                message: "Givens elimination did not produce a sufficiently diagonal matrix",
            },
        );
    }

    let mut diagonal_phases =
        Vec::new();

    diagonal_phases
        .try_reserve_exact(dimension)
        .map_err(|_| {
            UnitarySynthesisError::AllocationFailed {
                requested: dimension,
            }
        })?;

    for index in 0..dimension {
        let value =
            working
                .get_unchecked_by_contract(
                    index,
                    index,
                );

        let magnitude =
            value.norm();

        if (magnitude - 1.0).abs()
            > config.tolerance * 8.0
        {
            return Err(
                UnitarySynthesisError::FactorizationFailure {
                    message: "diagonal factor has non-unit magnitude",
                },
            );
        }

        let phase =
            value.im.atan2(value.re);

        diagonal_phases.push(
            phase,
        );
    }

    let global_phase =
        if config.allow_global_phase {
            diagonal_phases
                .first()
                .copied()
                .unwrap_or(0.0)
        } else {
            0.0
        };

    let mut basis_phases =
        Vec::new();

    let phase_start =
        if config.allow_global_phase {
            1
        } else {
            0
        };

    let mut phase_reserve =
        dimension;

    if config.allow_global_phase {
        phase_reserve =
            dimension.saturating_sub(1);
    }

    basis_phases
        .try_reserve_exact(phase_reserve)
        .map_err(|_| {
            UnitarySynthesisError::AllocationFailed {
                requested: phase_reserve,
            }
        })?;

    for index in phase_start..dimension {
        let phase =
            if config.allow_global_phase {
                normalize_angle(
                    diagonal_phases[index]
                        - global_phase,
                )
            } else {
                normalize_angle(
                    diagonal_phases[index],
                )
            };

        if phase.abs()
            > config.tolerance
        {
            basis_phases.push(
                BasisPhase::new(
                    index,
                    phase,
                )?,
            );
        }
    }

    if !config.allow_global_phase
        && dimension > 0
    {
        let phase =
            normalize_angle(
                diagonal_phases[0],
            );

        if phase.abs()
            > config.tolerance
        {
            basis_phases.insert(
                0,
                BasisPhase::new(
                    0,
                    phase,
                )?,
            );
        }
    }

    let statistics =
        UnitarySynthesisStatistics {
            qubits,
            dimension,
            matrix_elements:
                matrix.element_count(),
            two_level_factors:
                factors.len(),
            basis_phases:
                basis_phases.len(),
            has_global_phase:
                global_phase.abs()
                    > config.tolerance,
            skipped_zero_eliminations:
                skipped,
            performed_eliminations:
                performed,
        };

    let plan =
        UnitarySynthesisPlan {
            dimension,
            qubits,
            two_level_factors:
                factors,
            basis_phases,
            global_phase:
                GlobalPhase::new(
                    global_phase,
                )?,
            statistics,
        };

    if config.verify_output {
        let reconstructed =
            plan.reconstruct()?;

        let residual =
            if config.allow_global_phase {
                compare_up_to_global_phase(
                    matrix,
                    &reconstructed,
                )?
            } else {
                matrix.max_difference(
                    &reconstructed,
                )
            };

        if residual
            > config.tolerance * 32.0
        {
            return Err(
                UnitarySynthesisError::VerificationFailed {
                    residual,
                    tolerance:
                        config.tolerance,
                },
            );
        }
    }

    Ok(plan)
}

// ============================================================================
// Validation and limits
// ============================================================================

fn validate_tolerance(
    tolerance: f64,
) -> UnitarySynthesisResult<()> {
    if !tolerance.is_finite()
        || tolerance < MIN_TOLERANCE
    {
        return Err(
            UnitarySynthesisError::InvalidTolerance {
                tolerance,
            },
        );
    }

    Ok(())
}

fn checked_square(
    value: usize,
) -> UnitarySynthesisResult<usize> {
    value.checked_mul(value).ok_or(
        UnitarySynthesisError::ArithmeticOverflow {
            operation: "matrix dimension squared",
        },
    )
}

fn checked_power_of_two(
    qubits: usize,
) -> UnitarySynthesisResult<usize> {
    if qubits >= usize::BITS as usize {
        return Err(
            UnitarySynthesisError::ArithmeticOverflow {
                operation: "2^qubits",
            },
        );
    }

    1usize.checked_shl(
        qubits as u32,
    )
    .ok_or(
        UnitarySynthesisError::ArithmeticOverflow {
            operation: "2^qubits",
        },
    )
}

fn qubits_for_dimension(
    dimension: usize,
) -> UnitarySynthesisResult<usize> {
    if dimension == 0 {
        return Err(
            UnitarySynthesisError::EmptyMatrix,
        );
    }

    if !dimension.is_power_of_two() {
        return Err(
            UnitarySynthesisError::DimensionNotPowerOfTwo {
                dimension,
            },
        );
    }

    Ok(
        dimension.trailing_zeros()
            as usize,
    )
}

fn maximum_two_level_factor_count(
    dimension: usize,
) -> UnitarySynthesisResult<usize> {
    let first =
        dimension
            .checked_mul(
                dimension
                    .checked_sub(1)
                    .ok_or(
                        UnitarySynthesisError::ArithmeticOverflow {
                            operation:
                                "dimension minus one",
                        },
                    )?,
            )
            .ok_or(
                UnitarySynthesisError::ArithmeticOverflow {
                    operation:
                        "two-level factor count",
                },
            )?;

    Ok(first / 2)
}

fn check_optional_limit(
    resource: &'static str,
    required: usize,
    maximum: Option<usize>,
) -> UnitarySynthesisResult<()> {
    if let Some(maximum) = maximum {
        if required > maximum {
            return Err(
                UnitarySynthesisError::ResourceLimitExceeded {
                    resource,
                    maximum,
                    required,
                },
            );
        }
    }

    Ok(())
}

fn validate_dimension_against_limits(
    matrix: &UnitaryMatrix,
    limits: UnitarySynthesisLimits,
) -> UnitarySynthesisResult<()> {
    let dimension =
        matrix.dimension();

    let qubits =
        qubits_for_dimension(
            dimension,
        )?;

    check_optional_limit(
        "qubits",
        qubits,
        limits.max_qubits,
    )?;

    check_optional_limit(
        "matrix elements",
        matrix.element_count(),
        limits.max_matrix_elements,
    )?;

    Ok(())
}

// ============================================================================
// Numerical helpers
// ============================================================================

/// Normalizes an angle into `[-π, π)`.
#[must_use]
pub fn normalize_angle(
    angle: f64,
) -> f64 {
    if !angle.is_finite() {
        return angle;
    }

    let tau =
        std::f64::consts::TAU;

    let pi =
        std::f64::consts::PI;

    let mut value =
        angle.rem_euclid(tau);

    if value >= pi {
        value -= tau;
    }

    if value.abs()
        <= STRUCTURAL_ZERO
    {
        0.0
    } else {
        value
    }
}

/// Returns the maximum magnitude of all off-diagonal elements.
fn diagonal_residual(
    matrix: &UnitaryMatrix,
) -> f64 {
    let mut residual =
        0.0;

    for row in 0..matrix.dimension {
        for column in 0..matrix.dimension {
            if row == column {
                continue;
            }

            residual = residual.max(
                matrix
                    .get_unchecked_by_contract(
                        row,
                        column,
                    )
                    .norm(),
            );
        }
    }

    residual
}

/// Compares two matrices while allowing global phase.
fn compare_up_to_global_phase(
    lhs: &UnitaryMatrix,
    rhs: &UnitaryMatrix,
) -> UnitarySynthesisResult<f64> {
    if lhs.dimension
        != rhs.dimension
    {
        return Err(
            UnitarySynthesisError::FactorizationFailure {
                message: "cannot compare matrices of different dimensions",
            },
        );
    }

    let mut reference =
        None;

    for index in 0..lhs.data.len() {
        let left =
            lhs.data[index];

        let right =
            rhs.data[index];

        if right.norm()
            > STRUCTURAL_ZERO
        {
            reference =
                Some(left * right.conjugate());

            break;
        }
    }

    let phase =
        match reference {
            Some(value)
                if value.norm()
                    > STRUCTURAL_ZERO =>
            {
                value / value.norm()
            }

            _ => {
                return Ok(
                    lhs.max_difference(
                        rhs,
                    ),
                );
            }
        };

    let mut residual =
        0.0;

    for index in 0..lhs.data.len() {
        let expected =
            rhs.data[index]
                * phase;

        residual = residual.max(
            (lhs.data[index]
                - expected)
                .norm(),
        );
    }

    Ok(residual)
}

// ============================================================================
// Givens operations
// ============================================================================

/// Applies the Givens matrix
///
/// ```text
/// G = [ c   s ]
///     [-s* c*]
/// ```
///
/// to rows `first` and `second` of the working matrix.
fn apply_givens_left(
    matrix: &mut UnitaryMatrix,
    first: usize,
    second: usize,
    c: Complex64,
    s: Complex64,
) -> UnitarySynthesisResult<()> {
    if first == second
        || first >= matrix.dimension
        || second >= matrix.dimension
    {
        return Err(
            UnitarySynthesisError::InvalidTwoLevelFactor {
                first,
                second,
            },
        );
    }

    for column in
        0..matrix.dimension
    {
        let top =
            matrix
                .get_unchecked_by_contract(
                    first,
                    column,
                );

        let bottom =
            matrix
                .get_unchecked_by_contract(
                    second,
                    column,
                );

        let new_top =
            c * top
                + s * bottom;

        let new_bottom =
            (-s.conjugate()) * top
                + c.conjugate()
                    * bottom;

        matrix.set(
            first,
            column,
            new_top,
        )?;

        matrix.set(
            second,
            column,
            new_bottom,
        )?;
    }

    Ok(())
}

/// Applies a two-level unitary to the left side of a matrix.
fn apply_two_level_left(
    matrix: &mut UnitaryMatrix,
    factor: TwoLevelUnitary,
) -> UnitarySynthesisResult<()> {
    if factor.first
        >= matrix.dimension
        || factor.second
            >= matrix.dimension
        || factor.first
            == factor.second
    {
        return Err(
            UnitarySynthesisError::InvalidTwoLevelFactor {
                first: factor.first,
                second: factor.second,
            },
        );
    }

    for column in
        0..matrix.dimension
    {
        let top =
            matrix
                .get_unchecked_by_contract(
                    factor.first,
                    column,
                );

        let bottom =
            matrix
                .get_unchecked_by_contract(
                    factor.second,
                    column,
                );

        let new_top =
            factor.a * top
                + factor.b * bottom;

        let new_bottom =
            factor.c * top
                + factor.d * bottom;

        matrix.set(
            factor.first,
            column,
            new_top,
        )?;

        matrix.set(
            factor.second,
            column,
            new_bottom,
        )?;
    }

    Ok(())
}

// ============================================================================
// Gate-family helpers for integration
// ============================================================================

/// Returns whether a gate kind is a supported single-qubit primitive for
/// downstream unitary lowering.
#[must_use]
pub const fn is_single_qubit_lowering_gate(
    gate: GateKind,
) -> bool {
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

/// Returns whether a gate kind can serve as the standard two-qubit entangler
/// for a universal lowering strategy.
#[must_use]
pub const fn is_two_qubit_entangler(
    gate: GateKind,
) -> bool {
    matches!(
        gate,
        GateKind::CX
            | GateKind::CZ
            | GateKind::SWAP
            | GateKind::ISWAP
            | GateKind::ECR
    )
}

/// Converts a logical qubit index into the canonical logical qubit identifier.
#[must_use]
pub const fn logical_qubit(
    index: usize,
) -> QubitId {
    QubitId::new(index)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn c(
        re: f64,
        im: f64,
    ) -> Complex64 {
        Complex64::new(
            re,
            im,
        )
    }

    fn assert_matrix_equivalent(
        lhs: &UnitaryMatrix,
        rhs: &UnitaryMatrix,
        tolerance: f64,
    ) {
        let residual =
            compare_up_to_global_phase(
                lhs,
                rhs,
            )
            .expect("matrix dimensions must match");

        assert!(
            residual <= tolerance,
            "residual {residual:e} > tolerance {tolerance:e}"
        );
    }

    #[test]
    fn identity_one_qubit_synthesizes() {
        let matrix =
            UnitaryMatrix::from_rows(
                vec![
                    vec![
                        c(1.0, 0.0),
                        c(0.0, 0.0),
                    ],
                    vec![
                        c(0.0, 0.0),
                        c(1.0, 0.0),
                    ],
                ],
            )
            .expect("identity must construct");

        let plan =
            synthesize_unitary(
                &matrix,
            )
            .expect("identity must synthesize");

        assert_eq!(
            plan.qubits,
            1
        );

        assert!(
            plan.two_level_factors.is_empty()
        );

        assert!(
            plan.basis_phases.is_empty()
        );
    }

    #[test]
    fn x_gate_synthesizes() {
        let matrix =
            UnitaryMatrix::from_rows(
                vec![
                    vec![
                        c(0.0, 0.0),
                        c(1.0, 0.0),
                    ],
                    vec![
                        c(1.0, 0.0),
                        c(0.0, 0.0),
                    ],
                ],
            )
            .expect("X must construct");

        let plan =
            synthesize_unitary(
                &matrix,
            )
            .expect("X must synthesize");

        let reconstructed =
            plan.reconstruct()
                .expect("reconstruction must succeed");

        assert_matrix_equivalent(
            &matrix,
            &reconstructed,
            1.0e-10,
        );
    }

    #[test]
    fn hadamard_synthesizes() {
        let scale =
            1.0 / 2.0_f64.sqrt();

        let matrix =
            UnitaryMatrix::from_rows(
                vec![
                    vec![
                        c(scale, 0.0),
                        c(scale, 0.0),
                    ],
                    vec![
                        c(scale, 0.0),
                        c(-scale, 0.0),
                    ],
                ],
            )
            .expect("H must construct");

        let plan =
            synthesize_unitary(
                &matrix,
            )
            .expect("H must synthesize");

        let reconstructed =
            plan.reconstruct()
                .expect("reconstruction must succeed");

        assert_matrix_equivalent(
            &matrix,
            &reconstructed,
            1.0e-10,
        );
    }

    #[test]
    fn two_qubit_identity_synthesizes() {
        let matrix =
            UnitaryMatrix::try_identity(
                4,
                None,
            )
            .expect("identity must construct");

        let plan =
            synthesize_unitary(
                &matrix,
            )
            .expect("identity must synthesize");

        assert_eq!(
            plan.qubits,
            2
        );

        assert!(
            plan.two_level_factors.is_empty()
        );

        assert!(
            plan.basis_phases.is_empty()
        );
    }

    #[test]
    fn controlled_x_synthesizes() {
        let matrix =
            UnitaryMatrix::from_rows(
                vec![
                    vec![
                        c(1.0, 0.0),
                        c(0.0, 0.0),
                        c(0.0, 0.0),
                        c(0.0, 0.0),
                    ],
                    vec![
                        c(0.0, 0.0),
                        c(1.0, 0.0),
                        c(0.0, 0.0),
                        c(0.0, 0.0),
                    ],
                    vec![
                        c(0.0, 0.0),
                        c(0.0, 0.0),
                        c(0.0, 0.0),
                        c(1.0, 0.0),
                    ],
                    vec![
                        c(0.0, 0.0),
                        c(0.0, 0.0),
                        c(1.0, 0.0),
                        c(0.0, 0.0),
                    ],
                ],
            )
            .expect("CX must construct");

        let plan =
            synthesize_unitary(
                &matrix,
            )
            .expect("CX must synthesize");

        let reconstructed =
            plan.reconstruct()
                .expect("reconstruction must succeed");

        assert_matrix_equivalent(
            &matrix,
            &reconstructed,
            1.0e-10,
        );
    }

    #[test]
    fn random_like_two_qubit_unitary_round_trip() {
        let matrix =
            UnitaryMatrix::from_rows(
                vec![
                    vec![
                        c(0.5, 0.0),
                        c(0.5, 0.0),
                        c(0.5, 0.0),
                        c(0.5, 0.0),
                    ],
                    vec![
                        c(-0.5, 0.0),
                        c(0.5, 0.0),
                        c(-0.5, 0.0),
                        c(0.5, 0.0),
                    ],
                    vec![
                        c(-0.5, 0.0),
                        c(-0.5, 0.0),
                        c(0.5, 0.0),
                        c(0.5, 0.0),
                    ],
                    vec![
                        c(-0.5, 0.0),
                        c(0.5, 0.0),
                        c(0.5, 0.0),
                        c(-0.5, 0.0),
                    ],
                ],
            )
            .expect("matrix must construct");

        matrix
            .validate_unitary(
                DEFAULT_TOLERANCE,
            )
            .expect("matrix must be unitary");

        let plan =
            synthesize_unitary(
                &matrix,
            )
            .expect("matrix must synthesize");

        let reconstructed =
            plan.reconstruct()
                .expect("reconstruction must succeed");

        assert_matrix_equivalent(
            &matrix,
            &reconstructed,
            1.0e-9,
        );
    }

    #[test]
    fn rejects_non_unitary_matrix() {
        let matrix =
            UnitaryMatrix::from_rows(
                vec![
                    vec![
                        c(2.0, 0.0),
                        c(0.0, 0.0),
                    ],
                    vec![
                        c(0.0, 0.0),
                        c(1.0, 0.0),
                    ],
                ],
            )
            .expect("matrix construction itself is valid");

        assert!(
            synthesize_unitary(
                &matrix,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_non_power_of_two_dimension() {
        let matrix =
            UnitaryMatrix::from_rows(
                vec![
                    vec![
                        c(1.0, 0.0),
                        c(0.0, 0.0),
                        c(0.0, 0.0),
                    ],
                    vec![
                        c(0.0, 0.0),
                        c(1.0, 0.0),
                        c(0.0, 0.0),
                    ],
                    vec![
                        c(0.0, 0.0),
                        c(0.0, 0.0),
                        c(1.0, 0.0),
                    ],
                ],
            )
            .expect("matrix construction is valid");

        assert!(
            synthesize_unitary(
                &matrix,
            )
            .is_err()
        );
    }

    #[test]
    fn gray_path_is_valid() {
        let factor =
            TwoLevelUnitary::new(
                0,
                7,
                c(1.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(1.0, 0.0),
            )
            .expect("factor must construct");

        let path =
            factor
                .gray_path()
                .expect("Gray path must exist");

        assert_eq!(
            path.first().copied(),
            Some(0)
        );

        assert_eq!(
            path.last().copied(),
            Some(7)
        );

        for pair in path.windows(2) {
            assert_eq!(
                (pair[0] ^ pair[1])
                    .count_ones(),
                1
            );
        }
    }

    #[test]
    fn control_metadata_is_deterministic() {
        let factor =
            TwoLevelUnitary::new(
                0,
                3,
                c(1.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(1.0, 0.0),
            )
            .expect("factor must construct");

        let controls =
            factor
                .control_qubits(3)
                .expect("controls must construct");

        assert_eq!(
            controls,
            vec![0]
        );
    }

    #[test]
    fn limits_are_enforced_before_factorization() {
        let matrix =
            UnitaryMatrix::try_identity(
                4,
                None,
            )
            .expect("identity must construct");

        let config =
            UnitarySynthesisConfig {
                limits:
                    UnitarySynthesisLimits {
                        max_qubits:
                            Some(1),
                        ..UnitarySynthesisLimits::unlimited()
                    },
                ..UnitarySynthesisConfig::default()
            };

        let error =
            synthesize_unitary_with_config(
                &matrix,
                config,
            )
            .expect_err(
                "two-qubit matrix must exceed one-qubit limit",
            );

        assert!(matches!(
            error,
            UnitarySynthesisError::ResourceLimitExceeded {
                resource: "qubits",
                ..
            }
        ));
    }

    #[test]
    fn unlimited_configuration_is_valid() {
        UnitarySynthesisConfig::unlimited()
            .validate()
            .expect(
                "unlimited configuration must be valid",
            );
    }

    #[test]
    fn logical_qubit_helper_is_canonical() {
        let qubit =
            logical_qubit(7);

        assert_eq!(
            qubit.index(),
            7
        );
    }
}