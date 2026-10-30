
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Distributed Transport Errors
//!
//! Path: `src/distributed/transport/errors.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the transport-specific error contract for distributed
//! communication throughout Zamani.
//!
//! This module normalizes failures originating from different transport
//! implementations without introducing a dependency on a particular
//! networking protocol, operating system, hardware platform, or provider.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - TransportError.
//! - TransportErrorKind.
//! - TransportErrorContext.
//! - TransportFailurePhase.
//! - TransportFailureClassification.
//! - Transport error classification.
//! - Conversion into the canonical DistributedError.
//! - Underlying transport error preservation.
//!
//! This module does not own:
//!
//! - Physical network connections.
//! - Network protocols.
//! - Message encoding or decoding implementations.
//! - Peer discovery.
//! - Routing algorithms.
//! - Authentication implementations.
//! - Authorization policies.
//! - Retry execution.
//! - Execution scheduling.
//! - Distributed program semantics.
//!
//! ## Canonical error authority
//!
//! The canonical distributed error types remain owned by:
//!
//! `crate::distributed::error`
//!
//! This module MUST NOT create a competing distributed error hierarchy.
//!
//! ## Delivery semantics
//!
//! A transport failure does not necessarily mean that an operation
//! failed to execute.
//!
//! The following distinctions are mandatory:
//!
//! - Not sent.
//! - Rejected before execution.
//! - Possibly delivered.
//! - Response lost.
//! - Delivery or execution outcome unknown.
//!
//! An uncertain remote outcome MUST NOT automatically authorize retries.
//!
//! ## POCO-REAF and scalability
//!
//! This module imposes no universal maximum on:
//!
//! - Participants.
//! - Connections.
//! - Messages.
//! - Workers.
//! - Nodes.
//! - Channels.
//! - Transport implementations.
//! - Protocol implementations.
//!
//! Error classification is independent of physical system size.
//!
//! Resource budgets and implementation-specific limits belong to
//! configuration and transport providers.
//!
//! Literal infinite capacity is not promised. Rust, memory, address spaces,
//! operating systems, and physical communication resources remain finite.
//!
//! ## Integration
//!
//! - `src/distributed/error.rs`: canonical error classification.
//! - `src/distributed/transport/mod.rs`: transport module exports.
//! - `src/distributed/transport/traits.rs`: transport provider contracts.
//! - `src/distributed/transport/messages.rs`: protocol message failures.
//! - `src/distributed/transport/codec.rs`: encoding and decoding failures.
//! - `src/distributed/execution/`: execution failure propagation.
//! - `src/distributed/resilience/`: retry and reconciliation decisions.
//! - `src/distributed/security/`: authentication and authorization.
//! - `src/runtime/distributed.rs`: runtime error propagation.
//!
//! No networking implementation is required to compile this module.

use core::fmt;
use std::error::Error;

use crate::distributed::error::{
    DeliveryState,
    DistributedError,
    DistributedErrorKind,
    DistributedResult,
    RetryAdvice,
};

/// Boxed source error returned by a transport implementation.
///
/// The original error is preserved as a source rather than reduced
/// immediately to a formatted string.
pub type TransportErrorSource =
    Box<dyn Error + Send + Sync + 'static>;

/// Result type for transport operations.
pub type TransportResult<T> = Result<T, TransportError>;

/// Classification of a transport failure.
///
/// This enum is intentionally non-exhaustive. New transport technologies
/// and communication mechanisms must be able to introduce new failure
/// categories without requiring consumers to assume a permanently closed
/// transport vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransportErrorKind {
    /// An unspecified transport failure.
    Unknown,

    /// A connection could not be established.
    ConnectionFailed,

    /// A connection was refused by the remote endpoint.
    ConnectionRefused,

    /// A connection was interrupted.
    ConnectionInterrupted,

    /// A connection was closed unexpectedly.
    ConnectionClosed,

    /// A peer or communication endpoint is unavailable.
    PeerUnavailable,

    /// A communication operation exceeded its deadline.
    Timeout,

    /// A communication operation was cancelled.
    Cancelled,

    /// The transport is not initialized or is not ready.
    NotInitialized,

    /// An operation was attempted on a closed transport.
    Closed,

    /// A transport or protocol operation is unsupported.
    Unsupported,

    /// The peer rejected a communication request.
    PeerRejected,

    /// A protocol operation failed.
    ProtocolFailure,

    /// The local and remote protocol versions are incompatible.
    ProtocolIncompatible,

    /// Message encoding failed.
    SerializationFailure,

    /// Message decoding failed.
    DeserializationFailure,

    /// The received message violates its contract.
    InvalidMessage,

    /// The peer could not be authenticated.
    AuthenticationFailure,

    /// An authenticated peer was not authorized.
    AuthorizationFailure,

    /// A communication resource or configured budget is exhausted.
    ResourceExhausted,

    /// The transport cannot accept additional work temporarily.
    Backpressure,

    /// An integrity check failed.
    IntegrityFailure,

    /// A cryptographic operation failed.
    CryptographicFailure,

    /// A transport-specific configuration is invalid.
    InvalidConfiguration,

    /// The operation received invalid input.
    InvalidInput,

    /// A provider-specific implementation failed.
    ProviderFailure,
}

impl TransportErrorKind {
    /// Returns whether this category can potentially represent a transient
    /// communication failure.
    ///
    /// This is descriptive classification, not retry authorization.
    #[must_use]
    pub const fn may_be_transient(self) -> bool {
        matches!(
            self,
            Self::ConnectionFailed
                | Self::ConnectionInterrupted
                | Self::ConnectionClosed
                | Self::PeerUnavailable
                | Self::Timeout
                | Self::Backpressure
        )
    }

    /// Returns whether this failure represents a security rejection.
    #[must_use]
    pub const fn is_security_failure(self) -> bool {
        matches!(
            self,
            Self::AuthenticationFailure
                | Self::AuthorizationFailure
                | Self::IntegrityFailure
                | Self::CryptographicFailure
        )
    }

    /// Returns whether this failure represents a message or protocol
    /// contract violation.
    #[must_use]
    pub const fn is_protocol_failure(self) -> bool {
        matches!(
            self,
            Self::ProtocolFailure
                | Self::ProtocolIncompatible
                | Self::SerializationFailure
                | Self::DeserializationFailure
                | Self::InvalidMessage
        )
    }

    /// Returns a stable machine-readable category identifier.
    ///
    /// These identifiers are diagnostic classification values, not
    /// distributed wire-protocol identifiers.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "transport.unknown",
            Self::ConnectionFailed => "transport.connection_failed",
            Self::ConnectionRefused => "transport.connection_refused",
            Self::ConnectionInterrupted => "transport.connection_interrupted",
            Self::ConnectionClosed => "transport.connection_closed",
            Self::PeerUnavailable => "transport.peer_unavailable",
            Self::Timeout => "transport.timeout",
            Self::Cancelled => "transport.cancelled",
            Self::NotInitialized => "transport.not_initialized",
            Self::Closed => "transport.closed",
            Self::Unsupported => "transport.unsupported",
            Self::PeerRejected => "transport.peer_rejected",
            Self::ProtocolFailure => "transport.protocol_failure",
            Self::ProtocolIncompatible => "transport.protocol_incompatible",
            Self::SerializationFailure => "transport.serialization_failure",
            Self::DeserializationFailure => "transport.deserialization_failure",
            Self::InvalidMessage => "transport.invalid_message",
            Self::AuthenticationFailure => "transport.authentication_failure",
            Self::AuthorizationFailure => "transport.authorization_failure",
            Self::ResourceExhausted => "transport.resource_exhausted",
            Self::Backpressure => "transport.backpressure",
            Self::IntegrityFailure => "transport.integrity_failure",
            Self::CryptographicFailure => "transport.cryptographic_failure",
            Self::InvalidConfiguration => "transport.invalid_configuration",
            Self::InvalidInput => "transport.invalid_input",
            Self::ProviderFailure => "transport.provider_failure",
        }
    }
}

/// Stage at which a transport failure occurred.
///
/// This provides operational context without assuming a particular
/// networking protocol or transport implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransportFailurePhase {
    /// Transport initialization.
    Initialization,

    /// Peer discovery or endpoint resolution.
    Discovery,

    /// Connection establishment.
    Connection,

    /// Peer authentication.
    Authentication,

    /// Peer authorization.
    Authorization,

    /// Message encoding.
    Encoding,

    /// Message decoding.
    Decoding,

    /// Message transmission.
    Sending,

    /// Message reception.
    Receiving,

    /// Request-response correlation.
    Correlation,

    /// Protocol negotiation.
    Negotiation,

    /// Integrity verification.
    IntegrityVerification,

    /// Graceful transport shutdown.
    Shutdown,

    /// An implementation-specific stage.
    Provider,
}

/// Context identifying the operation associated with a transport failure.
///
/// Context is descriptive. It does not establish a physical network
/// identity or expose a transport implementation's internal state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransportErrorContext {
    operation: Option<String>,
    peer: Option<String>,
    phase: Option<TransportFailurePhase>,
    correlation: Option<String>,
}

impl TransportErrorContext {
    /// Creates empty transport error context.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operation: None,
            peer: None,
            phase: None,
            correlation: None,
        }
    }

    /// Associates the error with a logical operation.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Associates the error with a peer identifier.
    ///
    /// Callers must not supply passwords, credentials, private keys,
    /// access tokens, or other secrets.
    #[must_use]
    pub fn with_peer(
        mut self,
        peer: impl Into<String>,
    ) -> Self {
        self.peer = Some(peer.into());
        self
    }

    /// Associates the error with a transport phase.
    #[must_use]
    pub fn with_phase(
        mut self,
        phase: TransportFailurePhase,
    ) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Associates the error with a correlation identifier.
    #[must_use]
    pub fn with_correlation(
        mut self,
        correlation: impl Into<String>,
    ) -> Self {
        self.correlation = Some(correlation.into());
        self
    }

    /// Returns the associated operation.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Returns the associated peer.
    #[must_use]
    pub fn peer(&self) -> Option<&str> {
        self.peer.as_deref()
    }

    /// Returns the transport phase.
    #[must_use]
    pub const fn phase(&self) -> Option<TransportFailurePhase> {
        self.phase
    }

    /// Returns the correlation identifier.
    #[must_use]
    pub fn correlation(&self) -> Option<&str> {
        self.correlation.as_deref()
    }
}

/// Structured transport failure classification.
///
/// This is an informational snapshot. It does not perform retries,
/// change execution state, or establish delivery guarantees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransportFailureClassification {
    /// Transport error category.
    pub kind: TransportErrorKind,

    /// Delivery knowledge.
    pub delivery: DeliveryState,

    /// Whether the failure may be transient.
    pub potentially_transient: bool,

    /// Conservative retry guidance.
    pub retry_advice: RetryAdvice,
}

/// A normalized transport error.
///
/// The original provider error is retained through the standard Rust
/// error source chain.
///
/// This type deliberately does not implement `Clone` or `PartialEq`
/// because its source may own non-cloneable external error objects.
#[derive(Debug)]
pub struct TransportError {
    kind: TransportErrorKind,
    context: TransportErrorContext,
    delivery: DeliveryState,
    message: String,
    source: Option<TransportErrorSource>,
}

impl TransportError {
    /// Creates a transport error.
    ///
    /// `delivery` MUST reflect the best available knowledge from the
    /// transport provider.
    ///
    /// Use `DeliveryState::Unknown` when the provider cannot establish
    /// what happened.
    #[must_use]
    pub fn new(
        kind: TransportErrorKind,
        delivery: DeliveryState,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            context: TransportErrorContext::new(),
            delivery,
            message: message.into(),
            source: None,
        }
    }

    /// Creates a transport error with contextual information.
    #[must_use]
    pub fn with_context(
        kind: TransportErrorKind,
        delivery: DeliveryState,
        context: TransportErrorContext,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            context,
            delivery,
            message: message.into(),
            source: None,
        }
    }

    /// Creates a transport error while preserving its original cause.
    #[must_use]
    pub fn with_source<E>(
        kind: TransportErrorKind,
        delivery: DeliveryState,
        context: TransportErrorContext,
        message: impl Into<String>,
        source: E,
    ) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            kind,
            context,
            delivery,
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Returns the transport failure category.
    #[must_use]
    pub const fn kind(&self) -> TransportErrorKind {
        self.kind
    }

    /// Returns delivery knowledge.
    #[must_use]
    pub const fn delivery(&self) -> DeliveryState {
        self.delivery
    }

    /// Returns the contextual information.
    #[must_use]
    pub const fn context(&self) -> &TransportErrorContext {
        &self.context
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the original provider error, if available.
    #[must_use]
    pub fn underlying_source(
        &self,
    ) -> Option<&(dyn Error + Send + Sync + 'static)> {
        self.source.as_deref()
    }

    /// Returns a structured classification of this failure.
    #[must_use]
    pub const fn classification(
        &self,
    ) -> TransportFailureClassification {
        TransportFailureClassification {
            kind: self.kind,
            delivery: self.delivery,
            potentially_transient: self.kind.may_be_transient(),
            retry_advice: self.retry_advice(),
        }
    }

    /// Returns conservative retry guidance.
    ///
    /// This method never authorizes automatic retries.
    ///
    /// An uncertain execution outcome requires reconciliation before
    /// another attempt can be considered.
    #[must_use]
    pub const fn retry_advice(&self) -> RetryAdvice {
        match self.delivery {
            DeliveryState::MayHaveExecuted
            | DeliveryState::ResponseLost
            | DeliveryState::Unknown => {
                RetryAdvice::ReconcileOutcomeFirst
            }

            DeliveryState::NotSent
            | DeliveryState::RejectedBeforeExecution => {
                match self.kind {
                    TransportErrorKind::ConnectionFailed
                    | TransportErrorKind::ConnectionInterrupted
                    | TransportErrorKind::ConnectionClosed
                    | TransportErrorKind::PeerUnavailable
                    | TransportErrorKind::Timeout
                    | TransportErrorKind::Backpressure => {
                        RetryAdvice::PossiblyAfterBackoff
                    }

                    TransportErrorKind::ResourceExhausted
                    | TransportErrorKind::ProviderFailure => {
                        RetryAdvice::OnlyIfIdempotent
                    }

                    _ => RetryAdvice::Never,
                }
            }
        }
    }

    /// Converts this error into the canonical distributed error while
    /// preserving the original transport error as its source.
    ///
    /// The resulting canonical classification is ExternalFailure because
    /// the existing DistributedError model cannot attach arbitrary source
    /// errors to every specialized variant.
    ///
    /// Use `into_classified_distributed_error` when retaining the specific
    /// distributed error category is more important than preserving the
    /// provider error object.
    #[must_use]
    pub fn into_distributed_error(self) -> DistributedError {
        DistributedError::external(
            "distributed.transport",
            self.to_string(),
            self,
        )
    }

    /// Converts this error into its most specific existing canonical
    /// distributed error category.
    ///
    /// This conversion retains the explanatory source text but cannot
    /// preserve the original boxed source object in specialized variants
    /// because the canonical error definitions do not provide source
    /// fields for those variants.
    ///
    /// Use `into_distributed_error` when preservation of the complete
    /// underlying source chain is required.
    #[must_use]
    pub fn into_classified_distributed_error(
        self,
    ) -> DistributedError {
        let operation = self
            .context
            .operation
            .clone()
            .unwrap_or_else(|| "transport".to_owned());

        let peer = self.context.peer.clone();
        let message = self.to_string();

        match self.kind {
            TransportErrorKind::Timeout => {
                DistributedError::DeadlineExceeded {
                    operation,
                    delivery: self.delivery,
                }
            }

            TransportErrorKind::Cancelled => {
                DistributedError::Cancelled {
                    operation: Some(operation),
                    reason: message,
                }
            }

            TransportErrorKind::AuthenticationFailure => {
                DistributedError::AuthenticationFailed {
                    peer,
                    message,
                }
            }

            TransportErrorKind::AuthorizationFailure => {
                DistributedError::AuthorizationDenied {
                    principal: peer,
                    operation,
                }
            }

            TransportErrorKind::ProtocolIncompatible => {
                DistributedError::ProtocolIncompatible {
                    subject: operation,
                    expected: "compatible protocol".to_owned(),
                    actual: message,
                }
            }

            TransportErrorKind::SerializationFailure
            | TransportErrorKind::DeserializationFailure => {
                DistributedError::SerializationFailed {
                    operation,
                    message,
                }
            }

            TransportErrorKind::InvalidMessage => {
                DistributedError::InvalidMessage {
                    subject: operation,
                    message,
                }
            }

            TransportErrorKind::InvalidInput => {
                DistributedError::invalid_input(message)
            }

            TransportErrorKind::InvalidConfiguration => {
                DistributedError::invalid_configuration(
                    operation,
                    message,
                )
            }

            TransportErrorKind::Unsupported => {
                DistributedError::UnsupportedOperation {
                    operation,
                    provider: peer,
                    message,
                }
            }

            TransportErrorKind::ResourceExhausted => {
                DistributedError::resource_unsatisfied(
                    "transport.resources",
                    None,
                    None,
                    None,
                    false,
                )
            }

            TransportErrorKind::PeerUnavailable
            | TransportErrorKind::ConnectionFailed
            | TransportErrorKind::ConnectionRefused
            | TransportErrorKind::ConnectionInterrupted
            | TransportErrorKind::ConnectionClosed
            | TransportErrorKind::NotInitialized
            | TransportErrorKind::Closed
            | TransportErrorKind::Backpressure
            | TransportErrorKind::Unknown
            | TransportErrorKind::PeerRejected
            | TransportErrorKind::ProtocolFailure
            | TransportErrorKind::IntegrityFailure
            | TransportErrorKind::CryptographicFailure
            | TransportErrorKind::ProviderFailure => {
                match self.delivery {
                    DeliveryState::MayHaveExecuted
                    | DeliveryState::ResponseLost
                    | DeliveryState::Unknown => {
                        DistributedError::outcome_unknown(
                            self.context.correlation.clone(),
                            self.delivery,
                            message,
                        )
                    }

                    DeliveryState::NotSent
                    | DeliveryState::RejectedBeforeExecution => {
                        DistributedError::transport_failed(
                            operation,
                            self.delivery,
                            message,
                        )
                    }
                }
            }
        }
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.kind.as_str(),
            self.message
        )?;

        if let Some(operation) = self.context.operation() {
            write!(f, "; operation={operation}")?;
        }

        if let Some(peer) = self.context.peer() {
            write!(f, "; peer={peer}")?;
        }

        if let Some(phase) = self.context.phase() {
            write!(f, "; phase={phase:?}")?;
        }

        if let Some(correlation) = self.context.correlation() {
            write!(f, "; correlation={correlation}")?;
        }

        write!(f, "; delivery={:?}", self.delivery)
    }
}

impl Error for TransportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}

/// Converts a transport result into the canonical distributed result.
///
/// This conversion preserves the original transport error as a source.
pub fn normalize_transport_result<T>(
    result: TransportResult<T>,
) -> DistributedResult<T> {
    result.map_err(TransportError::into_distributed_error)
}

/// Converts a transport result while retaining its most specific
/// available distributed error classification.
///
/// See `TransportError::into_classified_distributed_error` for
/// source-preservation limitations.
pub fn classify_transport_result<T>(
    result: TransportResult<T>,
) -> DistributedResult<T> {
    result.map_err(TransportError::into_classified_distributed_error)
}

/// Returns whether a canonical distributed error represents a transport
/// failure.
///
/// Context-wrapped errors are handled through the canonical `kind()` API.
#[must_use]
pub fn is_transport_error(error: &DistributedError) -> bool {
    matches!(
        error.kind(),
        DistributedErrorKind::TransportFailed
            | DistributedErrorKind::SerializationFailed
            | DistributedErrorKind::ProtocolIncompatible
            | DistributedErrorKind::OperationOutcomeUnknown
            | DistributedErrorKind::DeadlineExceeded
            | DistributedErrorKind::Unavailable
            | DistributedErrorKind::AuthenticationFailed
            | DistributedErrorKind::AuthorizationDenied
            | DistributedErrorKind::InvalidMessage
            | DistributedErrorKind::ExternalFailure
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;
    use std::io;

    fn context() -> TransportErrorContext {
        TransportErrorContext::new()
            .with_operation("request")
            .with_peer("logical-participant")
            .with_phase(TransportFailurePhase::Sending)
            .with_correlation("correlation-1")
    }

    #[test]
    fn error_preserves_transport_context() {
        let error = TransportError::with_context(
            TransportErrorKind::ConnectionFailed,
            DeliveryState::NotSent,
            context(),
            "connection unavailable",
        );

        assert_eq!(error.kind(), TransportErrorKind::ConnectionFailed);
        assert_eq!(error.delivery(), DeliveryState::NotSent);
        assert_eq!(error.context().operation(), Some("request"));
        assert_eq!(
            error.context().peer(),
            Some("logical-participant")
        );
        assert_eq!(
            error.context().phase(),
            Some(TransportFailurePhase::Sending)
        );
        assert_eq!(
            error.context().correlation(),
            Some("correlation-1")
        );
    }

    #[test]
    fn source_error_chain_is_preserved() {
        let original = io::Error::other("connection reset");

        let error = TransportError::with_source(
            TransportErrorKind::ConnectionInterrupted,
            DeliveryState::NotSent,
            context(),
            "transport interrupted",
            original,
        );

        assert!(StdError::source(&error).is_some());

        let normalized = error.into_distributed_error();

        assert_eq!(
            normalized.kind(),
            DistributedErrorKind::ExternalFailure
        );

        assert!(StdError::source(&normalized).is_some());
        assert!(normalized.to_string().contains("connection reset"));
    }

    #[test]
    fn uncertain_delivery_requires_reconciliation() {
        let error = TransportError::new(
            TransportErrorKind::Timeout,
            DeliveryState::ResponseLost,
            "response timed out",
        );

        assert_eq!(
            error.retry_advice(),
            RetryAdvice::ReconcileOutcomeFirst
        );
    }

    #[test]
    fn definitely_unsent_transient_failure_may_be_retried_by_policy() {
        let error = TransportError::new(
            TransportErrorKind::ConnectionFailed,
            DeliveryState::NotSent,
            "connection refused",
        );

        assert_eq!(
            error.retry_advice(),
            RetryAdvice::PossiblyAfterBackoff
        );
    }

    #[test]
    fn authentication_failures_are_not_retryable() {
        let error = TransportError::new(
            TransportErrorKind::AuthenticationFailure,
            DeliveryState::RejectedBeforeExecution,
            "peer identity rejected",
        );

        assert_eq!(error.retry_advice(), RetryAdvice::Never);
    }

    #[test]
    fn authorization_failures_are_not_retryable() {
        let error = TransportError::new(
            TransportErrorKind::AuthorizationFailure,
            DeliveryState::RejectedBeforeExecution,
            "permission denied",
        );

        assert_eq!(error.retry_advice(), RetryAdvice::Never);
    }

    #[test]
    fn message_failures_are_not_transient() {
        let error = TransportError::new(
            TransportErrorKind::DeserializationFailure,
            DeliveryState::RejectedBeforeExecution,
            "invalid message encoding",
        );

        assert!(!error.kind().may_be_transient());
        assert!(error.kind().is_protocol_failure());
        assert_eq!(error.retry_advice(), RetryAdvice::Never);
    }

    #[test]
    fn classification_preserves_delivery_knowledge() {
        let error = TransportError::new(
            TransportErrorKind::Timeout,
            DeliveryState::MayHaveExecuted,
            "request timed out",
        );

        let classification = error.classification();

        assert_eq!(classification.kind, TransportErrorKind::Timeout);
        assert_eq!(
            classification.delivery,
            DeliveryState::MayHaveExecuted
        );
        assert_eq!(
            classification.retry_advice,
            RetryAdvice::ReconcileOutcomeFirst
        );
    }

    #[test]
    fn timeout_maps_to_canonical_deadline_error() {
        let error = TransportError::with_context(
            TransportErrorKind::Timeout,
            DeliveryState::NotSent,
            context(),
            "operation timed out",
        );

        let distributed = error.into_classified_distributed_error();

        assert_eq!(
            distributed.kind(),
            DistributedErrorKind::DeadlineExceeded
        );
    }

    #[test]
    fn unknown_remote_outcome_maps_to_reconciliation_error() {
        let error = TransportError::new(
            TransportErrorKind::ConnectionInterrupted,
            DeliveryState::ResponseLost,
            "connection interrupted after sending",
        );

        let distributed = error.into_classified_distributed_error();

        assert_eq!(
            distributed.kind(),
            DistributedErrorKind::OperationOutcomeUnknown
        );

        assert_eq!(
            distributed.retry_advice(),
            RetryAdvice::ReconcileOutcomeFirst
        );
    }

    #[test]
    fn authentication_maps_to_canonical_security_error() {
        let error = TransportError::new(
            TransportErrorKind::AuthenticationFailure,
            DeliveryState::RejectedBeforeExecution,
            "peer verification failed",
        );

        let distributed = error.into_classified_distributed_error();

        assert_eq!(
            distributed.kind(),
            DistributedErrorKind::AuthenticationFailed
        );

        assert_eq!(
            distributed.retry_advice(),
            RetryAdvice::Never
        );
    }

    #[test]
    fn provider_specific_categories_remain_supported() {
        let error = TransportError::new(
            TransportErrorKind::ProviderFailure,
            DeliveryState::NotSent,
            "provider-specific failure",
        );

        assert_eq!(
            error.kind().as_str(),
            "transport.provider_failure"
        );
    }

    #[test]
    fn error_display_contains_actionable_context() {
        let error = TransportError::with_context(
            TransportErrorKind::ConnectionInterrupted,
            DeliveryState::ResponseLost,
            context(),
            "connection interrupted",
        );

        let message = error.to_string();

        assert!(message.contains("transport.connection_interrupted"));
        assert!(message.contains("request"));
        assert!(message.contains("logical-participant"));
        assert!(message.contains("correlation-1"));
        assert!(message.contains("ResponseLost"));
    }

    #[test]
    fn result_normalization_preserves_success() {
        let result: TransportResult<u64> = Ok(42);

        assert_eq!(normalize_transport_result(result).unwrap(), 42);
    }

    #[test]
    fn result_normalization_propagates_failure() {
        let result: TransportResult<u64> = Err(
            TransportError::new(
                TransportErrorKind::ConnectionFailed,
                DeliveryState::NotSent,
                "connection unavailable",
            ),
        );

        let normalized = normalize_transport_result(result);

        assert!(normalized.is_err());

        assert_eq!(
            normalized.unwrap_err().kind(),
            DistributedErrorKind::ExternalFailure
        );
    }

    #[test]
    fn classified_result_preserves_specialized_failure_category() {
        let result: TransportResult<u64> = Err(
            TransportError::new(
                TransportErrorKind::AuthenticationFailure,
                DeliveryState::RejectedBeforeExecution,
                "authentication failed",
            ),
        );

        let normalized = classify_transport_result(result);

        assert_eq!(
            normalized.unwrap_err().kind(),
            DistributedErrorKind::AuthenticationFailed
        );
    }

    #[test]
    fn security_categories_are_recognized() {
        assert!(
            TransportErrorKind::AuthenticationFailure
                .is_security_failure()
        );

        assert!(
            TransportErrorKind::AuthorizationFailure
                .is_security_failure()
        );

        assert!(
            TransportErrorKind::IntegrityFailure
                .is_security_failure()
        );
    }
}
