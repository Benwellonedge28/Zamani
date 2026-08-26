//! Zamani Quantum Benchmarking — Mirror Circuit Benchmark
//!
//! Production mirror-circuit benchmarking protocol.
//!
//! # Purpose
//!
//! This module analyzes logical mirror circuits of the form:
//!
//! ```text
//! |psi> -- U -- U† -- M
//! ```
//!
//! where `U†` is the exact logical inverse generated from the same forward
//! circuit.
//!
//! In an ideal noiseless implementation, the complete circuit returns the
//! input state. For the default computational-basis input `|0...0>`, the ideal
//! output is therefore `0...0`.
//!
//! The protocol measures how often the implementation returns the expected
//! state and additionally computes Hamming-distance and effective-polarization
//! diagnostics.
//!
//! # Important scientific distinction
//!
//! This module implements **generic mirror-circuit benchmarking**.
//!
//! It must NOT be confused with Mirror Randomized Benchmarking (MRB).
//!
//! Generic mirror benchmarking asks:
//!
//! ```text
//! How faithfully does this particular U followed by U† experiment return
//! the expected state?
//! ```
//!
//! MRB additionally specifies a randomized layer distribution, randomized
//! Pauli dressing/twirling, depth-dependent ensembles, and a decay model used
//! to estimate an average layer infidelity.
//!
//! MRB is described in:
//!
//! Proctor et al., "Scalable Randomized Benchmarking of Quantum Computers
//! Using Mirror Circuits", Physical Review Letters 129, 150502 (2022).
//!
//! Therefore this file never reports a generic mirror return probability as
//! "average gate fidelity", "process fidelity", or "MRB error per layer".
//!
//! Such interpretations require additional protocol assumptions and belong
//! to dedicated protocol modules.
//!
//! # Architectural boundary
//!
//! This module owns:
//!
//! - mirror benchmark configuration;
//! - benchmark case identity;
//! - execution-observation validation;
//! - bitstring validation;
//! - return-probability calculation;
//! - Hamming-distance calculation;
//! - effective polarization calculation;
//! - binomial confidence intervals;
//! - depth aggregation;
//! - optional polarization decay fitting;
//! - protocol-level diagnostics;
//! - protocol-level result representation;
//! - protocol versioning;
//! - resource validation.
//!
//! This module does NOT own:
//!
//! - random circuit generation;
//! - logical mirror construction;
//! - Quantum IR;
//! - OpenQASM;
//! - Zamani-language parsing;
//! - routing;
//! - scheduling;
//! - hardware selection;
//! - backend execution;
//! - calibration;
//! - compiler optimization;
//! - universal benchmark result serialization;
//! - reporting;
//! - hardware-specific fidelity claims.
//!
//! The dependency direction is:
//!
//! ```text
//! generators::mirror_circuits
//!          │
//!          ▼
//! protocols::mirror
//!          │
//!          ▼
//! execution adapter
//!          │
//!          ▼
//! simulator / hardware
//!          │
//!          ▼
//! MirrorExecutionObservation
//!          │
//!          ▼
//! protocols::mirror::analyze
//!          │
//!          ├── return probability
//!          ├── Hamming distance
//!          ├── effective polarization
//!          └── optional decay fit
//!          │
//!          ▼
//! core::BenchmarkResult adapter
//! ```
//!
//! # Existing Zamani integration
//!
//! The existing:
//!
//! `generators/mirror_circuits.rs`
//!
//! provides:
//!
//! - `MirrorCircuit`;
//! - `MirrorLayer`;
//! - `MirrorOperation`;
//! - exact logical inversion;
//! - generator versioning;
//! - resource checks.
//!
//! This protocol consumes those types rather than reimplementing them.
//!
//! The generator intentionally does not depend on this protocol. That
//! dependency direction must remain unchanged.
//!
//! # Execution model
//!
//! The execution layer supplies normalized measurement counts:
//!
//! ```text
//! bitstring -> number of observations
//! ```
//!
//! For example:
//!
//! ```text
//! 0000 -> 920
//! 0001 -> 30
//! 0100 -> 25
//! 1000 -> 25
//! ```
//!
//! The total count must equal the declared shot count.
//!
//! The protocol never assumes a particular backend, simulator, QPU vendor,
//! measurement API, or execution transport.
//!
//! # Metrics
//!
//! ## Return probability
//!
//! ```text
//! P_return = N_expected / N_total
//! ```
//!
//! This is the direct probability of observing the expected output.
//!
//! ## Mean Hamming distance
//!
//! For an observed bitstring `x` and expected bitstring `s`:
//!
//! ```text
//! H(x, s) = number of differing bits
//! ```
//!
//! The benchmark reports the shot-weighted mean:
//!
//! ```text
//! mean_H = Σ count(x) H(x,s) / N_total
//! ```
//!
//! ## Effective polarization
//!
//! The protocol also reports:
//!
//! ```text
//! polarization = 1 - 2 * mean_H / n
//! ```
//!
//! This is a useful diagnostic for mirror-style randomized benchmarking, but
//! it is NOT automatically an entanglement fidelity.
//!
//! The scientific interpretation of effective polarization depends on the
//! circuit ensemble and noise assumptions.
//!
//! ## Decay fit
//!
//! When multiple benchmark depths are supplied, the protocol can fit:
//!
//! ```text
//! P(d) = A * p^d + B
//! ```
//!
//! or, for polarization:
//!
//! ```text
//! P_pol(d) = A * p^d + B
//! ```
//!
//! The fit is deliberately reported as a diagnostic model rather than a
//! universal physical law.
//!
//! # Statistical policy
//!
//! Return probability is a binomial proportion. The default interval is a
//! Wilson score interval.
//!
//! The protocol never uses the naive Wald interval.
//!
//! A confidence interval is not itself a pass/fail decision. The caller may
//! configure an acceptance threshold and choose whether the lower confidence
//! bound must exceed that threshold.
//!
//! # Production invariants
//!
//! This module guarantees:
//!
//! - no unsafe code;
//! - no global mutable state;
//! - no hidden random generation;
//! - no logging;
//! - no direct printing;
//! - no NaN/∞ acceptance;
//! - no negative measurements;
//! - no impossible shot counts;
//! - no malformed bitstrings;
//! - no mismatched circuit width;
//! - no integer overflow in aggregate counts;
//! - bounded benchmark cases;
//! - bounded observations;
//! - deterministic analysis;
//! - explicit confidence level;
//! - explicit statistical method;
//! - explicit fit model;
//! - explicit fit diagnostics;
//! - explicit scientific assumptions;
//! - explicit distinction between generic mirror benchmarking and MRB.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file is designed so that later integration with:
//!
//! - `core::BenchmarkResult`;
//! - `statistics::confidence`;
//! - `statistics::regression`;
//! - `metrics::fidelity`;
//! - `reporting::*`;
//! - `registry::*`;
//! - `stdlib::quantum`;
//! - `runtime::quantum`;
//!
//! can be implemented through adapters without changing the public protocol
//! semantics defined here.
//!
//! In particular, the execution backend should eventually adapt its normalized
//! result into [`MirrorExecutionObservation`].
//!
//! The protocol should remain independent of the execution implementation.

#![deny(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use super::super::generators::mirror_circuits::{
    MirrorCircuit,
    MIRROR_GENERATOR_ID,
    MIRROR_GENERATOR_VERSION,
};

// =============================================================================
// Public protocol identity
// =============================================================================

/// Stable benchmark identifier.
pub const MIRROR_BENCHMARK_ID: &str = "mirror_circuit";

/// Semantic protocol version.
///
/// Increment this when changing:
///
/// - observation semantics;
/// - statistical definitions;
/// - acceptance semantics;
/// - metric definitions;
/// - decay-fit semantics.
pub const MIRROR_BENCHMARK_PROTOCOL_VERSION: &str = "1.0.0";

/// Stable identifier for the generic mirror protocol.
pub const MIRROR_BENCHMARK_KIND: &str = "generic_mirror_circuit";

/// Stable identifier for the binomial interval method.
pub const MIRROR_BINOMIAL_INTERVAL_METHOD: &str = "wilson";

/// Stable identifier for the optional decay model.
pub const MIRROR_DECAY_MODEL: &str = "A*p^d+B";

/// Default confidence level.
///
/// This is a two-sided 95% interval.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Default maximum number of benchmark cases.
pub const DEFAULT_MAX_CASES: usize = 100_000;

/// Default maximum number of distinct output strings in one observation.
pub const DEFAULT_MAX_OUTCOMES: usize = 1_000_000;

/// Default maximum shot count per observation.
pub const DEFAULT_MAX_SHOTS: u64 = 10_000_000_000;

/// Minimum number of distinct depths required for a decay fit.
pub const MIN_DECAY_POINTS: usize = 3;

/// Maximum number of iterations used by the bounded decay optimizer.
const MAX_FIT_ITERATIONS: usize = 128;

/// Numerical tolerance for probability bounds.
const PROBABILITY_EPSILON: f64 = 1.0e-12;

/// Numerical tolerance for decay parameters.
const DECAY_EPSILON: f64 = 1.0e-12;

/// Maximum number of qubits accepted by the protocol.
///
/// The generator has its own safety limit. This protocol limit is deliberately
/// repeated here so that malformed external `MirrorCircuit` adapters cannot
/// bypass the protocol's resource boundary.
pub const DEFAULT_MAX_QUBITS: usize = 4096;

/// Maximum bitstring length accepted by this protocol.
pub const DEFAULT_MAX_BITSTRING_LENGTH: usize = 4096;

// =============================================================================
// Statistical interval
// =============================================================================

/// Binomial confidence interval.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BinomialConfidenceInterval {
    /// Lower confidence bound.
    pub lower: f64,

    /// Upper confidence bound.
    pub upper: f64,

    /// Confidence level.
    pub confidence_level: f64,

    /// Statistical method.
    pub method: &'static str,
}

// =============================================================================
// Benchmark configuration
// =============================================================================

/// Acceptance policy for mirror return probability.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AcceptancePolicy {
    /// Optional required minimum return probability.
    ///
    /// If `None`, the protocol reports the result without deciding pass/fail.
    pub minimum_return_probability: Option<f64>,

    /// If true, the lower confidence bound must be at least the threshold.
    ///
    /// If false, the point estimate must be at least the threshold.
    pub require_confidence_lower_bound: bool,
}

impl Default for AcceptancePolicy {
    fn default() -> Self {
        Self {
            minimum_return_probability: None,
            require_confidence_lower_bound: true,
        }
    }
}

impl AcceptancePolicy {
    /// Creates an acceptance policy requiring a point estimate threshold.
    pub fn point_estimate(threshold: f64) -> Result<Self, MirrorBenchmarkError> {
        validate_probability(threshold)?;

        Ok(Self {
            minimum_return_probability: Some(threshold),
            require_confidence_lower_bound: false,
        })
    }

    /// Creates an acceptance policy requiring the confidence lower bound.
    pub fn confidence_lower_bound(
        threshold: f64,
    ) -> Result<Self, MirrorBenchmarkError> {
        validate_probability(threshold)?;

        Ok(Self {
            minimum_return_probability: Some(threshold),
            require_confidence_lower_bound: true,
        })
    }

    fn validate(self) -> Result<Self, MirrorBenchmarkError> {
        if let Some(threshold) = self.minimum_return_probability {
            validate_probability(threshold)?;
        }

        Ok(self)
    }
}

/// Generic mirror benchmark configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirrorBenchmarkConfig {
    /// Confidence level used for return-probability intervals.
    pub confidence_level: f64,

    /// Acceptance policy.
    pub acceptance: AcceptancePolicy,

    /// Whether Hamming-distance diagnostics are calculated.
    pub calculate_hamming_metrics: bool,

    /// Whether effective polarization is calculated.
    pub calculate_polarization: bool,

    /// Whether a decay fit should be attempted when multiple depths exist.
    pub fit_decay: bool,

    /// Maximum number of cases.
    pub max_cases: usize,

    /// Maximum number of output strings retained in one observation.
    pub max_outcomes: usize,

    /// Maximum shots in one observation.
    pub max_shots: u64,
}

impl Default for MirrorBenchmarkConfig {
    fn default() -> Self {
        Self {
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            acceptance: AcceptancePolicy::default(),
            calculate_hamming_metrics: true,
            calculate_polarization: true,
            fit_decay: true,
            max_cases: DEFAULT_MAX_CASES,
            max_outcomes: DEFAULT_MAX_OUTCOMES,
            max_shots: DEFAULT_MAX_SHOTS,
        }
    }
}

impl MirrorBenchmarkConfig {
    /// Validates the benchmark configuration.
    pub fn validate(&self) -> Result<(), MirrorBenchmarkError> {
        validate_confidence_level(self.confidence_level)?;
        self.acceptance.validate()?;

        if self.max_cases == 0 {
            return Err(MirrorBenchmarkError::InvalidConfiguration {
                field: "max_cases",
                reason: "must be greater than zero".to_owned(),
            });
        }

        if self.max_outcomes == 0 {
            return Err(MirrorBenchmarkError::InvalidConfiguration {
                field: "max_outcomes",
                reason: "must be greater than zero".to_owned(),
            });
        }

        if self.max_shots == 0 {
            return Err(MirrorBenchmarkError::InvalidConfiguration {
                field: "max_shots",
                reason: "must be greater than zero".to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Benchmark case
// =============================================================================

/// One executable mirror benchmark case.
///
/// A case owns the generated logical circuit and its expected output.
///
/// The circuit is generated elsewhere by `generators::mirror_circuits`.
/// This protocol never regenerates or mutates it.
#[derive(Debug, Clone)]
pub struct MirrorBenchmarkCase {
    /// Stable caller-provided case identifier.
    pub case_id: String,

    /// Generated logical mirror circuit.
    pub circuit: MirrorCircuit,

    /// Expected output bitstring.
    ///
    /// For a standard `|0...0>` mirror experiment this is a string containing
    /// only zeroes.
    pub expected_output: String,

    /// Logical benchmark depth used for cross-case analysis.
    ///
    /// This is normally the forward depth of the mirror circuit, not the total
    /// U + U† depth.
    pub benchmark_depth: usize,
}

impl MirrorBenchmarkCase {
    /// Creates a case using the all-zero output expected for a standard
    /// computational-basis mirror experiment.
    pub fn new(
        case_id: impl Into<String>,
        circuit: MirrorCircuit,
    ) -> Result<Self, MirrorBenchmarkError> {
        let benchmark_depth = circuit.forward_depth();

        let expected_output = "0".repeat(circuit.qubit_count());

        Self::with_expected_output(
            case_id,
            circuit,
            expected_output,
        )
    }

    /// Creates a case with an explicit expected output bitstring.
    pub fn with_expected_output(
        case_id: impl Into<String>,
        circuit: MirrorCircuit,
        expected_output: impl Into<String>,
    ) -> Result<Self, MirrorBenchmarkError> {
        let case_id = case_id.into();
        let expected_output = expected_output.into();

        validate_case_id(&case_id)?;
        validate_circuit(&circuit)?;
        validate_bitstring(
            &expected_output,
            circuit.qubit_count(),
        )?;

        let benchmark_depth = circuit.forward_depth();

        Ok(Self {
            case_id,
            circuit,
            expected_output,
            benchmark_depth,
        })
    }

    /// Returns the logical qubit count.
    pub fn qubit_count(&self) -> usize {
        self.circuit.qubit_count()
    }

    /// Returns the complete physical/logical mirror depth before compilation.
    pub fn total_logical_depth(&self) -> usize {
        self.circuit.total_depth()
    }

    /// Returns the number of forward logical operations.
    pub fn forward_operation_count(&self) -> usize {
        self.circuit.forward_operation_count()
    }

    /// Returns the number of CX operations in the forward circuit.
    pub fn forward_cx_count(&self) -> usize {
        self.circuit.forward_cx_count()
    }

    /// Returns the generator version.
    pub fn generator_version(&self) -> u16 {
        self.circuit.generator_version()
    }

    /// Returns the stable generator identifier.
    pub fn generator_id(&self) -> &'static str {
        self.circuit.generator_id()
    }
}

// =============================================================================
// Execution observation
// =============================================================================

/// Normalized result supplied by the execution layer.
///
/// The execution layer converts backend-specific results into this structure.
/// It may therefore represent:
///
/// - a simulator;
/// - a QPU;
/// - an emulator;
/// - a remote provider;
/// - a local runtime.
///
/// The protocol does not know which.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirrorExecutionObservation {
    /// Case identifier corresponding to `MirrorBenchmarkCase::case_id`.
    pub case_id: String,

    /// Number of declared shots.
    pub shots: u64,

    /// Measurement counts keyed by canonical bitstrings.
    pub counts: BTreeMap<String, u64>,

    /// Optional backend execution time in nanoseconds.
    ///
    /// This is metadata only. It is not used to calculate circuit fidelity.
    pub execution_time_ns: Option<u64>,
}

impl MirrorExecutionObservation {
    /// Constructs an observation.
    pub fn new(
        case_id: impl Into<String>,
        shots: u64,
        counts: BTreeMap<String, u64>,
    ) -> Self {
        Self {
            case_id: case_id.into(),
            shots,
            counts,
            execution_time_ns: None,
        }
    }

    /// Adds execution timing metadata.
    pub fn with_execution_time_ns(
        mut self,
        execution_time_ns: u64,
    ) -> Self {
        self.execution_time_ns = Some(execution_time_ns);
        self
    }
}

// =============================================================================
// Per-case result
// =============================================================================

/// Complete analysis result for one mirror circuit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirrorCaseResult {
    /// Case identifier.
    pub case_id: String,

    /// Logical qubit count.
    pub qubits: usize,

    /// Forward benchmark depth.
    pub benchmark_depth: usize,

    /// Complete U + U† logical depth.
    pub total_logical_depth: usize,

    /// Forward operation count.
    pub forward_operation_count: usize,

    /// Forward CX count.
    pub forward_cx_count: usize,

    /// Declared shots.
    pub shots: u64,

    /// Number of expected-output observations.
    pub successful_shots: u64,

    /// Return probability.
    pub return_probability: f64,

    /// Wilson confidence interval around return probability.
    pub return_probability_interval: BinomialConfidenceInterval,

    /// Mean Hamming distance from the expected output.
    pub mean_hamming_distance: Option<f64>,

    /// Mean Hamming distance normalized by qubit count.
    pub normalized_hamming_distance: Option<f64>,

    /// Effective polarization.
    ///
    /// This is a diagnostic metric and must not automatically be interpreted
    /// as entanglement fidelity.
    pub effective_polarization: Option<f64>,

    /// Whether the configured acceptance policy passed.
    ///
    /// `None` means no threshold was configured.
    pub passed: Option<bool>,

    /// Optional execution time supplied by the backend adapter.
    pub execution_time_ns: Option<u64>,
}

// =============================================================================
// Decay fit
// =============================================================================

/// Quality diagnostics for an exponential decay fit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecayFitDiagnostics {
    /// Number of distinct depth points.
    pub points: usize,

    /// Sum of squared residuals.
    pub sum_squared_residuals: f64,

    /// Root mean squared error.
    pub root_mean_squared_error: f64,

    /// Coefficient of determination where defined.
    pub r_squared: Option<f64>,

    /// Whether the fit converged under the bounded optimizer.
    pub converged: bool,

    /// Whether the fit is physically constrained to `0 <= p <= 1`.
    pub constrained: bool,
}

/// Exponential decay fit.
///
/// Model:
///
/// ```text
/// y(d) = A * p^d + B
/// ```
///
/// `p` is the fitted per-depth decay parameter. It is deliberately not named
/// "fidelity" because the interpretation depends on the benchmark ensemble.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirrorDecayFit {
    /// Model identifier.
    pub model: &'static str,

    /// Amplitude.
    pub amplitude: f64,

    /// Decay parameter.
    pub decay: f64,

    /// Offset.
    pub offset: f64,

    /// Diagnostic fit information.
    pub diagnostics: DecayFitDiagnostics,
}

// =============================================================================
// Aggregate result
// =============================================================================

/// Complete mirror benchmark result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirrorBenchmarkResult {
    /// Benchmark identifier.
    pub benchmark_id: &'static str,

    /// Protocol version.
    pub protocol_version: &'static str,

    /// Protocol kind.
    pub benchmark_kind: &'static str,

    /// Number of analyzed cases.
    pub case_count: usize,

    /// Total shots across all cases.
    pub total_shots: u64,

    /// Total successful expected-output observations.
    pub total_successful_shots: u64,

    /// Pooled return probability.
    ///
    /// This is useful as a descriptive aggregate but should not be confused
    /// with a weighted estimate under a different experimental design.
    pub pooled_return_probability: f64,

    /// Pooled return-probability confidence interval.
    pub pooled_return_probability_interval: BinomialConfidenceInterval,

    /// Per-case results.
    pub cases: Vec<MirrorCaseResult>,

    /// Optional decay fit over benchmark depths.
    pub decay_fit: Option<MirrorDecayFit>,

    /// Generator identifier represented by the benchmark cases.
    pub generator_id: &'static str,

    /// Generator semantic version represented by the benchmark cases.
    pub generator_version: u16,

    /// Statistical assumptions.
    pub assumptions: Vec<&'static str>,
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the mirror benchmark protocol.
#[derive(Debug, Clone, PartialEq)]
pub enum MirrorBenchmarkError {
    /// Invalid configuration.
    InvalidConfiguration {
        field: &'static str,
        reason: String,
    },

    /// Case identifier is invalid.
    InvalidCaseId {
        case_id: String,
    },

    /// Number of cases exceeded the configured limit.
    CaseLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Circuit has invalid width or structure.
    InvalidCircuit {
        reason: String,
    },

    /// Bitstring has invalid syntax or width.
    InvalidBitstring {
        bitstring: String,
        expected_width: usize,
    },

    /// Observation has zero shots.
    ZeroShots {
        case_id: String,
    },

    /// Observation has too many output strings.
    OutcomeLimitExceeded {
        case_id: String,
        requested: usize,
        maximum: usize,
    },

    /// Sum of counts does not equal declared shots.
    InconsistentShotCount {
        case_id: String,
        declared: u64,
        observed: u64,
    },

    /// An observed count overflowed an aggregate.
    CountOverflow {
        case_id: String,
    },

    /// Duplicate case observation.
    DuplicateObservation {
        case_id: String,
    },

    /// Observation is missing for a configured case.
    MissingObservation {
        case_id: String,
    },

    /// Confidence level is invalid.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// Probability is invalid.
    InvalidProbability {
        value: f64,
    },

    /// Statistical calculation became non-finite.
    NonFiniteStatistic {
        statistic: &'static str,
    },

    /// Decay fit cannot be performed.
    InsufficientDecayData {
        points: usize,
    },

    /// Decay fit failed.
    DecayFitFailure {
        reason: String,
    },
}

impl fmt::Display for MirrorBenchmarkError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidConfiguration {
                field,
                reason,
            } => {
                write!(
                    f,
                    "invalid mirror benchmark configuration '{field}': {reason}"
                )
            }

            Self::InvalidCaseId { case_id } => {
                write!(
                    f,
                    "invalid mirror benchmark case identifier: {case_id:?}"
                )
            }

            Self::CaseLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "mirror benchmark case limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::InvalidCircuit { reason } => {
                write!(
                    f,
                    "invalid mirror benchmark circuit: {reason}"
                )
            }

            Self::InvalidBitstring {
                bitstring,
                expected_width,
            } => {
                write!(
                    f,
                    "invalid measurement bitstring {bitstring:?}; expected {expected_width} bits"
                )
            }

            Self::ZeroShots { case_id } => {
                write!(
                    f,
                    "mirror benchmark case {case_id:?} contains zero shots"
                )
            }

            Self::OutcomeLimitExceeded {
                case_id,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "mirror benchmark case {case_id:?} contains {requested} \
                     outcomes; maximum is {maximum}"
                )
            }

            Self::InconsistentShotCount {
                case_id,
                declared,
                observed,
            } => {
                write!(
                    f,
                    "mirror benchmark case {case_id:?} declares {declared} shots \
                     but contains {observed} observations"
                )
            }

            Self::CountOverflow { case_id } => {
                write!(
                    f,
                    "measurement-count aggregation overflowed for case {case_id:?}"
                )
            }

            Self::DuplicateObservation { case_id } => {
                write!(
                    f,
                    "duplicate mirror benchmark observation for case {case_id:?}"
                )
            }

            Self::MissingObservation { case_id } => {
                write!(
                    f,
                    "missing mirror benchmark observation for case {case_id:?}"
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    f,
                    "invalid confidence level {value}; expected 0 < level < 1"
                )
            }

            Self::InvalidProbability { value } => {
                write!(
                    f,
                    "invalid probability {value}; expected finite value in [0, 1]"
                )
            }

            Self::NonFiniteStatistic { statistic } => {
                write!(
                    f,
                    "mirror benchmark statistic '{statistic}' is non-finite"
                )
            }

            Self::InsufficientDecayData { points } => {
                write!(
                    f,
                    "at least {MIN_DECAY_POINTS} distinct depth points are required \
                     for decay fitting; received {points}"
                )
            }

            Self::DecayFitFailure { reason } => {
                write!(
                    f,
                    "mirror decay fit failed: {reason}"
                )
            }
        }
    }
}

impl Error for MirrorBenchmarkError {}

// =============================================================================
// Public protocol
// =============================================================================

/// Stateless mirror benchmark protocol.
///
/// The protocol is intentionally zero-sized. All experiment state is supplied
/// through method arguments and returned through explicit result values.
#[derive(Debug, Clone, Copy, Default)]
pub struct MirrorBenchmarkProtocol;

impl MirrorBenchmarkProtocol {
    /// Creates a protocol instance.
    pub const fn new() -> Self {
        Self
    }

    /// Returns the stable benchmark identifier.
    pub const fn benchmark_id(&self) -> &'static str {
        MIRROR_BENCHMARK_ID
    }

    /// Returns the semantic protocol version.
    pub const fn protocol_version(&self) -> &'static str {
        MIRROR_BENCHMARK_PROTOCOL_VERSION
    }

    /// Validates a benchmark plan before execution.
    pub fn validate_cases(
        &self,
        config: &MirrorBenchmarkConfig,
        cases: &[MirrorBenchmarkCase],
    ) -> Result<(), MirrorBenchmarkError> {
        config.validate()?;

        if cases.is_empty() {
            return Err(MirrorBenchmarkError::InvalidConfiguration {
                field: "cases",
                reason: "at least one benchmark case is required".to_owned(),
            });
        }

        if cases.len() > config.max_cases {
            return Err(MirrorBenchmarkError::CaseLimitExceeded {
                requested: cases.len(),
                maximum: config.max_cases,
            });
        }

        let mut previous_depth: Option<usize> = None;

        for case in cases {
            validate_case(case)?;

            if let Some(previous) = previous_depth {
                if case.benchmark_depth < previous {
                    return Err(MirrorBenchmarkError::InvalidConfiguration {
                        field: "cases",
                        reason:
                            "benchmark cases must be ordered by non-decreasing \
                             benchmark depth"
                                .to_owned(),
                    });
                }
            }

            previous_depth = Some(case.benchmark_depth);
        }

        Ok(())
    }

    /// Validates normalized execution observations against benchmark cases.
    pub fn validate_observations(
        &self,
        config: &MirrorBenchmarkConfig,
        cases: &[MirrorBenchmarkCase],
        observations: &[MirrorExecutionObservation],
    ) -> Result<(), MirrorBenchmarkError> {
        self.validate_cases(config, cases)?;

        if observations.len() != cases.len() {
            return Err(MirrorBenchmarkError::InvalidConfiguration {
                field: "observations",
                reason: format!(
                    "expected {} observations, received {}",
                    cases.len(),
                    observations.len()
                ),
            });
        }

        let case_map = cases
            .iter()
            .map(|case| (case.case_id.as_str(), case))
            .collect::<BTreeMap<_, _>>();

        let mut seen = BTreeMap::<&str, ()>::new();

        for observation in observations {
            if seen
                .insert(observation.case_id.as_str(), ())
                .is_some()
            {
                return Err(
                    MirrorBenchmarkError::DuplicateObservation {
                        case_id: observation.case_id.clone(),
                    },
                );
            }

            let case = case_map
                .get(observation.case_id.as_str())
                .ok_or_else(|| {
                    MirrorBenchmarkError::MissingObservation {
                        case_id: observation.case_id.clone(),
                    }
                })?;

            validate_observation(
                config,
                case,
                observation,
            )?;
        }

        Ok(())
    }

    /// Analyzes a complete mirror benchmark.
    ///
    /// The execution layer must have already completed all backend work.
    pub fn analyze(
        &self,
        config: &MirrorBenchmarkConfig,
        cases: &[MirrorBenchmarkCase],
        observations: &[MirrorExecutionObservation],
    ) -> Result<MirrorBenchmarkResult, MirrorBenchmarkError> {
        self.validate_observations(
            config,
            cases,
            observations,
        )?;

        let observation_map = observations
            .iter()
            .map(|observation| {
                (observation.case_id.as_str(), observation)
            })
            .collect::<BTreeMap<_, _>>();

        let mut case_results = Vec::with_capacity(cases.len());

        let mut total_shots = 0u64;
        let mut total_successful_shots = 0u64;

        for case in cases {
            let observation = observation_map
                .get(case.case_id.as_str())
                .ok_or_else(|| {
                    MirrorBenchmarkError::MissingObservation {
                        case_id: case.case_id.clone(),
                    }
                })?;

            let result = analyze_case(
                config,
                case,
                observation,
            )?;

            total_shots = total_shots
                .checked_add(result.shots)
                .ok_or_else(|| {
                    MirrorBenchmarkError::CountOverflow {
                        case_id: case.case_id.clone(),
                    }
                })?;

            total_successful_shots =
                total_successful_shots
                    .checked_add(result.successful_shots)
                    .ok_or_else(|| {
                        MirrorBenchmarkError::CountOverflow {
                            case_id: case.case_id.clone(),
                        }
                    })?;

            case_results.push(result);
        }

        if total_shots == 0 {
            return Err(MirrorBenchmarkError::NonFiniteStatistic {
                statistic: "total_shots",
            });
        }

        let pooled_return_probability =
            total_successful_shots as f64 / total_shots as f64;

        validate_probability(
            pooled_return_probability,
        )?;

        let pooled_interval = wilson_interval(
            total_successful_shots,
            total_shots,
            config.confidence_level,
        )?;

        let decay_fit = if config.fit_decay {
            fit_depth_decay(&case_results).ok()
        } else {
            None
        };

        Ok(MirrorBenchmarkResult {
            benchmark_id: MIRROR_BENCHMARK_ID,
            protocol_version: MIRROR_BENCHMARK_PROTOCOL_VERSION,
            benchmark_kind: MIRROR_BENCHMARK_KIND,
            case_count: case_results.len(),
            total_shots,
            total_successful_shots,
            pooled_return_probability,
            pooled_return_probability_interval: pooled_interval,
            cases: case_results,
            decay_fit,
            generator_id: MIRROR_GENERATOR_ID,
            generator_version: MIRROR_GENERATOR_VERSION,
            assumptions: vec![
                "The supplied expected output is the ideal logical output \
                 for the executed mirror circuit.",
                "Measurement counts are normalized to canonical bitstrings \
                 by the execution adapter.",
                "Return probability is a binomial proportion.",
                "Wilson intervals describe sampling uncertainty only.",
                "Effective polarization requires the chosen mirror ensemble \
                 to make that diagnostic scientifically meaningful.",
                "An exponential depth fit is a diagnostic model and is not \
                 automatically an average gate fidelity.",
                "Compiler, routing, scheduling, calibration, and hardware \
                 effects remain part of the measured end-to-end circuit.",
            ],
        })
    }

    /// Analyzes one benchmark case independently.
    ///
    /// This is useful for streaming execution and for independently
    /// re-analyzing archived observations.
    pub fn analyze_case(
        &self,
        config: &MirrorBenchmarkConfig,
        case: &MirrorBenchmarkCase,
        observation: &MirrorExecutionObservation,
    ) -> Result<MirrorCaseResult, MirrorBenchmarkError> {
        validate_observation(
            config,
            case,
            observation,
        )?;

        analyze_case(config, case, observation)
    }
}

// =============================================================================
// Validation
// =============================================================================

fn validate_case_id(
    case_id: &str,
) -> Result<(), MirrorBenchmarkError> {
    if case_id.is_empty()
        || case_id.len() > 256
        || case_id.chars().any(|character| {
            character.is_control()
        })
    {
        return Err(MirrorBenchmarkError::InvalidCaseId {
            case_id: case_id.to_owned(),
        });
    }

    Ok(())
}

fn validate_case(
    case: &MirrorBenchmarkCase,
) -> Result<(), MirrorBenchmarkError> {
    validate_case_id(&case.case_id)?;
    validate_circuit(&case.circuit)?;

    validate_bitstring(
        &case.expected_output,
        case.circuit.qubit_count(),
    )?;

    if case.benchmark_depth != case.circuit.forward_depth() {
        return Err(MirrorBenchmarkError::InvalidConfiguration {
            field: "benchmark_depth",
            reason: format!(
                "case {:?} declares depth {} but circuit forward depth is {}",
                case.case_id,
                case.benchmark_depth,
                case.circuit.forward_depth()
            ),
        });
    }

    Ok(())
}

fn validate_circuit(
    circuit: &MirrorCircuit,
) -> Result<(), MirrorBenchmarkError> {
    if circuit.qubit_count() == 0 {
        return Err(MirrorBenchmarkError::InvalidCircuit {
            reason: "circuit must contain at least one qubit".to_owned(),
        });
    }

    if circuit.qubit_count() > DEFAULT_MAX_QUBITS {
        return Err(MirrorBenchmarkError::InvalidCircuit {
            reason: format!(
                "circuit uses {} qubits; protocol maximum is {}",
                circuit.qubit_count(),
                DEFAULT_MAX_QUBITS
            ),
        });
    }

    if circuit.forward_depth() == 0 {
        return Err(MirrorBenchmarkError::InvalidCircuit {
            reason:
                "mirror benchmark requires at least one forward layer"
                    .to_owned(),
        });
    }

    if circuit.total_depth() != circuit.forward_depth() * 2 {
        return Err(MirrorBenchmarkError::InvalidCircuit {
            reason:
                "mirror circuit total depth is inconsistent with its forward depth"
                    .to_owned(),
        });
    }

    if circuit.forward_operation_count()
        != circuit.inverse_operation_count()
    {
        return Err(MirrorBenchmarkError::InvalidCircuit {
            reason:
                "forward and inverse operation counts differ".to_owned(),
        });
    }

    if circuit.total_operation_count()
        != circuit.forward_operation_count() * 2
    {
        return Err(MirrorBenchmarkError::InvalidCircuit {
            reason:
                "total operation count is inconsistent".to_owned(),
        });
    }

    circuit
        .validate_exact_inverse()
        .map_err(|error| {
            MirrorBenchmarkError::InvalidCircuit {
                reason: error.to_string(),
            }
        })?;

    Ok(())
}

fn validate_observation(
    config: &MirrorBenchmarkConfig,
    case: &MirrorBenchmarkCase,
    observation: &MirrorExecutionObservation,
) -> Result<(), MirrorBenchmarkError> {
    if observation.case_id != case.case_id {
        return Err(MirrorBenchmarkError::InvalidConfiguration {
            field: "observation.case_id",
            reason: format!(
                "expected {:?}, received {:?}",
                case.case_id,
                observation.case_id
            ),
        });
    }

    if observation.shots == 0 {
        return Err(MirrorBenchmarkError::ZeroShots {
            case_id: case.case_id.clone(),
        });
    }

    if observation.shots > config.max_shots {
        return Err(MirrorBenchmarkError::InvalidConfiguration {
            field: "observation.shots",
            reason: format!(
                "received {} shots; maximum is {}",
                observation.shots,
                config.max_shots
            ),
        });
    }

    if observation.counts.len() > config.max_outcomes {
        return Err(MirrorBenchmarkError::OutcomeLimitExceeded {
            case_id: case.case_id.clone(),
            requested: observation.counts.len(),
            maximum: config.max_outcomes,
        });
    }

    let mut observed_shots = 0u64;

    for (bitstring, count) in &observation.counts {
        validate_bitstring(
            bitstring,
            case.circuit.qubit_count(),
        )?;

        observed_shots = observed_shots
            .checked_add(*count)
            .ok_or_else(|| {
                MirrorBenchmarkError::CountOverflow {
                    case_id: case.case_id.clone(),
                }
            })?;
    }

    if observed_shots != observation.shots {
        return Err(
            MirrorBenchmarkError::InconsistentShotCount {
                case_id: case.case_id.clone(),
                declared: observation.shots,
                observed: observed_shots,
            },
        );
    }

    Ok(())
}

fn validate_bitstring(
    bitstring: &str,
    expected_width: usize,
) -> Result<(), MirrorBenchmarkError> {
    if expected_width > DEFAULT_MAX_BITSTRING_LENGTH
        || bitstring.len() != expected_width
        || bitstring
            .bytes()
            .any(|byte| byte != b'0' && byte != b'1')
    {
        return Err(MirrorBenchmarkError::InvalidBitstring {
            bitstring: bitstring.to_owned(),
            expected_width,
        });
    }

    Ok(())
}

fn validate_confidence_level(
    level: f64,
) -> Result<(), MirrorBenchmarkError> {
    if !level.is_finite()
        || level <= 0.0
        || level >= 1.0
    {
        return Err(
            MirrorBenchmarkError::InvalidConfidenceLevel {
                value: level,
            },
        );
    }

    Ok(())
}

fn validate_probability(
    probability: f64,
) -> Result<(), MirrorBenchmarkError> {
    if !probability.is_finite()
        || probability < -PROBABILITY_EPSILON
        || probability > 1.0 + PROBABILITY_EPSILON
    {
        return Err(MirrorBenchmarkError::InvalidProbability {
            value: probability,
        });
    }

    Ok(())
}

// =============================================================================
// Case analysis
// =============================================================================

fn analyze_case(
    config: &MirrorBenchmarkConfig,
    case: &MirrorBenchmarkCase,
    observation: &MirrorExecutionObservation,
) -> Result<MirrorCaseResult, MirrorBenchmarkError> {
    let successful_shots = observation
        .counts
        .get(&case.expected_output)
        .copied()
        .unwrap_or(0);

    let return_probability =
        successful_shots as f64 / observation.shots as f64;

    validate_probability(return_probability)?;

    let return_probability_interval = wilson_interval(
        successful_shots,
        observation.shots,
        config.confidence_level,
    )?;

    let hamming_metrics =
        if config.calculate_hamming_metrics {
            Some(calculate_hamming_metrics(
                &case.expected_output,
                &observation.counts,
            )?)
        } else {
            None
        };

    let mean_hamming_distance =
        hamming_metrics.map(|metrics| metrics.mean_distance);

    let normalized_hamming_distance =
        hamming_metrics.map(|metrics| metrics.normalized_distance);

    let effective_polarization =
        if config.calculate_polarization {
            hamming_metrics.map(|metrics| metrics.polarization)
        } else {
            None
        };

    let passed =
        config
            .acceptance
            .minimum_return_probability
            .map(|threshold| {
                if config.acceptance.require_confidence_lower_bound {
                    return_probability_interval.lower
                        >= threshold
                } else {
                    return_probability >= threshold
                }
            });

    Ok(MirrorCaseResult {
        case_id: case.case_id.clone(),
        qubits: case.qubit_count(),
        benchmark_depth: case.benchmark_depth,
        total_logical_depth: case.total_logical_depth(),
        forward_operation_count: case.forward_operation_count(),
        forward_cx_count: case.forward_cx_count(),
        shots: observation.shots,
        successful_shots,
        return_probability,
        return_probability_interval,
        mean_hamming_distance,
        normalized_hamming_distance,
        effective_polarization,
        passed,
        execution_time_ns: observation.execution_time_ns,
    })
}

#[derive(Debug, Clone, Copy)]
struct HammingMetrics {
    mean_distance: f64,
    normalized_distance: f64,
    polarization: f64,
}

fn calculate_hamming_metrics(
    expected: &str,
    counts: &BTreeMap<String, u64>,
) -> Result<HammingMetrics, MirrorBenchmarkError> {
    let width = expected.len();

    if width == 0 {
        return Err(MirrorBenchmarkError::InvalidCircuit {
            reason:
                "cannot calculate Hamming metrics for a zero-width output"
                    .to_owned(),
        });
    }

    let mut total_shots = 0u64;
    let mut weighted_distance = 0u128;

    for (observed, count) in counts {
        let distance = hamming_distance(
            expected,
            observed,
        )?;

        total_shots = total_shots
            .checked_add(*count)
            .ok_or_else(|| {
                MirrorBenchmarkError::CountOverflow {
                    case_id: "<hamming-analysis>".to_owned(),
                }
            })?;

        weighted_distance = weighted_distance
            .checked_add(
                (*count as u128)
                    .checked_mul(distance as u128)
                    .ok_or_else(|| {
                        MirrorBenchmarkError::CountOverflow {
                            case_id:
                                "<hamming-analysis>".to_owned(),
                        }
                    })?,
            )
            .ok_or_else(|| {
                MirrorBenchmarkError::CountOverflow {
                    case_id: "<hamming-analysis>".to_owned(),
                }
            })?;
    }

    if total_shots == 0 {
        return Err(MirrorBenchmarkError::NonFiniteStatistic {
            statistic: "hamming_total_shots",
        });
    }

    let mean_distance =
        weighted_distance as f64 / total_shots as f64;

    let normalized_distance =
        mean_distance / width as f64;

    let polarization =
        1.0 - 2.0 * normalized_distance;

    if !mean_distance.is_finite()
        || !normalized_distance.is_finite()
        || !polarization.is_finite()
    {
        return Err(MirrorBenchmarkError::NonFiniteStatistic {
            statistic: "hamming_metrics",
        });
    }

    Ok(HammingMetrics {
        mean_distance,
        normalized_distance,
        polarization,
    })
}

fn hamming_distance(
    expected: &str,
    observed: &str,
) -> Result<usize, MirrorBenchmarkError> {
    if expected.len() != observed.len() {
        return Err(MirrorBenchmarkError::InvalidBitstring {
            bitstring: observed.to_owned(),
            expected_width: expected.len(),
        });
    }

    Ok(expected
        .bytes()
        .zip(observed.bytes())
        .filter(|(left, right)| left != right)
        .count())
}

// =============================================================================
// Wilson interval
// =============================================================================

fn wilson_interval(
    successes: u64,
    samples: u64,
    confidence_level: f64,
) -> Result<BinomialConfidenceInterval, MirrorBenchmarkError> {
    if samples == 0 || successes > samples {
        return Err(MirrorBenchmarkError::InvalidConfiguration {
            field: "binomial_counts",
            reason: format!(
                "successes={successes} and samples={samples} are inconsistent"
            ),
        });
    }

    validate_confidence_level(confidence_level)?;

    let z = normal_quantile(
        0.5 + confidence_level / 2.0,
    )?;

    let n = samples as f64;
    let p = successes as f64 / n;
    let z2 = z * z;

    let denominator =
        1.0 + z2 / n;

    let center =
        (p + z2 / (2.0 * n)) / denominator;

    let half_width =
        z
            * ((p * (1.0 - p) / n
                + z2 / (4.0 * n * n))
                .sqrt())
            / denominator;

    let lower = clamp_unit_interval(
        center - half_width,
    );

    let upper = clamp_unit_interval(
        center + half_width,
    );

    if !lower.is_finite()
        || !upper.is_finite()
    {
        return Err(MirrorBenchmarkError::NonFiniteStatistic {
            statistic: "wilson_interval",
        });
    }

    Ok(BinomialConfidenceInterval {
        lower,
        upper,
        confidence_level,
        method: MIRROR_BINOMIAL_INTERVAL_METHOD,
    })
}

// =============================================================================
// Normal quantile
// =============================================================================

/// Inverse standard-normal CDF.
///
/// This implementation uses the Acklam rational approximation. It is
/// deterministic, dependency-free, and sufficiently accurate for benchmark
/// confidence intervals.
fn normal_quantile(
    probability: f64,
) -> Result<f64, MirrorBenchmarkError> {
    if !probability.is_finite()
        || probability <= 0.0
        || probability >= 1.0
    {
        return Err(MirrorBenchmarkError::InvalidProbability {
            value: probability,
        });
    }

    // Coefficients from Peter J. Acklam's inverse-normal approximation.
    const A1: f64 = -3.969683028665376e1;
    const A2: f64 = 2.209460984245205e2;
    const A3: f64 = -2.759285104469687e2;
    const A4: f64 = 1.383577518672690e2;
    const A5: f64 = -3.066479806614716e1;
    const A6: f64 = 2.506628277459239e0;

    const B1: f64 = -5.447609879822406e1;
    const B2: f64 = 1.615858368580409e2;
    const B3: f64 = -1.556989798598866e2;
    const B4: f64 = 6.680131188771972e1;
    const B5: f64 = -1.328068155288572e1;

    const C1: f64 = -7.784894002430293e-3;
    const C2: f64 = -3.223964580411365e-1;
    const C3: f64 = -2.400758277161838e0;
    const C4: f64 = -2.549732539343734e0;
    const C5: f64 = 4.374664141464968e0;
    const C6: f64 = 2.938163982698783e0;

    const D1: f64 = 7.784695709041462e-3;
    const D2: f64 = 3.224671290700398e-1;
    const D3: f64 = 2.445134137142996e0;
    const D4: f64 = 3.754408661907416e0;

    const P_LOW: f64 = 0.02425;
    const P_HIGH: f64 = 1.0 - P_LOW;

    let result = if probability < P_LOW {
        let q = (-2.0 * probability.ln()).sqrt();

        (((((C1 * q + C2) * q + C3) * q + C4) * q + C5) * q
            + C6)
            / ((((D1 * q + D2) * q + D3) * q + D4) * q + 1.0)
    } else if probability <= P_HIGH {
        let q = probability - 0.5;
        let r = q * q;

        (((((A1 * r + A2) * r + A3) * r + A4) * r + A5) * r
            + A6)
            * q
            / (((((B1 * r + B2) * r + B3) * r + B4) * r + B5) * r
                + 1.0)
    } else {
        let q = (-2.0 * (1.0 - probability).ln()).sqrt();

        -(((((C1 * q + C2) * q + C3) * q + C4) * q + C5) * q
            + C6)
            / ((((D1 * q + D2) * q + D3) * q + D4) * q + 1.0)
    };

    if !result.is_finite() {
        return Err(MirrorBenchmarkError::NonFiniteStatistic {
            statistic: "normal_quantile",
        });
    }

    Ok(result)
}

// =============================================================================
// Depth decay fitting
// =============================================================================

/// Fits an exponential model to effective polarization.
///
/// The fit is intentionally simple and deterministic:
///
/// ```text
/// y(d) = A * p^d + B
/// ```
///
/// For each candidate `p`, the optimal `A` and `B` are solved by linear least
/// squares. The bounded one-dimensional search then minimizes residual error.
///
/// This avoids introducing a heavyweight numerical optimizer merely for a
/// protocol diagnostic.
fn fit_depth_decay(
    cases: &[MirrorCaseResult],
) -> Result<MirrorDecayFit, MirrorBenchmarkError> {
    let mut points = cases
        .iter()
        .filter_map(|case| {
            case.effective_polarization
                .map(|polarization| {
                    (
                        case.benchmark_depth as f64,
                        polarization,
                    )
                })
        })
        .collect::<Vec<_>>();

    points.sort_by(|left, right| {
        left.0
            .partial_cmp(&right.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    points.dedup_by(|left, right| {
        (left.0 - right.0).abs() <= DECAY_EPSILON
    });

    if points.len() < MIN_DECAY_POINTS {
        return Err(
            MirrorBenchmarkError::InsufficientDecayData {
                points: points.len(),
            },
        );
    }

    for (_, y) in &points {
        if !y.is_finite() {
            return Err(
                MirrorBenchmarkError::NonFiniteStatistic {
                    statistic: "effective_polarization",
                },
            );
        }
    }

    let mut best = None::<(f64, f64, f64, f64)>;

    // Coarse-to-fine bounded search over p in [0, 1].
    //
    // We intentionally keep the optimizer deterministic and bounded.
    let mut lower = 0.0;
    let mut upper = 1.0;

    for iteration in 0..MAX_FIT_ITERATIONS {
        let span = upper - lower;

        if span <= 1.0e-10 {
            break;
        }

        let p1 = lower + span / 3.0;
        let p2 = upper - span / 3.0;

        let candidate1 =
            fit_linear_amplitude_offset(&points, p1)?;

        let candidate2 =
            fit_linear_amplitude_offset(&points, p2)?;

        if candidate1.0 <= candidate2.0 {
            upper = p2;
            best = Some((
                candidate1.0,
                p1,
                candidate1.1,
                candidate1.2,
            ));
        } else {
            lower = p1;
            best = Some((
                candidate2.0,
                p2,
                candidate2.1,
                candidate2.2,
            ));
        }

        if iteration + 1 == MAX_FIT_ITERATIONS {
            break;
        }
    }

    let (
        sum_squared_residuals,
        decay,
        amplitude,
        offset,
    ) = best.ok_or_else(|| {
        MirrorBenchmarkError::DecayFitFailure {
            reason:
                "bounded optimizer did not produce a candidate".to_owned(),
        }
    })?;

    let diagnostics =
        calculate_fit_diagnostics(
            &points,
            amplitude,
            decay,
            offset,
            sum_squared_residuals,
        )?;

    Ok(MirrorDecayFit {
        model: MIRROR_DECAY_MODEL,
        amplitude,
        decay,
        offset,
        diagnostics,
    })
}

/// Solves:
///
/// ```text
/// y = A*x + B
/// ```
///
/// for fixed `p`, where `x = p^depth`.
///
/// Returns:
///
/// ```text
/// (SSE, A, B)
/// ```
fn fit_linear_amplitude_offset(
    points: &[(f64, f64)],
    p: f64,
) -> Result<(f64, f64, f64), MirrorBenchmarkError> {
    if !p.is_finite()
        || p < 0.0
        || p > 1.0
    {
        return Err(
            MirrorBenchmarkError::InvalidProbability {
                value: p,
            },
        );
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xx = 0.0;
    let mut sum_xy = 0.0;

    let mut transformed = Vec::with_capacity(
        points.len(),
    );

    for &(depth, y) in points {
        let x = if depth == 0.0 {
            1.0
        } else {
            p.powf(depth)
        };

        if !x.is_finite() {
            return Err(
                MirrorBenchmarkError::NonFiniteStatistic {
                    statistic: "decay_basis",
                },
            );
        }

        transformed.push((x, y));

        sum_x += x;
        sum_y += y;
        sum_xx += x * x;
        sum_xy += x * y;
    }

    let n = transformed.len() as f64;

    let denominator =
        n * sum_xx - sum_x * sum_x;

    let (amplitude, offset) =
        if denominator.abs() <= DECAY_EPSILON {
            // Degenerate basis: use a constant model.
            (
                0.0,
                sum_y / n,
            )
        } else {
            let amplitude =
                (n * sum_xy - sum_x * sum_y)
                    / denominator;

            let offset =
                (sum_y - amplitude * sum_x)
                    / n;

            (amplitude, offset)
        };

    if !amplitude.is_finite()
        || !offset.is_finite()
    {
        return Err(
            MirrorBenchmarkError::NonFiniteStatistic {
                statistic: "decay_parameters",
            },
        );
    }

    let mut sse = 0.0;

    for &(x, y) in &transformed {
        let predicted =
            amplitude * x + offset;

        let residual =
            y - predicted;

        sse += residual * residual;
    }

    if !sse.is_finite() {
        return Err(
            MirrorBenchmarkError::NonFiniteStatistic {
                statistic: "decay_sse",
            },
        );
    }

    Ok((sse, amplitude, offset))
}

fn calculate_fit_diagnostics(
    points: &[(f64, f64)],
    amplitude: f64,
    decay: f64,
    offset: f64,
    sse: f64,
) -> Result<DecayFitDiagnostics, MirrorBenchmarkError> {
    let mean_y =
        points
            .iter()
            .map(|(_, y)| *y)
            .sum::<f64>()
            / points.len() as f64;

    let mut total_sum_squares = 0.0;

    for &(depth, y) in points {
        let predicted =
            amplitude * decay.powf(depth) + offset;

        if !predicted.is_finite() {
            return Err(
                MirrorBenchmarkError::NonFiniteStatistic {
                    statistic: "decay_prediction",
                },
            );
        }

        let centered =
            y - mean_y;

        total_sum_squares +=
            centered * centered;
    }

    let rmse =
        (sse / points.len() as f64).sqrt();

    let r_squared =
        if total_sum_squares <= DECAY_EPSILON {
            None
        } else {
            Some(
                1.0 - sse / total_sum_squares,
            )
        };

    if !rmse.is_finite()
        || r_squared
            .map(|value| !value.is_finite())
            .unwrap_or(false)
    {
        return Err(
            MirrorBenchmarkError::NonFiniteStatistic {
                statistic: "decay_diagnostics",
            },
        );
    }

    Ok(DecayFitDiagnostics {
        points: points.len(),
        sum_squared_residuals: sse,
        root_mean_squared_error: rmse,
        r_squared,
        converged: true,
        constrained: true,
    })
}

// =============================================================================
// Utility
// =============================================================================

fn clamp_unit_interval(
    value: f64,
) -> f64 {
    if value <= 0.0 {
        0.0
    } else if value >= 1.0 {
        1.0
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
    use crate::quantum::benchmarking::generators::clifford::CliffordPrimitive;
    use crate::quantum::benchmarking::generators::mirror_circuits::{
        MirrorLayer,
        MirrorOperation,
    };

    fn simple_mirror() -> MirrorCircuit {
        let layer = MirrorLayer::from_operations(
            vec![
                MirrorOperation::SingleQubit {
                    qubit: 0,
                    primitive: CliffordPrimitive::H,
                },
            ],
            1,
        )
        .expect("valid layer");

        MirrorCircuit::from_forward_layers(
            1,
            vec![layer],
        )
        .expect("valid mirror circuit")
    }

    fn counts(
        expected: &str,
        successful: u64,
        alternate: &str,
        alternate_count: u64,
    ) -> BTreeMap<String, u64> {
        let mut result = BTreeMap::new();

        if successful > 0 {
            result.insert(
                expected.to_owned(),
                successful,
            );
        }

        if alternate_count > 0 {
            result.insert(
                alternate.to_owned(),
                alternate_count,
            );
        }

        result
    }

    #[test]
    fn ideal_single_qubit_mirror_has_unit_return_probability() {
        let circuit = simple_mirror();

        let case = MirrorBenchmarkCase::new(
            "ideal",
            circuit,
        )
        .expect("valid case");

        let observation =
            MirrorExecutionObservation::new(
                "ideal",
                1_000,
                counts(
                    "0",
                    1_000,
                    "1",
                    0,
                ),
            );

        let result =
            MirrorBenchmarkProtocol::new()
                .analyze_case(
                    &MirrorBenchmarkConfig::default(),
                    &case,
                    &observation,
                )
                .expect("analysis succeeds");

        assert_eq!(
            result.successful_shots,
            1_000
        );

        assert_eq!(
            result.return_probability,
            1.0
        );

        assert_eq!(
            result.mean_hamming_distance,
            Some(0.0)
        );

        assert_eq!(
            result.effective_polarization,
            Some(1.0)
        );
    }

    #[test]
    fn hamming_distance_is_calculated_against_expected_state() {
        let circuit = simple_mirror();

        let case =
            MirrorBenchmarkCase::new(
                "hamming",
                circuit,
            )
            .expect("valid case");

        let observation =
            MirrorExecutionObservation::new(
                "hamming",
                100,
                counts(
                    "0",
                    75,
                    "1",
                    25,
                ),
            );

        let result =
            MirrorBenchmarkProtocol::new()
                .analyze_case(
                    &MirrorBenchmarkConfig::default(),
                    &case,
                    &observation,
                )
                .expect("analysis succeeds");

        assert_eq!(
            result.successful_shots,
            75
        );

        assert!(
            (result.return_probability - 0.75).abs()
                < 1.0e-12
        );

        assert!(
            (result.mean_hamming_distance.unwrap() - 0.25).abs()
                < 1.0e-12
        );

        assert!(
            (result.effective_polarization.unwrap() - 0.5).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn invalid_bitstring_is_rejected() {
        let circuit = simple_mirror();

        let case =
            MirrorBenchmarkCase::new(
                "invalid",
                circuit,
            )
            .expect("valid case");

        let mut result = BTreeMap::new();

        result.insert(
            "00".to_owned(),
            10,
        );

        let observation =
            MirrorExecutionObservation::new(
                "invalid",
                10,
                result,
            );

        let error =
            MirrorBenchmarkProtocol::new()
                .analyze_case(
                    &MirrorBenchmarkConfig::default(),
                    &case,
                    &observation,
                )
                .expect_err("invalid bitstring must fail");

        assert!(matches!(
            error,
            MirrorBenchmarkError::InvalidBitstring { .. }
        ));
    }

    #[test]
    fn inconsistent_shot_count_is_rejected() {
        let circuit = simple_mirror();

        let case =
            MirrorBenchmarkCase::new(
                "shots",
                circuit,
            )
            .expect("valid case");

        let observation =
            MirrorExecutionObservation::new(
                "shots",
                100,
                counts(
                    "0",
                    50,
                    "1",
                    25,
                ),
            );

        let error =
            MirrorBenchmarkProtocol::new()
                .analyze_case(
                    &MirrorBenchmarkConfig::default(),
                    &case,
                    &observation,
                )
                .expect_err("shot mismatch must fail");

        assert!(matches!(
            error,
            MirrorBenchmarkError::InconsistentShotCount { .. }
        ));
    }

    #[test]
    fn confidence_lower_bound_acceptance_is_conservative() {
        let circuit = simple_mirror();

        let case =
            MirrorBenchmarkCase::new(
                "threshold",
                circuit,
            )
            .expect("valid case");

        let mut config =
            MirrorBenchmarkConfig::default();

        config.acceptance =
            AcceptancePolicy::confidence_lower_bound(
                0.90,
            )
            .expect("valid threshold");

        let observation =
            MirrorExecutionObservation::new(
                "threshold",
                100,
                counts(
                    "0",
                    92,
                    "1",
                    8,
                ),
            );

        let result =
            MirrorBenchmarkProtocol::new()
                .analyze_case(
                    &config,
                    &case,
                    &observation,
                )
                .expect("analysis succeeds");

        assert_eq!(
            result.passed,
            Some(false)
        );
    }

    #[test]
    fn wilson_interval_is_inside_unit_interval() {
        let interval =
            wilson_interval(
                50,
                100,
                0.95,
            )
            .expect("valid interval");

        assert!(
            interval.lower >= 0.0
        );

        assert!(
            interval.upper <= 1.0
        );

        assert!(
            interval.lower <= interval.upper
        );
    }

    #[test]
    fn all_successes_have_valid_wilson_interval() {
        let interval =
            wilson_interval(
                1_000,
                1_000,
                0.95,
            )
            .expect("valid interval");

        assert_eq!(
            interval.upper,
            1.0
        );

        assert!(
            interval.lower >= 0.99
        );
    }

    #[test]
    fn no_successes_have_valid_wilson_interval() {
        let interval =
            wilson_interval(
                0,
                1_000,
                0.95,
            )
            .expect("valid interval");

        assert_eq!(
            interval.lower,
            0.0
        );

        assert!(
            interval.upper <= 0.01
        );
    }

    #[test]
    fn depth_fit_is_available_for_multiple_depths() {
        let circuit_1 = simple_mirror();

        let layer_2 =
            MirrorLayer::from_operations(
                vec![
                    MirrorOperation::SingleQubit {
                        qubit: 0,
                        primitive:
                            CliffordPrimitive::S,
                    },
                ],
                1,
            )
            .expect("valid layer");

        let circuit_2 =
            MirrorCircuit::from_forward_layers(
                1,
                vec![
                    layer_2.clone(),
                    layer_2,
                ],
            )
            .expect("valid circuit");

        let case_1 =
            MirrorBenchmarkCase::new(
                "depth-1",
                circuit_1,
            )
            .expect("valid case");

        let case_2 =
            MirrorBenchmarkCase::new(
                "depth-2",
                circuit_2,
            )
            .expect("valid case");

        // Third point: three forward layers.
        let layer_3 =
            MirrorLayer::from_operations(
                vec![
                    MirrorOperation::SingleQubit {
                        qubit: 0,
                        primitive:
                            CliffordPrimitive::H,
                    },
                ],
                1,
            )
            .expect("valid layer");

        let circuit_3 =
            MirrorCircuit::from_forward_layers(
                1,
                vec![
                    layer_3.clone(),
                    layer_3.clone(),
                    layer_3,
                ],
            )
            .expect("valid circuit");

        let case_3 =
            MirrorBenchmarkCase::new(
                "depth-3",
                circuit_3,
            )
            .expect("valid case");

        let observations =
            vec![
                MirrorExecutionObservation::new(
                    "depth-1",
                    1_000,
                    counts(
                        "0",
                        950,
                        "1",
                        50,
                    ),
                ),
                MirrorExecutionObservation::new(
                    "depth-2",
                    1_000,
                    counts(
                        "0",
                        900,
                        "1",
                        100,
                    ),
                ),
                MirrorExecutionObservation::new(
                    "depth-3",
                    1_000,
                    counts(
                        "0",
                        850,
                        "1",
                        150,
                    ),
                ),
            ];

        let result =
            MirrorBenchmarkProtocol::new()
                .analyze(
                    &MirrorBenchmarkConfig::default(),
                    &[
                        case_1,
                        case_2,
                        case_3,
                    ],
                    &observations,
                )
                .expect("analysis succeeds");

        assert!(
            result.decay_fit.is_some()
        );

        let fit =
            result.decay_fit.unwrap();

        assert!(
            fit.decay >= 0.0
                && fit.decay <= 1.0
        );

        assert!(
            fit.diagnostics.converged
        );
    }

    #[test]
    fn protocol_does_not_claim_mrb() {
        assert_eq!(
            MIRROR_BENCHMARK_KIND,
            "generic_mirror_circuit"
        );

        assert_ne!(
            MIRROR_BENCHMARK_ID,
            "mirror_randomized_benchmarking"
        );
    }
}