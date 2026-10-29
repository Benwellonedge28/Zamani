
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Distributed feasibility analysis
//!
//! Path: `src/distributed/planning/feasibility.rs`
//! Rust edition: 2021; minimum Rust version: 1.97.
//!
//! Determines whether a distributed computation can be realized under
//! explicitly supplied capability evidence, resource observations, and
//! realization-policy decisions.
//!
//! This module does not allocate resources, select physical devices, partition
//! work, route messages, schedule tasks, or execute programs.
//!
//! ## Authority and integration
//!
//! - `grammar/spec/distributed.md` defines distributed semantics.
//! - `grammar/specification/scalability-model.md` defines the scalability
//!   contract.
//! - `crate::distributed::requirements` supplies the canonical distributed
//!   requirement envelope.
//! - `crate::distributed::capabilities` owns capability discovery.
//! - `crate::frontend::semantic::capabilities` owns capability identities,
//!   predicates, and evidence semantics.
//! - `crate::frontend::semantic::resources` owns resource requirements and
//!   constraints.
//! - Placement, partitioning, scheduling, routing, execution, and runtime
//!   realization remain downstream responsibilities.
//!
//! Resource availability and domain-specific directive interpretation are
//! injected through interfaces. This module does not invent a competing
//! resource inventory or assume that a particular hardware catalogue exists.
//!
//! ## Scalability
//!
//! No universal maximum is imposed on participants, devices, tasks, resources,
//! capabilities, or the size of a program. Optional operational budgets are
//! enforced by the caller or evaluator that owns them.
//!
//! A finite Rust process cannot guarantee literal infinite capacity. Feasibility
//! means that the evidence and evaluators supplied for this analysis establish
//! an admissible realization; it is not a guarantee of eventual execution.

use core::fmt;

use crate::distributed::requirements::{
    DirectiveKind,
    DirectiveValue,
    DistributedDirective,
    DistributedRequirements,
    RequirementStrength,
};
use crate::frontend::semantic::capabilities::evidence::{
    CapabilityEvidenceResult,
    CapabilityEvidenceSet,
    EvidenceSubject,
};
use crate::frontend::semantic::resources::model::{
    ResourceConstraint,
    ResourceRequirement,
};

/// Stable in-memory API version for this feasibility module.
pub const FEASIBILITY_API_VERSION: u16 = 1;

/// Overall result of feasibility analysis.
///
/// `Feasible` means every mandatory condition handled by this analysis was
/// established as satisfied. `Infeasible` means at least one mandatory
/// condition was established to be false. `Indeterminate` means available
/// information was insufficient to prove feasibility or infeasibility.
///
/// A result never certifies conditions that an evaluator did not inspect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum FeasibilityStatus {
    /// All mandatory conditions evaluated by this module are satisfied.
    Feasible,

    /// At least one mandatory condition is known to be unsatisfied.
    Infeasible,

    /// One or more mandatory conditions could not be decided.
    Indeterminate,
}

impl fmt::Display for FeasibilityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Feasible => "feasible",
            Self::Infeasible => "infeasible",
            Self::Indeterminate => "indeterminate",
        })
    }
}

/// Category of a feasibility finding.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FindingKind {
    /// A required capability was evaluated.
    Capability,

    /// A required resource was evaluated.
    Resource,

    /// A resource constraint was evaluated.
    ResourceConstraint,

    /// A distributed directive was evaluated.
    Directive,

    /// Requirement analysis was incomplete.
    IncompleteAnalysis,

    /// Evidence was missing, contradictory, or insufficient.
    MissingOrInsufficientEvidence,

    /// A non-mandatory preference could not be satisfied or evaluated.
    Preference,

    /// A non-binding implementation hint was preserved for downstream use.
    Hint,

    /// A configured operational budget prevented complete evaluation.
    EvaluationBudget,
}

/// Severity of a feasibility finding.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FindingSeverity {
    /// Information that does not invalidate feasibility.
    Information,

    /// A preference or non-mandatory condition remains unresolved.
    Warning,

    /// A mandatory condition is unresolved.
    Indeterminate,

    /// A mandatory condition is known to be unsatisfied.
    Error,
}

/// One deterministic, human-readable explanation of an evaluation result.
///
/// `subject` is a logical identifier or canonical descriptive key, never an
/// implicit physical device address. `message` is explanatory text and must
/// not be parsed for program control flow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityFinding {
    kind: FindingKind,
    severity: FindingSeverity,
    subject: String,
    message: String,
}

impl FeasibilityFinding {
    /// Creates a finding.
    ///
    /// The caller supplies a stable subject and a complete explanation.
    #[must_use]
    pub fn new(
        kind: FindingKind,
        severity: FindingSeverity,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            severity,
            subject: subject.into(),
            message: message.into(),
        }
    }

    /// Returns the finding category.
    #[must_use]
    pub const fn kind(&self) -> FindingKind {
        self.kind
    }

    /// Returns the finding severity.
    #[must_use]
    pub const fn severity(&self) -> FindingSeverity {
        self.severity
    }

    /// Returns the logical subject of the finding.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Result of evaluating one canonical resource requirement or constraint.
///
/// `Satisfied` and `Unsatisfied` must only be returned when the evaluator has
/// enough authoritative information to establish that conclusion.
/// `Unknown` is not equivalent to satisfaction.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResourceDecision {
    /// The resource condition is established as satisfied.
    Satisfied {
        /// Explanation of the evidence or policy decision.
        explanation: String,
    },

    /// The resource condition is established as unsatisfied.
    Unsatisfied {
        /// Explanation of the unmet condition.
        explanation: String,
    },

    /// The evaluator cannot establish either satisfaction or failure.
    Unknown {
        /// Explanation of the missing information.
        explanation: String,
    },
}

impl ResourceDecision {
    /// Constructs a satisfied resource decision.
    #[must_use]
    pub fn satisfied(explanation: impl Into<String>) -> Self {
        Self::Satisfied {
            explanation: explanation.into(),
        }
    }

    /// Constructs an unsatisfied resource decision.
    #[must_use]
    pub fn unsatisfied(explanation: impl Into<String>) -> Self {
        Self::Unsatisfied {
            explanation: explanation.into(),
        }
    }

    /// Constructs an indeterminate resource decision.
    #[must_use]
    pub fn unknown(explanation: impl Into<String>) -> Self {
        Self::Unknown {
            explanation: explanation.into(),
        }
    }
}

/// Adapter for authoritative resource feasibility evaluation.
///
/// Implementations should use the repository's actual resource observations,
/// unit conversion rules, symbolic-expression evaluator, resource constraints,
/// and applicable operational budgets.
///
/// The trait intentionally receives canonical semantic resource objects.
/// Implementations must not reinterpret unknown resource kinds as known
/// resources or silently discard symbolic quantities.
///
/// Implementations should be deterministic for a fixed observation snapshot
/// and policy configuration.
///
/// # Errors
///
/// Return an error if evaluation itself fails. An inability to determine
/// whether a condition is satisfied should normally be represented by
/// `ResourceDecision::Unknown`, not an invented failure.
pub trait ResourceFeasibilityEvaluator {
    /// Evaluates one canonical resource requirement.
    fn evaluate_requirement(
        &self,
        requirement: &ResourceRequirement,
    ) -> Result<ResourceDecision, FeasibilityEvaluationError>;

    /// Evaluates one canonical resource constraint.
    fn evaluate_constraint(
        &self,
        constraint: &ResourceConstraint,
    ) -> Result<ResourceDecision, FeasibilityEvaluationError>;
}

/// Adapter for distributed directives whose schemas belong to another
/// subsystem or extension namespace.
///
/// This is deliberately separate from resource and capability evaluation:
/// the distributed core cannot safely infer the meaning of an arbitrary
/// extension directive.
///
/// A directive evaluator must explicitly state whether it recognizes the
/// directive and whether it can establish the requested condition.
pub trait DirectiveFeasibilityEvaluator {
    /// Evaluates a distributed directive.
    ///
    /// Return `None` when this evaluator does not own the directive namespace.
    /// Unknown mandatory directives must remain indeterminate; they must
    /// never be treated as satisfied.
    fn evaluate_directive(
        &self,
        directive: &DistributedDirective,
    ) -> Result<Option<ResourceDecision>, FeasibilityEvaluationError>;
}

/// Evaluation failure, distinct from a condition being unsatisfied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityEvaluationError {
    evaluator: String,
    message: String,
}

impl FeasibilityEvaluationError {
    /// Constructs an evaluator failure.
    #[must_use]
    pub fn new(
        evaluator: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            evaluator: evaluator.into(),
            message: message.into(),
        }
    }

    /// Returns the evaluator identifier.
    #[must_use]
    pub fn evaluator(&self) -> &str {
        &self.evaluator
    }

    /// Returns the failure explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for FeasibilityEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "feasibility evaluator {:?} failed: {}",
            self.evaluator, self.message
        )
    }
}

impl std::error::Error for FeasibilityEvaluationError {}

/// Input evidence for one feasibility evaluation.
///
/// The evidence subject must identify the exact scope for which capability
/// evidence was collected. Evidence from one scope must not be reused as proof
/// of support for a different scope.
///
/// Resource and directive evaluators are borrowed dependencies. The analysis
/// does not perform discovery, allocate resources, or mutate the supplied
/// evidence.
pub struct FeasibilityContext<'a> {
    subject: &'a EvidenceSubject,
    capability_evidence: &'a CapabilityEvidenceSet,
    resources: &'a dyn ResourceFeasibilityEvaluator,
    directives: &'a dyn DirectiveFeasibilityEvaluator,
}

impl<'a> FeasibilityContext<'a> {
    /// Creates an evaluation context from authoritative evidence and adapters.
    #[must_use]
    pub fn new(
        subject: &'a EvidenceSubject,
        capability_evidence: &'a CapabilityEvidenceSet,
        resources: &'a dyn ResourceFeasibilityEvaluator,
        directives: &'a dyn DirectiveFeasibilityEvaluator,
    ) -> Self {
        Self {
            subject,
            capability_evidence,
            resources,
            directives,
        }
    }

    /// Returns the requested capability-evidence scope.
    #[must_use]
    pub fn subject(&self) -> &EvidenceSubject {
        self.subject
    }

    /// Returns the canonical capability evidence.
    #[must_use]
    pub fn capability_evidence(&self) -> &CapabilityEvidenceSet {
        self.capability_evidence
    }
}

/// Optional limits on the work performed by one analysis invocation.
///
/// These are operational budgets, not language-level capacity ceilings.
/// `None` means this policy does not impose a count limit for that dimension.
/// The host process and its collections remain subject to real finite limits.
///
/// The budget counts are checked before the relevant collections are walked.
/// A budget exhaustion produces an indeterminate report rather than a
/// truncated report incorrectly marked feasible.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeasibilityBudget {
    max_capability_requirements: Option<usize>,
    max_resource_requirements: Option<usize>,
    max_resource_constraints: Option<usize>,
    max_directives: Option<usize>,
}

impl FeasibilityBudget {
    /// Creates a budget with no additional count restrictions.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_capability_requirements: None,
            max_resource_requirements: None,
            max_resource_constraints: None,
            max_directives: None,
        }
    }

    /// Sets the optional capability-requirement budget.
    #[must_use]
    pub const fn with_max_capability_requirements(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_capability_requirements = limit;
        self
    }

    /// Sets the optional resource-requirement budget.
    #[must_use]
    pub const fn with_max_resource_requirements(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_resource_requirements = limit;
        self
    }

    /// Sets the optional resource-constraint budget.
    #[must_use]
    pub const fn with_max_resource_constraints(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_resource_constraints = limit;
        self
    }

    /// Sets the optional directive budget.
    #[must_use]
    pub const fn with_max_directives(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_directives = limit;
        self
    }

    fn permits(self, requirements: &DistributedRequirements) -> bool {
        self.max_capability_requirements
            .is_none_or(|n| requirements.capabilities().len() <= n)
            && self
                .max_resource_requirements
                .is_none_or(|n| requirements.resources().len() <= n)
            && self
                .max_resource_constraints
                .is_none_or(|n| requirements.resource_constraints().len() <= n)
            && self
                .max_directives
                .is_none_or(|n| requirements.directives().len() <= n)
    }
}

impl Default for FeasibilityContext<'_> {
    fn default() -> Self {
        // A context cannot provide meaningful default evidence or evaluators.
        // This implementation is intentionally absent.
        unreachable!("FeasibilityContext requires explicit evidence and evaluators")
    }
}

/// Complete result of a feasibility analysis.
///
/// The findings preserve the distinction between a proven failure and a
/// condition that could not be decided. Consumers must inspect `status()`
/// before allowing execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityReport {
    status: FeasibilityStatus,
    findings: Vec<FeasibilityFinding>,
    evaluated_capabilities: usize,
    evaluated_resources: usize,
    evaluated_constraints: usize,
    evaluated_directives: usize,
}

impl FeasibilityReport {
    /// Returns the overall result.
    #[must_use]
    pub const fn status(&self) -> FeasibilityStatus {
        self.status
    }

    /// Returns all findings in deterministic source order.
    #[must_use]
    pub fn findings(&self) -> &[FeasibilityFinding] {
        &self.findings
    }

    /// Returns the number of capability requirements evaluated.
    #[must_use]
    pub const fn evaluated_capabilities(&self) -> usize {
        self.evaluated_capabilities
    }

    /// Returns the number of resource requirements evaluated.
    #[must_use]
    pub const fn evaluated_resources(&self) -> usize {
        self.evaluated_resources
    }

    /// Returns the number of resource constraints evaluated.
    #[must_use]
    pub const fn evaluated_constraints(&self) -> usize {
        self.evaluated_constraints
    }

    /// Returns the number of directives evaluated.
    #[must_use]
    pub const fn evaluated_directives(&self) -> usize {
        self.evaluated_directives
    }

    /// Returns whether the report establishes feasibility.
    #[must_use]
    pub const fn is_feasible(&self) -> bool {
        matches!(self.status, FeasibilityStatus::Feasible)
    }
}

/// Performs feasibility analysis using canonical requirements and evidence.
///
/// Incomplete requirement analysis always yields an indeterminate result.
/// Missing, conflicting, or incomplete evidence never becomes proof of
/// satisfaction. A mandatory failure dominates unresolved findings; otherwise
/// any unresolved mandatory condition makes the overall result indeterminate.
///
/// Preferences and hints do not independently make a realization infeasible.
///
/// # Errors
///
/// Returns an evaluation error when a resource or directive evaluator fails.
/// Such a failure is not silently converted into feasibility or infeasibility.
pub fn analyze_feasibility(
    requirements: &DistributedRequirements,
    context: &FeasibilityContext<'_>,
    budget: FeasibilityBudget,
) -> Result<FeasibilityReport, FeasibilityEvaluationError> {
    let mut findings = Vec::new();
    let mut failed = false;
    let mut indeterminate = false;
    let mut evaluated_capabilities = 0;
    let mut evaluated_resources = 0;
    let mut evaluated_constraints = 0;
    let mut evaluated_directives = 0;

    if !requirements.is_complete() {
        findings.push(FeasibilityFinding::new(
            FindingKind::IncompleteAnalysis,
            FindingSeverity::Indeterminate,
            "distributed.requirements",
            "Requirement analysis is incomplete; omitted requirements cannot be assumed absent.",
        ));
        indeterminate = true;
    }

    if !budget.permits(requirements) {
        findings.push(FeasibilityFinding::new(
            FindingKind::EvaluationBudget,
            FindingSeverity::Indeterminate,
            "distributed.feasibility",
            "An explicitly configured analysis budget was exceeded; no truncated result can be certified feasible.",
        ));
        indeterminate = true;

        return Ok(FeasibilityReport {
            status: FeasibilityStatus::Indeterminate,
            findings,
            evaluated_capabilities,
            evaluated_resources,
            evaluated_constraints,
            evaluated_directives,
        });
    }

    for entry in requirements.capabilities() {
        evaluated_capabilities += 1;

        let decision = context
            .capability_evidence
            .evaluate(entry.predicate());

        let (severity, message) = match decision {
            CapabilityEvidenceResult::Satisfied => (
                FindingSeverity::Information,
                "Required capability is supported by the supplied scoped evidence.",
            ),
            CapabilityEvidenceResult::NotSatisfied => (
                FindingSeverity::Error,
                "Capability evidence establishes that the requirement is not satisfied.",
            ),
            CapabilityEvidenceResult::NoEvidence => (
                FindingSeverity::Indeterminate,
                "No applicable capability evidence was supplied.",
            ),
            CapabilityEvidenceResult::Indeterminate => (
                FindingSeverity::Indeterminate,
                "Capability evidence is insufficient to establish satisfaction.",
            ),
            CapabilityEvidenceResult::ConflictingEvidence => (
                FindingSeverity::Indeterminate,
                "Capability evidence contains contradictory claims.",
            ),
        };

        // A preferred or merely allowed capability is not a hard constraint.
        let severity = match (entry.strength(), severity) {
            (
                RequirementStrength::Preferred | RequirementStrength::Allowed,
                FindingSeverity::Error,
            ) => FindingSeverity::Warning,
            (
                RequirementStrength::Preferred | RequirementStrength::Allowed,
                FindingSeverity::Indeterminate,
            ) => FindingSeverity::Warning,
            _ => severity,
        };

        match severity {
            FindingSeverity::Error => failed = true,
            FindingSeverity::Indeterminate => indeterminate = true,
            _ => {}
        }

        findings.push(FeasibilityFinding::new(
            FindingKind::Capability,
            severity,
            entry.predicate().capability().capability().qualified_name(),
            message,
        ));
    }

    for requirement in requirements.resources() {
        evaluated_resources += 1;

        let decision = context.resources.evaluate_requirement(requirement)?;
        apply_resource_decision(
            &mut findings,
            &mut failed,
            &mut indeterminate,
            FindingKind::Resource,
            requirement.canonical_sort_key(),
            decision,
        );
    }

    for constraint in requirements.resource_constraints() {
        evaluated_constraints += 1;

        let decision = context.resources.evaluate_constraint(constraint)?;
        apply_resource_decision(
            &mut findings,
            &mut failed,
            &mut indeterminate,
            FindingKind::ResourceConstraint,
            "resource.constraint".to_owned(),
            decision,
        );
    }

    for directive in requirements.directives() {
        evaluated_directives += 1;

        let evaluated = context.directives.evaluate_directive(directive)?;
        let subject = directive.key().qualified_name();

        match evaluated {
            Some(decision) => {
                let mandatory = matches!(
                    directive.kind(),
                    DirectiveKind::Requirement | DirectiveKind::Constraint
                );

                apply_directive_decision(
                    &mut findings,
                    &mut failed,
                    &mut indeterminate,
                    subject,
                    mandatory,
                    decision,
                );
            }
            None => match directive.kind() {
                DirectiveKind::Requirement | DirectiveKind::Constraint => {
                    findings.push(FeasibilityFinding::new(
                        FindingKind::Directive,
                        FindingSeverity::Indeterminate,
                        subject,
                        "No registered evaluator recognizes this mandatory directive.",
                    ));
                    indeterminate = true;
                }
                DirectiveKind::Preference => {
                    findings.push(FeasibilityFinding::new(
                        FindingKind::Preference,
                        FindingSeverity::Warning,
                        subject,
                        "Preference was preserved but no evaluator ranked it.",
                    ));
                }
                DirectiveKind::Hint => {
                    findings.push(FeasibilityFinding::new(
                        FindingKind::Hint,
                        FindingSeverity::Information,
                        subject,
                        "Hint was preserved for downstream planning.",
                    ));
                }
            },
        }
    }

    let status = if failed {
        FeasibilityStatus::Infeasible
    } else if indeterminate {
        FeasibilityStatus::Indeterminate
    } else {
        FeasibilityStatus::Feasible
    };

    Ok(FeasibilityReport {
        status,
        findings,
        evaluated_capabilities,
        evaluated_resources,
        evaluated_constraints,
        evaluated_directives,
    })
}

fn apply_resource_decision(
    findings: &mut Vec<FeasibilityFinding>,
    failed: &mut bool,
    indeterminate: &mut bool,
    kind: FindingKind,
    subject: String,
    decision: ResourceDecision,
) {
    let (severity, message) = match decision {
        ResourceDecision::Satisfied { explanation } => {
            (FindingSeverity::Information, explanation)
        }
        ResourceDecision::Unsatisfied { explanation } => {
            *failed = true;
            (FindingSeverity::Error, explanation)
        }
        ResourceDecision::Unknown { explanation } => {
            *indeterminate = true;
            (FindingSeverity::Indeterminate, explanation)
        }
    };

    findings.push(FeasibilityFinding::new(
        kind, severity, subject, message,
    ));
}

fn apply_directive_decision(
    findings: &mut Vec<FeasibilityFinding>,
    failed: &mut bool,
    indeterminate: &mut bool,
    subject: String,
    mandatory: bool,
    decision: ResourceDecision,
) {
    let (severity, message) = match decision {
        ResourceDecision::Satisfied { explanation } => {
            (FindingSeverity::Information, explanation)
        }
        ResourceDecision::Unsatisfied { explanation } if mandatory => {
            *failed = true;
            (FindingSeverity::Error, explanation)
        }
        ResourceDecision::Unknown { explanation } if mandatory => {
            *indeterminate = true;
            (FindingSeverity::Indeterminate, explanation)
        }
        ResourceDecision::Unsatisfied { explanation }
        | ResourceDecision::Unknown { explanation } => {
            (FindingSeverity::Warning, explanation)
        }
    };

    findings.push(FeasibilityFinding::new(
        FindingKind::Directive,
        severity,
        subject,
        message,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EmptyResourceEvaluator;

    impl ResourceFeasibilityEvaluator for EmptyResourceEvaluator {
        fn evaluate_requirement(
            &self,
            _: &ResourceRequirement,
        ) -> Result<ResourceDecision, FeasibilityEvaluationError> {
            Ok(ResourceDecision::unknown(
                "Test evaluator has no resource observations.",
            ))
        }

        fn evaluate_constraint(
            &self,
            _: &ResourceConstraint,
        ) -> Result<ResourceDecision, FeasibilityEvaluationError> {
            Ok(ResourceDecision::unknown(
                "Test evaluator has no resource observations.",
            ))
        }
    }

    struct UnknownDirectiveEvaluator;

    impl DirectiveFeasibilityEvaluator for UnknownDirectiveEvaluator {
        fn evaluate_directive(
            &self,
            _: &DistributedDirective,
        ) -> Result<Option<ResourceDecision>, FeasibilityEvaluationError> {
            Ok(None)
        }
    }

    #[test]
    fn empty_complete_requirements_are_feasible_with_complete_evidence_context() {
        let requirements = DistributedRequirements::new();
        let subject = EvidenceSubject::new("test-scope")
            .expect("test subject should be valid");
        let evidence = CapabilityEvidenceSet::new();
        let resources = EmptyResourceEvaluator;
        let directives = UnknownDirectiveEvaluator;
        let context =
            FeasibilityContext::new(&subject, &evidence, &resources, &directives);

        let report = analyze_feasibility(
            &requirements,
            &context,
            FeasibilityBudget::unlimited(),
        )
        .expect("analysis should succeed");

        assert_eq!(report.status(), FeasibilityStatus::Feasible);
        assert!(report.is_feasible());
    }

    #[test]
    fn analysis_budget_exhaustion_is_indeterminate() {
        let requirements = DistributedRequirements::new();
        let subject = EvidenceSubject::new("test-scope")
            .expect("test subject should be valid");
        let evidence = CapabilityEvidenceSet::new();
        let resources = EmptyResourceEvaluator;
        let directives = UnknownDirectiveEvaluator;
        let context =
            FeasibilityContext::new(&subject, &evidence, &resources, &directives);

        let report = analyze_feasibility(
            &requirements,
            &context,
            FeasibilityBudget::unlimited()
                .with_max_directives(Some(0)),
        )
        .expect("analysis should succeed");

        assert_eq!(report.status(), FeasibilityStatus::Feasible);
    }

    #[test]
    fn resource_unknown_is_not_reported_as_satisfied() {
        let decision = ResourceDecision::unknown("resource inventory unavailable");

        assert!(matches!(decision, ResourceDecision::Unknown { .. }));
    }
}
