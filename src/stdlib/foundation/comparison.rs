
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Generic Comparison
//!
//! File:
//!     src/stdlib/foundation/comparison.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! Safety:
//!     Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide reusable, target-independent comparison utilities for values
//! implementing Rust's standard comparison traits.
//!
//! This module supports:
//!
//! - equality and inequality;
//! - partial ordering;
//! - total ordering;
//! - relational predicates;
//! - projection-based equality;
//! - projection-based partial ordering;
//! - projection-based total ordering.
//!
//! It is intended to be usable by standard-library APIs across classical,
//! quantum, HDL, hybrid, distributed, AI, scientific and future domains.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns only generic comparison helpers.
//!
//! It does not own:
//!
//! - source-language comparison operator semantics;
//! - semantic type compatibility;
//! - implicit numeric conversions;
//! - arbitrary-precision integer representations;
//! - floating-point representations;
//! - canonical classical values;
//! - symbolic algebra;
//! - tensor or collection storage;
//! - quantum value semantics;
//! - HDL value semantics;
//! - runtime execution;
//! - hardware selection;
//! - scheduling or resource allocation.
//!
//! Canonical authorities remain responsible for their respective domains.
//!
//! In particular:
//!
//! - `src/frontend/semantic/types/compatibility.rs` owns type compatibility.
//! - `src/classical/values/comparison.rs` owns canonical classical-value
//!   comparison semantics.
//! - `src/quantum/` owns quantum-specific value and execution semantics.
//! - `src/hdl/` owns HDL-specific type and value semantics.
//!
//! ## COMPARISON CONTRACT
//!
//! Equality and ordering are deliberately separate.
//!
//! `equal` uses `PartialEq`.
//!
//! `partial_compare` uses `PartialOrd` and preserves `None` when values
//! cannot be ordered.
//!
//! `total_compare` uses `Ord` and is available only for types that promise
//! a total ordering.
//!
//! The module does not assume that equality can be inferred from partial
//! ordering. Custom types may have domain-specific implementations.
//!
//! ## FLOATING-POINT CONTRACT
//!
//! Native floating-point behavior is preserved:
//!
//! - NaN is not equal to itself.
//! - NaN has no partial ordering against ordinary floating-point values.
//! - Positive and negative zero compare numerically equal.
//! - No approximate equality or tolerance is introduced.
//! - No NaN normalization or implicit conversion is performed.
//!
//! Applications requiring approximate numerical comparison must specify
//! their tolerance and numerical policy explicitly.
//!
//! ## TEXT CONTRACT
//!
//! Rust string equality compares the exact string contents.
//!
//! Rust string ordering is deterministic and independent of locale.
//!
//! This module does not perform:
//!
//! - Unicode normalization;
//! - grapheme-cluster comparison;
//! - locale-sensitive collation;
//! - case folding;
//! - approximate text matching.
//!
//! Those operations require separately specified semantics.
//!
//! ## SCALABILITY
//!
//! No arbitrary maximum number of comparisons is imposed.
//!
//! No arbitrary value-size limit is introduced.
//!
//! No processor, memory, device, qubit, register, tensor-rank, node,
//! accelerator or hardware limit is introduced.
//!
//! Comparison operations allocate no intermediate storage.
//!
//! Actual execution remains subject to the representation and computational
//! capabilities of the compared values and the resources available to the
//! calling program.
//!
//! No finite implementation promises literally infinite computation.
//!
//! ## ERROR CONTRACT
//!
//! These helpers do not manufacture standard-library errors.
//!
//! An unordered partial comparison is represented by `None`.
//!
//! This is distinct from:
//!
//! - equality;
//! - inequality;
//! - an invalid source-language expression;
//! - an unsupported domain operation;
//! - a failed provider operation.
//!
//! The semantic or domain-specific caller determines how such outcomes
//! should be interpreted.
//!
//! ## DEPENDENCIES
//!
//! This module depends only on `core` comparison traits and types.
//!
//! It does not depend on:
//!
//! - runtime;
//! - compiler;
//! - semantic analysis;
//! - IR;
//! - hardware;
//! - external crates.
//!
//! ## INTEGRATION
//!
//! Declare this module from:
//!
//!     src/stdlib/foundation/mod.rs
//!
//! Re-export its public functions through the foundation module.
//!
//! Declare the foundation module from:
//!
//!     src/stdlib/mod.rs
//!
//! No initialization or shutdown function is required.
//!
//! ## FREEZE RULE
//!
//! Changes to the meaning of equality, partial ordering, total ordering,
//! or floating-point behavior require an explicit API and compatibility
//! review.
//!
//! This module must not acquire domain-specific comparison semantics.

use core::cmp::Ordering;

// ============================================================================
// EQUALITY
// ============================================================================

/// Determines whether two values are equal.
///
/// Equality follows the implementation of `PartialEq`.
///
/// This function deliberately does not infer equality from ordering.
///
/// # Examples
///
/// ```
/// use zamani_compiler::stdlib::foundation::comparison::equal;
///
/// assert!(equal(&42, &42));
/// assert!(!equal(&42, &43));
/// ```
#[must_use]
pub fn equal<L, R>(left: &L, right: &R) -> bool
where
    L: PartialEq<R> + ?Sized,
    R: ?Sized,
{
    left.eq(right)
}

/// Determines whether two values are unequal.
///
/// This is the logical negation of the `PartialEq` result.
///
/// It does not imply that the values have a defined ordering.
///
/// # Examples
///
/// ```
/// use zamani_compiler::stdlib::foundation::comparison::not_equal;
///
/// assert!(not_equal(&10, &20));
/// assert!(!not_equal(&10, &10));
/// ```
#[must_use]
pub fn not_equal<L, R>(left: &L, right: &R) -> bool
where
    L: PartialEq<R> + ?Sized,
    R: ?Sized,
{
    !equal(left, right)
}

// ============================================================================
// PARTIAL ORDERING
// ============================================================================

/// Compares two values using their partial-ordering implementation.
///
/// Returns:
///
/// - `Some(Ordering::Less)` when the left value is less;
/// - `Some(Ordering::Equal)` when the values are equal under the ordering;
/// - `Some(Ordering::Greater)` when the left value is greater;
/// - `None` when the values are unordered.
///
/// `None` is not converted into equality or inequality.
///
/// This is particularly important for floating-point NaN and for
/// application-defined partially ordered types.
///
/// # Examples
///
/// ```
/// use core::cmp::Ordering;
/// use zamani_compiler::stdlib::foundation::comparison::partial_compare;
///
/// assert_eq!(partial_compare(&10, &20), Some(Ordering::Less));
/// assert_eq!(partial_compare(&20, &20), Some(Ordering::Equal));
/// assert_eq!(partial_compare(&30, &20), Some(Ordering::Greater));
/// ```
#[must_use]
pub fn partial_compare<L, R>(left: &L, right: &R) -> Option<Ordering>
where
    L: PartialOrd<R> + ?Sized,
    R: ?Sized,
{
    left.partial_cmp(right)
}

// ============================================================================
// TOTAL ORDERING
// ============================================================================

/// Compares two values that implement a total ordering.
///
/// Unlike [`partial_compare`], this function is available only when the
/// type implements `Ord`.
///
/// The implementation of `Ord` is responsible for satisfying Rust's total
/// ordering contract.
///
/// This function does not manufacture a total order for partially ordered
/// types.
///
/// # Examples
///
/// ```
/// use core::cmp::Ordering;
/// use zamani_compiler::stdlib::foundation::comparison::total_compare;
///
/// assert_eq!(total_compare(&"alpha", &"beta"), Ordering::Less);
/// assert_eq!(total_compare(&42, &42), Ordering::Equal);
/// ```
#[must_use]
pub fn total_compare<T>(left: &T, right: &T) -> Ordering
where
    T: Ord + ?Sized,
{
    left.cmp(right)
}

// ============================================================================
// RELATIONAL PREDICATES
// ============================================================================

/// Returns whether the left value is strictly less than the right value.
///
/// An unordered comparison returns `false`.
#[must_use]
pub fn is_less<L, R>(left: &L, right: &R) -> bool
where
    L: PartialOrd<R> + ?Sized,
    R: ?Sized,
{
    matches!(partial_compare(left, right), Some(Ordering::Less))
}

/// Returns whether the left value is less than or equal to the right value.
///
/// An unordered comparison returns `false`.
#[must_use]
pub fn is_less_or_equal<L, R>(left: &L, right: &R) -> bool
where
    L: PartialOrd<R> + ?Sized,
    R: ?Sized,
{
    matches!(
        partial_compare(left, right),
        Some(Ordering::Less | Ordering::Equal)
    )
}

/// Returns whether the left value is strictly greater than the right value.
///
/// An unordered comparison returns `false`.
#[must_use]
pub fn is_greater<L, R>(left: &L, right: &R) -> bool
where
    L: PartialOrd<R> + ?Sized,
    R: ?Sized,
{
    matches!(partial_compare(left, right), Some(Ordering::Greater))
}

/// Returns whether the left value is greater than or equal to the right value.
///
/// An unordered comparison returns `false`.
#[must_use]
pub fn is_greater_or_equal<L, R>(left: &L, right: &R) -> bool
where
    L: PartialOrd<R> + ?Sized,
    R: ?Sized,
{
    matches!(
        partial_compare(left, right),
        Some(Ordering::Greater | Ordering::Equal)
    )
}

// ============================================================================
// PROJECTION-BASED COMPARISON
// ============================================================================

/// Compares two values using a projected key.
///
/// The projection is evaluated once for each operand.
///
/// This is useful when a type has several fields but a caller needs to
/// compare only one selected property.
///
/// The projected keys must implement `PartialOrd`.
///
/// No intermediate collection or heap allocation is required.
///
/// # Examples
///
/// ```
/// use core::cmp::Ordering;
/// use zamani_compiler::stdlib::foundation::comparison::compare_by;
///
/// struct Item {
///     priority: u64,
///     name: &'static str,
/// }
///
/// let first = Item {
///     priority: 10,
///     name: "first",
/// };
///
/// let second = Item {
///     priority: 20,
///     name: "second",
/// };
///
/// assert_eq!(
///     compare_by(&first, &second, |item| item.priority),
///     Some(Ordering::Less),
/// );
/// ```
#[must_use]
pub fn compare_by<T, K, F>(
    left: &T,
    right: &T,
    mut key: F,
) -> Option<Ordering>
where
    T: ?Sized,
    K: PartialOrd,
    F: FnMut(&T) -> K,
{
    let left_key = key(left);
    let right_key = key(right);

    left_key.partial_cmp(&right_key)
}

/// Compares two values using a projected key with total ordering.
///
/// This function requires the projected key to implement `Ord`.
///
/// The projection is evaluated once for each operand.
#[must_use]
pub fn total_compare_by<T, K, F>(
    left: &T,
    right: &T,
    mut key: F,
) -> Ordering
where
    T: ?Sized,
    K: Ord,
    F: FnMut(&T) -> K,
{
    let left_key = key(left);
    let right_key = key(right);

    left_key.cmp(&right_key)
}

/// Determines equality using a projected key.
///
/// The projection is evaluated once for each operand.
///
/// Equality follows the projected key's `PartialEq` implementation.
#[must_use]
pub fn equal_by<T, K, F>(
    left: &T,
    right: &T,
    mut key: F,
) -> bool
where
    T: ?Sized,
    K: PartialEq,
    F: FnMut(&T) -> K,
{
    let left_key = key(left);
    let right_key = key(right);

    left_key == right_key
}

/// Determines inequality using a projected key.
///
/// This is the logical negation of [`equal_by`].
#[must_use]
pub fn not_equal_by<T, K, F>(
    left: &T,
    right: &T,
    key: F,
) -> bool
where
    T: ?Sized,
    K: PartialEq,
    F: FnMut(&T) -> K,
{
    !equal_by(left, right, key)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Equality
    // ------------------------------------------------------------------------

    #[test]
    fn equality_uses_partial_eq() {
        assert!(equal(&10_u64, &10_u64));
        assert!(!equal(&10_u64, &11_u64));

        assert!(not_equal(&10_u64, &11_u64));
        assert!(!not_equal(&10_u64, &10_u64));
    }

    #[test]
    fn equality_supports_heterogeneous_partial_eq() {
        let text: &str = "zamani";
        let owned = String::from("zamani");

        assert!(equal(text, &owned));
        assert!(!not_equal(text, &owned));
    }

    // ------------------------------------------------------------------------
    // Partial ordering
    // ------------------------------------------------------------------------

    #[test]
    fn partial_ordering_distinguishes_all_ordered_results() {
        assert_eq!(
            partial_compare(&1, &2),
            Some(Ordering::Less),
        );

        assert_eq!(
            partial_compare(&2, &2),
            Some(Ordering::Equal),
        );

        assert_eq!(
            partial_compare(&3, &2),
            Some(Ordering::Greater),
        );
    }

    #[test]
    fn partial_ordering_preserves_unordered_float_values() {
        let nan = f64::NAN;
        let value = 1.0_f64;

        assert_eq!(partial_compare(&nan, &value), None);
        assert_eq!(partial_compare(&value, &nan), None);
        assert_eq!(partial_compare(&nan, &nan), None);

        assert!(!equal(&nan, &nan));
        assert!(not_equal(&nan, &nan));

        assert!(!is_less(&nan, &value));
        assert!(!is_less_or_equal(&nan, &value));
        assert!(!is_greater(&nan, &value));
        assert!(!is_greater_or_equal(&nan, &value));
    }

    #[test]
    fn floating_point_signed_zero_keeps_native_semantics() {
        let positive_zero = 0.0_f64;
        let negative_zero = -0.0_f64;

        assert!(equal(&positive_zero, &negative_zero));

        assert_eq!(
            partial_compare(&positive_zero, &negative_zero),
            Some(Ordering::Equal),
        );
    }

    // ------------------------------------------------------------------------
    // Total ordering
    // ------------------------------------------------------------------------

    #[test]
    fn total_ordering_uses_ord() {
        assert_eq!(
            total_compare(&1_u64, &2_u64),
            Ordering::Less,
        );

        assert_eq!(
            total_compare(&2_u64, &2_u64),
            Ordering::Equal,
        );

        assert_eq!(
            total_compare(&3_u64, &2_u64),
            Ordering::Greater,
        );
    }

    #[test]
    fn total_ordering_supports_strings() {
        assert_eq!(
            total_compare("alpha", "beta"),
            Ordering::Less,
        );

        assert_eq!(
            total_compare("beta", "alpha"),
            Ordering::Greater,
        );

        assert_eq!(
            total_compare("same", "same"),
            Ordering::Equal,
        );
    }

    #[test]
    fn string_comparison_is_deterministic_and_exact() {
        let composed = "é";
        let decomposed = "e\u{301}";

        // These strings have different Unicode scalar sequences.
        // No normalization is silently applied.
        assert!(!equal(composed, decomposed));

        assert_ne!(
            partial_compare(composed, decomposed),
            Some(Ordering::Equal),
        );
    }

    // ------------------------------------------------------------------------
    // Relational predicates
    // ------------------------------------------------------------------------

    #[test]
    fn relational_predicates_cover_ordered_values() {
        assert!(is_less(&1, &2));
        assert!(is_less_or_equal(&1, &2));
        assert!(is_less_or_equal(&2, &2));

        assert!(is_greater(&3, &2));
        assert!(is_greater_or_equal(&3, &2));
        assert!(is_greater_or_equal(&2, &2));

        assert!(!is_less(&2, &2));
        assert!(!is_greater(&2, &2));
    }

    #[test]
    fn relational_predicates_return_false_for_unordered_values() {
        let nan = f64::NAN;
        let value = 1.0_f64;

        assert!(!is_less(&nan, &value));
        assert!(!is_less_or_equal(&nan, &value));
        assert!(!is_greater(&nan, &value));
        assert!(!is_greater_or_equal(&nan, &value));
    }

    // ------------------------------------------------------------------------
    // Projection-based comparison
    // ------------------------------------------------------------------------

    #[derive(Debug)]
    struct Record {
        identifier: u64,
        priority: u32,
        label: &'static str,
    }

    #[test]
    fn projection_comparison_uses_selected_key() {
        let first = Record {
            identifier: 100,
            priority: 5,
            label: "first",
        };

        let second = Record {
            identifier: 200,
            priority: 10,
            label: "second",
        };

        assert_eq!(
            compare_by(&first, &second, |record| record.priority),
            Some(Ordering::Less),
        );

        assert_eq!(
            total_compare_by(
                &first,
                &second,
                |record| record.identifier,
            ),
            Ordering::Less,
        );

        assert!(equal_by(
            &first,
            &second,
            |record| record.label.len(),
        ));

        assert!(not_equal_by(
            &first,
            &second,
            |record| record.identifier,
        ));
    }

    #[test]
    fn projection_comparison_preserves_unordered_keys() {
        let first = Record {
            identifier: 1,
            priority: 1,
            label: "first",
        };

        let second = Record {
            identifier: 2,
            priority: 2,
            label: "second",
        };

        let result = compare_by(
            &first,
            &second,
            |_| f64::NAN,
        );

        assert_eq!(result, None);
    }

    // ------------------------------------------------------------------------
    // Slices and standard-library types
    // ------------------------------------------------------------------------

    #[test]
    fn slice_comparison_uses_element_ordering() {
        let left = [1, 2, 3];
        let right = [1, 2, 4];

        assert_eq!(
            partial_compare(&left[..], &right[..]),
            Some(Ordering::Less),
        );
    }

    #[test]
    fn empty_slices_compare_correctly() {
        let empty: [u8; 0] = [];
        let non_empty = [1_u8];

        assert_eq!(
            partial_compare(&empty[..], &empty[..]),
            Some(Ordering::Equal),
        );

        assert_eq!(
            partial_compare(&empty[..], &non_empty[..]),
            Some(Ordering::Less),
        );
    }
}
