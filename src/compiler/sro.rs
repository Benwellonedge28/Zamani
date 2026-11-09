#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Compiler — Self-Reflective Optimizer (SRO).
//!
//! Path: src/compiler/sro.rs
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! # Purpose
//!
//! SRO consumes validated compiler profiling measurements and produces
//! evidence-backed optimization hints.
//!
//! It does not implement optimization transformations, execute passes,
//! modify IR, select physical hardware, or schedule runtime execution.
//!
//! # Canonical authorities
//!
//! Profiling measurements:
//!     crate::compiler::profiling::measurements
//!
//! Profiling aggregation:
//!     crate::compiler::profiling::aggregation
//!
//! Optimization planning:
//!     crate::compiler::optimization::planning
//!
//! Optimization policies:
//!     crate::compiler::optimization::policies
//!
//! Profitability:
//!     crate::compiler::optimization::profitability
//!
//! Optimization decisions:
//!     crate::compiler::optimization::decisions
//!
//! Optimization validation:
//!     crate::compiler::optimization::validation
//!
//! Instruction fusion:
//!     crate::compiler::instruction_fusion
//!
//! Pass management:
//!     crate::compiler::passes
//!
//! Canonical pass execution:
//!     crate::zuir::pass
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Telemetry-informed optimization assessment.
//! - Configurable metric assessment rules.
//! - Optimization hint construction.
//! - Evidence and provenance association.
//! - Legacy telemetry compatibility.
//!
//! This module does not own:
//!
//! - Source-level semantic analysis.
//! - Optimization policy semantics.
//! - Cost model implementation.
//! - Transformation legality.
//! - Profitability evaluation.
//! - Pass registration.
//! - Pass execution.
//! - IR verification.
//! - Backend implementation.
//! - Physical hardware selection.
//! - Runtime scheduling.
//!
//! # Fundamental safety rule
//!
//! Profiling evidence is advisory.
//!
//! Profiling evidence MUST NOT override:
//!
//! - semantic correctness;
//! - transformation preconditions;
//! - ownership requirements;
//! - effect requirements;
//! - numerical guarantees;
//! - domain-specific invariants;
//! - resource constraints;
//! - reproducibility policies;
//! - canonical IR verification.
//!
//! # Scalability
//!
//! No arbitrary universal limits are imposed on:
//!
//! - metric identities;
//! - measurement identities;
//! - optimization rules;
//! - computational domains;
//! - targets;
//! - backend providers;
//! - devices;
//! - workers;
//! - optimization candidates.
//!
//! Optional history retention is an operational policy.
//!
//! Actual physical and implementation resources remain applicable.
//!
//! # POCO-REAF
//!
//! SRO is target-independent at its public architectural boundary.
//!
//! It does not assume that a performance observation from one target
//! applies to another target.
//!
//! Target-dependent measurements retain their classification.
//!
//! An optimization hint never changes program semantics.
//!
//! # Determinism
//!
//! Assessment is deterministic for identical:
//!
//! - validated measurements;
//! - measurement provenance;
//! - configuration;
//! - applicable rules.
//!
//! Wall-clock measurements are not converted into deterministic
//! compilation identity or cache fingerprints.

use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::num::NonZeroUsize;

use crate::compiler::profiling::measurements::{
    Measurement,
    MeasurementStability,
    MeasurementValue,
};

use crate::distributed::observability::metrics::{
    MetricName,
    MetricUnit,
};

/// Public SRO API version.
///
/// This version is independent of the language, semantic model,
/// profiling API, optimization API and backend ABI versions.
pub const SRO_API_VERSION: u16 = 2;

/// Legacy latency threshold.
///
/// This exists exclusively for compatibility with the historical
/// PerformanceTelemetry interface.
///
/// New implementations should use MetricRule.
#[deprecated(
    note = "Use configurable MetricRule values instead."
)]
pub const DEFAULT_LATENCY_THRESHOLD_US: u64 = 50;

/// Legacy instruction-count threshold.
#[deprecated(
    note = "Use configurable MetricRule values instead."
)]
pub const DEFAULT_INSTRUCTION_THRESHOLD: usize = 10;

/// Legacy cache-miss threshold.
#[deprecated(
    note = "Use configurable MetricRule values instead."
)]
pub const DEFAULT_CACHE_MISS_THRESHOLD: f64 = 0.10;

/// Result of an SRO operation.
pub type SroResult<T> = Result<T, SroError>;

// ============================================================================
// METRIC COMPARISON
// ============================================================================

/// Comparison applied to a measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricComparison {
    /// Observed value is greater than the configured threshold.
    GreaterThan,

    /// Observed value is greater than or equal to the threshold.
    GreaterThanOrEqual,

    /// Observed value is less than the configured threshold.
    LessThan,

    /// Observed value is less than or equal to the threshold.
    LessThanOrEqual,

    /// Observed value equals the configured threshold.
    Equal,
}

impl MetricComparison {
    fn matches(
        self,
        observed: MeasurementValue,
        threshold: MeasurementValue,
    ) -> SroResult<bool> {
        use MetricComparison::*;

        let ordering = compare_values(observed, threshold)?;

        Ok(match self {
            GreaterThan => ordering.is_gt(),
            GreaterThanOrEqual => !ordering.is_lt(),
            LessThan => ordering.is_lt(),
            LessThanOrEqual => !ordering.is_gt(),
            Equal => ordering.is_eq(),
        })
    }
}

/// Compares two measurements without converting integer values
/// into floating-point representations.
///
/// Different numeric representations are rejected.
fn compare_values(
    left: MeasurementValue,
    right: MeasurementValue,
) -> SroResult<std::cmp::Ordering> {
    use MeasurementValue::*;

    match (left, right) {
        (Unsigned(a), Unsigned(b)) => Ok(a.cmp(&b)),

        (Signed(a), Signed(b)) => Ok(a.cmp(&b)),

        (Float(a), Float(b)) => {
            a.partial_cmp(&b)
                .ok_or(SroError::InvalidMeasurementValue)
        }

        _ => Err(SroError::NumericRepresentationMismatch),
    }
}

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Stable identity of one SRO rule.
///
/// Rule identities are extensible and are not restricted to a
/// predefined list of computing domains.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SroRuleId(String);

impl SroRuleId {
    /// Creates a validated rule identity.
    pub fn new(value: impl Into<String>) -> SroResult<Self> {
        let value = value.into();

        validate_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SroRuleId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Validates an extensible identifier.
///
/// No arbitrary maximum length is imposed.
fn validate_identifier(value: &str) -> SroResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(SroError::InvalidIdentifier(
            value.to_owned(),
        ));
    }

    Ok(())
}

// ============================================================================
// METRIC RULES
// ============================================================================

/// Rule for evaluating one canonical profiling metric.
///
/// Metric names and units must match the measurement exactly.
///
/// Stability classifications must be explicitly accepted.
/// An empty stability list means that the rule accepts no measurements.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricRule {
    id: SroRuleId,
    metric: MetricName,
    unit: MetricUnit,
    comparison: MetricComparison,
    threshold: MeasurementValue,
    accepted_stability: Vec<MeasurementStability>,
    optimization_hint: String,
}

impl MetricRule {
    /// Creates a metric assessment rule.
    pub fn new(
        id: SroRuleId,
        metric: MetricName,
        unit: MetricUnit,
        comparison: MetricComparison,
        threshold: MeasurementValue,
        accepted_stability: Vec<MeasurementStability>,
        optimization_hint: impl Into<String>,
    ) -> SroResult<Self> {
        if !threshold.is_valid() {
            return Err(SroError::InvalidMeasurementValue);
        }

        let optimization_hint = optimization_hint.into();

        validate_identifier(&optimization_hint)?;

        let mut accepted_stability = accepted_stability;

        accepted_stability.sort_by_key(|value| value.as_str());
        accepted_stability.dedup();

        Ok(Self {
            id,
            metric,
            unit,
            comparison,
            threshold,
            accepted_stability,
            optimization_hint,
        })
    }

    /// Returns the rule identity.
    #[must_use]
    pub fn id(&self) -> &SroRuleId {
        &self.id
    }

    /// Returns the metric identity.
    #[must_use]
    pub fn metric(&self) -> &MetricName {
        &self.metric
    }

    /// Returns the metric unit.
    #[must_use]
    pub fn unit(&self) -> &MetricUnit {
        &self.unit
    }
}

// ============================================================================
// CONFIGURATION
// ============================================================================

/// SRO configuration.
///
/// All modern metric rules are explicitly supplied by the caller.
///
/// `legacy_history_capacity` is an optional operational retention
/// budget for the historical compatibility API.
///
/// Zero legacy capacity means that no telemetry history is retained.
#[derive(Debug, Clone, PartialEq)]
pub struct SroConfig {
    rules: BTreeMap<SroRuleId, MetricRule>,
    legacy_history_capacity: usize,

    /// Historical compatibility latency threshold.
    ///
    /// New integrations must use MetricRule.
    pub latency_threshold_us: u64,

    /// Historical compatibility instruction threshold.
    pub instruction_threshold: usize,

    /// Historical compatibility cache-miss threshold.
    pub cache_miss_threshold: f64,
}

impl Default for SroConfig {
    fn default() -> Self {
        Self {
            rules: BTreeMap::new(),
            legacy_history_capacity: 0,
            latency_threshold_us: 50,
            instruction_threshold: 10,
            cache_miss_threshold: 0.10,
        }
    }
}

impl SroConfig {
    /// Creates an empty modern configuration.
    ///
    /// No modern optimization rules are activated implicitly.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Registers a metric rule.
    ///
    /// Duplicate rule identities are rejected.
    pub fn register_rule(
        &mut self,
        rule: MetricRule,
    ) -> SroResult<()> {
        if self.rules.contains_key(&rule.id) {
            return Err(SroError::DuplicateRule(
                rule.id.to_string(),
            ));
        }

        self.rules.insert(rule.id.clone(), rule);

        Ok(())
    }

    /// Sets an optional legacy history-retention budget.
    ///
    /// This is not a limit on profiling or optimization.
    pub fn set_legacy_history_capacity(
        &mut self,
        capacity: usize,
    ) {
        self.legacy_history_capacity = capacity;
    }

    /// Returns the registered metric rules.
    #[must_use]
    pub fn rules(&self) -> &BTreeMap<SroRuleId, MetricRule> {
        &self.rules
    }

    /// Validates the configuration.
    pub fn validate(&self) -> SroResult<()> {
        if !self.cache_miss_threshold.is_finite()
            || !(0.0..=1.0).contains(&self.cache_miss_threshold)
        {
            return Err(SroError::InvalidConfiguration(
                "legacy cache-miss threshold must be within [0, 1]"
                    .to_owned(),
            ));
        }

        for rule in self.rules.values() {
            if !rule.threshold.is_valid() {
                return Err(SroError::InvalidMeasurementValue);
            }
        }

        Ok(())
    }
}

// ============================================================================
// OPTIMIZATION HINT
// ============================================================================

/// One evidence-backed optimization hint.
///
/// A hint is advisory and does not authorize a transformation.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationHint {
    rule_id: SroRuleId,
    metric_name: String,
    metric_unit: String,
    observed_value: MeasurementValue,
    threshold: MeasurementValue,
    comparison: MetricComparison,
    recommendation: String,
    source: String,
    stability: MeasurementStability,
    stage: Option<String>,
    compilation_identity: Option<String>,
}

impl OptimizationHint {
    /// Returns the originating rule.
    #[must_use]
    pub fn rule_id(&self) -> &SroRuleId {
        &self.rule_id
    }

    /// Returns the metric name.
    #[must_use]
    pub fn metric_name(&self) -> &str {
        &self.metric_name
    }

    /// Returns the observed value.
    #[must_use]
    pub const fn observed_value(&self) -> MeasurementValue {
        self.observed_value
    }

    /// Returns the recommended optimization direction.
    #[must_use]
    pub fn recommendation(&self) -> &str {
        &self.recommendation
    }

    /// Returns the measurement stability.
    #[must_use]
    pub const fn stability(&self) -> MeasurementStability {
        self.stability
    }

    /// Returns the associated compilation identity.
    #[must_use]
    pub fn compilation_identity(&self) -> Option<&str> {
        self.compilation_identity.as_deref()
    }

    /// Returns the associated compiler stage.
    #[must_use]
    pub fn stage(&self) -> Option<&str> {
        self.stage.as_deref()
    }
}

// ============================================================================
// ASSESSMENT
// ============================================================================

/// Outcome of evaluating one profiling measurement.
#[derive(Debug, Clone, PartialEq)]
pub struct SroAssessment {
    hints: Vec<OptimizationHint>,
}

impl SroAssessment {
    /// Returns all matching optimization hints.
    #[must_use]
    pub fn hints(&self) -> &[OptimizationHint] {
        &self.hints
    }

    /// Returns whether an optimization hint was produced.
    #[must_use]
    pub fn has_hints(&self) -> bool {
        !self.hints.is_empty()
    }

    /// Returns the number of matching rules.
    #[must_use]
    pub fn len(&self) -> usize {
        self.hints.len()
    }

    /// Returns whether the assessment is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.hints.is_empty()
    }
}

// ============================================================================
// LEGACY TELEMETRY COMPATIBILITY
// ============================================================================

/// Historical performance telemetry.
///
/// New profiling integrations should use
/// `compiler::profiling::measurements::Measurement`.
#[derive(Debug, Clone, PartialEq)]
#[deprecated(
    note = "Use canonical compiler profiling Measurement."
)]
pub struct PerformanceTelemetry {
    /// Backend identity.
    pub backend_name: String,

    /// Observed latency in microseconds.
    pub execution_latency_us: u64,

    /// Instruction count.
    pub instruction_count: usize,

    /// Cache-miss ratio.
    pub cache_miss_rate: f64,
}

#[allow(deprecated)]
impl PerformanceTelemetry {
    /// Creates validated legacy telemetry.
    pub fn new(
        backend_name: impl Into<String>,
        execution_latency_us: u64,
        instruction_count: usize,
        cache_miss_rate: f64,
    ) -> SroResult<Self> {
        let backend_name = backend_name.into();

        validate_identifier(&backend_name)?;

        if !cache_miss_rate.is_finite()
            || !(0.0..=1.0).contains(&cache_miss_rate)
        {
            return Err(SroError::InvalidCacheMissRate);
        }

        Ok(Self {
            backend_name,
            execution_latency_us,
            instruction_count,
            cache_miss_rate,
        })
    }

    /// Returns cache-miss percentage.
    #[must_use]
    pub fn cache_miss_percent(&self) -> f64 {
        self.cache_miss_rate * 100.0
    }
}

// ============================================================================
// LEGACY DECISION TYPES
// ============================================================================

/// Historical performance trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum OptimizationTrigger {
    /// High latency.
    HighLatency,

    /// High instruction count.
    HighInstructionCount,

    /// High cache-miss rate.
    HighCacheMissRate,
}

impl OptimizationTrigger {
    /// Returns the machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighLatency => "high_latency",
            Self::HighInstructionCount => "high_instruction_count",
            Self::HighCacheMissRate => "high_cache_miss_rate",
        }
    }
}

/// Historical SRO recommendation.
///
/// These values are recommendations, not executable transformations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SroAction {
    /// No optimization recommendation.
    NoChange,

    /// Investigate instruction-fusion opportunities.
    InstructionFusion,

    /// Investigate cache-sensitive optimization opportunities.
    CacheOptimization,

    /// Investigate multiple optimization opportunities.
    CombinedOptimization,
}

impl SroAction {
    /// Returns the machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoChange => "no_change",
            Self::InstructionFusion => "instruction_fusion",
            Self::CacheOptimization => "cache_optimization",
            Self::CombinedOptimization => "combined_optimization",
        }
    }
}

/// Historical structured SRO decision.
#[derive(Debug, Clone, PartialEq)]
#[deprecated(
    note = "Use SroAssessment and OptimizationHint."
)]
pub struct SroDecision {
    /// Backend identity.
    pub backend_name: String,

    /// Advisory action.
    pub action: SroAction,

    /// Triggering conditions.
    pub triggers: Vec<OptimizationTrigger>,

    /// Retained for compatibility.
    ///
    /// Always false: SRO does not modify instruction sets.
    pub instruction_set_changed: bool,
}

#[allow(deprecated)]
impl SroDecision {
    /// Returns whether an optimization recommendation exists.
    #[must_use]
    pub fn is_optimization_required(&self) -> bool {
        self.action != SroAction::NoChange
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// SRO errors.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum SroError {
    /// Invalid identifier.
    InvalidIdentifier(String),

    /// Invalid metric measurement.
    InvalidMeasurementValue,

    /// Incompatible numeric representations.
    NumericRepresentationMismatch,

    /// Invalid cache-miss ratio in legacy telemetry.
    InvalidCacheMissRate,

    /// Invalid configuration.
    InvalidConfiguration(String),

    /// Duplicate optimization rule.
    DuplicateRule(String),

    /// Synchronization or aggregation integration failure.
    IntegrationFailure(String),
}

impl fmt::Display for SroError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier(value) => {
                write!(formatter, "invalid SRO identifier: {value}")
            }

            Self::InvalidMeasurementValue => {
                formatter.write_str(
                    "invalid or non-finite SRO measurement",
                )
            }

            Self::NumericRepresentationMismatch => {
                formatter.write_str(
                    "incompatible numeric measurement representations",
                )
            }

            Self::InvalidCacheMissRate => {
                formatter.write_str(
                    "cache-miss ratio must be finite and within [0, 1]",
                )
            }

            Self::InvalidConfiguration(reason) => {
                write!(
                    formatter,
                    "invalid SRO configuration: {reason}"
                )
            }

            Self::DuplicateRule(rule) => {
                write!(
                    formatter,
                    "duplicate SRO rule: {rule}"
                )
            }

            Self::IntegrationFailure(reason) => {
                write!(
                    formatter,
                    "SRO integration failure: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for SroError {}

// ============================================================================
// SELF-REFLECTIVE OPTIMIZER
// ============================================================================

/// Self-Reflective Optimizer.
///
/// The modern interface consumes canonical profiling measurements.
///
/// Legacy history is retained only when an explicit operational
/// retention capacity is configured.
#[derive(Debug, Clone)]
pub struct SelfReflectiveOptimizer {
    config: SroConfig,
    legacy_history: VecDeque<PerformanceTelemetry>,
}

impl Default for SelfReflectiveOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(deprecated)]
impl SelfReflectiveOptimizer {
    /// Creates SRO with the empty default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: SroConfig::default(),
            legacy_history: VecDeque::new(),
        }
    }

    /// Creates SRO with explicit configuration.
    pub fn with_config(config: SroConfig) -> SroResult<Self> {
        config.validate()?;

        Ok(Self {
            config,
            legacy_history: VecDeque::new(),
        })
    }

    /// Returns the active configuration.
    #[must_use]
    pub fn config(&self) -> &SroConfig {
        &self.config
    }

    /// Assesses a canonical profiling measurement.
    ///
    /// The measurement must already have passed canonical
    /// profiling-measurement validation.
    ///
    /// No history is mutated.
    pub fn assess(
        &self,
        measurement: &Measurement,
    ) -> SroResult<SroAssessment> {
        let mut hints = Vec::new();

        for rule in self.config.rules.values() {
            if measurement.name() != &rule.metric
                || measurement.unit() != &rule.unit
            {
                continue;
            }

            if !rule
                .accepted_stability
                .contains(&measurement.stability())
            {
                continue;
            }

            if rule.comparison.matches(
                measurement.value(),
                rule.threshold,
            )? {
                hints.push(OptimizationHint {
                    rule_id: rule.id.clone(),
                    metric_name: measurement.name().as_str().to_owned(),
                    metric_unit: measurement.unit().as_str().to_owned(),
                    observed_value: measurement.value(),
                    threshold: rule.threshold,
                    comparison: rule.comparison,
                    recommendation: rule.optimization_hint.clone(),
                    source: measurement.source().as_str().to_owned(),
                    stability: measurement.stability(),
                    stage: measurement.stage().map(ToString::to_string),
                    compilation_identity: measurement
                        .compilation_identity()
                        .map(str::to_owned),
                });
            }
        }

        Ok(SroAssessment { hints })
    }

    /// Returns the number of retained legacy observations.
    #[must_use]
    pub fn history_len(&self) -> usize {
        self.legacy_history.len()
    }

    /// Returns whether legacy history exists.
    #[must_use]
    pub fn has_history(&self) -> bool {
        !self.legacy_history.is_empty()
    }

    /// Returns the latest retained legacy observation.
    #[must_use]
    pub fn latest_telemetry(&self) -> Option<&PerformanceTelemetry> {
        self.legacy_history.back()
    }

    /// Records validated legacy telemetry.
    ///
    /// If history retention is disabled, the observation is validated
    /// but not retained.
    pub fn record_telemetry(
        &mut self,
        telemetry: PerformanceTelemetry,
    ) -> SroResult<()> {
        PerformanceTelemetry::new(
            telemetry.backend_name.clone(),
            telemetry.execution_latency_us,
            telemetry.instruction_count,
            telemetry.cache_miss_rate,
        )?;

        let capacity = self.config.legacy_history_capacity;

        if capacity == 0 {
            return Ok(());
        }

        if self.legacy_history.len() >= capacity {
            self.legacy_history.pop_front();
        }

        self.legacy_history.push_back(telemetry);

        Ok(())
    }

    /// Evaluates legacy telemetry.
    ///
    /// This method does not modify any backend, instruction set,
    /// optimization pass or program representation.
    pub fn evaluate(
        &mut self,
        telemetry: &PerformanceTelemetry,
    ) -> SroResult<SroDecision> {
        let telemetry = PerformanceTelemetry::new(
            telemetry.backend_name.clone(),
            telemetry.execution_latency_us,
            telemetry.instruction_count,
            telemetry.cache_miss_rate,
        )?;

        self.record_telemetry(telemetry.clone())?;

        let triggers = self.evaluate_without_recording(&telemetry);

        let action = determine_action(&triggers);

        Ok(SroDecision {
            backend_name: telemetry.backend_name,
            action,
            triggers,

            // No instruction transformation has been executed.
            instruction_set_changed: false,
        })
    }

    /// Evaluates legacy telemetry without modifying history.
    pub fn evaluate_without_recording(
        &self,
        telemetry: &PerformanceTelemetry,
    ) -> Vec<OptimizationTrigger> {
        let mut triggers = Vec::new();

        if telemetry.execution_latency_us
            >= self.config.latency_threshold_us
        {
            triggers.push(OptimizationTrigger::HighLatency);
        }

        if telemetry.instruction_count
            >= self.config.instruction_threshold
        {
            triggers.push(OptimizationTrigger::HighInstructionCount);
        }

        if telemetry.cache_miss_rate.is_finite()
            && telemetry.cache_miss_rate
                >= self.config.cache_miss_threshold
        {
            triggers.push(OptimizationTrigger::HighCacheMissRate);
        }

        triggers
    }

    /// Legacy-compatible string recommendation.
    ///
    /// This method does not perform optimization.
    pub fn evaluate_and_optimize(
        &self,
        telemetry: &PerformanceTelemetry,
    ) -> String {
        match determine_action(
            &self.evaluate_without_recording(telemetry),
        ) {
            SroAction::NoChange => {
                "STANDARD_INSTRUCTION_SET".to_owned()
            }

            SroAction::InstructionFusion
            | SroAction::CombinedOptimization => {
                "OPTIMIZED_FUSED_INSTRUCTION_SET".to_owned()
            }

            SroAction::CacheOptimization => {
                "CACHE_OPTIMIZED_INSTRUCTION_SET".to_owned()
            }
        }
    }

    /// Returns a copy of the retained legacy history.
    #[must_use]
    pub fn telemetry_history(&self) -> Vec<PerformanceTelemetry> {
        self.legacy_history.iter().cloned().collect()
    }

    /// Clears legacy history.
    pub fn clear_history(&mut self) {
        self.legacy_history.clear();
    }
}

// ============================================================================
// LEGACY CLASSIFICATION
// ============================================================================

fn determine_action(
    triggers: &[OptimizationTrigger],
) -> SroAction {
    let has_instruction = triggers.iter().any(|trigger| {
        matches!(
            trigger,
            OptimizationTrigger::HighLatency
                | OptimizationTrigger::HighInstructionCount
        )
    });

    let has_cache = triggers.contains(
        &OptimizationTrigger::HighCacheMissRate,
    );

    match (has_instruction, has_cache) {
        (false, false) => SroAction::NoChange,

        (true, false) => SroAction::InstructionFusion,

        (false, true) => SroAction::CacheOptimization,

        (true, true) => SroAction::CombinedOptimization,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::profiling::measurements::{
        MeasurementSource,
    };

    use crate::distributed::observability::events::EventTimestamp;

    fn metric(name: &str) -> MetricName {
        MetricName::new(name).unwrap()
    }

    fn unit(name: &str) -> MetricUnit {
        MetricUnit::new(name).unwrap()
    }

    fn measurement(
        value: MeasurementValue,
        stability: MeasurementStability,
    ) -> Measurement {
        Measurement::new(
            metric("compiler.operation.count"),
            unit("operations"),
            value,
            EventTimestamp::new(1_800_000_000, 0).unwrap(),
            stability,
            MeasurementSource::new("compiler.test").unwrap(),
        )
        .unwrap()
    }

    fn rule() -> MetricRule {
        MetricRule::new(
            SroRuleId::new("performance.operation.count").unwrap(),
            metric("compiler.operation.count"),
            unit("operations"),
            MetricComparison::GreaterThan,
            MeasurementValue::Unsigned(100),
            vec![MeasurementStability::Deterministic],
            "Investigate operation reduction",
        )
        .unwrap()
    }

    #[test]
    fn api_version_is_available() {
        assert_eq!(SRO_API_VERSION, 2);
    }

    #[test]
    fn default_configuration_has_no_implicit_modern_rules() {
        let optimizer = SelfReflectiveOptimizer::new();

        assert!(optimizer.config().rules().is_empty());
        assert_eq!(optimizer.history_len(), 0);
    }

    #[test]
    fn accepts_registered_metric_rule() {
        let mut config = SroConfig::empty();

        config.register_rule(rule()).unwrap();

        assert_eq!(config.rules().len(), 1);
    }

    #[test]
    fn rejects_duplicate_rule_identity() {
        let mut config = SroConfig::empty();

        config.register_rule(rule()).unwrap();

        assert!(matches!(
            config.register_rule(rule()),
            Err(SroError::DuplicateRule(_))
        ));
    }

    #[test]
    fn deterministic_measurement_produces_hint() {
        let mut config = SroConfig::empty();

        config.register_rule(rule()).unwrap();

        let optimizer =
            SelfReflectiveOptimizer::with_config(config).unwrap();

        let result = optimizer
            .assess(&measurement(
                MeasurementValue::Unsigned(150),
                MeasurementStability::Deterministic,
            ))
            .unwrap();

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn environment_dependent_measurement_is_not_implicitly_accepted() {
        let mut config = SroConfig::empty();

        config.register_rule(rule()).unwrap();

        let optimizer =
            SelfReflectiveOptimizer::with_config(config).unwrap();

        let result = optimizer
            .assess(&measurement(
                MeasurementValue::Unsigned(150),
                MeasurementStability::EnvironmentDependent,
            ))
            .unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn integer_comparisons_preserve_exact_values() {
        let result = compare_values(
            MeasurementValue::Unsigned(u128::MAX),
            MeasurementValue::Unsigned(u128::MAX - 1),
        )
        .unwrap();

        assert!(result.is_gt());
    }

    #[test]
    fn incompatible_numeric_representations_are_rejected() {
        assert!(matches!(
            compare_values(
                MeasurementValue::Unsigned(10),
                MeasurementValue::Float(10.0),
            ),
            Err(SroError::NumericRepresentationMismatch)
        ));
    }

    #[test]
    fn non_finite_float_comparison_is_rejected() {
        assert!(matches!(
            compare_values(
                MeasurementValue::Float(f64::NAN),
                MeasurementValue::Float(10.0),
            ),
            Err(SroError::InvalidMeasurementValue)
        ));
    }

    #[test]
    fn no_history_is_retained_by_default() {
        #[allow(deprecated)]
        let telemetry = PerformanceTelemetry::new(
            "test.backend",
            100,
            100,
            0.1,
        )
        .unwrap();

        let mut optimizer = SelfReflectiveOptimizer::new();

        optimizer.record_telemetry(telemetry).unwrap();

        assert_eq!(optimizer.history_len(), 0);
    }

    #[test]
    fn configured_history_retention_is_respected() {
        #[allow(deprecated)]
        let telemetry = PerformanceTelemetry::new(
            "test.backend",
            100,
            100,
            0.1,
        )
        .unwrap();

        let mut config = SroConfig::default();

        config.set_legacy_history_capacity(2);

        let mut optimizer =
            SelfReflectiveOptimizer::with_config(config).unwrap();

        for _ in 0..3 {
            optimizer
                .record_telemetry(telemetry.clone())
                .unwrap();
        }

        assert_eq!(optimizer.history_len(), 2);
    }

    #[test]
    fn invalid_identifier_is_rejected() {
        assert!(SroRuleId::new("").is_err());
        assert!(SroRuleId::new("invalid\nidentifier").is_err());
    }

    #[test]
    fn clearing_history_is_safe() {
        let mut optimizer = SelfReflectiveOptimizer::new();

        optimizer.clear_history();

        assert!(!optimizer.has_history());
    }
}