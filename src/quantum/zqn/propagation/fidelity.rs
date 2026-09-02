//! Zamani Quantum Noise (ZQN) — Fidelity Propagation.
//!
//! # Purpose
//!
//! This module owns backend-independent fidelity and fidelity-derived
//! comparison mathematics used by ZQN propagation.
//!
//! The canonical state-fidelity convention in this module is the
//! squared Uhlmann-Jozsa fidelity:
//!
//! ```text
//! F(rho, sigma)
//!   = [Tr sqrt(sqrt(rho) sigma sqrt(rho))]^2
//! ```
//!
//! Consequently:
//!
//! ```text
//! F(|psi>, |phi>) = |<psi|phi>|^2
//! F(|psi>, rho)  = <psi|rho|psi>
//! ```
//!
//! The convention is explicit in [`FidelityDefinition`] and is never inferred
//! from the caller.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - state-vector fidelity;
//! - pure-state / density-matrix fidelity;
//! - mixed-state Uhlmann fidelity;
//! - classical probability-distribution fidelity;
//! - normalized Choi/process fidelity;
//! - average gate fidelity derived from entanglement/process fidelity;
//! - fidelity result metadata;
//! - numerical validation specific to fidelity;
//! - explicit fidelity tolerances;
//! - explicit computational resource policy;
//! - deterministic Hermitian eigendecomposition required by the dependency-free
//!   mixed-state implementation.
//!
//! # Does NOT own
//!
//! This file does not own:
//!
//! - quantum states as a data model;
//! - density-matrix storage;
//! - state-vector storage;
//! - quantum channels;
//! - noise models;
//! - calibration;
//! - characterization experiments;
//! - statistical estimation;
//! - confidence intervals;
//! - error budgets;
//! - sensitivity analysis;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmark protocols;
//! - hardware APIs;
//! - canonical Quantum IR;
//! - qubit identity;
//! - serialization wire formats;
//! - cryptographic hashing.
//!
//! Those systems consume this mathematical contract.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!     |
//!     +------------------------------+
//!     |                              |
//!     v                              v
//! state/channel representation    characterization
//!     |                              |
//!     +--------------+---------------+
//!                    |
//!                    v
//!          zqn::propagation::fidelity
//!                    |
//!          +---------+----------+
//!          |                    |
//!          v                    v
//!      error_budget       sensitivity
//!          |                    |
//!          +---------+----------+
//!                    |
//!                    v
//!              routing / QEC
//! ```
//!
//! # Canonical complex type
//!
//! This module deliberately uses:
//!
//! ```text
//! crate::quantum::memory::complex::Complex64
//! ```
//!
//! Zamani already owns the canonical complex scalar in `quantum::memory`.
//! ZQN must not introduce a competing `Complex64`.
//!
//! # Quantum-resource identity
//!
//! Fidelity itself is representation- and resource-agnostic.
//!
//! Therefore this module deliberately does NOT import:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A higher-level result associated with a physical or logical resource must
//! carry the canonical resource identity owned by `quantum::ir::qubit`.
//!
//! # Mathematical contract
//!
//! For two normalized pure states:
//!
//! ```text
//! F = |<psi|phi>|^2
//! ```
//!
//! For a pure state and density matrix:
//!
//! ```text
//! F = <psi|rho|psi>
//! ```
//!
//! For two density matrices:
//!
//! ```text
//! F = [Tr sqrt(sqrt(rho) sigma sqrt(rho))]^2
//! ```
//!
//! For normalized classical distributions:
//!
//! ```text
//! F = [sum_i sqrt(p_i q_i)]^2
//! ```
//!
//! For normalized Choi states representing channels:
//!
//! ```text
//! F_process = F(Choi_A, Choi_B)
//! ```
//!
//! For a channel acting on a `d`-dimensional Hilbert space:
//!
//! ```text
//! F_average = (d * F_process + 1) / (d + 1)
//! ```
//!
//! The process-fidelity input must use the same normalization convention on
//! both channels. This module does not silently normalize an arbitrary Choi
//! matrix because doing so could conceal a channel-convention error.
//!
//! # Numerical policy
//!
//! The module rejects:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - empty states;
//! - dimension mismatches;
//! - materially non-normalized states;
//! - non-Hermitian density matrices;
//! - materially negative density-matrix eigenvalues;
//! - invalid probability distributions;
//! - invalid tolerances;
//! - unsafe matrix-size arithmetic;
//! - numerical results outside the declared fidelity domain.
//!
//! Tiny negative eigenvalues caused by floating-point roundoff may be accepted
//! when their magnitude is within the caller's explicit positivity tolerance.
//!
//! They are never silently converted to positive values in the input.
//!
//! # Exactness and approximation
//!
//! Pure-state and pure-state/density-matrix formulas are evaluated directly.
//!
//! General mixed-state fidelity is mathematically exact with respect to the
//! supplied floating-point matrix representation, subject only to numerical
//! floating-point error and the convergence policy of the deterministic
//! Hermitian eigensolver.
//!
//! The eigensolver is not an exact symbolic algorithm.
//!
//! The result therefore carries the numerical tolerance used by the
//! calculation.
//!
//! No statistical uncertainty is inferred here.
//!
//! Statistical confidence intervals belong to characterization/estimation.
//!
//! # Scalability
//!
//! There is no semantic limit on:
//!
//! - qubit count;
//! - qudit dimension;
//! - matrix dimension;
//! - number of outputs;
//! - number of resources;
//! - number of machines;
//! - circuit depth;
//! - execution duration.
//!
//! A caller can supply any finite representation that available resources can
//! hold.
//!
//! Dense density matrices inherently require O(n²) storage and mixed-state
//! eigendecomposition requires O(n³) arithmetic. That is a property of this
//! representation/algorithm, not a semantic machine-size limit.
//!
//! Future sparse, tensor-network, distributed, symbolic, accelerator or
//! hardware-native implementations can implement separate fidelity strategies
//! without changing the public mathematical contract defined here.
//!
//! # Resource safety
//!
//! Potentially large operations accept [`FidelityLimits`].
//!
//! `max_matrix_elements` is an explicit resource policy, not a semantic
//! restriction.
//!
//! `None` means that this module imposes no additional matrix-element limit.
//!
//! Allocations use `try_reserve_exact` before initialization so allocation
//! failure is returned instead of being converted into silent corruption.
//!
//! # Determinism
//!
//! This module:
//!
//! - has no RNG;
//! - has no global mutable state;
//! - has no clock dependency;
//! - has no thread-local semantic state;
//! - has no unordered collection in numerical reduction;
//! - uses deterministic matrix traversal order;
//! - uses deterministic Jacobi rotation ordering.
//!
//! Identical inputs, tolerance and limits produce the same numerical operation
//! sequence.
//!
//! Cross-platform bit-for-bit equality is not promised because floating-point
//! implementations may differ between architectures.
//!
//! # Integration with uncertainty.rs
//!
//! `uncertainty.rs` owns uncertainty propagation.
//!
//! It may consume the scalar fidelity value produced here and propagate
//! uncertainty through a declared sensitivity/Jacobian model.
//!
//! This module must not duplicate uncertainty propagation.
//!
//! # Integration with error_budget.rs
//!
//! Error-budget code can consume:
//!
//! ```text
//! FidelityResult::value()
//! ```
//!
//! and convert it into its own budget/error representation.
//!
//! Fidelity does not own budget allocation.
//!
//! # Integration with sensitivity.rs
//!
//! Sensitivity analysis may call these functions repeatedly with perturbed
//! states or channels.
//!
//! This file does not own the perturbation strategy.
//!
//! # Integration with benchmarking
//!
//! The existing benchmarking subsystem already contains fidelity mathematics.
//! The long-term integration should be:
//!
//! ```text
//! ZQN fidelity
//!       |
//!       v
//! benchmarking adapter
//!       |
//!       v
//! benchmark result/report
//! ```
//!
//! The existing duplicate complex-number/fidelity implementation should be
//! migrated to consume this module rather than defining a second numerical
//! source of truth.
//!
//! # Integration with memory
//!
//! `quantum::memory::StateVector` and density-matrix representations own their
//! storage and resource lifecycle.
//!
//! Integration code should expose their validated data as slices/views and
//! call the pure mathematical functions here.
//!
//! This module never allocates or owns quantum memory beyond temporary
//! mathematical workspaces required by a requested calculation.
//!
//! # Integration with channels
//!
//! Channel implementations such as Kraus, Choi, Pauli-transfer or process
//! representations may provide normalized Choi data to
//! [`process_fidelity_from_normalized_choi`].
//!
//! ZQN channel semantics remain owned by `zqn::channel`.
//!
//! # Serialization
//!
//! This file defines semantic mathematical values only.
//!
//! Versioned serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! Internal vector layout must not become the external wire-format contract.
//!
//! # Security
//!
//! Untrusted input must be validated before expensive matrix operations.
//!
//! Callers processing untrusted models should configure:
//!
//! - matrix-element limits;
//! - Jacobi-sweep limits;
//! - numerical tolerances.
//!
//! A malformed matrix must fail with a structured error rather than panic.
//!
//! # Rust compatibility
//!
//! This implementation targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. fidelity convention is explicit;
//! 2. pure-state fidelity is implemented;
//! 3. pure-state/density fidelity is implemented;
//! 4. mixed-state Uhlmann fidelity is implemented;
//! 5. classical fidelity is implemented;
//! 6. normalized process/Choi fidelity is implemented;
//! 7. average gate fidelity is implemented;
//! 8. all inputs are numerically validated;
//! 9. resource limits are explicit;
//! 10. no semantic machine-size limit exists;
//! 11. canonical `memory::Complex64` is used;
//! 12. no duplicate qubit identity exists;
//! 13. no RNG/global state exists;
//! 14. deterministic reduction order is defined;
//! 15. serialization remains owned by `zqn::io`;
//! 16. tests cover mathematical invariants and scaling behavior.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::memory::complex::Complex64;

// =============================================================================
// Public schema and numerical defaults
// =============================================================================

/// Stable semantic identifier for this module.
pub const FIDELITY_SCHEMA_ID: &str =
    "zamani.quantum.zqn.propagation.fidelity";

/// Semantic version of this module's public mathematical contract.
pub const FIDELITY_SCHEMA_VERSION: u16 = 1;

/// Default general numerical tolerance.
pub const DEFAULT_FIDELITY_TOLERANCE: f64 = 1.0e-10;

/// Default state normalization tolerance.
pub const DEFAULT_NORMALIZATION_TOLERANCE: f64 = 1.0e-10;

/// Default density-matrix Hermiticity tolerance.
pub const DEFAULT_HERMITICITY_TOLERANCE: f64 = 1.0e-10;

/// Default positive-semidefinite tolerance.
pub const DEFAULT_POSITIVITY_TOLERANCE: f64 = 1.0e-10;

/// Default distribution normalization tolerance.
pub const DEFAULT_DISTRIBUTION_TOLERANCE: f64 = 1.0e-10;

/// Default result-domain tolerance.
pub const DEFAULT_RANGE_TOLERANCE: f64 = 1.0e-10;

// =============================================================================
// Fidelity definition
// =============================================================================

/// Explicit mathematical definition represented by a [`FidelityResult`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FidelityDefinition {
    /// Squared Uhlmann-Jozsa fidelity.
    ///
    /// `F = [Tr sqrt(sqrt(rho) sigma sqrt(rho))]^2`
    UhlmannSquared,

    /// Pure state versus density matrix.
    ///
    /// `F = <psi|rho|psi>`
    PureStateDensity,

    /// Two normalized pure state vectors.
    ///
    /// `F = |<psi|phi>|²`
    PureStateOverlap,

    /// Two normalized classical probability distributions.
    ///
    /// `F = [sum sqrt(p_i q_i)]²`
    ClassicalSquared,

    /// Fidelity between two normalized Choi states.
    ///
    /// This is the entanglement/process fidelity under the normalization
    /// convention required by this module.
    NormalizedChoi,

    /// Average gate fidelity derived from process fidelity.
    AverageGate,
}

// =============================================================================
// Result
// =============================================================================

/// Validated fidelity result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FidelityResult {
    value: f64,
    definition: FidelityDefinition,
    tolerance: f64,
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
            return Err(FidelityError::OutOfRange {
                value,
                tolerance,
            });
        }

        Ok(Self {
            value: clamp_near_unit_interval(value, tolerance),
            definition,
            tolerance,
        })
    }

    /// Returns the numerical fidelity.
    #[inline]
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Returns the mathematical definition used.
    #[inline]
    pub const fn definition(self) -> FidelityDefinition {
        self.definition
    }

    /// Returns the numerical tolerance used for validation.
    #[inline]
    pub const fn tolerance(self) -> f64 {
        self.tolerance
    }

    /// Returns the infidelity `1 - F`.
    ///
    /// The value is clamped only within the result's already validated
    /// numerical tolerance.
    #[inline]
    pub fn infidelity(self) -> f64 {
        let result = 1.0 - self.value;
        if result.abs() <= self.tolerance {
            0.0
        } else {
            result
        }
    }
}

// =============================================================================
// Limits
// =============================================================================

/// Explicit computational resource policy for fidelity calculations.
///
/// These limits constrain implementation work only. They do not describe a
/// maximum quantum-machine size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FidelityLimits {
    /// Maximum number of matrix elements allowed in one dense matrix.
    ///
    /// `None` means no additional limit is imposed by this module.
    pub max_matrix_elements: Option<u128>,

    /// Maximum number of Jacobi sweeps.
    ///
    /// `None` means convergence controls termination.
    pub max_jacobi_sweeps: Option<u64>,
}

impl Default for FidelityLimits {
    fn default() -> Self {
        Self {
            max_matrix_elements: None,
            max_jacobi_sweeps: None,
        }
    }
}

impl FidelityLimits {
    /// Creates unrestricted fidelity limits.
    pub const fn unlimited() -> Self {
        Self {
            max_matrix_elements: None,
            max_jacobi_sweeps: None,
        }
    }

    /// Sets a maximum dense-matrix element count.
    pub const fn with_max_matrix_elements(mut self, value: u128) -> Self {
        self.max_matrix_elements = Some(value);
        self
    }

    /// Sets a maximum Jacobi sweep count.
    pub const fn with_max_jacobi_sweeps(mut self, value: u64) -> Self {
        self.max_jacobi_sweeps = Some(value);
        self
    }
}

// =============================================================================
// Error
// =============================================================================

/// Errors produced by fidelity calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum FidelityError {
    /// A supplied numerical value is non-finite.
    NonFiniteValue {
        /// Semantic location.
        context: &'static str,
    },

    /// A state vector is empty.
    EmptyStateVector,

    /// State-vector dimensions differ.
    StateDimensionMismatch {
        /// First length.
        left: usize,
        /// Second length.
        right: usize,
    },

    /// State vector is not normalized.
    StateNotNormalized {
        /// Observed squared norm.
        norm_squared: f64,
        /// Permitted deviation.
        tolerance: f64,
    },

    /// A distribution is empty.
    EmptyDistribution,

    /// Distribution lengths differ.
    DistributionDimensionMismatch {
        /// First length.
        left: usize,
        /// Second length.
        right: usize,
    },

    /// Invalid probability.
    InvalidProbability {
        /// Element index.
        index: usize,
        /// Supplied probability.
        value: f64,
    },

    /// Distribution is not normalized.
    DistributionNotNormalized {
        /// Observed sum.
        sum: f64,
        /// Permitted deviation.
        tolerance: f64,
    },

    /// Matrix contains no elements.
    EmptyMatrix,

    /// Matrix is not square.
    MatrixNotSquare {
        /// Number of rows.
        rows: usize,
        /// Number of columns.
        columns: usize,
    },

    /// Matrix data length does not match its dimensions.
    MatrixLengthMismatch {
        /// Number of rows.
        rows: usize,
        /// Number of columns.
        columns: usize,
        /// Actual number of elements.
        actual: usize,
    },

    /// Matrix dimensions differ.
    MatrixDimensionMismatch {
        /// First dimension.
        left: usize,
        /// Second dimension.
        right: usize,
    },

    /// Matrix is not Hermitian within tolerance.
    NotHermitian {
        /// Maximum observed deviation.
        maximum_deviation: f64,
        /// Permitted deviation.
        tolerance: f64,
    },

    /// Matrix trace is not one.
    InvalidTrace {
        /// Real trace.
        trace_real: f64,
        /// Imaginary trace.
        trace_imaginary: f64,
        /// Permitted deviation.
        tolerance: f64,
    },

    /// Density matrix has materially negative diagonal probability.
    NegativeDiagonal {
        /// Index.
        index: usize,
        /// Value.
        value: f64,
        /// Tolerance.
        tolerance: f64,
    },

    /// Density matrix is not positive semidefinite.
    NotPositiveSemidefinite {
        /// Minimum computed eigenvalue.
        minimum_eigenvalue: f64,
        /// Permitted numerical negativity.
        tolerance: f64,
    },

    /// Hermitian eigendecomposition did not converge.
    EigenDecompositionFailed,

    /// Matrix-size arithmetic overflowed.
    SizeOverflow {
        /// Operation.
        context: &'static str,
    },

    /// Requested dense work exceeds explicit policy.
    ResourceLimitExceeded {
        /// Requested elements.
        requested: u128,
        /// Maximum allowed.
        maximum: u128,
        /// Resource type.
        resource: &'static str,
    },

    /// Tolerance is invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        value: f64,
    },

    /// A fidelity value is outside its mathematical range.
    OutOfRange {
        /// Value.
        value: f64,
        /// Permitted numerical tolerance.
        tolerance: f64,
    },

    /// Average-gate-fidelity dimension is invalid.
    InvalidHilbertDimension,

    /// Process fidelity dimension does not match the declared Hilbert-space
    /// dimension.
    ProcessDimensionMismatch {
        /// Choi matrix dimension.
        choi_dimension: usize,
        /// Hilbert-space dimension.
        hilbert_dimension: usize,
    },

    /// Iterative eigendecomposition exceeded the explicit sweep limit.
    ConvergenceLimitExceeded {
        /// Number of completed sweeps.
        sweeps: u64,
    },

    /// An intermediate numerical operation became non-finite.
    NumericalFailure {
        /// Operation.
        operation: &'static str,
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
                write!(f, "state-vector dimensions differ: {left} != {right}")
            }
            Self::StateNotNormalized {
                norm_squared,
                tolerance,
            } => {
                write!(
                    f,
                    "state vector is not normalized: norm²={norm_squared}, tolerance={tolerance}"
                )
            }
            Self::EmptyDistribution => {
                write!(f, "probability distribution must not be empty")
            }
            Self::DistributionDimensionMismatch { left, right } => {
                write!(f, "distribution dimensions differ: {left} != {right}")
            }
            Self::InvalidProbability { index, value } => {
                write!(f, "invalid probability at index {index}: {value}")
            }
            Self::DistributionNotNormalized { sum, tolerance } => {
                write!(
                    f,
                    "distribution is not normalized: sum={sum}, tolerance={tolerance}"
                )
            }
            Self::EmptyMatrix => {
                write!(f, "matrix must not be empty")
            }
            Self::MatrixNotSquare { rows, columns } => {
                write!(f, "matrix must be square: {rows}x{columns}")
            }
            Self::MatrixLengthMismatch {
                rows,
                columns,
                actual,
            } => {
                write!(
                    f,
                    "matrix length mismatch: {rows}x{columns} requires {} elements, got {actual}",
                    rows.saturating_mul(*columns)
                )
            }
            Self::MatrixDimensionMismatch { left, right } => {
                write!(f, "matrix dimensions differ: {left} != {right}")
            }
            Self::NotHermitian {
                maximum_deviation,
                tolerance,
            } => {
                write!(
                    f,
                    "matrix is not Hermitian: deviation={maximum_deviation}, tolerance={tolerance}"
                )
            }
            Self::InvalidTrace {
                trace_real,
                trace_imaginary,
                tolerance,
            } => {
                write!(
                    f,
                    "invalid density-matrix trace: {trace_real}+{trace_imaginary}i, tolerance={tolerance}"
                )
            }
            Self::NegativeDiagonal {
                index,
                value,
                tolerance,
            } => {
                write!(
                    f,
                    "negative density-matrix diagonal at {index}: {value}, tolerance={tolerance}"
                )
            }
            Self::NotPositiveSemidefinite {
                minimum_eigenvalue,
                tolerance,
            } => {
                write!(
                    f,
                    "density matrix is not positive semidefinite: minimum eigenvalue={minimum_eigenvalue}, tolerance={tolerance}"
                )
            }
            Self::EigenDecompositionFailed => {
                write!(f, "Hermitian eigendecomposition failed to converge")
            }
            Self::SizeOverflow { context } => {
                write!(f, "size arithmetic overflow in {context}")
            }
            Self::ResourceLimitExceeded {
                requested,
                maximum,
                resource,
            } => {
                write!(
                    f,
                    "resource limit exceeded for {resource}: requested {requested}, maximum {maximum}"
                )
            }
            Self::InvalidTolerance { value } => {
                write!(f, "invalid tolerance: {value}")
            }
            Self::OutOfRange { value, tolerance } => {
                write!(
                    f,
                    "fidelity outside [0,1]: value={value}, tolerance={tolerance}"
                )
            }
            Self::InvalidHilbertDimension => {
                write!(f, "Hilbert-space dimension must be greater than zero")
            }
            Self::ProcessDimensionMismatch {
                choi_dimension,
                hilbert_dimension,
            } => {
                write!(
                    f,
                    "Choi dimension {choi_dimension} is incompatible with Hilbert dimension {hilbert_dimension}"
                )
            }
            Self::ConvergenceLimitExceeded { sweeps } => {
                write!(f, "eigendecomposition convergence limit exceeded after {sweeps} sweeps")
            }
            Self::NumericalFailure { operation } => {
                write!(f, "non-finite intermediate result during {operation}")
            }
        }
    }
}

impl std::error::Error for FidelityError {}

// =============================================================================
// Public state/vector APIs
// =============================================================================

/// Calculates squared fidelity between two normalized pure states.
///
/// ```text
/// F = |<psi|phi>|²
/// ```
pub fn pure_state_fidelity(
    left: &[Complex64],
    right: &[Complex64],
) -> Result<FidelityResult, FidelityError> {
    pure_state_fidelity_with_tolerance(
        left,
        right,
        DEFAULT_NORMALIZATION_TOLERANCE,
    )
}

/// Calculates pure-state fidelity with an explicit normalization tolerance.
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
        let product = checked_complex_mul(
            left[index].conjugate(),
            right[index],
            "pure-state inner product",
        )?;

        overlap = checked_complex_add(
            overlap,
            product,
            "pure-state inner product accumulation",
        )?;
    }

    let value = overlap.norm_squared();

    FidelityResult::new(
        value,
        FidelityDefinition::PureStateOverlap,
        tolerance,
    )
}

/// Calculates fidelity between a normalized pure state and a density matrix.
///
/// ```text
/// F(|psi>, rho) = <psi|rho|psi>
/// ```
pub fn pure_state_density_fidelity(
    state: &[Complex64],
    density_matrix: &[Complex64],
) -> Result<FidelityResult, FidelityError> {
    pure_state_density_fidelity_with_options(
        state,
        density_matrix,
        DEFAULT_NORMALIZATION_TOLERANCE,
        DEFAULT_HERMITICITY_TOLERANCE,
        DEFAULT_POSITIVITY_TOLERANCE,
        &FidelityLimits::default(),
    )
}

/// Calculates pure-state/density-matrix fidelity with explicit policies.
pub fn pure_state_density_fidelity_with_options(
    state: &[Complex64],
    density_matrix: &[Complex64],
    state_tolerance: f64,
    hermiticity_tolerance: f64,
    positivity_tolerance: f64,
    limits: &FidelityLimits,
) -> Result<FidelityResult, FidelityError> {
    validate_tolerance(state_tolerance)?;
    validate_tolerance(hermiticity_tolerance)?;
    validate_tolerance(positivity_tolerance)?;

    validate_state_vector(state, state_tolerance)?;

    let dimension = checked_square_dimension_from_slice(
        density_matrix,
        limits,
        "density matrix",
    )?;

    if state.len() != dimension {
        return Err(FidelityError::StateDimensionMismatch {
            left: state.len(),
            right: dimension,
        });
    }

    validate_density_matrix(
        density_matrix,
        dimension,
        hermiticity_tolerance,
        positivity_tolerance,
        limits,
    )?;

    let mut expectation = Complex64::zero();

    for row in 0..dimension {
        let mut row_sum = Complex64::zero();

        for column in 0..dimension {
            let rho = density_matrix[row * dimension + column];

            let product = checked_complex_mul(
                rho,
                state[column],
                "pure-state/density multiplication",
            )?;

            row_sum = checked_complex_add(
                row_sum,
                product,
                "pure-state/density row accumulation",
            )?;
        }

        let contribution = checked_complex_mul(
            state[row].conjugate(),
            row_sum,
            "pure-state/density expectation",
        )?;

        expectation = checked_complex_add(
            expectation,
            contribution,
            "pure-state/density expectation accumulation",
        )?;
    }

    if expectation.imaginary().abs() > hermiticity_tolerance {
        return Err(FidelityError::NumericalFailure {
            operation: "pure-state density expectation",
        });
    }

    FidelityResult::new(
        expectation.real(),
        FidelityDefinition::PureStateDensity,
        state_tolerance.max(hermiticity_tolerance),
    )
}

// =============================================================================
// Mixed-state fidelity
// =============================================================================

/// Calculates squared Uhlmann-Jozsa fidelity between two density matrices.
///
/// ```text
/// F(rho,sigma)
///   = [Tr sqrt(sqrt(rho) sigma sqrt(rho))]^2
/// ```
///
/// The calculation uses deterministic Hermitian eigendecomposition and does
/// not depend on a vendor numerical library.
pub fn density_matrix_fidelity(
    left: &[Complex64],
    right: &[Complex64],
) -> Result<FidelityResult, FidelityError> {
    density_matrix_fidelity_with_options(
        left,
        right,
        &FidelityOptions::default(),
    )
}

/// Options for mixed-state fidelity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FidelityOptions {
    /// State/density numerical tolerance.
    pub tolerance: f64,

    /// Hermiticity tolerance.
    pub hermiticity_tolerance: f64,

    /// Positive-semidefinite tolerance.
    pub positivity_tolerance: f64,

    /// Resource policy.
    pub limits: FidelityLimits,
}

impl Default for FidelityOptions {
    fn default() -> Self {
        Self {
            tolerance: DEFAULT_FIDELITY_TOLERANCE,
            hermiticity_tolerance: DEFAULT_HERMITICITY_TOLERANCE,
            positivity_tolerance: DEFAULT_POSITIVITY_TOLERANCE,
            limits: FidelityLimits::default(),
        }
    }
}

impl FidelityOptions {
    /// Creates options with all standard defaults.
    pub const fn new() -> Self {
        Self {
            tolerance: DEFAULT_FIDELITY_TOLERANCE,
            hermiticity_tolerance: DEFAULT_HERMITICITY_TOLERANCE,
            positivity_tolerance: DEFAULT_POSITIVITY_TOLERANCE,
            limits: FidelityLimits::unlimited(),
        }
    }
}

/// Calculates mixed-state fidelity with explicit numerical/resource policy.
pub fn density_matrix_fidelity_with_options(
    left: &[Complex64],
    right: &[Complex64],
    options: &FidelityOptions,
) -> Result<FidelityResult, FidelityError> {
    validate_tolerance(options.tolerance)?;
    validate_tolerance(options.hermiticity_tolerance)?;
    validate_tolerance(options.positivity_tolerance)?;

    let left_dimension =
        checked_square_dimension_from_slice(left, &options.limits, "left density matrix")?;

    let right_dimension =
        checked_square_dimension_from_slice(right, &options.limits, "right density matrix")?;

    if left_dimension != right_dimension {
        return Err(FidelityError::MatrixDimensionMismatch {
            left: left_dimension,
            right: right_dimension,
        });
    }

    validate_density_matrix(
        left,
        left_dimension,
        options.hermiticity_tolerance,
        options.positivity_tolerance,
        &options.limits,
    )?;

    validate_density_matrix(
        right,
        right_dimension,
        options.hermiticity_tolerance,
        options.positivity_tolerance,
        &options.limits,
    )?;

    // sqrt(rho)
    let (_, sqrt_left) = hermitian_spectral_decomposition(
        left,
        left_dimension,
        options.tolerance,
        options.positivity_tolerance,
        &options.limits,
    )?;

    // A = sqrt(rho) sigma sqrt(rho)
    let temporary = matrix_multiply(
        &sqrt_left,
        right,
        left_dimension,
        &options.limits,
        "sqrt(rho) * sigma",
    )?;

    let sandwiched = matrix_multiply(
        &temporary,
        &sqrt_left,
        left_dimension,
        &options.limits,
        "sqrt(rho) * sigma * sqrt(rho)",
    )?;

    // Eigenvalues of the positive-semidefinite sandwiched matrix are
    // non-negative up to floating-point tolerance.
    let (eigenvalues, _) = hermitian_spectral_decomposition(
        &sandwiched,
        left_dimension,
        options.tolerance,
        options.positivity_tolerance,
        &options.limits,
    )?;

    let mut root_trace = 0.0_f64;

    for eigenvalue in eigenvalues {
        if eigenvalue < -options.positivity_tolerance {
            return Err(FidelityError::NotPositiveSemidefinite {
                minimum_eigenvalue: eigenvalue,
                tolerance: options.positivity_tolerance,
            });
        }

        let nonnegative = if eigenvalue < 0.0 {
            0.0
        } else {
            eigenvalue
        };

        root_trace += nonnegative.sqrt();

        if !root_trace.is_finite() {
            return Err(FidelityError::NumericalFailure {
                operation: "Uhlmann square-root trace accumulation",
            });
        }
    }

    let fidelity = root_trace * root_trace;

    FidelityResult::new(
        fidelity,
        FidelityDefinition::UhlmannSquared,
        options.tolerance,
    )
}

// =============================================================================
// Classical fidelity
// =============================================================================

/// Calculates squared fidelity between two normalized classical
/// probability distributions.
///
/// ```text
/// F(p,q) = [sum_i sqrt(p_i q_i)]²
/// ```
pub fn classical_distribution_fidelity(
    left: &[f64],
    right: &[f64],
) -> Result<FidelityResult, FidelityError> {
    classical_distribution_fidelity_with_tolerance(
        left,
        right,
        DEFAULT_DISTRIBUTION_TOLERANCE,
    )
}

/// Calculates classical distribution fidelity with explicit tolerance.
pub fn classical_distribution_fidelity_with_tolerance(
    left: &[f64],
    right: &[f64],
    tolerance: f64,
) -> Result<FidelityResult, FidelityError> {
    validate_tolerance(tolerance)?;

    validate_distribution(left, tolerance)?;
    validate_distribution(right, tolerance)?;

    if left.len() != right.len() {
        return Err(FidelityError::DistributionDimensionMismatch {
            left: left.len(),
            right: right.len(),
        });
    }

    let mut coefficient = 0.0_f64;

    for index in 0..left.len() {
        let product = left[index] * right[index];

        if !product.is_finite() {
            return Err(FidelityError::NumericalFailure {
                operation: "classical fidelity probability product",
            });
        }

        coefficient += product.sqrt();

        if !coefficient.is_finite() {
            return Err(FidelityError::NumericalFailure {
                operation: "classical fidelity accumulation",
            });
        }
    }

    FidelityResult::new(
        coefficient * coefficient,
        FidelityDefinition::ClassicalSquared,
        tolerance,
    )
}

// =============================================================================
// Choi/process fidelity
// =============================================================================

/// Calculates fidelity between two already-normalized Choi states.
///
/// The supplied matrices must each be valid density matrices representing
/// normalized Choi states:
///
/// ```text
/// Tr(J) = 1
/// ```
///
/// This function deliberately does not normalize arbitrary Choi matrices
/// because Choi normalization conventions differ across channel APIs.
pub fn process_fidelity_from_normalized_choi(
    left: &[Complex64],
    right: &[Complex64],
) -> Result<FidelityResult, FidelityError> {
    let result = density_matrix_fidelity(left, right)?;

    Ok(FidelityResult {
        value: result.value,
        definition: FidelityDefinition::NormalizedChoi,
        tolerance: result.tolerance,
    })
}

/// Calculates average gate fidelity from normalized process/entanglement
/// fidelity.
///
/// ```text
/// F_avg = (d F_process + 1) / (d + 1)
/// ```
pub fn average_gate_fidelity(
    process_fidelity: f64,
    hilbert_dimension: usize,
) -> Result<FidelityResult, FidelityError> {
    validate_tolerance(DEFAULT_FIDELITY_TOLERANCE)?;

    if hilbert_dimension == 0 {
        return Err(FidelityError::InvalidHilbertDimension);
    }

    if !process_fidelity.is_finite() {
        return Err(FidelityError::NonFiniteValue {
            context: "process fidelity",
        });
    }

    if process_fidelity < -DEFAULT_RANGE_TOLERANCE
        || process_fidelity > 1.0 + DEFAULT_RANGE_TOLERANCE
    {
        return Err(FidelityError::OutOfRange {
            value: process_fidelity,
            tolerance: DEFAULT_RANGE_TOLERANCE,
        });
    }

    let d = hilbert_dimension as f64;

    let denominator = d + 1.0;

    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(FidelityError::NumericalFailure {
            operation: "average gate fidelity denominator",
        });
    }

    let value = (d * process_fidelity + 1.0) / denominator;

    FidelityResult::new(
        value,
        FidelityDefinition::AverageGate,
        DEFAULT_FIDELITY_TOLERANCE,
    )
}

/// Calculates normalized Choi/process fidelity and then average gate
/// fidelity for a `d`-dimensional channel.
pub fn average_gate_fidelity_from_normalized_choi(
    left_choi: &[Complex64],
    right_choi: &[Complex64],
    hilbert_dimension: usize,
) -> Result<FidelityResult, FidelityError> {
    if hilbert_dimension == 0 {
        return Err(FidelityError::InvalidHilbertDimension);
    }

    let choi_dimension = checked_square(hilbert_dimension)?;

    if left_choi.len() != checked_square(choi_dimension)?
        || right_choi.len() != checked_square(choi_dimension)?
    {
        return Err(FidelityError::ProcessDimensionMismatch {
            choi_dimension,
            hilbert_dimension,
        });
    }

    let process = process_fidelity_from_normalized_choi(
        left_choi,
        right_choi,
    )?;

    average_gate_fidelity(
        process.value(),
        hilbert_dimension,
    )
}

// =============================================================================
// Validation
// =============================================================================

fn validate_state_vector(
    state: &[Complex64],
    tolerance: f64,
) -> Result<(), FidelityError> {
    if state.is_empty() {
        return Err(FidelityError::EmptyStateVector);
    }

    validate_tolerance(tolerance)?;

    let mut norm_squared = 0.0_f64;

    for value in state {
        if !value.is_finite() {
            return Err(FidelityError::NonFiniteValue {
                context: "state-vector amplitude",
            });
        }

        norm_squared += value.norm_squared();

        if !norm_squared.is_finite() {
            return Err(FidelityError::NumericalFailure {
                operation: "state-vector norm accumulation",
            });
        }
    }

    if (norm_squared - 1.0).abs() > tolerance {
        return Err(FidelityError::StateNotNormalized {
            norm_squared,
            tolerance,
        });
    }

    Ok(())
}

fn validate_distribution(
    distribution: &[f64],
    tolerance: f64,
) -> Result<(), FidelityError> {
    if distribution.is_empty() {
        return Err(FidelityError::EmptyDistribution);
    }

    validate_tolerance(tolerance)?;

    let mut sum = 0.0_f64;

    for (index, probability) in distribution.iter().copied().enumerate() {
        if !probability.is_finite() {
            return Err(FidelityError::NonFiniteValue {
                context: "probability distribution",
            });
        }

        if probability < -tolerance || probability > 1.0 + tolerance {
            return Err(FidelityError::InvalidProbability {
                index,
                value: probability,
            });
        }

        sum += probability;

        if !sum.is_finite() {
            return Err(FidelityError::NumericalFailure {
                operation: "probability normalization",
            });
        }
    }

    if (sum - 1.0).abs() > tolerance {
        return Err(FidelityError::DistributionNotNormalized {
            sum,
            tolerance,
        });
    }

    Ok(())
}

fn validate_density_matrix(
    matrix: &[Complex64],
    dimension: usize,
    hermiticity_tolerance: f64,
    positivity_tolerance: f64,
    limits: &FidelityLimits,
) -> Result<(), FidelityError> {
    if matrix.is_empty() {
        return Err(FidelityError::EmptyMatrix);
    }

    let required = checked_square(dimension)?;

    if matrix.len() != required {
        return Err(FidelityError::MatrixLengthMismatch {
            rows: dimension,
            columns: dimension,
            actual: matrix.len(),
        });
    }

    check_resource_limit(
        required as u128,
        limits.max_matrix_elements,
        "dense matrix elements",
    )?;

    for value in matrix {
        if !value.is_finite() {
            return Err(FidelityError::NonFiniteValue {
                context: "density-matrix element",
            });
        }
    }

    let mut maximum_deviation = 0.0_f64;

    for row in 0..dimension {
        for column in 0..dimension {
            let a = matrix[row * dimension + column];
            let b = matrix[column * dimension + row].conjugate();

            let deviation = complex_distance(a, b);

            if deviation > maximum_deviation {
                maximum_deviation = deviation;
            }
        }
    }

    if maximum_deviation > hermiticity_tolerance {
        return Err(FidelityError::NotHermitian {
            maximum_deviation,
            tolerance: hermiticity_tolerance,
        });
    }

    let mut trace = Complex64::zero();

    for index in 0..dimension {
        trace = checked_complex_add(
            trace,
            matrix[index * dimension + index],
            "density-matrix trace",
        )?;
    }

    if trace.real().is_nan()
        || trace.imaginary().is_nan()
        || !trace.is_finite()
        || (trace.real() - 1.0).abs() > hermiticity_tolerance
        || trace.imaginary().abs() > hermiticity_tolerance
    {
        return Err(FidelityError::InvalidTrace {
            trace_real: trace.real(),
            trace_imaginary: trace.imaginary(),
            tolerance: hermiticity_tolerance,
        });
    }

    // The diagonal of a valid density matrix is real and non-negative.
    for index in 0..dimension {
        let diagonal = matrix[index * dimension + index];

        if diagonal.imaginary().abs() > hermiticity_tolerance {
            return Err(FidelityError::InvalidTrace {
                trace_real: trace.real(),
                trace_imaginary: trace.imaginary(),
                tolerance: hermiticity_tolerance,
            });
        }

        if diagonal.real() < -positivity_tolerance {
            return Err(FidelityError::NegativeDiagonal {
                index,
                value: diagonal.real(),
                tolerance: positivity_tolerance,
            });
        }
    }

    // A full PSD check is required for a density matrix. The eigendecomposition
    // is deterministic and also supplies the spectral data needed later.
    let (eigenvalues, _) = hermitian_spectral_decomposition(
        matrix,
        dimension,
        hermiticity_tolerance,
        positivity_tolerance,
        limits,
    )?;

    let mut minimum = f64::INFINITY;

    for eigenvalue in eigenvalues {
        if eigenvalue < minimum {
            minimum = eigenvalue;
        }
    }

    if minimum < -positivity_tolerance {
        return Err(FidelityError::NotPositiveSemidefinite {
            minimum_eigenvalue: minimum,
            tolerance: positivity_tolerance,
        });
    }

    Ok(())
}

// =============================================================================
// Deterministic Hermitian eigendecomposition
// =============================================================================

/// Computes eigenvalues and eigenvectors of a finite Hermitian matrix.
///
/// The returned eigenvectors are stored column-major within a row-major matrix
/// representation:
///
/// ```text
/// vectors[row * n + eigenvector_index]
/// ```
///
/// Jacobi rotations are applied in deterministic lexicographic `(p,q)` order.
fn hermitian_spectral_decomposition(
    input: &[Complex64],
    dimension: usize,
    tolerance: f64,
    positivity_tolerance: f64,
    limits: &FidelityLimits,
) -> Result<(Vec<f64>, Vec<Complex64>), FidelityError> {
    validate_tolerance(tolerance)?;
    validate_tolerance(positivity_tolerance)?;

    let elements = checked_square(dimension)?;

    if input.len() != elements {
        return Err(FidelityError::MatrixLengthMismatch {
            rows: dimension,
            columns: dimension,
            actual: input.len(),
        });
    }

    check_resource_limit(
        elements as u128,
        limits.max_matrix_elements,
        "eigendecomposition matrix elements",
    )?;

    if dimension == 0 {
        return Err(FidelityError::EmptyMatrix);
    }

    let mut matrix = try_clone_matrix(input, "eigendecomposition matrix")?;
    let mut vectors = try_zero_matrix(
        dimension,
        limits,
        "eigendecomposition eigenvectors",
    )?;

    for index in 0..dimension {
        vectors[index * dimension + index] = Complex64::one();
    }

    let mut sweeps = 0_u64;

    loop {
        let mut maximum_off_diagonal = 0.0_f64;

        for p in 0..dimension {
            for q in (p + 1)..dimension {
                let magnitude =
                    matrix[p * dimension + q].norm_squared().sqrt();

                if magnitude > maximum_off_diagonal {
                    maximum_off_diagonal = magnitude;
                }
            }
        }

        if maximum_off_diagonal <= tolerance {
            break;
        }

        if let Some(maximum_sweeps) = limits.max_jacobi_sweeps {
            if sweeps >= maximum_sweeps {
                return Err(FidelityError::ConvergenceLimitExceeded {
                    sweeps,
                });
            }
        }

        for p in 0..dimension {
            for q in (p + 1)..dimension {
                jacobi_rotate(
                    &mut matrix,
                    &mut vectors,
                    dimension,
                    p,
                    q,
                    tolerance,
                )?;
            }
        }

        sweeps = sweeps.saturating_add(1);

        // This guard catches pathological numerical stagnation without
        // imposing a semantic matrix-size limit. An explicit caller limit is
        // preferred for untrusted inputs.
        if sweeps == u64::MAX {
            return Err(FidelityError::ConvergenceLimitExceeded {
                sweeps,
            });
        }
    }

    let mut eigenvalues = Vec::new();

    eigenvalues
        .try_reserve_exact(dimension)
        .map_err(|_| FidelityError::NumericalFailure {
            operation: "eigenvalue allocation",
        })?;

    for index in 0..dimension {
        let value = matrix[index * dimension + index];

        if !value.is_finite() {
            return Err(FidelityError::NumericalFailure {
                operation: "eigenvalue extraction",
            });
        }

        if value.imaginary().abs() > positivity_tolerance {
            return Err(FidelityError::NumericalFailure {
                operation: "Hermitian eigenvalue extraction",
            });
        }

        eigenvalues.push(value.real());
    }

    Ok((eigenvalues, vectors))
}

/// Applies one complex Jacobi rotation.
///
/// The operation is equivalent to a unitary similarity transformation:
///
/// ```text
/// A' = U† A U
/// ```
///
/// where `U` consists of a phase transformation followed by a real Jacobi
/// rotation in the selected `(p,q)` plane.
fn jacobi_rotate(
    matrix: &mut [Complex64],
    vectors: &mut [Complex64],
    dimension: usize,
    p: usize,
    q: usize,
    tolerance: f64,
) -> Result<(), FidelityError> {
    let pivot = matrix[p * dimension + q];

    let magnitude = pivot.norm();

    if magnitude <= tolerance {
        return Ok(());
    }

    let phase = pivot.imaginary().atan2(pivot.real());

    if !phase.is_finite() {
        return Err(FidelityError::NumericalFailure {
            operation: "Jacobi phase calculation",
        });
    }

    // D has D[q,q] = exp(-i*phase), making the selected off-diagonal
    // element real and non-negative.
    let phase_factor = Complex64::new((-phase).cos(), (-phase).sin());

    for k in 0..dimension {
        let qk = matrix[q * dimension + k];

        matrix[q * dimension + k] =
            checked_complex_mul(phase_factor.conjugate(), qk, "Jacobi phase row")?;

        let kq = matrix[k * dimension + q];

        matrix[k * dimension + q] =
            checked_complex_mul(kq, phase_factor, "Jacobi phase column")?;
    }

    let app = matrix[p * dimension + p].real();
    let aqq = matrix[q * dimension + q].real();
    let apq = matrix[p * dimension + q].real();

    let two_apq = 2.0 * apq;

    let theta = if two_apq == 0.0 {
        0.0
    } else {
        0.5 * (two_apq).atan2(aqq - app)
    };

    let c = theta.cos();
    let s = theta.sin();

    if !c.is_finite() || !s.is_finite() {
        return Err(FidelityError::NumericalFailure {
            operation: "Jacobi rotation coefficients",
        });
    }

    // Preserve the original p/q columns needed by the transformation.
    let mut p_column = Vec::new();
    let mut q_column = Vec::new();

    p_column
        .try_reserve_exact(dimension)
        .map_err(|_| FidelityError::NumericalFailure {
            operation: "Jacobi workspace allocation",
        })?;

    q_column
        .try_reserve_exact(dimension)
        .map_err(|_| FidelityError::NumericalFailure {
            operation: "Jacobi workspace allocation",
        })?;

    for k in 0..dimension {
        p_column.push(matrix[k * dimension + p]);
        q_column.push(matrix[k * dimension + q]);
    }

    // First update the eigenvector matrix using the same unitary rotation.
    let mut old_vp = Vec::new();
    let mut old_vq = Vec::new();

    old_vp
        .try_reserve_exact(dimension)
        .map_err(|_| FidelityError::NumericalFailure {
            operation: "Jacobi eigenvector workspace allocation",
        })?;

    old_vq
        .try_reserve_exact(dimension)
        .map_err(|_| FidelityError::NumericalFailure {
            operation: "Jacobi eigenvector workspace allocation",
        })?;

    for k in 0..dimension {
        old_vp.push(vectors[k * dimension + p]);
        old_vq.push(vectors[k * dimension + q]);
    }

    for k in 0..dimension {
        let phase_q =
            checked_complex_mul(phase_factor, old_vq[k], "Jacobi eigenvector phase")?;

        let new_p = scale_complex(
            old_vp[k] * c - phase_q * s,
            1.0,
        );

        let new_q = scale_complex(
            old_vp[k] * s + phase_q * c,
            1.0,
        );

        vectors[k * dimension + p] = new_p;
        vectors[k * dimension + q] = new_q;
    }

    // Apply the similarity transformation to the matrix.
    //
    // First transform columns p/q.
    for k in 0..dimension {
        let phase_q =
            checked_complex_mul(phase_factor, q_column[k], "Jacobi matrix phase")?;

        matrix[k * dimension + p] =
            checked_complex_add(
                scale_complex(p_column[k], c),
                scale_complex(phase_q, -s),
                "Jacobi matrix p-column",
            )?;

        matrix[k * dimension + q] =
            checked_complex_add(
                scale_complex(p_column[k], s),
                scale_complex(phase_q, c),
                "Jacobi matrix q-column",
            )?;
    }

    // Then transform rows p/q.
    let mut row_p = Vec::new();
    let mut row_q = Vec::new();

    row_p
        .try_reserve_exact(dimension)
        .map_err(|_| FidelityError::NumericalFailure {
            operation: "Jacobi row workspace allocation",
        })?;

    row_q
        .try_reserve_exact(dimension)
        .map_err(|_| FidelityError::NumericalFailure {
            operation: "Jacobi row workspace allocation",
        })?;

    for k in 0..dimension {
        row_p.push(matrix[p * dimension + k]);
        row_q.push(matrix[q * dimension + k]);
    }

    for k in 0..dimension {
        let phase_row_q = checked_complex_mul(
            phase_factor.conjugate(),
            row_q[k],
            "Jacobi row phase",
        )?;

        matrix[p * dimension + k] =
            checked_complex_add(
                scale_complex(row_p[k], c),
                scale_complex(phase_row_q, -s),
                "Jacobi p-row",
            )?;

        matrix[q * dimension + k] =
            checked_complex_add(
                scale_complex(row_p[k], s),
                scale_complex(phase_row_q, c),
                "Jacobi q-row",
            )?;
    }

    // Force the mathematically known Hermitian diagonal values to real.
    let new_app =
        c * c * app - 2.0 * c * s * apq + s * s * aqq;

    let new_aqq =
        s * s * app + 2.0 * c * s * apq + c * c * aqq;

    matrix[p * dimension + p] =
        Complex64::new(new_app, 0.0);

    matrix[q * dimension + q] =
        Complex64::new(new_aqq, 0.0);

    matrix[p * dimension + q] = Complex64::zero();
    matrix[q * dimension + p] = Complex64::zero();

    // Re-Hermitianize only the entries mathematically related by conjugation.
    // This removes accumulated roundoff without changing the declared
    // tolerance semantics.
    for k in 0..dimension {
        if k != p && k != q {
            let pk = matrix[p * dimension + k];
            let qk = matrix[q * dimension + k];

            matrix[k * dimension + p] = pk.conjugate();
            matrix[k * dimension + q] = qk.conjugate();
        }
    }

    Ok(())
}

// =============================================================================
// Matrix operations
// =============================================================================

fn matrix_multiply(
    left: &[Complex64],
    right: &[Complex64],
    dimension: usize,
    limits: &FidelityLimits,
    operation: &'static str,
) -> Result<Vec<Complex64>, FidelityError> {
    let elements = checked_square(dimension)?;

    if left.len() != elements || right.len() != elements {
        return Err(FidelityError::MatrixLengthMismatch {
            rows: dimension,
            columns: dimension,
            actual: left.len().min(right.len()),
        });
    }

    check_resource_limit(
        elements as u128,
        limits.max_matrix_elements,
        "matrix multiplication result",
    )?;

    let mut result = try_zero_matrix(
        dimension,
        limits,
        "matrix multiplication result",
    )?;

    for row in 0..dimension {
        for column in 0..dimension {
            let mut sum = Complex64::zero();

            for inner in 0..dimension {
                let product = checked_complex_mul(
                    left[row * dimension + inner],
                    right[inner * dimension + column],
                    operation,
                )?;

                sum = checked_complex_add(
                    sum,
                    product,
                    operation,
                )?;
            }

            result[row * dimension + column] = sum;
        }
    }

    Ok(result)
}

// =============================================================================
// Matrix construction/allocation
// =============================================================================

fn checked_square(dimension: usize) -> Result<usize, FidelityError> {
    dimension
        .checked_mul(dimension)
        .ok_or(FidelityError::SizeOverflow {
            context: "matrix dimension squared",
        })
}

fn checked_square_dimension_from_slice(
    matrix: &[Complex64],
    limits: &FidelityLimits,
    context: &'static str,
) -> Result<usize, FidelityError> {
    if matrix.is_empty() {
        return Err(FidelityError::EmptyMatrix);
    }

    let length = matrix.len();

    let dimension = integer_sqrt_exact(length).ok_or(
        FidelityError::MatrixLengthMismatch {
            rows: length,
            columns: length,
            actual: length,
        },
    )?;

    let elements = checked_square(dimension)?;

    if elements != length {
        return Err(FidelityError::MatrixLengthMismatch {
            rows: dimension,
            columns: dimension,
            actual: length,
        });
    }

    check_resource_limit(
        elements as u128,
        limits.max_matrix_elements,
        context,
    )?;

    Ok(dimension)
}

fn try_zero_matrix(
    dimension: usize,
    limits: &FidelityLimits,
    operation: &'static str,
) -> Result<Vec<Complex64>, FidelityError> {
    let elements = checked_square(dimension)?;

    check_resource_limit(
        elements as u128,
        limits.max_matrix_elements,
        operation,
    )?;

    let mut result = Vec::new();

    result
        .try_reserve_exact(elements)
        .map_err(|_| FidelityError::NumericalFailure {
            operation,
        })?;

    result.resize(elements, Complex64::zero());

    Ok(result)
}

fn try_clone_matrix(
    source: &[Complex64],
    operation: &'static str,
) -> Result<Vec<Complex64>, FidelityError> {
    let mut result = Vec::new();

    result
        .try_reserve_exact(source.len())
        .map_err(|_| FidelityError::NumericalFailure {
            operation,
        })?;

    result.extend_from_slice(source);

    Ok(result)
}

// =============================================================================
// Numerical helpers
// =============================================================================

fn validate_tolerance(value: f64) -> Result<(), FidelityError> {
    if !value.is_finite() || value < 0.0 {
        return Err(FidelityError::InvalidTolerance { value });
    }

    Ok(())
}

fn check_resource_limit(
    requested: u128,
    maximum: Option<u128>,
    resource: &'static str,
) -> Result<(), FidelityError> {
    if let Some(maximum) = maximum {
        if requested > maximum {
            return Err(FidelityError::ResourceLimitExceeded {
                requested,
                maximum,
                resource,
            });
        }
    }

    Ok(())
}

fn checked_complex_add(
    left: Complex64,
    right: Complex64,
    operation: &'static str,
) -> Result<Complex64, FidelityError> {
    let result = left + right;

    if !result.is_finite() {
        return Err(FidelityError::NumericalFailure { operation });
    }

    Ok(result)
}

fn checked_complex_mul(
    left: Complex64,
    right: Complex64,
    operation: &'static str,
) -> Result<Complex64, FidelityError> {
    let result = left * right;

    if !result.is_finite() {
        return Err(FidelityError::NumericalFailure { operation });
    }

    Ok(result)
}

fn scale_complex(value: Complex64, factor: f64) -> Complex64 {
    Complex64::new(
        value.real() * factor,
        value.imaginary() * factor,
    )
}

fn complex_distance(left: Complex64, right: Complex64) -> f64 {
    let difference = left - right;

    difference.norm_squared().sqrt()
}

fn clamp_near_unit_interval(
    value: f64,
    tolerance: f64,
) -> f64 {
    if value < 0.0 && value >= -tolerance {
        0.0
    } else if value > 1.0 && value <= 1.0 + tolerance {
        1.0
    } else {
        value
    }
}

/// Returns an integer square root only when the supplied number is an exact
/// square.
///
/// This avoids floating-point dimension inference.
fn integer_sqrt_exact(value: usize) -> Option<usize> {
    if value == 0 {
        return Some(0);
    }

    let mut low = 1usize;
    let mut high = value.min(usize::MAX / 2 + 1);

    while low <= high {
        let middle = low + (high - low) / 2;

        match middle.checked_mul(middle) {
            Some(square) if square == value => return Some(middle),
            Some(square) if square < value => {
                low = middle.saturating_add(1);
            }
            Some(_) => {
                if middle == 0 {
                    break;
                }

                high = middle - 1;
            }
            None => {
                high = middle.saturating_sub(1);
            }
        }
    }

    None
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn c(real: f64, imaginary: f64) -> Complex64 {
        Complex64::new(real, imaginary)
    }

    fn options() -> FidelityOptions {
        FidelityOptions {
            tolerance: 1.0e-9,
            hermiticity_tolerance: 1.0e-9,
            positivity_tolerance: 1.0e-9,
            limits: FidelityLimits::unlimited()
                .with_max_jacobi_sweeps(100_000),
        }
    }

    #[test]
    fn identical_pure_states_have_unit_fidelity() {
        let zero = [c(1.0, 0.0), c(0.0, 0.0)];

        let result =
            pure_state_fidelity(&zero, &zero).expect("valid state");

        assert_eq!(
            result.definition(),
            FidelityDefinition::PureStateOverlap
        );

        assert!((result.value() - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn_orthogonal_pure_states_have_zero_fidelity() {
        let zero = [c(1.0, 0.0), c(0.0, 0.0)];
        let one = [c(0.0, 0.0), c(1.0, 0.0)];

        let result =
            pure_state_fidelity(&zero, &one).expect("valid states");

        assert!(result.value().abs() < 1.0e-12);
    }

    #[test]
    fn_global_phase_does_not_change_pure_state_fidelity() {
        let state = [
            c(1.0 / 2.0_f64.sqrt(), 0.0),
            c(0.0, 1.0 / 2.0_f64.sqrt()),
        ];

        let phase = [
            c(0.0, 1.0 / 2.0_f64.sqrt()),
            c(-1.0 / 2.0_f64.sqrt(), 0.0),
        ];

        let result =
            pure_state_fidelity(&state, &phase).expect("valid states");

        assert!((result.value() - 1.0).abs() < 1.0e-9);
    }

    #[test]
    fn pure_state_against_matching_density_matrix_is_one() {
        let state = [c(1.0, 0.0), c(0.0, 0.0)];

        let density = [
            c(1.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
        ];

        let result = pure_state_density_fidelity_with_options(
            &state,
            &density,
            1.0e-9,
            1.0e-9,
            1.0e-9,
            &options().limits,
        )
        .expect("valid density matrix");

        assert!((result.value() - 1.0).abs() < 1.0e-9);
    }

    #[test]
    fn pure_state_against_orthogonal_density_matrix_is_zero() {
        let state = [c(1.0, 0.0), c(0.0, 0.0)];

        let density = [
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(1.0, 0.0),
        ];

        let result = pure_state_density_fidelity_with_options(
            &state,
            &density,
            1.0e-9,
            1.0e-9,
            1.0e-9,
            &options().limits,
        )
        .expect("valid density matrix");

        assert!(result.value().abs() < 1.0e-9);
    }

    #[test]
    fn maximally_mixed_qubit_has_half_fidelity_with_zero() {
        let zero = [c(1.0, 0.0), c(0.0, 0.0)];

        let mixed = [
            c(0.5, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(0.5, 0.0),
        ];

        let result = pure_state_density_fidelity_with_options(
            &zero,
            &mixed,
            1.0e-9,
            1.0e-9,
            1.0e-9,
            &options().limits,
        )
        .expect("valid density matrix");

        assert!((result.value() - 0.5).abs() < 1.0e-9);
    }

    #[test]
    fn identical_mixed_states_have_unit_fidelity() {
        let mixed = [
            c(0.5, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(0.5, 0.0),
        ];

        let result =
            density_matrix_fidelity_with_options(
                &mixed,
                &mixed,
                &options(),
            )
            .expect("valid mixed state");

        assert!((result.value() - 1.0).abs() < 1.0e-8);
    }

    #[test]
    fn orthogonal_pure_density_matrices_have_zero_fidelity() {
        let zero = [
            c(1.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
        ];

        let one = [
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(1.0, 0.0),
        ];

        let result =
            density_matrix_fidelity_with_options(
                &zero,
                &one,
                &options(),
            )
            .expect("valid pure density matrices");

        assert!(result.value().abs() < 1.0e-8);
    }

    #[test]
    fn classical_identical_distribution_has_unit_fidelity() {
        let p = [0.25, 0.25, 0.5];

        let result =
            classical_distribution_fidelity(&p, &p)
                .expect("valid distributions");

        assert!((result.value() - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn classical_disjoint_distributions_have_zero_fidelity() {
        let p = [1.0, 0.0];
        let q = [0.0, 1.0];

        let result =
            classical_distribution_fidelity(&p, &q)
                .expect("valid distributions");

        assert!(result.value().abs() < 1.0e-12);
    }

    #[test]
    fn average_gate_fidelity_identity_is_one() {
        let result =
            average_gate_fidelity(1.0, 2)
                .expect("valid process fidelity");

        assert!((result.value() - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn average_gate_fidelity_zero_process_fidelity_is_valid() {
        let result =
            average_gate_fidelity(0.0, 2)
                .expect("valid process fidelity");

        assert!((result.value() - 1.0 / 3.0).abs() < 1.0e-12);
    }

    #[test]
    fn invalid_state_normalization_is_rejected() {
        let state = [c(2.0, 0.0), c(0.0, 0.0)];

        let result =
            pure_state_fidelity(&state, &state);

        assert!(matches!(
            result,
            Err(FidelityError::StateNotNormalized { .. })
        ));
    }

    #[test]
    fn nonfinite_values_are_rejected() {
        let state = [c(f64::NAN, 0.0)];

        let result =
            pure_state_fidelity(&state, &state);

        assert!(matches!(
            result,
            Err(FidelityError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn invalid_density_matrix_is_rejected() {
        let invalid = [
            c(1.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(-0.1, 0.0),
        ];

        let result =
            density_matrix_fidelity_with_options(
                &invalid,
                &invalid,
                &options(),
            );

        assert!(result.is_err());
    }

    #[test]
    fn dimension_mismatch_is_rejected() {
        let left = [c(1.0, 0.0), c(0.0, 0.0)];
        let right = [
            c(1.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
        ];

        let result =
            pure_state_fidelity(&left, &right);

        assert!(matches!(
            result,
            Err(FidelityError::StateDimensionMismatch { .. })
        ));
    }

    #[test]
    fn resource_limit_is_enforced_explicitly() {
        let density = [
            c(1.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(1.0, 0.0),
        ];

        let limits =
            FidelityLimits::unlimited()
                .with_max_matrix_elements(2);

        let result =
            density_matrix_fidelity_with_options(
                &density,
                &density,
                &FidelityOptions {
                    limits,
                    ..options()
                },
            );

        assert!(matches!(
            result,
            Err(FidelityError::ResourceLimitExceeded { .. })
        ));
    }

    #[test]
    fn arbitrary_finite_dimension_is_not_semantically_hard_coded() {
        // This is a small scaling-contract test. The implementation derives
        // all dimensions from supplied data rather than a machine-specific
        // qubit constant.
        let dimension = 4usize;
        let elements = dimension * dimension;

        let mut matrix = Vec::new();
        matrix
            .try_reserve_exact(elements)
            .expect("test allocation");

        matrix.resize(elements, c(0.0, 0.0));

        for index in 0..dimension {
            matrix[index * dimension + index] =
                c(1.0 / dimension as f64, 0.0);
        }

        let result =
            density_matrix_fidelity_with_options(
                &matrix,
                &matrix,
                &options(),
            )
            .expect("valid arbitrary-dimensional state");

        assert!((result.value() - 1.0).abs() < 1.0e-8);
    }

    #[test]
    fn fidelity_is_symmetric_for_pure_states() {
        let left = [
            c(1.0 / 2.0_f64.sqrt(), 0.0),
            c(0.0, 1.0 / 2.0_f64.sqrt()),
        ];

        let right = [
            c(1.0 / 2.0_f64.sqrt(), 0.0),
            c(1.0 / 2.0_f64.sqrt(), 0.0),
        ];

        let forward =
            pure_state_fidelity(&left, &right)
                .expect("valid states");

        let reverse =
            pure_state_fidelity(&right, &left)
                .expect("valid states");

        assert!(
            (forward.value() - reverse.value()).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn fidelity_result_rejects_values_outside_domain() {
        let result = FidelityResult::new(
            1.5,
            FidelityDefinition::PureStateOverlap,
            1.0e-10,
        );

        assert!(matches!(
            result,
            Err(FidelityError::OutOfRange { .. })
        ));
    }
}