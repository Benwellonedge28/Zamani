
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Provider-independent asynchronous transport interfaces for Zamani.
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Ownership
//!
//! This module owns:
//! - TransportProvider and TransportConnection interfaces.
//! - TransportOperationContext.
//! - AuthenticationContext and AuthenticatedPeer.
//! - Delivery confirmation.
//! - Connection lifecycle and provider shutdown contracts.
//! - Request/response transport contracts.
//! - Safe, runtime-independent asynchronous interface types.
//!
//! This module does not own:
//! - Logical message schemas or message envelope definitions.
//! - Protocol encoding or decoding.
//! - Physical endpoints, addresses, or endpoint discovery.
//! - Routing, placement, partitioning, or scheduling.
//! - Authentication algorithms or credential storage.
//! - Authorization policy.
//! - Automatic retry, idempotency, or recovery.
//! - Task execution or backend selection.
//! - Hardware allocation or device control.
//!
//! # Authoritative integration
//!
//! - `crate::distributed::transport::messages` owns message envelopes.
//! - `crate::distributed::transport::errors` owns transport errors.
//! - `crate::distributed::error` owns canonical distributed errors.
//! - `crate::distributed::ids` owns logical identities.
//! - `crate::distributed::config` owns operational resource budgets.
//! - `crate::distributed::execution::cancellation` owns cancellation
//!   and local monotonic deadlines.
//! - The networking subsystem owns physical communication facilities.
//! - The security subsystem owns authentication and authorization policy.
//!
//! # Delivery semantics
//!
//! A successful transport receipt describes communication progress only.
//! It does not prove that an application operation executed or completed.
//!
//! A timeout after dispatch may leave the remote outcome unknown.
//! Transport implementations must preserve that uncertainty.
//!
//! # Scalability
//!
//! No fixed participant, connection, worker, message, or topology ceiling
//! is imposed here. Operational budgets are supplied through the existing
//! `ResourceBudgets` authority.
//!
//! Literal infinite physical capacity is not promised.
//!
//! # Asynchronous execution
//!
//! This module uses standard-library futures and does not prescribe an
//! asynchronous runtime. Boxed futures keep the provider interfaces usable
//! through trait objects.

use core::any::Any;
use core::future::Future;
use core::pin::Pin;

use std::time::Duration;

use crate::distributed::config::ResourceBudgets;
use crate::distributed::error::{
    DeliveryState,
    DistributedError,
    DistributedErrorKind,
    DistributedResult,
};
use crate::distributed::execution::cancellation::ExecutionControl;
use crate::distributed::ids::{
    MessageId,
    ParticipantId,
};
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

/// A boxed asynchronous operation suitable for provider trait objects.
///
/// The future is `Send` so that implementations can integrate with
/// multithreaded runtimes. No particular runtime is required.
pub type TransportFuture<'a, T> =
    Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Compatibility name for the repository's canonical resource-budget type.
///
/// This is an alias, not a second budget implementation. New integrations
/// should generally use `ResourceBudgets` directly.
pub type TransportLimits = ResourceBudgets;

// =============================================================================
// AUTHENTICATION CONTEXT
// =============================================================================

/// Opaque authentication context supplied by the security authority.
///
/// The transport provider may downcast `provider_data()` to the concrete
/// authentication-context type it supports. Credential interpretation,
/// credential lifetime, secret handling and authentication policy remain
/// owned by the security subsystem.
///
/// Implementations MUST NOT expose credentials through `Debug`, logs,
/// error messages, or transport message payloads.
pub trait AuthenticationContext: Send + Sync {
    /// Returns the logical identity of the local participant.
    fn local_participant(&self) -> ParticipantId;

    /// Returns provider-specific authentication material.
    ///
    /// The returned value is opaque to this module. Providers must only
    /// interpret types they explicitly support.
    fn provider_data(&self) -> &(dyn Any + Send + Sync);
}

// =============================================================================
// AUTHENTICATED PEER
// =============================================================================

/// An identity established by a completed peer-authentication procedure.
///
/// This value is not an authorization grant. Authorization must be
/// evaluated independently by the security authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticatedPeer {
    participant: ParticipantId,
    authentication_method: String,
    identity_reference: Option<String>,
}

impl AuthenticatedPeer {
    /// Constructs authenticated-peer evidence.
    ///
    /// This constructor does not perform authentication. It may only be
    /// called after the provider has completed the actual authentication
    /// procedure and verified that the evidence belongs to this connection.
    ///
    /// `identity_reference` must be a non-secret reference to an identity
    /// record, never a password, private key, bearer token, or credential.
    pub fn new(
        participant: ParticipantId,
        authentication_method: impl Into<String>,
        identity_reference: Option<String>,
    ) -> TransportResult<Self> {
        let authentication_method = authentication_method.into();

        if !valid_identifier(&authentication_method) {
            return Err(TransportError::new(
                TransportErrorKind::AuthenticationFailure,
                DeliveryState::NotSent,
                "Invalid authentication-method identifier",
            ));
        }

        if identity_reference
            .as_deref()
            .is_some_and(|value| !valid_identifier(value))
        {
            return Err(TransportError::new(
                TransportErrorKind::AuthenticationFailure,
                DeliveryState::NotSent,
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

    /// Returns the optional non-secret identity-record reference.
    #[must_use]
    pub fn identity_reference(&self) -> Option<&str> {
        self.identity_reference.as_deref()
    }
}

fn valid_identifier(value: &str) -> bool {
    !value.trim().is_empty() && !value.chars().any(char::is_control)
}

// =============================================================================
// OPERATION CONTEXT
// =============================================================================

/// Context supplied to a transport operation.
///
/// The execution control contains a local cancellation signal and,
/// optionally, a local monotonic-clock deadline. Such a deadline must
/// not be serialized directly as a timestamp for another machine.
///
/// Resource budgets are cloned from the existing configuration authority.
/// They are operational policies, not language-level capacity limits.
#[derive(Clone, Debug)]
pub struct TransportOperationContext {
    control: ExecutionControl,
    budgets: ResourceBudgets,
}

impl TransportOperationContext {
    /// Creates an operation context without additional resource budgets.
    ///
    /// The provider remains subject to actual system and implementation
    /// resource limits.
    #[must_use]
    pub fn new(control: ExecutionControl) -> Self {
        Self {
            control,
            budgets: ResourceBudgets::new(),
        }
    }

    /// Creates an operation context using the repository's resource budgets.
    #[must_use]
    pub fn with_budgets(
        control: ExecutionControl,
        budgets: ResourceBudgets,
    ) -> Self {
        Self { control, budgets }
    }

    /// Creates an operation context from existing execution control
    /// and the existing distributed configuration.
    #[must_use]
    pub fn from_config(
        control: ExecutionControl,
        config: &crate::distributed::config::DistributedConfig,
    ) -> Self {
        Self::with_budgets(
            control,
            config.resource_budgets().clone(),
        )
    }

    /// Creates an operation context without an explicit deadline,
    /// cancellation source, or configured operational budget.
    #[must_use]
    pub fn unrestricted() -> Self {
        Self::new(ExecutionControl::new())
    }

    /// Returns the underlying execution control.
    #[must_use]
    pub const fn control(&self) -> &ExecutionControl {
        &self.control
    }

    /// Returns the operational resource budgets.
    #[must_use]
    pub const fn budgets(&self) -> &ResourceBudgets {
        &self.budgets
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

    /// Returns the remaining duration until the local deadline.
    ///
    /// `None` means no deadline is configured.
    #[must_use]
    pub fn remaining(&self) -> Option<Duration> {
        self.control.remaining()
    }

    /// Checks cancellation and deadline conditions before dispatch.
    ///
    /// An error from this method means that this check did not dispatch
    /// the operation.
    pub fn check(
        &self,
        operation: &'static str,
    ) -> DistributedResult<()> {
        self.control.check_operation(operation)
    }
}

impl Default for TransportOperationContext {
    fn default() -> Self {
        Self::unrestricted()
    }
}

// =============================================================================
// DELIVERY CONFIRMATION
// =============================================================================

/// The strongest delivery confirmation obtained by the provider.
///
/// None of these states proves that the remote application executed
/// a task or durably committed its result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeliveryConfirmation {
    /// The local provider accepted responsibility for the message.
    ///
    /// This does not prove that the message left the local process.
    AcceptedLocally,

    /// The remote transport explicitly acknowledged receipt.
    ///
    /// This does not prove that the receiving application processed it.
    AcknowledgedByRemoteTransport,

    /// The remote application explicitly acknowledged receipt.
    ///
    /// This does not prove successful execution or durable completion.
    AcknowledgedByRemoteApplication,
}

/// Receipt returned after a successful message-dispatch operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendReceipt {
    message_id: MessageId,
    confirmation: DeliveryConfirmation,
}

impl SendReceipt {
    /// Constructs a receipt for a successfully dispatched message.
    #[must_use]
    pub const fn new(
        message_id: MessageId,
        confirmation: DeliveryConfirmation,
    ) -> Self {
        Self {
            message_id,
            confirmation,
        }
    }

    /// Returns the submitted logical message identity.
    #[must_use]
    pub const fn message_id(&self) -> MessageId {
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

    /// Number of locally tracked operations the provider confirms drained.
    ///
    /// This does not prove remote execution.
    pub drained_operations: usize,
}

// =============================================================================
// PROVIDER SHUTDOWN
// =============================================================================

/// Shutdown policy for a transport provider.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ShutdownMode {
    /// Stop accepting new work and drain operations that can complete
    /// within the supplied operation control.
    Graceful,

    /// Stop accepting new work and request cancellation of outstanding
    /// local provider operations.
    ///
    /// This does not prove that remote operations have stopped.
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

/// A bidirectional logical communication connection to one authenticated
/// participant.
///
/// The connection may internally use local queues, IPC, shared memory,
/// network protocols, remote communication services, hardware interfaces,
/// or future communication mechanisms.
///
/// Implementations must document their concurrency and ordering guarantees.
pub trait TransportConnection: Send + Sync {
    /// Returns the logical identity of the local participant.
    fn local_participant(&self) -> ParticipantId;

    /// Returns the authenticated logical peer.
    fn authenticated_peer(&self) -> &AuthenticatedPeer;

    /// Returns the current connection lifecycle state.
    fn state(&self) -> ConnectionState;

    /// Returns the provider's configured resource budgets.
    ///
    /// The returned type is the canonical `ResourceBudgets` type through
    /// the `TransportLimits` compatibility alias.
    fn limits(&self) -> &TransportLimits;

    /// Sends one validated logical message.
    ///
    /// This checked method validates the envelope, sender identity,
    /// recipient identity where specified, cancellation and deadline
    /// before calling `send_message`.
    fn send<'a>(
        &'a self,
        message: &'a MessageEnvelope,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<SendReceipt>> {
        Box::pin(async move {
            context
                .check("transport.send")
                .map_err(|error| {
                    control_error(
                        error,
                        DeliveryState::NotSent,
                        TransportFailurePhase::Sending,
                    )
                })?;

            validate_outgoing_message(
                message,
                self.local_participant(),
                self.authenticated_peer().participant(),
            )?;

            self.send_message(message, context).await
        })
    }

    /// Provider implementation hook for dispatching a structurally
    /// validated message.
    ///
    /// Consumers should call `send`, not this lower-level method.
    /// Implementations must preserve accurate delivery state on errors.
    fn send_message<'a>(
        &'a self,
        message: &'a MessageEnvelope,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<SendReceipt>>;

    /// Receives the next available validated logical message.
    ///
    /// A successful receive verifies that the claimed sender matches
    /// the connection's authenticated peer and that any specified
    /// recipient matches the local participant.
    fn receive<'a>(
        &'a self,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<MessageEnvelope>> {
        Box::pin(async move {
            context
                .check("transport.receive")
                .map_err(|error| {
                    control_error(
                        error,
                        DeliveryState::NotSent,
                        TransportFailurePhase::Receiving,
                    )
                })?;

            let message = self.receive_message(context).await?;

            message.validate().map_err(|error| {
                TransportError::with_source(
                    TransportErrorKind::InvalidMessage,
                    DeliveryState::Unknown,
                    TransportErrorContext::new()
                        .with_operation("transport.receive")
                        .with_peer(self.authenticated_peer().participant().to_string())
                        .with_phase(TransportFailurePhase::Receiving),
                    "Received message violates the envelope contract",
                    error,
                )
            })?;

            if message.sender() != self.authenticated_peer().participant() {
                return Err(TransportError::new(
                    TransportErrorKind::AuthenticationFailure,
                    DeliveryState::Unknown,
                    "Claimed message sender does not match the authenticated peer",
                ));
            }

            if message
                .recipient()
                .is_some_and(|recipient| recipient != self.local_participant())
            {
                return Err(TransportError::new(
                    TransportErrorKind::InvalidMessage,
                    DeliveryState::Unknown,
                    "Received message is addressed to another participant",
                ));
            }

            Ok(message)
        })
    }

    /// Provider implementation hook for receiving a message.
    ///
    /// Implementations must enforce codec budgets and complete peer
    /// authentication before returning a message to the checked `receive`
    /// method.
    fn receive_message<'a>(
        &'a self,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<MessageEnvelope>>;

    /// Performs one correlated request-response exchange.
    ///
    /// The checked method validates the request before dispatch and checks
    /// the response against the original request and authenticated peer.
    ///
    /// Implementations must multiplex concurrent exchanges correctly.
    /// They must not implement this by consuming arbitrary messages from
    /// a shared receive stream and discarding unrelated messages.
    fn request_response<'a>(
        &'a self,
        request: &'a MessageEnvelope,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<MessageEnvelope>> {
        Box::pin(async move {
            context
                .check("transport.request_response")
                .map_err(|error| {
                    control_error(
                        error,
                        DeliveryState::NotSent,
                        TransportFailurePhase::Sending,
                    )
                })?;

            validate_outgoing_message(
                request,
                self.local_participant(),
                self.authenticated_peer().participant(),
            )?;

            if request.role() != MessageRole::Request {
                return Err(TransportError::new(
                    TransportErrorKind::InvalidInput,
                    DeliveryState::NotSent,
                    "Request-response exchange requires a Request envelope",
                ));
            }

            let response = self.exchange_message(request, context).await?;

            validate_response(
                request,
                &response,
                self.authenticated_peer().participant(),
            )?;

            Ok(response)
        })
    }

    /// Provider implementation hook for an atomic, correlated exchange.
    ///
    /// The provider must preserve unrelated incoming messages and must
    /// correlate the response by correlation ID and original request ID.
    fn exchange_message<'a>(
        &'a self,
        request: &'a MessageEnvelope,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<MessageEnvelope>>;

    /// Closes the connection and releases locally owned resources.
    ///
    /// Closing a connection does not prove that a remote operation was
    /// cancelled or did not execute.
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
/// A provider owns its concrete communication resources and lifecycle.
/// Physical endpoint discovery and resolution are delegated to the
/// networking and membership authorities.
///
/// Providers receive logical participant identities rather than requiring
/// source programs to embed physical addresses.
pub trait TransportProvider: Send + Sync {
    /// Returns the provider's stable implementation identifier.
    ///
    /// This identifier is descriptive metadata, not a transport protocol
    /// catalogue or a physical endpoint.
    fn provider_id(&self) -> &str;

    /// Returns the provider's configured resource budgets.
    fn limits(&self) -> &TransportLimits;

    /// Establishes an authenticated connection to a logical participant.
    ///
    /// The default wrapper verifies that the returned connection identifies
    /// both the expected local participant and the requested remote peer.
    fn connect<'a>(
        &'a self,
        peer: ParticipantId,
        authentication: &'a dyn AuthenticationContext,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<Box<dyn TransportConnection + 'a>>> {
        Box::pin(async move {
            context
                .check("transport.connect")
                .map_err(|error| {
                    control_error(
                        error,
                        DeliveryState::NotSent,
                        TransportFailurePhase::Connection,
                    )
                })?;

            let connection = self
                .establish(peer, authentication, context)
                .await?;

            let identity_matches =
                connection.local_participant() == authentication.local_participant()
                    && connection.authenticated_peer().participant() == peer;

            if !identity_matches {
                let _ = connection
                    .close(ShutdownMode::CancelOutstanding, context)
                    .await;

                return Err(TransportError::new(
                    TransportErrorKind::AuthenticationFailure,
                    DeliveryState::NotSent,
                    "Established connection identity does not match the requested participants",
                ));
            }

            Ok(connection)
        })
    }

    /// Provider implementation hook for establishing a connection.
    ///
    /// Implementations must resolve the logical peer, negotiate protocol
    /// compatibility, authenticate the remote identity and reject a peer
    /// that cannot be verified.
    fn establish<'a>(
        &'a self,
        peer: ParticipantId,
        authentication: &'a dyn AuthenticationContext,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<Box<dyn TransportConnection + 'a>>>;

    /// Accepts the next incoming authenticated connection.
    ///
    /// The default wrapper checks that the accepted connection belongs
    /// to the expected local participant.
    fn accept<'a>(
        &'a self,
        authentication: &'a dyn AuthenticationContext,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<Box<dyn TransportConnection + 'a>>> {
        Box::pin(async move {
            context
                .check("transport.accept")
                .map_err(|error| {
                    control_error(
                        error,
                        DeliveryState::NotSent,
                        TransportFailurePhase::Receiving,
                    )
                })?;

            let connection = self.accept_connection(authentication, context).await?;

            if connection.local_participant() != authentication.local_participant() {
                let _ = connection
                    .close(ShutdownMode::CancelOutstanding, context)
                    .await;

                return Err(TransportError::new(
                    TransportErrorKind::AuthenticationFailure,
                    DeliveryState::NotSent,
                    "Accepted connection has an unexpected local participant identity",
                ));
            }

            Ok(connection)
        })
    }

    /// Provider implementation hook for accepting an incoming connection.
    ///
    /// Providers without listener/accept support must return an explicit
    /// `Unsupported` error. Implementations must authenticate the peer
    /// before returning the connection.
    fn accept_connection<'a>(
        &'a self,
        _authentication: &'a dyn AuthenticationContext,
        _context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<Box<dyn TransportConnection + 'a>>> {
        Box::pin(async {
            Err(TransportError::new(
                TransportErrorKind::Unsupported,
                DeliveryState::NotSent,
                "This transport provider does not support incoming connections",
            ))
        })
    }

    /// Begins provider shutdown.
    ///
    /// Graceful shutdown must stop accepting new work before draining
    /// outstanding local operations. Shutdown must be safe to invoke
    /// repeatedly and must report unresolved operations honestly.
    fn shutdown<'a>(
        &'a self,
        mode: ShutdownMode,
        context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<TransportShutdownReport>>;
}

// =============================================================================
// VALIDATION HELPERS
// =============================================================================

/// Validates a message before dispatch through a peer connection.
///
/// The sender must be the local participant. A specified recipient must
/// be the authenticated peer. An absent recipient is permitted for
/// logical event/control messages whose routing is handled elsewhere.
///
/// This function does not encode the message or perform network I/O.
pub fn validate_outgoing_message(
    message: &MessageEnvelope,
    local_participant: ParticipantId,
    authenticated_peer: ParticipantId,
) -> TransportResult<()> {
    message.validate().map_err(|error| {
        TransportError::with_source(
            TransportErrorKind::InvalidMessage,
            DeliveryState::NotSent,
            TransportErrorContext::new()
                .with_operation("transport.send")
                .with_phase(TransportFailurePhase::Sending),
            "Outgoing message violates the envelope contract",
            error,
        )
    })?;

    if message.sender() != local_participant {
        return Err(TransportError::new(
            TransportErrorKind::AuthenticationFailure,
            DeliveryState::NotSent,
            "Outgoing message sender does not match the local participant identity",
        ));
    }

    if message
        .recipient()
        .is_some_and(|recipient| recipient != authenticated_peer)
    {
        return Err(TransportError::new(
            TransportErrorKind::InvalidInput,
            DeliveryState::NotSent,
            "Message recipient does not match the authenticated connection peer",
        ));
    }

    Ok(())
}

/// Validates a correlated response received for a request.
///
/// The response must originate from the authenticated peer and must
/// identify the original request and correlation relationship.
pub fn validate_response(
    request: &MessageEnvelope,
    response: &MessageEnvelope,
    authenticated_peer: ParticipantId,
) -> TransportResult<()> {
    if request.role() != MessageRole::Request {
        return Err(TransportError::new(
            TransportErrorKind::InvalidInput,
            DeliveryState::NotSent,
            "Request-response exchange requires a Request message",
        ));
    }

    response.validate().map_err(|error| {
        TransportError::with_source(
            TransportErrorKind::InvalidMessage,
            DeliveryState::Unknown,
            TransportErrorContext::new()
                .with_operation("transport.request_response")
                .with_phase(TransportFailurePhase::Correlation),
            "Response violates the envelope contract",
            error,
        )
    })?;

    if response.sender() != authenticated_peer {
        return Err(TransportError::new(
            TransportErrorKind::AuthenticationFailure,
            DeliveryState::Unknown,
            "Response sender does not match the authenticated peer",
        ));
    }

    if !response.is_response_to(request) {
        return Err(TransportError::new(
            TransportErrorKind::InvalidMessage,
            DeliveryState::Unknown,
            "Response does not match the original request and correlation",
        ));
    }

    Ok(())
}

/// Converts an execution-control error into a transport error.
///
/// The caller must supply the actual delivery state. Use `NotSent` only
/// when dispatch definitely did not begin. Use `Unknown`, `MayHaveExecuted`,
/// or `ResponseLost` when the remote outcome cannot be established.
pub fn control_error(
    error: DistributedError,
    delivery: DeliveryState,
    phase: TransportFailurePhase,
) -> TransportError {
    let kind = match error.kind() {
        DistributedErrorKind::Cancelled => TransportErrorKind::Cancelled,
        DistributedErrorKind::DeadlineExceeded => TransportErrorKind::Timeout,
        DistributedErrorKind::ResourceUnsatisfied => {
            TransportErrorKind::ResourceExhausted
        }
        DistributedErrorKind::InvalidInput
        | DistributedErrorKind::InvalidConfiguration => {
            TransportErrorKind::InvalidInput
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

/// Returns whether a message is structurally a request.
///
/// This checks the existing envelope role only. Full validation remains
/// the responsibility of `MessageEnvelope::validate`.
#[must_use]
pub fn is_request(message: &MessageEnvelope) -> bool {
    message.role() == MessageRole::Request
}

/// Returns whether a message is structurally an event.
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

    use crate::distributed::ids::IdAllocator;
    use crate::distributed::transport::messages::{
        MessageEnvelopeParts,
        MessageExtensions,
        MessageKind,
        ProtocolVersion,
    };

    fn request(
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
            payload: b"request".to_vec(),
            extensions: MessageExtensions::new(),
        })
        .unwrap()
    }

    fn response(
        original: &MessageEnvelope,
        allocator: &IdAllocator,
    ) -> MessageEnvelope {
        MessageEnvelope::new(MessageEnvelopeParts {
            protocol_version: original.protocol_version(),
            message_id: allocator.message_id().unwrap(),
            kind: MessageKind::new("zamani", "test_response").unwrap(),
            role: MessageRole::Response,
            sender: original.recipient().unwrap(),
            recipient: Some(original.sender()),
            correlation_id: original.correlation_id(),
            in_reply_to: Some(original.message_id()),
            operation_id: original.operation_id(),
            task_id: original.task_id(),
            payload_format: "application/octet-stream".to_owned(),
            payload: b"response".to_vec(),
            extensions: MessageExtensions::new(),
        })
        .unwrap()
    }

    #[test]
    fn transport_limits_are_the_existing_resource_budget_type() {
        let budgets = TransportLimits::new();

        assert!(budgets.max_message_bytes().is_none());
        assert!(budgets.max_in_flight_bytes().is_none());
        assert!(budgets.max_in_flight_operations().is_none());
    }

    #[test]
    fn operation_context_preserves_resource_budgets() {
        let budgets = ResourceBudgets::new();
        let context = TransportOperationContext::with_budgets(
            ExecutionControl::new(),
            budgets.clone(),
        );

        assert_eq!(context.budgets(), &budgets);
        assert!(!context.is_cancelled());
        assert!(!context.is_deadline_expired());
        assert!(context.remaining().is_none());
    }

    #[test]
    fn outgoing_message_requires_local_sender_identity() {
        let allocator = IdAllocator::with_namespace(101).unwrap();
        let local = allocator.participant_id().unwrap();
        let peer = allocator.participant_id().unwrap();
        let other = allocator.participant_id().unwrap();

        let message = request(other, peer, &allocator);

        let result = validate_outgoing_message(&message, local, peer);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            TransportErrorKind::AuthenticationFailure
        );
    }

    #[test]
    fn outgoing_message_requires_matching_recipient() {
        let allocator = IdAllocator::with_namespace(102).unwrap();
        let local = allocator.participant_id().unwrap();
        let peer = allocator.participant_id().unwrap();
        let other = allocator.participant_id().unwrap();

        let message = request(local, other, &allocator);

        let result = validate_outgoing_message(&message, local, peer);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            TransportErrorKind::InvalidInput
        );
    }

    #[test]
    fn valid_outgoing_message_matches_both_participants() {
        let allocator = IdAllocator::with_namespace(103).unwrap();
        let local = allocator.participant_id().unwrap();
        let peer = allocator.participant_id().unwrap();

        let message = request(local, peer, &allocator);

        assert!(
            validate_outgoing_message(&message, local, peer).is_ok()
        );
    }

    #[test]
    fn response_must_match_request_and_authenticated_peer() {
        let allocator = IdAllocator::with_namespace(104).unwrap();
        let local = allocator.participant_id().unwrap();
        let peer = allocator.participant_id().unwrap();

        let request = request(local, peer, &allocator);
        let response = response(&request, &allocator);

        assert!(validate_response(&request, &response, peer).is_ok());
    }

    #[test]
    fn response_from_another_peer_is_rejected() {
        let allocator = IdAllocator::with_namespace(105).unwrap();
        let local = allocator.participant_id().unwrap();
        let peer = allocator.participant_id().unwrap();
        let impostor = allocator.participant_id().unwrap();

        let request = request(local, peer, &allocator);
        let response = response(&request, &allocator);

        let result = validate_response(&request, &response, impostor);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            TransportErrorKind::AuthenticationFailure
        );
    }

    #[test]
    fn authenticated_peer_rejects_invalid_method_identifiers() {
        let allocator = IdAllocator::with_namespace(106).unwrap();
        let participant = allocator.participant_id().unwrap();

        assert!(
            AuthenticatedPeer::new(participant, " ", None).is_err()
        );

        assert!(
            AuthenticatedPeer::new(
                participant,
                "valid.method",
                Some("invalid\nreference".to_owned()),
            )
            .is_err()
        );
    }

    #[test]
    fn authenticated_peer_preserves_identity_metadata() {
        let allocator = IdAllocator::with_namespace(107).unwrap();
        let participant = allocator.participant_id().unwrap();

        let peer = AuthenticatedPeer::new(
            participant,
            "test.authentication",
            Some("identity-record-1".to_owned()),
        )
        .unwrap();

        assert_eq!(peer.participant(), participant);
        assert_eq!(
            peer.authentication_method(),
            "test.authentication"
        );
        assert_eq!(
            peer.identity_reference(),
            Some("identity-record-1")
        );
    }

    #[test]
    fn request_detection_uses_existing_message_role() {
        let allocator = IdAllocator::with_namespace(108).unwrap();
        let local = allocator.participant_id().unwrap();
        let peer = allocator.participant_id().unwrap();

        let message = request(local, peer, &allocator);

        assert!(is_request(&message));
        assert!(!is_event(&message));
    }

    #[test]
    fn delivery_receipt_does_not_claim_execution() {
        let allocator = IdAllocator::with_namespace(109).unwrap();
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
