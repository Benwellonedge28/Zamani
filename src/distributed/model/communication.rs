
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! Logical communication models for Zamani distributed computation.
//!
//! # Ownership
//!
//! This module owns target-independent descriptions of communication channels,
//! operation names, delivery expectations, ordering expectations, and logical
//! message intents.
//!
//! It does not own:
//!
//! - distributed identities, which belong to `crate::distributed::ids`;
//! - source syntax, AST nodes, name resolution, or semantic analysis;
//! - canonical values, types, effects, or provenance;
//! - task dependencies or dependency-graph validation;
//! - resource requirements or capability evidence;
//! - topology discovery, physical endpoints, or routing;
//! - wire formats, serialization protocols, sockets, or network I/O;
//! - task scheduling, execution, retries, or recovery;
//! - ZUIR, ZCIR, `quantum::ir`, HDLIR, or backend implementations.
//!
//! # Grammar and semantic authority
//!
//! Source-level communication syntax is owned by `grammar/distributed/communication.g4`
//! and composed by `grammar/distributed/distributed.g4`.
//! Normative distributed semantics are owned by `grammar/spec/distributed.md`.
//!
//! The grammar intentionally allows open-world qualified operation names.
//! This module preserves that extensibility. It does not decide whether a
//! named operation is defined, authorized, type-correct, or executable.
//!
//! # Portability
//!
//! A logical channel is not a socket, network interface, physical link,
//! process-local queue, or hardware-specific communication resource.
//! A logical message is not its wire encoding or proof of delivery.
//!
//! The same logical communication intent may be realized by an appropriate
//! local mechanism, inter-process transport, network protocol, accelerator
//! link, or domain-specific provider. The networking and execution layers
//! select and perform the actual realization.
//!
//! # Scalability
//!
//! This module imposes no application-defined universal limit on the number
//! of channels, recipients, messages, or operation names. Collections grow
//! according to available memory and any explicit budget enforced by a
//! higher-level component.
//!
//! Rust address spaces, integer representations, and available resources are
//! finite. This module does not claim literal infinite capacity.
//!
//! # Serialization boundary
//!
//! These domain objects do not derive `Serialize` or `Deserialize`.
//! In particular, deserializing a value must not silently bypass the
//! constructors' invariants. A future transport implementation should define
//! versioned wire DTOs, decode them under configured resource budgets, and
//! convert them into validated domain objects.
//!
//! # Delivery guarantees
//!
//! Delivery and ordering enums describe requested semantics, not guarantees
//! this module can itself fulfill. A transport or execution provider must
//! either establish the requested guarantee or return a structured failure.
//! An `AtLeastOnce` request permits duplicates; callers must not infer
//! idempotency or exactly-once side effects from it.

use core::fmt;
use std::collections::HashSet;
use std::error::Error;

use crate::distributed::ids::{
    ChannelId,
    CorrelationId,
    MessageId,
    ParticipantId,
};

/// A validated, open-world name for a logical communication channel.
///
/// A channel name is descriptive and scoped by its owning program or runtime.
/// It is not a physical address, transport name, globally unique identifier,
/// or authorization credential.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChannelName(String);

impl ChannelName {
    /// Construct a channel name.
    ///
    /// The value must not be empty, whitespace-only, or contain Unicode
    /// control characters. Its spelling is preserved exactly.
    ///
    /// # Errors
    ///
    /// Returns [`CommunicationValidationError::InvalidText`] if the value
    /// is invalid.
    pub fn new(value: impl Into<String>) -> Result<Self, CommunicationValidationError> {
        let value = value.into();
        validate_text(&value, TextField::ChannelName)?;
        Ok(Self(value))
    }

    /// Return the channel name as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the name and return its text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ChannelName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An open-world, qualified communication operation name.
///
/// Both components are validated text, not a closed enumeration of built-in
/// operations. Semantic analysis and provider registries determine whether
/// an operation is defined and supported.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommunicationOperation {
    namespace: String,
    name: String,
}

impl CommunicationOperation {
    /// Construct a qualified communication operation name.
    ///
    /// For example, a semantic adapter might construct the components
    /// `("distributed", "send")`. This constructor does not reserve that
    /// namespace or declare the operation to be valid.
    ///
    /// # Errors
    ///
    /// Returns [`CommunicationValidationError::InvalidText`] if either
    /// component is empty, whitespace-only, or contains Unicode control
    /// characters.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CommunicationValidationError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_text(&namespace, TextField::OperationNamespace)?;
        validate_text(&name, TextField::OperationName)?;

        Ok(Self { namespace, name })
    }

    /// Return the operation namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the qualified name using `namespace::name` notation.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

impl fmt::Display for CommunicationOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

/// Requested delivery behavior for a logical communication.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DeliverySemantics {
    /// Best-effort delivery. Loss may be permitted by the requested contract.
    BestEffort,

    /// The provider must avoid knowingly delivering the same logical message
    /// more than once, to the extent its documented contract permits.
    ///
    /// This does not prove that an application side effect occurred exactly
    /// once; durable execution semantics remain a separate concern.
    AtMostOnce,

    /// The provider may deliver a message more than once but must implement
    /// the requested retry/delivery behavior.
    ///
    /// Consumers must tolerate duplicates or provide their own idempotency
    /// mechanism when duplicate effects would be unsafe.
    AtLeastOnce,
}

/// Requested ordering behavior for messages.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum OrderingSemantics {
    /// No message ordering is requested.
    Unordered,

    /// Messages from the same logical sender must be observed in sender order,
    /// as defined by the selected provider's documented contract.
    PerSender,

    /// Messages on the channel must be observed in one consistent total order.
    ///
    /// This may be unavailable or expensive for a selected provider. The
    /// planner and transport must not silently weaken this requirement.
    Total,
}

/// A target-independent logical communication channel.
///
/// This type records intent only. It contains no endpoint, transport handle,
/// topology, buffer, resource reservation, or live connection state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommunicationChannel {
    id: ChannelId,
    name: ChannelName,
    operation: CommunicationOperation,
    delivery: DeliverySemantics,
    ordering: OrderingSemantics,
}

impl CommunicationChannel {
    /// Construct a logical communication channel.
    ///
    /// The caller supplies an ID allocated by the distributed identity
    /// authority. This constructor does not allocate IDs or register the
    /// channel with a runtime.
    #[must_use]
    pub fn new(
        id: ChannelId,
        name: ChannelName,
        operation: CommunicationOperation,
        delivery: DeliverySemantics,
        ordering: OrderingSemantics,
    ) -> Self {
        Self {
            id,
            name,
            operation,
            delivery,
            ordering,
        }
    }

    /// Return the stable logical channel identity.
    #[must_use]
    pub const fn id(&self) -> ChannelId {
        self.id
    }

    /// Return the channel's descriptive name.
    #[must_use]
    pub fn name(&self) -> &ChannelName {
        &self.name
    }

    /// Return the open-world operation associated with this channel.
    #[must_use]
    pub fn operation(&self) -> &CommunicationOperation {
        &self.operation
    }

    /// Return the requested delivery semantics.
    #[must_use]
    pub const fn delivery(&self) -> DeliverySemantics {
        self.delivery
    }

    /// Return the requested ordering semantics.
    #[must_use]
    pub const fn ordering(&self) -> OrderingSemantics {
        self.ordering
    }
}

/// A validated logical destination set for a message.
///
/// Recipient order is preserved as supplied. The collection rejects duplicate
/// logical participant IDs, because duplicates usually indicate an adapter
/// error rather than additional recipients.
///
/// Whether all recipients are reachable or authorized is decided downstream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommunicationRecipients {
    participants: Vec<ParticipantId>,
}

impl CommunicationRecipients {
    /// Construct a non-empty set of distinct logical recipients.
    ///
    /// An empty collection is rejected to prevent an accidental message with
    /// no destination. Broadcast-to-zero semantics, if required by the
    /// language, should be represented by a distinct operation contract
    /// rather than inferred from an empty ordinary recipient list.
    ///
    /// # Errors
    ///
    /// Returns [`CommunicationValidationError::NoRecipients`] for an empty
    /// collection or [`CommunicationValidationError::DuplicateRecipient`]
    /// if an ID occurs more than once.
    pub fn new(
        participants: impl IntoIterator<Item = ParticipantId>,
    ) -> Result<Self, CommunicationValidationError> {
        let mut result = Vec::new();
        let mut seen = HashSet::new();

        for participant in participants {
            if !seen.insert(participant) {
                return Err(CommunicationValidationError::DuplicateRecipient(
                    participant,
                ));
            }
            result.push(participant);
        }

        if result.is_empty() {
            return Err(CommunicationValidationError::NoRecipients);
        }

        Ok(Self {
            participants: result,
        })
    }

    /// Return the logical recipient IDs in their supplied order.
    #[must_use]
    pub fn participants(&self) -> &[ParticipantId] {
        &self.participants
    }

    /// Return the number of recipients represented by this value.
    #[must_use]
    pub fn len(&self) -> usize {
        self.participants.len()
    }

    /// Return whether this value contains no recipients.
    ///
    /// Values constructed successfully by [`Self::new`] are never empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.participants.is_empty()
    }

    /// Return whether the supplied participant is a recipient.
    #[must_use]
    pub fn contains(&self, participant: ParticipantId) -> bool {
        self.participants.contains(&participant)
    }
}

/// A logical message intent carrying a caller-defined payload.
///
/// `P` is deliberately generic: this module does not define a competing
/// universal value representation or assume that every domain value can be
/// serialized, copied, migrated, or transmitted. The canonical semantic and
/// domain layers determine which payload types are valid.
///
/// This object records a requested message. It does not represent a sent,
/// acknowledged, received, or durably committed message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommunicationMessage<P> {
    id: MessageId,
    channel: ChannelId,
    sender: ParticipantId,
    recipients: CommunicationRecipients,
    correlation: Option<CorrelationId>,
    payload: P,
}

impl<P> CommunicationMessage<P> {
    /// Construct a logical message intent.
    ///
    /// The caller supplies IDs allocated by the distributed identity
    /// authority. This constructor performs local structural validation only;
    /// it does not check membership, authentication, authorization, channel
    /// registration, payload type correctness, or transport feasibility.
    ///
    /// # Errors
    ///
    /// Returns [`CommunicationValidationError::SelfDeliveryForbidden`] only
    /// if a future semantic policy explicitly forbids self-delivery. This
    /// constructor does not forbid self-delivery: local communication is a
    /// valid possible realization.
    pub fn new(
        id: MessageId,
        channel: ChannelId,
        sender: ParticipantId,
        recipients: CommunicationRecipients,
        correlation: Option<CorrelationId>,
        payload: P,
    ) -> Result<Self, CommunicationValidationError> {
        // Recipient validation is performed by CommunicationRecipients.
        // Self-delivery is deliberately allowed for local and loopback cases.
        Ok(Self {
            id,
            channel,
            sender,
            recipients,
            correlation,
            payload,
        })
    }

    /// Return the logical message identity.
    #[must_use]
    pub const fn id(&self) -> MessageId {
        self.id
    }

    /// Return the logical channel identity.
    #[must_use]
    pub const fn channel(&self) -> ChannelId {
        self.channel
    }

    /// Return the logical sender identity.
    #[must_use]
    pub const fn sender(&self) -> ParticipantId {
        self.sender
    }

    /// Return the validated logical recipients.
    #[must_use]
    pub fn recipients(&self) -> &CommunicationRecipients {
        &self.recipients
    }

    /// Return the optional correlation identity.
    #[must_use]
    pub const fn correlation(&self) -> Option<CorrelationId> {
        self.correlation
    }

    /// Borrow the payload.
    #[must_use]
    pub const fn payload(&self) -> &P {
        &self.payload
    }

    /// Mutably borrow the payload.
    ///
    /// Mutation does not change the message ID or establish that the payload
    /// remains semantically valid. Callers must preserve their canonical
    /// payload invariants.
    #[must_use]
    pub fn payload_mut(&mut self) -> &mut P {
        &mut self.payload
    }

    /// Consume the message and return its payload.
    #[must_use]
    pub fn into_payload(self) -> P {
        self.payload
    }
}

/// Fields used to identify the source of a text-validation failure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CommunicationTextField {
    /// A logical channel name was invalid.
    ChannelName,

    /// The namespace of a qualified operation was invalid.
    OperationNamespace,

    /// The name of a qualified operation was invalid.
    OperationName,
}

impl fmt::Display for CommunicationTextField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ChannelName => "channel name",
            Self::OperationNamespace => "operation namespace",
            Self::OperationName => "operation name",
        })
    }
}

/// Errors returned when a logical communication descriptor is invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CommunicationValidationError {
    /// A text component was empty, whitespace-only, or contained a control
    /// character.
    InvalidText {
        /// The component that failed validation.
        field: CommunicationTextField,
    },

    /// A message was constructed without any logical recipient.
    NoRecipients,

    /// A recipient appeared more than once.
    DuplicateRecipient(ParticipantId),

    /// A semantic policy forbids a particular self-delivery operation.
    ///
    /// The core constructors currently permit self-delivery. This variant
    /// exists for policy adapters that need to report such a rejection
    /// without inventing a second error taxonomy.
    SelfDeliveryForbidden(ParticipantId),
}

impl fmt::Display for CommunicationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidText { field } => {
                write!(f, "invalid communication {}", field)
            }
            Self::NoRecipients => {
                f.write_str("a communication message requires at least one recipient")
            }
            Self::DuplicateRecipient(participant) => {
                write!(f, "duplicate communication recipient {participant}")
            }
            Self::SelfDeliveryForbidden(participant) => {
                write!(f, "self-delivery is forbidden for participant {participant}")
            }
        }
    }
}

impl Error for CommunicationValidationError {}

#[derive(Clone, Copy)]
enum TextField {
    ChannelName,
    OperationNamespace,
    OperationName,
}

impl From<TextField> for CommunicationTextField {
    fn from(value: TextField) -> Self {
        match value {
            TextField::ChannelName => Self::ChannelName,
            TextField::OperationNamespace => Self::OperationNamespace,
            TextField::OperationName => Self::OperationName,
        }
    }
}

fn validate_text(
    value: &str,
    field: TextField,
) -> Result<(), CommunicationValidationError> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(CommunicationValidationError::InvalidText {
            field: field.into(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn channel_id() -> ChannelId {
        ChannelId::new(17, 1).expect("non-zero ID components")
    }

    fn message_id() -> MessageId {
        MessageId::new(17, 2).expect("non-zero ID components")
    }

    fn participant_id(sequence: u128) -> ParticipantId {
        ParticipantId::new(17, sequence).expect("non-zero ID components")
    }

    fn operation() -> CommunicationOperation {
        CommunicationOperation::new("distributed", "send")
            .expect("valid operation name")
    }

    #[test]
    fn rejects_empty_or_control_character_text() {
        assert!(ChannelName::new("").is_err());
        assert!(ChannelName::new(" \t ").is_err());
        assert!(ChannelName::new("channel\nname").is_err());

        assert!(CommunicationOperation::new("", "send").is_err());
        assert!(CommunicationOperation::new("distributed", "\0").is_err());
    }

    #[test]
    fn preserves_open_world_operation_names() {
        let operation = CommunicationOperation::new(
            "future::transport",
            "specialized_send",
        )
        .expect("valid open-world operation");

        assert_eq!(operation.namespace(), "future::transport");
        assert_eq!(operation.name(), "specialized_send");
        assert_eq!(
            operation.qualified_name(),
            "future::transport::specialized_send"
        );
    }

    #[test]
    fn channel_keeps_logical_intent_separate_from_realization() {
        let channel = CommunicationChannel::new(
            channel_id(),
            ChannelName::new("results").expect("valid name"),
            operation(),
            DeliverySemantics::AtLeastOnce,
            OrderingSemantics::PerSender,
        );

        assert_eq!(channel.id(), channel_id());
        assert_eq!(channel.name().as_str(), "results");
        assert_eq!(channel.operation().qualified_name(), "distributed::send");
        assert_eq!(channel.delivery(), DeliverySemantics::AtLeastOnce);
        assert_eq!(channel.ordering(), OrderingSemantics::PerSender);
    }

    #[test]
    fn rejects_empty_recipient_list() {
        assert!(matches!(
            CommunicationRecipients::new([]),
            Err(CommunicationValidationError::NoRecipients)
        ));
    }

    #[test]
    fn rejects_duplicate_recipients() {
        let participant = participant_id(1);

        assert!(matches!(
            CommunicationRecipients::new([participant, participant]),
            Err(CommunicationValidationError::DuplicateRecipient(id))
                if id == participant
        ));
    }

    #[test]
    fn permits_multiple_distinct_recipients() {
        let first = participant_id(1);
        let second = participant_id(2);
        let recipients = CommunicationRecipients::new([first, second])
            .expect("distinct recipients");

        assert_eq!(recipients.len(), 2);
        assert!(recipients.contains(first));
        assert!(recipients.contains(second));
    }

    #[test]
    fn message_preserves_payload_and_correlation() {
        let sender = participant_id(1);
        let recipient = participant_id(2);
        let correlation =
            CorrelationId::new(17, 3).expect("non-zero ID components");

        let message = CommunicationMessage::new(
            message_id(),
            channel_id(),
            sender,
            CommunicationRecipients::new([recipient]).expect("recipient"),
            Some(correlation),
            vec![1_u8, 2, 3],
        )
        .expect("valid message intent");

        assert_eq!(message.id(), message_id());
        assert_eq!(message.channel(), channel_id());
        assert_eq!(message.sender(), sender);
        assert_eq!(message.recipients().participants(), &[recipient]);
        assert_eq!(message.correlation(), Some(correlation));
        assert_eq!(message.payload(), &vec![1, 2, 3]);
        assert_eq!(message.into_payload(), vec![1, 2, 3]);
    }

    #[test]
    fn permits_self_delivery_for_local_realizations() {
        let participant = participant_id(1);
        let message = CommunicationMessage::new(
            message_id(),
            channel_id(),
            participant,
            CommunicationRecipients::new([participant]).expect("recipient"),
            None,
            "local payload",
        );

        assert!(message.is_ok());
    }

    #[test]
    fn delivery_semantics_do_not_claim_execution_guarantees() {
        let channel = CommunicationChannel::new(
            channel_id(),
            ChannelName::new("events").expect("valid name"),
            operation(),
            DeliverySemantics::AtMostOnce,
            OrderingSemantics::Unordered,
        );

        assert_eq!(channel.delivery(), DeliverySemantics::AtMostOnce);
        assert_eq!(channel.ordering(), OrderingSemantics::Unordered);
    }
}
