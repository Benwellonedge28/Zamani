
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Networking Transports
//!
//! Path: `src/stdlib/networking/transports.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines portable, extensible contracts for communication transports.
//!
//! This module describes how a networking provider exposes communication
//! capabilities. It does not implement operating-system sockets, network
//! drivers, physical links, routing, scheduling, or hardware access.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Transport identities.
//! - Communication-mode identities.
//! - Transport capability descriptions.
//! - Transport requests and options.
//! - Stream connection contracts.
//! - Stream listener contracts.
//! - Datagram transport contracts.
//! - Transport provider interfaces.
//! - Transport lifecycle contracts.
//! - Transport-specific structured errors.
//!
//! This module does not own:
//!
//! - Address declarations or address resolution.
//! - Endpoint declarations.
//! - Protocol declarations.
//! - Message schemas.
//! - Serialization.
//! - Cryptography.
//! - Authentication or authorization implementation.
//! - DNS.
//! - Socket creation.
//! - Routing.
//! - Network topology.
//! - Scheduling.
//! - Physical resource allocation.
//! - Runtime execution.
//! - Hardware discovery or control.
//!
//! ## Repository integration
//!
//! Address authority:
//!     crate::stdlib::networking::addresses
//!
//! Protocol grammar:
//!     grammar/networking/protocols.g4
//!
//! Stream grammar:
//!     grammar/networking/streaming.g4
//!
//! Socket grammar:
//!     grammar/networking/sockets.g4
//!
//! Networking specification:
//!     grammar/spec/networking.md
//!
//! Actual execution:
//!     crate::runtime
//!
//! Distributed execution:
//!     crate::distributed
//!
//! Hardware realization:
//!     HAL and provider implementations.
//!
//! ## POCO-REAF
//!
//! Transport identities and communication modes are open-world identifiers.
//! New transport technologies must not require modifying a closed Rust enum.
//!
//! The same logical communication request may be realized by different
//! providers on different targets.
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Transport identities.
//! - Providers.
//! - Capabilities.
//! - Transport properties.
//! - Connection counts.
//! - Datagram counts.
//! - Payload lengths.
//! - Listener counts.
//! - Address values.
//! - Communication domains.
//!
//! Practical limits are determined by available resources, provider
//! capabilities, operational budgets, and explicit program constraints.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Important semantics
//!
//! Stream operations may transfer fewer bytes than requested. Callers must
//! handle partial writes and reads.
//!
//! A stream receive operation returning zero bytes indicates end-of-stream.
//!
//! Datagram operations preserve message boundaries. A provider must not
//! silently report a partial datagram as a complete successful send.
//!
//! Closing a resource is an explicit operation. Implementations must document
//! whether closure is idempotent and how concurrent operations behave.
//!
//! This module provides synchronous contracts. Async execution and scheduling
//! belong to the runtime and its executor integration.

use core::fmt;
use std::error::Error as StdError;
use std::time::Duration;

use crate::stdlib::networking::addresses::{
    AddressDescriptor,
    AddressValue,
    ResolvedAddress,
};

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn is_valid_qualified_identifier(value: &str) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }

    if value.chars().any(char::is_control) {
        return false;
    }

    value.split("::").all(|component| {
        if component.is_empty() {
            return false;
        }

        let mut bytes = component.bytes();

        let Some(first) = bytes.next() else {
            return false;
        };

        if !(first.is_ascii_alphanumeric() || first == b'_') {
            return false;
        }

        bytes.all(|byte| {
            byte.is_ascii_alphanumeric()
                || byte == b'_'
                || byte == b'-'
                || byte == b'.'
        })
    })
}

// ============================================================================
// TRANSPORT ERRORS
// ============================================================================

/// Classification of transport failures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TransportErrorKind {
    /// A transport identifier is invalid.
    InvalidTransport,

    /// A communication-mode identifier is invalid.
    InvalidMode,

    /// A transport request is invalid.
    InvalidRequest,

    /// A transport provider does not support the requested operation.
    UnsupportedOperation,

    /// The requested transport is unavailable.
    Unavailable,

    /// The destination cannot be reached.
    Unreachable,

    /// The operation timed out.
    TimedOut,

    /// The operation was cancelled.
    Cancelled,

    /// The connection has been closed.
    Closed,

    /// The operation is not valid in the current connection state.
    InvalidState,

    /// The peer has closed its sending direction.
    EndOfStream,

    /// A provider encountered an underlying I/O failure.
    Io,

    /// A provider encountered an internal failure.
    ProviderFailure,

    /// A datagram was not transferred in its entirety.
    IncompleteDatagram,

    /// A provider returned an invalid result.
    InvalidProviderResult,
}

impl TransportErrorKind {
    /// Returns the stable textual classification.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidTransport => "invalid_transport",
            Self::InvalidMode => "invalid_mode",
            Self::InvalidRequest => "invalid_request",
            Self::UnsupportedOperation => "unsupported_operation",
            Self::Unavailable => "unavailable",
            Self::Unreachable => "unreachable",
            Self::TimedOut => "timed_out",
            Self::Cancelled => "cancelled",
            Self::Closed => "closed",
            Self::InvalidState => "invalid_state",
            Self::EndOfStream => "end_of_stream",
            Self::Io => "io",
            Self::ProviderFailure => "provider_failure",
            Self::IncompleteDatagram => "incomplete_datagram",
            Self::InvalidProviderResult => "invalid_provider_result",
        }
    }
}

impl fmt::Display for TransportErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Structured transport error.
///
/// An underlying provider error may be attached without making this module
/// depend on a particular networking implementation.
#[derive(Debug)]
pub struct TransportError {
    kind: TransportErrorKind,
    message: String,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl TransportError {
    /// Creates a transport error.
    #[must_use]
    pub fn new(
        kind: TransportErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            source: None,
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> TransportErrorKind {
        self.kind
    }

    /// Returns the human-readable error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Attaches an underlying error.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns whether an underlying error exists.
    #[must_use]
    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "network transport error [{}]: {}",
            self.kind,
            self.message
        )
    }
}

impl StdError for TransportError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

/// Result type for transport operations.
pub type TransportResult<T> = Result<T, TransportError>;

fn transport_error(
    kind: TransportErrorKind,
    message: impl Into<String>,
) -> TransportError {
    TransportError::new(kind, message)
}

// ============================================================================
// TRANSPORT IDENTITY
// ============================================================================

/// Open-world transport identity.
///
/// This is deliberately not an enum of TCP, UDP, QUIC, RDMA, or other
/// technologies. New transports can be introduced without modifying this
/// type.
///
/// Examples:
///
/// - `network::tcp`
/// - `network::udp`
/// - `network::quic`
/// - `network::rdma`
/// - `network::in_process`
/// - `fabric::hardware_interconnect`
/// - `vendor::future_transport`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransportId(String);

impl TransportId {
    /// Creates a validated transport identity.
    pub fn new(value: impl Into<String>) -> TransportResult<Self> {
        let value = value.into();

        if !is_valid_qualified_identifier(&value) {
            return Err(transport_error(
                TransportErrorKind::InvalidTransport,
                "invalid qualified transport identifier",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the complete transport identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Checks identity equality without allocating.
    #[must_use]
    pub fn matches(&self, value: &str) -> bool {
        self.0 == value
    }
}

impl fmt::Display for TransportId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// COMMUNICATION MODE
// ============================================================================

/// Open-world communication-mode identity.
///
/// Modes describe communication semantics rather than a particular
/// networking technology.
///
/// Standard identities are provided as associated constants. Additional
/// modes may be introduced through [`TransportMode::new`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransportMode(String);

impl TransportMode {
    /// Byte-stream communication mode.
    pub const STREAM: &'static str = "network::stream";

    /// Datagram communication mode.
    pub const DATAGRAM: &'static str = "network::datagram";

    /// Message-oriented communication mode.
    pub const MESSAGE: &'static str = "network::message";

    /// Record-oriented communication mode.
    pub const RECORD: &'static str = "network::record";

    /// Creates a validated communication mode.
    pub fn new(value: impl Into<String>) -> TransportResult<Self> {
        let value = value.into();

        if !is_valid_qualified_identifier(&value) {
            return Err(transport_error(
                TransportErrorKind::InvalidMode,
                "invalid qualified communication-mode identifier",
            ));
        }

        Ok(Self(value))
    }

    /// Creates the standard stream mode.
    #[must_use]
    pub fn stream() -> Self {
        Self(Self::STREAM.to_owned())
    }

    /// Creates the standard datagram mode.
    #[must_use]
    pub fn datagram() -> Self {
        Self(Self::DATAGRAM.to_owned())
    }

    /// Creates the standard message mode.
    #[must_use]
    pub fn message() -> Self {
        Self(Self::MESSAGE.to_owned())
    }

    /// Creates the standard record mode.
    #[must_use]
    pub fn record() -> Self {
        Self(Self::RECORD.to_owned())
    }

    /// Returns the communication-mode identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Checks whether this mode has the supplied identity.
    #[must_use]
    pub fn matches(&self, value: &str) -> bool {
        self.0 == value
    }
}

impl fmt::Display for TransportMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// TRANSPORT PROPERTIES
// ============================================================================

/// An extensible transport property.
///
/// Property interpretation belongs to the provider or semantic authority
/// that defines the property.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransportProperty {
    key: String,
    value: String,
}

impl TransportProperty {
    /// Creates a validated transport property.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> TransportResult<Self> {
        let key = key.into();
        let value = value.into();

        if !is_valid_qualified_identifier(&key) {
            return Err(transport_error(
                TransportErrorKind::InvalidRequest,
                "invalid transport property identifier",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(transport_error(
                TransportErrorKind::InvalidRequest,
                "transport property contains control characters",
            ));
        }

        Ok(Self { key, value })
    }

    /// Returns the property identifier.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the property value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// TRANSPORT REQUEST
// ============================================================================

/// Portable request for establishing or binding a transport.
///
/// The address is a logical descriptor. The provider is responsible for
/// interpreting it using the relevant address-resolution and platform
/// facilities.
///
/// A timeout is an operational request, not a guarantee that every provider
/// can satisfy it exactly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransportRequest {
    transport: TransportId,
    mode: TransportMode,
    address: AddressDescriptor,
    timeout: Option<Duration>,
    properties: Vec<TransportProperty>,
}

impl TransportRequest {
    /// Creates a transport request.
    #[must_use]
    pub fn new(
        transport: TransportId,
        mode: TransportMode,
        address: AddressDescriptor,
    ) -> Self {
        Self {
            transport,
            mode,
            address,
            timeout: None,
            properties: Vec::new(),
        }
    }

    /// Sets an optional operation timeout.
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Adds a provider-specific or standardized property.
    #[must_use]
    pub fn with_property(mut self, property: TransportProperty) -> Self {
        self.properties.push(property);
        self
    }

    /// Returns the requested transport.
    #[must_use]
    pub fn transport(&self) -> &TransportId {
        &self.transport
    }

    /// Returns the requested communication mode.
    #[must_use]
    pub fn mode(&self) -> &TransportMode {
        &self.mode
    }

    /// Returns the logical address descriptor.
    #[must_use]
    pub fn address(&self) -> &AddressDescriptor {
        &self.address
    }

    /// Returns the optional timeout.
    #[must_use]
    pub const fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Returns the ordered properties.
    #[must_use]
    pub fn properties(&self) -> &[TransportProperty] {
        &self.properties
    }

    /// Returns all properties matching a key.
    pub fn properties_named<'a>(
        &'a self,
        key: &'a str,
    ) -> impl Iterator<Item = &'a TransportProperty> + 'a {
        self.properties
            .iter()
            .filter(move |property| property.key() == key)
    }
}

// ============================================================================
// TRANSPORT CAPABILITIES
// ============================================================================

/// Describes a capability exposed by a transport provider.
///
/// Capability names are open-world identifiers. Their meaning must be
/// documented by the provider or the relevant capability authority.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TransportCapability {
    identity: String,
    description: Option<String>,
}

impl TransportCapability {
    /// Creates a transport capability.
    pub fn new(identity: impl Into<String>) -> TransportResult<Self> {
        let identity = identity.into();

        if !is_valid_qualified_identifier(&identity) {
            return Err(transport_error(
                TransportErrorKind::InvalidRequest,
                "invalid transport capability identity",
            ));
        }

        Ok(Self {
            identity,
            description: None,
        })
    }

    /// Attaches a human-readable description.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns the capability identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// PROVIDER INFORMATION
// ============================================================================

/// Descriptive information about a transport provider.
///
/// This is metadata only. Provider discovery and registration belong to
/// the standard-library registry or runtime provider infrastructure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransportProviderInfo {
    identity: String,
    version: String,
    transports: Vec<TransportId>,
    modes: Vec<TransportMode>,
    capabilities: Vec<TransportCapability>,
}

impl TransportProviderInfo {
    /// Creates provider metadata.
    pub fn new(
        identity: impl Into<String>,
        version: impl Into<String>,
    ) -> TransportResult<Self> {
        let identity = identity.into();
        let version = version.into();

        if !is_valid_qualified_identifier(&identity) {
            return Err(transport_error(
                TransportErrorKind::InvalidRequest,
                "invalid provider identity",
            ));
        }

        if version.trim().is_empty()
            || version.chars().any(char::is_control)
        {
            return Err(transport_error(
                TransportErrorKind::InvalidRequest,
                "invalid provider version",
            ));
        }

        Ok(Self {
            identity,
            version,
            transports: Vec::new(),
            modes: Vec::new(),
            capabilities: Vec::new(),
        })
    }

    /// Adds a supported transport identity.
    #[must_use]
    pub fn with_transport(mut self, transport: TransportId) -> Self {
        self.transports.push(transport);
        self
    }

    /// Adds a supported communication mode.
    #[must_use]
    pub fn with_mode(mut self, mode: TransportMode) -> Self {
        self.modes.push(mode);
        self
    }

    /// Adds a provider capability.
    #[must_use]
    pub fn with_capability(
        mut self,
        capability: TransportCapability,
    ) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Returns the provider version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns supported transport identities.
    #[must_use]
    pub fn transports(&self) -> &[TransportId] {
        &self.transports
    }

    /// Returns supported communication modes.
    #[must_use]
    pub fn modes(&self) -> &[TransportMode] {
        &self.modes
    }

    /// Returns declared capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &[TransportCapability] {
        &self.capabilities
    }

    /// Checks whether a transport identity is declared.
    #[must_use]
    pub fn supports_transport(&self, transport: &TransportId) -> bool {
        self.transports.iter().any(|item| item == transport)
    }

    /// Checks whether a communication mode is declared.
    #[must_use]
    pub fn supports_mode(&self, mode: &TransportMode) -> bool {
        self.modes.iter().any(|item| item == mode)
    }
}

// ============================================================================
// STREAM CONNECTION
// ============================================================================

/// A portable bidirectional byte-stream connection.
///
/// Implementations may transfer fewer bytes than requested.
///
/// A successful `send` returns the number of bytes accepted by the provider.
/// Callers must handle partial writes.
///
/// A successful `receive` returns the number of bytes written into the
/// supplied buffer. A return value of zero means the peer has ended its
/// sending direction.
///
/// These operations do not imply message boundaries.
pub trait StreamConnection: Send + Sync {
    /// Sends bytes through the connection.
    ///
    /// Returns the number of bytes accepted. A provider must not report more
    /// bytes than the supplied slice contains.
    fn send(&self, bytes: &[u8]) -> TransportResult<usize>;

    /// Receives bytes into the supplied buffer.
    ///
    /// Returns zero when the peer has closed its sending direction.
    /// A provider must not report more bytes than the buffer can contain.
    fn receive(&self, buffer: &mut [u8]) -> TransportResult<usize>;

    /// Requests shutdown of the connection.
    ///
    /// Implementations must document their shutdown and idempotency
    /// guarantees. An already-closed connection may return success if
    /// closure is idempotent.
    fn close(&self) -> TransportResult<()>;

    /// Returns the resolved local address when available.
    ///
    /// Some logical or virtual transports do not expose a physical address.
    fn local_address(&self) -> TransportResult<Option<ResolvedAddress>>;

    /// Returns the resolved peer address when available.
    fn peer_address(&self) -> TransportResult<Option<ResolvedAddress>>;
}

// ============================================================================
// STREAM LISTENER
// ============================================================================

/// A listener capable of accepting stream connections.
///
/// Accept operations may block according to the provider's documented
/// behavior. Async acceptance belongs to runtime executor integration.
pub trait StreamListener: Send + Sync {
    /// Accepts one incoming stream connection.
    fn accept(&self) -> TransportResult<Box<dyn StreamConnection>>;

    /// Requests that the listener stop accepting connections.
    fn close(&self) -> TransportResult<()>;

    /// Returns the resolved local address when available.
    fn local_address(&self) -> TransportResult<Option<ResolvedAddress>>;
}

// ============================================================================
// DATAGRAM TRANSPORT
// ============================================================================

/// A received datagram and its source address.
///
/// Datagram payload boundaries are preserved by the provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivedDatagram {
    source: ResolvedAddress,
    payload: Vec<u8>,
}

impl ReceivedDatagram {
    /// Creates a received datagram.
    #[must_use]
    pub fn new(source: ResolvedAddress, payload: Vec<u8>) -> Self {
        Self { source, payload }
    }

    /// Returns the source address.
    #[must_use]
    pub fn source(&self) -> &ResolvedAddress {
        &self.source
    }

    /// Returns the payload.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Consumes the datagram and returns its payload.
    #[must_use]
    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }
}

/// A portable message-preserving datagram transport.
///
/// Unlike a stream, a datagram transport preserves individual message
/// boundaries. The provider must document its delivery, ordering, loss,
/// duplication and maximum operational message-size behavior.
///
/// This interface does not impose a universal datagram-size limit.
pub trait DatagramTransport: Send + Sync {
    /// Sends one complete datagram to a resolved destination.
    ///
    /// Returns the complete payload length on success.
    ///
    /// Returning a smaller length is not a successful complete datagram
    /// transfer and must be treated as an error by the implementation.
    fn send_to(
        &self,
        destination: &ResolvedAddress,
        payload: &[u8],
    ) -> TransportResult<usize>;

    /// Receives one datagram.
    ///
    /// The returned payload contains exactly the bytes delivered by the
    /// provider for that datagram. Providers must document truncation
    /// behavior if their underlying platform can truncate datagrams.
    fn receive(&self) -> TransportResult<ReceivedDatagram>;

    /// Closes the datagram transport.
    fn close(&self) -> TransportResult<()>;

    /// Returns the resolved local address when available.
    fn local_address(&self) -> TransportResult<Option<ResolvedAddress>>;
}

// ============================================================================
// TRANSPORT PROVIDER
// ============================================================================

/// Provider contract for actual transport realization.
///
/// A provider is responsible for translating portable requests into
/// communication resources supported by its execution environment.
///
/// Providers may implement operating-system networking, in-process
/// communication, hardware fabrics, distributed transports, or future
/// communication technologies.
///
/// This trait does not perform provider registration, scheduling, routing,
/// resource allocation, or lifecycle orchestration.
///
/// Those responsibilities belong to the runtime and provider registry.
pub trait TransportProvider: Send + Sync {
    /// Returns provider identity, version, transports, modes and capabilities.
    fn info(&self) -> TransportProviderInfo;

    /// Establishes an outgoing stream connection.
    fn connect_stream(
        &self,
        request: &TransportRequest,
    ) -> TransportResult<Box<dyn StreamConnection>>;

    /// Creates a stream listener.
    fn bind_stream(
        &self,
        request: &TransportRequest,
    ) -> TransportResult<Box<dyn StreamListener>>;

    /// Creates a datagram transport.
    fn bind_datagram(
        &self,
        request: &TransportRequest,
    ) -> TransportResult<Box<dyn DatagramTransport>>;
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Validates a provider's reported byte count against the supplied length.
///
/// This helper prevents a provider from claiming to have transferred more
/// bytes than the caller supplied.
pub fn validate_transfer_count(
    transferred: usize,
    supplied: usize,
) -> TransportResult<usize> {
    if transferred > supplied {
        return Err(transport_error(
            TransportErrorKind::InvalidProviderResult,
            "provider reported more transferred bytes than supplied",
        ));
    }

    Ok(transferred)
}

/// Validates that a datagram was transferred completely.
///
/// Unlike stream writes, datagram sends must preserve message boundaries.
pub fn validate_datagram_transfer(
    transferred: usize,
    supplied: usize,
) -> TransportResult<usize> {
    validate_transfer_count(transferred, supplied)?;

    if transferred != supplied {
        return Err(transport_error(
            TransportErrorKind::IncompleteDatagram,
            "provider did not report a complete datagram transfer",
        ));
    }

    Ok(transferred)
}

/// Validates a provider's receive count against buffer capacity.
pub fn validate_receive_count(
    received: usize,
    capacity: usize,
) -> TransportResult<usize> {
    if received > capacity {
        return Err(transport_error(
            TransportErrorKind::InvalidProviderResult,
            "provider reported more received bytes than buffer capacity",
        ));
    }

    Ok(received)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::networking::addresses::{
        AddressFamilyId,
        AddressValue,
    };

    #[test]
    fn transport_identity_accepts_open_world_names() {
        let tcp = TransportId::new("network::tcp").unwrap();
        let future = TransportId::new("future_fabric::transport").unwrap();

        assert_eq!(tcp.as_str(), "network::tcp");
        assert_eq!(future.as_str(), "future_fabric::transport");
    }

    #[test]
    fn transport_identity_rejects_invalid_names() {
        assert!(TransportId::new("").is_err());
        assert!(TransportId::new("network::::tcp").is_err());
        assert!(TransportId::new("network:: tcp").is_err());
    }

    #[test]
    fn communication_modes_are_extensible() {
        let stream = TransportMode::stream();
        let custom = TransportMode::new("vendor::record_stream").unwrap();

        assert_eq!(stream.as_str(), TransportMode::STREAM);
        assert_eq!(custom.as_str(), "vendor::record_stream");
    }

    #[test]
    fn request_preserves_properties_and_timeout() {
        let family = AddressFamilyId::new("network::logical").unwrap();
        let value = AddressValue::new("service://compute").unwrap();

        let address = AddressDescriptor::new(family, value);

        let request = TransportRequest::new(
            TransportId::new("network::tcp").unwrap(),
            TransportMode::stream(),
            address,
        )
        .with_timeout(Duration::from_secs(5))
        .with_property(
            TransportProperty::new(
                "network::quality",
                "reliable",
            )
            .unwrap(),
        );

        assert_eq!(request.transport().as_str(), "network::tcp");
        assert_eq!(request.mode().as_str(), TransportMode::STREAM);
        assert_eq!(request.timeout(), Some(Duration::from_secs(5)));
        assert_eq!(
            request.properties_named("network::quality").count(),
            1
        );
    }

    #[test]
    fn provider_metadata_is_extensible() {
        let info = TransportProviderInfo::new(
            "provider::example",
            "1.0.0",
        )
        .unwrap()
        .with_transport(TransportId::new("network::quic").unwrap())
        .with_mode(TransportMode::stream())
        .with_capability(
            TransportCapability::new("network::reliable")
                .unwrap(),
        );

        assert!(info.supports_transport(
            &TransportId::new("network::quic").unwrap()
        ));

        assert!(info.supports_mode(&TransportMode::stream()));

        assert_eq!(info.capabilities().len(), 1);
    }

    #[test]
    fn stream_transfer_validation_allows_partial_writes() {
        assert_eq!(validate_transfer_count(4, 10).unwrap(), 4);
        assert_eq!(validate_transfer_count(0, 10).unwrap(), 0);
        assert!(validate_transfer_count(11, 10).is_err());
    }

    #[test]
    fn datagram_validation_requires_complete_transfer() {
        assert_eq!(validate_datagram_transfer(10, 10).unwrap(), 10);
        assert!(validate_datagram_transfer(5, 10).is_err());
        assert!(validate_datagram_transfer(11, 10).is_err());
    }

    #[test]
    fn receive_validation_rejects_invalid_provider_counts() {
        assert_eq!(validate_receive_count(5, 10).unwrap(), 5);
        assert_eq!(validate_receive_count(0, 10).unwrap(), 0);
        assert!(validate_receive_count(11, 10).is_err());
    }

    #[test]
    fn received_datagram_preserves_payload_and_source() {
        let family = AddressFamilyId::new("network::ipv4").unwrap();
        let value = AddressValue::new("192.0.2.1").unwrap();

        let source = ResolvedAddress::new(family, value);

        let datagram = ReceivedDatagram::new(
            source.clone(),
            vec![1, 2, 3, 4],
        );

        assert_eq!(datagram.source(), &source);
        assert_eq!(datagram.payload(), &[1, 2, 3, 4]);
        assert_eq!(datagram.into_payload(), vec![1, 2, 3, 4]);
    }
}
