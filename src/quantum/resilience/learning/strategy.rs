//! Zamani Quantum Resilience — Learned Strategy Selection
//!
//! Path:
//!     src/quantum/resilience/learning/strategy.rs
//!
//! # Purpose
//!
//! This module converts verified historical/predictive evidence into a
//! deterministic, policy-aware ranking of resilience strategies.
//!
//! It is intentionally a SELECTION layer.
//!
//! It does not:
//!
//! - execute recovery;
//! - implement QEC;
//! - implement routing;
//! - implement scheduling;
//! - implement optimization;
//! - implement mitigation;
//! - diagnose hardware faults;
//! - define quantum semantics;
//! - define physical or logical qubit identities;
//! - bypass policy;
//! - bypass verification;
//! - treat a machine-learning prediction as truth;
//! - require a machine-learning model;
//! - impose a fixed number of strategies;
//! - impose a fixed number of quantum resources;
//! - impose a fixed number of retries;
//! - impose a fixed machine size.
//!
//! The intended flow is:
//!
//! ```text
//! verified history / predictor
//!              │
//!              ▼
//!      StrategyEvidence
//!              │
//!              ▼
//!      StrategySelector
//!              │
//!              ├── policy constraints
//!              ├── safety constraints
//!              ├── capability feasibility
//!              ├── prediction confidence
//!              └── historical evidence
//!              │
//!              ▼
//!      RankedStrategy list
//!              │
//!              ▼
//!          planner
//!              │
//!              ▼
//!       policy / recovery
//! ```
//!
//! # Architectural boundary
//!
//! `learning/strategy.rs` answers:
//!
//!     "Given the currently available evidence, which already-defined
//!      resilience strategies should the planner consider first?"
//!
//! It does NOT answer:
//!
//!     "How do I perform the recovery?"
//!
//! That second question belongs to `planning`, `adaptation`, `recovery`,
//! `mitigation`, `QEC`, `routing`, `scheduling`, or the appropriate
//! authoritative subsystem.
//!
//! # Safety rule
//!
//! A learned recommendation is advisory evidence.
//!
//! It MUST NOT override:
//!
//! - semantic constraints;
//! - security policy;
//! - resource limits;
//! - explicit caller policy;
//! - capability requirements;
//! - verification requirements;
//! - recovery safety gates.
//!
//! A strategy can therefore receive a high learned score and still be rejected
//! from the candidate set.
//!
//! # Write once, scale everywhere
//!
//! This module contains no architectural machine-size limit.
//!
//! It must not contain constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_STRATEGIES
//! MAX_DEVICES
//! MAX_RETRIES
//! ```
//!
//! Collections grow according to caller/resource availability.
//!
//! A caller that needs bounded work can supply an explicit policy/budget.
//!
//! The selector itself remains generic over the number of strategies,
//! observations, resources, and execution targets.
//!
//! # Determinism
//!
//! Selection is deterministic when:
//!
//! - candidate order is deterministic;
//! - evidence is deterministic;
//! - policy is deterministic;
//! - the predictor is deterministic;
//! - tie-breaking metadata is deterministic.
//!
//! No clock, filesystem, network, global mutable state, or random generator is
//! accessed here.
//!
//! If stochastic exploration is desired, randomness must be supplied explicitly
//! by a higher-level component and recorded as part of execution provenance.
//!
//! # Learning safety
//!
//! Learning is subordinate to verification.
//!
//! Only verified outcomes should be converted into durable learning evidence.
//!
//! Unverified outcomes may be supplied as explicitly marked evidence, but this
//! selector will not treat them as equivalent to verified evidence.
//!
//! # Integration
//!
//! `learning/model.rs`
//!     Supplies model/prediction abstractions.
//!
//! `learning/features.rs`
//!     Supplies feature representations used to create predictions.
//!
//! `learning/predictor.rs`
//!     Supplies predictions.
//!
//! `learning/feedback.rs`
//!     Supplies verified historical feedback.
//!
//! `planning/*`
//!     Consumes the ranking produced here.
//!
//! `policy/*`
//!     Supplies safety, feasibility, objective, and budget constraints.
//!
//! `verification/*`
//!     Establishes whether an observed outcome is safe to use as learning
//!     feedback.
//!
//! `history/*`
//!     Supplies historical execution/recovery evidence.
//!
//! This file deliberately uses local, stable contracts so it can be integrated
//! with those modules without requiring those modules to depend on concrete
//! learning implementations.
//!
//! # Quantum identity rule
//!
//! This module does not define quantum identity types.
//!
//! If a future strategy-specific implementation needs a quantum identity, it
//! MUST use the canonical repository types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module itself only handles opaque resilience strategy identities.
//!
//! # Rust contract
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

/// Stable semantic schema identifier.
pub const LEARNED_STRATEGY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.learning.strategy";

/// Current semantic schema version.
///
/// This is not a hardware-size limit.
pub const LEARNED_STRATEGY_SCHEMA_VERSION: u32 = 1;

/// An opaque, stable identifier for a resilience strategy.
///
/// Strategy IDs belong to the strategy registry/planner boundary. They are
/// deliberately not provider-specific and do not encode a quantum resource
/// identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StrategyId(Arc<str>);

impl StrategyId {
    /// Creates a strategy ID from explicit caller/registry input.
    ///
    /// Empty or whitespace-only IDs are rejected because they cannot provide
    /// stable provenance.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, StrategyIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StrategyIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the stable textual identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for StrategyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors produced while constructing [`StrategyId`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StrategyIdError {
    /// The identifier was empty or whitespace-only.
    Empty,
}

impl fmt::Display for StrategyIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("strategy identifier is empty"),
        }
    }
}

impl std::error::Error for StrategyIdError {}

/// A stable classification of the kind of strategy being ranked.
///
/// The enum deliberately describes broad semantics rather than concrete
/// providers or hardware technologies.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StrategyKind {
    /// Repeat an execution when the operation is known to be safely retryable.
    Retry,

    /// Restart from an explicitly valid execution boundary.
    Restart,

    /// Resume from a valid checkpoint or execution boundary.
    Resume,

    /// Roll back to a previously validated state.
    Rollback,

    /// Request a new logical-to-physical mapping.
    Remap,

    /// Request a new physical route.
    Reroute,

    /// Request a new schedule.
    Reschedule,

    /// Request recompilation for a changed target/capability set.
    Recompile,

    /// Request another optimization pass/profile.
    Reoptimize,

    /// Request an alternative QEC configuration.
    QecAdaptation,

    /// Request an error-mitigation strategy.
    Mitigation,

    /// Move execution to another compatible resource.
    Migration,

    /// Apply a domain-defined compensating action.
    Compensation,

    /// Explicitly quarantine a degraded resource.
    Quarantine,

    /// Stop execution because continued execution is not safe.
    Abort,

    /// Extension point for a future strategy family.
    Custom,
}

impl StrategyKind {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Retry => "retry",
            Self::Restart => "restart",
            Self::Resume => "resume",
            Self::Rollback => "rollback",
            Self::Remap => "remap",
            Self::Reroute => "reroute",
            Self::Reschedule => "reschedule",
            Self::Recompile => "recompile",
            Self::Reoptimize => "reoptimize",
            Self::QecAdaptation => "qec_adaptation",
            Self::Mitigation => "mitigation",
            Self::Migration => "migration",
            Self::Compensation => "compensation",
            Self::Quarantine => "quarantine",
            Self::Abort => "abort",
            Self::Custom => "custom",
        }
    }

    /// Returns whether the strategy can potentially change the execution
    /// realization without changing the logical program.
    #[must_use]
    pub const fn is_adaptive(self) -> bool {
        matches!(
            self,
            Self::Remap
                | Self::Reroute
                | Self::Reschedule
                | Self::Recompile
                | Self::Reoptimize
                | Self::QecAdaptation
                | Self::Mitigation
                | Self::Migration
        )
    }

    /// Returns whether the strategy can terminate execution.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Abort)
    }
}

impl fmt::Display for StrategyKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Whether evidence has passed the verification boundary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EvidenceTrust {
    /// The evidence has been explicitly verified.
    Verified,

    /// The evidence is internally consistent but has not passed final
    /// verification.
    Unverified,

    /// The evidence is known to be stale, contradictory, or otherwise unsafe.
    Rejected,
}

impl EvidenceTrust {
    /// Returns whether the evidence is eligible for normal learning influence.
    #[must_use]
    pub const fn is_learning_eligible(self) -> bool {
        matches!(self, Self::Verified)
    }
}

impl fmt::Display for EvidenceTrust {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Verified => "verified",
            Self::Unverified => "unverified",
            Self::Rejected => "rejected",
        };

        formatter.write_str(value)
    }
}

/// A normalized probability-like score.
///
/// The value is guaranteed to be finite and within `[0, 1]`.
///
/// This type prevents NaN and infinity from contaminating strategy ordering.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Probability(f64);

impl Probability {
    /// Zero probability.
    pub const ZERO: Self = Self(0.0);

    /// Certain probability.
    pub const ONE: Self = Self(1.0);

    /// Creates a probability after validating its domain.
    pub fn new(value: f64) -> Result<Self, ProbabilityError> {
        if !value.is_finite() {
            return Err(ProbabilityError::NonFinite);
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(ProbabilityError::OutOfRange);
        }

        Ok(Self(value))
    }

    /// Creates a probability after clamping finite values.
    ///
    /// This constructor is useful for model adapters where a model's floating
    /// point output may have tiny numerical excursions outside the legal range.
    ///
    /// Non-finite values are rejected instead of being silently converted.
    pub fn clamped(value: f64) -> Result<Self, ProbabilityError> {
        if !value.is_finite() {
            return Err(ProbabilityError::NonFinite);
        }

        Ok(Self(value.clamp(0.0, 1.0)))
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl Default for Probability {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Eq for Probability {}

impl Ord for Probability {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(Ordering::Equal)
    }
}

/// Errors produced by [`Probability`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProbabilityError {
    /// The supplied value was NaN or infinite.
    NonFinite,

    /// The supplied value was outside `[0, 1]`.
    OutOfRange,
}

impl fmt::Display for ProbabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("probability must be finite"),
            Self::OutOfRange => formatter.write_str("probability must be within [0, 1]"),
        }
    }
}

impl std::error::Error for ProbabilityError {}

/// A signed utility score.
///
/// Utility is deliberately not restricted to `[0, 1]` because different
/// objectives may combine benefits and costs.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Utility(f64);

impl Utility {
    /// Creates a finite utility value.
    pub fn new(value: f64) -> Result<Self, UtilityError> {
        if !value.is_finite() {
            return Err(UtilityError::NonFinite);
        }

        Ok(Self(value))
    }

    /// Returns the underlying utility.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl Eq for Utility {}

impl Ord for Utility {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(Ordering::Equal)
    }
}

impl Default for Utility {
    fn default() -> Self {
        Self(0.0)
    }
}

/// Errors produced by [`Utility`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UtilityError {
    /// The supplied value was NaN or infinite.
    NonFinite,
}

impl fmt::Display for UtilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("utility must be finite"),
        }
    }
}

impl std::error::Error for UtilityError {}

/// A generic strategy identifier plus classification.
///
/// Keeping identity and classification separate allows registry implementations
/// to add new strategy implementations without changing this module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrategyDescriptor {
    id: StrategyId,
    kind: StrategyKind,
    label: Arc<str>,
}

impl StrategyDescriptor {
    /// Creates a strategy descriptor.
    pub fn new(
        id: StrategyId,
        kind: StrategyKind,
        label: impl Into<Arc<str>>,
    ) -> Result<Self, StrategyDescriptorError> {
        let label = label.into();

        if label.trim().is_empty() {
            return Err(StrategyDescriptorError::EmptyLabel);
        }

        Ok(Self { id, kind, label })
    }

    /// Returns the stable strategy ID.
    #[must_use]
    pub fn id(&self) -> &StrategyId {
        &self.id
    }

    /// Returns the broad strategy kind.
    #[must_use]
    pub const fn kind(&self) -> StrategyKind {
        self.kind
    }

    /// Returns the human-readable label.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_ref()
    }
}

/// Errors produced while constructing [`StrategyDescriptor`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StrategyDescriptorError {
    /// The display label is empty.
    EmptyLabel,
}

impl fmt::Display for StrategyDescriptorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLabel => formatter.write_str("strategy label is empty"),
        }
    }
}

impl std::error::Error for StrategyDescriptorError {}

/// Prediction associated with a strategy.
///
/// A prediction is deliberately decomposed into:
///
/// - predicted success;
/// - confidence;
/// - expected utility.
///
/// This prevents a highly confident prediction from being confused with a
/// high probability of success.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrategyPrediction {
    success_probability: Probability,
    confidence: Probability,
    expected_utility: Utility,
}

impl StrategyPrediction {
    /// Creates a validated prediction.
    pub const fn new(
        success_probability: Probability,
        confidence: Probability,
        expected_utility: Utility,
    ) -> Self {
        Self {
            success_probability,
            confidence,
            expected_utility,
        }
    }

    /// Returns predicted probability of success.
    #[must_use]
    pub const fn success_probability(self) -> Probability {
        self.success_probability
    }

    /// Returns confidence in the prediction.
    #[must_use]
    pub const fn confidence(self) -> Probability {
        self.confidence
    }

    /// Returns predicted utility.
    #[must_use]
    pub const fn expected_utility(self) -> Utility {
        self.expected_utility
    }
}

impl Default for StrategyPrediction {
    fn default() -> Self {
        Self {
            success_probability: Probability::ZERO,
            confidence: Probability::ZERO,
            expected_utility: Utility::default(),
        }
    }
}

/// Historical evidence for a strategy.
///
/// Only verified outcomes are allowed to influence the normal learned score.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HistoricalEvidence {
    /// Number of verified successful observations.
    successes: u64,

    /// Number of verified unsuccessful observations.
    failures: u64,

    /// Confidence that the historical evidence remains relevant.
    relevance: Probability,

    /// Trust level of the evidence source.
    trust: EvidenceTrust,
}

impl HistoricalEvidence {
    /// Creates historical evidence.
    pub const fn new(
        successes: u64,
        failures: u64,
        relevance: Probability,
        trust: EvidenceTrust,
    ) -> Self {
        Self {
            successes,
            failures,
            relevance,
            trust,
        }
    }

    /// Returns verified successful observations.
    #[must_use]
    pub const fn successes(self) -> u64 {
        self.successes
    }

    /// Returns verified failed observations.
    #[must_use]
    pub const fn failures(self) -> u64 {
        self.failures
    }

    /// Returns evidence relevance.
    #[must_use]
    pub const fn relevance(self) -> Probability {
        self.relevance
    }

    /// Returns evidence trust.
    #[must_use]
    pub const fn trust(self) -> EvidenceTrust {
        self.trust
    }

    /// Returns whether the evidence can influence normal learning.
    #[must_use]
    pub const fn is_learning_eligible(self) -> bool {
        self.trust.is_learning_eligible()
    }

    /// Computes the empirical success rate.
    ///
    /// No fixed sample-size assumption is made.
    #[must_use]
    pub fn success_rate(self) -> Probability {
        let total = self.successes.saturating_add(self.failures);

        if total == 0 {
            return Probability::ZERO;
        }

        let rate = self.successes as f64 / total as f64;

        // The operands are bounded by u64 and the result is mathematically
        // within [0, 1]. `clamped` cannot fail for finite integer-derived
        // values.
        Probability::clamped(rate).unwrap_or(Probability::ZERO)
    }
}

impl Default for HistoricalEvidence {
    fn default() -> Self {
        Self {
            successes: 0,
            failures: 0,
            relevance: Probability::ZERO,
            trust: EvidenceTrust::Unverified,
        }
    }
}

/// Policy weights used by the selector.
///
/// These values are supplied by the caller. No policy is hard-coded into the
/// learning subsystem.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrategyWeights {
    /// Weight assigned to predicted success.
    success: f64,

    /// Weight assigned to prediction confidence.
    confidence: f64,

    /// Weight assigned to historical success.
    history: f64,

    /// Weight assigned to expected utility.
    utility: f64,

    /// Weight assigned to evidence relevance.
    relevance: f64,
}

impl StrategyWeights {
    /// Creates strategy weights.
    ///
    /// Weights may be positive or negative, allowing a caller to express
    /// objective direction explicitly.
    pub fn new(
        success: f64,
        confidence: f64,
        history: f64,
        utility: f64,
        relevance: f64,
    ) -> Result<Self, StrategyWeightsError> {
        let values = [success, confidence, history, utility, relevance];

        if values.iter().any(|value| !value.is_finite()) {
            return Err(StrategyWeightsError::NonFinite);
        }

        Ok(Self {
            success,
            confidence,
            history,
            utility,
            relevance,
        })
    }

    /// Returns the default neutral weighting.
    ///
    /// The default does not encode a machine-specific preference.
    #[must_use]
    pub const fn neutral() -> Self {
        Self {
            success: 1.0,
            confidence: 0.0,
            history: 0.0,
            utility: 0.0,
            relevance: 0.0,
        }
    }

    #[must_use]
    pub const fn success(self) -> f64 {
        self.success
    }

    #[must_use]
    pub const fn confidence(self) -> f64 {
        self.confidence
    }

    #[must_use]
    pub const fn history(self) -> f64 {
        self.history
    }

    #[must_use]
    pub const fn utility(self) -> f64 {
        self.utility
    }

    #[must_use]
    pub const fn relevance(self) -> f64 {
        self.relevance
    }
}

impl Default for StrategyWeights {
    fn default() -> Self {
        Self::neutral()
    }
}

/// Errors produced by [`StrategyWeights`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StrategyWeightsError {
    /// One or more supplied weights were NaN or infinite.
    NonFinite,
}

impl fmt::Display for StrategyWeightsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("strategy weights must be finite"),
        }
    }
}

impl std::error::Error for StrategyWeightsError {}

/// Safety constraints applied before learned ranking.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StrategySafety {
    /// Whether unverified evidence may influence ranking.
    ///
    /// This defaults to false and should normally remain false.
    allow_unverified_evidence: bool,

    /// Whether terminal strategies may be ranked.
    allow_terminal_strategies: bool,

    /// Whether non-adaptive strategies may be ranked.
    allow_non_adaptive_strategies: bool,
}

impl StrategySafety {
    /// Creates a safety configuration.
    pub const fn new(
        allow_unverified_evidence: bool,
        allow_terminal_strategies: bool,
        allow_non_adaptive_strategies: bool,
    ) -> Self {
        Self {
            allow_unverified_evidence,
            allow_terminal_strategies,
            allow_non_adaptive_strategies,
        }
    }

    /// Strict production-safe defaults.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            allow_unverified_evidence: false,
            allow_terminal_strategies: false,
            allow_non_adaptive_strategies: true,
        }
    }

    #[must_use]
    pub const fn allow_unverified_evidence(self) -> bool {
        self.allow_unverified_evidence
    }

    #[must_use]
    pub const fn allow_terminal_strategies(self) -> bool {
        self.allow_terminal_strategies
    }

    #[must_use]
    pub const fn allow_non_adaptive_strategies(self) -> bool {
        self.allow_non_adaptive_strategies
    }
}

impl Default for StrategySafety {
    fn default() -> Self {
        Self::strict()
    }
}

/// Strategy-specific eligibility supplied by the planner/policy layer.
///
/// This keeps capability and semantic validation outside the learning layer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrategyEligibility {
    /// The strategy may be considered.
    Eligible,

    /// The strategy is not currently feasible.
    Infeasible,

    /// The strategy violates an explicit policy constraint.
    PolicyDenied,

    /// The strategy cannot be used without additional information.
    Indeterminate,
}

impl StrategyEligibility {
    /// Returns whether the strategy can enter the ranked set.
    #[must_use]
    pub const fn is_eligible(self) -> bool {
        matches!(self, Self::Eligible)
    }
}

impl fmt::Display for StrategyEligibility {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Eligible => "eligible",
            Self::Infeasible => "infeasible",
            Self::PolicyDenied => "policy_denied",
            Self::Indeterminate => "indeterminate",
        };

        formatter.write_str(value)
    }
}

/// All information required to rank one strategy.
///
/// The selector does not inspect hardware or quantum IR directly. Those
/// subsystems provide eligibility/evidence through the planner-facing contract.
#[derive(Clone, Debug, PartialEq)]
pub struct StrategyCandidate {
    descriptor: StrategyDescriptor,
    prediction: StrategyPrediction,
    history: HistoricalEvidence,
    eligibility: StrategyEligibility,
    explicit_priority: i64,
}

impl StrategyCandidate {
    /// Creates a candidate.
    pub fn new(
        descriptor: StrategyDescriptor,
        prediction: StrategyPrediction,
        history: HistoricalEvidence,
        eligibility: StrategyEligibility,
    ) -> Self {
        Self {
            descriptor,
            prediction,
            history,
            eligibility,
            explicit_priority: 0,
        }
    }

    /// Sets an explicit policy/registry priority.
    ///
    /// Explicit priority is only a tie-breaker after the learned score.
    #[must_use]
    pub const fn with_explicit_priority(mut self, priority: i64) -> Self {
        self.explicit_priority = priority;
        self
    }

    #[must_use]
    pub fn descriptor(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    #[must_use]
    pub const fn prediction(&self) -> StrategyPrediction {
        self.prediction
    }

    #[must_use]
    pub const fn history(&self) -> HistoricalEvidence {
        self.history
    }

    #[must_use]
    pub const fn eligibility(&self) -> StrategyEligibility {
        self.eligibility
    }

    #[must_use]
    pub const fn explicit_priority(&self) -> i64 {
        self.explicit_priority
    }
}

/// Configuration for learned strategy selection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrategySelectionConfig {
    weights: StrategyWeights,
    safety: StrategySafety,

    /// Minimum prediction confidence required for prediction influence.
    ///
    /// A value of zero means confidence does not gate prediction influence.
    minimum_prediction_confidence: Probability,

    /// Minimum evidence relevance required for historical influence.
    minimum_history_relevance: Probability,
}

impl StrategySelectionConfig {
    /// Creates a validated selection configuration.
    pub fn new(
        weights: StrategyWeights,
        safety: StrategySafety,
        minimum_prediction_confidence: Probability,
        minimum_history_relevance: Probability,
    ) -> Self {
        Self {
            weights,
            safety,
            minimum_prediction_confidence,
            minimum_history_relevance,
        }
    }

    /// Strict production configuration with neutral non-safety assumptions.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            weights: StrategyWeights::neutral(),
            safety: StrategySafety::strict(),
            minimum_prediction_confidence: Probability::ZERO,
            minimum_history_relevance: Probability::ZERO,
        }
    }

    #[must_use]
    pub const fn weights(self) -> StrategyWeights {
        self.weights
    }

    #[must_use]
    pub const fn safety(self) -> StrategySafety {
        self.safety
    }

    #[must_use]
    pub const fn minimum_prediction_confidence(self) -> Probability {
        self.minimum_prediction_confidence
    }

    #[must_use]
    pub const fn minimum_history_relevance(self) -> Probability {
        self.minimum_history_relevance
    }
}

impl Default for StrategySelectionConfig {
    fn default() -> Self {
        Self::strict()
    }
}

/// Why a candidate was excluded.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExclusionReason {
    /// The planner/policy marked it infeasible.
    Infeasible,

    /// The planner/policy denied it.
    PolicyDenied,

    /// Feasibility could not be established.
    Indeterminate,

    /// A terminal strategy was not permitted.
    TerminalStrategyDenied,

    /// An unverified prediction/evidence source was not permitted.
    UnverifiedEvidenceDenied,

    /// The strategy is adaptive but its current policy does not permit it.
    AdaptiveStrategyDenied,
}

impl fmt::Display for ExclusionReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Infeasible => "infeasible",
            Self::PolicyDenied => "policy_denied",
            Self::Indeterminate => "indeterminate",
            Self::TerminalStrategyDenied => "terminal_strategy_denied",
            Self::UnverifiedEvidenceDenied => "unverified_evidence_denied",
            Self::AdaptiveStrategyDenied => "adaptive_strategy_denied",
        };

        formatter.write_str(value)
    }
}

/// The result of ranking one eligible strategy.
#[derive(Clone, Debug, PartialEq)]
pub struct RankedStrategy {
    descriptor: StrategyDescriptor,
    score: Utility,
    prediction: StrategyPrediction,
    history: HistoricalEvidence,
    rank: usize,
}

impl RankedStrategy {
    #[must_use]
    pub fn descriptor(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    #[must_use]
    pub const fn score(&self) -> Utility {
        self.score
    }

    #[must_use]
    pub const fn prediction(&self) -> StrategyPrediction {
        self.prediction
    }

    #[must_use]
    pub const fn history(&self) -> HistoricalEvidence {
        self.history
    }

    #[must_use]
    pub const fn rank(&self) -> usize {
        self.rank
    }
}

/// An excluded candidate retained for auditability.
///
/// The selector never silently drops a candidate.
#[derive(Clone, Debug, PartialEq)]
pub struct ExcludedStrategy {
    descriptor: StrategyDescriptor,
    reason: ExclusionReason,
}

impl ExcludedStrategy {
    #[must_use]
    pub fn descriptor(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    #[must_use]
    pub const fn reason(&self) -> ExclusionReason {
        self.reason
    }
}

/// Complete deterministic ranking result.
///
/// Both selected and excluded candidates are retained so planners and
/// provenance systems can explain why a strategy was or was not considered.
#[derive(Clone, Debug, PartialEq)]
pub struct StrategyRanking {
    ranked: Vec<RankedStrategy>,
    excluded: Vec<ExcludedStrategy>,
}

impl StrategyRanking {
    /// Returns ranked candidates in descending score order.
    #[must_use]
    pub fn ranked(&self) -> &[RankedStrategy] {
        &self.ranked
    }

    /// Returns candidates excluded before learned ranking.
    #[must_use]
    pub fn excluded(&self) -> &[ExcludedStrategy] {
        &self.excluded
    }

    /// Returns the highest-ranked strategy, if one exists.
    #[must_use]
    pub fn best(&self) -> Option<&RankedStrategy> {
        self.ranked.first()
    }

    /// Returns whether at least one strategy is available.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ranked.is_empty()
    }

    /// Number of ranked candidates.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ranked.len()
    }
}

/// Errors produced by learned strategy ranking.
#[derive(Debug)]
pub enum StrategySelectionError {
    /// A strategy contained invalid/non-finite learned data.
    InvalidScore(UtilityError),

    /// A strategy ID occurred more than once in the same selection request.
    DuplicateStrategyId(StrategyId),

    /// The selector could not produce a finite score.
    NonFiniteComputedScore,

    /// A supplied strategy collection was too large for the caller-provided
    /// execution budget.
    BudgetExceeded,
}

impl fmt::Display for StrategySelectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidScore(error) => write!(formatter, "invalid strategy score: {error}"),
            Self::DuplicateStrategyId(id) => {
                write!(formatter, "duplicate strategy identifier: {id}")
            }
            Self::NonFiniteComputedScore => {
                formatter.write_str("computed strategy score is non-finite")
            }
            Self::BudgetExceeded => formatter.write_str("strategy selection budget exceeded"),
        }
    }
}

impl std::error::Error for StrategySelectionError {}

impl From<UtilityError> for StrategySelectionError {
    fn from(error: UtilityError) -> Self {
        Self::InvalidScore(error)
    }
}

/// Optional caller-owned budget for ranking work.
///
/// This is deliberately external to the selector's architecture. There is no
/// universal "maximum strategy count" baked into this file.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectionBudget {
    /// Maximum number of candidates the caller permits this invocation to
    /// inspect.
    ///
    /// `None` means no selector-level candidate-count budget.
    candidate_limit: Option<usize>,
}

impl SelectionBudget {
    /// Creates an unrestricted selector-level budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            candidate_limit: None,
        }
    }

    /// Creates an explicit candidate-count budget.
    #[must_use]
    pub const fn candidates(limit: usize) -> Self {
        Self {
            candidate_limit: Some(limit),
        }
    }

    #[must_use]
    pub const fn candidate_limit(self) -> Option<usize> {
        self.candidate_limit
    }
}

impl Default for SelectionBudget {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// A learned strategy selector.
///
/// The selector is stateless. This is intentional:
///
/// - no hidden global model;
/// - no mutable singleton;
/// - no implicit clock;
/// - no implicit random source;
/// - deterministic replay;
/// - easy testing;
/// - safe concurrent use by ownership/borrowing rules.
///
/// The caller supplies evidence and policy explicitly.
#[derive(Clone, Copy, Debug, Default)]
pub struct StrategySelector;

impl StrategySelector {
    /// Creates a stateless selector.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Ranks strategy candidates.
    ///
    /// The algorithm is:
    ///
    /// 1. enforce caller budget;
    /// 2. reject duplicate strategy IDs;
    /// 3. apply policy/safety eligibility;
    /// 4. compute learned utility;
    /// 5. sort deterministically;
    /// 6. assign one-based ranks.
    ///
    /// No execution or recovery action occurs here.
    pub fn rank<I>(
        &self,
        candidates: I,
        config: StrategySelectionConfig,
        budget: SelectionBudget,
    ) -> Result<StrategyRanking, StrategySelectionError>
    where
        I: IntoIterator<Item = StrategyCandidate>,
    {
        let mut candidates = candidates.into_iter();

        let mut seen = BTreeMap::<StrategyId, ()>::new();
        let mut ranked = Vec::new();
        let mut excluded = Vec::new();
        let mut inspected = 0usize;

        while let Some(candidate) = candidates.next() {
            if let Some(limit) = budget.candidate_limit {
                if inspected >= limit {
                    return Err(StrategySelectionError::BudgetExceeded);
                }
            }

            inspected = inspected.saturating_add(1);

            let id = candidate.descriptor.id().clone();

            if seen.insert(id.clone(), ()).is_some() {
                return Err(StrategySelectionError::DuplicateStrategyId(id));
            }

            if let Some(reason) = exclusion_reason(&candidate, config.safety) {
                excluded.push(ExcludedStrategy {
                    descriptor: candidate.descriptor,
                    reason,
                });
                continue;
            }

            let score = compute_score(&candidate, config)?;

            ranked.push(RankedStrategy {
                descriptor: candidate.descriptor,
                score,
                prediction: candidate.prediction,
                history: candidate.history,
                rank: 0,
            });
        }

        ranked.sort_by(compare_ranked);

        for (index, candidate) in ranked.iter_mut().enumerate() {
            candidate.rank = index.saturating_add(1);
        }

        Ok(StrategyRanking { ranked, excluded })
    }
}

/// Determines whether a candidate must be excluded before learned scoring.
fn exclusion_reason(
    candidate: &StrategyCandidate,
    safety: StrategySafety,
) -> Option<ExclusionReason> {
    match candidate.eligibility {
        StrategyEligibility::Eligible => {}
        StrategyEligibility::Infeasible => return Some(ExclusionReason::Infeasible),
        StrategyEligibility::PolicyDenied => return Some(ExclusionReason::PolicyDenied),
        StrategyEligibility::Indeterminate => return Some(ExclusionReason::Indeterminate),
    }

    if candidate.descriptor.kind().is_terminal()
        && !safety.allow_terminal_strategies()
    {
        return Some(ExclusionReason::TerminalStrategyDenied);
    }

    if candidate.descriptor.kind().is_adaptive()
        && !safety.allow_non_adaptive_strategies()
    {
        return Some(ExclusionReason::AdaptiveStrategyDenied);
    }

    if !safety.allow_unverified_evidence()
        && (!candidate.history.is_learning_eligible()
            || !candidate.prediction.confidence().value().is_finite())
    {
        return Some(ExclusionReason::UnverifiedEvidenceDenied);
    }

    None
}

/// Computes the learned utility score.
///
/// Important:
///
/// The calculation is deliberately simple and auditable. The learning model
/// itself lives in `learning/model.rs` / `learning/predictor.rs`. This file
/// consumes its outputs rather than implementing a model.
///
/// Prediction confidence gates the prediction contribution. Historical
/// evidence is only used when it is verified and sufficiently relevant.
fn compute_score(
    candidate: &StrategyCandidate,
    config: StrategySelectionConfig,
) -> Result<Utility, StrategySelectionError> {
    let prediction = candidate.prediction;
    let history = candidate.history;
    let weights = config.weights;

    let confidence = prediction.confidence().value();

    let prediction_component = if confidence
        >= config.minimum_prediction_confidence.value()
    {
        prediction.success_probability().value()
            * confidence
            * weights.success
    } else {
        0.0
    };

    let history_component = if history.is_learning_eligible()
        && history.relevance().value()
            >= config.minimum_history_relevance.value()
    {
        history.success_rate().value()
            * history.relevance().value()
            * weights.history
    } else {
        0.0
    };

    let confidence_component = confidence * weights.confidence;

    let utility_component =
        prediction.expected_utility().value() * weights.utility;

    let relevance_component =
        history.relevance().value() * weights.relevance;

    let score = prediction_component
        + history_component
        + confidence_component
        + utility_component
        + relevance_component;

    Utility::new(score).map_err(StrategySelectionError::from)
}

/// Deterministic ranking comparator.
///
/// Ordering:
///
/// 1. highest learned score;
/// 2. highest predicted success probability;
/// 3. highest prediction confidence;
/// 4. highest verified historical success rate;
/// 5. highest explicit registry/policy priority;
/// 6. stable strategy ID.
///
/// The final ID tie-breaker is critical for deterministic replay.
fn compare_ranked(left: &RankedStrategy, right: &RankedStrategy) -> Ordering {
    right
        .score
        .cmp(&left.score)
        .then_with(|| {
            right
                .prediction
                .success_probability()
                .cmp(&left.prediction.success_probability())
        })
        .then_with(|| {
            right
                .prediction
                .confidence()
                .cmp(&left.prediction.confidence())
        })
        .then_with(|| {
            right
                .history
                .success_rate()
                .cmp(&left.history.success_rate())
        })
        .then_with(|| left.descriptor.id().cmp(right.descriptor.id()))
}

/// A compact learning update derived from one verified outcome.
///
/// This type is intentionally not a mutable model.
///
/// `learning/feedback.rs` owns persistence/aggregation of feedback; this module
/// provides the normalization contract needed by strategy selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StrategyOutcome {
    /// The selected strategy produced an accepted result.
    Accepted,

    /// The strategy produced a degraded but explicitly accepted result.
    DegradedAccepted,

    /// The strategy failed verification.
    VerificationFailed,

    /// The strategy failed before verification.
    ExecutionFailed,

    /// The strategy was rejected by policy.
    PolicyRejected,

    /// The strategy was not executed.
    NotExecuted,
}

impl StrategyOutcome {
    /// Returns whether the outcome is positive learning evidence.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Accepted | Self::DegradedAccepted)
    }

    /// Returns whether the outcome is verified.
    ///
    /// Policy rejection and non-execution are not equivalent to execution
    /// failure and should not be blindly counted as failures.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(
            self,
            Self::Accepted
                | Self::DegradedAccepted
                | Self::VerificationFailed
                | Self::PolicyRejected
        )
    }
}

/// Feedback event for durable learning.
///
/// This is an immutable record. Persistence belongs to `learning/feedback.rs`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrategyFeedback {
    strategy_id: StrategyId,
    outcome: StrategyOutcome,
    trust: EvidenceTrust,
}

impl StrategyFeedback {
    /// Creates verified feedback.
    ///
    /// Callers should use this constructor only after the verification layer
    /// has established the outcome.
    pub const fn verified(
        strategy_id: StrategyId,
        outcome: StrategyOutcome,
    ) -> Self {
        Self {
            strategy_id,
            outcome,
            trust: EvidenceTrust::Verified,
        }
    }

    /// Creates explicitly unverified feedback.
    ///
    /// Unverified feedback is retained for diagnostics but must not normally
    /// influence learned strategy selection.
    pub const fn unverified(
        strategy_id: StrategyId,
        outcome: StrategyOutcome,
    ) -> Self {
        Self {
            strategy_id,
            outcome,
            trust: EvidenceTrust::Unverified,
        }
    }

    /// Returns strategy identity.
    #[must_use]
    pub fn strategy_id(&self) -> &StrategyId {
        &self.strategy_id
    }

    /// Returns outcome.
    #[must_use]
    pub const fn outcome(&self) -> StrategyOutcome {
        self.outcome
    }

    /// Returns evidence trust.
    #[must_use]
    pub const fn trust(&self) -> EvidenceTrust {
        self.trust
    }

    /// Returns whether this event is safe for normal learning.
    #[must_use]
    pub const fn is_learning_eligible(&self) -> bool {
        matches!(self.trust, EvidenceTrust::Verified)
    }
}

/// A deterministic aggregate of strategy feedback.
///
/// This is useful as an adapter between `learning/feedback.rs` and this
/// selector. It does not persist state and does not impose a finite history
/// capacity.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FeedbackAggregate {
    successes: u64,
    failures: u64,
    rejected: u64,
    not_executed: u64,
}

impl FeedbackAggregate {
    /// Incorporates one feedback record.
    ///
    /// Only verified outcomes are counted.
    pub fn observe(&mut self, feedback: &StrategyFeedback) {
        if !feedback.is_learning_eligible() {
            return;
        }

        match feedback.outcome {
            StrategyOutcome::Accepted | StrategyOutcome::DegradedAccepted => {
                self.successes = self.successes.saturating_add(1);
            }
            StrategyOutcome::VerificationFailed
            | StrategyOutcome::ExecutionFailed => {
                self.failures = self.failures.saturating_add(1);
            }
            StrategyOutcome::PolicyRejected => {
                self.rejected = self.rejected.saturating_add(1);
            }
            StrategyOutcome::NotExecuted => {
                self.not_executed = self.not_executed.saturating_add(1);
            }
        }
    }

    #[must_use]
    pub const fn successes(&self) -> u64 {
        self.successes
    }

    #[must_use]
    pub const fn failures(&self) -> u64 {
        self.failures
    }

    #[must_use]
    pub const fn rejected(&self) -> u64 {
        self.rejected
    }

    #[must_use]
    pub const fn not_executed(&self) -> u64 {
        self.not_executed
    }

    /// Converts the aggregate into historical evidence.
    ///
    /// Relevance is supplied by the caller because freshness/transferability
    /// are domain decisions owned by history/model layers.
    #[must_use]
    pub const fn historical_evidence(
        &self,
        relevance: Probability,
    ) -> HistoricalEvidence {
        HistoricalEvidence::new(
            self.successes,
            self.failures,
            relevance,
            EvidenceTrust::Verified,
        )
    }
}

/// A lightweight registry-facing recommendation.
///
/// The planner may convert this into its own `planning::action::Action`
/// representation without forcing the learning module to depend on planning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrategyRecommendation {
    strategy_id: StrategyId,
    strategy_kind: StrategyKind,
    rank: usize,
    score_bits: u64,
}

impl StrategyRecommendation {
    /// Creates a recommendation from a ranked strategy.
    #[must_use]
    pub fn from_ranked(strategy: &RankedStrategy) -> Self {
        Self {
            strategy_id: strategy.descriptor.id().clone(),
            strategy_kind: strategy.descriptor.kind(),
            rank: strategy.rank,
            score_bits: strategy.score.value().to_bits(),
        }
    }

    #[must_use]
    pub fn strategy_id(&self) -> &StrategyId {
        &self.strategy_id
    }

    #[must_use]
    pub const fn strategy_kind(&self) -> StrategyKind {
        self.strategy_kind
    }

    #[must_use]
    pub const fn rank(&self) -> usize {
        self.rank
    }

    /// Returns the exact IEEE-754 representation used by the score.
    ///
    /// This is useful for deterministic provenance without inventing a
    /// serialization dependency in this module.
    #[must_use]
    pub const fn score_bits(&self) -> u64 {
        self.score_bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strategy_id(value: &str) -> StrategyId {
        StrategyId::new(value).expect("test strategy ID must be valid")
    }

    fn descriptor(value: &str, kind: StrategyKind) -> StrategyDescriptor {
        StrategyDescriptor::new(
            strategy_id(value),
            kind,
            value,
        )
        .expect("test descriptor must be valid")
    }

    fn prediction(
        success: f64,
        confidence: f64,
        utility: f64,
    ) -> StrategyPrediction {
        StrategyPrediction::new(
            Probability::new(success).expect("valid probability"),
            Probability::new(confidence).expect("valid probability"),
            Utility::new(utility).expect("valid utility"),
        )
    }

    fn verified_history(
        successes: u64,
        failures: u64,
    ) -> HistoricalEvidence {
        HistoricalEvidence::new(
            successes,
            failures,
            Probability::ONE,
            EvidenceTrust::Verified,
        )
    }

    #[test]
    fn strategy_id_rejects_empty_values() {
        assert_eq!(
            StrategyId::new(""),
            Err(StrategyIdError::Empty)
        );

        assert_eq!(
            StrategyId::new("   "),
            Err(StrategyIdError::Empty)
        );
    }

    #[test]
    fn probability_rejects_non_finite_values() {
        assert_eq!(
            Probability::new(f64::NAN),
            Err(ProbabilityError::NonFinite)
        );

        assert_eq!(
            Probability::new(f64::INFINITY),
            Err(ProbabilityError::NonFinite)
        );
    }

    #[test]
    fn probability_rejects_out_of_range_values() {
        assert_eq!(
            Probability::new(-0.1),
            Err(ProbabilityError::OutOfRange)
        );

        assert_eq!(
            Probability::new(1.1),
            Err(ProbabilityError::OutOfRange)
        );
    }

    #[test]
    fn historical_success_rate_handles_empty_history() {
        let evidence = HistoricalEvidence::default();

        assert_eq!(
            evidence.success_rate(),
            Probability::ZERO
        );
    }

    #[test]
    fn historical_success_rate_is_deterministic() {
        let evidence = verified_history(3, 1);

        let rate = evidence.success_rate().value();

        assert!((rate - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn selector_ranks_highest_score_first() {
        let selector = StrategySelector::new();

        let candidates = vec![
            StrategyCandidate::new(
                descriptor("slow", StrategyKind::Restart),
                prediction(0.40, 0.80, 0.0),
                verified_history(1, 3),
                StrategyEligibility::Eligible,
            ),
            StrategyCandidate::new(
                descriptor("good", StrategyKind::Resume),
                prediction(0.90, 0.90, 0.0),
                verified_history(9, 1),
                StrategyEligibility::Eligible,
            ),
        ];

        let config = StrategySelectionConfig::new(
            StrategyWeights::new(
                1.0,
                0.0,
                1.0,
                0.0,
                0.0,
            )
            .expect("valid weights"),
            StrategySafety::strict(),
            Probability::ZERO,
            Probability::ZERO,
        );

        let ranking = selector
            .rank(
                candidates,
                config,
                SelectionBudget::unlimited(),
            )
            .expect("ranking must succeed");

        assert_eq!(
            ranking.best()
                .expect("a best strategy must exist")
                .descriptor()
                .id()
                .as_str(),
            "good"
        );

        assert_eq!(ranking.len(), 2);
        assert_eq!(ranking.ranked()[0].rank(), 1);
        assert_eq!(ranking.ranked()[1].rank(), 2);
    }

    #[test]
    fn selector_excludes_ineligible_candidates() {
        let selector = StrategySelector::new();

        let candidates = vec![
            StrategyCandidate::new(
                descriptor("unavailable", StrategyKind::Migration),
                prediction(1.0, 1.0, 0.0),
                verified_history(10, 0),
                StrategyEligibility::Infeasible,
            ),
            StrategyCandidate::new(
                descriptor("available", StrategyKind::Resume),
                prediction(0.5, 1.0, 0.0),
                verified_history(1, 1),
                StrategyEligibility::Eligible,
            ),
        ];

        let ranking = selector
            .rank(
                candidates,
                StrategySelectionConfig::default(),
                SelectionBudget::unlimited(),
            )
            .expect("ranking must succeed");

        assert_eq!(ranking.ranked().len(), 1);
        assert_eq!(ranking.excluded().len(), 1);

        assert_eq!(
            ranking.excluded()[0].reason(),
            ExclusionReason::Infeasible
        );
    }

    #[test]
    fn selector_rejects_duplicate_strategy_ids() {
        let selector = StrategySelector::new();

        let candidates = vec![
            StrategyCandidate::new(
                descriptor("duplicate", StrategyKind::Retry),
                prediction(0.5, 1.0, 0.0),
                verified_history(1, 1),
                StrategyEligibility::Eligible,
            ),
            StrategyCandidate::new(
                descriptor("duplicate", StrategyKind::Resume),
                prediction(0.5, 1.0, 0.0),
                verified_history(1, 1),
                StrategyEligibility::Eligible,
            ),
        ];

        let result = selector.rank(
            candidates,
            StrategySelectionConfig::default(),
            SelectionBudget::unlimited(),
        );

        assert!(matches!(
            result,
            Err(StrategySelectionError::DuplicateStrategyId(_))
        ));
    }

    #[test]
    fn selector_has_no_fixed_strategy_count() {
        let selector = StrategySelector::new();

        let mut candidates = Vec::new();

        for index in 0..64usize {
            let id = format!("strategy-{index}");

            candidates.push(StrategyCandidate::new(
                descriptor(&id, StrategyKind::Custom),
                prediction(
                    (index as f64 + 1.0) / 65.0,
                    1.0,
                    0.0,
                ),
                verified_history(index as u64, 0),
                StrategyEligibility::Eligible,
            ));
        }

        let ranking = selector
            .rank(
                candidates,
                StrategySelectionConfig::default(),
                SelectionBudget::unlimited(),
            )
            .expect("ranking must succeed");

        assert_eq!(ranking.ranked().len(), 64);
    }

    #[test]
    fn selection_budget_is_explicit() {
        let selector = StrategySelector::new();

        let candidates = vec![
            StrategyCandidate::new(
                descriptor("a", StrategyKind::Retry),
                prediction(0.5, 1.0, 0.0),
                verified_history(1, 0),
                StrategyEligibility::Eligible,
            ),
            StrategyCandidate::new(
                descriptor("b", StrategyKind::Retry),
                prediction(0.4, 1.0, 0.0),
                verified_history(1, 0),
                StrategyEligibility::Eligible,
            ),
        ];

        let result = selector.rank(
            candidates,
            StrategySelectionConfig::default(),
            SelectionBudget::candidates(1),
        );

        assert!(matches!(
            result,
            Err(StrategySelectionError::BudgetExceeded)
        ));
    }

    #[test]
    fn terminal_strategies_are_denied_by_strict_defaults() {
        let selector = StrategySelector::new();

        let candidate = StrategyCandidate::new(
            descriptor("abort", StrategyKind::Abort),
            prediction(1.0, 1.0, 1.0),
            verified_history(100, 0),
            StrategyEligibility::Eligible,
        );

        let ranking = selector
            .rank(
                vec![candidate],
                StrategySelectionConfig::default(),
                SelectionBudget::unlimited(),
            )
            .expect("ranking must succeed");

        assert!(ranking.ranked().is_empty());
        assert_eq!(
            ranking.excluded()[0].reason(),
            ExclusionReason::TerminalStrategyDenied
        );
    }

    #[test]
    fn verified_feedback_updates_aggregate() {
        let id = strategy_id("resume");

        let mut aggregate = FeedbackAggregate::default();

        aggregate.observe(&StrategyFeedback::verified(
            id.clone(),
            StrategyOutcome::Accepted,
        ));

        aggregate.observe(&StrategyFeedback::verified(
            id.clone(),
            StrategyOutcome::VerificationFailed,
        ));

        aggregate.observe(&StrategyFeedback::verified(
            id,
            StrategyOutcome::PolicyRejected,
        ));

        assert_eq!(aggregate.successes(), 1);
        assert_eq!(aggregate.failures(), 1);
        assert_eq!(aggregate.rejected(), 1);
        assert_eq!(aggregate.not_executed(), 0);
    }

    #[test]
    fn unverified_feedback_does_not_update_learning_aggregate() {
        let id = strategy_id("resume");

        let mut aggregate = FeedbackAggregate::default();

        aggregate.observe(&StrategyFeedback::unverified(
            id,
            StrategyOutcome::Accepted,
        ));

        assert_eq!(aggregate.successes(), 0);
        assert_eq!(aggregate.failures(), 0);
    }

    #[test]
    fn ranking_is_deterministic_for_equal_scores() {
        let selector = StrategySelector::new();

        let a = StrategyCandidate::new(
            descriptor("a", StrategyKind::Retry),
            prediction(0.5, 1.0, 0.0),
            verified_history(1, 1),
            StrategyEligibility::Eligible,
        );

        let b = StrategyCandidate::new(
            descriptor("b", StrategyKind::Retry),
            prediction(0.5, 1.0, 0.0),
            verified_history(1, 1),
            StrategyEligibility::Eligible,
        );

        let first = selector
            .rank(
                vec![b.clone(), a.clone()],
                StrategySelectionConfig::default(),
                SelectionBudget::unlimited(),
            )
            .expect("first ranking must succeed");

        let second = selector
            .rank(
                vec![a, b],
                StrategySelectionConfig::default(),
                SelectionBudget::unlimited(),
            )
            .expect("second ranking must succeed");

        assert_eq!(first, second);

        assert_eq!(
            first.ranked()[0].descriptor().id().as_str(),
            "a"
        );
    }

    #[test]
    fn recommendation_preserves_score_bits() {
        let candidate = StrategyCandidate::new(
            descriptor("resume", StrategyKind::Resume),
            prediction(0.75, 1.0, 0.25),
            verified_history(3, 1),
            StrategyEligibility::Eligible,
        );

        let selector = StrategySelector::new();

        let ranking = selector
            .rank(
                vec![candidate],
                StrategySelectionConfig::default(),
                SelectionBudget::unlimited(),
            )
            .expect("ranking must succeed");

        let recommendation =
            StrategyRecommendation::from_ranked(&ranking.ranked()[0]);

        assert_eq!(
            recommendation.strategy_id().as_str(),
            "resume"
        );

        assert_eq!(recommendation.rank(), 1);
    }
}