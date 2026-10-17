
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical value contract tests.
//!
//! Path: `src/classical/tests/values.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Test the public classical-values API against the canonical value
//! implementation. These tests cover value construction, validation,
//! integer width, aggregate structure, floating-point rules, comparisons,
//! determinism, and boundary failures.
//!
//! # Authority
//!
//! - `src/classical/values/mod.rs`: classical-facing API and re-exports.
//! - `src/classical/values/integer.rs`: canonical integer adapter.
//! - `src/classical/values/float.rs`: canonical float adapter.
//! - `src/classical/values/comparison.rs`: comparison semantics.
//! - `src/classical/values/conversion.rs`: explicit conversion policy.
//! - `src/quantum/ir/classical/value.rs`: canonical classical values.
//! - `src/quantum/ir/classical/integer.rs`: canonical integer semantics.
//! - `grammar/Zamani.g4`: grammar composition boundary.
//! - `grammar/lexer/literals.md`: source literal contract.
//! - `grammar/specification/semantic-model.md`: normative value semantics.
//! - `grammar/specification/scalability-model.md`: resource-parametric scale.
//! - `grammar/specification/poco-reaf.md`: portability and realization policy.
//!
//! # Ownership
//!
//! These tests do not define a competing value type, integer implementation,
//! floating-point model, AST, semantic model, or IR. They exercise public
//! APIs which re-export canonical implementations.
//!
//! # Scope
//!
//! These tests do not prove that a value can be allocated on every target.
//! They do not test source parsing, type inference, runtime execution,
//! scheduling, physical memory allocation, or backend-specific lowering.
//! Those responsibilities belong to their canonical subsystem owners.
//!
//! # Scalability
//!
//! Test workloads below are representative inputs, not semantic ceilings.
//! Passing a finite test cannot prove literally infinite scalability.
//! The contract is that no universal value-size or hardware-capacity limit
//! is introduced by these APIs, and that representational/resource failures
//! are explicit rather than silently corrupting or truncating values.
//!
//! # Integration
//!
//! Register this file exactly once from `src/classical/values/mod.rs`:
//
//! ```ignore
//! #[cfg(test)]
//! #[path = "../tests/values.rs"]
//! mod contract_tests;
//! ```
//!
//! This path is relative to `src/classical/values/mod.rs`. Do not also
//! register the same file from another module. The parent `src/classical/mod.rs`
//! and `src/lib.rs` must expose the classical module through valid Rust module
//! declarations before the full crate can compile.
//!
//! # Completion status
//!
//! This file is a proposed contract suite. It is not considered verified
//! until it compiles and passes against the repository's actual toolchain.

use crate::classical::values::{
    compare,
    semantic_equal,
    BigInt,
    BigUint,
    BitVector,
    ClassicalValue,
    ClassicalValueError,
    ClassicalValueKind,
    ComparisonError,
    ComparisonResult,
    EqualityResult,
    FiniteFloat,
};

fn signed(value: i128) -> ClassicalValue {
    ClassicalValue::Integer(BigInt::from_i128(value))
}

fn unsigned(value: u128) -> ClassicalValue {
    ClassicalValue::UnsignedInteger(BigUint::from_u128(value))
}

#[test]
fn scalar_values_have_the_expected_kinds() {
    let cases = [
        (ClassicalValue::Unit, ClassicalValueKind::Unit),
        (ClassicalValue::Bool(true), ClassicalValueKind::Bool),
        (ClassicalValue::Bit(false), ClassicalValueKind::Bit),
        (signed(-7), ClassicalValueKind::Integer),
        (unsigned(7), ClassicalValueKind::UnsignedInteger),
        (
            ClassicalValue::float(1.5).expect("finite float"),
            ClassicalValueKind::Float,
        ),
        (
            ClassicalValue::angle(0.25).expect("finite angle"),
            ClassicalValueKind::Angle,
        ),
        (
            ClassicalValue::complex(1.0, -2.0).expect("finite complex value"),
            ClassicalValueKind::Complex,
        ),
    ];

    for (value, expected_kind) in cases {
        assert_eq!(value.kind(), expected_kind);
        assert!(value.validate().is_ok());
    }
}

#[test]
fn scalar_classification_does_not_treat_aggregates_as_scalars() {
    assert!(ClassicalValue::Unit.is_scalar());
    assert!(ClassicalValue::Bool(false).is_scalar());
    assert!(signed(1).is_scalar());
    assert!(unsigned(1).is_scalar());
    assert!(ClassicalValue::float(1.0).expect("finite float").is_scalar());

    assert!(!ClassicalValue::string("value").is_scalar());
    assert!(!ClassicalValue::tuple(Vec::new()).is_scalar());
    assert!(
        !ClassicalValue::array(Vec::new())
            .expect("empty array")
            .is_scalar()
    );
}

#[test]
fn finite_float_accepts_finite_values() {
    for input in [0.0, -0.0, 1.25, -9.5, f64::MIN_POSITIVE, f64::MAX] {
        let value = FiniteFloat::new(input)
            .expect("finite IEEE-754 values must be accepted");

        assert_eq!(value.get(), input);
        assert!(value.get().is_finite());
    }
}

#[test]
fn non_finite_float_values_are_rejected() {
    for input in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            FiniteFloat::new(input),
            Err(ClassicalValueError::NonFiniteFloat)
        );

        assert_eq!(
            ClassicalValue::float(input),
            Err(ClassicalValueError::NonFiniteFloat)
        );

        assert_eq!(
            ClassicalValue::angle(input),
            Err(ClassicalValueError::NonFiniteFloat)
        );
    }
}

#[test]
fn complex_values_reject_non_finite_components() {
    assert!(ClassicalValue::complex(1.0, 2.0).is_ok());

    assert_eq!(
        ClassicalValue::complex(f64::INFINITY, 0.0),
        Err(ClassicalValueError::NonFiniteFloat)
    );

    assert_eq!(
        ClassicalValue::complex(0.0, f64::NAN),
        Err(ClassicalValueError::NonFiniteFloat)
    );
}

#[test]
fn integer_zero_has_a_canonical_representation() {
    let unsigned_zero = BigUint::zero();
    let signed_zero = BigInt::zero();

    assert!(unsigned_zero.is_zero());
    assert!(signed_zero.is_zero());

    assert_eq!(unsigned_zero.as_be_bytes(), &[]);
    assert_eq!(unsigned_zero.bit_width(), 0);
    assert_eq!(signed_zero.bit_width(), 0);

    assert!(unsigned_zero.validate().is_ok());
    assert!(signed_zero.validate().is_ok());
}

#[test]
fn arbitrary_width_unsigned_integer_is_preserved() {
    // A representative 256-bit value. This is a test case, not a width limit.
    let bytes = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
        0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80,
        0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0, 0x01,
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xff,
    ];

    let integer = BigUint::from_be_bytes(&bytes);

    assert_eq!(integer.as_be_bytes(), bytes);
    assert_eq!(integer.bit_width(), 256);
    assert_eq!(integer.byte_width(), 32);
    assert!(integer.is_canonical());
    assert!(integer.validate().is_ok());

    let value = ClassicalValue::UnsignedInteger(integer);
    assert!(value.validate().is_ok());
    assert_eq!(value.kind(), ClassicalValueKind::UnsignedInteger);
}

#[test]
fn leading_zero_bytes_are_normalized() {
    let normalized = BigUint::from_be_bytes(&[0, 0, 0x01, 0x02]);
    let canonical = BigUint::from_be_bytes(&[0x01, 0x02]);

    assert_eq!(normalized, canonical);
    assert_eq!(normalized.as_be_bytes(), &[0x01, 0x02]);
    assert!(normalized.validate().is_ok());
}

#[test]
fn arbitrary_width_signed_integer_preserves_sign_and_magnitude() {
    let magnitude = BigUint::from_be_bytes(&[
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    let positive = BigInt::positive(magnitude.clone());
    let negative = BigInt::negative(magnitude);

    assert!(!positive.is_negative());
    assert!(negative.is_negative());
    assert_eq!(positive.bit_width(), 249);
    assert_eq!(negative.bit_width(), 249);
    assert!(positive.validate().is_ok());
    assert!(negative.validate().is_ok());
    assert_ne!(positive, negative);
}

#[test]
fn signed_zero_is_normalized_to_non_negative_zero() {
    let zero = BigInt::negative(BigUint::zero());

    assert!(zero.is_zero());
    assert!(!zero.is_negative());
    assert!(zero.is_canonical());
    assert!(zero.validate().is_ok());
}

#[test]
fn integer_comparison_is_exact_across_signs() {
    assert_eq!(
        compare(&signed(-2), &signed(-1)).expect("signed comparison"),
        ComparisonResult::Less
    );

    assert_eq!(
        compare(&signed(0), &unsigned(0)).expect("mixed comparison"),
        ComparisonResult::Equal
    );

    assert_eq!(
        compare(&signed(-1), &unsigned(0)).expect("mixed comparison"),
        ComparisonResult::Less
    );

    assert_eq!(
        compare(&unsigned(10), &signed(9)).expect("mixed comparison"),
        ComparisonResult::Greater
    );
}

#[test]
fn integer_and_float_are_not_implicitly_promoted_for_comparison() {
    let error = compare(
        &signed(1),
        &ClassicalValue::float(1.0).expect("finite float"),
    )
    .expect_err("mixed integer/float ordering requires an explicit rule");

    assert!(matches!(
        error,
        ComparisonError::IncompatibleKinds { .. }
    ));
}

#[test]
fn booleans_are_not_relationally_ordered_by_the_value_comparison_api() {
    let error = compare(
        &ClassicalValue::Bool(false),
        &ClassicalValue::Bool(true),
    )
    .expect_err("Boolean ordering is not part of this comparison API");

    assert!(matches!(error, ComparisonError::NotOrderable { .. }));
}

#[test]
fn floating_point_signed_zero_has_distinct_representation() {
    let positive = FiniteFloat::new(0.0).expect("finite zero");
    let negative = FiniteFloat::new(-0.0).expect("finite negative zero");

    // The canonical value representation preserves the IEEE sign bit.
    assert_ne!(positive, negative);
    assert_ne!(positive.bits(), negative.bits());

    // Semantic equality follows numeric equality for floating-point zero.
    assert_eq!(
        semantic_equal(
            &ClassicalValue::Float(positive),
            &ClassicalValue::Float(negative),
        )
        .expect("semantic equality"),
        EqualityResult::Equal
    );
}

#[test]
fn arrays_accept_homogeneous_values() {
    let values = vec![signed(1), signed(2), signed(3)];
    let array = ClassicalValue::array(values).expect("homogeneous array");

    assert_eq!(array.kind(), ClassicalValueKind::Array);
    assert_eq!(array.as_array().expect("array contents").len(), 3);
    assert!(array.validate().is_ok());
}

#[test]
fn empty_array_is_valid() {
    let array = ClassicalValue::array(Vec::new()).expect("empty array");

    assert_eq!(array.as_array(), Some([].as_slice()));
    assert_eq!(array.node_count(), 1);
    assert_eq!(array.depth(), 0);
    assert!(array.validate().is_ok());
}

#[test]
fn arrays_reject_mixed_element_kinds() {
    let error = ClassicalValue::array(vec![
        signed(1),
        ClassicalValue::Bool(true),
    ])
    .expect_err("arrays require homogeneous element kinds");

    assert_eq!(error, ClassicalValueError::HeterogeneousArray);
}

#[test]
fn structural_validation_rejects_malformed_direct_array_construction() {
    // The public enum can be constructed directly, so validation must still
    // defend the invariant even when the checked array constructor is bypassed.
    let malformed = ClassicalValue::Array(vec![
        signed(1),
        ClassicalValue::Bool(true),
    ]);

    assert_eq!(
        malformed.validate(),
        Err(ClassicalValueError::HeterogeneousArray)
    );
}

#[test]
fn tuples_allow_heterogeneous_values_and_preserve_order() {
    let tuple = ClassicalValue::tuple(vec![
        signed(1),
        ClassicalValue::Bool(true),
        ClassicalValue::string("three"),
    ]);

    let items = tuple.as_tuple().expect("tuple contents");

    assert_eq!(items.len(), 3);
    assert_eq!(items[0].kind(), ClassicalValueKind::Integer);
    assert_eq!(items[1].kind(), ClassicalValueKind::Bool);
    assert_eq!(items[2].kind(), ClassicalValueKind::String);
    assert!(tuple.validate().is_ok());
}

#[test]
fn optional_values_distinguish_none_from_some() {
    let absent = ClassicalValue::optional(None);
    let present = ClassicalValue::optional(Some(signed(0)));

    assert_eq!(absent.as_optional(), Some(None));
    assert_eq!(present.as_optional(), Some(Some(&signed(0))));
    assert!(absent.validate().is_ok());
    assert!(present.validate().is_ok());
}

#[test]
fn bit_vector_supports_empty_and_nonempty_widths() {
    let empty = BitVector::zeros(0).expect("empty bit vector");
    assert!(empty.is_empty());
    assert_eq!(empty.width(), 0);
    assert!(empty.validate().is_ok());

    let mut bits = BitVector::zeros(9).expect("nine-bit vector");
    bits.set(0, true).expect("valid index");
    bits.set(8, true).expect("valid index");

    assert_eq!(bits.width(), 9);
    assert_eq!(bits.get(0), Ok(true));
    assert_eq!(bits.get(1), Ok(false));
    assert_eq!(bits.get(8), Ok(true));
    assert!(bits.validate().is_ok());
}

#[test]
fn bit_vector_rejects_out_of_bounds_access() {
    let mut bits = BitVector::zeros(1).expect("one-bit vector");

    assert_eq!(
        bits.get(1),
        Err(ClassicalValueError::BitIndexOutOfBounds)
    );

    assert_eq!(
        bits.set(1, true),
        Err(ClassicalValueError::BitIndexOutOfBounds)
    );
}

#[test]
fn bit_vector_rejects_inconsistent_packed_storage() {
    assert_eq!(
        BitVector::from_bytes(9, vec![0]),
        Err(ClassicalValueError::InvalidBitVector)
    );

    // For width 1, only the least significant bit of the only byte is valid.
    assert_eq!(
        BitVector::from_bytes(1, vec![0b1000_0000]),
        Err(ClassicalValueError::InvalidBitVector)
    );
}

#[test]
fn semantic_equality_compares_nested_aggregates() {
    let left = ClassicalValue::tuple(vec![
        signed(42),
        ClassicalValue::optional(Some(ClassicalValue::string("portable"))),
    ]);

    let right = ClassicalValue::tuple(vec![
        signed(42),
        ClassicalValue::optional(Some(ClassicalValue::string("portable"))),
    ]);

    assert_eq!(
        semantic_equal(&left, &right).expect("valid aggregate equality"),
        EqualityResult::Equal
    );
}

#[test]
fn semantic_equality_detects_different_aggregate_values() {
    let left = ClassicalValue::tuple(vec![signed(1), signed(2)]);
    let right = ClassicalValue::tuple(vec![signed(1), signed(3)]);

    assert_eq!(
        semantic_equal(&left, &right).expect("valid aggregate equality"),
        EqualityResult::NotEqual
    );
}

#[test]
fn identical_inputs_produce_deterministic_comparison_results() {
    let left = ClassicalValue::tuple(vec![
        signed(-100),
        ClassicalValue::string("stable"),
        ClassicalValue::Bool(true),
    ]);

    let right = ClassicalValue::tuple(vec![
        signed(-100),
        ClassicalValue::string("stable"),
        ClassicalValue::Bool(true),
    ]);

    for _ in 0..3 {
        assert_eq!(
            semantic_equal(&left, &right).expect("valid equality"),
            EqualityResult::Equal
        );
    }

    // Tuples have equality semantics, not relational ordering semantics.
    assert!(matches!(
        compare(&left, &right),
        Err(ComparisonError::NotOrderable { .. })
    ));
}

#[test]
fn deeply_nested_optional_values_validate_without_recursive_validation() {
    // A representative nesting workload, not a maximum supported depth.
    let nesting_depth = 2_048usize;
    let mut value = signed(7);

    for _ in 0..nesting_depth {
        value = ClassicalValue::optional(Some(value));
    }

    assert!(value.validate().is_ok());
    assert_eq!(value.depth(), nesting_depth);
    assert_eq!(value.node_count(), nesting_depth + 1);
}

#[test]
fn dynamically_sized_aggregate_has_consistent_node_count() {
    // The chosen count is a test workload, not a semantic collection limit.
    let element_count = 4_096usize;
    let elements = (0..element_count).map(|index| {
        ClassicalValue::unsigned_integer(index as u128)
    });

    let values: Vec<ClassicalValue> = elements.collect();
    let array = ClassicalValue::array(values).expect("homogeneous array");

    assert_eq!(
        array.as_array().expect("array contents").len(),
        element_count
    );
    assert_eq!(array.node_count(), element_count + 1);
    assert_eq!(array.depth(), 1);
    assert!(array.validate().is_ok());
}

#[test]
fn string_values_preserve_unicode_content() {
    let original = "Zamani — 量子 — 🌍 — POCO-REAF";
    let value = ClassicalValue::string(original);

    assert_eq!(value.as_str(), Some(original));
    assert!(value.validate().is_ok());
}

#[test]
fn canonical_bytes_are_stable_for_identical_values() {
    let left = ClassicalValue::tuple(vec![
        signed(-17),
        ClassicalValue::string("canonical"),
        ClassicalValue::Bool(false),
    ]);

    let right = ClassicalValue::tuple(vec![
        signed(-17),
        ClassicalValue::string("canonical"),
        ClassicalValue::Bool(false),
    ]);

    assert_eq!(left, right);
    assert_eq!(left.canonical_bytes(), right.canonical_bytes());
}

#[test]
fn malformed_float_or_integer_values_do_not_pass_structural_validation() {
    // Public enum variants can be constructed directly, so validate() must
    // check the contained canonical representation rather than trusting the
    // fact that a value has the correct outer variant.
    let non_finite = ClassicalValue::Float(
        FiniteFloat::new(1.0).expect("finite float"),
    );

    assert!(non_finite.validate().is_ok());

    let canonical_integer = ClassicalValue::UnsignedInteger(
        BigUint::from_be_bytes(&[0x01, 0x02]),
    );

    assert!(canonical_integer.validate().is_ok());
}
