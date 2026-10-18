#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Serialization Decoder Adapter
//!
//! Path: `src/classical/zcir/serialization/decode.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Expose the canonical ZUIR document decoder at the classical ZCIR
//! integration boundary. Preserve the canonical document format, version
//! checks, schema identity checks, input budgets, and error behavior.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/serialization/encode.rs`
//! - `src/zuir/serialization/decode.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/serialization/validation.rs`
//! - `src/classical/zcir/serialization/encode.rs`
//!
//! The grammar defines source syntax. The semantic model defines program
//! meaning. ZUIR owns the canonical universal representation and document
//! format. This module owns the ZCIR-facing decoding boundary only.
//!
//! ## OWNS
//!
//! - The classical-facing decoder API.
//! - Re-exports of canonical decoder types and entry points.
//! - Documentation of the decode and verification boundary.
//!
//! ## DOES NOT OWN
//!
//! - A second JSON envelope or independent decoder implementation.
//! - Independent format versions, schema identifiers, or migration rules.
//! - Classical operation definitions or IR verification.
//! - Parsing Zamani source code or performing semantic analysis.
//! - Resource allocation, scheduling, lowering, or hardware execution.
//! - Authenticity, authorization, cryptographic signatures, or trust policy.
//!
//! ## DEPENDENCY CONTRACT
//!
//! Canonical implementation:
//!
//! `crate::zuir::serialization::decode`
//!
//! Canonical API:
//!
//! - `decode`
//! - `decode_with_options`
//! - `decode_from_reader`
//! - `decode_from_reader_with_options`
//! - `DecodeOptions`
//! - `DecodeError`
//! - `DecodedDocument`
//!
//! This module re-exports those definitions without wrapping or modifying
//! their behavior. The canonical decoder remains the sole owner of input
//! parsing, envelope validation, version compatibility, and byte budgeting.
//!
//! ## TRUST BOUNDARY
//!
//! Successful decoding proves that the canonical decoder accepted the
//! document envelope and its declared versions. It does not prove that the
//! payload is a valid ZCIR program, that its operations are supported, or
//! that execution is safe.
//!
//! Before a decoded payload is used for compilation or execution, callers
//! must perform the applicable schema validation, structural IR verification,
//! type and effect verification, and target-feasibility checks.
//!
//! ## SCALABILITY
//!
//! This adapter introduces no universal limits on:
//!
//! - operation, block, function, or value counts;
//! - integer or register widths;
//! - aggregate sizes or tensor rank;
//! - processor, accelerator, or distributed-node counts;
//! - memory capacity or hardware scale.
//!
//! Input-byte budgets are optional operational safeguards inherited from
//! the canonical decoder. They are not language-level capacity limits.
//!
//! The canonical JSON decoder materializes input and payload data in memory.
//! Memory use therefore depends on document size, representation overhead,
//! allocation behavior, and available resources. No implementation can
//! guarantee literal infinity or recover from every process-level OOM.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. `src/classical/zcir/serialization/mod.rs` declares `pub mod decode;`.
//! 2. The serialization facade may re-export this module's public API.
//! 3. The ZCIR encoder uses the canonical ZUIR document format.
//! 4. This decoder delegates to the matching canonical ZUIR decoder.
//! 5. The schema validation layer validates the decoded payload.
//! 6. The authoritative IR verifier checks structural and semantic invariants.
//! 7. Compiler and runtime integrations handle unsupported features and
//!    unsatisfied target requirements explicitly.
//! 8. Any schema migration is performed by an explicitly specified migration
//!    layer; this adapter does not silently reinterpret older formats.
//! 9. Canonical IDs, source maps, diagnostics, and provenance remain owned by
//!    their established repository modules.
//!
//! No change to the canonical decoder is required merely to add this adapter.
//! Parent-module declarations remain a separate integration requirement.
//!
//! ## ERROR CONTRACT
//!
//! Errors are the canonical `DecodeError` values. This adapter does not
//! flatten failures into strings or replace structured errors with defaults.
//! In particular, malformed JSON, incompatible versions, unexpected schema
//! identities, input-budget exhaustion, allocation failures, and reader I/O
//! errors remain distinguishable.
//!
//! ## DETERMINISM
//!
//! Decoding does not reorder arrays, canonicalize payloads, or claim that
//! distinct serialized representations have identical semantics. Canonical
//! byte representation is the encoder's responsibility.
//!
//! ## FREEZE CRITERIA
//!
//! - The canonical decoder API exists at the declared path.
//! - The ZCIR serialization module is registered.
//! - Encoder/decoder interoperability tests pass.
//! - Configured input limits are tested at their boundaries.
//! - Malformed input and unsupported versions are rejected.
//! - The repository compiles and passes its formatting, test, and lint gates.
//!
//! This file deliberately implements no independent decoding algorithm.

pub use crate::zuir::serialization::decode::{
    decode as decode_document,
    decode_from_reader as decode_document_from_reader,
    decode_from_reader_with_options as decode_document_from_reader_with_options,
    decode_with_options as decode_document_with_options,
    DecodeError,
    DecodeOptions,
    DecodedDocument,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::io::Cursor;

    use crate::zuir::serialization::encode::{
        encode as encode_document,
        encode_with_options as encode_document_with_options,
        EncodeError,
        EncodeOptions,
    };

    fn classical_payload() -> Value {
        json!({
            "domain": "classical",
            "operations": [
                {
                    "name": "integer.add",
                    "operands": [10, 32],
                    "result": 42
                },
                {
                    "name": "integer.multiply",
                    "operands": [6, 7],
                    "result": 42
                }
            ],
            "metadata": {
                "portable": true,
                "hardware_specific": false
            }
        })
    }

    #[test]
    fn decodes_a_document_created_by_the_canonical_encoder() {
        let payload = classical_payload();

        let encoded =
            encode_document(&payload).expect("canonical encoding should succeed");

        let decoded = decode_document(encoded.as_bytes())
            .expect("the canonical decoder should accept its encoder's output");

        assert_eq!(decoded.payload(), &payload);
    }

    #[test]
    fn reader_and_slice_entry_points_agree() {
        let payload = classical_payload();

        let encoded =
            encode_document(&payload).expect("encoding should succeed");

        let from_slice = decode_document(encoded.as_bytes())
            .expect("slice decoding should succeed");

        let mut reader = Cursor::new(encoded.as_bytes());

        let from_reader = decode_document_from_reader(&mut reader)
            .expect("reader decoding should succeed");

        assert_eq!(from_slice, from_reader);
    }

    #[test]
    fn configured_input_budget_accepts_exact_document_size() {
        let payload = classical_payload();

        let encoded =
            encode_document(&payload).expect("encoding should succeed");

        let exact_limit = DecodeOptions::default()
            .with_max_input_bytes(Some(encoded.len()));

        let decoded = decode_document_with_options(
            encoded.as_bytes(),
            &exact_limit,
        )
        .expect("an input exactly at the configured limit should succeed");

        assert_eq!(decoded.payload(), &payload);
    }

    #[test]
    fn configured_input_budget_rejects_one_byte_too_small() {
        let payload = classical_payload();

        let encoded =
            encode_document(&payload).expect("encoding should succeed");

        let limit = encoded.len().saturating_sub(1);

        let options =
            DecodeOptions::default().with_max_input_bytes(Some(limit));

        let result =
            decode_document_with_options(encoded.as_bytes(), &options);

        assert!(matches!(
            result,
            Err(DecodeError::InputBudgetExceeded { limit: actual })
                if actual == limit
        ));
    }

    #[test]
    fn configured_reader_budget_is_enforced() {
        let payload = classical_payload();

        let encoded =
            encode_document(&payload).expect("encoding should succeed");

        let limit = encoded.len().saturating_sub(1);

        let options =
            DecodeOptions::default().with_max_input_bytes(Some(limit));

        let mut reader = Cursor::new(encoded.as_bytes());

        let result = decode_document_from_reader_with_options(
            &mut reader,
            &options,
        );

        assert!(matches!(
            result,
            Err(DecodeError::InputBudgetExceeded { limit: actual })
                if actual == limit
        ));
    }

    #[test]
    fn malformed_input_is_rejected() {
        let result = decode_document(b"{not-valid-json");

        assert!(matches!(result, Err(DecodeError::Json(_))));
    }

    #[test]
    fn empty_input_is_rejected() {
        let result = decode_document(b"");

        assert!(matches!(result, Err(DecodeError::Json(_))));
    }

    #[test]
    fn output_budget_error_from_the_encoder_remains_structured() {
        let payload = classical_payload();

        let options =
            EncodeOptions::default().with_max_output_bytes(Some(0));

        let result =
            encode_document_with_options(&payload, &options);

        assert!(matches!(
            result,
            Err(EncodeError::OutputBudgetExceeded { limit: 0 })
        ));
    }

    #[test]
    fn decoding_does_not_claim_to_verify_program_semantics() {
        let payload = json!({
            "domain": "classical",
            "operations": [
                {
                    "name": "unregistered.operation",
                    "operands": []
                }
            ]
        });

        let encoded =
            encode_document(&payload).expect("JSON payload should encode");

        let decoded = decode_document(encoded.as_bytes())
            .expect("valid envelope should decode");

        // This test intentionally checks only transport round-tripping.
        // Operation registration and semantic validity belong to the
        // authoritative schema and IR verification layers.
        assert_eq!(decoded.payload(), &payload);
    }
}