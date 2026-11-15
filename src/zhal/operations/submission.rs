
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Operation Submission
//!
//! Path: `src/zhal/operations/submission.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the provider-independent submission boundary for
//! individual hardware operations.
//!
//! Supports:
//!
//! - Synchronous operation providers.
//! - Asynchronous operation providers.
//! - Remote operation providers.
//! - Classical hardware.
//! - Quantum hardware.
//! - HDL realization.
//! - Accelerators.
//! - Embedded hardware.
//! - Hybrid hardware.
//! - Distributed hardware.
//! - Future computational technologies.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Submission requests.
//! - Submission dispatch contracts.
//! - Provider submission interfaces.
//! - Submission receipts.
//! - Acceptance classification.
//! - Partial acceptance representation.
//! - Indeterminate submission outcomes.
//! - Idempotency-key representation.
//! - Submission validation.
//! - Submission context validation.
//! - Submission conformance requirements.
//!
//! This module does NOT own:
//!
//! - Operation payload semantics.
//! - Domain-specific operation definitions.
//! - Operation identity definitions.
//! - Provider identity definitions.
//! - Provider registration.
//! - Provider lifecycle management.
//! - Hardware discovery.
//! - Device selection.
//! - Capability evaluation.
//! - Resource allocation.
//! - Resource reservation.
//! - Scheduling.
//! - Routing.
//! - Execution orchestration.
//! - Operation completion tracking.
//! - Operation cancellation implementation.
//! - Distributed transport.
//! - Quantum execution.
//! - Quantum routing.
//! - HDL synthesis.
//! - Backend generation.
//!
//! ## Canonical authorities
//!
//! Operation contracts:
//!     crate::zhal::operations::contract
//!
//! Operation identities:
//!     crate::zhal::identity
//!
//! Provider identities:
//!     crate::stdlib::provider
//!
//! Hardware operation errors:
//!     crate::zhal::errors
//!
//! Hardware operation context:
//!     crate::zhal::context
//!
//! Cancellation:
//!     crate::stdlib::context
//!
//! Runtime orchestration:
//!     crate::zur
//!
//! Distributed execution:
//!     crate::distributed
//!
//! Quantum execution:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! ## POCO-REAF
//!
//! Submission is independent of:
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
//!
//! No arbitrary universal hardware ceilings are introduced.
//!
//! Literal infinite physical computation or storage is not promised.
//!
//! Growth is governed by available resources and explicitly
//! configured operational budgets.
//!
//! ## Important guarantees
//!
//! 1. An operation identity is supplied by the caller.
//! 2. An operation contract is distinct from an operation instance.
//! 3. Domain-specific payloads remain domain-owned.
//! 4. Submission does not imply execution completion.
//! 5. Partial acceptance remains explicit.
//! 6. Unknown outcomes remain explicit.
//! 7. A lost acknowledgement must not be interpreted as rejection.
//! 8. Unknown outcomes must not be blindly retried.
//! 9. Idempotency guarantees must be declared, never assumed.
//! 10. Cancellation does not prove physical termination.
//! 11. Remote submission does not imply a particular transport.
//! 12. No hardware resource is allocated by this module.
//!
//! ## Grammar boundary
//!
//! Relevant grammar:
//!
//! - grammar/effects/hardware.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//!
//! The grammar describes computational intent.
//!
//! This module handles the resulting physical submission boundary.
//!
//! It does not parse source syntax or reinterpret semantic requirements.

use core::fmt;
use core::future::Future;
use core::pin::Pin;

use crate::stdlib::provider::ProviderOperationId;

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::{
    HardwareOperationIdentity,
};

use crate::zhal::operations::contract::{
    HardwareOperationContract,
    SubmissionSemantics,
};

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.operations.submission";

/// Submission schema version.
///
/// Independent of the language, package, provider API and
/// hardware firmware versions.
pub const SUBMISSION_SCHEMA_VERSION: &str = "1";

// ============================================================================
// FUTURE
// ============================================================================

/// Future returned by a hardware operation submitter.
///
/// The future is `Send` so it can be driven by a compatible
/// multithreaded runtime.
///
/// ZHAL does not provide or select an asynchronous executor.
///
/// The runtime owns asynchronous polling and execution.
pub type SubmissionFuture<'a> =
    Pin<Box<dyn Future<Output = ZhalResult<SubmissionReceipt>> + Send + 'a>>;

// ============================================================================
// DISPATCH MODEL
// ============================================================================

/// Describes how a provider accepts responsibility for dispatching
/// a submitted operation.
///
/// This describes the provider's submission mechanism.
///
/// It does not describe the computation's semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DispatchMode {
    /// The provider completes the submission handshake synchronously.
    ///
    /// This does not necessarily mean that the physical operation
    /// itself completes synchronously.
    Synchronous,

    /// The provider accepts the operation for asynchronous processing.
    Asynchronous,

    /// The provider submits the operation to a remote execution
    /// environment.
    ///
    /// No specific transport protocol is implied.
    Remote,
}

impl DispatchMode {
    /// Returns the stable descriptive name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Synchronous => "synchronous",
            Self::Asynchronous => "asynchronous",
            Self::Remote => "remote",
        }
    }
}

impl fmt::Display for DispatchMode {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// IDEMPOTENCY KEY
// ============================================================================

/// Opaque caller-supplied idempotency key.
///
/// The key allows a provider to correlate repeated submission
/// attempts representing the same logical request.
///
/// This type does not generate keys or establish uniqueness.
///
/// Uniqueness scope and retention guarantees belong to the
/// provider's operation contract.
///
/// Keys are deliberately excluded from Debug output.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    /// Constructs an idempotency key.
    ///
    /// Exact spelling is preserved.
    ///
    /// No arbitrary maximum length is imposed.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput if the key is empty or contains
    /// control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(submission_error(
                ZhalErrorKind::InvalidInput,
                "idempotency key must not be empty",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(submission_error(
                ZhalErrorKind::InvalidInput,
                "idempotency key must not contain control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque key.
    ///
    /// Callers must treat the returned value as potentially
    /// sensitive operational metadata.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for IdempotencyKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_tuple("IdempotencyKey")
            .field(&"[REDACTED]")
            .finish()
    }
}

// ============================================================================
// SUBMISSION REQUEST
// ============================================================================

/// A provider-facing hardware operation submission request.
///
/// The payload type is generic because classical, quantum, HDL,
/// accelerator and other domains have different operation semantics.
///
/// ZHAL does not serialize, reinterpret or transform the payload.
///
/// The owning domain adapter is responsible for defining and
/// validating its payload contract.
///
/// A request is immutable after construction.
///
/// # Type parameters
///
/// `P` is the domain-owned operation payload.
pub struct SubmissionRequest<P> {
    operation: HardwareOperationIdentity,

    contract: HardwareOperationContract,

    payload: P,

    idempotency_key: Option<IdempotencyKey>,
}

impl<P> SubmissionRequest<P> {
    /// Constructs a submission request.
    ///
    /// The operation identity identifies an individual operation.
    ///
    /// The contract identifies the provider-supported operation
    /// definition.
    ///
    /// They are deliberately distinct.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the operation identity does not
    /// belong to the provider specified by the operation context.
    ///
    /// If the context does not specify a provider, the identity
    /// itself remains the provider association.
    pub fn new(
        operation: HardwareOperationIdentity,
        contract: HardwareOperationContract,
        payload: P,
        context: &ZhalContext,
    ) -> ZhalResult<Self> {
        if let Some(provider_id) = context.provider_id() {
            if provider_id != operation.provider_id() {
                return Err(
                    submission_error(
                        ZhalErrorKind::InvalidInput,
                        "operation provider does not match submission context",
                    )
                    .with_operation(operation),
                );
            }
        }

        Ok(Self {
            operation,
            contract,
            payload,
            idempotency_key: None,
        })
    }

    /// Attaches an idempotency key.
    ///
    /// This does not establish that retrying is safe.
    ///
    /// Retry behavior must agree with the operation contract
    /// and provider guarantees.
    #[must_use]
    pub fn with_idempotency_key(
        mut self,
        key: IdempotencyKey,
    ) -> Self {
        self.idempotency_key = Some(key);
        self
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn operation(&self) -> &HardwareOperationIdentity {
        &self.operation
    }

    /// Returns the canonical operation contract.
    #[must_use]
    pub fn contract(&self) -> &HardwareOperationContract {
        &self.contract
    }

    /// Returns the optional idempotency key.
    ///
    /// Implementations must not expose this key in ordinary
    /// diagnostics or logs.
    #[must_use]
    pub fn idempotency_key(&self) -> Option<&IdempotencyKey> {
        self.idempotency_key.as_ref()
    }

    /// Borrows the domain-owned payload.
    #[must_use]
    pub fn payload(&self) -> &P {
        &self.payload
    }

    /// Consumes the request and returns its domain payload.
    #[must_use]
    pub fn into_payload(self) -> P {
        self.payload
    }
}

// ============================================================================
// ACCEPTANCE CLASSIFICATION
// ============================================================================

/// Describes the provider's acceptance decision.
///
/// Acceptance does not mean execution has completed.
///
/// Provider-defined acceptance semantics must agree with
/// the associated HardwareOperationContract.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Acceptance {
    /// The provider accepted the complete submitted operation.
    Accepted,

    /// The provider accepted only part of the submitted operation.
    ///
    /// The evidence reference is opaque and interpreted by the
    /// operation's domain-specific contract.
    ///
    /// ZHAL does not assume that every domain has countable,
    /// independently addressable work units.
    PartiallyAccepted {
        /// Provider-owned reference to acceptance evidence.
        evidence_reference: String,
    },

    /// The provider explicitly rejected the operation.
    ///
    /// Rejection means the provider confirms that it did not
    /// accept this submission under the declared contract.
    Rejected {
        /// Optional safe explanatory message.
        reason: Option<String>,
    },

    /// The acceptance outcome cannot currently be established.
    ///
    /// This commonly occurs when a remote acknowledgement is lost
    /// after the provider may already have accepted the operation.
    ///
    /// Callers must reconcile the outcome before attempting
    /// another submission unless the operation contract explicitly
    /// provides safe idempotent retry behavior.
    Indeterminate {
        /// Optional provider-owned reconciliation reference.
        reconciliation_reference: Option<String>,
    },
}

impl Acceptance {
    /// Returns whether the complete operation was accepted.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }

    /// Returns whether some but not all work was accepted.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        matches!(self, Self::PartiallyAccepted { .. })
    }

    /// Returns whether the operation was explicitly rejected.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected { .. })
    }

    /// Returns whether acceptance remains unknown.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self, Self::Indeterminate { .. })
    }

    /// Returns whether the provider confirmed acceptance of
    /// at least part of the submitted operation.
    #[must_use]
    pub const fn has_confirmed_acceptance(&self) -> bool {
        matches!(
            self,
            Self::Accepted | Self::PartiallyAccepted { .. }
        )
    }
}

// ============================================================================
// SUBMISSION RECEIPT
// ============================================================================

/// Provider-owned submission acknowledgement identity.
///
/// This is distinct from HardwareOperationIdentity.
///
/// It identifies a provider's receipt or acknowledgement.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SubmissionReceiptId(String);

impl SubmissionReceiptId {
    /// Constructs a provider receipt identity.
    ///
    /// No arbitrary maximum length is imposed.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput for empty values or control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        if value.is_empty()
            || value.chars().any(char::is_control)
        {
            return Err(submission_error(
                ZhalErrorKind::InvalidInput,
                "submission receipt identity is invalid",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the exact provider receipt identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SubmissionReceiptId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_tuple("SubmissionReceiptId")
            .field(&self.0)
            .finish()
    }
}

/// Immutable receipt returned by a hardware provider.
///
/// A receipt reports what the provider knows about submission.
///
/// It does not represent a completed execution result.
///
/// Completion tracking belongs to:
///
/// `crate::zhal::operations::completion`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubmissionReceipt {
    operation: HardwareOperationIdentity,

    receipt_id: Option<SubmissionReceiptId>,

    dispatch_mode: DispatchMode,

    acceptance: Acceptance,
}

impl SubmissionReceipt {
    /// Constructs a provider submission receipt.
    ///
    /// The provider must report the actual acceptance state.
    ///
    /// An indeterminate result must never be represented as
    /// a rejection.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the receipt contains inconsistent
    /// acceptance evidence.
    pub fn new(
        operation: HardwareOperationIdentity,
        receipt_id: Option<SubmissionReceiptId>,
        dispatch_mode: DispatchMode,
        acceptance: Acceptance,
    ) -> ZhalResult<Self> {
        if let Acceptance::PartiallyAccepted {
            evidence_reference,
        } = &acceptance
        {
            if evidence_reference.is_empty() {
                return Err(
                    submission_error(
                        ZhalErrorKind::InvalidInput,
                        "partial acceptance requires an evidence reference",
                    )
                    .with_operation(operation),
                );
            }
        }

        if let Acceptance::Indeterminate {
            reconciliation_reference,
        } = &acceptance
        {
            if reconciliation_reference
                .as_ref()
                .is_some_and(String::is_empty)
            {
                return Err(
                    submission_error(
                        ZhalErrorKind::InvalidInput,
                        "reconciliation reference must not be empty",
                    )
                    .with_operation(operation),
                );
            }
        }

        if let Acceptance::Rejected {
            reason: Some(reason),
        } = &acceptance
        {
            if reason.trim().is_empty() {
                return Err(
                    submission_error(
                        ZhalErrorKind::InvalidInput,
                        "rejection reason must not be empty",
                    )
                    .with_operation(operation),
                );
            }
        }

        Ok(Self {
            operation,
            receipt_id,
            dispatch_mode,
            acceptance,
        })
    }

    /// Returns the operation associated with the receipt.
    #[must_use]
    pub fn operation(&self) -> &HardwareOperationIdentity {
        &self.operation
    }

    /// Returns the optional provider receipt identity.
    #[must_use]
    pub fn receipt_id(&self) -> Option<&SubmissionReceiptId> {
        self.receipt_id.as_ref()
    }

    /// Returns the dispatch mechanism.
    #[must_use]
    pub const fn dispatch_mode(&self) -> DispatchMode {
        self.dispatch_mode
    }

    /// Returns the acceptance decision.
    #[must_use]
    pub fn acceptance(&self) -> &Acceptance {
        &self.acceptance
    }

    /// Returns whether submission was completely accepted.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        self.acceptance.is_accepted()
    }

    /// Returns whether submission was partially accepted.
    #[must_use]
    pub const fn is_partially_accepted(&self) -> bool {
        self.acceptance.is_partial()
    }

    /// Returns whether submission was explicitly rejected.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        self.acceptance.is_rejected()
    }

    /// Returns whether the submission outcome is unknown.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.acceptance.is_indeterminate()
    }
}

// ============================================================================
// PROVIDER SUBMISSION INTERFACE
// ============================================================================

/// Universal hardware operation submission interface.
///
/// Domain-specific providers implement this interface for their
/// own payload types.
///
/// The generic payload parameter deliberately prevents ZHAL from
/// imposing one universal hardware instruction or operation format.
///
/// A provider implementation must:
///
/// - Validate its own payload contract.
/// - Verify that the requested operation is supported.
/// - Observe cancellation and deadlines.
/// - Preserve operation identity.
/// - Respect the declared submission semantics.
/// - Return truthful acceptance information.
/// - Preserve unknown outcomes.
/// - Avoid claiming execution completion from submission.
/// - Avoid silently changing operation semantics.
///
/// # Type parameters
///
/// `P` is the provider's domain-owned payload type.
///
/// # Thread safety
///
/// Implementations must be Send + Sync.
///
/// Payloads must be Send + Sync because submission may be
/// coordinated by a multithreaded runtime.
pub trait HardwareOperationSubmitter<P>: Send + Sync
where
    P: Send + Sync,
{
    /// Submits an operation to the provider.
    ///
    /// The provider borrows the operation context so that
    /// cancellation and deadline information remain available.
    ///
    /// The future must not outlive the provider or context.
    ///
    /// # Important failure semantics
    ///
    /// If submission definitely did not reach the provider,
    /// an appropriate ZhalError may be returned.
    ///
    /// If submission may have reached the provider but the
    /// acknowledgement was lost, the provider must return
    /// a receipt with Acceptance::Indeterminate.
    ///
    /// It must not incorrectly report a definite rejection.
    ///
    /// A returned error after possible dispatch must not erase
    /// the uncertainty about physical execution.
    fn submit<'a>(
        &'a self,
        request: SubmissionRequest<P>,
        context: &'a ZhalContext,
    ) -> SubmissionFuture<'a>;
}

// ============================================================================
// SUBMISSION VALIDATION AND COORDINATION
// ============================================================================

/// Validates and submits an operation through a provider.
///
/// This function performs only submission-boundary validation.
///
/// It does not:
///
/// - Discover providers.
/// - Select devices.
/// - Allocate resources.
/// - Reserve hardware.
/// - Schedule execution.
/// - Retry failed submissions.
/// - Perform distributed routing.
///
/// Those responsibilities belong to their existing authorities.
///
/// # Errors
///
/// Returns a ZhalError when:
///
/// - The operation context is cancelled.
/// - The operation deadline has expired.
/// - The context provider does not match the operation provider.
/// - The provider returns an inconsistent receipt.
///
/// A provider must represent uncertain post-dispatch outcomes
/// through an indeterminate receipt rather than a definite failure.
pub async fn submit_operation<P, S>(
    submitter: &S,
    request: SubmissionRequest<P>,
    context: &ZhalContext,
) -> ZhalResult<SubmissionReceipt>
where
    P: Send + Sync,
    S: HardwareOperationSubmitter<P> + ?Sized,
{
    context.check_active().map_err(|error| {
        submission_error(
            if context.operation_context().is_cancelled() {
                ZhalErrorKind::Cancelled
            } else {
                ZhalErrorKind::OperationTimedOut
            },
            error.to_string(),
        )
        .with_operation(request.operation().clone())
    })?;

    let operation = request.operation().clone();

    if let Some(provider_id) = context.provider_id() {
        if provider_id != operation.provider_id() {
            return Err(
                submission_error(
                    ZhalErrorKind::InvalidInput,
                    "submission provider does not match operation context",
                )
                .with_operation(operation),
            );
        }
    }

    validate_submission_contract(&request)?;

    let receipt = submitter
        .submit(request, context)
        .await?;

    validate_receipt(&operation, &receipt)?;

    Ok(receipt)
}

/// Validates that the operation contract permits the declared
/// submission semantics.
///
/// ProviderDefined remains valid because specialized provider
/// contracts own those guarantees.
fn validate_submission_contract<P>(
    request: &SubmissionRequest<P>,
) -> ZhalResult<()> {
    let semantics = request.contract().lifecycle().submission();

    match semantics {
        SubmissionSemantics::AtomicAcceptance
        | SubmissionSemantics::PartialAcceptance
        | SubmissionSemantics::IndeterminateAcceptance
        | SubmissionSemantics::ProviderDefined => Ok(()),
    }
}

/// Validates the provider's acknowledgement against the original
/// operation identity.
///
/// A provider must never acknowledge another operation as though
/// it were the submitted operation.
fn validate_receipt(
    expected: &HardwareOperationIdentity,
    receipt: &SubmissionReceipt,
) -> ZhalResult<()> {
    if receipt.operation() != expected {
        return Err(
            submission_error(
                ZhalErrorKind::ContractViolation,
                "provider returned a receipt for a different operation",
            )
            .with_operation(expected.clone()),
        );
    }

    Ok(())
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Constructs a structured submission error.
fn submission_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .with_component(COMPONENT)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_modes_have_stable_names() {
        assert_eq!(
            DispatchMode::Synchronous.as_str(),
            "synchronous"
        );

        assert_eq!(
            DispatchMode::Asynchronous.as_str(),
            "asynchronous"
        );

        assert_eq!(
            DispatchMode::Remote.as_str(),
            "remote"
        );
    }

    #[test]
    fn acceptance_states_remain_distinct() {
        assert!(Acceptance::Accepted.is_accepted());

        assert!(
            Acceptance::PartiallyAccepted {
                evidence_reference: "evidence".to_owned(),
            }
            .is_partial()
        );

        assert!(
            Acceptance::Rejected {
                reason: None,
            }
            .is_rejected()
        );

        assert!(
            Acceptance::Indeterminate {
                reconciliation_reference: None,
            }
            .is_indeterminate()
        );
    }

    #[test]
    fn partial_acceptance_requires_evidence() {
        let operation = test_operation();

        let result = SubmissionReceipt::new(
            operation,
            None,
            DispatchMode::Remote,
            Acceptance::PartiallyAccepted {
                evidence_reference: String::new(),
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn indeterminate_acceptance_is_not_rejection() {
        let acceptance = Acceptance::Indeterminate {
            reconciliation_reference: Some(
                "remote-operation-reference".to_owned(),
            ),
        };

        assert!(acceptance.is_indeterminate());
        assert!(!acceptance.is_rejected());
        assert!(!acceptance.is_accepted());
    }

    #[test]
    fn empty_idempotency_keys_are_rejected() {
        assert!(IdempotencyKey::new("").is_err());
    }

    #[test]
    fn idempotency_key_debug_is_redacted() {
        let key = IdempotencyKey::new(
            "sensitive-submission-key",
        )
        .expect("valid key");

        let debug = format!("{key:?}");

        assert!(!debug.contains("sensitive-submission-key"));
        assert!(debug.contains("REDACTED"));
    }

    #[test]
    fn receipt_identity_is_preserved() {
        let operation = test_operation();

        let receipt = SubmissionReceipt::new(
            operation.clone(),
            Some(
                SubmissionReceiptId::new("provider-receipt")
                    .expect("valid receipt identity"),
            ),
            DispatchMode::Asynchronous,
            Acceptance::Accepted,
        )
        .expect("valid receipt");

        assert_eq!(receipt.operation(), &operation);

        assert_eq!(
            receipt.receipt_id().map(
                SubmissionReceiptId::as_str
            ),
            Some("provider-receipt")
        );

        assert!(receipt.is_accepted());
    }

    fn test_operation() -> HardwareOperationIdentity {
        use crate::zhal::identity::HardwareOperationId;
        use crate::stdlib::provider::ProviderId;

        HardwareOperationIdentity::new(
            ProviderId::new("test.provider")
                .expect("valid provider"),
            HardwareOperationId::new("operation.instance")
                .expect("valid operation"),
        )
    }
}
