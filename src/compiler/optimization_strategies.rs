//! Zamani Universal Meta-Compiler (UMC): Optimization Strategies
//!
//! This module is the strategy/orchestration layer for compiler optimization.
//!
//! IMPORTANT:
//! - The canonical IR optimizer lives in `crate::optimizer`.
//! - This module must not duplicate optimizer implementations.
//! - AI/planner components may recommend strategies, but they must not bypass
//!   deterministic compiler safety checks.
//! - Every optimization request is explicit, deterministic, and auditable.
//!
//! Pipeline:
//!
//!     IR
//!      │
//!      ▼
//!  OptimizationContext
//!      │
//!      ├── strategy selection
//!      ├── policy validation
//!      └── canonical optimizer
//!             │
//!             ▼
//!        optimized IR
//!
//! The strategy layer therefore decides *what optimization policy to use*;
//! `crate::optimizer` performs the actual IR transformations.

use std::collections::HashMap;
use std::fmt;

use crate::ast::{Identifier, Program};
use crate::ir_gen::IrModule;
use crate::optimizer::{OptimizationConfig, Optimizer};
use crate::source_map::Span;
use crate::stdlib::collections::{List, Map};
use crate::stdlib::meta_ops::MetaValue;

// -----------------------------------------------------------------------------
// Compatibility aliases
// -----------------------------------------------------------------------------

/// Canonical compiler optimization level.
///
/// Kept local to this module so callers can describe strategy policy without
/// depending on the implementation details of the optimizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrategyOptimizationLevel {
    None,
    Basic,
    Standard,
    Aggressive,
    Size,
}

impl Default for StrategyOptimizationLevel {
    fn default() -> Self {
        Self::Standard
    }
}

impl StrategyOptimizationLevel {
    /// Converts the strategy level into the canonical optimizer configuration.
    pub fn optimizer_config(self) -> OptimizationConfig {
        match self {
            Self::None => OptimizationConfig::none(),
            Self::Basic => OptimizationConfig::level(1),
            Self::Standard => OptimizationConfig::level(2),
            Self::Aggressive => OptimizationConfig::level(3),

            // The canonical optimizer currently has no dedicated size-only
            // level. Use its conservative/basic configuration rather than
            // pretending that a size optimizer exists.
            Self::Size => OptimizationConfig::level(1),
        }
    }
}

// -----------------------------------------------------------------------------
// Module lifecycle
// -----------------------------------------------------------------------------

/// Initializes the optimization-strategy subsystem.
pub fn init_optimization_strategies() {
    println!(
        "  - Initializing Zamani Optimization Strategies \
         (Deterministic, Adaptive, Secure)..."
    );
}

/// Shuts down the optimization-strategy subsystem.
pub fn shutdown_optimization_strategies() {
    println!("  - Shutting down Zamani Optimization Strategies...");
}

// -----------------------------------------------------------------------------
// Optimization categories
// -----------------------------------------------------------------------------

/// Broad category of an optimization strategy.
///
/// Categories describe *where* or *why* an optimization is performed. They do
/// not themselves implement transformations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationCategory {
    FrontEndHighLevel,
    IntermediateRepresentation,
    BackEndTargetSpecific,
    InterProcedural,
    RuntimeAdaptive,
    ArchitectureSpecific,
    AlgorithmicDesign,
    ZamaniMetaOptimization,
}

// -----------------------------------------------------------------------------
// Optimization pass metadata
// -----------------------------------------------------------------------------

/// Metadata describing an optimization pass.
///
/// This structure is intentionally descriptive. Actual transformation logic
/// belongs to `crate::optimizer` or a future dedicated optimization backend.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationPass {
    pub id: Identifier,
    pub category: OptimizationCategory,
    pub description: String,
    pub applicability_heuristics: List<OptimizationFact>,
    pub expected_impact: Map<String, f32>,
    pub security_risk_assessment: f32,
}

impl OptimizationPass {
    /// Creates a pass descriptor.
    pub fn new(
        id: Identifier,
        category: OptimizationCategory,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            category,
            description: description.into(),
            applicability_heuristics: List::new(),
            expected_impact: Map::new(),
            security_risk_assessment: 0.0,
        }
    }

    /// Returns whether the declared risk is valid.
    pub fn has_valid_risk_score(&self) -> bool {
        self.security_risk_assessment.is_finite()
            && (0.0..=1.0).contains(&self.security_risk_assessment)
    }
}

/// Lightweight strategy fact.
///
/// The previous implementation depended on the planner's internal `Fact`
/// representation. Keeping strategy metadata independent makes this module
/// less coupled to the AI subsystem.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationFact {
    pub name: String,
    pub value: String,
}

impl OptimizationFact {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

// -----------------------------------------------------------------------------
// AI strategy model
// -----------------------------------------------------------------------------

/// Strategy-selection model.
///
/// This model is deliberately deterministic. A future ML implementation can
/// replace the decision logic while preserving the same safety boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationAIModel {
    pub model_id: Identifier,
    pub enabled: bool,
}

impl OptimizationAIModel {
    pub fn new(model_id: Identifier) -> Self {
        Self {
            model_id,
            enabled: false,
        }
    }

    /// Enables model-assisted strategy selection.
    ///
    /// Enabling this does not permit arbitrary transformations. The canonical
    /// optimizer remains the authority for actual IR rewriting.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables model-assisted strategy selection.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Returns whether the model is active.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Selects a deterministic baseline optimization level.
    pub fn predict_level(
        &self,
        context: &OptimizationContext,
    ) -> Result<StrategyOptimizationLevel, OptimizationStrategyError> {
        if context.goal.0.trim().is_empty() {
            return Err(OptimizationStrategyError::InvalidContext(
                "optimization goal cannot be empty".to_string(),
            ));
        }

        // AI selection is intentionally conservative until a validated model
        // is supplied by the repository's ML infrastructure.
        if self.enabled {
            Ok(match context.goal.0.to_ascii_lowercase().as_str() {
                "minimize_binary_size" | "reduce_binary_size" | "minimize_size" => {
                    StrategyOptimizationLevel::Size
                }
                "maximize_performance"
                | "performance"
                | "throughput"
                | "maximize_throughput" => {
                    StrategyOptimizationLevel::Aggressive
                }
                "none" | "disable" | "no_optimization" => {
                    StrategyOptimizationLevel::None
                }
                _ => StrategyOptimizationLevel::Standard,
            })
        } else {
            Ok(context.optimization_level)
        }
    }
}

// -----------------------------------------------------------------------------
// Optimization context
// -----------------------------------------------------------------------------

/// Context supplied to the strategy orchestrator.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationContext {
    /// Optimization objective.
    pub goal: Identifier,

    /// Hard constraints.
    pub constraints: Map<String, MetaValue>,

    /// Characteristics discovered during IR analysis.
    pub ir_characteristics: Map<String, MetaValue>,

    /// Features supplied by the selected target.
    pub target_platform_features: Map<String, MetaValue>,

    /// Requested optimization level.
    pub optimization_level: StrategyOptimizationLevel,

    /// Maximum acceptable security-risk score for strategy metadata.
    pub max_security_risk: f32,
}

impl Default for OptimizationContext {
    fn default() -> Self {
        Self {
            goal: Identifier(
                "standard_optimization".to_string(),
                Span::dummy(),
            ),
            constraints: Map::new(),
            ir_characteristics: Map::new(),
            target_platform_features: Map::new(),
            optimization_level: StrategyOptimizationLevel::Standard,
            max_security_risk: 0.0,
        }
    }
}

impl OptimizationContext {
    /// Creates a context for a named goal.
    pub fn for_goal(goal: Identifier) -> Self {
        Self {
            goal,
            ..Self::default()
        }
    }

    /// Validates context values before optimization begins.
    pub fn validate(&self) -> Result<(), OptimizationStrategyError> {
        if self.goal.0.trim().is_empty() {
            return Err(OptimizationStrategyError::InvalidContext(
                "optimization goal cannot be empty".to_string(),
            ));
        }

        if !self.max_security_risk.is_finite()
            || !(0.0..=1.0).contains(&self.max_security_risk)
        {
            return Err(OptimizationStrategyError::InvalidContext(
                "max_security_risk must be finite and between 0.0 and 1.0"
                    .to_string(),
            ));
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Optimization result
// -----------------------------------------------------------------------------

/// Statistics returned by an optimization request.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OptimizationReport {
    pub initial_instruction_count: usize,
    pub final_instruction_count: usize,
    pub instruction_delta: isize,
    pub optimization_level: StrategyOptimizationLevel,
    pub passes_considered: usize,
    pub passes_applied: usize,
}

impl OptimizationReport {
    fn from_counts(
        initial: usize,
        final_count: usize,
        level: StrategyOptimizationLevel,
    ) -> Self {
        Self {
            initial_instruction_count: initial,
            final_instruction_count: final_count,
            instruction_delta: final_count as isize - initial as isize,
            optimization_level: level,
            passes_considered: 0,
            passes_applied: 1,
        }
    }
}

/// Complete result of strategy-level optimization.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationResult {
    pub module: IrModule,
    pub report: OptimizationReport,
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Structured optimization-strategy errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationStrategyError {
    InvalidContext(String),
    InvalidPass(String),
    OptimizationFailed(String),
}

impl fmt::Display for OptimizationStrategyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidContext(message) => {
                write!(formatter, "invalid optimization context: {message}")
            }
            Self::InvalidPass(message) => {
                write!(formatter, "invalid optimization pass: {message}")
            }
            Self::OptimizationFailed(message) => {
                write!(formatter, "optimization failed: {message}")
            }
        }
    }
}

impl std::error::Error for OptimizationStrategyError {}

// -----------------------------------------------------------------------------
// Optimization manager
// -----------------------------------------------------------------------------

/// Production optimization-strategy manager.
///
/// This type orchestrates the canonical optimizer instead of implementing
/// another optimizer inside `compiler/`.
#[derive(Debug, Clone)]
pub struct OptimizationManager {
    pub available_passes: List<OptimizationPass>,
    pub ai_optimization_model: OptimizationAIModel,
}

impl Default for OptimizationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationManager {
    /// Creates a manager with the repository's supported optimization metadata.
    pub fn new() -> Self {
        Self {
            available_passes: Self::load_all_passes(),
            ai_optimization_model: OptimizationAIModel::new(Identifier(
                "opt_strategy_model".to_string(),
                Span::dummy(),
            )),
        }
    }

    /// Registers an optimization-pass descriptor.
    pub fn register_pass(
        &mut self,
        pass: OptimizationPass,
    ) -> Result<(), OptimizationStrategyError> {
        if pass.id.0.trim().is_empty() {
            return Err(OptimizationStrategyError::InvalidPass(
                "pass identifier cannot be empty".to_string(),
            ));
        }

        if !pass.has_valid_risk_score() {
            return Err(OptimizationStrategyError::InvalidPass(
                "security risk must be finite and between 0.0 and 1.0"
                    .to_string(),
            ));
        }

        if self.available_passes.iter().any(|existing| existing.id == pass.id) {
            return Err(OptimizationStrategyError::InvalidPass(format!(
                "duplicate optimization pass '{}'",
                pass.id.0
            )));
        }

        self.available_passes.push(pass);
        Ok(())
    }

    /// Returns the number of registered strategy descriptors.
    pub fn pass_count(&self) -> usize {
        self.available_passes.len()
    }

    /// Optimizes an IR module through the canonical optimizer.
    pub fn optimize_ir(
        &mut self,
        ir: IrModule,
        context: OptimizationContext,
    ) -> Result<IrModule, OptimizationStrategyError> {
        Ok(self.optimize_ir_with_report(ir, context)?.module)
    }

    /// Optimizes IR and returns an auditable report.
    pub fn optimize_ir_with_report(
        &mut self,
        ir: IrModule,
        context: OptimizationContext,
    ) -> Result<OptimizationResult, OptimizationStrategyError> {
        context.validate()?;

        let level = self.ai_optimization_model.predict_level(&context)?;

        let initial_instruction_count = ir.instruction_count();

        println!(
            "[Compiler::OptStrat] Optimizing IR for goal '{}' at {:?}",
            context.goal.0, level
        );

        let config = level.optimizer_config();

        // The canonical optimizer is the sole implementation authority for
        // IR transformations.
        let mut optimizer = Optimizer::new(config);
        let optimized = optimizer.optimize(&ir);

        let final_instruction_count = optimized.instruction_count();

        let mut report = OptimizationReport::from_counts(
            initial_instruction_count,
            final_instruction_count,
            level,
        );

        report.passes_considered = self.available_passes.len();

        Ok(OptimizationResult {
            module: optimized,
            report,
        })
    }

    /// Performs a deterministic optimization without AI strategy selection.
    pub fn optimize_deterministic(
        &self,
        ir: IrModule,
        level: StrategyOptimizationLevel,
    ) -> Result<IrModule, OptimizationStrategyError> {
        let config = level.optimizer_config();
        let optimizer = Optimizer::new(config);

        Ok(optimizer.optimize(&ir))
    }

    /// Loads metadata for the optimization capabilities currently implemented
    /// by the canonical optimizer.
    fn load_all_passes() -> List<OptimizationPass> {
        vec![
            Self::pass(
                "constant_folding",
                "Constant folding and constant propagation.",
            ),
            Self::pass(
                "dead_code_elimination",
                "Removal of unreachable or unused computations.",
            ),
            Self::pass(
                "common_subexpression_elimination",
                "Elimination of redundant computations.",
            ),
            Self::pass(
                "strength_reduction",
                "Replacement of expensive operations with equivalent cheaper operations.",
            ),
            Self::pass(
                "dead_store_elimination",
                "Removal of stores whose values are never observed.",
            ),
            Self::pass(
                "branch_simplification",
                "Simplification of statically determined branches.",
            ),
        ]
    }

    fn pass(name: &str, description: &str) -> OptimizationPass {
        OptimizationPass::new(
            Identifier(name.to_string(), Span::dummy()),
            OptimizationCategory::IntermediateRepresentation,
            description,
        )
    }
}

// -----------------------------------------------------------------------------
// Paradigm routing
// -----------------------------------------------------------------------------

/// High-level compilation paradigm.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParadigmStrategy {
    Imperative,
    Functional,
    Quantum,
    Nano,
    Biological,
    Actor,
    Metaphysical,
}

/// Routes paradigm names to strategy categories.
///
/// This router does not perform compilation. It only resolves a normalized
/// paradigm name.
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

    pub fn resolve(
        &self,
        paradigm: &str,
    ) -> Result<ParadigmStrategy, String> {
        let key = normalize_name(paradigm);

        if key.is_empty() {
            return Err("paradigm name cannot be empty".to_string());
        }

        self.handlers.get(&key).cloned().ok_or_else(|| {
            format!(
                "Unknown paradigm '{}'. Available paradigms: {}",
                paradigm,
                self.available_paradigms().join(", ")
            )
        })
    }

    pub fn register(
        &mut self,
        name: impl Into<String>,
        strategy: ParadigmStrategy,
    ) -> Result<(), String> {
        let key = normalize_name(&name.into());

        if key.is_empty() {
            return Err("paradigm name cannot be empty".to_string());
        }

        self.handlers.insert(key, strategy);
        Ok(())
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
            .contains_key(&normalize_name(paradigm))
    }
}

fn normalize_name(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

// -----------------------------------------------------------------------------
// Compatibility compilation API
// -----------------------------------------------------------------------------

/// Minimal compatibility representation for callers that use this module as
/// a strategy-selection layer.
///
/// Actual machine-code/backend emission belongs elsewhere in the compiler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledBinary {
    pub data: Vec<u8>,
    pub format: String,
}

/// High-level compilation strategy.
#[derive(Debug, Clone, PartialEq)]
pub enum CompilationStrategy {
    AheadOfTime {
        optimization_level: StrategyOptimizationLevel,
    },
    JustInTime {
        profiling: bool,
        recompile_threshold: f32,
    },
    AdaptiveOptimization {
        learning_rate: f32,
    },
    MultiParadigmTranspilation {
        source: Identifier,
        target: Identifier,
    },
    HardwareSynthesis {
        target: Identifier,
        clock_speed_mhz: f32,
        power_budget_mw: f32,
    },
    QuantumCompilation {
        qubit_count: u32,
        error_correction_scheme: Identifier,
        target_qpu_architecture: Identifier,
    },
    NanoCompilation {
        agent_swarm_size: u32,
        target_nacu_version: Identifier,
        bio_compatibility_mode: bool,
    },
    MixedMode(Vec<CompilationStrategy>),
}

/// Strategy-level orchestrator.
///
/// It selects optimization policy only. It does not duplicate the main
/// compiler frontend/backend pipeline.
#[derive(Debug, Clone)]
pub struct CompilationOrchestrator {
    pub current_strategy: Option<CompilationStrategy>,
    pub optimization_manager: OptimizationManager,
}

impl CompilationOrchestrator {
    pub fn new() -> Self {
        Self {
            current_strategy: None,
            optimization_manager: OptimizationManager::new(),
        }
    }

    pub fn set_strategy(&mut self, strategy: CompilationStrategy) {
        self.current_strategy = Some(strategy);
    }

    pub fn strategy(&self) -> Option<&CompilationStrategy> {
        self.current_strategy.as_ref()
    }

    /// Compiles an already constructed program at strategy level.
    ///
    /// This intentionally does not emit native machine code. The canonical
    /// compiler/backend owns that responsibility.
    pub fn compile_program(
        &mut self,
        _program: Program,
        _target: impl fmt::Debug,
    ) -> Result<CompiledBinary, String> {
        let strategy = self.current_strategy.as_ref().ok_or_else(|| {
            "no compilation strategy has been selected".to_string()
        })?;

        let format = match strategy {
            CompilationStrategy::AheadOfTime {
                optimization_level,
            } => format!("aot:{optimization_level:?}"),

            CompilationStrategy::JustInTime {
                profiling,
                recompile_threshold,
            } => format!(
                "jit:profiling={profiling}:threshold={recompile_threshold}"
            ),

            CompilationStrategy::AdaptiveOptimization { learning_rate } => {
                format!("adaptive:learning_rate={learning_rate}")
            }

            CompilationStrategy::MultiParadigmTranspilation {
                source,
                target,
            } => format!("transpile:{}->{}", source.0, target.0),

            CompilationStrategy::HardwareSynthesis {
                target,
                clock_speed_mhz,
                power_budget_mw,
            } => format!(
                "hdl:{}:{}MHz:{}mW",
                target.0, clock_speed_mhz, power_budget_mw
            ),

            CompilationStrategy::QuantumCompilation {
                qubit_count,
                error_correction_scheme,
                target_qpu_architecture,
            } => format!(
                "quantum:{}qubits:{}:{}",
                qubit_count,
                error_correction_scheme.0,
                target_qpu_architecture.0
            ),

            CompilationStrategy::NanoCompilation {
                agent_swarm_size,
                target_nacu_version,
                bio_compatibility_mode,
            } => format!(
                "nano:{}agents:{}:bio={}",
                agent_swarm_size,
                target_nacu_version.0,
                bio_compatibility_mode
            ),

            CompilationStrategy::MixedMode(strategies) => {
                if strategies.is_empty() {
                    return Err(
                        "MixedMode requires at least one strategy".to_string()
                    );
                }

                format!("mixed:{}strategies", strategies.len())
            }
        };

        Ok(CompiledBinary {
            data: Vec::new(),
            format,
        })
    }
}

impl Default for CompilationOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn identifier(name: &str) -> Identifier {
        Identifier(name.to_string(), Span::dummy())
    }

    #[test]
    fn optimization_levels_map_to_canonical_optimizer() {
        assert_eq!(
            StrategyOptimizationLevel::None
                .optimizer_config()
                .level,
            0
        );

        assert_eq!(
            StrategyOptimizationLevel::Basic
                .optimizer_config()
                .level,
            1
        );

        assert_eq!(
            StrategyOptimizationLevel::Standard
                .optimizer_config()
                .level,
            2
        );

        assert_eq!(
            StrategyOptimizationLevel::Aggressive
                .optimizer_config()
                .level,
            3
        );
    }

    #[test]
    fn default_context_is_valid() {
        assert!(OptimizationContext::default().validate().is_ok());
    }

    #[test]
    fn invalid_security_risk_is_rejected() {
        let context = OptimizationContext {
            max_security_risk: 2.0,
            ..OptimizationContext::default()
        };

        assert!(context.validate().is_err());
    }

    #[test]
    fn manager_loads_real_optimizer_capabilities() {
        let manager = OptimizationManager::new();

        assert!(manager.pass_count() >= 6);
    }

    #[test]
    fn manager_performs_real_ir_optimization_path() {
        let manager = OptimizationManager::new();

        // An empty IR module is still a valid optimizer input and exercises
        // the integration boundary without requiring knowledge of additional
        // IR constructors.
        let ir = IrModule::new("optimization_test");

        let result = manager
            .optimize_deterministic(
                ir,
                StrategyOptimizationLevel::Standard,
            )
            .expect("canonical optimizer should succeed");

        assert_eq!(
            result.instruction_count(),
            0
        );
    }

    #[test]
    fn ai_model_is_conservative_when_disabled() {
        let model = OptimizationAIModel::new(identifier("test-model"));

        let context = OptimizationContext {
            optimization_level: StrategyOptimizationLevel::Aggressive,
            ..OptimizationContext::default()
        };

        assert_eq!(
            model.predict_level(&context).unwrap(),
            StrategyOptimizationLevel::Aggressive
        );
    }

    #[test]
    fn enabled_model_selects_size_strategy() {
        let mut model = OptimizationAIModel::new(identifier("test-model"));
        model.enable();

        let context = OptimizationContext {
            goal: identifier("minimize_binary_size"),
            ..OptimizationContext::default()
        };

        assert_eq!(
            model.predict_level(&context).unwrap(),
            StrategyOptimizationLevel::Size
        );
    }

    #[test]
    fn pass_registration_rejects_duplicate_ids() {
        let mut manager = OptimizationManager::new();

        let pass = OptimizationPass::new(
            identifier("constant_folding"),
            OptimizationCategory::IntermediateRepresentation,
            "duplicate",
        );

        assert!(manager.register_pass(pass).is_err());
    }

    #[test]
    fn pass_registration_rejects_invalid_risk() {
        let mut manager = OptimizationManager::new();

        let mut pass = OptimizationPass::new(
            identifier("unsafe_pass"),
            OptimizationCategory::IntermediateRepresentation,
            "invalid risk",
        );

        pass.security_risk_assessment = 2.0;

        assert!(manager.register_pass(pass).is_err());
    }

    #[test]
    fn paradigm_router_is_case_insensitive() {
        let router = ParadigmRouter::new();

        assert_eq!(
            router.resolve("QUANTUM").unwrap(),
            ParadigmStrategy::Quantum
        );
    }

    #[test]
    fn paradigm_router_rejects_empty_names() {
        let router = ParadigmRouter::new();

        assert!(router.resolve("   ").is_err());
    }

    #[test]
    fn paradigm_router_registers_custom_paradigm() {
        let mut router = ParadigmRouter::new();

        router
            .register("custom", ParadigmStrategy::Actor)
            .expect("registration should succeed");

        assert!(router.contains("CUSTOM"));
    }

    #[test]
    fn mixed_mode_requires_a_strategy() {
        let mut orchestrator = CompilationOrchestrator::new();

        orchestrator.set_strategy(CompilationStrategy::MixedMode(Vec::new()));

        let program =
            Program::new(Vec::new(), Span::dummy());

        assert!(
            orchestrator
                .compile_program(program, "test-target")
                .is_err()
        );
    }
}