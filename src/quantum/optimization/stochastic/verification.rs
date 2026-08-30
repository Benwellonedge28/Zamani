//! Zamani Quantum Optimization — Stochastic Verification
//!
//! Production-grade statistical verification infrastructure for optimized
//! quantum programs.
//!
//! # Architectural role
//!
//! This module belongs to:
//!
//! ```text
//! quantum::optimization
//!     └── stochastic
//!          └── verification
//! ```
//!
//! It provides a bounded, deterministic-capable, statistically principled
//! framework for randomized verification of optimization transformations.
//!
//! It does NOT define a quantum circuit representation and does NOT perform
//! exact circuit simulation itself.
//!
//! The intended dependency direction is:
//!
//! ```text
//!                    quantum::ir
//!                         │
//!                         ▼
//!                 optimization
//!                         │
//!                 ┌───────┴────────┐
//!                 │                │
//!                 ▼                ▼
//!          exact verification   stochastic
//!                               verification
//!                                    │
//!                                    ▼
//!                         TrialEvaluator / oracle
//! ```
//!
//! # Important semantic guarantee
//!
//! Stochastic verification is evidence, not an unconditional proof.
//!
//! A successful randomized verification means:
//!
//! - the configured randomized trials found no detected discrepancy;
//! - under the configured sampling policy;
//! - subject to the statistical confidence model;
//! - within the configured resource budget.
//!
//! It MUST NOT be reported as mathematical universal equivalence.
//!
//! Exact equivalence belongs to the exact/semantic verification subsystem.
//!
//! # Design goals
//!
//! This module is designed to:
//!
//! - work for tiny circuits;
//! - scale to extremely large circuits subject to available resources;
//! - avoid circuit-size assumptions;
//! - support deterministic reproducibility;
//! - support non-deterministic randomized verification;
//! - support early failure;
//! - support statistically justified early success;
//! - support fixed sample budgets;
//! - support adaptive/sequential sampling;
//! - provide confidence bounds;
//! - detect evaluator failures;
//! - prevent silent statistical misuse;
//! - avoid floating-point NaN/Infinity propagation;
//! - avoid integer overflow;
//! - provide machine-readable results;
//! - remain independent of circuit/IR implementation details;
//! - integrate with future optimization context/pipeline code;
//! - remain safe Rust only.
//!
//! # Statistical model
//!
//! The primary model treats each trial as a Bernoulli observation:
//!
//! ```text
//! success = no discrepancy detected
//! failure = discrepancy detected
//! ```
//!
//! For a mismatch probability `p`, Hoeffding's inequality gives a
//! distribution-free upper confidence bound for the empirical mismatch rate.
//!
//! If `m` mismatches are observed in `n` trials:
//!
//! ```text
//! p_hat = m / n
//!
//! upper_bound = p_hat + sqrt( ln(1 / alpha) / (2n) )
//! ```
//!
//! where:
//!
//! ```text
//! confidence = 1 - alpha
//! ```
//!
//! The bound is conservative and does not assume independence beyond the
//! configured trial model. Users must still ensure their evaluator's sampling
//! scheme matches the intended statistical interpretation.
//!
//! For the particularly useful zero-mismatch case:
//!
//! ```text
//! p_hat = 0
//! upper_bound = sqrt( ln(1 / alpha) / (2n) )
//! ```
//!
//! Therefore a zero-mismatch run can only be declared statistically
//! satisfactory when this upper bound is below the configured tolerance.
//!
//! # Why not claim absolute equivalence?
//!
//! Quantum circuit equivalence can be computationally difficult at scale.
//! Randomized methods are valuable because they can provide scalable evidence,
//! while exact methods remain available for cases where proof is required.
//!
//! This module therefore intentionally exposes:
//!
//! - `StatisticalPass`;
//! - `StatisticalFailure`;
//! - `Inconclusive`;
//! - `BudgetExhausted`;
//! - `EvaluatorError`;
//! - `InvalidConfiguration`;
//!
//! rather than pretending every successful randomized test is an exact proof.
//!
//! # Integration contract
//!
//! This file is intentionally independent of the concrete optimization IR.
//!
//! Future modules integrate through `TrialEvaluator`:
//!
//! ```text
//! verification.rs
//!       ▲
//!       │
//!       │ implements
//!       │
//! optimization/verification/*
//! exact circuit simulator
//! statevector evaluator
//! measurement evaluator
//! symbolic evaluator
//! differential backend evaluator
//! randomized witness generator
//! ```
//!
//! A future evaluator can use `quantum::ir` internally without this module
//! depending on it.
//!
//! This prevents a dependency cycle and means this file should not need to be
//! modified when the canonical Quantum IR evolves.
//!
//! # Optimization-context integration
//!
//! `OptimizationContext` already provides deterministic seed derivation,
//! verification work accounting, limits, cancellation and deadline handling.
//! A future integration layer can:
//!
//! 1. obtain the context-derived seed;
//! 2. construct `StochasticVerificationConfig`;
//! 3. create a `TrialEvaluator`;
//! 4. call `verify`;
//! 5. translate `StochasticVerificationResult` into the common optimization
//!    verification result.
//!
//! This module deliberately does not directly depend on `OptimizationContext`
//! so it remains independently testable and does not force the context to
//! depend on stochastic verification.
//!
//! # Resource scaling
//!
//! There is no artificial circuit-size limit in this module.
//!
//! The practical limits are:
//!
//! - `u64::MAX` trial count;
//! - configured maximum samples;
//! - evaluator cost;
//! - available CPU;
//! - available memory;
//! - configured wall-clock budget in the caller;
//! - operating-system/resource limits.
//!
//! Sample accounting uses checked arithmetic and saturating reporting where
//! appropriate.
//!
//! # Security and safety
//!
//! - no `unsafe`;
//! - no global mutable state;
//! - no ambient randomness when deterministic mode is selected;
//! - no unchecked arithmetic for resource counters;
//! - no implicit infinite loops;
//! - no unbounded collection growth;
//! - evaluator errors are never silently converted to successful trials;
//! - invalid statistical parameters are rejected.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Dependencies
//!
//! Uses the repository's existing `rand = 0.8` dependency.
//!
//! No additional crate is required.

use std::error::Error;
use std::fmt;

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

// =============================================================================
// Constants
// =============================================================================

/// Smallest supported confidence level.
///
/// A confidence level below this is statistically meaningless for this API
/// because the corresponding Hoeffding logarithm becomes numerically unstable
/// or operationally useless.
const MIN_CONFIDENCE: f64 = 0.5;

/// Largest supported confidence level.
///
/// `1.0` is excluded because it would require an infinite sample count for a
/// finite confidence bound.
const MAX_CONFIDENCE_EXCLUSIVE: f64 = 1.0;

/// Smallest supported mismatch tolerance.
const MIN_TOLERANCE: f64 = 0.0;

/// Largest supported mismatch tolerance.
const MAX_TOLERANCE: f64 = 1.0;

/// Maximum number of samples represented by this implementation.
///
/// `u64` is deliberately used instead of `usize` so the statistical contract
/// is independent of the host architecture.
const MAX_SAMPLE_COUNT: u64 = u64::MAX;

// =============================================================================
// Public result aliases
// =============================================================================

/// Result returned by stochastic verification.
pub type StochasticVerificationResult<T> = Result<T, StochasticVerificationError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by stochastic verification.
#[derive(Debug, Clone, PartialEq)]
pub enum StochasticVerificationError {
    /// Verification configuration is invalid.
    InvalidConfiguration {
        /// Name of the invalid configuration field.
        field: &'static str,

        /// Human-readable reason.
        reason: &'static str,
    },

    /// A numerical value was non-finite.
    NonFiniteValue {
        /// Name of the affected field.
        field: &'static str,
    },

    /// An evaluator reported an error.
    EvaluatorFailure {
        /// Trial number at which the evaluator failed.
        trial: u64,

        /// Stable human-readable error description.
        message: String,
    },

    /// The caller supplied an invalid sample count.
    InvalidSampleCount {
        /// Supplied count.
        value: u64,
    },

    /// The statistical calculation could not produce a valid finite result.
    StatisticalCalculationFailure {
        /// Stable explanation.
        reason: &'static str,
    },
}

impl fmt::Display for StochasticVerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { field, reason } => {
                write!(
                    formatter,
                    "invalid stochastic verification configuration `{field}`: {reason}"
                )
            }

            Self::NonFiniteValue { field } => {
                write!(
                    formatter,
                    "stochastic verification field `{field}` must be finite"
                )
            }

            Self::EvaluatorFailure { trial, message } => {
                write!(
                    formatter,
                    "stochastic verification evaluator failed at trial {trial}: {message}"
                )
            }

            Self::InvalidSampleCount { value } => {
                write!(
                    formatter,
                    "invalid stochastic verification sample count: {value}"
                )
            }

            Self::StatisticalCalculationFailure { reason } => {
                write!(
                    formatter,
                    "stochastic verification statistical calculation failed: {reason}"
                )
            }
        }
    }
}

impl Error for StochasticVerificationError {}

// =============================================================================
// Verification mode
// =============================================================================

/// Controls how randomized verification samples are scheduled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplingMode {
    /// Run exactly the requested number of samples unless a discrepancy or
    /// evaluator error causes earlier termination.
    Fixed,

    /// Continue sampling until either:
    ///
    /// - a discrepancy is detected;
    /// - statistical success is justified;
    /// - the maximum sample budget is exhausted.
    Sequential,

    /// Run at least `minimum_samples` and then continue until a statistical
    /// decision can be made or the maximum budget is exhausted.
    Adaptive,
}

impl Default for SamplingMode {
    fn default() -> Self {
        Self::Sequential
    }
}

// =============================================================================
// Verification decision
// =============================================================================

/// Final decision produced by stochastic verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationDecision {
    /// No discrepancy was detected and the configured statistical tolerance
    /// was satisfied.
    StatisticalPass,

    /// A discrepancy was detected.
    StatisticalFailure,

    /// Sampling completed without enough evidence for either decision.
    Inconclusive,

    /// The configured sample budget was exhausted.
    BudgetExhausted,
}

impl VerificationDecision {
    /// Returns `true` only for a statistically successful verification.
    pub fn is_success(self) -> bool {
        matches!(self, Self::StatisticalPass)
    }

    /// Returns `true` if a discrepancy was observed.
    pub fn is_failure(self) -> bool {
        matches!(self, Self::StatisticalFailure)
    }

    /// Returns `true` if the result does not establish equivalence evidence
    /// at the configured statistical threshold.
    pub fn is_inconclusive(self) -> bool {
        matches!(self, Self::Inconclusive | Self::BudgetExhausted)
    }
}

// =============================================================================
// Trial observation
// =============================================================================

/// Observation returned by one randomized verification trial.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VerificationObservation {
    /// `true` when the compared executions/semantics agreed for this trial.
    pub equivalent: bool,

    /// Optional normalized discrepancy score in `[0, 1]`.
    ///
    /// `0.0` means no observed discrepancy.
    ///
    /// `1.0` means maximal discrepancy under the evaluator's metric.
    ///
    /// The primary statistical decision is based on `equivalent`. The score is
    /// retained for diagnostics and future statistical methods.
    pub discrepancy: Option<f64>,
}

impl VerificationObservation {
    /// Constructs a successful trial observation.
    pub fn equivalent() -> Self {
        Self {
            equivalent: true,
            discrepancy: Some(0.0),
        }
    }

    /// Constructs a failed trial observation.
    pub fn mismatch() -> Self {
        Self {
            equivalent: false,
            discrepancy: Some(1.0),
        }
    }

    /// Constructs an observation with a custom discrepancy score.
    pub fn with_discrepancy(equivalent: bool, discrepancy: f64) -> Self {
        Self {
            equivalent,
            discrepancy: Some(discrepancy),
        }
    }

    /// Validates the observation.
    fn validate(&self) -> Result<(), StochasticVerificationError> {
        if let Some(value) = self.discrepancy {
            if !value.is_finite() {
                return Err(StochasticVerificationError::NonFiniteValue {
                    field: "discrepancy",
                });
            }

            if !(0.0..=1.0).contains(&value) {
                return Err(StochasticVerificationError::InvalidConfiguration {
                    field: "discrepancy",
                    reason: "must be within [0, 1]",
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Trial evaluator
// =============================================================================

/// Evaluates one randomized verification witness.
///
/// The evaluator owns all quantum-domain knowledge.
///
/// Examples of future implementations include:
///
/// - random computational-basis input generation;
/// - random stabilizer-state testing;
/// - random Pauli witnesses;
/// - random parameter assignments;
/// - random measurement experiments;
/// - statevector differential execution;
/// - backend differential execution.
///
/// The verifier itself does not know what a circuit or quantum state is.
pub trait TrialEvaluator {
    /// Evaluate one randomized witness.
    ///
    /// `trial_index` starts at zero and increases monotonically.
    ///
    /// `rng` is owned by the verifier and is deterministic when the verifier
    /// was configured with a deterministic seed.
    fn evaluate(
        &mut self,
        trial_index: u64,
        rng: &mut StdRng,
    ) -> Result<VerificationObservation, Box<dyn Error + Send + Sync>>;
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for stochastic verification.
#[derive(Debug, Clone, PartialEq)]
pub struct StochasticVerificationConfig {
    /// Sampling strategy.
    pub sampling_mode: SamplingMode,

    /// Minimum number of trials before statistical success may be declared.
    pub minimum_samples: u64,

    /// Maximum number of trials that may be executed.
    pub maximum_samples: u64,

    /// Required confidence in the upper mismatch-rate bound.
    ///
    /// Example:
    ///
    /// `0.99` means a 99% confidence bound.
    pub confidence: f64,

    /// Maximum acceptable mismatch probability.
    ///
    /// `0.0` requests evidence of zero mismatch probability under the
    /// statistical model. Since finite sampling cannot prove that exactly,
    /// this normally requires exact verification as a separate step.
    pub mismatch_tolerance: f64,

    /// Optional deterministic seed.
    ///
    /// When present, identical configuration and evaluator behavior produce
    /// the same witness sequence.
    pub seed: Option<u64>,

    /// Whether the verifier may stop immediately when a mismatch is observed.
    pub fail_fast_on_mismatch: bool,

    /// Whether the verifier may stop as soon as the configured statistical
    /// success condition is satisfied.
    pub stop_on_statistical_success: bool,
}

impl Default for StochasticVerificationConfig {
    fn default() -> Self {
        Self {
            sampling_mode: SamplingMode::Sequential,
            minimum_samples: 32,
            maximum_samples: 4096,
            confidence: 0.99,
            mismatch_tolerance: 0.0,
            seed: None,
            fail_fast_on_mismatch: true,
            stop_on_statistical_success: true,
        }
    }
}

impl StochasticVerificationConfig {
    /// Creates a conservative production configuration.
    pub fn production() -> Self {
        Self {
            sampling_mode: SamplingMode::Adaptive,
            minimum_samples: 64,
            maximum_samples: 16_384,
            confidence: 0.999,
            mismatch_tolerance: 0.0,
            seed: None,
            fail_fast_on_mismatch: true,
            stop_on_statistical_success: true,
        }
    }

    /// Creates a deterministic configuration suitable for compiler tests.
    pub fn deterministic(seed: u64) -> Self {
        Self {
            seed: Some(seed),
            ..Self::default()
        }
    }

    /// Creates a configuration appropriate for a small regression test.
    pub fn test(seed: u64) -> Self {
        Self {
            sampling_mode: SamplingMode::Fixed,
            minimum_samples: 16,
            maximum_samples: 16,
            confidence: 0.95,
            mismatch_tolerance: 0.0,
            seed: Some(seed),
            fail_fast_on_mismatch: true,
            stop_on_statistical_success: false,
        }
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), StochasticVerificationError> {
        if self.minimum_samples == 0 {
            return Err(StochasticVerificationError::InvalidConfiguration {
                field: "minimum_samples",
                reason: "must be greater than zero",
            });
        }

        if self.maximum_samples == 0 {
            return Err(StochasticVerificationError::InvalidConfiguration {
                field: "maximum_samples",
                reason: "must be greater than zero",
            });
        }

        if self.minimum_samples > self.maximum_samples {
            return Err(StochasticVerificationError::InvalidConfiguration {
                field: "minimum_samples",
                reason: "must not exceed maximum_samples",
            });
        }

        if self.maximum_samples > MAX_SAMPLE_COUNT {
            return Err(StochasticVerificationError::InvalidSampleCount {
                value: self.maximum_samples,
            });
        }

        if !self.confidence.is_finite() {
            return Err(StochasticVerificationError::NonFiniteValue {
                field: "confidence",
            });
        }

        if self.confidence < MIN_CONFIDENCE
            || self.confidence >= MAX_CONFIDENCE_EXCLUSIVE
        {
            return Err(StochasticVerificationError::InvalidConfiguration {
                field: "confidence",
                reason: "must be in [0.5, 1.0)",
            });
        }

        if !self.mismatch_tolerance.is_finite() {
            return Err(StochasticVerificationError::NonFiniteValue {
                field: "mismatch_tolerance",
            });
        }

        if !(MIN_TOLERANCE..=MAX_TOLERANCE).contains(&self.mismatch_tolerance) {
            return Err(StochasticVerificationError::InvalidConfiguration {
                field: "mismatch_tolerance",
                reason: "must be in [0, 1]",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Statistical summary
// =============================================================================

/// Statistical information accumulated during verification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatisticalSummary {
    /// Number of completed trials.
    pub samples: u64,

    /// Number of equivalent observations.
    pub matches: u64,

    /// Number of mismatching observations.
    pub mismatches: u64,

    /// Empirical mismatch rate.
    pub empirical_mismatch_rate: f64,

    /// One-sided Hoeffding upper confidence bound for the mismatch rate.
    pub upper_mismatch_bound: f64,

    /// Configured confidence.
    pub confidence: f64,

    /// Configured tolerance.
    pub mismatch_tolerance: f64,
}

impl StatisticalSummary {
    /// Constructs a summary from counts.
    pub fn from_counts(
        samples: u64,
        matches: u64,
        mismatches: u64,
        confidence: f64,
        mismatch_tolerance: f64,
    ) -> Result<Self, StochasticVerificationError> {
        if samples == 0 {
            return Err(StochasticVerificationError::InvalidSampleCount {
                value: samples,
            });
        }

        if matches > samples || mismatches > samples {
            return Err(StochasticVerificationError::StatisticalCalculationFailure {
                reason: "match/mismatch count exceeds sample count",
            });
        }

        if matches
            .checked_add(mismatches)
            .map(|value| value != samples)
            .unwrap_or(true)
        {
            return Err(StochasticVerificationError::StatisticalCalculationFailure {
                reason: "match and mismatch counts do not equal sample count",
            });
        }

        if !confidence.is_finite()
            || confidence < MIN_CONFIDENCE
            || confidence >= MAX_CONFIDENCE_EXCLUSIVE
        {
            return Err(StochasticVerificationError::InvalidConfiguration {
                field: "confidence",
                reason: "must be in [0.5, 1.0)",
            });
        }

        if !mismatch_tolerance.is_finite()
            || !(MIN_TOLERANCE..=MAX_TOLERANCE).contains(&mismatch_tolerance)
        {
            return Err(StochasticVerificationError::InvalidConfiguration {
                field: "mismatch_tolerance",
                reason: "must be in [0, 1]",
            });
        }

        let empirical = mismatches as f64 / samples as f64;

        let alpha = 1.0 - confidence;

        if !(0.0..1.0).contains(&alpha) || !alpha.is_finite() {
            return Err(
                StochasticVerificationError::StatisticalCalculationFailure {
                    reason: "invalid significance level",
                },
            );
        }

        let logarithmic_term = (1.0 / alpha).ln();

        if !logarithmic_term.is_finite() || logarithmic_term < 0.0 {
            return Err(
                StochasticVerificationError::StatisticalCalculationFailure {
                    reason: "invalid logarithmic confidence term",
                },
            );
        }

        let radius = (logarithmic_term / (2.0 * samples as f64)).sqrt();

        if !radius.is_finite() {
            return Err(
                StochasticVerificationError::StatisticalCalculationFailure {
                    reason: "non-finite confidence radius",
                },
            );
        }

        let upper_bound = (empirical + radius).min(1.0);

        Ok(Self {
            samples,
            matches,
            mismatches,
            empirical_mismatch_rate: empirical,
            upper_mismatch_bound: upper_bound,
            confidence,
            mismatch_tolerance,
        })
    }

    /// Returns true when the upper mismatch bound satisfies the configured
    /// tolerance.
    pub fn satisfies_tolerance(&self) -> bool {
        self.upper_mismatch_bound <= self.mismatch_tolerance
    }

    /// Returns the estimated statistical radius.
    pub fn confidence_radius(&self) -> f64 {
        (self.upper_mismatch_bound - self.empirical_mismatch_rate).max(0.0)
    }
}

// =============================================================================
// Verification result
// =============================================================================

/// Complete result of one stochastic verification run.
#[derive(Debug, Clone, PartialEq)]
pub struct StochasticVerificationReport {
    /// Final verification decision.
    pub decision: VerificationDecision,

    /// Statistical summary.
    pub statistics: StatisticalSummary,

    /// Deterministic seed used by this run, when one was configured.
    pub seed: Option<u64>,

    /// Number of evaluator invocations.
    pub evaluator_calls: u64,

    /// Number of evaluator failures.
    ///
    /// Currently a verifier run terminates on the first evaluator error, so
    /// this is either zero or one.
    pub evaluator_failures: u64,

    /// Whether execution stopped early.
    pub stopped_early: bool,

    /// Stable reason for termination.
    pub termination_reason: TerminationReason,
}

impl StochasticVerificationReport {
    /// Returns true only when statistical equivalence criteria were satisfied.
    pub fn is_success(&self) -> bool {
        self.decision.is_success()
    }

    /// Returns true when a discrepancy was observed.
    pub fn is_failure(&self) -> bool {
        self.decision.is_failure()
    }

    /// Returns true when the result is inconclusive.
    pub fn is_inconclusive(&self) -> bool {
        self.decision.is_inconclusive()
    }
}

// =============================================================================
// Termination reason
// =============================================================================

/// Explains why randomized verification terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminationReason {
    /// Statistical success threshold was reached.
    StatisticalSuccess,

    /// A discrepancy was observed.
    MismatchDetected,

    /// Maximum sample budget was reached.
    MaximumSamplesReached,

    /// Fixed sampling mode completed its requested budget.
    FixedBudgetCompleted,

    /// Evaluator returned an error.
    EvaluatorError,

    /// No decision was possible with the available evidence.
    InsufficientEvidence,
}

// =============================================================================
// Verifier
// =============================================================================

/// Production stochastic verifier.
///
/// The verifier is deliberately generic over the quantum-domain evaluator.
/// This keeps statistical infrastructure independent of the canonical Quantum
/// IR and allows the same verifier to be used for:
///
/// - statevector checks;
/// - measurement checks;
/// - parameterized circuit checks;
/// - backend differential testing;
/// - randomized witness generation;
/// - compiler regression testing.
#[derive(Debug, Clone)]
pub struct StochasticVerifier {
    config: StochasticVerificationConfig,
}

impl StochasticVerifier {
    /// Creates a verifier after validating configuration.
    pub fn new(
        config: StochasticVerificationConfig,
    ) -> Result<Self, StochasticVerificationError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the verifier configuration.
    pub fn config(&self) -> &StochasticVerificationConfig {
        &self.config
    }

    /// Verifies using the configured randomized sampling strategy.
    pub fn verify<E>(
        &self,
        evaluator: &mut E,
    ) -> StochasticVerificationResult<StochasticVerificationReport>
    where
        E: TrialEvaluator,
    {
        self.config.validate()?;

        let seed = self.config.seed.unwrap_or_else(random_seed);

        let mut rng = StdRng::seed_from_u64(seed);

        let mut matches = 0_u64;
        let mut mismatches = 0_u64;
        let mut evaluator_calls = 0_u64;
        let mut evaluator_failures = 0_u64;

        let mut trial_index = 0_u64;

        loop {
            if trial_index >= self.config.maximum_samples {
                let statistics = StatisticalSummary::from_counts(
                    trial_index,
                    matches,
                    mismatches,
                    self.config.confidence,
                    self.config.mismatch_tolerance,
                )?;

                let decision = if statistics.satisfies_tolerance() {
                    VerificationDecision::StatisticalPass
                } else {
                    VerificationDecision::BudgetExhausted
                };

                let reason = if self.config.sampling_mode == SamplingMode::Fixed {
                    TerminationReason::FixedBudgetCompleted
                } else {
                    TerminationReason::MaximumSamplesReached
                };

                return Ok(StochasticVerificationReport {
                    decision,
                    statistics,
                    seed: Some(seed),
                    evaluator_calls,
                    evaluator_failures,
                    stopped_early: false,
                    termination_reason: reason,
                });
            }

            let observation = match evaluator.evaluate(trial_index, &mut rng) {
                Ok(value) => value,
                Err(error) => {
                    evaluator_failures = evaluator_failures.saturating_add(1);

                    return Err(StochasticVerificationError::EvaluatorFailure {
                        trial: trial_index,
                        message: error.to_string(),
                    });
                }
            };

            evaluator_calls = evaluator_calls.saturating_add(1);

            observation.validate()?;

            if observation.equivalent {
                matches = matches.saturating_add(1);
            } else {
                mismatches = mismatches.saturating_add(1);
            }

            trial_index = trial_index.saturating_add(1);

            if !observation.equivalent && self.config.fail_fast_on_mismatch {
                let statistics = StatisticalSummary::from_counts(
                    trial_index,
                    matches,
                    mismatches,
                    self.config.confidence,
                    self.config.mismatch_tolerance,
                )?;

                return Ok(StochasticVerificationReport {
                    decision: VerificationDecision::StatisticalFailure,
                    statistics,
                    seed: Some(seed),
                    evaluator_calls,
                    evaluator_failures,
                    stopped_early: true,
                    termination_reason: TerminationReason::MismatchDetected,
                });
            }

            if trial_index < self.config.minimum_samples {
                continue;
            }

            let statistics = StatisticalSummary::from_counts(
                trial_index,
                matches,
                mismatches,
                self.config.confidence,
                self.config.mismatch_tolerance,
            )?;

            if self.config.stop_on_statistical_success
                && statistics.satisfies_tolerance()
            {
                return Ok(StochasticVerificationReport {
                    decision: VerificationDecision::StatisticalPass,
                    statistics,
                    seed: Some(seed),
                    evaluator_calls,
                    evaluator_failures,
                    stopped_early: trial_index < self.config.maximum_samples,
                    termination_reason: TerminationReason::StatisticalSuccess,
                });
            }

            if self.config.sampling_mode == SamplingMode::Fixed
                && trial_index >= self.config.maximum_samples
            {
                return Ok(StochasticVerificationReport {
                    decision: if statistics.satisfies_tolerance() {
                        VerificationDecision::StatisticalPass
                    } else {
                        VerificationDecision::BudgetExhausted
                    },
                    statistics,
                    seed: Some(seed),
                    evaluator_calls,
                    evaluator_failures,
                    stopped_early: false,
                    termination_reason: TerminationReason::FixedBudgetCompleted,
                });
            }
        }
    }

    /// Runs exactly `samples` trials.
    ///
    /// This helper is useful for benchmarking and reproducible regression
    /// tests. It overrides the configured maximum only after validating that
    /// the requested number does not exceed the configured maximum.
    pub fn verify_fixed<E>(
        &self,
        evaluator: &mut E,
        samples: u64,
    ) -> StochasticVerificationResult<StochasticVerificationReport>
    where
        E: TrialEvaluator,
    {
        if samples == 0 || samples > self.config.maximum_samples {
            return Err(StochasticVerificationError::InvalidSampleCount {
                value: samples,
            });
        }

        let mut config = self.config.clone();

        config.sampling_mode = SamplingMode::Fixed;
        config.minimum_samples = samples;
        config.maximum_samples = samples;
        config.stop_on_statistical_success = false;

        Self::new(config)?.verify(evaluator)
    }
}

// =============================================================================
// Seed generation
// =============================================================================

/// Generates a seed for non-deterministic verification.
///
/// The random source is intentionally isolated here. Deterministic compiler
/// builds should always provide an explicit seed, normally derived from
/// `OptimizationContext`.
fn random_seed() -> u64 {
    let mut bytes = [0_u8; 8];

    let mut rng = rand::thread_rng();

    rng.fill_bytes(&mut bytes);

    u64::from_le_bytes(bytes)
}

// =============================================================================
// Convenience evaluator
// =============================================================================

/// A simple closure-backed evaluator.
///
/// This is useful for unit tests and for future integration adapters where
/// allocating a dedicated evaluator type would add no value.
pub struct ClosureEvaluator<F> {
    function: F,
}

impl<F> ClosureEvaluator<F> {
    /// Creates a closure-backed evaluator.
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<F, E> TrialEvaluator for ClosureEvaluator<F>
where
    F: FnMut(u64, &mut StdRng) -> Result<VerificationObservation, E>,
    E: Error + Send + Sync + 'static,
{
    fn evaluate(
        &mut self,
        trial_index: u64,
        rng: &mut StdRng,
    ) -> Result<VerificationObservation, Box<dyn Error + Send + Sync>> {
        (self.function)(trial_index, rng)
            .map_err(|error| Box::new(error) as Box<dyn Error + Send + Sync>)
    }
}

// =============================================================================
// Utility functions
// =============================================================================

/// Calculates the minimum number of samples required to make a zero-mismatch
/// Hoeffding upper bound no larger than `tolerance`.
///
/// The calculation is:
///
/// ```text
/// sqrt(ln(1 / alpha) / (2n)) <= tolerance
/// ```
///
/// therefore:
///
/// ```text
/// n >= ln(1 / alpha) / (2 tolerance²)
/// ```
///
/// Returns `None` when the requested tolerance is exactly zero because no
/// finite randomized sample count can establish a zero probability of
/// mismatch.
pub fn required_samples_for_zero_mismatch(
    confidence: f64,
    tolerance: f64,
) -> Result<Option<u64>, StochasticVerificationError> {
    if !confidence.is_finite() {
        return Err(StochasticVerificationError::NonFiniteValue {
            field: "confidence",
        });
    }

    if !(MIN_CONFIDENCE..MAX_CONFIDENCE_EXCLUSIVE).contains(&confidence) {
        return Err(StochasticVerificationError::InvalidConfiguration {
            field: "confidence",
            reason: "must be in [0.5, 1.0)",
        });
    }

    if !tolerance.is_finite() {
        return Err(StochasticVerificationError::NonFiniteValue {
            field: "tolerance",
        });
    }

    if !(MIN_TOLERANCE..=MAX_TOLERANCE).contains(&tolerance) {
        return Err(StochasticVerificationError::InvalidConfiguration {
            field: "tolerance",
            reason: "must be in [0, 1]",
        });
    }

    if tolerance == 0.0 {
        return Ok(None);
    }

    let alpha = 1.0 - confidence;
    let numerator = (1.0 / alpha).ln();
    let denominator = 2.0 * tolerance * tolerance;

    let required = numerator / denominator;

    if !required.is_finite() || required < 0.0 {
        return Err(
            StochasticVerificationError::StatisticalCalculationFailure {
                reason: "required sample calculation overflowed or became non-finite",
            },
        );
    }

    if required >= u64::MAX as f64 {
        return Ok(Some(u64::MAX));
    }

    let rounded_up = required.ceil();

    if rounded_up <= 1.0 {
        return Ok(Some(1));
    }

    Ok(Some(rounded_up as u64))
}

/// Computes a one-sided Hoeffding upper confidence bound directly.
///
/// This is exposed so future verification modules can use exactly the same
/// statistical convention without duplicating the formula.
pub fn hoeffding_upper_bound(
    samples: u64,
    mismatches: u64,
    confidence: f64,
) -> Result<f64, StochasticVerificationError> {
    if samples == 0 {
        return Err(StochasticVerificationError::InvalidSampleCount {
            value: samples,
        });
    }

    if mismatches > samples {
        return Err(
            StochasticVerificationError::StatisticalCalculationFailure {
                reason: "mismatch count exceeds sample count",
            },
        );
    }

    if !confidence.is_finite() {
        return Err(StochasticVerificationError::NonFiniteValue {
            field: "confidence",
        });
    }

    if !(MIN_CONFIDENCE..MAX_CONFIDENCE_EXCLUSIVE).contains(&confidence) {
        return Err(StochasticVerificationError::InvalidConfiguration {
            field: "confidence",
            reason: "must be in [0.5, 1.0)",
        });
    }

    let empirical = mismatches as f64 / samples as f64;

    let alpha = 1.0 - confidence;

    let radius = ((1.0 / alpha).ln() / (2.0 * samples as f64)).sqrt();

    if !radius.is_finite() {
        return Err(
            StochasticVerificationError::StatisticalCalculationFailure {
                reason: "confidence radius is non-finite",
            },
        );
    }

    Ok((empirical + radius).min(1.0))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestError;

    impl fmt::Display for TestError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("test evaluator failure")
        }
    }

    impl Error for TestError {}

    #[test]
    fn default_configuration_is_valid() {
        assert!(StochasticVerificationConfig::default().validate().is_ok());
    }

    #[test]
    fn production_configuration_is_valid() {
        assert!(StochasticVerificationConfig::production().validate().is_ok());
    }

    #[test]
    fn deterministic_configuration_is_reproducible() {
        let first = StochasticVerificationConfig::deterministic(42);
        let second = StochasticVerificationConfig::deterministic(42);

        assert_eq!(first, second);
    }

    #[test]
    fn rejects_zero_maximum_samples() {
        let config = StochasticVerificationConfig {
            maximum_samples: 0,
            ..StochasticVerificationConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn rejects_minimum_greater_than_maximum() {
        let config = StochasticVerificationConfig {
            minimum_samples: 100,
            maximum_samples: 10,
            ..StochasticVerificationConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn rejects_invalid_confidence() {
        let config = StochasticVerificationConfig {
            confidence: 1.0,
            ..StochasticVerificationConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn rejects_invalid_tolerance() {
        let config = StochasticVerificationConfig {
            mismatch_tolerance: 1.1,
            ..StochasticVerificationConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn zero_mismatch_bound_decreases_with_samples() {
        let small = hoeffding_upper_bound(10, 0, 0.99).unwrap();
        let large = hoeffding_upper_bound(10_000, 0, 0.99).unwrap();

        assert!(large < small);
    }

    #[test]
    fn mismatch_increases_upper_bound() {
        let no_mismatch = hoeffding_upper_bound(100, 0, 0.99).unwrap();
        let mismatch = hoeffding_upper_bound(100, 10, 0.99).unwrap();

        assert!(mismatch > no_mismatch);
    }

    #[test]
    fn zero_tolerance_requires_infinite_randomized_evidence() {
        let result = required_samples_for_zero_mismatch(0.99, 0.0).unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn positive_tolerance_requires_finite_samples() {
        let result = required_samples_for_zero_mismatch(0.99, 0.01).unwrap();

        assert!(result.unwrap() > 0);
    }

    #[test]
    fn equivalent_evaluator_passes_with_reasonable_tolerance() {
        let config = StochasticVerificationConfig {
            sampling_mode: SamplingMode::Fixed,
            minimum_samples: 128,
            maximum_samples: 128,
            confidence: 0.95,
            mismatch_tolerance: 0.2,
            seed: Some(1234),
            fail_fast_on_mismatch: true,
            stop_on_statistical_success: false,
        };

        let verifier = StochasticVerifier::new(config).unwrap();

        let mut evaluator = ClosureEvaluator::new(
            |_trial, _rng| -> Result<VerificationObservation, TestError> {
                Ok(VerificationObservation::equivalent())
            },
        );

        let report = verifier.verify(&mut evaluator).unwrap();

        assert!(report.is_success());
        assert_eq!(report.statistics.mismatches, 0);
        assert_eq!(report.statistics.samples, 128);
    }

    #[test]
    fn mismatch_is_detected_immediately_when_fail_fast_is_enabled() {
        let config = StochasticVerificationConfig {
            sampling_mode: SamplingMode::Sequential,
            minimum_samples: 1,
            maximum_samples: 100,
            confidence: 0.95,
            mismatch_tolerance: 0.2,
            seed: Some(7),
            fail_fast_on_mismatch: true,
            stop_on_statistical_success: true,
        };

        let verifier = StochasticVerifier::new(config).unwrap();

        let mut evaluator = ClosureEvaluator::new(
            |_trial, _rng| -> Result<VerificationObservation, TestError> {
                Ok(VerificationObservation::mismatch())
            },
        );

        let report = verifier.verify(&mut evaluator).unwrap();

        assert!(report.is_failure());
        assert_eq!(report.statistics.mismatches, 1);
        assert!(report.stopped_early);
        assert_eq!(
            report.termination_reason,
            TerminationReason::MismatchDetected
        );
    }

    #[test]
    fn evaluator_error_is_not_silently_accepted() {
        let config = StochasticVerificationConfig {
            minimum_samples: 1,
            maximum_samples: 10,
            ..StochasticVerificationConfig::default()
        };

        let verifier = StochasticVerifier::new(config).unwrap();

        let mut evaluator = ClosureEvaluator::new(
            |_trial, _rng| -> Result<VerificationObservation, TestError> {
                Err(TestError)
            },
        );

        let result = verifier.verify(&mut evaluator);

        assert!(matches!(
            result,
            Err(StochasticVerificationError::EvaluatorFailure { .. })
        ));
    }

    #[test]
    fn fixed_verification_uses_exact_requested_sample_count() {
        let config = StochasticVerificationConfig {
            maximum_samples: 100,
            minimum_samples: 1,
            confidence: 0.95,
            mismatch_tolerance: 0.5,
            seed: Some(99),
            ..StochasticVerificationConfig::default()
        };

        let verifier = StochasticVerifier::new(config).unwrap();

        let mut evaluator = ClosureEvaluator::new(
            |_trial, _rng| -> Result<VerificationObservation, TestError> {
                Ok(VerificationObservation::equivalent())
            },
        );

        let report = verifier.verify_fixed(&mut evaluator, 25).unwrap();

        assert_eq!(report.statistics.samples, 25);
        assert_eq!(
            report.termination_reason,
            TerminationReason::FixedBudgetCompleted
        );
    }

    #[test]
    fn observation_rejects_non_finite_discrepancy() {
        let observation =
            VerificationObservation::with_discrepancy(true, f64::NAN);

        assert!(observation.validate().is_err());
    }

    #[test]
    fn observation_rejects_out_of_range_discrepancy() {
        let observation =
            VerificationObservation::with_discrepancy(true, 1.1);

        assert!(observation.validate().is_err());
    }

    #[test]
    fn statistical_summary_requires_matching_counts() {
        let result =
            StatisticalSummary::from_counts(10, 9, 0, 0.95, 0.1);

        assert!(result.is_err());
    }

    #[test]
    fn statistical_summary_reports_empirical_rate() {
        let summary =
            StatisticalSummary::from_counts(100, 90, 10, 0.95, 0.2).unwrap();

        assert!((summary.empirical_mismatch_rate - 0.1).abs() < 1e-12);
        assert!(summary.upper_mismatch_bound >= 0.1);
    }

    #[test]
    fn verification_decision_helpers_are_consistent() {
        assert!(VerificationDecision::StatisticalPass.is_success());
        assert!(VerificationDecision::StatisticalFailure.is_failure());
        assert!(VerificationDecision::Inconclusive.is_inconclusive());
        assert!(VerificationDecision::BudgetExhausted.is_inconclusive());
    }
}