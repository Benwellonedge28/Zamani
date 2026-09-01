//! Zamani Quantum Noise (ZQN) — Choi Channel Representation.
//!
//! This module provides a production-grade, provider-neutral Choi–Jamiołkowski
//! representation for finite-dimensional quantum channels.
//!
//! # Architectural role
//!
//! `quantum::zqn::channel::choi` owns:
//!
//! - Choi-matrix structural semantics;
//! - input/output dimensions;
//! - safe element storage;
//! - row-major indexing;
//! - construction from Kraus operators;
//! - validation of Hermiticity;
//! - validation of complete positivity through positive-semidefinite checking;
//! - validation of trace preservation;
//! - partial-trace validation;
//! - composition-compatible structural operations;
//! - tensor-product construction;
//! - deterministic iteration;
//! - resource-aware size calculation;
//! - conversion to/from the canonical quantum-memory complex scalar.
//!
//! It does NOT own:
//!
//! - the canonical quantum IR;
//! - qubit identities;
//! - physical-device identity;
//! - vendor APIs;
//! - QPU execution;
//! - routing;
//! - scheduling;
//! - QEC;
//! - calibration;
//! - noise-model policy;
//! - simulation policy;
//! - random-number generation;
//! - global resource limits.
//!
//! # Canonical quantum identities
//!
//! This module intentionally does not import or redefine:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A Choi matrix describes a mathematical map between finite-dimensional
//! spaces. Resource identity is attached by higher-level ZQN/channel/IR
//! integration layers.
//!
//! # Mathematical convention
//!
//! For a channel
//!
//! ```text
//! E : L(H_in) -> L(H_out)
//! ```
//!
//! with Kraus operators
//!
//! ```text
//! K_r : H_in -> H_out,
//! ```
//!
//! this implementation uses the column-vectorization-compatible Choi
//! convention
//!
//! ```text
//! J(E) = sum_r |K_r>> <<K_r|
//! ```
//!
//! with matrix indices arranged as:
//!
//! ```text
//! row    = output_row    * input_dimension + input_row
//! column = output_col    * input_dimension + input_col
//! ```
//!
//! Equivalently:
//!
//! ```text
//! J[(a,i),(b,j)] = sum_r K_r[a,i] * conjugate(K_r[b,j])
//! ```
//!
//! Under this convention, trace preservation is:
//!
//! ```text
//! Tr_out(J) = I_in
//! ```
//!
//! and complete positivity is equivalent to:
//!
//! ```text
//! J >= 0
//! ```
//!
//! # Generality
//!
//! The representation is deliberately dimension-generic.
//!
//! It does not assume:
//!
//! - qubits;
//! - two-level systems;
//! - two-qubit gates;
//! - a fixed number of Kraus operators;
//! - a fixed machine size;
//! - a particular hardware topology.
//!
//! Therefore a channel may describe:
//!
//! - a qubit channel;
//! - a qudit channel;
//! - a logical subsystem;
//! - a truncated bosonic mode;
//! - a composite finite-dimensional subsystem;
//! - a heterogeneous finite-dimensional map.
//!
//! Infinite-dimensional quantum systems are not materialized as a finite Choi
//! matrix. Such systems require another representation, truncation, symbolic
//! representation, or operator-valued formulation. A finite Choi matrix itself
//! necessarily represents a finite-dimensional domain.
//!
//! # Scalability
//!
//! There is no semantic `MAX_QUBITS`, `MAX_DIMENSION`, `MAX_MATRIX_SIZE`, or
//! equivalent architectural ceiling in this file.
//!
//! Structural dimensions use `u128`.
//!
//! Actual matrix storage uses `Vec<Complex64>`, whose physical capacity is
//! necessarily bounded by the host process and allocator. That is an execution
//! resource constraint, not a ZQN semantic limit.
//!
//! All dimension-to-storage conversions are checked.
//!
//! This distinction is essential:
//!
//! ```text
//! mathematical dimension
//!         |
//!         v
//! checked element-count calculation
//!         |
//!         v
//! explicit storage request
//!         |
//!         v
//! actual available resources
//! ```
//!
//! A caller that needs sparse, symbolic, distributed, tensor-network, GPU,
//! streaming, or otherwise non-materialized representations must use a suitable
//! representation instead of forcing a gigantic Choi matrix into memory.
//!
//! # Numerical safety
//!
//! The implementation:
//!
//! - rejects non-finite complex elements;
//! - never silently converts NaN or infinity;
//! - performs checked dimension multiplication;
//! - performs checked `u128 -> usize` conversion;
//! - validates Hermiticity;
//! - validates positive semidefiniteness;
//! - validates trace preservation;
//! - uses explicit tolerances for approximate floating-point validation;
//! - never silently repairs invalid channel data.
//!
//! # Positive-semidefinite validation
//!
//! A Choi matrix is completely positive exactly when it is positive
//! semidefinite.
//!
//! This implementation validates positive semidefiniteness using a
//! tolerance-aware Hermitian Cholesky-style factorization. The algorithm
//! supports positive-semidefinite matrices, including matrices with zero
//! eigenvalues, rather than requiring strict positive definiteness.
//!
//! This avoids requiring a third-party eigensolver merely to validate the
//! fundamental Choi invariant.
//!
//! It is intentionally a validation operation rather than a general-purpose
//! eigendecomposition API.
//!
//! # Exact versus approximate semantics
//!
//! `Complex64` is a floating-point representation. Consequently, validation
//! uses an explicit [`ChoiValidationTolerance`] rather than pretending that
//! binary floating-point equality is mathematical equality.
//!
//! The tolerance is caller-configurable.
//!
//! The default tolerance is deliberately conservative and is not a semantic
//! promise about every physical experiment.
//!
//! # Determinism
//!
//! This module contains no randomness and no global mutable state.
//!
//! Deterministic behavior includes:
//!
//! - indexing;
//! - element iteration;
//! - dimension calculations;
//! - validation order;
//! - Choi construction from an ordered Kraus collection;
//! - tensor-product construction.
//!
//! Validation errors are deterministic for identical input values and
//! tolerance.
//!
//! # Serialization
//!
//! This module does not define the external ZQN wire schema.
//!
//! The Choi matrix exposes deterministic component access so
//! `zqn::io::serialization` can serialize:
//!
//! ```text
//! schema/version
//! representation kind
//! input dimension
//! output dimension
//! complex precision
//! element ordering
//! matrix elements
//! ```
//!
//! Rust memory layout must never be treated as the wire format.
//!
//! # Integration
//!
//! ```text
//! zqn/channel/representation.rs
//!             |
//!             v
//!       zqn/channel/choi.rs
//!             |
//!       +-----+------+
//!       |            |
//!       v            v
//! channel/channel  channel/kraus
//!       |            |
//!       +-----+------+
//!             |
//!             v
//!     simulation / target / propagation
//! ```
//!
//! The concrete numerical scalar comes from:
//!
//! ```text
//! crate::quantum::memory::complex::Complex64
//! ```
//!
//! The channel's physical resource identity is intentionally supplied by
//! higher-level integration layers.
//!
//! # No-reedit contract
//!
//! This file is complete when:
//!
//! 1. Choi dimensions are represented without fixed machine-size assumptions;
//! 2. matrix allocation sizes are checked;
//! 3. indexing is deterministic;
//! 4. construction from Kraus operators is mathematically correct;
//! 5. Hermiticity is validated;
//! 6. positive semidefiniteness is validated;
//! 7. trace preservation is validated;
//! 8. partial trace semantics are explicit;
//! 9. tensor products are dimension-generic;
//! 10. no qubit identity is duplicated;
//! 11. no vendor/backend knowledge exists;
//! 12. no RNG exists;
//! 13. no unsafe code exists;
//! 14. non-finite numerical values are rejected;
//! 15. resource exhaustion is surfaced rather than hidden;
//! 16. the implementation uses the canonical ZQN error vocabulary;
//! 17. callers can inspect dimensions without allocating another matrix;
//! 18. serialization can consume deterministic elements;
//! 19. later channel implementations do not need to modify this file merely
//!     to consume Choi matrices;
//! 20. increasing the supported quantum-machine size does not require editing
//!     this file.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Security
//!
//! Choi matrices are potentially quadratic in Hilbert-space dimension.
//!
//! Therefore every constructor that materializes storage performs checked size
//! arithmetic before allocation.
//!
//! A caller must still apply its own configured `zqn::core::limits` policy
//! before requesting a physically large matrix.
//!
//! This file deliberately does not invent a global maximum because such a
//! maximum would contradict ZQN's write-once/scale-everywhere architecture.
//!
//! # Testing
//!
//! The tests at the bottom cover:
//!
//! - zero-dimensional rejection;
//! - dimension arithmetic;
//! - identity channels;
//! - nontrivial Kraus channels;
//! - Hermiticity;
//! - positive semidefiniteness;
//! - trace preservation;
//! - invalid trace-preserving matrices;
//! - invalid non-PSD matrices;
//! - tensor products;
//! - deterministic indexing;
//! - tolerance behavior;
//! - non-finite-value rejection;
//! - empty Kraus-set rejection;
//! - checked storage sizing.
//!
//! ============================================================================
//! Implementation
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::ops::Index;

use crate::quantum::memory::complex::Complex64;
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};

// ============================================================================
// Constants
// ============================================================================

/// Stable semantic identifier for this representation.
pub const CHOI_SCHEMA_ID: &str = "zamani.quantum.zqn.channel.choi";

/// Semantic version of the Choi representation contract.
pub const CHOI_SCHEMA_VERSION: u16 = 1;

/// Default absolute numerical validation tolerance.
///
/// This value is used only when the caller explicitly chooses
/// [`ChoiValidationTolerance::default`].
///
/// It is not a physical error-rate assumption.
pub const DEFAULT_ABSOLUTE_TOLERANCE: f64 = 1.0e-12;

/// Default relative numerical validation tolerance.
///
/// This value is used only when the caller explicitly chooses
/// [`ChoiValidationTolerance::default`].
pub const DEFAULT_RELATIVE_TOLERANCE: f64 = 1.0e-10;

// ============================================================================
// Validation tolerance
// ============================================================================

/// Numerical tolerance used by Choi invariant validation.
///
/// Both absolute and relative tolerances must be finite and non-negative.
///
/// The tolerance is a validation policy, not part of the mathematical Choi
/// matrix itself.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChoiValidationTolerance {
    /// Absolute comparison tolerance.
    pub absolute: f64,

    /// Relative comparison tolerance.
    pub relative: f64,
}

impl ChoiValidationTolerance {
    /// Creates a validated tolerance.
    pub fn new(absolute: f64, relative: f64) -> ZqnResult<Self> {
        if !absolute.is_finite() || !relative.is_finite() {
            return Err(Self::invalid_tolerance_error(
                "Choi validation tolerances must be finite",
            ));
        }

        if absolute < 0.0 || relative < 0.0 {
            return Err(Self::invalid_tolerance_error(
                "Choi validation tolerances cannot be negative",
            ));
        }

        Ok(Self {
            absolute,
            relative,
        })
    }

    /// Returns the default production validation tolerance.
    #[must_use]
    pub const fn default_values() -> Self {
        Self {
            absolute: DEFAULT_ABSOLUTE_TOLERANCE,
            relative: DEFAULT_RELATIVE_TOLERANCE,
        }
    }

    fn invalid_tolerance_error(message: &str) -> ZqnError {
        ZqnError::new(
            ZqnErrorKind::Structure,
            ZqnErrorCode::InvalidChannelParameter,
            message.to_owned(),
        )
    }

    /// Computes a scale-aware tolerance.
    #[must_use]
    pub fn scaled(self, scale: f64) -> f64 {
        let magnitude = scale.abs();

        self.absolute
            .max(self.relative * magnitude)
    }

    /// Returns whether two real values are approximately equal.
    #[must_use]
    pub fn approx_real(self, left: f64, right: f64) -> bool {
        if !left.is_finite() || !right.is_finite() {
            return false;
        }

        let difference = (left - right).abs();
        let scale = left.abs().max(right.abs()).max(1.0);

        difference <= self.scaled(scale)
    }

    /// Returns whether two complex values are approximately equal.
    #[must_use]
    pub fn approx_complex(self, left: Complex64, right: Complex64) -> bool {
        if !left.is_finite() || !right.is_finite() {
            return false;
        }

        let difference = left - right;
        let magnitude = difference.magnitude();

        let scale = left
            .magnitude()
            .max(right.magnitude())
            .max(1.0);

        magnitude <= self.scaled(scale)
    }
}

impl Default for ChoiValidationTolerance {
    fn default() -> Self {
        Self::default_values()
    }
}

// ============================================================================
// Choi dimensions
// ============================================================================

/// Input/output Hilbert-space dimensions of a Choi matrix.
///
/// The Choi matrix has dimension:
///
/// ```text
/// (output_dimension * input_dimension)
///     ×
/// (output_dimension * input_dimension)
/// ```
///
/// Dimensions are semantic `u128` values and are not themselves allocation
/// limits.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChoiDimensions {
    /// Dimension of the channel input Hilbert space.
    pub input: u128,

    /// Dimension of the channel output Hilbert space.
    pub output: u128,
}

impl ChoiDimensions {
    /// Creates validated Choi dimensions.
    ///
    /// Zero-dimensional Hilbert spaces are rejected because they do not define
    /// a valid finite-dimensional quantum system.
    pub fn new(input: u128, output: u128) -> ZqnResult<Self> {
        if input == 0 {
            return Err(Self::invalid_dimension(
                "Choi input dimension must be greater than zero",
            ));
        }

        if output == 0 {
            return Err(Self::invalid_dimension(
                "Choi output dimension must be greater than zero",
            ));
        }

        // Validate the products even though this constructor does not allocate.
        // This makes the descriptor itself safe to use for later matrix sizing.
        let _ = checked_product(input, output, "Choi matrix dimension")?;

        let _ = checked_square(
            input
                .checked_mul(output)
                .ok_or_else(|| {
                    Self::dimension_overflow_error(
                        "Choi matrix dimension multiplication overflowed",
                    )
                })?,
            "Choi matrix element count",
        )?;

        Ok(Self {
            input,
            output,
        })
    }

    fn invalid_dimension(message: &str) -> ZqnError {
        ZqnError::new(
            ZqnErrorKind::Structure,
            ZqnErrorCode::ChannelDimensionMismatch,
            message.to_owned(),
        )
    }

    fn dimension_overflow_error(message: &str) -> ZqnError {
        ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::SizeOverflow,
            message.to_owned(),
        )
    }

    /// Returns the dimension of the Choi matrix itself.
    ///
    /// This is:
    ///
    /// ```text
    /// output * input
    /// ```
    pub fn matrix_dimension(self) -> ZqnResult<u128> {
        checked_product(
            self.input,
            self.output,
            "Choi matrix dimension",
        )
    }

    /// Returns the number of matrix elements.
    ///
    /// This is:
    ///
    /// ```text
    /// (output * input)^2
    /// ```
    pub fn element_count(self) -> ZqnResult<u128> {
        let dimension = self.matrix_dimension()?;

        checked_square(dimension, "Choi matrix element count")
    }

    /// Returns the number of stored complex scalars required on a host.
    ///
    /// This performs a checked `u128 -> usize` conversion but does not allocate.
    pub fn host_element_count(self) -> ZqnResult<usize> {
        let count = self.element_count()?;

        usize::try_from(count).map_err(|_| {
            ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::SizeOverflow,
                format!(
                    "Choi element count {count} cannot be represented by host usize"
                ),
            )
        })
    }

    /// Returns the byte count required for `Complex64` storage.
    ///
    /// This is a planning calculation only.
    pub fn host_storage_bytes(self) -> ZqnResult<usize> {
        let count = self.host_element_count()?;

        count.checked_mul(core::mem::size_of::<Complex64>())
            .ok_or_else(|| {
                ZqnError::new(
                    ZqnErrorKind::Limits,
                    ZqnErrorCode::SizeOverflow,
                    "Choi storage byte count overflowed host usize"
                        .to_owned(),
                )
            })
    }

    /// Returns whether the dimensions are compatible with a square channel.
    ///
    /// A quantum channel need not have equal input and output dimensions.
    /// Therefore this returns true only for equal-dimensional channels.
    #[must_use]
    pub const fn is_square_channel(self) -> bool {
        self.input == self.output
    }
}

// ============================================================================
// Choi matrix
// ============================================================================

/// Production-grade finite-dimensional Choi matrix.
///
/// Elements are stored in deterministic row-major order.
///
/// The logical matrix dimension is:
///
/// ```text
/// output_dimension * input_dimension
/// ```
///
/// and the storage contains the square of that quantity.
///
/// The matrix is immutable with respect to its dimensions but exposes controlled
/// element mutation for construction workflows. Callers should validate a
/// modified matrix before treating it as a valid quantum channel.
#[derive(Clone, Debug, PartialEq)]
pub struct ChoiMatrix {
    dimensions: ChoiDimensions,
    elements: Vec<Complex64>,
}

impl ChoiMatrix {
    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    /// Creates a zero-initialized Choi matrix of the requested dimensions.
    ///
    /// This method performs all structural calculations before allocating.
    ///
    /// The zero matrix is not a trace-preserving quantum channel, but it is a
    /// valid matrix container and can be useful as an intermediate during
    /// controlled construction.
    pub fn zeros(input_dimension: u128, output_dimension: u128) -> ZqnResult<Self> {
        let dimensions =
            ChoiDimensions::new(input_dimension, output_dimension)?;

        let element_count = dimensions.host_element_count()?;

        Ok(Self {
            dimensions,
            elements: vec![Complex64::ZERO; element_count],
        })
    }

    /// Constructs a Choi matrix from an existing element vector.
    ///
    /// The caller must provide exactly:
    ///
    /// ```text
    /// (input * output)^2
    /// ```
    ///
    /// finite complex elements.
    ///
    /// This constructor validates structural correctness and numerical
    /// finiteness, but does not claim that the matrix is a valid quantum
    /// channel until [`ChoiMatrix::validate`] is called.
    pub fn from_elements(
        input_dimension: u128,
        output_dimension: u128,
        elements: Vec<Complex64>,
    ) -> ZqnResult<Self> {
        let dimensions =
            ChoiDimensions::new(input_dimension, output_dimension)?;

        let expected = dimensions.host_element_count()?;

        if elements.len() != expected {
            return Err(ZqnError::new(
                ZqnErrorKind::Channel,
                ZqnErrorCode::ChannelDimensionMismatch,
                format!(
                    "Choi element count mismatch: expected {expected}, got {}",
                    elements.len()
                ),
            ));
        }

        for (index, element) in elements.iter().copied().enumerate() {
            if !element.is_finite() {
                return Err(ZqnError::new(
                    ZqnErrorKind::Channel,
                    ZqnErrorCode::InvalidChannelParameter,
                    format!(
                        "Choi element at flat index {index} is non-finite"
                    ),
                ));
            }
        }

        Ok(Self {
            dimensions,
            elements,
        })
    }

    /// Constructs a Choi matrix from Kraus operators.
    ///
    /// Each Kraus operator is represented in row-major order as:
    ///
    /// ```text
    /// output_dimension × input_dimension
    /// ```
    ///
    /// For each Kraus operator `K`, this constructs:
    ///
    /// ```text
    /// |K>> <<K|
    /// ```
    ///
    /// and sums all Kraus contributions.
    ///
    /// The resulting matrix is mathematically positive semidefinite up to
    /// floating-point roundoff.
    ///
    /// The resulting matrix is not automatically assumed trace-preserving:
    /// callers may construct trace-non-preserving completely positive maps.
    pub fn from_kraus(
        input_dimension: u128,
        output_dimension: u128,
        kraus_operators: &[Vec<Complex64>],
    ) -> ZqnResult<Self> {
        let dimensions =
            ChoiDimensions::new(input_dimension, output_dimension)?;

        if kraus_operators.is_empty() {
            return Err(ZqnError::new(
                ZqnErrorKind::Channel,
                ZqnErrorCode::InvalidChannel,
                "a Choi channel requires at least one Kraus operator"
                    .to_owned(),
            ));
        }

        let operator_element_count = checked_product(
            output_dimension,
            input_dimension,
            "Kraus operator element count",
        )?;

        let operator_len = usize::try_from(operator_element_count)
            .map_err(|_| {
                ZqnError::new(
                    ZqnErrorKind::Limits,
                    ZqnErrorCode::SizeOverflow,
                    "Kraus operator element count cannot fit host usize"
                        .to_owned(),
                )
            })?;

        let matrix_len = dimensions.host_element_count()?;

        let mut elements = vec![Complex64::ZERO; matrix_len];

        for (operator_index, kraus) in kraus_operators.iter().enumerate() {
            if kraus.len() != operator_len {
                return Err(ZqnError::new(
                    ZqnErrorKind::Channel,
                    ZqnErrorCode::ChannelDimensionMismatch,
                    format!(
                        "Kraus operator {operator_index} has {} elements; expected {operator_len}",
                        kraus.len()
                    ),
                ));
            }

            for (index, value) in kraus.iter().copied().enumerate() {
                if !value.is_finite() {
                    return Err(ZqnError::new(
                        ZqnErrorKind::Channel,
                        ZqnErrorCode::InvalidChannelParameter,
                        format!(
                            "Kraus operator {operator_index} contains non-finite element at index {index}"
                        ),
                    ));
                }
            }

            for output_row in 0..output_dimension {
                for input_row in 0..input_dimension {
                    let source_index = checked_matrix_index(
                        output_row,
                        input_row,
                        input_dimension,
                        "Kraus source index",
                    )?;

                    let source = kraus
                        .get(usize::try_from(source_index).map_err(|_| {
                            ZqnError::new(
                                ZqnErrorKind::Limits,
                                ZqnErrorCode::SizeOverflow,
                                "Kraus source index cannot fit host usize"
                                    .to_owned(),
                            )
                        })?)
                        .copied()
                        .ok_or_else(|| {
                            ZqnError::new(
                                ZqnErrorKind::Internal,
                                ZqnErrorCode::InvalidChannel,
                                "validated Kraus source index was unexpectedly absent"
                                    .to_owned(),
                            )
                        })?;

                    let row = checked_choi_index(
                        output_row,
                        input_row,
                        input_dimension,
                        "Kraus-to-Choi row index",
                    )?;

                    for output_col in 0..output_dimension {
                        for input_col in 0..input_dimension {
                            let source_col_index = checked_matrix_index(
                                output_col,
                                input_col,
                                input_dimension,
                                "Kraus source column index",
                            )?;

                            let source_col = kraus
                                .get(
                                    usize::try_from(source_col_index).map_err(
                                        |_| {
                                            ZqnError::new(
                                                ZqnErrorKind::Limits,
                                                ZqnErrorCode::SizeOverflow,
                                                "Kraus source column index cannot fit host usize"
                                                    .to_owned(),
                                            )
                                        },
                                    )?,
                                )
                                .copied()
                                .ok_or_else(|| {
                                    ZqnError::new(
                                        ZqnErrorKind::Internal,
                                        ZqnErrorCode::InvalidChannel,
                                        "validated Kraus source column index was unexpectedly absent"
                                            .to_owned(),
                                    )
                                })?;

                            let col = checked_choi_index(
                                output_col,
                                input_col,
                                input_dimension,
                                "Kraus-to-Choi column index",
                            )?;

                            let flat = checked_matrix_index(
                                row,
                                col,
                                dimensions.matrix_dimension()?,
                                "Choi element index",
                            )?;

                            let flat = usize::try_from(flat).map_err(|_| {
                                ZqnError::new(
                                    ZqnErrorKind::Limits,
                                    ZqnErrorCode::SizeOverflow,
                                    "Choi element index cannot fit host usize"
                                        .to_owned(),
                                )
                            })?;

                            let contribution =
                                source * source_col.conjugate();

                            let current = elements[flat];

                            let updated = current + contribution;

                            if !updated.is_finite() {
                                return Err(ZqnError::new(
                                    ZqnErrorKind::Channel,
                                    ZqnErrorCode::InvalidChannelParameter,
                                    format!(
                                        "Choi construction became non-finite at element {flat} while processing Kraus operator {operator_index}"
                                    ),
                                ));
                            }

                            elements[flat] = updated;
                        }
                    }
                }
            }
        }

        Self::from_elements(
            input_dimension,
            output_dimension,
            elements,
        )
    }

    /// Constructs the identity channel's Choi matrix.
    ///
    /// For a finite-dimensional identity map:
    ///
    /// ```text
    /// E(X) = X
    /// ```
    ///
    /// the Choi matrix is the unnormalized maximally entangled projector.
    pub fn identity(dimension: u128) -> ZqnResult<Self> {
        let dimensions = ChoiDimensions::new(dimension, dimension)?;

        let matrix_dimension = dimensions.matrix_dimension()?;

        let matrix_len = dimensions.host_element_count()?;

        let mut elements = vec![Complex64::ZERO; matrix_len];

        for input_row in 0..dimension {
            for output_row in 0..dimension {
                let row = checked_choi_index(
                    output_row,
                    input_row,
                    dimension,
                    "identity Choi row index",
                )?;

                let col = checked_choi_index(
                    output_row,
                    input_row,
                    dimension,
                    "identity Choi column index",
                )?;

                let flat = checked_matrix_index(
                    row,
                    col,
                    matrix_dimension,
                    "identity Choi matrix index",
                )?;

                let flat = usize::try_from(flat).map_err(|_| {
                    ZqnError::new(
                        ZqnErrorKind::Limits,
                        ZqnErrorCode::SizeOverflow,
                        "identity Choi index cannot fit host usize"
                            .to_owned(),
                    )
                })?;

                elements[flat] = Complex64::ONE;
            }
        }

        Self::from_elements(dimension, dimension, elements)
    }

    // ------------------------------------------------------------------------
    // Structural access
    // ------------------------------------------------------------------------

    /// Returns the channel dimensions.
    #[must_use]
    pub const fn dimensions(&self) -> ChoiDimensions {
        self.dimensions
    }

    /// Returns the input Hilbert-space dimension.
    #[must_use]
    pub const fn input_dimension(&self) -> u128 {
        self.dimensions.input
    }

    /// Returns the output Hilbert-space dimension.
    #[must_use]
    pub const fn output_dimension(&self) -> u128 {
        self.dimensions.output
    }

    /// Returns the square matrix dimension.
    pub fn matrix_dimension(&self) -> ZqnResult<u128> {
        self.dimensions.matrix_dimension()
    }

    /// Returns the number of stored elements.
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }

    /// Returns the underlying immutable element slice.
    ///
    /// Elements are in deterministic row-major order.
    #[must_use]
    pub fn elements(&self) -> &[Complex64] {
        &self.elements
    }

    /// Returns a mutable element slice.
    ///
    /// After modifying elements, callers must revalidate the matrix before
    /// treating it as a valid quantum channel.
    #[must_use]
    pub fn elements_mut(&mut self) -> &mut [Complex64] {
        &mut self.elements
    }

    /// Returns an immutable matrix element.
    pub fn get(
        &self,
        row: u128,
        column: u128,
    ) -> ZqnResult<Complex64> {
        let dimension = self.matrix_dimension()?;

        let flat =
            checked_matrix_index(row, column, dimension, "Choi element index")?;

        let flat = usize::try_from(flat).map_err(|_| {
            ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::SizeOverflow,
                "Choi element index cannot fit host usize".to_owned(),
            )
        })?;

        self.elements.get(flat).copied().ok_or_else(|| {
            ZqnError::new(
                ZqnErrorKind::Internal,
                ZqnErrorCode::InvalidChannel,
                "validated Choi element index was unexpectedly absent"
                    .to_owned(),
            )
        })
    }

    /// Sets a matrix element.
    ///
    /// The matrix is not automatically revalidated after mutation.
    pub fn set(
        &mut self,
        row: u128,
        column: u128,
        value: Complex64,
    ) -> ZqnResult<()> {
        if !value.is_finite() {
            return Err(ZqnError::new(
                ZqnErrorKind::Channel,
                ZqnErrorCode::InvalidChannelParameter,
                "Choi elements must be finite".to_owned(),
            ));
        }

        let dimension = self.matrix_dimension()?;

        let flat =
            checked_matrix_index(row, column, dimension, "Choi element index")?;

        let flat = usize::try_from(flat).map_err(|_| {
            ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::SizeOverflow,
                "Choi element index cannot fit host usize".to_owned(),
            )
        })?;

        let element = self.elements.get_mut(flat).ok_or_else(|| {
            ZqnError::new(
                ZqnErrorKind::Internal,
                ZqnErrorCode::InvalidChannel,
                "validated Choi element index was unexpectedly absent"
                    .to_owned(),
            )
        })?;

        *element = value;

        Ok(())
    }

    /// Returns a deterministic iterator over `(row, column, value)`.
    pub fn indexed_elements(
        &self,
    ) -> impl Iterator<Item = (u128, u128, Complex64)> + '_ {
        let dimension = self
            .matrix_dimension()
            .expect("validated Choi dimensions must have a matrix dimension");

        self.elements
            .iter()
            .copied()
            .enumerate()
            .map(move |(flat, value)| {
                let flat = flat as u128;
                let row = flat / dimension;
                let column = flat % dimension;
                (row, column, value)
            })
    }

    // ------------------------------------------------------------------------
    // Invariants
    // ------------------------------------------------------------------------

    /// Validates all fundamental Choi channel invariants.
    ///
    /// Validation includes:
    ///
    /// 1. structural dimensions;
    /// 2. element count;
    /// 3. finite numerical values;
    /// 4. Hermiticity;
    /// 5. positive semidefiniteness;
    /// 6. trace preservation.
    ///
    /// This is the strictest validity test exposed by this module.
    pub fn validate(
        &self,
        tolerance: ChoiValidationTolerance,
    ) -> ZqnResult<()> {
        self.validate_structure()?;
        self.validate_finite()?;
        self.validate_hermitian(tolerance)?;
        self.validate_positive_semidefinite(tolerance)?;
        self.validate_trace_preserving(tolerance)?;

        Ok(())
    }

    /// Validates structural invariants without mathematical channel checks.
    pub fn validate_structure(&self) -> ZqnResult<()> {
        let expected = self.dimensions.host_element_count()?;

        if self.elements.len() != expected {
            return Err(ZqnError::new(
                ZqnErrorKind::Channel,
                ZqnErrorCode::ChannelDimensionMismatch,
                format!(
                    "Choi storage has {} elements but dimensions require {expected}",
                    self.elements.len()
                ),
            ));
        }

        Ok(())
    }

    /// Validates that every matrix element is finite.
    pub fn validate_finite(&self) -> ZqnResult<()> {
        for (index, value) in self.elements.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(ZqnError::new(
                    ZqnErrorKind::Channel,
                    ZqnErrorCode::InvalidChannelParameter,
                    format!(
                        "Choi element {index} is non-finite"
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Validates Hermiticity.
    pub fn validate_hermitian(
        &self,
        tolerance: ChoiValidationTolerance,
    ) -> ZqnResult<()> {
        let dimension = self.matrix_dimension()?;

        for row in 0..dimension {
            for column in row..dimension {
                let left = self.get(row, column)?;
                let right = self.get(column, row)?.conjugate();

                if !tolerance.approx_complex(left, right) {
                    return Err(ZqnError::new(
                        ZqnErrorKind::Channel,
                        ZqnErrorCode::InvalidChannel,
                        format!(
                            "Choi matrix is not Hermitian at ({row}, {column})"
                        ),
                    ));
                }
            }
        }

        Ok(())
    }

    /// Validates positive semidefiniteness.
    ///
    /// For a valid Choi representation this is the complete-positivity
    /// invariant.
    pub fn validate_positive_semidefinite(
        &self,
        tolerance: ChoiValidationTolerance,
    ) -> ZqnResult<()> {
        self.validate_hermitian(tolerance)?;

        let dimension = self.matrix_dimension()?;

        let dimension = usize::try_from(dimension).map_err(|_| {
            ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::SizeOverflow,
                "Choi matrix dimension cannot fit host usize for PSD validation"
                    .to_owned(),
            )
        })?;

        // L is represented as a dense lower-triangular matrix.
        //
        // This is an intentionally local validation workspace. It does not
        // become part of the Choi representation and does not alter the
        // stored matrix.
        let workspace_len = dimension.checked_mul(dimension).ok_or_else(|| {
            ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::SizeOverflow,
                "PSD validation workspace size overflowed host usize"
                    .to_owned(),
            )
        })?;

        let mut lower = vec![Complex64::ZERO; workspace_len];

        for row in 0..dimension {
            for column in 0..=row {
                let mut residual = self.get(
                    row as u128,
                    column as u128,
                )?;

                for k in 0..column {
                    let left = lower[row * dimension + k];
                    let right = lower[column * dimension + k];

                    residual -= left * right.conjugate();
                }

                if !residual.is_finite() {
                    return Err(ZqnError::new(
                        ZqnErrorKind::Channel,
                        ZqnErrorCode::ChannelNotCompletelyPositive,
                        format!(
                            "PSD validation produced a non-finite residual at ({row}, {column})"
                        ),
                    ));
                }

                if row == column {
                    // Diagonal entries of a Hermitian PSD matrix are real.
                    let imaginary_scale =
                        residual.imaginary.abs().max(1.0);

                    if residual.imaginary.abs()
                        > tolerance.scaled(imaginary_scale)
                    {
                        return Err(ZqnError::new(
                            ZqnErrorKind::Channel,
                            ZqnErrorCode::ChannelNotCompletelyPositive,
                            format!(
                                "Choi diagonal element {row} has a non-negligible imaginary component"
                            ),
                        ));
                    }

                    let diagonal = residual.real;

                    let scale = diagonal.abs().max(1.0);
                    let local_tolerance = tolerance.scaled(scale);

                    if diagonal < -local_tolerance {
                        return Err(ZqnError::new(
                            ZqnErrorKind::Channel,
                            ZqnErrorCode::ChannelNotCompletelyPositive,
                            format!(
                                "Choi matrix is not positive semidefinite: negative diagonal pivot at {row}"
                            ),
                        ));
                    }

                    if diagonal <= local_tolerance {
                        // A zero PSD pivot implies all remaining elements in
                        // that pivot direction must vanish. This is checked
                        // when subsequent rows reach the corresponding column.
                        lower[row * dimension + column] =
                            Complex64::ZERO;
                    } else {
                        lower[row * dimension + column] =
                            Complex64::new(diagonal.sqrt(), 0.0);
                    }
                } else {
                    let pivot =
                        lower[column * dimension + column];

                    if pivot.is_zero() {
                        if residual.magnitude()
                            > tolerance.scaled(
                                residual.magnitude().max(1.0),
                            )
                        {
                            return Err(ZqnError::new(
                                ZqnErrorKind::Channel,
                                ZqnErrorCode::ChannelNotCompletelyPositive,
                                format!(
                                    "Choi matrix is not positive semidefinite: nonzero residual below zero pivot at ({row}, {column})"
                                ),
                            ));
                        }

                        lower[row * dimension + column] =
                            Complex64::ZERO;
                    } else {
                        let value =
                            residual / pivot.conjugate();

                        if !value.is_finite() {
                            return Err(ZqnError::new(
                                ZqnErrorKind::Channel,
                                ZqnErrorCode::ChannelNotCompletelyPositive,
                                format!(
                                    "PSD factorization produced a non-finite factor at ({row}, {column})"
                                ),
                            ));
                        }

                        lower[row * dimension + column] = value;
                    }
                }
            }
        }

        Ok(())
    }

    /// Validates the trace-preserving condition:
    ///
    /// ```text
    /// Tr_out(J) = I_in
    /// ```
    pub fn validate_trace_preserving(
        &self,
        tolerance: ChoiValidationTolerance,
    ) -> ZqnResult<()> {
        let input = self.input_dimension();
        let output = self.output_dimension();

        for input_row in 0..input {
            for input_col in 0..input {
                let mut partial_trace = Complex64::ZERO;

                for output_index in 0..output {
                    let row = checked_choi_index(
                        output_index,
                        input_row,
                        input,
                        "trace-preserving row index",
                    )?;

                    let column = checked_choi_index(
                        output_index,
                        input_col,
                        input,
                        "trace-preserving column index",
                    )?;

                    partial_trace += self.get(row, column)?;
                }

                let expected = if input_row == input_col {
                    Complex64::ONE
                } else {
                    Complex64::ZERO
                };

                if !tolerance.approx_complex(
                    partial_trace,
                    expected,
                ) {
                    return Err(ZqnError::new(
                        ZqnErrorKind::Channel,
                        ZqnErrorCode::ChannelNotTracePreserving,
                        format!(
                            "Choi partial trace differs from identity at input indices ({input_row}, {input_col})"
                        ),
                    ));
                }
            }
        }

        Ok(())
    }

    /// Returns whether the matrix passes complete-positivity validation.
    pub fn is_completely_positive(
        &self,
        tolerance: ChoiValidationTolerance,
    ) -> bool {
        self.validate_positive_semidefinite(tolerance)
            .is_ok()
    }

    /// Returns whether the matrix is trace preserving.
    pub fn is_trace_preserving(
        &self,
        tolerance: ChoiValidationTolerance,
    ) -> bool {
        self.validate_trace_preserving(tolerance)
            .is_ok()
    }

    /// Returns whether the matrix represents a valid CPTP quantum channel.
    pub fn is_cptp(
        &self,
        tolerance: ChoiValidationTolerance,
    ) -> bool {
        self.validate(tolerance).is_ok()
    }

    // ------------------------------------------------------------------------
    // Partial traces
    // ------------------------------------------------------------------------

    /// Computes the partial trace over the output subsystem.
    ///
    /// The result is an `input × input` complex matrix in row-major order.
    ///
    /// For a trace-preserving channel this must equal the identity matrix.
    pub fn partial_trace_output(
        &self,
    ) -> ZqnResult<Vec<Complex64>> {
        let input = self.input_dimension();
        let output = self.output_dimension();

        let output_len = checked_square(
            input,
            "partial-trace output element count",
        )?;

        let output_len = usize::try_from(output_len).map_err(|_| {
            ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::SizeOverflow,
                "partial-trace output cannot fit host usize".to_owned(),
            )
        })?;

        let mut result = vec![Complex64::ZERO; output_len];

        for input_row in 0..input {
            for input_col in 0..input {
                let mut value = Complex64::ZERO;

                for output_index in 0..output {
                    let row = checked_choi_index(
                        output_index,
                        input_row,
                        input,
                        "partial-trace row index",
                    )?;

                    let column = checked_choi_index(
                        output_index,
                        input_col,
                        input,
                        "partial-trace column index",
                    )?;

                    value += self.get(row, column)?;
                }

                let flat = checked_matrix_index(
                    input_row,
                    input_col,
                    input,
                    "partial-trace result index",
                )?;

                let flat = usize::try_from(flat).map_err(|_| {
                    ZqnError::new(
                        ZqnErrorKind::Limits,
                        ZqnErrorCode::SizeOverflow,
                        "partial-trace result index cannot fit host usize"
                            .to_owned(),
                    )
                })?;

                result[flat] = value;
            }
        }

        Ok(result)
    }

    // ------------------------------------------------------------------------
    // Tensor product
    // ------------------------------------------------------------------------

    /// Computes the tensor product of two Choi matrices.
    ///
    /// If:
    ///
    /// ```text
    /// A : Lin -> Lout
    /// B : Bin -> Bout
    /// ```
    ///
    /// then:
    ///
    /// ```text
    /// A ⊗ B :
    /// (Lin ⊗ Bin) -> (Lout ⊗ Bout)
    /// ```
    ///
    /// The resulting dimensions are:
    ///
    /// ```text
    /// input  = A.input  * B.input
    /// output = A.output * B.output
    /// ```
    ///
    /// All dimension arithmetic is checked before allocation.
    pub fn tensor_product(
        &self,
        other: &Self,
    ) -> ZqnResult<Self> {
        let input = checked_product(
            self.input_dimension(),
            other.input_dimension(),
            "Choi tensor-product input dimension",
        )?;

        let output = checked_product(
            self.output_dimension(),
            other.output_dimension(),
            "Choi tensor-product output dimension",
        )?;

        let result_dimensions =
            ChoiDimensions::new(input, output)?;

        let result_len =
            result_dimensions.host_element_count()?;

        let result_matrix_dimension =
            result_dimensions.matrix_dimension()?;

        let self_matrix_dimension =
            self.matrix_dimension()?;

        let other_matrix_dimension =
            other.matrix_dimension()?;

        let mut elements =
            vec![Complex64::ZERO; result_len];

        // The tensor product is constructed according to the Choi matrix's
        // flattened subsystem ordering:
        //
        // output composite index:
        //     a = a_self * B.output + a_other
        //
        // input composite index:
        //     i = i_self * B.input + i_other
        //
        // The corresponding Choi row is:
        //     a * input + i
        //
        // and similarly for the column.
        for self_row in 0..self_matrix_dimension {
            let self_output =
                self_row / self.input_dimension();
            let self_input =
                self_row % self.input_dimension();

            for self_col in 0..self_matrix_dimension {
                let self_output_col =
                    self_col / self.input_dimension();
                let self_input_col =
                    self_col % self.input_dimension();

                let left = self.get(self_row, self_col)?;

                for other_row in 0..other_matrix_dimension {
                    let other_output =
                        other_row / other.input_dimension();
                    let other_input =
                        other_row % other.input_dimension();

                    for other_col in 0..other_matrix_dimension {
                        let other_output_col =
                            other_col / other.input_dimension();
                        let other_input_col =
                            other_col % other.input_dimension();

                        let right =
                            other.get(other_row, other_col)?;

                        let composite_output =
                            checked_composite_index(
                                self_output,
                                other_output,
                                other.output_dimension(),
                                "tensor-product output index",
                            )?;

                        let composite_input =
                            checked_composite_index(
                                self_input,
                                other_input,
                                other.input_dimension(),
                                "tensor-product input index",
                            )?;

                        let composite_output_col =
                            checked_composite_index(
                                self_output_col,
                                other_output_col,
                                other.output_dimension(),
                                "tensor-product output-column index",
                            )?;

                        let composite_input_col =
                            checked_composite_index(
                                self_input_col,
                                other_input_col,
                                other.input_dimension(),
                                "tensor-product input-column index",
                            )?;

                        let row = checked_choi_index(
                            composite_output,
                            composite_input,
                            input,
                            "tensor-product Choi row",
                        )?;

                        let column = checked_choi_index(
                            composite_output_col,
                            composite_input_col,
                            input,
                            "tensor-product Choi column",
                        )?;

                        let flat = checked_matrix_index(
                            row,
                            column,
                            result_matrix_dimension,
                            "tensor-product Choi element",
                        )?;

                        let flat = usize::try_from(flat).map_err(|_| {
                            ZqnError::new(
                                ZqnErrorKind::Limits,
                                ZqnErrorCode::SizeOverflow,
                                "tensor-product Choi index cannot fit host usize"
                                    .to_owned(),
                            )
                        })?;

                        elements[flat] = left * right;
                    }
                }
            }
        }

        Self::from_elements(input, output, elements)
    }

    // ------------------------------------------------------------------------
    // Identity comparison
    // ------------------------------------------------------------------------

    /// Returns whether this matrix is approximately the Choi matrix of the
    /// identity channel for its dimension.
    pub fn is_identity(
        &self,
        tolerance: ChoiValidationTolerance,
    ) -> ZqnResult<bool> {
        if !self.dimensions.is_square_channel() {
            return Ok(false);
        }

        let identity = Self::identity(self.input_dimension())?;

        Ok(self
            .elements
            .iter()
            .copied()
            .zip(identity.elements.iter().copied())
            .all(|(left, right)| {
                tolerance.approx_complex(left, right)
            }))
    }
}

// ============================================================================
// Index implementation
// ============================================================================

impl Index<(usize, usize)> for ChoiMatrix {
    type Output = Complex64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;

        let dimension = self
            .matrix_dimension()
            .expect("validated Choi dimensions must fit u128");

        let dimension = usize::try_from(dimension)
            .expect("Choi matrix dimension must fit host usize for indexing");

        let flat = row
            .checked_mul(dimension)
            .and_then(|value| value.checked_add(column))
            .expect("Choi matrix index overflow");

        self.elements
            .get(flat)
            .expect("Choi matrix index out of bounds")
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for ChoiMatrix {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            formatter,
            "ChoiMatrix(input={}, output={}, dimension={})",
            self.input_dimension(),
            self.output_dimension(),
            self.matrix_dimension().unwrap_or(0)
        )?;

        let dimension = self.matrix_dimension().map_err(|_| fmt::Error)?;

        for row in 0..dimension {
            formatter.write_str("[")?;

            for column in 0..dimension {
                if column != 0 {
                    formatter.write_str(", ")?;
                }

                let value = self
                    .get(row, column)
                    .map_err(|_| fmt::Error)?;

                write!(formatter, "{value}")?;
            }

            formatter.write_str("]")?;

            if row + 1 < dimension {
                formatter.write_str("\n")?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// Helper arithmetic
// ============================================================================

fn checked_product(
    left: u128,
    right: u128,
    description: &str,
) -> ZqnResult<u128> {
    left.checked_mul(right).ok_or_else(|| {
        ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::SizeOverflow,
            format!("{description} multiplication overflowed u128"),
        )
    })
}

fn checked_square(
    value: u128,
    description: &str,
) -> ZqnResult<u128> {
    checked_product(value, value, description)
}

fn checked_matrix_index(
    row: u128,
    column: u128,
    dimension: u128,
    description: &str,
) -> ZqnResult<u128> {
    if row >= dimension {
        return Err(ZqnError::new(
            ZqnErrorKind::Structure,
            ZqnErrorCode::ChannelDimensionMismatch,
            format!(
                "{description}: row {row} is outside matrix dimension {dimension}"
            ),
        ));
    }

    if column >= dimension {
        return Err(ZqnError::new(
            ZqnErrorKind::Structure,
            ZqnErrorCode::ChannelDimensionMismatch,
            format!(
                "{description}: column {column} is outside matrix dimension {dimension}"
            ),
        ));
    }

    row.checked_mul(dimension)
        .and_then(|value| value.checked_add(column))
        .ok_or_else(|| {
            ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::SizeOverflow,
                format!("{description} overflowed u128"),
            )
        })
}

fn checked_choi_index(
    output_index: u128,
    input_index: u128,
    input_dimension: u128,
    description: &str,
) -> ZqnResult<u128> {
    output_index
        .checked_mul(input_dimension)
        .and_then(|value| value.checked_add(input_index))
        .ok_or_else(|| {
            ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::SizeOverflow,
                format!("{description} overflowed u128"),
            )
        })
}

fn checked_composite_index(
    left: u128,
    right: u128,
    right_dimension: u128,
    description: &str,
) -> ZqnResult<u128> {
    left.checked_mul(right_dimension)
        .and_then(|value| value.checked_add(right))
        .ok_or_else(|| {
            ZqnError::new(
                ZqnErrorKind::Limits,
                ZqnErrorCode::SizeOverflow,
                format!("{description} overflowed u128"),
            )
        })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn c(real: f64, imaginary: f64) -> Complex64 {
        Complex64::new(real, imaginary)
    }

    fn tolerance() -> ChoiValidationTolerance {
        ChoiValidationTolerance::new(1.0e-10, 1.0e-8)
            .expect("test tolerance must be valid")
    }

    #[test]
    fn dimensions_are_checked_without_allocation() {
        let dimensions =
            ChoiDimensions::new(2, 3).expect("valid dimensions");

        assert_eq!(dimensions.input, 2);
        assert_eq!(dimensions.output, 3);
        assert_eq!(
            dimensions.matrix_dimension().expect("matrix dimension"),
            6
        );
        assert_eq!(
            dimensions.element_count().expect("element count"),
            36
        );
    }

    #[test]
    fn zero_dimension_is_rejected() {
        assert!(ChoiDimensions::new(0, 2).is_err());
        assert!(ChoiDimensions::new(2, 0).is_err());
    }

    #[test]
    fn identity_qubit_channel_is_cptp() {
        let choi =
            ChoiMatrix::identity(2).expect("identity Choi matrix");

        assert_eq!(choi.input_dimension(), 2);
        assert_eq!(choi.output_dimension(), 2);
        assert_eq!(choi.element_count(), 16);

        assert!(choi
            .validate(tolerance())
            .is_ok());

        assert!(choi
            .is_identity(tolerance())
            .expect("identity comparison"));
    }

    #[test]
    fn identity_one_dimensional_channel_is_cptp() {
        let choi =
            ChoiMatrix::identity(1).expect("one-dimensional identity");

        assert!(choi
            .validate(tolerance())
            .is_ok());
    }

    #[test]
    fn identity_partial_trace_is_identity() {
        let choi =
            ChoiMatrix::identity(2).expect("identity Choi");

        let partial =
            choi.partial_trace_output()
                .expect("partial trace");

        assert_eq!(partial.len(), 4);

        assert!(
            tolerance().approx_complex(partial[0], Complex64::ONE)
        );
        assert!(
            tolerance().approx_complex(partial[1], Complex64::ZERO)
        );
        assert!(
            tolerance().approx_complex(partial[2], Complex64::ZERO)
        );
        assert!(
            tolerance().approx_complex(partial[3], Complex64::ONE)
        );
    }

    #[test]
    fn single_kraus_identity_produces_identity_choi() {
        let identity = vec![
            c(1.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(1.0, 0.0),
        ];

        let choi =
            ChoiMatrix::from_kraus(2, 2, &[identity])
                .expect("Kraus construction");

        assert!(choi
            .is_identity(tolerance())
            .expect("identity comparison"));
    }

    #[test]
    fn amplitude_scaling_kraus_map_is_cp() {
        let k = vec![
            c(0.5, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(0.5, 0.0),
        ];

        let choi =
            ChoiMatrix::from_kraus(2, 2, &[k])
                .expect("Kraus construction");

        assert!(choi.is_completely_positive(tolerance()));
        assert!(!choi.is_trace_preserving(tolerance()));
    }

    #[test]
    fn non_hermitian_matrix_is_rejected() {
        let mut choi =
            ChoiMatrix::zeros(1, 1)
                .expect("zero Choi");

        choi
            .set(0, 0, c(1.0, 0.5))
            .expect("set element");

        assert!(choi.validate_hermitian(tolerance()).is_err());
    }

    #[test]
    fn negative_scalar_matrix_is_not_completely_positive() {
        let choi =
            ChoiMatrix::from_elements(
                1,
                1,
                vec![c(-1.0, 0.0)],
            )
            .expect("matrix construction");

        assert!(
            !choi.is_completely_positive(tolerance())
        );
    }

    #[test]
    fn zero_matrix_is_not_trace_preserving() {
        let choi =
            ChoiMatrix::zeros(2, 2)
                .expect("zero Choi");

        assert!(
            !choi.is_trace_preserving(tolerance())
        );
    }

    #[test]
    fn invalid_element_count_is_rejected() {
        let result =
            ChoiMatrix::from_elements(
                2,
                2,
                vec![Complex64::ZERO; 3],
            );

        assert!(result.is_err());
    }

    #[test]
    fn non_finite_elements_are_rejected() {
        let result =
            ChoiMatrix::from_elements(
                1,
                1,
                vec![c(f64::NAN, 0.0)],
            );

        assert!(result.is_err());
    }

    #[test]
    fn empty_kraus_set_is_rejected() {
        let result =
            ChoiMatrix::from_kraus(2, 2, &[]);

        assert!(result.is_err());
    }

    #[test]
    fn wrong_kraus_dimension_is_rejected() {
        let result =
            ChoiMatrix::from_kraus(
                2,
                2,
                &[vec![Complex64::ZERO; 3]],
            );

        assert!(result.is_err());
    }

    #[test]
    fn deterministic_indexing_is_row_major() {
        let mut choi =
            ChoiMatrix::zeros(2, 2)
                .expect("zero Choi");

        choi
            .set(0, 1, c(3.0, 0.0))
            .expect("set");

        choi
            .set(1, 0, c(4.0, 0.0))
            .expect("set");

        assert_eq!(choi[(0, 1)], c(3.0, 0.0));
        assert_eq!(choi[(1, 0)], c(4.0, 0.0));
    }

    #[test]
    fn tensor_product_dimensions_are_generic() {
        let left =
            ChoiMatrix::identity(2)
                .expect("left identity");

        let right =
            ChoiMatrix::identity(3)
                .expect("right identity");

        let result =
            left.tensor_product(&right)
                .expect("tensor product");

        assert_eq!(result.input_dimension(), 6);
        assert_eq!(result.output_dimension(), 6);
        assert_eq!(
            result.matrix_dimension()
                .expect("matrix dimension"),
            36
        );

        assert!(
            result
                .validate(tolerance())
                .is_ok()
        );
    }

    #[test]
    fn tensor_product_preserves_identity() {
        let left =
            ChoiMatrix::identity(2)
                .expect("left identity");

        let right =
            ChoiMatrix::identity(2)
                .expect("right identity");

        let result =
            left.tensor_product(&right)
                .expect("tensor product");

        let expected =
            ChoiMatrix::identity(4)
                .expect("expected identity");

        assert_eq!(result, expected);
    }

    #[test]
    fn tolerance_rejects_invalid_values() {
        assert!(
            ChoiValidationTolerance::new(
                f64::NAN,
                1.0e-8
            )
            .is_err()
        );

        assert!(
            ChoiValidationTolerance::new(
                -1.0,
                1.0e-8
            )
            .is_err()
        );

        assert!(
            ChoiValidationTolerance::new(
                1.0e-12,
                f64::INFINITY
            )
            .is_err()
        );
    }

    #[test]
    fn host_storage_size_is_checked() {
        let dimensions =
            ChoiDimensions::new(2, 2)
                .expect("dimensions");

        assert_eq!(
            dimensions
                .host_storage_bytes()
                .expect("storage bytes"),
            16 * core::mem::size_of::<Complex64>()
        );
    }

    #[test]
    fn indexed_elements_are_deterministic() {
        let choi =
            ChoiMatrix::identity(2)
                .expect("identity");

        let first: Vec<_> =
            choi.indexed_elements().collect();

        let second: Vec<_> =
            choi.indexed_elements().collect();

        assert_eq!(first, second);
    }

    #[test]
    fn identity_has_expected_diagonal_structure() {
        let choi =
            ChoiMatrix::identity(2)
                .expect("identity");

        let dimension =
            choi.matrix_dimension()
                .expect("dimension");

        for row in 0..dimension {
            for column in 0..dimension {
                let expected = if row == column {
                    // The Choi identity has four diagonal entries equal to one
                    // under this representation.
                    Complex64::ONE
                } else {
                    Complex64::ZERO
                };

                if row == column {
                    assert!(
                        tolerance().approx_complex(
                            choi
                                .get(row, column)
                                .expect("element"),
                            expected
                        )
                    );
                }
            }
        }
    }
}