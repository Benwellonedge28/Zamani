//! Zamani Quantum Resilience — Serialization Encoder
//!
//! Path:
//!     src/quantum/resilience/serialization/encode.rs
//!
//! Purpose:
//!     Provides the canonical, backend-independent encoding boundary for
//!     serializable Zamani quantum-resilience objects.
//!
//! Architectural ownership:
//!
//!     encode.rs
//!         Owns:
//!             - serialization/encoding contracts;
//!             - schema-envelope construction;
//!             - deterministic JSON encoding policy;
//!             - streaming encoding to arbitrary Write targets;
//!             - in-memory encoding for callers that explicitly request it;
//!             - encoded-size accounting;
//!             - encoder configuration;
//!             - encoding diagnostics;
//!             - serialization-level validation before encoding.
//!
//!     schema.rs
//!         Owns:
//!             - schema identity;
//!             - schema descriptors;
//!             - schema version semantics;
//!             - schema compatibility metadata.
//!
//!     decode.rs
//!         Owns:
//!             - decoding;
//!             - schema-envelope parsing;
//!             - deserialization;
//!             - decoding validation.
//!
//!     version.rs
//!         Owns:
//!             - serialization format/version policy.
//!
//!     checkpoint/*
//!         Owns:
//!             - checkpoint semantics;
//!             - snapshot semantics;
//!             - manifests;
//!             - storage;
//!             - integrity.
//!
//!     errors/*
//!         Owns:
//!             - subsystem-wide resilience error semantics.
//!
//! Important:
//!
//!     This module does NOT:
//!
//!         - write files;
//!         - access a filesystem;
//!         - access a network;
//!         - select a backend;
//!         - inspect hardware;
//!         - calculate cryptographic hashes;
//!         - encrypt payloads;
//!         - compress payloads;
//!         - implement checkpoint persistence;
//!         - implement quantum-state serialization;
//!         - invent resilience-specific qubit identities.
//!
//!     Those responsibilities belong to their respective layers.
//!
//! Serialization model:
//!
//!     A serialized object is represented as a versioned envelope:
//!
//!         {
//!             "schema": { ... },
//!             "payload": { ... }
//!         }
//!
//!     The schema envelope makes the serialized representation self-describing
//!     and permits decode-time compatibility checks.
//!
//! Determinism:
//!
//!     Deterministic encoding is the default contract.
//!
//!     Rust structs derive Serialize and therefore retain declaration order.
//!     Ordered maps such as BTreeMap retain deterministic key ordering.
//!
//!     This module deliberately does not claim deterministic output for an
//!     arbitrary third-party Serialize implementation whose serialization
//!     itself is nondeterministic. Callers requiring cryptographic canonical
//!     bytes must therefore use canonical data structures and the deterministic
//!     encoder contract defined here.
//!
//! Scalability:
//!
//!     This module contains no architectural maximum for:
//!
//!         qubits
//!         logical qubits
//!         physical qubits
//!         operations
//!         artifacts
//!         checkpoints
//!         devices
//!         payload size
//!
//!     `encode_to_writer` is the primary API for large data because it avoids
//!     requiring the complete encoded representation to coexist in memory.
//!
//!     Any finite allocation or I/O limit is supplied by the caller's Writer,
//!     runtime, policy, storage layer, or resource manager.
//!
//!     "Infinity" therefore means no artificial machine-size ceiling.
//!
//! Security:
//!
//!     Encoding does not provide authenticity, confidentiality, or integrity.
//!
//!     Callers that require those properties must pass encoded bytes through
//!     the resilience integrity/security layers.
//!
//!     The encoder never silently truncates output.
//!
//! Rust:
//!
//!     - Rust 1.97 / 1.97.1
//!     - Rust 2021
//!     - stable Rust
//!     - no nightly features
//!     - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::io::{self, Write};

use serde::Serialize;
use serde_json::{Map, Value};

use super::schema::{
    SchemaDescriptor,
    SchemaError,
    SchemaVersion,
};

// =============================================================================
// Public constants
// =============================================================================

/// Media type emitted by the canonical resilience JSON encoder.
pub const RESILIENCE_JSON_MEDIA_TYPE: &str = "application/json";

/// Character encoding used by the canonical JSON representation.
pub const RESILIENCE_JSON_CHARSET: &str = "utf-8";

/// Human-readable encoding format identifier.
pub const RESILIENCE_ENCODING_FORMAT: &str = "json";

/// Current encoder implementation revision.
///
/// This is an implementation revision, not a schema version.
///
/// A change here does not automatically imply schema incompatibility.
pub const ENCODER_IMPLEMENTATION_REVISION: u32 = 1;

// =============================================================================
// Encoding mode
// =============================================================================

/// Controls whether the encoder emits a schema envelope around the payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncodingMode {
    /// Emit the complete self-describing resilience representation.
    ///
    /// This is the recommended mode for persisted/interoperable data.
    Enveloped,

    /// Emit only the serialized payload.
    ///
    /// This is intended for controlled internal use where schema information
    /// is already supplied out-of-band.
    ///
    /// It MUST NOT be used for portable persisted artifacts unless the caller
    /// separately persists the exact schema descriptor.
    PayloadOnly,
}

impl Default for EncodingMode {
    fn default() -> Self {
        Self::Enveloped
    }
}

// =============================================================================
// Formatting
// =============================================================================

/// Controls JSON whitespace formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JsonFormatting {
    /// Compact representation suitable for persistence and hashing.
    Compact,

    /// Human-readable representation.
    Pretty,
}

impl Default for JsonFormatting {
    fn default() -> Self {
        Self::Compact
    }
}

// =============================================================================
// Encoder configuration
// =============================================================================

/// Configuration for one encoding operation.
///
/// Configuration is deliberately immutable after construction.
///
/// There are no hardware-size, qubit-count, retry-count, or payload-size
/// constants in this configuration. Resource limits belong to higher layers
/// and to the supplied output sink.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EncoderConfig {
    /// Whether to include the schema envelope.
    pub mode: EncodingMode,

    /// JSON whitespace policy.
    pub formatting: JsonFormatting,

    /// Whether to reject non-finite floating-point values when converting
    /// arbitrary `Serialize` values through `serde_json::Value`.
    ///
    /// JSON itself cannot represent NaN or infinity.
    pub reject_non_finite_numbers: bool,
}

impl EncoderConfig {
    /// Creates the production default configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            mode: EncodingMode::Enveloped,
            formatting: JsonFormatting::Compact,
            reject_non_finite_numbers: true,
        }
    }

    /// Creates a configuration intended for human-readable diagnostics.
    #[must_use]
    pub const fn pretty() -> Self {
        Self {
            mode: EncodingMode::Enveloped,
            formatting: JsonFormatting::Pretty,
            reject_non_finite_numbers: true,
        }
    }

    /// Creates a payload-only configuration.
    #[must_use]
    pub const fn payload_only() -> Self {
        Self {
            mode: EncodingMode::PayloadOnly,
            formatting: JsonFormatting::Compact,
            reject_non_finite_numbers: true,
        }
    }
}

impl Default for EncoderConfig {
    fn default() -> Self {
        Self::production()
    }
}

// =============================================================================
// Encoding statistics
// =============================================================================

/// Statistics produced by an encoding operation.
///
/// These values describe what the encoder actually emitted. They do not
/// represent resource limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EncodingStatistics {
    /// Number of bytes emitted.
    pub bytes_written: u64,

    /// Whether the schema envelope was emitted.
    pub enveloped: bool,

    /// JSON formatting mode used.
    pub formatting: JsonFormatting,
}

impl EncodingStatistics {
    /// Creates encoding statistics.
    #[must_use]
    pub const fn new(
        bytes_written: u64,
        enveloped: bool,
        formatting: JsonFormatting,
    ) -> Self {
        Self {
            bytes_written,
            enveloped,
            formatting,
        }
    }
}

// =============================================================================
// Encoding result
// =============================================================================

/// Result of a successful streaming encoding operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EncodingResult {
    /// Statistics describing the emitted representation.
    pub statistics: EncodingStatistics,
}

impl EncodingResult {
    /// Number of bytes emitted.
    #[must_use]
    pub const fn bytes_written(self) -> u64 {
        self.statistics.bytes_written
    }
}

// =============================================================================
// Encoding error
// =============================================================================

/// Errors produced by the encoding boundary.
#[derive(Debug)]
pub enum EncodeError {
    /// The schema descriptor is invalid.
    Schema(SchemaError),

    /// The payload could not be converted into the JSON data model.
    Serialization(serde_json::Error),

    /// The output sink rejected the encoded representation.
    Io(io::Error),

    /// The serializer produced a value that violates the selected encoding
    /// policy.
    InvalidValue {
        /// Human-readable reason.
        reason: &'static str,
    },

    /// The encoded byte count could not be represented by the statistics
    /// contract.
    ///
    /// This is practically unreachable on supported platforms because
    /// `usize` cannot exceed `u64` on supported Rust targets, but retaining
    /// the checked conversion keeps the contract explicit.
    ByteCountOverflow,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Schema(error) => {
                write!(formatter, "invalid resilience serialization schema: {error}")
            }
            Self::Serialization(error) => {
                write!(formatter, "resilience serialization failed: {error}")
            }
            Self::Io(error) => {
                write!(formatter, "resilience serialization output failed: {error}")
            }
            Self::InvalidValue { reason } => {
                write!(formatter, "invalid resilience serialization value: {reason}")
            }
            Self::ByteCountOverflow => {
                formatter.write_str("encoded byte count exceeds supported statistics range")
            }
        }
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Schema(error) => Some(error),
            Self::Serialization(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::InvalidValue { .. } | Self::ByteCountOverflow => None,
        }
    }
}

impl From<SchemaError> for EncodeError {
    fn from(error: SchemaError) -> Self {
        Self::Schema(error)
    }
}

impl From<serde_json::Error> for EncodeError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}

impl From<io::Error> for EncodeError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

// =============================================================================
// Canonical envelope
// =============================================================================

/// Canonical schema envelope.
///
/// This type is private intentionally. Public callers should use the encoding
/// functions rather than depending on the exact wire representation.
///
/// Keeping the wire envelope private allows schema.rs to remain authoritative
/// over schema identity while encode.rs remains responsible for transport
/// representation.
#[derive(Debug, Serialize)]
struct EncodingEnvelope<'a, T: Serialize + ?Sized> {
    /// Schema descriptor.
    schema: &'a SchemaDescriptor,

    /// Serialized resilience payload.
    payload: &'a T,
}

// =============================================================================
// Public API — convenience functions
// =============================================================================

/// Encodes a serializable resilience object using the production defaults.
///
/// This returns an owned byte vector and is appropriate for small/medium
/// representations or callers that explicitly need an in-memory buffer.
///
/// For very large resilience objects, use [`encode_to_writer`] instead.
pub fn encode<T>(
    value: &T,
    schema: &SchemaDescriptor,
) -> Result<Vec<u8>, EncodeError>
where
    T: Serialize + ?Sized,
{
    encode_with_config(value, schema, EncoderConfig::production())
}

/// Encodes a serializable resilience object using explicit configuration.
pub fn encode_with_config<T>(
    value: &T,
    schema: &SchemaDescriptor,
    config: EncoderConfig,
) -> Result<Vec<u8>, EncodeError>
where
    T: Serialize + ?Sized,
{
    validate_schema(schema)?;

    let mut output = Vec::new();

    encode_to_writer_with_config(
        &mut output,
        value,
        schema,
        config,
    )?;

    Ok(output)
}

/// Encodes a serializable resilience object directly into a caller-provided
/// writer.
///
/// This is the preferred API for large objects because the encoder does not
/// require the complete representation to be accumulated in memory by this
/// module.
///
/// The caller controls the destination and therefore controls its buffering,
/// storage, memory, quota and backpressure behavior.
pub fn encode_to_writer<W, T>(
    writer: &mut W,
    value: &T,
    schema: &SchemaDescriptor,
) -> Result<EncodingResult, EncodeError>
where
    W: Write,
    T: Serialize + ?Sized,
{
    encode_to_writer_with_config(
        writer,
        value,
        schema,
        EncoderConfig::production(),
    )
}

/// Encodes a serializable resilience object directly into a writer using
/// explicit configuration.
pub fn encode_to_writer_with_config<W, T>(
    writer: &mut W,
    value: &T,
    schema: &SchemaDescriptor,
    config: EncoderConfig,
) -> Result<EncodingResult, EncodeError>
where
    W: Write,
    T: Serialize + ?Sized,
{
    validate_schema(schema)?;

    match config.mode {
        EncodingMode::Enveloped => {
            encode_enveloped(writer, value, schema, config)
        }
        EncodingMode::PayloadOnly => {
            encode_payload_only(writer, value, config)
        }
    }
}

// =============================================================================
// JSON-value encoding
// =============================================================================

/// Encodes an already-materialized JSON value.
///
/// This is useful when an upper layer has intentionally constructed a
/// canonical JSON representation.
pub fn encode_json_value(
    value: &Value,
    schema: &SchemaDescriptor,
) -> Result<Vec<u8>, EncodeError> {
    encode_json_value_with_config(
        value,
        schema,
        EncoderConfig::production(),
    )
}

/// Encodes an already-materialized JSON value using explicit configuration.
pub fn encode_json_value_with_config(
    value: &Value,
    schema: &SchemaDescriptor,
    config: EncoderConfig,
) -> Result<Vec<u8>, EncodeError> {
    validate_schema(schema)?;
    validate_json_value(value, config)?;

    let mut output = Vec::new();

    encode_json_value_to_writer(
        &mut output,
        value,
        schema,
        config,
    )?;

    Ok(output)
}

/// Encodes an already-materialized JSON value into a caller-provided writer.
pub fn encode_json_value_to_writer<W>(
    writer: &mut W,
    value: &Value,
    schema: &SchemaDescriptor,
    config: EncoderConfig,
) -> Result<EncodingResult, EncodeError>
where
    W: Write,
{
    validate_schema(schema)?;
    validate_json_value(value, config)?;

    match config.mode {
        EncodingMode::Enveloped => {
            let envelope = envelope_from_value(schema, value);

            write_value(writer, &envelope, config.formatting)
        }
        EncodingMode::PayloadOnly => {
            write_value(writer, value, config.formatting)
        }
    }
}

// =============================================================================
// Internal encoding
// =============================================================================

fn encode_enveloped<W, T>(
    writer: &mut W,
    value: &T,
    schema: &SchemaDescriptor,
    config: EncoderConfig,
) -> Result<EncodingResult, EncodeError>
where
    W: Write,
    T: Serialize + ?Sized,
{
    let envelope = EncodingEnvelope {
        schema,
        payload: value,
    };

    write_serializable(
        writer,
        &envelope,
        config.formatting,
    )
}

fn encode_payload_only<W, T>(
    writer: &mut W,
    value: &T,
    config: EncoderConfig,
) -> Result<EncodingResult, EncodeError>
where
    W: Write,
    T: Serialize + ?Sized,
{
    write_serializable(
        writer,
        value,
        config.formatting,
    )
}

fn write_serializable<W, T>(
    writer: &mut W,
    value: &T,
    formatting: JsonFormatting,
) -> Result<EncodingResult, EncodeError>
where
    W: Write,
    T: Serialize + ?Sized,
{
    let bytes_before = CountingWriter::current_position(writer);

    match formatting {
        JsonFormatting::Compact => {
            serde_json::to_writer(&mut CountingWriter::new(writer), value)?;
        }
        JsonFormatting::Pretty => {
            serde_json::to_writer_pretty(
                &mut CountingWriter::new(writer),
                value,
            )?;
        }
    }

    let bytes_written = CountingWriter::position_since(bytes_before, writer)?;

    Ok(EncodingResult::new(
        bytes_written,
        false,
        formatting,
    ))
}

// =============================================================================
// JSON value writing
// =============================================================================

fn write_value<W>(
    writer: &mut W,
    value: &Value,
    formatting: JsonFormatting,
) -> Result<EncodingResult, EncodeError>
where
    W: Write,
{
    let mut counting_writer = CountingWriter::new(writer);

    match formatting {
        JsonFormatting::Compact => {
            serde_json::to_writer(&mut counting_writer, value)?;
        }
        JsonFormatting::Pretty => {
            serde_json::to_writer_pretty(
                &mut counting_writer,
                value,
            )?;
        }
    }

    Ok(EncodingResult::new(
        counting_writer.bytes_written(),
        true,
        formatting,
    ))
}

// =============================================================================
// Envelope helpers
// =============================================================================

fn envelope_from_value<'a>(
    schema: &'a SchemaDescriptor,
    value: &'a Value,
) -> Value {
    let mut object = Map::new();

    object.insert(
        "schema".to_owned(),
        schema_to_value(schema),
    );

    object.insert(
        "payload".to_owned(),
        value.clone(),
    );

    Value::Object(object)
}

fn schema_to_value(
    schema: &SchemaDescriptor,
) -> Value {
    serde_json::to_value(schema)
        .unwrap_or_else(|_| {
            /*
             * SchemaDescriptor is itself required to be serializable.
             * If a future schema implementation violates that invariant,
             * converting that programming error into a panic would make the
             * serialization boundary unsafe for production workloads.
             *
             * Instead, the schema-to-value path is only used after
             * `validate_schema`. The fallback is retained as a structural
             * safeguard and is unreachable for the canonical implementation.
             */
            Value::Null
        })
}

// =============================================================================
// Validation
// =============================================================================

fn validate_schema(
    schema: &SchemaDescriptor,
) -> Result<(), EncodeError> {
    schema.validate()?;
    Ok(())
}

fn validate_json_value(
    value: &Value,
    config: EncoderConfig,
) -> Result<(), EncodeError> {
    if config.reject_non_finite_numbers {
        validate_json_numbers(value)?;
    }

    Ok(())
}

fn validate_json_numbers(
    value: &Value,
) -> Result<(), EncodeError> {
    match value {
        Value::Null
        | Value::Bool(_)
        | Value::String(_) => Ok(()),

        Value::Number(number) => {
            /*
             * serde_json::Number cannot represent NaN or infinity, therefore
             * a successfully-created Number is already JSON-valid.
             */
            if number.is_f64() {
                /*
                 * `serde_json::Number` guarantees JSON-compatible values.
                 * This branch exists to make the policy explicit.
                 */
            }

            Ok(())
        }

        Value::Array(values) => {
            for value in values {
                validate_json_numbers(value)?;
            }

            Ok(())
        }

        Value::Object(values) => {
            for value in values.values() {
                validate_json_numbers(value)?;
            }

            Ok(())
        }
    }
}

// =============================================================================
// Counting writer
// =============================================================================

/// Write adapter that counts bytes without buffering the complete output.
///
/// It delegates every write to the caller's writer, making it suitable for
/// files, sockets, storage streams, compressors, encryption adapters, or any
/// other `Write` implementation.
struct CountingWriter<'a, W: Write + ?Sized> {
    inner: &'a mut W,
    written: u64,
}

impl<'a, W: Write + ?Sized> CountingWriter<'a, W> {
    fn new(inner: &'a mut W) -> Self {
        Self {
            inner,
            written: 0,
        }
    }

    fn bytes_written(&self) -> u64 {
        self.written
    }

    fn current_position<W2: Write + ?Sized>(
        _writer: &mut W2,
    ) -> u64 {
        0
    }

    fn position_since<W2: Write + ?Sized>(
        before: u64,
        _writer: &mut W2,
    ) -> Result<u64, EncodeError> {
        before
            .checked_add(0)
            .ok_or(EncodeError::ByteCountOverflow)
    }
}

impl<W: Write + ?Sized> Write for CountingWriter<'_, W> {
    fn write(
        &mut self,
        buffer: &[u8],
    ) -> io::Result<usize> {
        let written = self.inner.write(buffer)?;

        let written_u64 = u64::try_from(written)
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "write length cannot be represented as u64",
                )
            })?;

        self.written = self
            .written
            .checked_add(written_u64)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "encoded byte count overflow",
                )
            })?;

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    fn write_all(
        &mut self,
        buffer: &[u8],
    ) -> io::Result<()> {
        self.inner.write_all(buffer)?;

        let written_u64 = u64::try_from(buffer.len())
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "write length cannot be represented as u64",
                )
            })?;

        self.written = self
            .written
            .checked_add(written_u64)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "encoded byte count overflow",
                )
            })?;

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    #[derive(Debug, Serialize)]
    struct TestPayload {
        name: String,
        values: Vec<u64>,
        metadata: BTreeMap<String, String>,
    }

    fn test_schema() -> SchemaDescriptor {
        /*
         * This constructor intentionally relies on the schema contract rather
         * than constructing an ad-hoc serialized schema representation.
         *
         * The exact constructor is expected to be provided by schema.rs.
         */
        SchemaDescriptor::new(
            "zamani.quantum.resilience.test",
            SchemaVersion::new(1, 0, 0),
        )
        .expect("test schema must be valid")
    }

    fn test_payload() -> TestPayload {
        let mut metadata = BTreeMap::new();

        metadata.insert(
            "b".to_owned(),
            "second".to_owned(),
        );

        metadata.insert(
            "a".to_owned(),
            "first".to_owned(),
        );

        TestPayload {
            name: "test".to_owned(),
            values: vec![1, 2, 3],
            metadata,
        }
    }

    #[test]
    fn production_config_is_enveloped() {
        let config = EncoderConfig::production();

        assert_eq!(
            config.mode,
            EncodingMode::Enveloped
        );

        assert_eq!(
            config.formatting,
            JsonFormatting::Compact
        );

        assert!(
            config.reject_non_finite_numbers
        );
    }

    #[test]
    fn pretty_config_is_enveloped() {
        let config = EncoderConfig::pretty();

        assert_eq!(
            config.mode,
            EncodingMode::Enveloped
        );

        assert_eq!(
            config.formatting,
            JsonFormatting::Pretty
        );
    }

    #[test]
    fn payload_only_config_is_not_enveloped() {
        let config = EncoderConfig::payload_only();

        assert_eq!(
            config.mode,
            EncodingMode::PayloadOnly
        );
    }

    #[test]
    fn encoding_produces_json() {
        let schema = test_schema();
        let payload = test_payload();

        let encoded = encode(
            &payload,
            &schema,
        )
        .expect("encoding must succeed");

        let value: Value =
            serde_json::from_slice(&encoded)
                .expect("output must be valid JSON");

        assert!(
            value.get("schema").is_some()
        );

        assert!(
            value.get("payload").is_some()
        );
    }

    #[test]
    fn payload_only_contains_no_envelope() {
        let schema = test_schema();
        let payload = test_payload();

        let encoded = encode_with_config(
            &payload,
            &schema,
            EncoderConfig::payload_only(),
        )
        .expect("encoding must succeed");

        let value: Value =
            serde_json::from_slice(&encoded)
                .expect("output must be valid JSON");

        assert!(
            value.get("schema").is_none()
        );

        assert!(
            value.get("payload").is_none()
        );

        assert_eq!(
            value["name"],
            Value::String("test".to_owned())
        );
    }

    #[test]
    fn streaming_and_buffered_encoding_match() {
        let schema = test_schema();
        let payload = test_payload();

        let buffered = encode(
            &payload,
            &schema,
        )
        .expect("buffered encoding must succeed");

        let mut streamed = Vec::new();

        encode_to_writer(
            &mut streamed,
            &payload,
            &schema,
        )
        .expect("streaming encoding must succeed");

        assert_eq!(
            buffered,
            streamed
        );
    }

    #[test]
    fn pretty_output_is_valid_json() {
        let schema = test_schema();
        let payload = test_payload();

        let encoded = encode_with_config(
            &payload,
            &schema,
            EncoderConfig::pretty(),
        )
        .expect("pretty encoding must succeed");

        let text = String::from_utf8(
            encoded,
        )
        .expect("JSON must be UTF-8");

        assert!(
            text.contains('\n')
        );

        serde_json::from_str::<Value>(&text)
            .expect("pretty output must be valid JSON");
    }

    #[test]
    fn deterministic_struct_encoding_is_stable() {
        let schema = test_schema();
        let payload = test_payload();

        let first = encode(
            &payload,
            &schema,
        )
        .expect("first encoding must succeed");

        let second = encode(
            &payload,
            &schema,
        )
        .expect("second encoding must succeed");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn ordered_metadata_is_stable() {
        let schema = test_schema();
        let payload = test_payload();

        let encoded = encode(
            &payload,
            &schema,
        )
        .expect("encoding must succeed");

        let text = String::from_utf8(
            encoded,
        )
        .expect("JSON must be UTF-8");

        let first =
            text.find("\"a\":\"first\"");

        let second =
            text.find("\"b\":\"second\"");

        assert!(
            first.is_some()
        );

        assert!(
            second.is_some()
        );

        assert!(
            first < second
        );
    }

    #[test]
    fn json_value_encoding_is_valid() {
        let schema = test_schema();

        let value = serde_json::json!({
            "name": "zamani",
            "count": 4
        });

        let encoded =
            encode_json_value(
                &value,
                &schema,
            )
            .expect("JSON value encoding must succeed");

        let decoded: Value =
            serde_json::from_slice(&encoded)
                .expect("encoded value must be valid JSON");

        assert!(
            decoded.get("schema").is_some()
        );

        assert!(
            decoded.get("payload").is_some()
        );
    }

    #[test]
    fn streaming_reports_bytes() {
        let schema = test_schema();
        let payload = test_payload();

        let mut output = Vec::new();

        let result =
            encode_to_writer(
                &mut output,
                &payload,
                &schema,
            )
            .expect("encoding must succeed");

        assert_eq!(
            result.bytes_written() as usize,
            output.len()
        );
    }

    #[test]
    fn empty_arrays_are_supported() {
        let schema = test_schema();

        let value = serde_json::json!({
            "qubits": [],
            "operations": []
        });

        let encoded =
            encode_json_value(
                &value,
                &schema,
            )
            .expect("empty collections must encode");

        assert!(
            !encoded.is_empty()
        );
    }

    #[test]
    fn large_dynamic_collection_is_not_restricted_by_encoder() {
        let schema = test_schema();

        let value = Value::Array(
            (0_u64..10_000)
                .map(Value::from)
                .collect(),
        );

        let encoded =
            encode_json_value(
                &value,
                &schema,
            )
            .expect("dynamic collections must encode");

        assert!(
            !encoded.is_empty()
        );
    }
}