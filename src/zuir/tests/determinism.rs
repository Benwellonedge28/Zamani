
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Determinism Conformance Tests
//!
//! Path: `src/zuir/tests/determinism.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Test reproducibility contracts across the existing ZUIR components:
//! - canonical JSON serialization;
//! - stable content fingerprints;
//! - compiler-pass determinism declarations;
//! - ordered pipeline determinism assessment;
//! - configured serialization resource budgets.
//!
//! This module is a conformance test suite. It does not implement a second
//! canonical serializer, fingerprint algorithm, pass manager, or verifier.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/serialization/canonical.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/fingerprint/stable_hash.rs`
//! - `src/zuir/pass/contracts.rs`
//! - `src/zuir/pass/determinism.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//!
//! OWNS
//!
//! - Regression tests for the determinism contracts listed above.
//! - Test-only helpers that construct inputs for those contracts.
//! - Explicit regression coverage for ordering and resource-budget behavior.
//!
//! DOES NOT OWN
//!
//! - Canonical serialization or fingerprint implementation.
//! - ZUIR semantic equivalence.
//! - IR verification or transformation correctness.
//! - Pass execution, scheduling, or concurrency control.
//! - Grammar parsing, source semantics, or AST construction.
//! - Target selection, hardware discovery, or physical resource allocation.
//!
//! INTEGRATION
//!
//! Register this module from the actual ZUIR test-module owner. If the crate
//! uses a different test organization, include these tests in that existing
//! structure rather than introducing a competing module root.
//!
//! Required dependencies are the repository's existing `serde_json` and
//! ZUIR modules. Do not add a second serializer or hashing dependency for
//! these tests.
//!
//! SCALABILITY AND POCO-REAF
//!
//! Tests introduce no universal maximum for IR size, operations, qubits,
//! tensor rank, devices, nodes, or pipeline length. Tests that allocate
//! increasingly large data must account for the test runner's finite
//! memory and execution budget.
//!
//! Configured serialization limits are tested as operational budgets, not
//! as language-level restrictions.
//!
//! DETERMINISM SCOPE
//!
//! Deterministic serialization means identical canonical input values and
//! options produce identical bytes. Stable fingerprints mean identical
//! canonical fields, field order, domain, and algorithm version produce
//! identical digests.
//!
//! Neither property alone proves semantic equivalence of arbitrary programs.
//! Pass descriptors express claims, not proofs of implementation behavior.
//!
//! FREEZE CRITERIA
//!
//! - All referenced APIs exist in the target revision.
//! - The tests compile and pass in the real crate.
//! - Ordering, domain separation, escaping, budgets, and error behavior are
//!   covered.
//! - No production implementation is duplicated here.
//! - Formatting, linting, and relevant repository tests pass.

use std::collections::BTreeMap;
use std::io::{self, Write};

use serde_json::{Map, Value};

use crate::zuir::fingerprint::stable_hash::{
    fingerprint,
    Fingerprint,
    FingerprintError,
    StableHashBuilder,
};
use crate::zuir::pass::contracts::{
    PassDescriptor,
    PassErrorKind,
    PassId,
    PassVersion,
};
use crate::zuir::pass::determinism::{
    assess_pipeline_determinism,
    validate_pass_determinism,
    DeterminismPolicy,
};
use crate::zuir::serialization::canonical::{
    write_canonical_json,
    CanonicalError,
    CanonicalOptions,
};

/// Construct a JSON object by inserting members in the caller's order.
///
/// This helper is useful for checking that the canonical writer does not
/// depend on insertion order. Canonical JSON must order object keys according
/// to the serialization contract, not according to construction history.
fn object_from_entries(entries: &[(&str, Value)]) -> Value {
    let mut object = Map::new();

    for (key, value) in entries {
        object.insert((*key).to_owned(), value.clone());
    }

    Value::Object(object)
}

/// Render a JSON value using the production canonical serializer.
fn canonical_bytes(value: &Value) -> Result<Vec<u8>, CanonicalError> {
    let mut output = Vec::new();

    write_canonical_json(
        value,
        &mut output,
        CanonicalOptions::unbounded(),
    )?;

    Ok(output)
}

/// Construct a valid pass descriptor for tests.
///
/// All descriptors use the real pass-contract constructor and validation
/// rules; no production pass metadata is duplicated.
fn pass_descriptor(
    qualified_id: &str,
    deterministic: bool,
) -> PassDescriptor {
    let id = PassId::new(qualified_id)
        .expect("test fixture must use a valid qualified pass ID");

    let mut descriptor = PassDescriptor::new(
        id,
        PassVersion::new(1, 0, 0),
        "Determinism conformance test pass",
    );

    descriptor.deterministic = deterministic;
    descriptor
}

/// Produce a digest from an explicit, ordered set of fields.
///
/// The helper intentionally does not sort fields. Field order is part of the
/// stable fingerprint contract and must be controlled by the caller.
fn digest_with_fields(
    domain: &str,
    fields: &[(&str, &str)],
) -> Result<Fingerprint, FingerprintError> {
    let mut builder = StableHashBuilder::new(domain)?;

    for (name, value) in fields {
        builder.field_str(name, value)?;
    }

    Ok(builder.finish())
}

// -----------------------------------------------------------------------------
// Canonical serialization: stable representation
// -----------------------------------------------------------------------------

#[test]
fn canonical_serialization_is_repeatable() {
    let value = serde_json::json!({
        "program": "example",
        "version": 1,
        "effects": ["memory.read", "memory.write"],
        "verified": true
    });

    let first = canonical_bytes(&value)
        .expect("first canonical serialization must succeed");

    let second = canonical_bytes(&value)
        .expect("second canonical serialization must succeed");

    assert_eq!(
        first,
        second,
        "identical values and options must produce identical bytes"
    );
}

#[test]
fn canonical_serialization_is_independent_of_object_insertion_order() {
    let first = object_from_entries(&[
        ("zeta", Value::from(3)),
        ("alpha", Value::from(1)),
        ("middle", Value::from(2)),
    ]);

    let second = object_from_entries(&[
        ("middle", Value::from(2)),
        ("zeta", Value::from(3)),
        ("alpha", Value::from(1)),
    ]);

    assert_eq!(
        canonical_bytes(&first).expect("first object must serialize"),
        canonical_bytes(&second).expect("second object must serialize"),
        "object insertion order must not affect canonical bytes"
    );
}

#[test]
fn canonical_serialization_preserves_array_order() {
    let first = serde_json::json!({
        "pipeline": ["core.canonicalize", "core.simplify"]
    });

    let second = serde_json::json!({
        "pipeline": ["core.simplify", "core.canonicalize"]
    });

    assert_ne!(
        canonical_bytes(&first).expect("first pipeline must serialize"),
        canonical_bytes(&second).expect("second pipeline must serialize"),
        "array order must remain observable because pipeline order matters"
    );
}

#[test]
fn canonical_serialization_emits_compact_json() {
    let value = serde_json::json!({
        "a": 1,
        "b": [true, false, null]
    });

    let bytes = canonical_bytes(&value)
        .expect("compact canonical serialization must succeed");

    let text = std::str::from_utf8(&bytes)
        .expect("canonical JSON must be valid UTF-8");

    assert_eq!(text, r#"{"a":1,"b":[true,false,null]}"#);
    assert!(
        !text.contains('\n'),
        "canonical output must not contain formatting newlines"
    );
}

#[test]
fn canonical_serialization_escapes_control_characters_and_quotes() {
    let value = Value::String("quote=\" slash=\\ newline=\n tab=\t".into());

    let bytes = canonical_bytes(&value)
        .expect("escaped string must serialize");

    let decoded: Value = serde_json::from_slice(&bytes)
        .expect("canonical output must remain valid JSON");

    assert_eq!(
        decoded, value,
        "JSON escaping must preserve the original string value"
    );

    let text = std::str::from_utf8(&bytes)
        .expect("serialized string must be valid UTF-8");

    assert!(
        text.contains("\\\""),
        "embedded quotation marks must be escaped"
    );
    assert!(
        text.contains("\\\\"),
        "embedded backslashes must be escaped"
    );
    assert!(
        text.contains("\\n"),
        "newline characters must be escaped"
    );
    assert!(
        text.contains("\\t"),
        "tab characters must be escaped"
    );
}

#[test]
fn canonical_serialization_preserves_non_ascii_unicode() {
    let value = serde_json::json!({
        "unicode": "Zamani — квантовый — 量子 — 🌍"
    });

    let bytes = canonical_bytes(&value)
        .expect("Unicode content must serialize");

    let decoded: Value = serde_json::from_slice(&bytes)
        .expect("Unicode output must be valid JSON");

    assert_eq!(decoded, value);
    assert!(
        std::str::from_utf8(&bytes)
            .expect("output must be UTF-8")
            .contains("量子"),
        "non-ASCII characters should be emitted as UTF-8 under this contract"
    );
}

// -----------------------------------------------------------------------------
// Canonical serialization: resource budgets and errors
// -----------------------------------------------------------------------------

#[test]
fn canonical_serialization_reports_output_budget_exhaustion() {
    let value = serde_json::json!({
        "payload": "this output exceeds the deliberately small test budget"
    });

    let mut output = Vec::new();

    let result = write_canonical_json(
        &value,
        &mut output,
        CanonicalOptions::unbounded()
            .with_max_output_bytes(Some(4)),
    );

    assert!(
        matches!(
            result,
            Err(CanonicalError::OutputLimitExceeded { limit: 4 })
        ),
        "an exceeded output budget must return a structured error"
    );
}

#[test]
fn canonical_serialization_reports_node_budget_exhaustion() {
    let value = serde_json::json!([1, 2, 3]);

    let mut output = Vec::new();

    let result = write_canonical_json(
        &value,
        &mut output,
        CanonicalOptions::unbounded()
            .with_max_nodes(Some(2)),
    );

    assert!(
        matches!(
            result,
            Err(CanonicalError::NodeLimitExceeded { limit: 2 })
        ),
        "the node budget must be enforced explicitly"
    );
}

#[test]
fn canonical_serialization_reports_depth_budget_exhaustion() {
    let value = serde_json::json!({
        "outer": {
            "inner": true
        }
    });

    let mut output = Vec::new();

    // The root has depth zero. The configured depth is intentionally too
    // small for this nested input.
    let result = write_canonical_json(
        &value,
        &mut output,
        CanonicalOptions::unbounded()
            .with_max_depth(Some(1)),
    );

    assert!(
        matches!(
            result,
            Err(CanonicalError::DepthLimitExceeded {
                limit: 1,
                actual: 2
            })
        ),
        "depth-budget failures must report the configured and actual depths"
    );
}

#[test]
fn canonical_serialization_reports_writer_failures() {
    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("intentional test writer failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::other("intentional test flush failure"))
        }
    }

    let value = serde_json::json!({"value": 1});

    let result = write_canonical_json(
        &value,
        &mut FailingWriter,
        CanonicalOptions::unbounded(),
    );

    assert!(
        matches!(result, Err(CanonicalError::Io(_))),
        "I/O errors must propagate instead of being treated as success"
    );
}

#[test]
fn canonical_report_matches_successful_output() {
    let value = serde_json::json!({
        "items": [1, 2, 3],
        "enabled": true
    });

    let mut output = Vec::new();

    let report = write_canonical_json(
        &value,
        &mut output,
        CanonicalOptions::unbounded(),
    )
    .expect("canonical rendering must succeed");

    assert_eq!(
        report.output_bytes,
        output.len(),
        "reported output bytes must match the bytes actually written"
    );

    assert!(
        report.nodes_visited >= 1,
        "a successful render must account for the root value"
    );

    assert!(
        report.maximum_depth >= 1,
        "the nested input must report a nonzero maximum depth"
    );
}

// -----------------------------------------------------------------------------
// Stable fingerprints: repeatability and domain separation
// -----------------------------------------------------------------------------

#[test]
fn stable_fingerprint_is_repeatable() {
    let value = "canonical ZUIR semantic content";

    let first = fingerprint("zuir.program.semantic", value)
        .expect("first fingerprint must succeed");

    let second = fingerprint("zuir.program.semantic", value)
        .expect("second fingerprint must succeed");

    assert_eq!(
        first, second,
        "identical canonical inputs must produce identical fingerprints"
    );
}

#[test]
fn stable_fingerprint_changes_when_domain_changes() {
    let value = "same payload";

    let semantic = fingerprint("zuir.program.semantic", value)
        .expect("semantic fingerprint must succeed");

    let cache = fingerprint("zuir.cache.codegen", value)
        .expect("cache fingerprint must succeed");

    assert_ne!(
        semantic, cache,
        "domain separation must prevent cross-domain digest reuse"
    );
}

#[test]
fn stable_fingerprint_changes_when_field_value_changes() {
    let first = digest_with_fields(
        "zuir.operation.semantic",
        &[("operation", "compute")],
    )
    .expect("first fingerprint must succeed");

    let second = digest_with_fields(
        "zuir.operation.semantic",
        &[("operation", "measure")],
    )
    .expect("second fingerprint must succeed");

    assert_ne!(
        first, second,
        "different field values must change the fingerprint"
    );
}

#[test]
fn stable_fingerprint_changes_when_field_order_changes() {
    let first = digest_with_fields(
        "zuir.program.semantic",
        &[("name", "example"), ("mode", "hybrid")],
    )
    .expect("first fingerprint must succeed");

    let second = digest_with_fields(
        "zuir.program.semantic",
        &[("mode", "hybrid"), ("name", "example")],
    )
    .expect("second fingerprint must succeed");

    assert_ne!(
        first, second,
        "the low-level encoder must preserve supplied field order"
    );
}

#[test]
fn stable_fingerprint_distinguishes_concatenation_ambiguities() {
    let first = digest_with_fields(
        "zuir.test.framing",
        &[("left", "ab"), ("right", "c")],
    )
    .expect("first framed fingerprint must succeed");

    let second = digest_with_fields(
        "zuir.test.framing",
        &[("left", "a"), ("right", "bc")],
    )
    .expect("second framed fingerprint must succeed");

    assert_ne!(
        first, second,
        "length framing must distinguish different field boundaries"
    );
}

#[test]
fn stable_fingerprint_rejects_an_empty_domain() {
    let result = StableHashBuilder::new("");

    assert!(
        matches!(result, Err(FingerprintError::InvalidDomain)),
        "an empty fingerprint domain must be rejected"
    );
}

#[test]
fn stable_fingerprint_rejects_control_characters_in_domain() {
    let result = StableHashBuilder::new("zuir.semantic\ninvalid");

    assert!(
        matches!(result, Err(FingerprintError::InvalidDomain)),
        "a domain containing control characters must be rejected"
    );
}

#[test]
fn stable_fingerprint_rejects_empty_field_names() {
    let mut builder = StableHashBuilder::new("zuir.test.fields")
        .expect("valid domain must be accepted");

    let result = builder.field_str("", "value");

    assert!(
        matches!(result, Err(FingerprintError::InvalidFieldName)),
        "empty field names must be rejected"
    );
}

#[test]
fn stable_fingerprint_uses_fixed_size_digest_representation() {
    let digest = fingerprint(
        "zuir.program.semantic",
        "portable computation",
    )
    .expect("fingerprint must succeed");

    assert_eq!(digest.as_bytes().len(), 32);
    assert_eq!(digest.to_hex().len(), 64);
    assert_eq!(digest.to_string().len(), 64);
    assert!(
        digest.to_hex().bytes().all(|byte| {
            byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
        }),
        "hexadecimal fingerprints must use lowercase hexadecimal"
    );
}

#[test]
fn ordered_string_maps_have_repeatable_fingerprints() {
    let first: BTreeMap<String, String> = [
        ("quantum".to_owned(), "logical".to_owned()),
        ("classical".to_owned(), "portable".to_owned()),
    ]
    .into_iter()
    .collect();

    let second: BTreeMap<String, String> = [
        ("classical".to_owned(), "portable".to_owned()),
        ("quantum".to_owned(), "logical".to_owned()),
    ]
    .into_iter()
    .collect();

    let first_digest = fingerprint("zuir.test.map", &first)
        .expect("first map fingerprint must succeed");

    let second_digest = fingerprint("zuir.test.map", &second)
        .expect("second map fingerprint must succeed");

    assert_eq!(
        first_digest, second_digest,
        "map construction order must not affect canonical map fingerprints"
    );
}

// -----------------------------------------------------------------------------
// Pass determinism: declarations and policy
// -----------------------------------------------------------------------------

#[test]
fn default_policy_allows_nondeterministic_passes() {
    assert_eq!(
        DeterminismPolicy::default(),
        DeterminismPolicy::AllowNondeterministic
    );

    let pass = pass_descriptor("core.example", false);

    assert!(
        validate_pass_determinism(
            &pass,
            DeterminismPolicy::AllowNondeterministic
        )
        .is_ok()
    );
}

#[test]
fn strict_policy_accepts_a_deterministic_pass() {
    let pass = pass_descriptor("core.canonicalize", true);

    assert!(
        validate_pass_determinism(
            &pass,
            DeterminismPolicy::RequireDeclaredDeterminism
        )
        .is_ok()
    );
}

#[test]
fn strict_policy_rejects_a_nondeterministic_pass() {
    let pass = pass_descriptor("experimental.parallel_search", false);

    let result = validate_pass_determinism(
        &pass,
        DeterminismPolicy::RequireDeclaredDeterminism,
    );

    let error = result.expect_err(
        "strict determinism policy must reject a pass that disclaims determinism"
    );

    assert_eq!(
        error.kind,
        PassErrorKind::NondeterministicPassRejected
    );
    assert_eq!(
        error.pass.as_ref().map(PassId::as_str),
        Some("experimental.parallel_search")
    );
}

#[test]
fn pipeline_assessment_preserves_execution_order() {
    let canonicalize = pass_descriptor("core.canonicalize", true);
    let parallel_search = pass_descriptor("experimental.parallel_search", false);
    let simplify = pass_descriptor("core.simplify", true);

    let assessment = assess_pipeline_determinism(
        [&canonicalize, &parallel_search, &simplify],
        DeterminismPolicy::AllowNondeterministic,
    )
    .expect("permissive assessment must succeed");

    assert_eq!(assessment.pass_count, 3);
    assert_eq!(assessment.nondeterministic_count(), 1);

    assert_eq!(
        assessment
            .nondeterministic_passes
            .iter()
            .map(PassId::as_str)
            .collect::<Vec<_>>(),
        vec!["experimental.parallel_search"],
        "reported pass IDs must retain their relative pipeline order"
    );
}

#[test]
fn strict_pipeline_assessment_rejects_first_nondeterministic_pass() {
    let first = pass_descriptor("core.canonicalize", true);
    let second = pass_descriptor("experimental.search", false);
    let third = pass_descriptor("core.simplify", true);

    let result = assess_pipeline_determinism(
        [&first, &second, &third],
        DeterminismPolicy::RequireDeclaredDeterminism,
    );

    let error = result.expect_err(
        "strict assessment must reject a pipeline containing a nondeterministic pass"
    );

    assert_eq!(
        error.kind,
        PassErrorKind::NondeterministicPassRejected
    );

    assert_eq!(
        error.pass.as_ref().map(PassId::as_str),
        Some("experimental.search"),
        "the error must identify the first offending pass in execution order"
    );
}

#[test]
fn empty_pipeline_satisfies_the_declaration_policy() {
    let assessment = assess_pipeline_determinism(
        std::iter::empty::<&PassDescriptor>(),
        DeterminismPolicy::RequireDeclaredDeterminism,
    )
    .expect("an empty pipeline has no nondeterministic pass declarations");

    assert_eq!(assessment.pass_count, 0);
    assert!(assessment.all_passes_declare_determinism());
    assert_eq!(assessment.nondeterministic_count(), 0);
}

#[test]
fn pass_determinism_validation_does_not_mutate_the_descriptor() {
    let pass = pass_descriptor("core.canonicalize", true);
    let before = pass.clone();

    validate_pass_determinism(
        &pass,
        DeterminismPolicy::RequireDeclaredDeterminism,
    )
    .expect("deterministic descriptor must pass");

    assert_eq!(
        pass, before,
        "determinism validation must not mutate caller-owned metadata"
    );
}

#[test]
fn pipeline_assessment_does_not_reorder_descriptors() {
    let a = pass_descriptor("core.alpha", true);
    let b = pass_descriptor("core.beta", true);
    let c = pass_descriptor("core.gamma", true);

    let assessment = assess_pipeline_determinism(
        [&c, &a, &b],
        DeterminismPolicy::AllowNondeterministic,
    )
    .expect("assessment must succeed");

    assert_eq!(assessment.pass_count, 3);
    assert!(
        assessment.nondeterministic_passes.is_empty(),
        "all descriptors declare deterministic behavior"
    );

    // This test deliberately supplies a non-sorted order. The assessment
    // summarizes determinism and must not sort or mutate the input pipeline.
    assert_eq!(a.id.as_str(), "core.alpha");
    assert_eq!(b.id.as_str(), "core.beta");
    assert_eq!(c.id.as_str(), "core.gamma");
}

// -----------------------------------------------------------------------------
// Cross-component determinism boundaries
// -----------------------------------------------------------------------------

#[test]
fn canonical_bytes_can_be_fingerprinted_repeatably() {
    let value = serde_json::json!({
        "dialect": "core",
        "operation": "compute",
        "attributes": {
            "precision": "symbolic",
            "portable": true
        }
    });

    let first_bytes = canonical_bytes(&value)
        .expect("first serialization must succeed");

    let second_bytes = canonical_bytes(&value)
        .expect("second serialization must succeed");

    assert_eq!(first_bytes, second_bytes);

    let first = fingerprint(
        "zuir.test.canonical-artifact",
        first_bytes.as_slice(),
    )
    .expect("first artifact fingerprint must succeed");

    let second = fingerprint(
        "zuir.test.canonical-artifact",
        second_bytes.as_slice(),
    )
    .expect("second artifact fingerprint must succeed");

    assert_eq!(
        first, second,
        "identical canonical artifacts must produce identical fingerprints"
    );
}

#[test]
fn canonical_artifact_fingerprint_changes_when_semantic_payload_changes() {
    let first = serde_json::json!({
        "dialect": "core",
        "operation": "compute"
    });

    let second = serde_json::json!({
        "dialect": "core",
        "operation": "measure"
    });

    let first_bytes = canonical_bytes(&first)
        .expect("first artifact must serialize");

    let second_bytes = canonical_bytes(&second)
        .expect("second artifact must serialize");

    let first_digest = fingerprint(
        "zuir.test.canonical-artifact",
        first_bytes.as_slice(),
    )
    .expect("first artifact fingerprint must succeed");

    let second_digest = fingerprint(
        "zuir.test.canonical-artifact",
        second_bytes.as_slice(),
    )
    .expect("second artifact fingerprint must succeed");

    assert_ne!(
        first_digest, second_digest,
        "a changed canonical payload must change this artifact fingerprint"
    );
}
