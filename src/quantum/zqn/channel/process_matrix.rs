//! Zamani Quantum Noise (ZQN) — Process Matrix Representation
//!
//! This module owns a representation of a completely-positive quantum process
//! using a finite complex process matrix.
//!
//! # Architectural position
//!
//! `ProcessMatrix` is a mathematical representation.
//!
//! It is NOT:
//!
//! - the canonical Zamani Quantum IR;
//! - a quantum-state representation;
//! - a simulator;
//! - a hardware object;
//! - a calibration object;
//! - a noise model;
//! - a QEC decoder;
//! - a routing structure;
//! - a scheduler;
//! - a vendor API.
//!
//! The canonical semantic program remains owned by:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! ZQN owns the physical-noise/process representation that downstream systems
//! may consume.
//!
//! # Mathematical convention
//!
//! For a quantum channel
//!
//! ```text
//! Φ : L(H_in) -> L(H_out)
//! ```
//!
//! with input dimension `d_in` and output dimension `d_out`, this module uses
//! the unnormalized Choi/process-matrix convention
//!
//! ```text
//! J(Φ) = Σ(i,j) |i><j| ⊗ Φ(|i><j|)
//! ```
//!
//! with matrix dimension
//!
//! ```text
//! d_in * d_out
//! ```
//!
//! and flattened matrix index
//!
//! ```text
//! index(input, output) = input * d_out + output
//! ```
//!
//! For a trace-preserving channel:
//!
//! ```text
//! Tr_output(J) = I_input
//! ```
//!
//! and consequently:
//!
//! ```text
//! Tr(J) = d_in
//! ```
//!
//! A normalized process state is obtained by dividing by `d_in`.
//!
//! # Important distinction
//!
//! A process matrix is not itself a density matrix of the physical system.
//!
//! It represents a quantum operation through its Choi representation.
//!
//! # Generality
//!
//! This implementation deliberately does NOT assume:
//!
//! - qubits;
//! - dimension two;
//! - one- or two-qubit gates;
//! - a particular gate set;
//! - a particular hardware technology;
//! - a fixed number of quantum resources;
//! - a fixed process arity.
//!
//! `input_dimension` and `output_dimension` are data.
//!
//! Therefore the same representation can describe processes involving:
//!
//! - qubits;
//! - qudits;
//! - truncated bosonic modes;
//! - other finite-dimensional quantum subsystems;
//! - composite finite-dimensional systems.
//!
//! Infinite-dimensional physical systems require an explicit finite
//! representation/truncation before a materialized `ProcessMatrix` can exist.
//! This file does not silently pretend that a finite matrix is an infinite
//! object.
//!
//! # Scalability
//!
//! There is NO artificial ZQN machine-size limit in this file.
//!
//! In particular, this file does not define:
//!
//! ```text
//! MAX_QUBITS
//! MAX_DIMENSION
//! MAX_MATRIX_SIZE
//! MAX_PROCESS_SIZE
//! ```
//!
//! A materialized matrix necessarily requires memory proportional to the square
//! of its matrix dimension. That is a mathematical/property-of-representation
//! requirement, not a Zamani architectural machine-size limit.
//!
//! Allocation uses checked arithmetic and fallible `Vec` reservation.
//!
//! Therefore:
//!
//! ```text
//! insufficient host resources
//!         !=
//! architectural unsupported size
//! ```
//!
//! If resources are insufficient, construction returns an explicit error.
//!
//! No unsafe allocation primitive is used.
//!
//! # Resource model
//!
//! This file does not invent an arbitrary resource ceiling.
//!
//! A caller may impose an external resource policy before constructing a
//! process matrix. This module only rejects:
//!
//! - mathematically impossible dimensions;
//! - integer overflow;
//! - malformed matrix lengths;
//! - allocation failure;
//! - invalid numerical values;
//! - explicitly requested invalid operations.
//!
//! # Numerical representation
//!
//! The current implementation uses:
//!
//! ```text
//! Complex64 = f64 + i*f64
//! ```
//!
//! This is a representation choice.
//!
//! It is NOT a semantic limitation of ZQN.
//!
//! A future higher-precision/exact representation can implement the same
//! conceptual process-matrix semantics without changing the architectural
//! contract.
//!
//! # Positive semidefinite semantics
//!
//! A valid Choi matrix of a completely positive map must be positive
//! semidefinite.
//!
//! This module therefore provides:
//!
//! - Hermiticity validation;
//! - positive-semidefinite validation;
//! - trace-preservation validation;
//! - complete channel validation.
//!
//! Numerical validation always requires an explicit tolerance.
//!
//! No tolerance is silently selected for mathematically sensitive operations.
//!
//! # Determinism
//!
//! This module:
//!
//! - owns no RNG;
//! - reads no clock;
//! - uses no global mutable state;
//! - performs no I/O;
//! - performs no network operations;
//! - performs no hidden sampling.
//!
//! Matrix construction and validation are deterministic for identical inputs
//! and tolerances.
//!
//! # Qubit identity integration
//!
//! A process matrix does not inherently identify a physical resource.
//!
//! Consequently this file intentionally does NOT define another `QubitId` or
//! `PhysicalQubitId`.
//!
//! When another ZQN layer associates this process with resources, it MUST use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! as established by the canonical Quantum IR.
//!
//! A resource association belongs in the operation/noise/application layer,
//! not inside the mathematical matrix representation.
//!
//! # Integration
//!
//! Intended downstream flow:
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                       noise operation
//!                              │
//!                              ▼
//!                         ZQN channel
//!                              │
//!                              ▼
//!                      ProcessMatrix
//!                              │
//!             ┌────────────────┼────────────────┐
//!             │                │                │
//!             ▼                ▼                ▼
//!          simulator       characterization    target
//!             │                │                │
//!             ▼                ▼                ▼
//!          execution       calibration       lowering
//! ```
//!
//! Conversion from/to other channel representations belongs to the channel
//! representation/conversion layer. This file deliberately does not depend on
//! those higher layers, allowing it to be completed independently.
//!
//! # Serialization
//!
//! This file does NOT define a wire format.
//!
//! Serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! A serializer should persist:
//!
//! - representation/schema version;
//! - input dimension;
//! - output dimension;
//! - matrix elements;
//! - any normalization convention explicitly required by the schema.
//!
//! The in-memory layout must not accidentally become the external protocol.
//!
//! # Security
//!
//! Public constructors:
//!
//! - validate dimensions;
//! - use checked dimension multiplication;
//! - use fallible allocation;
//! - reject non-finite matrix values;
//! - do not silently normalize malformed data;
//! - do not clamp invalid values;
//! - do not invoke unsafe code.
//!
//! Computationally expensive validation methods are explicit operations rather
//! than being hidden inside ordinary element access.
//!
//! # Thread safety
//!
//! `ProcessMatrix` contains owned immutable-after-construction-compatible data
//! but exposes mutable element operations through `&mut self` only.
//!
//! No interior mutability or global state is used.
//!
//! The type is therefore suitable for ordinary Rust ownership/concurrency
//! patterns.
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
//! - no unsafe code;
//! - standard library only.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. dimensions are checked;
//! 2. matrix storage is fallibly allocated;
//! 3. overflow is rejected;
//! 4. non-finite values are rejected;
//! 5. matrix indexing is bounds checked;
//! 6. Hermiticity can be validated;
//! 7. positive semidefiniteness can be validated;
//! 8. trace preservation can be validated;
//! 9. normalized process matrices are explicitly distinguishable from
//!    unnormalized matrices;
//! 10. no fixed qubit count exists;
//! 11. no vendor assumptions exist;
//! 12. no unsafe code exists;
//! 13. no RNG/global state exists;
//! 14. no QubitId is redefined;
//! 15. no serialization format is accidentally imposed;
//! 16. all public failure modes are explicit;
//! 17. large dimensions fail through resource-aware errors rather than
//!     artificial hard-coded limits.
//!
//! # Examples
//!
//! ```
//! use crate::quantum::zqn::channel::process_matrix::{
//!     Complex64,
//!     ProcessMatrix,
//! };
//!
//! // Identity channel on a one-dimensional system.
//! let matrix = ProcessMatrix::identity_channel(1)
//!     .expect("one-dimensional identity process is representable");
//!
//! assert!(matrix.is_hermitian(1.0e-12).unwrap());
//! assert!(matrix.is_trace_preserving(1.0e-12).unwrap());
//! ```
//!
//! The examples use the canonical module path expected once this file is
//! exposed by `channel/mod.rs`.

// -----------------------------------------------------------------------------
// Safety contract
// -----------------------------------------------------------------------------

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;
use std::slice;

// =============================================================================
// Numerical primitive
// =============================================================================

/// Complex scalar used by the process-matrix representation.
///
/// This is deliberately implemented locally instead of requiring an external
/// complex-number dependency. The representation can later be replaced by a
/// higher-precision numerical backend without changing the process-matrix
/// semantic contract.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub fn norm_sqr(self) -> f64 {
        self.re.mul_add(self.re, self.im * self.im)
    }

    /// Returns the magnitude.
    #[must_use]
    pub fn norm(self) -> f64 {
        self.norm_sqr().sqrt()
    }

    /// Returns whether both components are finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    /// Returns whether the value is numerically close to zero.
    #[must_use]
    pub fn is_zero_within(self, tolerance: f64) -> bool {
        self.norm() <= tolerance
    }

    /// Returns the real part.
    #[must_use]
    pub const fn real(self) -> f64 {
        self.re
    }

    /// Returns the imaginary part.
    #[must_use]
    pub const fn imaginary(self) -> f64 {
        self.im
    }

    /// Returns the product of two complex values.
    #[must_use]
    pub fn mul(self, other: Self) -> Self {
        Self {
            re: self.re.mul_add(other.re, -(self.im * other.im)),
            im: self.re.mul_add(other.im, self.im * other.re),
        }
    }

    /// Returns the sum of two complex values.
    #[must_use]
    pub fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    /// Returns the difference of two complex values.
    #[must_use]
    pub fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    /// Returns the value multiplied by a real scalar.
    #[must_use]
    pub fn scale(self, scalar: f64) -> Self {
        Self {
            re: self.re * scalar,
            im: self.im * scalar,
        }
    }

    /// Returns whether the value is numerically real.
    #[must_use]
    pub fn is_real_within(self, tolerance: f64) -> bool {
        self.im.abs() <= tolerance
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

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by process-matrix construction and validation.
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessMatrixError {
    /// Input dimension was zero.
    ZeroInputDimension,

    /// Output dimension was zero.
    ZeroOutputDimension,

    /// `input_dimension * output_dimension` overflowed `usize`.
    DimensionOverflow {
        /// Input Hilbert-space dimension.
        input_dimension: usize,

        /// Output Hilbert-space dimension.
        output_dimension: usize,
    },

    /// Matrix element count overflowed while calculating the square matrix
    /// storage requirement.
    MatrixSizeOverflow {
        /// Matrix side length.
        dimension: usize,
    },

    /// The supplied matrix has the wrong number of elements.
    InvalidElementCount {
        /// Expected element count.
        expected: usize,

        /// Actual element count.
        actual: usize,
    },

    /// The requested allocation could not be reserved.
    AllocationFailure {
        /// Number of elements requested.
        elements: usize,

        /// Element size in bytes.
        element_size: usize,
    },

    /// A matrix element is not finite.
    NonFiniteElement {
        /// Flat matrix index.
        index: usize,

        /// Invalid value.
        value: Complex64,
    },

    /// Tolerance is invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        tolerance: f64,
    },

    /// A matrix index is outside the matrix.
    IndexOutOfBounds {
        /// Requested row.
        row: usize,

        /// Requested column.
        column: usize,

        /// Matrix side length.
        dimension: usize,
    },

    /// A flat element index is outside the matrix.
    ElementIndexOutOfBounds {
        /// Requested flat index.
        index: usize,

        /// Number of stored elements.
        length: usize,
    },

    /// The matrix is not Hermitian within the requested tolerance.
    NotHermitian {
        /// Row index.
        row: usize,

        /// Column index.
        column: usize,

        /// Difference between an element and the conjugate transpose element.
        difference: Complex64,
    },

    /// The matrix is not positive semidefinite within the requested tolerance.
    NotPositiveSemidefinite {
        /// Principal pivot at which the numerical PSD test failed.
        index: usize,

        /// Numerical value responsible for the failure.
        value: f64,
    },

    /// The process does not satisfy the trace-preservation condition.
    NotTracePreserving {
        /// Input-space row.
        row: usize,

        /// Input-space column.
        column: usize,

        /// Calculated partial-trace value.
        actual: Complex64,

        /// Required identity value.
        expected: Complex64,
    },

    /// The process matrix trace does not match the supplied expected trace.
    TraceMismatch {
        /// Actual trace.
        actual: Complex64,

        /// Expected trace.
        expected: Complex64,
    },

    /// Normalization would divide by zero.
    InvalidNormalizationDimension,

    /// An operation requires a square matrix but the representation does not
    /// satisfy the required relation.
    InvalidOperationDimensions,

    /// A numerical operation produced a non-finite result.
    NumericalFailure {
        /// Description of the operation.
        operation: &'static str,
    },
}

impl fmt::Display for ProcessMatrixError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroInputDimension => {
                write!(formatter, "process input dimension must be greater than zero")
            }
            Self::ZeroOutputDimension => {
                write!(formatter, "process output dimension must be greater than zero")
            }
            Self::DimensionOverflow {
                input_dimension,
                output_dimension,
            } => write!(
                formatter,
                "process dimension overflow: input={} output={}",
                input_dimension, output_dimension
            ),
            Self::MatrixSizeOverflow { dimension } => {
                write!(
                    formatter,
                    "process matrix element-count overflow for dimension {}",
                    dimension
                )
            }
            Self::InvalidElementCount { expected, actual } => write!(
                formatter,
                "invalid process matrix element count: expected {}, got {}",
                expected, actual
            ),
            Self::AllocationFailure {
                elements,
                element_size,
            } => write!(
                formatter,
                "unable to allocate process matrix storage: {} elements of {} bytes",
                elements, element_size
            ),
            Self::NonFiniteElement { index, value } => write!(
                formatter,
                "process matrix element {} is non-finite: {}",
                index, value
            ),
            Self::InvalidTolerance { tolerance } => {
                write!(formatter, "invalid numerical tolerance: {}", tolerance)
            }
            Self::IndexOutOfBounds {
                row,
                column,
                dimension,
            } => write!(
                formatter,
                "matrix index ({}, {}) is outside {}x{} matrix",
                row, column, dimension, dimension
            ),
            Self::ElementIndexOutOfBounds { index, length } => write!(
                formatter,
                "matrix element index {} is outside storage of length {}",
                index, length
            ),
            Self::NotHermitian {
                row,
                column,
                difference,
            } => write!(
                formatter,
                "process matrix is not Hermitian at ({}, {}); difference={}",
                row, column, difference
            ),
            Self::NotPositiveSemidefinite { index, value } => write!(
                formatter,
                "process matrix failed positive-semidefinite validation at pivot {}: {}",
                index, value
            ),
            Self::NotTracePreserving {
                row,
                column,
                actual,
                expected,
            } => write!(
                formatter,
                "process matrix is not trace preserving at ({}, {}): actual={}, expected={}",
                row, column, actual, expected
            ),
            Self::TraceMismatch { actual, expected } => write!(
                formatter,
                "process matrix trace mismatch: actual={}, expected={}",
                actual, expected
            ),
            Self::InvalidNormalizationDimension => {
                write!(formatter, "cannot normalize a process matrix with zero input dimension")
            }
            Self::InvalidOperationDimensions => {
                write!(formatter, "process-matrix operation has incompatible dimensions")
            }
            Self::NumericalFailure { operation } => {
                write!(formatter, "non-finite numerical result during {}", operation)
            }
        }
    }
}

impl Error for ProcessMatrixError {}

// =============================================================================
// Process matrix
// =============================================================================

/// Materialized finite-dimensional quantum process matrix.
///
/// The matrix uses row-major storage and the index convention:
///
/// ```text
/// flat_index(row, column) = row * matrix_dimension + column
/// ```
///
/// where:
///
/// ```text
/// matrix_dimension = input_dimension * output_dimension
/// ```
///
/// The input/output dimensions are independent. No qubit assumption exists.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessMatrix {
    input_dimension: usize,
    output_dimension: usize,
    elements: Vec<Complex64>,
}

impl ProcessMatrix {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates a zero-valued process matrix.
    ///
    /// Allocation is fallible.
    ///
    /// No arbitrary maximum dimension is imposed.
    pub fn zero(
        input_dimension: usize,
        output_dimension: usize,
    ) -> Result<Self, ProcessMatrixError> {
        let matrix_dimension = checked_matrix_dimension(
            input_dimension,
            output_dimension,
        )?;

        let element_count = checked_square(matrix_dimension)?;

        let mut elements = Vec::new();

        elements
            .try_reserve_exact(element_count)
            .map_err(|_| ProcessMatrixError::AllocationFailure {
                elements: element_count,
                element_size: std::mem::size_of::<Complex64>(),
            })?;

        elements.resize(element_count, Complex64::ZERO);

        Ok(Self {
            input_dimension,
            output_dimension,
            elements,
        })
    }

    /// Creates a process matrix from row-major complex elements.
    ///
    /// All elements must be finite.
    ///
    /// The matrix is not automatically validated as:
    ///
    /// - Hermitian;
    /// - positive semidefinite;
    /// - trace preserving.
    ///
    /// Those properties depend on the process being represented and are
    /// therefore checked explicitly by the corresponding validation methods.
    pub fn from_elements(
        input_dimension: usize,
        output_dimension: usize,
        elements: Vec<Complex64>,
    ) -> Result<Self, ProcessMatrixError> {
        let matrix_dimension =
            checked_matrix_dimension(input_dimension, output_dimension)?;

        let expected = checked_square(matrix_dimension)?;

        if elements.len() != expected {
            return Err(ProcessMatrixError::InvalidElementCount {
                expected,
                actual: elements.len(),
            });
        }

        validate_elements(&elements)?;

        Ok(Self {
            input_dimension,
            output_dimension,
            elements,
        })
    }

    /// Creates a process matrix from a flat slice.
    ///
    /// The slice is copied into owned storage.
    pub fn from_slice(
        input_dimension: usize,
        output_dimension: usize,
        elements: &[Complex64],
    ) -> Result<Self, ProcessMatrixError> {
        let matrix_dimension =
            checked_matrix_dimension(input_dimension, output_dimension)?;

        let expected = checked_square(matrix_dimension)?;

        if elements.len() != expected {
            return Err(ProcessMatrixError::InvalidElementCount {
                expected,
                actual: elements.len(),
            });
        }

        validate_elements(elements)?;

        let mut owned = Vec::new();

        owned
            .try_reserve_exact(elements.len())
            .map_err(|_| ProcessMatrixError::AllocationFailure {
                elements: elements.len(),
                element_size: std::mem::size_of::<Complex64>(),
            })?;

        owned.extend_from_slice(elements);

        Ok(Self {
            input_dimension,
            output_dimension,
            elements: owned,
        })
    }

    /// Creates the identity channel's Choi/process matrix.
    ///
    /// For input dimension `d` and output dimension `d`, the unnormalized
    /// identity-channel Choi matrix is:
    ///
    /// ```text
    /// |Ω><Ω|
    ///
    /// |Ω> = Σ_i |i,i>
    /// ```
    ///
    /// This method intentionally does not assume `d == 2`.
    pub fn identity_channel(
        dimension: usize,
    ) -> Result<Self, ProcessMatrixError> {
        if dimension == 0 {
            return Err(ProcessMatrixError::ZeroInputDimension);
        }

        let matrix_dimension = checked_matrix_dimension(dimension, dimension)?;
        let element_count = checked_square(matrix_dimension)?;

        let mut elements = Vec::new();

        elements
            .try_reserve_exact(element_count)
            .map_err(|_| ProcessMatrixError::AllocationFailure {
                elements: element_count,
                element_size: std::mem::size_of::<Complex64>(),
            })?;

        elements.resize(element_count, Complex64::ZERO);

        // |Ω><Ω| has unit real entries at:
        //
        // row = i*d + i
        // col = j*d + j
        //
        // for every input basis pair i,j.
        for i in 0..dimension {
            let row = i * dimension + i;

            for j in 0..dimension {
                let column = j * dimension + j;
                let index = row * matrix_dimension + column;

                elements[index] = Complex64::ONE;
            }
        }

        Ok(Self {
            input_dimension: dimension,
            output_dimension: dimension,
            elements,
        })
    }

    /// Creates a normalized identity-channel process state.
    ///
    /// The matrix is:
    ///
    /// ```text
    /// |Ω><Ω| / d
    /// ```
    ///
    /// and therefore has trace one.
    pub fn normalized_identity_channel(
        dimension: usize,
    ) -> Result<Self, ProcessMatrixError> {
        let matrix = Self::identity_channel(dimension)?;

        matrix.normalized()
    }

    // =========================================================================
    // Dimensions
    // =========================================================================

    /// Returns the input Hilbert-space dimension.
    #[must_use]
    pub const fn input_dimension(&self) -> usize {
        self.input_dimension
    }

    /// Returns the output Hilbert-space dimension.
    #[must_use]
    pub const fn output_dimension(&self) -> usize {
        self.output_dimension
    }

    /// Returns the side length of the materialized process matrix.
    #[must_use]
    pub fn matrix_dimension(&self) -> usize {
        // The invariant was established during construction.
        self.input_dimension * self.output_dimension
    }

    /// Returns the number of stored complex elements.
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether this process maps between equal-dimensional spaces.
    #[must_use]
    pub const fn is_dimension_preserving(&self) -> bool {
        self.input_dimension == self.output_dimension
    }

    // =========================================================================
    // Element access
    // =========================================================================

    /// Returns an element by matrix row and column.
    ///
    /// Bounds are checked explicitly.
    pub fn get(
        &self,
        row: usize,
        column: usize,
    ) -> Result<Complex64, ProcessMatrixError> {
        let index = self.flat_index(row, column)?;
        Ok(self.elements[index])
    }

    /// Sets an element by matrix row and column.
    ///
    /// The supplied value must be finite.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: Complex64,
    ) -> Result<(), ProcessMatrixError> {
        if !value.is_finite() {
            let index = self.flat_index(row, column)?;

            return Err(ProcessMatrixError::NonFiniteElement {
                index,
                value,
            });
        }

        let index = self.flat_index(row, column)?;
        self.elements[index] = value;

        Ok(())
    }

    /// Returns an element by flat row-major index.
    pub fn get_flat(
        &self,
        index: usize,
    ) -> Result<Complex64, ProcessMatrixError> {
        self.elements
            .get(index)
            .copied()
            .ok_or(ProcessMatrixError::ElementIndexOutOfBounds {
                index,
                length: self.elements.len(),
            })
    }

    /// Sets an element by flat row-major index.
    ///
    /// The supplied value must be finite.
    pub fn set_flat(
        &mut self,
        index: usize,
        value: Complex64,
    ) -> Result<(), ProcessMatrixError> {
        if !value.is_finite() {
            return Err(ProcessMatrixError::NonFiniteElement {
                index,
                value,
            });
        }

        let element = self.elements.get_mut(index).ok_or(
            ProcessMatrixError::ElementIndexOutOfBounds {
                index,
                length: self.elements.len(),
            },
        )?;

        *element = value;

        Ok(())
    }

    /// Returns the matrix elements as an immutable slice.
    #[must_use]
    pub fn as_slice(&self) -> &[Complex64] {
        &self.elements
    }

    /// Returns the matrix elements as an iterator.
    pub fn iter(&self) -> slice::Iter<'_, Complex64> {
        self.elements.iter()
    }

    /// Returns a mutable element iterator.
    ///
    /// The iterator itself cannot enforce the finite-value invariant if callers
    /// directly mutate `Complex64`. Consequently the resulting matrix should
    /// be revalidated with `validate_finite()` before being treated as a
    /// validated process.
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, Complex64> {
        self.elements.iter_mut()
    }

    // =========================================================================
    // Basic matrix properties
    // =========================================================================

    /// Returns the matrix trace.
    ///
    /// The trace is the sum of diagonal elements.
    pub fn trace(&self) -> Result<Complex64, ProcessMatrixError> {
        let dimension = self.matrix_dimension();
        let mut result = Complex64::ZERO;

        for index in 0..dimension {
            let element = self.elements[index * dimension + index];

            result = result.add(element);

            if !result.is_finite() {
                return Err(ProcessMatrixError::NumericalFailure {
                    operation: "process-matrix trace",
                });
            }
        }

        Ok(result)
    }

    /// Returns a normalized copy of the process matrix.
    ///
    /// The current ZQN convention normalizes by the input dimension:
    ///
    /// ```text
    /// J_normalized = J / d_in
    /// ```
    ///
    /// This is the conventional normalization that turns a trace-preserving
    /// channel's Choi matrix into a trace-one process state.
    pub fn normalized(&self) -> Result<Self, ProcessMatrixError> {
        if self.input_dimension == 0 {
            return Err(ProcessMatrixError::InvalidNormalizationDimension);
        }

        let scale = 1.0 / self.input_dimension as f64;

        if !scale.is_finite() {
            return Err(ProcessMatrixError::NumericalFailure {
                operation: "process-matrix normalization",
            });
        }

        let mut elements = Vec::new();

        elements
            .try_reserve_exact(self.elements.len())
            .map_err(|_| ProcessMatrixError::AllocationFailure {
                elements: self.elements.len(),
                element_size: std::mem::size_of::<Complex64>(),
            })?;

        for element in &self.elements {
            let scaled = element.scale(scale);

            if !scaled.is_finite() {
                return Err(ProcessMatrixError::NumericalFailure {
                    operation: "process-matrix normalization",
                });
            }

            elements.push(scaled);
        }

        Ok(Self {
            input_dimension: self.input_dimension,
            output_dimension: self.output_dimension,
            elements,
        })
    }

    /// Returns whether the matrix is trace one within the supplied tolerance.
    pub fn is_normalized(
        &self,
        tolerance: f64,
    ) -> Result<bool, ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        let trace = self.trace()?;

        Ok((trace.re - 1.0).abs() <= tolerance
            && trace.im.abs() <= tolerance)
    }

    /// Returns whether the trace equals the input dimension within tolerance.
    ///
    /// For a trace-preserving channel under the unnormalized Choi convention,
    /// this is a necessary consequence of trace preservation.
    pub fn has_channel_trace(
        &self,
        tolerance: f64,
    ) -> Result<bool, ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        let trace = self.trace()?;
        let expected = self.input_dimension as f64;

        Ok((trace.re - expected).abs() <= tolerance
            && trace.im.abs() <= tolerance)
    }

    /// Validates that every element is finite.
    pub fn validate_finite(&self) -> Result<(), ProcessMatrixError> {
        validate_elements(&self.elements)
    }

    // =========================================================================
    // Hermiticity
    // =========================================================================

    /// Checks Hermiticity within an explicit tolerance.
    pub fn is_hermitian(
        &self,
        tolerance: f64,
    ) -> Result<bool, ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        let dimension = self.matrix_dimension();

        for row in 0..dimension {
            for column in 0..dimension {
                let lhs = self.elements[row * dimension + column];
                let rhs = self.elements[column * dimension + row].conjugate();

                let difference = lhs.sub(rhs);

                if difference.norm() > tolerance {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Validates Hermiticity.
    pub fn validate_hermitian(
        &self,
        tolerance: f64,
    ) -> Result<(), ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        let dimension = self.matrix_dimension();

        for row in 0..dimension {
            for column in row..dimension {
                let lhs = self.elements[row * dimension + column];
                let rhs = self.elements[column * dimension + row].conjugate();

                let difference = lhs.sub(rhs);

                if difference.norm() > tolerance {
                    return Err(ProcessMatrixError::NotHermitian {
                        row,
                        column,
                        difference,
                    });
                }
            }
        }

        Ok(())
    }

    // =========================================================================
    // Positive semidefinite validation
    // =========================================================================

    /// Checks positive semidefiniteness using a Hermitian Cholesky-style
    /// factorization with numerical tolerance.
    ///
    /// For a Hermitian matrix `A`, the matrix is positive semidefinite when the
    /// factorization can proceed without a materially negative pivot.
    ///
    /// This method does not silently repair a matrix.
    pub fn is_positive_semidefinite(
        &self,
        tolerance: f64,
    ) -> Result<bool, ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        if !self.is_hermitian(tolerance)? {
            return Ok(false);
        }

        let dimension = self.matrix_dimension();

        if dimension == 0 {
            return Ok(true);
        }

        // Lower-triangular Cholesky-like storage.
        //
        // We allocate only after all dimension arithmetic has already been
        // validated. Fallible reservation prevents this mathematical check
        // from introducing an uncontrolled allocation panic.
        let element_count = checked_square(dimension)?;

        let mut lower = Vec::new();

        lower
            .try_reserve_exact(element_count)
            .map_err(|_| ProcessMatrixError::AllocationFailure {
                elements: element_count,
                element_size: std::mem::size_of::<Complex64>(),
            })?;

        lower.resize(element_count, Complex64::ZERO);

        for row in 0..dimension {
            for column in 0..=row {
                let mut sum = self.elements[row * dimension + column];

                for k in 0..column {
                    let left = lower[row * dimension + k];
                    let right = lower[column * dimension + k].conjugate();

                    sum = sum.sub(left.mul(right));
                }

                if !sum.is_finite() {
                    return Err(ProcessMatrixError::NumericalFailure {
                        operation: "positive-semidefinite factorization",
                    });
                }

                if row == column {
                    let diagonal = sum.re;

                    if sum.im.abs() > tolerance {
                        return Ok(false);
                    }

                    if diagonal < -tolerance {
                        return Ok(false);
                    }

                    if diagonal <= tolerance {
                        // A numerically zero pivot can be accepted only when
                        // the remaining residual is also numerically zero.
                        lower[row * dimension + column] = Complex64::ZERO;
                    } else {
                        let root = diagonal.sqrt();

                        if !root.is_finite() {
                            return Err(ProcessMatrixError::NumericalFailure {
                                operation: "positive-semidefinite square root",
                            });
                        }

                        lower[row * dimension + column] =
                            Complex64::new(root, 0.0);
                    }
                } else {
                    let pivot =
                        lower[column * dimension + column].re;

                    if pivot.abs() <= tolerance {
                        if sum.norm() > tolerance {
                            return Ok(false);
                        }

                        lower[row * dimension + column] =
                            Complex64::ZERO;
                    } else {
                        let value = sum.scale(1.0 / pivot);

                        if !value.is_finite() {
                            return Err(ProcessMatrixError::NumericalFailure {
                                operation: "positive-semidefinite factorization",
                            });
                        }

                        lower[row * dimension + column] = value;
                    }
                }
            }
        }

        Ok(true)
    }

    /// Validates positive semidefiniteness.
    pub fn validate_positive_semidefinite(
        &self,
        tolerance: f64,
    ) -> Result<(), ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        self.validate_hermitian(tolerance)?;

        let dimension = self.matrix_dimension();

        if dimension == 0 {
            return Ok(());
        }

        let element_count = checked_square(dimension)?;

        let mut lower = Vec::new();

        lower
            .try_reserve_exact(element_count)
            .map_err(|_| ProcessMatrixError::AllocationFailure {
                elements: element_count,
                element_size: std::mem::size_of::<Complex64>(),
            })?;

        lower.resize(element_count, Complex64::ZERO);

        for row in 0..dimension {
            for column in 0..=row {
                let mut sum = self.elements[row * dimension + column];

                for k in 0..column {
                    let left = lower[row * dimension + k];
                    let right = lower[column * dimension + k].conjugate();

                    sum = sum.sub(left.mul(right));
                }

                if !sum.is_finite() {
                    return Err(ProcessMatrixError::NumericalFailure {
                        operation: "positive-semidefinite validation",
                    });
                }

                if row == column {
                    if sum.im.abs() > tolerance {
                        return Err(ProcessMatrixError::NotPositiveSemidefinite {
                            index: row,
                            value: sum.re,
                        });
                    }

                    if sum.re < -tolerance {
                        return Err(ProcessMatrixError::NotPositiveSemidefinite {
                            index: row,
                            value: sum.re,
                        });
                    }

                    if sum.re <= tolerance {
                        lower[row * dimension + column] =
                            Complex64::ZERO;
                    } else {
                        let root = sum.re.sqrt();

                        if !root.is_finite() {
                            return Err(ProcessMatrixError::NumericalFailure {
                                operation: "positive-semidefinite validation",
                            });
                        }

                        lower[row * dimension + column] =
                            Complex64::new(root, 0.0);
                    }
                } else {
                    let pivot =
                        lower[column * dimension + column].re;

                    if pivot.abs() <= tolerance {
                        if sum.norm() > tolerance {
                            return Err(
                                ProcessMatrixError::NotPositiveSemidefinite {
                                    index: row,
                                    value: sum.re,
                                },
                            );
                        }

                        lower[row * dimension + column] =
                            Complex64::ZERO;
                    } else {
                        let value = sum.scale(1.0 / pivot);

                        if !value.is_finite() {
                            return Err(ProcessMatrixError::NumericalFailure {
                                operation: "positive-semidefinite validation",
                            });
                        }

                        lower[row * dimension + column] = value;
                    }
                }
            }
        }

        Ok(())
    }

    // =========================================================================
    // Partial trace
    // =========================================================================

    /// Computes the partial trace over the output subsystem.
    ///
    /// Under the matrix-index convention:
    ///
    /// ```text
    /// flat = input * output_dimension + output
    /// ```
    ///
    /// the result is an `input_dimension × input_dimension` matrix:
    ///
    /// ```text
    /// R[i,j] = Σ_a J[(i,a),(j,a)]
    /// ```
    ///
    /// This is the operation used to test trace preservation:
    ///
    /// ```text
    /// R == I
    /// ```
    ///
    /// The returned matrix uses `output_dimension == 1` because its own
    /// process-matrix representation requires two subsystem dimensions.
    ///
    /// For a raw operator-valued partial trace, use
    /// `partial_trace_output_elements`.
    pub fn partial_trace_output_elements(
        &self,
    ) -> Result<Vec<Complex64>, ProcessMatrixError> {
        let input = self.input_dimension;
        let output = self.output_dimension;

        let count = checked_square(input)?;

        let mut result = Vec::new();

        result
            .try_reserve_exact(count)
            .map_err(|_| ProcessMatrixError::AllocationFailure {
                elements: count,
                element_size: std::mem::size_of::<Complex64>(),
            })?;

        result.resize(count, Complex64::ZERO);

        let matrix_dimension = self.matrix_dimension();

        for i in 0..input {
            for j in 0..input {
                let mut sum = Complex64::ZERO;

                for a in 0..output {
                    let row = i * output + a;
                    let column = j * output + a;

                    let value =
                        self.elements[row * matrix_dimension + column];

                    sum = sum.add(value);

                    if !sum.is_finite() {
                        return Err(ProcessMatrixError::NumericalFailure {
                            operation: "output partial trace",
                        });
                    }
                }

                result[i * input + j] = sum;
            }
        }

        Ok(result)
    }

    /// Validates the trace-preserving condition.
    ///
    /// For the unnormalized Choi convention used by this module:
    ///
    /// ```text
    /// Tr_output(J) = I_input
    /// ```
    pub fn is_trace_preserving(
        &self,
        tolerance: f64,
    ) -> Result<bool, ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        let partial_trace = self.partial_trace_output_elements()?;
        let input = self.input_dimension;

        for row in 0..input {
            for column in 0..input {
                let actual = partial_trace[row * input + column];

                let expected = if row == column {
                    Complex64::ONE
                } else {
                    Complex64::ZERO
                };

                if actual.sub(expected).norm() > tolerance {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Validates the trace-preserving condition.
    pub fn validate_trace_preserving(
        &self,
        tolerance: f64,
    ) -> Result<(), ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        let partial_trace = self.partial_trace_output_elements()?;
        let input = self.input_dimension;

        for row in 0..input {
            for column in 0..input {
                let actual = partial_trace[row * input + column];

                let expected = if row == column {
                    Complex64::ONE
                } else {
                    Complex64::ZERO
                };

                if actual.sub(expected).norm() > tolerance {
                    return Err(ProcessMatrixError::NotTracePreserving {
                        row,
                        column,
                        actual,
                        expected,
                    });
                }
            }
        }

        Ok(())
    }

    // =========================================================================
    // Complete channel validation
    // =========================================================================

    /// Validates the mathematical properties required for a finite-dimensional
    /// completely-positive trace-preserving channel.
    ///
    /// The checks are:
    ///
    /// 1. all values finite;
    /// 2. Hermitian;
    /// 3. positive semidefinite;
    /// 4. trace preserving.
    pub fn validate_channel(
        &self,
        tolerance: f64,
    ) -> Result<(), ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        self.validate_finite()?;
        self.validate_hermitian(tolerance)?;
        self.validate_positive_semidefinite(tolerance)?;
        self.validate_trace_preserving(tolerance)?;

        Ok(())
    }

    /// Returns whether the process matrix satisfies the CPTP checks.
    pub fn is_valid_channel(
        &self,
        tolerance: f64,
    ) -> Result<bool, ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        Ok(self.validate_channel(tolerance).is_ok())
    }

    // =========================================================================
    // Normalization helpers
    // =========================================================================

    /// Validates that this matrix is a normalized process matrix and is
    /// positive semidefinite.
    pub fn validate_normalized_process(
        &self,
        tolerance: f64,
    ) -> Result<(), ProcessMatrixError> {
        validate_tolerance(tolerance)?;

        if !self.is_normalized(tolerance)? {
            let actual = self.trace()?;

            return Err(ProcessMatrixError::TraceMismatch {
                actual,
                expected: Complex64::ONE,
            });
        }

        self.validate_hermitian(tolerance)?;
        self.validate_positive_semidefinite(tolerance)?;

        Ok(())
    }

    // =========================================================================
    // Matrix transformations
    // =========================================================================

    /// Returns the conjugate transpose of this process matrix.
    ///
    /// This operation does not change input/output subsystem dimensions.
    pub fn adjoint(&self) -> Result<Self, ProcessMatrixError> {
        let dimension = self.matrix_dimension();

        let mut elements = Vec::new();

        elements
            .try_reserve_exact(self.elements.len())
            .map_err(|_| ProcessMatrixError::AllocationFailure {
                elements: self.elements.len(),
                element_size: std::mem::size_of::<Complex64>(),
            })?;

        elements.resize(self.elements.len(), Complex64::ZERO);

        for row in 0..dimension {
            for column in 0..dimension {
                elements[column * dimension + row] =
                    self.elements[row * dimension + column].conjugate();
            }
        }

        Ok(Self {
            input_dimension: self.input_dimension,
            output_dimension: self.output_dimension,
            elements,
        })
    }

    /// Returns a scaled copy.
    ///
    /// This is a representation operation. It does not claim that the result
    /// remains a valid quantum channel.
    pub fn scaled(&self, scalar: f64) -> Result<Self, ProcessMatrixError> {
        if !scalar.is_finite() {
            return Err(ProcessMatrixError::NumericalFailure {
                operation: "process-matrix scaling",
            });
        }

        let mut elements = Vec::new();

        elements
            .try_reserve_exact(self.elements.len())
            .map_err(|_| ProcessMatrixError::AllocationFailure {
                elements: self.elements.len(),
                element_size: std::mem::size_of::<Complex64>(),
            })?;

        for element in &self.elements {
            let value = element.scale(scalar);

            if !value.is_finite() {
                return Err(ProcessMatrixError::NumericalFailure {
                    operation: "process-matrix scaling",
                });
            }

            elements.push(value);
        }

        Ok(Self {
            input_dimension: self.input_dimension,
            output_dimension: self.output_dimension,
            elements,
        })
    }

    // =========================================================================
    // Internal indexing
    // =========================================================================

    fn flat_index(
        &self,
        row: usize,
        column: usize,
    ) -> Result<usize, ProcessMatrixError> {
        let dimension = self.matrix_dimension();

        if row >= dimension || column >= dimension {
            return Err(ProcessMatrixError::IndexOutOfBounds {
                row,
                column,
                dimension,
            });
        }

        Ok(row * dimension + column)
    }
}

// =============================================================================
// Dimension helpers
// =============================================================================

fn checked_matrix_dimension(
    input_dimension: usize,
    output_dimension: usize,
) -> Result<usize, ProcessMatrixError> {
    if input_dimension == 0 {
        return Err(ProcessMatrixError::ZeroInputDimension);
    }

    if output_dimension == 0 {
        return Err(ProcessMatrixError::ZeroOutputDimension);
    }

    input_dimension
        .checked_mul(output_dimension)
        .ok_or(ProcessMatrixError::DimensionOverflow {
            input_dimension,
            output_dimension,
        })
}

fn checked_square(
    dimension: usize,
) -> Result<usize, ProcessMatrixError> {
    dimension
        .checked_mul(dimension)
        .ok_or(ProcessMatrixError::MatrixSizeOverflow { dimension })
}

fn validate_elements(
    elements: &[Complex64],
) -> Result<(), ProcessMatrixError> {
    for (index, value) in elements.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(ProcessMatrixError::NonFiniteElement {
                index,
                value,
            });
        }
    }

    Ok(())
}

fn validate_tolerance(
    tolerance: f64,
) -> Result<(), ProcessMatrixError> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(ProcessMatrixError::InvalidTolerance { tolerance });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1.0e-12;

    #[test]
    fn complex_zero_and_one_are_correct() {
        assert_eq!(Complex64::ZERO, Complex64::new(0.0, 0.0));
        assert_eq!(Complex64::ONE, Complex64::new(1.0, 0.0));
    }

    #[test]
    fn complex_conjugation_is_correct() {
        let value = Complex64::new(2.0, -3.0);

        assert_eq!(
            value.conjugate(),
            Complex64::new(2.0, 3.0)
        );
    }

    #[test]
    fn complex_norm_is_correct() {
        let value = Complex64::new(3.0, 4.0);

        assert_eq!(value.norm_sqr(), 25.0);
        assert_eq!(value.norm(), 5.0);
    }

    #[test]
    fn complex_finiteness_is_checked() {
        assert!(Complex64::new(1.0, 2.0).is_finite());
        assert!(!Complex64::new(f64::NAN, 0.0).is_finite());
        assert!(!Complex64::new(0.0, f64::INFINITY).is_finite());
    }

    #[test]
    fn zero_matrix_has_expected_dimensions() {
        let matrix =
            ProcessMatrix::zero(2, 3).expect("matrix should construct");

        assert_eq!(matrix.input_dimension(), 2);
        assert_eq!(matrix.output_dimension(), 3);
        assert_eq!(matrix.matrix_dimension(), 6);
        assert_eq!(matrix.element_count(), 36);
    }

    #[test]
    fn zero_dimension_is_rejected() {
        assert_eq!(
            ProcessMatrix::zero(0, 2),
            Err(ProcessMatrixError::ZeroInputDimension)
        );

        assert_eq!(
            ProcessMatrix::zero(2, 0),
            Err(ProcessMatrixError::ZeroOutputDimension)
        );
    }

    #[test]
    fn wrong_element_count_is_rejected() {
        let result = ProcessMatrix::from_elements(
            2,
            2,
            vec![Complex64::ZERO; 3],
        );

        assert!(matches!(
            result,
            Err(ProcessMatrixError::InvalidElementCount {
                expected: 16,
                actual: 3
            })
        ));
    }

    #[test]
    fn non_finite_element_is_rejected() {
        let mut elements = vec![Complex64::ZERO; 4];

        elements[2] = Complex64::new(f64::NAN, 0.0);

        let result = ProcessMatrix::from_elements(1, 2, elements);

        assert!(matches!(
            result,
            Err(ProcessMatrixError::NonFiniteElement {
                index: 2,
                ..
            })
        ));
    }

    #[test]
    fn identity_channel_one_dimensional_is_valid() {
        let matrix =
            ProcessMatrix::identity_channel(1)
                .expect("identity process should construct");

        assert_eq!(matrix.get(0, 0).unwrap(), Complex64::ONE);

        assert!(matrix.is_hermitian(TOLERANCE).unwrap());
        assert!(
            matrix
                .is_positive_semidefinite(TOLERANCE)
                .unwrap()
        );
        assert!(matrix.is_trace_preserving(TOLERANCE).unwrap());
        assert!(matrix.has_channel_trace(TOLERANCE).unwrap());
        assert!(matrix.is_valid_channel(TOLERANCE).unwrap());
    }

    #[test]
    fn identity_channel_two_dimensional_has_correct_structure() {
        let matrix =
            ProcessMatrix::identity_channel(2)
                .expect("identity channel should construct");

        assert_eq!(matrix.matrix_dimension(), 4);

        // |Ω> = |00> + |11>
        //
        // Therefore |Ω><Ω| has non-zero entries at:
        //
        // (0,0), (0,3), (3,0), (3,3)
        assert_eq!(matrix.get(0, 0).unwrap(), Complex64::ONE);
        assert_eq!(matrix.get(0, 3).unwrap(), Complex64::ONE);
        assert_eq!(matrix.get(3, 0).unwrap(), Complex64::ONE);
        assert_eq!(matrix.get(3, 3).unwrap(), Complex64::ONE);

        assert_eq!(
            matrix.get(1, 1).unwrap(),
            Complex64::ZERO
        );

        assert!(matrix.is_hermitian(TOLERANCE).unwrap());
        assert!(
            matrix
                .is_positive_semidefinite(TOLERANCE)
                .unwrap()
        );
        assert!(matrix.is_trace_preserving(TOLERANCE).unwrap());
    }

    #[test]
    fn identity_channel_trace_equals_input_dimension() {
        for dimension in 1..=4 {
            let matrix =
                ProcessMatrix::identity_channel(dimension)
                    .expect("identity channel should construct");

            let trace = matrix.trace().unwrap();

            assert!((trace.re - dimension as f64).abs() <= TOLERANCE);
            assert!(trace.im.abs() <= TOLERANCE);
        }
    }

    #[test]
    fn normalized_identity_has_unit_trace() {
        let matrix =
            ProcessMatrix::normalized_identity_channel(2)
                .expect("normalized identity should construct");

        let trace = matrix.trace().unwrap();

        assert!((trace.re - 1.0).abs() <= TOLERANCE);
        assert!(trace.im.abs() <= TOLERANCE);
        assert!(matrix.is_normalized(TOLERANCE).unwrap());

        matrix
            .validate_normalized_process(TOLERANCE)
            .expect("normalized identity should be a valid process state");
    }

    #[test]
    fn trace_preserving_partial_trace_is_identity() {
        let matrix =
            ProcessMatrix::identity_channel(3)
                .expect("identity channel should construct");

        let partial_trace =
            matrix.partial_trace_output_elements().unwrap();

        assert_eq!(partial_trace.len(), 9);

        for row in 0..3 {
            for column in 0..3 {
                let expected = if row == column {
                    Complex64::ONE
                } else {
                    Complex64::ZERO
                };

                assert!(
                    partial_trace[row * 3 + column]
                        .sub(expected)
                        .norm()
                        <= TOLERANCE
                );
            }
        }
    }

    #[test]
    fn non_trace_preserving_matrix_is_rejected() {
        let mut matrix =
            ProcessMatrix::identity_channel(2)
                .expect("identity channel should construct");

        matrix
            .set(0, 0, Complex64::new(0.5, 0.0))
            .expect("valid element");

        assert!(
            !matrix
                .is_trace_preserving(TOLERANCE)
                .unwrap()
        );

        assert!(
            matrix
                .validate_trace_preserving(TOLERANCE)
                .is_err()
        );
    }

    #[test]
    fn hermiticity_detects_non_hermitian_matrix() {
        let mut matrix =
            ProcessMatrix::zero(1, 2)
                .expect("matrix should construct");

        matrix
            .set(0, 1, Complex64::new(1.0, 2.0))
            .expect("valid element");

        assert!(
            !matrix
                .is_hermitian(TOLERANCE)
                .unwrap()
        );

        assert!(
            matrix
                .validate_hermitian(TOLERANCE)
                .is_err()
        );
    }

    #[test]
    fn positive_semidefinite_validation_accepts_zero_matrix() {
        let matrix =
            ProcessMatrix::zero(1, 2)
                .expect("matrix should construct");

        assert!(
            matrix
                .is_positive_semidefinite(TOLERANCE)
                .unwrap()
        );
    }

    #[test]
    fn positive_semidefinite_validation_rejects_negative_matrix() {
        let mut matrix =
            ProcessMatrix::zero(1, 1)
                .expect("matrix should construct");

        matrix
            .set(0, 0, Complex64::new(-1.0, 0.0))
            .expect("valid element");

        assert!(
            !matrix
                .is_positive_semidefinite(TOLERANCE)
                .unwrap()
        );

        assert!(
            matrix
                .validate_positive_semidefinite(TOLERANCE)
                .is_err()
        );
    }

    #[test]
    fn adjoint_of_hermitian_identity_is_itself() {
        let matrix =
            ProcessMatrix::identity_channel(2)
                .expect("identity channel should construct");

        let adjoint =
            matrix.adjoint().expect("adjoint should construct");

        assert_eq!(matrix, adjoint);
    }

    #[test]
    fn scaling_is_explicit() {
        let matrix =
            ProcessMatrix::identity_channel(2)
                .expect("identity channel should construct");

        let scaled =
            matrix.scaled(0.5)
                .expect("scaling should construct");

        let trace = scaled.trace().unwrap();

        assert!((trace.re - 1.0).abs() <= TOLERANCE);
    }

    #[test]
    fn element_access_is_bounds_checked() {
        let matrix =
            ProcessMatrix::zero(1, 2)
                .expect("matrix should construct");

        assert!(matrix.get(2, 0).is_err());
        assert!(matrix.get(0, 2).is_err());
        assert!(matrix.get_flat(matrix.element_count()).is_err());
    }

    #[test]
    fn mutation_rejects_non_finite_values() {
        let mut matrix =
            ProcessMatrix::zero(1, 1)
                .expect("matrix should construct");

        assert!(
            matrix
                .set(0, 0, Complex64::new(f64::INFINITY, 0.0))
                .is_err()
        );

        assert!(
            matrix
                .set_flat(0, Complex64::new(0.0, f64::NAN))
                .is_err()
        );
    }

    #[test]
    fn tolerance_validation_rejects_invalid_values() {
        let matrix =
            ProcessMatrix::identity_channel(1)
                .expect("identity channel should construct");

        assert!(
            matches!(
                matrix.is_hermitian(f64::NAN),
                Err(ProcessMatrixError::InvalidTolerance { .. })
            )
        );

        assert!(
            matches!(
                matrix.is_hermitian(-1.0),
                Err(ProcessMatrixError::InvalidTolerance { .. })
            )
        );
    }

    #[test]
    fn arbitrary_finite_dimension_is_supported() {
        let matrix =
            ProcessMatrix::identity_channel(5)
                .expect("dimension five must not be hard-coded away");

        assert_eq!(matrix.input_dimension(), 5);
        assert_eq!(matrix.output_dimension(), 5);
        assert_eq!(matrix.matrix_dimension(), 25);

        assert!(matrix.is_valid_channel(TOLERANCE).unwrap());
    }

    #[test]
    fn non_square_input_output_dimensions_are_supported() {
        // This represents a finite-dimensional process from a 2-dimensional
        // input space to a 3-dimensional output space. It is not automatically
        // claimed to be trace preserving.
        let matrix =
            ProcessMatrix::zero(2, 3)
                .expect("rectangular input/output process must construct");

        assert_eq!(matrix.matrix_dimension(), 6);
        assert_eq!(matrix.element_count(), 36);
    }

    #[test]
    fn normalized_process_preserves_dimensions() {
        let matrix =
            ProcessMatrix::identity_channel(3)
                .expect("identity channel should construct");

        let normalized =
            matrix.normalized()
                .expect("normalization should construct");

        assert_eq!(
            normalized.input_dimension(),
            matrix.input_dimension()
        );

        assert_eq!(
            normalized.output_dimension(),
            matrix.output_dimension()
        );

        assert!(normalized.is_normalized(TOLERANCE).unwrap());
    }

    #[test]
    fn finite_validation_succeeds_for_valid_matrix() {
        let matrix =
            ProcessMatrix::identity_channel(2)
                .expect("identity channel should construct");

        matrix
            .validate_finite()
            .expect("identity matrix contains only finite values");
    }

    #[test]
    fn flat_and_two_dimensional_access_agree() {
        let mut matrix =
            ProcessMatrix::zero(2, 2)
                .expect("matrix should construct");

        let value = Complex64::new(1.5, -0.25);

        matrix
            .set(1, 2, value)
            .expect("valid matrix location");

        let flat = 1 * matrix.matrix_dimension() + 2;

        assert_eq!(matrix.get(1, 2).unwrap(), value);
        assert_eq!(matrix.get_flat(flat).unwrap(), value);
    }

    #[test]
    fn channel_validation_is_composed_from_individual_invariants() {
        let matrix =
            ProcessMatrix::identity_channel(3)
                .expect("identity channel should construct");

        matrix
            .validate_finite()
            .expect("finite");

        matrix
            .validate_hermitian(TOLERANCE)
            .expect("Hermitian");

        matrix
            .validate_positive_semidefinite(TOLERANCE)
            .expect("PSD");

        matrix
            .validate_trace_preserving(TOLERANCE)
            .expect("TP");

        matrix
            .validate_channel(TOLERANCE)
            .expect("CPTP");
    }
}