//! Zamani Universal Meta-Compiler (UMC): Compilation Techniques
//!
//! This module defines the high-level compilation strategies understood by
//! the Zamani compiler.  Concrete IR generation and backend code generation
//! belong to the repository-wide `ir_gen` and `backend` modules and are not
//! duplicated here.
//!
//! The purpose of this module is orchestration and strategy selection.

#![allow(dead_code)]

use std::collections::HashMap;

use crate::ast::{Identifier, Program};
use crate::compiler_types::{CompilationTarget, OptimizationLevel};

// -----------------------------------------------------------------------------
// Repository-compatible target aliases
// -----------------------------------------------------------------------------

/// Compatibility name for older callers of the compilation-techniques API.
///
/// The canonical target type is `crate::compiler_types::CompilationTarget`.
pub type TargetPlatform = CompilationTarget;

/// Result produced by a strategy-level compilation request.
///
/// Actual target-specific generation is performed by `crate::backend`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledBinary {
    pub data: Vec<u8>,
    pub format: String,
}

// -----------------------------------------------------------------------------
// Module lifecycle
// -----------------------------------------------------------------------------

/// Initializes the compilation-techniques subsystem.
pub fn init_compilation_techniques() {
    println!(
        "  - Initializing Zamani Compilation Techniques \
         (Hybrid, Adaptive, Multi-Paradigm)..."
    );
}

/// Shuts down the compilation-techniques subsystem.
pub fn shutdown_compilation_techniques() {
    println!("  - Shutting down Zamani Compilation Techniques...");
}

// -----------------------------------------------------------------------------
// Core compilation strategies
// -----------------------------------------------------------------------------

/// Primary compilation strategies supported by the UMC.
///
/// These are strategy descriptions.  They do not duplicate IR generators or
/// backend implementations.
#[derive(Debug, Clone, PartialEq)]
pub enum CompilationStrategy {
    /// Traditional whole-program compilation.
    AheadOfTime(AotConfig),

    /// Runtime-oriented compilation strategy.
    JustInTime(JitConfig),

    /// Feedback-driven compilation strategy.
    AdaptiveOptimization(AdaptiveOptConfig),

    /// Translation between programming paradigms.
    MultiParadigmTranspilation(TranspilationConfig),

    /// Hardware-description synthesis.
    HardwareSynthesis(HdlSynthConfig),

    /// Quantum compilation.
    QuantumCompilation(QuantumCompileConfig),

    /// Nano/NACU-oriented compilation.
    NanoCompilation(NanoCompileConfig),

    /// Combination of multiple compilation strategies.
    MixedMode(Vec<CompilationStrategy>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AotConfig {
    pub optimization_level: OptimizationLevel,
    pub target: TargetPlatform,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JitConfig {
    pub enable_profiling: bool,
    pub recompile_threshold: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdaptiveOptConfig {
    pub strategy_model: AiStrategyModel,
    pub learning_rate: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TranspilationConfig {
    pub source_paradigm: Identifier,
    pub target_paradigm: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HdlSynthConfig {
    pub target_chip_design: Identifier,
    pub clock_speed_mhz: f32,
    pub power_budget_mw: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCompileConfig {
    pub qubit_count: u32,
    pub error_correction_scheme: Identifier,
    pub target_qpu_architecture: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NanoCompileConfig {
    pub agent_swarm_size: u32,
    pub target_nacu_version: Identifier,
    pub bio_compatibility_mode: bool,
}

// -----------------------------------------------------------------------------
// AI-driven strategy model
// -----------------------------------------------------------------------------

/// Metadata used by adaptive strategy selection.
///
/// The model is intentionally strategy-level.  Actual ML inference belongs
/// to the appropriate AI/ML subsystem rather than this compiler module.
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

    /// Select a baseline strategy for a compilation target.
    ///
    /// This is deliberately deterministic.  A future AI planner can replace
    /// this method without changing the compiler's strategy API.
    pub fn predict_strategy(
        &self,
        target_env: &TargetPlatform,
    ) -> Result<CompilationStrategy, String> {
        Ok(CompilationStrategy::AheadOfTime(AotConfig {
            optimization_level: OptimizationLevel::Basic,
            target: target_env.clone(),
        }))
    }
}

// -----------------------------------------------------------------------------
// Compilation events
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationEvent {
    pub timestamp: u64,
    pub event_type: String,
    pub details: HashMap<String, String>,
}

// -----------------------------------------------------------------------------
// Compilation orchestrator
// -----------------------------------------------------------------------------

/// High-level compilation strategy orchestrator.
///
/// This component intentionally does not duplicate the compiler frontend,
/// IR generator, optimizer, or backend.
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

    /// Creates an orchestrator using the default strategy model.
    pub fn default_for_target(target: &TargetPlatform) -> Self {
        let model_id = Identifier(
            "default_compilation_strategy".to_string(),
            crate::source_map::Span::dummy(),
        );

        let mut orchestrator = Self::new(AiStrategyModel::new(model_id));

        orchestrator.current_strategy = Some(
            orchestrator
                .ai_model
                .predict_strategy(target)
                .unwrap_or_else(|_| {
                    CompilationStrategy::AheadOfTime(AotConfig {
                        optimization_level: OptimizationLevel::Basic,
                        target: target.clone(),
                    })
                }),
        );

        orchestrator
    }

    /// Sets an explicit compilation strategy.
    pub fn set_strategy(&mut self, strategy: CompilationStrategy) {
        self.current_strategy = Some(strategy);
    }

    /// Returns the currently selected strategy.
    pub fn strategy(&self) -> Option<&CompilationStrategy> {
        self.current_strategy.as_ref()
    }

    /// Selects a strategy using the AI strategy model.
    pub fn select_strategy(
        &mut self,
        target: &TargetPlatform,
    ) -> Result<CompilationStrategy, String> {
        let strategy = self.ai_model.predict_strategy(target)?;
        self.current_strategy = Some(strategy.clone());

        self.log_event("strategy_selected", HashMap::new());

        Ok(strategy)
    }

    /// Records a compiler event.
    pub fn log_event(&mut self, event_type: impl Into<String>, details: HashMap<String, String>) {
        self.compilation_log.push(CompilationEvent {
            timestamp: 0,
            event_type: event_type.into(),
            details,
        });
    }

    /// Returns the number of recorded compilation events.
    pub fn event_count(&self) -> usize {
        self.compilation_log.len()
    }

    /// Strategy-level compilation entry point.
    ///
    /// The actual Zamani compiler pipeline lives in `crate::compiler::compile`
    /// and performs:
    ///
    ///     source → lexer → parser → semantic analysis → IR → backend
    ///
    /// This method therefore performs strategy selection/validation rather
    /// than creating a second compiler pipeline.
    pub fn compile_program(
        &mut self,
        _program: Program,
        target: TargetPlatform,
    ) -> Result<CompiledBinary, String> {
        let strategy = match self.current_strategy.clone() {
            Some(strategy) => strategy,
            None => self.select_strategy(&target)?,
        };

        self.log_event("compilation_requested", HashMap::new());

        match strategy {
            CompilationStrategy::AheadOfTime(config) => self.compile_aot(config),
            CompilationStrategy::JustInTime(config) => self.compile_jit(config),
            CompilationStrategy::AdaptiveOptimization(config) => {
                self.compile_adaptive(config)
            }
            CompilationStrategy::MultiParadigmTranspilation(config) => {
                self.compile_transpile(config)
            }
            CompilationStrategy::HardwareSynthesis(config) => self.compile_hdl(config),
            CompilationStrategy::QuantumCompilation(config) => {
                self.compile_quantum(config)
            }
            CompilationStrategy::NanoCompilation(config) => self.compile_nano(config),
            CompilationStrategy::MixedMode(strategies) => {
                self.compile_mixed_mode(strategies, target)
            }
        }
    }

    // -------------------------------------------------------------------------
    // Strategy handlers
    // -------------------------------------------------------------------------

    fn compile_aot(&mut self, config: AotConfig) -> Result<CompiledBinary, String> {
        self.log_event("aot_selected", HashMap::new());

        Ok(CompiledBinary {
            data: Vec::new(),
            format: format!(
                "aot:{:?}:{:?}",
                config.target, config.optimization_level
            ),
        })
    }

    fn compile_jit(&mut self, config: JitConfig) -> Result<CompiledBinary, String> {
        self.log_event("jit_selected", HashMap::new());

        Ok(CompiledBinary {
            data: Vec::new(),
            format: format!(
                "jit:profiling={}:threshold={}",
                config.enable_profiling, config.recompile_threshold
            ),
        })
    }

    fn compile_adaptive(
        &mut self,
        config: AdaptiveOptConfig,
    ) -> Result<CompiledBinary, String> {
        self.log_event("adaptive_optimization_selected", HashMap::new());

        Ok(CompiledBinary {
            data: Vec::new(),
            format: format!(
                "adaptive:model={}:learning_rate={}",
                config.strategy_model.model_id, config.learning_rate
            ),
        })
    }

    fn compile_transpile(
        &mut self,
        config: TranspilationConfig,
    ) -> Result<CompiledBinary, String> {
        self.log_event("transpilation_selected", HashMap::new());

        Ok(CompiledBinary {
            data: Vec::new(),
            format: format!(
                "transpile:{}->{}",
                config.source_paradigm, config.target_paradigm
            ),
        })
    }

    fn compile_hdl(&mut self, config: HdlSynthConfig) -> Result<CompiledBinary, String> {
        self.log_event("hardware_synthesis_selected", HashMap::new());

        Ok(CompiledBinary {
            data: Vec::new(),
            format: format!(
                "hdl:{}:{}MHz:{}mW",
                config.target_chip_design,
                config.clock_speed_mhz,
                config.power_budget_mw
            ),
        })
    }

    fn compile_quantum(
        &mut self,
        config: QuantumCompileConfig,
    ) -> Result<CompiledBinary, String> {
        self.log_event("quantum_compilation_selected", HashMap::new());

        Ok(CompiledBinary {
            data: Vec::new(),
            format: format!(
                "qasm:{}qubits:{}:{}",
                config.qubit_count,
                config.error_correction_scheme,
                config.target_qpu_architecture
            ),
        })
    }

    fn compile_nano(&mut self, config: NanoCompileConfig) -> Result<CompiledBinary, String> {
        self.log_event("nano_compilation_selected", HashMap::new());

        Ok(CompiledBinary {
            data: Vec::new(),
            format: format!(
                "nano:{}agents:{}:bio={}",
                config.agent_swarm_size,
                config.target_nacu_version,
                config.bio_compatibility_mode
            ),
        })
    }

    fn compile_mixed_mode(
        &mut self,
        strategies: Vec<CompilationStrategy>,
        target: TargetPlatform,
    ) -> Result<CompiledBinary, String> {
        if strategies.is_empty() {
            return Err("MixedMode requires at least one compilation strategy".to_string());
        }

        self.log_event("mixed_mode_selected", HashMap::new());

        Ok(CompiledBinary {
            data: Vec::new(),
            format: format!(
                "mixed:{}strategies:target={:?}",
                strategies.len(),
                target
            ),
        })
    }
}

// -----------------------------------------------------------------------------
// Paradigm routing
// -----------------------------------------------------------------------------

/// High-level compilation strategy associated with a Zamani paradigm.
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

/// Routes paradigm names to compilation strategies.
///
/// This router deliberately accepts a string rather than depending on an
/// optional AST type that is not part of the repository's canonical AST API.
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

        handlers.insert("imperative".to_string(), ParadigmStrategy::Imperative);
        handlers.insert("functional".to_string(), ParadigmStrategy::Functional);
        handlers.insert("quantum".to_string(), ParadigmStrategy::Quantum);
        handlers.insert("nano".to_string(), ParadigmStrategy::Nano);
        handlers.insert("biological".to_string(), ParadigmStrategy::Biological);
        handlers.insert("actor".to_string(), ParadigmStrategy::Actor);
        handlers.insert(
            "metaphysical".to_string(),
            ParadigmStrategy::Metaphysical,
        );

        Self { handlers }
    }

    /// Resolves a paradigm name.
    pub fn resolve(&self, paradigm: &str) -> Result<ParadigmStrategy, String> {
        let key = paradigm.trim().to_lowercase();

        self.handlers.get(&key).cloned().ok_or_else(|| {
            format!(
                "Unknown paradigm '{}'. Available paradigms: {}",
                paradigm,
                self.available_paradigms().join(", ")
            )
        })
    }

    /// Registers a custom paradigm.
    pub fn register(&mut self, name: impl Into<String>, strategy: ParadigmStrategy) {
        self.handlers.insert(name.into().to_lowercase(), strategy);
    }

    /// Returns all registered paradigm names.
    pub fn available_paradigms(&self) -> Vec<String> {
        let mut paradigms: Vec<String> = self.handlers.keys().cloned().collect();
        paradigms.sort();
        paradigms
    }

    /// Returns the number of registered paradigms.
    pub fn count(&self) -> usize {
        self.handlers.len()
    }

    /// Returns whether a paradigm is registered.
    pub fn contains(&self, paradigm: &str) -> bool {
        self.handlers.contains_key(&paradigm.trim().to_lowercase())
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn identifier(name: &str) -> Identifier {
        Identifier(name.to_string(), crate::source_map::Span::dummy())
    }

    #[test]
    fn target_alias_matches_repository_target_type() {
        let target: TargetPlatform = CompilationTarget::X86_64Linux;

        assert_eq!(target, CompilationTarget::X86_64Linux);
    }

    #[test]
    fn strategy_model_selects_aot_by_default() {
        let model = AiStrategyModel::new(identifier("test-model"));

        let strategy = model
            .predict_strategy(&CompilationTarget::X86_64Linux)
            .expect("strategy selection should succeed");

        assert!(matches!(
            strategy,
            CompilationStrategy::AheadOfTime(_)
        ));
    }

    #[test]
    fn paradigm_router_resolves_known_paradigm() {
        let router = ParadigmRouter::new();

        assert_eq!(
            router.resolve("quantum").unwrap(),
            ParadigmStrategy::Quantum
        );
    }

    #[test]
    fn paradigm_router_rejects_unknown_paradigm() {
        let router = ParadigmRouter::new();

        assert!(router.resolve("does_not_exist").is_err());
    }

    #[test]
    fn orchestrator_can_select_strategy() {
        let model = AiStrategyModel::new(identifier("test-model"));
        let mut orchestrator = CompilationOrchestrator::new(model);

        let strategy = orchestrator
            .select_strategy(&CompilationTarget::Wasm32)
            .expect("strategy selection should succeed");

        assert!(matches!(
            strategy,
            CompilationStrategy::AheadOfTime(_)
        ));
        assert_eq!(orchestrator.event_count(), 1);
    }

    #[test]
    fn mixed_mode_rejects_empty_strategy_list() {
        let model = AiStrategyModel::new(identifier("test-model"));
        let mut orchestrator = CompilationOrchestrator::new(model);

        orchestrator.set_strategy(CompilationStrategy::MixedMode(Vec::new()));

        let program = Program::new(Vec::new(), crate::source_map::Span::dummy());

        let result = orchestrator.compile_program(
            program,
            CompilationTarget::X86_64Linux,
        );

        assert!(result.is_err());
    }
}