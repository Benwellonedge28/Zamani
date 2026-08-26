//! Zamani Quantum Benchmarking — Physical QEC Error Benchmark.
//!
//! # Purpose
//!
//! This module provides the physical-error benchmarking layer for Zamani's
//! quantum error-correction benchmarking framework.
//!
//! It measures experimentally observed physical error behavior without
//! owning:
//!
//! - QEC execution;
//! - decoder implementation;
//! - QEC resource enforcement;
//! - QPU credentials;
//! - QPU network I/O;
//! - circuit generation;
//! - raw syndrome storage;
//! - raw measurement storage;
//! - backend scheduling;
//! - calibration control.
//!
//! The module consumes bounded physical-error observations and produces a
//! deterministic, serializable benchmark result.
//!
//! # Architectural position
//!
//! ```text
//!                  Zamani Quantum IR
//!                         │
//!                         ▼
//!                  QEC execution
//!                         │
//!              ┌──────────┴──────────┐
//!              │                     │
//!              ▼                     ▼
//!        QEC metrics            QEC observations
//!              │                     │
//!              └──────────┬──────────┘
//!                         ▼
//!              benchmarking::qec::physical
//!                         │
//!          ┌──────────────┼──────────────┐
//!          ▼              ▼              ▼
//!     error rates     confidence     diagnostics
//!          │              │              │
//!          └──────────────┼──────────────┘
//!                         ▼
//!                  PhysicalQecResult
//!                         │
//!             ┌───────────┼────────────┐
//!             ▼           ▼            ▼
//!          metrics      reports      baseline
//! ```
//!
//! # Ownership boundary
//!
//! `physical.rs` owns:
//!
//! - physical-error observation schema;
//! - error-category classification;
//! - bounded observation aggregation;
//! - physical error-rate calculation;
//! - Wilson confidence intervals;
//! - per-qubit aggregation;
//! - per-round aggregation;
//! - per-error-kind aggregation;
//! - leakage/erasure accounting;
//! - measurement/readout accounting;
//! - correlation indicators;
//! - deterministic result construction;
//! - benchmark validity and warnings.
//!
//! It does NOT own:
//!
//! - QEC execution;
//! - decoder correctness;
//! - logical-error analysis;
//! - threshold fitting;
//! - decoder latency benchmarking;
//! - resource admission;
//! - memory management;
//! - QPU submission;
//! - calibration mutation.
//!
//! # Integration contract
//!
//! The module is designed to integrate with the existing QEC subsystem:
//!
//! ```text
//! quantum::error_correction::metrics
//!                  │
//!                  ▼
//!          PhysicalObservation
//!                  │
//!                  ▼
//!     PhysicalBenchmark::observe
//!                  │
//!                  ▼
//!       PhysicalQecAccumulator
//!                  │
//!                  ▼
//!       PhysicalQecResult
//! ```
//!
//! The existing QEC metrics implementation already owns aggregate execution
//! metrics and physical/logical error counters. This module deliberately
//! consumes those observations instead of duplicating QEC execution logic.
//!
//! # Statistical contract
//!
//! For a binary physical-error observation:
//!
//!     error_rate = errors / opportunities
//!
//! The default confidence interval is the Wilson score interval.
//!
//! A confidence interval is never produced when there are zero opportunities.
//! A zero-error experiment therefore has:
//!
//!     point estimate = 0
//!     interval = mathematically valid interval based on sample count
//!
//! rather than being treated as "no errors means no uncertainty".
//!
//! # Important scientific rule
//!
//! Physical error rate is not one universal physical constant.
//!
//! The result preserves the error category and measurement basis so that:
//!
//! - gate errors;
//! - measurement errors;
//! - preparation errors;
//! - leakage;
//! - erasure;
//! - X errors;
//! - Y errors;
//! - Z errors;
//! - correlated errors;
//! - unknown/other errors
//!
//! are not silently mixed.
//!
//! # Security/resource contract
//!
//! All externally supplied collection sizes are bounded.
//!
//! The accumulator never stores raw syndrome or measurement payloads.
//! It stores only aggregate counts.
//!
//! This prevents benchmark configuration from turning this module into an
//! unbounded memory sink.
//!
//! # Determinism contract
//!
//! The mathematical result depends only on the supplied observations and
//! configuration.
//!
//! No wall clock, thread scheduling, hash-map iteration order, or process
//! global state participates in result calculation.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1 / Rust 2021.
//!
//! No nightly features are required.
//!
//! The implementation intentionally uses only stable standard-library
//! functionality plus the repository's already-present `serde` dependency.
//!
//! # Future integration
//!
//! Later benchmarking modules can consume this result without changing this
//! module:
//!
//! - `metrics/logical.rs` can consume logical observations separately;
//! - `qec/threshold.rs` can consume `PhysicalQecResult` across code distances;
//! - `qec/resource_overhead.rs` can combine physical counts with logical
//!   counts;
//! - `analysis/compare.rs` can compare `PhysicalQecResult` values;
//! - `reporting/json.rs` can serialize the result directly;
//! - `reporting/markdown.rs` can render the same result;
//! - `registry/builtin.rs` can register this benchmark by its stable ID.
//!
//! This module therefore intentionally does not depend on any of those
//! downstream modules.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ============================================================================
// Public constants
// ============================================================================

/// Stable benchmark identifier.
pub const PHYSICAL_QEC_BENCHMARK_ID: &str = "qec.physical";

/// Current benchmark protocol version.
///
/// Increment when the mathematical meaning of a result changes.
pub const PHYSICAL_QEC_BENCHMARK_VERSION: u32 = 1;

/// Maximum number of separately tracked qubits.
///
/// This is a benchmark-input safety bound, not a QEC execution limit.
pub const MAX_TRACKED_QUBITS: usize = 1_000_000;

/// Maximum number of separately tracked rounds.
pub const MAX_TRACKED_ROUNDS: usize = 1_000_000;

/// Maximum number of custom error categories.
pub const MAX_CUSTOM_ERROR_KINDS: usize = 4_096;

/// Maximum observation records accepted by one accumulator.
///
/// The accumulator stores aggregates, not the records themselves, but this
/// protects callers against accidental unbounded ingestion loops.
pub const MAX_OBSERVATIONS: u64 = 10_000_000_000;

/// Default confidence level.
///
/// 0.95 is intentionally a general statistical default. Protocols such as
/// Quantum Volume may select a different protocol-specific confidence level.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Minimum accepted confidence level.
pub const MIN_CONFIDENCE_LEVEL: f64 = 0.50;

/// Maximum accepted confidence level.
pub const MAX_CONFIDENCE_LEVEL: f64 = 0.999_999;

// ============================================================================
// Result aliases
// ============================================================================

/// Result type used throughout this module.
pub type PhysicalQecResult<T> = Result<T, PhysicalQecError>;

// ============================================================================
// Error model
// ============================================================================

/// Errors raised by physical QEC benchmarking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicalQecError {
    /// A required numeric value was invalid.
    InvalidValue {
        field: &'static str,
        reason: &'static str,
    },

    /// A configured collection exceeded the benchmark safety bound.
    LimitExceeded {
        field: &'static str,
        requested: u64,
        maximum: u64,
    },

    /// A confidence level was outside the supported range.
    InvalidConfidenceLevel {
        value_bits: u64,
    },

    /// An observation contains an impossible count relationship.
    InvalidObservation {
        reason: &'static str,
    },

    /// An operation was attempted after finalization.
    Finalized,

    /// A custom category limit was exceeded.
    TooManyCustomErrorKinds,

    /// Two results cannot be merged safely.
    IncompatibleResults {
        reason: &'static str,
    },

    /// A numerical operation would produce a non-finite result.
    NonFiniteResult {
        field: &'static str,
    },
}

impl fmt::Display for PhysicalQecError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidValue { field, reason } => {
                write!(f, "invalid physical-QEC value `{field}`: {reason}")
            }

            Self::LimitExceeded {
                field,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "physical-QEC benchmark limit exceeded for `{field}`: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::InvalidConfidenceLevel { value_bits } => {
                let value = f64::from_bits(*value_bits);
                write!(f, "invalid confidence level: {value}")
            }

            Self::InvalidObservation { reason } => {
                write!(f, "invalid physical-QEC observation: {reason}")
            }

            Self::Finalized => {
                f.write_str("physical-QEC benchmark has already been finalized")
            }

            Self::TooManyCustomErrorKinds => {
                f.write_str("too many custom physical-error categories")
            }

            Self::IncompatibleResults { reason } => {
                write!(f, "incompatible physical-QEC results: {reason}")
            }

            Self::NonFiniteResult { field } => {
                write!(f, "non-finite physical-QEC result in `{field}`")
            }
        }
    }
}

impl std::error::Error for PhysicalQecError {}

// ============================================================================
// Error category
// ============================================================================

/// Physical error category.
///
/// The category is part of the scientific identity of an observation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum PhysicalErrorKind {
    /// State-preparation error.
    Preparation,

    /// Single-qubit gate error.
    SingleQubitGate,

    /// Two-qubit gate error.
    TwoQubitGate,

    /// Multi-qubit gate error.
    MultiQubitGate,

    /// Measurement/readout error.
    Measurement,

    /// Reset error.
    Reset,

    /// Leakage out of the computational subspace.
    Leakage,

    /// Erasure/error where the state becomes unavailable.
    Erasure,

    /// Error associated with idle evolution.
    Idle,

    /// Error associated with transport/motion.
    Transport,

    /// Error introduced by a QEC syndrome-extraction operation.
    SyndromeExtraction,

    /// Error not otherwise classified.
    Other,

    /// User-defined category.
    Custom(String),
}

impl PhysicalErrorKind {
    /// Stable machine-readable identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Preparation => "preparation",
            Self::SingleQubitGate => "single_qubit_gate",
            Self::TwoQubitGate => "two_qubit_gate",
            Self::MultiQubitGate => "multi_qubit_gate",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::Leakage => "leakage",
            Self::Erasure => "erasure",
            Self::Idle => "idle",
            Self::Transport => "transport",
            Self::SyndromeExtraction => "syndrome_extraction",
            Self::Other => "other",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Returns true when the category represents leakage.
    pub const fn is_leakage(&self) -> bool {
        matches!(self, Self::Leakage)
    }

    /// Returns true when the category represents erasure.
    pub const fn is_erasure(&self) -> bool {
        matches!(self, Self::Erasure)
    }

    /// Returns true when the category is a gate error.
    pub const fn is_gate_error(&self) -> bool {
        matches!(
            self,
            Self::SingleQubitGate
                | Self::TwoQubitGate
                | Self::MultiQubitGate
        )
    }

    /// Creates a validated custom category.
    pub fn custom(
        value: impl Into<String>,
    ) -> PhysicalQecResult<Self> {
        let value = value.into();
        let value = value.trim().to_owned();

        if value.is_empty() {
            return Err(PhysicalQecError::InvalidValue {
                field: "error_kind",
                reason: "custom error kind must not be empty",
            });
        }

        if value.len() > 128 {
            return Err(PhysicalQecError::InvalidValue {
                field: "error_kind",
                reason: "custom error kind exceeds 128 bytes",
            });
        }

        Ok(Self::Custom(value))
    }
}

impl Default for PhysicalErrorKind {
    fn default() -> Self {
        Self::Other
    }
}

impl fmt::Display for PhysicalErrorKind {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Error axis
// ============================================================================

/// Pauli/error axis classification.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum PhysicalErrorAxis {
    /// No Pauli axis was assigned.
    None,

    /// X-like error.
    X,

    /// Y-like error.
    Y,

    /// Z-like error.
    Z,

    /// Error has multiple/unknown axes.
    Correlated,

    /// Axis was not characterized.
    Unknown,
}

impl Default for PhysicalErrorAxis {
    fn default() -> Self {
        Self::None
    }
}

// ============================================================================
// Observation basis
// ============================================================================

/// Measurement/characterization basis of an observation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum PhysicalObservationBasis {
    /// Direct physical-error observation.
    Direct,

    /// Pauli error classification.
    Pauli,

    /// Randomized benchmarking-derived estimate.
    RandomizedBenchmarking,

    /// Cycle-benchmarking-derived estimate.
    CycleBenchmarking,

    /// Leakage experiment.
    Leakage,

    /// Readout/assignment characterization.
    Readout,

    /// Calibration-derived observation.
    Calibration,

    /// QEC syndrome-extraction experiment.
    SyndromeExtraction,

    /// Simulation-derived physical noise observation.
    Simulation,

    /// User-defined characterization method.
    Custom,
}

impl Default for PhysicalObservationBasis {
    fn default() -> Self {
        Self::Direct
    }
}

// ============================================================================
// Observation
// ============================================================================

/// One aggregate physical-error observation.
///
/// This is deliberately an aggregate record rather than a raw measurement.
/// For example:
///
/// ```text
/// opportunities = 10_000
/// errors        = 37
/// ```
///
/// represents 37 observed errors over 10,000 opportunities.
///
/// No bitstrings, syndrome payloads, circuits, credentials, or raw device
/// data are stored here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalErrorObservation {
    /// Error category.
    pub kind: PhysicalErrorKind,

    /// Pauli/error axis.
    pub axis: PhysicalErrorAxis,

    /// Characterization basis.
    pub basis: PhysicalObservationBasis,

    /// Optional physical qubit identifier.
    pub qubit: Option<u32>,

    /// Optional QEC/sampling round.
    pub round: Option<u32>,

    /// Number of opportunities.
    pub opportunities: u64,

    /// Number of observed errors.
    pub errors: u64,

    /// Number of correlated/multi-location error events.
    ///
    /// This is informational and does not replace `errors`.
    pub correlated_errors: u64,

    /// Number of leakage events.
    ///
    /// This is informational and should normally be used with
    /// `PhysicalErrorKind::Leakage`.
    pub leakage_events: u64,

    /// Number of erasure events.
    pub erasure_events: u64,

    /// Optional gate/operation identifier.
    ///
    /// This is metadata only and is bounded by validation.
    pub operation: Option<String>,
}

impl PhysicalErrorObservation {
    /// Creates a simple physical-error observation.
    pub fn new(
        kind: PhysicalErrorKind,
        opportunities: u64,
        errors: u64,
    ) -> PhysicalQecResult<Self> {
        let observation = Self {
            kind,
            axis: PhysicalErrorAxis::None,
            basis: PhysicalObservationBasis::Direct,
            qubit: None,
            round: None,
            opportunities,
            errors,
            correlated_errors: 0,
            leakage_events: 0,
            erasure_events: 0,
            operation: None,
        };

        observation.validate()?;
        Ok(observation)
    }

    /// Validates the observation.
    pub fn validate(&self) -> PhysicalQecResult<()> {
        if self.errors > self.opportunities {
            return Err(PhysicalQecError::InvalidObservation {
                reason: "errors cannot exceed opportunities",
            });
        }

        if self.correlated_errors > self.errors {
            return Err(PhysicalQecError::InvalidObservation {
                reason: "correlated errors cannot exceed errors",
            });
        }

        if self.leakage_events > self.errors {
            return Err(PhysicalQecError::InvalidObservation {
                reason: "leakage events cannot exceed errors",
            });
        }

        if self.erasure_events > self.errors {
            return Err(PhysicalQecError::InvalidObservation {
                reason: "erasure events cannot exceed errors",
            });
        }

        if let Some(operation) = &self.operation {
            if operation.trim().is_empty() {
                return Err(PhysicalQecError::InvalidObservation {
                    reason: "operation identifier cannot be empty",
                });
            }

            if operation.len() > 256 {
                return Err(PhysicalQecError::InvalidObservation {
                    reason: "operation identifier exceeds 256 bytes",
                });
            }
        }

        if let Some(qubit) = self.qubit {
            if qubit as usize >= MAX_TRACKED_QUBITS {
                return Err(PhysicalQecError::LimitExceeded {
                    field: "qubit",
                    requested: qubit as u64,
                    maximum: (MAX_TRACKED_QUBITS - 1) as u64,
                });
            }
        }

        if let Some(round) = self.round {
            if round as usize >= MAX_TRACKED_ROUNDS {
                return Err(PhysicalQecError::LimitExceeded {
                    field: "round",
                    requested: round as u64,
                    maximum: (MAX_TRACKED_ROUNDS - 1) as u64,
                });
            }
        }

        Ok(())
    }

    /// Returns the empirical error rate.
    #[must_use]
    pub fn error_rate(&self) -> Option<f64> {
        ratio(self.errors, self.opportunities)
    }

    /// Returns true if the observation has no statistical opportunities.
    #[must_use]
    pub const fn has_no_opportunities(&self) -> bool {
        self.opportunities == 0
    }
}

// ============================================================================
// Confidence interval
// ============================================================================

/// Supported confidence-interval methods.
///
/// Wilson is implemented directly in this foundation file so this benchmark
/// does not depend on the later global statistics module.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum ConfidenceIntervalMethod {
    /// Wilson score interval.
    Wilson,
}

impl Default for ConfidenceIntervalMethod {
    fn default() -> Self {
        Self::Wilson
    }
}

/// Statistical confidence interval for a binomial proportion.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    /// Point estimate.
    pub estimate: f64,

    /// Lower confidence bound.
    pub lower: f64,

    /// Upper confidence bound.
    pub upper: f64,

    /// Confidence level, e.g. 0.95.
    pub confidence_level: f64,

    /// Number of successes/errors.
    pub successes: u64,

    /// Number of trials/opportunities.
    pub trials: u64,

    /// Method used.
    pub method: ConfidenceIntervalMethod,
}

impl ConfidenceInterval {
    /// Constructs a Wilson confidence interval.
    pub fn wilson(
        successes: u64,
        trials: u64,
        confidence_level: f64,
    ) -> PhysicalQecResult<Self> {
        validate_confidence_level(confidence_level)?;

        if successes > trials {
            return Err(PhysicalQecError::InvalidObservation {
                reason: "successes cannot exceed trials",
            });
        }

        if trials == 0 {
            return Err(PhysicalQecError::InvalidObservation {
                reason: "confidence interval requires at least one trial",
            });
        }

        let p = successes as f64 / trials as f64;
        let z = normal_quantile_for_two_sided_confidence(confidence_level);

        let n = trials as f64;
        let z2 = z * z;

        let denominator = 1.0 + z2 / n;
        let centre = (p + z2 / (2.0 * n)) / denominator;

        let margin_inner = (p * (1.0 - p) / n)
            + (z2 / (4.0 * n * n));

        let margin = z * margin_inner.sqrt() / denominator;

        let lower = (centre - margin).clamp(0.0, 1.0);
        let upper = (centre + margin).clamp(0.0, 1.0);

        if !p.is_finite() || !lower.is_finite() || !upper.is_finite() {
            return Err(PhysicalQecError::NonFiniteResult {
                field: "confidence_interval",
            });
        }

        Ok(Self {
            estimate: p,
            lower,
            upper,
            confidence_level,
            successes,
            trials,
            method: ConfidenceIntervalMethod::Wilson,
        })
    }

    /// Returns interval width.
    #[must_use]
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }
}

// ============================================================================
// Benchmark configuration
// ============================================================================

/// Configuration for a physical QEC benchmark.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalQecConfig {
    /// Benchmark schema version.
    pub schema_version: u32,

    /// Statistical confidence level.
    pub confidence_level: f64,

    /// Maximum accepted observations.
    pub max_observations: u64,

    /// Maximum number of custom error kinds.
    pub max_custom_error_kinds: usize,

    /// Whether per-qubit statistics are collected.
    pub track_per_qubit: bool,

    /// Whether per-round statistics are collected.
    pub track_per_round: bool,

    /// Whether per-axis statistics are collected.
    pub track_axes: bool,

    /// Whether operation-level statistics are collected.
    pub track_operations: bool,

    /// Whether correlated errors are tracked.
    pub track_correlated_errors: bool,

    /// Whether leakage is tracked separately.
    pub track_leakage: bool,

    /// Whether erasure is tracked separately.
    pub track_erasure: bool,

    /// Minimum opportunities required for a primary estimate to be marked
    /// statistically meaningful.
    pub minimum_opportunities: u64,

    /// Benchmark label.
    pub name: String,
}

impl Default for PhysicalQecConfig {
    fn default() -> Self {
        Self {
            schema_version: PHYSICAL_QEC_BENCHMARK_VERSION,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            max_observations: MAX_OBSERVATIONS,
            max_custom_error_kinds: MAX_CUSTOM_ERROR_KINDS,

            track_per_qubit: true,
            track_per_round: true,
            track_axes: true,
            track_operations: true,

            track_correlated_errors: true,
            track_leakage: true,
            track_erasure: true,

            minimum_opportunities: 1_000,

            name: PHYSICAL_QEC_BENCHMARK_ID.to_owned(),
        }
    }
}

impl PhysicalQecConfig {
    /// Validates the benchmark configuration.
    pub fn validate(&self) -> PhysicalQecResult<()> {
        if self.schema_version != PHYSICAL_QEC_BENCHMARK_VERSION {
            return Err(PhysicalQecError::InvalidValue {
                field: "schema_version",
                reason: "unsupported physical-QEC benchmark schema version",
            });
        }

        validate_confidence_level(self.confidence_level)?;

        if self.max_observations == 0 {
            return Err(PhysicalQecError::InvalidValue {
                field: "max_observations",
                reason: "maximum observations must be greater than zero",
            });
        }

        if self.max_observations > MAX_OBSERVATIONS {
            return Err(PhysicalQecError::LimitExceeded {
                field: "max_observations",
                requested: self.max_observations,
                maximum: MAX_OBSERVATIONS,
            });
        }

        if self.max_custom_error_kinds == 0 {
            return Err(PhysicalQecError::InvalidValue {
                field: "max_custom_error_kinds",
                reason: "custom error-kind capacity must be greater than zero",
            });
        }

        if self.max_custom_error_kinds > MAX_CUSTOM_ERROR_KINDS {
            return Err(PhysicalQecError::LimitExceeded {
                field: "max_custom_error_kinds",
                requested: self.max_custom_error_kinds as u64,
                maximum: MAX_CUSTOM_ERROR_KINDS as u64,
            });
        }

        if self.name.trim().is_empty() {
            return Err(PhysicalQecError::InvalidValue {
                field: "name",
                reason: "benchmark name must not be empty",
            });
        }

        if self.name.len() > 256 {
            return Err(PhysicalQecError::InvalidValue {
                field: "name",
                reason: "benchmark name exceeds 256 bytes",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Aggregate counter
// ============================================================================

/// Bounded aggregate counter for one error category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhysicalErrorCounter {
    /// Opportunities.
    pub opportunities: u64,

    /// Errors.
    pub errors: u64,

    /// Correlated errors.
    pub correlated_errors: u64,

    /// Leakage events.
    pub leakage_events: u64,

    /// Erasure events.
    pub erasure_events: u64,
}

impl Default for PhysicalErrorCounter {
    fn default() -> Self {
        Self {
            opportunities: 0,
            errors: 0,
            correlated_errors: 0,
            leakage_events: 0,
            erasure_events: 0,
        }
    }
}

impl PhysicalErrorCounter {
    /// Adds one validated observation.
    pub fn add(
        &mut self,
        observation: &PhysicalErrorObservation,
    ) -> PhysicalQecResult<()> {
        let opportunities = self
            .opportunities
            .checked_add(observation.opportunities)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "opportunities",
                reason: "counter overflow",
            })?;

        let errors = self
            .errors
            .checked_add(observation.errors)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "errors",
                reason: "counter overflow",
            })?;

        let correlated_errors = self
            .correlated_errors
            .checked_add(observation.correlated_errors)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "correlated_errors",
                reason: "counter overflow",
            })?;

        let leakage_events = self
            .leakage_events
            .checked_add(observation.leakage_events)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "leakage_events",
                reason: "counter overflow",
            })?;

        let erasure_events = self
            .erasure_events
            .checked_add(observation.erasure_events)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "erasure_events",
                reason: "counter overflow",
            })?;

        if errors > opportunities {
            return Err(PhysicalQecError::InvalidObservation {
                reason: "aggregated errors exceed aggregated opportunities",
            });
        }

        self.opportunities = opportunities;
        self.errors = errors;
        self.correlated_errors = correlated_errors;
        self.leakage_events = leakage_events;
        self.erasure_events = erasure_events;

        Ok(())
    }

    /// Merges another aggregate.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> PhysicalQecResult<()> {
        let opportunities = self
            .opportunities
            .checked_add(other.opportunities)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "opportunities",
                reason: "counter overflow",
            })?;

        let errors = self
            .errors
            .checked_add(other.errors)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "errors",
                reason: "counter overflow",
            })?;

        if errors > opportunities {
            return Err(PhysicalQecError::InvalidObservation {
                reason: "merged errors exceed merged opportunities",
            });
        }

        self.opportunities = opportunities;
        self.errors = errors;

        self.correlated_errors = self
            .correlated_errors
            .checked_add(other.correlated_errors)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "correlated_errors",
                reason: "counter overflow",
            })?;

        self.leakage_events = self
            .leakage_events
            .checked_add(other.leakage_events)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "leakage_events",
                reason: "counter overflow",
            })?;

        self.erasure_events = self
            .erasure_events
            .checked_add(other.erasure_events)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "erasure_events",
                reason: "counter overflow",
            })?;

        Ok(())
    }

    /// Returns the empirical rate if opportunities exist.
    #[must_use]
    pub fn rate(&self) -> Option<f64> {
        ratio(self.errors, self.opportunities)
    }

    /// Calculates a confidence interval.
    pub fn confidence_interval(
        &self,
        confidence_level: f64,
    ) -> PhysicalQecResult<Option<ConfidenceInterval>> {
        if self.opportunities == 0 {
            return Ok(None);
        }

        Ok(Some(ConfidenceInterval::wilson(
            self.errors,
            self.opportunities,
            confidence_level,
        )?))
    }
}

// ============================================================================
// Per-qubit result
// ============================================================================

/// Per-qubit physical-error aggregate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QubitPhysicalErrorResult {
    /// Physical qubit identifier.
    pub qubit: u32,

    /// Aggregate error counts.
    pub counter: PhysicalErrorCounter,
}

impl QubitPhysicalErrorResult {
    /// Returns the physical error rate.
    #[must_use]
    pub fn error_rate(&self) -> Option<f64> {
        self.counter.rate()
    }
}

// ============================================================================
// Per-round result
// ============================================================================

/// Per-round physical-error aggregate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoundPhysicalErrorResult {
    /// Round identifier.
    pub round: u32,

    /// Aggregate error counts.
    pub counter: PhysicalErrorCounter,
}

impl RoundPhysicalErrorResult {
    /// Returns the physical error rate.
    #[must_use]
    pub fn error_rate(&self) -> Option<f64> {
        self.counter.rate()
    }
}

// ============================================================================
// Operation result
// ============================================================================

/// Per-operation physical-error aggregate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationPhysicalErrorResult {
    /// Stable operation identifier.
    pub operation: String,

    /// Aggregate error counts.
    pub counter: PhysicalErrorCounter,
}

impl OperationPhysicalErrorResult {
    /// Returns the physical error rate.
    #[must_use]
    pub fn error_rate(&self) -> Option<f64> {
        self.counter.rate()
    }
}

// ============================================================================
// Primary result
// ============================================================================

/// Physical QEC benchmark result.
///
/// This is the stable output boundary consumed by the later benchmarking
/// result/reporting/analysis layers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalQecResultData {
    /// Stable benchmark identifier.
    pub benchmark_id: String,

    /// Benchmark protocol version.
    pub benchmark_version: u32,

    /// Benchmark name.
    pub name: String,

    /// Number of accepted observations.
    pub observations: u64,

    /// Primary aggregate counter.
    pub total: PhysicalErrorCounter,

    /// Primary physical error-rate estimate.
    pub physical_error_rate: Option<f64>,

    /// Primary confidence interval.
    pub confidence_interval: Option<ConfidenceInterval>,

    /// Error counts grouped by category.
    pub by_kind: BTreeMap<PhysicalErrorKind, PhysicalErrorCounter>,

    /// Error counts grouped by Pauli/error axis.
    pub by_axis: BTreeMap<PhysicalErrorAxis, PhysicalErrorCounter>,

    /// Error counts grouped by physical qubit.
    pub by_qubit: BTreeMap<u32, QubitPhysicalErrorResult>,

    /// Error counts grouped by QEC round.
    pub by_round: BTreeMap<u32, RoundPhysicalErrorResult>,

    /// Error counts grouped by operation.
    pub by_operation: BTreeMap<String, OperationPhysicalErrorResult>,

    /// Correlated error rate.
    pub correlated_error_rate: Option<f64>,

    /// Leakage rate.
    pub leakage_rate: Option<f64>,

    /// Erasure rate.
    pub erasure_rate: Option<f64>,

    /// Confidence interval for leakage.
    pub leakage_confidence_interval: Option<ConfidenceInterval>,

    /// Confidence interval for erasure.
    pub erasure_confidence_interval: Option<ConfidenceInterval>,

    /// Confidence level used for the result.
    pub confidence_level: f64,

    /// Minimum statistical sample requirement.
    pub minimum_opportunities: u64,

    /// Whether the primary result has enough opportunities for the configured
    /// statistical requirement.
    pub statistically_sufficient: bool,

    /// Whether at least one logical/physical observation was rejected.
    pub had_invalid_observation: bool,

    /// Number of observations that were rejected.
    pub rejected_observations: u64,

    /// Human/machine-readable warnings.
    pub warnings: Vec<String>,
}

impl PhysicalQecResultData {
    /// Returns whether the benchmark produced a usable primary estimate.
    #[must_use]
    pub fn is_valid_estimate(&self) -> bool {
        self.physical_error_rate.is_some()
            && self.statistically_sufficient
            && !self.had_invalid_observation
    }

    /// Returns the primary error rate or zero.
    #[must_use]
    pub fn error_rate_or_zero(&self) -> f64 {
        self.physical_error_rate.unwrap_or(0.0)
    }

    /// Returns the number of physical error events.
    #[must_use]
    pub const fn error_count(&self) -> u64 {
        self.total.errors
    }

    /// Returns the number of error opportunities.
    #[must_use]
    pub const fn opportunity_count(&self) -> u64 {
        self.total.opportunities
    }

    /// Returns whether the result contains leakage.
    #[must_use]
    pub const fn contains_leakage(&self) -> bool {
        self.total.leakage_events > 0
    }

    /// Returns whether the result contains erasures.
    #[must_use]
    pub const fn contains_erasures(&self) -> bool {
        self.total.erasure_events > 0
    }

    /// Returns the worst observed qubit by point-estimate error rate.
    ///
    /// Ties are resolved by the lowest qubit identifier to preserve
    /// deterministic behavior.
    #[must_use]
    pub fn worst_qubit(&self) -> Option<&QubitPhysicalErrorResult> {
        self.by_qubit.values().max_by(|a, b| {
            compare_optional_rate(
                a.error_rate(),
                b.error_rate(),
            )
            .then_with(|| b.qubit.cmp(&a.qubit))
        })
    }
}

// ============================================================================
// Accumulator
// ============================================================================

/// Stateful physical-error benchmark accumulator.
///
/// The accumulator is intentionally not `Send + Sync` by itself. Callers that
/// need parallel collection should use independent accumulators and merge
/// them deterministically.
#[derive(Debug, Clone)]
pub struct PhysicalQecBenchmark {
    config: PhysicalQecConfig,

    observations: u64,

    total: PhysicalErrorCounter,

    by_kind: BTreeMap<PhysicalErrorKind, PhysicalErrorCounter>,

    by_axis: BTreeMap<PhysicalErrorAxis, PhysicalErrorCounter>,

    by_qubit: BTreeMap<u32, PhysicalErrorCounter>,

    by_round: BTreeMap<u32, PhysicalErrorCounter>,

    by_operation: BTreeMap<String, PhysicalErrorCounter>,

    custom_error_kinds: usize,

    rejected_observations: u64,

    had_invalid_observation: bool,

    finalized: bool,
}

impl PhysicalQecBenchmark {
    /// Creates a benchmark using production defaults.
    pub fn new() -> PhysicalQecResult<Self> {
        Self::with_config(PhysicalQecConfig::default())
    }

    /// Creates a benchmark with explicit configuration.
    pub fn with_config(
        config: PhysicalQecConfig,
    ) -> PhysicalQecResult<Self> {
        config.validate()?;

        Ok(Self {
            config,

            observations: 0,

            total: PhysicalErrorCounter::default(),

            by_kind: BTreeMap::new(),
            by_axis: BTreeMap::new(),
            by_qubit: BTreeMap::new(),
            by_round: BTreeMap::new(),
            by_operation: BTreeMap::new(),

            custom_error_kinds: 0,

            rejected_observations: 0,
            had_invalid_observation: false,

            finalized: false,
        })
    }

    /// Returns the benchmark configuration.
    #[must_use]
    pub const fn config(&self) -> &PhysicalQecConfig {
        &self.config
    }

    /// Returns the number of accepted observations.
    #[must_use]
    pub const fn observation_count(&self) -> u64 {
        self.observations
    }

    /// Records one observation.
    ///
    /// Validation occurs before mutation. Therefore an invalid observation
    /// cannot partially modify benchmark state.
    pub fn observe(
        &mut self,
        observation: PhysicalErrorObservation,
    ) -> PhysicalQecResult<()> {
        if self.finalized {
            return Err(PhysicalQecError::Finalized);
        }

        if self.observations >= self.config.max_observations {
            return Err(PhysicalQecError::LimitExceeded {
                field: "observations",
                requested: self.observations.saturating_add(1),
                maximum: self.config.max_observations,
            });
        }

        observation.validate()?;

        self.validate_custom_kind(&observation.kind)?;

        // Prepare cloned counters first so mutation is effectively
        // transactional.
        let mut total = self.total;
        total.add(&observation)?;

        let mut kind_counter = self
            .by_kind
            .get(&observation.kind)
            .copied()
            .unwrap_or_default();

        kind_counter.add(&observation)?;

        let mut axis_counter = self
            .by_axis
            .get(&observation.axis)
            .copied()
            .unwrap_or_default();

        axis_counter.add(&observation)?;

        let qubit_counter = if let Some(qubit) = observation.qubit {
            let mut counter = self
                .by_qubit
                .get(&qubit)
                .copied()
                .unwrap_or_default();

            counter.add(&observation)?;
            Some((qubit, counter))
        } else {
            None
        };

        let round_counter = if let Some(round) = observation.round {
            let mut counter = self
                .by_round
                .get(&round)
                .copied()
                .unwrap_or_default();

            counter.add(&observation)?;
            Some((round, counter))
        } else {
            None
        };

        let operation_counter = if self.config.track_operations {
            observation.operation.as_ref().map(|operation| {
                let mut counter = self
                    .by_operation
                    .get(operation)
                    .copied()
                    .unwrap_or_default();

                // Safe because the observation was already validated.
                let _ = counter.add(&observation);

                (operation.clone(), counter)
            })
        } else {
            None
        };

        // Commit only after all validation/calculation succeeded.
        self.total = total;

        self.by_kind
            .insert(observation.kind.clone(), kind_counter);

        if self.config.track_axes {
            self.by_axis
                .insert(observation.axis, axis_counter);
        }

        if let Some((qubit, counter)) = qubit_counter {
            if self.by_qubit.len() >= MAX_TRACKED_QUBITS
                && !self.by_qubit.contains_key(&qubit)
            {
                return Err(PhysicalQecError::LimitExceeded {
                    field: "tracked_qubits",
                    requested: self.by_qubit.len() as u64 + 1,
                    maximum: MAX_TRACKED_QUBITS as u64,
                });
            }

            self.by_qubit.insert(qubit, counter);
        }

        if let Some((round, counter)) = round_counter {
            if self.by_round.len() >= MAX_TRACKED_ROUNDS
                && !self.by_round.contains_key(&round)
            {
                return Err(PhysicalQecError::LimitExceeded {
                    field: "tracked_rounds",
                    requested: self.by_round.len() as u64 + 1,
                    maximum: MAX_TRACKED_ROUNDS as u64,
                });
            }

            self.by_round.insert(round, counter);
        }

        if let Some((operation, counter)) = operation_counter {
            self.by_operation.insert(operation, counter);
        }

        self.observations += 1;

        Ok(())
    }

    /// Records an observation and converts invalid observations into a
    /// rejected-observation count.
    ///
    /// This is useful for streaming benchmark ingestion where one malformed
    /// backend record should be reported without silently corrupting the
    /// aggregate.
    pub fn observe_loss_tolerant(
        &mut self,
        observation: PhysicalErrorObservation,
    ) -> PhysicalQecResult<bool> {
        match self.observe(observation) {
            Ok(()) => Ok(true),

            Err(
                error @ PhysicalQecError::InvalidObservation { .. },
            ) => {
                self.rejected_observations =
                    self.rejected_observations.saturating_add(1);

                self.had_invalid_observation = true;

                // The error is intentionally returned so callers cannot
                // accidentally ignore the rejection.
                Err(error)
            }

            Err(error) => Err(error),
        }
    }

    /// Merges another accumulator.
    ///
    /// Both accumulators must have equivalent configuration semantics.
    ///
    /// The merge operation is deterministic because all aggregates use
    /// ordered maps and integer addition.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> PhysicalQecResult<()> {
        if self.finalized || other.finalized {
            return Err(PhysicalQecError::Finalized);
        }

        if self.config != other.config {
            return Err(PhysicalQecError::IncompatibleResults {
                reason: "benchmark configurations differ",
            });
        }

        let observations = self
            .observations
            .checked_add(other.observations)
            .ok_or(PhysicalQecError::InvalidValue {
                field: "observations",
                reason: "observation count overflow",
            })?;

        if observations > self.config.max_observations {
            return Err(PhysicalQecError::LimitExceeded {
                field: "observations",
                requested: observations,
                maximum: self.config.max_observations,
            });
        }

        let mut total = self.total;
        total.merge(&other.total)?;

        let mut by_kind = self.by_kind.clone();

        for (kind, counter) in &other.by_kind {
            let entry = by_kind.entry(kind.clone()).or_default();
            entry.merge(counter)?;
        }

        let mut by_axis = self.by_axis.clone();

        for (axis, counter) in &other.by_axis {
            let entry = by_axis.entry(*axis).or_default();
            entry.merge(counter)?;
        }

        let mut by_qubit = self.by_qubit.clone();

        for (qubit, counter) in &other.by_qubit {
            if !by_qubit.contains_key(qubit)
                && by_qubit.len() >= MAX_TRACKED_QUBITS
            {
                return Err(PhysicalQecError::LimitExceeded {
                    field: "tracked_qubits",
                    requested: by_qubit.len() as u64 + 1,
                    maximum: MAX_TRACKED_QUBITS as u64,
                });
            }

            let entry = by_qubit.entry(*qubit).or_default();
            entry.merge(counter)?;
        }

        let mut by_round = self.by_round.clone();

        for (round, counter) in &other.by_round {
            if !by_round.contains_key(round)
                && by_round.len() >= MAX_TRACKED_ROUNDS
            {
                return Err(PhysicalQecError::LimitExceeded {
                    field: "tracked_rounds",
                    requested: by_round.len() as u64 + 1,
                    maximum: MAX_TRACKED_ROUNDS as u64,
                });
            }

            let entry = by_round.entry(*round).or_default();
            entry.merge(counter)?;
        }

        let mut by_operation = self.by_operation.clone();

        for (operation, counter) in &other.by_operation {
            let entry =
                by_operation.entry(operation.clone()).or_default();

            entry.merge(counter)?;
        }

        self.total = total;
        self.by_kind = by_kind;
        self.by_axis = by_axis;
        self.by_qubit = by_qubit;
        self.by_round = by_round;
        self.by_operation = by_operation;

        self.observations = observations;

        self.rejected_observations = self
            .rejected_observations
            .saturating_add(other.rejected_observations);

        self.had_invalid_observation =
            self.had_invalid_observation
                || other.had_invalid_observation;

        self.custom_error_kinds = self
            .by_kind
            .keys()
            .filter(|kind| matches!(kind, PhysicalErrorKind::Custom(_)))
            .count();

        Ok(())
    }

    /// Finalizes the benchmark and returns an immutable result.
    ///
    /// Calling `finalize` freezes the accumulator. Further observations are
    /// rejected.
    pub fn finalize(
        &mut self,
    ) -> PhysicalQecResult<PhysicalQecResultData> {
        if self.finalized {
            return Err(PhysicalQecError::Finalized);
        }

        self.finalized = true;

        self.build_result()
    }

    /// Builds a result without changing finalization state.
    ///
    /// Useful for live dashboards and progress reporting.
    pub fn snapshot(
        &self,
    ) -> PhysicalQecResult<PhysicalQecResultData> {
        self.build_result()
    }

    fn build_result(
        &self,
    ) -> PhysicalQecResult<PhysicalQecResultData> {
        let physical_error_rate = self.total.rate();

        let confidence_interval = self
            .total
            .confidence_interval(self.config.confidence_level)?;

        let leakage_counter = self
            .by_kind
            .get(&PhysicalErrorKind::Leakage)
            .copied()
            .unwrap_or_default();

        let erasure_counter = self
            .by_kind
            .get(&PhysicalErrorKind::Erasure)
            .copied()
            .unwrap_or_default();

        let correlated_error_rate =
            ratio(
                self.total.correlated_errors,
                self.total.opportunities,
            );

        let leakage_rate =
            ratio(
                leakage_counter.errors,
                leakage_counter.opportunities,
            );

        let erasure_rate =
            ratio(
                erasure_counter.errors,
                erasure_counter.opportunities,
            );

        let leakage_confidence_interval =
            leakage_counter.confidence_interval(
                self.config.confidence_level,
            )?;

        let erasure_confidence_interval =
            erasure_counter.confidence_interval(
                self.config.confidence_level,
            )?;

        let statistically_sufficient =
            self.total.opportunities
                >= self.config.minimum_opportunities;

        let mut warnings = Vec::new();

        if self.total.opportunities == 0 {
            warnings.push(
                "no physical-error opportunities were observed".to_owned(),
            );
        } else if !statistically_sufficient {
            warnings.push(format!(
                "only {} physical-error opportunities were observed; \
                 configured minimum is {}",
                self.total.opportunities,
                self.config.minimum_opportunities,
            ));
        }

        if self.had_invalid_observation {
            warnings.push(format!(
                "{} observation(s) were rejected",
                self.rejected_observations,
            ));
        }

        if self.total.leakage_events > 0 {
            warnings.push(
                "leakage was observed; computational-subspace error rate \
                 should not be interpreted as a complete physical noise \
                 characterization"
                    .to_owned(),
            );
        }

        if self.total.erasure_events > 0 {
            warnings.push(
                "erasure events were observed; erasure and ordinary Pauli \
                 error rates should be reported separately"
                    .to_owned(),
            );
        }

        if self.total.correlated_errors > 0 {
            warnings.push(
                "correlated errors were observed; an independent-error \
                 assumption may be inappropriate"
                    .to_owned(),
            );
        }

        Ok(PhysicalQecResultData {
            benchmark_id: PHYSICAL_QEC_BENCHMARK_ID.to_owned(),
            benchmark_version: PHYSICAL_QEC_BENCHMARK_VERSION,
            name: self.config.name.clone(),

            observations: self.observations,

            total: self.total,

            physical_error_rate,
            confidence_interval,

            by_kind: self.by_kind.clone(),
            by_axis: self.by_axis.clone(),

            by_qubit: self
                .by_qubit
                .iter()
                .map(|(qubit, counter)| {
                    (
                        *qubit,
                        QubitPhysicalErrorResult {
                            qubit: *qubit,
                            counter: *counter,
                        },
                    )
                })
                .collect(),

            by_round: self
                .by_round
                .iter()
                .map(|(round, counter)| {
                    (
                        *round,
                        RoundPhysicalErrorResult {
                            round: *round,
                            counter: *counter,
                        },
                    )
                })
                .collect(),

            by_operation: self
                .by_operation
                .iter()
                .map(|(operation, counter)| {
                    (
                        operation.clone(),
                        OperationPhysicalErrorResult {
                            operation: operation.clone(),
                            counter: *counter,
                        },
                    )
                })
                .collect(),

            correlated_error_rate,

            leakage_rate,
            erasure_rate,

            leakage_confidence_interval,
            erasure_confidence_interval,

            confidence_level: self.config.confidence_level,

            minimum_opportunities:
                self.config.minimum_opportunities,

            statistically_sufficient,

            had_invalid_observation:
                self.had_invalid_observation,

            rejected_observations:
                self.rejected_observations,

            warnings,
        })
    }

    fn validate_custom_kind(
        &mut self,
        kind: &PhysicalErrorKind,
    ) -> PhysicalQecResult<()> {
        if !matches!(kind, PhysicalErrorKind::Custom(_)) {
            return Ok(());
        }

        if self.by_kind.contains_key(kind) {
            return Ok(());
        }

        if self.custom_error_kinds
            >= self.config.max_custom_error_kinds
        {
            return Err(
                PhysicalQecError::TooManyCustomErrorKinds,
            );
        }

        self.custom_error_kinds += 1;

        Ok(())
    }
}

// ============================================================================
// Convenience constructors
// ============================================================================

impl PhysicalQecBenchmark {
    /// Records a standard physical error observation.
    pub fn record_error(
        &mut self,
        kind: PhysicalErrorKind,
        opportunities: u64,
        errors: u64,
    ) -> PhysicalQecResult<()> {
        self.observe(PhysicalErrorObservation::new(
            kind,
            opportunities,
            errors,
        )?)
    }

    /// Records a per-qubit physical error observation.
    pub fn record_qubit_error(
        &mut self,
        qubit: u32,
        kind: PhysicalErrorKind,
        opportunities: u64,
        errors: u64,
    ) -> PhysicalQecResult<()> {
        let mut observation =
            PhysicalErrorObservation::new(
                kind,
                opportunities,
                errors,
            )?;

        observation.qubit = Some(qubit);

        self.observe(observation)
    }

    /// Records a per-round physical error observation.
    pub fn record_round_error(
        &mut self,
        round: u32,
        kind: PhysicalErrorKind,
        opportunities: u64,
        errors: u64,
    ) -> PhysicalQecResult<()> {
        let mut observation =
            PhysicalErrorObservation::new(
                kind,
                opportunities,
                errors,
            )?;

        observation.round = Some(round);

        self.observe(observation)
    }

    /// Records a leakage observation.
    pub fn record_leakage(
        &mut self,
        opportunities: u64,
        leakage_events: u64,
    ) -> PhysicalQecResult<()> {
        let mut observation =
            PhysicalErrorObservation::new(
                PhysicalErrorKind::Leakage,
                opportunities,
                leakage_events,
            )?;

        observation.leakage_events = leakage_events;

        self.observe(observation)
    }

    /// Records an erasure observation.
    pub fn record_erasure(
        &mut self,
        opportunities: u64,
        erasure_events: u64,
    ) -> PhysicalQecResult<()> {
        let mut observation =
            PhysicalErrorObservation::new(
                PhysicalErrorKind::Erasure,
                opportunities,
                erasure_events,
            )?;

        observation.erasure_events = erasure_events;

        self.observe(observation)
    }

    /// Records a measurement/readout observation.
    pub fn record_measurement_error(
        &mut self,
        opportunities: u64,
        errors: u64,
    ) -> PhysicalQecResult<()> {
        self.record_error(
            PhysicalErrorKind::Measurement,
            opportunities,
            errors,
        )
    }

    /// Records a single-qubit gate observation.
    pub fn record_single_qubit_gate_error(
        &mut self,
        opportunities: u64,
        errors: u64,
    ) -> PhysicalQecResult<()> {
        self.record_error(
            PhysicalErrorKind::SingleQubitGate,
            opportunities,
            errors,
        )
    }

    /// Records a two-qubit gate observation.
    pub fn record_two_qubit_gate_error(
        &mut self,
        opportunities: u64,
        errors: u64,
    ) -> PhysicalQecResult<()> {
        self.record_error(
            PhysicalErrorKind::TwoQubitGate,
            opportunities,
            errors,
        )
    }
}

// ============================================================================
// Existing QEC metrics integration
// ============================================================================

/// Converts the existing QEC aggregate physical-error metrics into a
/// benchmarking observation.
///
/// This is intentionally a one-way adapter:
///
/// ```text
/// error_correction::metrics
///          │
///          ▼
/// benchmarking::qec::physical
/// ```
///
/// Benchmarking never becomes the owner of QEC runtime metrics.
///
/// The adapter is kept behind this function so the rest of this module
/// remains independent from the execution subsystem.
pub fn observation_from_qec_metrics(
    opportunities: u64,
    errors: u64,
) -> PhysicalQecResult<PhysicalErrorObservation> {
    PhysicalErrorObservation::new(
        PhysicalErrorKind::Other,
        opportunities,
        errors,
    )
}

// ============================================================================
// Statistical helpers
// ============================================================================

fn ratio(
    numerator: u64,
    denominator: u64,
) -> Option<f64> {
    if denominator == 0 {
        return None;
    }

    let value =
        numerator as f64 / denominator as f64;

    if value.is_finite() {
        Some(value.clamp(0.0, 1.0))
    } else {
        None
    }
}

fn validate_confidence_level(
    confidence_level: f64,
) -> PhysicalQecResult<()> {
    if !confidence_level.is_finite()
        || !(MIN_CONFIDENCE_LEVEL..=MAX_CONFIDENCE_LEVEL)
            .contains(&confidence_level)
    {
        return Err(
            PhysicalQecError::InvalidConfidenceLevel {
                value_bits: confidence_level.to_bits(),
            },
        );
    }

    Ok(())
}

/// Approximate inverse normal CDF.
///
/// This is the Acklam rational approximation, implemented directly to keep
/// the physical benchmark independent from a mandatory external statistics
/// crate.
///
/// Accuracy is sufficient for confidence intervals used by benchmarking;
/// protocol-level statistical modules may later provide higher-order
/// distribution machinery without changing this file's public result model.
fn inverse_normal_cdf(p: f64) -> f64 {
    const A: [f64; 6] = [
        -3.969683028665376e1,
        2.209460984245205e2,
        -2.759285104469687e2,
        1.383577518672690e2,
        -3.066479806614716e1,
        2.506628277459239,
    ];

    const B: [f64; 5] = [
        -5.447609879822406e1,
        1.615858368580409e2,
        -1.556989798598866e2,
        6.680131188771972e1,
        -1.328068155288572e1,
    ];

    const C: [f64; 6] = [
        -7.784894002430293e-3,
        -3.223964580411365e-1,
        -2.400758277161838,
        -2.549732539343734,
        4.374664141464968,
        2.938163982698783,
    ];

    const D: [f64; 4] = [
        7.784695709041462e-3,
        3.224671290700398e-1,
        2.445134137142996,
        3.754408661907416,
    ];

    const P_LOW: f64 = 0.02425;
    const P_HIGH: f64 = 1.0 - P_LOW;

    if p < P_LOW {
        let q = (-2.0 * p.ln()).sqrt();

        return (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q
            + C[4])
            * q)
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q
                + 1.0);
    }

    if p <= P_HIGH {
        let q = p - 0.5;
        let r = q * q;

        return (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r
            + A[4])
            * r)
            + A[5])
            * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r
                + B[4])
                * r)
                + 1.0);
    }

    let q = (-2.0 * (1.0 - p).ln()).sqrt();

    -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q
        + C[4])
        * q)
        + C[5])
        / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q
            + 1.0)
}

fn normal_quantile_for_two_sided_confidence(
    confidence_level: f64,
) -> f64 {
    let tail = (1.0 - confidence_level) / 2.0;
    inverse_normal_cdf(1.0 - tail)
}

fn compare_optional_rate(
    left: Option<f64>,
    right: Option<f64>,
) -> std::cmp::Ordering {
    match (left, right) {
        (Some(a), Some(b)) => {
            a.partial_cmp(&b)
                .unwrap_or(std::cmp::Ordering::Equal)
        }

        (Some(_), None) => std::cmp::Ordering::Greater,

        (None, Some(_)) => std::cmp::Ordering::Less,

        (None, None) => std::cmp::Ordering::Equal,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        let config = PhysicalQecConfig::default();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn simple_observation_calculates_error_rate() {
        let observation =
            PhysicalErrorObservation::new(
                PhysicalErrorKind::TwoQubitGate,
                1_000,
                10,
            )
            .expect("valid observation");

        assert_eq!(
            observation.error_rate(),
            Some(0.01)
        );
    }

    #[test]
    fn errors_cannot_exceed_opportunities() {
        let result =
            PhysicalErrorObservation::new(
                PhysicalErrorKind::Measurement,
                10,
                11,
            );

        assert!(matches!(
            result,
            Err(PhysicalQecError::InvalidObservation { .. })
        ));
    }

    #[test]
    fn wilson_interval_is_bounded() {
        let interval =
            ConfidenceInterval::wilson(
                10,
                1_000,
                0.95,
            )
            .expect("valid interval");

        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
        assert!(interval.lower <= interval.estimate);
        assert!(interval.estimate <= interval.upper);
    }

    #[test]
    fn zero_errors_still_have_uncertainty() {
        let interval =
            ConfidenceInterval::wilson(
                0,
                1_000,
                0.95,
            )
            .expect("valid interval");

        assert_eq!(interval.estimate, 0.0);
        assert!(interval.upper > 0.0);
    }

    #[test]
    fn benchmark_accumulates_observations() {
        let mut benchmark =
            PhysicalQecBenchmark::new()
                .expect("valid benchmark");

        benchmark
            .record_two_qubit_gate_error(1_000, 10)
            .expect("valid observation");

        benchmark
            .record_two_qubit_gate_error(2_000, 20)
            .expect("valid observation");

        let result =
            benchmark.snapshot().expect("valid snapshot");

        assert_eq!(result.observations, 2);
        assert_eq!(
            result.total.opportunities,
            3_000
        );
        assert_eq!(result.total.errors, 30);
    }

    #[test]
    fn physical_error_rate_is_derived_from_aggregate_counts() {
        let mut benchmark =
            PhysicalQecBenchmark::with_config(
                PhysicalQecConfig {
                    minimum_opportunities: 1,
                    ..PhysicalQecConfig::default()
                },
            )
            .expect("valid benchmark");

        benchmark
            .record_single_qubit_gate_error(
                10_000,
                100,
            )
            .expect("valid observation");

        let result =
            benchmark.finalize()
                .expect("valid result");

        assert_eq!(
            result.physical_error_rate,
            Some(0.01)
        );

        assert!(result.statistically_sufficient);
    }

    #[test]
    fn per_qubit_results_are_deterministic() {
        let mut benchmark =
            PhysicalQecBenchmark::new()
                .expect("valid benchmark");

        benchmark
            .record_qubit_error(
                2,
                PhysicalErrorKind::TwoQubitGate,
                1_000,
                20,
            )
            .expect("valid observation");

        benchmark
            .record_qubit_error(
                1,
                PhysicalErrorKind::TwoQubitGate,
                1_000,
                30,
            )
            .expect("valid observation");

        let result =
            benchmark.snapshot().expect("valid snapshot");

        assert_eq!(
            result.by_qubit.keys().copied().collect::<Vec<_>>(),
            vec![1, 2]
        );

        assert_eq!(
            result.worst_qubit().map(|q| q.qubit),
            Some(1)
        );
    }

    #[test]
    fn leakage_is_reported_separately() {
        let mut benchmark =
            PhysicalQecBenchmark::with_config(
                PhysicalQecConfig {
                    minimum_opportunities: 1,
                    ..PhysicalQecConfig::default()
                },
            )
            .expect("valid benchmark");

        benchmark
            .record_leakage(1_000, 5)
            .expect("valid leakage observation");

        let result =
            benchmark.snapshot().expect("valid snapshot");

        assert_eq!(
            result.leakage_rate,
            Some(0.005)
        );

        assert!(result.contains_leakage());
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn erasure_is_reported_separately() {
        let mut benchmark =
            PhysicalQecBenchmark::with_config(
                PhysicalQecConfig {
                    minimum_opportunities: 1,
                    ..PhysicalQecConfig::default()
                },
            )
            .expect("valid benchmark");

        benchmark
            .record_erasure(1_000, 3)
            .expect("valid erasure observation");

        let result =
            benchmark.snapshot().expect("valid snapshot");

        assert_eq!(
            result.erasure_rate,
            Some(0.003)
        );

        assert!(result.contains_erasures());
    }

    #[test]
    fn finalize_prevents_further_mutation() {
        let mut benchmark =
            PhysicalQecBenchmark::new()
                .expect("valid benchmark");

        benchmark
            .record_error(
                PhysicalErrorKind::Measurement,
                100,
                1,
            )
            .expect("valid observation");

        let _ =
            benchmark.finalize()
                .expect("valid result");

        let result =
            benchmark.record_error(
                PhysicalErrorKind::Measurement,
                100,
                1,
            );

        assert!(matches!(
            result,
            Err(PhysicalQecError::Finalized)
        ));
    }

    #[test]
    fn equivalent_accumulators_merge() {
        let config = PhysicalQecConfig {
            minimum_opportunities: 1,
            ..PhysicalQecConfig::default()
        };

        let mut first =
            PhysicalQecBenchmark::with_config(
                config.clone(),
            )
            .expect("valid benchmark");

        let mut second =
            PhysicalQecBenchmark::with_config(config)
                .expect("valid benchmark");

        first
            .record_error(
                PhysicalErrorKind::SingleQubitGate,
                1_000,
                10,
            )
            .expect("valid observation");

        second
            .record_error(
                PhysicalErrorKind::SingleQubitGate,
                2_000,
                20,
            )
            .expect("valid observation");

        first
            .merge(&second)
            .expect("merge should succeed");

        let result =
            first.snapshot().expect("valid snapshot");

        assert_eq!(
            result.total.opportunities,
            3_000
        );

        assert_eq!(
            result.total.errors,
            30
        );
    }

    #[test]
    fn incompatible_configurations_are_rejected() {
        let mut first =
            PhysicalQecBenchmark::new()
                .expect("valid benchmark");

        let mut second =
            PhysicalQecBenchmark::with_config(
                PhysicalQecConfig {
                    confidence_level: 0.99,
                    ..PhysicalQecConfig::default()
                },
            )
            .expect("valid benchmark");

        let result =
            first.merge(&second);

        assert!(matches!(
            result,
            Err(PhysicalQecError::IncompatibleResults { .. })
        ));

        // Keep `second` mutable in this test so this remains a valid
        // independent benchmark object for future extension.
        let _ = &mut second;
    }

    #[test]
    fn custom_error_kinds_are_bounded() {
        let config = PhysicalQecConfig {
            max_custom_error_kinds: 1,
            ..PhysicalQecConfig::default()
        };

        let mut benchmark =
            PhysicalQecBenchmark::with_config(config)
                .expect("valid benchmark");

        let first_kind =
            PhysicalErrorKind::custom("first")
                .expect("valid custom kind");

        let second_kind =
            PhysicalErrorKind::custom("second")
                .expect("valid custom kind");

        benchmark
            .record_error(first_kind, 100, 1)
            .expect("first custom kind should succeed");

        let result =
            benchmark.record_error(
                second_kind,
                100,
                1,
            );

        assert!(matches!(
            result,
            Err(PhysicalQecError::TooManyCustomErrorKinds)
        ));
    }

    #[test]
    fn deterministic_map_order_is_preserved() {
        let mut benchmark =
            PhysicalQecBenchmark::new()
                .expect("valid benchmark");

        benchmark
            .record_qubit_error(
                10,
                PhysicalErrorKind::Measurement,
                100,
                1,
            )
            .expect("valid observation");

        benchmark
            .record_qubit_error(
                2,
                PhysicalErrorKind::Measurement,
                100,
                2,
            )
            .expect("valid observation");

        benchmark
            .record_qubit_error(
                7,
                PhysicalErrorKind::Measurement,
                100,
                3,
            )
            .expect("valid observation");

        let result =
            benchmark.snapshot().expect("valid snapshot");

        let ids =
            result.by_qubit.keys().copied().collect::<Vec<_>>();

        assert_eq!(ids, vec![2, 7, 10]);
    }

    #[test]
    fn observation_from_qec_metrics_is_valid() {
        let observation =
            observation_from_qec_metrics(
                10_000,
                25,
            )
            .expect("valid metrics observation");

        assert_eq!(
            observation.kind,
            PhysicalErrorKind::Other
        );

        assert_eq!(
            observation.opportunities,
            10_000
        );

        assert_eq!(observation.errors, 25);
    }
}