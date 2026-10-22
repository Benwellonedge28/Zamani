
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Realization Plan Validation
//!
//! Path: `src/hybrid/planning/validation.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Validate an abstract hybrid realization plan against its canonical
//! hybrid graph and dependency edge set.
//!
//! This module is the validation boundary for the planning layer. It
//! coordinates the existing plan consistency checks and the canonical
//! graph verifier. It does not replace either implementation.
//!
//! ## Validation levels
//!
//! Structural validation establishes that:
//!
//! - the plan satisfies its local invariants;
//! - the plan belongs to the supplied graph;
//! - every planned node and dependency matches the canonical inputs;
//! - graph and edge identities and references are structurally valid;
//! - configured validation-work budgets are respected.
//!
//! Semantic validation additionally delegates to a
//! `HybridSemanticResolver`, which must resolve canonical domain contracts,
//! canonical IR references, edge semantics, and required dependencies.
//!
//! A successful structural validation MUST NOT be interpreted as proof
//! that a plan is semantically complete or executable on a target.
//!
//! Even successful semantic validation does not establish target
//! feasibility. Capability negotiation, resource feasibility, lowering,
//! placement, scheduling, routing, and execution belong to their respective
//! downstream authorities.
//!
//! ## Canonical ownership
//!
//! - `planning/plan.rs` owns the abstract plan and plan-to-input consistency.
//! - `graph/model.rs` owns the canonical hybrid graph.
//! - `graph/edges.rs` owns typed dependency edges.
//! - `graph/verify.rs` owns whole-graph structural and semantic verification.
//! - `config.rs` owns explicit operational budgets.
//! - `errors.rs` owns structured hybrid errors.
//! - Canonical domain IRs own the meaning of their operations.
//! - Compiler, backend, and runtime layers own target realization and execution.
//!
//! This module introduces no competing graph representation, semantic model,
//! dependency vocabulary, target registry, or execution schedule.
//!
//! ## Scalability
//!
//! No universal node, edge, domain, hardware, or operation ceiling is
//! introduced. Collections remain governed by their canonical owners and
//! available resources.
//!
//! A configured `ValidationSteps` budget is an operational policy. Exhaustion
//! is reported explicitly and never changes the meaning of the program.
//! Work accounting is a deterministic estimate of validation passes, not a
//! claim that every operation has identical computational cost.
//!
//! ## Integration
//!
//! Planning entry points should use `HybridPlanValidator::validate` when
//! structural validation is sufficient, or
//! `HybridPlanValidator::validate_with_resolver` when the plan must be
//! checked against authoritative semantic information.
//!
//! A caller that requires semantic completeness must inspect
//! `PlanValidationReport::is_fully_semantically_verified()` and must not
//! silently downgrade to structural-only validation.

use serde::{Deserialize, Serialize};

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::graph::edges::HybridEdgeSet;
use crate::hybrid::graph::model::HybridGraph;
use crate::hybrid::graph::verify::{
    HybridGraphVerifier,
    HybridSemanticResolver,
    VerificationReport,
};
use crate::hybrid::ids::HybridGraphId;

use super::plan::HybridPlan;

/// Describes the validation path used for a plan.
///
/// `StructuralOnly` means that canonical domain entities and all required
/// semantic dependencies have not necessarily been resolved.
///
/// `WithSemanticResolver` means that the caller supplied the repository's
/// authoritative semantic resolver. Full semantic verification is still
/// determined by the returned `VerificationReport`, not by this enum alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanValidationMode {
    /// Validate graph structure and plan consistency only.
    StructuralOnly,

    /// Validate graph structure and delegate semantic checks to a resolver.
    WithSemanticResolver,
}

/// Evidence returned after successful plan validation.
///
/// A report is created only when the requested validation passes without a
/// fatal validation error. It deliberately preserves the graph verifier's
/// unresolved-reference and unresolved-extension information instead of
/// treating structural validity as semantic completeness.
///
/// Counts use `u128` for reporting and checked accounting. The underlying
/// collections remain limited by the resources available to the process.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanValidationReport {
    /// Graph identity shared by the validated plan and canonical inputs.
    pub graph_id: HybridGraphId,

    /// Number of plan nodes checked.
    pub validated_plan_nodes: u128,

    /// Number of plan dependencies checked.
    pub validated_plan_dependencies: u128,

    /// Validation path requested by the caller.
    pub mode: PlanValidationMode,

    /// Canonical whole-graph verification report.
    ///
    /// This report remains authoritative for structural and semantic
    /// verification status, including unresolved optional extensions.
    pub graph_verification: VerificationReport,
}

impl PlanValidationReport {
    /// Return the graph identity covered by this report.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Return the validation mode requested by the caller.
    #[must_use]
    pub const fn mode(&self) -> PlanValidationMode {
        self.mode
    }

    /// Return whether structural plan and graph validation succeeded.
    ///
    /// A report can only be constructed after those checks succeed.
    #[must_use]
    pub const fn is_structurally_valid(&self) -> bool {
        true
    }

    /// Return whether canonical semantic verification is complete.
    ///
    /// This requires the graph verifier to report completed semantic checks,
    /// checked required dependencies, and no unresolved semantic references
    /// or optional extension relationships.
    ///
    /// This does not establish that a physical target can realize the plan.
    #[must_use]
    pub fn is_fully_semantically_verified(&self) -> bool {
        self.graph_verification.is_fully_semantically_verified()
    }

    /// Return whether unresolved optional extension semantics remain.
    #[must_use]
    pub fn has_unresolved_optional_extensions(&self) -> bool {
        self.graph_verification
            .has_unresolved_optional_extensions()
    }
}

/// Stateless validator for abstract hybrid realization plans.
///
/// This validator holds no global configuration, registry, cache, target
/// state, or mutable process-wide state. Callers provide all inputs
/// explicitly, making validation behavior reproducible for the same
/// canonical inputs, resolver behavior, and configuration.
#[derive(Debug, Default, Clone, Copy)]
pub struct HybridPlanValidator;

impl HybridPlanValidator {
    /// Validate a plan structurally against its canonical graph and edges.
    ///
    /// This method checks plan-to-input consistency and delegates graph
    /// verification to `HybridGraphVerifier::verify`.
    ///
    /// The returned report explicitly preserves unresolved canonical
    /// references, domain contracts, and optional extension semantics.
    /// This method does not claim that those references have been resolved.
    ///
    /// Use `validate_with_resolver` when semantic completeness is required.
    pub fn validate(
        plan: &HybridPlan,
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
        config: &HybridConfig,
    ) -> Result<PlanValidationReport, HybridError> {
        Self::validate_inner(plan, graph, edges, config, None)
    }

    /// Validate a plan with an authoritative semantic resolver.
    ///
    /// In addition to structural consistency, the canonical graph verifier
    /// delegates domain-contract resolution, canonical IR reference
    /// resolution, edge-semantic validation, and required-dependency
    /// verification to `resolver`.
    ///
    /// The resolver must use the authoritative domain registries and IR
    /// implementations. It must not infer that an entity exists merely
    /// because its identifier is syntactically valid.
    ///
    /// The returned report can still indicate unresolved optional extension
    /// semantics. Callers requiring full semantic verification must check
    /// `is_fully_semantically_verified()` before proceeding.
    pub fn validate_with_resolver(
        plan: &HybridPlan,
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
        config: &HybridConfig,
        resolver: &dyn HybridSemanticResolver,
    ) -> Result<PlanValidationReport, HybridError> {
        Self::validate_inner(plan, graph, edges, config, Some(resolver))
    }

    fn validate_inner(
        plan: &HybridPlan,
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
        config: &HybridConfig,
        resolver: Option<&dyn HybridSemanticResolver>,
    ) -> Result<PlanValidationReport, HybridError> {
        // Reject mismatched graph identities early, before performing the
        // more expensive plan-to-graph and whole-graph validation passes.
        if plan.graph_id() != graph.id() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "hybrid plan belongs to graph {}, but supplied graph is {}",
                    plan.graph_id(),
                    graph.id()
                ),
            )
            .in_component("hybrid plan validation"));
        }

        if edges.graph_id() != graph.id() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "hybrid edge set belongs to graph {}, but supplied graph is {}",
                    edges.graph_id(),
                    graph.id()
                ),
            )
            .in_component("hybrid plan validation"));
        }

        // Account for the canonical plan consistency pass and the graph
        // verification pass. This estimate includes the plan inventories,
        // graph inventories, and domain-instance inventory. It is checked
        // before the validation passes so an explicitly configured budget
        // can reject excessive work early.
        let estimated_steps = estimate_validation_steps(
            graph.composition().len() as u128,
            graph.node_count() as u128,
            edges.len() as u128,
            plan.node_count() as u128,
            plan.dependency_count() as u128,
        )
        .ok_or_else(|| {
            HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "hybrid plan validation work accounting overflowed",
            )
            .in_component("hybrid plan validation")
        })?;

        config
            .check_budget(BudgetKind::ValidationSteps, estimated_steps)
            .map_err(|error| error.in_component("hybrid plan validation"))?;

        // The plan module is the canonical owner of plan-local invariants and
        // exact consistency against the graph and edge set. Do not duplicate
        // those rules in this orchestration module.
        plan.validate_local()
            .map_err(|error| error.in_component("hybrid plan validation"))?;

        plan.validate_against(graph, edges)
            .map_err(|error| error.in_component("hybrid plan validation"))?;

        // The graph verifier is the canonical owner of whole-graph checks.
        // Use its structural or resolver-backed entry point rather than
        // reimplementing graph validation here.
        let graph_verification = match resolver {
            Some(resolver) => HybridGraphVerifier::verify_with_resolver(
                graph,
                edges,
                config,
                resolver,
            ),
            None => HybridGraphVerifier::verify(graph, edges, config),
        }
        .map_err(|error| error.in_component("hybrid plan validation"))?;

        let mode = if resolver.is_some() {
            PlanValidationMode::WithSemanticResolver
        } else {
            PlanValidationMode::StructuralOnly
        };

        Ok(PlanValidationReport {
            graph_id: graph.id(),
            validated_plan_nodes: plan.node_count() as u128,
            validated_plan_dependencies: plan.dependency_count() as u128,
            mode,
            graph_verification,
        })
    }
}

/// Estimate the validation work performed by plan consistency and graph
/// verification using checked arithmetic.
///
/// The graph and edge inventories are visited by both the plan consistency
/// check and the graph verifier. The plan inventories are additionally
/// checked locally. Domain instances are counted for the graph verifier.
///
/// This is deterministic operational accounting, not a complexity proof or
/// an exact count of machine instructions. `None` signals arithmetic
/// overflow and is reported as an explicit validation error by the caller.
fn estimate_validation_steps(
    domain_instances: u128,
    graph_nodes: u128,
    graph_edges: u128,
    plan_nodes: u128,
    plan_dependencies: u128,
) -> Option<u128> {
    domain_instances
        .checked_add(graph_nodes.checked_mul(2)?)?
        .checked_add(graph_edges.checked_mul(2)?)?
        .checked_add(plan_nodes)?
        .checked_add(plan_dependencies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_work_estimate_accounts_for_both_graph_passes() {
        let steps = estimate_validation_steps(3, 5, 7, 5, 7);

        assert_eq!(steps, Some(3 + 10 + 14 + 5 + 7));
    }

    #[test]
    fn validation_work_estimate_supports_empty_intermediate_graphs() {
        assert_eq!(
            estimate_validation_steps(0, 0, 0, 0, 0),
            Some(0)
        );
    }

    #[test]
    fn validation_work_estimate_detects_overflow() {
        assert_eq!(
            estimate_validation_steps(u128::MAX, 1, 0, 0, 0),
            None
        );

        assert_eq!(
            estimate_validation_steps(0, u128::MAX, 0, 0, 0),
            None
        );

        assert_eq!(
            estimate_validation_steps(0, 0, 0, u128::MAX, 1),
            None
        );
    }
}
