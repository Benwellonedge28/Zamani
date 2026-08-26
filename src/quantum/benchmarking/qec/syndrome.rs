//! Zamani Quantum Benchmarking — QEC Syndrome Benchmark.
//!
//! # Purpose
//!
//! This module benchmarks the quality, structure, consistency, and derived
//! detection-event characteristics of quantum-error-correction syndrome
//! streams.
//!
//! It is a BENCHMARKING layer. It does not redefine the canonical syndrome
//! representation.
//!
//! The authoritative syndrome representation remains:
//!
//! ```text
//! crate::quantum::error_correction::syndrome
//! ```
//!
//! This module consumes that representation and produces benchmark-specific
//! measurements.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - syndrome benchmark configuration;
//! - syndrome-stream validation for benchmarking;
//! - measurement-count metrics;
//! - active-syndrome density;
//! - measurement-confidence statistics;
//! - detection-event metrics;
//! - syndrome-domain consistency metrics;
//! - temporal consistency metrics;
//! - conservative memory observations;
//! - benchmark-local result representation;
//! - deterministic aggregation over a bounded syndrome stream;
//! - benchmark warnings and validity status.
//!
//! This module does NOT own:
//!
//! - raw QPU communication;
//! - credentials;
//! - authentication;
//! - QPU capability authorization;
//! - raw measurement extraction;
//! - syndrome construction;
//! - surface-code topology;
//! - stabilizer algebra;
//! - decoding;
//! - decoder performance;
//! - Pauli-frame application;
//! - runtime resource accounting;
//! - memory allocation enforcement;
//! - telemetry transport;
//! - checkpoint persistence;
//! - statistical fitting unrelated to syndrome-stream metrics.
//!
//! # Architectural boundary
//!
//! ```text
//! QPU / simulator / replay
//!          |
//!          v
//! error_correction::syndrome_extractor
//!          |
//!          v
//! error_correction::syndrome::Syndrome
//!          |
//!          +-------------------------------+
//!          |                               |
//!          v                               v
//! QEC decoder pipeline          benchmarking::qec::syndrome
//!                                          |
//!                                          v
//!                              SyndromeBenchmarkResult
//!                                          |
//!                     +--------------------+-------------------+
//!                     |                    |                   |
//!                     v                    v                   v
//!                metrics/qec        reporting          analysis/regression
//! ```
//!
//! The benchmark must never become an alternative source of truth for QEC
//! semantics.
//!
//! # Integration contract
//!
//! Future benchmarking modules may consume this file without modifying its
//! public semantics:
//!
//! ```text
//! benchmarking::qec::syndrome
//!          |
//!          +--> benchmarking::metrics::logical
//!          +--> benchmarking::metrics::throughput
//!          +--> benchmarking::metrics::stability
//!          +--> benchmarking::core::result
//!          +--> benchmarking::reporting::json
//!          +--> benchmarking::analysis::baseline
//! ```
//!
//! The canonical QEC subsystem consumes none of the benchmarking subsystem.
//!
//! Therefore the dependency direction is strictly:
//!
//! ```text
//! error_correction
//!        ^
//!        |
//! benchmarking::qec::syndrome
//! ```
//!
//! and never:
//!
//! ```text
//! error_correction -> benchmarking
//! ```
//!
//! # Benchmark semantics
//!
//! A syndrome benchmark measures several independent dimensions.
//!
//! ## 1. Syndrome population
//!
//! Number of stabilizers represented in each round.
//!
//! ## 2. Active syndrome density
//!
//! ```text
//! active_measurements / total_measurements
//! ```
//!
//! This is an observational metric. It is NOT itself a logical-error-rate
//! estimate.
//!
//! ## 3. Measurement confidence
//!
//! Confidence is represented by the canonical QEC `MeasurementConfidence`
//! basis-point type. The benchmark reports:
//!
//! - minimum confidence;
//! - arithmetic mean confidence;
//! - active-measurement mean confidence.
//!
//! ## 4. Detection events
//!
//! Consecutive canonical syndromes are converted using the canonical QEC
//! operation:
//!
//! ```text
//! D(t) = S(t) XOR S(t-1)
//! ```
//!
//! The benchmark reports the number and density of detection events.
//!
//! ## 5. Domain consistency
//!
//! Every consecutive pair must contain exactly the same stabilizer domain.
//!
//! A changed domain invalidates the temporal benchmark because missing
//! stabilizers must never be silently interpreted as zero.
//!
//! ## 6. Temporal consistency
//!
//! Consecutive rounds must satisfy the canonical QEC requirements:
//!
//! - current round = previous round + 1;
//! - current timestamp >= previous timestamp.
//!
//! The canonical `Syndrome` implementation already enforces these conditions
//! when generating detection events. This benchmark invokes that authoritative
//! implementation instead of duplicating its semantics.
//!
//! # Important distinction
//!
//! This benchmark does NOT claim that:
//!
//! ```text
//! syndrome_density == physical_error_rate
//! ```
//!
//! or:
//!
//! ```text
//! detection_event_rate == logical_error_rate
//! ```
//!
//! Those interpretations require a code definition, noise model, decoder,
//! logical observable and appropriate statistical analysis.
//!
//! This module only measures syndrome-level observations.
//!
//! # Security and resource safety
//!
//! The benchmark never trusts caller-provided collection sizes blindly.
//!
//! It validates:
//!
//! - maximum rounds;
//! - maximum measurements per round;
//! - maximum total measurements;
//! - maximum detection events;
//! - maximum estimated memory;
//! - arithmetic overflow.
//!
//! No benchmark-specific production ceiling replaces `QecLimits`.
//!
//! The canonical `QecLimits` from the QEC subsystem remains authoritative.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Determinism
//!
//! Given the same ordered syndrome stream and configuration, this module
//! produces exactly the same result.
//!
//! No random number generator is used.
//! No global state is used.
//! No wall clock is read.
//! No environment state affects the result.
//!
//! # Scientific reproducibility
//!
//! This result intentionally contains observations rather than pretending to
//! be a complete scientific provenance record. The future
//! `benchmarking::core::provenance` layer should attach:
//!
//! - benchmark identity;
//! - Zamani version;
//! - QEC configuration fingerprint;
//! - backend identity;
//! - calibration identity;
//! - circuit fingerprint;
//! - execution identity;
//! - seed where applicable;
//! - compiler/routing/scheduling metadata.
//!
//! This file does not need to be edited when that higher-level provenance
//! system is introduced.
//!
//! # Production rule
//!
//! Never silently discard malformed rounds.
//!
//! A benchmark result is marked invalid when the supplied syndrome stream
//! violates canonical QEC invariants.
//!
//! ---------------------------------------------------------------------------
//! Rust 1.97.1
//! ---------------------------------------------------------------------------

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use crate::quantum::error_correction::syndrome::{
    DetectionEvent,
    MeasurementConfidence,
    Syndrome,
};

// ============================================================================
// Public identifiers and schema
// ============================================================================

/// Stable benchmark identifier.
pub const QEC_SYNDROME_BENCHMARK_ID: &str = "qec.syndrome";

/// Stable result schema version.
///
/// This version belongs to this benchmark result representation and is
/// independent from the QEC `QEC_LIMITS_SCHEMA_VERSION`.
pub const QEC_SYNDROME_BENCHMARK_SCHEMA_VERSION: u32 = 1;

/// Human-readable benchmark name.
pub const QEC_SYNDROME_BENCHMARK_NAME: &str =
    "Quantum Error Correction Syndrome Benchmark";

/// Exact unit for confidence values.
pub const CONFIDENCE_UNIT_BASIS_POINTS: &str = "basis_points";

/// Unit used for density metrics.
pub const DENSITY_UNIT: &str = "ratio";

/// Unit used for memory observations.
pub const MEMORY_UNIT_BYTES: &str = "bytes";

// ============================================================================
// Errors
// ============================================================================

/// Errors specific to syndrome benchmarking.
///
/// The canonical QEC subsystem owns QEC semantic errors. This type only
/// describes errors introduced by the benchmark's own policy or aggregation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyndromeBenchmarkError {
    /// No rounds were supplied.
    EmptyBenchmark,

    /// A configured limit is invalid.
    InvalidLimit {
        name: &'static str,
        value: usize,
    },

    /// The supplied stream contains more rounds than permitted.
    TooManyRounds {
        actual: usize,
        maximum: usize,
    },

    /// The supplied stream contains more total measurements than permitted.
    TooManyMeasurements {
        actual: usize,
        maximum: usize,
    },

    /// A single syndrome exceeds the benchmark measurement limit.
    TooManyMeasurementsInRound {
        round: u64,
        actual: usize,
        maximum: usize,
    },

    /// The derived detection-event count exceeds the benchmark limit.
    TooManyDetectionEvents {
        actual: usize,
        maximum: usize,
    },

    /// Estimated benchmark memory exceeds the benchmark policy.
    MemoryLimitExceeded {
        estimated_bytes: u64,
        maximum_bytes: u64,
    },

    /// Checked integer arithmetic overflowed.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// A floating-point metric could not be represented as a finite value.
    NonFiniteMetric {
        metric: &'static str,
    },

    /// A density could not be computed because the denominator was zero.
    UndefinedDensity {
        metric: &'static str,
    },

    /// A canonical QEC operation rejected the supplied stream.
    InvalidSyndromeStream {
        message: String,
    },
}

impl fmt::Display for SyndromeBenchmarkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBenchmark => {
                formatter.write_str(
                    "syndrome benchmark requires at least one round",
                )
            }

            Self::InvalidLimit { name, value } => {
                write!(
                    formatter,
                    "syndrome benchmark limit {name} must be greater \
                     than zero, got {value}"
                )
            }

            Self::TooManyRounds { actual, maximum } => {
                write!(
                    formatter,
                    "syndrome benchmark contains {actual} rounds; \
                     maximum is {maximum}"
                )
            }

            Self::TooManyMeasurements { actual, maximum } => {
                write!(
                    formatter,
                    "syndrome benchmark contains {actual} measurements; \
                     maximum is {maximum}"
                )
            }

            Self::TooManyMeasurementsInRound {
                round,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "syndrome round {round} contains {actual} measurements; \
                     maximum is {maximum}"
                )
            }

            Self::TooManyDetectionEvents { actual, maximum } => {
                write!(
                    formatter,
                    "syndrome benchmark produced {actual} detection events; \
                     maximum is {maximum}"
                )
            }

            Self::MemoryLimitExceeded {
                estimated_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "syndrome benchmark estimated memory usage of \
                     {estimated_bytes} bytes; maximum is {maximum_bytes}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "arithmetic overflow during {operation}"
                )
            }

            Self::NonFiniteMetric { metric } => {
                write!(
                    formatter,
                    "syndrome benchmark metric {metric} is not finite"
                )
            }

            Self::UndefinedDensity { metric } => {
                write!(
                    formatter,
                    "syndrome benchmark density {metric} is undefined"
                )
            }

            Self::InvalidSyndromeStream { message } => {
                write!(
                    formatter,
                    "invalid syndrome stream: {message}"
                )
            }
        }
    }
}

impl std::error::Error for SyndromeBenchmarkError {}

/// Result alias for this benchmark.
pub type SyndromeBenchmarkResult<T> =
    Result<T, SyndromeBenchmarkError>;

// ============================================================================
// Configuration
// ============================================================================

/// Production limits for one syndrome benchmark execution.
///
/// These are benchmark admission limits, not replacements for `QecLimits`.
///
/// The canonical syndrome itself remains governed by the `QecLimits` embedded
/// in each `Syndrome`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyndromeBenchmarkLimits {
    /// Maximum number of rounds accepted by one benchmark.
    pub max_rounds: usize,

    /// Maximum number of measurements in one round.
    pub max_measurements_per_round: usize,

    /// Maximum total measurements across all rounds.
    pub max_total_measurements: usize,

    /// Maximum derived detection events.
    pub max_detection_events: usize,

    /// Maximum estimated memory consumed by the benchmark's derived data.
    pub max_estimated_memory_bytes: u64,
}

impl SyndromeBenchmarkLimits {
    /// Creates explicit limits.
    pub const fn new(
        max_rounds: usize,
        max_measurements_per_round: usize,
        max_total_measurements: usize,
        max_detection_events: usize,
        max_estimated_memory_bytes: u64,
    ) -> Self {
        Self {
            max_rounds,
            max_measurements_per_round,
            max_total_measurements,
            max_detection_events,
            max_estimated_memory_bytes,
        }
    }

    /// Validates the benchmark-specific policy.
    pub fn validate(self) -> SyndromeBenchmarkResult<Self> {
        if self.max_rounds == 0 {
            return Err(SyndromeBenchmarkError::InvalidLimit {
                name: "max_rounds",
                value: self.max_rounds,
            });
        }

        if self.max_measurements_per_round == 0 {
            return Err(SyndromeBenchmarkError::InvalidLimit {
                name: "max_measurements_per_round",
                value: self.max_measurements_per_round,
            });
        }

        if self.max_total_measurements == 0 {
            return Err(SyndromeBenchmarkError::InvalidLimit {
                name: "max_total_measurements",
                value: self.max_total_measurements,
            });
        }

        if self.max_detection_events == 0 {
            return Err(SyndromeBenchmarkError::InvalidLimit {
                name: "max_detection_events",
                value: self.max_detection_events,
            });
        }

        if self.max_estimated_memory_bytes == 0 {
            return Err(SyndromeBenchmarkError::InvalidLimit {
                name: "max_estimated_memory_bytes",
                value: self.max_estimated_memory_bytes,
            });
        }

        Ok(self)
    }
}

impl Default for SyndromeBenchmarkLimits {
    fn default() -> Self {
        Self {
            // These are deliberately bounded benchmark-admission defaults.
            //
            // They are not QEC production resource policy. QecLimits remains
            // the authoritative QEC resource policy.
            max_rounds: 1_000_000,
            max_measurements_per_round: 10_000_000,
            max_total_measurements: 100_000_000,
            max_detection_events: 100_000_000,
            max_estimated_memory_bytes: 16 * 1024 * 1024 * 1024,
        }
    }
}

/// Benchmark configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyndromeBenchmarkConfig {
    /// Benchmark-specific admission limits.
    pub limits: SyndromeBenchmarkLimits,

    /// Minimum acceptable individual measurement confidence in basis points.
    ///
    /// This is diagnostic only. A low-confidence measurement is not silently
    /// removed from the benchmark.
    pub minimum_confidence_basis_points: u16,

    /// Whether a low-confidence observation makes the result invalid.
    ///
    /// When false, low confidence produces a warning in the result.
    pub reject_below_minimum_confidence: bool,
}

impl SyndromeBenchmarkConfig {
    /// Creates the production default configuration.
    pub fn new() -> SyndromeBenchmarkResult<Self> {
        Self {
            limits: SyndromeBenchmarkLimits::default(),
            minimum_confidence_basis_points: 0,
            reject_below_minimum_confidence: false,
        }
        .validate()
    }

    /// Creates a configuration with explicit benchmark limits.
    pub fn with_limits(
        limits: SyndromeBenchmarkLimits,
    ) -> SyndromeBenchmarkResult<Self> {
        Self {
            limits,
            minimum_confidence_basis_points: 0,
            reject_below_minimum_confidence: false,
        }
        .validate()
    }

    /// Validates configuration.
    pub fn validate(self) -> SyndromeBenchmarkResult<Self> {
        self.limits.validate()?;

        if self.minimum_confidence_basis_points > 10_000 {
            return Err(SyndromeBenchmarkError::InvalidLimit {
                name: "minimum_confidence_basis_points",
                value: self.minimum_confidence_basis_points as usize,
            });
        }

        Ok(self)
    }
}

// ============================================================================
// Warning and validity state
// ============================================================================

/// Non-fatal benchmark observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyndromeBenchmarkWarning {
    /// At least one measurement was below the configured confidence floor.
    LowMeasurementConfidence,

    /// At least one round contained no active syndrome bits.
    TrivialSyndromeRound,

    /// The benchmark contained only one round, so detection-event metrics
    /// could not be computed.
    SingleRoundNoDetectionEvents,

    /// The stream contained no detection events.
    NoDetectionEvents,

    /// Detection-event density was computed from consecutive rounds.
    DetectionDensityIsTemporal,

    /// Memory is an estimate, not an allocator measurement.
    MemoryIsEstimated,
}

impl SyndromeBenchmarkWarning {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LowMeasurementConfidence => "low_measurement_confidence",
            Self::TrivialSyndromeRound => "trivial_syndrome_round",
            Self::SingleRoundNoDetectionEvents => {
                "single_round_no_detection_events"
            }
            Self::NoDetectionEvents => "no_detection_events",
            Self::DetectionDensityIsTemporal => {
                "detection_density_is_temporal"
            }
            Self::MemoryIsEstimated => "memory_is_estimated",
        }
    }
}

/// Overall benchmark validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyndromeBenchmarkValidity {
    /// All supplied rounds passed benchmark and canonical QEC validation.
    Valid,

    /// The result is structurally valid but contains diagnostic warnings.
    ValidWithWarnings,
}

impl SyndromeBenchmarkValidity {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::ValidWithWarnings => "valid_with_warnings",
        }
    }
}

// ============================================================================
// Per-round observation
// ============================================================================

/// Metrics for one syndrome round.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SyndromeRoundObservation {
    /// Measurement round identifier.
    pub round: u64,

    /// Backend-independent timestamp.
    pub timestamp: u64,

    /// Number of stabilizer measurements.
    pub measurement_count: usize,

    /// Number of non-trivial syndrome bits.
    pub active_measurement_count: usize,

    /// Active measurement density.
    pub active_density: f64,

    /// Minimum confidence across all measurements.
    pub minimum_confidence_basis_points: u16,

    /// Mean confidence across all measurements.
    pub mean_confidence_basis_points: f64,

    /// Mean confidence among active measurements.
    ///
    /// `None` means the round contained no active measurements.
    pub mean_active_confidence_basis_points: Option<f64>,

    /// Conservative representation memory estimate.
    pub estimated_memory_bytes: u64,

    /// Whether the round is trivial.
    pub is_trivial: bool,
}

impl SyndromeRoundObservation {
    /// Constructs an observation from a canonical syndrome.
    pub fn from_syndrome(
        syndrome: &Syndrome,
        limits: SyndromeBenchmarkLimits,
    ) -> SyndromeBenchmarkResult<Self> {
        let measurement_count = syndrome.len();

        if measurement_count > limits.max_measurements_per_round {
            return Err(
                SyndromeBenchmarkError::TooManyMeasurementsInRound {
                    round: syndrome.round().value(),
                    actual: measurement_count,
                    maximum: limits.max_measurements_per_round,
                },
            );
        }

        let estimated_memory_bytes =
            syndrome.estimated_memory_bytes().map_err(|error| {
                SyndromeBenchmarkError::InvalidSyndromeStream {
                    message: error.to_string(),
                }
            })?;

        if estimated_memory_bytes > limits.max_estimated_memory_bytes {
            return Err(
                SyndromeBenchmarkError::MemoryLimitExceeded {
                    estimated_bytes: estimated_memory_bytes,
                    maximum_bytes: limits.max_estimated_memory_bytes,
                },
            );
        }

        if measurement_count == 0 {
            return Ok(Self {
                round: syndrome.round().value(),
                timestamp: syndrome.timestamp().value(),
                measurement_count: 0,
                active_measurement_count: 0,
                active_density: 0.0,
                minimum_confidence_basis_points: 0,
                mean_confidence_basis_points: 0.0,
                mean_active_confidence_basis_points: None,
                estimated_memory_bytes,
                is_trivial: true,
            });
        }

        let mut active_measurement_count = 0usize;
        let mut confidence_sum = 0u64;
        let mut active_confidence_sum = 0u64;
        let mut minimum_confidence = u16::MAX;

        for measurement in syndrome.measurements() {
            let confidence = measurement.confidence();

            let basis_points = confidence.basis_points();

            minimum_confidence =
                minimum_confidence.min(basis_points);

            confidence_sum = confidence_sum
                .checked_add(u64::from(basis_points))
                .ok_or(SyndromeBenchmarkError::ArithmeticOverflow {
                    operation: "confidence_sum",
                })?;

            if measurement.value() {
                active_measurement_count = active_measurement_count
                    .checked_add(1)
                    .ok_or(
                        SyndromeBenchmarkError::ArithmeticOverflow {
                            operation: "active_measurement_count",
                        },
                    )?;

                active_confidence_sum = active_confidence_sum
                    .checked_add(u64::from(basis_points))
                    .ok_or(
                        SyndromeBenchmarkError::ArithmeticOverflow {
                            operation: "active_confidence_sum",
                        },
                    )?;
            }
        }

        let measurement_count_u64 =
            u64::try_from(measurement_count).map_err(|_| {
                SyndromeBenchmarkError::ArithmeticOverflow {
                    operation: "measurement_count conversion",
                }
            })?;

        let active_count_u64 =
            u64::try_from(active_measurement_count).map_err(|_| {
                SyndromeBenchmarkError::ArithmeticOverflow {
                    operation: "active_measurement_count conversion",
                }
            })?;

        let active_density =
            active_measurement_count as f64 / measurement_count as f64;

        let mean_confidence =
            confidence_sum as f64 / measurement_count_u64 as f64;

        let mean_active_confidence =
            if active_measurement_count == 0 {
                None
            } else {
                Some(
                    active_confidence_sum as f64
                        / active_count_u64 as f64,
                )
            };

        if !active_density.is_finite() {
            return Err(SyndromeBenchmarkError::NonFiniteMetric {
                metric: "active_density",
            });
        }

        if !mean_confidence.is_finite() {
            return Err(SyndromeBenchmarkError::NonFiniteMetric {
                metric: "mean_confidence_basis_points",
            });
        }

        if let Some(value) = mean_active_confidence {
            if !value.is_finite() {
                return Err(SyndromeBenchmarkError::NonFiniteMetric {
                    metric: "mean_active_confidence_basis_points",
                });
            }
        }

        Ok(Self {
            round: syndrome.round().value(),
            timestamp: syndrome.timestamp().value(),
            measurement_count,
            active_measurement_count,
            active_density,
            minimum_confidence_basis_points: minimum_confidence,
            mean_confidence_basis_points: mean_confidence,
            mean_active_confidence_basis_points: mean_active_confidence,
            estimated_memory_bytes,
            is_trivial: active_measurement_count == 0,
        })
    }
}

// ============================================================================
// Detection-event observation
// ============================================================================

/// Metrics derived from two consecutive syndrome rounds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DetectionEventObservation {
    /// Previous round.
    pub previous_round: u64,

    /// Current round.
    pub current_round: u64,

    /// Number of stabilizers compared.
    pub compared_measurements: usize,

    /// Number of detection events.
    pub detection_event_count: usize,

    /// Detection-event density relative to compared stabilizers.
    pub detection_event_density: f64,

    /// Mean confidence of detection events.
    ///
    /// `None` means no detection events occurred.
    pub mean_detection_confidence_basis_points: Option<f64>,

    /// Minimum confidence among detection events.
    ///
    /// `None` means no detection events occurred.
    pub minimum_detection_confidence_basis_points: Option<u16>,
}

impl DetectionEventObservation {
    /// Constructs a detection-event observation from the canonical QEC event
    /// list and the compared syndrome width.
    pub fn from_events(
        previous: &Syndrome,
        current: &Syndrome,
        events: &[DetectionEvent],
        limits: SyndromeBenchmarkLimits,
    ) -> SyndromeBenchmarkResult<Self> {
        let compared_measurements = current.len();

        if events.len() > limits.max_detection_events {
            return Err(
                SyndromeBenchmarkError::TooManyDetectionEvents {
                    actual: events.len(),
                    maximum: limits.max_detection_events,
                },
            );
        }

        if compared_measurements == 0 {
            return Err(SyndromeBenchmarkError::UndefinedDensity {
                metric: "detection_event_density",
            });
        }

        let mut confidence_sum = 0u64;
        let mut minimum_confidence = u16::MAX;

        for event in events {
            let basis_points =
                event.confidence().basis_points();

            confidence_sum = confidence_sum
                .checked_add(u64::from(basis_points))
                .ok_or(SyndromeBenchmarkError::ArithmeticOverflow {
                    operation: "detection_confidence_sum",
                })?;

            minimum_confidence =
                minimum_confidence.min(basis_points);
        }

        let density =
            events.len() as f64 / compared_measurements as f64;

        if !density.is_finite() {
            return Err(SyndromeBenchmarkError::NonFiniteMetric {
                metric: "detection_event_density",
            });
        }

        let mean_confidence = if events.is_empty() {
            None
        } else {
            Some(
                confidence_sum as f64
                    / events.len() as f64,
            )
        };

        if let Some(value) = mean_confidence {
            if !value.is_finite() {
                return Err(SyndromeBenchmarkError::NonFiniteMetric {
                    metric: "mean_detection_confidence_basis_points",
                });
            }
        }

        Ok(Self {
            previous_round: previous.round().value(),
            current_round: current.round().value(),
            compared_measurements,
            detection_event_count: events.len(),
            detection_event_density: density,
            mean_detection_confidence_basis_points: mean_confidence,
            minimum_detection_confidence_basis_points:
                if events.is_empty() {
                    None
                } else {
                    Some(minimum_confidence)
                },
        })
    }
}

// ============================================================================
// Aggregate result
// ============================================================================

/// Aggregate syndrome benchmark result.
///
/// This is deliberately self-contained so it can later be wrapped directly
/// into the universal `benchmarking::core::BenchmarkResult`.
#[derive(Debug, Clone, PartialEq)]
pub struct SyndromeBenchmarkReport {
    /// Stable benchmark identifier.
    pub benchmark_id: &'static str,

    /// Benchmark result schema version.
    pub schema_version: u32,

    /// Number of rounds analyzed.
    pub round_count: usize,

    /// Total number of stabilizer measurements.
    pub total_measurements: usize,

    /// Total number of active syndrome measurements.
    pub total_active_measurements: usize,

    /// Aggregate active syndrome density.
    pub active_density: f64,

    /// Mean confidence over all measurements.
    pub mean_confidence_basis_points: f64,

    /// Minimum confidence observed.
    pub minimum_confidence_basis_points: u16,

    /// Number of rounds with no active syndrome bits.
    pub trivial_round_count: usize,

    /// Number of consecutive round transitions analyzed.
    pub transition_count: usize,

    /// Total detection events.
    pub total_detection_events: usize,

    /// Detection-event density over all compared stabilizers.
    ///
    /// `None` occurs when fewer than two rounds were supplied.
    pub detection_event_density: Option<f64>,

    /// Mean detection-event confidence.
    pub mean_detection_confidence_basis_points: Option<f64>,

    /// Minimum detection-event confidence.
    pub minimum_detection_confidence_basis_points: Option<u16>,

    /// Sum of conservative syndrome memory estimates.
    pub estimated_syndrome_memory_bytes: u64,

    /// Benchmark validity.
    pub validity: SyndromeBenchmarkValidity,

    /// Non-fatal observations.
    pub warnings: Vec<SyndromeBenchmarkWarning>,

    /// Per-round observations.
    pub rounds: Vec<SyndromeRoundObservation>,

    /// Per-transition detection observations.
    pub transitions: Vec<DetectionEventObservation>,
}

impl SyndromeBenchmarkReport {
    /// Returns true when the benchmark contains at least one warning.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Returns the benchmark identifier.
    #[must_use]
    pub const fn benchmark_id(&self) -> &'static str {
        self.benchmark_id
    }

    /// Returns the result schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }
}

// ============================================================================
// Analyzer
// ============================================================================

/// Production syndrome benchmark analyzer.
///
/// The analyzer is immutable after construction and therefore safe to reuse
/// for multiple independent benchmark executions.
#[derive(Debug, Clone, Copy)]
pub struct SyndromeBenchmarkAnalyzer {
    config: SyndromeBenchmarkConfig,
}

impl SyndromeBenchmarkAnalyzer {
    /// Creates an analyzer using production defaults.
    pub fn new() -> SyndromeBenchmarkResult<Self> {
        Self::with_config(SyndromeBenchmarkConfig::new()?)
    }

    /// Creates an analyzer using explicit configuration.
    pub fn with_config(
        config: SyndromeBenchmarkConfig,
    ) -> SyndromeBenchmarkResult<Self> {
        Ok(Self {
            config: config.validate()?,
        })
    }

    /// Returns the benchmark configuration.
    #[must_use]
    pub const fn config(&self) -> SyndromeBenchmarkConfig {
        self.config
    }

    /// Benchmarks an ordered syndrome stream.
    ///
    /// The input order is authoritative. The analyzer does not sort the
    /// caller's slice because silently reordering hardware/replay data could
    /// hide an upstream temporal integrity failure.
    pub fn analyze(
        &self,
        syndromes: &[Syndrome],
    ) -> SyndromeBenchmarkResult<SyndromeBenchmarkReport> {
        self.validate_stream_admission(syndromes)?;

        let mut rounds = Vec::with_capacity(syndromes.len());

        let mut total_measurements = 0usize;
        let mut total_active_measurements = 0usize;
        let mut total_confidence = 0u64;
        let mut minimum_confidence = u16::MAX;
        let mut trivial_round_count = 0usize;
        let mut estimated_memory_bytes = 0u64;

        let mut warnings = Vec::new();

        for syndrome in syndromes {
            let observation =
                SyndromeRoundObservation::from_syndrome(
                    syndrome,
                    self.config.limits,
                )?;

            if observation.is_trivial {
                trivial_round_count =
                    trivial_round_count.checked_add(1).ok_or(
                        SyndromeBenchmarkError::ArithmeticOverflow {
                            operation: "trivial_round_count",
                        },
                    )?;

                push_warning_unique(
                    &mut warnings,
                    SyndromeBenchmarkWarning::TrivialSyndromeRound,
                );
            }

            if observation.minimum_confidence_basis_points
                < self.config.minimum_confidence_basis_points
            {
                if self.config.reject_below_minimum_confidence {
                    return Err(
                        SyndromeBenchmarkError::InvalidSyndromeStream {
                            message: format!(
                                "round {} contains a measurement below \
                                 the configured confidence floor",
                                observation.round
                            ),
                        },
                    );
                }

                push_warning_unique(
                    &mut warnings,
                    SyndromeBenchmarkWarning::LowMeasurementConfidence,
                );
            }

            total_measurements =
                total_measurements.checked_add(
                    observation.measurement_count,
                )
                .ok_or(
                    SyndromeBenchmarkError::ArithmeticOverflow {
                        operation: "total_measurements",
                    },
                )?;

            if total_measurements
                > self.config.limits.max_total_measurements
            {
                return Err(
                    SyndromeBenchmarkError::TooManyMeasurements {
                        actual: total_measurements,
                        maximum: self
                            .config
                            .limits
                            .max_total_measurements,
                    },
                );
            }

            total_active_measurements =
                total_active_measurements.checked_add(
                    observation.active_measurement_count,
                )
                .ok_or(
                    SyndromeBenchmarkError::ArithmeticOverflow {
                        operation: "total_active_measurements",
                    },
                )?;

            total_confidence = total_confidence
                .checked_add(
                    u64::from(observation.minimum_confidence_basis_points)
                        .checked_mul(0)
                        .unwrap_or(0),
                )
                .ok_or(
                    SyndromeBenchmarkError::ArithmeticOverflow {
                        operation: "confidence_placeholder",
                    },
                )?;

            // Recompute the exact confidence sum from the canonical
            // measurements. The per-round observation deliberately exposes
            // means rather than an additional hidden accumulator.
            //
            // This is kept explicit so the aggregate cannot depend on
            // floating-point reconstruction.
            for measurement in syndrome.measurements() {
                total_confidence = total_confidence
                    .checked_add(
                        u64::from(
                            measurement.confidence().basis_points(),
                        ),
                    )
                    .ok_or(
                        SyndromeBenchmarkError::ArithmeticOverflow {
                            operation: "total_confidence",
                        },
                    )?;

                minimum_confidence = minimum_confidence.min(
                    measurement.confidence().basis_points(),
                );
            }

            estimated_memory_bytes =
                estimated_memory_bytes
                    .checked_add(
                        observation.estimated_memory_bytes,
                    )
                    .ok_or(
                        SyndromeBenchmarkError::ArithmeticOverflow {
                            operation: "estimated_memory_bytes",
                        },
                    )?;

            if estimated_memory_bytes
                > self.config.limits.max_estimated_memory_bytes
            {
                return Err(
                    SyndromeBenchmarkError::MemoryLimitExceeded {
                        estimated_bytes: estimated_memory_bytes,
                        maximum_bytes: self
                            .config
                            .limits
                            .max_estimated_memory_bytes,
                    },
                );
            }

            rounds.push(observation);
        }

        // The loop above already guarantees a non-empty stream.
        if total_measurements == 0 {
            minimum_confidence = 0;
        }

        let active_density =
            total_active_measurements as f64
                / total_measurements as f64;

        if !active_density.is_finite() {
            return Err(SyndromeBenchmarkError::NonFiniteMetric {
                metric: "aggregate_active_density",
            });
        }

        // The exact aggregate confidence must be based on the total number
        // of measurements, not on the mean of per-round means.
        //
        // `total_confidence` contains the exact basis-point sum.
        let mean_confidence =
            total_confidence as f64
                / total_measurements as f64;

        if !mean_confidence.is_finite() {
            return Err(SyndromeBenchmarkError::NonFiniteMetric {
                metric: "aggregate_mean_confidence",
            });
        }

        let mut transitions = Vec::new();

        let mut total_detection_events = 0usize;
        let mut total_compared_measurements = 0usize;
        let mut total_detection_confidence = 0u64;
        let mut minimum_detection_confidence: Option<u16> = None;

        if syndromes.len() == 1 {
            push_warning_unique(
                &mut warnings,
                SyndromeBenchmarkWarning::SingleRoundNoDetectionEvents,
            );
        }

        for pair in syndromes.windows(2) {
            let previous = &pair[0];
            let current = &pair[1];

            let events = current
                .detection_events_against(previous)
                .map_err(|error| {
                    SyndromeBenchmarkError::InvalidSyndromeStream {
                        message: error.to_string(),
                    }
                })?;

            let transition =
                DetectionEventObservation::from_events(
                    previous,
                    current,
                    &events,
                    self.config.limits,
                )?;

            total_detection_events =
                total_detection_events.checked_add(
                    transition.detection_event_count,
                )
                .ok_or(
                    SyndromeBenchmarkError::ArithmeticOverflow {
                        operation: "total_detection_events",
                    },
                )?;

            if total_detection_events
                > self.config.limits.max_detection_events
            {
                return Err(
                    SyndromeBenchmarkError::TooManyDetectionEvents {
                        actual: total_detection_events,
                        maximum: self
                            .config
                            .limits
                            .max_detection_events,
                    },
                );
            }

            total_compared_measurements =
                total_compared_measurements.checked_add(
                    transition.compared_measurements,
                )
                .ok_or(
                    SyndromeBenchmarkError::ArithmeticOverflow {
                        operation: "total_compared_measurements",
                    },
                )?;

            for event in &events {
                total_detection_confidence =
                    total_detection_confidence
                        .checked_add(u64::from(
                            event.confidence().basis_points(),
                        ))
                        .ok_or(
                            SyndromeBenchmarkError::ArithmeticOverflow {
                                operation:
                                    "total_detection_confidence",
                            },
                        )?;

                let confidence =
                    event.confidence().basis_points();

                minimum_detection_confidence =
                    Some(match minimum_detection_confidence {
                        Some(current_min) => {
                            current_min.min(confidence)
                        }
                        None => confidence,
                    });
            }

            transitions.push(transition);
        }

        let detection_event_density =
            if syndromes.len() < 2 {
                None
            } else if total_compared_measurements == 0 {
                return Err(
                    SyndromeBenchmarkError::UndefinedDensity {
                        metric: "aggregate_detection_event_density",
                    },
                );
            } else {
                let value =
                    total_detection_events as f64
                        / total_compared_measurements as f64;

                if !value.is_finite() {
                    return Err(
                        SyndromeBenchmarkError::NonFiniteMetric {
                            metric:
                                "aggregate_detection_event_density",
                        },
                    );
                }

                Some(value)
            };

        let mean_detection_confidence =
            if total_detection_events == 0 {
                push_warning_unique(
                    &mut warnings,
                    SyndromeBenchmarkWarning::NoDetectionEvents,
                );

                None
            } else {
                let value =
                    total_detection_confidence as f64
                        / total_detection_events as f64;

                if !value.is_finite() {
                    return Err(
                        SyndromeBenchmarkError::NonFiniteMetric {
                            metric:
                                "aggregate_mean_detection_confidence",
                        },
                    );
                }

                Some(value)
            };

        if detection_event_density.is_some() {
            push_warning_unique(
                &mut warnings,
                SyndromeBenchmarkWarning::DetectionDensityIsTemporal,
            );
        }

        push_warning_unique(
            &mut warnings,
            SyndromeBenchmarkWarning::MemoryIsEstimated,
        );

        let validity = if warnings.is_empty() {
            SyndromeBenchmarkValidity::Valid
        } else {
            SyndromeBenchmarkValidity::ValidWithWarnings
        };

        Ok(SyndromeBenchmarkReport {
            benchmark_id: QEC_SYNDROME_BENCHMARK_ID,
            schema_version: QEC_SYNDROME_BENCHMARK_SCHEMA_VERSION,
            round_count: syndromes.len(),
            total_measurements,
            total_active_measurements,
            active_density,
            mean_confidence_basis_points: mean_confidence,
            minimum_confidence_basis_points: minimum_confidence,
            trivial_round_count,
            transition_count: transitions.len(),
            total_detection_events,
            detection_event_density,
            mean_detection_confidence_basis_points:
                mean_detection_confidence,
            minimum_detection_confidence_basis_points:
                minimum_detection_confidence,
            estimated_syndrome_memory_bytes: estimated_memory_bytes,
            validity,
            warnings,
            rounds,
            transitions,
        })
    }

    fn validate_stream_admission(
        &self,
        syndromes: &[Syndrome],
    ) -> SyndromeBenchmarkResult<()> {
        if syndromes.is_empty() {
            return Err(SyndromeBenchmarkError::EmptyBenchmark);
        }

        if syndromes.len() > self.config.limits.max_rounds {
            return Err(SyndromeBenchmarkError::TooManyRounds {
                actual: syndromes.len(),
                maximum: self.config.limits.max_rounds,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Utility functions
// ============================================================================

/// Adds a warning once, preserving deterministic warning order.
fn push_warning_unique(
    warnings: &mut Vec<SyndromeBenchmarkWarning>,
    warning: SyndromeBenchmarkWarning,
) {
    if !warnings.contains(&warning) {
        warnings.push(warning);
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::error_correction::syndrome::{
        MeasurementConfidence,
        MeasurementRound,
        MeasurementTimestamp,
        StabilizerId,
        SyndromeMeasurement,
        SyndromeOptions,
    };

    fn confidence(value: u16) -> MeasurementConfidence {
        MeasurementConfidence::from_basis_points(value)
            .expect("test confidence must be valid")
    }

    fn syndrome(
        round: u64,
        timestamp: u64,
        values: &[(usize, bool)],
    ) -> Syndrome {
        let round =
            MeasurementRound::new(round)
                .expect("test round must be valid");

        let timestamp =
            MeasurementTimestamp::new(timestamp)
                .expect("test timestamp must be valid");

        let measurements = values.iter().map(|(id, value)| {
            SyndromeMeasurement::new(
                StabilizerId::new(*id),
                *value,
                confidence(10_000),
            )
        });

        Syndrome::from_measurements(
            round,
            timestamp,
            measurements,
            SyndromeOptions::default(),
        )
        .expect("test syndrome must be valid")
    }

    #[test]
    fn default_configuration_is_valid() {
        let config =
            SyndromeBenchmarkConfig::new()
                .expect("default config must be valid");

        assert_eq!(
            config.minimum_confidence_basis_points,
            0
        );
    }

    #[test]
    fn single_round_produces_round_metrics() {
        let analyzer =
            SyndromeBenchmarkAnalyzer::new()
                .expect("analyzer must be constructible");

        let input = [
            syndrome(
                0,
                100,
                &[
                    (0, false),
                    (1, true),
                    (2, false),
                    (3, true),
                ],
            ),
        ];

        let result = analyzer
            .analyze(&input)
            .expect("benchmark must succeed");

        assert_eq!(result.round_count, 1);
        assert_eq!(result.total_measurements, 4);
        assert_eq!(result.total_active_measurements, 2);
        assert_eq!(result.trivial_round_count, 0);

        assert!(
            (result.active_density - 0.5).abs() < 1e-12
        );

        assert_eq!(
            result.minimum_confidence_basis_points,
            10_000
        );

        assert!(
            result.detection_event_density.is_none()
        );
    }

    #[test]
    fn consecutive_rounds_generate_detection_metrics() {
        let analyzer =
            SyndromeBenchmarkAnalyzer::new()
                .expect("analyzer must be constructible");

        let first = syndrome(
            0,
            100,
            &[
                (0, false),
                (1, false),
                (2, false),
                (3, false),
            ],
        );

        let second = syndrome(
            1,
            200,
            &[
                (0, true),
                (1, false),
                (2, true),
                (3, false),
            ],
        );

        let input = [first, second];

        let result = analyzer
            .analyze(&input)
            .expect("benchmark must succeed");

        assert_eq!(result.transition_count, 1);
        assert_eq!(result.total_detection_events, 2);

        assert!(
            (result.detection_event_density.unwrap() - 0.5)
                .abs()
                < 1e-12
        );
    }

    #[test]
    fn identical_rounds_have_zero_detection_events() {
        let analyzer =
            SyndromeBenchmarkAnalyzer::new()
                .expect("analyzer must be constructible");

        let first = syndrome(
            0,
            100,
            &[
                (0, true),
                (1, false),
            ],
        );

        let second = syndrome(
            1,
            200,
            &[
                (0, true),
                (1, false),
            ],
        );

        let result = analyzer
            .analyze(&[first, second])
            .expect("benchmark must succeed");

        assert_eq!(result.total_detection_events, 0);

        assert_eq!(
            result.detection_event_density,
            Some(0.0)
        );

        assert!(
            result
                .warnings
                .contains(
                    &SyndromeBenchmarkWarning::NoDetectionEvents
                )
        );
    }

    #[test]
    fn trivial_round_is_reported_as_warning() {
        let analyzer =
            SyndromeBenchmarkAnalyzer::new()
                .expect("analyzer must be constructible");

        let input = [
            syndrome(
                0,
                100,
                &[
                    (0, false),
                    (1, false),
                ],
            ),
        ];

        let result = analyzer
            .analyze(&input)
            .expect("benchmark must succeed");

        assert_eq!(result.trivial_round_count, 1);

        assert!(
            result
                .warnings
                .contains(
                    &SyndromeBenchmarkWarning::TrivialSyndromeRound
                )
        );
    }

    #[test]
    fn confidence_floor_can_reject_measurements() {
        let config = SyndromeBenchmarkConfig {
            limits: SyndromeBenchmarkLimits::default(),
            minimum_confidence_basis_points: 9_000,
            reject_below_minimum_confidence: true,
        };

        let analyzer =
            SyndromeBenchmarkAnalyzer::with_config(config)
                .expect("config must be valid");

        let round =
            MeasurementRound::new(0)
                .expect("test round must be valid");

        let timestamp =
            MeasurementTimestamp::new(100)
                .expect("test timestamp must be valid");

        let measurements = [
            SyndromeMeasurement::new(
                StabilizerId::new(0),
                true,
                confidence(8_000),
            ),
        ];

        let syndrome =
            Syndrome::from_measurements(
                round,
                timestamp,
                measurements,
                SyndromeOptions::default(),
            )
            .expect("test syndrome must be valid");

        let result = analyzer.analyze(&[syndrome]);

        assert!(result.is_err());
    }

    #[test]
    fn confidence_floor_can_warn_without_discarding_data() {
        let config = SyndromeBenchmarkConfig {
            limits: SyndromeBenchmarkLimits::default(),
            minimum_confidence_basis_points: 9_000,
            reject_below_minimum_confidence: false,
        };

        let analyzer =
            SyndromeBenchmarkAnalyzer::with_config(config)
                .expect("config must be valid");

        let round =
            MeasurementRound::new(0)
                .expect("test round must be valid");

        let timestamp =
            MeasurementTimestamp::new(100)
                .expect("test timestamp must be valid");

        let measurements = [
            SyndromeMeasurement::new(
                StabilizerId::new(0),
                true,
                confidence(8_000),
            ),
        ];

        let syndrome =
            Syndrome::from_measurements(
                round,
                timestamp,
                measurements,
                SyndromeOptions::default(),
            )
            .expect("test syndrome must be valid");

        let result = analyzer
            .analyze(&[syndrome])
            .expect("benchmark must succeed");

        assert_eq!(result.total_measurements, 1);

        assert!(
            result
                .warnings
                .contains(
                    &SyndromeBenchmarkWarning::LowMeasurementConfidence
                )
        );
    }

    #[test]
    fn non_consecutive_rounds_are_rejected_by_canonical_qec_logic() {
        let analyzer =
            SyndromeBenchmarkAnalyzer::new()
                .expect("analyzer must be constructible");

        let first = syndrome(
            0,
            100,
            &[
                (0, false),
                (1, false),
            ],
        );

        let third = syndrome(
            2,
            300,
            &[
                (0, true),
                (1, false),
            ],
        );

        let result = analyzer.analyze(&[first, third]);

        assert!(result.is_err());

        match result {
            Err(SyndromeBenchmarkError::InvalidSyndromeStream {
                ..
            }) => {}
            other => panic!(
                "expected invalid syndrome stream, got {other:?}"
            ),
        }
    }

    #[test]
    fn timestamp_regression_is_rejected_by_canonical_qec_logic() {
        let analyzer =
            SyndromeBenchmarkAnalyzer::new()
                .expect("analyzer must be constructible");

        let first = syndrome(
            0,
            200,
            &[
                (0, false),
                (1, false),
            ],
        );

        let second = syndrome(
            1,
            100,
            &[
                (0, true),
                (1, false),
            ],
        );

        let result = analyzer.analyze(&[first, second]);

        assert!(result.is_err());
    }

    #[test]
    fn stabilizer_domain_changes_are_rejected() {
        let analyzer =
            SyndromeBenchmarkAnalyzer::new()
                .expect("analyzer must be constructible");

        let first = syndrome(
            0,
            100,
            &[
                (0, false),
                (1, false),
            ],
        );

        let second = syndrome(
            1,
            200,
            &[
                (0, true),
                (2, false),
            ],
        );

        let result = analyzer.analyze(&[first, second]);

        assert!(result.is_err());
    }

    #[test]
    fn empty_input_is_rejected() {
        let analyzer =
            SyndromeBenchmarkAnalyzer::new()
                .expect("analyzer must be constructible");

        let result = analyzer.analyze(&[]);

        assert_eq!(
            result,
            Err(SyndromeBenchmarkError::EmptyBenchmark)
        );
    }

    #[test]
    fn deterministic_repeated_analysis_produces_equal_results() {
        let analyzer =
            SyndromeBenchmarkAnalyzer::new()
                .expect("analyzer must be constructible");

        let input = [
            syndrome(
                0,
                100,
                &[
                    (0, false),
                    (1, true),
                    (2, false),
                ],
            ),
            syndrome(
                1,
                200,
                &[
                    (0, true),
                    (1, true),
                    (2, false),
                ],
            ),
        ];

        let first = analyzer
            .analyze(&input)
            .expect("first analysis must succeed");

        let second = analyzer
            .analyze(&input)
            .expect("second analysis must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn benchmark_id_and_schema_are_stable() {
        assert_eq!(
            QEC_SYNDROME_BENCHMARK_ID,
            "qec.syndrome"
        );

        assert_eq!(
            QEC_SYNDROME_BENCHMARK_SCHEMA_VERSION,
            1
        );
    }
}