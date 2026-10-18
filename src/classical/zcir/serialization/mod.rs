
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Serialization Facade
//!
//! This module defines the public serialization boundary for Zamani's
//! Classical Intermediate Representation (ZCIR).
//!
//! ## Architectural authority
//!
//! ZCIR integrates with the canonical serialization infrastructure owned by
//! `crate::zuir::serialization`. It must not establish a competing wire
//! format, canonical JSON algorithm, schema-version authority, or document
//! envelope.
//!
//! The source grammar and semantic specifications remain authoritative for
//! source-language meaning. Serialization preserves a representation; it
//! does not establish that the represented program is semantically valid.
//!
//! ## Ownership
//!
//! This facade owns:
//!
//! - The public serialization module boundary for ZCIR.
//! - Stable, documented re-exports of supported serialization entry points.
//! - Integration between canonical rendering and document encoding/decoding.
//!
//! This facade does not own:
//!
//! - The canonical JSON algorithm.
//! - Document format and schema-version negotiation.
//! - ZCIR or ZUIR IR construction and verification.
//! - Source parsing, name resolution, type checking, or semantic analysis.
//! - Hardware discovery, allocation, scheduling, or target selection.
//! - Cryptographic fingerprints, signatures, or authentication.
//!
//! ## Module contracts
//!
//! - [`canonical`]: delegates deterministic JSON rendering to ZUIR.
//! - [`encode`]: exposes canonical document encoding.
//! - [`decode`]: exposes canonical document decoding.
//!
//! Each module must retain its owning implementation's error types and
//! operational-budget semantics. This facade must not wrap, suppress,
//! reinterpret, or flatten structured errors.
//!
//! ## Dependency direction
//!
//! ZCIR serialization adapters may depend on the canonical ZUIR serialization
//! implementation. The canonical ZUIR implementation must not depend on
//! this ZCIR facade. This direction avoids a dependency cycle and preserves
//! one authoritative implementation.
//!
//! ## Scalability and resource policy
//!
//! This facade imposes no fixed universal limits on program size, operation
//! count, collection length, integer width, devices, CPUs, or memory.
//!
//! Operational budgets, such as maximum input bytes, output bytes, traversal
//! depth, or visited nodes, belong to the relevant implementation options.
//! Such budgets are configurable safeguards, not language-semantic limits.
//! Physical resources and implementation-specific constraints still apply.
//!
//! The facade itself allocates no serialization buffers and performs no
//! traversal. Buffering and resource consumption are determined by the
//! selected underlying API.
//!
//! ## Determinism and compatibility
//!
//! Canonical rendering is defined by the ZUIR canonical serializer.
//! Array order is preserved; object-key ordering follows the canonical
//! renderer's contract. Canonical JSON ordering must not be confused with
//! reordering IR operations, effects, or control flow.
//!
//! Changes to canonical bytes, document envelopes, schema identifiers, or
//! version compatibility must be reviewed in their authoritative ZUIR
//! modules, with compatibility tests updated there and in ZCIR adapters.
//!
//! ## Validation boundary
//!
//! Successful decoding proves only that decoding succeeded under the
//! configured format, schema, and resource policies. It does not prove that
//! a payload is a valid ZCIR program or safe to execute.
//!
//! Consumers must invoke the authoritative schema validation and IR
//! verification stages before treating decoded data as a verified program.
//! Target-feasibility analysis and lowering remain separate pipeline stages.
//!
//! ## Integration contract
//!
//! The parent `src/classical/zcir/mod.rs` must declare this module.
//! The classical module root must expose ZCIR only through a reviewed,
//! acyclic module boundary.
//!
//! `canonical.rs`, `encode.rs`, and `decode.rs` must each compile against the
//! actual canonical ZUIR APIs. They must not depend on one another in a way
//! that creates circular ownership. This facade may re-export their public
//! APIs but must not introduce new serialization representations.
//!
//! ## Freeze criteria
//!
//! This module is ready to freeze when all three child modules exist, all
//! re-exported names match their actual declarations, the parent module
//! boundary is valid, encoder/decoder round-trip tests pass, and the
//! repository formatting, compilation, test, and lint gates pass.

// -------------------------------------------------------------------------
// Child modules
// -------------------------------------------------------------------------

/// Canonical JSON rendering, delegated to the canonical ZUIR implementation.
pub mod canonical;

/// Canonical document decoding, delegated to the ZUIR decoder.
pub mod decode;

/// Canonical document encoding, delegated to the ZUIR encoder.
pub mod encode;

// -------------------------------------------------------------------------
// Canonical rendering API
// -------------------------------------------------------------------------

/// Render a JSON value as canonical JSON bytes.
pub use self::canonical::canonical_json_bytes;

/// Render a JSON value as a canonical JSON string.
pub use self::canonical::canonical_json_string;

/// Stream canonical JSON to a writer.
pub use self::canonical::write_canonical_json;

/// Structured errors returned by canonical JSON rendering.
pub use self::canonical::CanonicalError;

/// Configuration for canonical JSON rendering.
pub use self::canonical::CanonicalOptions;

/// Statistics returned by canonical JSON rendering.
pub use self::canonical::CanonicalReport;

// -------------------------------------------------------------------------
// Document decoding API
//
// Explicit `decode_document*` names distinguish document decoding from
// other potential decode operations in the wider compiler.
// -------------------------------------------------------------------------

/// Decode a complete serialized document from bytes.
pub use self::decode::decode_document;

/// Decode a serialized document from a reader.
pub use self::decode::decode_document_from_reader;

/// Decode a serialized document from a reader with explicit options.
pub use self::decode::decode_document_from_reader_with_options;

/// Decode a serialized document from bytes with explicit options.
pub use self::decode::decode_document_with_options;

/// Structured document-decoding errors.
pub use self::decode::DecodeError;

/// Decoder options, including configured input resource budgets.
pub use self::decode::DecodeOptions;

/// A successfully decoded document envelope and payload.
pub use self::decode::DecodedDocument;

// -------------------------------------------------------------------------
// Document encoding API
// -------------------------------------------------------------------------

/// Encode a document using the canonical ZUIR document format.
pub use self::encode::encode;

/// Encode a document to a writer.
pub use self::encode::encode_to_writer;

/// Encode a document to a writer with explicit options.
pub use self::encode::encode_to_writer_with_options;

/// Encode a document with explicit options.
pub use self::encode::encode_with_options;

/// Structured document-encoding errors.
pub use self::encode::EncodeError;

/// Encoder options, including configured output resource budgets.
pub use self::encode::EncodeOptions;

/// The result of encoding a document.
pub use self::encode::EncodedArtifact;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_rendering_api_is_exposed() {
        let value = serde_json::json!({
            "z": 1,
            "a": 2
        });

        let rendered = canonical_json_string(
            &value,
            CanonicalOptions::default(),
        )
        .expect("canonical rendering should succeed");

        assert_eq!(rendered, r#"{"a":2,"z":1}"#);
    }

    #[test]
    fn canonical_bytes_match_the_canonical_string() {
        let value = serde_json::json!({
            "classical": true,
            "values": [1, 2, 3]
        });

        let bytes = canonical_json_bytes(
            &value,
            CanonicalOptions::default(),
        )
        .expect("canonical byte rendering should succeed");

        let string = canonical_json_string(
            &value,
            CanonicalOptions::default(),
        )
        .expect("canonical string rendering should succeed");

        assert_eq!(bytes, string.as_bytes());
    }

    #[test]
    fn document_encoding_and_decoding_round_trip() {
        let payload = serde_json::json!({
            "kind": "classical-conformance-fixture",
            "operations": [
                {"name": "constant"},
                {"name": "return"}
            ]
        });

        let encoded = encode(&payload)
            .expect("document encoding should succeed");

        let decoded = decode_document(encoded.as_bytes())
            .expect("the canonical document should decode");

        assert_eq!(decoded.payload(), &payload);
    }
}
