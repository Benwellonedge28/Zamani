//! Zamani Quantum Memory — Density-Matrix State Representation
//!
//! Production-grade, provider-neutral density-matrix storage and operations
//! for `crate::quantum::memory`.
//!
//! # Purpose
//!
//! A density matrix represents a general quantum state:
//!
//! ```text
//! ρ ∈ C^(2^n × 2^n)
//!
//! ρ = ρ†
//! Tr(ρ) = 1
//! ρ >= 0
//! ```
//!
//! It is required for:
//!
//! - mixed states;
//! - noisy quantum simulation;
//! - decoherence;
//! - quantum channels;
//! - CPTP maps;
//! - thermal states;
//! - open-system simulation;
//! - exact noise simulation;
//! - mid-circuit measurement;
//! - reset operations;
//! - subsystem reduction / partial trace;
//! - density-matrix benchmarking;
//! - verification against other state representations.
//!
//! # Architectural boundary
//!
//! This file owns the mathematical and storage semantics of a density matrix.
//!
//! It does NOT own:
//!
//! - Quantum IR;
//! - gate syntax;
//! - OpenQASM;
//! - routing;
//! - scheduling;
//! - QPU communication;
//! - vendor-specific APIs;
//! - CUDA;
//! - Metal;
//! - HIP;
//! - MPI;
//! - distributed transport;
//! - compiler parsing;
//! - QEC decoding;
//! - benchmarking protocols;
//! - random-number generation.
//!
//! Those responsibilities remain in their respective Zamani subsystems.
//!
//! # Canonical storage
//!
//! The matrix is stored in row-major order:
//!
//! ```text
//! element(row, column) = data[row * dimension + column]
//! ```
//!
//! For `n` qubits:
//!
//! ```text
//! dimension = 2^n
//! elements  = 4^n
//! ```
//!
//! Therefore a dense density matrix has exponential memory requirements in
//! both dimensions. This implementation performs checked dimension and
//! allocation arithmetic and exposes memory-requirement information before
//! construction.
//!
//! # Qubit ordering
//!
//! The canonical computational-basis index used by this representation is:
//!
//! ```text
//! index = Σ bit(q) * 2^q
//! ```
//!
//! Therefore `q = 0` is the least-significant computational-basis bit.
//!
//! This is the representation's canonical mathematical ordering. External
//! logical-to-physical layouts and permutations belong to `layout.rs` and
//! `permutation.rs`; they must not change the mathematical meaning of this
//! storage.
//!
//! # Hardware / QPU neutrality
//!
//! A density matrix is a representation, not a hardware backend.
//!
//! The API deliberately exposes no:
//!
//! - IBM type;
//! - Google type;
//! - Quantinuum type;
//! - Rigetti type;
//! - IonQ type;
//! - IQM type;
//! - Pasqal type;
//! - neutral-atom type;
//! - photonic vendor type;
//! - CUDA type;
//! - ROCm type;
//! - Metal type.
//!
//! QPU adapters may consume or produce compatible state information through
//! higher-level backend interfaces. A physical QPU generally does not expose
//! its complete density matrix, and this module therefore never assumes that
//! a real QPU can provide one.
//!
//! # Numerical policy
//!
//! `Complex64` is the default precision for high-accuracy work.
//!
//! `Complex32` is supported through the existing `ComplexScalar` abstraction
//! and is suitable for memory-constrained and accelerator-oriented workloads.
//!
//! No fixed numerical tolerance is scattered through the implementation.
//! Validation methods receive an explicit tolerance, while the standard
//! `Complex64` tolerance constants from `memory::complex` are used by the
//! convenience methods.
//!
//! # Safety
//!
//! - No `unsafe`.
//! - No raw pointers.
//! - No global mutable state.
//! - No hidden RNG.
//! - No vendor-specific state.
//! - No stdout/stderr output.
//! - No unchecked allocation-size arithmetic.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Integration contract
//!
//! This module consumes the already-established contracts from:
//!
//! ```text
//! memory::complex
//!     └── ComplexScalar
//!
//! memory::types
//!     └── QubitCount
//!
//! memory::errors
//!     └── MemoryError
//! ```
//!
//! Future modules integrate through the public API defined here:
//!
//! ```text
//! state.rs
//!     └── generic QuantumState abstraction
//!
//! measurement.rs
//!     └── measurement probabilities/results
//!
//! collapse.rs
//!     └── deterministic projection/collapse
//!
//! reset.rs
//!     └── deterministic reset
//!
//! tensor.rs
//!     └── future tensor interoperability
//!
//! migration.rs
//!     └── state-representation migration
//!
//! snapshot.rs / serialization.rs
//!     └── persistence
//!
//! gpu.rs / distributed.rs
//!     └── provider-specific storage adapters
//! ```
//!
//! This file deliberately does not require those later modules to compile.
//! Its public mathematical contract is complete independently.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::ops::{Add, Div, Mul, Sub};

use super::complex::{
    Complex32,
    Complex64,
    ComplexError,
    ComplexScalar,
    DEFAULT_F32_ABS_TOLERANCE,
    DEFAULT_F64_ABS_TOLERANCE,
};
use super::errors::MemoryError;
use super::types::QubitCount;

/// Stable identifier for the density-matrix memory representation.
pub const DENSITY_MATRIX_SCHEMA_ID: &str =
    "zamani.quantum.memory.density_matrix";

/// Semantic version of the density-matrix representation contract.
pub const DENSITY_MATRIX_SCHEMA_VERSION: u16 = 1;

/// Returns the default validation tolerance for `Complex64`.
pub const DEFAULT_DENSITY_MATRIX_F64_TOLERANCE: f64 =
    DEFAULT_F64_ABS_TOLERANCE;

/// Returns the default validation tolerance for `Complex32`.
pub const DEFAULT_DENSITY_MATRIX_F32_TOLERANCE: f32 =
    DEFAULT_F32_ABS_TOLERANCE;

/// Internal real-number abstraction.
///
/// `ComplexScalar` intentionally keeps its numerical contract small. Density
/// matrix validation additionally needs square roots and conversion to a
/// diagnostic `f64`. These operations are kept local so `complex.rs` does not
/// need to be reopened later merely because density-matrix validation needs
/// them.
trait DensityReal:
    Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn is_finite(self) -> bool;
    fn to_f64(self) -> f64;
}

impl DensityReal for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn abs(self) -> Self {
        f32::abs(self)
    }

    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }

    fn is_finite(self) -> bool {
        f32::is_finite(self)
    }

    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl DensityReal for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn abs(self) -> Self {
        f64::abs(self)
    }

    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }

    fn is_finite(self) -> bool {
        f64::is_finite(self)
    }

    fn to_f64(self) -> f64 {
        self
    }
}

/// Result type used by this module.
pub type DensityMatrixResult<T> = Result<T, MemoryError>;

/// Immutable information describing a density-matrix allocation.
///
/// This is intentionally available before allocation so higher-level memory
/// managers can apply `MemoryLimits`, `MemoryBudget`, reservations, GPU
/// capacity checks, or distributed-memory policies before constructing the
/// matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DensityMatrixMemoryRequirement {
    /// Number of qubits.
    qubits: QubitCount,

    /// Matrix dimension, `2^n`.
    dimension: usize,

    /// Number of complex elements, `4^n`.
    elements: usize,

    /// Number of bytes required by one matrix allocation.
    bytes: u64,

    /// Bytes per complex scalar.
    scalar_bytes: usize,
}

impl DensityMatrixMemoryRequirement {
    /// Returns the number of qubits.
    pub const fn qubits(self) -> QubitCount {
        self.qubits
    }

    /// Returns the matrix dimension.
    pub const fn dimension(self) -> usize {
        self.dimension
    }

    /// Returns the number of matrix elements.
    pub const fn elements(self) -> usize {
        self.elements
    }

    /// Returns the required bytes.
    pub const fn bytes(self) -> u64 {
        self.bytes
    }

    /// Returns the bytes occupied by one scalar.
    pub const fn scalar_bytes(self) -> usize {
        self.scalar_bytes
    }
}

/// A dense quantum density matrix.
///
/// The storage is row-major and contains exactly `dimension * dimension`
/// complex scalars.
///
/// The type is generic over Zamani's canonical `ComplexScalar`, allowing the
/// same mathematical implementation to support `Complex32` and `Complex64`.
#[derive(Clone, Debug, PartialEq)]
pub struct DensityMatrix<T: ComplexScalar> {
    qubits: QubitCount,
    dimension: usize,
    data: Vec<T>,
}

impl<T> DensityMatrix<T>
where
    T: ComplexScalar,
    T::Real: DensityReal,
{
    // =========================================================================
    // Construction and planning
    // =========================================================================

    /// Computes the checked memory requirement without allocating.
    ///
    /// This function must be called by higher-level memory-management code
    /// before a production allocation is committed to an allocator, budget,
    /// device, or distributed memory provider.
    pub fn memory_requirement(
        qubits: QubitCount,
    ) -> DensityMatrixResult<DensityMatrixMemoryRequirement> {
        let n = qubits.get();

        if n >= usize::BITS as usize {
            return Err(MemoryError::ArithmeticOverflow {
                operation: "2^qubits density-matrix dimension".to_owned(),
            });
        }

        let dimension = 1usize
            .checked_shl(n as u32)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "2^qubits density-matrix dimension".to_owned(),
            })?;

        let elements = dimension
            .checked_mul(dimension)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "density-matrix dimension^2".to_owned(),
            })?;

        let scalar_bytes = T::BYTE_SIZE;

        let elements_u64 =
            u64::try_from(elements).map_err(|_| MemoryError::ArithmeticOverflow {
                operation: "density-matrix element count to u64".to_owned(),
            })?;

        let scalar_bytes_u64 =
            u64::try_from(scalar_bytes).map_err(|_| MemoryError::ArithmeticOverflow {
                operation: "complex scalar byte size to u64".to_owned(),
            })?;

        let bytes = elements_u64
            .checked_mul(scalar_bytes_u64)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "density-matrix elements * scalar bytes".to_owned(),
            })?;

        Ok(DensityMatrixMemoryRequirement {
            qubits,
            dimension,
            elements,
            bytes,
            scalar_bytes,
        })
    }

    /// Creates the canonical `|0...0><0...0|` density matrix.
    ///
    /// The allocation size is checked for platform arithmetic overflow before
    /// the vector is allocated.
    ///
    /// Higher-level production memory managers must additionally apply the
    /// repository's `MemoryLimits` and `MemoryBudget` policies before invoking
    /// this constructor when operating under a configured resource policy.
    pub fn zero_state(qubits: QubitCount) -> DensityMatrixResult<Self> {
        let requirement = Self::memory_requirement(qubits)?;

        let mut data = vec![T::zero(); requirement.elements];

        if data.is_empty() {
            return Err(MemoryError::InvalidState {
                reason: "density matrix cannot contain zero elements".to_owned(),
            });
        }

        data[0] = T::one();

        Ok(Self {
            qubits,
            dimension: requirement.dimension,
            data,
        })
    }

    /// Creates a maximally mixed state:
    ///
    /// `ρ = I / d`.
    pub fn maximally_mixed(qubits: QubitCount) -> DensityMatrixResult<Self> {
        let requirement = Self::memory_requirement(qubits)?;

        let dimension_real = T::from_real(
            T::Real::from_f64(requirement.dimension as f64)?,
        )
        .map_err(map_complex_error)?;

        let inverse_dimension =
            T::one().checked_div(dimension_real).map_err(map_complex_error)?;

        let mut data = vec![T::zero(); requirement.elements];

        for i in 0..requirement.dimension {
            let index = matrix_index(i, i, requirement.dimension)?;
            data[index] = inverse_dimension;
        }

        Ok(Self {
            qubits,
            dimension: requirement.dimension,
            data,
        })
    }

    /// Constructs a density matrix from explicit row-major elements.
    ///
    /// The input is checked for:
    ///
    /// - correct dimension;
    /// - finite values.
    ///
    /// Mathematical physical-state validation is deliberately separate and
    /// available through `validate_physical`.
    pub fn from_elements(
        qubits: QubitCount,
        data: Vec<T>,
    ) -> DensityMatrixResult<Self> {
        let requirement = Self::memory_requirement(qubits)?;

        if data.len() != requirement.elements {
            return Err(MemoryError::StateDimensionMismatch {
                expected: requirement.elements as u64,
                actual: data.len() as u64,
            });
        }

        for (index, value) in data.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(MemoryError::NonFiniteValue {
                    index: index as u64,
                });
            }
        }

        Ok(Self {
            qubits,
            dimension: requirement.dimension,
            data,
        })
    }

    /// Constructs a density matrix from a pure state:
    ///
    /// `ρ = |ψ><ψ|`.
    ///
    /// The supplied vector must already be normalized within the supplied
    /// tolerance. This method never silently normalizes user input.
    pub fn from_pure_state(
        amplitudes: &[T],
        tolerance: T::Real,
    ) -> DensityMatrixResult<Self> {
        let dimension = amplitudes.len();

        let qubits = qubits_from_dimension(dimension)?;

        let norm = vector_norm_squared(amplitudes);

        let one = T::Real::one();

        if (norm - one).abs() > tolerance {
            return Err(MemoryError::NotNormalized {
                norm: norm.to_f64(),
                tolerance: tolerance.to_f64(),
            });
        }

        for (index, amplitude) in amplitudes.iter().copied().enumerate() {
            if !amplitude.is_finite() {
                return Err(MemoryError::NonFiniteValue {
                    index: index as u64,
                });
            }
        }

        let elements = dimension
            .checked_mul(dimension)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "pure-state dimension^2".to_owned(),
            })?;

        let mut data = vec![T::zero(); elements];

        for row in 0..dimension {
            for column in 0..dimension {
                let index = matrix_index(row, column, dimension)?;

                data[index] =
                    amplitudes[row] * amplitudes[column].conjugate();
            }
        }

        Ok(Self {
            qubits,
            dimension,
            data,
        })
    }

    /// Constructs a pure density matrix and explicitly normalizes the supplied
    /// vector.
    ///
    /// Normalization is explicit in the method name; callers cannot
    /// accidentally obtain implicit normalization through `from_pure_state`.
    pub fn from_pure_state_normalized(
        amplitudes: &[T],
    ) -> DensityMatrixResult<Self> {
        if amplitudes.is_empty() {
            return Err(MemoryError::InvalidState {
                reason: "pure-state amplitude vector is empty".to_owned(),
            });
        }

        let norm_squared = vector_norm_squared(amplitudes);

        if !norm_squared.is_finite() {
            return Err(MemoryError::NonFiniteValue { index: 0 });
        }

        if norm_squared <= T::Real::zero() {
            return Err(MemoryError::InvalidState {
                reason: "cannot normalize a zero pure state".to_owned(),
            });
        }

        let norm = norm_squared.sqrt();

        let mut normalized = Vec::with_capacity(amplitudes.len());

        for amplitude in amplitudes.iter().copied() {
            let magnitude = T::from_real(norm).map_err(map_complex_error)?;
            normalized.push(
                amplitude
                    .checked_div(magnitude)
                    .map_err(map_complex_error)?,
            );
        }

        Self::from_pure_state(
            &normalized,
            T::Real::from_f64(
                default_tolerance::<T::Real>(),
            )?,
        )
    }

    /// Returns the number of qubits.
    pub const fn qubit_count(&self) -> QubitCount {
        self.qubits
    }

    /// Returns the matrix dimension.
    pub const fn dimension(&self) -> usize {
        self.dimension
    }

    /// Returns the number of stored elements.
    pub const fn element_count(&self) -> usize {
        self.data.len()
    }

    /// Returns the required bytes for this matrix.
    pub fn memory_bytes(&self) -> DensityMatrixResult<u64> {
        Ok(Self::memory_requirement(self.qubits)?.bytes())
    }

    /// Returns a read-only view of the row-major storage.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns the mutable storage.
    ///
    /// This method intentionally exposes mutable storage only through an
    /// explicit method. Mutating values can invalidate physical-state
    /// invariants; callers should call `validate_physical` before treating the
    /// resulting matrix as a valid quantum state.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    // =========================================================================
    // Element access
    // =========================================================================

    /// Returns one matrix element.
    pub fn get(
        &self,
        row: usize,
        column: usize,
    ) -> DensityMatrixResult<T> {
        let index = matrix_index(row, column, self.dimension)?;
        self.data
            .get(index)
            .copied()
            .ok_or_else(|| MemoryError::OutOfBounds {
                index: index as u64,
                length: self.data.len() as u64,
                resource: "density-matrix storage".to_owned(),
            })
    }

    /// Sets one matrix element.
    ///
    /// The operation validates that the supplied scalar is finite.
    ///
    /// Because arbitrary mutation may break Hermiticity, trace, or positivity,
    /// the matrix should be validated before being used as a physical state.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: T,
    ) -> DensityMatrixResult<()> {
        if !value.is_finite() {
            return Err(MemoryError::NonFiniteValue {
                index: matrix_index(row, column, self.dimension)? as u64,
            });
        }

        let index = matrix_index(row, column, self.dimension)?;

        let slot =
            self.data
                .get_mut(index)
                .ok_or_else(|| MemoryError::OutOfBounds {
                    index: index as u64,
                    length: self.data.len() as u64,
                    resource: "density-matrix storage".to_owned(),
                })?;

        *slot = value;

        Ok(())
    }

    /// Returns the diagonal element at `index`.
    pub fn diagonal(
        &self,
        index: usize,
    ) -> DensityMatrixResult<T> {
        self.get(index, index)
    }

    /// Returns the Born-rule probability of computational-basis state `|index>`.
    pub fn basis_probability(
        &self,
        index: usize,
    ) -> DensityMatrixResult<T::Real> {
        let value = self.diagonal(index)?;

        let imaginary = value.imaginary();

        if imaginary.abs() > default_tolerance::<T::Real>() {
            return Err(MemoryError::InvalidProbability {
                probability: value.real().to_f64(),
                reason:
                    "density-matrix diagonal element has a non-negligible imaginary component"
                        .to_owned(),
            });
        }

        let probability = value.real();

        if probability < T::Real::zero() {
            return Err(MemoryError::InvalidProbability {
                probability: probability.to_f64(),
                reason: "basis probability is negative".to_owned(),
            });
        }

        Ok(probability)
    }

    /// Returns the probability that a qubit is measured as `0` or `1` in the
    /// computational basis.
    pub fn qubit_probability(
        &self,
        qubit: usize,
        one: bool,
    ) -> DensityMatrixResult<T::Real> {
        validate_qubit(qubit, self.qubits)?;

        let mut probability = T::Real::zero();

        for basis in 0..self.dimension {
            let bit_is_one = ((basis >> qubit) & 1) != 0;

            if bit_is_one == one {
                probability =
                    probability + self.basis_probability(basis)?;
            }
        }

        Ok(probability)
    }

    // =========================================================================
    // Basic mathematical properties
    // =========================================================================

    /// Returns the complex trace.
    pub fn trace(&self) -> T {
        let mut result = T::zero();

        for index in 0..self.dimension {
            result =
                result + self.data[matrix_index(index, index, self.dimension)?];
        }

        Ok(result)
    }

    /// Returns the real trace.
    ///
    /// This returns an error if the imaginary component of the trace is larger
    /// than the supplied tolerance.
    pub fn real_trace(
        &self,
        tolerance: T::Real,
    ) -> DensityMatrixResult<T::Real> {
        let trace = self.trace()?;

        if trace.imaginary().abs() > tolerance {
            return Err(MemoryError::InvalidTrace {
                trace: trace.real().to_f64(),
                tolerance: tolerance.to_f64(),
            });
        }

        Ok(trace.real())
    }

    /// Returns the maximum absolute entrywise Hermiticity deviation:
    ///
    /// `max |ρ_ij - conj(ρ_ji)|`.
    pub fn max_hermiticity_deviation(&self) -> DensityMatrixResult<T::Real> {
        let mut maximum = T::Real::zero();

        for row in 0..self.dimension {
            for column in 0..self.dimension {
                let lhs =
                    self.data[matrix_index(row, column, self.dimension)?];

                let rhs =
                    self.data[matrix_index(column, row, self.dimension)?]
                        .conjugate();

                let deviation = (lhs - rhs).magnitude();

                if deviation > maximum {
                    maximum = deviation;
                }
            }
        }

        Ok(maximum)
    }

    /// Checks Hermiticity without changing the state.
    pub fn validate_hermitian(
        &self,
        tolerance: T::Real,
    ) -> DensityMatrixResult<()> {
        let deviation = self.max_hermiticity_deviation()?;

        if deviation > tolerance {
            return Err(MemoryError::NotHermitian {
                maximum_deviation: deviation.to_f64(),
                tolerance: tolerance.to_f64(),
            });
        }

        Ok(())
    }

    /// Checks that the trace is one within `tolerance`.
    pub fn validate_trace(
        &self,
        tolerance: T::Real,
    ) -> DensityMatrixResult<()> {
        let trace = self.trace()?;

        let one = T::Real::one();

        if trace.imaginary().abs() > tolerance
            || (trace.real() - one).abs() > tolerance
        {
            return Err(MemoryError::InvalidTrace {
                trace: trace.real().to_f64(),
                tolerance: tolerance.to_f64(),
            });
        }

        Ok(())
    }

    /// Performs basic physical-state validation:
    ///
    /// - all values finite;
    /// - Hermitian;
    /// - trace one.
    ///
    /// This does not perform the potentially expensive positive-semidefinite
    /// check.
    pub fn validate_basic(
        &self,
        tolerance: T::Real,
    ) -> DensityMatrixResult<()> {
        self.validate_finite()?;
        self.validate_hermitian(tolerance)?;
        self.validate_trace(tolerance)?;

        Ok(())
    }

    /// Performs complete physical-state validation:
    ///
    /// - all values finite;
    /// - Hermitian;
    /// - trace one;
    /// - positive semidefinite.
    ///
    /// Positive-semidefinite validation uses a Cholesky-style factorization
    /// with explicit tolerance handling for semidefinite zero pivots.
    pub fn validate_physical(
        &self,
        tolerance: T::Real,
    ) -> DensityMatrixResult<()> {
        self.validate_basic(tolerance)?;
        self.validate_positive_semidefinite(tolerance)?;

        Ok(())
    }

    /// Validates that every matrix element is finite.
    pub fn validate_finite(&self) -> DensityMatrixResult<()> {
        for (index, value) in self.data.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(MemoryError::NonFiniteValue {
                    index: index as u64,
                });
            }
        }

        Ok(())
    }

    /// Checks positive semidefiniteness.
    ///
    /// The implementation is allocation-safe and uses checked indexing.
    ///
    /// This operation is intentionally separate from `validate_basic` because
    /// PSD validation is O(d^3), where `d = 2^n`.
    pub fn validate_positive_semidefinite(
        &self,
        tolerance: T::Real,
    ) -> DensityMatrixResult<()> {
        self.validate_hermitian(tolerance)?;

        let d = self.dimension;

        let mut lower = vec![T::zero(); d.checked_mul(d).ok_or_else(|| {
            MemoryError::ArithmeticOverflow {
                operation: "PSD factorization storage dimension^2".to_owned(),
            }
        })?];

        for i in 0..d {
            let mut diagonal = self.get(i, i)?.real();

            for k in 0..i {
                let l = lower[matrix_index(i, k, d)?];
                diagonal =
                    diagonal - (l * l.conjugate()).real();
            }

            if diagonal < -tolerance {
                return Err(MemoryError::InvalidState {
                    reason: format!(
                        "density matrix is not positive semidefinite: \
                         negative Cholesky pivot at index {i}"
                    ),
                });
            }

            if diagonal <= tolerance {
                lower[matrix_index(i, i, d)?] = T::zero();

                for row in (i + 1)..d {
                    let mut residual = self.get(row, i)?;

                    for k in 0..i {
                        let a = lower[matrix_index(row, k, d)?];
                        let b = lower[matrix_index(i, k, d)?];

                        residual = residual - a * b.conjugate();
                    }

                    if residual.magnitude() > tolerance {
                        return Err(MemoryError::InvalidState {
                            reason: format!(
                                "density matrix is not positive semidefinite: \
                                 non-zero residual at ({row},{i}) after a \
                                 zero pivot"
                            ),
                        });
                    }

                    lower[matrix_index(row, i, d)?] = T::zero();
                }

                continue;
            }

            let root = diagonal.sqrt();

            let root_complex =
                T::from_real(root).map_err(map_complex_error)?;

            lower[matrix_index(i, i, d)?] = root_complex;

            for row in (i + 1)..d {
                let mut residual = self.get(row, i)?;

                for k in 0..i {
                    let a = lower[matrix_index(row, k, d)?];
                    let b = lower[matrix_index(i, k, d)?];

                    residual = residual - a * b.conjugate();
                }

                lower[matrix_index(row, i, d)?] = residual
                    .checked_div(root_complex)
                    .map_err(map_complex_error)?;
            }
        }

        Ok(())
    }

    /// Returns the purity:
    ///
    /// `Tr(ρ²)`.
    pub fn purity(&self) -> DensityMatrixResult<T::Real> {
        let mut result = T::Real::zero();

        for row in 0..self.dimension {
            for column in 0..self.dimension {
                let rho_ij =
                    self.data[matrix_index(row, column, self.dimension)?];

                let rho_ji =
                    self.data[matrix_index(column, row, self.dimension)?];

                result =
                    result + (rho_ij * rho_ji).real();
            }
        }

        Ok(result)
    }

    /// Returns whether the state is approximately pure.
    pub fn is_pure(
        &self,
        tolerance: T::Real,
    ) -> DensityMatrixResult<bool> {
        let purity = self.purity()?;

        Ok((purity - T::Real::one()).abs() <= tolerance)
    }

    // =========================================================================
    // State transformations
    // =========================================================================

    /// Applies a full-system unitary transformation:
    ///
    /// `ρ' = UρU†`.
    ///
    /// `unitary` must contain exactly `d*d` elements where `d = 2^n`.
    pub fn apply_unitary(
        &mut self,
        unitary: &[T],
        tolerance: T::Real,
    ) -> DensityMatrixResult<()> {
        validate_operator_dimension(unitary, self.dimension)?;
        validate_unitary(unitary, self.dimension, tolerance)?;

        let transformed =
            conjugate_by_operator(&self.data, unitary, self.dimension)?;

        self.data = transformed;

        Ok(())
    }

    /// Returns a transformed copy under a full-system unitary.
    pub fn transformed_by_unitary(
        &self,
        unitary: &[T],
        tolerance: T::Real,
    ) -> DensityMatrixResult<Self> {
        let mut result = self.clone();

        result.apply_unitary(unitary, tolerance)?;

        Ok(result)
    }

    /// Applies an arbitrary unitary to a selected set of qubits.
    ///
    /// The unitary dimension must be `2^k × 2^k`, where `k` is the number of
    /// selected qubits.
    ///
    /// Qubit indices must be unique.
    ///
    /// This operation is representation-level and therefore works regardless
    /// of whether the physical implementation later maps the operation to a
    /// CPU, GPU, distributed simulator, or another execution substrate.
    pub fn apply_unitary_on_qubits(
        &mut self,
        qubits: &[usize],
        unitary: &[T],
        tolerance: T::Real,
    ) -> DensityMatrixResult<()> {
        validate_target_qubits(qubits, self.qubits)?;

        let local_dimension = checked_dimension(qubits.len())?;

        validate_operator_dimension(unitary, local_dimension)?;
        validate_unitary(unitary, local_dimension, tolerance)?;

        let transformed = apply_local_operator(
            &self.data,
            self.dimension,
            qubits,
            unitary,
        )?;

        self.data = transformed;

        Ok(())
    }

    /// Applies a quantum channel represented by Kraus operators to selected
    /// qubits:
    ///
    /// `ρ' = Σ_k K_k ρ K_k†`.
    ///
    /// Every Kraus operator must have dimension `2^k × 2^k`.
    ///
    /// The channel is validated as trace-preserving before application.
    pub fn apply_kraus_operators_on_qubits(
        &mut self,
        qubits: &[usize],
        kraus_operators: &[Vec<T>],
        tolerance: T::Real,
    ) -> DensityMatrixResult<()> {
        validate_target_qubits(qubits, self.qubits)?;

        if kraus_operators.is_empty() {
            return Err(MemoryError::InvalidArgument {
                argument: "kraus_operators".to_owned(),
                context: None,
            });
        }

        let local_dimension = checked_dimension(qubits.len())?;

        for operator in kraus_operators {
            validate_operator_dimension(operator, local_dimension)?;
        }

        validate_trace_preserving_kraus(
            kraus_operators,
            local_dimension,
            tolerance,
        )?;

        let mut result = vec![T::zero(); self.data.len()];

        for operator in kraus_operators {
            let transformed = apply_local_operator(
                &self.data,
                self.dimension,
                qubits,
                operator,
            )?;

            for index in 0..result.len() {
                result[index] =
                    result[index] + transformed[index];
            }
        }

        self.data = result;

        Ok(())
    }

    /// Applies a full-system Kraus channel.
    pub fn apply_kraus_operators(
        &mut self,
        kraus_operators: &[Vec<T>],
        tolerance: T::Real,
    ) -> DensityMatrixResult<()> {
        let targets: Vec<usize> = (0..self.qubits.get()).collect();

        self.apply_kraus_operators_on_qubits(
            &targets,
            kraus_operators,
            tolerance,
        )
    }

    // =========================================================================
    // Measurement and collapse
    // =========================================================================

    /// Returns the probability of a computational-basis measurement outcome
    /// on the selected qubits.
    ///
    /// `outcome` is encoded in the order supplied by `qubits`:
    ///
    /// ```text
    /// qubits = [q2, q5]
    /// outcome bit 0 -> q2
    /// outcome bit 1 -> q5
    /// ```
    pub fn measurement_probability(
        &self,
        qubits: &[usize],
        outcome: usize,
    ) -> DensityMatrixResult<T::Real> {
        validate_target_qubits(qubits, self.qubits)?;

        let local_dimension = checked_dimension(qubits.len())?;

        if outcome >= local_dimension {
            return Err(MemoryError::OutOfBounds {
                index: outcome as u64,
                length: local_dimension as u64,
                resource: "measurement outcome".to_owned(),
            });
        }

        let mut probability = T::Real::zero();

        for basis in 0..self.dimension {
            if extract_bits(basis, qubits) == outcome {
                probability =
                    probability + self.basis_probability(basis)?;
            }
        }

        if probability < -default_tolerance::<T::Real>()
            || probability > T::Real::one()
                + default_tolerance::<T::Real>()
        {
            return Err(MemoryError::MeasurementProbabilityError {
                reason: format!(
                    "measurement probability {} is outside [0,1]",
                    probability.to_f64()
                ),
            });
        }

        Ok(clamp_probability(probability))
    }

    /// Projects the density matrix onto a computational-basis measurement
    /// outcome and normalizes it.
    ///
    /// This is deterministic: it does not sample a result.
    ///
    /// Sampling belongs to `measurement.rs` and must use an explicitly
    /// injected RNG.
    pub fn project_measurement(
        &mut self,
        qubits: &[usize],
        outcome: usize,
        tolerance: T::Real,
    ) -> DensityMatrixResult<T::Real> {
        validate_target_qubits(qubits, self.qubits)?;

        let local_dimension = checked_dimension(qubits.len())?;

        if outcome >= local_dimension {
            return Err(MemoryError::OutOfBounds {
                index: outcome as u64,
                length: local_dimension as u64,
                resource: "measurement outcome".to_owned(),
            });
        }

        let probability =
            self.measurement_probability(qubits, outcome)?;

        if probability <= tolerance {
            return Err(MemoryError::CollapseError {
                reason:
                    "cannot project onto a zero-probability measurement outcome"
                        .to_owned(),
            });
        }

        let denominator =
            T::from_real(probability).map_err(map_complex_error)?;

        for row in 0..self.dimension {
            for column in 0..self.dimension {
                let row_outcome = extract_bits(row, qubits);
                let column_outcome = extract_bits(column, qubits);

                let index =
                    matrix_index(row, column, self.dimension)?;

                if row_outcome != outcome
                    || column_outcome != outcome
                {
                    self.data[index] = T::zero();
                } else {
                    self.data[index] = self.data[index]
                        .checked_div(denominator)
                        .map_err(map_complex_error)?;
                }
            }
        }

        Ok(probability)
    }

    /// Removes coherence between computational-basis sectors of the selected
    /// qubits without selecting a measurement outcome.
    ///
    /// This implements the non-selective computational-basis dephasing map.
    pub fn dephase_qubits(
        &mut self,
        qubits: &[usize],
    ) -> DensityMatrixResult<()> {
        validate_target_qubits(qubits, self.qubits)?;

        for row in 0..self.dimension {
            for column in 0..self.dimension {
                if extract_bits(row, qubits)
                    != extract_bits(column, qubits)
                {
                    let index =
                        matrix_index(row, column, self.dimension)?;

                    self.data[index] = T::zero();
                }
            }
        }

        Ok(())
    }

    /// Resets selected qubits to `|0>`.
    ///
    /// The operation is the deterministic CPTP reset channel:
    ///
    /// `ρ -> |0><0| ⊗ Tr_target(ρ)`
    ///
    /// with the selected qubits restored to their canonical positions.
    pub fn reset_qubits(
        &mut self,
        qubits: &[usize],
    ) -> DensityMatrixResult<()> {
        validate_target_qubits(qubits, self.qubits)?;

        let target_mask = mask_for_qubits(qubits)?;

        let mut result = vec![T::zero(); self.data.len()];

        for row in 0..self.dimension {
            if row & target_mask != 0 {
                continue;
            }

            for column in 0..self.dimension {
                if column & target_mask != 0 {
                    continue;
                }

                let mut value = T::zero();

                let target_dimension =
                    checked_dimension(qubits.len())?;

                for target_state in 0..target_dimension {
                    let old_row =
                        insert_bits(row, qubits, target_state)?;

                    let old_column =
                        insert_bits(column, qubits, target_state)?;

                    let old_index =
                        matrix_index(
                            old_row,
                            old_column,
                            self.dimension,
                        )?;

                    value = value + self.data[old_index];
                }

                let new_index =
                    matrix_index(row, column, self.dimension)?;

                result[new_index] = value;
            }
        }

        self.data = result;

        Ok(())
    }

    // =========================================================================
    // Subsystem operations
    // =========================================================================

    /// Computes the partial trace over the selected qubits.
    ///
    /// The selected qubits are removed from the returned density matrix.
    ///
    /// Example:
    ///
    /// ```text
    /// ρ_AB --partial_trace([B])--> ρ_A
    /// ```
    pub fn partial_trace(
        &self,
        traced_qubits: &[usize],
    ) -> DensityMatrixResult<Self> {
        validate_target_qubits(traced_qubits, self.qubits)?;

        let traced = unique_sorted(traced_qubits)?;

        let retained =
            complement_qubits(self.qubits.get(), &traced)?;

        let retained_dimension =
            checked_dimension(retained.len())?;

        let traced_dimension =
            checked_dimension(traced.len())?;

        let retained_elements =
            retained_dimension
                .checked_mul(retained_dimension)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation:
                        "partial-trace retained dimension^2".to_owned(),
                })?;

        let mut result =
            vec![T::zero(); retained_elements];

        for retained_row in 0..retained_dimension {
            for retained_column in 0..retained_dimension {
                let mut value = T::zero();

                for traced_state in 0..traced_dimension {
                    let row = embed_two_bitsets(
                        retained_row,
                        &retained,
                        traced_state,
                        &traced,
                    )?;

                    let column = embed_two_bitsets(
                        retained_column,
                        &retained,
                        traced_state,
                        &traced,
                    )?;

                    let source =
                        matrix_index(row, column, self.dimension)?;

                    value = value + self.data[source];
                }

                let destination =
                    matrix_index(
                        retained_row,
                        retained_column,
                        retained_dimension,
                    )?;

                result[destination] = value;
            }
        }

        Ok(Self {
            qubits: QubitCount::new(retained.len()),
            dimension: retained_dimension,
            data: result,
        })
    }

    /// Returns the reduced density matrix for all qubits except `qubit`.
    pub fn reduced_without_qubit(
        &self,
        qubit: usize,
    ) -> DensityMatrixResult<Self> {
        self.partial_trace(&[qubit])
    }

    /// Returns the reduced density matrix of the selected qubits.
    ///
    /// This is equivalent to tracing out every other qubit.
    pub fn reduced_to_qubits(
        &self,
        retained_qubits: &[usize],
    ) -> DensityMatrixResult<Self> {
        validate_target_qubits(retained_qubits, self.qubits)?;

        let retained = unique_sorted(retained_qubits)?;

        let traced =
            complement_qubits(self.qubits.get(), &retained)?;

        self.partial_trace(&traced)
    }

    // =========================================================================
    // Tensor products
    // =========================================================================

    /// Computes the tensor/Kronecker product:
    ///
    /// `self ⊗ other`.
    ///
    /// The qubits of `self` occupy the higher-order portion of the resulting
    /// basis index and `other` occupies the lower-order portion.
    pub fn tensor_product(
        &self,
        other: &Self,
    ) -> DensityMatrixResult<Self> {
        let total_qubits = self
            .qubits
            .checked_add(other.qubits)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "tensor-product qubit count".to_owned(),
            })?;

        let requirement =
            Self::memory_requirement(total_qubits)?;

        let mut result =
            vec![T::zero(); requirement.elements];

        for row_a in 0..self.dimension {
            for column_a in 0..self.dimension {
                let a =
                    self.data[matrix_index(
                        row_a,
                        column_a,
                        self.dimension,
                    )?];

                for row_b in 0..other.dimension {
                    for column_b in 0..other.dimension {
                        let b =
                            other.data[matrix_index(
                                row_b,
                                column_b,
                                other.dimension,
                            )?];

                        let row =
                            row_a
                                .checked_mul(other.dimension)
                                .and_then(|v| v.checked_add(row_b))
                                .ok_or_else(|| {
                                    MemoryError::ArithmeticOverflow {
                                        operation:
                                            "tensor-product row index"
                                                .to_owned(),
                                    }
                                })?;

                        let column =
                            column_a
                                .checked_mul(other.dimension)
                                .and_then(|v| v.checked_add(column_b))
                                .ok_or_else(|| {
                                    MemoryError::ArithmeticOverflow {
                                        operation:
                                            "tensor-product column index"
                                                .to_owned(),
                                    }
                                })?;

                        let destination =
                            matrix_index(
                                row,
                                column,
                                requirement.dimension,
                            )?;

                        result[destination] = a * b;
                    }
                }
            }
        }

        Ok(Self {
            qubits: total_qubits,
            dimension: requirement.dimension,
            data: result,
        })
    }

    // =========================================================================
    // Observables
    // =========================================================================

    /// Computes:
    ///
    /// `Tr(ρ O)`
    ///
    /// for a full-system observable `O`.
    pub fn expectation_value(
        &self,
        observable: &[T],
    ) -> DensityMatrixResult<T> {
        validate_operator_dimension(observable, self.dimension)?;

        let mut result = T::zero();

        for row in 0..self.dimension {
            for column in 0..self.dimension {
                let rho =
                    self.data[matrix_index(
                        row,
                        column,
                        self.dimension,
                    )?];

                let operator =
                    observable[matrix_index(
                        column,
                        row,
                        self.dimension,
                    )?];

                result = result + rho * operator;
            }
        }

        Ok(result)
    }

    /// Computes the Hilbert-Schmidt overlap:
    ///
    /// `Tr(A† B)`.
    ///
    /// This is useful for diagnostics and state-comparison infrastructure but
    /// is intentionally not called "fidelity", because quantum fidelity has a
    /// stricter mathematical definition.
    pub fn hilbert_schmidt_overlap(
        &self,
        other: &Self,
    ) -> DensityMatrixResult<T> {
        self.ensure_same_dimension(other)?;

        let mut result = T::zero();

        for row in 0..self.dimension {
            for column in 0..self.dimension {
                let lhs =
                    self.data[matrix_index(
                        row,
                        column,
                        self.dimension,
                    )?]
                    .conjugate();

                let rhs =
                    other.data[matrix_index(
                        row,
                        column,
                        self.dimension,
                    )?];

                result = result + lhs * rhs;
            }
        }

        Ok(result)
    }

    // =========================================================================
    // State comparison
    // =========================================================================

    /// Returns the maximum entrywise absolute difference.
    pub fn max_difference(
        &self,
        other: &Self,
    ) -> DensityMatrixResult<T::Real> {
        self.ensure_same_dimension(other)?;

        let mut maximum = T::Real::zero();

        for index in 0..self.data.len() {
            let difference =
                (self.data[index] - other.data[index]).magnitude();

            if difference > maximum {
                maximum = difference;
            }
        }

        Ok(maximum)
    }

    /// Returns whether two density matrices are approximately equal.
    pub fn approx_eq(
        &self,
        other: &Self,
        tolerance: T::Real,
    ) -> DensityMatrixResult<bool> {
        Ok(self.max_difference(other)? <= tolerance)
    }

    /// Returns a deep copy.
    pub fn snapshot_copy(&self) -> Self {
        self.clone()
    }

    // =========================================================================
    // Internal validation helpers exposed as stable API
    // =========================================================================

    /// Ensures another density matrix has the same number of qubits.
    pub fn ensure_same_dimension(
        &self,
        other: &Self,
    ) -> DensityMatrixResult<()> {
        if self.dimension != other.dimension {
            return Err(MemoryError::StateDimensionMismatch {
                expected: self.dimension as u64,
                actual: other.dimension as u64,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Specialized convenience constructors
// =============================================================================

impl DensityMatrix<Complex64> {
    /// Creates a `Complex64` zero state.
    pub fn zero_state_f64(
        qubits: QubitCount,
    ) -> DensityMatrixResult<Self> {
        Self::zero_state(qubits)
    }

    /// Validates using Zamani's canonical double-precision tolerance.
    pub fn validate_physical_f64(&self) -> DensityMatrixResult<()> {
        self.validate_physical(DEFAULT_DENSITY_MATRIX_F64_TOLERANCE)
    }

    /// Validates basic invariants using the canonical double-precision
    /// tolerance.
    pub fn validate_basic_f64(&self) -> DensityMatrixResult<()> {
        self.validate_basic(DEFAULT_DENSITY_MATRIX_F64_TOLERANCE)
    }
}

impl DensityMatrix<Complex32> {
    /// Creates a `Complex32` zero state.
    pub fn zero_state_f32(
        qubits: QubitCount,
    ) -> DensityMatrixResult<Self> {
        Self::zero_state(qubits)
    }

    /// Validates using Zamani's canonical single-precision tolerance.
    pub fn validate_physical_f32(&self) -> DensityMatrixResult<()> {
        self.validate_physical(DEFAULT_DENSITY_MATRIX_F32_TOLERANCE)
    }

    /// Validates basic invariants using the canonical single-precision
    /// tolerance.
    pub fn validate_basic_f32(&self) -> DensityMatrixResult<()> {
        self.validate_basic(DEFAULT_DENSITY_MATRIX_F32_TOLERANCE)
    }
}

// =============================================================================
// Matrix helper functions
// =============================================================================

fn matrix_index(
    row: usize,
    column: usize,
    dimension: usize,
) -> DensityMatrixResult<usize> {
    if row >= dimension {
        return Err(MemoryError::OutOfBounds {
            index: row as u64,
            length: dimension as u64,
            resource: "density-matrix row".to_owned(),
        });
    }

    if column >= dimension {
        return Err(MemoryError::OutOfBounds {
            index: column as u64,
            length: dimension as u64,
            resource: "density-matrix column".to_owned(),
        });
    }

    row.checked_mul(dimension)
        .and_then(|value| value.checked_add(column))
        .ok_or_else(|| MemoryError::ArithmeticOverflow {
            operation: "row * dimension + column".to_owned(),
        })
}

fn checked_dimension(qubits: usize) -> DensityMatrixResult<usize> {
    if qubits >= usize::BITS as usize {
        return Err(MemoryError::ArithmeticOverflow {
            operation: "2^qubits dimension".to_owned(),
        });
    }

    1usize
        .checked_shl(qubits as u32)
        .ok_or_else(|| MemoryError::ArithmeticOverflow {
            operation: "2^qubits dimension".to_owned(),
        })
}

fn qubits_from_dimension(
    dimension: usize,
) -> DensityMatrixResult<QubitCount> {
    if dimension == 0 {
        return Err(MemoryError::InvalidDimension {
            dimension: "state dimension".to_owned(),
            reason: "dimension must be non-zero".to_owned(),
        });
    }

    if !dimension.is_power_of_two() {
        return Err(MemoryError::InvalidDimension {
            dimension: "state dimension".to_owned(),
            reason: "density-matrix dimension must be a power of two"
                .to_owned(),
        });
    }

    Ok(QubitCount::new(dimension.trailing_zeros() as usize))
}

fn vector_norm_squared<T>(values: &[T]) -> T::Real
where
    T: ComplexScalar,
    T::Real: DensityReal,
{
    let mut result = T::Real::zero();

    for value in values.iter().copied() {
        result = result + value.norm_squared();
    }

    result
}

fn default_tolerance<R>() -> R
where
    R: DensityReal,
{
    R::from_f64(if core::mem::size_of::<R>() == 4 {
        DEFAULT_F32_ABS_TOLERANCE as f64
    } else {
        DEFAULT_F64_ABS_TOLERANCE
    })
    .unwrap_or_else(|_| R::zero())
}

trait DensityRealFromF64: DensityReal {
    fn from_f64(value: f64) -> Result<Self, ComplexError>;
}

impl DensityRealFromF64 for f32 {
    fn from_f64(value: f64) -> Result<Self, ComplexError> {
        let converted = value as f32;

        if !converted.is_finite() && value.is_finite() {
            return Err(ComplexError::ConversionNonFinite);
        }

        Ok(converted)
    }
}

impl DensityRealFromF64 for f64 {
    fn from_f64(value: f64) -> Result<Self, ComplexError> {
        if !value.is_finite() {
            return Err(ComplexError::NonFinite);
        }

        Ok(value)
    }
}

// Make the conversion helper available through the private bound used above.
impl<T> DensityMatrix<T>
where
    T: ComplexScalar,
    T::Real: DensityReal + DensityRealFromF64,
{
    /// Returns the default tolerance for this scalar precision.
    pub fn default_tolerance() -> T::Real {
        default_tolerance::<T::Real>()
    }
}

fn map_complex_error(error: ComplexError) -> MemoryError {
    match error {
        ComplexError::NonFinite
        | ComplexError::ConversionNonFinite => MemoryError::NonFiniteValue {
            index: 0,
        },

        ComplexError::DivisionByZero
        | ComplexError::CannotNormalizeZero
        | ComplexError::NegativeRadius
        | ComplexError::NonFinitePolarCoordinate
        | ComplexError::ConversionOverflow => MemoryError::InvalidState {
            reason: error.to_string(),
        },
    }
}

fn validate_qubit(
    qubit: usize,
    qubits: QubitCount,
) -> DensityMatrixResult<()> {
    if qubit >= qubits.get() {
        return Err(MemoryError::OutOfBounds {
            index: qubit as u64,
            length: qubits.get() as u64,
            resource: "density-matrix qubit".to_owned(),
        });
    }

    Ok(())
}

fn validate_target_qubits(
    qubits: &[usize],
    total_qubits: QubitCount,
) -> DensityMatrixResult<()> {
    for &qubit in qubits {
        validate_qubit(qubit, total_qubits)?;
    }

    let mut sorted = qubits.to_vec();
    sorted.sort_unstable();

    for pair in sorted.windows(2) {
        if pair[0] == pair[1] {
            return Err(MemoryError::InvalidPermutation {
                reason: format!("duplicate target qubit {}", pair[0]),
            });
        }
    }

    Ok(())
}

fn unique_sorted(
    qubits: &[usize],
) -> DensityMatrixResult<Vec<usize>> {
    let mut result = qubits.to_vec();
    result.sort_unstable();

    for pair in result.windows(2) {
        if pair[0] == pair[1] {
            return Err(MemoryError::InvalidPermutation {
                reason: format!("duplicate qubit {}", pair[0]),
            });
        }
    }

    Ok(result)
}

fn complement_qubits(
    total: usize,
    selected: &[usize],
) -> DensityMatrixResult<Vec<usize>> {
    let mut result = Vec::with_capacity(
        total.checked_sub(selected.len()).ok_or_else(|| {
            MemoryError::InvalidArgument {
                argument: "selected qubits".to_owned(),
                context: None,
            }
        })?,
    );

    let mut selected_index = 0usize;

    for qubit in 0..total {
        if selected_index < selected.len()
            && selected[selected_index] == qubit
        {
            selected_index += 1;
        } else {
            result.push(qubit);
        }
    }

    Ok(result)
}

fn extract_bits(
    value: usize,
    qubits: &[usize],
) -> usize {
    let mut result = 0usize;

    for (position, &qubit) in qubits.iter().enumerate() {
        let bit = (value >> qubit) & 1;
        result |= bit << position;
    }

    result
}

fn insert_bits(
    base_without_targets: usize,
    qubits: &[usize],
    target_value: usize,
) -> DensityMatrixResult<usize> {
    let mut result = base_without_targets;

    for (position, &qubit) in qubits.iter().enumerate() {
        let bit = (target_value >> position) & 1;

        let mask = 1usize
            .checked_shl(qubit as u32)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "target-qubit bit mask".to_owned(),
            })?;

        if bit == 1 {
            result |= mask;
        } else {
            result &= !mask;
        }
    }

    Ok(result)
}

fn embed_two_bitsets(
    first_value: usize,
    first_qubits: &[usize],
    second_value: usize,
    second_qubits: &[usize],
) -> DensityMatrixResult<usize> {
    let mut result = 0usize;

    for (position, &qubit) in first_qubits.iter().enumerate() {
        let bit = (first_value >> position) & 1;

        if bit == 1 {
            result |=
                1usize
                    .checked_shl(qubit as u32)
                    .ok_or_else(|| MemoryError::ArithmeticOverflow {
                        operation:
                            "partial-trace retained-qubit bit mask"
                                .to_owned(),
                    })?;
        }
    }

    for (position, &qubit) in second_qubits.iter().enumerate() {
        let bit = (second_value >> position) & 1;

        if bit == 1 {
            result |=
                1usize
                    .checked_shl(qubit as u32)
                    .ok_or_else(|| MemoryError::ArithmeticOverflow {
                        operation:
                            "partial-trace traced-qubit bit mask"
                                .to_owned(),
                    })?;
        }
    }

    Ok(result)
}

fn mask_for_qubits(
    qubits: &[usize],
) -> DensityMatrixResult<usize> {
    let mut mask = 0usize;

    for &qubit in qubits {
        let bit =
            1usize
                .checked_shl(qubit as u32)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "qubit mask".to_owned(),
                })?;

        mask |= bit;
    }

    Ok(mask)
}

// =============================================================================
// Operator validation
// =============================================================================

fn validate_operator_dimension<T>(
    operator: &[T],
    dimension: usize,
) -> DensityMatrixResult<()>
where
    T: ComplexScalar,
{
    let expected =
        dimension
            .checked_mul(dimension)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "operator dimension^2".to_owned(),
            })?;

    if operator.len() != expected {
        return Err(MemoryError::StateDimensionMismatch {
            expected: expected as u64,
            actual: operator.len() as u64,
        });
    }

    for (index, value) in operator.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(MemoryError::NonFiniteValue {
                index: index as u64,
            });
        }
    }

    Ok(())
}

fn validate_unitary<T>(
    unitary: &[T],
    dimension: usize,
    tolerance: T::Real,
) -> DensityMatrixResult<()>
where
    T: ComplexScalar,
    T::Real: DensityReal,
{
    validate_operator_dimension(unitary, dimension)?;

    for row in 0..dimension {
        for column in 0..dimension {
            let mut value = T::zero();

            for k in 0..dimension {
                let lhs =
                    unitary[matrix_index(k, row, dimension)?]
                        .conjugate();

                let rhs =
                    unitary[matrix_index(k, column, dimension)?];

                value = value + lhs * rhs;
            }

            let expected = if row == column {
                T::one()
            } else {
                T::zero()
            };

            if (value - expected).magnitude() > tolerance {
                return Err(MemoryError::InvalidState {
                    reason: format!(
                        "operator is not unitary at ({row},{column})"
                    ),
                });
            }
        }
    }

    Ok(())
}

fn validate_trace_preserving_kraus<T>(
    operators: &[Vec<T>],
    dimension: usize,
    tolerance: T::Real,
) -> DensityMatrixResult<()>
where
    T: ComplexScalar,
    T::Real: DensityReal,
{
    for row in 0..dimension {
        for column in 0..dimension {
            let mut value = T::zero();

            for operator in operators {
                let lhs =
                    operator[matrix_index(row, column, dimension)?]
                        .conjugate();

                let rhs =
                    operator[matrix_index(row, column, dimension)?];

                // This intermediate form alone is insufficient for
                // K†K when row != column, so calculate the complete sum
                // below.
                let _ = lhs;
                let _ = rhs;

                for k in 0..dimension {
                    let left =
                        operator[matrix_index(k, row, dimension)?]
                            .conjugate();

                    let right =
                        operator[matrix_index(k, column, dimension)?];

                    value = value + left * right;
                }
            }

            let expected = if row == column {
                T::one()
            } else {
                T::zero()
            };

            if (value - expected).magnitude() > tolerance {
                return Err(MemoryError::InvalidState {
                    reason: format!(
                        "Kraus operators are not trace-preserving at \
                         ({row},{column})"
                    ),
                });
            }
        }
    }

    Ok(())
}

// =============================================================================
// Full-system conjugation
// =============================================================================

fn conjugate_by_operator<T>(
    state: &[T],
    operator: &[T],
    dimension: usize,
) -> DensityMatrixResult<Vec<T>>
where
    T: ComplexScalar,
{
    let elements =
        dimension
            .checked_mul(dimension)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "operator transformation dimension^2".to_owned(),
            })?;

    let mut result = vec![T::zero(); elements];

    for row in 0..dimension {
        for column in 0..dimension {
            let mut value = T::zero();

            for left in 0..dimension {
                let u_left =
                    operator[matrix_index(
                        row,
                        left,
                        dimension,
                    )?];

                if u_left.is_zero() {
                    continue;
                }

                for right in 0..dimension {
                    let rho =
                        state[matrix_index(
                            left,
                            right,
                            dimension,
                        )?];

                    if rho.is_zero() {
                        continue;
                    }

                    let u_right =
                        operator[matrix_index(
                            column,
                            right,
                            dimension,
                        )?]
                        .conjugate();

                    if u_right.is_zero() {
                        continue;
                    }

                    value =
                        value + u_left * rho * u_right;
                }
            }

            result[matrix_index(row, column, dimension)?] =
                value;
        }
    }

    Ok(result)
}

// =============================================================================
// Local operator application
// =============================================================================

fn apply_local_operator<T>(
    state: &[T],
    dimension: usize,
    target_qubits: &[usize],
    operator: &[T],
) -> DensityMatrixResult<Vec<T>>
where
    T: ComplexScalar,
{
    let local_dimension =
        checked_dimension(target_qubits.len())?;

    validate_operator_dimension(operator, local_dimension)?;

    let elements =
        dimension
            .checked_mul(dimension)
            .ok_or_else(|| MemoryError::ArithmeticOverflow {
                operation: "local operator output dimension^2".to_owned(),
            })?;

    let mut result = vec![T::zero(); elements];

    for output_row in 0..dimension {
        let output_local_row =
            extract_bits(output_row, target_qubits);

        for output_column in 0..dimension {
            let output_local_column =
                extract_bits(output_column, target_qubits);

            let mut value = T::zero();

            for local_row in 0..local_dimension {
                let old_row =
                    replace_bits(
                        output_row,
                        target_qubits,
                        local_row,
                    )?;

                let left =
                    operator[matrix_index(
                        output_local_row,
                        local_row,
                        local_dimension,
                    )?];

                if left.is_zero() {
                    continue;
                }

                for local_column in 0..local_dimension {
                    let old_column =
                        replace_bits(
                            output_column,
                            target_qubits,
                            local_column,
                        )?;

                    let rho =
                        state[matrix_index(
                            old_row,
                            old_column,
                            dimension,
                        )?];

                    if rho.is_zero() {
                        continue;
                    }

                    let right =
                        operator[matrix_index(
                            output_local_column,
                            local_column,
                            local_dimension,
                        )?]
                        .conjugate();

                    if right.is_zero() {
                        continue;
                    }

                    value =
                        value + left * rho * right;
                }
            }

            result[matrix_index(
                output_row,
                output_column,
                dimension,
            )?] = value;
        }
    }

    Ok(result)
}

fn replace_bits(
    original: usize,
    qubits: &[usize],
    replacement: usize,
) -> DensityMatrixResult<usize> {
    let mut result = original;

    for (position, &qubit) in qubits.iter().enumerate() {
        let mask =
            1usize
                .checked_shl(qubit as u32)
                .ok_or_else(|| MemoryError::ArithmeticOverflow {
                    operation: "local operator qubit mask".to_owned(),
                })?;

        result &= !mask;

        if ((replacement >> position) & 1) != 0 {
            result |= mask;
        }
    }

    Ok(result)
}

fn clamp_probability<R>(value: R) -> R
where
    R: DensityReal,
{
    let zero = R::zero();
    let one = R::one();

    if value < zero {
        zero
    } else if value > one {
        one
    } else {
        value
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn c64(real: f64, imaginary: f64) -> Complex64 {
        Complex64::new(real, imaginary)
    }

    #[test]
    fn zero_state_is_physical() {
        let state =
            DensityMatrix::<Complex64>::zero_state(QubitCount::new(2))
                .expect("zero state");

        state
            .validate_physical_f64()
            .expect("zero state must be physical");

        assert_eq!(state.dimension(), 4);
        assert_eq!(
            state.basis_probability(0).expect("p0"),
            1.0
        );
    }

    #[test]
    fn maximally_mixed_state_is_physical() {
        let state =
            DensityMatrix::<Complex64>::maximally_mixed(
                QubitCount::new(2),
            )
            .expect("mixed state");

        state
            .validate_physical_f64()
            .expect("mixed state must be physical");

        assert!((state.purity().expect("purity") - 0.25).abs() < 1e-12);
    }

    #[test]
    fn bell_state_density_matrix() {
        let amplitude = 1.0 / 2.0_f64.sqrt();

        let state = DensityMatrix::<Complex64>::from_pure_state(
            &[
                c64(amplitude, 0.0),
                c64(0.0, 0.0),
                c64(0.0, 0.0),
                c64(amplitude, 0.0),
            ],
            1e-12,
        )
        .expect("Bell density matrix");

        state
            .validate_physical_f64()
            .expect("Bell density matrix must be physical");

        assert!((state.purity().expect("purity") - 1.0).abs() < 1e-12);

        assert!(
            (state.qubit_probability(0, false).expect("p0") - 0.5).abs()
                < 1e-12
        );

        assert!(
            (state.qubit_probability(0, true).expect("p1") - 0.5).abs()
                < 1e-12
        );
    }

    #[test]
    fn pure_state_conversion_preserves_outer_product() {
        let state =
            DensityMatrix::<Complex64>::from_pure_state(
                &[c64(1.0, 0.0), c64(0.0, 0.0)],
                1e-12,
            )
            .expect("density matrix");

        assert_eq!(
            state.get(0, 0).expect("rho00"),
            c64(1.0, 0.0)
        );

        assert_eq!(
            state.get(1, 1).expect("rho11"),
            c64(0.0, 0.0)
        );
    }

    #[test]
    fn unitary_identity_preserves_state() {
        let mut state =
            DensityMatrix::<Complex64>::zero_state(
                QubitCount::new(1),
            )
            .expect("state");

        let identity = vec![
            c64(1.0, 0.0),
            c64(0.0, 0.0),
            c64(0.0, 0.0),
            c64(1.0, 0.0),
        ];

        state
            .apply_unitary(&identity, 1e-12)
            .expect("identity");

        assert_eq!(
            state.basis_probability(0).expect("p0"),
            1.0
        );
    }

    #[test]
    fn x_unitary_maps_zero_to_one() {
        let mut state =
            DensityMatrix::<Complex64>::zero_state(
                QubitCount::new(1),
            )
            .expect("state");

        let x = vec![
            c64(0.0, 0.0),
            c64(1.0, 0.0),
            c64(1.0, 0.0),
            c64(0.0, 0.0),
        ];

        state
            .apply_unitary(&x, 1e-12)
            .expect("X");

        assert!(state.basis_probability(0).expect("p0") < 1e-12);
        assert!((state.basis_probability(1).expect("p1") - 1.0).abs() < 1e-12);
    }

    #[test]
    fn measurement_probability_and_projection() {
        let amplitude = 1.0 / 2.0_f64.sqrt();

        let mut state =
            DensityMatrix::<Complex64>::from_pure_state(
                &[
                    c64(amplitude, 0.0),
                    c64(0.0, 0.0),
                    c64(0.0, 0.0),
                    c64(amplitude, 0.0),
                ],
                1e-12,
            )
            .expect("Bell state");

        let probability =
            state
                .measurement_probability(&[0], 0)
                .expect("measurement probability");

        assert!((probability - 0.5).abs() < 1e-12);

        state
            .project_measurement(&[0], 0, 1e-12)
            .expect("projection");

        state
            .validate_physical_f64()
            .expect("projected state");

        assert!(
            (state.basis_probability(0).expect("p00") - 1.0).abs()
                < 1e-12
        );

        assert!(
            state.basis_probability(3).expect("p11") < 1e-12
        );
    }

    #[test]
    fn partial_trace_of_bell_state_is_maximally_mixed() {
        let amplitude = 1.0 / 2.0_f64.sqrt();

        let state =
            DensityMatrix::<Complex64>::from_pure_state(
                &[
                    c64(amplitude, 0.0),
                    c64(0.0, 0.0),
                    c64(0.0, 0.0),
                    c64(amplitude, 0.0),
                ],
                1e-12,
            )
            .expect("Bell state");

        let reduced =
            state.partial_trace(&[1]).expect("partial trace");

        reduced
            .validate_physical_f64()
            .expect("reduced state");

        assert!((reduced.basis_probability(0).expect("p0") - 0.5).abs() < 1e-12);
        assert!((reduced.basis_probability(1).expect("p1") - 0.5).abs() < 1e-12);
    }

    #[test]
    fn reset_returns_zero_state() {
        let mut state =
            DensityMatrix::<Complex64>::maximally_mixed(
                QubitCount::new(1),
            )
            .expect("mixed state");

        state.reset_qubits(&[0]).expect("reset");

        state
            .validate_physical_f64()
            .expect("reset state");

        assert!((state.basis_probability(0).expect("p0") - 1.0).abs() < 1e-12);
        assert!(state.basis_probability(1).expect("p1") < 1e-12);
    }

    #[test]
    fn tensor_product_has_expected_dimension() {
        let left =
            DensityMatrix::<Complex64>::zero_state(
                QubitCount::new(1),
            )
            .expect("left");

        let right =
            DensityMatrix::<Complex64>::zero_state(
                QubitCount::new(2),
            )
            .expect("right");

        let combined =
            left.tensor_product(&right)
                .expect("tensor product");

        assert_eq!(combined.qubit_count(), QubitCount::new(3));
        assert_eq!(combined.dimension(), 8);

        assert!(
            (combined.basis_probability(0).expect("p0") - 1.0).abs()
                < 1e-12
        );
    }

    #[test]
    fn memory_requirement_is_checked() {
        let requirement =
            DensityMatrix::<Complex64>::memory_requirement(
                QubitCount::new(2),
            )
            .expect("requirement");

        assert_eq!(requirement.dimension(), 4);
        assert_eq!(requirement.elements(), 16);
        assert_eq!(requirement.scalar_bytes(), 16);
        assert_eq!(requirement.bytes(), 256);
    }

    #[test]
    fn dephasing_removes_selected_coherence() {
        let amplitude = 1.0 / 2.0_f64.sqrt();

        let mut state =
            DensityMatrix::<Complex64>::from_pure_state(
                &[
                    c64(amplitude, 0.0),
                    c64(amplitude, 0.0),
                ],
                1e-12,
            )
            .expect("plus state");

        assert!(
            state.get(0, 1).expect("coherence").magnitude()
                > 0.9
        );

        state.dephase_qubits(&[0]).expect("dephase");

        assert!(
            state.get(0, 1).expect("coherence").magnitude()
                < 1e-12
        );

        state
            .validate_physical_f64()
            .expect("dephased state");
    }

    #[test]
    fn expectation_value_of_identity_is_one() {
        let state =
            DensityMatrix::<Complex64>::maximally_mixed(
                QubitCount::new(1),
            )
            .expect("state");

        let identity = vec![
            c64(1.0, 0.0),
            c64(0.0, 0.0),
            c64(0.0, 0.0),
            c64(1.0, 0.0),
        ];

        let expectation =
            state
                .expectation_value(&identity)
                .expect("expectation");

        assert!((expectation.real() - 1.0).abs() < 1e-12);
        assert!(expectation.imaginary().abs() < 1e-12);
    }
}