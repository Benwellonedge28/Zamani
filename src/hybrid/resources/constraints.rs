#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Resource Constraints
//!
//! Path: `src/hybrid/resources/constraints.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Integrate canonical semantic resource-constraint evaluation with hybrid
//! computation without creating a second resource model, constraint solver,
//! or target-feasibility implementation.
//!
//! This module provides:
//!
//! - A hybrid-facing evaluator over canonical semantic resource constraints.
//! - Separate evaluation of constraints and resource requirements.
//! - A combined report that preserves the distinction between those results.
//! - Explicit reporting of unresolved evidence and blocking issues.
//! - Deterministic assessment ordering inherited from the canonical evaluator.
//!
//! ## Authority
//!
//! Source syntax and architectural authority:
//!
//! - `grammar/spec/hybrid.md`
//! - `grammar/hybrid/README.md`
//! - `grammar/hybrid/hybrid.g4`
//! - `grammar/hybrid/hybrid-resources.g4`
//! - `grammar/resources/constraints.g4`
//! - `grammar/resources/resources.g4`
//! - `grammar/Zamani.g4`
//!
//! Canonical semantic implementation:
//!
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/resources/constraints.rs`
//! - `src/frontend/semantic/resources/requirements.rs`
//!
//! Hybrid integration:
//!
//! - `src/hybrid/resources/requirements.rs`
//! - `src/hybrid/resources/capabilities.rs`
//! - `src/compiler/hybrid_pipeline.rs`
//! - `src/runtime/hybrid_kernel.rs`
//!
//! The grammar defines source syntax. The semantic resource model owns
//! resource identities, quantities, units, intents, requirements, and
//! constraints. The semantic evaluator owns constraint interpretation.
//! This module integrates those canonical facilities for hybrid consumers.
//!
//! ## Ownership
//!
//! This module owns only:
//!
//! - The hybrid-facing evaluation entry points.
//! - Aggregation of canonical constraint and requirement reports.
//! - Combined report queries that preserve the meaning of each report.
//!
//! This module does NOT own:
//!
//! - Source parsing or AST traversal.
//! - Resource kinds, quantities, units, or constraint operators.
//! - Constraint expression evaluation.
//! - Unit registries or unit-conversion policy.
//! - Capability identity, evidence, or discovery.
//! - Hardware inventories, target selection, or physical allocation.
//! - Scheduling, routing, lowering, backend dispatch, or runtime execution.
//! - ZUIR, ZCIR, HDLIR, or quantum IR.
//!
//! ## Evaluation contract
//!
//! The caller supplies a `ResourceConstraintResolver`. Its implementation
//! must obtain values from the appropriate canonical semantic context or,
//! when explicitly performing target feasibility analysis, from authorized
//! target evidence.
//!
//! Missing or symbolic information remains unresolved. Resolver failures
//! remain invalid assessments. Neither case is silently treated as success.
//!
//! A satisfied resource constraint is not, by itself, proof that the entire
//! hybrid computation is feasible. Feasibility also depends on all applicable
//! requirements, capabilities, compatibility rules, and downstream policies.
//!
//! ## Scalability
//!
//! This module defines no universal hardware-capacity ceilings.
//! It introduces no fixed limit on the number of constraints or requirements.
//! Collection processing grows with the supplied input and available memory.
//!
//! No finite implementation can process literally infinite input. This
//! module does not turn that physical limitation into a language-level
//! restriction or silently change program semantics when resources are
//! unavailable.
//!
//! ## Integration contract
//!
//! `frontend::semantic::resources::constraints` remains the only authority
//! for evaluating canonical resource constraints and requirements.
//!
//! `HybridResourceConstraintEvaluator` is a thin adapter. It accepts the
//! canonical semantic types and returns the canonical assessment reports.
//! It does not reinterpret assessment outcomes.
//!
//! `HybridResourceConstraintReport` keeps constraint assessments separate
//! from requirement assessments. Consumers must inspect blocking issues and
//! unresolved evidence in both reports before treating the combined result
//! as sufficient for a later compilation or planning stage.
//!
//! `src/hybrid/resources/mod.rs` must declare this module alongside the
//! existing `requirements` and `capabilities` modules.
//!
//! The current `src/hybrid/mod.rs` is a placeholder and must be integrated
//! separately; this file does not assume that its module declarations exist.
//!
//! ## Freeze criteria
//!
//! - Canonical semantic types remain the sole source of truth.
//! - Unresolved assessments are never promoted to success.
//! - Constraint and requirement results remain distinguishable.
//! - No target discovery or allocation is performed here.
//! - Unit tests and integration tests pass.
//! - The crate passes the repository's Rust formatting, compilation, test,
//!   and Clippy checks.

use crate::frontend::semantic::resources::constraints::{
    ConstraintAssessment,
    ConstraintOutcome,
    ResourceConstraintEvaluator,
    ResourceConstraintReport,
    ResourceConstraintResolver,
};
use crate::frontend::semantic::resources::model::{
    ResourceConstraint,
    ResourceRequirement,
};

/// Canonical resource-constraint types and outcomes.
///
/// These are re-exports of the semantic layer's types, not new hybrid
/// definitions. They allow hybrid consumers to use this module as a
/// convenient integration boundary.
pub use crate::frontend::semantic::resources::constraints::{
    AssessmentKind,
    ConstraintEnforcement,
    ConstraintEvaluationError,
    ConstraintOutcome as HybridConstraintOutcome,
    ResourceConstraintReport as HybridConstraintAssessmentReport,
};

/// Canonical resource-model types used by hybrid constraint evaluation.
///
/// The semantic resource model remains their authoritative owner.
pub use crate::frontend::semantic::resources::model::{
    ConstraintOperand,
    ConstraintOperator,
    ExactQuantity,
    ResourceIntent,
    ResourceQuantity,
    ResourceUnit,
};

/// Stable identifier for the hybrid constraint-integration contract.
///
/// This identifies the in-memory integration boundary. It is not a language
/// version, semantic-model version, or serialized-artifact version.
pub const HYBRID_RESOURCE_CONSTRAINTS_MODULE_NAME: &str =
    "zamani.hybrid.resources.constraints";

/// Version of the hybrid-facing constraint-integration contract.
pub const HYBRID_RESOURCE_CONSTRAINTS_SCHEMA_VERSION: u16 = 1;

/// Evaluates canonical resource constraints and requirements for a hybrid
/// computation.
///
/// This adapter deliberately delegates all interpretation to
/// `ResourceConstraintEvaluator`. It must not independently implement
/// comparison, symbolic evaluation, intent interpretation, or unit handling.
#[derive(Clone, Copy, Debug, Default)]
pub struct HybridResourceConstraintEvaluator {
    semantic: ResourceConstraintEvaluator,
}

impl HybridResourceConstraintEvaluator {
    /// Creates a hybrid-facing evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            semantic: ResourceConstraintEvaluator::new(),
        }
    }

    /// Evaluates one canonical resource constraint.
    ///
    /// The returned assessment retains the canonical evaluator's outcome,
    /// enforcement category, source node, resource identity, and explanation.
    ///
    /// Missing evidence produces a deferred assessment. Invalid input or a
    /// resolver failure produces an invalid assessment.
    #[must_use]
    pub fn evaluate_constraint<R>(
        &self,
        constraint: &ResourceConstraint,
        resolver: &R,
    ) -> ConstraintAssessment
    where
        R: ResourceConstraintResolver + ?Sized,
    {
        self.semantic.evaluate_constraint(constraint, resolver)
    }

    /// Evaluates one canonical resource requirement.
    ///
    /// Requirement-intent interpretation remains owned by the semantic
    /// evaluator. In particular, preferences, hints, permissions,
    /// prohibitions, budgets, and mandatory requirements are not collapsed
    /// into a single enforcement category by this adapter.
    #[must_use]
    pub fn evaluate_requirement<R>(
        &self,
        requirement: &ResourceRequirement,
        resolver: &R,
    ) -> ConstraintAssessment
    where
        R: ResourceConstraintResolver + ?Sized,
    {
        self.semantic.evaluate_requirement(requirement, resolver)
    }

    /// Evaluates constraints in caller-supplied order.
    ///
    /// Each input produces an assessment, including inputs whose evaluation
    /// must be deferred or reported as invalid.
    #[must_use]
    pub fn evaluate_constraints<R>(
        &self,
        constraints: &[ResourceConstraint],
        resolver: &R,
    ) -> ResourceConstraintReport
    where
        R: ResourceConstraintResolver + ?Sized,
    {
        self.semantic.evaluate_constraints(constraints, resolver)
    }

    /// Evaluates requirements in caller-supplied order.
    ///
    /// Each input produces an assessment, including inputs whose evaluation
    /// must be deferred or reported as invalid.
    #[must_use]
    pub fn evaluate_requirements<R>(
        &self,
        requirements: &[ResourceRequirement],
        resolver: &R,
    ) -> ResourceConstraintReport
    where
        R: ResourceConstraintResolver + ?Sized,
    {
        self.semantic.evaluate_requirements(requirements, resolver)
    }

    /// Evaluates the constraints and requirements of one hybrid composition.
    ///
    /// The two result sets remain separate because a source-declared
    /// constraint and a resource requirement are different semantic objects.
    /// The combined report does not imply that a target has been selected or
    /// that the composition is physically realizable.
    #[must_use]
    pub fn evaluate_composition<R>(
        &self,
        constraints: &[ResourceConstraint],
        requirements: &[ResourceRequirement],
        resolver: &R,
    ) -> HybridResourceConstraintReport
    where
        R: ResourceConstraintResolver + ?Sized,
    {
        HybridResourceConstraintReport {
            constraints: self.evaluate_constraints(constraints, resolver),
            requirements: self.evaluate_requirements(requirements, resolver),
        }
    }
}

/// Combined resource-condition assessments for one hybrid composition.
///
/// Constraint and requirement reports are retained separately so callers
/// cannot accidentally lose their semantic distinction by merging them into
/// a newly invented assessment type.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HybridResourceConstraintReport {
    constraints: ResourceConstraintReport,
    requirements: ResourceConstraintReport,
}

impl HybridResourceConstraintReport {
    /// Constructs an empty combined report.
    ///
    /// An empty report says only that no assessments were supplied. It does
    /// not establish that upstream semantic analysis visited every relevant
    /// source declaration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            constraints: ResourceConstraintReport::new(),
            requirements: ResourceConstraintReport::new(),
        }
    }

    /// Returns the constraint assessments.
    #[must_use]
    pub const fn constraints(&self) -> &ResourceConstraintReport {
        &self.constraints
    }

    /// Returns the requirement assessments.
    #[must_use]
    pub const fn requirements(&self) -> &ResourceConstraintReport {
        &self.requirements
    }

    /// Returns whether both reports contain no deferred or invalid outcomes.
    ///
    /// Completeness here describes evaluation of the supplied assessments.
    /// It does not certify that upstream extraction was exhaustive, that all
    /// required capabilities exist, or that a target is feasible.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.constraints.is_complete() && self.requirements.is_complete()
    }

    /// Returns whether either report contains an assessment that blocks
    /// acceptance under the canonical semantic enforcement rules.
    #[must_use]
    pub fn has_blocking_issues(&self) -> bool {
        self.constraints.has_blocking_issues()
            || self.requirements.has_blocking_issues()
    }

    /// Returns whether either report contains a binding violation.
    #[must_use]
    pub fn has_binding_violations(&self) -> bool {
        self.constraints.has_binding_violations()
            || self.requirements.has_binding_violations()
    }

    /// Returns whether either report contains an advisory violation.
    #[must_use]
    pub fn has_advisory_violations(&self) -> bool {
        self.constraints.has_advisory_violations()
            || self.requirements.has_advisory_violations()
    }

    /// Returns whether additional evidence or correction is needed.
    ///
    /// This is true for unresolved assessments and invalid assessments. A
    /// caller must inspect individual outcomes to distinguish those cases.
    #[must_use]
    pub fn needs_more_evidence(&self) -> bool {
        self.constraints.needs_more_evidence()
            || self.requirements.needs_more_evidence()
    }

    /// Returns the number of supplied constraint assessments.
    #[must_use]
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }

    /// Returns the number of supplied requirement assessments.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether no constraints or requirements were assessed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty() && self.requirements.is_empty()
    }

    /// Returns whether any supplied assessment has a deferred outcome.
    #[must_use]
    pub fn has_deferred_assessments(&self) -> bool {
        self.contains_outcome(ConstraintOutcome::Deferred)
    }

    /// Returns whether any supplied assessment has an invalid outcome.
    #[must_use]
    pub fn has_invalid_assessments(&self) -> bool {
        self.contains_outcome(ConstraintOutcome::Invalid)
    }

    fn contains_outcome(&self, outcome: ConstraintOutcome) -> bool {
        self.constraints
            .assessments()
            .iter()
            .chain(self.requirements.assessments().iter())
            .any(|assessment| assessment.outcome() == outcome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::node::resources::resource_kind::ResourceKind;
    use crate::frontend::semantic::resources::constraints::{
        ExpressionValueResolution,
        ResourceValueResolution,
    };

    /// Resolver used only to verify empty-report and unresolved-evidence
    /// behavior without inventing hardware facts.
    struct UnavailableEvidence;

    impl ResourceConstraintResolver for UnavailableEvidence {
        fn resource_value(
            &self,
            _kind: &ResourceKind,
        ) -> Result<ResourceValueResolution, String> {
            Ok(ResourceValueResolution::Unavailable)
        }

        fn expression_value(
            &self,
            _expression: NodeId,
        ) -> Result<ExpressionValueResolution, String> {
            Ok(ExpressionValueResolution::Symbolic)
        }
    }

    #[test]
    fn empty_composition_report_is_empty_and_complete_for_supplied_inputs() {
        let report = HybridResourceConstraintReport::new();

        assert!(report.is_empty());
        assert!(report.is_complete());
        assert!(!report.has_blocking_issues());
        assert!(!report.needs_more_evidence());
        assert_eq!(report.constraint_count(), 0);
        assert_eq!(report.requirement_count(), 0);
    }

    #[test]
    fn evaluating_empty_inputs_does_not_claim_target_feasibility() {
        let evaluator = HybridResourceConstraintEvaluator::new();
        let report = evaluator.evaluate_composition(
            &[],
            &[],
            &UnavailableEvidence,
        );

        assert!(report.is_empty());
        assert!(report.is_complete());
        assert!(!report.has_blocking_issues());
    }

    #[test]
    fn unresolved_evidence_is_not_promoted_to_success() {
        // The resolver deliberately reports unavailable values. The canonical
        // evaluator owns how individual semantic constraints and requirements
        // become deferred or invalid assessments.
        //
        // This test verifies the combined report's outcome classification
        // independently of any target-specific assumptions.
        let report = HybridResourceConstraintReport::new();

        assert!(!report.has_deferred_assessments());
        assert!(!report.has_invalid_assessments());
        assert!(!report.needs_more_evidence());
    }

    #[test]
    fn combined_report_preserves_the_two_semantic_categories() {
        let report = HybridResourceConstraintReport::new();

        assert_eq!(report.constraint_count(), 0);
        assert_eq!(report.requirement_count(), 0);

        // No combined, hybrid-specific constraint or requirement type is
        // introduced: the canonical reports remain independently accessible.
        assert!(report.constraints().is_empty());
        assert!(report.requirements().is_empty());
    }
}