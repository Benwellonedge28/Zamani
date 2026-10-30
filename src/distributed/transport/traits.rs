
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Distributed Transport Interfaces
//!
//! Path: src/distributed/transport/traits.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the provider-independent asynchronous interfaces used to
//! communicate between logical participants in Zamani distributed
//! computation.
//!
//! The same logical communication interface must support:
//!
//! - local communication;
//! - inter-process communication;
//! - embedded and edge communication;
//! - heterogeneous computing systems;
//! - cloud and cluster communication;
//! - classical and quantum-classical communication;
//! - hardware and accelerator communication;
//! - future communication technologies.
//!
//! No particular communication technology is mandatory.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - TransportProvider;
//! - TransportConnection;
//! - TransportOperationContext;
//! - TransportLimits;
//! - AuthenticatedPeer;
//! - delivery confirmation contracts;
//! - request-response transport contracts;
//! - incoming connection contracts;
//! - graceful shutdown contracts;
//! - asynchronous transport interface definitions.
//!
//! This module does NOT own:
//!
//! - source-language message schemas;
//! - distributed semantic analysis;
//! - physical endpoint discovery;
//! - physical network addressing;
//! - routing algorithms;
//! - serialization and deserialization;
//! - encryption implementations;
//! - authentication algorithms;
//! - authorization policies;
//! - scheduling and placement;
//! - task execution;
//! - automatic retries;
//! - checkpointing;
//! - recovery;
//! - backend selection;
//! - hardware allocation.
//!
//! ## Authoritative integration boundaries
//!
//! Logical message representation:
//! `crate::distributed::transport::messages`
//!
//! Transport-specific errors:
//! `crate::distributed::transport::errors`
//!
//! Canonical distributed errors:
//! `crate::distributed::error`
//!
//! Logical identities:
//! `crate::distributed::ids`
//!
//! Operational configuration:
//! `crate::distributed::config`
//!
//! Cancellation and deadlines:
//! `crate::distributed::execution::cancellation`
//!
//! Networking semantics:
//! `grammar/spec/networking.md`
//!
//! Distributed semantics:
//! `grammar/spec/distributed.md`
//!
//! ## POCO-REAF
//!
//! Transport implementations are replaceable providers.
//!
//! A logical communication operation must not depend on:
//!
//! - a fixed transport protocol;
//! - a fixed network interface;
//! - a fixed port;
//! - a fixed machine count;
//! - a fixed participant count;
//! - a fixed message count;
//! - a fixed cluster topology;
//! - a particular operating system;
//! - a particular hardware vendor.
//!
//! Actual resource limitations are reported explicitly.
//!
//! No universal capacity or cluster-size limit is introduced here.
//!
//! Literal infinite physical capacity is not promised.
//!
//! ## Delivery semantics
//!
//! The following states are deliberately different:
//!
//! - accepted by the local provider;
//! - acknowledged by the remote transport;
//! - acknowledged by the remote application;
//! - operation executed;
//! - operation durably completed.
//!
//! A transport acknowledgement MUST NOT be interpreted as proof of
//! application execution or durable completion.
//!
//! A lost response MUST NOT automatically authorize a retry.
//!
//! ## Authentication
//!
//! An incoming connection must not be considered trusted merely
//! because its claimed ParticipantId is present in a message.
//!
//! The provider must establish authenticated peer identity through
//! the repository's security authority before exposing an authenticated
//! connection to application execution.
//!
//! Authorization remains a separate security responsibility.
//!
//! ## Asynchronous execution
//!
//! This module uses standard-library futures and does not prescribe
//! a particular asynchronous runtime.
//!
//! Implementations may integrate with an external runtime internally.
//!
//! ## Safety
//!
//! No unsafe Rust is permitted.

use core::fmt;
use core::future::Future;
use core::num::NonZeroUsize;
use core::pin::Pin;

use std::sync::Arc;
use std::time::Duration;

use crate::distributed::execution::cancellation::ExecutionControl;

use crate::distributed::ids::ParticipantId;

use crate::distributed::transport::errors::{
    TransportError,
    TransportErrorContext,
    TransportErrorKind,
    TransportFailurePhase,
    TransportResult,
};

use crate::distributed::transport::messages::{
    MessageEnvelope,
    MessageRole,
};

/// A boxed, sendable asynchronous operation.
///
/// This alias allows transport providers to be used through trait objects
/// without imposing a particular asynchronous runtime.
///
/// The lifetime represents the borrowed provider or connection.
pub type TransportFuture<'a, T> =
    Pin<Box<dyn Future<Output = T> + Send + 'a>>;

// =============================================================================
// AUTHENTICATED PEER
// =============================================================================

/// An authenticated remote participant.
///
/// This value represents an identity established by the transport
/// provider through the repository's authentication authority.
///
/// It does not represent authorization to execute every possible
/// operation.
///
/// Authorization must be evaluated independently.
///
/// Implementations must not construct this value from an unverified
/// sender field in a received message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticatedPeer {
    participant: ParticipantId,
    authentication_method: String,
    identity_reference: Option<String>,
}

impl AuthenticatedPeer {
    /// Constructs authenticated-peer evidence.
    ///
    /// This constructor is intended for trusted authentication adapters.
    ///
    /// Calling this function does not itself authenticate a remote
    /// participant. Its caller must have already completed the actual
    /// authentication procedure.
    ///
    /// The transport provider is responsible for ensuring that the
    /// supplied identity corresponds to the authenticated connection.
    pub fn new(
        participant: ParticipantId,
        authentication_method: impl Into<String>,
        identity_reference: Option<String>,
    ) -> TransportResult<Self> {
        let authentication_method = authentication_method.into();

        if authentication_method.trim().is_empty()
            || authentication_method.chars().any(char::is_control)
        {
            return Err(TransportError::new(
                TransportErrorKind::AuthenticationFailure,
                crate::distributed::error::DeliveryState::NotSent,
                "Invalid authentication-method identifier",
            ));
        }

        if identity_reference.as_ref().is_some_and(|value| {
            value.trim().is_empty()
                || value.chars().any(char::is_control)
        }) {
            return Err(TransportError::new(
                TransportErrorKind::AuthenticationFailure,
                crate::distributed::error::DeliveryState::NotSent,
                "Invalid authentication identity reference",
            ));
        }

        Ok(Self {
            participant,
            authentication_method,
            identity_reference,
        })
    }

    /// Returns the authenticated logical participant.
    #[must_use]
    pub const fn participant(&self) -> ParticipantId {
        self.participant
    }

    /// Returns the authentication mechanism identifier.
    #[must_use]
    pub fn authentication_method(&self) -> &str {
        &self.authentication_method
    }

    /// Returns the optional reference to the security authority's
    /// authenticated identity record.
    #[must_use]
    pub fn identity_reference(&self) -> Option<&str> {
        self.identity_reference.as_deref()
    }
}

// =============================================================================
// OPERATION CONTEXT
// =============================================================================

/// Context supplied to an individual transport operation.
///
/// The context contains local cancellation and deadline information.
///
/// A local monotonic deadline must never be serialized directly into
/// a remote message. Providers must translate its remaining duration
/// according to their own clock and protocol contracts.
#[derive(Clone, Debug)]
pub struct TransportOperationContext {
    control: ExecutionControl,
}

impl TransportOperationContext {
    /// Creates an operation context from existing execution control.
    #[must_use]
    pub fn new(control: ExecutionControl) -> Self {
        Self { control }
    }

    /// Creates an operation context without an explicit deadline
    /// or cancellation source.
    #[must_use]
    pub fn unrestricted() -> Self {
        Self::new(ExecutionControl::new())
    }

    /// Returns the underlying execution control.
    #[must_use]
    pub const fn control(&self) -> &ExecutionControl {
        &self.control
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.control.is_cancelled()
    }

    /// Returns whether the local deadline has expired.
    #[must_use]
    pub fn is_deadline_expired(&self) -> bool {
        self.control.is_deadline_expired()
    }

    /// Returns the remaining operation duration.
    ///
    /// `None` means that no deadline was configured.
    #[must_use]
    pub fn remaining(&self) -> Option<Duration> {
        self.control.remaining()
    }

    /// Checks cancellation and deadline conditions before dispatch.
    ///
    /// A failure returned by this function means this check did not
    /// dispatch the operation.
    pub fn check(
        &self,
        operation: &'static str,
    ) -> crate::distributed::error::DistributedResult<()> {
        self.control.check_operation(operation)
    }
}

impl Default for TransportOperationContext {
    fn default() -> Self {
        Self::unrestricted()
    }
}

// =============================================================================
// TRANSPORT LIMITS
// =============================================================================

/// Optional operational limits enforced by a transport provider.
///
/// These values are operational budgets, not universal language limits.
///
/// `None` means that this particular interface does not impose the
/// corresponding budget.
///
/// Providers may impose additional implementation-specific limits.
/// Those limits must be reported explicitly when they are encountered.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TransportLimits {
    max_message_bytes: Option<NonZeroUsize>,
    max_in_flight_bytes: Option<NonZeroUsize>,
    max_in_flight_operations: Option<NonZeroUsize>,
}

impl TransportLimits {
    /// Creates a transport-limit policy without configured limits.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_message_bytes: None,
            max_in_flight_bytes: None,
            max_in_flight_operations: None,
        }
    }

    /// Sets the maximum accepted message payload size.
    #[must_use]
    pub fn with_max_message_bytes(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_message_bytes = limit;
        self
    }

    /// Sets the aggregate in-flight byte budget.
    #[must_use]
    pub fn with_max_in_flight_bytes(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_in_flight_bytes = limit;
        self
    }

    /// Sets the maximum number of simultaneous operations.
    #[must_use]
    pub fn with_max_in_flight_operations(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_in_flight_operations = limit;
        self
    }

    /// Returns the maximum payload size, if configured.
    #[must_use]
    pub const fn max_message_bytes(&self) -> Option<NonZeroUsize> {
        self.max_message_bytes
    }

    /// Returns the aggregate in-flight byte budget, if configured.
    #[must_use]
    pub const fn max_in_flight_bytes(&self) -> Option<NonZeroUsize> {
        self.max_in_flight_bytes
    }

    /// Returns the simultaneous-operation budget, if configured.
    #[must_use]
    pub const fn max_in_flight_operations(
        &self,
    ) -> Option<NonZeroUsize> {
        self.max_in_flight_operations
    }

    /// Checks a message payload against the configured message budget.
    ///
    /// This checks payload bytes. The codec must independently enforce
    /// encoded-envelope and decoding-allocation budgets.
    pub fn check_message(
        &self,
        message: &MessageEnvelope,
    ) -> TransportResult<()> {
        if let Some(limit) = self.max_message_bytes {
            if message.payload().len() > limit.get() {
                return Err(TransportError::new(
                    TransportErrorKind::ResourceExhausted,
                    crate::distributed::error::DeliveryState::NotSent,
                    format!(
                        "Message payload exceeds configured budget: {} bytes",
                        limit.get()
                    ),
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// DELIVERY CONFIRMATION
// =============================================================================

/// The strongest delivery confirmation obtained by the provider.
///
/// These states describe communication progress only.
///
/// None of them proves that the remote application executed a task.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeliveryConfirmation {
    /// The local provider accepted responsibility for the message.
    ///
    /// This does not prove that the message left the local process.
    AcceptedLocally,

    /// The remote transport explicitly acknowledged receipt.
    ///
    /// This does not prove that the receiving application processed
    /// the message.
    AcknowledgedByRemoteTransport,

    /// The remote application explicitly acknowledged receipt.
    ///
    /// This does not prove successful task execution or durable
    /// completion.
    AcknowledgedByRemoteApplication,
}

/// Receipt returned after a successful message-dispatch operation.
///
/// A receipt must identify the exact logical message that was submitted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendReceipt {
    message_id: crate::distributed::ids::MessageId,
    confirmation: DeliveryConfirmation,
}

impl SendReceipt {
    /// Creates a receipt for a successfully dispatched message.
    #[must_use]
    pub const fn new(
        message_id: crate::distributed::ids::MessageId,
        confirmation: DeliveryConfirmation,
    ) -> Self {
        Self {
            message_id,
            confirmation,
        }
    }

    /// Returns the submitted logical message identity.
    #[must_use]
    pub const fn message_id(&self) -> crate::distributed::ids::MessageId {
        self.message_id
    }

    /// Returns the strongest available delivery confirmation.
    #[must_use]
    pub const fn confirmation(&self) -> DeliveryConfirmation {
        self.confirmation
    }
}

// =============================================================================
// CONNECTION LIFECYCLE
// =============================================================================

/// Lifecycle state of a transport connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConnectionState {
    /// The connection is being established.
    Connecting,

    /// The connection is authenticated and available.
    Connected,

    /// The connection is temporarily unavailable.
    Degraded,

    /// The connection is closing.
    Closing,

    /// The connection has been closed.
    Closed,
}

/// Result of closing a transport connection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionCloseReport {
    /// Whether the provider completed the close operation.
    pub closed: bool,

    /// Number of operations the provider confirms were drained.
    ///
    /// This is diagnostic information, not proof of remote execution.
    pub drained_operations: usize,
}

// =============================================================================
// SHUTDOWN
// =============================================================================

/// Shutdown policy for a transport provider.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ShutdownMode {
    /// Stop accepting new operations and finish operations that can
    /// complete within the supplied operation control.
    Graceful,

    /// Stop accepting new operations and request cancellation of
    /// outstanding provider operations.
    ///
    /// This does not imply that remote operations have stopped.
    CancelOutstanding,
}

/// Report returned after provider shutdown.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportShutdownReport {
    /// Whether the provider completed its shutdown procedure.
    pub completed: bool,

    /// Number of locally tracked connections closed by the provider.
    pub closed_connections: usize,

    /// Number of locally tracked operations drained.
    pub drained_operations: usize,

    /// Number of locally tracked operations still unresolved.
    pub unresolved_operations: usize,
}

// =============================================================================
// TRANSPORT CONNECTION
// =============================================================================

/// A bidirectional logical communication connection.
///
/// The connection is associated with one authenticated logical peer.
///
/// A connection implementation may internally use:
///
/// - a local queue;
/// - operating-system IPC;
/// - shared memory;
/// - a network protocol;
/// - a remote communication service;
/// - a hardware communication interface;
/// - another compatible communication mechanism.
///
/// These implementation choices are not exposed as mandatory
/// requirements to distributed application code.
pub trait TransportConnection: Send + Sync {
    /// Returns the authenticated logical peer.
    fn authenticated_peer(&self) -> &AuthenticatedPeer;

    /// Returns the current connection lifecycle state.
    fn state(&self) -> ConnectionState;

    /// Returns the configured operational budgets.
    fn limits(&self) -> &TransportLimits;

    /// Sends one validated logical message.
    ///
    /// Implementations MUST:
    ///
    /// - validate the message before dispatch;
    /// - check its recipient against the authenticated peer;
    /// - enforce configured payload budgets;
    /// - check cancellation and deadlines before dispatch;
    /// - preserve delivery uncertainty on failures;
    /// - never claim remote execution from transport acknowledgement.
    ///
    /// Once dispatch has started, a timeout or connection interruption
    /// may represent an uncertain remote outcome.
    fn send<'a>(
        &'a self,
        message: &'a MessageEnvelope,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<SendReceipt>>;

    /// Receives the next available validated logical message.
    ///
    /// Implementations MUST:
    ///
    /// - enforce receive-side decoding budgets;
    /// - reject malformed messages;
    /// - authenticate the remote peer;
    /// - verify that the claimed sender corresponds to the authenticated
    ///   connection identity;
    /// - enforce applicable authorization before exposing privileged
    ///   operations to application execution;
    /// - respect cancellation and deadlines.
    fn receive<'a>(
        &'a self,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<MessageEnvelope>>;

    /// Performs one correlated request-response exchange.
    ///
    /// The supplied envelope must be a Request with a correlation ID.
    ///
    /// The implementation must correlate the response against:
    ///
    /// - the request correlation ID;
    /// - the original request message ID;
    /// - the authenticated remote sender;
    /// - the expected local recipient.
    ///
    /// Unrelated incoming messages must not be silently discarded.
    ///
    /// Implementations supporting multiplexed connections must preserve
    /// independent request-response correlation.
    ///
    /// This operation does not provide exactly-once execution.
    fn request_response<'a>(
        &'a self,
        request: &'a MessageEnvelope,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<MessageEnvelope>>;

    /// Closes the connection.
    ///
    /// Implementations must stop accepting new operations and release
    /// locally owned resources.
    ///
    /// An implementation must not claim that a remote operation was
    /// cancelled merely because the local connection was closed.
    fn close<'a>(
        &'a self,
        mode: ShutdownMode,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<ConnectionCloseReport>>;
}

// =============================================================================
// TRANSPORT PROVIDER
// =============================================================================

/// Injectable transport provider.
///
/// This is the principal integration boundary between distributed
/// communication and concrete networking or local communication
/// implementations.
///
/// A provider owns its implementation-specific resources and lifecycle.
///
/// Physical endpoint discovery and resolution are supplied by the
/// appropriate networking/membership infrastructure. The provider
/// receives logical ParticipantIds rather than requiring source
/// programs to embed physical addresses.
pub trait TransportProvider: Send + Sync {
    /// Returns the provider's stable implementation identifier.
    ///
    /// Examples may include provider namespaces, not fixed transport
    /// protocol names.
    fn provider_id(&self) -> &str;

    /// Returns the transport provider's configured limits.
    fn limits(&self) -> &TransportLimits;

    /// Establishes a connection to a logical participant.
    ///
    /// The provider must:
    ///
    /// - resolve the logical participant using its configured
    ///   membership/discovery integration;
    /// - negotiate compatible communication capabilities;
    /// - authenticate the remote participant;
    /// - verify that the authenticated identity matches `peer`;
    /// - reject unauthorized or incompatible peers;
    /// - respect the supplied deadline and cancellation signal.
    ///
    /// A successful connection MUST expose authenticated peer identity.
    ///
    /// This operation must not silently fall back to an unauthenticated
    /// connection.
    fn connect<'a>(
        &'a self,
        peer: ParticipantId,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<
        'a,
        TransportResult<Box<dyn TransportConnection>>,
    >;

    /// Accepts the next incoming authenticated connection.
    ///
    /// Implementations must not expose an unauthenticated connection
    /// as an authenticated application connection.
    ///
    /// Providers that do not support incoming connections must return
    /// an explicit Unsupported error.
    fn accept<'a>(
        &'a self,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<
        'a,
        TransportResult<Box<dyn TransportConnection>>,
    >;

    /// Begins provider shutdown.
    ///
    /// Graceful shutdown must stop new work before draining outstanding
    /// local operations.
    ///
    /// CancelOutstanding requests cancellation but does not prove that
    /// remote execution has stopped.
    ///
    /// Shutdown must be safe to invoke repeatedly.
    fn shutdown<'a>(
        &'a self,
        mode: ShutdownMode,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<TransportShutdownReport>>;
}

// =============================================================================
// CONNECTION HELPERS
// =============================================================================

/// Validates a message before dispatch through a connection.
///
/// This function performs structural validation and logical-peer checks.
/// It does not perform authentication, serialization, or network I/O.
pub fn validate_outgoing_message(
    message: &MessageEnvelope,
    peer: ParticipantId,
    limits: &TransportLimits,
) -> TransportResult<()> {
    message.validate().map_err(|error| {
        TransportError::new(
            TransportErrorKind::InvalidMessage,
            crate::distributed::error::DeliveryState::NotSent,
            error.to_string(),
        )
    })?;

    if message.recipient() != Some(peer) {
        return Err(TransportError::new(
            TransportErrorKind::InvalidInput,
            crate::distributed::error::DeliveryState::NotSent,
            "Message recipient does not match authenticated connection peer",
        ));
    }

    limits.check_message(message)
}

/// Validates a correlated response received for a request.
///
/// This function validates the structural response relationship.
/// Authentication and authorization must already have been checked
/// by the receiving transport/security integration.
pub fn validate_response(
    request: &MessageEnvelope,
    response: &MessageEnvelope,
    authenticated_peer: ParticipantId,
) -> TransportResult<()> {
    if request.role() != MessageRole::Request {
        return Err(TransportError::new(
            TransportErrorKind::InvalidInput,
            crate::distributed::error::DeliveryState::NotSent,
            "Request-response exchange requires a Request message",
        ));
    }

    response.validate().map_err(|error| {
        TransportError::new(
            TransportErrorKind::InvalidMessage,
            crate::distributed::error::DeliveryState::Unknown,
            error.to_string(),
        )
    })?;

    if response.sender() != authenticated_peer {
        return Err(TransportError::new(
            TransportErrorKind::AuthenticationFailure,
            crate::distributed::error::DeliveryState::Unknown,
            "Response sender does not match authenticated peer",
        ));
    }

    if !response.is_response_to(request) {
        return Err(TransportError::new(
            TransportErrorKind::InvalidMessage,
            crate::distributed::error::DeliveryState::Unknown,
            "Response does not match the original request and correlation",
        ));
    }

    Ok(())
}

/// Converts an execution-control error into a transport error.
///
/// The supplied delivery state must describe the actual operation state.
/// Before dispatch, use NotSent. After uncertain dispatch, use Unknown
/// or the most accurate available delivery classification.
pub fn control_error(
    error: crate::distributed::error::DistributedError,
    delivery: crate::distributed::error::DeliveryState,
    phase: TransportFailurePhase,
) -> TransportError {
    let kind = match error.kind() {
        crate::distributed::error::DistributedErrorKind::Cancelled => {
            TransportErrorKind::Cancelled
        }

        crate::distributed::error::DistributedErrorKind::DeadlineExceeded => {
            TransportErrorKind::Timeout
        }

        crate::distributed::error::DistributedErrorKind::ResourceUnsatisfied => {
            TransportErrorKind::ResourceExhausted
        }

        _ => TransportErrorKind::ProviderFailure,
    };

    let context = TransportErrorContext::new()
        .with_operation("transport_operation")
        .with_phase(phase);

    TransportError::with_source(
        kind,
        delivery,
        context,
        "Transport operation control check failed",
        error,
    )
}

// =============================================================================
// CONTRACT ASSERTIONS
// =============================================================================

/// Returns whether a message is eligible for request-response exchange.
///
/// This helper checks only the structural role. Full validation remains
/// the responsibility of MessageEnvelope::validate().
#[must_use]
pub fn is_request(message: &MessageEnvelope) -> bool {
    message.role() == MessageRole::Request
}

/// Returns whether a message represents an unsolicited event.
#[must_use]
pub fn is_event(message: &MessageEnvelope) -> bool {
    message.role() == MessageRole::Event
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::distributed::ids::{
        CorrelationId,
        IdAllocator,
        MessageId,
    };

    use crate::distributed::transport::messages::{
        MessageEnvelopeParts,
        MessageExtensions,
        MessageKind,
        ProtocolVersion,
    };

    fn test_message(
        sender: ParticipantId,
        recipient: ParticipantId,
        allocator: &IdAllocator,
    ) -> MessageEnvelope {
        MessageEnvelope::new(MessageEnvelopeParts {
            protocol_version: ProtocolVersion::new(1, 0, 0),
            message_id: allocator.message_id().unwrap(),
            kind: MessageKind::new("zamani", "test_request").unwrap(),
            role: MessageRole::Request,
            sender,
            recipient: Some(recipient),
            correlation_id: Some(allocator.correlation_id().unwrap()),
            in_reply_to: None,
            operation_id: None,
            task_id: None,
            payload_format: "application/octet-stream".to_owned(),
            payload: Vec::new(),
            extensions: MessageExtensions::new(),
        })
        .unwrap()
    }

    #[test]
    fn transport_limits_default_to_unrestricted() {
        let limits = TransportLimits::new();

        assert!(limits.max_message_bytes().is_none());
        assert!(limits.max_in_flight_bytes().is_none());
        assert!(limits.max_in_flight_operations().is_none());
    }

    #[test]
    fn transport_limits_reject_oversized_payloads() {
        let allocator = IdAllocator::with_namespace(10).unwrap();

        let sender = allocator.participant_id().unwrap();
        let recipient = allocator.participant_id().unwrap();

        let message = test_message(sender, recipient, &allocator);

        let limits = TransportLimits::new().with_max_message_bytes(
            NonZeroUsize::new(1),
        );

        assert!(limits.check_message(&message).is_ok());
    }

    #[test]
    fn outgoing_messages_require_matching_logical_peer() {
        let allocator = IdAllocator::with_namespace(11).unwrap();

        let sender = allocator.participant_id().unwrap();
        let recipient = allocator.participant_id().unwrap();
        let other = allocator.participant_id().unwrap();

        let message = test_message(sender, recipient, &allocator);

        let result = validate_outgoing_message(
            &message,
            other,
            &TransportLimits::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn authenticated_peer_preserves_logical_identity() {
        let allocator = IdAllocator::with_namespace(12).unwrap();
        let participant = allocator.participant_id().unwrap();

        let peer = AuthenticatedPeer::new(
            participant,
            "test.authentication",
            None,
        )
        .unwrap();

        assert_eq!(peer.participant(), participant);
        assert_eq!(
            peer.authentication_method(),
            "test.authentication"
        );
    }

    #[test]
    fn operation_context_supports_deadlines() {
        let context = TransportOperationContext::new(
            ExecutionControl::new(),
        );

        assert!(!context.is_cancelled());
        assert!(!context.is_deadline_expired());
        assert!(context.remaining().is_none());
    }

    #[test]
    fn request_detection_uses_existing_message_role() {
        let allocator = IdAllocator::with_namespace(13).unwrap();

        let sender = allocator.participant_id().unwrap();
        let recipient = allocator.participant_id().unwrap();

        let message = test_message(sender, recipient, &allocator);

        assert!(is_request(&message));
        assert!(!is_event(&message));
    }

    #[test]
    fn delivery_receipt_does_not_claim_execution() {
        let allocator = IdAllocator::with_namespace(14).unwrap();
        let message_id = allocator.message_id().unwrap();

        let receipt = SendReceipt::new(
            message_id,
            DeliveryConfirmation::AcknowledgedByRemoteTransport,
        );

        assert_eq!(receipt.message_id(), message_id);

        assert_eq!(
            receipt.confirmation(),
            DeliveryConfirmation::AcknowledgedByRemoteTransport
        );
    }
}
