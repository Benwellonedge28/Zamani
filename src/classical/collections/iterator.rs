
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical collections: iterator utilities.
//!
//! Path: src/classical/collections/iterator.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provide shared, collection-independent utilities for traversing values
//! and fallibly materializing an iterator into a `Vec<T>`.
//!
//! # Ownership
//!
//! This module owns:
//! - `ClassicalIteratorError`, describing fallible collection failures;
//! - `ClassicalIteratorExt`, a convenience extension for standard iterators;
//! - the contract for fallible iterator-to-vector materialization.
//!
//! This module does not own:
//! - iterator semantics defined by Rust's `Iterator` trait;
//! - sequence, vector, array, matrix, tensor, map, or set storage;
//! - source-language iterator syntax or AST nodes;
//! - semantic type resolution, ownership analysis, or lifetimes;
//! - canonical classical values or IR;
//! - execution scheduling, placement, or hardware selection.
//!
//! # Semantics
//!
//! - Traversal follows the input iterator's order.
//! - The input iterator's `size_hint()` is an allocation hint, not a semantic
//!   limit or a promise that the iterator has an exact length.
//! - A successful collection contains every item yielded before exhaustion.
//! - Allocation failure is reported rather than silently truncating output.
//! - On collection failure, the iterator has been consumed up to the failure
//!   point and the partially built vector is dropped.
//! - The iterator may have external side effects; those effects cannot be
//!   rolled back by this utility.
//! - No `Clone`, `Copy`, `Default`, `Send`, or `Sync` bound is imposed on items.
//!
//! # POCO-REAF and scalability
//!
//! This module has no language-level maximum iterator length or item count.
//! A concrete `Vec<T>` is limited by Rust's `usize`, allocator, element layout,
//! and available resources. No finite in-memory representation can promise
//! infinite storage.
//!
//! Lazy iterators can describe streams or very large traversals without
//! materializing every element. Callers should only collect when materialized
//! storage is actually required. Distributed, streaming, or out-of-core
//! execution belongs to the appropriate runtime and execution abstractions.
//!
//! # Integration
//!
//! `collections/mod.rs` should declare `pub mod iterator;` and re-export the
//! public API as shown in the integration instructions accompanying this file.
//!
//! Existing `ClassicalSequence<T>` and `ClassicalVector<T>` iterators remain
//! authoritative. Their `iter()`, `iter_mut()`, and `IntoIterator` methods
//! work with this extension when the trait is in scope.
//!
//! This module does not depend on either collection type, preventing a
//! circular dependency and allowing use with arrays, slices, maps, sets,
//! ranges, and custom iterator implementations.
//!
//! Grammar and semantic analysis define source-language iterator meaning.
//! This Rust module does not parse syntax, define a second type system, or
//! construct classical or quantum IR.
//!
//! # Freeze rule
//!
//! Changes to failure semantics, item-consumption behavior, or public error
//! fields require compatibility review.

use core::fmt;
use std::collections::TryReserveError;

/// Describes a failure while materializing an iterator into a vector.
///
/// The input iterator is consumed by the attempted collection. Because
/// iterators can have arbitrary side effects, the operation cannot generally
/// be rolled back when allocation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalIteratorError {
    /// The vector could not reserve the requested additional capacity.
    AllocationFailed {
        /// Additional elements requested in the failed reservation.
        requested_additional: usize,

        /// Number of elements already collected before the failure.
        ///
        /// The partially collected vector is dropped when this error is
        /// returned; this field is diagnostic information, not a recovery
        /// handle.
        items_collected: usize,
    },
}

impl ClassicalIteratorError {
    /// Returns the number of items successfully appended before failure.
    #[must_use]
    pub const fn items_collected(&self) -> usize {
        match self {
            Self::AllocationFailed {
                items_collected, ..
            } => *items_collected,
        }
    }

    /// Returns the additional capacity requested by the failed reservation.
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

impl fmt::Display for ClassicalIteratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationFailed {
                requested_additional,
                items_collected,
            } => write!(
                f,
                "iterator collection allocation failed while requesting \
                 {requested_additional} additional elements after collecting \
                 {items_collected} elements"
            ),
        }
    }
}

impl std::error::Error for ClassicalIteratorError {}

impl From<TryReserveError> for ClassicalIteratorError {
    fn from(_: TryReserveError) -> Self {
        Self::AllocationFailed {
            requested_additional: 0,
            items_collected: 0,
        }
    }
}

/// Extension methods for standard Rust iterators.
///
/// This trait deliberately leaves ordinary traversal operations—such as
/// `map`, `filter`, `enumerate`, `zip`, `fold`, and `for_each`—to Rust's
/// standard `Iterator` trait. It adds only operations with Zamani-specific
/// fallible materialization semantics.
pub trait ClassicalIteratorExt: Iterator {
    /// Fallibly collects this iterator into a dynamically sized `Vec`.
    ///
    /// The iterator's lower size hint is used only as an initial reservation
    /// request. The upper size hint is never used as a hard limit.
    ///
    /// Each subsequent growth is fallible. If reservation fails, this method
    /// returns `ClassicalIteratorError::AllocationFailed` instead of silently
    /// returning a truncated vector.
    ///
    /// # Failure and side effects
    ///
    /// The iterator is consumed up to the point where allocation fails.
    /// Already collected elements are dropped with the temporary vector, and
    /// the iterator cannot be resumed through this call. Any side effects
    /// caused by yielding items remain observable.
    ///
    /// If the caller needs to preserve partial progress, use an explicit
    /// destination collection and its fallible incremental-extension API
    /// instead.
    ///
    /// # Complexity
    ///
    /// Time is O(n) for n yielded elements, excluding the cost of producing
    /// each item. Amortized vector growth is O(n) total; actual allocation
    /// behavior depends on the allocator and requested element layout.
    fn try_collect_vec(self) -> Result<Vec<Self::Item>, ClassicalIteratorError>
    where
        Self: Sized,
    {
        let mut iterator = self;
        let (lower_bound, _) = iterator.size_hint();

        let mut collected = Vec::new();

        collected
            .try_reserve(lower_bound)
            .map_err(|_| ClassicalIteratorError::AllocationFailed {
                requested_additional: lower_bound,
                items_collected: 0,
            })?;

        for item in iterator.by_ref() {
            collected
                .try_reserve(1)
                .map_err(|_| ClassicalIteratorError::AllocationFailed {
                    requested_additional: 1,
                    items_collected: collected.len(),
                })?;

            collected.push(item);
        }

        Ok(collected)
    }
}

// Every standard iterator receives the extension methods without requiring
// per-collection implementations or duplicated iterator wrapper types.
impl<I> ClassicalIteratorExt for I where I: Iterator {}

/// Converts an iterator into a vector using fallible allocation.
///
/// This free function is useful when importing the extension trait into the
/// surrounding scope is undesirable.
///
/// See [`ClassicalIteratorExt::try_collect_vec`] for full failure semantics.
pub fn try_collect_vec<I>(
    iterator: I,
) -> Result<Vec<I::Item>, ClassicalIteratorError>
where
    I: IntoIterator,
{
    iterator.into_iter().try_collect_vec()
}

#[cfg(test)]
mod tests {
    use super::{try_collect_vec, ClassicalIteratorError, ClassicalIteratorExt};

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
    fn preserves_duplicates() {
        let result = [4, 4, 2, 4]
            .into_iter()
            .try_collect_vec()
            .expect("collection should succeed");

        assert_eq!(result, vec![4, 4, 2, 4]);
    }

    #[test]
    fn supports_lazy_adapters() {
        let result = (0..20)
            .filter(|value| value % 2 == 0)
            .map(|value| value * 3)
            .try_collect_vec()
            .expect("adapted collection should succeed");

        assert_eq!(result, vec![0, 6, 12, 18, 24, 30, 36, 42, 48, 54]);
    }

    #[test]
    fn works_with_borrowed_slices() {
        let source = [10, 20, 30];

        let result = source
            .iter()
            .try_collect_vec()
            .expect("borrowed items should collect");

        assert_eq!(result, vec![&10, &20, &30]);
    }

    #[test]
    fn free_function_uses_the_same_contract() {
        let result = try_collect_vec(["a", "b", "c"])
            .expect("collection should succeed");

        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn supports_non_clone_items() {
        struct NonClone(u32);

        let result = (1..=3)
            .map(NonClone)
            .try_collect_vec()
            .expect("non-clone items should collect");

        assert_eq!(
            result.iter().map(|item| item.0).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn supports_an_iterator_with_no_upper_size_hint() {
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

        let result = UnknownUpper { next: 0, end: 100 }
            .try_collect_vec()
            .expect("unknown upper bound must not impose a limit");

        assert_eq!(result.len(), 100);
        assert_eq!(result.first(), Some(&0));
        assert_eq!(result.last(), Some(&99));
    }

    #[test]
    fn error_diagnostic_accessors_are_consistent() {
        let error = ClassicalIteratorError::AllocationFailed {
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

        let error = ClassicalIteratorError::AllocationFailed {
            requested_additional: 1,
            items_collected: 0,
        };

        accepts_error(&error);
    }
}
