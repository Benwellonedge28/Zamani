
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Searching Algorithms
//!
//! Path: `src/classical/algorithms/searching.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide generic, target-independent search operations for slices and
//! iterators. Searching determines whether, where, or how an item satisfying
//! a specified comparison or predicate occurs in an input sequence.
//!
//! # Ownership and authority
//!
//! - `grammar/spec/classical.md` defines normative classical-computing
//!   semantics.
//! - `grammar/spec/data.md` defines normative data, ordering, and
//!   scalability requirements.
//! - `grammar/concurrency/data-parallel.g4` owns data-parallel source syntax.
//! - The frontend owns parsing, AST construction, and source spans.
//! - Semantic analysis owns type checking, callable resolution, effects,
//!   ordering preconditions, and determinism requirements.
//! - The canonical IR owns semantic operation representation.
//! - Collection modules own collection representations and their invariants.
//! - Execution, scheduling, and resource systems own placement, budgets,
//!   cancellation, and target-specific realization.
//!
//! This file owns only generic sequential search algorithms and their
//! documented contracts. It does not define source syntax, AST nodes,
//! canonical values or types, IR nodes, hardware identities, or a scheduler.
//!
//! # Public API
//!
//! - `linear_search`: find an equal item in a slice.
//! - `linear_search_by`: find the first item satisfying a predicate.
//! - `linear_search_by_key`: find the first item whose key equals a target.
//! - `find_iter`: lazily consume an iterator until a predicate matches.
//! - `binary_search`: search an ascending, `Ord`-sorted slice.
//! - `binary_search_by`: search a sorted slice using a comparator.
//! - `binary_search_by_key`: search a sorted slice by a derived key.
//! - `lower_bound`: find the first position not less than a target.
//! - `lower_bound_by`: comparator-based lower bound.
//! - `upper_bound`: find the first position greater than a target.
//! - `upper_bound_by`: comparator-based upper bound.
//!
//! # Semantics
//!
//! Linear search preserves input order and returns the first match.
//!
//! Binary search and bound operations require the input to be sorted under
//! the same ordering relation used by the search. Violating this precondition
//! makes the result unspecified at the algorithm-contract level: a returned
//! position must not be treated as proof that an unsorted input was searched
//! correctly.
//!
//! `binary_search` returns `Ok(index)` when an equal item is found, and
//! `Err(insertion_index)` when no equal item is found. If duplicates exist,
//! the returned matching index is not guaranteed to be the first duplicate.
//! Use `lower_bound` to locate the first position in an equal range.
//!
//! Comparator functions receive an element and compare it with the logical
//! search target. `Ordering::Less` means the element precedes the target;
//! `Ordering::Equal` means it matches; `Ordering::Greater` means it follows
//! the target.
//!
//! Predicate-based bounds use a monotone predicate: all `true` elements must
//! precede all `false` elements. A predicate that violates this requirement
//! cannot provide a meaningful partition boundary.
//!
//! # Scalability and resources
//!
//! No Zamani-specific maximum input size is imposed. Slice operations use the
//! slice supplied by the caller; iterator search consumes items on demand and
//! does not collect them into a new collection.
//!
//! Linear search takes O(n) predicate/comparison calls in the worst case.
//! Binary search and bound operations take O(log n) comparisons on sorted
//! slices. Actual time depends on comparison cost, data layout, and target.
//!
//! An iterator search may terminate early when it finds a match. It may never
//! terminate if the input is infinite and no matching item occurs. This file
//! does not promise infinite computation, infinite address space, or unlimited
//! memory. Physical representation and available resources remain finite.
//!
//! # Determinism
//!
//! Results are reproducible when input order and the comparison or predicate
//! behavior are deterministic. Callbacks should not depend on changing
//! external state when reproducibility is required.
//!
//! # Integration contract
//!
//! `src/classical/algorithms/mod.rs` should declare `pub mod searching;`.
//! The parent module should export only the intended public API.
//!
//! `src/classical/collections/vector.rs` and `array.rs` may adapt their
//! existing ordered slices to these operations when their access and
//! mutation contracts permit it. This file must not define replacement
//! vector, array, map, set, or iterator collection types.
//!
//! Semantic analysis must validate the source operation, argument types,
//! comparator/key effects, sortedness preconditions where required, and any
//! determinism guarantees. Lowering must use the repository's existing
//! canonical IR; this module must not create a parallel IR.
//!
//! `parallel::` operations express logical data-parallel intent. This
//! sequential module does not claim to implement parallel or distributed
//! search. A downstream implementation may parallelize only when it preserves
//! the source operation's required result, ordering, effects, and determinism.
//!
//! # Freeze rule
//!
//! Changing first-match behavior, binary-search result semantics, comparator
//! direction, or bound semantics requires a compatibility review and
//! regression tests.

use core::cmp::Ordering;

/// Finds the first item equal to `target`.
///
/// Returns its zero-based index, or `None` if no equal item exists.
///
/// This function does not require the input to be sorted.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::searching::linear_search;
///
/// assert_eq!(linear_search(&[7, 3, 9, 3], &3), Some(1));
/// assert_eq!(linear_search(&[7, 9], &3), None);
/// ```
#[must_use]
pub fn linear_search<T: PartialEq>(values: &[T], target: &T) -> Option<usize> {
    values.iter().position(|value| value == target)
}

/// Finds the first item for which `predicate` returns `true`.
///
/// The input need not be sorted. The predicate may be called multiple times
/// and should be deterministic if reproducible results are required.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::searching::linear_search_by;
///
/// let values = [4, 7, 10, 13];
/// assert_eq!(linear_search_by(&values, |value| value % 2 == 1), Some(1));
/// ```
#[must_use]
pub fn linear_search_by<T, F>(values: &[T], mut predicate: F) -> Option<usize>
where
    F: FnMut(&T) -> bool,
{
    values.iter().position(|value| predicate(value))
}

/// Finds the first item whose derived key equals `target`.
///
/// This function does not require the input to be sorted.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::searching::linear_search_by_key;
///
/// let records = [("alpha", 10), ("beta", 20), ("gamma", 10)];
/// assert_eq!(linear_search_by_key(&records, &10, |record| record.1), Some(0));
/// ```
#[must_use]
pub fn linear_search_by_key<T, K, F>(
    values: &[T],
    target: &K,
    mut key: F,
) -> Option<usize>
where
    K: PartialEq,
    F: FnMut(&T) -> K,
{
    values.iter().position(|value| key(value) == *target)
}

/// Consumes an iterator until the first matching item is found.
///
/// Returns the matching item, or `None` if the iterator ends without a match.
/// The iterator is consumed only as far as necessary to find a match.
///
/// For an infinite iterator with no matching item, this function does not
/// terminate. Use an execution-layer budget or cancellation policy when
/// bounded execution is required.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::searching::find_iter;
///
/// assert_eq!(find_iter(0.., |value| value == 12), Some(12));
/// assert_eq!(find_iter([2, 4, 6], |value| value == 5), None);
/// ```
#[must_use]
pub fn find_iter<I, F>(iterable: I, predicate: F) -> Option<I::Item>
where
    I: IntoIterator,
    F: FnMut(&I::Item) -> bool,
{
    iterable.into_iter().find(predicate)
}

/// Searches an ascending slice using the type's `Ord` implementation.
///
/// Returns `Ok(index)` if an equal item is found. Otherwise, returns
/// `Err(insertion_index)`, the position at which the target could be inserted
/// while preserving ascending order.
///
/// The slice must already be sorted according to `Ord`.
///
/// If duplicates exist, the matching index is not guaranteed to be the first
/// occurrence. Use [`lower_bound`] to find the first position not less than
/// the target.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::searching::binary_search;
///
/// let values = [1, 3, 3, 8, 12];
/// assert!(matches!(binary_search(&values, &8), Ok(3)));
/// assert_eq!(binary_search(&values, &5), Err(3));
/// ```
#[must_use]
pub fn binary_search<T: Ord>(values: &[T], target: &T) -> Result<usize, usize> {
    values.binary_search(target)
}

/// Searches a sorted slice using a caller-provided comparator.
///
/// The comparator receives each slice element and compares it with the search
/// target. Its ordering must be consistent with the ordering used to sort the
/// slice.
///
/// Returns `Ok(index)` on a match or `Err(insertion_index)` otherwise.
///
/// # Examples
///
/// ```
/// use core::cmp::Ordering;
/// use zamani_compiler::classical::algorithms::searching::binary_search_by;
///
/// let values = [1, 3, 5, 7, 9];
/// assert_eq!(
///     binary_search_by(&values, |value| value.cmp(&7)),
///     Ok(3)
/// );
/// assert_eq!(
///     binary_search_by(&values, |value| value.cmp(&6)),
///     Err(3)
/// );
/// ```
#[must_use]
pub fn binary_search_by<T, F>(
    values: &[T],
    compare: F,
) -> Result<usize, usize>
where
    F: FnMut(&T) -> Ordering,
{
    values.binary_search_by(compare)
}

/// Searches a sorted slice by a derived key.
///
/// The slice must be sorted in ascending order by the same key relation.
/// The key function should be deterministic and free from order-changing
/// side effects.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::searching::binary_search_by_key;
///
/// let records = [("alpha", 1), ("beta", 3), ("gamma", 8)];
/// assert_eq!(binary_search_by_key(&records, &3, |record| record.1), Ok(1));
/// assert_eq!(binary_search_by_key(&records, &4, |record| record.1), Err(2));
/// ```
#[must_use]
pub fn binary_search_by_key<T, K, F>(
    values: &[T],
    target: &K,
    mut key: F,
) -> Result<usize, usize>
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    values.binary_search_by_key(target, |value| key(value))
}

/// Finds the first index whose value is not less than `target`.
///
/// Equivalently, this returns the first index `i` for which
/// `values[i] >= target`, or `values.len()` if every value is less.
///
/// The input must be sorted in ascending order.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::searching::lower_bound;
///
/// assert_eq!(lower_bound(&[1, 2, 2, 2, 5], &2), 1);
/// assert_eq!(lower_bound(&[1, 2, 5], &3), 2);
/// assert_eq!(lower_bound(&[1, 2, 5], &9), 3);
/// ```
#[must_use]
pub fn lower_bound<T: Ord>(values: &[T], target: &T) -> usize {
    lower_bound_by(values, |value| value.cmp(target))
}

/// Comparator-based lower bound.
///
/// `compare(element)` must return `Ordering::Less` exactly for elements that
/// precede the target. The slice must be sorted according to that comparison.
///
/// Returns the first position where the comparison is not `Less`.
#[must_use]
pub fn lower_bound_by<T, F>(values: &[T], mut compare: F) -> usize
where
    F: FnMut(&T) -> Ordering,
{
    values.partition_point(|value| compare(value) == Ordering::Less)
}

/// Finds the first index whose value is greater than `target`.
///
/// Equivalently, this returns the first index `i` for which
/// `values[i] > target`, or `values.len()` if no value is greater.
///
/// The input must be sorted in ascending order.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::searching::upper_bound;
///
/// assert_eq!(upper_bound(&[1, 2, 2, 2, 5], &2), 4);
/// assert_eq!(upper_bound(&[1, 2, 5], &3), 2);
/// assert_eq!(upper_bound(&[1, 2, 5], &9), 3);
/// ```
#[must_use]
pub fn upper_bound<T: Ord>(values: &[T], target: &T) -> usize {
    upper_bound_by(values, |value| value.cmp(target))
}

/// Comparator-based upper bound.
///
/// `compare(element)` must return `Ordering::Greater` exactly for elements
/// that follow the target. The slice must be sorted according to that
/// comparison.
///
/// Returns the first position where the comparison is `Greater`.
#[must_use]
pub fn upper_bound_by<T, F>(values: &[T], mut compare: F) -> usize
where
    F: FnMut(&T) -> Ordering,
{
    values.partition_point(|value| compare(value) != Ordering::Greater)
}

#[cfg(test)]
mod tests {
    use super::{
        binary_search, binary_search_by, binary_search_by_key, find_iter,
        linear_search, linear_search_by, linear_search_by_key, lower_bound,
        lower_bound_by, upper_bound, upper_bound_by,
    };
    use core::cmp::Ordering;

    #[test]
    fn linear_search_handles_empty_and_single_element_slices() {
        assert_eq!(linear_search::<i32>(&[], &1), None);
        assert_eq!(linear_search(&[1], &1), Some(0));
        assert_eq!(linear_search(&[1], &2), None);
    }

    #[test]
    fn linear_search_returns_first_duplicate() {
        assert_eq!(linear_search(&[4, 2, 4, 4], &4), Some(0));
        assert_eq!(linear_search(&[1, 2, 3], &9), None);
    }

    #[test]
    fn linear_search_by_uses_the_first_matching_predicate() {
        let values = [2, 4, 7, 9, 11];
        assert_eq!(linear_search_by(&values, |value| value % 2 != 0), Some(2));
        assert_eq!(linear_search_by(&values, |_| false), None);
    }

    #[test]
    fn linear_search_by_key_finds_first_matching_key() {
        let records = [("a", 4), ("b", 7), ("c", 4)];
        assert_eq!(linear_search_by_key(&records, &4, |record| record.1), Some(0));
        assert_eq!(linear_search_by_key(&records, &8, |record| record.1), None);
    }

    #[test]
    fn iterator_search_stops_at_the_first_match() {
        let mut visited = Vec::new();

        let found = find_iter(0..10, |value| {
            visited.push(*value);
            *value == 3
        });

        assert_eq!(found, Some(3));
        assert_eq!(visited, [0, 1, 2, 3]);
    }

    #[test]
    fn iterator_search_handles_no_match() {
        assert_eq!(find_iter([1, 3, 5], |value| value == &4), None);
    }

    #[test]
    fn binary_search_returns_match_or_insertion_index() {
        let values = [1, 3, 5, 7, 9];

        assert_eq!(binary_search(&values, &1), Ok(0));
        assert_eq!(binary_search(&values, &9), Ok(4));
        assert_eq!(binary_search(&values, &6), Err(3));
        assert_eq!(binary_search::<i32>(&[], &6), Err(0));
    }

    #[test]
    fn binary_search_by_uses_element_to_target_ordering() {
        let values = [10, 20, 30, 40];

        assert_eq!(binary_search_by(&values, |value| value.cmp(&30)), Ok(2));
        assert_eq!(binary_search_by(&values, |value| value.cmp(&25)), Err(2));
    }

    #[test]
    fn binary_search_by_key_searches_sorted_records() {
        let records = [("a", 1), ("b", 4), ("c", 9)];

        assert_eq!(binary_search_by_key(&records, &4, |record| record.1), Ok(1));
        assert_eq!(binary_search_by_key(&records, &5, |record| record.1), Err(2));
    }

    #[test]
    fn bounds_cover_duplicates_and_missing_values() {
        let values = [1, 2, 2, 2, 5];

        assert_eq!(lower_bound(&values, &2), 1);
        assert_eq!(upper_bound(&values, &2), 4);
        assert_eq!(lower_bound(&values, &3), 4);
        assert_eq!(upper_bound(&values, &3), 4);
    }

    #[test]
    fn bounds_handle_empty_and_extreme_positions() {
        let empty: [i32; 0] = [];

        assert_eq!(lower_bound(&empty, &0), 0);
        assert_eq!(upper_bound(&empty, &0), 0);
        assert_eq!(lower_bound(&[2, 4, 6], &0), 0);
        assert_eq!(upper_bound(&[2, 4, 6], &9), 3);
    }

    #[test]
    fn comparator_bounds_respect_ordering_direction() {
        let values = [1, 2, 2, 2, 7];

        assert_eq!(
            lower_bound_by(&values, |value| value.cmp(&2)),
            1
        );
        assert_eq!(
            upper_bound_by(&values, |value| value.cmp(&2)),
            4
        );
        assert_eq!(
            lower_bound_by(&values, |value| value.cmp(&6)),
            4
        );

        // Comparator semantics are element-to-target, not target-to-element.
        assert_eq!(Ordering::Less, 1.cmp(&2));
    }

    #[test]
    fn sorted_search_does_not_mutate_the_input() {
        let values = [1, 2, 2, 5, 8];
        let original = values;

        let _ = binary_search(&values, &5);
        let _ = lower_bound(&values, &2);
        let _ = upper_bound(&values, &2);

        assert_eq!(values, original);
    }

    #[test]
    fn lower_and_upper_bounds_define_the_equal_range() {
        let values = [1, 2, 2, 2, 3, 5];

        let start = lower_bound(&values, &2);
        let end = upper_bound(&values, &2);

        assert_eq!(&values[start..end], &[2, 2, 2]);
    }

    #[test]
    fn large_search_has_no_application_defined_size_ceiling() {
        let values: Vec<usize> = (0..100_000).collect();

        assert_eq!(linear_search(&values, &99_999), Some(99_999));
        assert_eq!(binary_search(&values, &99_999), Ok(99_999));
        assert_eq!(lower_bound(&values, &50_000), 50_000);
        assert_eq!(upper_bound(&values, &50_000), 50_001);
    }
}
