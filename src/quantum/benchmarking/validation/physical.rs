//! Zamani Quantum Benchmarking — Physical-Domain Validation.
//!
//! # Purpose
//!
//! This module validates physical quantities used by the quantum benchmarking
//! subsystem before they are accepted by metrics, protocols, analysis,
//! reporting, QEC benchmarking, or hardware integration.
//!
//! This module is deliberately a PURE VALIDATION LAYER.
//!
//! It does NOT:
//!
//! - execute quantum circuits;
//! - submit jobs to hardware;
//! - access QPU credentials;
//! - mutate calibration data;
//! - infer hardware capabilities;
//! - calculate benchmark scores;
//! - calculate statistical confidence intervals;
//! - fit randomized-benchmarking curves;
//! - classify raw quantum states;
//! - store raw measurement/syndrome payloads;
//! - perform network I/O;
//! - perform filesystem I/O;
//! - depend on a particular quantum technology.
//!
//! # Architectural position
//!
//! ```text
//!                         Quantum execution
//!                                │
//!                                ▼
//!                         Raw observations
//!                                │
//!                                ▼
//!                    ┌──────────────────────┐
//!                    │ validation::physical │
//!                    └──────────┬───────────┘
//!                               │
//!                  physically valid quantities
//!                               │
//!              ┌────────────────┼────────────────┐
//!              ▼                ▼                ▼
//!           metrics          protocols          QEC
//!              │                │                │
//!              └────────────────┼────────────────┘
//!                               ▼
//!                         BenchmarkResult
//! ```
//!
//! # Design principles
//!
//! ## 1. Physical validity is not statistical validity
//!
//! This module answers questions such as:
//!
//! - Is a probability in [0, 1]?
//! - Is an error rate in [0, 1]?
//! - Is fidelity in [0, 1]?
//! - Is a count relationship physically possible?
//! - Is a duration non-negative and finite?
//! - Is a qubit index within the declared device size?
//! - Is a two-qubit operation actually supplied with two distinct qubits?
//! - Is a matrix a valid stochastic/readout matrix?
//! - Is a density-matrix trace physically plausible?
//! - Is a physical/logical resource relationship coherent?
//!
//! It does NOT answer:
//!
//! - Is the estimate statistically significant?
//! - Is a confidence interval sufficiently narrow?
//! - Is an RB fit good?
//! - Did a benchmark pass its protocol threshold?
//!
//! Those belong to `statistics`, `protocols`, and higher-level validation.
//!
//! ## 2. Never silently repair invalid scientific data
//!
//! This module never:
//!
//! - clamps `1.000001` to `1.0`;
//! - turns `NaN` into zero;
//! - turns a negative duration into zero;
//! - normalizes an invalid probability vector;
//! - silently discards impossible observations.
//!
//! Invalid physical data is rejected explicitly.
//!
//! ## 3. Numerical tolerance is explicit
//!
//! Floating-point calculations can produce values infinitesimally outside a
//! mathematical boundary. Therefore validation supports an explicit tolerance.
//!
//! The tolerance is used only for floating-point boundary comparisons. The
//! original value is never modified.
//!
//! ## 4. No hidden global state
//!
//! Validation is deterministic. It depends only on the supplied arguments.
//!
//! ## 5. Bounded validation
//!
//! Matrix and vector validation has explicit element-count limits so malformed
//! external data cannot cause unbounded validation work.
//!
//! ## 6. Technology neutrality
//!
//! The validator does not assume:
//!
//! - superconducting qubits;
//! - trapped ions;
//! - neutral atoms;
//! - photonics;
//! - spin qubits;
//! - annealing;
//! - analog quantum systems;
//! - a particular simulator.
//!
//! Technology-specific rules belong in hardware capability/calibration
//! validation.
//!
//! # Integration contract
//!
//! This module is intended to be consumed by:
//!
//! - `validation/input.rs`
//! - `validation/statistical.rs`
//! - `validation/reproducibility.rs`
//! - `metrics/probability.rs`
//! - `metrics/fidelity.rs`
//! - `metrics/readout.rs`
//! - `metrics/leakage.rs`
//! - `metrics/logical.rs`
//! - `metrics/resource.rs`
//! - `qec/physical.rs`
//! - `qec/logical.rs`
//! - `qec/resource_overhead.rs`
//! - protocol implementations
//!
//! The dependency direction is:
//!
//! ```text
//! validation::physical
//!        │
//!        ├── metrics
//!        ├── protocols
//!        └── qec
//! ```
//!
//! Never introduce:
//!
//! ```text
//! validation::physical → protocol implementation
//! validation::physical → hardware implementation
//! validation::physical → runtime
//! ```
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! No nightly features are required.
//!
//! The implementation intentionally uses stable standard-library APIs and
//! `serde`, which is already part of the Zamani benchmarking architecture.
//!
//! # Serialization
//!
//! Validation configuration and errors are serializable so that a failed
//! benchmark can preserve the exact validation policy that rejected it.
//!
//! Validation itself does not serialize executable state.
//!
//! # Security
//!
//! This module is designed to be safe against malformed benchmark input:
//!
//! - finite-value checks happen before arithmetic;
//! - division-by-zero is rejected;
//! - matrix dimensions are bounded;
//! - qubit indices are bounds-checked;
//! - count relationships are checked before subtraction;
//! - vector lengths are checked before indexing;
//! - no user-supplied allocation size is trusted without a bound.
//!
//! This is a validation library, not a sandbox. It therefore does not attempt
//! to protect against memory corruption from unsafe code elsewhere in the
//! application.
//!
//! # Scientific scope
//!
//! The validator covers common physical-domain quantities:
//!
//! - probabilities;
//! - error rates;
//! - fidelities;
//! - counts;
//! - shots;
//! - gate counts;
//! - qubit counts;
//! - depths;
//! - durations;
//! - frequencies;
//! - energies;
//! - readout matrices;
//! - stochastic matrices;
//! - density-matrix trace constraints;
//! - Pauli probability distributions;
//! - leakage/erasure rates;
//! - physical/logical resource relationships.
//!
//! It intentionally does not claim that a scalar value alone proves that a
//! quantum experiment is physically correct. Context is required.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use serde::{Deserialize, Serialize};

// ============================================================================
// Constants
// ============================================================================

/// Stable validation component identifier.
pub const PHYSICAL_VALIDATION_ID: &str = "quantum.benchmarking.validation.physical";

/// Current validation policy version.
///
/// Increment when the accepted/rejected domain changes.
pub const PHYSICAL_VALIDATION_VERSION: u32 = 1;

/// Default floating-point boundary tolerance.
///
/// This is deliberately small. Callers performing high-precision scientific
/// work should provide their own explicit tolerance.
pub const DEFAULT_TOLERANCE: f64 = 1.0e-12;

/// Minimum tolerance accepted by the validator.
pub const MIN_TOLERANCE: f64 = 0.0;

/// Maximum tolerance accepted by the validator.
///
/// A tolerance larger than this can conceal physically meaningful errors.
pub const MAX_TOLERANCE: f64 = 1.0e-6;

/// Maximum number of elements inspected by matrix/vector validators.
///
/// This protects the validation layer from pathological external input.
pub const DEFAULT_MAX_ELEMENTS: usize = 16_777_216;

/// Maximum number of dimensions accepted for a square matrix.
///
/// This is a validation safety limit, not a hardware limit.
pub const DEFAULT_MAX_MATRIX_DIMENSION: usize = 4096;

/// Maximum UTF-8 byte length accepted for a validation field label.
pub const MAX_FIELD_NAME_BYTES: usize = 128;

// ============================================================================
// Result types
// ============================================================================

/// Result returned by physical validation operations.
pub type PhysicalValidationResult<T> = Result<T, PhysicalValidationError>;

// ============================================================================
// Validation policy
// ============================================================================

/// Explicit physical-validation policy.
///
/// A policy is passed into validation instead of relying on global mutable
/// state. This makes benchmark validation deterministic and reproducible.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicalValidationPolicy {
    /// Floating-point boundary tolerance.
    pub tolerance: f64,

    /// Maximum number of elements a vector/matrix validator may inspect.
    pub max_elements: usize,

    /// Maximum square matrix dimension.
    pub max_matrix_dimension: usize,

    /// Whether zero-opportunity rates are rejected.
    ///
    /// A rate with zero opportunities is undefined and therefore should not be
    /// accepted as a physical measurement.
    pub reject_zero_opportunities: bool,

    /// Whether empty probability distributions are rejected.
    pub reject_empty_distributions: bool,
}

impl Default for PhysicalValidationPolicy {
    fn default() -> Self {
        Self {
            tolerance: DEFAULT_TOLERANCE,
            max_elements: DEFAULT_MAX_ELEMENTS,
            max_matrix_dimension: DEFAULT_MAX_MATRIX_DIMENSION,
            reject_zero_opportunities: true,
            reject_empty_distributions: true,
        }
    }
}

impl PhysicalValidationPolicy {
    /// Creates and validates a policy.
    pub fn new(
        tolerance: f64,
        max_elements: usize,
        max_matrix_dimension: usize,
    ) -> PhysicalValidationResult<Self> {
        validate_tolerance(tolerance)?;

        if max_elements == 0 {
            return Err(PhysicalValidationError::InvalidConfiguration {
                field: "max_elements",
                reason: "must be greater than zero",
            });
        }

        if max_matrix_dimension == 0 {
            return Err(PhysicalValidationError::InvalidConfiguration {
                field: "max_matrix_dimension",
                reason: "must be greater than zero",
            });
        }

        let squared = max_matrix_dimension
            .checked_mul(max_matrix_dimension)
            .ok_or(PhysicalValidationError::LimitOverflow {
                field: "max_matrix_dimension",
            })?;

        if squared > max_elements {
            return Err(PhysicalValidationError::InvalidConfiguration {
                field: "max_matrix_dimension",
                reason: "square matrix exceeds max_elements",
            });
        }

        Ok(Self {
            tolerance,
            max_elements,
            max_matrix_dimension,
            ..Self::default()
        })
    }

    /// Returns whether a floating-point value is within [0, 1] under this
    /// policy's tolerance.
    pub fn is_unit_interval(&self, value: f64) -> bool {
        is_unit_interval(value, self.tolerance)
    }

    /// Returns whether a floating-point value is approximately zero.
    pub fn is_zero(&self, value: f64) -> bool {
        value.is_finite() && value.abs() <= self.tolerance
    }

    /// Returns whether a floating-point value is approximately one.
    pub fn is_one(&self, value: f64) -> bool {
        value.is_finite() && (value - 1.0).abs() <= self.tolerance
    }
}

// ============================================================================
// Error model
// ============================================================================

/// Error returned when a physical-domain value is invalid.
///
/// This type intentionally lives in this module so that the file can be
/// completed independently without depending on a not-yet-finalized global
/// benchmarking error enum.
///
/// `validation::input` or `core::errors` may later convert this error into the
/// repository-wide error envelope without changing the validation API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicalValidationError {
    /// A required value was not finite.
    NonFinite {
        field: &'static str,
    },

    /// A value was outside an allowed numeric range.
    OutOfRange {
        field: &'static str,
        value_bits: u64,
        minimum_bits: u64,
        maximum_bits: u64,
    },

    /// A value was negative when it must be non-negative.
    Negative {
        field: &'static str,
        value_bits: u64,
    },

    /// A count relationship was invalid.
    InvalidCountRelationship {
        field: &'static str,
        numerator: u64,
        denominator: u64,
    },

    /// A denominator/opportunity count was zero.
    ZeroOpportunities {
        field: &'static str,
    },

    /// A vector was empty.
    EmptyDistribution {
        field: &'static str,
    },

    /// Vector/matrix length was invalid.
    InvalidLength {
        field: &'static str,
        actual: usize,
        expected: usize,
    },

    /// A matrix was not square.
    NonSquareMatrix {
        field: &'static str,
        rows: usize,
        columns: usize,
    },

    /// A matrix/vector was too large to validate safely.
    ValidationLimitExceeded {
        field: &'static str,
        requested: usize,
        maximum: usize,
    },

    /// A matrix row did not satisfy a stochastic constraint.
    InvalidStochasticRow {
        field: &'static str,
        row: usize,
        sum_bits: u64,
    },

    /// A matrix contained an invalid element.
    InvalidMatrixElement {
        field: &'static str,
        row: usize,
        column: usize,
        value_bits: u64,
    },

    /// A qubit index was outside the declared range.
    QubitIndexOutOfRange {
        field: &'static str,
        index: u64,
        qubit_count: u64,
    },

    /// Two-qubit operation was supplied with the same qubit twice.
    DuplicateQubit {
        field: &'static str,
        qubit: u64,
    },

    /// A resource relationship is physically inconsistent.
    InvalidResourceRelationship {
        field: &'static str,
        reason: &'static str,
    },

    /// Configuration itself was invalid.
    InvalidConfiguration {
        field: &'static str,
        reason: &'static str,
    },

    /// An arithmetic bound would overflow.
    LimitOverflow {
        field: &'static str,
    },

    /// The validator received an invalid field name.
    InvalidFieldName,

    /// A vector contained a negative/invalid probability.
    InvalidProbabilityDistribution {
        field: &'static str,
        index: usize,
        value_bits: u64,
    },

    /// A probability distribution did not sum to one.
    DistributionNotNormalized {
        field: &'static str,
        sum_bits: u64,
    },

    /// A density-matrix trace was invalid.
    InvalidDensityMatrixTrace {
        field: &'static str,
        trace_real_bits: u64,
        trace_imag_bits: u64,
    },
}

impl fmt::Display for PhysicalValidationError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NonFinite { field } => {
                write!(f, "physical validation failed: `{field}` is not finite")
            }

            Self::OutOfRange {
                field,
                value_bits,
                minimum_bits,
                maximum_bits,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}`={} is outside [{}, {}]",
                    f64::from_bits(*value_bits),
                    f64::from_bits(*minimum_bits),
                    f64::from_bits(*maximum_bits),
                )
            }

            Self::Negative {
                field,
                value_bits,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}`={} is negative",
                    f64::from_bits(*value_bits),
                )
            }

            Self::InvalidCountRelationship {
                field,
                numerator,
                denominator,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}` has numerator {} greater than denominator {}",
                    numerator,
                    denominator
                )
            }

            Self::ZeroOpportunities { field } => {
                write!(
                    f,
                    "physical validation failed: `{field}` has zero opportunities"
                )
            }

            Self::EmptyDistribution { field } => {
                write!(
                    f,
                    "physical validation failed: `{field}` is empty"
                )
            }

            Self::InvalidLength {
                field,
                actual,
                expected,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}` has length {}, expected {}",
                    actual,
                    expected
                )
            }

            Self::NonSquareMatrix {
                field,
                rows,
                columns,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}` is not square: {}x{}",
                    rows,
                    columns
                )
            }

            Self::ValidationLimitExceeded {
                field,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "physical validation limit exceeded for `{field}`: {} > {}",
                    requested,
                    maximum
                )
            }

            Self::InvalidStochasticRow {
                field,
                row,
                sum_bits,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}` row {} sums to {} instead of one",
                    row,
                    f64::from_bits(*sum_bits)
                )
            }

            Self::InvalidMatrixElement {
                field,
                row,
                column,
                value_bits,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}[{}][{}]` = {} is invalid",
                    row,
                    column,
                    f64::from_bits(*value_bits)
                )
            }

            Self::QubitIndexOutOfRange {
                field,
                index,
                qubit_count,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}` qubit index {} is outside 0..{}",
                    index,
                    qubit_count
                )
            }

            Self::DuplicateQubit { field, qubit } => {
                write!(
                    f,
                    "physical validation failed: `{field}` uses qubit {} more than once",
                    qubit
                )
            }

            Self::InvalidResourceRelationship { field, reason } => {
                write!(
                    f,
                    "physical validation failed for `{field}`: {reason}"
                )
            }

            Self::InvalidConfiguration { field, reason } => {
                write!(
                    f,
                    "invalid physical validation configuration `{field}`: {reason}"
                )
            }

            Self::LimitOverflow { field } => {
                write!(
                    f,
                    "physical validation limit overflow for `{field}`"
                )
            }

            Self::InvalidFieldName => {
                f.write_str("invalid physical validation field name")
            }

            Self::InvalidProbabilityDistribution {
                field,
                index,
                value_bits,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}[{}]` = {} is not a probability",
                    index,
                    f64::from_bits(*value_bits)
                )
            }

            Self::DistributionNotNormalized {
                field,
                sum_bits,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}` sums to {} instead of one",
                    f64::from_bits(*sum_bits)
                )
            }

            Self::InvalidDensityMatrixTrace {
                field,
                trace_real_bits,
                trace_imag_bits,
            } => {
                write!(
                    f,
                    "physical validation failed: `{field}` has invalid trace {} + {}i",
                    f64::from_bits(*trace_real_bits),
                    f64::from_bits(*trace_imag_bits),
                )
            }
        }
    }
}

impl std::error::Error for PhysicalValidationError {}

// ============================================================================
// Scalar validation
// ============================================================================

/// Validates the supplied tolerance.
pub fn validate_tolerance(
    tolerance: f64,
) -> PhysicalValidationResult<()> {
    if !tolerance.is_finite() {
        return Err(PhysicalValidationError::NonFinite {
            field: "tolerance",
        });
    }

    if !(MIN_TOLERANCE..=MAX_TOLERANCE).contains(&tolerance) {
        return Err(PhysicalValidationError::OutOfRange {
            field: "tolerance",
            value_bits: tolerance.to_bits(),
            minimum_bits: MIN_TOLERANCE.to_bits(),
            maximum_bits: MAX_TOLERANCE.to_bits(),
        });
    }

    Ok(())
}

/// Validates a finite scalar.
pub fn validate_finite(
    field: &'static str,
    value: f64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    if !value.is_finite() {
        return Err(PhysicalValidationError::NonFinite { field });
    }

    Ok(())
}

/// Validates a non-negative finite scalar.
pub fn validate_non_negative(
    field: &'static str,
    value: f64,
) -> PhysicalValidationResult<()> {
    validate_finite(field, value)?;

    if value < 0.0 {
        return Err(PhysicalValidationError::Negative {
            field,
            value_bits: value.to_bits(),
        });
    }

    Ok(())
}

/// Validates a scalar within an inclusive range, allowing the supplied
/// tolerance at the boundaries.
pub fn validate_range(
    field: &'static str,
    value: f64,
    minimum: f64,
    maximum: f64,
    tolerance: f64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;
    validate_tolerance(tolerance)?;
    validate_finite(field, value)?;
    validate_finite("minimum", minimum)?;
    validate_finite("maximum", maximum)?;

    if minimum > maximum {
        return Err(PhysicalValidationError::InvalidConfiguration {
            field: "range",
            reason: "minimum must not exceed maximum",
        });
    }

    if value < minimum - tolerance || value > maximum + tolerance {
        return Err(PhysicalValidationError::OutOfRange {
            field,
            value_bits: value.to_bits(),
            minimum_bits: minimum.to_bits(),
            maximum_bits: maximum.to_bits(),
        });
    }

    Ok(())
}

/// Validates a probability in [0, 1].
pub fn validate_probability(
    field: &'static str,
    probability: f64,
    tolerance: f64,
) -> PhysicalValidationResult<()> {
    validate_range(field, probability, 0.0, 1.0, tolerance)
}

/// Validates an error rate in [0, 1].
pub fn validate_error_rate(
    field: &'static str,
    error_rate: f64,
    tolerance: f64,
) -> PhysicalValidationResult<()> {
    validate_probability(field, error_rate, tolerance)
}

/// Validates a fidelity in [0, 1].
pub fn validate_fidelity(
    field: &'static str,
    fidelity: f64,
    tolerance: f64,
) -> PhysicalValidationResult<()> {
    validate_probability(field, fidelity, tolerance)
}

/// Validates a survival probability in [0, 1].
pub fn validate_survival_probability(
    field: &'static str,
    probability: f64,
    tolerance: f64,
) -> PhysicalValidationResult<()> {
    validate_probability(field, probability, tolerance)
}

/// Validates a leakage probability/rate in [0, 1].
pub fn validate_leakage_rate(
    field: &'static str,
    leakage: f64,
    tolerance: f64,
) -> PhysicalValidationResult<()> {
    validate_probability(field, leakage, tolerance)
}

/// Validates an erasure probability/rate in [0, 1].
pub fn validate_erasure_rate(
    field: &'static str,
    erasure: f64,
    tolerance: f64,
) -> PhysicalValidationResult<()> {
    validate_probability(field, erasure, tolerance)
}

/// Validates a finite non-negative duration.
///
/// Duration is represented as seconds at this abstraction boundary.
pub fn validate_duration_seconds(
    field: &'static str,
    seconds: f64,
) -> PhysicalValidationResult<()> {
    validate_non_negative(field, seconds)
}

/// Validates a finite non-negative frequency.
///
/// Frequency is represented as hertz.
pub fn validate_frequency_hz(
    field: &'static str,
    frequency: f64,
) -> PhysicalValidationResult<()> {
    validate_non_negative(field, frequency)
}

/// Validates a finite non-negative energy.
///
/// The validator does not assume a particular energy unit.
pub fn validate_energy(
    field: &'static str,
    energy: f64,
) -> PhysicalValidationResult<()> {
    validate_non_negative(field, energy)
}

// ============================================================================
// Count validation
// ============================================================================

/// Validates an unsigned physical count.
pub fn validate_count(
    field: &'static str,
    count: u64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    // u64 is inherently non-negative. The function exists as an explicit
    // semantic boundary so callers do not need to duplicate validation logic.
    let _ = count;

    Ok(())
}

/// Validates errors/opportunities and derives the corresponding physical error
/// rate without performing any statistical inference.
///
/// This function rejects:
///
/// - zero opportunities;
/// - errors greater than opportunities.
///
/// It does not clamp values.
pub fn validate_error_counts(
    field: &'static str,
    errors: u64,
    opportunities: u64,
) -> PhysicalValidationResult<f64> {
    validate_field_name(field)?;

    if opportunities == 0 {
        return Err(PhysicalValidationError::ZeroOpportunities { field });
    }

    if errors > opportunities {
        return Err(PhysicalValidationError::InvalidCountRelationship {
            field,
            numerator: errors,
            denominator: opportunities,
        });
    }

    let rate = errors as f64 / opportunities as f64;

    if !rate.is_finite() {
        return Err(PhysicalValidationError::NonFinite { field });
    }

    Ok(rate)
}

/// Validates successes/opportunities and derives success probability.
pub fn validate_success_counts(
    field: &'static str,
    successes: u64,
    opportunities: u64,
) -> PhysicalValidationResult<f64> {
    validate_field_name(field)?;

    if opportunities == 0 {
        return Err(PhysicalValidationError::ZeroOpportunities { field });
    }

    if successes > opportunities {
        return Err(PhysicalValidationError::InvalidCountRelationship {
            field,
            numerator: successes,
            denominator: opportunities,
        });
    }

    let probability = successes as f64 / opportunities as f64;

    if !probability.is_finite() {
        return Err(PhysicalValidationError::NonFinite { field });
    }

    Ok(probability)
}

/// Validates that errors and successes partition all opportunities.
///
/// This is useful for binary physical-error experiments.
pub fn validate_binary_counts(
    field: &'static str,
    errors: u64,
    successes: u64,
    opportunities: u64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    if opportunities == 0 {
        return Err(PhysicalValidationError::ZeroOpportunities { field });
    }

    if errors > opportunities {
        return Err(PhysicalValidationError::InvalidCountRelationship {
            field,
            numerator: errors,
            denominator: opportunities,
        });
    }

    if successes > opportunities {
        return Err(PhysicalValidationError::InvalidCountRelationship {
            field,
            numerator: successes,
            denominator: opportunities,
        });
    }

    let sum = errors
        .checked_add(successes)
        .ok_or(PhysicalValidationError::LimitOverflow { field })?;

    if sum != opportunities {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "errors + successes must equal opportunities for a complete binary observation",
        });
    }

    Ok(())
}

/// Validates a count-derived rate and checks it against an independently
/// supplied rate.
///
/// This is useful when raw counts and a serialized rate are both present.
pub fn validate_count_rate_consistency(
    field: &'static str,
    errors: u64,
    opportunities: u64,
    reported_rate: f64,
    tolerance: f64,
) -> PhysicalValidationResult<()> {
    let derived = validate_error_counts(field, errors, opportunities)?;

    validate_error_rate(field, reported_rate, tolerance)?;

    if (derived - reported_rate).abs() > tolerance {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "reported rate is inconsistent with errors/opportunities",
        });
    }

    Ok(())
}

// ============================================================================
// Qubit/index validation
// ============================================================================

/// Validates a declared qubit count.
pub fn validate_qubit_count(
    field: &'static str,
    qubit_count: u64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    if qubit_count == 0 {
        return Err(PhysicalValidationError::InvalidConfiguration {
            field,
            reason: "qubit count must be greater than zero",
        });
    }

    Ok(())
}

/// Validates a qubit index against a declared qubit count.
///
/// Valid indices are:
///
/// `0 .. qubit_count`
pub fn validate_qubit_index(
    field: &'static str,
    index: u64,
    qubit_count: u64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;
    validate_qubit_count("qubit_count", qubit_count)?;

    if index >= qubit_count {
        return Err(PhysicalValidationError::QubitIndexOutOfRange {
            field,
            index,
            qubit_count,
        });
    }

    Ok(())
}

/// Validates two distinct qubit indices.
pub fn validate_two_qubit_indices(
    field: &'static str,
    first: u64,
    second: u64,
    qubit_count: u64,
) -> PhysicalValidationResult<()> {
    validate_qubit_index(field, first, qubit_count)?;
    validate_qubit_index(field, second, qubit_count)?;

    if first == second {
        return Err(PhysicalValidationError::DuplicateQubit {
            field,
            qubit: first,
        });
    }

    Ok(())
}

/// Validates a list of qubit indices.
///
/// Duplicate qubits are rejected because a physical operation's participant
/// set cannot contain the same physical qubit twice.
pub fn validate_qubit_indices(
    field: &'static str,
    indices: &[u64],
    qubit_count: u64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;
    validate_qubit_count("qubit_count", qubit_count)?;

    if indices.len() > DEFAULT_MAX_ELEMENTS {
        return Err(PhysicalValidationError::ValidationLimitExceeded {
            field,
            requested: indices.len(),
            maximum: DEFAULT_MAX_ELEMENTS,
        });
    }

    for (position, &index) in indices.iter().enumerate() {
        validate_qubit_index(field, index, qubit_count)?;

        for &previous in &indices[..position] {
            if previous == index {
                return Err(PhysicalValidationError::DuplicateQubit {
                    field,
                    qubit: index,
                });
            }
        }
    }

    Ok(())
}

// ============================================================================
// Probability distributions
// ============================================================================

/// Validates a finite probability distribution and requires normalization.
///
/// The distribution must:
///
/// - contain at least one element unless explicitly allowed by policy;
/// - contain only finite values;
/// - contain only values in [0, 1];
/// - sum to one within tolerance.
pub fn validate_probability_distribution(
    field: &'static str,
    probabilities: &[f64],
    policy: &PhysicalValidationPolicy,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    if probabilities.is_empty() && policy.reject_empty_distributions {
        return Err(PhysicalValidationError::EmptyDistribution { field });
    }

    if probabilities.len() > policy.max_elements {
        return Err(PhysicalValidationError::ValidationLimitExceeded {
            field,
            requested: probabilities.len(),
            maximum: policy.max_elements,
        });
    }

    let mut sum = 0.0;

    for (index, &probability) in probabilities.iter().enumerate() {
        if !probability.is_finite() {
            return Err(PhysicalValidationError::InvalidProbabilityDistribution {
                field,
                index,
                value_bits: probability.to_bits(),
            });
        }

        if !is_unit_interval(probability, policy.tolerance) {
            return Err(PhysicalValidationError::InvalidProbabilityDistribution {
                field,
                index,
                value_bits: probability.to_bits(),
            });
        }

        sum += probability;

        if !sum.is_finite() {
            return Err(PhysicalValidationError::NonFinite { field });
        }
    }

    if !approximately_equal(sum, 1.0, policy.tolerance) {
        return Err(PhysicalValidationError::DistributionNotNormalized {
            field,
            sum_bits: sum.to_bits(),
        });
    }

    Ok(())
}

/// Validates a probability distribution without requiring normalization.
///
/// This is useful for partial probability tables or probability weights.
pub fn validate_probability_weights(
    field: &'static str,
    probabilities: &[f64],
    policy: &PhysicalValidationPolicy,
) -> PhysicalValidationResult<f64> {
    validate_field_name(field)?;

    if probabilities.is_empty() && policy.reject_empty_distributions {
        return Err(PhysicalValidationError::EmptyDistribution { field });
    }

    if probabilities.len() > policy.max_elements {
        return Err(PhysicalValidationError::ValidationLimitExceeded {
            field,
            requested: probabilities.len(),
            maximum: policy.max_elements,
        });
    }

    let mut sum = 0.0;

    for (index, &probability) in probabilities.iter().enumerate() {
        if !probability.is_finite() || !is_unit_interval(probability, policy.tolerance) {
            return Err(PhysicalValidationError::InvalidProbabilityDistribution {
                field,
                index,
                value_bits: probability.to_bits(),
            });
        }

        sum += probability;

        if !sum.is_finite() {
            return Err(PhysicalValidationError::NonFinite { field });
        }
    }

    Ok(sum)
}

// ============================================================================
// Readout / stochastic matrix validation
// ============================================================================

/// Validates a square row-stochastic matrix.
///
/// Each entry must be a probability and every row must sum to one within the
/// configured tolerance.
///
/// This is appropriate for readout/assignment matrices when the matrix is
/// represented as:
///
/// `P(observed | prepared)`.
pub fn validate_row_stochastic_matrix(
    field: &'static str,
    matrix: &[Vec<f64>],
    policy: &PhysicalValidationPolicy,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    let dimension = matrix.len();

    if dimension == 0 {
        return Err(PhysicalValidationError::InvalidLength {
            field,
            actual: 0,
            expected: 1,
        });
    }

    if dimension > policy.max_matrix_dimension {
        return Err(PhysicalValidationError::ValidationLimitExceeded {
            field,
            requested: dimension,
            maximum: policy.max_matrix_dimension,
        });
    }

    let elements = dimension
        .checked_mul(dimension)
        .ok_or(PhysicalValidationError::LimitOverflow { field })?;

    if elements > policy.max_elements {
        return Err(PhysicalValidationError::ValidationLimitExceeded {
            field,
            requested: elements,
            maximum: policy.max_elements,
        });
    }

    for (row_index, row) in matrix.iter().enumerate() {
        if row.len() != dimension {
            return Err(PhysicalValidationError::NonSquareMatrix {
                field,
                rows: dimension,
                columns: row.len(),
            });
        }

        let mut sum = 0.0;

        for (column_index, &value) in row.iter().enumerate() {
            if !value.is_finite() || !is_unit_interval(value, policy.tolerance) {
                return Err(PhysicalValidationError::InvalidMatrixElement {
                    field,
                    row: row_index,
                    column: column_index,
                    value_bits: value.to_bits(),
                });
            }

            sum += value;

            if !sum.is_finite() {
                return Err(PhysicalValidationError::NonFinite { field });
            }
        }

        if !approximately_equal(sum, 1.0, policy.tolerance) {
            return Err(PhysicalValidationError::InvalidStochasticRow {
                field,
                row: row_index,
                sum_bits: sum.to_bits(),
            });
        }
    }

    Ok(())
}

/// Validates a square column-stochastic matrix.
///
/// This is useful when a backend or scientific representation uses:
///
/// `P(output | input)`
///
/// along columns rather than rows.
pub fn validate_column_stochastic_matrix(
    field: &'static str,
    matrix: &[Vec<f64>],
    policy: &PhysicalValidationPolicy,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    let dimension = matrix.len();

    if dimension == 0 {
        return Err(PhysicalValidationError::InvalidLength {
            field,
            actual: 0,
            expected: 1,
        });
    }

    if dimension > policy.max_matrix_dimension {
        return Err(PhysicalValidationError::ValidationLimitExceeded {
            field,
            requested: dimension,
            maximum: policy.max_matrix_dimension,
        });
    }

    let elements = dimension
        .checked_mul(dimension)
        .ok_or(PhysicalValidationError::LimitOverflow { field })?;

    if elements > policy.max_elements {
        return Err(PhysicalValidationError::ValidationLimitExceeded {
            field,
            requested: elements,
            maximum: policy.max_elements,
        });
    }

    for row in matrix {
        if row.len() != dimension {
            return Err(PhysicalValidationError::NonSquareMatrix {
                field,
                rows: dimension,
                columns: row.len(),
            });
        }
    }

    for column in 0..dimension {
        let mut sum = 0.0;

        for row in 0..dimension {
            let value = matrix[row][column];

            if !value.is_finite() || !is_unit_interval(value, policy.tolerance) {
                return Err(PhysicalValidationError::InvalidMatrixElement {
                    field,
                    row,
                    column,
                    value_bits: value.to_bits(),
                });
            }

            sum += value;

            if !sum.is_finite() {
                return Err(PhysicalValidationError::NonFinite { field });
            }
        }

        if !approximately_equal(sum, 1.0, policy.tolerance) {
            return Err(PhysicalValidationError::InvalidStochasticRow {
                field,
                row: column,
                sum_bits: sum.to_bits(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// Density-matrix validation
// ============================================================================

/// Complex value represented without introducing a dependency on a complex
/// number crate.
///
/// The representation is sufficient for physical trace validation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ComplexValue {
    /// Real component.
    pub real: f64,

    /// Imaginary component.
    pub imaginary: f64,
}

/// Validates the trace of a density matrix.
///
/// A physical density matrix must have:
///
/// `trace(real) = 1`
///
/// and:
///
/// `trace(imaginary) = 0`.
///
/// This function validates only the trace condition. It does NOT prove
/// Hermiticity or positive semidefiniteness.
pub fn validate_density_matrix_trace(
    field: &'static str,
    trace: ComplexValue,
    tolerance: f64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;
    validate_tolerance(tolerance)?;

    if !trace.real.is_finite() {
        return Err(PhysicalValidationError::InvalidDensityMatrixTrace {
            field,
            trace_real_bits: trace.real.to_bits(),
            trace_imag_bits: trace.imaginary.to_bits(),
        });
    }

    if !trace.imaginary.is_finite() {
        return Err(PhysicalValidationError::InvalidDensityMatrixTrace {
            field,
            trace_real_bits: trace.real.to_bits(),
            trace_imag_bits: trace.imaginary.to_bits(),
        });
    }

    if !approximately_equal(trace.real, 1.0, tolerance)
        || !approximately_equal(trace.imaginary, 0.0, tolerance)
    {
        return Err(PhysicalValidationError::InvalidDensityMatrixTrace {
            field,
            trace_real_bits: trace.real.to_bits(),
            trace_imag_bits: trace.imaginary.to_bits(),
        });
    }

    Ok(())
}

// ============================================================================
// Pauli probabilities
// ============================================================================

/// Validates a Pauli error probability distribution.
///
/// The expected order is:
///
/// ```text
/// [P(I), P(X), P(Y), P(Z)]
/// ```
///
/// The four values must sum to one.
pub fn validate_pauli_distribution(
    field: &'static str,
    identity: f64,
    x: f64,
    y: f64,
    z: f64,
    policy: &PhysicalValidationPolicy,
) -> PhysicalValidationResult<()> {
    let probabilities = [identity, x, y, z];

    validate_probability_distribution(field, &probabilities, policy)
}

// ============================================================================
// Resource relationships
// ============================================================================

/// Validates that a logical-qubit count does not exceed a physical-qubit
/// count.
///
/// This is a basic resource sanity check. It does not assert that a particular
/// QEC code is capable of achieving the requested encoding.
pub fn validate_physical_logical_qubits(
    field: &'static str,
    physical_qubits: u64,
    logical_qubits: u64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    if physical_qubits == 0 {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "physical qubit count must be greater than zero",
        });
    }

    if logical_qubits == 0 {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "logical qubit count must be greater than zero",
        });
    }

    if logical_qubits > physical_qubits {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "logical qubits cannot exceed physical qubits",
        });
    }

    Ok(())
}

/// Validates a physical/logical resource ratio.
pub fn validate_resource_ratio(
    field: &'static str,
    physical: u64,
    logical: u64,
) -> PhysicalValidationResult<f64> {
    validate_field_name(field)?;

    if logical == 0 {
        return Err(PhysicalValidationError::ZeroOpportunities { field });
    }

    if physical < logical {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "physical resource cannot be smaller than logical resource",
        });
    }

    let ratio = physical as f64 / logical as f64;

    if !ratio.is_finite() {
        return Err(PhysicalValidationError::NonFinite { field });
    }

    Ok(ratio)
}

/// Validates that a two-qubit gate count cannot exceed the total gate count.
pub fn validate_gate_counts(
    field: &'static str,
    total_gates: u64,
    single_qubit_gates: u64,
    two_qubit_gates: u64,
    multi_qubit_gates: u64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    let categorized = single_qubit_gates
        .checked_add(two_qubit_gates)
        .and_then(|value| value.checked_add(multi_qubit_gates))
        .ok_or(PhysicalValidationError::LimitOverflow { field })?;

    if categorized > total_gates {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "categorized gate count exceeds total gate count",
        });
    }

    Ok(())
}

/// Validates circuit depth relationships.
///
/// Two-qubit depth and total depth cannot exceed total depth.
pub fn validate_depths(
    field: &'static str,
    depth: u64,
    two_qubit_depth: u64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    if two_qubit_depth > depth {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "two-qubit depth cannot exceed total circuit depth",
        });
    }

    Ok(())
}

/// Validates measurement count against shot count when each shot is expected
/// to contribute at most one aggregate measurement opportunity.
pub fn validate_measurement_shots(
    field: &'static str,
    measurements: u64,
    shots: u64,
) -> PhysicalValidationResult<()> {
    validate_field_name(field)?;

    if shots == 0 {
        return Err(PhysicalValidationError::ZeroOpportunities { field });
    }

    if measurements > shots {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "measurement opportunities cannot exceed shots under the supplied one-measurement-per-shot model",
        });
    }

    Ok(())
}

// ============================================================================
// Leakage / erasure relationships
// ============================================================================

/// Validates a complete physical error decomposition.
///
/// The supplied rates represent mutually exclusive categories and therefore
/// must sum to no more than one.
///
/// This function does not require the categories to exhaust the full unit
/// interval because unclassified/other physical behavior may exist.
pub fn validate_error_decomposition(
    field: &'static str,
    preparation: f64,
    gate: f64,
    measurement: f64,
    leakage: f64,
    erasure: f64,
    tolerance: f64,
) -> PhysicalValidationResult<f64> {
    validate_probability("preparation", preparation, tolerance)?;
    validate_probability("gate", gate, tolerance)?;
    validate_probability("measurement", measurement, tolerance)?;
    validate_probability("leakage", leakage, tolerance)?;
    validate_probability("erasure", erasure, tolerance)?;

    let total = preparation + gate + measurement + leakage + erasure;

    if !total.is_finite() {
        return Err(PhysicalValidationError::NonFinite { field });
    }

    if total > 1.0 + tolerance {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "mutually exclusive error probabilities exceed one",
        });
    }

    Ok(total)
}

/// Validates that leakage and erasure are individually bounded and that their
/// combined probability does not exceed one.
pub fn validate_leakage_erasure(
    field: &'static str,
    leakage: f64,
    erasure: f64,
    tolerance: f64,
) -> PhysicalValidationResult<f64> {
    validate_leakage_rate("leakage", leakage, tolerance)?;
    validate_erasure_rate("erasure", erasure, tolerance)?;

    let total = leakage + erasure;

    if !total.is_finite() {
        return Err(PhysicalValidationError::NonFinite { field });
    }

    if total > 1.0 + tolerance {
        return Err(PhysicalValidationError::InvalidResourceRelationship {
            field,
            reason: "leakage + erasure exceeds one",
        });
    }

    Ok(total)
}

// ============================================================================
// Generic helpers
// ============================================================================

/// Returns whether a value is in [0, 1] with the supplied tolerance.
///
/// Non-finite values always return false.
pub fn is_unit_interval(
    value: f64,
    tolerance: f64,
) -> bool {
    if !value.is_finite() {
        return false;
    }

    value >= -tolerance && value <= 1.0 + tolerance
}

/// Returns whether two finite values are equal under absolute tolerance.
///
/// Non-finite values are never approximately equal.
pub fn approximately_equal(
    left: f64,
    right: f64,
    tolerance: f64,
) -> bool {
    if !left.is_finite() || !right.is_finite() {
        return false;
    }

    if !tolerance.is_finite() || tolerance < 0.0 {
        return false;
    }

    (left - right).abs() <= tolerance
}

/// Validates the static field labels used by this module.
///
/// This prevents accidental introduction of enormous dynamically generated
/// diagnostic labels at the validation boundary.
pub fn validate_field_name(
    field: &'static str,
) -> PhysicalValidationResult<()> {
    if field.is_empty() || field.len() > MAX_FIELD_NAME_BYTES {
        return Err(PhysicalValidationError::InvalidFieldName);
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> PhysicalValidationPolicy {
        PhysicalValidationPolicy::default()
    }

    #[test]
    fn default_policy_is_valid() {
        let value = policy();

        assert!(value.tolerance > 0.0);
        assert!(value.max_elements > 0);
        assert!(value.max_matrix_dimension > 0);
    }

    #[test]
    fn tolerance_is_validated() {
        assert!(validate_tolerance(DEFAULT_TOLERANCE).is_ok());
        assert!(validate_tolerance(f64::NAN).is_err());
        assert!(validate_tolerance(f64::INFINITY).is_err());
        assert!(validate_tolerance(MAX_TOLERANCE + 1.0e-3).is_err());
    }

    #[test]
    fn probability_boundary_values_are_valid() {
        assert!(validate_probability("p", 0.0, DEFAULT_TOLERANCE).is_ok());
        assert!(validate_probability("p", 1.0, DEFAULT_TOLERANCE).is_ok());
    }

    #[test]
    fn probability_outside_domain_is_rejected() {
        assert!(validate_probability("p", -0.1, DEFAULT_TOLERANCE).is_err());
        assert!(validate_probability("p", 1.1, DEFAULT_TOLERANCE).is_err());
    }

    #[test]
    fn nan_and_infinity_are_rejected() {
        assert!(validate_probability("p", f64::NAN, DEFAULT_TOLERANCE).is_err());
        assert!(validate_probability("p", f64::INFINITY, DEFAULT_TOLERANCE).is_err());
        assert!(validate_probability("p", f64::NEG_INFINITY, DEFAULT_TOLERANCE).is_err());
    }

    #[test]
    fn error_counts_are_validated() {
        let rate = validate_error_counts("errors", 25, 100).unwrap();

        assert!((rate - 0.25).abs() < 1.0e-15);
    }

    #[test]
    fn errors_cannot_exceed_opportunities() {
        assert!(validate_error_counts("errors", 101, 100).is_err());
    }

    #[test]
    fn zero_opportunities_are_rejected() {
        assert!(validate_error_counts("errors", 0, 0).is_err());
    }

    #[test]
    fn binary_counts_require_partition() {
        assert!(
            validate_binary_counts("experiment", 25, 75, 100).is_ok()
        );

        assert!(
            validate_binary_counts("experiment", 25, 74, 100).is_err()
        );
    }

    #[test]
    fn qubit_indices_are_zero_based() {
        assert!(
            validate_qubit_index("q", 0, 4).is_ok()
        );

        assert!(
            validate_qubit_index("q", 3, 4).is_ok()
        );

        assert!(
            validate_qubit_index("q", 4, 4).is_err()
        );
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        assert!(
            validate_two_qubit_indices("gate", 0, 0, 4).is_err()
        );

        assert!(
            validate_qubit_indices("gate", &[0, 1], 4).is_ok()
        );

        assert!(
            validate_qubit_indices("gate", &[0, 1, 0], 4).is_err()
        );
    }

    #[test]
    fn normalized_probability_distribution_is_valid() {
        let values = [0.25, 0.25, 0.25, 0.25];

        assert!(
            validate_probability_distribution(
                "distribution",
                &values,
                &policy(),
            )
            .is_ok()
        );
    }

    #[test]
    fn non_normalized_distribution_is_rejected() {
        let values = [0.25, 0.25];

        assert!(
            validate_probability_distribution(
                "distribution",
                &values,
                &policy(),
            )
            .is_err()
        );
    }

    #[test]
    fn invalid_probability_element_is_rejected() {
        let values = [0.5, 0.5, -0.01];

        assert!(
            validate_probability_distribution(
                "distribution",
                &values,
                &policy(),
            )
            .is_err()
        );
    }

    #[test]
    fn row_stochastic_matrix_is_valid() {
        let matrix = vec![
            vec![0.9, 0.1],
            vec![0.2, 0.8],
        ];

        assert!(
            validate_row_stochastic_matrix(
                "readout",
                &matrix,
                &policy(),
            )
            .is_ok()
        );
    }

    #[test]
    fn row_stochastic_matrix_must_be_square() {
        let matrix = vec![
            vec![1.0, 0.0],
            vec![0.0],
        ];

        assert!(
            validate_row_stochastic_matrix(
                "readout",
                &matrix,
                &policy(),
            )
            .is_err()
        );
    }

    #[test]
    fn row_stochastic_matrix_must_normalize() {
        let matrix = vec![
            vec![0.9, 0.2],
            vec![0.2, 0.8],
        ];

        assert!(
            validate_row_stochastic_matrix(
                "readout",
                &matrix,
                &policy(),
            )
            .is_err()
        );
    }

    #[test]
    fn column_stochastic_matrix_is_valid() {
        let matrix = vec![
            vec![0.9, 0.2],
            vec![0.1, 0.8],
        ];

        assert!(
            validate_column_stochastic_matrix(
                "transition",
                &matrix,
                &policy(),
            )
            .is_ok()
        );
    }

    #[test]
    fn density_matrix_trace_is_valid() {
        let trace = ComplexValue {
            real: 1.0,
            imaginary: 0.0,
        };

        assert!(
            validate_density_matrix_trace(
                "rho",
                trace,
                DEFAULT_TOLERANCE,
            )
            .is_ok()
        );
    }

    #[test]
    fn density_matrix_trace_is_rejected_when_invalid() {
        let trace = ComplexValue {
            real: 0.99,
            imaginary: 0.0,
        };

        assert!(
            validate_density_matrix_trace(
                "rho",
                trace,
                DEFAULT_TOLERANCE,
            )
            .is_err()
        );
    }

    #[test]
    fn pauli_distribution_requires_normalization() {
        assert!(
            validate_pauli_distribution(
                "pauli",
                0.9,
                0.03,
                0.02,
                0.05,
                &policy(),
            )
            .is_ok()
        );

        assert!(
            validate_pauli_distribution(
                "pauli",
                0.9,
                0.03,
                0.02,
                0.01,
                &policy(),
            )
            .is_err()
        );
    }

    #[test]
    fn logical_qubits_cannot_exceed_physical_qubits() {
        assert!(
            validate_physical_logical_qubits(
                "encoding",
                100,
                10,
            )
            .is_ok()
        );

        assert!(
            validate_physical_logical_qubits(
                "encoding",
                10,
                100,
            )
            .is_err()
        );
    }

    #[test]
    fn resource_ratio_is_validated() {
        let ratio =
            validate_resource_ratio("qubit_overhead", 100, 10)
                .unwrap();

        assert!((ratio - 10.0).abs() < 1.0e-15);
    }

    #[test]
    fn gate_counts_are_validated() {
        assert!(
            validate_gate_counts(
                "gates",
                100,
                60,
                30,
                10,
            )
            .is_ok()
        );

        assert!(
            validate_gate_counts(
                "gates",
                100,
                60,
                30,
                11,
            )
            .is_err()
        );
    }

    #[test]
    fn depth_relationship_is_validated() {
        assert!(
            validate_depths("depth", 100, 40).is_ok()
        );

        assert!(
            validate_depths("depth", 40, 100).is_err()
        );
    }

    #[test]
    fn leakage_and_erasure_are_bounded() {
        assert!(
            validate_leakage_erasure(
                "loss",
                0.1,
                0.2,
                DEFAULT_TOLERANCE,
            )
            .is_ok()
        );

        assert!(
            validate_leakage_erasure(
                "loss",
                0.8,
                0.3,
                DEFAULT_TOLERANCE,
            )
            .is_err()
        );
    }

    #[test]
    fn error_decomposition_is_bounded() {
        assert!(
            validate_error_decomposition(
                "errors",
                0.01,
                0.02,
                0.03,
                0.01,
                0.01,
                DEFAULT_TOLERANCE,
            )
            .is_ok()
        );

        assert!(
            validate_error_decomposition(
                "errors",
                0.5,
                0.5,
                0.1,
                0.0,
                0.0,
                DEFAULT_TOLERANCE,
            )
            .is_err()
        );
    }

    #[test]
    fn field_name_is_bounded() {
        assert!(validate_field_name("probability").is_ok());
        assert!(validate_field_name("").is_err());
    }

    #[test]
    fn approximately_equal_is_deterministic() {
        assert!(
            approximately_equal(
                1.0,
                1.0 + DEFAULT_TOLERANCE / 2.0,
                DEFAULT_TOLERANCE,
            )
        );

        assert!(
            !approximately_equal(
                1.0,
                1.1,
                DEFAULT_TOLERANCE,
            )
        );
    }
}