#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Serialization Encoder Adapter
//!
//! Path: `src/classical/zcir/serialization/encode.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the classical ZCIR serialization entry point while preserving
//! ZUIR's canonical document encoding, versioning, deterministic JSON
//! representation, output-budget enforcement, and error contracts.
//!
//! ZCIR is an integration boundary for classical computation. It must not
//! establish a second, competing universal IR or serialization protocol.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/serialization/encode.rs`
//! - `src/zuir/serialization/decode.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/classical/zcir/`
//!
//! The grammar owns syntax. Semantic analysis owns established source
//! meaning. ZUIR owns the canonical universal representation and its
//! document serialization contract. This file owns only the ZCIR-facing
//! serialization integration boundary.
//!
//! ## OWNS
//!
//! - The ZCIR module path for canonical encoding operations.
//! - Re-export of the canonical encoder's public API.
//! - Documentation of the ZCIR-to-ZUIR serialization contract.
//!
//! ## DOES NOT OWN
//!
//! - A second JSON envelope or binary serialization format.
//! - Independent schema, format-version, or compatibility definitions.
//! - IR construction, semantic analysis, or whole-IR verification.
//! - A duplicate canonical JSON implementation.
//! - Classical operation definitions or operation registries.
//! - Hardware allocation, memory layout selection, scheduling, or execution.
//! - Cryptographic fingerprints, signatures, or integrity guarantees.
//!
//! ## DEPENDENCY CONTRACT
//!
//! This adapter depends on:
//!
//! `crate::zuir::serialization::encode`
//!
//! The canonical implementation owns:
//!
//! - `EncodeOptions`
//! - `EncodeError`
//! - `EncodedArtifact`
//! - `encode`
//! - `encode_with_options`
//! - `encode_to_writer`
//! - `encode_to_writer_with_options`
//!
//! This file must not reimplement those operations. The canonical encoder
//! remains the sole owner of the document envelope and its byte contract.
//!
//! ## PUBLIC API CONTRACT
//!
//! The re-exported `encode` function accepts a Serde-serializable payload
//! and produces the canonical ZUIR versioned JSON document.
//!
//! The options-based entry point supports caller-selected schema identity,
//! schema version, and an optional maximum output size.
//!
//! Writer-based entry points write the completed encoded artifact to the
//! caller's destination. A destination may fail after accepting some bytes;
//! callers must treat that write as failed and must not consume partial
//! output as a valid complete document.
//!
//! ## DETERMINISM
//!
//! Determinism is inherited from the canonical ZUIR encoder and the
//! payload's `Serialize` implementation. Object-key ordering is canonicalized
//! by that implementation, while array order is preserved.
//!
//! This adapter makes no stronger determinism guarantee than its canonical
//! dependency. In particular, callers must not assume that semantically
//! equivalent payloads with different representations produce identical
//! bytes unless the relevant representation contract guarantees that result.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This file introduces no fixed limits on:
//!
//! - operation, block, function, or value counts;
//! - integer widths or register widths;
//! - collection lengths or tensor rank;
//! - processor, accelerator, or distributed-node counts;
//! - memory capacity or machine size.
//!
//! The canonical encoder's optional output-byte budget is an operational
//! policy, not a language or IR semantic limit. The default configuration
//! imposes no additional explicit output-byte limit.
//!
//! The canonical encoder stages its output in memory. Consequently, peak
//! memory consumption depends on payload representation, encoded size,
//! allocator behavior, and available resources. A configured byte budget
//! bounds encoded output; it does not guarantee a corresponding bound on
//! total peak memory because payload conversion and canonicalization also
//! require resources.
//!
//! No finite implementation can guarantee literal infinity. POCO-REAF
//! requires avoiding arbitrary universal ceilings and preserving program
//! meaning across supported targets, subject to resources and capabilities.
//!
//! ## SECURITY AND TRUST
//!
//! Successful encoding is not proof that a payload is a valid ZCIR or ZUIR
//! program. The caller must use the authoritative schema checks and IR
//! verifier before treating externally supplied or constructed IR as valid.
//!
//! Encoding does not authorize extensions, verify operation semantics,
//! execute payloads, or establish authenticity.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. `src/classical/zcir/serialization/mod.rs` must declare this module.
//! 2. `src/classical/zcir/serialization/mod.rs` may re-export these public
//!    items for callers that use the serialization facade.
//! 3. `src/classical/zcir/mod.rs` must register the ZCIR module hierarchy
//!    using valid Rust module declarations.
//! 4. ZCIR producers must supply the canonical representation or a
//!    deliberately specified Serde representation of their payload.
//! 5. The canonical ZUIR decoder must consume the produced document.
//! 6. The decoded payload must undergo applicable schema validation and
//!    authoritative IR verification before trusted downstream use.
//! 7. `canonical.rs`, if present in the ZCIR tree, must not introduce a
//!    competing canonical JSON implementation.
//! 8. `decode.rs`, if present in the ZCIR tree, must delegate to or adapt
//!    the canonical ZUIR decoder rather than invent a separate envelope.
//! 9. `compatibility.rs` must not claim format compatibility without
//!    checking the actual schema and format contracts.
//! 10. Content fingerprints must use the repository's designated
//!     fingerprint implementation and the specified canonical bytes.
//!
//! No other ZCIR file needs to be edited merely to implement encoding.
//! Parent-module declarations are required for compilation, but should be
//! completed and validated as their own integration task.
//!
//! ## TESTING CONTRACT
//!
//! The authoritative encoder's test suite must cover:
//!
//! - deterministic output for identical supported payloads and options;
//! - successful encoding of small and empty payloads where valid;
//! - preservation of ordered sequences;
//! - output-budget boundaries, including a zero-byte budget;
//! - schema-identifier validation;
//! - writer failure and partial-write behavior;
//! - interoperability with the canonical decoder;
//! - malformed or unsupported payload behavior;
//! - allocation and resource-exhaustion error handling.
//!
//! ZCIR integration tests must additionally prove that a representative
//! classical payload can be encoded, decoded, schema-validated, and passed
//! to the authoritative verifier without changing its meaning.
//!
//! ## FREEZE CRITERIA
//!
//! Freeze this file only after:
//!
//! - the canonical ZUIR encoder API exists at the referenced path;
//! - the ZCIR serialization module is registered;
//! - the actual repository compiles with Rust 1.97 or later;
//! - the ZCIR-to-ZUIR encode/decode integration tests pass;
//! - no duplicate serialization implementation has been introduced;
//! - formatting, tests, and lint checks pass.
//!
//! This adapter intentionally contains no independent encoding algorithm.
//! Production readiness depends on validating the canonical encoder and
//! the complete integration path in the actual repository.

// Reuse the canonical ZUIR implementation without wrapping or changing
// its envelope, format version, error semantics, or output-budget behavior.
pub use crate::zuir::serialization::encode::{
    encode,
    encode_to_writer,
    encode_to_writer_with_options,
    encode_with_options,
    EncodeError,
    EncodeOptions,
    EncodedArtifact,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    struct ClassicalPayload {
        operation: String,
        operands: Vec<u64>,
    }

    #[test]
    fn zcir_entry_point_uses_canonical_zuir_document_envelope() {
        let payload = ClassicalPayload {
            operation: "classical.example".to_owned(),
            operands: vec![1, 2, 3],
        };

        let artifact = encode(&payload)
            .expect("a serializable classical payload should encode");

        let document: serde_json::Value =
            serde_json::from_slice(artifact.as_bytes())
                .expect("the canonical encoder must produce valid JSON");

        assert_eq!(
            document.get("schema_id").and_then(|value| value.as_str()),
            Some("zamani.zuir.serialization.document"),
        );

        assert!(document.get("format_version").is_some());
        assert!(document.get("schema_version").is_some());

        assert_eq!(
            document
                .get("payload")
                .and_then(|value| value.get("operation"))
                .and_then(|value| value.as_str()),
            Some("classical.example"),
        );

        assert_eq!(
            document
                .get("payload")
                .and_then(|value| value.get("operands")),
            Some(&serde_json::json!([1, 2, 3])),
        );
    }

    #[test]
    fn zcir_entry_point_produces_repeatable_bytes() {
        let payload = ClassicalPayload {
            operation: "classical.example".to_owned(),
            operands: vec![3, 2, 1],
        };

        let first = encode(&payload)
            .expect("first encoding should succeed");
        let second = encode(&payload)
            .expect("second encoding should succeed");

        assert_eq!(first.as_bytes(), second.as_bytes());
    }

    #[test]
    fn zcir_entry_point_respects_canonical_output_budget() {
        let payload = ClassicalPayload {
            operation: "classical.example".to_owned(),
            operands: vec![1],
        };

        let options = EncodeOptions::default()
            .with_max_output_bytes(Some(0));

        let result = encode_with_options(&payload, &options);

        assert!(matches!(
            result,
            Err(EncodeError::OutputBudgetExceeded { limit: 0 })
        ));
    }

    #[test]
    fn writer_entry_point_emits_the_same_artifact_bytes() {
        let payload = ClassicalPayload {
            operation: "classical.example".to_owned(),
            operands: vec![7, 11],
        };

        let artifact = encode(&payload)
            .expect("encoding should succeed");

        let mut output = Vec::new();
        encode_to_writer(&payload, &mut output)
            .expect("writing the canonical document should succeed");

        assert_eq!(output, artifact.as_bytes());
    }
}