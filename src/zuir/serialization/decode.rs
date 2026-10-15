
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Serialization Decoder
//!
//! Path: `src/zuir/serialization/decode.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Decode the versioned JSON document emitted by
//! `serialization/encode.rs`. Validate its envelope, schema identity,
//! format version, and schema-version compatibility before exposing payload
//! data to downstream code.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/serialization/encode.rs`
//! - `src/zuir/serialization/validation.rs`, when available
//! - `src/zuir/ir/program.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! OWNS
//!
//! - The decoder's public API and decoding options.
//! - Validation of the serialized document envelope.
//! - Input-byte budgeting for byte-slice and reader entry points.
//! - Format-version and schema-version checks.
//! - Conversion of the wire document into a decoded, untrusted payload.
//!
//! DOES NOT OWN
//!
//! - The schema definition or schema migrations.
//! - Whole-program ZUIR structural or semantic verification.
//! - AST parsing, source-language semantics, or name resolution.
//! - Extension authorization or execution.
//! - Cryptographic signatures, checksums, or content fingerprints.
//! - Target selection, resource allocation, or hardware execution.
//!
//! SECURITY AND TRUST
//!
//! Decoded payloads are untrusted data. Successful decoding proves only
//! that the JSON document and its envelope satisfy this module's checks.
//! Callers MUST run the appropriate schema and IR verifier before compiling,
//! transforming, executing, or otherwise trusting the payload.
//!
//! COMPATIBILITY
//!
//! The wire envelope matches `serialization/encode.rs`:
//!
//! {
//!   "format_version": { "major": 1, "minor": 0, "patch": 0 },
//!   "schema_id": "zamani.zuir.serialization.document",
//!   "schema_version": { "major": 1, "minor": 0, "patch": 0 },
//!   "payload": ...
//! }
//!
//! The decoder deliberately rejects unknown envelope fields and unsupported
//! format versions. A future format requiring migrations must be handled by
//! the explicit migration layer rather than silently interpreted here.
//!
//! SCALABILITY
//!
//! There is no fixed program-size, operation-count, qubit-count, tensor-rank,
//! device-count, or hardware-size limit. An optional byte budget is an
//! operational safeguard, not a semantic machine limit.
//!
//! JSON decoding currently materializes the payload as `serde_json::Value`.
//! Memory use is therefore proportional to the input and decoded value.
//! Reader input is accumulated only up to the configured byte budget when
//! one is supplied. No finite process can promise infinite capacity.
//!
//! INTEGRATION
//!
//! `serialization/mod.rs` must declare `pub mod decode;`.
//! The serialization facade may re-export the public API.
//! `encode.rs` owns the matching output format.
//! Schema migration must occur before this strict decoder, or through a
//! separately specified migration-aware entry point.
//! IR verification remains a separate mandatory stage.
//!
//! FREEZE CRITERIA
//!
//! - The serialization facade exports this module.
//! - The envelope matches the encoder's current wire contract.
//! - Unsupported versions, invalid schema IDs, unknown envelope fields,
//!   malformed JSON, and configured input-budget violations are rejected.
//! - Reader and byte-slice entry points behave consistently.
//! - The crate and all existing integration tests compile and pass.
//!
//! This contract does not claim that the complete serialization pipeline
//! has already been integrated or verified in the repository.

use core::fmt;
use std::io::{self, Read};

use serde::Deserialize;
use serde_json::Value;

use crate::zuir::serialization::schema::{
    SchemaError, SchemaId, SchemaVersion, CURRENT_SCHEMA_VERSION,
    DOCUMENT_SCHEMA_ID,
};
use crate::zuir::version::{
    ZuirFormatVersion, CURRENT_ZUIR_FORMAT_VERSION,
};

/// Name of the serialized format-version field.
pub const FORMAT_VERSION_FIELD: &str = "format_version";

/// Name of the serialized schema-identity field.
pub const SCHEMA_ID_FIELD: &str = "schema_id";

/// Name of the serialized schema-version field.
pub const SCHEMA_VERSION_FIELD: &str = "schema_version";

/// Name of the serialized payload field.
pub const PAYLOAD_FIELD: &str = "payload";

/// Options for decoding a serialized ZUIR document.
///
/// A missing input budget means that this module applies no additional
/// byte limit. It does not guarantee unlimited memory or processing time.
#[derive(Debug, Clone)]
pub struct DecodeOptions {
    expected_schema_id: SchemaId,
    max_input_bytes: Option<usize>,
}

impl DecodeOptions {
    /// Creates options for the standard ZUIR document schema.
    pub fn new() -> Result<Self, DecodeError> {
        Ok(Self {
            expected_schema_id: SchemaId::new(DOCUMENT_SCHEMA_ID)
                .map_err(DecodeError::Schema)?,
            max_input_bytes: None,
        })
    }

    /// Sets the expected schema identifier.
    ///
    /// The identifier is syntax-validated immediately. This does not prove
    /// that the schema is registered, trusted, or supported by a consumer.
    pub fn with_expected_schema_id(
        mut self,
        schema_id: impl Into<String>,
    ) -> Result<Self, DecodeError> {
        self.expected_schema_id = SchemaId::new(schema_id)
            .map_err(DecodeError::Schema)?;
        Ok(self)
    }

    /// Sets an optional maximum number of input bytes.
    ///
    /// The limit applies to the complete serialized document, including
    /// envelope and payload. A document one byte larger than the limit
    /// is rejected.
    #[must_use]
    pub fn with_max_input_bytes(
        mut self,
        max_bytes: Option<usize>,
    ) -> Self {
        self.max_input_bytes = max_bytes;
        self
    }

    /// Returns the required schema identity.
    #[must_use]
    pub fn expected_schema_id(&self) -> &SchemaId {
        &self.expected_schema_id
    }

    /// Returns the configured input-byte budget.
    #[must_use]
    pub const fn max_input_bytes(&self) -> Option<usize> {
        self.max_input_bytes
    }
}

impl Default for DecodeOptions {
    fn default() -> Self {
        Self::new().expect("the built-in schema identifier is valid")
    }
}

/// A decoded document envelope and its JSON payload.
///
/// `payload` is not automatically a verified ZUIR program. It must be
/// validated against the relevant payload schema and IR invariants.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodedDocument {
    format_version: ZuirFormatVersion,
    schema_id: SchemaId,
    schema_version: SchemaVersion,
    payload: Value,
}

impl DecodedDocument {
    /// Returns the serialization-protocol version.
    #[must_use]
    pub const fn format_version(&self) -> ZuirFormatVersion {
        self.format_version
    }

    /// Returns the validated schema identifier.
    #[must_use]
    pub fn schema_id(&self) -> &SchemaId {
        &self.schema_id
    }

    /// Returns the declared schema version.
    #[must_use]
    pub const fn schema_version(&self) -> SchemaVersion {
        self.schema_version
    }

    /// Borrows the decoded JSON payload.
    #[must_use]
    pub fn payload(&self) -> &Value {
        &self.payload
    }

    /// Consumes the document and returns its JSON payload.
    #[must_use]
    pub fn into_payload(self) -> Value {
        self.payload
    }
}

/// Structured decoding failures.
#[derive(Debug)]
#[non_exhaustive]
pub enum DecodeError {
    /// The schema identifier is malformed.
    Schema(SchemaError),

    /// The document is not valid JSON or does not match the envelope shape.
    Json(serde_json::Error),

    /// The document's format version is not supported by this decoder.
    UnsupportedFormatVersion {
        found: ZuirFormatVersion,
        supported: ZuirFormatVersion,
    },

    /// The declared schema version is not compatible with this reader.
    IncompatibleSchemaVersion {
        found: SchemaVersion,
        supported: SchemaVersion,
    },

    /// The document schema does not match the caller's expected schema.
    UnexpectedSchema {
        expected: String,
        found: String,
    },

    /// The serialized document exceeds the configured input-byte budget.
    InputBudgetExceeded {
        limit: usize,
    },

    /// The accumulated input length cannot be represented.
    InputLengthOverflow,

    /// Memory reservation for input accumulation failed.
    AllocationFailed,

    /// The input reader returned an I/O error.
    Io(io::Error),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Schema(error) => {
                write!(f, "ZUIR decoding schema error: {error}")
            }
            Self::Json(error) => {
                write!(f, "ZUIR JSON decoding failed: {error}")
            }
            Self::UnsupportedFormatVersion {
                found,
                supported,
            } => {
                write!(
                    f,
                    "unsupported ZUIR format version {found}; \
                     this decoder supports {supported}"
                )
            }
            Self::IncompatibleSchemaVersion {
                found,
                supported,
            } => {
                write!(
                    f,
                    "incompatible ZUIR schema version {found}; \
                     this reader supports {supported}"
                )
            }
            Self::UnexpectedSchema { expected, found } => {
                write!(
                    f,
                    "unexpected ZUIR schema {found:?}; expected {expected:?}"
                )
            }
            Self::InputBudgetExceeded { limit } => {
                write!(
                    f,
                    "serialized ZUIR document exceeds input budget \
                     of {limit} bytes"
                )
            }
            Self::InputLengthOverflow => {
                f.write_str("serialized ZUIR input length overflow")
            }
            Self::AllocationFailed => {
                f.write_str("unable to reserve memory for ZUIR input")
            }
            Self::Io(error) => {
                write!(f, "reading serialized ZUIR document failed: {error}")
            }
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Schema(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::UnsupportedFormatVersion { .. }
            | Self::IncompatibleSchemaVersion { .. }
            | Self::UnexpectedSchema { .. }
            | Self::InputBudgetExceeded { .. }
            | Self::InputLengthOverflow
            | Self::AllocationFailed => None,
        }
    }
}

/// Wire representation of a numeric version triple.
///
/// Unknown fields are rejected so a newer representation cannot be
/// accidentally interpreted as the currently supported version.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct VersionWire {
    major: u32,
    minor: u32,
    patch: u32,
}

impl VersionWire {
    const fn as_format_version(&self) -> ZuirFormatVersion {
        ZuirFormatVersion::new(self.major, self.minor, self.patch)
    }

    const fn as_schema_version(&self) -> SchemaVersion {
        SchemaVersion::new(self.major, self.minor, self.patch)
    }
}

/// Wire document matching `serialization/encode.rs`.
///
/// Unknown top-level fields are rejected deliberately. Any future envelope
/// extension must first define its compatibility and preservation behavior.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DocumentWire {
    format_version: VersionWire,
    schema_id: String,
    schema_version: VersionWire,
    payload: Value,
}

/// Decodes a complete serialized document from a byte slice.
///
/// The byte budget is checked before JSON parsing. This function does not
/// perform whole-IR semantic verification.
pub fn decode(input: &[u8]) -> Result<DecodedDocument, DecodeError> {
    decode_with_options(input, &DecodeOptions::default())
}

/// Decodes a complete serialized document using explicit options.
pub fn decode_with_options(
    input: &[u8],
    options: &DecodeOptions,
) -> Result<DecodedDocument, DecodeError> {
    if let Some(limit) = options.max_input_bytes() {
        if input.len() > limit {
            return Err(DecodeError::InputBudgetExceeded { limit });
        }
    }

    let wire: DocumentWire =
        serde_json::from_slice(input).map_err(DecodeError::Json)?;

    validate_and_convert(wire, options)
}

/// Reads and decodes a document from a reader.
///
/// If a byte budget is configured, input accumulation stops as soon as the
/// reader supplies bytes that would exceed it. Without a budget, memory use
/// remains proportional to the complete input document.
///
/// A reader may itself have side effects; this function cannot roll back
/// bytes already consumed from it.
pub fn decode_from_reader<R: Read>(
    reader: &mut R,
) -> Result<DecodedDocument, DecodeError> {
    decode_from_reader_with_options(reader, &DecodeOptions::default())
}

/// Reads and decodes a document from a reader using explicit options.
pub fn decode_from_reader_with_options<R: Read>(
    reader: &mut R,
    options: &DecodeOptions,
) -> Result<DecodedDocument, DecodeError> {
    let input = read_bounded(reader, options.max_input_bytes())?;
    decode_with_options(&input, options)
}

fn validate_and_convert(
    wire: DocumentWire,
    options: &DecodeOptions,
) -> Result<DecodedDocument, DecodeError> {
    let found_format = wire.format_version.as_format_version();
    let supported_format = CURRENT_ZUIR_FORMAT_VERSION;

    // This decoder supports the current encoding contract only. Historical
    // encodings must pass through an explicit migration implementation.
    if found_format != supported_format {
        return Err(DecodeError::UnsupportedFormatVersion {
            found: found_format,
            supported: supported_format,
        });
    }

    let schema_id =
        SchemaId::new(wire.schema_id).map_err(DecodeError::Schema)?;

    if schema_id != *options.expected_schema_id() {
        return Err(DecodeError::UnexpectedSchema {
            expected: options.expected_schema_id().as_str().to_owned(),
            found: schema_id.as_str().to_owned(),
        });
    }

    let found_schema_version = wire.schema_version.as_schema_version();
    let supported_schema_version = CURRENT_SCHEMA_VERSION;

    // SchemaVersion::supports enforces the schema module's conservative
    // same-major, no-newer-writer rule. This does not replace payload-level
    // schema validation or a migration where one is required.
    if !supported_schema_version.supports(found_schema_version) {
        return Err(DecodeError::IncompatibleSchemaVersion {
            found: found_schema_version,
            supported: supported_schema_version,
        });
    }

    Ok(DecodedDocument {
        format_version: found_format,
        schema_id,
        schema_version: found_schema_version,
        payload: wire.payload,
    })
}

fn read_bounded<R: Read>(
    reader: &mut R,
    max_bytes: Option<usize>,
) -> Result<Vec<u8>, DecodeError> {
    const CHUNK_SIZE: usize = 8 * 1024;

    let mut bytes = Vec::new();
    let mut chunk = [0_u8; CHUNK_SIZE];

    loop {
        let count = reader.read(&mut chunk).map_err(DecodeError::Io)?;

        if count == 0 {
            return Ok(bytes);
        }

        let new_len = bytes
            .len()
            .checked_add(count)
            .ok_or(DecodeError::InputLengthOverflow)?;

        if let Some(limit) = max_bytes {
            if new_len > limit {
                return Err(DecodeError::InputBudgetExceeded { limit });
            }
        }

        bytes
            .try_reserve(count)
            .map_err(|_| DecodeError::AllocationFailed)?;

        bytes.extend_from_slice(&chunk[..count]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn document(payload: &str) -> String {
        format!(
            concat!(
                "{{",
                "\"format_version\":{{\"major\":{},\"minor\":{},\"patch\":{}}},",
                "\"schema_id\":\"{}\",",
                "\"schema_version\":{{\"major\":{},\"minor\":{},\"patch\":{}}},",
                "\"payload\":{}",
                "}}"
            ),
            CURRENT_ZUIR_FORMAT_VERSION.major(),
            CURRENT_ZUIR_FORMAT_VERSION.minor(),
            CURRENT_ZUIR_FORMAT_VERSION.patch(),
            DOCUMENT_SCHEMA_ID,
            CURRENT_SCHEMA_VERSION.major(),
            CURRENT_SCHEMA_VERSION.minor(),
            CURRENT_SCHEMA_VERSION.patch(),
            payload,
        )
    }

    #[test]
    fn decodes_a_valid_versioned_document() {
        let input = document(r#"{"answer":42}"#);
        let decoded = decode(input.as_bytes()).expect("valid document");

        assert_eq!(decoded.schema_id().as_str(), DOCUMENT_SCHEMA_ID);
        assert_eq!(decoded.schema_version(), CURRENT_SCHEMA_VERSION);
        assert_eq!(
            decoded.format_version(),
            CURRENT_ZUIR_FORMAT_VERSION
        );
        assert_eq!(decoded.payload()["answer"], 42);
    }

    #[test]
    fn rejects_malformed_json() {
        let result = decode(b"{not-json");

        assert!(matches!(result, Err(DecodeError::Json(_))));
    }

    #[test]
    fn rejects_missing_required_fields() {
        let result = decode(br#"{"schema_id":"zamani.zuir.serialization.document"}"#);

        assert!(matches!(result, Err(DecodeError::Json(_))));
    }

    #[test]
    fn rejects_unknown_envelope_fields() {
        let input = document(r#"{"answer":42}"#);
        let input = input.replace(
            "\"payload\":",
            "\"future_envelope_field\":true,\"payload\":",
        );

        assert!(matches!(
            decode(input.as_bytes()),
            Err(DecodeError::Json(_))
        ));
    }

    #[test]
    fn rejects_unknown_version_fields() {
        let input = document(r#"{"answer":42}"#);
        let input = input.replace(
            "\"major\":1",
            "\"major\":1,\"prerelease\":\"future\"",
        );

        assert!(matches!(
            decode(input.as_bytes()),
            Err(DecodeError::Json(_))
        ));
    }

    #[test]
    fn rejects_a_future_format_version() {
        let input = document(r#"{"answer":42}"#).replace(
            "\"format_version\":{\"major\":1,\"minor\":0,\"patch\":0}",
            "\"format_version\":{\"major\":2,\"minor\":0,\"patch\":0}",
        );

        assert!(matches!(
            decode(input.as_bytes()),
            Err(DecodeError::UnsupportedFormatVersion { .. })
        ));
    }

    #[test]
    fn rejects_a_future_schema_version() {
        let input = document(r#"{"answer":42}"#).replace(
            "\"schema_version\":{\"major\":1,\"minor\":0,\"patch\":0}",
            "\"schema_version\":{\"major\":1,\"minor\":1,\"patch\":0}",
        );

        assert!(matches!(
            decode(input.as_bytes()),
            Err(DecodeError::IncompatibleSchemaVersion { .. })
        ));
    }

    #[test]
    fn rejects_a_different_schema_identity() {
        let input = document(r#"{"answer":42}"#).replace(
            DOCUMENT_SCHEMA_ID,
            "zamani.zuir.serialization.other",
        );

        assert!(matches!(
            decode(input.as_bytes()),
            Err(DecodeError::UnexpectedSchema { .. })
        ));
    }

    #[test]
    fn rejects_an_input_over_the_configured_budget() {
        let input = document(r#"{"answer":42}"#);
        let options = DecodeOptions::default()
            .with_max_input_bytes(Some(input.len() - 1));

        assert!(matches!(
            decode_with_options(input.as_bytes(), &options),
            Err(DecodeError::InputBudgetExceeded { .. })
        ));
    }

    #[test]
    fn accepts_an_input_exactly_at_the_budget() {
        let input = document(r#"{"answer":42}"#);
        let options = DecodeOptions::default()
            .with_max_input_bytes(Some(input.len()));

        assert!(decode_with_options(input.as_bytes(), &options).is_ok());
    }

    #[test]
    fn reader_and_slice_decoding_agree() {
        let input = document(r#"{"values":[1,2,3]}"#);
        let from_slice = decode(input.as_bytes()).expect("slice decoding");

        let mut reader = Cursor::new(input.as_bytes());
        let from_reader =
            decode_from_reader(&mut reader).expect("reader decoding");

        assert_eq!(from_slice, from_reader);
    }

    #[test]
    fn reader_obeys_the_input_budget() {
        let input = document(r#"{"answer":42}"#);
        let options = DecodeOptions::default()
            .with_max_input_bytes(Some(input.len() - 1));
        let mut reader = Cursor::new(input.as_bytes());

        assert!(matches!(
            decode_from_reader_with_options(&mut reader, &options),
            Err(DecodeError::InputBudgetExceeded { .. })
        ));
    }

    #[test]
    fn preserves_payload_array_order() {
        let input = document(r#"{"values":[3,1,2]}"#);
        let decoded = decode(input.as_bytes()).expect("valid document");

        assert_eq!(
            decoded.payload()["values"],
            serde_json::json!([3, 1, 2])
        );
    }

    #[test]
    fn rejects_trailing_non_whitespace_data() {
        let mut input = document(r#"{"answer":42}"#).into_bytes();
        input.extend_from_slice(b" trailing");

        assert!(matches!(decode(&input), Err(DecodeError::Json(_))));
    }
}
