
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Versioned distributed transport messages for Zamani.
//!
//! Path: `src/distributed/transport/messages.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Ownership
//!
//! This module owns the transport-level envelope used to carry an already
//! defined logical message between distributed participants.
//!
//! It owns:
//!
//! - protocol version representation;
//! - open-world message kind identifiers;
//! - request, response, event, and control roles;
//! - message envelope structure;
//! - correlation and reply relationships;
//! - task and operation associations;
//! - opaque payload bytes and payload-format identifiers;
//! - opaque, namespaced extension metadata;
//! - validation of envelope invariants;
//! - the untrusted wire representation used by the codec.
//!
//! It does not own:
//!
//! - source-level message schemas;
//! - frontend AST or semantic message types;
//! - canonical values, types, effects, or provenance;
//! - logical communication channels or delivery requirements;
//! - distributed identity allocation;
//! - physical endpoints, routing, or topology;
//! - serialization format implementation;
//! - network I/O or transport selection;
//! - authentication, authorization, encryption, or key management;
//! - task execution, retries, deduplication, or recovery;
//! - ZUIR, ZCIR, quantum::ir, HDLIR, or backend representations.
//!
//! # Authoritative repository boundaries
//!
//! - `crate::distributed::ids` owns distributed logical identifiers.
//! - `crate::distributed::error` owns canonical distributed errors.
//! - `crate::distributed::model::communication` owns logical communication
//!   intent and delivery requirements.
//! - `grammar/networking/messages.g4` owns source-level message schemas.
//! - `grammar/distributed/messages.g4` adapts those schemas into distributed
//!   source constructs.
//! - `grammar/spec/distributed.md` owns normative distributed semantics.
//! - `transport/codec.rs` owns byte encoding, decoding, and decoding budgets.
//! - `transport/traits.rs` owns injectable transport operations.
//! - security modules own peer authentication and authorization.
//!
//! # Trust boundary
//!
//! A successfully decoded wire DTO is still untrusted input.
//!
//! The codec MUST:
//!
//! 1. enforce its configured byte and allocation budgets;
//! 2. decode the versioned wire representation;
//! 3. reject unsupported protocol versions;
//! 4. convert the wire representation into a validated envelope;
//! 5. pass the validated envelope to the transport/runtime boundary.
//!
//! Deserialization MUST NOT be treated as proof of authentication,
//! authorization, delivery, execution, or semantic validity.
//!
//! # Delivery and execution
//!
//! An envelope represents a message to be communicated. It does not prove
//! that the message was sent, delivered, acknowledged, executed, or durably
//! recorded.
//!
//! A response correlation ID does not establish exactly-once execution.
//! Operation deduplication and side-effect guarantees belong to the
//! execution and resilience authorities.
//!
//! # Security
//!
//! `sender` is a claimed logical participant identity. The receiving
//! transport MUST bind that claim to an authenticated peer identity and
//! enforce authorization before accepting an operation.
//!
//! Authentication credentials, private keys, bearer tokens, and other
//! secrets MUST NOT be placed in this envelope.
//!
//! # Scalability
//!
//! This module defines no universal maximum for messages, participants,
//! tasks, payloads, metadata entries, or message kinds.
//!
//! Actual allocation is limited by the process address space and available
//! resources. Configurable message and decoding budgets belong to the codec
//! and runtime configuration.
//!
//! # Compatibility
//!
//! Protocol versions are explicit. A matching major version identifies a
//! protocol compatibility line; it does not, by itself, prove that a
//! particular message kind, payload format, extension, or capability is
//! supported.
//!
//! The codec and transport handshake MUST apply the repository's negotiated
//! compatibility policy before accepting messages.
//!
//! # Safety
//!
//! This module uses safe Rust only.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::distributed::error::{DistributedError, DistributedResult};
use crate::distributed::ids::{
    CorrelationId,
    MessageId,
    OperationId,
    ParticipantId,
    TaskId,
};

/// Construct a structured invalid-message error.
///
/// The subject is a stable field or message-category identifier. The
/// explanation is diagnostic text and MUST NOT be parsed for control flow.
fn invalid_message(
    subject: impl Into<String>,
    message: impl Into<String>,
) -> DistributedError {
    DistributedError::InvalidMessage {
        subject: subject.into(),
        message: message.into(),
    }
}

/// Validate text used as a protocol identifier.
///
/// Identifiers must be non-empty, must not consist only of whitespace, and
/// must not contain Unicode control characters.
///
/// No fixed byte-length limit is imposed here. The codec is responsible for
/// enforcing configured wire and allocation budgets before constructing
/// these values.
fn validate_identifier(
    subject: &'static str,
    value: &str,
) -> DistributedResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_message(
            subject,
            "identifier must not be empty or whitespace-only",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_message(
            subject,
            "identifier must not contain control characters",
        ));
    }

    Ok(())
}

/// Version of the Zamani distributed transport protocol.
///
/// The version identifies the envelope/protocol contract. It does not
/// identify a particular payload schema or application message type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProtocolVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl ProtocolVersion {
    /// Construct a protocol version.
    ///
    /// Version components are represented by `u32`. This is a
    /// representation choice, not a machine-size or cluster-size limit.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major version.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Return the minor version.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Return the patch version.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Return whether two versions belong to the same major compatibility
    /// line.
    ///
    /// This is only a preliminary compatibility test. A codec or handshake
    /// MUST additionally check supported features, message kinds, schema
    /// versions, and negotiated requirements.
    #[must_use]
    pub const fn shares_major_line(self, other: Self) -> bool {
        self.major == other.major
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

/// An open-world, namespaced message-kind identifier.
///
/// The namespace and name are ordinary validated identifiers. They are not
/// a closed enumeration of every operation Zamani or an extension provider
/// may introduce.
///
/// Examples include:
///
/// - `zamani::task_request`
/// - `zamani::task_result`
/// - `zamani::heartbeat`
/// - `vendor::specialized_exchange`
/// - `future_domain::message_transfer`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageKind {
    namespace: String,
    name: String,
}

impl MessageKind {
    /// Construct an open-world message kind.
    ///
    /// Whether the receiving implementation supports this kind is decided
    /// by the protocol registry and receiving provider, not by this
    /// constructor.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> DistributedResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier("message_kind.namespace", &namespace)?;
        validate_identifier("message_kind.name", &name)?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the message-kind namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the message-kind name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the fully qualified message-kind name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }

    /// Validate this value again after deserialization.
    ///
    /// Serde deserialization does not call [`Self::new`], so all values
    /// received from an external wire representation MUST be validated.
    pub fn validate(&self) -> DistributedResult<()> {
        validate_identifier("message_kind.namespace", &self.namespace)?;
        validate_identifier("message_kind.name", &self.name)
    }
}

impl fmt::Display for MessageKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

/// Structural role of a transport message.
///
/// This describes the interaction shape, not the application operation.
/// Application operation names remain open-world in [`MessageKind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageRole {
    /// A request that expects a correlated response.
    Request,

    /// A response to a previously identified request.
    Response,

    /// An event that does not represent a request-response exchange.
    Event,

    /// A protocol-control message, such as a negotiated control operation.
    Control,
}

/// Namespaced opaque extension metadata.
///
/// Extension values are uninterpreted bytes. Their meaning is owned by the
/// extension or subsystem that defines the corresponding key.
///
/// Keys SHOULD be qualified, for example:
///
/// - `zamani.security.policy_reference`
/// - `zamani.tracing.context`
/// - `vendor.extension_name`
pub type MessageExtensions = BTreeMap<String, Vec<u8>>;

/// Input fields used to construct a validated [`MessageEnvelope`].
///
/// The fields are public to make construction explicit and auditable.
/// Constructing this input does not itself create a validated envelope;
/// [`MessageEnvelope::new`] validates all structural invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageEnvelopeParts {
    /// Protocol version used to interpret the envelope.
    pub protocol_version: ProtocolVersion,

    /// Unique logical message identity.
    pub message_id: MessageId,

    /// Open-world application/protocol message kind.
    pub kind: MessageKind,

    /// Structural interaction role.
    pub role: MessageRole,

    /// Claimed logical sender.
    pub sender: ParticipantId,

    /// Logical recipient, if this message is addressed to one participant.
    ///
    /// Group or broadcast routing is resolved by the communication and
    /// routing layers; it is not encoded as a physical endpoint here.
    pub recipient: Option<ParticipantId>,

    /// Request-response correlation identity.
    pub correlation_id: Option<CorrelationId>,

    /// Message identity of the request to which this message replies.
    pub in_reply_to: Option<MessageId>,

    /// Optional operation identity used by execution/idempotency authorities.
    ///
    /// Its presence does not establish deduplication or exactly-once
    /// execution.
    pub operation_id: Option<OperationId>,

    /// Optional logical task association.
    pub task_id: Option<TaskId>,

    /// Open-world identifier for the payload representation.
    ///
    /// Examples include `application/json`, `application/cbor`, or a
    /// provider-defined schema identifier.
    pub payload_format: String,

    /// Opaque payload bytes.
    ///
    /// This module does not interpret or serialize semantic values.
    pub payload: Vec<u8>,

    /// Opaque namespaced extension metadata.
    pub extensions: MessageExtensions,
}

/// A validated transport message envelope.
///
/// This is a transport representation of a logical message. It is not proof
/// of sending, delivery, acknowledgement, execution, or durable completion.
///
/// The type intentionally implements `Serialize` but not `Deserialize`.
/// External bytes must first be decoded into [`WireMessageEnvelope`] and
/// converted through its validation path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MessageEnvelope {
    protocol_version: ProtocolVersion,
    message_id: MessageId,
    kind: MessageKind,
    role: MessageRole,
    sender: ParticipantId,
    recipient: Option<ParticipantId>,
    correlation_id: Option<CorrelationId>,
    in_reply_to: Option<MessageId>,
    operation_id: Option<OperationId>,
    task_id: Option<TaskId>,
    payload_format: String,
    payload: Vec<u8>,
    extensions: MessageExtensions,
}

impl MessageEnvelope {
    /// Construct and validate a transport message envelope.
    ///
    /// Structural rules:
    ///
    /// - A request MUST have a recipient and correlation ID.
    /// - A request MUST NOT have `in_reply_to`.
    /// - A response MUST have a recipient, correlation ID, and `in_reply_to`.
    /// - An event MUST NOT have `in_reply_to`.
    /// - Control messages may carry correlation and reply information.
    ///
    /// These rules describe the transport interaction shape. They do not
    /// decide whether an operation is semantically valid or authorized.
    pub fn new(parts: MessageEnvelopeParts) -> DistributedResult<Self> {
        let envelope = Self {
            protocol_version: parts.protocol_version,
            message_id: parts.message_id,
            kind: parts.kind,
            role: parts.role,
            sender: parts.sender,
            recipient: parts.recipient,
            correlation_id: parts.correlation_id,
            in_reply_to: parts.in_reply_to,
            operation_id: parts.operation_id,
            task_id: parts.task_id,
            payload_format: parts.payload_format,
            payload: parts.payload,
            extensions: parts.extensions,
        };

        envelope.validate()?;
        Ok(envelope)
    }

    /// Validate every envelope invariant.
    ///
    /// This method is also called after decoding untrusted wire data.
    pub fn validate(&self) -> DistributedResult<()> {
        self.kind.validate()?;

        validate_identifier(
            "message.payload_format",
            &self.payload_format,
        )?;

        for key in self.extensions.keys() {
            validate_identifier("message.extension_key", key)?;
        }

        match self.role {
            MessageRole::Request => {
                if self.recipient.is_none() {
                    return Err(invalid_message(
                        "message.recipient",
                        "request messages require a logical recipient",
                    ));
                }

                if self.correlation_id.is_none() {
                    return Err(invalid_message(
                        "message.correlation_id",
                        "request messages require a correlation ID",
                    ));
                }

                if self.in_reply_to.is_some() {
                    return Err(invalid_message(
                        "message.in_reply_to",
                        "request messages cannot be replies",
                    ));
                }
            }

            MessageRole::Response => {
                if self.recipient.is_none() {
                    return Err(invalid_message(
                        "message.recipient",
                        "response messages require a logical recipient",
                    ));
                }

                if self.correlation_id.is_none() {
                    return Err(invalid_message(
                        "message.correlation_id",
                        "response messages require a correlation ID",
                    ));
                }

                if self.in_reply_to.is_none() {
                    return Err(invalid_message(
                        "message.in_reply_to",
                        "response messages must identify the request they answer",
                    ));
                }
            }

            MessageRole::Event => {
                if self.in_reply_to.is_some() {
                    return Err(invalid_message(
                        "message.in_reply_to",
                        "event messages cannot be request replies",
                    ));
                }
            }

            MessageRole::Control => {}
        }

        Ok(())
    }

    /// Return the protocol version.
    #[must_use]
    pub const fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }

    /// Return the logical message identity.
    #[must_use]
    pub const fn message_id(&self) -> MessageId {
        self.message_id
    }

    /// Return the message kind.
    #[must_use]
    pub fn kind(&self) -> &MessageKind {
        &self.kind
    }

    /// Return the structural interaction role.
    #[must_use]
    pub const fn role(&self) -> MessageRole {
        self.role
    }

    /// Return the claimed logical sender.
    #[must_use]
    pub const fn sender(&self) -> ParticipantId {
        self.sender
    }

    /// Return the logical recipient, if one is specified.
    #[must_use]
    pub const fn recipient(&self) -> Option<ParticipantId> {
        self.recipient
    }

    /// Return the request-response correlation identity.
    #[must_use]
    pub const fn correlation_id(&self) -> Option<CorrelationId> {
        self.correlation_id
    }

    /// Return the request message identity to which this message replies.
    #[must_use]
    pub const fn in_reply_to(&self) -> Option<MessageId> {
        self.in_reply_to
    }

    /// Return the optional idempotency/execution operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> Option<OperationId> {
        self.operation_id
    }

    /// Return the optional logical task association.
    #[must_use]
    pub const fn task_id(&self) -> Option<TaskId> {
        self.task_id
    }

    /// Return the payload-format identifier.
    #[must_use]
    pub fn payload_format(&self) -> &str {
        &self.payload_format
    }

    /// Borrow the opaque payload bytes.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Borrow the extension metadata.
    #[must_use]
    pub fn extensions(&self) -> &MessageExtensions {
        &self.extensions
    }

    /// Consume the envelope and return its opaque payload bytes.
    #[must_use]
    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }

    /// Return whether this envelope is a response to the supplied request.
    ///
    /// Both the request message identity and correlation identity must match.
    /// This establishes a structural relationship only; it does not prove
    /// that the response is authentic or that the request was executed.
    #[must_use]
    pub fn is_response_to(&self, request: &Self) -> bool {
        self.role == MessageRole::Response
            && request.role == MessageRole::Request
            && self.in_reply_to == Some(request.message_id)
            && self.correlation_id.is_some()
            && self.correlation_id == request.correlation_id
            && self.sender == request.recipient.unwrap_or(self.sender)
            && self.recipient == Some(request.sender)
    }

    /// Convert the validated envelope into the codec's wire DTO.
    ///
    /// The payload and extension values are cloned. Callers handling large
    /// payloads should avoid unnecessary conversions and let the codec
    /// serialize the envelope directly where possible.
    #[must_use]
    pub fn to_wire(&self) -> WireMessageEnvelope {
        WireMessageEnvelope {
            protocol_version: self.protocol_version,
            message_id: self.message_id,
            kind: self.kind.clone(),
            role: self.role,
            sender: self.sender,
            recipient: self.recipient,
            correlation_id: self.correlation_id,
            in_reply_to: self.in_reply_to,
            operation_id: self.operation_id,
            task_id: self.task_id,
            payload_format: self.payload_format.clone(),
            payload: self.payload.clone(),
            extensions: self.extensions.clone(),
        }
    }
}

/// Untrusted wire representation of a distributed message.
///
/// This DTO is the deserialization target for `transport/codec.rs`.
///
/// A decoded DTO MUST NOT be passed to execution or application code until
/// [`Self::into_envelope`] has successfully validated it.
///
/// The fields are private to prevent ordinary callers from treating an
/// unvalidated DTO as a valid message. Serde can still populate the fields
/// when decoding external bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireMessageEnvelope {
    protocol_version: ProtocolVersion,
    message_id: MessageId,
    kind: MessageKind,
    role: MessageRole,
    sender: ParticipantId,
    recipient: Option<ParticipantId>,
    correlation_id: Option<CorrelationId>,
    in_reply_to: Option<MessageId>,
    operation_id: Option<OperationId>,
    task_id: Option<TaskId>,
    payload_format: String,
    payload: Vec<u8>,
    #[serde(default)]
    extensions: MessageExtensions,
}

impl WireMessageEnvelope {
    /// Validate and convert untrusted wire data into a message envelope.
    ///
    /// This conversion checks structural invariants. The caller must still
    /// perform protocol compatibility, authentication, authorization,
    /// payload-schema validation, and resource-policy checks.
    pub fn into_envelope(self) -> DistributedResult<MessageEnvelope> {
        MessageEnvelope::new(MessageEnvelopeParts {
            protocol_version: self.protocol_version,
            message_id: self.message_id,
            kind: self.kind,
            role: self.role,
            sender: self.sender,
            recipient: self.recipient,
            correlation_id: self.correlation_id,
            in_reply_to: self.in_reply_to,
            operation_id: self.operation_id,
            task_id: self.task_id,
            payload_format: self.payload_format,
            payload: self.payload,
            extensions: self.extensions,
        })
    }

    /// Return the protocol version carried by the untrusted wire value.
    ///
    /// This accessor is useful for a codec's preliminary compatibility check.
    /// The DTO remains untrusted until full validation succeeds.
    #[must_use]
    pub const fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }
}

impl TryFrom<WireMessageEnvelope> for MessageEnvelope {
    type Error = DistributedError;

    fn try_from(value: WireMessageEnvelope) -> Result<Self, Self::Error> {
        value.into_envelope()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids() -> (
        MessageId,
        MessageId,
        CorrelationId,
        ParticipantId,
        ParticipantId,
        OperationId,
        TaskId,
    ) {
        (
            MessageId::new(1, 1).unwrap(),
            MessageId::new(1, 2).unwrap(),
            CorrelationId::new(1, 3).unwrap(),
            ParticipantId::new(1, 4).unwrap(),
            ParticipantId::new(1, 5).unwrap(),
            OperationId::new(1, 6).unwrap(),
            TaskId::new(1, 7).unwrap(),
        )
    }

    fn request() -> MessageEnvelope {
        let (message_id, _, correlation_id, sender, recipient, operation_id, task_id) =
            ids();

        MessageEnvelope::new(MessageEnvelopeParts {
            protocol_version: ProtocolVersion::new(1, 0, 0),
            message_id,
            kind: MessageKind::new("zamani", "task_request").unwrap(),
            role: MessageRole::Request,
            sender,
            recipient: Some(recipient),
            correlation_id: Some(correlation_id),
            in_reply_to: None,
            operation_id: Some(operation_id),
            task_id: Some(task_id),
            payload_format: "application/json".to_owned(),
            payload: br#"{"task":"example"}"#.to_vec(),
            extensions: MessageExtensions::new(),
        })
        .unwrap()
    }

    fn response(request: &MessageEnvelope) -> MessageEnvelope {
        let (_, response_id, correlation_id, sender, recipient, _, _) = ids();

        MessageEnvelope::new(MessageEnvelopeParts {
            protocol_version: ProtocolVersion::new(1, 0, 0),
            message_id: response_id,
            kind: MessageKind::new("zamani", "task_result").unwrap(),
            role: MessageRole::Response,
            sender,
            recipient: Some(recipient),
            correlation_id: Some(correlation_id),
            in_reply_to: Some(request.message_id()),
            operation_id: request.operation_id(),
            task_id: request.task_id(),
            payload_format: "application/json".to_owned(),
            payload: br#"{"completed":true}"#.to_vec(),
            extensions: MessageExtensions::new(),
        })
        .unwrap()
    }

    #[test]
    fn constructs_a_valid_request() {
        let message = request();

        assert_eq!(message.role(), MessageRole::Request);
        assert!(message.recipient().is_some());
        assert!(message.correlation_id().is_some());
        assert!(message.in_reply_to().is_none());
        assert_eq!(message.payload(), br#"{"task":"example"}"#);
    }

    #[test]
    fn rejects_request_without_recipient() {
        let (message_id, _, correlation_id, sender, _, operation_id, task_id) =
            ids();

        let result = MessageEnvelope::new(MessageEnvelopeParts {
            protocol_version: ProtocolVersion::new(1, 0, 0),
            message_id,
            kind: MessageKind::new("zamani", "task_request").unwrap(),
            role: MessageRole::Request,
            sender,
            recipient: None,
            correlation_id: Some(correlation_id),
            in_reply_to: None,
            operation_id: Some(operation_id),
            task_id: Some(task_id),
            payload_format: "application/json".to_owned(),
            payload: Vec::new(),
            extensions: MessageExtensions::new(),
        });

        assert!(result.is_err());
    }

    #[test]
    fn rejects_request_without_correlation() {
        let (message_id, _, _, sender, recipient, operation_id, task_id) =
            ids();

        let result = MessageEnvelope::new(MessageEnvelopeParts {
            protocol_version: ProtocolVersion::new(1, 0, 0),
            message_id,
            kind: MessageKind::new("zamani", "task_request").unwrap(),
            role: MessageRole::Request,
            sender,
            recipient: Some(recipient),
            correlation_id: None,
            in_reply_to: None,
            operation_id: Some(operation_id),
            task_id: Some(task_id),
            payload_format: "application/json".to_owned(),
            payload: Vec::new(),
            extensions: MessageExtensions::new(),
        });

        assert!(result.is_err());
    }

    #[test]
    fn rejects_response_without_request_identity() {
        let (message_id, _, correlation_id, sender, recipient, operation_id, task_id) =
            ids();

        let result = MessageEnvelope::new(MessageEnvelopeParts {
            protocol_version: ProtocolVersion::new(1, 0, 0),
            message_id,
            kind: MessageKind::new("zamani", "task_result").unwrap(),
            role: MessageRole::Response,
            sender,
            recipient: Some(recipient),
            correlation_id: Some(correlation_id),
            in_reply_to: None,
            operation_id: Some(operation_id),
            task_id: Some(task_id),
            payload_format: "application/json".to_owned(),
            payload: Vec::new(),
            extensions: MessageExtensions::new(),
        });

        assert!(result.is_err());
    }

    #[test]
    fn response_matches_request_correlation_and_identity() {
        let request = request();
        let response = response(&request);

        assert!(response.is_response_to(&request));
        assert!(!request.is_response_to(&response));
    }

    #[test]
    fn preserves_open_world_message_kinds() {
        let kind = MessageKind::new(
            "future_computational_domain",
            "specialized_transfer",
        )
        .unwrap();

        assert_eq!(
            kind.qualified_name(),
            "future_computational_domain::specialized_transfer"
        );
    }

    #[test]
    fn rejects_invalid_message_kind() {
        assert!(MessageKind::new("", "task_request").is_err());
        assert!(MessageKind::new("zamani", " ").is_err());
        assert!(MessageKind::new("zamani\n", "task_request").is_err());
    }

    #[test]
    fn rejects_invalid_payload_format() {
        let mut message = request().to_wire();
        message.payload_format = "  ".to_owned();

        assert!(message.into_envelope().is_err());
    }

    #[test]
    fn rejects_invalid_extension_key() {
        let mut message = request().to_wire();
        message.extensions.insert(
            "invalid\nkey".to_owned(),
            vec![1, 2, 3],
        );

        assert!(message.into_envelope().is_err());
    }

    #[test]
    fn wire_round_trip_preserves_envelope() {
        let original = request();
        let wire = original.to_wire();

        let encoded = serde_json::to_vec(&wire).unwrap();
        let decoded: WireMessageEnvelope =
            serde_json::from_slice(&encoded).unwrap();

        let reconstructed = decoded.into_envelope().unwrap();

        assert_eq!(original, reconstructed);
    }

    #[test]
    fn wire_decoding_does_not_bypass_validation() {
        let mut wire = request().to_wire();
        wire.correlation_id = None;

        let encoded = serde_json::to_vec(&wire).unwrap();
        let decoded: WireMessageEnvelope =
            serde_json::from_slice(&encoded).unwrap();

        assert!(decoded.into_envelope().is_err());
    }

    #[test]
    fn protocol_versions_expose_major_compatibility_line() {
        let first = ProtocolVersion::new(1, 0, 0);
        let compatible_line = ProtocolVersion::new(1, 9, 4);
        let incompatible_line = ProtocolVersion::new(2, 0, 0);

        assert!(first.shares_major_line(compatible_line));
        assert!(!first.shares_major_line(incompatible_line));
    }

    #[test]
    fn extensions_are_preserved_deterministically() {
        let mut wire = request().to_wire();

        wire.extensions.insert(
            "vendor.trace_context".to_owned(),
            vec![1, 2, 3, 4],
        );

        let reconstructed = wire.into_envelope().unwrap();

        assert_eq!(
            reconstructed.extensions().get("vendor.trace_context"),
            Some(&vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn message_payload_remains_opaque() {
        let mut wire = request().to_wire();
        wire.payload = vec![0, 255, 1, 128, 42];

        let reconstructed = wire.into_envelope().unwrap();

        assert_eq!(
            reconstructed.payload(),
            &[0, 255, 1, 128, 42]
        );
    }
}
