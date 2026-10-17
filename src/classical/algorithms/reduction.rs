
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Reduction Algorithms
//!
//! Path: `src/classical/algorithms/reduction.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! Status: Proposed implementation; repository integration and workspace
//! verification are required before production release.
//!
//! # Purpose
//!
//! Provide generic, target-independent reduction primitives for classical
//! computation. A reduction combines an ordered sequence of input values
//! into a single result.
//!
//! Supported primitives:
//!
//! - `reduce`: reduce a non-empty iterator without an identity value;
//! - `reduce_with`: reduce a non-empty iterator using a combining function;
//! - `reduce_with_identity`: reduce an iterator using an explicit identity;
//! - `try_reduce`: reduce while allowing the combining function to fail;
//! - `try_reduce_with_identity`: fallible reduction with an explicit identity;
//! - `fold`: fold an iterator using an explicit initial accumulator;
//! - `try_fold`: fallible folding with an explicit initial accumulator.
//!
//! The API accepts arbitrary iterator types. It does not allocate a
//! collection, introduce fixed input-size limits, select hardware, or
//! implement its own scheduler.
//!
//! # Architectural authority
//!
//! - `grammar/concurrency/data-parallel.g4` owns `parallel::reduce` syntax.
//! - The frontend AST owns source expressions and their source locations.
//! - Semantic analysis owns type checking, callable resolution, effects,
//!   ownership, purity, determinism, and reducer legality.
//! - Canonical ZUIR owns the semantic operations and their IR representation.
//! - `crate::classical::error` owns the subsystem-wide error vocabulary.
//! - Shared resource and execution systems own budgets, capabilities,
//!   placement, scheduling, and resource-exhaustion policy.
//!
//! This file owns only generic sequential reduction algorithms and their
//! documented execution-order contracts.
//!
//! # Semantics
//!
//! 1. Input is consumed in iterator order.
//! 2. A reduction without an identity returns `None` for an empty iterator.
//! 3. A reduction without an identity returns the sole element unchanged
//!    for a single-element iterator.
//! 4. An identity-based fold returns the initial accumulator for empty input.
//! 5. Combining functions are called in left-to-right fold order.
//! 6. Combining functions are not reassociated or parallelized here.
//! 7. A combining-function error stops the fold and is returned to the caller.
//! 8. Iteration is lazy until a reduction function consumes the iterator.
//! 9. The iterator's own behavior determines whether traversal can terminate.
//! 10. This module does not guarantee rollback of external side effects.
//!
//! # Parallelism and numerical correctness
//!
//! Associativity, commutativity, purity, and determinism are not assumed.
//! Floating-point addition, subtraction, and arbitrary user-defined
//! operations can produce different results when reordered.
//!
//! Parallel reduction belongs in the data-parallel execution layer, which
//! must obtain the relevant semantic guarantees before choosing a tree,
//! partitioning the input, or combining partial results.
//!
//! # Resource contract
//!
//! The algorithms do not materialize the input into a vector or impose an
//! architectural maximum on the number of elements. Memory consumption
//! attributable to these algorithms is constant apart from the iterator,
//! accumulator, and combining function's own behavior.
//!
//! A non-terminating iterator may never complete. Infinite input is not
//! automatically reducible to a finite result. Operational budgets,
//! cancellation, and resource failures belong to the calling execution
//! layer.
//!
//! # Integration
//!
//! - `src/classical/algorithms/mod.rs` declares this module and exports the
//!   intended public API.
//! - `src/classical/algorithms/scan.rs` may reuse compatible combining
//!   semantics, but scan must preserve its distinct prefix-result contract.
//! - `src/classical/numeric/` owns numerical operations and policies.
//! - `src/classical/collections/` owns collection representations.
//! - `src/classical/concurrency/` and `src/classical/execution/` own
//!   parallel execution, scheduling, cancellation, and resource policies.
//! - `src/classical/lowering/` maps validated reduction operations to the
//!   canonical IR without creating a second IR.
//!
//! These are integration contracts, not claims that all listed modules
//! currently exist or are already connected.
//!
//! # Compatibility
//!
//! Public functions are generic over `IntoIterator` and preserve the
//! iterator's element type unless the combining function changes the
//! accumulator type through `fold` or `try_fold`.
//!
//! The module defines no duplicate classical value, type, IR, resource,
//! diagnostic, or identity representation.

/// Reduces a non-empty iterator using the standard [`std::ops::Add`] trait.
///
/// Returns `None` for empty input and the original element for a
/// single-element iterator.
///
/// This operation is sequential and follows iterator order.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::reduction::reduce;
///
/// assert_eq!(reduce([1_i64, 2, 3, 4]), Some(10));
/// assert_eq!(reduce(std::iter::empty::<i64>()), None);
/// ```
pub fn reduce<I>(iterable: I) -> Option<I::Item>
where
    I: IntoIterator,
    I::Item: std::ops::Add<Output = I::Item>,
{
    reduce_with(iterable, |left, right| left + right)
}

/// Reduces a non-empty iterator with a caller-provided combining function.
///
/// The first input element initializes the accumulator. Each subsequent
/// element is combined with the accumulator in iterator order.
///
/// No identity is invented. Empty input returns `None`.
///
/// The combining function may have side effects. It is called only for
/// elements after the first one.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::reduction::reduce_with;
///
/// let result = reduce_with([2_u64, 3, 4], |a, b| a * b);
/// assert_eq!(result, Some(24));
/// ```
pub fn reduce_with<I, F>(iterable: I, mut combine: F) -> Option<I::Item>
where
    I: IntoIterator,
    F: FnMut(I::Item, I::Item) -> I::Item,
{
    let mut iter = iterable.into_iter();
    let mut accumulator = iter.next()?;

    for item in iter {
        accumulator = combine(accumulator, item);
    }

    Some(accumulator)
}

/// Reduces an iterator with an explicit initial accumulator.
///
/// Unlike [`reduce_with`], this function returns the initial accumulator
/// when the iterator is empty.
///
/// This is a left fold. It does not assume that the accumulator is a
/// mathematical identity or that the combining function is associative.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::reduction::reduce_with_identity;
///
/// assert_eq!(
///     reduce_with_identity([1_i64, 2, 3], 0, |a, b| a + b),
///     6
/// );
///
/// assert_eq!(
///     reduce_with_identity(std::iter::empty::<i64>(), 42, |a, b| a + b),
///     42
/// );
/// ```
pub fn reduce_with_identity<I, A, F>(
    iterable: I,
    identity: A,
    mut combine: F,
) -> A
where
    I: IntoIterator,
    F: FnMut(A, I::Item) -> A,
{
    let mut accumulator = identity;

    for item in iterable {
        accumulator = combine(accumulator, item);
    }

    accumulator
}

/// Reduces a non-empty iterator with a fallible combining function.
///
/// The first element initializes the accumulator. If combining any later
/// element fails, the error is returned immediately and no later elements
/// are consumed by this function.
///
/// Errors are generic so callers can preserve their canonical error type
/// without introducing a second subsystem error vocabulary.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::reduction::try_reduce;
///
/// let result = try_reduce([1_i32, 2, 3], |a, b| {
///     a.checked_add(b).ok_or("integer overflow")
/// });
///
/// assert_eq!(result, Ok(Some(6)));
/// ```
pub fn try_reduce<I, F, E>(
    iterable: I,
    mut combine: F,
) -> Result<Option<I::Item>, E>
where
    I: IntoIterator,
    F: FnMut(I::Item, I::Item) -> Result<I::Item, E>,
{
    let mut iter = iterable.into_iter();
    let Some(mut accumulator) = iter.next() else {
        return Ok(None);
    };

    for item in iter {
        accumulator = combine(accumulator, item)?;
    }

    Ok(Some(accumulator))
}

/// Performs a fallible reduction with an explicit initial accumulator.
///
/// The initial accumulator is returned successfully for empty input.
/// Errors from the combining function are propagated without being
/// discarded, converted to strings, or replaced with default values.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::reduction::try_reduce_with_identity;
///
/// let result = try_reduce_with_identity([2_u32, 3, 4], 1_u32, |a, b| {
///     a.checked_mul(b).ok_or("multiplication overflow")
/// });
///
/// assert_eq!(result, Ok(24));
/// ```
pub fn try_reduce_with_identity<I, A, F, E>(
    iterable: I,
    identity: A,
    mut combine: F,
) -> Result<A, E>
where
    I: IntoIterator,
    F: FnMut(A, I::Item) -> Result<A, E>,
{
    let mut accumulator = identity;

    for item in iterable {
        accumulator = combine(accumulator, item)?;
    }

    Ok(accumulator)
}

/// Folds an iterator using a caller-provided accumulator and combining
/// function.
///
/// This is an alias for [`reduce_with_identity`] with terminology familiar
/// to functional programming and compiler implementations.
pub fn fold<I, A, F>(iterable: I, initial: A, combine: F) -> A
where
    I: IntoIterator,
    F: FnMut(A, I::Item) -> A,
{
    reduce_with_identity(iterable, initial, combine)
}

/// Performs a fallible fold.
///
/// This is an alias for [`try_reduce_with_identity`].
pub fn try_fold<I, A, F, E>(
    iterable: I,
    initial: A,
    combine: F,
) -> Result<A, E>
where
    I: IntoIterator,
    F: FnMut(A, I::Item) -> Result<A, E>,
{
    try_reduce_with_identity(iterable, initial, combine)
}

#[cfg(test)]
mod tests {
    use super::{
        fold, reduce, reduce_with, reduce_with_identity, try_fold, try_reduce,
        try_reduce_with_identity,
    };

    #[test]
    fn empty_reduction_has_no_implicit_identity() {
        assert_eq!(reduce(std::iter::empty::<i64>()), None);
        assert_eq!(
            reduce_with(std::iter::empty::<i64>(), |a, b| a + b),
            None
        );
    }

    #[test]
    fn singleton_reduction_preserves_the_element() {
        assert_eq!(reduce([7_i64]), Some(7));
        assert_eq!(reduce_with([7_i64], |a, b| a + b), Some(7));
    }

    #[test]
    fn reduction_follows_iterator_order() {
        let mut visited = Vec::new();

        let result = reduce_with([1, 2, 3, 4], |a, b| {
            visited.push((a, b));
            a - b
        });

        assert_eq!(result, Some(-8));
        assert_eq!(visited, [(1, 2), (-1, 3), (-4, 4)]);
    }

    #[test]
    fn identity_fold_handles_empty_input() {
        assert_eq!(
            reduce_with_identity(std::iter::empty::<i64>(), 42, |a, b| a + b),
            42
        );

        assert_eq!(fold([1, 2, 3], 0, |a, b| a + b), 6);
    }

    #[test]
    fn custom_reducer_supports_non_additive_operations() {
        assert_eq!(reduce_with([2_u64, 3, 4], |a, b| a * b), Some(24));
        assert_eq!(reduce_with([9_i64, 4, 2], |a, b| a - b), Some(3));
    }

    #[test]
    fn fallible_reduction_propagates_errors() {
        let result = try_reduce([i32::MAX, 1, 2], |a, b| {
            a.checked_add(b).ok_or("overflow")
        });

        assert_eq!(result, Err("overflow"));
    }

    #[test]
    fn fallible_reduction_of_empty_input_returns_none() {
        let result = try_reduce(std::iter::empty::<i64>(), |a, b| {
            a.checked_add(b).ok_or("overflow")
        });

        assert_eq!(result, Ok(None));
    }

    #[test]
    fn fallible_identity_fold_propagates_errors() {
        let result = try_reduce_with_identity([i32::MAX, 1], 0, |a, b| {
            a.checked_add(b).ok_or("overflow")
        });

        assert_eq!(result, Err("overflow"));
    }

    #[test]
    fn fallible_fold_returns_identity_for_empty_input() {
        let result: Result<i64, &'static str> =
            try_fold(std::iter::empty::<i64>(), 17, |a, b| {
                a.checked_add(b).ok_or("overflow")
            });

        assert_eq!(result, Ok(17));
    }

    #[test]
    fn fallible_reduction_stops_after_first_error() {
        let mut visited = Vec::new();

        let result = try_reduce([1_i32, 2, 3, 4], |a, b| {
            visited.push((a, b));

            if b == 3 {
                Err("rejected element")
            } else {
                Ok(a + b)
            }
        });

        assert_eq!(result, Err("rejected element"));
        assert_eq!(visited, [(1, 2), (3, 3)]);
    }

    #[test]
    fn iterator_is_consumed_once_in_order() {
        let mut next_value = 0;

        let iterable = std::iter::from_fn(|| {
            if next_value == 5 {
                return None;
            }

            let value = next_value;
            next_value += 1;
            Some(value)
        });

        assert_eq!(reduce_with(iterable, |a, b| a + b), Some(10));
        assert_eq!(next_value, 5);
    }

    #[test]
    fn large_logical_input_needs_no_fixed_capacity_buffer() {
        // Exercises streaming consumption without materializing the input.
        let result = reduce_with(0_u64..100_000, |a, b| a + b);

        assert_eq!(result, Some(4_999_950_000));
    }
}
