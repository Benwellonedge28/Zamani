
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Partitioning Algorithms
//!
//! Path: `src/classical/algorithms/partitioning.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! Status: Proposed implementation. Repository-wide compilation and
//! integration verification are required before production release.
//!
//! # Purpose
//!
//! Provide generic partitioning primitives for classical computation:
//!
//! - in-place, unstable two-way partitioning by predicate;
//! - in-place, stable two-way partitioning by predicate;
//! - balanced logical partition-range planning without allocating a vector
//!   of ranges;
//! - inspection of the resulting plan's input length and partition count.
//!
//! These primitives distinguish the physical reordering of elements from
//! the logical division of an index domain into ranges.
//!
//! # Authority and ownership
//!
//! - `grammar/spec/classical.md` owns normative classical-computing semantics.
//! - `grammar/spec/data.md` owns normative data, ordering, scalability,
//!   determinism, and partitioning semantics.
//! - `grammar/concurrency/data-parallel.g4` owns data-parallel source syntax,
//!   including the source-level partition operation category.
//! - The frontend owns parsing, AST construction, and source locations.
//! - Semantic analysis owns type checking, callable resolution, effects,
//!   legality, and source-level determinism requirements.
//! - Existing collection modules own array, vector, tensor, and other
//!   collection representations and invariants.
//! - The repository's canonical IR owns semantic operation representation.
//! - Execution and resource systems own physical placement, scheduling,
//!   worker selection, budgets, cancellation, and resource failures.
//!
//! This module owns only generic sequential partition algorithms and
//! allocation-free logical range planning. It does not own source syntax,
//! collection types, a new IR, hardware discovery, or a scheduler.
//!
//! # Public API
//!
//! - `partition_by`: moves predicate-matching elements to the prefix.
//! - `stable_partition_by`: moves predicate-matching elements to the prefix
//!   while preserving the relative order of both groups.
//! - `PartitionPlan`: describes a balanced division of an index domain.
//! - `PartitionRanges`: lazily yields the plan's non-overlapping ranges.
//! - `PartitionPlanError`: reports invalid plan construction arguments.
//!
//! # In-memory partition semantics
//!
//! Both in-memory functions mutate the supplied slice and return a boundary
//! index `b` such that:
//!
//! - indices in `0..b` satisfy the predicate;
//! - indices in `b..values.len()` do not satisfy the predicate.
//!
//! `partition_by` is unstable: it may change the relative order of elements
//! within either group.
//!
//! `stable_partition_by` preserves the relative order within both groups.
//! It uses in-place rotations rather than an auxiliary element buffer.
//! This avoids scratch-buffer allocation but can require O(n^2) element
//! movements in the worst case.
//!
//! Predicates may be called multiple times. A deterministic predicate is
//! required for predictable results. If a predicate panics, the operation
//! may already have rearranged part or all of the slice; rollback is not
//! promised.
//!
//! # Logical range semantics
//!
//! `PartitionPlan::new(length, requested_partitions)` divides the logical
//! index domain `0..length` into at most the requested number of non-empty
//! ranges.
//!
//! A request of zero partitions is rejected. For an empty domain and a
//! positive request, the plan contains zero ranges. For a non-empty domain,
//! the effective partition count is the smaller of the requested count and
//! the domain length.
//!
//! Ranges are contiguous, non-overlapping, ordered, and collectively cover
//! exactly `0..length`. Their lengths differ by at most one. Any remainder
//! elements are assigned to the earliest ranges.
//!
//! This is a logical partition plan, not a promise that any partition will
//! execute concurrently or on a separate device.
//!
//! # Scalability and resources
//!
//! This module introduces no application-defined maximum for input length,
//! partition count, worker count, device count, or hardware topology.
//!
//! Logical range generation is lazy and does not allocate a collection of
//! ranges. The caller can iterate over the plan and dispatch ranges according
//! to its own execution policy.
//!
//! The representation is necessarily bounded by Rust's `usize` and slice
//! indexing model. Actual work is bounded by finite compute and memory
//! resources; this module does not promise infinite resources or execution.
//!
//! Operational limits, cancellation, placement, and resource exhaustion
//! belong to shared execution infrastructure, not to these algorithms.
//!
//! # Complexity
//!
//! - `partition_by`: O(n) predicate evaluations in the usual case,
//!   O(1) auxiliary algorithm storage.
//! - `stable_partition_by`: O(n) predicate evaluations and up to O(n^2)
//!   element movements; O(1) auxiliary algorithm storage.
//! - `PartitionPlan::new`: O(1) time and O(1) storage.
//! - Iterating `PartitionRanges`: O(1) work per emitted range and O(1)
//!   storage for the iterator.
//!
//! # Integration contract
//!
//! `src/classical/algorithms/mod.rs` should declare `pub mod partitioning;`.
//! It must register the public module without duplicating the ownership of
//! existing reduction, scan, sorting, or searching algorithms.
//!
//! Existing `src/classical/collections/vector.rs` and `array.rs` may adapt
//! their mutable slices to the in-memory partition functions when compatible
//! with their collection and mutation contracts. They must not duplicate
//! the underlying collection representation.
//!
//! The frontend and semantic-analysis layers must validate source operations
//! and callable effects. Lowering must map validated source intent into the
//! existing canonical IR rather than introduce partition-specific IR types.
//!
//! The execution layer may consume `PartitionPlan` to schedule logical
//! ranges. It remains responsible for selecting sequential, parallel,
//! distributed, or heterogeneous execution and for respecting capabilities,
//! resource budgets, cancellation, and required determinism.
//!
//! # Freeze rule
//!
//! Changing predicate-group semantics, stable-order guarantees, range
//! coverage, partition-count behavior, or remainder distribution requires
//! a compatibility review and regression tests.

use core::ops::Range;

/// An error raised while constructing a [`PartitionPlan`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartitionPlanError {
    /// A plan must request at least one partition.
    ///
    /// Empty input is still valid when the requested count is positive.
    ZeroPartitions,
}

impl core::fmt::Display for PartitionPlanError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ZeroPartitions => {
                formatter.write_str("partition count must be greater than zero")
            }
        }
    }
}

impl std::error::Error for PartitionPlanError {}

/// Partitions a mutable slice in place, without preserving element order.
///
/// On return, every element in `values[..boundary]` satisfies `predicate`,
/// and every element in `values[boundary..]` does not.
///
/// Returns the first index of the non-matching suffix. The input is not
/// required to be sorted.
///
/// This operation does not allocate an auxiliary element buffer.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::partitioning::partition_by;
///
/// let mut values = [1, 2, 3, 4, 5, 6];
/// let boundary = partition_by(&mut values, |value| value % 2 == 0);
///
/// assert!(values[..boundary].iter().all(|value| value % 2 == 0));
/// assert!(values[boundary..].iter().all(|value| value % 2 != 0));
/// assert_eq!(boundary, 3);
/// ```
pub fn partition_by<T, F>(values: &mut [T], mut predicate: F) -> usize
where
    F: FnMut(&T) -> bool,
{
    let mut left = 0;
    let mut right = values.len();

    loop {
        while left < right && predicate(&values[left]) {
            left += 1;
        }

        while left < right && !predicate(&values[right - 1]) {
            right -= 1;
        }

        if left >= right {
            return left;
        }

        values.swap(left, right - 1);
        left += 1;
        right -= 1;
    }
}

/// Stably partitions a mutable slice in place.
///
/// On return, every element in `values[..boundary]` satisfies `predicate`,
/// and every element in `values[boundary..]` does not. The relative order of
/// elements within each group is preserved.
///
/// This implementation uses slice rotations and does not allocate an
/// auxiliary element buffer. In exchange, worst-case element movement is
/// O(n^2); use [`partition_by`] when stability is unnecessary.
///
/// If the predicate panics, the input may have been partially rearranged.
/// Rollback is not guaranteed.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::partitioning::stable_partition_by;
///
/// let mut values = [("a", false), ("b", true), ("c", false), ("d", true)];
/// let boundary = stable_partition_by(&mut values, |item| item.1);
///
/// assert_eq!(&values[..boundary], &[("b", true), ("d", true)]);
/// assert_eq!(&values[boundary..], &[("a", false), ("c", false)]);
/// ```
pub fn stable_partition_by<T, F>(values: &mut [T], mut predicate: F) -> usize
where
    F: FnMut(&T) -> bool,
{
    let mut boundary = 0;

    for current in 0..values.len() {
        if predicate(&values[current]) {
            if current != boundary {
                // Move this matching item to the end of the matching prefix.
                // Rotation preserves the order of all intervening elements.
                values[boundary..=current].rotate_right(1);
            }

            boundary += 1;
        }
    }

    boundary
}

/// A balanced logical partition plan over an index domain.
///
/// The plan divides `0..length` into non-empty, ordered ranges. It does not
/// allocate a list of those ranges, select workers, or execute any work.
///
/// Use [`PartitionPlan::ranges`] to lazily iterate over the ranges.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::partitioning::PartitionPlan;
///
/// let plan = PartitionPlan::new(10, 3).expect("positive partition count");
/// let ranges: Vec<_> = plan.ranges().collect();
///
/// assert_eq!(ranges, [0..4, 4..7, 7..10]);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartitionPlan {
    length: usize,
    partition_count: usize,
    base_length: usize,
    remainder: usize,
}

impl PartitionPlan {
    /// Creates a balanced plan for a logical domain of `length` items.
    ///
    /// `requested_partitions` must be positive. If `length` is zero, the
    /// resulting plan contains no ranges. Otherwise, the actual partition
    /// count is `min(length, requested_partitions)`, so no empty ranges are
    /// created.
    ///
    /// The first `remainder` ranges contain `base_length + 1` items; any
    /// remaining ranges contain `base_length` items.
    ///
    /// # Errors
    ///
    /// Returns [`PartitionPlanError::ZeroPartitions`] when the requested
    /// partition count is zero.
    pub fn new(
        length: usize,
        requested_partitions: usize,
    ) -> Result<Self, PartitionPlanError> {
        if requested_partitions == 0 {
            return Err(PartitionPlanError::ZeroPartitions);
        }

        let partition_count = requested_partitions.min(length);

        if partition_count == 0 {
            return Ok(Self {
                length,
                partition_count: 0,
                base_length: 0,
                remainder: 0,
            });
        }

        Ok(Self {
            length,
            partition_count,
            base_length: length / partition_count,
            remainder: length % partition_count,
        })
    }

    /// Returns the length of the logical index domain.
    #[must_use]
    pub const fn length(&self) -> usize {
        self.length
    }

    /// Returns the number of non-empty ranges in this plan.
    #[must_use]
    pub const fn partition_count(&self) -> usize {
        self.partition_count
    }

    /// Returns `true` if the logical domain contains no items.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns a lazy iterator over the plan's ranges.
    ///
    /// The iterator allocates no range collection. Every yielded range is
    /// non-empty, within bounds, and adjacent to the previous range.
    #[must_use]
    pub const fn ranges(&self) -> PartitionRanges {
        PartitionRanges {
            next_start: 0,
            next_index: 0,
            remaining: self.partition_count,
            base_length: self.base_length,
            remainder: self.remainder,
        }
    }
}

/// A lazy iterator over the ranges in a [`PartitionPlan`].
///
/// This iterator stores only counters and the balanced-range parameters.
/// Its memory use does not grow with the number of partitions.
#[derive(Clone, Debug)]
pub struct PartitionRanges {
    next_start: usize,
    next_index: usize,
    remaining: usize,
    base_length: usize,
    remainder: usize,
}

impl Iterator for PartitionRanges {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let width = self.base_length + usize::from(self.next_index < self.remainder);
        let start = self.next_start;
        let end = start + width;

        self.next_start = end;
        self.next_index += 1;
        self.remaining -= 1;

        Some(start..end)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for PartitionRanges {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl core::iter::FusedIterator for PartitionRanges {}

#[cfg(test)]
mod tests {
    use super::{
        partition_by, stable_partition_by, PartitionPlan, PartitionPlanError,
    };

    #[test]
    fn unstable_partition_handles_empty_and_single_element_inputs() {
        let mut empty: [i32; 0] = [];
        assert_eq!(partition_by(&mut empty, |value| *value > 0), 0);

        let mut matching = [7];
        assert_eq!(partition_by(&mut matching, |value| *value > 0), 1);
        assert_eq!(matching, [7]);

        let mut non_matching = [-7];
        assert_eq!(partition_by(&mut non_matching, |value| *value > 0), 0);
        assert_eq!(non_matching, [-7]);
    }

    #[test]
    fn unstable_partition_places_matching_elements_in_prefix() {
        let mut values = [1, 2, 3, 4, 5, 6, 7, 8];
        let original_len = values.len();

        let boundary = partition_by(&mut values, |value| value % 2 == 0);

        assert!(values[..boundary].iter().all(|value| value % 2 == 0));
        assert!(values[boundary..].iter().all(|value| value % 2 != 0));
        assert_eq!(boundary, 4);
        assert_eq!(values.len(), original_len);
    }

    #[test]
    fn unstable_partition_handles_all_matching_and_none_matching() {
        let mut all_matching = [2, 4, 6, 8];
        assert_eq!(
            partition_by(&mut all_matching, |value| value % 2 == 0),
            all_matching.len()
        );

        let mut none_matching = [1, 3, 5, 7];
        assert_eq!(
            partition_by(&mut none_matching, |value| value % 2 == 0),
            0
        );
    }

    #[test]
    fn stable_partition_preserves_both_group_orders() {
        let mut values = [
            ("a", false),
            ("b", true),
            ("c", false),
            ("d", true),
            ("e", false),
            ("f", true),
        ];

        let boundary = stable_partition_by(&mut values, |item| item.1);

        assert_eq!(
            &values[..boundary],
            &[("b", true), ("d", true), ("f", true)]
        );
        assert_eq!(
            &values[boundary..],
            &[("a", false), ("c", false), ("e", false)]
        );
    }

    #[test]
    fn stable_partition_handles_empty_and_uniform_inputs() {
        let mut empty: [i32; 0] = [];
        assert_eq!(stable_partition_by(&mut empty, |value| *value > 0), 0);

        let mut matching = [1, 2, 3];
        assert_eq!(stable_partition_by(&mut matching, |_| true), 3);
        assert_eq!(matching, [1, 2, 3]);

        let mut non_matching = [1, 2, 3];
        assert_eq!(stable_partition_by(&mut non_matching, |_| false), 0);
        assert_eq!(non_matching, [1, 2, 3]);
    }

    #[test]
    fn plan_rejects_zero_requested_partitions() {
        assert_eq!(
            PartitionPlan::new(10, 0),
            Err(PartitionPlanError::ZeroPartitions)
        );
        assert_eq!(
            PartitionPlan::new(0, 0),
            Err(PartitionPlanError::ZeroPartitions)
        );
    }

    #[test]
    fn plan_for_empty_domain_has_no_ranges() {
        let plan = PartitionPlan::new(0, 5).expect("positive request");

        assert!(plan.is_empty());
        assert_eq!(plan.length(), 0);
        assert_eq!(plan.partition_count(), 0);
        assert_eq!(plan.ranges().count(), 0);
    }

    #[test]
    fn plan_reduces_partition_count_when_more_partitions_than_items() {
        let plan = PartitionPlan::new(3, 100).expect("positive request");

        assert_eq!(plan.partition_count(), 3);
        assert_eq!(plan.ranges().collect::<Vec<_>>(), [0..1, 1..2, 2..3]);
    }

    #[test]
    fn plan_distributes_remainder_to_earliest_ranges() {
        let plan = PartitionPlan::new(10, 3).expect("positive request");

        assert_eq!(plan.partition_count(), 3);
        assert_eq!(plan.ranges().collect::<Vec<_>>(), [0..4, 4..7, 7..10]);
    }

    #[test]
    fn plan_ranges_are_contiguous_non_overlapping_and_cover_domain() {
        for length in 0..64 {
            for requested in 1..16 {
                let plan =
                    PartitionPlan::new(length, requested).expect("positive request");
                let ranges: Vec<_> = plan.ranges().collect();

                assert_eq!(ranges.len(), plan.partition_count());

                if length == 0 {
                    assert!(ranges.is_empty());
                    continue;
                }

                assert_eq!(ranges.first().map(|range| range.start), Some(0));
                assert_eq!(ranges.last().map(|range| range.end), Some(length));

                for range in &ranges {
                    assert!(range.start < range.end);
                    assert!(range.end <= length);
                }

                for pair in ranges.windows(2) {
                    assert_eq!(pair[0].end, pair[1].start);
                }

                let shortest = ranges.iter().map(|range| range.len()).min().unwrap();
                let longest = ranges.iter().map(|range| range.len()).max().unwrap();

                assert!(longest - shortest <= 1);
                assert_eq!(ranges.iter().map(|range| range.len()).sum::<usize>(), length);
            }
        }
    }

    #[test]
    fn range_iterator_reports_exact_remaining_length() {
        let plan = PartitionPlan::new(11, 4).expect("positive request");
        let mut ranges = plan.ranges();

        assert_eq!(ranges.len(), 4);
        assert_eq!(ranges.next(), Some(0..3));
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges.next(), Some(3..6));
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges.next(), Some(6..9));
        assert_eq!(ranges.next(), Some(9..11));
        assert_eq!(ranges.len(), 0);
        assert_eq!(ranges.next(), None);
        assert_eq!(ranges.next(), None);
    }

    #[test]
    fn large_logical_domain_requires_no_range_vector() {
        let plan =
            PartitionPlan::new(100_000, 997).expect("positive request");

        assert_eq!(plan.partition_count(), 997);

        let mut total = 0usize;
        let mut last_end = 0usize;

        for range in plan.ranges() {
            assert_eq!(range.start, last_end);
            total += range.len();
            last_end = range.end;
        }

        assert_eq!(total, 100_000);
        assert_eq!(last_end, 100_000);
    }
}
