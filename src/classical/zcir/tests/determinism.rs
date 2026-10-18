
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Determinism and Canonical Serialization Tests
//!
//! Canonical path: `src/classical/zcir/tests/determinism.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety policy: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Verify that the ZCIR serialization façade preserves the deterministic
//! rendering contract owned by canonical ZUIR serialization.
//!
//! ## Architectural authority
//!
//! - `src/classical/zcir/serialization/mod.rs`
//! - `src/classical/zcir/serialization/canonical.rs`
//! - `src/classical/zcir/serialization/encode.rs`
//! - `src/classical/zcir/serialization/decode.rs`
//! - `src/zuir/serialization/mod.rs`
//! - `src/zuir/serialization/canonical.rs`
//! - `src/zuir/serialization/schema.rs`
//! - `src/zuir/serialization/encode.rs`
//! - `src/zuir/serialization/decode.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! ## Ownership
//!
//! This file owns only test fixtures and assertions. It does not implement
//! canonical serialization, IR fingerprints, semantic equivalence, schema
//! versioning, or a competing serialization format.
//!
//! ## Integration contract
//!
//! ZCIR canonical rendering must delegate to the canonical ZUIR renderer.
//! Both APIs must preserve array order, use canonical object-key ordering,
//! and return the owning implementation's structured errors.
//!
//! Register this module from `src/classical/zcir/tests/mod.rs` with:
//!
//!     mod determinism;
//!
//! The ZCIR root must register the test module through `#[cfg(test)] mod tests;`
//! after the root has been converted from its current text placeholder into
//! a valid Rust module.
//!
//! ## Determinism boundary
//!
//! These tests establish deterministic JSON rendering for a given supported
//! `serde_json::Value` and the same options. They do not establish that two
//! semantically equivalent IR programs with different representations have
//! identical bytes.
//!
//! Canonical JSON is not a semantic fingerprint, cryptographic hash,
//! authenticity proof, or validation of executable IR.
//!
//! ## Scalability
//!
//! Workload sizes in this file are finite test fixtures, not production
//! ceilings. Explicit serialization budgets are tested as operational
//! safeguards; they must not be interpreted as language-semantic limits.

use serde_json::{json, Value};
use std::io::{self, Write};

use crate::classical::zcir::serialization::canonical::{
    canonical_json_bytes,
    canonical_json_string,
    write_canonical_json,
    CanonicalError,
    CanonicalOptions,
};

/// Render a JSON value through the ZCIR canonical serialization boundary.
fn render(value: &Value) -> Result<String, CanonicalError> {
    canonical_json_string(value, CanonicalOptions::default())
}

/// A deterministic representative payload containing nested objects,
/// ordered arrays, Unicode, and characters requiring JSON escaping.
fn representative_payload() -> Value {
    json!({
        "domain": "classical",
        "operation": "integer.add",
        "operands": [17, 25],
        "metadata": {
            "zeta": "last",
            "alpha": "first",
            "unicode": "Zamani — λ — 世界 — 🚀",
            "escaped": "quote: \" slash: \\ newline:\n tab:\t"
        },
        "ordered_steps": [
            {"ordinal": 0, "name": "load"},
            {"ordinal": 1, "name": "compute"},
            {"ordinal": 2, "name": "store"}
        ]
    })
}

#[test]
fn repeated_rendering_of_the_same_value_is_byte_identical() {
    let value = representative_payload();

    let first = canonical_json_bytes(&value, CanonicalOptions::default())
        .expect("the representative payload should render");

    let second = canonical_json_bytes(&value, CanonicalOptions::default())
        .expect("rendering the same payload again should succeed");

    assert_eq!(
        first, second,
        "identical values and options must produce identical canonical bytes"
    );
}

#[test]
fn object_keys_are_rendered_in_canonical_order() {
    let value: Value = serde_json::from_str(
        r#"{"zeta":3,"middle":2,"alpha":1}"#,
    )
    .expect("the fixture must be valid JSON");

    let rendered = render(&value).expect("canonical rendering should succeed");

    assert_eq!(
        rendered,
        r#"{"alpha":1,"middle":2,"zeta":3}"#,
        "object keys must follow the canonical lexicographic order"
    );
}

#[test]
fn nested_object_keys_are_canonicalized_without_reordering_arrays() {
    let value: Value = serde_json::from_str(
        r#"{
            "z": {
                "y": 2,
                "a": 1
            },
            "items": [
                {"z": 3, "a": 1},
                {"z": 2, "a": 2},
                {"z": 1, "a": 3}
            ],
            "a": true
        }"#,
    )
    .expect("the fixture must be valid JSON");

    let rendered = render(&value).expect("canonical rendering should succeed");

    assert_eq!(
        rendered,
        r#"{"a":true,"items":[{"a":1,"z":3},{"a":2,"z":2},{"a":3,"z":1}],"z":{"a":1,"y":2}}"#
    );
}

#[test]
fn array_order_is_preserved() {
    let value = json!({
        "operations": [
            "load",
            "add",
            "branch",
            "store"
        ]
    });

    let rendered = render(&value).expect("canonical rendering should succeed");

    assert_eq!(
        rendered,
        r#"{"operations":["load","add","branch","store"]}"#
    );
}

#[test]
fn unicode_and_escaped_strings_are_preserved() {
    let value = json!({
        "unicode": "Zamani — λ — 世界 — 🚀",
        "quoted": "a \"quoted\" value",
        "backslash": "a\\b",
        "line_break": "first\nsecond",
        "tab": "left\tright"
    });

    let bytes = canonical_json_bytes(&value, CanonicalOptions::default())
        .expect("Unicode and escaped strings should render");

    let decoded: Value = serde_json::from_slice(&bytes)
        .expect("canonical output must remain valid JSON");

    assert_eq!(
        decoded, value,
        "canonical rendering must preserve decoded string values"
    );

    let first = canonical_json_bytes(&value, CanonicalOptions::default())
        .expect("first rendering should succeed");

    let second = canonical_json_bytes(&value, CanonicalOptions::default())
        .expect("second rendering should succeed");

    assert_eq!(first, second);
}

#[test]
fn zcir_and_zuir_entry_points_produce_identical_bytes() {
    let value = representative_payload();

    let zcir = canonical_json_bytes(&value, CanonicalOptions::default())
        .expect("ZCIR canonical rendering should succeed");

    let zuir =
        crate::zuir::serialization::canonical::canonical_json_bytes(
            &value,
            crate::zuir::serialization::canonical::CanonicalOptions::default(),
        )
        .expect("canonical ZUIR rendering should succeed");

    assert_eq!(
        zcir, zuir,
        "the ZCIR façade must not introduce a competing canonical renderer"
    );
}

#[test]
fn byte_and_string_entry_points_agree() {
    let value = representative_payload();

    let bytes = canonical_json_bytes(&value, CanonicalOptions::default())
        .expect("byte rendering should succeed");

    let string = canonical_json_string(&value, CanonicalOptions::default())
        .expect("string rendering should succeed");

    assert_eq!(
        bytes,
        string.as_bytes(),
        "byte and string APIs must expose the same canonical representation"
    );
}

#[test]
fn writer_output_matches_the_in_memory_api() {
    let value = representative_payload();
    let expected = canonical_json_bytes(&value, CanonicalOptions::default())
        .expect("in-memory canonical rendering should succeed");

    let mut output = Vec::new();

    let report = write_canonical_json(
        &value,
        &mut output,
        CanonicalOptions::default(),
    )
    .expect("writer-based canonical rendering should succeed");

    assert_eq!(output, expected);
    assert_eq!(report.output_bytes, output.len());
    assert!(
        report.nodes_visited > 0,
        "the report must account for the JSON root"
    );
}

#[test]
fn output_budget_accepts_exactly_sufficient_capacity() {
    let value = json!({
        "name": "Zamani",
        "items": [1, 2, 3]
    });

    let expected =
        canonical_json_bytes(&value, CanonicalOptions::default())
            .expect("unbudgeted rendering should succeed");

    let exact_limit = expected.len();

    let actual = canonical_json_bytes(
        &value,
        CanonicalOptions::default()
            .with_max_output_bytes(Some(exact_limit)),
    )
    .expect("the exact output size should fit its budget");

    assert_eq!(actual, expected);
}

#[test]
fn output_budget_rejects_insufficient_capacity_without_truncating_success() {
    let value = json!({
        "name": "Zamani",
        "items": [1, 2, 3]
    });

    let expected =
        canonical_json_bytes(&value, CanonicalOptions::default())
            .expect("unbudgeted rendering should succeed");

    assert!(
        !expected.is_empty(),
        "this fixture must produce a nonempty document"
    );

    let insufficient_limit = expected.len() - 1;

    let result = canonical_json_bytes(
        &value,
        CanonicalOptions::default()
            .with_max_output_bytes(Some(insufficient_limit)),
    );

    assert!(
        matches!(
            result,
            Err(CanonicalError::OutputLimitExceeded { limit })
                if limit == insufficient_limit
        ),
        "the canonical renderer must return the structured output-budget error"
    );
}

#[test]
fn node_budget_failure_is_deterministic() {
    let value = json!({
        "root": {
            "left": [1, 2, 3],
            "right": [4, 5, 6]
        }
    });

    let options = CanonicalOptions::default().with_max_nodes(Some(2));

    let first = canonical_json_bytes(&value, options);
    let second = canonical_json_bytes(&value, options);

    assert!(
        matches!(
            first,
            Err(CanonicalError::NodeLimitExceeded { limit: 2 })
        ),
        "the fixture should exceed the configured node budget"
    );

    assert!(
        matches!(
            second,
            Err(CanonicalError::NodeLimitExceeded { limit: 2 })
        ),
        "repeated rendering must report the same budget failure"
    );
}

#[test]
fn depth_budget_failure_is_deterministic() {
    let value = json!({
        "level_one": {
            "level_two": {
                "level_three": true
            }
        }
    });

    let options = CanonicalOptions::default().with_max_depth(Some(1));

    let first = canonical_json_bytes(&value, options);
    let second = canonical_json_bytes(&value, options);

    assert!(
        matches!(
            first,
            Err(CanonicalError::DepthLimitExceeded { limit: 1, .. })
        ),
        "the fixture should exceed the configured nesting-depth budget"
    );

    assert!(
        matches!(
            second,
            Err(CanonicalError::DepthLimitExceeded { limit: 1, .. })
        ),
        "repeated rendering must report the same depth-budget failure"
    );
}

/// A writer that deterministically rejects every write.
#[derive(Debug, Default)]
struct RejectingWriter;

impl Write for RejectingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("intentional test writer failure"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn writer_failure_is_reported_instead_of_being_silently_ignored() {
    let value = representative_payload();
    let mut writer = RejectingWriter;

    let result = write_canonical_json(
        &value,
        &mut writer,
        CanonicalOptions::default(),
    );

    assert!(
        matches!(result, Err(CanonicalError::Io(_))),
        "the renderer must preserve writer failures as structured errors"
    );
}
