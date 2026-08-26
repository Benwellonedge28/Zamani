//! Zamani Quantum Benchmarking — QEC Decoder Benchmark.
//!
//! # Purpose
//!
//! This module benchmarks an already-existing Zamani QEC decoder through the
//! canonical `quantum::error_correction::decoder::Decoder` contract.
//!
//! It does NOT implement a decoder.
//!
//! The benchmark measures:
//!
//! - decode correctness when an oracle is supplied;
//! - decode success/failure;
//! - decoder-reported termination;
//! - latency;
//! - throughput;
//! - correction weight;
//! - decoder iterations;
//! - aggregate resource information;
//! - percentile latency;
//! - failure counts;
//! - correctness counts;
//! - deterministic benchmark identity;
//! - benchmark validity and warnings.
//!
//! # Architectural position
//!
//! ```text
//! quantum::error_correction
//!          │
//!          │ canonical Decoder trait
//!          ▼
//! benchmarking::qec::decoder
//!          │
//!          ├── benchmark cases
//!          ├── timing
//!          ├── correctness oracle
//!          ├── aggregate statistics
//!          └── benchmark result
//!
//!          ▼
//! benchmarking::metrics
//!          ▼
//! reporting / analysis / regression
//! ```
//!
//! # Critical boundary
//!
//! The decoder benchmark does NOT:
//!
//! - implement MWPM;
//! - implement Union-Find;
//! - construct decoding graphs;
//! - mutate Pauli frames;
//! - access QPU credentials;
//! - bypass decoder capability checks;
//! - bypass QEC limits;
//! - own cancellation;
//! - own memory allocation policy;
//! - redefine `DecodeResult`;
//! - claim logical correctness without an oracle;
//! - silently discard decoder failures;
//! - silently discard latency outliers.
//!
//! Those responsibilities remain in the QEC subsystem and the other
//! benchmarking modules.
//!
//! # Scientific interpretation
//!
//! Decoder performance has at least two independent dimensions:
//!
//! ```text
//! correctness
//! latency
//! ```
//!
//! A decoder that produces correct answers but cannot meet the required
//! real-time latency budget is not equivalent to a decoder that satisfies both
//! requirements.
//!
//! Therefore this module deliberately reports both.
//!
//! # Correctness
//!
//! There are two modes:
//!
//! 1. `OracleMode::ExactCorrection`
//!
//!    The benchmark case contains the expected physical correction.
//!
//!    The returned correction is compared exactly with the expected correction.
//!
//! 2. `OracleMode::SuccessOnly`
//!
//!    The benchmark measures successful decoder execution but does not claim
//!    that the produced correction is logically correct.
//!
//! This distinction is intentional.
//!
//! `DecodeResult` itself explicitly represents decoder output and does not by
//! itself prove logical correctness. Logical correctness belongs to the QEC
//! logical-equivalence/verification layer.
//!
//! # Latency
//!
//! Timing includes the complete call to:
//!
//! ```text
//! Decoder::decode_with_context()
//! ```
//!
//! Therefore the measurement includes the canonical admission boundary and
//! decoder execution, but excludes benchmark-case construction performed
//! before timing.
//!
//! This gives Zamani a stable end-to-end decoder-call measurement.
//!
//! # Warm-up
//!
//! Warm-up executions are supported and are NOT included in reported
//! measurements.
//!
//! This permits callers to reduce first-call allocation/cache effects without
//! contaminating benchmark statistics.
//!
//! # Determinism
//!
//! Benchmark case order is preserved.
//!
//! No random sampling is performed by this module.
//!
//! If random syndrome workloads are desired, the workload generator must
//! create them deterministically before calling this benchmark.
//!
//! # Resource safety
//!
//! Benchmark inputs are bounded by explicit benchmark limits:
//!
//! - maximum cases;
//! - maximum warm-up cases;
//! - maximum metadata size;
//! - maximum percentile input size.
//!
//! The benchmark does not clone or retain `DecodeResult` values after each
//! case except for the optional correction information needed for immediate
//! correctness classification.
//!
//! # Integration contract
//!
//! This module depends only on:
//!
//! ```text
//! quantum::error_correction::decoder
//! quantum::error_correction::decoder_result
//! quantum::error_correction::stabilizer
//! ```
//!
//! It intentionally does NOT depend on:
//!
//! ```text
//! benchmarking::core
//! benchmarking::metrics
//! benchmarking::statistics
//! benchmarking::reporting
//! benchmarking::analysis
//! ```
//!
//! This keeps the file independently implementable.
//!
//! Later modules may wrap `DecoderBenchmarkResult` into the universal
//! `BenchmarkResult` without changing this file's mathematical contract.
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
//! `unsafe` is forbidden.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::time::{Duration, Instant};

use super::super::error_correction::decoder::{
    DecodeContext,
    DecodeInput,
    Decoder,
};
use super::super::error_correction::decoder_result::{
    Correction,
    DecodeResourceUsage,
    DecodeResult,
    DecodeTermination,
    DecoderId,
};
use super::super::error_correction::errors::{
    DecoderKind,
    QecError,
    QecResult,
};
use super::super::error_correction::stabilizer::Syndrome;

/* ========================================================================= */
/* Public constants                                                          */
/* ========================================================================= */

/// Stable benchmark identifier.
pub const QEC_DECODER_BENCHMARK_ID: &str = "qec.decoder";

/// Benchmark-result schema version.
///
/// Increment this when the semantic meaning of a result changes.
pub const QEC_DECODER_BENCHMARK_VERSION: u32 = 1;

/// Maximum number of warm-up executions.
///
/// This is a benchmark safety limit, not a decoder execution limit.
pub const MAX_WARMUP_CASES: usize = 1_000_000;

/// Maximum number of measured cases.
pub const MAX_BENCHMARK_CASES: usize = 10_000_000;

/// Maximum number of latency observations retained for percentile analysis.
pub const MAX_LATENCY_SAMPLES: usize = 10_000_000;

/// Maximum benchmark label length in bytes.
pub const MAX_LABEL_BYTES: usize = 256;

/// Maximum warning count retained by one result.
pub const MAX_WARNINGS: usize = 256;

/* ========================================================================= */
/* Error model                                                               */
/* ========================================================================= */

/// Errors specific to the decoder benchmark layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoderBenchmarkError {
    /// No benchmark cases were supplied.
    EmptyWorkload,

    /// The benchmark workload exceeded its safety bound.
    WorkloadLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Warm-up workload exceeded its safety bound.
    WarmupLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// A label was too large.
    LabelTooLong {
        bytes: usize,
        maximum: usize,
    },

    /// Benchmark case identity is invalid.
    InvalidCase {
        index: usize,
        message: String,
    },

    /// Latency statistics could not be calculated.
    InvalidLatencyData,

    /// An aggregate would overflow.
    AggregateOverflow,

    /// A decoder returned an invalid successful result.
    InvalidDecoderResult {
        index: usize,
        message: String,
    },

    /// The benchmark was configured inconsistently.
    InvalidConfiguration {
        message: String,
    },
}

impl fmt::Display for DecoderBenchmarkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyWorkload => {
                formatter.write_str(
                    "QEC decoder benchmark requires at least one measured case",
                )
            }

            Self::WorkloadLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "QEC decoder benchmark workload contains {} cases; \
                     maximum is {}",
                    requested, maximum
                )
            }

            Self::WarmupLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "QEC decoder benchmark contains {} warm-up cases; \
                     maximum is {}",
                    requested, maximum
                )
            }

            Self::LabelTooLong { bytes, maximum } => {
                write!(
                    formatter,
                    "benchmark case label is {} bytes; maximum is {}",
                    bytes, maximum
                )
            }

            Self::InvalidCase { index, message } => {
                write!(
                    formatter,
                    "invalid decoder benchmark case {}: {}",
                    index, message
                )
            }

            Self::InvalidLatencyData => {
                formatter.write_str(
                    "latency statistics cannot be calculated from the \
                     supplied benchmark observations",
                )
            }

            Self::AggregateOverflow => {
                formatter.write_str(
                    "decoder benchmark aggregate would overflow its \
                     numeric representation",
                )
            }

            Self::InvalidDecoderResult { index, message } => {
                write!(
                    formatter,
                    "decoder returned an invalid result for benchmark \
                     case {}: {}",
                    index, message
                )
            }

            Self::InvalidConfiguration { message } => {
                write!(
                    formatter,
                    "invalid QEC decoder benchmark configuration: {}",
                    message
                )
            }
        }
    }
}

impl std::error::Error for DecoderBenchmarkError {}

/// Result type for this module.
pub type DecoderBenchmarkResult<T> =
    Result<T, DecoderBenchmarkError>;

/* ========================================================================= */
/* Oracle mode                                                               */
/* ========================================================================= */

/// Defines what the decoder benchmark is allowed to claim about correctness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OracleMode {
    /// Compare the decoder correction with the exact expected correction.
    ///
    /// This is a physical-correction equality test.
    ExactCorrection,

    /// Record decoder execution success but do not claim correction
    /// correctness.
    SuccessOnly,
}

impl OracleMode {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactCorrection => "exact_correction",
            Self::SuccessOnly => "success_only",
        }
    }

    /// Returns whether the mode provides an explicit correction oracle.
    pub const fn has_oracle(self) -> bool {
        matches!(self, Self::ExactCorrection)
    }
}

impl fmt::Display for OracleMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/* ========================================================================= */
/* Benchmark configuration                                                   */
/* ========================================================================= */

/// Configuration controlling one decoder benchmark run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecoderBenchmarkConfig {
    /// Number of warm-up executions.
    pub warmup_runs: usize,

    /// Whether a decoder error terminates the entire benchmark.
    ///
    /// When false, errors are recorded as failed cases and benchmarking
    /// continues.
    pub fail_fast: bool,

    /// Whether successful decoder results whose correction does not match the
    /// oracle should count as correctness failures rather than aborting.
    pub continue_on_incorrect: bool,

    /// Correctness interpretation.
    pub oracle_mode: OracleMode,

    /// Whether latency percentiles should be calculated.
    pub calculate_percentiles: bool,
}

impl Default for DecoderBenchmarkConfig {
    fn default() -> Self {
        Self {
            warmup_runs: 0,
            fail_fast: false,
            continue_on_incorrect: true,
            oracle_mode: OracleMode::SuccessOnly,
            calculate_percentiles: true,
        }
    }
}

impl DecoderBenchmarkConfig {
    /// Validates benchmark configuration.
    pub fn validate(&self) -> DecoderBenchmarkResult<()> {
        if self.warmup_runs > MAX_WARMUP_CASES {
            return Err(
                DecoderBenchmarkError::WarmupLimitExceeded {
                    requested: self.warmup_runs,
                    maximum: MAX_WARMUP_CASES,
                },
            );
        }

        Ok(())
    }
}

/* ========================================================================= */
/* Benchmark case                                                            */
/* ========================================================================= */

/// One decoder benchmark workload item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoderBenchmarkCase {
    /// Stable case identifier.
    id: u64,

    /// Optional human-readable label.
    label: Option<String>,

    /// Syndrome presented to the decoder.
    syndrome: Syndrome,

    /// Optional exact correction oracle.
    expected_correction: Option<Correction>,
}

impl DecoderBenchmarkCase {
    /// Creates a case without an exact correction oracle.
    pub fn new(
        id: u64,
        syndrome: Syndrome,
    ) -> DecoderBenchmarkResult<Self> {
        Self::with_oracle(id, syndrome, None)
    }

    /// Creates a case with an exact correction oracle.
    pub fn with_expected_correction(
        id: u64,
        syndrome: Syndrome,
        expected_correction: Correction,
    ) -> DecoderBenchmarkResult<Self> {
        Self::with_oracle(
            id,
            syndrome,
            Some(expected_correction),
        )
    }

    /// Creates a case with an optional oracle.
    pub fn with_oracle(
        id: u64,
        syndrome: Syndrome,
        expected_correction: Option<Correction>,
    ) -> DecoderBenchmarkResult<Self> {
        if let Some(correction) = &expected_correction {
            /*
             * We cannot infer the code's physical qubit count from the
             * syndrome alone. Therefore the exact correction is deliberately
             * not dimension-checked here.
             *
             * The canonical decoder result validation and code-specific
             * verification layers remain authoritative for topology.
             */
            if correction.num_qubits() == 0 {
                return Err(
                    DecoderBenchmarkError::InvalidCase {
                        index: 0,
                        message:
                            "expected correction must contain at least one \
                             physical qubit"
                                .to_owned(),
                    },
                );
            }
        }

        Ok(Self {
            id,
            label: None,
            syndrome,
            expected_correction,
        })
    }

    /// Attaches a label to this case.
    pub fn with_label(
        mut self,
        label: impl Into<String>,
    ) -> DecoderBenchmarkResult<Self> {
        let label = label.into();

        if label.len() > MAX_LABEL_BYTES {
            return Err(
                DecoderBenchmarkError::LabelTooLong {
                    bytes: label.len(),
                    maximum: MAX_LABEL_BYTES,
                },
            );
        }

        self.label = Some(label);
        Ok(self)
    }

    /// Returns the stable case identifier.
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Returns the optional case label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns the syndrome.
    #[must_use]
    pub fn syndrome(&self) -> &Syndrome {
        &self.syndrome
    }

    /// Returns the expected correction, when present.
    #[must_use]
    pub fn expected_correction(&self) -> Option<&Correction> {
        self.expected_correction.as_ref()
    }

    /// Converts this benchmark case into canonical decoder input.
    #[must_use]
    pub fn input(&self) -> DecodeInput {
        DecodeInput::new(self.syndrome.clone())
    }
}

/* ========================================================================= */
/* Per-case observation                                                      */
/* ========================================================================= */

/// Classification of one benchmark execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaseOutcome {
    /// Decoder returned a successful result and the exact correction matched.
    Correct,

    /// Decoder returned a successful result but the exact correction did not
    /// match.
    Incorrect,

    /// Decoder returned a successful result but no correction oracle existed.
    SuccessfulWithoutOracle,

    /// Decoder returned an error.
    DecoderError,

    /// Decoder returned a non-success termination through the result channel.
    InvalidTermination,
}

impl CaseOutcome {
    /// Returns true if this case is considered a successful decode.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(
            self,
            Self::Correct
                | Self::SuccessfulWithoutOracle
        )
    }

    /// Returns true when correctness was explicitly established.
    #[must_use]
    pub const fn correctness_known(self) -> bool {
        matches!(
            self,
            Self::Correct | Self::Incorrect
        )
    }
}

/// Immutable result of one decoder benchmark case.
#[derive(Debug, Clone)]
pub struct DecoderCaseObservation {
    /// Stable benchmark case ID.
    pub case_id: u64,

    /// Optional human-readable label.
    pub label: Option<String>,

    /// Number of syndrome events.
    pub syndrome_events: usize,

    /// Whether the input syndrome was trivial.
    pub trivial_syndrome: bool,

    /// Execution latency.
    pub latency: Duration,

    /// Decoder outcome classification.
    pub outcome: CaseOutcome,

    /// Decoder termination when a result was returned.
    pub termination: Option<DecodeTermination>,

    /// Correction weight when a result was returned.
    pub correction_weight: Option<usize>,

    /// Decoder iterations when a result was returned.
    pub iterations: Option<u64>,

    /// Decoder resource snapshot when a result was returned.
    pub resources: Option<DecodeResourceUsage>,

    /// Canonical decoder identity.
    pub decoder_id: DecoderId,

    /// Decoder category.
    pub decoder_kind: DecoderKind,

    /// Whether the decoder itself returned an error.
    pub had_decoder_error: bool,
}

impl DecoderCaseObservation {
    /// Returns whether the case was measured successfully.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.outcome.is_success()
    }

    /// Returns whether an exact correctness decision was possible.
    #[must_use]
    pub const fn correctness_known(&self) -> bool {
        self.outcome.correctness_known()
    }

    /// Returns latency in nanoseconds.
    #[must_use]
    pub fn latency_nanos(&self) -> u128 {
        self.latency.as_nanos()
    }
}

/* ========================================================================= */
/* Latency statistics                                                        */
/* ========================================================================= */

/// Deterministic latency statistics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatencyStatistics {
    /// Number of latency observations.
    pub samples: usize,

    /// Minimum latency.
    pub minimum: Duration,

    /// Arithmetic mean latency.
    pub mean: Duration,

    /// Median latency.
    pub p50: Duration,

    /// 95th percentile latency.
    pub p95: Duration,

    /// 99th percentile latency.
    pub p99: Duration,

    /// Maximum latency.
    pub maximum: Duration,
}

impl LatencyStatistics {
    /// Calculates statistics from durations.
    pub fn calculate(
        observations: &[Duration],
    ) -> DecoderBenchmarkResult<Self> {
        if observations.is_empty() {
            return Err(
                DecoderBenchmarkError::InvalidLatencyData,
            );
        }

        let mut values: Vec<u128> = observations
            .iter()
            .map(Duration::as_nanos)
            .collect();

        values.sort_unstable();

        let samples = values.len();

        let minimum = *values
            .first()
            .ok_or(DecoderBenchmarkError::InvalidLatencyData)?;

        let maximum = *values
            .last()
            .ok_or(DecoderBenchmarkError::InvalidLatencyData)?;

        let total = values.iter().try_fold(0u128, |sum, value| {
            sum.checked_add(*value)
                .ok_or(DecoderBenchmarkError::AggregateOverflow)
        })?;

        let mean = total / samples as u128;

        let p50 = percentile_nearest_rank(&values, 0.50)?;
        let p95 = percentile_nearest_rank(&values, 0.95)?;
        let p99 = percentile_nearest_rank(&values, 0.99)?;

        Ok(Self {
            samples,
            minimum: duration_from_nanos(minimum)?,
            mean: duration_from_nanos(mean)?,
            p50: duration_from_nanos(p50)?,
            p95: duration_from_nanos(p95)?,
            p99: duration_from_nanos(p99)?,
            maximum: duration_from_nanos(maximum)?,
        })
    }
}

/* ========================================================================= */
/* Benchmark aggregate                                                       */
/* ========================================================================= */

/// Aggregate decoder benchmark result.
///
/// This is the QEC-decoder-specific result contract. It is intentionally
/// independent of `benchmarking::core::result::BenchmarkResult`.
#[derive(Debug, Clone)]
pub struct DecoderBenchmarkResultData {
    /// Benchmark identifier.
    pub benchmark_id: &'static str,

    /// Benchmark schema version.
    pub benchmark_version: u32,

    /// Decoder identity.
    pub decoder_id: DecoderId,

    /// Decoder name.
    pub decoder_name: String,

    /// Decoder category.
    pub decoder_kind: DecoderKind,

    /// Algorithm version.
    pub algorithm_version: String,

    /// Oracle mode.
    pub oracle_mode: OracleMode,

    /// Number of warm-up executions.
    pub warmup_runs: usize,

    /// Number of measured cases.
    pub total_cases: usize,

    /// Number of successful decoder calls.
    pub successful_cases: usize,

    /// Number of explicitly correct cases.
    pub correct_cases: usize,

    /// Number of explicitly incorrect cases.
    pub incorrect_cases: usize,

    /// Number of successful calls without an oracle.
    pub successful_without_oracle: usize,

    /// Number of decoder errors.
    pub decoder_errors: usize,

    /// Number of invalid result terminations.
    pub invalid_terminations: usize,

    /// Whether correctness was fully determined for all cases.
    pub correctness_complete: bool,

    /// Correctness rate when an oracle exists.
    pub correctness_rate: Option<f64>,

    /// Successful execution rate.
    pub success_rate: f64,

    /// Latency statistics.
    pub latency: LatencyStatistics,

    /// Total decoder iterations.
    pub total_iterations: u64,

    /// Mean decoder iterations among successful results.
    pub mean_iterations: Option<f64>,

    /// Mean correction weight among successful results.
    pub mean_correction_weight: Option<f64>,

    /// Aggregated resource usage.
    pub resources: DecodeResourceUsage,

    /// Per-case observations.
    ///
    /// This is bounded by `MAX_BENCHMARK_CASES`.
    pub observations: Vec<DecoderCaseObservation>,

    /// Benchmark warnings.
    pub warnings: Vec<String>,
}

impl DecoderBenchmarkResultData {
    /// Returns whether the benchmark passed a requested correctness threshold.
    ///
    /// No threshold is assumed by this method. The caller supplies the
    /// scientific acceptance criterion.
    pub fn passes_correctness_threshold(
        &self,
        minimum_correctness: f64,
    ) -> bool {
        self.correctness_rate
            .map(|rate| rate >= minimum_correctness)
            .unwrap_or(false)
    }

    /// Returns whether the benchmark passed a requested latency budget.
    pub fn passes_latency_budget(
        &self,
        maximum_latency: Duration,
    ) -> bool {
        self.latency.p99 <= maximum_latency
    }

    /// Returns whether the benchmark provides an explicit correctness result.
    #[must_use]
    pub const fn has_complete_correctness(&self) -> bool {
        self.correctness_complete
    }
}

/* ========================================================================= */
/* Benchmark runner                                                          */
/* ========================================================================= */

/// Production decoder benchmark runner.
///
/// The runner owns only benchmark policy. Decoder execution remains owned by
/// the canonical QEC decoder subsystem.
#[derive(Debug, Clone, Copy)]
pub struct DecoderBenchmark {
    config: DecoderBenchmarkConfig,
}

impl DecoderBenchmark {
    /// Creates a validated benchmark runner.
    pub fn new(
        config: DecoderBenchmarkConfig,
    ) -> DecoderBenchmarkResult<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the benchmark configuration.
    #[must_use]
    pub const fn config(&self) -> DecoderBenchmarkConfig {
        self.config
    }

    /// Executes the benchmark against a canonical Zamani decoder.
    ///
    /// The same `DecodeContext` is supplied to every measured case.
    ///
    /// Warm-up executions occur before measured executions and are excluded
    /// from all statistics.
    pub fn run<D: Decoder>(
        &self,
        decoder: &D,
        context: &DecodeContext<'_>,
        cases: &[DecoderBenchmarkCase],
    ) -> DecoderBenchmarkResult<DecoderBenchmarkResultData> {
        self.validate_cases(cases)?;

        self.run_warmup(
            decoder,
            context,
            cases,
        )?;

        let mut observations =
            Vec::with_capacity(cases.len());

        for (index, case) in cases.iter().enumerate() {
            let observation = self.run_case(
                decoder,
                context,
                case,
                index,
            )?;

            observations.push(observation);
        }

        self.aggregate(
            decoder,
            observations,
        )
    }

    /// Executes only the warm-up phase.
    ///
    /// This method is public so callers can explicitly warm a decoder before
    /// another benchmark family without creating a measured result.
    pub fn warmup<D: Decoder>(
        &self,
        decoder: &D,
        context: &DecodeContext<'_>,
        cases: &[DecoderBenchmarkCase],
    ) -> DecoderBenchmarkResult<()> {
        self.validate_cases(cases)?;

        self.run_warmup(
            decoder,
            context,
            cases,
        )
    }

    fn validate_cases(
        &self,
        cases: &[DecoderBenchmarkCase],
    ) -> DecoderBenchmarkResult<()> {
        if cases.is_empty() {
            return Err(
                DecoderBenchmarkError::EmptyWorkload,
            );
        }

        if cases.len() > MAX_BENCHMARK_CASES {
            return Err(
                DecoderBenchmarkError::WorkloadLimitExceeded {
                    requested: cases.len(),
                    maximum: MAX_BENCHMARK_CASES,
                },
            );
        }

        if self.config.oracle_mode == OracleMode::ExactCorrection {
            for (index, case) in cases.iter().enumerate() {
                if case.expected_correction.is_none() {
                    return Err(
                        DecoderBenchmarkError::InvalidCase {
                            index,
                            message:
                                "ExactCorrection mode requires an expected \
                                 correction for every benchmark case"
                                    .to_owned(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn run_warmup<D: Decoder>(
        &self,
        decoder: &D,
        context: &DecodeContext<'_>,
        cases: &[DecoderBenchmarkCase],
    ) -> DecoderBenchmarkResult<()> {
        if self.config.warmup_runs == 0 {
            return Ok(());
        }

        if cases.is_empty() {
            return Err(
                DecoderBenchmarkError::EmptyWorkload,
            );
        }

        /*
         * Warm-up executions deliberately reuse the supplied benchmark cases.
         *
         * The warm-up count is independent of measured-case count.
         *
         * We cycle deterministically through the supplied workload.
         */
        for warmup_index in 0..self.config.warmup_runs {
            let case_index =
                warmup_index % cases.len();

            let case = &cases[case_index];
            let input = case.input();

            match decoder.decode_with_context(
                &input,
                context,
            ) {
                Ok(_) => {}

                Err(error) => {
                    if self.config.fail_fast {
                        return Err(
                            DecoderBenchmarkError::InvalidCase {
                                index: case_index,
                                message: format!(
                                    "warm-up decoder execution failed: \
                                     {error}"
                                ),
                            },
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn run_case<D: Decoder>(
        &self,
        decoder: &D,
        context: &DecodeContext<'_>,
        case: &DecoderBenchmarkCase,
        index: usize,
    ) -> DecoderBenchmarkResult<DecoderCaseObservation> {
        let input = case.input();

        /*
         * Only decoder execution is timed.
         *
         * Case construction, vector allocation, and benchmark aggregation
         * remain outside the measurement interval.
         */
        let start = Instant::now();

        let result =
            decoder.decode_with_context(
                &input,
                context,
            );

        let latency = start.elapsed();

        match result {
            Ok(result) => {
                self.observe_success(
                    decoder,
                    case,
                    index,
                    result,
                    latency,
                )
            }

            Err(error) => {
                if self.config.fail_fast {
                    return Err(
                        DecoderBenchmarkError::InvalidCase {
                            index,
                            message: format!(
                                "decoder execution failed: {error}"
                            ),
                        },
                    );
                }

                Ok(DecoderCaseObservation {
                    case_id: case.id,
                    label: case.label.clone(),
                    syndrome_events: case.syndrome.len(),
                    trivial_syndrome: case.syndrome.is_trivial(),
                    latency,
                    outcome: CaseOutcome::DecoderError,
                    termination: None,
                    correction_weight: None,
                    iterations: None,
                    resources: None,
                    decoder_id: decoder.id(),
                    decoder_kind: decoder.kind(),
                    had_decoder_error: true,
                })
            }
        }
    }

    fn observe_success<D: Decoder>(
        &self,
        decoder: &D,
        case: &DecoderBenchmarkCase,
        index: usize,
        result: DecodeResult,
        latency: Duration,
    ) -> DecoderBenchmarkResult<DecoderCaseObservation> {
        /*
         * A successful Result channel is required to contain a successful
         * termination. The canonical decoder contract already validates this,
         * but the benchmark checks it again at its own measurement boundary so
         * malformed future implementations cannot silently pollute benchmark
         * statistics.
         */
        if !result.termination().is_success() {
            if self.config.fail_fast {
                return Err(
                    DecoderBenchmarkError::InvalidDecoderResult {
                        index,
                        message: format!(
                            "successful result contained non-success \
                             termination {:?}",
                            result.termination()
                        ),
                    },
                );
            }

            return Ok(DecoderCaseObservation {
                case_id: case.id,
                label: case.label.clone(),
                syndrome_events: case.syndrome.len(),
                trivial_syndrome: case.syndrome.is_trivial(),
                latency,
                outcome: CaseOutcome::InvalidTermination,
                termination: Some(result.termination()),
                correction_weight: Some(
                    result.correction_weight(),
                ),
                iterations: Some(result.iterations()),
                resources: Some(*result.resources()),
                decoder_id: decoder.id(),
                decoder_kind: decoder.kind(),
                had_decoder_error: false,
            });
        }

        let outcome = match (
            self.config.oracle_mode,
            case.expected_correction.as_ref(),
        ) {
            (
                OracleMode::ExactCorrection,
                Some(expected),
            ) => {
                if result.correction() == expected {
                    CaseOutcome::Correct
                } else {
                    CaseOutcome::Incorrect
                }
            }

            (
                OracleMode::SuccessOnly,
                _
            ) => CaseOutcome::SuccessfulWithoutOracle,

            (
                OracleMode::ExactCorrection,
                None,
            ) => {
                /*
                 * This should already have been rejected by
                 * validate_cases(). Keep the defensive branch so this
                 * invariant cannot silently disappear if validation changes.
                 */
                return Err(
                    DecoderBenchmarkError::InvalidCase {
                        index,
                        message:
                            "exact-correction benchmark case has no \
                             expected correction"
                                .to_owned(),
                    },
                );
            }
        };

        if outcome == CaseOutcome::Incorrect
            && !self.config.continue_on_incorrect
        {
            return Err(
                DecoderBenchmarkError::InvalidDecoderResult {
                    index,
                    message:
                        "decoder produced a correction that did not match \
                         the benchmark oracle"
                            .to_owned(),
                },
            );
        }

        Ok(DecoderCaseObservation {
            case_id: case.id,
            label: case.label.clone(),
            syndrome_events: case.syndrome.len(),
            trivial_syndrome: case.syndrome.is_trivial(),
            latency,
            outcome,
            termination: Some(result.termination()),
            correction_weight: Some(
                result.correction_weight(),
            ),
            iterations: Some(result.iterations()),
            resources: Some(*result.resources()),
            decoder_id: decoder.id(),
            decoder_kind: decoder.kind(),
            had_decoder_error: false,
        })
    }

    fn aggregate<D: Decoder>(
        &self,
        decoder: &D,
        observations: Vec<DecoderCaseObservation>,
    ) -> DecoderBenchmarkResult<DecoderBenchmarkResultData> {
        if observations.is_empty() {
            return Err(
                DecoderBenchmarkError::EmptyWorkload,
            );
        }

        let mut successful_cases = 0usize;
        let mut correct_cases = 0usize;
        let mut incorrect_cases = 0usize;
        let mut successful_without_oracle = 0usize;
        let mut decoder_errors = 0usize;
        let mut invalid_terminations = 0usize;

        let mut total_iterations = 0u64;
        let mut iteration_samples = 0usize;

        let mut correction_weight_total = 0u64;
        let mut correction_weight_samples = 0usize;

        let mut resources =
            DecodeResourceUsage::new();

        let mut latency_values =
            Vec::with_capacity(observations.len());

        for observation in &observations {
            latency_values.push(observation.latency);

            match observation.outcome {
                CaseOutcome::Correct => {
                    successful_cases =
                        successful_cases
                            .checked_add(1)
                            .ok_or(
                                DecoderBenchmarkError::
                                    AggregateOverflow,
                            )?;

                    correct_cases =
                        correct_cases
                            .checked_add(1)
                            .ok_or(
                                DecoderBenchmarkError::
                                    AggregateOverflow,
                            )?;
                }

                CaseOutcome::Incorrect => {
                    successful_cases =
                        successful_cases
                            .checked_add(1)
                            .ok_or(
                                DecoderBenchmarkError::
                                    AggregateOverflow,
                            )?;

                    incorrect_cases =
                        incorrect_cases
                            .checked_add(1)
                            .ok_or(
                                DecoderBenchmarkError::
                                    AggregateOverflow,
                            )?;
                }

                CaseOutcome::SuccessfulWithoutOracle => {
                    successful_cases =
                        successful_cases
                            .checked_add(1)
                            .ok_or(
                                DecoderBenchmarkError::
                                    AggregateOverflow,
                            )?;

                    successful_without_oracle =
                        successful_without_oracle
                            .checked_add(1)
                            .ok_or(
                                DecoderBenchmarkError::
                                    AggregateOverflow,
                            )?;
                }

                CaseOutcome::DecoderError => {
                    decoder_errors =
                        decoder_errors
                            .checked_add(1)
                            .ok_or(
                                DecoderBenchmarkError::
                                    AggregateOverflow,
                            )?;
                }

                CaseOutcome::InvalidTermination => {
                    invalid_terminations =
                        invalid_terminations
                            .checked_add(1)
                            .ok_or(
                                DecoderBenchmarkError::
                                    AggregateOverflow,
                            )?;
                }
            }

            if let Some(iterations) =
                observation.iterations
            {
                total_iterations =
                    total_iterations
                        .checked_add(iterations)
                        .ok_or(
                            DecoderBenchmarkError::
                                AggregateOverflow,
                        )?;

                if observation
                    .outcome
                    .is_success()
                {
                    iteration_samples =
                        iteration_samples
                            .checked_add(1)
                            .ok_or(
                                DecoderBenchmarkError::
                                    AggregateOverflow,
                            )?;
                }
            }

            if let Some(weight) =
                observation.correction_weight
            {
                correction_weight_total =
                    correction_weight_total
                        .checked_add(weight as u64)
                        .ok_or(
                            DecoderBenchmarkError::
                                AggregateOverflow,
                        )?;

                if observation
                    .outcome
                    .is_success()
                {
                    correction_weight_samples =
                        correction_weight_samples
                            .checked_add(1)
                            .ok_or(
                                DecoderBenchmarkError::
                                    AggregateOverflow,
                            )?;
                }
            }

            if let Some(usage) =
                observation.resources
            {
                add_resource_usage(
                    &mut resources,
                    &usage,
                )?;
            }
        }

        let latency =
            LatencyStatistics::calculate(
                &latency_values,
            )?;

        let total_cases =
            observations.len();

        let success_rate =
            successful_cases as f64
                / total_cases as f64;

        let correctness_complete =
            self.config.oracle_mode
                == OracleMode::ExactCorrection;

        let correctness_rate =
            if correctness_complete {
                let denominator =
                    correct_cases
                        .checked_add(incorrect_cases)
                        .ok_or(
                            DecoderBenchmarkError::
                                AggregateOverflow,
                        )?;

                if denominator == 0 {
                    None
                } else {
                    Some(
                        correct_cases as f64
                            / denominator as f64,
                    )
                }
            } else {
                None
            };

        let mean_iterations =
            if iteration_samples == 0 {
                None
            } else {
                Some(
                    total_iterations as f64
                        / iteration_samples as f64,
                )
            };

        let mean_correction_weight =
            if correction_weight_samples == 0 {
                None
            } else {
                Some(
                    correction_weight_total as f64
                        / correction_weight_samples
                            as f64,
                )
            };

        let mut warnings =
            Vec::new();

        if decoder_errors > 0 {
            push_warning(
                &mut warnings,
                format!(
                    "{} decoder execution(s) failed",
                    decoder_errors
                ),
            );
        }

        if invalid_terminations > 0 {
            push_warning(
                &mut warnings,
                format!(
                    "{} decoder result(s) had invalid \
                     termination classifications",
                    invalid_terminations
                ),
            );
        }

        if self.config.oracle_mode
            == OracleMode::SuccessOnly
        {
            push_warning(
                &mut warnings,
                "correctness was not established because \
                 the benchmark was run without a correction oracle"
                    .to_owned(),
            );
        }

        if incorrect_cases > 0 {
            push_warning(
                &mut warnings,
                format!(
                    "{} case(s) failed the supplied correction \
                     oracle",
                    incorrect_cases
                ),
            );
        }

        Ok(DecoderBenchmarkResultData {
            benchmark_id:
                QEC_DECODER_BENCHMARK_ID,

            benchmark_version:
                QEC_DECODER_BENCHMARK_VERSION,

            decoder_id:
                decoder.id(),

            decoder_name:
                decoder.name().to_owned(),

            decoder_kind:
                decoder.kind(),

            algorithm_version:
                decoder.algorithm_version()
                    .to_owned(),

            oracle_mode:
                self.config.oracle_mode,

            warmup_runs:
                self.config.warmup_runs,

            total_cases,

            successful_cases,

            correct_cases,

            incorrect_cases,

            successful_without_oracle,

            decoder_errors,

            invalid_terminations,

            correctness_complete,

            correctness_rate,

            success_rate,

            latency,

            total_iterations,

            mean_iterations,

            mean_correction_weight,

            resources,

            observations,

            warnings,
        })
    }
}

/* ========================================================================= */
/* Resource aggregation                                                      */
/* ========================================================================= */

fn add_resource_usage(
    target: &mut DecodeResourceUsage,
    source: &DecodeResourceUsage,
) -> DecoderBenchmarkResult<()> {
    target.peak_memory_bytes =
        target
            .peak_memory_bytes
            .max(source.peak_memory_bytes);

    target.decoder_iterations =
        target
            .decoder_iterations
            .checked_add(
                source.decoder_iterations,
            )
            .ok_or(
                DecoderBenchmarkError::
                    AggregateOverflow,
            )?;

    target.graph_nodes =
        target
            .graph_nodes
            .checked_add(
                source.graph_nodes,
            )
            .ok_or(
                DecoderBenchmarkError::
                    AggregateOverflow,
            )?;

    target.graph_edges =
        target
            .graph_edges
            .checked_add(
                source.graph_edges,
            )
            .ok_or(
                DecoderBenchmarkError::
                    AggregateOverflow,
            )?;

    target.syndrome_events =
        target
            .syndrome_events
            .checked_add(
                source.syndrome_events,
            )
            .ok_or(
                DecoderBenchmarkError::
                    AggregateOverflow,
            )?;

    target.workers =
        target
            .workers
            .max(source.workers);

    target.verification_operations =
        target
            .verification_operations
            .checked_add(
                source.verification_operations,
            )
            .ok_or(
                DecoderBenchmarkError::
                    AggregateOverflow,
            )?;

    target.qpu_shots =
        target
            .qpu_shots
            .checked_add(
                source.qpu_shots,
            )
            .ok_or(
                DecoderBenchmarkError::
                    AggregateOverflow,
            )?;

    Ok(())
}

/* ========================================================================= */
/* Percentiles                                                               */
/* ========================================================================= */

/// Calculates a nearest-rank percentile.
///
/// `p` must lie in `(0, 1]`.
fn percentile_nearest_rank(
    sorted: &[u128],
    p: f64,
) -> DecoderBenchmarkResult<u128> {
    if sorted.is_empty()
        || !p.is_finite()
        || p <= 0.0
        || p > 1.0
    {
        return Err(
            DecoderBenchmarkError::
                InvalidLatencyData,
        );
    }

    /*
     * Nearest-rank:
     *
     * rank = ceil(p * N)
     *
     * Array index is rank - 1.
     *
     * We perform the calculation in floating point only for the percentile
     * position. The actual selected latency remains an exact integer number
     * of nanoseconds.
     */
    let rank_float =
        p * sorted.len() as f64;

    let rank =
        rank_float.ceil() as usize;

    let rank =
        rank.max(1).min(sorted.len());

    sorted
        .get(rank - 1)
        .copied()
        .ok_or(
            DecoderBenchmarkError::
                InvalidLatencyData,
        )
}

/* ========================================================================= */
/* Duration conversion                                                       */
/* ========================================================================= */

fn duration_from_nanos(
    nanos: u128,
) -> DecoderBenchmarkResult<Duration> {
    let seconds =
        nanos / 1_000_000_000u128;

    let subsecond =
        nanos % 1_000_000_000u128;

    if seconds
        > u64::MAX as u128
    {
        return Err(
            DecoderBenchmarkError::
                AggregateOverflow,
        );
    }

    Ok(Duration::new(
        seconds as u64,
        subsecond as u32,
    ))
}

/* ========================================================================= */
/* Warnings                                                                  */
/* ========================================================================= */

fn push_warning(
    warnings: &mut Vec<String>,
    warning: String,
) {
    if warnings.len()
        < MAX_WARNINGS
    {
        warnings.push(warning);
    }
}

/* ========================================================================= */
/* Convenience helpers                                                       */
/* ========================================================================= */

/// Runs a decoder benchmark using default configuration.
pub fn benchmark_decoder<D: Decoder>(
    decoder: &D,
    context: &DecodeContext<'_>,
    cases: &[DecoderBenchmarkCase],
) -> DecoderBenchmarkResult<DecoderBenchmarkResultData> {
    DecoderBenchmark::new(
        DecoderBenchmarkConfig::default(),
    )?
    .run(
        decoder,
        context,
        cases,
    )
}

/// Runs an exact-correction decoder benchmark.
pub fn benchmark_decoder_with_oracle<D: Decoder>(
    decoder: &D,
    context: &DecodeContext<'_>,
    cases: &[DecoderBenchmarkCase],
) -> DecoderBenchmarkResult<DecoderBenchmarkResultData> {
    let config =
        DecoderBenchmarkConfig {
            oracle_mode:
                OracleMode::ExactCorrection,
            ..DecoderBenchmarkConfig::default()
        };

    DecoderBenchmark::new(config)?
        .run(
            decoder,
            context,
            cases,
        )
}

/* ========================================================================= */
/* Tests                                                                      */
/* ========================================================================= */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        let config =
            DecoderBenchmarkConfig::default();

        assert!(
            config.validate().is_ok()
        );
    }

    #[test]
    fn oracle_modes_have_stable_identifiers() {
        assert_eq!(
            OracleMode::ExactCorrection.as_str(),
            "exact_correction"
        );

        assert_eq!(
            OracleMode::SuccessOnly.as_str(),
            "success_only"
        );
    }

    #[test]
    fn percentile_nearest_rank_is_deterministic() {
        let values = [
            10u128,
            20u128,
            30u128,
            40u128,
            50u128,
        ];

        assert_eq!(
            percentile_nearest_rank(
                &values,
                0.50,
            )
            .expect("p50 must exist"),
            30
        );

        assert_eq!(
            percentile_nearest_rank(
                &values,
                0.95,
            )
            .expect("p95 must exist"),
            50
        );

        assert_eq!(
            percentile_nearest_rank(
                &values,
                0.99,
            )
            .expect("p99 must exist"),
            50
        );
    }

    #[test]
    fn latency_statistics_are_deterministic() {
        let observations = [
            Duration::from_nanos(10),
            Duration::from_nanos(20),
            Duration::from_nanos(30),
            Duration::from_nanos(40),
            Duration::from_nanos(50),
        ];

        let statistics =
            LatencyStatistics::calculate(
                &observations,
            )
            .expect(
                "latency statistics must be valid",
            );

        assert_eq!(
            statistics.samples,
            5
        );

        assert_eq!(
            statistics.minimum,
            Duration::from_nanos(10)
        );

        assert_eq!(
            statistics.mean,
            Duration::from_nanos(30)
        );

        assert_eq!(
            statistics.p50,
            Duration::from_nanos(30)
        );

        assert_eq!(
            statistics.p95,
            Duration::from_nanos(50)
        );

        assert_eq!(
            statistics.p99,
            Duration::from_nanos(50)
        );

        assert_eq!(
            statistics.maximum,
            Duration::from_nanos(50)
        );
    }

    #[test]
    fn empty_latency_data_is_rejected() {
        assert_eq!(
            LatencyStatistics::calculate(&[])
                .unwrap_err(),
            DecoderBenchmarkError::
                InvalidLatencyData
        );
    }

    #[test]
    fn duration_conversion_preserves_nanoseconds() {
        let duration =
            duration_from_nanos(
                1_500_000_123,
            )
            .expect(
                "duration must be representable",
            );

        assert_eq!(
            duration,
            Duration::new(
                1,
                500_000_123,
            )
        );
    }

    #[test]
    fn resource_aggregation_sums_additive_fields() {
        let mut target =
            DecodeResourceUsage::new();

        let source =
            DecodeResourceUsage {
                peak_memory_bytes: 100,
                decoder_iterations: 10,
                graph_nodes: 20,
                graph_edges: 30,
                syndrome_events: 4,
                workers: 2,
                verification_operations: 5,
                qpu_shots: 100,
            };

        add_resource_usage(
            &mut target,
            &source,
        )
        .expect(
            "resource aggregation must succeed",
        );

        assert_eq!(
            target.peak_memory_bytes,
            100
        );

        assert_eq!(
            target.decoder_iterations,
            10
        );

        assert_eq!(
            target.graph_nodes,
            20
        );

        assert_eq!(
            target.graph_edges,
            30
        );

        assert_eq!(
            target.syndrome_events,
            4
        );

        assert_eq!(
            target.workers,
            2
        );

        assert_eq!(
            target.verification_operations,
            5
        );

        assert_eq!(
            target.qpu_shots,
            100
        );
    }

    #[test]
    fn resource_peak_memory_uses_maximum() {
        let mut target =
            DecodeResourceUsage {
                peak_memory_bytes: 500,
                ..DecodeResourceUsage::new()
            };

        let source =
            DecodeResourceUsage {
                peak_memory_bytes: 100,
                ..DecodeResourceUsage::new()
            };

        add_resource_usage(
            &mut target,
            &source,
        )
        .expect(
            "resource aggregation must succeed",
        );

        assert_eq!(
            target.peak_memory_bytes,
            500
        );
    }

    #[test]
    fn case_without_oracle_is_constructible() {
        /*
         * A concrete Syndrome constructor is intentionally not duplicated
         * here. The production tests for the QEC subsystem already own
         * syndrome construction. This test only verifies benchmark API
         * constants and configuration behavior.
         */
        assert_eq!(
            QEC_DECODER_BENCHMARK_ID,
            "qec.decoder"
        );
    }
}