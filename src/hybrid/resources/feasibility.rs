
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Resource Feasibility
//!
//! Path: `src/hybrid/resources/feasibility.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Determine whether the available, explicitly supplied evidence establishes
//! that a hybrid computation is feasible, infeasible, or not yet decidable.
//!
//! This module combines existing hybrid resource requirements, canonical
//! resource-condition assessments, and scoped capability evidence. It does
//! not discover hardware, allocate resources, select targets, or realize
//! execution plans.
//!
//! ## Canonical ownership
//!
//! - `frontend::semantic::resources::model` owns resource semantics.
//! - `frontend::semantic::resources::constraints` owns resource evaluation.
//! - `frontend::semantic::capabilities::model` owns capability requirements.
//! - `frontend::semantic::capabilities::evidence` owns capability evidence
//!   and evidence aggregation.
//! - `hybrid::resources::requirements` owns hybrid requirement collection.
//! - `hybrid::resources::capabilities` owns the scoped capability view.
//! - `hybrid::resources::constraints` integrates canonical resource reports.
//! - This module owns only the combined feasibility decision and its findings.
//!
//! ## Decision contract
//!
//! Feasible:
//!   All required input scopes are declared complete, every required resource
//!   assessment is present and decisive, all binding resource conditions are
//!   satisfied, and every required capability has supporting evidence.
//!
//! Infeasible:
//!   Available, applicable evidence establishes at least one binding resource
//!   violation or an explicitly unsupported required capability.
//!
//! Indeterminate:
//!   Required evidence is missing, symbolic, incomplete, contradictory, or
//!   otherwise insufficient to establish feasibility or infeasibility.
//!
//! Invalid:
//!   A supplied assessment or capability-evidence view fails validation.
//!
//! A proven infeasibility takes precedence over unresolved findings because
//! an independently established mandatory violation is sufficient to reject
//! the proposed realization. Invalid evidence never establishes success.
//!
//! ## Non-ownership
//!
//! This module does not implement:
//!
//! - source parsing, AST traversal, or semantic extraction;
//! - a second resource or capability ontology;
//! - resource arithmetic, unit conversion, or constraint evaluation;
//! - hardware discovery or provider authentication;
//! - physical allocation, target selection, or placement;
//! - scheduling, routing, lowering, or backend dispatch;
//! - runtime execution or resource reservation.
//!
//! ## Scalability
//!
//! There are no universal hardware ceilings, fixed domain counts, or fixed
//! feasibility-input limits. Collections grow according to available memory.
//! Any operational work budget belongs to an explicitly configured caller.
//! Resource exhaustion must be reported by the owning operation; it must not
//! be interpreted as proof that a target is infeasible or feasible.
//!
//! No finite implementation can execute literally infinite work.
//!
//! ## Grammar and specification authority
//!
//! Source intent follows `grammar/Zamani.g4`, the canonical resource grammar,
//! `grammar/hybrid/hybrid-resources.g4`, `grammar/hybrid/hybrid.g4`,
//! `grammar/hybrid/README.md`, and `grammar/spec/hybrid.md`.
//!
//! The grammar expresses portable intent. This module evaluates supplied
//! evidence and does not change source-level semantics.

use core::fmt;
use std::error::Error;

use crate::frontend::semantic::capabilities::evidence::CapabilityEvidenceResult;

use super::capabilities::{
    HybridCapabilityError,
    HybridCapabilityView,
    SemanticCapabilityRequirement,
};
use super::constraints::{
    HybridConstraintOutcome,
    HybridResourceConstraintReport,
};
use super::requirements::HybridResourceRequirements;

/// Stable identity of the feasibility contract.
pub const HYBRID_FEASIBILITY_MODULE_NAME: &str =
    "zamani.hybrid.resources.feasibility";

/// Version of this in-memory feasibility-report contract.
///
/// This is not the Zamani language version or an interchange-format version.
pub const HYBRID_FEASIBILITY_SCHEMA_VERSION: u16 = 1;

/// Overall conclusion of a hybrid feasibility assessment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum FeasibilityStatus {
    /// All required scopes are complete and the supplied evidence establishes
    /// that the assessed requirements are satisfied.
    Feasible,

    /// Evidence establishes a mandatory violation or an unsupported required
    /// capability.
    Infeasible,

    /// Available evidence is insufficient to decide feasibility.
    ///
    /// Missing evidence, unresolved values, incomplete collection, and
    /// contradictory capability evidence must not be interpreted as success.
    Indeterminate,

    /// Supplied evidence or an assessment is invalid.
    ///
    /// Invalid evidence is not proof of feasibility or physical infeasibility.
    Invalid,
}

/// Category of a finding explaining a feasibility conclusion.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum FeasibilityFindingKind {
    /// The upstream resource-requirement collection is incomplete.
    ResourceRequirementsIncomplete,

    /// The upstream constraint collection is incomplete.
    ResourceConstraintsIncomplete,

    /// The upstream capability-requirement collection is incomplete.
    CapabilityRequirementsIncomplete,

    /// The number of evaluated resource requirements differs from the number
    /// collected for this assessment.
    ResourceAssessmentCountMismatch,

    /// A required resource or constraint could not be decided from evidence.
    ResourceAssessmentDeferred,

    /// A resource assessment contains invalid input or resolver evidence.
    ResourceAssessmentInvalid,

    /// A binding resource condition is proven to be violated.
    BindingResourceViolation,

    /// A required capability has no applicable evidence.
    CapabilityEvidenceMissing,

    /// Capability evidence does not establish support or non-support.
    CapabilityEvidenceIndeterminate,

    /// Capability evidence contradicts itself for the same subject.
    CapabilityEvidenceConflicting,

    /// A required capability is explicitly unsupported.
    RequiredCapabilityUnsupported,

    /// The capability-evidence view failed validation.
    CapabilityEvidenceInvalid,

    /// The available evidence establishes satisfaction of the assessed
    /// requirements and constraints.
    EvidenceSufficient,
}

/// One structured explanation for a feasibility result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityFinding {
    kind: FeasibilityFindingKind,
    subject: Option<String>,
    message: String,
}

impl FeasibilityFinding {
    fn new(
        kind: FeasibilityFindingKind,
        subject: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject,
            message: message.into(),
        }
    }

    /// Returns the finding category.
    #[must_use]
    pub const fn kind(&self) -> FeasibilityFindingKind {
        self.kind
    }

    /// Returns the associated resource or capability identity, if applicable.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Returns the human-readable explanation.
    ///
    /// Programmatic decisions should use `kind()` and the report status
    /// rather than parsing this message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for FeasibilityFinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.subject() {
            Some(subject) => {
                write!(formatter, "{:?} ({subject}): {}", self.kind, self.message)
            }
            None => write!(formatter, "{:?}: {}", self.kind, self.message),
        }
    }
}

/// Input evidence required to assess one hybrid-computation scope.
///
/// Completeness flags are explicit because an empty collection or an empty
/// report cannot independently prove that upstream analysis visited every
/// relevant declaration.
///
/// `resource_requirements` must describe the same requirement scope represented
/// by `resource_assessments`. Each collected resource requirement must have
/// exactly one corresponding requirement assessment in that report.
///
/// `resource_constraints_complete` means that upstream extraction of all
/// applicable resource constraints for this scope completed successfully.
///
/// `capability_requirements_complete` means that the supplied capability
/// requirements exhaust the applicable requirements for this scope.
pub struct HybridFeasibilityInput<'a> {
    resource_requirements: &'a HybridResourceRequirements,
    resource_assessments: &'a HybridResourceConstraintReport,
    capability_view: &'a HybridCapabilityView,
    capability_requirements: &'a [SemanticCapabilityRequirement],
    resource_constraints_complete: bool,
    capability_requirements_complete: bool,
}

impl<'a> HybridFeasibilityInput<'a> {
    /// Creates an input bundle for one feasibility assessment.
    ///
    /// The caller is responsible for passing mutually consistent evidence
    /// describing the same intended computation and capability subject.
    #[must_use]
    pub const fn new(
        resource_requirements: &'a HybridResourceRequirements,
        resource_assessments: &'a HybridResourceConstraintReport,
        capability_view: &'a HybridCapabilityView,
        capability_requirements: &'a [SemanticCapabilityRequirement],
        resource_constraints_complete: bool,
        capability_requirements_complete: bool,
    ) -> Self {
        Self {
            resource_requirements,
            resource_assessments,
            capability_view,
            capability_requirements,
            resource_constraints_complete,
            capability_requirements_complete,
        }
    }
}

/// Result of assessing hybrid resource and capability feasibility.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HybridFeasibilityReport {
    status: FeasibilityStatus,
    subject: String,
    findings: Vec<FeasibilityFinding>,
    capability_results: Vec<CapabilityFeasibilityAssessment>,
}

impl HybridFeasibilityReport {
    fn new(
        status: FeasibilityStatus,
        subject: String,
        findings: Vec<FeasibilityFinding>,
        capability_results: Vec<CapabilityFeasibilityAssessment>,
    ) -> Self {
        Self {
            status,
            subject,
            findings,
            capability_results,
        }
    }

    /// Returns the overall feasibility conclusion.
    #[must_use]
    pub const fn status(&self) -> FeasibilityStatus {
        self.status
    }

    /// Returns the evidence subject used for capability evaluation.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns the findings in deterministic evaluation order.
    ///
    /// Resource findings follow canonical assessment order. Capability
    /// findings follow the caller's capability-requirement order.
    #[must_use]
    pub fn findings(&self) -> &[FeasibilityFinding] {
        &self.findings
    }

    /// Returns individual capability assessment results in input order.
    #[must_use]
    pub fn capability_results(&self) -> &[CapabilityFeasibilityAssessment] {
        &self.capability_results
    }

    /// Returns whether the result establishes feasibility.
    #[must_use]
    pub const fn is_feasible(&self) -> bool {
        matches!(self.status, FeasibilityStatus::Feasible)
    }

    /// Returns whether the result establishes infeasibility.
    #[must_use]
    pub const fn is_infeasible(&self) -> bool {
        matches!(self.status, FeasibilityStatus::Infeasible)
    }

    /// Returns whether additional evidence or correction is needed before a
    /// definitive conclusion can be reached.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(
            self.status,
            FeasibilityStatus::Indeterminate | FeasibilityStatus::Invalid
        )
    }
}

/// Result for one required capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityFeasibilityAssessment {
    capability: String,
    evidence_result: CapabilityEvidenceResult,
}

impl CapabilityFeasibilityAssessment {
    fn new(
        capability: String,
        evidence_result: CapabilityEvidenceResult,
    ) -> Self {
        Self {
            capability,
            evidence_result,
        }
    }

    /// Returns the fully qualified capability identity.
    #[must_use]
    pub fn capability(&self) -> &str {
        &self.capability
    }

    /// Returns the canonical evidence evaluator's result.
    #[must_use]
    pub const fn evidence_result(&self) -> CapabilityEvidenceResult {
        self.evidence_result
    }
}

/// Stateless hybrid feasibility evaluator.
///
/// All evidence is supplied by the caller. This evaluator performs no I/O,
/// device discovery, target selection, allocation, or runtime actions.
#[derive(Clone, Copy, Debug, Default)]
pub struct HybridFeasibilityEvaluator;

impl HybridFeasibilityEvaluator {
    /// Creates a feasibility evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Assesses one hybrid computation using explicitly supplied evidence.
    ///
    /// A result is `Feasible` only when the supplied scopes are complete,
    /// every collected resource requirement has been assessed, no binding
    /// condition is violated, all resource assessments are decisive, and
    /// every required capability is supported by the scoped evidence.
    ///
    /// This method does not establish physical feasibility beyond the
    /// completeness and authority of the evidence supplied by its caller.
    #[must_use]
    pub fn evaluate(
        &self,
        input: HybridFeasibilityInput<'_>,
    ) -> HybridFeasibilityReport {
        let subject = input.capability_view.subject().as_str().to_owned();
        let mut findings = Vec::new();
        let mut capability_results = Vec::new();

        let mut proven_infeasible = false;
        let mut indeterminate = false;
        let mut invalid = false;

        if !input.resource_requirements.is_complete() {
            indeterminate = true;
            findings.push(FeasibilityFinding::new(
                FeasibilityFindingKind::ResourceRequirementsIncomplete,
                None,
                format!(
                    "resource requirement collection is incomplete; {} \
                     requirement(s) collected and {} upstream failure(s) recorded",
                    input.resource_requirements.len(),
                    input.resource_requirements.failure_count()
                ),
            ));
        }

        if !input.resource_constraints_complete {
            indeterminate = true;
            findings.push(FeasibilityFinding::new(
                FeasibilityFindingKind::ResourceConstraintsIncomplete,
                None,
                "upstream constraint extraction has not been established as complete",
            ));
        }

        if !input.capability_requirements_complete {
            indeterminate = true;
            findings.push(FeasibilityFinding::new(
                FeasibilityFindingKind::CapabilityRequirementsIncomplete,
                None,
                "upstream capability-requirement collection has not been established as complete",
            ));
        }

        let resource_report = input.resource_assessments;

        if resource_report.requirement_count() != input.resource_requirements.len() {
            indeterminate = true;
            findings.push(FeasibilityFinding::new(
                FeasibilityFindingKind::ResourceAssessmentCountMismatch,
                None,
                format!(
                    "collected {} resource requirement(s), but {} requirement assessment(s) were supplied",
                    input.resource_requirements.len(),
                    resource_report.requirement_count()
                ),
            ));
        }

        for assessment in resource_report
            .constraints()
            .assessments()
            .iter()
            .chain(resource_report.requirements().assessments().iter())
        {
            match assessment.outcome() {
                HybridConstraintOutcome::Satisfied
                | HybridConstraintOutcome::NotApplicable => {}

                HybridConstraintOutcome::Violated => {
                    if assessment.blocks_acceptance() {
                        proven_infeasible = true;
                        findings.push(FeasibilityFinding::new(
                            FeasibilityFindingKind::BindingResourceViolation,
                            Some(assessment.resource_kind().to_owned()),
                            assessment.explanation(),
                        ));
                    } else {
                        // An advisory violation is not a mandatory failure.
                        // It remains visible through the canonical report,
                        // but does not independently change feasibility.
                    }
                }

                HybridConstraintOutcome::Deferred => {
                    indeterminate = true;
                    findings.push(FeasibilityFinding::new(
                        FeasibilityFindingKind::ResourceAssessmentDeferred,
                        Some(assessment.resource_kind().to_owned()),
                        assessment.explanation(),
                    ));
                }

                HybridConstraintOutcome::Invalid => {
                    invalid = true;
                    findings.push(FeasibilityFinding::new(
                        FeasibilityFindingKind::ResourceAssessmentInvalid,
                        Some(assessment.resource_kind().to_owned()),
                        assessment.explanation(),
                    ));
                }

                _ => {
                    // The canonical outcome enum is non-exhaustive. Unknown
                    // future outcomes must never be treated as successful.
                    indeterminate = true;
                    findings.push(FeasibilityFinding::new(
                        FeasibilityFindingKind::ResourceAssessmentDeferred,
                        Some(assessment.resource_kind().to_owned()),
                        "unrecognized resource assessment outcome; feasibility is unresolved",
                    ));
                }
            }
        }

        if let Err(error) = input.capability_view.validate() {
            invalid = true;
            findings.push(FeasibilityFinding::new(
                FeasibilityFindingKind::CapabilityEvidenceInvalid,
                Some(subject.clone()),
                error.to_string(),
            ));
        } else {
            for requirement in input.capability_requirements {
                let result = input.capability_view.evaluate(requirement);
                let capability_name = requirement.capability().qualified_name();

                match result {
                    CapabilityEvidenceResult::Satisfied => {}

                    CapabilityEvidenceResult::NotSatisfied => {
                        proven_infeasible = true;
                        findings.push(FeasibilityFinding::new(
                            FeasibilityFindingKind::RequiredCapabilityUnsupported,
                            Some(capability_name.clone()),
                            "scoped evidence establishes that the required capability is unsupported",
                        ));
                    }

                    CapabilityEvidenceResult::NoEvidence => {
                        indeterminate = true;
                        findings.push(FeasibilityFinding::new(
                            FeasibilityFindingKind::CapabilityEvidenceMissing,
                            Some(capability_name.clone()),
                            "no applicable evidence was supplied for this required capability",
                        ));
                    }

                    CapabilityEvidenceResult::Indeterminate => {
                        indeterminate = true;
                        findings.push(FeasibilityFinding::new(
                            FeasibilityFindingKind::CapabilityEvidenceIndeterminate,
                            Some(capability_name.clone()),
                            "available capability evidence does not establish support or non-support",
                        ));
                    }

                    CapabilityEvidenceResult::ConflictingEvidence => {
                        indeterminate = true;
                        findings.push(FeasibilityFinding::new(
                            FeasibilityFindingKind::CapabilityEvidenceConflicting,
                            Some(capability_name.clone()),
                            "contradictory capability evidence exists for the same subject",
                        ));
                    }

                    _ => {
                        indeterminate = true;
                        findings.push(FeasibilityFinding::new(
                            FeasibilityFindingKind::CapabilityEvidenceIndeterminate,
                            Some(capability_name.clone()),
                            "unrecognized capability evidence result; feasibility is unresolved",
                        ));
                    }
                }

                capability_results.push(CapabilityFeasibilityAssessment::new(
                    capability_name,
                    result,
                ));
            }
        }

        let status = if proven_infeasible {
            FeasibilityStatus::Infeasible
        } else if invalid {
            FeasibilityStatus::Invalid
        } else if indeterminate {
            FeasibilityStatus::Indeterminate
        } else {
            findings.push(FeasibilityFinding::new(
                FeasibilityFindingKind::EvidenceSufficient,
                None,
                "all supplied complete resource and capability scopes satisfy their assessed conditions",
            ));
            FeasibilityStatus::Feasible
        };

        HybridFeasibilityReport::new(
            status,
            subject,
            findings,
            capability_results,
        )
    }
}

impl fmt::Display for FeasibilityStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Feasible => "feasible",
            Self::Infeasible => "infeasible",
            Self::Indeterminate => "indeterminate",
            Self::Invalid => "invalid",
        })
    }
}

/// Errors that may occur while assembling feasibility evidence.
///
/// This error type is reserved for future fallible construction paths. The
/// current evaluator reports invalid evidence as a structured report finding
/// instead of discarding partial results.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HybridFeasibilityError {
    /// A caller attempted to use inconsistent feasibility evidence.
    InconsistentEvidence(String),

    /// The supplied evidence failed validation.
    InvalidEvidence(String),
}

impl fmt::Display for HybridFeasibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InconsistentEvidence(message) => {
                write!(formatter, "inconsistent hybrid feasibility evidence: {message}")
            }
            Self::InvalidEvidence(message) => {
                write!(formatter, "invalid hybrid feasibility evidence: {message}")
            }
        }
    }
}

impl Error for HybridFeasibilityError {}

impl From<HybridCapabilityError> for HybridFeasibilityError {
    fn from(error: HybridCapabilityError) -> Self {
        Self::InvalidEvidence(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::semantic::capabilities::evidence::EvidenceSubject;

    #[test]
    fn incomplete_resource_requirements_never_prove_feasibility() {
        let mut requirements = HybridResourceRequirements::new();
        requirements.record_failure();

        let capability_view = HybridCapabilityView::new(
            EvidenceSubject::new("test-target")
                .expect("valid evidence subject"),
        );
        let assessments = HybridResourceConstraintReport::new();

        let input = HybridFeasibilityInput::new(
            &requirements,
            &assessments,
            &capability_view,
            &[],
            true,
            true,
        );

        let report = HybridFeasibilityEvaluator::new().evaluate(input);

        assert_eq!(report.status(), FeasibilityStatus::Indeterminate);
        assert!(!report.is_feasible());
    }

    #[test]
    fn incomplete_constraint_scope_never_proves_feasibility() {
        let requirements = HybridResourceRequirements::new();
        let capability_view = HybridCapabilityView::new(
            EvidenceSubject::new("test-target")
                .expect("valid evidence subject"),
        );
        let assessments = HybridResourceConstraintReport::new();

        let input = HybridFeasibilityInput::new(
            &requirements,
            &assessments,
            &capability_view,
            &[],
            false,
            true,
        );

        let report = HybridFeasibilityEvaluator::new().evaluate(input);

        assert_eq!(report.status(), FeasibilityStatus::Indeterminate);
    }

    #[test]
    fn missing_capability_evidence_never_proves_feasibility() {
        use crate::frontend::semantic::capabilities::model::{
            CapabilityId,
            CapabilityRequirement,
        };

        let requirements = HybridResourceRequirements::new();
        let capability_view = HybridCapabilityView::new(
            EvidenceSubject::new("test-target")
                .expect("valid evidence subject"),
        );
        let assessments = HybridResourceConstraintReport::new();
        let capability = CapabilityRequirement::new(
            CapabilityId::new("quantum", "measurement")
                .expect("valid capability identity"),
        );

        let input = HybridFeasibilityInput::new(
            &requirements,
            &assessments,
            &capability_view,
            &[capability],
            true,
            true,
        );

        let report = HybridFeasibilityEvaluator::new().evaluate(input);

        assert_eq!(report.status(), FeasibilityStatus::Indeterminate);
        assert_eq!(
            report.capability_results()[0].evidence_result(),
            CapabilityEvidenceResult::NoEvidence
        );
    }

    #[test]
    fn complete_empty_scope_can_be_feasible_when_declared_complete() {
        let requirements = HybridResourceRequirements::new();
        let capability_view = HybridCapabilityView::new(
            EvidenceSubject::new("test-target")
                .expect("valid evidence subject"),
        );
        let assessments = HybridResourceConstraintReport::new();

        let input = HybridFeasibilityInput::new(
            &requirements,
            &assessments,
            &capability_view,
            &[],
            true,
            true,
        );

        let report = HybridFeasibilityEvaluator::new().evaluate(input);

        assert_eq!(report.status(), FeasibilityStatus::Feasible);
        assert!(report.is_feasible());
    }
}
