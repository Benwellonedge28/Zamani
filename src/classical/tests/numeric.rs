
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Numerical Contract Tests
//!
//! File: src/classical/tests/numeric.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! STATUS
//!   Repository-aligned conformance test implementation.
//!   Full compilation and execution against the repository are pending.
//!
//! PURPOSE
//!   Test checked integer arithmetic, floating-point arithmetic, elementary
//!   functions, statistical operations, numerical reproducibility, canonical
//!   numeric representation boundaries, and scalability-related invariants.
//!
//! AUTHORITY
//!   Canonical unsigned and signed integers:
//!     crate::quantum::ir::classical::integer
//!
//!   Canonical floating-point values:
//!     crate::quantum::ir::classical::float
//!
//!   Numerical operations:
//!     crate::classical::numeric::arithmetic
//!     crate::classical::numeric::elementary
//!     crate::classical::numeric::statistics
//!     crate::classical::numeric::reproducibility
//!
//!   Normative specification and grammar:
//!     grammar/spec/classical.md
//!     grammar/classical/numeric.g4
//!     grammar/classical/numerical.g4
//!     grammar/expressions/arithmetic.g4
//!     grammar/expressions/expressions.g4
//!
//! OWNERSHIP BOUNDARY
//!   These tests exercise public contracts. They do not define another numeric
//!   representation, type system, AST, symbolic expression system, or IR.
//!
//!   Numeric representation belongs to the canonical classical IR.
//!   Type inference, conversions, overload resolution, and source diagnostics
//!   belong to semantic analysis.
//!   Concrete numerical operations belong to src/classical/numeric/.
//!   Optimization and target-specific realization belong to downstream layers.
//!
//! SCALABILITY
//!   Test input sizes are examples, not universal language limits.
//!   No test can prove literal infinite scalability. The objective is to
//!   detect accidental fixed-width assumptions, unchecked overflow, silent
//!   truncation, data loss, and nondeterministic behavior.
//!
//! INTEGRATION
//!   Register this file once through the existing classical test module.
//!   Do not create duplicate numeric types or duplicate numeric modules.
//!   If a test exposes a semantic discrepancy, fix the owning implementation
//!   or normative contract rather than weakening the assertion.
//!
//! COMPLETION
//!   Run cargo fmt, cargo check, cargo test, and cargo clippy against the
//!   actual repository. This file has not been executed in the user's checkout.

use crate::classical::numeric::arithmetic::{
    apply_float,
    apply_signed,
    apply_unsigned,
    unsigned_power,
    ArithmeticError,
    ArithmeticOperator,
};

use crate::classical::numeric::elementary::{
    evaluate_binary,
    evaluate_unary,
    BinaryElementaryFunction,
    ElementaryError,
    ElementaryFunction,
};

use crate::classical::numeric::reproducibility::{
    canonical_dot_product,
    canonical_sum,
    dot_product_with_policy,
    ordered_dot_product,
    ordered_sum,
    sum_with_policy,
    ContractionPolicy,
    ReductionOrder,
    ReproducibilityLevel,
    ReproducibilityPolicy,
};

use crate::classical::numeric::statistics::{
    mean,
    median,
    min_max,
    pearson_correlation,
    population_covariance,
    population_standard_deviation,
    population_variance,
    quantile,
    sample_covariance,
    sample_standard_deviation,
    sample_variance,
    sum,
    summarize,
    StatisticsError,
};

use crate::quantum::ir::classical::float::ClassicalFloat;
use crate::quantum::ir::classical::integer::{
    SignedInteger,
    UnsignedInteger,
};

// -------------------------------------------------------------------------
// Shared test helpers
// -------------------------------------------------------------------------

fn float(value: f64) -> ClassicalFloat {
    ClassicalFloat::new(value)
        .expect("test input must satisfy the canonical float contract")
}

fn unsigned(value: u128) -> UnsignedInteger {
    UnsignedInteger::from_u128(value)
}

fn signed(value: i128) -> SignedInteger {
    SignedInteger::from_i128(value)
}

fn floats(values: &[f64]) -> Vec<ClassicalFloat> {
    values.iter().copied().map(float).collect()
}

fn assert_float_eq(actual: ClassicalFloat, expected: f64) {
    assert_eq!(actual.get(), expected);
}

fn assert_float_close(
    actual: ClassicalFloat,
    expected: f64,
    tolerance: f64,
) {
    let difference = (actual.get() - expected).abs();

    assert!(
        difference <= tolerance,
        "expected {}, got {}, absolute error {}, tolerance {}",
        expected,
        actual.get(),
        difference,
        tolerance
    );
}

// -------------------------------------------------------------------------
// Canonical numeric representations
// -------------------------------------------------------------------------

#[test]
fn canonical_unsigned_integer_round_trips_supported_primitive_values() {
    for value in [0_u128, 1, 2, 255, 256, u64::MAX, u128::MAX] {
        let integer = unsigned(value);

        assert_eq!(
            integer.to_u128().expect("value should fit in u128"),
            value
        );

        integer
            .validate()
            .expect("constructed integer must be valid");
    }
}

#[test]
fn canonical_signed_integer_round_trips_supported_primitive_values() {
    for value in [
        i128::MIN,
        i64::MIN as i128,
        -1,
        0,
        1,
        i64::MAX as i128,
        i128::MAX,
    ] {
        let integer = signed(value);

        assert_eq!(
            integer.to_i128().expect("value should fit in i128"),
            value
        );

        integer
            .validate()
            .expect("constructed integer must be valid");
    }
}

#[test]
fn canonical_integer_width_is_not_assumed_to_be_a_machine_word() {
    let integer = UnsignedInteger::from_be_bytes(&[
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
        0x01,
    ]);

    integer
        .validate()
        .expect("canonical arbitrary-width integer must validate");

    assert!(
        integer.bit_width() > u64::BITS as u64,
        "test must exercise an integer wider than u64"
    );

    assert_eq!(
        UnsignedInteger::from_be_bytes(integer.as_be_bytes()),
        integer
    );
}

#[test]
fn canonical_float_rejects_non_finite_inputs() {
    assert!(ClassicalFloat::new(f64::NAN).is_err());
    assert!(ClassicalFloat::new(f64::INFINITY).is_err());
    assert!(ClassicalFloat::new(f64::NEG_INFINITY).is_err());
}

#[test]
fn canonical_float_preserves_signed_zero_when_supported_by_its_contract() {
    let positive_zero = float(0.0);
    let negative_zero = float(-0.0);

    assert!(positive_zero.is_zero());
    assert!(negative_zero.is_zero());

    assert!(positive_zero.is_positive_zero());
    assert!(negative_zero.is_negative_zero());

    assert_ne!(positive_zero.bits(), negative_zero.bits());
}

#[test]
fn canonical_float_round_trips_finite_values() {
    for value in [
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        -0.5,
        f64::MIN_POSITIVE,
        f64::MAX,
        f64::MIN,
    ] {
        let canonical = float(value);

        assert_eq!(canonical.get().to_bits(), value.to_bits());
        canonical
            .validate()
            .expect("constructed canonical float must validate");
    }
}

// -------------------------------------------------------------------------
// Unsigned integer arithmetic
// -------------------------------------------------------------------------

#[test]
fn unsigned_addition_and_multiplication_are_exact() {
    let lhs = unsigned(12);
    let rhs = unsigned(7);

    let sum = apply_unsigned(ArithmeticOperator::Add, &lhs, &rhs)
        .expect("addition should succeed");

    let product = apply_unsigned(ArithmeticOperator::Multiply, &lhs, &rhs)
        .expect("multiplication should succeed");

    assert_eq!(sum.to_u128().unwrap(), 19);
    assert_eq!(product.to_u128().unwrap(), 84);
}

#[test]
fn unsigned_subtraction_rejects_negative_results() {
    let result =
        apply_unsigned(ArithmeticOperator::Subtract, &unsigned(2), &unsigned(3));

    assert!(matches!(
        result,
        Err(ArithmeticError::NegativeUnsignedResult)
    ));
}

#[test]
fn unsigned_division_and_remainder_are_consistent() {
    let lhs = unsigned(29);
    let rhs = unsigned(6);

    let quotient =
        apply_unsigned(ArithmeticOperator::Divide, &lhs, &rhs)
            .expect("nonzero divisor");

    let remainder =
        apply_unsigned(ArithmeticOperator::Remainder, &lhs, &rhs)
            .expect("nonzero divisor");

    assert_eq!(quotient.to_u128().unwrap(), 4);
    assert_eq!(remainder.to_u128().unwrap(), 5);

    let reconstructed = apply_unsigned(
        ArithmeticOperator::Add,
        &apply_unsigned(ArithmeticOperator::Multiply, &quotient, &rhs)
            .unwrap(),
        &remainder,
    )
    .unwrap();

    assert_eq!(reconstructed, lhs);
}

#[test]
fn unsigned_division_by_zero_fails_without_panicking() {
    assert!(
        apply_unsigned(
            ArithmeticOperator::Divide,
            &unsigned(10),
            &unsigned(0),
        )
        .is_err()
    );

    assert!(
        apply_unsigned(
            ArithmeticOperator::Remainder,
            &unsigned(10),
            &unsigned(0),
        )
        .is_err()
    );
}

#[test]
fn unsigned_power_handles_zero_and_one_exponents() {
    assert_eq!(
        unsigned_power(&unsigned(9), &unsigned(0))
            .unwrap()
            .to_u128()
            .unwrap(),
        1
    );

    assert_eq!(
        unsigned_power(&unsigned(9), &unsigned(1))
            .unwrap()
            .to_u128()
            .unwrap(),
        9
    );

    assert_eq!(
        unsigned_power(&unsigned(9), &unsigned(3))
            .unwrap()
            .to_u128()
            .unwrap(),
        729
    );

    // The concrete arithmetic contract explicitly defines 0^0 as 1.
    assert_eq!(
        unsigned_power(&unsigned(0), &unsigned(0))
            .unwrap()
            .to_u128()
            .unwrap(),
        1
    );
}

#[test]
fn unsigned_arithmetic_handles_values_beyond_u64() {
    let lhs = UnsignedInteger::from_be_bytes(&[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let result =
        apply_unsigned(ArithmeticOperator::Add, &lhs, &unsigned(1))
            .expect("arbitrary-width addition should succeed");

    assert!(result.bit_width() > u64::BITS as u64);
    assert_ne!(result, lhs);
}

// -------------------------------------------------------------------------
// Signed integer arithmetic
// -------------------------------------------------------------------------

#[test]
fn signed_arithmetic_preserves_negative_results() {
    let subtraction =
        apply_signed(ArithmeticOperator::Subtract, &signed(2), &signed(7))
            .unwrap();

    let multiplication =
        apply_signed(ArithmeticOperator::Multiply, &signed(-6), &signed(7))
            .unwrap();

    assert_eq!(subtraction.to_i128().unwrap(), -5);
    assert_eq!(multiplication.to_i128().unwrap(), -42);
}

#[test]
fn signed_division_truncates_toward_zero() {
    let quotient =
        apply_signed(ArithmeticOperator::Divide, &signed(-17), &signed(5))
            .unwrap();

    let remainder =
        apply_signed(ArithmeticOperator::Remainder, &signed(-17), &signed(5))
            .unwrap();

    assert_eq!(quotient.to_i128().unwrap(), -3);
    assert_eq!(remainder.to_i128().unwrap(), -2);
}

#[test]
fn signed_division_by_zero_fails() {
    assert!(
        apply_signed(
            ArithmeticOperator::Divide,
            &signed(10),
            &signed(0),
        )
        .is_err()
    );

    assert!(
        apply_signed(
            ArithmeticOperator::Remainder,
            &signed(10),
            &signed(0),
        )
        .is_err()
    );
}

#[test]
fn signed_arithmetic_preserves_zero_identity() {
    let zero = signed(0);
    let value = signed(-123);

    assert_eq!(
        apply_signed(ArithmeticOperator::Add, &value, &zero).unwrap(),
        value
    );

    assert_eq!(
        apply_signed(ArithmeticOperator::Subtract, &value, &zero).unwrap(),
        value
    );

    assert_eq!(
        apply_signed(ArithmeticOperator::Multiply, &value, &signed(1))
            .unwrap(),
        value
    );
}

// -------------------------------------------------------------------------
// Floating-point arithmetic
// -------------------------------------------------------------------------

#[test]
fn floating_point_addition_subtraction_and_multiplication_work() {
    let lhs = float(12.5);
    let rhs = float(2.5);

    assert_float_eq(
        apply_float(ArithmeticOperator::Add, lhs, rhs).unwrap(),
        15.0,
    );

    assert_float_eq(
        apply_float(ArithmeticOperator::Subtract, lhs, rhs).unwrap(),
        10.0,
    );

    assert_float_eq(
        apply_float(ArithmeticOperator::Multiply, lhs, rhs).unwrap(),
        31.25,
    );
}

#[test]
fn floating_point_division_and_remainder_work() {
    let lhs = float(17.0);
    let rhs = float(5.0);

    assert_float_eq(
        apply_float(ArithmeticOperator::Divide, lhs, rhs).unwrap(),
        3.4,
    );

    assert_float_eq(
        apply_float(ArithmeticOperator::Remainder, lhs, rhs).unwrap(),
        2.0,
    );
}

#[test]
fn floating_point_division_by_zero_is_reported() {
    assert!(
        apply_float(ArithmeticOperator::Divide, float(1.0), float(0.0))
            .is_err()
    );

    assert!(
        apply_float(
            ArithmeticOperator::Remainder,
            float(1.0),
            float(0.0),
        )
        .is_err()
    );
}

#[test]
fn floating_point_overflow_is_not_silently_accepted() {
    let result = apply_float(
        ArithmeticOperator::Multiply,
        float(f64::MAX),
        float(2.0),
    );

    assert!(
        result.is_err(),
        "the checked arithmetic API must reject an unrepresentable result"
    );
}

#[test]
fn floating_point_power_uses_the_declared_arithmetic_api() {
    let result =
        apply_float(ArithmeticOperator::Power, float(2.0), float(10.0))
            .unwrap();

    assert_float_eq(result, 1024.0);
}

// -------------------------------------------------------------------------
// Elementary functions
// -------------------------------------------------------------------------

#[test]
fn elementary_square_root_and_exponential_are_correct() {
    assert_float_eq(
        evaluate_unary(ElementaryFunction::Sqrt, float(81.0)).unwrap(),
        9.0,
    );

    assert_float_close(
        evaluate_unary(ElementaryFunction::Exp, float(1.0)).unwrap(),
        core::f64::consts::E,
        1e-14,
    );
}

#[test]
fn elementary_logarithms_obey_mathematical_domains() {
    assert_float_eq(
        evaluate_unary(ElementaryFunction::Ln, float(1.0)).unwrap(),
        0.0,
    );

    assert!(
        evaluate_unary(ElementaryFunction::Ln, float(0.0)).is_err()
    );

    assert!(
        evaluate_unary(ElementaryFunction::Sqrt, float(-1.0)).is_err()
    );
}

#[test]
fn elementary_trigonometric_functions_use_radians() {
    assert_float_close(
        evaluate_unary(
            ElementaryFunction::Sin,
            float(core::f64::consts::FRAC_PI_2),
        )
        .unwrap(),
        1.0,
        1e-14,
    );

    assert_float_close(
        evaluate_unary(ElementaryFunction::Cos, float(0.0)).unwrap(),
        1.0,
        1e-14,
    );
}

#[test]
fn binary_elementary_minimum_and_maximum_work() {
    assert_float_eq(
        evaluate_binary(
            BinaryElementaryFunction::Min,
            float(-3.0),
            float(7.0),
        )
        .unwrap(),
        -3.0,
    );

    assert_float_eq(
        evaluate_binary(
            BinaryElementaryFunction::Max,
            float(-3.0),
            float(7.0),
        )
        .unwrap(),
        7.0,
    );
}

#[test]
fn binary_elementary_power_and_hypotenuse_work() {
    assert_float_eq(
        evaluate_binary(
            BinaryElementaryFunction::Pow,
            float(3.0),
            float(4.0),
        )
        .unwrap(),
        81.0,
    );

    assert_float_eq(
        evaluate_binary(
            BinaryElementaryFunction::Hypot,
            float(3.0),
            float(4.0),
        )
        .unwrap(),
        5.0,
    );
}

// -------------------------------------------------------------------------
// Descriptive statistics
// -------------------------------------------------------------------------

#[test]
fn statistics_sum_mean_and_min_max_are_correct() {
    let values = floats(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    assert_float_eq(sum(&values).unwrap(), 15.0);
    assert_float_eq(mean(&values).unwrap(), 3.0);

    let (minimum, maximum) = min_max(&values).unwrap();

    assert_float_eq(minimum, 1.0);
    assert_float_eq(maximum, 5.0);
}

#[test]
fn statistics_population_and_sample_variance_are_distinct() {
    let values = floats(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    assert_float_eq(population_variance(&values).unwrap(), 2.0);
    assert_float_eq(sample_variance(&values).unwrap(), 2.5);

    assert_float_eq(
        population_standard_deviation(&values).unwrap(),
        2.0_f64.sqrt(),
    );

    assert_float_eq(
        sample_standard_deviation(&values).unwrap(),
        2.5_f64.sqrt(),
    );
}

#[test]
fn statistics_quantiles_and_median_are_correct() {
    let values = floats(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    assert_float_eq(quantile(&values, 0.0).unwrap(), 1.0);
    assert_float_eq(quantile(&values, 0.5).unwrap(), 3.0);
    assert_float_eq(quantile(&values, 1.0).unwrap(), 5.0);
    assert_float_eq(median(&values).unwrap(), 3.0);
}

#[test]
fn statistics_reject_invalid_quantile_probabilities() {
    let values = floats(&[1.0, 2.0, 3.0]);

    assert!(matches!(
        quantile(&values, -0.1),
        Err(StatisticsError::InvalidQuantile)
    ));

    assert!(matches!(
        quantile(&values, 1.1),
        Err(StatisticsError::InvalidQuantile)
    ));

    assert!(matches!(
        quantile(&values, f64::NAN),
        Err(StatisticsError::InvalidQuantile)
    ));
}

#[test]
fn statistics_handle_empty_inputs_explicitly() {
    let empty: Vec<ClassicalFloat> = Vec::new();

    assert!(sum(&empty).is_err());
    assert!(mean(&empty).is_err());
    assert!(min_max(&empty).is_err());
    assert!(median(&empty).is_err());
    assert!(population_variance(&empty).is_err());
}

#[test]
fn sample_statistics_reject_insufficient_observations() {
    let singleton = floats(&[42.0]);

    assert!(sample_variance(&singleton).is_err());
    assert!(sample_standard_deviation(&singleton).is_err());
}

#[test]
fn covariance_and_correlation_preserve_relationships() {
    let x = floats(&[1.0, 2.0, 3.0, 4.0]);
    let y = floats(&[2.0, 4.0, 6.0, 8.0]);

    assert_float_close(
        population_covariance(&x, &y).unwrap(),
        2.5,
        1e-12,
    );

    assert_float_close(
        sample_covariance(&x, &y).unwrap(),
        10.0 / 3.0,
        1e-12,
    );

    assert_float_close(
        pearson_correlation(&x, &y).unwrap(),
        1.0,
        1e-12,
    );
}

#[test]
fn covariance_rejects_different_input_lengths() {
    let x = floats(&[1.0, 2.0, 3.0]);
    let y = floats(&[4.0, 5.0]);

    assert!(population_covariance(&x, &y).is_err());
    assert!(sample_covariance(&x, &y).is_err());
    assert!(pearson_correlation(&x, &y).is_err());
}

#[test]
fn statistics_summary_agrees_with_individual_operations() {
    let values = floats(&[2.0, 4.0, 6.0, 8.0]);

    let summary = summarize(&values).unwrap();

    assert_float_eq(summary.mean(), mean(&values).unwrap());
    assert_float_eq(summary.minimum(), float(2.0));
    assert_float_eq(summary.maximum(), float(8.0));
}

// -------------------------------------------------------------------------
// Numerical reproducibility
// -------------------------------------------------------------------------

#[test]
fn ordered_sum_preserves_sequential_accumulation_contract() {
    let values = floats(&[1.0, 2.0, 3.0, 4.0]);

    assert_float_eq(ordered_sum(&values).unwrap(), 10.0);
}

#[test]
fn canonical_sum_is_repeatable_for_identical_ordered_input() {
    let values = floats(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let first = canonical_sum(&values).unwrap();
    let second = canonical_sum(&values).unwrap();

    assert_eq!(first.bits(), second.bits());
    assert_float_eq(first, 21.0);
}

#[test]
fn reproducibility_policy_selects_reduction_order_explicitly() {
    let values = floats(&[1.0, 2.0, 3.0, 4.0]);

    let ordered = ReproducibilityPolicy::new(
        ReproducibilityLevel::Numerical,
        ReductionOrder::InputOrder,
        ContractionPolicy::Forbidden,
    )
    .expect("valid numerical policy");

    let pairwise = ReproducibilityPolicy::new(
        ReproducibilityLevel::Numerical,
        ReductionOrder::CanonicalPairwise,
        ContractionPolicy::Forbidden,
    )
    .expect("valid pairwise policy");

    assert_float_eq(sum_with_policy(&values, ordered).unwrap(), 10.0);
    assert_float_eq(sum_with_policy(&values, pairwise).unwrap(), 10.0);
}

#[test]
fn bitwise_policy_requires_the_declared_canonical_reduction() {
    let invalid = ReproducibilityPolicy::new(
        ReproducibilityLevel::Bitwise,
        ReductionOrder::InputOrder,
        ContractionPolicy::Forbidden,
    );

    assert!(invalid.is_err());

    let valid = ReproducibilityPolicy::new(
        ReproducibilityLevel::Bitwise,
        ReductionOrder::CanonicalPairwise,
        ContractionPolicy::Forbidden,
    );

    assert!(valid.is_ok());
}

#[test]
fn ordered_and_canonical_dot_products_agree_for_simple_inputs() {
    let left = floats(&[1.0, 2.0, 3.0]);
    let right = floats(&[4.0, 5.0, 6.0]);

    let ordered = ordered_dot_product(&left, &right).unwrap();
    let canonical = canonical_dot_product(&left, &right).unwrap();

    assert_float_eq(ordered, 32.0);
    assert_float_eq(canonical, 32.0);
}

#[test]
fn dot_product_rejects_mismatched_dimensions() {
    let left = floats(&[1.0, 2.0, 3.0]);
    let right = floats(&[4.0, 5.0]);

    assert!(ordered_dot_product(&left, &right).is_err());
    assert!(canonical_dot_product(&left, &right).is_err());
}

#[test]
fn dot_product_policy_is_explicit_and_repeatable() {
    let left = floats(&[1.0, 2.0, 3.0, 4.0]);
    let right = floats(&[5.0, 6.0, 7.0, 8.0]);

    let policy = ReproducibilityPolicy::new(
        ReproducibilityLevel::Numerical,
        ReductionOrder::CanonicalPairwise,
        ContractionPolicy::Forbidden,
    )
    .unwrap();

    let first =
        dot_product_with_policy(&left, &right, policy).unwrap();

    let second =
        dot_product_with_policy(&left, &right, policy).unwrap();

    assert_eq!(first.bits(), second.bits());
    assert_float_eq(first, 70.0);
}

// -------------------------------------------------------------------------
// Scalability and resource-independent contract checks
// -------------------------------------------------------------------------

#[test]
fn integer_arithmetic_does_not_truncate_to_u64() {
    let lhs = UnsignedInteger::from_be_bytes(&[
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ]);

    let incremented =
        apply_unsigned(ArithmeticOperator::Add, &lhs, &unsigned(1))
            .unwrap();

    assert!(incremented.bit_width() > 64);
    assert_ne!(incremented, lhs);
}

#[test]
fn reductions_accept_dynamically_sized_collections() {
    // A representative stress input, not a maximum supported collection size.
    let count = 2_048_usize;

    let values = (0..count)
        .map(|_| float(1.0))
        .collect::<Vec<_>>();

    let ordered = ordered_sum(&values).unwrap();
    let pairwise = canonical_sum(&values).unwrap();

    assert_float_eq(ordered, count as f64);
    assert_float_eq(pairwise, count as f64);
}

#[test]
fn canonical_reduction_is_independent_of_repeated_invocation() {
    let values = floats(&[
        0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0,
    ]);

    let expected = canonical_sum(&values).unwrap();

    for _ in 0..32 {
        let actual = canonical_sum(&values).unwrap();
        assert_eq!(actual.bits(), expected.bits());
    }
}

#[test]
fn input_order_remains_part_of_the_numerical_contract() {
    let first_order = floats(&[1.0, 2.0, 3.0, 4.0]);
    let second_order = floats(&[4.0, 3.0, 2.0, 1.0]);

    let first = ordered_sum(&first_order).unwrap();
    let second = ordered_sum(&second_order).unwrap();

    assert_float_eq(first, 10.0);
    assert_float_eq(second, 10.0);
}

// -------------------------------------------------------------------------
// Error and diagnostic behavior
// -------------------------------------------------------------------------

#[test]
fn arithmetic_errors_are_structured_and_do_not_require_message_parsing() {
    let result =
        apply_unsigned(ArithmeticOperator::Subtract, &unsigned(1), &unsigned(2));

    let error = result.expect_err("negative unsigned result must fail");

    assert!(matches!(
        error,
        ArithmeticError::NegativeUnsignedResult
    ));

    assert!(!error.to_string().is_empty());
}

#[test]
fn elementary_domain_errors_are_structured() {
    let result = evaluate_unary(ElementaryFunction::Sqrt, float(-4.0));

    let error = result.expect_err("negative square root must fail");

    assert!(matches!(
        error,
        ElementaryError::InvalidDomain { .. }
    ));

    assert!(!error.to_string().is_empty());
}
