
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Execution Results
//!
//! Path: `src/zur/execution/result.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Defines the normalized result of a ZUR-managed execution.
//!
//! An execution result describes what an execution authority has
//! established about an execution. It does not describe what was
//! merely requested, planned, submitted, reserved or predicted.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Normalized execution-result classification.
//! - References to externally owned output records.
//! - Partial-completion reporting.
//! - Execution failure association.
//! - Explicit cancellation-result reporting.
//! - Timeout-result reporting.
//! - Incomplete-result reporting.
//! - Recovery-status reporting and evidence references.
//! - Result consistency checks against canonical execution observations.
//!
//! This module does NOT own:
//!
//! - Execution identity.
//! - Execution lifecycle state.
//! - Execution submission or observation.
//! - Provider identity or provider execution outcomes.
//! - Provider-specific payloads or sessions.
//! - Artifact construction or storage.
//! - Compiler or execution plans.
//! - Resource allocation or reservation.
//! - Recovery implementation.
//! - Checkpoint implementation.
//! - Domain-specific result schemas.
//! - Source-language semantics.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Important distinctions
//!
//! - Submission != completion.
//! - Execution status != result payload.
//! - Full completion != partial completion.
//! - Cancellation request != confirmed cancellation.
//! - Timeout != successful completion.
//! - Incomplete result != successful result.
//! - Recovery recommendation != completed recovery.
//! - Recovery evidence reference != proof independently authenticated
//!   by this module.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Output references.
//! - Result records.
//! - Execution results.
//! - Recovery evidence records.
//! - Computational domains.
//! - Providers.
//! - Devices.
//! - Resource quantities.
//!
//! Collections grow subject to available memory and allocator capacity.
//!
//! Literal infinite physical computation or storage is not promised.
//!
//! ## Safety
//!
//! Safe Rust only.
//! No raw pointers.
//! No unsafe implementations.
//! No global mutable state.
//! No fabricated success.
//! No implicit recovery.
//! No implicit resource allocation.
//! No arbitrary hardware or domain ceilings.

use core::fmt;

use crate::zur::contracts::execution::{
    CancellationDisposition,
    ExecutionObservation,
    ExecutionReference,
    ExecutionStatus,
};

use crate::zur::error::{
    RecoveryDisposition,
    ZurError,
    ZurResult,
};

use crate::zur::ids::ExecutionId;

// ============================================================================
// RESULT STATUS
// ============================================================================

/// Normalized outcome classification for an execution.
///
/// This classification describes the result of execution, not the
/// execution lifecycle itself.
///
/// The canonical lifecycle remains owned by
/// `zur::contracts::execution::ExecutionStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ExecutionResultStatus {
    /// Execution completed successfully.
    ///
    /// This is the only status representing full successful completion.
    Completed,

    /// Execution produced a valid but incomplete subset of its intended
    /// outputs.
    ///
    /// Partial completion must never be represented as full success.
    PartiallyCompleted,

    /// Execution failed.
    Failed,

    /// Execution was explicitly confirmed as cancelled.
    Cancelled,

    /// Execution exceeded its applicable execution deadline or timeout
    /// policy.
    TimedOut,

    /// The available observation does not establish a complete outcome.
    ///
    /// This status is not successful completion.
    Incomplete,
}

impl ExecutionResultStatus {
    /// Returns whether this result represents full successful completion.
    #[must_use]
    pub const fn is_successful(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns whether this result represents partial completion.
    #[must_use]
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::PartiallyCompleted)
    }

    /// Returns whether this result represents a failure.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns whether this result represents confirmed cancellation.
    #[must_use]
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }

    /// Returns whether this result represents a timeout.
    #[must_use]
    pub const fn is_timed_out(self) -> bool {
        matches!(self, Self::TimedOut)
    }

    /// Returns whether the outcome is incomplete or indeterminate.
    #[must_use]
    pub const fn is_incomplete(self) -> bool {
        matches!(self, Self::Incomplete)
    }

    /// Returns whether this result represents any kind of successful
    /// completion.
    ///
    /// Partial completion is deliberately excluded.
    #[must_use]
    pub const fn is_fully_successful(self) -> bool {
        self.is_successful()
    }
}

impl fmt::Display for ExecutionResultStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Completed => "completed",
            Self::PartiallyCompleted => "partially_completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Incomplete => "incomplete",
        })
    }
}

// ============================================================================
// RECOVERY STATUS
// ============================================================================

/// Observed recovery state associated with an execution result.
///
/// This enum describes recovery reporting only.
///
/// It does not perform retries, restore checkpoints, restart providers,
/// migrate execution, or establish that a recovery operation is feasible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RecoveryStatus {
    /// Recovery is not applicable to this result.
    NotApplicable,

    /// No recovery operation has been reported.
    NotAttempted,

    /// Recovery has been reported as in progress.
    InProgress,

    /// Recovery has been reported as completed.
    Recovered,

    /// A recovery attempt has been reported as failed.
    Failed,

    /// Recovery is unavailable.
    Unavailable,

    /// External intervention is required.
    RequiresIntervention,
}

impl RecoveryStatus {
    /// Returns whether recovery has been reported as completed.
    #[must_use]
    pub const fn is_recovered(self) -> bool {
        matches!(self, Self::Recovered)
    }

    /// Returns whether recovery has been reported as failed.
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }
}

// ============================================================================
// RECOVERY REPORT
// ============================================================================

/// A recovery observation associated with an execution result.
///
/// Recovery evidence is referenced rather than duplicated. The
/// reliability or domain-specific recovery authority remains responsible
/// for producing, validating and interpreting the referenced record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryReport {
    status: RecoveryStatus,
    evidence: Option<ExecutionReference>,
    disposition: RecoveryDisposition,
}

impl RecoveryReport {
    /// Creates a recovery report.
    ///
    /// A report claiming successful or failed recovery must identify
    /// an external evidence record.
    ///
    /// This structural requirement does not authenticate the evidence.
    pub fn new(
        status: RecoveryStatus,
        evidence: Option<ExecutionReference>,
        disposition: RecoveryDisposition,
    ) -> ZurResult<Self> {
        if matches!(
            status,
            RecoveryStatus::Recovered | RecoveryStatus::Failed
        ) && evidence.is_none()
        {
            return Err(
                ZurError::invalid_input(
                    "completed or failed recovery requires an evidence reference",
                )
                .with_component("zur.execution.result")
                .with_operation("recovery_report.create"),
            );
        }

        Ok(Self {
            status,
            evidence,
            disposition,
        })
    }

    /// Creates a report indicating that recovery has not been attempted.
    #[must_use]
    pub const fn not_attempted() -> Self {
        Self {
            status: RecoveryStatus::NotAttempted,
            evidence: None,
            disposition: RecoveryDisposition::Unspecified,
        }
    }

    /// Creates a report indicating that recovery is not applicable.
    #[must_use]
    pub const fn not_applicable() -> Self {
        Self {
            status: RecoveryStatus::NotApplicable,
            evidence: None,
            disposition: RecoveryDisposition::NotRequired,
        }
    }

    /// Returns the reported recovery status.
    #[must_use]
    pub const fn status(&self) -> RecoveryStatus {
        self.status
    }

    /// Returns the optional external recovery evidence reference.
    #[must_use]
    pub fn evidence(&self) -> Option<&ExecutionReference> {
        self.evidence.as_ref()
    }

    /// Returns the recovery disposition.
    ///
    /// The disposition is guidance from the owning recovery authority;
    /// it does not cause recovery to occur.
    #[must_use]
    pub const fn disposition(&self) -> RecoveryDisposition {
        self.disposition
    }
}

// ============================================================================
// EXECUTION RESULT
// ============================================================================

/// Normalized result of one ZUR execution.
///
/// The result references externally owned output records. It does not
/// duplicate domain-specific result payloads or serialization formats.
///
/// A result may contain multiple output references. This supports
/// executions composed of multiple operations, providers, domains or
/// distributed participants without introducing a fixed result count.
///
/// The result owns an optional `ZurError` for locally reported failures.
/// It is intentionally not `Clone`, because `ZurError` preserves causal
/// error ownership and is not cloneable.
///
/// The canonical execution identity is reused from `zur::ids`.
#[derive(Debug)]
pub struct ExecutionResult {
    execution_id: ExecutionId,
    status: ExecutionResultStatus,
    outputs: Vec<ExecutionReference>,
    failure: Option<ZurError>,
    cancellation: Option<CancellationDisposition>,
    recovery: RecoveryReport,
}

impl ExecutionResult {
    /// Creates a fully completed execution result.
    ///
    /// An empty output collection is valid for executions that produce
    /// no externally referenced output records.
    ///
    /// The caller must only use this constructor when full completion
    /// has actually been established by the execution authority.
    pub fn completed(
        execution_id: ExecutionId,
        outputs: Vec<ExecutionReference>,
        recovery: RecoveryReport,
    ) -> Self {
        Self {
            execution_id,
            status: ExecutionResultStatus::Completed,
            outputs,
            failure: None,
            cancellation: None,
            recovery,
        }
    }

    /// Creates a partially completed execution result.
    ///
    /// The reason is mandatory so partial completion cannot be silently
    /// presented as an ordinary successful result.
    ///
    /// Any supplied outputs represent records actually produced and
    /// retained by their owning authorities.
    pub fn partially_completed(
        execution_id: ExecutionId,
        outputs: Vec<ExecutionReference>,
        reason: ZurError,
        recovery: RecoveryReport,
    ) -> Self {
        Self {
            execution_id,
            status: ExecutionResultStatus::PartiallyCompleted,
            outputs,
            failure: Some(reason),
            cancellation: None,
            recovery,
        }
    }

    /// Creates a failed execution result.
    ///
    /// A failed execution may still have output references when earlier
    /// operations produced durable results before the failure.
    ///
    /// The presence of outputs does not convert failure into success.
    pub fn failed(
        execution_id: ExecutionId,
        outputs: Vec<ExecutionReference>,
        error: ZurError,
        recovery: RecoveryReport,
    ) -> Self {
        Self {
            execution_id,
            status: ExecutionResultStatus::Failed,
            outputs,
            failure: Some(error),
            cancellation: None,
            recovery,
        }
    }

    /// Creates a cancelled execution result.
    ///
    /// Only `CancellationDisposition::Confirmed` is accepted.
    ///
    /// A requested, rejected, unknown or already-terminal cancellation
    /// disposition is not sufficient evidence that this execution was
    /// cancelled.
    pub fn cancelled(
        execution_id: ExecutionId,
        outputs: Vec<ExecutionReference>,
        disposition: CancellationDisposition,
        recovery: RecoveryReport,
    ) -> ZurResult<Self> {
        if !disposition.is_confirmed() {
            return Err(
                ZurError::invalid_input(
                    "a cancelled result requires explicitly confirmed cancellation",
                )
                .with_component("zur.execution.result")
                .with_operation("result.cancelled"),
            );
        }

        Ok(Self {
            execution_id,
            status: ExecutionResultStatus::Cancelled,
            outputs,
            failure: None,
            cancellation: Some(disposition),
            recovery,
        })
    }

    /// Creates a timed-out execution result.
    ///
    /// A timeout is not successful completion, even if the execution
    /// produced some outputs before the timeout.
    pub fn timed_out(
        execution_id: ExecutionId,
        outputs: Vec<ExecutionReference>,
        error: ZurError,
        recovery: RecoveryReport,
    ) -> Self {
        Self {
            execution_id,
            status: ExecutionResultStatus::TimedOut,
            outputs,
            failure: Some(error),
            cancellation: None,
            recovery,
        }
    }

    /// Creates an incomplete or indeterminate execution result.
    ///
    /// This constructor is appropriate when the available execution
    /// evidence cannot establish a final successful or failed outcome.
    ///
    /// It must not be used to conceal a known successful or failed result.
    pub fn incomplete(
        execution_id: ExecutionId,
        outputs: Vec<ExecutionReference>,
        reason: ZurError,
        recovery: RecoveryReport,
    ) -> Self {
        Self {
            execution_id,
            status: ExecutionResultStatus::Incomplete,
            outputs,
            failure: Some(reason),
            cancellation: None,
            recovery,
        }
    }

    /// Returns the canonical execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the normalized result classification.
    #[must_use]
    pub const fn status(&self) -> ExecutionResultStatus {
        self.status
    }

    /// Returns whether the execution fully completed successfully.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        self.status.is_successful()
    }

    /// Returns whether the execution partially completed.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        self.status.is_partial()
    }

    /// Returns whether the execution failed.
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        self.status.is_failure()
    }

    /// Returns whether the execution was confirmed as cancelled.
    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        self.status.is_cancelled()
    }

    /// Returns whether the execution timed out.
    #[must_use]
    pub const fn is_timed_out(&self) -> bool {
        self.status.is_timed_out()
    }

    /// Returns whether the execution outcome is incomplete.
    #[must_use]
    pub const fn is_incomplete(&self) -> bool {
        self.status.is_incomplete()
    }

    /// Returns all externally owned output references.
    #[must_use]
    pub fn outputs(&self) -> &[ExecutionReference] {
        &self.outputs
    }

    /// Returns the locally associated failure, if any.
    #[must_use]
    pub fn failure(&self) -> Option<&ZurError> {
        self.failure.as_ref()
    }

    /// Returns the cancellation disposition, if any.
    #[must_use]
    pub const fn cancellation_disposition(
        &self,
    ) -> Option<CancellationDisposition> {
        self.cancellation
    }

    /// Returns the recovery report.
    #[must_use]
    pub fn recovery(&self) -> &RecoveryReport {
        &self.recovery
    }

    /// Appends an externally owned output reference.
    ///
    /// Allocation failure is returned as a structured ZUR error.
    ///
    /// No fixed output-count limit is imposed.
    pub fn add_output(
        &mut self,
        output: ExecutionReference,
    ) -> ZurResult<()> {
        self.outputs.try_reserve(1).map_err(|error| {
            ZurError::resource_exhausted(
                format!(
                    "unable to reserve storage for an execution output reference: {error}"
                ),
            )
            .with_component("zur.execution.result")
            .with_operation("result.add_output")
            .with_execution(self.execution_id.to_string())
        })?;

        self.outputs.push(output);

        Ok(())
    }

    /// Validates this result against a canonical execution observation.
    ///
    /// This checks:
    ///
    /// - Execution identity equality.
    /// - Consistency between result classification and lifecycle state.
    /// - Presence of any result reference exposed by the observation.
    ///
    /// It does not authenticate the observation, verify external records,
    /// or independently prove that the provider's claims are true.
    pub fn validate_against(
        &self,
        observation: &ExecutionObservation,
    ) -> ZurResult<()> {
        if self.execution_id != *observation.execution_id() {
            return Err(
                ZurError::invalid_input(
                    "execution result identity does not match the execution observation",
                )
                .with_component("zur.execution.result")
                .with_operation("result.validate_observation")
                .with_execution(self.execution_id.to_string()),
            );
        }

        let observed_status = observation.state().status();

        let consistent = match self.status {
            ExecutionResultStatus::Completed => {
                observed_status == ExecutionStatus::Completed
            }

            ExecutionResultStatus::PartiallyCompleted => {
                observed_status == ExecutionStatus::PartiallyCompleted
            }

            ExecutionResultStatus::Failed => {
                observed_status == ExecutionStatus::Failed
            }

            ExecutionResultStatus::Cancelled => {
                observed_status == ExecutionStatus::Cancelled
                    && self.cancellation
                        == Some(CancellationDisposition::Confirmed)
            }

            ExecutionResultStatus::TimedOut => {
                observed_status == ExecutionStatus::TimedOut
            }

            ExecutionResultStatus::Incomplete => {
                observed_status != ExecutionStatus::Completed
                    && observed_status
                        != ExecutionStatus::PartiallyCompleted
            }
        };

        if !consistent {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "execution result status `{}` is inconsistent with observed lifecycle status `{:?}`",
                        self.status,
                        observed_status,
                    ),
                )
                .with_component("zur.execution.result")
                .with_operation("result.validate_observation")
                .with_execution(self.execution_id.to_string()),
            );
        }

        if let Some(observed_reference) = observation.result() {
            if !self.outputs.contains(observed_reference) {
                return Err(
                    ZurError::invalid_input(
                        "execution result does not contain the result reference reported by the execution observation",
                    )
                    .with_component("zur.execution.result")
                    .with_operation("result.validate_observation")
                    .with_execution(self.execution_id.to_string()),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// CONVERSION FROM CANONICAL RECOVERY DISPOSITION
// ============================================================================

/// Converts an existing canonical recovery disposition into a reporting
/// status where the mapping is unambiguous.
///
/// This conversion does not claim that recovery was attempted or completed.
impl From<RecoveryDisposition> for RecoveryStatus {
    fn from(disposition: RecoveryDisposition) -> Self {
        match disposition {
            RecoveryDisposition::NotRequired => Self::NotApplicable,

            RecoveryDisposition::RecoveryUnavailable => Self::Unavailable,

            RecoveryDisposition::ExternalInterventionRequired => {
                Self::RequiresIntervention
            }

            RecoveryDisposition::Unspecified
            | RecoveryDisposition::RetryMayBeAppropriate
            | RecoveryDisposition::RequiresConditionChange
            | RecoveryDisposition::RecoveryRequired
            | RecoveryDisposition::CheckpointRequired => Self::NotAttempted,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::ids::{
        ExecutionId,
        IdentityNamespace,
    };

    fn execution_id() -> ExecutionId {
        ExecutionId::new(
            IdentityNamespace::new(
                "zamani.runtime.execution",
            )
            .expect("valid identity namespace"),
            vec![1],
        )
        .expect("valid execution identity")
    }

    fn reference(record: &str) -> ExecutionReference {
        ExecutionReference::new(
            "zamani.results",
            record,
        )
        .expect("valid execution reference")
    }

    #[test]
    fn only_full_completion_is_success() {
        assert!(
            ExecutionResultStatus::Completed.is_successful()
        );

        assert!(
            !ExecutionResultStatus::PartiallyCompleted.is_successful()
        );

        assert!(
            !ExecutionResultStatus::Failed.is_successful()
        );

        assert!(
            !ExecutionResultStatus::Cancelled.is_successful()
        );

        assert!(
            !ExecutionResultStatus::TimedOut.is_successful()
        );

        assert!(
            !ExecutionResultStatus::Incomplete.is_successful()
        );
    }

    #[test]
    fn completed_result_preserves_identity_and_outputs() {
        let id = execution_id();

        let result = ExecutionResult::completed(
            id.clone(),
            vec![reference("output-1")],
            RecoveryReport::not_applicable(),
        );

        assert_eq!(result.execution_id(), &id);

        assert_eq!(
            result.status(),
            ExecutionResultStatus::Completed,
        );

        assert!(result.is_successful());

        assert_eq!(result.outputs().len(), 1);
    }

    #[test]
    fn completed_result_can_have_no_external_outputs() {
        let result = ExecutionResult::completed(
            execution_id(),
            Vec::new(),
            RecoveryReport::not_applicable(),
        );

        assert!(result.is_successful());

        assert!(result.outputs().is_empty());
    }

    #[test]
    fn partial_result_is_not_full_success() {
        let result = ExecutionResult::partially_completed(
            execution_id(),
            vec![reference("partial-output")],
            ZurError::execution(
                "some execution operations did not complete",
            ),
            RecoveryReport::not_attempted(),
        );

        assert!(result.is_partial());

        assert!(!result.is_successful());

        assert!(result.failure().is_some());

        assert_eq!(result.outputs().len(), 1);
    }

    #[test]
    fn failed_result_can_preserve_outputs_from_completed_operations() {
        let result = ExecutionResult::failed(
            execution_id(),
            vec![reference("committed-output")],
            ZurError::execution(
                "a later operation failed",
            ),
            RecoveryReport::not_attempted(),
        );

        assert!(result.is_failed());

        assert!(!result.is_successful());

        assert_eq!(result.outputs().len(), 1);
    }

    #[test]
    fn cancellation_requires_confirmation() {
        let result = ExecutionResult::cancelled(
            execution_id(),
            Vec::new(),
            CancellationDisposition::Requested,
            RecoveryReport::not_attempted(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn confirmed_cancellation_is_reported_honestly() {
        let result = ExecutionResult::cancelled(
            execution_id(),
            Vec::new(),
            CancellationDisposition::Confirmed,
            RecoveryReport::not_applicable(),
        )
        .expect("confirmed cancellation is valid");

        assert!(result.is_cancelled());

        assert!(!result.is_successful());

        assert_eq!(
            result.cancellation_disposition(),
            Some(CancellationDisposition::Confirmed),
        );
    }

    #[test]
    fn timeout_is_not_success() {
        let result = ExecutionResult::timed_out(
            execution_id(),
            Vec::new(),
            ZurError::timeout("execution deadline elapsed"),
            RecoveryReport::not_attempted(),
        );

        assert!(result.is_timed_out());

        assert!(!result.is_successful());
    }

    #[test]
    fn incomplete_result_is_not_success() {
        let result = ExecutionResult::incomplete(
            execution_id(),
            Vec::new(),
            ZurError::execution(
                "execution outcome could not be established",
            ),
            RecoveryReport::not_attempted(),
        );

        assert!(result.is_incomplete());

        assert!(!result.is_successful());
    }

    #[test]
    fn recovered_status_requires_evidence() {
        let result = RecoveryReport::new(
            RecoveryStatus::Recovered,
            None,
            RecoveryDisposition::NotRequired,
        );

        assert!(result.is_err());
    }

    #[test]
    fn recovered_status_accepts_evidence() {
        let report = RecoveryReport::new(
            RecoveryStatus::Recovered,
            Some(reference("recovery-record")),
            RecoveryDisposition::NotRequired,
        )
        .expect("recovery evidence is present");

        assert!(report.status().is_recovered());

        assert!(report.evidence().is_some());
    }

    #[test]
    fn output_collection_can_grow() {
        let mut result = ExecutionResult::completed(
            execution_id(),
            Vec::new(),
            RecoveryReport::not_applicable(),
        );

        result
            .add_output(reference("output-1"))
            .expect("output can be added");

        result
            .add_output(reference("output-2"))
            .expect("another output can be added");

        assert_eq!(result.outputs().len(), 2);
    }

    #[test]
    fn recovery_disposition_does_not_claim_recovery() {
        let status = RecoveryStatus::from(
            RecoveryDisposition::RecoveryRequired,
        );

        assert_eq!(status, RecoveryStatus::NotAttempted);
    }
}
