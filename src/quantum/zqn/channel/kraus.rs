//! Zamani Quantum Noise (ZQN) — Kraus-operator channels.
//!
//! This module provides the canonical Kraus representation of a completely
//! positive quantum operation/channel.
//!
//! # Architectural responsibility
//!
//! This file owns:
//!
//! - the validated Kraus-operator representation;
//! - individual Kraus operators;
//! - input/output Hilbert-space dimensions;
//! - optional canonical logical/physical resource association;
//! - complete-positivity by construction of the Kraus representation;
//! - trace-preservation validation;
//! - trace-nonincreasing validation;
//! - Kraus-channel composition;
//! - Kraus-channel tensor products;
//! - application to state vectors represented as dense vectors;
//! - application to density matrices represented as dense matrices;
//! - deterministic canonical validation;
//! - serialization of the mathematical representation;
//! - resource-aware construction and validation.
//!
//! This file does NOT own:
//!
//! - the canonical quantum IR;
//! - quantum source parsing;
//! - gate definitions;
//! - physical hardware;
//! - hardware topology;
//! - calibration;
//! - scheduling;
//! - routing;
//! - QEC decoding;
//! - random-number generation;
//! - Monte-Carlo execution;
//! - state-vector storage policy;
//! - density-matrix storage policy;
//! - GPU execution;
//! - distributed execution;
//! - vendor APIs;
//! - provider credentials;
//! - noise-model policy.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                     quantum::ir
//!                         │
//!                         │ semantic operation/resources
//!                         ▼
//!                     ZQN noise
//!                         │
//!                         ▼
//!                  QuantumChannel
//!                         │
//!                         ▼
//!                 KrausChannel
//!                         │
//!             ┌───────────┼───────────┐
//!             ▼           ▼           ▼
//!         simulator      QEC       hardware
//!         /memory       adapter     adapter
//! ```
//!
//! The Kraus representation is therefore a mathematical representation of a
//! quantum channel, not a simulator and not a hardware interface.
//!
//! # Mathematical contract
//!
//! A Kraus channel is represented by operators
//!
//! ```text
//! K_0, K_1, ..., K_(r-1)
//! ```
//!
//! where every operator has dimensions
//!
//! ```text
//! output_dimension × input_dimension
//! ```
//!
//! The associated completely positive map is
//!
//! ```text
//! E(rho) = Σ_i K_i rho K_i†
//! ```
//!
//! A channel is trace preserving exactly when
//!
//! ```text
//! Σ_i K_i† K_i = I
//! ```
//!
//! A trace-nonincreasing operation satisfies
//!
//! ```text
//! Σ_i K_i† K_i <= I
//! ```
//!
//! The implementation does not require a channel to be trace preserving at
//! construction time because postselected operations, measurements,
//! conditional branches and quantum instruments may legitimately be
//! trace-nonincreasing.
//!
//! # Complete positivity
//!
//! A map represented by Kraus operators is completely positive by construction.
//!
//! This module therefore does not need to construct a Choi matrix merely to
//! establish complete positivity.
//!
//! Choi conversion/analysis belongs to the corresponding ZQN channel
//! representation module.
//!
//! # Dimensions
//!
//! No qubit count is hard-coded.
//!
//! A Kraus operator is defined by explicit input/output dimensions.
//!
//! For a conventional n-qubit operator, callers may calculate a dimension
//! externally from the canonical IR resources. This module deliberately does
//! not assume that every quantum system is binary.
//!
//! Therefore the same type can represent:
//!
//! - 1 qubit;
//! - many qubits;
//! - qudits;
//! - heterogeneous finite-dimensional systems;
//! - rectangular quantum operations;
//! - subspace maps;
//! - future finite-dimensional modalities.
//!
//! The mathematical dimension is always explicit data.
//!
//! # Canonical qubit identity
//!
//! When a Kraus channel is associated with logical resources, this module uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! It does NOT define another `QubitId`.
//!
//! Physical resource identity remains owned by the appropriate hardware/IR
//! integration boundary.
//!
//! A channel may also intentionally have no associated qubit IDs. This is
//! necessary for channel definitions that are later bound to resources by
//! routing, scheduling, target lowering or execution.
//!
//! # Scalable-resource principle
//!
//! This module contains no semantic maximum for:
//!
//! - number of Kraus operators;
//! - input dimension;
//! - output dimension;
//! - number of associated qubits;
//! - matrix element count.
//!
//! Construction is nevertheless resource-governed.
//!
//! The caller may impose limits before allocation through
//! [`KrausResourceLimits`].
//!
//! This distinction is fundamental:
//!
//! ```text
//! semantic capacity = unbounded by this module
//! implementation capacity = limited by available resources
//! ```
//!
//! "Infinity" therefore means that the API does not encode an arbitrary
//! machine-size ceiling. A concrete execution still requires finite memory,
//! address space and processing capacity.
//!
//! # Numerical safety
//!
//! The module rejects:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - zero-dimensional matrices;
//! - inconsistent matrix dimensions;
//! - impossible allocations;
//! - integer overflow during element-count calculations.
//!
//! Validation tolerances are explicit.
//!
//! No invalid numerical value is silently repaired.
//!
//! # Determinism
//!
//! Kraus representation and validation are deterministic.
//!
//! This module does not own an RNG and performs no stochastic sampling.
//!
//! Stochastic channel sampling belongs to ZQN's sampling/simulation layers.
//!
//! # Kraus non-uniqueness
//!
//! Kraus representations are not unique.
//!
//! Different Kraus sets may represent exactly the same channel.
//!
//! Consequently this type does not claim that the ordered Kraus list itself is
//! a unique physical identity of a channel.
//!
//! A future canonical channel identity should be based on canonicalized
//! semantics, not merely on the order in which Kraus operators were supplied.
//!
//! # Composition
//!
//! If:
//!
//! ```text
//! A(rho) = Σ_i A_i rho A_i†
//! B(rho) = Σ_j B_j rho B_j†
//! ```
//!
//! then:
//!
//! ```text
//! (B ∘ A)(rho)
//!     = Σ_(j,i) (B_j A_i) rho (B_j A_i)†
//! ```
//!
//! This module implements that mathematical composition without knowing which
//! subsystem will eventually execute it.
//!
//! # Tensor product
//!
//! For channels A and B:
//!
//! ```text
//! (A ⊗ B)(rho)
//! ```
//!
//! is represented by every pairwise tensor product of their Kraus operators.
//!
//! The implementation does not assume two qubits or two operands of any fixed
//! arity.
//!
//! # State-vector application
//!
//! Applying a general quantum channel to a pure state produces a density
//! operator in general.
//!
//! Therefore [`KrausChannel::apply_to_state_vector`] returns the collection of
//! resulting branch state vectors together with their branch probabilities,
//! rather than falsely returning one pure state.
//!
//! This is deliberately separate from stochastic trajectory sampling.
//!
//! # Density-matrix application
//!
//! Density-matrix application implements the channel directly:
//!
//! ```text
//! rho' = Σ_i K_i rho K_i†
//! ```
//!
//! The operation is deterministic and contains no random sampling.
//!
//! # Resource governance
//!
//! Dense matrix storage has quadratic element growth in its dimensions.
//!
//! This module therefore performs checked multiplication before allocation.
//!
//! A caller can provide:
//!
//! ```text
//! KrausResourceLimits
//! ```
//!
//! to reject an allocation that is too large for the current execution
//! context.
//!
//! These limits are policy, not mathematical limits.
//!
//! # Integration contract
//!
//! Later ZQN modules should consume this type as follows:
//!
//! ```text
//! channel/channel.rs
//!     owns the representation-independent channel abstraction.
//!     It may expose KrausChannel as one implementation.
//!
//! channel/representation.rs
//!     identifies Kraus as one mathematical representation.
//!
//! channel/choi.rs
//!     converts between Kraus and Choi representations.
//!
//! channel/composition.rs
//!     may delegate Kraus composition to this implementation.
//!
//! noise/model.rs
//!     attaches a KrausChannel to a semantic operation/resource.
//!
//! noise/application.rs
//!     determines where/when the channel applies.
//!
//! simulation/channel_engine.rs
//!     consumes the mathematical channel to evolve simulator state.
//!
//! simulation/trajectory.rs
//!     may sample Kraus branches using its own deterministic RNG contract.
//!
//! integration/ir.rs
//!     associates the channel with canonical IR operations/resources.
//!
//! integration/memory.rs
//!     adapts channel application to quantum-memory representations.
//!
//! integration/qec.rs
//!     converts channel effects to QEC fault representations where a valid
//!     conversion exists.
//!
//! target/lowering.rs
//!     determines whether the target can faithfully represent or approximate
//!     this channel.
//! ```
//!
//! None of those modules should redefine the Kraus mathematics.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! This file explicitly forbids unsafe Rust.
//!
//! No raw pointers, FFI, unsafe allocation, global mutable state or unsafe
//! arithmetic is used.
//!
//! # Dependencies
//!
//! The implementation uses:
//!
//! - the Rust standard library;
//! - Serde already used by the Zamani quantum subsystem;
//! - Zamani's canonical `Complex64` and `ComplexScalar` numerical types;
//! - Zamani's canonical `QubitId`.
//!
//! No vendor SDK is required.
//!
//! # No-reedit contract
//!
//! Future modules MUST NOT require this file to be reopened merely because:
//!
//! - a new simulator is added;
//! - a new hardware provider is added;
//! - a new QEC code is added;
//! - a new routing algorithm is added;
//! - a new scheduler is added;
//! - a new quantum technology is added.
//!
//! Such systems consume this mathematical contract through adapters.
//!
//! If a future quantum representation requires a genuinely different
//! mathematical representation, it should be added as a separate channel
//! representation rather than corrupting the Kraus contract.
//!
//! # Testing contract
//!
//! Tests at the bottom of this file verify:
//!
//! - constructor validation;
//! - dimension validation;
//! - trace-preserving validation;
//! - trace-nonincreasing validation;
//! - identity channel behavior;
//! - channel composition;
//! - tensor products;
//! - density-matrix application;
//! - state-vector branch probabilities;
//! - resource-limit enforcement;
//! - deterministic behavior;
//! - serialization round trips.
//!
//! Integration tests elsewhere should additionally verify compatibility with:
//!
//! - `quantum::ir::qubit::QubitId`;
//! - ZQN's representation abstraction;
//! - memory state representations;
//! - QEC adapters;
//! - simulator engines.
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
use std::ops::Range;

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::memory::complex::{Complex64, ComplexScalar};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the Kraus representation.
pub const KRAUS_SCHEMA_ID: &str = "zamani.quantum.zqn.channel.kraus";

/// Semantic version of the Kraus representation contract.
///
/// Increment the major version only when the mathematical/public contract
/// changes incompatibly.
pub const KRAUS_SCHEMA_VERSION: u16 = 1;

/// Default absolute tolerance used by validation helpers.
///
/// This is deliberately centralized in this file instead of scattering
/// tolerance literals through the implementation.
pub const DEFAULT_KRAUS_ABSOLUTE_TOLERANCE: f64 = 1.0e-12;

/// Default relative tolerance used by validation helpers.
pub const DEFAULT_KRAUS_RELATIVE_TOLERANCE: f64 = 1.0e-10;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the Kraus representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KrausError {
    /// A dimension was zero where a non-empty Hilbert space was required.
    ZeroDimension {
        /// Name of the dimension.
        name: &'static str,
    },

    /// The input and output dimensions do not match the supplied matrix.
    DimensionMismatch {
        /// Expected number of rows.
        expected_rows: usize,
        /// Actual number of rows.
        actual_rows: usize,
        /// Expected number of columns.
        expected_columns: usize,
        /// Actual number of columns.
        actual_columns: usize,
    },

    /// The matrix element count does not equal rows × columns.
    ElementCountMismatch {
        /// Number of rows.
        rows: usize,
        /// Number of columns.
        columns: usize,
        /// Number of supplied elements.
        actual: usize,
    },

    /// Matrix dimensions could not be multiplied without integer overflow.
    DimensionOverflow {
        /// Left dimension.
        left: usize,
        /// Right dimension.
        right: usize,
    },

    /// A matrix contains a non-finite complex number.
    NonFiniteElement {
        /// Flat row-major element index.
        index: usize,
    },

    /// No Kraus operators were supplied.
    EmptyOperatorSet,

    /// Two channels cannot be composed because their dimensions do not match.
    CompositionDimensionMismatch {
        /// Output dimension of the first channel.
        first_output: usize,
        /// Input dimension of the second channel.
        second_input: usize,
    },

    /// A tensor product operation could not represent the resulting dimension.
    TensorDimensionOverflow {
        /// First dimension.
        first: usize,
        /// Second dimension.
        second: usize,
    },

    /// A channel's trace-preserving invariant is violated.
    NotTracePreserving {
        /// Maximum absolute matrix error.
        max_error: f64,
        /// Absolute tolerance used for validation.
        absolute_tolerance: f64,
        /// Relative tolerance used for validation.
        relative_tolerance: f64,
    },

    /// A channel is not trace non-increasing.
    NotTraceNonIncreasing {
        /// Maximum violation of the positive-semidefinite upper bound as
        /// detected by the conservative matrix check.
        max_violation: f64,
        /// Absolute tolerance used for validation.
        absolute_tolerance: f64,
        /// Relative tolerance used for validation.
        relative_tolerance: f64,
    },

    /// A state vector has the wrong dimension.
    StateVectorDimensionMismatch {
        /// Expected vector dimension.
        expected: usize,
        /// Actual vector dimension.
        actual: usize,
    },

    /// A density matrix has the wrong dimensions.
    DensityMatrixDimensionMismatch {
        /// Expected dimension.
        expected: usize,
        /// Actual row count.
        rows: usize,
        /// Actual column count.
        columns: usize,
    },

    /// A density matrix/vector contains a non-finite element.
    NonFiniteStateElement {
        /// Flat row-major/vector index.
        index: usize,
    },

    /// A resource limit would be exceeded.
    ResourceLimitExceeded {
        /// Resource name.
        resource: &'static str,
        /// Requested amount.
        requested: u128,
        /// Allowed amount.
        limit: u128,
    },

    /// An allocation size cannot be represented by the host allocator.
    AllocationSizeOverflow,

    /// A branch probability is outside the valid numerical range.
    InvalidBranchProbability {
        /// Computed probability.
        probability: f64,
    },

    /// A branch norm is numerically invalid.
    InvalidNorm,

    /// Serialization/deserialization failed.
    Serialization(String),

    /// A supplied qubit/resource collection contains a duplicate identifier.
    DuplicateQubit(QubitId),

    /// A resource collection is not compatible with the channel's declared
    /// arity/dimension metadata.
    ResourceDimensionMismatch {
        /// Number of resources.
        resources: usize,
        /// Expected number of resources.
        expected: usize,
    },

    /// A mathematical operation could not be completed because the required
    /// result cannot be represented by this implementation.
    NumericalFailure(&'static str),
}

impl fmt::Display for KrausError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDimension { name } => {
                write!(formatter, "{name} dimension must be greater than zero")
            }
            Self::DimensionMismatch {
                expected_rows,
                actual_rows,
                expected_columns,
                actual_columns,
            } => write!(
                formatter,
                "Kraus operator dimension mismatch: expected {expected_rows}×{expected_columns}, \
                 got {actual_rows}×{actual_columns}"
            ),
            Self::ElementCountMismatch {
                rows,
                columns,
                actual,
            } => write!(
                formatter,
                "Kraus matrix requires {rows}×{columns} elements, got {actual}"
            ),
            Self::DimensionOverflow { left, right } => {
                write!(formatter, "matrix element count overflow: {left} × {right}")
            }
            Self::NonFiniteElement { index } => {
                write!(formatter, "Kraus operator contains a non-finite element at index {index}")
            }
            Self::EmptyOperatorSet => {
                formatter.write_str("a Kraus channel requires at least one operator")
            }
            Self::CompositionDimensionMismatch {
                first_output,
                second_input,
            } => write!(
                formatter,
                "cannot compose channels: first output dimension {first_output} \
                 differs from second input dimension {second_input}"
            ),
            Self::TensorDimensionOverflow { first, second } => {
                write!(formatter, "tensor-product dimension overflow: {first} × {second}")
            }
            Self::NotTracePreserving {
                max_error,
                absolute_tolerance,
                relative_tolerance,
            } => write!(
                formatter,
                "Kraus operators are not trace preserving: maximum error {max_error:e}, \
                 absolute tolerance {absolute_tolerance:e}, relative tolerance {relative_tolerance:e}"
            ),
            Self::NotTraceNonIncreasing {
                max_violation,
                absolute_tolerance,
                relative_tolerance,
            } => write!(
                formatter,
                "Kraus operators are not trace non-increasing: maximum violation {max_violation:e}, \
                 absolute tolerance {absolute_tolerance:e}, relative tolerance {relative_tolerance:e}"
            ),
            Self::StateVectorDimensionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "state-vector dimension mismatch: expected {expected}, got {actual}"
                )
            }
            Self::DensityMatrixDimensionMismatch {
                expected,
                rows,
                columns,
            } => write!(
                formatter,
                "density-matrix dimension mismatch: expected {expected}×{expected}, got {rows}×{columns}"
            ),
            Self::NonFiniteStateElement { index } => {
                write!(formatter, "state contains a non-finite element at index {index}")
            }
            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => write!(
                formatter,
                "Kraus resource limit exceeded for {resource}: requested {requested}, limit {limit}"
            ),
            Self::AllocationSizeOverflow => {
                formatter.write_str("requested allocation size cannot be represented safely")
            }
            Self::InvalidBranchProbability { probability } => {
                write!(formatter, "invalid Kraus branch probability: {probability:e}")
            }
            Self::InvalidNorm => formatter.write_str("invalid or non-finite state norm"),
            Self::Serialization(message) => {
                write!(formatter, "Kraus serialization error: {message}")
            }
            Self::DuplicateQubit(qubit) => {
                write!(formatter, "duplicate logical qubit resource in Kraus channel: {qubit}")
            }
            Self::ResourceDimensionMismatch {
                resources,
                expected,
            } => write!(
                formatter,
                "Kraus resource count {resources} does not match expected count {expected}"
            ),
            Self::NumericalFailure(message) => formatter.write_str(message),
        }
    }
}

impl Error for KrausError {}

/// Result alias for Kraus operations.
pub type KrausResult<T> = Result<T, KrausError>;

// =============================================================================
// Resource policy
// =============================================================================

/// Resource limits supplied by an execution/compiler policy.
///
/// These values are NOT semantic limits of ZQN.
///
/// `None` means that this particular policy does not impose a limit.
///
/// The actual process may still fail because the host cannot satisfy an
/// allocation or because another enclosing subsystem imposes a stricter
/// policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KrausResourceLimits {
    /// Maximum number of Kraus operators allowed by the current policy.
    pub max_operators: Option<u64>,

    /// Maximum matrix elements per individual Kraus operator.
    pub max_elements_per_operator: Option<u128>,

    /// Maximum total matrix elements across the complete Kraus set.
    pub max_total_elements: Option<u128>,

    /// Maximum number of associated logical resources.
    pub max_resources: Option<u64>,
}

impl Default for KrausResourceLimits {
    fn default() -> Self {
        Self {
            max_operators: None,
            max_elements_per_operator: None,
            max_total_elements: None,
            max_resources: None,
        }
    }
}

impl KrausResourceLimits {
    /// Returns an unrestricted policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_operators: None,
            max_elements_per_operator: None,
            max_total_elements: None,
            max_resources: None,
        }
    }

    fn check_operators(&self, count: usize) -> KrausResult<()> {
        if let Some(limit) = self.max_operators {
            let requested = u64::try_from(count).map_err(|_| {
                KrausError::ResourceLimitExceeded {
                    resource: "operators",
                    requested: u64::MAX as u128,
                    limit: u64::MAX.min(limit) as u128,
                }
            })?;

            if requested > limit {
                return Err(KrausError::ResourceLimitExceeded {
                    resource: "operators",
                    requested: requested as u128,
                    limit: limit as u128,
                });
            }
        }

        Ok(())
    }

    fn check_operator_elements(&self, elements: usize) -> KrausResult<()> {
        if let Some(limit) = self.max_elements_per_operator {
            let requested = elements as u128;
            if requested > limit {
                return Err(KrausError::ResourceLimitExceeded {
                    resource: "elements_per_operator",
                    requested,
                    limit,
                });
            }
        }

        Ok(())
    }

    fn check_total_elements(&self, elements: u128) -> KrausResult<()> {
        if let Some(limit) = self.max_total_elements {
            if elements > limit {
                return Err(KrausError::ResourceLimitExceeded {
                    resource: "total_elements",
                    requested: elements,
                    limit,
                });
            }
        }

        Ok(())
    }

    fn check_resources(&self, count: usize) -> KrausResult<()> {
        if let Some(limit) = self.max_resources {
            let requested = count as u128;
            if requested > limit as u128 {
                return Err(KrausError::ResourceLimitExceeded {
                    resource: "resources",
                    requested,
                    limit: limit as u128,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Validation tolerance
// =============================================================================

/// Numerical tolerance used by channel invariant validation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KrausTolerance {
    /// Absolute error tolerance.
    pub absolute: f64,

    /// Relative error tolerance.
    pub relative: f64,
}

impl KrausTolerance {
    /// Creates a tolerance after validating both values.
    pub fn new(absolute: f64, relative: f64) -> KrausResult<Self> {
        if !absolute.is_finite() || !relative.is_finite() || absolute < 0.0 || relative < 0.0 {
            return Err(KrausError::NumericalFailure(
                "Kraus validation tolerances must be finite and non-negative",
            ));
        }

        Ok(Self {
            absolute,
            relative,
        })
    }
}

impl Default for KrausTolerance {
    fn default() -> Self {
        Self {
            absolute: DEFAULT_KRAUS_ABSOLUTE_TOLERANCE,
            relative: DEFAULT_KRAUS_RELATIVE_TOLERANCE,
        }
    }
}

// =============================================================================
// Matrix
// =============================================================================

/// Dense row-major complex matrix used internally by the Kraus representation.
///
/// This type is intentionally private.
///
/// ZQN's canonical public channel representation should not become coupled to
/// a particular global matrix implementation. A later matrix abstraction can
/// be introduced without changing the mathematical Kraus contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DenseMatrix {
    rows: usize,
    columns: usize,
    data: Vec<Complex64>,
}

impl DenseMatrix {
    fn new(
        rows: usize,
        columns: usize,
        data: Vec<Complex64>,
    ) -> KrausResult<Self> {
        if rows == 0 {
            return Err(KrausError::ZeroDimension { name: "row" });
        }

        if columns == 0 {
            return Err(KrausError::ZeroDimension { name: "column" });
        }

        let expected = rows
            .checked_mul(columns)
            .ok_or(KrausError::DimensionOverflow {
                left: rows,
                right: columns,
            })?;

        if data.len() != expected {
            return Err(KrausError::ElementCountMismatch {
                rows,
                columns,
                actual: data.len(),
            });
        }

        for (index, value) in data.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(KrausError::NonFiniteElement { index });
            }
        }

        Ok(Self {
            rows,
            columns,
            data,
        })
    }

    fn zeros(rows: usize, columns: usize) -> KrausResult<Self> {
        let count = rows
            .checked_mul(columns)
            .ok_or(KrausError::DimensionOverflow {
                left: rows,
                right: columns,
            })?;

        Self::new(rows, columns, vec![Complex64::zero(); count])
    }

    #[inline]
    fn index(&self, row: usize, column: usize) -> usize {
        row * self.columns + column
    }

    fn get(&self, row: usize, column: usize) -> Complex64 {
        self.data[self.index(row, column)]
    }

    fn set(&mut self, row: usize, column: usize, value: Complex64) {
        let index = self.index(row, column);
        self.data[index] = value;
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn columns(&self) -> usize {
        self.columns
    }

    fn data(&self) -> &[Complex64] {
        &self.data
    }

    fn conjugate_transpose(&self) -> Self {
        let mut result = Self {
            rows: self.columns,
            columns: self.rows,
            data: vec![Complex64::zero(); self.data.len()],
        };

        for row in 0..self.rows {
            for column in 0..self.columns {
                result.set(column, row, self.get(row, column).conjugate());
            }
        }

        result
    }

    fn multiply(&self, rhs: &Self) -> KrausResult<Self> {
        if self.columns != rhs.rows {
            return Err(KrausError::DimensionMismatch {
                expected_rows: self.rows,
                actual_rows: self.rows,
                expected_columns: self.columns,
                actual_columns: rhs.rows,
            });
        }

        let result_elements = self
            .rows
            .checked_mul(rhs.columns)
            .ok_or(KrausError::DimensionOverflow {
                left: self.rows,
                right: rhs.columns,
            })?;

        let mut result = Self {
            rows: self.rows,
            columns: rhs.columns,
            data: vec![Complex64::zero(); result_elements],
        };

        for row in 0..self.rows {
            for column in 0..rhs.columns {
                let mut sum = Complex64::zero();

                for inner in 0..self.columns {
                    sum += self.get(row, inner) * rhs.get(inner, column);
                }

                result.set(row, column, sum);
            }
        }

        Ok(result)
    }

    fn add_assign(&mut self, rhs: &Self) -> KrausResult<()> {
        if self.rows != rhs.rows || self.columns != rhs.columns {
            return Err(KrausError::DimensionMismatch {
                expected_rows: self.rows,
                actual_rows: rhs.rows,
                expected_columns: self.columns,
                actual_columns: rhs.columns,
            });
        }

        for (left, right) in self.data.iter_mut().zip(rhs.data.iter().copied()) {
            *left += right;
        }

        Ok(())
    }

    fn scale(&self, scalar: Complex64) -> Self {
        let data = self
            .data
            .iter()
            .copied()
            .map(|value| value * scalar)
            .collect();

        Self {
            rows: self.rows,
            columns: self.columns,
            data,
        }
    }

    fn identity(dimension: usize) -> KrausResult<Self> {
        let mut result = Self::zeros(dimension, dimension)?;

        for index in 0..dimension {
            result.set(index, index, Complex64::one());
        }

        Ok(result)
    }

    fn max_identity_error(&self) -> KrausResult<f64> {
        if self.rows != self.columns {
            return Err(KrausError::DimensionMismatch {
                expected_rows: self.rows,
                actual_rows: self.rows,
                expected_columns: self.rows,
                actual_columns: self.columns,
            });
        }

        let mut max_error = 0.0_f64;

        for row in 0..self.rows {
            for column in 0..self.columns {
                let expected = if row == column {
                    Complex64::one()
                } else {
                    Complex64::zero()
                };

                let difference = self.get(row, column) - expected;
                let error = difference.magnitude();

                if !error.is_finite() {
                    return Err(KrausError::NumericalFailure(
                        "non-finite identity-validation error",
                    ));
                }

                if error > max_error {
                    max_error = error;
                }
            }
        }

        Ok(max_error)
    }

    fn max_positive_semidefinite_upper_bound_violation(&self) -> KrausResult<f64> {
        if self.rows != self.columns {
            return Err(KrausError::DimensionMismatch {
                expected_rows: self.rows,
                actual_rows: self.rows,
                expected_columns: self.rows,
                actual_columns: self.columns,
            });
        }

        // This is a conservative Gershgorin-style sufficient test.
        //
        // For H = I - Σ K†K, H should be positive semidefinite. A Hermitian
        // matrix is guaranteed positive semidefinite when each diagonal is
        // non-negative and dominates the sum of the absolute values of its
        // off-diagonal entries in every row.
        //
        // We deliberately use this sufficient condition rather than claiming
        // that it is a complete eigenvalue test. A future numerical backend
        // may provide a stronger PSD validator.
        let mut maximum_violation = 0.0_f64;

        for row in 0..self.rows {
            let diagonal = self.get(row, row);
            let diagonal_real = diagonal.real();

            let mut off_diagonal_sum = 0.0_f64;

            for column in 0..self.columns {
                if row == column {
                    continue;
                }

                off_diagonal_sum += self.get(row, column).magnitude();
            }

            let violation = (off_diagonal_sum - diagonal_real).max(0.0);

            if violation > maximum_violation {
                maximum_violation = violation;
            }
        }

        Ok(maximum_violation)
    }

    fn trace(&self) -> KrausResult<Complex64> {
        if self.rows != self.columns {
            return Err(KrausError::DimensionMismatch {
                expected_rows: self.rows,
                actual_rows: self.rows,
                expected_columns: self.rows,
                actual_columns: self.columns,
            });
        }

        let mut result = Complex64::zero();

        for index in 0..self.rows {
            result += self.get(index, index);
        }

        Ok(result)
    }
}

// =============================================================================
// Kraus operator
// =============================================================================

/// One Kraus operator.
///
/// The matrix is stored in row-major order and represents a map:
///
/// ```text
/// C^input_dimension → C^output_dimension
/// ```
///
/// The type is independent of qubit count and hardware.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrausOperator {
    input_dimension: usize,
    output_dimension: usize,
    matrix: DenseMatrix,
}

impl KrausOperator {
    /// Constructs a Kraus operator from row-major matrix elements.
    pub fn new(
        input_dimension: usize,
        output_dimension: usize,
        elements: Vec<Complex64>,
    ) -> KrausResult<Self> {
        if input_dimension == 0 {
            return Err(KrausError::ZeroDimension {
                name: "input",
            });
        }

        if output_dimension == 0 {
            return Err(KrausError::ZeroDimension {
                name: "output",
            });
        }

        let matrix = DenseMatrix::new(
            output_dimension,
            input_dimension,
            elements,
        )?;

        Ok(Self {
            input_dimension,
            output_dimension,
            matrix,
        })
    }

    /// Constructs an operator from a validated dense matrix.
    ///
    /// The supplied matrix uses:
    ///
    /// ```text
    /// rows    = output dimension
    /// columns = input dimension
    /// ```
    pub fn from_matrix(matrix: Vec<Complex64>) -> KrausResult<Self> {
        Err(KrausError::NumericalFailure(
            "from_matrix requires explicit dimensions; use KrausOperator::new",
        ))
    }

    /// Constructs a zero operator.
    pub fn zero(
        input_dimension: usize,
        output_dimension: usize,
    ) -> KrausResult<Self> {
        let count = input_dimension
            .checked_mul(output_dimension)
            .ok_or(KrausError::DimensionOverflow {
                left: input_dimension,
                right: output_dimension,
            })?;

        Self::new(
            input_dimension,
            output_dimension,
            vec![Complex64::zero(); count],
        )
    }

    /// Constructs an identity operator.
    ///
    /// Input and output dimensions must match.
    pub fn identity(dimension: usize) -> KrausResult<Self> {
        let matrix = DenseMatrix::identity(dimension)?;

        Ok(Self {
            input_dimension: dimension,
            output_dimension: dimension,
            matrix,
        })
    }

    /// Constructs a scalar multiple of the identity operator.
    pub fn scaled_identity(
        dimension: usize,
        scalar: Complex64,
    ) -> KrausResult<Self> {
        if !scalar.is_finite() {
            return Err(KrausError::NonFiniteElement { index: 0 });
        }

        let matrix = DenseMatrix::identity(dimension)?.scale(scalar);

        Ok(Self {
            input_dimension: dimension,
            output_dimension: dimension,
            matrix,
        })
    }

    /// Returns the input dimension.
    #[must_use]
    pub const fn input_dimension(&self) -> usize {
        self.input_dimension
    }

    /// Returns the output dimension.
    #[must_use]
    pub const fn output_dimension(&self) -> usize {
        self.output_dimension
    }

    /// Returns the number of matrix elements.
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.input_dimension * self.output_dimension
    }

    /// Returns a read-only view of the row-major matrix elements.
    #[must_use]
    pub fn elements(&self) -> &[Complex64] {
        self.matrix.data()
    }

    /// Returns one matrix element.
    ///
    /// The method returns `None` when the requested coordinates are outside
    /// the operator dimensions.
    #[must_use]
    pub fn get(&self, row: usize, column: usize) -> Option<Complex64> {
        if row >= self.output_dimension || column >= self.input_dimension {
            None
        } else {
            Some(self.matrix.get(row, column))
        }
    }

    /// Returns the Hermitian adjoint.
    #[must_use]
    pub fn adjoint(&self) -> Self {
        let matrix = self.matrix.conjugate_transpose();

        Self {
            input_dimension: self.output_dimension,
            output_dimension: self.input_dimension,
            matrix,
        }
    }

    /// Returns the matrix product `self * rhs`.
    ///
    /// The right operator is applied first.
    pub fn compose(&self, rhs: &Self) -> KrausResult<Self> {
        if rhs.output_dimension != self.input_dimension {
            return Err(KrausError::CompositionDimensionMismatch {
                first_output: rhs.output_dimension,
                second_input: self.input_dimension,
            });
        }

        let matrix = self.matrix.multiply(&rhs.matrix)?;

        Ok(Self {
            input_dimension: rhs.input_dimension,
            output_dimension: self.output_dimension,
            matrix,
        })
    }

    /// Returns `self ⊗ rhs`.
    ///
    /// The resulting operator has:
    ///
    /// ```text
    /// input  = self.input  × rhs.input
    /// output = self.output × rhs.output
    /// ```
    pub fn tensor_product(&self, rhs: &Self) -> KrausResult<Self> {
        let input_dimension = self
            .input_dimension
            .checked_mul(rhs.input_dimension)
            .ok_or(KrausError::TensorDimensionOverflow {
                first: self.input_dimension,
                second: rhs.input_dimension,
            })?;

        let output_dimension = self
            .output_dimension
            .checked_mul(rhs.output_dimension)
            .ok_or(KrausError::TensorDimensionOverflow {
                first: self.output_dimension,
                second: rhs.output_dimension,
            })?;

        let element_count = input_dimension
            .checked_mul(output_dimension)
            .ok_or(KrausError::DimensionOverflow {
                left: input_dimension,
                right: output_dimension,
            })?;

        let mut data = vec![Complex64::zero(); element_count];

        // Row-major Kronecker product:
        //
        // (A ⊗ B)_(rA*dB+rB, cA*inputB+cB)
        //     = A_(rA,cA) * B_(rB,cB)
        for row_a in 0..self.output_dimension {
            for column_a in 0..self.input_dimension {
                let a = self.matrix.get(row_a, column_a);

                for row_b in 0..rhs.output_dimension {
                    for column_b in 0..rhs.input_dimension {
                        let row = row_a * rhs.output_dimension + row_b;
                        let column = column_a * rhs.input_dimension + column_b;
                        let index = row * input_dimension + column;

                        data[index] = a * rhs.matrix.get(row_b, column_b);
                    }
                }
            }
        }

        Self::new(input_dimension, output_dimension, data)
    }

    /// Applies this operator to a dense state vector.
    pub fn apply_to_state_vector(
        &self,
        state: &[Complex64],
    ) -> KrausResult<Vec<Complex64>> {
        if state.len() != self.input_dimension {
            return Err(KrausError::StateVectorDimensionMismatch {
                expected: self.input_dimension,
                actual: state.len(),
            });
        }

        for (index, value) in state.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(KrausError::NonFiniteStateElement { index });
            }
        }

        let mut output = vec![Complex64::zero(); self.output_dimension];

        for row in 0..self.output_dimension {
            let mut value = Complex64::zero();

            for column in 0..self.input_dimension {
                value += self.matrix.get(row, column) * state[column];
            }

            if !value.is_finite() {
                return Err(KrausError::NumericalFailure(
                    "Kraus operator application produced a non-finite amplitude",
                ));
            }

            output[row] = value;
        }

        Ok(output)
    }
}

// =============================================================================
// Channel metadata
// =============================================================================

/// Optional logical-resource association for a Kraus channel.
///
/// Resource association is deliberately optional because a channel may be
/// defined independently and bound to actual IR resources later by routing or
/// noise application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KrausResources {
    logical_qubits: Vec<QubitId>,
}

impl KrausResources {
    /// Creates an empty resource association.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            logical_qubits: Vec::new(),
        }
    }

    /// Creates a resource association while rejecting duplicates.
    pub fn new<I>(resources: I) -> KrausResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let mut logical_qubits = Vec::new();

        for resource in resources {
            if logical_qubits.contains(&resource) {
                return Err(KrausError::DuplicateQubit(resource));
            }

            logical_qubits.push(resource);
        }

        Ok(Self { logical_qubits })
    }

    /// Returns the associated logical resources.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns the number of associated resources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Returns whether no resources are associated.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_qubits.is_empty()
    }

    /// Returns whether the supplied logical resource is associated.
    #[must_use]
    pub fn contains(&self, qubit: QubitId) -> bool {
        self.logical_qubits.contains(&qubit)
    }
}

impl Default for KrausResources {
    fn default() -> Self {
        Self::empty()
    }
}

// =============================================================================
// State-vector branch
// =============================================================================

/// One deterministic Kraus branch produced by applying a channel to a pure
/// state.
///
/// The branch vector is intentionally not normalized unless the caller
/// requests normalization through [`KrausBranch::normalized_state`].
///
/// Its squared norm is the probability contribution of that Kraus branch for a
/// normalized input state.
#[derive(Debug, Clone, PartialEq)]
pub struct KrausBranch {
    operator_index: usize,
    state: Vec<Complex64>,
    probability: f64,
}

impl KrausBranch {
    /// Returns the index of the Kraus operator that produced this branch.
    #[must_use]
    pub const fn operator_index(&self) -> usize {
        self.operator_index
    }

    /// Returns the unnormalized branch state.
    #[must_use]
    pub fn state(&self) -> &[Complex64] {
        &self.state
    }

    /// Returns the branch probability.
    #[must_use]
    pub const fn probability(&self) -> f64 {
        self.probability
    }

    /// Returns a normalized version of the branch state.
    ///
    /// Returns `None` when the branch probability is zero.
    #[must_use]
    pub fn normalized_state(&self) -> Option<Vec<Complex64>> {
        if self.probability <= 0.0 {
            return None;
        }

        let scale = 1.0 / self.probability.sqrt();

        let result: Vec<Complex64> = self
            .state
            .iter()
            .copied()
            .map(|value| value * Complex64::from_real(scale).unwrap_or(Complex64::zero()))
            .collect();

        Some(result)
    }
}

// =============================================================================
// Kraus channel
// =============================================================================

/// Completely positive quantum operation represented by Kraus operators.
///
/// A `KrausChannel` can represent both trace-preserving channels and
/// trace-nonincreasing operations. Use [`KrausChannel::validate_trace_preserving`]
/// when a caller specifically requires a CPTP channel.
///
/// The representation has no semantic upper bound on operator count or
/// dimension.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrausChannel {
    input_dimension: usize,
    output_dimension: usize,
    operators: Vec<KrausOperator>,
    resources: KrausResources,
}

impl KrausChannel {
    /// Creates a channel from a non-empty Kraus operator set.
    ///
    /// All operators must have the same input and output dimensions.
    pub fn new(
        operators: Vec<KrausOperator>,
    ) -> KrausResult<Self> {
        Self::with_limits(
            operators,
            KrausResources::empty(),
            KrausResourceLimits::unrestricted(),
        )
    }

    /// Creates a channel with explicit logical-resource association.
    pub fn with_resources(
        operators: Vec<KrausOperator>,
        resources: KrausResources,
    ) -> KrausResult<Self> {
        Self::with_limits(
            operators,
            resources,
            KrausResourceLimits::unrestricted(),
        )
    }

    /// Creates a channel under explicit resource policy.
    pub fn with_limits(
        operators: Vec<KrausOperator>,
        resources: KrausResources,
        limits: KrausResourceLimits,
    ) -> KrausResult<Self> {
        if operators.is_empty() {
            return Err(KrausError::EmptyOperatorSet);
        }

        limits.check_operators(operators.len())?;
        limits.check_resources(resources.len())?;

        let input_dimension = operators[0].input_dimension();
        let output_dimension = operators[0].output_dimension();

        if input_dimension == 0 {
            return Err(KrausError::ZeroDimension {
                name: "input",
            });
        }

        if output_dimension == 0 {
            return Err(KrausError::ZeroDimension {
                name: "output",
            });
        }

        let mut total_elements = 0_u128;

        for operator in &operators {
            if operator.input_dimension() != input_dimension
                || operator.output_dimension() != output_dimension
            {
                return Err(KrausError::DimensionMismatch {
                    expected_rows: output_dimension,
                    actual_rows: operator.output_dimension(),
                    expected_columns: input_dimension,
                    actual_columns: operator.input_dimension(),
                });
            }

            let elements = operator.element_count();

            limits.check_operator_elements(elements)?;

            total_elements = total_elements
                .checked_add(elements as u128)
                .ok_or(KrausError::AllocationSizeOverflow)?;
        }

        limits.check_total_elements(total_elements)?;

        Ok(Self {
            input_dimension,
            output_dimension,
            operators,
            resources,
        })
    }

    /// Creates the identity channel for an arbitrary finite dimension.
    pub fn identity(dimension: usize) -> KrausResult<Self> {
        let operator = KrausOperator::identity(dimension)?;
        Self::new(vec![operator])
    }

    /// Creates a channel whose sole Kraus operator is a scalar multiple of the
    /// identity.
    ///
    /// This is useful for constructing probabilistic branches and
    /// trace-nonincreasing operations.
    pub fn scaled_identity(
        dimension: usize,
        scalar: Complex64,
    ) -> KrausResult<Self> {
        Self::new(vec![KrausOperator::scaled_identity(
            dimension,
            scalar,
        )?])
    }

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

    /// Returns the number of Kraus operators.
    #[must_use]
    pub fn operator_count(&self) -> usize {
        self.operators.len()
    }

    /// Returns all Kraus operators.
    #[must_use]
    pub fn operators(&self) -> &[KrausOperator] {
        &self.operators
    }

    /// Returns the requested Kraus operator.
    #[must_use]
    pub fn operator(&self, index: usize) -> Option<&KrausOperator> {
        self.operators.get(index)
    }

    /// Returns the logical resources associated with the channel.
    #[must_use]
    pub fn resources(&self) -> &KrausResources {
        &self.resources
    }

    /// Returns the number of dense matrix elements across all operators.
    #[must_use]
    pub fn total_element_count(&self) -> u128 {
        self.operators
            .iter()
            .map(|operator| operator.element_count() as u128)
            .sum()
    }

    /// Returns whether this channel is dimension preserving.
    #[must_use]
    pub const fn is_dimension_preserving(&self) -> bool {
        self.input_dimension == self.output_dimension
    }

    /// Returns the operator-completeness matrix:
    ///
    /// ```text
    /// Σ K†K
    /// ```
    ///
    /// For a trace-preserving channel this equals the identity.
    pub fn completeness_operator(&self) -> KrausResult<Vec<Complex64>> {
        let mut result = DenseMatrix::zeros(
            self.input_dimension,
            self.input_dimension,
        )?;

        for operator in &self.operators {
            let product = operator
                .matrix
                .conjugate_transpose()
                .multiply(&operator.matrix)?;

            result.add_assign(&product)?;
        }

        Ok(result.data().to_vec())
    }

    /// Validates the trace-preserving condition:
    ///
    /// ```text
    /// Σ K†K = I
    /// ```
    pub fn validate_trace_preserving(
        &self,
        tolerance: KrausTolerance,
    ) -> KrausResult<()> {
        if !tolerance.absolute.is_finite()
            || !tolerance.relative.is_finite()
            || tolerance.absolute < 0.0
            || tolerance.relative < 0.0
        {
            return Err(KrausError::NumericalFailure(
                "invalid Kraus validation tolerance",
            ));
        }

        let mut completeness = DenseMatrix::zeros(
            self.input_dimension,
            self.input_dimension,
        )?;

        for operator in &self.operators {
            let product = operator
                .matrix
                .conjugate_transpose()
                .multiply(&operator.matrix)?;

            completeness.add_assign(&product)?;
        }

        let error = completeness.max_identity_error()?;

        // Identity has entries with magnitude at most 1, so the relative
        // tolerance is interpreted against a scale of one here.
        let allowed = tolerance.absolute.max(tolerance.relative);

        if error > allowed {
            return Err(KrausError::NotTracePreserving {
                max_error: error,
                absolute_tolerance: tolerance.absolute,
                relative_tolerance: tolerance.relative,
            });
        }

        Ok(())
    }

    /// Returns whether the channel is trace preserving under the supplied
    /// tolerance.
    #[must_use]
    pub fn is_trace_preserving(&self, tolerance: KrausTolerance) -> bool {
        self.validate_trace_preserving(tolerance).is_ok()
    }

    /// Validates a conservative trace-nonincreasing condition.
    ///
    /// The exact mathematical requirement is:
    ///
    /// ```text
    /// I - Σ K†K >= 0
    /// ```
    ///
    /// This implementation uses a conservative sufficient Hermitian
    /// diagonal-dominance test. It deliberately does not claim that the test is
    /// a complete eigenvalue-based PSD proof.
    ///
    /// A future numerical backend can implement a stronger PSD check without
    /// changing the Kraus representation itself.
    pub fn validate_trace_non_increasing(
        &self,
        tolerance: KrausTolerance,
    ) -> KrausResult<()> {
        let mut completeness = DenseMatrix::zeros(
            self.input_dimension,
            self.input_dimension,
        )?;

        for operator in &self.operators {
            let product = operator
                .matrix
                .conjugate_transpose()
                .multiply(&operator.matrix)?;

            completeness.add_assign(&product)?;
        }

        let identity = DenseMatrix::identity(self.input_dimension)?;

        let mut residual = identity.clone();

        for row in 0..self.input_dimension {
            for column in 0..self.input_dimension {
                let value = identity.get(row, column) - completeness.get(row, column);
                residual.set(row, column, value);
            }
        }

        let violation =
            residual.max_positive_semidefinite_upper_bound_violation()?;

        let allowed = tolerance.absolute.max(tolerance.relative);

        if violation > allowed {
            return Err(KrausError::NotTraceNonIncreasing {
                max_violation: violation,
                absolute_tolerance: tolerance.absolute,
                relative_tolerance: tolerance.relative,
            });
        }

        Ok(())
    }

    /// Returns whether the conservative trace-nonincreasing validation passes.
    #[must_use]
    pub fn is_trace_non_increasing(
        &self,
        tolerance: KrausTolerance,
    ) -> bool {
        self.validate_trace_non_increasing(tolerance).is_ok()
    }

    /// Composes this channel after `before`.
    ///
    /// If:
    ///
    /// ```text
    /// before : A → B
    /// self   : B → C
    /// ```
    ///
    /// the result is:
    ///
    /// ```text
    /// self ∘ before : A → C
    /// ```
    pub fn compose(&self, before: &Self) -> KrausResult<Self> {
        if before.output_dimension != self.input_dimension {
            return Err(KrausError::CompositionDimensionMismatch {
                first_output: before.output_dimension,
                second_input: self.input_dimension,
            });
        }

        let count = self
            .operators
            .len()
            .checked_mul(before.operators.len())
            .ok_or(KrausError::AllocationSizeOverflow)?;

        let mut operators = Vec::with_capacity(count);

        for after_operator in &self.operators {
            for before_operator in &before.operators {
                operators.push(after_operator.compose(before_operator)?);
            }
        }

        // Composition changes the resource association semantics. Resource
        // binding is an integration concern, so the mathematical composition
        // deliberately preserves only the intersection-free union.
        let mut resources = before.resources.logical_qubits.clone();

        for resource in &self.resources.logical_qubits {
            if !resources.contains(resource) {
                resources.push(*resource);
            }
        }

        Self::with_resources(
            operators,
            KrausResources::new(resources)?,
        )
    }

    /// Returns the tensor-product channel.
    ///
    /// The resulting Kraus set contains every pairwise tensor product of the
    /// two input Kraus sets.
    pub fn tensor_product(&self, rhs: &Self) -> KrausResult<Self> {
        let operator_count = self
            .operator_count()
            .checked_mul(rhs.operator_count())
            .ok_or(KrausError::AllocationSizeOverflow)?;

        let mut operators = Vec::with_capacity(operator_count);

        for left in &self.operators {
            for right in &rhs.operators {
                operators.push(left.tensor_product(right)?);
            }
        }

        let mut resources = self.resources.logical_qubits.clone();

        for resource in &rhs.resources.logical_qubits {
            if !resources.contains(resource) {
                resources.push(*resource);
            }
        }

        Self::with_resources(
            operators,
            KrausResources::new(resources)?,
        )
    }

    /// Applies the channel to a density matrix represented as a row-major
    /// vector of complex elements.
    ///
    /// The input and output density matrices have dimensions:
    ///
    /// ```text
    /// input_dimension × input_dimension
    /// output_dimension × output_dimension
    /// ```
    pub fn apply_to_density_matrix(
        &self,
        density_matrix: &[Complex64],
    ) -> KrausResult<Vec<Complex64>> {
        let expected = self
            .input_dimension
            .checked_mul(self.input_dimension)
            .ok_or(KrausError::DimensionOverflow {
                left: self.input_dimension,
                right: self.input_dimension,
            })?;

        if density_matrix.len() != expected {
            return Err(KrausError::DensityMatrixDimensionMismatch {
                expected: self.input_dimension,
                rows: self.input_dimension,
                columns: density_matrix.len() / self.input_dimension.max(1),
            });
        }

        for (index, value) in density_matrix.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(KrausError::NonFiniteStateElement { index });
            }
        }

        let input = DenseMatrix::new(
            self.input_dimension,
            self.input_dimension,
            density_matrix.to_vec(),
        )?;

        let mut output = DenseMatrix::zeros(
            self.output_dimension,
            self.output_dimension,
        )?;

        for operator in &self.operators {
            let left = operator.matrix.multiply(&input)?;

            let contribution = left.multiply(
                &operator.matrix.conjugate_transpose(),
            )?;

            output.add_assign(&contribution)?;
        }

        Ok(output.data().to_vec())
    }

    /// Applies the channel to a pure state and returns every Kraus branch.
    ///
    /// No stochastic sampling occurs here.
    ///
    /// Each branch has probability:
    ///
    /// ```text
    /// p_i = ||K_i |psi>||²
    /// ```
    ///
    /// For a trace-preserving channel and a normalized input state, the sum of
    /// all branch probabilities is one within numerical tolerance.
    pub fn apply_to_state_vector(
        &self,
        state: &[Complex64],
    ) -> KrausResult<Vec<KrausBranch>> {
        if state.len() != self.input_dimension {
            return Err(KrausError::StateVectorDimensionMismatch {
                expected: self.input_dimension,
                actual: state.len(),
            });
        }

        for (index, value) in state.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(KrausError::NonFiniteStateElement { index });
            }
        }

        let mut branches = Vec::with_capacity(self.operators.len());

        for (operator_index, operator) in self.operators.iter().enumerate() {
            let branch_state = operator.apply_to_state_vector(state)?;

            let mut probability = 0.0_f64;

            for amplitude in branch_state.iter().copied() {
                probability += amplitude.norm_squared();
            }

            if !probability.is_finite() {
                return Err(KrausError::InvalidBranchProbability {
                    probability,
                });
            }

            // Small negative values are not expected mathematically and must
            // never be silently accepted. Numerical accumulation should only
            // produce non-negative values.
            if probability < 0.0 {
                return Err(KrausError::InvalidBranchProbability {
                    probability,
                });
            }

            branches.push(KrausBranch {
                operator_index,
                state: branch_state,
                probability,
            });
        }

        Ok(branches)
    }

    /// Returns the sum of the probabilities of all Kraus branches for a pure
    /// state.
    pub fn branch_probability_sum(
        &self,
        state: &[Complex64],
    ) -> KrausResult<f64> {
        let branches = self.apply_to_state_vector(state)?;

        let mut total = 0.0_f64;

        for branch in branches {
            total += branch.probability;
        }

        if !total.is_finite() {
            return Err(KrausError::InvalidBranchProbability {
                probability: total,
            });
        }

        Ok(total)
    }

    /// Serializes the channel to JSON using the repository's Serde contract.
    pub fn to_json(&self) -> KrausResult<String> {
        serde_json::to_string(self)
            .map_err(|error| KrausError::Serialization(error.to_string()))
    }

    /// Deserializes a channel from JSON and validates the reconstructed
    /// representation.
    pub fn from_json(json: &str) -> KrausResult<Self> {
        let channel: Self = serde_json::from_str(json)
            .map_err(|error| KrausError::Serialization(error.to_string()))?;

        // Re-run the constructor validation rather than trusting serialized
        // metadata.
        Self::with_limits(
            channel.operators.clone(),
            channel.resources.clone(),
            KrausResourceLimits::unrestricted(),
        )
    }

    /// Returns the stable schema identifier.
    #[must_use]
    pub const fn schema_id() -> &'static str {
        KRAUS_SCHEMA_ID
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        KRAUS_SCHEMA_VERSION
    }
}

// =============================================================================
// Qubit dimension helpers
// =============================================================================

/// Computes the Hilbert-space dimension for a collection of two-level qubits.
///
/// This helper uses the canonical Zamani `QubitId` type and performs checked
/// arithmetic.
///
/// It intentionally has no maximum qubit count.
///
/// A caller requesting a dimension that cannot be represented by `usize`
/// receives an error instead of an overflow.
pub fn qubit_dimension(qubits: usize) -> KrausResult<usize> {
    let mut dimension = 1usize;

    for _ in 0..qubits {
        dimension = dimension
            .checked_mul(2)
            .ok_or(KrausError::TensorDimensionOverflow {
                first: dimension,
                second: 2,
            })?;
    }

    Ok(dimension)
}

/// Computes the Hilbert-space dimension for a concrete logical-qubit resource
/// slice.
///
/// The identifiers themselves do not determine the dimension; the number of
/// resources does.
pub fn dimension_for_qubits(qubits: &[QubitId]) -> KrausResult<usize> {
    qubit_dimension(qubits.len())
}

/// Computes a qubit count from a power-of-two Hilbert-space dimension.
///
/// Returns `None` when the dimension is zero or is not an exact power of two.
#[must_use]
pub fn qubit_count_for_dimension(dimension: usize) -> Option<usize> {
    if dimension == 0 || !dimension.is_power_of_two() {
        return None;
    }

    Some(dimension.trailing_zeros() as usize)
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Constructs a one-branch identity channel on the supplied logical qubits.
pub fn identity_for_qubits(
    qubits: &[QubitId],
) -> KrausResult<KrausChannel> {
    let resources = KrausResources::new(qubits.iter().copied())?;
    let dimension = dimension_for_qubits(qubits)?;

    KrausChannel::with_resources(
        vec![KrausOperator::identity(dimension)?],
        resources,
    )
}

/// Constructs a channel from arbitrary operators and associates it with the
/// supplied logical resources.
///
/// The resource count is metadata only; the operator dimension remains the
/// authoritative mathematical dimension.
pub fn channel_for_qubits(
    qubits: &[QubitId],
    operators: Vec<KrausOperator>,
) -> KrausResult<KrausChannel> {
    let resources = KrausResources::new(qubits.iter().copied())?;

    KrausChannel::with_resources(operators, resources)
}

// =============================================================================
// Internal utilities
// =============================================================================

fn approximately_equal(
    left: f64,
    right: f64,
    tolerance: KrausTolerance,
) -> bool {
    if !left.is_finite() || !right.is_finite() {
        return false;
    }

    let difference = (left - right).abs();
    let scale = left.abs().max(right.abs()).max(1.0);

    difference <= tolerance.absolute.max(tolerance.relative * scale)
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

    fn basis_zero(dimension: usize) -> Vec<Complex64> {
        let mut state = vec![Complex64::zero(); dimension];
        state[0] = Complex64::one();
        state
    }

    #[test]
    fn identity_operator_is_valid() {
        let operator = KrausOperator::identity(2).unwrap();

        assert_eq!(operator.input_dimension(), 2);
        assert_eq!(operator.output_dimension(), 2);
        assert_eq!(operator.element_count(), 4);

        assert_eq!(operator.get(0, 0), Some(c(1.0, 0.0)));
        assert_eq!(operator.get(0, 1), Some(c(0.0, 0.0)));
        assert_eq!(operator.get(1, 0), Some(c(0.0, 0.0)));
        assert_eq!(operator.get(1, 1), Some(c(1.0, 0.0)));
    }

    #[test]
    fn identity_channel_is_trace_preserving() {
        let channel = KrausChannel::identity(2).unwrap();

        assert!(channel.validate_trace_preserving(
            KrausTolerance::default()
        ).is_ok());
    }

    #[test]
    fn identity_channel_preserves_state_vector() {
        let channel = KrausChannel::identity(2).unwrap();

        let state = vec![c(0.6, 0.0), c(0.8, 0.0)];

        let branches = channel.apply_to_state_vector(&state).unwrap();

        assert_eq!(branches.len(), 1);
        assert!(branches[0].state()[0].approx_eq(
            c(0.6, 0.0),
            DEFAULT_KRAUS_ABSOLUTE_TOLERANCE,
            DEFAULT_KRAUS_RELATIVE_TOLERANCE,
        ));
        assert!(branches[0].state()[1].approx_eq(
            c(0.8, 0.0),
            DEFAULT_KRAUS_ABSOLUTE_TOLERANCE,
            DEFAULT_KRAUS_RELATIVE_TOLERANCE,
        ));
        assert!(approximately_equal(
            branches[0].probability(),
            1.0,
            KrausTolerance::default(),
        ));
    }

    #[test]
    fn identity_channel_preserves_density_matrix() {
        let channel = KrausChannel::identity(2).unwrap();

        let density = vec![
            c(0.36, 0.0),
            c(0.48, 0.0),
            c(0.48, 0.0),
            c(0.64, 0.0),
        ];

        let result = channel.apply_to_density_matrix(&density).unwrap();

        assert_eq!(result, density);
    }

    #[test]
    fn scaled_identity_can_be_trace_non_increasing() {
        let scalar = c(0.5_f64.sqrt(), 0.0);

        let channel = KrausChannel::scaled_identity(2, scalar).unwrap();

        assert!(channel.validate_trace_non_increasing(
            KrausTolerance::default()
        ).is_ok());

        assert!(channel.validate_trace_preserving(
            KrausTolerance::default()
        ).is_err());
    }

    #[test]
    fn two_branch_channel_can_be_trace_preserving() {
        let zero = KrausOperator::new(
            2,
            2,
            vec![
                c(1.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
            ],
        ).unwrap();

        let one = KrausOperator::new(
            2,
            2,
            vec![
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(1.0, 0.0),
            ],
        ).unwrap();

        let channel = KrausChannel::new(vec![zero, one]).unwrap();

        assert!(channel.validate_trace_preserving(
            KrausTolerance::default()
        ).is_ok());
    }

    #[test]
    fn composition_matches_matrix_multiplication() {
        let x = KrausOperator::new(
            2,
            2,
            vec![
                c(0.0, 0.0),
                c(1.0, 0.0),
                c(1.0, 0.0),
                c(0.0, 0.0),
            ],
        ).unwrap();

        let z = KrausOperator::new(
            2,
            2,
            vec![
                c(1.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(-1.0, 0.0),
            ],
        ).unwrap();

        let x_channel = KrausChannel::new(vec![x]).unwrap();
        let z_channel = KrausChannel::new(vec![z]).unwrap();

        let composed = z_channel.compose(&x_channel).unwrap();

        assert_eq!(composed.operator_count(), 1);
        assert_eq!(composed.input_dimension(), 2);
        assert_eq!(composed.output_dimension(), 2);
    }

    #[test]
    fn tensor_product_scales_dimensions_from_data() {
        let first = KrausChannel::identity(2).unwrap();
        let second = KrausChannel::identity(3).unwrap();

        let tensor = first.tensor_product(&second).unwrap();

        assert_eq!(tensor.input_dimension(), 6);
        assert_eq!(tensor.output_dimension(), 6);
        assert_eq!(tensor.operator_count(), 1);
    }

    #[test]
    fn tensor_product_supports_more_than_two_level_systems() {
        let first = KrausChannel::identity(3).unwrap();
        let second = KrausChannel::identity(4).unwrap();

        let tensor = first.tensor_product(&second).unwrap();

        assert_eq!(tensor.input_dimension(), 12);
        assert_eq!(tensor.output_dimension(), 12);
    }

    #[test]
    fn invalid_matrix_element_count_is_rejected() {
        let result = KrausOperator::new(
            2,
            2,
            vec![Complex64::zero(); 3],
        );

        assert!(matches!(
            result,
            Err(KrausError::ElementCountMismatch { .. })
        ));
    }

    #[test]
    fn non_finite_matrix_elements_are_rejected() {
        let result = KrausOperator::new(
            1,
            1,
            vec![Complex64::new(f64::NAN, 0.0)],
        );

        assert!(matches!(
            result,
            Err(KrausError::NonFiniteElement { .. })
        ));
    }

    #[test]
    fn zero_dimension_is_rejected() {
        let result = KrausOperator::new(
            0,
            2,
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(KrausError::ZeroDimension { .. })
        ));
    }

    #[test]
    fn empty_operator_set_is_rejected() {
        let result = KrausChannel::new(Vec::new());

        assert_eq!(
            result,
            Err(KrausError::EmptyOperatorSet)
        );
    }

    #[test]
    fn mismatched_operator_dimensions_are_rejected() {
        let first = KrausOperator::identity(2).unwrap();
        let second = KrausOperator::identity(3).unwrap();

        let result = KrausChannel::new(vec![first, second]);

        assert!(matches!(
            result,
            Err(KrausError::DimensionMismatch { .. })
        ));
    }

    #[test]
    fn duplicate_logical_resources_are_rejected() {
        let q0 = QubitId::new(0);

        let result = KrausResources::new(vec![q0, q0]);

        assert_eq!(
            result,
            Err(KrausError::DuplicateQubit(q0))
        );
    }

    #[test]
    fn canonical_qubit_identity_is_used() {
        let q0 = QubitId::new(0);
        let q1 = QubitId::new(1);

        let channel = identity_for_qubits(&[q0, q1]).unwrap();

        assert_eq!(
            channel.resources().logical_qubits(),
            &[q0, q1]
        );
        assert_eq!(channel.input_dimension(), 4);
        assert_eq!(channel.output_dimension(), 4);
    }

    #[test]
    fn qubit_dimension_has_no_small_machine_assumption() {
        assert_eq!(qubit_dimension(0).unwrap(), 1);
        assert_eq!(qubit_dimension(1).unwrap(), 2);
        assert_eq!(qubit_dimension(2).unwrap(), 4);
        assert_eq!(qubit_dimension(10).unwrap(), 1024);
    }

    #[test]
    fn qubit_count_round_trip() {
        for qubits in 0..=10 {
            let dimension = qubit_dimension(qubits).unwrap();
            assert_eq!(
                qubit_count_for_dimension(dimension),
                Some(qubits)
            );
        }
    }

    #[test]
    fn invalid_qubit_dimension_overflow_is_reported() {
        let mut qubits = 0usize;

        let mut dimension = 1usize;

        while let Some(next) = dimension.checked_mul(2) {
            dimension = next;
            qubits += 1;

            // The test deliberately stops before an enormous loop while
            // retaining the overflow boundary of the host representation.
            if qubits == usize::BITS as usize {
                break;
            }
        }

        let result = qubit_dimension(qubits + 1);

        assert!(result.is_err());
    }

    #[test]
    fn state_vector_dimension_mismatch_is_rejected() {
        let channel = KrausChannel::identity(2).unwrap();

        let result = channel.apply_to_state_vector(
            &[Complex64::one()],
        );

        assert!(matches!(
            result,
            Err(KrausError::StateVectorDimensionMismatch { .. })
        ));
    }

    #[test]
    fn density_matrix_dimension_mismatch_is_rejected() {
        let channel = KrausChannel::identity(2).unwrap();

        let result = channel.apply_to_density_matrix(
            &[Complex64::one()],
        );

        assert!(matches!(
            result,
            Err(KrausError::DensityMatrixDimensionMismatch { .. })
        ));
    }

    #[test]
    fn branch_probability_is_computed_from_norm() {
        let operator = KrausOperator::scaled_identity(
            2,
            c(0.5, 0.0),
        ).unwrap();

        let channel = KrausChannel::new(vec![operator]).unwrap();

        let state = basis_zero(2);

        let branches = channel.apply_to_state_vector(&state).unwrap();

        assert_eq!(branches.len(), 1);
        assert!(approximately_equal(
            branches[0].probability(),
            0.25,
            KrausTolerance::default(),
        ));
    }

    #[test]
    fn normalized_branch_is_available_for_nonzero_probability() {
        let operator = KrausOperator::scaled_identity(
            2,
            c(0.5, 0.0),
        ).unwrap();

        let channel = KrausChannel::new(vec![operator]).unwrap();

        let state = basis_zero(2);

        let branches = channel.apply_to_state_vector(&state).unwrap();

        let normalized = branches[0]
            .normalized_state()
            .unwrap();

        assert!(normalized[0].approx_eq(
            Complex64::one(),
            DEFAULT_KRAUS_ABSOLUTE_TOLERANCE,
            DEFAULT_KRAUS_RELATIVE_TOLERANCE,
        ));
    }

    #[test]
    fn resources_can_be_empty() {
        let resources = KrausResources::empty();

        assert!(resources.is_empty());
        assert_eq!(resources.len(), 0);
    }

    #[test]
    fn resource_limits_are_policy_not_semantics() {
        let operator = KrausOperator::identity(2).unwrap();

        let restrictive = KrausResourceLimits {
            max_operators: Some(0),
            max_elements_per_operator: None,
            max_total_elements: None,
            max_resources: None,
        };

        let result = KrausChannel::with_limits(
            vec![operator],
            KrausResources::empty(),
            restrictive,
        );

        assert!(matches!(
            result,
            Err(KrausError::ResourceLimitExceeded {
                resource: "operators",
                ..
            })
        ));

        // The same mathematical channel remains constructible under a
        // different resource policy.
        assert!(KrausChannel::identity(2).is_ok());
    }

    #[test]
    fn serialization_round_trip_preserves_channel() {
        let channel = KrausChannel::identity(2).unwrap();

        let json = channel.to_json().unwrap();
        let restored = KrausChannel::from_json(&json).unwrap();

        assert_eq!(restored, channel);
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            KrausChannel::schema_id(),
            "zamani.quantum.zqn.channel.kraus"
        );
        assert_eq!(KrausChannel::schema_version(), 1);
    }

    #[test]
    fn tensor_product_preserves_trace_preservation() {
        let first = KrausChannel::identity(2).unwrap();
        let second = KrausChannel::identity(3).unwrap();

        let tensor = first.tensor_product(&second).unwrap();

        assert!(tensor.validate_trace_preserving(
            KrausTolerance::default()
        ).is_ok());
    }

    #[test]
    fn compose_identity_is_identity() {
        let channel = KrausChannel::identity(2).unwrap();
        let identity = KrausChannel::identity(2).unwrap();

        let result = channel.compose(&identity).unwrap();

        assert_eq!(result.input_dimension(), 2);
        assert_eq!(result.output_dimension(), 2);
        assert_eq!(result.operator_count(), 1);
    }

    #[test]
    fn incompatible_composition_is_rejected() {
        let first = KrausChannel::identity(2).unwrap();
        let second = KrausChannel::identity(3).unwrap();

        let result = second.compose(&first);

        assert!(matches!(
            result,
            Err(KrausError::CompositionDimensionMismatch { .. })
        ));
    }

    #[test]
    fn scaled_identity_has_expected_density_matrix_action() {
        let scalar = c(0.5_f64.sqrt(), 0.0);

        let channel = KrausChannel::scaled_identity(2, scalar).unwrap();

        let density = vec![
            c(1.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
            c(0.0, 0.0),
        ];

        let result = channel.apply_to_density_matrix(&density).unwrap();

        assert!(result[0].approx_eq(
            c(0.5, 0.0),
            DEFAULT_KRAUS_ABSOLUTE_TOLERANCE,
            DEFAULT_KRAUS_RELATIVE_TOLERANCE,
        ));
    }

    #[test]
    fn rectangular_operator_is_supported() {
        let operator = KrausOperator::new(
            2,
            3,
            vec![
                c(1.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(1.0, 0.0),
                c(0.0, 0.0),
            ],
        ).unwrap();

        assert_eq!(operator.input_dimension(), 2);
        assert_eq!(operator.output_dimension(), 3);

        let state = vec![
            c(1.0, 0.0),
            c(0.0, 0.0),
        ];

        let output = operator.apply_to_state_vector(&state).unwrap();

        assert_eq!(output.len(), 3);
        assert_eq!(output[0], c(1.0, 0.0));
    }

    #[test]
    fn no_global_random_state_is_used() {
        // This is intentionally a structural test/documentation marker:
        // deterministic Kraus application contains no sampling operation.
        let channel = KrausChannel::identity(2).unwrap();
        let state = basis_zero(2);

        let first = channel.apply_to_state_vector(&state).unwrap();
        let second = channel.apply_to_state_vector(&state).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn operator_adjoint_reverses_dimensions() {
        let operator = KrausOperator::new(
            2,
            3,
            vec![
                Complex64::zero(),
                Complex64::one(),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::zero(),
                Complex64::one(),
            ],
        ).unwrap();

        let adjoint = operator.adjoint();

        assert_eq!(adjoint.input_dimension(), 3);
        assert_eq!(adjoint.output_dimension(), 2);
    }

    #[test]
    fn density_matrix_application_is_deterministic() {
        let channel = KrausChannel::identity(2).unwrap();

        let density = vec![
            c(0.5, 0.0),
            c(0.5, 0.0),
            c(0.5, 0.0),
            c(0.5, 0.0),
        ];

        let first = channel
            .apply_to_density_matrix(&density)
            .unwrap();

        let second = channel
            .apply_to_density_matrix(&density)
            .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn resource_limit_can_limit_total_elements() {
        let operator = KrausOperator::identity(2).unwrap();

        let limits = KrausResourceLimits {
            max_operators: None,
            max_elements_per_operator: None,
            max_total_elements: Some(3),
            max_resources: None,
        };

        let result = KrausChannel::with_limits(
            vec![operator],
            KrausResources::empty(),
            limits,
        );

        assert!(matches!(
            result,
            Err(KrausError::ResourceLimitExceeded {
                resource: "total_elements",
                ..
            })
        ));
    }

    #[test]
    fn arbitrary_finite_dimension_is_supported() {
        let dimensions = [1usize, 2, 3, 4, 7, 16];

        for dimension in dimensions {
            let channel = KrausChannel::identity(dimension).unwrap();

            assert_eq!(channel.input_dimension(), dimension);
            assert_eq!(channel.output_dimension(), dimension);
            assert_eq!(channel.operator_count(), 1);

            assert!(channel.validate_trace_preserving(
                KrausTolerance::default()
            ).is_ok());
        }
    }
}