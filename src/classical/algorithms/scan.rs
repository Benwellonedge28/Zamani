
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Prefix Scan Algorithms
//!
//! File: `src/classical/algorithms/scan.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Status
//!
//! Proposed implementation. Parent-module registration, workspace compilation,
//! semantic/lowering integration, and CI verification remain required.
//!
//! # Purpose
//!
//! A scan computes intermediate prefix results over an ordered input sequence.
//! Unlike a reduction, which returns one final result, a scan yields a result
//! for each successfully processed input element.
//!
//! This module provides:
//!
//! - `inclusive_scan`: lazily yields inclusive prefix results;
//! - `exclusive_scan`: lazily yields the accumulator before each input element;
//! - `try_inclusive_scan`: an inclusive scan whose combining function may fail.
//!
//! # Authority and ownership
//!
//! - `grammar/concurrency/data-parallel.g4` owns the `parallel::scan` syntax.
//! - The frontend owns source spans and AST construction.
//! - Semantic analysis owns type checking, callable resolution, effects,
//!   ordering guarantees, determinism, and scan legality.
//! - The repository's canonical IR owns semantic operation representation.
//! - Numeric semantics and policies own overflow, precision, and exceptional
//!   numeric behavior for Zamani numeric types.
//! - Shared execution and resource systems own budgets, cancellation,
//!   scheduling, placement, and physical parallelization.
//!
//! This file owns only generic sequential scan adapters and their iterator
//! contracts. It does not create AST nodes, IR types, numeric types, resource
//! registries, or a scheduler.
//!
//! # Semantics
//!
//! 1. Input elements are consumed in iterator order.
//! 2. Inclusive scans include the current input in each yielded prefix.
//! 3. Exclusive scans yield the accumulator before incorporating the current
//!    input element.
//! 4. Inclusive scans without an initial value yield no result for empty input.
//! 5. Exclusive scans yield no result for empty input.
//! 6. An exclusive scan with N input elements yields N accumulator values.
//! 7. The combining function is applied sequentially and is not reassociated.
//! 8. No associativity, commutativity, purity, or determinism is assumed.
//! 9. The fallible scan yields the first combining error and then terminates.
//! 10. This module does not roll back effects performed by a combining function.
//!
//! # Scalability and resources
//!
//! The adapters consume iterators lazily and do not collect the input or output
//! into a `Vec`. They introduce no architectural maximum for the number of
//! elements. Memory attributable to scan state is constant, apart from values
//! retained by the iterator, accumulator, and combining closure.
//!
//! This does not make every infinite scan complete: an infinite iterator
//! produces an infinite stream of results. A consumer that requires all results
//! must have an external termination condition or explicit resource policy.
//! Cancellation, resource budgets, and target selection belong to the caller.
//!
//! # Numerical and parallel correctness
//!
//! Floating-point operations and arbitrary user functions can be sensitive to
//! evaluation order. These implementations preserve sequential iterator order.
//! A parallel scan must be implemented by a downstream execution layer and may
//! reorder operations only when semantic analysis proves the transformation
//! legal under the program's numerical, effect, and determinism contracts.
//!
//! # Integration contract
//!
//! `src/classical/algorithms/mod.rs` should declare `pub mod scan;` and export
//! the public API as appropriate for the repository's visibility policy.
//!
//! The frontend and semantic layer should map `parallel::scan(values, scanner)`
//! to the canonical operation representation. Lowering may choose this
//! sequential implementation as a fallback when its semantics match the
//! source contract. The optimizer, scheduler, and runtime may choose other
//! implementations only after validating their legality and resource needs.
//!
//! No dependency on a not-yet-existing `crate::classical::error` is introduced:
//! the fallible API preserves the caller's error type.
//!
//! # Compatibility
//!
//! The API is generic over `IntoIterator` and uses stable Rust standard-library
//! traits only. The examples assume the library target is named
//! `zamani_compiler`, as declared in the repository's `Cargo.toml`.

// -----------------------------------------------------------------------------
// Inclusive scan
// -----------------------------------------------------------------------------

/// Lazily computes inclusive prefix results.
///
/// The first input element is yielded unchanged. Every subsequent element is
/// combined with the previous prefix result and the new input element.
///
/// `I::Item: Clone` is required because each prefix result must both be yielded
/// to the caller and retained as the next accumulator.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::scan::inclusive_scan;
///
/// let result: Vec<_> = inclusive_scan([1_i64, 2, 3, 4], |a, b| a + b).collect();
/// assert_eq!(result, [1, 3, 6, 10]);
/// ```
pub fn inclusive_scan<I, F>(
    iterable: I,
    combine: F,
) -> InclusiveScan<I::IntoIter, F>
where
    I: IntoIterator,
    I::Item: Clone,
    F: FnMut(I::Item, I::Item) -> I::Item,
{
    InclusiveScan {
        iter: iterable.into_iter(),
        accumulator: None,
        combine,
    }
}

/// Iterator adapter for [`inclusive_scan`].
#[must_use = "iterators are lazy; consume the scan to obtain its results"]
pub struct InclusiveScan<I, F>
where
    I: Iterator,
    I::Item: Clone,
    F: FnMut(I::Item, I::Item) -> I::Item,
{
    iter: I,
    accumulator: Option<I::Item>,
    combine: F,
}

impl<I, F> Iterator for InclusiveScan<I, F>
where
    I: Iterator,
    I::Item: Clone,
    F: FnMut(I::Item, I::Item) -> I::Item,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;

        let next_accumulator = match self.accumulator.take() {
            None => item,
            Some(previous) => (self.combine)(previous, item),
        };

        self.accumulator = Some(next_accumulator.clone());
        Some(next_accumulator)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// -----------------------------------------------------------------------------
// Exclusive scan
// -----------------------------------------------------------------------------

/// Lazily computes exclusive prefix results from an explicit initial value.
///
/// For each input element, this yields the accumulator *before* incorporating
/// that element, then updates the accumulator with `combine(accumulator, item)`.
/// The initial value is not required to be a mathematical identity; it is the
/// caller-supplied starting state.
///
/// This API requires `A: Clone` because it must yield the current accumulator
/// while retaining a value to pass to the combining function.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::scan::exclusive_scan;
///
/// let result: Vec<_> = exclusive_scan([1_i64, 2, 3, 4], 0, |a, b| a + b)
///     .collect();
///
/// assert_eq!(result, [0, 1, 3, 6]);
/// ```
pub fn exclusive_scan<I, A, F>(
    iterable: I,
    initial: A,
    combine: F,
) -> ExclusiveScan<I::IntoIter, A, F>
where
    I: IntoIterator,
    A: Clone,
    F: FnMut(A, I::Item) -> A,
{
    ExclusiveScan {
        iter: iterable.into_iter(),
        accumulator: initial,
        combine,
    }
}

/// Iterator adapter for [`exclusive_scan`].
#[must_use = "iterators are lazy; consume the scan to obtain its results"]
pub struct ExclusiveScan<I, A, F>
where
    I: Iterator,
    A: Clone,
    F: FnMut(A, I::Item) -> A,
{
    iter: I,
    accumulator: A,
    combine: F,
}

impl<I, A, F> Iterator for ExclusiveScan<I, A, F>
where
    I: Iterator,
    A: Clone,
    F: FnMut(A, I::Item) -> A,
{
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        let previous = self.accumulator.clone();
        self.accumulator = (self.combine)(self.accumulator.clone(), item);
        Some(previous)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// -----------------------------------------------------------------------------
// Fallible inclusive scan
// -----------------------------------------------------------------------------

/// Lazily computes inclusive prefix results with a fallible combining function.
///
/// The first input is yielded unchanged. If combining a later input returns an
/// error, that error is yielded once and the scan terminates. No subsequent
/// input is consumed after the error.
///
/// The error type is generic so callers can preserve the canonical error type
/// used by their subsystem without this module defining a competing error
/// vocabulary.
///
/// # Examples
///
/// ```
/// use zamani_compiler::classical::algorithms::scan::try_inclusive_scan;
///
/// let result: Result<Vec<_>, _> = try_inclusive_scan(
///     [1_i32, 2, 3],
///     |a, b| a.checked_add(b).ok_or("integer overflow"),
/// ).collect();
///
/// assert_eq!(result, Ok(vec![1, 3, 6]));
/// ```
pub fn try_inclusive_scan<I, F, E>(
    iterable: I,
    combine: F,
) -> TryInclusiveScan<I::IntoIter, F>
where
    I: IntoIterator,
    I::Item: Clone,
    F: FnMut(I::Item, I::Item) -> Result<I::Item, E>,
{
    TryInclusiveScan {
        iter: iterable.into_iter(),
        accumulator: None,
        combine,
        failed: false,
    }
}

/// Iterator adapter for [`try_inclusive_scan`].
#[must_use = "iterators are lazy; consume the scan to obtain its results"]
pub struct TryInclusiveScan<I, F>
where
    I: Iterator,
    I::Item: Clone,
    F: FnMut(I::Item, I::Item) -> Result<I::Item, E>,
{
    iter: I,
    accumulator: Option<I::Item>,
    combine: F,
    failed: bool,
}

// `E` is part of the combining function's output type and is inferred from F.
impl<I, F, E> Iterator for TryInclusiveScan<I, F>
where
    I: Iterator,
    I::Item: Clone,
    F: FnMut(I::Item, I::Item) -> Result<I::Item, E>,
{
    type Item = Result<I::Item, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            return None;
        }

        let item = self.iter.next()?;

        match self.accumulator.take() {
            None => {
                self.accumulator = Some(item.clone());
                Some(Ok(item))
            }
            Some(previous) => match (self.combine)(previous, item) {
                Ok(next_accumulator) => {
                    self.accumulator = Some(next_accumulator.clone());
                    Some(Ok(next_accumulator))
                }
                Err(error) => {
                    self.failed = true;
                    Some(Err(error))
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.failed {
            return (0, Some(0));
        }

        let (lower, upper) = self.iter.size_hint();

        // A combining error can terminate the scan before all inputs are used.
        (0.min(lower), upper)
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{exclusive_scan, inclusive_scan, try_inclusive_scan};

    #[test]
    fn inclusive_scan_produces_prefixes() {
        let result: Vec<_> = inclusive_scan([1_i64, 2, 3, 4], |a, b| a + b)
            .collect();

        assert_eq!(result, [1, 3, 6, 10]);
    }

    #[test]
    fn inclusive_scan_handles_empty_input() {
        let result: Vec<i64> =
            inclusive_scan(std::iter::empty::<i64>(), |a, b| a + b).collect();

        assert!(result.is_empty());
    }

    #[test]
    fn inclusive_scan_handles_one_element() {
        let result: Vec<_> = inclusive_scan([9_i64], |a, b| a + b).collect();

        assert_eq!(result, [9]);
    }

    #[test]
    fn inclusive_scan_preserves_non_associative_order() {
        let result: Vec<_> = inclusive_scan([10_i64, 3, 2], |a, b| a - b)
            .collect();

        assert_eq!(result, [10, 7, 5]);
    }

    #[test]
    fn exclusive_scan_yields_accumulator_before_each_item() {
        let result: Vec<_> =
            exclusive_scan([1_i64, 2, 3, 4], 0, |a, b| a + b).collect();

        assert_eq!(result, [0, 1, 3, 6]);
    }

    #[test]
    fn exclusive_scan_handles_empty_input() {
        let result: Vec<_> =
            exclusive_scan(std::iter::empty::<i64>(), 42, |a, b| a + b)
                .collect();

        assert!(result.is_empty());
    }

    #[test]
    fn exclusive_scan_handles_single_element() {
        let result: Vec<_> = exclusive_scan([7_i64], 5, |a, b| a + b).collect();

        assert_eq!(result, [5]);
    }

    #[test]
    fn exclusive_scan_does_not_assume_identity_is_mathematical_identity() {
        let result: Vec<_> = exclusive_scan([2_i64, 3, 4], 10, |a, b| a * b)
            .collect();

        assert_eq!(result, [10, 20, 60]);
    }

    #[test]
    fn fallible_scan_returns_prefixes_when_successful() {
        let result: Result<Vec<_>, &str> = try_inclusive_scan(
            [1_i32, 2, 3, 4],
            |a, b| a.checked_add(b).ok_or("overflow"),
        )
        .collect();

        assert_eq!(result, Ok(vec![1, 3, 6, 10]));
    }

    #[test]
    fn fallible_scan_yields_error_and_then_terminates() {
        let mut visited = Vec::new();

        let mut scan = try_inclusive_scan([1_i32, 2, 3, 4], |a, b| {
            visited.push((a, b));

            if b == 3 {
                Err("rejected")
            } else {
                Ok(a + b)
            }
        });

        assert_eq!(scan.next(), Some(Ok(1)));
        assert_eq!(scan.next(), Some(Ok(3)));
        assert_eq!(scan.next(), Some(Err("rejected")));
        assert_eq!(scan.next(), None);
        assert_eq!(visited, [(1, 2), (3, 3)]);
    }

    #[test]
    fn fallible_scan_handles_empty_input() {
        let result: Result<Vec<i32>, &str> =
            try_inclusive_scan(std::iter::empty::<i32>(), |a, b| {
                a.checked_add(b).ok_or("overflow")
            })
            .collect();

        assert_eq!(result, Ok(Vec::new()));
    }

    #[test]
    fn scan_consumes_input_lazily() {
        let mut produced = 0_u64;

        let source = std::iter::from_fn(|| {
            if produced == 5 {
                return None;
            }

            let item = produced;
            produced += 1;
            Some(item)
        });

        let mut scan = inclusive_scan(source, |a, b| a + b);

        assert_eq!(produced, 0);
        assert_eq!(scan.next(), Some(0));
        assert_eq!(produced, 1);
        assert_eq!(scan.next(), Some(1));
        assert_eq!(produced, 2);
    }

    #[test]
    fn scan_supports_large_streams_without_fixed_capacity_buffers() {
        let result = inclusive_scan(0_u64..100_000, |a, b| a + b).last();

        assert_eq!(result, Some(4_999_950_000));
    }

    #[test]
    fn scan_size_hint_tracks_remaining_input() {
        let mut scan = inclusive_scan([1_i32, 2, 3], |a, b| a + b);

        assert_eq!(scan.size_hint(), (3, Some(3)));
        assert_eq!(scan.next(), Some(1));
        assert_eq!(scan.size_hint(), (2, Some(2)));
    }
}
