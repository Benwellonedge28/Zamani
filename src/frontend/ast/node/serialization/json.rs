//! # Zamani Frontend AST — JSON Serialization
//!
//! Production JSON serialization boundary for the native Zamani AST.
//!
//! ## Architectural position
//!
//! This module belongs to:
//!
//!     src/frontend/ast/node/serialization/json.rs
//!
//! and owns only the JSON wire representation of AST data.
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! lexer / parser
//!      │
//!      ▼
//! native Zamani AST
//!      │
//!      ├── node
//!      ├── node_id
//!      ├── node_kind
//!      ├── source
//!      └── metadata
//!      │
//!      ▼
//! serialization/json.rs  ← this module
//!      │
//!      ▼
//! JSON interchange / persistence / tooling
//! ```
//!
//! This module does NOT perform:
//!
//! - parsing;
//! - lexical analysis;
//! - semantic analysis;
//! - type checking;
//! - ZUIR lowering;
//! - quantum compilation;
//! - quantum routing;
//! - scheduling;
//! - error correction;
//! - calibration;
//! - hardware mapping;
//! - backend selection;
//! - execution.
//!
//! Those responsibilities belong to other compiler layers.
//!
//! ## POCO-REAF
//!
//! The serialization format introduces no architectural machine-size limit.
//!
//! In particular, this module does not define:
//!
//! - maximum AST nodes;
//! - maximum number of qubits;
//! - maximum register size;
//! - maximum program size;
//! - maximum number of declarations;
//! - maximum number of operations;
//! - maximum JSON document size;
//! - maximum collection length.
//!
//! Concrete applications may and should impose explicit resource/security
//! policies when processing untrusted data. Such limits belong to the caller
//! or validation policy and are never part of Zamani language semantics.
//!
//! "Infinity" therefore means that this serialization layer does not introduce
//! an artificial finite architectural ceiling. Actual execution remains
//! bounded by available memory, address space, storage, I/O bandwidth and
//! configured security policy.
//!
//! ## Generic design
//!
//! The serializer intentionally operates on any `serde::Serialize` AST value
//! instead of importing every AST node type.
//!
//! This is important because:
//!
//! 1. adding an AST node must not require editing this file;
//! 2. future AST extensions remain serializable automatically when they
//!    implement Serde;
//! 3. the JSON layer remains independent of semantic/compiler subsystems;
//! 4. this module can be completed before higher-level AST modules change.
//!
//! ## Determinism
//!
//! Deterministic JSON requires deterministic input serialization.
//!
//! This module:
//!
//! - never uses `serde_json::Value` as an intermediate representation;
//! - never uses unordered maps itself;
//! - preserves the serializer's declared struct-field order;
//! - supports deterministic `BTreeMap`-based AST data;
//! - never injects timestamps;
//! - never injects memory addresses;
//! - never injects process identifiers;
//! - never injects random values.
//!
//! If an AST type itself contains a `HashMap`, its serialized ordering is
//! determined by that type's serializer and should not be relied upon as a
//! canonical content representation. Canonical AST data should use ordered
//! representations such as `BTreeMap` where map ordering is semantically
//! irrelevant.
//!
//! Canonical cryptographic hashing must remain owned by the AST hashing layer,
//! not this JSON convenience module.
//!
//! ## Version separation
//!
//! Three version concepts must remain independent:
//!
//! ```text
//! Zamani language version
//!        ≠
//! AST schema version
//!        ≠
//! JSON serialization format version
//! ```
//!
//! This file owns the JSON serialization format version.
//!
//! AST node/schema versions remain owned by the corresponding AST modules.
//!
//! ## Envelope
//!
//! Serialized documents produced by [`to_json`] have the following conceptual
//! shape:
//!
//! ```json
//! {
//!   "format": "zamani.ast.json",
//!   "format_version": 1,
//!   "payload": {}
//! }
//! ```
//!
//! The payload is the caller-provided AST value.
//!
//! The envelope gives tooling a stable way to distinguish a Zamani AST JSON
//! document from arbitrary JSON without requiring this module to know the
//! concrete AST type.
//!
//! ## Compatibility
//!
//! The reader accepts only the format versions explicitly supported by this
//! implementation. Unknown future versions are rejected rather than silently
//! interpreted as a different schema.
//!
//! This prevents a newer serialization format from being accidentally treated
//! as an older one.
//!
//! ## Streaming
//!
//! [`to_json_writer`] and [`from_json_reader`] are provided so callers can use
//! streams rather than first materializing the entire JSON document as a
//! `String` or `Vec<u8>`.
//!
//! This is important for very large ASTs.
//!
//! The in-memory AST itself necessarily consumes finite resources, but JSON
//! transport should not add an unnecessary second full representation where a
//! stream is available.
//!
//! ## Security
//!
//! Deserialization treats JSON as untrusted input.
//!
//! This module:
//!
//! - rejects malformed JSON;
//! - rejects unknown serialization-format versions;
//! - rejects wrong format identifiers;
//! - rejects trailing non-whitespace data;
//! - propagates allocation/Serde failures;
//! - never executes data;
//! - never accesses the filesystem;
//! - never accesses the network;
//! - never invokes a backend;
//! - never invokes unsafe code.
//!
//! Resource limits must be imposed by the caller's input policy where needed.
//!
//! ## Dependency contract
//!
//! This module may depend on:
//!
//! - `serde`;
//! - `serde_json`;
//! - Rust standard library.
//!
//! It must NOT depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - OpenQASM;
//! - MLIR;
//! - LLVM;
//! - hardware;
//! - routing;
//! - scheduling;
//! - runtime;
//! - backend SDKs;
//! - network clients;
//! - filesystem APIs for implicit I/O.
//!
//! ## Integration contract
//!
//! ```text
//! AST node types
//!      │
//!      │ implement Serialize / Deserialize
//!      ▼
//! this module
//!      │
//!      ├── to_json
//!      ├── to_json_bytes
//!      ├── to_json_writer
//!      ├── from_json
//!      ├── from_json_bytes
//!      └── from_json_reader
//!      │
//!      ▼
//! tooling / persistence / IPC / caches
//! ```
//!
//! Semantic consumers must deserialize the AST and then perform structural and
//! semantic validation through their respective compiler phases.
//!
//! JSON deserialization MUST NOT be treated as semantic validation.
//!
//! ## Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The safety requirement is compiler-enforced below.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::io::{Read, Write};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Stable media/type identifier for the native Zamani AST JSON format.
///
/// This identifies the serialization protocol, not a programming-language
/// domain and not a hardware target.
pub const AST_JSON_FORMAT: &str = "zamani.ast.json";

/// Current version of the AST JSON serialization envelope.
///
/// This version is intentionally independent of:
///
/// - Zamani language version;
/// - AST node schema versions;
/// - compiler version;
/// - extension versions;
/// - ZUIR version.
///
/// Increment this when the JSON envelope or its compatibility contract changes.
pub const AST_JSON_FORMAT_VERSION: u16 = 1;

/// Maximum serialization-format version understood by this implementation.
///
/// Keeping this explicit makes the compatibility policy visible and testable.
pub const MAX_SUPPORTED_AST_JSON_FORMAT_VERSION: u16 = AST_JSON_FORMAT_VERSION;

/// Versioned JSON document envelope.
///
/// The generic `T` is normally the complete native AST root, but the type is
/// intentionally generic so the serialization boundary remains independent of
/// the particular AST root representation.
///
/// # Wire format
///
/// ```json
/// {
///   "format": "zamani.ast.json",
///   "format_version": 1,
///   "payload": {}
/// }
/// ```
///
/// The field order is intentionally stable and should not be changed casually:
///
/// 1. `format`
/// 2. `format_version`
/// 3. `payload`
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstJsonEnvelope<T> {
    /// Stable serialization-format identifier.
    pub format: String,

    /// Serialization-format version.
    pub format_version: u16,

    /// Serialized AST payload.
    pub payload: T,
}

impl<T> AstJsonEnvelope<T> {
    /// Creates an envelope for the current serialization format.
    #[must_use]
    pub fn new(payload: T) -> Self {
        Self {
            format: AST_JSON_FORMAT.to_owned(),
            format_version: AST_JSON_FORMAT_VERSION,
            payload,
        }
    }

    /// Returns the serialization-format identifier.
    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Returns the serialization-format version.
    #[must_use]
    pub const fn format_version(&self) -> u16 {
        self.format_version
    }

    /// Returns a reference to the payload.
    #[must_use]
    pub fn payload(&self) -> &T {
        &self.payload
    }

    /// Consumes the envelope and returns the payload.
    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }
}

/// Errors produced by the AST JSON serialization boundary.
///
/// The error deliberately preserves the underlying `serde_json::Error` rather
/// than flattening it into a string. Callers therefore retain machine-readable
/// error categories and location information supplied by Serde JSON.
#[derive(Debug)]
pub enum AstJsonError {
    /// JSON serialization failed.
    Serialize(serde_json::Error),

    /// JSON deserialization failed.
    Deserialize(serde_json::Error),

    /// The input was valid JSON but did not identify the expected Zamani AST
    /// JSON format.
    InvalidFormat {
        /// Format identifier supplied by the document.
        actual: String,
    },

    /// The document uses a serialization format version unsupported by this
    /// implementation.
    UnsupportedFormatVersion {
        /// Version supplied by the document.
        actual: u16,

        /// Highest version understood by this implementation.
        maximum_supported: u16,
    },

    /// I/O failed while streaming JSON.
    Io(std::io::Error),
}

impl fmt::Display for AstJsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialize(error) => {
                write!(formatter, "failed to serialize Zamani AST as JSON: {error}")
            }
            Self::Deserialize(error) => {
                write!(formatter, "failed to deserialize Zamani AST JSON: {error}")
            }
            Self::InvalidFormat { actual } => {
                write!(
                    formatter,
                    "invalid Zamani AST JSON format identifier: expected {:?}, got {:?}",
                    AST_JSON_FORMAT, actual
                )
            }
            Self::UnsupportedFormatVersion {
                actual,
                maximum_supported,
            } => {
                write!(
                    formatter,
                    "unsupported Zamani AST JSON format version {actual}; \
                     maximum supported version is {maximum_supported}"
                )
            }
            Self::Io(error) => {
                write!(formatter, "I/O error while processing Zamani AST JSON: {error}")
            }
        }
    }
}

impl std::error::Error for AstJsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialize(error) | Self::Deserialize(error) => Some(error),
            Self::InvalidFormat { .. } | Self::UnsupportedFormatVersion { .. } => None,
            Self::Io(error) => Some(error),
        }
    }
}

impl From<serde_json::Error> for AstJsonError {
    fn from(error: serde_json::Error) -> Self {
        Self::Deserialize(error)
    }
}

impl From<std::io::Error> for AstJsonError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// Serializes an AST value into the versioned Zamani AST JSON format.
///
/// This produces compact UTF-8 JSON.
///
/// # Determinism
///
/// Determinism is inherited from the supplied AST's Serde implementation.
/// This function does not insert nondeterministic values.
///
/// For deterministic maps, AST data should use ordered structures such as
/// `BTreeMap`.
///
/// # Memory
///
/// The resulting `String` necessarily contains the complete JSON document.
/// For very large ASTs prefer [`to_json_writer`] to avoid additionally
/// materializing the entire serialized document.
///
/// # Errors
///
/// Returns [`AstJsonError::Serialize`] if serialization fails.
pub fn to_json<T>(value: &T) -> Result<String, AstJsonError>
where
    T: Serialize,
{
    let envelope = AstJsonEnvelope::new(value);

    serde_json::to_string(&envelope).map_err(AstJsonError::Serialize)
}

/// Serializes an AST value into UTF-8 JSON bytes.
///
/// This is equivalent to [`to_json`] followed by UTF-8 conversion, but uses
/// `serde_json::to_vec` directly to avoid constructing an intermediate UTF-8
/// `String`.
///
/// # Errors
///
/// Returns [`AstJsonError::Serialize`] if serialization fails.
pub fn to_json_bytes<T>(value: &T) -> Result<Vec<u8>, AstJsonError>
where
    T: Serialize,
{
    let envelope = AstJsonEnvelope::new(value);

    serde_json::to_vec(&envelope).map_err(AstJsonError::Serialize)
}

/// Serializes an AST value into a writer without first constructing a complete
/// JSON `String` or `Vec<u8>`.
///
/// This is the preferred API for very large ASTs when the caller has a suitable
/// output stream.
///
/// The writer is supplied by the caller. This function never opens files,
/// sockets or other external resources itself.
///
/// # Writer semantics
///
/// Bytes already accepted by the writer remain written if a later serialization
/// error occurs. Callers requiring atomic persistence should write to their own
/// temporary destination and perform the atomic replacement themselves.
///
/// # Errors
///
/// Returns:
///
/// - [`AstJsonError::Serialize`] for Serde JSON serialization errors;
/// - [`AstJsonError::Io`] for writer failures.
pub fn to_json_writer<T, W>(value: &T, writer: &mut W) -> Result<(), AstJsonError>
where
    T: Serialize,
    W: Write,
{
    let envelope = AstJsonEnvelope::new(value);

    let mut serializer = serde_json::Serializer::new(writer);

    envelope
        .serialize(&mut serializer)
        .map_err(AstJsonError::Serialize)
}

/// Serializes an AST value into a writer using pretty-printed JSON.
///
/// This is intended for human-readable diagnostics, debugging and source
/// tooling. It is not the preferred representation for content-addressed
/// storage or canonical hashing because whitespace is intentionally included.
///
/// # Scalability
///
/// Serialization is streaming with respect to the output writer.
///
/// The AST itself remains in memory as supplied by the caller.
pub fn to_json_writer_pretty<T, W>(
    value: &T,
    writer: &mut W,
) -> Result<(), AstJsonError>
where
    T: Serialize,
    W: Write,
{
    let envelope = AstJsonEnvelope::new(value);

    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut serializer = serde_json::Serializer::with_formatter(writer, formatter);

    envelope
        .serialize(&mut serializer)
        .map_err(AstJsonError::Serialize)
}

/// Deserializes a versioned Zamani AST JSON string.
///
/// The complete input must contain exactly one JSON document. Trailing
/// non-whitespace data is rejected by `serde_json`.
///
/// # Important
///
/// Successful JSON deserialization establishes only that the document matches
/// the serialization format and the requested Rust/Serde data model. It does
/// NOT establish semantic validity.
///
/// Callers must still perform:
///
/// ```text
/// JSON
///   ↓
/// AST deserialization
///   ↓
/// structural AST validation
///   ↓
/// semantic analysis
///   ↓
/// ZUIR lowering
/// ```
///
/// # Errors
///
/// Returns an [`AstJsonError`] when JSON is malformed or when the envelope is
/// incompatible with this implementation.
pub fn from_json<T>(json: &str) -> Result<T, AstJsonError>
where
    T: DeserializeOwned,
{
    let envelope: AstJsonEnvelope<T> =
        serde_json::from_str(json).map_err(AstJsonError::Deserialize)?;

    validate_envelope(&envelope)?;

    Ok(envelope.into_payload())
}

/// Deserializes a versioned Zamani AST JSON byte sequence.
///
/// The input must be valid UTF-8 JSON.
///
/// Prefer this function over converting a byte buffer to a `String` manually,
/// because `serde_json` can report malformed input directly.
pub fn from_json_bytes<T>(bytes: &[u8]) -> Result<T, AstJsonError>
where
    T: DeserializeOwned,
{
    let envelope: AstJsonEnvelope<T> =
        serde_json::from_slice(bytes).map_err(AstJsonError::Deserialize)?;

    validate_envelope(&envelope)?;

    Ok(envelope.into_payload())
}

/// Deserializes a versioned Zamani AST JSON document from a reader.
///
/// This is the preferred API when the JSON document is stored in a large file,
/// pipe or other stream and the caller does not want to materialize the whole
/// document before parsing.
///
/// The reader is supplied by the caller; this module performs no implicit
/// filesystem or network access.
///
/// # Resource policy
///
/// This function intentionally does not impose a fixed byte limit. Applications
/// processing untrusted input should place the reader behind an explicit
/// resource-limiting policy appropriate to their environment.
///
/// Such a policy is intentionally external to the language AST.
///
/// # Errors
///
/// Returns [`AstJsonError::Io`] for reader failures and
/// [`AstJsonError::Deserialize`] for malformed JSON.
pub fn from_json_reader<T, R>(reader: R) -> Result<T, AstJsonError>
where
    T: DeserializeOwned,
    R: Read,
{
    let mut deserializer = serde_json::Deserializer::from_reader(reader);

    let envelope: AstJsonEnvelope<T> =
        serde::Deserialize::deserialize(&mut deserializer)
            .map_err(AstJsonError::Deserialize)?;

    validate_envelope(&envelope)?;

    Ok(envelope.into_payload())
}

/// Validates the serialization envelope independently of its payload.
///
/// This is public so callers that deserialize the envelope themselves can
/// reuse the exact compatibility policy.
///
/// The function does not validate the AST payload semantically.
pub fn validate_envelope<T>(
    envelope: &AstJsonEnvelope<T>,
) -> Result<(), AstJsonError> {
    if envelope.format != AST_JSON_FORMAT {
        return Err(AstJsonError::InvalidFormat {
            actual: envelope.format.clone(),
        });
    }

    if envelope.format_version > MAX_SUPPORTED_AST_JSON_FORMAT_VERSION {
        return Err(AstJsonError::UnsupportedFormatVersion {
            actual: envelope.format_version,
            maximum_supported: MAX_SUPPORTED_AST_JSON_FORMAT_VERSION,
        });
    }

    if envelope.format_version == 0 {
        return Err(AstJsonError::UnsupportedFormatVersion {
            actual: envelope.format_version,
            maximum_supported: MAX_SUPPORTED_AST_JSON_FORMAT_VERSION,
        });
    }

    Ok(())
}

/// Returns the currently supported JSON serialization-format version.
///
/// This is preferable to duplicating the version constant throughout tooling.
#[must_use]
pub const fn format_version() -> u16 {
    AST_JSON_FORMAT_VERSION
}

/// Returns the stable JSON format identifier.
#[must_use]
pub const fn format_identifier() -> &'static str {
    AST_JSON_FORMAT
}

/// Returns whether a serialization-format version is understood by this
/// implementation.
///
/// Version `0` is intentionally rejected because a versionless document is
/// not a member of the production versioned format.
///
/// Future versions must not be silently accepted until their compatibility
/// contract has been implemented.
#[must_use]
pub const fn supports_format_version(version: u16) -> bool {
    version >= 1 && version <= MAX_SUPPORTED_AST_JSON_FORMAT_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;
    use std::io::Cursor;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct TestAst {
        name: String,
        nodes: Vec<TestNode>,
        metadata: BTreeMap<String, String>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct TestNode {
        id: u64,
        kind: String,
    }

    fn sample_ast() -> TestAst {
        let mut metadata = BTreeMap::new();

        metadata.insert("zeta".to_owned(), "last".to_owned());
        metadata.insert("alpha".to_owned(), "first".to_owned());

        TestAst {
            name: "scalable-program".to_owned(),
            nodes: vec![
                TestNode {
                    id: 0,
                    kind: "program".to_owned(),
                },
                TestNode {
                    id: 1,
                    kind: "operation".to_owned(),
                },
            ],
            metadata,
        }
    }

    #[test]
    fn round_trip_string() {
        let original = sample_ast();

        let encoded = to_json(&original).expect("serialization must succeed");
        let decoded: TestAst =
            from_json(&encoded).expect("deserialization must succeed");

        assert_eq!(decoded, original);
    }

    #[test]
    fn round_trip_bytes() {
        let original = sample_ast();

        let encoded =
            to_json_bytes(&original).expect("serialization must succeed");

        let decoded: TestAst =
            from_json_bytes(&encoded).expect("deserialization must succeed");

        assert_eq!(decoded, original);
    }

    #[test]
    fn round_trip_writer() {
        let original = sample_ast();

        let mut output = Vec::new();

        to_json_writer(&original, &mut output)
            .expect("stream serialization must succeed");

        let decoded: TestAst =
            from_json_bytes(&output).expect("deserialization must succeed");

        assert_eq!(decoded, original);
    }

    #[test]
    fn round_trip_reader() {
        let original = sample_ast();

        let encoded =
            to_json_bytes(&original).expect("serialization must succeed");

        let reader = Cursor::new(encoded);

        let decoded: TestAst =
            from_json_reader(reader).expect("stream deserialization must succeed");

        assert_eq!(decoded, original);
    }

    #[test]
    fn pretty_writer_produces_valid_json() {
        let original = sample_ast();

        let mut output = Vec::new();

        to_json_writer_pretty(&original, &mut output)
            .expect("pretty serialization must succeed");

        let decoded: TestAst =
            from_json_bytes(&output).expect("pretty JSON must deserialize");

        assert_eq!(decoded, original);

        let text =
            String::from_utf8(output).expect("JSON output must be UTF-8");

        assert!(text.contains('\n'));
        assert!(text.contains("  "));
    }

    #[test]
    fn envelope_has_stable_format_identifier() {
        let envelope = AstJsonEnvelope::new(sample_ast());

        assert_eq!(envelope.format(), AST_JSON_FORMAT);
        assert_eq!(envelope.format_version(), AST_JSON_FORMAT_VERSION);
    }

    #[test]
    fn current_version_is_supported() {
        assert!(supports_format_version(AST_JSON_FORMAT_VERSION));
    }

    #[test]
    fn_zero_version_is_not_supported() {
        assert!(!supports_format_version(0));
    }

    #[test]
    fn_future_version_is_not_supported() {
        assert!(!supports_format_version(
            AST_JSON_FORMAT_VERSION.saturating_add(1)
        ));
    }

    #[test]
    fn wrong_format_is_rejected() {
        let envelope = AstJsonEnvelope {
            format: "some.other.format".to_owned(),
            format_version: AST_JSON_FORMAT_VERSION,
            payload: sample_ast(),
        };

        let error =
            validate_envelope(&envelope).expect_err("wrong format must fail");

        assert!(matches!(error, AstJsonError::InvalidFormat { .. }));
    }

    #[test]
    fn future_format_version_is_rejected() {
        let envelope = AstJsonEnvelope {
            format: AST_JSON_FORMAT.to_owned(),
            format_version: AST_JSON_FORMAT_VERSION.saturating_add(1),
            payload: sample_ast(),
        };

        let error = validate_envelope(&envelope)
            .expect_err("future format version must fail");

        assert!(matches!(
            error,
            AstJsonError::UnsupportedFormatVersion { .. }
        ));
    }

    #[test]
    fn zero_format_version_is_rejected() {
        let envelope = AstJsonEnvelope {
            format: AST_JSON_FORMAT.to_owned(),
            format_version: 0,
            payload: sample_ast(),
        };

        let error = validate_envelope(&envelope)
            .expect_err("version zero must fail");

        assert!(matches!(
            error,
            AstJsonError::UnsupportedFormatVersion { .. }
        ));
    }

    #[test]
    fn malformed_json_is_rejected() {
        let error =
            from_json::<TestAst>("{not valid json")
                .expect_err("malformed JSON must fail");

        assert!(matches!(error, AstJsonError::Deserialize(_)));
    }

    #[test]
    fn trailing_json_is_rejected() {
        let encoded = to_json(&sample_ast()).expect("serialization must succeed");

        let mut with_trailing = encoded;
        with_trailing.push_str(" trailing");

        let error = from_json::<TestAst>(&with_trailing)
            .expect_err("trailing data must fail");

        assert!(matches!(error, AstJsonError::Deserialize(_)));
    }

    #[test]
    fn trailing_second_json_document_is_rejected() {
        let encoded = to_json(&sample_ast()).expect("serialization must succeed");

        let with_trailing = format!("{encoded}{{}}");

        let error = from_json::<TestAst>(&with_trailing)
            .expect_err("multiple JSON documents must fail");

        assert!(matches!(error, AstJsonError::Deserialize(_)));
    }

    #[test]
    fn unicode_round_trip_is_lossless() {
        let original = TestAst {
            name: "Zamani — 永久 — quantum — ∞".to_owned(),
            nodes: vec![TestNode {
                id: 0,
                kind: "operation_λ".to_owned(),
            }],
            metadata: BTreeMap::new(),
        };

        let encoded = to_json(&original).expect("serialization must succeed");
        let decoded: TestAst =
            from_json(&encoded).expect("deserialization must succeed");

        assert_eq!(decoded, original);
    }

    #[test]
    fn empty_ast_collections_are_valid() {
        let original = TestAst {
            name: String::new(),
            nodes: Vec::new(),
            metadata: BTreeMap::new(),
        };

        let encoded = to_json(&original).expect("serialization must succeed");
        let decoded: TestAst =
            from_json(&encoded).expect("deserialization must succeed");

        assert_eq!(decoded, original);
    }

    #[test]
    fn ordered_maps_produce_stable_output() {
        let mut first_map = BTreeMap::new();
        first_map.insert("z".to_owned(), "3".to_owned());
        first_map.insert("a".to_owned(), "1".to_owned());
        first_map.insert("m".to_owned(), "2".to_owned());

        let mut second_map = BTreeMap::new();
        second_map.insert("m".to_owned(), "2".to_owned());
        second_map.insert("z".to_owned(), "3".to_owned());
        second_map.insert("a".to_owned(), "1".to_owned());

        let first = TestAst {
            name: "stable".to_owned(),
            nodes: Vec::new(),
            metadata: first_map,
        };

        let second = TestAst {
            name: "stable".to_owned(),
            nodes: Vec::new(),
            metadata: second_map,
        };

        let first_json =
            to_json(&first).expect("first serialization must succeed");
        let second_json =
            to_json(&second).expect("second serialization must succeed");

        assert_eq!(first_json, second_json);
    }

    #[test]
    fn large_collection_does_not_depend_on_fixed_ast_size_constant() {
        let node_count = 100_000usize;

        let mut nodes = Vec::with_capacity(node_count);

        for id in 0..node_count {
            nodes.push(TestNode {
                id: id as u64,
                kind: "operation".to_owned(),
            });
        }

        let original = TestAst {
            name: "large-program".to_owned(),
            nodes,
            metadata: BTreeMap::new(),
        };

        let encoded =
            to_json_bytes(&original).expect("large AST serialization must succeed");

        let decoded: TestAst =
            from_json_bytes(&encoded).expect("large AST deserialization must succeed");

        assert_eq!(decoded.nodes.len(), node_count);
        assert_eq!(decoded.nodes[0], original.nodes[0]);
        assert_eq!(
            decoded.nodes[node_count - 1],
            original.nodes[node_count - 1]
        );
    }

    #[test]
    fn envelope_round_trip_preserves_payload() {
        let original = AstJsonEnvelope::new(sample_ast());

        let encoded =
            serde_json::to_string(&original).expect("envelope serialization");

        let decoded: AstJsonEnvelope<TestAst> =
            serde_json::from_str(&encoded).expect("envelope deserialization");

        assert_eq!(decoded, original);
    }

    #[test]
    fn payload_can_be_extracted_without_clone() {
        let original = sample_ast();
        let envelope = AstJsonEnvelope::new(original.clone());

        let extracted = envelope.into_payload();

        assert_eq!(extracted, original);
    }
}