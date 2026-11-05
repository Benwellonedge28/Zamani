
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Optimization Validation.
//!
//! Path:
//!     src/compiler/optimization/validation.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! # Purpose
//!
//! Validate structural consistency between canonical optimization pass
//! descriptors, transformation planning contracts, and completed
//! optimization plans.
//!
//! # Architectural authorities
//!
//! - `super::planning`: candidates, transformation contracts and plans.
//! - `super::cost_model`: cost-estimation contracts.
//! - `super::policies`: resolved optimization policies.
//! - `super::profitability`: profitability evidence.
//! - `super::decisions`: auditable decision records.
//! - `crate::zuir::pass::contracts`: canonical pass contracts.
//! - `crate::zuir::pass::manager`: authoritative pass execution.
//! - `crate::optimizer`: canonical optimization implementation.
//!
//! # Ownership
//!
//! This module owns:
//! - Cross-contract consistency validation.
//! - Candidate metadata validation.
//! - Optimization-plan integrity validation.
//! - Dependency-order validation.
//! - Planning-decision consistency validation.
//! - Reproducibility declaration validation.
//! - Structured validation errors.
//!
//! This module does not own:
//! - Source-language semantics.
//! - Type checking.
//! - Ownership or effect analysis.
//! - Transformation legality proofs.
//! - Cost estimation.
//! - Profitability calculations.
//! - Policy resolution.
//! - Pass registration.
//! - Pass execution.
//! - IR verification algorithms.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Scheduling or routing.
//! - Runtime execution.
//!
//! # Important correctness boundary
//!
//! Successful validation means that the optimization metadata and plan
//! are structurally consistent.
//!
//! It does not prove that a transformation preserves program semantics.
//! It does not prove that an optimization is profitable.
//! It does not establish that a target can execute the plan.
//! It does not authorize direct execution of a transformation.
//!
//! The canonical planner, profitability evaluator, IR verifier and
//! ZUIR pass manager retain their respective authorities.
//!
//! # Scalability
//!
//! No artificial limits are imposed on:
//! - Optimization candidates.
//! - Pass identities.
//! - Transformation dependencies.
//! - Optimization steps.
//! - Computing domains.
//! - Resource dimensions.
//! - Analysis identities.
//!
//! Actual memory, processing and configured operational budgets apply.
//!
//! # Safety
//!
//! Safe Rust only. Unsafe Rust is forbidden.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::planning::{
    OptimizationCandidate,
    OptimizationPlan,
    OptimizationPlanStep,
    PlanningDecision,
    TransformationContract,
};

/// Stable public optimization-validation API version.
pub const OPTIMIZATION_VALIDATION_API_VERSION: u16 = 1;

/// Result type for optimization validation.
pub type ValidationResult<T> = Result<T, OptimizationValidationError>;

// ============================================================================
// VALIDATION POLICY
// ============================================================================

/// Controls additional structural validation requirements.
///
/// These are validation requirements, not language semantics or
/// hardware-capacity limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptimizationValidationPolicy {
    /// Whether the plan must explicitly declare reproducibility.
    pub require_reproducible_plan: bool,

    /// Whether every transformation must declare verified input.
    ///
    /// The canonical pass manager remains responsible for performing
    /// actual IR verification.
    pub require_verified_input: bool,

    /// Whether every transformation must declare verified output.
    ///
    /// The canonical pass manager remains responsible for performing
    /// actual IR verification.
    pub require_verified_output: bool,
}

impl OptimizationValidationPolicy {
    /// Creates an explicit validation policy.
    #[must_use]
    pub const fn new(
        require_reproducible_plan: bool,
        require_verified_input: bool,
        require_verified_output: bool,
    ) -> Self {
        Self {
            require_reproducible_plan,
            require_verified_input,
            require_verified_output,
        }
    }

    /// Creates the conservative production policy.
    ///
    /// This policy requires reproducibility and verified input/output
    /// declarations. Actual verification remains an execution-time
    /// responsibility of the canonical pass manager.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            require_reproducible_plan: true,
            require_verified_input: true,
            require_verified_output: true,
        }
    }
}

impl Default for OptimizationValidationPolicy {
    fn default() -> Self {
        Self::conservative()
    }
}

// ============================================================================
// VALIDATION SUMMARY
// ============================================================================

/// Summary of a successfully validated optimization candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateValidation {
    /// Canonical pass identity.
    pub pass_id: String,

    /// Number of required analyses.
    pub required_analysis_count: usize,

    /// Number of invalidated analyses.
    pub invalidated_analysis_count: usize,

    /// Number of declared dependencies.
    pub dependency_count: usize,

    /// Whether the transformation declares deterministic behavior.
    pub deterministic: bool,

    /// Whether verified input is required.
    pub requires_verified_input: bool,

    /// Whether verified output is required.
    pub requires_verified_output: bool,
}

impl CandidateValidation {
    /// Returns whether the candidate declares deterministic behavior.
    #[must_use]
    pub const fn is_deterministic(&self) -> bool {
        self.deterministic
    }
}

/// Summary of a successfully validated optimization plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanValidation {
    /// Number of optimization steps.
    pub step_count: usize,

    /// Number of accepted planning decisions.
    pub accepted_decision_count: usize,

    /// Whether the plan declares reproducibility.
    pub reproducible: bool,

    /// Number of dependency edges validated.
    pub dependency_edge_count: usize,

    /// API version of the validated plan.
    pub plan_api_version: u16,
}

impl PlanValidation {
    /// Returns whether the plan has no optimization steps.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.step_count == 0
    }

    /// Returns the number of optimization steps.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.step_count
    }
}

// ============================================================================
// VALIDATOR
// ============================================================================

/// Stateless optimization validator.
///
/// The validator can be reused across compilation sessions.
///
/// It does not retain candidates, plans, IR modules or mutable global state.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptimizationValidator;

impl OptimizationValidator {
    /// Creates a validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates one optimization candidate.
    ///
    /// The candidate is checked against:
    ///
    /// - Its canonical pass descriptor.
    /// - Its transformation contract.
    /// - Its analysis declarations.
    /// - Its invalidation declarations.
    /// - Its determinism declarations.
    /// - Its verification declarations.
    ///
    /// This method validates metadata consistency only.
    ///
    /// The canonical planner must still establish whether required
    /// analyses and preconditions are actually available.
    pub fn validate_candidate(
        &self,
        candidate: &OptimizationCandidate,
        policy: OptimizationValidationPolicy,
    ) -> ValidationResult<CandidateValidation> {
        candidate
            .validate()
            .map_err(|error| {
                OptimizationValidationError::InvalidCandidate {
                    pass_id: candidate.pass_id().to_owned(),
                    reason: error.to_string(),
                }
            })?;

        let descriptor = &candidate.descriptor;
        let contract = &candidate.contract;

        validate_analysis_declarations(
            candidate.pass_id(),
            contract,
            descriptor,
        )?;

        validate_determinism_declarations(
            candidate.pass_id(),
            contract,
            descriptor,
        )?;

        validate_verification_declarations(
            candidate.pass_id(),
            contract,
            descriptor,
            policy,
        )?;

        validate_preservation_declarations(
            candidate.pass_id(),
            contract,
            descriptor,
        )?;

        Ok(CandidateValidation {
            pass_id: candidate.pass_id().to_owned(),
            required_analysis_count: contract.required_analyses.len(),
            invalidated_analysis_count: contract.invalidated_analyses.len(),
            dependency_count: contract.dependencies.len(),
            deterministic: contract.deterministic,
            requires_verified_input: contract.requires_verified_input,
            requires_verified_output: contract.requires_verified_output,
        })
    }

    /// Validates a transformation contract independently.
    ///
    /// This is useful when a subsystem constructs transformation metadata
    /// before constructing a complete optimization candidate.
    ///
    /// It does not establish that the transformation is legal for a
    /// particular program.
    pub fn validate_contract(
        &self,
        contract: &TransformationContract,
    ) -> ValidationResult<()> {
        contract
            .validate()
            .map_err(|error| {
                OptimizationValidationError::InvalidTransformationContract {
                    reason: error.to_string(),
                }
            })
    }

    /// Validates a completed optimization plan.
    ///
    /// The validator checks:
    ///
    /// - Supported plan API version.
    /// - The plan's own structural validity.
    /// - Accepted planning decisions.
    /// - Complete representation of accepted decisions.
    /// - Dependency existence.
    /// - Dependency ordering.
    /// - Reproducibility consistency.
    /// - Optional production validation requirements.
    ///
    /// It does not execute any pass.
    pub fn validate_plan(
        &self,
        plan: &OptimizationPlan,
        policy: OptimizationValidationPolicy,
    ) -> ValidationResult<PlanValidation> {
        if plan.api_version != super::planning::OPTIMIZATION_PLANNING_API_VERSION {
            return Err(
                OptimizationValidationError::UnsupportedPlanVersion {
                    found: plan.api_version,
                    supported: super::planning::OPTIMIZATION_PLANNING_API_VERSION,
                },
            );
        }

        plan.validate().map_err(|error| {
            OptimizationValidationError::InvalidPlan {
                reason: error.to_string(),
            }
        })?;

        if policy.require_reproducible_plan && !plan.reproducible {
            return Err(
                OptimizationValidationError::ReproducibilityRequired,
            );
        }

        let mut positions = BTreeMap::<String, usize>::new();

        for (position, step) in plan.steps.iter().enumerate() {
            validate_plan_step(step, position, policy)?;

            let pass_id = step.pass_id.as_str().to_owned();

            if positions.insert(pass_id.clone(), position).is_some() {
                return Err(
                    OptimizationValidationError::DuplicatePlanPass {
                        pass_id,
                    },
                );
            }
        }

        let mut dependency_edge_count = 0usize;

        for (position, step) in plan.steps.iter().enumerate() {
            for dependency in &step.contract.dependencies {
                dependency_edge_count = dependency_edge_count
                    .checked_add(1)
                    .ok_or(
                        OptimizationValidationError::ArithmeticOverflow,
                    )?;

                let dependency_position = positions
                    .get(dependency)
                    .copied()
                    .ok_or_else(|| {
                        OptimizationValidationError::MissingPlanDependency {
                            pass_id: step.pass_id.as_str().to_owned(),
                            dependency: dependency.clone(),
                        }
                    })?;

                if dependency_position >= position {
                    return Err(
                        OptimizationValidationError::DependencyOrderViolation {
                            pass_id: step.pass_id.as_str().to_owned(),
                            dependency: dependency.clone(),
                        },
                    );
                }
            }
        }

        let accepted_decision_count =
            validate_decision_coverage(plan, &positions)?;

        validate_plan_reproducibility(plan)?;

        Ok(PlanValidation {
            step_count: plan.steps.len(),
            accepted_decision_count,
            reproducible: plan.reproducible,
            dependency_edge_count,
            plan_api_version: plan.api_version,
        })
    }
}

impl OptimizationValidator {
    /// Validates a plan using the conservative production policy.
    pub fn validate_production_plan(
        &self,
        plan: &OptimizationPlan,
    ) -> ValidationResult<PlanValidation> {
        self.validate_plan(
            plan,
            OptimizationValidationPolicy::conservative(),
        )
    }

    /// Validates a candidate using the conservative production policy.
    pub fn validate_production_candidate(
        &self,
        candidate: &OptimizationCandidate,
    ) -> ValidationResult<CandidateValidation> {
        self.validate_candidate(
            candidate,
            OptimizationValidationPolicy::conservative(),
        )
    }
}

// ============================================================================
// CANDIDATE CONTRACT CONSISTENCY
// ============================================================================

/// Validates agreement between canonical pass analysis declarations
/// and the additional transformation planning contract.
fn validate_analysis_declarations(
    pass_id: &str,
    contract: &TransformationContract,
    descriptor: &crate::zuir::pass::contracts::PassDescriptor,
) -> ValidationResult<()> {
    let contract_required: BTreeSet<&str> = contract
        .required_analyses
        .iter()
        .map(|analysis| analysis.as_str())
        .collect();

    let descriptor_required: BTreeSet<&str> = descriptor
        .required_analyses
        .iter()
        .map(String::as_str)
        .collect();

    if contract_required != descriptor_required {
        return Err(
            OptimizationValidationError::AnalysisDeclarationMismatch {
                pass_id: pass_id.to_owned(),
                declaration: "required analyses".to_owned(),
            },
        );
    }

    let contract_invalidated: BTreeSet<&str> = contract
        .invalidated_analyses
        .iter()
        .map(|analysis| analysis.as_str())
        .collect();

    let descriptor_invalidated: BTreeSet<&str> = descriptor
        .invalidated_analyses
        .iter()
        .map(String::as_str)
        .collect();

    if contract_invalidated != descriptor_invalidated {
        return Err(
            OptimizationValidationError::AnalysisDeclarationMismatch {
                pass_id: pass_id.to_owned(),
                declaration: "invalidated analyses".to_owned(),
            },
        );
    }

    Ok(())
}

/// Validates determinism declarations across both contracts.
fn validate_determinism_declarations(
    pass_id: &str,
    contract: &TransformationContract,
    descriptor: &crate::zuir::pass::contracts::PassDescriptor,
) -> ValidationResult<()> {
    if contract.deterministic != descriptor.deterministic {
        return Err(
            OptimizationValidationError::DeterminismDeclarationMismatch {
                pass_id: pass_id.to_owned(),
                contract_deterministic: contract.deterministic,
                descriptor_deterministic: descriptor.deterministic,
            },
        );
    }

    if contract.deterministic && contract.stochastic {
        return Err(
            OptimizationValidationError::ContradictoryDeterminism {
                pass_id: pass_id.to_owned(),
            },
        );
    }

    Ok(())
}

/// Validates verification declarations.
fn validate_verification_declarations(
    pass_id: &str,
    contract: &TransformationContract,
    descriptor: &crate::zuir::pass::contracts::PassDescriptor,
    policy: OptimizationValidationPolicy,
) -> ValidationResult<()> {
    if contract.requires_verified_input && !descriptor.requires_verified_input {
        return Err(
            OptimizationValidationError::VerificationDeclarationMismatch {
                pass_id: pass_id.to_owned(),
                declaration: "verified input".to_owned(),
            },
        );
    }

    if contract.requires_verified_output && !descriptor.verifies_output {
        return Err(
            OptimizationValidationError::VerificationDeclarationMismatch {
                pass_id: pass_id.to_owned(),
                declaration: "verified output".to_owned(),
            },
        );
    }

    if policy.require_verified_input
        && (!contract.requires_verified_input
            || !descriptor.requires_verified_input)
    {
        return Err(
            OptimizationValidationError::VerificationRequirementMissing {
                pass_id: pass_id.to_owned(),
                requirement: "verified input".to_owned(),
            },
        );
    }

    if policy.require_verified_output
        && (!contract.requires_verified_output
            || !descriptor.verifies_output)
    {
        return Err(
            OptimizationValidationError::VerificationRequirementMissing {
                pass_id: pass_id.to_owned(),
                requirement: "verified output".to_owned(),
            },
        );
    }

    Ok(())
}

/// Validates analysis-preservation declarations.
fn validate_preservation_declarations(
    pass_id: &str,
    contract: &TransformationContract,
    descriptor: &crate::zuir::pass::contracts::PassDescriptor,
) -> ValidationResult<()> {
    if descriptor.preserves_all_analyses
        && !descriptor.invalidated_analyses.is_empty()
    {
        return Err(
            OptimizationValidationError::ContradictoryAnalysisPreservation {
                pass_id: pass_id.to_owned(),
            },
        );
    }

    if descriptor.preserves_all_analyses
        && !contract.invalidated_analyses.is_empty()
    {
        return Err(
            OptimizationValidationError::ContradictoryAnalysisPreservation {
                pass_id: pass_id.to_owned(),
            },
        );
    }

    if contract.preservation.preserve_effects
        && contract.effects.is_empty()
    {
        return Err(
            OptimizationValidationError::IncompletePreservationDeclaration {
                pass_id: pass_id.to_owned(),
                declaration: "effect preservation".to_owned(),
            },
        );
    }

    Ok(())
}

// ============================================================================
// PLAN VALIDATION
// ============================================================================

/// Validates one plan step's transformation metadata.
fn validate_plan_step(
    step: &OptimizationPlanStep,
    position: usize,
    policy: OptimizationValidationPolicy,
) -> ValidationResult<()> {
    step.contract.validate().map_err(|error| {
        OptimizationValidationError::InvalidPlanStep {
            pass_id: step.pass_id.as_str().to_owned(),
            position,
            reason: error.to_string(),
        }
    })?;

    step.cost.validate().map_err(|error| {
        OptimizationValidationError::InvalidPlanStep {
            pass_id: step.pass_id.as_str().to_owned(),
            position,
            reason: error.to_string(),
        }
    })?;

    if policy.require_verified_input && !step.contract.requires_verified_input {
        return Err(
            OptimizationValidationError::VerificationRequirementMissing {
                pass_id: step.pass_id.as_str().to_owned(),
                requirement: "verified input".to_owned(),
            },
        );
    }

    if policy.require_verified_output && !step.contract.requires_verified_output {
        return Err(
            OptimizationValidationError::VerificationRequirementMissing {
                pass_id: step.pass_id.as_str().to_owned(),
                requirement: "verified output".to_owned(),
            },
        );
    }

    Ok(())
}

/// Ensures every plan step corresponds to an accepted planner decision,
/// and every accepted planner decision is represented by a plan step.
fn validate_decision_coverage(
    plan: &OptimizationPlan,
    positions: &BTreeMap<String, usize>,
) -> ValidationResult<usize> {
    let mut accepted_count = 0usize;

    for (pass_id, decision) in &plan.decisions {
        match decision {
            PlanningDecision::Accepted => {
                accepted_count = accepted_count
                    .checked_add(1)
                    .ok_or(
                        OptimizationValidationError::ArithmeticOverflow,
                    )?;

                if !positions.contains_key(pass_id) {
                    return Err(
                        OptimizationValidationError::AcceptedDecisionMissingFromPlan {
                            pass_id: pass_id.clone(),
                        },
                    );
                }
            }

            _ => {
                if positions.contains_key(pass_id) {
                    return Err(
                        OptimizationValidationError::RejectedDecisionInPlan {
                            pass_id: pass_id.clone(),
                        },
                    );
                }
            }
        }
    }

    for pass_id in positions.keys() {
        if !plan.decisions.contains_key(pass_id) {
            return Err(
                OptimizationValidationError::MissingPlanningDecision {
                    pass_id: pass_id.clone(),
                },
            );
        }
    }

    Ok(accepted_count)
}

/// Checks the consistency of the plan-level reproducibility declaration.
///
/// A plan may declare itself non-reproducible even when all its current
/// steps are deterministic, for example when the caller did not request
/// reproducible planning. It may never declare reproducibility when a
/// contained transformation is nondeterministic or stochastic.
fn validate_plan_reproducibility(
    plan: &OptimizationPlan,
) -> ValidationResult<()> {
    if plan.reproducible {
        for step in &plan.steps {
            if !step.contract.deterministic || step.contract.stochastic {
                return Err(
                    OptimizationValidationError::FalseReproducibilityClaim {
                        pass_id: step.pass_id.as_str().to_owned(),
                    },
                );
            }
        }
    }

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Structured optimization-validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationValidationError {
    /// Candidate metadata is invalid.
    InvalidCandidate {
        /// Canonical pass identity.
        pass_id: String,
        /// Underlying reason.
        reason: String,
    },

    /// Transformation contract is invalid.
    InvalidTransformationContract {
        /// Underlying reason.
        reason: String,
    },

    /// Completed plan is invalid.
    InvalidPlan {
        /// Underlying reason.
        reason: String,
    },

    /// A plan step contains invalid metadata.
    InvalidPlanStep {
        /// Pass identity.
        pass_id: String,
        /// Step position.
        position: usize,
        /// Underlying reason.
        reason: String,
    },

    /// Plan API version is unsupported.
    UnsupportedPlanVersion {
        /// Version found in the plan.
        found: u16,
        /// Supported planning API version.
        supported: u16,
    },

    /// The policy requires a reproducible plan.
    ReproducibilityRequired,

    /// A pass has contradictory determinism declarations.
    DeterminismDeclarationMismatch {
        /// Pass identity.
        pass_id: String,
        /// Transformation-contract declaration.
        contract_deterministic: bool,
        /// Canonical pass-descriptor declaration.
        descriptor_deterministic: bool,
    },

    /// A pass claims deterministic and stochastic behavior together.
    ContradictoryDeterminism {
        /// Pass identity.
        pass_id: String,
    },

    /// Required or invalidated analysis declarations disagree.
    AnalysisDeclarationMismatch {
        /// Pass identity.
        pass_id: String,
        /// Declaration category.
        declaration: String,
    },

    /// Verification declarations disagree.
    VerificationDeclarationMismatch {
        /// Pass identity.
        pass_id: String,
        /// Declaration category.
        declaration: String,
    },

    /// A required verification declaration is missing.
    VerificationRequirementMissing {
        /// Pass identity.
        pass_id: String,
        /// Missing requirement.
        requirement: String,
    },

    /// Analysis preservation declarations contradict invalidation.
    ContradictoryAnalysisPreservation {
        /// Pass identity.
        pass_id: String,
    },

    /// Preservation metadata is incomplete.
    IncompletePreservationDeclaration {
        /// Pass identity.
        pass_id: String,
        /// Missing declaration.
        declaration: String,
    },

    /// The plan contains a duplicate pass.
    DuplicatePlanPass {
        /// Pass identity.
        pass_id: String,
    },

    /// A dependency is not represented in the plan.
    MissingPlanDependency {
        /// Dependent pass.
        pass_id: String,
        /// Required dependency.
        dependency: String,
    },

    /// A dependency occurs after the pass that depends on it.
    DependencyOrderViolation {
        /// Dependent pass.
        pass_id: String,
        /// Required dependency.
        dependency: String,
    },

    /// A pass was accepted but is absent from the plan.
    AcceptedDecisionMissingFromPlan {
        /// Pass identity.
        pass_id: String,
    },

    /// A rejected or deferred pass appears in the plan.
    RejectedDecisionInPlan {
        /// Pass identity.
        pass_id: String,
    },

    /// A plan step has no corresponding planning decision.
    MissingPlanningDecision {
        /// Pass identity.
        pass_id: String,
    },

    /// A plan claims reproducibility despite containing a
    /// nondeterministic or stochastic transformation.
    FalseReproducibilityClaim {
        /// Pass identity.
        pass_id: String,
    },

    /// A validation counter exceeded its representable range.
    ArithmeticOverflow,
}

impl fmt::Display for OptimizationValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCandidate { pass_id, reason } => {
                write!(
                    formatter,
                    "optimization candidate {pass_id:?} is invalid: {reason}"
                )
            }

            Self::InvalidTransformationContract { reason } => {
                write!(
                    formatter,
                    "invalid transformation contract: {reason}"
                )
            }

            Self::InvalidPlan { reason } => {
                write!(formatter, "invalid optimization plan: {reason}")
            }

            Self::InvalidPlanStep {
                pass_id,
                position,
                reason,
            } => {
                write!(
                    formatter,
                    "invalid optimization step {pass_id:?} at position \
                     {position}: {reason}"
                )
            }

            Self::UnsupportedPlanVersion { found, supported } => {
                write!(
                    formatter,
                    "unsupported optimization plan API version {found}; \
                     supported version is {supported}"
                )
            }

            Self::ReproducibilityRequired => {
                formatter.write_str(
                    "optimization policy requires a reproducible plan",
                )
            }

            Self::DeterminismDeclarationMismatch {
                pass_id,
                contract_deterministic,
                descriptor_deterministic,
            } => {
                write!(
                    formatter,
                    "pass {pass_id:?} has inconsistent determinism \
                     declarations: contract={contract_deterministic}, \
                     descriptor={descriptor_deterministic}"
                )
            }

            Self::ContradictoryDeterminism { pass_id } => {
                write!(
                    formatter,
                    "pass {pass_id:?} declares contradictory deterministic \
                     and stochastic behavior"
                )
            }

            Self::AnalysisDeclarationMismatch {
                pass_id,
                declaration,
            } => {
                write!(
                    formatter,
                    "pass {pass_id:?} has inconsistent {declaration} declarations"
                )
            }

            Self::VerificationDeclarationMismatch {
                pass_id,
                declaration,
            } => {
                write!(
                    formatter,
                    "pass {pass_id:?} has inconsistent {declaration} declarations"
                )
            }

            Self::VerificationRequirementMissing {
                pass_id,
                requirement,
            } => {
                write!(
                    formatter,
                    "pass {pass_id:?} does not declare required {requirement}"
                )
            }

            Self::ContradictoryAnalysisPreservation { pass_id } => {
                write!(
                    formatter,
                    "pass {pass_id:?} claims to preserve all analyses \
                     while also invalidating analyses"
                )
            }

            Self::IncompletePreservationDeclaration {
                pass_id,
                declaration,
            } => {
                write!(
                    formatter,
                    "pass {pass_id:?} declares {declaration} without \
                     identifying the relevant effects"
                )
            }

            Self::DuplicatePlanPass { pass_id } => {
                write!(
                    formatter,
                    "optimization plan contains duplicate pass {pass_id:?}"
                )
            }

            Self::MissingPlanDependency {
                pass_id,
                dependency,
            } => {
                write!(
                    formatter,
                    "pass {pass_id:?} depends on {dependency:?}, \
                     which is absent from the optimization plan"
                )
            }

            Self::DependencyOrderViolation {
                pass_id,
                dependency,
            } => {
                write!(
                    formatter,
                    "dependency {dependency:?} must occur before \
                     dependent pass {pass_id:?}"
                )
            }

            Self::AcceptedDecisionMissingFromPlan { pass_id } => {
                write!(
                    formatter,
                    "accepted pass {pass_id:?} is missing from the plan"
                )
            }

            Self::RejectedDecisionInPlan { pass_id } => {
                write!(
                    formatter,
                    "rejected or deferred pass {pass_id:?} appears in the plan"
                )
            }

            Self::MissingPlanningDecision { pass_id } => {
                write!(
                    formatter,
                    "plan step {pass_id:?} has no corresponding planning decision"
                )
            }

            Self::FalseReproducibilityClaim { pass_id } => {
                write!(
                    formatter,
                    "plan claims reproducibility but pass {pass_id:?} \
                     is nondeterministic or stochastic"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "optimization validation counter overflowed",
                )
            }
        }
    }
}

impl Error for OptimizationValidationError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    use crate::zuir::pass::contracts::{
        PassDescriptor,
        PassId,
        PassVersion,
    };

    use super::super::planning::{
        AnalysisId,
        OptimizationObjective,
        OptimizationPlanStep,
        RepresentationId,
        TransformationContract,
    };

    fn candidate(
        pass_name: &str,
        deterministic: bool,
    ) -> OptimizationCandidate {
        let pass_id = PassId::new(pass_name)
            .expect("valid pass identity");

        let mut descriptor = PassDescriptor::new(
            pass_id,
            PassVersion::new(1, 0, 0),
            "Test optimization pass",
        );

        descriptor.deterministic = deterministic;
        descriptor.requires_verified_input = true;
        descriptor.verifies_output = true;

        let mut contract = TransformationContract::new(
            RepresentationId::new("zuir.core.v1")
                .expect("valid input representation"),
            RepresentationId::new("zuir.core.v1")
                .expect("valid output representation"),
        );

        contract.deterministic = deterministic;
        contract.requires_verified_input = true;
        contract.requires_verified_output = true;

        OptimizationCandidate::new(descriptor, contract)
    }

    fn accepted_plan(
        steps: Vec<OptimizationPlanStep>,
        decisions: BTreeMap<String, PlanningDecision>,
        reproducible: bool,
    ) -> OptimizationPlan {
        OptimizationPlan {
            api_version: super::super::planning::OPTIMIZATION_PLANNING_API_VERSION,
            objective: OptimizationObjective::new("execution.latency")
                .expect("valid objective"),
            steps,
            decisions,
            cost_model_id: None,
            profile_id: None,
            reproducible,
        }
    }

    #[test]
    fn accepts_consistent_candidate() {
        let validator = OptimizationValidator::new();
        let value = candidate("classical.constant_fold", true);

        let result = validator
            .validate_candidate(
                &value,
                OptimizationValidationPolicy::default(),
            )
            .expect("candidate should be valid");

        assert_eq!(result.pass_id, "classical.constant_fold");
        assert!(result.deterministic);
    }

    #[test]
    fn rejects_analysis_declaration_mismatch() {
        let validator = OptimizationValidator::new();
        let mut value = candidate("classical.constant_fold", true);

        value.contract.required_analyses.insert(
            AnalysisId::new("classical.constant_values")
                .expect("valid analysis"),
        );

        let result = validator.validate_candidate(
            &value,
            OptimizationValidationPolicy::default(),
        );

        assert!(matches!(
            result,
            Err(
                OptimizationValidationError::AnalysisDeclarationMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_determinism_declaration_mismatch() {
        let validator = OptimizationValidator::new();
        let mut value = candidate("classical.vectorize", true);

        value.contract.deterministic = false;

        let result = validator.validate_candidate(
            &value,
            OptimizationValidationPolicy::default(),
        );

        assert!(matches!(
            result,
            Err(
                OptimizationValidationError::DeterminismDeclarationMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_missing_verified_output_declaration() {
        let validator = OptimizationValidator::new();
        let mut value = candidate("classical.vectorize", true);

        value.contract.requires_verified_output = false;

        let result = validator.validate_candidate(
            &value,
            OptimizationValidationPolicy::conservative(),
        );

        assert!(matches!(
            result,
            Err(
                OptimizationValidationError::VerificationRequirementMissing {
                    ..
                }
            )
        ));
    }

    #[test]
    fn accepts_empty_plan_without_steps_or_decisions() {
        let validator = OptimizationValidator::new();

        let plan = accepted_plan(
            Vec::new(),
            BTreeMap::new(),
            true,
        );

        let result = validator
            .validate_production_plan(&plan)
            .expect("empty plan is structurally valid");

        assert!(result.is_empty());
        assert_eq!(result.accepted_decision_count, 0);
    }

    #[test]
    fn rejects_accepted_decision_missing_from_plan() {
        let validator = OptimizationValidator::new();

        let mut decisions = BTreeMap::new();
        decisions.insert(
            "classical.constant_fold".to_owned(),
            PlanningDecision::Accepted,
        );

        let plan = accepted_plan(
            Vec::new(),
            decisions,
            true,
        );

        let result = validator.validate_production_plan(&plan);

        assert!(matches!(
            result,
            Err(
                OptimizationValidationError::AcceptedDecisionMissingFromPlan {
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_rejected_pass_in_plan() {
        let validator = OptimizationValidator::new();

        let value = candidate("classical.constant_fold", true);

        let step = OptimizationPlanStep {
            pass_id: value.descriptor.id.clone(),
            pass_version: value.descriptor.version,
            contract: value.contract.clone(),
            cost: value.cost.clone(),
            priority: value.priority,
        };

        let mut decisions = BTreeMap::new();
        decisions.insert(
            "classical.constant_fold".to_owned(),
            PlanningDecision::UnverifiedInput,
        );

        let plan = accepted_plan(
            vec![step],
            decisions,
            true,
        );

        let result = validator.validate_production_plan(&plan);

        assert!(matches!(
            result,
            Err(
                OptimizationValidationError::RejectedDecisionInPlan {
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_missing_dependency() {
        let validator = OptimizationValidator::new();

        let mut dependent = candidate("classical.vectorize", true);

        dependent.contract.dependencies.insert(
            "classical.canonicalize".to_owned(),
        );

        let step = OptimizationPlanStep {
            pass_id: dependent.descriptor.id.clone(),
            pass_version: dependent.descriptor.version,
            contract: dependent.contract.clone(),
            cost: dependent.cost.clone(),
            priority: dependent.priority,
        };

        let mut decisions = BTreeMap::new();
        decisions.insert(
            "classical.vectorize".to_owned(),
            PlanningDecision::Accepted,
        );

        let plan = accepted_plan(
            vec![step],
            decisions,
            true,
        );

        let result = validator.validate_production_plan(&plan);

        assert!(matches!(
            result,
            Err(
                OptimizationValidationError::MissingPlanDependency {
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_dependency_after_dependent() {
        let validator = OptimizationValidator::new();

        let mut dependent = candidate("classical.vectorize", true);
        let dependency = candidate("classical.canonicalize", true);

        dependent.contract.dependencies.insert(
            "classical.canonicalize".to_owned(),
        );

        let dependent_step = OptimizationPlanStep {
            pass_id: dependent.descriptor.id.clone(),
            pass_version: dependent.descriptor.version,
            contract: dependent.contract.clone(),
            cost: dependent.cost.clone(),
            priority: dependent.priority,
        };

        let dependency_step = OptimizationPlanStep {
            pass_id: dependency.descriptor.id.clone(),
            pass_version: dependency.descriptor.version,
            contract: dependency.contract.clone(),
            cost: dependency.cost.clone(),
            priority: dependency.priority,
        };

        let mut decisions = BTreeMap::new();
        decisions.insert(
            "classical.vectorize".to_owned(),
            PlanningDecision::Accepted,
        );
        decisions.insert(
            "classical.canonicalize".to_owned(),
            PlanningDecision::Accepted,
        );

        let plan = accepted_plan(
            vec![dependent_step, dependency_step],
            decisions,
            true,
        );

        let result = validator.validate_production_plan(&plan);

        assert!(matches!(
            result,
            Err(
                OptimizationValidationError::DependencyOrderViolation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn accepts_dependency_before_dependent() {
        let validator = OptimizationValidator::new();

        let mut dependent = candidate("classical.vectorize", true);
        let dependency = candidate("classical.canonicalize", true);

        dependent.contract.dependencies.insert(
            "classical.canonicalize".to_owned(),
        );

        let dependency_step = OptimizationPlanStep {
            pass_id: dependency.descriptor.id.clone(),
            pass_version: dependency.descriptor.version,
            contract: dependency.contract.clone(),
            cost: dependency.cost.clone(),
            priority: dependency.priority,
        };

        let dependent_step = OptimizationPlanStep {
            pass_id: dependent.descriptor.id.clone(),
            pass_version: dependent.descriptor.version,
            contract: dependent.contract.clone(),
            cost: dependent.cost.clone(),
            priority: dependent.priority,
        };

        let mut decisions = BTreeMap::new();
        decisions.insert(
            "classical.canonicalize".to_owned(),
            PlanningDecision::Accepted,
        );
        decisions.insert(
            "classical.vectorize".to_owned(),
            PlanningDecision::Accepted,
        );

        let plan = accepted_plan(
            vec![dependency_step, dependent_step],
            decisions,
            true,
        );

        let result = validator
            .validate_production_plan(&plan)
            .expect("dependencies are correctly ordered");

        assert_eq!(result.step_count, 2);
        assert_eq!(result.dependency_edge_count, 1);
    }

    #[test]
    fn rejects_false_reproducibility_claim() {
        let validator = OptimizationValidator::new();

        let value = candidate("classical.stochastic_search", false);

        let step = OptimizationPlanStep {
            pass_id: value.descriptor.id.clone(),
            pass_version: value.descriptor.version,
            contract: value.contract.clone(),
            cost: value.cost.clone(),
            priority: value.priority,
        };

        let mut decisions = BTreeMap::new();
        decisions.insert(
            "classical.stochastic_search".to_owned(),
            PlanningDecision::Accepted,
        );

        let plan = accepted_plan(
            vec![step],
            decisions,
            true,
        );

        let result = validator.validate_plan(
            &plan,
            OptimizationValidationPolicy::new(
                false,
                true,
                true,
            ),
        );

        assert!(matches!(
            result,
            Err(
                OptimizationValidationError::FalseReproducibilityClaim {
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_unsupported_plan_version() {
        let validator = OptimizationValidator::new();

        let mut plan = accepted_plan(
            Vec::new(),
            BTreeMap::new(),
            true,
        );

        plan.api_version = u16::MAX;

        let result = validator.validate_production_plan(&plan);

        assert!(matches!(
            result,
            Err(
                OptimizationValidationError::UnsupportedPlanVersion {
                    ..
                }
            )
        ));
    }
}
