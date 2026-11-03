
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Agent Communication Contracts
//!
//! Path: `src/ai/agents/communication.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines portable, declarative communication contracts for AI agents.
//!
//! This module describes:
//!
//! - Logical communication endpoints.
//! - Logical communication channels.
//! - Logical communication streams.
//! - Message contracts.
//! - Message invocation descriptions.
//! - Message delivery guarantees.
//! - Message ordering guarantees.
//! - Communication profile validation.
//! - Communication profile composition.
//!
//! It describes communication intent, not physical communication.
//!
//! ## Canonical ownership
//!
//! - `src/ai/ids.rs`
//!   Owns AI agent identities.
//!
//! - `src/ai/agents/definition.rs`
//!   Owns agent definitions and agent interfaces.
//!
//! - `src/ai/agents/capabilities.rs`
//!   Owns agent capability declarations.
//!
//! - `grammar/ai/communication.g4`
//!   Owns communication source syntax.
//!
//! - `grammar/ai/agents.g4`
//!   Owns agent source syntax.
//!
//! - `grammar/ai/multi-agent.g4`
//!   Owns multi-agent composition syntax.
//!
//! - `src/frontend/semantic/`
//!   Owns source resolution, canonical types, effects, policies,
//!   security, capabilities and semantic validation.
//!
//! - `src/distributed/`
//!   Owns distributed communication, transport, routing and
//!   distributed execution.
//!
//! - `src/runtime/`
//!   Owns actual execution and runtime lifecycle.
//!
//! ## Ownership boundaries
//!
//! This module owns declarative agent communication contracts.
//!
//! It does not own:
//!
//! - Parsing or AST construction.
//! - Universal type definitions.
//! - Physical network endpoints.
//! - Network protocols.
//! - Message serialization.
//! - Compression or encryption.
//! - Authentication or authorization.
//! - Physical message queues.
//! - Transport buffers.
//! - Network routing.
//! - Distributed membership.
//! - Message delivery implementation.
//! - Agent scheduling.
//! - Agent execution.
//! - Hardware selection.
//! - Physical resource allocation.
//! - Universal communication IR.
//!
//! ## POCO-REAF
//!
//! Communication contracts are independent of:
//!
//! - CPU architecture.
//! - GPU architecture.
//! - Accelerator vendor.
//! - FPGA or ASIC implementation.
//! - Quantum processor.
//! - Neuromorphic architecture.
//! - Operating system.
//! - Process topology.
//! - Network topology.
//! - Distributed node count.
//! - Physical communication medium.
//!
//! The same logical communication contract can be considered for
//! local, embedded, multicore, heterogeneous, distributed, quantum-
//! assisted and future execution environments.
//!
//! This does not guarantee that every environment can realize every
//! communication requirement.
//!
//! Unsupported or infeasible requirements must be reported explicitly.
//!
//! ## Scalability
//!
//! This module imposes no artificial universal maximum on:
//!
//! - Agents.
//! - Endpoints.
//! - Channels.
//! - Streams.
//! - Message contracts.
//! - Message arguments.
//! - Communication relationships.
//! - Participants.
//! - Communication declarations.
//!
//! Collections grow according to actual memory availability and
//! explicitly configured operational budgets.
//!
//! Literal infinite physical capacity is not promised.
//!
//! ## Dependency direction
//!
//! ```text
//! Canonical grammar
//!       |
//!       v
//! Canonical frontend AST
//!       |
//!       v
//! Semantic analysis
//!       |
//!       v
//! Agent communication contracts
//!       |
//!       v
//! Capability and policy validation
//!       |
//!       v
//! Canonical IR
//!       |
//!       v
//! Distributed planning
//!       |
//!       v
//! Transport and runtime
//! ```
//!
//! This module must not depend on transport or runtime implementations.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

use crate::ai::ids::AiAgentId;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for agent communication operations.
pub type AgentCommunicationResult<T> =
    Result<T, AgentCommunicationError>;

// ============================================================================
// COMMUNICATION NAME
// ============================================================================

/// A validated, open-world logical communication name.
///
/// Names identify logical declarations such as endpoints, channels,
/// streams and message contracts.
///
/// A communication name is not a physical network address.
///
/// The canonical frontend remains responsible for resolving source
/// identifiers and checking namespace visibility.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommunicationName(String);

impl CommunicationName {
    /// Construct a communication name.
    ///
    /// Names must not be empty, have surrounding whitespace or contain
    /// control characters.
    ///
    /// The complete Zamani identifier grammar remains owned by the
    /// canonical frontend.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentCommunicationResult<Self> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(
                AgentCommunicationError::InvalidName(value),
            );
        }

        Ok(Self(value))
    }

    /// Return the name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the name and return its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for CommunicationName {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// OPAQUE TYPE REFERENCES
// ============================================================================

/// An opaque reference to a canonical Zamani semantic type.
///
/// This is deliberately not a new type system.
///
/// The frontend adapter supplies a stable representation of a type
/// already resolved or identified by the canonical semantic layer.
///
/// This module does not parse, resolve or validate the meaning of
/// the referenced type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommunicationTypeRef(String);

impl CommunicationTypeRef {
    /// Construct an opaque semantic type reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentCommunicationResult<Self> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(
                AgentCommunicationError::InvalidTypeReference(
                    value,
                ),
            );
        }

        Ok(Self(value))
    }

    /// Return the opaque type reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommunicationTypeRef {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// COMMUNICATION REFERENCES
// ============================================================================

/// An opaque reference to a logical communication target.
///
/// The target may represent an agent, endpoint, channel, service,
/// actor, stream or another semantic communication participant.
///
/// It is not a physical address or network route.
///
/// Resolution belongs to frontend semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommunicationReference(String);

impl CommunicationReference {
    /// Construct a logical communication reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentCommunicationResult<Self> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(
                AgentCommunicationError::InvalidReference(
                    value,
                ),
            );
        }

        Ok(Self(value))
    }

    /// Return the unresolved logical reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommunicationReference {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// VALUE REFERENCES
// ============================================================================

/// An opaque reference to a canonical expression or value.
///
/// The frontend adapter supplies this reference from the canonical AST
/// or semantic representation.
///
/// This module does not define another expression or value hierarchy.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommunicationValueRef(String);

impl CommunicationValueRef {
    /// Construct a reference to a canonical expression or value.
    pub fn new(
        value: impl Into<String>,
    ) -> AgentCommunicationResult<Self> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(
                AgentCommunicationError::InvalidValueReference(
                    value,
                ),
            );
        }

        Ok(Self(value))
    }

    /// Return the opaque canonical value reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// ENDPOINT DIRECTION
// ============================================================================

/// Logical direction of an agent communication endpoint.
///
/// This describes the contract, not a physical socket direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EndpointDirection {
    /// The agent declares an endpoint through which it receives
    /// communication.
    Inbound,

    /// The agent declares an endpoint through which it initiates
    /// communication.
    Outbound,

    /// The endpoint supports both directions.
    Bidirectional,
}

// ============================================================================
// DELIVERY GUARANTEES
// ============================================================================

/// Declarative message delivery guarantee.
///
/// A guarantee is a requirement or desired semantic property.
///
/// It does not prove that a particular transport can provide it.
///
/// Exactly-once delivery, for example, may require transactional,
/// deduplication or recovery mechanisms supplied downstream.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeliveryGuarantee {
    /// Delivery behavior is not specified by this contract.
    Unspecified,

    /// Best-effort delivery.
    BestEffort,

    /// A message is delivered zero or one times.
    AtMostOnce,

    /// A message may be delivered more than once but should not
    /// be silently lost when the guarantee is successfully realized.
    AtLeastOnce,

    /// The requested semantic guarantee is exactly-once processing
    /// or delivery, as defined by the relevant communication contract.
    ///
    /// Realizability must be checked by the execution infrastructure.
    ExactlyOnce,

    /// An extensible, registered delivery semantic.
    Custom(CommunicationName),
}

// ============================================================================
// ORDERING GUARANTEES
// ============================================================================

/// Declarative message ordering guarantee.
///
/// Ordering requirements must be interpreted in the scope declared
/// by the communication contract and validated downstream.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrderingGuarantee {
    /// No ordering guarantee is requested.
    Unspecified,

    /// Preserve ordering within one sender's message sequence.
    PerSender,

    /// Preserve ordering within one logical channel.
    PerChannel,

    /// Request one total ordering across the declared communication scope.
    Total,

    /// An extensible, registered ordering semantic.
    Custom(CommunicationName),
}

// ============================================================================
// MESSAGE CONTRACT
// ============================================================================

/// Declarative contract describing a class of messages.
///
/// A message contract describes the logical payload type and requested
/// delivery and ordering properties.
///
/// It does not contain a serialized message or a physical transport
/// representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentMessageContract {
    name: CommunicationName,
    payload_type: Option<CommunicationTypeRef>,
    delivery: DeliveryGuarantee,
    ordering: OrderingGuarantee,
}

impl AgentMessageContract {
    /// Construct a message contract.
    ///
    /// `payload_type` may be absent when the source declaration does
    /// not provide an explicit type. Semantic analysis must then
    /// establish whether the inferred or dynamically resolved payload
    /// is valid.
    #[must_use]
    pub fn new(
        name: CommunicationName,
        payload_type: Option<CommunicationTypeRef>,
    ) -> Self {
        Self {
            name,
            payload_type,
            delivery: DeliveryGuarantee::Unspecified,
            ordering: OrderingGuarantee::Unspecified,
        }
    }

    /// Set the requested delivery guarantee.
    #[must_use]
    pub fn with_delivery(
        mut self,
        delivery: DeliveryGuarantee,
    ) -> Self {
        self.delivery = delivery;
        self
    }

    /// Set the requested ordering guarantee.
    #[must_use]
    pub fn with_ordering(
        mut self,
        ordering: OrderingGuarantee,
    ) -> Self {
        self.ordering = ordering;
        self
    }

    /// Return the message contract name.
    #[must_use]
    pub fn name(&self) -> &CommunicationName {
        &self.name
    }

    /// Return the optional payload type reference.
    #[must_use]
    pub fn payload_type(
        &self,
    ) -> Option<&CommunicationTypeRef> {
        self.payload_type.as_ref()
    }

    /// Return the delivery guarantee.
    #[must_use]
    pub fn delivery(&self) -> &DeliveryGuarantee {
        &self.delivery
    }

    /// Return the ordering guarantee.
    #[must_use]
    pub fn ordering(&self) -> &OrderingGuarantee {
        &self.ordering
    }
}

// ============================================================================
// LOGICAL ENDPOINT
// ============================================================================

/// Declarative logical endpoint belonging to an agent.
///
/// An endpoint is a semantic communication boundary.
///
/// It is not a network address, socket, physical queue, device port
/// or transport connection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentCommunicationEndpoint {
    name: CommunicationName,
    direction: EndpointDirection,
    message_type: Option<CommunicationTypeRef>,
}

impl AgentCommunicationEndpoint {
    /// Construct a logical communication endpoint.
    #[must_use]
    pub fn new(
        name: CommunicationName,
        direction: EndpointDirection,
        message_type: Option<CommunicationTypeRef>,
    ) -> Self {
        Self {
            name,
            direction,
            message_type,
        }
    }

    /// Return the endpoint name.
    #[must_use]
    pub fn name(&self) -> &CommunicationName {
        &self.name
    }

    /// Return the endpoint direction.
    #[must_use]
    pub const fn direction(&self) -> EndpointDirection {
        self.direction
    }

    /// Return the optional canonical message type reference.
    #[must_use]
    pub fn message_type(
        &self,
    ) -> Option<&CommunicationTypeRef> {
        self.message_type.as_ref()
    }
}

// ============================================================================
// LOGICAL CHANNEL
// ============================================================================

/// Declarative logical channel belonging to an agent.
///
/// The channel describes a communication relationship.
///
/// Buffering, queue implementation, backpressure, transport and
/// physical realization are downstream concerns.
///
/// No channel capacity is hard-coded here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentCommunicationChannel {
    name: CommunicationName,
    message_type: Option<CommunicationTypeRef>,
    delivery: DeliveryGuarantee,
    ordering: OrderingGuarantee,
}

impl AgentCommunicationChannel {
    /// Construct a logical communication channel.
    #[must_use]
    pub fn new(
        name: CommunicationName,
        message_type: Option<CommunicationTypeRef>,
    ) -> Self {
        Self {
            name,
            message_type,
            delivery: DeliveryGuarantee::Unspecified,
            ordering: OrderingGuarantee::Unspecified,
        }
    }

    /// Set the requested delivery guarantee.
    #[must_use]
    pub fn with_delivery(
        mut self,
        delivery: DeliveryGuarantee,
    ) -> Self {
        self.delivery = delivery;
        self
    }

    /// Set the requested ordering guarantee.
    #[must_use]
    pub fn with_ordering(
        mut self,
        ordering: OrderingGuarantee,
    ) -> Self {
        self.ordering = ordering;
        self
    }

    /// Return the channel name.
    #[must_use]
    pub fn name(&self) -> &CommunicationName {
        &self.name
    }

    /// Return the optional message type.
    #[must_use]
    pub fn message_type(
        &self,
    ) -> Option<&CommunicationTypeRef> {
        self.message_type.as_ref()
    }

    /// Return the requested delivery guarantee.
    #[must_use]
    pub fn delivery(&self) -> &DeliveryGuarantee {
        &self.delivery
    }

    /// Return the requested ordering guarantee.
    #[must_use]
    pub fn ordering(&self) -> &OrderingGuarantee {
        &self.ordering
    }
}

// ============================================================================
// LOGICAL STREAM
// ============================================================================

/// Declarative logical communication stream.
///
/// A stream describes a sequence-oriented communication abstraction.
///
/// Its ordering, flow control, buffering, persistence, backpressure,
/// transport and execution are determined by downstream semantic
/// validation and runtime realization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentCommunicationStream {
    name: CommunicationName,
    item_type: Option<CommunicationTypeRef>,
    ordering: OrderingGuarantee,
}

impl AgentCommunicationStream {
    /// Construct a logical communication stream.
    #[must_use]
    pub fn new(
        name: CommunicationName,
        item_type: Option<CommunicationTypeRef>,
    ) -> Self {
        Self {
            name,
            item_type,
            ordering: OrderingGuarantee::Unspecified,
        }
    }

    /// Set the requested ordering guarantee.
    #[must_use]
    pub fn with_ordering(
        mut self,
        ordering: OrderingGuarantee,
    ) -> Self {
        self.ordering = ordering;
        self
    }

    /// Return the stream name.
    #[must_use]
    pub fn name(&self) -> &CommunicationName {
        &self.name
    }

    /// Return the optional stream item type.
    #[must_use]
    pub fn item_type(
        &self,
    ) -> Option<&CommunicationTypeRef> {
        self.item_type.as_ref()
    }

    /// Return the requested ordering guarantee.
    #[must_use]
    pub fn ordering(&self) -> &OrderingGuarantee {
        &self.ordering
    }
}

// ============================================================================
// MESSAGE INVOCATION
// ============================================================================

/// Declarative description of a message invocation.
///
/// The sender is the agent whose communication profile is being used.
///
/// The target and arguments are opaque references to canonical frontend
/// expressions or semantic values.
///
/// This structure does not send, receive, serialize or deliver anything.
///
/// Argument ordering is preserved exactly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentMessageInvocation {
    sender: AiAgentId,
    target: CommunicationReference,
    arguments: Vec<CommunicationValueRef>,
}

impl AgentMessageInvocation {
    /// Construct a message invocation description.
    ///
    /// The caller supplies the sender identity and the target and
    /// arguments resolved or represented by the frontend adapter.
    #[must_use]
    pub fn new(
        sender: AiAgentId,
        target: CommunicationReference,
        arguments: Vec<CommunicationValueRef>,
    ) -> Self {
        Self {
            sender,
            target,
            arguments,
        }
    }

    /// Return the sending agent identity.
    #[must_use]
    pub const fn sender(&self) -> AiAgentId {
        self.sender
    }

    /// Return the logical target reference.
    #[must_use]
    pub fn target(&self) -> &CommunicationReference {
        &self.target
    }

    /// Return the ordered argument references.
    #[must_use]
    pub fn arguments(&self) -> &[CommunicationValueRef] {
        &self.arguments
    }
}

// ============================================================================
// AGENT COMMUNICATION PROFILE
// ============================================================================

/// Declarative communication profile belonging to one AI agent.
///
/// The profile aggregates:
///
/// - Logical endpoints.
/// - Logical channels.
/// - Logical streams.
/// - Message contracts.
///
/// It is independent of the number of agents or physical execution
/// resources available.
///
/// The profile does not implement communication.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentCommunicationProfile {
    agent_id: AiAgentId,
    endpoints: Vec<AgentCommunicationEndpoint>,
    channels: Vec<AgentCommunicationChannel>,
    streams: Vec<AgentCommunicationStream>,
    messages: Vec<AgentMessageContract>,
}

impl AgentCommunicationProfile {
    /// Construct an empty communication profile.
    #[must_use]
    pub fn new(agent_id: AiAgentId) -> Self {
        Self {
            agent_id,
            endpoints: Vec::new(),
            channels: Vec::new(),
            streams: Vec::new(),
            messages: Vec::new(),
        }
    }

    /// Return the owning agent identity.
    #[must_use]
    pub const fn agent_id(&self) -> AiAgentId {
        self.agent_id
    }

    /// Return all logical endpoints.
    #[must_use]
    pub fn endpoints(
        &self,
    ) -> &[AgentCommunicationEndpoint] {
        &self.endpoints
    }

    /// Return all logical channels.
    #[must_use]
    pub fn channels(
        &self,
    ) -> &[AgentCommunicationChannel] {
        &self.channels
    }

    /// Return all logical streams.
    #[must_use]
    pub fn streams(
        &self,
    ) -> &[AgentCommunicationStream] {
        &self.streams
    }

    /// Return all message contracts.
    #[must_use]
    pub fn messages(
        &self,
    ) -> &[AgentMessageContract] {
        &self.messages
    }

    /// Return whether the profile contains no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.endpoints.is_empty()
            && self.channels.is_empty()
            && self.streams.is_empty()
            && self.messages.is_empty()
    }

    // ========================================================================
    // ENDPOINT REGISTRATION
    // ========================================================================

    /// Add a logical endpoint.
    ///
    /// Duplicate endpoint names within this profile are rejected.
    pub fn add_endpoint(
        &mut self,
        endpoint: AgentCommunicationEndpoint,
    ) -> AgentCommunicationResult<()> {
        let name = endpoint.name().as_str();

        if self.endpoints.iter().any(
            |existing| existing.name().as_str() == name,
        ) {
            return Err(
                AgentCommunicationError::DuplicateEndpoint(
                    name.to_owned(),
                ),
            );
        }

        self.endpoints.push(endpoint);
        Ok(())
    }

    /// Find an endpoint by its logical name.
    #[must_use]
    pub fn find_endpoint(
        &self,
        name: &CommunicationName,
    ) -> Option<&AgentCommunicationEndpoint> {
        self.endpoints
            .iter()
            .find(|endpoint| endpoint.name() == name)
    }

    /// Remove an endpoint by its logical name.
    pub fn remove_endpoint(
        &mut self,
        name: &CommunicationName,
    ) -> Option<AgentCommunicationEndpoint> {
        let index = self.endpoints
            .iter()
            .position(|endpoint| endpoint.name() == name)?;

        Some(self.endpoints.remove(index))
    }

    // ========================================================================
    // CHANNEL REGISTRATION
    // ========================================================================

    /// Add a logical channel.
    ///
    /// Duplicate channel names within this profile are rejected.
    pub fn add_channel(
        &mut self,
        channel: AgentCommunicationChannel,
    ) -> AgentCommunicationResult<()> {
        let name = channel.name().as_str();

        if self.channels.iter().any(
            |existing| existing.name().as_str() == name,
        ) {
            return Err(
                AgentCommunicationError::DuplicateChannel(
                    name.to_owned(),
                ),
            );
        }

        self.channels.push(channel);
        Ok(())
    }

    /// Find a channel by its logical name.
    #[must_use]
    pub fn find_channel(
        &self,
        name: &CommunicationName,
    ) -> Option<&AgentCommunicationChannel> {
        self.channels
            .iter()
            .find(|channel| channel.name() == name)
    }

    /// Remove a channel by its logical name.
    pub fn remove_channel(
        &mut self,
        name: &CommunicationName,
    ) -> Option<AgentCommunicationChannel> {
        let index = self.channels
            .iter()
            .position(|channel| channel.name() == name)?;

        Some(self.channels.remove(index))
    }

    // ========================================================================
    // STREAM REGISTRATION
    // ========================================================================

    /// Add a logical stream.
    ///
    /// Duplicate stream names within this profile are rejected.
    pub fn add_stream(
        &mut self,
        stream: AgentCommunicationStream,
    ) -> AgentCommunicationResult<()> {
        let name = stream.name().as_str();

        if self.streams.iter().any(
            |existing| existing.name().as_str() == name,
        ) {
            return Err(
                AgentCommunicationError::DuplicateStream(
                    name.to_owned(),
                ),
            );
        }

        self.streams.push(stream);
        Ok(())
    }

    /// Find a stream by its logical name.
    #[must_use]
    pub fn find_stream(
        &self,
        name: &CommunicationName,
    ) -> Option<&AgentCommunicationStream> {
        self.streams
            .iter()
            .find(|stream| stream.name() == name)
    }

    /// Remove a stream by its logical name.
    pub fn remove_stream(
        &mut self,
        name: &CommunicationName,
    ) -> Option<AgentCommunicationStream> {
        let index = self.streams
            .iter()
            .position(|stream| stream.name() == name)?;

        Some(self.streams.remove(index))
    }

    // ========================================================================
    // MESSAGE CONTRACT REGISTRATION
    // ========================================================================

    /// Add a message contract.
    ///
    /// Duplicate message contract names within this profile are rejected.
    pub fn add_message_contract(
        &mut self,
        message: AgentMessageContract,
    ) -> AgentCommunicationResult<()> {
        let name = message.name().as_str();

        if self.messages.iter().any(
            |existing| existing.name().as_str() == name,
        ) {
            return Err(
                AgentCommunicationError::DuplicateMessageContract(
                    name.to_owned(),
                ),
            );
        }

        self.messages.push(message);
        Ok(())
    }

    /// Find a message contract by its logical name.
    #[must_use]
    pub fn find_message_contract(
        &self,
        name: &CommunicationName,
    ) -> Option<&AgentMessageContract> {
        self.messages
            .iter()
            .find(|message| message.name() == name)
    }

    /// Remove a message contract by its logical name.
    pub fn remove_message_contract(
        &mut self,
        name: &CommunicationName,
    ) -> Option<AgentMessageContract> {
        let index = self.messages
            .iter()
            .position(|message| message.name() == name)?;

        Some(self.messages.remove(index))
    }

    // ========================================================================
    // VALIDATION
    // ========================================================================

    /// Validate local structural invariants.
    ///
    /// This method verifies:
    ///
    /// - No duplicate endpoint names.
    /// - No duplicate channel names.
    /// - No duplicate stream names.
    /// - No duplicate message contract names.
    /// - All stored names are structurally valid.
    ///
    /// It does not resolve source references, verify type compatibility,
    /// establish capability availability, authorize communication,
    /// or prove transport feasibility.
    ///
    /// Those checks belong to the canonical frontend and downstream
    /// execution infrastructure.
    pub fn validate(&self) -> AgentCommunicationResult<()> {
        validate_unique_names(
            self.endpoints.iter().map(
                |item| item.name().as_str(),
            ),
            "endpoint",
        )?;

        validate_unique_names(
            self.channels.iter().map(
                |item| item.name().as_str(),
            ),
            "channel",
        )?;

        validate_unique_names(
            self.streams.iter().map(
                |item| item.name().as_str(),
            ),
            "stream",
        )?;

        validate_unique_names(
            self.messages.iter().map(
                |item| item.name().as_str(),
            ),
            "message contract",
        )?;

        Ok(())
    }

    // ========================================================================
    // PROFILE COMPOSITION
    // ========================================================================

    /// Merge declarations from another profile belonging to the same agent.
    ///
    /// The operation is atomic with respect to validation errors:
    /// the receiver remains unchanged if any incoming declaration
    /// conflicts with an existing declaration.
    ///
    /// The method does not merge profiles belonging to different agents.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> AgentCommunicationResult<()> {
        if self.agent_id != other.agent_id {
            return Err(
                AgentCommunicationError::AgentMismatch {
                    expected: self.agent_id,
                    actual: other.agent_id,
                },
            );
        }

        let mut merged = self.clone();

        for endpoint in &other.endpoints {
            merged.add_endpoint(endpoint.clone())?;
        }

        for channel in &other.channels {
            merged.add_channel(channel.clone())?;
        }

        for stream in &other.streams {
            merged.add_stream(stream.clone())?;
        }

        for message in &other.messages {
            merged.add_message_contract(message.clone())?;
        }

        merged.validate()?;

        *self = merged;

        Ok(())
    }
}

// ============================================================================
// INTERNAL VALIDATION
// ============================================================================

fn validate_unique_names<'a>(
    names: impl IntoIterator<Item = &'a str>,
    category: &'static str,
) -> AgentCommunicationResult<()> {
    let mut seen = BTreeSet::new();

    for name in names {
        if !seen.insert(name) {
            return Err(
                AgentCommunicationError::DuplicateDeclaration {
                    category,
                    name: name.to_owned(),
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by agent communication contract operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AgentCommunicationError {
    /// A logical communication name is invalid.
    InvalidName(String),

    /// A semantic type reference is invalid.
    InvalidTypeReference(String),

    /// A logical communication reference is invalid.
    InvalidReference(String),

    /// A canonical expression/value reference is invalid.
    InvalidValueReference(String),

    /// An endpoint name is already declared.
    DuplicateEndpoint(String),

    /// A channel name is already declared.
    DuplicateChannel(String),

    /// A stream name is already declared.
    DuplicateStream(String),

    /// A message contract name is already declared.
    DuplicateMessageContract(String),

    /// A generic duplicate declaration was detected.
    DuplicateDeclaration {
        /// Kind of declaration.
        category: &'static str,

        /// Duplicate declaration name.
        name: String,
    },

    /// The profiles belong to different agents.
    AgentMismatch {
        /// Identity expected by the receiving profile.
        expected: AiAgentId,

        /// Identity supplied by the incoming profile.
        actual: AiAgentId,
    },
}

impl fmt::Display for AgentCommunicationError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidName(value) => {
                write!(
                    f,
                    "invalid communication name: {value:?}"
                )
            }

            Self::InvalidTypeReference(value) => {
                write!(
                    f,
                    "invalid communication type reference: {value:?}"
                )
            }

            Self::InvalidReference(value) => {
                write!(
                    f,
                    "invalid communication reference: {value:?}"
                )
            }

            Self::InvalidValueReference(value) => {
                write!(
                    f,
                    "invalid communication value reference: {value:?}"
                )
            }

            Self::DuplicateEndpoint(name) => {
                write!(
                    f,
                    "duplicate communication endpoint: {name}"
                )
            }

            Self::DuplicateChannel(name) => {
                write!(
                    f,
                    "duplicate communication channel: {name}"
                )
            }

            Self::DuplicateStream(name) => {
                write!(
                    f,
                    "duplicate communication stream: {name}"
                )
            }

            Self::DuplicateMessageContract(name) => {
                write!(
                    f,
                    "duplicate message contract: {name}"
                )
            }

            Self::DuplicateDeclaration {
                category,
                name,
            } => {
                write!(
                    f,
                    "duplicate {category} declaration: {name}"
                )
            }

            Self::AgentMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "communication profile agent mismatch: \
                     expected {expected}, received {actual}"
                )
            }
        }
    }
}

impl Error for AgentCommunicationError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn agent_id(value: u128) -> AiAgentId {
        AiAgentId::new(value)
            .expect("nonzero agent identity")
    }

    fn name(value: &str) -> CommunicationName {
        CommunicationName::new(value)
            .expect("valid communication name")
    }

    #[test]
    fn creates_empty_profile() {
        let profile = AgentCommunicationProfile::new(
            agent_id(1),
        );

        assert_eq!(profile.agent_id(), agent_id(1));
        assert!(profile.is_empty());
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn rejects_empty_communication_names() {
        assert!(
            CommunicationName::new("").is_err()
        );
    }

    #[test]
    fn rejects_surrounding_whitespace() {
        assert!(
            CommunicationName::new(" channel ").is_err()
        );
    }

    #[test]
    fn accepts_open_world_names() {
        let value = name(
            "future.quantum.agent.communication",
        );

        assert_eq!(
            value.as_str(),
            "future.quantum.agent.communication"
        );
    }

    #[test]
    fn registers_endpoint() {
        let mut profile =
            AgentCommunicationProfile::new(agent_id(1));

        let endpoint = AgentCommunicationEndpoint::new(
            name("inbox"),
            EndpointDirection::Inbound,
            None,
        );

        assert!(
            profile.add_endpoint(endpoint).is_ok()
        );

        assert_eq!(profile.endpoints().len(), 1);
    }

    #[test]
    fn rejects_duplicate_endpoints() {
        let mut profile =
            AgentCommunicationProfile::new(agent_id(1));

        let first = AgentCommunicationEndpoint::new(
            name("inbox"),
            EndpointDirection::Inbound,
            None,
        );

        let second = AgentCommunicationEndpoint::new(
            name("inbox"),
            EndpointDirection::Bidirectional,
            None,
        );

        assert!(profile.add_endpoint(first).is_ok());

        assert!(matches!(
            profile.add_endpoint(second),
            Err(AgentCommunicationError::DuplicateEndpoint(_))
        ));
    }

    #[test]
    fn registers_channels_and_streams() {
        let mut profile =
            AgentCommunicationProfile::new(agent_id(1));

        assert!(
            profile.add_channel(
                AgentCommunicationChannel::new(
                    name("research"),
                    None,
                )
            ).is_ok()
        );

        assert!(
            profile.add_stream(
                AgentCommunicationStream::new(
                    name("observations"),
                    None,
                )
            ).is_ok()
        );

        assert_eq!(profile.channels().len(), 1);
        assert_eq!(profile.streams().len(), 1);
    }

    #[test]
    fn registers_message_contracts() {
        let mut profile =
            AgentCommunicationProfile::new(agent_id(1));

        let message = AgentMessageContract::new(
            name("research_result"),
            Some(
                CommunicationTypeRef::new(
                    "zamani::ResearchResult",
                )
                .expect("valid type reference"),
            ),
        )
        .with_delivery(DeliveryGuarantee::AtLeastOnce)
        .with_ordering(OrderingGuarantee::PerChannel);

        assert!(
            profile.add_message_contract(message).is_ok()
        );

        assert_eq!(profile.messages().len(), 1);
    }

    #[test]
    fn rejects_duplicate_message_contracts() {
        let mut profile =
            AgentCommunicationProfile::new(agent_id(1));

        let first = AgentMessageContract::new(
            name("result"),
            None,
        );

        let second = AgentMessageContract::new(
            name("result"),
            None,
        );

        assert!(
            profile.add_message_contract(first).is_ok()
        );

        assert!(matches!(
            profile.add_message_contract(second),
            Err(
                AgentCommunicationError::DuplicateMessageContract(_)
            )
        ));
    }

    #[test]
    fn preserves_message_argument_order() {
        let invocation = AgentMessageInvocation::new(
            agent_id(1),
            CommunicationReference::new("research_agent")
                .expect("valid target"),
            vec![
                CommunicationValueRef::new("value_a")
                    .expect("valid reference"),
                CommunicationValueRef::new("value_b")
                    .expect("valid reference"),
            ],
        );

        assert_eq!(invocation.arguments().len(), 2);

        assert_eq!(
            invocation.arguments()[0].as_str(),
            "value_a"
        );

        assert_eq!(
            invocation.arguments()[1].as_str(),
            "value_b"
        );
    }

    #[test]
    fn rejects_merge_between_different_agents() {
        let mut first =
            AgentCommunicationProfile::new(agent_id(1));

        let second =
            AgentCommunicationProfile::new(agent_id(2));

        assert!(matches!(
            first.merge(&second),
            Err(AgentCommunicationError::AgentMismatch { .. })
        ));

        assert!(first.is_empty());
    }

    #[test]
    fn merge_is_atomic_on_duplicate_declarations() {
        let mut first =
            AgentCommunicationProfile::new(agent_id(1));

        let mut second =
            AgentCommunicationProfile::new(agent_id(1));

        first.add_channel(
            AgentCommunicationChannel::new(
                name("shared"),
                None,
            ),
        ).expect("first channel");

        second.add_channel(
            AgentCommunicationChannel::new(
                name("shared"),
                None,
            ),
        ).expect("second channel");

        assert!(first.merge(&second).is_err());

        assert_eq!(first.channels().len(), 1);
    }

    #[test]
    fn merges_profiles_for_the_same_agent() {
        let mut first =
            AgentCommunicationProfile::new(agent_id(1));

        let mut second =
            AgentCommunicationProfile::new(agent_id(1));

        first.add_endpoint(
            AgentCommunicationEndpoint::new(
                name("inbox"),
                EndpointDirection::Inbound,
                None,
            ),
        ).expect("endpoint");

        second.add_channel(
            AgentCommunicationChannel::new(
                name("events"),
                None,
            ),
        ).expect("channel");

        assert!(first.merge(&second).is_ok());

        assert_eq!(first.endpoints().len(), 1);
        assert_eq!(first.channels().len(), 1);
    }

    #[test]
    fn removes_endpoint_by_name() {
        let mut profile =
            AgentCommunicationProfile::new(agent_id(1));

        profile.add_endpoint(
            AgentCommunicationEndpoint::new(
                name("inbox"),
                EndpointDirection::Inbound,
                None,
            ),
        ).expect("endpoint");

        assert!(
            profile.remove_endpoint(&name("inbox")).is_some()
        );

        assert!(profile.endpoints().is_empty());
    }

    #[test]
    fn supports_custom_delivery_and_ordering_semantics() {
        let message = AgentMessageContract::new(
            name("custom"),
            None,
        )
        .with_delivery(
            DeliveryGuarantee::Custom(
                name("future_delivery_semantics"),
            ),
        )
        .with_ordering(
            OrderingGuarantee::Custom(
                name("future_ordering_semantics"),
            ),
        );

        assert!(matches!(
            message.delivery(),
            DeliveryGuarantee::Custom(_)
        ));

        assert!(matches!(
            message.ordering(),
            OrderingGuarantee::Custom(_)
        ));
    }
}
