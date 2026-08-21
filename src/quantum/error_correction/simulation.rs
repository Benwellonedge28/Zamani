//! Zamani Quantum Error Correction — Statistical Simulation
//!
//! Production bounded simulation harness for QEC experiments.
//!
//! # Ownership
//!
//! `simulation.rs` owns:
//!
//! - simulation execution orchestration;
//! - shot scheduling;
//! - deterministic per-shot seed derivation;
//! - simulation option validation;
//! - simulation preflight;
//! - bounded retention of shot classifications;
//! - streaming simulation counters;
//! - simulation-local confidence interval representation;
//! - simulation termination semantics;
//! - simulation resource-usage reporting;
//! - integration boundaries for noise, syndrome extraction, decoders,
//!   Pauli frames, logical classification and QPU adapters.
//!
//! It does NOT own:
//!
//! - physical noise-model mathematics (`noise.rs`);
//! - syndrome extraction (`syndrome.rs`, `syndrome_extractor.rs`);
//! - decoder algorithms (`decoder.rs`, `mwpm.rs`, `union_find.rs`);
//! - logical-equivalence mathematics (`logical.rs`, `logical_equivalence.rs`);
//! - QPU credentials or transport (`qpu_adapter.rs`);
//! - canonical runtime resource accounting (`resources.rs`);
//! - memory allocation policy (`memory.rs`);
//! - cancellation state (`cancellation.rs`);
//! - canonical QEC resource policy (`limits.rs`);
//! - telemetry transport (`telemetry.rs`);
//! - advanced statistical algorithms (`statistical.rs`).
//!
//! # Integration contract
//!
//! ```text
//!                         QecConfig
//!                             |
//!                             v
//!                     SimulationOptions
//!                             |
//!                             v
//!                    Simulation preflight
//!                             |
//!             +---------------+---------------+
//!             |                               |
//!             v                               v
//!      CancellationToken              deterministic seed
//!             |                               |
//!             +---------------+---------------+
//!                             |
//!                             v
//!                       ShotRunner
//!                             |
//!          +------------------+------------------+
//!          |                  |                  |
//!          v                  v                  v
//!       noise.rs        decoder.rs          qpu_adapter.rs
//!          |                  |                  |
//!          +------------------+------------------+
//!                             |
//!                             v
//!                       ShotOutcome
//!                             |
//!                             v
//!                    SimulationCounts
//!                             |
//!                 +-----------+-----------+
//!                 |                       |
//!                 v                       v
//!       ConfidenceInterval        ResourceUsage
//!                 |                       |
//!                 +-----------+-----------+
//!                             |
//!                             v
//!                    SimulationReport
//! ```
//!
//! # Noise integration
//!
//! A `ShotRunner` implementation that uses physical noise should call
//! `NoiseModel::sample` from `noise.rs` using the supplied per-shot seed.
//!
//! The simulation engine does not duplicate the noise model. This prevents
//! the statistical layer from becoming coupled to one particular noise model.
//!
//! # Determinism
//!
//! When deterministic execution is enabled:
//!
//! ```text
//! configuration
//! + base seed
//! + shot index
//! ----------------------
//! -> stable shot seed
//! ```
//!
//! The runner must use that seed as its only stochastic input.
//!
//! # Resource safety
//!
//! Simulation is streaming. It never stores every shot unless the caller
//! explicitly requests bounded retention.
//!
//! Retained outcomes are bounded by both:
//!
//! - `MAX_RETAINED_OUTCOMES`;
//! - the configured QEC memory budget.
//!
//! The simulation engine does not create worker threads.
//!
//! # Statistical correctness
//!
//! `complete` means every requested shot completed.
//!
//! `statistically_meaningful` additionally requires:
//!
//! - at least `minimum_shots` completed shots;
//! - no unknown logical classifications;
//! - a valid classified-shot denominator.
//!
//! Target-failure stopping, cancellation and time-limit termination are never
//! reported as statistically complete.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97.1 using stable standard-library facilities only.

#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::mem::size_of;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use super::cancellation::CancellationToken;
use super::configuration::QecConfig;
use super::errors::{QecError, QecResult, ResourceKind};

/// Serialized simulation schema version.
///
/// Increment when the serialized meaning of simulation structures changes.
pub const SIMULATION_SCHEMA_VERSION: u32 = 2;

/// Default minimum number of completed shots required for a meaningful
/// statistical result.
pub const DEFAULT_MINIMUM_SHOTS: u64 = 100;

/// API-level safety ceiling for simulation requests.
///
/// This is not a second production QEC resource policy. It prevents an
/// untrusted API caller from expressing an obviously unreasonable request
/// before canonical resource admission occurs.
pub const MAX_SIMULATION_SHOTS: u64 = 1_000_000_000;

/// Maximum number of individual shot classifications retained in memory.
pub const MAX_RETAINED_OUTCOMES: u64 = 1_000_000;

/// Default confidence level.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Result type for simulation-local validation.
pub type SimulationResult<T> = Result<T, SimulationError>;

/// Errors specific to simulation orchestration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationError {
    /// Simulation options are invalid.
    InvalidConfiguration(String),

    /// Statistical options are invalid.
    InvalidStatistics(String),

    /// Requested shots exceed the API safety boundary.
    ShotLimitExceeded {
        requested: u64,
        limit: u64,
    },

    /// Retention would exceed the explicitly permitted memory budget.
    RetentionMemoryExceeded {
        requested_bytes: u64,
        maximum_bytes: u64,
    },

    /// A runner returned an invalid shot result.
    InvalidOutcome(String),

    /// Simulation cancellation was requested.
    Cancelled,

    /// Simulation execution exceeded its configured time budget.
    TimeLimitExceeded,

    /// Checked simulation aggregation overflowed.
    ArithmeticOverflow,

    /// Statistical computation received an invalid denominator.
    InvalidStatisticalSample,
}

impl fmt::Display for SimulationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(
                    formatter,
                    "invalid simulation configuration: {message}"
                )
            }

            Self::InvalidStatistics(message) => {
                write!(
                    formatter,
                    "invalid simulation statistics: {message}"
                )
            }

            Self::ShotLimitExceeded {
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "simulation requested {requested} shots; limit is {limit}"
                )
            }

            Self::RetentionMemoryExceeded {
                requested_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "simulation retention requires {requested_bytes} bytes; \
                     maximum permitted is {maximum_bytes}"
                )
            }

            Self::InvalidOutcome(message) => {
                write!(
                    formatter,
                    "invalid simulation outcome: {message}"
                )
            }

            Self::Cancelled => {
                write!(formatter, "simulation cancelled")
            }

            Self::TimeLimitExceeded => {
                write!(
                    formatter,
                    "simulation execution time limit exceeded"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    formatter,
                    "simulation arithmetic overflow"
                )
            }

            Self::InvalidStatisticalSample => {
                write!(
                    formatter,
                    "invalid statistical sample"
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
                "simulation shot request exceeds safety boundary",
            ),

            SimulationError::RetentionMemoryExceeded {
                requested_bytes,
                maximum_bytes,
            } => QecError::resource_limit(
                ResourceKind::MemoryBytes,
                u128::from(requested_bytes),
                u128::from(maximum_bytes),
                "simulation retention exceeds memory budget",
            ),

            SimulationError::InvalidOutcome(message) => {
                QecError::invalid_input(message)
            }

            SimulationError::Cancelled => {
                QecError::cancelled(
                    "simulation cancellation requested",
                )
            }

            SimulationError::TimeLimitExceeded => {
                QecError::time_limit(
                    0,
                    0,
                    "simulation execution time limit exceeded",
                )
            }

            SimulationError::ArithmeticOverflow => {
                QecError::numerical_failure(
                    super::errors::NumericalOperation::Accumulation,
                    "simulation arithmetic overflow",
                )
            }

            SimulationError::InvalidStatisticalSample => {
                QecError::invalid_input(
                    "invalid statistical simulation sample",
                )
            }
        }
    }
}

/// Classification of one completed QEC shot.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub enum ShotClassification {
    /// QEC completed without a logical failure.
    Success,

    /// QEC completed and a logical failure was identified.
    LogicalFailure,

    /// QEC completed but no valid logical classification was available.
    Unknown,
}

impl ShotClassification {
    /// Returns true only for a logical failure.
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::LogicalFailure)
    }

    /// Returns true only for a successful logical result.
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Success)
    }

    /// Returns true when logical classification is unavailable.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Result produced by one simulation shot.
///
/// The simulation engine deliberately receives the final classification
/// instead of implementing decoder or logical-equivalence mathematics itself.
///
/// A runner may obtain that classification through:
///
/// ```text
/// NoiseModel
///     ↓
/// Syndrome extraction
///     ↓
/// Decoder
///     ↓
/// PauliFrame
///     ↓
/// Logical equivalence
///     ↓
/// ShotOutcome
/// ```
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct ShotOutcome {
    /// Final logical classification.
    pub classification: ShotClassification,

    /// Number of physical faults generated during this shot.
    pub physical_fault_count: u64,

    /// Number of syndrome/detection events generated.
    pub detection_event_count: u64,

    /// Number of correction operations produced.
    pub correction_count: u64,
}

impl ShotOutcome {
    /// Creates a successful zero-counter outcome.
    pub const fn success() -> Self {
        Self {
            classification: ShotClassification::Success,
            physical_fault_count: 0,
            detection_event_count: 0,
            correction_count: 0,
        }
    }

    /// Creates a logical-failure zero-counter outcome.
    pub const fn logical_failure() -> Self {
        Self {
            classification: ShotClassification::LogicalFailure,
            physical_fault_count: 0,
            detection_event_count: 0,
            correction_count: 0,
        }
    }

    /// Creates an unknown-classification zero-counter outcome.
    pub const fn unknown() -> Self {
        Self {
            classification: ShotClassification::Unknown,
            physical_fault_count: 0,
            detection_event_count: 0,
            correction_count: 0,
        }
    }

    /// Validates the outcome at the simulation boundary.
    pub fn validate(&self) -> SimulationResult<()> {
        // All counters are unsigned and therefore cannot be negative.
        //
        // This method intentionally exists as a stable validation boundary
        // for future invariants without requiring changes to the engine.
        Ok(())
    }
}

/// Integration boundary for one simulation shot.
///
/// Implementations may connect this interface to:
///
/// - `noise.rs`;
/// - `syndrome.rs`;
/// - `syndrome_extractor.rs`;
/// - `decoder.rs`;
/// - `mwpm.rs`;
/// - `union_find.rs`;
/// - `pauli_frame.rs`;
/// - `logical.rs`;
/// - `qpu_adapter.rs`.
///
/// The simulation engine itself remains independent from all of those
/// implementations.
pub trait ShotRunner {
    /// Executes exactly one shot.
    ///
    /// `shot_index` is stable and begins at zero.
    ///
    /// `seed` is the deterministic per-shot seed generated by the simulation
    /// engine.
    fn run_shot(
        &mut self,
        shot_index: u64,
        seed: u64,
    ) -> QecResult<ShotOutcome>;
}

/// Simulation execution options.
///
/// These options describe one simulation request. They do not replace
/// `QecLimits`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulationOptions {
    /// Number of requested shots.
    pub shots: u64,

    /// Base seed supplied by the caller.
    pub seed: u64,

    /// Whether this request requires deterministic execution.
    pub deterministic: bool,

    /// Minimum number of completed shots required for a meaningful report.
    pub minimum_shots: u64,

    /// Confidence level for the Wilson interval.
    pub confidence_level: f64,

    /// Stop once this many logical failures have been observed.
    ///
    /// This is an early-stop condition and therefore never produces a
    /// complete experiment unless it coincides with the requested shot count.
    pub target_failures: Option<u64>,

    /// Optional execution timeout in milliseconds.
    ///
    /// The canonical QEC configuration remains authoritative for global
    /// execution policy.
    pub max_duration_ms: Option<u64>,

    /// Whether to retain individual shot classifications.
    pub retain_outcomes: bool,

    /// Maximum number of classifications retained.
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
            max_duration_ms: None,
            retain_outcomes: false,
            max_retained_outcomes: 0,
        }
    }
}

impl SimulationOptions {
    /// Validates the request independently of `QecConfig`.
    pub fn validate(&self) -> SimulationResult<()> {
        if self.shots == 0 {
            return Err(
                SimulationError::InvalidConfiguration(
                    "shots must be greater than zero".to_owned(),
                ),
            );
        }

        if self.shots > MAX_SIMULATION_SHOTS {
            return Err(
                SimulationError::ShotLimitExceeded {
                    requested: self.shots,
                    limit: MAX_SIMULATION_SHOTS,
                },
            );
        }

        if self.minimum_shots == 0 {
            return Err(
                SimulationError::InvalidStatistics(
                    "minimum_shots must be greater than zero"
                        .to_owned(),
                ),
            );
        }

        if self.minimum_shots > self.shots {
            return Err(
                SimulationError::InvalidStatistics(
                    "minimum_shots cannot exceed shots"
                        .to_owned(),
                ),
            );
        }

        if !self.confidence_level.is_finite()
            || self.confidence_level <= 0.0
            || self.confidence_level >= 1.0
        {
            return Err(
                SimulationError::InvalidStatistics(
                    "confidence_level must be finite and in (0, 1)"
                        .to_owned(),
                ),
            );
        }

        if let Some(target) = self.target_failures {
            if target == 0 {
                return Err(
                    SimulationError::InvalidStatistics(
                        "target_failures must be greater than zero"
                            .to_owned(),
                    ),
                );
            }
        }

        if let Some(timeout) = self.max_duration_ms {
            if timeout == 0 {
                return Err(
                    SimulationError::InvalidConfiguration(
                        "max_duration_ms must be greater than zero"
                            .to_owned(),
                    ),
                );
            }
        }

        if self.retain_outcomes
            && self.max_retained_outcomes == 0
        {
            return Err(
                SimulationError::InvalidConfiguration(
                    "retain_outcomes requires a positive \
                     max_retained_outcomes"
                        .to_owned(),
                ),
            );
        }

        if self.max_retained_outcomes > MAX_RETAINED_OUTCOMES {
            return Err(
                SimulationError::InvalidConfiguration(
                    "max_retained_outcomes exceeds the \
                     simulation safety ceiling"
                        .to_owned(),
                ),
            );
        }

        if !self.retain_outcomes
            && self.max_retained_outcomes != 0
        {
            return Err(
                SimulationError::InvalidConfiguration(
                    "max_retained_outcomes must be zero when \
                     retain_outcomes is disabled"
                        .to_owned(),
                ),
            );
        }

        Ok(())
    }

    /// Returns the effective execution timeout.
    pub fn timeout(&self) -> Option<Duration> {
        self.max_duration_ms.map(Duration::from_millis)
    }

    /// Derives the deterministic seed for one shot.
    ///
    /// The mixer is intentionally not cryptographic. Its purpose is stable
    /// separation of simulation trials.
    pub fn shot_seed(&self, shot_index: u64) -> u64 {
        splitmix64(
            self.seed.wrapping_add(
                shot_index.wrapping_mul(
                    0x9E37_79B9_7F4A_7C15,
                ),
            ),
        )
    }
}

/// Streaming counters for a simulation.
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct SimulationCounts {
    /// Requested number of shots.
    pub requested_shots: u64,

    /// Number of successfully returned shot outcomes.
    pub completed_shots: u64,

    /// Successful logical classifications.
    pub successful_shots: u64,

    /// Logical failures.
    pub logical_failures: u64,

    /// Unknown logical classifications.
    pub unknown_shots: u64,

    /// Total physical faults reported by runners.
    pub physical_faults: u64,

    /// Total detection events reported by runners.
    pub detection_events: u64,

    /// Total correction operations reported by runners.
    pub corrections: u64,
}

impl SimulationCounts {
    /// Records one completed shot using checked arithmetic.
    pub fn record(
        &mut self,
        outcome: ShotOutcome,
    ) -> SimulationResult<()> {
        outcome.validate()?;

        self.completed_shots = self
            .completed_shots
            .checked_add(1)
            .ok_or(
                SimulationError::ArithmeticOverflow,
            )?;

        self.physical_faults = self
            .physical_faults
            .checked_add(
                outcome.physical_fault_count,
            )
            .ok_or(
                SimulationError::ArithmeticOverflow,
            )?;

        self.detection_events = self
            .detection_events
            .checked_add(
                outcome.detection_event_count,
            )
            .ok_or(
                SimulationError::ArithmeticOverflow,
            )?;

        self.corrections = self
            .corrections
            .checked_add(
                outcome.correction_count,
            )
            .ok_or(
                SimulationError::ArithmeticOverflow,
            )?;

        match outcome.classification {
            ShotClassification::Success => {
                self.successful_shots = self
                    .successful_shots
                    .checked_add(1)
                    .ok_or(
                        SimulationError::ArithmeticOverflow,
                    )?;
            }

            ShotClassification::LogicalFailure => {
                self.logical_failures = self
                    .logical_failures
                    .checked_add(1)
                    .ok_or(
                        SimulationError::ArithmeticOverflow,
                    )?;
            }

            ShotClassification::Unknown => {
                self.unknown_shots = self
                    .unknown_shots
                    .checked_add(1)
                    .ok_or(
                        SimulationError::ArithmeticOverflow,
                    )?;
            }
        }

        Ok(())
    }

    /// Number of shots with a valid logical classification.
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

    /// Physical-fault count per completed shot.
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
/// This is deliberately a small report-level primitive. Advanced statistical
/// analysis belongs to `statistical.rs`, which can consume this representation
/// without requiring changes to the simulation execution API.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct ConfidenceInterval {
    /// Confidence level.
    pub confidence_level: f64,

    /// Point estimate.
    pub estimate: f64,

    /// Lower bound.
    pub lower: f64,

    /// Upper bound.
    pub upper: f64,
}

impl ConfidenceInterval {
    /// Computes a Wilson interval for `successes / trials`.
    pub fn wilson(
        successes: u64,
        trials: u64,
        confidence_level: f64,
    ) -> SimulationResult<Self> {
        if trials == 0 || successes > trials {
            return Err(
                SimulationError::InvalidStatisticalSample,
            );
        }

        if !confidence_level.is_finite()
            || confidence_level <= 0.0
            || confidence_level >= 1.0
        {
            return Err(
                SimulationError::InvalidStatistics(
                    "confidence level must be in (0, 1)"
                        .to_owned(),
                ),
            );
        }

        let p =
            successes as f64 / trials as f64;

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
                * (
                    (p * (1.0 - p) / n)
                        + z2 / (4.0 * n * n)
                )
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

/// Measured resource usage of one simulation.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct SimulationResourceUsage {
    /// Number of runner invocations.
    pub shot_executions: u64,

    /// Total reported physical faults.
    pub physical_faults: u64,

    /// Total reported detection events.
    pub detection_events: u64,

    /// Total reported correction operations.
    pub corrections: u64,

    /// Peak number of retained outcomes.
    pub peak_retained_outcomes: u64,

    /// Wall-clock execution time in nanoseconds.
    pub elapsed_nanos: u64,
}

impl SimulationResourceUsage {
    fn from_counts(
        counts: &SimulationCounts,
        peak_retained_outcomes: u64,
        elapsed_nanos: u64,
    ) -> Self {
        Self {
            shot_executions: counts.completed_shots,
            physical_faults: counts.physical_faults,
            detection_events: counts.detection_events,
            corrections: counts.corrections,
            peak_retained_outcomes,
            elapsed_nanos,
        }
    }
}

/// Why a simulation stopped.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub enum TerminationReason {
    /// All requested shots completed.
    Completed,

    /// The configured logical-failure target was reached.
    TargetFailuresReached,

    /// Cancellation was requested.
    Cancelled,

    /// The execution time budget expired.
    TimeLimitExceeded,
}

/// Complete simulation report.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct SimulationReport {
    /// Simulation schema version.
    pub schema_version: u32,

    /// QEC configuration schema version.
    pub configuration_schema_version: u32,

    /// Base simulation seed.
    pub seed: u64,

    /// Whether deterministic execution was required.
    pub deterministic: bool,

    /// Simulation counters.
    pub counts: SimulationCounts,

    /// Logical-error confidence interval.
    pub logical_error_interval: Option<ConfidenceInterval>,

    /// Physical fault rate.
    pub physical_fault_rate: Option<f64>,

    /// Execution resource usage.
    pub resource_usage: SimulationResourceUsage,

    /// Why execution terminated.
    pub termination: TerminationReason,

    /// True only when every requested shot completed.
    pub complete: bool,

    /// True when enough classified data exists for the configured minimum.
    pub statistically_meaningful: bool,

    /// Retained classifications, if requested.
    pub retained_outcomes: Vec<ShotClassification>,
}

impl SimulationReport {
    /// Returns the logical error rate.
    pub fn logical_error_rate(&self) -> Option<f64> {
        self.counts.logical_error_rate()
    }

    /// Returns whether the experiment is fully complete.
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Returns whether the report is safe to use as a complete statistical
    /// experiment under its configured minimum-shot policy.
    pub fn is_statistically_complete(&self) -> bool {
        self.complete
            && self.statistically_meaningful
            && self.counts.unknown_shots == 0
    }
}

/// Production simulation engine.
///
/// The engine is intentionally single-runner and streaming.
///
/// Parallel simulation belongs to `scheduler.rs` / `distributed.rs`, where
/// worker limits, deterministic reductions and resource admission can be
/// enforced centrally.
pub struct SimulationEngine<'a> {
    config: &'a QecConfig,
    options: SimulationOptions,
}

impl<'a> SimulationEngine<'a> {
    /// Creates a validated simulation engine.
    pub fn new(
        config: &'a QecConfig,
        options: SimulationOptions,
    ) -> QecResult<Self> {
        config
            .validate()
            .map_err(|error| {
                QecError::invalid_input(
                    error.to_string(),
                )
            })?;

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

    /// Performs all simulation preflight before executing a shot.
    fn preflight(
        config: &QecConfig,
        options: &SimulationOptions,
    ) -> QecResult<()> {
        if config.limits.max_parallelism == 0 {
            return Err(QecError::resource_limit(
                ResourceKind::Parallelism,
                1,
                0,
                "simulation requires one execution slot",
            ));
        }

        if config.requires_determinism()
            && !options.deterministic
        {
            return Err(QecError::invalid_input(
                "QecConfig requires deterministic execution \
                 but SimulationOptions disabled it",
            ));
        }

        // QPU shot limits are authoritative when the configured execution
        // backend is QPU. Classical simulation requests use the explicit
        // simulation API safety ceiling above; this avoids incorrectly
        // treating a QPU-specific policy field as the simulation policy.
        if config.requires_qpu()
            && options.shots
                > config.limits.max_qpu_shots
        {
            return Err(QecError::resource_limit(
                ResourceKind::QpuShots,
                u128::from(options.shots),
                u128::from(
                    config.limits.max_qpu_shots,
                ),
                "simulation QPU-shot request exceeds QEC limits",
            ));
        }

        if options.retain_outcomes {
            let retained =
                options
                    .max_retained_outcomes
                    .min(options.shots);

            let bytes_per_outcome =
                u64::try_from(
                    size_of::<ShotClassification>(),
                )
                .unwrap_or(u64::MAX);

            let requested_bytes =
                retained
                    .checked_mul(bytes_per_outcome)
                    .ok_or_else(|| {
                        QecError::numerical_failure(
                            super::errors::NumericalOperation::MemorySize,
                            "simulation retention-size calculation overflow",
                        )
                    })?;

            if requested_bytes
                > config.limits.max_memory_bytes
            {
                return Err(
                    SimulationError::RetentionMemoryExceeded {
                        requested_bytes,
                        maximum_bytes:
                            config
                                .limits
                                .max_memory_bytes,
                    }
                    .into(),
                );
            }
        }

        Ok(())
    }

    /// Executes the simulation without external cancellation.
    pub fn run<R>(
        &self,
        runner: &mut R,
    ) -> QecResult<SimulationReport>
    where
        R: ShotRunner,
    {
        let token =
            CancellationToken::new();

        self.run_with_token(
            runner,
            &token,
        )
    }

    /// Executes the simulation with the canonical QEC cancellation token.
    pub fn run_with_token<R>(
        &self,
        runner: &mut R,
        cancellation: &CancellationToken,
    ) -> QecResult<SimulationReport>
    where
        R: ShotRunner,
    {
        let started =
            Instant::now();

        let configured_timeout =
            self.options.timeout();

        let global_timeout =
            if self.config.limits.max_decoder_time_ns > 0
            {
                Some(Duration::from_nanos(
                    self.config
                        .limits
                        .max_decoder_time_ns,
                ))
            } else {
                None
            };

        let effective_timeout =
            match (
                configured_timeout,
                global_timeout,
            ) {
                (Some(local), Some(global)) => {
                    Some(local.min(global))
                }

                (Some(local), None) => {
                    Some(local)
                }

                (None, Some(global)) => {
                    Some(global)
                }

                (None, None) => None,
            };

        let mut counts =
            SimulationCounts {
                requested_shots:
                    self.options.shots,
                ..SimulationCounts::default()
            };

        let mut retained =
            Vec::<ShotClassification>::new();

        let retention_capacity =
            if self.options.retain_outcomes {
                self.options
                    .max_retained_outcomes
                    .min(self.options.shots)
            } else {
                0
            };

        if retention_capacity > 0 {
            retained
                .try_reserve_exact(
                    usize::try_from(
                        retention_capacity,
                    )
                    .unwrap_or(usize::MAX),
                )
                .map_err(|_| {
                    SimulationError::RetentionMemoryExceeded {
                        requested_bytes:
                            retention_capacity
                                .saturating_mul(
                                    u64::try_from(
                                        size_of::<
                                            ShotClassification
                                        >(),
                                    )
                                    .unwrap_or(u64::MAX),
                                ),
                        maximum_bytes:
                            self.config
                                .limits
                                .max_memory_bytes,
                    }
                })?;
        }

        let mut termination =
            TerminationReason::Completed;

        for shot_index in 0..self.options.shots {
            cancellation
                .check()
                .map_err(|_| {
                    termination =
                        TerminationReason::Cancelled;

                    SimulationError::Cancelled
                })
                .ok();

            if cancellation.is_cancelled() {
                termination =
                    TerminationReason::Cancelled;
                break;
            }

            if let Some(timeout) =
                effective_timeout
            {
                if started.elapsed() >= timeout {
                    termination =
                        TerminationReason::TimeLimitExceeded;
                    break;
                }
            }

            let seed =
                self.options
                    .shot_seed(shot_index);

            let outcome =
                runner.run_shot(
                    shot_index,
                    seed,
                )?;

            outcome
                .validate()
                .map_err(QecError::from)?;

            counts
                .record(outcome)
                .map_err(QecError::from)?;

            if retained.len()
                < retention_capacity
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

        let statistically_meaningful =
            counts.completed_shots
                >= self.options.minimum_shots
                && counts.classified_shots() > 0
                && counts.unknown_shots == 0;

        let logical_error_interval =
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

        let resource_usage =
            SimulationResourceUsage::from_counts(
                &counts,
                retained.len() as u64,
                elapsed_nanos,
            );

        Ok(SimulationReport {
            schema_version:
                SIMULATION_SCHEMA_VERSION,

            configuration_schema_version:
                self.config.schema_version(),

            seed:
                self.options.seed,

            deterministic:
                self.options.deterministic,

            counts,

            logical_error_interval,

            physical_fault_rate:
                resource_usage
                    .shot_executions
                    .checked_sub(0)
                    .and_then(|_| {
                        if resource_usage
                            .shot_executions
                            == 0
                        {
                            None
                        } else {
                            Some(
                                resource_usage
                                    .physical_faults
                                    as f64
                                    / resource_usage
                                        .shot_executions
                                        as f64,
                            )
                        }
                    }),

            resource_usage,

            termination,

            complete,

            statistically_meaningful,

            retained_outcomes:
                retained,
        })
    }

    /// Compatibility convenience wrapper using a callback.
    ///
    /// The canonical implementation remains `run_with_token`.
    pub fn run_with_cancellation<R, C>(
        &self,
        runner: &mut R,
        cancellation: C,
    ) -> QecResult<SimulationReport>
    where
        R: ShotRunner,
        C: Fn() -> bool,
    {
        if cancellation() {
            return Ok(SimulationReport {
                schema_version:
                    SIMULATION_SCHEMA_VERSION,
                configuration_schema_version:
                    self.config.schema_version(),
                seed:
                    self.options.seed,
                deterministic:
                    self.options.deterministic,
                counts:
                    SimulationCounts {
                        requested_shots:
                            self.options.shots,
                        ..SimulationCounts::default()
                    },
                logical_error_interval:
                    None,
                physical_fault_rate:
                    None,
                resource_usage:
                    SimulationResourceUsage::default(),
                termination:
                    TerminationReason::Cancelled,
                complete:
                    false,
                statistically_meaningful:
                    false,
                retained_outcomes:
                    Vec::new(),
            });
        }

        let source =
            super::cancellation::CancellationSource::new();

        let token =
            source.token();

        let cancelled =
            cancellation();

        if cancelled {
            source.cancel();
        }

        self.run_with_token(
            runner,
            &token,
        )
    }
}

/// Convenience simulation function.
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

/// Convenience simulation function with canonical cancellation.
pub fn simulate_with_token<R>(
    config: &QecConfig,
    options: SimulationOptions,
    runner: &mut R,
    cancellation: &CancellationToken,
) -> QecResult<SimulationReport>
where
    R: ShotRunner,
{
    SimulationEngine::new(
        config,
        options,
    )?
    .run_with_token(
        runner,
        cancellation,
    )
}

/// Convenience simulation function retaining the callback API.
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

/// Stable deterministic 64-bit mixer.
fn splitmix64(
    mut value: u64,
) -> u64 {
    value = value.wrapping_add(
        0x9E37_79B9_7F4A_7C15,
    );

    let mut z = value;

    z = (z ^ (z >> 30))
        .wrapping_mul(
            0xBF58_476D_1CE4_E5B9,
        );

    z = (z ^ (z >> 27))
        .wrapping_mul(
            0x94D0_49BB_1331_11EB,
        );

    z ^ (z >> 31)
}

/// Approximation to the standard-normal inverse CDF.
///
/// Domain:
///
/// `0 < p < 1`
fn normal_quantile(
    p: f64,
) -> f64 {
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
                + 1.0;
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
                + 1.0);
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
                Ok(
                    ShotOutcome::logical_failure(),
                )
            } else {
                Ok(
                    ShotOutcome::success(),
                )
            }
        }
    }

    #[test]
    fn deterministic_seeds_are_stable() {
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
    fn counts_are_checked_and_correct() {
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
    fn engine_completes_requested_shots() {
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
            report.complete
        );

        assert!(
            report.is_statistically_complete()
        );
    }

    #[test]
    fn target_failure_stop_is_not_complete() {
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
            !report.complete
        );

        assert!(
            !report.is_statistically_complete()
        );
    }

    #[test]
    fn retention_is_bounded() {
        let config =
            QecConfig::deterministic_test();

        let options =
            SimulationOptions {
                shots: 20,
                minimum_shots: 1,
                retain_outcomes: true,
                max_retained_outcomes: 5,
                ..SimulationOptions::default()
            };

        let mut runner =
            FixedRunner {
                failures: 0,
            };

        let report =
            simulate(
                &config,
                options,
                &mut runner,
            )
            .unwrap();

        assert_eq!(
            report.retained_outcomes.len(),
            5
        );

        assert_eq!(
            report.resource_usage
                .peak_retained_outcomes,
            5
        );
    }

    #[test]
    fn zero_confidence_is_rejected() {
        let options =
            SimulationOptions {
                confidence_level: 0.0,
                ..SimulationOptions::default()
            };

        assert!(
            options.validate().is_err()
        );
    }
}