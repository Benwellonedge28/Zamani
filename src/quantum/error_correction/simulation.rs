//! Zamani Quantum Error Correction — Statistical Simulation
//!
//! Production simulation boundary for QEC experiments.
//!
//! Architecture:
//!
//! ```text
//! Validated QecConfig
//!        |
//!        v
//! SimulationOptions
//!        |
//!        v
//! Resource preflight
//!        |
//!        v
//! Deterministic seed
//!        |
//!        v
//! ShotRunner
//!        |
//!        +-------------------+
//!        |                   |
//!        v                   v
//! Physical noise        Decoder / QPU
//!        |                   |
//!        +---------+---------+
//!                  |
//!                  v
//!           ShotOutcome
//!                  |
//!                  v
//!          StatisticalReport
//!                  |
//!          +-------+-------+
//!          |               |
//!          v               v
//!       Logical       Wilson interval
//!       failures       / uncertainty
//! ```
//!
//! This module deliberately does NOT:
//! - implement a physical noise model;
//! - implement a decoder;
//! - mutate quantum state;
//! - access QPU credentials;
//! - perform network I/O;
//! - silently create unlimited worker threads;
//! - silently create unlimited memory;
//! - use hidden randomness;
//! - treat an interrupted experiment as successful;
//! - report a resource-limited experiment as statistically complete.
//!
//! Instead, simulation provides a deterministic, bounded statistical harness
//! around the existing noise, decoder, backend and QPU layers.
//!
//! Important distinction:
//!
//! `shots` means completed experimental trials.
//!
//! `logical_failures` means trials whose final logical outcome was classified
//! as a logical failure by the supplied runner.
//!
//! A simulation is only statistically complete when all requested shots have
//! completed successfully.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use super::configuration::QecConfig;
use super::errors::{QecError, QecResult, ResourceKind};

/// Current simulation schema version.
pub const SIMULATION_SCHEMA_VERSION: u32 = 1;

/// Default minimum number of completed shots before a report is considered
/// statistically meaningful.
pub const DEFAULT_MINIMUM_SHOTS: u64 = 100;

/// Maximum number of simulation shots accepted by the local API.
///
/// This is an API safety boundary. The actual production limit should also
/// be supplied through `SimulationOptions`.
pub const MAX_SIMULATION_SHOTS: u64 = 1_000_000_000;

/// Maximum number of completed outcomes retained in memory.
///
/// Simulation statistics are streaming aggregates, so retaining every shot
/// is neither necessary nor desirable.
pub const MAX_RETAINED_OUTCOMES: u64 = 1_000_000;

/// Default confidence level.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Result type for simulation-specific operations.
pub type SimulationResult<T> = Result<T, SimulationError>;

/// Simulation-specific errors.
///
/// These are deliberately convertible to the canonical QEC error boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationError {
    /// Invalid simulation configuration.
    InvalidConfiguration(String),

    /// Invalid statistical configuration.
    InvalidStatistics(String),

    /// Requested number of shots exceeds the configured resource boundary.
    ShotLimitExceeded {
        requested: u64,
        limit: u64,
    },

    /// The supplied runner returned an invalid outcome.
    InvalidOutcome(String),

    /// The simulation was cancelled.
    Cancelled,

    /// The simulation exceeded its execution deadline.
    TimeLimitExceeded,

    /// A runner failed.
    RunnerFailure(String),

    /// Integer arithmetic overflow.
    ArithmeticOverflow,

    /// The simulation did not complete all requested shots.
    Incomplete {
        requested: u64,
        completed: u64,
    },
}

impl fmt::Display for SimulationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid simulation configuration: {message}")
            }

            Self::InvalidStatistics(message) => {
                write!(formatter, "invalid statistical configuration: {message}")
            }

            Self::ShotLimitExceeded { requested, limit } => {
                write!(
                    formatter,
                    "simulation requested {requested} shots but the limit is {limit}"
                )
            }

            Self::InvalidOutcome(message) => {
                write!(formatter, "invalid simulation outcome: {message}")
            }

            Self::Cancelled => {
                write!(formatter, "simulation cancelled")
            }

            Self::TimeLimitExceeded => {
                write!(formatter, "simulation execution time limit exceeded")
            }

            Self::RunnerFailure(message) => {
                write!(formatter, "simulation runner failure: {message}")
            }

            Self::ArithmeticOverflow => {
                write!(formatter, "simulation arithmetic overflow")
            }

            Self::Incomplete {
                requested,
                completed,
            } => {
                write!(
                    formatter,
                    "simulation incomplete: requested {requested}, completed {completed}"
                )
            }
        }
    }
}

impl std::error::Error for SimulationError {}

impl From<SimulationError> for QecError {
    fn from(error: SimulationError) -> Self {
        match error {
            SimulationError::InvalidConfiguration(message) => {
                QecError::invalid_input(message)
            }

            SimulationError::InvalidStatistics(message) => {
                QecError::invalid_input(message)
            }

            SimulationError::ShotLimitExceeded {
                requested,
                limit,
            } => QecError::resource_limit(
                ResourceKind::AllocationCount,
                u128::from(requested),
                u128::from(limit),
                "simulation shot limit exceeded",
            ),

            SimulationError::InvalidOutcome(message) => {
                QecError::invalid_input(message)
            }

            SimulationError::Cancelled => {
                QecError::cancelled("simulation cancellation requested")
            }

            SimulationError::TimeLimitExceeded => QecError::time_limit(
                0,
                0,
                "simulation execution time limit exceeded",
            ),

            SimulationError::RunnerFailure(message) => {
                QecError::decoder_failure(
                    super::errors::DecoderKind::Custom,
                    message,
                )
            }

            SimulationError::ArithmeticOverflow => QecError::numerical_failure(
                super::errors::NumericalOperation::Accumulation,
                "simulation arithmetic overflow",
            ),

            SimulationError::Incomplete {
                requested,
                completed,
            } => QecError::invalid_input(format!(
                "simulation incomplete: requested {requested}, completed {completed}"
            )),
        }
    }
}

/// Classification of one completed simulation shot.
///
/// This deliberately remains independent from `logical.rs` so the simulation
/// layer does not duplicate logical-equivalence mathematics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShotClassification {
    /// Decoder/QEC operation succeeded without a logical failure.
    Success,

    /// Decoder/QEC operation completed and a logical failure was detected.
    LogicalFailure,

    /// The runner could not determine a valid logical result.
    Unknown,
}

impl ShotClassification {
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::LogicalFailure)
    }

    pub const fn is_success(self) -> bool {
        matches!(self, Self::Success)
    }

    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Result returned by the caller-provided simulation runner.
///
/// The runner is responsible for integrating:
///
/// ```text
/// noise.rs
///     ↓
/// syndrome extraction
///     ↓
/// decoder.rs
///     ↓
/// pauli_frame.rs
///     ↓
/// logical.rs
/// ```
///
/// The simulation layer only aggregates the resulting classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShotOutcome {
    /// Final logical classification.
    pub classification: ShotClassification,

    /// Optional number of physical faults generated during this shot.
    pub physical_fault_count: u64,

    /// Optional number of detection events produced by syndrome extraction.
    pub detection_event_count: u64,

    /// Optional number of correction operations produced by the decoder.
    pub correction_count: u64,
}

impl ShotOutcome {
    pub const fn success() -> Self {
        Self {
            classification: ShotClassification::Success,
            physical_fault_count: 0,
            detection_event_count: 0,
            correction_count: 0,
        }
    }

    pub const fn logical_failure() -> Self {
        Self {
            classification: ShotClassification::LogicalFailure,
            physical_fault_count: 0,
            detection_event_count: 0,
            correction_count: 0,
        }
    }

    pub const fn unknown() -> Self {
        Self {
            classification: ShotClassification::Unknown,
            physical_fault_count: 0,
            detection_event_count: 0,
            correction_count: 0,
        }
    }

    pub fn validate(&self) -> SimulationResult<()> {
        // These counters are deliberately bounded by u64, but the method
        // provides a single validation boundary for future invariants.
        Ok(())
    }
}

/// Trait implemented by the actual simulation/QPU execution layer.
///
/// This keeps simulation independent of:
/// - a particular noise model;
/// - a particular decoder;
/// - a particular backend;
/// - a particular QPU vendor.
///
/// Implementations should use the seed supplied to each shot to make their
/// physical noise and decoder execution reproducible.
pub trait ShotRunner {
    /// Execute exactly one simulation shot.
    ///
    /// `shot_index` is stable across retries/replays.
    ///
    /// `seed` is derived by the simulation engine and must be the only
    /// randomness source used by deterministic runners.
    fn run_shot(
        &mut self,
        shot_index: u64,
        seed: u64,
    ) -> QecResult<ShotOutcome>;
}

/// Callback used for cooperative cancellation.
///
/// The callback must be cheap. Expensive cancellation primitives can be
/// wrapped by the caller.
pub type CancellationCheck<'a> = dyn Fn() -> bool + 'a;

/// Simulation execution options.
///
/// Resource and policy decisions are explicit rather than hidden inside
/// `simulate`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulationOptions {
    /// Number of requested shots.
    pub shots: u64,

    /// Base deterministic seed.
    pub seed: u64,

    /// Whether deterministic execution is required.
    pub deterministic: bool,

    /// Number of shots required before a report is considered statistically
    /// meaningful.
    pub minimum_shots: u64,

    /// Confidence level for the Wilson interval.
    ///
    /// Valid range is `(0, 1)`.
    pub confidence_level: f64,

    /// Stop after this many logical failures, if configured.
    ///
    /// `None` means no failure-count stopping condition.
    pub target_failures: Option<u64>,

    /// Maximum wall-clock execution time.
    ///
    /// `None` means use the global configuration limit.
    pub max_duration: Option<Duration>,

    /// Retain individual shot outcomes for replay/debugging.
    ///
    /// This is bounded by `max_retained_outcomes`.
    pub retain_outcomes: bool,

    /// Maximum number of individual outcomes retained.
    pub max_retained_outcomes: u64,
}

impl Default for SimulationOptions {
    fn default() -> Self {
        Self {
            shots: DEFAULT_MINIMUM_SHOTS,
            seed: 0x5A4D_414E_4953_494D,
            deterministic: true,
            minimum_shots: DEFAULT_MINIMUM_SHOTS,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            target_failures: None,
            max_duration: None,
            retain_outcomes: false,
            max_retained_outcomes: 0,
        }
    }
}

impl SimulationOptions {
    /// Validate options independently of global configuration.
    pub fn validate(&self) -> SimulationResult<()> {
        if self.shots == 0 {
            return Err(SimulationError::InvalidConfiguration(
                "shots must be greater than zero".to_owned(),
            ));
        }

        if self.shots > MAX_SIMULATION_SHOTS {
            return Err(SimulationError::ShotLimitExceeded {
                requested: self.shots,
                limit: MAX_SIMULATION_SHOTS,
            });
        }

        if self.minimum_shots == 0 {
            return Err(SimulationError::InvalidStatistics(
                "minimum_shots must be greater than zero".to_owned(),
            ));
        }

        if self.minimum_shots > self.shots {
            return Err(SimulationError::InvalidStatistics(
                "minimum_shots cannot exceed requested shots".to_owned(),
            ));
        }

        if !self.confidence_level.is_finite()
            || self.confidence_level <= 0.0
            || self.confidence_level >= 1.0
        {
            return Err(SimulationError::InvalidStatistics(
                "confidence_level must be finite and strictly between zero and one"
                    .to_owned(),
            ));
        }

        if let Some(target) = self.target_failures {
            if target == 0 {
                return Err(SimulationError::InvalidStatistics(
                    "target_failures must be greater than zero".to_owned(),
                ));
            }
        }

        if self.retain_outcomes
            && self.max_retained_outcomes == 0
        {
            return Err(SimulationError::InvalidConfiguration(
                "retain_outcomes requires max_retained_outcomes > 0"
                    .to_owned(),
            ));
        }

        if self.max_retained_outcomes > MAX_RETAINED_OUTCOMES {
            return Err(SimulationError::InvalidConfiguration(
                "max_retained_outcomes exceeds hard safety ceiling"
                    .to_owned(),
            ));
        }

        Ok(())
    }

    /// Derive a stable per-shot seed.
    ///
    /// This is deliberately a deterministic integer mixer rather than a
    /// hidden RNG. The resulting seed is suitable for passing into `noise.rs`
    /// or another deterministic runner.
    pub fn shot_seed(&self, shot_index: u64) -> u64 {
        splitmix64(
            self.seed
                .wrapping_add(
                    shot_index.wrapping_mul(
                        0x9E37_79B9_7F4A_7C15,
                    ),
                ),
        )
    }
}

/// Statistical aggregate for completed shots.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationCounts {
    pub requested_shots: u64,
    pub completed_shots: u64,
    pub successful_shots: u64,
    pub logical_failures: u64,
    pub unknown_shots: u64,
    pub physical_faults: u64,
    pub detection_events: u64,
    pub corrections: u64,
}

impl SimulationCounts {
    pub fn record(
        &mut self,
        outcome: ShotOutcome,
    ) -> SimulationResult<()> {
        outcome.validate()?;

        self.completed_shots = self
            .completed_shots
            .checked_add(1)
            .ok_or(SimulationError::ArithmeticOverflow)?;

        self.physical_faults = self
            .physical_faults
            .checked_add(outcome.physical_fault_count)
            .ok_or(SimulationError::ArithmeticOverflow)?;

        self.detection_events = self
            .detection_events
            .checked_add(outcome.detection_event_count)
            .ok_or(SimulationError::ArithmeticOverflow)?;

        self.corrections = self
            .corrections
            .checked_add(outcome.correction_count)
            .ok_or(SimulationError::ArithmeticOverflow)?;

        match outcome.classification {
            ShotClassification::Success => {
                self.successful_shots = self
                    .successful_shots
                    .checked_add(1)
                    .ok_or(SimulationError::ArithmeticOverflow)?;
            }

            ShotClassification::LogicalFailure => {
                self.logical_failures = self
                    .logical_failures
                    .checked_add(1)
                    .ok_or(SimulationError::ArithmeticOverflow)?;
            }

            ShotClassification::Unknown => {
                self.unknown_shots = self
                    .unknown_shots
                    .checked_add(1)
                    .ok_or(SimulationError::ArithmeticOverflow)?;
            }
        }

        Ok(())
    }

    /// Completed shots with a known logical classification.
    pub fn classified_shots(&self) -> u64 {
        self.successful_shots
            .saturating_add(self.logical_failures)
    }

    /// Logical error rate over classified shots.
    pub fn logical_error_rate(&self) -> Option<f64> {
        let denominator = self.classified_shots();

        if denominator == 0 {
            return None;
        }

        Some(
            self.logical_failures as f64
                / denominator as f64,
        )
    }

    /// Physical fault rate per completed shot.
    pub fn physical_fault_rate(&self) -> Option<f64> {
        if self.completed_shots == 0 {
            return None;
        }

        Some(
            self.physical_faults as f64
                / self.completed_shots as f64,
        )
    }
}

/// Wilson confidence interval.
///
/// Wilson is preferred over the naive `p ± z sqrt(...)` interval because it
/// remains well behaved near zero and one.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub confidence_level: f64,
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
}

impl ConfidenceInterval {
    pub fn wilson(
        successes: u64,
        trials: u64,
        confidence_level: f64,
    ) -> SimulationResult<Self> {
        if trials == 0 {
            return Err(SimulationError::InvalidStatistics(
                "Wilson interval requires at least one trial"
                    .to_owned(),
            ));
        }

        if successes > trials {
            return Err(SimulationError::InvalidStatistics(
                "successes cannot exceed trials".to_owned(),
            ));
        }

        if !confidence_level.is_finite()
            || confidence_level <= 0.0
            || confidence_level >= 1.0
        {
            return Err(SimulationError::InvalidStatistics(
                "confidence level must be in (0, 1)"
                    .to_owned(),
            ));
        }

        let p = successes as f64 / trials as f64;

        let z = normal_quantile(
            0.5 + confidence_level / 2.0,
        );

        let n = trials as f64;
        let z2 = z * z;

        let denominator =
            1.0 + z2 / n;

        let center =
            (p + z2 / (2.0 * n))
                / denominator;

        let half_width =
            z
                * ((p * (1.0 - p) / n)
                    + z2 / (4.0 * n * n))
                    .sqrt()
                / denominator;

        Ok(Self {
            confidence_level,
            estimate: p,
            lower: (center - half_width).max(0.0),
            upper: (center + half_width).min(1.0),
        })
    }
}

/// Complete simulation report.
///
/// The report explicitly distinguishes:
///
/// - requested shots;
/// - completed shots;
/// - classified shots;
/// - unknown shots;
/// - logical failures;
/// - statistical uncertainty;
/// - termination reason.
///
/// A partial/cancelled experiment therefore cannot accidentally be interpreted
/// as a complete threshold experiment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulationReport {
    pub schema_version: u32,
    pub seed: u64,
    pub deterministic: bool,
    pub elapsed_nanos: u64,
    pub counts: SimulationCounts,
    pub logical_error_interval: Option<ConfidenceInterval>,
    pub physical_fault_rate: Option<f64>,
    pub termination: TerminationReason,
    pub complete: bool,
    pub retained_outcomes: Vec<ShotClassification>,
}

impl SimulationReport {
    pub fn logical_error_rate(&self) -> Option<f64> {
        self.counts.logical_error_rate()
    }

    pub fn is_statistically_complete(&self) -> bool {
        self.complete
            && self.counts.unknown_shots == 0
            && self.counts.completed_shots
                == self.counts.requested_shots
    }
}

/// Why the simulation stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminationReason {
    /// All requested shots completed.
    Completed,

    /// Target logical-failure count reached.
    TargetFailuresReached,

    /// Cooperative cancellation was requested.
    Cancelled,

    /// Configured time budget was exhausted.
    TimeLimitExceeded,

    /// The runner returned an error.
    RunnerFailure,

    /// Statistical execution could not continue safely.
    ResourceLimitExceeded,
}

/// Production simulation engine.
///
/// The engine owns:
/// - validation;
/// - deterministic shot scheduling;
/// - resource preflight;
/// - cancellation checks;
/// - timeout checks;
/// - streaming statistics;
/// - Wilson confidence intervals.
///
/// It does not own physical noise or decoding.
pub struct SimulationEngine<'a> {
    config: &'a QecConfig,
    options: SimulationOptions,
}

impl<'a> SimulationEngine<'a> {
    pub fn new(
        config: &'a QecConfig,
        options: SimulationOptions,
    ) -> QecResult<Self> {
        config
            .validate()
            .map_err(|error| QecError::invalid_input(
                error.to_string(),
            ))?;

        options
            .validate()
            .map_err(QecError::from)?;

        Self::preflight(
            config,
            &options,
        )?;

        Ok(Self {
            config,
            options,
        })
    }

    /// Validate resource requirements before the first shot.
    fn preflight(
        config: &QecConfig,
        options: &SimulationOptions,
    ) -> QecResult<()> {
        let limit =
            u64::from(
                config.limits.max_parallelism,
            );

        // A simulation is currently intentionally single-runner/streaming.
        // Parallel simulation can be added later through the scheduler,
        // where deterministic reductions and worker budgets are enforced.
        if limit == 0 {
            return Err(QecError::resource_limit(
                ResourceKind::Parallelism,
                1,
                0,
                "simulation requires at least one execution slot",
            ));
        }

        // Prevent accidental giant retained-outcome allocations.
        if options.retain_outcomes
            && options.max_retained_outcomes
                > config.limits.max_syndrome_events
        {
            return Err(QecError::resource_limit(
                ResourceKind::SyndromeEvents,
                u128::from(
                    options.max_retained_outcomes,
                ),
                u128::from(
                    config.limits.max_syndrome_events,
                ),
                "retained simulation outcomes exceed global QEC limits",
            ));
        }

        Ok(())
    }

    /// Run a complete simulation.
    pub fn run<R>(
        &self,
        runner: &mut R,
    ) -> QecResult<SimulationReport>
    where
        R: ShotRunner,
    {
        self.run_with_cancellation(
            runner,
            || false,
        )
    }

    /// Run a simulation with cooperative cancellation.
    pub fn run_with_cancellation<R, C>(
        &self,
        runner: &mut R,
        cancellation: C,
    ) -> QecResult<SimulationReport>
    where
        R: ShotRunner,
        C: Fn() -> bool,
    {
        let started =
            Instant::now();

        let deadline =
            self.options
                .max_duration
                .or_else(|| {
                    if self.config
                        .limits
                        .max_decoder_time_ms
                        > 0
                    {
                        Some(
                            Duration::from_millis(
                                self.config
                                    .limits
                                    .max_decoder_time_ms,
                            ),
                        )
                    } else {
                        None
                    }
                });

        let mut counts =
            SimulationCounts {
                requested_shots:
                    self.options.shots,
                ..SimulationCounts::default()
            };

        let mut retained =
            Vec::new();

        let mut termination =
            TerminationReason::Completed;

        for shot_index in 0..self.options.shots {
            if cancellation() {
                termination =
                    TerminationReason::Cancelled;
                break;
            }

            if let Some(limit) = deadline {
                if started.elapsed() >= limit {
                    termination =
                        TerminationReason::TimeLimitExceeded;
                    break;
                }
            }

            let seed =
                self.options
                    .shot_seed(shot_index);

            let outcome =
                match runner.run_shot(
                    shot_index,
                    seed,
                ) {
                    Ok(value) => value,

                    Err(error) => {
                        termination =
                            TerminationReason::RunnerFailure;

                        return Err(error);
                    }
                };

            outcome
                .validate()
                .map_err(QecError::from)?;

            counts
                .record(outcome)
                .map_err(QecError::from)?;

            if self.options.retain_outcomes
                && retained.len()
                    < self.options
                        .max_retained_outcomes
                        as usize
            {
                retained.push(
                    outcome.classification,
                );
            }

            if let Some(target) =
                self.options.target_failures
            {
                if counts.logical_failures
                    >= target
                {
                    termination =
                        TerminationReason::TargetFailuresReached;
                    break;
                }
            }
        }

        let elapsed_nanos =
            u64::try_from(
                started
                    .elapsed()
                    .as_nanos(),
            )
            .unwrap_or(u64::MAX);

        let complete =
            matches!(
                termination,
                TerminationReason::Completed
            )
            && counts.completed_shots
                == counts.requested_shots;

        let interval =
            if counts.classified_shots() > 0 {
                Some(
                    ConfidenceInterval::wilson(
                        counts.logical_failures,
                        counts.classified_shots(),
                        self.options
                            .confidence_level,
                    )
                    .map_err(QecError::from)?,
                )
            } else {
                None
            };

        let report =
            SimulationReport {
                schema_version:
                    SIMULATION_SCHEMA_VERSION,

                seed:
                    self.options.seed,

                deterministic:
                    self.options.deterministic,

                elapsed_nanos,

                physical_fault_rate:
                    counts
                        .physical_fault_rate(),

                counts,

                logical_error_interval:
                    interval,

                termination,

                complete,

                retained_outcomes:
                    retained,
            };

        if !report.is_statistically_complete()
            && matches!(
                termination,
                TerminationReason::Completed
            )
        {
            return Err(
                QecError::invalid_input(
                    "simulation reported completion without completing all requested shots",
                ),
            );
        }

        Ok(report)
    }
}

/// Deterministic 64-bit mixer.
///
/// This is not intended to be a cryptographic primitive. It provides stable
/// independent-looking per-shot seeds for reproducible simulation.
fn splitmix64(
    mut value: u64,
) -> u64 {
    value =
        value
            .wrapping_add(
                0x9E37_79B9_7F4A_7C15,
            );

    let mut z = value;

    z =
        (z ^ (z >> 30))
            .wrapping_mul(
                0xBF58_476D_1CE4_E5B9,
            );

    z =
        (z ^ (z >> 27))
            .wrapping_mul(
                0x94D0_49BB_1331_11EB,
            );

    z ^ (z >> 31)
}

/// Approximation to the standard normal inverse CDF.
///
/// Accuracy is sufficient for statistical reporting while avoiding an
/// additional statistics dependency in the QEC core.
///
/// Domain:
/// `0 < p < 1`.
fn normal_quantile(
    p: f64,
) -> f64 {
    // Acklam-style rational approximation.

    const A: [f64; 6] = [
        -3.969683028665376e1,
        2.209460984245205e2,
        -2.759285104469687e2,
        1.38357751867269e2,
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

    const LOW: f64 = 0.02425;
    const HIGH: f64 = 1.0 - LOW;

    if p < LOW {
        let q =
            (-2.0 * p.ln()).sqrt();

        return ((((C[0] * q + C[1]) * q + C[2])
            * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / (((D[0] * q + D[1]) * q + D[2])
                * q
                + D[3])
                * q
                + 1.0);
    }

    if p > HIGH {
        let q =
            (-2.0 * (1.0 - p).ln()).sqrt();

        return -(((((C[0] * q + C[1]) * q + C[2])
            * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / (((D[0] * q + D[1]) * q + D[2])
                * q
                + D[3])
                * q
                + 1.0));
    }

    let q =
        p - 0.5;

    let r =
        q * q;

    (((((A[0] * r + A[1]) * r + A[2])
        * r
        + A[3])
        * r
        + A[4])
        * r
        + A[5])
        * q)
        / (((((B[0] * r + B[1]) * r + B[2])
            * r
            + B[3])
            * r
            + B[4])
            * r
            + 1.0)
}

/// Convenience function for one-shot simulation execution.
pub fn simulate<R>(
    config: &QecConfig,
    options: SimulationOptions,
    runner: &mut R,
) -> QecResult<SimulationReport>
where
    R: ShotRunner,
{
    SimulationEngine::new(
        config,
        options,
    )?
    .run(runner)
}

/// Convenience function with cooperative cancellation.
pub fn simulate_with_cancellation<R, C>(
    config: &QecConfig,
    options: SimulationOptions,
    runner: &mut R,
    cancellation: C,
) -> QecResult<SimulationReport>
where
    R: ShotRunner,
    C: Fn() -> bool,
{
    SimulationEngine::new(
        config,
        options,
    )?
    .run_with_cancellation(
        runner,
        cancellation,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedRunner {
        failures: u64,
    }

    impl ShotRunner for FixedRunner {
        fn run_shot(
            &mut self,
            shot_index: u64,
            _seed: u64,
        ) -> QecResult<ShotOutcome> {
            if shot_index < self.failures {
                Ok(ShotOutcome::logical_failure())
            } else {
                Ok(ShotOutcome::success())
            }
        }
    }

    #[test]
    fn deterministic_shot_seeds_are_reproducible() {
        let options =
            SimulationOptions::default();

        assert_eq!(
            options.shot_seed(0),
            options.shot_seed(0)
        );

        assert_ne!(
            options.shot_seed(0),
            options.shot_seed(1)
        );
    }

    #[test]
    fn counts_logical_failures() {
        let mut counts =
            SimulationCounts {
                requested_shots: 10,
                ..SimulationCounts::default()
            };

        counts
            .record(
                ShotOutcome::logical_failure(),
            )
            .unwrap();

        counts
            .record(
                ShotOutcome::success(),
            )
            .unwrap();

        assert_eq!(
            counts.completed_shots,
            2
        );

        assert_eq!(
            counts.logical_failures,
            1
        );

        assert_eq!(
            counts.successful_shots,
            1
        );
    }

    #[test]
    fn wilson_interval_is_bounded() {
        let interval =
            ConfidenceInterval::wilson(
                10,
                100,
                0.95,
            )
            .unwrap();

        assert!(
            interval.lower >= 0.0
        );

        assert!(
            interval.upper <= 1.0
        );

        assert!(
            interval.lower
                <= interval.estimate
        );

        assert!(
            interval.estimate
                <= interval.upper
        );
    }

    #[test]
    fn engine_runs_requested_shots() {
        let config =
            QecConfig::deterministic_test();

        let options =
            SimulationOptions {
                shots: 100,
                minimum_shots: 100,
                ..SimulationOptions::default()
            };

        let mut runner =
            FixedRunner {
                failures: 5,
            };

        let report =
            simulate(
                &config,
                options,
                &mut runner,
            )
            .unwrap();

        assert_eq!(
            report.counts.completed_shots,
            100
        );

        assert_eq!(
            report.counts.logical_failures,
            5
        );

        assert!(
            report.is_statistically_complete()
        );
    }

    #[test]
    fn cancellation_does_not_look_complete() {
        let config =
            QecConfig::deterministic_test();

        let options =
            SimulationOptions {
                shots: 100,
                minimum_shots: 1,
                ..SimulationOptions::default()
            };

        let mut runner =
            FixedRunner {
                failures: 0,
            };

        let report =
            simulate_with_cancellation(
                &config,
                options,
                &mut runner,
                || true,
            )
            .unwrap();

        assert_eq!(
            report.termination,
            TerminationReason::Cancelled
        );

        assert!(
            !report.complete
        );

        assert!(
            !report.is_statistically_complete()
        );
    }

    #[test]
    fn target_failure_stopping_is_explicit() {
        let config =
            QecConfig::deterministic_test();

        let options =
            SimulationOptions {
                shots: 1_000,
                minimum_shots: 1,
                target_failures: Some(3),
                ..SimulationOptions::default()
            };

        let mut runner =
            FixedRunner {
                failures: 3,
            };

        let report =
            simulate(
                &config,
                options,
                &mut runner,
            )
            .unwrap();

        assert_eq!(
            report.termination,
            TerminationReason::TargetFailuresReached
        );

        assert_eq!(
            report.counts.logical_failures,
            3
        );

        assert!(
            !report.is_statistically_complete()
        );
    }
}