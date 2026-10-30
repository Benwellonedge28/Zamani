
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Integration tests for Zamani's distributed transport subsystem.
//!
//! Path: src/distributed/tests/transport.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! These tests verify the existing transport contracts. They do not define
//! a second transport implementation or protocol.
//!
//! The in-memory provider is test infrastructure only. Production providers
//! must integrate with the repository's networking and security authorities.
//!
//! No test establishes a universal message-size, participant-count, or
//! topology ceiling. Explicit budget tests verify configured policy behavior.

use std::any::Any;
use std::collections::VecDeque;
use std::future::Future;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;

use crate::distributed::config::ResourceBudgets;
use crate::distributed::error::DeliveryState;
use crate::distributed::execution::cancellation::{
    CancellationToken,
    ExecutionControl,
};
use crate::distributed::ids::{
    CorrelationId,
    IdAllocator,
    MessageId,
    ParticipantId,
};
use crate::distributed::transport::codec::{
    CodecLimits,
    DistributedMessageCodec,
    ProtocolCompatibility,
};
use crate::distributed::transport::errors::{
    TransportError,
    TransportErrorKind,
    TransportResult,
};
use crate::distributed::transport::messages::{
    MessageEnvelope,
    MessageEnvelopeParts,
    MessageExtensions,
    MessageKind,
    MessageRole,
    ProtocolVersion,
};
use crate::distributed::transport::traits::{
    AuthenticatedPeer,
    AuthenticationContext,
    ConnectionCloseReport,
    ConnectionState,
    DeliveryConfirmation,
    SendReceipt,
    ShutdownMode,
    TransportConnection,
    TransportFuture,
    TransportOperationContext,
    TransportProvider,
    TransportShutdownReport,
};

// =============================================================================
// TEST EXECUTOR
// =============================================================================

/// A safe, minimal executor for the immediately-completing futures used
/// by the in-memory test provider.
///
/// This deliberately avoids requiring Tokio or another asynchronous
/// runtime merely to test the runtime-independent transport interfaces.
struct ThreadWake(thread::Thread);

impl Wake for ThreadWake {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(ThreadWake(thread::current())));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    loop {
        match Pin::as_mut(&mut future).poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => thread::park(),
        }
    }
}

// =============================================================================
// TEST IDENTITIES AND MESSAGE BUILDERS
// =============================================================================

fn local_id() -> ParticipantId {
    ParticipantId::new(1, 1).expect("valid local participant ID")
}

fn remote_id() -> ParticipantId {
    ParticipantId::new(1, 2).expect("valid remote participant ID")
}

fn third_party_id() -> ParticipantId {
    ParticipantId::new(1, 3).expect("valid third participant ID")
}

fn request_message(
    sender: ParticipantId,
    recipient: ParticipantId,
    message_id: MessageId,
    correlation_id: CorrelationId,
) -> MessageEnvelope {
    MessageEnvelope::new(MessageEnvelopeParts {
        protocol_version: ProtocolVersion::new(1, 0, 0),
        message_id,
        kind: MessageKind::new("zamani", "task_request")
            .expect("valid message kind"),
        role: MessageRole::Request,
        sender,
        recipient: Some(recipient),
        correlation_id: Some(correlation_id),
        in_reply_to: None,
        operation_id: None,
        task_id: None,
        payload_format: "application/octet-stream".to_owned(),
        payload: vec![1, 2, 3, 4],
        extensions: MessageExtensions::new(),
    })
    .expect("valid request envelope")
}

fn response_message(
    sender: ParticipantId,
    recipient: ParticipantId,
    message_id: MessageId,
    request: &MessageEnvelope,
    correlation_id: CorrelationId,
) -> MessageEnvelope {
    MessageEnvelope::new(MessageEnvelopeParts {
        protocol_version: request.protocol_version(),
        message_id,
        kind: MessageKind::new("zamani", "task_result")
            .expect("valid message kind"),
        role: MessageRole::Response,
        sender,
        recipient: Some(recipient),
        correlation_id: Some(correlation_id),
        in_reply_to: Some(request.message_id()),
        operation_id: request.operation_id(),
        task_id: request.task_id(),
        payload_format: "application/octet-stream".to_owned(),
        payload: vec![9, 8, 7],
        extensions: MessageExtensions::new(),
    })
    .expect("valid response envelope")
}

fn codec(
    version: ProtocolVersion,
    compatibility: ProtocolCompatibility,
    limits: CodecLimits,
) -> DistributedMessageCodec {
    DistributedMessageCodec::new(version, compatibility, limits)
}

fn default_codec() -> DistributedMessageCodec {
    codec(
        ProtocolVersion::new(1, 0, 0),
        ProtocolCompatibility::SameMajor,
        CodecLimits::new(),
    )
}

// =============================================================================
// AUTHENTICATION TEST FIXTURE
// =============================================================================

/// Test authentication context.
///
/// This supplies identity metadata only. It does not implement
/// authentication or establish trust.
struct TestAuthentication {
    local: ParticipantId,
    provider_data: (),
}

impl TestAuthentication {
    fn new(local: ParticipantId) -> Self {
        Self {
            local,
            provider_data: (),
        }
    }
}

impl AuthenticationContext for TestAuthentication {
    fn local_participant(&self) -> ParticipantId {
        self.local
    }

    fn provider_data(&self) -> &(dyn Any + Send + Sync) {
        &self.provider_data
    }
}

// =============================================================================
// IN-MEMORY CONNECTION FIXTURE
// =============================================================================

#[derive(Clone, Copy)]
enum ResponseBehavior {
    Correct,
    WrongCorrelation,
    WrongSender,
}

/// A test-only in-memory connection.
///
/// It models the transport interface and its validation boundaries.
/// It does not model real network authentication, encryption, durability,
/// or remote execution.
struct InMemoryConnection {
    local: ParticipantId,
    peer: AuthenticatedPeer,
    budgets: ResourceBudgets,
    state: AtomicBool,
    send_count: AtomicUsize,
    incoming: Mutex<VecDeque<MessageEnvelope>>,
    response_behavior: ResponseBehavior,
}

impl InMemoryConnection {
    fn new(
        local: ParticipantId,
        peer: ParticipantId,
        response_behavior: ResponseBehavior,
    ) -> Self {
        Self {
            local,
            peer: AuthenticatedPeer::new(
                peer,
                "test.authentication",
                Some("test-peer-record".to_owned()),
            )
            .expect("valid authenticated peer fixture"),
            budgets: ResourceBudgets::new(),
            state: AtomicBool::new(true),
            send_count: AtomicUsize::new(0),
            incoming: Mutex::new(VecDeque::new()),
            response_behavior,
        }
    }

    fn with_incoming(
        mut self,
        message: MessageEnvelope,
    ) -> Self {
        self.incoming
            .get_mut()
            .expect("test queue is not poisoned")
            .push_back(message);

        self
    }

    fn send_count(&self) -> usize {
        self.send_count.load(Ordering::Acquire)
    }
}

impl TransportConnection for InMemoryConnection {
    fn local_participant(&self) -> ParticipantId {
        self.local
    }

    fn authenticated_peer(&self) -> &AuthenticatedPeer {
        &self.peer
    }

    fn state(&self) -> ConnectionState {
        if self.state.load(Ordering::Acquire) {
            ConnectionState::Connected
        } else {
            ConnectionState::Closed
        }
    }

    fn limits(&self) -> &ResourceBudgets {
        &self.budgets
    }

    fn send_message<'a>(
        &'a self,
        message: &'a MessageEnvelope,
        _context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<SendReceipt>> {
        Box::pin(async move {
            self.send_count.fetch_add(1, Ordering::AcqRel);

            Ok(SendReceipt::new(
                message.message_id(),
                DeliveryConfirmation::AcceptedLocally,
            ))
        })
    }

    fn receive_message<'a>(
        &'a self,
        _context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<MessageEnvelope>> {
        Box::pin(async move {
            self.incoming
                .lock()
                .map_err(|_| {
                    TransportError::new(
                        TransportErrorKind::ProviderFailure,
                        DeliveryState::Unknown,
                        "test receive queue lock was poisoned",
                    )
                })?
                .pop_front()
                .ok_or_else(|| {
                    TransportError::new(
                        TransportErrorKind::PeerUnavailable,
                        DeliveryState::NotSent,
                        "no test message is available",
                    )
                })
        })
    }

    fn exchange_message<'a>(
        &'a self,
        request: &'a MessageEnvelope,
        _context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<MessageEnvelope>> {
        Box::pin(async move {
            let response_sender = match self.response_behavior {
                ResponseBehavior::WrongSender => third_party_id(),
                ResponseBehavior::Correct | ResponseBehavior::WrongCorrelation => {
                    self.peer.participant()
                }
            };

            let correlation = match self.response_behavior {
                ResponseBehavior::WrongCorrelation => {
                    CorrelationId::new(1, 999)
                        .expect("valid test correlation ID")
                }
                ResponseBehavior::Correct | ResponseBehavior::WrongSender => {
                    request
                        .correlation_id()
                        .expect("request has correlation ID")
                }
            };

            Ok(response_message(
                response_sender,
                self.local,
                MessageId::new(1, 500)
                    .expect("valid response message ID"),
                request,
                correlation,
            ))
        })
    }

    fn close<'a>(
        &'a self,
        _mode: ShutdownMode,
        _context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<ConnectionCloseReport>> {
        Box::pin(async move {
            self.state.store(false, Ordering::Release);

            Ok(ConnectionCloseReport {
                closed: true,
                drained_operations: 0,
            })
        })
    }
}

// =============================================================================
// IN-MEMORY PROVIDER FIXTURE
// =============================================================================

struct InMemoryProvider {
    local: ParticipantId,
    peer: ParticipantId,
    budgets: ResourceBudgets,
    response_behavior: ResponseBehavior,
    shutdown_count: AtomicUsize,
}

impl InMemoryProvider {
    fn new(
        local: ParticipantId,
        peer: ParticipantId,
        response_behavior: ResponseBehavior,
    ) -> Self {
        Self {
            local,
            peer,
            budgets: ResourceBudgets::new(),
            response_behavior,
            shutdown_count: AtomicUsize::new(0),
        }
    }
}

impl TransportProvider for InMemoryProvider {
    fn provider_id(&self) -> &str {
        "zamani.test.in_memory"
    }

    fn limits(&self) -> &ResourceBudgets {
        &self.budgets
    }

    fn establish<'a>(
        &'a self,
        peer: ParticipantId,
        authentication: &'a dyn AuthenticationContext,
        _context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<Box<dyn TransportConnection + 'a>>> {
        Box::pin(async move {
            if peer != self.peer {
                return Err(TransportError::new(
                    TransportErrorKind::PeerUnavailable,
                    DeliveryState::NotSent,
                    "test provider does not know the requested peer",
                ));
            }

            Ok(Box::new(InMemoryConnection::new(
                authentication.local_participant(),
                self.peer,
                self.response_behavior,
            )) as Box<dyn TransportConnection + 'a>)
        })
    }

    fn shutdown<'a>(
        &'a self,
        _mode: ShutdownMode,
        _context: &'a TransportOperationContext,
    ) -> TransportFuture<'a, TransportResult<TransportShutdownReport>> {
        Box::pin(async move {
            self.shutdown_count.fetch_add(1, Ordering::AcqRel);

            Ok(TransportShutdownReport {
                completed: true,
                closed_connections: 1,
                drained_operations: 0,
                unresolved_operations: 0,
            })
        })
    }
}

// =============================================================================
// MESSAGE CONTRACT TESTS
// =============================================================================

#[test]
fn request_envelope_preserves_logical_identity_and_correlation() {
    let request = request_message(
        local_id(),
        remote_id(),
        MessageId::new(1, 10).unwrap(),
        CorrelationId::new(1, 11).unwrap(),
    );

    assert_eq!(request.role(), MessageRole::Request);
    assert_eq!(request.sender(), local_id());
    assert_eq!(request.recipient(), Some(remote_id()));
    assert_eq!(
        request.correlation_id(),
        Some(CorrelationId::new(1, 11).unwrap())
    );
    assert_eq!(request.payload(), &[1, 2, 3, 4]);
    assert!(request.validate().is_ok());
}

#[test]
fn response_must_identify_its_request_and_correlation() {
    let request = request_message(
        local_id(),
        remote_id(),
        MessageId::new(1, 20).unwrap(),
        CorrelationId::new(1, 21).unwrap(),
    );

    let response = response_message(
        remote_id(),
        local_id(),
        MessageId::new(1, 22).unwrap(),
        &request,
        CorrelationId::new(1, 21).unwrap(),
    );

    assert!(response.is_response_to(&request));
    assert!(response.validate().is_ok());
}

#[test]
fn request_without_recipient_is_rejected() {
    let result = MessageEnvelope::new(MessageEnvelopeParts {
        protocol_version: ProtocolVersion::new(1, 0, 0),
        message_id: MessageId::new(1, 30).unwrap(),
        kind: MessageKind::new("zamani", "task_request").unwrap(),
        role: MessageRole::Request,
        sender: local_id(),
        recipient: None,
        correlation_id: Some(CorrelationId::new(1, 31).unwrap()),
        in_reply_to: None,
        operation_id: None,
        task_id: None,
        payload_format: "application/octet-stream".to_owned(),
        payload: Vec::new(),
        extensions: MessageExtensions::new(),
    });

    assert!(result.is_err());
}

// =============================================================================
// CODEC AND PROTOCOL COMPATIBILITY TESTS
// =============================================================================

#[test]
fn codec_round_trip_preserves_message_content() {
    let mut extensions = MessageExtensions::new();
    extensions.insert(
        "zamani.trace.context".to_owned(),
        vec![4, 5, 6],
    );

    let message = MessageEnvelope::new(MessageEnvelopeParts {
        protocol_version: ProtocolVersion::new(1, 2, 3),
        message_id: MessageId::new(1, 40).unwrap(),
        kind: MessageKind::new("zamani", "task_request").unwrap(),
        role: MessageRole::Request,
        sender: local_id(),
        recipient: Some(remote_id()),
        correlation_id: Some(CorrelationId::new(1, 41).unwrap()),
        in_reply_to: None,
        operation_id: None,
        task_id: None,
        payload_format: "application/octet-stream".to_owned(),
        payload: vec![0, 1, 2, 3, 255],
        extensions,
    })
    .unwrap();

    let codec = default_codec();
    let encoded = codec.encode(&message).unwrap();
    let decoded = codec.decode(&encoded).unwrap();

    assert_eq!(decoded, message);
}

#[test]
fn codec_rejects_malformed_wire_data() {
    let error = default_codec()
        .decode(b"{ definitely not valid JSON")
        .expect_err("malformed input must not be accepted");

    assert_eq!(
        error.kind(),
        TransportErrorKind::DeserializationFailure
    );
    assert_eq!(error.delivery(), DeliveryState::NotSent);
}

#[test]
fn exact_protocol_policy_rejects_different_minor_version() {
    let codec = codec(
        ProtocolVersion::new(1, 0, 0),
        ProtocolCompatibility::Exact,
        CodecLimits::new(),
    );

    let error = codec
        .validate_protocol_version(ProtocolVersion::new(1, 1, 0))
        .expect_err("exact compatibility rejects a different version");

    assert_eq!(
        error.kind(),
        TransportErrorKind::ProtocolIncompatible
    );
}

#[test]
fn same_major_policy_accepts_minor_and_patch_changes() {
    let codec = default_codec();

    assert!(
        codec
            .validate_protocol_version(ProtocolVersion::new(1, 9, 7))
            .is_ok()
    );
}

#[test]
fn same_major_policy_rejects_a_different_major_version() {
    let codec = default_codec();

    let error = codec
        .validate_protocol_version(ProtocolVersion::new(2, 0, 0))
        .expect_err("different major version must be rejected");

    assert_eq!(
        error.kind(),
        TransportErrorKind::ProtocolIncompatible
    );
}

#[test]
fn codec_enforces_configured_payload_budget() {
    let limits = CodecLimits::new().with_max_payload_bytes(
        NonZeroUsize::new(3),
    );

    let codec = codec(
        ProtocolVersion::new(1, 0, 0),
        ProtocolCompatibility::SameMajor,
        limits,
    );

    let request = request_message(
        local_id(),
        remote_id(),
        MessageId::new(1, 50).unwrap(),
        CorrelationId::new(1, 51).unwrap(),
    );

    let error = codec
        .encode(&request)
        .expect_err("payload exceeds configured budget");

    assert_eq!(error.kind(), TransportErrorKind::ResourceExhausted);
    assert_eq!(error.delivery(), DeliveryState::NotSent);
}

#[test]
fn codec_enforces_configured_wire_budget() {
    let limits = CodecLimits::new().with_max_wire_bytes(
        NonZeroUsize::new(1),
    );

    let codec = codec(
        ProtocolVersion::new(1, 0, 0),
        ProtocolCompatibility::SameMajor,
        limits,
    );

    let request = request_message(
        local_id(),
        remote_id(),
        MessageId::new(1, 60).unwrap(),
        CorrelationId::new(1, 61).unwrap(),
    );

    let error = codec
        .encode(&request)
        .expect_err("encoded message exceeds configured budget");

    assert_eq!(error.kind(), TransportErrorKind::ResourceExhausted);
}

#[test]
fn codec_enforces_extension_entry_budget() {
    let limits = CodecLimits::new().with_max_extension_entries(
        NonZeroUsize::new(1),
    );

    let codec = codec(
        ProtocolVersion::new(1, 0, 0),
        ProtocolCompatibility::SameMajor,
        limits,
    );

    let mut extensions = MessageExtensions::new();
    extensions.insert("vendor.first".to_owned(), vec![1]);
    extensions.insert("vendor.second".to_owned(), vec![2]);

    let request = MessageEnvelope::new(MessageEnvelopeParts {
        protocol_version: ProtocolVersion::new(1, 0, 0),
        message_id: MessageId::new(1, 70).unwrap(),
        kind: MessageKind::new("zamani", "task_request").unwrap(),
        role: MessageRole::Request,
        sender: local_id(),
        recipient: Some(remote_id()),
        correlation_id: Some(CorrelationId::new(1, 71).unwrap()),
        in_reply_to: None,
        operation_id: None,
        task_id: None,
        payload_format: "application/octet-stream".to_owned(),
        payload: Vec::new(),
        extensions,
    })
    .unwrap();

    let error = codec
        .encode(&request)
        .expect_err("extension entry budget must be enforced");

    assert_eq!(error.kind(), TransportErrorKind::ResourceExhausted);
}

#[test]
fn codec_accepts_larger_payload_when_no_budget_is_configured() {
    // This is a practical regression workload, not a universal size limit.
    // The codec must not invent a language-level ceiling.
    let payload = vec![0x5a; 512 * 1024];

    let request = MessageEnvelope::new(MessageEnvelopeParts {
        protocol_version: ProtocolVersion::new(1, 0, 0),
        message_id: MessageId::new(1, 80).unwrap(),
        kind: MessageKind::new("zamani", "task_request").unwrap(),
        role: MessageRole::Request,
        sender: local_id(),
        recipient: Some(remote_id()),
        correlation_id: Some(CorrelationId::new(1, 81).unwrap()),
        in_reply_to: None,
        operation_id: None,
        task_id: None,
        payload_format: "application/octet-stream".to_owned(),
        payload,
        extensions: MessageExtensions::new(),
    })
    .unwrap();

    let encoded = default_codec().encode(&request).unwrap();
    let decoded = default_codec().decode(&encoded).unwrap();

    assert_eq!(decoded.payload(), request.payload());
}

// =============================================================================
// TRANSPORT CONNECTION CONTRACT TESTS
// =============================================================================

#[test]
fn checked_send_accepts_matching_sender_and_peer() {
    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    );

    let message = request_message(
        local_id(),
        remote_id(),
        MessageId::new(1, 90).unwrap(),
        CorrelationId::new(1, 91).unwrap(),
    );

    let receipt = block_on(
        connection.send(
            &message,
            &TransportOperationContext::unrestricted(),
        ),
    )
    .unwrap();

    assert_eq!(receipt.message_id(), message.message_id());
    assert_eq!(
        receipt.confirmation(),
        DeliveryConfirmation::AcceptedLocally
    );

    // A local acceptance receipt does not claim remote execution.
    assert_eq!(connection.send_count(), 1);
}

#[test]
fn checked_send_rejects_a_forged_sender_before_dispatch() {
    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    );

    let forged = request_message(
        third_party_id(),
        remote_id(),
        MessageId::new(1, 100).unwrap(),
        CorrelationId::new(1, 101).unwrap(),
    );

    let error = block_on(
        connection.send(
            &forged,
            &TransportOperationContext::unrestricted(),
        ),
    )
    .expect_err("sender must match the local participant");

    assert_eq!(
        error.kind(),
        TransportErrorKind::AuthenticationFailure
    );
    assert_eq!(error.delivery(), DeliveryState::NotSent);
    assert_eq!(connection.send_count(), 0);
}

#[test]
fn checked_send_rejects_a_wrong_recipient_before_dispatch() {
    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    );

    let message = request_message(
        local_id(),
        third_party_id(),
        MessageId::new(1, 110).unwrap(),
        CorrelationId::new(1, 111).unwrap(),
    );

    let error = block_on(
        connection.send(
            &message,
            &TransportOperationContext::unrestricted(),
        ),
    )
    .expect_err("recipient must match authenticated peer");

    assert_eq!(error.kind(), TransportErrorKind::InvalidInput);
    assert_eq!(error.delivery(), DeliveryState::NotSent);
    assert_eq!(connection.send_count(), 0);
}

#[test]
fn checked_receive_accepts_authenticated_peer_message() {
    let incoming = request_message(
        remote_id(),
        local_id(),
        MessageId::new(1, 120).unwrap(),
        CorrelationId::new(1, 121).unwrap(),
    );

    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    )
    .with_incoming(incoming.clone());

    let received = block_on(
        connection.receive(&TransportOperationContext::unrestricted()),
    )
    .unwrap();

    assert_eq!(received, incoming);
}

#[test]
fn checked_receive_rejects_claimed_sender_mismatch() {
    let incoming = request_message(
        third_party_id(),
        local_id(),
        MessageId::new(1, 130).unwrap(),
        CorrelationId::new(1, 131).unwrap(),
    );

    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    )
    .with_incoming(incoming);

    let error = block_on(
        connection.receive(&TransportOperationContext::unrestricted()),
    )
    .expect_err("claimed sender must match authenticated peer");

    assert_eq!(
        error.kind(),
        TransportErrorKind::AuthenticationFailure
    );
    assert_eq!(error.delivery(), DeliveryState::Unknown);
}

#[test]
fn checked_request_response_accepts_correctly_correlated_response() {
    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    );

    let request = request_message(
        local_id(),
        remote_id(),
        MessageId::new(1, 140).unwrap(),
        CorrelationId::new(1, 141).unwrap(),
    );

    let response = block_on(
        connection.request_response(
            &request,
            &TransportOperationContext::unrestricted(),
        ),
    )
    .unwrap();

    assert!(response.is_response_to(&request));
    assert_eq!(response.sender(), remote_id());
    assert_eq!(response.recipient(), Some(local_id()));
}

#[test]
fn checked_request_response_rejects_wrong_correlation() {
    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::WrongCorrelation,
    );

    let request = request_message(
        local_id(),
        remote_id(),
        MessageId::new(1, 150).unwrap(),
        CorrelationId::new(1, 151).unwrap(),
    );

    let error = block_on(
        connection.request_response(
            &request,
            &TransportOperationContext::unrestricted(),
        ),
    )
    .expect_err("response must match original request and correlation");

    assert_eq!(error.kind(), TransportErrorKind::InvalidMessage);
    assert_eq!(error.delivery(), DeliveryState::Unknown);
}

#[test]
fn checked_request_response_rejects_wrong_authenticated_sender() {
    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::WrongSender,
    );

    let request = request_message(
        local_id(),
        remote_id(),
        MessageId::new(1, 160).unwrap(),
        CorrelationId::new(1, 161).unwrap(),
    );

    let error = block_on(
        connection.request_response(
            &request,
            &TransportOperationContext::unrestricted(),
        ),
    )
    .expect_err("response sender must match authenticated peer");

    assert_eq!(
        error.kind(),
        TransportErrorKind::AuthenticationFailure
    );
    assert_eq!(error.delivery(), DeliveryState::Unknown);
}

#[test]
fn cancellation_prevents_dispatch() {
    let token = CancellationToken::new();
    assert!(token.cancel("test cancellation"));

    let control = ExecutionControl::with_token(token);
    let context = TransportOperationContext::new(control);

    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    );

    let request = request_message(
        local_id(),
        remote_id(),
        MessageId::new(1, 170).unwrap(),
        CorrelationId::new(1, 171).unwrap(),
    );

    let error = block_on(connection.send(&request, &context))
        .expect_err("cancelled operation must not be dispatched");

    assert_eq!(error.kind(), TransportErrorKind::Cancelled);
    assert_eq!(error.delivery(), DeliveryState::NotSent);
    assert_eq!(connection.send_count(), 0);
}

#[test]
fn expired_deadline_prevents_dispatch() {
    let control = ExecutionControl::new()
        .with_deadline(std::time::Instant::now());

    let context = TransportOperationContext::new(control);

    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    );

    let request = request_message(
        local_id(),
        remote_id(),
        MessageId::new(1, 180).unwrap(),
        CorrelationId::new(1, 181).unwrap(),
    );

    let error = block_on(connection.send(&request, &context))
        .expect_err("expired operation must not be dispatched");

    assert_eq!(error.kind(), TransportErrorKind::Timeout);
    assert_eq!(error.delivery(), DeliveryState::NotSent);
    assert_eq!(connection.send_count(), 0);
}

// =============================================================================
// PROVIDER LIFECYCLE TESTS
// =============================================================================

#[test]
fn provider_connect_validates_requested_peer_identity() {
    let provider = InMemoryProvider::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    );

    let authentication = TestAuthentication::new(local_id());

    let connection = block_on(provider.connect(
        remote_id(),
        &authentication,
        &TransportOperationContext::unrestricted(),
    ))
    .unwrap();

    assert_eq!(connection.local_participant(), local_id());
    assert_eq!(
        connection.authenticated_peer().participant(),
        remote_id()
    );
    assert_eq!(provider.provider_id(), "zamani.test.in_memory");
}

#[test]
fn provider_rejects_unknown_peer_before_connection() {
    let provider = InMemoryProvider::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    );

    let authentication = TestAuthentication::new(local_id());

    let error = block_on(provider.connect(
        third_party_id(),
        &authentication,
        &TransportOperationContext::unrestricted(),
    ))
    .expect_err("provider must not connect an unknown peer");

    assert_eq!(error.kind(), TransportErrorKind::PeerUnavailable);
    assert_eq!(error.delivery(), DeliveryState::NotSent);
}

#[test]
fn connection_close_reports_local_completion_truthfully() {
    let connection = InMemoryConnection::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    );

    let report = block_on(connection.close(
        ShutdownMode::Graceful,
        &TransportOperationContext::unrestricted(),
    ))
    .unwrap();

    assert!(report.closed);
    assert_eq!(report.drained_operations, 0);
    assert_eq!(connection.state(), ConnectionState::Closed);
}

#[test]
fn provider_shutdown_reports_unresolved_work() {
    let provider = InMemoryProvider::new(
        local_id(),
        remote_id(),
        ResponseBehavior::Correct,
    );

    let report = block_on(provider.shutdown(
        ShutdownMode::Graceful,
        &TransportOperationContext::unrestricted(),
    ))
    .unwrap();

    assert!(report.completed);
    assert_eq!(report.closed_connections, 1);
    assert_eq!(report.unresolved_operations, 0);
    assert_eq!(
        provider.shutdown_count.load(Ordering::Acquire),
        1
    );
}

// =============================================================================
// IDENTITY ALLOCATION AND DYNAMIC WORKLOAD TESTS
// =============================================================================

#[test]
fn allocated_message_and_correlation_ids_are_distinct() {
    let allocator = IdAllocator::with_namespace(0xfeed).unwrap();

    let first = allocator.message_id().unwrap();
    let second = allocator.message_id().unwrap();
    let correlation = allocator.correlation_id().unwrap();

    assert_ne!(first, second);
    assert_ne!(first.to_string(), correlation.to_string());
}

#[test]
fn codec_handles_a_dynamically_generated_message_sequence() {
    // This exercises increasing message volumes without declaring a
    // maximum message count for the language or transport architecture.
    let allocator = IdAllocator::with_namespace(0xbeef).unwrap();
    let codec = default_codec();

    for _ in 0..256 {
        let message = request_message(
            local_id(),
            remote_id(),
            allocator.message_id().unwrap(),
            allocator.correlation_id().unwrap(),
        );

        let encoded = codec.encode(&message).unwrap();
        let decoded = codec.decode(&encoded).unwrap();

        assert_eq!(decoded, message);
    }
}
