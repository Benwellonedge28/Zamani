
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Classical Resource Validation
//!
//! Path: `src/classical/validation/resources.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Validate the resource-related inputs to classical execution planning
//! without duplicating the canonical frontend semantic resource model.
//!
//! This module checks:
//!
//! - whether requirement extraction completed successfully;
//! - explicit operational budgets for validation work;
//! - each available canonical resource requirement;
//! - each available canonical resource constraint;
//! - whether validation completed fully or only partially;
//! - whether errors and budget exhaustion were reported explicitly.
//!
//! ## Authority
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/resources/requirements.rs`
//! - `src/classical/execution/requirements.rs`
//!
//! The grammar owns syntax. The frontend semantic model owns canonical
//! resource meaning. The execution-requirements aggregate owns collected
//! requirement data and its extraction-completeness status.
//!
//! ## Ownership
//!
//! This module owns validation policy, validation findings, validation
//! completeness reporting, and the orchestration of resource validation.
//!
//! It does not own resource kinds, quantities, units, symbolic expressions,
//! constraints, capability identities, hardware inventories, target
//! feasibility, allocation, placement, routing, scheduling, or execution.
//!
//! ## Scalability
//!
//! There are no universal resource-count or machine-capacity ceilings.
//! Optional limits in `ResourceValidationPolicy` are operational work
//! budgets, not restrictions on valid Zamani programs.
//!
//! An absent budget means this layer imposes no additional count budget.
//! It does not promise infinite memory or infinite execution time.
//!
//! Resource quantities remain in their canonical semantic representation.
//! This module never narrows them to `usize`, `u64`, or `u128`.
//!
//! ## Determinism
//!
//! Findings follow deterministic input order: completeness findings first,
//! followed by resource requirements and then resource constraints.
//! Entries are not deduplicated. Validators must themselves be deterministic
//! for the same input and semantic environment.
//!
//! ## Integration
//!
//! Callers supply a `ResourceSemanticsValidator` backed by the canonical
//! semantic constraint and resource-validation services. This avoids
//! duplicating semantic rules or guessing the shape of canonical models.
//!
//! This module performs no hardware discovery, I/O, global mutation,
//! allocation, scheduling, or runtime execution.

use core::fmt;
use std::error::Error;

use crate::classical::execution::requirements::ExecutionRequirements;
use crate::frontend::semantic::resources::model::{
    ResourceConstraint,
    ResourceRequirement,
};

/// Result type for resource validation operations.
pub type ResourceValidationResult<T> = Result<T, ResourceValidationError>;

/// Explicit operational budgets for resource validation.
///
/// These limits control the amount of validation work performed by this
/// implementation. They are not language-level limits and must not be used
/// to reject a program as semantically invalid merely because it is large.
///
/// `None` means this policy imposes no additional count budget. Callers may
/// configure budgets from application policy, compilation budgets, or other
/// shared operational configuration.
///
/// A budget is checked before validating entries beyond that budget. If a
/// budget is exceeded, the report is incomplete and must not be interpreted
/// as proof that every resource declaration was validated.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResourceValidationPolicy {
    max_resource_requirements: Option<usize>,
    max_resource_constraints: Option<usize>,
}

impl ResourceValidationPolicy {
    /// Create a policy without additional count budgets.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_resource_requirements: None,
            max_resource_constraints: None,
        }
    }

    /// Create a policy with optional requirement and constraint budgets.
    ///
    /// The values are operational limits for this validation invocation.
    /// They do not alter the semantics or validity of the source program.
    #[must_use]
    pub const fn new(
        max_resource_requirements: Option<usize>,
        max_resource_constraints: Option<usize>,
    ) -> Self {
        Self {
            max_resource_requirements,
            max_resource_constraints,
        }
    }

    /// Return the maximum number of requirements this invocation may inspect.
    #[must_use]
    pub const fn max_resource_requirements(self) -> Option<usize> {
        self.max_resource_requirements
    }

    /// Return the maximum number of constraints this invocation may inspect.
    #[must_use]
    pub const fn max_resource_constraints(self) -> Option<usize> {
        self.max_resource_constraints
    }
}

/// Stable classification of a resource-validation finding.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResourceValidationIssueKind {
    /// Requirement extraction or semantic preparation was incomplete.
    IncompleteRequirements,

    /// The configured requirement-validation budget was exhausted.
    RequirementBudgetExceeded,

    /// The configured constraint-validation budget was exhausted.
    ConstraintBudgetExceeded,

    /// A canonical resource requirement failed semantic validation.
    InvalidRequirement,

    /// A canonical resource constraint failed semantic validation.
    InvalidConstraint,
}

impl ResourceValidationIssueKind {
    /// Return the stable machine-readable issue code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncompleteRequirements => {
                "classical.resources.incomplete_requirements"
            }
            Self::RequirementBudgetExceeded => {
                "classical.resources.requirement_budget_exceeded"
            }
            Self::ConstraintBudgetExceeded => {
                "classical.resources.constraint_budget_exceeded"
            }
            Self::InvalidRequirement => {
                "classical.resources.invalid_requirement"
            }
            Self::InvalidConstraint => {
                "classical.resources.invalid_constraint"
            }
        }
    }
}

/// Identifies the collection and position associated with a finding.
///
/// Indices are zero-based positions in the original input collections.
/// They are not resource IDs, source offsets, or target-specific identifiers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResourceValidationSubject {
    /// A finding about requirement collection completeness.
    RequirementCollection,

    /// A finding about a resource requirement at the specified input index.
    Requirement {
        /// Zero-based position in the requirement collection.
        index: usize,
    },

    /// A finding about a resource constraint at the specified input index.
    Constraint {
        /// Zero-based position in the constraint collection.
        index: usize,
    },
}

/// One structured resource-validation finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceValidationIssue {
    kind: ResourceValidationIssueKind,
    subject: ResourceValidationSubject,
    message: String,
}

impl ResourceValidationIssue {
    /// Construct a structured finding.
    #[must_use]
    pub fn new(
        kind: ResourceValidationIssueKind,
        subject: ResourceValidationSubject,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject,
            message: message.into(),
        }
    }

    /// Return the stable finding classification.
    #[must_use]
    pub const fn kind(&self) -> ResourceValidationIssueKind {
        self.kind
    }

    /// Return the stable machine-readable code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.as_str()
    }

    /// Return the affected collection or entry.
    #[must_use]
    pub const fn subject(&self) -> ResourceValidationSubject {
        self.subject
    }

    /// Return the explanatory diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Whether validation covered all entries in the supplied aggregate.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ResourceValidationCompleteness {
    /// All entries within the supplied aggregate were examined.
    ///
    /// This does not by itself mean that validation succeeded: findings may
    /// still report invalid requirements or constraints.
    #[default]
    Complete,

    /// Validation did not cover the full aggregate.
    ///
    /// This includes incomplete upstream requirement preparation and
    /// operational budget exhaustion.
    Incomplete,
}

/// Result of validating resource requirements and constraints.
///
/// A report separates completeness from validity. An incomplete report must
/// never be treated as proof that all resource inputs are valid.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceValidationReport {
    completeness: ResourceValidationCompleteness,
    issues: Vec<ResourceValidationIssue>,
    requirements_examined: usize,
    constraints_examined: usize,
}

impl ResourceValidationReport {
    /// Create an empty report whose completeness is initially complete.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the report's completeness state.
    #[must_use]
    pub const fn completeness(&self) -> ResourceValidationCompleteness {
        self.completeness
    }

    /// Return whether every supplied collection entry was examined.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(
            self.completeness,
            ResourceValidationCompleteness::Complete
        )
    }

    /// Return whether validation completed without any findings.
    ///
    /// A successful result means this validation invocation found no issues.
    /// It does not establish that a target has the resources needed to execute
    /// the program.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.is_complete() && self.issues.is_empty()
    }

    /// Return all findings in deterministic validation order.
    #[must_use]
    pub fn issues(&self) -> &[ResourceValidationIssue] {
        &self.issues
    }

    /// Return the number of findings.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Return the number of resource requirements actually examined.
    #[must_use]
    pub const fn requirements_examined(&self) -> usize {
        self.requirements_examined
    }

    /// Return the number of resource constraints actually examined.
    #[must_use]
    pub const fn constraints_examined(&self) -> usize {
        self.constraints_examined
    }

    /// Return whether a finding of the specified kind exists.
    #[must_use]
    pub fn has_issue(&self, kind: ResourceValidationIssueKind) -> bool {
        self.issues.iter().any(|issue| issue.kind() == kind)
    }

    fn mark_incomplete(&mut self) {
        self.completeness = ResourceValidationCompleteness::Incomplete;
    }

    fn push_issue(&mut self, issue: ResourceValidationIssue) {
        self.issues.push(issue);
    }

    fn examined_requirement(&mut self) {
        self.requirements_examined += 1;
    }

    fn examined_constraint(&mut self) {
        self.constraints_examined += 1;
    }
}

/// Validates canonical resource requirements and constraints.
///
/// Implementations must delegate to the repository's authoritative semantic
/// resource and constraint services. They must not query target availability
/// or reinterpret a resource requirement as a physical allocation.
///
/// Returning `Err` means the supplied semantic value failed validation.
/// Error messages are diagnostic text; consumers should use the structured
/// issue kind rather than parsing messages.
///
/// Implementations should be deterministic for equivalent inputs and semantic
/// environments. They should not silently accept unknown or malformed
/// semantic values.
pub trait ResourceSemanticsValidator {
    /// Validate one canonical resource requirement.
    fn validate_requirement(
        &self,
        requirement: &ResourceRequirement,
    ) -> Result<(), String>;

    /// Validate one canonical resource constraint.
    fn validate_constraint(
        &self,
        constraint: &ResourceConstraint,
    ) -> Result<(), String>;
}

/// Validate the resource inputs in an execution-requirements aggregate.
///
/// The validator examines entries in input order, preserving duplicate
/// declarations. It continues after individual semantic failures so callers
/// can receive multiple useful findings in one report.
///
/// Upstream incompleteness is reported explicitly. Any operational budget
/// exceeded makes the report incomplete; unexamined entries are not claimed
/// to have been validated.
///
/// This function does not establish target feasibility, resolve symbolic
/// expressions, allocate resources, or make scheduling decisions.
#[must_use]
pub fn validate_resource_requirements<V>(
    requirements: &ExecutionRequirements,
    policy: ResourceValidationPolicy,
    validator: &V,
) -> ResourceValidationReport
where
    V: ResourceSemanticsValidator + ?Sized,
{
    let mut report = ResourceValidationReport::new();

    if !requirements.is_complete() {
        report.mark_incomplete();
        report.push_issue(ResourceValidationIssue::new(
            ResourceValidationIssueKind::IncompleteRequirements,
            ResourceValidationSubject::RequirementCollection,
            format!(
                "Resource requirement preparation is incomplete: {} upstream issue(s) recorded.",
                requirements.issue_count()
            ),
        ));
    }

    validate_requirements(
        requirements,
        policy,
        validator,
        &mut report,
    );

    validate_constraints(
        requirements,
        policy,
        validator,
        &mut report,
    );

    report
}

fn validate_requirements<V>(
    requirements: &ExecutionRequirements,
    policy: ResourceValidationPolicy,
    validator: &V,
    report: &mut ResourceValidationReport,
)
where
    V: ResourceSemanticsValidator + ?Sized,
{
    let entries = requirements.resource_requirements();
    let allowed = policy.max_resource_requirements();

    if let Some(limit) = allowed {
        if entries.len() > limit {
            report.mark_incomplete();
            report.push_issue(ResourceValidationIssue::new(
                ResourceValidationIssueKind::RequirementBudgetExceeded,
                ResourceValidationSubject::RequirementCollection,
                format!(
                    "Resource requirement validation budget exhausted: \
                     {} entries supplied, but this invocation may examine {limit}.",
                    entries.len()
                ),
            ));
        }
    }

    let count = allowed.map_or(entries.len(), |limit| entries.len().min(limit));

    for (index, requirement) in entries.iter().take(count).enumerate() {
        report.examined_requirement();

        if let Err(message) = validator.validate_requirement(requirement) {
            report.push_issue(ResourceValidationIssue::new(
                ResourceValidationIssueKind::InvalidRequirement,
                ResourceValidationSubject::Requirement { index },
                message,
            ));
        }
    }
}

fn validate_constraints<V>(
    requirements: &ExecutionRequirements,
    policy: ResourceValidationPolicy,
    validator: &V,
    report: &mut ResourceValidationReport,
)
where
    V: ResourceSemanticsValidator + ?Sized,
{
    let entries = requirements.resource_constraints();
    let allowed = policy.max_resource_constraints();

    if let Some(limit) = allowed {
        if entries.len() > limit {
            report.mark_incomplete();
            report.push_issue(ResourceValidationIssue::new(
                ResourceValidationIssueKind::ConstraintBudgetExceeded,
                ResourceValidationSubject::RequirementCollection,
                format!(
                    "Resource constraint validation budget exhausted: \
                     {} entries supplied, but this invocation may examine {limit}.",
                    entries.len()
                ),
            ));
        }
    }

    let count = allowed.map_or(entries.len(), |limit| entries.len().min(limit));

    for (index, constraint) in entries.iter().take(count).enumerate() {
        report.examined_constraint();

        if let Err(message) = validator.validate_constraint(constraint) {
            report.push_issue(ResourceValidationIssue::new(
                ResourceValidationIssueKind::InvalidConstraint,
                ResourceValidationSubject::Constraint { index },
                message,
            ));
        }
    }
}

/// Errors returned when a caller requires a complete, successful report.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResourceValidationError {
    /// Validation was incomplete because an upstream or operational budget
    /// prevented complete examination.
    Incomplete {
        /// Number of findings in the report.
        issue_count: usize,
    },

    /// Validation completed but found one or more invalid resource inputs.
    Invalid {
        /// Number of findings in the report.
        issue_count: usize,
    },
}

impl fmt::Display for ResourceValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Incomplete { issue_count } => write!(
                formatter,
                "resource validation is incomplete: {issue_count} finding(s)"
            ),
            Self::Invalid { issue_count } => write!(
                formatter,
                "resource validation failed: {issue_count} finding(s)"
            ),
        }
    }
}

impl Error for ResourceValidationError {}

/// Require a complete, successful resource-validation report.
///
/// This helper deliberately distinguishes incomplete validation from a
/// complete validation that found invalid resource inputs.
pub fn ensure_valid(
    report: &ResourceValidationReport,
) -> ResourceValidationResult<()> {
    if !report.is_complete() {
        return Err(ResourceValidationError::Incomplete {
            issue_count: report.issue_count(),
        });
    }

    if !report.issues().is_empty() {
        return Err(ResourceValidationError::Invalid {
            issue_count: report.issue_count(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A test validator that accepts every canonical resource value.
    struct AcceptAll;

    impl ResourceSemanticsValidator for AcceptAll {
        fn validate_requirement(
            &self,
            _requirement: &ResourceRequirement,
        ) -> Result<(), String> {
            Ok(())
        }

        fn validate_constraint(
            &self,
            _constraint: &ResourceConstraint,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn empty_complete_input_produces_a_valid_report() {
        let requirements = ExecutionRequirements::new();

        let report = validate_resource_requirements(
            &requirements,
            ResourceValidationPolicy::unbounded(),
            &AcceptAll,
        );

        assert!(report.is_complete());
        assert!(report.is_valid());
        assert_eq!(report.issue_count(), 0);
        assert_eq!(report.requirements_examined(), 0);
        assert_eq!(report.constraints_examined(), 0);
        assert!(ensure_valid(&report).is_ok());
    }

    #[test]
    fn upstream_incompleteness_is_never_silently_accepted() {
        let mut requirements = ExecutionRequirements::new();
        requirements.record_issue(
            crate::classical::execution::requirements::RequirementIssue::new(
                crate::classical::execution::requirements::RequirementIssueKind::AnalysisFailure,
                None,
                "Test preparation failure.",
            ),
        );

        let report = validate_resource_requirements(
            &requirements,
            ResourceValidationPolicy::unbounded(),
            &AcceptAll,
        );

        assert!(!report.is_complete());
        assert!(!report.is_valid());
        assert!(report.has_issue(
            ResourceValidationIssueKind::IncompleteRequirements
        ));
        assert!(matches!(
            ensure_valid(&report),
            Err(ResourceValidationError::Incomplete { .. })
        ));
    }

    #[test]
    fn zero_budgets_are_explicit_operational_limits() {
        let requirements = ExecutionRequirements::new();

        let report = validate_resource_requirements(
            &requirements,
            ResourceValidationPolicy::new(Some(0), Some(0)),
            &AcceptAll,
        );

        assert!(report.is_complete());
        assert!(report.is_valid());
        assert_eq!(report.requirements_examined(), 0);
        assert_eq!(report.constraints_examined(), 0);
    }

    #[test]
    fn report_completeness_and_validity_are_distinct() {
        let mut report = ResourceValidationReport::new();

        report.push_issue(ResourceValidationIssue::new(
            ResourceValidationIssueKind::InvalidRequirement,
            ResourceValidationSubject::Requirement { index: 0 },
            "Test semantic failure.",
        ));

        assert!(report.is_complete());
        assert!(!report.is_valid());
        assert!(matches!(
            ensure_valid(&report),
            Err(ResourceValidationError::Invalid { issue_count: 1 })
        ));
    }

    #[test]
    fn issue_codes_are_stable() {
        assert_eq!(
            ResourceValidationIssueKind::InvalidRequirement.as_str(),
            "classical.resources.invalid_requirement"
        );
        assert_eq!(
            ResourceValidationIssueKind::ConstraintBudgetExceeded.as_str(),
            "classical.resources.constraint_budget_exceeded"
        );
    }

    #[test]
    fn an_exhausted_budget_marks_the_report_incomplete() {
        let mut report = ResourceValidationReport::new();
        report.mark_incomplete();
        report.push_issue(ResourceValidationIssue::new(
            ResourceValidationIssueKind::RequirementBudgetExceeded,
            ResourceValidationSubject::RequirementCollection,
            "Test budget exhaustion.",
        ));

        assert!(!report.is_complete());
        assert!(!report.is_valid());
    }
}
