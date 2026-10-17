
#![forbid(unsafe_code)]

//! Zamani classical data-parallel computation.
//!
//! Rust baseline: 1.97+; edition 2021.
//!
//! # Contract
//!
//! Purpose:
//! - Represent portable data-parallel work over logical index domains.
//! - Produce deterministic, lazily generated work partitions.
//! - Supply safe sequential reference implementations for map and reduction.
//! - Preserve the distinction between logical parallelism and physical execution.
//!
//! Owns:
//! - `IndexDomain`
//! - `PartitionGrain`
//! - `DataParallelHint`
//! - `DataParallelPlan`
//! - `DataPartition`
//! - `PartitionIter`
//! - sequential reference algorithms and their errors
//!
//! Does not own:
//! - source syntax, AST, classical types, or canonical IR;
//! - task identities or task lifecycle;
//! - worker pools, executors, scheduling, or device placement;
//! - resource discovery or capability registries;
//! - reduction reassociation or floating-point semantics;
//! - quantum or HDL execution.
//!
//! # POCO-REAF
//!
//! A logical domain does not encode a physical worker count. The same plan can
//! be consumed by a sequential executor, a thread executor, a device backend,
//! or a distributed runtime, provided that the target satisfies the program's
//! semantic requirements.
//!
//! There is no universal maximum domain length, partition count, worker count,
//! or device count. The host representation is finite, and concrete execution
//! remains constrained by available resources and explicit runtime policies.
//!
//! # Integration
//!
//! - `concurrency/mod.rs`: register `pub mod data_parallel;`.
//! - `concurrency/task.rs`: schedulers may translate partitions into tasks
//!   using the canonical task contract; this module does not duplicate TaskId.
//! - `concurrency/scheduling.rs`: consume partitions and select execution order.
//! - `concurrency/determinism.rs`: enforce the program's determinism contract.
//! - `execution/requirements.rs` and `execution/capabilities.rs`: resolve
//!   required capabilities and available execution resources.
//! - Semantic analysis: validate effects, aliasing, ownership, dependencies,
//!   and whether parallel execution is legal.
//! - Canonical frontend/IR: lower existing grammar constructs into the
//!   repository's canonical representation; do not create a second IR.
//!
//! This module is intentionally standard-library-only and does not assume
//! unfinished APIs from other Zamani modules.

use std::error::Error;
use std::fmt;
use std::iter::FusedIterator;
use std::num::{NonZeroU128, NonZeroUsize};
use std::ops::Range;

/// A half-open logical index domain `[start, end)`.
///
/// The domain describes indices, not allocated storage. Its size can exceed
/// `usize` on platforms where `usize` is narrower than `u128`; an executor
/// must not assume that all indices can be materialized simultaneously.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IndexDomain {
    start: u128,
    end: u128,
}

impl IndexDomain {
    /// Construct a domain. The end is exclusive.
    pub fn new(start: u128, end: u128) -> Result<Self, DataParallelError> {
        if start > end {
            return Err(DataParallelError::InvalidDomain { start, end });
        }

        Ok(Self { start, end })
    }

    pub const fn start(self) -> u128 {
        self.start
    }

    pub const fn end(self) -> u128 {
        self.end
    }

    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Return the number of logical indices.
    ///
    /// Because `end >= start`, this subtraction cannot overflow.
    pub const fn len(self) -> u128 {
        self.end - self.start
    }

    pub const fn is_singleton(self) -> bool {
        self.len() == 1
    }

    /// Return whether an index belongs to this half-open domain.
    pub const fn contains(self, index: u128) -> bool {
        index >= self.start && index < self.end
    }
}

/// A positive logical partition grain.
///
/// Grain is a planning parameter, not a hardware vector width, worker count,
/// or language-level size restriction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PartitionGrain(NonZeroU128);

impl PartitionGrain {
    pub const fn new(value: NonZeroU128) -> Self {
        Self(value)
    }

    pub fn try_from_u128(value: u128) -> Result<Self, DataParallelError> {
        NonZeroU128::new(value)
            .map(Self)
            .ok_or(DataParallelError::ZeroPartitionGrain)
    }

    pub const fn get(self) -> u128 {
        self.0.get()
    }
}

/// Execution intent passed to downstream planning.
///
/// These hints never select a physical device. `RequireParallel` means that
/// the execution contract must be checked by the scheduler; silently
/// substituting sequential execution would violate that requirement.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum DataParallelHint {
    /// Let the optimizer and scheduler choose a legal realization.
    #[default]
    Auto,

    /// Prefer sequential execution when that is beneficial and legal.
    PreferSequential,

    /// Prefer a parallel realization when that is beneficial and legal.
    PreferParallel,

    /// Require a realization satisfying the language's parallel-execution
    /// contract. The scheduler must reject an unsatisfied requirement.
    RequireParallel,
}

/// A declarative data-parallel execution plan.
///
/// Constructing a plan does not allocate storage proportional to the domain
/// size, create tasks, or start execution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DataParallelPlan {
    domain: IndexDomain,
    grain: PartitionGrain,
    hint: DataParallelHint,
}

impl DataParallelPlan {
    pub const fn new(
        domain: IndexDomain,
        grain: PartitionGrain,
        hint: DataParallelHint,
    ) -> Self {
        Self {
            domain,
            grain,
            hint,
        }
    }

    pub const fn domain(self) -> IndexDomain {
        self.domain
    }

    pub const fn grain(self) -> PartitionGrain {
        self.grain
    }

    pub const fn hint(self) -> DataParallelHint {
        self.hint
    }

    /// Number of logical partitions implied by the grain.
    ///
    /// This is metadata only. It does not allocate or materialize partitions.
    pub const fn partition_count(self) -> u128 {
        let length = self.domain.len();
        let grain = self.grain.get();
        let quotient = length / grain;
        let remainder = length % grain;

        // If remainder is nonzero, quotient is strictly less than u128::MAX.
        quotient + if remainder == 0 { 0 } else { 1 }
    }

    /// Create a lazy iterator over this plan's partitions.
    pub const fn partitions(self) -> PartitionIter {
        PartitionIter {
            next_start: self.domain.start,
            end: self.domain.end,
            grain: self.grain.get(),
            next_index: 0,
        }
    }

    /// Whether this plan contains no work.
    pub const fn is_empty(self) -> bool {
        self.domain.is_empty()
    }
}

/// One logical partition of a data-parallel domain.
///
/// `index` identifies this partition within its plan. It is not a globally
/// unique task ID and must not be used as a replacement for the task identity
/// owned by `concurrency/task.rs`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DataPartition {
    index: u128,
    range: Range<u128>,
}

impl DataPartition {
    pub const fn index(self) -> u128 {
        self.index
    }

    pub const fn start(self) -> u128 {
        self.range.start
    }

    pub const fn end(self) -> u128 {
        self.range.end
    }

    pub fn range(self) -> Range<u128> {
        self.range
    }

    pub const fn len(self) -> u128 {
        self.range.end - self.range.start
    }

    pub const fn is_empty(self) -> bool {
        self.range.start == self.range.end
    }
}

/// Lazily yields non-overlapping partitions in ascending index order.
///
/// Memory use is constant with respect to the number of partitions.
/// Each yielded partition is nonempty, except that an empty domain yields
/// no partitions.
#[derive(Clone, Debug)]
pub struct PartitionIter {
    next_start: u128,
    end: u128,
    grain: u128,
    next_index: u128,
}

impl Iterator for PartitionIter {
    type Item = DataPartition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_start >= self.end {
            return None;
        }

        let start = self.next_start;

        // Saturation is intentional: the end is exclusive, and the result
        // is clamped to the domain end. This avoids wrapping at u128::MAX.
        let end = start.saturating_add(self.grain).min(self.end);

        let partition = DataPartition {
            index: self.next_index,
            range: start..end,
        };

        self.next_start = end;

        // For a valid domain and positive grain, a yielded partition cannot
        // have index u128::MAX: at least one index remains to be represented.
        self.next_index += 1;

        Some(partition)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end - self.next_start;
        let quotient = remaining / self.grain;
        let remainder = remaining % self.grain;
        let count = quotient + if remainder == 0 { 0 } else { 1 };

        match usize::try_from(count) {
            Ok(count) => (count, Some(count)),
            Err(_) => (usize::MAX, None),
        }
    }
}

impl FusedIterator for PartitionIter {}

/// Failures in domain validation or reference algorithms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataParallelError {
    InvalidDomain {
        start: u128,
        end: u128,
    },
    ZeroPartitionGrain,
    DomainTooLargeForHost {
        length: u128,
    },
}

impl fmt::Display for DataParallelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDomain { start, end } => {
                write!(
                    f,
                    "invalid half-open data domain: start {start} exceeds end {end}"
                )
            }
            Self::ZeroPartitionGrain => {
                write!(f, "data-parallel partition grain must be positive")
            }
            Self::DomainTooLargeForHost { length } => {
                write!(
                    f,
                    "logical domain length {length} cannot be materialized \
                     as a host-sized collection"
                )
            }
        }
    }
}

impl Error for DataParallelError {}

/// Execute a map operation sequentially, preserving input order.
///
/// This is a portable reference implementation, not a parallel executor.
/// It provides a correctness baseline against which a parallel backend can
/// test its results.
///
/// The caller is responsible for ensuring that the operation is valid under
/// Zamani's effect, ownership, and determinism rules.
pub fn map_sequential<I, F, O>(
    input: I,
    operation: F,
) -> Vec<O>
where
    I: IntoIterator,
    F: FnMut(I::Item) -> O,
{
    input.into_iter().map(operation).collect()
}

/// Execute a fallible map sequentially, stopping at the first error.
///
/// The result preserves input order. Partial results are not returned on
/// failure; callers that require streaming or incremental commitment should
/// use a dedicated execution API with an explicit partial-result contract.
pub fn try_map_sequential<I, F, O, E>(
    input: I,
    operation: F,
) -> Result<Vec<O>, E>
where
    I: IntoIterator,
    F: FnMut(I::Item) -> Result<O, E>,
{
    input.into_iter().map(operation).collect()
}

/// Perform an ordered reduction over an iterator.
///
/// The supplied operation is applied in input order. This function does not
/// reassociate operations, which is important for floating-point arithmetic,
/// non-associative operations, and observable effects.
///
/// A parallel reduction may change the grouping of operations only when
/// semantic analysis has established that reassociation is legal under the
/// program's numerical and effect contracts.
pub fn reduce_ordered<I, F>(
    input: I,
    mut operation: F,
) -> Option<I::Item>
where
    I: IntoIterator,
    F: FnMut(I::Item, I::Item) -> I::Item,
{
    let mut iterator = input.into_iter();
    let first = iterator.next()?;

    Some(iterator.fold(first, |accumulator, item| {
        operation(accumulator, item)
    }))
}

/// Compute a checked host-sized length for APIs that must materialize a
/// domain into a host collection.
///
/// Do not use this conversion when a symbolic or streaming representation
/// suffices. Failure is explicit rather than silently truncating the domain.
pub fn materializable_length(
    domain: IndexDomain,
) -> Result<usize, DataParallelError> {
    usize::try_from(domain.len()).map_err(|_| {
        DataParallelError::DomainTooLargeForHost {
            length: domain.len(),
        }
    })
}

/// Optional host-side execution information supplied by a resource resolver.
///
/// These values are observations, not semantic ceilings. A scheduler must
/// account for changes in resource availability and cannot assume that a
/// snapshot guarantees resources remain available.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExecutionResources {
    available_workers: Option<NonZeroUsize>,
}

impl ExecutionResources {
    pub const fn new(available_workers: Option<NonZeroUsize>) -> Self {
        Self { available_workers }
    }

    pub const fn available_workers(self) -> Option<NonZeroUsize> {
        self.available_workers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grain(value: u128) -> PartitionGrain {
        PartitionGrain::try_from_u128(value).expect("positive test grain")
    }

    fn domain(start: u128, end: u128) -> IndexDomain {
        IndexDomain::new(start, end).expect("valid test domain")
    }

    #[test]
    fn rejects_reversed_domain() {
        assert_eq!(
            IndexDomain::new(9, 2),
            Err(DataParallelError::InvalidDomain {
                start: 9,
                end: 2,
            })
        );
    }

    #[test]
    fn accepts_empty_domain() {
        let domain = domain(4, 4);

        assert!(domain.is_empty());
        assert_eq!(domain.len(), 0);

        let plan = DataParallelPlan::new(
            domain,
            grain(3),
            DataParallelHint::Auto,
        );

        assert_eq!(plan.partition_count(), 0);
        assert_eq!(plan.partitions().count(), 0);
    }

    #[test]
    fn rejects_zero_grain() {
        assert_eq!(
            PartitionGrain::try_from_u128(0),
            Err(DataParallelError::ZeroPartitionGrain)
        );
    }

    #[test]
    fn partitions_cover_domain_without_gaps_or_overlap() {
        let plan = DataParallelPlan::new(
            domain(10, 23),
            grain(4),
            DataParallelHint::PreferParallel,
        );

        let partitions: Vec<_> = plan.partitions().collect();

        assert_eq!(partitions.len(), 4);
        assert_eq!(partitions[0].range(), 10..14);
        assert_eq!(partitions[1].range(), 14..18);
        assert_eq!(partitions[2].range(), 18..22);
        assert_eq!(partitions[3].range(), 22..23);

        assert_eq!(
            partitions.iter().map(|part| part.len()).sum::<u128>(),
            plan.domain().len()
        );
    }

    #[test]
    fn partition_count_uses_ceiling_division() {
        let plan = DataParallelPlan::new(
            domain(0, 10),
            grain(3),
            DataParallelHint::Auto,
        );

        assert_eq!(plan.partition_count(), 4);
        assert_eq!(plan.partitions().count(), 4);
    }

    #[test]
    fn singleton_domain_has_one_partition() {
        let plan = DataParallelPlan::new(
            domain(8, 9),
            grain(10),
            DataParallelHint::Auto,
        );

        assert_eq!(plan.partition_count(), 1);

        let partition = plan.partitions().next().expect("one partition");

        assert_eq!(partition.range(), 8..9);
        assert_eq!(partition.index(), 0);
        assert!(plan.partitions().next().is_none());
    }

    #[test]
    fn full_host_representation_boundary_does_not_wrap() {
        let plan = DataParallelPlan::new(
            domain(u128::MAX - 2, u128::MAX),
            grain(2),
            DataParallelHint::Auto,
        );

        let partitions: Vec<_> = plan.partitions().collect();

        assert_eq!(partitions.len(), 1);
        assert_eq!(
            partitions[0].range(),
            (u128::MAX - 2)..u128::MAX
        );
    }

    #[test]
    fn huge_domain_is_planned_without_materializing_partitions() {
        let plan = DataParallelPlan::new(
            domain(0, u128::MAX),
            grain(1),
            DataParallelHint::Auto,
        );

        assert_eq!(plan.partition_count(), u128::MAX);

        // Do not collect the iterator: its logical length exceeds ordinary
        // host collection capacity. Only inspect a few lazy items.
        let mut partitions = plan.partitions();

        assert_eq!(partitions.next().expect("first").range(), 0..1);
        assert_eq!(partitions.next().expect("second").range(), 1..2);
        assert_eq!(partitions.next().expect("third").range(), 2..3);
    }

    #[test]
    fn lazy_partition_iteration_is_deterministic() {
        let plan = DataParallelPlan::new(
            domain(3, 29),
            grain(5),
            DataParallelHint::Auto,
        );

        assert_eq!(
            plan.partitions().collect::<Vec<_>>(),
            plan.partitions().collect::<Vec<_>>()
        );
    }

    #[test]
    fn sequential_map_preserves_order() {
        let result = map_sequential(0..5, |value| value * 2);

        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn fallible_map_stops_at_first_error() {
        let result = try_map_sequential(0..5, |value| {
            if value == 3 {
                Err("three")
            } else {
                Ok(value * 2)
            }
        });

        assert_eq!(result, Err("three"));
    }

    #[test]
    fn ordered_reduction_preserves_input_order() {
        let result = reduce_ordered(["a", "b", "c"], |left, right| {
            format!("({left}+{right})")
        });

        assert_eq!(result.as_deref(), Some("((a+b)+c)"));
    }

    #[test]
    fn empty_reduction_returns_none() {
        let result = reduce_ordered(
            std::iter::empty::<u64>(),
            |left, right| left + right,
        );

        assert_eq!(result, None);
    }

    #[test]
    fn singleton_reduction_returns_original_value() {
        let result = reduce_ordered([17_u64], |left, right| left + right);

        assert_eq!(result, Some(17));
    }

    #[test]
    fn host_length_conversion_is_checked() {
        let domain = domain(10, 20);

        assert_eq!(materializable_length(domain), Ok(10));
    }

    #[test]
    fn execution_hint_does_not_change_partition_semantics() {
        let domain = domain(0, 20);
        let grain = grain(3);

        let automatic = DataParallelPlan::new(
            domain,
            grain,
            DataParallelHint::Auto,
        );
        let required = DataParallelPlan::new(
            domain,
            grain,
            DataParallelHint::RequireParallel,
        );

        assert_eq!(automatic.partition_count(), required.partition_count());
        assert_eq!(
            automatic.partitions().collect::<Vec<_>>(),
            required.partitions().collect::<Vec<_>>()
        );
    }
}
