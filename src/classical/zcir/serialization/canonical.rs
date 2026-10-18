
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Canonical Serialization Adapter
//!
//! Path: `src/classical/zcir/serialization/canonical.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Expose ZUIR's canonical JSON rendering contract through the classical
//! ZCIR namespace without introducing a competing implementation.
//!
//! This module is an adapter. It deliberately delegates canonical rendering,
//! deterministic object-key ordering, traversal, output accounting, resource
//! budgets, and structured errors to:
//!
//! `crate::zuir::serialization::canonical`
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
//! - `src/zuir/serialization/mod.rs`
//! - `src/zuir/serialization/canonical.rs`
//! - `src/zuir/serialization/encode.rs`
//! - `src/zuir/serialization/decode.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/classical/zcir/serialization/encode.rs`
//! - `src/classical/zcir/serialization/decode.rs`
//!
//! The grammar owns source syntax. The semantic model owns established
//! program meaning. ZUIR owns the shared computational representation and
//! canonical JSON renderer. ZCIR exposes the classical integration boundary.
//!
//! ## OWNS
//!
//! - The canonical-rendering API under the ZCIR serialization namespace.
//! - Explicit re-exports of the authoritative ZUIR implementation.
//! - ZCIR-specific integration documentation and adapter conformance tests.
//!
//! ## DOES NOT OWN
//!
//! - A second canonical JSON algorithm.
//! - A competing JSON, binary, or document-envelope format.
//! - Schema identifiers, schema versions, or format-version negotiation.
//! - Document encoding or decoding.
//! - IR construction, schema validation, or whole-program verification.
//! - AST parsing, source-language semantic analysis, or type inference.
//! - Cryptographic fingerprints, checksums, signatures, or authentication.
//! - Classical operation semantics, target selection, scheduling, or execution.
//! - Physical resource allocation or hardware-specific layouts.
//!
//! ## DEPENDENCY CONTRACT
//!
//! The sole implementation dependency is:
//!
//! `crate::zuir::serialization::canonical`
//!
//! The canonical ZUIR module owns:
//!
//! - `CanonicalOptions`
//! - `CanonicalReport`
//! - `CanonicalError`
//! - `write_canonical_json`
//! - `canonical_json_bytes`
//! - `canonical_json_string`
//!
//! This adapter must re-export these exact types and functions. It must not
//! wrap their errors, change option defaults, reorder data, or implement
//! separate traversal logic.
//!
//! ## CANONICAL REPRESENTATION CONTRACT
//!
//! The canonicalization behavior is inherited from ZUIR:
//!
//! - Object keys are ordered lexicographically.
//! - Array element order is preserved.
//! - JSON strings use deterministic escaping.
//! - Numbers use the canonical renderer's supported serde_json representation.
//! - Insignificant whitespace is omitted.
//! - Output-byte accounting includes JSON punctuation and escaped content.
//! - Optional depth, node-count, and output-byte budgets are enforced by ZUIR.
//! - Counter overflow and writer failures remain structured errors.
//!
//! This is Zamani's defined canonical JSON representation. It is not a claim
//! of conformance to every external canonical-JSON standard.
//!
//! Canonical rendering defines representation, not computational meaning.
//! It must never be used as justification for reordering operations,
//! operands, effects, control flow, concurrent actions, or other sequences
//! whose order carries semantics.
//!
//! ## DETERMINISM
//!
//! For the same supported `serde_json::Value` and the same options, the
//! delegated renderer defines the byte representation.
//!
//! This adapter adds no independent ordering rules. In particular, it does
//! not sort arrays or normalize semantically meaningful values.
//!
//! Deterministic rendering alone does not guarantee that semantically
//! equivalent IR programs have identical bytes. Such a guarantee would
//! require a separately specified, semantics-preserving IR normalization
//! contract.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This adapter introduces no universal maximum for:
//!
//! - Functions, blocks, operations, values, or operands.
//! - Integer or register widths.
//! - Collection lengths or tensor rank.
//! - CPUs, threads, accelerators, devices, or distributed nodes.
//! - Memory capacity or hardware size.
//!
//! Optional depth, node-count, and output-byte budgets are operational
//! safeguards. They are not language-level or IR-semantic capacity limits.
//!
//! The canonical ZUIR renderer uses an explicit traversal stack, avoiding
//! recursive traversal of nested JSON values. That does not eliminate
//! the need for memory to hold the input value, traversal state, output,
//! object-member ordering information, or the caller's surrounding workload.
//!
//! The byte-producing convenience APIs materialize their complete output.
//! Callers requiring incremental output should use `write_canonical_json`
//! with an appropriate writer and explicit budgets when necessary.
//!
//! No finite implementation can guarantee literal infinity. POCO-REAF means
//! avoiding unjustified universal ceilings and preserving defined program
//! meaning across supported targets, subject to resources, capabilities,
//! operational policies, and physical reality.
//!
//! ## ERROR AND FAILURE CONTRACT
//!
//! All failures use the canonical ZUIR `CanonicalError` taxonomy.
//!
//! The adapter must not:
//!
//! - Replace structured errors with strings.
//! - Treat a budget violation as proof that the program is invalid.
//! - Silently truncate output.
//! - Suppress writer failures.
//! - Claim that partial writer output is a complete valid artifact.
//!
//! When `write_canonical_json` fails, the destination may already contain
//! partial bytes. Callers must discard that output unless their own protocol
//! explicitly provides a safe transactional publication mechanism.
//!
//! ## TRUST AND VALIDATION BOUNDARY
//!
//! Canonical JSON rendering does not prove that a value is a valid ZCIR or
//! ZUIR program.
//!
//! Before a rendered or decoded payload is used for compilation or execution,
//! callers must apply the appropriate schema validation and authoritative IR
//! verification. Target-feasibility checks remain a separate responsibility.
//!
//! Canonical output does not establish authenticity, integrity, authorization,
//! semantic equivalence, or execution safety.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. `src/zuir/serialization/canonical.rs` remains the sole implementation.
//! 2. `src/classical/zcir/serialization/mod.rs` declares `pub mod canonical;`.
//! 3. The ZCIR serialization facade may re-export this module's public API.
//! 4. `encode.rs` continues using the canonical ZUIR document encoder.
//! 5. `decode.rs` continues using the canonical ZUIR document decoder.
//! 6. The encoder and decoder retain ownership of the document envelope.
//! 7. Schema validation remains separate from canonical rendering.
//! 8. Whole-IR verification remains the responsibility of the authoritative
//!    verifier.
//! 9. Content fingerprints, if exposed, use the designated fingerprint
//!    implementation and its specified canonical-byte contract.
//! 10. Any change to canonical byte semantics is reviewed as a serialization
//!     compatibility change in the owning ZUIR implementation.
//!
//! This file must not depend on the ZCIR facade's re-exports to reach ZUIR.
//! Direct owner-module paths avoid unnecessary dependency cycles.
//!
//! ## SECURITY AND PERFORMANCE
//!
//! No unsafe code, filesystem access, network access, global mutable state,
//! or hardware discovery is introduced.
//!
//! This adapter adds no rendering pass or output copy beyond what callers
//! choose through the delegated API. The allocation and traversal behavior
//! is governed by the canonical ZUIR implementation.
//!
//! ## TESTING CONTRACT
//!
//! Tests here verify that the ZCIR entry points remain behaviorally identical
//! to the canonical ZUIR entry points. They do not duplicate the complete
//! canonical-renderer test suite.
//!
//! Required coverage:
//!
//! - Stable object-key ordering.
//! - Preserved array ordering.
//! - Equivalent bytes through the ZCIR and ZUIR APIs.
//! - Consistent structured budget failures.
//! - Writer and in-memory entry-point agreement.
//! - Unicode and escaped-string preservation.
//!
//! Deep nesting, counter overflow, detailed writer failure behavior,
//! and exhaustive budget boundaries belong primarily to the authoritative
//! `src/zuir/serialization/canonical.rs` test suite.
//!
//! ## FREEZE CRITERIA
//!
//! Freeze this file only after:
//!
//! - The ZUIR canonical API exists at the declared path.
//! - The ZCIR serialization module and its parent are valid Rust modules.
//! - These adapter conformance tests compile and pass.
//! - Existing encoder and decoder integration tests pass.
//! - The repository passes formatting, compilation, tests, and lint gates.
//!
//! This adapter's existence alone does not establish that the full ZCIR
//! subsystem or POCO-REAF pipeline is production-ready.

// Re-export the single canonical renderer. Keep these names and signatures
// identical to the owning ZUIR implementation.
pub use crate::zuir::serialization::canonical::{
    canonical_json_bytes,
    canonical_json_string,
    write_canonical_json,
    CanonicalError,
    CanonicalOptions,
    CanonicalReport,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn render(value: &Value) -> String {
        canonical_json_string(value, CanonicalOptions::default())
            .expect("the test value should render")
    }

    #[test]
    fn zcir_and_zuir_entry_points_produce_identical_bytes() {
        let value = json!({
            "operation": "classical.example",
            "operands": [3, 1, 2],
            "metadata": {
                "z": true,
                "a": "stable"
            }
        });

        let zcir = canonical_json_bytes(
            &value,
            CanonicalOptions::default(),
        )
        .expect("ZCIR adapter rendering should succeed");

        let zuir = crate::zuir::serialization::canonical::canonical_json_bytes(
            &value,
            CanonicalOptions::default(),
        )
        .expect("canonical ZUIR rendering should succeed");

        assert_eq!(zcir, zuir);
    }

    #[test]
    fn object_keys_are_rendered_in_canonical_order() {
        let value = json!({
            "z": 3,
            "m": 2,
            "a": 1
        });

        assert_eq!(render(&value), r#"{"a":1,"m":2,"z":3}"#);
    }

    #[test]
    fn array_order_is_preserved() {
        let value = json!([3, 1, 2]);

        assert_eq!(render(&value), "[3,1,2]");
    }

    #[test]
    fn nested_object_keys_are_canonicalized_without_reordering_arrays() {
        let value = json!({
            "outer": {
                "z": [3, 2, 1],
                "a": {
                    "y": true,
                    "b": false
                }
            }
        });

        assert_eq!(
            render(&value),
            r#"{"outer":{"a":{"b":false,"y":true},"z":[3,2,1]}}"#
        );
    }

    #[test]
    fn unicode_and_json_escapes_are_preserved() {
        let value = json!({
            "text": "Zamani — \n \"portable\" \\"
        });

        let encoded = render(&value);
        let decoded: Value =
            serde_json::from_str(&encoded)
                .expect("canonical output should be valid JSON");

        assert_eq!(decoded, value);
    }

    #[test]
    fn byte_and_string_entry_points_agree() {
        let value = json!({
            "classical": true,
            "sequence": [1, 2, 3]
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
    fn writer_entry_point_matches_the_byte_entry_point() {
        let value = json!({
            "name": "classical",
            "values": [5, 8, 13]
        });

        let expected = canonical_json_bytes(
            &value,
            CanonicalOptions::default(),
        )
        .expect("canonical byte rendering should succeed");

        let mut actual = Vec::new();
        let report = write_canonical_json(
            &value,
            &mut actual,
            CanonicalOptions::default(),
        )
        .expect("canonical writer rendering should succeed");

        assert_eq!(actual, expected);
        assert_eq!(report.output_bytes, actual.len());
        assert!(report.nodes_visited > 0);
    }

    #[test]
    fn output_budget_failure_is_preserved_as_a_structured_error() {
        let value = json!({"payload": "larger than two bytes"});

        let result = canonical_json_bytes(
            &value,
            CanonicalOptions::default()
                .with_max_output_bytes(Some(2)),
        );

        assert!(matches!(
            result,
            Err(CanonicalError::OutputLimitExceeded { limit: 2 })
        ));
    }

    #[test]
    fn depth_budget_failure_is_preserved_as_a_structured_error() {
        let value = json!({
            "outer": {
                "inner": 1
            }
        });

        let result = canonical_json_bytes(
            &value,
            CanonicalOptions::default().with_max_depth(Some(1)),
        );

        assert!(matches!(
            result,
            Err(CanonicalError::DepthLimitExceeded {
                limit: 1,
                actual: 2
            })
        ));
    }

    #[test]
    fn node_budget_failure_is_preserved_as_a_structured_error() {
        let value = json!([1, 2, 3]);

        let result = canonical_json_bytes(
            &value,
            CanonicalOptions::default().with_max_nodes(Some(2)),
        );

        assert!(matches!(
            result,
            Err(CanonicalError::NodeLimitExceeded { limit: 2 })
        ));
    }

    #[test]
    fn empty_and_scalar_values_are_supported() {
        assert_eq!(render(&Value::Null), "null");
        assert_eq!(render(&json!(true)), "true");
        assert_eq!(render(&json!(42)), "42");
        assert_eq!(render(&json!("")), r#""""#);
        assert_eq!(render(&json!([])), "[]");
        assert_eq!(render(&json!({})), "{}");
    }
}
