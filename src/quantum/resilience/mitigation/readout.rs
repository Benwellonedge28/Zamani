//! Readout-error mitigation for the Zamani quantum resilience subsystem.
//!
//! This module provides a backend-independent abstraction for mitigating
//! classical measurement/readout errors after quantum measurement.
//!
//! Design goals
//! ------------
//! * No provider-specific behavior.
//! * No fixed qubit count.
//! * No fixed number of outcomes.
//! * No fixed matrix representation.
//! * No `unsafe`.
//! * Works for one qubit through arbitrarily large registers, subject only
//!   to the resources required by the selected mitigation representation.
//! * Supports local, correlated, sparse, factorized, matrix-free, and
//!   provider-supplied mitigation models.
//! * Keeps quantum measurement semantics separate from classical readout
//!   correction.
//! * Preserves probability normalization and reports invalid/unstable
//!   corrections rather than silently producing incorrect results.
//! * Deterministic by default.
//! * Suitable for integration with `mitigation::strategy`, `selection`,
//!   `executor`, `verification`, `telemetry`, `history`, and hardware
//!   capability discovery.
//!
//! Architectural boundary
//! ----------------------
//! This module does NOT:
//! * perform quantum measurement;
//! * execute a quantum circuit;
//! * discover hardware;
//! * choose a backend;
//! * route qubits;
//! * schedule operations;
//! * perform QEC;
//! * implement arbitrary quantum-noise correction;
//! * silently retry failed executions.
//!
//! It operates on classical measurement observations and an explicit
//! readout-error model.
//!
//! Canonical quantum identity
//! --------------------------
//! When readout calibration is associated with physical qubits, callers
//! should use the canonical `crate::quantum::ir::qubit::QubitId` type.
//! This module deliberately does not create a second qubit identifier type.
//!
//! Mathematical convention
//! -----------------------
//! A readout model describes:
//
//!     P(observed = i | true = j)
//
//! The observed distribution is therefore:
//
//!     p_observed = M * p_true
//
//! Mitigation estimates p_true from p_observed without assuming that M is
//! dense, local, or globally representable.
//
//! IMPORTANT:
//! A mitigation result is an estimate. It is not automatically proof that
//! the underlying quantum computation was correct. Verification remains the
//! responsibility of `quantum::resilience::verification`.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::QubitId;

/// Stable identifier for a readout mitigation model.
///
/// The identifier is intentionally an opaque string. Providers, calibration
/// services, simulators, and future mitigation systems can introduce their
/// own identifiers without changing this module.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ReadoutModelId(Arc<str>);

impl ReadoutModelId {
    /// Creates a model identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ReadoutModelError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ReadoutModelError::InvalidModelId);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ReadoutModelId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Classical outcome identifier.
///
/// The representation is intentionally not tied to `u64` because a quantum
/// register can exceed the bit width of a machine integer. A canonical
/// bitstring is therefore represented as an owned string.
///
/// The string must contain only `0` and `1`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ReadoutOutcome(Arc<str>);

impl ReadoutOutcome {
    /// Creates an outcome from a binary string.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ReadoutModelError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ReadoutModelError::EmptyOutcome);
        }

        if !value.bytes().all(|byte| byte == b'0' || byte == b'1') {
            return Err(ReadoutModelError::InvalidOutcome);
        }

        Ok(Self(value))
    }

    /// Number of classical bits represented by this outcome.
    pub fn width(&self) -> usize {
        self.0.len()
    }

    /// Returns the canonical bitstring.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ReadoutOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Probability mass associated with a classical outcome.
///
/// This is kept as `f64` because readout mitigation is inherently numerical.
/// All public constructors and arithmetic paths validate finiteness.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Probability(f64);

impl Probability {
    /// Creates a probability in [0, 1].
    pub fn new(value: f64) -> Result<Self, ReadoutModelError> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(ReadoutModelError::InvalidProbability { value });
        }

        Ok(Self(value))
    }

    /// Returns the underlying value.
    pub fn get(self) -> f64 {
        self.0
    }
}

/// A probability distribution over observed classical outcomes.
#[derive(Clone, Debug, Default)]
pub struct ObservedDistribution {
    counts: BTreeMap<ReadoutOutcome, u64>,
    shots: u64,
}

impl ObservedDistribution {
    /// Creates an empty distribution.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a distribution from counts.
    pub fn from_counts(
        counts: BTreeMap<ReadoutOutcome, u64>,
    ) -> Result<Self, ReadoutModelError> {
        let shots = counts.values().try_fold(0_u64, |total, count| {
            total
                .checked_add(*count)
                .ok_or(ReadoutModelError::ShotCountOverflow)
        })?;

        if shots == 0 {
            return Err(ReadoutModelError::NoSamples);
        }

        Ok(Self { counts, shots })
    }

    /// Adds one outcome count.
    pub fn insert(
        &mut self,
        outcome: ReadoutOutcome,
        count: u64,
    ) -> Result<(), ReadoutModelError> {
        let new_shots = self
            .shots
            .checked_add(count)
            .ok_or(ReadoutModelError::ShotCountOverflow)?;

        let entry = self.counts.entry(outcome).or_insert(0);

        *entry = entry
            .checked_add(count)
            .ok_or(ReadoutModelError::ShotCountOverflow)?;

        self.shots = new_shots;

        Ok(())
    }

    /// Number of shots.
    pub fn shots(&self) -> u64 {
        self.shots
    }

    /// Returns the raw counts.
    pub fn counts(&self) -> &BTreeMap<ReadoutOutcome, u64> {
        &self.counts
    }

    /// Converts counts into a probability distribution.
    pub fn probabilities(&self) -> Result<MitigatedDistribution, ReadoutModelError> {
        if self.shots == 0 {
            return Err(ReadoutModelError::NoSamples);
        }

        let denominator = self.shots as f64;

        let mut probabilities = BTreeMap::new();

        for (outcome, count) in &self.counts {
            let probability = (*count as f64) / denominator;

            probabilities.insert(
                outcome.clone(),
                Probability::new(probability)?,
            );
        }

        Ok(MitigatedDistribution::new(probabilities)?)
    }
}

/// A probability distribution used both before and after mitigation.
///
/// Negative quasi-probabilities are intentionally NOT represented here.
/// Readout mitigation may mathematically produce negative intermediate
/// estimates when using unrestricted inversion. Such values must either be
/// projected/regularized into a physical distribution or returned as an
/// explicit error.
///
/// This prevents silent invalid probability output.
#[derive(Clone, Debug)]
pub struct MitigatedDistribution {
    probabilities: BTreeMap<ReadoutOutcome, Probability>,
}

impl MitigatedDistribution {
    /// Constructs and validates a probability distribution.
    pub fn new(
        probabilities: BTreeMap<ReadoutOutcome, Probability>,
    ) -> Result<Self, ReadoutModelError> {
        if probabilities.is_empty() {
            return Err(ReadoutModelError::EmptyDistribution);
        }

        let sum = probabilities
            .values()
            .map(|probability| probability.get())
            .sum::<f64>();

        if !sum.is_finite() {
            return Err(ReadoutModelError::NumericalFailure);
        }

        // Normalization is allowed to have a small numerical deviation.
        // The exact tolerance is not used as a machine-specific limit;
        // it is a numerical representation invariant.
        let tolerance = f64::EPSILON * (probabilities.len() as f64).max(1.0) * 32.0;

        if (sum - 1.0).abs() > tolerance {
            return Err(ReadoutModelError::DistributionNotNormalized { sum });
        }

        Ok(Self { probabilities })
    }

    /// Returns the mitigated probabilities.
    pub fn probabilities(&self) -> &BTreeMap<ReadoutOutcome, Probability> {
        &self.probabilities
    }

    /// Returns the probability of an outcome.
    pub fn probability(&self, outcome: &ReadoutOutcome) -> Option<Probability> {
        self.probabilities.get(outcome).copied()
    }
}

/// Readout assignment probability.
///
/// Represents:
///
///     P(observed | true)
#[derive(Clone, Copy, Debug)]
pub struct AssignmentProbability {
    value: f64,
}

impl AssignmentProbability {
    /// Creates an assignment probability.
    pub fn new(value: f64) -> Result<Self, ReadoutModelError> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(ReadoutModelError::InvalidProbability { value });
        }

        Ok(Self { value })
    }

    /// Returns the probability.
    pub fn get(self) -> f64 {
        self.value
    }
}

/// A sparse readout transition row.
///
/// `true_outcome -> observed_outcome -> probability`
#[derive(Clone, Debug)]
pub struct ReadoutTransitionRow {
    true_outcome: ReadoutOutcome,
    observed: BTreeMap<ReadoutOutcome, AssignmentProbability>,
}

impl ReadoutTransitionRow {
    /// Creates a row.
    pub fn new(true_outcome: ReadoutOutcome) -> Self {
        Self {
            true_outcome,
            observed: BTreeMap::new(),
        }
    }

    /// Adds an observed outcome probability.
    pub fn insert(
        &mut self,
        observed_outcome: ReadoutOutcome,
        probability: AssignmentProbability,
    ) {
        self.observed.insert(observed_outcome, probability);
    }

    /// Validates that the row represents a probability distribution.
    pub fn validate(&self) -> Result<(), ReadoutModelError> {
        if self.observed.is_empty() {
            return Err(ReadoutModelError::EmptyTransitionRow);
        }

        let sum = self
            .observed
            .values()
            .map(|probability| probability.get())
            .sum::<f64>();

        if !sum.is_finite() {
            return Err(ReadoutModelError::NumericalFailure);
        }

        let tolerance = f64::EPSILON * (self.observed.len() as f64).max(1.0) * 64.0;

        if (sum - 1.0).abs() > tolerance {
            return Err(ReadoutModelError::TransitionRowNotNormalized {
                true_outcome: self.true_outcome.clone(),
                sum,
            });
        }

        Ok(())
    }

    /// Returns the true outcome.
    pub fn true_outcome(&self) -> &ReadoutOutcome {
        &self.true_outcome
    }

    /// Returns observed transitions.
    pub fn observed(&self) -> &BTreeMap<ReadoutOutcome, AssignmentProbability> {
        &self.observed
    }
}

/// Readout model representation.
///
/// The model deliberately supports several representations so the core
/// resilience layer does not force exponential dense matrices.
///
/// A provider can supply a local/factorized model, a sparse model, or a
/// custom matrix-free implementation.
pub enum ReadoutModel {
    /// Independent local readout models.
    ///
    /// Each physical qubit has a 2x2 assignment model.
    Local(LocalReadoutModel),

    /// Sparse global assignment model.
    Sparse(SparseReadoutModel),

    /// Matrix-free custom model.
    MatrixFree(Arc<dyn MatrixFreeReadoutModel>),
}

impl fmt::Debug for ReadoutModel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local(model) => formatter.debug_tuple("Local").field(model).finish(),
            Self::Sparse(model) => formatter.debug_tuple("Sparse").field(model).finish(),
            Self::MatrixFree(_) => formatter.write_str("MatrixFree(..)"),
        }
    }
}

/// Single-qubit readout assignment model.
///
/// Matrix convention:
///
///       observed
///       0       1
///
/// true 0: p00     p10
/// true 1: p01     p11
///
/// Equivalently:
///
/// M[i,j] = P(observed=i | true=j)
#[derive(Clone, Copy, Debug)]
pub struct SingleQubitReadoutModel {
    p_observed_0_given_true_0: f64,
    p_observed_1_given_true_0: f64,
    p_observed_0_given_true_1: f64,
    p_observed_1_given_true_1: f64,
}

impl SingleQubitReadoutModel {
    /// Constructs a single-qubit model.
    pub fn new(
        p_observed_0_given_true_0: f64,
        p_observed_1_given_true_0: f64,
        p_observed_0_given_true_1: f64,
        p_observed_1_given_true_1: f64,
    ) -> Result<Self, ReadoutModelError> {
        let model = Self {
            p_observed_0_given_true_0,
            p_observed_1_given_true_0,
            p_observed_0_given_true_1,
            p_observed_1_given_true_1,
        };

        model.validate()?;

        Ok(model)
    }

    /// Validates the model.
    pub fn validate(&self) -> Result<(), ReadoutModelError> {
        for value in [
            self.p_observed_0_given_true_0,
            self.p_observed_1_given_true_0,
            self.p_observed_0_given_true_1,
            self.p_observed_1_given_true_1,
        ] {
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                return Err(ReadoutModelError::InvalidProbability { value });
            }
        }

        validate_pair(
            self.p_observed_0_given_true_0,
            self.p_observed_1_given_true_0,
        )?;

        validate_pair(
            self.p_observed_0_given_true_1,
            self.p_observed_1_given_true_1,
        )?;

        Ok(())
    }

    /// Returns P(observed=0 | true=0).
    pub fn p00(&self) -> f64 {
        self.p_observed_0_given_true_0
    }

    /// Returns P(observed=1 | true=0).
    pub fn p10(&self) -> f64 {
        self.p_observed_1_given_true_0
    }

    /// Returns P(observed=0 | true=1).
    pub fn p01(&self) -> f64 {
        self.p_observed_0_given_true_1
    }

    /// Returns P(observed=1 | true=1).
    pub fn p11(&self) -> f64 {
        self.p_observed_1_given_true_1
    }
}

/// Validates a probability pair.
fn validate_pair(first: f64, second: f64) -> Result<(), ReadoutModelError> {
    let sum = first + second;

    let tolerance = f64::EPSILON * 64.0;

    if (sum - 1.0).abs() > tolerance {
        return Err(ReadoutModelError::TransitionRowNotNormalized {
            true_outcome: ReadoutOutcome::new("0")?,
            sum,
        });
    }

    Ok(())
}

/// Local readout model indexed by canonical physical qubit identity.
///
/// This is the preferred scalable representation for independent/local
/// measurement errors.
#[derive(Clone, Debug, Default)]
pub struct LocalReadoutModel {
    qubits: BTreeMap<QubitId, SingleQubitReadoutModel>,
}

impl LocalReadoutModel {
    /// Creates an empty local model.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces calibration for a physical qubit.
    pub fn insert(
        &mut self,
        qubit: QubitId,
        model: SingleQubitReadoutModel,
    ) -> Result<(), ReadoutModelError> {
        model.validate()?;
        self.qubits.insert(qubit, model);
        Ok(())
    }

    /// Returns the calibration for a qubit.
    pub fn get(&self, qubit: &QubitId) -> Option<&SingleQubitReadoutModel> {
        self.qubits.get(qubit)
    }

    /// Returns all calibrated qubits.
    pub fn qubits(&self) -> &BTreeMap<QubitId, SingleQubitReadoutModel> {
        &self.qubits
    }

    /// Number of calibrated qubits.
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Whether no qubits are calibrated.
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }
}

/// Sparse global readout model.
///
/// This representation is appropriate when the global assignment channel
/// is not fully dense but a sparse representation is still practical.
#[derive(Clone, Debug, Default)]
pub struct SparseReadoutModel {
    rows: BTreeMap<ReadoutOutcome, ReadoutTransitionRow>,
}

impl SparseReadoutModel {
    /// Creates an empty sparse model.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a validated transition row.
    pub fn insert(
        &mut self,
        row: ReadoutTransitionRow,
    ) -> Result<(), ReadoutModelError> {
        row.validate()?;
        self.rows.insert(row.true_outcome.clone(), row);
        Ok(())
    }

    /// Returns a row.
    pub fn row(&self, true_outcome: &ReadoutOutcome) -> Option<&ReadoutTransitionRow> {
        self.rows.get(true_outcome)
    }

    /// Returns all rows.
    pub fn rows(&self) -> &BTreeMap<ReadoutOutcome, ReadoutTransitionRow> {
        &self.rows
    }
}

/// Matrix-free readout mitigation model.
///
/// This is the primary extension point for scalable techniques such as
/// iterative solvers, factorized models, tensor-network models, locality
/// approximations, matrix-free providers, or future algorithms.
pub trait MatrixFreeReadoutModel: Send + Sync + fmt::Debug {
    /// Stable model identifier.
    fn id(&self) -> &ReadoutModelId;

    /// Applies the forward readout channel:
    ///
    ///     p_observed = M * p_true
    ///
    /// This is useful for validation and diagnostics.
    fn apply_forward(
        &self,
        true_distribution: &MitigatedDistribution,
    ) -> Result<MitigatedDistribution, ReadoutModelError>;

    /// Estimates the true distribution from observed samples.
    ///
    /// Implementations must not return negative probabilities through the
    /// public `MitigatedDistribution` type.
    fn mitigate(
        &self,
        observed: &ObservedDistribution,
        options: &ReadoutMitigationOptions,
    ) -> Result<ReadoutMitigationResult, ReadoutModelError>;
}

/// What estimator/regularization method should be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReadoutEstimator {
    /// Direct inverse where the model is sufficiently well-conditioned.
    Inverse,

    /// Constrained correction that maintains a physical probability
    /// distribution.
    Constrained,

    /// Iterative correction.
    Iterative,

    /// Delegate to the supplied matrix-free model.
    MatrixFree,
}

/// Options controlling readout mitigation.
///
/// No value here represents a hardware limit. These are execution-policy
/// parameters supplied by the caller/policy layer.
#[derive(Clone, Debug)]
pub struct ReadoutMitigationOptions {
    estimator: ReadoutEstimator,
    regularization: Option<f64>,
    max_iterations: Option<u64>,
    convergence_tolerance: f64,
    reject_unstable_models: bool,
    normalize_output: bool,
}

impl Default for ReadoutMitigationOptions {
    fn default() -> Self {
        Self {
            estimator: ReadoutEstimator::Constrained,
            regularization: None,
            max_iterations: None,
            convergence_tolerance: f64::EPSILON.sqrt(),
            reject_unstable_models: true,
            normalize_output: true,
        }
    }
}

impl ReadoutMitigationOptions {
    /// Creates default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Selects the estimator.
    pub fn with_estimator(mut self, estimator: ReadoutEstimator) -> Self {
        self.estimator = estimator;
        self
    }

    /// Sets regularization.
    pub fn with_regularization(
        mut self,
        value: f64,
    ) -> Result<Self, ReadoutModelError> {
        if !value.is_finite() || value < 0.0 {
            return Err(ReadoutModelError::InvalidRegularization { value });
        }

        self.regularization = Some(value);
        Ok(self)
    }

    /// Sets the maximum iteration budget.
    pub fn with_max_iterations(mut self, value: u64) -> Self {
        self.max_iterations = Some(value);
        self
    }

    /// Sets convergence tolerance.
    pub fn with_convergence_tolerance(
        mut self,
        value: f64,
    ) -> Result<Self, ReadoutModelError> {
        if !value.is_finite() || value <= 0.0 {
            return Err(ReadoutModelError::InvalidTolerance { value });
        }

        self.convergence_tolerance = value;
        Ok(self)
    }

    /// Configures model stability enforcement.
    pub fn with_reject_unstable_models(mut self, value: bool) -> Self {
        self.reject_unstable_models = value;
        self
    }

    /// Configures normalization.
    pub fn with_normalization(mut self, value: bool) -> Self {
        self.normalize_output = value;
        self
    }

    /// Selected estimator.
    pub fn estimator(&self) -> ReadoutEstimator {
        self.estimator
    }

    /// Regularization parameter.
    pub fn regularization(&self) -> Option<f64> {
        self.regularization
    }

    /// Iteration budget.
    pub fn max_iterations(&self) -> Option<u64> {
        self.max_iterations
    }

    /// Convergence tolerance.
    pub fn convergence_tolerance(&self) -> f64 {
        self.convergence_tolerance
    }

    /// Whether unstable models should be rejected.
    pub fn reject_unstable_models(&self) -> bool {
        self.reject_unstable_models
    }

    /// Whether output normalization is requested.
    pub fn normalize_output(&self) -> bool {
        self.normalize_output
    }
}

/// Outcome of readout mitigation.
#[derive(Clone, Debug)]
pub struct ReadoutMitigationResult {
    distribution: MitigatedDistribution,
    diagnostics: ReadoutMitigationDiagnostics,
}

impl ReadoutMitigationResult {
    /// Creates a result.
    pub fn new(
        distribution: MitigatedDistribution,
        diagnostics: ReadoutMitigationDiagnostics,
    ) -> Self {
        Self {
            distribution,
            diagnostics,
        }
    }

    /// Returns the mitigated distribution.
    pub fn distribution(&self) -> &MitigatedDistribution {
        &self.distribution
    }

    /// Returns diagnostics.
    pub fn diagnostics(&self) -> &ReadoutMitigationDiagnostics {
        &self.diagnostics
    }
}

/// Diagnostics produced by mitigation.
///
/// These values are deliberately informational. They do not certify
/// correctness.
#[derive(Clone, Debug, Default)]
pub struct ReadoutMitigationDiagnostics {
    shots: u64,
    model_id: Option<ReadoutModelId>,
    estimator: Option<ReadoutEstimator>,
    regularization: Option<f64>,
    condition_indicator: Option<f64>,
    correction_norm: Option<f64>,
    residual_norm: Option<f64>,
    converged: Option<bool>,
}

impl ReadoutMitigationDiagnostics {
    /// Creates diagnostics.
    pub fn new(shots: u64) -> Self {
        Self {
            shots,
            ..Self::default()
        }
    }

    /// Sets model identifier.
    pub fn with_model_id(mut self, id: ReadoutModelId) -> Self {
        self.model_id = Some(id);
        self
    }

    /// Sets estimator.
    pub fn with_estimator(mut self, estimator: ReadoutEstimator) -> Self {
        self.estimator = Some(estimator);
        self
    }

    /// Sets regularization.
    pub fn with_regularization(mut self, value: Option<f64>) -> Self {
        self.regularization = value;
        self
    }

    /// Sets condition indicator.
    pub fn with_condition_indicator(mut self, value: f64) -> Self {
        self.condition_indicator = Some(value);
        self
    }

    /// Sets correction norm.
    pub fn with_correction_norm(mut self, value: f64) -> Self {
        self.correction_norm = Some(value);
        self
    }

    /// Sets residual norm.
    pub fn with_residual_norm(mut self, value: f64) -> Self {
        self.residual_norm = Some(value);
        self
    }

    /// Sets convergence state.
    pub fn with_converged(mut self, value: bool) -> Self {
        self.converged = Some(value);
        self
    }

    /// Number of shots.
    pub fn shots(&self) -> u64 {
        self.shots
    }

    /// Model identifier.
    pub fn model_id(&self) -> Option<&ReadoutModelId> {
        self.model_id.as_ref()
    }

    /// Estimator.
    pub fn estimator(&self) -> Option<ReadoutEstimator> {
        self.estimator
    }

    /// Condition indicator.
    pub fn condition_indicator(&self) -> Option<f64> {
        self.condition_indicator
    }

    /// Correction norm.
    pub fn correction_norm(&self) -> Option<f64> {
        self.correction_norm
    }

    /// Residual norm.
    pub fn residual_norm(&self) -> Option<f64> {
        self.residual_norm
    }

    /// Whether the iterative procedure converged.
    pub fn converged(&self) -> Option<bool> {
        self.converged
    }
}

/// Readout mitigation engine.
///
/// This type is intentionally stateless. Calibration and execution state
/// belong to the caller/runtime and are passed explicitly.
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadoutMitigator;

impl ReadoutMitigator {
    /// Creates a readout mitigator.
    pub const fn new() -> Self {
        Self
    }

    /// Applies readout mitigation using an explicit model.
    pub fn mitigate(
        &self,
        model: &ReadoutModel,
        observed: &ObservedDistribution,
        options: &ReadoutMitigationOptions,
    ) -> Result<ReadoutMitigationResult, ReadoutModelError> {
        if observed.shots() == 0 {
            return Err(ReadoutModelError::NoSamples);
        }

        match model {
            ReadoutModel::Local(local) => {
                self.mitigate_local(local, observed, options)
            }
            ReadoutModel::Sparse(sparse) => {
                self.mitigate_sparse(sparse, observed, options)
            }
            ReadoutModel::MatrixFree(matrix_free) => {
                matrix_free.mitigate(observed, options)
            }
        }
    }

    /// Mitigates a local independent-readout model.
    ///
    /// This implementation uses a tensor-product model without constructing
    /// the full 2^n x 2^n matrix.
    ///
    /// The current implementation deliberately refuses to silently perform
    /// a potentially exponential expansion when the observed support and
    /// calibrated qubits do not match. A matrix-free implementation can be
    /// supplied for larger or correlated systems.
    fn mitigate_local(
        &self,
        model: &LocalReadoutModel,
        observed: &ObservedDistribution,
        options: &ReadoutMitigationOptions,
    ) -> Result<ReadoutMitigationResult, ReadoutModelError> {
        let width = distribution_width(observed)?;

        if model.len() != width {
            return Err(ReadoutModelError::ModelWidthMismatch {
                expected: model.len(),
                actual: width,
            });
        }

        if model.is_empty() {
            return Err(ReadoutModelError::EmptyModel);
        }

        match options.estimator() {
            ReadoutEstimator::MatrixFree => {
                Err(ReadoutModelError::UnsupportedEstimator {
                    estimator: ReadoutEstimator::MatrixFree,
                    model: "local",
                })
            }
            ReadoutEstimator::Inverse | ReadoutEstimator::Constrained => {
                self.mitigate_local_exact_support(model, observed, options)
            }
            ReadoutEstimator::Iterative => {
                self.mitigate_local_iterative(model, observed, options)
            }
        }
    }

    /// Exact-support local mitigation.
    ///
    /// This path avoids materializing the assignment matrix but still works
    /// on the observed support. It is appropriate when the observed support
    /// is manageable.
    fn mitigate_local_exact_support(
        &self,
        model: &LocalReadoutModel,
        observed: &ObservedDistribution,
        options: &ReadoutMitigationOptions,
    ) -> Result<ReadoutMitigationResult, ReadoutModelError> {
        let mut corrected = BTreeMap::new();

        /*
         * For independent readout channels, direct tensor-product inversion
         * is mathematically straightforward but the support of the complete
         * distribution can itself be exponential.
         *
         * We therefore perform correction only over the supplied observed
         * support and require the caller to use the matrix-free path for
         * workloads whose complete distribution cannot be represented.
         *
         * The implementation below uses a coordinate-wise expectation-style
         * correction only when the distribution contains one-bit outcomes.
         * For multi-bit distributions, callers should use the iterative
         * matrix-free path.
         */
        if observed.counts().keys().any(|outcome| outcome.width() != 1) {
            return Err(ReadoutModelError::RequiresMatrixFreeForWideRegister {
                width: distribution_width(observed)?,
            });
        }

        let mut result = BTreeMap::new();

        for (outcome, count) in observed.counts() {
            let qubit = select_single_qubit(model)?;
            let calibration = model
                .get(&qubit)
                .ok_or(ReadoutModelError::MissingCalibration)?;

            let observed_one = outcome.as_str() == "1";
            let observed_probability = (*count as f64) / observed.shots() as f64;

            let corrected = if observed_one {
                invert_binary_probability(
                    observed_probability,
                    calibration.p10(),
                    calibration.p11(),
                    options,
                )?
            } else {
                let one_probability = invert_binary_probability(
                    1.0 - observed_probability,
                    calibration.p10(),
                    calibration.p11(),
                    options,
                )?;

                1.0 - one_probability
            };

            result.insert(
                outcome.clone(),
                Probability::new(corrected)?,
            );
        }

        let normalized = normalize_distribution(result)?;

        let diagnostics = ReadoutMitigationDiagnostics::new(observed.shots())
            .with_estimator(options.estimator())
            .with_regularization(options.regularization());

        Ok(ReadoutMitigationResult::new(normalized, diagnostics))
    }

    /// Iterative local mitigation.
    ///
    /// A full implementation for arbitrary register distributions should
    /// preferably be supplied through a matrix-free model. This built-in
    /// path intentionally avoids allocating a 2^n state vector.
    fn mitigate_local_iterative(
        &self,
        _model: &LocalReadoutModel,
        _observed: &ObservedDistribution,
        _options: &ReadoutMitigationOptions,
    ) -> Result<ReadoutMitigationResult, ReadoutModelError> {
        Err(ReadoutModelError::RequiresMatrixFreeForIterativeLocalModel)
    }

    /// Mitigates a sparse global model.
    fn mitigate_sparse(
        &self,
        _model: &SparseReadoutModel,
        _observed: &ObservedDistribution,
        options: &ReadoutMitigationOptions,
    ) -> Result<ReadoutMitigationResult, ReadoutModelError> {
        if options.estimator() == ReadoutEstimator::MatrixFree {
            return Err(ReadoutModelError::UnsupportedEstimator {
                estimator: ReadoutEstimator::MatrixFree,
                model: "sparse",
            });
        }

        /*
         * Sparse global inversion requires a solver and an explicit numerical
         * policy. It must not be implemented as an implicit dense matrix
         * conversion because that destroys the scalability contract.
         *
         * Therefore this file intentionally exposes the representation while
         * requiring a matrix-free solver implementation for production use
         * on arbitrary sparse global models.
         */
        Err(ReadoutModelError::SparseSolverRequired)
    }
}

/// Selects the only calibrated qubit in a one-bit model.
///
/// This function intentionally does not assume that the canonical qubit
/// identifier is contiguous or starts at zero.
fn select_single_qubit(
    model: &LocalReadoutModel,
) -> Result<QubitId, ReadoutModelError> {
    let mut iterator = model.qubits().keys();

    let first = iterator
        .next()
        .ok_or(ReadoutModelError::EmptyModel)?;

    if iterator.next().is_some() {
        return Err(ReadoutModelError::AmbiguousSingleQubitModel);
    }

    Ok(first.clone())
}

/// Determines the common classical register width.
fn distribution_width(
    observed: &ObservedDistribution,
) -> Result<usize, ReadoutModelError> {
    let mut width = None;

    for outcome in observed.counts().keys() {
        match width {
            None => width = Some(outcome.width()),
            Some(existing) if existing == outcome.width() => {}
            Some(existing) => {
                return Err(ReadoutModelError::InconsistentOutcomeWidth {
                    expected: existing,
                    actual: outcome.width(),
                });
            }
        }
    }

    width.ok_or(ReadoutModelError::NoSamples)
}

/// Performs binary readout inversion.
///
/// Observed probability:
///
///     p_obs = a + (b-a)p_true
///
/// where:
///
///     a = P(1 | 0)
///     b = P(1 | 1)
///
/// Thus:
///
///     p_true = (p_obs - a)/(b-a)
fn invert_binary_probability(
    observed_probability: f64,
    p_one_given_zero: f64,
    p_one_given_one: f64,
    options: &ReadoutMitigationOptions,
) -> Result<f64, ReadoutModelError> {
    if !observed_probability.is_finite() {
        return Err(ReadoutModelError::NumericalFailure);
    }

    let denominator = p_one_given_one - p_one_given_zero;

    if !denominator.is_finite() {
        return Err(ReadoutModelError::NumericalFailure);
    }

    let regularization = options.regularization().unwrap_or(0.0);

    let effective_denominator = if regularization > 0.0 {
        if denominator >= 0.0 {
            denominator + regularization
        } else {
            denominator - regularization
        }
    } else {
        denominator
    };

    let numerical_floor = f64::EPSILON.sqrt();

    if effective_denominator.abs() <= numerical_floor {
        return Err(ReadoutModelError::IllConditionedReadoutModel);
    }

    let corrected =
        (observed_probability - p_one_given_zero) / effective_denominator;

    if !corrected.is_finite() {
        return Err(ReadoutModelError::NumericalFailure);
    }

    if !(0.0..=1.0).contains(&corrected) {
        if options.estimator() == ReadoutEstimator::Constrained {
            return Ok(corrected.clamp(0.0, 1.0));
        }

        return Err(ReadoutModelError::NonPhysicalProbability {
            value: corrected,
        });
    }

    Ok(corrected)
}

/// Normalizes a distribution while retaining a physical probability vector.
fn normalize_distribution(
    values: BTreeMap<ReadoutOutcome, Probability>,
) -> Result<MitigatedDistribution, ReadoutModelError> {
    if values.is_empty() {
        return Err(ReadoutModelError::EmptyDistribution);
    }

    let sum = values
        .values()
        .map(|probability| probability.get())
        .sum::<f64>();

    if !sum.is_finite() || sum <= 0.0 {
        return Err(ReadoutModelError::NumericalFailure);
    }

    let mut normalized = BTreeMap::new();

    for (outcome, probability) in values {
        let value = probability.get() / sum;

        normalized.insert(outcome, Probability::new(value)?);
    }

    /*
     * Correct accumulated floating-point error deterministically by
     * reconstructing the final distribution through a second normalization.
     */
    let final_sum = normalized
        .values()
        .map(|probability| probability.get())
        .sum::<f64>();

    if !final_sum.is_finite() || final_sum <= 0.0 {
        return Err(ReadoutModelError::NumericalFailure);
    }

    let mut final_values = BTreeMap::new();

    for (outcome, probability) in normalized {
        final_values.insert(
            outcome,
            Probability::new(probability.get() / final_sum)?,
        );
    }

    MitigatedDistribution::new(final_values)
}

/// Errors produced by readout mitigation.
///
/// These errors are deliberately specific enough for the central resilience
/// error taxonomy to classify them later without losing the original cause.
#[derive(Clone, Debug, PartialEq)]
pub enum ReadoutModelError {
    InvalidModelId,
    EmptyOutcome,
    InvalidOutcome,
    InvalidProbability {
        value: f64,
    },
    InvalidRegularization {
        value: f64,
    },
    InvalidTolerance {
        value: f64,
    },
    NoSamples,
    EmptyDistribution,
    EmptyModel,
    EmptyTransitionRow,
    TransitionRowNotNormalized {
        true_outcome: ReadoutOutcome,
        sum: f64,
    },
    DistributionNotNormalized {
        sum: f64,
    },
    ShotCountOverflow,
    InconsistentOutcomeWidth {
        expected: usize,
        actual: usize,
    },
    ModelWidthMismatch {
        expected: usize,
        actual: usize,
    },
    MissingCalibration,
    AmbiguousSingleQubitModel,
    IllConditionedReadoutModel,
    NonPhysicalProbability {
        value: f64,
    },
    NumericalFailure,
    SparseSolverRequired,
    RequiresMatrixFreeForWideRegister {
        width: usize,
    },
    RequiresMatrixFreeForIterativeLocalModel,
    UnsupportedEstimator {
        estimator: ReadoutEstimator,
        model: &'static str,
}

impl fmt::Display for ReadoutModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModelId => formatter.write_str("readout model identifier is empty"),
            Self::EmptyOutcome => formatter.write_str("readout outcome is empty"),
            Self::InvalidOutcome => {
                formatter.write_str("readout outcome must contain only binary digits")
            }
            Self::InvalidProbability { value } => {
                write!(formatter, "invalid probability: {value}")
            }
            Self::InvalidRegularization { value } => {
                write!(formatter, "invalid regularization: {value}")
            }
            Self::InvalidTolerance { value } => {
                write!(formatter, "invalid convergence tolerance: {value}")
            }
            Self::NoSamples => formatter.write_str("no measurement samples were supplied"),
            Self::EmptyDistribution => {
                formatter.write_str("probability distribution is empty")
            }
            Self::EmptyModel => formatter.write_str("readout model is empty"),
            Self::EmptyTransitionRow => {
                formatter.write_str("readout transition row is empty")
            }
            Self::TransitionRowNotNormalized {
                true_outcome,
                sum,
            } => write!(
                formatter,
                "transition row for true outcome {true_outcome} is not normalized: {sum}"
            ),
            Self::DistributionNotNormalized { sum } => {
                write!(formatter, "distribution is not normalized: {sum}")
            }
            Self::ShotCountOverflow => {
                formatter.write_str("measurement shot count overflowed")
            }
            Self::InconsistentOutcomeWidth { expected, actual } => write!(
                formatter,
                "inconsistent readout outcome width: expected {expected}, got {actual}"
            ),
            Self::ModelWidthMismatch { expected, actual } => write!(
                formatter,
                "readout model width mismatch: expected {expected}, got {actual}"
            ),
            Self::MissingCalibration => {
                formatter.write_str("required readout calibration is missing")
            }
            Self::AmbiguousSingleQubitModel => {
                formatter.write_str("expected exactly one calibrated qubit")
            }
            Self::IllConditionedReadoutModel => {
                formatter.write_str("readout model is numerically ill-conditioned")
            }
            Self::NonPhysicalProbability { value } => {
                write!(formatter, "mitigation produced non-physical probability: {value}")
            }
            Self::NumericalFailure => formatter.write_str("numerical failure during mitigation"),
            Self::SparseSolverRequired => formatter.write_str(
                "sparse global mitigation requires an explicit sparse/matrix-free solver",
            ),
            Self::RequiresMatrixFreeForWideRegister { width } => write!(
                formatter,
                "wide local-register mitigation requires a matrix-free implementation: width {width}"
            ),
            Self::RequiresMatrixFreeForIterativeLocalModel => formatter.write_str(
                "iterative local mitigation requires a matrix-free implementation",
            ),
            Self::UnsupportedEstimator { estimator, model } => {
                write!(formatter, "estimator {estimator:?} is unsupported for {model} model")
            }
        }
    }
}

impl std::error::Error for ReadoutModelError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome_rejects_non_binary_data() {
        assert!(ReadoutOutcome::new("02").is_err());
        assert!(ReadoutOutcome::new("10").is_ok());
    }

    #[test]
    fn probability_rejects_invalid_values() {
        assert!(Probability::new(-0.1).is_err());
        assert!(Probability::new(1.1).is_err());
        assert!(Probability::new(f64::NAN).is_err());
        assert!(Probability::new(0.5).is_ok());
    }

    #[test]
    fn observed_distribution_accumulates_shots() {
        let outcome = ReadoutOutcome::new("0").expect("valid outcome");

        let mut distribution = ObservedDistribution::new();

        distribution
            .insert(outcome, 3)
            .expect("insert must succeed");

        assert_eq!(distribution.shots(), 3);
    }

    #[test]
    fn single_qubit_model_validates() {
        let model =
            SingleQubitReadoutModel::new(0.95, 0.05, 0.08, 0.92)
                .expect("valid model");

        assert_eq!(model.p00(), 0.95);
        assert_eq!(model.p11(), 0.92);
    }

    #[test]
    fn binary_inverse_recovers_known_probability() {
        let true_probability = 0.7;

        let p_one_given_zero = 0.05;
        let p_one_given_one = 0.9;

        let observed =
            p_one_given_zero
                + (p_one_given_one - p_one_given_zero)
                    * true_probability;

        let options = ReadoutMitigationOptions::default();

        let recovered = invert_binary_probability(
            observed,
            p_one_given_zero,
            p_one_given_one,
            &options,
        )
        .expect("well-conditioned model must invert");

        assert!((recovered - true_probability).abs() < 1.0e-12);
    }

    #[test]
    fn constrained_inverse_prevents_negative_probability() {
        let options = ReadoutMitigationOptions::default()
            .with_estimator(ReadoutEstimator::Constrained);

        let corrected =
            invert_binary_probability(0.0, 0.2, 0.8, &options)
                .expect("constrained correction must succeed");

        assert!((0.0..=1.0).contains(&corrected));
    }

    #[test]
    fn normalized_distribution_is_valid() {
        let zero = ReadoutOutcome::new("0").expect("valid outcome");
        let one = ReadoutOutcome::new("1").expect("valid outcome");

        let mut values = BTreeMap::new();

        values.insert(zero, Probability::new(0.25).expect("valid"));
        values.insert(one, Probability::new(0.75).expect("valid"));

        let distribution =
            MitigatedDistribution::new(values).expect("normalized distribution");

        assert_eq!(distribution.probabilities().len(), 2);
    }
}