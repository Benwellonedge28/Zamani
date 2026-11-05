
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Optimization Planning.
//!
//! Path:
//!     src/compiler/optimization/planning.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Construct deterministic, auditable, legality-aware optimization plans
//! for Zamani programs and their canonical intermediate representations.
//!
//! This module plans optimization. It does not implement or execute
//! optimization transformations.
//!
//! ARCHITECTURAL AUTHORITIES
//!
//! Canonical pass contracts:
//!     crate::zuir::pass::contracts
//!
//! Pass registration:
//!     crate::zuir::pass::registry
//!
//! Pass execution:
//!     crate::zuir::pass::manager
//!
//! Compiler-facing pass integration:
//!     crate::compiler::passes
//!
//! Compiler configuration and budgets:
//!     crate::compiler::configuration
//!
//! Compiler analysis:
//!     crate::compiler::analysis
//!
//! Canonical IR integration:
//!     crate::compiler::ir
//!
//! Classical optimization:
//!     crate::optimizer
//!
//! Domain-specific optimization:
//!     Authoritative domain subsystems.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - Optimization candidate descriptions.
//! - Transformation planning contracts.
//! - Optimization planning context.
//! - Optimization legality assessment at planning level.
//! - Cost and profitability evidence.
//! - Dependency-aware plan construction.
//! - Deterministic plan ordering.
//! - Structured planning decisions.
//! - Plan validation.
//!
//! This module does not own:
//! - IR definitions.
//! - Pass identities or canonical pass descriptors.
//! - Pass registration.
//! - Pass execution.
//! - Optimization algorithms.
//! - Semantic analysis.
//! - Ownership or effect semantics.
//! - IR verification algorithms.
//! - Physical hardware discovery.
//! - Hardware allocation.
//! - Runtime scheduling.
//! - Physical routing.
//! - Backend code generation.
//!
//! SCALABILITY
//!
//! No universal limits are imposed on:
//! - Number of optimization passes.
//! - Number of IR operations.
//! - Number of computational domains.
//! - Number of analyses.
//! - Number of resources.
//! - Number of targets.
//! - Number of dependencies.
//!
//! Actual memory and processing constraints remain applicable.
//!
//! Configured budgets are operational restrictions, not language limits.
//!
//! IMPORTANT
//!
//! A plan is not proof of semantic preservation.
//! A pass declaration is not proof of implementation correctness.
//! A cost estimate is not a performance guarantee.
//! A feasible optimization plan is not an execution guarantee.
//!
//! All transformations must pass the authoritative verification
//! requirements before their results are accepted.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;

use crate::zuir::pass::contracts::{
    PassDescriptor,
    PassId,
    PassVersion,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Stable public optimization-planning API version.
pub const OPTIMIZATION_PLANNING_API_VERSION: u16 = 1;

// ============================================================================
// COMMON TYPES
// ============================================================================

/// Result type for optimization planning.
pub type PlanningResult<T> = Result<T, PlanningError>;

/// Stable identifier for a representation or dialect.
///
/// Examples:
/// - `zuir.core.v1`
/// - `classical.zcir.v1`
/// - `quantum.ir.v1`
/// - `hdl.hdlir.v1`
/// - `ai.tensor.v1`
///
/// This is an extensible identifier, not a closed domain enumeration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RepresentationId(String);

impl RepresentationId {
    /// Creates a representation identifier.
    pub fn new(value: impl Into<String>) -> PlanningResult<Self> {
        let value = value.into();

        validate_identifier(&value, "representation")?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Extensible analysis identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnalysisId(String);

impl AnalysisId {
    /// Creates an analysis identifier.
    pub fn new(value: impl Into<String>) -> PlanningResult<Self> {
        let value = value.into();

        validate_identifier(&value, "analysis")?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Extensible effect identifier.
///
/// Effect meaning remains owned by the semantic and effect subsystems.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EffectId(String);

impl EffectId {
    /// Creates an effect identifier.
    pub fn new(value: impl Into<String>) -> PlanningResult<Self> {
        let value = value.into();

        validate_identifier(&value, "effect")?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Extensible resource-dimension identifier.
///
/// This identifies a resource dimension without assigning it a universal
/// physical capacity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(String);

impl ResourceId {
    /// Creates a resource identifier.
    pub fn new(value: impl Into<String>) -> PlanningResult<Self> {
        let value = value.into();

        validate_identifier(&value, "resource")?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validates an extensible identifier.
///
/// No arbitrary string-length ceiling is imposed.
fn validate_identifier(
    value: &str,
    kind: &str,
) -> PlanningResult<()> {
    if value.trim().is_empty() {
        return Err(PlanningError::InvalidIdentifier {
            kind: kind.to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(PlanningError::InvalidIdentifier {
            kind: kind.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// OPTIMIZATION OBJECTIVES
// ============================================================================

/// Optimization objective.
///
/// Objective names are extensible because future computing domains
/// may introduce new optimization objectives.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OptimizationObjective(String);

impl OptimizationObjective {
    /// Creates an objective.
    pub fn new(value: impl Into<String>) -> PlanningResult<Self> {
        let value = value.into();

        validate_identifier(&value, "optimization objective")?;

        Ok(Self(value))
    }

    /// Returns the objective name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// PRESERVATION CONTRACT
// ============================================================================

/// Declares the preservation properties required from a transformation.
///
/// These declarations are obligations, not proofs.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PreservationContract {
    /// Required semantic properties.
    pub semantic_properties: BTreeSet<String>,

    /// Required control-flow properties.
    pub control_flow_properties: BTreeSet<String>,

    /// Required data-flow properties.
    pub data_flow_properties: BTreeSet<String>,

    /// Required ownership properties.
    pub ownership_properties: BTreeSet<String>,

    /// Required observable behaviour.
    pub observable_behaviour: BTreeSet<String>,

    /// Whether the transformation must preserve the program's
    /// externally observable behaviour.
    pub preserve_observable_behaviour: bool,

    /// Whether the transformation must preserve ordering guarantees.
    pub preserve_ordering: bool,

    /// Whether the transformation must preserve numerical guarantees.
    pub preserve_numerical_guarantees: bool,

    /// Whether the transformation must preserve resource semantics.
    pub preserve_resource_semantics: bool,

    /// Whether the transformation must preserve effect semantics.
    pub preserve_effects: bool,
}

impl PreservationContract {
    /// Validates preservation metadata.
    pub fn validate(&self) -> PlanningResult<()> {
        validate_string_set(&self.semantic_properties)?;
        validate_string_set(&self.control_flow_properties)?;
        validate_string_set(&self.data_flow_properties)?;
        validate_string_set(&self.ownership_properties)?;
        validate_string_set(&self.observable_behaviour)?;

        Ok(())
    }

    /// Returns whether the contract declares any preservation obligations.
    pub fn is_declared(&self) -> bool {
        self.preserve_observable_behaviour
            || self.preserve_ordering
            || self.preserve_numerical_guarantees
            || self.preserve_resource_semantics
            || self.preserve_effects
            || !self.semantic_properties.is_empty()
            || !self.control_flow_properties.is_empty()
            || !self.data_flow_properties.is_empty()
            || !self.ownership_properties.is_empty()
            || !self.observable_behaviour.is_empty()
    }
}

// ============================================================================
// TRANSFORMATION CONTRACT
// ============================================================================

/// Declares the complete planning contract for one transformation.
///
/// The actual transformation is registered and executed by the canonical
/// pass infrastructure.
///
/// This type adds planning information without replacing PassDescriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformationContract {
    /// Input representation.
    pub input: RepresentationId,

    /// Output representation.
    pub output: RepresentationId,

    /// Required analyses.
    pub required_analyses: BTreeSet<AnalysisId>,

    /// Analyses invalidated by the transformation.
    pub invalidated_analyses: BTreeSet<AnalysisId>,

    /// Preconditions required for applicability.
    pub preconditions: BTreeSet<String>,

    /// Semantic preservation obligations.
    pub preservation: PreservationContract,

    /// Effects that the transformation may interact with.
    pub effects: BTreeSet<EffectId>,

    /// Abstract resource requirements.
    pub resource_requirements: BTreeMap<ResourceId, u128>,

    /// Other passes that must execute first.
    pub dependencies: BTreeSet<String>,

    /// Whether this transformation may approximate results.
    pub approximate: bool,

    /// Whether this transformation may use stochastic behaviour.
    pub stochastic: bool,

    /// Whether this transformation supports deterministic execution.
    pub deterministic: bool,

    /// Whether successful execution requires verified input.
    pub requires_verified_input: bool,

    /// Whether successful execution requires verified output.
    pub requires_verified_output: bool,
}

impl TransformationContract {
    /// Creates a transformation contract.
    pub fn new(
        input: RepresentationId,
        output: RepresentationId,
    ) -> Self {
        Self {
            input,
            output,
            required_analyses: BTreeSet::new(),
            invalidated_analyses: BTreeSet::new(),
            preconditions: BTreeSet::new(),
            preservation: PreservationContract::default(),
            effects: BTreeSet::new(),
            resource_requirements: BTreeMap::new(),
            dependencies: BTreeSet::new(),
            approximate: false,
            stochastic: false,
            deterministic: true,
            requires_verified_input: true,
            requires_verified_output: true,
        }
    }

    /// Validates the transformation contract.
    pub fn validate(&self) -> PlanningResult<()> {
        self.preservation.validate()?;

        validate_string_set(&self.preconditions)?;
        validate_string_set(&self.dependencies)?;

        for dependency in &self.dependencies {
            validate_identifier(dependency, "pass dependency")?;
        }

        for (resource, _) in &self.resource_requirements {
            validate_identifier(
                resource.as_str(),
                "resource requirement",
            )?;
        }

        if self.deterministic && self.stochastic {
            return Err(PlanningError::ContradictoryContract {
                reason: "a transformation cannot simultaneously declare \
                         deterministic and stochastic behaviour"
                    .to_owned(),
            });
        }

        Ok(())
    }
}

fn validate_string_set(
    values: &BTreeSet<String>,
) -> PlanningResult<()> {
    for value in values {
        validate_identifier(value, "contract property")?;
    }

    Ok(())
}

// ============================================================================
// COST MODEL
// ============================================================================

/// Cost dimension.
///
/// Names are extensible rather than tied to one machine architecture.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CostDimension(String);

impl CostDimension {
    /// Creates a cost dimension.
    pub fn new(value: impl Into<String>) -> PlanningResult<Self> {
        let value = value.into();

        validate_identifier(&value, "cost dimension")?;

        Ok(Self(value))
    }

    /// Returns the dimension name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Estimated cost of an optimization transformation.
///
/// All estimates are non-negative and finite.
///
/// These estimates are informational. They do not establish measured
/// execution performance.
#[derive(Debug, Clone, PartialEq)]
pub struct CostEstimate {
    /// Estimated cost per dimension.
    pub costs: BTreeMap<CostDimension, f64>,

    /// Expected improvement, if known.
    ///
    /// The interpretation depends on the selected objective.
    pub expected_benefit: Option<f64>,

    /// Confidence in the estimate, between zero and one.
    pub confidence: Option<f64>,

    /// Identity of the model that produced the estimate.
    pub model_id: Option<String>,
}

impl Default for CostEstimate {
    fn default() -> Self {
        Self {
            costs: BTreeMap::new(),
            expected_benefit: None,
            confidence: None,
            model_id: None,
        }
    }
}

impl CostEstimate {
    /// Validates cost estimates.
    pub fn validate(&self) -> PlanningResult<()> {
        for value in self.costs.values() {
            validate_non_negative_finite(*value, "cost")?;
        }

        if let Some(value) = self.expected_benefit {
            validate_non_negative_finite(value, "expected benefit")?;
        }

        if let Some(value) = self.confidence {
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                return Err(PlanningError::InvalidCostEstimate {
                    reason: "confidence must be between zero and one"
                        .to_owned(),
                });
            }
        }

        if let Some(model) = &self.model_id {
            validate_identifier(model, "cost model")?;
        }

        Ok(())
    }
}

fn validate_non_negative_finite(
    value: f64,
    field: &str,
) -> PlanningResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(PlanningError::InvalidCostEstimate {
            reason: format!("{field} must be finite and non-negative"),
        });
    }

    Ok(())
}

// ============================================================================
// COST MODEL INTERFACE
// ============================================================================

/// Interface for external optimization cost models.
///
/// The compiler does not own the model implementation.
///
/// A model may use static analysis, profiling, target characteristics,
/// mathematical models, or other approved evidence.
pub trait OptimizationCostModel {
    /// Returns the model's stable identifier.
    fn model_id(&self) -> &str;

    /// Estimates the cost of one transformation.
    fn estimate(
        &self,
        pass_id: &PassId,
        contract: &TransformationContract,
        objective: &OptimizationObjective,
    ) -> PlanningResult<CostEstimate>;
}

// ============================================================================
// OPTIMIZATION CANDIDATE
// ============================================================================

/// One registered optimization candidate.
///
/// A candidate combines the canonical pass descriptor with its additional
/// planning contract.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationCandidate {
    /// Canonical pass descriptor.
    pub descriptor: PassDescriptor,

    /// Additional transformation planning contract.
    pub contract: TransformationContract,

    /// Optional cost estimate.
    pub cost: CostEstimate,

    /// Explicit candidate priority.
    ///
    /// Higher values indicate stronger preference when otherwise
    /// equivalent candidates are ordered.
    pub priority: i64,
}

impl OptimizationCandidate {
    /// Creates a candidate.
    pub fn new(
        descriptor: PassDescriptor,
        contract: TransformationContract,
    ) -> Self {
        Self {
            descriptor,
            contract,
            cost: CostEstimate::default(),
            priority: 0,
        }
    }

    /// Returns the stable pass identifier.
    pub fn pass_id(&self) -> &str {
        self.descriptor.id.as_str()
    }

    /// Validates candidate metadata.
    pub fn validate(&self) -> PlanningResult<()> {
        self.descriptor
            .validate()
            .map_err(|error| PlanningError::InvalidPass {
                pass: self.pass_id().to_owned(),
                reason: error.to_string(),
            })?;

        self.contract.validate()?;
        self.cost.validate()?;

        if self.contract.deterministic && !self.descriptor.deterministic {
            return Err(PlanningError::ContradictoryContract {
                reason: format!(
                    "pass {} declares deterministic execution in its \
                     transformation contract but not in its canonical \
                     pass descriptor",
                    self.pass_id()
                ),
            });
        }

        if self.contract.requires_verified_input
            && !self.descriptor.requires_verified_input
        {
            return Err(PlanningError::ContradictoryContract {
                reason: format!(
                    "pass {} has inconsistent verified-input declarations",
                    self.pass_id()
                ),
            });
        }

        if self.contract.requires_verified_output
            && !self.descriptor.verifies_output
        {
            return Err(PlanningError::ContradictoryContract {
                reason: format!(
                    "pass {} has inconsistent verified-output declarations",
                    self.pass_id()
                ),
            });
        }

        Ok(())
    }
}

// ============================================================================
// PLANNING EVIDENCE
// ============================================================================

/// Evidence supplied by authoritative analysis subsystems.
///
/// This is evidence, not a replacement for semantic analysis.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlanningEvidence {
    /// Analyses known to be available and valid.
    pub available_analyses: BTreeSet<AnalysisId>,

    /// Preconditions known to hold.
    pub established_preconditions: BTreeSet<String>,

    /// Available resource quantities.
    ///
    /// Absence means unknown, not zero.
    pub available_resources: BTreeMap<ResourceId, u128>,

    /// Whether input IR has been successfully verified.
    pub input_verified: bool,

    /// Whether effect information is complete.
    pub effects_complete: bool,

    /// Whether semantic preservation evidence is available.
    pub preservation_evidence_available: bool,
}

// ============================================================================
// PLANNING POLICY
// ============================================================================

/// Policy governing optimization-plan construction.
///
/// These are compiler policies, not source-language semantic authorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizationPlanningPolicy {
    /// Whether nondeterministic transformations are allowed.
    pub allow_nondeterministic: bool,

    /// Whether stochastic transformations are allowed.
    pub allow_stochastic: bool,

    /// Whether approximate transformations are allowed.
    pub allow_approximate: bool,

    /// Whether complete effect information is mandatory.
    pub require_complete_effects: bool,

    /// Whether preservation evidence is mandatory.
    pub require_preservation_evidence: bool,

    /// Whether verified input is mandatory.
    pub require_verified_input: bool,

    /// Whether verified output is mandatory.
    pub require_verified_output: bool,

    /// Whether every candidate must have a cost estimate.
    pub require_cost_estimate: bool,

    /// Optional resource budgets.
    ///
    /// A missing entry means this policy imposes no explicit budget
    /// for that resource.
    pub resource_budgets: BTreeMap<ResourceId, u128>,
}

impl Default for OptimizationPlanningPolicy {
    fn default() -> Self {
        Self {
            allow_nondeterministic: false,
            allow_stochastic: false,
            allow_approximate: false,
            require_complete_effects: true,
            require_preservation_evidence: true,
            require_verified_input: true,
            require_verified_output: true,
            require_cost_estimate: false,
            resource_budgets: BTreeMap::new(),
        }
    }
}

// ============================================================================
// PLANNING CONTEXT
// ============================================================================

/// Complete input to optimization planning.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationPlanningContext {
    /// Selected optimization objective.
    pub objective: OptimizationObjective,

    /// Planning policy.
    pub policy: OptimizationPlanningPolicy,

    /// Authoritative evidence.
    pub evidence: PlanningEvidence,

    /// Optional external cost model identifier.
    pub cost_model_id: Option<String>,

    /// Optional compilation-profile identity.
    pub profile_id: Option<String>,

    /// Whether the plan must be reproducible.
    pub require_reproducible_plan: bool,
}

impl OptimizationPlanningContext {
    /// Creates a conservative planning context.
    pub fn new(objective: OptimizationObjective) -> Self {
        Self {
            objective,
            policy: OptimizationPlanningPolicy::default(),
            evidence: PlanningEvidence::default(),
            cost_model_id: None,
            profile_id: None,
            require_reproducible_plan: true,
        }
    }

    /// Validates the context.
    pub fn validate(&self) -> PlanningResult<()> {
        if let Some(value) = &self.cost_model_id {
            validate_identifier(value, "cost model")?;
        }

        if let Some(value) = &self.profile_id {
            validate_identifier(value, "compilation profile")?;
        }

        validate_string_set(&self.evidence.established_preconditions)?;

        Ok(())
    }
}

// ============================================================================
// PLANNING DECISIONS
// ============================================================================

/// Why an optimization candidate was accepted or rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanningDecision {
    /// Candidate was accepted.
    Accepted,

    /// One or more analyses are unavailable.
    MissingAnalyses(Vec<String>),

    /// Required preconditions are unavailable.
    MissingPreconditions(Vec<String>),

    /// Input IR has not been verified.
    UnverifiedInput,

    /// Required effect information is incomplete.
    IncompleteEffects,

    /// Preservation evidence is unavailable.
    MissingPreservationEvidence,

    /// Candidate requires a resource beyond the configured budget.
    ResourceBudgetExceeded {
        resource: String,
        required: u128,
        available: u128,
    },

    /// Candidate requires a resource for which availability is unknown.
    UnknownResourceAvailability {
        resource: String,
        required: u128,
    },

    /// Candidate is nondeterministic under a reproducibility policy.
    Nondeterministic,

    /// Candidate uses stochastic behaviour without permission.
    StochasticNotAllowed,

    /// Candidate uses approximation without permission.
    ApproximationNotAllowed,

    /// Required cost information is missing.
    MissingCostEstimate,

    /// Candidate was rejected for another explicit reason.
    Rejected(String),
}

// ============================================================================
// PLAN STEP
// ============================================================================

/// One approved optimization step in a plan.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationPlanStep {
    /// Canonical pass identity.
    pub pass_id: PassId,

    /// Canonical pass version.
    pub pass_version: PassVersion,

    /// Transformation contract.
    pub contract: TransformationContract,

    /// Cost estimate.
    pub cost: CostEstimate,

    /// Priority.
    pub priority: i64,
}

// ============================================================================
// PLAN
// ============================================================================

/// Immutable optimization plan.
///
/// This object describes the approved transformation order.
///
/// It does not execute transformations.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationPlan {
    /// Planning API version.
    pub api_version: u16,

    /// Optimization objective.
    pub objective: OptimizationObjective,

    /// Ordered transformations.
    pub steps: Vec<OptimizationPlanStep>,

    /// Decisions for all considered candidates.
    pub decisions: BTreeMap<String, PlanningDecision>,

    /// Cost-model identity.
    pub cost_model_id: Option<String>,

    /// Compilation-profile identity.
    pub profile_id: Option<String>,

    /// Whether the plan is reproducible under its declared inputs.
    pub reproducible: bool,
}

impl OptimizationPlan {
    /// Returns the number of accepted transformations.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Returns whether the plan contains no transformations.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Returns an iterator over approved pass IDs.
    pub fn pass_ids(&self) -> impl Iterator<Item = &PassId> {
        self.steps.iter().map(|step| &step.pass_id)
    }

    /// Validates the final plan structure.
    pub fn validate(&self) -> PlanningResult<()> {
        let mut seen = BTreeSet::new();

        for step in &self.steps {
            let id = step.pass_id.as_str().to_owned();

            if !seen.insert(id.clone()) {
                return Err(PlanningError::DuplicatePass {
                    pass: id,
                });
            }

            step.contract.validate()?;
            step.cost.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// PLANNER
// ============================================================================

/// Stateless optimization planner.
///
/// It consumes registered candidates and authoritative analysis evidence.
///
/// It does not execute transformations or mutate IR.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptimizationPlanner;

impl OptimizationPlanner {
    /// Creates a planner.
    pub fn new() -> Self {
        Self
    }

    /// Builds an optimization plan.
    ///
    /// Candidate ordering is deterministic.
    ///
    /// Missing evidence never becomes implicit success.
    pub fn build_plan(
        &self,
        context: &OptimizationPlanningContext,
        candidates: impl IntoIterator<Item = OptimizationCandidate>,
    ) -> PlanningResult<OptimizationPlan> {
        context.validate()?;

        let mut candidates_by_id = BTreeMap::new();

        for candidate in candidates {
            candidate.validate()?;

            let id = candidate.pass_id().to_owned();

            if candidates_by_id.insert(id.clone(), candidate).is_some() {
                return Err(PlanningError::DuplicatePass {
                    pass: id,
                });
            }
        }

        let mut decisions = BTreeMap::new();
        let mut eligible = BTreeMap::new();

        for (id, candidate) in candidates_by_id {
            let decision = self.assess_candidate(context, &candidate);

            if decision == PlanningDecision::Accepted {
                eligible.insert(id.clone(), candidate);
            }

            decisions.insert(id, decision);
        }

        // A candidate whose dependency was rejected cannot be executed.
        // Reject it explicitly rather than silently omitting the dependency.
        let mut changed = true;

        while changed {
            changed = false;

            let ids: Vec<String> = eligible.keys().cloned().collect();

            for id in ids {
                let candidate = eligible
                    .get(&id)
                    .expect("candidate was obtained from eligible map");

                let missing: Vec<String> = candidate
                    .contract
                    .dependencies
                    .iter()
                    .filter(|dependency| !eligible.contains_key(*dependency))
                    .cloned()
                    .collect();

                if !missing.is_empty() {
                    eligible.remove(&id);

                    decisions.insert(
                        id,
                        PlanningDecision::Rejected(format!(
                            "required dependencies are unavailable: {}",
                            missing.join(", ")
                        )),
                    );

                    changed = true;
                }
            }
        }

        let ordered = topological_order(&eligible)?;

        let steps = ordered
            .into_iter()
            .filter_map(|id| eligible.get(&id))
            .map(|candidate| OptimizationPlanStep {
                pass_id: candidate.descriptor.id.clone(),
                pass_version: candidate.descriptor.version,
                contract: candidate.contract.clone(),
                cost: candidate.cost.clone(),
                priority: candidate.priority,
            })
            .collect::<Vec<_>>();

        let reproducible = context.require_reproducible_plan
            && steps.iter().all(|step| {
                step.contract.deterministic
                    && !step.contract.stochastic
            });

        if context.require_reproducible_plan && !reproducible {
            return Err(PlanningError::NonReproduciblePlan);
        }

        let plan = OptimizationPlan {
            api_version: OPTIMIZATION_PLANNING_API_VERSION,
            objective: context.objective.clone(),
            steps,
            decisions,
            cost_model_id: context.cost_model_id.clone(),
            profile_id: context.profile_id.clone(),
            reproducible,
        };

        plan.validate()?;

        Ok(plan)
    }

    /// Assesses whether one candidate is eligible.
    pub fn assess_candidate(
        &self,
        context: &OptimizationPlanningContext,
        candidate: &OptimizationCandidate,
    ) -> PlanningDecision {
        let contract = &candidate.contract;
        let evidence = &context.evidence;
        let policy = &context.policy;

        let missing_analyses: Vec<String> = contract
            .required_analyses
            .iter()
            .filter(|analysis| {
                !evidence.available_analyses.contains(*analysis)
            })
            .map(|analysis| analysis.as_str().to_owned())
            .collect();

        if !missing_analyses.is_empty() {
            return PlanningDecision::MissingAnalyses(missing_analyses);
        }

        let missing_preconditions: Vec<String> = contract
            .preconditions
            .iter()
            .filter(|condition| {
                !evidence
                    .established_preconditions
                    .contains(*condition)
            })
            .cloned()
            .collect();

        if !missing_preconditions.is_empty() {
            return PlanningDecision::MissingPreconditions(
                missing_preconditions,
            );
        }

        if (policy.require_verified_input
            || contract.requires_verified_input)
            && !evidence.input_verified
        {
            return PlanningDecision::UnverifiedInput;
        }

        if policy.require_complete_effects
            && !evidence.effects_complete
        {
            return PlanningDecision::IncompleteEffects;
        }

        if policy.require_preservation_evidence
            && !evidence.preservation_evidence_available
        {
            return PlanningDecision::MissingPreservationEvidence;
        }

        if contract.approximate && !policy.allow_approximate {
            return PlanningDecision::ApproximationNotAllowed;
        }

        if contract.stochastic && !policy.allow_stochastic {
            return PlanningDecision::StochasticNotAllowed;
        }

        if (!contract.deterministic || contract.stochastic)
            && (!policy.allow_nondeterministic
                || context.require_reproducible_plan)
        {
            return PlanningDecision::Nondeterministic;
        }

        if policy.require_cost_estimate
            && candidate.cost.costs.is_empty()
            && candidate.cost.expected_benefit.is_none()
        {
            return PlanningDecision::MissingCostEstimate;
        }

        for (resource, required) in &contract.resource_requirements {
            if let Some(budget) = policy.resource_budgets.get(resource) {
                if required > budget {
                    return PlanningDecision::ResourceBudgetExceeded {
                        resource: resource.as_str().to_owned(),
                        required: *required,
                        available: *budget,
                    };
                }
            }

            if let Some(available) =
                evidence.available_resources.get(resource)
            {
                if required > available {
                    return PlanningDecision::ResourceBudgetExceeded {
                        resource: resource.as_str().to_owned(),
                        required: *required,
                        available: *available,
                    };
                }
            } else if !contract.resource_requirements.is_empty() {
                return PlanningDecision::UnknownResourceAvailability {
                    resource: resource.as_str().to_owned(),
                    required: *required,
                };
            }
        }

        PlanningDecision::Accepted
    }
}

// ============================================================================
// DETERMINISTIC DEPENDENCY ORDERING
// ============================================================================

/// Returns a deterministic topological ordering of eligible passes.
///
/// Dependencies are expressed using canonical pass-ID strings.
///
/// Independent passes are ordered by:
/// - Higher explicit priority first.
/// - Lexicographical pass identity as the stable tie-breaker.
///
/// Cycles are rejected.
fn topological_order(
    candidates: &BTreeMap<String, OptimizationCandidate>,
) -> PlanningResult<Vec<String>> {
    let mut indegree = BTreeMap::<String, usize>::new();
    let mut dependents = BTreeMap::<String, BTreeSet<String>>::new();

    for id in candidates.keys() {
        indegree.insert(id.clone(), 0);
        dependents.insert(id.clone(), BTreeSet::new());
    }

    for (id, candidate) in candidates {
        for dependency in &candidate.contract.dependencies {
            if !candidates.contains_key(dependency) {
                return Err(PlanningError::MissingDependency {
                    pass: id.clone(),
                    dependency: dependency.clone(),
                });
            }

            *indegree
                .get_mut(id)
                .expect("candidate indegree exists") += 1;

            dependents
                .get_mut(dependency)
                .expect("dependency node exists")
                .insert(id.clone());
        }
    }

    let mut ready = BTreeSet::<(i64, String)>::new();

    for (id, degree) in &indegree {
        if *degree == 0 {
            let priority = candidates
                .get(id)
                .expect("candidate exists")
                .priority;

            ready.insert((-priority, id.clone()));
        }
    }

    let mut ordered = Vec::with_capacity(candidates.len());

    while let Some((priority, id)) = ready.iter().next().cloned() {
        ready.remove(&(priority, id.clone()));
        ordered.push(id.clone());

        for dependent in dependents
            .get(&id)
            .expect("dependent collection exists")
        {
            let degree = indegree
                .get_mut(dependent)
                .expect("dependent indegree exists");

            *degree -= 1;

            if *degree == 0 {
                let dependent_priority = candidates
                    .get(dependent)
                    .expect("dependent candidate exists")
                    .priority;

                ready.insert((
                    -dependent_priority,
                    dependent.clone(),
                ));
            }
        }
    }

    if ordered.len() != candidates.len() {
        let cyclic: Vec<String> = indegree
            .iter()
            .filter(|(_, degree)| **degree > 0)
            .map(|(id, _)| id.clone())
            .collect();

        return Err(PlanningError::DependencyCycle {
            passes: cyclic,
        });
    }

    Ok(ordered)
}

// ============================================================================
// PLANNING ERRORS
// ============================================================================

/// Optimization planning errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanningError {
    /// Invalid identifier.
    InvalidIdentifier {
        /// Identifier category.
        kind: String,
    },

    /// Invalid candidate.
    InvalidPass {
        /// Pass identity.
        pass: String,

        /// Explanation.
        reason: String,
    },

    /// Duplicate pass.
    DuplicatePass {
        /// Duplicate identity.
        pass: String,
    },

    /// Invalid cost estimate.
    InvalidCostEstimate {
        /// Explanation.
        reason: String,
    },

    /// Contradictory transformation contract.
    ContradictoryContract {
        /// Explanation.
        reason: String,
    },

    /// Missing dependency.
    MissingDependency {
        /// Dependent pass.
        pass: String,

        /// Missing prerequisite.
        dependency: String,
    },

    /// Cyclic dependencies.
    DependencyCycle {
        /// Involved pass identities.
        passes: Vec<String>,
    },

    /// Plan cannot satisfy reproducibility policy.
    NonReproduciblePlan,
}

impl fmt::Display for PlanningError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind } => {
                write!(formatter, "invalid {kind} identifier")
            }

            Self::InvalidPass { pass, reason } => {
                write!(formatter, "invalid pass {pass}: {reason}")
            }

            Self::DuplicatePass { pass } => {
                write!(formatter, "duplicate optimization pass: {pass}")
            }

            Self::InvalidCostEstimate { reason } => {
                write!(formatter, "invalid cost estimate: {reason}")
            }

            Self::ContradictoryContract { reason } => {
                write!(formatter, "contradictory transformation contract: {reason}")
            }

            Self::MissingDependency { pass, dependency } => {
                write!(
                    formatter,
                    "pass {pass} requires unavailable dependency {dependency}"
                )
            }

            Self::DependencyCycle { passes } => {
                write!(
                    formatter,
                    "optimization dependency cycle: {}",
                    passes.join(", ")
                )
            }

            Self::NonReproduciblePlan => {
                formatter.write_str(
                    "optimization plan violates reproducibility policy",
                )
            }
        }
    }
}

impl Error for PlanningError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor(name: &str) -> PassDescriptor {
        PassDescriptor::new(
            PassId::new(name).expect("valid pass ID"),
            PassVersion::new(1, 0, 0),
            "Optimization planning test pass",
        )
    }

    fn candidate(name: &str) -> OptimizationCandidate {
        let mut contract = TransformationContract::new(
            RepresentationId::new("zuir.core.v1").unwrap(),
            RepresentationId::new("zuir.core.v1").unwrap(),
        );

        contract.preservation.preserve_observable_behaviour = true;
        contract.preservation.preserve_effects = true;

        OptimizationCandidate::new(
            descriptor(name),
            contract,
        )
    }

    fn context() -> OptimizationPlanningContext {
        let mut context = OptimizationPlanningContext::new(
            OptimizationObjective::new("performance").unwrap(),
        );

        context.evidence.input_verified = true;
        context.evidence.effects_complete = true;
        context.evidence.preservation_evidence_available = true;

        context
    }

    #[test]
    fn accepts_valid_candidate() {
        let planner = OptimizationPlanner::new();

        let plan = planner
            .build_plan(&context(), [candidate("core.fold")])
            .unwrap();

        assert_eq!(plan.len(), 1);
        assert!(plan.reproducible);
    }

    #[test]
    fn rejects_unverified_input() {
        let planner = OptimizationPlanner::new();
        let mut context = context();

        context.evidence.input_verified = false;

        let decision = planner.assess_candidate(
            &context,
            &candidate("core.fold"),
        );

        assert_eq!(
            decision,
            PlanningDecision::UnverifiedInput
        );
    }

    #[test]
    fn rejects_incomplete_effect_information() {
        let planner = OptimizationPlanner::new();
        let mut context = context();

        context.evidence.effects_complete = false;

        let decision = planner.assess_candidate(
            &context,
            &candidate("core.fold"),
        );

        assert_eq!(
            decision,
            PlanningDecision::IncompleteEffects
        );
    }

    #[test]
    fn rejects_approximation_without_permission() {
        let planner = OptimizationPlanner::new();
        let mut context = context();

        let mut candidate = candidate("core.approximate");
        candidate.contract.approximate = true;

        assert_eq!(
            planner.assess_candidate(&context, &candidate),
            PlanningDecision::ApproximationNotAllowed
        );

        context.policy.allow_approximate = true;

        assert_eq!(
            planner.assess_candidate(&context, &candidate),
            PlanningDecision::Accepted
        );
    }

    #[test]
    fn rejects_missing_analysis() {
        let planner = OptimizationPlanner::new();
        let mut candidate = candidate("core.fold");

        candidate.contract.required_analyses.insert(
            AnalysisId::new("core.constant_facts").unwrap(),
        );

        let decision = planner.assess_candidate(
            &context(),
            &candidate,
        );

        assert!(matches!(
            decision,
            PlanningDecision::MissingAnalyses(_)
        ));
    }

    #[test]
    fn rejects_missing_preconditions() {
        let planner = OptimizationPlanner::new();
        let mut candidate = candidate("core.fold");

        candidate.contract.preconditions.insert(
            "core.constants_known".to_owned(),
        );

        let decision = planner.assess_candidate(
            &context(),
            &candidate,
        );

        assert!(matches!(
            decision,
            PlanningDecision::MissingPreconditions(_)
        ));
    }

    #[test]
    fn respects_resource_budgets() {
        let planner = OptimizationPlanner::new();
        let mut context = context();

        let resource = ResourceId::new("compiler.memory").unwrap();

        context.policy.resource_budgets.insert(
            resource.clone(),
            100,
        );

        context.evidence.available_resources.insert(
            resource.clone(),
            100,
        );

        let mut candidate = candidate("core.fold");

        candidate.contract.resource_requirements.insert(
            resource,
            101,
        );

        assert!(matches!(
            planner.assess_candidate(&context, &candidate),
            PlanningDecision::ResourceBudgetExceeded { .. }
        ));
    }

    #[test]
    fn orders_dependencies_before_dependents() {
        let planner = OptimizationPlanner::new();

        let mut first = candidate("core.first");
        let second = candidate("core.second");

        first.contract.dependencies.insert(
            "core.second".to_owned(),
        );

        let plan = planner
            .build_plan(&context(), [first, second])
            .unwrap();

        let ids: Vec<&str> = plan
            .pass_ids()
            .map(PassId::as_str)
            .collect();

        assert_eq!(
            ids,
            vec!["core.second", "core.first"]
        );
    }

    #[test]
    fn rejects_dependency_cycles() {
        let planner = OptimizationPlanner::new();

        let mut first = candidate("core.first");
        let mut second = candidate("core.second");

        first.contract.dependencies.insert(
            "core.second".to_owned(),
        );

        second.contract.dependencies.insert(
            "core.first".to_owned(),
        );

        assert!(matches!(
            planner.build_plan(&context(), [first, second]),
            Err(PlanningError::DependencyCycle { .. })
        ));
    }

    #[test]
    fn rejects_nondeterministic_candidates_for_reproducible_plans() {
        let planner = OptimizationPlanner::new();

        let mut candidate = candidate("core.stochastic");
        candidate.contract.deterministic = false;
        candidate.contract.stochastic = true;
        candidate.descriptor.deterministic = false;

        let plan = planner
            .build_plan(&context(), [candidate])
            .unwrap();

        assert!(plan.is_empty());

        assert_eq!(
            plan.decisions.get("core.stochastic"),
            Some(&PlanningDecision::StochasticNotAllowed)
        );
    }

    #[test]
    fn rejects_duplicate_candidates() {
        let planner = OptimizationPlanner::new();

        assert!(matches!(
            planner.build_plan(
                &context(),
                [
                    candidate("core.fold"),
                    candidate("core.fold")
                ]
            ),
            Err(PlanningError::DuplicatePass { .. })
        ));
    }

    #[test]
    fn accepts_extensible_representation_names() {
        assert!(
            RepresentationId::new("future.photonic.ir.v1").is_ok()
        );

        assert!(
            RepresentationId::new("future.neuromorphic.ir.v1").is_ok()
        );
    }

    #[test]
    fn rejects_non_finite_costs() {
        let mut estimate = CostEstimate::default();

        estimate.costs.insert(
            CostDimension::new("execution.time").unwrap(),
            f64::INFINITY,
        );

        assert!(estimate.validate().is_err());
    }

    #[test]
    fn produces_deterministic_ordering() {
        let planner = OptimizationPlanner::new();

        let first = candidate("core.alpha");
        let second = candidate("core.beta");

        let plan_a = planner
            .build_plan(&context(), [first.clone(), second.clone()])
            .unwrap();

        let plan_b = planner
            .build_plan(&context(), [second, first])
            .unwrap();

        assert_eq!(plan_a, plan_b);
    }
}
