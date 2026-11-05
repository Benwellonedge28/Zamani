
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Optimization.
//!
//! Path:
//!     src/compiler/optimization/mod.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! # Purpose
//!
//! This module is the public composition root for compiler-level
//! optimization planning, policy resolution, cost estimation,
//! profitability assessment, optimization decisions and validation.
//!
//! It coordinates the existing optimization infrastructure.
//! It does not implement a competing optimizer.
//!
//! # Architecture
//!
//! ```text
//!                     Compiler Optimization
//!                              |
//!                              v
//!                         Optimization
//!                              |
//!             +----------------+----------------+
//!             |                |                |
//!             v                v                v
//!          Planning          Policies        Cost Models
//!             |                |                |
//!             +----------------+----------------+
//!                              |
//!                              v
//!                         Profitability
//!                              |
//!                              v
//!                         Decisions
//!                              |
//!                              v
//!                         Validation
//!                              |
//!                              v
//!                    Compiler Pass Integration
//!                              |
//!                              v
//!                       Canonical ZUIR
//!                              |
//!                              v
//!                     Domain Optimizers
//! ```
//!
//! # Architectural authorities
//!
//! - `crate::zuir`: Universal Intermediate Representation.
//! - `crate::zuir::pass`: Canonical pass contracts and execution.
//! - `crate::compiler::passes`: Compiler-facing pass integration.
//! - `crate::compiler::analysis`: Compiler analysis integration.
//! - `crate::compiler::configuration`: Compiler configuration and budgets.
//! - `crate::optimizer`: Existing optimization subsystem.
//! - `crate::classical`: Classical computation.
//! - `crate::quantum::ir`: Canonical quantum IR.
//! - `crate::hdl::hdlir`: Canonical HDL representation.
//! - `crate::ai`: AI computation.
//! - `crate::hybrid`: Hybrid computation.
//! - `crate::distributed`: Distributed computation.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - The public optimization module namespace.
//! - Stable optimization API re-exports.
//! - Module composition.
//! - Optimization API version.
//! - Cross-module documentation.
//!
//! This module does not own:
//!
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Effect analysis.
//! - IR definitions.
//! - IR verification algorithms.
//! - Optimization transformation implementations.
//! - Pass registration.
//! - Pass execution.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Physical routing.
//! - Runtime scheduling.
//! - Backend code generation.
//!
//! # Scalability
//!
//! The optimization subsystem must not impose universal limits on:
//!
//! - Number of optimization passes.
//! - Number of transformations.
//! - Number of IR operations.
//! - Number of computational domains.
//! - Number of optimization objectives.
//! - Number of cost dimensions.
//! - Number of resources.
//! - Number of targets.
//! - Number of dependencies.
//! - Number of optimization policies.
//!
//! Actual resource consumption remains bounded by available
//! implementation and execution resources.
//!
//! Configured budgets are operational policies, not semantic limits.
//!
//! # POCO-REAF
//!
//! Optimization must preserve the meaning of a Zamani program
//! independently of the eventual target realization.
//!
//! Target-independent optimization and target-specific optimization
//! must remain distinguishable.
//!
//! A target-specific transformation requires evidence that its
//! preconditions and target capabilities are satisfied.
//!
//! # Safety
//!
//! This module uses safe Rust exclusively.
//!
//! No unsafe implementation is permitted.
//!
//! # Failure behavior
//!
//! Invalid plans, missing preservation evidence, unavailable
//! cost information required by policy, incompatible contracts,
//! and failed validation must be reported explicitly.
//!
//! This module must never silently accept an invalid optimization.
//!
//! # Compatibility
//!
//! Existing modules under `src/compiler/optimization/` remain
//! independently accessible through their public module paths.
//!
//! The public facade exports selected stable contracts without
//! requiring callers to depend on internal implementation details.

// ============================================================================
// MODULE COMPOSITION
// ============================================================================

/// Optimization planning, candidate descriptions, transformation
/// contracts, legality evidence and deterministic plan construction.
pub mod planning;

/// Optimization policies, policy identity, composition,
/// precedence and resolution.
pub mod policies;

/// Extensible cost dimensions, workload descriptions,
/// cost models and cost estimation.
pub mod cost_model;

/// Profitability policies, objective evaluation,
/// uncertainty handling and improvement assessment.
pub mod profitability;

/// Structured optimization decisions, decision records,
/// evidence and decision provenance.
pub mod decisions;

/// Optimization candidate and plan validation,
/// including validation policies and structured failures.
pub mod validation;

// ============================================================================
// MODULE API VERSION
// ============================================================================

/// Stable public API version of the compiler optimization subsystem.
///
/// Increment this when making an incompatible change to the
/// public optimization facade.
pub const OPTIMIZATION_API_VERSION: u16 = 1;

// ============================================================================
// PLANNING API
// ============================================================================

/// Public planning contracts.
pub use planning::{
    AnalysisId,
    CostDimension,
    CostEstimate,
    OptimizationCandidate,
    OptimizationObjective,
    OptimizationPlan,
    OptimizationPlanStep,
    OptimizationPlanner,
    OptimizationPlanningContext,
    OptimizationPlanningPolicy,
    PlanningDecision,
    PlanningError,
    PlanningEvidence,
    PlanningResult,
    PreservationContract,
    RepresentationId,
    ResourceId,
    TransformationContract,
    OPTIMIZATION_PLANNING_API_VERSION,
};

// ============================================================================
// POLICY API
// ============================================================================

/// Public policy contracts.
pub use policies::{
    OptimizationPolicy,
    OptimizationPolicySet,
    OptimizationPoliciesApiVersionPlaceholder,
    PolicyError,
    PolicyId,
    PolicyPatch,
    PolicyResult,
    PolicyValue,
    PolicyVersion,
    ResolvedOptimizationPolicy,
    OPTIMIZATION_POLICIES_API_VERSION,
};

// ============================================================================
// COST MODEL API
// ============================================================================

/// Public cost-model contracts.
pub use cost_model::{
    AnalyticalCostModel,
    CostCoefficient,
    CostModel,
    CostModelError,
    CostProfile,
    EstimatedCost,
    MetricId,
    ModelId,
    Objective,
    ObjectiveWeight,
    ProfileId,
    Quantity,
    UnitId,
    Workload,
};

// ============================================================================
// PROFITABILITY API
// ============================================================================

/// Public profitability contracts.
pub use profitability::{
    Improvement,
    ObjectiveDirection,
    ProfitabilityAssessment,
    ProfitabilityError,
    ProfitabilityEvaluator,
    ProfitabilityPolicy,
    ProfitabilityResult,
    ProfitabilityStatus,
    UncertaintyMode,
    PROFITABILITY_API_VERSION,
};

// ============================================================================
// DECISION API
// ============================================================================

/// Public optimization decision contracts.
pub use decisions::{
    DecisionDisposition,
    DecisionError,
    DecisionReason,
    DecisionSummary,
    OptimizationDecisionLedger,
    OptimizationDecisionRecord,
    ProfitabilityEvidence,
    DecisionResult,
    OPTIMIZATION_DECISIONS_API_VERSION,
};

// ============================================================================
// VALIDATION API
// ============================================================================

/// Public optimization validation contracts.
pub use validation::{
    CandidateValidation,
    OptimizationValidationError,
    OptimizationValidationPolicy,
    OptimizationValidator,
    PlanValidation,
    ValidationResult,
    OPTIMIZATION_VALIDATION_API_VERSION,
};

// ============================================================================
// MODULE INTEGRATION CONTRACT
// ============================================================================

/// Returns the public optimization API version.
///
/// This is useful for integration diagnostics and compatibility
/// checks between independently developed compiler components.
#[must_use]
pub const fn optimization_api_version() -> u16 {
    OPTIMIZATION_API_VERSION
}

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

/// Describes the responsibilities of this module.
///
/// This function is intentionally descriptive. It does not initialize
/// global state or execute optimization passes.
#[must_use]
pub const fn optimization_ownership() -> &'static str {
    "Optimization planning, policies, cost estimation, profitability, \
     decisions and validation. Transformation implementation and \
     pass execution remain with canonical optimizer infrastructure."
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_api_version_is_stable() {
        assert_eq!(optimization_api_version(), 1);
    }

    #[test]
    fn planning_api_is_available() {
        assert_eq!(
            OPTIMIZATION_PLANNING_API_VERSION,
            1
        );
    }

    #[test]
    fn policy_api_is_available() {
        assert_eq!(
            OPTIMIZATION_POLICIES_API_VERSION,
            1
        );
    }

    #[test]
    fn profitability_api_is_available() {
        assert_eq!(
            PROFITABILITY_API_VERSION,
            1
        );
    }

    #[test]
    fn decisions_api_is_available() {
        assert_eq!(
            OPTIMIZATION_DECISIONS_API_VERSION,
            1
        );
    }

    #[test]
    fn validation_api_is_available() {
        assert_eq!(
            OPTIMIZATION_VALIDATION_API_VERSION,
            1
        );
    }

    #[test]
    fn ownership_description_is_available() {
        assert!(
            optimization_ownership()
                .contains("pass execution")
        );
    }
}
