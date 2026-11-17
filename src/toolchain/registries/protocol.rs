
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Registry Wire Protocol
//!
//! Path: src/toolchain/registries/protocol.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the versioned, provider-neutral wire protocol used to exchange
//! requests and responses with package registries.
//!
//! The implementation supports independently operated registry systems,
//! including public, private, enterprise, local, self-hosted and federated
//! registries.
//!
//! It does not prescribe a network transport or registry vendor.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Registry wire-message envelopes.
//! - Request serialization and deserialization.
//! - Response serialization and deserialization.
//! - Envelope schema validation.
//! - Protocol-version negotiation.
//! - Protocol compatibility validation.
//! - Registry identity preservation.
//! - Opaque payload encoding and decoding.
//! - Malformed-message rejection.
//!
//! This module does not own:
//!
//! - Registry provider execution.
//! - Network transport.
//! - HTTP or other transport clients.
//! - Authentication or authorization.
//! - Credential storage.
//! - Registry configuration.
//! - Registry discovery.
//! - Package metadata interpretation.
//! - Dependency resolution.
//! - Package archive encoding.
//! - Package integrity or signature verification.
//! - Registry federation.
//! - Hardware discovery.
//! - Computational-domain selection.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Registry identity:
//!     crate::toolchain::ids::RegistryId
//!
//! Registry protocol version:
//!     crate::toolchain::version::RegistryProtocolVersion
//!
//! Version compatibility:
//!     crate::toolchain::version::VersionRange
//!
//! Registry request and response:
//!     super::provider
//!
//! Provider execution:
//!     super::provider::execute_checked
//!
//! Registry configuration:
//!     super::registry
//!
//! Structured errors:
//!     crate::toolchain::errors
//!
//! ## WIRE FORMAT
//!
//! The envelope uses JSON.
//!
//! Binary payloads are represented as hexadecimal strings.
//!
//! The payload itself remains opaque to this module.
//!
//! The protocol version identifies the applicable envelope and operation
//! contract. Changes that cannot be interpreted compatibly require a
//! separately versioned protocol contract.
//!
//! Unknown envelope fields are rejected. This prevents silently accepting
//! an incompatible message as though it were valid.
//!
//! ## SCALABILITY
//!
//! No arbitrary application-level maximum is imposed on:
//!
//! - Payload size.
//! - Registry count.
//! - Operations.
//! - Protocol versions.
//! - Computational domains.
//! - Package counts.
//! - Providers.
//! - Federated participants.
//!
//! Actual allocations and processing remain subject to available resources
//! and caller-configured operational budgets.
//!
//! No literal infinite memory, storage or computation is promised.
//!
//! ## SECURITY
//!
//! - Malformed envelopes are rejected.
//! - Invalid identities are rejected.
//! - Unknown envelope fields are rejected.
//! - Invalid protocol versions are rejected.
//! - Invalid hexadecimal payloads are rejected.
//! - Registry identity mismatches are rejected.
//! - Protocol mismatches are rejected.
//! - Credentials must not be embedded in registry payloads by default.
//! - Transport authentication and authorization remain external duties.
//!
//! ## INTEGRATION
//!
//! provider.rs supplies the opaque request and response types.
//! registry.rs supplies canonical registry configuration.
//! client.rs coordinates provider execution.
//!
//! This module can encode and decode messages independently of the
//! transport and execution provider.

use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::toolchain::errors::{
    Result,
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::ids::RegistryId;

use crate::toolchain::version::{
    RegistryProtocolVersion,
    VersionRange,
};

use super::provider::{
    RegistryOperationId,
    RegistryRequest,
    RegistryResponse,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_PROTOCOL: &str = "registry.protocol";
const OPERATION_ENCODE_REQUEST: &str = "registry.protocol.encode_request";
const OPERATION_DECODE_REQUEST: &str = "registry.protocol.decode_request";
const OPERATION_ENCODE_RESPONSE: &str = "registry.protocol.encode_response";
const OPERATION_DECODE_RESPONSE: &str = "registry.protocol.decode_response";
const OPERATION_NEGOTIATE: &str = "registry.protocol.negotiate";

// ============================================================================
// WIRE MESSAGE IDENTIFIERS
// ============================================================================

/// Canonical registry request message identifier.
pub const REGISTRY_REQUEST_MESSAGE: &str = "zamani.registry.request";

/// Canonical registry response message identifier.
pub const REGISTRY_RESPONSE_MESSAGE: &str = "zamani.registry.response";

/// Canonical binary payload representation.
pub const REGISTRY_PAYLOAD_ENCODING: &str = "hex";

// ============================================================================
// PROTOCOL ERRORS
// ============================================================================

/// Errors encountered while validating a registry protocol message.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RegistryProtocolError {
    /// The message contains an invalid envelope.
    InvalidEnvelope,

    /// The message has an unexpected message identifier.
    InvalidMessageType,

    /// The message contains an invalid registry identity.
    InvalidRegistryIdentity,

    /// The message contains an invalid operation identifier.
    InvalidOperation,

    /// The protocol version is malformed.
    InvalidProtocolVersion,

    /// The requested protocol version is incompatible.
    IncompatibleProtocol,

    /// The payload encoding is unsupported.
    UnsupportedPayloadEncoding,

    /// The payload is not valid hexadecimal data.
    InvalidPayloadEncoding,

    /// The request and response refer to different registries.
    RegistryIdentityMismatch,

    /// The response uses a different protocol version.
    ProtocolVersionMismatch,

    /// The request and response do not satisfy their expected contract.
    MessageContractMismatch,

    /// No mutually supported protocol version exists.
    NoCompatibleProtocol,
}

impl fmt::Display for RegistryProtocolError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidEnvelope => {
                formatter.write_str("Invalid registry protocol envelope.")
            }

            Self::InvalidMessageType => {
                formatter.write_str("Unexpected registry message type.")
            }

            Self::InvalidRegistryIdentity => {
                formatter.write_str("Invalid registry identity.")
            }

            Self::InvalidOperation => {
                formatter.write_str("Invalid registry operation identifier.")
            }

            Self::InvalidProtocolVersion => {
                formatter.write_str("Invalid registry protocol version.")
            }

            Self::IncompatibleProtocol => {
                formatter.write_str("Incompatible registry protocol version.")
            }

            Self::UnsupportedPayloadEncoding => {
                formatter.write_str("Unsupported registry payload encoding.")
            }

            Self::InvalidPayloadEncoding => {
                formatter.write_str("Invalid registry payload encoding.")
            }

            Self::RegistryIdentityMismatch => {
                formatter.write_str(
                    "Registry response identity does not match the request.",
                )
            }

            Self::ProtocolVersionMismatch => {
                formatter.write_str(
                    "Registry response protocol version does not match the request.",
                )
            }

            Self::MessageContractMismatch => {
                formatter.write_str(
                    "Registry message does not satisfy its expected contract.",
                )
            }

            Self::NoCompatibleProtocol => {
                formatter.write_str(
                    "No mutually supported registry protocol version exists.",
                )
            }
        }
    }
}

impl std::error::Error for RegistryProtocolError {}

// ============================================================================
// WIRE ENVELOPES
// ============================================================================

/// Serialized registry request envelope.
///
/// The structure is private to this module so wire-format changes remain
/// controlled by the protocol implementation.
///
/// All fields are explicitly represented in the wire contract.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RegistryRequestEnvelope {
    /// Message discriminator.
    message_type: String,

    /// Negotiated protocol version.
    protocol_version: String,

    /// Canonical namespace-qualified registry identity.
    registry_id: String,

    /// Open-world registry operation identifier.
    operation: String,

    /// Binary payload encoding.
    payload_encoding: String,

    /// Encoded opaque payload.
    payload: String,
}

/// Serialized registry response envelope.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RegistryResponseEnvelope {
    /// Message discriminator.
    message_type: String,

    /// Negotiated protocol version.
    protocol_version: String,

    /// Canonical namespace-qualified registry identity.
    registry_id: String,

    /// Binary payload encoding.
    payload_encoding: String,

    /// Encoded opaque payload.
    payload: String,
}

// ============================================================================
// PROTOCOL NEGOTIATION
// ============================================================================

/// Result of successful registry protocol negotiation.
///
/// This represents an agreed protocol version, not proof that a remote
/// registry is reachable or that authentication has succeeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProtocolNegotiation {
    selected: RegistryProtocolVersion,
}

impl ProtocolNegotiation {
    /// Construct a negotiation result.
    #[must_use]
    pub const fn new(
        selected: RegistryProtocolVersion,
    ) -> Self {
        Self { selected }
    }

    /// Return the selected protocol version.
    #[must_use]
    pub const fn selected(self) -> RegistryProtocolVersion {
        self.selected
    }
}

impl fmt::Display for ProtocolNegotiation {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.selected.fmt(formatter)
    }
}

// ============================================================================
// PROTOCOL CODEC
// ============================================================================

/// Provider-neutral registry protocol codec.
///
/// This is a stateless codec. It owns no registry connection, provider,
/// authentication session, network client or mutable global configuration.
///
/// Multiple callers can use the same codec independently.
///
/// The codec is deliberately transport-independent.
#[derive(Debug, Clone, Copy, Default)]
pub struct RegistryProtocolCodec;

impl RegistryProtocolCodec {
    /// Construct a registry protocol codec.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    // ========================================================================
    // REQUEST ENCODING
    // ========================================================================

    /// Encode a canonical registry request into its wire representation.
    ///
    /// The request payload is opaque binary data. It is encoded using
    /// hexadecimal representation without interpreting its contents.
    ///
    /// Serialization uses a deterministic struct field order.
    ///
    /// # Errors
    ///
    /// Returns a structured error if serialization fails.
    pub fn encode_request(
        &self,
        request: &RegistryRequest,
    ) -> Result<Vec<u8>> {
        let envelope = RegistryRequestEnvelope {
            message_type: REGISTRY_REQUEST_MESSAGE.to_owned(),
            protocol_version: request.protocol_version().to_string(),
            registry_id: request.registry_id().to_string(),
            operation: request.operation().as_str().to_owned(),
            payload_encoding: REGISTRY_PAYLOAD_ENCODING.to_owned(),
            payload: hex::encode(request.payload()),
        };

        serde_json::to_vec(&envelope).map_err(|error| {
            serialization_error(
                OPERATION_ENCODE_REQUEST,
                error,
            )
        })
    }

    // ========================================================================
    // REQUEST DECODING
    // ========================================================================

    /// Decode and validate a registry request envelope.
    ///
    /// Validation includes:
    ///
    /// - JSON syntax.
    /// - Exact message discriminator.
    /// - Protocol version syntax.
    /// - Registry identity syntax.
    /// - Operation identifier syntax.
    /// - Supported payload encoding.
    /// - Payload hexadecimal validity.
    ///
    /// This method does not establish whether the selected provider
    /// supports the requested operation.
    ///
    /// That decision belongs to provider.rs.
    pub fn decode_request(
        &self,
        bytes: &[u8],
    ) -> Result<RegistryRequest> {
        let envelope: RegistryRequestEnvelope =
            serde_json::from_slice(bytes).map_err(|error| {
                serialization_error(
                    OPERATION_DECODE_REQUEST,
                    error,
                )
            })?;

        if envelope.message_type != REGISTRY_REQUEST_MESSAGE {
            return Err(protocol_error(
                RegistryProtocolError::InvalidMessageType,
                OPERATION_DECODE_REQUEST,
            ));
        }

        let protocol_version =
            parse_protocol_version(&envelope.protocol_version)?;

        let registry_id =
            RegistryId::parse(&envelope.registry_id).map_err(|error| {
                protocol_error_with_source(
                    RegistryProtocolError::InvalidRegistryIdentity,
                    OPERATION_DECODE_REQUEST,
                    error,
                )
            })?;

        let operation =
            RegistryOperationId::new(envelope.operation).map_err(|error| {
                error
                    .with_operation(OPERATION_DECODE_REQUEST)
            })?;

        let payload =
            decode_payload(
                &envelope.payload_encoding,
                &envelope.payload,
                OPERATION_DECODE_REQUEST,
            )?;

        Ok(RegistryRequest::new(
            registry_id,
            protocol_version,
            operation,
            payload,
        ))
    }

    // ========================================================================
    // RESPONSE ENCODING
    // ========================================================================

    /// Encode a canonical registry response into its wire representation.
    ///
    /// The response payload is opaque binary data.
    ///
    /// This method does not claim that the response is valid for a
    /// particular request. Use `validate_response_for_request` when
    /// request-specific validation is required.
    pub fn encode_response(
        &self,
        response: &RegistryResponse,
    ) -> Result<Vec<u8>> {
        let envelope = RegistryResponseEnvelope {
            message_type: REGISTRY_RESPONSE_MESSAGE.to_owned(),
            protocol_version: response.protocol_version().to_string(),
            registry_id: response.registry_id().to_string(),
            payload_encoding: REGISTRY_PAYLOAD_ENCODING.to_owned(),
            payload: hex::encode(response.payload()),
        };

        serde_json::to_vec(&envelope).map_err(|error| {
            serialization_error(
                OPERATION_ENCODE_RESPONSE,
                error,
            )
        })
    }

    // ========================================================================
    // RESPONSE DECODING
    // ========================================================================

    /// Decode and validate a registry response envelope.
    ///
    /// This validates the envelope and its identity/version syntax.
    ///
    /// It does not establish that the response corresponds to a particular
    /// outstanding request. That requires request-specific validation.
    pub fn decode_response(
        &self,
        bytes: &[u8],
    ) -> Result<RegistryResponse> {
        let envelope: RegistryResponseEnvelope =
            serde_json::from_slice(bytes).map_err(|error| {
                serialization_error(
                    OPERATION_DECODE_RESPONSE,
                    error,
                )
            })?;

        if envelope.message_type != REGISTRY_RESPONSE_MESSAGE {
            return Err(protocol_error(
                RegistryProtocolError::InvalidMessageType,
                OPERATION_DECODE_RESPONSE,
            ));
        }

        let protocol_version =
            parse_protocol_version(&envelope.protocol_version)?;

        let registry_id =
            RegistryId::parse(&envelope.registry_id).map_err(|error| {
                protocol_error_with_source(
                    RegistryProtocolError::InvalidRegistryIdentity,
                    OPERATION_DECODE_RESPONSE,
                    error,
                )
            })?;

        let payload =
            decode_payload(
                &envelope.payload_encoding,
                &envelope.payload,
                OPERATION_DECODE_RESPONSE,
            )?;

        Ok(RegistryResponse::new(
            registry_id,
            protocol_version,
            payload,
        ))
    }

    // ========================================================================
    // REQUEST / RESPONSE VALIDATION
    // ========================================================================

    /// Validate a decoded response against its originating request.
    ///
    /// Both registry identity and protocol version must match exactly.
    ///
    /// A mismatch is rejected rather than silently normalized.
    ///
    /// The response operation is not independently verified because
    /// RegistryResponse currently does not carry an operation identifier.
    /// Providers must preserve request/response association in their
    /// execution context.
    pub fn validate_response_for_request(
        &self,
        request: &RegistryRequest,
        response: &RegistryResponse,
    ) -> Result<()> {
        if request.registry_id() != response.registry_id() {
            return Err(protocol_error(
                RegistryProtocolError::RegistryIdentityMismatch,
                OPERATION_DECODE_RESPONSE,
            ));
        }

        if request.protocol_version() != response.protocol_version() {
            return Err(protocol_error(
                RegistryProtocolError::ProtocolVersionMismatch,
                OPERATION_DECODE_RESPONSE,
            ));
        }

        Ok(())
    }

    /// Decode a response and validate it against the originating request.
    ///
    /// This is the preferred response-decoding entry point when the
    /// originating request is available.
    pub fn decode_response_for_request(
        &self,
        request: &RegistryRequest,
        bytes: &[u8],
    ) -> Result<RegistryResponse> {
        let response = self.decode_response(bytes)?;

        self.validate_response_for_request(
            request,
            &response,
        )?;

        Ok(response)
    }

    // ========================================================================
    // PROTOCOL NEGOTIATION
    // ========================================================================

    /// Negotiate the highest mutually supported protocol version.
    ///
    /// Both sides provide inclusive version ranges.
    ///
    /// The selected version is the highest version contained in at least
    /// one range supplied by each participant.
    ///
    /// This method:
    ///
    /// - Validates all supplied ranges.
    /// - Supports multiple disjoint ranges.
    /// - Does not enumerate protocol versions.
    /// - Does not assume a particular registry vendor.
    /// - Does not assume a permanent protocol catalogue.
    /// - Does not silently select an unsupported version.
    ///
    /// No compatible version results in an explicit error.
    pub fn negotiate(
        &self,
        local: &[VersionRange<RegistryProtocolVersion>],
        remote: &[VersionRange<RegistryProtocolVersion>],
    ) -> Result<ProtocolNegotiation> {
        if local.is_empty() || remote.is_empty() {
            return Err(protocol_error(
                RegistryProtocolError::NoCompatibleProtocol,
                OPERATION_NEGOTIATE,
            ));
        }

        validate_ranges(local, OPERATION_NEGOTIATE)?;
        validate_ranges(remote, OPERATION_NEGOTIATE)?;

        let mut selected: Option<RegistryProtocolVersion> = None;

        for local_range in local {
            for remote_range in remote {
                let minimum =
                    std::cmp::max(
                        local_range.minimum(),
                        remote_range.minimum(),
                    );

                let maximum =
                    std::cmp::min(
                        local_range.maximum(),
                        remote_range.maximum(),
                    );

                if minimum <= maximum {
                    selected = Some(
                        selected
                            .map(|current| std::cmp::max(current, maximum))
                            .unwrap_or(maximum),
                    );
                }
            }
        }

        selected
            .map(ProtocolNegotiation::new)
            .ok_or_else(|| {
                protocol_error(
                    RegistryProtocolError::NoCompatibleProtocol,
                    OPERATION_NEGOTIATE,
                )
            })
    }

    /// Verify that a selected protocol version belongs to the supplied
    /// supported-version ranges.
    ///
    /// An empty range collection is not interpreted as universal support.
    pub fn validate_supported_version(
        &self,
        version: RegistryProtocolVersion,
        supported: &[VersionRange<RegistryProtocolVersion>],
    ) -> Result<()> {
        validate_ranges(supported, OPERATION_PROTOCOL)?;

        if supported.iter().any(|range| range.contains(version)) {
            return Ok(());
        }

        Err(protocol_error(
            RegistryProtocolError::IncompatibleProtocol,
            OPERATION_PROTOCOL,
        ))
    }
}

// ============================================================================
// FREE FUNCTIONS
// ============================================================================

/// Parse the canonical registry protocol version.
fn parse_protocol_version(
    value: &str,
) -> Result<RegistryProtocolVersion> {
    RegistryProtocolVersion::from_str(value).map_err(|error| {
        protocol_error_with_source(
            RegistryProtocolError::InvalidProtocolVersion,
            OPERATION_PROTOCOL,
            error,
        )
    })
}

/// Decode an opaque binary payload.
fn decode_payload(
    encoding: &str,
    payload: &str,
    operation: &str,
) -> Result<Vec<u8>> {
    if encoding != REGISTRY_PAYLOAD_ENCODING {
        return Err(protocol_error(
            RegistryProtocolError::UnsupportedPayloadEncoding,
            operation,
        ));
    }

    hex::decode(payload).map_err(|error| {
        protocol_error_with_source(
            RegistryProtocolError::InvalidPayloadEncoding,
            operation,
            error,
        )
    })
}

/// Validate a collection of protocol version ranges.
fn validate_ranges(
    ranges: &[VersionRange<RegistryProtocolVersion>],
    operation: &str,
) -> Result<()> {
    if ranges.is_empty() {
        return Err(protocol_error(
            RegistryProtocolError::NoCompatibleProtocol,
            operation,
        ));
    }

    for range in ranges {
        range.validate().map_err(|error| {
            protocol_error_with_source(
                RegistryProtocolError::IncompatibleProtocol,
                operation,
                error,
            )
        })?;
    }

    Ok(())
}

/// Construct a structured protocol error.
fn protocol_error(
    error: RegistryProtocolError,
    operation: &str,
) -> ToolchainError {
    ToolchainError::new(
        match error {
            RegistryProtocolError::IncompatibleProtocol
            | RegistryProtocolError::NoCompatibleProtocol
            | RegistryProtocolError::ProtocolVersionMismatch => {
                ToolchainErrorKind::IncompatibleVersion
            }

            RegistryProtocolError::RegistryIdentityMismatch => {
                ToolchainErrorKind::ValidationFailed
            }

            RegistryProtocolError::UnsupportedPayloadEncoding => {
                ToolchainErrorKind::UnsupportedOperation
            }

            _ => ToolchainErrorKind::InvalidInput,
        },
        error.to_string(),
    )
    .with_operation(operation)
}

/// Construct a protocol error while preserving its causal error.
fn protocol_error_with_source<E>(
    error: RegistryProtocolError,
    operation: &str,
    source: E,
) -> ToolchainError
where
    E: std::error::Error + Send + Sync + 'static,
{
    protocol_error(error, operation).with_source(source)
}

/// Convert a serialization failure into a structured toolchain error.
fn serialization_error<E>(
    operation: &str,
    source: E,
) -> ToolchainError
where
    E: std::error::Error + Send + Sync + 'static,
{
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        "Registry wire-message serialization or deserialization failed.",
    )
    .with_operation(operation)
    .with_source(source)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    use crate::toolchain::ids::{
        IdNamespace,
        RegistryId,
    };

    fn registry_id() -> RegistryId {
        RegistryId::new(
            IdNamespace::new("test").expect("valid namespace"),
            1,
        )
        .expect("valid registry ID")
    }

    fn protocol(
        major: u32,
        minor: u32,
        patch: u32,
    ) -> RegistryProtocolVersion {
        RegistryProtocolVersion::new(
            major,
            minor,
            patch,
        )
    }

    fn operation() -> RegistryOperationId {
        RegistryOperationId::new("packages.query")
            .expect("valid operation")
    }

    fn request() -> RegistryRequest {
        RegistryRequest::new(
            registry_id(),
            protocol(1, 0, 0),
            operation(),
            b"opaque request payload".to_vec(),
        )
    }

    fn response() -> RegistryResponse {
        RegistryResponse::new(
            registry_id(),
            protocol(1, 0, 0),
            b"opaque response payload".to_vec(),
        )
    }

    fn range(
        minimum: RegistryProtocolVersion,
        maximum: RegistryProtocolVersion,
    ) -> VersionRange<RegistryProtocolVersion> {
        VersionRange::new(minimum, maximum)
            .expect("valid version range")
    }

    #[test]
    fn codec_is_constructible() {
        let codec = RegistryProtocolCodec::new();

        assert_eq!(
            codec,
            RegistryProtocolCodec,
        );
    }

    #[test]
    fn request_round_trip_preserves_identity_version_operation_and_payload() {
        let codec = RegistryProtocolCodec::new();
        let original = request();

        let encoded = codec
            .encode_request(&original)
            .expect("request encoding");

        let decoded = codec
            .decode_request(&encoded)
            .expect("request decoding");

        assert_eq!(
            original.registry_id(),
            decoded.registry_id(),
        );

        assert_eq!(
            original.protocol_version(),
            decoded.protocol_version(),
        );

        assert_eq!(
            original.operation(),
            decoded.operation(),
        );

        assert_eq!(
            original.payload(),
            decoded.payload(),
        );
    }

    #[test]
    fn response_round_trip_preserves_identity_version_and_payload() {
        let codec = RegistryProtocolCodec::new();
        let original = response();

        let encoded = codec
            .encode_response(&original)
            .expect("response encoding");

        let decoded = codec
            .decode_response(&encoded)
            .expect("response decoding");

        assert_eq!(
            original.registry_id(),
            decoded.registry_id(),
        );

        assert_eq!(
            original.protocol_version(),
            decoded.protocol_version(),
        );

        assert_eq!(
            original.payload(),
            decoded.payload(),
        );
    }

    #[test]
    fn empty_payload_is_supported() {
        let codec = RegistryProtocolCodec::new();

        let original = RegistryRequest::new(
            registry_id(),
            protocol(1, 0, 0),
            operation(),
            Vec::<u8>::new(),
        );

        let encoded = codec
            .encode_request(&original)
            .expect("encoding succeeds");

        let decoded = codec
            .decode_request(&encoded)
            .expect("decoding succeeds");

        assert!(decoded.payload().is_empty());
    }

    #[test]
    fn arbitrary_binary_payload_is_preserved() {
        let codec = RegistryProtocolCodec::new();

        let payload = vec![
            0,
            1,
            2,
            127,
            128,
            254,
            255,
        ];

        let original = RegistryRequest::new(
            registry_id(),
            protocol(1, 0, 0),
            operation(),
            payload.clone(),
        );

        let encoded = codec
            .encode_request(&original)
            .expect("encoding succeeds");

        let decoded = codec
            .decode_request(&encoded)
            .expect("decoding succeeds");

        assert_eq!(
            decoded.payload(),
            payload.as_slice(),
        );
    }

    #[test]
    fn response_matches_originating_request() {
        let codec = RegistryProtocolCodec::new();
        let request = request();
        let response = response();

        assert!(
            codec
                .validate_response_for_request(
                    &request,
                    &response,
                )
                .is_ok()
        );
    }

    #[test]
    fn response_with_wrong_protocol_is_rejected() {
        let codec = RegistryProtocolCodec::new();

        let request = request();

        let response = RegistryResponse::new(
            registry_id(),
            protocol(2, 0, 0),
            b"response".to_vec(),
        );

        assert!(
            codec
                .validate_response_for_request(
                    &request,
                    &response,
                )
                .is_err()
        );
    }

    #[test]
    fn response_with_wrong_registry_is_rejected() {
        let codec = RegistryProtocolCodec::new();

        let request = request();

        let other_registry = RegistryId::new(
            IdNamespace::new("other").expect("namespace"),
            2,
        )
        .expect("registry ID");

        let response = RegistryResponse::new(
            other_registry,
            protocol(1, 0, 0),
            b"response".to_vec(),
        );

        assert!(
            codec
                .validate_response_for_request(
                    &request,
                    &response,
                )
                .is_err()
        );
    }

    #[test]
    fn unknown_envelope_fields_are_rejected() {
        let codec = RegistryProtocolCodec::new();

        let message = br#"{
            "message_type":"zamani.registry.request",
            "protocol_version":"1.0.0",
            "registry_id":"test:1",
            "operation":"packages.query",
            "payload_encoding":"hex",
            "payload":"00",
            "unexpected":"value"
        }"#;

        assert!(codec.decode_request(message).is_err());
    }

    #[test]
    fn incorrect_message_type_is_rejected() {
        let codec = RegistryProtocolCodec::new();

        let message = br#"{
            "message_type":"zamani.registry.response",
            "protocol_version":"1.0.0",
            "registry_id":"test:1",
            "operation":"packages.query",
            "payload_encoding":"hex",
            "payload":"00"
        }"#;

        assert!(codec.decode_request(message).is_err());
    }

    #[test]
    fn malformed_json_is_rejected() {
        let codec = RegistryProtocolCodec::new();

        assert!(
            codec
                .decode_request(b"{ malformed")
                .is_err()
        );
    }

    #[test]
    fn malformed_protocol_version_is_rejected() {
        let codec = RegistryProtocolCodec::new();

        let message = br#"{
            "message_type":"zamani.registry.request",
            "protocol_version":"1.2",
            "registry_id":"test:1",
            "operation":"packages.query",
            "payload_encoding":"hex",
            "payload":"00"
        }"#;

        assert!(codec.decode_request(message).is_err());
    }

    #[test]
    fn invalid_registry_identity_is_rejected() {
        let codec = RegistryProtocolCodec::new();

        let message = br#"{
            "message_type":"zamani.registry.request",
            "protocol_version":"1.0.0",
            "registry_id":"invalid",
            "operation":"packages.query",
            "payload_encoding":"hex",
            "payload":"00"
        }"#;

        assert!(codec.decode_request(message).is_err());
    }

    #[test]
    fn invalid_operation_identifier_is_rejected() {
        let codec = RegistryProtocolCodec::new();

        let message = br#"{
            "message_type":"zamani.registry.request",
            "protocol_version":"1.0.0",
            "registry_id":"test:1",
            "operation":"invalid operation",
            "payload_encoding":"hex",
            "payload":"00"
        }"#;

        assert!(codec.decode_request(message).is_err());
    }

    #[test]
    fn unsupported_payload_encoding_is_rejected() {
        let codec = RegistryProtocolCodec::new();

        let message = br#"{
            "message_type":"zamani.registry.request",
            "protocol_version":"1.0.0",
            "registry_id":"test:1",
            "operation":"packages.query",
            "payload_encoding":"base64",
            "payload":"AA=="
        }"#;

        assert!(codec.decode_request(message).is_err());
    }

    #[test]
    fn malformed_hex_payload_is_rejected() {
        let codec = RegistryProtocolCodec::new();

        let message = br#"{
            "message_type":"zamani.registry.request",
            "protocol_version":"1.0.0",
            "registry_id":"test:1",
            "operation":"packages.query",
            "payload_encoding":"hex",
            "payload":"not-hexadecimal"
        }"#;

        assert!(codec.decode_request(message).is_err());
    }

    #[test]
    fn negotiation_selects_highest_common_version() {
        let codec = RegistryProtocolCodec::new();

        let local = [
            range(protocol(1, 0, 0), protocol(2, 0, 0)),
        ];

        let remote = [
            range(protocol(1, 5, 0), protocol(3, 0, 0)),
        ];

        let negotiated = codec
            .negotiate(&local, &remote)
            .expect("compatible protocol");

        assert_eq!(
            negotiated.selected(),
            protocol(2, 0, 0),
        );
    }

    #[test]
    fn negotiation_supports_multiple_disjoint_ranges() {
        let codec = RegistryProtocolCodec::new();

        let local = [
            range(protocol(1, 0, 0), protocol(1, 5, 0)),
            range(protocol(3, 0, 0), protocol(4, 0, 0)),
        ];

        let remote = [
            range(protocol(1, 2, 0), protocol(1, 4, 0)),
            range(protocol(3, 1, 0), protocol(3, 8, 0)),
        ];

        let negotiated = codec
            .negotiate(&local, &remote)
            .expect("compatible ranges");

        assert_eq!(
            negotiated.selected(),
            protocol(3, 8, 0),
        );
    }

    #[test]
    fn negotiation_rejects_disjoint_ranges() {
        let codec = RegistryProtocolCodec::new();

        let local = [
            range(protocol(1, 0, 0), protocol(1, 5, 0)),
        ];

        let remote = [
            range(protocol(2, 0, 0), protocol(2, 5, 0)),
        ];

        assert!(
            codec
                .negotiate(&local, &remote)
                .is_err()
        );
    }

    #[test]
    fn negotiation_rejects_empty_support_declarations() {
        let codec = RegistryProtocolCodec::new();

        let local = [
            range(protocol(1, 0, 0), protocol(1, 5, 0)),
        ];

        assert!(
            codec
                .negotiate(&local, &[])
                .is_err()
        );

        assert!(
            codec
                .negotiate(&[], &local)
                .is_err()
        );
    }

    #[test]
    fn supported_version_validation_accepts_matching_version() {
        let codec = RegistryProtocolCodec::new();

        let supported = [
            range(protocol(1, 0, 0), protocol(2, 0, 0)),
        ];

        assert!(
            codec
                .validate_supported_version(
                    protocol(1, 5, 0),
                    &supported,
                )
                .is_ok()
        );
    }

    #[test]
    fn supported_version_validation_rejects_unknown_version() {
        let codec = RegistryProtocolCodec::new();

        let supported = [
            range(protocol(1, 0, 0), protocol(1, 5, 0)),
        ];

        assert!(
            codec
                .validate_supported_version(
                    protocol(2, 0, 0),
                    &supported,
                )
                .is_err()
        );
    }

    #[test]
    fn protocol_identifiers_are_not_domain_restricted() {
        let codec = RegistryProtocolCodec::new();

        let domains = BTreeSet::from([
            "classical".to_owned(),
            "quantum".to_owned(),
            "hdl".to_owned(),
            "hybrid".to_owned(),
            "distributed".to_owned(),
            "future.neuromorphic".to_owned(),
        ]);

        assert_eq!(domains.len(), 6);

        assert_eq!(
            codec,
            RegistryProtocolCodec::new(),
        );
    }
}
