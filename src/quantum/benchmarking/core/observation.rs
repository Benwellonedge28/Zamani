//! Zamani Quantum Benchmarking — Execution Observation Model
//!
//! This module defines the canonical, backend-independent representation of
//! raw observations produced by quantum benchmark execution.
//!
//! # Architectural role
//!
//! `Observation` is deliberately positioned between execution and analysis:
//
//! ```text
//! BenchmarkExperiment
//!        │
//!        ▼
//! execution/*
//!        │
//!        ▼
//! ┌──────────────────────────────┐
//! │ core::observation            │
//! │                              │
//! │ normalized raw observations  │
//! └──────────────┬───────────────┘
//!                │
//!       ┌────────┼─────────┐
//!       ▼        ▼         ▼
//!    metrics  statistics protocols
//! ```
//!
//! This module does NOT:
//!
//! - execute circuits;
//! - generate circuits;
//! - choose a backend;
//! - perform routing;
//! - perform scheduling;
//! - perform compilation;
//! - calculate benchmark metrics;
//! - fit statistical models;
//! - decide benchmark pass/fail;
//! - interpret application-specific results;
//! - assume that every quantum system is a gate-model QPU;
//! - assume that every backend returns bitstrings.
//!
//! # Supported observation families
//!
//! The model supports:
//!
//! - computational-basis counts;
//! - normalized probability distributions;
//! - expectation values;
//! - state amplitudes;
//! - density matrices;
//! - analog measurements;
//! - annealing samples;
//! - QEC syndrome observations;
//! - timing observations;
//! - calibration observations;
//! - backend metadata;
//! - extensible named observations.
//!
//! This allows the same benchmarking subsystem to support gate-model,
//! analog, annealing, photonic, simulator, emulator, and error-correction
//! workloads without forcing all backends into a bitstring-only model.
//!
//! # Important semantic rule
//!
//! Raw observations are not metrics.
//!
//! For example:
//!
//! `0.97`
//!
//! might be:
//!
//! - a measured probability;
//! - an expectation value;
//! - a fidelity;
//! - a readout value;
//! - a calibration parameter.
//!
//! The observation layer must preserve what was actually observed. Metric
//! interpretation belongs to `metrics/*` and `protocols/*`.
//!
//! # Production guarantees
//!
//! This module provides:
//!
//! - explicit observation kinds;
//! - validated numeric domains;
//! - deterministic ordering for named distributions;
//! - checked count/probability conversions;
//! - resource-limit enforcement;
//! - explicit shot/sample counts;
//! - explicit basis/state dimensions;
//! - explicit timing units;
//! - explicit provenance references;
//! - partial-result representation;
//! - execution status;
//! - structured warnings;
//! - no logging or printing;
//! - no global state;
//! - no unsafe code;
//! - no protocol-specific dependencies.
//!
//! # Resource safety
//!
//! Constructors which can allocate dynamic observation data require
//! `BenchmarkLimits`. Callers which already have an appropriately configured
//! policy may use the corresponding `with_limits` constructors.
//!
//! The limits are a safety boundary and are intentionally checked before
//! materializing large vectors where the requested element count is known.
//!
//! # Serialization
//!
//! The observation model derives Serde serialization for interoperability with
//! the future canonical benchmark-result/reporting layers.
//!
//! Deserialized data MUST be validated with [`Observation::validate`] before
//! it is trusted by execution-analysis or reporting code. The future result
//! boundary should therefore use:
//!
//! ```text
//! deserialize
//!     ↓
//! Observation::validate
//!     ↓
//! BenchmarkResult
//! ```
//!
//! # Integration contract
//!
//! This file depends only on:
//!
//! - the Rust standard library;
//! - `serde`;
//! - `crate::quantum::benchmarking::core::limits`.
//!
//! It intentionally does NOT depend on:
//!
//! - `core::metric`;
//! - `core::result`;
//! - `core::experiment`;
//! - `core::circuit`;
//! - `quantum::ir`;
//! - `quantum::hardware`;
//! - `runtime::quantum`;
//! - individual benchmark protocols.
//!
//! This makes the file safe to complete before those modules exist.
//!
//! Downstream integration:
//!
//! ```text
//! execution::response
//!        │
//!        ▼
//! ObservationSet
//!        │
//!        ├── metrics::*
//!        ├── statistics::*
//!        ├── protocols::*
//!        └── core::result
//! ```
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! Rust 2021.
//! No nightly features.
//! No unsafe code.

#![deny(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::limits::{BenchmarkLimits, LimitError};

// =============================================================================
// Schema
// =============================================================================

/// Current schema version for normalized benchmark observations.
///
/// This version is independent of the overall benchmark-result schema.
pub const OBSERVATION_SCHEMA_VERSION: u16 = 1;

/// Maximum number of bytes used to represent one bitstring.
///
/// 65,536 bits is deliberately large enough for current benchmark workloads
/// while preventing accidental unbounded bitstring materialization.
pub const MAX_BITSTRING_BITS: usize = 65_536;

/// Maximum number of matrix elements accepted by the observation model before
/// the configured benchmark limit is applied.
pub const MAX_MATRIX_ELEMENTS: u64 = 1_073_741_824;

/// Maximum number of expectation values in one observation payload.
pub const MAX_EXPECTATION_VALUES: usize = 1_000_000;

/// Maximum number of analog samples in one observation payload.
pub const MAX_ANALOG_SAMPLES: usize = 10_000_000;

/// Maximum number of annealing samples in one observation payload.
pub const MAX_ANNEALING_SAMPLES: usize = 10_000_000;

/// Maximum number of syndrome records in one observation payload.
pub const MAX_SYNDROME_RECORDS: usize = 10_000_000;

/// Maximum number of calibration fields in one observation payload.
pub const MAX_CALIBRATION_FIELDS: usize = 16_384;

/// Maximum number of backend metadata fields in one observation payload.
pub const MAX_METADATA_FIELDS: usize = 16_384;

/// Maximum number of extensible observation fields.
pub const MAX_CUSTOM_FIELDS: usize = 16_384;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating observations.
#[derive(Debug, Clone, PartialEq)]
pub enum ObservationError {
    /// An observation identifier was empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// An identifier contains invalid characters.
    InvalidIdentifier {
        field: &'static str,
    },

    /// A numeric value was not finite.
    NonFiniteValue {
        field: &'static str,
    },

    /// A probability was outside `[0, 1]`.
    InvalidProbability {
        field: &'static str,
        value: f64,
    },

    /// An expectation value was outside its declared range.
    InvalidExpectation {
        value: f64,
    },

    /// A count was inconsistent with another count.
    InvalidCount {
        field: &'static str,
    },

    /// A probability distribution was invalid.
    InvalidDistribution {
        reason: &'static str,
    },

    /// A matrix did not have a valid shape.
    InvalidMatrixShape {
        rows: usize,
        columns: usize,
        elements: usize,
    },

    /// A state-vector length is not a valid power of two.
    InvalidStateDimension {
        length: usize,
    },

    /// A density-matrix shape is invalid.
    InvalidDensityMatrix {
        rows: usize,
        columns: usize,
    },

    /// A requested observation payload exceeds a configured resource limit.
    LimitExceeded {
        resource: &'static str,
        requested: u64,
        maximum: u64,
    },

    /// A resource calculation overflowed.
    ArithmeticOverflow {
        resource: &'static str,
    },

    /// A bitstring is too large for the observation representation.
    BitstringTooLarge {
        bits: usize,
    },

    /// An observation was internally inconsistent.
    Inconsistent {
        reason: &'static str,
    },

    /// A required field was absent.
    MissingField {
        field: &'static str,
    },

    /// An observation kind does not permit a supplied payload.
    InvalidPayload {
        kind: &'static str,
    },

    /// A timing value is invalid.
    InvalidDuration {
        field: &'static str,
    },

    /// A syndrome record contains an invalid bit.
    InvalidSyndromeBit,

    /// A calibration value is invalid.
    InvalidCalibrationValue {
        field: String,
    },
}

impl fmt::Display for ObservationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(f, "{field} cannot be empty")
            }

            Self::InvalidIdentifier { field } => {
                write!(
                    f,
                    "{field} contains unsupported identifier characters"
                )
            }

            Self::NonFiniteValue { field } => {
                write!(f, "{field} must be finite")
            }

            Self::InvalidProbability { field, value } => {
                write!(
                    f,
                    "{field} must be within [0, 1], got {value}"
                )
            }

            Self::InvalidExpectation { value } => {
                write!(
                    f,
                    "expectation value must be within [-1, 1], got {value}"
                )
            }

            Self::InvalidCount { field } => {
                write!(f, "invalid count relationship for {field}")
            }

            Self::InvalidDistribution { reason } => {
                write!(f, "invalid probability distribution: {reason}")
            }

            Self::InvalidMatrixShape {
                rows,
                columns,
                elements,
            } => {
                write!(
                    f,
                    "invalid matrix shape: {rows}x{columns} with \
                     {elements} elements"
                )
            }

            Self::InvalidStateDimension { length } => {
                write!(
                    f,
                    "state-vector length {length} is not a positive power of two"
                )
            }

            Self::InvalidDensityMatrix { rows, columns } => {
                write!(
                    f,
                    "invalid density-matrix shape: {rows}x{columns}"
                )
            }

            Self::LimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "observation resource '{resource}' exceeds limit: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "observation arithmetic overflowed for '{resource}'"
                )
            }

            Self::BitstringTooLarge { bits } => {
                write!(
                    f,
                    "bitstring contains {bits} bits, maximum is \
                     {MAX_BITSTRING_BITS}"
                )
            }

            Self::Inconsistent { reason } => {
                write!(f, "inconsistent observation: {reason}")
            }

            Self::MissingField { field } => {
                write!(f, "required observation field '{field}' is missing")
            }

            Self::InvalidPayload { kind } => {
                write!(
                    f,
                    "payload is invalid for observation kind '{kind}'"
                )
            }

            Self::InvalidDuration { field } => {
                write!(f, "{field} contains an invalid duration")
            }

            Self::InvalidSyndromeBit => {
                write!(f, "syndrome bits must contain only 0 or 1")
            }

            Self::InvalidCalibrationValue { field } => {
                write!(
                    f,
                    "calibration field '{field}' contains an invalid value"
                )
            }
        }
    }
}

impl std::error::Error for ObservationError {}

impl From<LimitError> for ObservationError {
    fn from(error: LimitError) -> Self {
        match error {
            LimitError::ZeroValue { resource } => {
                Self::LimitExceeded {
                    resource,
                    requested: 0,
                    maximum: 0,
                }
            }

            LimitError::Exceeded {
                resource,
                requested,
                maximum,
            } => Self::LimitExceeded {
                resource,
                requested,
                maximum,
            },

            LimitError::ArithmeticOverflow { resource } => {
                Self::ArithmeticOverflow { resource }
            }

            LimitError::InvalidTimeout { milliseconds } => {
                Self::LimitExceeded {
                    resource: "timeout_ms",
                    requested: milliseconds,
                    maximum: super::limits::MAX_DURATION_MS,
                }
            }
        }
    }
}

// =============================================================================
// Observation identity
// =============================================================================

/// Stable identifier for one observation payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObservationId(String);

impl ObservationId {
    /// Creates a validated observation identifier.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, ObservationError> {
        let value = value.into();

        validate_identifier("observation_id", &value)?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for ObservationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Observation kind
// =============================================================================

/// Semantic category of a raw quantum benchmark observation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObservationKind {
    /// Computational-basis shot counts.
    Counts,

    /// A normalized probability distribution.
    Probabilities,

    /// One or more expectation values.
    ExpectationValues,

    /// A state vector represented by complex amplitudes.
    StateVector,

    /// A density matrix represented by complex entries.
    DensityMatrix,

    /// Analog samples or continuous measurement values.
    Analog,

    /// Samples returned by an annealing/optimization-oriented backend.
    AnnealingSamples,

    /// Quantum error-correction syndrome observations.
    Syndrome,

    /// Execution and system timing observations.
    Timing,

    /// Calibration snapshot values.
    Calibration,

    /// Backend/device metadata observed at execution time.
    BackendMetadata,

    /// Protocol-specific raw observation.
    Custom(String),
}

impl ObservationKind {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Counts => "counts",
            Self::Probabilities => "probabilities",
            Self::ExpectationValues => "expectation_values",
            Self::StateVector => "state_vector",
            Self::DensityMatrix => "density_matrix",
            Self::Analog => "analog",
            Self::AnnealingSamples => "annealing_samples",
            Self::Syndrome => "syndrome",
            Self::Timing => "timing",
            Self::Calibration => "calibration",
            Self::BackendMetadata => "backend_metadata",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Returns whether the observation normally represents sampled data.
    #[must_use]
    pub const fn is_sample_based(&self) -> bool {
        matches!(
            self,
            Self::Counts
                | Self::Probabilities
                | Self::Analog
                | Self::AnnealingSamples
                | Self::Syndrome
        )
    }

    /// Returns whether the observation normally represents a state object.
    #[must_use]
    pub const fn is_state_based(&self) -> bool {
        matches!(
            self,
            Self::StateVector | Self::DensityMatrix
        )
    }
}

// =============================================================================
// Complex numbers
// =============================================================================

/// Backend-independent complex scalar.
///
/// This intentionally lives inside the observation boundary rather than
/// depending on a numerical crate. A future numerical subsystem can convert
/// it without changing the benchmark observation schema.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Complex64 {
    /// Real component.
    pub re: f64,

    /// Imaginary component.
    pub im: f64,
}

impl Complex64 {
    /// Creates a complex value after validating both components.
    pub fn new(re: f64, im: f64) -> Result<Self, ObservationError> {
        if !re.is_finite() {
            return Err(ObservationError::NonFiniteValue {
                field: "complex.re",
            });
        }

        if !im.is_finite() {
            return Err(ObservationError::NonFiniteValue {
                field: "complex.im",
            });
        }

        Ok(Self { re, im })
    }

    /// Returns the squared magnitude.
    #[must_use]
    pub fn norm_squared(self) -> f64 {
        self.re.mul_add(self.re, self.im * self.im)
    }
}

// =============================================================================
// Bitstrings
// =============================================================================

/// Canonical computational-basis bitstring.
///
/// Bits are stored in logical measurement order:
///
/// `bits[0]` is the first logical bit supplied by the executor.
///
/// The observation layer does not reinterpret endianness. Backend-specific
/// endianness normalization must happen before constructing this type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BitString {
    bits: Vec<u8>,
}

impl BitString {
    /// Creates a bitstring from validated binary values.
    pub fn new(bits: Vec<u8>) -> Result<Self, ObservationError> {
        if bits.len() > MAX_BITSTRING_BITS {
            return Err(ObservationError::BitstringTooLarge {
                bits: bits.len(),
            });
        }

        if bits.iter().any(|bit| *bit > 1) {
            return Err(ObservationError::InvalidSyndromeBit);
        }

        Ok(Self { bits })
    }

    /// Creates a bitstring from a binary string such as `"0101"`.
    pub fn from_binary_string(
        value: &str,
    ) -> Result<Self, ObservationError> {
        let mut bits = Vec::with_capacity(value.len());

        for byte in value.bytes() {
            match byte {
                b'0' => bits.push(0),
                b'1' => bits.push(1),
                _ => {
                    return Err(
                        ObservationError::InvalidSyndromeBit
                    )
                }
            }
        }

        Self::new(bits)
    }

    /// Returns the number of bits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Returns whether the bitstring contains no bits.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Returns the bits in logical measurement order.
    #[must_use]
    pub fn bits(&self) -> &[u8] {
        &self.bits
    }

    /// Returns the bit at an index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<u8> {
        self.bits.get(index).copied()
    }

    /// Returns a binary string representation.
    #[must_use]
    pub fn to_binary_string(&self) -> String {
        self.bits
            .iter()
            .map(|bit| if *bit == 0 { '0' } else { '1' })
            .collect()
    }
}

impl fmt::Display for BitString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bit in &self.bits {
            write!(f, "{bit}")?;
        }

        Ok(())
    }
}

// =============================================================================
// Counts
// =============================================================================

/// One computational-basis outcome and its observed frequency.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeCount {
    /// Measured bitstring.
    pub outcome: BitString,

    /// Number of shots producing this outcome.
    pub count: u64,
}

impl OutcomeCount {
    /// Creates an outcome-count record.
    pub fn new(
        outcome: BitString,
        count: u64,
    ) -> Self {
        Self { outcome, count }
    }
}

/// Computational-basis measurement counts.
///
/// A `BTreeMap` is used intentionally so serialized results and hashes do not
/// depend on hash-map iteration order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountsObservation {
    /// Number of requested/observed shots.
    shots: u64,

    /// Ordered outcome counts.
    counts: BTreeMap<String, u64>,

    /// Number of measured bits.
    bit_width: usize,
}

impl CountsObservation {
    /// Creates counts from binary outcomes.
    ///
    /// The constructor checks:
    ///
    /// - all outcomes have equal width;
    /// - outcomes contain only binary values;
    /// - total counts do not overflow;
    /// - total counts equal `shots`.
    pub fn new(
        shots: u64,
        outcomes: Vec<OutcomeCount>,
    ) -> Result<Self, ObservationError> {
        Self::with_limits(
            shots,
            outcomes,
            &BenchmarkLimits::default(),
        )
    }

    /// Creates counts under an explicit resource policy.
    pub fn with_limits(
        shots: u64,
        outcomes: Vec<OutcomeCount>,
        limits: &BenchmarkLimits,
    ) -> Result<Self, ObservationError> {
        limits.check_shots(shots)?;

        if outcomes.len() > usize::MAX {
            return Err(ObservationError::ArithmeticOverflow {
                resource: "outcome_count",
            });
        }

        let mut counts = BTreeMap::new();
        let mut bit_width = None;
        let mut total = 0_u64;

        for entry in outcomes {
            let width = entry.outcome.len();

            if let Some(expected) = bit_width {
                if expected != width {
                    return Err(ObservationError::Inconsistent {
                        reason: "count outcomes have different bit widths",
                    });
                }
            } else {
                bit_width = Some(width);
            }

            total = total.checked_add(entry.count).ok_or(
                ObservationError::ArithmeticOverflow {
                    resource: "count_total",
                },
            )?;

            let key = entry.outcome.to_binary_string();

            let existing = counts.get(&key).copied().unwrap_or(0);

            let merged = existing.checked_add(entry.count).ok_or(
                ObservationError::ArithmeticOverflow {
                    resource: "outcome_count",
                },
            )?;

            counts.insert(key, merged);
        }

        if total != shots {
            return Err(ObservationError::InvalidCount {
                field: "shots",
            });
        }

        let bit_width = bit_width.unwrap_or(0);

        limits.check_observations(
            u64::try_from(counts.len()).map_err(|_| {
                ObservationError::ArithmeticOverflow {
                    resource: "outcome_count",
                }
            })?,
        )?;

        Ok(Self {
            shots,
            counts,
            bit_width,
        })
    }

    /// Creates an empty zero-shot observation.
    ///
    /// This is useful for explicit partial/cancelled execution results.
    pub fn empty() -> Self {
        Self {
            shots: 0,
            counts: BTreeMap::new(),
            bit_width: 0,
        }
    }

    /// Returns the number of shots.
    #[must_use]
    pub const fn shots(&self) -> u64 {
        self.shots
    }

    /// Returns the number of measured bits.
    #[must_use]
    pub const fn bit_width(&self) -> usize {
        self.bit_width
    }

    /// Returns the ordered count map.
    #[must_use]
    pub fn counts(&self) -> &BTreeMap<String, u64> {
        &self.counts
    }

    /// Returns the number of distinct outcomes.
    #[must_use]
    pub fn distinct_outcomes(&self) -> usize {
        self.counts.len()
    }

    /// Returns the count for an outcome.
    #[must_use]
    pub fn count_for(&self, outcome: &str) -> Option<u64> {
        self.counts.get(outcome).copied()
    }

    /// Converts counts to normalized probabilities.
    ///
    /// This performs no metric interpretation.
    pub fn probabilities(
        &self,
    ) -> Result<ProbabilityDistribution, ObservationError> {
        if self.shots == 0 {
            return Err(ObservationError::InvalidCount {
                field: "shots",
            });
        }

        let mut probabilities = BTreeMap::new();

        for (outcome, count) in &self.counts {
            probabilities.insert(
                outcome.clone(),
                *count as f64 / self.shots as f64,
            );
        }

        ProbabilityDistribution::new(
            probabilities,
            self.shots,
        )
    }
}

// =============================================================================
// Probability distributions
// =============================================================================

/// Normalized computational-basis probability distribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProbabilityDistribution {
    /// Ordered probabilities keyed by canonical bitstring.
    probabilities: BTreeMap<String, f64>,

    /// Number of samples from which the empirical distribution was derived,
    /// when known.
    sample_count: Option<u64>,
}

impl ProbabilityDistribution {
    /// Creates a distribution without an associated sample count.
    pub fn new(
        probabilities: BTreeMap<String, f64>,
        sample_count: u64,
    ) -> Result<Self, ObservationError> {
        Self::new_optional_samples(
            probabilities,
            Some(sample_count),
        )
    }

    /// Creates a distribution with an optional sample count.
    pub fn new_optional_samples(
        probabilities: BTreeMap<String, f64>,
        sample_count: Option<u64>,
    ) -> Result<Self, ObservationError> {
        let mut total = 0.0_f64;
        let mut width = None;

        for (outcome, probability) in &probabilities {
            validate_probability(
                "probability",
                *probability,
            )?;

            let parsed = BitString::from_binary_string(outcome)?;

            if let Some(expected) = width {
                if expected != parsed.len() {
                    return Err(ObservationError::InvalidDistribution {
                        reason: "outcomes have different widths",
                    });
                }
            } else {
                width = Some(parsed.len());
            }

            total += *probability;
        }

        if !total.is_finite() {
            return Err(ObservationError::NonFiniteValue {
                field: "probability_sum",
            });
        }

        // A probability distribution supplied to the observation boundary is
        // required to be normalized. We use a small numerical tolerance to
        // accommodate floating-point summation.
        const NORMALIZATION_TOLERANCE: f64 = 1.0e-9;

        if (total - 1.0).abs() > NORMALIZATION_TOLERANCE {
            return Err(ObservationError::InvalidDistribution {
                reason: "probabilities must sum to one",
            });
        }

        if let Some(samples) = sample_count {
            if samples == 0 {
                return Err(ObservationError::InvalidCount {
                    field: "sample_count",
                });
            }
        }

        Ok(Self {
            probabilities,
            sample_count,
        })
    }

    /// Returns the ordered distribution.
    #[must_use]
    pub fn probabilities(&self) -> &BTreeMap<String, f64> {
        &self.probabilities
    }

    /// Returns the optional empirical sample count.
    #[must_use]
    pub const fn sample_count(&self) -> Option<u64> {
        self.sample_count
    }

    /// Returns a probability for an outcome.
    #[must_use]
    pub fn probability_for(
        &self,
        outcome: &str,
    ) -> Option<f64> {
        self.probabilities.get(outcome).copied()
    }

    /// Returns the number of distinct outcomes.
    #[must_use]
    pub fn distinct_outcomes(&self) -> usize {
        self.probabilities.len()
    }
}

// =============================================================================
// Expectation values
// =============================================================================

/// One named expectation value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectationValue {
    /// Observable/operator identifier.
    pub observable: String,

    /// Measured expectation value.
    pub value: f64,

    /// Optional standard error.
    pub standard_error: Option<f64>,

    /// Number of samples used to estimate the expectation value.
    pub samples: Option<u64>,
}

impl ExpectationValue {
    /// Creates an expectation value.
    pub fn new<S: Into<String>>(
        observable: S,
        value: f64,
    ) -> Result<Self, ObservationError> {
        Self::with_statistics(
            observable,
            value,
            None,
            None,
        )
    }

    /// Creates an expectation value with optional uncertainty/sample count.
    pub fn with_statistics<S: Into<String>>(
        observable: S,
        value: f64,
        standard_error: Option<f64>,
        samples: Option<u64>,
    ) -> Result<Self, ObservationError> {
        let observable = observable.into();

        validate_identifier(
            "observable",
            &observable,
        )?;

        if !value.is_finite() {
            return Err(ObservationError::NonFiniteValue {
                field: "expectation.value",
            });
        }

        if !(-1.0..=1.0).contains(&value) {
            return Err(ObservationError::InvalidExpectation {
                value,
            });
        }

        if let Some(error) = standard_error {
            if !error.is_finite() || error < 0.0 {
                return Err(ObservationError::NonFiniteValue {
                    field: "expectation.standard_error",
                });
            }
        }

        if let Some(samples) = samples {
            if samples == 0 {
                return Err(ObservationError::InvalidCount {
                    field: "expectation.samples",
                });
            }
        }

        Ok(Self {
            observable,
            value,
            standard_error,
            samples,
        })
    }
}

/// Collection of expectation-value observations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectationValuesObservation {
    values: Vec<ExpectationValue>,
}

impl ExpectationValuesObservation {
    /// Creates expectation values.
    pub fn new(
        values: Vec<ExpectationValue>,
    ) -> Result<Self, ObservationError> {
        if values.is_empty() {
            return Err(ObservationError::InvalidCount {
                field: "expectation_values",
            });
        }

        if values.len() > MAX_EXPECTATION_VALUES {
            return Err(ObservationError::LimitExceeded {
                resource: "expectation_values",
                requested: values.len() as u64,
                maximum: MAX_EXPECTATION_VALUES as u64,
            });
        }

        Ok(Self { values })
    }

    /// Returns the expectation values.
    #[must_use]
    pub fn values(&self) -> &[ExpectationValue] {
        &self.values
    }
}

// =============================================================================
// State vectors
// =============================================================================

/// State-vector observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateVectorObservation {
    amplitudes: Vec<Complex64>,
    qubits: usize,
}

impl StateVectorObservation {
    /// Creates a state-vector observation.
    ///
    /// The vector length must equal `2^qubits`.
    pub fn new(
        qubits: usize,
        amplitudes: Vec<Complex64>,
    ) -> Result<Self, ObservationError> {
        Self::with_limits(
            qubits,
            amplitudes,
            &BenchmarkLimits::default(),
        )
    }

    /// Creates a state vector under explicit resource limits.
    pub fn with_limits(
        qubits: usize,
        amplitudes: Vec<Complex64>,
        limits: &BenchmarkLimits,
    ) -> Result<Self, ObservationError> {
        limits.check_qubits(qubits)?;

        let expected = checked_power_of_two(
            qubits,
            "state_vector_dimension",
        )?;

        if amplitudes.len() != expected {
            return Err(ObservationError::InvalidStateDimension {
                length: amplitudes.len(),
            });
        }

        let elements = u64::try_from(amplitudes.len()).map_err(
            |_| ObservationError::ArithmeticOverflow {
                resource: "state_vector_elements",
            },
        )?;

        limits.check_observations(elements)?;

        Ok(Self {
            amplitudes,
            qubits,
        })
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Returns amplitudes in computational-basis order.
    #[must_use]
    pub fn amplitudes(&self) -> &[Complex64] {
        &self.amplitudes
    }

    /// Returns the number of amplitudes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.amplitudes.len()
    }

    /// Returns whether the vector is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.amplitudes.is_empty()
    }
}

// =============================================================================
// Density matrices
// =============================================================================

/// Density-matrix observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DensityMatrixObservation {
    entries: Vec<Complex64>,
    dimension: usize,
    qubits: usize,
}

impl DensityMatrixObservation {
    /// Creates a density matrix.
    ///
    /// The matrix must be square with dimension `2^qubits`.
    pub fn new(
        qubits: usize,
        entries: Vec<Complex64>,
    ) -> Result<Self, ObservationError> {
        Self::with_limits(
            qubits,
            entries,
            &BenchmarkLimits::default(),
        )
    }

    /// Creates a density matrix under explicit limits.
    pub fn with_limits(
        qubits: usize,
        entries: Vec<Complex64>,
        limits: &BenchmarkLimits,
    ) -> Result<Self, ObservationError> {
        limits.check_qubits(qubits)?;

        let dimension = checked_power_of_two(
            qubits,
            "density_matrix_dimension",
        )?;

        let elements = checked_mul_usize(
            dimension,
            dimension,
            "density_matrix_elements",
        )?;

        if entries.len() != elements {
            return Err(ObservationError::InvalidDensityMatrix {
                rows: dimension,
                columns: if dimension == 0 {
                    0
                } else {
                    entries.len() / dimension
                },
            });
        }

        let elements_u64 = u64::try_from(elements).map_err(
            |_| ObservationError::ArithmeticOverflow {
                resource: "density_matrix_elements",
            },
        )?;

        if elements_u64 > MAX_MATRIX_ELEMENTS {
            return Err(ObservationError::LimitExceeded {
                resource: "density_matrix_elements",
                requested: elements_u64,
                maximum: MAX_MATRIX_ELEMENTS,
            });
        }

        limits.check_observations(elements_u64)?;

        Ok(Self {
            entries,
            dimension,
            qubits,
        })
    }

    /// Returns the logical qubit count.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Returns the matrix dimension.
    #[must_use]
    pub const fn dimension(&self) -> usize {
        self.dimension
    }

    /// Returns row-major matrix entries.
    #[must_use]
    pub fn entries(&self) -> &[Complex64] {
        &self.entries
    }

    /// Returns the matrix element `(row, column)`.
    #[must_use]
    pub fn get(
        &self,
        row: usize,
        column: usize,
    ) -> Option<Complex64> {
        if row >= self.dimension || column >= self.dimension {
            return None;
        }

        self.entries
            .get(row * self.dimension + column)
            .copied()
    }
}

// =============================================================================
// Analog observations
// =============================================================================

/// One analog sample.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalogSample {
    /// Sample channels.
    pub channels: Vec<f64>,

    /// Optional acquisition timestamp relative to experiment start.
    pub timestamp: Option<Duration>,
}

impl AnalogSample {
    /// Creates an analog sample.
    pub fn new(
        channels: Vec<f64>,
    ) -> Result<Self, ObservationError> {
        Self::with_timestamp(channels, None)
    }

    /// Creates an analog sample with an optional timestamp.
    pub fn with_timestamp(
        channels: Vec<f64>,
        timestamp: Option<Duration>,
    ) -> Result<Self, ObservationError> {
        if channels.is_empty() {
            return Err(ObservationError::InvalidCount {
                field: "analog.channels",
            });
        }

        if channels.iter().any(|value| !value.is_finite()) {
            return Err(ObservationError::NonFiniteValue {
                field: "analog.channel",
            });
        }

        Ok(Self {
            channels,
            timestamp,
        })
    }
}

/// Analog measurement observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalogObservation {
    samples: Vec<AnalogSample>,
    sample_rate_hz: Option<f64>,
}

impl AnalogObservation {
    /// Creates an analog observation.
    pub fn new(
        samples: Vec<AnalogSample>,
    ) -> Result<Self, ObservationError> {
        Self::with_sample_rate(samples, None)
    }

    /// Creates an analog observation with an optional sample rate.
    pub fn with_sample_rate(
        samples: Vec<AnalogSample>,
        sample_rate_hz: Option<f64>,
    ) -> Result<Self, ObservationError> {
        if samples.is_empty() {
            return Err(ObservationError::InvalidCount {
                field: "analog.samples",
            });
        }

        if samples.len() > MAX_ANALOG_SAMPLES {
            return Err(ObservationError::LimitExceeded {
                resource: "analog_samples",
                requested: samples.len() as u64,
                maximum: MAX_ANALOG_SAMPLES as u64,
            });
        }

        if let Some(rate) = sample_rate_hz {
            if !rate.is_finite() || rate <= 0.0 {
                return Err(ObservationError::NonFiniteValue {
                    field: "analog.sample_rate_hz",
                });
            }
        }

        Ok(Self {
            samples,
            sample_rate_hz,
        })
    }

    /// Returns analog samples.
    #[must_use]
    pub fn samples(&self) -> &[AnalogSample] {
        &self.samples
    }

    /// Returns the optional sample rate.
    #[must_use]
    pub const fn sample_rate_hz(&self) -> Option<f64> {
        self.sample_rate_hz
    }
}

// =============================================================================
// Annealing observations
// =============================================================================

/// One annealing/sample-based optimization result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnealingSample {
    /// Binary/spin state.
    pub state: Vec<i8>,

    /// Objective/energy value associated with the sample.
    pub energy: f64,

    /// Number of occurrences of this state.
    pub occurrences: u64,
}

impl AnnealingSample {
    /// Creates an annealing sample.
    pub fn new(
        state: Vec<i8>,
        energy: f64,
        occurrences: u64,
    ) -> Result<Self, ObservationError> {
        if state.is_empty() {
            return Err(ObservationError::InvalidCount {
                field: "annealing.state",
            });
        }

        if !energy.is_finite() {
            return Err(ObservationError::NonFiniteValue {
                field: "annealing.energy",
            });
        }

        if occurrences == 0 {
            return Err(ObservationError::InvalidCount {
                field: "annealing.occurrences",
            });
        }

        if state.iter().any(|value| *value != -1 && *value != 0 && *value != 1)
        {
            return Err(ObservationError::Inconsistent {
                reason: "annealing states must use -1/0/1 values",
            });
        }

        Ok(Self {
            state,
            energy,
            occurrences,
        })
    }
}

/// Annealing observation set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnealingSamplesObservation {
    samples: Vec<AnnealingSample>,
    reads: u64,
}

impl AnnealingSamplesObservation {
    /// Creates annealing observations.
    pub fn new(
        samples: Vec<AnnealingSample>,
        reads: u64,
    ) -> Result<Self, ObservationError> {
        if reads == 0 {
            return Err(ObservationError::InvalidCount {
                field: "annealing.reads",
            });
        }

        if samples.is_empty() {
            return Err(ObservationError::InvalidCount {
                field: "annealing.samples",
            });
        }

        if samples.len() > MAX_ANNEALING_SAMPLES {
            return Err(ObservationError::LimitExceeded {
                resource: "annealing_samples",
                requested: samples.len() as u64,
                maximum: MAX_ANNEALING_SAMPLES as u64,
            });
        }

        let mut total_occurrences = 0_u64;

        for sample in &samples {
            total_occurrences = total_occurrences
                .checked_add(sample.occurrences)
                .ok_or(
                    ObservationError::ArithmeticOverflow {
                        resource: "annealing_occurrences",
                    },
                )?;
        }

        if total_occurrences != reads {
            return Err(ObservationError::InvalidCount {
                field: "annealing.reads",
            });
        }

        Ok(Self {
            samples,
            reads,
        })
    }

    /// Returns samples.
    #[must_use]
    pub fn samples(&self) -> &[AnnealingSample] {
        &self.samples
    }

    /// Returns total reads.
    #[must_use]
    pub const fn reads(&self) -> u64 {
        self.reads
    }
}

// =============================================================================
// QEC syndrome observations
// =============================================================================

/// One syndrome measurement record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyndromeRecord {
    /// Syndrome bits in the canonical detector order.
    pub bits: Vec<u8>,

    /// Optional round/index of the syndrome.
    pub round: Option<u64>,

    /// Whether this record is known to be a valid/no-error syndrome.
    pub reference: Option<bool>,
}

impl SyndromeRecord {
    /// Creates a syndrome record.
    pub fn new(
        bits: Vec<u8>,
    ) -> Result<Self, ObservationError> {
        Self::with_metadata(bits, None, None)
    }

    /// Creates a syndrome record with metadata.
    pub fn with_metadata(
        bits: Vec<u8>,
        round: Option<u64>,
        reference: Option<bool>,
    ) -> Result<Self, ObservationError> {
        if bits.is_empty() {
            return Err(ObservationError::InvalidCount {
                field: "syndrome.bits",
            });
        }

        if bits.len() > MAX_BITSTRING_BITS {
            return Err(ObservationError::BitstringTooLarge {
                bits: bits.len(),
            });
        }

        if bits.iter().any(|bit| *bit > 1) {
            return Err(ObservationError::InvalidSyndromeBit);
        }

        Ok(Self {
            bits,
            round,
            reference,
        })
    }
}

/// QEC syndrome observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyndromeObservation {
    records: Vec<SyndromeRecord>,
    rounds: u64,
}

impl SyndromeObservation {
    /// Creates syndrome observations.
    pub fn new(
        records: Vec<SyndromeRecord>,
        rounds: u64,
    ) -> Result<Self, ObservationError> {
        if rounds == 0 {
            return Err(ObservationError::InvalidCount {
                field: "syndrome.rounds",
            });
        }

        if records.is_empty() {
            return Err(ObservationError::InvalidCount {
                field: "syndrome.records",
            });
        }

        if records.len() > MAX_SYNDROME_RECORDS {
            return Err(ObservationError::LimitExceeded {
                resource: "syndrome_records",
                requested: records.len() as u64,
                maximum: MAX_SYNDROME_RECORDS as u64,
            });
        }

        Ok(Self {
            records,
            rounds,
        })
    }

    /// Returns syndrome records.
    #[must_use]
    pub fn records(&self) -> &[SyndromeRecord] {
        &self.records
    }

    /// Returns the number of syndrome rounds.
    #[must_use]
    pub const fn rounds(&self) -> u64 {
        self.rounds
    }
}

// =============================================================================
// Timing observations
// =============================================================================

/// A single timing component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimingComponent {
    /// Component name.
    pub name: TimingKind,

    /// Duration.
    pub duration: Duration,
}

impl TimingComponent {
    /// Creates a timing component.
    pub fn new(
        name: TimingKind,
        duration: Duration,
    ) -> Self {
        Self { name, duration }
    }
}

/// Standardized execution timing components.
///
/// These are intentionally separate because benchmark analysis must not
/// collapse queue, compilation, execution, and readout time into one number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimingKind {
    /// Client-side request construction.
    Preparation,

    /// Compilation/transformation time.
    Compilation,

    /// Transpilation time.
    Transpilation,

    /// Routing time.
    Routing,

    /// Scheduling time.
    Scheduling,

    /// Backend queue delay.
    Queue,

    /// Submission/network transfer time.
    Submission,

    /// Actual quantum execution time.
    Execution,

    /// Measurement/readout time.
    Readout,

    /// Classical post-processing time.
    PostProcessing,

    /// Benchmark statistical analysis time.
    Analysis,

    /// Complete end-to-end wall-clock duration.
    Total,

    /// Backend-defined timing component.
    Custom,
}

/// Timing observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimingObservation {
    components: Vec<TimingComponent>,
}

impl TimingObservation {
    /// Creates a timing observation.
    pub fn new(
        components: Vec<TimingComponent>,
    ) -> Result<Self, ObservationError> {
        if components.is_empty() {
            return Err(ObservationError::InvalidCount {
                field: "timing.components",
            });
        }

        // 4,096 timing fields is comfortably above expected backend needs and
        // prevents malformed metadata from creating unbounded structures.
        if components.len() > 4_096 {
            return Err(ObservationError::LimitExceeded {
                resource: "timing_components",
                requested: components.len() as u64,
                maximum: 4_096,
            });
        }

        Ok(Self { components })
    }

    /// Returns timing components.
    #[must_use]
    pub fn components(&self) -> &[TimingComponent] {
        &self.components
    }

    /// Returns the first component of a requested kind.
    #[must_use]
    pub fn get(
        &self,
        kind: TimingKind,
    ) -> Option<Duration> {
        self.components
            .iter()
            .find(|component| component.name == kind)
            .map(|component| component.duration)
    }

    /// Returns the sum of all timing components.
    ///
    /// This is descriptive only. It must not be interpreted as an end-to-end
    /// time unless the caller knows the components are non-overlapping.
    pub fn total_component_duration(
        &self,
    ) -> Option<Duration> {
        self.components.iter().try_fold(
            Duration::ZERO,
            |total, component| total.checked_add(component.duration),
        )
    }
}

// =============================================================================
// Calibration observations
// =============================================================================

/// Typed calibration value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CalibrationValue {
    /// Scalar numeric calibration.
    Scalar(f64),

    /// Numeric vector.
    Vector(Vec<f64>),

    /// Textual calibration identifier/value.
    Text(String),

    /// Boolean calibration state.
    Boolean(bool),
}

impl CalibrationValue {
    /// Validates a calibration value.
    pub fn validate(
        &self,
        field: &str,
    ) -> Result<(), ObservationError> {
        match self {
            Self::Scalar(value) => {
                if !value.is_finite() {
                    return Err(
                        ObservationError::InvalidCalibrationValue {
                            field: field.to_owned(),
                        },
                    );
                }
            }

            Self::Vector(values) => {
                if values.iter().any(|value| !value.is_finite()) {
                    return Err(
                        ObservationError::InvalidCalibrationValue {
                            field: field.to_owned(),
                        },
                    );
                }
            }

            Self::Text(_) | Self::Boolean(_) => {}
        }

        Ok(())
    }
}

/// Calibration snapshot observed with a benchmark execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrationObservation {
    fields: BTreeMap<String, CalibrationValue>,
}

impl CalibrationObservation {
    /// Creates a calibration snapshot.
    pub fn new(
        fields: BTreeMap<String, CalibrationValue>,
    ) -> Result<Self, ObservationError> {
        if fields.len() > MAX_CALIBRATION_FIELDS {
            return Err(ObservationError::LimitExceeded {
                resource: "calibration_fields",
                requested: fields.len() as u64,
                maximum: MAX_CALIBRATION_FIELDS as u64,
            });
        }

        for (field, value) in &fields {
            validate_identifier(
                "calibration_field",
                field,
            )?;

            value.validate(field)?;
        }

        Ok(Self { fields })
    }

    /// Returns calibration fields.
    #[must_use]
    pub fn fields(
        &self,
    ) -> &BTreeMap<String, CalibrationValue> {
        &self.fields
    }

    /// Returns one calibration field.
    #[must_use]
    pub fn get(
        &self,
        field: &str,
    ) -> Option<&CalibrationValue> {
        self.fields.get(field)
    }
}

// =============================================================================
// Backend metadata
// =============================================================================

/// Backend metadata value.
///
/// Metadata is descriptive and must not be interpreted as a benchmark metric
/// by this module.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetadataValue {
    /// Textual value.
    Text(String),

    /// Integer value.
    Integer(i64),

    /// Floating-point value.
    Float(f64),

    /// Boolean value.
    Boolean(bool),
}

impl MetadataValue {
    /// Validates a metadata value.
    pub fn validate(
        &self,
        field: &str,
    ) -> Result<(), ObservationError> {
        if let Self::Float(value) = self {
            if !value.is_finite() {
                return Err(
                    ObservationError::NonFiniteValue {
                        field: Box::leak(
                            field.to_owned().into_boxed_str(),
                        ),
                    },
                );
            }
        }

        Ok(())
    }
}

/// Backend metadata observed during execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackendMetadataObservation {
    fields: BTreeMap<String, MetadataValue>,
}

impl BackendMetadataObservation {
    /// Creates backend metadata.
    pub fn new(
        fields: BTreeMap<String, MetadataValue>,
    ) -> Result<Self, ObservationError> {
        if fields.len() > MAX_METADATA_FIELDS {
            return Err(ObservationError::LimitExceeded {
                resource: "backend_metadata_fields",
                requested: fields.len() as u64,
                maximum: MAX_METADATA_FIELDS as u64,
            });
        }

        for (field, value) in &fields {
            validate_identifier(
                "backend_metadata_field",
                field,
            )?;

            value.validate(field)?;
        }

        Ok(Self { fields })
    }

    /// Returns metadata.
    #[must_use]
    pub fn fields(
        &self,
    ) -> &BTreeMap<String, MetadataValue> {
        &self.fields
    }

    /// Returns a metadata field.
    #[must_use]
    pub fn get(
        &self,
        field: &str,
    ) -> Option<&MetadataValue> {
        self.fields.get(field)
    }
}

// =============================================================================
// Custom observations
// =============================================================================

/// JSON-like primitive value for protocol-specific observations.
///
/// This intentionally does not use `serde_json::Value` so the core observation
/// model remains independent of the JSON representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomValue {
    /// Null value.
    Null,

    /// Boolean.
    Boolean(bool),

    /// Integer.
    Integer(i64),

    /// Finite floating-point value.
    Float(f64),

    /// String.
    String(String),

    /// Ordered values.
    Array(Vec<CustomValue>),

    /// Deterministically ordered object fields.
    Object(BTreeMap<String, CustomValue>),
}

impl CustomValue {
    /// Validates this custom value recursively.
    pub fn validate(&self) -> Result<(), ObservationError> {
        match self {
            Self::Null
            | Self::Boolean(_)
            | Self::Integer(_)
            | Self::String(_) => Ok(()),

            Self::Float(value) => {
                if !value.is_finite() {
                    return Err(
                        ObservationError::NonFiniteValue {
                            field: "custom.float",
                        },
                    );
                }

                Ok(())
            }

            Self::Array(values) => {
                if values.len() > MAX_CUSTOM_FIELDS {
                    return Err(
                        ObservationError::LimitExceeded {
                            resource: "custom_array",
                            requested: values.len() as u64,
                            maximum: MAX_CUSTOM_FIELDS as u64,
                        },
                    );
                }

                for value in values {
                    value.validate()?;
                }

                Ok(())
            }

            Self::Object(fields) => {
                if fields.len() > MAX_CUSTOM_FIELDS {
                    return Err(
                        ObservationError::LimitExceeded {
                            resource: "custom_object",
                            requested: fields.len() as u64,
                            maximum: MAX_CUSTOM_FIELDS as u64,
                        },
                    );
                }

                for (field, value) in fields {
                    validate_identifier(
                        "custom_field",
                        field,
                    )?;

                    value.validate()?;
                }

                Ok(())
            }
        }
    }
}

/// Protocol-specific extensible observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomObservation {
    /// Stable protocol-specific type identifier.
    pub type_id: String,

    /// Version of that custom payload.
    pub version: u16,

    /// Payload.
    pub payload: CustomValue,
}

impl CustomObservation {
    /// Creates a custom observation.
    pub fn new<S: Into<String>>(
        type_id: S,
        version: u16,
        payload: CustomValue,
    ) -> Result<Self, ObservationError> {
        let type_id = type_id.into();

        validate_identifier(
            "custom.type_id",
            &type_id,
        )?;

        payload.validate()?;

        Ok(Self {
            type_id,
            version,
            payload,
        })
    }
}

// =============================================================================
// Observation payload
// =============================================================================

/// Raw payload associated with one observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObservationPayload {
    /// Computational-basis counts.
    Counts(CountsObservation),

    /// Probability distribution.
    Probabilities(ProbabilityDistribution),

    /// Expectation values.
    ExpectationValues(ExpectationValuesObservation),

    /// State vector.
    StateVector(StateVectorObservation),

    /// Density matrix.
    DensityMatrix(DensityMatrixObservation),

    /// Analog measurements.
    Analog(AnalogObservation),

    /// Annealing samples.
    AnnealingSamples(AnnealingSamplesObservation),

    /// QEC syndrome records.
    Syndrome(SyndromeObservation),

    /// Timing data.
    Timing(TimingObservation),

    /// Calibration snapshot.
    Calibration(CalibrationObservation),

    /// Backend metadata.
    BackendMetadata(BackendMetadataObservation),

    /// Protocol-specific custom payload.
    Custom(CustomObservation),
}

impl ObservationPayload {
    /// Returns the semantic kind of the payload.
    #[must_use]
    pub fn kind(&self) -> ObservationKind {
        match self {
            Self::Counts(_) => ObservationKind::Counts,
            Self::Probabilities(_) => ObservationKind::Probabilities,
            Self::ExpectationValues(_) => {
                ObservationKind::ExpectationValues
            }
            Self::StateVector(_) => ObservationKind::StateVector,
            Self::DensityMatrix(_) => {
                ObservationKind::DensityMatrix
            }
            Self::Analog(_) => ObservationKind::Analog,
            Self::AnnealingSamples(_) => {
                ObservationKind::AnnealingSamples
            }
            Self::Syndrome(_) => ObservationKind::Syndrome,
            Self::Timing(_) => ObservationKind::Timing,
            Self::Calibration(_) => ObservationKind::Calibration,
            Self::BackendMetadata(_) => {
                ObservationKind::BackendMetadata
            }
            Self::Custom(custom) => {
                ObservationKind::Custom(custom.type_id.clone())
            }
        }
    }

    /// Returns whether the payload is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Counts(value) => value.shots() == 0,
            Self::Probabilities(value) => {
                value.probabilities().is_empty()
            }
            Self::ExpectationValues(value) => {
                value.values().is_empty()
            }
            Self::StateVector(value) => value.is_empty(),
            Self::DensityMatrix(value) => value.entries().is_empty(),
            Self::Analog(value) => value.samples().is_empty(),
            Self::AnnealingSamples(value) => {
                value.samples().is_empty()
            }
            Self::Syndrome(value) => value.records().is_empty(),
            Self::Timing(value) => value.components().is_empty(),
            Self::Calibration(value) => value.fields().is_empty(),
            Self::BackendMetadata(value) => value.fields().is_empty(),
            Self::Custom(_) => false,
        }
    }
}

// =============================================================================
// Observation status
// =============================================================================

/// Completion status of an observation/execution result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObservationStatus {
    /// Complete result; all requested observations are available.
    Complete,

    /// Execution completed but only part of the requested data was returned.
    Partial,

    /// Execution was cancelled before completion.
    Cancelled,

    /// Execution timed out.
    TimedOut,

    /// Backend reported an execution failure.
    Failed,
}

impl ObservationStatus {
    /// Returns whether this status represents a complete observation.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns whether this status represents usable partial data.
    #[must_use]
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::Partial)
    }

    /// Returns whether this status represents an execution failure.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(
            self,
            Self::Cancelled
                | Self::TimedOut
                | Self::Failed
        )
    }
}

// =============================================================================
// Observation diagnostics
// =============================================================================

/// Severity of an observation diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObservationDiagnosticSeverity {
    /// Informational condition.
    Info,

    /// Result is usable but has an important qualification.
    Warning,

    /// Observation cannot be trusted without intervention.
    Error,
}

/// Structured observation diagnostic.
///
/// Diagnostics are data, not logging. The caller decides whether/how to
/// present them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationDiagnostic {
    /// Diagnostic severity.
    pub severity: ObservationDiagnosticSeverity,

    /// Stable diagnostic code.
    pub code: String,

    /// Human-readable description.
    pub message: String,
}

impl ObservationDiagnostic {
    /// Creates a diagnostic.
    pub fn new<C, M>(
        severity: ObservationDiagnosticSeverity,
        code: C,
        message: M,
    ) -> Result<Self, ObservationError>
    where
        C: Into<String>,
        M: Into<String>,
    {
        let code = code.into();

        validate_identifier(
            "diagnostic.code",
            &code,
        )?;

        let message = message.into();

        if message.is_empty() {
            return Err(ObservationError::EmptyIdentifier {
                field: "diagnostic.message",
            });
        }

        Ok(Self {
            severity,
            code,
            message,
        })
    }
}

// =============================================================================
// Observation envelope
// =============================================================================

/// Canonical normalized raw observation.
///
/// This is the primary type consumed by `execution::response`,
/// `statistics::*`, `metrics::*`, and `core::result`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    /// Observation schema version.
    schema_version: u16,

    /// Stable observation identity.
    id: ObservationId,

    /// Logical kind of the observation.
    kind: ObservationKind,

    /// Raw observation payload.
    payload: ObservationPayload,

    /// Number of requested shots/samples when applicable.
    ///
    /// This is separate from payload-specific sample counts because some
    /// observations, such as timing/calibration data, do not have shots.
    samples: Option<u64>,

    /// Execution status associated with the observation.
    status: ObservationStatus,

    /// Optional backend execution sequence number.
    sequence: Option<u64>,

    /// Structured diagnostics.
    diagnostics: Vec<ObservationDiagnostic>,
}

impl Observation {
    /// Creates a complete observation.
    pub fn new(
        id: ObservationId,
        payload: ObservationPayload,
    ) -> Result<Self, ObservationError> {
        Self::with_metadata(
            id,
            payload,
            None,
            ObservationStatus::Complete,
            None,
            Vec::new(),
        )
    }

    /// Creates an observation with execution metadata.
    pub fn with_metadata(
        id: ObservationId,
        payload: ObservationPayload,
        samples: Option<u64>,
        status: ObservationStatus,
        sequence: Option<u64>,
        diagnostics: Vec<ObservationDiagnostic>,
    ) -> Result<Self, ObservationError> {
        if let Some(samples) = samples {
            if samples == 0 && status == ObservationStatus::Complete {
                return Err(ObservationError::InvalidCount {
                    field: "samples",
                });
            }
        }

        if diagnostics.len()
            > BenchmarkLimits::default().max_diagnostics
        {
            return Err(ObservationError::LimitExceeded {
                resource: "diagnostics",
                requested: diagnostics.len() as u64,
                maximum: BenchmarkLimits::default()
                    .max_diagnostics as u64,
            });
        }

        let observation = Self {
            schema_version: OBSERVATION_SCHEMA_VERSION,
            id,
            kind: payload.kind(),
            payload,
            samples,
            status,
            sequence,
            diagnostics,
        };

        observation.validate()?;

        Ok(observation)
    }

    /// Returns the observation schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the stable observation identifier.
    #[must_use]
    pub fn id(&self) -> &ObservationId {
        &self.id
    }

    /// Returns the observation kind.
    #[must_use]
    pub fn kind(&self) -> &ObservationKind {
        &self.kind
    }

    /// Returns the raw payload.
    #[must_use]
    pub fn payload(&self) -> &ObservationPayload {
        &self.payload
    }

    /// Returns the optional sample count.
    #[must_use]
    pub const fn samples(&self) -> Option<u64> {
        self.samples
    }

    /// Returns execution status.
    #[must_use]
    pub const fn status(&self) -> ObservationStatus {
        self.status
    }

    /// Returns the optional execution sequence.
    #[must_use]
    pub const fn sequence(&self) -> Option<u64> {
        self.sequence
    }

    /// Returns diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[ObservationDiagnostic] {
        &self.diagnostics
    }

    /// Returns whether the observation is complete.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Returns whether the observation contains partial data.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        self.status.is_partial()
    }

    /// Returns whether execution failed.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        self.status.is_failure()
    }

    /// Validates the complete observation envelope.
    ///
    /// This MUST be called after deserializing observations from an untrusted
    /// or external source before they are passed into benchmark analysis.
    pub fn validate(&self) -> Result<(), ObservationError> {
        if self.schema_version == 0 {
            return Err(ObservationError::Inconsistent {
                reason: "schema version must be non-zero",
            });
        }

        validate_identifier(
            "observation_id",
            self.id.as_str(),
        )?;

        if self.kind != self.payload.kind() {
            return Err(ObservationError::Inconsistent {
                reason: "observation kind does not match payload",
            });
        }

        if let Some(samples) = self.samples {
            if samples == 0
                && self.status == ObservationStatus::Complete
            {
                return Err(ObservationError::InvalidCount {
                    field: "samples",
                });
            }
        }

        if self.diagnostics.len()
            > BenchmarkLimits::default().max_diagnostics
        {
            return Err(ObservationError::LimitExceeded {
                resource: "diagnostics",
                requested: self.diagnostics.len() as u64,
                maximum: BenchmarkLimits::default()
                    .max_diagnostics as u64,
            });
        }

        validate_payload(&self.payload)?;

        Ok(())
    }

    /// Returns the computational-basis counts, if this is a counts
    /// observation.
    #[must_use]
    pub fn counts(&self) -> Option<&CountsObservation> {
        match &self.payload {
            ObservationPayload::Counts(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the probability distribution, if available.
    #[must_use]
    pub fn probabilities(
        &self,
    ) -> Option<&ProbabilityDistribution> {
        match &self.payload {
            ObservationPayload::Probabilities(value) => Some(value),
            _ => None,
        }
    }

    /// Returns expectation values, if available.
    #[must_use]
    pub fn expectation_values(
        &self,
    ) -> Option<&ExpectationValuesObservation> {
        match &self.payload {
            ObservationPayload::ExpectationValues(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    /// Returns state-vector data, if available.
    #[must_use]
    pub fn state_vector(
        &self,
    ) -> Option<&StateVectorObservation> {
        match &self.payload {
            ObservationPayload::StateVector(value) => Some(value),
            _ => None,
        }
    }

    /// Returns density-matrix data, if available.
    #[must_use]
    pub fn density_matrix(
        &self,
    ) -> Option<&DensityMatrixObservation> {
        match &self.payload {
            ObservationPayload::DensityMatrix(value) => Some(value),
            _ => None,
        }
    }

    /// Returns timing data, if available.
    #[must_use]
    pub fn timing(
        &self,
    ) -> Option<&TimingObservation> {
        match &self.payload {
            ObservationPayload::Timing(value) => Some(value),
            _ => None,
        }
    }

    /// Returns a JSON representation.
    ///
    /// This method is deliberately placed here as a convenience for the
    /// reporting/interchange boundary. It does not define the canonical
    /// benchmark-result schema.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserializes and validates an observation.
    ///
    /// Callers handling untrusted benchmark result data should use this
    /// instead of `serde_json::from_str::<Observation>`.
    pub fn from_json(
        value: &str,
    ) -> Result<Self, ObservationJsonError> {
        let observation: Self =
            serde_json::from_str(value)
                .map_err(ObservationJsonError::Deserialize)?;

        observation
            .validate()
            .map_err(ObservationJsonError::Invalid)?;

        Ok(observation)
    }
}

/// Errors returned when parsing serialized observations.
#[derive(Debug)]
pub enum ObservationJsonError {
    /// Serialization/deserialization failed.
    Deserialize(serde_json::Error),

    /// Serialization succeeded but semantic validation failed.
    Invalid(ObservationError),
}

impl fmt::Display for ObservationJsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deserialize(error) => {
                write!(
                    f,
                    "invalid serialized observation: {error}"
                )
            }

            Self::Invalid(error) => {
                write!(
                    f,
                    "invalid observation semantics: {error}"
                )
            }
        }
    }
}

impl std::error::Error for ObservationJsonError {}

// =============================================================================
// Observation set
// =============================================================================

/// Collection of observations belonging to one execution response.
///
/// Ordering is preserved because execution sequence can be scientifically
/// meaningful, particularly for drift, calibration, randomized benchmarking,
/// QEC rounds, and time-series experiments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservationSet {
    /// Observation schema version.
    schema_version: u16,

    /// Ordered observations.
    observations: Vec<Observation>,

    /// Whether all expected observations were received.
    complete: bool,
}

impl ObservationSet {
    /// Creates an empty complete set.
    pub fn empty() -> Self {
        Self {
            schema_version: OBSERVATION_SCHEMA_VERSION,
            observations: Vec::new(),
            complete: true,
        }
    }

    /// Creates a set from observations.
    pub fn new(
        observations: Vec<Observation>,
        complete: bool,
    ) -> Result<Self, ObservationError> {
        Self::with_limits(
            observations,
            complete,
            &BenchmarkLimits::default(),
        )
    }

    /// Creates an observation set under an explicit resource policy.
    pub fn with_limits(
        observations: Vec<Observation>,
        complete: bool,
        limits: &BenchmarkLimits,
    ) -> Result<Self, ObservationError> {
        limits.check_observations(
            u64::try_from(observations.len()).map_err(
                |_| ObservationError::ArithmeticOverflow {
                    resource: "observations",
                },
            )?,
        )?;

        for observation in &observations {
            observation.validate()?;
        }

        Ok(Self {
            schema_version: OBSERVATION_SCHEMA_VERSION,
            observations,
            complete,
        })
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns observations in execution order.
    #[must_use]
    pub fn observations(&self) -> &[Observation] {
        &self.observations
    }

    /// Returns the number of observations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns whether there are no observations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    /// Returns whether all expected observations were received.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Adds an observation while enforcing the supplied limits.
    pub fn push(
        &mut self,
        observation: Observation,
        limits: &BenchmarkLimits,
    ) -> Result<(), ObservationError> {
        observation.validate()?;

        let next = self
            .observations
            .len()
            .checked_add(1)
            .ok_or(
                ObservationError::ArithmeticOverflow {
                    resource: "observation_count",
                },
            )?;

        limits.check_observations(
            u64::try_from(next).map_err(|_| {
                ObservationError::ArithmeticOverflow {
                    resource: "observation_count",
                }
            })?,
        )?;

        self.observations.push(observation);

        Ok(())
    }

    /// Marks the set as partial/incomplete.
    pub fn mark_incomplete(&mut self) {
        self.complete = false;
    }

    /// Returns all observations of a particular kind.
    #[must_use]
    pub fn by_kind(
        &self,
        kind: &ObservationKind,
    ) -> Vec<&Observation> {
        self.observations
            .iter()
            .filter(|observation| observation.kind() == kind)
            .collect()
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_payload(
    payload: &ObservationPayload,
) -> Result<(), ObservationError> {
    match payload {
        ObservationPayload::Counts(value) => {
            if value.shots() == 0 {
                // Zero-shot counts are permitted only as explicit partial
                // observations.
                return Ok(());
            }

            let total = value
                .counts()
                .values()
                .try_fold(0_u64, |total, count| {
                    total.checked_add(*count)
                })
                .ok_or(
                    ObservationError::ArithmeticOverflow {
                        resource: "count_total",
                    },
                )?;

            if total != value.shots() {
                return Err(ObservationError::InvalidCount {
                    field: "counts.total",
                });
            }

            Ok(())
        }

        ObservationPayload::Probabilities(value) => {
            ProbabilityDistribution::new_optional_samples(
                value.probabilities.clone(),
                value.sample_count,
            )
            .map(|_| ())
        }

        ObservationPayload::ExpectationValues(value) => {
            for expectation in &value.values {
                ExpectationValue::with_statistics(
                    expectation.observable.clone(),
                    expectation.value,
                    expectation.standard_error,
                    expectation.samples,
                )?;
            }

            Ok(())
        }

        ObservationPayload::StateVector(value) => {
            let expected = checked_power_of_two(
                value.qubits,
                "state_vector_dimension",
            )?;

            if value.amplitudes.len() != expected {
                return Err(
                    ObservationError::InvalidStateDimension {
                        length: value.amplitudes.len(),
                    },
                );
            }

            for amplitude in &value.amplitudes {
                Complex64::new(
                    amplitude.re,
                    amplitude.im,
                )?;
            }

            Ok(())
        }

        ObservationPayload::DensityMatrix(value) => {
            let expected = checked_power_of_two(
                value.qubits,
                "density_matrix_dimension",
            )?;

            let expected_elements = checked_mul_usize(
                expected,
                expected,
                "density_matrix_elements",
            )?;

            if value.dimension != expected
                || value.entries.len() != expected_elements
            {
                return Err(
                    ObservationError::InvalidDensityMatrix {
                        rows: value.dimension,
                        columns: value.dimension,
                    },
                );
            }

            for entry in &value.entries {
                Complex64::new(entry.re, entry.im)?;
            }

            Ok(())
        }

        ObservationPayload::Analog(value) => {
            if value.samples.is_empty() {
                return Err(ObservationError::InvalidCount {
                    field: "analog.samples",
                });
            }

            if let Some(rate) = value.sample_rate_hz {
                if !rate.is_finite() || rate <= 0.0 {
                    return Err(ObservationError::NonFiniteValue {
                        field: "analog.sample_rate_hz",
                    });
                }
            }

            for sample in &value.samples {
                if sample.channels.is_empty() {
                    return Err(ObservationError::InvalidCount {
                        field: "analog.channels",
                    });
                }

                if sample
                    .channels
                    .iter()
                    .any(|channel| !channel.is_finite())
                {
                    return Err(
                        ObservationError::NonFiniteValue {
                            field: "analog.channel",
                        },
                    );
                }
            }

            Ok(())
        }

        ObservationPayload::AnnealingSamples(value) => {
            AnnealingSamplesObservation::new(
                value.samples.clone(),
                value.reads,
            )
            .map(|_| ())
        }

        ObservationPayload::Syndrome(value) => {
            SyndromeObservation::new(
                value.records.clone(),
                value.rounds,
            )
            .map(|_| ())
        }

        ObservationPayload::Timing(value) => {
            if value.components.is_empty() {
                return Err(ObservationError::InvalidCount {
                    field: "timing.components",
                });
            }

            Ok(())
        }

        ObservationPayload::Calibration(value) => {
            CalibrationObservation::new(
                value.fields.clone(),
            )
            .map(|_| ())
        }

        ObservationPayload::BackendMetadata(value) => {
            BackendMetadataObservation::new(
                value.fields.clone(),
            )
            .map(|_| ())
        }

        ObservationPayload::Custom(value) => {
            CustomObservation::new(
                value.type_id.clone(),
                value.version,
                value.payload.clone(),
            )
            .map(|_| ())
        }
    }
}

fn validate_probability(
    field: &'static str,
    value: f64,
) -> Result<(), ObservationError> {
    if !value.is_finite() {
        return Err(ObservationError::NonFiniteValue {
            field,
        });
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(ObservationError::InvalidProbability {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), ObservationError> {
    if value.is_empty() {
        return Err(ObservationError::EmptyIdentifier {
            field,
        });
    }

    // Benchmark identifiers are intentionally conservative because these
    // values become registry keys, report fields, filenames, and language
    // integration identifiers.
    if value.len() > 256 {
        return Err(ObservationError::InvalidIdentifier {
            field,
        });
    }

    let valid = value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'_'
                    | b'-'
                    | b'.'
                    | b':'
                    | b'/'
            )
    });

    if !valid {
        return Err(ObservationError::InvalidIdentifier {
            field,
        });
    }

    Ok(())
}

fn checked_power_of_two(
    exponent: usize,
    resource: &'static str,
) -> Result<usize, ObservationError> {
    if exponent >= usize::BITS as usize {
        return Err(ObservationError::ArithmeticOverflow {
            resource,
        });
    }

    1usize
        .checked_shl(exponent as u32)
        .ok_or(ObservationError::ArithmeticOverflow {
            resource,
        })
}

fn checked_mul_usize(
    left: usize,
    right: usize,
    resource: &'static str,
) -> Result<usize, ObservationError> {
    left.checked_mul(right).ok_or(
        ObservationError::ArithmeticOverflow {
            resource,
        },
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn bitstring(value: &str) -> BitString {
        BitString::from_binary_string(value).unwrap()
    }

    #[test]
    fn bitstring_rejects_non_binary_values() {
        assert!(
            BitString::from_binary_string("0102").is_err()
        );
    }

    #[test]
    fn bitstring_preserves_logical_order() {
        let value = bitstring("0101");

        assert_eq!(value.bits(), &[0, 1, 0, 1]);
        assert_eq!(value.to_binary_string(), "0101");
    }

    #[test]
    fn counts_require_total_to_equal_shots() {
        let result = CountsObservation::new(
            10,
            vec![OutcomeCount::new(
                bitstring("0"),
                9,
            )],
        );

        assert!(result.is_err());
    }

    #[test]
    fn counts_are_deterministically_ordered() {
        let value = CountsObservation::new(
            10,
            vec![
                OutcomeCount::new(bitstring("1"), 5),
                OutcomeCount::new(bitstring("0"), 5),
            ],
        )
        .unwrap();

        let keys = value
            .counts()
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            keys,
            vec!["0".to_owned(), "1".to_owned()]
        );
    }

    #[test]
    fn counts_convert_to_probabilities() {
        let value = CountsObservation::new(
            10,
            vec![
                OutcomeCount::new(bitstring("0"), 7),
                OutcomeCount::new(bitstring("1"), 3),
            ],
        )
        .unwrap();

        let probabilities = value.probabilities().unwrap();

        assert_eq!(
            probabilities.probability_for("0"),
            Some(0.7)
        );

        assert_eq!(
            probabilities.probability_for("1"),
            Some(0.3)
        );
    }

    #[test]
    fn probabilities_must_be_normalized() {
        let mut probabilities = BTreeMap::new();

        probabilities.insert("0".to_owned(), 0.6);
        probabilities.insert("1".to_owned(), 0.6);

        assert!(
            ProbabilityDistribution::new(
                probabilities,
                10,
            )
            .is_err()
        );
    }

    #[test]
    fn expectation_values_are_bounded() {
        assert!(
            ExpectationValue::new(
                "Z",
                1.5,
            )
            .is_err()
        );

        assert!(
            ExpectationValue::new(
                "Z",
                0.5,
            )
            .is_ok()
        );
    }

    #[test]
    fn state_vector_dimension_is_validated() {
        let result = StateVectorObservation::new(
            2,
            vec![
                Complex64::new(1.0, 0.0).unwrap(),
                Complex64::new(0.0, 0.0).unwrap(),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn state_vector_accepts_power_of_two_dimension() {
        let result = StateVectorObservation::new(
            2,
            vec![
                Complex64::new(1.0, 0.0).unwrap(),
                Complex64::new(0.0, 0.0).unwrap(),
                Complex64::new(0.0, 0.0).unwrap(),
                Complex64::new(0.0, 0.0).unwrap(),
            ],
        );

        assert!(result.is_ok());
    }

    #[test]
    fn density_matrix_requires_square_power_of_two_dimension() {
        let result = DensityMatrixObservation::new(
            1,
            vec![
                Complex64::new(1.0, 0.0).unwrap(),
                Complex64::new(0.0, 0.0).unwrap(),
                Complex64::new(0.0, 0.0).unwrap(),
                Complex64::new(1.0, 0.0).unwrap(),
            ],
        );

        assert!(result.is_ok());
    }

    #[test]
    fn syndrome_rejects_non_binary_values() {
        let result = SyndromeRecord::new(vec![0, 1, 2]);

        assert!(result.is_err());
    }

    #[test]
    fn annealing_samples_must_match_reads() {
        let sample = AnnealingSample::new(
            vec![1, -1, 1],
            -2.0,
            4,
        )
        .unwrap();

        let result =
            AnnealingSamplesObservation::new(
                vec![sample],
                3,
            );

        assert!(result.is_err());
    }

    #[test]
    fn timing_components_are_kept_separate() {
        let value = TimingObservation::new(vec![
            TimingComponent::new(
                TimingKind::Compilation,
                Duration::from_millis(10),
            ),
            TimingComponent::new(
                TimingKind::Execution,
                Duration::from_millis(20),
            ),
        ])
        .unwrap();

        assert_eq!(
            value.get(TimingKind::Compilation),
            Some(Duration::from_millis(10))
        );

        assert_eq!(
            value.get(TimingKind::Execution),
            Some(Duration::from_millis(20))
        );
    }

    #[test]
    fn observation_kind_matches_payload() {
        let id = ObservationId::new("test-observation").unwrap();

        let counts = CountsObservation::new(
            2,
            vec![
                OutcomeCount::new(bitstring("0"), 1),
                OutcomeCount::new(bitstring("1"), 1),
            ],
        )
        .unwrap();

        let observation = Observation::new(
            id,
            ObservationPayload::Counts(counts),
        )
        .unwrap();

        assert_eq!(
            observation.kind(),
            &ObservationKind::Counts
        );

        assert!(observation.validate().is_ok());
    }

    #[test]
    fn observation_rejects_kind_payload_mismatch() {
        let id = ObservationId::new("test-observation").unwrap();

        let counts = CountsObservation::new(
            2,
            vec![
                OutcomeCount::new(bitstring("0"), 1),
                OutcomeCount::new(bitstring("1"), 1),
            ],
        )
        .unwrap();

        let mut observation = Observation::new(
            id,
            ObservationPayload::Counts(counts),
        )
        .unwrap();

        observation.kind =
            ObservationKind::Probabilities;

        assert!(observation.validate().is_err());
    }

    #[test]
    fn observation_set_preserves_execution_order() {
        let first_id =
            ObservationId::new("first").unwrap();

        let second_id =
            ObservationId::new("second").unwrap();

        let first = Observation::new(
            first_id,
            ObservationPayload::Timing(
                TimingObservation::new(vec![
                    TimingComponent::new(
                        TimingKind::Execution,
                        Duration::from_millis(1),
                    ),
                ])
                .unwrap(),
            ),
        )
        .unwrap();

        let second = Observation::new(
            second_id,
            ObservationPayload::Timing(
                TimingObservation::new(vec![
                    TimingComponent::new(
                        TimingKind::Execution,
                        Duration::from_millis(2),
                    ),
                ])
                .unwrap(),
            ),
        )
        .unwrap();

        let set =
            ObservationSet::new(
                vec![first, second],
                true,
            )
            .unwrap();

        assert_eq!(
            set.observations()[0].id().as_str(),
            "first"
        );

        assert_eq!(
            set.observations()[1].id().as_str(),
            "second"
        );
    }

    #[test]
    fn json_round_trip_preserves_observation() {
        let id =
            ObservationId::new("round-trip").unwrap();

        let counts = CountsObservation::new(
            4,
            vec![
                OutcomeCount::new(bitstring("00"), 2),
                OutcomeCount::new(bitstring("11"), 2),
            ],
        )
        .unwrap();

        let original = Observation::new(
            id,
            ObservationPayload::Counts(counts),
        )
        .unwrap();

        let json = original.to_json().unwrap();

        let decoded =
            Observation::from_json(&json).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn custom_values_reject_non_finite_numbers() {
        let value = CustomValue::Float(f64::NAN);

        assert!(value.validate().is_err());
    }

    #[test]
    fn invalid_identifier_is_rejected() {
        assert!(
            ObservationId::new("invalid identifier")
                .is_err()
        );

        assert!(
            ObservationId::new("valid.identifier-v1")
                .is_ok()
        );
    }

    #[test]
    fn observation_set_can_be_partial() {
        let mut set = ObservationSet::empty();

        set.mark_incomplete();

        assert!(!set.is_complete());
    }
}