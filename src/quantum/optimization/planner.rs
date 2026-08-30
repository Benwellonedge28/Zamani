//! Zamani Quantum Optimization — Production Optimization Planner
//!
//! The planner is the policy-to-plan boundary of `quantum::optimization`.
//!
//! It does not execute passes and does not mutate the canonical Quantum IR.
//! It inspects a canonical `QuantumCircuit`, combines circuit characteristics
//! with `OptimizationConfig`, and produces a deterministic, bounded plan that
//! the registry/pipeline can execute later.
//!
//! Architectural direction:
//!
//! ```text
//! quantum::ir::QuantumCircuit
//!          │
//!          ▼
//!     OptimizationConfig
//!          │
//!          ▼
//!        Planner ───────────► PlannerCatalog / registry
//!          │
//!          ▼
//!   OptimizationPlan
//!          │
//!          ▼
//!      pipeline.rs
//! ```
//!
//! The planner deliberately does not depend on concrete pass implementations.
//! This is what allows a new pass to be registered later without changing this
//! file. A pass is selected by its stable `PassId` string; the registry resolves
//! that identifier to an implementation.
//!
//! # Production properties
//!
//! - canonical Quantum IR only;
//! - no duplicate gate/circuit representation;
//! - no `unsafe` code;
//! - deterministic by construction for deterministic configurations;
//! - explicit handling of enabled/disabled passes;
//! - bounded plan construction;
//! - checked arithmetic for all work estimates;
//! - circuit-size-aware pass selection;
//! - target/profile/objective-aware planning;
//! - measurement/reset/barrier safety awareness;
//! - symbolic-parameter awareness;
//! - fault-tolerant planning;
//! - two-qubit/depth/gate-count specialization;
//! - optional registry/catalog filtering without coupling to registry.rs;
//! - no backend I/O;
//! - no routing;
//! - no scheduling;
//! - no benchmark execution;
//! - no QPU execution.
//!
//! # Scaling
//!
//! Circuit size is represented using `u128` in the planner's analysis model so
//! the planner itself does not introduce a small-integer artificial ceiling.
//! Actual circuit creation and execution remain bounded by the canonical IR
//! and optimizer resource policies. Planning work is proportional to the
//! number of operations plus a small fixed number of planning rules; it never
//! expands into a search proportional to the number of possible circuits.
//!
//! # Integration contract
//!
//! `config.rs` owns policy. `operation.rs` owns semantic operation
//! classification. `registry.rs` owns pass discovery. `pipeline.rs` owns
//! execution. `context.rs` owns invocation state. `cost.rs` owns detailed cost
//! comparison. `verification/*` owns semantic equivalence. `routing` and
//! `scheduling` remain downstream.
//!
//! This file intentionally contains enough information for those modules to
//! integrate later without requiring planner changes merely because another
//! pass, analysis, target, or backend is added.

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::QuantumCircuit;

use super::config::{
    Determinism,
    FixedPointPolicy,
    OptimizationConfig,
    OptimizationLevel,
    OptimizationObjective,
    OptimizationProfile,
    TargetSelection,
    VerificationMode,
};
use super::operation::OperationView;

// =============================================================================
// Stable planner constants
// =============================================================================

/// Current planner contract version.
///
/// Increment this when the meaning/order of planning policy changes in a way
/// that affects reproducibility or persisted plan diagnostics.
pub const PLANNER_VERSION: u32 = 1;

/// Maximum number of passes the planner may place in one plan.
///
/// This is deliberately independent of circuit size. A huge circuit should
/// not cause the planner to allocate a huge pass list.
pub const MAX_PLAN_PASSES: usize = 128;

/// Maximum number of explanatory notes retained in one plan.
pub const MAX_PLAN_NOTES: usize = 128;

// =============================================================================
// Pass identifiers
// =============================================================================

/// Stable planner pass identifiers.
///
/// These are strings rather than registry types so this module remains
/// independent from `registry.rs`. The registry resolves them later.
pub mod pass_id {
    /// Canonical IR normalization.
    pub const NORMALIZE_CANONICAL: &str = "normalize.canonical";

    /// Parameter-expression normalization.
    pub const NORMALIZE_PARAMETERS: &str = "normalize.parameters";

    /// Identity elimination.
    pub const LOCAL_IDENTITY: &str = "local.identity";

    /// Generic inverse/cancellation optimization.
    pub const LOCAL_CANCELLATION: &str = "local.cancellation";

    /// Inverse-pair optimization.
    pub const LOCAL_INVERSE: &str = "local.inverse";

    /// Rotation/phase fusion.
    pub const LOCAL_ROTATION: &str = "local.rotation";

    /// Peephole optimization.
    pub const LOCAL_PEEPHOLE: &str = "local.peephole";

    /// Commutation-aware optimization.
    pub const LOCAL_COMMUTATION: &str = "local.commutation";

    /// Template rewriting.
    pub const LOCAL_TEMPLATES: &str = "local.templates";

    /// Gate fusion.
    pub const LOCAL_FUSION: &str = "local.gate_fusion";

    /// Clifford algebra optimization.
    pub const ALGEBRA_CLIFFORD: &str = "algebra.clifford";

    /// Phase-polynomial optimization.
    pub const ALGEBRA_PHASE_POLYNOMIAL: &str = "algebra.phase_polynomial";

    /// Diagonal-circuit optimization.
    pub const ALGEBRA_DIAGONAL: &str = "algebra.diagonal";

    /// Single-qubit synthesis.
    pub const SYNTHESIS_SINGLE_QUBIT: &str = "synthesis.single_qubit";

    /// Two-qubit synthesis.
    pub const SYNTHESIS_TWO_QUBIT: &str = "synthesis.two_qubit";

    /// Clifford synthesis.
    pub const SYNTHESIS_CLIFFORD: &str = "synthesis.clifford";

    /// Phase synthesis.
    pub const SYNTHESIS_PHASE: &str = "synthesis.phase";

    /// T-count optimization.
    pub const FAULT_TOLERANT_T_COUNT: &str = "fault_tolerant.t_count";

    /// T-depth optimization.
    pub const FAULT_TOLERANT_T_DEPTH: &str = "fault_tolerant.t_depth";

    /// T-gate reduction.
    pub const FAULT_TOLERANT_T_REDUCTION: &str =
        "fault_tolerant.t_gate_reduction";

    /// Clifford+T optimization.
    pub const FAULT_TOLERANT_CLIFFORD_T: &str =
        "fault_tolerant.clifford_t";

    /// Constant folding.
    pub const PARAMETER_CONSTANT_FOLD: &str =
        "parameter.constant_fold";

    /// Symbolic-parameter optimization.
    pub const PARAMETER_SYMBOLIC: &str = "parameter.symbolic";

    /// Block formation.
    pub const STRUCTURE_BLOCK: &str = "structure.block";

    /// Region optimization.
    pub const STRUCTURE_REGION: &str = "structure.region";

    /// Conditional optimization.
    pub const STRUCTURE_CONDITIONAL: &str =
        "structure.conditional";

    /// General control-flow optimization.
    pub const STRUCTURE_CONTROL_FLOW: &str =
        "structure.control_flow";

    /// Composite gate-count optimization.
    pub const OPTIMIZE_GATE_COUNT: &str =
        "passes.optimize_gate_count";

    /// Composite depth optimization.
    pub const OPTIMIZE_DEPTH: &str =
        "passes.optimize_depth";

    /// Composite width optimization.
    pub const OPTIMIZE_WIDTH: &str =
        "passes.optimize_width";

    /// Composite two-qubit optimization.
    pub const OPTIMIZE_TWO_QUBIT: &str =
        "passes.optimize_two_qubit";

    /// Composite fault-tolerance optimization.
    pub const OPTIMIZE_FAULT_TOLERANCE: &str =
        "passes.optimize_fault_tolerance";
}

// =============================================================================
// Errors
// =============================================================================

/// Planner-specific errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannerError {
    /// The supplied optimizer configuration is invalid.
    InvalidConfiguration {
        /// Human-readable configuration error.
        message: String,
    },

    /// A plan would exceed the planner's explicit pass-count safety boundary.
    PlanPassLimitExceeded {
        /// Requested pass count.
        requested: usize,

        /// Maximum permitted pass count.
        maximum: usize,
    },

    /// The circuit is larger than the configured optimizer policy permits.
    CircuitOperationsLimitExceeded {
        /// Actual operation count.
        requested: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The circuit uses more qubits than the configured optimizer policy
    /// permits.
    CircuitQubitsLimitExceeded {
        /// Actual qubit count.
        requested: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A checked arithmetic operation used for work estimation overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// A caller supplied an invalid pass identifier through an explicit
    /// enable/disable list.
    InvalidPassIdentifier {
        /// Invalid identifier.
        value: String,
    },

    /// An explicitly requested pass is disabled by configuration.
    ConflictingPassSelection {
        /// Conflicting pass.
        pass: String,
    },

    /// A required explicit pass is unavailable in a supplied catalog.
    MissingPass {
        /// Missing pass identifier.
        pass: String,
    },

    /// A requested target is unsupported by the supplied catalog.
    UnsupportedTarget {
        /// Requested target.
        target: String,
    },
}

impl fmt::Display for PlannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { message } => {
                write!(
                    f,
                    "invalid optimization planner configuration: {message}"
                )
            }

            Self::PlanPassLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "optimization plan contains {requested} passes, maximum {maximum}"
                )
            }

            Self::CircuitOperationsLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "circuit contains {requested} operations, \
                     optimizer maximum is {maximum}"
                )
            }

            Self::CircuitQubitsLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "circuit contains {requested} qubits, \
                     optimizer maximum is {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    f,
                    "optimization planner arithmetic overflow while \
                     calculating {calculation}"
                )
            }

            Self::InvalidPassIdentifier { value } => {
                write!(
                    f,
                    "invalid optimization pass identifier `{value}`"
                )
            }

            Self::ConflictingPassSelection { pass } => {
                write!(
                    f,
                    "optimization pass `{pass}` is both enabled and disabled"
                )
            }

            Self::MissingPass { pass } => {
                write!(
                    f,
                    "required optimization pass `{pass}` is not \
                     available in the planner catalog"
                )
            }

            Self::UnsupportedTarget { target } => {
                write!(
                    f,
                    "optimization target `{target}` is unsupported \
                     by the planner catalog"
                )
            }
        }
    }
}

impl std::error::Error for PlannerError {}

/// Result type used by the planner.
pub type PlannerResult<T> = Result<T, PlannerError>;

// =============================================================================
// Circuit characteristics
// =============================================================================

/// Immutable characteristics extracted from the canonical Quantum IR.
///
/// This is deliberately a planner-local summary, not a second IR or a
/// replacement for `analysis/*`. The full analysis subsystem will provide
/// richer cached analyses; the planner only needs a small, bounded feature set
/// to select an initial strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CircuitCharacteristics {
    /// Number of logical qubits.
    pub qubits: u128,

    /// Total operation count.
    pub operations: u128,

    /// Unitary operation count.
    pub unitary_operations: u128,

    /// Non-unitary operation count.
    pub non_unitary_operations: u128,

    /// Single-qubit operation count.
    pub single_qubit_operations: u128,

    /// Multi-qubit operation count.
    pub multi_qubit_operations: u128,

    /// Measurement count.
    pub measurements: u128,

    /// Reset count.
    pub resets: u128,

    /// Barrier count.
    pub barriers: u128,

    /// Clifford operation count.
    pub clifford_operations: u128,

    /// Conservatively classified non-Clifford operation count.
    pub non_clifford_operations: u128,

    /// Rotation/phase operation count.
    pub rotations: u128,

    /// Symbolically parameterized operation count.
    pub symbolic_operations: u128,

    /// Constant-parameter operation count.
    pub constant_parameter_operations: u128,

    /// Controlled-operation count.
    pub controlled_operations: u128,

    /// Diagonal-operation count.
    pub diagonal_operations: u128,

    /// Identity operation count.
    pub identity_operations: u128,

    /// Operations with classical destinations.
    pub classical_targets: u128,
}

impl CircuitCharacteristics {
    /// Returns whether the circuit is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.operations == 0
    }

    /// Returns whether the circuit contains semantic boundaries.
    #[must_use]
    pub const fn has_semantic_boundaries(self) -> bool {
        self.measurements > 0
            || self.resets > 0
            || self.barriers > 0
            || self.classical_targets > 0
    }

    /// Returns whether the circuit is predominantly Clifford.
    #[must_use]
    pub const fn is_predominantly_clifford(self) -> bool {
        self.operations > 0
            && self.clifford_operations.saturating_mul(4)
                >= self.operations.saturating_mul(3)
    }

    /// Returns whether the circuit is non-Clifford heavy.
    #[must_use]
    pub const fn is_non_clifford_heavy(self) -> bool {
        self.operations > 0
            && self.non_clifford_operations.saturating_mul(3)
                >= self.operations
    }

    /// Returns whether the circuit is entangling heavy.
    #[must_use]
    pub const fn is_entangling_heavy(self) -> bool {
        self.operations > 0
            && self.multi_qubit_operations.saturating_mul(3)
                >= self.operations
    }

    /// Returns whether symbolic parameters are significant.
    #[must_use]
    pub const fn is_symbolic_heavy(self) -> bool {
        self.operations > 0
            && self.symbolic_operations.saturating_mul(4)
                >= self.operations
    }

    /// Returns a coarse size class used only for planning heuristics.
    #[must_use]
    pub const fn size_class(self) -> CircuitSizeClass {
        if self.operations <= 32 {
            CircuitSizeClass::Tiny
        } else if self.operations <= 1_024 {
            CircuitSizeClass::Small
        } else if self.operations <= 100_000 {
            CircuitSizeClass::Medium
        } else if self.operations <= 10_000_000 {
            CircuitSizeClass::Large
        } else {
            CircuitSizeClass::Massive
        }
    }
}

/// Planner-only circuit size class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CircuitSizeClass {
    /// Up to 32 operations.
    Tiny,

    /// Up to 1,024 operations.
    Small,

    /// Up to 100,000 operations.
    Medium,

    /// Up to 10 million operations.
    Large,

    /// Above 10 million operations.
    Massive,
}

// =============================================================================
// Plan stages and reasons
// =============================================================================

/// Logical stage of a planned optimization pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlanStage {
    /// Canonicalization.
    Normalize,

    /// Parameter normalization.
    Parameter,

    /// Local rewriting.
    Local,

    /// Algebraic rewriting.
    Algebraic,

    /// Structural optimization.
    Structural,

    /// Synthesis.
    Synthesis,

    /// Fault-tolerant optimization.
    FaultTolerant,

    /// Objective-specific optimization.
    Objective,

    /// Finalization.
    Finalize,
}

impl PlanStage {
    /// Returns the stable textual stage identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normalize => "normalize",
            Self::Parameter => "parameter",
            Self::Local => "local",
            Self::Algebraic => "algebraic",
            Self::Structural => "structural",
            Self::Synthesis => "synthesis",
            Self::FaultTolerant => "fault_tolerant",
            Self::Objective => "objective",
            Self::Finalize => "finalize",
        }
    }
}

/// Why a pass was selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanReason {
    /// Canonical normalization is a fundamental optimizer boundary.
    RequiredNormalization,

    /// Parameters require dedicated handling.
    ParameterizedCircuit,

    /// Local simplification is beneficial.
    LocalSimplification,

    /// The circuit contains significant entangling work.
    EntanglingCircuit,

    /// The circuit is predominantly Clifford.
    CliffordHeavy,

    /// The circuit has significant non-Clifford work.
    NonCliffordHeavy,

    /// Fault-tolerant resources are the objective.
    FaultTolerantObjective,

    /// Depth is the primary objective.
    DepthObjective,

    /// Gate count is the primary objective.
    GateCountObjective,

    /// Two-qubit count is the primary objective.
    TwoQubitObjective,

    /// Width is the primary objective.
    WidthObjective,

    /// Verification-oriented profile.
    VerificationProfile,

    /// Aggressive profile.
    AggressiveProfile,

    /// Structural boundaries require special handling.
    StructuralBoundaries,

    /// Target-specific optimization was requested.
    TargetAware,

    /// Explicit user selection.
    ExplicitUserSelection,
}

impl PlanReason {
    /// Returns the stable textual reason identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredNormalization => "required_normalization",
            Self::ParameterizedCircuit => "parameterized_circuit",
            Self::LocalSimplification => "local_simplification",
            Self::EntanglingCircuit => "entangling_circuit",
            Self::CliffordHeavy => "clifford_heavy",
            Self::NonCliffordHeavy => "non_clifford_heavy",
            Self::FaultTolerantObjective => "fault_tolerant_objective",
            Self::DepthObjective => "depth_objective",
            Self::GateCountObjective => "gate_count_objective",
            Self::TwoQubitObjective => "two_qubit_objective",
            Self::WidthObjective => "width_objective",
            Self::VerificationProfile => "verification_profile",
            Self::AggressiveProfile => "aggressive_profile",
            Self::StructuralBoundaries => "structural_boundaries",
            Self::TargetAware => "target_aware",
            Self::ExplicitUserSelection => "explicit_user_selection",
        }
    }
}

// =============================================================================
// Planned pass
// =============================================================================

/// One pass selected by the planner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedPass {
    id: String,
    stage: PlanStage,
    reason: PlanReason,
    priority: u16,
    repeatable: bool,
}

impl PlannedPass {
    fn new(
        id: &'static str,
        stage: PlanStage,
        reason: PlanReason,
        priority: u16,
        repeatable: bool,
    ) -> Self {
        Self {
            id: id.to_owned(),
            stage,
            reason,
            priority,
            repeatable,
        }
    }

    /// Stable pass identifier resolved by `registry.rs`.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Logical plan stage.
    #[must_use]
    pub const fn stage(&self) -> PlanStage {
        self.stage
    }

    /// Selection reason.
    #[must_use]
    pub const fn reason(&self) -> PlanReason {
        self.reason
    }

    /// Relative planner priority. Lower values execute earlier.
    #[must_use]
    pub const fn priority(&self) -> u16 {
        self.priority
    }

    /// Whether this pass may safely participate in fixed-point execution.
    #[must_use]
    pub const fn repeatable(&self) -> bool {
        self.repeatable
    }
}

// =============================================================================
// Plan
// =============================================================================

/// Complete immutable plan produced for one optimization invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizationPlan {
    planner_version: u32,
    characteristics: CircuitCharacteristics,
    target: TargetSelection,
    profile: OptimizationProfile,
    level: OptimizationLevel,
    objective: OptimizationObjective,
    fixed_point: FixedPointPolicy,
    verification: VerificationMode,
    deterministic: bool,
    seeded: bool,
    passes: Vec<PlannedPass>,
    estimated_work_units: u128,
    notes: Vec<String>,
}

impl OptimizationPlan {
    /// Returns the planner contract version.
    #[must_use]
    pub const fn planner_version(&self) -> u32 {
        self.planner_version
    }

    /// Returns the planner's circuit summary.
    #[must_use]
    pub const fn characteristics(&self) -> CircuitCharacteristics {
        self.characteristics
    }

    /// Returns the selected target.
    #[must_use]
    pub fn target(&self) -> &TargetSelection {
        &self.target
    }

    /// Returns the selected profile.
    #[must_use]
    pub const fn profile(&self) -> OptimizationProfile {
        self.profile
    }

    /// Returns the selected optimization level.
    #[must_use]
    pub const fn level(&self) -> OptimizationLevel {
        self.level
    }

    /// Returns the selected objective.
    #[must_use]
    pub const fn objective(&self) -> OptimizationObjective {
        self.objective
    }

    /// Returns the fixed-point policy.
    #[must_use]
    pub const fn fixed_point(&self) -> FixedPointPolicy {
        self.fixed_point
    }

    /// Returns the semantic verification mode.
    #[must_use]
    pub const fn verification(&self) -> VerificationMode {
        self.verification
    }

    /// Returns whether the plan is deterministic.
    #[must_use]
    pub const fn deterministic(&self) -> bool {
        self.deterministic
    }

    /// Returns whether deterministic seeding is required.
    #[must_use]
    pub const fn seeded(&self) -> bool {
        self.seeded
    }

    /// Returns the ordered planned passes.
    #[must_use]
    pub fn passes(&self) -> &[PlannedPass] {
        &self.passes
    }

    /// Returns the planner's conservative work estimate.
    #[must_use]
    pub const fn estimated_work_units(&self) -> u128 {
        self.estimated_work_units
    }

    /// Returns explanatory planner notes.
    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Returns only the stable pass identifiers.
    #[must_use]
    pub fn pass_ids(&self) -> impl Iterator<Item = &str> {
        self.passes.iter().map(PlannedPass::id)
    }

    /// Returns whether a pass is present.
    #[must_use]
    pub fn contains(&self, id: &str) -> bool {
        self.passes.iter().any(|pass| pass.id == id)
    }
}

// =============================================================================
// Planner catalog abstraction
// =============================================================================

/// Minimal registry contract required by the planner.
///
/// `registry.rs` can implement this trait later without changing planner
/// logic. Keeping this trait intentionally small prevents the planner from
/// depending on the complete registry implementation.
pub trait PlannerCatalog {
    /// Returns whether a pass identifier exists.
    fn contains(&self, pass_id: &str) -> bool;

    /// Returns whether the pass is enabled for automatic discovery.
    fn enabled_by_default(&self, pass_id: &str) -> bool;

    /// Returns whether the pass supports the requested target identifier.
    ///
    /// The planner deliberately passes the canonical target string and leaves
    /// interpretation to the registry/catalog adapter.
    fn supports_target(&self, pass_id: &str, target: &str) -> bool;
}

// =============================================================================
// Planner
// =============================================================================

/// Production optimization planner.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptimizationPlanner;

impl OptimizationPlanner {
    /// Creates a planner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Plans optimization without requiring the registry.
    ///
    /// The returned pass identifiers are resolved by `pipeline.rs` through the
    /// pass registry. This method is useful during compiler bootstrap and unit
    /// testing where no registry is available yet.
    pub fn plan(
        &self,
        circuit: &QuantumCircuit,
        config: &OptimizationConfig,
    ) -> PlannerResult<OptimizationPlan> {
        let characteristics = characterize(circuit);

        self.plan_from_characteristics(characteristics, config)
    }

    /// Plans using a registry/catalog for availability and target filtering.
    ///
    /// Explicitly enabled passes are required to exist. Automatically selected
    /// passes are omitted when unavailable; this permits reduced compiler builds
    /// to use the same planner without pretending an absent optional pass exists.
    pub fn plan_with_catalog<C: PlannerCatalog>(
        &self,
        circuit: &QuantumCircuit,
        config: &OptimizationConfig,
        catalog: &C,
    ) -> PlannerResult<OptimizationPlan> {
        let mut plan = self.plan(circuit, config)?;

        let target = target_identifier(&config.target);

        for requested in &config.enabled_passes {
            if !catalog.contains(requested) {
                return Err(PlannerError::MissingPass {
                    pass: requested.clone(),
                });
            }

            if !catalog.supports_target(requested, target) {
                return Err(PlannerError::UnsupportedTarget {
                    target: target.to_owned(),
                });
            }
        }

        plan.passes.retain(|pass| {
            if config
                .enabled_passes
                .iter()
                .any(|id| id == pass.id())
            {
                return true;
            }

            catalog.contains(pass.id())
                && catalog.supports_target(pass.id(), target)
        });

        // Explicit enablement is authoritative and can add a pass that was
        // not selected by the automatic heuristic.
        for requested in &config.enabled_passes {
            if !plan.contains(requested) {
                if plan.passes.len() >= MAX_PLAN_PASSES {
                    return Err(
                        PlannerError::PlanPassLimitExceeded {
                            requested: plan.passes.len() + 1,
                            maximum: MAX_PLAN_PASSES,
                        },
                    );
                }

                plan.passes.push(PlannedPass {
                    id: requested.clone(),
                    stage: PlanStage::Objective,
                    reason: PlanReason::ExplicitUserSelection,
                    priority: 90_000,
                    repeatable: false,
                });
            }
        }

        sort_and_deduplicate(&mut plan.passes);
        recompute_work_estimate(&mut plan)?;

        Ok(plan)
    }

    /// Plans directly from a bounded characteristic summary.
    ///
    /// This API is useful for incremental compilation, cached analyses, and
    /// distributed planning where the complete circuit is not retained by the
    /// planner process.
    pub fn plan_from_characteristics(
        &self,
        characteristics: CircuitCharacteristics,
        config: &OptimizationConfig,
    ) -> PlannerResult<OptimizationPlan> {
        config.validate().map_err(|error| {
            PlannerError::InvalidConfiguration {
                message: error.to_string(),
            }
        })?;

        check_limits(characteristics, config)?;

        let mut builder = PlanBuilder::new(characteristics);

        // ---------------------------------------------------------------------
        // Mandatory normalization
        // ---------------------------------------------------------------------

        builder.add(
            pass_id::NORMALIZE_CANONICAL,
            PlanStage::Normalize,
            PlanReason::RequiredNormalization,
            100,
            false,
        )?;

        // ---------------------------------------------------------------------
        // Parameter handling
        // ---------------------------------------------------------------------

        if characteristics.symbolic_operations > 0
            || characteristics.constant_parameter_operations > 0
        {
            builder.add(
                pass_id::NORMALIZE_PARAMETERS,
                PlanStage::Parameter,
                PlanReason::ParameterizedCircuit,
                200,
                false,
            )?;

            builder.add(
                pass_id::PARAMETER_CONSTANT_FOLD,
                PlanStage::Parameter,
                PlanReason::ParameterizedCircuit,
                210,
                false,
            )?;

            if characteristics.symbolic_operations > 0 {
                builder.add(
                    pass_id::PARAMETER_SYMBOLIC,
                    PlanStage::Parameter,
                    PlanReason::ParameterizedCircuit,
                    220,
                    true,
                )?;
            }
        }

        // ---------------------------------------------------------------------
        // Cheap local optimization
        // ---------------------------------------------------------------------

        if config.level != OptimizationLevel::O0 {
            builder.add(
                pass_id::LOCAL_IDENTITY,
                PlanStage::Local,
                PlanReason::LocalSimplification,
                300,
                true,
            )?;

            builder.add(
                pass_id::LOCAL_CANCELLATION,
                PlanStage::Local,
                PlanReason::LocalSimplification,
                310,
                true,
            )?;

            builder.add(
                pass_id::LOCAL_INVERSE,
                PlanStage::Local,
                PlanReason::LocalSimplification,
                320,
                true,
            )?;

            if characteristics.rotations > 0 {
                builder.add(
                    pass_id::LOCAL_ROTATION,
                    PlanStage::Local,
                    PlanReason::ParameterizedCircuit,
                    330,
                    true,
                )?;
            }

            builder.add(
                pass_id::LOCAL_PEEPHOLE,
                PlanStage::Local,
                PlanReason::LocalSimplification,
                340,
                true,
            )?;
        }

        // ---------------------------------------------------------------------
        // Semantic-boundary safety
        // ---------------------------------------------------------------------

        let structurally_open =
            !config.respect_measurements
                || characteristics.measurements == 0;

        let reset_safe =
            !config.respect_resets || characteristics.resets == 0;

        let barrier_safe =
            !config.respect_barriers || characteristics.barriers == 0;

        let movement_safe =
            structurally_open
                && reset_safe
                && barrier_safe
                && characteristics.classical_targets == 0;

        // ---------------------------------------------------------------------
        // Commutation
        // ---------------------------------------------------------------------

        if movement_safe && characteristics.operations > 1 {
            builder.add(
                pass_id::LOCAL_COMMUTATION,
                PlanStage::Local,
                PlanReason::EntanglingCircuit,
                350,
                true,
            )?;
        }

        // ---------------------------------------------------------------------
        // Templates and fusion
        // ---------------------------------------------------------------------

        if characteristics.operations > 2 {
            builder.add(
                pass_id::LOCAL_TEMPLATES,
                PlanStage::Local,
                PlanReason::LocalSimplification,
                360,
                true,
            )?;
        }

        if characteristics.operations > 0
            && characteristics.single_qubit_operations > 0
        {
            builder.add(
                pass_id::LOCAL_FUSION,
                PlanStage::Local,
                PlanReason::LocalSimplification,
                370,
                true,
            )?;
        }

        // ---------------------------------------------------------------------
        // Clifford optimization
        // ---------------------------------------------------------------------

        if characteristics.is_predominantly_clifford() {
            builder.add(
                pass_id::ALGEBRA_CLIFFORD,
                PlanStage::Algebraic,
                PlanReason::CliffordHeavy,
                500,
                true,
            )?;
        }

        // ---------------------------------------------------------------------
        // Diagonal optimization
        // ---------------------------------------------------------------------

        if characteristics.diagonal_operations > 0
            && characteristics.multi_qubit_operations > 0
        {
            builder.add(
                pass_id::ALGEBRA_DIAGONAL,
                PlanStage::Algebraic,
                PlanReason::EntanglingCircuit,
                510,
                true,
            )?;
        }

        // ---------------------------------------------------------------------
        // Phase-polynomial optimization
        // ---------------------------------------------------------------------

        if characteristics.is_non_clifford_heavy()
            || config.profile.is_fault_tolerant()
        {
            builder.add(
                pass_id::ALGEBRA_PHASE_POLYNOMIAL,
                PlanStage::Algebraic,
                PlanReason::NonCliffordHeavy,
                520,
                true,
            )?;
        }

        // ---------------------------------------------------------------------
        // Synthesis
        // ---------------------------------------------------------------------

        if characteristics.operations > 0
            && characteristics.multi_qubit_operations > 0
        {
            builder.add(
                pass_id::SYNTHESIS_TWO_QUBIT,
                PlanStage::Synthesis,
                PlanReason::EntanglingCircuit,
                600,
                false,
            )?;
        }

        if characteristics.single_qubit_operations > 0 {
            builder.add(
                pass_id::SYNTHESIS_SINGLE_QUBIT,
                PlanStage::Synthesis,
                PlanReason::LocalSimplification,
                610,
                false,
            )?;
        }

        // ---------------------------------------------------------------------
        // Structural optimization
        // ---------------------------------------------------------------------

        if config.allow_ancillas
            && characteristics.multi_qubit_operations > 0
        {
            builder.add(
                pass_id::STRUCTURE_BLOCK,
                PlanStage::Structural,
                PlanReason::StructuralBoundaries,
                700,
                false,
            )?;
        }

        if characteristics.has_semantic_boundaries() {
            builder.note(
                "semantic boundaries detected; generic movement \
                 passes must respect measurement/reset/barrier/classical \
                 dependencies",
            );
        }

        // ---------------------------------------------------------------------
        // Primary objective
        // ---------------------------------------------------------------------

        match config.objective {
            OptimizationObjective::MinimizeGateCount
            | OptimizationObjective::PreserveCost => {
                builder.add(
                    pass_id::OPTIMIZE_GATE_COUNT,
                    PlanStage::Objective,
                    PlanReason::GateCountObjective,
                    800,
                    false,
                )?;
            }

            OptimizationObjective::MinimizeDepth
            | OptimizationObjective::MinimizeTwoQubitDepth => {
                if movement_safe {
                    builder.add(
                        pass_id::LOCAL_COMMUTATION,
                        PlanStage::Objective,
                        PlanReason::DepthObjective,
                        810,
                        true,
                    )?;
                }

                builder.add(
                    pass_id::OPTIMIZE_DEPTH,
                    PlanStage::Objective,
                    PlanReason::DepthObjective,
                    820,
                    false,
                )?;
            }

            OptimizationObjective::MinimizeTwoQubitGates => {
                builder.add(
                    pass_id::ALGEBRA_PHASE_POLYNOMIAL,
                    PlanStage::Algebraic,
                    PlanReason::TwoQubitObjective,
                    815,
                    true,
                )?;

                builder.add(
                    pass_id::OPTIMIZE_TWO_QUBIT,
                    PlanStage::Objective,
                    PlanReason::TwoQubitObjective,
                    830,
                    false,
                )?;
            }

            OptimizationObjective::MinimizeTCount => {
                builder.add(
                    pass_id::FAULT_TOLERANT_T_REDUCTION,
                    PlanStage::FaultTolerant,
                    PlanReason::FaultTolerantObjective,
                    900,
                    true,
                )?;

                builder.add(
                    pass_id::FAULT_TOLERANT_T_COUNT,
                    PlanStage::FaultTolerant,
                    PlanReason::FaultTolerantObjective,
                    910,
                    false,
                )?;
            }

            OptimizationObjective::MinimizeTDepth => {
                builder.add(
                    pass_id::FAULT_TOLERANT_T_REDUCTION,
                    PlanStage::FaultTolerant,
                    PlanReason::FaultTolerantObjective,
                    900,
                    true,
                )?;

                builder.add(
                    pass_id::FAULT_TOLERANT_T_DEPTH,
                    PlanStage::FaultTolerant,
                    PlanReason::FaultTolerantObjective,
                    920,
                    false,
                )?;
            }

            OptimizationObjective::MinimizeWidth => {
                builder.add(
                    pass_id::OPTIMIZE_WIDTH,
                    PlanStage::Objective,
                    PlanReason::WidthObjective,
                    840,
                    false,
                )?;
            }

            OptimizationObjective::MinimizeMeasurements => {
                // Deliberately no generic measurement-removal pass is invented.
                // Measurement transformations require semantic analysis and
                // belong to specialized passes.
            }

            OptimizationObjective::MinimizeDuration
            | OptimizationObjective::MinimizeError => {
                // These objectives require a concrete target/cost model.
                // The planner does not guess hardware-specific transformations.
            }

            OptimizationObjective::Balanced
            | OptimizationObjective::Lexicographic => {}
        }

        // ---------------------------------------------------------------------
        // Fault-tolerant planning
        // ---------------------------------------------------------------------

        if config.level.is_fault_tolerant()
            || config.profile.is_fault_tolerant()
        {
            builder.add(
                pass_id::FAULT_TOLERANT_CLIFFORD_T,
                PlanStage::FaultTolerant,
                PlanReason::FaultTolerantObjective,
                930,
                true,
            )?;

            builder.add(
                pass_id::FAULT_TOLERANT_T_REDUCTION,
                PlanStage::FaultTolerant,
                PlanReason::FaultTolerantObjective,
                940,
                true,
            )?;

            builder.add(
                pass_id::FAULT_TOLERANT_T_COUNT,
                PlanStage::FaultTolerant,
                PlanReason::FaultTolerantObjective,
                950,
                false,
            )?;

            builder.add(
                pass_id::FAULT_TOLERANT_T_DEPTH,
                PlanStage::FaultTolerant,
                PlanReason::FaultTolerantObjective,
                960,
                false,
            )?;

            builder.add(
                pass_id::OPTIMIZE_FAULT_TOLERANCE,
                PlanStage::FaultTolerant,
                PlanReason::FaultTolerantObjective,
                970,
                false,
            )?;
        }

        // ---------------------------------------------------------------------
        // Aggressive planning
        // ---------------------------------------------------------------------

        if matches!(
            config.profile,
            OptimizationProfile::Aggressive
        ) || config.level == OptimizationLevel::O3
        {
            if movement_safe {
                builder.add(
                    pass_id::LOCAL_COMMUTATION,
                    PlanStage::Local,
                    PlanReason::AggressiveProfile,
                    400,
                    true,
                )?;

                builder.add(
                    pass_id::LOCAL_TEMPLATES,
                    PlanStage::Local,
                    PlanReason::AggressiveProfile,
                    410,
                    true,
                )?;

                builder.add(
                    pass_id::ALGEBRA_PHASE_POLYNOMIAL,
                    PlanStage::Algebraic,
                    PlanReason::AggressiveProfile,
                    530,
                    true,
                )?;
            }

            builder.add(
                pass_id::SYNTHESIS_CLIFFORD,
                PlanStage::Synthesis,
                PlanReason::AggressiveProfile,
                620,
                false,
            )?;

            builder.add(
                pass_id::SYNTHESIS_PHASE,
                PlanStage::Synthesis,
                PlanReason::AggressiveProfile,
                630,
                false,
            )?;
        }

        // ---------------------------------------------------------------------
        // Verification-oriented policy
        // ---------------------------------------------------------------------

        if matches!(
            config.profile,
            OptimizationProfile::Verified
        ) || matches!(
            config.verification.mode,
            VerificationMode::EveryRewrite
                | VerificationMode::ExhaustiveSmall
        ) {
            builder.note(
                "verification-oriented planning requested",
            );
        }

        // ---------------------------------------------------------------------
        // Target policy
        // ---------------------------------------------------------------------

        if !matches!(
            config.target,
            TargetSelection::Auto | TargetSelection::Generic
        ) {
            builder.note(
                "named target selected; target-aware passes \
                 must be resolved by the registry/catalog",
            );
        }

        // ---------------------------------------------------------------------
        // Explicit user selection
        // ---------------------------------------------------------------------

        for requested in &config.enabled_passes {
            validate_pass_identifier(requested)?;

            if config
                .disabled_passes
                .iter()
                .any(|disabled| disabled == requested)
            {
                return Err(
                    PlannerError::ConflictingPassSelection {
                        pass: requested.clone(),
                    },
                );
            }

            if !builder.contains(requested) {
                builder.add_owned(
                    requested,
                    PlanStage::Objective,
                    PlanReason::ExplicitUserSelection,
                    90_000,
                    false,
                )?;
            }
        }

        // ---------------------------------------------------------------------
        // Explicit disabling
        // ---------------------------------------------------------------------

        for disabled in &config.disabled_passes {
            validate_pass_identifier(disabled)?;
            builder.remove(disabled);
        }

        // ---------------------------------------------------------------------
        // O0 semantics
        // ---------------------------------------------------------------------

        // O0 means normalization-only by default. Explicitly selected passes
        // remain authoritative.
        if config.level == OptimizationLevel::O0 {
            builder.retain_only_normalization_parameter_and_explicit(
                &config.enabled_passes,
            );
        }

        let mut plan = builder.finish(config)?;

        // ---------------------------------------------------------------------
        // Large-circuit safety
        // ---------------------------------------------------------------------

        // Do not automatically select expensive search-heavy transformations
        // for very large circuits unless the user explicitly requested an
        // aggressive/fault-tolerant profile.
        if matches!(
            characteristics.size_class(),
            CircuitSizeClass::Large | CircuitSizeClass::Massive
        ) && !matches!(
            config.profile,
            OptimizationProfile::Aggressive
                | OptimizationProfile::FaultTolerant
        ) {
            plan.passes.retain(|pass| {
                !is_expensive_search_pass(pass.id())
                    || config
                        .enabled_passes
                        .iter()
                        .any(|id| id == pass.id())
            });
        }

        sort_and_deduplicate(&mut plan.passes);
        recompute_work_estimate(&mut plan)?;

        Ok(plan)
    }
}

// =============================================================================
// Internal plan builder
// =============================================================================

struct PlanBuilder {
    characteristics: CircuitCharacteristics,
    passes: Vec<PlannedPass>,
    notes: Vec<String>,
}

impl PlanBuilder {
    fn new(characteristics: CircuitCharacteristics) -> Self {
        Self {
            characteristics,
            passes: Vec::new(),
            notes: Vec::new(),
        }
    }

    fn add(
        &mut self,
        id: &'static str,
        stage: PlanStage,
        reason: PlanReason,
        priority: u16,
        repeatable: bool,
    ) -> PlannerResult<()> {
        self.add_owned(
            id,
            stage,
            reason,
            priority,
            repeatable,
        )
    }

    fn add_owned(
        &mut self,
        id: impl Into<String>,
        stage: PlanStage,
        reason: PlanReason,
        priority: u16,
        repeatable: bool,
    ) -> PlannerResult<()> {
        let id = id.into();

        validate_pass_identifier(&id)?;

        if self.passes.iter().any(|pass| pass.id == id) {
            return Ok(());
        }

        if self.passes.len() >= MAX_PLAN_PASSES {
            return Err(
                PlannerError::PlanPassLimitExceeded {
                    requested: self.passes.len() + 1,
                    maximum: MAX_PLAN_PASSES,
                },
            );
        }

        self.passes.push(PlannedPass {
            id,
            stage,
            reason,
            priority,
            repeatable,
        });

        Ok(())
    }

    fn remove(&mut self, id: &str) {
        self.passes.retain(|pass| pass.id != id);
    }

    fn contains(&self, id: &str) -> bool {
        self.passes.iter().any(|pass| pass.id == id)
    }

    fn note(&mut self, note: impl Into<String>) {
        if self.notes.len() < MAX_PLAN_NOTES {
            self.notes.push(note.into());
        }
    }

    fn retain_only_normalization_parameter_and_explicit(
        &mut self,
        explicit: &[String],
    ) {
        self.passes.retain(|pass| {
            matches!(
                pass.stage,
                PlanStage::Normalize | PlanStage::Parameter
            ) || explicit.iter().any(|id| id == &pass.id)
        });
    }

    fn finish(
        mut self,
        config: &OptimizationConfig,
    ) -> PlannerResult<OptimizationPlan> {
        sort_and_deduplicate(&mut self.passes);

        if self.passes.len() > MAX_PLAN_PASSES {
            return Err(
                PlannerError::PlanPassLimitExceeded {
                    requested: self.passes.len(),
                    maximum: MAX_PLAN_PASSES,
                },
            );
        }

        let deterministic = matches!(
            config.determinism,
            Determinism::Deterministic | Determinism::Seeded(_)
        );

        let seeded =
            matches!(config.determinism, Determinism::Seeded(_));

        let mut plan = OptimizationPlan {
            planner_version: PLANNER_VERSION,
            characteristics: self.characteristics,
            target: config.target.clone(),
            profile: config.profile,
            level: config.level,
            objective: config.objective,
            fixed_point: config.fixed_point,
            verification: config.verification.mode,
            deterministic,
            seeded,
            passes: self.passes,
            estimated_work_units: 0,
            notes: self.notes,
        };

        recompute_work_estimate(&mut plan)?;

        Ok(plan)
    }
}

// =============================================================================
// Circuit characterization
// =============================================================================

fn characterize(
    circuit: &QuantumCircuit,
) -> CircuitCharacteristics {
    let mut result = CircuitCharacteristics {
        qubits: circuit.num_qubits() as u128,
        operations: circuit.len() as u128,
        ..CircuitCharacteristics::default()
    };

    for gate in circuit.operations() {
        let view = OperationView::new(gate);
        let descriptor = view.descriptor();
        let properties = descriptor.properties();

        if view.is_unitary() {
            result.unitary_operations += 1;
        } else {
            result.non_unitary_operations += 1;
        }

        if view.is_multi_qubit() {
            result.multi_qubit_operations += 1;
        } else {
            result.single_qubit_operations += 1;
        }

        if view.is_measurement() {
            result.measurements += 1;
        }

        if view.is_reset() {
            result.resets += 1;
        }

        if view.is_barrier() {
            result.barriers += 1;
        }

        if view.is_clifford() {
            result.clifford_operations += 1;
        }

        if view.is_non_clifford() {
            result.non_clifford_operations += 1;
        }

        if properties.is_rotation() {
            result.rotations += 1;
        }

        if view.is_symbolic() {
            result.symbolic_operations += 1;
        }

        if view.parameter_class().is_constant() {
            result.constant_parameter_operations += 1;
        }

        if view.is_controlled() {
            result.controlled_operations += 1;
        }

        if view.is_diagonal() {
            result.diagonal_operations += 1;
        }

        if view.is_identity() {
            result.identity_operations += 1;
        }

        if view.has_classical_target() {
            result.classical_targets += 1;
        }
    }

    result
}

// =============================================================================
// Validation and limits
// =============================================================================

fn check_limits(
    characteristics: CircuitCharacteristics,
    config: &OptimizationConfig,
) -> PlannerResult<()> {
    let operations = usize_from_u128(
        characteristics.operations,
        "circuit operation count",
    )?;

    let qubits = usize_from_u128(
        characteristics.qubits,
        "circuit qubit count",
    )?;

    if operations > config.limits.max_circuit_operations {
        return Err(
            PlannerError::CircuitOperationsLimitExceeded {
                requested: operations,
                maximum: config.limits.max_circuit_operations,
            },
        );
    }

    if qubits > config.limits.max_circuit_qubits {
        return Err(
            PlannerError::CircuitQubitsLimitExceeded {
                requested: qubits,
                maximum: config.limits.max_circuit_qubits,
            },
        );
    }

    Ok(())
}

fn usize_from_u128(
    value: u128,
    calculation: &'static str,
) -> PlannerResult<usize> {
    usize::try_from(value).map_err(|_| {
        PlannerError::ArithmeticOverflow { calculation }
    })
}

fn validate_pass_identifier(
    value: &str,
) -> PlannerResult<()> {
    if value.is_empty() || value.len() > 256 {
        return Err(
            PlannerError::InvalidPassIdentifier {
                value: value.to_owned(),
            },
        );
    }

    if value.starts_with('.')
        || value.ends_with('.')
        || value.contains("..")
    {
        return Err(
            PlannerError::InvalidPassIdentifier {
                value: value.to_owned(),
            },
        );
    }

    if !value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(byte, b'.' | b'_' | b'-')
    }) {
        return Err(
            PlannerError::InvalidPassIdentifier {
                value: value.to_owned(),
            },
        );
    }

    Ok(())
}

fn target_identifier(
    target: &TargetSelection,
) -> &str {
    match target {
        TargetSelection::Auto => "auto",
        TargetSelection::Generic => "generic",
        TargetSelection::Named(name) => name.as_str(),
    }
}

// =============================================================================
// Ordering and work estimation
// =============================================================================

fn sort_and_deduplicate(
    passes: &mut Vec<PlannedPass>,
) {
    passes.sort_by(|left, right| {
        left.priority
            .cmp(&right.priority)
            .then_with(|| left.stage.cmp(&right.stage))
            .then_with(|| left.id.cmp(&right.id))
    });

    passes.dedup_by(|left, right| {
        left.id == right.id
    });
}

fn is_expensive_search_pass(id: &str) -> bool {
    matches!(
        id,
        pass_id::ALGEBRA_PHASE_POLYNOMIAL
            | pass_id::SYNTHESIS_CLIFFORD
            | pass_id::SYNTHESIS_PHASE
            | pass_id::OPTIMIZE_TWO_QUBIT
            | pass_id::OPTIMIZE_FAULT_TOLERANCE
    )
}

fn recompute_work_estimate(
    plan: &mut OptimizationPlan,
) -> PlannerResult<()> {
    let n = plan.characteristics.operations;
    let q = plan.characteristics.qubits;
    let passes = plan.passes.len() as u128;

    let mut total = n.checked_mul(passes).ok_or(
        PlannerError::ArithmeticOverflow {
            calculation: "planner linear work estimate",
        },
    )?;

    let qubit_work = q.checked_mul(passes).ok_or(
        PlannerError::ArithmeticOverflow {
            calculation: "planner qubit work estimate",
        },
    )?;

    total = total.checked_add(qubit_work).ok_or(
        PlannerError::ArithmeticOverflow {
            calculation: "planner total work estimate",
        },
    )?;

    for pass in &plan.passes {
        let multiplier: u128 =
            if is_expensive_search_pass(&pass.id) {
                8
            } else if pass.repeatable {
                2
            } else {
                1
            };

        let pass_work = n.checked_mul(multiplier).ok_or(
            PlannerError::ArithmeticOverflow {
                calculation: "planner pass work estimate",
            },
        )?;

        total = total.checked_add(pass_work).ok_or(
            PlannerError::ArithmeticOverflow {
                calculation: "planner accumulated work estimate",
            },
        )?;
    }

    plan.estimated_work_units = total;

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::QuantumCircuit;

    #[test]
    fn empty_circuit_gets_bounded_normalization_plan() {
        let circuit =
            QuantumCircuit::new(0, 0)
                .expect("empty circuit must be valid");

        let config = OptimizationConfig::o0();

        let plan = OptimizationPlanner::new()
            .plan(&circuit, &config)
            .expect("planning must succeed");

        assert!(
            plan.contains(pass_id::NORMALIZE_CANONICAL)
        );

        assert!(
            plan.passes().len() <= MAX_PLAN_PASSES
        );
    }

    #[test]
    fn planning_is_deterministic() {
        let circuit =
            QuantumCircuit::new(4, 0)
                .expect("circuit must be valid");

        let config =
            OptimizationConfig::balanced();

        let planner =
            OptimizationPlanner::new();

        let first = planner
            .plan(&circuit, &config)
            .expect("first plan");

        let second = planner
            .plan(&circuit, &config)
            .expect("second plan");

        assert_eq!(first, second);
    }

    #[test]
    fn explicit_disabled_pass_is_removed() {
        let circuit =
            QuantumCircuit::new(2, 0)
                .expect("circuit must be valid");

        let mut config =
            OptimizationConfig::balanced();

        config
            .disabled_passes
            .push(
                pass_id::LOCAL_CANCELLATION
                    .to_owned(),
            );

        let plan = OptimizationPlanner::new()
            .plan(&circuit, &config)
            .expect("planning must succeed");

        assert!(
            !plan.contains(
                pass_id::LOCAL_CANCELLATION
            )
        );
    }

    #[test]
    fn invalid_pass_identifier_is_rejected() {
        let circuit =
            QuantumCircuit::new(1, 0)
                .expect("circuit must be valid");

        let mut config =
            OptimizationConfig::balanced();

        config
            .enabled_passes
            .push("bad pass id".to_owned());

        let error = OptimizationPlanner::new()
            .plan(&circuit, &config)
            .expect_err(
                "invalid id must fail",
            );

        assert!(
            matches!(
                error,
                PlannerError::InvalidConfiguration { .. }
            )
        );
    }

    #[test]
    fn explicit_unknown_pass_remains_extensible() {
        let circuit =
            QuantumCircuit::new(1, 0)
                .expect("circuit must be valid");

        let mut config =
            OptimizationConfig::balanced();

        config
            .enabled_passes
            .push(
                "future.domain.pass"
                    .to_owned(),
            );

        let plan = OptimizationPlanner::new()
            .plan(&circuit, &config)
            .expect(
                "unknown future pass must be allowed \
                 without a registry",
            );

        assert!(
            plan.contains("future.domain.pass")
        );
    }
}