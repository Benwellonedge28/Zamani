
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Distributed Transport Codec
//!
//! Path: src/distributed/transport/codec.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides safe, resource-aware serialization and deserialization for
//! distributed transport messages.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Message byte encoding.
//! - Message byte decoding.
//! - Configurable codec resource limits.
//! - Protocol-version compatibility checks.
//! - Wire representation validation.
//! - Decoded-message structural validation.
//! - Deterministic JSON serialization.
//!
//! This module does not own:
//!
//! - MessageEnvelope or WireMessageEnvelope.
//! - Logical message schemas.
//! - Protocol negotiation.
//! - Physical transport implementations.
//! - Network connections.
//! - Authentication or authorization.
//! - Encryption or cryptographic integrity.
//! - Task execution.
//! - Retry or exactly-once guarantees.
//! - Distributed scheduling or placement.
//! - Source-level message semantics.
//! - ZUIR, ZCIR, quantum::ir, or HDLIR.
//!
//! ## Authoritative dependencies
//!
//! - transport/messages.rs:
//!   MessageEnvelope, WireMessageEnvelope, ProtocolVersion.
//!
//! - transport/errors.rs:
//!   TransportError and TransportErrorKind.
//!
//! - distributed/error.rs:
//!   DeliveryState.
//!
//! - distributed/config.rs:
//!   ResourceBudgets.
//!
//! ## Trust boundary
//!
//! Every decoded message is untrusted.
//!
//! Successful decoding does not establish:
//!
//! - sender authenticity;
//! - authorization;
//! - message delivery;
//! - successful execution;
//! - exactly-once execution;
//! - payload semantic validity.
//!
//! These responsibilities belong to their respective subsystems.
//!
//! ## Scalability
//!
//! No universal message-size limit is imposed.
//!
//! Limits are optional, configurable operational budgets.
//!
//! Encoding uses a bounded writer when a wire-size budget is configured.
//! Decoding checks the input size before deserialization.
//!
//! ## Safety
//!
//! This implementation uses safe Rust only.

use std::io::{self, Write};
use std::num::NonZeroUsize;

use serde::Serialize;

use crate::distributed::config::ResourceBudgets;
use crate::distributed::error::DeliveryState;

use crate::distributed::transport::errors::{
    TransportError,
    TransportErrorContext,
    TransportErrorKind,
    TransportResult,
    TransportFailurePhase,
};

use crate::distributed::transport::messages::{
    MessageEnvelope,
    ProtocolVersion,
    WireMessageEnvelope,
};

/// Protocol compatibility policy.
///
/// This is a preliminary envelope-version check.
///
/// A matching major version does not establish that a particular
/// message kind, payload format, extension, or capability is supported.
///
/// Those checks remain the responsibility of the protocol and
/// application-level compatibility authorities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolCompatibility {
    /// Accept only the exact configured protocol version.
    Exact,

    /// Accept versions sharing the configured major version.
    ///
    /// Minor and patch compatibility must still be established by
    /// the applicable protocol and message-kind registries.
    SameMajor,
}

/// Optional operational limits for message encoding and decoding.
///
/// These limits do not restrict the Zamani language or define
/// universal limits on distributed computation.
///
/// `None` means this codec configuration does not impose
/// that particular limit.
///
/// Actual memory availability, address-space limits, and
/// implementation constraints still apply.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CodecLimits {
    max_wire_bytes: Option<NonZeroUsize>,
    max_payload_bytes: Option<NonZeroUsize>,
    max_extension_entries: Option<NonZeroUsize>,
    max_extension_bytes: Option<NonZeroUsize>,
}

impl CodecLimits {
    /// Creates codec limits without arbitrary ceilings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_wire_bytes: None,
            max_payload_bytes: None,
            max_extension_entries: None,
            max_extension_bytes: None,
        }
    }

    /// Creates codec limits from existing distributed resource budgets.
    ///
    /// The distributed configuration's maximum message size becomes
    /// the maximum encoded and decoded wire-message size.
    ///
    /// Other codec-specific limits remain unset unless configured
    /// separately.
    #[must_use]
    pub fn from_resource_budgets(budgets: &ResourceBudgets) -> Self {
        Self {
            max_wire_bytes: budgets.max_message_bytes(),
            max_payload_bytes: None,
            max_extension_entries: None,
            max_extension_bytes: None,
        }
    }

    /// Sets the optional maximum encoded or decoded wire size.
    #[must_use]
    pub fn with_max_wire_bytes(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_wire_bytes = limit;
        self
    }

    /// Sets the optional maximum opaque payload size.
    #[must_use]
    pub fn with_max_payload_bytes(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_payload_bytes = limit;
        self
    }

    /// Sets the optional maximum number of extension entries.
    #[must_use]
    pub fn with_max_extension_entries(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_extension_entries = limit;
        self
    }

    /// Sets the optional aggregate extension metadata byte budget.
    ///
    /// This counts extension keys and their opaque byte values.
    #[must_use]
    pub fn with_max_extension_bytes(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_extension_bytes = limit;
        self
    }

    /// Returns the configured wire-size budget.
    #[must_use]
    pub const fn max_wire_bytes(&self) -> Option<NonZeroUsize> {
        self.max_wire_bytes
    }

    /// Returns the configured payload budget.
    #[must_use]
    pub const fn max_payload_bytes(&self) -> Option<NonZeroUsize> {
        self.max_payload_bytes
    }

    /// Returns the configured extension-entry budget.
    #[must_use]
    pub const fn max_extension_entries(&self) -> Option<NonZeroUsize> {
        self.max_extension_entries
    }

    /// Returns the configured extension-byte budget.
    #[must_use]
    pub const fn max_extension_bytes(&self) -> Option<NonZeroUsize> {
        self.max_extension_bytes
    }
}

/// Distributed message codec.
///
/// The codec is independent of the actual transport implementation.
///
/// It can be used by TCP, QUIC, MPI, shared-memory transports,
/// accelerator communication providers, simulators, and future
/// communication mechanisms.
///
/// The type does not open connections or perform network I/O.
#[derive(Debug, Clone)]
pub struct DistributedMessageCodec {
    local_version: ProtocolVersion,
    compatibility: ProtocolCompatibility,
    limits: CodecLimits,
}

impl DistributedMessageCodec {
    /// Creates a codec with explicit protocol compatibility and limits.
    ///
    /// No protocol version is silently assumed.
    #[must_use]
    pub const fn new(
        local_version: ProtocolVersion,
        compatibility: ProtocolCompatibility,
        limits: CodecLimits,
    ) -> Self {
        Self {
            local_version,
            compatibility,
            limits,
        }
    }

    /// Creates a codec using the repository's distributed resource budgets.
    #[must_use]
    pub fn from_resource_budgets(
        local_version: ProtocolVersion,
        compatibility: ProtocolCompatibility,
        budgets: &ResourceBudgets,
    ) -> Self {
        Self::new(
            local_version,
            compatibility,
            CodecLimits::from_resource_budgets(budgets),
        )
    }

    /// Returns the local protocol version.
    #[must_use]
    pub const fn local_version(&self) -> ProtocolVersion {
        self.local_version
    }

    /// Returns the configured compatibility policy.
    #[must_use]
    pub const fn compatibility(&self) -> ProtocolCompatibility {
        self.compatibility
    }

    /// Returns the configured resource limits.
    #[must_use]
    pub const fn limits(&self) -> &CodecLimits {
        &self.limits
    }

    /// Encodes a validated distributed message into JSON wire bytes.
    ///
    /// The original MessageEnvelope is serialized directly.
    /// This avoids cloning its potentially large payload into
    /// an intermediate WireMessageEnvelope.
    ///
    /// When a wire-size budget exists, the writer refuses to
    /// append bytes beyond that budget.
    ///
    /// # Errors
    ///
    /// Returns a typed transport error if:
    ///
    /// - the message violates its structural invariants;
    /// - the configured payload budget is exceeded;
    /// - extension metadata exceeds its configured budget;
    /// - the encoded message exceeds its wire-size budget;
    /// - serialization fails.
    pub fn encode(
        &self,
        message: &MessageEnvelope,
    ) -> TransportResult<Vec<u8>> {
        message
            .validate()
            .map_err(|error| {
                Self::invalid_message(
                    "encode",
                    error.to_string(),
                )
            })?;

        self.validate_message_budgets(
            message.payload().len(),
            message.extensions(),
        )?;

        let mut writer = BoundedWriter::new(
            self.limits.max_wire_bytes,
        );

        match serde_json::to_writer(&mut writer, message) {
            Ok(()) => Ok(writer.into_inner()),

            Err(error) if writer.budget_exceeded() => {
                Err(Self::resource_exhausted(
                    "encode",
                    "encoded message exceeds the configured wire-size budget",
                ))
            }

            Err(error) => {
                Err(Self::serialization_error(
                    "encode",
                    error.to_string(),
                ))
            }
        }
    }

    /// Decodes untrusted JSON bytes into a validated message envelope.
    ///
    /// Processing order:
    ///
    /// 1. Check the input wire-size budget.
    /// 2. Deserialize the wire DTO.
    /// 3. Validate protocol compatibility.
    /// 4. Validate payload and extension budgets.
    /// 5. Convert the DTO into a validated MessageEnvelope.
    ///
    /// The returned envelope is structurally valid.
    ///
    /// Authentication, authorization, payload-schema validation,
    /// and semantic validation must still occur downstream.
    ///
    /// # Errors
    ///
    /// Returns a typed transport error if:
    ///
    /// - the input exceeds the configured wire budget;
    /// - JSON is malformed;
    /// - the protocol version is incompatible;
    /// - payload or extension budgets are exceeded;
    /// - message structural validation fails.
    pub fn decode(
        &self,
        bytes: &[u8],
    ) -> TransportResult<MessageEnvelope> {
        self.validate_wire_size(bytes.len(), "decode")?;

        let wire: WireMessageEnvelope =
            serde_json::from_slice(bytes).map_err(|error| {
                Self::deserialization_error(
                    "decode",
                    error.to_string(),
                )
            })?;

        self.validate_protocol_version(
            wire.protocol_version(),
        )?;

        // Validate payload and extension budgets before converting
        // the wire DTO into an application-facing envelope.
        //
        // The DTO's private fields intentionally prevent this codec
        // from duplicating its schema or bypassing its owner.
        //
        // Wire DTO conversion performs the authoritative structural
        // validation.
        let message = wire.into_envelope().map_err(|error| {
            Self::invalid_message(
                "decode",
                error.to_string(),
            )
        })?;

        self.validate_message_budgets(
            message.payload().len(),
            message.extensions(),
        )?;

        Ok(message)
    }

    /// Validates a protocol version against the configured policy.
    ///
    /// This is an envelope compatibility check, not a complete
    /// application protocol negotiation.
    pub fn validate_protocol_version(
        &self,
        remote: ProtocolVersion,
    ) -> TransportResult<()> {
        let compatible = match self.compatibility {
            ProtocolCompatibility::Exact => {
                self.local_version == remote
            }

            ProtocolCompatibility::SameMajor => {
                self.local_version.major() == remote.major()
            }
        };

        if compatible {
            Ok(())
        } else {
            Err(
                TransportError::with_context(
                    TransportErrorKind::ProtocolIncompatible,
                    DeliveryState::NotSent,
                    TransportErrorContext::new()
                        .with_operation("validate_protocol_version")
                        .with_phase(TransportFailurePhase::Negotiation),
                    format!(
                        "incompatible distributed protocol version: local {}, remote {}",
                        self.local_version,
                        remote
                    ),
                )
            )
        }
    }

    fn validate_wire_size(
        &self,
        actual: usize,
        operation: &'static str,
    ) -> TransportResult<()> {
        if let Some(limit) = self.limits.max_wire_bytes {
            if actual > limit.get() {
                return Err(Self::resource_exhausted(
                    operation,
                    "wire message exceeds the configured byte budget",
                ));
            }
        }

        Ok(())
    }

    fn validate_message_budgets(
        &self,
        payload_bytes: usize,
        extensions: &std::collections::BTreeMap<String, Vec<u8>>,
    ) -> TransportResult<()> {
        if let Some(limit) = self.limits.max_payload_bytes {
            if payload_bytes > limit.get() {
                return Err(Self::resource_exhausted(
                    "validate_message",
                    "payload exceeds the configured byte budget",
                ));
            }
        }

        if let Some(limit) = self.limits.max_extension_entries {
            if extensions.len() > limit.get() {
                return Err(Self::resource_exhausted(
                    "validate_message",
                    "extension count exceeds the configured budget",
                ));
            }
        }

        if let Some(limit) = self.limits.max_extension_bytes {
            let total = extensions.iter().try_fold(
                0usize,
                |total, (key, value)| {
                    total
                        .checked_add(key.len())
                        .and_then(|n| n.checked_add(value.len()))
                },
            );

            let total = total.ok_or_else(|| {
                Self::resource_exhausted(
                    "validate_message",
                    "extension metadata size overflowed its representation",
                )
            })?;

            if total > limit.get() {
                return Err(Self::resource_exhausted(
                    "validate_message",
                    "extension metadata exceeds the configured byte budget",
                ));
            }
        }

        Ok(())
    }

    fn invalid_message(
        operation: &'static str,
        message: impl Into<String>,
    ) -> TransportError {
        TransportError::with_context(
            TransportErrorKind::InvalidMessage,
            DeliveryState::NotSent,
            TransportErrorContext::new()
                .with_operation(operation)
                .with_phase(TransportFailurePhase::Decoding),
            message,
        )
    }

    fn serialization_error(
        operation: &'static str,
        message: impl Into<String>,
    ) -> TransportError {
        TransportError::with_context(
            TransportErrorKind::SerializationFailure,
            DeliveryState::NotSent,
            TransportErrorContext::new()
                .with_operation(operation)
                .with_phase(TransportFailurePhase::Encoding),
            message,
        )
    }

    fn deserialization_error(
        operation: &'static str,
        message: impl Into<String>,
    ) -> TransportError {
        TransportError::with_context(
            TransportErrorKind::DeserializationFailure,
            DeliveryState::NotSent,
            TransportErrorContext::new()
                .with_operation(operation)
                .with_phase(TransportFailurePhase::Decoding),
            message,
        )
    }

    fn resource_exhausted(
        operation: &'static str,
        message: impl Into<String>,
    ) -> TransportError {
        TransportError::with_context(
            TransportErrorKind::ResourceExhausted,
            DeliveryState::NotSent,
            TransportErrorContext::new()
                .with_operation(operation)
                .with_phase(TransportFailurePhase::Encoding),
            message,
        )
    }
}

/// A vector-backed writer that enforces an optional output budget.
///
/// Unlike serializing into an unrestricted Vec and checking afterward,
/// this writer refuses a write that would exceed the configured budget.
///
/// It does not reserve a fixed amount of memory or impose a universal cap.
struct BoundedWriter {
    bytes: Vec<u8>,
    limit: Option<NonZeroUsize>,
    exceeded: bool,
}

impl BoundedWriter {
    fn new(limit: Option<NonZeroUsize>) -> Self {
        Self {
            bytes: Vec::new(),
            limit,
            exceeded: false,
        }
    }

    fn budget_exceeded(&self) -> bool {
        self.exceeded
    }

    fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for BoundedWriter {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        let new_length = self.bytes.len().checked_add(input.len());

        let Some(new_length) = new_length else {
            self.exceeded = true;

            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "encoded message size overflow",
            ));
        };

        if let Some(limit) = self.limit {
            if new_length > limit.get() {
                self.exceeded = true;

                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "encoded message exceeds configured budget",
                ));
            }
        }

        self.bytes.extend_from_slice(input);

        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::distributed::ids::{
        CorrelationId,
        MessageId,
        ParticipantId,
    };

    use crate::distributed::transport::messages::{
        MessageEnvelope,
        MessageEnvelopeParts,
        MessageExtensions,
        MessageKind,
        MessageRole,
    };

    fn codec() -> DistributedMessageCodec {
        DistributedMessageCodec::new(
            ProtocolVersion::new(1, 0, 0),
            ProtocolCompatibility::SameMajor,
            CodecLimits::new(),
        )
    }

    fn request() -> MessageEnvelope {
        MessageEnvelope::new(MessageEnvelopeParts {
            protocol_version: ProtocolVersion::new(1, 0, 0),
            message_id: MessageId::new(1, 1).unwrap(),
            kind: MessageKind::new(
                "zamani",
                "task_request",
            )
            .unwrap(),
            role: MessageRole::Request,
            sender: ParticipantId::new(1, 2).unwrap(),
            recipient: Some(
                ParticipantId::new(1, 3).unwrap(),
            ),
            correlation_id: Some(
                CorrelationId::new(1, 4).unwrap(),
            ),
            in_reply_to: None,
            operation_id: None,
            task_id: None,
            payload_format: "application/json".to_owned(),
            payload: br#"{"work":"example"}"#.to_vec(),
            extensions: MessageExtensions::new(),
        })
        .unwrap()
    }

    #[test]
    fn round_trip_preserves_message() {
        let codec = codec();
        let original = request();

        let encoded = codec.encode(&original).unwrap();
        let decoded = codec.decode(&encoded).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn rejects_malformed_json() {
        let result = codec().decode(b"{invalid-json");

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert_eq!(
            error.kind(),
            TransportErrorKind::DeserializationFailure
        );
    }

    #[test]
    fn rejects_incompatible_protocol_major() {
        let codec = codec();

        let result = codec.validate_protocol_version(
            ProtocolVersion::new(2, 0, 0),
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            TransportErrorKind::ProtocolIncompatible
        );
    }

    #[test]
    fn exact_policy_rejects_minor_difference() {
        let codec = DistributedMessageCodec::new(
            ProtocolVersion::new(1, 0, 0),
            ProtocolCompatibility::Exact,
            CodecLimits::new(),
        );

        assert!(
            codec
                .validate_protocol_version(
                    ProtocolVersion::new(1, 1, 0)
                )
                .is_err()
        );
    }

    #[test]
    fn same_major_policy_accepts_minor_difference() {
        let codec = codec();

        assert!(
            codec
                .validate_protocol_version(
                    ProtocolVersion::new(1, 9, 4)
                )
                .is_ok()
        );
    }

    #[test]
    fn rejects_wire_message_over_budget() {
        let limits = CodecLimits::new()
            .with_max_wire_bytes(
                NonZeroUsize::new(8),
            );

        let codec = DistributedMessageCodec::new(
            ProtocolVersion::new(1, 0, 0),
            ProtocolCompatibility::SameMajor,
            limits,
        );

        let result = codec.decode(
            br#"{"message":"too large"}"#,
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            TransportErrorKind::ResourceExhausted
        );
    }

    #[test]
    fn rejects_payload_over_budget() {
        let limits = CodecLimits::new()
            .with_max_payload_bytes(
                NonZeroUsize::new(4),
            );

        let codec = DistributedMessageCodec::new(
            ProtocolVersion::new(1, 0, 0),
            ProtocolCompatibility::SameMajor,
            limits,
        );

        let result = codec.encode(&request());

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            TransportErrorKind::ResourceExhausted
        );
    }

    #[test]
    fn rejects_extension_count_over_budget() {
        let mut message = request();

        // Construct a second envelope through the existing authoritative
        // constructor so the test never bypasses message validation.
        let mut extensions = MessageExtensions::new();
        extensions.insert(
            "zamani.trace".to_owned(),
            vec![1, 2, 3],
        );

        extensions.insert(
            "zamani.provenance".to_owned(),
            vec![4, 5, 6],
        );

        let mut parts = MessageEnvelopeParts {
            protocol_version: message.protocol_version(),
            message_id: message.message_id(),
            kind: message.kind().clone(),
            role: message.role(),
            sender: message.sender(),
            recipient: message.recipient(),
            correlation_id: message.correlation_id(),
            in_reply_to: message.in_reply_to(),
            operation_id: message.operation_id(),
            task_id: message.task_id(),
            payload_format: message.payload_format().to_owned(),
            payload: message.payload().to_vec(),
            extensions,
        };

        message = MessageEnvelope::new(parts).unwrap();

        let limits = CodecLimits::new()
            .with_max_extension_entries(
                NonZeroUsize::new(1),
            );

        let codec = DistributedMessageCodec::new(
            ProtocolVersion::new(1, 0, 0),
            ProtocolCompatibility::SameMajor,
            limits,
        );

        let result = codec.encode(&message);

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            TransportErrorKind::ResourceExhausted
        );
    }

    #[test]
    fn rejects_extension_bytes_over_budget() {
        let mut extensions = MessageExtensions::new();

        extensions.insert(
            "zamani.trace".to_owned(),
            vec![1, 2, 3, 4, 5],
        );

        let mut parts = MessageEnvelopeParts {
            protocol_version: ProtocolVersion::new(1, 0, 0),
            message_id: MessageId::new(1, 10).unwrap(),
            kind: MessageKind::new(
                "zamani",
                "task_request",
            )
            .unwrap(),
            role: MessageRole::Request,
            sender: ParticipantId::new(1, 11).unwrap(),
            recipient: Some(
                ParticipantId::new(1, 12).unwrap(),
            ),
            correlation_id: Some(
                CorrelationId::new(1, 13).unwrap(),
            ),
            in_reply_to: None,
            operation_id: None,
            task_id: None,
            payload_format: "application/json".to_owned(),
            payload: Vec::new(),
            extensions,
        };

        let message = MessageEnvelope::new(parts).unwrap();

        let limits = CodecLimits::new()
            .with_max_extension_bytes(
                NonZeroUsize::new(8),
            );

        let codec = DistributedMessageCodec::new(
            ProtocolVersion::new(1, 0, 0),
            ProtocolCompatibility::SameMajor,
            limits,
        );

        let result = codec.encode(&message);

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            TransportErrorKind::ResourceExhausted
        );
    }

    #[test]
    fn bounded_writer_never_exceeds_its_budget() {
        let mut writer = BoundedWriter::new(
            NonZeroUsize::new(4),
        );

        writer.write_all(b"1234").unwrap();

        assert!(writer.write_all(b"5").is_err());

        assert!(writer.budget_exceeded());

        assert_eq!(writer.into_inner(), b"1234");
    }

    #[test]
    fn unbounded_writer_accepts_resource_available_output() {
        let mut writer = BoundedWriter::new(None);

        writer.write_all(b"portable").unwrap();

        assert_eq!(writer.into_inner(), b"portable");
    }
}
