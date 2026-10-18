
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Interoperability Conformance Tests
//!
//! Path: `src/classical/zcir/tests/interoperability.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Verify the interoperability contracts that currently exist between:
//!
//! - The ZCIR-facing API and canonical ZUIR IR entities.
//! - The ZUIR-to-ZCIR and ZCIR-to-ZUIR identity adapters.
//! - The ZCIR canonical-serialization facade and the canonical ZUIR
//!   JSON renderer.
//!
//! These tests deliberately exercise established public APIs. They do not
//! invent a legacy-IR conversion, a quantum-classical value conversion,
//! or a target-specific lowering API.
//!
//! ## Architectural authority
//!
//! Source syntax and source meaning remain governed by:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! Canonical representation and adapter authority:
//!
//! - `src/zuir/ir/`
//! - `src/zuir/ids.rs`
//! - `src/zuir/serialization/canonical.rs`
//! - `src/classical/zcir/ir/mod.rs`
//! - `src/classical/zcir/lowering/to_zuir.rs`
//! - `src/classical/zcir/lowering/from_zuir.rs`
//! - `src/classical/zcir/serialization/canonical.rs`
//!
//! ## Ownership
//!
//! This file owns integration regression tests only.
//!
//! It does not define IR entities, IDs, serializers, semantic rules,
//! conversion implementations, or hardware resource policies.
//!
//! ## Interoperability contract
//!
//! 1. ZCIR-facing aliases identify the same Rust entity types as their
//!    canonical ZUIR counterparts.
//! 2. Identity conversion preserves the entity's canonical identity and
//!    locally observable structure.
//! 3. Borrowed adapters retain the original entity and do not require
//!    cloning.
//! 4. The ZCIR canonical serializer produces the same canonical bytes as
//!    the ZUIR implementation for the same JSON value and options.
//! 5. Serialization budgets and errors are inherited from the canonical
//!    serializer rather than redefined by ZCIR.
//! 6. An identity conversion does not prove whole-IR validity, semantic
//!    preservation across a distinct representation, or target feasibility.
//! 7. Unsupported or lossy conversion must not be represented as a
//!    successful identity conversion.
//!
//! ## Scalability
//!
//! These tests use small deterministic fixtures so they can run on
//! resource-constrained development machines. The fixture sizes are test
//! workloads, not language or IR capacity limits.
//!
//! Production scalability requires additional stress, fuzz, and integration
//! tests using configurable workloads and explicit operational budgets.
//! No finite test can prove literal infinite capacity.
//!
//! ## Integration prerequisites
//!
//! The parent ZCIR module must register the existing IR, lowering, and
//! serialization modules and register this test module under `#[cfg(test)]`.
//! No nonexistent modules should be declared to make these tests compile.
//!
//! These tests must run alongside the repository's complete test suite.

use serde_json::{json, Value};

use crate::classical::zcir::ir::block::ZcirBlock;
use crate::classical::zcir::lowering::from_zuir::{
    as_zcir,
    as_zcir_mut,
    from_zuir,
};
use crate::classical::zcir::lowering::to_zuir::{
    as_zuir,
    as_zuir_mut,
    to_zuir,
    ZuirCanonical,
};
use crate::classical::zcir::serialization::canonical as zcir_canonical;
use crate::zuir::ids::BlockId;
use crate::zuir::ir::ZuirBlock;
use crate::zuir::serialization::canonical as zuir_canonical;

/// Construct a valid nonzero canonical block identity for a test fixture.
fn block_id(value: u128) -> BlockId {
    BlockId::new(value).expect("test block IDs must be nonzero")
}

/// Construct a canonical block that is valid under the block-local
/// validation contract.
fn valid_block(value: u128) -> ZuirBlock {
    let mut block = ZuirBlock::new(block_id(value));

    block
        .validate()
        .expect("a newly constructed empty block should be valid");

    block
}

/// Compile-time assertion that two type paths name the same type.
///
/// This is useful for ensuring that ZCIR remains an adapter over canonical
/// ZUIR instead of silently introducing a competing block representation.
fn assert_same_type<T>(_: &T, _: &T) {}

#[test]
fn zcir_block_is_the_canonical_zuir_block_type() {
    let zcir_block = ZcirBlock::new(block_id(1));
    let zuir_block = ZuirBlock::new(block_id(2));

    assert_same_type(&zcir_block, &zuir_block);
}

#[test]
fn canonical_block_implements_the_shared_adapter_contract() {
    fn assert_canonical<T: ZuirCanonical>() {}

    assert_canonical::<ZuirBlock>();
    assert_canonical::<ZcirBlock>();
}

#[test]
fn zcir_to_zuir_conversion_preserves_block_identity_and_structure() {
    let original = valid_block(10);
    let original_id = original.id();
    let original_arguments = original.argument_count();
    let original_operations = original.operation_count();

    let converted: ZuirBlock = to_zuir(original);

    assert_eq!(converted.id(), original_id);
    assert_eq!(converted.argument_count(), original_arguments);
    assert_eq!(converted.operation_count(), original_operations);
    assert!(converted.operations().is_empty());

    converted
        .validate()
        .expect("identity conversion must preserve local block validity");
}

#[test]
fn zuir_to_zcir_conversion_preserves_block_identity_and_structure() {
    let original = valid_block(11);
    let original_id = original.id();
    let original_arguments = original.argument_count();
    let original_operations = original.operation_count();

    let converted: ZcirBlock = from_zuir(original);

    assert_eq!(converted.id(), original_id);
    assert_eq!(converted.argument_count(), original_arguments);
    assert_eq!(converted.operation_count(), original_operations);
    assert!(converted.operations().is_empty());

    converted
        .validate()
        .expect("identity conversion must preserve local block validity");
}

#[test]
fn borrowed_adapters_preserve_the_original_entity() {
    let block = valid_block(12);
    let original_id = block.id();

    let through_zcir = as_zcir(&block);
    let through_zuir = as_zuir(&block);

    assert_eq!(through_zcir.id(), original_id);
    assert_eq!(through_zuir.id(), original_id);
    assert_eq!(block.id(), original_id);

    assert_same_type(through_zcir, through_zuir);
}

#[test]
fn mutable_borrowed_adapters_preserve_the_original_identity() {
    let mut block = valid_block(13);
    let original_id = block.id();

    {
        let through_zcir = as_zcir_mut(&mut block);
        assert_eq!(through_zcir.id(), original_id);
    }

    {
        let through_zuir = as_zuir_mut(&mut block);
        assert_eq!(through_zuir.id(), original_id);
    }

    assert_eq!(block.id(), original_id);
    block
        .validate()
        .expect("borrow-only adapter access must not invalidate the block");
}

#[test]
fn repeated_identity_conversions_do_not_change_the_entity() {
    let original = valid_block(14);
    let original_id = original.id();

    let first = to_zuir(original);
    let second = from_zuir(first);
    let third = to_zuir(second);
    let fourth = from_zuir(third);

    assert_eq!(fourth.id(), original_id);
    assert_eq!(fourth.argument_count(), 0);
    assert_eq!(fourth.operation_count(), 0);

    fourth
        .validate()
        .expect("repeated identity conversions must preserve local validity");
}

#[test]
fn zcir_and_zuir_canonical_serializers_emit_identical_bytes() {
    let value = json!({
        "artifact": "interoperability-test",
        "classical": {
            "enabled": true,
            "values": [3, 5, 8, 13]
        },
        "metadata": {
            "alpha": 1,
            "omega": 2
        }
    });

    let zcir_bytes = zcir_canonical::canonical_json_bytes(
        &value,
        zcir_canonical::CanonicalOptions::default(),
    )
    .expect("ZCIR canonical serialization should succeed");

    let zuir_bytes = zuir_canonical::canonical_json_bytes(
        &value,
        zuir_canonical::CanonicalOptions::default(),
    )
    .expect("ZUIR canonical serialization should succeed");

    assert_eq!(zcir_bytes, zuir_bytes);
}

#[test]
fn canonical_serializer_adapters_preserve_nested_data_and_array_order() {
    let value = json!({
        "z": [9, 4, 1],
        "a": {
            "last": "omega",
            "first": "alpha"
        }
    });

    let zcir_text = zcir_canonical::canonical_json_string(
        &value,
        zcir_canonical::CanonicalOptions::default(),
    )
    .expect("ZCIR canonical serialization should succeed");

    let zuir_text = zuir_canonical::canonical_json_string(
        &value,
        zuir_canonical::CanonicalOptions::default(),
    )
    .expect("ZUIR canonical serialization should succeed");

    assert_eq!(zcir_text, zuir_text);

    let decoded: Value = serde_json::from_str(&zcir_text)
        .expect("canonical JSON should remain valid JSON");

    assert_eq!(decoded, value);
    assert!(
        zcir_text.find("\"a\"").expect("key a exists")
            < zcir_text.find("\"z\"").expect("key z exists"),
        "canonical object keys should be ordered"
    );
    assert!(
        zcir_text.find("[9,4,1]").is_some(),
        "canonical rendering must preserve array order"
    );
}

#[test]
fn canonical_serialization_is_repeatable_across_adapter_calls() {
    let value = json!({
        "unicode": "Zamani — POCO-REAF",
        "sequence": [1, 1, 2, 3, 5, 8],
        "nested": {
            "present": true,
            "optional": null
        }
    });

    let first = zcir_canonical::canonical_json_bytes(
        &value,
        zcir_canonical::CanonicalOptions::default(),
    )
    .expect("first serialization should succeed");

    let second = zcir_canonical::canonical_json_bytes(
        &value,
        zcir_canonical::CanonicalOptions::default(),
    )
    .expect("second serialization should succeed");

    assert_eq!(first, second);
}

#[test]
fn zcir_adapter_preserves_canonical_output_budget_errors() {
    let value = json!({
        "payload": "larger than a two-byte output budget"
    });

    let result = zcir_canonical::canonical_json_bytes(
        &value,
        zcir_canonical::CanonicalOptions::default()
            .with_max_output_bytes(Some(2)),
    );

    assert!(matches!(
        result,
        Err(zcir_canonical::CanonicalError::OutputLimitExceeded {
            limit: 2
        })
    ));
}

#[test]
fn zcir_adapter_preserves_canonical_node_budget_errors() {
    let value = json!([1, 2, 3]);

    let result = zcir_canonical::canonical_json_bytes(
        &value,
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
fn zcir_adapter_preserves_canonical_depth_budget_errors() {
    let value = json!({
        "outer": {
            "inner": 1
        }
    });

    let result = zcir_canonical::canonical_json_bytes(
        &value,
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
fn canonical_output_budgets_are_not_silent_data_truncation() {
    let value = json!({
        "sequence": [10, 20, 30, 40],
        "status": "complete"
    });

    let complete = zcir_canonical::canonical_json_bytes(
        &value,
        zcir_canonical::CanonicalOptions::default(),
    )
    .expect("unbudgeted test serialization should succeed");

    let restricted = zcir_canonical::canonical_json_bytes(
        &value,
        zcir_canonical::CanonicalOptions::default()
            .with_max_output_bytes(Some(complete.len().saturating_sub(1))),
    );

    assert!(
        restricted.is_err(),
        "a budget that cannot hold the complete document must report failure"
    );
}

#[test]
fn empty_and_scalar_json_values_interoperate_between_serializers() {
    let values = [
        Value::Null,
        json!(true),
        json!(42),
        json!(""),
        json!([]),
        json!({}),
    ];

    for value in values {
        let zcir_bytes = zcir_canonical::canonical_json_bytes(
            &value,
            zcir_canonical::CanonicalOptions::default(),
        )
        .expect("ZCIR should serialize supported JSON values");

        let zuir_bytes = zuir_canonical::canonical_json_bytes(
            &value,
            zuir_canonical::CanonicalOptions::default(),
        )
        .expect("ZUIR should serialize supported JSON values");

        assert_eq!(
            zcir_bytes, zuir_bytes,
            "ZCIR and ZUIR must share the canonical serialization contract"
        );
    }
}
