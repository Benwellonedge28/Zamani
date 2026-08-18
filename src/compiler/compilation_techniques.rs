//! Zamani Universal Meta-Compiler (UMC): Compilation Techniques.
//!
//! This module is the strategy-selection and orchestration boundary for the
//! Zamani compiler.
//!
//! It deliberately does NOT duplicate:
//! - parsing;
//! - semantic analysis;
//! - ownership/borrow analysis;
//! - IR generation;
//! - optimization passes;
//! - IR verification;
//! - security inspection;
//! - target-specific code generation;
//! - native executable linking.
//!
//! Those responsibilities belong to the canonical compiler pipeline and
//! backend modules.
//!
//! The important production invariant is:
//!
//!     strategy selection != compilation
//!
//! A selected strategy must eventually be executed by the canonical compiler
//! pipeline. This module therefore never fabricates an artifact merely to
//! claim that compilation succeeded.
//!
//! The canonical production pipeline is:
//
//!     source
//!       -> lexer
//!       -> parser
//!       -> semantic analysis
//!       -> ownership/borrow analysis
//!       -> IR generation
//!       -> optimization
//!       -> IR verification
//!       -> security inspection
//!       -> backend
//!       -> artifact
//!
//! Strategy selection is an orchestration concern that sits above that
//! pipeline.

use std::collections::HashMap;
use std::fmt;

use crate::ast::{Identifier, Program};
use crate::compiler_types::{CompilationTarget, CompilerConfig, OptimizationLevel};

// -----------------------------------------------------------------------------
// Repository-compatible aliases
// -----------------------------------------------------------------------------

/// Historical compatibility alias.
///
/// `CompilationTarget` remains the canonical repository target type.
pub type TargetPlatform = CompilationTarget;

// -----------------------------------------------------------------------------
// Artifact
// -----------------------------------------------------------------------------

/// Artifact produced by the canonical compiler backend.
///
/// This type is intentionally small and independent from backend internals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledBinary {
    /// Generated artifact contents.
    pub data: Vec<u8>,

    /// Backend-defined stable format identifier.
    pub format: String,
}

impl CompiledBinary {
    /// Constructs an artifact while enforcing the fundamental production
    /// invariant that successful compilation cannot produce an empty artifact.
    pub fn new(
        data: Vec<u8>,
        format: impl Into<String>,
    ) -> Result<Self, CompilationTechniqueError> {
        let format = format.into();

        if format.trim().is_empty() {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "artifact format cannot be empty".to_string(),
                ),
            );
        }

        if data.is_empty() {
            return Err(
                CompilationTechniqueError::EmptyArtifact { format },
            );
        }

        Ok(Self { data, format })
    }

    /// Returns whether the artifact contains no data.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the artifact size in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Structured errors for the compilation-strategy layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilationTechniqueError {
    /// No strategy was selected.
    StrategyNotSelected,

    /// Configuration is invalid.
    InvalidConfiguration(String),

    /// Strategy/target combination is unsupported.
    UnsupportedTarget {
        strategy: String,
        target: String,
    },

    /// The selected strategy requires a backend that is not implemented by
    /// the current repository integration.
    BackendRequired {
        strategy: String,
    },

    /// Mixed mode has no child strategies.
    EmptyMixedMode,

    /// Mixed mode recursively contains another MixedMode.
    NestedMixedMode,

    /// A backend returned an empty artifact.
    EmptyArtifact {
        format: String,
    },

    /// A paradigm is not registered.
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
                write!(
                    formatter,
                    "no compilation strategy has been selected"
                )
            }

            Self::InvalidConfiguration(message) => {
                write!(
                    formatter,
                    "invalid compilation configuration: {message}"
                )
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
                    "strategy '{strategy}' requires a concrete backend integration"
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
// Lifecycle
// -----------------------------------------------------------------------------

/// Initializes the compilation-strategy subsystem.
///
/// The subsystem is deliberately stateless. This function remains part of the
/// public lifecycle API for compatibility with `compiler::initialize_compiler`.
pub fn init_compilation_techniques() {}

/// Shuts down the compilation-strategy subsystem.
///
/// No global resources are owned by this module.
pub fn shutdown_compilation_techniques() {}

// -----------------------------------------------------------------------------
// Compilation strategies
// -----------------------------------------------------------------------------

/// High-level strategy understood by the UMC.
#[derive(Debug, Clone, PartialEq)]
pub enum CompilationStrategy {
    /// Traditional whole-program compilation.
    AheadOfTime(AotConfig),

    /// Runtime-oriented compilation.
    JustInTime(JitConfig),

    /// Feedback-driven optimization.
    AdaptiveOptimization(AdaptiveOptConfig),

    /// Paradigm translation.
    MultiParadigmTranspilation(TranspilationConfig),

    /// Hardware synthesis.
    HardwareSynthesis(HdlSynthConfig),

    /// Quantum compilation.
    QuantumCompilation(QuantumCompileConfig),

    /// Nano/NACU compilation.
    NanoCompilation(NanoCompileConfig),

    /// Combination of independent strategies.
    MixedMode(Vec<CompilationStrategy>),
}

impl CompilationStrategy {
    /// Stable machine-readable strategy identifier.
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

    /// Validates this strategy and all of its configuration.
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

            Self::QuantumCompilation(config) => {
                config.validate()
            }

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

    /// Returns the target explicitly associated with the strategy when one
    /// exists.
    pub fn target(&self) -> Option<&TargetPlatform> {
        match self {
            Self::AheadOfTime(config) => Some(&config.target),
            Self::JustInTime(_)
            | Self::AdaptiveOptimization(_)
            | Self::MultiParadigmTranspilation(_)
            | Self::HardwareSynthesis(_)
            | Self::QuantumCompilation(_)
            | Self::NanoCompilation(_)
            | Self::MixedMode(_) => None,
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
                    "error-correction scheme cannot be empty"
                        .to_string(),
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

/// Deterministic strategy metadata.
///
/// This is not an ML inference engine. AI/ML systems can populate the
/// metadata, but strategy selection remains deterministic.
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

    /// Selects a deterministic AOT baseline.
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
                    3 => Some(OptimizationLevel::UltraAGI),
                    _ => None,
                }
            })
            .unwrap_or(OptimizationLevel::Basic);

        Ok(CompilationStrategy::AheadOfTime(
            AotConfig {
                optimization_level,
                target: target_env.clone(),
            },
        ))
    }
}

// -----------------------------------------------------------------------------
// Compilation events
// -----------------------------------------------------------------------------

/// Deterministic compiler event.
///
/// `timestamp` is retained for API compatibility but represents a monotonically
/// increasing event sequence, not wall-clock time.
#[derive(Debug, Clone, PartialEq)]
pub struct CompilationEvent {
    pub timestamp: u64,
    pub event_type: String,
    pub details: HashMap<String, String>,
}

// -----------------------------------------------------------------------------
// Orchestrator
// -----------------------------------------------------------------------------

/// High-level strategy orchestrator.
///
/// The orchestrator is deliberately separate from the canonical
/// `compiler::compile_source` pipeline.
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

    /// Creates an orchestrator using deterministic default strategy selection.
    pub fn default_for_target(target: &TargetPlatform) -> Self {
        let model_id = Identifier(
            "default_compilation_strategy".to_string(),
            crate::source_map::Span::dummy(),
        );

        let mut orchestrator =
            Self::new(AiStrategyModel::new(model_id));

        orchestrator.current_strategy =
            orchestrator.ai_model.predict_strategy(target).ok();

        orchestrator
    }

    /// Sets an explicit strategy after validation.
    pub fn set_strategy(
        &mut self,
        strategy: CompilationStrategy,
    ) -> Result<(), CompilationTechniqueError> {
        strategy.validate()?;
        self.current_strategy = Some(strategy);
        Ok(())
    }

    /// Returns the currently selected strategy.
    pub fn strategy(&self) -> Option<&CompilationStrategy> {
        self.current_strategy.as_ref()
    }

    /// Selects a strategy using the deterministic strategy model.
    pub fn select_strategy(
        &mut self,
        target: &TargetPlatform,
    ) -> Result<CompilationStrategy, String> {
        let strategy =
            self.ai_model.predict_strategy(target)?;

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
        let timestamp =
            self.compilation_log.len() as u64;

        self.compilation_log.push(
            CompilationEvent {
                timestamp,
                event_type: event_type.into(),
                details,
            },
        );
    }

    pub fn event_count(&self) -> usize {
        self.compilation_log.len()
    }

    /// Validates a strategy against the requested target.
    pub fn validate_for_target(
        strategy: &CompilationStrategy,
        target: &TargetPlatform,
    ) -> Result<(), CompilationTechniqueError> {
        strategy.validate()?;

        if let Some(strategy_target) = strategy.target() {
            if strategy_target != target {
                return Err(
                    CompilationTechniqueError::UnsupportedTarget {
                        strategy: strategy.name().to_string(),
                        target: format!("{target:?}"),
                    },
                );
            }
        }

        Ok(())
    }

    /// Strategy-level compilation entry point.
    ///
    /// IMPORTANT:
    ///
    /// This function does not create a second compiler pipeline. The canonical
    /// source compilation pipeline is owned by `crate::compiler`.
    ///
    /// For AOT, the caller should use `crate::compiler::compile_source` with
    /// the target/optimization configuration represented by the selected
    /// strategy.
    ///
    /// Strategies whose concrete backend integration is not yet exposed by
    /// the canonical compiler return an explicit error instead of returning
    /// fake bytes.
    pub fn compile_program(
        &mut self,
        program: Program,
        target: TargetPlatform,
    ) -> Result<CompiledBinary, String> {
        let strategy = match self.current_strategy.clone() {
            Some(strategy) => strategy,
            None => self.select_strategy(&target)?,
        };

        Self::validate_for_target(
            &strategy,
            &target,
        )
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
                self.compile_aot(program, config)
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
                self.compile_mixed_mode(
                    strategies,
                    target,
                )
            }
        }
    }

    /// Executes the AOT strategy through the canonical backend.
    ///
    /// `Program` is consumed here to preserve the historical API. The actual
    /// canonical source pipeline already owns parsing and IR generation, so
    /// callers that already have an AST should use the compiler's AST/IR
    /// integration point when one is exposed.
    fn compile_aot(
        &mut self,
        _program: Program,
        config: AotConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate()
            .map_err(|error| error.to_string())?;

        self.log_event(
            "aot_selected",
            HashMap::from([
                (
                    "target".to_string(),
                    format!("{:?}", config.target),
                ),
                (
                    "optimization".to_string(),
                    format!("{:?}", config.optimization_level),
                ),
            ]),
        );

        // The canonical compiler pipeline owns Program -> IR lowering.
        //
        // We deliberately do not duplicate that pipeline here. Returning an
        // error is safer than fabricating output from an AST that cannot yet
        // be handed to the canonical pipeline through a stable public API.
        Err(
            CompilationTechniqueError::BackendRequired {
                strategy: "aot".to_string(),
            }
            .to_string(),
        )
    }

    fn compile_jit(
        &mut self,
        config: JitConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate()
            .map_err(|error| error.to_string())?;

        self.log_event(
            "jit_selected",
            HashMap::from([
                (
                    "profiling".to_string(),
                    config.enable_profiling.to_string(),
                ),
                (
                    "threshold".to_string(),
                    config.recompile_threshold.to_string(),
                ),
            ]),
        );

        Err(
            CompilationTechniqueError::BackendRequired {
                strategy: "jit".to_string(),
            }
            .to_string(),
        )
    }

    fn compile_adaptive(
        &mut self,
        config: AdaptiveOptConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate()
            .map_err(|error| error.to_string())?;

        self.log_event(
            "adaptive_selected",
            HashMap::from([(
                "model".to_string(),
                config.strategy_model.model_id.0.clone(),
            )]),
        );

        Err(
            CompilationTechniqueError::BackendRequired {
                strategy: "adaptive".to_string(),
            }
            .to_string(),
        )
    }

    fn compile_transpile(
        &mut self,
        config: TranspilationConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate()
            .map_err(|error| error.to_string())?;

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

        Err(
            CompilationTechniqueError::BackendRequired {
                strategy: "transpilation".to_string(),
            }
            .to_string(),
        )
    }

    fn compile_hdl(
        &mut self,
        config: HdlSynthConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate()
            .map_err(|error| error.to_string())?;

        self.log_event(
            "hardware_selected",
            HashMap::from([(
                "chip".to_string(),
                config.target_chip_design.0.clone(),
            )]),
        );

        Err(
            CompilationTechniqueError::BackendRequired {
                strategy: "hardware".to_string(),
            }
            .to_string(),
        )
    }

    fn compile_quantum(
        &mut self,
        config: QuantumCompileConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate()
            .map_err(|error| error.to_string())?;

        self.log_event(
            "quantum_selected",
            HashMap::from([
                (
                    "qubits".to_string(),
                    config.qubit_count.to_string(),
                ),
                (
                    "qpu".to_string(),
                    config.target_qpu_architecture.0.clone(),
                ),
            ]),
        );

        Err(
            CompilationTechniqueError::BackendRequired {
                strategy: "quantum".to_string(),
            }
            .to_string(),
        )
    }

    fn compile_nano(
        &mut self,
        config: NanoCompileConfig,
    ) -> Result<CompiledBinary, String> {
        config.validate()
            .map_err(|error| error.to_string())?;

        self.log_event(
            "nano_selected",
            HashMap::from([
                (
                    "agents".to_string(),
                    config.agent_swarm_size.to_string(),
                ),
                (
                    "nacu".to_string(),
                    config.target_nacu_version.0.clone(),
                ),
            ]),
        );

        Err(
            CompilationTechniqueError::BackendRequired {
                strategy: "nano".to_string(),
            }
            .to_string(),
        )
    }

    fn compile_mixed_mode(
        &mut self,
        strategies: Vec<CompilationStrategy>,
        target: TargetPlatform,
    ) -> Result<CompiledBinary, String> {
        let strategy =
            CompilationStrategy::MixedMode(strategies);

        strategy
            .validate()
            .map_err(|error| error.to_string())?;

        Self::validate_for_target(
            &strategy,
            &target,
        )
        .map_err(|error| error.to_string())?;

        self.log_event(
            "mixed_mode_selected",
            HashMap::from([(
                "strategy_count".to_string(),
                match &strategy {
                    CompilationStrategy::MixedMode(items) => {
                        items.len().to_string()
                    }
                    _ => unreachable!(),
                },
            )]),
        );

        Err(
            CompilationTechniqueError::BackendRequired {
                strategy: "mixed".to_string(),
            }
            .to_string(),
        )
    }
}

// -----------------------------------------------------------------------------
// Compiler-config bridge
// -----------------------------------------------------------------------------

/// Converts an AOT strategy into the repository's canonical compiler
/// configuration.
///
/// This is the preferred bridge between strategy selection and
/// `crate::compiler::compile_source`.
pub fn compiler_config_for_aot(
    config: &AotConfig,
) -> Result<CompilerConfig, CompilationTechniqueError> {
    if matches!(
        config.optimization_level,
        OptimizationLevel::UltraAGI
    ) {
        return Err(
            CompilationTechniqueError::InvalidConfiguration(
                "UltraAGI optimization is experimental and cannot be selected by the production AOT strategy"
                    .to_string(),
            ),
        );
    }

    Ok(CompilerConfig {
        target: config.target.clone(),
        opt_level: config.optimization_level,
        debug_info: false,
        verify: true,
        emit_ir: false,
        parallel: false,
    })
}

// -----------------------------------------------------------------------------
// Paradigm routing
// -----------------------------------------------------------------------------

/// High-level strategy family associated with a Zamani paradigm.
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

/// Routes paradigm names to strategy families.
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

    /// Resolves a paradigm name case-insensitively.
    pub fn resolve(
        &self,
        paradigm: &str,
    ) -> Result<ParadigmStrategy, CompilationTechniqueError> {
        let key = normalize_identifier(paradigm);

        self.handlers
            .get(&key)
            .cloned()
            .ok_or_else(|| {
                let mut available =
                    self.available_paradigms();

                available.sort();

                CompilationTechniqueError::UnknownParadigm {
                    paradigm: paradigm.to_string(),
                    available,
                }
            })
    }

    /// Registers a custom paradigm.
    pub fn register(
        &mut self,
        name: impl Into<String>,
        strategy: ParadigmStrategy,
    ) -> Result<(), CompilationTechniqueError> {
        let name = normalize_identifier(&name.into());

        if name.is_empty() {
            return Err(
                CompilationTechniqueError::InvalidConfiguration(
                    "paradigm name cannot be empty".to_string(),
                ),
            );
        }

        self.handlers.insert(name, strategy);

        Ok(())
    }

    /// Returns all registered paradigms in deterministic order.
    pub fn available_paradigms(&self) -> Vec<String> {
        let mut paradigms =
            self.handlers.keys().cloned().collect::<Vec<_>>();

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
    value.trim().to_ascii_lowercase()
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
    fn target_alias_matches_repository_type() {
        let target: TargetPlatform =
            CompilationTarget::X86_64Linux;

        assert_eq!(
            target,
            CompilationTarget::X86_64Linux
        );
    }

    #[test]
    fn artifact_rejects_empty_data() {
        let result =
            CompiledBinary::new(Vec::new(), "assembly");

        assert!(matches!(
            result,
            Err(
                CompilationTechniqueError::EmptyArtifact { .. }
            )
        ));
    }

    #[test]
    fn artifact_rejects_empty_format() {
        let result =
            CompiledBinary::new(vec![1], " ");

        assert!(matches!(
            result,
            Err(
                CompilationTechniqueError::InvalidConfiguration(_)
            )
        ));
    }

    #[test]
    fn artifact_accepts_real_data() {
        let artifact =
            CompiledBinary::new(vec![1, 2, 3], "test")
                .expect("artifact should be valid");

        assert_eq!(artifact.len(), 3);
        assert!(!artifact.is_empty());
    }

    #[test]
    fn strategy_model_selects_aot() {
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
    fn strategy_validation_rejects_empty_mixed_mode() {
        let strategy =
            CompilationStrategy::MixedMode(Vec::new());

        assert!(matches!(
            strategy.validate(),
            Err(
                CompilationTechniqueError::EmptyMixedMode
            )
        ));
    }

    #[test]
    fn strategy_validation_rejects_nested_mixed_mode() {
        let strategy =
            CompilationStrategy::MixedMode(vec![
                CompilationStrategy::MixedMode(Vec::new()),
            ]);

        assert!(matches!(
            strategy.validate(),
            Err(
                CompilationTechniqueError::NestedMixedMode
            )
        ));
    }

    #[test]
    fn jit_rejects_nan_threshold() {
        let strategy =
            CompilationStrategy::JustInTime(
                JitConfig {
                    enable_profiling: true,
                    recompile_threshold: f32::NAN,
                },
            );

        assert!(strategy.validate().is_err());
    }

    #[test]
    fn quantum_rejects_zero_qubits() {
        let strategy =
            CompilationStrategy::QuantumCompilation(
                QuantumCompileConfig {
                    qubit_count: 0,
                    error_correction_scheme:
                        identifier("surface"),
                    target_qpu_architecture:
                        identifier("generic"),
                },
            );

        assert!(strategy.validate().is_err());
    }

    #[test]
    fn nano_rejects_zero_agents() {
        let strategy =
            CompilationStrategy::NanoCompilation(
                NanoCompileConfig {
                    agent_swarm_size: 0,
                    target_nacu_version:
                        identifier("1.0"),
                    bio_compatibility_mode: false,
                },
            );

        assert!(strategy.validate().is_err());
    }

    #[test]
    fn paradigm_router_is_case_insensitive() {
        let router = ParadigmRouter::new();

        assert_eq!(
            router.resolve("QuAnTuM").unwrap(),
            ParadigmStrategy::Quantum
        );
    }

    #[test]
    fn paradigm_router_rejects_unknown() {
        let router = ParadigmRouter::new();

        assert!(matches!(
            router.resolve("does-not-exist"),
            Err(
                CompilationTechniqueError::UnknownParadigm { .. }
            )
        ));
    }

    #[test]
    fn paradigm_router_registration_is_validated() {
        let mut router =
            ParadigmRouter::new();

        assert!(router
            .register("  ", ParadigmStrategy::Actor)
            .is_err());

        assert!(router
            .register("custom", ParadigmStrategy::Actor)
            .is_ok());

        assert!(router.contains("CUSTOM"));
    }

    #[test]
    fn events_are_deterministic() {
        let model =
            AiStrategyModel::new(identifier("test"));

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
    fn compiler_config_bridge_enables_verification() {
        let config =
            AotConfig {
                optimization_level:
                    OptimizationLevel::Basic,
                target:
                    CompilationTarget::X86_64Linux,
            };

        let compiler_config =
            compiler_config_for_aot(&config)
                .expect("configuration should be valid");

        assert!(compiler_config.verify);

        assert_eq!(
            compiler_config.target,
            CompilationTarget::X86_64Linux
        );
    }

    #[test]
    fn compiler_config_bridge_rejects_experimental_optimization() {
        let config =
            AotConfig {
                optimization_level:
                    OptimizationLevel::UltraAGI,
                target:
                    CompilationTarget::X86_64Linux,
            };

        assert!(
            compiler_config_for_aot(&config).is_err()
        );
    }

    #[test]
    fn explicit_strategy_is_validated() {
        let model =
            AiStrategyModel::new(identifier("test"));

        let mut orchestrator =
            CompilationOrchestrator::new(model);

        let strategy =
            CompilationStrategy::JustInTime(
                JitConfig {
                    enable_profiling: true,
                    recompile_threshold: 10.0,
                },
            );

        assert!(
            orchestrator
                .set_strategy(strategy)
                .is_ok()
        );
    }
}