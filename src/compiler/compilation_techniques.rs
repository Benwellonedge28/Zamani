//! Zamani Universal Meta-Compiler (UMC): Compilation Techniques.
//!
//! This module owns high-level compilation strategy selection and validation.
//!
//! It deliberately does NOT implement:
//! - parsing;
//! - semantic analysis;
//! - IR generation;
//! - optimization passes;
//! - machine-code generation;
//! - executable linking.
//!
//! Those responsibilities belong to the corresponding compiler/backend
//! modules. This module therefore acts as a deterministic orchestration
//! boundary.
//!
//! A critical invariant of this module is:
//!
//! > A strategy-level request must never claim successful compilation by
//! > returning an empty artifact.
//!
//! If the concrete backend is unavailable, an explicit error is returned.

use std::collections::HashMap;
use std::fmt;

use crate::ast::{Identifier, Program};
use crate::compiler_types::{CompilationTarget, OptimizationLevel};

// -----------------------------------------------------------------------------
// Repository-compatible target aliases
// -----------------------------------------------------------------------------

/// Compatibility alias for callers using the historical API name.
pub type TargetPlatform = CompilationTarget;

// -----------------------------------------------------------------------------
// Compiled artifact
// -----------------------------------------------------------------------------

/// Result of a successful concrete compilation.
///
/// `CompiledBinary` represents an artifact produced by an actual backend.
/// This module itself does not manufacture artifact bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledBinary {
    /// Generated artifact bytes.
    pub data: Vec<u8>,

    /// Stable artifact format identifier.
    pub format: String,
}

impl CompiledBinary {
    /// Creates an artifact after validating that it contains data.
    pub fn new(
        data: Vec<u8>,
        format: impl Into<String>,
    ) -> Result<Self, CompilationTechniqueError> {
        let format = format.into();

        if format.trim().is_empty() {
            return Err(CompilationTechniqueError::InvalidConfiguration(
                "compiled artifact format cannot be empty".to_string(),
            ));
        }

        if data.is_empty() {
            return Err(CompilationTechniqueError::EmptyArtifact {
                format,
            });
        }

        Ok(Self { data, format })
    }

    /// Returns whether the artifact contains bytes.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the artifact size.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Structured errors emitted by the strategy layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilationTechniqueError {
    /// No strategy has been selected.
    StrategyNotSelected,

    /// A configuration is invalid.
    InvalidConfiguration(String),

    /// A target cannot be used by the requested strategy.
    UnsupportedTarget {
        strategy: String,
        target: String,
    },

    /// A strategy is known but requires a backend not available here.
    BackendRequired {
        strategy: String,
    },

    /// A mixed-mode strategy contains no children.
    EmptyMixedMode,

    /// A mixed-mode strategy contains another mixed-mode strategy.
    NestedMixedMode,

    /// Compilation returned no bytes.
    EmptyArtifact {
        format: String,
    },

    /// An unknown paradigm was requested.
    UnknownParadigm {
        paradigm: String,
        available: Vec<String>,
    },
}

impl fmt::Display for CompilationTechniqueError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::StrategyNotSelected => {
                write!(formatter, "no compilation strategy has been selected")
            }

            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid compilation configuration: {message}")
            }

            Self::UnsupportedTarget { strategy, target } => {
                write!(
                    formatter,
                    "strategy '{strategy}' does not support target '{target}'"
                )
            }

            Self::BackendRequired { strategy } => {
                write!(
                    formatter,
                    "strategy '{strategy}' requires its concrete backend"
                )
            }

            Self::EmptyMixedMode => {
                write!(
                    formatter,
                    "MixedMode requires at least one strategy"
                )
            }

            Self::NestedMixedMode => {
                write!(
                    formatter,
                    "nested MixedMode strategies are not permitted"
                )
            }

            Self::EmptyArtifact { format } => {
                write!(
                    formatter,
                    "backend produced an empty '{format}' artifact"
                )
            }

            Self::UnknownParadigm {
                paradigm,
                available,
            } => {
                write!(
                    formatter,
                    "unknown paradigm '{paradigm}'; available paradigms: {}",
                    available.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for CompilationTechniqueError {}

// -----------------------------------------------------------------------------
// Module lifecycle
// -----------------------------------------------------------------------------

/// Initializes the strategy subsystem.
///
/// The subsystem is intentionally stateless, so initialization currently has
/// no side effects. The function is retained for compatibility with the
/// compiler lifecycle API.
pub fn init_compilation_techniques() {}

/// Shuts down the strategy subsystem.
///
/// The subsystem owns no global resources.
pub fn shutdown_compilation_techniques() {}

// -----------------------------------------------------------------------------
// Core strategies
// -----------------------------------------------------------------------------

/// High-level compilation strategy.
#[derive(Debug, Clone, PartialEq)]
pub enum CompilationStrategy {
    /// Traditional whole-program compilation.
    AheadOfTime(AotConfig),

    /// Runtime-oriented compilation.
    JustInTime(JitConfig),

    /// Feedback-driven optimization.
    AdaptiveOptimization(AdaptiveOptConfig),

    /// Translation between programming paradigms.
    MultiParadigmTranspilation(TranspilationConfig),

    /// Hardware-description synthesis.
    HardwareSynthesis(HdlSynthConfig),

    /// Quantum compilation.
    QuantumCompilation(QuantumCompileConfig),

    /// Nano/NACU compilation.
    NanoCompilation(NanoCompileConfig),

    /// Combination of multiple independent strategies.
    MixedMode(Vec<CompilationStrategy>),
}

impl CompilationStrategy {
    /// Stable strategy identifier.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::AheadOfTime(_) => "aot",
            Self::JustInTime(_) => "jit",
            Self::AdaptiveOptimization(_) => "adaptive",
            Self::MultiParadigmTranspilation(_) => "transpilation",
            Self::HardwareSynthesis(_) => "hardware",
            Self::QuantumCompilation(_) => "quantum",
            Self::NanoCompilation(_) => "nano",
            Self::MixedMode(_) => "mixed",
        }
    }

    /// Validates the complete strategy configuration.
    pub fn validate(
        &self,
    ) -> Result<(), CompilationTechniqueError> {
        match self {
            Self::AheadOfTime(config) => config.validate(),

            Self::JustInTime(config) => config.validate(),

            Self::AdaptiveOptimization(config) => config.validate(),

            Self::MultiParadigmTranspilation(config) => {
                config.validate()
            }

            Self::HardwareSynthesis(config) => config.validate(),

            Self::QuantumCompilation(config) => config.validate(),

            Self::NanoCompilation(config) => config.validate(),

            Self::MixedMode(strategies) => {
                if strategies.is_empty() {
                    return Err(
                        CompilationTechniqueError::EmptyMixedMode
                    );
                }

                for strategy in strategies {
                    if matches!(
                        strategy,
                        CompilationStrategy::MixedMode(_)
                    ) {
                        return Err(
                            CompilationTechniqueError::NestedMixedMode
                        );
                    }

                    strategy.validate()?;
                }

                Ok(())
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Strategy configuration
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct AotConfig {
    pub optimization_level: OptimizationLevel,
    pub target: TargetPlatform,
}

impl AotConfig {
    fn validate(
        &self,
    ) -> Result<(), CompilationTechniqueError> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JitConfig {
    pub enable_profiling: bool,
    pub recompile_threshold: f32,
}

impl JitConfig {
    fn validate(
        &self,
    ) -> Result<(), CompilationTechniqueError> {
        if !self.recompile_threshold.is_finite()
            || self.recompile_threshold < 0.0
        {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "JIT recompile threshold must be finite and non-negative"
                        .to_string(),
                ),
            );
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdaptiveOptConfig {
    pub strategy_model: AiStrategyModel,
    pub learning_rate: f32,
}

impl AdaptiveOptConfig {
    fn validate(
        &self,
    ) -> Result<(), CompilationTechniqueError> {
        if !self.learning_rate.is_finite()
            || self.learning_rate <= 0.0
        {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "adaptive learning rate must be finite and greater than zero"
                        .to_string(),
                ),
            );
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TranspilationConfig {
    pub source_paradigm: Identifier,
    pub target_paradigm: Identifier,
}

impl TranspilationConfig {
    fn validate(
        &self,
    ) -> Result<(), CompilationTechniqueError> {
        if self.source_paradigm.0.trim().is_empty()
            || self.target_paradigm.0.trim().is_empty()
        {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "source and target paradigms cannot be empty"
                        .to_string(),
                ),
            );
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HdlSynthConfig {
    pub target_chip_design: Identifier,
    pub clock_speed_mhz: f32,
    pub power_budget_mw: f32,
}

impl HdlSynthConfig {
    fn validate(
        &self,
    ) -> Result<(), CompilationTechniqueError> {
        if self.target_chip_design.0.trim().is_empty() {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "target chip design cannot be empty".to_string(),
                ),
            );
        }

        if !self.clock_speed_mhz.is_finite()
            || self.clock_speed_mhz <= 0.0
        {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "clock speed must be finite and greater than zero"
                        .to_string(),
                ),
            );
        }

        if !self.power_budget_mw.is_finite()
            || self.power_budget_mw <= 0.0
        {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "power budget must be finite and greater than zero"
                        .to_string(),
                ),
            );
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCompileConfig {
    pub qubit_count: u32,
    pub error_correction_scheme: Identifier,
    pub target_qpu_architecture: Identifier,
}

impl QuantumCompileConfig {
    fn validate(
        &self,
    ) -> Result<(), CompilationTechniqueError> {
        if self.qubit_count == 0 {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "quantum compilation requires at least one qubit"
                        .to_string(),
                ),
            );
        }

        if self.error_correction_scheme.0.trim().is_empty() {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "error-correction scheme cannot be empty".to_string(),
                ),
            );
        }

        if self.target_qpu_architecture.0.trim().is_empty() {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "target QPU architecture cannot be empty"
                        .to_string(),
                ),
            );
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NanoCompileConfig {
    pub agent_swarm_size: u32,
    pub target_nacu_version: Identifier,
    pub bio_compatibility_mode: bool,
}

impl NanoCompileConfig {
    fn validate(
        &self,
    ) -> Result<(), CompilationTechniqueError> {
        if self.agent_swarm_size == 0 {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "nano compilation requires at least one agent"
                        .to_string(),
                ),
            );
        }

        if self.target_nacu_version.0.trim().is_empty() {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "target NACU version cannot be empty".to_string(),
                ),
            );
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// AI strategy model
// -----------------------------------------------------------------------------

/// Deterministic metadata used by strategy selection.
///
/// This is intentionally not an ML inference engine. External AI systems may
/// populate the profile, but the compiler remains deterministic unless a
/// caller explicitly provides different metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct AiStrategyModel {
    pub model_id: Identifier,
    pub performance_profile: HashMap<String, Vec<f32>>,
    pub ethical_constraints: Vec<String>,
}

impl AiStrategyModel {
    pub fn new(model_id: Identifier) -> Self {
        Self {
            model_id,
            performance_profile: HashMap::new(),
            ethical_constraints: Vec::new(),
        }
    }

    /// Selects a deterministic baseline strategy.
    ///
    /// The performance profile is used only when it contains a valid
    /// `optimization_level` hint:
    ///
    /// - `none`
    /// - `basic`
    /// - `aggressive`
    /// - `full`
    ///
    /// Unknown or malformed hints safely fall back to `Basic`.
    pub fn predict_strategy(
        &self,
        target_env: &TargetPlatform,
    ) -> Result<CompilationStrategy, String> {
        let optimization_level = self
            .performance_profile
            .get("optimization_level")
            .and_then(|values| values.first())
            .and_then(|value| {
                if !value.is_finite() {
                    return None;
                }

                match *value as u32 {
                    0 => Some(OptimizationLevel::None),
                    1 => Some(OptimizationLevel::Basic),
                    2 => Some(OptimizationLevel::Aggressive),
                    3 => Some(OptimizationLevel::Full),
                    _ => None,
                }
            })
            .unwrap_or(OptimizationLevel::Basic);

        Ok(CompilationStrategy::AheadOfTime(AotConfig {
            optimization_level,
            target: target_env.clone(),
        }))
    }
}

// -----------------------------------------------------------------------------
// Compilation events
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationEvent {
    /// Monotonic event sequence number.
    ///
    /// This field retains the historical `timestamp` name for API
    /// compatibility. It is deliberately a sequence number rather than wall
    /// clock time, making compiler output deterministic.
    pub timestamp: u64,

    pub event_type: String,
    pub details: HashMap<String, String>,
}

// -----------------------------------------------------------------------------
// Compilation orchestrator
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CompilationOrchestrator {
    pub current_strategy: Option<CompilationStrategy>,
    pub ai_model: AiStrategyModel,
    pub compilation_log: Vec<CompilationEvent>,
}

impl CompilationOrchestrator {
    pub fn new(ai_model: AiStrategyModel) -> Self {
        Self {
            current_strategy: None,
            ai_model,
            compilation_log: Vec::new(),
        }
    }

    pub fn default_for_target(target: &TargetPlatform) -> Self {
        let model_id = Identifier(
            "default_compilation_strategy".to_string(),
            crate::source_map::Span::dummy(),
        );

        let mut orchestrator =
            Self::new(AiStrategyModel::new(model_id));

        orchestrator.current_strategy = orchestrator
            .ai_model
            .predict_strategy(target)
            .ok();

        orchestrator
    }

    /// Sets and validates an explicit strategy.
    pub fn set_strategy(
        &mut self,
        strategy: CompilationStrategy,
    ) -> Result<(), CompilationTechniqueError> {
        strategy.validate()?;
        self.current_strategy = Some(strategy);
        Ok(())
    }

    /// Returns the current strategy.
    pub fn strategy(&self) -> Option<&CompilationStrategy> {
        self.current_strategy.as_ref()
    }

    /// Selects a strategy using the deterministic strategy model.
    pub fn select_strategy(
        &mut self,
        target: &TargetPlatform,
    ) -> Result<CompilationStrategy, String> {
        let strategy = self.ai_model.predict_strategy(target)?;

        strategy
            .validate()
            .map_err(|error| error.to_string())?;

        self.current_strategy = Some(strategy.clone());

        self.log_event(
            "strategy_selected",
            HashMap::from([(
                "strategy".to_string(),
                strategy.name().to_string(),
            )]),
        );

        Ok(strategy)
    }

    /// Records a deterministic compiler event.
    pub fn log_event(
        &mut self,
        event_type: impl Into<String>,
        details: HashMap<String, String>,
    ) {
        let timestamp = self.compilation_log.len() as u64;

        self.compilation_log.push(CompilationEvent {
            timestamp,
            event_type: event_type.into(),
            details,
        });
    }

    pub fn event_count(&self) -> usize {
        self.compilation_log.len()
    }

    /// Strategy-level compilation entry point.
    ///
    /// This function deliberately refuses to fabricate an artifact. Concrete
    /// backend integration must call the appropriate backend module and then
    /// construct `CompiledBinary` through `CompiledBinary::new`.
    pub fn compile_program(
        &mut self,
        _program: Program,
        target: TargetPlatform,
    ) -> Result<CompiledBinary, String> {
        let strategy = match self.current_strategy.clone() {
            Some(strategy) => strategy,
            None => self.select_strategy(&target)?,
        };

        strategy
            .validate()
            .map_err(|error| error.to_string())?;

        self.log_event(
            "compilation_requested",
            HashMap::from([(
                "strategy".to_string(),
                strategy.name().to_string(),
            )]),
        );

        match strategy {
            CompilationStrategy::AheadOfTime(config) => {
                self.compile_aot(config)
            }

            CompilationStrategy::JustInTime(config) => {
                self.compile_jit(config)
            }

            CompilationStrategy::AdaptiveOptimization(config) => {
                self.compile_adaptive(config)
            }

            CompilationStrategy::MultiParadigmTranspilation(config) => {
                self.compile_transpile(config)
            }

            CompilationStrategy::HardwareSynthesis(config) => {
                self.compile_hdl(config)
            }

            CompilationStrategy::QuantumCompilation(config) => {
                self.compile_quantum(config)
            }

            CompilationStrategy::NanoCompilation(config) => {
                self.compile_nano(config)
            }

            CompilationStrategy::MixedMode(strategies) => {
                self.compile_mixed_mode(strategies, target)
            }
        }
    }

    fn backend_required(
        &mut self,
        strategy: &'static str,
    ) -> Result<CompiledBinary, String> {
        self.log_event(
            "backend_required",
            HashMap::from([(
                "strategy".to_string(),
                strategy.to_string(),
            )]),
        );

        Err(
            CompilationTechniqueError::BackendRequired {
                strategy: strategy.to_string(),
            }
            .to_string(),
        )
    }

    fn compile_aot(
        &mut self,
        config: AotConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate().map_err(|e| e.to_string())?;

        self.log_event(
            "aot_selected",
            HashMap::from([(
                "target".to_string(),
                format!("{:?}", config.target),
            )]),
        );

        self.backend_required("aot")
    }

    fn compile_jit(
        &mut self,
        config: JitConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate().map_err(|e| e.to_string())?;

        self.log_event("jit_selected", HashMap::new());

        self.backend_required("jit")
    }

    fn compile_adaptive(
        &mut self,
        config: AdaptiveOptConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate().map_err(|e| e.to_string())?;

        self.log_event(
            "adaptive_optimization_selected",
            HashMap::from([(
                "model".to_string(),
                config.strategy_model.model_id.0.clone(),
            )]),
        );

        self.backend_required("adaptive")
    }

    fn compile_transpile(
        &mut self,
        config: TranspilationConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate().map_err(|e| e.to_string())?;

        self.log_event(
            "transpilation_selected",
            HashMap::from([
                (
                    "source".to_string(),
                    config.source_paradigm.0.clone(),
                ),
                (
                    "target".to_string(),
                    config.target_paradigm.0.clone(),
                ),
            ]),
        );

        self.backend_required("transpilation")
    }

    fn compile_hdl(
        &mut self,
        config: HdlSynthConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate().map_err(|e| e.to_string())?;

        self.log_event(
            "hardware_synthesis_selected",
            HashMap::from([(
                "target".to_string(),
                config.target_chip_design.0.clone(),
            )]),
        );

        self.backend_required("hardware")
    }

    fn compile_quantum(
        &mut self,
        config: QuantumCompileConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate().map_err(|e| e.to_string())?;

        self.log_event(
            "quantum_compilation_selected",
            HashMap::from([(
                "qpu".to_string(),
                config.target_qpu_architecture.0.clone(),
            )]),
        );

        self.backend_required("quantum")
    }

    fn compile_nano(
        &mut self,
        config: NanoCompileConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate().map_err(|e| e.to_string())?;

        self.log_event(
            "nano_compilation_selected",
            HashMap::from([(
                "nacu".to_string(),
                config.target_nacu_version.0.clone(),
            )]),
        );

        self.backend_required("nano")
    }

    fn compile_mixed_mode(
        &mut self,
        strategies: Vec<CompilationStrategy>,
        _target: TargetPlatform,
    ) -> Result<CompiledBinary, String> {
        if strategies.is_empty() {
            return Err(
                CompilationTechniqueError::EmptyMixedMode.to_string()
            );
        }

        for strategy in &strategies {
            strategy.validate().map_err(|e| e.to_string())?;

            if matches!(
                strategy,
                CompilationStrategy::MixedMode(_)
            ) {
                return Err(
                    CompilationTechniqueError::NestedMixedMode
                        .to_string(),
                );
            }
        }

        self.log_event(
            "mixed_mode_selected",
            HashMap::from([(
                "strategy_count".to_string(),
                strategies.len().to_string(),
            )]),
        );

        self.backend_required("mixed")
    }
}

// -----------------------------------------------------------------------------
// Paradigm routing
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParadigmStrategy {
    Imperative,
    Functional,
    Quantum,
    Nano,
    Biological,
    Actor,
    Metaphysical,
}

#[derive(Debug, Clone)]
pub struct ParadigmRouter {
    handlers: HashMap<String, ParadigmStrategy>,
}

impl Default for ParadigmRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl ParadigmRouter {
    pub fn new() -> Self {
        let mut handlers = HashMap::new();

        handlers.insert(
            "imperative".to_string(),
            ParadigmStrategy::Imperative,
        );

        handlers.insert(
            "functional".to_string(),
            ParadigmStrategy::Functional,
        );

        handlers.insert(
            "quantum".to_string(),
            ParadigmStrategy::Quantum,
        );

        handlers.insert(
            "nano".to_string(),
            ParadigmStrategy::Nano,
        );

        handlers.insert(
            "biological".to_string(),
            ParadigmStrategy::Biological,
        );

        handlers.insert(
            "actor".to_string(),
            ParadigmStrategy::Actor,
        );

        handlers.insert(
            "metaphysical".to_string(),
            ParadigmStrategy::Metaphysical,
        );

        Self { handlers }
    }

    pub fn resolve(
        &self,
        paradigm: &str,
    ) -> Result<ParadigmStrategy, String> {
        let key = normalize_identifier(paradigm);

        self.handlers
            .get(&key)
            .cloned()
            .ok_or_else(|| {
                CompilationTechniqueError::UnknownParadigm {
                    paradigm: paradigm.to_string(),
                    available: self.available_paradigms(),
                }
                .to_string()
            })
    }

    pub fn register(
        &mut self,
        name: impl Into<String>,
        strategy: ParadigmStrategy,
    ) {
        let name = normalize_identifier(&name.into());

        if !name.is_empty() {
            self.handlers.insert(name, strategy);
        }
    }

    pub fn available_paradigms(&self) -> Vec<String> {
        let mut paradigms: Vec<String> =
            self.handlers.keys().cloned().collect();

        paradigms.sort();

        paradigms
    }

    pub fn count(&self) -> usize {
        self.handlers.len()
    }

    pub fn contains(&self, paradigm: &str) -> bool {
        self.handlers
            .contains_key(&normalize_identifier(paradigm))
    }
}

fn normalize_identifier(value: &str) -> String {
    value.trim().to_lowercase()
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn identifier(name: &str) -> Identifier {
        Identifier(
            name.to_string(),
            crate::source_map::Span::dummy(),
        )
    }

    #[test]
    fn target_alias_matches_repository_target_type() {
        let target: TargetPlatform =
            CompilationTarget::X86_64Linux;

        assert_eq!(
            target,
            CompilationTarget::X86_64Linux
        );
    }

    #[test]
    fn strategy_model_selects_aot_by_default() {
        let model =
            AiStrategyModel::new(identifier("test-model"));

        let strategy = model
            .predict_strategy(
                &CompilationTarget::X86_64Linux,
            )
            .expect("strategy selection should succeed");

        assert!(matches!(
            strategy,
            CompilationStrategy::AheadOfTime(_)
        ));
    }

    #[test]
    fn strategy_model_respects_valid_optimization_profile() {
        let mut model =
            AiStrategyModel::new(identifier("test-model"));

        model.performance_profile.insert(
            "optimization_level".to_string(),
            vec![3.0],
        );

        let strategy = model
            .predict_strategy(
                &CompilationTarget::X86_64Linux,
            )
            .expect("strategy selection should succeed");

        match strategy {
            CompilationStrategy::AheadOfTime(config) => {
                assert_eq!(
                    config.optimization_level,
                    OptimizationLevel::Full
                );
            }

            _ => panic!("expected AOT strategy"),
        }
    }

    #[test]
    fn invalid_jit_threshold_is_rejected() {
        let strategy =
            CompilationStrategy::JustInTime(JitConfig {
                enable_profiling: true,
                recompile_threshold: -1.0,
            });

        assert!(strategy.validate().is_err());
    }

    #[test]
    fn invalid_adaptive_learning_rate_is_rejected() {
        let strategy =
            CompilationStrategy::AdaptiveOptimization(
                AdaptiveOptConfig {
                    strategy_model: AiStrategyModel::new(
                        identifier("model"),
                    ),
                    learning_rate: 0.0,
                },
            );

        assert!(strategy.validate().is_err());
    }

    #[test]
    fn quantum_zero_qubits_are_rejected() {
        let strategy =
            CompilationStrategy::QuantumCompilation(
                QuantumCompileConfig {
                    qubit_count: 0,
                    error_correction_scheme: identifier("surface"),
                    target_qpu_architecture: identifier("qpu"),
                },
            );

        assert!(strategy.validate().is_err());
    }

    #[test]
    fn mixed_mode_rejects_empty_strategy_list() {
        let strategy =
            CompilationStrategy::MixedMode(Vec::new());

        assert!(strategy.validate().is_err());
    }

    #[test]
    fn nested_mixed_mode_is_rejected() {
        let strategy =
            CompilationStrategy::MixedMode(vec![
                CompilationStrategy::MixedMode(vec![
                    CompilationStrategy::AheadOfTime(
                        AotConfig {
                            optimization_level:
                                OptimizationLevel::Basic,
                            target:
                                CompilationTarget::X86_64Linux,
                        },
                    ),
                ]),
            ]);

        assert!(strategy.validate().is_err());
    }

    #[test]
    fn paradigm_router_resolves_known_paradigm() {
        let router = ParadigmRouter::new();

        assert_eq!(
            router.resolve("QUANTUM").unwrap(),
            ParadigmStrategy::Quantum
        );
    }

    #[test]
    fn paradigm_router_rejects_unknown_paradigm() {
        let router = ParadigmRouter::new();

        assert!(router
            .resolve("does_not_exist")
            .is_err());
    }

    #[test]
    fn paradigm_router_registration_is_normalized() {
        let mut router = ParadigmRouter::new();

        router.register(
            "  CustomParadigm  ",
            ParadigmStrategy::Actor,
        );

        assert!(router.contains("customparadigm"));
    }

    #[test]
    fn orchestrator_can_select_strategy() {
        let model =
            AiStrategyModel::new(identifier("test-model"));

        let mut orchestrator =
            CompilationOrchestrator::new(model);

        let strategy = orchestrator
            .select_strategy(
                &CompilationTarget::Wasm32,
            )
            .expect("strategy selection should succeed");

        assert!(matches!(
            strategy,
            CompilationStrategy::AheadOfTime(_)
        ));

        assert_eq!(
            orchestrator.event_count(),
            1
        );
    }

    #[test]
    fn events_have_deterministic_sequence_numbers() {
        let model =
            AiStrategyModel::new(identifier("test-model"));

        let mut orchestrator =
            CompilationOrchestrator::new(model);

        orchestrator.log_event(
            "first",
            HashMap::new(),
        );

        orchestrator.log_event(
            "second",
            HashMap::new(),
        );

        assert_eq!(
            orchestrator.compilation_log[0].timestamp,
            0
        );

        assert_eq!(
            orchestrator.compilation_log[1].timestamp,
            1
        );
    }

    #[test]
    fn explicit_strategy_is_validated() {
        let model =
            AiStrategyModel::new(identifier("test-model"));

        let mut orchestrator =
            CompilationOrchestrator::new(model);

        let result = orchestrator.set_strategy(
            CompilationStrategy::JustInTime(
                JitConfig {
                    enable_profiling: true,
                    recompile_threshold: -1.0,
                },
            ),
        );

        assert!(result.is_err());
    }

    #[test]
    fn compile_does_not_fabricate_empty_artifact() {
        let model =
            AiStrategyModel::new(identifier("test-model"));

        let mut orchestrator =
            CompilationOrchestrator::new(model);

        let program =
            Program::new(
                Vec::new(),
                crate::source_map::Span::dummy(),
            );

        let result = orchestrator.compile_program(
            program,
            CompilationTarget::X86_64Linux,
        );

        assert!(result.is_err());

        let message =
            result.expect_err("backend must be required");

        assert!(
            message.contains("requires its concrete backend")
        );
    }

    #[test]
    fn compiled_binary_rejects_empty_data() {
        let result =
            CompiledBinary::new(
                Vec::new(),
                "native",
            );

        assert!(result.is_err());
    }

    #[test]
    fn compiled_binary_accepts_real_data() {
        let binary =
            CompiledBinary::new(
                vec![1, 2, 3],
                "native",
            )
            .expect("non-empty artifact should succeed");

        assert_eq!(binary.len(), 3);
        assert!(!binary.is_empty());
        assert_eq!(binary.format, "native");
    }

    #[test]
    fn paradigm_list_is_sorted() {
        let router = ParadigmRouter::new();
        let paradigms = router.available_paradigms();

        let mut sorted = paradigms.clone();
        sorted.sort();

        assert_eq!(paradigms, sorted);
    }
}