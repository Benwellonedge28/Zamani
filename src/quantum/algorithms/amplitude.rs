//! Quantum amplitude amplification and estimation.
//!
//! This module provides two related but distinct algorithms:
//!
//! - [`AmplitudeAmplification`]: amplifies the probability of measuring a
//!   "good" state using repeated Grover-style reflections.
//! - [`AmplitudeEstimation`]: estimates an unknown success probability.
//!
//! The module deliberately keeps execution backend-independent. Algorithms
//! construct an [`AmplitudeCircuit`] description and delegate execution to an
//! [`AmplitudeExecutor`].
//!
//! Architectural boundary:
//!
//! ```text
//! amplitude.rs
//!      |
//!      +--> AmplitudeCircuit
//!      |
//!      +--> AmplitudeExecutor
//!                |
//!                v
//!        algorithms::execution
//!                |
//!                v
//!             IR / backend
//! ```
//!
//! This module does not own:
//!
//! - physical qubits,
//! - routing,
//! - hardware calibration,
//! - error correction,
//! - backend credentials,
//! - IR gate definitions,
//! - persistent storage.
//!
//! Those responsibilities belong to the corresponding quantum subsystems.

use std::fmt;

use super::error::{AlgorithmError, Result};
use super::types::{AlgorithmId, AlgorithmVersion, ExecutionConfig};

/// Stable identifier for the amplitude-amplification algorithm.
pub const AMPLITUDE_AMPLIFICATION_ID: AlgorithmId = AlgorithmId::AmplitudeAmplification;

/// Stable identifier for the amplitude-estimation algorithm.
pub const AMPLITUDE_ESTIMATION_ID: AlgorithmId = AlgorithmId::AmplitudeEstimation;

/// Current public algorithm version.
///
/// Increment the major version when the mathematical or execution contract
/// becomes incompatible with previous versions.
pub const AMPLITUDE_ALGORITHM_VERSION: AlgorithmVersion = AlgorithmVersion::new(1, 0, 0);

/// Maximum probability accepted at an API boundary.
///
/// Values outside `[0, 1]` are invalid. A tiny tolerance is intentionally
/// avoided: callers must provide mathematically valid probabilities.
const MIN_PROBABILITY: f64 = 0.0;
const MAX_PROBABILITY: f64 = 1.0;

/// Default confidence used by estimation.
const DEFAULT_CONFIDENCE: f64 = 0.95;

/// Default estimation precision.
const DEFAULT_PRECISION: f64 = 0.01;

/// Default number of amplification iterations when the caller requests
/// automatic iteration selection.
const DEFAULT_ITERATIONS: usize = 1;

/// A logical predicate describing whether a computational basis state is
/// considered "good".
///
/// The actual oracle implementation is intentionally abstract. A backend or
/// IR adapter may translate this contract into a concrete oracle circuit.
pub trait AmplitudeOracle {
    /// Number of logical qubits required by the oracle.
    fn qubit_count(&self) -> usize;

    /// Returns whether a computational basis state is marked as good.
    ///
    /// The state is represented as little-endian bits:
    ///
    /// ```text
    /// state[0] == least-significant qubit
    /// ```
    fn is_good(&self, state: &[bool]) -> Result<bool>;

    /// Stable identifier for reproducibility and execution records.
    fn identifier(&self) -> &str;
}

/// State-preparation abstraction used by amplitude algorithms.
///
/// State preparation is deliberately separate from the oracle because the
/// probability being estimated is the probability of obtaining a good state
/// after preparation.
pub trait StatePreparation {
    /// Number of logical qubits prepared.
    fn qubit_count(&self) -> usize;

    /// Stable identifier for reproducibility.
    fn identifier(&self) -> &str;
}

/// Backend-independent description of an amplitude circuit.
///
/// This is an algorithm-level circuit plan, not the canonical quantum IR.
/// Conversion into `quantum::ir::Circuit` belongs at the integration boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmplitudeCircuit {
    qubit_count: usize,
    amplification_iterations: usize,
    state_preparation_id: String,
    oracle_id: String,
}

impl AmplitudeCircuit {
    /// Creates a validated amplitude-amplification circuit description.
    pub fn new(
        qubit_count: usize,
        amplification_iterations: usize,
        state_preparation_id: impl Into<String>,
        oracle_id: impl Into<String>,
    ) -> Result<Self> {
        if qubit_count == 0 {
            return Err(AlgorithmError::InvalidQubitCount {
                expected: 1,
                actual: 0,
            });
        }

        if state_preparation_id.into().is_empty() {
            return Err(AlgorithmError::InvalidInput(
                "state preparation identifier must not be empty".to_owned(),
            ));
        }

        if oracle_id.into().is_empty() {
            return Err(AlgorithmError::InvalidInput(
                "oracle identifier must not be empty".to_owned(),
            ));
        }

        Ok(Self {
            qubit_count,
            amplification_iterations,
            state_preparation_id: state_preparation_id.into(),
            oracle_id: oracle_id.into(),
        })
    }

    /// Number of logical qubits.
    pub fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Number of amplification iterations.
    pub fn amplification_iterations(&self) -> usize {
        self.amplification_iterations
    }

    /// State-preparation identifier.
    pub fn state_preparation_id(&self) -> &str {
        &self.state_preparation_id
    }

    /// Oracle identifier.
    pub fn oracle_id(&self) -> &str {
        &self.oracle_id
    }
}

/// Execution request produced by the amplitude algorithms.
#[derive(Clone, Debug, PartialEq)]
pub struct AmplitudeExecutionRequest {
    /// Circuit description.
    pub circuit: AmplitudeCircuit,

    /// Execution configuration.
    pub execution: ExecutionConfig,

    /// Algorithm identifier.
    pub algorithm: AlgorithmId,

    /// Algorithm version.
    pub algorithm_version: AlgorithmVersion,

    /// Reproducibility seed.
    pub seed: Option<u64>,
}

impl AmplitudeExecutionRequest {
    /// Validates the complete request before it reaches a backend.
    pub fn validate(&self) -> Result<()> {
        if self.circuit.qubit_count() == 0 {
            return Err(AlgorithmError::InvalidQubitCount {
                expected: 1,
                actual: 0,
            });
        }

        self.execution.validate()?;

        Ok(())
    }
}

/// Measurement result returned by an amplitude executor.
#[derive(Clone, Debug, PartialEq)]
pub struct AmplitudeExecutionResult {
    /// Number of shots performed.
    pub shots: u64,

    /// Number of good outcomes.
    pub good_count: u64,

    /// Optional backend identifier.
    pub backend_id: Option<String>,

    /// Optional backend version.
    pub backend_version: Option<String>,
}

impl AmplitudeExecutionResult {
    /// Creates a validated execution result.
    pub fn new(shots: u64, good_count: u64) -> Result<Self> {
        if shots == 0 {
            return Err(AlgorithmError::InvalidInput(
                "execution must contain at least one shot".to_owned(),
            ));
        }

        if good_count > shots {
            return Err(AlgorithmError::InvalidInput(
                "good measurement count cannot exceed shot count".to_owned(),
            ));
        }

        Ok(Self {
            shots,
            good_count,
            backend_id: None,
            backend_version: None,
        })
    }

    /// Observed success probability.
    pub fn probability(&self) -> f64 {
        self.good_count as f64 / self.shots as f64
    }
}

/// Backend-independent execution boundary for amplitude algorithms.
///
/// A simulator, CPU backend, GPU backend, QPU backend, or remote executor can
/// implement this trait. The algorithm itself never needs to know which one
/// is being used.
pub trait AmplitudeExecutor {
    /// Executes an amplitude circuit.
    fn execute(&mut self, request: &AmplitudeExecutionRequest)
        -> Result<AmplitudeExecutionResult>;
}

/// Limits protecting amplitude algorithms from unbounded execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmplitudeResourceLimits {
    /// Maximum logical qubits.
    pub max_qubits: usize,

    /// Maximum amplification iterations.
    pub max_iterations: usize,

    /// Maximum shots.
    pub max_shots: u64,
}

impl Default for AmplitudeResourceLimits {
    fn default() -> Self {
        Self {
            max_qubits: 64,
            max_iterations: 1_000_000,
            max_shots: 10_000_000,
        }
    }
}

impl AmplitudeResourceLimits {
    /// Validates the limits themselves.
    pub fn validate(&self) -> Result<()> {
        if self.max_qubits == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "max_qubits must be greater than zero".to_owned(),
            ));
        }

        if self.max_iterations == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "max_iterations must be greater than zero".to_owned(),
            ));
        }

        if self.max_shots == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "max_shots must be greater than zero".to_owned(),
            ));
        }

        Ok(())
    }

    fn validate_qubits(&self, qubits: usize) -> Result<()> {
        if qubits > self.max_qubits {
            return Err(AlgorithmError::ResourceLimitExceeded(format!(
                "amplitude algorithm requires {} qubits but limit is {}",
                qubits, self.max_qubits
            )));
        }

        Ok(())
    }

    fn validate_iterations(&self, iterations: usize) -> Result<()> {
        if iterations > self.max_iterations {
            return Err(AlgorithmError::ResourceLimitExceeded(format!(
                "amplitude algorithm requires {} iterations but limit is {}",
                iterations, self.max_iterations
            )));
        }

        Ok(())
    }

    fn validate_shots(&self, shots: u64) -> Result<()> {
        if shots > self.max_shots {
            return Err(AlgorithmError::ResourceLimitExceeded(format!(
                "amplitude algorithm requires {} shots but limit is {}",
                shots, self.max_shots
            )));
        }

        Ok(())
    }
}

/// Configuration for amplitude amplification.
#[derive(Clone, Debug, PartialEq)]
pub struct AmplitudeAmplificationConfig {
    /// Optional explicit number of Grover iterations.
    ///
    /// `None` means the algorithm computes a recommended iteration count from
    /// the supplied initial success probability.
    pub iterations: Option<usize>,

    /// Estimated initial success probability.
    ///
    /// Required when automatic iteration selection is requested.
    pub initial_probability: Option<f64>,

    /// Shared execution configuration.
    pub execution: ExecutionConfig,

    /// Resource limits.
    pub limits: AmplitudeResourceLimits,
}

impl Default for AmplitudeAmplificationConfig {
    fn default() -> Self {
        Self {
            iterations: None,
            initial_probability: None,
            execution: ExecutionConfig::default(),
            limits: AmplitudeResourceLimits::default(),
        }
    }
}

impl AmplitudeAmplificationConfig {
    /// Validates configuration without requiring an oracle or state
    /// preparation object.
    pub fn validate(&self) -> Result<()> {
        self.limits.validate()?;
        self.execution.validate()?;

        if let Some(iterations) = self.iterations {
            self.limits.validate_iterations(iterations)?;
        } else if let Some(probability) = self.initial_probability {
            validate_probability(probability)?;
        } else {
            return Err(AlgorithmError::InvalidConfiguration(
                "initial_probability is required when iterations are not specified"
                    .to_owned(),
            ));
        }

        self.limits.validate_shots(self.execution.shots)?;

        Ok(())
    }
}

/// Result of amplitude amplification.
#[derive(Clone, Debug, PartialEq)]
pub struct AmplitudeAmplificationResult {
    /// Number of Grover/amplification iterations performed.
    pub iterations: usize,

    /// Number of measurement shots.
    pub shots: u64,

    /// Number of measured good states.
    pub good_count: u64,

    /// Empirical probability of a good state.
    pub measured_probability: f64,

    /// Algorithm identifier.
    pub algorithm: AlgorithmId,

    /// Algorithm version.
    pub algorithm_version: AlgorithmVersion,

    /// Seed used for deterministic execution, when configured.
    pub seed: Option<u64>,
}

impl AmplitudeAmplificationResult {
    /// Returns the measured success probability.
    pub fn probability(&self) -> f64 {
        self.measured_probability
    }
}

/// Production amplitude-amplification implementation.
#[derive(Clone, Debug)]
pub struct AmplitudeAmplification {
    config: AmplitudeAmplificationConfig,
}

impl AmplitudeAmplification {
    /// Creates a validated amplitude-amplification algorithm.
    pub fn new(config: AmplitudeAmplificationConfig) -> Result<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the algorithm configuration.
    pub fn config(&self) -> &AmplitudeAmplificationConfig {
        &self.config
    }

    /// Calculates the recommended Grover iteration count.
    ///
    /// For initial success probability `a`, the optimal iteration count is
    /// approximately:
    ///
    /// `floor(pi / (4 * asin(sqrt(a))))`
    ///
    /// The result is capped by the configured resource limit.
    pub fn recommended_iterations(&self) -> Result<usize> {
        let probability = self
            .config
            .initial_probability
            .ok_or_else(|| {
                AlgorithmError::InvalidConfiguration(
                    "initial_probability is required to calculate automatic iterations"
                        .to_owned(),
                )
            })?;

        validate_probability(probability)?;

        if probability == 0.0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "automatic amplitude amplification cannot determine an optimal iteration count \
                 when initial probability is zero"
                    .to_owned(),
            ));
        }

        if probability >= 1.0 {
            return Ok(0);
        }

        let theta = probability.sqrt().asin();
        let raw = std::f64::consts::PI / (4.0 * theta);

        if !raw.is_finite() {
            return Err(AlgorithmError::NumericalInstability(
                "automatic iteration calculation produced a non-finite value".to_owned(),
            ));
        }

        let iterations = raw.floor() as usize;

        self.config.limits.validate_iterations(iterations)?;

        Ok(iterations)
    }

    /// Executes amplitude amplification.
    pub fn run<S, O, E>(
        &self,
        state_preparation: &S,
        oracle: &O,
        executor: &mut E,
    ) -> Result<AmplitudeAmplificationResult>
    where
        S: StatePreparation,
        O: AmplitudeOracle,
        E: AmplitudeExecutor,
    {
        let qubits = validate_problem(state_preparation, oracle, &self.config.limits)?;

        let iterations = match self.config.iterations {
            Some(value) => value,
            None => self.recommended_iterations()?,
        };

        self.config.limits.validate_iterations(iterations)?;

        let circuit = AmplitudeCircuit::new(
            qubits,
            iterations,
            state_preparation.identifier(),
            oracle.identifier(),
        )?;

        let request = AmplitudeExecutionRequest {
            circuit,
            execution: self.config.execution.clone(),
            algorithm: AMPLITUDE_AMPLIFICATION_ID,
            algorithm_version: AMPLITUDE_ALGORITHM_VERSION,
            seed: self.config.execution.seed,
        };

        request.validate()?;

        let execution = executor.execute(&request)?;

        if execution.shots != self.config.execution.shots {
            return Err(AlgorithmError::ExecutionFailed(format!(
                "executor returned {} shots but {} were requested",
                execution.shots, self.config.execution.shots
            )));
        }

        let probability = execution.probability();

        validate_probability(probability)?;

        Ok(AmplitudeAmplificationResult {
            iterations,
            shots: execution.shots,
            good_count: execution.good_count,
            measured_probability: probability,
            algorithm: AMPLITUDE_AMPLIFICATION_ID,
            algorithm_version: AMPLITUDE_ALGORITHM_VERSION,
            seed: self.config.execution.seed,
        })
    }
}

/// Configuration for amplitude estimation.
///
/// This implementation uses statistically valid sampling of the amplified
/// circuit. It therefore estimates a success probability from measurement
/// results; it does not falsely represent ordinary sampling as the canonical
/// Brassard-style Quantum Amplitude Estimation algorithm.
#[derive(Clone, Debug, PartialEq)]
pub struct AmplitudeEstimationConfig {
    /// Number of amplification iterations evaluated.
    pub iterations: usize,

    /// Number of shots.
    pub shots: u64,

    /// Desired confidence level.
    ///
    /// Must satisfy `0 < confidence < 1`.
    pub confidence: f64,

    /// Optional target precision.
    ///
    /// This is an acceptance criterion, not a promise that the statistical
    /// estimator can always reach it with the configured shot count.
    pub precision: Option<f64>,

    /// Shared execution configuration.
    pub execution: ExecutionConfig,

    /// Resource limits.
    pub limits: AmplitudeResourceLimits,
}

impl Default for AmplitudeEstimationConfig {
    fn default() -> Self {
        let mut execution = ExecutionConfig::default();
        execution.shots = 10_000;

        Self {
            iterations: DEFAULT_ITERATIONS,
            shots: 10_000,
            confidence: DEFAULT_CONFIDENCE,
            precision: Some(DEFAULT_PRECISION),
            execution,
            limits: AmplitudeResourceLimits::default(),
        }
    }
}

impl AmplitudeEstimationConfig {
    /// Validates estimation configuration.
    pub fn validate(&self) -> Result<()> {
        self.limits.validate()?;

        if self.iterations == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "iterations must be greater than zero".to_owned(),
            ));
        }

        self.limits.validate_iterations(self.iterations)?;

        if self.shots == 0 {
            return Err(AlgorithmError::InvalidConfiguration(
                "shots must be greater than zero".to_owned(),
            ));
        }

        self.limits.validate_shots(self.shots)?;

        validate_confidence(self.confidence)?;

        if let Some(precision) = self.precision {
            validate_precision(precision)?;
        }

        self.execution.validate()?;

        if self.execution.shots != self.shots {
            return Err(AlgorithmError::InvalidConfiguration(
                "execution.shots must equal amplitude-estimation shots".to_owned(),
            ));
        }

        Ok(())
    }
}

/// Result returned by amplitude estimation.
#[derive(Clone, Debug, PartialEq)]
pub struct AmplitudeEstimationResult {
    /// Estimated success probability.
    pub estimate: f64,

    /// Lower confidence bound.
    pub lower_bound: f64,

    /// Upper confidence bound.
    pub upper_bound: f64,

    /// Width of the confidence interval.
    pub confidence_interval_width: f64,

    /// Confidence level used.
    pub confidence: f64,

    /// Number of shots.
    pub shots: u64,

    /// Number of observed good states.
    pub good_count: u64,

    /// Number of amplification iterations.
    pub iterations: usize,

    /// Algorithm identifier.
    pub algorithm: AlgorithmId,

    /// Algorithm version.
    pub algorithm_version: AlgorithmVersion,

    /// Reproducibility seed.
    pub seed: Option<u64>,
}

impl AmplitudeEstimationResult {
    /// Returns whether the configured precision criterion was achieved.
    pub fn meets_precision(&self, precision: f64) -> Result<bool> {
        validate_precision(precision)?;
        Ok(self.confidence_interval_width <= 2.0 * precision)
    }

    /// Returns the estimated probability.
    pub fn estimate(&self) -> f64 {
        self.estimate
    }
}

/// Production statistical amplitude estimator.
///
/// This estimator uses direct measurement sampling. It is intentionally
/// distinct from canonical Quantum Amplitude Estimation based on phase
/// estimation / amplitude-estimation operators.
#[derive(Clone, Debug)]
pub struct AmplitudeEstimation {
    config: AmplitudeEstimationConfig,
}

impl AmplitudeEstimation {
    /// Creates a validated amplitude estimator.
    pub fn new(config: AmplitudeEstimationConfig) -> Result<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the estimator configuration.
    pub fn config(&self) -> &AmplitudeEstimationConfig {
        &self.config
    }

    /// Executes the estimator.
    pub fn run<S, O, E>(
        &self,
        state_preparation: &S,
        oracle: &O,
        executor: &mut E,
    ) -> Result<AmplitudeEstimationResult>
    where
        S: StatePreparation,
        O: AmplitudeOracle,
        E: AmplitudeExecutor,
    {
        let qubits = validate_problem(state_preparation, oracle, &self.config.limits)?;

        let circuit = AmplitudeCircuit::new(
            qubits,
            self.config.iterations,
            state_preparation.identifier(),
            oracle.identifier(),
        )?;

        let request = AmplitudeExecutionRequest {
            circuit,
            execution: self.config.execution.clone(),
            algorithm: AMPLITUDE_ESTIMATION_ID,
            algorithm_version: AMPLITUDE_ALGORITHM_VERSION,
            seed: self.config.execution.seed,
        };

        request.validate()?;

        let execution = executor.execute(&request)?;

        if execution.shots != self.config.shots {
            return Err(AlgorithmError::ExecutionFailed(format!(
                "executor returned {} shots but {} were requested",
                execution.shots, self.config.shots
            )));
        }

        let estimate = execution.probability();

        validate_probability(estimate)?;

        let (lower_bound, upper_bound) =
            wilson_interval(execution.good_count, execution.shots, self.config.confidence)?;

        let width = upper_bound - lower_bound;

        if !width.is_finite() {
            return Err(AlgorithmError::NumericalInstability(
                "confidence interval produced a non-finite width".to_owned(),
            ));
        }

        Ok(AmplitudeEstimationResult {
            estimate,
            lower_bound,
            upper_bound,
            confidence_interval_width: width,
            confidence: self.config.confidence,
            shots: execution.shots,
            good_count: execution.good_count,
            iterations: self.config.iterations,
            algorithm: AMPLITUDE_ESTIMATION_ID,
            algorithm_version: AMPLITUDE_ALGORITHM_VERSION,
            seed: self.config.execution.seed,
        })
    }
}

/// Validates a state-preparation/oracle pair.
fn validate_problem<S, O>(
    state_preparation: &S,
    oracle: &O,
    limits: &AmplitudeResourceLimits,
) -> Result<usize>
where
    S: StatePreparation,
    O: AmplitudeOracle,
{
    let state_qubits = state_preparation.qubit_count();
    let oracle_qubits = oracle.qubit_count();

    if state_qubits == 0 {
        return Err(AlgorithmError::InvalidQubitCount {
            expected: 1,
            actual: 0,
        });
    }

    if oracle_qubits == 0 {
        return Err(AlgorithmError::InvalidQubitCount {
            expected: 1,
            actual: 0,
        });
    }

    if state_qubits != oracle_qubits {
        return Err(AlgorithmError::DimensionMismatch {
            expected: state_qubits,
            actual: oracle_qubits,
        });
    }

    limits.validate_qubits(state_qubits)?;

    if state_preparation.identifier().is_empty() {
        return Err(AlgorithmError::InvalidInput(
            "state preparation identifier must not be empty".to_owned(),
        ));
    }

    if oracle.identifier().is_empty() {
        return Err(AlgorithmError::InvalidInput(
            "oracle identifier must not be empty".to_owned(),
        ));
    }

    Ok(state_qubits)
}

/// Validates a probability.
fn validate_probability(value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(AlgorithmError::NonFiniteValue(
            "probability must be finite".to_owned(),
        ));
    }

    if !(MIN_PROBABILITY..=MAX_PROBABILITY).contains(&value) {
        return Err(AlgorithmError::InvalidParameter(format!(
            "probability must be in [0, 1], received {}",
            value
        )));
    }

    Ok(())
}

/// Validates a confidence value.
fn validate_confidence(value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(AlgorithmError::NonFiniteValue(
            "confidence must be finite".to_owned(),
        ));
    }

    if value <= 0.0 || value >= 1.0 {
        return Err(AlgorithmError::InvalidParameter(
            "confidence must be strictly between 0 and 1".to_owned(),
        ));
    }

    Ok(())
}

/// Validates a requested precision.
fn validate_precision(value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(AlgorithmError::NonFiniteValue(
            "precision must be finite".to_owned(),
        ));
    }

    if value <= 0.0 || value >= 1.0 {
        return Err(AlgorithmError::InvalidParameter(
            "precision must be greater than zero and less than one".to_owned(),
        ));
    }

    Ok(())
}

/// Calculates a Wilson score interval for a binomial proportion.
///
/// The implementation uses a deterministic inverse-normal approximation so
/// that this module has no dependency on a statistics crate.
fn wilson_interval(good_count: u64, shots: u64, confidence: f64) -> Result<(f64, f64)> {
    if shots == 0 {
        return Err(AlgorithmError::InvalidInput(
            "cannot calculate an interval with zero shots".to_owned(),
        ));
    }

    if good_count > shots {
        return Err(AlgorithmError::InvalidInput(
            "good_count cannot exceed shots".to_owned(),
        ));
    }

    validate_confidence(confidence)?;

    let z = inverse_standard_normal(0.5 + confidence / 2.0)?;

    let n = shots as f64;
    let p = good_count as f64 / n;
    let z2 = z * z;

    let denominator = 1.0 + z2 / n;
    let center = (p + z2 / (2.0 * n)) / denominator;

    let variance = (p * (1.0 - p) / n) + (z2 / (4.0 * n * n));

    if variance < 0.0 || !variance.is_finite() {
        return Err(AlgorithmError::NumericalInstability(
            "Wilson interval variance became invalid".to_owned(),
        ));
    }

    let margin = z * variance.sqrt() / denominator;

    let lower = (center - margin).clamp(0.0, 1.0);
    let upper = (center + margin).clamp(0.0, 1.0);

    if !lower.is_finite() || !upper.is_finite() || lower > upper {
        return Err(AlgorithmError::NumericalInstability(
            "Wilson interval produced invalid bounds".to_owned(),
        ));
    }

    Ok((lower, upper))
}

/// Deterministic inverse standard-normal CDF.
///
/// Uses the Acklam rational approximation.
fn inverse_standard_normal(p: f64) -> Result<f64> {
    if !p.is_finite() {
        return Err(AlgorithmError::NonFiniteValue(
            "normal-distribution probability must be finite".to_owned(),
        ));
    }

    if p <= 0.0 || p >= 1.0 {
        return Err(AlgorithmError::InvalidParameter(
            "normal-distribution probability must be strictly between 0 and 1"
                .to_owned(),
        ));
    }

    const A: [f64; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];

    const B: [f64; 5] = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];

    const C: [f64; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];

    const D: [f64; 4] = [
        7.784_695_709_041_462e-3,
        3.224_671_290_700_398e-1,
        2.445_134_137_142_996,
        3.754_408_661_907_416,
    ];

    const LOW: f64 = 0.024_25;
    const HIGH: f64 = 1.0 - LOW;

    let result;

    if p < LOW {
        let q = (-2.0 * p.ln()).sqrt();

        result = (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0);
    } else if p <= HIGH {
        let q = p - 0.5;
        let r = q * q;

        result = (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r
            + A[5])
            * q)
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0);
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();

        result = -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0));
    }

    if !result.is_finite() {
        return Err(AlgorithmError::NumericalInstability(
            "inverse normal approximation produced a non-finite value".to_owned(),
        ));
    }

    Ok(result)
}

impl fmt::Display for AmplitudeAmplificationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "AmplitudeAmplification(iterations={}, shots={}, good_count={}, probability={:.8})",
            self.iterations, self.shots, self.good_count, self.measured_probability
        )
    }
}

impl fmt::Display for AmplitudeEstimationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "AmplitudeEstimation(estimate={:.8}, confidence={:.4}, interval=[{:.8}, {:.8}], shots={}, iterations={})",
            self.estimate,
            self.confidence,
            self.lower_bound,
            self.upper_bound,
            self.shots,
            self.iterations
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct TestPreparation {
        qubits: usize,
        id: &'static str,
    }

    impl StatePreparation for TestPreparation {
        fn qubit_count(&self) -> usize {
            self.qubits
        }

        fn identifier(&self) -> &str {
            self.id
        }
    }

    #[derive(Clone, Debug)]
    struct TestOracle {
        qubits: usize,
        id: &'static str,
        good: bool,
    }

    impl AmplitudeOracle for TestOracle {
        fn qubit_count(&self) -> usize {
            self.qubits
        }

        fn is_good(&self, _state: &[bool]) -> Result<bool> {
            Ok(self.good)
        }

        fn identifier(&self) -> &str {
            self.id
        }
    }

    #[derive(Default)]
    struct TestExecutor {
        calls: usize,
        good_count: u64,
        shots: u64,
    }

    impl AmplitudeExecutor for TestExecutor {
        fn execute(
            &mut self,
            request: &AmplitudeExecutionRequest,
        ) -> Result<AmplitudeExecutionResult> {
            self.calls += 1;

            if request.execution.shots != self.shots {
                return Err(AlgorithmError::ExecutionFailed(
                    "test executor received unexpected shot count".to_owned(),
                ));
            }

            AmplitudeExecutionResult::new(self.shots, self.good_count)
        }
    }

    fn execution_config(shots: u64) -> ExecutionConfig {
        let mut config = ExecutionConfig::default();
        config.shots = shots;
        config.seed = Some(42);
        config.deterministic = true;
        config
    }

    #[test]
    fn probability_validation_rejects_non_finite_values() {
        assert!(validate_probability(f64::NAN).is_err());
        assert!(validate_probability(f64::INFINITY).is_err());
        assert!(validate_probability(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn probability_validation_rejects_values_outside_range() {
        assert!(validate_probability(-0.1).is_err());
        assert!(validate_probability(1.1).is_err());
        assert!(validate_probability(0.0).is_ok());
        assert!(validate_probability(1.0).is_ok());
    }

    #[test]
    fn resource_limits_reject_zero_values() {
        let limits = AmplitudeResourceLimits {
            max_qubits: 0,
            max_iterations: 1,
            max_shots: 1,
        };

        assert!(limits.validate().is_err());
    }

    #[test]
    fn circuit_rejects_zero_qubits() {
        assert!(AmplitudeCircuit::new(0, 1, "state", "oracle").is_err());
    }

    #[test]
    fn circuit_accepts_valid_input() {
        let circuit = AmplitudeCircuit::new(4, 3, "state", "oracle").unwrap();

        assert_eq!(circuit.qubit_count(), 4);
        assert_eq!(circuit.amplification_iterations(), 3);
        assert_eq!(circuit.state_preparation_id(), "state");
        assert_eq!(circuit.oracle_id(), "oracle");
    }

    #[test]
    fn execution_result_rejects_good_count_above_shots() {
        assert!(AmplitudeExecutionResult::new(10, 11).is_err());
    }

    #[test]
    fn recommended_iterations_are_deterministic() {
        let mut config = AmplitudeAmplificationConfig::default();
        config.initial_probability = Some(0.25);
        config.iterations = None;

        let algorithm = AmplitudeAmplification::new(config).unwrap();

        let first = algorithm.recommended_iterations().unwrap();
        let second = algorithm.recommended_iterations().unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn recommended_iterations_for_quarter_probability() {
        let mut config = AmplitudeAmplificationConfig::default();
        config.initial_probability = Some(0.25);

        let algorithm = AmplitudeAmplification::new(config).unwrap();

        assert_eq!(algorithm.recommended_iterations().unwrap(), 1);
    }

    #[test]
    fn automatic_iterations_reject_zero_probability() {
        let mut config = AmplitudeAmplificationConfig::default();
        config.initial_probability = Some(0.0);

        let algorithm = AmplitudeAmplification::new(config).unwrap();

        assert!(algorithm.recommended_iterations().is_err());
    }

    #[test]
    fn problem_rejects_mismatched_qubit_counts() {
        let preparation = TestPreparation {
            qubits: 3,
            id: "state",
        };

        let oracle = TestOracle {
            qubits: 4,
            id: "oracle",
            good: true,
        };

        let limits = AmplitudeResourceLimits::default();

        assert!(validate_problem(&preparation, &oracle, &limits).is_err());
    }

    #[test]
    fn amplification_runs_through_executor_boundary() {
        let mut config = AmplitudeAmplificationConfig::default();
        config.iterations = Some(2);
        config.execution = execution_config(100);

        let algorithm = AmplitudeAmplification::new(config).unwrap();

        let preparation = TestPreparation {
            qubits: 2,
            id: "state",
        };

        let oracle = TestOracle {
            qubits: 2,
            id: "oracle",
            good: true,
        };

        let mut executor = TestExecutor {
            calls: 0,
            good_count: 75,
            shots: 100,
        };

        let result = algorithm
            .run(&preparation, &oracle, &mut executor)
            .unwrap();

        assert_eq!(executor.calls, 1);
        assert_eq!(result.iterations, 2);
        assert_eq!(result.good_count, 75);
        assert!((result.probability() - 0.75).abs() < 1e-12);
        assert_eq!(result.seed, Some(42));
    }

    #[test]
    fn estimation_runs_through_executor_boundary() {
        let mut config = AmplitudeEstimationConfig::default();
        config.shots = 1_000;
        config.execution = execution_config(1_000);

        let estimator = AmplitudeEstimation::new(config).unwrap();

        let preparation = TestPreparation {
            qubits: 3,
            id: "state",
        };

        let oracle = TestOracle {
            qubits: 3,
            id: "oracle",
            good: true,
        };

        let mut executor = TestExecutor {
            calls: 0,
            good_count: 250,
            shots: 1_000,
        };

        let result = estimator
            .run(&preparation, &oracle, &mut executor)
            .unwrap();

        assert_eq!(executor.calls, 1);
        assert!((result.estimate() - 0.25).abs() < 1e-12);
        assert!(result.lower_bound >= 0.0);
        assert!(result.upper_bound <= 1.0);
        assert!(result.lower_bound <= result.estimate());
        assert!(result.estimate() <= result.upper_bound);
    }

    #[test]
    fn wilson_interval_is_bounded() {
        let (lower, upper) = wilson_interval(50, 100, 0.95).unwrap();

        assert!(lower >= 0.0);
        assert!(upper <= 1.0);
        assert!(lower < 0.5);
        assert!(upper > 0.5);
    }

    #[test]
    fn wilson_interval_handles_all_failures() {
        let (lower, upper) = wilson_interval(0, 100, 0.95).unwrap();

        assert_eq!(lower, 0.0);
        assert!(upper > 0.0);
        assert!(upper <= 1.0);
    }

    #[test]
    fn wilson_interval_handles_all_successes() {
        let (lower, upper) = wilson_interval(100, 100, 0.95).unwrap();

        assert!(lower < 1.0);
        assert_eq!(upper, 1.0);
    }

    #[test]
    fn inverse_normal_is_zero_at_half() {
        let value = inverse_standard_normal(0.5).unwrap();

        assert!(value.abs() < 1e-10);
    }

    #[test]
    fn inverse_normal_is_positive_above_half() {
        let value = inverse_standard_normal(0.975).unwrap();

        assert!(value > 1.9);
        assert!(value < 2.0);
    }

    #[test]
    fn inverse_normal_rejects_invalid_probability() {
        assert!(inverse_standard_normal(0.0).is_err());
        assert!(inverse_standard_normal(1.0).is_err());
        assert!(inverse_standard_normal(-0.1).is_err());
        assert!(inverse_standard_normal(1.1).is_err());
    }

    #[test]
    fn confidence_validation_is_strict() {
        assert!(validate_confidence(0.0).is_err());
        assert!(validate_confidence(1.0).is_err());
        assert!(validate_confidence(0.95).is_ok());
    }

    #[test]
    fn precision_validation_is_strict() {
        assert!(validate_precision(0.0).is_err());
        assert!(validate_precision(1.0).is_err());
        assert!(validate_precision(0.01).is_ok());
    }

    #[test]
    fn estimation_requires_matching_shots() {
        let mut config = AmplitudeEstimationConfig::default();
        config.shots = 1_000;
        config.execution = execution_config(500);

        assert!(AmplitudeEstimation::new(config).is_err());
    }

    #[test]
    fn amplification_result_display_is_stable() {
        let result = AmplitudeAmplificationResult {
            iterations: 2,
            shots: 100,
            good_count: 75,
            measured_probability: 0.75,
            algorithm: AMPLITUDE_AMPLIFICATION_ID,
            algorithm_version: AMPLITUDE_ALGORITHM_VERSION,
            seed: Some(42),
        };

        let rendered = result.to_string();

        assert!(rendered.contains("AmplitudeAmplification"));
        assert!(rendered.contains("iterations=2"));
        assert!(rendered.contains("probability=0.75000000"));
    }

    #[test]
    fn estimation_result_display_is_stable() {
        let result = AmplitudeEstimationResult {
            estimate: 0.25,
            lower_bound: 0.20,
            upper_bound: 0.30,
            confidence_interval_width: 0.10,
            confidence: 0.95,
            shots: 1_000,
            good_count: 250,
            iterations: 1,
            algorithm: AMPLITUDE_ESTIMATION_ID,
            algorithm_version: AMPLITUDE_ALGORITHM_VERSION,
            seed: Some(42),
        };

        let rendered = result.to_string();

        assert!(rendered.contains("AmplitudeEstimation"));
        assert!(rendered.contains("estimate=0.25000000"));
        assert!(rendered.contains("confidence=0.9500"));
    }
}