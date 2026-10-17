
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Sorting Algorithms
//!
//! File: `src/classical/algorithms/sorting.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! Status: Proposed implementation. Module registration, repository-wide
//! compilation, semantic integration, and CI verification are still required.
//!
//! # Purpose
//!
//! Provide target-independent sorting adapters for mutable, contiguous slices.
//! Sorting reorders existing elements according to an explicit ordering
//! contract; it does not select hardware, allocate a replacement collection,
//! or determine the meaning of source-language expressions.
//!
//! # Public API
//!
//! - `sort`: stable sort using `Ord`;
//! - `sort_by`: stable sort using a caller-provided comparator;
//! - `sort_by_key`: stable sort using a caller-provided key;
//! - `sort_unstable`: in-place, unstable sort using `Ord`;
//! - `sort_unstable_by`: in-place, unstable sort using a comparator;
//! - `sort_unstable_by_key`: in-place, unstable sort using a key.
//!
//! # Authority and ownership
//!
//! - `grammar/spec/data.md` defines normative data semantics.
//! - `grammar/concurrency/data-parallel.g4` owns data-parallel syntax,
//!   including the generic `parallel::sort` operation category.
//! - `grammar/classical/optimization.g4` owns the classical optimization
//!   composition boundary; it does not implement sorting algorithms.
//! - The frontend owns parsing, AST construction, and source locations.
//! - Semantic analysis owns type checking, callable resolution, effects,
//!   comparison legality, ordering requirements, and determinism guarantees.
//! - The existing classical collection modules own their collection types.
//! - The canonical compiler IR owns semantic operation representation.
//! - Execution and resource systems own placement, scheduling, budgets,
//!   cancellation, and target-specific realization.
//!
//! This module owns only generic slice-sorting adapters and documents their
//! ordering, stability, memory, and panic contracts.
//!
//! # Semantics
//!
//! 1. The input slice is reordered in place.
//! 2. Stable functions preserve the relative input order of elements for which
//!    the comparator returns `Ordering::Equal`.
//! 3. Unstable functions do not promise preservation of the relative order of
//!    equal elements.
//! 4. `Ord`-based functions use the type's declared total order.
//! 5. Comparator-based functions require a consistent total ordering. A
//!    comparator that violates ordering requirements can cause a panic or
//!    produce an ordering that does not meet the caller's intended semantics.
//! 6. Key-based functions inherit the ordering of the returned key type.
//! 7. These functions do not assume elements are numerically comparable.
//! 8. Floating-point callers must explicitly choose an ordering policy, such
//!    as `f32::total_cmp` or `f64::total_cmp`, when using comparator APIs.
//! 9. No function silently truncates the slice or drops elements.
//! 10. These functions do not define language-wide null, NaN, locale, collation,
//!     or domain-specific ordering policies.
//!
//! # Scalability and resources
//!
//! There is no fixed element-count ceiling in this module. The slice length is
//! supplied by the caller and is bounded by Rust's slice representation,
//! addressable memory, and available resources—not by a Zamani-specific
//! architectural maximum.
//!
//! Stable sorting delegates to Rust's stable slice sort and may use temporary
//! scratch storage. Unstable sorting delegates to Rust's in-place unstable
//! slice sort and does not require an auxiliary element buffer.
//!
//! Sorting still requires finite resources. No implementation can promise to
//! sort an arbitrarily large collection in finite time or memory. Callers
//! needing resource budgets, cancellation, external-memory sorting, streaming
//! sorting, or distributed sorting must use the appropriate higher-level
//! execution/data subsystem.
//!
//! # Complexity
//!
//! The standard-library sorting implementations provide O(n log n) worst-case
//! comparison complexity. Actual cost depends on the comparator/key function,
//! input distribution, element moves, and target implementation.
//!
//! Stable sorting may use additional scratch memory. Unstable sorting avoids
//! an auxiliary element buffer. This module does not promise constant runtime
//! memory for stable sorting.
//!
//! # Determinism and portability
//!
//! Results are deterministic when the input, ordering relation, and any
//! comparator/key behavior are deterministic. A comparator must not depend on
//! unstable external state if reproducible results are required.
//!
//! Target-specific optimization may replace these adapters only if it
//! preserves the validated ordering, stability, effects, and observable
//! semantics required by the source program.
//!
//! # Integration contract
//!
//! `src/classical/algorithms/mod.rs` should declare `pub mod sorting;` and
//! export the public API according to the subsystem's visibility policy.
//!
//! Collection adapters may call these functions on their existing mutable
//! slices when that matches their ownership and mutation contract. They must
//! not create competing array/vector abstractions.
//!
//! Semantic analysis should validate the source operation and its comparator.
//! Lowering should map the validated operation into the existing canonical IR;
//! this file does not define IR nodes or rewrite source syntax.
//!
//! `parallel::sort` is a logical operation, not a promise of physical
//! parallelism. Parallel/distributed sorting belongs in the execution layer
//! and must preserve the language's ordering and determinism contract.
//!
//! # Compatibility and freeze rule
//!
//! This API uses stable Rust standard-library slice sorting. Public names and
//! stability guarantees are part of its contract. Changing stable sorting to
//! unstable sorting, changing comparator semantics, or introducing implicit
//! ordering policies requires a compatibility review and regression tests.

use core::cmp::Ordering;

/// Sorts a mutable slice into ascending order using `T::cmp`.
///
/// This sort is stable: elements that compare equal retain their relative
/// order from the input.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::sorting::sort;
///
/// let mut values = [4, 1, 3, 1, 2];
/// sort(&mut values);
///
/// assert_eq!(values, [1, 1, 2, 3, 4]);
/// ```
pub fn sort<T: Ord>(values: &mut [T]) {
    values.sort();
}

/// Sorts a mutable slice using a caller-provided comparator.
///
/// This sort is stable. The comparator must implement a consistent total
/// ordering for the values being sorted.
///
/// The comparator may be called multiple times for the same element pair.
/// If reproducibility is required, it should not depend on changing external
/// state.
///
/// # Examples
///
/// ```
/// use core::cmp::Ordering;
/// use zamani_compiler::classical::algorithms::sorting::sort_by;
///
/// let mut values = ["pear", "fig", "banana", "plum"];
///
/// sort_by(&mut values, |left, right| {
///     left.len().cmp(&right.len()).then_with(|| left.cmp(right))
/// });
///
/// assert_eq!(values, ["fig", "pear", "plum", "banana"]);
/// ```
pub fn sort_by<T, F>(values: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    values.sort_by(|left, right| compare(left, right));
}

/// Sorts a mutable slice by a derived key.
///
/// This sort is stable. Keys are computed as required by Rust's stable
/// `sort_by_key` implementation; a key function can be called more than once
/// for an element.
///
/// Prefer a pure, deterministic key function when reproducible ordering is
/// required.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::sorting::sort_by_key;
///
/// let mut records = [("gamma", 3), ("alpha", 1), ("beta", 2)];
/// sort_by_key(&mut records, |record| record.1);
///
/// assert_eq!(records, [("alpha", 1), ("beta", 2), ("gamma", 3)]);
/// ```
pub fn sort_by_key<T, K, F>(values: &mut [T], mut key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    values.sort_by_key(|value| key(value));
}

/// Sorts a mutable slice into ascending order using `T::cmp`.
///
/// This sort is unstable: equal elements may change relative order. It avoids
/// an auxiliary element buffer and is suitable when stability is not required.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::sorting::sort_unstable;
///
/// let mut values = [9, 2, 7, 2, 4];
/// sort_unstable(&mut values);
///
/// assert_eq!(values, [2, 2, 4, 7, 9]);
/// ```
pub fn sort_unstable<T: Ord>(values: &mut [T]) {
    values.sort_unstable();
}

/// Sorts a mutable slice using a caller-provided comparator without a
/// stability guarantee.
///
/// The comparator must implement a consistent total ordering. This function
/// does not allocate an auxiliary element buffer for the sorting algorithm.
///
/// # Examples
///
/// ```
/// use core::cmp::Ordering;
/// use zamani_compiler::classical::algorithms::sorting::sort_unstable_by;
///
/// let mut values = [3.5_f64, f64::NAN, -0.0, 0.0, -2.0];
///
/// sort_unstable_by(&mut values, f64::total_cmp);
///
/// assert!(values.windows(2).all(|pair| {
///     pair[0].total_cmp(&pair[1]) != Ordering::Greater
/// }));
/// ```
pub fn sort_unstable_by<T, F>(values: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    values.sort_unstable_by(|left, right| compare(left, right));
}

/// Sorts a mutable slice by a derived key without a stability guarantee.
///
/// The key type must implement `Ord`. The key function can be invoked more
/// than once per element, so avoid relying on side effects.
///
/// This function avoids an auxiliary element buffer for sorting.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::sorting::sort_unstable_by_key;
///
/// let mut values = [("third", 3), ("first", 1), ("second", 2)];
/// sort_unstable_by_key(&mut values, |value| value.1);
///
/// assert_eq!(values.map(|value| value.1), [1, 2, 3]);
/// ```
pub fn sort_unstable_by_key<T, K, F>(values: &mut [T], mut key: F)
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    values.sort_unstable_by_key(|value| key(value));
}

#[cfg(test)]
mod tests {
    use super::{
        sort, sort_by, sort_by_key, sort_unstable, sort_unstable_by,
        sort_unstable_by_key,
    };
    use core::cmp::Ordering;

    #[test]
    fn stable_sort_orders_empty_slice() {
        let mut values: [i32; 0] = [];
        sort(&mut values);
        assert!(values.is_empty());
    }

    #[test]
    fn stable_sort_orders_single_element() {
        let mut values = [42];
        sort(&mut values);
        assert_eq!(values, [42]);
    }

    #[test]
    fn stable_sort_orders_duplicates() {
        let mut values = [5, 2, 5, 1, 2, 3];
        sort(&mut values);
        assert_eq!(values, [1, 2, 2, 3, 5, 5]);
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Record {
        key: i32,
        original_position: usize,
    }

    #[test]
    fn stable_sort_by_preserves_equal_element_order() {
        let mut values = [
            Record { key: 2, original_position: 0 },
            Record { key: 1, original_position: 1 },
            Record { key: 2, original_position: 2 },
            Record { key: 1, original_position: 3 },
            Record { key: 2, original_position: 4 },
        ];

        sort_by(&mut values, |left, right| left.key.cmp(&right.key));

        assert_eq!(
            values.iter().map(|item| item.original_position).collect::<Vec<_>>(),
            [1, 3, 0, 2, 4]
        );
    }

    #[test]
    fn stable_sort_by_key_orders_records() {
        let mut values = [("c", 3), ("a", 1), ("b", 2)];
        sort_by_key(&mut values, |value| value.1);
        assert_eq!(values, [("a", 1), ("b", 2), ("c", 3)]);
    }

    #[test]
    fn unstable_sort_orders_values() {
        let mut values = [8, 1, 7, 1, 5, 2];
        sort_unstable(&mut values);
        assert_eq!(values, [1, 1, 2, 5, 7, 8]);
    }

    #[test]
    fn unstable_sort_by_supports_explicit_float_ordering() {
        let mut values = [f64::NAN, 4.0, -0.0, 0.0, -3.0];

        sort_unstable_by(&mut values, f64::total_cmp);

        assert!(values.windows(2).all(|pair| {
            pair[0].total_cmp(&pair[1]) != Ordering::Greater
        }));
    }

    #[test]
    fn unstable_sort_by_key_orders_values_by_key() {
        let mut values = [("gamma", 3), ("alpha", 1), ("beta", 2)];
        sort_unstable_by_key(&mut values, |value| value.1);

        assert_eq!(
            values.iter().map(|value| value.1).collect::<Vec<_>>(),
            [1, 2, 3]
        );
    }

    #[test]
    fn sorting_is_idempotent_for_a_deterministic_total_order() {
        let mut values = [6, -1, 6, 0, 8, -1];

        sort(&mut values);
        let once = values;
        sort(&mut values);

        assert_eq!(values, once);
    }

    #[test]
    fn sorting_preserves_the_number_of_elements() {
        let mut values = [7, 3, 9, 1, 4, 2];
        let original_len = values.len();

        sort_unstable(&mut values);

        assert_eq!(values.len(), original_len);
        assert_eq!(values, [1, 2, 3, 4, 7, 9]);
    }

    #[test]
    fn large_slice_needs_no_fixed_capacity_buffer_in_this_api() {
        let mut values: Vec<u64> = (0..100_000).rev().collect();

        sort_unstable(&mut values);

        assert!(values.windows(2).all(|pair| pair[0] <= pair[1]));
        assert_eq!(values.first(), Some(&0));
        assert_eq!(values.last(), Some(&99_999));
    }
}
