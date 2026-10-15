#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Serialization Facade
//!
//! Canonical path: `src/zuir/serialization/mod.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the canonical public module boundary for ZUIR serialization.
//! Expose the encoder, decoder, canonical JSON renderer, schema contracts,
//! and serialization validation APIs without duplicating their logic.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/serialization/encode.rs`
//! - `src/zuir/serialization/decode.rs`
//! - `src/zuir/serialization/canonical.rs`
//! - `src/zuir/serialization/validation.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! The grammar owns source syntax. The semantic model owns source meaning.
//! ZUIR owns its computational representation. This module owns only the
//! serialization module boundary and its deliberate public exports.
//!
//! ## OWNS
//!
//! - Serialization submodule declarations.
//! - Stable, intentional re-exports for downstream Rust callers.
//! - Documentation of the serialization integration boundary.
//!
//! ## DOES_NOT_OWN
//!
//! - JSON encoding or decoding algorithms.
//! - Canonical JSON rendering algorithms.
//! - Schema definitions, version negotiation, or schema migration.
//! - Payload validation or whole-program semantic verification.
//! - AST parsing, grammar interpretation, or semantic analysis.
//! - Cryptographic fingerprints, signatures, or checksums.
//! - Domain-specific operation semantics or physical hardware allocation.
//!
//! ## DEPENDENCY CONTRACT
//!
//! This facade may depend on its child modules through public re-exports.
//! Child modules must not depend on this facade's re-export aliases.
//! Where internal module-to-module imports are needed, those modules should
//! use their explicit module paths.
//!
//! This arrangement avoids circular dependencies and keeps implementation
//! ownership within the module that defines each API.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `canonical.rs` owns deterministic JSON rendering.
//! - `schema.rs` owns schema descriptors, identifiers, and schema policy.
//! - `encode.rs` owns document encoding and its envelope.
//! - `decode.rs` owns document decoding, envelope checks, and input budgets.
//! - `validation.rs` owns schema and JSON-payload structural validation.
//! - The ZUIR verifier owns semantic validity of a complete IR program.
//! - `fingerprint/` owns stable content fingerprints, if implemented.
//! - `interchange/` owns external-format import/export and migrations, if
//!   implemented.
//!
//! The decoder does not by itself prove that a decoded payload is a valid
//! ZUIR program. Callers must perform the appropriate payload/schema checks
//! and then invoke the authoritative IR verifier before treating untrusted
//! data as verified IR.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This facade imposes no universal limit on operation count, qubits,
//! processor count, memory capacity, tensor rank, register width, devices,
//! nodes, or topology size.
//!
//! Resource budgets remain explicit options owned by the relevant operation.
//! A resource-budget failure must not be confused with semantic invalidity.
//! No finite process can guarantee literal infinity or unlimited resources.
//!
//! ## COMPATIBILITY
//!
//! Public exports are intentional. Adding a new implementation module does
//! not automatically make every internal helper part of the stable API.
//! Existing serialized documents remain governed by the schema and format
//! version contracts, not by Rust module paths.
//!
//! Renaming or removing a public export is an API change and must follow the
//! repository's compatibility policy.
//!
//! ## FREEZE CRITERIA
//!
//! - All declared child files exist in the target branch.
//! - The parent ZUIR module exposes this module.
//! - Re-exported symbols match the implementation files.
//! - The crate compiles with the repository's supported Rust toolchain.
//! - Serialization tests cover deterministic rendering, round trips,
//!   malformed envelopes, version incompatibility, and budget failures.
//! - No duplicate implementation or unsafe code is introduced.

/// Deterministic JSON rendering.
///
/// This module canonicalizes a JSON value; it does not validate the
/// computational meaning of a ZUIR program.
pub mod canonical;

/// Decoding of versioned serialized ZUIR documents.
pub mod decode;

/// Encoding of versioned serialized ZUIR documents.
pub mod encode;

/// Serialization schema descriptors and compatibility contracts.
pub mod schema;

/// Structural validation of schema descriptors and JSON payloads.
pub mod validation;

// ---------------------------------------------------------------------------
// Canonical JSON API
// ---------------------------------------------------------------------------

pub use self::canonical::{
    canonical_json_bytes,
    canonical_json_string,
    write_canonical_json,
    CanonicalError,
    CanonicalOptions,
    CanonicalReport,
};

// ---------------------------------------------------------------------------
// Document decoding API
//
// Aliases avoid confusion between the `decode` module and the decoding
// entry-point function when callers use the facade.
// ---------------------------------------------------------------------------

pub use self::decode::{
    decode as decode_document,
    decode_from_reader as decode_document_from_reader,
    decode_from_reader_with_options as decode_document_from_reader_with_options,
    decode_with_options as decode_document_with_options,
    DecodeError,
    DecodedDocument,
    DecodeOptions,
};

// ---------------------------------------------------------------------------
// Document encoding API
// ---------------------------------------------------------------------------

pub use self::encode::{
    encode as encode_document,
    encode_to_writer as encode_document_to_writer,
    encode_to_writer_with_options as encode_document_to_writer_with_options,
    encode_with_options as encode_document_with_options,
    EncodeError,
    EncodedArtifact,
    EncodeOptions,
};

// ---------------------------------------------------------------------------
// Schema API
// ---------------------------------------------------------------------------

pub use self::schema::{
    CompatibilityPolicy,
    FieldCardinality,
    FieldDescriptor,
    FieldId,
    SchemaDescriptor,
    SchemaError,
    SchemaId,
    SchemaKind,
    SchemaManifest,
    SchemaValidationLimits,
    SchemaVersion,
    UnknownFieldPolicy,
    WireKind,
    CURRENT_SCHEMA_VERSION,
    DOCUMENT_SCHEMA_ID,
    PAYLOAD_SCHEMA_ID,
    SCHEMA_NAMESPACE,
};

// ---------------------------------------------------------------------------
// Structural validation API
// ---------------------------------------------------------------------------

pub use self::validation::{
    validate_payload,
    validate_payload_unbounded,
    validate_schema,
    validate_schema_compatibility,
    ValidationError,
    ValidationLimits,
    ValidationReport,
    ValuePath,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_exposes_canonical_json_api() {
        let value = serde_json::json!({
            "z": 1,
            "a": 2
        });

        let output = canonical_json_string(
            &value,
            CanonicalOptions::unbounded(),
        )
        .expect("a valid JSON value should be rendered");

        assert_eq!(output, r#"{"a":2,"z":1}"#);
    }

    #[test]
    fn facade_exposes_configurable_validation_limits() {
        let limits = ValidationLimits::unbounded();

        // The facade exposes validation configuration without imposing a
        // universal IR-size or hardware-capacity limit.
        let _ = limits;
    }

    #[test]
    fn facade_exposes_schema_version_contract() {
        let current = SchemaVersion::current();

        assert_eq!(
            current,
            CURRENT_SCHEMA_VERSION,
        );
    }
}