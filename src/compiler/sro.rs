//! Zamani Compiler — Self-Reflective Optimizer (SRO)
//!
//! SRO evaluates compiler/backend performance telemetry and produces
//! deterministic optimization decisions.
//!
//! Design principles:
//! - SRO observes telemetry; it does not execute generated code.
//! - Decisions are deterministic for identical inputs and configuration.
//! - Telemetry is validated before being used.
//! - Thresholds are configurable rather than hard-coded.
//! - Optimization history is bounded.
//! - The legacy string-returning API is preserved for compatibility.
//!
//! SRO is intended to work with instruction-fusion and backend optimization
//! passes, but does not duplicate those implementations.

use std::collections::VecDeque;
use std::fmt;

/// Default latency threshold in microseconds.
pub const DEFAULT_LATENCY_THRESHOLD_US: u64 = 50;

/// Default instruction-count threshold.
pub const DEFAULT_INSTRUCTION_THRESHOLD: usize = 10;

/// Default cache-miss threshold.
///
/// A value of `0.10` means 10%.
pub const DEFAULT_CACHE_MISS_THRESHOLD: f64 = 0.10;

/// Default maximum number of telemetry records retained.
pub const DEFAULT_HISTORY_CAPACITY: usize = 256;

/// Maximum supported backend-name length.
pub const MAX_BACKEND_NAME_LENGTH: usize = 256;

/// Performance telemetry supplied to SRO.
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceTelemetry {
    /// Backend producing the telemetry.
    pub backend_name: String,

    /// Observed execution latency in microseconds.
    pub execution_latency_us: u64,

    /// Number of instructions executed/generated.
    pub instruction_count: usize,

    /// Cache miss rate represented as a ratio in `[0.0, 1.0]`.
    pub cache_miss_rate: f64,
}

impl PerformanceTelemetry {
    /// Creates validated telemetry.
    pub fn new(
        backend_name: impl Into<String>,
        execution_latency_us: u64,
        instruction_count: usize,
        cache_miss_rate: f64,
    ) -> Result<Self, SroError> {
        let backend_name = backend_name.into();

        validate_backend_name(&backend_name)?;

        if !cache_miss_rate.is_finite() {
            return Err(SroError::InvalidCacheMissRate(cache_miss_rate));
        }

        if !(0.0..=1.0).contains(&cache_miss_rate) {
            return Err(SroError::InvalidCacheMissRate(cache_miss_rate));
        }

        Ok(Self {
            backend_name,
            execution_latency_us,
            instruction_count,
            cache_miss_rate,
        })
    }

    /// Returns the cache-miss percentage.
    pub fn cache_miss_percent(&self) -> f64 {
        self.cache_miss_rate * 100.0
    }
}

/// Configurable SRO decision thresholds.
#[derive(Debug, Clone, PartialEq)]
pub struct SroConfig {
    /// Trigger optimization when latency is at or above this value.
    pub latency_threshold_us: u64,

    /// Trigger optimization when instruction count is at or above this
    /// value.
    pub instruction_threshold: usize,

    /// Trigger optimization when cache-miss rate is at or above this value.
    pub cache_miss_threshold: f64,

    /// Maximum number of telemetry samples retained.
    pub history_capacity: usize,
}

impl Default for SroConfig {
    fn default() -> Self {
        Self {
            latency_threshold_us: DEFAULT_LATENCY_THRESHOLD_US,
            instruction_threshold: DEFAULT_INSTRUCTION_THRESHOLD,
            cache_miss_threshold: DEFAULT_CACHE_MISS_THRESHOLD,
            history_capacity: DEFAULT_HISTORY_CAPACITY,
        }
    }
}

impl SroConfig {
    pub fn validate(&self) -> Result<(), SroError> {
        if self.history_capacity == 0 {
            return Err(SroError::InvalidConfiguration(
                "history capacity must be greater than zero".to_string(),
            ));
        }

        if !self.cache_miss_threshold.is_finite()
            || !(0.0..=1.0).contains(&self.cache_miss_threshold)
        {
            return Err(SroError::InvalidConfiguration(
                "cache-miss threshold must be within [0.0, 1.0]".to_string(),
            ));
        }

        Ok(())
    }
}

/// Individual performance conditions observed by SRO.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationTrigger {
    HighLatency,
    HighInstructionCount,
    HighCacheMissRate,
}

impl OptimizationTrigger {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HighLatency => "high_latency",
            Self::HighInstructionCount => "high_instruction_count",
            Self::HighCacheMissRate => "high_cache_miss_rate",
        }
    }
}

/// Action recommended by SRO.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SroAction {
    /// Keep the current instruction strategy.
    NoChange,

    /// Run instruction-fusion/macro-op analysis.
    InstructionFusion,

    /// Investigate cache-sensitive transformations.
    CacheOptimization,

    /// Multiple independent performance problems were detected.
    CombinedOptimization,
}

impl SroAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoChange => "no_change",
            Self::InstructionFusion => "instruction_fusion",
            Self::CacheOptimization => "cache_optimization",
            Self::CombinedOptimization => "combined_optimization",
        }
    }
}

/// Structured optimization decision.
#[derive(Debug, Clone, PartialEq)]
pub struct SroDecision {
    /// Backend to which the decision applies.
    pub backend_name: String,

    /// Selected action.
    pub action: SroAction,

    /// Reasons that caused the decision.
    pub triggers: Vec<OptimizationTrigger>,

    /// Whether the decision requests a different instruction strategy.
    pub instruction_set_changed: bool,
}

impl SroDecision {
    pub fn is_optimization_required(&self) -> bool {
        self.action != SroAction::NoChange
    }
}

/// SRO errors.
#[derive(Debug, Clone, PartialEq)]
pub enum SroError {
    InvalidBackendName(String),
    InvalidCacheMissRate(f64),
    InvalidConfiguration(String),
}

impl fmt::Display for SroError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBackendName(name) => {
                write!(formatter, "invalid backend name '{}'", name)
            }

            Self::InvalidCacheMissRate(rate) => {
                write!(
                    formatter,
                    "invalid cache miss rate '{}'; expected finite value in [0.0, 1.0]",
                    rate
                )
            }

            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid SRO configuration: {}", message)
            }
        }
    }
}

impl std::error::Error for SroError {}

/// Self-Reflective Optimizer.
///
/// The optimizer retains a bounded history of observations and produces
/// deterministic optimization decisions.
#[derive(Debug, Clone)]
pub struct SelfReflectiveOptimizer {
    config: SroConfig,
    history: VecDeque<PerformanceTelemetry>,
}

impl Default for SelfReflectiveOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl SelfReflectiveOptimizer {
    /// Creates SRO using the production default configuration.
    pub fn new() -> Self {
        Self::with_config(SroConfig::default())
            .expect("default SRO configuration must be valid")
    }

    /// Creates SRO with explicit configuration.
    pub fn with_config(config: SroConfig) -> Result<Self, SroError> {
        config.validate()?;

        Ok(Self {
            history: VecDeque::with_capacity(config.history_capacity),
            config,
        })
    }

    /// Returns the active configuration.
    pub fn config(&self) -> &SroConfig {
        &self.config
    }

    /// Returns the number of retained telemetry samples.
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Returns whether SRO has retained any telemetry.
    pub fn has_history(&self) -> bool {
        !self.history.is_empty()
    }

    /// Returns the most recent telemetry sample.
    pub fn latest_telemetry(&self) -> Option<&PerformanceTelemetry> {
        self.history.back()
    }

    /// Records validated telemetry.
    pub fn record_telemetry(
        &mut self,
        telemetry: PerformanceTelemetry,
    ) -> Result<(), SroError> {
        validate_backend_name(&telemetry.backend_name)?;

        if !telemetry.cache_miss_rate.is_finite()
            || !(0.0..=1.0).contains(&telemetry.cache_miss_rate)
        {
            return Err(SroError::InvalidCacheMissRate(
                telemetry.cache_miss_rate,
            ));
        }

        if self.history.len() >= self.config.history_capacity {
            self.history.pop_front();
        }

        self.history.push_back(telemetry);

        Ok(())
    }

    /// Evaluates telemetry and returns a structured optimization decision.
    pub fn evaluate(
        &mut self,
        telemetry: &PerformanceTelemetry,
    ) -> Result<SroDecision, SroError> {
        validate_backend_name(&telemetry.backend_name)?;

        if !telemetry.cache_miss_rate.is_finite()
            || !(0.0..=1.0).contains(&telemetry.cache_miss_rate)
        {
            return Err(SroError::InvalidCacheMissRate(
                telemetry.cache_miss_rate,
            ));
        }

        self.record_telemetry(telemetry.clone())?;

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

        if telemetry.cache_miss_rate
            >= self.config.cache_miss_threshold
        {
            triggers.push(OptimizationTrigger::HighCacheMissRate);
        }

        let action = determine_action(&triggers);

        Ok(SroDecision {
            backend_name: telemetry.backend_name.clone(),
            action,
            triggers,
            instruction_set_changed: matches!(
                action,
                SroAction::InstructionFusion
                    | SroAction::CombinedOptimization
            ),
        })
    }

    /// Legacy-compatible evaluation API.
    ///
    /// Existing repository callers expecting the old string-based result can
    /// continue using this method. New code should use `evaluate()`.
    pub fn evaluate_and_optimize(
        &self,
        telemetry: &PerformanceTelemetry,
    ) -> String {
        let triggers = self.evaluate_without_recording(telemetry);

        match determine_action(&triggers) {
            SroAction::NoChange => {
                "STANDARD_INSTRUCTION_SET".to_string()
            }

            SroAction::InstructionFusion
            | SroAction::CombinedOptimization => {
                "OPTIMIZED_FUSED_INSTRUCTION_SET".to_string()
            }

            SroAction::CacheOptimization => {
                "CACHE_OPTIMIZED_INSTRUCTION_SET".to_string()
            }
        }
    }

    /// Evaluates without modifying history.
    ///
    /// This is useful for speculative planning.
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

    /// Returns an immutable snapshot of the telemetry history.
    pub fn telemetry_history(&self) -> Vec<PerformanceTelemetry> {
        self.history.iter().cloned().collect()
    }

    /// Clears retained telemetry.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

/// Determines the action from independent performance triggers.
fn determine_action(triggers: &[OptimizationTrigger]) -> SroAction {
    let has_latency_or_instruction = triggers.iter().any(|trigger| {
        matches!(
            trigger,
            OptimizationTrigger::HighLatency
                | OptimizationTrigger::HighInstructionCount
        )
    });

    let has_cache = triggers
        .iter()
        .any(|trigger| *trigger == OptimizationTrigger::HighCacheMissRate);

    match (has_latency_or_instruction, has_cache) {
        (false, false) => SroAction::NoChange,
        (true, false) => SroAction::InstructionFusion,
        (false, true) => SroAction::CacheOptimization,
        (true, true) => SroAction::CombinedOptimization,
    }
}

/// Validates a backend identifier.
fn validate_backend_name(name: &str) -> Result<(), SroError> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Err(SroError::InvalidBackendName(
            "backend name cannot be empty".to_string(),
        ));
    }

    if trimmed.len() > MAX_BACKEND_NAME_LENGTH {
        return Err(SroError::InvalidBackendName(format!(
            "backend name exceeds {} bytes",
            MAX_BACKEND_NAME_LENGTH
        )));
    }

    if trimmed.chars().any(char::is_control) {
        return Err(SroError::InvalidBackendName(
            "backend name contains control characters".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn telemetry(
        latency: u64,
        instructions: usize,
        cache_miss_rate: f64,
    ) -> PerformanceTelemetry {
        PerformanceTelemetry::new(
            "test-backend",
            latency,
            instructions,
            cache_miss_rate,
        )
        .unwrap()
    }

    #[test]
    fn default_configuration_is_valid() {
        let optimizer = SelfReflectiveOptimizer::new();

        assert_eq!(
            optimizer.config().latency_threshold_us,
            DEFAULT_LATENCY_THRESHOLD_US
        );
    }

    #[test]
    fn low_cost_telemetry_requires_no_change() {
        let mut optimizer = SelfReflectiveOptimizer::new();

        let decision = optimizer
            .evaluate(&telemetry(10, 5, 0.01))
            .unwrap();

        assert_eq!(decision.action, SroAction::NoChange);
        assert!(decision.triggers.is_empty());
    }

    #[test]
    fn high_latency_triggers_instruction_fusion() {
        let mut optimizer = SelfReflectiveOptimizer::new();

        let decision = optimizer
            .evaluate(&telemetry(100, 5, 0.01))
            .unwrap();

        assert_eq!(decision.action, SroAction::InstructionFusion);
        assert!(decision
            .triggers
            .contains(&OptimizationTrigger::HighLatency));
    }

    #[test]
    fn high_instruction_count_triggers_instruction_fusion() {
        let mut optimizer = SelfReflectiveOptimizer::new();

        let decision = optimizer
            .evaluate(&telemetry(10, 20, 0.01))
            .unwrap();

        assert_eq!(decision.action, SroAction::InstructionFusion);
        assert!(decision
            .triggers
            .contains(&OptimizationTrigger::HighInstructionCount));
    }

    #[test]
    fn high_cache_miss_rate_triggers_cache_optimization() {
        let mut optimizer = SelfReflectiveOptimizer::new();

        let decision = optimizer
            .evaluate(&telemetry(10, 5, 0.25))
            .unwrap();

        assert_eq!(decision.action, SroAction::CacheOptimization);
        assert!(decision
            .triggers
            .contains(&OptimizationTrigger::HighCacheMissRate));
    }

    #[test]
    fn multiple_conditions_trigger_combined_optimization() {
        let mut optimizer = SelfReflectiveOptimizer::new();

        let decision = optimizer
            .evaluate(&telemetry(100, 20, 0.25))
            .unwrap();

        assert_eq!(
            decision.action,
            SroAction::CombinedOptimization
        );

        assert_eq!(decision.triggers.len(), 3);
    }

    #[test]
    fn legacy_api_preserves_optimized_result() {
        let optimizer = SelfReflectiveOptimizer::new();

        let result =
            optimizer.evaluate_and_optimize(&telemetry(100, 20, 0.01));

        assert_eq!(
            result,
            "OPTIMIZED_FUSED_INSTRUCTION_SET"
        );
    }

    #[test]
    fn legacy_api_preserves_standard_result() {
        let optimizer = SelfReflectiveOptimizer::new();

        let result =
            optimizer.evaluate_and_optimize(&telemetry(10, 5, 0.01));

        assert_eq!(
            result,
            "STANDARD_INSTRUCTION_SET"
        );
    }

    #[test]
    fn legacy_api_supports_cache_optimization() {
        let optimizer = SelfReflectiveOptimizer::new();

        let result =
            optimizer.evaluate_and_optimize(&telemetry(10, 5, 0.50));

        assert_eq!(
            result,
            "CACHE_OPTIMIZED_INSTRUCTION_SET"
        );
    }

    #[test]
    fn telemetry_history_is_bounded() {
        let config = SroConfig {
            history_capacity: 2,
            ..SroConfig::default()
        };

        let mut optimizer =
            SelfReflectiveOptimizer::with_config(config).unwrap();

        optimizer
            .evaluate(&telemetry(1, 1, 0.0))
            .unwrap();

        optimizer
            .evaluate(&telemetry(2, 2, 0.0))
            .unwrap();

        optimizer
            .evaluate(&telemetry(3, 3, 0.0))
            .unwrap();

        assert_eq!(optimizer.history_len(), 2);

        assert_eq!(
            optimizer.latest_telemetry().unwrap().execution_latency_us,
            3
        );
    }

    #[test]
    fn speculative_evaluation_does_not_change_history() {
        let optimizer = SelfReflectiveOptimizer::new();

        let result =
            optimizer.evaluate_without_recording(&telemetry(100, 20, 0.01));

        assert_eq!(
            result,
            vec![
                OptimizationTrigger::HighLatency,
                OptimizationTrigger::HighInstructionCount
            ]
        );

        assert_eq!(optimizer.history_len(), 0);
    }

    #[test]
    fn invalid_cache_rate_is_rejected() {
        let result =
            PerformanceTelemetry::new("backend", 10, 10, 1.5);

        assert!(matches!(
            result,
            Err(SroError::InvalidCacheMissRate(_))
        ));
    }

    #[test]
    fn nan_cache_rate_is_rejected() {
        let result =
            PerformanceTelemetry::new("backend", 10, 10, f64::NAN);

        assert!(matches!(
            result,
            Err(SroError::InvalidCacheMissRate(_))
        ));
    }

    #[test]
    fn empty_backend_name_is_rejected() {
        let result =
            PerformanceTelemetry::new("", 10, 10, 0.1);

        assert!(matches!(
            result,
            Err(SroError::InvalidBackendName(_))
        ));
    }

    #[test]
    fn control_characters_in_backend_name_are_rejected() {
        let result =
            PerformanceTelemetry::new("backend\n", 10, 10, 0.1);

        assert!(matches!(
            result,
            Err(SroError::InvalidBackendName(_))
        ));
    }

    #[test]
    fn configuration_rejects_zero_history_capacity() {
        let config = SroConfig {
            history_capacity: 0,
            ..SroConfig::default()
        };

        assert!(SelfReflectiveOptimizer::with_config(config).is_err());
    }

    #[test]
    fn configuration_rejects_invalid_cache_threshold() {
        let config = SroConfig {
            cache_miss_threshold: 2.0,
            ..SroConfig::default()
        };

        assert!(SelfReflectiveOptimizer::with_config(config).is_err());
    }

    #[test]
    fn clear_history_removes_all_samples() {
        let mut optimizer = SelfReflectiveOptimizer::new();

        optimizer
            .evaluate(&telemetry(100, 10, 0.1))
            .unwrap();

        assert_eq!(optimizer.history_len(), 1);

        optimizer.clear_history();

        assert_eq!(optimizer.history_len(), 0);
        assert!(!optimizer.has_history());
    }

    #[test]
    fn decision_marks_instruction_set_change_correctly() {
        let mut optimizer = SelfReflectiveOptimizer::new();

        let decision = optimizer
            .evaluate(&telemetry(100, 20, 0.25))
            .unwrap();

        assert!(decision.instruction_set_changed);
    }
}