
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Operation Cancellation
//!
//! Path: src/zhal/operations/cancellation.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides the provider-independent cancellation boundary for
//! individual hardware operations.
//!
//! Supports:
//!
//! - Classical hardware operations.
//! - Quantum hardware operations.
//! - HDL realization operations.
//! - Accelerator operations.
//! - Embedded hardware operations.
//! - Hybrid operations.
//! - Distributed operations.
//! - Remote hardware operations.
//! - Future computational technologies.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Cancellation request representation.
//! - Cancellation reference validation.
//! - Provider cancellation interface.
//! - Asynchronous cancellation dispatch.
//! - Cancellation acknowledgement representation.
//! - Cancellation outcome classification.
//! - Submission/cancellation correlation.
//! - Cancellation contract validation.
//! - Cancellation response validation.
//!
//! This module does NOT own:
//!
//! - Cancellation token implementation.
//! - Standard-library cancellation state.
//! - Runtime cancellation state.
//! - Distributed cancellation state.
//! - Hardware operation identity.
//! - Provider identity.
//! - Provider registration.
//! - Provider lifecycle.
//! - Hardware discovery.
//! - Device selection.
//! - Resource allocation.
//! - Scheduling.
//! - Routing.
//! - Physical hardware interruption.
//! - Quantum execution semantics.
//! - HDL realization semantics.
//! - Operation completion tracking.
//! - Automatic retry.
//! - Runtime execution orchestration.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Standard-library cancellation:
//!     crate::stdlib::context
//!
//! ZHAL operation context:
//!     crate::zhal::context
//!
//! Operation identity:
//!     crate::zhal::identity
//!
//! Operation errors:
//!     crate::zhal::errors
//!
//! Operation lifecycle semantics:
//!     crate::zhal::operations::contract
//!
//! Submission:
//!     crate::zhal::operations::submission
//!
//! Completion:
//!     crate::zhal::operations::completion
//!
//! Runtime cancellation:
//!     crate::zur
//!
//! Distributed cancellation:
//!     crate::distributed::execution::cancellation
//!
//! ## CRITICAL CANCELLATION GUARANTEE
//!
//! A cancellation request is NOT proof that physical execution stopped.
//!
//! The following states are distinct:
//!
//! 1. Cancellation requested.
//! 2. Provider accepted the cancellation request.
//! 3. Provider confirmed terminal cancellation.
//! 4. Provider confirmed that execution had already completed.
//! 5. Cancellation outcome remains unknown.
//!
//! Only the third state represents confirmed cancellation.
//!
//! An accepted cancellation request must never be represented
//! as confirmed physical termination.
//!
//! ## CONTEXT CANCELLATION
//!
//! ZhalContext exposes the canonical standard-library
//! CancellationSignal.
//!
//! This module does not introduce another cancellation token.
//!
//! An already-cancelled caller context must not automatically
//! prevent dispatching a cancellation request.
//!
//! Cancellation is a control operation and must remain available
//! when the operation context has already received cancellation.
//!
//! The provider must receive the context and independently
//! observe its cancellation/deadline requirements according
//! to its control-plane implementation.
//!
//! ## UNKNOWN OUTCOMES
//!
//! If a provider cannot establish whether execution stopped,
//! it must return an indeterminate outcome.
//!
//! It must not fabricate successful cancellation.
//!
//! A lost remote acknowledgement does not establish failure
//! or successful termination.
//!
//! Reconciliation belongs to the provider and runtime authorities.
//!
//! ## POCO-REAF
//!
//! This module imposes no arbitrary limits on:
//!
//! - Operation count.
//! - Provider count.
//! - Device count.
//! - Cancellation requests.
//! - Distributed participants.
//! - Hardware categories.
//! - Cancellation reference lengths.
//! - Computational domains.
//!
//! Memory usage is constrained by actual available resources
//! and platform representation limits.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## GRAMMAR BOUNDARY
//!
//! The grammar describes source-level execution and lifecycle intent.
//!
//! This module implements the physical hardware-provider
//! cancellation boundary.
//!
//! It does not parse grammar or redefine semantic effects.

use core::fmt;
use core::future::Future;
use core::pin::Pin;

use crate::stdlib::provider::ProviderId;

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::HardwareOperationIdentity;

use crate::zhal::operations::contract::CancellationSemantics;

use crate::zhal::operations::submission::{
    SubmissionReceipt,
    SubmissionReceiptId,
};

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.operations.cancellation";

/// Cancellation schema version.
///
/// Independent of the language, package, provider API and
/// hardware firmware versions.
pub const CANCELLATION_SCHEMA_VERSION: &str = "1";

// ============================================================================
// FUTURE
// ============================================================================

/// Future returned by a hardware operation cancellation provider.
///
/// The future is Send so it can be coordinated by a compatible
/// multithreaded runtime.
///
/// ZHAL does not create or select an asynchronous executor.
///
/// The runtime owns asynchronous polling and execution.
pub type CancellationFuture<'a> =
    Pin<
        Box<
            dyn Future<Output = ZhalResult<CancellationReceipt>>
                + Send
                + 'a
        >
    >;

// ============================================================================
// CANCELLATION REFERENCE
// ============================================================================

/// Opaque provider-owned cancellation reference.
///
/// This may identify:
///
/// - A cancellation request.
/// - Cancellation evidence.
/// - A reconciliation request.
/// - A provider-specific cancellation record.
///
/// ZHAL does not interpret the reference.
///
/// No arbitrary maximum length is imposed.
///
/// References are descriptive metadata, not authorization
/// credentials.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CancellationReference(String);

impl CancellationReference {
    /// Constructs a cancellation reference.
    ///
    /// The original value is preserved.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the value is empty,
    /// whitespace-only or contains control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(cancellation_error(
                ZhalErrorKind::InvalidInput,
                "cancellation reference must not be empty",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(cancellation_error(
                ZhalErrorKind::InvalidInput,
                "cancellation reference contains control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the exact cancellation reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CancellationReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// CANCELLATION REQUEST
// ============================================================================

/// Immutable request to cancel an individual hardware operation.
///
/// The request identifies an operation rather than a provider's
/// generic operation definition.
///
/// An optional submission receipt correlates the cancellation
/// with an earlier provider submission.
///
/// The receipt may be absent when:
///
/// - Cancellation is requested before a receipt is available.
/// - Submission acknowledgement was lost.
/// - A provider supports operation-identity-based cancellation.
/// - An operation is being reconciled after an indeterminate submission.
///
/// A rejected submission cannot be used as evidence that
/// an accepted operation exists.
///
/// The provider remains responsible for determining whether
/// the operation can actually be cancelled.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CancellationRequest {
    operation: HardwareOperationIdentity,

    submission_receipt_id: Option<SubmissionReceiptId>,
}

impl CancellationRequest {
    /// Constructs a cancellation request.
    ///
    /// When a submission receipt is provided, it must identify
    /// the same operation.
    ///
    /// An explicitly rejected submission is not cancellable
    /// through that receipt.
    ///
    /// An indeterminate or partially accepted submission remains
    /// eligible for cancellation because execution may have started.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when:
    ///
    /// - The receipt belongs to another operation.
    /// - The submission was explicitly rejected.
    pub fn new(
        operation: HardwareOperationIdentity,
        submission: Option<&SubmissionReceipt>,
    ) -> ZhalResult<Self> {
        let submission_receipt_id = match submission {
            None => None,

            Some(receipt) => {
                if receipt.operation() != &operation {
                    return Err(
                        operation_error(
                            ZhalErrorKind::InvalidInput,
                            "submission receipt belongs to another operation",
                            &operation,
                        )
                    );
                }

                if receipt.is_rejected() {
                    return Err(
                        operation_error(
                            ZhalErrorKind::InvalidInput,
                            "a rejected submission cannot establish a cancellable operation",
                            &operation,
                        )
                    );
                }

                receipt.receipt_id().cloned()
            }
        };

        Ok(Self {
            operation,
            submission_receipt_id,
        })
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn operation(&self) -> &HardwareOperationIdentity {
        &self.operation
    }

    /// Returns the optional submission receipt identity.
    #[must_use]
    pub fn submission_receipt_id(
        &self,
    ) -> Option<&SubmissionReceiptId> {
        self.submission_receipt_id.as_ref()
    }

    /// Returns the associated provider.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        self.operation.provider_id()
    }
}

// ============================================================================
// CANCELLATION OUTCOME
// ============================================================================

/// Provider-reported outcome of a cancellation request.
///
/// The variants deliberately distinguish acknowledgement
/// from confirmed physical termination.
///
/// In particular:
///
/// `RequestAccepted` does NOT mean `Cancelled`.
///
/// Only `Cancelled` means the provider confirms that the
/// operation reached a cancelled terminal state.
///
/// Unknown outcomes must remain unknown.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CancellationOutcome {
    /// The provider accepted the cancellation request.
    ///
    /// The operation may still be executing.
    ///
    /// This is an acknowledgement of the control request,
    /// not confirmation of physical termination.
    RequestAccepted {
        /// Optional provider-owned cancellation request reference.
        request_reference: Option<CancellationReference>,
    },

    /// The provider confirms that the operation reached
    /// a cancelled terminal state.
    ///
    /// The evidence reference may identify the provider's
    /// cancellation confirmation.
    ///
    /// The provider must only return this outcome when it
    /// can substantiate the terminal cancellation state.
    Cancelled {
        /// Optional provider-owned cancellation evidence.
        evidence: Option<CancellationReference>,
    },

    /// The provider confirms that the operation had already
    /// completed before cancellation could take effect.
    ///
    /// This is not a cancellation success.
    AlreadyCompleted {
        /// Optional provider-owned completion reference.
        completion_reference: Option<CancellationReference>,
    },

    /// The provider explicitly rejected the cancellation request.
    ///
    /// The operation may still be executing.
    ///
    /// A rejection does not establish the operation's terminal state.
    Rejected {
        /// Optional safe explanatory message.
        reason: Option<String>,
    },

    /// The provider cannot currently establish the final
    /// cancellation outcome.
    ///
    /// This commonly occurs when a remote acknowledgement
    /// is lost or execution state cannot be queried.
    ///
    /// Reconciliation is required before claiming termination.
    Indeterminate {
        /// Optional provider-owned reconciliation reference.
        reconciliation_reference: Option<CancellationReference>,
    },
}

impl CancellationOutcome {
    /// Returns whether terminal cancellation is confirmed.
    #[must_use]
    pub const fn is_confirmed_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled { .. })
    }

    /// Returns whether the provider accepted the request
    /// without confirming terminal cancellation.
    #[must_use]
    pub const fn is_request_accepted(&self) -> bool {
        matches!(self, Self::RequestAccepted { .. })
    }

    /// Returns whether the operation was already completed.
    #[must_use]
    pub const fn is_already_completed(&self) -> bool {
        matches!(self, Self::AlreadyCompleted { .. })
    }

    /// Returns whether the provider rejected the request.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected { .. })
    }

    /// Returns whether the outcome is indeterminate.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self, Self::Indeterminate { .. })
    }

    /// Returns whether terminal cancellation has been established.
    ///
    /// Accepted, rejected and indeterminate requests do not
    /// establish terminal cancellation.
    #[must_use]
    pub const fn has_confirmed_terminal_cancellation(&self) -> bool {
        self.is_confirmed_cancelled()
    }
}

// ============================================================================
// CANCELLATION RECEIPT
// ============================================================================

/// Immutable provider acknowledgement of a cancellation request.
///
/// The receipt records the provider's reported outcome.
///
/// It does not independently prove that the physical hardware
/// has stopped. Its evidentiary meaning is determined by the
/// provider contract and the outcome variant.
///
/// The operation identity must match the original request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CancellationReceipt {
    operation: HardwareOperationIdentity,

    outcome: CancellationOutcome,
}

impl CancellationReceipt {
    /// Constructs a provider cancellation receipt.
    ///
    /// The operation identity must match the original request.
    ///
    /// # Errors
    ///
    /// Returns ContractViolation when the provider responds
    /// for a different operation.
    ///
    /// Returns InvalidInput when the provider supplies invalid
    /// cancellation metadata.
    pub fn new(
        request: &CancellationRequest,
        operation: HardwareOperationIdentity,
        outcome: CancellationOutcome,
    ) -> ZhalResult<Self> {
        if request.operation() != &operation {
            return Err(
                operation_error(
                    ZhalErrorKind::ContractViolation,
                    "cancellation receipt identifies another operation",
                    request.operation(),
                )
            );
        }

        validate_outcome(
            &operation,
            &outcome,
        )?;

        Ok(Self {
            operation,
            outcome,
        })
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn operation(&self) -> &HardwareOperationIdentity {
        &self.operation
    }

    /// Returns the cancellation outcome.
    #[must_use]
    pub fn outcome(&self) -> &CancellationOutcome {
        &self.outcome
    }

    /// Returns whether cancellation is confirmed.
    #[must_use]
    pub const fn is_confirmed_cancelled(&self) -> bool {
        self.outcome.is_confirmed_cancelled()
    }

    /// Returns whether the cancellation request was accepted.
    ///
    /// Acceptance does not establish physical termination.
    #[must_use]
    pub const fn is_request_accepted(&self) -> bool {
        self.outcome.is_request_accepted()
    }

    /// Returns whether the final cancellation outcome is unknown.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.outcome.is_indeterminate()
    }

    /// Returns whether terminal cancellation has been established.
    #[must_use]
    pub const fn has_confirmed_terminal_cancellation(&self) -> bool {
        self.outcome.has_confirmed_terminal_cancellation()
    }
}

// ============================================================================
// PROVIDER CANCELLATION INTERFACE
// ============================================================================

/// Universal hardware operation cancellation interface.
///
/// Classical, quantum, HDL, accelerator, embedded and distributed
/// providers may implement this interface.
///
/// The provider owns the actual cancellation mechanism.
///
/// Examples include:
///
/// - Cancelling an accelerator command.
/// - Interrupting a cooperative processor task.
/// - Requesting cancellation from a remote execution service.
/// - Stopping a supported quantum execution job.
/// - Cancelling a hardware realization operation.
///
/// The interface deliberately does not require every hardware
/// technology to support the same physical interruption mechanism.
///
/// # Thread safety
///
/// Implementations must be Send + Sync.
///
/// # Provider requirements
///
/// Implementations must:
///
/// - Validate the operation identity.
/// - Respect the declared cancellation semantics.
/// - Preserve submission correlation.
/// - Avoid claiming termination without evidence.
/// - Preserve unknown outcomes.
/// - Preserve partial execution information through their
///   appropriate domain-specific completion authority.
/// - Respect provider security and authorization boundaries.
/// - Avoid changing operation semantics.
/// - Avoid blindly repeating cancellation side effects.
/// - Return a receipt for the requested operation.
///
/// The provider must not silently treat a cancellation request
/// as successful physical termination.
pub trait HardwareOperationCanceller: Send + Sync {
    /// Requests cancellation of an individual hardware operation.
    ///
    /// This method requests cancellation.
    ///
    /// It does not itself establish that physical execution stopped.
    ///
    /// The future must not outlive the provider or context.
    ///
    /// An uncertain remote acknowledgement must be represented
    /// by CancellationOutcome::Indeterminate.
    ///
    /// An accepted request must be represented by
    /// CancellationOutcome::RequestAccepted unless terminal
    /// cancellation has actually been established.
    fn request_cancellation<'a>(
        &'a self,
        request: CancellationRequest,
        context: &'a ZhalContext,
    ) -> CancellationFuture<'a>;
}

// ============================================================================
// CANCELLATION DISPATCH
// ============================================================================

/// Requests cancellation through a hardware provider.
///
/// This function performs cancellation-boundary validation
/// and provider dispatch.
///
/// It does not:
///
/// - Discover providers.
/// - Select devices.
/// - Allocate resources.
/// - Reserve hardware.
/// - Schedule execution.
/// - Perform retries.
/// - Implement physical interruption.
/// - Perform distributed routing.
/// - Reconcile unknown outcomes.
///
/// Those responsibilities remain with their respective authorities.
///
/// # Context behavior
///
/// This function intentionally does not call `context.check_active()`.
///
/// A context may already be cancelled precisely because its caller
/// is requesting cancellation.
///
/// Rejecting the cancellation request solely because that context
/// is cancelled would defeat cancellation propagation.
///
/// The provider receives the original context and can inspect
/// its cancellation and deadline observations.
///
/// # Cancellation semantics
///
/// Unsupported cancellation is rejected before provider dispatch.
///
/// Cooperative, best-effort and provider-defined cancellation
/// are dispatched through the provider interface.
///
/// # Errors
///
/// Returns a ZhalError when:
///
/// - Cancellation is explicitly unsupported.
/// - The context provider does not match the operation provider.
/// - The provider returns a receipt for another operation.
/// - The provider violates the cancellation contract.
/// - The provider reports invalid cancellation metadata.
/// - The provider returns an operational failure.
pub async fn request_cancellation<C>(
    canceller: &C,
    request: CancellationRequest,
    semantics: CancellationSemantics,
    context: &ZhalContext,
) -> ZhalResult<CancellationReceipt>
where
    C: HardwareOperationCanceller + ?Sized,
{
    let operation = request.operation().clone();

    // Cancellation must be routed to the provider that owns
    // the operation identity.
    if let Some(provider_id) = context.provider_id() {
        if provider_id != operation.provider_id() {
            return Err(
                operation_error(
                    ZhalErrorKind::InvalidInput,
                    "cancellation provider does not match operation provider",
                    &operation,
                )
            );
        }
    }

    // Cancellation semantics are obtained from the original
    // HardwareOperationContract by the caller.
    //
    // Do not silently turn unsupported cancellation into
    // a provider-specific best-effort operation.
    match semantics {
        CancellationSemantics::Unsupported => {
            return Err(
                operation_error(
                    ZhalErrorKind::CapabilityUnsupported,
                    "the operation contract does not support cancellation",
                    &operation,
                )
            );
        }

        CancellationSemantics::Cooperative
        | CancellationSemantics::BestEffort
        | CancellationSemantics::ProviderDefined => {}

        _ => {
            return Err(
                operation_error(
                    ZhalErrorKind::ContractViolation,
                    "unrecognized cancellation semantics",
                    &operation,
                )
            );
        }
    }

    // Do not check the caller's cancellation signal here.
    //
    // A cancellation control request must remain dispatchable
    // even when the associated operation context is cancelled.
    //
    // Likewise, an expired operation deadline must not
    // automatically suppress a cancellation control request.

    let receipt = canceller
        .request_cancellation(
            request,
            context,
        )
        .await?;

    validate_receipt(
        &operation,
        &receipt,
    )?;

    Ok(receipt)
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Validates provider cancellation outcome metadata.
fn validate_outcome(
    operation: &HardwareOperationIdentity,
    outcome: &CancellationOutcome,
) -> ZhalResult<()> {
    match outcome {
        CancellationOutcome::RequestAccepted {
            request_reference: _,
        } => {}

        CancellationOutcome::Cancelled {
            evidence: _,
        } => {}

        CancellationOutcome::AlreadyCompleted {
            completion_reference: _,
        } => {}

        CancellationOutcome::Rejected {
            reason,
        } => {
            if let Some(reason) = reason {
                if reason.trim().is_empty()
                    || reason.chars().any(char::is_control)
                {
                    return Err(
                        operation_error(
                            ZhalErrorKind::InvalidInput,
                            "cancellation rejection reason is invalid",
                            operation,
                        )
                    );
                }
            }
        }

        CancellationOutcome::Indeterminate {
            reconciliation_reference: _,
        } => {}
    }

    Ok(())
}

/// Validates the provider response against the original request.
///
/// A provider must never acknowledge cancellation of another
/// operation as though it were the requested operation.
fn validate_receipt(
    expected: &HardwareOperationIdentity,
    receipt: &CancellationReceipt,
) -> ZhalResult<()> {
    if receipt.operation() != expected {
        return Err(
            operation_error(
                ZhalErrorKind::ContractViolation,
                "provider returned a cancellation receipt for another operation",
                expected,
            )
        );
    }

    Ok(())
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Constructs a cancellation-specific error.
fn cancellation_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(
        kind,
        message,
    )
    .in_component(COMPONENT)
}

/// Constructs an operation-attributed cancellation error.
fn operation_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
    operation: &HardwareOperationIdentity,
) -> ZhalError {
    ZhalError::new(
        kind,
        message,
    )
    .in_component(COMPONENT)
    .with_operation(operation.clone())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::provider::ProviderId;

    use crate::zhal::identity::HardwareOperationId;

    use crate::zhal::operations::submission::{
        Acceptance,
        DispatchMode,
        SubmissionReceiptId,
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

    fn cancellation_request() -> CancellationRequest {
        CancellationRequest::new(
            operation_identity(),
            None,
        )
        .expect("valid cancellation request")
    }

    fn context() -> ZhalContext {
        ZhalContext::builder()
            .with_provider_id(provider_id())
            .build()
    }

    fn receipt(
        outcome: CancellationOutcome,
    ) -> CancellationReceipt {
        CancellationReceipt::new(
            &cancellation_request(),
            operation_identity(),
            outcome,
        )
        .expect("valid cancellation receipt")
    }

    // ------------------------------------------------------------------------
    // Cancellation references
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_reference_rejects_empty_values() {
        assert!(
            CancellationReference::new("").is_err()
        );
    }

    #[test]
    fn cancellation_reference_rejects_whitespace_only_values() {
        assert!(
            CancellationReference::new("   ").is_err()
        );
    }

    #[test]
    fn cancellation_reference_rejects_control_characters() {
        assert!(
            CancellationReference::new("reference\ninjected").is_err()
        );
    }

    #[test]
    fn cancellation_reference_preserves_exact_identity() {
        let reference = CancellationReference::new(
            "provider/remote/cancellation-001",
        )
        .expect("valid reference");

        assert_eq!(
            reference.as_str(),
            "provider/remote/cancellation-001"
        );
    }

    #[test]
    fn cancellation_reference_has_no_arbitrary_length_limit() {
        let value = "cancellation.reference.".repeat(4096);

        let reference = CancellationReference::new(
            value.clone(),
        )
        .expect("valid extended reference");

        assert_eq!(
            reference.as_str(),
            value
        );
    }

    // ------------------------------------------------------------------------
    // Cancellation requests
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_request_preserves_operation_identity() {
        let operation = operation_identity();

        let request = CancellationRequest::new(
            operation.clone(),
            None,
        )
        .expect("valid request");

        assert_eq!(
            request.operation(),
            &operation
        );

        assert_eq!(
            request.provider_id(),
            operation.provider_id()
        );

        assert!(
            request.submission_receipt_id().is_none()
        );
    }

    #[test]
    fn cancellation_request_rejects_another_operations_receipt() {
        let other_operation = HardwareOperationIdentity::new(
            ProviderId::new("provider.test").unwrap(),
            HardwareOperationId::new("operation.other").unwrap(),
        );

        let submission = SubmissionReceipt::new(
            other_operation,
            None,
            DispatchMode::Asynchronous,
            Acceptance::Accepted,
        )
        .unwrap();

        assert!(
            CancellationRequest::new(
                operation_identity(),
                Some(&submission),
            )
            .is_err()
        );
    }

    #[test]
    fn cancellation_request_rejects_explicitly_rejected_submission() {
        let submission = SubmissionReceipt::new(
            operation_identity(),
            None,
            DispatchMode::Asynchronous,
            Acceptance::Rejected {
                reason: Some("not accepted".to_owned()),
            },
        )
        .unwrap();

        assert!(
            CancellationRequest::new(
                operation_identity(),
                Some(&submission),
            )
            .is_err()
        );
    }

    #[test]
    fn cancellation_request_accepts_indeterminate_submission() {
        let submission = SubmissionReceipt::new(
            operation_identity(),
            None,
            DispatchMode::Remote,
            Acceptance::Indeterminate {
                reconciliation_reference: Some(
                    "reconcile.001".to_owned(),
                ),
            },
        )
        .unwrap();

        assert!(
            CancellationRequest::new(
                operation_identity(),
                Some(&submission),
            )
            .is_ok()
        );
    }

    #[test]
    fn cancellation_request_preserves_submission_receipt_identity() {
        let receipt_id = SubmissionReceiptId::new(
            "submission.receipt.001",
        )
        .unwrap();

        let submission = SubmissionReceipt::new(
            operation_identity(),
            Some(receipt_id.clone()),
            DispatchMode::Asynchronous,
            Acceptance::Accepted,
        )
        .unwrap();

        let request = CancellationRequest::new(
            operation_identity(),
            Some(&submission),
        )
        .unwrap();

        assert_eq!(
            request.submission_receipt_id(),
            Some(&receipt_id)
        );
    }

    // ------------------------------------------------------------------------
    // Cancellation outcomes
    // ------------------------------------------------------------------------

    #[test]
    fn accepted_request_is_not_confirmed_cancellation() {
        let result = receipt(
            CancellationOutcome::RequestAccepted {
                request_reference: Some(
                    CancellationReference::new("request.001")
                        .unwrap(),
                ),
            },
        );

        assert!(result.is_request_accepted());

        assert!(
            !result.is_confirmed_cancelled()
        );

        assert!(
            !result.has_confirmed_terminal_cancellation()
        );
    }

    #[test]
    fn confirmed_cancellation_is_terminal() {
        let result = receipt(
            CancellationOutcome::Cancelled {
                evidence: Some(
                    CancellationReference::new("evidence.001")
                        .unwrap(),
                ),
            },
        );

        assert!(result.is_confirmed_cancelled());

        assert!(
            result.has_confirmed_terminal_cancellation()
        );
    }

    #[test]
    fn already_completed_is_not_cancellation_success() {
        let result = receipt(
            CancellationOutcome::AlreadyCompleted {
                completion_reference: Some(
                    CancellationReference::new("completion.001")
                        .unwrap(),
                ),
            },
        );

        assert!(
            result.outcome().is_already_completed()
        );

        assert!(
            !result.is_confirmed_cancelled()
        );
    }

    #[test]
    fn rejected_request_is_not_confirmed_cancellation() {
        let result = receipt(
            CancellationOutcome::Rejected {
                reason: Some("operation cannot be interrupted".to_owned()),
            },
        );

        assert!(
            result.outcome().is_rejected()
        );

        assert!(
            !result.is_confirmed_cancelled()
        );
    }

    #[test]
    fn indeterminate_request_remains_unknown() {
        let result = receipt(
            CancellationOutcome::Indeterminate {
                reconciliation_reference: Some(
                    CancellationReference::new("reconcile.001")
                        .unwrap(),
                ),
            },
        );

        assert!(
            result.is_indeterminate()
        );

        assert!(
            !result.is_confirmed_cancelled()
        );
    }

    // ------------------------------------------------------------------------
    // Receipt validation
    // ------------------------------------------------------------------------

    #[test]
    fn receipt_rejects_another_operation() {
        let request = cancellation_request();

        let other_operation = HardwareOperationIdentity::new(
            provider_id(),
            HardwareOperationId::new("operation.other").unwrap(),
        );

        let result = CancellationReceipt::new(
            &request,
            other_operation,
            CancellationOutcome::Cancelled {
                evidence: None,
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn receipt_preserves_operation_identity() {
        let operation = operation_identity();

        let request = CancellationRequest::new(
            operation.clone(),
            None,
        )
        .unwrap();

        let result = CancellationReceipt::new(
            &request,
            operation.clone(),
            CancellationOutcome::Cancelled {
                evidence: None,
            },
        )
        .unwrap();

        assert_eq!(
            result.operation(),
            &operation
        );
    }

    // ------------------------------------------------------------------------
    // Cancellation semantics
    // ------------------------------------------------------------------------

    #[test]
    fn unsupported_cancellation_is_not_supported() {
        let semantics = CancellationSemantics::Unsupported;

        assert_eq!(
            semantics,
            CancellationSemantics::Unsupported
        );
    }

    #[test]
    fn cooperative_cancellation_is_supported() {
        let semantics = CancellationSemantics::Cooperative;

        assert_ne!(
            semantics,
            CancellationSemantics::Unsupported
        );
    }

    #[test]
    fn best_effort_cancellation_is_supported() {
        let semantics = CancellationSemantics::BestEffort;

        assert_ne!(
            semantics,
            CancellationSemantics::Unsupported
        );
    }

    // ------------------------------------------------------------------------
    // Context integration
    // ------------------------------------------------------------------------

    #[test]
    fn context_preserves_provider_identity() {
        let context = context();

        assert_eq!(
            context.provider_id(),
            Some(&provider_id())
        );
    }

    #[test]
    fn cancellation_context_is_not_a_second_token() {
        let context = context();

        assert!(
            !context.is_cancelled()
        );

        assert!(
            context.check_cancelled().is_ok()
        );
    }

    // ------------------------------------------------------------------------
    // Scalability
    // ------------------------------------------------------------------------

    #[test]
    fn multiple_cancellation_references_are_supported() {
        let references: Vec<_> = (0..512)
            .map(|index| {
                CancellationReference::new(
                    format!("cancellation.{index}"),
                )
                .unwrap()
            })
            .collect();

        assert_eq!(
            references.len(),
            512
        );
    }

    #[test]
    fn generic_operation_identity_is_domain_independent() {
        let operation = operation_identity();

        let request = CancellationRequest::new(
            operation.clone(),
            None,
        )
        .unwrap();

        assert_eq!(
            request.operation(),
            &operation
        );
    }
}
