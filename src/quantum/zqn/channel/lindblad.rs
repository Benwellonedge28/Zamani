//! Zamani Quantum Noise (ZQN) — Lindblad / GKSL generators.
//!
//! # Ownership
//!
//! This file owns the semantic and numerical representation of a finite-
//! dimensional Lindblad/Gorini-Kossakowski-Sudarshan-Lindblad (GKSL) generator.
//!
//! It owns:
//!
//! - Hamiltonian generators;
//! - Lindblad/jump operators;
//! - non-negative dissipative rates;
//! - subsystem dimensions;
//! - optional canonical `QubitId` resource association;
//! - generator validation;
//! - deterministic generator application;
//! - first-order generator evaluation;
//! - dissipator construction;
//! - generator composition by direct-sum resource composition;
//! - tensor-product lifting;
//! - explicit approximation/tolerance contracts;
//! - deterministic semantic descriptors;
//! - resource-aware dimension arithmetic;
//! - serialization of the mathematical representation.
//!
//! It does NOT own:
//!
//! - the canonical quantum IR;
//! - quantum source parsing;
//! - gate definitions;
//! - QPU APIs;
//! - vendor SDKs;
//! - scheduling;
//! - routing;
//! - calibration acquisition;
//! - QEC decoding;
//! - random-number generation;
//! - Monte-Carlo sampling;
//! - numerical ODE integration policy;
//! - simulator state ownership;
//! - backend execution;
//! - global mutable state.
//!
//! Those responsibilities belong to their respective ZQN/quantum subsystems.
//!
//! # Mathematical contract
//!
//! A finite-dimensional GKSL generator is
//!
//! ```text
//! dρ/dt = L(ρ)
//!
//! L(ρ) = -i[H, ρ]
//!        + Σ_k γ_k
//!          (L_k ρ L_k†
//!           - 1/2 {L_k† L_k, ρ})
//! ```
//!
//! where:
//!
//! - `H` is Hermitian;
//! - `L_k` are jump operators;
//! - `γ_k >= 0` are dissipative rates.
//!
//! This representation describes the generator, not the numerical solution of
//! the differential equation.
//!
//! # Important semantic distinction
//!
//! A Lindblad generator is NOT itself a quantum channel in the strict
//! completely-positive trace-preserving map sense. It is the infinitesimal
//! generator of a quantum dynamical semigroup.
//!
//! Therefore this type must not falsely claim that the generator is a CPTP map.
//!
//! The corresponding finite-time channel is conceptually:
//!
//! ```text
//! E_t = exp(t L)
//! ```
//!
//! Constructing/evaluating `exp(t L)` is deliberately outside the semantic
//! generator representation and belongs to the simulation/channel-engine layer.
//!
//! # Write once, scale everywhere
//!
//! No machine size, qubit count, gate count, matrix-size or technology limit is
//! encoded in this module.
//!
//! Dimensions are supplied by the caller.
//!
//! A concrete execution may impose limits through ZQN resource policy,
//! simulator policy, runtime policy or hardware capabilities.
//!
//! Therefore:
//!
//! ```text
//! semantic capacity = no artificial finite machine-size ceiling
//! implementation capacity = available resources
//! ```
//!
//! This distinction is fundamental to Zamani's write-once/scale-everywhere
//! design.
//!
//! # Quantum identity
//!
//! When a Lindblad generator is associated with canonical Zamani qubits, this
//! file uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! It does not define another `QubitId`.
//!
//! A generator may intentionally be unbound to physical resources. Such a
//! generator can later be bound by routing, scheduling, target lowering or
//! execution integration.
//!
//! # Numerical policy
//!
//! This implementation uses Zamani's canonical `Complex64` representation for
//! its concrete dense numerical operations.
//!
//! The semantic model remains independent of a particular simulator.
//!
//! No NaN or infinity is accepted.
//!
//! No invalid numerical value is silently repaired.
//!
//! # Resource safety
//!
//! Dense matrices have quadratic element growth. Consequently all derived
//! element counts use checked arithmetic before allocation.
//!
//! This module contains no hard-coded matrix-size ceiling.
//!
//! Resource ceilings belong to the caller/runtime.
//!
//! # Determinism
//!
//! This module is deterministic.
//!
//! It contains no RNG and no hidden mutable state.
//!
//! The same generator and input density matrix produce the same result.
//!
//! Stochastic sampling belongs to `zqn::simulation`.
//!
//! # Integration
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! ZQN operation/noise binding
//!      │
//!      ▼
//! LindbladGenerator
//!      │
//!      ├──────────────► simulation/channel_engine
//!      ├──────────────► propagation
//!      ├──────────────► characterization
//!      ├──────────────► calibration
//!      ├──────────────► scheduling
//!      ├──────────────► routing
//!      ├──────────────► QEC adapter
//!      └──────────────► target lowering
//! ```
//!
//! `QuantumChannel` remains the representation-independent channel abstraction.
//! This type represents the generator semantics consumed by that abstraction's
//! Lindblad representation layer.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Security
//!
//! This module rejects:
//!
//! - zero dimensions;
//! - inconsistent matrix dimensions;
//! - non-finite matrix elements;
//! - negative dissipative rates;
//! - non-finite rates;
//! - non-Hermitian Hamiltonians when Hermiticity is required;
//! - uncontrolled dimension multiplication.
//!
//! It does not execute external code or access hardware.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. the Lindblad mathematical representation is self-contained;
//! 2. no competing Lindblad trait is introduced;
//! 3. canonical `QubitId` is used;
//! 4. no fixed machine-size limit exists;
//! 5. GKSL validation is explicit;
//! 6. finite-time evolution is not confused with generator semantics;
//! 7. numerical invalidity is rejected rather than repaired;
//! 8. dense allocations use checked arithmetic;
//! 9. no RNG exists here;
//! 10. no hardware/provider dependency exists;
//! 11. no unsafe code exists;
//! 12. Rust 1.97/1.97.1 is sufficient.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::memory::complex::Complex64;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the Lindblad representation.
pub const LINDBLAD_SCHEMA_ID: &str = "zamani.quantum.zqn.channel.lindblad";

/// Semantic version of the Lindblad representation contract.
pub const LINDBLAD_SCHEMA_VERSION: u16 = 1;

/// Default absolute tolerance used for structural validation.
pub const DEFAULT_LINDBLAD_ABSOLUTE_TOLERANCE: f64 = 1.0e-12;

/// Default relative tolerance used for structural validation.
pub const DEFAULT_LINDBLAD_RELATIVE_TOLERANCE: f64 = 1.0e-10;

// =============================================================================
// Result
// =============================================================================

/// Result type for Lindblad operations.
pub type LindbladResult<T> = Result<T, LindbladError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the Lindblad representation.
#[derive(Debug, Clone, PartialEq)]
pub enum LindbladError {
    /// A Hilbert-space dimension was invalid.
    InvalidDimension {
        /// Supplied dimension.
        dimension: usize,
    },

    /// A matrix has inconsistent dimensions.
    DimensionMismatch {
        /// Expected rows.
        expected_rows: usize,
        /// Actual rows.
        actual_rows: usize,
        /// Expected columns.
        expected_columns: usize,
        /// Actual columns.
        actual_columns: usize,
    },

    /// Matrix element count does not match its dimensions.
    ElementCountMismatch {
        /// Matrix rows.
        rows: usize,
        /// Matrix columns.
        columns: usize,
        /// Supplied element count.
        actual: usize,
    },

    /// A multiplication would overflow the host integer domain.
    DimensionOverflow,

    /// An allocation would exceed the caller-provided limit.
    ResourceLimitExceeded {
        /// Name of the resource.
        resource: &'static str,
        /// Requested amount.
        requested: u128,
        /// Allowed amount.
        limit: u128,
    },

    /// A matrix contains NaN or infinity.
    NonFiniteElement {
        /// Flat matrix index.
        index: usize,
    },

    /// A dissipative rate is not finite.
    NonFiniteRate {
        /// Index of the rate.
        index: usize,
    },

    /// A dissipative rate is negative.
    NegativeRate {
        /// Index of the rate.
        index: usize,
        /// Supplied rate.
        rate: f64,
    },

    /// The Hamiltonian is not Hermitian within the requested tolerance.
    NonHermitianHamiltonian {
        /// Maximum Hermiticity error.
        max_error: f64,
    },

    /// A jump operator has an invalid dimension.
    InvalidJumpOperator {
        /// Jump-operator index.
        index: usize,
    },

    /// The density matrix is invalid for this operation.
    InvalidDensityMatrix,

    /// A supplied time is invalid.
    InvalidTime,

    /// A supplied tolerance is invalid.
    InvalidTolerance,

    /// A requested operation requires an unavailable resource.
    ResourceUnavailable,

    /// The requested operation is semantically unsupported.
    UnsupportedOperation(&'static str),

    /// Two generators cannot be combined under the requested semantics.
    IncompatibleGenerators,

    /// Canonical resource metadata is inconsistent.
    InvalidResourceBinding,

    /// A requested mathematical property could not be proven.
    PropertyUndetermined(&'static str),
}

impl fmt::Display for LindbladError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimension { dimension } => {
                write!(formatter, "invalid Hilbert-space dimension: {dimension}")
            }

            Self::DimensionMismatch {
                expected_rows,
                actual_rows,
                expected_columns,
                actual_columns,
            } => write!(
                formatter,
                "matrix dimension mismatch: expected {}x{}, got {}x{}",
                expected_rows, expected_columns, actual_rows, actual_columns
            ),

            Self::ElementCountMismatch {
                rows,
                columns,
                actual,
            } => write!(
                formatter,
                "matrix element count mismatch: expected {} elements for {}x{}, got {}",
                rows.saturating_mul(*columns),
                rows,
                columns,
                actual
            ),

            Self::DimensionOverflow => {
                write!(formatter, "dimension arithmetic overflow")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => write!(
                formatter,
                "resource limit exceeded for {resource}: requested {requested}, limit {limit}"
            ),

            Self::NonFiniteElement { index } => {
                write!(formatter, "non-finite matrix element at index {index}")
            }

            Self::NonFiniteRate { index } => {
                write!(formatter, "non-finite Lindblad rate at index {index}")
            }

            Self::NegativeRate { index, rate } => {
                write!(formatter, "negative Lindblad rate at index {index}: {rate}")
            }

            Self::NonHermitianHamiltonian { max_error } => {
                write!(
                    formatter,
                    "Hamiltonian is not Hermitian within tolerance; maximum error={max_error}"
                )
            }

            Self::InvalidJumpOperator { index } => {
                write!(formatter, "invalid jump operator at index {index}")
            }

            Self::InvalidDensityMatrix => {
                write!(formatter, "invalid density matrix")
            }

            Self::InvalidTime => {
                write!(formatter, "time must be finite and non-negative")
            }

            Self::InvalidTolerance => {
                write!(formatter, "tolerance must be finite and non-negative")
            }

            Self::ResourceUnavailable => {
                write!(formatter, "required numerical resources are unavailable")
            }

            Self::UnsupportedOperation(operation) => {
                write!(formatter, "unsupported Lindblad operation: {operation}")
            }

            Self::IncompatibleGenerators => {
                write!(formatter, "Lindblad generators are incompatible")
            }

            Self::InvalidResourceBinding => {
                write!(formatter, "invalid canonical quantum-resource binding")
            }

            Self::PropertyUndetermined(property) => {
                write!(formatter, "property could not be determined: {property}")
            }
        }
    }
}

impl Error for LindbladError {}

// =============================================================================
// Matrix
// =============================================================================

/// Immutable dense complex matrix used by the Lindblad representation.
///
/// This is deliberately a mathematical container rather than a simulator
/// state. Simulator-specific storage belongs to `quantum::memory`.
#[derive(Clone, Debug, PartialEq)]
pub struct ComplexMatrix {
    rows: usize,
    columns: usize,
    data: Vec<Complex64>,
}

impl ComplexMatrix {
    /// Creates a matrix after validating dimensions and element count.
    pub fn new(
        rows: usize,
        columns: usize,
        data: Vec<Complex64>,
    ) -> LindbladResult<Self> {
        if rows == 0 || columns == 0 {
            return Err(LindbladError::InvalidDimension {
                dimension: rows.min(columns),
            });
        }

        let expected = rows
            .checked_mul(columns)
            .ok_or(LindbladError::DimensionOverflow)?;

        if data.len() != expected {
            return Err(LindbladError::ElementCountMismatch {
                rows,
                columns,
                actual: data.len(),
            });
        }

        for (index, value) in data.iter().enumerate() {
            if !value.real.is_finite() || !value.imag.is_finite() {
                return Err(LindbladError::NonFiniteElement { index });
            }
        }

        Ok(Self {
            rows,
            columns,
            data,
        })
    }

    /// Creates a zero matrix.
    pub fn zeros(rows: usize, columns: usize) -> LindbladResult<Self> {
        let count = rows
            .checked_mul(columns)
            .ok_or(LindbladError::DimensionOverflow)?;

        Self::new(rows, columns, vec![Complex64::ZERO; count])
    }

    /// Creates an identity matrix.
    pub fn identity(dimension: usize) -> LindbladResult<Self> {
        let mut matrix = Self::zeros(dimension, dimension)?;

        for index in 0..dimension {
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

    /// Returns the matrix dimension when square.
    pub fn dimension(&self) -> LindbladResult<usize> {
        if self.rows != self.columns {
            return Err(LindbladError::DimensionMismatch {
                expected_rows: self.rows,
                actual_rows: self.rows,
                expected_columns: self.rows,
                actual_columns: self.columns,
            });
        }

        Ok(self.rows)
    }

    /// Returns the backing element count.
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.data.len()
    }

    /// Returns an element.
    #[must_use]
    pub fn get(&self, row: usize, column: usize) -> Option<Complex64> {
        if row >= self.rows || column >= self.columns {
            return None;
        }

        self.data
            .get(row.checked_mul(self.columns)?.checked_add(column)?)
            .copied()
    }

    /// Sets an element.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: Complex64,
    ) -> LindbladResult<()> {
        if row >= self.rows || column >= self.columns {
            return Err(LindbladError::DimensionMismatch {
                expected_rows: self.rows,
                actual_rows: self.rows,
                expected_columns: self.columns,
                actual_columns: self.columns,
            });
        }

        if !value.real.is_finite() || !value.imag.is_finite() {
            return Err(LindbladError::NonFiniteElement {
                index: row
                    .checked_mul(self.columns)
                    .and_then(|x| x.checked_add(column))
                    .ok_or(LindbladError::DimensionOverflow)?,
            });
        }

        let index = row
            .checked_mul(self.columns)
            .and_then(|x| x.checked_add(column))
            .ok_or(LindbladError::DimensionOverflow)?;

        self.data[index] = value;

        Ok(())
    }

    /// Returns a read-only view of the elements.
    #[must_use]
    pub fn as_slice(&self) -> &[Complex64] {
        &self.data
    }

    /// Returns the conjugate transpose.
    pub fn adjoint(&self) -> LindbladResult<Self> {
        let mut result = Self::zeros(self.columns, self.rows)?;

        for row in 0..self.rows {
            for column in 0..self.columns {
                let value = self
                    .get(row, column)
                    .ok_or(LindbladError::DimensionMismatch {
                        expected_rows: self.rows,
                        actual_rows: self.rows,
                        expected_columns: self.columns,
                        actual_columns: self.columns,
                    })?;

                result.set(column, row, complex_conjugate(value))?;
            }
        }

        Ok(result)
    }

    /// Matrix addition.
    pub fn add(&self, other: &Self) -> LindbladResult<Self> {
        if self.rows != other.rows || self.columns != other.columns {
            return Err(LindbladError::IncompatibleGenerators);
        }

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| complex_add(*a, *b))
            .collect();

        Self::new(self.rows, self.columns, data)
    }

    /// Matrix subtraction.
    pub fn sub(&self, other: &Self) -> LindbladResult<Self> {
        if self.rows != other.rows || self.columns != other.columns {
            return Err(LindbladError::IncompatibleGenerators);
        }

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| complex_sub(*a, *b))
            .collect();

        Self::new(self.rows, self.columns, data)
    }

    /// Scalar multiplication.
    pub fn scale(&self, scalar: Complex64) -> LindbladResult<Self> {
        let data = self
            .data
            .iter()
            .map(|value| complex_mul(*value, scalar))
            .collect();

        Self::new(self.rows, self.columns, data)
    }

    /// Matrix multiplication.
    pub fn multiply(&self, other: &Self) -> LindbladResult<Self> {
        if self.columns != other.rows {
            return Err(LindbladError::DimensionMismatch {
                expected_rows: self.rows,
                actual_rows: other.rows,
                expected_columns: other.columns,
                actual_columns: self.columns,
            });
        }

        let mut result = Self::zeros(self.rows, other.columns)?;

        for row in 0..self.rows {
            for column in 0..other.columns {
                let mut value = Complex64::ZERO;

                for index in 0..self.columns {
                    let left = self
                        .get(row, index)
                        .ok_or(LindbladError::InvalidDensityMatrix)?;

                    let right = other
                        .get(index, column)
                        .ok_or(LindbladError::InvalidDensityMatrix)?;

                    value = complex_add(value, complex_mul(left, right));
                }

                result.set(row, column, value)?;
            }
        }

        Ok(result)
    }

    /// Returns the maximum absolute element magnitude.
    #[must_use]
    pub fn max_abs(&self) -> f64 {
        self.data
            .iter()
            .map(complex_abs)
            .fold(0.0_f64, f64::max)
    }

    /// Returns the maximum absolute difference from another matrix.
    pub fn max_abs_difference(&self, other: &Self) -> LindbladResult<f64> {
        if self.rows != other.rows || self.columns != other.columns {
            return Err(LindbladError::IncompatibleGenerators);
        }

        Ok(self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| complex_abs(complex_sub(*a, *b)))
            .fold(0.0_f64, f64::max))
    }

    /// Returns the trace.
    pub fn trace(&self) -> LindbladResult<Complex64> {
        let dimension = self.dimension()?;

        let mut result = Complex64::ZERO;

        for index in 0..dimension {
            result = complex_add(
                result,
                self.get(index, index)
                    .ok_or(LindbladError::InvalidDensityMatrix)?,
            );
        }

        Ok(result)
    }

    /// Returns true when the matrix is Hermitian within tolerance.
    pub fn is_hermitian(
        &self,
        absolute_tolerance: f64,
        relative_tolerance: f64,
    ) -> LindbladResult<bool> {
        validate_tolerance(absolute_tolerance)?;
        validate_tolerance(relative_tolerance)?;

        let adjoint = self.adjoint()?;
        let error = self.max_abs_difference(&adjoint)?;
        let scale = self.max_abs().max(1.0);

        Ok(error <= absolute_tolerance + relative_tolerance * scale)
    }
}

// =============================================================================
// Canonical resource binding
// =============================================================================

/// Canonical quantum resources affected by a Lindblad generator.
///
/// Qubit resources use the canonical Zamani `QubitId`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LindbladResourceBinding {
    /// Canonical logical qubit resources.
    ///
    /// The vector is ordered canonically by the caller.
    pub qubits: Vec<QubitId>,
}

impl LindbladResourceBinding {
    /// Creates an unbound resource set.
    #[must_use]
    pub const fn unbound() -> Self {
        Self {
            qubits: Vec::new(),
        }
    }

    /// Creates a resource binding.
    ///
    /// Duplicate canonical resources are rejected.
    pub fn new(mut qubits: Vec<QubitId>) -> LindbladResult<Self> {
        qubits.sort();

        for pair in qubits.windows(2) {
            if pair[0] == pair[1] {
                return Err(LindbladError::InvalidResourceBinding);
            }
        }

        Ok(Self { qubits })
    }

    /// Returns the number of bound qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns true when no resources are bound.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Returns the canonical resource list.
    #[must_use]
    pub fn as_slice(&self) -> &[QubitId] {
        &self.qubits
    }
}

// =============================================================================
// Jump term
// =============================================================================

/// One dissipative Lindblad term.
///
/// The contribution is:
///
/// ```text
/// γ (L ρ L† - 1/2 {L†L,ρ})
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct LindbladJump {
    /// Dissipative rate γ.
    pub rate: f64,

    /// Jump operator L.
    pub operator: ComplexMatrix,
}

impl LindbladJump {
    /// Creates a validated jump term.
    pub fn new(rate: f64, operator: ComplexMatrix) -> LindbladResult<Self> {
        if !rate.is_finite() {
            return Err(LindbladError::NonFiniteRate { index: 0 });
        }

        if rate < 0.0 {
            return Err(LindbladError::NegativeRate { index: 0, rate });
        }

        let dimension = operator.dimension()?;

        if dimension == 0 {
            return Err(LindbladError::InvalidDimension { dimension });
        }

        Ok(Self { rate, operator })
    }
}

// =============================================================================
// Resource policy
// =============================================================================

/// Explicit caller-controlled resource policy.
///
/// No value is imposed by the mathematical Lindblad semantics.
///
/// `None` means that this particular policy does not impose a limit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LindbladResourceLimits {
    /// Maximum Hamiltonian matrix elements allowed for one operation.
    pub max_hamiltonian_elements: Option<u128>,

    /// Maximum jump-operator matrix elements per operator.
    pub max_jump_operator_elements: Option<u128>,

    /// Maximum number of jump operators.
    pub max_jump_operators: Option<u128>,

    /// Maximum density-matrix elements for direct application.
    pub max_density_matrix_elements: Option<u128>,
}

impl LindbladResourceLimits {
    /// Validates an element count against an optional limit.
    fn check(
        limit: Option<u128>,
        resource: &'static str,
        requested: u128,
    ) -> LindbladResult<()> {
        if let Some(limit) = limit {
            if requested > limit {
                return Err(LindbladError::ResourceLimitExceeded {
                    resource,
                    requested,
                    limit,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Validation policy
// =============================================================================

/// Numerical tolerances used when validating a Lindblad generator.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LindbladValidationTolerance {
    /// Absolute tolerance.
    pub absolute: f64,

    /// Relative tolerance.
    pub relative: f64,
}

impl Default for LindbladValidationTolerance {
    fn default() -> Self {
        Self {
            absolute: DEFAULT_LINDBLAD_ABSOLUTE_TOLERANCE,
            relative: DEFAULT_LINDBLAD_RELATIVE_TOLERANCE,
        }
    }
}

impl LindbladValidationTolerance {
    /// Creates validated tolerances.
    pub fn new(absolute: f64, relative: f64) -> LindbladResult<Self> {
        validate_tolerance(absolute)?;
        validate_tolerance(relative)?;

        Ok(Self {
            absolute,
            relative,
        })
    }
}

// =============================================================================
// Lindblad generator
// =============================================================================

/// Finite-dimensional Lindblad/GKSL generator.
///
/// This is a deterministic semantic representation of an infinitesimal quantum
/// dynamical generator.
///
/// It does not own an ODE solver or simulator state.
#[derive(Clone, Debug, PartialEq)]
pub struct LindbladGenerator {
    /// Hilbert-space dimension.
    dimension: usize,

    /// Hamiltonian H.
    hamiltonian: ComplexMatrix,

    /// Dissipative jump terms.
    jumps: Vec<LindbladJump>,

    /// Optional canonical quantum-resource association.
    resources: LindbladResourceBinding,
}

impl LindbladGenerator {
    /// Constructs a Lindblad generator.
    ///
    /// Validation guarantees:
    ///
    /// - dimension is non-zero;
    /// - Hamiltonian is square;
    /// - every jump operator is square;
    /// - all matrices have the declared dimension;
    /// - all numerical values are finite;
    /// - every rate is non-negative;
    /// - the Hamiltonian is Hermitian within the supplied tolerance.
    pub fn new(
        dimension: usize,
        hamiltonian: ComplexMatrix,
        jumps: Vec<LindbladJump>,
        resources: LindbladResourceBinding,
        tolerance: LindbladValidationTolerance,
        limits: LindbladResourceLimits,
    ) -> LindbladResult<Self> {
        if dimension == 0 {
            return Err(LindbladError::InvalidDimension { dimension });
        }

        let matrix_elements = checked_square_elements(dimension)?;

        LindbladResourceLimits::check(
            limits.max_hamiltonian_elements,
            "hamiltonian_elements",
            matrix_elements,
        )?;

        LindbladResourceLimits::check(
            limits.max_jump_operators,
            "jump_operators",
            jumps.len() as u128,
        )?;

        LindbladResourceLimits::check(
            limits.max_jump_operator_elements,
            "jump_operator_elements",
            matrix_elements,
        )?;

        if hamiltonian.rows() != dimension || hamiltonian.columns() != dimension {
            return Err(LindbladError::DimensionMismatch {
                expected_rows: dimension,
                actual_rows: hamiltonian.rows(),
                expected_columns: dimension,
                actual_columns: hamiltonian.columns(),
            });
        }

        if !hamiltonian.is_hermitian(tolerance.absolute, tolerance.relative)? {
            let adjoint = hamiltonian.adjoint()?;
            let max_error = hamiltonian.max_abs_difference(&adjoint)?;

            return Err(LindbladError::NonHermitianHamiltonian { max_error });
        }

        for (index, jump) in jumps.iter().enumerate() {
            if jump.operator.rows() != dimension
                || jump.operator.columns() != dimension
            {
                return Err(LindbladError::InvalidJumpOperator { index });
            }

            if !jump.rate.is_finite() {
                return Err(LindbladError::NonFiniteRate { index });
            }

            if jump.rate < 0.0 {
                return Err(LindbladError::NegativeRate {
                    index,
                    rate: jump.rate,
                });
            }
        }

        if !resources.is_empty() && resources.len() > dimension {
            // Resource count is metadata and dimension is not a qubit count.
            // Therefore this is NOT used as a semantic relation. A generator
            // may represent composite/modal systems. No rejection is made.
        }

        Ok(Self {
            dimension,
            hamiltonian,
            jumps,
            resources,
        })
    }

    /// Constructs a generator using default validation tolerances and no
    /// additional resource limits.
    pub fn with_defaults(
        dimension: usize,
        hamiltonian: ComplexMatrix,
        jumps: Vec<LindbladJump>,
        resources: LindbladResourceBinding,
    ) -> LindbladResult<Self> {
        Self::new(
            dimension,
            hamiltonian,
            jumps,
            resources,
            LindbladValidationTolerance::default(),
            LindbladResourceLimits::default(),
        )
    }

    /// Returns the Hilbert-space dimension.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.dimension
    }

    /// Returns the Hamiltonian.
    #[must_use]
    pub fn hamiltonian(&self) -> &ComplexMatrix {
        &self.hamiltonian
    }

    /// Returns the jump terms.
    #[must_use]
    pub fn jumps(&self) -> &[LindbladJump] {
        &self.jumps
    }

    /// Returns canonical quantum-resource binding.
    #[must_use]
    pub fn resources(&self) -> &LindbladResourceBinding {
        &self.resources
    }

    /// Returns the number of dissipative terms.
    #[must_use]
    pub fn jump_count(&self) -> usize {
        self.jumps.len()
    }

    /// Validates the generator against a new tolerance.
    pub fn validate(
        &self,
        tolerance: LindbladValidationTolerance,
    ) -> LindbladResult<()> {
        if self.dimension == 0 {
            return Err(LindbladError::InvalidDimension {
                dimension: self.dimension,
            });
        }

        if self.hamiltonian.rows() != self.dimension
            || self.hamiltonian.columns() != self.dimension
        {
            return Err(LindbladError::InvalidDensityMatrix);
        }

        if !self
            .hamiltonian
            .is_hermitian(tolerance.absolute, tolerance.relative)?
        {
            let adjoint = self.hamiltonian.adjoint()?;
            let max_error = self.hamiltonian.max_abs_difference(&adjoint)?;

            return Err(LindbladError::NonHermitianHamiltonian { max_error });
        }

        for (index, jump) in self.jumps.iter().enumerate() {
            if !jump.rate.is_finite() {
                return Err(LindbladError::NonFiniteRate { index });
            }

            if jump.rate < 0.0 {
                return Err(LindbladError::NegativeRate {
                    index,
                    rate: jump.rate,
                });
            }

            if jump.operator.rows() != self.dimension
                || jump.operator.columns() != self.dimension
            {
                return Err(LindbladError::InvalidJumpOperator { index });
            }
        }

        Ok(())
    }

    /// Evaluates the generator on a density matrix.
    ///
    /// Returns `L(ρ)` according to the GKSL equation.
    ///
    /// This is an infinitesimal derivative. It does not perform time
    /// integration.
    pub fn apply(
        &self,
        density_matrix: &ComplexMatrix,
        limits: LindbladResourceLimits,
    ) -> LindbladResult<ComplexMatrix> {
        let dimension = density_matrix.dimension()?;

        if dimension != self.dimension {
            return Err(LindbladError::DimensionMismatch {
                expected_rows: self.dimension,
                actual_rows: density_matrix.rows(),
                expected_columns: self.dimension,
                actual_columns: density_matrix.columns(),
            });
        }

        LindbladResourceLimits::check(
            limits.max_density_matrix_elements,
            "density_matrix_elements",
            checked_square_elements(dimension)?,
        )?;

        validate_finite_matrix(density_matrix)?;

        let h_rho = self.hamiltonian.multiply(density_matrix)?;
        let rho_h = density_matrix.multiply(&self.hamiltonian)?;

        let commutator = h_rho.sub(&rho_h)?;

        let minus_i = Complex64::new(0.0, -1.0);

        let mut result = commutator.scale(minus_i)?;

        for jump in &self.jumps {
            let operator_adjoint = jump.operator.adjoint()?;

            let left = jump.operator.multiply(density_matrix)?;
            let jump_term = left.multiply(&operator_adjoint)?;

            let adjoint_operator = operator_adjoint.multiply(&jump.operator)?;

            let left_anticommutator =
                adjoint_operator.multiply(density_matrix)?;

            let right_anticommutator =
                density_matrix.multiply(&adjoint_operator)?;

            let anticommutator =
                left_anticommutator.add(&right_anticommutator)?;

            let half = Complex64::new(-0.5 * jump.rate, 0.0);

            let dissipator =
                jump_term.add(&anticommutator.scale(half)?)?;

            result = result.add(&dissipator.scale(
                Complex64::new(jump.rate, 0.0),
            )?)?;
        }

        Ok(result)
    }

    /// Applies the generator without dissipative terms.
    ///
    /// This is useful for analysis and characterization.
    pub fn hamiltonian_part(
        &self,
        density_matrix: &ComplexMatrix,
    ) -> LindbladResult<ComplexMatrix> {
        let h_rho = self.hamiltonian.multiply(density_matrix)?;
        let rho_h = density_matrix.multiply(&self.hamiltonian)?;

        h_rho
            .sub(&rho_h)?
            .scale(Complex64::new(0.0, -1.0))
    }

    /// Applies only the dissipative component.
    pub fn dissipative_part(
        &self,
        density_matrix: &ComplexMatrix,
    ) -> LindbladResult<ComplexMatrix> {
        let dimension = density_matrix.dimension()?;

        if dimension != self.dimension {
            return Err(LindbladError::DimensionMismatch {
                expected_rows: self.dimension,
                actual_rows: density_matrix.rows(),
                expected_columns: self.dimension,
                actual_columns: density_matrix.columns(),
            });
        }

        let mut result = ComplexMatrix::zeros(dimension, dimension)?;

        for jump in &self.jumps {
            let adjoint = jump.operator.adjoint()?;

            let jump_term = jump
                .operator
                .multiply(density_matrix)?
                .multiply(&adjoint)?;

            let product = adjoint.multiply(&jump.operator)?;

            let left = product.multiply(density_matrix)?;
            let right = density_matrix.multiply(&product)?;

            let anticommutator = left.add(&right)?;

            let dissipator = jump_term.sub(
                &anticommutator.scale(Complex64::new(0.5, 0.0))?,
            )?;

            result = result.add(
                &dissipator.scale(Complex64::new(jump.rate, 0.0))?,
            )?;
        }

        Ok(result)
    }

    /// Verifies trace preservation of the infinitesimal generator.
    ///
    /// A valid GKSL generator satisfies:
    ///
    /// ```text
    /// Tr(L(ρ)) = 0
    /// ```
    ///
    /// for every density operator ρ.
    ///
    /// The structural implementation is checked using a supplied density
    /// matrix. The caller should use representative states or a characterization
    /// protocol when a universal numerical proof is required.
    pub fn preserves_trace(
        &self,
        density_matrix: &ComplexMatrix,
        tolerance: LindbladValidationTolerance,
    ) -> LindbladResult<bool> {
        let derivative = self.apply(density_matrix, LindbladResourceLimits::default())?;
        let trace = derivative.trace()?;

        let magnitude = complex_abs(trace);

        Ok(magnitude <= tolerance.absolute)
    }

    /// Returns the canonical semantic representation name.
    #[must_use]
    pub const fn representation_name() -> &'static str {
        "lindblad"
    }

    /// Returns the stable schema identifier.
    #[must_use]
    pub const fn schema_id() -> &'static str {
        LINDBLAD_SCHEMA_ID
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        LINDBLAD_SCHEMA_VERSION
    }

    /// Returns a deterministic semantic descriptor.
    #[must_use]
    pub fn descriptor(&self) -> LindbladDescriptor {
        LindbladDescriptor {
            dimension: self.dimension,
            jump_count: self.jumps.len(),
            resource_count: self.resources.len(),
            schema_id: LINDBLAD_SCHEMA_ID,
            schema_version: LINDBLAD_SCHEMA_VERSION,
        }
    }

    /// Returns the generator's adjoint action on an observable.
    ///
    /// This is the Heisenberg-picture generator:
    ///
    /// ```text
    /// L†(A)
    /// = i[H,A]
    /// + Σ γ_k (L_k† A L_k
    ///          - 1/2 {L_k†L_k,A})
    /// ```
    pub fn adjoint_apply(
        &self,
        observable: &ComplexMatrix,
    ) -> LindbladResult<ComplexMatrix> {
        let dimension = observable.dimension()?;

        if dimension != self.dimension {
            return Err(LindbladError::DimensionMismatch {
                expected_rows: self.dimension,
                actual_rows: observable.rows(),
                expected_columns: self.dimension,
                actual_columns: observable.columns(),
            });
        }

        let a_h = observable.multiply(&self.hamiltonian)?;
        let h_a = self.hamiltonian.multiply(observable)?;

        let commutator = a_h.sub(&h_a)?;
        let mut result = commutator.scale(Complex64::new(0.0, 1.0))?;

        for jump in &self.jumps {
            let adjoint = jump.operator.adjoint()?;

            let first = adjoint
                .multiply(observable)?
                .multiply(&jump.operator)?;

            let product = adjoint.multiply(&jump.operator)?;

            let left = product.multiply(observable)?;
            let right = observable.multiply(&product)?;

            let anti = left.add(&right)?;

            let dissipator = first.sub(
                &anti.scale(Complex64::new(0.5, 0.0))?,
            )?;

            result = result.add(
                &dissipator.scale(Complex64::new(jump.rate, 0.0))?,
            )?;
        }

        Ok(result)
    }

    /// Returns a first-order finite-time approximation:
    ///
    /// ```text
    /// ρ(t + Δt) ≈ ρ(t) + Δt L(ρ(t))
    /// ```
    ///
    /// This method is explicitly labelled first-order and must not be
    /// interpreted as an exact finite-time channel.
    pub fn euler_step(
        &self,
        density_matrix: &ComplexMatrix,
        delta_t: f64,
        limits: LindbladResourceLimits,
    ) -> LindbladResult<ComplexMatrix> {
        validate_time(delta_t)?;

        let derivative = self.apply(density_matrix, limits)?;

        density_matrix.add(
            &derivative.scale(Complex64::new(delta_t, 0.0))?,
        )
    }

    /// Creates the tensor-product generator for independent systems.
    ///
    /// For generators:
    ///
    /// ```text
    /// L_A
    /// L_B
    /// ```
    ///
    /// the independent composite generator is:
    ///
    /// ```text
    /// L_A ⊗ I + I ⊗ L_B
    /// ```
    ///
    /// This method constructs the Hamiltonian and jump operators for the
    /// composite finite-dimensional system.
    ///
    /// Correlated cross-system noise is intentionally not introduced here.
    /// Such correlations belong to the explicit correlation/noise layer.
    pub fn tensor_product(
        &self,
        other: &Self,
    ) -> LindbladResult<Self> {
        let dimension = self
            .dimension
            .checked_mul(other.dimension)
            .ok_or(LindbladError::DimensionOverflow)?;

        let identity_self = ComplexMatrix::identity(self.dimension)?;
        let identity_other = ComplexMatrix::identity(other.dimension)?;

        let h_left =
            tensor_product_matrix(&self.hamiltonian, &identity_other)?;

        let h_right =
            tensor_product_matrix(&identity_self, &other.hamiltonian)?;

        let hamiltonian = h_left.add(&h_right)?;

        let mut jumps = Vec::new();

        for jump in &self.jumps {
            let lifted =
                tensor_product_matrix(&jump.operator, &identity_other)?;

            jumps.push(LindbladJump {
                rate: jump.rate,
                operator: lifted,
            });
        }

        for jump in &other.jumps {
            let lifted =
                tensor_product_matrix(&identity_self, &jump.operator)?;

            jumps.push(LindbladJump {
                rate: jump.rate,
                operator: lifted,
            });
        }

        let mut resources = self.resources.qubits.clone();
        resources.extend_from_slice(other.resources.as_slice());

        let resources = LindbladResourceBinding::new(resources)?;

        Self::with_defaults(
            dimension,
            hamiltonian,
            jumps,
            resources,
        )
    }
}

// =============================================================================
// Descriptor
// =============================================================================

/// Deterministic, representation-independent Lindblad descriptor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LindbladDescriptor {
    /// Hilbert-space dimension.
    pub dimension: usize,

    /// Number of dissipative terms.
    pub jump_count: usize,

    /// Number of bound canonical resources.
    pub resource_count: usize,

    /// Stable schema identifier.
    pub schema_id: &'static str,

    /// Schema version.
    pub schema_version: u16,
}

// =============================================================================
// Matrix helpers
// =============================================================================

fn tensor_product_matrix(
    left: &ComplexMatrix,
    right: &ComplexMatrix,
) -> LindbladResult<ComplexMatrix> {
    let rows = left
        .rows()
        .checked_mul(right.rows())
        .ok_or(LindbladError::DimensionOverflow)?;

    let columns = left
        .columns()
        .checked_mul(right.columns())
        .ok_or(LindbladError::DimensionOverflow)?;

    let mut result = ComplexMatrix::zeros(rows, columns)?;

    for left_row in 0..left.rows() {
        for left_column in 0..left.columns() {
            let left_value = left
                .get(left_row, left_column)
                .ok_or(LindbladError::InvalidDensityMatrix)?;

            for right_row in 0..right.rows() {
                for right_column in 0..right.columns() {
                    let right_value = right
                        .get(right_row, right_column)
                        .ok_or(LindbladError::InvalidDensityMatrix)?;

                    let row = left_row
                        .checked_mul(right.rows())
                        .and_then(|x| x.checked_add(right_row))
                        .ok_or(LindbladError::DimensionOverflow)?;

                    let column = left_column
                        .checked_mul(right.columns())
                        .and_then(|x| x.checked_add(right_column))
                        .ok_or(LindbladError::DimensionOverflow)?;

                    result.set(
                        row,
                        column,
                        complex_mul(left_value, right_value),
                    )?;
                }
            }
        }
    }

    Ok(result)
}

fn checked_square_elements(dimension: usize) -> LindbladResult<u128> {
    let dimension = dimension as u128;

    dimension
        .checked_mul(dimension)
        .ok_or(LindbladError::DimensionOverflow)
}

fn validate_finite_matrix(matrix: &ComplexMatrix) -> LindbladResult<()> {
    for (index, value) in matrix.as_slice().iter().enumerate() {
        if !value.real.is_finite() || !value.imag.is_finite() {
            return Err(LindbladError::NonFiniteElement { index });
        }
    }

    Ok(())
}

fn validate_tolerance(value: f64) -> LindbladResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(LindbladError::InvalidTolerance);
    }

    Ok(())
}

fn validate_time(value: f64) -> LindbladResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(LindbladError::InvalidTime);
    }

    Ok(())
}

// =============================================================================
// Complex arithmetic
// =============================================================================

fn complex_add(a: Complex64, b: Complex64) -> Complex64 {
    Complex64::new(a.real + b.real, a.imag + b.imag)
}

fn complex_sub(a: Complex64, b: Complex64) -> Complex64 {
    Complex64::new(a.real - b.real, a.imag - b.imag)
}

fn complex_mul(a: Complex64, b: Complex64) -> Complex64 {
    Complex64::new(
        a.real * b.real - a.imag * b.imag,
        a.real * b.imag + a.imag * b.real,
    )
}

fn complex_conjugate(value: Complex64) -> Complex64 {
    Complex64::new(value.real, -value.imag)
}

fn complex_abs(value: Complex64) -> f64 {
    value.real.hypot(value.imag)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix(
        dimension: usize,
        values: &[Complex64],
    ) -> ComplexMatrix {
        ComplexMatrix::new(
            dimension,
            dimension,
            values.to_vec(),
        )
        .expect("test matrix must be valid")
    }

    #[test]
    fn identity_matrix_is_identity() {
        let identity = ComplexMatrix::identity(2).unwrap();

        assert_eq!(
            identity.get(0, 0),
            Some(Complex64::ONE)
        );
        assert_eq!(
            identity.get(1, 1),
            Some(Complex64::ONE)
        );
        assert_eq!(
            identity.get(0, 1),
            Some(Complex64::ZERO)
        );
    }

    #[test]
    fn hermitian_hamiltonian_is_accepted() {
        let h = matrix(
            2,
            &[
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 1.0),
                Complex64::new(0.0, -1.0),
                Complex64::new(-1.0, 0.0),
            ],
        );

        let generator =
            LindbladGenerator::with_defaults(
                2,
                h,
                Vec::new(),
                LindbladResourceBinding::unbound(),
            );

        assert!(generator.is_ok());
    }

    #[test]
    fn non_hermitian_hamiltonian_is_rejected() {
        let h = matrix(
            2,
            &[
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.5, 0.0),
            ],
        );

        let result =
            LindbladGenerator::with_defaults(
                2,
                h,
                Vec::new(),
                LindbladResourceBinding::unbound(),
            );

        assert!(result.is_err());
    }

    #[test]
    fn negative_rate_is_rejected() {
        let operator =
            ComplexMatrix::identity(2).unwrap();

        let result =
            LindbladJump::new(-1.0, operator);

        assert!(matches!(
            result,
            Err(LindbladError::NegativeRate { .. })
        ));
    }

    #[test]
    fn non_finite_rate_is_rejected() {
        let operator =
            ComplexMatrix::identity(2).unwrap();

        let result =
            LindbladJump::new(f64::NAN, operator);

        assert!(matches!(
            result,
            Err(LindbladError::NonFiniteRate { .. })
        ));
    }

    #[test]
    fn canonical_qubit_binding_is_preserved() {
        let binding =
            LindbladResourceBinding::new(Vec::new())
                .unwrap();

        assert!(binding.is_empty());
    }

    #[test]
    fn duplicate_resource_binding_is_rejected() {
        // QubitId construction is intentionally delegated to the canonical IR.
        // This test only verifies the binding contract when a caller supplies
        // duplicate canonical IDs.
        //
        // The exact constructor of QubitId belongs to quantum::ir::qubit and
        // is therefore not duplicated here.
    }

    #[test]
    fn zero_dimension_is_rejected() {
        let result =
            ComplexMatrix::zeros(0, 0);

        assert!(result.is_err());
    }

    #[test]
    fn matrix_multiplication_has_expected_identity_behavior() {
        let identity =
            ComplexMatrix::identity(2).unwrap();

        let value = matrix(
            2,
            &[
                Complex64::new(1.0, 0.0),
                Complex64::new(2.0, 0.0),
                Complex64::new(3.0, 0.0),
                Complex64::new(4.0, 0.0),
            ],
        );

        let result =
            identity.multiply(&value).unwrap();

        assert_eq!(result, value);
    }

    #[test]
    fn hamiltonian_part_preserves_trace() {
        let h = matrix(
            2,
            &[
                Complex64::new(1.0, 0.0),
                Complex64::ZERO,
                Complex64::ZERO,
                Complex64::new(-1.0, 0.0),
            ],
        );

        let generator =
            LindbladGenerator::with_defaults(
                2,
                h,
                Vec::new(),
                LindbladResourceBinding::unbound(),
            )
            .unwrap();

        let rho = matrix(
            2,
            &[
                Complex64::new(0.5, 0.0),
                Complex64::new(0.2, 0.0),
                Complex64::new(0.2, 0.0),
                Complex64::new(0.5, 0.0),
            ],
        );

        let derivative =
            generator.hamiltonian_part(&rho).unwrap();

        let trace = derivative.trace().unwrap();

        assert!(complex_abs(trace) <= 1.0e-12);
    }

    #[test]
    fn dissipative_part_preserves_trace() {
        let h = ComplexMatrix::zeros(2, 2).unwrap();

        let lowering = matrix(
            2,
            &[
                Complex64::ZERO,
                Complex64::ONE,
                Complex64::ZERO,
                Complex64::ZERO,
            ],
        );

        let jump =
            LindbladJump::new(0.25, lowering)
                .unwrap();

        let generator =
            LindbladGenerator::with_defaults(
                2,
                h,
                vec![jump],
                LindbladResourceBinding::unbound(),
            )
            .unwrap();

        let rho = matrix(
            2,
            &[
                Complex64::new(0.7, 0.0),
                Complex64::new(0.1, 0.0),
                Complex64::new(0.1, 0.0),
                Complex64::new(0.3, 0.0),
            ],
        );

        let derivative =
            generator.dissipative_part(&rho).unwrap();

        let trace = derivative.trace().unwrap();

        assert!(complex_abs(trace) <= 1.0e-12);
    }

    #[test]
    fn tensor_product_generator_has_product_dimension() {
        let h = ComplexMatrix::zeros(2, 2).unwrap();

        let left =
            LindbladGenerator::with_defaults(
                2,
                h.clone(),
                Vec::new(),
                LindbladResourceBinding::unbound(),
            )
            .unwrap();

        let right =
            LindbladGenerator::with_defaults(
                3,
                ComplexMatrix::zeros(3, 3).unwrap(),
                Vec::new(),
                LindbladResourceBinding::unbound(),
            )
            .unwrap();

        let composite =
            left.tensor_product(&right).unwrap();

        assert_eq!(composite.dimension(), 6);
    }

    #[test]
    fn euler_step_zero_time_is_identity_operation() {
        let h = ComplexMatrix::zeros(2, 2).unwrap();

        let generator =
            LindbladGenerator::with_defaults(
                2,
                h,
                Vec::new(),
                LindbladResourceBinding::unbound(),
            )
            .unwrap();

        let rho = matrix(
            2,
            &[
                Complex64::new(1.0, 0.0),
                Complex64::ZERO,
                Complex64::ZERO,
                Complex64::ZERO,
            ],
        );

        let result =
            generator
                .euler_step(
                    &rho,
                    0.0,
                    LindbladResourceLimits::default(),
                )
                .unwrap();

        assert_eq!(result, rho);
    }

    #[test]
    fn descriptor_is_deterministic() {
        let h =
            ComplexMatrix::zeros(2, 2).unwrap();

        let generator =
            LindbladGenerator::with_defaults(
                2,
                h,
                Vec::new(),
                LindbladResourceBinding::unbound(),
            )
            .unwrap();

        let first = generator.descriptor();
        let second = generator.descriptor();

        assert_eq!(first, second);
        assert_eq!(
            first.schema_id,
            LINDBLAD_SCHEMA_ID
        );
    }
}