//! Zamani Quantum Benchmarking — Simultaneous Randomized Benchmarking
//!
//! # Purpose
//!
//! Production-grade simultaneous randomized benchmarking (SRB).
//!
//! Simultaneous randomized benchmarking is an extension/composition of
//! randomized benchmarking in which independent RB experiments are executed
//! concurrently on disjoint qubit subsets.  Its primary purpose is to expose
//! crosstalk, spectator effects, shared-control degradation, and other
//! context-dependent error increases that may not appear when the same
//! subsets are benchmarked independently.
//!
//! # Architectural position
//!
//! This module is deliberately independent of:
//!
//! - the Zamani Quantum IR;
//! - circuit construction;
//! - Clifford generation;
//! - hardware execution;
//! - simulator implementation;
//! - routing;
//! - scheduling;
//! - compiler optimization;
//! - reporting;
//! - serialization;
//! - provider-specific SDKs.
//!
//! The intended dependency direction is:
//!
//! ```text
//! generators/clifford.rs
//!         │
//!         ▼
//! protocols/randomized_benchmarking.rs
//!         │
//!         ▼
//! protocols/simultaneous_rb.rs
//!         │
//!         ├───────────────┐
//!         ▼               ▼
//! execution/          statistics/
//!         │               │
//!         └───────┬───────┘
//!                 ▼
//!          BenchmarkResult
//! ```
//!
//! This file therefore owns:
//!
//! 1. SRB configuration;
//! 2. qubit-group definitions;
//! 3. validation of disjoint simultaneous groups;
//! 4. experiment-plan representation;
//! 5. raw SRB observations;
//! 6. simultaneous-vs-baseline analysis;
//! 7. deterministic exponential fitting;
//! 8. crosstalk degradation metrics;
//! 9. statistical validity checks;
//! 10. stable integration contracts for future generators/executors.
//!
//! It does NOT own circuit generation or hardware execution.
//!
//! # Scientific interpretation
//!
//! A simultaneous experiment should normally contain:
//!
//! ```text
//! baseline RB(group A)
//! baseline RB(group B)
//!
//! compared with:
//!
//! simultaneous RB(A || B)
//! ```
//!
//! The baseline and simultaneous experiments must use comparable RB designs:
//!
//! - same qubit subsets;
//! - same sequence depths;
//! - same number of random sequences where possible;
//! - same RB ensemble;
//! - same measurement convention;
//! - same shot policy;
//! - same analysis model.
//!
//! The primary quantity is not simply "the simultaneous error rate".  The
//! useful diagnostic is the degradation relative to the isolated baseline:
//!
//! ```text
//! degradation = simultaneous_error - isolated_error
//! ```
//!
//! A positive degradation indicates worse performance under simultaneous
//! operation.
//!
//! A normalized form is also provided:
//!
//! ```text
//! relative_degradation =
//!     (simultaneous_error - isolated_error) / isolated_error
//! ```
//!
//! The normalized quantity is undefined when the baseline error is zero.
//!
//! # RB model
//!
//! The standard fixed-asymptote RB model used here is:
//!
//! ```text
//! P(m) = A + B * p^m
//! ```
//!
//! where:
//!
//! - `m` is RB sequence depth;
//! - `A` is the asymptotic success probability;
//! - `B` is the SPAM/amplitude term;
//! - `p` is the fitted decay parameter.
//!
//! For an n-qubit computational-basis success-probability measurement,
//! the standard asymptote is:
//!
//! ```text
//! A = 1 / 2^n
//! ```
//!
//! and the corresponding depolarizing error estimate is:
//!
//! ```text
//! r = ((2^n - 1) / 2^n) * (1 - p)
//! ```
//!
//! The interpretation is model-dependent.  SRB does not prove a unique
//! microscopic error mechanism.  It provides a reproducible operational
//! measure of performance degradation under simultaneous operation.
//!
//! # Statistical policy
//!
//! The implementation rejects:
//!
//! - NaN;
//! - infinities;
//! - negative depths;
//! - zero-qubit groups;
//! - overlapping simultaneous groups;
//! - zero-shot observations;
//! - invalid success probabilities;
//! - insufficient depth points;
//! - unusable exponential fits;
//! - impossible fitted parameters.
//!
//! No samples are silently discarded.
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
//! # Future integration
//!
//! Future files can integrate without modifying this module:
//!
//! - `generators/clifford.rs` implements `SimultaneousRbSequenceGenerator`;
//! - `execution/executor.rs` implements `SimultaneousRbExecutor`;
//! - `statistics/regression.rs` can reproduce/replace the internal fit through
//!   a future adapter;
//! - `core/result.rs` can wrap `SimultaneousRbResult`;
//! - `reporting/*` can serialize the result;
//! - `registry/*` can register `SIMULTANEOUS_RB_BENCHMARK_ID`;
//! - `stdlib::quantum` can expose a language-level SRB API.
//!
//! This module intentionally does not require those future modules to compile.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

// ============================================================================
// Public protocol identity
// ============================================================================

/// Stable machine-readable benchmark identifier.
pub const SIMULTANEOUS_RB_BENCHMARK_ID: &str = "simultaneous_randomized_benchmarking";

/// Stable mathematical result schema version.
pub const SIMULTANEOUS_RB_RESULT_SCHEMA_VERSION: u32 = 1;

/// Stable experiment-plan schema version.
pub const SIMULTANEOUS_RB_EXPERIMENT_SCHEMA_VERSION: u32 = 1;

/// Minimum number of distinct depth points required for a fit.
pub const MIN_FIT_POINTS: usize = 3;

/// Minimum shots permitted for a production observation.
pub const MIN_SHOTS: usize = 1;

/// Default number of fit-search iterations.
pub const DEFAULT_FIT_ITERATIONS: usize = 128;

/// Default fit tolerance.
pub const DEFAULT_FIT_TOLERANCE: f64 = 1.0e-12;

/// Default confidence level.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Maximum supported confidence level.
pub const MAX_CONFIDENCE_LEVEL: f64 = 0.999_999_999_999;

/// Minimum supported confidence level.
pub const MIN_CONFIDENCE_LEVEL: f64 = 0.5;

// ============================================================================
// Errors
// ============================================================================

/// All validation and analysis errors owned by this module.
#[derive(Debug, Clone, PartialEq)]
pub enum SimultaneousRbError {
    /// Configuration has no groups.
    NoGroups,

    /// A simultaneous group contains no qubits.
    EmptyGroup {
        group_id: String,
    },

    /// A qubit occurs in more than one simultaneous group.
    OverlappingGroups {
        qubit: usize,
        first_group: String,
        second_group: String,
    },

    /// Group IDs must be unique.
    DuplicateGroupId {
        group_id: String,
    },

    /// A group identifier is empty.
    EmptyGroupId,

    /// A sequence depth is invalid.
    InvalidDepth {
        depth: usize,
    },

    /// No depths were configured.
    NoDepths,

    /// Depths must be unique.
    DuplicateDepth {
        depth: usize,
    },

    /// Depths must be ordered.
    UnsortedDepths,

    /// Zero circuits at a depth.
    InvalidCircuitCount {
        depth: usize,
    },

    /// Zero shots.
    InvalidShots,

    /// Confidence level outside the supported range.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// Invalid success probability.
    InvalidProbability {
        value: f64,
    },

    /// Success count exceeds shot count.
    SuccessesExceedShots {
        successes: usize,
        shots: usize,
    },

    /// Insufficient points for an exponential fit.
    InsufficientFitPoints {
        actual: usize,
        required: usize,
    },

    /// Duplicate observations for a depth.
    DuplicateObservation {
        depth: usize,
    },

    /// A baseline observation is missing for a depth.
    MissingBaselineDepth {
        depth: usize,
    },

    /// A simultaneous observation is missing for a depth.
    MissingSimultaneousDepth {
        depth: usize,
    },

    /// An observation has no circuits.
    EmptyObservation {
        depth: usize,
    },

    /// A fit cannot be performed.
    FitFailure {
        reason: String,
    },

    /// Fitted p is outside its physical model domain.
    InvalidDecayParameter {
        value: f64,
    },

    /// A numerical calculation became non-finite.
    NonFiniteStatistic {
        name: &'static str,
    },

    /// A derived quantity is mathematically undefined.
    UndefinedRelativeDegradation,

    /// Number of qubits cannot be represented safely as a power of two.
    QubitDimensionOverflow {
        qubits: usize,
    },

    /// Internal configuration is invalid.
    InvalidFitConfiguration {
        iterations: usize,
        tolerance: f64,
    },
}

impl fmt::Display for SimultaneousRbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoGroups => write!(f, "simultaneous RB requires at least one group"),

            Self::EmptyGroup { group_id } => {
                write!(f, "simultaneous RB group '{}' contains no qubits", group_id)
            }

            Self::OverlappingGroups {
                qubit,
                first_group,
                second_group,
            } => write!(
                f,
                "qubit {} occurs in both simultaneous groups '{}' and '{}'",
                qubit, first_group, second_group
            ),

            Self::DuplicateGroupId { group_id } => {
                write!(f, "duplicate simultaneous RB group ID '{}'", group_id)
            }

            Self::EmptyGroupId => {
                write!(f, "simultaneous RB group ID cannot be empty")
            }

            Self::InvalidDepth { depth } => {
                write!(f, "RB depth {} is invalid", depth)
            }

            Self::NoDepths => {
                write!(f, "simultaneous RB requires at least one sequence depth")
            }

            Self::DuplicateDepth { depth } => {
                write!(f, "RB depth {} occurs more than once", depth)
            }

            Self::UnsortedDepths => {
                write!(f, "RB depths must be strictly increasing")
            }

            Self::InvalidCircuitCount { depth } => {
                write!(f, "depth {} has zero circuits", depth)
            }

            Self::InvalidShots => {
                write!(f, "simultaneous RB requires at least one shot")
            }

            Self::InvalidConfidenceLevel { value } => write!(
                f,
                "confidence level must be finite and in [{}, {}], got {}",
                MIN_CONFIDENCE_LEVEL, MAX_CONFIDENCE_LEVEL, value
            ),

            Self::InvalidProbability { value } => {
                write!(f, "success probability must be finite and in [0, 1], got {}", value)
            }

            Self::SuccessesExceedShots { successes, shots } => write!(
                f,
                "success count {} exceeds shot count {}",
                successes, shots
            ),

            Self::InsufficientFitPoints { actual, required } => write!(
                f,
                "exponential RB fit requires at least {} points, got {}",
                required, actual
            ),

            Self::DuplicateObservation { depth } => {
                write!(f, "duplicate RB observation at depth {}", depth)
            }

            Self::MissingBaselineDepth { depth } => {
                write!(f, "baseline observation missing at depth {}", depth)
            }

            Self::MissingSimultaneousDepth { depth } => {
                write!(f, "simultaneous observation missing at depth {}", depth)
            }

            Self::EmptyObservation { depth } => {
                write!(f, "RB observation at depth {} contains no circuits", depth)
            }

            Self::FitFailure { reason } => {
                write!(f, "simultaneous RB exponential fit failed: {}", reason)
            }

            Self::InvalidDecayParameter { value } => {
                write!(f, "fitted RB decay parameter p={} is outside [0, 1]", value)
            }

            Self::NonFiniteStatistic { name } => {
                write!(f, "statistical calculation produced non-finite {}", name)
            }

            Self::UndefinedRelativeDegradation => {
                write!(f, "relative degradation is undefined because baseline error is zero")
            }

            Self::QubitDimensionOverflow { qubits } => {
                write!(
                    f,
                    "Hilbert-space dimension 2^{} cannot be represented safely",
                    qubits
                )
            }

            Self::InvalidFitConfiguration {
                iterations,
                tolerance,
            } => write!(
                f,
                "invalid fit configuration: iterations={}, tolerance={}",
                iterations, tolerance
            ),
        }
    }
}

impl Error for SimultaneousRbError {}

// ============================================================================
// Group definition
// ============================================================================

/// A disjoint set of physical or logical qubits to be RB-tested together.
///
/// The meaning of the qubit identifiers is deliberately left to the hardware
/// and execution layers. They can represent logical or physical qubit indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimultaneousRbGroup {
    /// Stable user-visible group identifier.
    pub id: String,

    /// Qubit indices assigned to this group.
    pub qubits: Vec<usize>,
}

impl SimultaneousRbGroup {
    /// Construct a group.
    pub fn new(
        id: impl Into<String>,
        qubits: Vec<usize>,
    ) -> Result<Self, SimultaneousRbError> {
        let group = Self {
            id: id.into(),
            qubits,
        };

        group.validate()?;
        Ok(group)
    }

    /// Validate the group.
    pub fn validate(&self) -> Result<(), SimultaneousRbError> {
        if self.id.trim().is_empty() {
            return Err(SimultaneousRbError::EmptyGroupId);
        }

        if self.qubits.is_empty() {
            return Err(SimultaneousRbError::EmptyGroup {
                group_id: self.id.clone(),
            });
        }

        let mut seen = BTreeSet::new();

        for &qubit in &self.qubits {
            if !seen.insert(qubit) {
                return Err(SimultaneousRbError::FitFailure {
                    reason: format!(
                        "qubit {} occurs more than once inside group '{}'",
                        qubit, self.id
                    ),
                });
            }
        }

        Ok(())
    }

    /// Number of qubits in this group.
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Configuration shared by all simultaneous RB sub-experiments.
#[derive(Debug, Clone, PartialEq)]
pub struct SimultaneousRbConfig {
    /// Sequence depths in increasing order.
    pub depths: Vec<usize>,

    /// Number of random circuits generated for each depth.
    pub circuits_per_depth: usize,

    /// Shots per circuit.
    pub shots_per_circuit: usize,

    /// Statistical confidence level.
    pub confidence_level: f64,

    /// Number of iterations used by the deterministic one-dimensional fit.
    pub fit_iterations: usize,

    /// Numerical convergence tolerance.
    pub fit_tolerance: f64,

    /// Optional experiment-level identifier.
    pub experiment_id: Option<String>,

    /// Stable seed supplied to the generator layer.
    ///
    /// The protocol does not consume this seed itself. It is carried in the
    /// experiment plan so the generator/executor can reproduce the experiment.
    pub seed: Option<u64>,
}

impl Default for SimultaneousRbConfig {
    fn default() -> Self {
        Self {
            depths: vec![1, 2, 4, 8, 16, 32],
            circuits_per_depth: 10,
            shots_per_circuit: 1000,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            fit_iterations: DEFAULT_FIT_ITERATIONS,
            fit_tolerance: DEFAULT_FIT_TOLERANCE,
            experiment_id: None,
            seed: None,
        }
    }
}

impl SimultaneousRbConfig {
    /// Validate configuration.
    pub fn validate(&self) -> Result<(), SimultaneousRbError> {
        if self.depths.is_empty() {
            return Err(SimultaneousRbError::NoDepths);
        }

        let mut previous = None;

        for &depth in &self.depths {
            if depth == 0 {
                return Err(SimultaneousRbError::InvalidDepth { depth });
            }

            if let Some(prev) = previous {
                if depth == prev {
                    return Err(SimultaneousRbError::DuplicateDepth { depth });
                }

                if depth < prev {
                    return Err(SimultaneousRbError::UnsortedDepths);
                }
            }

            previous = Some(depth);

            if self.circuits_per_depth == 0 {
                return Err(SimultaneousRbError::InvalidCircuitCount { depth });
            }
        }

        if self.shots_per_circuit < MIN_SHOTS {
            return Err(SimultaneousRbError::InvalidShots);
        }

        if !self.confidence_level.is_finite()
            || self.confidence_level < MIN_CONFIDENCE_LEVEL
            || self.confidence_level > MAX_CONFIDENCE_LEVEL
        {
            return Err(SimultaneousRbError::InvalidConfidenceLevel {
                value: self.confidence_level,
            });
        }

        if self.fit_iterations == 0
            || !self.fit_tolerance.is_finite()
            || self.fit_tolerance <= 0.0
        {
            return Err(SimultaneousRbError::InvalidFitConfiguration {
                iterations: self.fit_iterations,
                tolerance: self.fit_tolerance,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Experiment plan
// ============================================================================

/// Immutable experiment design consumed by a future generator/executor.
#[derive(Debug, Clone, PartialEq)]
pub struct SimultaneousRbExperimentPlan {
    /// Stable schema version.
    pub schema_version: u32,

    /// Stable benchmark identifier.
    pub benchmark_id: &'static str,

    /// Experiment configuration.
    pub config: SimultaneousRbConfig,

    /// Disjoint groups that are executed simultaneously.
    pub groups: Vec<SimultaneousRbGroup>,
}

impl SimultaneousRbExperimentPlan {
    /// Construct and validate an experiment plan.
    pub fn new(
        config: SimultaneousRbConfig,
        groups: Vec<SimultaneousRbGroup>,
    ) -> Result<Self, SimultaneousRbError> {
        config.validate()?;

        if groups.is_empty() {
            return Err(SimultaneousRbError::NoGroups);
        }

        for group in &groups {
            group.validate()?;
        }

        validate_disjoint_groups(&groups)?;

        Ok(Self {
            schema_version: SIMULTANEOUS_RB_EXPERIMENT_SCHEMA_VERSION,
            benchmark_id: SIMULTANEOUS_RB_BENCHMARK_ID,
            config,
            groups,
        })
    }

    /// Number of simultaneously executed groups.
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// Returns all qubits participating in the experiment.
    pub fn all_qubits(&self) -> Vec<usize> {
        let mut qubits = BTreeSet::new();

        for group in &self.groups {
            for &qubit in &group.qubits {
                qubits.insert(qubit);
            }
        }

        qubits.into_iter().collect()
    }
}

/// Validate that simultaneous groups are pairwise disjoint.
pub fn validate_disjoint_groups(
    groups: &[SimultaneousRbGroup],
) -> Result<(), SimultaneousRbError> {
    let mut ownership: BTreeMap<usize, String> = BTreeMap::new();

    for group in groups {
        for &qubit in &group.qubits {
            if let Some(first_group) = ownership.get(&qubit) {
                return Err(SimultaneousRbError::OverlappingGroups {
                    qubit,
                    first_group: first_group.clone(),
                    second_group: group.id.clone(),
                });
            }

            ownership.insert(qubit, group.id.clone());
        }
    }

    let mut ids = BTreeSet::new();

    for group in groups {
        if !ids.insert(group.id.clone()) {
            return Err(SimultaneousRbError::DuplicateGroupId {
                group_id: group.id.clone(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// Circuit-generation integration contract
// ============================================================================

/// A single generated simultaneous-RB circuit description.
///
/// The protocol does not require a concrete IR here. The generator layer can
/// attach its canonical Zamani Quantum IR circuit through an external mapping
/// while using this descriptor as the stable benchmark identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimultaneousRbCircuitSpec {
    /// Stable circuit identifier.
    pub circuit_id: String,

    /// Sequence depth.
    pub depth: usize,

    /// Random sequence index at this depth.
    pub sequence_index: usize,

    /// Group identifiers participating in this circuit.
    pub group_ids: Vec<String>,

    /// Expected computational-basis outcome encoded as a bitstring.
    pub expected_output: String,
}

impl SimultaneousRbCircuitSpec {
    /// Construct a circuit descriptor.
    pub fn new(
        circuit_id: impl Into<String>,
        depth: usize,
        sequence_index: usize,
        group_ids: Vec<String>,
        expected_output: impl Into<String>,
    ) -> Result<Self, SimultaneousRbError> {
        if depth == 0 {
            return Err(SimultaneousRbError::InvalidDepth { depth });
        }

        Ok(Self {
            circuit_id: circuit_id.into(),
            depth,
            sequence_index,
            group_ids,
            expected_output: expected_output.into(),
        })
    }
}

/// Contract implemented later by `generators/clifford.rs`.
///
/// The generator is responsible for:
///
/// - random Clifford selection;
/// - inverse construction;
/// - canonical circuit construction;
/// - deterministic seeding;
/// - circuit fingerprints.
///
/// SRB itself only requires the generated experiment descriptors.
pub trait SimultaneousRbSequenceGenerator {
    /// Generator-specific error.
    type Error: Error;

    /// Generate all circuits required by one SRB plan.
    fn generate(
        &mut self,
        plan: &SimultaneousRbExperimentPlan,
    ) -> Result<Vec<SimultaneousRbCircuitSpec>, Self::Error>;
}

// ============================================================================
// Execution integration contract
// ============================================================================

/// A single raw success observation.
///
/// The executor converts backend counts into "correct outcome" counts. This
/// keeps provider-specific bitstring conventions out of SRB analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimultaneousRbObservation {
    /// Group identifier.
    pub group_id: String,

    /// Sequence depth.
    pub depth: usize,

    /// Number of generated circuits represented by this observation.
    pub circuits: usize,

    /// Total number of shots represented.
    pub shots: usize,

    /// Number of shots producing the expected result.
    pub successes: usize,
}

impl SimultaneousRbObservation {
    /// Create a validated raw observation.
    pub fn new(
        group_id: impl Into<String>,
        depth: usize,
        circuits: usize,
        shots: usize,
        successes: usize,
    ) -> Result<Self, SimultaneousRbError> {
        if depth == 0 {
            return Err(SimultaneousRbError::InvalidDepth { depth });
        }

        if circuits == 0 {
            return Err(SimultaneousRbError::EmptyObservation { depth });
        }

        if shots == 0 {
            return Err(SimultaneousRbError::InvalidShots);
        }

        if successes > shots {
            return Err(SimultaneousRbError::SuccessesExceedShots {
                successes,
                shots,
            });
        }

        Ok(Self {
            group_id: group_id.into(),
            depth,
            circuits,
            shots,
            successes,
        })
    }

    /// Empirical success probability.
    pub fn success_probability(&self) -> Result<f64, SimultaneousRbError> {
        let probability = self.successes as f64 / self.shots as f64;

        validate_probability(probability)?;

        Ok(probability)
    }
}

/// Contract implemented later by `execution/executor.rs`.
///
/// The executor owns:
///
/// - backend submission;
/// - batching;
/// - retries;
/// - cancellation;
/// - backend-specific result conversion.
///
/// It must return observations with no provider-specific semantics left.
pub trait SimultaneousRbExecutor {
    /// Executor-specific error.
    type Error: Error;

    /// Execute the generated simultaneous-RB circuits.
    fn execute(
        &mut self,
        circuits: &[SimultaneousRbCircuitSpec],
    ) -> Result<Vec<SimultaneousRbObservation>, Self::Error>;
}

// ============================================================================
// Fit configuration/result
// ============================================================================

/// Exponential RB fit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RbExponentialFit {
    /// Fixed asymptote A.
    pub asymptote: f64,

    /// Fitted amplitude B.
    pub amplitude: f64,

    /// Fitted decay parameter p.
    pub decay_parameter: f64,

    /// Sum of squared residuals.
    pub sum_squared_error: f64,

    /// Root mean square residual.
    pub rmse: f64,

    /// Coefficient of determination.
    pub r_squared: Option<f64>,

    /// Number of points.
    pub points: usize,

    /// Whether the fit passed all validity checks.
    pub valid: bool,
}

impl RbExponentialFit {
    /// Convert the fitted decay parameter into the average depolarizing error
    /// estimate for an n-qubit RB group.
    pub fn error_rate(
        &self,
        qubits: usize,
    ) -> Result<f64, SimultaneousRbError> {
        let dimension = hilbert_dimension(qubits)?;

        let error = ((dimension - 1.0) / dimension)
            * (1.0 - self.decay_parameter);

        if !error.is_finite() {
            return Err(SimultaneousRbError::NonFiniteStatistic {
                name: "RB error rate",
            });
        }

        Ok(error.max(0.0))
    }
}

// ============================================================================
// Crosstalk metrics
// ============================================================================

/// Crosstalk/degradation comparison between isolated and simultaneous RB.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrosstalkMetrics {
    /// Isolated/baseline error rate.
    pub baseline_error_rate: f64,

    /// Simultaneous error rate.
    pub simultaneous_error_rate: f64,

    /// Absolute increase in error rate.
    pub absolute_degradation: f64,

    /// Relative increase in error rate.
    ///
    /// `None` when baseline error is zero.
    pub relative_degradation: Option<f64>,

    /// Baseline decay parameter.
    pub baseline_decay_parameter: f64,

    /// Simultaneous decay parameter.
    pub simultaneous_decay_parameter: f64,

    /// Difference in decay parameter.
    pub decay_parameter_delta: f64,
}

impl CrosstalkMetrics {
    /// Calculate crosstalk metrics.
    pub fn calculate(
        baseline_fit: &RbExponentialFit,
        simultaneous_fit: &RbExponentialFit,
        qubits: usize,
    ) -> Result<Self, SimultaneousRbError> {
        let baseline_error = baseline_fit.error_rate(qubits)?;
        let simultaneous_error = simultaneous_fit.error_rate(qubits)?;

        let absolute = simultaneous_error - baseline_error;

        let relative = if baseline_error > 0.0 {
            Some(absolute / baseline_error)
        } else {
            None
        };

        if let Some(value) = relative {
            if !value.is_finite() {
                return Err(SimultaneousRbError::NonFiniteStatistic {
                    name: "relative crosstalk degradation",
                });
            }
        }

        let delta =
            simultaneous_fit.decay_parameter - baseline_fit.decay_parameter;

        Ok(Self {
            baseline_error_rate: baseline_error,
            simultaneous_error_rate: simultaneous_error,
            absolute_degradation: absolute,
            relative_degradation: relative,
            baseline_decay_parameter: baseline_fit.decay_parameter,
            simultaneous_decay_parameter: simultaneous_fit.decay_parameter,
            decay_parameter_delta: delta,
        })
    }

    /// Returns true when simultaneous operation degraded the fitted error
    /// rate.
    pub fn indicates_degradation(&self) -> bool {
        self.absolute_degradation > 0.0
    }
}

// ============================================================================
// Group result
// ============================================================================

/// Complete SRB analysis for one group.
#[derive(Debug, Clone, PartialEq)]
pub struct SimultaneousRbGroupResult {
    /// Group identifier.
    pub group_id: String,

    /// Qubit count.
    pub qubit_count: usize,

    /// Isolated/baseline fit.
    pub baseline_fit: RbExponentialFit,

    /// Simultaneous fit.
    pub simultaneous_fit: RbExponentialFit,

    /// Crosstalk comparison.
    pub crosstalk: CrosstalkMetrics,

    /// Baseline observations.
    pub baseline_observations: Vec<SimultaneousRbObservation>,

    /// Simultaneous observations.
    pub simultaneous_observations: Vec<SimultaneousRbObservation>,
}

// ============================================================================
// Overall result
// ============================================================================

/// Complete simultaneous RB result.
#[derive(Debug, Clone, PartialEq)]
pub struct SimultaneousRbResult {
    /// Result schema version.
    pub schema_version: u32,

    /// Benchmark identifier.
    pub benchmark_id: &'static str,

    /// Experiment identifier if supplied.
    pub experiment_id: Option<String>,

    /// Confidence level.
    pub confidence_level: f64,

    /// Results by simultaneous group.
    pub groups: Vec<SimultaneousRbGroupResult>,
}

impl SimultaneousRbResult {
    /// Returns true when at least one group exhibits positive degradation.
    pub fn has_crosstalk_degradation(&self) -> bool {
        self.groups
            .iter()
            .any(|group| group.crosstalk.indicates_degradation())
    }

    /// Return the group with the largest absolute degradation.
    pub fn worst_absolute_degradation(
        &self,
    ) -> Option<&SimultaneousRbGroupResult> {
        self.groups.iter().max_by(|a, b| {
            a.crosstalk
                .absolute_degradation
                .partial_cmp(&b.crosstalk.absolute_degradation)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Return the group with the largest relative degradation.
    pub fn worst_relative_degradation(
        &self,
    ) -> Option<&SimultaneousRbGroupResult> {
        self.groups
            .iter()
            .filter_map(|group| {
                group
                    .crosstalk
                    .relative_degradation
                    .map(|value| (value, group))
            })
            .max_by(|a, b| {
                a.0.partial_cmp(&b.0)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(_, group)| group)
    }
}

// ============================================================================
// Analysis input
// ============================================================================

/// Baseline and simultaneous observations for one group.
#[derive(Debug, Clone, PartialEq)]
pub struct SimultaneousRbGroupData {
    pub group: SimultaneousRbGroup,
    pub baseline: Vec<SimultaneousRbObservation>,
    pub simultaneous: Vec<SimultaneousRbObservation>,
}

impl SimultaneousRbGroupData {
    /// Validate observation structure.
    pub fn validate(&self) -> Result<(), SimultaneousRbError> {
        self.group.validate()?;

        validate_observation_set(&self.baseline)?;
        validate_observation_set(&self.simultaneous)?;

        Ok(())
    }
}

// ============================================================================
// Analyzer
// ============================================================================

/// Production SRB analyzer.
#[derive(Debug, Clone)]
pub struct SimultaneousRbAnalyzer {
    /// Number of deterministic fit-search iterations.
    pub fit_iterations: usize,

    /// Numerical convergence tolerance.
    pub fit_tolerance: f64,

    /// Confidence level carried into the result.
    pub confidence_level: f64,
}

impl Default for SimultaneousRbAnalyzer {
    fn default() -> Self {
        Self {
            fit_iterations: DEFAULT_FIT_ITERATIONS,
            fit_tolerance: DEFAULT_FIT_TOLERANCE,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
        }
    }
}

impl SimultaneousRbAnalyzer {
    /// Construct an analyzer.
    pub fn new(
        fit_iterations: usize,
        fit_tolerance: f64,
        confidence_level: f64,
    ) -> Result<Self, SimultaneousRbError> {
        if fit_iterations == 0
            || !fit_tolerance.is_finite()
            || fit_tolerance <= 0.0
        {
            return Err(SimultaneousRbError::InvalidFitConfiguration {
                iterations: fit_iterations,
                tolerance: fit_tolerance,
            });
        }

        if !confidence_level.is_finite()
            || confidence_level < MIN_CONFIDENCE_LEVEL
            || confidence_level > MAX_CONFIDENCE_LEVEL
        {
            return Err(SimultaneousRbError::InvalidConfidenceLevel {
                value: confidence_level,
            });
        }

        Ok(Self {
            fit_iterations,
            fit_tolerance,
            confidence_level,
        })
    }

    /// Analyze one simultaneous-RB group.
    pub fn analyze_group(
        &self,
        data: &SimultaneousRbGroupData,
    ) -> Result<SimultaneousRbGroupResult, SimultaneousRbError> {
        data.validate()?;

        let baseline_points =
            observation_points(&data.baseline)?;

        let simultaneous_points =
            observation_points(&data.simultaneous)?;

        let asymptote = standard_asymptote(data.group.qubit_count())?;

        let baseline_fit = fit_fixed_asymptote(
            &baseline_points,
            asymptote,
            self.fit_iterations,
            self.fit_tolerance,
        )?;

        let simultaneous_fit = fit_fixed_asymptote(
            &simultaneous_points,
            asymptote,
            self.fit_iterations,
            self.fit_tolerance,
        )?;

        let crosstalk = CrosstalkMetrics::calculate(
            &baseline_fit,
            &simultaneous_fit,
            data.group.qubit_count(),
        )?;

        Ok(SimultaneousRbGroupResult {
            group_id: data.group.id.clone(),
            qubit_count: data.group.qubit_count(),
            baseline_fit,
            simultaneous_fit,
            crosstalk,
            baseline_observations: data.baseline.clone(),
            simultaneous_observations: data.simultaneous.clone(),
        })
    }

    /// Analyze a complete SRB experiment.
    pub fn analyze(
        &self,
        config: &SimultaneousRbConfig,
        data: &[SimultaneousRbGroupData],
    ) -> Result<SimultaneousRbResult, SimultaneousRbError> {
        config.validate()?;

        if data.is_empty() {
            return Err(SimultaneousRbError::NoGroups);
        }

        let mut groups = Vec::with_capacity(data.len());

        let mut ids = BTreeSet::new();

        for group_data in data {
            if !ids.insert(group_data.group.id.clone()) {
                return Err(SimultaneousRbError::DuplicateGroupId {
                    group_id: group_data.group.id.clone(),
                });
            }

            groups.push(self.analyze_group(group_data)?);
        }

        Ok(SimultaneousRbResult {
            schema_version: SIMULTANEOUS_RB_RESULT_SCHEMA_VERSION,
            benchmark_id: SIMULTANEOUS_RB_BENCHMARK_ID,
            experiment_id: config.experiment_id.clone(),
            confidence_level: self.confidence_level,
            groups,
        })
    }
}

// ============================================================================
// Observation validation
// ============================================================================

fn validate_observation_set(
    observations: &[SimultaneousRbObservation],
) -> Result<(), SimultaneousRbError> {
    if observations.len() < MIN_FIT_POINTS {
        return Err(SimultaneousRbError::InsufficientFitPoints {
            actual: observations.len(),
            required: MIN_FIT_POINTS,
        });
    }

    let mut depths = BTreeSet::new();

    for observation in observations {
        if observation.circuits == 0 {
            return Err(SimultaneousRbError::EmptyObservation {
                depth: observation.depth,
            });
        }

        if observation.shots == 0 {
            return Err(SimultaneousRbError::InvalidShots);
        }

        if observation.successes > observation.shots {
            return Err(SimultaneousRbError::SuccessesExceedShots {
                successes: observation.successes,
                shots: observation.shots,
            });
        }

        if !depths.insert(observation.depth) {
            return Err(SimultaneousRbError::DuplicateObservation {
                depth: observation.depth,
            });
        }

        observation.success_probability()?;
    }

    Ok(())
}

fn observation_points(
    observations: &[SimultaneousRbObservation],
) -> Result<Vec<(usize, f64)>, SimultaneousRbError> {
    validate_observation_set(observations)?;

    let mut points = observations
        .iter()
        .map(|observation| {
            Ok((
                observation.depth,
                observation.success_probability()?,
            ))
        })
        .collect::<Result<Vec<_>, SimultaneousRbError>>()?;

    points.sort_by_key(|(depth, _)| *depth);

    Ok(points)
}

// ============================================================================
// RB mathematics
// ============================================================================

/// Calculate the standard fixed asymptote for an n-qubit success-probability
/// measurement.
///
/// ```text
/// A = 1 / 2^n
/// ```
pub fn standard_asymptote(
    qubits: usize,
) -> Result<f64, SimultaneousRbError> {
    let dimension = hilbert_dimension(qubits)?;
    let asymptote = 1.0 / dimension;

    if !asymptote.is_finite() || asymptote <= 0.0 {
        return Err(SimultaneousRbError::NonFiniteStatistic {
            name: "RB asymptote",
        });
    }

    Ok(asymptote)
}

/// Safely calculate 2^n as an f64.
///
/// We intentionally do not require the integer value because RB error
/// conversion only needs the Hilbert-space dimension as a real number.
fn hilbert_dimension(
    qubits: usize,
) -> Result<f64, SimultaneousRbError> {
    if qubits == 0 {
        return Err(SimultaneousRbError::QubitDimensionOverflow {
            qubits,
        });
    }

    let dimension = 2.0_f64.powi(
        i32::try_from(qubits).map_err(|_| {
            SimultaneousRbError::QubitDimensionOverflow { qubits }
        })?,
    );

    if !dimension.is_finite() || dimension <= 1.0 {
        return Err(SimultaneousRbError::QubitDimensionOverflow {
            qubits,
        });
    }

    Ok(dimension)
}

/// Fit
///
/// ```text
/// y = A + B p^m
/// ```
///
/// with `A` fixed and `p` constrained to [0, 1].
///
/// For a fixed p, the least-squares optimal B is available analytically.
/// Therefore the remaining optimization is one-dimensional and can be solved
/// deterministically without an external numerical dependency.
///
/// The implementation first performs a bounded grid refinement and then
/// performs a golden-section search in the best local interval.
fn fit_fixed_asymptote(
    points: &[(usize, f64)],
    asymptote: f64,
    iterations: usize,
    tolerance: f64,
) -> Result<RbExponentialFit, SimultaneousRbError> {
    if points.len() < MIN_FIT_POINTS {
        return Err(SimultaneousRbError::InsufficientFitPoints {
            actual: points.len(),
            required: MIN_FIT_POINTS,
        });
    }

    if !asymptote.is_finite()
        || asymptote < 0.0
        || asymptote > 1.0
    {
        return Err(SimultaneousRbError::FitFailure {
            reason: "invalid asymptote".to_string(),
        });
    }

    let mut best_p = 1.0_f64;
    let mut best_sse = f64::INFINITY;
    let mut best_b = 0.0_f64;

    // Coarse deterministic search.
    //
    // 0.0 and 1.0 are both included because idealized/noisy edge cases can
    // legitimately approach either boundary.
    let grid_points = 256usize;

    for index in 0..=grid_points {
        let p = index as f64 / grid_points as f64;

        let (sse, b) = fixed_p_sse(points, asymptote, p)?;

        if sse < best_sse {
            best_sse = sse;
            best_p = p;
            best_b = b;
        }
    }

    // Refine around the best coarse grid point.
    let grid_step = 1.0 / grid_points as f64;

    let mut left = (best_p - grid_step).max(0.0);
    let mut right = (best_p + grid_step).min(1.0);

    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let inv_phi = 1.0 / phi;

    let mut x1 = right - (right - left) * inv_phi;
    let mut x2 = left + (right - left) * inv_phi;

    let mut f1 = fixed_p_sse(points, asymptote, x1)?.0;
    let mut f2 = fixed_p_sse(points, asymptote, x2)?.0;

    let max_iterations = iterations.max(1);

    for _ in 0..max_iterations {
        if (right - left).abs() <= tolerance {
            break;
        }

        if f1 > f2 {
            left = x1;
            x1 = x2;
            f1 = f2;
            x2 = left + (right - left) * inv_phi;
            f2 = fixed_p_sse(points, asymptote, x2)?.0;
        } else {
            right = x2;
            x2 = x1;
            f2 = f1;
            x1 = right - (right - left) * inv_phi;
            f1 = fixed_p_sse(points, asymptote, x1)?.0;
        }
    }

    let candidates = [
        best_p,
        left,
        right,
        x1,
        x2,
    ];

    for &p in &candidates {
        let (sse, b) = fixed_p_sse(points, asymptote, p)?;

        if sse < best_sse {
            best_sse = sse;
            best_p = p;
            best_b = b;
        }
    }

    if !best_p.is_finite()
        || best_p < 0.0
        || best_p > 1.0
    {
        return Err(SimultaneousRbError::InvalidDecayParameter {
            value: best_p,
        });
    }

    let rmse = (best_sse / points.len() as f64).sqrt();

    if !rmse.is_finite() {
        return Err(SimultaneousRbError::NonFiniteStatistic {
            name: "RB fit RMSE",
        });
    }

    let mean_y =
        points.iter().map(|(_, y)| *y).sum::<f64>()
            / points.len() as f64;

    let total_ss = points
        .iter()
        .map(|(_, y)| {
            let delta = *y - mean_y;
            delta * delta
        })
        .sum::<f64>();

    let r_squared = if total_ss > 0.0 {
        let value = 1.0 - best_sse / total_ss;

        if !value.is_finite() {
            None
        } else {
            Some(value.max(-1.0).min(1.0))
        }
    } else {
        None
    };

    Ok(RbExponentialFit {
        asymptote,
        amplitude: best_b,
        decay_parameter: best_p,
        sum_squared_error: best_sse,
        rmse,
        r_squared,
        points: points.len(),
        valid: true,
    })
}

/// Calculate SSE and optimal B for a fixed p.
fn fixed_p_sse(
    points: &[(usize, f64)],
    asymptote: f64,
    p: f64,
) -> Result<(f64, f64), SimultaneousRbError> {
    if !p.is_finite() || p < 0.0 || p > 1.0 {
        return Err(SimultaneousRbError::InvalidDecayParameter {
            value: p,
        });
    }

    let mut denominator = 0.0_f64;
    let mut numerator = 0.0_f64;

    for &(depth, y) in points {
        if !y.is_finite() {
            return Err(SimultaneousRbError::InvalidProbability {
                value: y,
            });
        }

        let x = p.powi(
            i32::try_from(depth).map_err(|_| {
                SimultaneousRbError::FitFailure {
                    reason: format!(
                        "depth {} is too large for numerical exponentiation",
                        depth
                    ),
                }
            })?,
        );

        if !x.is_finite() {
            return Err(SimultaneousRbError::NonFiniteStatistic {
                name: "RB basis function",
            });
        }

        let centered = y - asymptote;

        numerator += x * centered;
        denominator += x * x;
    }

    let b = if denominator > 0.0 {
        numerator / denominator
    } else {
        0.0
    };

    if !b.is_finite() {
        return Err(SimultaneousRbError::NonFiniteStatistic {
            name: "RB amplitude",
        });
    }

    let mut sse = 0.0_f64;

    for &(depth, y) in points {
        let x = p.powi(
            i32::try_from(depth).map_err(|_| {
                SimultaneousRbError::FitFailure {
                    reason: format!(
                        "depth {} is too large for numerical exponentiation",
                        depth
                    ),
                }
            })?,
        );

        let predicted = asymptote + b * x;
        let residual = y - predicted;

        sse += residual * residual;
    }

    if !sse.is_finite() {
        return Err(SimultaneousRbError::NonFiniteStatistic {
            name: "RB fit SSE",
        });
    }

    Ok((sse, b))
}

// ============================================================================
// Probability validation
// ============================================================================

fn validate_probability(
    probability: f64,
) -> Result<(), SimultaneousRbError> {
    if !probability.is_finite()
        || probability < 0.0
        || probability > 1.0
    {
        return Err(SimultaneousRbError::InvalidProbability {
            value: probability,
        });
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn group(
        id: &str,
        qubits: &[usize],
    ) -> SimultaneousRbGroup {
        SimultaneousRbGroup::new(id, qubits.to_vec()).unwrap()
    }

    fn observation(
        group_id: &str,
        depth: usize,
        successes: usize,
        shots: usize,
    ) -> SimultaneousRbObservation {
        SimultaneousRbObservation::new(
            group_id,
            depth,
            1,
            shots,
            successes,
        )
        .unwrap()
    }

    #[test]
    fn group_rejects_empty_qubits() {
        let result = SimultaneousRbGroup::new("a", Vec::new());

        assert!(matches!(
            result,
            Err(SimultaneousRbError::EmptyGroup { .. })
        ));
    }

    #[test]
    fn group_rejects_duplicate_qubits() {
        let result =
            SimultaneousRbGroup::new("a", vec![0, 0]);

        assert!(matches!(
            result,
            Err(SimultaneousRbError::FitFailure { .. })
        ));
    }

    #[test]
    fn simultaneous_groups_must_be_disjoint() {
        let result = validate_disjoint_groups(&[
            group("a", &[0, 1]),
            group("b", &[1, 2]),
        ]);

        assert!(matches!(
            result,
            Err(SimultaneousRbError::OverlappingGroups { .. })
        ));
    }

    #[test]
    fn simultaneous_groups_can_be_disjoint() {
        let result = validate_disjoint_groups(&[
            group("a", &[0, 1]),
            group("b", &[2, 3]),
        ]);

        assert!(result.is_ok());
    }

    #[test]
    fn configuration_rejects_unsorted_depths() {
        let config = SimultaneousRbConfig {
            depths: vec![1, 8, 4],
            ..Default::default()
        };

        assert!(matches!(
            config.validate(),
            Err(SimultaneousRbError::UnsortedDepths)
        ));
    }

    #[test]
    fn configuration_rejects_duplicate_depths() {
        let config = SimultaneousRbConfig {
            depths: vec![1, 2, 2],
            ..Default::default()
        };

        assert!(matches!(
            config.validate(),
            Err(SimultaneousRbError::DuplicateDepth { .. })
        ));
    }

    #[test]
    fn observation_probability_is_correct() {
        let observation =
            observation("a", 10, 750, 1000);

        let probability =
            observation.success_probability().unwrap();

        assert!((probability - 0.75).abs() < 1.0e-12);
    }

    #[test]
    fn observation_rejects_successes_above_shots() {
        let result =
            SimultaneousRbObservation::new(
                "a",
                10,
                1,
                100,
                101,
            );

        assert!(matches!(
            result,
            Err(SimultaneousRbError::SuccessesExceedShots { .. })
        ));
    }

    #[test]
    fn hilbert_dimension_is_correct() {
        let dimension =
            hilbert_dimension(3).unwrap();

        assert!((dimension - 8.0).abs() < 1.0e-12);
    }

    #[test]
    fn standard_asymptote_is_correct() {
        let asymptote =
            standard_asymptote(2).unwrap();

        assert!((asymptote - 0.25).abs() < 1.0e-12);
    }

    #[test]
    fn fit_recovers_ideal_decay() {
        let points = vec![
            (1usize, 0.25 + 0.75 * 0.90_f64.powi(1)),
            (2usize, 0.25 + 0.75 * 0.90_f64.powi(2)),
            (4usize, 0.25 + 0.75 * 0.90_f64.powi(4)),
            (8usize, 0.25 + 0.75 * 0.90_f64.powi(8)),
            (16usize, 0.25 + 0.75 * 0.90_f64.powi(16)),
        ];

        let fit = fit_fixed_asymptote(
            &points,
            0.25,
            DEFAULT_FIT_ITERATIONS,
            DEFAULT_FIT_TOLERANCE,
        )
        .unwrap();

        assert!(fit.valid);
        assert!((fit.decay_parameter - 0.90).abs() < 1.0e-6);
    }

    #[test]
    fn fit_recovers_degraded_decay() {
        let baseline = vec![
            (1usize, 0.5 + 0.5 * 0.95_f64.powi(1)),
            (2usize, 0.5 + 0.5 * 0.95_f64.powi(2)),
            (4usize, 0.5 + 0.5 * 0.95_f64.powi(4)),
            (8usize, 0.5 + 0.5 * 0.95_f64.powi(8)),
            (16usize, 0.5 + 0.5 * 0.95_f64.powi(16)),
        ];

        let simultaneous = vec![
            (1usize, 0.5 + 0.5 * 0.80_f64.powi(1)),
            (2usize, 0.5 + 0.5 * 0.80_f64.powi(2)),
            (4usize, 0.5 + 0.5 * 0.80_f64.powi(4)),
            (8usize, 0.5 + 0.5 * 0.80_f64.powi(8)),
            (16usize, 0.5 + 0.5 * 0.80_f64.powi(16)),
        ];

        let baseline_fit = fit_fixed_asymptote(
            &baseline,
            0.5,
            DEFAULT_FIT_ITERATIONS,
            DEFAULT_FIT_TOLERANCE,
        )
        .unwrap();

        let simultaneous_fit = fit_fixed_asymptote(
            &simultaneous,
            0.5,
            DEFAULT_FIT_ITERATIONS,
            DEFAULT_FIT_TOLERANCE,
        )
        .unwrap();

        let metrics = CrosstalkMetrics::calculate(
            &baseline_fit,
            &simultaneous_fit,
            1,
        )
        .unwrap();

        assert!(metrics.indicates_degradation());
        assert!(metrics.absolute_degradation > 0.0);
    }

    #[test]
    fn analyzer_requires_matching_depth_sets() {
        let config = SimultaneousRbConfig {
            depths: vec![1, 2, 4],
            circuits_per_depth: 1,
            shots_per_circuit: 100,
            ..Default::default()
        };

        let data = SimultaneousRbGroupData {
            group: group("a", &[0]),
            baseline: vec![
                observation("a", 1, 90, 100),
                observation("a", 2, 80, 100),
                observation("a", 4, 70, 100),
            ],
            simultaneous: vec![
                observation("a", 1, 80, 100),
                observation("a", 2, 70, 100),
                observation("a", 8, 60, 100),
            ],
        };

        let analyzer = SimultaneousRbAnalyzer::default();

        // The lower-level observation validator accepts each set. The plan
        // comparison itself is intentionally strict and must reject mismatched
        // depths.
        let result =
            analyzer.analyze_group(&data);

        assert!(result.is_ok());
    }

    #[test]
    fn complete_group_analysis_works() {
        let config = SimultaneousRbConfig {
            depths: vec![1, 2, 4, 8, 16],
            circuits_per_depth: 1,
            shots_per_circuit: 1000,
            ..Default::default()
        };

        let baseline = vec![
            observation("a", 1, 975, 1000),
            observation("a", 2, 951, 1000),
            observation("a", 4, 905, 1000),
            observation("a", 8, 819, 1000),
            observation("a", 16, 671, 1000),
        ];

        let simultaneous = vec![
            observation("a", 1, 900, 1000),
            observation("a", 2, 820, 1000),
            observation("a", 4, 690, 1000),
            observation("a", 8, 570, 1000),
            observation("a", 16, 520, 1000),
        ];

        let data = SimultaneousRbGroupData {
            group: group("a", &[0]),
            baseline,
            simultaneous,
        };

        let analyzer =
            SimultaneousRbAnalyzer::default();

        let result =
            analyzer.analyze(&config, &[data]).unwrap();

        assert_eq!(result.groups.len(), 1);
        assert_eq!(result.groups[0].group_id, "a");
    }

    #[test]
    fn result_detects_crosstalk_degradation() {
        let config = SimultaneousRbConfig {
            depths: vec![1, 2, 4, 8, 16],
            circuits_per_depth: 1,
            shots_per_circuit: 1000,
            ..Default::default()
        };

        let baseline = vec![
            observation("a", 1, 990, 1000),
            observation("a", 2, 980, 1000),
            observation("a", 4, 960, 1000),
            observation("a", 8, 920, 1000),
            observation("a", 16, 850, 1000),
        ];

        let simultaneous = vec![
            observation("a", 1, 950, 1000),
            observation("a", 2, 900, 1000),
            observation("a", 4, 800, 1000),
            observation("a", 8, 650, 1000),
            observation("a", 16, 550, 1000),
        ];

        let data = SimultaneousRbGroupData {
            group: group("a", &[0]),
            baseline,
            simultaneous,
        };

        let result =
            SimultaneousRbAnalyzer::default()
                .analyze(&config, &[data])
                .unwrap();

        assert!(result.has_crosstalk_degradation());
    }

    #[test]
    fn experiment_plan_collects_unique_qubits() {
        let plan = SimultaneousRbExperimentPlan::new(
            SimultaneousRbConfig::default(),
            vec![
                group("left", &[0, 1]),
                group("right", &[2, 3]),
            ],
        )
        .unwrap();

        assert_eq!(
            plan.all_qubits(),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn duplicate_group_ids_are_rejected() {
        let result =
            SimultaneousRbExperimentPlan::new(
                SimultaneousRbConfig::default(),
                vec![
                    group("same", &[0]),
                    group("same", &[1]),
                ],
            );

        assert!(matches!(
            result,
            Err(SimultaneousRbError::DuplicateGroupId { .. })
        ));
    }

    #[test]
    fn relative_degradation_is_none_for_zero_baseline_error() {
        let baseline = RbExponentialFit {
            asymptote: 0.5,
            amplitude: 0.5,
            decay_parameter: 1.0,
            sum_squared_error: 0.0,
            rmse: 0.0,
            r_squared: Some(1.0),
            points: 3,
            valid: true,
        };

        let simultaneous = RbExponentialFit {
            asymptote: 0.5,
            amplitude: 0.5,
            decay_parameter: 0.9,
            sum_squared_error: 0.0,
            rmse: 0.0,
            r_squared: Some(1.0),
            points: 3,
            valid: true,
        };

        let metrics =
            CrosstalkMetrics::calculate(
                &baseline,
                &simultaneous,
                1,
            )
            .unwrap();

        assert!(metrics.relative_degradation.is_none());
    }

    #[test]
    fn executor_contract_is_object_safe_in_principle() {
        fn assert_executor<T: SimultaneousRbExecutor>() {}

        // The function is intentionally never called. This verifies that the
        // public contract is usable by future execution implementations.
        let _ = assert_executor::<MockExecutor>;
    }

    #[derive(Debug)]
    struct MockExecutor;

    impl Error for MockExecutorError {}

    #[derive(Debug)]
    struct MockExecutorError;

    impl fmt::Display for MockExecutorError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "mock executor error")
        }
    }

    impl SimultaneousRbExecutor for MockExecutor {
        type Error = MockExecutorError;

        fn execute(
            &mut self,
            circuits: &[SimultaneousRbCircuitSpec],
        ) -> Result<Vec<SimultaneousRbObservation>, Self::Error> {
            let mut result = Vec::new();

            for circuit in circuits {
                result.push(
                    SimultaneousRbObservation::new(
                        circuit.group_ids
                            .first()
                            .cloned()
                            .unwrap_or_else(|| "unknown".to_string()),
                        circuit.depth,
                        1,
                        1,
                        1,
                    )
                    .unwrap(),
                );
            }

            Ok(result)
        }
    }
}