
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Iteration
//!
//! Canonical path: `src/stdlib/foundation/iteration.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module provides the standard-library foundation for generic,
//! collection-independent iteration.
//!
//! It builds on Rust's established iterator traits and adapters rather
//! than introducing a competing iterator protocol.
//!
//! Its additional functionality is fallible materialization into a
//! dynamically sized `Vec<T>`.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - `IterationError`;
//! - `FallibleIteratorExt`;
//! - `try_collect_vec`;
//! - `try_collect_vec_with_capacity`;
//! - the allocation-failure and partial-progress contract for these
//!   materialization operations;
//! - the standard-library public re-exports of iterator primitives.
//!
//! This module does not own:
//!
//! - source-language iteration semantics;
//! - AST or semantic representations;
//! - collection storage implementations;
//! - tensor storage or tensor iteration semantics;
//! - asynchronous execution;
//! - dataset streaming;
//! - network streaming;
//! - distributed communication;
//! - parallel scheduling;
//! - hardware discovery or selection;
//! - resource allocation policy.
//!
//! ## POCO-REAF and scalability
//!
//! No universal maximum iterator length, number of items, number of
//! iterators, or number of computational domains is imposed here.
//!
//! Lazy iterators can represent potentially unbounded sequences without
//! materializing all their elements.
//!
//! Materializing a sequence into `Vec<T>` necessarily requires finite
//! addressable memory and is subject to:
//!
//! - the host `usize` representation;
//! - the allocator;
//! - element layout;
//! - available memory;
//! - operating-system and process constraints.
//!
//! These are implementation and resource constraints, not language-level
//! limits.
//!
//! This module does not promise literal infinite memory or guarantee that
//! an infinite iterator can be completely collected.
//!
//! Callers should retain lazy iteration when full materialization is not
//! required.
//!
//! ## Iterator semantics
//!
//! Ordinary operations such as:
//!
//! - `map`;
//! - `filter`;
//! - `filter_map`;
//! - `flat_map`;
//! - `flatten`;
//! - `enumerate`;
//! - `zip`;
//! - `chain`;
//! - `fold`;
//! - `try_fold`;
//! - `for_each`;
//! - `try_for_each`;
//! - `take`;
//! - `skip`;
//! - `inspect`;
//!
//! remain owned by Rust's standard `Iterator` trait.
//!
//! This module does not redefine their behavior.
//!
//! ## Allocation behavior
//!
//! Fallible materialization uses `Vec::try_reserve`.
//!
//! The iterator's lower size hint is treated only as an initial reservation
//! request. Its upper size hint is never treated as a semantic limit.
//!
//! Subsequent growth is also fallible.
//!
//! An allocation failure is reported instead of silently returning a
//! truncated collection.
//!
//! ## Failure and side effects
//!
//! The input iterator is consumed up to the point of allocation failure.
//!
//! The partially constructed vector is dropped when an error is returned.
//!
//! Iterator side effects that have already occurred cannot be rolled back.
//!
//! This behavior is intentional and documented.
//!
//! ## Integration
//!
//! The canonical implementation is exposed through:
//!
//! `crate::stdlib::foundation::iteration`
//!
//! The former classical path:
//!
//! `crate::classical::collections::iterator`
//!
//! becomes a compatibility facade re-exporting this implementation.
//!
//! Existing collection types continue to provide their own `iter()`,
//! `iter_mut()`, and `IntoIterator` implementations.
//!
//! The standard library does not require collections to depend on this
//! module or to implement a second iterator protocol.
//!
//! ## Grammar boundary
//!
//! Source syntax remains owned by:
//!
//! - `grammar/statements/loops.g4`;
//! - `grammar/functions/generators.g4`;
//! - `grammar/networking/streaming.g4`;
//! - the canonical expression and semantic specifications.
//!
//! This module does not parse source syntax, define generator semantics,
//! or construct AST or IR nodes.
//!
//! ## Freeze criteria
//!
//! This module is complete when:
//!
//! - standard iterator traits are reused rather than duplicated;
//! - fallible vector materialization reports allocation failure;
//! - no arbitrary item-count ceiling exists;
//! - lazy iterators remain lazy until consumed;
//! - iterator order and duplicates are preserved;
//! - non-Clone items are supported;
//! - borrowed items are supported;
//! - the old classical API remains available through re-exports;
//! - no unsafe code is introduced;
//! - all tests pass under the repository's supported Rust toolchain.

use core::fmt;
use std::collections::TryReserveError;

// ============================================================================
// STANDARD ITERATOR INTERFACES
// ============================================================================

/// Standard Rust iterator traits, adapters, and constructors.
///
/// These are re-exported for a consistent Zamani standard-library import
/// path. Their semantics remain those of Rust's standard library.
pub use core::iter::{
    empty,
    from_fn,
    once,
    once_with,
    repeat,
    repeat_with,
    successors,
    Chain,
    Cloned,
    Copied,
    Cycle,
    DoubleEndedIterator,
    Enumerate,
    ExactSizeIterator,
    Extend,
    Filter,
    FilterMap,
    FlatMap,
    Flatten,
    FromIterator,
    FusedIterator,
    Fuse,
    Inspect,
    IntoIterator,
    Iterator,
    Map,
    MapWhile,
    Once,
    OnceWith,
    Peekable,
    Repeat,
    RepeatWith,
    Rev,
    Scan,
    Skip,
    SkipWhile,
    StepBy,
    Successors,
    Take,
    TakeWhile,
    Zip,
};

// ============================================================================
// ERRORS
// ============================================================================

/// Describes a failure while materializing an iterator into a vector.
///
/// The iterator may already have been partially consumed when this error
/// is returned. Its side effects cannot generally be rolled back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IterationError {
    /// The vector could not reserve the requested additional capacity.
    AllocationFailed {
        /// Additional elements requested by the failed reservation.
        requested_additional: usize,

        /// Number of elements successfully collected before the failure.
        ///
        /// The partially constructed vector is dropped when the error
        /// is returned. This field is diagnostic information and is not
        /// a recovery handle.
        items_collected: usize,
    },
}

impl IterationError {
    /// Returns the number of elements collected before the failure.
    #[inline]
    #[must_use]
    pub const fn items_collected(&self) -> usize {
        match self {
            Self::AllocationFailed {
                items_collected,
                ..
            } => *items_collected,
        }
    }

    /// Returns the additional capacity requested by the failed reservation.
    #[inline]
    #[must_use]
    pub const fn requested_additional(&self) -> usize {
        match self {
            Self::AllocationFailed {
                requested_additional,
                ..
            } => *requested_additional,
        }
    }
}

impl fmt::Display for IterationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationFailed {
                requested_additional,
                items_collected,
            } => write!(
                formatter,
                "iterator materialization allocation failed: \
                 requested {requested_additional} additional elements \
                 after collecting {items_collected} elements"
            ),
        }
    }
}

impl std::error::Error for IterationError {}

// ============================================================================
// FALLIBLE ITERATOR EXTENSION
// ============================================================================

/// Fallible materialization operations for standard Rust iterators.
///
/// Ordinary iteration operations remain provided by Rust's `Iterator`
/// trait. This extension adds only operations requiring the additional
/// allocation-failure contract defined by Zamani.
///
/// Every type implementing `Iterator` automatically receives this
/// extension when the trait is in scope.
///
/// # Example
///
/// ```
/// use zamani::stdlib::foundation::iteration::FallibleIteratorExt;
///
/// let values = (0_u64..10)
///     .map(|value| value * 2)
///     .try_collect_vec()
///     .expect("collection should succeed");
///
/// assert_eq!(values, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
/// ```
///
/// Replace `zamani` with the actual crate name if it differs in the
/// consuming package.
pub trait FallibleIteratorExt: Iterator {
    /// Fallibly collects this iterator into a dynamically sized vector.
    ///
    /// This operation does not impose a fixed maximum item count.
    ///
    /// It uses the iterator's lower size hint as an initial reservation
    /// request and grows the vector as more items are yielded.
    ///
    /// The upper size hint is never treated as a maximum.
    ///
    /// # Ordering
    ///
    /// Items are appended in the exact order yielded by the iterator.
    /// Duplicate items are preserved.
    ///
    /// # Allocation
    ///
    /// Every required capacity reservation is fallible.
    ///
    /// An allocation failure returns [`IterationError::AllocationFailed`].
    /// It never silently truncates the result.
    ///
    /// # Failure and side effects
    ///
    /// The iterator is consumed up to the allocation failure.
    ///
    /// The partially collected vector is dropped. Any side effects that
    /// occurred while producing items remain observable.
    ///
    /// # Complexity
    ///
    /// For `n` successfully collected elements, traversal takes O(n) time,
    /// excluding the cost of producing each item.
    ///
    /// Amortized vector growth is O(n) total.
    ///
    /// # Infinite iterators
    ///
    /// An infinite iterator cannot be completely materialized.
    ///
    /// Use a lazy adapter such as `take` when a finite prefix is required.
    ///
    /// # Examples
    ///
    /// ```
    /// use zamani::stdlib::foundation::iteration::FallibleIteratorExt;
    ///
    /// let values = (0..)
    ///     .take(5)
    ///     .try_collect_vec()
    ///     .expect("finite prefix should collect");
    ///
    /// assert_eq!(values, vec![0, 1, 2, 3, 4]);
    /// ```
    fn try_collect_vec(self) -> Result<Vec<Self::Item>, IterationError>
    where
        Self: Sized,
    {
        self.try_collect_vec_with_capacity(0)
    }

    /// Fallibly collects this iterator into a vector using an initial
    /// reservation request.
    ///
    /// `initial_capacity` is a reservation hint, not a maximum length.
    /// The resulting vector continues growing as required by the iterator.
    ///
    /// The iterator's lower size hint may request a larger initial
    /// reservation than `initial_capacity`.
    ///
    /// Neither the supplied value nor the iterator's size hint creates
    /// a language-level or collection-level item limit.
    ///
    /// # Errors
    ///
    /// Returns [`IterationError::AllocationFailed`] when an initial or
    /// subsequent capacity reservation cannot be satisfied.
    ///
    /// # Failure behavior
    ///
    /// The iterator may be partially consumed before failure. Its side
    /// effects cannot be rolled back.
    fn try_collect_vec_with_capacity(
        self,
        initial_capacity: usize,
    ) -> Result<Vec<Self::Item>, IterationError>
    where
        Self: Sized,
    {
        let mut iterator = self;

        // The lower size hint is a valid minimum according to the Iterator
        // contract. The upper hint is deliberately not used as a limit.
        let (lower_bound, _) = iterator.size_hint();

        // The caller's requested capacity is a reservation hint only.
        // It does not restrict subsequent vector growth.
        let initial_reservation = initial_capacity.max(lower_bound);

        let mut collected = Vec::new();

        collected
            .try_reserve(initial_reservation)
            .map_err(|_| IterationError::AllocationFailed {
                requested_additional: initial_reservation,
                items_collected: 0,
            })?;

        for item in iterator.by_ref() {
            // Reserve before pushing so that this operation can report
            // capacity failures instead of relying on an infallible
            // allocation inside Vec::push.
            collected
                .try_reserve(1)
                .map_err(|_| IterationError::AllocationFailed {
                    requested_additional: 1,
                    items_collected: collected.len(),
                })?;

            collected.push(item);
        }

        Ok(collected)
    }
}

// Every standard iterator receives the extension automatically.
//
// No per-collection implementation is required.
impl<I> FallibleIteratorExt for I
where
    I: Iterator,
{
}

// ============================================================================
// FREE-FUNCTION API
// ============================================================================

/// Fallibly collects any `IntoIterator` into a dynamically sized vector.
///
/// This function provides the same allocation and failure semantics as
/// [`FallibleIteratorExt::try_collect_vec`].
///
/// It is useful when importing an extension trait into the surrounding
/// scope is undesirable.
///
/// # Examples
///
/// ```
/// use zamani::stdlib::foundation::iteration::try_collect_vec;
///
/// let values = try_collect_vec([3, 1, 4, 1, 5])
///     .expect("collection should succeed");
///
/// assert_eq!(values, vec![3, 1, 4, 1, 5]);
/// ```
pub fn try_collect_vec<I>(
    iterator: I,
) -> Result<Vec<I::Item>, IterationError>
where
    I: IntoIterator,
{
    iterator.into_iter().try_collect_vec()
}

/// Fallibly collects any `IntoIterator` into a vector with an initial
/// reservation request.
///
/// The requested capacity is not a maximum. The vector can continue
/// growing as required by the iterator and available resources.
///
/// # Errors
///
/// Returns [`IterationError::AllocationFailed`] when the initial or
/// subsequent allocation reservation fails.
pub fn try_collect_vec_with_capacity<I>(
    iterator: I,
    initial_capacity: usize,
) -> Result<Vec<I::Item>, IterationError>
where
    I: IntoIterator,
{
    iterator
        .into_iter()
        .try_collect_vec_with_capacity(initial_capacity)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{
        try_collect_vec,
        try_collect_vec_with_capacity,
        FallibleIteratorExt,
        IterationError,
    };

    #[test]
    fn collects_empty_iterator() {
        let result: Vec<u8> = core::iter::empty()
            .try_collect_vec()
            .expect("empty collection should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn collects_finite_iterator_in_order() {
        let result = (0..5)
            .try_collect_vec()
            .expect("small collection should succeed");

        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn preserves_duplicate_items() {
        let result = [4, 4, 2, 4]
            .into_iter()
            .try_collect_vec()
            .expect("collection should succeed");

        assert_eq!(result, vec![4, 4, 2, 4]);
    }

    #[test]
    fn supports_lazy_iterator_adapters() {
        let result = (0..20)
            .filter(|value| value % 2 == 0)
            .map(|value| value * 3)
            .try_collect_vec()
            .expect("adapted collection should succeed");

        assert_eq!(
            result,
            vec![0, 6, 12, 18, 24, 30, 36, 42, 48, 54]
        );
    }

    #[test]
    fn supports_borrowed_items() {
        let source = [10, 20, 30];

        let result = source
            .iter()
            .try_collect_vec()
            .expect("borrowed items should collect");

        assert_eq!(result, vec![&10, &20, &30]);
    }

    #[test]
    fn supports_non_clone_items() {
        struct NonClone(u32);

        let result = (1..=3)
            .map(NonClone)
            .try_collect_vec()
            .expect("non-Clone items should collect");

        assert_eq!(
            result.iter().map(|item| item.0).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn free_function_uses_the_same_contract() {
        let result = try_collect_vec(["a", "b", "c"])
            .expect("collection should succeed");

        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn initial_capacity_is_not_a_maximum() {
        let result = try_collect_vec_with_capacity(0..100, 1)
            .expect("vector must grow beyond its initial reservation");

        assert_eq!(result.len(), 100);
        assert_eq!(result.first(), Some(&0));
        assert_eq!(result.last(), Some(&99));
    }

    #[test]
    fn supports_iterators_without_an_upper_size_hint() {
        struct UnknownUpper {
            next: usize,
            end: usize,
        }

        impl Iterator for UnknownUpper {
            type Item = usize;

            fn next(&mut self) -> Option<Self::Item> {
                if self.next >= self.end {
                    return None;
                }

                let value = self.next;
                self.next += 1;

                Some(value)
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (0, None)
            }
        }

        let result = UnknownUpper {
            next: 0,
            end: 100,
        }
        .try_collect_vec()
        .expect("unknown upper bound must not impose a limit");

        assert_eq!(result.len(), 100);
        assert_eq!(result.first(), Some(&0));
        assert_eq!(result.last(), Some(&99));
    }

    #[test]
    fn supports_a_finite_prefix_of_an_infinite_iterator() {
        let result = core::iter::repeat(7_u64)
            .take(64)
            .try_collect_vec()
            .expect("finite prefix should collect");

        assert_eq!(result.len(), 64);
        assert!(result.iter().all(|value| *value == 7));
    }

    #[test]
    fn error_diagnostic_accessors_are_consistent() {
        let error = IterationError::AllocationFailed {
            requested_additional: 8,
            items_collected: 21,
        };

        assert_eq!(error.requested_additional(), 8);
        assert_eq!(error.items_collected(), 21);
        assert!(error.to_string().contains("21 elements"));
    }

    #[test]
    fn error_implements_standard_error() {
        fn accepts_error(_: &dyn std::error::Error) {}

        let error = IterationError::AllocationFailed {
            requested_additional: 1,
            items_collected: 0,
        };

        accepts_error(&error);
    }
}
