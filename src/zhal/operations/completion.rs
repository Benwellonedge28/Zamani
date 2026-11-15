
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Operation Completion
//!
//! Path: `src/zhal/operations/completion.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the provider-independent completion boundary for
//! hardware operations.
//!
//! Supports:
//!
//! - Successful operation completion.
//! - Partial operation completion.
//! - Failed operations.
//! - Confirmed cancellation.
//! - In-progress observations.
//! - Indeterminate completion outcomes.
//! - Generic domain-owned result payloads.
//! - Provider and device provenance.
//! - Submission receipt correlation.
//! - Completion validation.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Completion status.
//! - Completion outcome representation.
//! - Generic operation result representation.
//! - Partial completion evidence.
//! - Completion failure metadata.
//! - Completion provenance.
//! - Completion reference validation.
//! - Completion/submission consistency validation.
//! - Completion lifecycle validation.
//!
//! This module DOES NOT own:
//!
//! - Operation identity definitions.
//! - Provider identity definitions.
//! - Operation contract definitions.
//! - Submission.
//! - Physical hardware execution.
//! - Execution scheduling.
//! - Resource allocation.
//! - Resource reservation.
//! - Hardware discovery.
//! - Device selection.
//! - Automatic retries.
//! - Cancellation implementation.
//! - Distributed transport.
//! - Quantum execution.
//! - Quantum routing.
//! - HDL realization.
//! - Classical execution.
//! - Domain-specific result semantics.
//! - Result serialization.
//! - Runtime execution orchestration.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Operation identity:
//!     crate::zhal::identity
//!
//! Provider identity:
//!     crate::stdlib::provider
//!
//! Submission:
//!     crate::zhal::operations::submission
//!
//! Operation contracts:
//!     crate::zhal::operations::contract
//!
//! Error classification:
//!     crate::zhal::errors
//!
//! Operation context:
//!     crate::zhal::context
//!
//! Runtime orchestration:
//!     crate::zur
//!
//! ## RESULT OWNERSHIP
//!
//! Result payloads are generic.
//!
//! Classical, quantum, HDL, accelerator and distributed
//! computation have different result semantics.
//!
//! ZHAL must not introduce a universal result payload format.
//!
//! The owning domain defines the result type.
//!
//! ## COMPLETION SEMANTICS
//!
//! Completion and submission are different lifecycle stages.
//!
//! A submission receipt establishes what the provider knows
//! about acceptance.
//!
//! A completion record establishes what the provider knows
//! about execution progress or outcome.
//!
//! Neither should be substituted for the other.
//!
//! ## UNKNOWN OUTCOMES
//!
//! Unknown is not equivalent to failure.
//!
//! A provider may have completed physical execution while
//! its completion acknowledgement was lost.
//!
//! Such a condition must remain explicitly unknown until
//! reconciliation establishes the actual outcome.
//!
//! ## PARTIAL COMPLETION
//!
//! Partial completion is represented using provider-owned
//! evidence references.
//!
//! ZHAL does not assume that every computational domain
//! has countable or independently addressable work units.
//!
//! ## CANCELLATION
//!
//! Cancelled means the provider confirms the operation
//! reached a cancelled terminal state.
//!
//! A cancellation request alone does not establish this.
//!
//! An operation whose cancellation outcome is uncertain
//! must remain InProgress or Unknown as appropriate.
//!
//! ## POCO-REAF
//!
//! This module is independent of:
//!
//! - Processor count.
//! - Memory capacity.
//! - Device count.
//! - Qubit count.
//! - Accelerator count.
//! - Distributed participant count.
//! - Hardware vendor.
//! - Physical topology.
//! - Physical location.
//! - Computational domain.
//!
//! No arbitrary universal hardware ceilings are introduced.
//!
//! Collections and payloads are constrained by available
//! resources and explicitly configured operational budgets.
//!
//! Literal infinite physical computation or storage is
//! not promised.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Relevant grammars:
//!
//! - grammar/effects/hardware.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//! - grammar/hardware/hardware.g4
//!
//! These grammars describe source-level computational intent.
//!
//! This module describes hardware-facing completion results.
//!
//! It does not parse grammar syntax or reinterpret semantic
//! requirements, capabilities, preferences, constraints or hints.

use core::fmt;

use crate::stdlib::provider::ProviderId;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalOperationOutcome,
    ZhalResult,
    ZhalRetryability,
};

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareOperationIdentity,
};

use crate::zhal::operations::submission::{
    SubmissionReceipt,
    SubmissionReceiptId,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable completion component identifier.
pub const COMPONENT: &str = "zhal.operations.completion";

/// Completion schema version.
///
/// Independent of the Cargo package version, language version,
/// provider API version and hardware firmware version.
pub const COMPLETION_SCHEMA_VERSION: &str = "1";

// ============================================================================
// COMPLETION REFERENCES
// ============================================================================

/// Opaque provider-owned completion reference.
///
/// A reference may identify:
///
/// - A completion event.
/// - A partial completion report.
/// - A provider-side result.
/// - A reconciliation record.
/// - An incremental progress observation.
///
/// Its interpretation belongs to the provider or domain adapter.
///
/// References must not contain credentials, authentication tokens
/// or other secrets.
///
/// No arbitrary maximum length is imposed.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CompletionReference(String);

impl CompletionReference {
    /// Constructs a validated completion reference.
    ///
    /// Empty, whitespace-only and control-character-containing
    /// references are rejected.
    ///
    /// Exact spelling is preserved.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the reference is invalid.
    pub fn new(value: impl Into<String>) -> ZhalResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(completion_error(
                ZhalErrorKind::InvalidInput,
                "completion reference must not be empty",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(completion_error(
                ZhalErrorKind::InvalidInput,
                "completion reference must not contain control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the exact completion reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the reference and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Debug for CompletionReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CompletionReference")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for CompletionReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// COMPLETION STATUS
// ============================================================================

/// General classification of a hardware operation completion observation.
///
/// This enum is provider-independent.
///
/// It does not encode domain-specific execution states.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CompletionStatus {
    /// Execution is known to be in progress.
    InProgress,

    /// The complete operation successfully completed.
    Succeeded,

    /// Some, but not all, requested work completed.
    PartiallyCompleted,

    /// The operation is known to have failed.
    Failed,

    /// The provider confirms the operation reached a cancelled
    /// terminal state.
    Cancelled,

    /// The final outcome cannot currently be established.
    Unknown,
}

impl CompletionStatus {
    /// Returns the stable descriptive name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProgress => "in_progress",
            Self::Succeeded => "succeeded",
            Self::PartiallyCompleted => "partially_completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the provider has established a terminal outcome.
    ///
    /// Unknown is deliberately not considered terminal because
    /// reconciliation may still establish the actual outcome.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded
                | Self::PartiallyCompleted
                | Self::Failed
                | Self::Cancelled
        )
    }

    /// Returns whether the operation is still executing.
    #[must_use]
    pub const fn is_in_progress(self) -> bool {
        matches!(self, Self::InProgress)
    }

    /// Returns whether the final outcome is uncertain.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl fmt::Display for CompletionStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// COMPLETION PROVENANCE
// ============================================================================

/// Provenance associated with a hardware completion observation.
///
/// Provenance identifies the source of a completion observation.
///
/// It does not establish that the provider or device is trusted.
/// Trust verification remains owned by the security subsystem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletionProvenance {
    provider_id: ProviderId,

    source_reference: CompletionReference,

    device: Option<HardwareDeviceIdentity>,

    submission_receipt_id: Option<SubmissionReceiptId>,

    correlation_reference: Option<CompletionReference>,
}

impl CompletionProvenance {
    /// Constructs completion provenance.
    ///
    /// The provider identity must identify the provider that
    /// supplied the completion observation.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the source reference is invalid.
    pub fn new(
        provider_id: ProviderId,
        source_reference: CompletionReference,
    ) -> Self {
        Self {
            provider_id,
            source_reference,
            device: None,
            submission_receipt_id: None,
            correlation_reference: None,
        }
    }

    /// Associates the physical or provider-backed device.
    ///
    /// The device must belong to the same provider.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput if provider identities differ.
    pub fn with_device(
        mut self,
        device: HardwareDeviceIdentity,
    ) -> ZhalResult<Self> {
        if device.provider_id() != &self.provider_id {
            return Err(completion_error(
                ZhalErrorKind::InvalidInput,
                "completion device belongs to another provider",
            ));
        }

        self.device = Some(device);

        Ok(self)
    }

    /// Associates an existing submission receipt.
    ///
    /// The receipt must belong to the same provider.
    ///
    /// The receipt identifies submission, not completion.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput if provider identities differ.
    #[must_use]
    pub fn with_submission_receipt(
        mut self,
        receipt: &SubmissionReceipt,
    ) -> ZhalResult<Self> {
        if receipt.operation().provider_id() != &self.provider_id {
            return Err(completion_error(
                ZhalErrorKind::InvalidInput,
                "submission receipt belongs to another provider",
            ));
        }

        self.submission_receipt_id =
            receipt.receipt_id().cloned();

        Ok(self)
    }

    /// Associates an external operation correlation reference.
    ///
    /// This may refer to a tracing, distributed execution or
    /// application-level correlation record.
    ///
    /// It does not establish authenticated identity.
    #[must_use]
    pub fn with_correlation_reference(
        mut self,
        reference: CompletionReference,
    ) -> Self {
        self.correlation_reference = Some(reference);
        self
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the completion source reference.
    #[must_use]
    pub fn source_reference(&self) -> &CompletionReference {
        &self.source_reference
    }

    /// Returns the optional physical device identity.
    #[must_use]
    pub fn device(&self) -> Option<&HardwareDeviceIdentity> {
        self.device.as_ref()
    }

    /// Returns the optional submission receipt identity.
    #[must_use]
    pub fn submission_receipt_id(
        &self,
    ) -> Option<&SubmissionReceiptId> {
        self.submission_receipt_id.as_ref()
    }

    /// Returns the optional correlation reference.
    #[must_use]
    pub fn correlation_reference(
        &self,
    ) -> Option<&CompletionReference> {
        self.correlation_reference.as_ref()
    }
}

// ============================================================================
// PARTIAL COMPLETION EVIDENCE
// ============================================================================

/// Evidence describing a partially completed operation.
///
/// The reference is interpreted by the owning domain.
///
/// It may identify:
///
/// - Completed work units.
/// - Completed execution segments.
/// - Completed stages.
/// - Partial physical realization.
/// - A provider-side execution report.
///
/// ZHAL does not impose a universal work-unit representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartialCompletionEvidence {
    reference: CompletionReference,

    description: Option<String>,
}

impl PartialCompletionEvidence {
    /// Constructs partial completion evidence.
    ///
    /// A valid provider-owned reference is required.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the optional description is
    /// empty or contains control characters.
    pub fn new(
        reference: CompletionReference,
        description: Option<String>,
    ) -> ZhalResult<Self> {
        if let Some(description) = &description {
            if description.trim().is_empty()
                || description.chars().any(char::is_control)
            {
                return Err(completion_error(
                    ZhalErrorKind::InvalidInput,
                    "partial completion description is invalid",
                ));
            }
        }

        Ok(Self {
            reference,
            description,
        })
    }

    /// Returns the provider-owned evidence reference.
    #[must_use]
    pub fn reference(&self) -> &CompletionReference {
        &self.reference
    }

    /// Returns the optional explanatory description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// COMPLETION FAILURE
// ============================================================================

/// Structured information about an operation failure.
///
/// This is a portable failure description.
///
/// The canonical ZHAL error taxonomy is reused.
///
/// The original domain-specific error may additionally be retained
/// by the owning adapter or runtime as a causal diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletionFailure {
    kind: ZhalErrorKind,

    message: String,

    code: Option<String>,

    outcome: ZhalOperationOutcome,

    retryability: ZhalRetryability,
}

impl CompletionFailure {
    /// Constructs structured completion failure information.
    ///
    /// The outcome must establish either:
    ///
    /// - The operation failed.
    /// - The operation never started.
    ///
    /// Unknown outcomes must use CompletionOutcome::Unknown.
    ///
    /// Partial completion must use CompletionOutcome::PartiallyCompleted.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput for inconsistent failure information.
    pub fn new(
        kind: ZhalErrorKind,
        message: impl Into<String>,
        code: Option<String>,
        outcome: ZhalOperationOutcome,
        retryability: ZhalRetryability,
    ) -> ZhalResult<Self> {
        let message = message.into();

        if message.trim().is_empty() {
            return Err(completion_error(
                ZhalErrorKind::InvalidInput,
                "completion failure message must not be empty",
            ));
        }

        if message.chars().any(char::is_control) {
            return Err(completion_error(
                ZhalErrorKind::InvalidInput,
                "completion failure message contains control characters",
            ));
        }

        if let Some(code) = &code {
            if code.trim().is_empty()
                || code.chars().any(char::is_control)
            {
                return Err(completion_error(
                    ZhalErrorKind::InvalidInput,
                    "completion failure code is invalid",
                ));
            }
        }

        match outcome {
            ZhalOperationOutcome::Failed
            | ZhalOperationOutcome::NotStarted => {}

            _ => {
                return Err(completion_error(
                    ZhalErrorKind::InvalidInput,
                    "completion failure has an inconsistent operation outcome",
                ));
            }
        }

        Ok(Self {
            kind,
            message,
            code,
            outcome,
            retryability,
        })
    }

    /// Returns the canonical ZHAL error classification.
    #[must_use]
    pub const fn kind(&self) -> ZhalErrorKind {
        self.kind
    }

    /// Returns the safe human-readable failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the optional provider-specific error code.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns the established physical operation outcome.
    #[must_use]
    pub const fn outcome(&self) -> ZhalOperationOutcome {
        self.outcome
    }

    /// Returns the advisory retryability classification.
    ///
    /// This does not trigger retries.
    #[must_use]
    pub const fn retryability(&self) -> ZhalRetryability {
        self.retryability
    }

    /// Converts the structured failure into the canonical ZHAL error.
    ///
    /// The operation identity is attached to the resulting error.
    #[must_use]
    pub fn into_zhal_error(
        self,
        operation: HardwareOperationIdentity,
    ) -> ZhalError {
        let error = ZhalError::new(
            self.kind,
            self.message,
        )
        .with_retryability(self.retryability)
        .with_operation(operation);

        error
    }
}

// ============================================================================
// COMPLETION OUTCOME
// ============================================================================

/// Complete observation of a hardware operation.
///
/// The generic result type remains owned by the computational domain.
///
/// This enum makes inconsistent combinations of status, failure
/// and result substantially harder to represent.
///
/// In particular, an unknown outcome cannot accidentally be
/// represented as a successful or failed operation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CompletionOutcome<T> {
    /// Execution is known to be in progress.
    ///
    /// A progress reference is optional because not every
    /// provider exposes incremental progress information.
    InProgress {
        /// Optional provider-owned progress reference.
        progress_reference: Option<CompletionReference>,
    },

    /// The complete operation successfully completed.
    ///
    /// The result payload is owned by the computational domain.
    Succeeded {
        /// Domain-owned operation result.
        result: T,
    },

    /// Some, but not all, requested work completed.
    ///
    /// The optional result may contain the completed portion's
    /// domain-owned output.
    ///
    /// Evidence is mandatory.
    PartiallyCompleted {
        /// Optional partial domain result.
        result: Option<T>,

        /// Provider-owned partial completion evidence.
        evidence: PartialCompletionEvidence,
    },

    /// The operation is known to have failed.
    Failed {
        /// Structured failure information.
        failure: CompletionFailure,
    },

    /// The provider confirms that cancellation reached a
    /// terminal state.
    ///
    /// This is not equivalent to merely requesting cancellation.
    Cancelled {
        /// Optional safe explanatory message.
        reason: Option<String>,
    },

    /// The final outcome cannot currently be established.
    ///
    /// The operation may have:
    ///
    /// - Not started.
    /// - Started.
    /// - Partially completed.
    /// - Fully completed.
    ///
    /// Reconciliation is required to establish the actual outcome.
    Unknown {
        /// Optional provider-owned reconciliation reference.
        reconciliation_reference: Option<CompletionReference>,
    },
}

impl<T> CompletionOutcome<T> {
    /// Returns the general completion status.
    #[must_use]
    pub const fn status(&self) -> CompletionStatus {
        match self {
            Self::InProgress { .. } => CompletionStatus::InProgress,

            Self::Succeeded { .. } => CompletionStatus::Succeeded,

            Self::PartiallyCompleted { .. } => {
                CompletionStatus::PartiallyCompleted
            }

            Self::Failed { .. } => CompletionStatus::Failed,

            Self::Cancelled { .. } => CompletionStatus::Cancelled,

            Self::Unknown { .. } => CompletionStatus::Unknown,
        }
    }

    /// Returns whether a domain-owned result is present.
    #[must_use]
    pub const fn has_result(&self) -> bool {
        matches!(
            self,
            Self::Succeeded { .. }
                | Self::PartiallyCompleted {
                    result: Some(_),
                    ..
                }
        )
    }

    /// Returns whether the operation has a successful terminal result.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Succeeded { .. })
    }

    /// Returns whether the operation completed only partially.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        matches!(self, Self::PartiallyCompleted { .. })
    }

    /// Returns whether the operation is known to have failed.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    /// Returns whether the operation is confirmed cancelled.
    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled { .. })
    }

    /// Returns whether the outcome remains unknown.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown { .. })
    }

    /// Returns a borrowed successful result, if present.
    #[must_use]
    pub fn result(&self) -> Option<&T> {
        match self {
            Self::Succeeded { result } => Some(result),

            Self::PartiallyCompleted {
                result: Some(result),
                ..
            } => Some(result),

            _ => None,
        }
    }

    /// Consumes the outcome and returns its optional result.
    #[must_use]
    pub fn into_result(self) -> Option<T> {
        match self {
            Self::Succeeded { result } => Some(result),

            Self::PartiallyCompleted {
                result,
                ..
            } => result,

            _ => None,
        }
    }
}

// ============================================================================
// COMPLETION RECORD
// ============================================================================

/// Immutable provider-independent hardware operation completion record.
///
/// A completion record combines:
///
/// - Individual operation identity.
/// - Provider/device provenance.
/// - Optional submission correlation.
/// - Completion outcome.
/// - Domain-owned result.
///
/// The record does not own the execution lifecycle manager.
///
/// It represents an observation supplied by a provider or
/// an appropriate domain adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletionRecord<T> {
    operation: HardwareOperationIdentity,

    provenance: CompletionProvenance,

    outcome: CompletionOutcome<T>,
}

impl<T> CompletionRecord<T> {
    /// Constructs and validates a completion record.
    ///
    /// Provider and operation identities must agree.
    ///
    /// If a device is specified, it must belong to the same provider.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when provenance and operation
    /// identities are inconsistent.
    pub fn new(
        operation: HardwareOperationIdentity,
        provenance: CompletionProvenance,
        outcome: CompletionOutcome<T>,
    ) -> ZhalResult<Self> {
        validate_provenance(&operation, &provenance)?;

        validate_outcome(&operation, &outcome)?;

        Ok(Self {
            operation,
            provenance,
            outcome,
        })
    }

    /// Returns the individual hardware operation identity.
    #[must_use]
    pub fn operation(&self) -> &HardwareOperationIdentity {
        &self.operation
    }

    /// Returns completion provenance.
    #[must_use]
    pub fn provenance(&self) -> &CompletionProvenance {
        &self.provenance
    }

    /// Returns the completion outcome.
    #[must_use]
    pub fn outcome(&self) -> &CompletionOutcome<T> {
        &self.outcome
    }

    /// Returns the completion status.
    #[must_use]
    pub const fn status(&self) -> CompletionStatus {
        self.outcome.status()
    }

    /// Returns a borrowed domain result, if present.
    #[must_use]
    pub fn result(&self) -> Option<&T> {
        self.outcome.result()
    }

    /// Consumes the completion record and returns its domain result.
    #[must_use]
    pub fn into_result(self) -> Option<T> {
        self.outcome.into_result()
    }

    /// Returns whether the outcome is terminal.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.outcome.status().is_terminal()
    }

    /// Validates the completion against its original submission receipt.
    ///
    /// The submission and completion must identify the same
    /// individual operation.
    ///
    /// A provider that explicitly rejected a submission cannot
    /// subsequently report its execution as a valid completion
    /// under that same receipt.
    ///
    /// An indeterminate submission remains eligible for
    /// reconciliation.
    ///
    /// # Errors
    ///
    /// Returns ContractViolation when the submission and
    /// completion contradict each other.
    pub fn validate_against_submission(
        &self,
        receipt: &SubmissionReceipt,
    ) -> ZhalResult<()> {
        if receipt.operation() != &self.operation {
            return Err(operation_error(
                ZhalErrorKind::ContractViolation,
                "completion operation does not match submission receipt",
                &self.operation,
            ));
        }

        if receipt.is_rejected() {
            return Err(operation_error(
                ZhalErrorKind::ContractViolation,
                "rejected submission cannot establish execution completion",
                &self.operation,
            ));
        }

        if let Some(provenance_receipt_id) =
            self.provenance.submission_receipt_id()
        {
            if receipt.receipt_id() != Some(provenance_receipt_id) {
                return Err(operation_error(
                    ZhalErrorKind::ContractViolation,
                    "completion provenance references another submission receipt",
                    &self.operation,
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Validates completion provenance against an operation identity.
fn validate_provenance(
    operation: &HardwareOperationIdentity,
    provenance: &CompletionProvenance,
) -> ZhalResult<()> {
    if operation.provider_id() != provenance.provider_id() {
        return Err(operation_error(
            ZhalErrorKind::InvalidInput,
            "completion provenance belongs to another provider",
            operation,
        ));
    }

    if let Some(device) = provenance.device() {
        if device.provider_id() != operation.provider_id() {
            return Err(operation_error(
                ZhalErrorKind::InvalidInput,
                "completion device belongs to another provider",
                operation,
            ));
        }
    }

    Ok(())
}

/// Validates the structural consistency of a completion outcome.
fn validate_outcome<T>(
    operation: &HardwareOperationIdentity,
    outcome: &CompletionOutcome<T>,
) -> ZhalResult<()> {
    match outcome {
        CompletionOutcome::InProgress {
            progress_reference: _,
        } => {}

        CompletionOutcome::Succeeded { .. } => {}

        CompletionOutcome::PartiallyCompleted {
            result: _,
            evidence: _,
        } => {}

        CompletionOutcome::Failed { failure } => {
            match failure.outcome() {
                ZhalOperationOutcome::Failed
                | ZhalOperationOutcome::NotStarted => {}

                _ => {
                    return Err(operation_error(
                        ZhalErrorKind::InvalidInput,
                        "failure outcome contradicts completion status",
                        operation,
                    ));
                }
            }
        }

        CompletionOutcome::Cancelled { reason } => {
            if let Some(reason) = reason {
                if reason.trim().is_empty()
                    || reason.chars().any(char::is_control)
                {
                    return Err(operation_error(
                        ZhalErrorKind::InvalidInput,
                        "cancellation reason is invalid",
                        operation,
                    ));
                }
            }
        }

        CompletionOutcome::Unknown {
            reconciliation_reference: _,
        } => {}
    }

    Ok(())
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Constructs a completion-specific error.
fn completion_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

/// Constructs an operation-attributed completion error.
fn operation_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
    operation: &HardwareOperationIdentity,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
        .with_operation(operation.clone())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceId;

    use crate::zhal::identity::{
        HardwareDeviceIdentity,
        HardwareOperationId,
    };

    fn provider_id() -> ProviderId {
        ProviderId::new("provider.test")
            .expect("valid provider identity")
    }

    fn operation_identity() -> HardwareOperationIdentity {
        HardwareOperationIdentity::new(
            provider_id(),
            HardwareOperationId::new("operation.test")
                .expect("valid operation identity"),
        )
    }

    fn provenance() -> CompletionProvenance {
        CompletionProvenance::new(
            provider_id(),
            CompletionReference::new("provider.completion.001")
                .expect("valid completion reference"),
        )
    }

    // ------------------------------------------------------------------------
    // Completion references
    // ------------------------------------------------------------------------

    #[test]
    fn completion_reference_rejects_empty_values() {
        assert!(
            CompletionReference::new("").is_err()
        );
    }

    #[test]
    fn completion_reference_rejects_whitespace_only_values() {
        assert!(
            CompletionReference::new("   ").is_err()
        );
    }

    #[test]
    fn completion_reference_rejects_control_characters() {
        assert!(
            CompletionReference::new("reference\ninjected").is_err()
        );
    }

    #[test]
    fn completion_reference_preserves_exact_identity() {
        let reference = CompletionReference::new(
            "provider/quantum/completion-001"
        )
        .expect("valid reference");

        assert_eq!(
            reference.as_str(),
            "provider/quantum/completion-001"
        );
    }

    // ------------------------------------------------------------------------
    // Completion status
    // ------------------------------------------------------------------------

    #[test]
    fn successful_completion_is_terminal() {
        let outcome = CompletionOutcome::Succeeded {
            result: 42_u64,
        };

        assert_eq!(
            outcome.status(),
            CompletionStatus::Succeeded
        );

        assert!(outcome.status().is_terminal());
        assert!(outcome.is_success());
        assert!(outcome.has_result());
    }

    #[test]
    fn in_progress_is_not_terminal() {
        let outcome: CompletionOutcome<()> =
            CompletionOutcome::InProgress {
                progress_reference: None,
            };

        assert_eq!(
            outcome.status(),
            CompletionStatus::InProgress
        );

        assert!(!outcome.status().is_terminal());
    }

    #[test]
    fn unknown_outcome_is_not_terminal() {
        let outcome: CompletionOutcome<()> =
            CompletionOutcome::Unknown {
                reconciliation_reference: None,
            };

        assert_eq!(
            outcome.status(),
            CompletionStatus::Unknown
        );

        assert!(!outcome.status().is_terminal());
        assert!(outcome.is_unknown());
        assert!(!outcome.is_failure());
    }

    // ------------------------------------------------------------------------
    // Partial completion
    // ------------------------------------------------------------------------

    #[test]
    fn partial_completion_requires_evidence() {
        let evidence = PartialCompletionEvidence::new(
            CompletionReference::new("partial.result.001")
                .expect("valid reference"),
            Some("completed execution segments".to_owned()),
        )
        .expect("valid evidence");

        let outcome = CompletionOutcome::PartiallyCompleted {
            result: Some(42_u64),
            evidence,
        };

        assert!(outcome.is_partial());
        assert!(outcome.has_result());

        assert_eq!(
            outcome.status(),
            CompletionStatus::PartiallyCompleted
        );
    }

    #[test]
    fn partial_completion_supports_missing_partial_payload() {
        let evidence = PartialCompletionEvidence::new(
            CompletionReference::new("partial.evidence")
                .unwrap(),
            None,
        )
        .unwrap();

        let outcome: CompletionOutcome<()> =
            CompletionOutcome::PartiallyCompleted {
                result: None,
                evidence,
            };

        assert!(outcome.is_partial());
        assert!(!outcome.has_result());
    }

    // ------------------------------------------------------------------------
    // Failure validation
    // ------------------------------------------------------------------------

    #[test]
    fn failure_rejects_empty_messages() {
        assert!(
            CompletionFailure::new(
                ZhalErrorKind::OperationFailed,
                "",
                None,
                ZhalOperationOutcome::Failed,
                ZhalRetryability::Unknown,
            )
            .is_err()
        );
    }

    #[test]
    fn failure_rejects_unknown_outcome_classification() {
        assert!(
            CompletionFailure::new(
                ZhalErrorKind::OperationFailed,
                "operation failed",
                None,
                ZhalOperationOutcome::Unknown,
                ZhalRetryability::Unknown,
            )
            .is_err()
        );
    }

    #[test]
    fn failure_preserves_retryability() {
        let failure = CompletionFailure::new(
            ZhalErrorKind::HardwareCommunication,
            "hardware communication failed",
            Some("hardware.communication.failed".to_owned()),
            ZhalOperationOutcome::Failed,
            ZhalRetryability::AfterConditionChange,
        )
        .expect("valid failure");

        assert_eq!(
            failure.retryability(),
            ZhalRetryability::AfterConditionChange
        );

        assert_eq!(
            failure.code(),
            Some("hardware.communication.failed")
        );
    }

    // ------------------------------------------------------------------------
    // Provenance validation
    // ------------------------------------------------------------------------

    #[test]
    fn completion_rejects_another_provider() {
        let operation = operation_identity();

        let other_provider = ProviderId::new(
            "provider.other"
        )
        .unwrap();

        let provenance = CompletionProvenance::new(
            other_provider,
            CompletionReference::new("completion.other")
                .unwrap(),
        );

        let result = CompletionRecord::new(
            operation,
            provenance,
            CompletionOutcome::Succeeded {
                result: (),
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn completion_rejects_device_from_another_provider() {
        let device = HardwareDeviceIdentity::new(
            ProviderId::new("provider.other").unwrap(),
            DeviceId::new("device.test").unwrap(),
        );

        let result = provenance()
            .with_device(device);

        assert!(result.is_err());
    }

    #[test]
    fn completion_accepts_matching_device_provenance() {
        let device = HardwareDeviceIdentity::new(
            provider_id(),
            DeviceId::new("device.test").unwrap(),
        );

        assert!(
            provenance()
                .with_device(device)
                .is_ok()
        );
    }

    // ------------------------------------------------------------------------
    // Completion record
    // ------------------------------------------------------------------------

    #[test]
    fn completion_record_preserves_operation_identity() {
        let operation = operation_identity();

        let record = CompletionRecord::new(
            operation.clone(),
            provenance(),
            CompletionOutcome::Succeeded {
                result: "result",
            },
        )
        .expect("valid completion");

        assert_eq!(
            record.operation(),
            &operation
        );

        assert_eq!(
            record.status(),
            CompletionStatus::Succeeded
        );

        assert_eq!(
            record.result(),
            Some(&"result")
        );
    }

    #[test]
    fn completion_record_preserves_unknown_outcome() {
        let record = CompletionRecord::new(
            operation_identity(),
            provenance(),
            CompletionOutcome::<()>::Unknown {
                reconciliation_reference: Some(
                    CompletionReference::new("reconcile.001")
                        .unwrap(),
                ),
            },
        )
        .unwrap();

        assert_eq!(
            record.status(),
            CompletionStatus::Unknown
        );

        assert!(!record.is_terminal());
    }

    #[test]
    fn completion_record_rejects_invalid_cancellation_reason() {
        let result = CompletionRecord::new(
            operation_identity(),
            provenance(),
            CompletionOutcome::<()>::Cancelled {
                reason: Some("   ".to_owned()),
            },
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Scalability
    // ------------------------------------------------------------------------

    #[test]
    fn completion_references_have_no_arbitrary_length_ceiling() {
        let value = "completion.reference.".repeat(8192);

        let reference = CompletionReference::new(value.clone())
            .expect("valid extended reference");

        assert_eq!(reference.as_str(), value);
    }

    #[test]
    fn generic_payload_supports_domain_owned_data() {
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct QuantumResult {
            measurement_reference: String,
            measurement_data: Vec<u8>,
        }

        let result = QuantumResult {
            measurement_reference: "quantum.measurement.001".to_owned(),
            measurement_data: vec![0, 1, 1, 0],
        };

        let record = CompletionRecord::new(
            operation_identity(),
            provenance(),
            CompletionOutcome::Succeeded {
                result: result.clone(),
            },
        )
        .unwrap();

        assert_eq!(
            record.result(),
            Some(&result)
        );
    }

    #[test]
    fn completion_collections_can_grow_with_available_resources() {
        let mut records = Vec::new();

        for index in 0..512 {
            let operation = HardwareOperationIdentity::new(
                provider_id(),
                HardwareOperationId::new(
                    format!("operation.{index}")
                )
                .unwrap(),
            );

            let record = CompletionRecord::new(
                operation,
                provenance(),
                CompletionOutcome::Succeeded {
                    result: index,
                },
            )
            .unwrap();

            records.push(record);
        }

        assert_eq!(records.len(), 512);
    }
}
