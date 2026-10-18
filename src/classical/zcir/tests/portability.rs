
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Portability Contract Tests
//!
//! Path: `src/classical/zcir/tests/portability.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Test the target-independent contracts currently exposed by ZCIR:
//!
//! - ZCIR and ZUIR share canonical IR entity types.
//! - Identity adapters preserve canonical IR identity.
//! - Canonical JSON rendering is consistent across the ZCIR and ZUIR
//!   public entry points.
//! - Structured computational descriptions retain their data through
//!   canonical serialization and parsing.
//! - Configured operational budgets fail explicitly rather than silently
//!   producing a truncated canonical document.
//! - Portability fixtures do not encode assumptions about CPU count,
//!   pointer width, memory capacity, accelerator count, or device identity.
//!
//! These tests do not claim that the repository currently supports compiling
//! and executing every program on every target. Actual cross-target
//! compilation requires target discovery, capability negotiation, lowering,
//! and executable conformance infrastructure.
//!
//! ## Architectural authority
//!
//! Source syntax and semantics:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! Canonical IR and interoperability:
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/serialization/canonical.rs`
//! - `src/classical/zcir/ir/mod.rs`
//! - `src/classical/zcir/lowering/to_zuir.rs`
//! - `src/classical/zcir/lowering/from_zuir.rs`
//! - `src/classical/zcir/serialization/canonical.rs`
//!
//! ## Ownership
//!
//! This file owns tests only. It introduces no new IR, type system, target
//! model, serializer, hardware abstraction, or execution engine.
//!
//! ## Portability contract
//!
//! Portability means preserving specified program meaning across compatible
//! targets. It does not mean that every target has the capabilities or
//! resources required by every program.
//!
//! These tests establish representation-level invariants. They do not
//! establish machine-code equivalence, numerical equivalence for every
//! floating-point environment, distributed consistency, or target feasibility.
//!
//! ## Scalability
//!
//! The fixture sizes below are bounded test workloads, not language or IR
//! limits. Production implementations must support growth subject to
//! available resources, explicit operational budgets, and representational
//! constraints. They must not impose unjustified universal hardware ceilings.
//!
//! All operations use safe Rust.

use serde_json::{json, Value};

use crate::classical::zcir::ir::block::ZcirBlock;
use crate::classical::zcir::lowering::from_zuir::from_zuir;
use crate::classical::zcir::lowering::to_zuir::to_zuir;
use crate::classical::zcir::serialization::canonical as zcir_canonical;
use crate::zuir::ids::BlockId;
use crate::zuir::ir::ZuirBlock;
use crate::zuir::serialization::canonical as zuir_canonical;

/// Number of representative data elements in the portability fixture.
///
/// This is a test workload, not a limit on program data.
const PORTABILITY_FIXTURE_ITEMS: usize = 128;

/// Build a canonical nonzero block ID for a test.
fn block_id(value: u128) -> BlockId {
    BlockId::new(value).expect("test block IDs must be nonzero")
}

/// Build a locally valid, empty canonical block.
fn empty_block(value: u128) -> ZuirBlock {
    let mut block = ZuirBlock::new(block_id(value));
    block
        .validate()
        .expect("a newly constructed empty block should be valid");
    block
}

/// Create a portable data fixture without embedding machine or target
/// properties in the payload.
fn portable_fixture() -> Value {
    let sequence: Vec<Value> = (0..PORTABILITY_FIXTURE_ITEMS)
        .map(|index| {
            json!({
                "index": index,
                "enabled": index % 2 == 0,
                "label": format!("item-{index}")
            })
        })
        .collect();

    json!({
        "format": "zamani-portability-fixture",
        "schema_version": 1,
        "program": {
            "intent": "target-independent-data-preservation",
            "requires": [],
            "preferences": [],
            "constraints": []
        },
        "sequence": sequence
    })
}

/// Serialize using the canonical ZCIR entry point.
fn zcir_bytes(value: &Value) -> Result<Vec<u8>, zcir_canonical::CanonicalError> {
    zcir_canonical::canonical_json_bytes(
        value,
        zcir_canonical::CanonicalOptions::default(),
    )
}

/// Serialize using the canonical ZUIR entry point.
fn zuir_bytes(value: &Value) -> Result<Vec<u8>, zuir_canonical::CanonicalError> {
    zuir_canonical::canonical_json_bytes(
        value,
        zuir_canonical::CanonicalOptions::default(),
    )
}

#[test]
fn zcir_and_zuir_share_the_same_canonical_block_type() {
    let zcir_block = ZcirBlock::new(block_id(1));
    let zuir_block = ZuirBlock::new(block_id(2));

    // This is a compile-time type compatibility assertion.
    fn assert_same_type<T>(_: &T, _: &T) {}
    assert_same_type(&zcir_block, &zuir_block);
}

#[test]
fn block_identity_survives_zcir_to_zuir_to_zcir_adaptation() {
    let original = empty_block(10);
    let original_id = original.id();

    let in_zuir: ZuirBlock = to_zuir(original);
    assert_eq!(in_zuir.id(), original_id);

    let back_in_zcir: ZcirBlock = from_zuir(in_zuir);
    assert_eq!(back_in_zcir.id(), original_id);
    assert_eq!(back_in_zcir.argument_count(), 0);
    assert_eq!(back_in_zcir.operation_count(), 0);

    back_in_zcir
        .validate()
        .expect("identity adapters must preserve block-local validity");
}

#[test]
fn canonical_bytes_do_not_depend_on_the_zcir_or_zuir_entry_point() {
    let fixture = portable_fixture();

    let zcir = zcir_bytes(&fixture).expect("ZCIR serialization should succeed");
    let zuir = zuir_bytes(&fixture).expect("ZUIR serialization should succeed");

    assert_eq!(zcir, zuir);
}

#[test]
fn canonical_bytes_are_repeatable_for_the_same_input() {
    let fixture = portable_fixture();

    let first = zcir_bytes(&fixture).expect("first serialization should succeed");
    let second = zcir_bytes(&fixture).expect("second serialization should succeed");

    assert_eq!(first, second);
}

#[test]
fn canonical_output_preserves_sequence_order() {
    let fixture = json!({
        "sequence": [
            {"ordinal": 3},
            {"ordinal": 1},
            {"ordinal": 2}
        ]
    });

    let bytes = zcir_bytes(&fixture).expect("serialization should succeed");
    let decoded: Value =
        serde_json::from_slice(&bytes).expect("output should be valid JSON");

    assert_eq!(decoded, fixture);
    assert_eq!(
        decoded["sequence"][0]["ordinal"],
        json!(3)
    );
    assert_eq!(
        decoded["sequence"][1]["ordinal"],
        json!(1)
    );
    assert_eq!(
        decoded["sequence"][2]["ordinal"],
        json!(2)
    );
}

#[test]
fn canonical_output_orders_object_keys_without_reordering_arrays() {
    let fixture = json!({
        "zeta": [3, 2, 1],
        "alpha": {
            "omega": true,
            "beta": false
        }
    });

    let rendered =
        zcir_canonical::canonical_json_string(
            &fixture,
            zcir_canonical::CanonicalOptions::default(),
        )
        .expect("canonical string serialization should succeed");

    let alpha_position = rendered.find("\"alpha\"").expect("alpha key exists");
    let zeta_position = rendered.find("\"zeta\"").expect("zeta key exists");

    assert!(alpha_position < zeta_position);
    assert!(
        rendered.find("[3,2,1]").is_some(),
        "canonical serialization must preserve array element order"
    );
}

#[test]
fn canonical_serialization_preserves_unicode_and_escaped_text() {
    let fixture = json!({
        "language": "Zamani",
        "description": "portable — classical — computation",
        "escaped": "quote: \" slash: \\ newline:\n",
        "empty": ""
    });

    let bytes = zcir_bytes(&fixture).expect("serialization should succeed");
    let decoded: Value =
        serde_json::from_slice(&bytes).expect("serialized data should parse");

    assert_eq!(decoded, fixture);
}

#[test]
fn canonical_serialization_handles_empty_and_scalar_values() {
    let fixtures = [
        Value::Null,
        json!(true),
        json!(false),
        json!(0),
        json!(""),
        json!([]),
        json!({}),
    ];

    for fixture in fixtures {
        let zcir = zcir_bytes(&fixture)
            .expect("ZCIR should serialize the JSON fixture");
        let zuir = zuir_bytes(&fixture)
            .expect("ZUIR should serialize the JSON fixture");

        assert_eq!(zcir, zuir);

        let decoded: Value =
            serde_json::from_slice(&zcir).expect("output should parse");
        assert_eq!(decoded, fixture);
    }
}

#[test]
fn output_budget_exhaustion_is_reported_without_silent_truncation() {
    let fixture = portable_fixture();
    let complete = zcir_bytes(&fixture).expect("unrestricted fixture should serialize");

    assert!(!complete.is_empty());

    let budget = complete.len().saturating_sub(1);
    let restricted = zcir_canonical::canonical_json_bytes(
        &fixture,
        zcir_canonical::CanonicalOptions::default()
            .with_max_output_bytes(Some(budget)),
    );

    assert!(
        matches!(
            restricted,
            Err(zcir_canonical::CanonicalError::OutputLimitExceeded {
                limit
            }) if limit == budget
        ),
        "an insufficient output budget must return a structured error"
    );
}

#[test]
fn node_budget_exhaustion_is_reported_explicitly() {
    let fixture = json!([1, 2, 3]);

    let result = zcir_canonical::canonical_json_bytes(
        &fixture,
        zcir_canonical::CanonicalOptions::default().with_max_nodes(Some(2)),
    );

    assert!(matches!(
        result,
        Err(zcir_canonical::CanonicalError::NodeLimitExceeded {
            limit: 2
        })
    ));
}

#[test]
fn depth_budget_exhaustion_is_reported_explicitly() {
    let fixture = json!({
        "outer": {
            "inner": 1
        }
    });

    let result = zcir_canonical::canonical_json_bytes(
        &fixture,
        zcir_canonical::CanonicalOptions::default().with_max_depth(Some(1)),
    );

    assert!(matches!(
        result,
        Err(zcir_canonical::CanonicalError::DepthLimitExceeded {
            limit: 1,
            actual: 2
        })
    ));
}

#[test]
fn fixture_size_is_a_test_workload_not_a_semantic_limit() {
    let fixture = portable_fixture();

    assert_eq!(
        fixture["sequence"]
            .as_array()
            .expect("fixture sequence should be an array")
            .len(),
        PORTABILITY_FIXTURE_ITEMS
    );

    // The fixture generator has no target-specific configuration and the
    // constant above controls this test only. It does not configure ZCIR,
    // ZUIR, the language, a backend, or a runtime.
    let rendered = zcir_bytes(&fixture).expect("fixture should serialize");
    let decoded: Value =
        serde_json::from_slice(&rendered).expect("fixture should round-trip");

    assert_eq!(decoded, fixture);
}
