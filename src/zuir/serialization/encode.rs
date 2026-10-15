
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Serialization Encoder
//!
//! Canonical path: `src/zuir/serialization/encode.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Encode serializable ZUIR documents into a versioned, deterministic JSON
//! representation. This module transports representation data; it does not
//! establish semantic validity, authorize extensions, or execute IR.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/ir/program.rs`
//! - `src/zuir/ir/operation.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! OWNERSHIP
//!
//! OWNS:
//! - The JSON document envelope used by this encoder.
//! - Canonical JSON object-key ordering.
//! - Encoding options and structured encoding errors.
//! - Writer-based and in-memory encoding entry points.
//! - Output-budget enforcement.
//!
//! DOES NOT OWN:
//! - Schema definitions or schema migration.
//! - Decoding, IR verification, or semantic analysis.
//! - AST or grammar interpretation.
//! - Domain-specific operations or hardware allocation.
//! - Cryptographic integrity, signatures, or content fingerprints.
//!
//! DETERMINISM
//!
//! Object keys are sorted lexicographically. Array order is preserved.
//! The output also depends on the payload's Serialize implementation,
//! serde_json's numeric formatting, the schema identity, and format version.
//! This is a versioned deterministic JSON contract, not a claim of
//! conformance to every external canonical-JSON standard.
//!
//! SCALABILITY
//!
//! No hardware or IR entity count is hard-coded. Output budgets are optional
//! operational safeguards. Encoding uses memory proportional to the payload
//! and encoded document because the complete document is staged before it
//! is written to the caller's destination.
//!
//! No finite process can guarantee infinite capacity.
//!
//! INTEGRATION
//!
//! `serialization/mod.rs` must declare and export this module.
//! The decoder must consume the exact envelope and version contract below.
//! Schema migration belongs in the migration layer. Whole-IR validation
//! remains the responsibility of the verifier/serialization validation layer.
//!
//! FREEZE CRITERIA
//!
//! - The module is exported by the serialization facade.
//! - The decoder agrees with the envelope and version fields.
//! - Round-trip, determinism, budget, and writer-failure tests pass.
//! - The real repository compiles and its existing test suite passes.
//!
//! This module is not proof that the entire serialization subsystem is
//! already implemented or production-ready.

use core::fmt;
use serde::Serialize;
use serde_json::Value;
use std::io::{self, Write};

use crate::zuir::serialization::schema::{
    SchemaError, SchemaId, SchemaVersion, CURRENT_SCHEMA_VERSION,
    DOCUMENT_SCHEMA_ID,
};
use crate::zuir::version::CURRENT_ZUIR_FORMAT_VERSION;

/// Name of the JSON field carrying the serialization format version.
pub const FORMAT_VERSION_FIELD: &str = "format_version";

/// Name of the JSON field carrying the schema identifier.
pub const SCHEMA_ID_FIELD: &str = "schema_id";

/// Name of the JSON field carrying the schema version.
pub const SCHEMA_VERSION_FIELD: &str = "schema_version";

/// Name of the JSON field carrying the serialized payload.
pub const PAYLOAD_FIELD: &str = "payload";

/// Configuration for a single encoding operation.
///
/// A missing output budget means that this module imposes no additional
/// byte limit. It does not guarantee that memory or the destination has
/// unlimited capacity.
#[derive(Debug, Clone)]
pub struct EncodeOptions {
    schema_id: SchemaId,
    schema_version: SchemaVersion,
    max_output_bytes: Option<usize>,
}

impl EncodeOptions {
    /// Creates options for the current document schema.
    pub fn new() -> Result<Self, EncodeError> {
        Ok(Self {
            schema_id: SchemaId::new(DOCUMENT_SCHEMA_ID)
                .map_err(EncodeError::Schema)?,
            schema_version: CURRENT_SCHEMA_VERSION,
            max_output_bytes: None,
        })
    }

    /// Sets the schema identifier after validating its syntax.
    pub fn with_schema_id(
        mut self,
        schema_id: impl Into<String>,
    ) -> Result<Self, EncodeError> {
        self.schema_id = SchemaId::new(schema_id)
            .map_err(EncodeError::Schema)?;
        Ok(self)
    }

    /// Sets the schema version to declare in the document.
    ///
    /// This is an explicit declaration, not proof that the payload conforms
    /// to that schema. Validate the payload before encoding it.
    #[must_use]
    pub fn with_schema_version(
        mut self,
        version: SchemaVersion,
    ) -> Self {
        self.schema_version = version;
        self
    }

    /// Applies an optional maximum encoded-document size in bytes.
    ///
    /// The budget covers the complete JSON document, including its envelope.
    #[must_use]
    pub fn with_max_output_bytes(
        mut self,
        max_bytes: Option<usize>,
    ) -> Self {
        self.max_output_bytes = max_bytes;
        self
    }

    /// Returns the schema identifier.
    #[must_use]
    pub fn schema_id(&self) -> &SchemaId {
        &self.schema_id
    }

    /// Returns the declared schema version.
    #[must_use]
    pub const fn schema_version(&self) -> SchemaVersion {
        self.schema_version
    }

    /// Returns the configured output budget.
    #[must_use]
    pub const fn max_output_bytes(&self) -> Option<usize> {
        self.max_output_bytes
    }
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self::new().expect("the built-in schema identifier is valid")
    }
}

/// Structured failures returned by the encoder.
#[derive(Debug)]
#[non_exhaustive]
pub enum EncodeError {
    /// The schema identifier or schema contract is invalid.
    Schema(SchemaError),

    /// The payload could not be represented as JSON.
    ///
    /// Examples include unsupported map-key types and non-finite floating
    /// point values.
    Json(serde_json::Error),

    /// The encoded document exceeded the configured output budget.
    OutputBudgetExceeded {
        /// Configured maximum document size.
        limit: usize,
    },

    /// The destination writer failed while receiving the completed document.
    ///
    /// A writer can fail after accepting some bytes. In that case its output
    /// may contain a partial document; the caller must treat the write as
    /// failed and must not consume that output as a complete artifact.
    Io(io::Error),
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Schema(error) => {
                write!(f, "ZUIR encoding schema error: {error}")
            }
            Self::Json(error) => {
                write!(f, "ZUIR JSON encoding failed: {error}")
            }
            Self::OutputBudgetExceeded { limit } => {
                write!(
                    f,
                    "encoded ZUIR document exceeds output budget \
                     of {limit} bytes"
                )
            }
            Self::Io(error) => {
                write!(f, "writing encoded ZUIR document failed: {error}")
            }
        }
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Schema(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::OutputBudgetExceeded { .. } => None,
        }
    }
}

/// A fully encoded document.
///
/// This owns its bytes so callers can persist, transmit, or hash the same
/// artifact without serializing the payload a second time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedArtifact {
    bytes: Vec<u8>,
}

impl EncodedArtifact {
    /// Borrows the complete encoded document.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the encoded document's byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the document is empty.
    ///
    /// A valid JSON envelope is never empty; this method is provided for
    /// conventional collection ergonomics.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consumes the artifact and returns its encoded bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Writes the complete artifact to a destination.
    ///
    /// The destination can still fail partway through the write.
    pub fn write_to<W: Write>(
        &self,
        destination: &mut W,
    ) -> Result<(), EncodeError> {
        destination
            .write_all(&self.bytes)
            .map_err(EncodeError::Io)
    }
}

/// Stable, explicit wire representation of the format version.
///
/// Numeric components are retained as integers rather than parsed from an
/// implementation-dependent version string.
#[derive(Serialize)]
struct FormatVersionWire {
    major: u32,
    minor: u32,
    patch: u32,
}

impl FormatVersionWire {
    fn current() -> Self {
        let version = CURRENT_ZUIR_FORMAT_VERSION;
        Self {
            major: version.major(),
            minor: version.minor(),
            patch: version.patch(),
        }
    }
}

/// Stable JSON envelope.
///
/// Field order is part of this encoder's versioned wire contract.
#[derive(Serialize)]
struct DocumentWire<'a> {
    format_version: FormatVersionWire,
    schema_id: &'a str,
    schema_version: SchemaVersionWire,
    payload: &'a Value,
}

/// Serialization-schema version represented on the wire.
#[derive(Serialize)]
struct SchemaVersionWire {
    major: u32,
    minor: u32,
    patch: u32,
}

impl From<SchemaVersion> for SchemaVersionWire {
    fn from(version: SchemaVersion) -> Self {
        Self {
            major: version.major(),
            minor: version.minor(),
            patch: version.patch(),
        }
    }
}

/// Encodes a payload into an owned artifact using the default options.
///
/// The payload is converted to a JSON value, its object keys are normalized
/// to lexicographic order, and the versioned envelope is serialized.
pub fn encode<T: Serialize + ?Sized>(
    payload: &T,
) -> Result<EncodedArtifact, EncodeError> {
    encode_with_options(payload, &EncodeOptions::default())
}

/// Encodes a payload using explicit options.
///
/// The complete document is staged in memory before an artifact is returned.
/// When a byte budget is configured, the bounded staging writer stops output
/// once the budget would be exceeded.
pub fn encode_with_options<T: Serialize + ?Sized>(
    payload: &T,
    options: &EncodeOptions,
) -> Result<EncodedArtifact, EncodeError> {
    let payload = serde_json::to_value(payload)
        .map_err(EncodeError::Json)?;

    let payload = canonicalize_value(payload);

    let document = DocumentWire {
        format_version: FormatVersionWire::current(),
        schema_id: options.schema_id().as_str(),
        schema_version: options.schema_version().into(),
        payload: &payload,
    };

    let bytes = encode_document(&document, options.max_output_bytes())?;

    Ok(EncodedArtifact { bytes })
}

/// Encodes a payload and writes it to a destination.
///
/// Unlike serializing directly into the caller's writer, this function stages
/// the complete artifact first. Therefore JSON conversion or output-budget
/// errors do not leave a partial document in the destination. The destination
/// can still fail during `write_all`.
pub fn encode_to_writer<T: Serialize + ?Sized, W: Write>(
    payload: &T,
    destination: &mut W,
) -> Result<(), EncodeError> {
    encode(payload)?.write_to(destination)
}

/// Encodes a payload with explicit options and writes the artifact.
pub fn encode_to_writer_with_options<
    T: Serialize + ?Sized,
    W: Write,
>(
    payload: &T,
    options: &EncodeOptions,
    destination: &mut W,
) -> Result<(), EncodeError> {
    encode_with_options(payload, options)?.write_to(destination)
}

fn encode_document(
    document: &impl Serialize,
    max_output_bytes: Option<usize>,
) -> Result<Vec<u8>, EncodeError> {
    let mut writer = BoundedVecWriter::new(max_output_bytes);

    match serde_json::to_writer(&mut writer, document) {
        Ok(()) => Ok(writer.into_inner()),
        Err(error) if writer.exceeded_budget() => {
            Err(EncodeError::OutputBudgetExceeded {
                limit: max_output_bytes.unwrap_or(0),
            })
        }
        Err(error) => Err(EncodeError::Json(error)),
    }
}

/// A staging writer that refuses to exceed an optional byte budget.
///
/// It never uses unchecked arithmetic. The output remains private until
/// serialization succeeds, so budget failures cannot leak partial output
/// into a caller-provided destination.
struct BoundedVecWriter {
    bytes: Vec<u8>,
    limit: Option<usize>,
    exceeded: bool,
}

impl BoundedVecWriter {
    fn new(limit: Option<usize>) -> Self {
        Self {
            bytes: Vec::new(),
            limit,
            exceeded: false,
        }
    }

    fn exceeded_budget(&self) -> bool {
        self.exceeded
    }

    fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for BoundedVecWriter {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        if let Some(limit) = self.limit {
            let Some(new_len) = self.bytes.len().checked_add(input.len()) else {
                self.exceeded = true;
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "encoded output length overflow",
                ));
            };

            if new_len > limit {
                self.exceeded = true;
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "configured encoded-output budget exceeded",
                ));
            }
        }

        self.bytes
            .try_reserve(input.len())
            .map_err(|error| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("unable to reserve encoded output: {error}"),
                )
            })?;

        self.bytes.extend_from_slice(input);
        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Recursively canonicalizes JSON object keys.
///
/// Arrays are deliberately left in their original order because array order
/// can represent program semantics, operation order, operands, control flow,
/// or other ordered data. Object keys are metadata names, not sequences.
///
/// Sorting is explicit so this remains deterministic even if Cargo feature
/// unification changes the underlying serde_json map implementation.
fn canonicalize_value(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(
            values.into_iter().map(canonicalize_value).collect(),
        ),
        Value::Object(object) => {
            let mut entries: Vec<(String, Value)> =
                object.into_iter().collect();

            entries.sort_unstable_by(|left, right| {
                left.0.cmp(&right.0)
            });

            let mut canonical = serde_json::Map::new();

            for (key, value) in entries {
                canonical.insert(key, canonicalize_value(value));
            }

            Value::Object(canonical)
        }
        scalar => scalar,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn encodes_a_versioned_document() {
        let artifact = encode(&json!({"answer": 42}))
            .expect("encoding should succeed");

        let decoded: Value = serde_json::from_slice(artifact.as_bytes())
            .expect("encoded JSON should parse");

        assert_eq!(
            decoded["schema_id"],
            DOCUMENT_SCHEMA_ID
        );
        assert_eq!(
            decoded["schema_version"]["major"],
            CURRENT_SCHEMA_VERSION.major()
        );
        assert_eq!(decoded["payload"]["answer"], 42);
        assert!(decoded["format_version"]["major"].is_number());
    }

    #[test]
    fn object_key_order_is_deterministic() {
        let first = json!({
            "zeta": 3,
            "alpha": {"y": 2, "b": 1}
        });

        let second = json!({
            "alpha": {"b": 1, "y": 2},
            "zeta": 3
        });

        let first_bytes = encode(&first)
            .expect("first encoding")
            .into_bytes();

        let second_bytes = encode(&second)
            .expect("second encoding")
            .into_bytes();

        assert_eq!(first_bytes, second_bytes);
    }

    #[test]
    fn array_order_is_preserved() {
        let first = encode(&json!([1, 2, 3]))
            .expect("first encoding");
        let second = encode(&json!([3, 2, 1]))
            .expect("second encoding");

        assert_ne!(first.as_bytes(), second.as_bytes());
    }

    #[test]
    fn output_budget_rejects_oversized_document() {
        let options = EncodeOptions::default()
            .with_max_output_bytes(Some(1));

        let result = encode_with_options(&json!({"data": "value"}), &options);

        assert!(matches!(
            result,
            Err(EncodeError::OutputBudgetExceeded { limit: 1 })
        ));
    }

    #[test]
    fn writer_receives_the_same_bytes_as_owned_encoding() {
        let value = json!({"domain": "quantum", "operations": [1, 2]});
        let expected = encode(&value)
            .expect("encoding")
            .into_bytes();

        let mut destination = Vec::new();
        encode_to_writer(&value, &mut destination)
            .expect("writer encoding should succeed");

        assert_eq!(destination, expected);
    }

    #[test]
    fn schema_identity_is_validated() {
        let options = EncodeOptions::default()
            .with_schema_id("invalid schema name");

        assert!(options.is_err());
    }

    #[test]
    fn schema_version_can_be_declared_explicitly() {
        let version = SchemaVersion::new(2, 1, 0);
        let options = EncodeOptions::default()
            .with_schema_version(version);

        let artifact = encode_with_options(&json!({}), &options)
            .expect("encoding");

        let decoded: Value = serde_json::from_slice(artifact.as_bytes())
            .expect("JSON");

        assert_eq!(decoded["schema_version"]["major"], 2);
        assert_eq!(decoded["schema_version"]["minor"], 1);
        assert_eq!(decoded["schema_version"]["patch"], 0);
    }

    #[test]
    fn btree_map_payloads_are_supported() {
        let mut payload = BTreeMap::new();
        payload.insert("classical", 1_u64);
        payload.insert("quantum", 2_u64);
        payload.insert("hdl", 3_u64);

        let artifact = encode(&payload).expect("encoding");

        let decoded: Value = serde_json::from_slice(artifact.as_bytes())
            .expect("JSON");

        assert_eq!(decoded["payload"]["quantum"], 2);
    }

    #[test]
    fn non_finite_float_fails_explicitly() {
        let result = encode(&f64::NAN);

        assert!(matches!(result, Err(EncodeError::Json(_))));
    }

    #[test]
    fn writer_failure_is_reported() {
        struct FailingWriter;

        impl Write for FailingWriter {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("intentional failure"))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let result = encode_to_writer(&json!({"value": 1}), &mut FailingWriter);

        assert!(matches!(result, Err(EncodeError::Io(_))));
    }

    #[test]
    fn zero_budget_rejects_nonempty_document() {
        let options = EncodeOptions::default()
            .with_max_output_bytes(Some(0));

        let result = encode_with_options(&json!({}), &options);

        assert!(matches!(
            result,
            Err(EncodeError::OutputBudgetExceeded { limit: 0 })
        ));
    }
}
