//! Zamani Quantum Resilience — Probabilistic Error Cancellation
//!
//! Path:
//!     src/quantum/resilience/mitigation/probabilistic.rs
//!
//! Purpose:
//!     Provider-independent probabilistic error cancellation (PEC) and
//!     quasi-probability mitigation primitives.
//!
//! =============================================================================
//! Architectural position
//! =============================================================================
//!
//! ```text
//!                    Canonical Zamani Quantum IR
//!                               |
//!                               v
//!                       Resilience Controller
//!                               |
//!                               v
//!                     Mitigation Selection
//!                               |
//!                               v
//!                    Probabilistic Strategy
//!                               |
//!                    +----------+----------+
//!                    |                     |
//!                    v                     v
//!             QPD / quasi-probabilities   Policy
//!                    |
//!                    v
//!             Execution Specification
//!                    |
//!                    v
//!             Mitigation Executor
//!                    |
//!                    v
//!             Hardware / Simulator
//!                    |
//!                    v
//!              Raw observations
//!                    |
//!                    v
//!             PEC estimator
//!                    |
//!                    v
//!               Verification
//! ```
//!
//! This module is deliberately a mathematical/domain layer.
//!
//! It does NOT:
//!
//! - execute quantum circuits;
//! - communicate with providers;
//! - access credentials;
//! - perform routing;
//! - perform scheduling;
//! - perform QEC;
//! - perform calibration;
//! - own random-number generation;
//! - mutate a QuantumCircuit;
//! - invent physical-qubit identifiers;
//! - impose a fixed qubit count;
//! - impose a fixed shot count;
//! - impose a fixed number of decomposition terms;
//! - impose a fixed circuit size;
//! - silently clamp estimates;
//! - silently retry execution;
//! - silently change program semantics.
//!
//! Actual execution belongs to:
//!
//!     mitigation/executor.rs
//!
//! Circuit transformation belongs to the appropriate compiler/optimization
//! subsystem.
//!
//! Hardware capability information belongs to:
//!
//!     quantum::hardware
//!
//! Fault/noise semantics belong to:
//!
//!     quantum::zqn
//!
//! Canonical quantum semantics belong to:
//!
//!     quantum::ir
//!
//! Resilience policy belongs to:
//!
//!     quantum::resilience::policy
//!
//! Strategy selection belongs to:
//!
//!     quantum::resilience::mitigation::selection
//!
//! =============================================================================
//! Probabilistic error cancellation
//! =============================================================================
//!
//! PEC represents a target operation/channel as a quasi-probability
//! decomposition:
//!
//!     E^{-1} ~= sum_i c_i B_i
//!
//! where:
//!
//!     c_i ∈ R
//!
//! and `B_i` are executable basis operations supplied by the execution layer.
//!
//! Unlike an ordinary probability distribution, coefficients may be negative.
//! Therefore sampling is performed according to:
//!
//!     p_i = |c_i| / gamma
//!
//! where:
//!
//!     gamma = sum_i |c_i|
//!
//! and the sign:
//!
//!     sign_i = sign(c_i)
//!
//! is carried into the estimator.
//!
//! For an observable `X`, the ideal PEC estimator has the form:
//!
//!     X_hat = gamma * mean(sign_i * X_i)
//!
//! This implementation deliberately keeps the execution of `B_i` outside this
//! module. The executor maps the opaque basis-operation identifiers to actual
//! canonical IR transformations/executions.
//!
//! =============================================================================
//! Write once, scale everywhere
//! =============================================================================
//!
//! There is intentionally no:
//!
//!     MAX_QUBITS
//!     MAX_TERMS
//!     MAX_SHOTS
//!     MAX_GATES
//!     DEFAULT_GAMMA
//!     DEFAULT_RETRY_COUNT
//!
//! The representation is dynamically sized and uses caller-supplied resources.
//!
//! A very large decomposition may require substantial classical memory and
//! execution resources. That is a runtime/resource-policy concern, not an
//! artificial architectural ceiling.
//!
//! =============================================================================
//! Numerical safety
//! =============================================================================
//!
//! PEC can have substantial sampling overhead. In particular, the variance
//! can scale with the square of the quasi-probability L1 norm:
//!
//!     gamma^2
//!
//! This module therefore:
//!
//! - validates all floating-point inputs;
//! - rejects NaN;
//! - rejects infinities;
//! - rejects invalid probabilities;
//! - rejects empty decompositions;
//! - rejects zero-L1 decompositions;
//! - detects arithmetic overflow/non-finite intermediate values;
//! - never silently clamps results;
//! - exposes the L1 norm explicitly;
//! - exposes an overhead metric for policy/planning.
//!
//! The module does not assume that PEC is unbiased in the presence of an
//! inaccurate noise model. Model quality remains an external responsibility.
//!
//! =============================================================================
//! Determinism
//! =============================================================================
//!
//! The core sampler is deterministic for identical:
//!
//! - quasi-probability representation;
//! - random variate;
//! - representation version.
//!
//! No random generator is hidden inside this module.
//!
//! This allows the executor/runtime to provide a cryptographically secure,
//! pseudo-random, hardware-random, reproducible, or externally coordinated
//! random source without changing the PEC implementation.
//!
//! =============================================================================
//! Canonical qubit identity
//! =============================================================================
//!
//! This file does not need to import `QubitId` directly.
//!
//! A PEC strategy may still be scoped through the canonical
//! `MitigationScope::LogicalQubits(...)` defined by `strategy.rs`, which uses:
//!
//!     quantum::ir::qubit::QubitId
//!
//! No alternative qubit identifier is introduced here.
//!
//! =============================================================================
//! Integration contract
//! =============================================================================
//!
//! `strategy.rs`
//!     Supplies the stable `MitigationStrategy` contract implemented below.
//!
//! `selection.rs`
//!     Evaluates applicability and uses the descriptor/overhead metadata.
//!
//! `executor.rs`
//!     Consumes `ProbabilisticExecutionSpec`, performs actual basis-operation
//!     executions, and supplies observations to `ProbabilisticEstimator`.
//!
//! `custom.rs`
//!     May provide custom QPD/basis-operation construction.
//!
//! `policy/*`
//!     Decides whether the potentially high sampling overhead is permitted.
//!
//! `planning/*`
//!     Uses `gamma`, expected executions and other overhead metadata when
//!     ranking mitigation plans.
//!
//! `verification/*`
//!     Verifies that the mitigated result satisfies semantic and confidence
//!     requirements.
//!
//! `telemetry/*`
//!     Records representation identity, gamma, estimator configuration and
//!     execution provenance.
//!
//! `history/*`
//!     Records observed PEC variance, success and cost.
//!
//! `serialization/*`
//!     Can serialize these immutable domain values through an appropriate
//!     schema implementation.
//!
//! `quantum::ir`
//!     Remains authoritative for the actual quantum program.
//!
//! `quantum::zqn`
//!     Remains authoritative for fault/noise semantics.
//!
//! `quantum::hardware`
//!     Remains authoritative for hardware capabilities.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::sync::Arc;

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

use crate::quantum::resilience::mitigation::strategy::{
    Applicability,
    ExpectedOverhead,
    MitigationScope,
    MitigationStrategy,
    OverheadDimension,
    OverheadLevel,
    StrategyContext,
    StrategyDescriptor,
    StrategyFamily,
    StrategyId,
    StrategyPhase,
    StrategyRequirement,
    StrategyVersion,
};

// =============================================================================
// Schema identity
// =============================================================================

/// Stable schema identifier for probabilistic mitigation.
pub const PROBABILISTIC_MITIGATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.mitigation.probabilistic";

/// Semantic version of the probabilistic mitigation contract.
pub const PROBABILISTIC_MITIGATION_SCHEMA_VERSION: u16 = 1;

/// Stable strategy identifier.
pub const PROBABILISTIC_STRATEGY_ID: &str =
    "probabilistic_error_cancellation";

// =============================================================================
// Floating-point validation
// =============================================================================

fn finite(value: f64, field: &'static str) -> ResilienceResult<f64> {
    if !value.is_finite() {
        return Err(
            ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
                format!("{field} must be finite"),
            )
            .with_operation(field),
        );
    }

    Ok(value)
}

fn positive_finite(value: f64, field: &'static str) -> ResilienceResult<f64> {
    finite(value, field)?;

    if value <= 0.0 {
        return Err(
            ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
                format!("{field} must be greater than zero"),
            )
            .with_operation(field),
        );
    }

    Ok(value)
}

// =============================================================================
// Basis operation identity
// =============================================================================

/// Opaque provider-independent identity of an executable basis operation.
///
/// The probabilistic layer does not interpret this value.
///
/// The executor is responsible for resolving it to a valid transformation of
/// canonical Zamani IR.
///
/// It deliberately does not contain a physical qubit number or provider name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BasisOperationId(String);

impl BasisOperationId {
    /// Creates a basis-operation identifier.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvalidIdentifier,
                    "basis operation identifier must not be empty",
                )
                .with_operation("basis_operation_id"),
            );
        }

        Ok(Self(value))
    }

    /// Returns the stable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for BasisOperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Quasi-probability term
// =============================================================================

/// One term of a quasi-probability decomposition.
///
/// The coefficient is allowed to be positive or negative, but must be finite
/// and non-zero.
///
/// Zero-coefficient terms are rejected instead of being retained because they
/// have no sampling probability and create ambiguous provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct QuasiProbabilityTerm {
    /// Executable basis-operation identity.
    pub operation: BasisOperationId,

    /// Signed quasi-probability coefficient.
    pub coefficient: f64,
}

impl QuasiProbabilityTerm {
    /// Creates a quasi-probability term.
    pub fn new(
        operation: BasisOperationId,
        coefficient: f64,
    ) -> ResilienceResult<Self> {
        finite(coefficient, "coefficient")?;

        if coefficient == 0.0 {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                    "quasi-probability coefficient must not be zero",
                )
                .with_operation("quasi_probability_term"),
            );
        }

        Ok(Self {
            operation,
            coefficient,
        })
    }

    /// Returns the absolute coefficient.
    #[must_use]
    pub fn absolute_coefficient(&self) -> f64 {
        self.coefficient.abs()
    }

    /// Returns whether the coefficient is positive.
    #[must_use]
    pub fn is_positive(&self) -> bool {
        self.coefficient.is_sign_positive()
    }

    /// Returns the coefficient sign as `+1` or `-1`.
    #[must_use]
    pub fn sign(&self) -> f64 {
        if self.coefficient.is_sign_positive() {
            1.0
        } else {
            -1.0
        }
    }
}

// =============================================================================
// Quasi-probability representation
// =============================================================================

/// Immutable quasi-probability decomposition.
///
/// The representation is normalized only for sampling:
///
///     p_i = |c_i| / gamma
///
/// The coefficients themselves are never modified.
///
/// No assumption is made that the coefficient sum is exactly one. Different
/// representation families can have different normalization conventions, and
/// the decomposition must remain faithful to the supplied mathematical model.
#[derive(Debug, Clone, PartialEq)]
pub struct QuasiProbabilityRepresentation {
    terms: Arc<[QuasiProbabilityTerm]>,
    l1_norm: f64,
}

impl QuasiProbabilityRepresentation {
    /// Constructs and validates a quasi-probability representation.
    ///
    /// Terms may be supplied in any deterministic order. The order is preserved
    /// and therefore forms part of deterministic sampling/provenance.
    pub fn new<I>(terms: I) -> ResilienceResult<Self>
    where
        I: IntoIterator<Item = QuasiProbabilityTerm>,
    {
        let terms: Vec<QuasiProbabilityTerm> = terms.into_iter().collect();

        if terms.is_empty() {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                    "quasi-probability representation must contain at least one term",
                )
                .with_operation("quasi_probability_representation"),
            );
        }

        let mut l1_norm = 0.0_f64;

        for term in &terms {
            finite(term.coefficient, "coefficient")?;

            let magnitude = term.absolute_coefficient();

            l1_norm = l1_norm.checked_add(magnitude).ok_or_else(|| {
                ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                    "quasi-probability L1 norm overflowed",
                )
                .with_operation("quasi_probability_l1_norm")
            })?;

            if !l1_norm.is_finite() {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                        "quasi-probability L1 norm is not finite",
                    )
                    .with_operation("quasi_probability_l1_norm"),
                );
            }
        }

        if l1_norm <= 0.0 {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                    "quasi-probability L1 norm must be greater than zero",
                )
                .with_operation("quasi_probability_l1_norm"),
            );
        }

        Ok(Self {
            terms: terms.into(),
            l1_norm,
        })
    }

    /// Returns all decomposition terms.
    #[must_use]
    pub fn terms(&self) -> &[QuasiProbabilityTerm] {
        &self.terms
    }

    /// Returns the number of decomposition terms.
    #[must_use]
    pub fn len(&self) -> usize {
        self.terms.len()
    }

    /// Returns whether the decomposition is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Returns the quasi-probability L1 norm.
    ///
    /// This is the fundamental PEC sampling-overhead factor.
    #[must_use]
    pub fn l1_norm(&self) -> f64 {
        self.l1_norm
    }

    /// Returns the squared L1 norm.
    ///
    /// This is a useful theoretical indicator of sampling variance overhead.
    pub fn l1_norm_squared(&self) -> ResilienceResult<f64> {
        let value = self.l1_norm * self.l1_norm;

        if !value.is_finite() {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                    "squared quasi-probability L1 norm is not finite",
                )
                .with_operation("quasi_probability_l1_norm_squared"),
            );
        }

        Ok(value)
    }

    /// Returns the normalized sampling probability for a term.
    pub fn probability(&self, index: usize) -> ResilienceResult<f64> {
        let term = self.terms.get(index).ok_or_else(|| {
            ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
                "quasi-probability term index is out of bounds",
            )
            .with_operation("quasi_probability_probability")
        })?;

        let probability = term.absolute_coefficient() / self.l1_norm;

        if !probability.is_finite() || probability < 0.0 {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                    "quasi-probability sampling probability is invalid",
                )
                .with_operation("quasi_probability_probability"),
            );
        }

        Ok(probability)
    }

    /// Samples a decomposition term from a caller-supplied uniform variate.
    ///
    /// `random` must satisfy:
    ///
    ///     0.0 <= random < 1.0
    ///
    /// The method is deterministic and has no hidden random source.
    ///
    /// The caller is responsible for providing an appropriate entropy source.
    pub fn sample(&self, random: f64) -> ResilienceResult<SampledTerm> {
        finite(random, "random")?;

        if !(0.0..1.0).contains(&random) {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvalidArgument,
                    "random variate must be in the half-open interval [0, 1)",
                )
                .with_operation("quasi_probability_sample"),
            );
        }

        let mut cumulative = 0.0_f64;

        for (index, term) in self.terms.iter().enumerate() {
            let probability = term.absolute_coefficient() / self.l1_norm;

            cumulative += probability;

            if !cumulative.is_finite() {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                        "sampling cumulative probability became non-finite",
                    )
                    .with_operation("quasi_probability_sample"),
                );
            }

            if random < cumulative {
                return Ok(SampledTerm {
                    index,
                    operation: term.operation.clone(),
                    sign: term.sign(),
                    probability,
                });
            }
        }

        // Floating-point accumulation can leave a tiny interval immediately
        // below one due to rounding. The representation is valid, so assigning
        // that residual to the final term is mathematically equivalent to
        // sampling from the normalized discrete distribution.
        let last_index = self.terms.len() - 1;
        let last = &self.terms[last_index];
        let probability = last.absolute_coefficient() / self.l1_norm;

        Ok(SampledTerm {
            index: last_index,
            operation: last.operation.clone(),
            sign: last.sign(),
            probability,
        })
    }
}

// =============================================================================
// Sampled term
// =============================================================================

/// Result of deterministic quasi-probability sampling.
#[derive(Debug, Clone, PartialEq)]
pub struct SampledTerm {
    /// Index in the original representation.
    pub index: usize,

    /// Selected basis operation.
    pub operation: BasisOperationId,

    /// Sign carried into the PEC estimator.
    pub sign: f64,

    /// Sampling probability of the selected term.
    pub probability: f64,
}

impl SampledTerm {
    /// Validates the sampled term.
    pub fn validate(&self) -> ResilienceResult<()> {
        if self.sign != 1.0 && self.sign != -1.0 {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvariantViolation,
                    "sampled quasi-probability sign must be either +1 or -1",
                )
                .with_operation("sampled_term"),
            );
        }

        finite(self.probability, "sample_probability")?;

        if self.probability <= 0.0 || self.probability > 1.0 {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvariantViolation,
                    "sample probability must be in the interval (0, 1]",
                )
                .with_operation("sampled_term"),
            );
        }

        Ok(())
    }
}

// =============================================================================
// Execution scope
// =============================================================================

/// Scope of a probabilistic mitigation request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbabilisticScope {
    /// Logical/program scope.
    pub scope: MitigationScope,
}

impl ProbabilisticScope {
    /// Creates a scope.
    #[must_use]
    pub const fn new(scope: MitigationScope) -> Self {
        Self { scope }
    }
}

// =============================================================================
// Execution specification
// =============================================================================

/// Immutable specification consumed by the mitigation executor.
///
/// It describes what needs to be sampled but does not execute anything.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilisticExecutionSpec {
    /// Stable mitigation schema.
    pub schema_id: &'static str,

    /// Schema version.
    pub schema_version: u16,

    /// Quasi-probability representation.
    pub representation: QuasiProbabilityRepresentation,

    /// Requested logical/program scope.
    pub scope: ProbabilisticScope,

    /// Whether deterministic replay is required.
    ///
    /// This does not create or control the random source. It tells the
    /// executor that the random source must be externally reproducible and
    /// recorded in provenance.
    pub deterministic_replay_required: bool,
}

impl ProbabilisticExecutionSpec {
    /// Creates an execution specification.
    pub fn new(
        representation: QuasiProbabilityRepresentation,
        scope: ProbabilisticScope,
        deterministic_replay_required: bool,
    ) -> ResilienceResult<Self> {
        let spec = Self {
            schema_id: PROBABILISTIC_MITIGATION_SCHEMA_ID,
            schema_version: PROBABILISTIC_MITIGATION_SCHEMA_VERSION,
            representation,
            scope,
            deterministic_replay_required,
        };

        spec.validate()?;

        Ok(spec)
    }

    /// Validates the complete specification.
    pub fn validate(&self) -> ResilienceResult<()> {
        if self.schema_id != PROBABILISTIC_MITIGATION_SCHEMA_ID {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::CompatibilityFailure,
                    "unsupported probabilistic mitigation schema identifier",
                )
                .with_operation("probabilistic_execution_spec"),
            );
        }

        if self.schema_version != PROBABILISTIC_MITIGATION_SCHEMA_VERSION {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::UnsupportedSchemaVersion,
                    "unsupported probabilistic mitigation schema version",
                )
                .with_operation("probabilistic_execution_spec"),
            );
        }

        if self.representation.is_empty() {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                    "probabilistic representation must not be empty",
                )
                .with_operation("probabilistic_execution_spec"),
            );
        }

        Ok(())
    }

    /// Returns the sampling overhead factor.
    #[must_use]
    pub fn sampling_overhead(&self) -> f64 {
        self.representation.l1_norm()
    }
}

// =============================================================================
// Estimator sample
// =============================================================================

/// One executed observation supplied to the PEC estimator.
///
/// The executor obtains the sign from the sampled quasi-probability term and
/// associates it with the measured observable value.
///
/// `observable` may be any finite scalar observable supplied by the caller.
/// This keeps the estimator independent of a particular measurement encoding.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProbabilisticObservation {
    /// Measured observable value.
    pub observable: f64,

    /// Signed quasi-probability factor, normally `+1` or `-1`.
    pub sign: f64,
}

impl ProbabilisticObservation {
    /// Creates and validates an observation.
    pub fn new(observable: f64, sign: f64) -> ResilienceResult<Self> {
        finite(observable, "observable")?;
        finite(sign, "sign")?;

        if sign != 1.0 && sign != -1.0 {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvalidArgument,
                    "probabilistic observation sign must be +1 or -1",
                )
                .with_operation("probabilistic_observation"),
            );
        }

        Ok(Self { observable, sign })
    }
}

// =============================================================================
// Estimator configuration
// =============================================================================

/// Statistical estimator configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilisticEstimatorConfig {
    /// Whether to compute an empirical standard error.
    pub calculate_standard_error: bool,

    /// Optional externally supplied confidence multiplier.
///
/// The multiplier is deliberately caller-controlled. This module does not
/// hard-code a confidence level.
    pub confidence_multiplier: Option<f64>,
}

impl Default for ProbabilisticEstimatorConfig {
    fn default() -> Self {
        Self {
            calculate_standard_error: true,
            confidence_multiplier: None,
        }
    }
}

impl ProbabilisticEstimatorConfig {
    /// Validates estimator configuration.
    pub fn validate(&self) -> ResilienceResult<()> {
        if let Some(multiplier) = self.confidence_multiplier {
            finite(multiplier, "confidence_multiplier")?;

            if multiplier <= 0.0 {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::InvalidConfiguration,
                        "confidence multiplier must be greater than zero",
                    )
                    .with_operation("probabilistic_estimator_config"),
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Estimation result
// =============================================================================

/// Result of probabilistic error cancellation estimation.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilisticEstimate {
    /// Mitigated observable estimate.
    pub estimate: f64,

    /// Optional empirical standard error.
    pub standard_error: Option<f64>,

    /// Optional confidence half-width.
    pub confidence_half_width: Option<f64>,

    /// Number of observations used.
    pub observation_count: usize,

    /// PEC L1 sampling overhead factor.
    pub sampling_overhead: f64,

    /// Squared PEC sampling overhead factor.
    pub variance_overhead: f64,
}

impl ProbabilisticEstimate {
    /// Validates the result.
    pub fn validate(&self) -> ResilienceResult<()> {
        finite(self.estimate, "estimate")?;
        finite(self.sampling_overhead, "sampling_overhead")?;
        finite(self.variance_overhead, "variance_overhead")?;

        if self.observation_count == 0 {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvariantViolation,
                    "probabilistic estimate must contain at least one observation",
                )
                .with_operation("probabilistic_estimate"),
            );
        }

        if let Some(value) = self.standard_error {
            finite(value, "standard_error")?;

            if value < 0.0 {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::InvariantViolation,
                        "standard error must not be negative",
                    )
                    .with_operation("probabilistic_estimate"),
                );
            }
        }

        if let Some(value) = self.confidence_half_width {
            finite(value, "confidence_half_width")?;

            if value < 0.0 {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::InvariantViolation,
                        "confidence half-width must not be negative",
                    )
                    .with_operation("probabilistic_estimate"),
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Estimator
// =============================================================================

/// Stateless PEC estimator.
///
/// The estimator stores no execution state and therefore can safely be used
/// concurrently by independent executions.
#[derive(Debug, Clone, Copy)]
pub struct ProbabilisticEstimator {
    configuration: ProbabilisticEstimatorConfig,
}

impl ProbabilisticEstimator {
    /// Creates an estimator.
    pub fn new(configuration: ProbabilisticEstimatorConfig) -> ResilienceResult<Self> {
        configuration.validate()?;

        Ok(Self { configuration })
    }

    /// Returns the estimator configuration.
    #[must_use]
    pub const fn configuration(&self) -> &ProbabilisticEstimatorConfig {
        &self.configuration
    }

    /// Estimates a mitigated observable.
    ///
    /// The estimator computes:
    ///
    ///     gamma * mean(sign_i * observable_i)
    ///
    /// where `gamma` is the representation L1 norm.
    ///
    /// The empirical standard error is calculated from the signed observations
    /// and then scaled by `gamma`.
    pub fn estimate(
        &self,
        representation: &QuasiProbabilityRepresentation,
        observations: &[ProbabilisticObservation],
    ) -> ResilienceResult<ProbabilisticEstimate> {
        if observations.is_empty() {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvalidArgument,
                    "at least one probabilistic observation is required",
                )
                .with_operation("probabilistic_estimate"),
            );
        }

        let gamma = representation.l1_norm();

        let mut sum = 0.0_f64;

        for observation in observations {
            finite(observation.observable, "observable")?;

            if observation.sign != 1.0 && observation.sign != -1.0 {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::InvalidArgument,
                        "observation sign must be +1 or -1",
                    )
                    .with_operation("probabilistic_estimate"),
                );
            }

            let signed_value = observation.sign * observation.observable;

            if !signed_value.is_finite() {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                        "signed observable became non-finite",
                    )
                    .with_operation("probabilistic_estimate"),
                );
            }

            sum += signed_value;

            if !sum.is_finite() {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                        "observable accumulation became non-finite",
                    )
                    .with_operation("probabilistic_estimate"),
                );
            }
        }

        let count = observations.len() as f64;
        let mean = sum / count;

        if !mean.is_finite() {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                    "probabilistic sample mean became non-finite",
                )
                .with_operation("probabilistic_estimate"),
            );
        }

        let estimate = gamma * mean;

        if !estimate.is_finite() {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                    "probabilistic mitigated estimate became non-finite",
                )
                .with_operation("probabilistic_estimate"),
            );
        }

        let standard_error = if self.configuration.calculate_standard_error {
            if observations.len() < 2 {
                Some(0.0)
            } else {
                let mut squared_deviation_sum = 0.0_f64;

                for observation in observations {
                    let value = observation.sign * observation.observable;
                    let deviation = value - mean;
                    let squared = deviation * deviation;

                    if !squared.is_finite() {
                        return Err(
                            ResilienceError::new(
                                ResilienceErrorCode::ArithmeticOverflow,
                                "probabilistic variance calculation became non-finite",
                            )
                            .with_operation("probabilistic_estimate"),
                        );
                    }

                    squared_deviation_sum += squared;

                    if !squared_deviation_sum.is_finite() {
                        return Err(
                            ResilienceError::new(
                                ResilienceErrorCode::ArithmeticOverflow,
                                "probabilistic variance accumulation became non-finite",
                            )
                            .with_operation("probabilistic_estimate"),
                        );
                    }
                }

                let sample_variance =
                    squared_deviation_sum / ((observations.len() - 1) as f64);

                if !sample_variance.is_finite() || sample_variance < 0.0 {
                    return Err(
                        ResilienceError::new(
                            ResilienceErrorCode::ArithmeticOverflow,
                            "probabilistic sample variance is invalid",
                        )
                        .with_operation("probabilistic_estimate"),
                    );
                }

                let standard_error_of_mean =
                    (sample_variance / count).sqrt();

                let result = gamma * standard_error_of_mean;

                if !result.is_finite() {
                    return Err(
                        ResilienceError::new(
                            ResilienceErrorCode::ArithmeticOverflow,
                            "probabilistic standard error became non-finite",
                        )
                        .with_operation("probabilistic_estimate"),
                    );
                }

                Some(result)
            }
        } else {
            None
        };

        let confidence_half_width = match (
            standard_error,
            self.configuration.confidence_multiplier,
        ) {
            (Some(error), Some(multiplier)) => {
                let value = error * multiplier;

                if !value.is_finite() {
                    return Err(
                        ResilienceError::new(
                            ResilienceErrorCode::ArithmeticOverflow,
                            "probabilistic confidence interval became non-finite",
                        )
                        .with_operation("probabilistic_estimate"),
                    );
                }

                Some(value)
            }

            _ => None,
        };

        let variance_overhead = representation.l1_norm_squared()?;

        let result = ProbabilisticEstimate {
            estimate,
            standard_error,
            confidence_half_width,
            observation_count: observations.len(),
            sampling_overhead: gamma,
            variance_overhead,
        };

        result.validate()?;

        Ok(result)
    }
}

// =============================================================================
// Strategy
// =============================================================================

/// Production probabilistic-error-cancellation strategy.
///
/// This strategy is declarative. It creates and validates PEC domain objects
/// but never executes the associated quantum operations.
#[derive(Debug, Clone)]
pub struct ProbabilisticErrorCancellationStrategy {
    descriptor: StrategyDescriptor,
    representation: QuasiProbabilityRepresentation,
    scope: MitigationScope,
    deterministic_replay_required: bool,
}

impl ProbabilisticErrorCancellationStrategy {
    /// Creates a PEC strategy from a validated quasi-probability
    /// representation.
    pub fn new(
        representation: QuasiProbabilityRepresentation,
        scope: MitigationScope,
        deterministic_replay_required: bool,
    ) -> ResilienceResult<Self> {
        let id = StrategyId::new(PROBABILISTIC_STRATEGY_ID).map_err(|_| {
            ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
                "invalid probabilistic strategy identifier",
            )
            .with_operation("probabilistic_strategy")
        })?;

        let descriptor = StrategyDescriptor {
            id,
            version: StrategyVersion::new(1, 0, 0),
            family: StrategyFamily::Probabilistic,
            phase: StrategyPhase::CrossPhase,
            description: Arc::from(
                "Probabilistic error cancellation using a validated \
                 quasi-probability representation",
            ),
            requirements: Arc::from([
                StrategyRequirement::ClassicalPostProcessing,
                StrategyRequirement::MeasurementResults,
                StrategyRequirement::RepeatedExecution,
                StrategyRequirement::StatisticalAnalysis,
                StrategyRequirement::Provenance,
                StrategyRequirement::CrossExecutionCorrelation,
                StrategyRequirement::VariantExecution,
                StrategyRequirement::ScopedExecution,
            ]),
            expected_overhead: Arc::from([
                ExpectedOverhead::new(
                    OverheadDimension::Executions,
                    OverheadLevel::VeryHigh,
                ),
                ExpectedOverhead::new(
                    OverheadDimension::ClassicalComputation,
                    OverheadLevel::Medium,
                ),
                ExpectedOverhead::new(
                    OverheadDimension::Variants,
                    OverheadLevel::High,
                ),
                ExpectedOverhead::new(
                    OverheadDimension::StatisticalSampling,
                    OverheadLevel::VeryHigh,
                ),
            ]),
            deterministic: deterministic_replay_required,
            requires_explicit_authorization: true,
        };

        Ok(Self {
            descriptor,
            representation,
            scope,
            deterministic_replay_required,
        })
    }

    /// Returns the quasi-probability representation.
    #[must_use]
    pub fn representation(&self) -> &QuasiProbabilityRepresentation {
        &self.representation
    }

    /// Returns the mitigation scope.
    #[must_use]
    pub const fn scope(&self) -> &MitigationScope {
        &self.scope
    }

    /// Returns whether deterministic replay is required.
    #[must_use]
    pub const fn deterministic_replay_required(&self) -> bool {
        self.deterministic_replay_required
    }

    /// Returns the PEC sampling overhead factor.
    #[must_use]
    pub fn sampling_overhead(&self) -> f64 {
        self.representation.l1_norm()
    }

    /// Returns a validated execution specification.
    pub fn execution_spec(&self) -> ResilienceResult<ProbabilisticExecutionSpec> {
        ProbabilisticExecutionSpec::new(
            self.representation.clone(),
            ProbabilisticScope::new(self.scope.clone()),
            self.deterministic_replay_required,
        )
    }

    /// Creates an estimator using the supplied configuration.
    pub fn estimator(
        &self,
        configuration: ProbabilisticEstimatorConfig,
    ) -> ResilienceResult<ProbabilisticEstimator> {
        ProbabilisticEstimator::new(configuration)
    }

    /// Estimates a result from executor-produced observations.
    pub fn estimate(
        &self,
        observations: &[ProbabilisticObservation],
        configuration: ProbabilisticEstimatorConfig,
    ) -> ResilienceResult<ProbabilisticEstimate> {
        let estimator = ProbabilisticEstimator::new(configuration)?;

        estimator.estimate(&self.representation, observations)
    }
}

impl MitigationStrategy for ProbabilisticErrorCancellationStrategy {
    fn descriptor(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    fn evaluate(&self, context: &StrategyContext) -> crate::quantum::resilience::mitigation::strategy::StrategyEvaluation {
        let base = <Self as MitigationStrategy>::descriptor(self);

        if !context.policy_authorized {
            return crate::quantum::resilience::mitigation::strategy::StrategyEvaluation::new(
                base,
                Applicability::RequiresPolicyValidation,
                vec![StrategyRequirement::ExplicitPolicyAuthorization],
            );
        }

        let evaluation = <Self as MitigationStrategy>::evaluate_default_for_context(
            base,
            context,
        );

        evaluation
    }
}

// =============================================================================
// Internal strategy-evaluation helper
// =============================================================================
//
// The repository's strategy contract currently supplies the normal evaluation
// algorithm directly inside the trait. Rust does not expose a callable
// "super" implementation for trait methods, so the helper below reproduces
// only the stable requirement evaluation needed by this concrete strategy.
// It deliberately delegates requirement semantics to the canonical
// `requirement_satisfied` function from strategy.rs.

trait StrategyEvaluationHelper {
    fn evaluate_default_for_context(
        descriptor: &StrategyDescriptor,
        context: &StrategyContext,
    ) -> crate::quantum::resilience::mitigation::strategy::StrategyEvaluation;
}

impl StrategyEvaluationHelper for ProbabilisticErrorCancellationStrategy {
    fn evaluate_default_for_context(
        descriptor: &StrategyDescriptor,
        context: &StrategyContext,
    ) -> crate::quantum::resilience::mitigation::strategy::StrategyEvaluation {
        let mut missing = Vec::new();

        for requirement in descriptor.requirements.iter() {
            if !crate::quantum::resilience::mitigation::strategy::requirement_satisfied(
                *requirement,
                context,
            ) {
                missing.push(*requirement);
            }
        }

        if !missing.is_empty() {
            return crate::quantum::resilience::mitigation::strategy::StrategyEvaluation::new(
                descriptor,
                Applicability::RequiresCapabilityValidation,
                missing,
            );
        }

        crate::quantum::resilience::mitigation::strategy::StrategyEvaluation::new(
            descriptor,
            Applicability::Applicable,
            Vec::new(),
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(
        name: &str,
    ) -> BasisOperationId {
        BasisOperationId::new(name).expect("valid operation")
    }

    fn representation() -> QuasiProbabilityRepresentation {
        QuasiProbabilityRepresentation::new([
            QuasiProbabilityTerm::new(operation("basis.a"), 0.75)
                .expect("valid term"),
            QuasiProbabilityTerm::new(operation("basis.b"), -0.25)
                .expect("valid term"),
        ])
        .expect("valid representation")
    }

    #[test]
    fn representation_computes_l1_norm() {
        let value = representation();

        assert_eq!(value.l1_norm(), 1.0);
        assert_eq!(
            value.l1_norm_squared().expect("finite"),
            1.0
        );
    }

    #[test]
    fn representation_accepts_negative_coefficients() {
        let value = representation();

        assert!(value.terms()[1].coefficient < 0.0);
        assert_eq!(value.terms()[1].sign(), -1.0);
    }

    #[test]
    fn zero_coefficient_is_rejected() {
        let result =
            QuasiProbabilityTerm::new(operation("basis"), 0.0);

        assert!(result.is_err());
    }

    #[test]
    fn non_finite_coefficient_is_rejected() {
        let result =
            QuasiProbabilityTerm::new(operation("basis"), f64::NAN);

        assert!(result.is_err());
    }

    #[test]
    fn empty_representation_is_rejected() {
        let result =
            QuasiProbabilityRepresentation::new(Vec::<QuasiProbabilityTerm>::new());

        assert!(result.is_err());
    }

    #[test]
    fn sampling_is_deterministic() {
        let value = representation();

        let first = value.sample(0.10).expect("sample");
        let second = value.sample(0.10).expect("sample");

        assert_eq!(first, second);
    }

    #[test]
    fn sampling_selects_expected_term() {
        let value = representation();

        let first = value.sample(0.10).expect("sample");
        assert_eq!(first.index, 0);

        let second = value.sample(0.90).expect("sample");
        assert_eq!(second.index, 1);
    }

    #[test]
    fn invalid_random_value_is_rejected() {
        let value = representation();

        assert!(value.sample(-0.1).is_err());
        assert!(value.sample(1.0).is_err());
        assert!(value.sample(f64::NAN).is_err());
    }

    #[test]
    fn estimator_recovers_simple_signed_mean() {
        let value = representation();

        let observations = [
            ProbabilisticObservation::new(1.0, 1.0)
                .expect("observation"),
            ProbabilisticObservation::new(1.0, -1.0)
                .expect("observation"),
        ];

        let estimator =
            ProbabilisticEstimator::new(
                ProbabilisticEstimatorConfig::default(),
            )
            .expect("estimator");

        let result = estimator
            .estimate(&value, &observations)
            .expect("estimate");

        assert_eq!(result.estimate, 0.0);
        assert_eq!(result.observation_count, 2);
        assert!(result.standard_error.is_some());
    }

    #[test]
    fn estimator_applies_l1_factor() {
        let representation =
            QuasiProbabilityRepresentation::new([
                QuasiProbabilityTerm::new(
                    operation("basis.a"),
                    2.0,
                )
                .expect("term"),
            ])
            .expect("representation");

        let observations = [
            ProbabilisticObservation::new(0.5, 1.0)
                .expect("observation"),
        ];

        let estimator =
            ProbabilisticEstimator::new(
                ProbabilisticEstimatorConfig {
                    calculate_standard_error: false,
                    confidence_multiplier: None,
                },
            )
            .expect("estimator");

        let result = estimator
            .estimate(&representation, &observations)
            .expect("estimate");

        assert_eq!(result.estimate, 1.0);
        assert_eq!(result.sampling_overhead, 2.0);
        assert_eq!(result.variance_overhead, 4.0);
    }

    #[test]
    fn confidence_multiplier_is_optional() {
        let configuration = ProbabilisticEstimatorConfig {
            calculate_standard_error: true,
            confidence_multiplier: Some(2.0),
        };

        let estimator =
            ProbabilisticEstimator::new(configuration)
                .expect("estimator");

        let value = representation();

        let observations = [
            ProbabilisticObservation::new(1.0, 1.0)
                .expect("observation"),
            ProbabilisticObservation::new(0.5, 1.0)
                .expect("observation"),
        ];

        let result = estimator
            .estimate(&value, &observations)
            .expect("estimate");

        assert!(result.confidence_half_width.is_some());
    }

    #[test]
    fn empty_observations_are_rejected() {
        let value = representation();

        let estimator =
            ProbabilisticEstimator::new(
                ProbabilisticEstimatorConfig::default(),
            )
            .expect("estimator");

        assert!(
            estimator
                .estimate(&value, &[])
                .is_err()
        );
    }

    #[test]
    fn strategy_exposes_probabilistic_family() {
        let strategy =
            ProbabilisticErrorCancellationStrategy::new(
                representation(),
                MitigationScope::Program,
                true,
            )
            .expect("strategy");

        assert_eq!(
            strategy.descriptor().family,
            StrategyFamily::Probabilistic
        );

        assert_eq!(
            strategy.descriptor().id.as_str(),
            PROBABILISTIC_STRATEGY_ID
        );

        assert!(strategy.descriptor().deterministic);
    }

    #[test]
    fn execution_spec_preserves_representation() {
        let strategy =
            ProbabilisticErrorCancellationStrategy::new(
                representation(),
                MitigationScope::Program,
                true,
            )
            .expect("strategy");

        let spec =
            strategy.execution_spec().expect("spec");

        assert_eq!(
            spec.representation.l1_norm(),
            strategy.sampling_overhead()
        );

        assert!(spec.deterministic_replay_required);
    }

    #[test]
    fn strategy_requires_policy_authorization() {
        let strategy =
            ProbabilisticErrorCancellationStrategy::new(
                representation(),
                MitigationScope::Program,
                true,
            )
            .expect("strategy");

        let context = StrategyContext::default();

        let evaluation = strategy.evaluate(&context);

        assert_eq!(
            evaluation.applicability,
            Applicability::RequiresPolicyValidation
        );
    }

    #[test]
    fn strategy_can_be_applicable_when_requirements_are_available() {
        let strategy =
            ProbabilisticErrorCancellationStrategy::new(
                representation(),
                MitigationScope::Program,
                true,
            )
            .expect("strategy");

        let context = StrategyContext {
            scope: MitigationScope::Program,
            measurement_results_available: true,
            repeated_execution_allowed: true,
            noise_scaling_available: false,
            parameter_variation_available: false,
            randomized_compilation_available: false,
            randomness_provenance_available: false,
            schedule_control_available: false,
            timing_information_available: false,
            pulse_control_available: false,
            statistical_analysis_available: true,
            provenance_available: true,
            cross_execution_correlation_available: true,
            policy_authorized: true,
        };

        let evaluation = strategy.evaluate(&context);

        assert_eq!(
            evaluation.applicability,
            Applicability::Applicable
        );
    }

    #[test]
    fn representation_supports_arbitrary_term_count() {
        let terms = (0..64usize)
            .map(|index| {
                QuasiProbabilityTerm::new(
                    operation(&format!("basis.{index}")),
                    1.0,
                )
                .expect("term")
            })
            .collect::<Vec<_>>();

        let representation =
            QuasiProbabilityRepresentation::new(terms)
                .expect("representation");

        assert_eq!(representation.len(), 64);
        assert_eq!(representation.l1_norm(), 64.0);
    }
}