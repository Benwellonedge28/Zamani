
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Realization Reports
//!
//! Path: `src/hdl/realization/reports.rs`.
//!
//! Records realization outcomes, per-operation results, issues, and a
//! snapshot of the plan's canonical target and feasibility evidence.
//!
//! This module does not perform realization or implement HDLIR.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::hdl::plan::{
    HdlRealizationOperation,
    HdlRealizationPlan,
};
use crate::hdl::targets::feasibility::{
    FeasibilityStatus,
    HdlTargetFeasibilityReport,
};
use crate::hdl::targets::model::TargetId;

/// Stable architectural name of this module.
pub const HDL_REALIZATION_REPORTS_MODULE_NAME: &str =
    "zamani.hdl.realization.reports";

/// Version of this module's in-memory report contract.
///
/// This is not the language, target, provider, or HDLIR version.
pub const HDL_REALIZATION_REPORT_SCHEMA_VERSION: u16 = 1;

/// Overall reported outcome of a realization attempt.
///
/// This is independent of target feasibility. In particular, a provider
/// result does not authenticate a target or upgrade feasibility evidence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HdlRealizationOutcome {
    /// No realization attempt has started.
    NotStarted,

    /// The provider reports that the requested realization completed.
    ///
    /// This records the provider's reported outcome. It is not independent
    /// verification of the generated implementation.
    Completed,

    /// The attempt completed with one or more unresolved or incomplete parts.
    Partial,

    /// The attempt failed.
    Failed,

    /// The attempt was cancelled before full completion.
    Cancelled,
}

impl HdlRealizationOutcome {
    /// Return whether the reported attempt completed without a reported
    /// partial result.
    ///
    /// This does not establish feasibility, correctness, or independent
    /// verification of the resulting implementation.
    #[must_use]
    pub const fn is_completed(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Return whether the outcome explicitly reports failure.
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Return whether the result is incomplete.
    #[must_use]
    pub const fn is_incomplete(self) -> bool {
        matches!(
            self,
            Self::NotStarted | Self::Partial | Self::Cancelled
        )
    }
}

/// Reported outcome of one requested provider operation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HdlOperationOutcome {
    /// The operation has not yet produced a result.
    Pending,

    /// The provider reports successful completion of the operation.
    Completed,

    /// The provider reports that the operation failed.
    Failed,

    /// The operation was cancelled.
    Cancelled,

    /// The operation produced an incomplete result.
    Partial,

    /// The provider does not support the requested operation.
    Unsupported,
}

impl HdlOperationOutcome {
    /// Return whether the operation remains unresolved.
    #[must_use]
    pub const fn is_pending(self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Return whether the operation reports successful completion.
    #[must_use]
    pub const fn is_completed(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Return whether the operation is in a terminal unsuccessful state.
    #[must_use]
    pub const fn is_unsuccessful(self) -> bool {
        matches!(
            self,
            Self::Failed | Self::Cancelled | Self::Unsupported
        )
    }
}

/// Severity of a realization-report issue.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlReportSeverity {
    /// Informational evidence that does not itself indicate a problem.
    Info,

    /// A condition worth reviewing.
    Warning,

    /// A failure or unmet condition.
    Error,
}

/// Category of a realization-report issue.
///
/// The category is open-ended through [`HdlReportIssue::code`]; these
/// variants classify common cases without defining a closed universe of
/// possible provider diagnostics.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HdlReportIssueKind {
    /// Target feasibility is unresolved or negative.
    Feasibility,

    /// A required operation is unsupported or failed.
    Operation,

    /// A realization requirement was not established.
    Requirement,

    /// Resource information is missing or incomplete.
    Resource,

    /// A required implementation decision remains unresolved.
    UnresolvedDecision,

    /// A provider or toolchain reported a limitation.
    Provider,

    /// An operational budget or cancellation prevented completion.
    OperationalLimit,

    /// A result could not be independently verified.
    Verification,

    /// Another issue defined by a provider or future extension.
    Other,
}

/// A structured issue attached to a realization report.
///
/// `code` is a stable, open-ended machine-readable identifier. The message
/// is descriptive and must not be used as a programmatic discriminator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlReportIssue {
    code: String,
    kind: HdlReportIssueKind,
    severity: HdlReportSeverity,
    message: String,
    operation_id: Option<String>,
}

impl HdlReportIssue {
    /// Construct a report issue.
    ///
    /// Identifiers and messages must be nonempty and contain no control
    /// characters. Namespaced and future-defined issue codes are allowed.
    ///
    /// # Errors
    ///
    /// Returns an error when an identifier or message is invalid.
    pub fn new(
        code: impl Into<String>,
        kind: HdlReportIssueKind,
        severity: HdlReportSeverity,
        message: impl Into<String>,
        operation_id: Option<String>,
    ) -> Result<Self, HdlReportError> {
        let code = code.into();
        let message = message.into();

        validate_text(&code, "issue code")?;
        validate_text(&message, "issue message")?;

        if let Some(id) = operation_id.as_deref() {
            validate_text(id, "operation identifier")?;
        }

        Ok(Self {
            code,
            kind,
            severity,
            message,
            operation_id,
        })
    }

    /// Return the stable issue code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the issue category.
    #[must_use]
    pub const fn kind(&self) -> HdlReportIssueKind {
        self.kind
    }

    /// Return the severity.
    #[must_use]
    pub const fn severity(&self) -> HdlReportSeverity {
        self.severity
    }

    /// Return the descriptive message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the associated operation identifier, if any.
    #[must_use]
    pub fn operation_id(&self) -> Option<&str> {
        self.operation_id.as_deref()
    }
}

/// Result for a requested realization operation.
///
/// A pending result is retained until the coordinator records a result.
/// A description is optional because providers may report a machine-readable
/// outcome without producing explanatory text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlOperationResult {
    operation_id: String,
    outcome: HdlOperationOutcome,
    description: Option<String>,
}

impl HdlOperationResult {
    /// Construct an operation result.
    ///
    /// The identifier must match a requested operation when inserted into a
    /// report. This constructor validates the record, not plan membership.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid identifiers or descriptions.
    pub fn new(
        operation_id: impl Into<String>,
        outcome: HdlOperationOutcome,
        description: Option<String>,
    ) -> Result<Self, HdlReportError> {
        let operation_id = operation_id.into();
        validate_text(&operation_id, "operation identifier")?;

        if let Some(text) = description.as_deref() {
            validate_text(text, "operation description")?;
        }

        Ok(Self {
            operation_id,
            outcome,
            description,
        })
    }

    /// Return the requested operation identifier.
    #[must_use]
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    /// Return the reported operation outcome.
    #[must_use]
    pub const fn outcome(&self) -> HdlOperationOutcome {
        self.outcome
    }

    /// Return the optional explanation.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// A report for one HDL realization attempt.
///
/// The report owns a snapshot of the plan's target ID, feasibility assessment,
/// and requested operations. It also retains the canonical resource profile
/// exactly as supplied by the plan.
///
/// The resource profile represents planning inputs, not measured resource
/// usage. This module deliberately does not invent a measurement or allocation
/// model that the existing resource subsystem does not provide.
///
/// Operation results and issues are stored in ordered maps. Identical
/// repeated insertions are idempotent; conflicting records are rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlRealizationReport {
    target_id: TargetId,
    outcome: HdlRealizationOutcome,
    feasibility: Option<HdlTargetFeasibilityReport>,
    resource_profile: crate::hdl::resources::HdlResourceProfile,
    operations: BTreeMap<String, HdlOperationResult>,
    issues: BTreeMap<String, HdlReportIssue>,
}

impl HdlRealizationReport {
    /// Create a report from an existing realization plan.
    ///
    /// All operations requested by the plan start in the `Pending` state.
    /// No operation is assumed to have run merely because it appears in the
    /// plan. The initial outcome is `NotStarted`.
    ///
    /// # Errors
    ///
    /// Returns an error if a requested operation cannot be represented by
    /// the report's operation-result contract.
    pub fn from_plan(
        plan: &HdlRealizationPlan,
    ) -> Result<Self, HdlReportError> {
        let mut operations = BTreeMap::new();

        for operation in plan.operations() {
            let result = HdlOperationResult::new(
                operation.id(),
                HdlOperationOutcome::Pending,
                None,
            )?;

            operations.insert(
                result.operation_id().to_owned(),
                result,
            );
        }

        Ok(Self {
            target_id: plan.target_id().clone(),
            outcome: HdlRealizationOutcome::NotStarted,
            feasibility: plan.feasibility().cloned(),
            resource_profile: plan.resources().clone(),
            operations,
            issues: BTreeMap::new(),
        })
    }

    /// Return the target-description identity for this report.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Return the overall reported realization outcome.
    #[must_use]
    pub const fn outcome(&self) -> HdlRealizationOutcome {
        self.outcome
    }

    /// Set the overall reported outcome.
    ///
    /// This records a coordinator/provider result. It does not execute an
    /// operation, recompute feasibility, or independently verify an artifact.
    ///
    /// Consumers should use [`Self::validate_consistency`] before treating
    /// the report as a complete record.
    pub fn set_outcome(&mut self, outcome: HdlRealizationOutcome) {
        self.outcome = outcome;
    }

    /// Return the feasibility assessment snapshot, if one was attached to
    /// the original plan.
    ///
    /// A missing assessment remains missing. This report does not infer
    /// feasibility from provider success or from an empty requirement set.
    #[must_use]
    pub fn feasibility(&self) -> Option<&HdlTargetFeasibilityReport> {
        self.feasibility.as_ref()
    }

    /// Return the feasibility conclusion, if an assessment exists.
    #[must_use]
    pub fn feasibility_status(&self) -> Option<FeasibilityStatus> {
        self.feasibility.as_ref().map(|report| report.status())
    }

    /// Return the resource profile copied from the plan.
    ///
    /// This profile contains canonical requirements and estimates. It is
    /// not a physical measurement, allocation receipt, or achieved-resource
    /// report.
    #[must_use]
    pub fn resource_profile(
        &self,
    ) -> &crate::hdl::resources::HdlResourceProfile {
        &self.resource_profile
    }

    /// Return operation results in deterministic identifier order.
    pub fn operations(
        &self,
    ) -> impl ExactSizeIterator<Item = &HdlOperationResult> {
        self.operations.values()
    }

    /// Return the number of requested operations recorded in this report.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Look up a result by its operation identifier.
    #[must_use]
    pub fn operation(&self, id: &str) -> Option<&HdlOperationResult> {
        self.operations.get(id)
    }

    /// Record a result for a requested operation.
    ///
    /// An identical repeated result is idempotent. Replacing a different
    /// result requires an explicit future revision policy and is rejected
    /// here to prevent silent history loss.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation was not requested, or if a different
    /// result already exists for the identifier.
    pub fn record_operation(
        &mut self,
        result: HdlOperationResult,
    ) -> Result<(), HdlReportError> {
        let id = result.operation_id().to_owned();

        match self.operations.get(&id) {
            None => Err(HdlReportError::UnknownOperation { id }),
            Some(existing) if existing == &result => Ok(()),
            Some(existing)
                if existing.outcome() == HdlOperationOutcome::Pending =>
            {
                self.operations.insert(id, result);
                Ok(())
            }
            Some(_) => Err(HdlReportError::ConflictingRecord {
                kind: "operation result",
                id,
            }),
        }
    }

    /// Return issues in deterministic identifier order.
    pub fn issues(
        &self,
    ) -> impl ExactSizeIterator<Item = &HdlReportIssue> {
        self.issues.values()
    }

    /// Return the number of issues.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Return whether the report contains an error-severity issue.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues
            .values()
            .any(|issue| issue.severity() == HdlReportSeverity::Error)
    }

    /// Add an issue to the report.
    ///
    /// Issue codes are the record keys. Re-inserting identical data is
    /// idempotent; reusing a code for different data is rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if a conflicting issue already uses the code.
    pub fn add_issue(
        &mut self,
        issue: HdlReportIssue,
    ) -> Result<(), HdlReportError> {
        let id = issue.code().to_owned();

        match self.issues.get(&id) {
            Some(existing) if existing == &issue => Ok(()),
            Some(_) => Err(HdlReportError::ConflictingRecord {
                kind: "issue",
                id,
            }),
            None => {
                self.issues.insert(id, issue);
                Ok(())
            }
        }
    }

    /// Return the number of operations that remain pending.
    #[must_use]
    pub fn pending_operation_count(&self) -> usize {
        self.operations
            .values()
            .filter(|result| result.outcome().is_pending())
            .count()
    }

    /// Return whether every requested operation has a non-pending result.
    ///
    /// An empty operation set satisfies this specific predicate. It does not
    /// imply that the overall design is feasible or has been realized.
    #[must_use]
    pub fn all_operations_resolved(&self) -> bool {
        self.pending_operation_count() == 0
    }

    /// Validate internal consistency without claiming semantic correctness.
    ///
    /// This checks report bookkeeping. It cannot independently verify
    /// provider claims, generated hardware, feasibility, or physical results.
    ///
    /// # Errors
    ///
    /// Returns an error when a completed report still contains pending
    /// operations, or when a failed report contains no recorded failure
    /// evidence.
    pub fn validate_consistency(&self) -> Result<(), HdlReportError> {
        if self.outcome == HdlRealizationOutcome::Completed
            && !self.all_operations_resolved()
        {
            return Err(HdlReportError::InvariantViolation {
                invariant: "completed_report_has_pending_operations",
            });
        }

        if self.outcome == HdlRealizationOutcome::Failed
            && !self.has_errors()
            && !self.operations.values().any(|result| {
                result.outcome() == HdlOperationOutcome::Failed
                    || result.outcome() == HdlOperationOutcome::Unsupported
            })
        {
            return Err(HdlReportError::InvariantViolation {
                invariant: "failed_report_lacks_failure_evidence",
            });
        }

        Ok(())
    }
}

/// Errors produced by realization-report construction and mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlReportError {
    /// A required text field was empty or contained control characters.
    InvalidText {
        /// Field being validated.
        field: &'static str,
    },

    /// A result refers to an operation absent from the original plan.
    UnknownOperation {
        /// Unknown operation identifier.
        id: String,
    },

    /// An identifier was reused for different record contents.
    ConflictingRecord {
        /// Type of record that conflicts.
        kind: &'static str,
        /// Conflicting record identifier.
        id: String,
    },

    /// A report invariant was not satisfied.
    InvariantViolation {
        /// Stable descriptive identifier of the invariant.
        invariant: &'static str,
    },
}

impl fmt::Display for HdlReportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidText { field } => {
                write!(formatter, "invalid HDL report {field}")
            }
            Self::UnknownOperation { id } => {
                write!(formatter, "unknown realization operation `{id}`")
            }
            Self::ConflictingRecord { kind, id } => {
                write!(
                    formatter,
                    "conflicting HDL report {kind} record `{id}`"
                )
            }
            Self::InvariantViolation { invariant } => {
                write!(
                    formatter,
                    "HDL realization report invariant violated: {invariant}"
                )
            }
        }
    }
}

impl Error for HdlReportError {}

fn validate_text(
    value: &str,
    field: &'static str,
) -> Result<(), HdlReportError> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(HdlReportError::InvalidText { field });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hdl::targets::model::{
        HdlTargetDescription,
        TargetId,
    };

    fn plan() -> HdlRealizationPlan {
        let target_id = TargetId::new("test.target")
            .expect("target ID is valid");

        let target = HdlTargetDescription::new(
            target_id,
            "Test target",
            "test",
        )
        .expect("target description is valid");

        HdlRealizationPlan::new(&target)
            .expect("plan is valid")
    }

    #[test]
    fn new_report_preserves_target_and_starts_not_started() {
        let plan = plan();
        let report =
            HdlRealizationReport::from_plan(&plan).expect("report builds");

        assert_eq!(report.target_id(), plan.target_id());
        assert_eq!(report.outcome(), HdlRealizationOutcome::NotStarted);
        assert_eq!(report.operation_count(), 0);
        assert_eq!(report.issue_count(), 0);
        assert_eq!(report.feasibility_status(), None);
    }

    #[test]
    fn feasibility_is_not_inferred_from_an_empty_plan() {
        let plan = plan();
        let report =
            HdlRealizationReport::from_plan(&plan).expect("report builds");

        assert_eq!(report.feasibility_status(), None);
        assert!(!report.outcome().is_completed());
    }

    #[test]
    fn report_issue_codes_are_open_ended_and_deterministic() {
        let mut report =
            HdlRealizationReport::from_plan(&plan()).expect("report builds");

        let issue = HdlReportIssue::new(
            "provider.future_feature_unavailable",
            HdlReportIssueKind::Provider,
            HdlReportSeverity::Warning,
            "The provider does not implement this optional feature.",
            None,
        )
        .expect("issue is valid");

        report.add_issue(issue.clone()).expect("first insert succeeds");
        report.add_issue(issue).expect("identical insert is idempotent");

        assert_eq!(report.issue_count(), 1);
        assert!(!report.has_errors());
    }

    #[test]
    fn conflicting_issue_codes_are_rejected() {
        let mut report =
            HdlRealizationReport::from_plan(&plan()).expect("report builds");

        let first = HdlReportIssue::new(
            "provider.issue",
            HdlReportIssueKind::Provider,
            HdlReportSeverity::Warning,
            "First message",
            None,
        )
        .expect("issue is valid");

        let second = HdlReportIssue::new(
            "provider.issue",
            HdlReportIssueKind::Provider,
            HdlReportSeverity::Error,
            "Different message",
            None,
        )
        .expect("issue is valid");

        report.add_issue(first).expect("first insert succeeds");

        assert!(matches!(
            report.add_issue(second),
            Err(HdlReportError::ConflictingRecord { .. })
        ));

        assert_eq!(report.issue_count(), 1);
    }

    #[test]
    fn completed_report_cannot_have_pending_operations() {
        let mut plan = plan();

        plan.add_operation(
            HdlRealizationOperation::new(
                "synthesis.prepare",
                Some("Prepare the design".to_owned()),
                true,
            )
            .expect("operation is valid"),
        )
        .expect("operation inserts");

        let mut report =
            HdlRealizationReport::from_plan(&plan).expect("report builds");

        report.set_outcome(HdlRealizationOutcome::Completed);

        assert!(matches!(
            report.validate_consistency(),
            Err(HdlReportError::InvariantViolation {
                invariant: "completed_report_has_pending_operations"
            })
        ));
    }

    #[test]
    fn operation_results_must_belong_to_the_plan() {
        let mut report =
            HdlRealizationReport::from_plan(&plan()).expect("report builds");

        let result = HdlOperationResult::new(
            "unknown.operation",
            HdlOperationOutcome::Completed,
            None,
        )
        .expect("result is locally valid");

        assert!(matches!(
            report.record_operation(result),
            Err(HdlReportError::UnknownOperation { .. })
        ));
    }

    #[test]
    fn failed_report_requires_failure_evidence() {
        let mut report =
            HdlRealizationReport::from_plan(&plan()).expect("report builds");

        report.set_outcome(HdlRealizationOutcome::Failed);

        assert!(matches!(
            report.validate_consistency(),
            Err(HdlReportError::InvariantViolation {
                invariant: "failed_report_lacks_failure_evidence"
            })
        ));
    }

    #[test]
    fn invalid_issue_text_is_rejected() {
        assert!(matches!(
            HdlReportIssue::new(
                "",
                HdlReportIssueKind::Other,
                HdlReportSeverity::Info,
                "message",
                None,
            ),
            Err(HdlReportError::InvalidText { .. })
        ));
    }
}
