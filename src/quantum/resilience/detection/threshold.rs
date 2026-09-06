//! Zamani Quantum Resilience — Threshold Detection
//!
//! Production threshold detector for `quantum::resilience::detection`.
//!
//! # Responsibility
//!
//! This module evaluates caller-supplied threshold rules against a streaming
//! sequence of normalized scalar observations.
//!
//! It owns:
//!
//! - threshold rule representation;
//! - typed scalar threshold values;
//! - threshold predicates;
//! - optional hysteresis/reset predicates;
//! - threshold detector state;
//! - deterministic observation ordering;
//! - transition/violation emission policy;
//! - validation of threshold configuration;
//! - O(1) state per detector instance;
//! - integration with the canonical resilience error contract;
//! - integration with canonical resilience resource identities.
//!
//! It does NOT own:
//!
//! - telemetry collection;
//! - hardware discovery;
//! - calibration;
//! - anomaly modelling;
//! - statistical inference;
//! - QEC;
//! - noise modelling;
//! - diagnosis;
//! - recovery;
//! - mitigation;
//! - policy decisions;
//! - routing;
//! - scheduling;
//! - canonical quantum IR.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! telemetry / execution / QEC / hardware
//!                    |
//!                    v
//!             normalized observation
//!                    |
//!                    v
//!             ThresholdDetector
//!                    |
//!             +------+------+\
//!             |             |
//!             v             v
//!          trigger        clear
//!             |             |
//!             +------+------+\
//!                    |
//!                    v
//!          ThresholdEvaluation
//!                    |
//!                    v
//!        detection::detector.rs
//!                    |
//!                    v
//!          normalized detection signal
//!                    |
//!                    v
//!                 diagnosis
//! ```
//!
//! `threshold.rs` is therefore a detector mechanism, not the entire detection
//! subsystem.
//!
//! # Write once, scale everywhere
//!
//! This implementation deliberately contains no machine-size assumption.
//!
//! It does NOT contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_DEVICES
//! MAX_RULES
//! MAX_SAMPLES
//! retry_count = 3
//! fidelity < 0.95
//! error_rate > 0.01
//! qubit == 127
//! ```
//!
//! A concrete threshold is data supplied by the caller.
//!
//! A detector instance maintains only the state necessary to evaluate its
//! configured rule. It does not retain an unbounded observation history.
//! Therefore memory consumption is O(1) per detector instance regardless of
//! the number of observations processed.
//!
//! Multiple detector instances can be created for arbitrarily many metrics or
//! resources, subject only to resources actually available to the process.
//!
//! # Important distinction: mathematical threshold vs resilience policy
//!
//! This module answers only:
//!
//! > "Did this observation satisfy this configured threshold predicate?"
//!
//! It does NOT answer:
//!
//! > "Should the system recover?"
//!
//! The latter belongs to policy, diagnosis, planning and recovery.
//!
//! # Hysteresis
//!
//! A threshold can optionally have a separate reset predicate.
//!
//! Example:
//!
//! ```text
//! trigger: value >= 0.95
//! reset:   value <= 0.90
//! ```
//!
//! This prevents rapid trigger/clear oscillation around a boundary.
//!
//! The values above are merely an example. No such values are built into the
//! implementation.
//!
//! # Determinism
//!
//! Observations carry an explicit monotonically non-decreasing sequence number.
//! The detector rejects observations that move backwards in sequence.
//!
//! No wall-clock access, randomness, hash-map iteration, thread scheduling or
//! global mutable state is used by this module.
//!
//! Identical configuration + identical ordered observations produce identical
//! evaluations.
//!
//! # Canonical resource identity
//!
//! Resource references reuse:
//!
//! ```text
//! quantum::resilience::model::resource::ResourceIdentity
//! ```
//!
//! which itself uses the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file therefore never defines another qubit identifier.
//!
//! # Rust contract
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! The safety boundary is compiler-enforced with `#![forbid(unsafe_code)]`.
//!
//! # Integration contract
//!
//! `detection::detector.rs` should consume [`ThresholdEvaluation`] and map it
//! into the canonical detection signal/event type.
//!
//! `model::resource.rs` supplies resource identity.
//!
//! `errors::error.rs` supplies the common [`ResilienceError`] contract.
//!
//! `policy::*` supplies threshold values when thresholds are policy-driven.
//!
//! `telemetry::*` supplies observations.
//!
//! `diagnosis::*` interprets threshold events and combines them with other
//! evidence.
//!
//! Threshold detection must never directly invoke recovery.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use crate::quantum::resilience::errors::error::{
    ResilienceError,
    ResilienceErrorCode,
};
use crate::quantum::resilience::model::resource::ResourceIdentity;

// ============================================================================
// Public schema
// ============================================================================

/// Stable semantic schema identifier for threshold detection.
pub const THRESHOLD_DETECTOR_SCHEMA_ID: &str =
    "zamani.quantum.resilience.detection.threshold";

/// Semantic schema version.
///
/// Increment only when the externally observable contract changes.
pub const THRESHOLD_DETECTOR_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Threshold rule identity
// ============================================================================

/// Stable identity of a threshold rule.
///
/// Rule identifiers are caller-owned semantic identifiers. They must not
/// encode hardware-provider assumptions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ThresholdRuleId(String);

impl ThresholdRuleId {
    /// Creates a validated rule identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(Self::invalid(
                "threshold rule identifier must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    fn invalid(message: impl Into<String>) -> ResilienceError {
        ResilienceError::new(
            ResilienceErrorCode::InvalidArgument,
            message.into(),
        )
        .with_operation("threshold_rule_id")
    }
}

impl fmt::Display for ThresholdRuleId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Metric identity
// ============================================================================

/// Stable semantic identity of the metric being thresholded.
///
/// Examples:
///
/// ```text
/// gate_error_rate
/// readout_error_rate
/// logical_error_rate
/// fidelity
/// queue_latency
/// execution_latency
/// temperature
/// calibration_drift
/// syndrome_rate
/// leakage_rate
/// loss_rate
/// ```
///
/// The detector does not prescribe a vocabulary. Metric semantics belong to
/// the producer/policy layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetricId(String);

impl MetricId {
    /// Creates a validated metric identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(Self::invalid(
                "threshold metric identifier must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the metric identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    fn invalid(message: impl Into<String>) -> ResilienceError {
        ResilienceError::new(
            ResilienceErrorCode::InvalidArgument,
            message.into(),
        )
        .with_operation("threshold_metric_id")
    }
}

impl fmt::Display for MetricId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Threshold value
// ============================================================================

/// Numeric value supported by the threshold engine.
///
/// A tagged representation is used instead of converting everything to
/// `f64`. This prevents accidental comparisons such as:
///
/// ```text
/// resource count == floating-point fidelity
/// ```
///
/// and avoids precision loss for large counters.
///
/// `Real` values must be finite. NaN and infinities are rejected because their
/// ordering semantics are not suitable for deterministic threshold decisions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThresholdValue {
    /// Unsigned integral value.
    Unsigned(u128),

    /// Signed integral value.
    Signed(i128),

    /// Finite floating-point value.
    Real(f64),
}

impl ThresholdValue {
    /// Creates an unsigned value.
    #[must_use]
    pub const fn unsigned(value: u128) -> Self {
        Self::Unsigned(value)
    }

    /// Creates a signed value.
    #[must_use]
    pub const fn signed(value: i128) -> Self {
        Self::Signed(value)
    }

    /// Creates a finite real value.
    ///
    /// Returns an error for NaN or infinity.
    pub fn real(value: f64) -> Result<Self, ResilienceError> {
        if !value.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
                "threshold real values must be finite",
            )
            .with_operation("threshold_value"));
        }

        Ok(Self::Real(value))
    }

    /// Returns true when the value is a finite real.
    #[must_use]
    pub const fn is_real(self) -> bool {
        matches!(self, Self::Real(_))
    }

    /// Returns true when the value is unsigned.
    #[must_use]
    pub const fn is_unsigned(self) -> bool {
        matches!(self, Self::Unsigned(_))
    }

    /// Returns true when the value is signed.
    #[must_use]
    pub const fn is_signed(self) -> bool {
        matches!(self, Self::Signed(_))
    }

    /// Compares two threshold values.
    ///
    /// Different numeric domains are rejected rather than silently converted.
    pub fn compare(
        self,
        other: Self,
    ) -> Result<std::cmp::Ordering, ResilienceError> {
        match (self, other) {
            (Self::Unsigned(left), Self::Unsigned(right)) => {
                Ok(left.cmp(&right))
            }
            (Self::Signed(left), Self::Signed(right)) => Ok(left.cmp(&right)),
            (Self::Real(left), Self::Real(right)) => left
                .partial_cmp(&right)
                .ok_or_else(|| {
                    ResilienceError::new(
                        ResilienceErrorCode::DetectionInconsistent,
                        "threshold real comparison was unordered",
                    )
                    .with_operation("threshold_value_compare")
                }),
            _ => Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
                "threshold values use incompatible numeric domains",
            )
            .with_operation("threshold_value_compare")),
        }
    }
}

impl Eq for ThresholdValue {}

impl fmt::Display for ThresholdValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsigned(value) => write!(formatter, "{value}"),
            Self::Signed(value) => write!(formatter, "{value}"),
            Self::Real(value) => write!(formatter, "{value}"),
        }
    }
}

// ============================================================================
// Predicate
// ============================================================================

/// Threshold comparison predicate.
///
/// Predicates are deliberately explicit so callers cannot accidentally infer
/// whether equality is included at a boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThresholdPredicate {
    /// `value > bound`
    GreaterThan(ThresholdValue),

    /// `value >= bound`
    GreaterThanOrEqual(ThresholdValue),

    /// `value < bound`
    LessThan(ThresholdValue),

    /// `value <= bound`
    LessThanOrEqual(ThresholdValue),

    /// `value == bound`
    Equal(ThresholdValue),

    /// `value != bound`
    NotEqual(ThresholdValue),

    /// `value < lower || value > upper`
    Outside {
        /// Lower boundary.
        lower: ThresholdValue,

        /// Upper boundary.
        upper: ThresholdValue,

        /// Whether boundary equality is considered outside.
        inclusive: bool,
    },

    /// `lower < value < upper`, or inclusive equivalent.
    Inside {
        /// Lower boundary.
        lower: ThresholdValue,

        /// Upper boundary.
        upper: ThresholdValue,

        /// Whether boundary equality is considered inside.
        inclusive: bool,
    },
}

impl ThresholdPredicate {
    /// Validates predicate structure and numeric-domain compatibility.
    pub fn validate(&self) -> Result<(), ResilienceError> {
        match *self {
            Self::GreaterThan(_)
            | Self::GreaterThanOrEqual(_)
            | Self::LessThan(_)
            | Self::LessThanOrEqual(_)
            | Self::Equal(_)
            | Self::NotEqual(_) => Ok(()),

            Self::Outside {
                lower,
                upper,
                ..
            }
            | Self::Inside {
                lower,
                upper,
                ..
            } => {
                let ordering = lower.compare(upper)?;

                if ordering != std::cmp::Ordering::Less {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::InvalidArgument,
                        "threshold range requires lower < upper",
                    )
                    .with_operation("threshold_predicate"));
                }

                Ok(())
            }
        }
    }

    /// Evaluates this predicate against a value.
    pub fn matches(
        &self,
        value: ThresholdValue,
    ) -> Result<bool, ResilienceError> {
        self.validate()?;

        match *self {
            Self::GreaterThan(bound) => {
                Ok(value.compare(bound)? == std::cmp::Ordering::Greater)
            }

            Self::GreaterThanOrEqual(bound) => {
                Ok(matches!(
                    value.compare(bound)?,
                    std::cmp::Ordering::Greater | std::cmp::Ordering::Equal
                ))
            }

            Self::LessThan(bound) => {
                Ok(value.compare(bound)? == std::cmp::Ordering::Less)
            }

            Self::LessThanOrEqual(bound) => {
                Ok(matches!(
                    value.compare(bound)?,
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal
                ))
            }

            Self::Equal(bound) => {
                Ok(value.compare(bound)? == std::cmp::Ordering::Equal)
            }

            Self::NotEqual(bound) => {
                Ok(value.compare(bound)? != std::cmp::Ordering::Equal)
            }

            Self::Outside {
                lower,
                upper,
                inclusive,
            } => {
                let lower_cmp = value.compare(lower)?;
                let upper_cmp = value.compare(upper)?;

                if inclusive {
                    Ok(lower_cmp == std::cmp::Ordering::Less
                        || upper_cmp == std::cmp::Ordering::Greater)
                } else {
                    Ok(lower_cmp != std::cmp::Ordering::Less
                        && upper_cmp != std::cmp::Ordering::Greater)
                }
            }

            Self::Inside {
                lower,
                upper,
                inclusive,
            } => {
                let lower_cmp = value.compare(lower)?;
                let upper_cmp = value.compare(upper)?;

                if inclusive {
                    Ok(matches!(
                        lower_cmp,
                        std::cmp::Ordering::Greater
                            | std::cmp::Ordering::Equal
                    ) && matches!(
                        upper_cmp,
                        std::cmp::Ordering::Less
                            | std::cmp::Ordering::Equal
                    ))
                } else {
                    Ok(lower_cmp == std::cmp::Ordering::Greater
                        && upper_cmp == std::cmp::Ordering::Less)
                }
            }
        }
    }
}

// ============================================================================
// Emission policy
// ============================================================================

/// Determines when a threshold detector emits an evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ThresholdEmission {
    /// Emit only when the detector enters or leaves the triggered state.
    TransitionsOnly,

    /// Emit every observation while the threshold is triggered.
    EveryViolation,

    /// Emit every valid observation.
    EveryObservation,
}

// ============================================================================
// Detector state
// ============================================================================

/// Stateful threshold status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ThresholdState {
    /// No threshold violation is currently active.
    Clear,

    /// The configured trigger predicate is active.
    Triggered,
}

impl ThresholdState {
    /// Returns true when the threshold is triggered.
    #[must_use]
    pub const fn is_triggered(self) -> bool {
        matches!(self, Self::Triggered)
    }

    /// Returns true when the threshold is clear.
    #[must_use]
    pub const fn is_clear(self) -> bool {
        matches!(self, Self::Clear)
    }
}

// ============================================================================
// Evaluation kind
// ============================================================================

/// Semantic event produced by a threshold evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ThresholdEventKind {
    /// Threshold became active.
    Triggered,

    /// Threshold remained active.
    Violation,

    /// Threshold became inactive.
    Cleared,

    /// Observation was valid but produced no state transition.
    Normal,
}

// ============================================================================
// Observation
// ============================================================================

/// One normalized scalar observation.
///
/// The observation sequence is supplied by the caller. It is deliberately
/// independent of wall-clock time so replay and deterministic testing do not
/// depend on system clocks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThresholdObservation {
    /// Metric being observed.
    pub metric: MetricId,

    /// Observed scalar value.
    pub value: ThresholdValue,

    /// Monotonically non-decreasing caller-defined observation sequence.
    pub sequence: u64,

    /// Optional resource associated with the observation.
    ///
    /// This reuses canonical logical/physical resource identities.
    pub resource: Option<ResourceIdentity>,
}

impl ThresholdObservation {
    /// Creates an observation without an explicit resource.
    #[must_use]
    pub const fn new(
        metric: MetricId,
        value: ThresholdValue,
        sequence: u64,
    ) -> Self {
        Self {
            metric,
            value,
            sequence,
            resource: None,
        }
    }

    /// Associates the observation with a canonical resource.
    #[must_use]
    pub const fn with_resource(
        mut self,
        resource: ResourceIdentity,
    ) -> Self {
        self.resource = Some(resource);
        self
    }
}

// ============================================================================
// Rule
// ============================================================================

/// Complete immutable threshold configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdRule {
    /// Stable rule identity.
    pub id: ThresholdRuleId,

    /// Metric consumed by this rule.
    pub metric: MetricId,

    /// Predicate that enters the triggered state.
    pub trigger: ThresholdPredicate,

    /// Optional predicate that clears a triggered state.
///
/// When absent, the trigger predicate is evaluated in the conventional
/// single-threshold manner: a triggered state clears when the trigger is no
/// longer satisfied.
///
/// A separate reset predicate enables hysteresis.
    pub reset: Option<ThresholdPredicate>,

    /// Controls event emission.
    pub emission: ThresholdEmission,
}

impl ThresholdRule {
    /// Creates a threshold rule.
    pub fn new(
        id: ThresholdRuleId,
        metric: MetricId,
        trigger: ThresholdPredicate,
    ) -> Result<Self, ResilienceError> {
        trigger.validate()?;

        Ok(Self {
            id,
            metric,
            trigger,
            reset: None,
            emission: ThresholdEmission::TransitionsOnly,
        })
    }

    /// Sets a separate reset predicate, enabling hysteresis.
    pub fn with_reset(
        mut self,
        reset: ThresholdPredicate,
    ) -> Result<Self, ResilienceError> {
        reset.validate()?;

        self.reset = Some(reset);
        Ok(self)
    }

    /// Sets the emission policy.
    #[must_use]
    pub const fn with_emission(
        mut self,
        emission: ThresholdEmission,
    ) -> Self {
        self.emission = emission;
        self
    }

    /// Validates the complete rule.
    pub fn validate(&self) -> Result<(), ResilienceError> {
        if self.id.as_str().trim().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
                "threshold rule identifier must not be empty",
            )
            .with_operation("threshold_rule"));
        }

        if self.metric.as_str().trim().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
                "threshold metric identifier must not be empty",
            )
            .with_operation("threshold_rule"));
        }

        self.trigger.validate()?;

        if let Some(reset) = self.reset {
            reset.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// Evaluation
// ============================================================================

/// Result of evaluating one observation.
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdEvaluation {
    /// Rule that produced this evaluation.
    pub rule_id: ThresholdRuleId,

    /// Metric being evaluated.
    pub metric: MetricId,

    /// Observation sequence.
    pub sequence: u64,

    /// Observed value.
    pub value: ThresholdValue,

    /// Optional associated canonical resource.
    pub resource: Option<ResourceIdentity>,

    /// State before evaluation.
    pub previous_state: ThresholdState,

    /// State after evaluation.
    pub state: ThresholdState,

    /// Semantic event generated by the evaluation.
    pub event: ThresholdEventKind,
}

impl ThresholdEvaluation {
    /// Returns true when the evaluation represents an active violation.
    #[must_use]
    pub const fn is_violation(&self) -> bool {
        matches!(
            self.event,
            ThresholdEventKind::Triggered | ThresholdEventKind::Violation
        )
    }

    /// Returns true when the threshold transitioned into the triggered state.
    #[must_use]
    pub const fn is_triggered(&self) -> bool {
        matches!(self.event, ThresholdEventKind::Triggered)
    }

    /// Returns true when the threshold transitioned back to clear.
    #[must_use]
    pub const fn is_cleared(&self) -> bool {
        matches!(self.event, ThresholdEventKind::Cleared)
    }
}

// ============================================================================
// Detector
// ============================================================================

/// Stateful production threshold detector.
///
/// Each detector owns exactly one immutable threshold rule and a small amount
/// of mutable state.
///
/// Memory complexity:
///
/// ```text
/// O(1)
/// ```
///
/// per detector instance, independent of the number of observations processed.
///
/// Time complexity:
///
/// ```text
/// O(1)
/// ```
///
/// per observation, excluding ordinary string/resource ownership costs.
///
/// This design intentionally avoids retaining historical samples. Historical
/// analysis belongs to `statistical.rs`, `history/`, or another appropriate
/// subsystem.
#[derive(Debug, Clone)]
pub struct ThresholdDetector {
    rule: ThresholdRule,
    state: ThresholdState,
    last_sequence: Option<u64>,
}

impl ThresholdDetector {
    /// Creates a detector from a validated rule.
    pub fn new(rule: ThresholdRule) -> Result<Self, ResilienceError> {
        rule.validate()?;

        Ok(Self {
            rule,
            state: ThresholdState::Clear,
            last_sequence: None,
        })
    }

    /// Returns the immutable rule.
    #[must_use]
    pub const fn rule(&self) -> &ThresholdRule {
        &self.rule
    }

    /// Returns the current detector state.
    #[must_use]
    pub const fn state(&self) -> ThresholdState {
        self.state
    }

    /// Returns the last accepted observation sequence.
    #[must_use]
    pub const fn last_sequence(&self) -> Option<u64> {
        self.last_sequence
    }

    /// Evaluates one observation.
    ///
    /// Returns `None` when the observation is valid but the configured
    /// emission policy says that no event should be emitted.
    pub fn evaluate(
        &mut self,
        observation: ThresholdObservation,
    ) -> Result<Option<ThresholdEvaluation>, ResilienceError> {
        if observation.metric != self.rule.metric {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidDetectionInput,
                "threshold observation metric does not match detector rule",
            )
            .with_operation("threshold_detector"));
        }

        if let Some(last_sequence) = self.last_sequence {
            if observation.sequence < last_sequence {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::DetectionInconsistent,
                    "threshold observation sequence moved backwards",
                )
                .with_operation("threshold_detector"));
            }
        }

        let previous_state = self.state;

        let next_state = match self.state {
            ThresholdState::Clear => {
                if self.rule.trigger.matches(observation.value)? {
                    ThresholdState::Triggered
                } else {
                    ThresholdState::Clear
                }
            }

            ThresholdState::Triggered => {
                let should_clear = match self.rule.reset {
                    Some(reset) => reset.matches(observation.value)?,
                    None => !self.rule.trigger.matches(observation.value)?,
                };

                if should_clear {
                    ThresholdState::Clear
                } else {
                    ThresholdState::Triggered
                }
            }
        };

        let event = match (previous_state, next_state) {
            (ThresholdState::Clear, ThresholdState::Triggered) => {
                ThresholdEventKind::Triggered
            }

            (ThresholdState::Triggered, ThresholdState::Clear) => {
                ThresholdEventKind::Cleared
            }

            (ThresholdState::Triggered, ThresholdState::Triggered) => {
                ThresholdEventKind::Violation
            }

            (ThresholdState::Clear, ThresholdState::Clear) => {
                ThresholdEventKind::Normal
            }
        };

        self.state = next_state;
        self.last_sequence = Some(observation.sequence);

        let should_emit = match self.rule.emission {
            ThresholdEmission::TransitionsOnly => {
                matches!(
                    event,
                    ThresholdEventKind::Triggered | ThresholdEventKind::Cleared
                )
            }

            ThresholdEmission::EveryViolation => {
                matches!(
                    event,
                    ThresholdEventKind::Triggered
                        | ThresholdEventKind::Violation
                        | ThresholdEventKind::Cleared
                )
            }

            ThresholdEmission::EveryObservation => true,
        };

        if !should_emit {
            return Ok(None);
        }

        Ok(Some(ThresholdEvaluation {
            rule_id: self.rule.id.clone(),
            metric: self.rule.metric.clone(),
            sequence: observation.sequence,
            value: observation.value,
            resource: observation.resource,
            previous_state,
            state: next_state,
            event,
        }))
    }

    /// Evaluates an iterator of observations.
    ///
    /// This method does not allocate a result collection. The caller receives
    /// each emitted evaluation through the supplied callback.
    ///
    /// This is important for large telemetry streams: memory consumption does
    /// not grow with the number of observations.
    pub fn evaluate_stream<I, F>(
        &mut self,
        observations: I,
        mut emit: F,
    ) -> Result<(), ResilienceError>
    where
        I: IntoIterator<Item = ThresholdObservation>,
        F: FnMut(ThresholdEvaluation),
    {
        for observation in observations {
            if let Some(evaluation) = self.evaluate(observation)? {
                emit(evaluation);
            }
        }

        Ok(())
    }

    /// Resets detector state without changing its configuration.
    ///
    /// This is an explicit lifecycle operation. It must not be confused with
    /// a threshold clear event caused by an observation.
    pub fn reset(&mut self) {
        self.state = ThresholdState::Clear;
        self.last_sequence = None;
    }

    /// Returns a copy of the current state suitable for deterministic replay.
    #[must_use]
    pub const fn snapshot(&self) -> ThresholdDetectorState {
        ThresholdDetectorState {
            state: self.state,
            last_sequence: self.last_sequence,
        }
    }

    /// Restores detector state after validating that the supplied sequence is
    /// internally consistent.
    pub fn restore(
        &mut self,
        snapshot: ThresholdDetectorState,
    ) -> Result<(), ResilienceError> {
        if snapshot.state == ThresholdState::Clear
            && snapshot.last_sequence.is_none()
        {
            self.state = ThresholdState::Clear;
            self.last_sequence = None;
            return Ok(());
        }

        if snapshot.last_sequence.is_none() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidState,
                "triggered threshold state requires an observation sequence",
            )
            .with_operation("threshold_detector_restore"));
        }

        self.state = snapshot.state;
        self.last_sequence = snapshot.last_sequence;

        Ok(())
    }
}

// ============================================================================
// Persistable state
// ============================================================================

/// Minimal deterministic state required to resume a threshold detector.
///
/// The immutable [`ThresholdRule`] remains separate so a restored detector
/// cannot silently change its rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThresholdDetectorState {
    /// Current threshold state.
    pub state: ThresholdState,

    /// Last accepted observation sequence.
    pub last_sequence: Option<u64>,
}

// ============================================================================
// Constructors for common predicates
// ============================================================================

/// Creates a `value > threshold` predicate.
#[must_use]
pub const fn above(threshold: ThresholdValue) -> ThresholdPredicate {
    ThresholdPredicate::GreaterThan(threshold)
}

/// Creates a `value >= threshold` predicate.
#[must_use]
pub const fn at_or_above(threshold: ThresholdValue) -> ThresholdPredicate {
    ThresholdPredicate::GreaterThanOrEqual(threshold)
}

/// Creates a `value < threshold` predicate.
#[must_use]
pub const fn below(threshold: ThresholdValue) -> ThresholdPredicate {
    ThresholdPredicate::LessThan(threshold)
}

/// Creates a `value <= threshold` predicate.
#[must_use]
pub const fn at_or_below(threshold: ThresholdValue) -> ThresholdPredicate {
    ThresholdPredicate::LessThanOrEqual(threshold)
}

/// Creates a `value == threshold` predicate.
#[must_use]
pub const fn equal(threshold: ThresholdValue) -> ThresholdPredicate {
    ThresholdPredicate::Equal(threshold)
}

/// Creates a `value != threshold` predicate.
#[must_use]
pub const fn not_equal(threshold: ThresholdValue) -> ThresholdPredicate {
    ThresholdPredicate::NotEqual(threshold)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn metric() -> MetricId {
        MetricId::new("test.metric").expect("valid metric")
    }

    fn rule_id() -> ThresholdRuleId {
        ThresholdRuleId::new("test.rule").expect("valid rule")
    }

    fn real(value: f64) -> ThresholdValue {
        ThresholdValue::real(value).expect("finite real")
    }

    #[test]
    fn greater_than_is_strict() {
        let predicate = above(real(10.0));

        assert!(!predicate.matches(real(10.0)).expect("comparison"));
        assert!(predicate.matches(real(10.1)).expect("comparison"));
    }

    #[test]
    fn greater_than_or_equal_includes_boundary() {
        let predicate = at_or_above(real(10.0));

        assert!(predicate.matches(real(10.0)).expect("comparison"));
        assert!(predicate.matches(real(10.1)).expect("comparison"));
        assert!(!predicate.matches(real(9.9)).expect("comparison"));
    }

    #[test]
    fn less_than_is_strict() {
        let predicate = below(real(10.0));

        assert!(predicate.matches(real(9.9)).expect("comparison"));
        assert!(!predicate.matches(real(10.0)).expect("comparison"));
    }

    #[test]
    fn less_than_or_equal_includes_boundary() {
        let predicate = at_or_below(real(10.0));

        assert!(predicate.matches(real(10.0)).expect("comparison"));
        assert!(predicate.matches(real(9.9)).expect("comparison"));
        assert!(!predicate.matches(real(10.1)).expect("comparison"));
    }

    #[test]
    fn hysteresis_prevents_boundary_flapping() {
        let rule = ThresholdRule::new(
            rule_id(),
            metric(),
            at_or_above(real(0.95)),
        )
        .expect("valid rule")
        .with_reset(at_or_below(real(0.90)))
        .expect("valid reset");

        let mut detector =
            ThresholdDetector::new(rule).expect("valid detector");

        let first = detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(0.96),
                1,
            ))
            .expect("evaluation")
            .expect("transition");

        assert_eq!(first.event, ThresholdEventKind::Triggered);

        let second = detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(0.93),
                2,
            ))
            .expect("evaluation");

        assert!(second.is_none());
        assert_eq!(detector.state(), ThresholdState::Triggered);

        let third = detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(0.90),
                3,
            ))
            .expect("evaluation")
            .expect("clear event");

        assert_eq!(third.event, ThresholdEventKind::Cleared);
        assert_eq!(detector.state(), ThresholdState::Clear);
    }

    #[test]
    fn transition_only_is_default() {
        let rule = ThresholdRule::new(
            rule_id(),
            metric(),
            at_or_above(real(1.0)),
        )
        .expect("valid rule");

        let mut detector =
            ThresholdDetector::new(rule).expect("valid detector");

        let first = detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(2.0),
                1,
            ))
            .expect("evaluation");

        assert!(matches!(
            first.map(|value| value.event),
            Some(ThresholdEventKind::Triggered)
        ));

        let second = detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(2.0),
                2,
            ))
            .expect("evaluation");

        assert!(second.is_none());
    }

    #[test]
    fn every_violation_emits_sustained_violations() {
        let rule = ThresholdRule::new(
            rule_id(),
            metric(),
            at_or_above(real(1.0)),
        )
        .expect("valid rule")
        .with_emission(ThresholdEmission::EveryViolation);

        let mut detector =
            ThresholdDetector::new(rule).expect("valid detector");

        let first = detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(2.0),
                1,
            ))
            .expect("evaluation")
            .expect("first event");

        assert_eq!(first.event, ThresholdEventKind::Triggered);

        let second = detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(2.0),
                2,
            ))
            .expect("evaluation")
            .expect("violation event");

        assert_eq!(second.event, ThresholdEventKind::Violation);
    }

    #[test]
    fn every_observation_emits_normal_samples() {
        let rule = ThresholdRule::new(
            rule_id(),
            metric(),
            at_or_above(real(1.0)),
        )
        .expect("valid rule")
        .with_emission(ThresholdEmission::EveryObservation);

        let mut detector =
            ThresholdDetector::new(rule).expect("valid detector");

        let evaluation = detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(0.5),
                1,
            ))
            .expect("evaluation")
            .expect("normal event");

        assert_eq!(evaluation.event, ThresholdEventKind::Normal);
    }

    #[test]
    fn backward_sequences_are_rejected() {
        let rule = ThresholdRule::new(
            rule_id(),
            metric(),
            at_or_above(real(1.0)),
        )
        .expect("valid rule");

        let mut detector =
            ThresholdDetector::new(rule).expect("valid detector");

        detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(0.5),
                10,
            ))
            .expect("first evaluation");

        let error = detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(0.5),
                9,
            ))
            .expect_err("backward sequence must fail");

        assert_eq!(
            error.code(),
            ResilienceErrorCode::DetectionInconsistent
        );
    }

    #[test]
    fn wrong_metric_is_rejected() {
        let rule = ThresholdRule::new(
            rule_id(),
            metric(),
            at_or_above(real(1.0)),
        )
        .expect("valid rule");

        let mut detector =
            ThresholdDetector::new(rule).expect("valid detector");

        let other_metric =
            MetricId::new("other.metric").expect("valid metric");

        let error = detector
            .evaluate(ThresholdObservation::new(
                other_metric,
                real(2.0),
                1,
            ))
            .expect_err("metric mismatch must fail");

        assert_eq!(
            error.code(),
            ResilienceErrorCode::InvalidDetectionInput
        );
    }

    #[test]
    fn incompatible_numeric_domains_are_rejected() {
        let predicate = above(ThresholdValue::unsigned(10));

        let error = predicate
            .matches(ThresholdValue::signed(11))
            .expect_err("domains must not be silently converted");

        assert_eq!(
            error.code(),
            ResilienceErrorCode::InvalidArgument
        );
    }

    #[test]
    fn nan_is_rejected() {
        let error =
            ThresholdValue::real(f64::NAN).expect_err("NaN is invalid");

        assert_eq!(
            error.code(),
            ResilienceErrorCode::InvalidArgument
        );
    }

    #[test]
    fn positive_infinity_is_rejected() {
        let error =
            ThresholdValue::real(f64::INFINITY)
                .expect_err("infinity is invalid");

        assert_eq!(
            error.code(),
            ResilienceErrorCode::InvalidArgument
        );
    }

    #[test]
    fn negative_infinity_is_rejected() {
        let error =
            ThresholdValue::real(f64::NEG_INFINITY)
                .expect_err("infinity is invalid");

        assert_eq!(
            error.code(),
            ResilienceErrorCode::InvalidArgument
        );
    }

    #[test]
    fn range_requires_lower_less_than_upper() {
        let predicate = ThresholdPredicate::Inside {
            lower: real(2.0),
            upper: real(1.0),
            inclusive: false,
        };

        let error = predicate
            .validate()
            .expect_err("invalid range must fail");

        assert_eq!(
            error.code(),
            ResilienceErrorCode::InvalidArgument
        );
    }

    #[test]
    fn stream_processing_does_not_require_result_storage() {
        let rule = ThresholdRule::new(
            rule_id(),
            metric(),
            at_or_above(real(1.0)),
        )
        .expect("valid rule")
        .with_emission(ThresholdEmission::EveryObservation);

        let mut detector =
            ThresholdDetector::new(rule).expect("valid detector");

        let observations = (0_u64..10_000_u64).map(|sequence| {
            ThresholdObservation::new(
                metric(),
                real(if sequence % 2 == 0 { 2.0 } else { 0.0 }),
                sequence,
            )
        });

        let mut count = 0_u64;

        detector
            .evaluate_stream(observations, |_evaluation| {
                count += 1;
            })
            .expect("stream must succeed");

        assert_eq!(count, 10_000);
    }

    #[test]
    fn snapshot_and_restore_are_deterministic() {
        let rule = ThresholdRule::new(
            rule_id(),
            metric(),
            at_or_above(real(1.0)),
        )
        .expect("valid rule");

        let mut first =
            ThresholdDetector::new(rule.clone()).expect("detector");

        first
            .evaluate(ThresholdObservation::new(
                metric(),
                real(2.0),
                42,
            ))
            .expect("evaluation");

        let snapshot = first.snapshot();

        let mut second =
            ThresholdDetector::new(rule).expect("detector");

        second.restore(snapshot).expect("restore");

        assert_eq!(first.state(), second.state());
        assert_eq!(first.last_sequence(), second.last_sequence());
    }

    #[test]
    fn resource_identity_is_carried_without_redefinition() {
        let resource = ResourceIdentity::logical_qubit(
            crate::quantum::ir::qubit::QubitId::new(0),
        );

        let rule = ThresholdRule::new(
            rule_id(),
            metric(),
            at_or_above(real(1.0)),
        )
        .expect("valid rule");

        let mut detector =
            ThresholdDetector::new(rule).expect("detector");

        let evaluation = detector
            .evaluate(
                ThresholdObservation::new(metric(), real(2.0), 1)
                    .with_resource(resource),
            )
            .expect("evaluation")
            .expect("trigger");

        assert_eq!(evaluation.resource, Some(resource));
    }

    #[test]
    fn reset_returns_detector_to_initial_state() {
        let rule = ThresholdRule::new(
            rule_id(),
            metric(),
            at_or_above(real(1.0)),
        )
        .expect("valid rule");

        let mut detector =
            ThresholdDetector::new(rule).expect("detector");

        detector
            .evaluate(ThresholdObservation::new(
                metric(),
                real(2.0),
                1,
            ))
            .expect("evaluation");

        assert_eq!(detector.state(), ThresholdState::Triggered);

        detector.reset();

        assert_eq!(detector.state(), ThresholdState::Clear);
        assert_eq!(detector.last_sequence(), None);
    }
}