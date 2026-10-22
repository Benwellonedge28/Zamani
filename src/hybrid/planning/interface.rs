#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Computing — Planning Interface
//!
//! Path: `src/hybrid/planning/interface.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Define the stable boundary between the canonical hybrid composition model
//! and downstream realization planners.
//!
//! The interface accepts:
//!
//! - the canonical hybrid graph and its separately owned edge set;
//! - canonical hybrid resource requirements;
//! - canonical resource-constraint assessment results;
//! - scoped capability evidence and canonical capability requirements;
//! - explicit operational configuration;
//! - an optional authoritative semantic resolver.
//!
//! It returns a validated abstract planning artifact or a structured
//! `HybridError`.
//!
//! ## Ownership
//!
//! This module owns only:
//!
//! - The planning request contract.
//! - The validation-strength policy for this interface.
//! - The prepared planning artifact and its readiness summary.
//! - Coordination of existing graph, plan, and validation APIs.
//!
//! It does not own:
//!
//! - Source syntax, ASTs, type checking, or semantic analysis.
//! - ZUIR, ZCIR, HDLIR, quantum IR, or any other domain IR.
//! - A second resource or capability model.
//! - Target discovery, device allocation, placement, scheduling, or routing.
//! - Backend selection, target-specific lowering, or code generation.
//! - Runtime execution, vendor SDKs, or hardware handles.
//! - A new feasibility solver or portability assessor.
//!
//! ## Canonical integration
//!
//! - `graph/model.rs` owns `HybridGraph`.
//! - `graph/edges.rs` owns `HybridEdgeSet` and dependency semantics.
//! - `planning/plan.rs` owns `HybridPlan`.
//! - `planning/validation.rs` owns plan validation evidence.
//! - `resources/requirements.rs` owns the hybrid collection of canonical
//!   semantic resource requirements.
//! - `resources/constraints.rs` owns the hybrid-facing resource assessments.
//! - `resources/capabilities.rs` owns the scoped capability-evidence view.
//! - `config.rs` owns operational budgets and configuration.
//! - `errors.rs` owns structured hybrid errors.
//! - Canonical semantic analysis owns program meaning.
//! - Compiler, backend, and runtime modules own realization and execution.
//!
//! ## Portability and scalability
//!
//! A prepared artifact describes obligations and references; it is not proof
//! that every target can satisfy them. Incomplete requirements, deferred
//! assessments, or blocking constraints remain visible in the readiness
//! summary and must not be treated as successful feasibility.
//!
//! This module imposes no universal maximum on domains, graph nodes, edges,
//! capabilities, or hardware resources. Explicit budgets are operational
//! policies, not restrictions on the meaning of valid Zamani programs.
//!
//! No finite machine can guarantee literally infinite computation. When an
//! explicit budget is exceeded, the operation returns an error rather than
//! silently changing program semantics.
//!
//! ## Grammar authority
//!
//! Source-language intent remains governed by `grammar/Zamani.g4`, the
//! canonical composed grammar, `grammar/hybrid/hybrid.g4`, and the normative
//! `grammar/spec/hybrid.md`. This module does not parse source text or
//! invent additional source-language syntax.

use crate::hybrid::config::HybridConfig;
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::graph::edges::HybridEdgeSet;
use crate::hybrid::graph::model::HybridGraph;
use crate::hybrid::planning::plan::HybridPlan;
use crate::hybrid::planning::validation::{
    HybridPlanValidator,
    PlanValidationMode,
    PlanValidationReport,
};
use crate::hybrid::resources::capabilities::{
    HybridCapabilityView,
    SemanticCapabilityRequirement,
};
use crate::hybrid::resources::constraints::HybridResourceConstraintReport;
use crate::hybrid::resources::requirements::HybridResourceRequirements;
use crate::hybrid::graph::verify::HybridSemanticResolver;

/// Stable identifier for this interface contract.
pub const HYBRID_PLANNING_INTERFACE_NAME: &str =
    "zamani.hybrid.planning.interface";

/// Version of this in-memory interface contract.
///
/// This is not a language version, graph schema version, interchange-format
/// version, or domain-contract version.
pub const HYBRID_PLANNING_INTERFACE_VERSION: u16 = 1;

/// The minimum verification strength required before preparing an artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlanningValidationRequirement {
    /// Require structural validation of the graph and abstract plan.
    ///
    /// This mode does not establish that canonical domain operations or
    /// extension semantics have all been resolved.
    Structural,

    /// Require completed semantic verification by the authoritative resolver.
    ///
    /// If no resolver is supplied, or semantic verification remains
    /// incomplete, preparation fails explicitly.
    Semantic,
}

/// Evidence about whether the supplied inputs are sufficiently complete for
/// downstream realization.
///
/// These flags report the state of the supplied evidence. They do not
/// independently establish target feasibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanningReadiness {
    resource_requirements_complete: bool,
    resource_assessments_complete: bool,
    resource_constraints_blocking: bool,
    capability_evidence_present: bool,
    semantic_verification_complete: bool,
}

impl PlanningReadiness {
    /// Creates a readiness summary from the supplied assessment facts.
    ///
    /// This constructor is public so integration adapters can retain the
    /// same vocabulary when they assemble evidence outside this interface.
    #[must_use]
    pub const fn new(
        resource_requirements_complete: bool,
        resource_assessments_complete: bool,
        resource_constraints_blocking: bool,
        capability_evidence_present: bool,
        semantic_verification_complete: bool,
    ) -> Self {
        Self {
            resource_requirements_complete,
            resource_assessments_complete,
            resource_constraints_blocking,
            capability_evidence_present,
            semantic_verification_complete,
        }
    }

    /// Returns whether upstream resource-requirement collection completed.
    #[must_use]
    pub const fn resource_requirements_complete(self) -> bool {
        self.resource_requirements_complete
    }

    /// Returns whether supplied resource assessments are decisive.
    ///
    /// This reflects the canonical assessment report only. It does not prove
    /// that upstream extraction included every applicable constraint.
    #[must_use]
    pub const fn resource_assessments_complete(self) -> bool {
        self.resource_assessments_complete
    }

    /// Returns whether the canonical constraint report contains blocking
    /// issues under its own evaluation rules.
    #[must_use]
    pub const fn resource_constraints_blocking(self) -> bool {
        self.resource_constraints_blocking
    }

    /// Returns whether any capability evidence was supplied.
    ///
    /// Evidence presence is not equivalent to satisfying every capability
    /// requirement. Consumers must evaluate the actual requirements against
    /// the supplied evidence.
    #[must_use]
    pub const fn capability_evidence_present(self) -> bool {
        self.capability_evidence_present
    }

    /// Returns whether semantic verification completed successfully.
    #[must_use]
    pub const fn semantic_verification_complete(self) -> bool {
        self.semantic_verification_complete
    }

    /// Returns whether this summary indicates any known incompleteness or
    /// blocking condition.
    ///
    /// This is a conservative readiness predicate, not a target-feasibility
    /// decision.
    #[must_use]
    pub const fn has_known_blockers(self) -> bool {
        !self.resource_requirements_complete
            || !self.resource_assessments_complete
            || self.resource_constraints_blocking
            || !self.semantic_verification_complete
    }
}

/// All canonical inputs needed to prepare a hybrid planning artifact.
///
/// The request borrows the caller's canonical structures; it does not clone
/// or create competing copies of their semantic contents.
///
/// The caller must ensure that requirements, constraint assessments, and
/// capability evidence refer to the same intended computation and evidence
/// scope. The interface cannot infer missing upstream semantic facts.
pub struct HybridPlanningRequest<'a> {
    graph: &'a HybridGraph,
    edges: &'a HybridEdgeSet,
    resource_requirements: &'a HybridResourceRequirements,
    resource_assessments: &'a HybridResourceConstraintReport,
    capability_view: &'a HybridCapabilityView,
    capability_requirements: &'a [SemanticCapabilityRequirement],
    config: &'a HybridConfig,
    validation_requirement: PlanningValidationRequirement,
    semantic_resolver: Option<&'a dyn HybridSemanticResolver>,
}

impl<'a> HybridPlanningRequest<'a> {
    /// Creates a request from canonical hybrid and resource inputs.
    ///
    /// Construction stores references only. Use
    /// `CanonicalHybridPlanningInterface::prepare` to validate those inputs
    /// and create a prepared artifact.
    #[must_use]
    pub const fn new(
        graph: &'a HybridGraph,
        edges: &'a HybridEdgeSet,
        resource_requirements: &'a HybridResourceRequirements,
        resource_assessments: &'a HybridResourceConstraintReport,
        capability_view: &'a HybridCapabilityView,
        capability_requirements: &'a [SemanticCapabilityRequirement],
        config: &'a HybridConfig,
        validation_requirement: PlanningValidationRequirement,
        semantic_resolver: Option<&'a dyn HybridSemanticResolver>,
    ) -> Self {
        Self {
            graph,
            edges,
            resource_requirements,
            resource_assessments,
            capability_view,
            capability_requirements,
            config,
            validation_requirement,
            semantic_resolver,
        }
    }

    /// Returns the canonical graph.
    #[must_use]
    pub const fn graph(&self) -> &'a HybridGraph {
        self.graph
    }

    /// Returns the canonical graph edge set.
    #[must_use]
    pub const fn edges(&self) -> &'a HybridEdgeSet {
        self.edges
    }

    /// Returns canonical hybrid resource requirements.
    #[must_use]
    pub const fn resource_requirements(&self) -> &'a HybridResourceRequirements {
        self.resource_requirements
    }

    /// Returns the canonical resource-constraint assessment report.
    #[must_use]
    pub const fn resource_assessments(&self) -> &'a HybridResourceConstraintReport {
        self.resource_assessments
    }

    /// Returns the scoped capability-evidence view.
    #[must_use]
    pub const fn capability_view(&self) -> &'a HybridCapabilityView {
        self.capability_view
    }

    /// Returns the canonical capability requirements in caller-supplied order.
    #[must_use]
    pub const fn capability_requirements(&self) -> &'a [SemanticCapabilityRequirement] {
        self.capability_requirements
    }

    /// Returns the operational configuration.
    #[must_use]
    pub const fn config(&self) -> &'a HybridConfig {
        self.config
    }

    /// Returns the required validation strength.
    #[must_use]
    pub const fn validation_requirement(&self) -> PlanningValidationRequirement {
        self.validation_requirement
    }

    /// Returns the optional authoritative semantic resolver.
    #[must_use]
    pub const fn semantic_resolver(
        &self,
    ) -> Option<&'a dyn HybridSemanticResolver> {
        self.semantic_resolver
    }
}

/// A prepared, target-independent planning artifact.
///
/// The artifact owns its abstract plan and validation report but retains
/// references to the original canonical inputs. This lets downstream
/// realization code inspect the evidence without reconstructing the graph or
/// inventing another resource or capability representation.
///
/// This is not executable code and does not represent a physical schedule.
pub struct PreparedHybridPlanning<'a> {
    plan: HybridPlan,
    validation: PlanValidationReport,
    readiness: PlanningReadiness,
    graph: &'a HybridGraph,
    edges: &'a HybridEdgeSet,
    resource_requirements: &'a HybridResourceRequirements,
    resource_assessments: &'a HybridResourceConstraintReport,
    capability_view: &'a HybridCapabilityView,
    capability_requirements: &'a [SemanticCapabilityRequirement],
    config: &'a HybridConfig,
}

impl<'a> PreparedHybridPlanning<'a> {
    /// Returns the validated abstract plan.
    #[must_use]
    pub const fn plan(&self) -> &HybridPlan {
        &self.plan
    }

    /// Returns the canonical plan-validation evidence.
    #[must_use]
    pub const fn validation(&self) -> &PlanValidationReport {
        &self.validation
    }

    /// Returns the readiness summary.
    #[must_use]
    pub const fn readiness(&self) -> PlanningReadiness {
        self.readiness
    }

    /// Returns the original canonical graph.
    #[must_use]
    pub const fn graph(&self) -> &'a HybridGraph {
        self.graph
    }

    /// Returns the original canonical edge set.
    #[must_use]
    pub const fn edges(&self) -> &'a HybridEdgeSet {
        self.edges
    }

    /// Returns the original resource requirements.
    #[must_use]
    pub const fn resource_requirements(&self) -> &'a HybridResourceRequirements {
        self.resource_requirements
    }

    /// Returns the original resource assessment report.
    #[must_use]
    pub const fn resource_assessments(&self) -> &'a HybridResourceConstraintReport {
        self.resource_assessments
    }

    /// Returns the original scoped capability evidence.
    #[must_use]
    pub const fn capability_view(&self) -> &'a HybridCapabilityView {
        self.capability_view
    }

    /// Returns the original capability requirements.
    #[must_use]
    pub const fn capability_requirements(&self) -> &'a [SemanticCapabilityRequirement] {
        self.capability_requirements
    }

    /// Returns the operational policy used to prepare this artifact.
    #[must_use]
    pub const fn config(&self) -> &'a HybridConfig {
        self.config
    }

    /// Returns whether known blockers remain for downstream realization.
    ///
    /// `false` does not prove target feasibility; downstream capability
    /// negotiation and feasibility assessment are still required.
    #[must_use]
    pub const fn has_known_blockers(&self) -> bool {
        self.readiness.has_known_blockers()
    }
}

/// Canonical interface for preparing hybrid planning artifacts.
///
/// This implementation coordinates existing validation and plan
/// implementations. It does not implement target selection, placement,
/// scheduling, routing, lowering, or execution.
#[derive(Debug, Default, Clone, Copy)]
pub struct CanonicalHybridPlanningInterface;

impl CanonicalHybridPlanningInterface {
    /// Validates the request and prepares its target-independent artifact.
    ///
    /// The method:
    ///
    /// 1. Checks that graph and edge identities agree.
    /// 2. Validates the capability-evidence view.
    /// 3. Builds the abstract plan using the canonical plan implementation.
    /// 4. Validates the plan against the graph and edge set.
    /// 5. Enforces the requested validation strength.
    /// 6. Preserves resource and capability evidence for downstream planners.
    ///
    /// Incomplete resource evidence or blocking resource assessments remain
    /// visible in `PlanningReadiness`. They are not misreported as proof that
    /// the abstract graph itself is invalid. A downstream target-realization
    /// operation must not proceed as if such evidence were conclusive.
    pub fn prepare<'a>(
        &self,
        request: HybridPlanningRequest<'a>,
    ) -> Result<PreparedHybridPlanning<'a>, HybridError> {
        if request.graph.id() != request.edges.graph_id() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "hybrid graph identity {} does not match edge-set identity {}",
                    request.graph.id(),
                    request.edges.graph_id()
                ),
            )
            .in_component("hybrid planning interface"));
        }

        request
            .capability_view
            .validate()
            .map_err(|error| {
                HybridError::new(
                    HybridErrorCode::InvalidRepresentation,
                    format!("invalid capability-evidence view: {error}"),
                )
                .in_component("hybrid planning interface")
            })?;

        let plan = HybridPlan::from_graph(
            request.graph,
            request.edges,
            request.config,
        )
        .map_err(|error| error.in_component("hybrid planning interface"))?;

        let validation = match request.validation_requirement {
            PlanningValidationRequirement::Structural => {
                HybridPlanValidator::validate(
                    &plan,
                    request.graph,
                    request.edges,
                    request.config,
                )?
            }
            PlanningValidationRequirement::Semantic => {
                let resolver = request.semantic_resolver.ok_or_else(|| {
                    HybridError::new(
                        HybridErrorCode::InvalidConfiguration,
                        "semantic validation was required but no authoritative semantic resolver was supplied",
                    )
                    .in_component("hybrid planning interface")
                })?;

                HybridPlanValidator::validate_with_resolver(
                    &plan,
                    request.graph,
                    request.edges,
                    request.config,
                    resolver,
                )?
            }
        };

        if request.validation_requirement == PlanningValidationRequirement::Semantic
            && !validation.is_fully_semantically_verified()
        {
            return Err(HybridError::new(
                HybridErrorCode::SemanticPreservationFailure,
                "semantic validation was required, but the canonical verifier did not establish complete semantic verification",
            )
            .in_component("hybrid planning interface"));
        }

        let readiness = PlanningReadiness::new(
            request.resource_requirements.is_complete(),
            request.resource_assessments.is_complete(),
            request.resource_assessments.has_blocking_issues(),
            !request.capability_view.is_empty(),
            validation.is_fully_semantically_verified(),
        );

        Ok(PreparedHybridPlanning {
            plan,
            validation,
            readiness,
            graph: request.graph,
            edges: request.edges,
            resource_requirements: request.resource_requirements,
            resource_assessments: request.resource_assessments,
            capability_view: request.capability_view,
            capability_requirements: request.capability_requirements,
            config: request.config,
        })
    }
}

/// Compatibility alias for callers that prefer the shorter interface name.
pub type HybridPlanningInterface = CanonicalHybridPlanningInterface;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readiness_does_not_conflate_evidence_presence_with_feasibility() {
        let readiness = PlanningReadiness::new(
            true,  // requirements complete
            true,  // supplied assessments decisive
            false, // no blocking constraint reported
            true,  // some capability evidence exists
            true,  // semantic verification complete
        );

        assert!(!readiness.has_known_blockers());
        assert!(readiness.capability_evidence_present());
    }

    #[test]
    fn missing_capability_evidence_is_not_claimed_as_support() {
        let readiness = PlanningReadiness::new(
            true,
            true,
            false,
            false,
            true,
        );

        // Missing evidence remains visible to downstream feasibility checks.
        assert!(readiness.capability_evidence_present() == false);
        assert!(!readiness.has_known_blockers());
    }

    #[test]
    fn incomplete_or_blocking_resource_evidence_is_not_hidden() {
        let readiness = PlanningReadiness::new(
            false,
            false,
            true,
            false,
            false,
        );

        assert!(readiness.has_known_blockers());
        assert!(!readiness.resource_requirements_complete());
        assert!(!readiness.resource_assessments_complete());
        assert!(readiness.resource_constraints_blocking());
    }

    #[test]
    fn semantic_validation_strength_is_explicit() {
        assert_ne!(
            PlanningValidationRequirement::Structural,
            PlanningValidationRequirement::Semantic
        );
    }
}